//! **The map editor** (ROADMAP WM5, plan C10 of 2026-11-14): what the
//! `larder-edit` window does to a Larder World map (`mould::map`, WM1),
//! kept here, away from the window, so it is tested without a GPU and the
//! headless pictures are the window's own.
//!
//! An `Editor` holds one `Map` in the loader's own cells (1024 x 640) and
//! changes it only as the map format can say it: the ground byte a cell
//! (soil kind in the high nibble, `G_WET`, `G_ROCK`) and the items
//! (crystals, the mould's starts, the squad's landing, walls). So what is
//! saved is exactly what was drawn, and `larder-world map=DIR/NAME` plays
//! it with nothing added.
//!
//! **Ground brushes.** Plain clears the cell. Sand, clay and barren set the
//! soil kind and clear rock (a cell can be wet sand, not rock sand: the
//! engine keeps wet only on open ground). Rock and wet keep the cell's
//! soil, so painting rock away with wet or a soil brush gives back what
//! lay under it only for the soil, not for its old wet.
//!
//! **Mirror.** Most RTS maps are fair by symmetry, so the editor can draw
//! each stroke and item twice: across the vertical centre line (`X`) or
//! through the centre point (`Turn`, a half turn). The landing is one item
//! and is never mirrored; a mirrored mould start gives the map two starts,
//! which the loader shares the walkers between (the Passes has two).
//!
//! **Undo** keeps whole maps (640 KiB each, at most `UNDO` of them): an
//! action is a stroke or one click, so a handful of steps is a few MiB.

use super::map::{Crystal, Items, Map, Wall, G_ROCK, G_WET, MAP_H, MAP_W};

/// Undo steps kept.
pub const UNDO: usize = 40;

/// A new crystal's radius (map cells) and charge a cell: the designed
/// maps' middle crystal (the Fork's flanks, `crystal 330 300 30 384`).
pub const CRYSTAL_R: f32 = 30.0;
pub const CRYSTAL_CHARGE: f32 = 384.0;
/// A new wall's full width and health a cell (`build_wall`'s width, and
/// `wall_hp`'s default, the Nest's walls).
pub const WALL_THICK: f32 = 8.0;
pub const WALL_HP: f32 = 400.0;
/// How near (map cells) a click must be to an item to erase it.
pub const PICK: f32 = 24.0;

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Brush {
    Plain,
    Sand,
    Clay,
    Barren,
    Rock,
    Wet,
}

impl Brush {
    pub const ALL: [Brush; 6] = [Brush::Plain, Brush::Sand, Brush::Clay, Brush::Barren, Brush::Rock, Brush::Wet];

    /// The cell's G byte after this brush.
    pub fn apply(self, g: u8) -> u8 {
        let soil = g & 0xF0;
        let wet = g & G_WET;
        match self {
            Brush::Plain => 0,
            Brush::Sand => 0x10 | wet,
            Brush::Clay => 0x20 | wet,
            Brush::Barren => 0x30 | wet,
            Brush::Rock => soil | G_ROCK,
            Brush::Wet => soil | G_WET,
        }
    }

    pub fn name(self) -> &'static str {
        match self {
            Brush::Plain => "plain",
            Brush::Sand => "sand",
            Brush::Clay => "clay",
            Brush::Barren => "barren",
            Brush::Rock => "rock",
            Brush::Wet => "wet",
        }
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Tool {
    Paint(Brush),
    Crystal,
    Mould,
    Landing,
    Wall,
    Erase,
}

impl Tool {
    /// The tools in the panel's order, with their keys (the window's
    /// digits and letters).
    pub const ALL: [(Tool, &'static str); 11] = [
        (Tool::Paint(Brush::Plain), "1"),
        (Tool::Paint(Brush::Sand), "2"),
        (Tool::Paint(Brush::Clay), "3"),
        (Tool::Paint(Brush::Barren), "4"),
        (Tool::Paint(Brush::Rock), "5"),
        (Tool::Paint(Brush::Wet), "6"),
        (Tool::Crystal, "C"),
        (Tool::Mould, "M"),
        (Tool::Landing, "L"),
        (Tool::Wall, "W"),
        (Tool::Erase, "E"),
    ];

    pub fn name(self) -> &'static str {
        match self {
            Tool::Paint(b) => b.name(),
            Tool::Crystal => "crystal",
            Tool::Mould => "mould start",
            Tool::Landing => "landing",
            Tool::Wall => "wall",
            Tool::Erase => "erase item",
        }
    }

