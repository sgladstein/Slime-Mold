//! **The start menu** (cloud plan 2026-11-14, item C2): what a player sees
//! before a round. Its pages: the main page (sandbox, campaign, options,
//! keys, quit), the campaign's mission list, an options page for the
//! window's switches with each one's default beside it, and the key card
//! (also F1 or ? from any page).
//!
//! Like the HUD (`ui`), it holds no game rules: the game hands it the
//! missions, the options (name, choices, default) and the key card, draws
//! it on the UI canvas each frame, and gets back a `StartEvent` when the
//! player picks a round. It is drawn as panels down the left of the screen
//! rather than a centred box, so the live dish behind it (the window plays
//! a round under the menu) stays in view: what is on screen is the
//! simulation (owner, 2026-11-09/10).
//!
//! Keyboard: Up/Down pick a row, Enter takes it, Left/Right change an
//! option, Backspace sets it back to its default, Esc goes back a page,
//! 1-9 start that mission on the campaign page.

use winit::keyboard::KeyCode;

use super::canvas::{Align, Canvas, Rect, Rgba, Weight};
use super::controls::Button;
use super::icons::Icon;
use super::ui::theme::*;

/// One switch on the options page: its value is `choices[at]`.
#[derive(Clone, Debug, PartialEq)]
pub struct Opt {
    /// The game's own name for it (the command-line key).
    pub key: String,
    pub label: String,
    pub choices: Vec<String>,
    pub default: usize,
    pub at: usize,
    /// One line on what it does.
    pub note: String,
}

impl Opt {
    /// An option whose default is `default`, added to `choices` if missing,
    /// starting at `current` (also added if missing: a command-line value
    /// off the list still shows).
    pub fn new(key: &str, label: &str, choices: &[&str], default: &str, current: &str, note: &str) -> Self {
        let mut c: Vec<String> = choices.iter().map(|s| s.to_string()).collect();
        for v in [default, current] {
            if !c.iter().any(|x| x == v) {
                c.push(v.to_string());
            }
        }
        let find = |v: &str| c.iter().position(|x| x == v).unwrap_or(0);
        let (default, at) = (find(default), find(current));
        Opt { key: key.into(), label: label.into(), choices: c, default, at, note: note.into() }
    }

    pub fn value(&self) -> &str {
        &self.choices[self.at]
    }

    pub fn is_default(&self) -> bool {
        self.at == self.default
    }

    fn turn(&mut self, by: isize) {
        let n = self.choices.len() as isize;
        self.at = (self.at as isize + by).rem_euclid(n) as usize;
    }
}

/// A mission on the campaign page.
#[derive(Clone, Debug, PartialEq)]
pub struct Entry {
    /// What starts it (`mission=STEM`).
    pub stem: String,
    pub name: String,
    /// The goal sentence.
    pub says: String,
    /// A muted line under it (the map and the mould).
    pub detail: String,
}

/// Where a shop row sits on the page (owner, 2026-11-14 16:55Z: "This
/// whole menu should not just be uniform buttons. There should be one area
/// for unlocking/upgrading that should match the player menu from the
/// screen (buildings and tools should be separate menus that you open up).
/// The second area of the screen is for Guns/bullet cards." The first
/// shop, #280, was one list of uniform rows in columns.)
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ShopArea {
    /// A tile on the upgrade grid's first page, as on the command card.
    Upgrades,
    /// A tile in the folder named by the row's `group` (Buildings, Tools),
    /// opened from the grid as the command card's Q and E pages are.
    Folder,
    /// A gun card in the armory.
    Guns,
    /// A bullet card: those `on` fill the carried gun's slots, the rest sit
    /// under their `group` (the bag, for sale).
    Cards,
}

/// A row of the run's shop page (the game fills it: `mould::run::Run::shop`).
#[derive(Clone, Debug, PartialEq)]
pub struct ShopRow {
    /// The folder or heading it sits under.
    pub group: String,
    pub area: ShopArea,
    pub icon: Icon,
    /// The icon's colour (a card's kind, a lucky gun's gold).
    pub tint: Rgba,
    pub name: String,
    pub about: String,
    /// Its price, or empty.
    pub price: String,
    /// "2/4", "owned", "in the gun"...
    pub state: String,
    /// A pick does something now (bright), or not (dim).
    pub ready: bool,
    /// The carried gun, a card in it.
    pub on: bool,
    /// Levels owned of most, as pips (0 of 0: none drawn).
    pub level: (u8, u8),
    /// A gun's numbers as bars: label, 0-1, the value.
    pub bars: Vec<(String, f32, String)>,
    /// A card's budget cost, as pips.
    pub cost: u32,
}

impl ShopRow {
    /// A row with the plain look; the game sets the rest.
    pub fn new(group: &str, area: ShopArea, icon: Icon, name: &str) -> Self {
        ShopRow { group: group.into(), area, icon, tint: TEXT, name: name.into(), about: String::new(), price: String::new(), state: String::new(), ready: false, on: false, level: (0, 0), bars: Vec::new(), cost: 0 }
    }
}

/// The run's shop, between two levels.
#[derive(Clone, Debug, Default, PartialEq)]
pub struct Shop {
    /// "Planet 3 next: the Swarmer on the Rim".
    pub heading: String,
    /// Lines under it (the run so far, what the gun holds).
    pub lines: Vec<String>,
    /// The points to spend, as shown.
    pub purse: String,
    pub rows: Vec<ShopRow>,
    /// The carried gun's slots and its budget (used, all).
    pub slots: u32,
    pub budget: (u32, u32),
    /// A folder's icon by its name (the command card's page button);
    /// a folder not named shows its first row's.
    pub folder_icons: Vec<(String, Icon)>,
}

/// Where everything on the shop page goes at one scale.
struct ShopPlan {
    s: f32,
    left: Rect,
    right: Rect,
    tiles: Vec<(Rect, Hit)>,
    guns: Vec<(Rect, usize)>,
    /// The carried gun's slots: the card in each, if any.
    slots: Vec<(Rect, Option<usize>)>,
    cards: Vec<(Rect, usize)>,
    heads: Vec<((f32, f32), String)>,
    detail: Rect,
    fits: bool,
}

/// The size `text` fits `max` pixels at: `px`, smaller down to `least`
/// (a tile's name shrinks before it is cut).
fn fit_px(c: &Canvas, text: &str, px: f32, least: f32, max: f32) -> f32 {
    let w = c.atlas.measure(Weight::SemiBold, px, text);
    if w <= max {
        px
    } else {
        (px * max / w).max(least)
    }
}

fn with_alpha(c: Rgba, a: f32) -> Rgba {
    [c[0], c[1], c[2], (c[3] as f32 * a.clamp(0.0, 1.0)) as u8]
}

/// What the player picked.
#[derive(Clone, Debug, PartialEq)]
pub enum StartEvent {
    Sandbox,
    /// The index into `missions`.
    Mission(usize),
    /// Open the map editor (`larder-edit`, #209).
    EditMaps,
    /// Start a roguelike run, or go back to the one under way.
    Run,
    /// Shop row `i` picked.
    Buy(usize),
    /// Land on the next planet.
    NextLevel,
    /// Give the run up.
    EndRun,
    /// A round on a new generated planet (`mould::levelgen`).
    RandomPlanet,
    Quit,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Page {
    Main,
    Missions,
    Options,
    Keys,
    /// The run's shop.
    Shop,
}

#[derive(Clone, Copy, Debug, PartialEq)]
enum Hit {
    Panel,
    Sandbox,
    Run,
    Buy(usize),
    /// Open the shop's folder `k` (`StartMenu::folders`), or close it.
    Folder(usize),
    FolderBack,
    Next,
    EndRun,
    Campaign,
    Options,
    Keys,
    EditMaps,
    RandomPlanet,
    Quit,
    Back,
    Mission(usize),
    /// Option `i` one choice back or on.
    Turn(usize, isize),
    Reset(usize),
    ResetAll,
}

pub struct StartMenu {
    pub title: String,
    pub missions: Vec<Entry>,
    pub options: Vec<Opt>,
    /// The key card's rows: key, what it does.
    pub keys: Vec<(String, String)>,
    pub page: Page,
    /// The main page offers "Edit maps" (the game has an editor to start).
    pub edit_maps: bool,
    /// The main page offers "Random planet" (the game makes planets).
    pub planets: bool,
    /// A line the game wants shown on the main page (such as why the map
    /// editor did not start).
    pub notice: Option<String>,
    /// The run's shop (the Shop page draws it).
    pub shop: Option<Shop>,
    /// A run is under way: the main page's Run row goes back to it.
    pub run_live: bool,
    /// The shop's open folder, by name.
    folder: Option<String>,
    /// Which row the keyboard is on, on this page.
    pub focus: usize,
    /// The page the key card goes back to.
    keys_from: Page,
    hover: Option<(f32, f32)>,
    pressed: Option<Hit>,
    hits: Vec<(Rect, Hit)>,
}

impl StartMenu {
    pub fn new(title: &str, missions: Vec<Entry>, options: Vec<Opt>, keys: Vec<(String, String)>) -> Self {
        StartMenu { title: title.into(), missions, options, keys, page: Page::Main, edit_maps: false, planets: false, notice: None, shop: None, run_live: false, folder: None, focus: 0, keys_from: Page::Main, hover: None, pressed: None, hits: Vec::new() }
    }

