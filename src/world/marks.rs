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
/// buildings, bait, shot lines and range rings, about 1,000). 4096 since
/// 2026-11-16, for the 200-soldier cap (`Rules::max_squad`): a body, a
/// health bar, a selection ring, a muzzle flash and a head (drawn again
/// above every body, `Marks::soldier_head`) each, the dead (drawn
/// first, and never removed in a level) and every bullet's tracer could
/// pass 2,048, and past the cap the newest marks, the living, are dropped.
/// The marks pass costs by marks drawn, not by the buffer.
pub const MAX_MARKS: usize = 4096;

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
    /// flash (0-255), bit 8 dead, bits 9-16 its carried load (0-255 of a
    /// full one), bits 17-24 the pulse of a take from a crystal. Drawn and lit in the view's shader.
    /// A dead one (`Marks::fallen`) reads bits 0-7 as how far its pool of
    /// blood has spread (0-255) and bits 9-16 as the seed of its shape.
    /// Bit 25 draws the helmet alone (`Marks::soldier_head`).
    Soldier = 7,
    /// A top-down building `hw` across its footprint radius (world cells
    /// with `WORLD`): kind `fill` (0 turret, 1 lure, 2 pump, 3 outpost, 4 siren,
    /// 5 larder, 6 a pump's rig round its crystal), `extra` bits 0-7 how built (0-255), 8-15 its health share,
    /// bit 16 working (its lamp lit), bits 17-24 a flare of its lamp. The
    /// sprite atlas's tiles 4-15.
    Building = 8,
    /// Something bright in flight (a bullet), drawn along this frame's
    /// travel: head at the anchor, tail at (`hw`, `hh`), a world point;
    /// `fill` the core's radius in pixels; `colour` its glow (alpha the
    /// glow's strength), `colour2` its hot core. Added to the world as
    /// light (`view.wgsl`, `streak_at`), not painted over it.
    Streak = 9,
    /// A muzzle flash (gun look, 2026-11-15): flame jets out of a barrel's
    /// tip at the anchor, the forward jet `hw` world cells long, facing
    /// `fill` radians (as `Soldier`); `extra` bits 0-1 the gun's kind (0
    /// rifle, 1 carbine, 2 long gun, 3 scattergun), 2-9 a seed that varies
    /// each shot's jets, 10-17 how much of the flash is left (0-255);
    /// `colour` its glow, `colour2` its hot core. Light, as `Streak`
    /// (`view.wgsl`, `flash_at`).
    Flash = 10,
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