    /// Whether the tool drags a stroke (paint) or a line (wall), rather
    /// than acting on the click.
    pub fn drags(self) -> bool {
        matches!(self, Tool::Paint(_) | Tool::Wall)
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Mirror {
    Off,
    /// Across the vertical centre line.
    X,
    /// Through the centre point.
    Turn,
}

impl Mirror {
    pub fn next(self) -> Mirror {
        match self {
            Mirror::Off => Mirror::X,
            Mirror::X => Mirror::Turn,
            Mirror::Turn => Mirror::Off,
        }
    }

    pub fn name(self) -> &'static str {
        match self {
            Mirror::Off => "off",
            Mirror::X => "left-right",
            Mirror::Turn => "half turn",
        }
    }

    /// The mirrored point, if the mirror is on.
    pub fn of(self, p: (f32, f32)) -> Option<(f32, f32)> {
        let (w, h) = (MAP_W as f32, MAP_H as f32);
        match self {
            Mirror::Off => None,
            Mirror::X => Some((w - p.0, p.1)),
            Mirror::Turn => Some((w - p.0, h - p.1)),
        }
    }
}

pub struct Editor {
    pub map: Map,
    pub tool: Tool,
    /// The paint brush's radius, map cells.
    pub radius: f32,
    /// A new crystal's radius, map cells, and its charge a cell.
    pub crystal_r: f32,
    pub charge: f32,
    pub mirror: Mirror,
    /// Changed since the last save.
    pub dirty: bool,
    undo: Vec<Map>,
    redo: Vec<Map>,
}

impl Editor {
    pub fn new(map: Map) -> Self {
        Editor { map, tool: Tool::Paint(Brush::Sand), radius: 16.0, crystal_r: CRYSTAL_R, charge: CRYSTAL_CHARGE, mirror: Mirror::Off, dirty: false, undo: Vec::new(), redo: Vec::new() }
    }

    /// A blank map: plain open ground, no items.
    pub fn blank() -> Map {
        Map { ground: vec![0; MAP_W * MAP_H], items: Items::default() }
    }

    /// Call before each action (a stroke's first stamp, a click): keeps the
    /// map as it was for `undo`.
    pub fn begin(&mut self) {
        self.undo.push(self.map.clone());
        if self.undo.len() > UNDO {
            self.undo.remove(0);
        }
        self.redo.clear();
        self.dirty = true;
    }

    pub fn undo(&mut self) -> bool {
        let Some(m) = self.undo.pop() else { return false };
        self.redo.push(std::mem::replace(&mut self.map, m));
        self.dirty = true;
        true
    }

    pub fn redo(&mut self) -> bool {
        let Some(m) = self.redo.pop() else { return false };
        self.undo.push(std::mem::replace(&mut self.map, m));
        self.dirty = true;
        true
    }

    /// The point and its mirror image, if any.
    fn both(&self, p: (f32, f32)) -> Vec<(f32, f32)> {
        let mut v = vec![p];
        v.extend(self.mirror.of(p));
        v
    }

    /// One disc of `brush` at `p` (and its mirror).
    pub fn stamp(&mut self, brush: Brush, p: (f32, f32)) {
        let r = self.radius.max(0.5);
        for (cx, cy) in self.both(p) {
            let (x0, x1) = (((cx - r).floor().max(0.0)) as usize, ((cx + r).ceil().max(0.0) as usize).min(MAP_W));
            let (y0, y1) = (((cy - r).floor().max(0.0)) as usize, ((cy + r).ceil().max(0.0) as usize).min(MAP_H));
            for y in y0..y1 {
                for x in x0..x1 {
                    let (dx, dy) = (x as f32 + 0.5 - cx, y as f32 + 0.5 - cy);
                    if dx * dx + dy * dy <= r * r {
                        let g = &mut self.map.ground[y * MAP_W + x];
                        *g = brush.apply(*g);
                    }
                }
            }
        }
    }

    /// Paint from `a` to `b`: discs a quarter radius apart, so a fast drag
    /// leaves no gaps.
    pub fn stroke(&mut self, brush: Brush, a: (f32, f32), b: (f32, f32)) {
        let len = (b.0 - a.0).hypot(b.1 - a.1);
        let n = (len / (self.radius * 0.25).max(0.5)).ceil().max(1.0) as usize;
        for i in 0..=n {
            let t = i as f32 / n as f32;
            self.stamp(brush, (a.0 + (b.0 - a.0) * t, a.1 + (b.1 - a.1) * t));
        }
    }

    /// The click's item for the item tools (Crystal, Mould, Landing,
    /// Erase); paint and wall tools act through `stroke` and `wall`.
    pub fn click(&mut self, p: (f32, f32)) {
        let p = clamp(p);
        match self.tool {
            Tool::Crystal => {
                for (x, y) in self.both(p) {
                    self.map.items.crystals.push(Crystal { x, y, r: self.crystal_r, charge: self.charge });
                }
            }
            Tool::Mould => {
                let starts = self.both(p);
                self.map.items.moulds.extend(starts);
            }
            Tool::Landing => self.map.items.landing = Some(p),
            Tool::Erase => {
                for q in self.both(p) {
                    self.erase(q);
                }
            }
            Tool::Paint(b) => self.stamp(b, p),
            Tool::Wall => {}
        }
    }

