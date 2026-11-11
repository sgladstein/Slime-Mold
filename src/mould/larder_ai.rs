//! **A player AI for Larder** (owner, 2026-11-11 14:07Z: "a player AI
//! thread, for automated testing so you can keep developing without me
//! having to playtest everything", and later "two soldier teams against
//! multiple molds so in the future the AI could also play soldiers").
//!
//! **Plays like a player, through the player's hands.** The AI gives only
//! `Cmd`s, through `Game::apply`, the same commands the window's mouse and
//! keys give, so every AI round is a replay file (`Game::replay_text`) and
//! nothing it does is out of a human's reach. It never calls `Game::shoot`:
//! a player fires by stance and burn order, so the AI does too. (The M1-M3
//! scripted plans in `examples/larderm2.rs` call `shoot`, `order` and the
//! game's RNG directly and are not replays.)
//!
//! **Sees what a player sees.** It reads a `View`, built each think from the
//! game: its own soldiers and turrets, the bank, the piles (food, and
//! whether mould is on one), the mould as walker counts on an 8-cell grid
//! (what the screen shows as tube, coarsened), a drop's smell once it is
//! smelt, and the clock. It does not read scent, walker headings or the
//! mould's hunger number.
//!
//! **One side, not the game.** It commands a `Side`: a home and the
//! soldiers it owns. Today's game has one squad, so a side is all of it
//! (`Side::whole`). Two soldier teams later are two sides with two brains;
//! the Larder thread's world version adds a team id to `Cmd` when it lands
//! (agreed 2026-11-11 14:12Z). Nothing here assumes there is one squad.
//!
//! **One brain, several styles.** The play is one set of rules (`Brain`)
//! turned by knobs (`Style`): how many soldiers guard home and when, how far
//! it farms, when it buys, whether it builds turrets or lays bait, when it
//! hunts the mould and how hurt a soldier may get before it is pulled
//! back. The presets (`Style::presets`) are plans a player would name:
//! turtle, boom, rush, lure, adapt. Knobs, not separate scripts, so a search
//! over them (an exploiter against a rule change) needs no new code.
//!
//! **A harness to port** (Larder thread, 14:12Z: the owner ruled no more
//! small-dish tuning). It runs on today's CPU field dish so it can be built
//! and checked now; its results here are about the AI, not the game's
//! balance. `examples/larderai.rs` runs it.

use super::larder::{Cmd, Game, Order, Rules, Stance};
use super::{Rng, H, W};

/// Cells a side of one mould-grid square.
pub const GRID: usize = 8;
const GW: usize = W.div_ceil(GRID);
const GH: usize = H.div_ceil(GRID);

fn dist(a: (f32, f32), b: (f32, f32)) -> f32 {
    (a.0 - b.0).hypot(a.1 - b.1)
}

/// The soldiers a brain commands and the home it holds.
#[derive(Clone, Debug)]
pub struct Side {
    pub home: (f32, f32),
    /// Soldier indices in `Game::soldiers` (indices never move: the dead
    /// stay in the list).
    pub ids: Vec<usize>,
    /// Every soldier, including ones bought later: today's one-squad game.
    pub whole: bool,
}

impl Side {
    pub fn whole(g: &Game) -> Side {
        Side {
            home: g.larder,
            ids: (0..g.soldiers.len()).collect(),
            whole: true,
        }
    }
}

#[derive(Clone, Copy, Debug)]
pub struct UnitView {
    pub id: usize,
    pub at: (f32, f32),
    pub hp: f32,
    pub order: Order,
    pub stance: Stance,
    pub carry: f32,
}

#[derive(Clone, Copy, Debug)]
pub struct PileView {
    pub at: (f32, f32),
    pub r: f32,
    pub food: f32,
    pub full: f32,
    /// Mould on it (the screen shows tube on the pile).
    pub eaten: bool,
}

/// **What one side's player sees.**
#[derive(Clone, Debug)]
pub struct View {
    pub t: f32,
    pub bank: f32,
    pub home: (f32, f32),
    /// The mould's home pile, and whether a soldier has taken it.
    pub core: (f32, f32),
    pub core_down: bool,
    pub units: Vec<UnitView>,
    pub turrets: Vec<(f32, f32)>,
    pub piles: Vec<PileView>,
    /// Walkers in each `GRID`-cell square, row-major `GW` by `GH`.
    pub mould: Vec<u32>,
    pub walkers: usize,
    /// A drop's spot once its smell is on the dish.
    pub drop: Option<(f32, f32)>,
    pub rules: Rules,
}

