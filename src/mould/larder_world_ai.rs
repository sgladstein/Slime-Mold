//! **The player AI on the Larder world** (ROADMAP L-AI; owner, 2026-11-11
//! 14:07Z: "a player AI ... for automated testing so you can keep
//! developing without me having to playtest everything", and "two soldier
//! teams against multiple molds so in the future the AI could also play
//! soldiers"). The small-dish AI (`larder_ai`, PR #65) ported onto the
//! world's game layer (`larder_world::Game`, W2b).
//!
//! **Plays through the player's hands.** It gives only `Cmd`s, through
//! `Game::apply`, so every AI round is a replay (`Game::replay_text`), and
//! it plays on any `World`.
//!
//! **Sees what a player sees** (`View`): its own soldiers, the bank, the
//! crystals and their charge, the clock, and the mould as walker counts on
//! a 4-dish-cell grid (`World::mould_grid`: the tube on screen, coarsened).
//! It does not read scent, headings or the core's clear timer. The GPU
//! world has no grid yet (it needs an occupancy read-back), so there the
//! brain reads mould only from its own soldiers' touch: it plays, but
//! half-blind.
//!
//! **Commands a `Side`**: a home and the soldiers it owns. The world game
//! has one squad, so a side is all of it today; a second team is a second
//! side with its own brain (test `a_side_commands_only_its_own`).
//!
//! **One brain, turned by knobs** (`Style`). The world game is scored at the
//! cap by the share of the contested crystals held (`Game::share`): a
//! crystal is ours with a soldier on it and no mould touching that soldier,
//! the mould's when it has eaten from it and no soldier is on it. So the
//! knobs are about which crystals to farm, which mould-held ones to take
//! back and when, walls, and a strike on the core. The presets
//! (`Style::presets`) are plans a player would name: boom, claim, late,
//! fort and strike.
//!
//! What changed from the small-dish AI, and why: the world has no turrets,
//! bait, hunger or drops (W2b did not port them), so their knobs went; it
//! has walls, crystals that run dry and a share at the cap, so `walls`,
//! `claim_*` and `keep` came in. The M2 reader (adapt) is gone: its signal
//! was the mould turning small, and on the race world the mould only grows
//! (no W2b hand ever got it below its start).

use super::larder_world::{Cmd, Game, Order, Rules, Stance, World};

/// Dish cells a side of one mould-grid square.
pub const GRID: f32 = 4.0;

fn dist(a: (f32, f32), b: (f32, f32)) -> f32 {
    (a.0 - b.0).hypot(a.1 - b.1)
}

/// Where soldier `k` stands round the point it is sent to, dish cells: the
/// game's sunflower (`larder_world::slot`), which spreads a squad sent to
/// one point. A holder is sent to the crystal minus its slot so it stands
/// on the crystal itself; soldier 30's slot is 8 dish cells out, off the
/// crystal, and a soldier off it doesn't count toward the share.
fn slot(k: usize) -> (f32, f32) {
    if k == 0 {
        return (0.0, 0.0);
    }
    let r = 1.5 * (k as f32).sqrt();
    let a = k as f32 * 2.399_963;
    (r * a.cos(), r * a.sin())
}

/// The soldiers a brain commands and the home it holds.
#[derive(Clone, Debug)]
pub struct Side {
    /// Fine cells.
    pub home: (f32, f32),
    /// Soldier indices this side owns; with `whole`, every soldier.
    pub ids: Vec<usize>,
    pub whole: bool,
}

impl Side {
    pub fn whole<W: World>(g: &Game<W>) -> Side {
        Side { home: g.larder, ids: Vec::new(), whole: true }
    }

    pub fn owns(&self, k: usize) -> bool {
        self.whole || self.ids.contains(&k)
    }
}

/// One of the side's live soldiers as the player sees it.
#[derive(Clone, Copy, Debug)]
pub struct UnitView {
    pub k: usize,
    /// Fine cells.
    pub at: (f32, f32),
    pub hp: f32,
    pub order: Order,
    pub stance: Stance,
    pub touching: u32,
}

