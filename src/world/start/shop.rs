//! The run's shop and the gun range's armory: one screen at every UI size,
//! never scrolled (the owner's pick, 11-15 19:37Z, "Shop B" of the blind
//! pair in `Reports/menu-layout-research-2026-11-15/`).
//!
//! A top bar says where the run is going and holds the points. Under it
//! the carried gun's slots and the bag stay pinned, so a card can always
//! be dragged into the gun. Then tabs over one counter: Cards, Guns,
//! Squad, Buildings and Tools in a run; Guns, Bullets and the two shelves
//! of modifiers in the gun range, whose bullets sit in a grid of shape by
//! payload. A pane at the right tells the picked thing in full and holds
//! its button (Buy, Put it in the gun, Take it out). A click picks; Enter
//! or the button buys; a drag from the counter or the bag onto a slot
//! buys and places in one move. The bottom bar holds Land and Give up
//! (the range: Back to the range, Leave, and the range's five tools).
//!
//! Until then the shop was one long page: the upgrades in a grid with
//! Buildings and Tools as folders, the armory beside it. At 200% it
//! scrolled, and the box describing a card had scrolled away by the time
//! the card was in view (the owner, 17:31Z: "When you click on any of the
//! bullets you can't read the descriptions ... Having to scroll is
//! horrible").

use super::*;

/// The shop's top and bottom bars, UI units.
const TOP: f32 = 46.0;
const BOTTOM: f32 = 62.0;
/// The bench's column of row names (Gun, Bag).
const LABEL: f32 = 46.0;
/// The fused shots' frames, shot by shot (cycled), and the every-shot
/// cards' (the owner's pick B, 2026-11-24).
const SHOT_INKS: [Rgba; 4] = [[96, 190, 255, 255], [255, 126, 182, 255], [182, 156, 255, 255], [150, 214, 120, 255]];
const EVERY_INK: Rgba = [63, 208, 191, 255];
/// The space between tiles.
const GAP: f32 = 8.0;
/// An upgrade tile: a little wider than tall, so a two-word name fits at
/// 12.5 units (the old grid's tiles were 78 square with names at 11.5).
const TILE_W: f32 = 100.0;
const TILE_H: f32 = 78.0;
/// The narrowest a bag card gets before the bag turns pages.
const CARD_MIN: f32 = 84.0;
/// The widest a card on the counter is stretched to.
const CARD_MAX: f32 = 210.0;
/// The gun table's rows: as tall as this, down to the least, then pages.
const ROW: f32 = 30.0;
const ROW_LEAST: f32 = 22.0;
/// The pager's strip under a paged counter.
const PAGER: f32 = 36.0;
/// A shelf row on the counter (Look D, 2026-11-20): its height, and the
/// least width of a column of them.
const SHELF_H: f32 = 74.0;
/// The tallest a shelf row grows when the counter has room (`shelf`).
const SHELF_TALL: f32 = 112.0;
const SHELF_W: f32 = 420.0;
/// The hand of cards (`hand`): at most this many offers, each card at
/// least `HAND_LEAST` and at most `HAND_W` wide, `HAND_TALL` times as
/// tall as wide, and the least height that shows a card whole.
const HAND_MOST: usize = 6;
const HAND_LEAST: f32 = 170.0;
const HAND_W: f32 = 250.0;
const HAND_TALL: f32 = 1.15;
const HAND_H: f32 = 230.0;
/// How tall a card grows with room (`hand`): at most this many times its
/// width, and this share of the counter's height.
const HAND_STRETCH: f32 = 1.7;
const HAND_FILL: f32 = 0.74;

/// What a shop tab holds, so how its counter is laid out.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub(super) enum TabKind {
    /// Bullet cards, as tiles (in a grid of shape by payload when every
    /// card has its cell: the gun range's bullets).
    Cards,
    /// Guns, as a table against the carried one.
    Guns,
    /// Upgrades, as the command card's tiles.
    Tiles,
}

/// A tab of the shop's counter.
#[derive(Clone, Debug, PartialEq)]
pub(super) struct ShopTab {
    pub name: String,
    pub kind: TabKind,
    /// Its shop rows, in the order they show.
    pub rows: Vec<usize>,
}

/// A row bought out or at its last level.
fn owned(r: &ShopRow) -> bool {
    r.level.1 > 0 && r.level.0 >= r.level.1
}

/// Wrapped text from a first baseline at `at`, stopping before a line
/// would pass `limit`; the height it took.
#[allow(clippy::too_many_arguments)]
fn para(c: &mut Canvas, weight: Weight, px: f32, at: (f32, f32), max: f32, lead: f32, text: &str, colour: Rgba, limit: f32) -> f32 {
    let mut y = at.1;
    for l in c.wrap(weight, px, max, text) {
        if y + px * 0.3 > limit {
            break;
        }
        c.text(weight, px, (at.0, y), Align::Left, &l, colour);
        y += lead;
    }
    y - at.1
}

/// Worse than yours, in the gun table's and the pane's red.
const WORSE: Rgba = [240, 128, 112, 255];

/// An estimated damage a second as the shop writes it: whole numbers, one
/// decimal under 10 so the rifle's 10 and a 9.6 read apart.
fn dps_text(v: f32) -> String {
    if v < 9.95 { format!("{v:.1}") } else { format!("{v:.0}") }
}

/// The change a pick makes to the gun's estimate, as the shop words it:
/// "+4 DPS" green, "-2 DPS" red, "DPS same" faint; None with nothing to
/// say. The bool: no change.
fn dps_words(now: Option<f32>, after: Option<f32>, no_room: bool) -> Option<(String, Rgba, bool)> {
    let (a, b) = (now?, after?);
    let d = b - a;
    // Under a twentieth of a point, or a twentieth of a percent: the same.
    let (t, ink, same) = if d.abs() < 0.05_f32.max(0.0005 * a) {
        ("DPS same".to_string(), FAINT, true)
    } else if d > 0.0 {
        (format!("+{} DPS", dps_text(d)), GOOD, false)
    } else {
        (format!("-{} DPS", dps_text(-d)), WORSE, false)
    };
    // With no room in the gun it goes to the bag: what it would do there,
    // said as such, in a quieter ink.
    if no_room {
        return Some((format!("{t} \u{b7} no room"), if same { FAINT } else { with_alpha(ink, 0.7) }, same));
    }
    Some((t, ink, same))
}

/// `dps_words` as a pill whose right edge is at `right` and whose text
/// sits on `base`. Returns its width (0: nothing to say).
#[allow(clippy::too_many_arguments)]
fn dps_chip(c: &mut Canvas, now: Option<f32>, after: Option<f32>, no_room: bool, right: f32, base: f32, px: f32, s: f32, dim: f32) -> f32 {
    let Some((text, ink, same)) = dps_words(now, after, no_room) else { return 0.0 };
    let w = c.atlas.measure(Weight::SemiBold, px, &text) + 12.0 * s;
    let h = px + 8.0 * s;
    let r = Rect::new(right - w, base - px * 0.78 - 4.0 * s, w, h);
    c.rrect(r, h / 2.0, with_alpha(ink, if same { 0.10 } else { 0.16 } * dim));
    c.text(Weight::SemiBold, px, (r.x + 6.0 * s, base), Align::Left, &text, with_alpha(ink, dim));
    w
}

/// A row's words as the shop shows them: a rolled bullet whose numbers are
/// drawn as bars (`ShopRow::bars`) loses the sentence that lists them
/// ("Bullet: damage 6.4, speed 1.8x, ..."), which the bars say.
fn words(row: &ShopRow) -> &str {
    let a = row.about.trim();
    if row.area == ShopArea::Cards && !row.bars.is_empty() && a.starts_with("Bullet: damage") {
        return a.split_once(". ").map_or(a, |x| x.1).trim();
    }
    a
}

/// Stat bars (the owner, 2026-11-22: "more of visual representation of
/// damage speed, range, ect. instead of a paragraph of numbers"): a row a
/// stat, its name, a track filled to its 0-1 in `tint`, its value at the
/// right; with `mine`, the carried gun's place on the same stat as a white
/// tick, and the fill green where this one is better, red where worse.
/// Rows `lead` apart from the first's top at `y`; returns the height used,
/// stopping before `limit`.
#[allow(clippy::too_many_arguments)]
fn stat_bars(c: &mut Canvas, bars: &[(String, f32, String)], mine: Option<&[(String, f32, String)]>, x: f32, y: f32, w: f32, lead: f32, px: f32, tint: Rgba, dim: f32, limit: f32, s: f32) -> f32 {
    let label = bars.iter().map(|b| c.atlas.measure(Weight::Medium, px, &b.0)).fold(0.0f32, f32::max) + 8.0 * s;
    let value = bars.iter().map(|b| c.atlas.measure(Weight::SemiBold, px, &b.2)).fold(0.0f32, f32::max) + 8.0 * s;
    let track = (w - label - value).max(20.0 * s);
    let th = (px * 0.5).max(4.0 * s);
    let mut at = y;
    for (l, f, v) in bars {
        if at + lead > limit + 2.0 * s {
            break;
        }
        let base = at + lead / 2.0 + px * 0.36;
        c.text(Weight::Medium, px, (x, base), Align::Left, l, with_alpha(MUTED, dim));
        let t = Rect::new(x + label, at + (lead - th) / 2.0, track, th);
        c.rrect(t, th / 2.0, [255, 255, 255, 22]);
        let theirs = mine.and_then(|m| m.iter().find(|b| b.0 == *l)).map(|b| b.1);
        let ink = match theirs {
            Some(m) if *f > m + 0.005 => GOOD,
            Some(m) if *f < m - 0.005 => WORSE,
            _ => tint,
        };
        let fill = f.clamp(0.0, 1.0) * track;
        if fill > 0.5 {
            c.rrect(Rect::new(t.x, t.y, fill.max(th), th), th / 2.0, with_alpha(ink, 0.9 * dim));
        }
        if let Some(m) = theirs {
            let mx = t.x + m.clamp(0.0, 1.0) * track;
            c.rrect(Rect::new(mx - 1.0 * s, t.y - 3.0 * s, 2.0 * s, th + 6.0 * s), 1.0 * s, with_alpha(TEXT, 0.85 * dim));
        }
        c.text(Weight::SemiBold, px, (x + w, base), Align::Right, v, with_alpha(if theirs.is_some() { ink } else { TEXT }, dim));
        at += lead;
    }
    at - y
}

/// The counter as shelves (Look D, 2026-11-20): every offer a row with its
/// one-sentence text, in as many columns as fit, when all of them fit on
/// one page; None leaves the tiles, which page.
fn shelf(n: usize, area: Rect, g: f32, s: f32) -> Option<(usize, f32, f32)> {
    if n == 0 {
        return None;
    }
    let cols = (((area.w + g) / (SHELF_W * s + g)).floor() as usize).clamp(1, 3);
    let rows = ((area.h + g) / (SHELF_H * s + g)).floor() as usize;
    let cols = cols.min(n);
    if rows * cols < n {
        return None;
    }
    let w = (area.w - (cols - 1) as f32 * g) / cols as f32;
    // Taller rows while the counter has the room (Look D, 11-20): five
    // offers at 74 left three quarters of the counter empty. Up to
    // `SHELF_TALL`, which shows the whole of what an offer does.
    let used = n.div_ceil(cols) as f32;
    let h = ((area.h - (used - 1.0) * g) / used).clamp(SHELF_H * s, SHELF_TALL * s);
    Some((cols, w, h))
}

