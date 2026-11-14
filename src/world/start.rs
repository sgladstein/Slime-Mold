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

use super::canvas::{Align, Canvas, Rect, Weight};
use super::controls::Button;
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

/// What the player picked.
#[derive(Clone, Debug, PartialEq)]
pub enum StartEvent {
    Sandbox,
    /// The index into `missions`.
    Mission(usize),
    Quit,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Page {
    Main,
    Missions,
    Options,
    Keys,
}

#[derive(Clone, Copy, Debug, PartialEq)]
enum Hit {
    Panel,
    Sandbox,
    Campaign,
    Options,
    Keys,
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
        StartMenu { title: title.into(), missions, options, keys, page: Page::Main, focus: 0, keys_from: Page::Main, hover: None, pressed: None, hits: Vec::new() }
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
    }

    /// The rows the keyboard walks on this page, top to bottom.
    fn rows(&self) -> Vec<Hit> {
        match self.page {
            Page::Main => vec![Hit::Sandbox, Hit::Campaign, Hit::Options, Hit::Keys, Hit::Quit],
            Page::Missions => (0..self.missions.len()).map(Hit::Mission).chain([Hit::Back]).collect(),
            Page::Options => (0..self.options.len()).map(Hit::Reset).chain([Hit::ResetAll, Hit::Back]).collect(),
            Page::Keys => vec![Hit::Back],
        }
    }

    fn back(&mut self) {
        let to = if self.page == Page::Keys { self.keys_from } else { Page::Main };
        let from = self.page;
        self.show(to);
        // Back on the main page, on the row that led away.
        if to == Page::Main {
            self.focus = match from {
                Page::Missions => 1,
                Page::Options => 2,
                Page::Keys => 3,
                Page::Main => 0,
            };
        }
    }

    /// What a hit does; a round picked comes back.
    fn act(&mut self, h: Hit) -> Option<StartEvent> {
        match h {
            Hit::Sandbox => return Some(StartEvent::Sandbox),
            Hit::Campaign => self.show(Page::Missions),
            Hit::Options => self.show(Page::Options),
            Hit::Keys => self.show(Page::Keys),
            Hit::Quit => return Some(StartEvent::Quit),
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
        let s = ((sh / 1080.0).clamp(0.7, 2.4) * 20.0).round() / 20.0;
        self.hits.clear();
        // A shade down the left, fading out, so the panel reads over the dish.
        let shade_w = (720.0 * s).min(sw);
        for k in 0..12 {
            let f = k as f32 / 12.0;
            c.rrect(Rect::new(0.0, 0.0, shade_w * (1.0 - f), sh), 0.0, [4, 6, 8, (24.0 * (1.0 - f)) as u8 + 6]);
        }
        let x = 48.0 * s;
        let w = (560.0 * s).min(sw - 2.0 * x);
        c.text(Weight::SemiBold, 44.0 * s, (x, 96.0 * s), Align::Left, &self.title, TEXT);
        let sub = match self.page {
            Page::Main => "A mould that grows, feeds and fights back",
            Page::Missions => "Campaign",
            Page::Options => "Options",
            Page::Keys => "Keys",
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
        }
        let hint = match self.page {
            Page::Main => "Up/Down and Enter, or click  ·  F1 or ? keys",
            Page::Missions => "1-9 or Enter starts a mission  ·  Esc back",
            Page::Options => "Left/Right change  ·  Backspace or click the row: default  ·  Esc back",
            Page::Keys => "Esc or F1 back",
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
        let items: [(&str, String, Hit, bool); 5] = [
            ("Sandbox", sandbox, Hit::Sandbox, true),
            ("Campaign", campaign, Hit::Campaign, false),
            ("Options", "The mould's and the squad's switches".into(), Hit::Options, false),
            ("Keys", "F1 or ? in the game too".into(), Hit::Keys, false),
            ("Quit", String::new(), Hit::Quit, false),
        ];
        let mut y = top + 12.0 * s;
        for (label, sub, hit, primary) in items {
            let h = if sub.is_empty() { 48.0 * s } else { 62.0 * s };
            self.item(Rect::new(x, y, w.min(420.0 * s), h), label, &sub, hit, primary, focus == Some(hit), c, s);
            y += h + 12.0 * s;
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
        let by = panel.bottom() + 16.0 * s;
        self.item(Rect::new(x, by, 140.0 * s, 42.0 * s), "Back", "", Hit::Back, true, focus == Some(Hit::Back), c, s);
        self.item(Rect::new(x + 152.0 * s, by, 200.0 * s, 42.0 * s), "All to defaults", "", Hit::ResetAll, false, focus == Some(Hit::ResetAll), c, s);
    }

    fn keys_page(&mut self, c: &mut Canvas, x: f32, top: f32, w: f32, s: f32) {
        let sh = c.screen.1 as f32;
        let rows = self.keys.clone();
        // Two columns when one does not fit.
        let lh = 22.0 * s;
        let room = sh - top - 130.0 * s;
        let per = ((room - 24.0 * s) / lh).floor().max(1.0) as usize;
        let cols = if rows.len() > per { 2 } else { 1 };
        let col_w = (w * 1.6).min(c.screen.0 as f32 - 2.0 * x) / cols as f32;
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
        assert!(m.key(KeyCode::Enter, true).is_empty());
        assert_eq!(m.page, Page::Missions);
        assert_eq!(m.key(KeyCode::Digit2, true), vec![StartEvent::Mission(1)]);
        assert!(m.key(KeyCode::Digit9, true).is_empty(), "no ninth mission");
        m.key(KeyCode::ArrowDown, true);
        assert_eq!(m.key(KeyCode::Enter, true), vec![StartEvent::Mission(1)]);
        m.key(KeyCode::Escape, true);
        assert_eq!((m.page, m.focus), (Page::Main, 1), "back on the campaign row");
        m.key(KeyCode::ArrowUp, true);
        m.key(KeyCode::ArrowUp, true);
        assert_eq!(m.key(KeyCode::Enter, true), vec![StartEvent::Quit], "up from the top wraps to quit");
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