/// One crystal with charge left.
#[derive(Clone, Copy, Debug)]
pub struct CrystalView {
    pub i: usize,
    /// Fine cells.
    pub at: (f32, f32),
    pub charge: f32,
    /// Its radius, fine cells: a map's crystals differ (WM1).
    pub r: f32,
    /// Walkers on it (within its radius and 2 dish cells) and near it
    /// (within `NEAR` dish cells); 0 when the world gives no grid.
    pub on: u32,
    pub near: u32,
}

/// Dish cells round a crystal that count as near it.
pub const NEAR: f32 = 14.0;

/// What a player sees, once a think.
#[derive(Clone, Debug)]
pub struct View {
    pub t: f32,
    pub rules: Rules,
    /// Fine cells a dish cell.
    pub scale: f32,
    pub bank: f32,
    pub home: (f32, f32),
    pub core: (f32, f32),
    pub core_down: bool,
    /// Food banked from harvest over the game (`Game::harvested`), for the
    /// brain's income (`Brain::income`).
    pub harvested: f32,
    pub units: Vec<UnitView>,
    pub crystals: Vec<CrystalView>,
    /// Walkers alive, as the screen's count shows.
    pub mould: usize,
    /// The mould grid: squares of `bin` fine cells, `cols` across.
    pub grid: Option<Vec<u32>>,
    pub bin: usize,
    pub cols: usize,
}

impl View {
    pub fn of<W: World>(g: &Game<W>, side: &Side) -> View {
        let scale = g.world.scale();
        let bin = (GRID * scale).round().max(1.0) as usize;
        let cols = g.world.size().0.div_ceil(bin);
        let grid = g.world.mould_grid(bin);
        let units = (0..g.soldiers.len())
            .filter(|&k| g.soldiers[k].alive && side.owns(k))
            .map(|k| {
                let s = g.soldiers[k];
                UnitView { k, at: (s.x, s.y), hp: s.hp, order: s.order, stance: s.stance, touching: s.touching }
            })
            .collect();
        let mut v = View {
            t: g.time(),
            rules: g.rules,
            scale,
            bank: g.bank,
            home: side.home,
            core: g.core,
            core_down: g.core_down.is_some(),
            harvested: g.harvested,
            units,
            crystals: Vec::new(),
            mould: g.mould,
            grid,
            bin,
            cols,
        };
        // A crystal under one frame's full ask is dry: the game sends its
        // carrier home to hold (`Rules::home_dry`), so the brain neither
        // farms nor claims it, and gives the holder a new crystal. (Under
        // `> 0.0` a soldier went back for the 0.8 charge of crumbs a crystal
        // keeps.)
        let r = g.rules;
        let dry = r.harvest * r.frame_ticks as f32 / r.tps as f32 * r.charge;
        v.crystals = g
            .world
            .crystals()
            .into_iter()
            .enumerate()
            .filter(|c| c.1 .1 >= dry)
            .map(|(i, (at, charge))| {
                let r = g.world.crystal_r(i);
                CrystalView { i, at, charge, r, on: v.mass(at, r + 2.0 * scale), near: v.mass(at, NEAR * scale) }
            })
            .collect();
        v
    }

    /// Walkers within `r` fine cells of `c`, by grid square centres.
    pub fn mass(&self, c: (f32, f32), r: f32) -> u32 {
        let Some(grid) = &self.grid else { return 0 };
        let b = self.bin as f32;
        let rows = grid.len() / self.cols;
        let (x0, x1) = (((c.0 - r) / b).floor().max(0.0) as usize, (((c.0 + r) / b) as usize).min(self.cols - 1));
        let (y0, y1) = (((c.1 - r) / b).floor().max(0.0) as usize, (((c.1 + r) / b) as usize).min(rows - 1));
        let mut n = 0;
        for y in y0..=y1 {
            for x in x0..=x1 {
                if dist(((x as f32 + 0.5) * b, (y as f32 + 0.5) * b), c) <= r {
                    n += grid[y * self.cols + x];
                }
            }
        }
        n
    }

