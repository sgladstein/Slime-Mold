//! **The start menu** (cloud plan 2026-11-14, item C2): what a player sees
//! before a round. Its pages: the main page (sandbox, campaign, options,
//! keys, quit), the campaign's mission list, an options page for the
//! window's switches in tabs with the focused one's note and default
//! beside them, and the key card (its last tab; also F1 or ? from any
//! page). The inner pages (Options, the keys, the campaign) fit one
//! screen under a one-line header and page a list too long for it rather
//! than scroll (owner, 11-15 17:31Z: "Having to scroll is horrible");
//! the main page and the shop still scroll until the owner picks their
//! layouts (`Reports/menu-layout-research-2026-11-15/`).
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
//! option (on the keys, turn the page), Tab the next Options tab,
//! PageUp/PageDown a paged list's page, Backspace sets an option back to
//! its default, Esc goes back a page, 1-9 start that mission on the
//! campaign page.

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
    /// The Options page's tab it sits on (`in_tab`; the first tab when
    /// empty). Tabs keep each list to a screen (owner, 11-15 17:31Z:
    /// "Having to scroll is horrible").
    pub tab: String,
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
        Opt { key: key.into(), label: label.into(), choices: c, default, at, note: note.into(), tab: String::new() }
    }

    /// The same option on the Options page's tab `tab`.
    pub fn in_tab(mut self, tab: &str) -> Self {
        self.tab = tab.into();
        self
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
    /// A rolled bullet's set (slug, needle, buckshot), as a glyph in the
    /// card's corner, as the HUD's loadout strip badges it.
    pub badge: Option<Icon>,
}

impl ShopRow {
    /// A row with the plain look; the game sets the rest.
    pub fn new(group: &str, area: ShopArea, icon: Icon, name: &str) -> Self {
        ShopRow { group: group.into(), area, icon, tint: TEXT, name: name.into(), about: String::new(), price: String::new(), state: String::new(), ready: false, on: false, level: (0, 0), bars: Vec::new(), cost: 0, badge: None }
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
    /// The gun range's armory, not a run's shop: its buttons go back to
    /// the range or leave it, and Esc goes back to the range.
    pub range: bool,
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
    /// Below the slots: a gun card dropped here comes out.
    bag: Rect,
    heads: Vec<((f32, f32), String)>,
    detail: Rect,
    fits: bool,
    /// It fits the window's width (the height can scroll).
    fits_w: bool,
}

/// The Options page's last tab, the key list (also F1 or ?).
const KEYS_TAB: &str = "Keys";

/// The tab `o` sits on.
fn tab_name(o: &Opt) -> &str {
    if o.tab.is_empty() { "Game" } else { &o.tab }
}

/// The page of a list `per` rows a page that holds row `at`, as the range
/// of rows it shows, and how many pages there are.
fn sheet_of(n: usize, per: usize, sheet: usize) -> (std::ops::Range<usize>, usize) {
    let per = per.max(1);
    let pages = n.div_ceil(per).max(1);
    let p = sheet.min(pages - 1);
    (p * per..((p + 1) * per).min(n), pages)
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
    /// The gun range's armory (owner, 11-15 02:55Z: "a test environment
    /// ... where I can play with the guns and cards and bullets").
    Range,
    /// Shop row `i` picked.
    Buy(usize),
    /// Shop row `i` (a bullet card) dragged to the carried gun's slot
    /// `slot`, 0 the first it fires (the owner, 20:32Z: "I don't see how
    /// to move cards around").
    Move(usize, usize),
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
    Range,
    Buy(usize),
    /// Open the shop's folder `k` (`StartMenu::folders`), or close it.
    Folder(usize),
    FolderBack,
    /// An empty slot of the carried gun (a place to drop a card).
    Slot(usize),
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
    /// Option `i`'s row: a click focuses it, Enter turns it on.
    Reset(usize),
    /// The "default X" under a changed option: a click resets it.
    Default(usize),
    ResetAll,
    /// The Options page's tab `k` (the last is the key list).
    Tab(usize),
    /// A paged list one page back or on (the key list; a tab or the
    /// mission list longer than the screen).
    Flip(isize),
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
    /// Where the left button went down, for telling a drag from a click.
    press_at: Option<(f32, f32)>,
    /// The bullet card (shop row) being dragged.
    drag: Option<usize>,
    /// The carried gun's slots as last drawn, in firing order, and the
    /// bag's area (a gun card dropped there comes out).
    slot_rects: Vec<Rect>,
    bag: Option<Rect>,
    hits: Vec<(Rect, Hit)>,
    /// How far the page is scrolled (pixels), the most it can be (as last
    /// drawn), and the rows it scrolls in.
    scroll: f32,
    scroll_max: f32,
    body: Option<Rect>,
    /// Every hit as drawn, scrolled out of view or not (`reveal`).
    drawn: Vec<(Rect, Hit)>,
    /// The keyboard moved the focus: scroll it into view.
    follow: bool,
    /// The Options page's tab showing (the key list's is the last).
    opt_tab: usize,
    /// The page showing of a paged list, and how many it has, as last
    /// drawn.
    sheet: usize,
    sheets: usize,
    /// Rows a page of it, as last drawn.
    per: usize,
}

/// The inner pages' header band and the hint line under them, UI units.
const HEAD: f32 = 56.0;
const FOOT: f32 = 44.0;

/// How far the pointer moves, held down on a card, before a click becomes
/// a drag (pixels).
const DRAG_FROM: f32 = 6.0;
/// A bullet card in the shop, UI units (landscape since the UI review,
/// 11-15: 78 x 98 before).
const CARD_W: f32 = 184.0;
const CARD_H: f32 = 64.0;

impl StartMenu {
    pub fn new(title: &str, missions: Vec<Entry>, options: Vec<Opt>, keys: Vec<(String, String)>) -> Self {
        StartMenu { title: title.into(), missions, options, keys, page: Page::Main, edit_maps: false, planets: false, notice: None, shop: None, run_live: false, folder: None, focus: 0, keys_from: Page::Main, hover: None, pressed: None, press_at: None, drag: None, slot_rects: Vec::new(), bag: None, hits: Vec::new(), scroll: 0.0, scroll_max: 0.0, body: None, drawn: Vec::new(), follow: false, opt_tab: 0, sheet: 0, sheets: 1, per: 1 }
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
        self.scroll = 0.0;
        self.sheet = 0;
    }

    /// The Options page's tabs: the options' own, in the order they come
    /// (an option with none is on the first, "Game"), then the key list.
    fn tabs(&self) -> Vec<String> {
        let mut t: Vec<String> = Vec::new();
        for o in &self.options {
            let n = tab_name(o);
            if !t.iter().any(|x| x == n) {
                t.push(n.to_string());
            }
        }
        t.push(KEYS_TAB.into());
        t
    }

    /// The options on the tab showing, by index.
    fn tab_opts(&self) -> Vec<usize> {
        let tabs = self.tabs();
        let Some(name) = tabs.get(self.opt_tab).filter(|n| *n != KEYS_TAB) else { return Vec::new() };
        (0..self.options.len()).filter(|&i| tab_name(&self.options[i]) == name).collect()
    }

    /// Shows the Options page's tab `k` (the last is the key list).
    /// Whether the page shows the Options tabs: Options, and the key list
    /// unless the shop or the campaign opened it (F1 there), whose Back
    /// goes back there.
    fn tabbed(&self) -> bool {
        self.page == Page::Options || (self.page == Page::Keys && matches!(self.keys_from, Page::Main | Page::Options))
    }

    fn open_tab(&mut self, k: usize) {
        let n = self.tabs().len();
        let k = k % n;
        if k + 1 == n {
            if self.page != Page::Keys {
                self.show(Page::Keys);
                // A tab of Options: Back leaves Options.
                self.keys_from = Page::Main;
            }
        } else {
            if self.page != Page::Options {
                self.show(Page::Options);
            }
            self.opt_tab = k;
            self.focus = 0;
            self.sheet = 0;
        }
    }

    /// The mouse wheel: a page taller than the window scrolls (`notches`
    /// up is positive, as the window gives them).
    pub fn wheel(&mut self, notches: i32) {
        let step = 90.0 * self.body.map_or(1.0, |b| (b.h / 600.0).max(0.5));
        self.scroll = (self.scroll - notches as f32 * step).clamp(0.0, self.scroll_max);
    }

    /// Scrolls so `hit`, as last drawn, is in view (the tests; the
    /// keyboard's focus follows by itself). Draw again to see it there.
    #[cfg(test)]
    fn reveal(&mut self, hit: Hit) {
        // On a paged list (Options, the campaign) the page that holds it:
        // the keyboard's row picks the page, so it takes the focus (and an
        // option's tab opens).
        if !self.drawn.iter().any(|(_, h)| *h == hit) {
            if let (Page::Options, Hit::Reset(i)) = (self.page, hit) {
                if let Some(k) = self.options.get(i).and_then(|o| self.tabs().iter().position(|t| t == tab_name(o))) {
                    self.opt_tab = k;
                }
            }
            if let Some(k) = self.rows().iter().position(|&h| h == hit) {
                self.focus = k;
            }
        }
        if let Some(r) = self.drawn.iter().find(|(_, h)| *h == hit).map(|(r, _)| *r) {
            self.scroll_to(r);
        }
    }

    fn scroll_to(&mut self, r: Rect) {
        let Some(b) = self.body else { return };
        let pad = 12.0 * (b.h / 600.0).clamp(0.5, 3.0);
        if r.bottom() > b.bottom() {
            self.scroll += r.bottom() - b.bottom() + pad;
        } else if r.y < b.y {
            self.scroll -= b.y - r.y + pad;
        }
        self.scroll = self.scroll.clamp(0.0, self.scroll_max.max(self.scroll));
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

    /// Shop row `i`'s slot in the carried gun (0 fires first), if it is
    /// a card in the gun, and how many cards the gun holds.
    fn slot_of(&self, i: usize) -> (Option<usize>, usize) {
        let Some(shop) = &self.shop else { return (None, 0) };
        let on: Vec<usize> = shop.rows.iter().enumerate().filter(|(_, r)| r.area == ShopArea::Cards && r.on).map(|(k, _)| k).collect();
        (on.iter().position(|&k| k == i), on.len())
    }

    /// Where a card dragged from shop row `i` and let go at `p` goes: a
    /// slot moves it there (a bag card goes in at that slot), the bag
    /// takes a gun card out; anywhere else, or its own slot, nothing.
    fn drop_at(&self, i: usize, p: (f32, f32)) -> Option<StartEvent> {
        let (from, _) = self.slot_of(i);
        // Only where the page shows: a slot or the bag scrolled under the
        // title or the buttons takes no drop. (The slots keep their places
        // in the list, so `k` is the slot's own number: dropping only those
        // scrolled away from the list once moved a card to the wrong slot.)
        if self.body.is_some_and(|b| !b.contains(p)) {
            return None;
        }
        if let Some(k) = self.slot_rects.iter().position(|r| r.contains(p)) {
            return (from != Some(k)).then_some(StartEvent::Move(i, k));
        }
        (from.is_some() && self.bag.is_some_and(|b| b.contains(p))).then_some(StartEvent::Buy(i))
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
            Page::Main => [Hit::Run, Hit::Sandbox, Hit::Campaign].into_iter().chain(self.planets.then_some(Hit::RandomPlanet)).chain([Hit::Range, Hit::Options, Hit::Keys]).chain(self.edit_maps.then_some(Hit::EditMaps)).chain([Hit::Quit]).collect(),
            Page::Missions => (0..self.missions.len()).map(Hit::Mission).chain([Hit::Back]).collect(),
            Page::Options => self.tab_opts().into_iter().map(Hit::Reset).chain([Hit::ResetAll, Hit::Back]).collect(),
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
                Page::Shop if self.shop.as_ref().is_some_and(|s| s.range) => Hit::Range,
                Page::Shop => Hit::Run,
                Page::Main => Hit::Run,
            };
            self.focus = self.rows().iter().position(|&h| h == led).unwrap_or(0);
        }
    }