/// A short counter laid out as a hand of cards (Look D, 11-21): up to six
/// offers, one row of tall cards, when the counter is tall enough for
/// them. Shelf rows at their tallest left half the counter empty with five
/// offers (filmed 11-21 at 1600 x 972: Cards, Buildings and Tools each
/// three rows of rows and the rest dark). Returns the row's left and top
/// edges, a card's width and height; None where the shelf or tiles take it.
fn hand(n: usize, area: Rect, g: f32, s: f32) -> Option<(f32, f32, f32, f32)> {
    if n == 0 || n > HAND_MOST {
        return None;
    }
    let w = ((area.w - (n - 1) as f32 * g) / n as f32).min(HAND_W * s);
    // Taller where the counter has the room (polish review finding 8,
    // 11-24: five cards at 1.15 left about 45% of the counter blank above
    // and below them), up to `HAND_STRETCH` times as tall as wide and
    // `HAND_FILL` of the counter; `hand_card` spreads its face over it.
    let h = (w * HAND_TALL).max((area.h * HAND_FILL).min(w * HAND_STRETCH)).min(area.h);
    if w < HAND_LEAST * s || h < HAND_H * s {
        return None;
    }
    let used = n as f32 * w + (n - 1) as f32 * g;
    // Centred on the counter both ways, a little above its middle: laid
    // at the top, the row left the counter's lower half dark and read
    // unfinished; laid in the middle, as a hand on a table, it reads as
    // the choice.
    Some((area.x + (area.w - used) / 2.0, area.y + 0.4 * (area.h - h), w, h))
}

/// Draws the planet ahead (`Glance`) in `r`, with a line under it naming
/// the marks; returns the height used. The ground in its coarse cells, a
/// crystal as a cyan hexagon its size (at least a few pixels), each start
/// of the slime as an amber blot with a glow, the landing as a white ring.
fn glance(c: &mut Canvas, g: &Glance, r: Rect, s: f32) -> f32 {
    draw_glance(c, g, r, s)
}

/// `glance` for the other pages (the campaign's note, 11-24).
pub(super) fn draw_glance(c: &mut Canvas, g: &Glance, r: Rect, s: f32) -> f32 {
    c.shadow(r, 6.0 * s, 10.0 * s, 2.0 * s, [0, 0, 0, 120]);
    // Bilinear between the cells' centres (polish review finding 8,
    // 11-24: the 64-cell mosaic read as blocks): columns a quarter cell
    // wide (at least 1.5 UI pixels), each sampled at its middle, and down
    // each column one box a span between two rows of centres, whose
    // vertical gradient is the blend down it. Until 11-24 it was 2 x 2
    // flat quarters a cell, 23,000 shapes a frame on a 96-cell map and
    // tile lists ~170 deep under it (Look, the 4K count for Engine); now
    // half the shapes, and smooth down each column where quarters
    // stepped. (Columns a cell wide read as vertical stripes.) A hair
    // over each box, so no seam shows.
    let cols = ((r.w / (1.5 * s)).round() as usize).clamp(8, 2 * g.w.max(1));
    let cw = r.w / cols as f32;
    let cell = |i: isize, j: isize| g.cells[(j.clamp(0, g.h as isize - 1) as usize) * g.w + i.clamp(0, g.w as isize - 1) as usize];
    // The map's colour at (x, row of centres j): bilinear along x only.
    let at_row = |fx: f32, j: isize| {
        let x0 = fx.floor();
        let t = fx - x0;
        let (a, b) = (cell(x0 as isize, j), cell(x0 as isize + 1, j));
        let mut q = [0u8, 0, 0, 255];
        for k in 0..3 {
            q[k] = (a[k] as f32 * (1.0 - t) + b[k] as f32 * t).round() as u8;
        }
        q
    };
    let ch = r.h / g.h as f32;
    for i in 0..cols {
        let fx = (i as f32 + 0.5) * g.w as f32 / cols as f32 - 0.5;
        let x = r.x + i as f32 * cw;
        // Spans: the half-cell above the first centre, h - 1 between
        // centres, the half-cell below the last.
        for j in -1..g.h as isize {
            let y0 = r.y + ((j as f32 + 0.5) * ch).max(0.0);
            let y1 = r.y + ((j as f32 + 1.5) * ch).min(r.h);
            c.panel(Rect::new(x, y0, cw + 0.6, y1 - y0 + 0.6), 0.0, at_row(fx, j), at_row(fx, j + 1), 0.0, [0, 0, 0, 0]);
        }
    }
    c.outline(r, 4.0 * s, 1.0 * s, [255, 255, 255, 40]);
    let at = |x: f32, y: f32| (r.x + x * r.w, r.y + y * r.h);
    // The slime starts on its home crystal (the level generator puts a
    // nest on a rich crystal): that crystal drawn in the slime's amber in
    // a glow, the rest cyan.
    for &(x, y, cr) in &g.crystals {
        let (px, py) = at(x, y);
        let d = (2.0 * cr * r.w).max(7.0 * s);
        let home = g.moulds.iter().any(|&(mx, my)| ((mx - x) * r.w).hypot((my - y) * r.h) <= cr * r.w + 1.0);
        if home {
            let b = d * 0.9;
            c.rrect(Rect::new(px - b, py - b, 2.0 * b, 2.0 * b), b, [240, 150, 40, 70]);
        }
        c.icon(Icon::Crystal, (px - d / 2.0, py - d / 2.0), d, if home { [246, 176, 66, 255] } else { [92, 214, 240, 255] });
    }
    // A nest off every crystal (none on today's planets) as a blot.
    for &(x, y) in &g.moulds {
        if g.crystals.iter().any(|&(cx, cy, cr)| ((cx - x) * r.w).hypot((cy - y) * r.h) <= cr * r.w + 1.0) {
            continue;
        }
        let (px, py) = at(x, y);
        let b = 6.0 * s;
        c.rrect(Rect::new(px - 2.0 * b, py - 2.0 * b, 4.0 * b, 4.0 * b), 2.0 * b, [240, 150, 40, 60]);
        c.rrect(Rect::new(px - b, py - b, 2.0 * b, 2.0 * b), b, [244, 170, 60, 235]);
    }
    if let Some((x, y)) = g.landing {
        let (px, py) = at(x, y);
        let b = 8.0 * s;
        c.outline(Rect::new(px - b, py - b, 2.0 * b, 2.0 * b), b, 2.0 * s, [240, 244, 248, 240]);
    }
    let key = r.bottom() + 18.0 * s;
    c.text(Weight::Medium, 12.5 * s, (r.x, key), Align::Left, "Crystals cyan, the slime's home amber, you land in the ring", MUTED);
    r.h + 34.0 * s
}

/// How `n` boxes at least `w` wide and `h` tall sit in `area`: the boxes a
/// row, their width (stretched to the area, up to `max_w`), and a page's
/// worth and the pages, a strip left for the pager when it pages.
fn flow(n: usize, area: Rect, w: f32, max_w: f32, h: f32, g: f32, pager: f32) -> (usize, f32, usize, usize) {
    let per_row = ((area.w + g) / (w + g)).floor().max(1.0) as usize;
    let w = ((area.w - (per_row - 1) as f32 * g) / per_row as f32).min(max_w);
    let rows = |room: f32| ((room + g) / (h + g)).floor().max(1.0) as usize;
    let mut per = per_row * rows(area.h);
    if n > per {
        per = per_row * rows(area.h - pager);
    }
    (per_row, w, per, n.div_ceil(per).max(1))
}

impl StartMenu {
    /// The shop's tabs, in the order they show: a run's cards on offer
    /// first, then its guns and upgrades; the gun range's guns first, then
    /// its shelves.
    pub(super) fn shop_tabs(&self) -> Vec<ShopTab> {
        let Some(shop) = &self.shop else { return Vec::new() };
        fn put(v: &mut Vec<ShopTab>, kind: TabKind, name: &str, i: usize) {
            match v.iter_mut().find(|t| t.name == name) {
                Some(t) => t.rows.push(i),
                None => v.push(ShopTab { name: name.into(), kind, rows: vec![i] }),
            }
        }
        let (mut cards, mut guns, mut tiles) = (Vec::new(), Vec::new(), Vec::new());
        for (i, r) in shop.rows.iter().enumerate() {
            match r.area {
                ShopArea::Cards if r.on || r.group == BAG => {}
                ShopArea::Cards => put(&mut cards, TabKind::Cards, &r.group, i),
                ShopArea::Guns => put(&mut guns, TabKind::Guns, "Guns", i),
                ShopArea::Upgrades if r.group.is_empty() => {}
                ShopArea::Upgrades => put(&mut tiles, TabKind::Tiles, &r.group, i),
            }
        }
        // The carried gun heads its table.
        for t in &mut guns {
            t.rows.sort_by_key(|&i| !shop.rows[i].on);
        }
        let mut v = if shop.range { [guns, cards].concat() } else { [cards, guns].concat() };
        v.extend(tiles);
        v
    }

    /// The shop's rows that match `f`, in order.
    fn shop_where(&self, f: impl Fn(&ShopRow) -> bool) -> Vec<usize> {
        self.shop.iter().flat_map(|s| s.rows.iter().enumerate()).filter(|(_, r)| f(r)).map(|(i, _)| i).collect()
    }

    /// The cards in the carried gun, in firing order.
    fn gun_cards(&self) -> Vec<usize> {
        self.shop_where(|r| r.area == ShopArea::Cards && r.on)
    }

    fn bag_cards(&self) -> Vec<usize> {
        self.shop_where(|r| r.area == ShopArea::Cards && !r.on && r.group == BAG)
    }

    /// The gun range's own buttons (an upgrade with no tab).
    fn tools(&self) -> Vec<usize> {
        self.shop_where(|r| r.area == ShopArea::Upgrades && r.group.is_empty())
    }

    /// What the keyboard walks: the tab showing, the gun's cards, the bag,
    /// the range's tools, then Land and Give up. (The tab's first, so the
    /// focus a tab opens on is its first item.)
    pub(super) fn shop_rows(&self) -> Vec<Hit> {
        if self.shop.is_none() {
            return vec![Hit::Next, Hit::EndRun];
        }
        let tabs = self.shop_tabs();
        let mut v: Vec<Hit> = tabs.get(self.shop_tab.min(tabs.len().saturating_sub(1))).map_or(Vec::new(), |t| t.rows.iter().map(|&i| Hit::Buy(i)).collect());
        v.extend(self.gun_cards().into_iter().map(Hit::Buy));
        v.extend(self.bag_cards().into_iter().map(Hit::Buy));
        v.extend(self.tools().into_iter().map(Hit::Do));
        v.push(Hit::Next);
        if !self.shop.as_ref().is_some_and(|s| s.field) {
            v.push(Hit::EndRun);
        }
        v
    }

    /// Shows the shop's tab `k` (wrapping), nothing picked.
    pub(super) fn open_shop_tab(&mut self, k: usize) {
        let n = self.shop_tabs().len();
        if n > 0 {
            self.shop_tab = k % n;
            self.focus = 0;
            self.sheet = 0;
            self.picked = false;
        }
    }

    /// Shows the shop's tab named `name`, or the one holding a row so
    /// named (the pictures and the drag films, `menu=shop-build`); false
    /// if there is none.
    pub fn open_shop_tab_named(&mut self, name: &str) -> bool {
        let tabs = self.shop_tabs();
        let rows = self.shop.as_ref().map(|s| &s.rows);
        let k = tabs.iter().position(|t| t.name == name).or_else(|| tabs.iter().position(|t| t.rows.iter().any(|&i| rows.is_some_and(|r| r[i].name == name))));
        if let Some(k) = k {
            self.open_shop_tab(k);
        }
        k.is_some()
    }

    /// Picks the row named `name` the keyboard can reach (the tab showing
    /// first, then the gun and the bag), as a click on it does (the
    /// pictures); false if there is none.
    pub fn pick_named(&mut self, name: &str) -> bool {
        let rows = self.shop.as_ref().map(|s| &s.rows);
        let h = self.rows().into_iter().find(|h| matches!(h, Hit::Buy(i) | Hit::Do(i) if rows.is_some_and(|r| r[*i].name == name)));
        if let Some(h) = h {
            self.pick_row(h);
        }
        h.is_some()
    }

    /// The keyboard (or a click) picks `h`: the pane tells it.
    pub(super) fn pick_row(&mut self, h: Hit) {
        if let Some(k) = self.rows().iter().position(|&r| r == h) {
            self.focus = k;
            self.picked = true;
        }
    }