    /// A wall from `a` to `b` (and its mirror), `WALL_THICK` wide.
    pub fn wall(&mut self, a: (f32, f32), b: (f32, f32)) {
        let (a, b) = (clamp(a), clamp(b));
        if (b.0 - a.0).hypot(b.1 - a.1) < 1.0 {
            return;
        }
        self.map.items.walls.push(Wall { a, b, thick: WALL_THICK, hp: WALL_HP });
        if let (Some(ma), Some(mb)) = (self.mirror.of(a), self.mirror.of(b)) {
            self.map.items.walls.push(Wall { a: ma, b: mb, thick: WALL_THICK, hp: WALL_HP });
        }
    }

    /// Removes the item nearest `p` within `PICK` (a crystal counts from
    /// its edge, a wall from its line). Returns what it took.
    pub fn erase(&mut self, p: (f32, f32)) -> Option<&'static str> {
        let it = &self.map.items;
        let d = |q: (f32, f32)| (q.0 - p.0).hypot(q.1 - p.1);
        let mut best: Option<(f32, usize, usize)> = None;
        let mut take = |dist: f32, kind: usize, i: usize| {
            if dist <= PICK && best.is_none_or(|b| dist < b.0) {
                best = Some((dist, kind, i));
            }
        };
        for (i, c) in it.crystals.iter().enumerate() {
            take((d((c.x, c.y)) - c.r).max(0.0), 0, i);
        }
        for (i, &m) in it.moulds.iter().enumerate() {
            take(d(m), 1, i);
        }
        if let Some(l) = it.landing {
            take(d(l), 2, 0);
        }
        for (i, w) in it.walls.iter().enumerate() {
            take(segment_distance(p, w.a, w.b), 3, i);
        }
        let (_, kind, i) = best?;
        let it = &mut self.map.items;
        Some(match kind {
            0 => {
                it.crystals.remove(i);
                "crystal"
            }
            1 => {
                it.moulds.remove(i);
                "mould start"
            }
            2 => {
                it.landing = None;
                "landing"
            }
            _ => {
                it.walls.remove(i);
                "wall"
            }
        })
    }

    /// Grows (`+1`) or shrinks (`-1`) what the tool places: the brush, or
    /// a new crystal.
    pub fn size(&mut self, step: i32) {
        let k = if step > 0 { 1.25 } else { 0.8 };
        match self.tool {
            Tool::Crystal => self.crystal_r = (self.crystal_r * k).clamp(8.0, 120.0).round(),
            _ => self.radius = (self.radius * k).clamp(1.0, 160.0).round(),
        }
    }

    /// What stops the map playing (`larder-world` wants a crystal, a mould
    /// start and the landing) or would play oddly; empty when it is ready.
    pub fn problems(&self) -> Vec<String> {
        let it = &self.map.items;
        let mut out = Vec::new();
        if it.crystals.is_empty() {
            out.push("no crystal (C)".to_string());
        }
        if it.moulds.is_empty() {
            out.push("no mould start (M)".to_string());
        }
        if it.landing.is_none() {
            out.push("no landing (L)".to_string());
        }
        let rock = |p: (f32, f32)| self.ground_at(p).is_some_and(|g| g & G_ROCK != 0);
        for (i, &m) in it.moulds.iter().enumerate() {
            if rock(m) {
                out.push(format!("mould start {} is on rock", i + 1));
            }
        }
        if it.landing.is_some_and(rock) {
            out.push("the landing is on rock".to_string());
        }
        out
    }

    /// The G byte under map point `p`, if it is on the map.
    pub fn ground_at(&self, p: (f32, f32)) -> Option<u8> {
        if p.0 < 0.0 || p.1 < 0.0 || p.0 >= MAP_W as f32 || p.1 >= MAP_H as f32 {
            return None;
        }
        Some(self.map.ground[p.1 as usize * MAP_W + p.0 as usize])
    }