impl View {
    pub fn of(g: &Game, side: &Side) -> View {
        let mut mould = vec![0u32; GW * GH];
        for w in &g.dish.walkers {
            let (gx, gy) = ((w.x as usize / GRID).min(GW - 1), (w.y as usize / GRID).min(GH - 1));
            mould[gy * GW + gx] += 1;
        }
        let ids: Vec<usize> = if side.whole {
            (0..g.soldiers.len()).collect()
        } else {
            side.ids.clone()
        };
        let units = ids
            .into_iter()
            .filter(|&k| g.soldiers[k].alive)
            .map(|k| {
                let s = g.soldiers[k];
                UnitView {
                    id: k,
                    at: (s.x, s.y),
                    hp: s.hp,
                    order: s.order,
                    stance: s.stance,
                    carry: s.carry,
                }
            })
            .collect();
        let warn = super::larder::secs(g.rules.drop_warn);
        View {
            t: g.time(),
            bank: g.bank,
            home: side.home,
            core: g.core,
            core_down: g.core_down.is_some(),
            units,
            turrets: g.turrets.iter().map(|t| (t.x, t.y)).collect(),
            piles: g
                .dish
                .piles
                .iter()
                .map(|p| PileView {
                    at: (p.x, p.y),
                    r: p.r,
                    food: p.food,
                    full: p.full,
                    eaten: p.eaters > 0,
                })
                .collect(),
            mould,
            walkers: g.dish.walkers.len(),
            drop: g.next_drop.filter(|&(when, _)| g.dish.t + warn >= when).map(|d| d.1),
            rules: g.rules,
        }
    }

    /// Walkers in grid squares whose centre is within `r` of `c`.
    pub fn mass(&self, c: (f32, f32), r: f32) -> u32 {
        let (mut n, h) = (0, GRID as f32 / 2.0);
        let lo = |v: f32| ((v - r - h).max(0.0) as usize) / GRID;
        let (x1, y1) = (
            (((c.0 + r + h).max(0.0)) as usize / GRID).min(GW - 1),
            (((c.1 + r + h).max(0.0)) as usize / GRID).min(GH - 1),
        );
        for gy in lo(c.1)..=y1 {
            for gx in lo(c.0)..=x1 {
                let m = self.mould[gy * GW + gx];
                if m > 0 && dist(self.square(gx, gy), c) <= r {
                    n += m;
                }
            }
        }
        n
    }

    fn square(&self, gx: usize, gy: usize) -> (f32, f32) {
        (
            (gx * GRID) as f32 + GRID as f32 / 2.0,
            (gy * GRID) as f32 + GRID as f32 / 2.0,
        )
    }

    /// The centre of the mould within `r` of `c`, weighted by walkers, and
    /// how many walkers that is.
    pub fn centroid(&self, c: (f32, f32), r: f32) -> Option<((f32, f32), u32)> {
        let (mut sx, mut sy, mut n) = (0.0, 0.0, 0u32);
        for gy in 0..GH {
            for gx in 0..GW {
                let m = self.mould[gy * GW + gx];
                let p = self.square(gx, gy);
                if m > 0 && dist(p, c) <= r {
                    sx += p.0 * m as f32;
                    sy += p.1 * m as f32;
                    n += m;
                }
            }
        }
        (n > 0).then(|| ((sx / n as f32, sy / n as f32), n))
    }

    /// The nearest grid square with at least `min` walkers.
    pub fn nearest_mould(&self, c: (f32, f32), min: u32) -> Option<(f32, f32)> {
        let mut best: Option<(f32, (f32, f32))> = None;
        for gy in 0..GH {
            for gx in 0..GW {
                if self.mould[gy * GW + gx] >= min {
                    let p = self.square(gx, gy);
                    let d = dist(p, c);
                    if best.is_none_or(|b| d < b.0) {
                        best = Some((d, p));
                    }
                }
            }
        }
        best.map(|b| b.1)
    }

    fn is_core(&self, p: (f32, f32)) -> bool {
        dist(p, self.core) < 1.0
    }
}

