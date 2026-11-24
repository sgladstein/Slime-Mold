//! **The mouse cursor says what a click will do** (owner, 2026-11-21
//! 07:32Z: "Cursor should be crosshair when controlling a squad and should
//! obviously change when you have other options selected. This needs to
//! look polished"). Until then the window kept the system's arrow in every
//! mode, so aiming a squad, placing a wall and throwing a blast all looked
//! like pointing at a menu.
//!
//! **Hardware cursors, drawn here.** Each look is a small vector picture
//! rasterised with `tiny_skia` when the window opens (and again when the UI
//! size or the screen's DPI changes) and handed to the system as a custom
//! cursor (`winit::window::CustomCursor`). The system draws it, so it moves
//! with the mouse at the screen's own rate and never lags a frame behind,
//! which a cursor drawn into the HUD would (the aim is where the squad
//! fires, so a lagging crosshair would read as an inaccurate gun). The
//! cost is nothing a frame: `App::update_cursor` works out the look and
//! calls `set_cursor` only when it changes.
//!
//! **The look** (`wiki/looks/art-direction.md`): every cursor has the same
//! build, a dark outline 1.5 units wide all round, a soft drop shadow down
//! and to the right, and a faint top-lit gradient on each part (the slime's
//! key light is top left too), so they read on the dark ground, on bright
//! slime and over the HUD's glass alike. Colours keep their roles: cream
//! and the HUD's one accent blue for your own pointing and aiming, the
//! danger red only for attacking and firing, amber for building, cyan only
//! where a crystal is the target (pump, drain), hazard orange for the
//! thrown devices. The hot spot is always where the picture points: the
//! arrow's tip, the crosshair's middle, the placement brackets' middle, a
//! wall's first end.
//!
//! **Which look when** (`App::cursor_look`), first match wins:
//! menus and the HUD give the arrow, or the lit arrow over anything a click
//! works on; a middle-button pan gives the four arrows; a box being dragged
//! gives the box; a card command waiting for its point gives that
//! command's look (wall, door, pump, building, device, attack, guard,
//! drain, rally, or a plain order mark); in the squad mode a commanded
//! squad gives the crosshair, tightened and red-hot while the left button
//! fires; with no squad commanded, with Ctrl held (a click then picks
//! squads), or in the RTS round, the arrow, with a selection badge over a
//! squad or soldier a click would take.

use super::*;
use tiny_skia::{Color, FillRule, GradientStop, LinearGradient, Paint, Path, PathBuilder, Pixmap, Point, Shader, SpreadMode, Stroke, LineCap, LineJoin, Transform};
use winit::window::CustomCursor;

/// What the cursor shows: one look for each thing a click can do.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub(super) enum Look {
    /// Menus, the HUD's glass, the world with nothing commanded.
    Arrow,
    /// Over a button or anything else a click works on (the HUD, the menus).
    Button,
    /// Over a squad or soldier a click takes command of or selects.
    Select,
    /// A squad commanded: where it fires.
    Aim,
    /// The left button held: the squad is firing.
    Fire,
    /// Attack-move waiting for its point.
    Attack,
    /// Any other order waiting for its point.
    Order,
    /// Guard waiting for its point.
    Guard,
    /// Drain (harvest) waiting for its crystal.
    Drain,
    /// The rally point waiting for its place.
    Rally,
    /// A building waiting for its place (turret, mortar, lure, siren, outpost).
    Build,
    /// A pump waiting for its crystal.
    Pump,
    /// A wall waiting for its ends.
    Wall,
    /// A door waiting for its wall.
    Door,
    /// A thrown device (blast, det cord, mine, germ, gas, and the squad's
    /// device slot) waiting for its point.
    Device,
    /// A selection box being dragged.
    Boxing,
    /// The camera being dragged with the middle button.
    Pan,
}

impl Look {
    pub(super) const ALL: [Look; 17] = [Look::Arrow, Look::Button, Look::Select, Look::Aim, Look::Fire, Look::Attack, Look::Order, Look::Guard, Look::Drain, Look::Rally, Look::Build, Look::Pump, Look::Wall, Look::Door, Look::Device, Look::Boxing, Look::Pan];