    /// The walkers' centre within `r` fine cells of `c`, and how many.
    pub fn centroid(&self, c: (f32, f32), r: f32) -> Option<((f32, f32), u32)> {
        let grid = self.grid.as_ref()?;
        let b = self.bin as f32;
        let rows = grid.len() / self.cols;
        let (x0, x1) = (((c.0 - r) / b).floor().max(0.0) as usize, (((c.0 + r) / b) as usize).min(self.cols - 1));
        let (y0, y1) = (((c.1 - r) / b).floor().max(0.0) as usize, (((c.1 + r) / b) as usize).min(rows - 1));
        let (mut sx, mut sy, mut n) = (0.0, 0.0, 0u32);
        for y in y0..=y1 {
            for x in x0..=x1 {
                let m = grid[y * self.cols + x];
                let p = ((x as f32 + 0.5) * b, (y as f32 + 0.5) * b);
                if m > 0 && dist(p, c) <= r {
                    sx += p.0 * m as f32;
                    sy += p.1 * m as f32;
                    n += m;
                }
            }
        }
        (n > 0).then(|| ((sx / n as f32, sy / n as f32), n))
    }

    /// Food the bank must keep to arm `n` soldiers for `secs` game seconds.
    pub fn ammo(&self, n: usize, secs: f32) -> f32 {
        n as f32 * self.rules.price / self.rules.shot_every * secs
    }
}

/// **The knobs one brain is turned by.** Distances in dish cells, times in
/// game seconds, food in food. `usize::MAX` and `f32::INFINITY` mean never
/// or no limit.
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct Style {
    pub name: &'static str,
    /// Crystals farmed at once, one soldier each, nearest home first.
    pub farm: usize,
    /// Walkers near a crystal (within `NEAR`) past which no farmer is sent
    /// to it.
    pub dare: u32,
    /// Buy up to `buy_to` soldiers, keeping `reserve` food and `ammo` game
    /// seconds of fire for the whole squad (the small-dish AI's lesson: a
    /// soldier that can't fire dies and feeds the mould).
    pub buy_to: usize,
    pub reserve: f32,
    pub ammo: f32,
    /// From `claim_at`, take back mould-held crystals: `claim_group`
    /// soldiers stand on each (Move onto it, `claim_free` or Defend), up to
    /// `claims` crystals at once, nearest home first, while `keep` go on
    /// farming.
    pub claim_at: f32,
    pub claim_group: usize,
    pub claims: usize,
    pub claim_free: bool,
    pub keep: usize,
    /// Walls laid at most, each across the mould's road to a farmed
    /// crystal when `wall_alarm` walkers come within `NEAR` of it.
    pub walls: usize,
    pub wall_alarm: u32,
    /// From `strike_at`, once the side is `strike_army` strong, all
    /// soldiers attack the core; once it is down they claim.
    pub strike_at: f32,
    pub strike_army: usize,
    /// A soldier below this share of full health with mould touching walks
    /// home and stays (health never comes back). 0 never pulls back.
    pub retreat: f32,
}

impl Style {
    /// **Boom**: farm the twenty nearest crystals with a soldier bought for
    /// each, nothing else (the W2b farm hand, wider, buying with the ammo
    /// reserve). On seeds 1-4, twelve scored a mean of +0.14 and twenty
    /// +0.25; the farm hand's eight, -0.05.
    pub const BOOM: Style = Style {
        name: "boom",
        farm: 20,
        dare: 40,
        buy_to: 20,
        reserve: 400.0,
        ammo: 30.0,
        claim_at: f32::INFINITY,
        claim_group: 2,
        claims: 4,
        claim_free: false,
        keep: 12,
        walls: 0,
        wall_alarm: 10,
        strike_at: f32::INFINITY,
        strike_army: 20,
        retreat: 0.0,
    };

    /// **Claim**: boom to 28 soldiers, and from 10:00 send pairs to stand
    /// on the nearest mould-held crystals while twelve keep farming. On
    /// seeds 1-4 a claim from 5:00 (mean +0.03) and fours on Free fire
    /// (+0.03) did worse than one from 10:00 (+0.08): holders stood in the
    /// network and were eaten.
    pub const CLAIM: Style = Style { name: "claim", buy_to: 28, claim_at: 600.0, ..Style::BOOM };