/// **The knobs one brain is turned by.** Distances in cells, times in game
/// seconds, food in food. `usize::MAX` and `u32::MAX` mean no limit.
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct Style {
    pub name: &'static str,
    /// Mould within `alarm_r` of home, in walkers, that rings the alarm:
    /// the guard turns out and bait is laid.
    pub alarm: u32,
    pub alarm_r: f32,
    /// Soldiers that burn an arc round home while the alarm is up (and 10 s
    /// after); with `always_guard` they burn it from the start.
    pub guard: usize,
    pub always_guard: bool,
    /// Piles farmed at once, one soldier each, nearest first; `farm_small`
    /// once the mould is small (below `hunt_small`).
    pub farm: usize,
    pub farm_small: usize,
    /// Mould within 12 cells of a pile, in walkers, past which a lone
    /// farmer is not sent to it.
    pub dare: u32,
    /// Food kept in the bank before buying anything, and game seconds of
    /// fire for the whole squad kept on top of it.
    pub reserve: f32,
    pub ammo: f32,
    /// Buy soldiers up to `buy_to`, but only while the mould has fewer than
    /// `buy_below` walkers (M2: soldiers bought against a big mould die and
    /// feed it).
    pub buy_to: usize,
    pub buy_below: usize,
    pub turrets: usize,
    /// Lay bait toward the mould when the alarm rings and it is not small.
    pub bait: bool,
    pub bait_food: f32,
    /// The mould is **small** below `hunt_small` walkers, read from
    /// `hunt_after` on. Hunt when it is small and the squad is `hunt_army`
    /// strong, or from `strike_at` whatever it sees.
    pub hunt_small: usize,
    pub hunt_after: f32,
    pub hunt_army: usize,
    pub strike_at: f32,
    /// Hunters fire freely; otherwise they defend.
    pub free_fire: bool,
    /// A soldier below this share of full health with mould near it walks
    /// home and stays on the larder (health never comes back, and a dead
    /// soldier is 400 food for the mould). 0 never pulls back.
    pub retreat: f32,
    /// Walk a spare soldier to a drop's spot while it smells.
    pub drops: bool,
}

impl Style {
    /// **Adapt**: M2's reader, the best scripted plan of M2 and M3, played
    /// through commands. Bait and farm three piles while the mould is big;
    /// once it is small, farm eight with a soldier bought for each; hunt
    /// with sixteen. What it reads is on screen: the mould's size and its
    /// own squad.
    pub const ADAPT: Style = Style {
        name: "adapt",
        alarm: 20,
        alarm_r: 50.0,
        guard: 0,
        always_guard: false,
        farm: 3,
        farm_small: 8,
        dare: u32::MAX,
        reserve: 600.0,
        ammo: 30.0,
        buy_to: 8,
        buy_below: 3000,
        turrets: 0,
        bait: true,
        bait_food: 300.0,
        hunt_small: 3000,
        hunt_after: 120.0,
        hunt_army: 16,
        strike_at: f32::INFINITY,
        free_fire: true,
        retreat: 0.0,
        drops: false,
    };

    /// **Turtle**: a burning ring round home from the start, turrets on a
    /// ring outside it, the larder farmed and nothing else; hunts only a
    /// nearly dead mould (M1's fortress).
    pub const TURTLE: Style = Style {
        name: "turtle",
        guard: 3,
        always_guard: true,
        farm: 1,
        farm_small: 1,
        buy_to: 0,
        turrets: 6,
        bait: false,
        hunt_small: 400,
        hunt_army: 0,
        ..Style::ADAPT
    };

    /// **Boom**: farm wide and buy a soldier a pile whatever the mould's
    /// size, no bait, then hunt with 24 (M3's farmer, with a hunt).
    /// Tried and reverted, seeds 1-8: two guards burning round home when
    /// 60 walkers came within 30 cells. The squad was wiped on 8 of 8 in
    /// both fields (median end 145 s); without them it held to the cap 8 of
    /// 8 fed and won 8 of 8 lean.
    pub const BOOM: Style = Style {
        name: "boom",
        farm: 10,
        farm_small: 10,
        reserve: 400.0,
        buy_to: 24,
        buy_below: usize::MAX,
        bait: false,
        hunt_small: usize::MAX,
        hunt_after: 0.0,
        hunt_army: 24,
        ..Style::ADAPT
    };

    /// **Rush**: spend the bank on soldiers at once and go for the mould
    /// from the first second (M2's hunter).
    pub const RUSH: Style = Style {
        name: "rush",
        farm: 0,
        farm_small: 0,
        reserve: 0.0,
        buy_to: 40,
        buy_below: usize::MAX,
        bait: false,
        hunt_small: usize::MAX,
        hunt_after: 0.0,
        hunt_army: 0,
        ..Style::ADAPT
    };

    /// **Lure**: bait and farm three piles, buy nothing until 6:00, then buy
    /// and hunt whatever it sees (M3's striker).
    pub const LURE: Style = Style {
        name: "lure",
        farm_small: 3,
        buy_below: 0,
        hunt_small: 0,
        hunt_army: usize::MAX,
        strike_at: 360.0,
        ..Style::ADAPT
    };