    #[cfg(test)]
    pub(super) fn name(self) -> &'static str {
        match self {
            Look::Arrow => "Arrow",
            Look::Button => "Button",
            Look::Select => "Select",
            Look::Aim => "Aim",
            Look::Fire => "Fire",
            Look::Attack => "Attack",
            Look::Order => "Order",
            Look::Guard => "Guard",
            Look::Drain => "Drain",
            Look::Rally => "Rally",
            Look::Build => "Build",
            Look::Pump => "Pump",
            Look::Wall => "Wall",
            Look::Door => "Door",
            Look::Device => "Device",
            Look::Boxing => "Box select",
            Look::Pan => "Pan",
        }
    }
}

/// The look a card command gives while it waits for its point. Commands
/// not named here fall back by their target: a line is a wall's look, a
/// point or area a plain order mark. A device slot or any new thrown tool
/// should be added to the device arm.
pub(super) fn target_look(id: u32, target: Target) -> Look {
    match id {
        WALL => Look::Wall,
        DOOR => Look::Door,
        PUMP => Look::Pump,
        ATTACK => Look::Attack,
        GUARD => Look::Guard,
        HARVEST => Look::Drain,
        RALLY => Look::Rally,
        T_BLAST | T_CORD | T_MINE | T_GERM | T_GAS | T_PROVOKE | T_STILL => Look::Device,
        _ if build_kind(id).is_some() => Look::Build,
        _ => match target {
            Target::Line { .. } => Look::Wall,
            Target::Area { .. } => Look::Build,
            _ => Look::Order,
        },
    }
}

/// The cursors made for one size, and the one showing.
pub(super) struct Cursors {
    scale: f32,
    made: Vec<(Look, CustomCursor)>,
    now: Option<Look>,
}

/// The cursor's size over the UI size (`UiSize`: the player's size times
/// the screen's DPI), held between 1 and 2.5: 32 to 80 pixels. Above 2.5
/// a cursor starts to hide what it points at.
fn cursor_scale(ui: UiSize) -> f32 {
    let s = (ui.user * ui.dpi).clamp(1.0, 2.5);
    // In eighths, so a UI size nudged by a hair does not remake them all.
    (s * 8.0).round() / 8.0
}

impl App {
    /// The look for this frame (the module's *Which look when*).
    pub(super) fn cursor_look(&self) -> Look {
        if let Some(m) = &self.menu {
            return if m.over_button() { Look::Button } else { Look::Arrow };
        }
        let Some(p) = self.controls.cursor else { return Look::Arrow };
        if self.controls.panning() {
            return Look::Pan;
        }
        if self.controls.boxing() {
            return Look::Boxing;
        }
        let minimap = self.hud.on_minimap(p).is_some();
        if self.hud.over(p) && !minimap {
            return if self.hud.clickable(p) { Look::Button } else { Look::Arrow };
        }
        let Some(g) = self.game.as_ref() else { return Look::Arrow };
        if g.end.is_some() {
            return Look::Arrow;
        }
        // A command waiting for its point: on the minimap too, where it
        // places through the map (`Hud::aim`).
        if let Some((id, target, _)) = self.hud.targeting() {
            return target_look(id, target);
        }
        if minimap {
            return Look::Arrow;
        }
        let world = self.camera.world_at(p);
        if let Some(sq) = &self.sq {
            // Ctrl turns the left button from firing to picking squads
            // (`squad_button`: a Ctrl click takes the squad under it, a
            // Ctrl drag boxes several), so the crosshair gives way to the
            // pick's look while it is held.
            if g.active.is_some() && !sq.free && !(self.ctrl && !sq.firing()) {
                return if sq.firing() { Look::Fire } else { Look::Aim };
            }
            return if world.and_then(|w| self.squad_at(w)).is_some() { Look::Select } else { Look::Arrow };
        }
        if self.controls.pick(&self.units(), &self.camera, p).is_some() {
            return Look::Select;
        }
        Look::Arrow
    }