    /// The option named `key`.
    pub fn option(&self, key: &str) -> Option<&Opt> {
        self.options.iter().find(|o| o.key == key)
    }

    /// `key`'s value, if the player moved it off its default.
    pub fn changed(&self, key: &str) -> Option<&str> {
        self.option(key).filter(|o| !o.is_default()).map(|o| o.value())
    }

    pub fn show(&mut self, page: Page) {
        if page == Page::Keys && self.page != Page::Keys {
            self.keys_from = self.page;
        }
        self.page = page;
        self.focus = 0;
        self.folder = None;
    }

    /// Opens the shop's folder `name` (the pictures; a click does it in play).
    pub fn open_folder(&mut self, name: &str) {
        if self.folders().iter().any(|f| f == name) {
            self.folder = Some(name.to_string());
            self.focus = 0;
        }
    }

    /// The shop's folders, in the order their rows come.
    fn folders(&self) -> Vec<String> {
        let mut f: Vec<String> = Vec::new();
        for r in self.shop.iter().flat_map(|s| &s.rows).filter(|r| r.area == ShopArea::Folder) {
            if !f.contains(&r.group) {
                f.push(r.group.clone());
            }
        }
        f
    }

    /// The upgrade grid's tiles on the page showing (the first, or a folder).
    fn grid(&self) -> Vec<Hit> {
        let Some(shop) = &self.shop else { return Vec::new() };
        let of = |area: ShopArea, group: Option<&String>| -> Vec<Hit> { shop.rows.iter().enumerate().filter(|(_, r)| r.area == area && group.is_none_or(|g| r.group == *g)).map(|(i, _)| Hit::Buy(i)).collect() };
        match &self.folder {
            Some(f) => std::iter::once(Hit::FolderBack).chain(of(ShopArea::Folder, Some(f))).collect(),
            None => of(ShopArea::Upgrades, None).into_iter().chain((0..self.folders().len()).map(Hit::Folder)).collect(),
        }
    }

    /// The bullet cards out of the gun, by heading, in the order they come.
    fn card_groups(&self) -> Vec<(String, Vec<usize>)> {
        let mut g: Vec<(String, Vec<usize>)> = Vec::new();
        for (i, r) in self.shop.iter().flat_map(|s| s.rows.iter().enumerate()).filter(|(_, r)| r.area == ShopArea::Cards && !r.on) {
            match g.iter_mut().find(|(h, _)| *h == r.group) {
                Some((_, v)) => v.push(i),
                None => g.push((r.group.clone(), vec![i])),
            }
        }
        g
    }

    /// The shop's tiles in reading order: the grid, the guns, the gun's
    /// cards, the rest of the cards.
    fn shop_rows(&self) -> Vec<Hit> {
        let Some(shop) = &self.shop else { return vec![Hit::Next, Hit::EndRun] };
        let area = |a: ShopArea, on: bool| shop.rows.iter().enumerate().filter(move |(_, r)| r.area == a && (a != ShopArea::Cards || r.on == on)).map(|(i, _)| Hit::Buy(i));
        let mut v = self.grid();
        v.extend(area(ShopArea::Guns, false));
        v.extend(area(ShopArea::Cards, true));
        v.extend(self.card_groups().into_iter().flat_map(|(_, c)| c).map(Hit::Buy));
        v.extend([Hit::Next, Hit::EndRun]);
        v
    }

    /// The rows the keyboard walks on this page, top to bottom.
    fn rows(&self) -> Vec<Hit> {
        match self.page {
            Page::Main => [Hit::Sandbox, Hit::Run, Hit::Campaign].into_iter().chain(self.planets.then_some(Hit::RandomPlanet)).chain([Hit::Options, Hit::Keys]).chain(self.edit_maps.then_some(Hit::EditMaps)).chain([Hit::Quit]).collect(),
            Page::Missions => (0..self.missions.len()).map(Hit::Mission).chain([Hit::Back]).collect(),
            Page::Options => (0..self.options.len()).map(Hit::Reset).chain([Hit::ResetAll, Hit::Back]).collect(),
            Page::Keys => vec![Hit::Back],
            Page::Shop => self.shop_rows(),
        }
    }

    fn back(&mut self) {
        if self.page == Page::Shop {
            if let Some(f) = self.folder.take() {
                // Back on the grid, on the folder's tile.
                let k = self.folders().iter().position(|x| *x == f).unwrap_or(0);
                self.focus = self.rows().iter().position(|&h| h == Hit::Folder(k)).unwrap_or(0);
                return;
            }
        }
        let to = if self.page == Page::Keys { self.keys_from } else { Page::Main };
        let from = self.page;
        self.show(to);
        // Back on the main page, on the row that led away.
        if to == Page::Main {
            let led = match from {
                Page::Missions => Hit::Campaign,
                Page::Options => Hit::Options,
                Page::Keys => Hit::Keys,
                Page::Shop => Hit::Run,
                Page::Main => Hit::Sandbox,
            };
            self.focus = self.rows().iter().position(|&h| h == led).unwrap_or(0);
        }
    }

    /// What a hit does; a round picked comes back.
    fn act(&mut self, h: Hit) -> Option<StartEvent> {
        match h {
            Hit::Sandbox => return Some(StartEvent::Sandbox),
            Hit::Run => return Some(StartEvent::Run),
            Hit::Buy(i) => return Some(StartEvent::Buy(i)),
            Hit::Folder(k) => {
                self.folder = self.folders().get(k).cloned();
                self.focus = 0;
            }
            Hit::FolderBack => self.back(),
            Hit::Next => return Some(StartEvent::NextLevel),
            Hit::EndRun => return Some(StartEvent::EndRun),
            Hit::Campaign => self.show(Page::Missions),
            Hit::Options => self.show(Page::Options),
            Hit::Keys => self.show(Page::Keys),
            Hit::Quit => return Some(StartEvent::Quit),
            Hit::EditMaps => return Some(StartEvent::EditMaps),
            Hit::RandomPlanet => return Some(StartEvent::RandomPlanet),
            Hit::Back => self.back(),
            Hit::Mission(i) => return Some(StartEvent::Mission(i)),
            Hit::Turn(i, by) => self.options[i].turn(by),
            // On the keyboard, Enter on an option's row turns it on.
            Hit::Reset(i) => self.options[i].turn(1),
            Hit::ResetAll => {
                for o in &mut self.options {
                    o.at = o.default;
                }
            }
            Hit::Panel => {}
        }
        None
    }

    pub fn moved(&mut self, p: Option<(f32, f32)>) {
        self.hover = p;
    }

    fn hit(&self, p: (f32, f32)) -> Option<Hit> {
        // Last drawn is on top.
        self.hits.iter().rev().find(|(r, _)| r.contains(p)).map(|&(_, h)| h)
    }

    /// A mouse button: a click is a press and a release on the same thing.
    pub fn button(&mut self, b: Button, pressed: bool) -> Vec<StartEvent> {
        if b != Button::Left {
            if b == Button::Right && pressed && self.page != Page::Main {
                self.back();
            }
            return Vec::new();
        }
        let hit = self.hover.and_then(|p| self.hit(p));
        if pressed {
            self.pressed = hit;
            return Vec::new();
        }
        let was = self.pressed.take();
        match hit {
            Some(h) if was == Some(h) => {
                // A row clicked: an option's row resets it (its arrows turn it).
                let h = match h {
                    Hit::Reset(i) => {
                        self.options[i].at = self.options[i].default;
                        return Vec::new();
                    }
                    h => h,
                };
                self.act(h).into_iter().collect()
            }
            _ => Vec::new(),
        }
    }