    pub fn presets() -> [Style; 5] {
        [Style::TURTLE, Style::BOOM, Style::RUSH, Style::LURE, Style::ADAPT]
    }

    /// A preset by name, with knobs changed after a colon:
    /// `lure:buy_to=4+guard=0` (`+`, as the harness splits hands on commas).
    pub fn named(name: &str) -> Option<Style> {
        let (base, knobs) = name.split_once(':').unwrap_or((name, ""));
        let mut s = Style::presets().into_iter().find(|s| s.name == base)?;
        for kv in knobs.split('+').filter(|k| !k.is_empty()) {
            let (k, v) = kv.split_once('=')?;
            s.set(k, v.parse().ok()?).then_some(())?;
        }
        Some(s)
    }

    /// Set one knob by name; booleans are 0 or 1, "inf" is no limit.
    pub fn set(&mut self, k: &str, v: f32) -> bool {
        let n = if v.is_infinite() { usize::MAX } else { v as usize };
        match k {
            "alarm" => self.alarm = if v.is_infinite() { u32::MAX } else { v as u32 },
            "alarm_r" => self.alarm_r = v,
            "guard" => self.guard = n,
            "always_guard" => self.always_guard = v != 0.0,
            "farm" => self.farm = n,
            "farm_small" => self.farm_small = n,
            "buy_below" => self.buy_below = n,
            "dare" => self.dare = if v.is_infinite() { u32::MAX } else { v as u32 },
            "reserve" => self.reserve = v,
            "ammo" => self.ammo = v,
            "buy_to" => self.buy_to = n,
            "turrets" => self.turrets = n,
            "bait" => self.bait = v != 0.0,
            "bait_food" => self.bait_food = v,
            "hunt_small" => self.hunt_small = n,
            "hunt_after" => self.hunt_after = v,
            "hunt_army" => self.hunt_army = n,
            "strike_at" => self.strike_at = v,
            "free_fire" => self.free_fire = v != 0.0,
            "retreat" => self.retreat = v,
            "drops" => self.drops = v != 0.0,
            _ => return false,
        }
        true
    }
}

/// What a brain has its soldiers doing this think.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Mode {
    Farm,
    Hunt,
}

/// **The AI player.** Call `think` once a game second; apply what it gives.
pub struct Brain {
    pub style: Style,
    pub side: Side,
    pub mode: Mode,
    /// Soldiers pulled back hurt: they guard home from then on.
    hurt: Vec<usize>,
    /// Game second the alarm last rang; the guard stands down 10 s after.
    alarm_at: f32,
    target: Option<(f32, f32)>,
    tries: u32,
    /// The mould read small at the last think.
    small: bool,
}

/// Radius of the burning arc round home, cells (M1's fortress ring).
const GUARD_R: f32 = 10.0;
/// Turrets stand on this ring round home.
const TURRET_R: f32 = 16.0;
/// Bait goes this far from home toward the mould.
const BAIT_AWAY: f32 = 35.0;

impl Brain {
    pub fn new(style: Style, side: Side) -> Brain {
        Brain {
            style,
            side,
            mode: Mode::Farm,
            hurt: Vec::new(),
            alarm_at: f32::NEG_INFINITY,
            target: None,
            tries: 0,
            small: false,
        }
    }

    /// Look, decide, and play the commands on `g`. Returns how many
    /// commands it gave.
    pub fn play(&mut self, g: &mut Game) -> usize {
        let before = g.soldiers.len();
        let cmds = self.think(&View::of(g, &self.side));
        for &c in &cmds {
            g.apply(c);
        }
        if !self.side.whole {
            // Soldiers bought this think are ours.
            self.side.ids.extend(before..g.soldiers.len());
        }
        cmds.len()
    }

