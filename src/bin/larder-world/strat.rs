//! **The strategy bot** (2026-11-19, the thread "Build a smarter bot that
//! explores strategies"; the owner, 11-18 23:58Z: "There are probably tons
//! of different strategies that I have not come up with yet", and 23:59Z:
//! "a variety of bots that play with different strategies or a bot that can
//! switch/test different strategies").
//!
//! One hand, `hand=strat:PLAN`, whose play is set by a plan of dials, so
//! each plan is its own bot and a search can walk the dials. A plan is a
//! preset name and/or `key=value` dials joined by commas:
//! `strat:turtle,go=40,lure=1`. `Plan::DIALS` lists every dial; `PRESETS`
//! the named families. With no dials it plays as the run bot does
//! (`hands::bot_hand`): drain crystals, buy soldiers, hunt.
//!
//! What it adds to the run bot, each a dial:
//! - **an economy**: pumps on the best quiet crystals near the landing
//!   (`pumps`, `walk`), upgraded (`up`), and more pumps on crystals the
//!   squad takes while out (`more`);
//! - **a base**: turrets round the first `guard` pumps on the slime's
//!   side (`turrets`), a wall arc in front (`walls`), a lure past the
//!   turrets so calm slime walks into their fire (`lure`);
//! - **timing**: soldiers bought before any building (`hold`), soldiers
//!   before the squad leaves home (`go`), the most it keeps (`cap`),
//!   whether it goes back to a raided pump (`defend`), when it steps back
//!   (`back`), and how thin the slime by its home must be before it walks
//!   in (`per`);
//! - **the shop** between planets (`shop`, `cards`), read by the league
//!   (`league.rs`), not by the hand.
//!
//! It never changes the game's numbers: it plays through `Game::apply`
//! as the window does, and builds only what the run has unlocked (`own`,
//! which the league fills from the run; walls alone, as a run starts,
//! when not given).

use super::*;
use std::cell::RefCell;

/// A plan: the dials one strategy is played by.
#[derive(Clone, Debug, PartialEq)]
pub(super) struct Plan {
    /// Pumps built from home, on the best quiet crystals near it.
    pub pumps: u32,
    /// How far a pump's crystal may be for its charge, in gun ranges: a
    /// crystal's worth is its charge over 1 + walk / (`walk` ranges).
    pub walk: f32,
    /// Upgrades bought on each pump (0 to 2).
    pub up: u32,
    /// Pumps, of the first, given turrets, walls and a lure.
    pub guard: u32,
    /// Turrets round each guarded pump, on the slime's side.
    pub turrets: u32,
    /// A wall in front of each guarded pump: 0 none, 1 an arc, 2 a wider
    /// arc (two thirds round).
    pub walls: u32,
    /// A lure past each guarded pump's turrets.
    pub lure: bool,
    /// Defences of a pump before the next pump (`true`), or every pump
    /// first (`false`).
    pub guard_first: bool,
    /// Soldiers bought before anything is built.
    pub hold: u32,
    /// Soldiers before the squad leaves home to work and hunt.
    pub go: u32,
    /// The most soldiers it buys.
    pub cap: u32,
    /// Health (share of full) at which a bitten squad steps back.
    pub back: f32,
    /// The slime by its home crystal, walkers a soldier, under which the
    /// squad walks in (the run bot's 200).
    pub per: f32,
    /// Out of home, the squad goes back to a guarded pump with slime on it.
    pub defend: bool,
    /// Pumps put on crystals the squad takes while out (beyond `pumps`).
    pub more: u32,
    /// Out of home: 0 drain crystals by hand as the run bot does, 1 hunt
    /// the slime only.
    pub take: u32,
    /// The shop's items in the order bought, one level of each a pass
    /// (`runbot::WANTS` keys).
    pub shop: Vec<String>,
    /// Gun cards: 0 none, 1 the card the planet asks for (`run::bring`),
    /// 2 that and then any offer the points and the gun's budget allow.
    pub cards: u32,
    /// The unlocks the run owns (`Run::has` keys); None: walls only.
    pub own: Option<Vec<String>>,
}

impl Default for Plan {
    /// The run bot's play (`hands::bot_hand`).
    fn default() -> Self {
        Plan {
            pumps: 0,
            walk: 4.0,
            up: 0,
            guard: 0,
            turrets: 0,
            walls: 0,
            lure: false,
            guard_first: true,
            hold: 0,
            go: 30,
            cap: 80,
            back: 0.5,
            per: 200.0,
            defend: false,
            more: 0,
            take: 0,
            shop: ["strike", "rate", "hp", "squad", "harvest", "range", "bank"].map(String::from).to_vec(),
            cards: 1,
            own: None,
        }
    }
}