    /// What a hit does; a round picked comes back.
    fn act(&mut self, h: Hit) -> Option<StartEvent> {
        match h {
            Hit::Sandbox => return Some(StartEvent::Sandbox),
            Hit::Run => return Some(StartEvent::Run),
            Hit::Range => return Some(StartEvent::Range),
            Hit::Buy(i) => return Some(StartEvent::Buy(i)),
            Hit::Folder(k) => {
                self.folder = self.folders().get(k).cloned();
                self.focus = 0;
            }
            Hit::FolderBack => self.back(),
            Hit::Slot(_) => {}
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
            Hit::Turn(i, by) => {
                self.options[i].turn(by);
                // The row turned takes the focus, so the note beside the
                // list is the one for it.
                if let Some(k) = self.rows().iter().position(|&r| r == Hit::Reset(i)) {
                    self.focus = k;
                }
            }
            // On the keyboard, Enter on an option's row turns it on.
            Hit::Reset(i) => self.options[i].turn(1),
            // Every rule back, the UI size kept (a display choice, not a rule).
            Hit::ResetAll => {
                for o in self.options.iter_mut().filter(|o| o.key != "ui") {
                    o.at = o.default;
                }
            }
            Hit::Default(i) => self.options[i].at = self.options[i].default,
            Hit::Tab(k) => self.open_tab(k),
            Hit::Flip(by) => self.flip(by),
            Hit::Panel => {}
        }
        None
    }

    /// A paged list one page back or on (wrapping). Where the page follows
    /// the keyboard (the options, the missions), the focus goes to the new
    /// page's first row.
    fn flip(&mut self, by: isize) {
        let n = self.sheets.max(1) as isize;
        self.sheet = (self.sheet as isize + by).rem_euclid(n) as usize;
        if matches!(self.page, Page::Options | Page::Missions) {
            self.focus = self.sheet * self.per.max(1);
        }
    }

    /// Esc (or a right click) on the gun range's armory, no folder open:
    /// back to the range, not to the main page.
    fn leaves_armory(&self) -> bool {
        self.page == Page::Shop && self.folder.is_none() && self.shop.as_ref().is_some_and(|s| s.range)
    }

    pub fn moved(&mut self, p: Option<(f32, f32)>) {
        self.hover = p;
        // Held down on a bullet card and moved: a drag.
        if let (None, Some(Hit::Buy(i)), Some(a), Some(p)) = (self.drag, self.pressed, self.press_at, p) {
            let card = self.page == Page::Shop && self.shop.as_ref().is_some_and(|s| s.rows.get(i).is_some_and(|r| r.area == ShopArea::Cards));
            if card && (p.0 - a.0).hypot(p.1 - a.1) > DRAG_FROM {
                self.drag = Some(i);
            }
        }
    }

    fn hit(&self, p: (f32, f32)) -> Option<Hit> {
        // Last drawn is on top.
        self.hits.iter().rev().find(|(r, _)| r.contains(p)).map(|&(_, h)| h)
    }

