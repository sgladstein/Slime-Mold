//! **Marks**: what a game draws over the world each frame (units, rings,
//! health bars, buildings, order lines, the drag box, short labels), as a
//! list the view pass draws on the GPU (`world::view`). No game rules: the
//! game fills a `Marks` from its own state every frame.
//!
//! A mark is anchored at a world point and offset in screen pixels, so a
//! unit's disc can be sized in world cells (it grows as you zoom in) while
//! its health bar stays a fixed size under it. Sizes are half-extents.
//!
//! Marks are things a player gave or owns (units, orders), not the
//! simulation: crystals, terrain and the mould are fields drawn into the
//! world frame itself (the owner's rule that the picture is the data).

/// Words a mark: x, y, ox, oy, hw, hh, shape, colour, colour2, fill, flags, extra.
pub const MARK_WORDS: usize = 12;
/// The most marks a frame (the Larder thread's budget: two teams of 40,
/// buildings, bait, shot lines and range rings, about 1,000).
pub const MAX_MARKS: usize = 2048;

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u32)]
pub enum Shape {
    Disc = 0,
    /// A ring, `fill` pixels wide (1.5 when `fill` is 0).
    Ring = 1,
    Rect = 2,
    /// A rectangle's outline, one pixel wide.
    Frame = 3,
    /// A bar `fill` full: `colour` left of the fill, `colour2` right.
    Bar = 4,
    /// One 5x7 glyph, `hw` pixels a font pixel, its top-left at the anchor.
    Glyph = 5,
    /// A line from the anchor to (`hw`, `hh`), a world point; dashed every
    /// `fill` pixels if `fill` > 0.
    Line = 6,
    /// A top-down soldier `hw` across the radius (world cells with
    /// `WORLD`): facing `fill` radians (0 east, clockwise on screen, as
    /// `atan2(dy, dx)`), the team's colour, `extra` bits 0-7 the muzzle
    /// flash (0-255), bit 8 dead. Drawn and lit in the view's shader.
    Soldier = 7,
    /// A top-down building `hw` across its footprint radius (world cells
    /// with `WORLD`): kind `fill` (0 turret, 1 lure, 2 pump, 3 outpost, 4 siren,
    /// 5 larder), `extra` bits 0-7 how built (0-255), 8-15 its health share,
    /// bit 16 working (its lamp lit). The sprite atlas's tiles 4-15.
    Building = 8,
}

/// `hw` and `hh` are world cells (scaled by the zoom, at least 2 pixels)
/// rather than screen pixels.
pub const WORLD: u32 = 1;
/// Also a dot on the minimap.
pub const MINIMAP: u32 = 2;

/// RGBA, alpha 255 opaque.
pub type Rgba = [u8; 4];

#[derive(Clone, Copy, Debug, PartialEq)]
pub struct Mark {
    pub at: (f32, f32),
    pub offset: (f32, f32),
    pub half: (f32, f32),
    pub shape: Shape,
    pub colour: Rgba,
    pub colour2: Rgba,
    pub fill: f32,
    pub flags: u32,
    /// A glyph's bits 32-34 (bits 0-31 go in `colour2`).
    pub extra: u32,
}

impl Mark {
    pub fn new(shape: Shape, at: (f32, f32), half: (f32, f32), colour: Rgba) -> Self {
        Mark { at, offset: (0.0, 0.0), half, shape, colour, colour2: [0; 4], fill: 0.0, flags: 0, extra: 0 }
    }
}

fn rgba(c: Rgba) -> u32 {
    u32::from_le_bytes(c)
}

/// The colours teams are drawn in: team 0 (the local player) white, then
/// sky blue, rose and lime. Mould species have their own colours in the
/// world frame; these are only for marks.
pub const TEAMS: [Rgba; 4] = [[245, 245, 240, 255], [110, 180, 255, 255], [255, 120, 150, 255], [170, 235, 110, 255]];
/// The selection ring and drag box: cream, the team's own colour (art
/// direction rule 3), over a dark ring (`Marks::selected`). Heavy yellow
/// rings 1.5 pixels wide hid a soldier on gold slime (art review
/// 2026-11-14) and were the slime's own hue.
/// A striding figure, 5x7 (`Marks::icon`): a builder on the way.
pub const WALKING: [u8; 7] = [0b00100, 0b01110, 0b10101, 0b00100, 0b01010, 0b01001, 0b10001];
/// A standing figure; with `SLASH` over it, no builder sent.
pub const STANDING: [u8; 7] = [0b00100, 0b01110, 0b10101, 0b00100, 0b01010, 0b01010, 0b01010];
/// A slash corner to corner, drawn over `STANDING` in the warning colour.
pub const SLASH: [u8; 7] = [0b00001, 0b00010, 0b00010, 0b00100, 0b01000, 0b01000, 0b10000];
pub const SELECTED: Rgba = [250, 246, 228, 255];
/// The dark under-ring that keeps a selection ring readable on bright slime.
pub const SELECTED_INK: Rgba = [14, 13, 10, 200];
/// A health bar's fill and its empty part.
pub const HEALTH: Rgba = [110, 226, 120, 255];
pub const HEALTH_GONE: Rgba = [24, 22, 20, 230];