    /// A key press.
    pub fn key(&mut self, code: KeyCode, pressed: bool) -> Vec<StartEvent> {
        if !pressed {
            return Vec::new();
        }
        let rows = self.rows();
        self.focus = self.focus.min(rows.len().saturating_sub(1));
        let row = rows.get(self.focus).copied();
        match code {
            KeyCode::F1 | KeyCode::Slash => {
                if self.page == Page::Keys {
                    self.back();
                } else {
                    self.show(Page::Keys);
                }
            }
            KeyCode::Escape | KeyCode::Backspace if code == KeyCode::Escape || self.page != Page::Options => {
                if self.page != Page::Main {
                    self.back();
                }
            }
            KeyCode::Backspace => {
                if let Some(Hit::Reset(i)) = row {
                    self.options[i].at = self.options[i].default;
                }
            }
            KeyCode::ArrowUp | KeyCode::KeyW => self.focus = (self.focus + rows.len() - 1) % rows.len(),
            KeyCode::ArrowDown | KeyCode::KeyS => self.focus = (self.focus + 1) % rows.len(),
            KeyCode::ArrowLeft | KeyCode::KeyA if self.page == Page::Shop => self.focus = (self.focus + rows.len() - 1) % rows.len(),
            KeyCode::ArrowRight | KeyCode::KeyD if self.page == Page::Shop => self.focus = (self.focus + 1) % rows.len(),
            KeyCode::ArrowLeft | KeyCode::ArrowRight | KeyCode::KeyA | KeyCode::KeyD => {
                if let Some(Hit::Reset(i)) = row {
                    self.options[i].turn(if matches!(code, KeyCode::ArrowLeft | KeyCode::KeyA) { -1 } else { 1 });
                }
            }
            KeyCode::Enter | KeyCode::NumpadEnter | KeyCode::Space => {
                if let Some(h) = row {
                    return self.act(h).into_iter().collect();
                }
            }
            _ => {
                if self.page == Page::Missions {
                    if let Some(n) = digit(code).filter(|&n| n >= 1 && n <= self.missions.len()) {
                        return vec![StartEvent::Mission(n - 1)];
                    }
                }
            }
        }
        Vec::new()
    }

    /// Draws the page on `c` (cleared by the caller).
    pub fn draw(&mut self, c: &mut Canvas) {
        let (sw, sh) = (c.screen.0 as f32, c.screen.1 as f32);
        // The UI size (`UiSize::px`), held where the options page, the
        // tallest, would run off the bottom (at 150% on a 900-line window
        // its last rows, the UI size among them, did); at 100% it never is.
        // Each page is held by its own height: until 11-14 20:23Z every page
        // was held by the options page's, so with a dozen options the main
        // menu stopped growing past about 100% on a 1080-line screen and
        // Ctrl + seemed to do nothing there (owner: "changing the UI zoom,
        // doesn't change the main menu"). The main page and the campaign's
        // list count their own rows (62 units and a 12 gap a row).
        let held = match self.page {
            Page::Main | Page::Missions => 330.0 + 74.0 * self.rows().len() as f32,
            _ => 330.0 + 62.0 * self.options.len() as f32,
        };
        let s = c.ui.px(c.screen).min(sh / held);
        let s = (s * 20.0 + 1e-3).floor() / 20.0;
        self.hits.clear();
        // A shade down the left, fading out, so the panel reads over the
        // dish; the shop spans the screen, so it dims all of it.
        if self.page == Page::Shop {
            c.rrect(Rect::new(0.0, 0.0, sw, sh), 0.0, [4, 6, 8, 150]);
        }
        let shade_w = (720.0 * s).min(sw);
        // (Forty thin steps: twelve showed as bands on the 1600-wide shot.)
        for k in 0..40 {
            let f = k as f32 / 40.0;
            c.rrect(Rect::new(0.0, 0.0, shade_w * (1.0 - f), sh), 0.0, [4, 6, 8, (9.0 * (1.0 - f)) as u8 + 2]);
        }
        let x = 48.0 * s;
        let w = (560.0 * s).min(sw - 2.0 * x);
        c.text(Weight::SemiBold, 44.0 * s, (x, 96.0 * s), Align::Left, &self.title, TEXT);
        let sub = match self.page {
            Page::Main => "A mould that grows, feeds and fights back",
            Page::Missions => "Campaign",
            Page::Options => "Options",
            Page::Keys => "Keys",
            Page::Shop => "The run: spend your points, then land",
        };
        c.text(Weight::Medium, 16.0 * s, (x, 126.0 * s), Align::Left, sub, MUTED);
        let top = 160.0 * s;
        let rows = self.rows();
        let focus = rows.get(self.focus.min(rows.len().saturating_sub(1))).copied();
        match self.page {
            Page::Main => self.main_page(c, x, top, w, s, focus),
            Page::Missions => self.missions_page(c, x, top, w, s, focus),
            Page::Options => self.options_page(c, x, top, w, s, focus),
            Page::Keys => self.keys_page(c, x, top, w, s),
            Page::Shop => self.shop_page(c, x, top, s, focus),
        }
        let hint = match self.page {
            Page::Main => "Up/Down and Enter, or click  ·  F1 or ? keys",
            Page::Missions => "1-9 or Enter starts a mission  ·  Esc back",
            Page::Options => "Left/Right change  ·  Backspace or click the row: default  ·  Esc back",
            Page::Keys => "Esc or F1 back",
            Page::Shop => "Click or Enter buys  ·  bright tiles you can afford  ·  Esc closes a folder, then the shop (the run waits)",
        };
        c.text_fit(Weight::Medium, 12.5 * s, (x, sh - 28.0 * s), Align::Left, hint, sw - 2.0 * x, FAINT);
    }

    fn glass(&mut self, r: Rect, c: &mut Canvas, s: f32) {
        c.shadow(r, 14.0 * s, 24.0 * s, 6.0 * s, SHADOW);
        c.panel(r, 14.0 * s, PANEL_TOP, PANEL_BOTTOM, 1.0, EDGE);
        self.hits.push((r, Hit::Panel));
    }

    /// A button row: `label` big, `sub` muted under it.
    #[allow(clippy::too_many_arguments)]
    fn item(&mut self, r: Rect, label: &str, sub: &str, hit: Hit, primary: bool, focused: bool, c: &mut Canvas, s: f32) {
        let hot = focused || self.hover.is_some_and(|p| r.contains(p));
        let (t, b) = match (primary, hot) {
            (true, false) => ([70, 156, 222, 255], [50, 124, 186, 255]),
            (true, true) => ([92, 176, 240, 255], [62, 140, 204, 255]),
            (false, false) => (BUTTON_TOP, BUTTON_BOTTOM),
            (false, true) => (BUTTON_HOVER_TOP, BUTTON_HOVER_BOTTOM),
        };
        c.panel(r, 10.0 * s, t, b, 1.0, [255, 255, 255, 34]);
        if focused {
            c.outline(r.inset(-2.0 * s), 12.0 * s, 1.5 * s, ACCENT);
        }
        let pad = 18.0 * s;
        if sub.is_empty() {
            c.text(Weight::SemiBold, 17.0 * s, (r.x + pad, r.y + r.h / 2.0 + 6.0 * s), Align::Left, label, TEXT);
        } else {
            c.text(Weight::SemiBold, 17.0 * s, (r.x + pad, r.y + r.h / 2.0 - 2.0 * s), Align::Left, label, TEXT);
            let col = if primary { [220, 236, 250, 255] } else { MUTED };
            c.text_fit(Weight::Medium, 12.5 * s, (r.x + pad, r.y + r.h / 2.0 + 16.0 * s), Align::Left, sub, r.w - 2.0 * pad, col);
        }
        self.hits.push((r, hit));
    }

    fn main_page(&mut self, c: &mut Canvas, x: f32, top: f32, w: f32, s: f32, focus: Option<Hit>) {
        let changed: Vec<String> = self.options.iter().filter(|o| !o.is_default()).map(|o| format!("{} {}", o.key, o.value())).collect();
        let sandbox = if changed.is_empty() { "Free play on the map, with today's defaults".to_string() } else { format!("Free play, with {}", changed.join(", ")) };
        let campaign = format!("{} missions, one new verb each", self.missions.len());
        let mut items: Vec<(&str, String, Hit, bool)> = vec![
            ("Sandbox", sandbox, Hit::Sandbox, true),
            (if self.run_live { "Back to the run" } else { "Run" }, if self.run_live { "The shop, then the next planet".into() } else { "Planet after planet toward the slime's home world; upgrades between".into() }, Hit::Run, false),
            ("Campaign", campaign, Hit::Campaign, false),
        ];
        if self.planets {
            let depth = self.option("planet").map_or("1", |o| o.value());
            items.push(("Random planet", format!("A new generated planet, planet {depth} of a run (Options: Planet)"), Hit::RandomPlanet, false));
        }
        items.extend([
            ("Options", "The mould's and the squad's switches".into(), Hit::Options, false),
            ("Keys", "F1 or ? in the game too".into(), Hit::Keys, false),
        ]);
        if self.edit_maps {
            items.push(("Edit maps", "Draw a map in the map editor".into(), Hit::EditMaps, false));
        }
        items.push(("Quit", String::new(), Hit::Quit, false));
        let mut y = top + 12.0 * s;
        for (label, sub, hit, primary) in items {
            let h = if sub.is_empty() { 48.0 * s } else { 62.0 * s };
            self.item(Rect::new(x, y, w.min(420.0 * s), h), label, &sub, hit, primary, focus == Some(hit), c, s);
            y += h + 12.0 * s;
        }
        if let Some(n) = self.notice.clone() {
            c.text_fit(Weight::Medium, 13.0 * s, (x, y + 14.0 * s), Align::Left, &n, w, WARN);
        }
    }