    /// Shows this frame's look (`cursor_look`), making the cursors first
    /// when the window opens or its UI size changes. Called every frame; a
    /// frame whose look has not changed costs one comparison and a few
    /// lookups.
    pub(super) fn update_cursor(&mut self, event_loop: &ActiveEventLoop) {
        let (Some(window), Some(canvas)) = (self.window.as_ref(), self.canvas.as_ref()) else { return };
        let scale = cursor_scale(canvas.ui);
        if self.cursors.as_ref().is_none_or(|c| c.scale != scale) {
            let made = Look::ALL
                .iter()
                .filter_map(|&look| {
                    let a = art(look, scale);
                    let source = CustomCursor::from_rgba(a.rgba, a.size as u16, a.size as u16, a.hot.0, a.hot.1).ok()?;
                    Some((look, event_loop.create_custom_cursor(source)))
                })
                .collect();
            self.cursors = Some(Cursors { scale, made, now: None });
        }
        let look = self.cursor_look();
        let Some(c) = self.cursors.as_mut() else { return };
        if c.now == Some(look) {
            return;
        }
        if let Some((_, cursor)) = c.made.iter().find(|(l, _)| *l == look) {
            window.set_cursor(cursor.clone());
            c.now = Some(look);
        }
    }
}

/// One cursor's picture: straight (not premultiplied) RGBA, square, and
/// its hot spot in pixels.
pub(super) struct Art {
    pub(super) rgba: Vec<u8>,
    pub(super) size: u32,
    pub(super) hot: (u16, u16),
}

type Rgb = [u8; 3];
const CREAM: Rgb = [246, 240, 226];
const INK: Rgb = [10, 12, 16];
/// The HUD's accent (`theme::ACCENT`), a touch lighter to read at 2 px.
const BLUE: Rgb = [120, 202, 255];
/// The danger red (`theme::BAD`).
const RED: Rgb = [242, 98, 98];
/// A firing crosshair's hot white-orange.
const HOT: Rgb = [255, 186, 120];
/// Building (`theme::WARN`).
const AMBER: Rgb = [242, 192, 84];
/// Crystals (`theme::ENERGY`).
const CYAN: Rgb = [128, 224, 255];
/// Guarding (`theme::GOOD`).
const GREEN: Rgb = [94, 214, 132];
/// The thrown devices.
const HAZARD: Rgb = [255, 140, 66];
/// A wall's warm grey (the walls' own hue).
const STONE: Rgb = [190, 178, 160];

/// The design grid: every look is drawn on 32 units, one unit a pixel at
/// scale 1.
const GRID: f32 = 32.0;
/// The outline's width outside every part, units.
const OUTLINE: f32 = 1.5;

/// One part of a look, drawn in order over the outline and shadow of all
/// of them.
enum Part {
    /// A filled shape.
    Fill(Path, Rgb, f32),
    /// A stroked line, `width` units.
    Line(Path, f32, Rgb, f32),
    /// A detail drawn on top with no outline of its own (a facet, a joint).
    Detail(Path, f32, Rgb, f32),
}

fn fill(p: Option<Path>, c: Rgb) -> Option<Part> {
    Some(Part::Fill(p?, c, 1.0))
}

fn line(p: Option<Path>, w: f32, c: Rgb) -> Option<Part> {
    Some(Part::Line(p?, w, c, 1.0))
}

fn detail(p: Option<Path>, w: f32, c: Rgb, a: f32) -> Option<Part> {
    Some(Part::Detail(p?, w, c, a))
}

fn poly(pts: &[(f32, f32)]) -> Option<Path> {
    let mut b = PathBuilder::new();
    let (&first, rest) = pts.split_first()?;
    b.move_to(first.0, first.1);
    for &(x, y) in rest {
        b.line_to(x, y);
    }
    b.close();
    b.finish()
}

fn open(pts: &[(f32, f32)]) -> Option<Path> {
    let mut b = PathBuilder::new();
    let (&first, rest) = pts.split_first()?;
    b.move_to(first.0, first.1);
    for &(x, y) in rest {
        b.line_to(x, y);
    }
    b.finish()
}

fn seg(a: (f32, f32), b: (f32, f32)) -> Option<Path> {
    open(&[a, b])
}

fn disc(c: (f32, f32), r: f32) -> Option<Path> {
    PathBuilder::from_circle(c.0, c.1, r)
}