    /// The ground under `p` in words, for the status line.
    pub fn describe(&self, p: (f32, f32)) -> String {
        let Some(g) = self.ground_at(p) else { return "off the map".into() };
        let soil = match g >> 4 {
            1 => "sand",
            2 => "clay",
            3 => "barren",
            _ => "plain",
        };
        let kind = if g & G_ROCK != 0 {
            " rock"
        } else if g & G_WET != 0 {
            " wet"
        } else {
            ""
        };
        format!("{soil}{kind}")
    }
}

fn clamp(p: (f32, f32)) -> (f32, f32) {
    (p.0.clamp(0.0, MAP_W as f32 - 1.0), p.1.clamp(0.0, MAP_H as f32 - 1.0))
}

pub fn segment_distance(p: (f32, f32), a: (f32, f32), b: (f32, f32)) -> f32 {
    let (dx, dy) = (b.0 - a.0, b.1 - a.1);
    let l2 = dx * dx + dy * dy;
    let t = if l2 > 0.0 { (((p.0 - a.0) * dx + (p.1 - a.1) * dy) / l2).clamp(0.0, 1.0) } else { 0.0 };
    (p.0 - a.0 - t * dx).hypot(p.1 - a.1 - t * dy)
}

/// Whether `(dx, dy)` from a crystal's centre is inside its pointy-top
/// hexagon of circumradius `r`: `BigDish::in_pile`'s test, so the editor
/// draws the footprint the engine will make.
pub fn in_hex(dx: f32, dy: f32, r: f32) -> bool {
    let ax = dx.abs();
    ax <= r * 0.866_025_4 && dy.abs() <= r - ax * 0.577_350_3
}

/// The ground's colour, the world view's (`view.wgsl` `cell_mat` on flat
/// land: plain soil halfway between SOIL_LOW and SOIL_HIGH, the soil
/// kinds' own RGB, wet's water and rock's grey), so a map looks in the
/// editor as it will when played, before any mould is on it.
pub fn ground_rgb(g: u8) -> [u8; 3] {
    if g & G_ROCK != 0 {
        return [94, 94, 93];
    }
    let soil = match g >> 4 {
        1 => [150, 134, 92],
        2 => [104, 54, 38],
        3 => [78, 76, 72],
        _ => [57, 54, 43],
    };
    if g & G_WET != 0 {
        // Water over the soil, a little of the soil showing.
        return [((22 * 3 + soil[0] as u32) / 4) as u8, ((46 * 3 + soil[1] as u32) / 4) as u8, ((62 * 3 + soil[2] as u32) / 4) as u8];
    }
    soil
}

/// Where the map sits on screen: `zoom` screen pixels a map cell, with map
/// point `origin` at screen (0, 0).
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct EditView {
    pub origin: (f32, f32),
    pub zoom: f32,
}

impl EditView {
    /// The whole map in the rectangle `(x, y, w, h)` of the screen.
    pub fn fit(rect: (f32, f32, f32, f32)) -> Self {
        let zoom = (rect.2 / MAP_W as f32).min(rect.3 / MAP_H as f32);
        let (mw, mh) = (MAP_W as f32 * zoom, MAP_H as f32 * zoom);
        let sx = rect.0 + (rect.2 - mw) / 2.0;
        let sy = rect.1 + (rect.3 - mh) / 2.0;
        EditView { origin: (-sx / zoom, -sy / zoom), zoom }
    }

    pub fn to_map(&self, s: (f32, f32)) -> (f32, f32) {
        (self.origin.0 + s.0 / self.zoom, self.origin.1 + s.1 / self.zoom)
    }

    pub fn to_screen(&self, p: (f32, f32)) -> (f32, f32) {
        ((p.0 - self.origin.0) * self.zoom, (p.1 - self.origin.1) * self.zoom)
    }

    /// Zoom by `k` keeping screen point `s` over the same map point.
    pub fn zoom_at(&mut self, k: f32, s: (f32, f32)) {
        let p = self.to_map(s);
        self.zoom = (self.zoom * k).clamp(0.25, 24.0);
        self.origin = (p.0 - s.0 / self.zoom, p.1 - s.1 / self.zoom);
    }

    pub fn pan(&mut self, ds: (f32, f32)) {
        self.origin.0 -= ds.0 / self.zoom;
        self.origin.1 -= ds.1 / self.zoom;
    }
}

/// The panel's width, logical pixels (the screen is drawn at `ui` screen
/// pixels a logical one, 2 for the crisp font).
pub const PANEL: i32 = 150;

/// What the window shows besides the map: the cursor, a drag in progress,
/// the map's name and the last message.
pub struct Overlay<'a> {
    pub cursor: Option<(f32, f32)>,
    /// A wall being dragged, from its start to the cursor.
    pub wall_from: Option<(f32, f32)>,
    pub name: &'a str,
    pub message: &'a str,
}