    /// **Late**: as claim, but the claim starts 90 s before the cap. The
    /// share is counted only at the cap, so this asks whether a last-minute
    /// grab beats holding (an exploiter's question about the scoring).
    pub const LATE: Style = Style { name: "late", claim_at: -90.0, ..Style::CLAIM };

    /// **Fort**: boom, plus up to eight walls, each across the mould's road
    /// to a farmed crystal when it comes near (W2b's one wall on the
    /// core-to-larder road added nothing over farming; this one walls where
    /// the mould is, not where it starts).
    pub const FORT: Style = Style { name: "fort", walls: 8, ..Style::BOOM };

    /// **Strike**: boom to 30 soldiers, and from 6:00, once 20 stand, all
    /// but eight attack the core (W2b's rush of 4 killed about 900 walkers
    /// and died within 36 s); after the core falls, claim.
    pub const STRIKE: Style = Style { name: "strike", buy_to: 30, strike_at: 360.0, keep: 8, ..Style::BOOM };

    pub fn presets() -> [Style; 5] {
        [Style::BOOM, Style::CLAIM, Style::LATE, Style::FORT, Style::STRIKE]
    }

    /// A preset by name, with knobs after a colon: `claim:claim_at=200+keep=4`
    /// (`+`, as the harness splits hands on commas).
    pub fn named(name: &str) -> Option<Style> {
        let (base, knobs) = name.split_once(':').unwrap_or((name, ""));
        let mut s = Style::presets().into_iter().find(|s| s.name == base)?;
        for kv in knobs.split('+').filter(|k| !k.is_empty()) {
            let (k, v) = kv.split_once('=')?;
            s.set(k, v.parse().ok()?).then_some(())?;
        }
        Some(s)
    }

    /// Set one knob by name; booleans are 0 or 1, "inf" is never.
    pub fn set(&mut self, k: &str, v: f32) -> bool {
        let n = if v.is_infinite() { usize::MAX } else { v as usize };
        let u = if v.is_infinite() { u32::MAX } else { v as u32 };
        match k {
            "farm" => self.farm = n,
            "dare" => self.dare = u,
            "buy_to" => self.buy_to = n,
            "reserve" => self.reserve = v,
            "ammo" => self.ammo = v,
            "claim_at" => self.claim_at = v,
            "claim_group" => self.claim_group = n,
            "claims" => self.claims = n,
            "claim_free" => self.claim_free = v != 0.0,
            "keep" => self.keep = n,
            "walls" => self.walls = n,
            "wall_alarm" => self.wall_alarm = u,
            "strike_at" => self.strike_at = v,
            "strike_army" => self.strike_army = n,
            "retreat" => self.retreat = v,
            _ => return false,
        }
        true
    }
}

/// Walkers on a crystal that make it the mould's to a brain.
const HELD: u32 = 4;
/// Wall length and how far out from the crystal it stands, dish cells.
const WALL_LEN: f32 = 16.0;
const WALL_OUT: f32 = 8.0;
/// Game seconds before the same crystal gets another wall.
const WALL_EVERY: f32 = 60.0;

/// Game seconds the brain's income average looks back over.
const INCOME_TAU: f32 = 60.0;
/// The share of its income a brain spends on rations: the rest is margin
/// for a crystal running dry or a carrier eaten on the way home.
const FEED_SHARE: f32 = 0.8;
/// Game seconds of rations the spare bank (above the price, the reserve
/// and the ammo) may pay for an army its income can't yet feed: the
/// opening buys, before the first carrier banks.
const FEED_HORIZON: f32 = 120.0;
/// A brain whose income can't pay the rations and whose bank holds less
/// than this many seconds of them sends every soldier to farm.
const LEAN_SECS: f32 = 60.0;
/// The share of the crystals' food left that a brain counts on banking
/// before the cap; the mould eats the rest.
const LEFT_SHARE: f32 = 0.5;
/// Game seconds ahead (or to the cap, if sooner) that the bank and that
/// food must feed the army. Until the cap, the campaign's Landfall (8
/// soldiers, 4,000 banked, 45 a second coming in, little food left on
/// its small crystals) stopped buying short of the 12 its goal asks.
const LEFT_SECS: f32 = 300.0;