/// A turret's lamp (`Marks::turret`): nothing in reach, mould in reach
/// with food to fire, and the bank too low to pay for a shot.
pub const TURRET_IDLE: u32 = 0;
pub const TURRET_TRACKING: u32 = 1;
pub const TURRET_DRY: u32 = 2;

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

    /// A streak from `tail` to `head` (world points), its core `core_px`
    /// pixels across the radius: see `Shape::Streak`.
    pub fn streak(&mut self, head: (f32, f32), tail: (f32, f32), core_px: f32, glow: Rgba, core: Rgba) {
        self.push(Mark { colour2: core, fill: core_px, ..Mark::new(Shape::Streak, head, tail, glow) });
    }

    /// A muzzle flash at `tip` (a barrel's end, world cells), its forward
    /// jet `len` world cells, facing `facing` radians, for gun kind `kind`
    /// (0-3), `seed` varying its jets, `left` 0-1 of the flash still to
    /// burn: see `Shape::Flash`.
    #[allow(clippy::too_many_arguments)]
    pub fn muzzle_flash(&mut self, tip: (f32, f32), len: f32, facing: f32, kind: u32, seed: u32, left: f32, glow: Rgba, core: Rgba) {
        let extra = kind.min(3) | (seed & 255) << 2 | ((left.clamp(0.0, 1.0) * 255.0).round() as u32) << 10;
        self.push(Mark { flags: WORLD, fill: facing, extra, colour2: core, ..Mark::new(Shape::Flash, tip, (len, len), glow) });
    }

    /// A soldier at `at`, `r` world cells across the radius, facing
    /// `facing` radians, `flash` 0-1 the muzzle flash of a shot this
    /// moment (0 none). Also a dot on the minimap.
    pub fn soldier(&mut self, at: (f32, f32), r: f32, facing: f32, colour: Rgba, flash: f32, alive: bool) {
        self.soldier_working(at, r, facing, colour, flash, alive, 0.0, 0.0);
    }

    /// A soldier as `soldier`, with `carry` 0-1 its load (cyan charge in
    /// its pack and a glow that grows with it) and `take` 0-1 the pulse at
    /// its hands while it takes from a crystal.
    #[allow(clippy::too_many_arguments)]
    pub fn soldier_working(&mut self, at: (f32, f32), r: f32, facing: f32, colour: Rgba, flash: f32, alive: bool, carry: f32, take: f32) {
        self.soldier_bitten(at, r, facing, colour, flash, alive, carry, take, 0.0, 0.0);
    }

    /// A soldier as `soldier_working`, `bite` 0-1 the slime's last bite
    /// fading (building looks, 11-14: "a flinch and a brief light where
    /// slime bites, from the real damage"): the figure flushes blood red
    /// (`view.wgsl`, `soldier_at`). `toward`, radians (screen frame, as
    /// `facing`), is the side it came from, which lit that side alone until
    /// 2026-11-18; the view no longer reads it. Both ride in the second
    /// colour's word, which a soldier does not use: the bite a byte, the
    /// angle 16 bits above it.
    #[allow(clippy::too_many_arguments)]
    pub fn soldier_bitten(&mut self, at: (f32, f32), r: f32, facing: f32, colour: Rgba, flash: f32, alive: bool, carry: f32, take: f32, bite: f32, toward: f32) {
        if !alive {
            self.fallen(at, r, facing, colour, 1.0, 0);
            return;
        }
        let byte = |v: f32| (v.clamp(0.0, 1.0) * 255.0).round() as u32;
        let extra = byte(flash) | byte(carry) << 9 | byte(take) << 17;
        let turn = toward.rem_euclid(std::f32::consts::TAU) / std::f32::consts::TAU;
        let word = byte(bite) | ((turn * 65535.0).round() as u32) << 8;
        self.push(Mark { flags: WORLD | MINIMAP, fill: facing, extra, colour2: word.to_le_bytes(), ..Mark::new(Shape::Soldier, at, (r, r), colour) });
    }

    /// Soldier `soldier_bitten`'s helmet alone, drawn again above every
    /// figure so a packed squad shows each head (the brief: "space the
    /// squad so 40 soldiers can be counted"). Same arguments as the figure
    /// it tops, so its pixels are that figure's own where its helmet is.
    #[allow(clippy::too_many_arguments)]
    pub fn soldier_head(&mut self, at: (f32, f32), r: f32, facing: f32, colour: Rgba, flash: f32, carry: f32, take: f32, bite: f32, toward: f32) {
        let n = self.list.len();
        self.soldier_bitten(at, r, facing, colour, flash, true, carry, take, bite, toward);
        if self.list.len() > n {
            let m = &mut self.list[n];
            m.extra |= 1 << 25;
            m.flags &= !MINIMAP;
        }
    }

    /// The soldier marks pushed since `from` (`len()` before a
    /// `soldier_bitten` or `soldier_head`) dressed in Armour look `skin`:
    /// 0 olive, 1 slate, 2 dark camo (owner, 2026-11-19; `view.wgsl`,
    /// `skin_body`). It rides at bits 27-28 of `extra`.
    pub fn dress_since(&mut self, from: usize, skin: u32) {
        for m in self.list.iter_mut().skip(from).filter(|m| m.shape == Shape::Soldier) {
            m.extra |= skin.min(2) << 27;
        }
    }

    /// The soldier marks pushed since `from` (`len()` before a
    /// `soldier_bitten` or `soldier_head`) asleep in their own gas
    /// (`Rules::doze`, 2026-11-19): `tint` 0-1 how much of the gas's
    /// lavender the figure takes (the cloud's level at its feet), `asleep`
    /// whether it is out (it neither fires nor walks), which also dims
    /// it. Both ride where a soldier mark has room: the tint in the second
    /// colour's top byte, the flag at bit 26 of `extra`.
    pub fn doze_since(&mut self, from: usize, tint: f32, asleep: bool) {
        let byte = (tint.clamp(0.0, 1.0) * 255.0).round() as u32;
        for m in self.list.iter_mut().skip(from).filter(|m| m.shape == Shape::Soldier) {
            m.colour2[3] = byte as u8;
            m.extra |= (asleep as u32) << 26;
        }
    }

    /// A fallen soldier at `at`, `r` world cells across the radius, lying
    /// as it fell from facing `facing` radians (it fell back, so its head
    /// lies behind it), over a pool of blood spread `pool` 0-1 of the way
    /// (it spreads in the seconds after the fall, `POOL_GROW` in the
    /// window), its shape varied by `seed`. The owner's playtest
    /// (2026-11-18 16:12Z): "When soliders die, the body on the floor looks
    /// really bad. You could create a better looking body or just a blood
    /// splotch."
    pub fn fallen(&mut self, at: (f32, f32), r: f32, facing: f32, colour: Rgba, pool: f32, seed: u32) {
        let extra = (pool.clamp(0.0, 1.0) * 255.0).round() as u32 | 256 | (seed & 255) << 9;
        self.push(Mark { flags: WORLD | MINIMAP, fill: facing, extra, ..Mark::new(Shape::Soldier, at, (r, r), colour) });
    }

    /// A building of kind `kind` (0 turret, 1 lure, 2 pump, 3 outpost, 4 siren, 5 larder) at
    /// `at`, footprint radius `r` world cells: `built` and `health` 0-1,
    /// `working` lights its lamp. Everything it shows is the game's state:
    /// it rises from its foot as it is built, darkens as it is hurt.
    pub fn building(&mut self, at: (f32, f32), r: f32, kind: u32, built: f32, health: f32, working: bool) {
        self.building_pulse(at, r, kind, built, health, working, 0.0);
    }

    /// A building as `building`, its lamp flaring by `pulse` 0-1 (the
    /// larder's cells as a load is banked).
    #[allow(clippy::too_many_arguments)]
    pub fn building_pulse(&mut self, at: (f32, f32), r: f32, kind: u32, built: f32, health: f32, working: bool, pulse: f32) {
        self.building_facing(at, r, kind, built, health, working, pulse, 0.0);
    }

    /// A building as `building_pulse`, turned to `facing` (radians): a
    /// pump's hose into the crystal it drains (building looks, 11-14).
    /// Only the pump turns; the facing rides in the second colour's word,
    /// as the turret's does.
    #[allow(clippy::too_many_arguments)]
    pub fn building_facing(&mut self, at: (f32, f32), r: f32, kind: u32, built: f32, health: f32, working: bool, pulse: f32, facing: f32) {
        let b = (built.clamp(0.0, 1.0) * 255.0).round() as u32;
        let h = (health.clamp(0.0, 1.0) * 255.0).round() as u32;
        let extra = b | h << 8 | (working as u32) << 16 | ((pulse.clamp(0.0, 1.0) * 255.0).round() as u32) << 17;
        self.push(Mark { flags: WORLD | MINIMAP, fill: kind as f32, extra, colour2: facing.to_bits().to_le_bytes(), ..Mark::new(Shape::Building, at, (r, r), [200, 200, 200, 255]) });
    }

    /// A pump's rig round its crystal (building kind 6, 2026-11-15): a
    /// steel hexagon `r` world cells out with a pylon at each corner and
    /// struts to the middle, drawn as shapes in the view's shader (crisp
    /// at any zoom). `built` and `health` as `building`; `draw` 0-1 the
    /// pump's draw this frame of its full draw (the struts' charge and the
    /// lamps), `level` 1-3 (an inner ring a level).
    #[allow(clippy::too_many_arguments)]
    pub fn pump_rig(&mut self, at: (f32, f32), r: f32, built: f32, health: f32, working: bool, draw: f32, level: u8) {
        let b = (built.clamp(0.0, 1.0) * 255.0).round() as u32;
        let h = (health.clamp(0.0, 1.0) * 255.0).round() as u32;
        let extra = b | h << 8 | (working as u32) << 16 | ((draw.clamp(0.0, 1.0) * 255.0).round() as u32) << 17 | (u32::from(level.clamp(1, 3)) - 1) << 25;
        // The building box is twice the half each way (`draw_marks`).
        self.push(Mark { flags: WORLD, fill: 6.0, extra, ..Mark::new(Shape::Building, at, (r / 2.0, r / 2.0), [200, 200, 200, 255]) });
    }

    /// A turret (building kind 0) as `building`, its head turned to
    /// `facing` (radians, the game's aim), kicked back by `shot` 0-1 (the
    /// shot just fired, fading) with the muzzle flash, and its lamp by
    /// `state`: `TURRET_IDLE`, `TURRET_TRACKING` or `TURRET_DRY`
    /// (owner's playtest 11-14 08:15Z: "turrets need to look better"). The
    /// facing rides in the second colour's word, which a building does not
    /// use.
    #[allow(clippy::too_many_arguments)]
    pub fn turret(&mut self, at: (f32, f32), r: f32, built: f32, health: f32, facing: f32, shot: f32, state: u32) {
        self.gun_building(at, r, 0.0, built, health, facing, shot, state);
    }

    /// A turret or a mortar, building kind `kind`.
    #[allow(clippy::too_many_arguments)]
    fn gun_building(&mut self, at: (f32, f32), r: f32, kind: f32, built: f32, health: f32, facing: f32, shot: f32, state: u32) {
        let b = (built.clamp(0.0, 1.0) * 255.0).round() as u32;
        let h = (health.clamp(0.0, 1.0) * 255.0).round() as u32;
        let extra = b | h << 8 | ((shot.clamp(0.0, 1.0) * 255.0).round() as u32) << 17 | (state.min(3)) << 25;
        self.push(Mark { flags: WORLD | MINIMAP, fill: kind, extra, colour2: facing.to_bits().to_le_bytes(), ..Mark::new(Shape::Building, at, (r, r), [200, 200, 200, 255]) });
    }

    /// A mortar (building kind 7, the turrets thread 2026-11-17) as
    /// `turret`: the turret's base under a short wide tube turned to
    /// `facing`, sinking back by `shot` as a shell leaves.
    #[allow(clippy::too_many_arguments)]
    pub fn mortar(&mut self, at: (f32, f32), r: f32, built: f32, health: f32, facing: f32, shot: f32, state: u32) {
        self.gun_building(at, r, 7.0, built, health, facing, shot, state);
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
    fn a_streak_carries_its_tail_core_and_colours() {
        let mut m = Marks::default();
        m.streak((10.0, 4.0), (7.0, 4.0), 2.5, [255, 120, 40, 210], [255, 236, 190, 255]);
        let w = m.words();
        assert_eq!(w[6], Shape::Streak as u32);
        assert_eq!((f32::from_bits(w[4]), f32::from_bits(w[5])), (7.0, 4.0), "the tail is a world point");
        assert_eq!(f32::from_bits(w[9]), 2.5, "the core's radius in pixels");
        assert_eq!(w[8], u32::from_le_bytes([255, 236, 190, 255]));
    }

    #[test]
    fn a_muzzle_flash_packs_its_kind_seed_and_life() {
        let mut m = Marks::default();
        m.muzzle_flash((5.0, 6.0), 1.5, 0.5, 3, 300, 0.5, [255, 200, 110, 255], [255, 250, 235, 255]);
        let w = m.words();
        assert_eq!(w[6], Shape::Flash as u32);
        assert_eq!(f32::from_bits(w[9]), 0.5, "the facing");
        assert_eq!(w[10] & WORLD, WORLD, "its length is world cells");
        assert_eq!((w[11] & 3, w[11] >> 2 & 255, w[11] >> 10 & 255), (3, 300 & 255, 128));
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