/// Draws the editor into an RGBA frame `size` pixels big: the map in `view`,
/// its items over it, the panel on the left and the status line below.
pub fn draw(ed: &Editor, view: &EditView, frame: &mut [u8], size: (u32, u32), o: &Overlay) {
    let (w, h) = (size.0 as usize, size.1 as usize);
    let lut: Vec<[u8; 3]> = (0..=255u8).map(ground_rgb).collect();
    // The ground, cell by cell under each pixel; off the map is dark.
    for sy in 0..h {
        let my = view.origin.1 + (sy as f32 + 0.5) / view.zoom;
        for sx in 0..w {
            let mx = view.origin.0 + (sx as f32 + 0.5) / view.zoom;
            let c = if mx >= 0.0 && my >= 0.0 && (mx as usize) < MAP_W && (my as usize) < MAP_H { lut[ed.map.ground[my as usize * MAP_W + mx as usize] as usize] } else { [14, 16, 18] };
            let i = (sy * w + sx) * 4;
            frame[i..i + 4].copy_from_slice(&[c[0], c[1], c[2], 255]);
        }
    }
    let it = &ed.map.items;
    for wl in &it.walls {
        line(frame, size, view.to_screen(wl.a), view.to_screen(wl.b), (wl.thick * view.zoom / 2.0).max(1.0), [168, 156, 132, 255]);
    }
    for c in &it.crystals {
        // The engine's crystal colour at full charge; brighter with more
        // charge a cell (the designed maps run 192 to 512).
        let k = (c.charge / 512.0).clamp(0.25, 1.0);
        let col = [(30.0 + 40.0 * k) as u8, (120.0 + 80.0 * k) as u8, (140.0 + 80.0 * k) as u8, 255];
        hexagon(frame, size, view, (c.x, c.y), c.r, col);
    }
    for (i, &m) in it.moulds.iter().enumerate() {
        let s = view.to_screen(m);
        disc(frame, size, s, 9.0, [0, 0, 0, 255]);
        disc(frame, size, s, 7.0, [232, 168, 40, 255]);
        crate::hud::draw_text_crisp(frame, size, ((s.0 / 2.0) as i32 + 6, (s.1 / 2.0) as i32 - 3), &format!("M{}", i + 1), [255, 220, 140, 255]);
    }
    if let Some(l) = it.landing {
        let s = view.to_screen(l);
        rect(frame, size, (s.0 - 9.0, s.1 - 9.0), (18.0, 18.0), [0, 0, 0, 255]);
        rect(frame, size, (s.0 - 7.0, s.1 - 7.0), (14.0, 14.0), [90, 170, 255, 255]);
        crate::hud::draw_text_crisp(frame, size, ((s.0 / 2.0) as i32 + 6, (s.1 / 2.0) as i32 - 3), "SQUAD", [180, 215, 255, 255]);
    }
    // The mirror's line or centre.
    match ed.mirror {
        Mirror::Off => {}
        Mirror::X => {
            let a = view.to_screen((MAP_W as f32 / 2.0, 0.0));
            let b = view.to_screen((MAP_W as f32 / 2.0, MAP_H as f32));
            line(frame, size, a, b, 0.6, [255, 255, 255, 90]);
        }
        Mirror::Turn => {
            let c = view.to_screen((MAP_W as f32 / 2.0, MAP_H as f32 / 2.0));
            ring(frame, size, c, 6.0, [255, 255, 255, 140]);
        }
    }
    if let (Some(a), Some(b)) = (o.wall_from, o.cursor) {
        line(frame, size, view.to_screen(a), view.to_screen(b), (WALL_THICK * view.zoom / 2.0).max(1.0), [200, 190, 160, 160]);
    }
    // What the tool would touch, under the cursor (and its mirror).
    if let Some(c) = o.cursor {
        for p in ed.both(c) {
            let s = view.to_screen(p);
            match ed.tool {
                Tool::Paint(_) => ring(frame, size, s, ed.radius * view.zoom, [255, 255, 255, 200]),
                Tool::Crystal => hex_outline(frame, size, view, p, ed.crystal_r, [180, 255, 255, 220]),
                Tool::Erase => ring(frame, size, s, PICK * view.zoom, [255, 110, 90, 200]),
                _ => ring(frame, size, s, 8.0, [255, 255, 255, 200]),
            }
        }
    }
    panel(ed, frame, size, o);
}