/// An arc about `c`, radius `r`, from angle `a0` to `a1` (degrees, 0 to
/// the right, 90 down).
fn arc(c: (f32, f32), r: f32, a0: f32, a1: f32) -> Option<Path> {
    let n = (((a1 - a0).abs() / 6.0).ceil() as usize).max(2);
    let pts: Vec<(f32, f32)> = (0..=n)
        .map(|i| {
            let a = (a0 + (a1 - a0) * i as f32 / n as f32).to_radians();
            (c.0 + r * a.cos(), c.1 + r * a.sin())
        })
        .collect();
    open(&pts)
}

/// A hexagon (crystals are hexagons, owner 11-11), point up.
fn hexagon(c: (f32, f32), r: f32) -> Option<Path> {
    let pts: Vec<(f32, f32)> = (0..6).map(|i| (-90.0 + 60.0 * i as f32).to_radians()).map(|a| (c.0 + r * a.cos(), c.1 + r * a.sin())).collect();
    poly(&pts)
}

/// Placement brackets: the four corners of the square `lo..hi`, arms `arm`
/// long.
fn brackets(lo: f32, hi: f32, arm: f32, w: f32, c: Rgb) -> Vec<Option<Part>> {
    vec![
        line(open(&[(lo, lo + arm), (lo, lo), (lo + arm, lo)]), w, c),
        line(open(&[(hi - arm, lo), (hi, lo), (hi, lo + arm)]), w, c),
        line(open(&[(hi, hi - arm), (hi, hi), (hi - arm, hi)]), w, c),
        line(open(&[(lo + arm, hi), (lo, hi), (lo, hi - arm)]), w, c),
    ]
}

/// The four ticks of a crosshair about `c`, from radius `r0` to `r1`.
fn ticks(c: (f32, f32), r0: f32, r1: f32, w: f32, col: Rgb) -> Vec<Option<Part>> {
    [(1.0, 0.0), (-1.0, 0.0), (0.0, 1.0), (0.0, -1.0)].iter().map(|&(dx, dy)| line(seg((c.0 + dx * r0, c.1 + dy * r0), (c.0 + dx * r1, c.1 + dy * r1)), w, col)).collect()
}

/// The small target mark an order's badge hangs off: its hot spot.
fn pip(c: (f32, f32)) -> Vec<Option<Part>> {
    vec![line(disc(c, 3.4), 1.6, CREAM), fill(disc(c, 1.1), CREAM)]
}

/// The arrow, tip at (4, 3).
fn arrow_path() -> Option<Path> {
    poly(&[(4.0, 3.0), (4.0, 25.5), (9.3, 20.7), (13.1, 28.7), (16.7, 27.1), (13.0, 19.3), (20.2, 19.3)])
}

