//! **The start menu** (cloud plan 2026-11-14, item C2): what a player sees
//! before a round. Its pages: the main page (sandbox, campaign, options,
//! keys, quit), the campaign's mission list, an options page for the
//! window's switches in tabs with the focused one's note and default
//! beside them, and the key card (its last tab; also F1 or ? from any
//! page), the run shop / gun range armory (`shop`), and what the build
//! holds (`BuildPage`: what is new, with the lines new to this PC marked,
//! and the things to try with their ticks). Every page fits
//! one screen at every UI size and pages a list too long for it rather
//! than scroll (owner, 11-15 17:31Z: "Having to scroll is horrible"):
//! the inner pages (Options, the keys, the campaign) under a one-line
//! header, the main page as menu A and the shop as shop B, the owner's
//! picks of 19:37Z (`Reports/menu-layout-research-2026-11-15/`,
//! `Reports/shop-b-2026-11-15/`).
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
use super::ui::{experiment_card, experiment_card_h, experiment_meter, ExpInfo, Experiment};

mod shop;

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
    /// A declared switch's own words (`experiment`): a tab whose options
    /// all carry them is drawn as the switches' cards, not as rows.
    pub exp: Option<ExpInfo>,
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
        Opt { key: key.into(), label: label.into(), choices: c, default, at, note: note.into(), tab: String::new(), exp: None }
    }

    /// The same option as a declared switch, with its owner, date and
    /// reasons (the Experiments tab's cards).
    pub fn experiment(mut self, info: ExpInfo) -> Self {
        self.exp = Some(info);
        self
    }

    /// It as the card draws it (`ui::experiment_card`).
    fn as_experiment(&self) -> Experiment {
        Experiment { key: self.key.clone(), label: self.label.clone(), choices: self.choices.clone(), default: self.default, at: self.at, next: None, info: self.exp.clone().unwrap_or_default() }
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

/// One line of the game's "what is new" list, as the game hands it over.
#[derive(Clone, Debug, PartialEq)]
pub struct NewsLine {
    /// The day it was written (YYYY-MM-DD) and the lane that wrote it.
    pub date: String,
    pub lane: String,
    /// What the player will see.
    pub sees: String,
    /// How to try it; may be empty.
    pub try_it: String,
    /// No round has been played on this PC on a build that held it.
    pub new: bool,
}

/// One thing to try, with its tick.
#[derive(Clone, Debug, PartialEq)]
pub struct TryLine {
    pub text: String,
    pub tried: bool,
}

/// **What the build says of itself** (the plan of 2026-11-17: "The start
/// menu shows the build number, what is new since the build you last
/// played ... and at most five things worth trying"; the owner that day:
/// features left switched off are a problem because he cannot tell what
/// is on or off or what needs testing). The game fills it from
/// `build::news`; the menu shows the build in the main page's corner, a
/// row that counts the new lines, and a page of two tabs.
#[derive(Clone, Debug, Default, PartialEq)]
pub struct BuildPage {
    /// "build 3127", the page's name, and the whole of it for the corner
    /// ("build 3127 (d08583d, 2026-11-18)").
    pub name: String,
    pub long: String,
    /// The last build a round was played on here ("build 3098,
    /// 2026-11-17"); None on a PC that has not played.
    pub last: Option<String>,
    /// What is new, the lines new to this PC first, each the newest first.
    pub news: Vec<NewsLine>,
    /// At most five, the most important first.
    pub to_try: Vec<TryLine>,
}

impl BuildPage {
    fn fresh(&self) -> usize {
        self.news.iter().filter(|n| n.new).count()
    }

    fn untried(&self) -> usize {
        self.to_try.iter().filter(|t| !t.tried).count()
    }

    /// One sentence on the news: how much is new, and since which build.
    fn since(&self) -> String {
        let n = self.fresh();
        let last = self.last.as_ref().map_or(String::new(), |l| format!(" ({l})"));
        match (n, &self.last) {
            (0, _) => format!("Nothing new since you last played{last}"),
            (_, None) => "All of it is new here: no round has been played on this PC yet".into(),
            (1, _) => format!("1 change since you last played{last}"),
            _ => format!("{n} changes since you last played{last}"),
        }
    }

    /// And one on the things to try.
    fn left(&self) -> String {
        match (self.untried(), self.to_try.len()) {
            (_, 0) => String::new(),
            (0, n) => format!("All {n} things to try are ticked"),
            (k, n) => format!("{k} of {n} things to try are not ticked yet"),
        }
    }
}

/// Where a shop row sits on the page (owner, 2026-11-14 16:55Z: "This
/// whole menu should not just be uniform buttons. There should be one area
/// for unlocking/upgrading that should match the player menu from the
/// screen (buildings and tools should be separate menus that you open up).
/// The second area of the screen is for Guns/bullet cards." The first
/// shop, #280, was one list of uniform rows in columns.)
///
/// Since Shop B (owner's pick, 11-15 19:37Z) every area is a tab of one
/// counter, so Buildings and Tools are tabs like Squad, not folders.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ShopArea {
    /// A tile on the tab named by the row's `group` (Squad, Buildings,
    /// Tools, Dev mode); with no group, a button on the bottom bar (the gun
    /// range's tools).
    Upgrades,
    /// A gun, a row of the Guns tab's table.
    Guns,
    /// A bullet card: those `on` fill the carried gun's slots, those in
    /// the `BAG` group sit in the bag under them, the rest on the tab named
    /// by their `group` (Cards; the range's Bullets and its shelves).
    Cards,
}

/// The group of the cards owned and out of the gun.
pub const BAG: &str = "In the bag";

/// A row of the run's shop page (the game fills it: `mould::run::Run::shop`).
#[derive(Clone, Debug, PartialEq)]
pub struct ShopRow {
    /// The tab it sits on (a card's shelf, `BAG`, "Guns", an upgrade's
    /// group; an empty group is a tool on the shop's bottom bar).
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
    /// Its place in a grid of bullets, row and column, each as an order
    /// and a name (the gun range's shelf: shape by payload, so its tiles
    /// need no names). A tab lays out as the grid only when each of its
    /// cards has its own cell.
    pub cell: Option<((u8, String), (u8, String))>,
}

impl ShopRow {
    /// A row with the plain look; the game sets the rest.
    pub fn new(group: &str, area: ShopArea, icon: Icon, name: &str) -> Self {
        ShopRow { group: group.into(), area, icon, tint: TEXT, name: name.into(), about: String::new(), price: String::new(), state: String::new(), ready: false, on: false, level: (0, 0), bars: Vec::new(), cost: 0, badge: None, cell: None }
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
    /// The gun range's armory, not a run's shop: its buttons go back to
    /// the range or leave it, and Esc goes back to the range.
    pub range: bool,
    /// The gun slots show the turrets' deck (`run::Pick::EditTurrets`):
    /// the bench is named for them, not the carried gun.
    pub turrets: bool,
    /// The bench's name: the loadout shown and its gun (2026-11-19); empty
    /// names it by the carried gun, as before.
    pub bench: String,
    /// The loadout panel opened on a planet (H): its one button goes back
    /// to the planet, as Esc does, and nothing is for sale.
    pub field: bool,
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
    /// The slime viewer (2026-11-20, `viewer.rs` in the window: each slime
    /// and its events on demand, its looks and rules to tune).
    Viewer,
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
    /// Thing to try `i` ticked or unticked: the game remembers it.
    Tried(usize, bool),
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
    /// What the build holds: what is new, and the things to try.
    News,
}

#[derive(Clone, Copy, Debug, PartialEq)]
enum Hit {
    Panel,
    Sandbox,
    Run,
    Range,
    Viewer,
    /// A shop row: a click picks it (the pane tells it), Enter buys it.
    Buy(usize),
    /// Shop row `i` bought or done at once (the pane's button, the gun
    /// range's tools).
    Do(usize),
    /// The shop's tab `k`.
    ShopTab(usize),
    /// The bag's cards a page back or on (a bag fuller than its row).
    BagFlip(isize),
    /// An empty slot of the carried gun (a place to drop a card).
    Slot(usize),
    Next,
    EndRun,
    Campaign,
    Options,
    EditMaps,
    RandomPlanet,
    Quit,
    Back,
    Mission(usize),
    /// Option `i` one choice back or on.
    Turn(usize, isize),
    /// Option `i` set to its choice `k` (an Experiments card's button).
    Pick(usize, usize),
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
    /// The main page's "What is new" row.
    Build,
    /// "What is new" line `i`: a click or the keyboard's row tells it in
    /// full beside the list.
    Line(usize),
    /// Thing to try `i`: a click or Enter ticks it.
    Try(usize),
    /// The build page's tab `k`: what is new, the things to try.
    NewsTab(usize),
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
    /// What the build holds (the main page's row and corner, the News
    /// page); None draws neither.
    pub build: Option<BuildPage>,
    /// How many declared switches the plan allows at once (`MOST_OFF`),
    /// for the Experiments tab's count.
    pub most_off: usize,
    /// A run is under way: the main page's Run row goes back to it.
    pub run_live: bool,
    /// The shop's tab showing, whether the player has picked something
    /// on it (the pane tells the focused row; until then, the planet
    /// ahead), and the bag's page.
    shop_tab: usize,
    picked: bool,
    bag_sheet: usize,
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
    /// The build page's tab showing (0 what is new, 1 the things to try),
    /// and the first thing to try on each page of that list as last drawn
    /// (its rows are as tall as their text, so a page has no fixed count).
    news_tab: usize,
    try_first: Vec<usize>,
}

/// The inner pages' header band and the hint line under them, UI units.
const HEAD: f32 = 56.0;
const FOOT: f32 = 44.0;

/// How far the pointer moves, held down on a card, before a click becomes
/// a drag (pixels).
const DRAG_FROM: f32 = 6.0;
/// A bullet card in the shop, UI units: half the 184 x 64 of the UI
/// review's landscape cards (owner, 11-15 17:41Z; 78 x 98 portrait before
/// that), stretched on the counter to fill its rows.
const CARD_W: f32 = 150.0;
const CARD_H: f32 = 50.0;

impl StartMenu {
    pub fn new(title: &str, missions: Vec<Entry>, options: Vec<Opt>, keys: Vec<(String, String)>) -> Self {
        StartMenu { title: title.into(), missions, options, keys, page: Page::Main, edit_maps: false, planets: false, notice: None, shop: None, run_live: false, shop_tab: 0, picked: false, bag_sheet: 0, focus: 0, keys_from: Page::Main, hover: None, pressed: None, press_at: None, drag: None, slot_rects: Vec::new(), bag: None, hits: Vec::new(), scroll: 0.0, scroll_max: 0.0, body: None, drawn: Vec::new(), follow: false, opt_tab: 0, sheet: 0, sheets: 1, per: 1, build: None, most_off: 3, news_tab: 0, try_first: Vec::new() }
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
        (self.shop_tab, self.picked, self.bag_sheet) = (0, false, 0);
        self.scroll = 0.0;
        self.sheet = 0;
        self.news_tab = 0;
    }

