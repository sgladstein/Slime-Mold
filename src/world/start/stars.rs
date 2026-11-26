//! **The run's star map** (2026-11-26, `mould::route`; the owner's pick of
//! a route with forks and contracts, 11-25 23:45Z): one screen between a
//! won planet and its shop. The run's twelve steps run left to right, the
//! landing world first and the home world last; each step's planets sit
//! in lanes down the map, joined to the next step's by dotted lanes. The
//! planets the route can go to next are lit; a click picks one and the
//! pane at the right tells it (its slime, biome, twist, size, pay and
//! contracts) with the heat switch, and "Fly there" takes the route on to
//! its shop.
//!
//! The pips are the planet's difficulty, drawn as one to three dots in
//! its ring (Safe green, Fair blue, Hard amber), on every node of the map,
//! so a route can be read ahead (his "clearly different difficulties").

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
    /// The slime's size as a factor on the first planet's.
    pub size: f32,
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
    pub purse: String,
    /// What heat does, for the switch's line.
    pub heat_says: String,
}

/// The map's bars, UI units (as the shop's).
const TOP: f32 = 46.0;
const BOTTOM: f32 = 62.0;
/// A node's ring, and the pane's width.
const NODE: f32 = 30.0;
const PANE: f32 = 380.0;

/// The pip colours: Safe, Fair, Hard.
pub(crate) const PIP_INKS: [Rgba; 3] = [GOOD, ACCENT, WARN];

