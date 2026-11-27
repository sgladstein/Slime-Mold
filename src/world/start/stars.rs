//! **The run's star map** (2026-11-26, `mould::route`; the owner's pick of
//! a route with forks and contracts, 11-25 23:45Z): one screen between a
//! won planet and its shop. The run's twelve steps run left to right
//! across the top, the landing world first and the home world last; each
//! step's planets sit in lanes down the map, joined to the next step's by
//! dotted lanes. Under it, the planets the route can go to next stand side
//! by side as cards, so the choice is read at once (2026-11-27, the
//! owner: "The planet path selection map needs to be improved and planet
//! text needs to be improved"). Until then one pane at the right told one
//! planet at a time, half empty, and the choice was made by clicking
//! between rings that looked alike.
//!
//! What the map says at a glance: a planet's difficulty is its ring's
//! colour (Safe green, Fair blue, Hard red-orange; Hard was the amber of
//! the stars won and read as a prize), its size (a Hard planet's disc is
//! half again a Safe one's) and one to three pips, on every node, so a
//! route can be read ahead (his "clearly different difficulties"); its
//! fill is its biome's colour. A step's planets share their slime
//! (`route`), so the slime is named once, over its column, where it was
//! under every node. Picking a planet lights the lanes it leads on to.
//!
//! A card says what differs between the planets in the same words and
//! the same order: the slime's size against the planet just won, the hits
//! it takes, the twist, a Hard planet's rare card, the bonus goals
//! (contracts), the world and its map's size, where it leads, and its pay
//! at the foot. Where the card is short (a large UI size) the rows drop
//! from the end, the pay kept. The heat switch moved to the bottom bar,
//! beside "Fly", as it changes the planet picked.

use super::*;

/// A planet on the map as the page draws it (the window fills it from
/// `Run::star_map`).
#[derive(Clone, Debug, PartialEq, Default)]
pub struct StarNode {
    pub k: u8,
    /// Its lane across the map, -1 to 1.
    pub lane: f32,
    /// 1 Safe, 2 Fair, 3 Hard.
    pub pips: u8,
    /// The next step's nodes it leads to.
    pub to: Vec<u8>,
    pub name: String,
    pub slime: String,
    pub biome: String,
    /// The twist's name and line.
    pub twist: Option<(String, String)>,
    /// The slime's size as a factor on the first planet's (the cards
    /// compare it with `StarView::last_size`).
    pub size: f32,
    /// The blows a bit of its slime takes (`run::blows`).
    pub hits: f32,
    /// Its map's area, in cells.
    pub area: f32,
    /// What winning it pays at two stars, without and with heat.
    pub pay: u32,
    pub heat_pay: u32,
    /// The pip's line (`route::Pip::about`).
    pub about: String,
    /// Each contract's words and pay.
    pub contracts: Vec<(String, u32)>,
    /// On the route taken (a step behind, or the planet being played).
    pub taken: bool,
    /// The route can go here next.
    pub open: bool,
    /// Stars won on it, once freed.
    pub stars: Option<u8>,
}

/// The run's star map, a step a column.
#[derive(Clone, Debug, PartialEq, Default)]
pub struct StarView {
    pub steps: Vec<Vec<StarNode>>,
    /// The step ahead, from 1.
    pub at: u32,
    pub heading: String,
    /// The run so far ("5 planets freed, 10 stars").
    pub tally: String,
    pub purse: String,
    /// The planet just won's slime size and map area, which the cards
    /// measure the next planets against (None before the first fork).
    pub last_size: Option<f32>,
    pub last_area: Option<f32>,
    /// What heat does, for the switch's line.
    pub heat_says: String,
}

/// The map's bars, UI units (as the shop's).
const TOP: f32 = 46.0;
const BOTTOM: f32 = 62.0;
/// A Fair planet's disc on the map at most, and a card's size at most.
const NODE: f32 = 15.0;
const CARD_W: f32 = 420.0;
const CARD_H: f32 = 404.0;

/// The pip colours: Safe, Fair, Hard. Hard is a red-orange (it was the
/// amber of the stars won and the slime, and read as a reward).
pub(crate) const PIP_INKS: [Rgba; 3] = [GOOD, ACCENT, [255, 120, 76, 255]];

fn pip_ink(pips: u8) -> Rgba {
    PIP_INKS[(pips.clamp(1, 3) - 1) as usize]
}