    fn missions_page(&mut self, c: &mut Canvas, x: f32, top: f32, w: f32, s: f32, focus: Option<Hit>) {
        let sh = c.screen.1 as f32;
        let n = self.missions.len();
        // Rows shrink to fit a short window.
        let room = sh - top - 140.0 * s;
        let h = (room / (n.max(1) as f32) - 10.0 * s).clamp(52.0 * s, 78.0 * s);
        let mut y = top + 12.0 * s;
        if n == 0 {
            c.text(Weight::Medium, 15.0 * s, (x, y + 24.0 * s), Align::Left, "No missions found", MUTED);
            y += 48.0 * s;
        }
        for i in 0..n {
            let e = self.missions[i].clone();
            let r = Rect::new(x, y, w, h);
            self.item(r, &format!("{}. {}", i + 1, e.name), &e.says, Hit::Mission(i), false, focus == Some(Hit::Mission(i)), c, s);
            if !e.detail.is_empty() && h >= 70.0 * s {
                c.text_fit(Weight::Medium, 11.5 * s, (r.right() - 18.0 * s, r.y + 22.0 * s), Align::Right, &e.detail, r.w * 0.4, FAINT);
            }
            y += h + 10.0 * s;
        }
        self.item(Rect::new(x, y + 8.0 * s, 140.0 * s, 42.0 * s), "Back", "", Hit::Back, false, focus == Some(Hit::Back), c, s);
    }

    fn options_page(&mut self, c: &mut Canvas, x: f32, top: f32, w: f32, s: f32, focus: Option<Hit>) {
        let n = self.options.len();
        let rh = 62.0 * s;
        let panel = Rect::new(x, top + 12.0 * s, w, n as f32 * rh + 24.0 * s);
        self.glass(panel, c, s);
        let mut y = panel.y + 12.0 * s;
        for i in 0..n {
            let o = self.options[i].clone();
            let row = Rect::new(panel.x + 8.0 * s, y, panel.w - 16.0 * s, rh - 4.0 * s);
            let focused = focus == Some(Hit::Reset(i));
            let hot = self.hover.is_some_and(|p| row.contains(p));
            if focused || hot {
                c.rrect(row, 9.0 * s, [255, 255, 255, if focused { 22 } else { 12 }]);
            }
            if focused {
                c.outline(row, 9.0 * s, 1.5 * s, ACCENT);
            }
            self.hits.push((row, Hit::Reset(i)));
            let lx = row.x + 14.0 * s;
            c.text(Weight::SemiBold, 15.5 * s, (lx, row.y + 24.0 * s), Align::Left, &o.label, TEXT);
            c.text_fit(Weight::Medium, 12.0 * s, (lx, row.y + 44.0 * s), Align::Left, &o.note, row.w * 0.52, MUTED);
            // The value between its arrows, the default under it.
            let aw = 30.0 * s;
            let vw = 150.0 * s;
            let right = row.right() - 10.0 * s;
            let next = Rect::new(right - aw, row.y + 6.0 * s, aw, 28.0 * s);
            let prev = Rect::new(next.x - vw - aw, next.y, aw, 28.0 * s);
            for (r, by, label) in [(prev, -1isize, "<"), (next, 1, ">")] {
                let hot = self.hover.is_some_and(|p| r.contains(p));
                c.panel(r, 7.0 * s, if hot { BUTTON_HOVER_TOP } else { BUTTON_TOP }, if hot { BUTTON_HOVER_BOTTOM } else { BUTTON_BOTTOM }, 1.0, [255, 255, 255, 30]);
                c.text(Weight::SemiBold, 15.0 * s, (r.x + r.w / 2.0, r.y + 19.0 * s), Align::Centre, label, TEXT);
                self.hits.push((r, Hit::Turn(i, by)));
            }
            let vc = if o.is_default() { TEXT } else { WARN };
            c.text_fit(Weight::SemiBold, 15.0 * s, (prev.right() + vw / 2.0, row.y + 26.0 * s), Align::Centre, o.value(), vw - 8.0 * s, vc);
            let d = if o.is_default() { "default".to_string() } else { format!("default {}", o.choices[o.default]) };
            c.text_fit(Weight::Medium, 11.5 * s, (prev.right() + vw / 2.0, row.y + 48.0 * s), Align::Centre, &d, vw + 2.0 * aw, if o.is_default() { FAINT } else { MUTED });
            y += rh;
        }
        // The defaults move as the game is tuned (D5: "applies to this build").
        c.text_fit(Weight::Medium, 12.0 * s, (x, panel.bottom() + 20.0 * s), Align::Left, "Defaults are this build's; they move as the game is tuned", w, FAINT);
        let by = panel.bottom() + 34.0 * s;
        self.item(Rect::new(x, by, 140.0 * s, 42.0 * s), "Back", "", Hit::Back, true, focus == Some(Hit::Back), c, s);
        self.item(Rect::new(x + 152.0 * s, by, 200.0 * s, 42.0 * s), "All to defaults", "", Hit::ResetAll, false, focus == Some(Hit::ResetAll), c, s);
    }

    /// The shop page laid out at scale `s` from `top`: the upgrade grid on
    /// the left (six tiles a row, as the command card is six by two, with a
    /// box under it saying what the tile under the cursor does), the
    /// armory on the right (gun cards, the carried gun's slots, the bullet
    /// cards by heading). The grid keeps the height of its tallest page so
    /// opening a folder does not move the rest.
    fn shop_plan(&self, shop: &Shop, screen: (f32, f32), x: f32, top: f32, s: f32, foot: f32) -> ShopPlan {
        let (sw, sh) = screen;
        let (t, g, p, head) = (60.0 * s, 6.0 * s, 14.0 * s, 26.0 * s);
        let cols = 6usize;
        let left_w = 2.0 * p + cols as f32 * t + (cols - 1) as f32 * g;
        let pages = std::iter::once(shop.rows.iter().filter(|r| r.area == ShopArea::Upgrades).count() + self.folders().len())
            .chain(self.folders().into_iter().map(|f| 1 + shop.rows.iter().filter(|r| r.area == ShopArea::Folder && r.group == f).count()));
        let grid_rows = pages.max().unwrap_or(1).div_ceil(cols).max(1);
        let grid_h = grid_rows as f32 * t + (grid_rows - 1) as f32 * g;
        let detail_h = 116.0 * s;
        let left = Rect::new(x, top, left_w, head + 8.0 * s + grid_h + 12.0 * s + detail_h + p);
        let mut tiles = Vec::new();
        for (k, h) in self.grid().into_iter().enumerate() {
            let (row, col) = (k / cols, k % cols);
            tiles.push((Rect::new(left.x + p + col as f32 * (t + g), left.y + head + 8.0 * s + row as f32 * (t + g), t, t), h));
        }
        let detail = Rect::new(left.x + p, left.y + head + 8.0 * s + grid_h + 12.0 * s, left_w - 2.0 * p, detail_h);
        // The armory: what is left of the width.
        let rx = left.right() + 16.0 * s;
        let rw = (sw - x - rx).min(980.0 * s);
        let (gw, gh) = (226.0 * s, 146.0 * s);
        let (cw, ch) = (78.0 * s, 98.0 * s);
        let mut heads = Vec::new();
        let (x0, x1) = (rx + p, rx + rw - p);
        let mut y = top + head + 8.0 * s;
        heads.push(((x0, top + 22.0 * s), "Guns".to_string()));
        // Lays `n` boxes `w` by `h` in rows from `y`; returns them and the
        // height used.
        let flow = |n: usize, w: f32, h: f32, y: f32| -> (Vec<Rect>, f32) {
            let per = (((x1 - x0) + g) / (w + g)).floor().max(1.0) as usize;
            let rects: Vec<Rect> = (0..n).map(|k| Rect::new(x0 + (k % per) as f32 * (w + g), y + (k / per) as f32 * (h + g), w, h)).collect();
            let rows = n.div_ceil(per);
            (rects, if rows == 0 { 0.0 } else { rows as f32 * (h + g) - g })
        };
        let gun_rows: Vec<usize> = shop.rows.iter().enumerate().filter(|(_, r)| r.area == ShopArea::Guns).map(|(i, _)| i).collect();
        let (rects, used) = flow(gun_rows.len(), gw, gh, y);
        let guns: Vec<(Rect, usize)> = rects.into_iter().zip(gun_rows).collect();
        y += used + 14.0 * s;
        // The carried gun's slots, the cards in it first.
        let on: Vec<usize> = shop.rows.iter().enumerate().filter(|(_, r)| r.area == ShopArea::Cards && r.on).map(|(i, _)| i).collect();
        let n_slots = (shop.slots as usize).max(on.len());
        heads.push(((x0, y + 16.0 * s), format!("In the gun  ·  budget {} of {}  ·  {} of {} slots", shop.budget.0, shop.budget.1, on.len(), shop.slots)));
        y += head;
        let (rects, used) = flow(n_slots, cw, ch, y);
        let slots: Vec<(Rect, Option<usize>)> = rects.into_iter().enumerate().map(|(k, r)| (r, on.get(k).copied())).collect();
        y += used + 14.0 * s;
        let mut cards = Vec::new();
        for (name, idx) in self.card_groups() {
            heads.push(((x0, y + 16.0 * s), name));
            y += head;
            let (rects, used) = flow(idx.len(), cw, ch, y);
            cards.extend(rects.into_iter().zip(idx));
            y += used + 14.0 * s;
        }
        let right = Rect::new(rx, top, rw, (y - top).max(left.h));
        let room = sh - foot - top;
        let fits = left.h <= room && right.h <= room && rw >= gw + 2.0 * p && left.right() <= sw;
        ShopPlan { s, left, right, tiles, guns, slots, cards, heads, detail, fits }
    }