fn panel(ed: &Editor, frame: &mut [u8], size: (u32, u32), o: &Overlay) {
    use crate::hud::draw_text_crisp as text;
    let (w, h) = (size.0 as f32, size.1 as f32);
    rect(frame, size, (0.0, 0.0), (PANEL as f32 * 2.0, h), [20, 24, 28, 235]);
    rect(frame, size, (0.0, h - 36.0), (w, 36.0), [20, 24, 28, 235]);
    let (white, dim, accent, warn) = ([235, 235, 230, 255], [150, 156, 160, 255], [120, 220, 230, 255], [255, 170, 110, 255]);
    let mut y = 8;
    text(frame, size, (8, y), "MAP EDITOR", accent);
    y += 12;
    text(frame, size, (8, y), o.name, if ed.dirty { warn } else { dim });
    y += 16;
    for (tool, key) in Tool::ALL {
        let on = tool == ed.tool;
        if on {
            rect(frame, size, (4.0, y as f32 * 2.0 - 4.0), (PANEL as f32 * 2.0 - 8.0, 22.0), [50, 70, 80, 255]);
        }
        let swatch = match tool {
            Tool::Paint(b) => {
                let c = ground_rgb(b.apply(0));
                Some([c[0], c[1], c[2], 255])
            }
            Tool::Crystal => Some([60, 190, 210, 255]),
            Tool::Mould => Some([232, 168, 40, 255]),
            Tool::Landing => Some([90, 170, 255, 255]),
            Tool::Wall => Some([168, 156, 132, 255]),
            Tool::Erase => None,
        };
        if let Some(c) = swatch {
            rect(frame, size, (16.0, y as f32 * 2.0), (14.0, 14.0), c);
        }
        text(frame, size, (20, y), &format!("{key}  {}", tool.name()), if on { white } else { dim });
        y += 12;
    }
    y += 6;
    let size_line = match ed.tool {
        Tool::Crystal => format!("crystal r {} [ ]", ed.crystal_r),
        Tool::Paint(_) => format!("brush r {} [ ]", ed.radius),
        _ => String::new(),
    };
    text(frame, size, (8, y), &size_line, dim);
    y += 12;
    text(frame, size, (8, y), &format!("mirror {} (X)", ed.mirror.name()), dim);
    y += 18;
    for line in ["CTRL+S save", "CTRL+Z undo", "CTRL+Y redo", "ENTER save + play", "F fit view", "WHEEL zoom", "RIGHT drag pan", "N new blank map"] {
        text(frame, size, (8, y), line, dim);
        y += 11;
    }
    y += 8;
    let it = &ed.map.items;
    text(frame, size, (8, y), &format!("{} crystals", it.crystals.len()), dim);
    y += 11;
    text(frame, size, (8, y), &format!("{} mould starts", it.moulds.len()), dim);
    y += 11;
    text(frame, size, (8, y), &format!("{} walls", it.walls.len()), dim);
    y += 14;
    let problems = ed.problems();
    if problems.is_empty() {
        text(frame, size, (8, y), "ready to play", accent);
    } else {
        for p in &problems {
            text(frame, size, (8, y), p, warn);
            y += 11;
        }
    }
    // The status line: what is under the cursor, then the last message.
    let at = o.cursor.map(|c| format!("{} {}  {}", c.0 as i32, c.1 as i32, ed.describe(c))).unwrap_or_default();
    let sy = (h as i32 - 36) / 2 + 4;
    text(frame, size, (PANEL + 8, sy), &at, dim);
    text(frame, size, (PANEL + 160, sy), o.message, white);
}

/// Blends `c` (its alpha) over pixel (x, y).
fn blend(frame: &mut [u8], size: (u32, u32), x: i32, y: i32, c: [u8; 4]) {
    if x < 0 || y < 0 || x >= size.0 as i32 || y >= size.1 as i32 {
        return;
    }
    let i = (y as usize * size.0 as usize + x as usize) * 4;
    let a = c[3] as u32;
    for k in 0..3 {
        frame[i + k] = ((c[k] as u32 * a + frame[i + k] as u32 * (255 - a)) / 255) as u8;
    }
    frame[i + 3] = 255;
}

fn rect(frame: &mut [u8], size: (u32, u32), at: (f32, f32), wh: (f32, f32), c: [u8; 4]) {
    for y in at.1 as i32..(at.1 + wh.1) as i32 {
        for x in at.0 as i32..(at.0 + wh.0) as i32 {
            blend(frame, size, x, y, c);
        }
    }
}

fn disc(frame: &mut [u8], size: (u32, u32), at: (f32, f32), r: f32, c: [u8; 4]) {
    for y in (at.1 - r) as i32..=(at.1 + r) as i32 {
        for x in (at.0 - r) as i32..=(at.0 + r) as i32 {
            if (x as f32 + 0.5 - at.0).hypot(y as f32 + 0.5 - at.1) <= r {
                blend(frame, size, x, y, c);
            }
        }
    }
}

fn ring(frame: &mut [u8], size: (u32, u32), at: (f32, f32), r: f32, c: [u8; 4]) {
    let r = r.max(2.0);
    for y in (at.1 - r - 1.0) as i32..=(at.1 + r + 1.0) as i32 {
        for x in (at.0 - r - 1.0) as i32..=(at.0 + r + 1.0) as i32 {
            if ((x as f32 + 0.5 - at.0).hypot(y as f32 + 0.5 - at.1) - r).abs() <= 0.75 {
                blend(frame, size, x, y, c);
            }
        }
    }
}

/// A line `half` pixels either side of a to b.
fn line(frame: &mut [u8], size: (u32, u32), a: (f32, f32), b: (f32, f32), half: f32, c: [u8; 4]) {
    let (x0, x1) = ((a.0.min(b.0) - half - 1.0) as i32, (a.0.max(b.0) + half + 1.0) as i32);
    let (y0, y1) = ((a.1.min(b.1) - half - 1.0) as i32, (a.1.max(b.1) + half + 1.0) as i32);
    let (x0, x1) = (x0.max(0), x1.min(size.0 as i32));
    let (y0, y1) = (y0.max(0), y1.min(size.1 as i32));
    for y in y0..y1 {
        for x in x0..x1 {
            if segment_distance((x as f32 + 0.5, y as f32 + 0.5), a, b) <= half {
                blend(frame, size, x, y, c);
            }
        }
    }
}