/// A lane as drawn: its rank (higher over lower), its ends, its ink, its
/// dots' radius, and the two planets' radii it stops short of.
type Lane = (u8, (f32, f32), (f32, f32), Rgba, f32, f32, f32);

fn pip_name(pips: u8) -> &'static str {
    ["Safe", "Fair", "Hard"][(pips.clamp(1, 3) - 1) as usize]
}

/// A planet's disc against a Fair one's: a Hard planet is drawn half
/// again a Safe one's.
fn pip_scale(pips: u8) -> f32 {
    [0.8, 1.0, 1.2][(pips.clamp(1, 3) - 1) as usize]
}

/// A biome's colour, for a planet's disc (the ground's own hues, lifted).
fn biome_ink(biome: &str) -> Rgba {
    match biome {
        "meadow" => [104, 150, 84, 255],
        "barrens" => [160, 132, 100, 255],
        "fen" => [86, 136, 118, 255],
        "river valley" => [84, 128, 172, 255],
        "jungle" => [62, 148, 92, 255],
        "highlands" => [146, 146, 164, 255],
        "cave" => [118, 100, 164, 255],
        "core" => [184, 88, 74, 255],
        _ => [116, 126, 146, 255],
    }
}

fn scaled(c: Rgba, k: f32, a: f32) -> Rgba {
    [(c[0] as f32 * k).min(255.0) as u8, (c[1] as f32 * k).min(255.0) as u8, (c[2] as f32 * k).min(255.0) as u8, (255.0 * a).clamp(0.0, 255.0) as u8]
}

/// A planet drawn at `(cx, cy)`, radius `r`: its biome's colour lit from
/// the top left, its ring in its pip's colour, and the pips inside.
#[allow(clippy::too_many_arguments)]
fn planet(c: &mut Canvas, cx: f32, cy: f32, r: f32, biome: &str, pips: u8, fade: f32, s: f32) {
    let b = biome_ink(biome);
    let ink = pip_ink(pips);
    let disc = Rect::new(cx - r, cy - r, 2.0 * r, 2.0 * r);
    if pips >= 3 {
        // A Hard planet's halo, so it reads as the dangerous one from
        // across the map.
        c.rrect(disc.inset(-3.5 * s), r + 3.5 * s, with_alpha(ink, 0.16 * fade));
    }
    c.panel(disc, r, scaled(b, 0.62, fade.max(0.5)), scaled(b, 0.26, fade.max(0.5)), 0.0, [0; 4]);
    // The lit side.
    let lr = r * 0.55;
    c.rrect(Rect::new(cx - r * 0.62, cy - r * 0.68, lr, lr), lr / 2.0, scaled(b, 1.15, 0.22 * fade));
    c.outline(disc, r, (if pips >= 3 { 2.2 } else { 1.6 }) * s, with_alpha(ink, fade));
    let pr = (r * 0.13).max(1.3 * s);
    let gap = pr * 2.8;
    let w0 = cx - gap * (pips as f32 - 1.0) / 2.0;
    for p in 0..pips {
        let px = w0 + gap * p as f32;
        c.rrect(Rect::new(px - pr, cy - pr, 2.0 * pr, 2.0 * pr), pr, with_alpha([240, 244, 248, 255], 0.9 * fade));
    }
}

/// "+30%", "-20%" or "same": `now` against `last`, and its colour (more
/// is the warning's, less the good's).
fn change(now: f32, last: f32) -> (String, Rgba) {
    let f = now / last.max(1e-3);
    if (f - 1.0).abs() < 0.05 {
        ("same as last".into(), MUTED)
    } else if f >= 1.95 {
        (format!("{f:.1}x the last"), PIP_INKS[2])
    } else if f > 1.0 {
        (format!("+{:.0}% vs last", (f - 1.0) * 100.0), WARN)
    } else {
        (format!("-{:.0}% vs last", (1.0 - f) * 100.0), GOOD)
    }
}

/// "2" or "2-3" hits.
fn hits(b: f32) -> String {
    if b.fract() < 0.05 || b.fract() > 0.95 {
        format!("{:.0}", b.round())
    } else {
        format!("{:.0}-{:.0}", b.floor(), b.ceil())
    }
}

