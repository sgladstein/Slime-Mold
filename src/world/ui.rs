//! **The game's UI**: the panels a player reads and clicks, drawn on the
//! UI canvas (`canvas`) over the world. No game rules: each frame the game
//! fills a `Model` (its resources, the selection, the command card, what
//! the cursor is over) and gets back `HudEvent`s, which it maps to its own
//! commands, as it does the controls' `UiCommand`s.
//!
//! Asked for by the owner, 2026-11-11 20:14-20:16Z: "we don't have much of
//! a UI yet": soldier health and numbers, ways to control them, weapons,
//! upgrades, walls "and probably other things in the future", menus and
//! enemy info, looking "modern and professional".
//!
//! - **Top bar**: the game's resources (icon, value, a rate), the
//!   objective, the clock, the speed and a menu button.
//! - **Selection panel** (bottom): one unit or a hovered enemy in detail
//!   (`Info`: health, stats), or every selected unit as a card with its
//!   health; control-group tabs and an idle-units button above it.
//! - **Command card** (bottom right, beside the minimap): up to 4 x 3
//!   buttons, each with an icon, hotkey, cost, a cooldown sweep and an on
//!   state (a stance, a weapon in hand). The game decides what is on it, so
//!   new weapons, upgrades and buildings are new `Command`s, not new UI.
//!   A button whose `target` is a point or a line enters targeting: a ghost
//!   follows the cursor (green where the game says it can go, red where
//!   not), left click places it, right click or Esc cancels, Shift keeps it.
//! - **Tooltips** on every button: name, hotkey, cost, what it does, and
//!   why it is greyed out.
//! - **Alerts** (top left, under the bar) with a ping on the minimap; Space
//!   jumps the camera to the newest.
//! - **Menu** (Esc or the button): resume, restart, controls, quit; and an
//!   end-of-round screen.

use std::time::{Duration, Instant};

use winit::keyboard::KeyCode;

use super::camera::Camera;
use super::canvas::{Align, Canvas, Rect, Rgba, UiSize, Weight};
use super::controls::{Button, Controls};
use super::icons::Icon;
use super::marks::{Mark, Marks, Shape, MINIMAP};

/// The palette: dark glass panels, light text, one cyan accent.
pub mod theme {
    use super::Rgba;
    pub const PANEL_TOP: Rgba = [24, 29, 36, 232];
    pub const PANEL_BOTTOM: Rgba = [13, 16, 20, 240];
    pub const EDGE: Rgba = [255, 255, 255, 26];
    pub const SHADOW: Rgba = [0, 0, 0, 110];
    pub const TEXT: Rgba = [236, 240, 244, 255];
    pub const MUTED: Rgba = [146, 156, 168, 255];
    pub const FAINT: Rgba = [96, 106, 118, 255];
    pub const ACCENT: Rgba = [96, 190, 255, 255];
    pub const GOOD: Rgba = [94, 214, 132, 255];
    pub const WARN: Rgba = [242, 192, 84, 255];
    pub const BAD: Rgba = [242, 98, 98, 255];
    pub const ENERGY: Rgba = [128, 224, 255, 255];
    pub const BUTTON_TOP: Rgba = [44, 52, 62, 255];
    pub const BUTTON_BOTTOM: Rgba = [30, 36, 44, 255];
    pub const BUTTON_HOVER_TOP: Rgba = [58, 68, 82, 255];
    pub const BUTTON_HOVER_BOTTOM: Rgba = [38, 46, 56, 255];
    pub const TRACK: Rgba = [255, 255, 255, 28];
}
use theme::*;

/// The command card's buttons and its padding, UI units (pixels at 1080
/// lines). Until the art director's layout pass (11-14 02:34Z: "the
/// bottom HUD bar is 160 of 800 px high") 58, 6 and 10, a 214-pixel bar
/// at 1080 lines (20%); now 42, 5 and 8, 160 pixels (15%).
const CELL: f32 = 42.0;
/// A labelled command button's width (UI review, 11-15: the owner, "the
/// text seems too small ... not always making the best use of space"). The
/// card's buttons were 42-unit squares with only an icon, a 9-pixel key
/// letter and a 9-pixel cost, while the selection panel beside them held
/// one squad tile in 1,100 pixels of empty glass. Now each button is this
/// wide with its name beside the icon, the same height, so the bar keeps
/// its row and its height; a window too narrow for it (or a big UI size)
/// falls back to the square buttons (`Hud::bar_fit`; until the review's
/// allocator, where the selection kept under 340 units beside a full gun
/// strip).
///
/// 108 since the card went 6 x 2 (owner 2026-11-21 06:21Z: "Shrink the gun
/// section to get another column"); 116 at 5 x 2.
const CELL_W: f32 = 108.0;
const CELL_GAP: f32 = 5.0;
const CARD_PAD: f32 = 8.0;
/// The command card's grid. 4 x 3 until the owner's ask of 11-14 08:33Z
/// ("re-arrange the bottom menu to be one row for the squad and two rows
/// for the other buttons so the whole thing is wider but shorter"): 6 x 2,
/// the 4 x 3 slots read in order (`card_slot`), so the bar went from 160
/// to 113 units at 1080 lines (15% to 10% of the screen). 5 x 2 since the
/// owner's Card 2 (2026-11-15 19:38Z): one fire button that cycles in the
/// three stances' place, Burn a line gone for Guard, no empty tile; 122
/// units narrower labelled, which goes to the selection's labels. 6 x 2
/// again since 2026-11-21 (owner 06:21Z: "Shrink the gun section to get
/// another column. Add walls to the main page."): Hold fire, Formation,
/// Heal and Wall all on the squad mode's main page; the buttons narrowed
/// to 108 and the gun strip gives the rest (`GUN_FLOOR`).
const CARD_COLS: u8 = 6;
const CARD_ROWS: u8 = 2;
/// The bottom bar's size against the rest of the HUD. Until the owner's
/// playtest of 11-14 20:19Z the bar drew at 1 and the minimap kept the
/// bar's old height (152 units), standing above the shorter bar, and only
/// showed once zoomed in, so zooming rearranged the whole bar ("the bottom
/// hud menu changes totally when you zoom in ... The minimap should always
/// be visible and everything could be made slightly small to fit in the
/// same row with the same height"). Now the minimap is always in the row
/// at the bar's height, and the gun cards, selection, squad tiles and
/// command card draw at 92% so the row still fits: at the 200% UI size on
/// 1920x1080 the selection keeps 300 units (at 1 it would have 160, under
/// the 240 where the gun strip leaves its own panel). Back to 1 after the
/// UI review (11-15, "the text seems too small"): the command card's square
/// buttons now give way when the row is short (`Hud::bar_fit`), which is
/// what 92% bought, so the bar is 113 of 1080 pixels again (it was 104).
const BAR: f32 = 1.0;
/// The minimap's widest, in bar units (a wide world's minimap stops here
/// and is letterboxed in its frame).
const MINIMAP_W: f32 = 300.0;

/// What the gun strip shows, as the bar's allocator (`Hud::draw`) gives
/// it room: Swap, the name column (or only the gun's icon, its names in
/// the tooltip), and the size it draws at. (Empty slots drew as 14-unit
/// stubs on a short row from 11-15 until the steady-HUD pass, 11-20: the
/// strip then changed width as cards went in, and the row with it.)
#[derive(Clone, Copy, Debug, PartialEq)]
struct GunFit {
    swap: bool,
    name: bool,
    /// 1, or down to 0.6 where even the barest row leaves the selection
    /// under its floor (a gun with many slots at a big UI size): the
    /// cards shrink inside their own panel.
    scale: f32,
}

impl GunFit {
    const FULL: GunFit = GunFit { swap: true, name: true, scale: 1.0 };
    const BARE: GunFit = GunFit { swap: false, name: false, scale: 1.0 };
}

/// The gun's icon where the name column is shed, UI units.
const GUN_TAB: f32 = 24.0;

/// The selection panel's least width in the row, units: the squad
/// header's three buttons with their keys, and the panel's padding
/// (`Hud::bar_fit`). The tiles shrink inside it to fit every squad (300,
/// three tiles at their 86-unit least, cost the rifle its Swap at 125% on
/// the game's 1600 x 972 window).
const SEL_FLOOR: f32 = 270.0;
/// The gun strip's width the command card's labels leave room for: the
/// rifle's four cards and the gun's icon, no Swap (`Hud::bar_fit`), drawn
/// at 0.7 of their size since the card's sixth column (2026-11-21, owner
/// 06:21Z: "Shrink the gun section to get another column"); full size
/// until then.
const GUN_FLOOR: f32 = 0.7 * (2.0 * 8.0 + 4.0 * LOADOUT_CELL + 3.0 * 5.0 + 10.0 + GUN_TAB);

/// The loadout strip's width: its slots (the widest gun's in play), the
/// name and state (or the gun's icon), and Swap, as `f` shows them. How
/// many cards are in is no part of it.
fn loadout_w(l: &Loadout, f: GunFit, s: f32) -> f32 {
    let s = s * f.scale;
    let slots = l.slots.max(l.widest).max(l.cards.len()).max(1);
    let cards = slots as f32 * LOADOUT_CELL + (slots - 1) as f32 * 5.0;
    let info = if f.name { LOADOUT_INFO } else { GUN_TAB };
    (2.0 * 8.0 + cards + 10.0 + info + if f.swap && l.cycle.is_some() { LOADOUT_SWAP + 6.0 } else { 0.0 }) * s
}

/// The squad header's buttons, from the right: each label and the key it
/// shows (none the card takes: with the build page open I, Y, J and L
/// place an outpost, a siren, switch the lures and place a lure, the
/// three-reviewer review, 11-15). One list, measured by the bar's
/// allocator and by `squad_tiles`, and drawn by `squad_tiles` (the fit
/// once counted four buttons and #330's Heal, a fifth, then ran over
/// "1 squad" at 150%).
/// Guards (now Lookouts) and Heal moved onto the command card with the
/// owner's Card 2 (2026-11-15 19:38Z), which freed about 150 units here.
fn header_labels(p: &SquadPanel, taken: &[String]) -> [(String, Option<String>); 3] {
    let shown = |k: &Option<String>| k.clone().filter(|k| !taken.contains(k));
    [("Let go".into(), shown(&p.free_key)), ("Combine".into(), shown(&p.combine_key)), ("Split".into(), shown(&p.split_key))]
}

/// A header button's text: its label, and its key where `keys`.
fn header_text(l: &str, k: &Option<String>, keys: bool) -> String {
    match k {
        Some(k) if keys => format!("{l}  {k}"),
        _ => l.to_string(),
    }
}

/// The squad header's steps, richest first: text size, keys, the header
/// word. `squad_tiles` draws the first that fits; the allocator gives
/// the keys room before the command card's labels and the header word
/// after the gun's name (the designer's order, 11-15).
const HEADER_STEPS: [(f32, bool, bool); 4] = [(13.5, true, true), (12.0, true, true), (12.0, true, false), (12.0, false, false)];

/// The header's width at a step: its buttons, 26 units of padding and gap
/// each, and the header word.
fn header_w(c: &Canvas, labels: &[(String, Option<String>)], head: &str, step: (f32, bool, bool), s: f32) -> f32 {
    let (px, keys, with_head) = step;
    let row: f32 = labels.iter().map(|(l, k)| c.atlas.measure(Weight::SemiBold, px * s, &header_text(l, k, keys)) + 26.0 * s).sum();
    row + if with_head { c.atlas.measure(Weight::SemiBold, 16.0 * s, head) + 12.0 * s } else { 0.0 }
}

/// `text`'s width with every digit as wide as the font's widest (Inter's
/// digits are proportional), so a number that ticks keeps what follows it
/// still (the steady-HUD pass, 11-20).
fn steady_w(c: &Canvas, w: Weight, px: f32, text: &str) -> f32 {
    let dw = |d: char| c.atlas.measure(w, px, d.encode_utf8(&mut [0; 4]));
    let wide = ('0'..='9').max_by(|a, b| dw(*a).total_cmp(&dw(*b))).unwrap_or('0');
    let t: String = text.chars().map(|ch| if ch.is_ascii_digit() { wide } else { ch }).collect();
    c.atlas.measure(w, px, &t).max(c.atlas.measure(w, px, text))
}

/// The squad count over the tiles.
fn squads_head(n: usize) -> String {
    if n == 1 { "1 squad".to_string() } else { format!("{n} squads") }
}
/// A padlock `h` tall with its top left at `p`, 0.8 h wide: the shackle,
/// then the body over its foot.
fn lock_glyph(c: &mut Canvas, p: (f32, f32), h: f32, col: Rgba) {
    c.outline(Rect::new(p.0 + 0.17 * h, p.1, 0.46 * h, 0.62 * h), 0.23 * h, 0.12 * h, col);
    c.rrect(Rect::new(p.0, p.1 + 0.42 * h, 0.8 * h, 0.58 * h), 0.12 * h, col);
}

/// The loadout strip's card slots and its name column, UI units: 34 and
/// 112 until the UI review (11-15), a 51-unit strip in a 105-unit panel
/// with the gun's name at 12 pixels.
const LOADOUT_CELL: f32 = 46.0;
const LOADOUT_INFO: f32 = 124.0;
/// The loadout button's width (30 to 11-24, a square with only its key;
/// now wide enough for its word under the key, the polish review).
const LOADOUT_SWAP: f32 = 54.0;

/// A command's place on the card: its `slot` (row, column), kept on the
/// grid. (Until Card 2, 2026-11-15, slots were the old 4 x 3 card's, read
/// in order onto the 6 x 2 one.)
fn card_slot(slot: (u8, u8)) -> (u8, u8) {
    (slot.0.min(CARD_ROWS - 1), slot.1.min(CARD_COLS - 1))
}
/// The selection panel's height with nothing selected.
const COLLAPSED: f32 = 40.0;

// Pixels a UI unit: `UiSize::px` (`settings`), the height's share of 1080
// lines with the window's DPI and the player's size over it (11-14).

/// Health's colour: green, then amber below 60%, red below 30%.
pub fn health_colour(f: f32) -> Rgba {
    if f > 0.6 {
        GOOD
    } else if f > 0.3 {
        WARN
    } else {
        BAD
    }
}

/// A line `px` screen pixels wide between two world points (marks' lines
/// are one pixel: parallel ones, `zoom` pixels a world cell apart), on a
/// dark edge either side when `ink` so it reads on bright slime as on bare
/// ground, crisp at any zoom (owner 11-14: play is zoomed in).
fn stroke(world: &mut Marks, a: (f32, f32), b: (f32, f32), px: u32, zoom: f32, col: Rgba, ink: bool) {
    let (dx, dy) = (b.0 - a.0, b.1 - a.1);
    let len = dx.hypot(dy);
    if len <= 0.0 {
        return;
    }
    let (nx, ny) = (-dy / len / zoom.max(0.01), dx / len / zoom.max(0.01));
    let half = (px as f32 - 1.0) / 2.0;
    let mut at = |o: f32, c: Rgba| world.line((a.0 + nx * o, a.1 + ny * o), (b.0 + nx * o, b.1 + ny * o), 0.0, c);
    if ink {
        let edge = [14, 13, 10, (col[3] as f32 * 0.6) as u8];
        at(-half - 1.0, edge);
        at(half + 1.0, edge);
    }
    for i in 0..px {
        at(i as f32 - half, col);
    }
}

/// A dashed ring of short strokes (a big `Ring` mark walks its whole box:
/// 0.7 ms at 480 pixels, the turret's reach ring), `dashes` dashes round.
fn dashed_ring(world: &mut Marks, c: (f32, f32), r: f32, dashes: u32, px: u32, zoom: f32, col: Rgba) {
    let n = 2 * dashes.max(3);
    for i in (0..n).step_by(2) {
        let (a0, a1) = (i as f32 / n as f32 * std::f32::consts::TAU, (i + 1) as f32 / n as f32 * std::f32::consts::TAU);
        stroke(world, (c.0 + r * a0.cos(), c.1 + r * a0.sin()), (c.0 + r * a1.cos(), c.1 + r * a1.sin()), px, zoom, col, false);
    }
}

/// A throw's reach ring and arc while it can go: the cream of the order
/// lines and selection rings, which reads on the gold slime.
pub const THROW_LINE: Rgba = [250, 242, 220, 255];

/// How high a throw's arc bows over its ground line: a share of its length.
pub const ARC_RISE: f32 = 0.28;
/// Dashes along a throw's arc.
const ARC_DASHES: u32 = 14;

/// The arc's point `f` of the way from `a` to `b` (0-1): along the ground
/// line, lifted toward the screen's top by a parabola `ARC_RISE` of the
/// length high in the middle, so a top-down throw reads as lobbed.
pub fn arc_point(a: (f32, f32), b: (f32, f32), f: f32) -> (f32, f32) {
    let len = (b.0 - a.0).hypot(b.1 - a.1);
    let lift = 4.0 * f * (1.0 - f) * ARC_RISE * len;
    (a.0 + (b.0 - a.0) * f, a.1 + (b.1 - a.1) * f - lift)
}

/// A throw's aim (`Throw`): the thrower's reach as a dashed ring, a ring on
/// the thrower, its arc to `land` (dashed, brightening toward the landing)
/// over a faint ground line, and the rings past the area. Out of reach
/// (`ghost_ok` false, `col` red): the arc stops at the reach's edge with a
/// mark where the throw would fall short, and dots on to the cursor.
fn throw_aim(w: &Throw, land: (f32, f32), col: Rgba, area: Rgba, zoom: f32, world: &mut Marks) {
    // The arc first, then the rings: marks past `MAX_MARKS` are dropped,
    // and a busy frame dropped the arc when it came last. About 250 marks
    // in all (the ring's 2-pixel dashes cost 2 lines each, inked 4).
    let from = w.from;
    let (dx, dy) = (land.0 - from.0, land.1 - from.1);
    let d = dx.hypot(dy);
    let short = d > w.reach;
    world.ring(from, w.from_r + 1.5 / zoom.max(0.01), [14, 13, 10, 170]);
    world.ring(from, w.from_r, col);
    if d > 0.0 {
        // The ground line: where the throw passes, dashed every 4 pixels.
        world.line(from, land, 4.0, alpha([14, 13, 10, 255], 0.35));
        let reach_f = if short { w.reach / d } else { 1.0 };
        let n = 2 * ARC_DASHES;
        for i in (0..n).step_by(2) {
            let (f0, f1) = (i as f32 / n as f32, (i + 1) as f32 / n as f32);
            if f0 >= reach_f {
                // Past the reach: a dot every other dash, faint.
                if (i / 2) % 2 == 0 {
                    world.ring_thin(arc_point(from, land, f0), 1.5, 1.5, alpha(col, 0.5));
                }
                continue;
            }
            let a = 0.45 + 0.55 * f1;
            stroke(world, arc_point(from, land, f0), arc_point(from, land, f1.min(reach_f)), 3, zoom, alpha(col, a), true);
        }
        if short {
            // Where it would fall: a cross at the reach's edge on the ground.
            let p = (from.0 + dx / d * w.reach, from.1 + dy / d * w.reach);
            let k = 5.0 / zoom.max(0.01);
            stroke(world, (p.0 - k, p.1 - k), (p.0 + k, p.1 + k), 2, zoom, col, true);
            stroke(world, (p.0 - k, p.1 + k), (p.0 + k, p.1 - k), 2, zoom, col, true);
        }
    }
    if !short {
        // The landing: a small ring at the arc's end.
        world.ring_thin(land, 5.0, 1.0, [14, 13, 10, 200]);
        world.ring_thin(land, 4.0, 2.0, col);
    }
    // The reach: 2-pixel dashes about every 2 cells of the ring, 30-60.
    let dashes = ((std::f32::consts::TAU * w.reach / 4.0) as u32).clamp(30, 60);
    dashed_ring(world, from, w.reach, dashes, 2, zoom, alpha(col, if short { 0.9 } else { 0.75 }));
    for &(r, a) in &w.rings {
        let n = ((std::f32::consts::TAU * r / 2.0) as u32).clamp(12, 32);
        dashed_ring(world, land, r, n, 1, zoom, [area[0], area[1], area[2], a]);
    }
}

fn alpha(c: Rgba, a: f32) -> Rgba {
    [c[0], c[1], c[2], (c[3] as f32 * a.clamp(0.0, 1.0)) as u8]
}

/// A command waiting for its point: its id, its target, and a line's
/// first end once pressed.
pub type Aim = (u32, Target, Option<(f32, f32)>);

/// A placement under way: the command's id, the cursor's world point, and
/// a line's first end.
pub type Ghost = (u32, (f32, f32), Option<(f32, f32)>);

/// What a command needs from the world before it is given.
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum Target {
    /// Given at once (a stance, a purchase, a weapon swap).
    Instant,
    /// A point: a ghost disc `radius` world cells (bait, a turret).
    Point { radius: f32 },
    /// A square or rectangle, half-extents in world cells (a building).
    Area { half: (f32, f32) },
    /// A line, dragged: a ghost `width` cells wide (a wall).
    Line { width: f32 },
}

/// A command-card button.
#[derive(Clone, Debug, PartialEq)]
pub struct Command {
    /// The game's own id, echoed back in `HudEvent`s.
    pub id: u32,
    /// Row (0-1) and column (0-4) on the card.
    pub slot: (u8, u8),
    pub icon: Icon,
    pub name: String,
    pub key: KeyCode,
    /// What it costs, as the game words it ("300"), shown with `cost_icon`.
    pub cost: Option<String>,
    pub cost_icon: Icon,
    /// It can be used now.
    pub ready: bool,
    /// It is the current choice (a stance, the weapon in hand).
    pub on: bool,
    /// Cooling down: 1 just used, 0 ready.
    pub cooldown: f32,
    /// The time left, in the button's middle while it cools ("7s").
    pub wait: Option<String>,
    pub target: Target,
    /// The tooltip's body.
    pub about: String,
    /// Shown in the tooltip, in red, when it is not ready.
    pub why_not: Option<String>,
    /// Not had yet (a run's shop sells it): drawn with a lock, not a price.
    pub locked: bool,
    /// Keys that give another command at once, tried before every button's
    /// own key: the fire button's Z, X and C pick hold, defend and free
    /// while a click cycles them (Card 2, 2026-11-15).
    pub also: Vec<(KeyCode, u32)>,
}

impl Command {
    pub fn new(id: u32, slot: (u8, u8), icon: Icon, name: &str, key: KeyCode) -> Self {
        Command { id, slot, icon, name: name.into(), key, cost: None, cost_icon: Icon::Crystal, ready: true, on: false, cooldown: 0.0, wait: None, target: Target::Instant, about: String::new(), why_not: None, locked: false, also: Vec::new() }
    }
}

/// A resource on the top bar.
#[derive(Clone, Debug, PartialEq)]
pub struct Resource {
    pub icon: Icon,
    pub colour: Rgba,
    pub label: String,
    pub value: String,
    /// "+12/s", green; a leading '-' draws it red.
    pub rate: Option<String>,
}

/// One selected unit's card.
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct Card {
    pub id: usize,
    pub icon: Icon,
    pub team: u8,
    /// Health, 0-1.
    pub hp: f32,
    /// A small badge: its weapon or what it is doing.
    pub badge: Option<Icon>,
}

/// One thing in detail: a selected unit, or the enemy or mould under the
/// cursor.
#[derive(Clone, Debug, PartialEq)]
pub struct Info {
    pub icon: Icon,
    pub colour: Rgba,
    pub title: String,
    pub subtitle: String,
    /// Health, 0-1, and its words ("62 / 100").
    pub hp: Option<(f32, String)>,
    /// Label and value pairs, two columns.
    pub stats: Vec<(String, String)>,
    pub note: Option<String>,
}

/// What a gun card does, which sets its frame (a spell deck's card types:
/// the shot, what changes it, what sets it off).
#[derive(Clone, Copy, Debug, PartialEq, Eq, Default)]
pub enum CardKind {
    /// Casts where the shot hit (blast, gas, germ).
    #[default]
    Payload,
    /// Changes the next payload (wide, scatter, quiet).
    Modifier,
    /// Decides when the next payload goes off (delay, thick).
    Trigger,
    /// Changes every bullet in flight (pierce, split).
    Flight,
}

impl CardKind {
    pub fn label(self) -> &'static str {
        match self {
            // In plain words (owner 11-15 02:57Z: the cards' text "should
            // be simplified and clarified"): what the card works on.
            // Until then: Payload, Modifier, Trigger, Flight.
            CardKind::Payload => "Bullet",
            CardKind::Modifier | CardKind::Trigger => "Next bullet",
            CardKind::Flight => "Always",
        }
    }

    /// The frame's colour: a payload's frame is its own colour.
    pub fn colour(self, own: Rgba) -> Rgba {
        match self {
            CardKind::Payload => own,
            CardKind::Modifier => [108, 156, 255, 255],
            CardKind::Trigger => [242, 192, 84, 255],
            CardKind::Flight => [96, 214, 214, 255],
        }
    }
}

/// One card in a loadout (`Loadout::cards`), left to right.
#[derive(Clone, Debug, PartialEq, Default)]
pub struct GunCard {
    pub icon: Option<Icon>,
    /// A small glyph in its corner: a rolled bullet's set (slug, needle,
    /// buckshot, standard) beside its payload's glyph.
    pub badge: Option<Icon>,
    pub name: String,
    pub kind: CardKind,
    /// The glyph's colour: the colour of the payload it goes with, as its
    /// bullets fly.
    pub colour: Rgba,
    /// What it does, for its tooltip.
    pub about: String,
    /// In the cast the gun's next shot fires: lit.
    pub next: bool,
    /// After the last payload: it casts nothing.
    pub idle: bool,
}

/// A gun's loadout, as a spell deck shows it (the owner, 11-14 01:49Z:
/// "I also have no way to see what the gun cards are in the interface").
#[derive(Clone, Debug, PartialEq, Default)]
pub struct Loadout {
    /// "Trap".
    pub name: String,
    /// Whose gun: "Soldier 3", "Squad (5)".
    pub whose: String,
    pub cards: Vec<GunCard>,
    /// Slots the gun has (empty ones are drawn empty).
    pub slots: usize,
    /// A line under the cards ("Cost 7 of 10"); the game's words.
    pub note: Option<String>,
    /// The command that swaps the loadout, and its key: the panel's
    /// button sends `HudEvent::Command` with it.
    pub cycle: Option<(u32, KeyCode)>,
    /// The gun the cards are in (`mould::guns`): its icon, its name
    /// ("Lucky carbine") and its numbers, shown by the name and in the
    /// tooltip. None draws the strip as before.
    pub gun: Option<(Icon, String, String)>,
    /// The most slots of any gun in play (every squad's), so the strip
    /// keeps one width as the commanded squad changes; 0 for this gun's
    /// own (`slots`). The room past this gun's slots goes to its name.
    pub widest: usize,
}

/// One squad on the squad panel (`SquadPanel::rows`).
#[derive(Clone, Debug, PartialEq, Default)]
pub struct SquadRow {
    /// Its hotkey ("1").
    pub key: String,
    /// "Squad 1".
    pub name: String,
    pub soldiers: usize,
    /// Its soldiers' mean health, 0 to 1.
    pub hp: f32,
    /// A short state after the count ("line", "guarding").
    pub note: Option<String>,
    /// The squad you command.
    pub active: bool,
    /// Close enough to the one you command to combine (its + shows; owner
    /// 2026-11-14 15:06Z: combine "should only appear when squads overlap").
    pub near: bool,
    /// Linked with the one you command (`Cmd::Link`): they move and take
    /// orders together, as one. Its corner unlinks it; a squad not linked
    /// shows the link button only while `near` (owner 15:49Z: "you can
    /// only link when touching").
    pub linked: bool,
    /// Runs on its own orders: neither the one you command nor linked with
    /// it. Its tile says AUTO (owner 2026-11-15 02:58Z).
    pub auto: bool,
    /// What it is doing, one word or two ("firing", "holding"), on its
    /// tile; the note keeps the rest. Empty: the note's first word.
    pub state: String,
}

/// The squad mode's squads, drawn in the selection panel's place as a
/// tile a squad, as the soldiers' cards are in the RTS round (owner,
/// 08:48Z: "This is supposed to replace the bottom middle menu each squad
/// should be a tile how each solider is now"; a first cut at 08:40Z was a
/// separate panel at the top right). Asked for at 2026-11-14 08:22Z: "list each squad with
/// the number of soliders and its hotkey, you can click to take command.
/// There should also be buttons, to combine, to split"). A row click sends
/// `HudEvent::Squad(row)`, its + sends `HudEvent::SquadInto(row)` (that
/// squad joins yours), and the two buttons `SquadSplit` and `SquadCombine`.
#[derive(Clone, Debug, PartialEq, Default)]
pub struct SquadPanel {
    pub rows: Vec<SquadRow>,
    /// The buttons' keys ("X"), shown on them.
    pub split_key: Option<String>,
    pub combine_key: Option<String>,
    /// Why a button would do nothing now (greyed, the reason its tooltip).
    pub split_off: Option<String>,
    pub combine_off: Option<String>,
    /// No squad commanded (Esc): the header says so and Let go greys out.
    pub free: bool,
    /// Let go's key ("Esc"), shown on it; it sends `HudEvent::SquadFree`.
    pub free_key: Option<String>,
}

/// One of the commanded squad's settings on the squad panel (controls
/// review, 2026-11-21: with one to three squads the tiles fill a third of
/// the panel on a wide screen and the rest sat empty, while the squad's
/// formation, queue and tools showed nowhere). An icon and a few words
/// ("line", "2 queued"), and its tooltip. The window lists them most
/// wanted first; the strip drops from the end what does not fit.
#[derive(Clone, Debug, PartialEq)]
pub struct SquadFact {
    pub icon: Icon,
    pub text: String,
    pub about: String,
}

/// The landing card (`Model::intro`): "Planet 4 of 8, Nezanus" over the
/// slime's name and its one line, at `alpha`.
#[derive(Clone, Debug, PartialEq, Default)]
pub struct Intro {
    pub kicker: String,
    pub title: String,
    pub line: String,
    /// The planet's twist, if it rolled one (`run::PlanetTwist`, Enemy's
    /// #830): its name and its line, under the slime's in the twist's
    /// ice blue.
    pub twist: Option<(String, String)>,
    pub alpha: f32,
}

/// A planet twist's colour on the HUD: an ice blue, apart from the slime's
/// amber and the accent's cyan (the cold snap and the wind alike).
pub const TWIST: Rgba = [156, 208, 255, 255];