    /// The commands for this game second. Gives an order only when it
    /// differs from the soldier's current one, so a replay stays short and
    /// a soldier is not reset mid-task.
    pub fn think(&mut self, v: &View) -> Vec<Cmd> {
        let st = self.style;
        let r = v.rules;
        let mut out = Vec::new();
        let mut bank = v.bank;
        let squad = v.units.len();
        if squad == 0 {
            return out;
        }

        // ---------------------------------------------------- the mode
        let small = v.t >= st.hunt_after && v.walkers < st.hunt_small;
        self.mode = if (small && squad >= st.hunt_army) || v.t >= st.strike_at {
            Mode::Hunt
        } else {
            Mode::Farm
        };
        self.small = small;

        // ------------------------------------------------- spending
        let buying = v.walkers < st.buy_below || self.mode == Mode::Hunt;
        let buy_to = if self.mode == Mode::Hunt {
            r.max_squad
        } else {
            st.buy_to.min(r.max_squad)
        };
        // Never buy a soldier the bank cannot arm: after the buy, `ammo`
        // game seconds of fire for the whole squad must be left (first
        // census, lure seed 1: 28 soldiers bought at 6:00 emptied the bank,
        // none could fire, and 28 died in 25 s, each a meal for the mould).
        let arm = |n: usize| st.reserve + n as f32 * r.price / r.shot_every * st.ammo;
        let mut n = squad;
        while buying && n < buy_to && bank >= r.soldier_cost + arm(n + 1) {
            out.push(Cmd::Buy);
            bank -= r.soldier_cost;
            n += 1;
            // A rush buys all it can at once; others one a think.
            if st.reserve > 0.0 {
                break;
            }
        }
        let threat = v.centroid(v.home, st.alarm_r);
        if v.turrets.len() < st.turrets && bank >= st.reserve + r.turret_cost {
            // On the ring, facing the mould (or its core), a new angle each
            // try so a blocked spot is not tried forever.
            let toward = threat.map(|t| t.0).unwrap_or(v.core);
            let a0 = (toward.1 - v.home.1).atan2(toward.0 - v.home.0);
            let k = v.turrets.len() as f32 + self.tries as f32 * 0.37;
            let a = a0 + (k * 2.399_963).sin() * 1.2;
            out.push(Cmd::Turret((
                v.home.0 + TURRET_R * a.cos(),
                v.home.1 + TURRET_R * a.sin(),
            )));
            self.tries += 1;
            bank -= r.turret_cost;
        }
        let alarm = threat.is_some_and(|t| t.1 >= st.alarm);
        if alarm {
            self.alarm_at = v.t;
        }
        // Bait diverts a big mould; a small one is farmed past and hunted
        // (the reader baited only while the mould was big).
        // Not while hunting: the hunt fights the mould near home, its
        // bait is eaten the second it lands and a new one is laid each think
        // (first census, lure seed 1: 300 food a second from 6:00 on).
        if st.bait && alarm && !self.small && self.mode == Mode::Farm && bank >= st.reserve + st.bait_food {
            let uneaten = v
                .piles
                .iter()
                .any(|p| p.full == st.bait_food && p.food > 0.0 && !p.eaten);
            if !uneaten {
                let c = threat.expect("alarm").0;
                let d = dist(c, v.home).max(0.1);
                let at = (
                    v.home.0 + (c.0 - v.home.0) / d * BAIT_AWAY,
                    v.home.1 + (c.1 - v.home.1) / d * BAIT_AWAY,
                );
                // Within reach of the nearest soldier.
                let s = v
                    .units
                    .iter()
                    .min_by(|a, b| dist(a.at, at).total_cmp(&dist(b.at, at)))
                    .expect("squad")
                    .at;
                let e = dist(s, at).max(0.1);
                let reach = e.min(r.range - 1.0);
                out.push(Cmd::Bait(
                    ((s.0 + (at.0 - s.0) / e * reach), (s.1 + (at.1 - s.1) / e * reach)),
                    st.bait_food,
                ));
            }
        }

        // ---------------------------------------------------- roles
        let mut free: Vec<UnitView> = Vec::new();
        let mut want: Vec<(usize, Order, Stance)> = Vec::new();
        for u in &v.units {
            let near = v.mass(u.at, 6.0);
            if st.retreat > 0.0 && !self.hurt.contains(&u.id) && u.hp < st.retreat * r.hp && near > 0 {
                self.hurt.push(u.id);
            }
            if self.hurt.contains(&u.id) {
                want.push((u.id, Order::Clear(v.home.0, v.home.1), Stance::Defend));
            } else {
                free.push(*u);
            }
        }
        // The guard: nearest to home first.
        let guarding = st.always_guard || v.t - self.alarm_at < 10.0;
        let n_guard = if guarding { st.guard.min(free.len()) } else { 0 };
        free.sort_by(|a, b| dist(a.at, v.home).total_cmp(&dist(b.at, v.home)).then(a.id.cmp(&b.id)));
        let guards: Vec<UnitView> = free.drain(..n_guard).collect();
        if !guards.is_empty() {
            // An arc facing the mould, a radian a guard; the whole ring with
            // three or more.
            let toward = threat.map(|t| t.0).unwrap_or(v.core);
            let mid = (toward.1 - v.home.1).atan2(toward.0 - v.home.0);
            let span = (guards.len() as f32).min(std::f32::consts::TAU);
            let n = guards.len();
            for (j, u) in guards.iter().enumerate() {
                let a0 = mid - span / 2.0 + span * j as f32 / n as f32;
                let a1 = mid - span / 2.0 + span * (j + 1) as f32 / n as f32;
                let p = |a: f32| (v.home.0 + GUARD_R * a.cos(), v.home.1 + GUARD_R * a.sin());
                let o = Order::Burn(p(a0), p(a1));
                // Keep a ring burning that is still close to the right one,
                // so the guard is not re-sent every time the mould shifts.
                let keep = matches!(u.order, Order::Burn(a, b) if dist(a, p(a0)) < 4.0 && dist(b, p(a1)) < 4.0);
                want.push((u.id, if keep { u.order } else { o }, Stance::Defend));
            }
        }

        match self.mode {
            Mode::Farm => self.farm(v, &free, &mut want),
            Mode::Hunt => self.hunt(v, &free, &mut want),
        }

        for (id, o, s) in want {
            let u = v.units.iter().find(|u| u.id == id).expect("own soldier");
            if u.order != o {
                out.push(Cmd::Order(id, o));
            }
            if u.stance != s {
                out.push(Cmd::Stance(id, s));
            }
        }
        out
    }