    fn shop_page(&mut self, c: &mut Canvas, x: f32, top: f32, s: f32, focus: Option<Hit>) {
        let (sw, sh) = (c.screen.0 as f32, c.screen.1 as f32);
        let Some(shop) = self.shop.clone() else { return };
        c.text_fit(Weight::SemiBold, 18.0 * s, (x, top + 14.0 * s), Align::Left, &shop.heading, sw - 2.0 * x - 160.0 * s, TEXT);
        let mut y = top + 36.0 * s;
        for l in &shop.lines {
            c.text_fit(Weight::Medium, 13.0 * s, (x, y), Align::Left, l, sw - 2.0 * x, MUTED);
            y += 19.0 * s;
        }
        c.text(Weight::SemiBold, 18.0 * s, (sw - x, top + 14.0 * s), Align::Right, &shop.purse, ACCENT);
        // The layout starts a little over the UI's own size (the menu's
        // `s` is held down for the options page, which the shop is not)
        // and shrinks until both areas fit above the buttons.
        let body = y + 2.0 * s;
        let foot_s = s;
        let mut k = (c.ui.px(c.screen) * 1.2).max(s);
        let plan = loop {
            let plan = self.shop_plan(&shop, (sw, sh), x, body, k, 108.0 * foot_s);
            if plan.fits || k < 0.3 {
                break plan;
            }
            k *= 0.92;
        };
        let s = plan.s;
        self.glass(plan.left, c, s);
        self.glass(plan.right, c, s);
        let title = match &self.folder {
            Some(f) => format!("Upgrades  ›  {f}"),
            None => "Upgrades".to_string(),
        };
        c.text(Weight::SemiBold, 13.0 * s, (plan.left.x + 14.0 * s, plan.left.y + 22.0 * s), Align::Left, &title, FAINT);
        for (at, h) in &plan.heads {
            c.text_fit(Weight::SemiBold, 13.0 * s, *at, Align::Left, h, plan.right.w - 28.0 * s, FAINT);
        }
        let hot = |r: Rect| self.hover.is_some_and(|p| r.contains(p));
        // What the detail box describes: the tile under the cursor, else
        // the keyboard's.
        let mut shown = focus;
        let folders = self.folders();
        for &(r, h) in &plan.tiles {
            if hot(r) {
                shown = Some(h);
            }
            let focused = focus == Some(h);
            match h {
                Hit::Buy(i) => self.tile(&shop.rows[i], r, hot(r), focused, c, s),
                Hit::Folder(f) => {
                    let inside: Vec<&ShopRow> = shop.rows.iter().filter(|x| x.area == ShopArea::Folder && x.group == folders[f]).collect();
                    let icon = shop.folder_icons.iter().find(|(n, _)| *n == folders[f]).map(|x| x.1).or(inside.first().map(|x| x.icon)).unwrap_or(Icon::Menu);
                    let any = inside.iter().any(|x| x.ready);
                    self.folder_tile(&folders[f], icon, any, r, hot(r), focused, c, s);
                }
                _ => self.folder_tile("Back", Icon::Close, false, r, hot(r), focused, c, s),
            }
            self.hits.push((r, h));
        }
        for &(r, i) in &plan.guns {
            if hot(r) {
                shown = Some(Hit::Buy(i));
            }
            self.gun_card(&shop.rows[i], r, hot(r), focus == Some(Hit::Buy(i)), c, s);
            self.hits.push((r, Hit::Buy(i)));
        }
        for &(r, i) in &plan.slots {
            match i {
                Some(i) => {
                    if hot(r) {
                        shown = Some(Hit::Buy(i));
                    }
                    self.bullet_card(&shop.rows[i], r, hot(r), focus == Some(Hit::Buy(i)), c, s);
                    self.hits.push((r, Hit::Buy(i)));
                }
                None => {
                    c.rrect(r, 9.0 * s, [255, 255, 255, 6]);
                    c.outline(r, 9.0 * s, 1.0, [255, 255, 255, 30]);
                    c.text(Weight::Medium, 10.5 * s, (r.x + r.w / 2.0, r.y + r.h / 2.0 + 4.0 * s), Align::Centre, "empty", FAINT);
                }
            }
        }
        for &(r, i) in &plan.cards {
            if hot(r) {
                shown = Some(Hit::Buy(i));
            }
            self.bullet_card(&shop.rows[i], r, hot(r), focus == Some(Hit::Buy(i)), c, s);
            self.hits.push((r, Hit::Buy(i)));
        }
        self.detail(&shop, shown, &folders, plan.detail, c, s);
        // The buttons sit over the hint line (at `sh - 28` of the menu's
        // scale).
        let s = foot_s;
        let by = sh - 96.0 * s;
        self.item(Rect::new(x, by, 240.0 * s, 44.0 * s), "Land on the next planet", "", Hit::Next, true, focus == Some(Hit::Next), c, s);
        self.item(Rect::new(x + 252.0 * s, by, 170.0 * s, 44.0 * s), "Give up the run", "", Hit::EndRun, false, focus == Some(Hit::EndRun), c, s);
    }

    /// A tile's face: bright when a pick does something, dark when not.
    #[allow(clippy::too_many_arguments)]
    fn face(&self, r: Rect, ready: bool, on: bool, hot: bool, focused: bool, c: &mut Canvas, s: f32) {
        let (t, b) = match (ready || on, hot) {
            (true, false) => (BUTTON_TOP, BUTTON_BOTTOM),
            (true, true) => (BUTTON_HOVER_TOP, BUTTON_HOVER_BOTTOM),
            (false, false) => ([18, 22, 28, 235], [13, 16, 21, 235]),
            (false, true) => ([28, 33, 40, 240], [20, 24, 30, 240]),
        };
        let edge = if on { [96, 190, 255, 200] } else if ready { [255, 255, 255, 34] } else { [255, 255, 255, 12] };
        c.panel(r, 9.0 * s, t, b, if on { 1.5 * s } else { 1.0 }, edge);
        if focused {
            c.outline(r.inset(-2.0 * s), 11.0 * s, 1.5 * s, ACCENT);
        }
    }

    /// An upgrade tile, drawn as the command card draws a command: the
    /// icon in the middle, the name under it, levels as pips, the price.
    #[allow(clippy::too_many_arguments)]
    fn tile(&self, row: &ShopRow, r: Rect, hot: bool, focused: bool, c: &mut Canvas, s: f32) {
        let owned = row.level.1 > 0 && row.level.0 >= row.level.1;
        self.face(r, row.ready || owned, false, hot, focused, c, s);
        let dim = if row.ready || owned { 1.0 } else { 0.42 };
        let ic = 24.0 * s;
        c.icon(row.icon, (r.x + (r.w - ic) / 2.0, r.y + 8.0 * s), ic, with_alpha(if owned { GOOD } else { row.tint }, dim));
        let px = fit_px(c, &row.name, 9.5 * s, 7.5 * s, r.w - 6.0 * s);
        c.text_fit(Weight::SemiBold, px, (r.x + r.w / 2.0, r.y + 44.0 * s), Align::Centre, &row.name, r.w - 6.0 * s, with_alpha(TEXT, dim.max(0.6)));
        if row.level.1 > 1 {
            let d = 4.0 * s;
            for k in 0..row.level.1 {
                let at = Rect::new(r.x + 5.0 * s + k as f32 * (d + 2.0 * s), r.bottom() - 9.0 * s, d, d);
                c.rrect(at, d / 2.0, if k < row.level.0 { ACCENT } else { [255, 255, 255, 40] });
            }
        }
        if owned {
            c.icon(Icon::Check, (r.right() - 14.0 * s, r.y + 4.0 * s), 10.0 * s, GOOD);
        } else if !row.price.is_empty() {
            c.text(Weight::SemiBold, 9.5 * s, (r.right() - 4.0 * s, r.bottom() - 4.0 * s), Align::Right, &row.price, with_alpha(if row.ready { ENERGY } else { BAD }, 0.95));
        }
    }