impl StartMenu {
    /// The page's rows for the keyboard: the planets the route can go to,
    /// then heat, fly and give up.
    pub(super) fn star_rows(&self) -> Vec<Hit> {
        let Some(v) = self.stars.as_ref() else { return vec![Hit::EndRun] };
        let open = v.steps.get(v.at as usize - 1).map_or(Vec::new(), |st| st.iter().filter(|n| n.open).map(|n| Hit::Star(n.k)).collect());
        open.into_iter().chain([Hit::Heat, Hit::Next, Hit::EndRun]).collect()
    }

    /// Picks planet `k` of the step ahead (heat off again).
    pub(super) fn pick_star(&mut self, k: u8) {
        if self.star_pick != Some(k) {
            self.heat = false;
        }
        self.star_pick = Some(k);
        if let Some(i) = self.rows().iter().position(|&h| h == Hit::Star(k)) {
            self.focus = i;
        }
    }

    /// The star map, drawn whole on one screen: the route across the top,
    /// the planets to pick from as cards under it.
    pub(super) fn stars_page(&mut self, c: &mut Canvas, s: f32, focus: Option<Hit>) {
        let Some(v) = self.stars.clone() else { return };
        let (sw, sh) = (c.screen.0 as f32, c.screen.1 as f32);
        let x = 24.0 * s;
        let hover = self.hover;
        let hot = |r: Rect| hover.is_some_and(|p| r.contains(p));
        let here = v.steps.get(v.at as usize - 1).cloned().unwrap_or_default();
        // The pick: kept while it is open, else the Fair planet if it is,
        // else the first open.
        if !self.star_pick.is_some_and(|k| here.iter().any(|n| n.k == k && n.open)) {
            self.star_pick = here.iter().find(|n| n.open && n.k == 0).or_else(|| here.iter().find(|n| n.open)).map(|n| n.k);
            self.heat = false;
        }
        // A keyboard row on a planet picks it.
        if let Some(Hit::Star(k)) = focus {
            if self.star_pick != Some(k) {
                self.star_pick = Some(k);
                self.heat = false;
            }
        }
        let pick = self.star_pick;
        // The choices, in their lanes' order down the map, so the cards
        // read left to right as the map does top to bottom.
        let mut open: Vec<StarNode> = here.iter().filter(|n| n.open).cloned().collect();
        open.sort_by(|a, b| a.lane.total_cmp(&b.lane));

        c.rrect(Rect::new(0.0, 0.0, sw, sh), 0.0, [7, 9, 14, 255]);
        // A few faint stars behind the map, fixed by the screen.
        let mut z: u32 = 0x9E37_79B9;
        for _ in 0..140 {
            z ^= z << 13;
            z ^= z >> 17;
            z ^= z << 5;
            let px = (z % 10_000) as f32 / 10_000.0 * sw;
            z ^= z << 13;
            z ^= z >> 17;
            z ^= z << 5;
            let py = (z % 10_000) as f32 / 10_000.0 * sh;
            let r = (1.0 + (z >> 28) as f32 * 0.12) * s;
            c.rrect(Rect::new(px, py, r, r), r / 2.0, [200, 210, 230, 20 + (z >> 26) as u8 % 40]);
        }

        // The top bar: the heading, the run so far, the points.
        let top = TOP * s;
        c.rrect(Rect::new(0.0, 0.0, sw, top), 0.0, [8, 11, 14, 236]);
        c.rrect(Rect::new(0.0, top - s.max(1.0), sw, s.max(1.0)), 0.0, EDGE);
        let purse_w = c.text(Weight::SemiBold, 20.0 * s, (sw - x, 30.0 * s), Align::Right, &v.purse, ACCENT);
        let head_w = c.text_fit(Weight::SemiBold, 19.0 * s, (x, 30.0 * s), Align::Left, &v.heading, sw - 2.0 * x - purse_w - 24.0 * s, TEXT);
        let tx = x + head_w + 18.0 * s;
        if sw - x - purse_w - 24.0 * s - tx > 80.0 * s {
            c.text_fit(Weight::Medium, 15.0 * s, (tx, 30.0 * s), Align::Left, &v.tally, sw - x - purse_w - 24.0 * s - tx, MUTED);
        }

        // The map over the cards: the cards take what they need, up to
        // three fifths of the screen, the map the rest.
        let by = sh - BOTTOM * s;
        let gap = 16.0 * s;
        let body = Rect::new(x, top + gap, sw - 2.0 * x, by - top - 2.0 * gap);
        let card_h = if open.is_empty() { 0.0 } else { (CARD_H * s).min(body.h * 0.62) };
        let map = Rect::new(body.x, body.y, body.w, body.h - card_h - if open.is_empty() { 0.0 } else { gap });
        let hover_node = self.route_map(c, s, &v, map, pick, focus);
        // Hovering a planet names it, and what it is.
        if let Some((cx, cy, r, nd)) = hover_node {
            let tip = format!("{} · {} · the {} · {}", pip_name(nd.pips), nd.name, nd.slime.trim_start_matches("The "), nd.biome);
            let tw = c.atlas.measure(Weight::Medium, 13.5 * s, &tip) + 20.0 * s;
            let tx = (cx - tw / 2.0).clamp(x, sw - x - tw);
            let tr = Rect::new(tx, cy - r - 36.0 * s, tw, 26.0 * s);
            c.panel(tr, 7.0 * s, PANEL_TOP, PANEL_BOTTOM, 1.0, EDGE);
            c.text(Weight::Medium, 13.5 * s, (tr.x + 10.0 * s, tr.y + 17.5 * s), Align::Left, &tip, TEXT);
        }

        // The cards: one a planet the route can go to.
        if open.is_empty() {
            c.text(Weight::Medium, 15.0 * s, (sw / 2.0, map.bottom() - 30.0 * s), Align::Centre, "No planet to pick from here.", MUTED);
        }
        let n = open.len().max(1) as f32;
        let cw = (CARD_W * s).min((body.w - gap * (n - 1.0)) / n);
        let x0 = body.x + (body.w - cw * n - gap * (n - 1.0)) / 2.0;
        // What the cards' bars are measured on: the biggest slime among
        // them and the last planet's.
        let most = open.iter().map(|o| o.size).chain(v.last_size).fold(0.0f32, f32::max).max(1e-3);
        for (j, nd) in open.iter().enumerate() {
            let r = Rect::new(x0 + j as f32 * (cw + gap), map.bottom() + gap, cw, card_h);
            let picked = Some(nd.k) == pick;
            self.star_card(c, s, &v, nd, r, picked, hot(r), focus == Some(Hit::Star(nd.k)), most);
            self.hits.push((r, Hit::Star(nd.k)));
        }

        // The bottom bar: fly there, heat, give up.
        c.rrect(Rect::new(0.0, by, sw, sh - by), 0.0, [8, 11, 14, 236]);
        c.rrect(Rect::new(0.0, by, sw, s.max(1.0)), 0.0, EDGE);
        let picked = here.iter().find(|n| Some(n.k) == pick);
        let next = match picked {
            Some(nd) => format!("Fly to {}", nd.name),
            None => "Pick a planet".to_string(),
        };
        let end = "Give up the run";
        let bh = 40.0 * s;
        let bt = by + (BOTTOM * s - bh) / 2.0;
        let wn = c.atlas.measure(Weight::SemiBold, 18.0 * s, &next) + 36.0 * s;
        self.item(Rect::new(x, bt, wn, bh), &next, "", Hit::Next, true, focus == Some(Hit::Next), c, s);
        // Heat: a switch for the planet picked.
        let on = self.heat;
        let label = match (on, picked) {
            (true, Some(nd)) => format!("Heat on: pays {}", nd.heat_pay),
            (true, None) => "Heat on".to_string(),
            (false, _) => "Heat off".to_string(),
        };
        let lw = c.atlas.measure(Weight::SemiBold, 15.0 * s, &label).max(c.atlas.measure(Weight::Medium, 12.0 * s, &v.heat_says));
        let hr = Rect::new(x + wn + 10.0 * s, bt, (60.0 * s + lw + 14.0 * s).min(sw * 0.4), bh);
        let lit = hot(hr) || focus == Some(Hit::Heat);
        c.panel(hr, 8.0 * s, if lit { BUTTON_HOVER_TOP } else { BUTTON_TOP }, if lit { BUTTON_HOVER_BOTTOM } else { BUTTON_BOTTOM }, 1.0, if on { with_alpha(BAD, 0.8) } else { [255, 255, 255, 30] });
        if focus == Some(Hit::Heat) {
            c.outline(hr.inset(-2.0 * s), 10.0 * s, 1.5 * s, ACCENT);
        }
        let knob = Rect::new(hr.x + 12.0 * s, hr.y + (bh - 20.0 * s) / 2.0, 34.0 * s, 20.0 * s);
        c.rrect(knob, 10.0 * s, if on { with_alpha(BAD, 0.85) } else { [70, 78, 90, 255] });
        let kx = if on { knob.right() - 18.0 * s } else { knob.x + 2.0 * s };
        c.rrect(Rect::new(kx, knob.y + 2.0 * s, 16.0 * s, 16.0 * s), 8.0 * s, TEXT);
        let tx = knob.right() + 12.0 * s;
        c.text_fit(Weight::SemiBold, 15.0 * s, (tx, hr.y + 17.0 * s), Align::Left, &label, hr.right() - tx - 8.0 * s, if on { BAD } else { TEXT });
        c.text_fit(Weight::Medium, 12.0 * s, (tx, hr.y + 32.0 * s), Align::Left, &v.heat_says, hr.right() - tx - 8.0 * s, MUTED);
        self.hits.push((hr, Hit::Heat));
        let we = c.atlas.measure(Weight::SemiBold, 18.0 * s, end) + 36.0 * s;
        self.item(Rect::new(hr.right() + 10.0 * s, bt, we, bh), end, "", Hit::EndRun, false, focus == Some(Hit::EndRun), c, s);
        // A word from the game (a contract kept) in the hint's place.
        let (hint, ink) = match &self.notice {
            Some(n) => (n.as_str(), GOOD),
            None => ("Click a card or a lit planet  ·  H heat  ·  Enter flies, then its shop", MUTED),
        };
        let hx = hr.right() + we + 34.0 * s;
        if sw - hx - x > 60.0 * s {
            c.text_fit(Weight::Medium, 13.5 * s, (hx, by + BOTTOM * s / 2.0 + 5.0 * s), Align::Left, hint, sw - hx - x, ink);
        }
    }