/// What the game shows this frame.
#[derive(Clone, Debug, PartialEq, Default)]
pub struct Model {
    pub resources: Vec<Resource>,
    pub clock: String,
    /// "Paused", "x4": a chip beside the clock.
    pub speed: Option<String>,
    /// The experiments that are on, each as its name and choice ("Shake
    /// and hit-stop on"): a small amber mark under the clock, so a round
    /// that is not the default game says so for as long as it is played.
    pub trying: Vec<String>,
    /// The declared switches, for the pause menu's Experiments page (none:
    /// no button), and how many the plan allows at once (`MOST_OFF`).
    pub experiments: Vec<Experiment>,
    pub most_off: usize,
    pub objective: Option<String>,
    pub selection: Vec<Card>,
    /// Drawn instead of the cards when set.
    pub info: Option<Info>,
    pub commands: Vec<Command>,
    /// Local units with nothing to do.
    pub idle: usize,
    /// Whether the targeting ghost is somewhere the command can go; a
    /// placement where it cannot is refused (targeting stays) and
    /// `ghost_note` says why.
    pub ghost_ok: bool,
    /// Beside the cursor while placing: the cost of what is drawn so far,
    /// or why it cannot go there.
    pub ghost_note: Option<String>,
    /// A one-time hint pinned beside a world point (fine cells): its words,
    /// the point, how far below it to sit (world cells: the thing's own
    /// reach, so it never covers it) and its opacity. The first Starfall or
    /// core of a run says how to collect it (owner, 2026-11-22).
    pub hint: Option<(String, (f32, f32), f32, f32)>,
    /// The ring round a first-sight hint's thing (fine cells, 0 for none):
    /// the simulation's own size for it, a body's radius or a reach
    /// (`hints.rs` in the window).
    pub hint_ring: f32,
    /// The planet's introduction at landing (Direction 11-24: each planet
    /// does one nameable thing to you, and nothing on screen named it): a
    /// card under the goal line naming the slime and its one line, fading.
    pub intro: Option<Intro>,
    /// The planet's slime's trait as a number the simulation keeps
    /// (Direction 11-24: one tell per trait, no invented meters): its icon,
    /// the short words ("62% asleep") and the hover line, in a small pill
    /// under the goal line.
    pub tell: Option<(Icon, String, String)>,
    /// The planet's twist now (`Game::twist_now`): its icon, short words
    /// ("Cold snap in 0:12", "Wind from the west") and hover, a pill in
    /// `TWIST` beside the tell's.
    pub twist: Option<(Icon, String, String)>,
    /// The first minute's coaching (the window's `Coach`): the step's
    /// words, "1 of 3" and its opacity, in a panel over the command bar.
    pub coach: Option<(String, String, f32)>,
    /// Where the ghost snaps to instead of the cursor, and its radius in
    /// world cells (a pump onto its crystal's middle, owner 2026-11-15:
    /// "it should automatically center itself on the crystal").
    pub ghost_snap: Option<((f32, f32), f32)>,
    /// A small card by the cursor for what is under it (an enemy, the
    /// mould), leaving the selection panel alone.
    pub hover: Option<Info>,
    /// The round is over: what the end screen shows.
    pub end: Option<Ending>,
    /// Extra lines for the controls page (key, what it does).
    pub help: Vec<(String, String)>,
    /// The rows that open the key card in place of the RTS controls' own
    /// (selecting, groups, the wheel...), for a game whose mouse and keys
    /// mean something else (the squad mode).
    pub help_base: Option<Vec<(String, String)>>,
    /// The player's buildings, for whoever draws them (the window draws
    /// plain marks now; the graphics thread's sprites read this).
    pub buildings: Vec<Site>,
    /// The game has a start menu: the menu and the end screen offer "Main
    /// menu" (`MenuItem::MainMenu`).
    pub main_menu: bool,
    /// Lines the pause menu shows under "Esc to resume": in a roguelike
    /// run, what quitting mid-planet keeps (Milestone 2's save and resume,
    /// step C, 2026-11-24: the run is saved as it landed, the planet is
    /// not, so it relands fresh).
    pub pause_note: Vec<String>,
    /// In a run, the run as the pause menu shows it (the second polish
    /// pass, 11-24: the pause menu said nothing of where you were): the
    /// planet, the run's strip and its tally (`start::run_strip`).
    pub pause_run: Option<super::start::RunView>,
    /// The planet's landing card as the pause menu repeats it (Direction
    /// 11-24: "so he can check them mid-planet without leaving"): the
    /// slime's name and line, and the twist's, under the goal
    /// (`objective`). The card's `alpha` is not read here.
    pub pause_planet: Option<Intro>,
    /// The end screen's first button carries on (`MenuItem::Continue`)
    /// under this label, instead of "Play again" (a roguelike run: the
    /// shop, or the run's end).
    pub carry_on: Option<String>,
    /// The gun's cards, drawn as a strip on the selection panel's top edge.
    pub loadout: Option<Loadout>,
    /// The squad mode's squads, a panel at the top right.
    pub squads: Option<SquadPanel>,
    /// The commanded squad's settings, a strip right of its tiles where
    /// the row leaves room (`SquadFact`).
    pub squad_facts: Vec<SquadFact>,
    /// The player is steering (WASD in the squad mode): the screen moves
    /// under a still cursor, so the hover card stays away (`HoverCard`).
    pub aiming: bool,
    /// The ghost is a throw (a toolkit tool; a squad's device later): drawn
    /// with its thrower, reach and arc as well as where it lands.
    pub throw: Option<Throw>,
}

/// A throw being aimed (`Model::throw`; owner 2026-11-16: "when throwing
/// an explosive, we should improve the visual for aiming and range"). Any
/// point or line command can carry one, so a squad's device slot (the gun
/// thread's step E) aims with the same ghost as the Tools page.
#[derive(Clone, Debug, PartialEq, Default)]
pub struct Throw {
    /// Who throws: the soldier nearest the landing point that may.
    pub from: (f32, f32),
    /// The thrower's drawn radius, world cells (a ring round it).
    pub from_r: f32,
    /// How far it throws, world cells (a dashed ring round `from`).
    pub reach: f32,
    /// The tool's colour, for the area and the arc while it can go.
    pub tint: Rgba,
    /// Where it lands when that is not the cursor (a det cord's middle).
    pub at: Option<(f32, f32)>,
    /// Dashed rings round the landing point, world cells, with their
    /// alpha: what the throw reaches past its area (a blast's crater and
    /// the alarm it wakes the mould with).
    pub rings: Vec<(f32, u8)>,
}

/// One building as the view sees it (`Model::buildings`).
#[derive(Clone, Debug, PartialEq, Default)]
pub struct Site {
    /// "turret", "lure", "pump", "outpost" or "wall".
    pub kind: &'static str,
    /// World cells; a wall's far end in `to`.
    pub at: (f32, f32),
    pub to: Option<(f32, f32)>,
    /// Footprint radius, world cells.
    pub r: f32,
    pub hp: f32,
    pub max_hp: f32,
    /// 0 a ghost, 1 whole.
    pub built: f32,
    /// Doing its job this frame: a turret firing, a lure laying scent, a
    /// pump drawing charge.
    pub working: bool,
}

/// What the end screen shows (the Look and interface lane, 2026-11-18;
/// the brief's "win and lose screens that show the planet's numbers").
/// Before, a title and three grey lines, the last cut off with "..." at
/// 100%: a run's planet was not named, nor its slime, nor what it did.
#[derive(Clone, Debug, Default, PartialEq)]
pub struct Ending {
    /// Small, over the title: where ("Planet 3 of 8 · Dreskor · the
    /// Trunk"), in the outcome's colour.
    pub place: String,
    pub title: String,
    /// A win (green), a loss (red), or neither (a sandbox round at its
    /// clock: the accent).
    pub won: Option<bool>,
    /// A mission's stars, of three: three stars under the title, the
    /// earned ones filled in the outcome's colour.
    pub stars: Option<u8>,
    /// What the slime did to you, one sentence (a run's planet:
    /// `run::does_to_you`), under the title.
    pub says: String,
    /// The round's numbers, each a value and what it counts, three to a
    /// row.
    pub tiles: Vec<(String, String)>,
    /// Lines under the numbers: how it ended, the run's points.
    pub lines: Vec<String>,
    /// The slime's walkers and the soldiers standing, a game second each
    /// (`Game::curve`), drawn as a graph under the numbers; empty for none.
    /// The owner, 2026-11-21: "I would be curious to see slime population
    /// overtime after each match".
    pub curve: Vec<(u32, u32)>,
}

impl Ending {
    /// A plain ending: a title and lines (the tests' and a film's).
    pub fn plain(title: &str, lines: Vec<String>) -> Ending {
        Ending { title: title.to_string(), lines, ..Ending::default() }
    }
}

/// **What a declared switch says of itself** (`build::switches`): who
/// owns it, when it landed, what the player will see with it on and why
/// it is not yet the default game. The start menu's Experiments tab and
/// the pause menu's Experiments page draw the same card from it
/// (`experiment_card`; the Look and interface lane, 2026-11-19). Until
/// then the tab was Options' plain one-line rows, with all of this run
/// together in the note pane ("... Not on yet: ...") and no lane, date or
/// count anywhere.
#[derive(Clone, Debug, Default, PartialEq)]
pub struct ExpInfo {
    pub lane: String,
    /// YYYY-MM-DD.
    pub since: String,
    pub sees: String,
    pub why_off: String,
    /// A pick takes effect at once in play (its keys are all the
    /// window's own: shake and hit-stop); else at the next round.
    pub live: bool,
}

/// One switch as the pause menu's Experiments page shows it
/// (`Model::experiments`).
#[derive(Clone, Debug, Default, PartialEq)]
pub struct Experiment {
    /// The switch's key (`try=KEY`).
    pub key: String,
    pub label: String,
    pub choices: Vec<String>,
    pub default: usize,
    /// The choice the round plays now.
    pub at: usize,
    /// The choice picked in play that waits for the next round (a switch
    /// that is not `live`), when it differs from `at`.
    pub next: Option<usize>,
    pub info: ExpInfo,
}

impl Experiment {
    /// The choice the card lights: the pick waiting, else what plays.
    pub fn picked(&self) -> usize {
        self.next.unwrap_or(self.at)
    }
}

/// The card's widths and sizes, UI units.
const EXP_PAD: f32 = 18.0;
/// The column the "You will see" and "Off because" captions sit in.
const EXP_CAPTION: f32 = 112.0;
const EXP_LEAD: f32 = 21.0;

/// The text column's width in a card `w` wide.
fn exp_text_w(w: f32, s: f32) -> f32 {
    (w - (2.0 * EXP_PAD + 6.0 + EXP_CAPTION) * s).max(60.0 * s)
}

/// **A switch's card's height**, `w` wide: its name and state, a line of
/// who and when, and the two sentences, each wrapped whole (a cut
/// sentence is the thing the owner was missing).
pub fn experiment_card_h(c: &Canvas, w: f32, e: &Experiment, s: f32) -> f32 {
    let tw = exp_text_w(w, s);
    let n = |t: &str| if t.is_empty() { 0.0 } else { c.wrap(Weight::Medium, 15.0 * s, tw, t).len() as f32 };
    // The first sentence's baseline 86 down, 6 between the two, 18
    // under the last.
    89.0 * s + (n(&e.info.sees).max(1.0) + n(&e.info.why_off).max(1.0)) * EXP_LEAD * s
}

/// **Draws a switch's card** in `r` (`experiment_card_h` tall) and gives
/// back each of its choices' buttons, by choice. Left to right and top to
/// bottom: a stripe down its left edge, amber while the switch is off the
/// default game, grey at the default; its name and a state chip (ON in
/// amber, OFF in grey, ON AT RESTART outlined while a pick waits); its
/// choices as one segmented button at the right, the picked one lit
/// (amber off the default, pale at it); who owns it, the day it landed
/// and when a pick applies, muted; then what you will see and why it is
/// off, whole, under small captions. `in_play`: the pause menu's card,
/// whose last word says when a pick here applies; the start menu's says
/// when it would in play.
#[allow(clippy::too_many_arguments)]
pub fn experiment_card(c: &mut Canvas, r: Rect, e: &Experiment, hover: Option<(f32, f32)>, focused: bool, in_play: bool, s: f32) -> Vec<(Rect, usize)> {
    let on = e.at != e.default;
    let picked = e.picked();
    let lit = picked != e.default;
    let hot = hover.is_some_and(|p| r.contains(p));
    c.panel(r, 12.0 * s, [255, 255, 255, if hot || focused { 16 } else { 9 }], [255, 255, 255, if hot || focused { 10 } else { 5 }], 1.0, if lit { alpha(WARN, 0.42) } else { EDGE });
    if focused {
        c.outline(r.inset(-2.0 * s), 14.0 * s, 1.5 * s, ACCENT);
    }
    c.rrect(Rect::new(r.x + 7.0 * s, r.y + 12.0 * s, 4.0 * s, r.h - 24.0 * s), 2.0 * s, if lit { WARN } else { [255, 255, 255, 34] });
    let x = r.x + (EXP_PAD + 6.0) * s;
    let right = r.right() - EXP_PAD * s;
    // The choices, as one segmented button at the right of the first
    // line: "custom" (a film's own number) only while it is what plays.
    let shown: Vec<usize> = (0..e.choices.len()).filter(|&k| e.choices[k] != "custom" || k == e.at).collect();
    let seg_h = 30.0 * s;
    let seg_w: Vec<f32> = shown.iter().map(|&k| (c.atlas.measure(Weight::SemiBold, 14.5 * s, &e.choices[k]) + 30.0 * s).max(58.0 * s)).collect();
    let total: f32 = seg_w.iter().sum();
    let sy = r.y + 12.0 * s;
    let mut sx = right - total;
    let group = Rect::new(sx, sy, total, seg_h);
    c.panel(group, 8.0 * s, BUTTON_TOP, BUTTON_BOTTOM, 1.0, [255, 255, 255, 30]);
    let mut out = Vec::new();
    for (j, &k) in shown.iter().enumerate() {
        let b = Rect::new(sx, sy, seg_w[j], seg_h);
        let hot = hover.is_some_and(|p| b.contains(p));
        if k == picked {
            let fill = if k == e.default { [210, 218, 226, 255] } else { WARN };
            c.rrect(b.inset(2.0 * s), 6.5 * s, fill);
            c.text(Weight::SemiBold, 14.5 * s, (b.x + b.w / 2.0, sy + 20.0 * s), Align::Centre, &e.choices[k], [16, 19, 24, 255]);
        } else {
            if hot {
                c.rrect(b.inset(2.0 * s), 6.5 * s, [255, 255, 255, 22]);
            }
            c.text(Weight::SemiBold, 14.5 * s, (b.x + b.w / 2.0, sy + 20.0 * s), Align::Centre, &e.choices[k], if hot { TEXT } else { MUTED });
        }
        out.push((b, k));
        sx += seg_w[j];
    }
    // The name, and its state beside it.
    let base = r.y + 33.0 * s;
    let lw = c.text_fit(Weight::SemiBold, 18.5 * s, (x, base), Align::Left, &e.label, (group.x - x - 170.0 * s).max(80.0 * s), TEXT);
    let (word, ink, filled) = match (on, e.next.filter(|&k| k != e.at)) {
        (_, Some(k)) if k != e.default => ("ON AT RESTART".to_string(), WARN, false),
        (_, Some(_)) => ("OFF AT RESTART".to_string(), MUTED, false),
        (true, None) if e.choices[e.at] == "on" => ("ON".to_string(), WARN, true),
        (true, None) => (e.choices[e.at].to_uppercase(), WARN, true),
        (false, None) => ("OFF".to_string(), MUTED, true),
    };
    let cw = c.atlas.measure(Weight::SemiBold, 12.0 * s, &word) + 18.0 * s;
    let chip = Rect::new(x + lw + 12.0 * s, base - 15.0 * s, cw, 20.0 * s);
    if filled {
        c.rrect(chip, chip.h / 2.0, if on { alpha(WARN, 0.2) } else { [255, 255, 255, 20] });
    } else {
        c.outline(chip, chip.h / 2.0, 1.2 * s, alpha(ink, 0.8));
    }
    c.text(Weight::SemiBold, 12.0 * s, (chip.x + chip.w / 2.0, base - 0.5 * s), Align::Centre, &word, ink);
    // Who, when, and when a pick applies.
    let when = match (e.info.live, in_play) {
        (true, true) => "a pick here applies at once",
        (false, true) => "a pick here applies when you restart",
        (true, false) => "in play, Esc changes it at once",
        (false, false) => "in play, Esc changes it for the next round",
    };
    let meta = format!("{}  ·  landed {}  ·  {when}", e.info.lane, e.info.since);
    c.text_fit(Weight::Medium, 13.5 * s, (x, r.y + 57.0 * s), Align::Left, &meta, right - x, MUTED);
    // The two sentences, whole, under their captions.
    let tw = exp_text_w(r.w, s);
    let tx = x + EXP_CAPTION * s;
    let mut y = r.y + 86.0 * s;
    for (caption, text, ink) in [("YOU WILL SEE", &e.info.sees, TEXT), ("OFF BECAUSE", &e.info.why_off, [196, 204, 214, 255])] {
        c.text(Weight::SemiBold, 11.5 * s, (x, y - 1.0 * s), Align::Left, caption, FAINT);
        let n = c.paragraph(Weight::Medium, 15.0 * s, (tx, y), tw, EXP_LEAD * s, text, ink).max(1);
        y += n as f32 * EXP_LEAD * s + 6.0 * s;
    }
    out
}