/// **The AI player.** Call `play` once a game second, before the step.
pub struct Brain {
    pub style: Style,
    pub side: Side,
    /// Soldiers pulled back hurt.
    hurt: Vec<usize>,
    /// Game second each crystal was last walled.
    walled: Vec<(usize, f32)>,
    walls: usize,
    striking: bool,
    /// The cap, for a claim timed from it (negative `claim_at`).
    cap: f32,
    /// Food banked from harvest a game second, averaged over `INCOME_TAU`,
    /// and the last (time, harvested) it was read at.
    income: f32,
    seen: Option<(f32, f32)>,
}

impl Brain {
    pub fn new(style: Style, side: Side) -> Brain {
        Brain { style, side, hurt: Vec::new(), walled: Vec::new(), walls: 0, striking: false, cap: 0.0, income: 0.0, seen: None }
    }

    /// Look, decide, and give the commands on `g`. Returns how many it gave.
    pub fn play<W: World>(&mut self, g: &mut Game<W>) -> usize {
        self.cap = g.rules.cap;
        let before = g.soldiers.len();
        let cmds = self.think(&View::of(g, &self.side));
        for &c in &cmds {
            let ok = g.apply(c);
            if ok && matches!(c, Cmd::Wall(..)) {
                self.walls += 1;
            }
        }
        if !self.side.whole {
            // Soldiers bought this think are ours.
            self.side.ids.extend(before..g.soldiers.len());
        }
        cmds.len()
    }

