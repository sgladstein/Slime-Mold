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
const CELL_GAP: f32 = 5.0;
const CARD_PAD: f32 = 8.0;
/// The command card's grid. 4 x 3 until the owner's ask of 11-14 08:33Z
/// ("re-arrange the bottom menu to be one row for the squad and two rows
/// for the other buttons so the whole thing is wider but shorter"): 6 x 2,
/// the 4 x 3 slots read in order (`card_slot`), so the bar went from 160
/// to 113 units at 1080 lines (15% to 10% of the screen).
const CARD_COLS: u8 = 6;
const CARD_ROWS: u8 = 2;
/// The minimap keeps the bar's old height (152 units) in the corner,
/// standing above the shorter bar, so it does not shrink with it.
const MINIMAP_H: f32 = 3.0 * CELL + 2.0 * CELL_GAP + 2.0 * CARD_PAD;

/// The loadout strip's width: its slots, the name and state, and Swap.
fn loadout_w(l: &Loadout, s: f32) -> f32 {
    let slots = l.slots.max(l.cards.len()).max(1) as f32;
    2.0 * 6.0 * s + slots * 34.0 * s + (slots - 1.0) * 4.0 * s + 8.0 * s + 112.0 * s + if l.cycle.is_some() { 26.0 * s + 6.0 * s } else { 0.0 }
}

/// A command's place on the 6 x 2 card from its 4 x 3 `slot` (row, col):
/// the twelve slots in reading order, wrapped at six.
fn card_slot(slot: (u8, u8)) -> (u8, u8) {
    let k = slot.0.min(2) * 4 + slot.1.min(3);
    (k / CARD_COLS, k % CARD_COLS)
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
    /// Row (0-2) and column (0-3) on the card.
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
}

impl Command {
    pub fn new(id: u32, slot: (u8, u8), icon: Icon, name: &str, key: KeyCode) -> Self {
        Command { id, slot, icon, name: name.into(), key, cost: None, cost_icon: Icon::Crystal, ready: true, on: false, cooldown: 0.0, wait: None, target: Target::Instant, about: String::new(), why_not: None }
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
            CardKind::Payload => "Payload",
            CardKind::Modifier => "Modifier",
            CardKind::Trigger => "Trigger",
            CardKind::Flight => "Flight",
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
    pub name: String,
    pub kind: CardKind,
    /// The glyph's colour: the colour of the payload it goes with, as its
    /// bullets fly.
    pub colour: Rgba,
    /// What it does, for its tooltip.
    pub about: String,
    /// A payload's price a cast ("40").
    pub cost: Option<String>,
    /// In the cast the gun's next hit fires: lit.
    pub next: bool,
    /// After the last payload: it weighs on the recharge and casts nothing.
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
    /// The recharge after the last cast, seconds.
    pub recharge: f32,
    /// How much of it is still to go, 0 (ready) to 1 (just spent).
    pub charging: f32,
    /// A line under the cards ("3 of 5 guns ready"); the game's words.
    pub note: Option<String>,
    /// The command that swaps the loadout, and its key: the panel's
    /// button sends `HudEvent::Command` with it.
    pub cycle: Option<(u32, KeyCode)>,
}

/// What the game shows this frame.
#[derive(Clone, Debug, PartialEq, Default)]
pub struct Model {
    pub resources: Vec<Resource>,
    pub clock: String,
    /// "Paused", "x4": a chip beside the clock.
    pub speed: Option<String>,
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
    /// A small card by the cursor for what is under it (an enemy, the
    /// mould), leaving the selection panel alone.
    pub hover: Option<Info>,
    /// The round is over: a title and lines.
    pub end: Option<(String, Vec<String>)>,
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
    /// The gun's cards, drawn as a strip on the selection panel's top edge.
    pub loadout: Option<Loadout>,
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

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum MenuItem {
    Opened,
    Resume,
    Restart,
    /// Back to the start menu (`Model::main_menu`).
    MainMenu,
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
    /// A command was pressed that is not ready; the hud shows why.
    NotReady { id: u32 },
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
    Back,
    /// The loadout panel's swap button.
    Cycle,
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
    /// The world's size in cells, as last drawn (for the minimap).
    world: (f32, f32),
    /// The end-of-round screen is up (its buttons take every click).
    end: bool,
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
}

impl Default for Hud {
    fn default() -> Self {
        Self::new()
    }
}

impl Hud {
    pub fn new() -> Self {
        Hud { scale: 1.0, hits: Vec::new(), hover: None, pressed: None, targeting: None, menu: None, help_direct: false, alerts: Vec::new(), commands: Vec::new(), bottom: 0.0, minimap_box: None, minimap: None, top: 0.0, world: (1.0, 1.0), end: false, took: [false; 3], ghost_ok: true, ghost_note: None, cycle: None, started: Instant::now() }
    }