/// Named families, each a starting point the search moves from: the dials
/// after the name win.
pub(super) const PRESETS: [(&str, &str); 8] = [
    // The run bot as it is (drain, buy, hunt).
    ("bot", ""),
    // Everything into soldiers and straight at the slime.
    ("rush", "go=0,cap=200,shop=strike/rate/hp/squad/range"),
    // Pumps and their upgrades first, a small squad until the income is up.
    ("econ", "pumps=4,up=2,hold=4,go=40,more=4,shop=pump+/bank/squad/strike/rate/hp"),
    // Pumps behind turrets, walls and a lure; the squad stays home long.
    ("turtle", "pumps=2,guard=2,turrets=3,walls=1,lure=1,hold=4,go=60,defend=1,more=2,shop=turret/turret+/pump+/strike/rate/hp"),
    // The owner's planet 3 (11-18 22:10Z): a pump, four turrets round it,
    // walls, a second pump, soldiers from 2:10, out at about 40.
    ("scott", "pumps=3,guard=1,turrets=4,walls=1,hold=4,go=40,defend=1,more=2,shop=turret/strike/rate/hp/pump+"),
    // A kill zone: turrets and a lure, no walls.
    ("lure", "pumps=2,guard=1,turrets=4,lure=1,hold=4,go=50,defend=1,more=2,shop=turret/turret+/strike/rate/hp"),
    // Pumps, no defences, a big squad early that takes crystals.
    ("boom", "pumps=2,hold=2,go=20,more=6,cap=120,shop=pump+/squad/strike/rate/hp"),
    // The run bot with pumps on what it takes in place of draining.
    ("taker", "more=8,go=20,shop=strike/rate/hp/pump+/squad"),
];