    /// The route across the top: a column a step, the slime named over
    /// it, the planets in their lanes and the lanes between. Returns the
    /// planet under the pointer (its centre, radius and node).
    fn route_map(&mut self, c: &mut Canvas, s: f32, v: &StarView, map: Rect, pick: Option<u8>, focus: Option<Hit>) -> Option<(f32, f32, f32, StarNode)> {
        let hover = self.hover;
        let n = v.steps.len().max(1);
        let at_step = v.at as usize;
        let col = map.w / n as f32;
        // Each column's head: its step and its slime (a step's planets
        // share it; each named under its planet where they don't).
        let head = 34.0 * s;
        let foot = 26.0 * s;
        let ny0 = map.y + head + 8.0 * s;
        let ny1 = map.bottom() - foot;
        let node = (NODE * s).min(col * 0.2).min((ny1 - ny0) / 7.0).max(4.0 * s);
        let at = |step: usize, lane: f32| (map.x + col * (step as f32 + 0.5), ny0 + (ny1 - ny0) * (0.5 + 0.42 * lane));
        // Names under the planets only where the lanes leave room for
        // them (at a large UI size the map is short, and the cards name
        // the planets to pick from anyway).
        let roomy = (ny1 - ny0) * 0.42 > node * 2.4 + 30.0 * s;
        let shared = |step: &[StarNode]| step.first().filter(|f| step.iter().all(|o| o.slime == f.slime)).map(|f| f.slime.trim_start_matches("The ").to_string());
        // The step ahead's column, lit.
        if at_step >= 1 && at_step <= n {
            let cx = map.x + col * (at_step as f32 - 0.5);
            let band = Rect::new(cx - col * 0.46, map.y, col * 0.92, map.h);
            c.rrect(band, 10.0 * s, [96, 190, 255, 14]);
            c.outline(band, 10.0 * s, s.max(1.0), [96, 190, 255, 40]);
        }
        for (i, step) in v.steps.iter().enumerate() {
            let (cx, _) = at(i, 0.0);
            let now = i + 1 == at_step;
            let past = i + 1 < at_step;
            let ink = if now { TEXT } else if past { with_alpha(MUTED, 0.6) } else { MUTED };
            c.text(Weight::SemiBold, 12.0 * s, (cx, map.y + 14.0 * s), Align::Centre, &(i + 1).to_string(), if now { ACCENT } else { ink });
            let name = if i + 1 == n { "Home world".to_string() } else { shared(step).unwrap_or_default() };
            c.text_fit(Weight::Medium, 13.0 * s, (cx, map.y + 30.0 * s), Align::Centre, &name, col - 6.0 * s, ink);
        }
        // The lanes: dots from each planet to those it leads to; the
        // route taken bright, the ways on from here in their planet's
        // colour, the picked one's ways on after it lit, the rest faint.
        let mut lanes: Vec<Lane> = Vec::new();
        for (i, step) in v.steps.iter().enumerate() {
            let Some(next) = v.steps.get(i + 1) else { break };
            for a in step {
                for b in next.iter().filter(|b| a.to.contains(&b.k)) {
                    let (p, q) = (at(i, a.lane), at(i + 1, b.lane));
                    let (ra, rb) = (node * pip_scale(a.pips), node * pip_scale(b.pips));
                    let (rank, ink, r) = if a.taken && b.taken {
                        (3, [220, 228, 240, 210], 2.4 * s)
                    } else if a.taken && b.open && i + 2 == at_step {
                        if Some(b.k) == pick { (4, with_alpha(pip_ink(b.pips), 0.95), 2.6 * s) } else { (2, with_alpha(pip_ink(b.pips), 0.45), 2.0 * s) }
                    } else if a.open && Some(a.k) == pick && i + 1 == at_step {
                        (1, [200, 212, 228, 120], 1.9 * s)
                    } else {
                        (0, [150, 160, 180, 36], 1.5 * s)
                    };
                    lanes.push((rank, p, q, ink, r, ra, rb));
                }
            }
        }
        lanes.sort_by_key(|l| l.0);
        for (_, p, q, ink, r, ra, rb) in lanes {
            let len = ((q.0 - p.0).powi(2) + (q.1 - p.1).powi(2)).sqrt();
            let dots = (len / (7.0 * s)).max(2.0) as usize;
            for d in 1..dots {
                let f = d as f32 / dots as f32;
                let (dx, dy) = (p.0 + (q.0 - p.0) * f, p.1 + (q.1 - p.1) * f);
                if f * len < ra + 4.0 * s || (1.0 - f) * len < rb + 4.0 * s {
                    continue;
                }
                c.rrect(Rect::new(dx - r, dy - r, 2.0 * r, 2.0 * r), r, ink);
            }
        }
        // The planets.
        let mut hover_node = None;
        for (i, step) in v.steps.iter().enumerate() {
            let named = shared(step).is_none();
            for nd in step {
                let (cx, cy) = at(i, nd.lane);
                let home = i + 1 == n;
                let rad = node * pip_scale(nd.pips) * if home { 1.5 } else { 1.0 };
                let r = Rect::new(cx - rad, cy - rad, 2.0 * rad, 2.0 * rad);
                let ink = pip_ink(nd.pips);
                let past = (i + 1) < at_step;
                let now = i + 1 == at_step;
                let picked = now && Some(nd.k) == pick;
                let fade: f32 = if nd.open || nd.taken { 1.0 } else if past { 0.22 } else { 0.55 };
                if picked {
                    c.rrect(r.inset(-8.0 * s), rad + 8.0 * s, with_alpha(ink, 0.2));
                    c.outline(r.inset(-6.0 * s), rad + 6.0 * s, 2.0 * s, ink);
                } else if (nd.open && hover.is_some_and(|p| r.inset(-8.0 * s).contains(p))) || (now && focus == Some(Hit::Star(nd.k))) {
                    c.outline(r.inset(-5.0 * s), rad + 5.0 * s, 1.5 * s, with_alpha(ink, 0.8));
                }
                // The planet just won: where the route stands.
                if nd.taken && i + 2 == at_step {
                    c.outline(r.inset(-5.0 * s), rad + 5.0 * s, 1.5 * s, [236, 240, 244, 200]);
                }
                planet(c, cx, cy, rad, &nd.biome, nd.pips, fade, s);
                // Under it: its stars once freed; the planets to pick
                // from, their names; a column of mixed slimes, each one's.
                let below = cy + rad + if nd.open && now { 22.0 } else { 14.0 } * s;
                match nd.stars {
                    Some(st) if nd.taken => {
                        let d = (11.0 * s).min(col / 3.4);
                        let x0 = cx - 1.5 * d;
                        for k in 0..3u8 {
                            let (icon, ink) = if k < st { (Icon::Star, WARN) } else { (Icon::StarOpen, [255, 255, 255, 70]) };
                            c.icon(icon, (x0 + k as f32 * d, cy + rad + 4.0 * s), d, ink);
                        }
                    }
                    _ if nd.open && now && roomy => {
                        c.text_fit(Weight::SemiBold, 12.5 * s, (cx, below), Align::Centre, &nd.name, col - 4.0 * s, if picked { TEXT } else { with_alpha(TEXT, 0.8) });
                    }
                    _ if named && !past && roomy => {
                        c.text_fit(Weight::Medium, 11.5 * s, (cx, below), Align::Centre, nd.slime.trim_start_matches("The "), col - 4.0 * s, with_alpha(MUTED, fade.max(0.7)));
                    }
                    _ => {}
                }
                if nd.open && now {
                    self.hits.push((r.inset(-8.0 * s), Hit::Star(nd.k)));
                }
                if hover.is_some_and(|p| r.inset(-6.0 * s).contains(p)) {
                    hover_node = Some((cx, cy, rad, nd.clone()));
                }
            }
        }
        hover_node
    }