/// A look's parts and its hot spot, in grid units.
fn design(look: Look) -> (Vec<Option<Part>>, (f32, f32)) {
    let mid = (16.0, 16.0);
    match look {
        Look::Arrow => (vec![fill(arrow_path(), CREAM)], (4.0, 3.0)),
        Look::Button => (vec![fill(arrow_path(), BLUE), detail(seg((5.6, 7.0), (5.6, 21.0)), 1.1, [255, 255, 255], 0.55)], (4.0, 3.0)),
        Look::Select => {
            let mut v = vec![fill(arrow_path(), CREAM)];
            v.extend(brackets(19.0, 29.0, 3.6, 1.8, BLUE));
            (v, (4.0, 3.0))
        }
        Look::Aim => {
            let mut v = Vec::new();
            for k in 0..4 {
                let a = 45.0 + 90.0 * k as f32;
                v.push(line(arc(mid, 12.0, a - 22.0, a + 22.0), 1.6, CREAM));
            }
            v.extend(ticks(mid, 4.5, 10.0, 2.0, CREAM));
            v.push(fill(disc(mid, 1.5), BLUE));
            (v, mid)
        }
        Look::Fire => {
            let mut v = Vec::new();
            for k in 0..4 {
                let a = 45.0 + 90.0 * k as f32;
                v.push(line(arc(mid, 10.0, a - 30.0, a + 30.0), 1.8, HOT));
            }
            v.extend(ticks(mid, 3.0, 8.0, 2.0, HOT));
            v.push(fill(disc(mid, 1.8), RED));
            (v, mid)
        }
        Look::Attack => {
            let mut v = vec![line(disc(mid, 10.0), 1.8, RED)];
            v.extend(ticks(mid, 5.5, 14.0, 2.0, CREAM));
            v.push(fill(disc(mid, 1.5), RED));
            (v, mid)
        }
        Look::Order => (vec![line(disc(mid, 6.0), 1.8, CREAM), fill(disc(mid, 1.6), CREAM)], mid),
        Look::Guard => {
            let at = (8.0, 8.0);
            let mut v = pip(at);
            v.push(fill(poly(&[(15.5, 15.5), (21.5, 13.5), (27.5, 15.5), (27.5, 21.0), (21.5, 28.5), (15.5, 21.0)]), GREEN));
            // The shield's lit half (the key light is top left).
            v.push(detail(poly(&[(17.0, 16.6), (21.5, 15.1), (21.5, 26.0), (17.0, 20.6)]), 0.0, [255, 255, 255], 0.28));
            (v, at)
        }
        Look::Drain => {
            let at = (8.0, 8.0);
            let mut v = pip(at);
            v.push(fill(hexagon((21.5, 21.0), 7.0), CYAN));
            // An arrow down into the crystal: it is drawn from.
            v.push(detail(seg((21.5, 16.5), (21.5, 24.5)), 1.7, INK, 0.8));
            v.push(detail(open(&[(18.6, 21.8), (21.5, 24.7), (24.4, 21.8)]), 1.7, INK, 0.8));
            (v, at)
        }
        Look::Rally => {
            let at = (8.0, 8.0);
            let mut v = pip(at);
            v.push(line(seg((17.5, 29.0), (17.5, 13.5)), 1.8, CREAM));
            v.push(fill(poly(&[(17.5, 13.0), (28.5, 16.8), (17.5, 21.2)]), BLUE));
            (v, at)
        }
        Look::Build => {
            let mut v = brackets(5.5, 26.5, 5.5, 2.2, AMBER);
            v.push(line(seg((16.0, 12.0), (16.0, 20.0)), 1.8, CREAM));
            v.push(line(seg((12.0, 16.0), (20.0, 16.0)), 1.8, CREAM));
            (v, mid)
        }
        Look::Pump => {
            let mut v = brackets(5.5, 26.5, 5.5, 2.2, AMBER);
            v.push(fill(hexagon(mid, 6.0), CYAN));
            // Its facets, lit from the top left.
            v.push(detail(open(&[(16.0, 10.0), (16.0, 16.0), (21.2, 19.0)]), 0.9, INK, 0.35));
            v.push(detail(seg((16.0, 16.0), (10.8, 19.0)), 0.9, INK, 0.35));
            (v, mid)
        }
        Look::Wall => {
            let (a, b) = ((8.5, 8.5), (26.0, 26.0));
            let mut v = vec![line(seg(a, b), 6.0, STONE)];
            // Joints across the wall: it is built of blocks.
            for t in [0.36, 0.68] {
                let p = (a.0 + (b.0 - a.0) * t, a.1 + (b.1 - a.1) * t);
                v.push(detail(seg((p.0 - 2.4, p.1 + 2.4), (p.0 + 2.4, p.1 - 2.4)), 1.0, INK, 0.55));
            }
            let at = (5.0, 5.0);
            v.push(fill(disc(at, 3.2), AMBER));
            v.push(detail(disc(at, 1.0), 0.0, INK, 0.75));
            (v, at)
        }
        Look::Door => {
            // The plan view's door: a gap in a wall, its leaf, and its swing.
            let y = 20.0;
            let v = vec![
                line(seg((3.0, y), (9.0, y)), 4.5, STONE),
                line(seg((23.0, y), (29.0, y)), 4.5, STONE),
                line(arc((9.5, y), 13.0, -90.0, 0.0), 1.3, CREAM),
                line(seg((9.5, y), (9.5, y - 13.0)), 2.4, AMBER),
                // Where it goes: the middle of the doorway.
                fill(disc((16.0, y), 1.4), CREAM),
            ];
            (v, (16.0, y))
        }
        Look::Device => {
            let mut v = Vec::new();
            for k in 0..8 {
                let a = 22.5 + 45.0 * k as f32;
                v.push(line(arc(mid, 11.0, a - 12.0, a + 12.0), 2.0, HAZARD));
            }
            v.push(line(disc(mid, 4.5), 1.4, CREAM));
            v.push(fill(disc(mid, 1.4), HAZARD));
            (v, mid)
        }
        Look::Boxing => {
            let at = (7.0, 7.0);
            let mut v = vec![line(seg((at.0 - 3.5, at.1), (at.0 + 3.5, at.1)), 1.6, CREAM), line(seg((at.0, at.1 - 3.5), (at.0, at.1 + 3.5)), 1.6, CREAM)];
            v.push(detail(poly(&[(13.0, 13.0), (28.0, 13.0), (28.0, 26.0), (13.0, 26.0)]), 0.0, BLUE, 0.22));
            // A dashed box, dashes running round from its top left corner.
            let corners: [(f32, f32); 5] = [(13.0, 13.0), (28.0, 13.0), (28.0, 26.0), (13.0, 26.0), (13.0, 13.0)];
            for w in corners.windows(2) {
                let (p, q) = (w[0], w[1]);
                let len = (q.0 - p.0).hypot(q.1 - p.1);
                let n = (len / 5.0).round().max(1.0) as usize;
                for i in 0..n {
                    let t0 = i as f32 / n as f32;
                    let t1 = t0 + 0.6 / n as f32;
                    let at = |t: f32| (p.0 + (q.0 - p.0) * t, p.1 + (q.1 - p.1) * t);
                    v.push(line(seg(at(t0), at(t1)), 1.6, BLUE));
                }
            }
            (v, at)
        }
        Look::Pan => {
            let mut v = vec![line(seg((16.0, 8.0), (16.0, 24.0)), 1.6, CREAM), line(seg((8.0, 16.0), (24.0, 16.0)), 1.6, CREAM)];
            v.push(fill(poly(&[(16.0, 2.5), (20.5, 8.5), (11.5, 8.5)]), CREAM));
            v.push(fill(poly(&[(16.0, 29.5), (11.5, 23.5), (20.5, 23.5)]), CREAM));
            v.push(fill(poly(&[(2.5, 16.0), (8.5, 11.5), (8.5, 20.5)]), CREAM));
            v.push(fill(poly(&[(29.5, 16.0), (23.5, 20.5), (23.5, 11.5)]), CREAM));
            v.push(fill(disc(mid, 2.0), BLUE));
            (v, mid)
        }
    }
}