    /// A folder on the grid (or its way back): its icon, its name and a
    /// dot when something in it can be bought now.
    #[allow(clippy::too_many_arguments)]
    fn folder_tile(&self, name: &str, icon: Icon, any: bool, r: Rect, hot: bool, focused: bool, c: &mut Canvas, s: f32) {
        self.face(r, true, false, hot, focused, c, s);
        c.outline(r.inset(3.0 * s), 7.0 * s, 1.0, [96, 190, 255, 60]);
        let ic = 24.0 * s;
        c.icon(icon, (r.x + (r.w - ic) / 2.0, r.y + 8.0 * s), ic, ACCENT);
        c.text_fit(Weight::SemiBold, 9.5 * s, (r.x + r.w / 2.0, r.y + 44.0 * s), Align::Centre, name, r.w - 6.0 * s, TEXT);
        c.text(Weight::SemiBold, 11.0 * s, (r.right() - 6.0 * s, r.bottom() - 4.0 * s), Align::Right, if name == "Back" { "‹" } else { "›" }, MUTED);
        if any {
            c.rrect(Rect::new(r.right() - 11.0 * s, r.y + 5.0 * s, 6.0 * s, 6.0 * s), 3.0 * s, ENERGY);
        }
    }

    /// A gun card: its kind's icon, its name, its price or where it is,
    /// and its numbers as bars.
    #[allow(clippy::too_many_arguments)]
    fn gun_card(&self, row: &ShopRow, r: Rect, hot: bool, focused: bool, c: &mut Canvas, s: f32) {
        self.face(r, row.ready, row.on, hot, focused, c, s);
        let p = 12.0 * s;
        let dim = if row.ready || row.on { 1.0 } else { 0.5 };
        let ic = 30.0 * s;
        c.icon(row.icon, (r.x + p, r.y + p), ic, with_alpha(row.tint, dim));
        c.text_fit(Weight::SemiBold, 14.5 * s, (r.x + p + ic + 10.0 * s, r.y + p + 13.0 * s), Align::Left, &row.name, r.w - 2.0 * p - ic - 10.0 * s, with_alpha(TEXT, dim.max(0.6)));
        let (tag, col) = if !row.price.is_empty() {
            (format!("{}  ·  {}", row.price, row.state), if row.ready { ACCENT } else { BAD })
        } else {
            (row.state.clone(), if row.on { GOOD } else { MUTED })
        };
        c.text_fit(Weight::Medium, 11.0 * s, (r.x + p + ic + 10.0 * s, r.y + p + 29.0 * s), Align::Left, &tag, r.w - 2.0 * p - ic - 10.0 * s, col);
        let mut y = r.y + 56.0 * s;
        let step = ((r.bottom() - p - y) / row.bars.len().max(1) as f32).min(16.0 * s);
        for (label, f, value) in &row.bars {
            c.text(Weight::Medium, 10.0 * s, (r.x + p, y + 4.0 * s), Align::Left, label, FAINT);
            let bar = Rect::new(r.x + p + 50.0 * s, y - 1.0 * s, r.w - 2.0 * p - 50.0 * s - 46.0 * s, 5.0 * s);
            c.bar(bar, *f, with_alpha(row.tint, 0.85 * dim), [255, 255, 255, 22]);
            c.text(Weight::SemiBold, 10.0 * s, (r.right() - p, y + 4.0 * s), Align::Right, value, with_alpha(TEXT, dim.max(0.6)));
            y += step;
        }
    }

    /// A bullet card: a band of its kind's colour, its icon, its name, its
    /// budget cost as pips and its price.
    #[allow(clippy::too_many_arguments)]
    fn bullet_card(&self, row: &ShopRow, r: Rect, hot: bool, focused: bool, c: &mut Canvas, s: f32) {
        self.face(r, row.ready, row.on, hot, focused, c, s);
        let dim = if row.ready || row.on { 1.0 } else { 0.45 };
        c.rrect(Rect::new(r.x + 6.0 * s, r.y + 5.0 * s, r.w - 12.0 * s, 4.0 * s), 2.0 * s, with_alpha(row.tint, 0.9 * dim));
        let ic = 28.0 * s;
        c.icon(row.icon, (r.x + (r.w - ic) / 2.0, r.y + 18.0 * s), ic, with_alpha(row.tint, dim));
        c.text_fit(Weight::SemiBold, 11.0 * s, (r.x + r.w / 2.0, r.y + 62.0 * s), Align::Centre, &row.name, r.w - 8.0 * s, with_alpha(TEXT, dim.max(0.6)));
        let d = 5.0 * s;
        for k in 0..row.cost.min(6) {
            c.rrect(Rect::new(r.x + 7.0 * s + k as f32 * (d + 2.0 * s), r.bottom() - 11.0 * s, d, d), d / 2.0, with_alpha(ENERGY, 0.85 * dim.max(0.6)));
        }
        if row.cost == 0 {
            c.text(Weight::Medium, 9.0 * s, (r.x + 7.0 * s, r.bottom() - 6.0 * s), Align::Left, "free", FAINT);
        }
        if !row.price.is_empty() {
            c.text(Weight::SemiBold, 10.5 * s, (r.right() - 6.0 * s, r.bottom() - 6.0 * s), Align::Right, &row.price, if row.ready { ACCENT } else { BAD });
        }
    }

    /// The box under the grid: what the tile under the cursor (or the
    /// keyboard's) is, does and costs.
    #[allow(clippy::too_many_arguments)]
    fn detail(&self, shop: &Shop, shown: Option<Hit>, folders: &[String], r: Rect, c: &mut Canvas, s: f32) {
        c.panel(r, 9.0 * s, [10, 13, 17, 220], [8, 10, 13, 220], 1.0, [255, 255, 255, 14]);
        let p = 12.0 * s;
        let (name, about, price, state, ready) = match shown {
            Some(Hit::Buy(i)) => {
                let row = &shop.rows[i];
                (row.name.clone(), row.about.clone(), row.price.clone(), row.state.clone(), row.ready)
            }
            Some(Hit::Folder(f)) => {
                let n = shop.rows.iter().filter(|x| x.area == ShopArea::Folder && x.group == folders[f]).count();
                (folders[f].clone(), format!("Open: {n} to unlock or upgrade, as the command card's own page."), String::new(), String::new(), true)
            }
            Some(Hit::FolderBack) => ("Back".into(), "Back to the first page of upgrades.".into(), String::new(), String::new(), true),
            _ => ("Point at a tile".into(), "Upgrades on the left, guns and their bullet cards on the right. Bright tiles are what your points buy now; a card in the gun comes out with a click, a card in the bag goes in.".into(), String::new(), String::new(), true),
        };
        c.text_fit(Weight::SemiBold, 15.0 * s, (r.x + p, r.y + 22.0 * s), Align::Left, &name, r.w - 2.0 * p - 120.0 * s, TEXT);
        let tag = if price.is_empty() { state } else if state.is_empty() { format!("{price} points") } else { format!("{price} points  ·  {state}") };
        c.text_fit(Weight::SemiBold, 12.0 * s, (r.right() - p, r.y + 22.0 * s), Align::Right, &tag, 120.0 * s, if !price.is_empty() && !ready { BAD } else { ACCENT });
        let lead = 16.0 * s;
        let lines = c.wrap(Weight::Medium, 12.0 * s, r.w - 2.0 * p, &about);
        let room = ((r.h - 34.0 * s) / lead).floor().max(1.0) as usize;
        for (k, l) in lines.iter().take(room).enumerate() {
            c.text(Weight::Medium, 12.0 * s, (r.x + p, r.y + 42.0 * s + k as f32 * lead), Align::Left, l, MUTED);
        }
    }