    /// One soldier a pile, the best piles first: food over distance from
    /// home, leaving out piles under more mould than `dare`. A soldier
    /// already clearing a chosen pile keeps it; spare soldiers stand by the
    /// larder, or walk to a smelt drop.
    fn farm(&mut self, v: &View, free: &[UnitView], want: &mut Vec<(usize, Order, Stance)>) {
        let st = self.style;
        let mut piles: Vec<(f32, (f32, f32))> = v
            .piles
            .iter()
            .filter(|p| p.food > 50.0 && !v.is_core(p.at) && dist(p.at, v.home) > 0.5)
            .filter(|p| v.mass(p.at, 12.0) <= st.dare)
            .map(|p| (dist(p.at, v.home), p.at))
            .collect();
        // Nearest first, as M2's farmer: a far pile is a long walk through
        // mould (first census: ranking by food over distance sent farmers
        // past the mould and the squad was wiped on 4 of 4 seeds).
        piles.sort_by(|a, b| a.0.total_cmp(&b.0));
        let farm = if self.small { st.farm_small } else { st.farm };
        let mut chosen: Vec<(f32, f32)> = piles.iter().take(farm).map(|p| p.1).collect();
        // The larder is always farmed first when it has food (its food in
        // the bank cannot be eaten).
        if farm > 0 && v.piles.iter().any(|p| dist(p.at, v.home) <= 0.5 && p.food > 50.0) {
            chosen.insert(0, v.home);
            chosen.truncate(farm);
        }
        let mut taken = vec![false; chosen.len()];
        let mut idle = Vec::new();
        for u in free {
            let mine = match u.order {
                Order::Clear(x, y) => chosen.iter().position(|&c| dist(c, (x, y)) < 0.5),
                _ => None,
            };
            match mine {
                Some(i) if !taken[i] => {
                    taken[i] = true;
                    want.push((u.id, u.order, Stance::Defend));
                }
                _ => idle.push(*u),
            }
        }
        for u in idle {
            // Nearest open chosen pile to this soldier.
            let pick = (0..chosen.len())
                .filter(|&i| !taken[i])
                .min_by(|&a, &b| dist(chosen[a], u.at).total_cmp(&dist(chosen[b], u.at)));
            let o = match pick {
                Some(i) => {
                    taken[i] = true;
                    Order::Clear(chosen[i].0, chosen[i].1)
                }
                None => match v.drop {
                    Some(d) if st.drops && v.mass(d, 12.0) <= st.dare => Order::Move(d.0, d.1),
                    _ => {
                        if v.piles.iter().any(|p| dist(p.at, v.home) <= 0.5) {
                            Order::Clear(v.home.0, v.home.1)
                        } else {
                            Order::Move(v.home.0, v.home.1)
                        }
                    }
                },
            };
            want.push((u.id, o, Stance::Defend));
        }
    }