pub fn team_colour(team: u8) -> Rgba {
    TEAMS[team as usize % TEAMS.len()]
}

/// A frame's marks, packed as the view pass reads them.
#[derive(Clone, Default, Debug)]
pub struct Marks {
    pub list: Vec<Mark>,
}

impl Marks {
    pub fn clear(&mut self) {
        self.list.clear();
    }

    pub fn len(&self) -> usize {
        self.list.len()
    }

    pub fn is_empty(&self) -> bool {
        self.list.is_empty()
    }

    /// Adds a mark (past `MAX_MARKS` it is dropped).
    pub fn push(&mut self, m: Mark) {
        if self.list.len() < MAX_MARKS {
            self.list.push(m);
        }
    }

    /// A filled disc `r` world cells across the radius.
    pub fn disc(&mut self, at: (f32, f32), r: f32, colour: Rgba, minimap: bool) {
        self.push(Mark { flags: WORLD | if minimap { MINIMAP } else { 0 }, ..Mark::new(Shape::Disc, at, (r, r), colour) });
    }

    /// A ring `r` world cells out, one pixel wide-ish.
    pub fn ring(&mut self, at: (f32, f32), r: f32, colour: Rgba) {
        self.push(Mark { flags: WORLD, ..Mark::new(Shape::Ring, at, (r, r), colour) });
    }

    /// A ring `r` screen pixels out (a selection ring that stays visible at
    /// any zoom).
    pub fn ring_px(&mut self, at: (f32, f32), r: f32, colour: Rgba) {
        self.push(Mark::new(Shape::Ring, at, (r, r), colour));
    }

    /// A ring `r` screen pixels out and `wide` pixels wide.
    pub fn ring_thin(&mut self, at: (f32, f32), r: f32, wide: f32, colour: Rgba) {
        self.push(Mark { fill: wide, ..Mark::new(Shape::Ring, at, (r, r), colour) });
    }

    /// A unit's selection: a thin cream ring `r` screen pixels out on a
    /// dark one just inside it, so it reads on the slime and the ground
    /// alike without hiding the figure (art review 2026-11-14: a heavy
    /// yellow ring hid a soldier at zoom 6).
    pub fn selected(&mut self, at: (f32, f32), r: f32) {
        self.ring_thin(at, r - 1.0, 1.0, SELECTED_INK);
        self.ring_thin(at, r, 1.0, SELECTED);
    }

    /// A unit's health, shown only when it is hurt (`hp` under 0.95, so a
    /// nick from one walker doesn't put bars on a whole squad; art
    /// direction rule 6): an 18 x 3 pixel bar `below` pixels under the
    /// anchor in a dark frame, green, amber under 60%, red under 30%. Whole health draws nothing.
    pub fn health(&mut self, at: (f32, f32), below: f32, hp: f32) {
        if hp >= 0.95 {
            return;
        }
        // Green, then amber under 60%, red (the warning colour) under 30%.
        let colour = if hp > 0.6 { HEALTH } else if hp > 0.3 { [240, 190, 70, 255] } else { [235, 80, 70, 255] };
        self.bar(at, below, (10.0, 2.5), 1.0, HEALTH_GONE, HEALTH_GONE);
        self.bar(at, below, (9.0, 1.5), hp, colour, HEALTH_GONE);
    }

    /// A filled square or rectangle, half-extents in world cells (a
    /// building).
    pub fn rect(&mut self, at: (f32, f32), half: (f32, f32), colour: Rgba, minimap: bool) {
        self.push(Mark { flags: WORLD | if minimap { MINIMAP } else { 0 }, ..Mark::new(Shape::Rect, at, half, colour) });
    }

    /// A rectangle's outline between two world corners (the drag box).
    pub fn frame(&mut self, a: (f32, f32), b: (f32, f32), colour: Rgba) {
        let at = ((a.0 + b.0) / 2.0, (a.1 + b.1) / 2.0);
        let half = ((a.0 - b.0).abs() / 2.0, (a.1 - b.1).abs() / 2.0);
        self.push(Mark { flags: WORLD, ..Mark::new(Shape::Frame, at, half, colour) });
    }

    /// A bar `fill` full (0-1), `half` pixels, `below` pixels under the
    /// anchor: a health bar.
    pub fn bar(&mut self, at: (f32, f32), below: f32, half: (f32, f32), fill: f32, colour: Rgba, back: Rgba) {
        self.push(Mark { offset: (0.0, below), colour2: back, fill: fill.clamp(0.0, 1.0), ..Mark::new(Shape::Bar, at, half, colour) });
    }

    /// A line between two world points, dashed every `dash` pixels (0 solid).
    pub fn line(&mut self, a: (f32, f32), b: (f32, f32), dash: f32, colour: Rgba) {
        self.push(Mark { fill: dash, ..Mark::new(Shape::Line, a, b, colour) });
    }