    /// The menu is open (the game should pause).
    pub fn menu_open(&self) -> bool {
        self.menu.is_some()
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
        // The end screen takes every key (only its buttons act).
        if self.end {
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
                Some(MenuPage::Main) => {
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
                Some(MenuPage::Help) => self.menu = Some(MenuPage::Main),
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
        let pad = CARD_PAD * s;

        // The minimap's frame (bottom right), the command card left of it,
        // the selection panel left of that, all one height.
        let cell = CELL * s;
        let gap = CELL_GAP * s;
        let card_w = CARD_COLS as f32 * cell + (CARD_COLS - 1) as f32 * gap + 2.0 * pad;
        let (_, inset_bottom) = Hud::insets(c.screen, c.ui);
        let panel_h = inset_bottom - 8.0 * s;
        let base = sh - 8.0 * s;
        let top_of = base - panel_h;
        self.bottom = top_of;
        let radius = 12.0 * s;
        let mut right = sw - 8.0 * s;
        self.minimap_box = None;
        self.minimap = None;
        if minimap {
            let mm_h = (MINIMAP_H * s).max(panel_h);
            let ih = mm_h - 2.0 * pad;
            // (min then max, not `clamp`, which panics in a narrow window.)
            let iw = (ih * cam.world.0 / cam.world.1).min((360.0 * s).min(sw * 0.25)).max(ih * 0.6);
            let f = Rect::new(right - iw - 2.0 * pad, base - mm_h, iw + 2.0 * pad, mm_h);
            let bx = ((f.x + pad).round() as u32, (f.y + pad).round() as u32, iw.round() as u32, ih.round() as u32);
            let mm = super::view::minimap_in((cam.world.0 as usize, cam.world.1 as usize), bx);
            self.minimap_box = Some(bx);
            self.minimap = Some(mm);
            let hole = Rect::new(mm.0 as f32, mm.1 as f32, mm.2 as f32, mm.3 as f32);
            c.shadow(f, radius, 16.0 * s, 4.0 * s, SHADOW);
            // A panel round the minimap, open over it (the minimap is the
            // view's, drawn before the canvas).
            c.panel_with_hole(f, radius, PANEL_TOP, PANEL_BOTTOM, 1.0, EDGE, hole);
            c.outline(Rect::new(hole.x - 1.0, hole.y - 1.0, hole.w + 2.0, hole.h + 2.0), 2.0, 1.0, [255, 255, 255, 40]);
            // Clicks on the frame are the UI's; inside, the controls'.
            for r in [Rect::new(f.x, f.y, f.w, hole.y - f.y), Rect::new(f.x, hole.bottom(), f.w, f.bottom() - hole.bottom()), Rect::new(f.x, f.y, hole.x - f.x, f.h), Rect::new(hole.right(), f.y, f.right() - hole.right(), f.h)] {
                self.hits.push((r, Hit::Panel));
            }
            right = f.x - 8.0 * s;
        }
        let card = Rect::new(right - card_w, top_of, card_w, panel_h);
        // The gun cards' panel at the bar's far left (owner, 08:46Z: "gun/card
        // menu should go on the far left to fill the rest of the screen"),
        // the selection panel filling the row between it and the command
        // card (one card row tall, owner 08:33Z). Where that leaves the
        // selection under 240 units the strip goes back over the panels.
        let gun_w = m.loadout.as_ref().map(|l| loadout_w(l, s));
        let gun = gun_w.map(|w| Rect::new(8.0 * s, top_of, w, panel_h)).filter(|g| card.x - 8.0 * s - (g.right() + 8.0 * s) >= 240.0 * s);
        let left = gun.map_or(8.0 * s, |g| g.right() + 8.0 * s);
        let sel = Rect::new(left, top_of, (card.x - 8.0 * s - left).max(0.0), panel_h);
        // With nothing selected the selection panel collapses to a strip
        // along the bottom (the art director, 11-14 02:34Z: a full-height
        // panel that only said "Nothing selected").
        let empty = m.info.is_none() && m.selection.is_empty();
        // The loadout strip (about 312 units wide) takes the rest of the row.
        let sel_drawn = if empty { Rect::new(sel.x, base - COLLAPSED * s, (sel.w - 320.0 * s).clamp(150.0 * s, 330.0 * s).min(sel.w), COLLAPSED * s) } else { sel };

        self.top_bar(m, c, s);
        self.selection_panel(m, controls, sel_drawn, c, s);
        // Before the command card, so the card's tooltips cover it; its
        // own card tooltip after.
        self.cycle = m.loadout.as_ref().and_then(|l| l.cycle.map(|c| c.0));
        let tip = m.loadout.as_ref().and_then(|l| self.loadout_draw(l, gun, sel, empty, card, c, s));
        self.command_card(m, card, c, s);
        match (&m.loadout, tip) {
            (Some(l), Some((k, r))) if k < l.cards.len() => self.card_tip(&l.cards[k], r, c, s),
            (Some(l), Some((k, r))) => self.loadout_tip(l, r, k == usize::MAX - 1, c, s),
            _ => {}
        }
        self.alerts_draw(c, world, s);
        self.ghost(m, cam, c, world, s);
        if let (Some(h), Some(p), None) = (&m.hover, self.hover, self.targeting) {
            self.hover_card(h, p, c, s);
        }
        self.end = m.end.is_some();
        if let Some((title, lines)) = &m.end {
            self.end_screen(title, lines, m.main_menu, c, s);
        } else if let Some(page) = self.menu {
            self.menu_draw(page, m, c, s);
        }
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
        let px = 15.0 * s;
        let small = 12.0 * s;
        let base = y + h / 2.0 + 5.0 * s;
        // Resources: a pill of icon, value and rate each.
        let mut widths = Vec::new();
        for r in &m.resources {
            let w = 18.0 * s + 6.0 * s + c.atlas.measure(Weight::SemiBold, px, &r.value) + r.rate.as_ref().map(|t| 6.0 * s + c.atlas.measure(Weight::Medium, small, t)).unwrap_or(0.0);
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
                let vw = c.text(Weight::SemiBold, px, (tx, base), Align::Left, &r.value, TEXT);
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
        let cw = c.atlas.measure(Weight::SemiBold, px, &m.clock);
        let chip = m.speed.as_ref().map(|t| c.atlas.measure(Weight::SemiBold, small, &t.to_uppercase()) + 16.0 * s).unwrap_or(0.0);
        let w = 14.0 * s + 18.0 * s + 6.0 * s + cw + if chip > 0.0 { 10.0 * s + chip } else { 0.0 } + 14.0 * s;
        let pill = Rect::new(menu.x - 8.0 * s - w, y, w, h);
        self.glass(pill, h / 2.0, c, s);
        let mut x = pill.x + 14.0 * s;
        c.icon(Icon::Clock, (x, y + (h - 18.0 * s) / 2.0), 18.0 * s, MUTED);
        x += 24.0 * s;
        x += c.text(Weight::SemiBold, px, (x, base), Align::Left, &m.clock, TEXT) + 10.0 * s;
        if let Some(sp) = &m.speed {
            let r = Rect::new(x, y + 9.0 * s, chip, h - 18.0 * s);
            c.rrect(r, r.h / 2.0, alpha(WARN, 0.18));
            c.text(Weight::SemiBold, small, (r.x + r.w / 2.0, base - 1.0 * s), Align::Centre, &sp.to_uppercase(), WARN);
        }
        // The objective, centred.
        if let Some(obj) = &m.objective {
            let ow = c.atlas.measure(Weight::Medium, 13.0 * s, obj) + 40.0 * s;
            let r = Rect::new((sw - ow) / 2.0, y + 3.0 * s, ow, h - 6.0 * s);
            self.glass(r, r.h / 2.0, c, s);
            c.icon(Icon::Target, (r.x + 12.0 * s, r.y + (r.h - 14.0 * s) / 2.0), 14.0 * s, ACCENT);
            c.text(Weight::Medium, 13.0 * s, (r.x + 30.0 * s, base - 1.0 * s), Align::Left, obj, TEXT);
        }
    }

    fn tip_small(&mut self, text: &str, at: (f32, f32), c: &mut Canvas, s: f32) {
        let w = c.atlas.measure(Weight::Medium, 12.0 * s, text) + 16.0 * s;
        let r = Rect::new(at.0, at.1, w, 24.0 * s);
        c.shadow(r, 6.0 * s, 10.0 * s, 2.0 * s, SHADOW);
        c.panel(r, 6.0 * s, [36, 42, 50, 245], [28, 33, 40, 245], 1.0, EDGE);
        c.text(Weight::Medium, 12.0 * s, (r.x + 8.0 * s, r.y + 16.0 * s), Align::Left, text, TEXT);
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
            let label = format!("{}", m.idle);
            let w = 34.0 * s + c.atlas.measure(Weight::SemiBold, 12.0 * s, &label);
            let chip = Rect::new(x, y, w, chip_h);
            self.chip(chip, Hit::Idle, false, c, s);
            c.icon(Icon::Idle, (chip.x + 8.0 * s, chip.y + 4.0 * s), 16.0 * s, WARN);
            c.text(Weight::SemiBold, 12.0 * s, (chip.x + 28.0 * s, chip.y + 16.5 * s), Align::Left, &label, TEXT);
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
        if let Some(info) = &m.info {
            self.info_draw(info, inner, c, s);
            return;
        }
        if m.selection.is_empty() {
            // The collapsed strip: one line.
            let mid = r.y + r.h / 2.0;
            c.icon(Icon::Soldier, (r.x + 14.0 * s, mid - 9.0 * s), 18.0 * s, FAINT);
            let tw = c.text(Weight::SemiBold, 13.0 * s, (r.x + 40.0 * s, mid + 4.5 * s), Align::Left, "Nothing selected", MUTED);
            c.text_fit(Weight::Medium, 12.0 * s, (r.x + 48.0 * s + tw, mid + 4.5 * s), Align::Left, "click or drag to select", r.w - 62.0 * s - tw, FAINT);
            return;
        }
        // Every selected unit as a card.
        let n = m.selection.len();
        let mean = m.selection.iter().map(|q| q.hp).sum::<f32>() / n as f32;
        let hurt = m.selection.iter().filter(|q| q.hp < 0.6).count();
        let head = format!("{n} selected");
        let hw = c.text(Weight::SemiBold, 15.0 * s, (inner.x, inner.y + 14.0 * s), Align::Left, &head, TEXT);
        let sub = if hurt > 0 { format!("{:.0}% health on average, {hurt} hurt", mean * 100.0) } else { format!("{:.0}% health on average", mean * 100.0) };
        c.text(Weight::Medium, 12.5 * s, (inner.x + hw + 12.0 * s, inner.y + 14.0 * s), Align::Left, &sub, MUTED);
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
        c.text_fit(Weight::Medium, 12.5 * s, (x, r.y + 36.0 * s), Align::Left, &info.subtitle, w, MUTED);
        let mut y = r.y + 50.0 * s;
        if let Some((f, words)) = &info.hp {
            c.icon(Icon::Heart, (x, y - 1.0 * s), 14.0 * s, health_colour(*f));
            let bw = (w - 110.0 * s).max(60.0 * s);
            c.bar(Rect::new(x + 20.0 * s, y + 3.0 * s, bw, 7.0 * s), *f, health_colour(*f), TRACK);
            c.text(Weight::SemiBold, 12.5 * s, (x + 28.0 * s + bw, y + 11.0 * s), Align::Left, words, TEXT);
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
            let lw = c.text(Weight::Medium, 12.0 * s, (sx, sy), Align::Left, label, MUTED);
            c.text_fit(Weight::SemiBold, 12.5 * s, (sx + lw + 8.0 * s, sy), Align::Left, value, colw - lw - 16.0 * s, TEXT);
        }
        if let Some(n) = &info.note {
            c.text_fit(Weight::Medium, 12.0 * s, (x, r.bottom() - 2.0 * s), Align::Left, n, w, WARN);
        }
    }

    fn command_card(&mut self, m: &Model, r: Rect, c: &mut Canvas, s: f32) {
        self.glass(r, 12.0 * s, c, s);
        let pad = CARD_PAD * s;
        let cell = CELL * s;
        let gap = CELL_GAP * s;
        let x0 = r.x + pad;
        let y0 = r.bottom() - pad - CARD_ROWS as f32 * cell - (CARD_ROWS - 1) as f32 * gap;
        // Empty slots, faintly, so the grid reads as a grid.
        for j in 0..CARD_ROWS {
            for i in 0..CARD_COLS {
                let b = Rect::new(x0 + i as f32 * (cell + gap), y0 + j as f32 * (cell + gap), cell, cell);
                c.rrect(b, 10.0 * s, [255, 255, 255, 8]);
            }
        }
        let mut tip = None;
        for (k, cmd) in m.commands.iter().enumerate() {
            let (row, col) = card_slot(cmd.slot);
            let (row, col) = (row as f32, col as f32);
            let b = Rect::new(x0 + col * (cell + gap), y0 + row * (cell + gap), cell, cell);
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
            let ic = 22.0 * s;
            c.icon(cmd.icon, (b.x + (cell - ic) / 2.0, b.y + (cell - ic) / 2.0 - 2.0 * s), ic, alpha(if cmd.on { ACCENT } else { TEXT }, dim));
            // The hotkey, top left.
            if let Some(k) = key_name(cmd.key) {
                c.text(Weight::SemiBold, 10.0 * s, (b.x + 4.5 * s, b.y + 11.5 * s), Align::Left, &k, alpha(TEXT, 0.72 * dim.max(0.6)));
            }
            if let Some(cost) = &cmd.cost {
                let cw = c.atlas.measure(Weight::SemiBold, 9.5 * s, cost);
                let cx = b.right() - 4.0 * s - cw;
                c.text(Weight::SemiBold, 9.5 * s, (cx, b.bottom() - 3.5 * s), Align::Left, cost, alpha(if cmd.ready { ENERGY } else { BAD }, 0.95));
                c.icon(cmd.cost_icon, (cx - 10.0 * s, b.bottom() - 12.0 * s), 8.5 * s, alpha(ENERGY, dim.max(0.6)));
            }
            if cmd.cooldown > 0.0 {
                // A dark wipe over the part still to go (a ring round the
                // edge read as a broken border), and the time left.
                c.ring(b, 10.0 * s, cell, cmd.cooldown.min(1.0), [0, 0, 0, 150]);
                if let Some(wt) = &cmd.wait {
                    c.text(Weight::SemiBold, 15.0 * s, (b.x + cell / 2.0, b.y + cell / 2.0 + 5.0 * s), Align::Centre, wt, TEXT);
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
    /// left to right, the cast its next hit fires lit, the recharge as a
    /// thin bar under them, the loadout's name and state, and Swap. It sits
    /// on the selection panel's top edge at the right (the art director,
    /// 11-14 02:34Z: a strip of card icons next to the selection, not a
    /// tall box; the details in a tooltip). Before that review it was a
    /// 140-pixel box over the command card with each card's name under its
    /// glyph. Returns the hovered card (or `usize::MAX` for the name and
    /// state, whose tooltip is the whole loadout) and its rectangle.
    #[allow(clippy::too_many_arguments)]
    fn loadout_draw(&mut self, l: &Loadout, panel: Option<Rect>, sel: Rect, collapsed: bool, card: Rect, c: &mut Canvas, s: f32) -> Option<(usize, Rect)> {
        let pad = 6.0 * s;
        let cell = 34.0 * s;
        let gap = 4.0 * s;
        let slots = l.slots.max(l.cards.len()).max(1);
        let info_w = 112.0 * s;
        let swap = 26.0 * s;
        let w = loadout_w(l, s);
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
        for i in 0..slots {
            let b = Rect::new(r.x + pad + i as f32 * (cell + gap), y, cell, cell);
            let Some(k) = l.cards.get(i) else {
                c.rrect(b, 7.0 * s, [255, 255, 255, 8]);
                c.outline(b, 7.0 * s, 1.0, [255, 255, 255, 18]);
                continue;
            };
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
            let ic = 20.0 * s;
            if let Some(icon) = k.icon {
                c.icon(icon, (b.x + (b.w - ic) / 2.0, b.y + (b.h - ic) / 2.0 + 1.5 * s), ic, alpha(k.colour, dim));
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
        // The recharge: a thin bar under the cards that fills back up.
        let cards_w = slots as f32 * cell + (slots - 1) as f32 * gap;
        let ready = l.charging <= 0.0;
        if !l.cards.is_empty() {
            c.bar(Rect::new(r.x + pad, y + cell + 3.0 * s, cards_w, 2.5 * s), (1.0 - l.charging).clamp(0.0, 1.0), if ready { GOOD } else { WARN }, TRACK);
        }
        // The name and the state.
        let tx = r.x + pad + cards_w + 8.0 * s;
        c.text_fit(Weight::SemiBold, 13.0 * s, (tx, y + 14.0 * s), Align::Left, &l.name, info_w, TEXT);
        let (state, col) = if l.cards.is_empty() {
            ("Plain shots".to_string(), MUTED)
        } else if ready {
            ("Ready".to_string(), GOOD)
        } else {
            (format!("Recharging {:.1} s", l.charging * l.recharge), WARN)
        };
        c.text_fit(Weight::Medium, 11.0 * s, (tx, y + 30.0 * s), Align::Left, &state, info_w, col);
        let words = Rect::new(tx - 4.0 * s, r.y, info_w + 4.0 * s, r.h);
        if tip.is_none() && self.hover.is_some_and(|p| words.contains(p)) {
            tip = Some((usize::MAX, words));
        }
        if let Some((_, key)) = l.cycle {
            let b = Rect::new(r.right() - pad - swap, y + (cell - swap) / 2.0, swap, swap);
            let hot = self.hover.is_some_and(|p| b.contains(p));
            let (t, bt) = if hot { (BUTTON_HOVER_TOP, BUTTON_HOVER_BOTTOM) } else { (BUTTON_TOP, BUTTON_BOTTOM) };
            c.panel(b, 6.0 * s, t, bt, 1.0, [255, 255, 255, 30]);
            let k = key_name(key).unwrap_or_else(|| "~".into());
            c.text(Weight::SemiBold, 12.0 * s, (b.x + b.w / 2.0, b.y + b.h / 2.0 + 4.5 * s), Align::Centre, &k, TEXT);
            self.hits.push((b, Hit::Cycle));
            if hot {
                tip = Some((usize::MAX - 1, b));
            }
        }
        tip
    }

    /// The loadout's own tooltip (over its name, or Swap): whose gun, the
    /// guns ready, the recharge, and how to swap.
    fn loadout_tip(&mut self, l: &Loadout, b: Rect, swap: bool, c: &mut Canvas, s: f32) {
        let mut lines = vec![l.whose.clone()];
        if let Some(n) = &l.note {
            lines.push(n.clone());
        } else if !l.cards.is_empty() {
            lines.push(format!("Recharge {:.1} s after the last cast", l.recharge));
        }
        if let Some((_, key)) = l.cycle {
            lines.push(match key_name(key) {
                Some(k) if swap => format!("Click or press {k}: the next loadout"),
                Some(k) => format!("{k} or the button: the next loadout"),
                None => "The button: the next loadout".into(),
            });
        }
        let w = 260.0 * s;
        let h = 40.0 * s + lines.len() as f32 * 18.0 * s;
        let sw = c.screen.0 as f32;
        let x = (b.x + b.w / 2.0 - w / 2.0).min(sw - 8.0 * s - w).max(8.0 * s);
        let r = Rect::new(x, (b.y - 14.0 * s - h).max(8.0 * s), w, h);
        c.shadow(r, 10.0 * s, 18.0 * s, 4.0 * s, SHADOW);
        c.panel(r, 10.0 * s, [34, 40, 49, 248], [24, 29, 36, 250], 1.0, EDGE);
        let mut y = r.y + 26.0 * s;
        c.icon(Icon::Rifle, (r.x + 14.0 * s, y - 14.0 * s), 16.0 * s, MUTED);
        c.text(Weight::SemiBold, 15.0 * s, (r.x + 38.0 * s, y), Align::Left, &l.name, TEXT);
        y += 22.0 * s;
        for t in &lines {
            c.text_fit(Weight::Medium, 12.5 * s, (r.x + 14.0 * s, y), Align::Left, t, w - 28.0 * s, MUTED);
            y += 18.0 * s;
        }
    }

    /// A gun card's tooltip, above the loadout strip.
    fn card_tip(&mut self, k: &GunCard, b: Rect, c: &mut Canvas, s: f32) {
        let w = 290.0 * s;
        let px = 12.5 * s;
        let lines = c.wrap(Weight::Medium, px, w - 28.0 * s, &k.about);
        let extra = if k.idle { 22.0 * s } else { 0.0 };
        let h = 18.0 * s + 22.0 * s + lines.len() as f32 * 18.0 * s + 10.0 * s + extra;
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
        if let Some(cost) = &k.cost {
            let cw = c.atlas.measure(Weight::SemiBold, 12.0 * s, cost);
            let cx = kr.x - 10.0 * s - cw;
            c.icon(Icon::Crystal, (cx - 16.0 * s, y - 12.0 * s), 13.0 * s, ENERGY);
            c.text(Weight::SemiBold, 12.0 * s, (cx, y - 1.0 * s), Align::Left, cost, ENERGY);
        }
        y += 22.0 * s;
        for l in &lines {
            c.text(Weight::Medium, px, (r.x + 14.0 * s, y), Align::Left, l, MUTED);
            y += 18.0 * s;
        }
        if k.idle {
            c.text_fit(Weight::SemiBold, 12.0 * s, (r.x + 14.0 * s, y + 4.0 * s), Align::Left, "After the last payload: it only slows the recharge", w - 28.0 * s, WARN);
        }
    }

    fn tooltip(&mut self, cmd: &Command, b: Rect, card: Rect, c: &mut Canvas, s: f32) {
        let w = 300.0 * s;
        let px = 12.5 * s;
        let lines = c.wrap(Weight::Medium, px, w - 28.0 * s, &cmd.about);
        let why = if cmd.ready { None } else { cmd.why_not.clone() };
        let h = 18.0 * s + 22.0 * s + if cmd.about.is_empty() { 0.0 } else { lines.len() as f32 * 18.0 * s + 8.0 * s } + if why.is_some() { 20.0 * s } else { 0.0 } + 10.0 * s;
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
                y += 18.0 * s;
            }
            y += 6.0 * s;
        }
        if let Some(wn) = why {
            c.text_fit(Weight::SemiBold, 12.5 * s, (r.x + 14.0 * s, y), Align::Left, &wn, w - 28.0 * s, BAD);
        }
        let hint = match cmd.target {
            Target::Instant => None,
            Target::Line { .. } => Some("Drag to lay it"),
            _ => Some("Click to place it"),
        };
        if let (Some(hn), true) = (hint, cmd.ready) {
            c.text(Weight::Medium, 11.0 * s, (r.right() - 14.0 * s, r.bottom() - 9.0 * s), Align::Right, hn, FAINT);
        }
    }

    fn alerts_draw(&mut self, c: &mut Canvas, world: &mut Marks, s: f32) {
        let now = Instant::now();
        let mut y = self.top + 12.0 * s;
        let alerts = self.alerts.clone();
        for a in alerts.iter().rev() {
            let age = now.duration_since(a.when).as_secs_f32();
            let life = ALERT_LIFE.as_secs_f32();
            let fade = ((life - age) / ALERT_FADE).clamp(0.0, 1.0);
            let w = c.atlas.measure(Weight::Medium, 13.0 * s, &a.text) + 46.0 * s;
            let r = Rect::new(8.0 * s, y, w, 32.0 * s);
            c.shadow(r, 8.0 * s, 12.0 * s, 3.0 * s, alpha(SHADOW, fade));
            c.panel(r, 8.0 * s, alpha(PANEL_TOP, fade), alpha(PANEL_BOTTOM, fade), 1.0, alpha(EDGE, fade));
            c.rrect(Rect::new(r.x, r.y + 6.0 * s, 3.0 * s, r.h - 12.0 * s), 1.5 * s, alpha(a.colour, fade));
            c.icon(Icon::Alert, (r.x + 12.0 * s, r.y + 8.0 * s), 16.0 * s, alpha(a.colour, fade));
            c.text(Weight::Medium, 13.0 * s, (r.x + 34.0 * s, r.y + 21.0 * s), Align::Left, &a.text, alpha(TEXT, fade));
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
        let col = if m.ghost_ok { [96, 230, 150, 255] } else { [245, 96, 96, 255] };
        let fill = alpha(col, 0.28);
        match t.target {
            Target::Point { radius } => {
                world.disc(at, radius, fill, false);
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
    fn hover_card(&mut self, info: &Info, p: (f32, f32), c: &mut Canvas, s: f32) {
        let (sw, sh) = (c.screen.0 as f32, c.screen.1 as f32);
        let stats = info.stats.iter().map(|(l, v)| format!("{l} {v}")).collect::<Vec<_>>().join("  ·  ");
        let w = (c.atlas.measure(Weight::SemiBold, 14.0 * s, &info.title).max(c.atlas.measure(Weight::Medium, 12.0 * s, &stats)).max(c.atlas.measure(Weight::Medium, 12.0 * s, &info.subtitle)) + 28.0 * s).max(200.0 * s);
        let h = 22.0 * s + 18.0 * s + if info.hp.is_some() { 18.0 * s } else { 0.0 } + if stats.is_empty() { 0.0 } else { 18.0 * s } + 12.0 * s;
        // Below right of the cursor, kept on the screen and off the panels.
        let x = (p.0 + 20.0 * s).min(sw - w - 8.0 * s).max(8.0 * s);
        let y = (p.1 + 20.0 * s).min(self.bottom - h - 8.0 * s).min(sh - h - 8.0 * s).max(self.top + 8.0 * s);
        let r = Rect::new(x, y, w, h);
        c.shadow(r, 10.0 * s, 14.0 * s, 3.0 * s, SHADOW);
        c.panel(r, 10.0 * s, [34, 40, 49, 245], [24, 29, 36, 248], 1.0, alpha(info.colour, 0.5));
        let mut ty = r.y + 22.0 * s;
        c.icon(info.icon, (r.x + 12.0 * s, ty - 14.0 * s), 16.0 * s, info.colour);
        c.text(Weight::SemiBold, 14.0 * s, (r.x + 34.0 * s, ty), Align::Left, &info.title, TEXT);
        ty += 18.0 * s;
        c.text_fit(Weight::Medium, 12.0 * s, (r.x + 14.0 * s, ty), Align::Left, &info.subtitle, w - 28.0 * s, MUTED);
        if let Some((f, words)) = &info.hp {
            ty += 18.0 * s;
            let ww = c.atlas.measure(Weight::SemiBold, 12.0 * s, words);
            c.bar(Rect::new(r.x + 14.0 * s, ty - 8.0 * s, w - 40.0 * s - ww, 6.0 * s), *f, health_colour(*f), TRACK);
            c.text(Weight::SemiBold, 12.0 * s, (r.right() - 14.0 * s, ty), Align::Right, words, TEXT);
        }
        if !stats.is_empty() {
            ty += 18.0 * s;
            c.text_fit(Weight::Medium, 12.0 * s, (r.x + 14.0 * s, ty), Align::Left, &stats, w - 28.0 * s, MUTED);
        }
    }

    fn modal(&mut self, w: f32, h: f32, c: &mut Canvas, s: f32) -> Rect {
        let (sw, sh) = (c.screen.0 as f32, c.screen.1 as f32);
        c.rrect(Rect::new(0.0, 0.0, sw, sh), 0.0, [4, 6, 8, 150]);
        let r = Rect::new((sw - w) / 2.0, (sh - h) / 2.0, w, h);
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
        c.text(Weight::SemiBold, 14.0 * s, (r.x + r.w / 2.0, r.y + r.h / 2.0 + 5.0 * s), Align::Centre, label, TEXT);
        self.hits.push((r, hit));
    }

    fn menu_draw(&mut self, page: MenuPage, m: &Model, c: &mut Canvas, s: f32) {
        match page {
            MenuPage::Main => {
                let w = 340.0 * s;
                let bh = 42.0 * s;
                let mut items: Vec<(&str, Hit, bool)> = vec![("Resume", Hit::Menu(MenuItem::Resume), true), ("Controls", Hit::Help, false), ("Restart", Hit::Menu(MenuItem::Restart), false)];
                if m.main_menu {
                    items.push(("Main menu", Hit::Menu(MenuItem::MainMenu), false));
                }
                items.push(("Quit", Hit::Menu(MenuItem::Quit), false));
                let h = 84.0 * s + items.len() as f32 * (bh + 10.0 * s) + 14.0 * s;
                let r = self.modal(w, h, c, s);
                c.text(Weight::SemiBold, 22.0 * s, (r.x + w / 2.0, r.y + 44.0 * s), Align::Centre, "Paused", TEXT);
                c.text(Weight::Medium, 12.5 * s, (r.x + w / 2.0, r.y + 64.0 * s), Align::Centre, "Esc to resume", MUTED);
                let mut y = r.y + 84.0 * s;
                for (label, hit, primary) in items {
                    self.wide_button(Rect::new(r.x + 24.0 * s, y, w - 48.0 * s, bh), label, hit, primary, c, s);
                    y += bh + 10.0 * s;
                }
            }
            MenuPage::Help => {
                let mut rows: Vec<(String, String)> = if let Some(base) = &m.help_base {
                    let mut b = base.clone();
                    b.push(("Esc".into(), "Cancel placing, or this menu".into()));
                    b.push(("F1 or ?".into(), "This key card".into()));
                    b.push(("Ctrl + / Ctrl -".into(), "Bigger or smaller UI and text".into()));
                    b
                } else { vec![
                    ("Left click / drag".into(), "Select soldiers".into()),
                    ("Right click".into(), "Move, or work the crystal or mould clicked".into()),
                    ("Shift".into(), "Add to the selection, or keep placing".into()),
                    ("Ctrl + 1-9".into(), "Make a group; 1-9 selects it, twice to look".into()),
                    ("Wheel, middle drag".into(), "Zoom, pan; the screen's edge scrolls".into()),
                    ("Minimap".into(), "Click to look there; right click to send".into()),
                    ("Space".into(), "Look at the newest alert".into()),
                    ("Esc".into(), "Cancel placing, or this menu".into()),
                    ("F1 or ?".into(), "This key card".into()),
                    ("Ctrl + / Ctrl -".into(), "Bigger or smaller UI and text".into()),
                ] };
                for cmd in &m.commands {
                    if let Some(k) = key_name(cmd.key) {
                        rows.push((k, cmd.name.clone()));
                    }
                }
                rows.extend(m.help.iter().cloned());
                let w = 560.0 * s;
                let lh = 24.0 * s;
                let h = 80.0 * s + rows.len() as f32 * lh + 70.0 * s;
                let r = self.modal(w, h, c, s);
                c.text(Weight::SemiBold, 20.0 * s, (r.x + 28.0 * s, r.y + 42.0 * s), Align::Left, "Controls", TEXT);
                let mut y = r.y + 76.0 * s;
                for (k, what) in &rows {
                    c.text(Weight::SemiBold, 13.0 * s, (r.x + 28.0 * s, y), Align::Left, k, ACCENT);
                    c.text_fit(Weight::Medium, 13.0 * s, (r.x + 200.0 * s, y), Align::Left, what, w - 228.0 * s, TEXT);
                    y += lh;
                }
                self.wide_button(Rect::new(r.x + w - 24.0 * s - 120.0 * s, r.bottom() - 24.0 * s - 38.0 * s, 120.0 * s, 38.0 * s), "Back", Hit::Back, true, c, s);
            }
        }
    }

    fn end_screen(&mut self, title: &str, lines: &[String], main_menu: bool, c: &mut Canvas, s: f32) {
        let w = 420.0 * s;
        let bh = 42.0 * s;
        let buttons = if main_menu { 3.0 } else { 2.0 };
        let h = 96.0 * s + lines.len() as f32 * 22.0 * s + buttons * (bh + 10.0 * s) + 20.0 * s;
        let r = self.modal(w, h, c, s);
        c.text(Weight::SemiBold, 24.0 * s, (r.x + w / 2.0, r.y + 50.0 * s), Align::Centre, title, TEXT);
        let mut y = r.y + 84.0 * s;
        for l in lines {
            c.text_fit(Weight::Medium, 13.5 * s, (r.x + w / 2.0, y), Align::Centre, l, w - 48.0 * s, MUTED);
            y += 22.0 * s;
        }
        y += 10.0 * s;
        self.wide_button(Rect::new(r.x + 24.0 * s, y, w - 48.0 * s, bh), "Play again", Hit::Menu(MenuItem::Restart), true, c, s);
        let mut y = y + bh + 10.0 * s;
        if main_menu {
            self.wide_button(Rect::new(r.x + 24.0 * s, y, w - 48.0 * s, bh), "Main menu", Hit::Menu(MenuItem::MainMenu), false, c, s);
            y += bh + 10.0 * s;
        }
        self.wide_button(Rect::new(r.x + 24.0 * s, y, w - 48.0 * s, bh), "Quit", Hit::Menu(MenuItem::Quit), false, c, s);
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
        let panel_h = CARD_ROWS as f32 * CELL * s + (CARD_ROWS - 1) as f32 * CELL_GAP * s + 2.0 * CARD_PAD * s;
        (8.0 * s + 38.0 * s, panel_h + 8.0 * s)
    }

    /// Opens the menu on its controls page.
    pub fn show_help(&mut self) {
        self.menu = Some(MenuPage::Help);
        self.help_direct = false;
    }

    /// Seconds since the HUD was made (for UI animations).
    pub fn age(&self) -> f32 {
        self.started.elapsed().as_secs_f32()
    }
}

/// A key's short name for the button's corner.
pub fn key_name(k: KeyCode) -> Option<String> {
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
        Space => "Space",
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
        let card = |name: &str, kind: CardKind, next: bool, idle: bool| GunCard { icon: Some(Icon::Blast), name: name.into(), kind, colour: [255, 150, 60, 255], about: format!("{name} does a thing to the mould where the shot lands, at length."), cost: (kind == CardKind::Payload).then(|| "60".into()), next, idle };
        Loadout {
            name: "Lull".into(),
            whose: "Soldier 1".into(),
            cards: vec![card("Quiet", CardKind::Modifier, false, false), card("Gas", CardKind::Payload, false, false), card("Quiet", CardKind::Modifier, true, false), card("Blast", CardKind::Payload, true, false)],
            slots: 4,
            recharge: 5.0,
            charging: 0.4,
            note: None,
            cycle: Some((NOW + 10, KeyCode::KeyH)),
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

    /// The 4 x 3 slots land on twelve different places of the 6 x 2 card,
    /// in reading order.
    #[test]
    fn the_command_card_is_six_by_two_in_reading_order() {
        let places: Vec<(u8, u8)> = (0..3).flat_map(|r| (0..4).map(move |c| card_slot((r, c)))).collect();
        assert_eq!(places[0], (0, 0));
        assert_eq!(places[5], (0, 5));
        assert_eq!(places[6], (1, 0));
        assert_eq!(places[11], (1, 5));
        let mut sorted = places.clone();
        sorted.sort();
        sorted.dedup();
        assert_eq!(sorted.len(), 12);
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
            let full = r.hud.hits.iter().filter(|(b, h)| *h == Hit::Panel && b.h >= 100.0 * r.hud.scale).count();
            r.m.selection.clear();
            r.draw();
            let empty = r.hud.hits.iter().filter(|(b, h)| *h == Hit::Panel && b.h >= 100.0 * r.hud.scale).count();
            assert_eq!(empty + 1, full, "{screen:?}: the selection panel collapses");
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
        r.m.end = Some(("Over".into(), vec![]));
        r.draw();
        assert_eq!(r.key(KeyCode::KeyA), (true, vec![]));
        assert_eq!(r.key(KeyCode::Escape), (true, vec![]));
        assert!(!r.hud.menu_open() && r.hud.targeting().is_none());
        // A click off its buttons does nothing either.
        r.at(WORLD_SPOT);
        assert_eq!(r.click(Button::Left, true), (true, vec![]));
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

    #[test]
    fn the_main_menu_button_shows_only_when_the_game_has_one() {
        for on in [false, true] {
            let mut r = Rig::new((1920, 1080));
            r.m.main_menu = on;
            r.key(KeyCode::Escape);
            r.draw();
            assert_eq!(r.hud.hits.iter().any(|(_, h)| *h == Hit::Menu(MenuItem::MainMenu)), on);
            r.m.end = Some(("Over".into(), vec![]));
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