    /// The build page's tab `k` (0 what is new, 1 the things to try).
    pub fn open_news_tab(&mut self, k: usize) {
        if self.page != Page::News {
            self.show(Page::News);
        }
        (self.news_tab, self.focus, self.sheet) = (k % 2, 0, 0);
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
            // (On the shop, the tab that holds it.)
            if let (Page::Shop, Hit::Buy(i)) = (self.page, hit) {
                if let Some(k) = self.shop_tabs().iter().position(|t| t.rows.contains(&i)) {
                    if k != self.shop_tab {
                        self.open_shop_tab(k);
                    }
                }
            }
            if let (Page::Options, Hit::Reset(i)) = (self.page, hit) {
                if let Some(k) = self.options.get(i).and_then(|o| self.tabs().iter().position(|t| t == tab_name(o))) {
                    self.opt_tab = k;
                }
            }
            if let Some(k) = self.rows().iter().position(|&h| h == hit) {
                self.focus = k;
                // (The shop's bag turns to it, as for the keyboard.)
                self.follow = true;
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
        // (The slots keep their places in the list, so `k` is the slot's
        // own number: when the shop scrolled, dropping only those in view
        // once moved a card to the wrong slot. It no longer scrolls.)
        if let Some(k) = self.slot_rects.iter().position(|r| r.contains(p)) {
            return (from != Some(k)).then_some(StartEvent::Move(i, k));
        }
        (from.is_some() && self.bag.is_some_and(|b| b.contains(p))).then_some(StartEvent::Buy(i))
    }

    /// The rows the keyboard walks on this page, top to bottom.
    fn rows(&self) -> Vec<Hit> {
        match self.page {
            // (No Keys row since 11-15: the key list is Options' last tab,
            // and F1 or ? opens it from anywhere.)
            Page::Main => [Hit::Run, Hit::Sandbox, Hit::Campaign].into_iter().chain(self.planets.then_some(Hit::RandomPlanet)).chain([Hit::Range, Hit::Viewer]).chain(self.build.is_some().then_some(Hit::Build)).chain([Hit::Options]).chain(self.edit_maps.then_some(Hit::EditMaps)).chain([Hit::Quit]).collect(),
            Page::Missions => (0..self.missions.len()).map(Hit::Mission).chain([Hit::Back]).collect(),
            Page::Options => self.tab_opts().into_iter().map(Hit::Reset).chain([Hit::ResetAll, Hit::Back]).collect(),
            Page::Keys => vec![Hit::Back],
            Page::Shop => self.shop_rows(),
            Page::News => {
                let b = self.build.as_ref();
                let n = b.map_or(0, |b| if self.news_tab == 0 { b.news.len() } else { b.to_try.len() });
                (0..n).map(|i| if self.news_tab == 0 { Hit::Line(i) } else { Hit::Try(i) }).chain([Hit::Back]).collect()
            }
        }
    }

    fn back(&mut self) {
        if self.page == Page::Shop {
            // A refused buy's notice stays in the shop.
            self.notice = None;
        }
        let to = if self.page == Page::Keys { self.keys_from } else { Page::Main };
        let from = self.page;
        self.show(to);
        // Back on the main page, on the row that led away.
        if to == Page::Main {
            let led = match from {
                Page::Missions => Hit::Campaign,
                Page::Options => Hit::Options,
                Page::Keys => Hit::Options,
                Page::Shop if self.shop.as_ref().is_some_and(|s| s.range) => Hit::Range,
                Page::Shop => Hit::Run,
                Page::News => Hit::Build,
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
            Hit::Viewer => return Some(StartEvent::Viewer),
            // (A new try: the game says again if it can't be done.)
            Hit::Buy(i) | Hit::Do(i) => {
                self.notice = None;
                return Some(StartEvent::Buy(i));
            }
            Hit::ShopTab(k) => self.open_shop_tab(k),
            Hit::BagFlip(by) => self.bag_sheet = (self.bag_sheet as isize + by).max(0) as usize,
            Hit::Slot(_) => {}
            Hit::Next => return Some(StartEvent::NextLevel),
            Hit::EndRun => return Some(StartEvent::EndRun),
            Hit::Campaign => self.show(Page::Missions),
            Hit::Options => self.show(Page::Options),
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
            // Every rule back, the UI size and the volume kept (the
            // player's choices, not rules).
            Hit::ResetAll => {
                for o in self.options.iter_mut().filter(|o| !matches!(o.key.as_str(), "ui" | "volume")) {
                    o.at = o.default;
                }
            }
            Hit::Default(i) => self.options[i].at = self.options[i].default,
            Hit::Pick(i, k) => {
                self.options[i].at = k.min(self.options[i].choices.len() - 1);
                if let Some(k) = self.rows().iter().position(|&r| r == Hit::Reset(i)) {
                    self.focus = k;
                }
            }
            Hit::Tab(k) => self.open_tab(k),
            Hit::Flip(by) => self.flip(by),
            Hit::Build => self.show(Page::News),
            Hit::NewsTab(k) => self.open_news_tab(k),
            // (A line's row only takes the focus: `button`.)
            Hit::Line(_) => {}
            Hit::Try(i) => {
                let t = self.build.as_mut().and_then(|b| b.to_try.get_mut(i))?;
                t.tried = !t.tried;
                let now = t.tried;
                if let Some(k) = self.rows().iter().position(|&r| r == Hit::Try(i)) {
                    self.focus = k;
                }
                return Some(StartEvent::Tried(i, now));
            }
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
        if matches!(self.page, Page::Options | Page::Missions | Page::Shop) || (self.page == Page::News && self.news_tab == 0) {
            self.focus = self.sheet * self.per.max(1);
        } else if self.page == Page::News {
            self.focus = self.try_first.get(self.sheet).copied().unwrap_or(0);
        }
    }

    /// Esc (or a right click) on the gun range's armory: back to the
    /// range, not to the main page.
    fn leaves_armory(&self) -> bool {
        self.page == Page::Shop && self.shop.as_ref().is_some_and(|s| s.range || s.field)
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

    /// The cursor is over something a click works on (a button, a row, a
    /// card), not only a panel: the window lights its cursor.
    pub fn over_button(&self) -> bool {
        self.hover.and_then(|p| self.hit(p)).is_some_and(|h| h != Hit::Panel)
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
                    Hit::Line(_) => {
                        if let Some(k) = self.rows().iter().position(|&r| r == h) {
                            self.focus = k;
                        }
                        return Vec::new();
                    }
                    // A shop row clicked is picked: the pane tells it and
                    // its button (or Enter) buys it (Shop B, 11-15: a click
                    // bought at once, and the box telling what was bought
                    // had often scrolled away).
                    Hit::Buy(_) if self.page == Page::Shop => {
                        self.pick_row(h);
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
            KeyCode::Tab if self.page == Page::News => self.open_news_tab(self.news_tab + 1),
            // The shop's tabs: Tab goes round, 1 to 9 pick one.
            KeyCode::Tab if self.page == Page::Shop => self.open_shop_tab(self.shop_tab + 1),
            _ if self.page == Page::Shop && digit(code).is_some_and(|n| n >= 1 && n <= self.shop_tabs().len()) => {
                self.open_shop_tab(digit(code).unwrap_or(1) - 1);
            }
            // The shop's first arrow or Enter picks the row the keyboard is
            // on (the tab's first), rather than passing it or buying it
            // unseen.
            KeyCode::ArrowUp | KeyCode::ArrowDown | KeyCode::ArrowLeft | KeyCode::ArrowRight | KeyCode::KeyW | KeyCode::KeyS | KeyCode::KeyA | KeyCode::KeyD | KeyCode::Enter | KeyCode::NumpadEnter | KeyCode::Space
                if self.page == Page::Shop && !self.picked =>
            {
                self.picked = true;
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
        let inner = matches!(self.page, Page::Options | Page::Keys | Page::Missions | Page::News);
        // A shade down the left, fading out, so the panel reads over the
        // dish; the shop and the inner pages span the screen, so they dim
        // all of it.
        if self.page == Page::Shop || inner {
            // The shop darker (Look D, 11-20): its gun and bag rows sit on
            // the dimmed planet with no panel, and at 150 a crystal behind
            // them read brighter than an empty slot's text.
            let a = if self.page == Page::Shop { 215 } else { 150 };
            c.rrect(Rect::new(0.0, 0.0, sw, sh), 0.0, [4, 6, 8, a]);
        }
        let shade_w = (720.0 * s).min(sw);
        // (Forty thin steps: twelve showed as bands on the 1600-wide shot.)
        for k in 0..40 {
            let f = k as f32 / 40.0;
            c.rrect(Rect::new(0.0, 0.0, shade_w * (1.0 - f), sh), 0.0, [4, 6, 8, (9.0 * (1.0 - f)) as u8 + 2]);
        }
        // The main page's title and its foot hint over a dish that fills the
        // window (`menu_camera`, 11-20): a shade down from the top and up
        // from the foot, so a crystal under the title does not wash it out.
        if self.page == Page::Main {
            let (top_h, foot_h) = (130.0 * s, 60.0 * s);
            for k in 0..24 {
                let f = k as f32 / 24.0;
                let a = (7.0 * (1.0 - f)) as u8 + 1;
                c.rrect(Rect::new(0.0, 0.0, sw, top_h * (1.0 - f)), 0.0, [4, 6, 8, a]);
                c.rrect(Rect::new(0.0, sh - foot_h * (1.0 - f), sw, foot_h * (1.0 - f)), 0.0, [4, 6, 8, a]);
            }
        }
        // The shop and the gun range's armory: one screen of their own
        // (`shop.rs`, Shop B), never scrolled.
        if self.page == Page::Shop {
            let rows = self.rows();
            let focus = rows.get(self.focus.min(rows.len().saturating_sub(1))).copied();
            (self.scroll, self.scroll_max, self.body) = (0.0, 0.0, None);
            self.shop_page(c, s, focus);
            self.drawn = self.hits.clone();
            return;
        }
        let x = if inner { 40.0 * s } else { 48.0 * s };
        let w = (560.0 * s).min(sw - 2.0 * x);
        if self.page == Page::Main {
            // The title on the main page only, and smaller (44 at 96 until
            // 11-15, with the rows from 160: the list ran under the hint at
            // 200%; the owner picked menu A, names with one caption box).
            c.text(Weight::SemiBold, 36.0 * s, (x, 64.0 * s), Align::Left, &self.title, TEXT);
            c.text(Weight::Medium, 15.0 * s, (x, 90.0 * s), Align::Left, "A mould that grows, feeds and fights back", MUTED);
        }
        let top = if inner { HEAD * s + 16.0 * s } else { 100.0 * s };
        let rows = self.rows();
        let focus = rows.get(self.focus.min(rows.len().saturating_sub(1))).copied();
        // The rows the page scrolls in: under the title, over the hint.
        let foot = FOOT * s;
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
            Page::News if self.news_tab == 0 => self.news_page(c, x, at, s, focus),
            Page::News => self.try_page(c, x, at, s, focus),
            // (Its own frame, above.)
            Page::Shop => at,
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
        let hint = match self.page {
            Page::Main => "Up/Down and Enter, or click  ·  F1 or ? keys  ·  Options holds the key list",
            Page::Missions => "1-9 or Enter starts a mission  ·  Esc back",
            Page::Options => "Up/Down pick  ·  Left/Right change  ·  Backspace resets  ·  Tab next tab  ·  Esc back",
            Page::Keys if self.sheets > 1 && self.tabbed() => "Left/Right next page  ·  Tab next tab  ·  Esc or F1 back",
            Page::Keys if self.sheets > 1 => "Left/Right next page  ·  Esc or F1 back",
            Page::Keys if self.tabbed() => "Tab next tab  ·  Esc or F1 back",
            Page::Keys => "Esc or F1 back",
            Page::Shop => "",
            Page::News if self.news_tab == 0 => "Up/Down pick a line  ·  Tab things to try  ·  Esc back",
            Page::News => "Up/Down pick  ·  Enter or click ticks it  ·  Tab what is new  ·  Esc back",
        };
        let mut pager_w = if paged { 170.0 * s } else { 0.0 };
        // The main page's corner: which build this is, and left of it, in
        // amber, any experiment that is on, so a game that is not the
        // default one says so before a round starts (the HUD's mark says
        // it in play). On the hint's own line: a line over it met the
        // caption box of the last rows on a narrow screen.
        if let (Page::Main, Some(b)) = (self.page, &self.build) {
            let bw = c.text(Weight::Medium, 13.5 * s, (sw - x, sh - 28.0 * s), Align::Right, &b.long, MUTED);
            pager_w = bw + 24.0 * s;
            let on: Vec<&Opt> = self.options.iter().filter(|o| o.tab == "Experiments" && !o.is_default()).collect();
            let says = match on.as_slice() {
                [] => String::new(),
                [o] => format!("Experiment on: {}", o.label),
                all => format!("{} experiments on", all.len()),
            };
            if !says.is_empty() {
                pager_w += c.text(Weight::SemiBold, 13.5 * s, (sw - x - bw - 16.0 * s, sh - 28.0 * s), Align::Right, &says, WARN) + 16.0 * s;
            }
        }
        // On the main page a notice from the game (why the map editor did
        // not start) takes the hint's place, in full view.
        let (hint, ink) = match &self.notice {
            Some(n) if self.page == Page::Main => (n.as_str(), WARN),
            _ => (hint, MUTED),
        };
        c.text_fit(Weight::Medium, 13.5 * s, (x, sh - 28.0 * s), Align::Left, hint, sw - 2.0 * x - pager_w, ink);
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

    /// The main page (the owner's pick, 11-15 19:37Z, menu A of the
    /// research's blind pair): the ways to play, then Options and the map
    /// editor, then Quit, as names alone in one column, and one caption box
    /// beside the row under the cursor (else the keyboard's) saying what it
    /// is. Until then every row carried its line under its name, 66 units
    /// a row, and at 200% the list ran under the hint and scrolled.
    fn main_page(&mut self, c: &mut Canvas, x: f32, top: f32, _w: f32, s: f32, focus: Option<Hit>) -> f32 {
        // The rules changed, by their labels; the UI size and the volume
        // are not rules ("Free play, with ui 200%", the review 11-15).
        let changed: Vec<String> = self.options.iter().filter(|o| !o.is_default() && !matches!(o.key.as_str(), "ui" | "volume")).map(|o| format!("{} {}", o.label, o.value())).collect();
        let sandbox = if changed.is_empty() { "Free play on the map, with today's defaults".to_string() } else { format!("Free play, with {}", changed.join(", ")) };
        let campaign = format!("{} missions, one new verb each", self.missions.len());
        // The run first, and the one lit (the whole-game review, 11-15:
        // the run is the game; Sandbox led and was lit before). A gap
        // before Options and before Quit.
        let mut items: Vec<(&str, String, Hit, bool, bool)> = vec![
            (if self.run_live { "Back to the run" } else { "Run" }, if self.run_live { "The shop, then the next planet".into() } else { "Planet after planet toward the slime's home world; upgrades between".into() }, Hit::Run, true, false),
            ("Sandbox", sandbox, Hit::Sandbox, false, false),
            ("Campaign", campaign, Hit::Campaign, false, false),
        ];
        if self.planets {
            let depth = self.option("planet").map_or("1", |o| o.value());
            items.push(("Random planet", format!("A new generated planet, planet {depth} of a run (Options: Planet)"), Hit::RandomPlanet, false, false));
        }
        items.push(("Gun range", "Every gun, card and bullet, free: build a gun and shoot the slime".into(), Hit::Range, false, false));
        items.push(("Slime viewer", "Each slime and its events on demand; tune its looks and rules, and save them".into(), Hit::Viewer, false, false));
        // What the build holds, first of the rows that are not a way to
        // play: how much is new since the last build played here, and
        // what is left to try.
        let fresh = self.build.as_ref().map_or(0, BuildPage::fresh);
        if let Some(b) = &self.build {
            let sub: Vec<String> = [b.since(), b.left()].into_iter().filter(|t| !t.is_empty()).collect();
            items.push(("What is new", sub.join(". "), Hit::Build, false, true));
        }
        items.push(("Options", "The UI size, the game and the planet, the mould's rules, and the keys".into(), Hit::Options, false, self.build.is_none()));
        if self.edit_maps {
            items.push(("Edit maps", "Draw a map in the map editor".into(), Hit::EditMaps, false, false));
        }
        items.push(("Quit", "Close the game".into(), Hit::Quit, false, true));
        let longest = items.iter().map(|it| c.atlas.measure(Weight::SemiBold, 18.0 * s, it.0)).fold(0.0f32, f32::max);
        // (Room for the "What is new" row's count beside its name.)
        let w = (longest + 48.0 * s + if fresh > 0 { 70.0 * s } else { 0.0 }).max(280.0 * s);
        // Rows of 40 units, down to 34 where the page would otherwise run
        // under the hint: with the "What is new" row the page has nine,
        // and at 200% on 1080 lines nine rows of 40 scrolled (11-18). Down
        // to 30 since the slime viewer's row made ten (11-20: at 200% on a
        // 1280x540 window ten rows of 34 ran 28 units under the hint).
        let (gap, apart_by) = (4.0 * s, 12.0 * s);
        let aparts = items.iter().filter(|it| it.4).count() as f32;
        // (Half a pixel under the room: filled to the last bit, the sum
        // of the rows came out 0.00003 over and the page scrolled.)
        let room = c.screen.1 as f32 - (FOOT + 12.0) * s - (top + self.scroll + 8.0 * s) - 0.5;
        let rh = ((room - aparts * apart_by) / items.len() as f32 - gap).clamp(30.0 * s, 40.0 * s);
        let mut y = top + 8.0 * s;
        let mut placed = Vec::new();
        for (label, sub, hit, primary, apart) in items {
            if apart {
                y += apart_by;
            }
            let r = Rect::new(x, y, w, rh);
            self.item(r, label, "", hit, primary, focus == Some(hit), c, s);
            if hit == Hit::Build && fresh > 0 {
                // How many lines are new to this PC, at the row's right.
                let t = format!("{fresh} new");
                let bw = c.atlas.measure(Weight::SemiBold, 13.0 * s, &t) + 18.0 * s;
                let chip = Rect::new(r.right() - 12.0 * s - bw, r.y + (rh - 22.0 * s) / 2.0, bw, 22.0 * s);
                c.rrect(chip, 11.0 * s, with_alpha(ACCENT, 0.18));
                c.text(Weight::SemiBold, 13.0 * s, (chip.x + chip.w / 2.0, chip.y + 15.5 * s), Align::Centre, &t, ACCENT);
            }
            placed.push((r, label.to_string(), sub, hit));
            y += rh + gap;
        }
        // The row under the cursor, else the keyboard's.
        let hot = placed.iter().position(|p| self.hover.is_some_and(|h| p.0.contains(h)));
        let told = hot.or_else(|| placed.iter().position(|p| focus == Some(p.3))).map(|k| placed.swap_remove(k)).map(|(r, n, sub, _)| (r, n, sub));
        // (A notice, such as why the map editor did not start, takes the
        // hint's line: `draw`.)
        // The caption box beside the row it tells, with a notch toward it.
        if let Some((r, name, sub)) = told {
            let bx = r.right() + 18.0 * s;
            let bw = (380.0 * s).min(c.screen.0 as f32 - x - bx);
            let tw = bw - 32.0 * s;
            let lines = Self::count(c, Weight::Medium, 15.0 * s, tw, &sub);
            let bh = 48.0 * s + lines * 22.0 * s;
            let by = r.y.min(c.screen.1 as f32 - FOOT * s - 12.0 * s - bh);
            let b = Rect::new(bx, by, bw, bh);
            c.shadow(b, 10.0 * s, 18.0 * s, 4.0 * s, SHADOW);
            c.panel(b, 10.0 * s, PANEL_TOP, PANEL_BOTTOM, 1.0, EDGE);
            let mid = r.y + r.h / 2.0;
            for k in 0..6 {
                let d = 6.0 - k as f32;
                c.rrect(Rect::new(bx - d * s, mid - d * s, (d + 1.0) * s, 2.0 * d * s), 0.0, PANEL_TOP);
            }
            c.text(Weight::SemiBold, 18.0 * s, (bx + 16.0 * s, by + 30.0 * s), Align::Left, &name, TEXT);
            self.lines(c, Weight::Medium, 15.0 * s, (bx + 16.0 * s, by + 54.0 * s), tw, 22.0 * s, &sub, [201, 210, 219, 255]);
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
        // (The build page is named for the build: "Build 3127".)
        let build = self.build.as_ref().filter(|_| self.page == Page::News);
        let name = match self.page {
            Page::Missions => "Campaign".to_string(),
            Page::News => build.map_or(String::new(), |b| {
                let mut n = b.name.clone();
                if let Some(f) = n.get_mut(0..1) {
                    f.make_ascii_uppercase();
                }
                n
            }),
            _ if self.tabbed() => "Options".into(),
            _ => "Keys".into(),
        };
        // Its tabs count what is left to look at: the lines new to this
        // PC, the things not ticked.
        let counted = |t: &str, n: usize| if n > 0 { format!("{t} ({n})") } else { t.to_string() };
        let tabs: Vec<(String, Hit)> = match build {
            Some(b) => vec![(counted("What is new", b.fresh()), Hit::NewsTab(0)), (counted("Things to try", b.untried()), Hit::NewsTab(1))],
            None if self.tabbed() => self.tabs().into_iter().enumerate().map(|(k, t)| (t, Hit::Tab(k))).collect(),
            None => Vec::new(),
        };
        let tx = x + c.text(Weight::SemiBold, 22.0 * s, (x, 36.0 * s), Align::Left, &name, TEXT) + 24.0 * s;
        let back = Rect::new(sw - x - 116.0 * s, 10.0 * s, 116.0 * s, 36.0 * s);
        if !tabs.is_empty() {
            let lit = match self.page {
                Page::News => self.news_tab,
                Page::Keys => tabs.len() - 1,
                _ => self.opt_tab,
            };
            let mut tx = tx;
            for (k, (t, hit)) in tabs.iter().enumerate() {
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
                self.hits.push((r, *hit));
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
        if !opts.is_empty() && opts.iter().all(|&i| self.options[i].exp.is_some()) {
            return self.experiments_page(c, x, top, s, focus, &opts);
        }
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

    /// **The Experiments tab** (the Look and interface lane, 2026-11-19):
    /// the declared switches as cards (`ui::experiment_card`), each with
    /// its state, its choices as one button, its lane, the day it landed,
    /// when a pick applies in play, what you will see and why it is off,
    /// all whole; over them what an experiment is and the count against
    /// the plan's limit of three. The keyboard walks the cards as rows
    /// (Up/Down, Left/Right turns one, Backspace sets it back). Until
    /// 11-19 the tab was the other tabs' one-line rows with every reason
    /// run into the note pane, and no lane, date or count anywhere. A
    /// list that outgrows the screen is paged, never scrolled.
    fn experiments_page(&mut self, c: &mut Canvas, x: f32, top: f32, s: f32, focus: Option<Hit>, opts: &[usize]) -> f32 {
        let sw = c.screen.0 as f32;
        let w = (sw - 2.0 * x).min(1000.0 * s);
        let room = self.room(c, top, s);
        let cards: Vec<(usize, Experiment)> = opts.iter().map(|&i| (i, self.options[i].as_experiment())).collect();
        let pad = 16.0 * s;
        let inner = w - 2.0 * pad;
        let heights: Vec<f32> = cards.iter().map(|(_, e)| experiment_card_h(c, inner, e, s)).collect();
        // The head: what an experiment is, the count against the limit,
        // and every switch back to the default game; then as many cards a
        // page as the tallest lets fit. (A foot under the cards for the
        // reset left room for one card a page at 200% on 1080 lines.)
        let gap = 10.0 * s;
        let words = format!("{} of {} allowed  ·  {} on in this game", cards.len(), self.most_off, cards.iter().filter(|(_, e)| e.at != e.default).count());
        let mw = 26.0 * self.most_off.max(cards.len()) as f32 * s + 8.0 * s + c.atlas.measure(Weight::SemiBold, 14.0 * s, &words);
        let say = "At most three at once across the game. Each is settled once you have played it: on for everyone, one slime's trait, or deleted. A pick here lasts until the game closes";
        let sub = Self::count(c, Weight::Medium, 14.0 * s, w - 2.0 * pad - 8.0 * s, say);
        let head = 50.0 * s + sub * 20.0 * s + 8.0 * s;
        let tallest = heights.iter().cloned().fold(0.0, f32::max) + gap;
        let per = ((room - head - 2.0 * pad + gap) / tallest).floor().max(1.0) as usize;
        if let Some(k) = focus.and_then(|h| if let Hit::Reset(i) = h { opts.iter().position(|&o| o == i) } else { None }) {
            self.sheet = k / per;
        }
        let (shown, pages) = sheet_of(cards.len(), per, self.sheet);
        (self.sheets, self.per) = (pages, per);
        self.sheet = self.sheet.min(pages - 1);
        let body: f32 = heights[shown.clone()].iter().map(|h| h + gap).sum::<f32>() - gap;
        let panel = Rect::new(x, top, w, (head + body + 2.0 * pad).min(room.max(head)));
        self.glass(panel, c, s);
        let px = panel.x + pad + 4.0 * s;
        let all = Rect::new(panel.right() - pad - 170.0 * s, panel.y + 14.0 * s, 170.0 * s, 34.0 * s);
        self.small_button(all, "All to defaults", "", Hit::ResetAll, focus == Some(Hit::ResetAll), c, s);
        let on = cards.iter().filter(|(_, e)| e.at != e.default).count();
        let tw = c.text_fit(Weight::SemiBold, 19.0 * s, (px, panel.y + 38.0 * s), Align::Left, "Built, but not the default game yet", all.x - px - mw - 40.0 * s, TEXT);
        experiment_meter(c, ((all.x - 24.0 * s - mw).max(px + tw + 24.0 * s), panel.y + 37.0 * s), cards.len(), on, self.most_off, s);
        self.lines(c, Weight::Medium, 14.0 * s, (px, panel.y + 64.0 * s), w - 2.0 * pad - 8.0 * s, 20.0 * s, say, MUTED);
        let mut y = panel.y + pad + head;
        for (j, (i, e)) in cards[shown.clone()].iter().enumerate() {
            let h = heights[shown.start + j];
            let card = Rect::new(panel.x + pad, y, inner, h);
            let focused = focus == Some(Hit::Reset(*i));
            // The card takes the focus on a click; its buttons pick (they
            // are drawn after it, so they are on top).
            self.hits.push((card, Hit::Reset(*i)));
            for (b, k) in experiment_card(c, card, e, self.hover, focused, false, s) {
                self.hits.push((b, Hit::Pick(*i, k)));
            }
            y += h + gap;
        }
        panel.bottom()
    }

    /// **What is new** (the build page's first tab): the list's lines as
    /// one-line rows, the ones new to this PC first and marked, a page of
    /// them if they outgrow the screen, and beside them the line under the
    /// cursor (else the keyboard's, else the first) in full: what the
    /// player will see, how to try it, who wrote it and when. The same
    /// list-and-note shape as Options and the campaign, since a line is
    /// two or three sentences and a row has room for half of one.
    fn news_page(&mut self, c: &mut Canvas, x: f32, top: f32, s: f32, focus: Option<Hit>) -> f32 {
        let Some(b) = self.build.clone() else { return top };
        let n = b.news.len();
        let (lw, nw) = self.split(c, x, s);
        let (rh, cap) = (40.0 * s, 40.0 * s);
        let room = self.room(c, top, s);
        let per = ((room - cap - 12.0 * s) / rh).floor().max(1.0) as usize;
        if let Some(Hit::Line(i)) = focus {
            self.sheet = i / per;
        }
        let (shown, pages) = sheet_of(n, per, self.sheet);
        (self.sheets, self.per) = (pages, per);
        self.sheet = self.sheet.min(pages - 1);
        let panel = Rect::new(x, top, lw, cap + shown.len().max(1) as f32 * rh + 12.0 * s);
        self.glass(panel, c, s);
        // How much is new, and since which build.
        c.text_fit(Weight::SemiBold, 15.0 * s, (panel.x + 20.0 * s, panel.y + 28.0 * s), Align::Left, &b.since(), panel.w - 40.0 * s, if b.fresh() > 0 { ACCENT } else { MUTED });
        let chip_w = c.atlas.measure(Weight::SemiBold, 12.5 * s, "NEW") + 14.0 * s;
        let date_w = c.atlas.measure(Weight::Medium, 13.5 * s, "00-00") + 12.0 * s;
        let first = shown.start;
        let mut told = None;
        let mut y = panel.y + cap + 4.0 * s;
        for i in shown {
            let l = &b.news[i];
            let row = Rect::new(panel.x + 8.0 * s, y, panel.w - 16.0 * s, rh - 4.0 * s);
            let focused = focus == Some(Hit::Line(i));
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
            let base = row.y + row.h / 2.0 + 5.5 * s;
            let mut tx = row.x + 10.0 * s;
            if l.new {
                let chip = Rect::new(tx, row.y + (row.h - 20.0 * s) / 2.0, chip_w, 20.0 * s);
                c.rrect(chip, 10.0 * s, with_alpha(ACCENT, 0.18));
                c.text(Weight::SemiBold, 12.5 * s, (chip.x + chip.w / 2.0, chip.y + 14.5 * s), Align::Centre, "NEW", ACCENT);
            }
            // (The seen lines keep the column, so the dates line up.)
            tx += chip_w + 8.0 * s;
            c.text(Weight::Medium, 13.5 * s, (tx, base), Align::Left, l.date.get(5..).unwrap_or(&l.date), MUTED);
            tx += date_w;
            let (weight, ink) = if l.new { (Weight::SemiBold, TEXT) } else { (Weight::Medium, [201, 210, 219, 255]) };
            c.text_fit(weight, 15.5 * s, (tx, base), Align::Left, &l.sees, row.right() - 12.0 * s - tx, ink);
            self.hits.push((row, Hit::Line(i)));
            y += rh;
        }
        if n == 0 {
            c.text(Weight::Medium, 15.5 * s, (panel.x + 20.0 * s, panel.y + cap + 28.0 * s), Align::Left, "The list is empty", MUTED);
        }
        // The note. Its text shrinks a step or two before it is cut: the
        // longest lines at 200% on 1080 lines need it.
        let Some(l) = told.or((n > 0).then_some(first)).map(|i| &b.news[i]) else { return panel.bottom() };
        let tw = nw - 36.0 * s;
        let fixed = (34.0 + 30.0 + 10.0 + 46.0) * s + if l.try_it.is_empty() { 0.0 } else { 30.0 * s };
        let need = |c: &Canvas, k: f32| fixed + Self::count(c, Weight::Medium, 16.0 * k * s, tw, &l.sees) * 23.0 * k * s + Self::count(c, Weight::Medium, 15.0 * k * s, tw, &l.try_it) * 22.0 * k * s;
        let k = [1.0, 0.92, 0.84].into_iter().find(|&k| need(c, k) <= room).unwrap_or(0.84);
        let r = Rect::new(x + lw + 20.0 * s, top, nw, panel.h.max(260.0 * s).max(need(c, k)).min(room));
        self.glass(r, c, s);
        let px = r.x + 18.0 * s;
        let mut y = r.y + 34.0 * s;
        let (tag, ink) = if l.new { ("New since you last played", ACCENT) } else { ("On a build you have played", MUTED) };
        c.text(Weight::SemiBold, 13.5 * s, (px, y), Align::Left, tag, ink);
        y += 30.0 * s;
        // (Cut to the pane if the smallest size still runs over.)
        let foot = r.bottom() - 46.0 * s;
        let most = |y: f32, lead: f32| ((foot - y) / lead).floor().max(1.0) as usize + 1;
        y += self.lines_most(c, 16.0 * k * s, (px, y), tw, 23.0 * k * s, &l.sees, TEXT, most(y, 23.0 * k * s)) + 10.0 * s;
        if !l.try_it.is_empty() && y + 30.0 * s <= foot {
            c.text(Weight::SemiBold, 13.5 * s, (px, y), Align::Left, "How to try it", MUTED);
            y += 26.0 * s;
            self.lines_most(c, 15.0 * k * s, (px, y), tw, 22.0 * k * s, &l.try_it, [201, 210, 219, 255], most(y, 22.0 * k * s));
        }
        c.text_fit(Weight::Medium, 13.0 * s, (px, r.bottom() - 18.0 * s), Align::Left, &format!("{}  ·  {}", l.lane, l.date), tw, MUTED);
        panel.bottom().max(r.bottom())
    }

    /// Wrapped text of at most `most` lines from a first baseline at `at`,
    /// the last ending in an ellipsis when it was cut; the height it took.
    #[allow(clippy::too_many_arguments)]
    fn lines_most(&self, c: &mut Canvas, px: f32, at: (f32, f32), max: f32, lead: f32, text: &str, colour: Rgba, most: usize) -> f32 {
        if text.is_empty() {
            return 0.0;
        }
        let mut lines = c.wrap(Weight::Medium, px, max, text);
        if lines.len() > most {
            lines.truncate(most.max(1));
            if let Some(l) = lines.last_mut() {
                while !l.is_empty() && c.atlas.measure(Weight::Medium, px, &format!("{l}…")) > max {
                    l.pop();
                }
                l.push('…');
            }
        }
        for (k, l) in lines.iter().enumerate() {
            c.text(Weight::Medium, px, (at.0, at.1 + k as f32 * lead), Align::Left, l, colour);
        }
        lines.len() as f32 * lead
    }

    /// **The things to try** (the build page's second tab; the Direction
    /// lane's list, at most five, the most important first): each in
    /// full, wrapped, with a tick the game remembers on this PC. A click
    /// anywhere on a row, or Enter, ticks it. Rows are as tall as their
    /// text, so the list pages by height like the key list.
    fn try_page(&mut self, c: &mut Canvas, x: f32, top: f32, s: f32, focus: Option<Hit>) -> f32 {
        let Some(b) = self.build.clone() else { return top };
        let w = (c.screen.0 as f32 - 2.0 * x).min(920.0 * s);
        let room = self.room(c, top, s);
        let cap = 40.0 * s;
        let (px, lead) = (16.0 * s, 23.0 * s);
        // The text starts right of the tick box.
        let tx = 54.0 * s;
        let tw = w - 16.0 * s - tx - 18.0 * s;
        let heights: Vec<f32> = b.to_try.iter().map(|t| c.wrap(Weight::Medium, px, tw, &t.text).len() as f32 * lead + 20.0 * s).collect();
        let col_h = room - cap - 12.0 * s;
        let mut first = vec![0usize];
        let mut used = 0.0f32;
        for (i, h) in heights.iter().enumerate() {
            if used + h > col_h && used > 0.0 {
                first.push(i);
                used = 0.0;
            }
            used += h;
        }
        if let Some(Hit::Try(i)) = focus {
            self.sheet = first.iter().rposition(|&f| f <= i).unwrap_or(0);
        }
        self.sheets = first.len();
        self.sheet = self.sheet.min(self.sheets - 1);
        let (from, to) = (first[self.sheet], first.get(self.sheet + 1).copied().unwrap_or(heights.len()));
        self.try_first = first;
        let used: f32 = heights[from..to].iter().sum();
        let panel = Rect::new(x, top, w, (cap + used.max(40.0 * s) + 12.0 * s).min(room));
        self.glass(panel, c, s);
        let says = if b.to_try.is_empty() { "Nothing is on the list right now" } else { "The most important first. Tick what you have tried: the game remembers it on this PC" };
        c.text_fit(Weight::SemiBold, 15.0 * s, (panel.x + 20.0 * s, panel.y + 28.0 * s), Align::Left, says, panel.w - 40.0 * s, MUTED);
        let mut y = panel.y + cap + 4.0 * s;
        for (i, (t, &h)) in b.to_try.iter().zip(&heights).enumerate().take(to).skip(from) {
            let row = Rect::new(panel.x + 8.0 * s, y, panel.w - 16.0 * s, h - 4.0 * s);
            let focused = focus == Some(Hit::Try(i));
            let hot = self.hover.is_some_and(|p| row.contains(p));
            if focused || hot {
                c.rrect(row, 9.0 * s, [255, 255, 255, if focused { 22 } else { 12 }]);
            }
            if focused {
                c.outline(row, 9.0 * s, 1.5 * s, ACCENT);
            }
            let tick = Rect::new(row.x + 14.0 * s, row.y + 8.0 * s, 22.0 * s, 22.0 * s);
            if t.tried {
                c.rrect(tick, 6.0 * s, with_alpha(GOOD, 0.22));
                c.outline(tick, 6.0 * s, 1.5 * s, GOOD);
                c.icon(Icon::Check, (tick.x + 3.0 * s, tick.y + 3.0 * s), 16.0 * s, GOOD);
            } else {
                c.outline(tick, 6.0 * s, 1.5 * s, if hot || focused { TEXT } else { MUTED });
            }
            c.paragraph(Weight::Medium, px, (row.x + tx, row.y + 25.0 * s), tw, lead, &t.text, if t.tried { MUTED } else { TEXT });
            self.hits.push((row, Hit::Try(i)));
            y += h;
        }
        panel.bottom()
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

    /// The pages the last drawn list took: an Options tab, the key list or
    /// the campaign (1 elsewhere). The fit tests'.
    pub fn sheets(&self) -> usize {
        if matches!(self.page, Page::Options | Page::Keys | Page::Missions | Page::Shop | Page::News) {
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

    /// **The Experiments tab's cards** (2026-11-19): a tab whose options
    /// are all declared switches draws each whole (its state, lane, date,
    /// what you will see, why it is off) with the count against the
    /// limit; a click on a card's choice sets it, Left/Right still turn
    /// the keyboard's card, Backspace sets it back, and every word fits
    /// the screen at 100% and 200%, two switches on one page.
    #[test]
    fn the_experiments_tab_shows_each_switch_whole() {
        let info = |live| ExpInfo { lane: "Enemy".into(), since: "2026-11-11".into(), sees: "A crystal that is not being eaten slowly fills again".into(), why_off: "To be settled by play on a world that is mostly slime".into(), live };
        for (screen, ui) in [((1280, 800), 1.0), ((1920, 1080), 1.0), ((1920, 1080), 2.0)] {
            let mut m = menu();
            m.options.push(Opt::new("exp:feel", "Shake and hit-stop", &["off", "on"], "off", "on", "").in_tab("Experiments").experiment(ExpInfo { lane: "Squad and arsenal".into(), ..info(true) }));
            m.options.push(Opt::new("exp:crystal_regrow", "Crystals refill", &["off", "on"], "off", "off", "").in_tab("Experiments").experiment(info(false)));
            let mut c = Canvas::new(screen);
            c.ui.user = ui;
            m.show(Page::Options);
            while m.tabs()[m.opt_tab] != "Experiments" {
                m.key(KeyCode::Tab, true);
            }
            c.clear();
            m.draw(&mut c);
            let said = |c: &Canvas, t: &str| c.runs.iter().any(|r| r.text == t);
            for t in ["Built, but not the default game yet", "Shake and hit-stop", "Crystals refill", "ON", "OFF", "2 of 3 allowed  ·  1 on in this game", "YOU WILL SEE", "OFF BECAUSE", "All to defaults"] {
                assert!(said(&c, t), "no {t:?} at {screen:?} {ui}");
            }
            assert!(c.runs.iter().any(|r| r.text == "Enemy  ·  landed 2026-11-11  ·  in play, Esc changes it for the next round"), "at {screen:?} {ui}");
            assert_eq!(m.sheets, 1, "both on one page at {screen:?} {ui}");
            for r in &c.runs {
                assert!(r.x >= 0.0 && r.x + r.w <= screen.0 as f32 + 0.5 && r.base <= screen.1 as f32, "{:?} off the screen at {screen:?} {ui}", r.text);
            }
            // Crystals refill's "on", clicked.
            let i = m.options.iter().position(|o| o.key == "exp:crystal_regrow").unwrap();
            assert!(click(&mut m, &mut c, Hit::Pick(i, 1)).is_empty());
            assert_eq!(m.option("exp:crystal_regrow").map(|o| o.value()), Some("on"));
            assert_eq!(m.rows()[m.focus], Hit::Reset(i), "the card picked takes the focus");
            m.key(KeyCode::Backspace, true);
            assert_eq!(m.option("exp:crystal_regrow").map(|o| o.value()), Some("off"));
            m.key(KeyCode::ArrowRight, true);
            assert_eq!(m.option("exp:crystal_regrow").map(|o| o.value()), Some("on"));
        }
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

    /// A shop like the run's: `n` bullet cards (the first three in the
    /// gun, half in the bag, the rest on offer) beside eight Squad
    /// upgrades, seven Buildings, five Tools and three guns. Rows 0-7 are
    /// the Squad's, 8-14 Buildings, 15-19 Tools, 20-22 guns (20 carried),
    /// cards from 23.
    fn shop(n: usize) -> Shop {
        let mut rows = Vec::new();
        let mut add = |group: &str, area: ShopArea, k: usize| {
            let mut r = ShopRow::new(group, area, Icon::Wall, &format!("{group} {k}"));
            (r.about, r.price, r.state, r.ready, r.level) = ("Does it".into(), format!("{}", 100 + k), "0/3".into(), k.is_multiple_of(2), (1, 3));
            if area == ShopArea::Guns {
                r.bars = vec![("Range".into(), 0.5, "24".into()), ("Rate".into(), 0.3 + 0.1 * k as f32, "1.0".into()), ("Budget".into(), 0.4, "10".into()), ("Slots".into(), 0.6, "4".into())];
            }
            r.cost = k as u32 % 4;
            r.on = area == ShopArea::Cards && k < 3 || area == ShopArea::Guns && k == 0;
            rows.push(r);
        };
        for k in 0..8 {
            add("Squad", ShopArea::Upgrades, k);
        }
        for k in 0..7 {
            add("Buildings", ShopArea::Upgrades, k);
        }
        for k in 0..5 {
            add("Tools", ShopArea::Upgrades, k);
        }
        for k in 0..3 {
            add("Guns", ShopArea::Guns, k);
        }
        for k in 0..n {
            add(if k < n / 2 { BAG } else { "Cards" }, ShopArea::Cards, k);
        }
        Shop { heading: "Planet 2 next".into(), lines: vec!["1 planet freed".into()], purse: "200 points".into(), rows, slots: 4, budget: (6, 10), range: false, turrets: false, bench: String::new(), field: false }
    }

    /// The gun range's armory as the game fills it, with the longest
    /// names: eight guns, twenty bullets on a grid of four shapes by five
    /// payloads, fourteen modifiers on two shelves, `bag` cards in the bag,
    /// a six-slot gun with two in it, and the range's five tools.
    fn armory(bag: usize) -> Shop {
        let mut rows = Vec::new();
        for k in 0..8 {
            let mut r = ShopRow::new("Guns", ShopArea::Guns, Icon::Rifle, ["Rifle", "Carbine", "Long gun", "Scattergun"][k % 4]);
            r.on = k == 0;
            r.state = if k == 0 { "carried" } else if k < 4 { "armory" } else { "free" }.into();
            r.ready = k > 0;
            r.about = "Fast and short, a little wild. More slots. From planet 4.".into();
            r.bars = vec![("Range".into(), 0.5, "24".into()), ("Rate".into(), 0.1 * k as f32, "1.0/s".into()), ("Aim".into(), 0.9, "0.5".into()), ("Budget".into(), 0.4, "10".into()), ("Slots".into(), 0.6, "4".into())];
            if k % 4 == 3 {
                r.bars.insert(3, ("Shots".into(), 0.6, "4".into()));
            }
            rows.push(r);
        }
        let (sets, shells) = (["Standard", "Slug", "Needle", "Buckshot"], ["Bullet", "Blast", "Bomb", "Gas", "Germ"]);
        for (a, set) in sets.iter().enumerate() {
            for (b, shell) in shells.iter().enumerate() {
                let mut r = ShopRow::new("Bullets", ShopArea::Cards, Icon::Blast, &format!("{set} {}", shell.to_lowercase()));
                (r.ready, r.state, r.cost, r.badge) = (true, "free".into(), (a + b) as u32 % 6 + 1, Some(Icon::Slug));
                r.cell = Some(((a as u8, set.to_string()), (b as u8, shell.to_string())));
                r.about = "Puts slime where it hits to sleep. Needle: its own damage, speed, range and spread. Budget 3.".into();
                rows.push(r);
            }
        }
        let mods = ["Thrift", "Scorch", "Smother", "Wide", "Scatter", "Quiet", "Delay", "Thick", "Pierce", "Split", "Bait", "Faster rate", "Spread wide", "Double tap"];
        for (k, name) in mods.iter().enumerate() {
            let group = if k < 6 { "Cards for the next bullet" } else { "Cards for every bullet" };
            let mut r = ShopRow::new(group, ShopArea::Cards, Icon::Wide, name);
            (r.ready, r.state, r.cost) = (true, "free".into(), k as u32 % 3);
            rows.push(r);
        }
        for k in 0..bag + 2 {
            let mut r = ShopRow::new(BAG, ShopArea::Cards, Icon::Gas, &format!("Buckshot gas {k}"));
            (r.ready, r.on, r.state, r.cost) = (true, k < 2, if k < 2 { format!("slot {}", k + 1) } else { "in the bag".into() }, 2);
            rows.push(r);
        }
        for name in ["New rolls", "Deeper rolls", "Shallower rolls", "Empty the gun", "Empty the bag"] {
            let mut r = ShopRow::new("", ShopArea::Upgrades, Icon::Fast, name);
            (r.ready, r.about) = (true, "Roll the shelf's bullets and the rack's guns again.".into());
            rows.push(r);
        }
        let lines = vec!["Everything is free. Put cards in the gun: the squad fires them left to right, over and over.".into(), "Guns and bullets are rolled as on planet 3; the buttons along the bottom roll them again, deeper or shallower".into()];
        Shop { heading: "The gun range".into(), lines, purse: "Free".into(), rows, slots: 6, budget: (4, 10), range: true, turrets: false, bench: String::new(), field: false }
    }

    /// Shop B (owner's pick, 11-15 19:37Z): a click picks a row and the
    /// pane tells it; its button or Enter buys it. The tabs open by a
    /// click, Tab and their number; Land and Give up act at once; Esc goes
    /// back to the main page, on Run.
    #[test]
    fn the_shops_rows_pick_and_buy() {
        let mut m = menu();
        m.shop = Some(shop(8));
        m.show(Page::Shop);
        let mut c = Canvas::new((1280, 800));
        let names = |m: &StartMenu| m.shop_tabs().into_iter().map(|t| t.name).collect::<Vec<_>>();
        assert_eq!(names(&m), ["Cards", "Guns", "Squad", "Buildings", "Tools"]);
        // A card on offer (rows 27-30): a click picks it, the pane's button
        // buys it, and so does Enter.
        assert!(click(&mut m, &mut c, Hit::Buy(27)).is_empty(), "a click picks");
        assert!(m.picked && m.rows()[m.focus] == Hit::Buy(27));
        assert_eq!(click(&mut m, &mut c, Hit::Do(27)), vec![StartEvent::Buy(27)], "the pane's button buys");
        assert_eq!(m.key(KeyCode::Enter, true), vec![StartEvent::Buy(27)], "so does Enter");
        // The upgrades on their tabs.
        assert!(click(&mut m, &mut c, Hit::ShopTab(3)).is_empty());
        assert_eq!(names(&m)[m.shop_tab], "Buildings");
        c.clear();
        m.draw(&mut c);
        assert!(m.rect_of(Hit::Buy(0)).is_none() && m.rect_of(Hit::Buy(27)).is_none(), "the other tabs' rows are put away");
        assert!(m.rect_of(Hit::Buy(23)).is_some() && m.rect_of(Hit::Buy(26)).is_some(), "the gun and the bag stay");
        assert!(click(&mut m, &mut c, Hit::Buy(8)).is_empty());
        assert_eq!(click(&mut m, &mut c, Hit::Do(8)), vec![StartEvent::Buy(8)]);
        // Tab goes round; a number picks a tab, nothing picked on it.
        m.key(KeyCode::Tab, true);
        assert_eq!(names(&m)[m.shop_tab], "Tools");
        m.key(KeyCode::Digit2, true);
        assert_eq!((names(&m)[m.shop_tab].as_str(), m.picked), ("Guns", false));
        // The keyboard's first key picks the tab's first row (the carried
        // gun heads the table) rather than buying it unseen.
        assert!(m.key(KeyCode::Enter, true).is_empty(), "the first Enter picks");
        assert_eq!((m.rows()[m.focus], m.picked), (Hit::Buy(20), true));
        m.key(KeyCode::ArrowRight, true);
        assert_eq!(m.key(KeyCode::Enter, true), vec![StartEvent::Buy(21)]);
        // The carried gun's button does nothing; a gun card's takes it out.
        assert!(click(&mut m, &mut c, Hit::Buy(20)).is_empty());
        assert!(m.rect_of(Hit::Do(20)).is_none(), "the carried gun has no button");
        assert!(click(&mut m, &mut c, Hit::Buy(23)).is_empty());
        assert_eq!(StartMenu::button_of(&m.shop.as_ref().unwrap().rows[23]), ("Take it out of the gun".to_string(), true));
        assert_eq!(click(&mut m, &mut c, Hit::Do(23)), vec![StartEvent::Buy(23)]);
        assert_eq!(click(&mut m, &mut c, Hit::Next), vec![StartEvent::NextLevel]);
        assert_eq!(click(&mut m, &mut c, Hit::EndRun), vec![StartEvent::EndRun]);
        // A refused buy's notice shows in the shop and goes with it.
        m.notice = Some("Scorch: not enough points, or locked".into());
        m.key(KeyCode::Escape, true);
        assert_eq!((m.page, m.rows()[m.focus], m.notice.as_deref()), (Page::Main, Hit::Run, None));
    }

    /// The shop stays put when a card goes into the bag or comes out of
    /// it (owner, 11-15 02:58Z): the gun's slots, the bag, the tabs and
    /// the buttons keep their places at every screen and UI size.
    #[test]
    fn the_shop_holds_still_when_the_bag_fills() {
        let place = |bag: bool, screen: (u32, u32), user: f32| {
            let mut sh = shop(6);
            for r in sh.rows.iter_mut().filter(|r| r.area == ShopArea::Cards && !r.on) {
                r.group = if bag && r.name.ends_with('3') { BAG } else { "Cards" }.into();
            }
            let mut m = menu();
            m.shop = Some(sh);
            m.show(Page::Shop);
            let mut c = Canvas::new(screen);
            c.ui.user = user;
            m.draw(&mut c);
            let mut at: Vec<(Hit, Rect)> = m.hits.iter().filter(|(_, h)| matches!(h, Hit::Slot(_) | Hit::ShopTab(_) | Hit::Next | Hit::EndRun)).map(|&(r, h)| (h, r)).collect();
            at.extend(m.slot_rects.iter().map(|&r| (Hit::Next, r)));
            at.extend(m.bag.map(|r| (Hit::EndRun, r)));
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
    /// a right click; Leave the range is `EndRun`; its tools act at once.
    /// The main page's Gun range row opens it.
    #[test]
    fn the_gun_ranges_armory_goes_back_to_the_range() {
        let mut m = menu();
        let mut c = Canvas::new((1280, 800));
        assert_eq!(click(&mut m, &mut c, Hit::Range), vec![StartEvent::Range]);
        assert!(m.rows().iter().position(|&h| h == Hit::Range) < m.rows().iter().position(|&h| h == Hit::Options), "before Options");
        m.shop = Some(armory(3));
        m.show(Page::Shop);
        let names: Vec<String> = m.shop_tabs().into_iter().map(|t| t.name).collect();
        assert_eq!(names, ["Guns", "Bullets", "Cards for the next bullet", "Cards for every bullet"]);
        let tool = m.shop.as_ref().unwrap().rows.iter().position(|r| r.name == "Deeper rolls").unwrap();
        assert_eq!(click(&mut m, &mut c, Hit::Do(tool)), vec![StartEvent::Buy(tool)], "a tool acts at once");
        assert_eq!(click(&mut m, &mut c, Hit::Next), vec![StartEvent::NextLevel]);
        assert_eq!(click(&mut m, &mut c, Hit::EndRun), vec![StartEvent::EndRun]);
        assert_eq!(m.key(KeyCode::Escape, true), vec![StartEvent::NextLevel]);
        assert_eq!(m.button(Button::Right, true), vec![StartEvent::NextLevel]);
        // The bullets lie on their grid: a shape's row, a payload's column.
        m.open_shop_tab(1);
        c.clear();
        m.draw(&mut c);
        let at = |m: &StartMenu, name: &str| m.rect_of(Hit::Buy(m.shop.as_ref().unwrap().rows.iter().position(|r| r.name == name).unwrap())).unwrap();
        let (a, b, d) = (at(&m, "Standard bullet"), at(&m, "Standard germ"), at(&m, "Buckshot bullet"));
        assert!((a.y - b.y).abs() < 0.5 && b.x > a.x && (a.x - d.x).abs() < 0.5 && d.y > a.y, "a row a shape, a column a payload");
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

    /// A card dragged from the counter onto any slot of a six-slot gun, at
    /// 200% on a small screen, lands in that slot by its own number (the
    /// 11-15 review: when the shop scrolled, a drop could land in the wrong
    /// slot); one let go on the top bar does nothing.
    #[test]
    fn a_drop_lands_in_its_own_slot() {
        let mut m = menu();
        let mut sh = shop(14);
        sh.slots = 6;
        m.shop = Some(sh);
        m.show(Page::Shop);
        let mut c = Canvas::new((1280, 720));
        c.ui.user = 2.0;
        c.clear();
        m.draw(&mut c);
        assert!(!m.scrolls());
        let card = 30;
        assert!(m.rect_of(Hit::Buy(card)).is_some(), "a card on the counter");
        for k in 0..6 {
            let to = centre(m.slot_rects[k]);
            assert_eq!(drag(&mut m, &mut c, Hit::Buy(card), to), vec![StartEvent::Move(card, k)], "slot {k}");
        }
        assert!(drag(&mut m, &mut c, Hit::Buy(card), (300.0, 10.0)).is_empty(), "the top bar takes no drop");
    }

    /// The owner, 20:32Z: "When I buy a card it goes into the gun but I
    /// don't see how to move cards around." A card dragged onto a slot
    /// moves there (a bag card or one on offer goes in at that slot), onto
    /// the bag comes out, onto its own slot or nowhere does nothing; a
    /// short wobble is still a click, which picks it; Q and E move the
    /// focused card a slot and the focus follows.
    #[test]
    fn cards_drag_to_slots_and_q_and_e_move_them() {
        let mut m = menu();
        m.shop = Some(shop(8));
        m.show(Page::Shop);
        let mut c = Canvas::new((1600, 900));
        c.clear();
        m.draw(&mut c);
        // Rows 23-25 are in the gun (slots 1-3), slot 4 is empty, 26 in the
        // bag, 27-30 on offer.
        let (first, gun3) = (23, 25);
        assert_eq!((m.slot_of(gun3), m.slot_of(26)), ((Some(2), 3), (None, 3)));
        let slot: Vec<(f32, f32)> = m.slot_rects.iter().map(|&r| centre(r)).collect();
        assert_eq!(slot.len(), 4);
        assert_eq!(drag(&mut m, &mut c, Hit::Buy(first), slot[2]), vec![StartEvent::Move(first, 2)], "first to third");
        assert_eq!(drag(&mut m, &mut c, Hit::Buy(gun3), slot[0]), vec![StartEvent::Move(gun3, 0)], "third to first");
        assert!(drag(&mut m, &mut c, Hit::Buy(gun3), slot[2]).is_empty(), "its own slot");
        assert!(m.rect_of(Hit::Slot(3)).is_some(), "the empty slot is a target");
        assert_eq!(drag(&mut m, &mut c, Hit::Buy(26), slot[3]), vec![StartEvent::Move(26, 3)], "a bag card into the empty slot");
        assert_eq!(drag(&mut m, &mut c, Hit::Buy(27), slot[1]), vec![StartEvent::Move(27, 1)], "one on offer, bought into a slot");
        let bag = m.bag.unwrap();
        assert_eq!(drag(&mut m, &mut c, Hit::Buy(first), (bag.right() - 4.0, bag.y + 4.0)), vec![StartEvent::Buy(first)], "onto the bag: out");
        assert!(drag(&mut m, &mut c, Hit::Buy(26), centre(bag)).is_empty(), "a bag card onto the bag");
        assert!(drag(&mut m, &mut c, Hit::Buy(first), (5.0, 5.0)).is_empty(), "let go nowhere");
        m.open_shop_tab(1);
        assert!(drag(&mut m, &mut c, Hit::Buy(21), slot[0]).is_empty(), "guns do not drag");
        // A wobble under `DRAG_FROM` is a click: it picks the card.
        let r = m.rect_of(Hit::Buy(first)).unwrap();
        m.moved(Some(centre(r)));
        m.button(Button::Left, true);
        m.moved(Some((centre(r).0 + 3.0, centre(r).1)));
        assert!(m.button(Button::Left, false).is_empty());
        assert!(m.picked && m.rows()[m.focus] == Hit::Buy(first));
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

    /// The main page with every row (Run, Random planet, Edit maps, What
    /// is new: nine)
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
                // (Nine with "What is new", 11-18: the rows shrink to fit.)
                m.build = Some(build_page(6, 2));
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
        // Quit is still last, and Esc from the key card lands on Options
        // (whose last tab it is; the main page has no Keys row since 11-15).
        m.key(KeyCode::ArrowUp, true);
        assert_eq!(m.key(KeyCode::Enter, true), vec![StartEvent::Quit]);
        m.show(Page::Keys);
        m.key(KeyCode::Escape, true);
        assert_eq!(m.rows()[m.focus], Hit::Options);
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
        // The volume too (sound pass two, 2026-11-20).
        m.options.push(Opt::new("volume", "Volume", &["50%", "100%"], "100%", "50%", ""));
        let mut c = Canvas::new((1280, 800));
        m.show(Page::Options);
        click(&mut m, &mut c, Hit::Turn(1, 1));
        click(&mut m, &mut c, Hit::ResetAll);
        assert_eq!(m.changed("ui"), Some("200%"));
        assert_eq!(m.changed("volume"), Some("50%"));
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
            m.key(KeyCode::F1, true);
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
            for page in [Page::Main, Page::Missions, Page::Options, Page::Keys, Page::Shop, Page::News] {
                let size = |user: f32| {
                    let mut m = menu();
                    m.build = Some(build_page(12, 4));
                    m.keys = (0..40).map(|k| (format!("K{k}"), "Does a thing".to_string())).collect();
                    m.shop = Some(if page == Page::Shop { armory(9) } else { shop(14) });
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
                        // (The shop is one screen of its own, no body.)
                        let b = m.body.unwrap_or(Rect::new(0.0, 0.0, screen.0 as f32, screen.1 as f32));
                        // The shop's two buttons and the inner pages'
                        // header are fixed, over and under the body.
                        let fixed = matches!(h, Hit::Next | Hit::EndRun) || (matches!(page, Page::Options | Page::Keys | Page::Missions | Page::News) && h == Hit::Back);
                        assert!(r.y >= 0.0 && r.bottom() <= screen.1 as f32, "{screen:?} {page:?} at {user}: {h:?} {r:?} off the screen");
                        assert!(fixed || (r.y >= b.y - 0.5 && r.bottom() <= b.bottom() + 0.5), "{screen:?} {page:?} at {user}: {h:?} {r:?} not in view of {b:?}");
                    }
                    let probe = match page {
                        // (Land: the bottom bar's, whose height is fixed;
                        // the counter's rows shrink to fit before they page.)
                        Page::Shop => Hit::Next,
                        Page::Options => Hit::Reset(0),
                        Page::Missions => Hit::Mission(0),
                        Page::News => Hit::Line(0),
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
                assert!(b >= a * grew * 0.9, "{screen:?} {page:?}: 200% drew {b} against {a} at 100% (the size grew {grew}x)");
            }
        }
    }
    /// A build page like the game's: `n` lines of what is new, the first
    /// `fresh` new to this PC, every third as long as the game's longest
    /// (on 11-18 one ran to 310 characters with 150 on how to try it),
    /// and five things to try of two sentences.
    fn build_page(n: usize, fresh: usize) -> BuildPage {
        let long = "Two new bullets, Provoke and Still. Slime near a Provoke hit goes grey after about 5 seconds, lets go of what it holds and charges your guns. Slime near a Still hit is calm for 10 seconds, so a raid drifts past instead of turning on you, and a surge near it does not start until the calm wears off";
        let how = "Buy them in the run's shop from planet 2, or load them in the gun range's armory (Special bullets) and fire at a thick arm of slime and at a raid";
        let news = (0..n)
            .map(|i| NewsLine {
                date: format!("2026-11-{:02}", 18 - (i * 2 / n.max(1)).min(1)),
                lane: "Squad and arsenal".into(),
                sees: if i % 3 == 0 { format!("{i}: {long}") } else { format!("Line {i}: the lime wash on the ground round a firefight is gone") },
                try_it: if i % 2 == 0 { how.into() } else { String::new() },
                new: i < fresh,
            })
            .collect();
        let to_try = (0..5).map(|i| TryLine { text: format!("Thing {i}: drain crystals and buy soldiers from the first second. Does it still win, and what should stop it? Then build two pumps first instead: the slime now raids them about a minute in."), tried: i == 1 }).collect();
        BuildPage { name: "build 3127".into(), long: "build 3127 (d08583d, 2026-11-18)".into(), last: Some("build 3098, 2026-11-17".into()), news, to_try }
    }

    /// What the build holds (the plan of 2026-11-17: the menu shows the
    /// build, what is new since the last build played, and the things to
    /// try): a row on the main page that counts the new lines, the build
    /// in the corner, and a page of two tabs where the new lines are
    /// marked and a thing to try is ticked by a click or Enter.
    #[test]
    fn the_build_page_marks_what_is_new_and_ticks_what_was_tried() {
        let said = |c: &Canvas, t: &str| c.runs.iter().filter(|r| r.text == t).count();
        let mut m = menu();
        let mut c = Canvas::new((1280, 800));
        m.draw(&mut c);
        assert!(m.rect_of(Hit::Build).is_none() && said(&c, "What is new") == 0, "no row without a build page");
        m.build = Some(build_page(6, 2));
        c.clear();
        m.draw(&mut c);
        assert_eq!(m.rows(), [Hit::Run, Hit::Sandbox, Hit::Campaign, Hit::Range, Hit::Viewer, Hit::Build, Hit::Options, Hit::Quit]);
        assert_eq!((said(&c, "2 new"), said(&c, "build 3127 (d08583d, 2026-11-18)"), said(&c, "Experiment on: Shake")), (1, 1, 0));
        // An experiment that is on is named in the corner; two are counted.
        m.options.push(Opt::new("exp:feel", "Shake", &["off", "on"], "off", "on", "").in_tab("Experiments"));
        c.clear();
        m.draw(&mut c);
        assert_eq!(said(&c, "Experiment on: Shake"), 1);
        m.options.push(Opt::new("exp:crystal_regrow", "Crystals refill", &["off", "on"], "off", "on", "").in_tab("Experiments"));
        c.clear();
        m.draw(&mut c);
        assert_eq!(said(&c, "2 experiments on"), 1);
        // The row opens what is new: the new lines first and marked, the
        // first told in full beside the list.
        assert!(click(&mut m, &mut c, Hit::Build).is_empty());
        assert_eq!(m.page, Page::News);
        m.moved(None);
        c.clear();
        m.draw(&mut c);
        assert_eq!((said(&c, "NEW"), said(&c, "New since you last played"), said(&c, "On a build you have played")), (2, 1, 0));
        assert_eq!(said(&c, "2 changes since you last played (build 3098, 2026-11-17)"), 1);
        assert_eq!((said(&c, "What is new (2)"), said(&c, "Things to try (4)"), said(&c, "Build 3127")), (1, 1, 1));
        // A click on a line that was seen tells that one, and starts nothing.
        assert!(click(&mut m, &mut c, Hit::Line(4)).is_empty());
        assert_eq!(m.rows()[m.focus], Hit::Line(4));
        m.moved(None);
        c.clear();
        m.draw(&mut c);
        assert_eq!((said(&c, "New since you last played"), said(&c, "On a build you have played")), (0, 1));
        // Tab: the things to try. A click ticks one and the game is told;
        // Enter on the keyboard's row unticks one.
        m.key(KeyCode::Tab, true);
        assert_eq!(m.rows(), [Hit::Try(0), Hit::Try(1), Hit::Try(2), Hit::Try(3), Hit::Try(4), Hit::Back]);
        assert_eq!(click(&mut m, &mut c, Hit::Try(3)), vec![StartEvent::Tried(3, true)]);
        assert!(m.build.as_ref().unwrap().to_try[3].tried);
        m.key(KeyCode::ArrowUp, true);
        m.key(KeyCode::ArrowUp, true);
        assert_eq!(m.key(KeyCode::Enter, true), vec![StartEvent::Tried(1, false)]);
        c.clear();
        m.draw(&mut c);
        assert_eq!(said(&c, "Things to try (4)"), 1, "one ticked, one unticked: four are left");
        // Tab again goes back to what is new; Esc leaves for the main
        // page, on the row that led here.
        m.key(KeyCode::Tab, true);
        assert_eq!(m.rows()[0], Hit::Line(0));
        m.key(KeyCode::Escape, true);
        assert_eq!((m.page, m.rows()[m.focus]), (Page::Main, Hit::Build));
        // Nothing new: the row has no count and the page says so.
        m.build = Some(build_page(6, 0));
        c.clear();
        m.draw(&mut c);
        assert!(!c.runs.iter().any(|r| r.text.strip_suffix(" new").is_some_and(|n| n.parse::<u32>().is_ok())));
        m.show(Page::News);
        c.clear();
        m.draw(&mut c);
        assert_eq!((said(&c, "NEW"), said(&c, "Nothing new since you last played (build 3098, 2026-11-17)"), said(&c, "What is new")), (0, 1, 1));
        // A PC that has not played: all of it is new, and the page says why.
        let mut b = build_page(3, 3);
        b.last = None;
        m.build = Some(b);
        c.clear();
        m.draw(&mut c);
        assert_eq!((said(&c, "NEW"), said(&c, "All of it is new here: no round has been played on this PC yet")), (3, 1));
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
        let mut m = StartMenu::new("Larder World", missions, options, keys);
        // What the build holds, with more lines than a screen has rows at
        // the big sizes, and an experiment on (the corner names it).
        m.build = Some(build_page(24, 9));
        m.options.push(Opt::new("exp:feel", "Shake and hit-stop", &["off", "on"], "off", "on", "The camera shakes on blasts").in_tab("Experiments"));
        m
    }

    /// The owner, 11-15 17:31Z: "Having to scroll is horrible. That's not
    /// professional." The main page, every Options tab, every page of the
    /// key list and the campaign fit one screen at the worst sizes (16:9 and 21:9 at
    /// 200%, a small laptop at 175%, the window as it opens at 125%, and
    /// smaller than the game opens): nothing scrolls, every button and
    /// word is on the screen, no two words' glyphs touch, none under 12.5
    /// units, and the options' tabs and the campaign need no second page
    /// at any of them. The shop joins with its new layout.
    #[test]
    fn the_menu_pages_fit_one_screen() {
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
                assert!(matches!(m.page, Page::Main | Page::Shop) || m.hits.iter().any(|(_, h)| *h == Hit::Back), "{at}: no Back");
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
            for tab in 0..4 {
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
            m.open_tab(4);
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
            // The build page: what is new with every line focused in turn
            // (its page follows, the note tells it in full, the longest
            // shrunk to fit), then the things to try.
            for tab in 0..2 {
                m.open_news_tab(tab);
                for row in 0..m.rows().len() {
                    m.focus = row;
                    draw(&mut m, &mut c);
                    let h = m.rows()[row];
                    let at = format!("{screen:?} at {user}, build page tab {tab}, {h:?}");
                    check(&m, &c, &at);
                    assert!(m.rect_of(h).is_some(), "{at}: not drawn when focused");
                }
            }
            // The main page with every row it can have (Random planet, the
            // map editor, what is new) and a notice, each row focused in
            // turn, its caption box beside it.
            m.planets = true;
            m.edit_maps = true;
            m.notice = Some("The map editor did not start: larder-edit was not found next to the game".into());
            m.show(Page::Main);
            for row in 0..m.rows().len() {
                m.focus = row;
                draw(&mut m, &mut c);
                check(&m, &c, &format!("{screen:?} at {user}, main row {row}"));
            }
            // The run's shop (as a run has it, and with 30 cards, a full
            // bag that pages, and a six-slot gun) and the gun range's
            // armory (with a small bag and a full one): every tab with each
            // of its rows picked (the pane tells it), and on the first tab
            // the gun's, the bag's and the bottom bar's too.
            let mut big = shop(30);
            big.slots = 6;
            for (name, sh) in [("shop", shop(14)), ("full shop", big), ("armory", armory(3)), ("full armory", armory(14))] {
                m.shop = Some(sh);
                m.show(Page::Shop);
                draw(&mut m, &mut c);
                check(&m, &c, &format!("{screen:?} at {user}, {name}, nothing picked"));
                for tab in 0..m.shop_tabs().len() {
                    m.open_shop_tab(tab);
                    let n = if tab == 0 { m.rows().len() } else { m.shop_tabs()[tab].rows.len() };
                    for row in 0..n {
                        (m.focus, m.picked, m.follow) = (row, true, true);
                        draw(&mut m, &mut c);
                        let h = m.rows()[row];
                        let at = format!("{screen:?} at {user}, {name} tab {tab}, {h:?} picked");
                        check(&m, &c, &at);
                        assert!(m.rect_of(h).is_some(), "{at}: not drawn");
                    }
                }
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