fn hexagon(frame: &mut [u8], size: (u32, u32), view: &EditView, centre: (f32, f32), r: f32, c: [u8; 4]) {
    let s = view.to_screen(centre);
    let rs = r * view.zoom;
    let (x0, x1) = (((s.0 - rs) as i32).max(0), ((s.0 + rs) as i32 + 1).min(size.0 as i32));
    let (y0, y1) = (((s.1 - rs) as i32).max(0), ((s.1 + rs) as i32 + 1).min(size.1 as i32));
    for y in y0..y1 {
        for x in x0..x1 {
            let (dx, dy) = ((x as f32 + 0.5 - s.0) / view.zoom, (y as f32 + 0.5 - s.1) / view.zoom);
            if in_hex(dx, dy, r) {
                // Brightest deep inside, as the engine's glow.
                let edge = !in_hex(dx, dy, r * 0.8);
                let col = if edge { [c[0] / 2, c[1] / 2, c[2] / 2, 255] } else { c };
                blend(frame, size, x, y, col);
            }
        }
    }
}

fn hex_outline(frame: &mut [u8], size: (u32, u32), view: &EditView, centre: (f32, f32), r: f32, c: [u8; 4]) {
    let corner = |k: usize| {
        let a = std::f32::consts::FRAC_PI_3 * k as f32 - std::f32::consts::FRAC_PI_2;
        view.to_screen((centre.0 + r * a.cos(), centre.1 + r * a.sin()))
    };
    for k in 0..6 {
        line(frame, size, corner(k), corner(k + 1), 0.8, c);
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn count(m: &Map, g: u8) -> usize {
        m.ground.iter().filter(|&&v| v == g).count()
    }

    #[test]
    fn brushes_write_the_ground_byte_the_loader_reads() {
        assert_eq!(Brush::Sand.apply(G_ROCK), 0x10);
        assert_eq!(Brush::Clay.apply(G_WET), 0x20 | G_WET);
        assert_eq!(Brush::Barren.apply(0x10), 0x30);
        assert_eq!(Brush::Rock.apply(0x21), 0x20 | G_ROCK);
        assert_eq!(Brush::Wet.apply(0x18), 0x10 | G_WET);
        assert_eq!(Brush::Plain.apply(0x39), 0);
    }

    #[test]
    fn a_stamp_is_a_disc_and_a_stroke_has_no_gaps() {
        let mut ed = Editor::new(Editor::blank());
        ed.radius = 10.0;
        ed.stamp(Brush::Clay, (100.0, 100.0));
        let n = count(&ed.map, 0x20) as f32;
        assert!((n - std::f32::consts::PI * 100.0).abs() < 20.0, "{n} cells");
        // A long fast drag: every cell on the line is painted.
        ed.stroke(Brush::Sand, (200.0, 300.0), (800.0, 300.0));
        assert!((200..=800).all(|x| ed.map.ground[300 * MAP_W + x] == 0x10));
        // Off the map is clipped, not a panic.
        ed.stamp(Brush::Rock, (-5.0, MAP_H as f32 + 3.0));
        assert!(ed.map.ground[(MAP_H - 1) * MAP_W] & G_ROCK != 0);
    }

    #[test]
    fn mirror_doubles_strokes_and_items_but_not_the_landing() {
        let mut ed = Editor::new(Editor::blank());
        ed.mirror = Mirror::Turn;
        ed.radius = 4.0;
        ed.stamp(Brush::Rock, (100.0, 100.0));
        assert!(ed.map.ground[100 * MAP_W + 100] & G_ROCK != 0);
        assert!(ed.map.ground[(MAP_H - 100) * MAP_W + MAP_W - 100] & G_ROCK != 0);
        ed.tool = Tool::Crystal;
        ed.click((200.0, 150.0));
        assert_eq!(ed.map.items.crystals.iter().map(|c| (c.x, c.y)).collect::<Vec<_>>(), vec![(200.0, 150.0), (824.0, 490.0)]);
        ed.tool = Tool::Landing;
        ed.click((512.0, 320.0));
        assert_eq!(ed.map.items.landing, Some((512.0, 320.0)));
        ed.mirror = Mirror::X;
        ed.tool = Tool::Mould;
        ed.click((100.0, 50.0));
        assert_eq!(ed.map.items.moulds, vec![(100.0, 50.0), (924.0, 50.0)]);
        ed.wall((10.0, 10.0), (60.0, 10.0));
        assert_eq!(ed.map.items.walls.len(), 2);
        assert_eq!(ed.map.items.walls[1].a, (1014.0, 10.0));
    }

    #[test]
    fn erase_takes_the_nearest_item_and_nothing_far() {
        let mut ed = Editor::new(Editor::blank());
        ed.tool = Tool::Crystal;
        ed.click((300.0, 300.0));
        ed.tool = Tool::Mould;
        ed.click((380.0, 300.0));
        ed.wall((500.0, 100.0), (500.0, 500.0));
        assert_eq!(ed.erase((700.0, 300.0)), None);
        // 10 cells from the crystal's edge, 50 from the start: the crystal.
        assert_eq!(ed.erase((340.0, 300.0)), Some("crystal"));
        assert_eq!(ed.erase((360.0, 300.0)), Some("mould start"));
        assert_eq!(ed.erase((510.0, 250.0)), Some("wall"));
        assert_eq!(ed.map.items, Items::default());
    }

    #[test]
    fn undo_and_redo_step_through_actions() {
        let mut ed = Editor::new(Editor::blank());
        ed.begin();
        ed.stamp(Brush::Sand, (50.0, 50.0));
        ed.begin();
        ed.tool = Tool::Landing;
        ed.click((60.0, 60.0));
        assert!(ed.undo());
        assert_eq!(ed.map.items.landing, None);
        assert!(count(&ed.map, 0x10) > 0);
        assert!(ed.undo());
        assert_eq!(count(&ed.map, 0x10), 0);
        assert!(!ed.undo());
        assert!(ed.redo() && ed.redo());
        assert_eq!(ed.map.items.landing, Some((60.0, 60.0)));
        for _ in 0..UNDO + 10 {
            ed.begin();
        }
        assert_eq!(ed.undo.len(), UNDO);
    }

    #[test]
    fn problems_name_what_a_playable_map_lacks() {
        let mut ed = Editor::new(Editor::blank());
        assert_eq!(ed.problems().len(), 3);
        ed.tool = Tool::Crystal;
        ed.click((100.0, 100.0));
        ed.tool = Tool::Mould;
        ed.click((120.0, 100.0));
        ed.tool = Tool::Landing;
        ed.click((800.0, 500.0));
        assert!(ed.problems().is_empty());
        ed.radius = 5.0;
        ed.stamp(Brush::Rock, (800.0, 500.0));
        assert_eq!(ed.problems(), vec!["the landing is on rock".to_string()]);
    }

    /// The editor's map goes through the loader's files and comes back the
    /// same, so what was drawn is what plays.
    #[test]
    fn an_edited_map_saves_and_loads_the_same() {
        let mut ed = Editor::new(Editor::blank());
        ed.radius = 40.0;
        ed.stroke(Brush::Clay, (100.0, 100.0), (900.0, 500.0));
        ed.stamp(Brush::Wet, (512.0, 320.0));
        ed.tool = Tool::Crystal;
        ed.click((300.0, 200.0));
        ed.tool = Tool::Mould;
        ed.click((320.0, 500.0));
        ed.tool = Tool::Landing;
        ed.click((700.0, 150.0));
        ed.wall((400.0, 100.0), (400.0, 300.0));
        let dir = std::env::temp_dir().join(format!("map-edit-test-{}", std::process::id()));
        std::fs::create_dir_all(&dir).unwrap();
        ed.map.save(&dir, "drawn").unwrap();
        let back = Map::load(&dir, "drawn").unwrap();
        std::fs::remove_dir_all(&dir).ok();
        assert_eq!(back, ed.map);
    }

    #[test]
    fn the_view_fits_and_zooms_about_the_cursor() {
        let v = EditView::fit((300.0, 0.0, 1024.0, 640.0));
        assert_eq!(v.to_screen((0.0, 0.0)), (300.0, 0.0));
        assert_eq!(v.to_map((300.0 + 512.0, 320.0)), (512.0, 320.0));
        let mut v = v;
        let s = (700.0, 200.0);
        let p = v.to_map(s);
        v.zoom_at(2.0, s);
        let q = v.to_map(s);
        assert!((p.0 - q.0).abs() < 1e-3 && (p.1 - q.1).abs() < 1e-3);
    }

    #[test]
    fn the_panel_uses_only_glyphs_the_font_has() {
        let mut texts: Vec<String> = Tool::ALL.iter().map(|(t, k)| format!("{k}  {}", t.name())).collect();
        texts.extend(["MAP EDITOR", "CTRL+S save", "ENTER save + play", "RIGHT drag pan", "ready to play", "no crystal (C)", "mould start 1 is on rock", "the landing is on rock", "crystal r 30 [ ]", "mirror left-right (X)", "mirror half turn (X)", "SQUAD", "plain wet"].map(String::from));
        for t in texts {
            assert!(t.chars().all(crate::hud::has_crisp_glyph), "{t:?}");
        }
    }
}