    /// Its turn from a view: the commands to give, in order.
    pub fn think(&mut self, v: &View) -> Vec<Cmd> {
        let st = self.style;
        let sc = v.scale;
        let mut out = Vec::new();
        let n = v.units.len();
        if let Some((t0, h0)) = self.seen {
            let dt = v.t - t0;
            if dt > 0.0 {
                let a = (dt / INCOME_TAU).min(1.0);
                self.income += a * ((v.harvested - h0) / dt - self.income);
            }
        }
        self.seen = Some((v.t, v.harvested));
        // Buy while the bank keeps the reserve and fire for the bigger
        // squad, and the income can feed it (`Rules::ration`): a soldier
        // eats `ration` a second whatever it does. Before, the strike
        // bought 30 whatever the bank could feed, and under rations (#170)
        // its army starved on 3 of 16 headless race games (5-20; the
        // fight-feel thread, 11-12 22:58Z).
        // For its first `INCOME_TAU` the brain has no income to go by, so
        // it buys on the bank alone, as before: gated from the start, the
        // claim was too slow to win the campaign's Landfall in 200 s
        // (`campaign::tests`). The income is now's; the crystals run dry and the mould takes
        // them, so the army is also held to what the bank and a share of
        // the food on crystals the mould is not near can feed for the next
        // `LEFT_SECS`.
        let left: f32 = v.crystals.iter().filter(|c| c.near < st.dare).map(|c| c.charge).sum::<f32>() / v.rules.charge;
        let to_cap = (self.cap - v.t).clamp(1.0, LEFT_SECS);
        let feeds = |size: usize, bank: f32| {
            let spare = (bank - v.rules.soldier_cost - st.reserve - v.ammo(size + 1, st.ammo)).max(0.0);
            let need = (size + 1) as f32 * v.rules.ration;
            v.rules.ration <= 0.0 || v.t < INCOME_TAU || (need <= FEED_SHARE * self.income + spare / FEED_HORIZON && need <= (spare + LEFT_SHARE * left) / to_cap)
        };
        let mut bank = v.bank;
        let mut size = n;
        while size < st.buy_to.min(v.rules.max_squad) && bank - v.rules.soldier_cost >= st.reserve + v.ammo(size + 1, st.ammo) && feeds(size, bank) {
            out.push(Cmd::Buy);
            bank -= v.rules.soldier_cost;
            size += 1;
        }
        // Pull back the hurt.
        for u in &v.units {
            if st.retreat > 0.0 && u.touching > 0 && u.hp < st.retreat * v.rules.hp && !self.hurt.contains(&u.k) {
                self.hurt.push(u.k);
                out.push(Cmd::Order(u.k, Order::Move(v.home.0, v.home.1)));
            }
        }
        let free: Vec<UnitView> = v.units.iter().copied().filter(|u| !self.hurt.contains(&u.k)).collect();
        let claim_at = if st.claim_at < 0.0 { self.cap + st.claim_at } else { st.claim_at };
        if !self.striking && v.t >= st.strike_at && free.len() >= st.strike_army {
            self.striking = true;
        }
        // Lean: the rations outrun the income and the bank is nearly out,
        // so the strike and the claim wait and every soldier farms.
        let upkeep = v.rules.ration * n as f32;
        let lean = upkeep > self.income && v.bank < upkeep * LEAN_SECS;
        let striking = self.striking && !lean;
        // The strike: all but `keep` attack the core, those already
        // attacking first, then the nearest to it. (The first version sent
        // everyone: with no farmers left the bank ran dry mid-strike, and
        // on seeds 1-4 three of four squads ended with one soldier or none.)
        let mut striker = vec![false; free.len()];
        if striking && !v.core_down {
            let mut by: Vec<usize> = (0..free.len()).collect();
            by.sort_by(|&a, &b| (free[b].order == Order::Attack).cmp(&(free[a].order == Order::Attack)).then(dist(free[a].at, v.core).total_cmp(&dist(free[b].at, v.core))));
            for &j in by.iter().take(free.len().saturating_sub(st.keep)) {
                striker[j] = true;
                if free[j].order != Order::Attack {
                    out.push(Cmd::Order(free[j].k, Order::Attack));
                }
            }
        }
        let claiming = !lean && (v.t >= claim_at || (striking && v.core_down));
        // Farm the nearest crystals the mould is not near, the core last.
        let mut farmable: Vec<&CrystalView> = v.crystals.iter().filter(|c| dist(c.at, v.core) > 1.0 && c.near < st.dare).collect();
        farmable.sort_by(|a, b| dist(a.at, v.home).total_cmp(&dist(b.at, v.home)));
        farmable.truncate(st.farm);
        // Crystals the mould holds, nearest home first, for the claim.
        let mut held: Vec<&CrystalView> = v.crystals.iter().filter(|c| c.on >= HELD).collect();
        held.sort_by(|a, b| dist(a.at, v.home).total_cmp(&dist(b.at, v.home)));
        let farmers = if claiming || striking { st.keep.min(free.len()) } else { free.len() };
        // Soldiers keep a job they hold if it is still wanted, so the
        // squad doesn't walk back and forth between thinks.
        let mut used = striker.clone();
        let mut farm_taken: Vec<usize> = Vec::new();
        let mut nfarm = 0;
        for (j, u) in free.iter().enumerate() {
            if let Order::Clear(x, y) = u.order {
                if nfarm < farmers {
                    if let Some(c) = farmable.iter().find(|c| dist(c.at, (x, y)) < 1.0) {
                        if !farm_taken.contains(&c.i) {
                            farm_taken.push(c.i);
                            used[j] = true;
                            nfarm += 1;
                        }
                    }
                }
            }
        }
        for c in &farmable {
            if nfarm >= farmers {
                break;
            }
            if farm_taken.contains(&c.i) {
                continue;
            }
            // The nearest unused soldier.
            let Some(j) = (0..free.len()).filter(|&j| !used[j]).min_by(|&a, &b| dist(free[a].at, c.at).total_cmp(&dist(free[b].at, c.at))) else { break };
            used[j] = true;
            farm_taken.push(c.i);
            nfarm += 1;
            out.push(Cmd::Order(free[j].k, Order::Clear(c.at.0, c.at.1)));
            if free[j].stance != Stance::Defend {
                out.push(Cmd::Stance(free[j].k, Stance::Defend));
            }
        }
        // Tried and reverted: a mop-up. After the core fell, every soldier
        // not farming walked to the mould nearest it on Free fire. On
        // seeds 21-36 the strike scored lower with it on 6 seeds, higher on
        // 1 and the same on 9, and wiped the mould on one seed either way
        // (23 without, 36 with): the hunters left their crystals, and tens
        // to hundreds of walkers lived to the cap anyway.
        // Claim: groups onto held crystals.
        if claiming {
            let want = if st.claim_free { Stance::Free } else { Stance::Defend };
            let spot = |c: (f32, f32), k: usize| {
                let o = slot(k);
                (c.0 - o.0 * sc, c.1 - o.1 * sc)
            };
            // Holders already on a held (or just-freed) crystal stay.
            let mut groups: Vec<((f32, f32), usize)> = Vec::new();
            for (j, u) in free.iter().enumerate() {
                if used[j] {
                    continue;
                }
                if let Order::Move(x, y) = u.order {
                    let target = v.crystals.iter().find(|c| dist(spot(c.at, u.k), (x, y)) < 1.0);
                    if let Some(c) = target {
                        let g = groups.iter().position(|g| dist(g.0, c.at) < 1.0);
                        match g {
                            Some(g) if groups[g].1 < st.claim_group => {
                                groups[g].1 += 1;
                                used[j] = true;
                            }
                            None if groups.len() < st.claims => {
                                groups.push((c.at, 1));
                                used[j] = true;
                            }
                            _ => {}
                        }
                    }
                }
            }
            for c in &held {
                let at = match groups.iter().position(|g| dist(g.0, c.at) < 1.0) {
                    Some(g) => g,
                    None if groups.len() < st.claims => {
                        groups.push((c.at, 0));
                        groups.len() - 1
                    }
                    None => continue,
                };
                while groups[at].1 < st.claim_group {
                    let Some(j) = (0..free.len()).filter(|&j| !used[j]).min_by(|&a, &b| dist(free[a].at, c.at).total_cmp(&dist(free[b].at, c.at))) else { break };
                    used[j] = true;
                    groups[at].1 += 1;
                    let p = spot(c.at, free[j].k);
                    out.push(Cmd::Order(free[j].k, Order::Move(p.0, p.1)));
                    if free[j].stance != want {
                        out.push(Cmd::Stance(free[j].k, want));
                    }
                }
            }
        }
        // The rest stand at home on Defend.
        for (j, u) in free.iter().enumerate() {
            if !used[j] && u.order != Order::Hold && u.order != Order::Move(v.home.0, v.home.1) {
                out.push(Cmd::Order(u.k, Order::Move(v.home.0, v.home.1)));
            }
        }
        // Walls across the mould's road to a farmed crystal it comes near.
        if self.walls < st.walls {
            // Crystals a soldier was clearing at this think's start: one
            // the mould has come near has just left `farmable` (`dare`),
            // and that is when it needs the wall.
            let clearing: Vec<(f32, f32)> = v.units.iter().filter_map(|u| if let Order::Clear(x, y) = u.order { Some((x, y)) } else { None }).collect();
            for c in v.crystals.iter().filter(|c| clearing.iter().any(|&p| dist(p, c.at) < 1.0) && c.near >= st.wall_alarm && c.on < HELD) {
                if self.walled.iter().any(|w| w.0 == c.i && v.t - w.1 < WALL_EVERY) {
                    continue;
                }
                let Some((m, _)) = v.centroid(c.at, NEAR * sc) else { continue };
                let (dx, dy) = (m.0 - c.at.0, m.1 - c.at.1);
                let d = dx.hypot(dy).max(1.0);
                let (ux, uy) = (dx / d, dy / d);
                let mid = (c.at.0 + ux * WALL_OUT * sc, c.at.1 + uy * WALL_OUT * sc);
                let h = WALL_LEN * sc / 2.0;
                let (a, b) = ((mid.0 - uy * h, mid.1 + ux * h), (mid.0 + uy * h, mid.1 - ux * h));
                let price = (WALL_LEN).max(1.0) * v.rules.wall_cost;
                if bank - price < st.reserve + v.ammo(size, st.ammo) {
                    break;
                }
                self.walled.retain(|w| w.0 != c.i);
                self.walled.push((c.i, v.t));
                out.push(Cmd::Wall(a, b));
                break;
            }
        }
        out
    }
}

