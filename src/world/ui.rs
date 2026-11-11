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
use super::canvas::{Align, Canvas, Rect, Rgba, Weight};
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
    pub target: Target,
    /// The tooltip's body.
    pub about: String,
    /// Shown in the tooltip, in red, when it is not ready.
    pub why_not: Option<String>,
}

impl Command {
    pub fn new(id: u32, slot: (u8, u8), icon: Icon, name: &str, key: KeyCode) -> Self {
        Command { id, slot, icon, name: name.into(), key, cost: None, cost_icon: Icon::Crystal, ready: true, on: false, cooldown: 0.0, target: Target::Instant, about: String::new(), why_not: None }
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
    /// Whether the targeting ghost is somewhere the command can go.
    pub ghost_ok: bool,
    /// The round is over: a title and lines.
    pub end: Option<(String, Vec<String>)>,
    /// Extra lines for the controls page (key, what it does).
    pub help: Vec<(String, String)>,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum MenuItem {
    Opened,
    Resume,
    Restart,
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
    /// The end-of-round screen is up (its buttons take every click).
    end: bool,
    started: Instant,
}

impl Default for Hud {
    fn default() -> Self {
        Self::new()
    }
}

impl Hud {
    pub fn new() -> Self {
        Hud { scale: 1.0, hits: Vec::new(), hover: None, pressed: None, targeting: None, menu: None, alerts: Vec::new(), commands: Vec::new(), bottom: 0.0, minimap_box: None, minimap: None, top: 0.0, end: false, started: Instant::now() }
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
    pub fn over(&self, p: (f32, f32)) -> bool {
        self.menu.is_some() || self.end || self.hits.iter().any(|(r, _)| r.contains(p))
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
                    Some(Hit::Help) => self.menu = Some(MenuPage::Help),
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
                match (b, pressed) {
                    (Button::Right, true) => {
                        self.targeting = None;
                        return (true, out);
                    }
                    (Button::Left, true) => {
                        if let Some(w) = cam.world_at(p) {
                            match t.target {
                                Target::Line { .. } => self.targeting = Some(Targeting { from: Some(w), ..t }),
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
                    (Button::Left, false) => {
                        if let (Target::Line { .. }, Some(from)) = (t.target, t.from) {
                            let to = cam.to_world(p);
                            let to = (to.0.clamp(0.0, cam.world.0 - 1.0), to.1.clamp(0.0, cam.world.1 - 1.0));
                            out.push(HudEvent::Line { id: t.id, from, to });
                            self.targeting = if shift { Some(Targeting { from: None, ..t }) } else { None };
                        }
                        return (true, out);
                    }
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
        if code == KeyCode::Escape {
            if self.targeting.take().is_some() {
                return (true, out);
            }
            match self.menu {
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
        if code == KeyCode::Space {
            if let Some(at) = self.alerts.iter().rev().find_map(|a| a.at) {
                cam.centre_on(at);
                return (true, out);
            }
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
        let s = (sh / 1080.0).clamp(0.7, 1.6);
        self.scale = s;
        self.hits.clear();
        self.commands = m.commands.clone();
        if let Some(t) = self.targeting {
            // The command may have gone (selection changed); keep it only
            // while it is still on the card and ready.
            let still = m.commands.iter().position(|k| k.id == t.id && k.ready);
            self.targeting = still.map(|index| Targeting { index, ..t });
        }
        let now = Instant::now();
        self.alerts.retain(|a| now.duration_since(a.when) < ALERT_LIFE);
        let pad = 10.0 * s;

        // The minimap's frame (bottom right), the command card left of it,
        // the selection panel left of that, all one height.
        let cell = 58.0 * s;
        let gap = 6.0 * s;
        let card_w = 4.0 * cell + 3.0 * gap + 2.0 * pad;
        let panel_h = 3.0 * cell + 2.0 * gap + 2.0 * pad;
        let base = sh - 8.0 * s;
        let top_of = base - panel_h;
        self.bottom = top_of;
        let radius = 12.0 * s;
        let mut right = sw - 8.0 * s;
        self.minimap_box = None;
        self.minimap = None;
        if minimap {
            let ih = panel_h - 2.0 * pad;
            let iw = (ih * cam.world.0 / cam.world.1).clamp(ih * 0.6, (360.0 * s).min(sw * 0.25));
            let f = Rect::new(right - iw - 2.0 * pad, top_of, iw + 2.0 * pad, panel_h);
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
        let sel_w = (600.0 * s).min(card.x - 16.0 * s);
        let sel = Rect::new(card.x - 8.0 * s - sel_w, top_of, sel_w, panel_h);

        self.top_bar(m, c, s);
        self.selection_panel(m, controls, sel, c, s);
        self.command_card(m, card, c, s);
        self.alerts_draw(c, world, s);
        self.ghost(m, cam, c, world, s);
        self.end = m.end.is_some();
        if let Some((title, lines)) = &m.end {
            self.end_screen(title, lines, c, s);
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
        let pad = 14.0 * s;
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
            let alive = members.iter().filter(|k| m.selection.iter().any(|q| q.id == **k) || controls.selected.contains(k)).count().max(members.len());
            if members.is_empty() {
                continue;
            }
            let label = format!("{}", g + 1);
            let count = format!("{alive}");
            let w = 18.0 * s + c.atlas.measure(Weight::SemiBold, 12.0 * s, &label) + c.atlas.measure(Weight::Medium, 11.0 * s, &count) + 6.0 * s;
            let chip = Rect::new(x, y, w, chip_h);
            let on = !controls.selected.is_empty() && controls.selected == *members;
            self.chip(chip, Hit::Group(g), on, c, s);
            let lw = c.text(Weight::SemiBold, 12.0 * s, (chip.x + 9.0 * s, chip.y + 16.5 * s), Align::Left, &label, if on { ACCENT } else { TEXT });
            c.text(Weight::Medium, 11.0 * s, (chip.x + 15.0 * s + lw, chip.y + 16.5 * s), Align::Left, &count, MUTED);
            x += w + 6.0 * s;
        }

        let inner = r.inset(pad);
        if let Some(info) = &m.info {
            self.info_draw(info, inner, c, s);
            return;
        }
        if m.selection.is_empty() {
            let base = inner.y + inner.h / 2.0;
            c.icon(Icon::Soldier, (inner.x, base - 22.0 * s), 28.0 * s, FAINT);
            c.text(Weight::SemiBold, 15.0 * s, (inner.x + 40.0 * s, base - 4.0 * s), Align::Left, "Nothing selected", MUTED);
            c.text(Weight::Medium, 12.5 * s, (inner.x + 40.0 * s, base + 14.0 * s), Align::Left, "Click a soldier, or drag a box round several", FAINT);
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
        let cw = 46.0 * s;
        let cgap = 6.0 * s;
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
            c.icon(q.icon, (cell.x + (cw - 24.0 * s) / 2.0, cell.y + 6.0 * s), 24.0 * s, col);
            c.bar(Rect::new(cell.x + 6.0 * s, cell.bottom() - 10.0 * s, cw - 12.0 * s, 4.0 * s), q.hp, health_colour(q.hp), TRACK);
            if let Some(b) = q.badge {
                c.icon(b, (cell.right() - 15.0 * s, cell.y + 3.0 * s), 12.0 * s, MUTED);
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
        let pad = 10.0 * s;
        let cell = 58.0 * s;
        let gap = 6.0 * s;
        let x0 = r.x + pad;
        let y0 = r.bottom() - pad - 3.0 * cell - 2.0 * gap;
        // Empty slots, faintly, so the grid reads as a grid.
        for j in 0..3 {
            for i in 0..4 {
                let b = Rect::new(x0 + i as f32 * (cell + gap), y0 + j as f32 * (cell + gap), cell, cell);
                c.rrect(b, 10.0 * s, [255, 255, 255, 8]);
            }
        }
        let mut tip = None;
        for (k, cmd) in m.commands.iter().enumerate() {
            let (row, col) = (cmd.slot.0.min(2) as f32, cmd.slot.1.min(3) as f32);
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
            let ic = 28.0 * s;
            c.icon(cmd.icon, (b.x + (cell - ic) / 2.0, b.y + (cell - ic) / 2.0 - 2.0 * s), ic, alpha(if cmd.on { ACCENT } else { TEXT }, dim));
            // The hotkey, top left.
            if let Some(k) = key_name(cmd.key) {
                c.text(Weight::SemiBold, 10.5 * s, (b.x + 6.0 * s, b.y + 13.0 * s), Align::Left, &k, alpha(MUTED, dim.max(0.6)));
            }
            if let Some(cost) = &cmd.cost {
                let cw = c.atlas.measure(Weight::SemiBold, 10.5 * s, cost);
                let cx = b.right() - 5.0 * s - cw;
                c.text(Weight::SemiBold, 10.5 * s, (cx, b.bottom() - 5.0 * s), Align::Left, cost, alpha(if cmd.ready { ENERGY } else { BAD }, 0.95));
                c.icon(cmd.cost_icon, (cx - 12.0 * s, b.bottom() - 15.0 * s), 10.0 * s, alpha(ENERGY, dim.max(0.6)));
            }
            if cmd.cooldown > 0.0 {
                c.rrect(b, 10.0 * s, [0, 0, 0, 110]);
                c.ring(b.inset(3.0 * s), 8.0 * s, 2.5 * s, cmd.cooldown, alpha(ACCENT, 0.9));
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

    fn tooltip(&mut self, cmd: &Command, b: Rect, card: Rect, c: &mut Canvas, s: f32) {
        let w = 300.0 * s;
        let px = 12.5 * s;
        let lines = c.wrap(Weight::Medium, px, w - 28.0 * s, &cmd.about);
        let why = if cmd.ready { None } else { cmd.why_not.clone() };
        let h = 18.0 * s + 22.0 * s + if cmd.about.is_empty() { 0.0 } else { lines.len() as f32 * 18.0 * s + 8.0 * s } + if why.is_some() { 20.0 * s } else { 0.0 } + 10.0 * s;
        let x = (b.x + b.w / 2.0 - w / 2.0).clamp(8.0 * s, card.right() - w);
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
        if self.hit(p).is_some() {
            return;
        }
        let at = cam.to_world(p);
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
        let text = format!("{name}: {how}, right-click to cancel");
        self.tip_small(&text, (p.0 + 18.0 * s, p.1 + 18.0 * s), c, s);
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
                let items: [(&str, Hit, bool); 4] = [("Resume", Hit::Menu(MenuItem::Resume), true), ("Controls", Hit::Help, false), ("Restart", Hit::Menu(MenuItem::Restart), false), ("Quit", Hit::Menu(MenuItem::Quit), false)];
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
                let mut rows: Vec<(String, String)> = vec![
                    ("Left click / drag".into(), "Select soldiers".into()),
                    ("Right click".into(), "Move, or work the crystal or mould clicked".into()),
                    ("Shift".into(), "Add to the selection, or keep placing".into()),
                    ("Ctrl + 1-9".into(), "Make a group; 1-9 selects it, twice to look".into()),
                    ("Wheel, middle drag".into(), "Zoom, pan; the screen's edge scrolls".into()),
                    ("Minimap".into(), "Click to look there; right click to send".into()),
                    ("Space".into(), "Look at the newest alert".into()),
                    ("Esc".into(), "Cancel placing, or this menu".into()),
                ];
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

    fn end_screen(&mut self, title: &str, lines: &[String], c: &mut Canvas, s: f32) {
        let w = 420.0 * s;
        let bh = 42.0 * s;
        let h = 96.0 * s + lines.len() as f32 * 22.0 * s + 2.0 * (bh + 10.0 * s) + 20.0 * s;
        let r = self.modal(w, h, c, s);
        c.text(Weight::SemiBold, 24.0 * s, (r.x + w / 2.0, r.y + 50.0 * s), Align::Centre, title, TEXT);
        let mut y = r.y + 84.0 * s;
        for l in lines {
            c.text_fit(Weight::Medium, 13.5 * s, (r.x + w / 2.0, y), Align::Centre, l, w - 48.0 * s, MUTED);
            y += 22.0 * s;
        }
        y += 10.0 * s;
        self.wide_button(Rect::new(r.x + 24.0 * s, y, w - 48.0 * s, bh), "Play again", Hit::Menu(MenuItem::Restart), true, c, s);
        self.wide_button(Rect::new(r.x + 24.0 * s, y + bh + 10.0 * s, w - 48.0 * s, bh), "Quit", Hit::Menu(MenuItem::Quit), false, c, s);
    }

    /// Where a command's button was last drawn.
    pub fn command_rect(&self, id: u32) -> Option<Rect> {
        let k = self.commands.iter().position(|c| c.id == id)?;
        self.hits.iter().find(|(_, h)| *h == Hit::Command(k)).map(|(r, _)| *r)
    }

    /// Opens the menu on its controls page.
    pub fn show_help(&mut self) {
        self.menu = Some(MenuPage::Help);
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