    /// One planet to pick from, as a card: what it is, how it differs,
    /// what it pays.
    #[allow(clippy::too_many_arguments)]
    fn star_card(&mut self, c: &mut Canvas, s: f32, v: &StarView, nd: &StarNode, r: Rect, picked: bool, lit: bool, focused: bool, most: f32) {
        let ink = pip_ink(nd.pips);
        c.shadow(r, 14.0 * s, 20.0 * s, 6.0 * s, SHADOW);
        let (t, b) = if lit && !picked { (BUTTON_HOVER_TOP, PANEL_BOTTOM) } else { (PANEL_TOP, PANEL_BOTTOM) };
        c.panel(r, 14.0 * s, t, b, 1.0, if picked { with_alpha(ink, 0.9) } else { EDGE });
        if picked {
            c.outline(r.inset(-3.0 * s), 17.0 * s, 2.0 * s, ink);
        } else if focused {
            c.outline(r.inset(-3.0 * s), 17.0 * s, 1.5 * s, ACCENT);
        }
        // Its colour along the top.
        c.rrect(Rect::new(r.x + 18.0 * s, r.y, r.w - 36.0 * s, 4.0 * s), 2.0 * s, ink);
        let p = 20.0 * s;
        let px = r.x + p;
        let tw = r.w - 2.0 * p;
        // Its planet at the top right.
        let pr = 21.0 * s;
        planet(c, r.right() - p - pr, r.y + 24.0 * s + pr, pr, &nd.biome, nd.pips, 1.0, s);
        let nw = tw - 2.0 * pr - 12.0 * s;
        let mut y = r.y + 34.0 * s;
        c.text(Weight::SemiBold, 14.0 * s, (px, y), Align::Left, &pip_name(nd.pips).to_uppercase(), ink);
        y += 30.0 * s;
        c.text_fit(Weight::SemiBold, 24.0 * s, (px, y), Align::Left, &nd.name, nw, TEXT);
        // Its pip's line in words, where the card has its full height (the
        // facts under it say the same in numbers, and come first).
        if r.h >= CARD_H * s - 1.0 {
            y += 22.0 * s;
            c.text_fit(Weight::Medium, 13.5 * s, (px, y), Align::Left, &nd.about, tw, MUTED);
        }
        y += 14.0 * s;
        c.rrect(Rect::new(px, y, tw, s.max(1.0)), 0.0, [255, 255, 255, 22]);
        y += 24.0 * s;

        // The pay at the foot; the facts fill what is left above it.
        let foot = r.bottom() - 20.0 * s;
        let limit = foot - 34.0 * s;
        let lw = 62.0 * s;
        let vx = px + lw;
        let vw = tw - lw;
        let row = 23.0 * s;
        // A fact: its label, its words, and a note at the right.
        let fact = |c: &mut Canvas, y: f32, label: &str, words: &str, note: Option<(&str, Rgba)>, ink: Rgba| {
            c.text(Weight::Medium, 13.0 * s, (px, y), Align::Left, label, MUTED);
            let nw = note.map_or(0.0, |(n, k)| c.text(Weight::SemiBold, 13.5 * s, (px + tw, y), Align::Right, n, k) + 10.0 * s);
            c.text_fit(Weight::Medium, 14.5 * s, (vx, y), Align::Left, words, vw - nw, ink);
        };
        // The slime: its name, its size on the last planet's and a bar.
        if y + 14.0 * s <= limit {
            let note = v.last_size.map(|l| change(nd.size, l));
            fact(c, y, "Slime", &nd.slime, note.as_ref().map(|(t, k)| (t.as_str(), *k)), TEXT);
            let bar = Rect::new(vx, y + 8.0 * s, vw, 5.0 * s);
            c.bar(bar, nd.size / most, with_alpha(ink, 0.85), [255, 255, 255, 18]);
            if let Some(l) = v.last_size {
                // The last planet's size, a tick on the bar.
                let tx = bar.x + bar.w * (l / most).clamp(0.0, 1.0);
                c.rrect(Rect::new(tx - 1.0 * s, bar.y - 3.0 * s, 2.0 * s, bar.h + 6.0 * s), s, [236, 240, 244, 200]);
            }
            y += row + 12.0 * s;
        }
        if y <= limit {
            fact(c, y, "Tough", &format!("each bit takes {} hit{}", hits(nd.hits), if nd.hits < 1.05 { "" } else { "s" }), None, TEXT);
            y += row;
        }
        // The twist: its name, then its line in up to two lines.
        if y <= limit {
            match &nd.twist {
                Some((name, line)) => {
                    fact(c, y, "Twist", name, None, crate::world::ui::TWIST);
                    y += 19.0 * s;
                    for l in c.wrap(Weight::Medium, 13.0 * s, vw, line).into_iter().take(2) {
                        if y > limit + 4.0 * s {
                            break;
                        }
                        c.text(Weight::Medium, 13.0 * s, (vx, y), Align::Left, &l, MUTED);
                        y += 17.0 * s;
                    }
                    y += 6.0 * s;
                }
                None => {
                    fact(c, y, "Twist", "none", None, MUTED);
                    y += row;
                }
            }
        }
        if nd.pips >= 3 && y <= limit {
            c.icon(Icon::Star, (px, y - 12.0 * s), 14.0 * s, WARN);
            c.text_fit(Weight::Medium, 14.0 * s, (vx, y), Align::Left, "A rare card waits in its shop", vw, WARN);
            y += row;
        }
        // The bonus goals (contracts): each with its pay.
        if !nd.contracts.is_empty() && y + 4.0 * s <= limit {
            y += 4.0 * s;
            c.text(Weight::Medium, 13.0 * s, (px, y), Align::Left, "Bonus", MUTED);
            for (name, pay) in &nd.contracts {
                if y > limit {
                    break;
                }
                let pw = c.text(Weight::SemiBold, 14.0 * s, (px + tw, y), Align::Right, &format!("+{pay}"), ACCENT);
                c.text_fit(Weight::Medium, 14.0 * s, (vx, y), Align::Left, name, vw - pw - 10.0 * s, TEXT);
                y += 20.0 * s;
            }
            y += 4.0 * s;
        }
        // The world and its map's size.
        if y <= limit {
            let note = v.last_area.map(|l| change(nd.area, l));
            let world = format!("{}{}", nd.biome[..1].to_uppercase(), &nd.biome[1..]);
            fact(c, y, "World", &world, note.as_ref().map(|(t, k)| (format!("map {t}"), *k)).as_ref().map(|(t, k)| (t.as_str(), *k)), TEXT);
            y += row;
        }
        // Where it leads: the next step's planets, by difficulty.
        let next = v.steps.iter().position(|st| st.iter().any(|o| o.k == nd.k && o.open)).and_then(|i| v.steps.get(i + 1));
        if let Some(next) = next.filter(|_| y <= limit) {
            c.text(Weight::Medium, 13.0 * s, (px, y), Align::Left, "Then", MUTED);
            let mut tx = vx;
            let mut to: Vec<&StarNode> = next.iter().filter(|o| nd.to.contains(&o.k)).collect();
            to.sort_by_key(|o| o.pips);
            for (k, o) in to.iter().enumerate() {
                if k > 0 {
                    tx += c.text(Weight::Medium, 14.0 * s, (tx, y), Align::Left, if k + 1 == to.len() { " or " } else { ", " }, MUTED);
                }
                tx += c.text(Weight::SemiBold, 14.0 * s, (tx, y), Align::Left, pip_name(o.pips), pip_ink(o.pips));
            }
        }
        // The pay.
        let pay = if picked && self.heat { nd.heat_pay } else { nd.pay };
        let w = c.text(Weight::SemiBold, 24.0 * s, (px, foot), Align::Left, &format!("+{pay}"), if picked && self.heat { BAD } else { ACCENT });
        let says = if picked && self.heat { "points for a two-star win, with heat" } else { "points for a two-star win" };
        c.text_fit(Weight::Medium, 13.5 * s, (px + w + 10.0 * s, foot - 1.0 * s), Align::Left, says, tw - w - 10.0 * s, MUTED);
    }
}