    fn keys_page(&mut self, c: &mut Canvas, x: f32, top: f32, w: f32, s: f32) {
        let sh = c.screen.1 as f32;
        let rows = self.keys.clone();
        // Two columns when one does not fit.
        let lh = 22.0 * s;
        let room = sh - top - 130.0 * s;
        let per = ((room - 24.0 * s) / lh).floor().max(1.0) as usize;
        let cols = if rows.len() > per { 2 } else { 1 };
        let col_w = if cols == 1 { w } else { (w * 1.6).min(c.screen.0 as f32 - 2.0 * x) / 2.0 };
        let per = rows.len().div_ceil(cols);
        let panel = Rect::new(x, top + 12.0 * s, col_w * cols as f32, per as f32 * lh + 30.0 * s);
        self.glass(panel, c, s);
        for (k, (key, what)) in rows.iter().enumerate() {
            let (col, row) = (k / per.max(1), k % per.max(1));
            let cx = panel.x + 20.0 * s + col as f32 * col_w;
            let y = panel.y + 28.0 * s + row as f32 * lh;
            let kw = (col_w * 0.34).min(170.0 * s);
            c.text_fit(Weight::SemiBold, 13.0 * s, (cx, y), Align::Left, key, kw - 8.0 * s, ACCENT);
            c.text_fit(Weight::Medium, 13.0 * s, (cx + kw, y), Align::Left, what, col_w - kw - 28.0 * s, TEXT);
        }
        self.item(Rect::new(x, panel.bottom() + 16.0 * s, 140.0 * s, 42.0 * s), "Back", "", Hit::Back, true, true, c, s);
    }

    /// Where `hit` was last drawn (tests and the window's smoke hand).
    fn rect_of(&self, hit: Hit) -> Option<Rect> {
        self.hits.iter().find(|(_, h)| *h == hit).map(|(r, _)| *r)
    }

    /// The centre of the button that starts mission `i`, as last drawn.
    pub fn mission_at(&self, i: usize) -> Option<(f32, f32)> {
        self.rect_of(Hit::Mission(i)).map(|r| (r.x + r.w / 2.0, r.y + r.h / 2.0))
    }
}

fn digit(code: KeyCode) -> Option<usize> {
    use KeyCode as K;
    let ds = [K::Digit0, K::Digit1, K::Digit2, K::Digit3, K::Digit4, K::Digit5, K::Digit6, K::Digit7, K::Digit8, K::Digit9];
    ds.iter().position(|&d| d == code)
}

#[cfg(test)]
mod tests {
    use super::*;

    fn menu() -> StartMenu {
        let missions = (1..=3).map(|i| Entry { stem: format!("{i}-m"), name: format!("M{i}"), says: "Do it".into(), detail: "Fork".into() }).collect();
        let options = vec![
            Opt::new("own", "Own mould", &["off", "on"], "off", "off", "A second mould of yours"),
            Opt::new("panic", "Panic", &["0", "1", "2", "4"], "2", "2", "A cut-off part panics"),
        ];
        StartMenu::new("Larder World", missions, options, vec![("A".into(), "Attack".into())])
    }

    fn click(m: &mut StartMenu, c: &mut Canvas, hit: Hit) -> Vec<StartEvent> {
        c.clear();
        m.draw(c);
        let r = m.rect_of(hit).unwrap_or_else(|| panic!("{hit:?} not drawn on {:?}", m.page));
        m.moved(Some((r.x + r.w / 2.0, r.y + r.h / 2.0)));
        assert!(m.button(Button::Left, true).is_empty());
        m.button(Button::Left, false)
    }

    #[test]
    fn an_option_starts_at_a_given_value_and_keeps_its_default() {
        let o = Opt::new("bullet", "Bullets", &["0", "45"], "45", "30", "");
        assert_eq!((o.value(), o.choices[o.default].as_str(), o.is_default()), ("30", "45", false));
        let o = Opt::new("wither", "Wither", &["0"], "0.01", "0.01", "");
        assert!(o.is_default() && o.choices.len() == 2);
    }

    #[test]
    fn the_keyboard_walks_the_pages() {
        let mut m = menu();
        assert_eq!(m.key(KeyCode::Enter, true), vec![StartEvent::Sandbox]);
        m.key(KeyCode::ArrowDown, true);
        assert_eq!(m.key(KeyCode::Enter, true), vec![StartEvent::Run], "the run is second");
        m.key(KeyCode::ArrowDown, true);
        assert!(m.key(KeyCode::Enter, true).is_empty());
        assert_eq!(m.page, Page::Missions);
        assert_eq!(m.key(KeyCode::Digit2, true), vec![StartEvent::Mission(1)]);
        assert!(m.key(KeyCode::Digit9, true).is_empty(), "no ninth mission");
        m.key(KeyCode::ArrowDown, true);
        assert_eq!(m.key(KeyCode::Enter, true), vec![StartEvent::Mission(1)]);
        m.key(KeyCode::Escape, true);
        assert_eq!((m.page, m.focus), (Page::Main, 2), "back on the campaign row");
        m.key(KeyCode::ArrowUp, true);
        m.key(KeyCode::ArrowUp, true);
        m.key(KeyCode::ArrowUp, true);
        assert_eq!(m.key(KeyCode::Enter, true), vec![StartEvent::Quit], "up from the top wraps to quit");
    }

    /// A shop like the run's: `n` bullet cards beside eight upgrades, two
    /// folders, three guns.
    fn shop(n: usize) -> Shop {
        let mut rows = Vec::new();
        let mut add = |group: &str, area: ShopArea, k: usize| {
            let mut r = ShopRow::new(group, area, Icon::Wall, &format!("{group} {k}"));
            (r.about, r.price, r.state, r.ready, r.level) = ("Does it".into(), format!("{}", 100 + k), "0/3".into(), k.is_multiple_of(2), (1, 3));
            if area == ShopArea::Guns {
                r.bars = vec![("Range".into(), 0.5, "24".into()), ("Rate".into(), 0.3, "1.0".into()), ("Budget".into(), 0.4, "10".into()), ("Slots".into(), 0.6, "4".into())];
            }
            r.cost = k as u32 % 4;
            r.on = area == ShopArea::Cards && k < 3 || area == ShopArea::Guns && k == 0;
            rows.push(r);
        };
        for k in 0..8 {
            add("Squad", ShopArea::Upgrades, k);
        }
        for k in 0..7 {
            add("Buildings", ShopArea::Folder, k);
        }
        for k in 0..5 {
            add("Tools", ShopArea::Folder, k);
        }
        for k in 0..3 {
            add("Guns", ShopArea::Guns, k);
        }
        for k in 0..n {
            add(if k < n / 2 { "In the bag" } else { "For sale" }, ShopArea::Cards, k);
        }
        Shop { heading: "Planet 2 next".into(), lines: vec!["1 planet freed".into()], purse: "200 points".into(), rows, slots: 4, budget: (6, 10), folder_icons: Vec::new() }
    }

    /// Every tile of the shop, on the first grid page and in each folder,
    /// and the two buttons, are on screen, apart and clickable at every
    /// size, with as many cards as a run gathers and more.
    #[test]
    fn the_shop_fits_and_its_rows_pick() {
        for screen in [(1280, 720), (1600, 900), (1920, 1080)] {
            for user in crate::world::settings::STEPS {
                for n in [6, 14, 24] {
                    for folder in [None, Some(0), Some(1)] {
                        let mut m = menu();
                        m.shop = Some(shop(n));
                        m.show(Page::Shop);
                        if let Some(k) = folder {
                            m.act(Hit::Folder(k));
                        }
                        let mut c = Canvas::new(screen);
                        c.ui.user = user;
                        m.draw(&mut c);
                        let mut rects: Vec<Rect> = Vec::new();
                        for h in m.rows() {
                            let r = m.rect_of(h).unwrap_or_else(|| panic!("{screen:?} {user} {n} {folder:?}: {h:?} not drawn"));
                            assert!(r.bottom() <= screen.1 as f32 + 0.5 && r.right() <= screen.0 as f32 + 0.5 && r.x >= 0.0, "{screen:?} at {user}, {n} cards: {h:?} {r:?} off screen");
                            assert!(rects.iter().all(|q| !(q.x < r.right() && r.x < q.right() && q.y < r.bottom() && r.y < q.bottom())), "{screen:?} at {user}, {n} cards: {h:?} overlaps");
                            rects.push(r);
                        }
                    }
                }
            }
        }
        let mut m = menu();
        m.shop = Some(shop(6));
        m.show(Page::Shop);
        let mut c = Canvas::new((1280, 800));
        assert_eq!(click(&mut m, &mut c, Hit::Buy(3)), vec![StartEvent::Buy(3)]);
        // Buildings is a folder: its tiles show once it is opened.
        assert!(m.rect_of(Hit::Buy(8)).is_none());
        assert!(click(&mut m, &mut c, Hit::Folder(0)).is_empty());
        assert_eq!(click(&mut m, &mut c, Hit::Buy(8)), vec![StartEvent::Buy(8)]);
        assert!(m.rect_of(Hit::Buy(0)).is_none(), "the first page's tiles are put away");
        assert!(click(&mut m, &mut c, Hit::FolderBack).is_empty());
        assert!(click(&mut m, &mut c, Hit::Folder(1)).is_empty());
        assert_eq!(click(&mut m, &mut c, Hit::Buy(15)), vec![StartEvent::Buy(15)], "the first tool");
        // Esc closes the folder first, onto its tile, then the shop.
        m.key(KeyCode::Escape, true);
        assert_eq!((m.page, m.rows()[m.focus]), (Page::Shop, Hit::Folder(1)));
        // Guns and cards pick from either area.
        assert_eq!(click(&mut m, &mut c, Hit::Buy(20)), vec![StartEvent::Buy(20)], "a gun");
        assert_eq!(click(&mut m, &mut c, Hit::Buy(25)), vec![StartEvent::Buy(25)], "a card");
        assert_eq!(click(&mut m, &mut c, Hit::Next), vec![StartEvent::NextLevel]);
        assert_eq!(click(&mut m, &mut c, Hit::EndRun), vec![StartEvent::EndRun]);
        m.focus = 0;
        assert_eq!(m.key(KeyCode::Enter, true), vec![StartEvent::Buy(0)]);
        m.key(KeyCode::ArrowRight, true);
        assert_eq!(m.key(KeyCode::Enter, true), vec![StartEvent::Buy(1)], "right walks the grid");
        m.key(KeyCode::Escape, true);
        assert_eq!((m.page, m.rows()[m.focus]), (Page::Main, Hit::Run));
    }