    /// A mouse button: a click is a press and a release on the same thing.
    pub fn button(&mut self, b: Button, pressed: bool) -> Vec<StartEvent> {
        if b != Button::Left {
            if b == Button::Right && pressed && self.page != Page::Main {
                if self.leaves_armory() {
                    return vec![StartEvent::NextLevel];
                }
                self.back();
            }
            return Vec::new();
        }
        let hit = self.hover.and_then(|p| self.hit(p));
        if pressed {
            self.pressed = hit;
            self.press_at = self.hover;
            self.drag = None;
            return Vec::new();
        }
        let was = self.pressed.take();
        self.press_at = None;
        if let Some(i) = self.drag.take() {
            return self.hover.and_then(|p| self.drop_at(i, p)).into_iter().collect();
        }
        match hit {
            Some(h) if was == Some(h) => {
                // A row clicked takes the focus: its arrows turn it, its
                // "default" line resets it. (A click on the row reset it,
                // so one stray click took a 200% UI back to 100%: the
                // review, 11-15.)
                let h = match h {
                    Hit::Reset(i) => {
                        if let Some(k) = self.rows().iter().position(|&r| r == Hit::Reset(i)) {
                            self.focus = k;
                        }
                        return Vec::new();
                    }
                    Hit::Default(i) => {
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
        self.follow = true;
        match code {
            KeyCode::F1 | KeyCode::Slash => {
                if self.page == Page::Keys {
                    self.back();
                } else {
                    self.show(Page::Keys);
                }
            }
            KeyCode::Escape if self.leaves_armory() => return vec![StartEvent::NextLevel],
            KeyCode::Tab if self.tabbed() => {
                let at = if self.page == Page::Keys { self.tabs().len() - 1 } else { self.opt_tab };
                self.open_tab(at + 1);
            }
            KeyCode::PageUp | KeyCode::PageDown => self.flip(if code == KeyCode::PageUp { -1 } else { 1 }),
            KeyCode::ArrowLeft | KeyCode::ArrowRight | KeyCode::KeyA | KeyCode::KeyD if self.page == Page::Keys => {
                self.flip(if matches!(code, KeyCode::ArrowLeft | KeyCode::KeyA) { -1 } else { 1 });
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
            // A card in the gun one slot sooner (Q) or later (E); the
            // focus goes with it (the gun's cards are walked in slot
            // order, `shop_rows`).
            KeyCode::KeyQ | KeyCode::KeyE if self.page == Page::Shop => {
                if let Some(Hit::Buy(i)) = row {
                    if let (Some(k), n) = self.slot_of(i) {
                        let to = if code == KeyCode::KeyQ { k.checked_sub(1) } else { Some(k + 1).filter(|&t| t < n) };
                        if let Some(to) = to {
                            self.focus = self.focus + to - k;
                            return vec![StartEvent::Move(i, to)];
                        }
                    }
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
        // The UI size (`UiSize::px`) on every page, and a page taller than
        // the window scrolls (the wheel; the keyboard's focus is kept in
        // view) under its title and over its hint and buttons. Until 11-15
        // each page was held down to what fitted the window's height
        // (`sh / held`: the options page at 1080 lines drew at 0.85 whatever
        // the size, the key card too, and the shop and the gun range's
        // armory shrank to fit, to about 0.6), so the owner found Ctrl +
        // changed nothing there: "Changing the UI /text percentage increase
        // does do what it should the menus are unaffected. The gun range is
        // unaffected (I cannot read any of the text)" (11-15 07:56Z).
        let s = c.ui.px(c.screen);
        let s = (s * 20.0 + 1e-3).floor() / 20.0;
        self.hits.clear();
        // The inner pages (Options and its key list, the campaign) span the
        // screen under a one-line header, with no title: all three
        // researchers on the owner's "Having to scroll is horrible" (11-15
        // 17:31Z) found the title and its line repeated on every page took
        // 130 to 220 of the 565 units a 1080-line screen has at 200%
        // (`Reports/menu-layout-research-2026-11-15/`).
        let inner = matches!(self.page, Page::Options | Page::Keys | Page::Missions);
        // A shade down the left, fading out, so the panel reads over the
        // dish; the shop and the inner pages span the screen, so they dim
        // all of it.
        if self.page == Page::Shop || inner {
            c.rrect(Rect::new(0.0, 0.0, sw, sh), 0.0, [4, 6, 8, 150]);
        }
        let shade_w = (720.0 * s).min(sw);
        // (Forty thin steps: twelve showed as bands on the 1600-wide shot.)
        for k in 0..40 {
            let f = k as f32 / 40.0;
            c.rrect(Rect::new(0.0, 0.0, shade_w * (1.0 - f), sh), 0.0, [4, 6, 8, (9.0 * (1.0 - f)) as u8 + 2]);
        }
        let x = if inner { 40.0 * s } else { 48.0 * s };
        let w = (560.0 * s).min(sw - 2.0 * x);
        if !inner {
            c.text(Weight::SemiBold, 44.0 * s, (x, 96.0 * s), Align::Left, &self.title, TEXT);
            let sub = match self.page {
                Page::Shop if self.shop.as_ref().is_some_and(|s| s.range) => "The gun range: build a gun, then shoot",
                Page::Shop => "The run: spend your points, then land",
                _ => "A mould that grows, feeds and fights back",
            };
            c.text(Weight::Medium, 17.0 * s, (x, 127.0 * s), Align::Left, sub, MUTED);
        }
        let top = if inner { HEAD * s + 16.0 * s } else { 160.0 * s };
        let rows = self.rows();
        let focus = rows.get(self.focus.min(rows.len().saturating_sub(1))).copied();
        // The rows the page scrolls in: under the title, over the hint (and
        // the shop's two buttons).
        let foot = if self.page == Page::Shop { 108.0 * s } else { FOOT * s };
        let body = Rect::new(0.0, top - 4.0 * s, sw, (sh - foot - top + 4.0 * s).max(1.0));
        self.body = Some(body);
        self.scroll = self.scroll.clamp(0.0, self.scroll_max);
        let at = top - self.scroll;
        c.clip = Some((body.y, body.bottom()));
        let bottom = match self.page {
            Page::Main => self.main_page(c, x, at, w, s, focus),
            Page::Missions => self.missions_page(c, x, at, w, s, focus),
            Page::Options => self.options_page(c, x, at, w, s, focus),
            Page::Keys => self.keys_page(c, x, at, w, s),
            Page::Shop => self.shop_page(c, x, at, s, focus),
        };
        c.clip = None;
        self.scroll_max = (bottom + self.scroll + 12.0 * s - body.bottom()).max(0.0);
        // The keyboard's row into view (seen the next frame).
        if std::mem::take(&mut self.follow) {
            if let Some(r) = focus.and_then(|h| self.rect_of(h)) {
                self.scroll_to(r);
            }
        }
        // What scrolled out of the rows cannot be clicked; what is partly
        // in, only there.
        self.drawn = self.hits.clone();
        self.hits.retain_mut(|(r, _)| {
            let (y0, y1) = (r.y.max(body.y), r.bottom().min(body.bottom()));
            r.y = y0;
            r.h = y1 - y0;
            r.h > 0.0
        });
        // The header over the body, and a paged list's pager at the right
        // of the foot, never scrolled.
        let paged = inner && self.sheets > 1;
        if inner {
            let n = self.hits.len();
            self.header(c, x, s, focus);
            if paged {
                self.pager(c, sw - x, sh - foot + 7.0 * s, s);
            }
            self.drawn.extend_from_slice(&self.hits[n..]);
        }
        if self.scroll_max > 0.0 {
            // Where the page is: a thin bar down the right edge.
            let track = Rect::new(sw - 12.0 * s, body.y + 8.0 * s, 5.0 * s, body.h - 16.0 * s);
            c.rrect(track, 2.5 * s, [255, 255, 255, 18]);
            let total = body.h + self.scroll_max;
            let th = (track.h * body.h / total).max(30.0 * s);
            let ty = track.y + (track.h - th) * (self.scroll / self.scroll_max);
            c.rrect(Rect::new(track.x, ty, track.w, th), 2.5 * s, [255, 255, 255, 90]);
        }
        if self.page == Page::Shop {
            if let Some(shop) = &self.shop {
                // The buttons sit over the hint line, fixed.
                let by = sh - 96.0 * s;
                let (next, end) = if shop.range { ("Back to the range", "Leave the range") } else { ("Land on the next planet", "Give up the run") };
                self.item(Rect::new(x, by, 240.0 * s, 44.0 * s), next, "", Hit::Next, true, focus == Some(Hit::Next), c, s);
                self.item(Rect::new(x + 252.0 * s, by, 170.0 * s, 44.0 * s), end, "", Hit::EndRun, false, focus == Some(Hit::EndRun), c, s);
            }
        }
        let hint = match self.page {
            Page::Main => "Up/Down and Enter, or click  ·  F1 or ? keys",
            Page::Missions => "1-9 or Enter starts a mission  ·  Esc back",
            Page::Options => "Up/Down pick  ·  Left/Right change  ·  Backspace resets  ·  Tab next tab  ·  Esc back",
            Page::Keys if self.sheets > 1 && self.tabbed() => "Left/Right next page  ·  Tab next tab  ·  Esc or F1 back",
            Page::Keys if self.sheets > 1 => "Left/Right next page  ·  Esc or F1 back",
            Page::Keys if self.tabbed() => "Tab next tab  ·  Esc or F1 back",
            Page::Keys => "Esc or F1 back",
            Page::Shop if self.shop.as_ref().is_some_and(|s| s.range) => "Click takes a card  ·  drag cards (or Q and E) to set the gun's order  ·  Esc back to the range",
            Page::Shop => "Click or Enter buys  ·  drag cards (or Q and E) to set the gun's order  ·  Esc back",
        };
        let pager_w = if paged { 170.0 * s } else { 0.0 };
        c.text_fit(Weight::Medium, 13.5 * s, (x, sh - 28.0 * s), Align::Left, hint, sw - 2.0 * x - pager_w, MUTED);
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
        // (17 and 12.5 until the UI review, 11-15: "the text seems too
        // small".)
        if sub.is_empty() {
            c.text(Weight::SemiBold, 18.0 * s, (r.x + pad, r.y + r.h / 2.0 + 6.5 * s), Align::Left, label, TEXT);
        } else {
            c.text(Weight::SemiBold, 18.0 * s, (r.x + pad, r.y + r.h / 2.0 - 3.0 * s), Align::Left, label, TEXT);
            let col = if primary { [220, 236, 250, 255] } else { MUTED };
            c.text_fit(Weight::Medium, 14.0 * s, (r.x + pad, r.y + r.h / 2.0 + 17.0 * s), Align::Left, sub, r.w - 2.0 * pad, col);
        }
        self.hits.push((r, hit));
    }

    fn main_page(&mut self, c: &mut Canvas, x: f32, top: f32, w: f32, s: f32, focus: Option<Hit>) -> f32 {
        // The rules changed, by their labels; the UI size is not a rule
        // ("Free play, with ui 200%", the review 11-15).
        let changed: Vec<String> = self.options.iter().filter(|o| !o.is_default() && o.key != "ui").map(|o| format!("{} {}", o.label, o.value())).collect();
        let sandbox = if changed.is_empty() { "Free play on the map, with today's defaults".to_string() } else { format!("Free play, with {}", changed.join(", ")) };
        let campaign = format!("{} missions, one new verb each", self.missions.len());
        // The run first, and the one lit (the whole-game review, 11-15:
        // the run is the game; Sandbox led and was lit before).
        let mut items: Vec<(&str, String, Hit, bool)> = vec![
            (if self.run_live { "Back to the run" } else { "Run" }, if self.run_live { "The shop, then the next planet".into() } else { "Planet after planet toward the slime's home world; upgrades between".into() }, Hit::Run, true),
            ("Sandbox", sandbox, Hit::Sandbox, false),
            ("Campaign", campaign, Hit::Campaign, false),
        ];
        if self.planets {
            let depth = self.option("planet").map_or("1", |o| o.value());
            items.push(("Random planet", format!("A new generated planet, planet {depth} of a run (Options: Planet)"), Hit::RandomPlanet, false));
        }
        items.extend([
            ("Gun range", "Every gun, card and bullet, free: build a gun and shoot the slime".into(), Hit::Range, false),
            ("Options", "The mould's and the squad's switches".into(), Hit::Options, false),
            ("Keys", "F1 or ? in the game too".into(), Hit::Keys, false),
        ]);
        if self.edit_maps {
            items.push(("Edit maps", "Draw a map in the map editor".into(), Hit::EditMaps, false));
        }
        items.push(("Quit", String::new(), Hit::Quit, false));
        let mut y = top + 12.0 * s;
        for (label, sub, hit, primary) in items {
            let h = if sub.is_empty() { 50.0 * s } else { 66.0 * s };
            self.item(Rect::new(x, y, w.min(500.0 * s), h), label, &sub, hit, primary, focus == Some(hit), c, s);
            y += h + 12.0 * s;
        }
        if let Some(n) = self.notice.clone() {
            c.text_fit(Weight::Medium, 14.0 * s, (x, y + 14.0 * s), Align::Left, &n, w, WARN);
            y += 24.0 * s;
        }
        y
    }

    /// The inner pages' header: the page's name, the Options tabs (the key
    /// list is the last), and Back at the right.
    fn header(&mut self, c: &mut Canvas, x: f32, s: f32, focus: Option<Hit>) {
        let sw = c.screen.0 as f32;
        let h = HEAD * s;
        c.rrect(Rect::new(0.0, 0.0, sw, h), 0.0, [8, 11, 14, 236]);
        c.rrect(Rect::new(0.0, h - s.max(1.0), sw, s.max(1.0)), 0.0, EDGE);
        let name = match self.page {
            Page::Missions => "Campaign",
            _ if self.tabbed() => "Options",
            _ => "Keys",
        };
        let tx = x + c.text(Weight::SemiBold, 22.0 * s, (x, 36.0 * s), Align::Left, name, TEXT) + 24.0 * s;
        let back = Rect::new(sw - x - 116.0 * s, 10.0 * s, 116.0 * s, 36.0 * s);
        if self.tabbed() {
            let lit = if self.page == Page::Keys { self.tabs().len() - 1 } else { self.opt_tab };
            let mut tx = tx;
            for (k, t) in self.tabs().iter().enumerate() {
                let w = c.atlas.measure(Weight::SemiBold, 15.0 * s, t) + 28.0 * s;
                let r = Rect::new(tx, 11.0 * s, w, 34.0 * s);
                if r.right() > back.x - 12.0 * s {
                    break;
                }
                let on = k == lit;
                let hot = self.hover.is_some_and(|p| r.contains(p));
                let (top, bottom) = match (on, hot) {
                    (true, _) => ([40, 70, 98, 255], [30, 54, 78, 255]),
                    (false, true) => (BUTTON_HOVER_TOP, BUTTON_HOVER_BOTTOM),
                    (false, false) => (BUTTON_TOP, BUTTON_BOTTOM),
                };
                c.panel(r, 8.0 * s, top, bottom, 1.0, if on { ACCENT } else { [255, 255, 255, 30] });
                c.text(Weight::SemiBold, 15.0 * s, (r.x + 14.0 * s, r.y + 22.5 * s), Align::Left, t, if on { TEXT } else { MUTED });
                self.hits.push((r, Hit::Tab(k)));
                tx = r.right() + 8.0 * s;
            }
        }
        self.small_button(back, "Back", "Esc", Hit::Back, focus == Some(Hit::Back), c, s);
    }

    /// A one-line button with its key small at the right.
    #[allow(clippy::too_many_arguments)]
    fn small_button(&mut self, r: Rect, label: &str, key: &str, hit: Hit, focused: bool, c: &mut Canvas, s: f32) {
        let hot = focused || self.hover.is_some_and(|p| r.contains(p));
        let (t, b) = if hot { (BUTTON_HOVER_TOP, BUTTON_HOVER_BOTTOM) } else { (BUTTON_TOP, BUTTON_BOTTOM) };
        c.panel(r, 8.0 * s, t, b, 1.0, [255, 255, 255, 34]);
        if focused {
            c.outline(r.inset(-2.0 * s), 10.0 * s, 1.5 * s, ACCENT);
        }
        c.text(Weight::SemiBold, 16.0 * s, (r.x + 14.0 * s, r.y + r.h / 2.0 + 6.0 * s), Align::Left, label, TEXT);
        if !key.is_empty() {
            c.text(Weight::Medium, 12.5 * s, (r.right() - 10.0 * s, r.y + r.h / 2.0 + 5.0 * s), Align::Right, key, MUTED);
        }
        self.hits.push((r, hit));
    }

    /// "<  2 of 3  >" ending at `right`, from `y` down, for a paged list
    /// (in the foot, right of the hint).
    fn pager(&mut self, c: &mut Canvas, right: f32, y: f32, s: f32) {
        if self.sheets <= 1 {
            return;
        }
        let bw = 34.0 * s;
        let next = Rect::new(right - bw, y, bw, 30.0 * s);
        let label = format!("{} of {}", self.sheet + 1, self.sheets);
        let lw = c.atlas.measure(Weight::SemiBold, 14.5 * s, &label) + 20.0 * s;
        let prev = Rect::new(next.x - lw - bw, y, bw, 30.0 * s);
        c.text(Weight::SemiBold, 14.5 * s, (prev.right() + lw / 2.0, y + 20.0 * s), Align::Centre, &label, TEXT);
        for (r, by, t) in [(prev, -1isize, "<"), (next, 1, ">")] {
            let hot = self.hover.is_some_and(|p| r.contains(p));
            c.panel(r, 7.0 * s, if hot { BUTTON_HOVER_TOP } else { BUTTON_TOP }, if hot { BUTTON_HOVER_BOTTOM } else { BUTTON_BOTTOM }, 1.0, [255, 255, 255, 30]);
            c.text(Weight::SemiBold, 15.0 * s, (r.x + r.w / 2.0, r.y + 20.0 * s), Align::Centre, t, TEXT);
            self.hits.push((r, Hit::Flip(by)));
        }
    }

    /// The list and the note beside it: the list's width and the note's,
    /// on an inner page `x` from each side.
    fn split(&self, c: &Canvas, x: f32, s: f32) -> (f32, f32) {
        let avail = c.screen.0 as f32 - 2.0 * x;
        let note = (avail * 0.42).clamp(300.0 * s, 480.0 * s).min(avail * 0.5);
        let list = (avail - note - 20.0 * s).min(680.0 * s);
        (list, note)
    }

    /// The room an inner page's body has under its header, over its hint.
    fn room(&self, c: &Canvas, top: f32, s: f32) -> f32 {
        // (The 12 units the page keeps over its hint: `draw`'s scroll_max.)
        c.screen.1 as f32 - FOOT * s - 12.0 * s - top
    }

    /// The missions as one-line rows, a page of them if they do not fit,
    /// with the one under the cursor (else the keyboard's) told in full in
    /// a note beside them. (Two-line rows of 78 units until 11-15: eight
    /// needed 624 units, more than a 1080-line screen has at 200%.)
    fn missions_page(&mut self, c: &mut Canvas, x: f32, top: f32, _w: f32, s: f32, focus: Option<Hit>) -> f32 {
        let n = self.missions.len();
        let (lw, nw) = self.split(c, x, s);
        let rh = 46.0 * s;
        let room = self.room(c, top, s);
        let per = ((room - 16.0 * s) / rh).floor().max(1.0) as usize;
        if let Some(k) = focus.and_then(|h| if let Hit::Mission(i) = h { Some(i) } else { None }) {
            self.sheet = k / per;
        }
        let (shown, pages) = sheet_of(n, per, self.sheet);
        (self.sheets, self.per) = (pages, per);
        self.sheet = self.sheet.min(pages - 1);
        let panel = Rect::new(x, top, lw, shown.len().max(1) as f32 * rh + 16.0 * s);
        self.glass(panel, c, s);
        if n == 0 {
            c.text(Weight::Medium, 16.0 * s, (x + 20.0 * s, top + 34.0 * s), Align::Left, "No missions found", MUTED);
        }
        let mut told = None;
        let mut y = panel.y + 8.0 * s;
        for i in shown {
            let e = self.missions[i].clone();
            let row = Rect::new(panel.x + 8.0 * s, y, panel.w - 16.0 * s, rh - 4.0 * s);
            let focused = focus == Some(Hit::Mission(i));
            let hot = self.hover.is_some_and(|p| row.contains(p));
            if focused || hot {
                c.rrect(row, 9.0 * s, [255, 255, 255, if focused { 22 } else { 12 }]);
            }
            if focused {
                c.outline(row, 9.0 * s, 1.5 * s, ACCENT);
            }
            if hot || (focused && told.is_none()) {
                told = Some(i);
            }
            let base = row.y + row.h / 2.0 + 6.0 * s;
            let dw = c.atlas.measure(Weight::Medium, 14.0 * s, &e.detail).min(row.w * 0.4);
            c.text_fit(Weight::SemiBold, 17.0 * s, (row.x + 14.0 * s, base), Align::Left, &format!("{}. {}", i + 1, e.name), row.w - dw - 44.0 * s, TEXT);
            c.text_fit(Weight::Medium, 14.0 * s, (row.right() - 14.0 * s, base), Align::Right, &e.detail, row.w * 0.4, MUTED);
            self.hits.push((row, Hit::Mission(i)));
            y += rh;
        }
        // The note: the mission under the cursor, else the keyboard's, else
        // the first.
        if let Some(e) = told.or((n > 0).then_some(0)).map(|i| (i, self.missions[i].clone())) {
            let (i, e) = e;
            let tw = nw - 36.0 * s;
            let lines = Self::count(c, Weight::SemiBold, 21.0 * s, tw, &format!("{}. {}", i + 1, e.name)) * 28.0 + Self::count(c, Weight::Medium, 15.5 * s, tw, &e.says) * 23.0 + Self::count(c, Weight::Medium, 14.0 * s, tw, &e.detail) * 21.0;
            let need = (36.0 + 10.0 + 48.0) * s + lines * s;
            let r = Rect::new(x + lw + 20.0 * s, top, nw, panel.h.max(220.0 * s).max(need).min(room));
            self.glass(r, c, s);
            let px = r.x + 18.0 * s;
            let mut y = r.y + 36.0 * s;
            y += self.lines(c, Weight::SemiBold, 21.0 * s, (px, y), tw, 28.0 * s, &format!("{}. {}", i + 1, e.name), TEXT) + 6.0 * s;
            y += self.lines(c, Weight::Medium, 15.5 * s, (px, y), tw, 23.0 * s, &e.says, TEXT) + 4.0 * s;
            self.lines(c, Weight::Medium, 14.0 * s, (px, y), tw, 21.0 * s, &e.detail, MUTED);
            c.text(Weight::Medium, 13.5 * s, (px, r.bottom() - 18.0 * s), Align::Left, &format!("Click or {} starts it", i + 1), MUTED);
        }
        panel.bottom()
    }

    /// The lines `text` wraps to in `max` (none when empty).
    fn count(c: &Canvas, weight: Weight, px: f32, max: f32, text: &str) -> f32 {
        if text.is_empty() {
            0.0
        } else {
            c.wrap(weight, px, max, text).len() as f32
        }
    }

    /// Wrapped text from a first baseline at `at`; the height it took
    /// (to the next first baseline).
    #[allow(clippy::too_many_arguments)]
    fn lines(&self, c: &mut Canvas, weight: Weight, px: f32, at: (f32, f32), max: f32, lead: f32, text: &str, colour: Rgba) -> f32 {
        if text.is_empty() {
            return 0.0;
        }
        c.paragraph(weight, px, at, max, lead, text, colour) as f32 * lead
    }

    /// The Options page: the tab's settings as one-line rows (a page of
    /// them if a tab outgrows the screen), and beside them the focused
    /// one's whole note, its default and its reset. Until 11-15 every
    /// setting was a two-line row of 62 units with its note cut to fit,
    /// 17 of them, so the page scrolled even at 100% on 1080 lines; three
    /// researchers on the owner's "Having to scroll is horrible" (17:31Z)
    /// agreed on tabs, single-line rows and a fixed note pane
    /// (`Reports/menu-layout-research-2026-11-15/`).
    fn options_page(&mut self, c: &mut Canvas, x: f32, top: f32, _w: f32, s: f32, focus: Option<Hit>) -> f32 {
        let opts = self.tab_opts();
        let n = opts.len();
        let (lw, nw) = self.split(c, x, s);
        let rh = 46.0 * s;
        let room = self.room(c, top, s);
        let per = ((room - 16.0 * s) / rh).floor().max(1.0) as usize;
        // The page follows the keyboard's row.
        if let Some(k) = focus.and_then(|h| if let Hit::Reset(i) = h { opts.iter().position(|&o| o == i) } else { None }) {
            self.sheet = k / per;
        }
        let (shown, pages) = sheet_of(n, per, self.sheet);
        (self.sheets, self.per) = (pages, per);
        self.sheet = self.sheet.min(pages - 1);
        let panel = Rect::new(x, top, lw, shown.len().max(1) as f32 * rh + 16.0 * s);
        self.glass(panel, c, s);
        let mut told = None;
        let mut y = panel.y + 8.0 * s;
        for &i in &opts[shown] {
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
            if hot || (focused && told.is_none()) {
                told = Some(i);
            }
            self.hits.push((row, Hit::Reset(i)));
            // The value between its arrows.
            let aw = 30.0 * s;
            let vw = 130.0 * s;
            let next = Rect::new(row.right() - 8.0 * s - aw, row.y + (row.h - 30.0 * s) / 2.0, aw, 30.0 * s);
            let prev = Rect::new(next.x - vw - aw, next.y, aw, 30.0 * s);
            let base = row.y + row.h / 2.0 + 6.0 * s;
            let lx = row.x + 14.0 * s;
            let label_w = c.text_fit(Weight::SemiBold, 17.0 * s, (lx, base), Align::Left, &o.label, prev.x - lx - 30.0 * s, TEXT);
            if !o.is_default() {
                // Moved off its default: a dot after the name (the note
                // says what it was).
                c.rrect(Rect::new(lx + label_w + 8.0 * s, base - 9.0 * s, 7.0 * s, 7.0 * s), 3.5 * s, WARN);
            }
            for (r, by, label) in [(prev, -1isize, "<"), (next, 1, ">")] {
                let hot = self.hover.is_some_and(|p| r.contains(p));
                c.panel(r, 7.0 * s, if hot { BUTTON_HOVER_TOP } else { BUTTON_TOP }, if hot { BUTTON_HOVER_BOTTOM } else { BUTTON_BOTTOM }, 1.0, [255, 255, 255, 30]);
                c.text(Weight::SemiBold, 15.0 * s, (r.x + r.w / 2.0, r.y + 20.0 * s), Align::Centre, label, TEXT);
                self.hits.push((r, Hit::Turn(i, by)));
            }
            let vc = if o.is_default() { TEXT } else { WARN };
            c.text_fit(Weight::SemiBold, 16.0 * s, (prev.right() + vw / 2.0, base), Align::Centre, o.value(), vw - 8.0 * s, vc);
            y += rh;
        }
        // The note: the row under the cursor, else the keyboard's, else
        // the tab's first.
        let tw = nw - 36.0 * s;
        let told = told.or(opts.first().copied()).map(|i| (i, self.options[i].clone()));
        let state = |o: &Opt| {
            if o.is_default() {
                format!("{} is the default", o.value())
            } else {
                format!("Now {}  ·  the default is {}", o.value(), o.choices[o.default])
            }
        };
        // Every rule back (the UI size kept), and where the defaults come
        // from (D5: "applies to this build"), at the note's foot.
        let foot = "Defaults are this build's; they move as the game is tuned";
        let fl = Self::count(c, Weight::Medium, 13.0 * s, tw, foot);
        let tail = 8.0 * s + fl * 19.0 * s + 66.0 * s;
        // As tall as the list, or as the note needs.
        let need = told.as_ref().map_or(0.0, |(_, o)| {
            let lines = Self::count(c, Weight::SemiBold, 21.0 * s, tw, &o.label) * 28.0 + Self::count(c, Weight::Medium, 15.5 * s, tw, &o.note) * 23.0 + Self::count(c, Weight::Medium, 14.0 * s, tw, &state(o)) * 21.0;
            (48.0 + if o.is_default() { -10.0 } else { 30.0 }) * s + lines * s
        });
        let r = Rect::new(x + lw + 20.0 * s, top, nw, panel.h.max(300.0 * s).max(need + tail).min(room));
        self.glass(r, c, s);
        let px = r.x + 18.0 * s;
        let all = Rect::new(px, r.bottom() - 54.0 * s, (190.0 * s).min(tw), 40.0 * s);
        if let Some((i, o)) = told {
            let mut y = r.y + 36.0 * s;
            y += self.lines(c, Weight::SemiBold, 21.0 * s, (px, y), tw, 28.0 * s, &o.label, TEXT) + 6.0 * s;
            y += self.lines(c, Weight::Medium, 15.5 * s, (px, y), tw, 23.0 * s, &o.note, TEXT) + 6.0 * s;
            y += self.lines(c, Weight::Medium, 14.0 * s, (px, y), tw, 21.0 * s, &state(&o), MUTED);
            if !o.is_default() {
                let label = format!("Back to {}", o.choices[o.default]);
                let bw = (c.atlas.measure(Weight::SemiBold, 16.0 * s, &label) + 70.0 * s).min(tw);
                let b = Rect::new(px, y - 6.0 * s, bw, 36.0 * s);
                if b.bottom() <= all.y - 8.0 * s - fl * 19.0 * s {
                    self.small_button(b, &label, "Bksp", Hit::Default(i), false, c, s);
                }
            }
        }
        c.paragraph(Weight::Medium, 13.0 * s, (px, all.y - 12.0 * s - (fl - 1.0) * 19.0 * s), tw, 19.0 * s, foot, MUTED);
        self.small_button(all, "All to defaults", "", Hit::ResetAll, focus == Some(Hit::ResetAll), c, s);
        panel.bottom().max(r.bottom())
    }

    /// The key list, the Options page's last tab (also F1 or ?): two
    /// columns where there is room, each line wrapped, a page at a time
    /// (Left/Right or the arrows under it). It scrolled until 11-15.
    fn keys_page(&mut self, c: &mut Canvas, x: f32, top: f32, _w: f32, s: f32) -> f32 {
        let rows = self.keys.clone();
        let avail = c.screen.0 as f32 - 2.0 * x;
        let room = self.room(c, top, s);
        let gap = 28.0 * s;
        // Two columns down to 788 units across (a 1280-wide screen at
        // 200% is 984 units: at 440 a column it drew one, four pages).
        let cols = if avail >= 2.0 * 380.0 * s + gap { 2 } else { 1 };
        let col_w = (avail - (cols - 1) as f32 * gap) / cols as f32;
        // The key's column: most keys are short, and a narrower one wraps
        // fewer of the longer descriptions (0.36 and 210 drew three pages
        // at 200% on 1080 lines, two now).
        let kw = (col_w * 0.3).min(180.0 * s);
        let (px, lead) = (15.0 * s, 21.0 * s);
        // Each row's height: its longer side, wrapped.
        let heights: Vec<f32> = rows
            .iter()
            .map(|(k, w)| {
                let a = c.wrap(Weight::SemiBold, px, kw - 12.0 * s, k).len();
                let b = c.wrap(Weight::Medium, px, col_w - kw - 24.0 * s, w).len();
                a.max(b) as f32 * lead + 5.0 * s
            })
            .collect();
        // Fill pages: column by column, each as tall as the room.
        let col_h = room - 36.0 * s;
        let mut sheets: Vec<Vec<(usize, usize, f32)>> = vec![Vec::new()];
        let (mut col, mut y) = (0usize, 0.0f32);
        for (i, h) in heights.iter().enumerate() {
            if y + h > col_h && y > 0.0 {
                col += 1;
                y = 0.0;
                if col == cols {
                    sheets.push(Vec::new());
                    col = 0;
                }
            }
            sheets.last_mut().unwrap().push((i, col, y));
            y += h;
        }
        self.sheets = sheets.len();
        self.sheet = self.sheet.min(self.sheets - 1);
        let shown = &sheets[self.sheet];
        let used = shown.iter().map(|&(i, _, y)| y + heights[i]).fold(0.0f32, f32::max);
        let panel = Rect::new(x, top, avail, used + 36.0 * s);
        self.glass(panel, c, s);
        for &(i, col, y) in shown {
            let (k, w) = &rows[i];
            let cx = panel.x + 20.0 * s + col as f32 * (col_w + gap);
            let by = panel.y + 28.0 * s + y;
            c.paragraph(Weight::SemiBold, px, (cx, by), kw - 12.0 * s, lead, k, ACCENT);
            c.paragraph(Weight::Medium, px, (cx + kw, by), col_w - kw - 24.0 * s, lead, w, TEXT);
        }
        panel.bottom()
    }

    /// The shop page laid out at scale `s` from `top`: the upgrade grid on
    /// the left (six tiles a row, as the command card is six by two, with a
    /// box under it saying what the tile under the cursor does), the
    /// armory on the right (gun cards, the carried gun's slots, the bullet
    /// cards by heading). The grid keeps the height of its tallest page so
    /// opening a folder does not move the rest.
    fn shop_plan(&self, shop: &Shop, screen: (f32, f32), x: f32, top: f32, s: f32, foot: f32) -> ShopPlan {
        let (sw, sh) = screen;
        // Sizes from the UI review (11-15, "the text seems too small ...
        // not always making the best use of space"): tiles 60 -> 78 with
        // their names at 12.5, not 9.5 (which the fit shrank to 7.5); the
        // bullet cards 78 x 98 portrait -> 184 x 64 landscape, as the
        // armory had width to spare and ran out of height (the page shrank
        // to fit, so taller cards only made every word smaller).
        let (t, g, p, head) = (78.0 * s, 7.0 * s, 14.0 * s, 30.0 * s);
        let cols = 6usize;
        let left_w = 2.0 * p + cols as f32 * t + (cols - 1) as f32 * g;
        let pages = std::iter::once(shop.rows.iter().filter(|r| r.area == ShopArea::Upgrades).count() + self.folders().len())
            .chain(self.folders().into_iter().map(|f| 1 + shop.rows.iter().filter(|r| r.area == ShopArea::Folder && r.group == f).count()));
        let grid_rows = pages.max().unwrap_or(1).div_ceil(cols).max(1);
        let grid_h = grid_rows as f32 * t + (grid_rows - 1) as f32 * g;
        let detail_h = 136.0 * s;
        let left = Rect::new(x, top, left_w, head + 8.0 * s + grid_h + 12.0 * s + detail_h + p);
        let mut tiles = Vec::new();
        for (k, h) in self.grid().into_iter().enumerate() {
            let (row, col) = (k / cols, k % cols);
            tiles.push((Rect::new(left.x + p + col as f32 * (t + g), left.y + head + 8.0 * s + row as f32 * (t + g), t, t), h));
        }
        let detail = Rect::new(left.x + p, left.y + head + 8.0 * s + grid_h + 12.0 * s, left_w - 2.0 * p, detail_h);
        // The armory: what is left of the width.
        // Under the upgrades, the screen's width, where beside them it would
        // hold one gun card a row (a big UI size: 200% on 1920 x 1080 left
        // it a column 777 pixels wide, every card under the last).
        let (gw, gh) = (262.0 * s, 166.0 * s);
        let stacked = sw - x - (left.right() + 16.0 * s) < 2.0 * gw + g + 2.0 * p;
        let (rx, rtop) = if stacked { (x, left.bottom() + 16.0 * s) } else { (left.right() + 16.0 * s, top) };
        let rw = sw - x - rx;
        let (cw, ch) = (CARD_W * s, CARD_H * s);
        let mut heads = Vec::new();
        let (x0, x1) = (rx + p, rx + rw - p);
        let mut y = rtop + head + 8.0 * s;
        heads.push(((x0, rtop + 24.0 * s), "Guns".to_string()));
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
        heads.push(((x0, y + 19.0 * s), format!("In the gun, fired left to right  ·  budget {} of {}  ·  {} of {} slots", shop.budget.0, shop.budget.1, on.len(), shop.slots)));
        y += head;
        let (rects, used) = flow(n_slots, cw, ch, y);
        let slots: Vec<(Rect, Option<usize>)> = rects.into_iter().enumerate().map(|(k, r)| (r, on.get(k).copied())).collect();
        y += used + 14.0 * s;
        let mut cards = Vec::new();
        let bag_top = y;
        for (name, idx) in self.card_groups() {
            heads.push(((x0, y + 19.0 * s), name));
            y += head;
            let (rects, used) = flow(idx.len(), cw, ch, y);
            cards.extend(rects.into_iter().zip(idx));
            y += used + 14.0 * s;
        }
        // Room for two card groups (for sale, in the bag) of a row each,
        // there or not: sized to its contents, the page shrank the moment
        // the first card went into the bag and grew back when it left
        // (owner, 11-15 02:58Z: "everything shifts in the shop. Once you
        // add a bag that shouldn't happen"). The scale now changes only
        // when a group needs a second row.
        let groups = self.card_groups().len();
        y += 2usize.saturating_sub(groups) as f32 * (head + ch + 14.0 * s);
        let right = Rect::new(rx, rtop, rw, if stacked { y - rtop } else { (y - rtop).max(left.h) });
        let bag = Rect::new(rx, bag_top, rw, (right.bottom() - bag_top).max(0.0));
        let room = sh - foot - top;
        let fits_w = rw >= gw + 2.0 * p && left.right() <= sw;
        let fits = left.h <= room && right.bottom() - top <= room && fits_w;
        ShopPlan { s, left, right, tiles, guns, slots, cards, bag, heads, detail, fits, fits_w }
    }

    fn shop_page(&mut self, c: &mut Canvas, x: f32, top: f32, s: f32, focus: Option<Hit>) -> f32 {
        let (sw, sh) = (c.screen.0 as f32, c.screen.1 as f32);
        let Some(shop) = self.shop.clone() else { return top };
        c.text_fit(Weight::SemiBold, 20.0 * s, (x, top + 15.0 * s), Align::Left, &shop.heading, sw - 2.0 * x - 200.0 * s, TEXT);
        let mut y = top + 38.0 * s;
        for l in &shop.lines {
            c.text_fit(Weight::Medium, 14.5 * s, (x, y), Align::Left, l, sw - 2.0 * x, MUTED);
            y += 21.0 * s;
        }
        c.text(Weight::SemiBold, 22.0 * s, (sw - x, top + 16.0 * s), Align::Right, &shop.purse, ACCENT);
        // The layout starts a little over the UI's own size and shrinks
        // until both areas fit above the buttons, but not under the UI's
        // size: past that the page scrolls (it went down to whatever fitted
        // until 11-15, about 0.6 for the gun range's armory at 1080 lines:
        // "I cannot read any of the text"). Only the width shrinks it
        // further.
        let body = y + 2.0 * s;
        let foot_s = s;
        // (Laid out from the unscrolled top, so the scale does not change
        // as the page scrolls.)
        let shift = self.scroll;
        let mut k = s * 1.3;
        let plan = loop {
            let plan = self.shop_plan(&shop, (sw, sh), x, body + shift, k, 108.0 * foot_s);
            if plan.fits || (plan.fits_w && k <= s) || k < 0.3 {
                break plan;
            }
            k = if plan.fits_w { (k * 0.92).max(s) } else { k * 0.92 };
        };
        let plan = self.shop_plan(&shop, (sw, sh), x, body, plan.s, 108.0 * foot_s);
        let s = plan.s;
        self.glass(plan.left, c, s);
        self.glass(plan.right, c, s);
        let title = match &self.folder {
            Some(f) => format!("Upgrades  ›  {f}"),
            None if shop.range => "The range".to_string(),
            None => "Upgrades".to_string(),
        };
        c.text(Weight::SemiBold, 14.5 * s, (plan.left.x + 14.0 * s, plan.left.y + 24.0 * s), Align::Left, &title, MUTED);
        for (at, h) in &plan.heads {
            c.text_fit(Weight::SemiBold, 14.5 * s, *at, Align::Left, h, plan.right.w - 28.0 * s, MUTED);
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
        self.slot_rects = plan.slots.iter().map(|x| x.0).collect();
        self.bag = Some(plan.bag);
        let drag = self.drag.filter(|&i| i < shop.rows.len());
        // The slot a drag would land in, and the card's own place.
        let target = drag.and_then(|i| self.hover.and_then(|p| self.drop_at(i, p)));
        let mut from = None;
        for (k, &(r, i)) in plan.slots.iter().enumerate() {
            match i {
                Some(i) => {
                    if hot(r) {
                        shown = Some(Hit::Buy(i));
                    }
                    self.bullet_card(&shop.rows[i], r, hot(r) && drag.is_none(), focus == Some(Hit::Buy(i)), c, s);
                    self.hits.push((r, Hit::Buy(i)));
                    if drag == Some(i) {
                        from = Some(r);
                    }
                }
                None => {
                    if hot(r) {
                        shown = Some(Hit::Slot(k));
                    }
                    c.rrect(r, 9.0 * s, [255, 255, 255, 6]);
                    c.outline(r, 9.0 * s, 1.0, [255, 255, 255, 30]);
                    c.text(Weight::Medium, 13.0 * s, (r.x + r.w / 2.0, r.y + r.h / 2.0 + 4.5 * s), Align::Centre, "empty", FAINT);
                    self.hits.push((r, Hit::Slot(k)));
                }
            }
            // Its place in the firing order.
            c.text(Weight::SemiBold, 12.0 * s, (r.x + 8.0 * s, r.y + 19.0 * s), Align::Left, &format!("{}", k + 1), with_alpha(TEXT, 0.6));
        }
        for &(r, i) in &plan.cards {
            if hot(r) {
                shown = Some(Hit::Buy(i));
            }
            self.bullet_card(&shop.rows[i], r, hot(r) && drag.is_none(), focus == Some(Hit::Buy(i)), c, s);
            self.hits.push((r, Hit::Buy(i)));
            if drag == Some(i) {
                from = Some(r);
            }
        }
        if let (Some(i), Some(p)) = (drag, self.hover) {
            // The card left behind shows as a hole; where it would land is
            // ringed; the card itself rides under the pointer.
            if let Some(r) = from {
                c.rrect(r, 9.0 * s, [4, 6, 8, 170]);
                c.outline(r, 9.0 * s, 1.0, [255, 255, 255, 40]);
            }
            match target {
                Some(StartEvent::Move(_, k)) => {
                    let r = self.slot_rects[k];
                    c.outline(r.inset(-3.0 * s), 12.0 * s, 2.5 * s, ACCENT);
                    c.rrect(Rect::new(r.x - 5.0 * s, r.y + 6.0 * s, 3.0 * s, r.h - 12.0 * s), 1.5 * s, ACCENT);
                }
                Some(_) => c.outline(plan.bag.inset(4.0 * s), 12.0 * s, 2.0 * s, with_alpha(ACCENT, 0.7)),
                None => {}
            }
            let (w, h) = plan.slots.first().map_or((CARD_W * s, CARD_H * s), |x| (x.0.w, x.0.h));
            let r = Rect::new(p.0 - w / 2.0, p.1 - h * 0.4, w, h);
            // (Over everything, not kept to the scrolling rows.)
            let clip = c.clip.take();
            c.rrect(Rect::new(r.x + 4.0 * s, r.y + 6.0 * s, w, h), 10.0 * s, [0, 0, 0, 110]);
            self.bullet_card(&shop.rows[i], r, true, false, c, s);
            c.clip = clip;
            shown = Some(Hit::Buy(i));
        }
        self.detail(&shop, shown, &folders, plan.detail, c, s);
        // (The buttons are `draw`'s, fixed under the scrolling page.)
        let _ = foot_s;
        plan.left.bottom().max(plan.right.bottom())
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
        let dim: f32 = if row.ready || owned { 1.0 } else { 0.42 };
        // A long name ("More cards on the gun") goes on two lines under a
        // smaller icon rather than shrinking to nothing.
        let ink = with_alpha(TEXT, dim.max(0.6));
        let room = r.w - 8.0 * s;
        let lines = if c.atlas.measure(Weight::SemiBold, 11.5 * s, &row.name) > room - 2.0 * s { c.wrap(Weight::SemiBold, 11.5 * s, room, &row.name) } else { vec![row.name.clone()] };
        let ic = if lines.len() > 1 { 22.0 * s } else { 28.0 * s };
        c.icon(row.icon, (r.x + (r.w - ic) / 2.0, r.y + 8.0 * s), ic, with_alpha(if owned { GOOD } else { row.tint }, dim));
        if lines.len() > 1 {
            c.text_fit(Weight::SemiBold, 11.5 * s, (r.x + r.w / 2.0, r.y + 44.0 * s), Align::Centre, &lines[0], room, ink);
            c.text_fit(Weight::SemiBold, 11.5 * s, (r.x + r.w / 2.0, r.y + 57.0 * s), Align::Centre, &lines[1..].join(" "), room, ink);
        } else {
            let px = fit_px(c, &row.name, 12.5 * s, 11.5 * s, room - 2.0 * s);
            c.text_fit(Weight::SemiBold, px, (r.x + r.w / 2.0, r.y + 54.0 * s), Align::Centre, &row.name, room, ink);
        }
        if row.level.1 > 1 {
            let d = 5.0 * s;
            for k in 0..row.level.1 {
                let at = Rect::new(r.x + 6.0 * s + k as f32 * (d + 2.5 * s), r.bottom() - 11.0 * s, d, d);
                c.rrect(at, d / 2.0, if k < row.level.0 { ACCENT } else { [255, 255, 255, 40] });
            }
        }
        if owned {
            c.icon(Icon::Check, (r.right() - 17.0 * s, r.y + 5.0 * s), 13.0 * s, GOOD);
        } else if !row.price.is_empty() {
            c.text(Weight::SemiBold, 12.0 * s, (r.right() - 5.0 * s, r.bottom() - 5.0 * s), Align::Right, &row.price, with_alpha(if row.ready { ENERGY } else { BAD }, 0.95));
        }
    }

    /// A folder on the grid (or its way back): its icon, its name and a
    /// dot when something in it can be bought now.
    #[allow(clippy::too_many_arguments)]
    fn folder_tile(&self, name: &str, icon: Icon, any: bool, r: Rect, hot: bool, focused: bool, c: &mut Canvas, s: f32) {
        self.face(r, true, false, hot, focused, c, s);
        c.outline(r.inset(3.0 * s), 7.0 * s, 1.0, [96, 190, 255, 60]);
        let ic = 28.0 * s;
        c.icon(icon, (r.x + (r.w - ic) / 2.0, r.y + 9.0 * s), ic, ACCENT);
        c.text_fit(Weight::SemiBold, 12.5 * s, (r.x + r.w / 2.0, r.y + 54.0 * s), Align::Centre, name, r.w - 6.0 * s, TEXT);
        c.text(Weight::SemiBold, 14.0 * s, (r.right() - 6.0 * s, r.bottom() - 4.0 * s), Align::Right, if name == "Back" { "‹" } else { "›" }, MUTED);
        if any {
            c.rrect(Rect::new(r.right() - 13.0 * s, r.y + 6.0 * s, 7.0 * s, 7.0 * s), 3.5 * s, ENERGY);
        }
    }

    /// A gun card: its kind's icon, its name, its price or where it is,
    /// and its numbers as bars.
    #[allow(clippy::too_many_arguments)]
    fn gun_card(&self, row: &ShopRow, r: Rect, hot: bool, focused: bool, c: &mut Canvas, s: f32) {
        self.face(r, row.ready, row.on, hot, focused, c, s);
        let p = 12.0 * s;
        let dim = if row.ready || row.on { 1.0 } else { 0.5 };
        let ic = 34.0 * s;
        c.icon(row.icon, (r.x + p, r.y + p), ic, with_alpha(row.tint, dim));
        c.text_fit(Weight::SemiBold, 17.0 * s, (r.x + p + ic + 10.0 * s, r.y + p + 14.0 * s), Align::Left, &row.name, r.w - 2.0 * p - ic - 10.0 * s, with_alpha(TEXT, dim.max(0.6)));
        let (tag, col) = if !row.price.is_empty() {
            (format!("{}  ·  {}", row.price, row.state), if row.ready { ACCENT } else { BAD })
        } else {
            (row.state.clone(), if row.on { GOOD } else { MUTED })
        };
        c.text_fit(Weight::Medium, 13.5 * s, (r.x + p + ic + 10.0 * s, r.y + p + 33.0 * s), Align::Left, &tag, r.w - 2.0 * p - ic - 10.0 * s, col);
        let mut y = r.y + 66.0 * s;
        let step = ((r.bottom() - p - y + 6.0 * s) / row.bars.len().max(1) as f32).min(19.0 * s);
        for (label, f, value) in &row.bars {
            c.text(Weight::Medium, 12.5 * s, (r.x + p, y + 4.5 * s), Align::Left, label, MUTED);
            let bar = Rect::new(r.x + p + 62.0 * s, y - 1.0 * s, r.w - 2.0 * p - 62.0 * s - 52.0 * s, 6.0 * s);
            c.bar(bar, *f, with_alpha(row.tint, 0.85 * dim), [255, 255, 255, 22]);
            c.text(Weight::SemiBold, 12.5 * s, (r.right() - p, y + 4.5 * s), Align::Right, value, with_alpha(TEXT, dim.max(0.6)));
            y += step;
        }
    }

    /// A bullet card: a band of its kind's colour, its icon, its name, its
    /// budget cost as pips and its price.
    #[allow(clippy::too_many_arguments)]
    fn bullet_card(&self, row: &ShopRow, r: Rect, hot: bool, focused: bool, c: &mut Canvas, s: f32) {
        // Landscape since the UI review (11-15): the icon at the left, the
        // name, its budget cost as pips and its price beside it.
        self.face(r, row.ready, row.on, hot, focused, c, s);
        let dim = if row.ready || row.on { 1.0 } else { 0.45 };
        c.rrect(Rect::new(r.x + 6.0 * s, r.y + 4.0 * s, r.w - 12.0 * s, 3.0 * s), 1.5 * s, with_alpha(row.tint, 0.9 * dim));
        let ic = 30.0 * s;
        c.icon(row.icon, (r.x + 22.0 * s, r.y + (r.h - ic) / 2.0 + 3.0 * s), ic, with_alpha(row.tint, dim));
        let tx = r.x + 22.0 * s + ic + 10.0 * s;
        let mut room = r.right() - 8.0 * s - tx;
        if let Some(b) = row.badge {
            let bs = 15.0 * s;
            c.icon(b, (r.right() - bs - 7.0 * s, r.y + 12.0 * s), bs, with_alpha(TEXT, 0.85 * dim.max(0.6)));
            room -= bs + 4.0 * s;
        }
        let px = fit_px(c, &row.name, 13.5 * s, 11.5 * s, room);
        c.text_fit(Weight::SemiBold, px, (tx, r.y + 28.0 * s), Align::Left, &row.name, room, with_alpha(TEXT, dim.max(0.6)));
        let d = 6.0 * s;
        let foot = r.bottom() - 13.0 * s;
        for k in 0..row.cost.min(6) {
            c.rrect(Rect::new(tx + k as f32 * (d + 2.5 * s), foot - d / 2.0 - 4.0 * s, d, d), d / 2.0, with_alpha(ENERGY, 0.85 * dim.max(0.6)));
        }
        if row.cost == 0 {
            c.text(Weight::Medium, 12.0 * s, (tx, foot), Align::Left, "free", MUTED);
        }
        if !row.price.is_empty() {
            c.text(Weight::SemiBold, 13.5 * s, (r.right() - 8.0 * s, foot), Align::Right, &row.price, if row.ready { ACCENT } else { BAD });
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
            Some(Hit::Slot(k)) => (format!("Slot {}: empty", k + 1), "Drag a card here.".into(), String::new(), String::new(), true),
            _ => ("Point at a tile".into(), "The gun fires its cards left to right. Drag a card to a slot to move it; click a card to take it out or put it in.".into(), String::new(), String::new(), true),
        };
        c.text_fit(Weight::SemiBold, 17.0 * s, (r.x + p, r.y + 24.0 * s), Align::Left, &name, r.w - 2.0 * p - 150.0 * s, TEXT);
        let tag = if price.is_empty() { state } else if state.is_empty() { format!("{price} points") } else { format!("{price} points  ·  {state}") };
        c.text_fit(Weight::SemiBold, 14.0 * s, (r.right() - p, r.y + 24.0 * s), Align::Right, &tag, 150.0 * s, if !price.is_empty() && !ready { BAD } else { ACCENT });
        let lead = 19.5 * s;
        let lines = c.wrap(Weight::Medium, 14.0 * s, r.w - 2.0 * p, &about);
        let room = ((r.h - 38.0 * s) / lead).floor().max(1.0) as usize;
        for (k, l) in lines.iter().take(room).enumerate() {
            c.text(Weight::Medium, 14.0 * s, (r.x + p, r.y + 48.0 * s + k as f32 * lead), Align::Left, l, MUTED);
        }
    }

    /// The pages the last drawn list took: an Options tab, the key list or
    /// the campaign (1 elsewhere). The fit tests'.
    pub fn sheets(&self) -> usize {
        if matches!(self.page, Page::Options | Page::Keys | Page::Missions) {
            self.sheets
        } else {
            1
        }
    }

    /// Whether the page last drawn was taller than the screen (it scrolls).
    pub fn scrolls(&self) -> bool {
        self.scroll_max > 0.0
    }

    /// Where `hit` was last drawn (tests and the window's smoke hand).
    fn rect_of(&self, hit: Hit) -> Option<Rect> {
        self.hits.iter().find(|(_, h)| *h == hit).map(|(r, _)| *r)
    }

    /// The centre of the carried gun's slot `k` (0 fires first), as last
    /// drawn (the drag films, `drag=`).
    pub fn slot_at(&self, k: usize) -> Option<(f32, f32)> {
        self.slot_rects.get(k).map(|r| (r.x + r.w / 2.0, r.y + r.h / 2.0))
    }

    /// The centre of the first card out of the gun named `name`, as last
    /// drawn; else of the last gun card so named (the one on sale, or the
    /// gun range's rack, after those owned).
    pub fn card_at(&self, name: &str) -> Option<(f32, f32)> {
        let shop = self.shop.as_ref()?;
        let gun = shop.rows.iter().rposition(|r| r.area == ShopArea::Guns && r.name == name);
        let i = self.card_groups().into_iter().flat_map(|(_, c)| c).find(|&i| shop.rows[i].name == name).or(gun)?;
        self.rect_of(Hit::Buy(i)).map(|r| (r.x + r.w / 2.0, r.y + r.h / 2.0))
    }

    /// A point in the bag's area (a gun card dropped there comes out).
    pub fn bag_at(&self) -> Option<(f32, f32)> {
        self.bag.map(|r| (r.x + r.w * 0.75, r.y + r.h / 2.0))
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

    /// Draws `m`, scrolled so `hit` is in view.
    fn shown(m: &mut StartMenu, c: &mut Canvas, hit: Hit) {
        c.clear();
        m.draw(c);
        m.reveal(hit);
        c.clear();
        m.draw(c);
    }

    fn click(m: &mut StartMenu, c: &mut Canvas, hit: Hit) -> Vec<StartEvent> {
        shown(m, c, hit);
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
        assert_eq!(m.key(KeyCode::Enter, true), vec![StartEvent::Run], "the run is first");
        m.key(KeyCode::ArrowDown, true);
        assert_eq!(m.key(KeyCode::Enter, true), vec![StartEvent::Sandbox], "the sandbox is second");
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
        Shop { heading: "Planet 2 next".into(), lines: vec!["1 planet freed".into()], purse: "200 points".into(), rows, slots: 4, budget: (6, 10), folder_icons: Vec::new(), range: false }
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
                        let mut rects: Vec<Rect> = Vec::new();
                        // Each scrolled into view (the page scrolls since
                        // 11-15), then where it sits on the whole page.
                        for h in m.rows() {
                            shown(&mut m, &mut c, h);
                            let r = m.rect_of(h).unwrap_or_else(|| panic!("{screen:?} {user} {n} {folder:?}: {h:?} not drawn"));
                            assert!(r.bottom() <= screen.1 as f32 + 0.5 && r.right() <= screen.0 as f32 + 0.5 && r.x >= 0.0, "{screen:?} at {user}, {n} cards: {h:?} {r:?} off screen");
                            let fixed = matches!(h, Hit::Next | Hit::EndRun);
                            let r = if fixed { Rect::new(r.x, r.y + 1e5, r.w, r.h) } else { Rect::new(r.x, r.y + m.scroll, r.w, r.h) };
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

    /// The shop stays put when a card goes into the bag or comes out of
    /// it (owner, 11-15 02:58Z): the same scale, the same gun, slot and
    /// tile places, at every screen and UI size.
    #[test]
    fn the_shop_holds_still_when_the_bag_fills() {
        let place = |bag: bool, screen: (u32, u32), user: f32| {
            let mut sh = shop(6);
            for r in sh.rows.iter_mut().filter(|r| r.area == ShopArea::Cards && !r.on) {
                r.group = if bag && r.name.ends_with('3') { "In the bag" } else { "For sale" }.into();
            }
            let mut m = menu();
            m.shop = Some(sh);
            m.show(Page::Shop);
            let mut c = Canvas::new(screen);
            c.ui.user = user;
            m.draw(&mut c);
            let mut at: Vec<(Hit, Rect)> = m.rows().into_iter().filter(|h| !matches!(h, Hit::Buy(i) if *i >= 23)).filter_map(|h| Some((h, m.rect_of(h)?))).collect();
            at.extend(m.slot_rects.iter().map(|&r| (Hit::Next, r)));
            at
        };
        for screen in [(1280, 720), (1600, 900), (1920, 1080)] {
            for user in crate::world::settings::STEPS {
                assert_eq!(place(false, screen, user), place(true, screen, user), "{screen:?} at {user}: the shop moved when a card went into the bag");
            }
        }
    }

    /// The gun range's armory is the shop page with its own buttons: Back
    /// to the range goes on to the round (`NextLevel`), and so do Esc and
    /// a right click with no folder open; Leave the range is `EndRun`. The
    /// main page's Gun range row opens it.
    #[test]
    fn the_gun_ranges_armory_goes_back_to_the_range() {
        let mut m = menu();
        let mut c = Canvas::new((1280, 800));
        assert_eq!(click(&mut m, &mut c, Hit::Range), vec![StartEvent::Range]);
        assert!(m.rows().iter().position(|&h| h == Hit::Range) < m.rows().iter().position(|&h| h == Hit::Options), "before Options");
        let mut shop = shop(6);
        shop.range = true;
        m.shop = Some(shop);
        m.show(Page::Shop);
        assert_eq!(click(&mut m, &mut c, Hit::Next), vec![StartEvent::NextLevel]);
        assert_eq!(click(&mut m, &mut c, Hit::EndRun), vec![StartEvent::EndRun]);
        assert_eq!(m.key(KeyCode::Escape, true), vec![StartEvent::NextLevel]);
        assert_eq!(m.button(Button::Right, true), vec![StartEvent::NextLevel]);
        // A folder open: Esc closes it first.
        m.act(Hit::Folder(0));
        assert!(m.key(KeyCode::Escape, true).is_empty());
        assert_eq!((m.page, m.folder.as_deref()), (Page::Shop, None));
        // The run's shop goes back to the main page, on Run.
        m.shop.as_mut().unwrap().range = false;
        assert!(m.key(KeyCode::Escape, true).is_empty());
        assert_eq!((m.page, m.rows()[m.focus]), (Page::Main, Hit::Run));
    }

    /// Drags `from` to `to` (both as last drawn): press, move past
    /// `DRAG_FROM`, move on, let go.
    fn drag(m: &mut StartMenu, c: &mut Canvas, from: Hit, to: (f32, f32)) -> Vec<StartEvent> {
        c.clear();
        m.draw(c);
        let r = m.rect_of(from).unwrap_or_else(|| panic!("{from:?} not drawn"));
        let a = (r.x + r.w / 2.0, r.y + r.h / 2.0);
        m.moved(Some(a));
        assert!(m.button(Button::Left, true).is_empty());
        m.moved(Some((a.0 + 10.0, a.1 + 10.0)));
        m.moved(Some(to));
        // Drawn mid-drag: the card rides under the pointer, nothing panics.
        c.clear();
        m.draw(c);
        m.button(Button::Left, false)
    }

    fn centre(r: Rect) -> (f32, f32) {
        (r.x + r.w / 2.0, r.y + r.h / 2.0)
    }

    /// Scrolled so the gun's slots are partly under the title, a drop
    /// lands in the slot it is let go on, by its own number, and one let
    /// go over the title (on a slot scrolled under it) does nothing
    /// (11-15 review: the slot list was cut to the shown ones, so a drop
    /// moved the card to the wrong slot).
    #[test]
    fn a_drop_after_scrolling_lands_in_its_own_slot() {
        let mut m = menu();
        // A gun of twelve slots: several rows of them.
        let mut sh = shop(14);
        sh.slots = 12;
        m.shop = Some(sh);
        m.show(Page::Shop);
        let mut c = Canvas::new((1280, 720));
        c.ui.user = 2.0;
        c.clear();
        m.draw(&mut c);
        assert!(m.scroll_max > 0.0, "the shop scrolls at 200% on 1280 x 720");
        // Scroll until the first slot is above the page's rows.
        let body = m.body.unwrap();
        let mut k = None;
        for _ in 0..200 {
            m.wheel(-1);
            c.clear();
            m.draw(&mut c);
            if m.slot_rects[0].bottom() < body.y {
                k = (1..m.slot_rects.len()).find(|&j| body.contains(centre(m.slot_rects[j])));
                if k.is_some() {
                    break;
                }
            }
        }
        let card = m.card_groups().into_iter().flat_map(|(_, v)| v).find(|&i| m.rect_of(Hit::Buy(i)).is_some()).expect("a card in view");
        let k = k.expect("a slot past the first in view with the first scrolled away");
        {
            let to = centre(m.slot_rects[k]);
            assert_eq!(drag(&mut m, &mut c, Hit::Buy(card), to), vec![StartEvent::Move(card, k)]);
        }
        let hidden = m.slot_rects[0];
        {
            assert!(hidden.bottom() < body.y);
            assert!(drag(&mut m, &mut c, Hit::Buy(card), centre(hidden)).is_empty(), "a slot scrolled under the title takes no drop");
        }
    }

    /// The owner, 20:32Z: "When I buy a card it goes into the gun but I
    /// don't see how to move cards around." A card dragged onto a slot
    /// moves there (a bag card goes in at that slot), onto the bag comes
    /// out, onto its own slot or nowhere does nothing; a short wobble is
    /// still a click; Q and E move the focused card a slot and the focus
    /// follows.
    #[test]
    fn cards_drag_to_slots_and_q_and_e_move_them() {
        let mut m = menu();
        m.shop = Some(shop(8));
        m.show(Page::Shop);
        let mut c = Canvas::new((1600, 900));
        c.clear();
        m.draw(&mut c);
        // Rows 23-25 are in the gun (slots 1-3), slot 4 is empty, 26-30 in
        // the bag or for sale.
        let (first, gun3) = (23, 25);
        assert_eq!((m.slot_of(gun3), m.slot_of(26)), ((Some(2), 3), (None, 3)));
        // (The shop does not change here, so neither does its layout.)
        let slot: Vec<(f32, f32)> = m.slot_rects.iter().map(|&r| centre(r)).collect();
        assert_eq!(slot.len(), 4);
        assert_eq!(drag(&mut m, &mut c, Hit::Buy(first), slot[2]), vec![StartEvent::Move(first, 2)], "first to third");
        assert_eq!(drag(&mut m, &mut c, Hit::Buy(gun3), slot[0]), vec![StartEvent::Move(gun3, 0)], "third to first");
        assert!(drag(&mut m, &mut c, Hit::Buy(gun3), slot[2]).is_empty(), "its own slot");
        assert!(m.rect_of(Hit::Slot(3)).is_some(), "the empty slot is a target");
        assert_eq!(drag(&mut m, &mut c, Hit::Buy(26), slot[3]), vec![StartEvent::Move(26, 3)], "a bag card into the empty slot");
        assert_eq!(drag(&mut m, &mut c, Hit::Buy(27), slot[1]), vec![StartEvent::Move(27, 1)], "a bag card in between");
        let bag = m.rect_of(Hit::Buy(28)).unwrap();
        assert_eq!(drag(&mut m, &mut c, Hit::Buy(first), (bag.right() + 4.0, bag.y + 4.0)), vec![StartEvent::Buy(first)], "onto the bag: out");
        assert!(drag(&mut m, &mut c, Hit::Buy(26), centre(bag)).is_empty(), "a bag card onto the bag");
        assert!(drag(&mut m, &mut c, Hit::Buy(first), (5.0, 5.0)).is_empty(), "let go nowhere");
        assert!(drag(&mut m, &mut c, Hit::Buy(20), slot[0]).is_empty(), "guns do not drag");
        // A wobble under `DRAG_FROM` is a click.
        let r = m.rect_of(Hit::Buy(first)).unwrap();
        m.moved(Some(centre(r)));
        m.button(Button::Left, true);
        m.moved(Some((centre(r).0 + 3.0, centre(r).1)));
        assert_eq!(m.button(Button::Left, false), vec![StartEvent::Buy(first)]);
        // Q and E: the focus is on the second card in the gun.
        m.focus = m.rows().iter().position(|&h| h == Hit::Buy(first + 1)).unwrap();
        let at = m.focus;
        assert_eq!(m.key(KeyCode::KeyQ, true), vec![StartEvent::Move(first + 1, 0)]);
        assert_eq!(m.focus, at - 1, "the focus goes with the card");
        m.focus = at;
        assert_eq!(m.key(KeyCode::KeyE, true), vec![StartEvent::Move(first + 1, 2)]);
        assert_eq!(m.focus, at + 1);
        m.focus = at - 1;
        assert!(m.key(KeyCode::KeyQ, true).is_empty(), "the first slot can't go sooner");
        m.focus = at + 1;
        assert!(m.key(KeyCode::KeyE, true).is_empty(), "the last can't go later");
        m.focus = m.rows().iter().position(|&h| h == Hit::Buy(26)).unwrap();
        assert!(m.key(KeyCode::KeyE, true).is_empty(), "a bag card has no slot");
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
                // (Scrolled to each since 11-15: a big UI size scrolls.)
                for h in m.rows() {
                    shown(&mut m, &mut c, h);
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
        assert_eq!(m.rows()[3], Hit::RandomPlanet, "after Run, Sandbox and Campaign");
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
        assert!(!m.option("panic").unwrap().is_default(), "a click on the row only takes the focus");
        assert_eq!(m.rows()[m.focus], Hit::Reset(1));
        click(&mut m, &mut c, Hit::Default(1));
        assert!(m.option("panic").unwrap().is_default(), "its \"back to\" line resets it");
        c.clear();
        m.draw(&mut c);
        assert!(m.rect_of(Hit::Default(1)).is_none(), "and goes once it is the default");
    }

    /// The UI size is a display choice: All to defaults leaves it, and the
    /// sandbox's line does not list it as a rule (the review, 11-15).
    #[test]
    fn all_to_defaults_keeps_the_ui_size() {
        let mut m = menu();
        m.options.push(Opt::new("ui", "UI and text size", &["100%", "150%", "200%"], "100%", "200%", ""));
        let mut c = Canvas::new((1280, 800));
        m.show(Page::Options);
        click(&mut m, &mut c, Hit::Turn(1, 1));
        click(&mut m, &mut c, Hit::ResetAll);
        assert_eq!(m.changed("ui"), Some("200%"));
        assert_eq!(m.changed("panic"), None);
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
    /// on the screen (they ran off at 150% on 900 lines before the cap),
    /// a tab of more than fit paged, not scrolled.
    #[test]
    fn every_ui_size_reaches_every_option() {
        for screen in [(1280, 800), (1600, 900), (1920, 1080)] {
            for user in crate::world::settings::STEPS {
                let mut m = menu();
                // As many options as the game has, and more.
                for k in 0..20 {
                    m.options.push(Opt::new(&format!("o{k}"), "An option", &["0", "1"], "0", "0", "What it does"));
                }
                m.show(Page::Options);
                let mut c = Canvas::new(screen);
                c.ui.user = user;
                let last = m.options.len() - 1;
                for h in [Hit::Reset(0), Hit::Reset(last), Hit::Back, Hit::ResetAll] {
                    shown(&mut m, &mut c, h);
                    let r = m.rect_of(h).unwrap();
                    // (Back is in the header, over the body.)
                    assert!(h == Hit::Back || (r.y >= m.body.unwrap().y && r.bottom() <= m.body.unwrap().bottom() + 0.5), "{screen:?} at {user}: {h:?} {r:?} not in view");
                    assert_eq!(m.scroll_max, 0.0, "{screen:?} at {user}: the options page scrolls");
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
                shown(&mut m, &mut c, Hit::Quit);
                let r = m.rect_of(Hit::Quit).unwrap();
                assert!(r.bottom() <= screen.1 as f32 && r.right() <= screen.0 as f32, "{screen:?} at {user}: {r:?}");
                m.scroll = 0.0;
                c.clear();
                m.draw(&mut c);
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
            m.scroll = 0.0;
            c.clear();
            m.draw(&mut c);
            let a = m.rect_of(Hit::Sandbox).unwrap();
            let q = m.rect_of(Hit::Run).unwrap();
            m.moved(Some((a.x + 4.0, a.y + 4.0)));
            m.button(Button::Left, true);
            m.moved(Some((q.x + 4.0, q.y + 4.0)));
            assert!(m.button(Button::Left, false).is_empty());
        }
    }

    /// The owner, 11-15 07:56Z: "Changing the UI /text percentage increase
    /// does do what it should the menus are unaffected. The gun range is
    /// unaffected (I cannot read any of the text)". Every page is drawn at
    /// the UI size: 200% doubles it (within the cap on the window), and a
    /// page that no longer fits scrolls to every row, the keyboard's too.
    #[test]
    fn every_page_follows_the_ui_size() {
        for screen in [(1280, 800), (1920, 1080), (2560, 1440)] {
            for page in [Page::Main, Page::Missions, Page::Options, Page::Keys, Page::Shop] {
                let size = |user: f32| {
                    let mut m = menu();
                    m.keys = (0..40).map(|k| (format!("K{k}"), "Does a thing".to_string())).collect();
                    let mut sh = shop(14);
                    sh.range = page == Page::Shop;
                    m.shop = Some(sh);
                    m.show(page);
                    let mut c = Canvas::new(screen);
                    c.ui.user = user;
                    c.clear();
                    m.draw(&mut c);
                    // Every row reachable by the keyboard alone.
                    for _ in 0..m.rows().len() {
                        m.key(KeyCode::ArrowDown, true);
                        c.clear();
                        m.draw(&mut c);
                        c.clear();
                        m.draw(&mut c);
                        let h = m.rows()[m.focus];
                        let r = m.rect_of(h).unwrap_or_else(|| panic!("{screen:?} {page:?} at {user}: {h:?} scrolled away"));
                        let b = m.body.unwrap();
                        // The shop's two buttons and the inner pages'
                        // header are fixed, over and under the body.
                        let fixed = matches!(h, Hit::Next | Hit::EndRun) || (matches!(page, Page::Options | Page::Keys | Page::Missions) && h == Hit::Back);
                        assert!(r.y >= 0.0 && r.bottom() <= screen.1 as f32, "{screen:?} {page:?} at {user}: {h:?} {r:?} off the screen");
                        assert!(fixed || (r.y >= b.y - 0.5 && r.bottom() <= b.bottom() + 0.5), "{screen:?} {page:?} at {user}: {h:?} {r:?} not in view of {b:?}");
                    }
                    let probe = match page {
                        Page::Shop => Hit::Buy(0),
                        Page::Options => Hit::Reset(0),
                        Page::Missions => Hit::Mission(0),
                        Page::Keys | Page::Main => Hit::Back,
                    };
                    let probe = if page == Page::Main { Hit::Sandbox } else { probe };
                    m.scroll = 0.0;
                    shown(&mut m, &mut c, probe);
                    (m.drawn.iter().find(|(_, h)| *h == probe).unwrap().0.h, c.ui.px(screen))
                };
                let (a, pa) = size(1.0);
                let (b, pb) = size(2.0);
                let grew = pb / pa;
                assert!(grew > 1.25, "{screen:?}: 200% is capped to {grew}x");
                if page == Page::Shop {
                    // The shop starts a little over the UI's size where it
                    // has room, but is never under it: a tile is 78 units.
                    assert!(a >= 78.0 * pa * 0.97 && b >= 78.0 * pb * 0.97, "{screen:?}: the shop's tiles {a} at 100%, {b} at 200%, under the UI size");
                } else {
                    assert!(b >= a * grew * 0.9, "{screen:?} {page:?}: 200% drew {b} against {a} at 100% (the size grew {grew}x)");
                }
            }
        }
    }
    /// A menu with the game's own lists' sizes: 17 options on three tabs
    /// with the longest notes, eight missions and 46 key lines (the
    /// game's, 11-15), some of them long.
    fn full_menu() -> StartMenu {
        let long = "When the slime has eaten enough from its crystals, the slime near your soldiers and buildings creeps toward them, then lunges; the longest wait between surges in steps (1800 is 60 s); 0 is off";
        let mut options = vec![Opt::new("ui", "UI and text size", &["75%", "100%", "200%"], "100%", "200%", "Every panel, card, tooltip and this menu, over the screen's own scale; Ctrl + and Ctrl - anywhere")];
        for (k, tab) in [("map", "Planet"), ("planet", "Planet"), ("own", "Planet"), ("plants", "Planet")] {
            options.push(Opt::new(k, "Your own mould", &["off", "on"], "off", "on", "A green mould the squad sows and feeds; sandbox on the race world").in_tab(tab));
        }
        for k in ["mode", "rounds", "bullet", "dev"] {
            options.push(Opt::new(k, "Bullet speed", &["0", "30", "45", "60", "90"], "45", "0", "Dish cells a game second a shot flies; 0 is the instant ray"));
        }
        for k in ["panic", "wither", "energy_flow", "condense", "roam", "thicken", "surge", "flinch"] {
            options.push(Opt::new(k, "Shared food", &["0", "0.025", "0.05", "0.1"], "0.05", "0", long).in_tab("The mould"));
        }
        let missions = ["Landfall", "First contact", "The gaps", "Cut the cord", "Kill zone", "The nest", "Spore season", "Two moulds"]
            .iter()
            .map(|n| Entry { stem: n.to_lowercase(), name: n.to_string(), says: "Grow your squad to 12 and harvest 3 crystals at once, then hold them while the slime surges".into(), detail: "The Passes, The Swarmer".into() })
            .collect();
        let keys = (0..46)
            .map(|k| match k % 5 {
                0 => ("Right click a half-built building".to_string(), "Back to building it (nothing paid again)".to_string()),
                1 => ("Esc".into(), "Let go of the squad: every squad runs on its own and WASD pans; Esc again: the menu".into()),
                _ => (format!("Ctrl + {k}"), "Make a group, select it".into()),
            })
            .collect();
        StartMenu::new("Larder World", missions, options, keys)
    }

    /// The owner, 11-15 17:31Z: "Having to scroll is horrible. That's not
    /// professional." Every Options tab, every page of the key list and
    /// the campaign fit one screen at the worst sizes (16:9 and 21:9 at
    /// 200%, a small laptop at 175%, the window as it opens at 125%, and
    /// smaller than the game opens): nothing scrolls, every button and
    /// word is on the screen, no two words' glyphs touch, none under 12.5
    /// units, and the options' tabs and the campaign need no second page
    /// at any of them. The main page and the shop join when the owner has
    /// picked their layouts (`Reports/menu-layout-research-2026-11-15/`).
    #[test]
    fn the_inner_pages_fit_one_screen() {
        let sizes = [((1920, 1080), 2.0), ((1280, 540), 2.0), ((1280, 720), 2.0), ((1920, 1080), 1.0), ((1280, 800), 1.75), ((1600, 972), 1.25), ((640, 400), 1.0), ((2560, 1440), 3.0)];
        for (screen, user) in sizes {
            let (sw, sh) = (screen.0 as f32, screen.1 as f32);
            let mut m = full_menu();
            let mut c = Canvas::new(screen);
            c.ui.user = user;
            let s = (c.ui.px(screen) * 20.0 + 1e-3).floor() / 20.0;
            let check = |m: &StartMenu, c: &Canvas, at: &str| {
                assert!(!m.scrolls(), "{at}: scrolls by {}", m.scroll_max);
                for (r, h) in &m.hits {
                    assert!(r.x >= -0.5 && r.y >= -0.5 && r.right() <= sw + 0.5 && r.bottom() <= sh + 0.5, "{at}: {h:?} {r:?} off the screen");
                }
                assert!(m.hits.iter().any(|(_, h)| *h == Hit::Back), "{at}: no Back");
                let runs: Vec<_> = c.runs.iter().filter(|t| t.ink.0 < t.ink.2).collect();
                for (k, t) in runs.iter().enumerate() {
                    assert!(t.ink.0 >= 0.0 && t.ink.1 >= 0.0 && t.ink.2 <= sw && t.ink.3 <= sh, "{at}: {:?} {:?} off the screen", t.text, t.ink);
                    assert!(t.px >= 12.5 * s - 0.01, "{at}: {:?} at {} px, under 12.5 x {s}", t.text, t.px);
                    for u in &runs[k + 1..] {
                        let apart = t.ink.2 <= u.ink.0 || u.ink.2 <= t.ink.0 || t.ink.3 <= u.ink.1 || u.ink.3 <= t.ink.1;
                        assert!(apart, "{at}: {:?} {:?} and {:?} {:?} overlap", t.text, t.ink, u.text, u.ink);
                    }
                }
            };
            let draw = |m: &mut StartMenu, c: &mut Canvas| {
                for _ in 0..2 {
                    c.clear();
                    m.draw(c);
                }
            };
            // Each Options tab, every row focused in turn (the note shows
            // the focused one's), with every option moved off its default
            // (the note's longest: its "back to" button).
            m.show(Page::Options);
            for tab in 0..3 {
                m.open_tab(tab);
                for row in 0..m.rows().len() {
                    m.focus = row;
                    draw(&mut m, &mut c);
                    let at = format!("{screen:?} at {user}, Options tab {tab} row {row}");
                    check(&m, &c, &at);
                    assert_eq!(m.sheets(), 1, "{at}: the tab takes {} pages", m.sheets());
                }
            }
            // The key list, page by page.
            m.open_tab(3);
            assert_eq!(m.page, Page::Keys);
            draw(&mut m, &mut c);
            let pages = m.sheets();
            for k in 0..pages {
                draw(&mut m, &mut c);
                check(&m, &c, &format!("{screen:?} at {user}, Keys page {k} of {pages}"));
                assert_eq!(m.sheet, k);
                m.key(KeyCode::ArrowRight, true);
            }
            // (How many pages the game's own key list takes: the window's
            // test `the_menus_real_lists_fit_one_screen`.)
            assert!(pages >= 1);
            // The campaign, each mission focused in turn.
            m.show(Page::Missions);
            for row in 0..m.rows().len() {
                m.focus = row;
                draw(&mut m, &mut c);
                let at = format!("{screen:?} at {user}, Campaign row {row}");
                check(&m, &c, &at);
                assert_eq!(m.sheets(), 1, "{at}: the campaign takes {} pages", m.sheets());
            }
        }
    }

    /// A list longer than the screen holds pages, not scrolls: 30
    /// missions and 30 options on a tab at 200% on 1080 lines, every row
    /// reached by the keyboard and by the pager's arrows, and back.
    #[test]
    fn a_long_list_pages_instead_of_scrolling() {
        let mut m = full_menu();
        m.missions = (0..30).map(|i| Entry { stem: format!("{i}"), name: format!("Mission {i}"), says: "Do it".into(), detail: "The Fork".into() }).collect();
        for k in 0..30 {
            m.options.push(Opt::new(&format!("o{k}"), "An option", &["0", "1"], "0", "0", "What it does").in_tab("The mould"));
        }
        let mut c = Canvas::new((1920, 1080));
        c.ui.user = 2.0;
        for page in [Page::Missions, Page::Options] {
            m.show(page);
            if page == Page::Options {
                m.open_tab(2);
            }
            c.clear();
            m.draw(&mut c);
            let pages = m.sheets();
            assert!(pages >= 3, "{page:?}: {pages} pages");
            // The keyboard: every row drawn when focused, the page following.
            for row in 0..m.rows().len() {
                m.focus = row;
                c.clear();
                m.draw(&mut c);
                let h = m.rows()[row];
                assert!(m.rect_of(h).is_some(), "{page:?}: row {row} {h:?} not drawn when focused");
                assert!(!m.scrolls(), "{page:?}: scrolls");
            }
            // The pager's arrows: on, round to the first, and back.
            m.focus = 0;
            c.clear();
            m.draw(&mut c);
            for k in 1..=pages {
                click(&mut m, &mut c, Hit::Flip(1));
                assert_eq!(m.sheet, k % pages, "{page:?}: page {k}");
            }
            click(&mut m, &mut c, Hit::Flip(-1));
            assert_eq!(m.sheet, pages - 1);
        }
    }
}