    /// The centre of the first card out of the gun named `name`, as last
    /// drawn (the counter's tab, the bag); else of the last gun so named
    /// (the one on sale, or the range's rack, after those owned).
    pub fn card_at(&self, name: &str) -> Option<(f32, f32)> {
        let shop = self.shop.as_ref()?;
        let drawn = |i: &usize| self.rect_of(Hit::Buy(*i)).is_some();
        let card = shop.rows.iter().enumerate().filter(|(i, r)| r.area == ShopArea::Cards && !r.on && r.name == name && drawn(i)).map(|(i, _)| i).next();
        let gun = shop.rows.iter().enumerate().filter(|(i, r)| r.area == ShopArea::Guns && r.name == name && drawn(i)).map(|(i, _)| i).next_back();
        self.rect_of(Hit::Buy(card.or(gun)?)).map(|r| (r.x + r.w / 2.0, r.y + r.h / 2.0))
    }

    /// The shop, drawn whole on one screen.
    pub(super) fn shop_page(&mut self, c: &mut Canvas, s: f32, focus: Option<Hit>) {
        let Some(shop) = self.shop.clone() else { return };
        let (sw, sh) = (c.screen.0 as f32, c.screen.1 as f32);
        let (x, g) = (24.0 * s, GAP * s);
        let tabs = self.shop_tabs();
        self.shop_tab = self.shop_tab.min(tabs.len().saturating_sub(1));
        let tab = tabs.get(self.shop_tab).cloned();
        // The keyboard's ring and the pane follow what is picked; until the
        // player picks something the pane tells the planet ahead.
        let picked = focus.filter(|_| self.picked);
        let ring = move |h: Hit| picked == Some(h);
        let hover = self.hover;
        let hot = |r: Rect| hover.is_some_and(|p| r.contains(p));
        let drag = self.drag.filter(|&i| i < shop.rows.len());

        // An opaque floor under the whole shop (polish review finding 8,
        // 11-24: the planet under the menu showed through, a crystal's
        // hexagon behind the bag row). Mid-planet the loadout panel keeps
        // the planet faintly behind it.
        if !shop.field {
            c.rrect(Rect::new(0.0, 0.0, sw, sh), 0.0, [11, 14, 18, 255]);
        }
        // The top bar: where the run is going, and the points.
        let top = TOP * s;
        c.rrect(Rect::new(0.0, 0.0, sw, top), 0.0, [8, 11, 14, 236]);
        c.rrect(Rect::new(0.0, top - s.max(1.0), sw, s.max(1.0)), 0.0, EDGE);
        let purse_w = c.text(Weight::SemiBold, 20.0 * s, (sw - x, 30.0 * s), Align::Right, &shop.purse, ACCENT);
        // The run's planets as a strip before the points (Look D, 11-24),
        // where the heading keeps its whole width beside it.
        let mut room = sw - 2.0 * x - purse_w - 24.0 * s;
        let mut tip = None;
        if let Some(run) = shop.run.as_ref().filter(|r| !r.stops.is_empty()) {
            let sw_ = (run.stops.len() as f32 * 26.0 * s).min(420.0 * s);
            let head = c.atlas.measure(Weight::SemiBold, 18.0 * s, &shop.heading);
            if head <= room - sw_ - 28.0 * s {
                let r = Rect::new(sw - x - purse_w - 28.0 * s - sw_, 6.0 * s, sw_, top - 10.0 * s);
                tip = super::run_strip(c, &run.stops, r, hover, s);
                room -= sw_ + 28.0 * s;
            }
        }
        c.text_fit(Weight::SemiBold, 18.0 * s, (x, 30.0 * s), Align::Left, &shop.heading, room, TEXT);

        // The bottom bar: on (Land; the range's round), out (Give up; leave
        // the range), the range's tools, and the hint or a notice.
        let by = sh - BOTTOM * s;
        c.rrect(Rect::new(0.0, by, sw, sh - by), 0.0, [8, 11, 14, 236]);
        c.rrect(Rect::new(0.0, by, sw, s.max(1.0)), 0.0, EDGE);
        let (next, end) = if shop.range { ("Back to the range", "Leave the range") } else if shop.field { ("Back to the planet", "") } else { ("Land on the next planet", "Give up the run") };
        let bh = 40.0 * s;
        let bt = by + (BOTTOM * s - bh) / 2.0;
        let wn = c.atlas.measure(Weight::SemiBold, 18.0 * s, next) + 36.0 * s;
        let we = c.atlas.measure(Weight::SemiBold, 18.0 * s, end) + 36.0 * s;
        self.item(Rect::new(x, bt, wn, bh), next, "", Hit::Next, true, ring(Hit::Next), c, s);
        // The loadout panel on a planet has no way out but back.
        let we = if shop.field { -10.0 * s } else { we };
        if !shop.field {
            self.item(Rect::new(x + wn + 10.0 * s, bt, we, bh), end, "", Hit::EndRun, false, ring(Hit::EndRun), c, s);
        }
        let mut bx = x + wn + 10.0 * s + we + 20.0 * s;
        let tools = self.tools();
        if !tools.is_empty() {
            // As big as fits, down to 12.5 units.
            let room = sw - x - bx;
            let width = |px: f32, pad: f32| tools.iter().map(|&i| c.atlas.measure(Weight::SemiBold, px, &shop.rows[i].name) + 2.0 * pad).sum::<f32>() + (tools.len() - 1) as f32 * 6.0 * s;
            let (px, pad) = if width(14.5 * s, 12.0 * s) <= room { (14.5 * s, 12.0 * s) } else { (12.5 * s, 9.0 * s) };
            let th = 34.0 * s;
            for &i in &tools {
                let name = &shop.rows[i].name;
                let r = Rect::new(bx, by + (BOTTOM * s - th) / 2.0, c.atlas.measure(Weight::SemiBold, px, name) + 2.0 * pad, th);
                let lit = hot(r) || ring(Hit::Do(i));
                c.panel(r, 7.0 * s, if lit { BUTTON_HOVER_TOP } else { BUTTON_TOP }, if lit { BUTTON_HOVER_BOTTOM } else { BUTTON_BOTTOM }, 1.0, [255, 255, 255, 30]);
                if ring(Hit::Do(i)) {
                    c.outline(r.inset(-2.0 * s), 9.0 * s, 1.5 * s, ACCENT);
                }
                // The one on (the loadout the gun slots show) reads as on.
                let on = shop.rows[i].on;
                if on {
                    c.outline(r, 7.0 * s, 1.5 * s, ACCENT);
                }
                c.text(Weight::SemiBold, px, (r.x + pad, r.y + r.h / 2.0 + px * 0.36), Align::Left, name, if on { ACCENT } else { TEXT });
                self.hits.push((r, Hit::Do(i)));
                bx = r.right() + 6.0 * s;
            }
            bx += 14.0 * s;
        }
        // The hint whole, or a shorter one, or none (never cut short).
        let hints: [&str; 2] = if shop.range {
            ["Click picks  ·  Enter takes it  ·  drag a card into the gun  ·  Tab next tab  ·  Esc back to the range", "Click picks  ·  Enter takes it  ·  Esc back"]
        } else {
            ["Click picks  ·  Enter buys  ·  drag a card into the gun  ·  Tab next tab  ·  Esc back", "Click picks  ·  Enter buys  ·  Esc back"]
        };
        let room = sw - x - bx;
        let base = by + BOTTOM * s / 2.0 + 5.0 * s;
        match &self.notice {
            // A buy the game refused says why, in the hint's place.
            Some(n) if room > 120.0 * s => {
                c.text_fit(Weight::SemiBold, 14.0 * s, (sw - x, base), Align::Right, n, room, WARN);
            }
            None => {
                if let Some(h) = hints.iter().find(|h| c.atlas.measure(Weight::Medium, 13.5 * s, h) <= room) {
                    c.text(Weight::Medium, 13.5 * s, (sw - x, base), Align::Right, h, MUTED);
                }
            }
            _ => {}
        }

        // The pane at the right, the rest at the left.
        let pw = ((sw / s) * 0.31).clamp(300.0, 420.0) * s;
        let pane = Rect::new(sw - x - pw, top + 10.0 * s, pw, by - 10.0 * s - (top + 10.0 * s));
        let lw = pane.x - 16.0 * s - x;

        // The bench: the carried gun's slots and the bag, pinned.
        let gun_name = if !shop.bench.is_empty() {
            shop.bench.as_str()
        } else if shop.turrets {
            "Every turret"
        } else {
            shop.rows.iter().find(|r| r.area == ShopArea::Guns && r.on).map_or("Your gun", |r| r.name.as_str())
        };
        let gy = top + 30.0 * s;
        let nw = c.text(Weight::SemiBold, 16.5 * s, (x, gy), Align::Left, gun_name, TEXT);
        let on = self.gun_cards();
        let line = format!("budget {} of {}  ·  {} of {} slots  ·  fired left to right", shop.budget.0, shop.budget.1, on.len(), shop.slots);
        // Its estimated damage a second at the line's right end (the
        // owner, 2026-11-22): the number large, what it is small before it.
        let mut dps_w = 0.0;
        if let Some(d) = shop.dps {
            let v = dps_text(d);
            let vw = c.text(Weight::SemiBold, 15.0 * s, (x + lw, gy), Align::Right, &v, TEXT);
            let lab = if shop.turrets { "est. DPS a turret" } else { "est. DPS a soldier" };
            let w2 = c.text(Weight::Medium, 12.5 * s, (x + lw - vw - 7.0 * s, gy), Align::Right, lab, MUTED);
            dps_w = vw + 7.0 * s + w2 + 16.0 * s;
        }
        c.text_fit(Weight::Medium, 13.5 * s, (x + nw + 12.0 * s, gy), Align::Left, &line, lw - nw - 12.0 * s - dps_w, MUTED);
        let ch = CARD_H * s;
        let (sx, sroom) = (x + LABEL * s, lw - LABEL * s);
        // (Room above the row for the shots' names.)
        let row_y = top + 54.0 * s;
        c.text(Weight::SemiBold, 13.5 * s, (x, row_y + ch / 2.0 + 5.0 * s), Align::Left, if shop.turrets { "Turrets" } else { "Gun" }, MUTED);
        let n = (shop.slots as usize).max(on.len()).max(1);
        // Fused shots (the owner's pick, 2026-11-24 15:51Z: "B is best";
        // the six options are `Reports/card-links-2026-11-24/`): the cards
        // of one shot touch inside one frame and shots stand apart, so you
        // see which bullet a Wide or a Trigger works on. The gun's own
        // plain bullet that leftover cards go on, which no card stands
        // for, is a ghost card after them. The slots keep their order (the
        // mock-up moved the every-shot cards to the end; here they stay
        // put, framed where they sit, so a drag lands where it looks).
        let fused = !on.is_empty() && shop.shots.len() == on.len();
        let tag = |k: usize| if fused { shop.shots.get(k).copied() } else { None };
        let ghost_shot = shop.plain_shot.filter(|_| fused);
        let ghost_after = ghost_shot.and_then(|p| (0..on.len()).rev().find(|&k| tag(k) == Some(ShotTag::Fired(p))));
        // The row's places: Some(slot), or None for the ghost.
        let mut places: Vec<Option<usize>> = Vec::with_capacity(n + 1);
        for k in 0..n {
            places.push(Some(k));
            if ghost_after == Some(k) {
                places.push(None);
            }
        }
        let group = |p: Option<usize>| match p {
            Some(k) => tag(k).filter(|_| k < on.len()),
            None => ghost_shot.map(ShotTag::Fired),
        };
        let (tight, apart) = (2.0 * s, 14.0 * s);
        let gap_at = |i: usize| -> f32 {
            match (group(places[i - 1]), group(places[i])) {
                (Some(a), Some(b)) if a == b => tight,
                (Some(_), _) | (_, Some(_)) if fused => apart,
                _ => g,
            }
        };
        let gaps: f32 = (1..places.len()).map(gap_at).sum();
        let cw = ((sroom - gaps) / places.len() as f32).min(CARD_W * s);
        let mut px = sx;
        let mut place_rects = Vec::with_capacity(places.len());
        for i in 0..places.len() {
            if i > 0 {
                px += gap_at(i);
            }
            place_rects.push(Rect::new(px, row_y, cw, ch));
            px += cw;
        }
        let slots: Vec<(Rect, Option<usize>)> = places.iter().zip(&place_rects).filter_map(|(p, &r)| p.map(|k| (r, on.get(k).copied()))).collect();
        self.slot_rects = slots.iter().map(|x| x.0).collect();
        // Each run of places in one shot: a frame round them, its name
        // over it.
        if fused {
            let mut i = 0;
            while i < places.len() {
                let Some(t) = group(places[i]) else {
                    i += 1;
                    continue;
                };
                let mut j = i;
                while j + 1 < places.len() && group(places[j + 1]) == Some(t) {
                    j += 1;
                }
                let (a, b) = (place_rects[i], place_rects[j]);
                let f = Rect::new(a.x - 3.0 * s, a.y - 3.0 * s, b.right() - a.x + 6.0 * s, ch + 6.0 * s);
                let (ink, name) = match t {
                    ShotTag::Fired(k) => (SHOT_INKS[k as usize % SHOT_INKS.len()], format!("Shot {}", k + 1)),
                    ShotTag::Every => (EVERY_INK, "Every shot".to_string()),
                    ShotTag::Idle => (FAINT, "Does nothing".to_string()),
                };
                c.rrect(f, 10.0 * s, with_alpha(ink, 0.08));
                c.outline(f, 10.0 * s, 1.5 * s, with_alpha(ink, 0.8));
                c.text_fit(Weight::SemiBold, 11.5 * s, (f.x + 6.0 * s, f.y - 4.0 * s), Align::Left, &name, f.w - 6.0 * s, ink);
                i = j + 1;
            }
            // The ghost: the gun's plain bullet, which the leftovers ride.
            if let Some((r, p)) = places.iter().zip(&place_rects).find(|(p, _)| p.is_none()).map(|(_, &r)| r).zip(ghost_shot) {
                let ink = SHOT_INKS[p as usize % SHOT_INKS.len()];
                c.rrect(r, 8.0 * s, [255, 255, 255, 5]);
                c.outline(r, 8.0 * s, 1.0, with_alpha(ink, 0.45));
                c.text_fit(Weight::SemiBold, 12.5 * s, (r.x + r.w / 2.0, r.y + r.h / 2.0 - 1.0 * s), Align::Centre, "Plain bullet", r.w - 10.0 * s, MUTED);
                c.text_fit(Weight::Medium, 11.0 * s, (r.x + r.w / 2.0, r.y + r.h / 2.0 + 13.0 * s), Align::Centre, "the gun's own", r.w - 10.0 * s, FAINT);
            }
        }
        let mut from = None;
        for (k, &(r, i)) in slots.iter().enumerate() {
            match i {
                Some(i) => {
                    self.card_tile(&shop.rows[i], r, hot(r) && drag.is_none(), ring(Hit::Buy(i)), Some(k), true, c, s);
                    self.hits.push((r, Hit::Buy(i)));
                    if drag == Some(i) {
                        from = Some(r);
                    }
                }
                None => {
                    c.rrect(r, 8.0 * s, [255, 255, 255, 6]);
                    c.outline(r, 8.0 * s, 1.0, [255, 255, 255, 30]);
                    c.text(Weight::SemiBold, 12.5 * s, (r.x + 7.0 * s, r.y + 18.0 * s), Align::Left, &format!("{}", k + 1), with_alpha(TEXT, 0.5));
                    c.text(Weight::Medium, 13.0 * s, (r.x + r.w / 2.0, r.y + r.h / 2.0 + 9.0 * s), Align::Centre, "empty", FAINT);
                    self.hits.push((r, Hit::Slot(k)));
                }
            }
        }
        // The bag under it: cards at their full width with empty places
        // after them; more cards narrower, then pages.
        let bag_y = row_y + ch + 8.0 * s;
        c.text(Weight::SemiBold, 13.5 * s, (x, bag_y + ch / 2.0 + 5.0 * s), Align::Left, "Bag", MUTED);
        let bag = self.bag_cards();
        let cells = ((sroom + g) / (CARD_W * s + g)).floor().max(1.0) as usize;
        let (bw, per, places) = if bag.len() <= cells {
            (CARD_W * s, cells, cells)
        } else {
            let w = (sroom - (bag.len() - 1) as f32 * g) / bag.len() as f32;
            if w >= CARD_MIN * s {
                (w, bag.len(), bag.len())
            } else {
                let per = ((sroom - 70.0 * s + g) / (CARD_MIN * s + g)).floor().max(1.0) as usize;
                (CARD_MIN * s, per, per)
            }
        };
        let pages = bag.len().div_ceil(per).max(1);
        // The keyboard on a card in the bag: its page.
        if std::mem::take(&mut self.follow) {
            if let Some(k) = focus.and_then(|h| bag.iter().position(|&b| Hit::Buy(b) == h)) {
                self.bag_sheet = k / per;
            }
        }
        self.bag_sheet = self.bag_sheet.min(pages - 1);
        let shown = &bag[(self.bag_sheet * per).min(bag.len())..((self.bag_sheet + 1) * per).min(bag.len())];
        // An empty bag is one quiet place that says what it is for, not a
        // row of empty outlines (Look D, 11-20).
        if bag.is_empty() {
            let r = Rect::new(sx, bag_y, sroom, ch);
            c.rrect(r, 8.0 * s, [255, 255, 255, 6]);
            c.outline(r, 8.0 * s, 1.0, [255, 255, 255, 24]);
            let say = if shop.range { "Cards off the gun wait here" } else { "Cards with no room on the gun wait here" };
            c.text_fit(Weight::Medium, 13.0 * s, (r.x + r.w / 2.0, r.y + r.h / 2.0 + 5.0 * s), Align::Centre, say, r.w - 24.0 * s, FAINT);
        }
        for k in 0..if bag.is_empty() { 0 } else { places } {
            let r = Rect::new(sx + k as f32 * (bw + g), bag_y, bw, ch);
            match shown.get(k) {
                Some(&i) => {
                    self.card_tile(&shop.rows[i], r, hot(r) && drag.is_none(), ring(Hit::Buy(i)), None, true, c, s);
                    self.hits.push((r, Hit::Buy(i)));
                    if drag == Some(i) {
                        from = Some(r);
                    }
                }
                None => c.outline(r, 8.0 * s, 1.0, [255, 255, 255, 18]),
            }
        }
        if pages > 1 {
            // "<" and ">" at the row's end.
            for (k, (t, by_)) in [("<", -1isize), (">", 1)].into_iter().enumerate() {
                let r = Rect::new(x + lw - (2 - k) as f32 * 32.0 * s + 2.0 * s, bag_y, 30.0 * s, ch);
                c.panel(r, 7.0 * s, if hot(r) { BUTTON_HOVER_TOP } else { BUTTON_TOP }, if hot(r) { BUTTON_HOVER_BOTTOM } else { BUTTON_BOTTOM }, 1.0, [255, 255, 255, 30]);
                c.text(Weight::SemiBold, 15.0 * s, (r.x + r.w / 2.0, r.y + r.h / 2.0 + 5.5 * s), Align::Centre, t, TEXT);
                self.hits.push((r, Hit::BagFlip(by_)));
            }
        }
        let bag_rect = Rect::new(x, bag_y, lw, ch);
        self.bag = Some(bag_rect);

        // The tabs: the name and how many are on offer there, a dot where
        // something can be bought now; as big as fits, down to 12.5 units.
        let ty = bag_y + ch + 12.0 * s;
        let th = 34.0 * s;
        let count = |t: &ShopTab| t.rows.iter().filter(|&&i| !shop.rows[i].on && !owned(&shop.rows[i])).count();
        let dot = |t: &ShopTab| !shop.range && t.rows.iter().any(|&i| shop.rows[i].ready && !shop.rows[i].price.is_empty());
        let size = |c: &Canvas, px: f32, counts: bool| -> Vec<f32> {
            // (Room for two digits and the dot whether shown or not, so a
            // buy that changes them moves no tab: the owner, 11-15 02:58Z,
            // "everything shifts in the shop".)
            let digits = |t: &ShopTab| c.atlas.measure(Weight::Medium, px, &count(t).to_string()).max(c.atlas.measure(Weight::Medium, px, "88"));
            tabs.iter().map(|t| c.atlas.measure(Weight::SemiBold, px, &t.name) + if counts { 8.0 * s + digits(t) } else { 0.0 } + if shop.range { 0.0 } else { 12.0 * s } + 24.0 * s).collect()
        };
        let fits = |w: &[f32]| w.iter().sum::<f32>() + w.len().saturating_sub(1) as f32 * 6.0 * s <= lw;
        let (px, counts) = [(15.0, true), (14.0, true), (13.0, true), (12.5, true), (12.5, false)].into_iter().map(|(p, k)| (p * s, k)).find(|&(p, k)| fits(&size(c, p, k))).unwrap_or((12.5 * s, false));
        let mut tx = x;
        for (k, (t, w)) in tabs.iter().zip(size(c, px, counts)).enumerate() {
            let r = Rect::new(tx, ty, w, th);
            let lit = k == self.shop_tab;
            let (t0, t1) = match (lit, hot(r)) {
                (true, _) => ([40, 70, 98, 255], [30, 54, 78, 255]),
                (false, true) => (BUTTON_HOVER_TOP, BUTTON_HOVER_BOTTOM),
                (false, false) => (BUTTON_TOP, BUTTON_BOTTOM),
            };
            c.panel(r, 8.0 * s, t0, t1, 1.0, if lit { ACCENT } else { [255, 255, 255, 30] });
            let base = r.y + r.h / 2.0 + px * 0.36;
            let mut at = r.x + 12.0 * s + c.text(Weight::SemiBold, px, (r.x + 12.0 * s, base), Align::Left, &t.name, if lit { TEXT } else { MUTED });
            if counts {
                c.text(Weight::Medium, px, (at + 8.0 * s, base), Align::Left, &count(t).to_string(), if lit { ACCENT } else { FAINT });
                at += 8.0 * s + c.atlas.measure(Weight::Medium, px, &count(t).to_string()).max(c.atlas.measure(Weight::Medium, px, "88"));
            }
            if dot(t) {
                c.rrect(Rect::new(at + 6.0 * s, r.y + r.h / 2.0 - 3.0 * s, 6.0 * s, 6.0 * s), 3.0 * s, ENERGY);
            }
            self.hits.push((r, Hit::ShopTab(k)));
            tx = r.right() + 6.0 * s;
        }

        // The counter: the tab's things.
        let cy = ty + th + 8.0 * s;
        let counter = Rect::new(x, cy, lw, by - 10.0 * s - cy);
        self.glass(counter, c, s);
        let area = counter.inset(12.0 * s);
        self.sheets = 1;
        if let Some(tab) = &tab {
            let n = tab.rows.len();
            // The page the keyboard's row is on, else the one turned to.
            let at = (self.focus < n).then_some(self.focus);
            let page = |per: usize, pages: usize, me: &mut Self| {
                me.per = per;
                me.sheets = pages;
                me.sheet = at.map_or(me.sheet, |f| f / per.max(1)).min(pages - 1);
                me.sheet * per..((me.sheet + 1) * per).min(n)
            };
            match tab.kind {
                TabKind::Guns => {
                    // A table: each gun's numbers against the carried one's.
                    let head = 24.0 * s;
                    let fit = |rh: f32| ((area.h - head) / rh).floor().max(1.0) as usize;
                    let rh = if fit(ROW * s) >= n { ROW * s } else { (((area.h - head) / n as f32).floor()).max(ROW_LEAST * s) };
                    let per = if fit(rh) >= n { n } else { ((area.h - head - PAGER * s) / rh).floor().max(1.0) as usize };
                    let range = page(per, n.div_ceil(per.max(1)).max(1), self);
                    self.gun_table(&shop, &tab.rows[range], area, head, rh, ring, c, s);
                }
                TabKind::Tiles if hand(n, area, g, s).is_some() => {
                    let (x0, y0, w, h) = hand(n, area, g, s).unwrap_or((area.x, area.y, area.w, area.h));
                    page(n, 1, self);
                    for (j, &i) in tab.rows.iter().enumerate() {
                        let r = Rect::new(x0 + j as f32 * (w + g), y0, w, h);
                        self.hand_card(&shop.rows[i], shop.dps, &shop.stats, r, hot(r), ring(Hit::Buy(i)), c, s);
                        self.hits.push((r, Hit::Buy(i)));
                    }
                }
                TabKind::Tiles if shelf(n, area, g, s).is_some() => {
                    let (cols, w, h) = shelf(n, area, g, s).unwrap_or((1, area.w, SHELF_H * s));
                    page(n, 1, self);
                    for (j, &i) in tab.rows.iter().enumerate() {
                        let r = Rect::new(area.x + (j % cols) as f32 * (w + g), area.y + (j / cols) as f32 * (h + g), w, h);
                        self.shelf_row(&shop.rows[i], shop.dps, r, hot(r), ring(Hit::Buy(i)), c, s);
                        self.hits.push((r, Hit::Buy(i)));
                    }
                }
                TabKind::Tiles => {
                    let (per_row, w, per, pages) = flow(n, area, TILE_W * s, TILE_W * s * 1.3, TILE_H * s, g, PAGER * s);
                    let range = page(per, pages, self);
                    for (j, k) in range.clone().enumerate() {
                        let i = tab.rows[k];
                        let r = Rect::new(area.x + (j % per_row) as f32 * (w + g), area.y + (j / per_row) as f32 * (TILE_H * s + g), w, TILE_H * s);
                        self.tile(&shop.rows[i], r, hot(r), ring(Hit::Buy(i)), c, s);
                        self.hits.push((r, Hit::Buy(i)));
                    }
                }
                TabKind::Cards => {
                    if self.card_grid(&shop, tab, area, ring, &mut from, c, s) {
                    } else if let Some((x0, y0, w, h)) = hand(n, area, g, s) {
                        page(n, 1, self);
                        for (j, &i) in tab.rows.iter().enumerate() {
                            let r = Rect::new(x0 + j as f32 * (w + g), y0, w, h);
                            self.hand_card(&shop.rows[i], shop.dps, &shop.stats, r, hot(r) && drag.is_none(), ring(Hit::Buy(i)), c, s);
                            self.hits.push((r, Hit::Buy(i)));
                            if drag == Some(i) {
                                from = Some(r);
                            }
                        }
                    } else if let Some((cols, w, h)) = shelf(n, area, g, s) {
                        page(n, 1, self);
                        for (j, &i) in tab.rows.iter().enumerate() {
                            let r = Rect::new(area.x + (j % cols) as f32 * (w + g), area.y + (j / cols) as f32 * (h + g), w, h);
                            self.shelf_row(&shop.rows[i], shop.dps, r, hot(r) && drag.is_none(), ring(Hit::Buy(i)), c, s);
                            self.hits.push((r, Hit::Buy(i)));
                            if drag == Some(i) {
                                from = Some(r);
                            }
                        }
                    } else {
                        let (per_row, w, per, pages) = flow(n, area, CARD_W * s, CARD_MAX * s, ch, g, PAGER * s);
                        let range = page(per, pages, self);
                        for (j, k) in range.clone().enumerate() {
                            let i = tab.rows[k];
                            let r = Rect::new(area.x + (j % per_row) as f32 * (w + g), area.y + (j / per_row) as f32 * (ch + g), w, ch);
                            self.card_tile(&shop.rows[i], r, hot(r) && drag.is_none(), ring(Hit::Buy(i)), None, true, c, s);
                            self.hits.push((r, Hit::Buy(i)));
                            if drag == Some(i) {
                                from = Some(r);
                            }
                        }
                    }
                }
            }
            if self.sheets > 1 {
                self.pager(c, counter.right() - 12.0 * s, counter.bottom() - 40.0 * s, s);
            }
        } else {
            c.text(Weight::Medium, 15.0 * s, (area.x, area.y + 20.0 * s), Align::Left, "Nothing on offer", MUTED);
        }

        // The pane: the thing under the pointer (the owner, 2026-11-22:
        // "hover over should show what is the info, I don't need to
        // select"), else the picked thing, else the planet ahead. The
        // hits this frame has laid so far are the gun, the bag, the tabs,
        // the counter and the bottom bar's tools.
        let peek = match (drag, hover) {
            (None, Some(p)) if !pane.contains(p) => self.hits.iter().rev().find(|(r, h)| r.contains(p) && matches!(h, Hit::Buy(_) | Hit::Do(_))).map(|x| x.1),
            _ => None,
        };
        self.pane(&shop, peek.filter(|&h| Some(h) != picked), picked, pane, c, s);

        // A card being dragged: a hole where it was, its landing place
        // ringed, the card under the pointer.
        if let (Some(i), Some(p)) = (drag, hover) {
            if let Some(r) = from {
                c.rrect(r, 8.0 * s, [4, 6, 8, 170]);
                c.outline(r, 8.0 * s, 1.0, [255, 255, 255, 40]);
            }
            match self.drop_at(i, p) {
                Some(StartEvent::Move(_, k)) => {
                    let r = self.slot_rects[k];
                    c.outline(r.inset(-3.0 * s), 11.0 * s, 2.5 * s, ACCENT);
                    c.rrect(Rect::new(r.x - 5.0 * s, r.y + 6.0 * s, 3.0 * s, r.h - 12.0 * s), 1.5 * s, ACCENT);
                }
                Some(_) => c.outline(bag_rect.inset(-3.0 * s), 11.0 * s, 2.0 * s, with_alpha(ACCENT, 0.7)),
                None => {}
            }
            let r = Rect::new(p.0 - CARD_W * s / 2.0, p.1 - ch * 0.4, CARD_W * s, ch);
            c.rrect(Rect::new(r.x + 4.0 * s, r.y + 6.0 * s, r.w, r.h), 9.0 * s, [0, 0, 0, 110]);
            self.card_tile(&shop.rows[i], r, true, false, None, true, c, s);
        }
        // The strip's bead under the pointer, named, over everything.
        if let Some((at, t)) = tip.filter(|_| drag.is_none()) {
            super::strip_tip(c, at, &t, s);
        }
    }