    /// The main page with every row (Run, Random planet, Edit maps: eight)
    /// stays on screen at every size and UI scale, with the game's eleven
    /// options (whose page sets the scale for every page).
    #[test]
    fn the_full_main_page_fits() {
        for screen in [(1280, 720), (1600, 900), (1920, 1080)] {
            for user in crate::world::settings::STEPS {
                let mut m = menu();
                while m.options.len() < 11 {
                    let k = format!("o{}", m.options.len());
                    m.options.push(Opt::new(&k, &k, &["0", "1"], "0", "0", ""));
                }
                (m.planets, m.edit_maps, m.run_live) = (true, true, true);
                let mut c = Canvas::new(screen);
                c.ui.user = user;
                m.draw(&mut c);
                for h in m.rows() {
                    let r = m.rect_of(h).unwrap_or_else(|| panic!("{h:?} not drawn"));
                    assert!(r.bottom() <= screen.1 as f32 - 30.0, "{screen:?} at {user}: {h:?} {r:?} reaches the hint line");
                }
            }
        }
    }

    /// "Random planet" (the roguelike's planets, 2026-11-14) shows only
    /// when the game makes planets, after the campaign, and the keyboard
    /// walks to it in the order drawn.
    #[test]
    fn random_planet_shows_only_when_the_game_makes_planets() {
        let mut m = menu();
        let mut c = Canvas::new((1280, 800));
        m.draw(&mut c);
        assert!(m.rect_of(Hit::RandomPlanet).is_none());
        m.planets = true;
        assert_eq!(click(&mut m, &mut c, Hit::RandomPlanet), vec![StartEvent::RandomPlanet]);
        assert_eq!(m.rows()[3], Hit::RandomPlanet, "after Sandbox, Run and Campaign");
        let y = |m: &StartMenu, h: Hit| m.rect_of(h).unwrap().y;
        assert!(y(&m, Hit::Campaign) < y(&m, Hit::RandomPlanet) && y(&m, Hit::RandomPlanet) < y(&m, Hit::Options));
        m.focus = 0;
        for _ in 0..3 {
            m.key(KeyCode::ArrowDown, true);
        }
        assert_eq!(m.key(KeyCode::Enter, true), vec![StartEvent::RandomPlanet]);
    }

    #[test]
    fn edit_maps_shows_only_when_the_game_has_an_editor() {
        let mut m = menu();
        let mut c = Canvas::new((1280, 800));
        m.draw(&mut c);
        assert!(m.rect_of(Hit::EditMaps).is_none());
        m.edit_maps = true;
        assert_eq!(click(&mut m, &mut c, Hit::EditMaps), vec![StartEvent::EditMaps]);
        // Quit is still last, and Esc from the key card lands on Keys.
        m.key(KeyCode::ArrowUp, true);
        assert_eq!(m.key(KeyCode::Enter, true), vec![StartEvent::Quit]);
        m.show(Page::Keys);
        m.key(KeyCode::Escape, true);
        assert_eq!(m.rows()[m.focus], Hit::Keys);
    }

    #[test]
    fn options_turn_and_go_back_to_their_defaults() {
        let mut m = menu();
        m.show(Page::Options);
        m.key(KeyCode::ArrowDown, true);
        m.key(KeyCode::ArrowLeft, true);
        assert_eq!(m.changed("panic"), Some("1"));
        assert_eq!(m.changed("own"), None);
        m.key(KeyCode::Backspace, true);
        assert_eq!(m.page, Page::Options, "Backspace on the options page resets, not back");
        assert_eq!(m.changed("panic"), None);
        let mut c = Canvas::new((1280, 800));
        click(&mut m, &mut c, Hit::Turn(0, 1));
        assert_eq!(m.changed("own"), Some("on"));
        click(&mut m, &mut c, Hit::Turn(1, -1));
        click(&mut m, &mut c, Hit::ResetAll);
        assert!(m.options.iter().all(|o| o.is_default()));
        click(&mut m, &mut c, Hit::Turn(1, 1));
        click(&mut m, &mut c, Hit::Reset(1));
        assert!(m.option("panic").unwrap().is_default(), "a click on the row resets it");
    }

    #[test]
    fn f1_and_question_mark_open_the_key_card_from_any_page_and_go_back() {
        let mut m = menu();
        m.show(Page::Options);
        m.key(KeyCode::F1, true);
        assert_eq!(m.page, Page::Keys);
        m.key(KeyCode::Slash, true);
        assert_eq!(m.page, Page::Options);
        m.key(KeyCode::Slash, true);
        m.key(KeyCode::Escape, true);
        assert_eq!(m.page, Page::Options);
    }

    /// At every UI size the options page's last row and its buttons stay
    /// on the screen (they ran off at 150% on 900 lines before the cap).
    #[test]
    fn every_ui_size_keeps_the_options_page_on_screen() {
        for screen in [(1280, 800), (1600, 900), (1920, 1080)] {
            for user in crate::world::settings::STEPS {
                let mut m = menu();
                m.show(Page::Options);
                let mut c = Canvas::new(screen);
                c.ui.user = user;
                m.draw(&mut c);
                let last = m.options.len() - 1;
                for h in [Hit::Reset(last), Hit::Back, Hit::ResetAll] {
                    let r = m.rect_of(h).unwrap();
                    assert!(r.bottom() <= screen.1 as f32 && r.right() <= screen.0 as f32, "{screen:?} at {user}: {h:?} {r:?}");
                }
            }
        }
    }

    /// The owner, 11-14 20:23Z: "changing the UI zoom, doesn't change the
    /// main menu". The main page grows with the UI size and stays on screen.
    #[test]
    fn the_main_page_follows_the_ui_size() {
        for screen in [(1280, 800), (1600, 900), (1920, 1080), (2560, 1440)] {
            let size = |user: f32| {
                let mut m = menu();
                let mut c = Canvas::new(screen);
                c.ui.user = user;
                m.draw(&mut c);
                let r = m.rect_of(Hit::Quit).unwrap();
                assert!(r.bottom() <= screen.1 as f32 && r.right() <= screen.0 as f32, "{screen:?} at {user}: {r:?}");
                m.rect_of(Hit::Sandbox).unwrap().h
            };
            assert!(size(1.5) > size(1.0) * 1.2, "{screen:?}: 150% grows the main page");
            assert!(size(0.75) < size(1.0), "{screen:?}: 75% shrinks it");
        }
    }

    #[test]
    fn clicks_pick_rounds_at_every_size() {
        for screen in [(640, 400), (1280, 800), (1920, 1080), (3840, 2160)] {
            let mut m = menu();
            let mut c = Canvas::new(screen);
            assert_eq!(click(&mut m, &mut c, Hit::Sandbox), vec![StartEvent::Sandbox], "{screen:?}");
            click(&mut m, &mut c, Hit::Campaign);
            assert_eq!(click(&mut m, &mut c, Hit::Mission(2)), vec![StartEvent::Mission(2)], "{screen:?}");
            click(&mut m, &mut c, Hit::Back);
            click(&mut m, &mut c, Hit::Keys);
            assert_eq!(m.page, Page::Keys);
            click(&mut m, &mut c, Hit::Back);
            assert_eq!(click(&mut m, &mut c, Hit::Quit), vec![StartEvent::Quit]);
            // A press on one button and a release on another is no click.
            c.clear();
            m.draw(&mut c);
            let a = m.rect_of(Hit::Sandbox).unwrap();
            let q = m.rect_of(Hit::Quit).unwrap();
            m.moved(Some((a.x + 4.0, a.y + 4.0)));
            m.button(Button::Left, true);
            m.moved(Some((q.x + 4.0, q.y + 4.0)));
            assert!(m.button(Button::Left, false).is_empty());
        }
    }
}