    /// The whole free squad on one eaten pile at a time, the one nearest
    /// home first, kept until it is clear; then the core; once the core is
    /// down, the nearest mould.
    fn hunt(&mut self, v: &View, free: &[UnitView], want: &mut Vec<(usize, Order, Stance)>) {
        let st = self.style;
        let stance = if st.free_fire { Stance::Free } else { Stance::Defend };
        // A target is kept while it has food and mould near it, not only
        // while mould is on it: a volley clears the pile for a moment, and
        // re-aiming then walked the squad between piles every few seconds
        // (lure seed 1, first census: three targets in 9 s).
        let live = |p: (f32, f32)| {
            v.piles
                .iter()
                .any(|q| dist(q.at, p) < 0.5 && q.food > 0.0 && (q.eaten || v.mass(p, q.r + 8.0) > 0))
        };
        if self.target.is_some_and(|t| !live(t)) {
            self.target = None;
        }
        if self.target.is_none() {
            self.target = v
                .piles
                .iter()
                .filter(|p| p.eaten && p.food > 0.0 && !v.is_core(p.at))
                .min_by(|a, b| dist(a.at, v.home).total_cmp(&dist(b.at, v.home)))
                .map(|p| p.at);
        }
        for u in free {
            let o = match self.target {
                Some(p) => Order::Clear(p.0, p.1),
                // Nothing eaten but the core, and the mould big: hold the
                // larder (M1: marching on a big mould's core at 0 s
                // wiped the squad by 38 s).
                None if !self.small => Order::Clear(v.home.0, v.home.1),
                None if !v.core_down => Order::Attack,
                None => match v.nearest_mould(u.at, 1) {
                    Some(m) => Order::Move(m.0, m.1),
                    None => Order::Hold,
                },
            };
            // A walk toward the nearest mould is re-aimed only when the
            // mould has moved off; otherwise every think would re-send it.
            let o = match (u.order, o) {
                (Order::Move(x, y), Order::Move(a, b)) if dist((x, y), (a, b)) < GRID as f32 * 1.5 => u.order,
                _ => o,
            };
            want.push((u.id, o, stance));
        }
    }
}

/// **The random hand**, a control: once a game second, with chance `rate`,
/// one random legal-looking command (an order to a random soldier to a
/// random spot or pile, a stance, a buy, a turret or bait near a soldier).
/// A good game is one a plan wins and this loses.
pub struct RandomHand {
    pub rng: Rng,
    pub rate: f32,
    pub side: Side,
}

impl RandomHand {
    pub fn new(seed: u64, side: Side) -> RandomHand {
        RandomHand {
            rng: Rng::new(seed ^ 0x7a1d_0a1d),
            rate: 0.5,
            side,
        }
    }

    pub fn play(&mut self, g: &mut Game) -> usize {
        let v = View::of(g, &self.side);
        if v.units.is_empty() || self.rng.f32() >= self.rate {
            return 0;
        }
        let u = v.units[self.rng.below(v.units.len() as u32) as usize];
        let spot = |rng: &mut Rng| (rng.f32() * W as f32, rng.f32() * H as f32);
        let near = |rng: &mut Rng| {
            let a = rng.f32() * std::f32::consts::TAU;
            let d = rng.f32() * (v.rules.range - 1.0);
            (u.at.0 + d * a.cos(), u.at.1 + d * a.sin())
        };
        let c = match self.rng.below(10) {
            0..=2 => Cmd::Order(u.id, Order::Move(spot(&mut self.rng).0, spot(&mut self.rng).1)),
            3..=4 => {
                let p = v.piles[self.rng.below(v.piles.len() as u32) as usize].at;
                Cmd::Order(u.id, Order::Clear(p.0, p.1))
            }
            5 => Cmd::Order(u.id, Order::Burn(near(&mut self.rng), near(&mut self.rng))),
            6 => Cmd::Stance(
                u.id,
                [Stance::HoldFire, Stance::Defend, Stance::Free][self.rng.below(3) as usize],
            ),
            7 => Cmd::Buy,
            8 => Cmd::Turret(near(&mut self.rng)),
            _ => Cmd::Bait(near(&mut self.rng), 300.0),
        };
        let before = g.soldiers.len();
        g.apply(c);
        if !self.side.whole {
            self.side.ids.extend(before..g.soldiers.len());
        }
        1
    }
}

/// Who plays a round: nobody, the random hand, or a brain.
pub enum Hand {
    Idle,
    Random(RandomHand),
    Brain(Box<Brain>),
}

impl Hand {
    /// `idle`, `random`, or a preset's name.
    pub fn named(name: &str, g: &Game, seed: u64) -> Option<Hand> {
        Some(match name {
            "idle" => Hand::Idle,
            "random" => Hand::Random(RandomHand::new(seed, Side::whole(g))),
            _ => Hand::Brain(Box::new(Brain::new(Style::named(name)?, Side::whole(g)))),
        })
    }

    /// Its turn: call once a game second, before the step.
    pub fn play(&mut self, g: &mut Game) -> usize {
        match self {
            Hand::Idle => 0,
            Hand::Random(r) => r.play(g),
            Hand::Brain(b) => b.play(g),
        }
    }
}