    /// The gun range's bullets as a grid, shape by payload, when every card
    /// on the tab has its own cell and the grid fits; false otherwise.
    #[allow(clippy::too_many_arguments)]
    fn card_grid(&mut self, shop: &Shop, tab: &ShopTab, area: Rect, ring: impl Fn(Hit) -> bool, from: &mut Option<Rect>, c: &mut Canvas, s: f32) -> bool {
        let cells: Vec<_> = tab.rows.iter().map(|&i| shop.rows[i].cell.clone()).collect();
        if cells.iter().any(|x| x.is_none()) || cells.is_empty() {
            return false;
        }
        let cells: Vec<((u8, String), (u8, String))> = cells.into_iter().flatten().collect();
        let mut rows: Vec<(u8, String)> = cells.iter().map(|x| x.0.clone()).collect();
        let mut cols: Vec<(u8, String)> = cells.iter().map(|x| x.1.clone()).collect();
        rows.sort();
        rows.dedup();
        cols.sort();
        cols.dedup();
        let mut seen: Vec<(u8, u8)> = cells.iter().map(|x| (x.0 .0, x.1 .0)).collect();
        seen.sort();
        seen.dedup();
        if seen.len() < cells.len() {
            return false;
        }
        let g = 6.0 * s;
        let head = 22.0 * s;
        let label = rows.iter().map(|r| c.atlas.measure(Weight::SemiBold, 14.0 * s, &r.1)).fold(0.0f32, f32::max) + 14.0 * s;
        let w = (area.w - label - (cols.len() - 1) as f32 * g) / cols.len() as f32;
        let h = ((area.h - head - (rows.len() - 1) as f32 * g) / rows.len() as f32).min(CARD_H * s);
        if h < 36.0 * s || w < 64.0 * s {
            return false;
        }
        self.per = tab.rows.len();
        self.sheets = 1;
        self.sheet = 0;
        let hover = self.hover;
        let drag = self.drag;
        for (k, col) in cols.iter().enumerate() {
            // The column in its payload's colour.
            let tint = tab.rows.iter().zip(&cells).find(|(_, x)| x.1 .0 == col.0).map_or(MUTED, |(&i, _)| shop.rows[i].tint);
            c.text_fit(Weight::SemiBold, 13.5 * s, (area.x + label + k as f32 * (w + g) + w / 2.0, area.y + 14.0 * s), Align::Centre, &col.1, w, tint);
        }
        for (k, row) in rows.iter().enumerate() {
            c.text(Weight::SemiBold, 14.0 * s, (area.x, area.y + head + k as f32 * (h + g) + h / 2.0 + 5.0 * s), Align::Left, &row.1, MUTED);
        }
        for (&i, cell) in tab.rows.iter().zip(&cells) {
            let (rk, ck) = (rows.iter().position(|r| r.0 == cell.0 .0).unwrap_or(0), cols.iter().position(|x| x.0 == cell.1 .0).unwrap_or(0));
            let r = Rect::new(area.x + label + ck as f32 * (w + g), area.y + head + rk as f32 * (h + g), w, h);
            let lit = hover.is_some_and(|p| r.contains(p)) && drag.is_none();
            self.card_tile(&shop.rows[i], r, lit, ring(Hit::Buy(i)), None, false, c, s);
            self.hits.push((r, Hit::Buy(i)));
            if drag == Some(i) {
                *from = Some(r);
            }
        }
        true
    }