/// Who plays a round: nobody, or a brain.
pub enum Hand {
    Idle,
    Brain(Box<Brain>),
}

impl Hand {
    /// "idle", or a style by name with its knobs (`Style::named`).
    pub fn named<W: World>(name: &str, g: &Game<W>) -> Option<Hand> {
        if name == "idle" {
            return Some(Hand::Idle);
        }
        Some(Hand::Brain(Box::new(Brain::new(Style::named(name)?, Side::whole(g)))))
    }

    /// Its turn: call once a game second, before the step.
    pub fn play<W: World>(&mut self, g: &mut Game<W>) -> usize {
        match self {
            Hand::Idle => 0,
            Hand::Brain(b) => b.play(g),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::super::larder_world::race_world;
    use super::*;
    use crate::mould::big::BigDish;

    fn round(name: &str, seed: u64, secs: f32, rules: Rules) -> Game<BigDish> {
        let mut g = Game::new(race_world(seed, 1, 1_000), rules, seed);
        let mut h = Hand::named(name, &g).expect("hand");
        let tps = g.rules.tps as u64;
        while g.end.is_none() && g.time() < secs {
            if g.world.ticks().is_multiple_of(tps) {
                h.play(&mut g);
            }
            g.step();
        }
        g
    }

    #[test]
    fn an_ai_round_is_a_replay() {
        let g = round("claim:claim_at=20+keep=2", 3, 60.0, Rules::default());
        assert!(g.log.len() > 4, "it gave commands: {}", g.log.len());
        let mut again = Game::new(race_world(3, 1, 1_000), Rules::default(), 3);
        assert!(Game::replay(&mut again, &g.replay_text(3)));
        assert_eq!(again.world.ticks(), g.world.ticks());
        assert_eq!(again.world.hash(), g.world.hash(), "the replay reaches the same world");
        assert_eq!(again.bank, g.bank);
    }

    #[test]
    fn styles_play_differently() {
        let mut logs = Vec::new();
        for s in ["boom", "claim:claim_at=10+keep=2", "strike:strike_at=10+strike_army=4+keep=1"] {
            let g = round(s, 2, 40.0, Rules::default());
            logs.push(g.log.iter().map(|c| c.1.to_line()).collect::<Vec<_>>());
        }
        assert_ne!(logs[0], logs[1]);
        assert_ne!(logs[0], logs[2]);
        assert!(logs[2].iter().any(|l| l.contains("attack")), "strike attacks: {:?}", logs[2]);
    }

    #[test]
    fn a_side_commands_only_its_own() {
        let mut g = Game::new(race_world(4, 1, 1_000), Rules::default(), 4);
        let side = Side { home: g.larder, ids: vec![0, 1], whole: false };
        let mut b = Brain::new(Style::BOOM, side);
        for _ in 0..30 {
            b.play(&mut g);
            for _ in 0..g.rules.tps {
                g.step();
            }
        }
        for (_, c) in &g.log {
            if let Cmd::Order(k, _) | Cmd::Stance(k, _) = c {
                assert!(b.side.ids.contains(k), "commanded {k}, owns {:?}", b.side.ids);
            }
        }
        assert!(g.soldiers[2].order == Order::Hold && g.soldiers[3].order == Order::Hold);
    }

    #[test]
    fn styles_take_knobs_by_name() {
        let s = Style::named("late:keep=3+claim_free=1").unwrap();
        assert_eq!((s.keep, s.claim_free, s.claim_at), (3, true, -90.0));
        assert!(Style::named("late:nonsense=3").is_none());
        assert!(Style::named("nobody").is_none());
    }

    #[test]
    fn the_view_sees_the_mould_on_its_home_crystal() {
        let g = Game::new(race_world(1, 1, 1_000), Rules::default(), 1);
        let v = View::of(&g, &Side::whole(&g));
        let home = v.crystals.iter().find(|c| dist(c.at, g.core) < 1.0).expect("core crystal");
        assert!(home.on >= HELD, "mould on its start: {}", home.on);
        let total: u32 = v.grid.as_ref().unwrap().iter().sum();
        assert_eq!(total as usize, g.world.living(), "the grid counts every walker");
    }
}