/// **The count against the limit** (the plan of 2026-11-17, rule 3: at
/// most `most` switches off the built game at once, across all lanes): a
/// slot a switch the list may hold, the declared ones filled (bright
/// while on in this game), and the words. From `at` (its left end and its
/// text's baseline); gives back its width.
pub fn experiment_meter(c: &mut Canvas, at: (f32, f32), declared: usize, on: usize, most: usize, s: f32) -> f32 {
    let (x, base) = at;
    let (sw, sh, gap) = (22.0 * s, 10.0 * s, 4.0 * s);
    for k in 0..most.max(declared) {
        let slot = Rect::new(x + k as f32 * (sw + gap), base - 10.0 * s, sw, sh);
        let fill = if k < on {
            WARN
        } else if k < declared {
            alpha(WARN, 0.38)
        } else {
            TRACK
        };
        c.rrect(slot, 3.0 * s, fill);
    }
    let tx = x + most.max(declared) as f32 * (sw + gap) + 8.0 * s;
    let words = format!("{declared} of {most} allowed  ·  {on} on in this game");
    tx - x + c.text(Weight::SemiBold, 14.0 * s, (tx, base), Align::Left, &words, if on > 0 { WARN } else { TEXT })
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum MenuItem {
    Opened,
    Resume,
    Restart,
    /// Back to the start menu (`Model::main_menu`).
    MainMenu,
    /// The end screen's first button when the game names one
    /// (`Model::carry_on`: a run's "On to the shop").
    Continue,
    Quit,
}

#[derive(Clone, Debug, PartialEq)]
pub enum HudEvent {
    /// An instant command.
    Command { id: u32 },
    /// A targeted command, placed at a world point.
    Point { id: u32, at: (f32, f32) },
    /// A line command, dragged between two world points.
    Line { id: u32, from: (f32, f32), to: (f32, f32) },
    /// A card was clicked: select that unit alone (Shift: drop it).
    SelectOnly(usize),
    Deselect(usize),
    /// A group tab was clicked: select group `g` (0-8).
    Group(usize),
    /// The idle button: select the next idle unit.
    Idle,
    Menu(MenuItem),
    /// The pause menu's Experiments page: `Model::experiments[row]` set to
    /// its choice `choice`.
    Experiment { row: usize, choice: usize },
    /// A command was pressed that is not ready; the hud shows why.
    NotReady { id: u32 },
    /// The squad panel: take command of the squad on row `r`.
    Squad(usize),
    /// The squad panel: the squad on row `r` joins the one you command.
    SquadInto(usize),
    /// The squad panel: link the squad on row `r` with the one you command,
    /// or unlink it.
    SquadLink(usize),
    /// The squad panel's Split and Combine buttons.
    SquadSplit,
    SquadCombine,
    /// The squad panel's Let go: command no squad.
    SquadFree,
}

#[derive(Clone, Copy, Debug, PartialEq)]
enum Hit {
    Panel,
    Command(usize),
    Card(usize),
    Group(usize),
    Idle,
    MenuButton,
    Menu(MenuItem),
    Help,
    /// The pause menu's Experiments button, and a choice on its page.
    ExpPage,
    Exp(usize, usize),
    Back,
    /// The loadout panel's swap button.
    Cycle,
    /// The squad panel's row, its +, and its two buttons.
    Squad(usize),
    SquadInto(usize),
    SquadLink(usize),
    SquadSplit,
    SquadCombine,
    SquadFree,
}

#[derive(Clone, Debug)]
struct Alert {
    text: String,
    at: Option<(f32, f32)>,
    colour: Rgba,
    when: Instant,
}

/// How long an alert stays, and fades.
const ALERT_LIFE: Duration = Duration::from_secs(6);
const ALERT_FADE: f32 = 1.0;
const ALERTS_MAX: usize = 5;

#[derive(Clone, Copy, Debug, PartialEq)]
struct Targeting {
    index: usize,
    id: u32,
    target: Target,
    from: Option<(f32, f32)>,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
enum MenuPage {
    Main,
    Help,
    /// The declared switches, each as the start menu's card
    /// (2026-11-19): set from play.
    Experiments,
}

pub struct Hud {
    /// Pixels a UI unit (1 at 1080 lines).
    pub scale: f32,
    hits: Vec<(Rect, Hit)>,
    hover: Option<(f32, f32)>,
    pressed: Option<Hit>,
    targeting: Option<Targeting>,
    menu: Option<MenuPage>,
    /// The controls page was opened straight from play (F1 or ?), so Esc,
    /// Back or F1 again closes it rather than going to the main menu page.
    help_direct: bool,
    alerts: Vec<Alert>,
    /// The commands as last drawn (hotkeys and targeting read them).
    commands: Vec<Command>,
    /// The bottom panels' top edge, for the camera's inset.
    pub bottom: f32,
    /// The box the minimap should be fitted into (give it to the view's
    /// `minimap_box`), and the minimap's rectangle in it, as last drawn.
    pub minimap_box: Option<(u32, u32, u32, u32)>,
    pub minimap: Option<(u32, u32, u32, u32)>,
    pub top: f32,
    /// Where the HUD's own words under the top bar start (the alerts): the
    /// bar's foot, or the goal's when it takes a row of its own.
    below: f32,
    /// The world's size in cells, as last drawn (for the minimap).
    world: (f32, f32),
    /// The end-of-round screen is up (its buttons take every click).
    end: bool,
    /// Its first button carries a run on (`Model::carry_on`), as last
    /// drawn: Enter presses it (`key`).
    end_carry: bool,
    /// Which buttons (left, right, middle) the UI took on their press: a
    /// release is the UI's only when its press was, so a drag-box or a pan
    /// that ends over a panel still ends in the controls.
    took: [bool; 3],
    /// The model's `ghost_ok` and `ghost_note`, as last drawn.
    ghost_ok: bool,
    ghost_note: Option<String>,
    /// The loadout's swap command, as last drawn.
    cycle: Option<u32>,
    started: Instant,
    /// The hover card's timing (`HoverCard`).
    card: HoverCard,
    /// The mouse buttons held (left, right, middle): no hover card then.
    held: [bool; 3],
    /// The bottom row as last laid out, for the layout tests: the panels
    /// left to right (gun, selection, command card, minimap frame) and
    /// what `bar_fit` gave.
    #[cfg(test)]
    row: (Vec<Rect>, bool, GunFit),
}

/// When the card over the world shows (owner, 11-15 07:54Z: "the overlay
/// that pops up when I hover over the molds home crystal is annoying when
/// I am trying to shoot the slime that are always over the crystal. It
/// should maybe go away after a couple seconds"). It waits for the cursor
/// to rest on a thing (`REST`), stays `SHOW` seconds, fades out, and then
/// stays away until the cursor goes to something else; a click puts it
/// away too. Until then it showed the whole time the cursor was on the
/// thing, beside the cursor, over whatever was being shot at.
#[derive(Clone, Debug)]
struct HoverCard {
    /// What the cursor is on (the card's title) and since when it rested.
    key: Option<String>,
    still: Instant,
    at: (f32, f32),
    /// When it showed for `key`; `shut` once it went away.
    shown: Option<Instant>,
    shut: bool,
}

impl HoverCard {
    const REST: f32 = 0.35;
    const SHOW: f32 = 2.0;
    const FADE: f32 = 0.4;

    fn new() -> Self {
        HoverCard { key: None, still: Instant::now(), at: (0.0, 0.0), shown: None, shut: false }
    }

    /// How opaque the card for `key` is now (0: not drawn). `quiet`: a
    /// mouse button is held or the squad is steered, so the player is
    /// aiming. The card goes, and what the cursor reaches meanwhile stays
    /// shut until it is on something else (the three-reviewer review,
    /// 11-15: a click put away only the card already up, so aim sliding
    /// onto the home crystal mid-volley still raised one 0.35 s later).
    fn alpha(&mut self, key: Option<&str>, now: Instant, quiet: bool) -> f32 {
        if self.key.as_deref() != key {
            *self = HoverCard { key: key.map(str::to_string), still: now, at: self.at, shown: None, shut: quiet };
        }
        if quiet {
            self.shut = true;
        }
        if self.key.is_none() || self.shut {
            return 0.0;
        }
        let shown = match self.shown {
            Some(t) => t,
            None if now.duration_since(self.still).as_secs_f32() >= Self::REST => *self.shown.insert(now),
            None => return 0.0,
        };
        let age = now.duration_since(shown).as_secs_f32();
        if age >= Self::SHOW + Self::FADE {
            self.shut = true;
            return 0.0;
        }
        (1.0 - (age - Self::SHOW) / Self::FADE).clamp(0.0, 1.0)
    }

    /// The cursor moved: waiting to show, it starts its rest again.
    fn moved(&mut self, p: (f32, f32), now: Instant) {
        if (p.0 - self.at.0).abs() + (p.1 - self.at.1).abs() > 4.0 {
            self.at = p;
            if self.shown.is_none() {
                self.still = now;
            }
        }
    }
}

impl Default for Hud {
    fn default() -> Self {
        Self::new()
    }
}

impl Hud {
    pub fn new() -> Self {
        Hud { scale: 1.0, hits: Vec::new(), hover: None, pressed: None, targeting: None, menu: None, help_direct: false, alerts: Vec::new(), commands: Vec::new(), bottom: 0.0, minimap_box: None, minimap: None, top: 0.0, below: 0.0, world: (1.0, 1.0), end: false, end_carry: false, took: [false; 3], ghost_ok: true, ghost_note: None, cycle: None, started: Instant::now(), card: HoverCard::new(), held: [false; 3], #[cfg(test)] row: (Vec::new(), false, GunFit::BARE) }
    }

    /// The menu is open (the game should pause).
    pub fn menu_open(&self) -> bool {
        self.menu.is_some()
    }

    /// Where the newest alert still showing (`ALERT_LIFE`) that has a place
    /// is: the squad mode's Space goes there while the camera follows the
    /// squad (controls review, 2026-11-21: Space is the squad's there, so a
    /// raid's or a landing ore's alert could only be reached by panning).
    pub fn newest_place(&self) -> Option<(f32, f32)> {
        self.alerts.iter().rev().filter(|a| a.when.elapsed() < ALERT_LIFE).find_map(|a| a.at)
    }

    /// A targeted command is waiting for its point: its id and target, and
    /// a line's first end once pressed.
    pub fn targeting(&self) -> Option<Aim> {
        self.targeting.map(|t| (t.id, t.target, t.from))
    }

    pub fn cancel_targeting(&mut self) {
        self.targeting = None;
    }

    /// Whether a screen point is on the UI (clicks there are the UI's).
    /// Whether a screen point is on the UI, the minimap included (the
    /// world behind them is not under the cursor).
    pub fn over(&self, p: (f32, f32)) -> bool {
        self.menu.is_some() || self.end || self.hits.iter().any(|(r, _)| r.contains(p)) || self.on_minimap(p).is_some()
    }

    /// Whether a click at screen point `p` works on something there (a
    /// button, a tile, a menu row), not only the HUD's glass: the cursor
    /// lights over it (`larder-world`'s `cursor.rs`).
    pub fn clickable(&self, p: (f32, f32)) -> bool {
        self.hit(p).is_some_and(|h| h != Hit::Panel)
    }

    /// The world point a screen point on the minimap stands for.
    pub fn on_minimap(&self, p: (f32, f32)) -> Option<(f32, f32)> {
        let mm = self.minimap?;
        let w = self.world;
        super::view::minimap_world(mm, (w.0 as usize, w.1 as usize), p)
    }

    /// Where a placement at screen point `p` goes in the world: through the
    /// minimap when it is on it (the world behind the minimap is hidden),
    /// none over a panel or off the world.
    pub fn aim(&self, p: (f32, f32), cam: &Camera) -> Option<(f32, f32)> {
        if let Some(w) = self.on_minimap(p) {
            return Some(w);
        }
        if self.hit(p).is_some() {
            return None;
        }
        cam.world_at(p)
    }

    fn hit(&self, p: (f32, f32)) -> Option<Hit> {
        self.hits.iter().rev().find(|(r, _)| r.contains(p)).map(|(_, h)| *h)
    }

    /// An alert: `text`, at a world point if it has one (pinged on the
    /// minimap; Space goes there).
    pub fn alert(&mut self, text: &str, at: Option<(f32, f32)>, colour: Rgba) {
        // The same words again refresh the old one rather than stack.
        self.alerts.retain(|a| a.text != text);
        self.alerts.push(Alert { text: text.into(), at, colour, when: Instant::now() });
        if self.alerts.len() > ALERTS_MAX {
            self.alerts.remove(0);
        }
    }

    pub fn moved(&mut self, p: Option<(f32, f32)>) {
        self.hover = p;
        if let Some(p) = p {
            self.card.moved(p, Instant::now());
        }
    }

    fn activate(&mut self, index: usize, out: &mut Vec<HudEvent>) {
        let Some(c) = self.commands.get(index).cloned() else { return };
        if !c.ready {
            if let Some(w) = &c.why_not {
                self.alert(w, None, WARN);
            }
            out.push(HudEvent::NotReady { id: c.id });
            return;
        }
        match c.target {
            Target::Instant => out.push(HudEvent::Command { id: c.id }),
            t => self.targeting = Some(Targeting { index, id: c.id, target: t, from: None }),
        }
    }

    /// A mouse button. Returns whether the UI took it (then the controls
    /// should not see it) and what it gave the game.
    pub fn button(&mut self, b: Button, pressed: bool, cam: &Camera, shift: bool) -> (bool, Vec<HudEvent>) {
        let k = match b {
            Button::Left => 0,
            Button::Right => 1,
            Button::Middle => 2,
        };
        if pressed && self.card.key.is_some() {
            self.card.shut = true;
        }
        self.held[k] = pressed;
        if pressed {
            let (took, out) = self.button_inner(b, true, cam, shift);
            self.took[k] = took;
            return (took, out);
        }
        let took = std::mem::take(&mut self.took[k]);
        // A line being laid ends where the button comes up, over a panel
        // or not (on a panel it ends at the world edge under the cursor).
        if let (Button::Left, Some(t)) = (b, self.targeting) {
            if let (Target::Line { .. }, Some(from)) = (t.target, t.from) {
                let mut out = Vec::new();
                let p = self.hover.unwrap_or((0.0, 0.0));
                let to = self.aim(p, cam).unwrap_or_else(|| {
                    let w = cam.to_world(p);
                    (w.0.clamp(0.0, cam.world.0 - 1.0), w.1.clamp(0.0, cam.world.1 - 1.0))
                });
                if self.refused(t.id) {
                    self.targeting = Some(Targeting { from: None, ..t });
                } else {
                    out.push(HudEvent::Line { id: t.id, from, to });
                    self.targeting = if shift { Some(Targeting { from: None, ..t }) } else { None };
                }
                return (true, out);
            }
        }
        if !took {
            self.pressed = None;
            return (false, Vec::new());
        }
        let (_, out) = self.button_inner(b, false, cam, shift);
        (true, out)
    }

    /// A placement where the ghost is red, or of a command no longer
    /// ready: refused, with the reason as an alert; targeting stays.
    fn refused(&mut self, id: u32) -> bool {
        let cmd = self.commands.iter().find(|c| c.id == id).cloned();
        let why = match cmd {
            Some(c) if !c.ready => Some(c.why_not.unwrap_or_else(|| "Not ready".into())),
            _ if !self.ghost_ok => Some(self.ghost_note.clone().unwrap_or_else(|| "It cannot go there".into())),
            _ => None,
        };
        match why {
            Some(w) => {
                self.alert(&w, None, WARN);
                true
            }
            None => false,
        }
    }

    fn button_inner(&mut self, b: Button, pressed: bool, cam: &Camera, shift: bool) -> (bool, Vec<HudEvent>) {
        let mut out = Vec::new();
        let Some(p) = self.hover else { return (false, out) };
        let hit = self.hit(p);
        if self.menu.is_some() || self.end {
            if b == Button::Left && !pressed {
                match hit {
                    Some(Hit::Menu(MenuItem::Resume)) | Some(Hit::Menu(MenuItem::Opened)) => {
                        self.menu = None;
                        out.push(HudEvent::Menu(MenuItem::Resume));
                    }
                    Some(Hit::Menu(m)) => {
                        self.menu = None;
                        out.push(HudEvent::Menu(m));
                    }
                    Some(Hit::Help) => {
                        self.menu = Some(MenuPage::Help);
                        self.help_direct = false;
                    }
                    Some(Hit::Back) if self.help_direct => {
                        self.menu = None;
                        out.push(HudEvent::Menu(MenuItem::Resume));
                    }
                    Some(Hit::Back) => self.menu = Some(MenuPage::Main),
                    Some(Hit::ExpPage) => self.menu = Some(MenuPage::Experiments),
                    Some(Hit::Exp(row, choice)) => out.push(HudEvent::Experiment { row, choice }),
                    _ => {}
                }
            }
            return (true, out);
        }
        if let Some(t) = self.targeting {
            if hit.is_some() && hit != Some(Hit::Panel) && b == Button::Left && pressed {
                // A click on another button while targeting: that button.
                self.targeting = None;
            } else if hit.is_none() {
                // (Over the minimap too: a placement there lands on the
                // world point the minimap shows.)
                match (b, pressed) {
                    (Button::Right, true) => {
                        self.targeting = None;
                        return (true, out);
                    }
                    (Button::Left, true) => {
                        if let Some(w) = self.aim(p, cam) {
                            match t.target {
                                Target::Line { .. } => self.targeting = Some(Targeting { from: Some(w), ..t }),
                                _ if self.refused(t.id) => {}
                                _ => {
                                    out.push(HudEvent::Point { id: t.id, at: w });
                                    if !shift {
                                        self.targeting = None;
                                    }
                                }
                            }
                        }
                        return (true, out);
                    }
                    (Button::Left, false) => return (true, out),
                    _ => return (b != Button::Middle, out),
                }
            }
        }
        let Some(h) = hit else {
            if !pressed {
                self.pressed = None;
            }
            return (false, out);
        };
        if b != Button::Left {
            return (true, out);
        }
        if pressed {
            self.pressed = Some(h);
            return (true, out);
        }
        let was = self.pressed.take();
        if was != Some(h) {
            return (true, out);
        }
        match h {
            Hit::Command(i) => self.activate(i, &mut out),
            Hit::Card(id) => out.push(if shift { HudEvent::Deselect(id) } else { HudEvent::SelectOnly(id) }),
            Hit::Group(g) => out.push(HudEvent::Group(g)),
            Hit::Idle => out.push(HudEvent::Idle),
            Hit::Cycle => out.extend(self.cycle.map(|id| HudEvent::Command { id })),
            Hit::Squad(r) => out.push(HudEvent::Squad(r)),
            Hit::SquadInto(r) => out.push(HudEvent::SquadInto(r)),
            Hit::SquadSplit => out.push(HudEvent::SquadSplit),
            Hit::SquadCombine => out.push(HudEvent::SquadCombine),
            Hit::SquadLink(r) => out.push(HudEvent::SquadLink(r)),
            Hit::SquadFree => out.push(HudEvent::SquadFree),
            Hit::MenuButton => {
                self.menu = Some(MenuPage::Main);
                out.push(HudEvent::Menu(MenuItem::Opened));
            }
            _ => {}
        }
        (true, out)
    }

    /// A key press. Returns whether the UI took it.
    pub fn key(&mut self, code: KeyCode, pressed: bool, ctrl: bool, cam: &mut Camera) -> (bool, Vec<HudEvent>) {
        let mut out = Vec::new();
        if !pressed {
            return (false, out);
        }
        // The end screen takes every key; Enter presses its first button
        // (Direction 11-24, the time between planets: the card's one way
        // on was a click on it). Space stays dead there: it is the
        // alerts' key in play, and held through the card it would skip it.
        if self.end {
            if matches!(code, KeyCode::Enter | KeyCode::NumpadEnter) {
                out.push(HudEvent::Menu(if self.end_carry { MenuItem::Continue } else { MenuItem::Restart }));
            }
            return (true, out);
        }
        // F1 or ? (the Slash key) is the key card from play or the menu
        // (cloud plan 2026-11-14, C2); again, or Esc, puts it away.
        if matches!(code, KeyCode::F1 | KeyCode::Slash) && !ctrl {
            self.targeting = None;
            match self.menu {
                Some(MenuPage::Help) => {
                    self.menu = if self.help_direct { None } else { Some(MenuPage::Main) };
                    if self.help_direct {
                        out.push(HudEvent::Menu(MenuItem::Resume));
                    }
                }
                Some(MenuPage::Main | MenuPage::Experiments) => {
                    self.menu = Some(MenuPage::Help);
                    self.help_direct = false;
                }
                None => {
                    self.menu = Some(MenuPage::Help);
                    self.help_direct = true;
                    out.push(HudEvent::Menu(MenuItem::Opened));
                }
            }
            return (true, out);
        }
        if code == KeyCode::Escape {
            if self.targeting.take().is_some() {
                return (true, out);
            }
            match self.menu {
                Some(MenuPage::Help) if self.help_direct => {
                    self.menu = None;
                    out.push(HudEvent::Menu(MenuItem::Resume));
                }
                Some(MenuPage::Help | MenuPage::Experiments) => self.menu = Some(MenuPage::Main),
                Some(MenuPage::Main) => {
                    self.menu = None;
                    out.push(HudEvent::Menu(MenuItem::Resume));
                }
                None => {
                    self.menu = Some(MenuPage::Main);
                    out.push(HudEvent::Menu(MenuItem::Opened));
                }
            }
            return (true, out);
        }
        if self.menu.is_some() {
            return (true, out);
        }
        // Space is always the alerts' key (it once fell through to the
        // sandbox's pause when no alert had a place, so what it did
        // depended on state the player cannot see; P pauses).
        if code == KeyCode::Space {
            if let Some(at) = self.alerts.iter().rev().find_map(|a| a.at) {
                cam.centre_on(at);
            }
            return (true, out);
        }
        if ctrl {
            return (false, out);
        }
        // A button's other keys first (the fire button's Z, X and C), then
        // the buttons' own.
        if let Some((i, id)) = self.commands.iter().enumerate().find_map(|(i, c)| c.also.iter().find(|a| a.0 == code).map(|a| (i, a.1))) {
            if self.commands[i].ready {
                out.push(HudEvent::Command { id });
            } else {
                self.activate(i, &mut out);
            }
            return (true, out);
        }
        if let Some(i) = self.commands.iter().position(|c| c.key == code) {
            self.activate(i, &mut out);
            return (true, out);
        }
        (false, out)
    }

    /// Draws the UI for `m` (and the targeting ghost into `world`, the
    /// world marks). `minimap`: whether the view shows one (then the UI
    /// frames it, bottom right; the game passes `minimap_box` on to the
    /// view).
    pub fn draw(&mut self, m: &Model, controls: &Controls, cam: &Camera, minimap: bool, c: &mut Canvas, world: &mut Marks) {
        let (sw, sh) = (c.screen.0 as f32, c.screen.1 as f32);
        let s = c.ui.px(c.screen);
        // The bottom bar's unit (`BAR`); the top bar, alerts, tooltips and
        // menus keep `s`.
        let b = s * BAR;
        self.scale = s;
        self.world = cam.world;
        self.hits.clear();
        self.commands = m.commands.clone();
        self.ghost_ok = m.ghost_ok;
        self.ghost_note = m.ghost_note.clone();
        if let Some(t) = self.targeting {
            // The command may have gone (selection changed); keep it while
            // it is still on the card. (It once also had to stay ready, so
            // a wall drag ended silently when energy dipped mid-drag; now
            // the placement is refused with the reason instead.)
            let still = m.commands.iter().position(|k| k.id == t.id);
            self.targeting = still.map(|index| Targeting { index, ..t });
        }
        let now = Instant::now();
        self.alerts.retain(|a| now.duration_since(a.when) < ALERT_LIFE);
        let pad = CARD_PAD * b;

        // The minimap's frame (bottom right), the command card left of it,
        // the selection panel left of that, all one height, whatever the
        // zoom (owner, 11-14 20:19Z).
        let cell = CELL * b;
        let gap = CELL_GAP * b;
        let card_of = |w: f32| CARD_COLS as f32 * w + (CARD_COLS - 1) as f32 * gap + 2.0 * pad;
        let (_, inset_bottom) = Hud::insets(c.screen, c.ui);
        let panel_h = inset_bottom - 8.0 * b;
        let base = sh - 8.0 * b;
        let top_of = base - panel_h;
        self.bottom = top_of;
        let radius = 12.0 * b;
        let mut right = sw - 8.0 * b;
        self.minimap_box = None;
        self.minimap = None;
        if minimap {
            let mm_h = panel_h;
            let ih = mm_h - 2.0 * pad;
            // (min then max, not `clamp`, which panics in a narrow window.)
            let iw = (ih * cam.world.0 / cam.world.1).min((MINIMAP_W * b).min(sw * 0.2)).max(ih * 0.6);
            let f = Rect::new(right - iw - 2.0 * pad, base - mm_h, iw + 2.0 * pad, mm_h);
            let bx = ((f.x + pad).round() as u32, (f.y + pad).round() as u32, iw.round() as u32, ih.round() as u32);
            let mm = super::view::minimap_in((cam.world.0 as usize, cam.world.1 as usize), bx);
            self.minimap_box = Some(bx);
            self.minimap = Some(mm);
            let hole = Rect::new(mm.0 as f32, mm.1 as f32, mm.2 as f32, mm.3 as f32);
            c.shadow(f, radius, 16.0 * b, 4.0 * b, SHADOW);
            // A panel round the minimap, open over it (the minimap is the
            // view's, drawn before the canvas).
            c.panel_with_hole(f, radius, PANEL_TOP, PANEL_BOTTOM, 1.0, EDGE, hole);
            c.outline(Rect::new(hole.x - 1.0, hole.y - 1.0, hole.w + 2.0, hole.h + 2.0), 2.0, 1.0, [255, 255, 255, 40]);
            // Clicks on the frame are the UI's; inside, the controls'.
            for r in [Rect::new(f.x, f.y, f.w, hole.y - f.y), Rect::new(f.x, hole.bottom(), f.w, f.bottom() - hole.bottom()), Rect::new(f.x, f.y, hole.x - f.x, f.h), Rect::new(hole.right(), f.y, f.right() - hole.right(), f.h)] {
                self.hits.push((r, Hit::Panel));
            }
            right = f.x - 8.0 * b;
        }
        // The gun cards' panel at the bar's far left (owner, 08:46Z: "gun/card
        // menu should go on the far left to fill the rest of the screen"),
        // the selection panel filling the row between it and the command
        // card (one card row tall, owner 08:33Z), all in the one row (owner,
        // 11-15 07:51Z: "everything was supposed to be on one row, why is
        // the gun in a seperate row on top").
        let (labelled, fit) = Hud::bar_fit(m, right - 8.0 * b, b);
        let card_w = if labelled { card_of(CELL_W * b) } else { card_of(cell) };
        let card = Rect::new(right - card_w, top_of, card_w, panel_h);
        let gun = m.loadout.as_ref().map(|l| Rect::new(8.0 * b, top_of, loadout_w(l, fit, b), panel_h));
        let left = gun.map_or(8.0 * b, |g| g.right() + 8.0 * b);
        let sel = Rect::new(left, top_of, (card.x - 8.0 * b - left).max(0.0), panel_h);
        // With nothing selected the selection panel collapses to a strip
        // along the bottom (the art director, 11-14 02:34Z: a full-height
        // panel that only said "Nothing selected").
        let empty = m.info.is_none() && m.selection.is_empty() && m.squads.is_none();
        // The loadout strip (about 312 units wide) takes the rest of the row.
        let sel_drawn = if empty { Rect::new(sel.x, base - COLLAPSED * b, (sel.w - 320.0 * b).clamp(150.0 * b, 330.0 * b).min(sel.w), COLLAPSED * b) } else { sel };

        #[cfg(test)]
        {
            let mm = self.minimap_box.map(|(x, y, w, h)| Rect::new(x as f32 - pad, y as f32 - pad, w as f32 + 2.0 * pad, h as f32 + 2.0 * pad));
            self.row = (gun.into_iter().chain([sel_drawn, card]).chain(mm).collect(), labelled, fit);
        }
        self.top_bar(m, c, s);
        self.selection_panel(m, controls, sel_drawn, c, b);
        // Before the command card, so the card's tooltips cover it; its
        // own card tooltip after.
        self.cycle = m.loadout.as_ref().and_then(|l| l.cycle.map(|c| c.0));
        let tip = m.loadout.as_ref().and_then(|l| self.loadout_draw(l, gun, sel, empty, card, fit, c, b));
        self.command_card(m, card, c, b);
        match (&m.loadout, tip) {
            (Some(l), Some((k, r))) if k < l.cards.len() => self.card_tip(&l.cards[k], r, c, s),
            (Some(l), Some((k, r))) => self.loadout_tip(l, r, k == usize::MAX - 1, c, s),
            _ => {}
        }
        // Not under the end card (Look D, 11-20): its lines say the same
        // ("Goal met", "Your last soldier fell"), and a stale "the mould is
        // at the larder" beside a won planet's card read as still in play.
        if m.end.is_none() {
            self.alerts_draw(c, world, s);
        }
        // Nor under the pause card (Look D, 11-24): the planet's
        // introduction stood half over it at 4K, a second panel of words
        // behind the one being read; it shows again on Resume.
        let open = self.menu.is_some();
        if let (Some(i), None, false) = (&m.intro, &m.end, open) {
            self.intro_card(i, c, s);
        }
        if let (Some((words, step, a)), None, false) = (&m.coach, &m.end, open) {
            self.coach_panel(words, step, *a, c, s);
        }
        if let (Some((text, at, gap, a)), None) = (&m.hint, &m.end) {
            let p = cam.to_screen(*at);
            let r = m.hint_ring * cam.zoom;
            // A reach wider than the screen reads as a stray arc across
            // it (the strikers' band at play zoom, 11-24 film): only a
            // ring that fits is drawn.
            if r > 4.0 * s && r < 0.42 * (c.screen.0.min(c.screen.1) as f32) {
                // Thin, and gold as the panel's edge: a ring at the thing's
                // own size, not a pulse (it never grows or flashes).
                let w = 2.0 * s;
                // On a dark band a pixel wider (the colour check, 11-24:
                // under protanopia and deuteranopia the gold ring vanished
                // into yellow slime; the dark edge holds by brightness).
                c.ring(Rect::new(p.0 - r, p.1 - r, 2.0 * r, 2.0 * r), r, w + 2.0 * s, 1.0, alpha([14, 13, 10, 255], 0.55 * a));
                c.ring(Rect::new(p.0 - r, p.1 - r, 2.0 * r, 2.0 * r), r, w, 1.0, alpha([255, 214, 90, 255], 0.75 * a));
            }
            // A reach ring can be wider than the screen: the panel hangs
            // near the thing, never more than 90 px (at 100%) below it.
            self.world_hint(text, (p.0, p.1 + (gap * cam.zoom).min(90.0 * s)), *a, c, s);
        }
        self.ghost(m, cam, c, world, s);
        let key = m.hover.as_ref().filter(|_| self.hover.is_some() && self.targeting.is_none()).map(|h| h.title.as_str());
        let quiet = m.aiming || self.held.iter().any(|&h| h);
        let a = self.card.alpha(key, Instant::now(), quiet);
        if let (Some(h), Some(p), true) = (&m.hover, self.hover, a > 0.0) {
            self.hover_card(h, p, a, c, s);
        }
        self.end = m.end.is_some();
        self.end_carry = m.carry_on.is_some();
        if let Some(e) = &m.end {
            self.end_screen(e, m.main_menu, m.carry_on.as_deref(), c, s);
        } else if let Some(page) = self.menu {
            self.menu_draw(page, m, c, s);
        }
    }

    /// The selection panel's widths, pixels: the least that shows all it
    /// holds (every squad button, three tiles), then with the squad
    /// header's keys, then everything at full size. The allocator of 11-15
    /// gave the row out by these; since the steady-HUD pass (11-20) only
    /// the printout reads them, against `SEL_FLOOR`.
    #[cfg(test)]
    fn sel_needs(&self, m: &Model, c: &Canvas, s: f32) -> [f32; 3] {
        // The panel's padding, and a unit spare: a need met to the last
        // float dropped Guards at 200% (`squad_tiles` measures again).
        let pad = 2.0 * 12.0 * s + 1.0 * s;
        if let Some(p) = &m.squads {
            let taken: Vec<String> = self.commands.iter().filter_map(|c| key_name(c.key)).collect();
            let labels = header_labels(p, &taken);
            let head = squads_head(p.rows.len());
            let w = |k: usize| header_w(c, &labels, &head, HEADER_STEPS[k], s);
            // Up to three tiles at their least before the "+N" tile.
            let n = p.rows.len().clamp(1, 3) as f32;
            let tiles = n * 86.0 * s + (n - 1.0) * 7.0 * s;
            return [w(3).max(tiles) + pad, w(2).max(tiles) + pad, w(0).max(tiles) + pad];
        }
        let one = if m.info.is_some() {
            420.0 * s
        } else if !m.selection.is_empty() {
            // "12 selected", its health line, and up to six cards.
            let n = m.selection.len();
            let line = c.atlas.measure(Weight::SemiBold, 16.0 * s, &format!("{n} selected")) + 12.0 * s + c.atlas.measure(Weight::Medium, 13.5 * s, "85% health on average, 2 hurt");
            let k = n.min(6) as f32;
            line.max(k * 54.0 * s + (k - 1.0) * 7.0 * s) + pad
        } else {
            // The collapsed strip.
            330.0 * s
        };
        [one; 3]
    }

    /// The bottom row's allocator. Since the steady-HUD pass (owner,
    /// 2026-11-20 22:08Z: "it changes as tiles resize. if we have many
    /// squads then the actions shrink or once we have guns with lots of
    /// cards the actions shrink. Things should stay more consistent. squads
    /// or gun cards can shrink themselves within their own panel") it
    /// reads only the room and the gun's widest slot count, never what is
    /// selected:
    ///
    /// - The command card is labelled wherever the row holds it beside a
    ///   rifle's bare strip (`GUN_FLOOR`) and the selection's floor
    ///   (`SEL_FLOOR`), so its buttons keep one size and place for the
    ///   window and UI size.
    /// - The gun strip takes the room left over the selection's floor, at
    ///   the richest fit that goes in (the name sheds first, then Swap),
    ///   and below the bare strip its cards draw smaller, down to 60%. Its width follows the widest gun in
    ///   play (`Loadout::widest`), so commanding another squad does not
    ///   move it.
    /// - The selection takes the rest: squad tiles shrink inside it.
    ///
    /// The allocator before (the three-reviewer review, 11-15) gave the
    /// row out by what the selection held (`sel_needs`): labels where the
    /// selection kept its need, then the squad keys, empty slots, Swap and
    /// the gun's name. It used the space better, but at 125% on the game's
    /// 1600 x 972 window the card went square with four squads and a
    /// 6-slot gun and labelled again with nine squads and two slots, and
    /// the gun strip changed width with them. Returns whether the card is labelled, and the strip's fit.
    fn bar_fit(m: &Model, room: f32, b: f32) -> (bool, GunFit) {
        let gap = CELL_GAP * b;
        let card_of = |w: f32| CARD_COLS as f32 * w + (CARD_COLS - 1) as f32 * gap + 2.0 * CARD_PAD * b;
        // The selection's floor and the two gaps either side of it.
        let sel = (8.0 + SEL_FLOOR + 8.0) * b;
        let labelled = room - card_of(CELL_W * b) - sel >= GUN_FLOOR * b;
        let Some(l) = &m.loadout else { return (labelled, GunFit::FULL) };
        let budget = room - card_of(if labelled { CELL_W } else { CELL } * b) - sel;
        let fits = [GunFit::FULL, GunFit { name: false, ..GunFit::FULL }, GunFit::BARE];
        if let Some(f) = fits.into_iter().find(|&f| loadout_w(l, f, b) <= budget) {
            return (labelled, f);
        }
        let w = loadout_w(l, GunFit::BARE, b);
        (labelled, GunFit { scale: (budget / w).clamp(0.6, 1.0), ..GunFit::BARE })
    }

    fn glass(&mut self, r: Rect, radius: f32, c: &mut Canvas, s: f32) {
        c.shadow(r, radius, 16.0 * s, 4.0 * s, SHADOW);
        c.panel(r, radius, PANEL_TOP, PANEL_BOTTOM, 1.0, EDGE);
        self.hits.push((r, Hit::Panel));
    }

    fn top_bar(&mut self, m: &Model, c: &mut Canvas, s: f32) {
        let sw = c.screen.0 as f32;
        let h = 38.0 * s;
        let y = 8.0 * s;
        self.top = y + h;
        self.below = self.top;
        let px = 15.0 * s;
        let small = 12.0 * s;
        let base = y + h / 2.0 + 5.0 * s;
        // Resources: a pill of icon, value and rate each.
        let mut widths = Vec::new();
        for r in &m.resources {
            // Each number as wide as its widest digits (`steady_w`): the
            // food ticking from 1,111 to 1,088 moved every pill after it
            // and the objective a pixel or two a frame.
            let w = 18.0 * s + 6.0 * s + steady_w(c, Weight::SemiBold, px, &r.value) + r.rate.as_ref().map(|t| 6.0 * s + steady_w(c, Weight::Medium, small, t)).unwrap_or(0.0);
            widths.push(w);
        }
        let inner: f32 = widths.iter().sum::<f32>() + (widths.len().saturating_sub(1)) as f32 * 20.0 * s;
        if !m.resources.is_empty() {
            let pill = Rect::new(8.0 * s, y, inner + 28.0 * s, h);
            self.glass(pill, h / 2.0, c, s);
            let mut x = pill.x + 14.0 * s;
            for (r, w) in m.resources.iter().zip(&widths) {
                c.icon(r.icon, (x, y + (h - 18.0 * s) / 2.0), 18.0 * s, r.colour);
                let tx = x + 24.0 * s;
                c.text(Weight::SemiBold, px, (tx, base), Align::Left, &r.value, TEXT);
                let vw = steady_w(c, Weight::SemiBold, px, &r.value);
                if let Some(rate) = &r.rate {
                    let col = if rate.starts_with('-') { BAD } else { GOOD };
                    c.text(Weight::Medium, small, (tx + vw + 6.0 * s, base), Align::Left, rate, col);
                }
                // The label shows on hover, as a tooltip under the pill.
                let area = Rect::new(x - 4.0 * s, y, w + 8.0 * s, h);
                if self.hover.is_some_and(|p| area.contains(p)) {
                    self.tip_small(&r.label, (x, y + h + 6.0 * s), c, s);
                }
                x += w + 20.0 * s;
            }
        }
        // The clock, the speed and the menu button, on the right.
        let menu = Rect::new(sw - 8.0 * s - h, y, h, h);
        let hot = self.hover.is_some_and(|p| menu.contains(p));
        c.shadow(menu, h / 2.0, 16.0 * s, 4.0 * s, SHADOW);
        let (t, b) = if hot { (BUTTON_HOVER_TOP, BUTTON_HOVER_BOTTOM) } else { (PANEL_TOP, PANEL_BOTTOM) };
        c.panel(menu, h / 2.0, t, b, 1.0, EDGE);
        c.icon(Icon::Menu, (menu.x + (h - 18.0 * s) / 2.0, y + (h - 18.0 * s) / 2.0), 18.0 * s, TEXT);
        self.hits.push((menu, Hit::MenuButton));
        let cw = steady_w(c, Weight::SemiBold, px, &m.clock);
        let chip = m.speed.as_ref().map(|t| c.atlas.measure(Weight::SemiBold, small, &t.to_uppercase()) + 16.0 * s).unwrap_or(0.0);
        let w = 14.0 * s + 18.0 * s + 6.0 * s + cw + if chip > 0.0 { 10.0 * s + chip } else { 0.0 } + 14.0 * s;
        let pill = Rect::new(menu.x - 8.0 * s - w, y, w, h);
        self.glass(pill, h / 2.0, c, s);
        let mut x = pill.x + 14.0 * s;
        c.icon(Icon::Clock, (x, y + (h - 18.0 * s) / 2.0), 18.0 * s, MUTED);
        x += 24.0 * s;
        c.text(Weight::SemiBold, px, (x, base), Align::Left, &m.clock, TEXT);
        x += cw + 10.0 * s;
        if let Some(sp) = &m.speed {
            let r = Rect::new(x, y + 9.0 * s, chip, h - 18.0 * s);
            c.rrect(r, r.h / 2.0, alpha(WARN, 0.18));
            c.text(Weight::SemiBold, small, (r.x + r.w / 2.0, base - 1.0 * s), Align::Centre, &sp.to_uppercase(), WARN);
        }
        // The experiments that are on (the plan of 2026-11-17: "the HUD
        // shows a small mark when anything is off its default"; the owner
        // that day could not tell what was on or off). Under the clock,
        // at the screen's edge and off the fight: in the clock's own pill
        // it met the objective at 200% while paused. Hovering it names
        // them.
        let mut mark = sw;
        if !m.trying.is_empty() {
            let label = if m.trying.len() == 1 { "EXPERIMENT ON".to_string() } else { format!("{} EXPERIMENTS ON", m.trying.len()) };
            let w = c.atlas.measure(Weight::SemiBold, small, &label) + 22.0 * s;
            let r = Rect::new(menu.right() - w, y + h + 6.0 * s, w, 22.0 * s);
            mark = r.x;
            c.shadow(r, r.h / 2.0, 10.0 * s, 2.0 * s, SHADOW);
            c.panel(r, r.h / 2.0, PANEL_TOP, PANEL_BOTTOM, 1.0, alpha(WARN, 0.55));
            c.text(Weight::SemiBold, small, (r.x + r.w / 2.0, r.y + 15.5 * s), Align::Centre, &label, WARN);
            self.hits.push((r, Hit::Panel));
            if self.hover.is_some_and(|p| r.contains(p)) {
                // Under the mark, ending at its right edge (`tip_small`'s
                // own flip put it a mark's width to the left).
                let tip = format!("Not the default game: {}. Esc, Experiments to change", m.trying.join(", "));
                let tw = c.atlas.measure(Weight::Medium, 13.5 * s, &tip) + 18.0 * s;
                self.tip_small(&tip, ((r.right() - tw).max(8.0 * s), r.bottom() + 6.0 * s), c, s);
            }
        }
        // The objective, centred between the resources' pill and the
        // clock's, smaller if it must be; and if it still does not fit,
        // on a row of its own under the bar, left of the experiments'
        // mark, cut to the room there. Centred on the screen alone, at
        // 200% on 1080 lines it covered the resources' fourth number
        // (11-18).
        if let Some(obj) = &m.objective {
            let left = if m.resources.is_empty() { 8.0 * s } else { 8.0 * s + inner + 28.0 * s } + 10.0 * s;
            let right = pill.x - 10.0 * s;
            let mut fs = 14.0 * s;
            let mut ow = c.atlas.measure(Weight::Medium, fs, obj) + 40.0 * s;
            if ow > right - left {
                fs = (fs * (right - left - 40.0 * s) / (ow - 40.0 * s)).max(12.0 * s);
                ow = c.atlas.measure(Weight::Medium, fs, obj) + 40.0 * s;
            }
            let r = if ow <= right - left {
                Rect::new(((sw - ow) / 2.0).clamp(left, right - ow), y + 3.0 * s, ow, h - 6.0 * s)
            } else {
                fs = 14.0 * s;
                let room = (mark.min(sw - 8.0 * s) - 10.0 * s - 8.0 * s).max(40.0 * s);
                let ow = (c.atlas.measure(Weight::Medium, fs, obj) + 40.0 * s).min(room);
                let r = Rect::new(((sw - ow) / 2.0).clamp(8.0 * s, 8.0 * s + room - ow), y + h + 6.0 * s, ow, h - 6.0 * s);
                self.below = r.bottom();
                r
            };
            self.glass(r, r.h / 2.0, c, s);
            c.icon(Icon::Target, (r.x + 12.0 * s, r.y + (r.h - 14.0 * s) / 2.0), 14.0 * s, ACCENT);
            c.text_fit(Weight::Medium, fs, (r.x + 30.0 * s, r.y + r.h / 2.0 + fs * (4.0 / 14.0)), Align::Left, obj, r.w - 40.0 * s, TEXT);
            // The slime's trait as its number (`Model::tell`), a small
            // amber pill hung under the goal's middle; its hover says
            // what the number is.
            // The planet's twist (`Model::twist`) beside it in ice blue, the
            // two centred together under the goal.
            let ts = 13.0 * s;
            let pills: Vec<(&(Icon, String, String), Rgba)> = m.tell.iter().map(|t| (t, WARN)).chain(m.twist.iter().map(|t| (t, TWIST))).collect();
            let widths: Vec<f32> = pills.iter().map(|((_, words, _), _)| c.atlas.measure(Weight::SemiBold, ts, words) + 42.0 * s).collect();
            let gap = 8.0 * s;
            let mut x = r.x + (r.w - widths.iter().sum::<f32>() - gap * widths.len().saturating_sub(1) as f32) / 2.0;
            for (((icon, words, tip), colour), tw) in pills.into_iter().zip(widths) {
                let t = Rect::new(x, r.bottom() + 5.0 * s, tw, 24.0 * s);
                x += tw + gap;
                self.glass(t, t.h / 2.0, c, s);
                c.icon(*icon, (t.x + 11.0 * s, t.y + 5.0 * s), 14.0 * s, colour);
                c.text(Weight::SemiBold, ts, (t.x + 31.0 * s, t.y + 16.5 * s), Align::Left, words, colour);
                self.below = self.below.max(t.bottom());
                if self.hover.is_some_and(|p| t.contains(p)) {
                    self.tip_small(tip, (t.x, t.bottom() + 6.0 * s), c, s);
                }
            }
        }
    }

    /// The planet's introduction (`Model::intro`): a card centred under
    /// the goal line, the planet small and muted, the slime's name large in
    /// the warning's amber, its line under it; the whole at `i.alpha`.
    /// Clicks go through it to the map (it is never in the way to play).
    fn intro_card(&mut self, i: &Intro, c: &mut Canvas, s: f32) {
        let a = i.alpha.clamp(0.0, 1.0);
        if a <= 0.0 {
            return;
        }
        let sw = c.screen.0 as f32;
        let w = (560.0 * s).min(sw - 32.0 * s);
        let tw = w - 48.0 * s;
        let lines = c.wrap(Weight::Medium, 17.0 * s, tw, &i.line);
        // The twist under the slime's line: its name, then its own line.
        let twist = i.twist.as_ref().map(|(name, line)| (name, c.wrap(Weight::Medium, 16.0 * s, tw, line)));
        let more = twist.as_ref().map_or(0.0, |(_, l)| 46.0 * s + l.len() as f32 * 22.0 * s);
        let h = 96.0 * s + lines.len() as f32 * 24.0 * s + more;
        let r = Rect::new((sw - w) / 2.0, self.below + 18.0 * s, w, h);
        c.shadow(r, 14.0 * s, 24.0 * s, 6.0 * s, alpha(SHADOW, a));
        c.panel(r, 14.0 * s, alpha(PANEL_TOP, a), alpha(PANEL_BOTTOM, a), 1.0, alpha(EDGE, a));
        c.rrect(Rect::new(r.x + 24.0 * s, r.y, w - 48.0 * s, 3.0 * s), 1.5 * s, alpha(WARN, 0.9 * a));
        let cx = r.x + w / 2.0;
        c.text_fit(Weight::Medium, 13.5 * s, (cx, r.y + 32.0 * s), Align::Centre, &i.kicker, tw, alpha(MUTED, a));
        c.text_fit(Weight::SemiBold, 26.0 * s, (cx, r.y + 66.0 * s), Align::Centre, &i.title, tw, alpha(WARN, a));
        for (k, l) in lines.iter().enumerate() {
            c.text(Weight::Medium, 17.0 * s, (cx, r.y + 96.0 * s + k as f32 * 24.0 * s), Align::Centre, l, alpha(TEXT, a));
        }
        if let Some((name, tl)) = &twist {
            let y = r.y + 96.0 * s + lines.len() as f32 * 24.0 * s;
            c.rrect(Rect::new(cx - 40.0 * s, y - 4.0 * s, 80.0 * s, 1.0 * s), 0.5 * s, alpha(EDGE, a));
            c.text_fit(Weight::SemiBold, 15.0 * s, (cx, y + 20.0 * s), Align::Centre, &name.to_uppercase(), tw, alpha(TWIST, a));
            for (k, l) in tl.iter().enumerate() {
                c.text(Weight::Medium, 16.0 * s, (cx, y + 44.0 * s + k as f32 * 22.0 * s), Align::Centre, l, alpha(TEXT, a));
            }
        }
    }

    /// The first minute's step (`Model::coach`): a panel centred over the
    /// command bar, the step's count in the accent at its left.
    fn coach_panel(&mut self, words: &str, step: &str, a: f32, c: &mut Canvas, s: f32) {
        let a = a.clamp(0.0, 1.0);
        if a <= 0.0 {
            return;
        }
        let (sw, sh) = (c.screen.0 as f32, c.screen.1 as f32);
        let foot = if self.bottom > 0.0 { self.bottom } else { sh };
        let px = 16.0 * s;
        let lead = c.atlas.measure(Weight::SemiBold, 13.0 * s, step) + 34.0 * s;
        let w = (c.atlas.measure(Weight::Medium, px, words) + lead + 22.0 * s).min(sw - 32.0 * s);
        let lines = c.wrap(Weight::Medium, px, w - lead - 22.0 * s, words);
        let h = 22.0 * s + lines.len() as f32 * 22.0 * s;
        let r = Rect::new((sw - w) / 2.0, foot - h - 16.0 * s, w, h);
        c.shadow(r, 10.0 * s, 18.0 * s, 4.0 * s, alpha(SHADOW, a));
        c.panel(r, 10.0 * s, alpha(PANEL_TOP, a), alpha(PANEL_BOTTOM, a), 1.0, alpha(ACCENT, 0.6 * a));
        c.text(Weight::SemiBold, 13.0 * s, (r.x + 16.0 * s, r.y + 26.0 * s), Align::Left, step, alpha(ACCENT, a));
        for (k, l) in lines.iter().enumerate() {
            c.text(Weight::Medium, px, (r.x + lead, r.y + 27.0 * s + k as f32 * 22.0 * s), Align::Left, l, alpha(TEXT, a));
        }
    }

    /// A hint pinned under a world point (`Model::hint`): a gold-edged
    /// panel hung from a short stem at `at` (screen pixels), kept on the
    /// screen. Quiet by design (owner: "some not too intrusive text"): the
    /// alerts' size and panel, no sound, gone once it has been followed.
    fn world_hint(&mut self, text: &str, at: (f32, f32), a: f32, c: &mut Canvas, s: f32) {
        const GOLD: Rgba = [255, 214, 90, 255];
        let size = 15.0 * s;
        let w = c.atlas.measure(Weight::Medium, size, text) + 30.0 * s;
        let h = 32.0 * s;
        let (sw, sh) = (c.screen.0 as f32, c.screen.1 as f32);
        // Long enough to hang clear of the drop's clock under its dial.
        let stem = 34.0 * s;
        let x = (at.0 - w / 2.0).clamp(8.0 * s, (sw - w - 8.0 * s).max(8.0 * s));
        let foot = if self.bottom > 0.0 { self.bottom } else { sh };
        let y = (at.1 + stem).clamp(self.below + 8.0 * s, (foot - h - 8.0 * s).max(self.below + 8.0 * s));
        let r = Rect::new(x, y, w, h);
        if (y - at.1) > 2.0 * s {
            c.rrect(Rect::new(at.0 - 1.0 * s, at.1, 2.0 * s, y - at.1), 1.0 * s, alpha(GOLD, 0.8 * a));
        }
        c.shadow(r, 8.0 * s, 12.0 * s, 3.0 * s, alpha(SHADOW, a));
        c.panel(r, 8.0 * s, alpha(PANEL_TOP, a), alpha(PANEL_BOTTOM, a), 1.0, alpha(GOLD, 0.7 * a));
        c.text(Weight::Medium, size, (r.x + 15.0 * s, r.y + 21.5 * s), Align::Left, text, alpha(TEXT, a));
    }

    fn tip_small(&mut self, text: &str, at: (f32, f32), c: &mut Canvas, s: f32) {
        let w = c.atlas.measure(Weight::Medium, 13.5 * s, text) + 18.0 * s;
        // To the cursor's left where it would run off the screen's right
        // edge (an out-of-reach throw's note was cut off there, 11-16).
        let x = if at.0 + w > c.screen.0 as f32 - 8.0 * s { (at.0 - 36.0 * s - w).max(8.0 * s) } else { at.0 };
        let r = Rect::new(x, at.1, w, 27.0 * s);
        c.shadow(r, 6.0 * s, 10.0 * s, 2.0 * s, SHADOW);
        c.panel(r, 6.0 * s, [36, 42, 50, 245], [28, 33, 40, 245], 1.0, EDGE);
        c.text(Weight::Medium, 13.5 * s, (r.x + 9.0 * s, r.y + 18.5 * s), Align::Left, text, TEXT);
    }

    fn selection_panel(&mut self, m: &Model, controls: &Controls, r: Rect, c: &mut Canvas, s: f32) {
        if r.w < 120.0 * s {
            return;
        }
        let radius = 12.0 * s;
        self.glass(r, radius, c, s);
        let pad = 12.0 * s;
        // Groups and the idle button ride on the panel's top edge.
        let chip_h = 24.0 * s;
        let mut x = r.x;
        let y = r.y - chip_h - 6.0 * s;
        if m.idle > 0 {
            // "11 idle", not a bare "11" (the polish review, 11-24).
            let label = format!("{} idle", m.idle);
            let w = 34.0 * s + c.atlas.measure(Weight::SemiBold, 13.0 * s, &label);
            let chip = Rect::new(x, y, w, chip_h);
            self.chip(chip, Hit::Idle, false, c, s);
            c.icon(Icon::Idle, (chip.x + 8.0 * s, chip.y + 4.0 * s), 16.0 * s, WARN);
            c.text(Weight::SemiBold, 13.0 * s, (chip.x + 28.0 * s, chip.y + 17.0 * s), Align::Left, &label, TEXT);
            if self.hover.is_some_and(|p| chip.contains(p)) {
                self.tip_small("Idle soldiers: select the next", (chip.x, chip.y - 30.0 * s), c, s);
            }
            x += w + 8.0 * s;
        }
        for (g, members) in controls.groups.iter().enumerate() {
            // The controls prune the dead from groups every frame, so the
            // members are the living.
            if members.is_empty() {
                continue;
            }
            let label = format!("{}", g + 1);
            let count = format!("{}", members.len());
            // The group's key, then a soldier and how many.
            let w = 9.0 * s + c.atlas.measure(Weight::SemiBold, 12.0 * s, &label) + 8.0 * s + 12.0 * s + 3.0 * s + c.atlas.measure(Weight::Medium, 11.5 * s, &count) + 10.0 * s;
            let chip = Rect::new(x, y, w, chip_h);
            let on = !controls.selected.is_empty() && controls.selected == *members;
            self.chip(chip, Hit::Group(g), on, c, s);
            let lw = c.text(Weight::SemiBold, 12.0 * s, (chip.x + 9.0 * s, chip.y + 16.5 * s), Align::Left, &label, if on { ACCENT } else { TEXT });
            let ix = chip.x + 17.0 * s + lw;
            c.icon(Icon::Soldier, (ix, chip.y + 6.0 * s), 12.0 * s, MUTED);
            c.text(Weight::Medium, 11.5 * s, (ix + 15.0 * s, chip.y + 16.5 * s), Align::Left, &count, MUTED);
            if self.hover.is_some_and(|p| chip.contains(p)) {
                self.tip_small(&format!("Group {}: {} soldiers. {} selects it, twice to look", g + 1, members.len(), g + 1), (chip.x, chip.y - 30.0 * s), c, s);
            }
            x += w + 6.0 * s;
        }

        let inner = r.inset(pad);
        if let Some(p) = &m.squads {
            self.squad_tiles(p, &m.squad_facts, inner, c, s);
            return;
        }
        if let Some(info) = &m.info {
            self.info_draw(info, inner, c, s);
            return;
        }
        if m.selection.is_empty() {
            // The collapsed strip: one line.
            let mid = r.y + r.h / 2.0;
            c.icon(Icon::Soldier, (r.x + 14.0 * s, mid - 9.0 * s), 18.0 * s, FAINT);
            let tw = c.text(Weight::SemiBold, 14.0 * s, (r.x + 40.0 * s, mid + 5.0 * s), Align::Left, "Nothing selected", MUTED);
            c.text_fit(Weight::Medium, 13.0 * s, (r.x + 48.0 * s + tw, mid + 5.0 * s), Align::Left, "click or drag to select", r.w - 62.0 * s - tw, FAINT);
            return;
        }
        // Every selected unit as a card.
        let n = m.selection.len();
        let mean = m.selection.iter().map(|q| q.hp).sum::<f32>() / n as f32;
        let hurt = m.selection.iter().filter(|q| q.hp < 0.6).count();
        let head = format!("{n} selected");
        let hw = c.text(Weight::SemiBold, 16.0 * s, (inner.x, inner.y + 15.0 * s), Align::Left, &head, TEXT);
        let sub = if hurt > 0 { format!("{:.0}% health on average, {hurt} hurt", mean * 100.0) } else { format!("{:.0}% health on average", mean * 100.0) };
        c.text(Weight::Medium, 13.5 * s, (inner.x + hw + 12.0 * s, inner.y + 15.0 * s), Align::Left, &sub, MUTED);
        let cw = 54.0 * s;
        let cgap = 7.0 * s;
        let top = inner.y + 26.0 * s;
        let cols = ((inner.w + cgap) / (cw + cgap)).floor().max(1.0) as usize;
        let rows = ((inner.bottom() - top + cgap) / (cw + cgap)).floor().max(1.0) as usize;
        let room = cols * rows;
        for (k, q) in m.selection.iter().enumerate() {
            if k >= room {
                break;
            }
            let (i, j) = (k % cols, k / cols);
            let cell = Rect::new(inner.x + i as f32 * (cw + cgap), top + j as f32 * (cw + cgap), cw, cw);
            if k == room - 1 && n > room {
                c.rrect(cell, 8.0 * s, [255, 255, 255, 14]);
                c.text(Weight::SemiBold, 14.0 * s, (cell.x + cw / 2.0, cell.y + cw / 2.0 + 5.0 * s), Align::Centre, &format!("+{}", n - room + 1), MUTED);
                break;
            }
            let hot = self.hover.is_some_and(|p| cell.contains(p));
            let (t, b) = if hot { (BUTTON_HOVER_TOP, BUTTON_HOVER_BOTTOM) } else { (BUTTON_TOP, BUTTON_BOTTOM) };
            c.panel(cell, 8.0 * s, t, b, 1.0, EDGE);
            let col = super::marks::team_colour(q.team);
            c.icon(q.icon, (cell.x + 6.0 * s, cell.y + 7.0 * s), 26.0 * s, col);
            c.bar(Rect::new(cell.x + 6.0 * s, cell.bottom() - 11.0 * s, cw - 12.0 * s, 5.0 * s), q.hp, health_colour(q.hp), TRACK);
            if let Some(b) = q.badge {
                // The weapon, big enough to read (it was 12 px).
                c.icon(b, (cell.right() - 21.0 * s, cell.y + 5.0 * s), 17.0 * s, TEXT);
            }
            self.hits.push((cell, Hit::Card(q.id)));
        }
        // What the selection carries: a count a badge (its weapons), along
        // the panel's foot.
        let mut kinds: Vec<(Icon, usize)> = Vec::new();
        for q in &m.selection {
            if let Some(b) = q.badge {
                match kinds.iter_mut().find(|(i, _)| *i == b) {
                    Some(k) => k.1 += 1,
                    None => kinds.push((b, 1)),
                }
            }
        }
        let used = rows.min(m.selection.len().div_ceil(cols));
        let foot = top + used as f32 * (cw + cgap) + 4.0 * s;
        if kinds.len() > 1 && foot + 22.0 * s <= inner.bottom() {
            let mut x = inner.x;
            for (icon, n) in kinds {
                c.icon(icon, (x, foot + 2.0 * s), 16.0 * s, MUTED);
                x += 20.0 * s + c.text(Weight::SemiBold, 12.5 * s, (x + 20.0 * s, foot + 15.0 * s), Align::Left, &format!("{n}"), TEXT) + 14.0 * s;
            }
        }
    }

    /// The squad mode's middle panel (`Model::squads`): a header line with
    /// the squad count and who you command, Split and Combine at its right
    /// end, and a tile a squad the size of a soldier's card (wider, for the
    /// count): its hotkey, soldiers, their health, and a + that brings it
    /// into the squad you command. The one you command is outlined.
    fn squad_tiles(&mut self, p: &SquadPanel, facts: &[SquadFact], inner: Rect, c: &mut Canvas, s: f32) {
        let n = p.rows.len();
        let head = squads_head(n);
        // The header's buttons, from the right end; smaller, then without
        // the header word, where they would run over it (the Let go button
        // made "1 squad" read "1 squa" under them at 150%, 11-15). Then
        // without their keys, and last a button that still does not fit is
        // left off rather than drawn over the gun strip. A key shows only
        // while it does this (`header_labels`).
        let free_off = p.free.then(|| "No squad commanded: every squad runs on its own. Click a tile or a squad, or press its number, to command it".to_string());
        let taken: Vec<String> = self.commands.iter().filter_map(|c| key_name(c.key)).collect();
        let labels = header_labels(p, &taken);
        let what = [
            (&free_off, Hit::SquadFree, "Let go: command no squad. Each keeps its orders, fires and guards on its own, and WASD pans the camera"),
            (&p.combine_off, Hit::SquadCombine, "Combine: the nearest squad that overlaps yours joins it"),
            (&p.split_off, Hit::SquadSplit, "Split: the front half of the squad you command becomes a new squad, and you command it"),
        ];
        let buttons: Vec<_> = labels.iter().zip(what).map(|((l, k), (off, hit, about))| (l.as_str(), k.clone(), off, hit, about)).collect();
        let label = header_text;
        // Measured as two digits' worth, so the header word does not come
        // and go as a squad splits from 9 to 10 (the steady-HUD pass, 11-20).
        let measure = squads_head(n.max(10));
        let (bpx, keys, with_head) = HEADER_STEPS
            .into_iter()
            .find(|&step| header_w(c, &labels, &measure, step, s) <= inner.w)
            .map(|(px, keys, h)| (px * s, keys, h))
            .unwrap_or((12.0 * s, false, false));
        let hw = if with_head { c.text(Weight::SemiBold, 16.0 * s, (inner.x, inner.y + 15.0 * s), Align::Left, &head, TEXT) } else { -12.0 * s };
        // Nothing left of here: the header word, or the panel's edge.
        let floor = inner.x + hw + 12.0 * s;
        // Where they still do not fit, Combine goes first. (Guards, then
        // Heal, went before it until both moved onto the card, Card 2.)
        let mut keep = [true; 3];
        let used: f32 = buttons.iter().map(|(l, k, ..)| c.atlas.measure(Weight::SemiBold, bpx, &label(l, k, keys)) + 26.0 * s).sum();
        if inner.right() - used < floor {
            keep[1] = false;
        }
        let bh = 22.0 * s;
        let mut bx = inner.right();
        let mut tip: Option<(String, Rect)> = None;
        for ((label_text, key, off, hit, about), _) in buttons.into_iter().zip(keep).filter(|(_, k)| *k) {
            let text = label(label_text, &key, keys);
            let w = c.atlas.measure(Weight::SemiBold, bpx, &text) + 20.0 * s;
            if bx - w < floor {
                continue;
            }
            bx -= w;
            let b = Rect::new(bx, inner.y, w, bh);
            bx -= 6.0 * s;
            let hot = self.hover.is_some_and(|h| b.contains(h));
            let (t, bt) = if hot && off.is_none() { (BUTTON_HOVER_TOP, BUTTON_HOVER_BOTTOM) } else { (BUTTON_TOP, BUTTON_BOTTOM) };
            c.panel(b, 6.0 * s, t, bt, 1.0, EDGE);
            self.hits.push((b, if off.is_none() { hit } else { Hit::Panel }));
            c.text(Weight::SemiBold, bpx, (b.x + b.w / 2.0, b.y + 16.0 * s), Align::Centre, &text, if off.is_some() { FAINT } else { TEXT });
            if hot {
                tip = Some((off.clone().unwrap_or_else(|| about.to_string()), b));
            }
        }
        // With none commanded, a line says so where it fits whole. (It said
        // "commanding Squad 1, 4 soldiers" too, which the outlined YOU tile
        // shows, cut to "commandin..." where short: dropped in the review,
        // 11-15.)
        if p.free && with_head {
            let sub = "none commanded: all on their own, WASD pans";
            let left = inner.x + hw + 12.0 * s;
            if c.atlas.measure(Weight::Medium, 13.5 * s, sub) <= bx - left - 6.0 * s {
                c.text(Weight::Medium, 13.5 * s, (left, inner.y + 15.0 * s), Align::Left, sub, MUTED);
            }
        }
        // Wider than a soldier's card since the UI review (11-15; 72): the
        // squad's state ("holding", "working") on the tile, not only in
        // its tooltip, in the room the panel had spare.
        // (Narrower, down to the old 72, where the row cannot hold them all.)
        // (At least 86, for the squad thread's AUTO / YOU tag.)
        // (At most 168, up from 136 in the play-zoom pass, 11-20: one
        // squad's tile in a panel with room for three cut "working" to
        // "working..." beside the YOU tag.)
        // Too many squads for the row at full size: the tiles shrink, all
        // of them, till every squad has one (owner, 2026-11-19 16:55Z:
        // "when we have too many squads to show in the squad panel of the
        // hud, instead of hiding some, the squad tile should shrink").
        // Only past the smallest tile does the last say "+N" as before.
        //
        // Three sizes (owner, 2026-11-22: "when more than 4 squads the
        // shrunken tiles need to be improved"). Until then the whole tile
        // scaled down, height and all, to 0.4, and below 0.575 became a
        // bare "1: 4" in a squat box with a hairline bar: at twelve squads
        // a strip of tiny boxes along the top with half the panel empty
        // under it. Now, in the same panel height:
        // - Full (one row, 86 to 168 wide): as before.
        // - Compact (one row, full height, down to 48 wide): the key small
        //   top left, the soldier and the count big in the middle, the
        //   health bar along the foot, as the full tile has them.
        // - Mini (two rows of half height, down to 40 wide): the key and
        //   the count on one line, a thin health bar.
        let top = inner.y + 26.0 * s;
        let room_h = (inner.bottom() - top).max(20.0 * s);
        let full_h = (54.0 * s).min(room_h);
        let fit = |gap: f32, cols: usize| (inner.w + gap) / cols.max(1) as f32 - gap;
        #[derive(Clone, Copy, PartialEq)]
        enum Size {
            Full,
            Compact,
            Mini,
        }
        let (size, cw, ch, cgap, cols, rows) = if fit(7.0 * s, n) >= 86.0 * s {
            (Size::Full, fit(7.0 * s, n).min(168.0 * s), full_h, 7.0 * s, n.max(1), 1)
        } else if fit(5.0 * s, n) >= 48.0 * s {
            (Size::Compact, fit(5.0 * s, n), full_h, 5.0 * s, n, 1)
        } else {
            let g = 4.0 * s;
            let rows = if room_h >= 2.0 * 20.0 * s + g { 2 } else { 1 };
            let want = n.div_ceil(rows);
            let cols = if fit(g, want) >= 40.0 * s { want } else { ((inner.w + g) / (40.0 * s + g)).floor().max(1.0) as usize };
            (Size::Mini, fit(g, cols), (room_h.min(54.0 * s) - g * (rows - 1) as f32) / rows as f32, g, cols, rows)
        };
        // Sizes inside a full tile.
        let u = s;
        let room = cols * rows;
        // A key the squad has ("1" to "9"); past nine squads none.
        fn key_of(q: &SquadRow) -> Option<&str> {
            Some(q.key.as_str()).filter(|k| !k.is_empty() && *k != "-")
        }
        for (k, q) in p.rows.iter().enumerate() {
            if k >= room {
                break;
            }
            let (i, j) = (k % cols, k / cols);
            let cell = Rect::new(inner.x + i as f32 * (cw + cgap), top + j as f32 * (ch + cgap), cw, ch);
            if k == room - 1 && n > room {
                c.rrect(cell, 6.0 * s, [255, 255, 255, 14]);
                c.text(Weight::SemiBold, 13.0 * s, (cell.x + cw / 2.0, cell.y + ch / 2.0 + 4.5 * s), Align::Centre, &format!("+{}", n - room + 1), MUTED);
                break;
            }
            // Top right: the link toggle; left of it, the + while the
            // squad overlaps yours.
            let link = Rect::new(cell.right() - 21.0 * u, cell.y + 4.0 * u, 17.0 * u, 17.0 * u);
            let plus = Rect::new(link.x - 20.0 * u, link.y, 17.0 * u, 17.0 * u);
            let small = size != Size::Full;
            let can_link = !small && !q.active && (q.near || q.linked);
            let on_link = can_link && self.hover.is_some_and(|h| link.contains(h));
            let on_plus = !small && !q.active && q.near && self.hover.is_some_and(|h| plus.contains(h));
            let hot = self.hover.is_some_and(|h| cell.contains(h)) && !on_plus && !on_link;
            let (t, b) = if hot { (BUTTON_HOVER_TOP, BUTTON_HOVER_BOTTOM) } else { (BUTTON_TOP, BUTTON_BOTTOM) };
            let edge = if q.active { ACCENT } else if q.linked { alpha(ACCENT, 0.55) } else { EDGE };
            let r = if size == Size::Mini { 5.0 * s } else { 8.0 * s };
            c.panel(cell, r, t, b, if q.active || q.linked { 2.0 * s } else { 1.0 }, edge);
            self.hits.push((cell, Hit::Squad(k)));
            let ink = if q.active { ACCENT } else { TEXT };
            let count = format!("{}", q.soldiers);
            if size == Size::Compact {
                // The key top left; the soldier and the count in the middle
                // (the count alone where the two do not fit); the health
                // along the foot.
                if let Some(key) = key_of(q) {
                    c.text(Weight::SemiBold, 11.5 * s, (cell.x + 6.0 * s, cell.y + 13.0 * s), Align::Left, key, if q.active { ACCENT } else { MUTED });
                }
                let px = 19.0 * s;
                let nw = c.atlas.measure(Weight::SemiBold, px, &count);
                let icon = 17.0 * s;
                let with_icon = icon + 4.0 * s + nw <= cw - 10.0 * s;
                let w = if with_icon { icon + 4.0 * s + nw } else { nw };
                let x0 = cell.x + (cw - w) / 2.0;
                let mid = cell.y + 0.56 * ch;
                if with_icon {
                    c.icon(Icon::Soldier, (x0, mid - icon / 2.0 - 1.0 * s), icon, ink);
                }
                c.text(Weight::SemiBold, px, (x0 + w - nw, mid + 0.36 * px - 1.0 * s), Align::Left, &count, ink);
                c.bar(Rect::new(cell.x + 5.0 * s, cell.bottom() - 9.0 * s, cw - 10.0 * s, 4.0 * s), q.hp, health_colour(q.hp), TRACK);
                // A squad on its own: the AUTO tag's green as a dot, top
                // right (the word does not fit).
                if q.auto && !q.active {
                    c.rrect(Rect::new(cell.right() - 10.0 * s, cell.y + 6.0 * s, 5.0 * s, 5.0 * s), 2.5 * s, GOOD);
                }
            }
            if size == Size::Mini {
                // The key and the count on one line, the health under them.
                let px = (0.5 * ch).clamp(10.5 * s, 13.0 * s);
                let y = cell.y + 0.5 * ch + 0.36 * px - 2.0 * s;
                match key_of(q) {
                    Some(key) if cw >= 46.0 * s => {
                        c.text(Weight::SemiBold, 10.5 * s, (cell.x + 6.0 * s, y), Align::Left, key, if q.active { ACCENT } else { MUTED });
                        c.text(Weight::SemiBold, px, (cell.right() - 6.0 * s, y), Align::Right, &count, ink);
                    }
                    _ => {
                        c.text(Weight::SemiBold, px, (cell.x + cw / 2.0, y), Align::Centre, &count, ink);
                    }
                }
                c.bar(Rect::new(cell.x + 4.0 * s, cell.bottom() - 5.0 * s, cw - 8.0 * s, 2.5 * s), q.hp, health_colour(q.hp), TRACK);
            }
            if small {
                if hot {
                    let state = q.note.as_deref().unwrap_or("");
                    let key = key_of(q).map_or(String::new(), |k| format!(" (key {k})"));
                    let what = if q.active { format!("{}: {} soldiers, {state}. You command it", q.name, q.soldiers) } else { format!("{}: {} soldiers, {state}. Click to command it{key}", q.name, q.soldiers) };
                    tip = Some((what, cell));
                }
                continue;
            }
            // The hotkey, top left; the soldiers, big; their health along the foot.
            c.text(Weight::SemiBold, 13.0 * u, (cell.x + 7.0 * u, cell.y + 15.0 * u), Align::Left, &q.key, if q.active { ACCENT } else { MUTED });
            let ink = if q.active { ACCENT } else { TEXT };
            c.icon(Icon::Soldier, (cell.x + 8.0 * u, cell.y + 19.0 * u), 20.0 * u, ink);
            let nw = c.text(Weight::SemiBold, 19.0 * u, (cell.x + 32.0 * u, cell.y + 37.0 * u), Align::Left, &format!("{}", q.soldiers), ink);
            c.bar(Rect::new(cell.x + 6.0 * u, cell.bottom() - 11.0 * u, cw - 12.0 * u, 5.0 * u), q.hp, health_colour(q.hp), TRACK);
            // Right of the count: AUTO on a squad running on its own, YOU
            // on the one you command (owner 2026-11-15 02:58Z); the
            // squad's state between the count and the tag where it fits.
            let tag = if q.active { Some(("YOU", ACCENT)) } else if q.auto { Some(("AUTO", GOOD)) } else { None };
            let mut end = cell.right() - 6.0 * u;
            if let Some((t, ink)) = tag {
                let tw = c.text(Weight::SemiBold, 11.5 * u, (end, cell.y + 36.0 * u), Align::Right, t, ink);
                end -= tw + 8.0 * u;
            }
            // Its activity ("firing", "holding"): the note's first word was
            // "on its own" for an AUTO squad, beside the AUTO tag.
            let first = q.note.as_deref().and_then(|n| n.split(", ").find(|w| !w.is_empty()));
            if let Some(state) = Some(q.state.as_str()).filter(|t| !t.is_empty()).or(first) {
                let sx = cell.x + 32.0 * u + nw + 8.0 * u;
                if end - sx >= 30.0 * u {
                    // "working, 3 on guard" where it fits, else "working":
                    // a whole word reads, a cut one ("working, 3 o...") not.
                    // Not even the first word whole ("wor...", "holdi..." at
                    // five squads, 11-22): none, the tooltip has it.
                    let fits = c.atlas.measure(Weight::Medium, 12.5 * u, state) <= end - sx;
                    let state = if fits { state } else { state.split(", ").next().unwrap_or(state) };
                    if c.atlas.measure(Weight::Medium, 12.5 * u, state) <= end - sx {
                        c.text(Weight::Medium, 12.5 * u, (sx, cell.y + 36.0 * u), Align::Left, state, MUTED);
                    }
                }
            }
            if can_link {
                let (t, b) = if on_link { (BUTTON_HOVER_TOP, BUTTON_HOVER_BOTTOM) } else { (PANEL_TOP, PANEL_BOTTOM) };
                c.panel(link, 4.0 * u, t, b, 1.0, if q.linked { alpha(ACCENT, 0.8) } else { EDGE });
                c.icon(Icon::Group, (link.x + 2.0 * u, link.y + 2.0 * u), 13.0 * u, if q.linked { ACCENT } else if on_link { TEXT } else { MUTED });
                self.hits.push((link, Hit::SquadLink(k)));
            }
            if !q.active && q.near {
                let (t, b) = if on_plus { (BUTTON_HOVER_TOP, BUTTON_HOVER_BOTTOM) } else { (PANEL_TOP, PANEL_BOTTOM) };
                c.panel(plus, 4.0 * u, t, b, 1.0, EDGE);
                c.text(Weight::SemiBold, 13.0 * u, (plus.x + plus.w / 2.0, plus.y + 13.0 * u), Align::Centre, "+", if on_plus { GOOD } else { MUTED });
                self.hits.push((plus, Hit::SquadInto(k)));
            }
            let state = q.note.as_deref().unwrap_or("");
            if on_link {
                tip = Some((if q.linked { format!("{} is linked with yours: click to unlink", q.name) } else { format!("Link {} with yours: they move and take orders together, each with its own guns and settings", q.name) }, link));
            } else if on_plus {
                tip = Some((format!("{} joins the squad you command", q.name), plus));
            } else if hot {
                let what = if q.active { format!("{}: {} soldiers, {state}. You command it", q.name, q.soldiers) } else { format!("{}: {} soldiers, {state}. Click to command it (key {})", q.name, q.soldiers, q.key) };
                tip = Some((what, cell));
            }
        }
        // The commanded squad's settings, right of the tiles in the room
        // a short row leaves: two lines of icon-and-words, flowed in the
        // window's order, and what does not fit is left off (not cut).
        if n > 0 && n <= cols && !p.free && !facts.is_empty() {
            let x0 = inner.x + n as f32 * (cw + cgap) - cgap + 12.0 * s;
            if inner.right() - x0 >= 120.0 * s {
                c.rrect(Rect::new(x0, top + 4.0 * s, s.max(1.0), ch - 8.0 * s), 0.0, EDGE);
                let left = x0 + 12.0 * s;
                let lh = ch / 2.0;
                // Placed first, so a strip that fits on one line sits on
                // the tiles' middle rather than on their top half.
                let (mut x, mut line) = (left, 0);
                let mut placed = Vec::new();
                for f in facts {
                    let w = 21.0 * s + c.atlas.measure(Weight::Medium, 12.5 * s, &f.text);
                    if x + w > inner.right() && x > left {
                        line += 1;
                        x = left;
                    }
                    if line > 1 || x + w > inner.right() {
                        continue;
                    }
                    placed.push((f, x, line, w));
                    x += w + 16.0 * s;
                }
                let one = placed.iter().all(|p| p.2 == 0);
                for (f, x, line, w) in placed {
                    let y = top + if one { lh / 2.0 } else { line as f32 * lh };
                    let hit = Rect::new(x - 3.0 * s, y + 2.0 * s, w + 6.0 * s, lh - 4.0 * s);
                    let hot = self.hover.is_some_and(|h| hit.contains(h));
                    c.icon(f.icon, (x, y + lh / 2.0 - 8.0 * s), 16.0 * s, if hot { TEXT } else { MUTED });
                    c.text(Weight::Medium, 12.5 * s, (x + 21.0 * s, y + lh / 2.0 + 4.5 * s), Align::Left, &f.text, if hot { TEXT } else { MUTED });
                    self.hits.push((hit, Hit::Panel));
                    if hot {
                        tip = Some((f.about.clone(), hit));
                    }
                }
            }
        }
        if let Some((t, at)) = tip {
            let sw = c.screen.0 as f32;
            let w = c.atlas.measure(Weight::Medium, 13.5 * s, &t) + 18.0 * s;
            self.tip_small(&t, (at.x.min(sw - 8.0 * s - w).max(8.0 * s), at.y - 30.0 * s), c, s);
        }
    }

    fn chip(&mut self, r: Rect, hit: Hit, on: bool, c: &mut Canvas, s: f32) {
        let hot = self.hover.is_some_and(|p| r.contains(p));
        c.shadow(r, r.h / 2.0, 10.0 * s, 2.0 * s, SHADOW);
        let (t, b) = if hot { (BUTTON_HOVER_TOP, BUTTON_HOVER_BOTTOM) } else { (PANEL_TOP, PANEL_BOTTOM) };
        c.panel(r, r.h / 2.0, t, b, 1.0, if on { alpha(ACCENT, 0.8) } else { EDGE });
        self.hits.push((r, hit));
    }

    fn info_draw(&mut self, info: &Info, r: Rect, c: &mut Canvas, s: f32) {
        let tile = (r.h - 4.0 * s).min(88.0 * s);
        let t = Rect::new(r.x, r.y + 2.0 * s, tile, tile);
        c.panel(t, 10.0 * s, [36, 44, 54, 255], [22, 27, 34, 255], 1.5 * s, alpha(info.colour, 0.55));
        c.icon(info.icon, (t.x + tile * 0.2, t.y + tile * 0.2), tile * 0.6, info.colour);
        let x = t.right() + 16.0 * s;
        let w = r.right() - x;
        c.text_fit(Weight::SemiBold, 18.0 * s, (x, r.y + 18.0 * s), Align::Left, &info.title, w, TEXT);
        c.text_fit(Weight::Medium, 13.5 * s, (x, r.y + 37.0 * s), Align::Left, &info.subtitle, w, MUTED);
        let mut y = r.y + 50.0 * s;
        if let Some((f, words)) = &info.hp {
            c.icon(Icon::Heart, (x, y - 1.0 * s), 14.0 * s, health_colour(*f));
            let bw = (w - 110.0 * s).max(60.0 * s);
            c.bar(Rect::new(x + 20.0 * s, y + 3.0 * s, bw, 7.0 * s), *f, health_colour(*f), TRACK);
            c.text(Weight::SemiBold, 13.5 * s, (x + 28.0 * s + bw, y + 11.0 * s), Align::Left, words, TEXT);
            y += 24.0 * s;
        }
        let colw = w / 2.0;
        for (k, (label, value)) in info.stats.iter().enumerate() {
            let (i, j) = (k % 2, k / 2);
            let sx = x + i as f32 * colw;
            let sy = y + j as f32 * 19.0 * s + 12.0 * s;
            if sy > r.bottom() {
                break;
            }
            let lw = c.text(Weight::Medium, 13.0 * s, (sx, sy), Align::Left, label, MUTED);
            c.text_fit(Weight::SemiBold, 13.5 * s, (sx + lw + 8.0 * s, sy), Align::Left, value, colw - lw - 16.0 * s, TEXT);
        }
        if let Some(n) = &info.note {
            c.text_fit(Weight::Medium, 13.0 * s, (x, r.bottom() - 2.0 * s), Align::Left, n, w, WARN);
        }
    }

    fn command_card(&mut self, m: &Model, r: Rect, c: &mut Canvas, s: f32) {
        self.glass(r, 12.0 * s, c, s);
        let pad = CARD_PAD * s;
        let cell = CELL * s;
        let gap = CELL_GAP * s;
        // The buttons' width from the card's (`draw` picks square or
        // labelled).
        let cw = (r.w - 2.0 * pad - (CARD_COLS - 1) as f32 * gap) / CARD_COLS as f32;
        let labelled = cw > cell * 1.5;
        let x0 = r.x + pad;
        let y0 = r.bottom() - pad - CARD_ROWS as f32 * cell - (CARD_ROWS - 1) as f32 * gap;
        // Empty slots, faintly, so the grid reads as a grid.
        for j in 0..CARD_ROWS {
            for i in 0..CARD_COLS {
                let b = Rect::new(x0 + i as f32 * (cw + gap), y0 + j as f32 * (cell + gap), cw, cell);
                c.rrect(b, 10.0 * s, [255, 255, 255, 8]);
            }
        }
        let mut tip = None;
        for (k, cmd) in m.commands.iter().enumerate() {
            let (row, col) = card_slot(cmd.slot);
            let (row, col) = (row as f32, col as f32);
            let b = Rect::new(x0 + col * (cw + gap), y0 + row * (cell + gap), cw, cell);
            let hot = self.hover.is_some_and(|p| b.contains(p));
            let down = hot && self.pressed == Some(Hit::Command(k));
            let aiming = self.targeting.is_some_and(|t| t.id == cmd.id);
            let (mut t, mut bt) = if hot && !down { (BUTTON_HOVER_TOP, BUTTON_HOVER_BOTTOM) } else { (BUTTON_TOP, BUTTON_BOTTOM) };
            if down {
                (t, bt) = (BUTTON_BOTTOM, BUTTON_TOP);
            }
            if cmd.on || aiming {
                t = [36, 62, 84, 255];
                bt = [24, 40, 56, 255];
            }
            let edge = if aiming || cmd.on { alpha(ACCENT, 0.9) } else { [255, 255, 255, 30] };
            c.panel(b, 10.0 * s, t, bt, if cmd.on || aiming { 1.5 * s } else { 1.0 }, edge);
            let dim = if cmd.ready { 1.0 } else { 0.38 };
            let ink = alpha(if cmd.on { ACCENT } else { TEXT }, dim);
            if labelled {
                // The icon at the left, the name beside it on one or two
                // lines (or one line and the cost), the key top right.
                let ic = 20.0 * s;
                c.icon(cmd.icon, (b.x + 8.0 * s, b.y + (cell - ic) / 2.0), ic, ink);
                let tx = b.x + 8.0 * s + ic + 7.0 * s;
                let key = key_name(cmd.key);
                let kw = key.as_ref().map_or(0.0, |k| keycap_w(c, k, 11.0 * s, s) + 6.0 * s);
                let room = b.right() - tx - 6.0 * s;
                if let Some(k) = &key {
                    keycap(c, k, (b.right() - 4.0 * s, b.bottom() - 4.0 * s), 11.0 * s, dim.max(0.6), s);
                }
                let label = short_label(&cmd.name);
                let words = alpha(TEXT, dim.max(0.55));
                // The name on the first line (shrunk a little, to 11.5, before
                // it wraps), then the cost or the rest of the name; the
                // key at the bottom right, so the first line has the width.
                let px = fit_px(c, &label, 13.0 * s, 11.5 * s, room);
                // One line only clear of the key (since the card's sixth
                // column, 11-21, a name that filled the narrower button ran
                // under its key: "Drain crystal" over its G).
                let one = c.atlas.measure(Weight::SemiBold, px, &label) <= room - kw;
                let second = room - kw;
                // Locked: a lock and "Locked" where the price goes, grey. A
                // red price read as "can't afford" with the bank full (the
                // review, 11-15).
                let cost = if cmd.locked { Some("Locked") } else { cmd.cost.as_deref() };
                if let Some(cost) = cost {
                    c.text_fit(Weight::SemiBold, px, (tx, b.y + 18.0 * s), Align::Left, &label, room, words);
                    if cmd.locked {
                        lock_glyph(c, (tx + 1.0 * s, b.y + 24.5 * s), 11.0 * s, alpha(MUTED, 0.95));
                    } else {
                        c.icon(cmd.cost_icon, (tx, b.y + 24.5 * s), 11.0 * s, alpha(ENERGY, dim.max(0.6)));
                    }
                    let col = if cmd.locked { MUTED } else if cmd.ready { ENERGY } else { BAD };
                    // Shrunk to 10.5 before it is cut: at 1536 wide the
                    // sixth column left "Loc..." and "20/..." (polish
                    // review finding 7, 11-24). A lock that still doesn't
                    // fit its word keeps the lock alone.
                    let cr = second - 14.0 * s;
                    // "20/cell" that won't fit keeps its number: the tip
                    // says it costs by the cell.
                    let cost = match cost.split_once('/') {
                        Some((n, _)) if c.atlas.measure(Weight::SemiBold, 10.5 * s, cost) > cr => n,
                        _ => cost,
                    };
                    // The atlas measures by whole-pixel sizes, so a scaled
                    // size can still overrun by a hair: then the least.
                    let mut cpx = fit_px(c, cost, 12.0 * s, 10.5 * s, cr);
                    if c.atlas.measure(Weight::SemiBold, cpx, cost) > cr {
                        cpx = 10.5 * s;
                    }
                    if !(cmd.locked && c.atlas.measure(Weight::SemiBold, cpx, cost) > cr) {
                        c.text_fit(Weight::SemiBold, cpx, (tx + 14.0 * s, b.y + 34.0 * s), Align::Left, cost, cr, alpha(col, 0.95));
                    }
                } else if one {
                    c.text(Weight::SemiBold, px, (tx, b.y + cell / 2.0 + 4.5 * s), Align::Left, &label, words);
                } else {
                    let (first, rest) = split_label(c, &label, 12.0 * s, room);
                    c.text_fit(Weight::SemiBold, 12.0 * s, (tx, b.y + 18.0 * s), Align::Left, &first, room, words);
                    c.text_fit(Weight::SemiBold, 12.0 * s, (tx, b.y + 34.0 * s), Align::Left, &rest, second, words);
                }
            } else {
                let ic = 22.0 * s;
                c.icon(cmd.icon, (b.x + (cell - ic) / 2.0, b.y + (cell - ic) / 2.0 - 2.0 * s), ic, ink);
                // The hotkey, top left.
                if let Some(k) = key_name(cmd.key) {
                    let kw = keycap_w(c, &k, 10.5 * s, s);
                    keycap(c, &k, (b.x + 3.0 * s + kw, b.y + 3.0 * s + 15.5 * s), 10.5 * s, dim.max(0.6), s);
                }
                if cmd.locked {
                    lock_glyph(c, (b.right() - 13.0 * s, b.bottom() - 14.0 * s), 10.5 * s, alpha(MUTED, 0.95));
                } else if let Some(cost) = &cmd.cost {
                    let cw = c.atlas.measure(Weight::SemiBold, 10.5 * s, cost);
                    let cx = b.right() - 4.0 * s - cw;
                    c.text(Weight::SemiBold, 10.5 * s, (cx, b.bottom() - 3.5 * s), Align::Left, cost, alpha(if cmd.ready { ENERGY } else { BAD }, 0.95));
                    c.icon(cmd.cost_icon, (cx - 11.0 * s, b.bottom() - 13.0 * s), 9.5 * s, alpha(ENERGY, dim.max(0.6)));
                }
            }
            if cmd.cooldown > 0.0 {
                // A dark wipe over the part still to go (a ring round the
                // edge read as a broken border), and the time left.
                c.ring(b, 10.0 * s, b.w.max(b.h), cmd.cooldown.min(1.0), [0, 0, 0, 150]);
                if let Some(wt) = &cmd.wait {
                    c.text(Weight::SemiBold, 15.0 * s, (b.x + b.w / 2.0, b.y + cell / 2.0 + 5.0 * s), Align::Centre, wt, TEXT);
                }
            }
            self.hits.push((b, Hit::Command(k)));
            if hot {
                tip = Some((k, b));
            }
        }
        if let Some((k, b)) = tip {
            self.tooltip(&m.commands[k], b, r, c, s);
        }
    }

    /// The loadout strip: the gun's cards as a row of glyphs in its slots,
    /// left to right, the cast its next shot fires lit, the loadout's name
    /// and state, and Swap. (Until the recharge went, owner 18:05Z, a thin
    /// bar under the cards filled back up after the last cast.) It sits
    /// on the selection panel's top edge at the right (the art director,
    /// 11-14 02:34Z: a strip of card icons next to the selection, not a
    /// tall box; the details in a tooltip). Before that review it was a
    /// 140-pixel box over the command card with each card's name under its
    /// glyph. Returns the hovered card (or `usize::MAX` for the name and
    /// state, whose tooltip is the whole loadout) and its rectangle.
    #[allow(clippy::too_many_arguments)]
    fn loadout_draw(&mut self, l: &Loadout, panel: Option<Rect>, sel: Rect, collapsed: bool, card: Rect, fit: GunFit, c: &mut Canvas, b: f32) -> Option<(usize, Rect)> {
        let s = b * fit.scale;
        let pad = 8.0 * s;
        let cell = LOADOUT_CELL * s;
        let gap = 5.0 * s;
        let slots = l.slots.max(l.cards.len()).max(1);
        let info_w = LOADOUT_INFO * s;
        let swap = LOADOUT_SWAP * s;
        let w = loadout_w(l, fit, b);
        let h = 2.0 * pad + cell + 5.0 * s;
        // Its own panel at the bar's far left (11-14 08:46Z), the strip
        // centred in it; else, as before, on the selection panel's top edge
        // at its right, or the command card's where the window is too
        // narrow; with the selection collapsed, at the bottom of its place.
        let r = if let Some(p) = panel {
            Rect::new(p.x, p.y + (p.h - h) / 2.0, p.w, h)
        } else if collapsed && sel.w >= w + 158.0 * s {
            Rect::new(sel.right() - w, sel.bottom() - h, w, h)
        } else {
            let on = if sel.w >= 120.0 * s && sel.w >= w + 150.0 * s { sel } else { card };
            Rect::new((on.right() - w).max(8.0 * s), on.y - 6.0 * s - h, w, h)
        };
        if r.y < self.top + 8.0 * s {
            return None;
        }
        match panel {
            Some(p) => self.glass(p, 12.0 * s, c, s),
            None => self.glass(r, 10.0 * s, c, s),
        }
        let mut tip = None;
        let y = r.y + pad;
        let mut x = r.x + pad;
        // No cards yet: one well over the empty slots saying so (the polish
        // review, 11-24, finding 6: four blank boxes beside the gun's name
        // said nothing).
        if l.cards.is_empty() {
            let well = Rect::new(x, y, slots as f32 * cell + (slots - 1) as f32 * gap, cell);
            c.rrect(well, (7.0 * s).min(well.h / 2.0), [255, 255, 255, 7]);
            c.outline(well, (7.0 * s).min(well.h / 2.0), 1.0, [255, 255, 255, 16]);
            let room = well.w - 14.0 * s;
            let (one, two) = ("No gun cards yet", "the shop sells them");
            if room >= c.atlas.measure(Weight::Medium, 12.0 * s, two) && slots > 1 {
                c.text(Weight::SemiBold, 12.5 * s, (well.x + well.w / 2.0, well.y + well.h / 2.0 - 2.0 * s), Align::Centre, one, alpha(MUTED, 0.95));
                c.text(Weight::Medium, 12.0 * s, (well.x + well.w / 2.0, well.y + well.h / 2.0 + 13.0 * s), Align::Centre, two, alpha(MUTED, 0.7));
            } else {
                c.text_fit(Weight::Medium, 11.5 * s, (well.x + well.w / 2.0, well.y + well.h / 2.0 + 4.0 * s), Align::Centre, "No cards", room, alpha(MUTED, 0.85));
            }
            x = well.right() + gap;
        }
        for i in 0..slots {
            if l.cards.is_empty() {
                break;
            }
            let Some(k) = l.cards.get(i) else {
                // An empty slot (only the shop or H fills it).
                let b = Rect::new(x, y, cell, cell);
                x = b.right() + gap;
                c.rrect(b, (7.0 * s).min(b.w / 2.0), [255, 255, 255, 8]);
                c.outline(b, (7.0 * s).min(b.w / 2.0), 1.0, [255, 255, 255, 18]);
                continue;
            };
            let b = Rect::new(x, y, cell, cell);
            x = b.right() + gap;
            let frame = k.kind.colour(k.colour);
            let dim = if k.idle { 0.4 } else { 1.0 };
            if k.next {
                c.shadow(b, 7.0 * s, 10.0 * s, 0.0, alpha(k.colour, 0.5));
            }
            let shade = |a: Rgba, f: f32| -> Rgba { [(a[0] as f32 * f) as u8, (a[1] as f32 * f) as u8, (a[2] as f32 * f) as u8, 255] };
            let edge = if k.next { k.colour } else { alpha(frame, 0.55 * dim) };
            c.panel(b, 7.0 * s, shade(frame, 0.30 * dim), shade(frame, 0.12 * dim), if k.next { 2.0 * s } else { 1.0 * s }, edge);
            // The kind's stripe along the top, as a card's frame.
            c.rrect(Rect::new(b.x + 5.0 * s, b.y + 3.0 * s, b.w - 10.0 * s, 2.0 * s), 1.0 * s, alpha(frame, 0.9 * dim));
            let ic = 26.0 * s;
            if let Some(icon) = k.icon {
                c.icon(icon, (b.x + (b.w - ic) / 2.0, b.y + (b.h - ic) / 2.0 + 1.5 * s), ic, alpha(k.colour, dim));
            }
            if let Some(badge) = k.badge {
                let bs = 14.0 * s;
                c.icon(badge, (b.right() - bs - 2.0 * s, b.bottom() - bs - 2.0 * s), bs, alpha(TEXT, 0.9 * dim));
            }
            if k.next {
                // A notch above the lit cast.
                let nx = b.x + b.w / 2.0;
                c.rrect(Rect::new(nx - 5.0 * s, b.y - 4.0 * s, 10.0 * s, 2.5 * s), 1.25 * s, k.colour);
            }
            if self.hover.is_some_and(|p| b.contains(p)) {
                tip = Some((i, b));
            }
        }
        // The name and the state, after the last slot, to Swap: past a
        // gun with fewer slots than the widest in play the name has their
        // room (`Loadout::widest`).
        let tx = x - gap + 10.0 * s;
        let info_w = (r.right() - pad - if fit.swap && l.cycle.is_some() { swap + 6.0 * s } else { 0.0 } - tx).max(info_w);
        if !fit.name {
            // Only the gun's icon where the row is short: its names are in
            // the tooltip it opens (the review, 11-15: shed parts, never
            // shrink the words).
            let icon = l.gun.as_ref().map_or(Icon::Rifle, |g| g.0);
            c.icon(icon, (tx, y + (cell - 18.0 * s) / 2.0), 18.0 * s, MUTED);
            let words = Rect::new(tx - 4.0 * s, r.y, GUN_TAB * s + 4.0 * s, r.h);
            if tip.is_none() && self.hover.is_some_and(|p| words.contains(p)) {
                tip = Some((usize::MAX, words));
            }
        } else {
            tip = self.loadout_words(l, tx, y, r, info_w, tip, c, s);
        }
        if let (Some((_, key)), true) = (l.cycle, fit.swap) {
            // As tall as a card, its key on a keycap over the word for
            // what it does (the polish review, 11-24, finding 6: a lone
            // "H" box was unexplained).
            let b = Rect::new(r.right() - pad - swap, y, swap, cell);
            let hot = self.hover.is_some_and(|p| b.contains(p));
            let (t, bt) = if hot { (BUTTON_HOVER_TOP, BUTTON_HOVER_BOTTOM) } else { (BUTTON_TOP, BUTTON_BOTTOM) };
            c.panel(b, 6.0 * s, t, bt, 1.0, [255, 255, 255, 30]);
            let k = key_name(key).unwrap_or_else(|| "~".into());
            let kw = keycap_w(c, &k, 11.0 * s, s);
            keycap(c, &k, (b.x + (b.w + kw) / 2.0, b.y + 22.0 * s), 11.0 * s, 1.0, s);
            c.text_fit(Weight::SemiBold, 10.5 * s, (b.x + b.w / 2.0, b.bottom() - 7.0 * s), Align::Centre, "Loadout", b.w - 6.0 * s, alpha(MUTED, 0.95));
            self.hits.push((b, Hit::Cycle));
            if hot {
                tip = Some((usize::MAX - 1, b));
            }
        }
        tip
    }

    /// The loadout strip's name column: the gun's icon and name, then
    /// what its cards make. Returns the hovered part, as `loadout_draw`.
    #[allow(clippy::too_many_arguments)]
    fn loadout_words(&mut self, l: &Loadout, tx: f32, y: f32, r: Rect, info_w: f32, tip: Option<(usize, Rect)>, c: &mut Canvas, s: f32) -> Option<(usize, Rect)> {
        let mut tip = tip;
        // The gun's icon ahead of the name, the name after it.
        let gx = if let Some((icon, ..)) = &l.gun {
            c.icon(*icon, (tx, y + 4.0 * s), 18.0 * s, MUTED);
            22.0 * s
        } else {
            0.0
        };
        // The gun's name first ("Rifle"), then what its cards make: the
        // loadout's name, or plain shots. (It read "Plain / Plain shots"
        // with the gun only an icon: the review, 11-15.)
        let (head, state, col) = match &l.gun {
            Some((_, gun, _)) if l.cards.is_empty() => (gun.as_str(), "Plain shots", MUTED),
            Some((_, gun, _)) => (gun.as_str(), l.name.as_str(), GOOD),
            None if l.cards.is_empty() => (l.name.as_str(), "Plain shots", MUTED),
            None => (l.name.as_str(), "A card a shot", GOOD),
        };
        c.text_fit(Weight::SemiBold, 15.0 * s, (tx + gx, y + 18.0 * s), Align::Left, head, info_w - gx, TEXT);
        c.text_fit(Weight::Medium, 13.0 * s, (tx, y + 38.0 * s), Align::Left, state, info_w, col);
        let words = Rect::new(tx - 4.0 * s, r.y, info_w + 4.0 * s, r.h);
        if tip.is_none() && self.hover.is_some_and(|p| words.contains(p)) {
            tip = Some((usize::MAX, words));
        }
        tip
    }

    /// The loadout's own tooltip (over its name, or Swap): whose gun, its
    /// numbers, the note, and how to swap.
    fn loadout_tip(&mut self, l: &Loadout, b: Rect, swap: bool, c: &mut Canvas, s: f32) {
        let mut lines = vec![l.whose.clone()];
        if let Some((_, title, numbers)) = &l.gun {
            lines.push(format!("{title}: {numbers}"));
        }
        if let Some(n) = &l.note {
            lines.push(n.clone());
        }
        if !l.cards.is_empty() {
            lines.push("Each shot fires the next card in turn".into());
        }
        if let Some((_, key)) = l.cycle {
            lines.push(match key_name(key) {
                Some(k) if swap => format!("Click or press {k}: the next loadout"),
                Some(k) => format!("{k} or the button: the next loadout"),
                None => "The button: the next loadout".into(),
            });
        }
        let w = 300.0 * s;
        let h = 40.0 * s + lines.len() as f32 * 19.5 * s;
        let sw = c.screen.0 as f32;
        let x = (b.x + b.w / 2.0 - w / 2.0).min(sw - 8.0 * s - w).max(8.0 * s);
        let r = Rect::new(x, (b.y - 14.0 * s - h).max(8.0 * s), w, h);
        c.shadow(r, 10.0 * s, 18.0 * s, 4.0 * s, SHADOW);
        c.panel(r, 10.0 * s, [34, 40, 49, 248], [24, 29, 36, 250], 1.0, EDGE);
        let mut y = r.y + 26.0 * s;
        c.icon(l.gun.as_ref().map_or(Icon::Rifle, |g| g.0), (r.x + 14.0 * s, y - 14.0 * s), 16.0 * s, MUTED);
        c.text(Weight::SemiBold, 15.0 * s, (r.x + 38.0 * s, y), Align::Left, &l.name, TEXT);
        y += 22.0 * s;
        for t in &lines {
            c.text_fit(Weight::Medium, 13.5 * s, (r.x + 14.0 * s, y), Align::Left, t, w - 28.0 * s, MUTED);
            y += 19.5 * s;
        }
    }

    /// A gun card's tooltip, above the loadout strip.
    fn card_tip(&mut self, k: &GunCard, b: Rect, c: &mut Canvas, s: f32) {
        let w = 320.0 * s;
        let px = 13.5 * s;
        let lines = c.wrap(Weight::Medium, px, w - 28.0 * s, &k.about);
        let extra = if k.idle { 22.0 * s } else { 0.0 };
        let h = 18.0 * s + 22.0 * s + lines.len() as f32 * 19.5 * s + 10.0 * s + extra;
        let sw = c.screen.0 as f32;
        let x = (b.x + b.w / 2.0 - w / 2.0).min(sw - 8.0 * s - w).max(8.0 * s);
        let r = Rect::new(x, (b.y - 14.0 * s - h).max(8.0 * s), w, h);
        let frame = k.kind.colour(k.colour);
        c.shadow(r, 10.0 * s, 18.0 * s, 4.0 * s, SHADOW);
        c.panel(r, 10.0 * s, [34, 40, 49, 248], [24, 29, 36, 250], 1.0, alpha(frame, 0.6));
        let mut y = r.y + 26.0 * s;
        if let Some(icon) = k.icon {
            c.icon(icon, (r.x + 14.0 * s, y - 15.0 * s), 18.0 * s, k.colour);
        }
        c.text(Weight::SemiBold, 15.0 * s, (r.x + 38.0 * s, y), Align::Left, &k.name, TEXT);
        let kind = k.kind.label().to_uppercase();
        let kw = c.atlas.measure(Weight::SemiBold, 10.5 * s, &kind) + 12.0 * s;
        let kr = Rect::new(r.right() - 14.0 * s - kw, y - 15.0 * s, kw, 20.0 * s);
        c.rrect(kr, 5.0 * s, alpha(frame, 0.18));
        c.text(Weight::SemiBold, 10.5 * s, (kr.x + kw / 2.0, y - 1.0 * s), Align::Centre, &kind, frame);
        y += 22.0 * s;
        for l in &lines {
            c.text(Weight::Medium, px, (r.x + 14.0 * s, y), Align::Left, l, MUTED);
            y += 19.5 * s;
        }
        if k.idle {
            c.text_fit(Weight::SemiBold, 12.0 * s, (r.x + 14.0 * s, y + 4.0 * s), Align::Left, "No bullet after it: it does nothing", w - 28.0 * s, WARN);
        }
    }

    fn tooltip(&mut self, cmd: &Command, b: Rect, card: Rect, c: &mut Canvas, s: f32) {
        let w = 340.0 * s;
        let px = 13.5 * s;
        let lines = c.wrap(Weight::Medium, px, w - 28.0 * s, &cmd.about);
        let why = if cmd.ready { None } else { cmd.why_not.clone() };
        let h = 18.0 * s + 22.0 * s + if cmd.about.is_empty() { 0.0 } else { lines.len() as f32 * 19.5 * s + 8.0 * s } + if why.is_some() { 20.0 * s } else { 0.0 } + 10.0 * s;
        // Not `clamp`: it panics when the card is narrower than the tip.
        let x = (b.x + b.w / 2.0 - w / 2.0).min(card.right() - w).max(8.0 * s);
        let r = Rect::new(x, card.y - h - 10.0 * s, w, h);
        c.shadow(r, 10.0 * s, 18.0 * s, 4.0 * s, SHADOW);
        c.panel(r, 10.0 * s, [34, 40, 49, 248], [24, 29, 36, 250], 1.0, EDGE);
        let mut y = r.y + 26.0 * s;
        c.text(Weight::SemiBold, 15.0 * s, (r.x + 14.0 * s, y), Align::Left, &cmd.name, TEXT);
        if let Some(k) = key_name(cmd.key) {
            let kw = c.atlas.measure(Weight::SemiBold, 11.0 * s, &k) + 12.0 * s;
            let kr = Rect::new(r.right() - 14.0 * s - kw, y - 15.0 * s, kw, 20.0 * s);
            c.rrect(kr, 5.0 * s, [255, 255, 255, 24]);
            c.outline(kr, 5.0 * s, 1.0, [255, 255, 255, 40]);
            c.text(Weight::SemiBold, 11.0 * s, (kr.x + kw / 2.0, y - 1.0 * s), Align::Centre, &k, TEXT);
        }
        if let Some(cost) = &cmd.cost {
            let kw = c.atlas.measure(Weight::SemiBold, 12.0 * s, cost);
            let off = key_name(cmd.key).map(|k| c.atlas.measure(Weight::SemiBold, 11.0 * s, &k) + 26.0 * s).unwrap_or(0.0);
            let cx = r.right() - 14.0 * s - off - kw;
            c.icon(cmd.cost_icon, (cx - 16.0 * s, y - 12.0 * s), 13.0 * s, ENERGY);
            c.text(Weight::SemiBold, 12.0 * s, (cx, y - 1.0 * s), Align::Left, cost, ENERGY);
        }
        y += 22.0 * s;
        if !cmd.about.is_empty() {
            for l in &lines {
                c.text(Weight::Medium, px, (r.x + 14.0 * s, y), Align::Left, l, MUTED);
                y += 19.5 * s;
            }
            y += 6.0 * s;
        }
        if let Some(wn) = why {
            c.text_fit(Weight::SemiBold, 13.5 * s, (r.x + 14.0 * s, y), Align::Left, &wn, w - 28.0 * s, BAD);
        }
        let hint = match cmd.target {
            Target::Instant => None,
            Target::Line { .. } => Some("Drag to lay it"),
            _ => Some("Click to place it"),
        };
        if let (Some(hn), true) = (hint, cmd.ready) {
            c.text(Weight::Medium, 12.5 * s, (r.right() - 14.0 * s, r.bottom() - 9.0 * s), Align::Right, hn, MUTED);
        }
    }

    fn alerts_draw(&mut self, c: &mut Canvas, world: &mut Marks, s: f32) {
        let now = Instant::now();
        let mut y = self.below + 12.0 * s;
        let alerts = self.alerts.clone();
        for a in alerts.iter().rev() {
            let age = now.duration_since(a.when).as_secs_f32();
            let life = ALERT_LIFE.as_secs_f32();
            let fade = ((life - age) / ALERT_FADE).clamp(0.0, 1.0);
            let w = c.atlas.measure(Weight::Medium, 14.0 * s, &a.text) + 46.0 * s;
            let r = Rect::new(8.0 * s, y, w, 32.0 * s);
            c.shadow(r, 8.0 * s, 12.0 * s, 3.0 * s, alpha(SHADOW, fade));
            c.panel(r, 8.0 * s, alpha(PANEL_TOP, fade), alpha(PANEL_BOTTOM, fade), 1.0, alpha(EDGE, fade));
            c.rrect(Rect::new(r.x, r.y + 6.0 * s, 3.0 * s, r.h - 12.0 * s), 1.5 * s, alpha(a.colour, fade));
            c.icon(Icon::Alert, (r.x + 12.0 * s, r.y + 8.0 * s), 16.0 * s, alpha(a.colour, fade));
            c.text(Weight::Medium, 14.0 * s, (r.x + 34.0 * s, r.y + 21.5 * s), Align::Left, &a.text, alpha(TEXT, fade));
            y += 38.0 * s;
            // A ping where it happened: a ring that opens over a second, on
            // the world and the minimap.
            if let Some(at) = a.at {
                if age < 3.0 {
                    let k = age % 1.0;
                    let r = 6.0 + 30.0 * k;
                    // In screen pixels, so it reads at any zoom; the flag
                    // adds its dot on the minimap.
                    world.push(Mark { flags: MINIMAP, ..Mark::new(Shape::Ring, at, (r, r), alpha(a.colour, 1.0 - k)) });
                }
            }
        }
    }

    fn ghost(&mut self, m: &Model, cam: &Camera, c: &mut Canvas, world: &mut Marks, s: f32) {
        let (Some(t), Some(p)) = (self.targeting, self.hover) else { return };
        let Some(at) = self.aim(p, cam) else { return };
        let col = match (&m.throw, m.ghost_ok) {
            (_, false) => [245, 96, 96, 255],
            (Some(w), true) => w.tint,
            (None, true) => [96, 230, 150, 255],
        };
        let fill = alpha(col, 0.28);

        let snap = m.ghost_snap.filter(|_| matches!(t.target, Target::Point { .. }));
        if let Some((c, radius)) = snap {
            // A ring only: the crystal stays in sight under it.
            world.ring(c, radius, col);
            world.ring(c, radius * 0.96, alpha(col, 0.6));
        }
        match t.target {
            Target::Point { .. } if snap.is_some() => {}
            Target::Point { radius } => {
                world.disc(at, radius, fill, false);
                if m.throw.is_some() {
                    // A dark edge outside, so the area holds on the slime.
                    world.ring(at, radius + 1.5 / cam.zoom.max(0.01), [14, 13, 10, 170]);
                }
                world.ring(at, radius, col);
            }
            Target::Area { half } => {
                world.rect(at, half, fill, false);
                world.frame((at.0 - half.0, at.1 - half.1), (at.0 + half.0, at.1 + half.1), col);
            }
            Target::Line { width } => {
                if let Some(from) = t.from {
                    world.line(from, at, 0.0, col);
                    world.disc(from, width / 2.0, fill, false);
                    world.disc(at, width / 2.0, fill, false);
                } else {
                    world.disc(at, width / 2.0, fill, false);
                    world.ring(at, width / 2.0, col);
                }
            }
            Target::Instant => {}
        }
        if let Some(w) = &m.throw {
            // Over the area, which would hide the arc's end. The reach and
            // the arc in the order lines' cream (the tools' fire colours
            // sat on the gold slime and the arc was lost in it), red when
            // it can't go; the area in the tool's colour.
            let line = if m.ghost_ok { THROW_LINE } else { col };
            throw_aim(w, w.at.unwrap_or(at), line, col, cam.zoom, world);
        }
        // A chip by the cursor: what is being placed and how.
        let name = self.commands.get(t.index).map(|k| k.name.clone()).unwrap_or_default();
        let how = match (t.target, t.from) {
            (Target::Line { .. }, None) => "drag to lay",
            (Target::Line { .. }, Some(_)) => "release to lay",
            _ => "click to place",
        };
        let text = match &m.ghost_note {
            Some(n) => format!("{name}: {n}"),
            None => format!("{name}: {how}, right-click to cancel"),
        };
        self.tip_small(&text, (p.0 + 18.0 * s, p.1 + 18.0 * s), c, s);
    }

    /// What is under the cursor, in a small card beside it: name, health
    /// and a line of stats.
    fn hover_card(&mut self, info: &Info, p: (f32, f32), a: f32, c: &mut Canvas, s: f32) {
        let (sw, sh) = (c.screen.0 as f32, c.screen.1 as f32);
        let stats = info.stats.iter().map(|(l, v)| format!("{l} {v}")).collect::<Vec<_>>().join("  ·  ");
        let w = (c.atlas.measure(Weight::SemiBold, 14.0 * s, &info.title).max(c.atlas.measure(Weight::Medium, 12.0 * s, &stats)).max(c.atlas.measure(Weight::Medium, 12.0 * s, &info.subtitle)) + 28.0 * s).max(200.0 * s);
        let h = 22.0 * s + 18.0 * s + if info.hp.is_some() { 18.0 * s } else { 0.0 } + if stats.is_empty() { 0.0 } else { 18.0 * s } + 12.0 * s;
        // Below right of the cursor, kept on the screen and off the panels:
        // where it would not fit it goes to the cursor's other side, not
        // under it (it slid left or up onto the cursor near the right edge
        // and the bar, the review 11-15).
        let x = if p.0 + 20.0 * s + w <= sw - 8.0 * s { p.0 + 20.0 * s } else { (p.0 - 20.0 * s - w).max(8.0 * s) };
        let foot = self.bottom.min(sh) - 8.0 * s;
        let y = if p.1 + 20.0 * s + h <= foot { p.1 + 20.0 * s } else { (p.1 - 20.0 * s - h).max(self.top + 8.0 * s) };
        let r = Rect::new(x, y, w, h);
        c.shadow(r, 10.0 * s, 14.0 * s, 3.0 * s, alpha(SHADOW, a));
        c.panel(r, 10.0 * s, alpha([34, 40, 49, 245], a), alpha([24, 29, 36, 248], a), 1.0, alpha(info.colour, 0.5 * a));
        let mut ty = r.y + 22.0 * s;
        c.icon(info.icon, (r.x + 12.0 * s, ty - 14.0 * s), 16.0 * s, alpha(info.colour, a));
        c.text(Weight::SemiBold, 14.0 * s, (r.x + 34.0 * s, ty), Align::Left, &info.title, alpha(TEXT, a));
        ty += 18.0 * s;
        c.text_fit(Weight::Medium, 12.0 * s, (r.x + 14.0 * s, ty), Align::Left, &info.subtitle, w - 28.0 * s, alpha(MUTED, a));
        if let Some((f, words)) = &info.hp {
            ty += 18.0 * s;
            let ww = c.atlas.measure(Weight::SemiBold, 12.0 * s, words);
            c.bar(Rect::new(r.x + 14.0 * s, ty - 8.0 * s, w - 40.0 * s - ww, 6.0 * s), *f, alpha(health_colour(*f), a), alpha(TRACK, a));
            c.text(Weight::SemiBold, 12.0 * s, (r.right() - 14.0 * s, ty), Align::Right, words, alpha(TEXT, a));
        }
        if !stats.is_empty() {
            ty += 18.0 * s;
            c.text_fit(Weight::Medium, 12.0 * s, (r.x + 14.0 * s, ty), Align::Left, &stats, w - 28.0 * s, alpha(MUTED, a));
        }
    }

    fn modal(&mut self, w: f32, h: f32, c: &mut Canvas, s: f32) -> Rect {
        let (sw, sh) = (c.screen.0 as f32, c.screen.1 as f32);
        self.modal_at(Rect::new((sw - w) / 2.0, (sh - h) / 2.0, w, h), c, s)
    }

    /// `modal` at `r`, for a card with a panel beside it.
    fn modal_at(&mut self, r: Rect, c: &mut Canvas, s: f32) -> Rect {
        let (sw, sh) = (c.screen.0 as f32, c.screen.1 as f32);
        c.rrect(Rect::new(0.0, 0.0, sw, sh), 0.0, [4, 6, 8, 150]);
        c.shadow(r, 16.0 * s, 30.0 * s, 8.0 * s, [0, 0, 0, 160]);
        c.panel(r, 16.0 * s, [30, 36, 44, 250], [18, 22, 27, 252], 1.0, EDGE);
        self.hits.push((Rect::new(0.0, 0.0, sw, sh), Hit::Panel));
        r
    }

    fn wide_button(&mut self, r: Rect, label: &str, hit: Hit, primary: bool, c: &mut Canvas, s: f32) {
        let hot = self.hover.is_some_and(|p| r.contains(p));
        let (t, b) = match (primary, hot) {
            (true, false) => ([70, 156, 222, 255], [50, 124, 186, 255]),
            (true, true) => ([92, 176, 240, 255], [62, 140, 204, 255]),
            (false, false) => (BUTTON_TOP, BUTTON_BOTTOM),
            (false, true) => (BUTTON_HOVER_TOP, BUTTON_HOVER_BOTTOM),
        };
        c.panel(r, 9.0 * s, t, b, 1.0, [255, 255, 255, 34]);
        c.text(Weight::SemiBold, 15.5 * s, (r.x + r.w / 2.0, r.y + r.h / 2.0 + 5.5 * s), Align::Centre, label, TEXT);
        self.hits.push((r, hit));
    }

    fn menu_draw(&mut self, page: MenuPage, m: &Model, c: &mut Canvas, s: f32) {
        match page {
            MenuPage::Main => {
                let w = 340.0 * s;
                let bh = 42.0 * s;
                // Experiments (2026-11-19): the declared switches, set
                // from play; the button says how many are on, and Restart
                // how many picks it will play.
                let on = m.experiments.iter().filter(|e| e.at != e.default).count();
                let waiting = m.experiments.iter().filter(|e| e.next.is_some_and(|k| k != e.at)).count();
                let exp = match on {
                    0 => "Experiments".to_string(),
                    n => format!("Experiments  ·  {n} on"),
                };
                let restart = match waiting {
                    // In a run Restart lands this planet again, fresh.
                    0 if m.pause_run.is_some() => "Restart planet".to_string(),
                    0 => "Restart".to_string(),
                    1 => "Restart with the new pick".to_string(),
                    _ => "Restart with the new picks".to_string(),
                };
                let mut items: Vec<(String, Hit, bool)> = vec![("Resume".into(), Hit::Menu(MenuItem::Resume), true), ("Controls".into(), Hit::Help, false)];
                if !m.experiments.is_empty() {
                    items.push((exp, Hit::ExpPage, false));
                }
                items.push((restart, Hit::Menu(MenuItem::Restart), false));
                if m.main_menu {
                    items.push(("Main menu".into(), Hit::Menu(MenuItem::MainMenu), false));
                }
                items.push(("Quit".into(), Hit::Menu(MenuItem::Quit), false));
                let w = if m.pause_run.is_some() { 400.0 * s } else { w };
                // The run: the planet's line, its strip and the tally.
                let run_h = if m.pause_run.is_some() { 92.0 * s } else { 0.0 };
                // The planet (Direction 11-24): its goal, its slime's line
                // and its twist, each a small heading over its words, so
                // they can be read again mid-planet without leaving it. A
                // panel beside the card where the screen is wide enough,
                // else in the card under the run where it is tall enough
                // (a card of both was 700 units, over a 1080-line screen
                // at 200%), else left out.
                let pw = 380.0 * s;
                let (sw, sh) = (c.screen.0 as f32, c.screen.1 as f32);
                let side = sw >= w + 16.0 * s + pw + 32.0 * s;
                let tw = if side { pw } else { w } - 48.0 * s;
                let (px, lead) = (13.5 * s, 18.0 * s);
                let mut parts: Vec<(String, Rgba, Vec<String>)> = Vec::new();
                if let Some(g) = m.objective.as_ref().filter(|_| m.pause_planet.is_some()) {
                    parts.push(("THE GOAL".into(), MUTED, c.wrap(Weight::Medium, px, tw, g)));
                }
                if let Some(i) = &m.pause_planet {
                    parts.push((i.title.to_uppercase(), WARN, c.wrap(Weight::Medium, px, tw, &i.line)));
                    if let Some((name, line)) = &i.twist {
                        parts.push((name.to_uppercase(), TWIST, c.wrap(Weight::Medium, px, tw, line)));
                    }
                }
                let part_h = |p: &(String, Rgba, Vec<String>)| 24.0 * s + p.2.len() as f32 * lead + 8.0 * s;
                let plan_h = if parts.is_empty() { 0.0 } else { 14.0 * s + parts.iter().map(part_h).sum::<f32>() };
                let card_h = |run_h: f32| {
                    let note = m.pause_note.len() as f32 * 18.0 * s + run_h;
                    84.0 * s + note + if note > 0.0 { 8.0 * s } else { 0.0 } + items.len() as f32 * (bh + 10.0 * s) + 14.0 * s
                };
                let inside = !side && card_h(run_h + plan_h) <= sh - 32.0 * s;
                if !side && !inside {
                    parts.clear();
                }
                let (plan_h, side_h) = if inside { (plan_h, 0.0) } else { (0.0, plan_h) };
                let run_h = run_h + plan_h;
                let note = m.pause_note.len() as f32 * 18.0 * s + run_h;
                let h = card_h(run_h);
                let r = if side && !parts.is_empty() {
                    let all = w + 16.0 * s + pw;
                    self.modal_at(Rect::new((sw - all) / 2.0, (sh - h) / 2.0, w, h), c, s)
                } else {
                    self.modal(w, h, c, s)
                };
                let mut tip = None;
                if let Some(v) = &m.pause_run {
                    let n = v.stops.len();
                    let here = v.stops.get(v.depth.saturating_sub(1) as usize);
                    let title = match here {
                        Some(p) => format!("Planet {} of {n}, {}: {}", v.depth, p.name, p.slime),
                        None => format!("Planet {} of {n}", v.depth),
                    };
                    c.text_fit(Weight::SemiBold, 14.5 * s, (r.x + w / 2.0, r.y + 96.0 * s), Align::Centre, &title, w - 40.0 * s, TEXT);
                    let sr = Rect::new(r.x + 24.0 * s, r.y + 106.0 * s, w - 48.0 * s, 30.0 * s);
                    tip = super::start::run_strip(c, &v.stops, sr, self.hover, s);
                    let st = v.stars();
                    let mut tally = format!("{} freed, {st} star{}, {} points", v.freed(), if st == 1 { "" } else { "s" }, v.points);
                    if let Some(t) = v.played {
                        tally += &format!(", played {}", super::start::RunView::clock(t));
                    }
                    c.text_fit(Weight::Medium, 13.0 * s, (r.x + w / 2.0, r.y + 158.0 * s), Align::Centre, &tally, w - 40.0 * s, MUTED);
                }
                if !parts.is_empty() {
                    // Beside: its own panel, level with the card's top.
                    let (cx, mut y) = if inside {
                        let y = r.y + 84.0 * s + run_h - plan_h;
                        c.rrect(Rect::new(r.x + w / 2.0 - 40.0 * s, y + 6.0 * s, 80.0 * s, 1.0 * s), 0.5 * s, EDGE);
                        (r.x + w / 2.0, y + 14.0 * s)
                    } else {
                        let pr = Rect::new(r.right() + 16.0 * s, r.y, pw, 6.0 * s + side_h);
                        c.shadow(pr, 16.0 * s, 30.0 * s, 8.0 * s, [0, 0, 0, 160]);
                        c.panel(pr, 16.0 * s, [30, 36, 44, 250], [18, 22, 27, 252], 1.0, EDGE);
                        c.rrect(Rect::new(pr.x + 24.0 * s, pr.y, pw - 48.0 * s, 3.0 * s), 1.5 * s, alpha(WARN, 0.9));
                        (pr.x + pw / 2.0, pr.y + 10.0 * s)
                    };
                    for p in &parts {
                        c.text_fit(Weight::SemiBold, 12.5 * s, (cx, y + 16.0 * s), Align::Centre, &p.0, tw, p.1);
                        for (k, l) in p.2.iter().enumerate() {
                            c.text(Weight::Medium, px, (cx, y + 36.0 * s + k as f32 * lead), Align::Centre, l, TEXT);
                        }
                        y += part_h(p);
                    }
                }
                c.text(Weight::SemiBold, 22.0 * s, (r.x + w / 2.0, r.y + 44.0 * s), Align::Centre, "Paused", TEXT);
                c.text(Weight::Medium, 14.0 * s, (r.x + w / 2.0, r.y + 65.0 * s), Align::Centre, "Esc to resume", MUTED);
                for (k, l) in m.pause_note.iter().enumerate() {
                    c.text_fit(Weight::Medium, 13.0 * s, (r.x + w / 2.0, r.y + 86.0 * s + run_h + k as f32 * 18.0 * s), Align::Centre, l, w - 32.0 * s, MUTED);
                }
                let mut y = r.y + 84.0 * s + note + if note > 0.0 { 8.0 * s } else { 0.0 };
                for (label, hit, primary) in items {
                    let b = Rect::new(r.x + 24.0 * s, y, w - 48.0 * s, bh);
                    self.wide_button(b, &label, hit, primary, c, s);
                    // An experiment on, or a pick waiting: an amber dot at
                    // the button's left, the HUD mark's colour.
                    if (hit == Hit::ExpPage && on > 0) || (hit == Hit::Menu(MenuItem::Restart) && waiting > 0) {
                        c.rrect(Rect::new(b.x + 16.0 * s, b.y + b.h / 2.0 - 4.0 * s, 8.0 * s, 8.0 * s), 4.0 * s, WARN);
                    }
                    y += bh + 10.0 * s;
                }
                if let Some((at, t)) = tip {
                    super::start::strip_tip(c, at, &t, s);
                }
            }
            MenuPage::Experiments => self.experiments_draw(m, c, s),
            MenuPage::Help => {
                let mut rows: Vec<(String, String)> = if let Some(base) = &m.help_base {
                    base.clone()
                } else { vec![
                    ("Left click / drag".into(), "Select soldiers".into()),
                    ("Right click".into(), "Move, or work the crystal or slime clicked".into()),
                    ("Shift".into(), "Add to the selection, or keep placing".into()),
                    ("Ctrl + 1-9".into(), "Make a group; 1-9 selects it, twice to look".into()),
                    ("Wheel, middle drag".into(), "Zoom, pan; the screen's edge scrolls".into()),
                    ("Minimap".into(), "Click to look there; right click to send".into()),
                    ("Space".into(), "Look at the newest alert".into()),
                ] };
                // Every game's card ends the same.
                if rows.iter().any(|r| r.1.is_empty()) {
                    rows.push(("The game".into(), String::new()));
                }
                rows.push(("Esc".into(), "Cancel placing, or this menu".into()));
                rows.push(("F1 or ?".into(), "This key card".into()));
                rows.push(("Ctrl + / Ctrl -".into(), "Bigger or smaller UI and text".into()));
                if rows.iter().any(|r| r.1.is_empty()) && m.commands.iter().any(|cmd| key_name(cmd.key).is_some()) {
                    rows.push(("The card now".into(), String::new()));
                }
                for cmd in &m.commands {
                    if let Some(k) = key_name(cmd.key) {
                        rows.push((k, cmd.name.clone()));
                    }
                }
                rows.extend(m.help.iter().cloned());
                // In columns (up to three) where one runs off the screen, and
                // smaller only past that: one 620-unit column ran off the
                // top and bottom of a 1080-line screen at 200% (11-15), the
                // squad mode's card having some forty rows.
                let (sw, sh) = (c.screen.0 as f32, c.screen.1 as f32);
                let n = rows.len().max(1);
                let mut s = s;
                let (cols, per) = loop {
                    let per = (((sh - 40.0 * s - 150.0 * s) / (25.0 * s)).floor() as usize).max(1);
                    let cols = n.div_ceil(per);
                    if (cols <= 3 && cols as f32 * 600.0 * s + 20.0 * s <= sw - 40.0 * s) || s < 0.4 {
                        break (cols.max(1), n.div_ceil(cols.max(1)));
                    }
                    s *= 0.92;
                };
                let col_w = 600.0 * s;
                let w = cols as f32 * col_w + 20.0 * s;
                let lh = 25.0 * s;
                let h = 80.0 * s + per as f32 * lh + 70.0 * s;
                let r = self.modal(w, h, c, s);
                c.text(Weight::SemiBold, 20.0 * s, (r.x + 28.0 * s, r.y + 42.0 * s), Align::Left, "Controls", TEXT);
                for (i, (k, what)) in rows.iter().enumerate() {
                    let (col, row) = (i / per, i % per);
                    let x = r.x + 28.0 * s + col as f32 * col_w;
                    let y = r.y + 76.0 * s + row as f32 * lh;
                    if what.is_empty() {
                        // A group's heading (`squad_help`; polish review
                        // finding 11, 11-24).
                        c.text(Weight::SemiBold, 12.5 * s, (x, y), Align::Left, &k.to_uppercase(), MUTED);
                        c.rrect(Rect::new(x, y + 6.0 * s, col_w - 40.0 * s, s.max(1.0)), 0.0, [255, 255, 255, 22]);
                        continue;
                    }
                    c.text_fit(Weight::SemiBold, 14.5 * s, (x, y), Align::Left, k, 174.0 * s, ACCENT);
                    c.text_fit(Weight::Medium, 14.5 * s, (x + 182.0 * s, y), Align::Left, what, col_w - 210.0 * s, TEXT);
                }
                self.wide_button(Rect::new(r.x + w - 24.0 * s - 120.0 * s, r.bottom() - 24.0 * s - 38.0 * s, 120.0 * s, 38.0 * s), "Back", Hit::Back, true, c, s);
            }
        }
    }

    /// **The pause menu's Experiments page** (2026-11-19): every declared
    /// switch as the start menu's card, its choices clickable, the count
    /// against the plan's limit over them, and Restart when a pick waits
    /// for one. A card too tall for the screen (at 200% on 1080 lines
    /// with three switches) draws the page smaller to fit, as the key card
    /// does.
    fn experiments_draw(&mut self, m: &Model, c: &mut Canvas, s: f32) {
        let (sw, sh) = (c.screen.0 as f32, c.screen.1 as f32);
        let waiting: Vec<&Experiment> = m.experiments.iter().filter(|e| e.next.is_some_and(|k| k != e.at)).collect();
        let mut s = s;
        let (w, cards, h) = loop {
            let w = (sw - 48.0 * s).min(820.0 * s);
            let cards: Vec<f32> = m.experiments.iter().map(|e| experiment_card_h(c, w - 48.0 * s, e, s)).collect();
            let h = (96.0 + 58.0 + 24.0) * s + cards.iter().map(|ch| ch + 10.0 * s).sum::<f32>();
            if h <= sh - 24.0 * s || s < 0.4 {
                break (w, cards, h);
            }
            s *= 0.92;
        };
        let r = self.modal(w, h, c, s);
        let px = r.x + 24.0 * s;
        c.text(Weight::SemiBold, 22.0 * s, (px, r.y + 42.0 * s), Align::Left, "Experiments", TEXT);
        c.text_fit(Weight::Medium, 14.0 * s, (px, r.y + 66.0 * s), Align::Left, "Built, but not the default game yet. Each is settled once you have played it", w - 48.0 * s, MUTED);
        let on = m.experiments.iter().filter(|e| e.at != e.default).count();
        experiment_meter(c, (px, r.y + 88.0 * s), m.experiments.len(), on, m.most_off, s);
        let mut y = r.y + 104.0 * s;
        for (row, (e, ch)) in m.experiments.iter().zip(&cards).enumerate() {
            let card = Rect::new(px, y, w - 48.0 * s, *ch);
            for (b, k) in experiment_card(c, card, e, self.hover, false, true, s) {
                self.hits.push((b, Hit::Exp(row, k)));
            }
            y += ch + 10.0 * s;
        }
        // The foot: what a Restart would play, and the buttons.
        let bh = 38.0 * s;
        let by = r.bottom() - 20.0 * s - bh;
        let back = Rect::new(r.right() - 24.0 * s - 110.0 * s, by, 110.0 * s, bh);
        let mut left = back.x - 16.0 * s;
        if !waiting.is_empty() {
            let b = Rect::new(back.x - 12.0 * s - 150.0 * s, by, 150.0 * s, bh);
            self.wide_button(b, "Restart now", Hit::Menu(MenuItem::Restart), true, c, s);
            left = b.x - 16.0 * s;
        }
        let note = if waiting.is_empty() {
            let live: Vec<&str> = m.experiments.iter().filter(|e| e.info.live).map(|e| e.label.as_str()).collect();
            match live.len() {
                0 => "A pick here plays from the next round. Picks last until the game closes".to_string(),
                n if n == m.experiments.len() => "A pick here applies at once. Picks last until the game closes".to_string(),
                _ => format!("{} changes at once, the rest from the next round. Picks last until the game closes", live.join(" and ")),
            }
        } else {
            format!("Waiting for a restart: {}", waiting.iter().map(|e| format!("{} {}", e.label, e.choices[e.picked()])).collect::<Vec<_>>().join(", "))
        };
        c.text_fit(Weight::Medium, 13.5 * s, (px, by + bh / 2.0 + 5.0 * s), Align::Left, &note, left - px, if waiting.is_empty() { MUTED } else { WARN });
        self.wide_button(back, "Back", Hit::Back, false, c, s);
    }

    /// The end screen: a card over the dimmed world. Where, in the
    /// outcome's colour; the title; a mission's stars; what the slime
    /// did; the round's numbers as tiles, three to a row (a value big,
    /// what it counts small under it); how it ended; then the buttons.
    /// Before 2026-11-18 it was a 420-wide card of a title and grey lines.
    /// A card taller than the screen (at 200% on 1080 lines) is drawn
    /// smaller to fit.
    fn end_screen(&mut self, e: &Ending, main_menu: bool, carry_on: Option<&str>, c: &mut Canvas, s: f32) {
        let buttons = if main_menu { 3.0 } else { 2.0 };
        let cols = 3usize;
        let rows = e.tiles.len().div_ceil(cols);
        let stars = e.stars.filter(|&n| n > 0);
        // Its height at size `s`: the title and the slime's sentence in
        // lines as wrapped at that size.
        let lay = |c: &Canvas, s: f32| {
            let w = (560.0 * s).min(c.screen.0 as f32 - 32.0);
            let inner = w - 56.0 * s;
            let title = c.wrap(Weight::SemiBold, 28.0 * s, inner, &e.title);
            let says = if e.says.is_empty() { Vec::new() } else { c.wrap(Weight::Medium, 15.0 * s, inner, &e.says) };
            let tiles_h = if rows == 0 { 0.0 } else { rows as f32 * 64.0 * s + (rows - 1) as f32 * 10.0 * s + 20.0 * s };
            let graph_h = if e.curve.len() >= 2 { CURVE_H * s + 14.0 * s } else { 0.0 };
            let h = 28.0 * s
                + graph_h
                + if e.place.is_empty() { 0.0 } else { 22.0 * s }
                + title.len() as f32 * 34.0 * s
                + if stars.is_some() { 34.0 * s } else { 0.0 }
                + says.len() as f32 * 21.0 * s
                + 14.0 * s
                + tiles_h
                + e.lines.len() as f32 * 21.0 * s
                + 12.0 * s
                + buttons * 52.0 * s
                + 18.0 * s;
            (w, h, title, says)
        };
        let room = c.screen.1 as f32 - 32.0;
        let (_, h0, ..) = lay(c, s);
        let s = if h0 > room { s * room / h0 } else { s };
        let (w, h, title, says) = lay(c, s);
        let pad = 28.0 * s;
        let inner = w - 2.0 * pad;
        let bh = 42.0 * s;
        let r = self.modal(w, h, c, s);
        let tone = match e.won {
            Some(true) => GOOD,
            Some(false) => BAD,
            None => ACCENT,
        };
        // The outcome's colour along the card's top edge.
        c.rrect(Rect::new(r.x + 16.0 * s, r.y, w - 32.0 * s, 3.0 * s), 1.5 * s, tone);
        let mut y = r.y + pad;
        if !e.place.is_empty() {
            c.text_fit(Weight::SemiBold, 13.5 * s, (r.x + w / 2.0, y + 10.0 * s), Align::Centre, &e.place.to_uppercase(), inner, tone);
            y += 22.0 * s;
        }
        for l in &title {
            c.text(Weight::SemiBold, 28.0 * s, (r.x + w / 2.0, y + 24.0 * s), Align::Centre, l, TEXT);
            y += 34.0 * s;
        }
        if let Some(n) = stars {
            // Stars, the middle one raised and larger (Look D, 11-20: three
            // 11-unit discs read as loading dots, not a score).
            let (d, gap) = (26.0 * s, 4.0 * s);
            let x0 = r.x + w / 2.0 - (3.0 * d + 2.0 * gap) / 2.0;
            for k in 0..3u8 {
                let big = if k == 1 { 1.15 } else { 1.0 };
                let px = d * big;
                let at = (x0 + k as f32 * (d + gap) + (d - px) / 2.0, y + 2.0 * s + if k == 1 { -3.0 * s } else { 0.0 });
                if k < n {
                    c.icon(Icon::Star, at, px, tone);
                } else {
                    c.icon(Icon::StarOpen, at, px, [255, 255, 255, 70]);
                }
            }
            y += 34.0 * s;
        }
        for l in &says {
            y += 21.0 * s;
            c.text(Weight::Medium, 15.0 * s, (r.x + w / 2.0, y), Align::Centre, l, MUTED);
        }
        y += 14.0 * s;
        if rows > 0 {
            y += 6.0 * s;
            let (tile_h, gap) = (64.0 * s, 10.0 * s);
            let tw = (inner - (cols - 1) as f32 * gap) / cols as f32;
            for (k, (value, what)) in e.tiles.iter().enumerate() {
                let (col, row) = (k % cols, k / cols);
                // A short last row sits in the middle.
                let in_row = (e.tiles.len() - row * cols).min(cols);
                let x0 = r.x + pad + (inner - (in_row as f32 * tw + (in_row - 1) as f32 * gap)) / 2.0;
                let t = Rect::new(x0 + col as f32 * (tw + gap), y + row as f32 * (tile_h + gap), tw, tile_h);
                c.panel(t, 10.0 * s, [255, 255, 255, 12], [255, 255, 255, 6], 1.0, EDGE);
                c.text_fit(Weight::SemiBold, 23.0 * s, (t.x + tw / 2.0, t.y + 30.0 * s), Align::Centre, value, tw - 16.0 * s, TEXT);
                c.text_fit(Weight::Medium, 13.0 * s, (t.x + tw / 2.0, t.y + 51.0 * s), Align::Centre, what, tw - 12.0 * s, MUTED);
            }
            y += rows as f32 * tile_h + (rows - 1) as f32 * gap + 14.0 * s;
        }
        if e.curve.len() >= 2 {
            end_curve(&e.curve, Rect::new(r.x + pad, y, inner, CURVE_H * s), c, s);
            y += CURVE_H * s + 14.0 * s;
        }
        for l in &e.lines {
            y += 21.0 * s;
            c.text_fit(Weight::Medium, 14.0 * s, (r.x + w / 2.0, y - 4.0 * s), Align::Centre, l, inner, MUTED);
        }
        y += 12.0 * s;
        match carry_on {
            Some(label) => self.wide_button(Rect::new(r.x + pad, y, inner, bh), label, Hit::Menu(MenuItem::Continue), true, c, s),
            None => self.wide_button(Rect::new(r.x + pad, y, inner, bh), "Play again", Hit::Menu(MenuItem::Restart), true, c, s),
        }
        // Its key at the right, as the menus' Back shows Esc (Look D,
        // 11-24: Enter presses it since the time between planets, #812,
        // and nothing on the card said so).
        c.text(Weight::Medium, 13.0 * s, (r.x + pad + inner - 16.0 * s, y + bh / 2.0 + 5.0 * s), Align::Right, "Enter", [220, 236, 250, 200]);
        let mut y = y + bh + 10.0 * s;
        if main_menu {
            self.wide_button(Rect::new(r.x + pad, y, inner, bh), "Main menu", Hit::Menu(MenuItem::MainMenu), false, c, s);
            y += bh + 10.0 * s;
        }
        self.wide_button(Rect::new(r.x + pad, y, inner, bh), "Quit", Hit::Menu(MenuItem::Quit), false, c, s);
    }

    /// Where a command's button was last drawn.
    pub fn command_rect(&self, id: u32) -> Option<Rect> {
        let k = self.commands.iter().position(|c| c.id == id)?;
        self.hits.iter().find(|(_, h)| *h == Hit::Command(k)).map(|(r, _)| *r)
    }

    /// The HUD's insets on a screen this size, top and bottom pixels (the
    /// top bar; the bottom panels' height), as `draw` lays them out: for a
    /// camera made before the HUD has drawn, so it can centre in the band
    /// the panels leave (the opening view on the squad).
    pub fn insets(screen: (u32, u32), ui: UiSize) -> (f32, f32) {
        let s = ui.px(screen);
        let b = s * BAR;
        let panel_h = CARD_ROWS as f32 * CELL * b + (CARD_ROWS - 1) as f32 * CELL_GAP * b + 2.0 * CARD_PAD * b;
        (8.0 * s + 38.0 * s, panel_h + 8.0 * b)
    }

    /// Opens the menu on its controls page.
    pub fn show_help(&mut self) {
        self.menu = Some(MenuPage::Help);
        self.help_direct = false;
    }

    /// Opens the menu on its Experiments page (its button's click; the
    /// films).
    pub fn show_experiments(&mut self) {
        self.menu = Some(MenuPage::Experiments);
    }

    /// Seconds since the HUD was made (for UI animations).
    pub fn age(&self) -> f32 {
        self.started.elapsed().as_secs_f32()
    }
}

/// The end screen's graph's height, UI units.
const CURVE_H: f32 = 112.0;
/// The slime's colour on the end screen's graph: the amber the slime is
/// drawn in on the planet, so the bars read as the slime.
const CURVE_SLIME: Rgba = [255, 178, 64, 255];
/// The area under the slime's curve: its amber, dim.
const CURVE_FILL: Rgba = [236, 150, 52, 90];

/// **The match's curve on the end screen** (2026-11-21, the owner: "I
/// would be curious to see slime population overtime after each match"):
/// the slime's size as an amber area with a bright edge, one column per
/// few seconds (the most in each, so a peak is never averaged away), and
/// the soldiers standing as a cyan step line on their own scale, with the
/// slime's peak and the clock at both ends. Plain rectangles, as the rest
/// of the card: the canvas draws no lines (gapped bars and dots to 11-24).
fn end_curve(curve: &[(u32, u32)], r: Rect, c: &mut Canvas, s: f32) {
    c.panel(r, 10.0 * s, [255, 255, 255, 12], [255, 255, 255, 6], 1.0, EDGE);
    let (padx, top, bottom) = (12.0 * s, 26.0 * s, 20.0 * s);
    let plot = Rect::new(r.x + padx, r.y + top, r.w - 2.0 * padx, r.h - top - bottom);
    let peak = curve.iter().map(|p| p.0).max().unwrap_or(0).max(1) as f32;
    let most = curve.iter().map(|p| p.1).max().unwrap_or(0).max(1) as f32;
    // The key, left; the peak, right.
    let ky = r.y + 15.0 * s;
    c.rrect(Rect::new(r.x + padx, ky - 5.0 * s, 9.0 * s, 9.0 * s), 2.0 * s, CURVE_SLIME);
    let w = c.text(Weight::Medium, 12.5 * s, (r.x + padx + 14.0 * s, ky + 4.0 * s), Align::Left, "Slime", MUTED);
    let kx = r.x + padx + 14.0 * s + w + 14.0 * s;
    c.rrect(Rect::new(kx, ky - 1.0 * s, 10.0 * s, 2.0 * s), 0.0, ACCENT);
    c.text(Weight::Medium, 12.5 * s, (kx + 15.0 * s, ky + 4.0 * s), Align::Left, &format!("Soldiers (most {})", most as u32), MUTED);
    let peak_label = if peak >= 10_000.0 { format!("peak {:.1}k", peak / 1000.0) } else { format!("peak {}", peak as u32) };
    c.text(Weight::Medium, 12.5 * s, (r.right() - padx, ky + 4.0 * s), Align::Right, &peak_label, MUTED);
    // The floor.
    c.rrect(Rect::new(plot.x, plot.bottom(), plot.w, 1.0 * s), 0.0, TRACK);
    // A faint line at half the peak, so the area's height reads.
    c.rrect(Rect::new(plot.x, plot.y + plot.h / 2.0, plot.w, 1.0 * s), 0.0, [255, 255, 255, 10]);
    // The slime as a filled area (the polish review, 11-24, finding 15: a
    // row of gapped amber bars all alike read as a placeholder): a column
    // every 2 units or more with no gap, each the most of its seconds, its
    // body a dim amber and a bright cap along the top, so the columns'
    // tops join into the curve's edge. The soldiers as a cyan step line
    // over it, each column's level joined to the next by an upright.
    let n = curve.len().min((plot.w / (2.0 * s)).floor().max(1.0) as usize);
    let cw = plot.w / n as f32;
    let cap = 2.0 * s;
    let lw = 2.0 * s;
    let mut prev: Option<f32> = None;
    for k in 0..n {
        let (a, b) = (k * curve.len() / n, ((k + 1) * curve.len() / n).max(k * curve.len() / n + 1));
        let part = &curve[a..b.min(curve.len())];
        let v = part.iter().map(|p| p.0).max().unwrap_or(0) as f32;
        // Whole pixels, edge to edge: an overlap doubled the dim fill
        // into seams, a gap into stripes.
        let x = (plot.x + k as f32 * cw).round();
        let w = (plot.x + (k + 1) as f32 * cw).round() - x;
        if v > 0.0 {
            let h = (v / peak * plot.h).max(cap);
            c.rrect(Rect::new(x, plot.bottom() - h, w, h), 0.0, CURVE_FILL);
            c.rrect(Rect::new(x, plot.bottom() - h, w, cap), 0.0, CURVE_SLIME);
        }
        let men = part.last().map_or(0, |p| p.1) as f32;
        let my = plot.bottom() - men / most * (plot.h - lw) - lw;
        c.rrect(Rect::new(x, my, w, lw), 0.0, ACCENT);
        if let Some(py) = prev.filter(|py| (py - my).abs() > 0.5) {
            c.rrect(Rect::new(x, py.min(my), lw, (py - my).abs() + lw), 0.0, ACCENT);
        }
        prev = Some(my);
    }
    // The clock under it, from the landing to the end.
    let secs = curve.len() as u32;
    let by = r.bottom() - 6.0 * s;
    c.text(Weight::Medium, 11.5 * s, (plot.x, by), Align::Left, "0:00", FAINT);
    c.text(Weight::Medium, 11.5 * s, (plot.x + plot.w / 2.0, by), Align::Centre, &format!("{}:{:02}", secs / 2 / 60, secs / 2 % 60), FAINT);
    c.text(Weight::Medium, 11.5 * s, (plot.right(), by), Align::Right, &format!("{}:{:02}", secs / 60, secs % 60), FAINT);
}

/// A command's name as its button shows it: the little words dropped
/// ("Harvest a crystal" reads "Harvest crystal"); the tooltip keeps the
/// whole name.
pub fn short_label(name: &str) -> String {
    name.split(' ').filter(|w| !matches!(*w, "a" | "an" | "the" | "your")).collect::<Vec<_>>().join(" ")
}

/// A label cut in two for a two-line button: at the last space (or
/// hyphen) whose first part fits `room`.
fn split_label(c: &Canvas, label: &str, px: f32, room: f32) -> (String, String) {
    let cuts: Vec<usize> = label.char_indices().filter(|&(_, ch)| ch == ' ' || ch == '-').map(|(i, _)| i).collect();
    for &i in cuts.iter().rev() {
        // A hyphen is cut without its dash ("Attack" over "move": the
        // polish review, 11-24, read "Attack-" over "move" as a broken word).
        let keep = i;
        let first = &label[..keep];
        if c.atlas.measure(Weight::SemiBold, px, first) <= room {
            return (first.to_string(), label[i + 1..].trim().to_string());
        }
    }
    match cuts.first() {
        Some(&i) => (label[..i].to_string(), label[i + 1..].trim().to_string()),
        None => (label.to_string(), String::new()),
    }
}

/// Text size from `px` down to `least` that fits `max` wide.
fn fit_px(c: &Canvas, text: &str, px: f32, least: f32, max: f32) -> f32 {
    let w = c.atlas.measure(Weight::SemiBold, px, text);
    if w <= max {
        px
    } else {
        (px * max / w).max(least)
    }
}

/// **A key's name on a small keycap** (the polish review, 2026-11-24,
/// finding 6: bare 11 px letters in the buttons' corners read as debug
/// text, not as keys). A dark plate with a faint rim and the name centred
/// on it, its bottom-right corner at `at`. Returns its width.
fn keycap(c: &mut Canvas, k: &str, at: (f32, f32), px: f32, a: f32, s: f32) -> f32 {
    let h = px + 5.0 * s;
    let w = (c.atlas.measure(Weight::SemiBold, px, k) + 8.0 * s).max(h);
    let r = Rect::new(at.0 - w, at.1 - h, w, h);
    c.rrect(r, 3.5 * s, [8, 11, 15, (150.0 * a) as u8]);
    c.outline(r, 3.5 * s, 1.0, [255, 255, 255, (46.0 * a) as u8]);
    c.text(Weight::SemiBold, px, (r.x + w / 2.0, r.y + h / 2.0 + 0.36 * px), Align::Centre, k, alpha(TEXT, 0.82 * a));
    w
}

/// The keycap's width, as `keycap` draws it.
fn keycap_w(c: &Canvas, k: &str, px: f32, s: f32) -> f32 {
    (c.atlas.measure(Weight::SemiBold, px, k) + 8.0 * s).max(px + 5.0 * s)
}

/// A key's short name for the button's corner: the key now pressed for
/// `k` (`keys`: a rebound key shows its new letter wherever the game
/// names it).
pub fn key_name(k: KeyCode) -> Option<String> {
    raw_key_name(super::keys::pressed(k))
}

/// The key's own name, rebinding aside.
pub fn raw_key_name(k: KeyCode) -> Option<String> {
    use KeyCode::*;
    let s = match k {
        KeyA => "A",
        KeyB => "B",
        KeyC => "C",
        KeyD => "D",
        KeyE => "E",
        KeyF => "F",
        KeyG => "G",
        KeyH => "H",
        KeyI => "I",
        KeyJ => "J",
        KeyK => "K",
        KeyL => "L",
        KeyM => "M",
        KeyN => "N",
        KeyO => "O",
        KeyP => "P",
        KeyQ => "Q",
        KeyR => "R",
        KeyS => "S",
        KeyT => "T",
        KeyU => "U",
        KeyV => "V",
        KeyW => "W",
        KeyX => "X",
        KeyY => "Y",
        KeyZ => "Z",
        Tab => "Tab",
        Backquote => "`",
        // The squad's spread keys (2026-11-23); without them the key card
        // read ", " for "[, ]" (polish review, 2026-11-24).
        BracketLeft => "[",
        BracketRight => "]",
        Space => "Space",
        Delete => "Del",
        _ => return None,
    };
    Some(s.into())
}

#[cfg(test)]
mod tests {
    use super::*;

    const POINT: u32 = 1;
    const WALL: u32 = 2;
    const NOW: u32 = 3;

    struct Rig {
        hud: Hud,
        c: Canvas,
        cam: Camera,
        k: Controls,
        m: Model,
        marks: Marks,
    }

    impl Rig {
        fn new(screen: (u32, u32)) -> Self {
            let m = Model {
                commands: vec![
                    Command { target: Target::Point { radius: 4.0 }, ..Command::new(POINT, (0, 0), Icon::Attack, "Point", KeyCode::KeyA) },
                    Command { target: Target::Line { width: 2.0 }, ..Command::new(WALL, (2, 2), Icon::Wall, "Wall", KeyCode::KeyV) },
                    Command::new(NOW, (0, 1), Icon::Stop, "Now", KeyCode::KeyS),
                ],
                ghost_ok: true,
                ..Model::default()
            };
            let mut cam = Camera::new((2048, 1280), screen);
            cam.zoom_to(2.0, (screen.0 as f32 / 2.0, screen.1 as f32 / 2.0));
            let mut r = Rig { hud: Hud::new(), c: Canvas::new(screen), cam, k: Controls::new(0), m, marks: Marks::default() };
            r.draw();
            r
        }

        fn draw(&mut self) {
            self.c.clear();
            self.marks.clear();
            self.hud.draw(&self.m, &self.k, &self.cam, true, &mut self.c, &mut self.marks);
        }

        fn centre(&self, id: u32) -> (f32, f32) {
            let r = self.hud.command_rect(id).unwrap();
            (r.x + r.w / 2.0, r.y + r.h / 2.0)
        }

        fn at(&mut self, p: (f32, f32)) {
            self.hud.moved(Some(p));
        }

        fn click(&mut self, b: Button, pressed: bool) -> (bool, Vec<HudEvent>) {
            self.hud.button(b, pressed, &self.cam, false)
        }

        fn key(&mut self, code: KeyCode) -> (bool, Vec<HudEvent>) {
            self.hud.key(code, true, false, &mut self.cam)
        }
    }

    const WORLD_SPOT: (f32, f32) = (700.0, 400.0);

    fn loadout() -> Loadout {
        let card = |name: &str, kind: CardKind, next: bool, idle: bool| GunCard { icon: Some(Icon::Blast), badge: None, name: name.into(), kind, colour: [255, 150, 60, 255], about: format!("{name} does a thing to the mould where the shot lands, at length."), next, idle };
        Loadout {
            name: "Lull".into(),
            whose: "Soldier 1".into(),
            cards: vec![card("Quiet", CardKind::Modifier, false, false), card("Gas", CardKind::Payload, false, false), card("Quiet", CardKind::Modifier, true, false), card("Blast", CardKind::Payload, true, false)],
            slots: 4,
            note: None,
            cycle: Some((NOW + 10, KeyCode::KeyH)),
            gun: None,
            widest: 0,
        }
    }

    #[test]
    fn the_loadout_swap_button_gives_its_command_and_a_card_takes_the_click() {
        let mut r = Rig::new((1920, 1080));
        r.m.loadout = Some(loadout());
        r.draw();
        let swap = r.hud.hits.iter().find(|(_, h)| *h == Hit::Cycle).map(|(b, _)| *b).expect("a swap button");
        r.at((swap.x + swap.w / 2.0, swap.y + swap.h / 2.0));
        assert!(r.click(Button::Left, true).0);
        assert_eq!(r.click(Button::Left, false), (true, vec![HudEvent::Command { id: NOW + 10 }]));
        // A card is the panel's: hovered it draws its tooltip, a click on
        // it is not a click on the world.
        let strip = r.hud.hits.iter().find(|(b, h)| *h == Hit::Panel && b.contains((swap.x, swap.y))).map(|(b, _)| *b).expect("the strip");
        r.at((strip.x + 20.0 * r.hud.scale, swap.y + swap.h / 2.0));
        let before = r.c.len();
        r.draw();
        assert!(r.c.len() > before, "the card's tooltip adds to the canvas");
        assert!(r.click(Button::Left, true).0);
        assert_eq!(r.click(Button::Left, false), (true, vec![]));
    }

    /// Card 2 (owner 2026-11-15 19:38Z) and its sixth column (2026-11-21
    /// 06:21Z): six by two, a slot is its own place, and one past the edge
    /// stays on the card.
    #[test]
    fn the_command_card_is_six_by_two() {
        assert_eq!((CARD_COLS, CARD_ROWS), (6, 2));
        let places: Vec<(u8, u8)> = (0..2).flat_map(|r| (0..6).map(move |c| card_slot((r, c)))).collect();
        assert_eq!(places, (0..2).flat_map(|r| (0..6).map(move |c| (r, c))).collect::<Vec<_>>());
        assert_eq!(card_slot((2, 7)), (1, 5));
    }

    /// A button's other keys (the fire button's Z, X and C) give their own
    /// command, before any button's key; its click gives its own.
    #[test]
    fn a_buttons_other_keys_give_their_commands() {
        let mut r = Rig::new((1920, 1080));
        r.m.commands = vec![Command { also: vec![(KeyCode::KeyZ, 50), (KeyCode::KeyX, 51)], ..Command::new(NOW, (0, 0), Icon::Eye, "Fire", KeyCode::KeyX) }];
        r.draw();
        assert_eq!(r.key(KeyCode::KeyX), (true, vec![HudEvent::Command { id: 51 }]));
        assert_eq!(r.key(KeyCode::KeyZ), (true, vec![HudEvent::Command { id: 50 }]));
        let b = r.hud.command_rect(NOW).unwrap();
        r.at((b.x + b.w / 2.0, b.y + b.h / 2.0));
        r.draw();
        assert!(r.click(Button::Left, true).0);
        assert_eq!(r.click(Button::Left, false), (true, vec![HudEvent::Command { id: NOW }]));
    }

    /// The UI size (owner, 11-14 08:07Z): at every step on every screen the
    /// HUD's panels and buttons stay on the screen, and it grows with the
    /// size until `UiSize::px`'s cap.
    #[test]
    fn every_ui_size_keeps_the_hud_on_screen() {
        for screen in [(1280, 800), (1600, 900), (1920, 1080), (2560, 1440)] {
            let mut last = 0.0;
            for user in crate::world::settings::STEPS {
                let mut r = Rig::new(screen);
                r.c.ui.user = user;
                r.m.selection = vec![Card { id: 0, icon: Icon::Soldier, team: 0, hp: 1.0, badge: None }; 3];
                r.m.loadout = Some(loadout());
                r.draw();
                assert!(r.hud.scale >= last, "{screen:?} at {user}: {} after {last}", r.hud.scale);
                last = r.hud.scale;
                let (sw, sh) = (screen.0 as f32 + 0.5, screen.1 as f32 + 0.5);
                for (b, h) in &r.hud.hits {
                    assert!(b.x >= -0.5 && b.y >= -0.5 && b.right() <= sw && b.bottom() <= sh, "{screen:?} at {user}: {h:?} {b:?} off the screen");
                }
                assert!(screen.1 as f32 - r.hud.bottom <= screen.1 as f32 * 0.34, "{screen:?} at {user}: the bottom bar is too tall");
            }
            assert!(last > Rig::new(screen).hud.scale * 1.3, "{screen:?}: the biggest size grows the HUD");
        }
    }

    /// The squad header's buttons, with a gun loaded, at every UI size:
    /// none overlaps another and all stay in the squad panel, off the gun
    /// strip (11-15: at 150% the fifth, Heal, ran over "1 squad"; the fit
    /// had counted four. The header word itself is not in the hits: the
    /// code keeps buttons right of it, `floor`).
    /// The hover card's clock: it waits for the cursor to rest, shows,
    /// fades and stays shut for that thing; held buttons or steering keep
    /// it away, and what the aim reaches meanwhile stays shut too (the
    /// review, 11-15: aim sliding onto the home crystal mid-volley raised
    /// a card).
    #[test]
    fn the_hover_card_waits_fades_and_keeps_out_of_a_volley() {
        use std::time::Duration;
        let t0 = Instant::now();
        let at = |secs: f32| t0 + Duration::from_secs_f32(secs);
        let mut h = HoverCard::new();
        assert_eq!(h.alpha(Some("Crystal A"), at(0.0), false), 0.0, "not before the rest");
        assert_eq!(h.alpha(Some("Crystal A"), at(0.2), false), 0.0);
        assert_eq!(h.alpha(Some("Crystal A"), at(0.4), false), 1.0, "up after 0.35 s");
        let fading = h.alpha(Some("Crystal A"), at(0.4 + HoverCard::SHOW + 0.2), false);
        assert!(fading > 0.0 && fading < 1.0, "fading: {fading}");
        assert_eq!(h.alpha(Some("Crystal A"), at(3.0), false), 0.0, "gone after 2 s and the fade");
        assert_eq!(h.alpha(Some("Crystal A"), at(9.0), false), 0.0, "and stays gone for it");
        assert_eq!(h.alpha(Some("Crystal B"), at(9.0), false), 0.0, "something else waits its rest");
        assert_eq!(h.alpha(Some("Crystal B"), at(9.5), false), 1.0);
        // A volley: the card goes, and the home crystal reached mid-volley
        // does not come up, during it or after it.
        assert_eq!(h.alpha(Some("Crystal B"), at(9.6), true), 0.0, "a held button puts it away");
        assert_eq!(h.alpha(Some("Home"), at(9.7), true), 0.0);
        assert_eq!(h.alpha(Some("Home"), at(12.0), true), 0.0);
        assert_eq!(h.alpha(Some("Home"), at(13.0), false), 0.0, "still shut once the button is up");
        // Moving off and back on brings it again after the rest.
        assert_eq!(h.alpha(None, at(14.0), false), 0.0);
        assert_eq!(h.alpha(Some("Home"), at(14.1), false), 0.0);
        assert_eq!(h.alpha(Some("Home"), at(14.5), false), 1.0);
    }

    /// The squad header shows a key only where it does that: with the
    /// build page's letters on the card, Split and Combine lose theirs (the
    /// review, 11-15; Guards and Heal are on the card since Card 2).
    #[test]
    fn the_squad_header_shows_no_key_the_card_takes() {
        let row = |k: &str, active: bool| SquadRow { key: k.into(), name: format!("Squad {k}"), soldiers: 4, hp: 0.5, note: Some("holding".into()), active, near: false, linked: false, auto: !active, state: String::new() };
        let panel = SquadPanel { rows: vec![row("1", true)], split_key: Some("Y".into()), combine_key: Some("J".into()), split_off: None, combine_off: None, free: false, free_key: Some("Esc".into()) };
        // Each header button's width: a key shown makes it wider.
        let widths = |cmds: Vec<Command>| {
            let mut r = Rig::new((2560, 1440));
            r.m.squads = Some(panel.clone());
            r.m.commands = cmds;
            r.draw();
            let w = |h: Hit| r.hud.hits.iter().find(|(_, x)| *x == h).map(|(b, _)| b.w).unwrap();
            (w(Hit::SquadSplit), w(Hit::SquadCombine))
        };
        let (split, combine) = widths(Vec::new());
        let (split_b, combine_b) = widths(vec![Command::new(1, (0, 0), Icon::Turret, "Siren", KeyCode::KeyY), Command::new(2, (0, 1), Icon::Turret, "Lure", KeyCode::KeyL)]);
        assert!(split_b < split - 5.0, "Y is the card's: split {split} -> {split_b}");
        assert!((combine_b - combine).abs() < 0.5, "J is not on that card: {combine} -> {combine_b}");
    }

    #[test]
    fn the_squad_header_keeps_clear_at_every_size() {
        let row = |k: &str, active: bool| SquadRow { key: k.into(), name: format!("Squad {k}"), soldiers: 4, hp: 0.8, note: Some("holding".into()), active, near: false, linked: false, auto: !active, state: String::new() };
        let panel = SquadPanel { rows: vec![row("1", true)], split_key: Some("Y".into()), combine_key: Some("J".into()), split_off: None, combine_off: None, free: false, free_key: Some("Esc".into()) };
        let heads = [Hit::SquadFree, Hit::SquadCombine, Hit::SquadSplit];
        for screen in [(1280, 800), (1920, 1080), (2560, 1440)] {
            for user in crate::world::settings::STEPS {
                let mut r = Rig::new(screen);
                r.m.squads = Some(panel.clone());
                r.m.loadout = Some(loadout());
                r.c.ui.user = user;
                r.draw();
                let tile = r.hud.hits.iter().find(|(_, h)| *h == Hit::Squad(0)).map(|(b, _)| *b).unwrap();
                let bs: Vec<Rect> = r.hud.hits.iter().filter(|(_, h)| heads.contains(h)).map(|(b, _)| *b).collect();
                assert!(!bs.is_empty(), "{screen:?} at {user}: no header buttons");
                for (k, b) in bs.iter().enumerate() {
                    // (The tiles start at the panel's inner edge, right of the gun strip.)
                    assert!(b.x >= tile.x - 0.5, "{screen:?} at {user}: a header button {b:?} left of the squad panel ({tile:?})");
                    for c in &bs[k + 1..] {
                        assert!(b.right() <= c.x + 0.5 || c.right() <= b.x + 0.5, "{screen:?} at {user}: {b:?} and {c:?} overlap");
                    }
                }
            }
        }
    }

    /// Many squads (owner, 2026-11-19 16:55Z): the tiles shrink so each
    /// squad keeps one, on the panel and apart, rather than some hiding
    /// behind a "+N".
    #[test]
    fn many_squads_shrink_their_tiles_instead_of_hiding() {
        let row = |k: usize| SquadRow { key: format!("{}", k + 1), name: format!("Squad {}", k + 1), soldiers: 4, hp: 0.8, note: Some("holding".into()), active: k == 0, near: false, linked: false, auto: k != 0, state: String::new() };
        for screen in [(1920, 1080), (1600, 972), (1280, 720)] {
            for n in [3, 9, 16] {
                let mut r = Rig::new(screen);
                r.m.squads = Some(SquadPanel { rows: (0..n).map(row).collect(), split_key: Some("Y".into()), combine_key: Some("J".into()), split_off: None, combine_off: None, free: false, free_key: Some("Esc".into()) });
                r.m.loadout = Some(loadout());
                r.draw();
                let tiles: Vec<Rect> = (0..n).map(|i| r.hud.hits.iter().find(|(_, h)| *h == Hit::Squad(i)).map(|(b, _)| *b).unwrap_or_else(|| panic!("{screen:?}, {n} squads: squad {} has no tile", i + 1))).collect();
                for (i, a) in tiles.iter().enumerate() {
                    assert!(a.w >= 24.0 && a.h >= 16.0, "{screen:?}, {n} squads: tile {a:?} too small to click");
                    for b in &tiles[i + 1..] {
                        let apart = a.right() <= b.x + 0.5 || b.right() <= a.x + 0.5 || a.bottom() <= b.y + 0.5 || b.bottom() <= a.y + 0.5;
                        assert!(apart, "{screen:?}, {n} squads: {a:?} and {b:?} overlap");
                    }
                }
            }
        }
    }

    /// The commanded squad's strip (controls review, 11-21): right of the
    /// tiles, inside the panel, none over another; on a wide screen with
    /// one squad all five show; with no squad commanded, none.
    #[test]
    fn the_squad_strip_sits_right_of_the_tiles() {
        let row = |k: usize| SquadRow { key: format!("{}", k + 1), name: format!("Squad {}", k + 1), soldiers: 4, hp: 0.8, note: Some("holding".into()), active: k == 0, near: false, linked: false, auto: k != 0, state: String::new() };
        let fact = |t: &str| SquadFact { icon: Icon::Group, text: t.into(), about: "x".into() };
        let facts = vec![fact("ring, loose"), fact("fire: defend"), fact("2 queued"), fact("Loadout 1"), fact("4 tools")];
        for screen in [(1920, 1080), (1600, 972), (1280, 720)] {
            for n in [1, 2, 3] {
                for free in [false, true] {
                    let mut r = Rig::new(screen);
                    r.m.squads = Some(SquadPanel { rows: (0..n).map(row).collect(), split_key: Some("Y".into()), combine_key: Some("J".into()), split_off: None, combine_off: None, free, free_key: Some("Esc".into()) });
                    r.m.squad_facts = facts.clone();
                    r.m.loadout = Some(loadout());
                    r.draw();
                    let tiles: Vec<Rect> = (0..n).filter_map(|i| r.hud.hits.iter().find(|(_, h)| *h == Hit::Squad(i)).map(|(b, _)| *b)).collect();
                    let end = tiles.iter().map(|t| t.right()).fold(0.0, f32::max);
                    let (row_top, row_bottom) = (tiles[0].y, tiles[0].bottom());
                    let shown: Vec<Rect> = r.hud.hits.iter().filter(|(b, h)| *h == Hit::Panel && b.x > end && b.y >= row_top - 1.0 && b.bottom() <= row_bottom + 1.0 && b.h < 40.0).map(|(b, _)| *b).collect();
                    if free {
                        assert!(shown.is_empty(), "{screen:?}, {n} squads, none commanded: {} facts drawn", shown.len());
                        continue;
                    }
                    if screen.0 == 1920 && n == 1 {
                        assert_eq!(shown.len(), facts.len(), "{screen:?}: one squad leaves room for every fact");
                    }
                    let panel = r.hud.hits.iter().filter(|(b, h)| *h == Hit::Panel && b.contains((tiles[0].x, tiles[0].y))).map(|(b, _)| *b).fold(None, |a: Option<Rect>, b| Some(a.map_or(b, |a| if b.w > a.w { b } else { a }))).unwrap();
                    for (i, a) in shown.iter().enumerate() {
                        assert!(a.right() <= panel.right() + 0.5, "{screen:?}, {n} squads: fact {a:?} runs out of the panel {panel:?}");
                        for b in &shown[i + 1..] {
                            let apart = a.right() <= b.x + 0.5 || b.right() <= a.x + 0.5 || a.bottom() <= b.y + 0.5 || b.bottom() <= a.y + 0.5;
                            assert!(apart, "{screen:?}, {n} squads: facts {a:?} and {b:?} overlap");
                        }
                    }
                }
            }
        }
    }

    #[test]
    fn squad_tiles_command_a_squad_bring_one_in_and_split_and_combine() {
        let row = |k: &str, n: usize, active: bool| SquadRow { key: k.into(), name: format!("Squad {k}"), soldiers: n, hp: 0.8, note: Some("holding".into()), active, near: k == "3", linked: false, auto: !active, state: String::new() };
        let panel = SquadPanel { rows: vec![row("1", 8, true), row("2", 3, false), row("3", 1, false)], split_key: Some("X".into()), combine_key: Some("G".into()), split_off: None, combine_off: None, free: false, free_key: Some("Esc".into()) };
        for screen in [(1920, 1080), (1280, 720), (640, 360), (3840, 2160)] {
            let mut r = Rig::new(screen);
            r.m.squads = Some(panel.clone());
            r.draw();
            let find = |r: &Rig, h: Hit| r.hud.hits.iter().rev().find(|(_, x)| *x == h).map(|(b, _)| *b).unwrap_or_else(|| panic!("{screen:?}: no {h:?}"));
            let press = |r: &mut Rig, b: Rect| {
                r.at((b.x + b.w / 2.0, b.y + b.h / 2.0));
                r.draw();
                assert!(r.click(Button::Left, true).0);
                r.click(Button::Left, false)
            };
            // Each squad is a tile in the bottom bar's middle panel.
            for i in 0..3 {
                let b = find(&r, Hit::Squad(i));
                assert!(b.y >= r.hud.bottom && b.bottom() <= screen.1 as f32 && b.right() <= screen.0 as f32 && b.x >= 0.0, "{screen:?}: tile {i} at {b:?}");
            }
            assert!(r.hud.hits.iter().all(|(_, h)| *h != Hit::SquadInto(0) && *h != Hit::SquadLink(0)), "the squad you command has no + or link");
            assert!(r.hud.hits.iter().all(|(_, h)| *h != Hit::SquadInto(1)), "a squad that doesn't overlap yours has no +");
            assert!(r.hud.hits.iter().all(|(_, h)| *h != Hit::SquadLink(1)), "a squad not touching yours can't be linked");
            let b = find(&r, Hit::SquadLink(2));
            assert_eq!(press(&mut r, b), (true, vec![HudEvent::SquadLink(2)]));
            let b = find(&r, Hit::Squad(1));
            // Left of the +, so the tile itself.
            assert_eq!(press(&mut r, Rect::new(b.x, b.y + b.h / 2.0, b.w / 3.0, b.h / 2.0)), (true, vec![HudEvent::Squad(1)]));
            let b = find(&r, Hit::SquadInto(2));
            assert_eq!(press(&mut r, b), (true, vec![HudEvent::SquadInto(2)]));
            let b = find(&r, Hit::SquadSplit);
            assert_eq!(press(&mut r, b), (true, vec![HudEvent::SquadSplit]));
            let b = find(&r, Hit::SquadCombine);
            assert_eq!(press(&mut r, b), (true, vec![HudEvent::SquadCombine]));
            // A button that would do nothing is greyed and takes the click
            // for nothing.
            r.m.squads = Some(SquadPanel { combine_off: Some("No other squad".into()), ..panel.clone() });
            r.draw();
            assert!(r.hud.hits.iter().all(|(_, h)| *h != Hit::SquadCombine));
            // Let go: command no squad; greyed once none is commanded.
            r.m.squads = Some(panel.clone());
            r.draw();
            let b = find(&r, Hit::SquadFree);
            assert!(b.x >= 0.0 && b.right() <= screen.0 as f32, "{screen:?}: Let go at {b:?}");
            assert_eq!(press(&mut r, b), (true, vec![HudEvent::SquadFree]));
            let free = SquadPanel { free: true, rows: panel.rows.iter().map(|q| SquadRow { active: false, auto: true, ..q.clone() }).collect(), ..panel.clone() };
            r.m.squads = Some(free);
            r.draw();
            assert!(r.hud.hits.iter().all(|(_, h)| *h != Hit::SquadFree), "nothing to let go of");
            let b = find(&r, Hit::Squad(0));
            assert_eq!(press(&mut r, Rect::new(b.x, b.y + b.h / 2.0, b.w / 3.0, b.h / 2.0)), (true, vec![HudEvent::Squad(0)]), "a tile still takes command");
        }
    }

    #[test]
    fn the_bottom_panels_cover_at_most_about_15_percent_and_collapse_when_empty() {
        for screen in [(1920, 1080), (1280, 800), (2560, 1440)] {
            let mut r = Rig::new(screen);
            r.m.selection = vec![Card { id: 0, icon: Icon::Soldier, team: 0, hp: 1.0, badge: None }];
            r.draw();
            let share = (screen.1 as f32 - r.hud.bottom) / screen.1 as f32;
            // 15% before the owner's 08:33Z ask (one squad row, two command
            // rows); 10.5% after.
            assert!(share <= 0.11, "{screen:?}: the bar is {:.1}% of the screen", share * 100.0);
            assert!((Hud::insets(r.c.screen, r.c.ui).1 - (screen.1 as f32 - r.hud.bottom)).abs() < 0.01 && (Hud::insets(r.c.screen, r.c.ui).0 - r.hud.top).abs() < 0.01, "insets match the drawn layout");
            // With nothing selected, no panel reaches above the collapsed
            // strip except the command card and minimap.
            let full = r.hud.hits.iter().filter(|(b, h)| *h == Hit::Panel && b.h >= 90.0 * r.hud.scale).count();
            r.m.selection.clear();
            r.draw();
            let empty = r.hud.hits.iter().filter(|(b, h)| *h == Hit::Panel && b.h >= 90.0 * r.hud.scale).count();
            assert_eq!(empty + 1, full, "{screen:?}: the selection panel collapses");
        }
    }

    /// The bottom row over the real cases (the three-reviewer review,
    /// 11-15, step 2): the game's own window (1600 x 972 on a 1080p
    /// monitor, 1600 x 1000 on 1440p) and the review's screens, every UI
    /// step, squads (1, 4, 9) and RTS soldiers, guns of 3, 5 and 8 slots,
    /// the main page and the build page. The panels keep apart and on the
    /// screen; every text run in the bar sits inside one panel, off every
    /// other run, and at no less than 10.5 units (the square card's
    /// costs), except in a strip drawn smaller, which only the barest row
    /// does; the card keeps its labels only with all five squad buttons
    /// drawn. Returns the cases for the printout.
    fn bar_cases(mut each: impl FnMut((u32, u32), f32, &str, usize, &Rig)) {
        let row = |k: usize| SquadRow { key: format!("{}", k + 1), name: format!("Squad {}", k + 1), soldiers: 4 + k, hp: 0.8, note: Some("holding".into()), active: k == 0, near: k == 1, linked: false, auto: k != 0, state: if k == 0 { "firing".into() } else { "holding".into() } };
        let panel = |n: usize| SquadPanel { rows: (0..n).map(row).collect(), split_key: Some("Y".into()), combine_key: Some("J".into()), split_off: None, combine_off: None, free: false, free_key: Some("Esc".into()) };
        let gun = |slots: usize| Loadout { slots, cards: loadout().cards.into_iter().cycle().take(slots - 1).collect(), gun: Some((Icon::Rifle, "Rifle".into(), "range 20, 2 a second".into())), ..loadout() };
        // The squad mode's Card 2 and its build page (option 1), as the
        // game words them (owner 2026-11-15).
        let main = || -> Vec<Command> {
            [
                ("Attack-move", KeyCode::KeyR, None),
                ("Guard", KeyCode::KeyF, None),
                ("Drain a crystal", KeyCode::KeyG, None),
                ("Fire: defend", KeyCode::KeyX, None),
                ("Heal", KeyCode::KeyL, None),
                ("Buy a soldier", KeyCode::KeyB, Some("400")),
                ("Pump", KeyCode::KeyU, Some("500")),
                ("Build", KeyCode::KeyQ, None),
                ("Tools", KeyCode::KeyE, None),
                ("Lookouts: some", KeyCode::KeyI, None),
            ]
            .into_iter()
            .enumerate()
            .map(|(k, (n, key, cost))| Command { cost: cost.map(Into::into), ..Command::new(k as u32 + 1, ((k / 5) as u8, (k % 5) as u8), Icon::Move, n, key) })
            .collect()
        };
        let build = || -> Vec<Command> {
            [("Turret", KeyCode::KeyT, "500"), ("Pump", KeyCode::KeyU, "500"), ("Wall", KeyCode::KeyV, "20/cell"), ("Lure", KeyCode::KeyL, "300"), ("Siren", KeyCode::KeyY, "300"), ("Lures, sirens on/off", KeyCode::KeyJ, ""), ("Outpost", KeyCode::KeyI, "400")]
                .into_iter()
                .enumerate()
                .map(|(k, (n, key, cost))| Command { cost: (!cost.is_empty()).then(|| cost.into()), locked: k == 6, ..Command::new(k as u32 + 1, ((k / 5) as u8, (k % 5) as u8), Icon::Turret, n, key) })
                .chain([Command::new(9, (1, 4), Icon::Close, "Back", KeyCode::KeyQ)])
                .collect()
        };
        for screen in [(1600, 972), (1600, 1000), (1280, 800), (1920, 1080), (2560, 1440)] {
            for user in crate::world::settings::STEPS {
                for (what, squads, page) in [("squads 1", 1, 0), ("squads 4", 4, 0), ("squads 9", 9, 0), ("squads 1 build", 1, 1), ("rts 6", 0, 0), ("rts 6 build", 0, 1)] {
                    // (Guns hold 2 to 6, the rifle 4: `mould::guns`.)
                    for slots in [2, 4, 6] {
                        let mut r = Rig::new(screen);
                        r.c.ui.user = user;
                        r.m.commands = if page == 1 { build() } else { main() };
                        if squads > 0 {
                            r.m.squads = Some(panel(squads));
                        } else {
                            r.m.selection = vec![Card { id: 0, icon: Icon::Soldier, team: 0, hp: 0.7, badge: Some(Icon::Rifle) }; 6];
                        }
                        r.m.loadout = Some(gun(slots));
                        r.draw();
                        each(screen, user, what, slots, &r);
                    }
                }
            }
        }
    }

    /// The table behind `Reports/bar-allocator-2026-11-15/raw/row-cases.txt`:
    /// each case's fit and panel widths (units), the selection's needs,
    /// and what the rule before the allocator chose. Run with
    /// `cargo test --release --lib bottom_row_printout -- --ignored --nocapture`.
    #[test]
    #[ignore]
    fn bottom_row_printout() {
        bar_cases(|screen, user, what, slots, r| {
            let (p, lab, fit) = &r.hud.row;
            let s = r.hud.scale;
            let w: Vec<String> = p.iter().map(|q| format!("{:.0}", q.w / s)).collect();
            let mut c = Canvas::new(screen);
            c.ui.user = user;
            let need = r.hud.sel_needs(&r.m, &c, s);
            // The rule before the allocator (main at 2f6ac3f4), from the
            // same row: labels where 340 units stayed beside the full
            // strip, then the strip down to 60% where under 300 stayed.
            let right = p[2].right();
            let l = r.m.loadout.as_ref().unwrap();
            let full = loadout_w(l, GunFit::FULL, s);
            let card_of = |w: f32| CARD_COLS as f32 * w + (CARD_COLS - 1) as f32 * CELL_GAP * s + 2.0 * CARD_PAD * s;
            let old_lab = right - card_of(CELL_W * s) - 24.0 * s - (full + 8.0 * s) >= 340.0 * s;
            let card_x = right - card_of(if old_lab { CELL_W } else { CELL } * s);
            let room = card_x - 24.0 * s - 300.0 * s;
            let old_gb = if full > room { (room / full).clamp(0.6, 1.0) } else { 1.0 };
            eprintln!("{screen:?} {user} {what} {slots}: s {s} lab {lab} {fit:?} widths {} need {:.0}/{:.0}/{:.0} | before lab {old_lab} gun {old_gb:.2}", w.join("/"), need[0] / s, need[1] / s, need[2] / s);
        });
    }

    #[test]
    fn the_bottom_row_keeps_its_text_at_every_size() {
        let heads = [Hit::SquadFree, Hit::SquadCombine, Hit::SquadSplit];
        let (mut cases, mut shrunk, mut labelled) = (0, Vec::new(), 0);
        bar_cases(|screen, user, what, slots, r| {
            cases += 1;
            let at = format!("{screen:?} at {user}, {what}, {slots} slots");
            let (panels, lab, fit) = &r.hud.row;
            let s = r.hud.scale;
            let sw = screen.0 as f32 + 0.5;
            // The panels: on the screen, left to right, apart.
            let shown: Vec<Rect> = panels.iter().copied().filter(|p| p.w >= 120.0 * s || p.h >= 90.0 * s).collect();
            for (k, p) in shown.iter().enumerate() {
                assert!(p.x >= -0.5 && p.right() <= sw, "{at}: panel {p:?} off the screen");
                if let Some(q) = shown.get(k + 1) {
                    assert!(p.right() <= q.x + 0.5, "{at}: panels {p:?} and {q:?} overlap");
                }
            }
            // The text in the bar.
            let runs: Vec<_> = r.c.runs.iter().filter(|t| t.base >= r.hud.bottom && t.ink.0 < t.ink.2).collect();
            assert!(!runs.is_empty(), "{at}: no text in the bar");
            let gun = panels[0];
            for (k, t) in runs.iter().enumerate() {
                let (x0, x1) = (t.ink.0, t.ink.2);
                let inside = shown.iter().any(|p| x0 >= p.x - 0.5 && x1 <= p.right() + 0.5);
                assert!(inside, "{at}: {:?} at {x0}..{x1} is in no panel", t.text);
                assert!(t.px >= 10.5 * s - 0.01, "{at}: {:?} at {} px, under 10.5 x {s}", t.text, t.px);
                // The glyphs' own pixels: no two runs touch.
                for u in &runs[k + 1..] {
                    let apart = t.ink.2 <= u.ink.0 || u.ink.2 <= t.ink.0 || t.ink.3 <= u.ink.1 || u.ink.3 <= t.ink.1;
                    assert!(apart, "{at}: {:?} {:?} and {:?} {:?} overlap", t.text, t.ink, u.text, u.ink);
                }
            }
            if fit.scale < 1.0 {
                // Only the bare strip, which has no words, draws smaller.
                assert!(!fit.name && !fit.swap, "{at}: {fit:?}");
                assert!(!runs.iter().any(|t| t.ink.0 < gun.right()), "{at}: words in a smaller strip");
                shrunk.push((at.clone(), s));
            }
            // Every squad button is drawn wherever the row's floor holds
            // (all but the 6-slot gun at the biggest sizes, whose strip
            // stops at 60%), so labels are never bought with one.
            if r.m.squads.is_some() {
                let drawn = heads.iter().filter(|h| r.hud.hits.iter().any(|(_, x)| x == *h)).count();
                assert!(drawn == heads.len() || (fit.scale <= 0.6 && !lab), "{at}: {drawn} squad buttons drawn, {fit:?}");
            }
            labelled += *lab as usize;
        });
        // The strip draws smaller only near the biggest UI sizes (the
        // screen at 1.3 units a pixel or more), and below that only a gun
        // of more than the rifle's four slots: since the steady-HUD pass
        // (11-20) a 6-slot gun's cards shrink inside their panel at 125%
        // on 1600 x 972 and 1280 x 800, where the command card went square
        // for it before.
        //
        // Since the card's sixth column (owner 2026-11-21 06:21Z: "Shrink
        // the gun section to get another column") the rifle's strip draws
        // smaller too wherever the row is tight (125% on 1600 x 972 and
        // 1280 x 800, 175% and up on 1920 x 1080): the card keeps its
        // labels and the gun gives the room. At 100% on 1920 x 1080 and
        // wider the rifle's strip keeps its size.
        assert!(shrunk.iter().all(|(at, s)| *s >= 0.9 || at.ends_with("6 slots")), "the strip shrank at a small size: {shrunk:?}");
        assert!(labelled > cases / 4, "labels in only {labelled} of {cases} cases");
    }

    /// The owner's ask (2026-11-20 22:08Z): "if we have many squads then
    /// the actions shrink or once we have guns with lots of cards the
    /// actions shrink. Things should stay more consistent." Over every
    /// case of `bar_cases`, the command card keeps one place and size for
    /// the window and UI size whatever the squads, the page or the gun, and
    /// the gun strip one width whatever the selection; a gun with fewer
    /// slots than the widest in play keeps the widest's width.
    #[test]
    fn the_command_card_and_gun_strip_hold_still() {
        use std::collections::HashMap;
        let key = |r: Rect| [r.x, r.y, r.w, r.h].map(|v| (v * 4.0).round() as i32);
        let mut cards: HashMap<String, ([i32; 4], String)> = HashMap::new();
        let mut guns: HashMap<String, ([i32; 4], String)> = HashMap::new();
        bar_cases(|screen, user, what, slots, r| {
            let (panels, ..) = &r.hud.row;
            let at = format!("{screen:?} at {user}");
            let card = key(panels[2]);
            let (was, first) = cards.entry(at.clone()).or_insert((card, format!("{what}, {slots} slots")));
            assert_eq!(*was, card, "{at}: the command card moved between {first} and {what}, {slots} slots");
            let gun = key(panels[0]);
            let (was, first) = guns.entry(format!("{at}, {slots} slots")).or_insert((gun, what.to_string()));
            assert_eq!(*was, gun, "{at}, {slots} slots: the gun strip changed between {first} and {what}");
        });
        // A squad with a 2-slot gun beside one with a 6-slot gun.
        for screen in [(1600, 972), (1920, 1080), (2560, 1440)] {
            for user in crate::world::settings::STEPS {
                let mut r = Rig::new(screen);
                r.c.ui.user = user;
                let gun = |slots: usize, widest: usize| Loadout { slots, widest, cards: loadout().cards.into_iter().take(slots).collect(), ..loadout() };
                r.m.loadout = Some(gun(6, 6));
                r.draw();
                let wide = r.hud.row.0[0];
                r.m.loadout = Some(gun(2, 6));
                r.draw();
                assert_eq!(key(r.hud.row.0[0]), key(wide), "{screen:?} at {user}: the strip changed width with the commanded squad's gun");
            }
        }
    }

    /// The owner's case (the review, 11-15): 125% on 1920 x 1080, one
    /// squad, the rifle's four slots. Before the allocator the card went
    /// to icons there while the squad panel kept about 450 px empty; now
    /// the card keeps its labels and the gun strip its cards, and at 100%
    /// the strip keeps everything.
    #[test]
    fn at_125_percent_on_1080p_the_card_keeps_its_labels() {
        let mut found = Vec::new();
        bar_cases(|screen, user, what, slots, r| {
            if screen == (1920, 1080) && what == "squads 1" && slots == 4 && (user == 1.0 || user == 1.25) {
                found.push((user, r.hud.row.1, r.hud.row.2));
            }
        });
        assert_eq!(found.len(), 2);
        for (user, lab, fit) in found {
            assert!(lab, "{user}: the card went square ({fit:?})");
            assert_eq!(fit.scale, 1.0, "{user}: the strip shrank");
            if user == 1.0 {
                assert_eq!(fit, GunFit::FULL, "at 100% the strip keeps everything");
            }
        }
    }

    #[test]
    fn a_loadout_draws_in_any_window_and_with_no_cards() {
        for screen in [(1920, 1080), (640, 360), (200, 120), (3840, 2160)] {
            let mut r = Rig::new(screen);
            r.m.loadout = Some(loadout());
            r.draw();
            r.m.loadout = Some(Loadout { cards: vec![], cycle: None, ..loadout() });
            r.draw();
            // More cards than slots still draws every card.
            let mut l = loadout();
            l.cards.extend(l.cards.clone());
            r.m.loadout = Some(l);
            r.draw();
        }
    }

    #[test]
    fn a_drag_or_pan_that_ends_over_a_panel_still_ends_in_the_controls() {
        let mut r = Rig::new((1920, 1080));
        for b in [Button::Left, Button::Middle, Button::Right] {
            r.at(WORLD_SPOT);
            assert!(!r.click(b, true).0, "{b:?} press on the world");
            r.at(r.centre(NOW));
            let (took, ev) = r.click(b, false);
            assert!(!took && ev.is_empty(), "{b:?} release over a panel was swallowed");
        }
    }

    #[test]
    fn a_button_clicked_gives_its_command() {
        let mut r = Rig::new((1920, 1080));
        r.at(r.centre(NOW));
        assert!(r.click(Button::Left, true).0);
        assert_eq!(r.click(Button::Left, false), (true, vec![HudEvent::Command { id: NOW }]));
    }

    #[test]
    fn a_wall_released_over_a_panel_is_still_laid() {
        let mut r = Rig::new((1920, 1080));
        r.key(KeyCode::KeyV);
        r.draw();
        r.at(WORLD_SPOT);
        assert!(r.click(Button::Left, true).0);
        assert!(r.hud.targeting().unwrap().2.is_some(), "the line's first end");
        r.at(r.centre(NOW));
        let (took, ev) = r.click(Button::Left, false);
        assert!(took);
        assert!(matches!(ev.as_slice(), [HudEvent::Line { id: WALL, .. }]), "{ev:?}");
        assert!(r.hud.targeting().is_none());
    }

    #[test]
    fn a_placement_on_the_minimap_lands_where_the_minimap_shows() {
        let mut r = Rig::new((1920, 1080));
        let mm = r.hud.minimap.expect("a minimap");
        r.key(KeyCode::KeyA);
        r.draw();
        r.at((mm.0 as f32 + mm.2 as f32 / 2.0, mm.1 as f32 + mm.3 as f32 / 2.0));
        let (took, ev) = r.click(Button::Left, true);
        assert!(took);
        let [HudEvent::Point { id: POINT, at }] = ev.as_slice() else { panic!("{ev:?}") };
        assert!((at.0 - 1024.0).abs() < 8.0 && (at.1 - 640.0).abs() < 8.0, "{at:?}");
    }

    #[test]
    fn a_throws_arc_bows_up_the_screen_and_ends_on_its_points() {
        let (a, b) = ((10.0, 50.0), (50.0, 50.0));
        assert_eq!(arc_point(a, b, 0.0), a);
        assert_eq!(arc_point(a, b, 1.0), b);
        let top = arc_point(a, b, 0.5);
        assert_eq!(top.0, 30.0);
        assert!((top.1 - (50.0 - ARC_RISE * 40.0)).abs() < 1e-4, "the middle is ARC_RISE of the length up the screen: {top:?}");
    }

    #[test]
    fn a_red_ghost_is_refused_and_targeting_stays() {
        let mut r = Rig::new((1920, 1080));
        r.m.ghost_ok = false;
        r.m.ghost_note = Some("too close".into());
        r.key(KeyCode::KeyA);
        r.draw();
        r.at(WORLD_SPOT);
        let (took, ev) = r.click(Button::Left, true);
        assert!(took && ev.is_empty(), "{ev:?}");
        assert!(r.hud.targeting().is_some());
        assert!(r.hud.alerts.iter().any(|a| a.text == "too close"));
    }

    #[test]
    fn the_end_screen_takes_every_key() {
        let mut r = Rig::new((1920, 1080));
        r.m.end = Some(Ending::plain("Over", vec![]));
        r.draw();
        assert_eq!(r.key(KeyCode::KeyA), (true, vec![]));
        assert_eq!(r.key(KeyCode::Escape), (true, vec![]));
        assert!(!r.hud.menu_open() && r.hud.targeting().is_none());
        // A click off its buttons does nothing either.
        r.at(WORLD_SPOT);
        assert_eq!(r.click(Button::Left, true), (true, vec![]));
        // Enter presses its first button (the time between planets, 11-24):
        // Play again, or a run's carry-on; Space stays the alerts' key.
        assert_eq!(r.key(KeyCode::Enter), (true, vec![HudEvent::Menu(MenuItem::Restart)]));
        assert_eq!(r.key(KeyCode::Space), (true, vec![]));
        r.m.carry_on = Some("On to the shop".into());
        r.draw();
        assert_eq!(r.key(KeyCode::NumpadEnter), (true, vec![HudEvent::Menu(MenuItem::Continue)]));
    }

    #[test]
    fn escape_walks_the_menu_pages() {
        let mut r = Rig::new((1920, 1080));
        assert_eq!(r.key(KeyCode::Escape).1, vec![HudEvent::Menu(MenuItem::Opened)]);
        r.hud.show_help();
        assert_eq!(r.key(KeyCode::Escape).1, vec![]);
        assert!(r.hud.menu_open(), "help goes back to the main page");
        assert_eq!(r.key(KeyCode::Escape).1, vec![HudEvent::Menu(MenuItem::Resume)]);
        assert!(!r.hud.menu_open());
        // With the menu open, hotkeys do nothing.
        r.key(KeyCode::Escape);
        assert_eq!(r.key(KeyCode::KeyS), (true, vec![]));
    }

    #[test]
    fn f1_or_question_mark_is_the_key_card_from_play() {
        for k in [KeyCode::F1, KeyCode::Slash] {
            let mut r = Rig::new((1920, 1080));
            assert_eq!(r.key(k), (true, vec![HudEvent::Menu(MenuItem::Opened)]), "{k:?} pauses");
            assert!(r.hud.menu_open());
            // Again, or Esc, goes straight back to play (not the main page).
            assert_eq!(r.key(k).1, vec![HudEvent::Menu(MenuItem::Resume)]);
            assert!(!r.hud.menu_open());
            r.key(k);
            assert_eq!(r.key(KeyCode::Escape).1, vec![HudEvent::Menu(MenuItem::Resume)]);
            assert!(!r.hud.menu_open());
            // From the paused menu it is the menu's controls page, and Esc
            // goes back to the menu.
            r.key(KeyCode::Escape);
            assert_eq!(r.key(k).1, vec![]);
            assert_eq!(r.key(KeyCode::Escape).1, vec![]);
            assert!(r.hud.menu_open());
        }
    }

    /// The key card from play with as many rows as the squad mode's (some
    /// forty) stays on the screen at every UI size, in columns, and at
    /// 200% is drawn bigger than at 100% (11-15: one column ran off a
    /// 1080-line screen).
    #[test]
    fn the_key_card_fits_at_every_size() {
        for screen in [(1280, 800), (1920, 1080), (2560, 1440)] {
            let back = |user: f32| {
                let mut r = Rig::new(screen);
                r.m.help = (0..30).map(|k| (format!("K{k}"), "Does a thing".to_string())).collect();
                r.c.ui.user = user;
                r.key(KeyCode::F1);
                r.draw();
                r.hud.hits.iter().find(|(_, h)| *h == Hit::Back).map(|(b, _)| *b).unwrap()
            };
            for user in crate::world::settings::STEPS {
                let b = back(user);
                assert!(b.y >= 0.0 && b.bottom() <= screen.1 as f32 && b.x >= 0.0 && b.right() <= screen.0 as f32, "{screen:?} at {user}: Back {b:?} off screen");
            }
            assert!(back(2.0).h > back(1.0).h * 1.25, "{screen:?}: 200% grows the card");
        }
    }

    #[test]
    fn the_main_menu_button_shows_only_when_the_game_has_one() {
        for on in [false, true] {
            let mut r = Rig::new((1920, 1080));
            r.m.main_menu = on;
            r.key(KeyCode::Escape);
            r.draw();
            assert_eq!(r.hud.hits.iter().any(|(_, h)| *h == Hit::Menu(MenuItem::MainMenu)), on);
            r.m.end = Some(Ending::plain("Over", vec![]));
            r.draw();
            let b = r.hud.hits.iter().find(|(_, h)| *h == Hit::Menu(MenuItem::MainMenu)).map(|(b, _)| *b);
            assert_eq!(b.is_some(), on, "end screen");
            if let Some(b) = b {
                r.at((b.x + 4.0, b.y + 4.0));
                r.click(Button::Left, true);
                assert_eq!(r.click(Button::Left, false).1, vec![HudEvent::Menu(MenuItem::MainMenu)]);
            }
        }
    }

    #[test]
    fn the_pause_note_sits_above_the_buttons() {
        // Save and resume C (2026-11-24): the run's pause menu says what a
        // quit keeps; the note pushes the buttons down, never under them.
        let resume_top = |note: Vec<String>| {
            let mut r = Rig::new((1280, 720));
            r.m.pause_note = note;
            r.key(KeyCode::Escape);
            r.draw();
            r.hud.hits.iter().find(|(_, h)| *h == Hit::Menu(MenuItem::Resume)).map(|(b, _)| b.y).unwrap()
        };
        let plain = resume_top(Vec::new());
        let noted = resume_top(vec!["Quit mid-planet: you restart this planet".into(), "with your run intact.".into()]);
        // The modal stays centred, so the buttons move down by half the
        // note's two lines and gap (22 points at this scale's 0.7).
        assert!(noted >= plain + 15.0, "{plain} then {noted}");
    }

    /// The planet on the pause menu (Direction 11-24): the goal, the
    /// slime's line and the twist, beside the card where the screen is
    /// wide, in it where it is only tall, and left out where neither fits;
    /// the card and its buttons stay on the screen at every size.
    #[test]
    fn the_pause_menu_tells_the_planet() {
        let line = "Where you hurt it, it grows back tough. Don't fight on old ground";
        for (screen, user, told) in [((1920, 1080), 1.0, true), ((3840, 2160), 2.5, true), ((1280, 720), 1.0, true), ((1920, 1080), 2.0, true), ((1280, 540), 2.0, true), ((800, 800), 1.0, true)] {
            let mut r = Rig::new(screen);
            r.c.ui.user = user;
            r.m.objective = Some("Kill the slime's heart on its home crystal: 1,660 of 1,660 left, breaks under 166".into());
            r.m.pause_note = vec!["Quit any time: this planet and".into(), "your run are saved.".into()];
            r.m.pause_planet = Some(Intro { kicker: "Planet 7 of 8 · Ossary".into(), title: "The Grudge".into(), line: line.into(), twist: Some(("Cold snap".into(), "Every two minutes the cold slows the slime and your squads for 20 seconds".into())), alpha: 1.0 });
            r.key(KeyCode::Escape);
            r.draw();
            let said = |t: &str| r.c.runs.iter().any(|x| x.text == t);
            assert_eq!(said("THE GRUDGE") && said("COLD SNAP") && said("THE GOAL"), told, "{screen:?} at {user}");
            let (sw, sh) = (screen.0 as f32, screen.1 as f32);
            for (b, h) in &r.hud.hits {
                assert!(b.x >= -0.5 && b.y >= -0.5 && b.right() <= sw + 0.5 && b.bottom() <= sh + 0.5, "{screen:?} at {user}: {h:?} {b:?} off the screen");
            }
            for x in &r.c.runs {
                assert!(x.ink.0 >= -0.5 && x.ink.2 <= sw + 0.5 && x.ink.1 >= -0.5 && x.ink.3 <= sh + 0.5, "{screen:?} at {user}: {:?} off the screen", x.text);
            }
        }
    }

    fn two_experiments() -> Vec<Experiment> {
        let info = |live: bool| ExpInfo { lane: "Squad and arsenal".into(), since: "2026-11-15".into(), sees: "The camera shakes on blasts, and the game holds still for a moment when a big blast lands".into(), why_off: "To be settled by play".into(), live };
        let on_off = |key: &str, label: &str, at: usize, live: bool| Experiment { key: key.into(), label: label.into(), choices: vec!["off".into(), "on".into()], default: 0, at, next: None, info: info(live) };
        vec![on_off("feel", "Shake and hit-stop", 1, true), on_off("crystal_regrow", "Crystals refill", 0, false)]
    }

    /// **The pause menu's Experiments page** (2026-11-19): a button on
    /// the menu only when the game has switches, saying how many are on;
    /// on the page each switch whole (name, state, lane, date, what you
    /// will see, why it is off) and the count against the limit; a click
    /// on a choice is the game's to apply; a pick waiting for a restart
    /// shows on its card and lights Restart; Esc goes back to the menu.
    /// At 200% on 1080 lines the page stays on the screen.
    #[test]
    fn the_pause_menu_sets_the_experiments() {
        let mut r = Rig::new((1920, 1080));
        r.key(KeyCode::Escape);
        r.draw();
        assert!(!r.hud.hits.iter().any(|(_, h)| *h == Hit::ExpPage), "no switches, no button");
        r.m.experiments = two_experiments();
        r.m.most_off = 3;
        r.draw();
        let said = |r: &Rig, t: &str| r.c.runs.iter().any(|x| x.text == t);
        assert!(said(&r, "Experiments  ·  1 on"));
        let b = r.hud.hits.iter().find(|(_, h)| *h == Hit::ExpPage).map(|(b, _)| *b).expect("the button");
        r.at((b.x + 4.0, b.y + 4.0));
        r.click(Button::Left, true);
        assert_eq!(r.click(Button::Left, false).1, vec![]);
        r.draw();
        for t in ["Experiments", "Shake and hit-stop", "Crystals refill", "ON", "OFF", "2 of 3 allowed  ·  1 on in this game", "YOU WILL SEE", "OFF BECAUSE", "To be settled by play"] {
            assert!(said(&r, t), "no {t:?} on the page");
        }
        assert!(r.c.runs.iter().any(|x| x.text.starts_with("Squad and arsenal  ·  landed 2026-11-15  ·  a pick here applies at once")));
        assert!(r.c.runs.iter().any(|x| x.text.contains("a pick here applies when you restart")));
        // Crystals refill's "on".
        let on = r.hud.hits.iter().find(|(_, h)| *h == Hit::Exp(1, 1)).map(|(b, _)| *b).expect("its choice");
        r.at((on.x + on.w / 2.0, on.y + on.h / 2.0));
        r.click(Button::Left, true);
        assert_eq!(r.click(Button::Left, false).1, vec![HudEvent::Experiment { row: 1, choice: 1 }]);
        assert!(r.hud.menu_open(), "a pick stays on the page");
        // The game says it waits for a restart.
        r.m.experiments[1].next = Some(1);
        r.draw();
        assert!(said(&r, "ON AT RESTART") && said(&r, "Waiting for a restart: Crystals refill on") && said(&r, "Restart now"));
        assert!(r.hud.hits.iter().any(|(_, h)| *h == Hit::Menu(MenuItem::Restart)));
        // Every word inside the screen.
        for x in &r.c.runs {
            assert!(x.x >= 0.0 && x.x + x.w <= 1920.0 && x.base <= 1080.0, "{:?} off the screen", x.text);
        }
        assert_eq!(r.key(KeyCode::Escape).1, vec![]);
        assert!(r.hud.menu_open());
        r.draw();
        assert!(said(&r, "Restart with the new pick"));
        // At 200%, three switches: the page shrinks to fit.
        let mut r = Rig::new((1920, 1080));
        r.c.ui.user = 2.0;
        r.m.experiments = two_experiments();
        r.m.experiments.push(Experiment { key: "third".into(), label: "A third".into(), ..r.m.experiments[1].clone() });
        r.m.most_off = 3;
        r.hud.show_experiments();
        r.draw();
        for x in &r.c.runs {
            assert!(x.x >= 0.0 && x.x + x.w <= 1920.0 && x.base <= 1080.0 && x.base >= 0.0, "{:?} off the screen at 200%", x.text);
        }
    }

    /// A run's end screen carries on (to the shop) in place of Play again.
    #[test]
    fn the_end_screen_carries_on_when_the_game_names_it() {
        for carry in [None, Some("On to the shop")] {
            let mut r = Rig::new((1600, 900));
            r.m.carry_on = carry.map(str::to_string);
            r.m.end = Some(Ending::plain("Planet 1 freed", vec![]));
            r.draw();
            let has = |r: &Rig, h: Hit| r.hud.hits.iter().find(|(_, x)| *x == h).map(|(b, _)| *b);
            assert_eq!(has(&r, Hit::Menu(MenuItem::Restart)).is_some(), carry.is_none());
            if let Some(b) = has(&r, Hit::Menu(MenuItem::Continue)) {
                r.at((b.x + 4.0, b.y + 4.0));
                r.click(Button::Left, true);
                assert_eq!(r.click(Button::Left, false).1, vec![HudEvent::Menu(MenuItem::Continue)]);
            } else {
                assert!(carry.is_none());
            }
        }
    }

    /// The end screen (11-18): a run's planet lost, with everything it
    /// can hold and three buttons, stays on the screen at every UI size
    /// and screen; its numbers and the slime's sentence are drawn, and
    /// the stars only for a win.
    #[test]
    fn the_end_screen_shows_the_planets_numbers_and_fits() {
        let says = "It walls the crystals it holds in amber, and sets where you shoot it. The walls take three hits and stop your soldiers. Fire burns them: bring a Torch";
        let tiles: Vec<(String, String)> = [("9:41", "played"), ("12,604", "slime killed"), ("1,520", "shots, 8.3 kills each"), ("0", "soldiers standing, 9 lost"), ("5,700", "food harvested"), ("42.3k of 43.4k", "slime left, of its peak")].iter().map(|(a, b)| (a.to_string(), b.to_string())).collect();
        let end = Ending { place: "Planet 3 of 8 · Drevix · The Trunk".into(), title: "The squad is lost".into(), won: Some(false), stars: Some(0), says: says.into(), tiles, lines: vec!["Your last soldier fell at 9:41 (best 40% of the goal)".into(), "The run ends on planet 3".into()], curve: (0..581u32).map(|t| (20_000 + (t * 37 % 9_000), 30 - t / 20)).collect() };
        for screen in [(1280, 800), (1920, 1080), (2560, 1440)] {
            for user in crate::world::settings::STEPS {
                let mut r = Rig::new(screen);
                r.c.ui.user = user;
                r.m.main_menu = true;
                r.m.carry_on = Some("End the run".into());
                r.m.end = Some(end.clone());
                r.draw();
                let quit = r.hud.hits.iter().find(|(_, h)| *h == Hit::Menu(MenuItem::Quit)).map(|(b, _)| *b).unwrap();
                assert!(quit.bottom() <= screen.1 as f32 && quit.y > 0.0 && quit.x >= 0.0 && quit.right() <= screen.0 as f32, "{screen:?} at {user}: Quit {quit:?} off screen");
                for t in ["12,604", "slime killed", "42.3k of 43.4k", "THE TRUNK", "Slime", "peak ", "9:41"] {
                    assert!(r.c.runs.iter().any(|x| x.text.contains(t)), "{screen:?} at {user}: no {t}");
                }
                assert!(r.c.runs.iter().any(|x| x.text.starts_with("It walls the crystals")), "{screen:?} at {user}: the slime's sentence");
            }
        }
    }

    /// The experiments mark (the plan of 2026-11-17: "the HUD shows a
    /// small mark when anything is off its default"): none in the default
    /// game; with a switch on, a chip under the clock at the screen's
    /// right edge, clear of the top bar's pills and of the bottom bar,
    /// that names the switches when the cursor is on it and keeps a click
    /// from reaching the world. At the plain size and at 200%.
    #[test]
    fn a_round_off_the_default_game_wears_a_mark() {
        for user in [1.0, 2.0] {
            let mut r = Rig::new((1920, 1080));
            r.c.ui.user = user;
            r.m.clock = "1:00 / 18:00".into();
            r.m.speed = Some("Paused".into());
            r.m.objective = Some("Wipe out the slime: cut it to 1% of its peak (counting from 1:00; slime count 43,957)".into());
            r.draw();
            let said = |r: &Rig, t: &str| r.c.runs.iter().find(|x| x.text == t).map(|x| x.ink);
            assert!(said(&r, "EXPERIMENT ON").is_none() && !r.c.runs.iter().any(|x| x.text.contains("EXPERIMENTS ON")), "at {user}: a mark in the default game");
            r.m.trying = vec!["Shake and hit-stop on".into()];
            r.draw();
            let ink = said(&r, "EXPERIMENT ON").unwrap_or_else(|| panic!("at {user}: no mark with a switch on"));
            // Under the top bar, at the right, on the screen, and touching
            // no other word.
            assert!(ink.1 >= r.hud.top && ink.2 <= 1920.0 && ink.0 >= 1920.0 * 0.75 && ink.3 <= 1080.0 * 0.2, "at {user}: the mark at {ink:?}, the top bar ends at {}", r.hud.top);
            for x in r.c.runs.iter().filter(|x| x.text != "EXPERIMENT ON" && x.ink.0 < x.ink.2) {
                let apart = ink.2 <= x.ink.0 || x.ink.2 <= ink.0 || ink.3 <= x.ink.1 || x.ink.3 <= ink.1;
                assert!(apart, "at {user}: the mark {ink:?} over {:?} {:?}", x.text, x.ink);
            }
            // On it, the cursor is over the HUD (a click does not reach the
            // world), and the tip names what is on.
            r.at(((ink.0 + ink.2) / 2.0, (ink.1 + ink.3) / 2.0));
            r.draw();
            assert!(r.c.runs.iter().any(|x| x.text == "Not the default game: Shake and hit-stop on. Esc, Experiments to change"), "at {user}: no tip");
            assert!(r.click(Button::Left, true).0, "at {user}: a click on the mark went to the world");
            r.click(Button::Left, false);
            // Two are counted.
            r.m.trying.push("Crystals refill on".into());
            r.draw();
            assert!(said(&r, "2 EXPERIMENTS ON").is_some(), "at {user}");
        }
    }

    /// The goal never covers the top bar's numbers, the clock, the
    /// experiments' mark or an alert, at every UI size on three screens
    /// (11-18: at 200% on 1080 lines, centred on the screen, it covered
    /// the resources' fourth number).
    #[test]
    fn the_goal_keeps_off_the_top_bars_numbers() {
        let goal = "Wipe out the slime: kill every last walker (counting from 1:00; slime count 40,451)";
        for screen in [(1280, 800), (1920, 1080), (2560, 1440)] {
            for user in crate::world::settings::STEPS {
                let mut r = Rig::new(screen);
                r.c.ui.user = user;
                let rates = [Some("+23/s"), Some("-8 lost"), None, None];
                r.m.resources = ["2,091", "11", "40.4k", "95%"].iter().zip(rates).map(|(v, rate)| Resource { icon: Icon::Clock, colour: TEXT, label: "a number".into(), value: (*v).into(), rate: rate.map(String::from) }).collect();
                r.m.clock = "1:00 / 18:00".into();
                r.m.speed = Some("Paused".into());
                r.m.objective = Some(goal.into());
                r.m.trying = vec!["Shake and hit-stop on".into()];
                r.hud.alert("A soldier has fallen", None, WARN);
                r.draw();
                let g = r.c.runs.iter().find(|x| x.text.starts_with("Wipe out")).unwrap_or_else(|| panic!("{screen:?} at {user}: no goal")).ink;
                assert!(g.0 >= 0.0 && g.2 <= screen.0 as f32, "{screen:?} at {user}: the goal off the screen: {g:?}");
                for x in r.c.runs.iter().filter(|x| !x.text.starts_with("Wipe out") && x.ink.0 < x.ink.2) {
                    let apart = g.2 <= x.ink.0 || x.ink.2 <= g.0 || g.3 <= x.ink.1 || x.ink.3 <= g.1;
                    assert!(apart, "{screen:?} at {user}: the goal {g:?} over {:?} {:?}", x.text, x.ink);
                }
            }
        }
    }

    #[test]
    fn space_is_always_the_alerts_key() {
        let mut r = Rig::new((1920, 1080));
        assert!(r.key(KeyCode::Space).0, "with no alert");
        r.hud.alert("here", Some((100.0, 100.0)), WARN);
        assert!(r.key(KeyCode::Space).0);
    }

    #[test]
    fn a_tiny_window_draws_without_panicking() {
        for screen in [(320, 200), (200, 120), (1, 1)] {
            let mut r = Rig::new(screen);
            if let Some(b) = r.hud.command_rect(NOW) {
                r.at((b.x + 2.0, b.y + 2.0));
            }
            r.draw();
        }
    }

    #[test]
    fn the_scale_steps_so_resizing_reuses_glyphs() {
        let mut r = Rig::new((1920, 1081));
        assert_eq!(r.hud.scale, 1.0);
        r.c.screen = (1920, 1123);
        r.draw();
        assert_eq!(r.hud.scale, 1.05);
    }
}