    /// The guns on a page of the Guns tab as a table: name, each number
    /// coloured against the carried gun's (green better, red worse), and
    /// its price or where it is.
    #[allow(clippy::too_many_arguments)]
    fn gun_table(&mut self, shop: &Shop, rows: &[usize], area: Rect, head: f32, rh: f32, ring: impl Fn(Hit) -> bool, c: &mut Canvas, s: f32) {
        let yours = shop.rows.iter().find(|r| r.area == ShopArea::Guns && r.on);
        let mut labels: Vec<String> = Vec::new();
        for &i in rows.iter().chain(shop.rows.iter().enumerate().filter(|(_, r)| r.area == ShopArea::Guns).map(|(i, _)| i).collect::<Vec<_>>().iter()) {
            for (l, _, _) in &shop.rows[i].bars {
                if !labels.contains(l) {
                    labels.push(l.clone());
                }
            }
        }
        // Each gun's estimated damage a second with your cards (`Shop::dps`,
        // `ShopRow::dps`; the owner, 2026-11-22) heads its numbers.
        let dps = shop.dps.is_some();
        if dps {
            labels.insert(0, "DPS".into());
        }
        let name_w = (area.w * 0.34).max(150.0 * s);
        let price_w = 78.0 * s;
        let col_w = ((area.w - name_w - price_w) / labels.len().max(1) as f32).max(1.0);
        let base = area.y + 15.0 * s;
        c.text(Weight::SemiBold, 13.0 * s, (area.x + 8.0 * s, base), Align::Left, "Gun", MUTED);
        for (k, l) in labels.iter().enumerate() {
            c.text_fit(Weight::SemiBold, 13.0 * s, (area.x + name_w + (k + 1) as f32 * col_w - 8.0 * s, base), Align::Right, l, col_w - 8.0 * s, MUTED);
        }
        c.text(Weight::SemiBold, 13.0 * s, (area.right() - 8.0 * s, base), Align::Right, "Price", MUTED);
        let hover = self.hover;
        for (k, &i) in rows.iter().enumerate() {
            let row = &shop.rows[i];
            let r = Rect::new(area.x, area.y + head + k as f32 * rh, area.w, rh - 3.0 * s);
            let lit = hover.is_some_and(|p| r.contains(p));
            let bg = if row.on { [255, 255, 255, 16] } else if lit { [255, 255, 255, 12] } else { [0, 0, 0, 0] };
            c.rrect(r, 7.0 * s, bg);
            if ring(Hit::Buy(i)) {
                c.outline(r, 7.0 * s, 1.5 * s, ACCENT);
            }
            let mid = r.y + r.h / 2.0 + 5.0 * s;
            let ic = 18.0 * s;
            c.icon(row.icon, (r.x + 8.0 * s, r.y + (r.h - ic) / 2.0), ic, row.tint);
            let name = if row.on { format!("{} (yours)", row.name) } else { row.name.clone() };
            c.text_fit(Weight::SemiBold, 14.5 * s, (r.x + 14.0 * s + ic, mid), Align::Left, &name, name_w - 22.0 * s - ic, TEXT);
            for (k, l) in labels.iter().enumerate() {
                if dps && k == 0 {
                    let (mine, this) = (shop.dps, if row.on { shop.dps } else { row.dps });
                    let Some(v) = this else { continue };
                    let ink = match mine {
                        Some(m) if !row.on && v > m + 0.05 => GOOD,
                        Some(m) if !row.on && v < m - 0.05 => WORSE,
                        _ => ENERGY,
                    };
                    c.text_fit(Weight::SemiBold, 14.0 * s, (area.x + name_w + col_w - 8.0 * s, mid), Align::Right, &dps_text(v), col_w - 8.0 * s, ink);
                    continue;
                }
                let Some((_, f, v)) = row.bars.iter().find(|b| b.0 == *l) else { continue };
                let mine = yours.and_then(|y| y.bars.iter().find(|b| b.0 == *l)).map(|b| b.1);
                let ink = match mine {
                    Some(m) if !row.on && *f > m + 0.005 => GOOD,
                    Some(m) if !row.on && *f < m - 0.005 => WORSE,
                    _ => TEXT,
                };
                // Its number over a thin bar of it, so the column reads at
                // a glance.
                let right = area.x + name_w + (k + 1) as f32 * col_w - 8.0 * s;
                c.text_fit(Weight::Medium, 14.0 * s, (right, mid - 2.0 * s), Align::Right, v, col_w - 8.0 * s, ink);
                if r.h >= 24.0 * s {
                    let bw = (col_w - 16.0 * s).min(70.0 * s);
                    let t = Rect::new(right - bw, r.bottom() - 6.0 * s, bw, 3.0 * s);
                    c.rrect(t, 1.5 * s, [255, 255, 255, 20]);
                    c.rrect(Rect::new(t.x, t.y, (f.clamp(0.0, 1.0) * bw).max(3.0 * s), t.h), 1.5 * s, with_alpha(if ink == TEXT { ACCENT } else { ink }, 0.8));
                }
            }
            let (tag, ink) = if !row.price.is_empty() { (row.price.clone(), if row.ready { ACCENT } else { BAD }) } else { (row.state.clone(), if row.on { GOOD } else { MUTED }) };
            c.text_fit(Weight::SemiBold, 14.0 * s, (area.right() - 8.0 * s, mid), Align::Right, &tag, price_w - 8.0 * s, ink);
            self.hits.push((r, Hit::Buy(i)));
        }
        if rows.len() > 1 && yours.is_some() && head + rows.len() as f32 * rh + 24.0 * s <= area.h - if self.sheets > 1 { PAGER * s } else { 0.0 } {
            c.text(Weight::Medium, 13.0 * s, (area.x + 8.0 * s, area.y + head + rows.len() as f32 * rh + 18.0 * s), Align::Left, "Green: better than yours  ·  red: worse", FAINT);
        }
    }