impl Plan {
    /// Every dial's key, for the search and the docs.
    pub const DIALS: [&'static str; 20] = ["pumps", "walk", "up", "guard", "turrets", "walls", "lure", "first", "hold", "go", "cap", "back", "per", "defend", "more", "take", "shop", "cards", "own", "name"];

    /// `PRESET,key=value,...` (either part may be missing).
    pub fn parse(spec: &str) -> Result<Plan, String> {
        let mut p = Plan::default();
        for w in spec.split(',').map(str::trim).filter(|w| !w.is_empty()) {
            match w.split_once('=') {
                None => {
                    let (_, dials) = PRESETS.iter().find(|x| x.0 == w).ok_or_else(|| format!("strat: no preset {w} ({})", PRESETS.map(|x| x.0).join(", ")))?;
                    let base = Plan::parse(dials)?;
                    p = Plan { own: p.own.take().or(base.own.clone()), ..base };
                }
                Some((k, v)) => p.set(k, v)?,
            }
        }
        Ok(p)
    }

    fn set(&mut self, k: &str, v: &str) -> Result<(), String> {
        let num = |v: &str| v.parse::<f32>().map_err(|e| format!("strat: {k}={v}: {e}"));
        let whole = |v: &str| num(v).map(|x| x.max(0.0).round() as u32);
        match k {
            "pumps" => self.pumps = whole(v)?,
            "walk" => self.walk = num(v)?.max(0.1),
            "up" => self.up = whole(v)?.min(2),
            "guard" => self.guard = whole(v)?,
            "turrets" => self.turrets = whole(v)?.min(8),
            "walls" => self.walls = whole(v)?.min(2),
            "lure" => self.lure = num(v)? > 0.0,
            "first" => self.guard_first = v == "guard",
            "hold" => self.hold = whole(v)?,
            "go" => self.go = whole(v)?,
            "cap" => self.cap = whole(v)?.max(1),
            "back" => self.back = num(v)?.clamp(0.0, 1.0),
            "per" => self.per = num(v)?.max(1.0),
            "defend" => self.defend = num(v)? > 0.0,
            "more" => self.more = whole(v)?,
            "take" => self.take = whole(v)?.min(1),
            "shop" => self.shop = v.split('/').filter(|s| !s.is_empty()).map(String::from).collect(),
            "cards" => self.cards = whole(v)?.min(2),
            "own" => self.own = Some(v.split('/').filter(|s| !s.is_empty()).map(String::from).collect()),
            // A label for the league's rows; the hand ignores it.
            "name" => {}
            _ => return Err(format!("strat: no dial {k} ({})", Plan::DIALS.join(", "))),
        }
        Ok(())
    }

    /// The run owns unlock `key` (`Run::has`): walls alone when not given.
    pub fn owns(&self, key: &str) -> bool {
        match &self.own {
            Some(o) => o.iter().any(|x| x == key),
            None => key == "walls",
        }
    }
}

/// One building the plan wants, and the building that stands for it.
#[derive(Clone, Copy, Debug)]
struct Site {
    kind: Kind,
    at: (f32, f32),
    /// `Game::buildings` index once placed.
    b: Option<usize>,
    /// Times placed: a site lost three times is given up.
    tries: u8,
}

#[derive(Default)]
struct Memory {
    spec: String,
    plan: Plan,
    /// Home: the first pump's crystal, else the landing.
    home: Option<(f32, f32)>,
    /// Pumps placed, by crystal, in order; each one's defences.
    pumps: Vec<(usize, Vec<Site>)>,
    /// When the bank started saving for the next building.
    saving: Option<f32>,
    /// The run bot's crystal memory: the crystal worked, its harvest when
    /// picked and when; crystals left until when; crystals ever left.
    work: Option<(usize, f32, f32)>,
    left: Vec<(usize, f32)>,
    barren: Vec<usize>,
    /// Crystals a pump was refused on.
    refused: Vec<usize>,
    /// Crystals whose pump was eaten, until when they wait.
    eaten: Vec<(usize, f32)>,
    /// The ghost the squad is raising and since when: one not up in 40 s
    /// is called off (its price comes back).
    raising: Option<(usize, f32)>,
    /// Soldiers ever bought and the most alive, for the league's row.
    pub bought: u32,
    pub peak: usize,
    /// Seconds the squad spent home, out, building and stepping back.
    pub secs: [u32; 4],
}

thread_local! {
    static MEM: RefCell<Memory> = RefCell::new(Memory::default());
}

/// What the league reads back after a game: soldiers bought, the most
/// alive, and the seconds home, out, building and stepping back.
pub(super) fn tally() -> (u32, usize, [u32; 4]) {
    MEM.with_borrow(|m| (m.bought, m.peak, m.secs))
}

/// Slime squares (`mould_grid`) round a point and on a line, shared by
/// the volley, the hunt and the defences.
pub(super) struct Sense {
    grid: Vec<u32>,
    bin: usize,
    gw: usize,
    w: usize,
    set: Vec<u32>,
}

impl Sense {
    pub(super) fn of(g: &Game<GpuWorld>) -> Sense {
        let sc = g.world.scale();
        let bin = (4.0 * sc).round().max(1.0) as usize;
        let (w, _) = g.world.size();
        Sense { grid: g.world.mould_grid(bin).unwrap_or_default(), bin, gw: w.div_ceil(bin), w, set: g.world.set_cells().unwrap_or_default() }
    }

    pub(super) fn centre(&self, i: usize) -> (f32, f32) {
        (((i % self.gw) as f32 + 0.5) * self.bin as f32, ((i / self.gw) as f32 + 0.5) * self.bin as f32)
    }

    /// Walkers in the squares within `r` of `p`.
    pub(super) fn mass_near(&self, p: (f32, f32), r: f32) -> u32 {
        self.grid.iter().enumerate().filter(|&(i, _)| dist(self.centre(i), p) <= r).map(|(_, &c)| c).sum()
    }

    /// The slime's middle, by weight.
    pub(super) fn middle(&self) -> Option<(f32, f32)> {
        let (mut sx, mut sy, mut n) = (0.0f64, 0.0f64, 0.0f64);
        for (i, &c) in self.grid.iter().enumerate().filter(|&(_, &c)| c >= 8) {
            let p = self.centre(i);
            sx += f64::from(p.0) * f64::from(c);
            sy += f64::from(p.1) * f64::from(c);
            n += f64::from(c);
        }
        (n > 0.0).then(|| ((sx / n) as f32, (sy / n) as f32))
    }

    /// A line of fire over open ground with no set walker on it (the run
    /// bot's `clear`).
    pub(super) fn clear(&self, g: &Game<GpuWorld>, a: (f32, f32), b: (f32, f32)) -> bool {
        let n = dist(a, b).ceil().max(1.0) as usize;
        (0..=n).all(|i| {
            let f = i as f32 / n as f32;
            let (x, y) = (a.0 + (b.0 - a.0) * f, a.1 + (b.1 - a.1) * f);
            g.ground(x, y) && (x < 0.0 || y < 0.0 || x as usize >= self.w || self.set.get(y as usize * self.w + x as usize).is_none_or(|&c| c == 0))
        })
    }