fn colour(c: Rgb, a: f32) -> Color {
    Color::from_rgba8(c[0], c[1], c[2], (a.clamp(0.0, 1.0) * 255.0).round() as u8)
}

/// A part's paint: lit from the top, about 10% brighter at the top of
/// the part than its colour and 10% darker at the bottom.
fn lit(path: &Path, c: Rgb, a: f32) -> Paint<'static> {
    let b = path.bounds();
    let shade = |k: f32| [(c[0] as f32 * k).min(255.0) as u8, (c[1] as f32 * k).min(255.0) as u8, (c[2] as f32 * k).min(255.0) as u8];
    let shader = if b.height() > 1.0 {
        LinearGradient::new(Point::from_xy(0.0, b.top()), Point::from_xy(0.0, b.bottom()), vec![GradientStop::new(0.0, colour(shade(1.1), a)), GradientStop::new(1.0, colour(shade(0.88), a))], SpreadMode::Pad, Transform::identity())
    } else {
        None
    };
    let mut p = Paint { anti_alias: true, ..Paint::default() };
    p.shader = shader.unwrap_or(Shader::SolidColor(colour(c, a)));
    p
}

fn solid(c: Rgb, a: f32) -> Paint<'static> {
    let mut p = Paint { anti_alias: true, ..Paint::default() };
    p.set_color(colour(c, a));
    p
}

fn stroke(w: f32) -> Stroke {
    Stroke { width: w, line_cap: LineCap::Round, line_join: LineJoin::Round, ..Stroke::default() }
}