    /// What the pane's button says for shop row `i`, and whether it does
    /// anything (a carried gun, an upgrade bought out: no).
    pub(super) fn button_of(row: &ShopRow) -> (String, bool) {
        let buy = |p: &str| format!("Buy  ·  {p} points");
        match row.area {
            ShopArea::Cards if row.on => ("Take it out of the gun".into(), true),
            ShopArea::Cards if row.group == BAG => ("Put it in the gun".into(), true),
            ShopArea::Cards if row.price.is_empty() => ("Take it".into(), true),
            ShopArea::Guns if row.on => ("Your gun".into(), false),
            ShopArea::Guns if row.price.is_empty() => ("Carry this gun".into(), true),
            _ if owned(row) => ("Owned".into(), false),
            ShopArea::Upgrades if row.price.is_empty() && row.level.1 == 0 => (row.name.clone(), row.ready),
            ShopArea::Upgrades if row.price.is_empty() => (row.state.clone(), false),
            _ => (buy(&row.price), true),
        }
    }

    /// The pane: the picked thing in full and its button, else the planet
    /// ahead (or the range's own words).
    fn pane(&mut self, shop: &Shop, peek: Option<Hit>, picked: Option<Hit>, r: Rect, c: &mut Canvas, s: f32) {
        self.glass(r, c, s);
        let p = 18.0 * s;
        let iw = r.w - 2.0 * p;
        let mut y = r.y + p;
        let row = match peek.or(picked) {
            Some(Hit::Buy(i) | Hit::Do(i)) => shop.rows.get(i).map(|x| (i, x)),
            _ => None,
        };
        let Some((i, row)) = row else {
            // Nothing picked: the planet ahead, then how to buy. The top
            // bar holds the heading, so the pane names its subject in small
            // capitals and goes straight to the briefing (Look D, 11-20; it
            // repeated the heading word for word).
            let limit = r.bottom() - p;
            y += 14.0 * s;
            let label = if shop.range { "THE RANGE" } else { "THE PLANET AHEAD" };
            c.text(Weight::SemiBold, 12.5 * s, (r.x + p, y), Align::Left, label, ACCENT);
            y += 22.0 * s;
            // The planet's slime and its one line, as its landing card
            // will say them (Direction 11-24, the planet's introduction).
            if let Some((name, line)) = &shop.slime {
                let b = Rect::new(r.x + p, y - 6.0 * s, iw, 0.0);
                let lines = c.wrap(Weight::Medium, 14.5 * s, iw - 26.0 * s, line).len() as f32;
                let h = 40.0 * s + lines * 20.0 * s;
                let b = Rect::new(b.x, b.y, b.w, h);
                c.rrect(b, 8.0 * s, [255, 196, 72, 18]);
                c.rrect(Rect::new(b.x, b.y + 6.0 * s, 3.0 * s, h - 12.0 * s), 1.5 * s, WARN);
                c.text_fit(Weight::SemiBold, 17.0 * s, (b.x + 14.0 * s, b.y + 24.0 * s), Align::Left, name, iw - 26.0 * s, WARN);
                para(c, Weight::Medium, 14.5 * s, (b.x + 14.0 * s, b.y + 46.0 * s), iw - 26.0 * s, 20.0 * s, line, TEXT, limit);
                y += h + 10.0 * s;
            }
            for l in &shop.lines {
                y += para(c, Weight::Medium, 14.5 * s, (r.x + p, y), iw, 21.0 * s, l, [201, 210, 219, 255], limit) + 6.0 * s;
            }
            // The planet as a small map, where the pane has the room for
            // it over the two lines of how to buy (Look D, 11-21).
            if let Some(gl) = shop.glance.as_ref().filter(|g| g.w > 0 && g.h > 0) {
                let room = limit - y - 70.0 * s;
                let mw = iw.min(room * gl.w as f32 / gl.h as f32);
                let mh = mw * gl.h as f32 / gl.w as f32;
                if mw >= 120.0 * s {
                    y += glance(c, gl, Rect::new(r.x + p + (iw - mw) / 2.0, y + 6.0 * s, mw, mh), s) + 6.0 * s;
                }
            }
            let how = if shop.range { "Pick a gun, a bullet or a card to see it here. Drag a card onto a slot of the gun; the squad fires them left to right." } else { "Pick something on the counter or in the gun to see it here and buy it. Drag a card onto a slot of the gun to buy it and put it there." };
            para(c, Weight::Medium, 13.5 * s, (r.x + p, y + 8.0 * s), iw, 19.0 * s, how, MUTED, limit);
            return;
        };
        let (label, live) = Self::button_of(row);
        let btn = Rect::new(r.x + p, r.bottom() - p - 44.0 * s, iw, 44.0 * s);
        let limit = btn.y - 12.0 * s;
        // Its icon and name.
        let ic = 28.0 * s;
        c.icon(row.icon, (r.x + p, y), ic, if owned(row) { GOOD } else { row.tint });
        c.text_fit(Weight::SemiBold, 20.0 * s, (r.x + p + ic + 10.0 * s, y + 21.0 * s), Align::Left, &row.name, iw - ic - 10.0 * s, TEXT);
        y += ic + 22.0 * s;
        // What it is and where: its slot, its levels, its price.
        let mut kind: Vec<String> = Vec::new();
        match row.area {
            ShopArea::Cards => {
                kind.push(match self.slot_of(i).0 {
                    Some(k) => format!("In the gun, slot {}", k + 1),
                    None if row.group == BAG => "In the bag".into(),
                    None => "On offer".into(),
                });
                kind.push(if row.cost == 0 { "takes no budget".into() } else { format!("budget {}", row.cost) });
            }
            ShopArea::Guns => kind.push(if row.on { "Your gun".into() } else { format!("Gun, {}", row.state) }),
            ShopArea::Upgrades => {
                if row.level.1 > 1 {
                    kind.push(format!("Level {} of {}", row.level.0, row.level.1));
                } else if !row.state.is_empty() {
                    kind.push(row.state.clone());
                }
            }
        }
        if !row.price.is_empty() {
            kind.push(format!("{} points", row.price));
        }
        y += para(c, Weight::Medium, 14.0 * s, (r.x + p, y), iw, 20.0 * s, &kind.join("  ·  "), MUTED, limit) + 6.0 * s;
        y += para(c, Weight::Medium, 15.0 * s, (r.x + p, y), iw, 21.0 * s, words(row), [214, 222, 230, 255], limit) + 8.0 * s;
        // Its stats as bars first (the owner, 2026-11-22 08:53Z: "make
        // visual for speed range, etc"; the first cut led with DPS and he
        // read the whole shop as DPS). A rolled bullet's own numbers, a
        // standard bullet's at the middle of each.
        let head = |c: &mut Canvas, y: f32, t: &str| {
            c.text(Weight::SemiBold, 12.5 * s, (r.x + p, y + 10.0 * s), Align::Left, t, ACCENT);
            y + 20.0 * s
        };
        if row.area == ShopArea::Cards && !row.bars.is_empty() && y + 40.0 * s <= limit {
            y = head(c, y, "THIS BULLET");
            y += stat_bars(c, &row.bars, None, r.x + p, y, iw, 21.0 * s, 13.5 * s, row.tint, 1.0, limit, s) + 8.0 * s;
        }
        // The gun with it (or without it, or this gun against yours):
        // damage a shot, fire rate, range, speed and aim, with a tick
        // where the gun stands now, green where it is better, red worse.
        if !row.stats.is_empty() && !shop.stats.is_empty() && y + 40.0 * s <= limit {
            let t = match row.area {
                ShopArea::Cards if row.on => "YOUR GUN WITHOUT IT",
                ShopArea::Cards if row.no_room => "YOUR GUN WITH IT, IF IT FIT",
                ShopArea::Guns if row.on => "YOUR GUN",
                ShopArea::Guns => "THIS GUN, WITH YOUR CARDS",
                _ => "YOUR GUN WITH IT",
            };
            y = head(c, y, t);
            let mine = (!(row.area == ShopArea::Guns && row.on)).then_some(shop.stats.as_slice());
            y += stat_bars(c, &row.stats, mine, r.x + p, y, iw, 21.0 * s, 13.5 * s, [150, 162, 176, 255], 1.0, limit, s);
            if mine.is_some() && y + 18.0 * s <= limit {
                let note = if row.no_room { "Tick: now  ·  no room now, it would wait in the bag" } else { "Tick: your gun now  ·  green better, red worse" };
                c.text_fit(Weight::Medium, 12.5 * s, (r.x + p, y + 10.0 * s), Align::Left, note, iw, FAINT);
                y += 20.0 * s;
            }
            y += 6.0 * s;
        }
        // An upgrade's effect now and at its next level (Fireproof's fire
        // harm, 2026-11-22), in the warm colour a worse number reads in.
        if row.area == ShopArea::Upgrades && !row.bars.is_empty() {
            y += stat_bars(c, &row.bars, None, r.x + p, y, iw, 22.0 * s, 13.5 * s, WORSE, 1.0, limit, s) + 8.0 * s;
        }
        // A gun's budget and slots, against the carried gun's.
        if row.area == ShopArea::Guns && !row.bars.is_empty() {
            let yours = shop.rows.iter().find(|x| x.area == ShopArea::Guns && x.on).filter(|_| !row.on);
            let keep: Vec<_> = row.bars.iter().filter(|b| matches!(b.0.as_str(), "Budget" | "Slots" | "Shots")).cloned().collect();
            y += stat_bars(c, &keep, yours.map(|x| x.bars.as_slice()), r.x + p, y, iw, 21.0 * s, 13.5 * s, ACCENT, 1.0, limit, s) + 6.0 * s;
        }
        // Then, second and small, the estimated damage a second: now, with
        // it, and the change.
        if let (Some(now), Some(then)) = (shop.dps, row.dps) {
            if y + 22.0 * s <= limit {
                let cw = dps_chip(c, Some(now), Some(then), row.no_room, r.x + p + iw, y + 12.0 * s, 12.5 * s, s, 1.0);
                let with = match row.area {
                    ShopArea::Cards if row.on => "without it",
                    ShopArea::Guns if !row.on => "this gun",
                    _ => "with it",
                };
                let t = format!("Est. DPS  {} now  \u{b7}  {} {with}", dps_text(now), dps_text(then));
                c.text_fit(Weight::Medium, 13.0 * s, (r.x + p, y + 12.0 * s), Align::Left, &t, iw - cw - 10.0 * s, MUTED);
            }
        }
        // A look under the pointer: the button belongs to the picked
        // thing, so it says how to get this one's.
        if peek.is_some() {
            c.panel(btn, 10.0 * s, [22, 26, 32, 200], [16, 19, 24, 200], 1.0, [255, 255, 255, 14]);
            let say = if live { "Click it to pick it, then buy or place" } else { "" };
            let say = if row.area == ShopArea::Cards && !row.price.is_empty() { "Click to pick  ·  drag into the gun to buy" } else { say };
            c.text_fit(Weight::Medium, 14.0 * s, (btn.x + btn.w / 2.0, btn.y + btn.h / 2.0 + 5.0 * s), Align::Centre, if say.is_empty() { &label } else { say }, btn.w - 24.0 * s, MUTED);
            return;
        }
        // The button: the game says if it can't be done (the notice).
        if live {
            let primary = row.ready || row.price.is_empty();
            self.item(btn, &label, "", Hit::Do(i), primary, false, c, s);
            if !row.ready && !row.price.is_empty() {
                c.outline(btn, 10.0 * s, 1.5 * s, with_alpha(BAD, 0.8));
            }
        } else {
            c.panel(btn, 10.0 * s, [22, 26, 32, 235], [16, 19, 24, 235], 1.0, [255, 255, 255, 18]);
            c.text(Weight::SemiBold, 17.0 * s, (btn.x + 18.0 * s, btn.y + btn.h / 2.0 + 6.0 * s), Align::Left, &label, MUTED);
        }
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
        c.panel(r, 8.0 * s, t, b, if on { 1.5 * s } else { 1.0 }, edge);
        if focused {
            c.outline(r.inset(-2.0 * s), 10.0 * s, 1.5 * s, ACCENT);
        }
    }