/// **Set one rule by name**, for balance tests from the command line
/// (`set=price=20`). False if there is no such rule.
pub fn set_rule(r: &mut Rules, name: &str, v: f32) -> bool {
    let f = match name {
        "speed" => &mut r.speed,
        "range" => &mut r.range,
        "line" => &mut r.line,
        "width" => &mut r.width,
        "shot_every" => &mut r.shot_every,
        "price" => &mut r.price,
        "wound" => &mut r.wound,
        "hp" => &mut r.hp,
        "drain" => &mut r.drain,
        "touch" => &mut r.touch,
        "smell" => &mut r.smell,
        "body" => &mut r.body,
        "harvest" => &mut r.harvest,
        "carry" => &mut r.carry,
        "core_r" => &mut r.core_r,
        "core_secs" => &mut r.core_secs,
        "cap" => &mut r.cap,
        "start_bank" => &mut r.start_bank,
        "soldier_cost" => &mut r.soldier_cost,
        "turret_cost" => &mut r.turret_cost,
        "turret_hp" => &mut r.turret_hp,
        "turret_range" => &mut r.turret_range,
        "hoard" => &mut r.hoard,
        "hunger" => &mut r.hunger,
        "drop_every" => &mut r.drop_every,
        "drop_food" => &mut r.drop_food,
        "drop_warn" => &mut r.drop_warn,
        "drops_stop" => &mut r.drops_stop,
        "squad" => {
            r.squad = v as usize;
            return true;
        }
        "max_squad" => {
            r.max_squad = v as usize;
            return true;
        }
        _ => return false,
    };
    *f = v;
    true
}

#[cfg(test)]
mod tests {
    use super::super::larder::{field_game, GAME_SECOND};
    use super::*;

    fn round(name: &str, seed: u64, secs: u64) -> Game {
        let mut g = field_game(seed, Rules::default());
        let mut h = Hand::named(name, &g, seed).expect("hand");
        while g.end.is_none() && g.dish.t < secs * GAME_SECOND {
            if g.dish.t.is_multiple_of(GAME_SECOND) {
                h.play(&mut g);
            }
            g.step();
        }
        g
    }

    #[test]
    fn an_ai_round_is_a_replay() {
        for name in ["adapt", "random"] {
            let g = round(name, 3, 90);
            assert!(!g.log.is_empty(), "{name} gave commands");
            let again = Game::replay(&g.replay_text(3), Rules::default()).expect("parses");
            assert_eq!(again.dish.state_hash(), g.dish.state_hash(), "{name}");
            assert_eq!(again.bank.to_bits(), g.bank.to_bits(), "{name}");
        }
    }

    #[test]
    fn styles_play_differently() {
        let rush = round("rush", 2, 20);
        let turtle = round("turtle", 2, 20);
        assert!(rush.bought > 0, "rush buys at once");
        assert_eq!(turtle.bought, 0, "turtle keeps its reserve");
        assert!(
            turtle
                .soldiers
                .iter()
                .filter(|s| matches!(s.order, Order::Burn(..)))
                .count()
                >= 3,
            "turtle rings home"
        );
        assert!(rush
            .soldiers
            .iter()
            .any(|s| s.alive && matches!(s.order, Order::Attack | Order::Clear(..))));
    }

    #[test]
    fn a_side_commands_only_its_own() {
        let mut g = field_game(1, Rules::default());
        let side = Side {
            home: g.larder,
            ids: vec![0, 1],
            whole: false,
        };
        let mut b = Brain::new(Style::BOOM, side);
        for _ in 0..30 {
            b.play(&mut g);
            for _ in 0..GAME_SECOND {
                g.step();
            }
        }
        for (_, c) in &g.log {
            if let Cmd::Order(k, _) | Cmd::Stance(k, _) = c {
                assert!(b.side.ids.contains(k), "ordered soldier {k}, not its own");
            }
        }
        assert!(g.soldiers[2].order == Order::Hold && g.soldiers[3].order == Order::Hold);
    }

    #[test]
    fn styles_take_knobs_by_name() {
        let s = Style::named("lure:buy_to=4+bait=0+dare=inf").expect("parses");
        assert_eq!((s.buy_to, s.bait, s.dare, s.strike_at), (4, false, u32::MAX, 360.0));
        assert!(Style::named("lure:nonsense=1").is_none());
        assert!(Style::named("nobody").is_none());
    }

    #[test]
    fn rules_set_by_name() {
        let mut r = Rules::default();
        assert!(set_rule(&mut r, "price", 25.0));
        assert!(set_rule(&mut r, "squad", 6.0));
        assert!(!set_rule(&mut r, "nonsense", 1.0));
        assert_eq!((r.price, r.squad), (25.0, 6));
    }
}