    /// The heaviest slime squares weighed by the walk from `from`, at most 40.
    pub(super) fn ranked(&self, from: (f32, f32), range: f32) -> Vec<((f32, f32), f32)> {
        let mut v: Vec<((f32, f32), f32, f32)> = self
            .grid
            .iter()
            .enumerate()
            .filter(|&(_, &c)| c >= 8)
            .map(|(i, &c)| {
                let p = self.centre(i);
                let d = dist(p, from);
                (p, d, c as f32 / (1.0 + d / (2.0 * range.max(1.0))))
            })
            .collect();
        v.sort_by(|a, b| b.2.total_cmp(&a.2));
        v.truncate(40);
        v.into_iter().map(|(p, d, _)| (p, d)).collect()
    }
}

pub(super) fn dist(a: (f32, f32), b: (f32, f32)) -> f32 {
    (a.0 - b.0).hypot(a.1 - b.1)
}

/// The nearest point to `p` where a `kind` may go now (`build_blocked`),
/// searched outward in half-cell rings.
pub(super) fn spot(g: &Game<GpuWorld>, kind: Kind, p: (f32, f32)) -> Option<(f32, f32)> {
    let sc = g.world.scale();
    (0..=12).flat_map(|r| (0..12).map(move |k| (r, k))).map(|(r, k)| {
        let a = k as f32 / 12.0 * std::f32::consts::TAU;
        (p.0 + 0.5 * r as f32 * sc * a.cos(), p.1 + 0.5 * r as f32 * sc * a.sin())
    }).find(|q| g.build_blocked(kind, *q).is_none())
}

/// The defences of a pump on crystal `c`, facing `to` (the slime's middle):
/// turrets in an arc just off the crystal, a wall arc past them, a lure
/// past the wall within the turrets' reach.
fn defences(g: &Game<GpuWorld>, p: &Plan, c: usize, to: (f32, f32)) -> Vec<Site> {
    let sc = g.world.scale();
    let at = g.world.crystals()[c].0;
    let r = g.world.crystal_r(c);
    let face = (to.1 - at.1).atan2(to.0 - at.0);
    let on = |rad: f32, a: f32| (at.0 + rad * a.cos(), at.1 + rad * a.sin());
    let mut out = Vec::new();
    if p.owns("turret") && p.turrets > 0 {
        let n = p.turrets as f32;
        // Spread over up to 160 degrees, 40 apart.
        let step = (160.0f32 / n.max(1.0)).min(40.0).to_radians();
        for i in 0..p.turrets {
            let a = face + (i as f32 - (n - 1.0) / 2.0) * step;
            out.push(Site { kind: Kind::Turret, at: on(r + 4.0 * sc, a), b: None, tries: 0 });
        }
    }
    if p.lure {
        out.push(Site { kind: Kind::Lure, at: on(r + 11.0 * sc, face), b: None, tries: 0 });
    }
    if p.owns("walls") && p.walls > 0 {
        let half = if p.walls == 1 { 60.0f32 } else { 120.0 }.to_radians();
        let segs = if p.walls == 1 { 2 } else { 4 };
        let rad = r + 8.0 * sc;
        for s in 0..segs {
            let a0 = face - half + 2.0 * half * s as f32 / segs as f32;
            let a1 = face - half + 2.0 * half * (s + 1) as f32 / segs as f32;
            out.push(Site { kind: Kind::Wall { to: on(rad, a1) }, at: on(rad, a0), b: None, tries: 0 });
        }
    }
    out
}

/// What the bank goes to next.
#[derive(Clone, Copy, Debug)]
enum Want {
    /// A pump on crystal `c`.
    Pump(usize),
    /// Site `s` of pump slot `i`.
    Site(usize, usize),
    /// Upgrade building `b`.
    Upgrade(usize),
}

pub(super) fn strat_hand(g: &mut Game<GpuWorld>, spec: &str) {
    let t = g.time();
    let fresh = MEM.with_borrow(|m| m.spec != spec) || t < 1.0;
    if fresh {
        let plan = match Plan::parse(spec) {
            Ok(p) => p,
            Err(e) => {
                eprintln!("{e}");
                return;
            }
        };
        MEM.with_borrow_mut(|m| *m = Memory { spec: spec.to_string(), plan, ..Memory::default() });
    }
    let p = MEM.with_borrow(|m| m.plan.clone());
    let sc = g.world.scale();
    // One squad: every other squad joins the first.
    let live = g.live_squads();
    let Some(&q) = live.first() else { return };
    for &o in &live[1..] {
        g.apply(Cmd::Merge(q as u8, o as u8));
    }
    if g.active != Some(q) {
        g.apply(Cmd::Active(q as u8));
    }
    let Some(m) = g.middle(q) else { return };
    let range = g.gun_range(q) * sc;
    let sense = Sense::of(g);
    let cs = g.world.crystals();
    let home = MEM.with_borrow(|mm| mm.home).unwrap_or(g.larder);
    let quiet = |g: &Game<GpuWorld>, c: usize| g.world.walkers_in(cs[c].0, g.world.crystal_r(c) + 0.7 * range) < 30;

    // ---- The bank: soldiers, pumps, defences, upgrades.
    // Forget buildings that fell, so their sites are wanted again.
    MEM.with_borrow_mut(|mm| {
        // A crystal whose pump the slime ate waits 90 s for another.
        for &(c, _) in mm.pumps.iter().filter(|(c, _)| g.pump_on(*c).is_none()) {
            mm.eaten.push((c, t + 90.0));
        }
        mm.eaten.retain(|&(_, until)| until > t);
        mm.pumps.retain(|(c, _)| g.pump_on(*c).is_some());
        for (_, sites) in &mut mm.pumps {
            for s in sites.iter_mut() {
                if s.b.is_some_and(|b| !g.buildings.get(b).is_some_and(|bd| bd.alive)) {
                    s.b = None;
                }
            }
        }
    });
    let out = g.alive() as u32 >= p.go;
    let want = |g: &Game<GpuWorld>| -> Option<Want> {
        let (pumps, home, mut refused) = MEM.with_borrow(|mm| (mm.pumps.clone(), mm.home, mm.refused.clone()));
        refused.extend(MEM.with_borrow(|mm| mm.eaten.iter().map(|e| e.0).collect::<Vec<_>>()));
        let base = home.unwrap_or(g.larder);
        let next_pump = || {
            (1..cs.len())
                .filter(|&c| cs[c].1 > 0.0 && g.pump_on(c).is_none() && g.pump_flows(c) && quiet(g, c) && !refused.contains(&c))
                .max_by(|&a, &b| {
                    let v = |c: usize| cs[c].1.min(400_000.0) / (1.0 + dist(cs[c].0, base) / (p.walk * range).max(1.0));
                    v(a).total_cmp(&v(b))
                })
        };
        let site_want = |i: usize, sites: &[Site]| sites.iter().position(|s| s.b.is_none() && s.tries < 3).map(|s| Want::Site(i, s));
        // Pumps (and each one's defences first when `guard_first`).
        for i in 0..p.pumps as usize {
            match pumps.get(i) {
                Some((_, sites)) => {
                    if p.guard_first {
                        if let Some(w) = site_want(i, sites) {
                            return Some(w);
                        }
                    }
                }
                None => match next_pump() {
                    Some(c) => return Some(Want::Pump(c)),
                    None => break,
                },
            }
        }
        for (i, (_, sites)) in pumps.iter().enumerate() {
            if let Some(w) = site_want(i, sites) {
                return Some(w);
            }
        }
        // Upgrades.
        for (c, _) in &pumps {
            if let Some(b) = g.pump_on(*c).filter(|&b| g.buildings[b].up() && u32::from(g.buildings[b].level) <= p.up) {
                if g.upgrade_price(b).is_some() {
                    return Some(Want::Upgrade(b));
                }
            }
        }
        None
    };
    let price = |g: &Game<GpuWorld>, w: Want| match w {
        Want::Pump(c) => g.build_price(Kind::Pump, cs[c].0),
        Want::Site(i, s) => MEM.with_borrow(|mm| {
            let site = mm.pumps[i].1[s];
            g.build_price(site.kind, site.at)
        }),
        Want::Upgrade(b) => g.upgrade_price(b).unwrap_or(f32::INFINITY),
    };
    let mut bought = 0;
    for _ in 0..8 {
        let alive = g.alive() as u32;
        if alive < p.hold.min(p.cap) {
            if g.bank >= g.rules.soldier_cost && g.apply(Cmd::Buy) {
                bought += 1;
                continue;
            }
            break;
        }
        match want(g) {
            Some(w) => {
                let cost = price(g, w);
                if g.bank >= cost {
                    MEM.with_borrow_mut(|mm| mm.saving = None);
                    if !place(g, &p, w, &sense) {
                        // Refused: count it as tried so another is wanted.
                        MEM.with_borrow_mut(|mm| match w {
                            Want::Site(i, s) => mm.pumps[i].1[s].tries += 1,
                            Want::Pump(c) => mm.refused.push(c),
                            Want::Upgrade(_) => {}
                        });
                        break;
                    }
                    continue;
                }
                // Saving: no soldiers for a minute, then soldiers again.
                let since = MEM.with_borrow_mut(|mm| *mm.saving.get_or_insert(t));
                if t - since < 60.0 || alive >= p.cap {
                    break;
                }
                if g.bank >= g.rules.soldier_cost + cost.min(g.rules.soldier_cost) && g.apply(Cmd::Buy) {
                    bought += 1;
                    continue;
                }
                break;
            }
            None => {
                if alive < p.cap && g.bank >= g.rules.soldier_cost && g.apply(Cmd::Buy) {
                    bought += 1;
                    continue;
                }
                break;
            }
        }
    }
    MEM.with_borrow_mut(|mm| {
        mm.bought += bought;
        mm.peak = mm.peak.max(g.alive());
    });

    // ---- The squad.
    let members = g.members(q);
    let n = members.len().max(1);
    let hp = members.iter().map(|&k| g.soldiers[k].hp).sum::<f32>() / n as f32 / g.rules.hp.max(1.0);
    let bitten = members.iter().filter(|&&k| g.soldiers[k].touching > 0).count();
    let eaten = hp < p.back && bitten > 0;
    // The volley: the heaviest slime in range with a clear line.
    let near = sense.ranked(m, range).into_iter().find(|&(a, d)| d <= range && sense.clear(g, m, a)).map(|(a, _)| a);
    g.apply(Cmd::Fire(q as u8, near));
    let fresh_order = |g: &Game<GpuWorld>, o: Order| !members.iter().all(|&k| g.soldiers[k].order == o);
    let log = (t - t.round()).abs() < 0.02 && (t.round() as u64).is_multiple_of(10);
    let fin = |g: &Game<GpuWorld>, doing: usize| {
        MEM.with_borrow_mut(|mm| mm.secs[doing] += 1);
        if log {
            eprintln!(
                "STRAT t {t:.0} {} mould {} soldiers {} hp {hp:.2} bank {:.0} killed {} turret {} pumps {} buildings {}",
                ["home", "out", "build", "back"][doing],
                g.mould,
                g.alive(),
                g.bank,
                g.killed,
                g.turret_killed,
                g.buildings.iter().filter(|b| b.alive && b.kind == Kind::Pump).count(),
                g.buildings.iter().filter(|b| b.alive).count(),
            );
        }
    };
    if eaten {
        let away = near.unwrap_or(home);
        let d = dist(away, m).max(1.0);
        let back = (m.0 - (away.0 - m.0) / d * range, m.1 - (away.1 - m.1) / d * range);
        if near.is_some() {
            g.apply(Cmd::SquadOrder(q as u8, Order::Move(back.0, back.1)));
        } else if !g.squads[q].heal && g.bank > 0.0 {
            g.apply(Cmd::Heal(q as u8));
        }
        return fin(g, 3);
    }
    // A ghost of ours to raise, with no thick slime on it: the squad
    // raises it (builders add up).
    let ghost = (0..g.buildings.len())
        .filter(|&b| g.buildings[b].alive && !g.buildings[b].up())
        .filter(|&b| sense.mass_near(g.buildings[b].at, range * 0.5) < 40 * n as u32)
        .min_by(|&a, &b| dist(g.buildings[a].at, m).total_cmp(&dist(g.buildings[b].at, m)));
    if let Some(b) = ghost {
        let since = MEM.with_borrow_mut(|mm| match mm.raising {
            Some((x, at)) if x == b => at,
            _ => {
                mm.raising = Some((b, t));
                t
            }
        });
        if t - since > 40.0 {
            g.apply(Cmd::Unbuild(b));
            MEM.with_borrow_mut(|mm| mm.raising = None);
            return fin(g, 2);
        }
        if fresh_order(g, Order::Build(b)) {
            g.apply(Cmd::SquadOrder(q as u8, Order::Build(b)));
        }
        return fin(g, 2);
    }
    // A guarded pump with slime on it: back to it (`defend`).
    let guarded: Vec<(f32, f32)> = MEM.with_borrow(|mm| mm.pumps.iter().take(p.guard as usize).map(|(c, _)| cs[*c].0).collect());
    if p.defend && out {
        let raided = guarded.iter().copied().filter(|&a| sense.mass_near(a, range) > 60).max_by(|&a, &b| sense.mass_near(a, range).cmp(&sense.mass_near(b, range)));
        if let Some(a) = raided.filter(|&a| dist(a, m) > 1.5 * range) {
            g.apply(Cmd::SquadOrder(q as u8, Order::Fight(a.0, a.1)));
            return fin(g, 0);
        }
    }
    // Home: guard the first pump (or the landing) until `go` soldiers;
    // with no pump to guard and no plan to build one, drain as the run bot.
    let has_home = MEM.with_borrow(|mm| mm.home.is_some()) || p.pumps > 0;
    if !out && has_home {
        let o = Order::Guard(home.0, home.1);
        if fresh_order(g, o) && dist(m, home) > 0.25 * range {
            g.apply(Cmd::SquadOrder(q as u8, o));
        }
        return fin(g, 0);
    }
    // Out: a crystal to pump (`more`), else to drain (`take` 0), else hunt.
    let pumps_now = MEM.with_borrow(|mm| mm.pumps.len()) as u32;
    if out && p.more > 0 && pumps_now < p.pumps + p.more {
        let target = (1..cs.len())
            .filter(|&c| cs[c].1 > 0.0 && g.pump_on(c).is_none() && g.pump_flows(c))
            .filter(|&c| sense.mass_near(cs[c].0, g.world.crystal_r(c) + range) < 120 * n as u32)
            .min_by(|&a, &b| {
                let v = |c: usize| dist(cs[c].0, m) / (1.0 + cs[c].1.min(400_000.0) / 100_000.0);
                v(a).total_cmp(&v(b))
            });
        if let Some(c) = target {
            if g.bank >= g.build_price(Kind::Pump, cs[c].0) && dist(cs[c].0, m) < 2.0 * range && place(g, &p, Want::Pump(c), &sense) {
                return fin(g, 2);
            }
            let o = Order::Clear(cs[c].0 .0, cs[c].0 .1);
            if fresh_order(g, o) {
                g.apply(Cmd::SquadOrder(q as u8, o));
            }
            return fin(g, 1);
        }
    }
    hunt(g, &p, q, m, range, &sense, &members, out);
    fin(g, 1)
}

/// Spend on want `w`: place the pump or site, or buy the upgrade. True if
/// the game took it.
fn place(g: &mut Game<GpuWorld>, p: &Plan, w: Want, sense: &Sense) -> bool {
    let cs = g.world.crystals();
    match w {
        Want::Pump(c) => {
            // On its rim, toward the squad: the pump snaps to the middle.
            let (cx, cy) = cs[c].0;
            let from = g.live_squads().first().and_then(|&q| g.middle(q)).unwrap_or(g.larder);
            let d = dist(from, (cx, cy)).max(1.0);
            let r = g.world.crystal_r(c) * 0.9;
            if !g.apply(Cmd::Build(Kind::Pump, (cx + (from.0 - cx) / d * r, cy + (from.1 - cy) / d * r))) {
                return false;
            }
            let to = sense.middle().unwrap_or(cs[0].0);
            let sites = if MEM.with_borrow(|mm| mm.pumps.len()) < p.guard as usize { defences(g, p, c, to) } else { Vec::new() };
            MEM.with_borrow_mut(|mm| {
                mm.home.get_or_insert((cx, cy));
                mm.pumps.push((c, sites));
            });
            true
        }
        Want::Site(i, s) => {
            let site = MEM.with_borrow(|mm| mm.pumps[i].1[s]);
            let ok = match site.kind {
                Kind::Wall { to } => g.build_blocked(site.kind, site.at).is_none() && g.apply(Cmd::Build(Kind::Wall { to }, site.at)),
                k => spot(g, k, site.at).is_some_and(|at| g.apply(Cmd::Build(k, at))),
            };
            if ok {
                let b = g.buildings.len() - 1;
                MEM.with_borrow_mut(|mm| {
                    let st = &mut mm.pumps[i].1[s];
                    st.b = Some(b);
                    st.tries += 1;
                });
            }
            ok
        }
        Want::Upgrade(b) => g.apply(Cmd::Upgrade(b)),
    }
}

/// Out of home with nothing to build: the run bot's play (work the
/// crystal with the most charge, leave one that pays nothing, take the
/// home when the slime by it is thin, else hunt the heaviest slime).
#[allow(clippy::too_many_arguments)]
fn hunt(g: &mut Game<GpuWorld>, p: &Plan, q: usize, m: (f32, f32), range: f32, sense: &Sense, members: &[usize], out: bool) {
    let t = g.time();
    let cs = g.world.crystals();
    let fresh = |g: &Game<GpuWorld>, o: Order| !members.iter().all(|&k| g.soldiers[k].order == o);
    let left = MEM.with_borrow_mut(|b| {
        b.left.retain(|&(_, until)| until > t);
        b.left.iter().map(|l| l.0).collect::<Vec<_>>()
    });
    let home_mass = sense.mass_near(cs[0].0, g.world.crystal_r(0) + range);
    let home_ok = (home_mass as f32) < p.per * g.alive() as f32;
    let barren = MEM.with_borrow(|b| b.barren.clone());
    let strong = out || (1..cs.len()).all(|c| cs[c].1 <= 0.0 || barren.contains(&c));
    let drains = p.take == 0;
    let quiet = |c: usize| g.world.walkers_in(cs[c].0, g.world.crystal_r(c) + range) < 30;
    let pick = drains
        .then(|| {
            (0..cs.len())
                .filter(|&c| cs[c].1 > 0.0 && !left.contains(&c) && (c > 0 || home_ok) && (strong || (c > 0 && quiet(c))) && g.pump_on(c).is_none())
                .max_by(|&a, &b| {
                    let walk = if strong { 4.0 * range } else { 0.5 * range };
                    let v = |c: usize| cs[c].1.min(200_000.0) / (1.0 + dist(cs[c].0, m) / walk.max(1.0));
                    v(a).total_cmp(&v(b))
                })
        })
        .flatten();
    let work = MEM.with_borrow_mut(|b| {
        match (b.work, pick) {
            (Some((c, h, since)), _) if drains && cs.get(c).is_some_and(|x| x.1 > 0.0) && !b.left.iter().any(|l| l.0 == c) && (c > 0 || home_ok) && g.pump_on(c).is_none() => {
                if g.harvested > h + 50.0 {
                    b.work = Some((c, g.harvested, t));
                } else if t - since > 30.0 && c > 0 {
                    b.left.push((c, t + 60.0));
                    if !b.barren.contains(&c) {
                        b.barren.push(c);
                    }
                    b.work = pick.filter(|&x| x != c).map(|x| (x, g.harvested, t));
                }
            }
            (_, Some(x)) => b.work = Some((x, g.harvested, t)),
            _ => b.work = None,
        }
        b.work.map(|w| w.0)
    });
    if let Some(c) = work.filter(|&c| c > 0 || home_ok) {
        let at = cs[c].0;
        let o = Order::Clear(at.0, at.1);
        if fresh(g, o) {
            g.apply(Cmd::SquadOrder(q as u8, o));
        }
        return;
    }
    // Hunt: from a stand with a clear line to one of the heaviest squares;
    // the home's slime only once it is thin enough to walk into.
    let stand_for = |a: (f32, f32)| {
        let d = dist(a, m).max(1.0);
        (a.0 - (a.0 - m.0) / d * 0.7 * range, a.1 - (a.1 - m.1) / d * 0.7 * range)
    };
    let by_home = |a: (f32, f32)| dist(a, cs[0].0) <= g.world.crystal_r(0) + range;
    let ranked = sense.ranked(m, range);
    let shot = ranked.iter().map(|&(a, _)| (a, stand_for(a))).filter(|&(a, _)| home_ok || !by_home(a)).find(|&(a, st)| g.ground(st.0, st.1) && sense.clear(g, st, a));
    let shot = shot.or_else(|| ranked.iter().map(|&(a, _)| (a, stand_for(a))).find(|&(a, _)| home_ok || !by_home(a)));
    let Some((a, stand)) = shot.or_else(|| ranked.first().map(|&(a, _)| (a, stand_for(a)))) else { return };
    let d = dist(a, m).max(1.0);
    if d > 0.8 * range || !sense.clear(g, m, a) {
        g.apply(Cmd::SquadOrder(q as u8, Order::Fight(stand.0, stand.1)));
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn every_preset_parses_and_dials_win_over_it() {
        for (name, _) in PRESETS {
            Plan::parse(name).unwrap_or_else(|e| panic!("{name}: {e}"));
        }
        let p = Plan::parse("turtle,go=12,own=walls/turret").unwrap();
        assert_eq!(p.go, 12);
        assert_eq!(p.turrets, 3);
        assert!(p.owns("turret") && !p.owns("mortar"));
        assert!(Plan::parse("nonsense").is_err());
        assert!(Plan::parse("pumps=x").is_err());
        assert_eq!(Plan::parse("").unwrap(), Plan::default());
    }
}