    /// An upgrade tile, drawn as the command card draws a command: the
    /// icon, the name under it (two lines when long), levels as pips, the
    /// price.
    #[allow(clippy::too_many_arguments)]
    fn tile(&self, row: &ShopRow, r: Rect, hot: bool, focused: bool, c: &mut Canvas, s: f32) {
        let done = owned(row);
        self.face(r, row.ready || done, false, hot, focused, c, s);
        let dim: f32 = if row.ready || done { 1.0 } else { 0.42 };
        let ink = with_alpha(TEXT, dim.max(0.6));
        let room = r.w - 10.0 * s;
        let lines = if c.atlas.measure(Weight::SemiBold, 12.5 * s, &row.name) > room { c.wrap(Weight::SemiBold, 12.5 * s, room, &row.name) } else { vec![row.name.clone()] };
        let ic = if lines.len() > 1 { 20.0 * s } else { 26.0 * s };
        c.icon(row.icon, (r.x + (r.w - ic) / 2.0, r.y + 7.0 * s), ic, with_alpha(if done { GOOD } else { row.tint }, dim));
        if lines.len() > 1 {
            c.text_fit(Weight::SemiBold, 12.5 * s, (r.x + r.w / 2.0, r.y + 41.0 * s), Align::Centre, &lines[0], room, ink);
            c.text_fit(Weight::SemiBold, 12.5 * s, (r.x + r.w / 2.0, r.y + 55.0 * s), Align::Centre, &lines[1..].join(" "), room, ink);
        } else {
            let px = fit_px(c, &row.name, 13.5 * s, 12.5 * s, room);
            c.text_fit(Weight::SemiBold, px, (r.x + r.w / 2.0, r.y + 50.0 * s), Align::Centre, &row.name, room, ink);
        }
        if row.level.1 > 1 {
            let d = 5.0 * s;
            for k in 0..row.level.1 {
                let at = Rect::new(r.x + 7.0 * s + k as f32 * (d + 2.5 * s), r.bottom() - 11.0 * s, d, d);
                c.rrect(at, d / 2.0, if k < row.level.0 { ACCENT } else { [255, 255, 255, 40] });
            }
        }
        if done {
            c.icon(Icon::Check, (r.right() - 18.0 * s, r.y + 5.0 * s), 13.0 * s, GOOD);
        } else if !row.price.is_empty() {
            c.text(Weight::SemiBold, 12.5 * s, (r.right() - 6.0 * s, r.bottom() - 5.0 * s), Align::Right, &row.price, with_alpha(if row.ready { ENERGY } else { BAD }, 0.95));
        }
    }

    /// An offer on the counter's shelf (Look D, 2026-11-20): the counter
    /// held five half-size tiles in its top corner and two thirds of the
    /// screen stood empty, and what a card does showed only once picked.
    /// Now each offer is a row: its kind's band down the left, its icon
    /// large, its name, its one sentence (`about`, cut to two lines), and
    /// at the right its price over its budget pips or levels. Bends the
    /// 11-15 "half-size tiles" of shop B while every offer fits on one
    /// page: the row adds what the tile left out; more offers than fit
    /// go back to tiles that page.
    #[allow(clippy::too_many_arguments)]
    /// An offer as a tall card in the hand (`hand`): its kind's colour as a
    /// band along the top, its icon large, its name and its price, then
    /// everything it does, and its pips at the foot. The same face, colours
    /// and words as its shelf row; only the shape is a card's.
    fn hand_card(&self, row: &ShopRow, now: Option<f32>, mine: &[(String, f32, String)], r: Rect, hot: bool, focused: bool, c: &mut Canvas, s: f32) {
        // A card taller than `HAND_TALL` (`hand`, 11-24) draws its face up
        // to a seventh larger, and centres it in the height.
        let s = s * (r.h / (r.w * HAND_TALL)).clamp(1.0, 1.15);
        let done = owned(row);
        self.face(r, row.ready || row.on || done, row.on, hot, focused, c, s);
        let dim: f32 = if row.ready || row.on || done { 1.0 } else { 0.5 };
        let tint = with_alpha(if done { GOOD } else { row.tint }, dim);
        c.rrect(Rect::new(r.x + 10.0 * s, r.y + 6.0 * s, r.w - 20.0 * s, 3.0 * s), 1.5 * s, with_alpha(tint, 0.9 * dim));
        let pad = 14.0 * s;
        let room = r.w - 2.0 * pad;
        // The price (or a tick) top right, the badge top left.
        if done {
            c.icon(Icon::Check, (r.right() - pad - 16.0 * s, r.y + 16.0 * s), 16.0 * s, GOOD);
        } else if !row.price.is_empty() {
            c.text(Weight::SemiBold, 17.0 * s, (r.right() - pad, r.y + 30.0 * s), Align::Right, &row.price, with_alpha(if row.ready { ENERGY } else { BAD }, 0.95));
        }
        // A card held for the next shops (`Pick::Hold`, gun step F) says
        // so under its price (Look, Direction 12:18Z: Hold read only on
        // the bottom bar's button).
        if row.state == "held" {
            c.text(Weight::SemiBold, 11.5 * s, (r.right() - pad, r.y + 46.0 * s), Align::Right, "HELD", with_alpha(ACCENT, 0.95));
        }
        // The badge top left, and after it, small and second to the
        // bars, what it does to the gun's estimated damage a second.
        let mut bx = r.x + pad;
        if let Some(b) = row.badge {
            c.icon(b, (bx, r.y + 16.0 * s), 15.0 * s, with_alpha(TEXT, 0.8 * dim.max(0.6)));
            bx += 21.0 * s;
        }
        if let Some((t, ink, _)) = dps_words(now, row.dps, row.no_room) {
            let room = r.right() - pad - c.atlas.measure(Weight::SemiBold, 17.0 * s, &row.price) - 8.0 * s - bx;
            c.text_fit(Weight::SemiBold, 12.5 * s, (bx, r.y + 28.0 * s), Align::Left, &t, room, with_alpha(ink, dim.max(0.6)));
        }
        // Its stats as bars (the owner, 2026-11-22 08:53Z: "make visual
        // for speed range, etc"): a rolled bullet's own numbers; else only
        // the gun's numbers it changes, each as before and after ("Damage
        // 72 → 144", the tick where the gun is now). Every row of the gun
        // on every card read the same on most of them (polish review
        // finding 8, 11-24: Delay and Tinder showed five identical rows).
        // (A device body's rolls too, on the Tools tab, 11-24.)
        let own = (row.area == ShopArea::Cards || row.group == "Tools") && !row.bars.is_empty();
        let changed: Vec<(String, f32, String)> = if !own && row.bars.is_empty() && !mine.is_empty() {
            row.stats
                .iter()
                .filter_map(|(l, f, v)| {
                    let m = mine.iter().find(|b| b.0 == *l)?;
                    (m.2 != *v || (m.1 - f).abs() > 0.005).then(|| (l.clone(), *f, format!("{} \u{2192} {v}", m.2)))
                })
                .collect()
        } else {
            Vec::new()
        };
        let bars: &[(String, f32, String)] = if own { &row.bars } else { &changed };
        let lead_b = 16.0 * s;
        // The icon smaller than the first cut's 56, so the bars fit under
        // the name.
        let ic = if bars.is_empty() { 56.0 * s } else { 40.0 * s };
        let pips = if row.area == ShopArea::Cards { row.cost.min(6) } else if row.level.1 > 1 { row.level.1.min(8) as u32 } else { 0 };
        let foot = if pips > 0 { 26.0 * s } else { 10.0 * s };
        let about = words(row);
        let (lead, apx) = (18.0 * s, 13.5 * s);
        let lines = if about.is_empty() { Vec::new() } else { c.wrap(Weight::Medium, apx, room, about) };
        // The face from the icon down, lowered by a share of what the card
        // grew past `HAND_TALL`: the same for every card in the hand, so
        // their icons and names stand in one line (centring each face on
        // its own height set them at different heights).
        // On a grown card a bars card's smaller icon sits at the foot of
        // the large one's box, so every name in the hand is on one line.
        let grown = (r.h - r.w * HAND_TALL).max(0.0);
        let ic_box = if grown >= 16.0 * s { 56.0 * s } else { ic };
        let name_dy = ic_box + 30.0 * s;
        let top = r.y + 38.0 * s + 0.3 * (grown - 16.0 * s).max(0.0);
        c.icon(row.icon, (r.x + (r.w - ic) / 2.0, top + ic_box - ic), ic, tint);
        let ink = with_alpha(TEXT, dim.max(0.6));
        let px = fit_px(c, &row.name, 17.0 * s, 13.5 * s, room);
        let name_y = top + name_dy;
        c.text_fit(Weight::SemiBold, px, (r.x + r.w / 2.0, name_y), Align::Centre, &row.name, room, ink);
        let mut ty = name_y + 26.0 * s;
        let floor = r.bottom() - foot - 4.0 * s;
        if own {
            ty += stat_bars(c, bars, None, r.x + pad, ty - 12.0 * s, room, lead_b, 12.5 * s, row.tint, dim.max(0.6), floor, s) + 4.0 * s;
        } else if !bars.is_empty() {
            let theirs: Vec<(String, f32, String)> = mine.iter().filter(|m| bars.iter().any(|b| b.0 == m.0)).cloned().collect();
            ty += stat_bars(c, bars, Some(&theirs), r.x + pad, ty - 12.0 * s, room, lead_b, 12.5 * s, [150, 162, 176, 255], dim.max(0.6), floor, s) + 4.0 * s;
        }
        if !lines.is_empty() {
            let fit = (((r.bottom() - foot - ty) / lead).floor() as usize + 1).max(1);
            for (k, l) in lines.iter().take(fit).enumerate() {
                if ty + k as f32 * lead > r.bottom() - foot + 2.0 * s {
                    break;
                }
                let cut = k + 1 == fit && lines.len() > fit;
                let text = if cut { format!("{}\u{2026}", l.trim_end_matches(['.', ',', ' '])) } else { l.clone() };
                c.text_fit(Weight::Medium, apx, (r.x + r.w / 2.0, ty + k as f32 * lead), Align::Centre, &text, room, with_alpha([196, 205, 214, 255], dim.max(0.6)));
            }
        }
        let d = 6.0 * s;
        let pw = pips as f32 * (d + 3.0 * s) - 3.0 * s;
        for k in 0..pips {
            let lit = if row.area == ShopArea::Cards { true } else { k < row.level.0 as u32 };
            let at = Rect::new(r.x + (r.w - pw) / 2.0 + k as f32 * (d + 3.0 * s), r.bottom() - 16.0 * s, d, d);
            c.rrect(at, d / 2.0, if lit { with_alpha(if row.area == ShopArea::Cards { ENERGY } else { ACCENT }, 0.85 * dim.max(0.6)) } else { [255, 255, 255, 40] });
        }
    }