/// A look drawn at `scale` (pixels a grid unit).
pub(super) fn art(look: Look, scale: f32) -> Art {
    let size = (GRID * scale).round().max(1.0) as u32;
    let t = Transform::from_scale(scale, scale);
    let (parts, hot) = design(look);
    let parts: Vec<Part> = parts.into_iter().flatten().collect();
    let mut out = Pixmap::new(size, size).expect("cursor size");
    // The outline: every part but the details, grown by `OUTLINE`.
    let mut edge = Pixmap::new(size, size).expect("cursor size");
    let ink = solid(INK, 1.0);
    for p in &parts {
        match p {
            Part::Fill(path, ..) => {
                edge.fill_path(path, &ink, FillRule::Winding, t, None);
                edge.stroke_path(path, &ink, &stroke(2.0 * OUTLINE), t, None);
            }
            Part::Line(path, w, ..) => edge.stroke_path(path, &ink, &stroke(w + 2.0 * OUTLINE), t, None),
            Part::Detail(..) => {}
        }
    }
    // The shadow: the outline's shape, blurred, down and to the right.
    let shadow = blur(&edge, (1.4 * scale).max(1.0), (0.8 * scale, 1.3 * scale));
    for (o, s) in out.pixels_mut().iter_mut().zip(&shadow) {
        let a = (s * 0.42 * 255.0).round() as u8;
        *o = tiny_skia::PremultipliedColorU8::from_rgba(0, 0, 0, a).expect("black is premultiplied");
    }
    out.draw_pixmap(0, 0, edge.as_ref(), &tiny_skia::PixmapPaint { opacity: 0.92, ..Default::default() }, Transform::identity(), None);
    // The parts in order, each lit from the top.
    for p in &parts {
        match p {
            Part::Fill(path, c, a) => out.fill_path(path, &lit(path, *c, *a), FillRule::Winding, t, None),
            Part::Line(path, w, c, a) => out.stroke_path(path, &lit(path, *c, *a), &stroke(*w), t, None),
            Part::Detail(path, w, c, a) if *w > 0.0 => out.stroke_path(path, &solid(*c, *a), &stroke(*w), t, None),
            Part::Detail(path, _, c, a) => out.fill_path(path, &solid(*c, *a), FillRule::Winding, t, None),
        }
    }
    let rgba = out.pixels().iter().flat_map(|p| {
        let c = p.demultiply();
        [c.red(), c.green(), c.blue(), c.alpha()]
    });
    let max = size.saturating_sub(1) as f32;
    Art { rgba: rgba.collect(), size, hot: ((hot.0 * scale).round().clamp(0.0, max) as u16, (hot.1 * scale).round().clamp(0.0, max) as u16) }
}