fn pip_ink(pips: u8) -> Rgba {
    PIP_INKS[(pips.clamp(1, 3) - 1) as usize]
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

    /// The star map, drawn whole on one screen.
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

        // The top bar: the heading and the points.
        let top = TOP * s;
        c.rrect(Rect::new(0.0, 0.0, sw, top), 0.0, [8, 11, 14, 236]);
        c.rrect(Rect::new(0.0, top - s.max(1.0), sw, s.max(1.0)), 0.0, EDGE);
        let purse_w = c.text(Weight::SemiBold, 20.0 * s, (sw - x, 30.0 * s), Align::Right, &v.purse, ACCENT);
        c.text_fit(Weight::SemiBold, 18.0 * s, (x, 30.0 * s), Align::Left, &v.heading, sw - 2.0 * x - purse_w - 24.0 * s, TEXT);

        // The map, left of the pane.
        let pane_w = (PANE * s).min(sw * 0.4);
        let by = sh - BOTTOM * s;
        let map = Rect::new(x, top + 16.0 * s, sw - 3.0 * x - pane_w, by - top - 32.0 * s);
        let n = v.steps.len().max(1);
        let col = map.w / n as f32;
        let node = (NODE * s).min(col * 0.62).min(map.h / 7.0);
        let at = |step: usize, lane: f32| (map.x + col * (step as f32 + 0.5), map.y + map.h * (0.5 + 0.36 * lane) + 4.0 * s);
        // The step numbers along the foot of the map.
        for i in 0..n {
            let (cx, _) = at(i, 0.0);
            let ink = if i + 1 == v.at as usize { TEXT } else { MUTED };
            c.text(Weight::Medium, 12.5 * s, (cx, map.bottom() - 2.0 * s), Align::Centre, &(i + 1).to_string(), ink);
        }
        // The lanes: dots from each planet to those it leads to; the
        // route taken bright, the ways on from here lit, the rest faint.
        for (i, step) in v.steps.iter().enumerate() {
            let Some(next) = v.steps.get(i + 1) else { break };
            for a in step {
                for b in next.iter().filter(|b| a.to.contains(&b.k)) {
                    let (p, q) = (at(i, a.lane), at(i + 1, b.lane));
                    let ink = if a.taken && b.taken {
                        [220, 228, 240, 200]
                    } else if a.taken && b.open && i + 2 == v.at as usize {
                        if Some(b.k) == pick { with_alpha(pip_ink(b.pips), 0.95) } else { [200, 212, 228, 120] }
                    } else {
                        [150, 160, 180, 40]
                    };
                    let len = ((q.0 - p.0).powi(2) + (q.1 - p.1).powi(2)).sqrt();
                    let dots = (len / (7.0 * s)).max(2.0) as usize;
                    let r = if a.taken && (b.taken || b.open) { 2.4 * s } else { 1.7 * s };
                    for d in 1..dots {
                        let f = d as f32 / dots as f32;
                        let (dx, dy) = (p.0 + (q.0 - p.0) * f, p.1 + (q.1 - p.1) * f);
                        if ((dx - p.0).powi(2) + (dy - p.1).powi(2)).sqrt() < node * 0.6 || ((dx - q.0).powi(2) + (dy - q.1).powi(2)).sqrt() < node * 0.6 {
                            continue;
                        }
                        c.rrect(Rect::new(dx - r, dy - r, 2.0 * r, 2.0 * r), r, ink);
                    }
                }
            }
        }
        // The planets.
        let mut hover_node: Option<(usize, StarNode)> = None;
        for (i, step) in v.steps.iter().enumerate() {
            for nd in step {
                let (cx, cy) = at(i, nd.lane);
                let r = Rect::new(cx - node / 2.0, cy - node / 2.0, node, node);
                let ink = pip_ink(nd.pips);
                let past = (i + 1) < v.at as usize;
                let picked = i + 1 == v.at as usize && Some(nd.k) == pick;
                let lit = nd.open || nd.taken || i + 1 >= v.at as usize;
                let fade: f32 = if nd.open || nd.taken { 1.0 } else if past { 0.25 } else if lit { 0.55 } else { 0.3 };
                if picked {
                    c.rrect(r.inset(-7.0 * s), node / 2.0 + 7.0 * s, with_alpha(ink, 0.18));
                    c.outline(r.inset(-5.0 * s), node / 2.0 + 5.0 * s, 2.0 * s, ink);
                } else if nd.open && hot(r.inset(-6.0 * s)) {
                    c.outline(r.inset(-4.0 * s), node / 2.0 + 4.0 * s, 1.5 * s, with_alpha(ink, 0.7));
                }
                if focus == Some(Hit::Star(nd.k)) && i + 1 == v.at as usize && !picked {
                    c.outline(r.inset(-4.0 * s), node / 2.0 + 4.0 * s, 1.5 * s, ACCENT);
                }
                let body = if nd.taken { [34, 40, 50, 255] } else { [18, 22, 30, 255] };
                c.rrect(r, node / 2.0, with_alpha(body, fade.max(0.6)));
                c.outline(r, node / 2.0, (if nd.open { 2.0 } else { 1.4 }) * s, with_alpha(ink, fade));
                // The pips: one to three dots across the ring.
                let pr = (node * 0.1).max(1.6 * s);
                let gap = pr * 2.7;
                let w0 = cx - gap * (nd.pips as f32 - 1.0) / 2.0;
                for p in 0..nd.pips {
                    let px = w0 + gap * p as f32;
                    c.rrect(Rect::new(px - pr, cy - pr, 2.0 * pr, 2.0 * pr), pr, with_alpha(ink, fade));
                }
                // Freed: its stars under it; here and ahead: its slime.
                match nd.stars {
                    Some(st) if nd.taken => {
                        let d = (12.0 * s).min(col / 3.2);
                        let x0 = cx - 1.5 * d;
                        for k in 0..3u8 {
                            let (icon, ink) = if k < st { (Icon::Star, WARN) } else { (Icon::StarOpen, [255, 255, 255, 70]) };
                            c.icon(icon, (x0 + k as f32 * d, cy + node / 2.0 + 4.0 * s), d, ink);
                        }
                    }
                    _ if lit && !past && col > 34.0 * s => {
                        c.text_fit(Weight::Medium, 12.0 * s, (cx, cy + node / 2.0 + 15.0 * s), Align::Centre, nd.slime.trim_start_matches("The "), col - 4.0 * s, with_alpha(MUTED, fade.max(0.7)));
                    }
                    _ => {}
                }
                if nd.open && i + 1 == v.at as usize {
                    self.hits.push((r.inset(-6.0 * s), Hit::Star(nd.k)));
                }
                if hot(r.inset(-4.0 * s)) {
                    hover_node = Some((i, nd.clone()));
                }
            }
        }
        // Hovering a planet off the route names it.
        if let Some((i, nd)) = hover_node.filter(|(_, nd)| !(nd.open && Some(nd.k) == pick)) {
            let (cx, cy) = at(i, nd.lane);
            let tip = format!("{} · {} · {}", nd.name, nd.slime, ["Safe", "Fair", "Hard"][(nd.pips.clamp(1, 3) - 1) as usize]);
            let tw = c.atlas.measure(Weight::Medium, 13.5 * s, &tip) + 20.0 * s;
            let tx = (cx - tw / 2.0).clamp(x, sw - x - tw);
            let tr = Rect::new(tx, cy - node / 2.0 - 36.0 * s, tw, 26.0 * s);
            c.panel(tr, 7.0 * s, PANEL_TOP, PANEL_BOTTOM, 1.0, EDGE);
            c.text(Weight::Medium, 13.5 * s, (tr.x + 10.0 * s, tr.y + 17.5 * s), Align::Left, &tip, TEXT);
        }

        // The pane: the planet picked.
        let pane = Rect::new(sw - x - pane_w, top + 16.0 * s, pane_w, by - top - 32.0 * s);
        c.shadow(pane, 14.0 * s, 24.0 * s, 6.0 * s, SHADOW);
        c.panel(pane, 14.0 * s, PANEL_TOP, PANEL_BOTTOM, 1.0, EDGE);
        self.hits.push((pane, Hit::Panel));
        let px = pane.x + 20.0 * s;
        let tw = pane.w - 40.0 * s;
        if let Some(nd) = here.iter().find(|n| Some(n.k) == pick) {
            let ink = pip_ink(nd.pips);
            let mut y = pane.y + 34.0 * s;
            let kind = ["Safe", "Fair", "Hard"][(nd.pips.clamp(1, 3) - 1) as usize];
            c.text_fit(Weight::Medium, 13.5 * s, (px, y), Align::Left, &format!("{kind} · {}", nd.biome), tw, ink);
            y += 30.0 * s;
            c.text_fit(Weight::SemiBold, 24.0 * s, (px, y), Align::Left, &nd.name, tw, TEXT);
            y += 26.0 * s;
            c.text_fit(Weight::Medium, 15.5 * s, (px, y), Align::Left, &format!("{}, {:.1} times the first planet's slime", nd.slime, nd.size), tw, TEXT);
            y += 22.0 * s;
            let lines = c.paragraph(Weight::Medium, 14.0 * s, (px, y), tw, 19.0 * s, &nd.about, MUTED);
            y += lines as f32 * 19.0 * s + 10.0 * s;
            if let Some((name, line)) = &nd.twist {
                c.text_fit(Weight::SemiBold, 14.5 * s, (px, y), Align::Left, name, tw, [140, 200, 255, 255]);
                y += 19.0 * s;
                let lines = c.paragraph(Weight::Medium, 13.5 * s, (px, y), tw, 18.0 * s, line, MUTED);
                y += lines as f32 * 18.0 * s + 10.0 * s;
            }
            // The pay, then the contracts.
            let pay = if self.heat { nd.heat_pay } else { nd.pay };
            c.text_fit(Weight::SemiBold, 15.5 * s, (px, y), Align::Left, &format!("Pays {pay} points for a two-star win"), tw, ACCENT);
            y += 28.0 * s;
            if !nd.contracts.is_empty() {
                c.text(Weight::SemiBold, 14.5 * s, (px, y), Align::Left, "Contracts, if you take them on the way:", TEXT);
                y += 21.0 * s;
                for (name, p) in &nd.contracts {
                    let pw = c.text(Weight::SemiBold, 14.0 * s, (pane.right() - 20.0 * s, y), Align::Right, &format!("+{p}"), ACCENT);
                    c.text_fit(Weight::Medium, 14.0 * s, (px + 12.0 * s, y), Align::Left, name, tw - pw - 20.0 * s, TEXT);
                    c.rrect(Rect::new(px, y - 7.0 * s, 5.0 * s, 5.0 * s), 2.5 * s, ink);
                    y += 20.0 * s;
                }
                y += 8.0 * s;
            }
            // Heat: a switch.
            let hr = Rect::new(px, y.min(pane.bottom() - 74.0 * s), tw, 54.0 * s);
            let on = self.heat;
            let lit = hot(hr) || focus == Some(Hit::Heat);
            c.panel(hr, 10.0 * s, if lit { BUTTON_HOVER_TOP } else { BUTTON_TOP }, if lit { BUTTON_HOVER_BOTTOM } else { BUTTON_BOTTOM }, 1.0, if on { with_alpha(BAD, 0.8) } else { [255, 255, 255, 30] });
            if focus == Some(Hit::Heat) {
                c.outline(hr.inset(-2.0 * s), 12.0 * s, 1.5 * s, ACCENT);
            }
            let knob = Rect::new(hr.x + 14.0 * s, hr.y + 17.0 * s, 34.0 * s, 20.0 * s);
            c.rrect(knob, 10.0 * s, if on { with_alpha(BAD, 0.85) } else { [70, 78, 90, 255] });
            let kx = if on { knob.right() - 18.0 * s } else { knob.x + 2.0 * s };
            c.rrect(Rect::new(kx, knob.y + 2.0 * s, 16.0 * s, 16.0 * s), 8.0 * s, TEXT);
            let tx = knob.right() + 12.0 * s;
            c.text_fit(Weight::SemiBold, 15.5 * s, (tx, hr.y + 23.0 * s), Align::Left, if on { "Heat on" } else { "Heat off" }, hr.right() - tx - 10.0 * s, if on { BAD } else { TEXT });
            c.text_fit(Weight::Medium, 12.5 * s, (tx, hr.y + 41.0 * s), Align::Left, &v.heat_says, hr.right() - tx - 10.0 * s, MUTED);
            self.hits.push((hr, Hit::Heat));
        } else {
            c.paragraph(Weight::Medium, 15.0 * s, (px, pane.y + 40.0 * s), tw, 21.0 * s, "Pick a lit planet on the map.", MUTED);
        }

        // The bottom bar: fly there, give up.
        c.rrect(Rect::new(0.0, by, sw, sh - by), 0.0, [8, 11, 14, 236]);
        c.rrect(Rect::new(0.0, by, sw, s.max(1.0)), 0.0, EDGE);
        let next = match here.iter().find(|n| Some(n.k) == pick) {
            Some(nd) => format!("Fly to {}", nd.name),
            None => "Pick a planet".to_string(),
        };
        let end = "Give up the run";
        let bh = 40.0 * s;
        let bt = by + (BOTTOM * s - bh) / 2.0;
        let wn = c.atlas.measure(Weight::SemiBold, 18.0 * s, &next) + 36.0 * s;
        let we = c.atlas.measure(Weight::SemiBold, 18.0 * s, end) + 36.0 * s;
        self.item(Rect::new(x, bt, wn, bh), &next, "", Hit::Next, true, focus == Some(Hit::Next), c, s);
        self.item(Rect::new(x + wn + 10.0 * s, bt, we, bh), end, "", Hit::EndRun, false, focus == Some(Hit::EndRun), c, s);
        // A word from the game (a contract kept) in the hint's place.
        let (hint, ink) = match &self.notice {
            Some(n) => (n.as_str(), GOOD),
            None => ("Click a lit planet, or Up/Down  ·  H heat  ·  Enter flies  ·  its shop is next", MUTED),
        };
        let hx = x + wn + we + 34.0 * s;
        c.text_fit(Weight::Medium, 13.5 * s, (hx, by + BOTTOM * s / 2.0 + 5.0 * s), Align::Left, hint, sw - hx - x, ink);
    }
}