    #[allow(clippy::too_many_arguments)]
    fn shelf_row(&self, row: &ShopRow, now: Option<f32>, r: Rect, hot: bool, focused: bool, c: &mut Canvas, s0: f32) {
        // A taller row (`shelf`) draws its icon, name and price up to a
        // third larger, and its whole text where one sentence was cut.
        let tall = r.h > SHELF_H * s0 * 1.1;
        let s = s0 * (r.h / (SHELF_H * s0)).clamp(1.0, 1.25);
        let done = owned(row);
        self.face(r, row.ready || row.on || done, row.on, hot, focused, c, s);
        let dim: f32 = if row.ready || row.on || done { 1.0 } else { 0.5 };
        c.rrect(Rect::new(r.x + 3.0 * s, r.y + 6.0 * s, 3.0 * s, r.h - 12.0 * s), 1.5 * s, with_alpha(if done { GOOD } else { row.tint }, 0.9 * dim));
        let ic = 30.0 * s;
        c.icon(row.icon, (r.x + 16.0 * s, r.y + (r.h - ic) / 2.0), ic, with_alpha(if done { GOOD } else { row.tint }, dim));
        let tx = r.x + 16.0 * s + ic + 14.0 * s;
        // The right column: the price (or a tick), the pips under it.
        let price = if done { String::new() } else { row.price.clone() };
        let pw = if price.is_empty() { 0.0 } else { c.atlas.measure(Weight::SemiBold, 16.0 * s, &price) };
        let d = 6.0 * s;
        let pips = if row.area == ShopArea::Cards { row.cost.min(6) } else if row.level.1 > 1 { row.level.1.min(8) as u32 } else { 0 };
        let pips_w = if pips > 0 { pips as f32 * (d + 3.0 * s) - 3.0 * s } else { 0.0 };
        let right = pw.max(pips_w).max(if done { 18.0 * s } else { 0.0 });
        let rx = r.right() - 12.0 * s;
        if done {
            c.icon(Icon::Check, (rx - 16.0 * s, r.y + 14.0 * s), 16.0 * s, GOOD);
        } else if !price.is_empty() {
            c.text(Weight::SemiBold, 16.0 * s, (rx, r.y + 27.0 * s), Align::Right, &price, with_alpha(if row.ready { ENERGY } else { BAD }, 0.95));
        }
        for k in 0..pips {
            let lit = if row.area == ShopArea::Cards { true } else { k < row.level.0 as u32 };
            let at = Rect::new(rx - pips_w + k as f32 * (d + 3.0 * s), r.bottom() - 17.0 * s, d, d);
            c.rrect(at, d / 2.0, if lit { with_alpha(if row.area == ShopArea::Cards { ENERGY } else { ACCENT }, 0.85 * dim.max(0.6)) } else { [255, 255, 255, 40] });
        }
        // What it does to the gun's damage a second, left of the price.
        let chip = if price.is_empty() && !done { rx } else { rx - right - 10.0 * s };
        let cw = dps_chip(c, now, row.dps, row.no_room, chip, r.y + 26.0 * s, 12.5 * s0, s0, dim.max(0.6));
        let right = if cw > 0.0 { rx - chip + cw } else { right };
        // The name, a badge after it, and the sentence.
        let room = (rx - right - 14.0 * s - tx).max(40.0 * s);
        let ink = with_alpha(TEXT, dim.max(0.6));
        let nw = c.text_fit(Weight::SemiBold, 15.5 * s, (tx, r.y + 24.0 * s), Align::Left, &row.name, room, ink);
        if let Some(b) = row.badge {
            let bs = 14.0 * s;
            if nw + 8.0 * s + bs <= room {
                c.icon(b, (tx + nw + 8.0 * s, r.y + 12.0 * s), bs, with_alpha(TEXT, 0.8 * dim.max(0.6)));
            }
        }
        // The text: the first sentence of `about`, two lines at most; in a
        // tall row all of it, as many lines as fit over the pips.
        // A device body's rolls (`menu::body_bars`) on a line of their own
        // before the sentence: on a shelf its bars had no room, and with
        // seven Tools rows the owned bodies read as words only (Look,
        // Direction 12:18Z: reach, cooldown and slots on its card).
        let stats = (row.group == "Tools" && !row.bars.is_empty()).then(|| row.bars.iter().map(|(l, _, v)| match l.as_str() {
            "Cooldown" => v.clone(),
            "Card slots" => format!("{v} slots"),
            _ => format!("{} {v}", l.to_lowercase()),
        }).collect::<Vec<_>>().join(" \u{b7} "));
        let top = if let Some(st) = &stats {
            c.text_fit(Weight::Medium, 13.0 * s, (tx, r.y + 44.0 * s), Align::Left, st, room, with_alpha(ACCENT, dim.max(0.6)));
            17.0 * s
        } else {
            0.0
        };
        let about = if tall { words(row) } else { words(row).split_inclusive(". ").next().unwrap_or("").trim() };
        if !about.is_empty() {
            let lines = c.wrap(Weight::Medium, 13.0 * s, room, about);
            let fit = if tall { (((r.h - 44.0 * s - top - 10.0 * s0) / (17.0 * s)).floor() as usize + 1).max(2) } else { 2 };
            let fit = if stats.is_some() && !tall { 1 } else { fit };
            for (k, l) in lines.iter().take(fit).enumerate() {
                let cut = k + 1 == fit && lines.len() > fit;
                let text = if cut { format!("{}…", l.trim_end_matches(['.', ',', ' '])) } else { l.clone() };
                c.text_fit(Weight::Medium, 13.0 * s, (tx, r.y + 44.0 * s + top + k as f32 * 17.0 * s), Align::Left, &text, room, with_alpha([196, 205, 214, 255], dim.max(0.6)));
            }
        }
    }

    /// A bullet card, half the old size (owner, 17:41Z: "The bullet tiles
    /// could easily be half the size without even losing any info"): a
    /// band of its kind's colour, its icon, its name (two lines when long,
    /// none in the range's grid, whose row and column name it), its budget
    /// cost as pips and its price; on the gun, its slot number.
    #[allow(clippy::too_many_arguments)]
    fn card_tile(&self, row: &ShopRow, r: Rect, hot: bool, focused: bool, slot: Option<usize>, named: bool, c: &mut Canvas, s: f32) {
        // (The gun's own row says which are in it: no edge of their own,
        // which read as all picked.)
        self.face(r, row.ready || row.on, false, hot, focused, c, s);
        let dim = if row.ready || row.on { 1.0 } else { 0.45 };
        c.rrect(Rect::new(r.x + 5.0 * s, r.y + 3.0 * s, r.w - 10.0 * s, 3.0 * s), 1.5 * s, with_alpha(row.tint, 0.9 * dim));
        let d = 5.0 * s;
        let price = (!row.price.is_empty()).then(|| row.price.clone());
        let price_w = price.as_ref().map_or(0.0, |p| c.atlas.measure(Weight::SemiBold, 13.0 * s, p) + 6.0 * s);
        if let Some(p) = &price {
            c.text(Weight::SemiBold, 13.0 * s, (r.right() - 6.0 * s, r.bottom() - 6.0 * s), Align::Right, p, if row.ready { ACCENT } else { BAD });
        }
        if !named {
            // The range's grid: the icon and the pips, centred.
            let ic = 22.0 * s;
            let pips = row.cost.min(6) as f32;
            let w = ic + if pips > 0.0 { 6.0 * s + pips * (d + 2.5 * s) } else { 0.0 };
            let x0 = r.x + (r.w - price_w - w).max(0.0) / 2.0;
            c.icon(row.icon, (x0, r.y + (r.h - ic) / 2.0 + 2.0 * s), ic, with_alpha(row.tint, dim));
            for k in 0..row.cost.min(6) {
                c.rrect(Rect::new(x0 + ic + 6.0 * s + k as f32 * (d + 2.5 * s), r.y + r.h / 2.0, d, d), d / 2.0, with_alpha(ENERGY, 0.85 * dim.max(0.6)));
            }
            return;
        }
        // The icon in a column at the left when there is room for it.
        let wide = r.w >= 110.0 * s;
        let mut tx = r.x + 7.0 * s;
        if let Some(k) = slot {
            let w = c.text(Weight::SemiBold, 12.5 * s, (tx, r.y + 19.0 * s), Align::Left, &format!("{}", k + 1), with_alpha(TEXT, 0.55));
            if !wide {
                tx += w + 5.0 * s;
            }
        }
        if wide {
            let ic = 18.0 * s;
            c.icon(row.icon, (r.x + 7.0 * s, r.bottom() - ic - 7.0 * s), ic, with_alpha(row.tint, dim));
            tx = r.x + 31.0 * s;
        }
        let mut room = r.right() - 6.0 * s - tx;
        if let (Some(b), true) = (row.badge, wide) {
            let bs = 14.0 * s;
            c.icon(b, (r.right() - bs - 6.0 * s, r.y + 9.0 * s), bs, with_alpha(TEXT, 0.85 * dim.max(0.6)));
            room -= bs + 4.0 * s;
        }
        let ink = with_alpha(TEXT, dim.max(0.6));
        let two = c.atlas.measure(Weight::SemiBold, 12.5 * s, &row.name) > room;
        if two {
            let lines = c.wrap(Weight::SemiBold, 12.5 * s, room, &row.name);
            c.text_fit(Weight::SemiBold, 12.5 * s, (tx, r.y + 19.0 * s), Align::Left, &lines[0], room, ink);
            c.text_fit(Weight::SemiBold, 12.5 * s, (tx, r.y + 32.0 * s), Align::Left, &lines[1..].join(" "), (r.right() - 6.0 * s - price_w - tx).max(room * 0.5), ink);
        } else {
            let px = fit_px(c, &row.name, 13.5 * s, 12.5 * s, room);
            c.text_fit(Weight::SemiBold, px, (tx, r.y + 21.0 * s), Align::Left, &row.name, room, ink);
        }
        // The budget it takes, as pips (none: free).
        let foot = r.bottom() - 9.0 * s;
        for k in 0..row.cost.min(6) {
            c.rrect(Rect::new(tx + k as f32 * (d + 2.5 * s), foot - d / 2.0, d, d), d / 2.0, with_alpha(ENERGY, 0.85 * dim.max(0.6)));
        }
    }
}