/// The alpha of `p`, box-blurred twice (near enough a Gaussian) by radius
/// `r` pixels and moved by `by`; 0 to 1 a pixel.
fn blur(p: &Pixmap, r: f32, by: (f32, f32)) -> Vec<f32> {
    let (w, h) = (p.width() as i32, p.height() as i32);
    let mut a: Vec<f32> = p.pixels().iter().map(|c| c.alpha() as f32 / 255.0).collect();
    let r = r.round().max(1.0) as i32;
    let pass = |src: &[f32], dx: i32, dy: i32| -> Vec<f32> {
        let mut out = vec![0.0; src.len()];
        let n = (2 * r + 1) as f32;
        for y in 0..h {
            for x in 0..w {
                let mut s = 0.0;
                for k in -r..=r {
                    let (xx, yy) = (x + k * dx, y + k * dy);
                    if xx >= 0 && yy >= 0 && xx < w && yy < h {
                        s += src[(yy * w + xx) as usize];
                    }
                }
                out[(y * w + x) as usize] = s / n;
            }
        }
        out
    };
    for _ in 0..2 {
        a = pass(&a, 1, 0);
        a = pass(&a, 0, 1);
    }
    let (ox, oy) = (by.0.round() as i32, by.1.round() as i32);
    (0..h)
        .flat_map(|y| (0..w).map(move |x| (x, y)))
        .map(|(x, y)| {
            let (sx, sy) = (x - ox, y - oy);
            if sx >= 0 && sy >= 0 && sx < w && sy < h {
                a[(sy * w + sx) as usize]
            } else {
                0.0
            }
        })
        .collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    /// Every look draws at every size, opaque where its hot spot is or
    /// next to it (the hot spot is on the picture, not in empty space),
    /// and no two looks are the same picture.
    #[test]
    fn every_cursor_draws_and_points_where_it_shows() {
        for scale in [1.0, 1.5, 2.0, 2.5] {
            let mut seen: Vec<Vec<u8>> = Vec::new();
            for look in Look::ALL {
                let a = art(look, scale);
                assert_eq!(a.rgba.len(), (a.size * a.size * 4) as usize);
                let alpha = |x: i32, y: i32| if x < 0 || y < 0 || x >= a.size as i32 || y >= a.size as i32 { 0 } else { a.rgba[((y as u32 * a.size + x as u32) * 4 + 3) as usize] };
                let (hx, hy) = (a.hot.0 as i32, a.hot.1 as i32);
                let r = (2.0 * scale).ceil() as i32;
                let near = (-r..=r).flat_map(|dy| (-r..=r).map(move |dx| (dx, dy))).map(|(dx, dy)| alpha(hx + dx, hy + dy)).max().unwrap();
                assert!(near > 200, "{} at {scale}: nothing drawn at its hot spot", look.name());
                assert!(!seen.contains(&a.rgba), "{} at {scale} looks like another", look.name());
                seen.push(a.rgba);
            }
        }
    }

    /// The commands that place or throw get their own looks, so the squad's
    /// crosshair never shows while a building, wall or device waits.
    #[test]
    fn waiting_commands_have_their_own_looks() {
        let p = Target::Point { radius: 1.0 };
        assert_eq!(target_look(WALL, Target::Line { width: 1.0 }), Look::Wall);
        assert_eq!(target_look(DOOR, p), Look::Door);
        assert_eq!(target_look(PUMP, p), Look::Pump);
        for id in [TURRET, MORTAR, SPRAYER, HUNTER, LURE, SIREN, OUTPOST] {
            assert_eq!(target_look(id, p), Look::Build);
        }
        for id in [T_BLAST, T_CORD, T_MINE, T_GERM, T_GAS, T_PROVOKE, T_STILL] {
            assert_eq!(target_look(id, p), Look::Device);
        }
        assert_eq!(target_look(ATTACK, p), Look::Attack);
        assert_eq!(target_look(GUARD, p), Look::Guard);
        assert_eq!(target_look(HARVEST, p), Look::Drain);
        assert_eq!(target_look(RALLY, p), Look::Rally);
    }

    /// `CURSOR_SHEET=out.png cargo test ... cursor_sheet`: every look at
    /// 1x and 2x on the game's dark ground and on bright slime, for the
    /// pull request's picture.
    #[test]
    fn cursor_sheet() {
        let Ok(path) = std::env::var("CURSOR_SHEET") else { return };
        let looks = Look::ALL;
        let cell = 96u32;
        let (w, h) = (cell * looks.len() as u32, cell * 4);
        let mut img = image::RgbaImage::new(w, h);
        for (i, look) in looks.iter().enumerate() {
            for (row, (scale, bg)) in [(1.0, [44u8, 48, 54]), (2.0, [44, 48, 54]), (1.0, [214, 168, 70]), (2.0, [214, 168, 70])].iter().enumerate() {
                let a = art(*look, *scale);
                let (x0, y0) = (i as u32 * cell, row as u32 * cell);
                for y in 0..cell {
                    for x in 0..cell {
                        img.put_pixel(x0 + x, y0 + y, image::Rgba([bg[0], bg[1], bg[2], 255]));
                    }
                }
                let off = (cell - a.size) / 2;
                for y in 0..a.size {
                    for x in 0..a.size {
                        let k = ((y * a.size + x) * 4) as usize;
                        let al = a.rgba[k + 3] as f32 / 255.0;
                        let px = img.get_pixel_mut(x0 + off + x, y0 + off + y);
                        for c in 0..3 {
                            px[c] = (a.rgba[k + c] as f32 * al + px[c] as f32 * (1.0 - al)).round() as u8;
                        }
                    }
                }
            }
        }
        img.save(path).unwrap();
    }
}