    /// Text at a world point, offset `offset` pixels, `scale` pixels a font
    /// pixel (a count beside a crystal or a building).
    pub fn label(&mut self, at: (f32, f32), offset: (f32, f32), text: &str, scale: f32, colour: Rgba) {
        let advance = 6.0 * scale;
        for (i, c) in text.chars().enumerate() {
            if c == ' ' {
                continue;
            }
            self.icon(at, (offset.0 + i as f32 * advance, offset.1), crate::hud::glyph_rows(c), scale, colour);
        }
    }

    /// One 5x7 picture (rows top down, bit 4 the left column), drawn as a
    /// glyph: a small sign beside a building, such as `WALKING`.
    pub fn icon(&mut self, at: (f32, f32), offset: (f32, f32), rows: [u8; 7], scale: f32, colour: Rgba) {
        let mut bits = 0u64;
        for (r, row) in rows.iter().enumerate() {
            for col in 0..5 {
                if row >> (4 - col) & 1 == 1 {
                    bits |= 1 << (r * 5 + col);
                }
            }
        }
        self.push(Mark { offset, colour2: (bits as u32).to_le_bytes(), extra: (bits >> 32) as u32, ..Mark::new(Shape::Glyph, at, (scale, scale), colour) });
    }

    /// A soldier at `at`, `r` world cells across the radius, facing
    /// `facing` radians, `flash` 0-1 the muzzle flash of a shot this
    /// moment (0 none). Also a dot on the minimap.
    pub fn soldier(&mut self, at: (f32, f32), r: f32, facing: f32, colour: Rgba, flash: f32, alive: bool) {
        let extra = (flash.clamp(0.0, 1.0) * 255.0).round() as u32 | if alive { 0 } else { 256 };
        self.push(Mark { flags: WORLD | MINIMAP, fill: facing, extra, ..Mark::new(Shape::Soldier, at, (r, r), colour) });
    }

    /// A building of kind `kind` (0 turret, 1 lure, 2 pump, 3 outpost, 4 siren, 5 larder) at
    /// `at`, footprint radius `r` world cells: `built` and `health` 0-1,
    /// `working` lights its lamp. Everything it shows is the game's state:
    /// it rises from its foot as it is built, darkens as it is hurt.
    pub fn building(&mut self, at: (f32, f32), r: f32, kind: u32, built: f32, health: f32, working: bool) {
        let b = (built.clamp(0.0, 1.0) * 255.0).round() as u32;
        let h = (health.clamp(0.0, 1.0) * 255.0).round() as u32;
        let extra = b | h << 8 | (working as u32) << 16;
        self.push(Mark { flags: WORLD | MINIMAP, fill: kind as f32, extra, ..Mark::new(Shape::Building, at, (r, r), [200, 200, 200, 255]) });
    }

    /// The words the view pass reads, `MARK_WORDS` a mark.
    pub fn words(&self) -> Vec<u32> {
        let mut w = Vec::with_capacity(self.list.len() * MARK_WORDS);
        for m in &self.list {
            w.extend([
                m.at.0.to_bits(),
                m.at.1.to_bits(),
                m.offset.0.to_bits(),
                m.offset.1.to_bits(),
                m.half.0.to_bits(),
                m.half.1.to_bits(),
                m.shape as u32,
                rgba(m.colour),
                rgba(m.colour2),
                m.fill.to_bits(),
                m.flags,
                m.extra,
            ]);
        }
        w
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn a_label_is_a_glyph_a_character_with_its_bits() {
        let mut m = Marks::default();
        m.label((10.0, 10.0), (4.0, -3.0), "1 2", 2.0, [255; 4]);
        assert_eq!(m.len(), 2, "the space draws nothing");
        assert_eq!(m.list[1].offset, (4.0 + 2.0 * 12.0, -3.0));
        // '1''s rows hold 7 pixels down its middle column at least.
        let bits = u32::from_le_bytes(m.list[0].colour2) as u64 | (m.list[0].extra as u64) << 32;
        assert!(bits.count_ones() >= 7 && bits < 1 << 35);
        assert_eq!(m.words().len(), 2 * MARK_WORDS);
    }

    #[test]
    fn marks_past_the_budget_are_dropped() {
        let mut m = Marks::default();
        for k in 0..MAX_MARKS + 10 {
            m.disc((k as f32, 0.0), 1.0, [255; 4], false);
        }
        assert_eq!(m.len(), MAX_MARKS);
    }

    #[test]
    fn health_shows_only_when_hurt() {
        let mut m = Marks::default();
        m.health((0.0, 0.0), 6.0, 1.0);
        m.health((0.0, 0.0), 6.0, 0.97);
        assert!(m.is_empty(), "whole health, or a nick, draws nothing");
        m.health((0.0, 0.0), 6.0, 0.4);
        assert_eq!(m.len(), 2, "a frame and the bar");
        assert_eq!(m.list[1].fill, 0.4);
    }
}
