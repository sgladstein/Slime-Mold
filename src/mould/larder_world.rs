//! **Larder in the world** (ROADMAP W2b; the plan is the *W2 plan* section of
//! `Reports/larder-world-roadmap-2026-11-11.md`). M3's squad (`larder.rs`)
//! on the Larder World: a growing mould that starts on one crystal, crystals
//! that run dry, rock, wet ground and walls the mould wears down. No window
//! here; `examples/larderworld.rs` plays idle, random and planned hands on
//! paired seeds, and the window (W2c) drives the same `Game`.
//!
//! **Two worlds, one game.** The rules run over `World`, which the CPU big
//! dish implements here (tests, CI and small censuses). The GPU engine is
//! to implement it next (play, and censuses on the GTX 1070); it first
//! needs the CPU race world's pace, since the race world takes `scale`
//! fine steps a tick and the engine one. Everything
//! the squad does to the mould and reads from it goes through one squad
//! frame (`squad::Frame`, W2a): shots that stop at the first slime and
//! strike there, the soldiers' smell, then the ticks, then each soldier's
//! touch, its densest mould in sight and its harvest. So the game never
//! reads the world's fields itself, and the GPU needs one small read-back a
//! frame.
//!
//! **What changed from M3**, and why:
//! - Food is crystal charge for both sides (the plan): a clear order
//!   harvests charge from the crystal's cells into the soldier's carry,
//!   walked home to the larder. What the squad takes the mould can't eat.
//!   The bank counts food, `Rules::charge` charge to one food, so M3's
//!   prices carry over.
//! - A shot is a ray from the soldier toward its target that stops at the
//!   first cell that looks like slime, or at rock or a wall, and strikes a
//!   disc there (owner, 16:48Z: "Gun fire should hit the first slime in its
//!   path"). M3's shot was a line laid across the target point.
//! - The aim is the densest 4 x 4 block in sight from the frame before
//!   (`UnitOut::target`), so a soldier fires on the mould's thickest part
//!   it can see, and the ray stops on whatever slime is in front of it.
//! - Soldiers walk round rock and walls on a distance map to their goal
//!   (`distance_map`), and fire only along a clear line (`clear_line`).
//!   M3's dish had no rock. Without either, on seed 1 three of four
//!   rushing soldiers stood behind rock 50 cells short of the core and put
//!   196 of 200 shots into it; with both they reached the core and killed
//!   1,282 walkers with 200 shots, against 13.
//! - The mould steals from the larder: walkers touching it take
//!   `Rules::steal` food a game second from the bank. In M3 the larder was
//!   a pile the mould ate; here the bank is not on the ground, and without
//!   a cost at home an idle squad behind a wall loses nothing.
//! - Not yet ported: turrets, bait, hunger (it barely acted in M2) and
//!   bodies as crystals (they need a spare crystal slot on both worlds).
//!
//! Distances in `Rules` are dish cells, as in M3 (the 512 x 320 world at
//! scale 2 is M3's 256 x 160 dish); the game works in fine cells and
//! converts through `World::scale`. Time is game seconds of `Rules::tps`
//! ticks.

use super::big::{BigDish, BigParams, LIFE_DEAD};
use super::squad::{self, Frame, FrameOut, ShotIn, UnitIn, UnitOut};
use super::Rng;

/// The world under the game: what the rules need of it, and nothing else.
pub trait World {
    /// Fine cells across and down.
    fn size(&self) -> (usize, usize);
    /// Fine cells a dish cell.
    fn scale(&self) -> f32;
    /// Ticks run.
    fn ticks(&self) -> u64;
    /// One squad frame around `ticks` ticks (`squad::frame`).
    fn frame(&mut self, f: &Frame, ticks: u32) -> FrameOut;
    /// Starts frame `f` as `frame` does, without waiting for its results
    /// where the world can (the GPU world): then None, and `frame_finish`
    /// gives them. The default runs the frame at once.
    fn frame_start(&mut self, f: &Frame, ticks: u32) -> Option<FrameOut> {
        Some(self.frame(f, ticks))
    }
    /// The results of the frame `frame_start` left running, waiting for
    /// them if they are not in. Only called after a `frame_start` that
    /// returned None.
    fn frame_finish(&mut self) -> FrameOut {
        unreachable!("no frame running")
    }
    /// Whether `frame_finish` would return without waiting.
    fn frame_ready(&self) -> bool {
        true
    }
    /// A wall from `a` to `b`, dish cells, `width` dish cells thick.
    fn build_wall(&mut self, a: (f32, f32), b: (f32, f32), width: f32);
    /// Walkers alive (not counting the dead).
    fn living(&self) -> usize;
    /// Each crystal's centre (fine cells) and charge left.
    fn crystals(&self) -> Vec<((f32, f32), f32)>;
    /// Crystal `c`'s radius, fine cells.
    fn crystal_r(&self, c: usize) -> f32;
    /// Where the squad lands, fine cells, if the world places it (a map,
    /// `BigDish::with_map`); else the game puts the larder on the crystal
    /// farthest from the mould's home (`far_crystal`).
    fn landing(&self) -> Option<(f32, f32)> {
        None
    }
    /// Open ground a soldier can stand on (agar: not rock, wall or rim).
    fn open(&self, x: f32, y: f32) -> bool;
    /// The mould's core is taken: no more births.
    fn stop_births(&mut self);
    /// A full pile's scent a step, which a soldier's smell is a share of.
    fn food_deposit(&self) -> f32;
    fn hash(&self) -> u64;
    /// Walkers on each `bin` x `bin` square of fine cells, row by row (the
    /// tube a player sees on screen, coarsened), for the player AI's eyes
    /// (`larder_world_ai::View`). None where the world can't give it
    /// cheaply: the GPU world needs an occupancy read-back first.
    fn mould_grid(&self, _bin: usize) -> Option<Vec<u32>> {
        None
    }
    /// For each point (fine cells), the gassed sleeping walkers within `r`
    /// (`BigDish::gassed_near`), which don't bite; 0 where the world has
    /// no sleep gas (the GPU world, so far).
    fn calm(&self, at: &[(f32, f32)], _r: f32) -> Vec<u32> {
        vec![0; at.len()]
    }
    /// The player's own mould (`own_mould::Garden`): the soldiers' tasks,
    /// once a frame before the frame's `ticks`, with the bank they may
    /// spend; returns the food spent. Worlds with no player mould do
    /// nothing.
    fn tend(&mut self, _soldiers: &[Soldier], _bank: f32, _ticks: u32) -> f32 {
        0.0
    }
}

impl World for BigDish {
    fn size(&self) -> (usize, usize) {
        (self.w, self.h)
    }

    fn scale(&self) -> f32 {
        self.p.scale as f32
    }

    fn ticks(&self) -> u64 {
        self.ticks
    }

    fn frame(&mut self, f: &Frame, ticks: u32) -> FrameOut {
        squad::frame(self, f, ticks)
    }

    fn build_wall(&mut self, a: (f32, f32), b: (f32, f32), width: f32) {
        BigDish::build_wall(self, a.0, a.1, b.0, b.1, width);
    }

    fn living(&self) -> usize {
        let c = self.life_counts();
        c.iter().sum::<usize>() - c[LIFE_DEAD as usize]
    }

    fn crystals(&self) -> Vec<((f32, f32), f32)> {
        let s = self.p.scale as f32;
        self.piles.iter().enumerate().map(|(p, &(x, y))| ((x * s, y * s), self.pile_charge.get(p).copied().unwrap_or(0.0))).collect()
    }

    fn crystal_r(&self, c: usize) -> f32 {
        self.pile_rs[c] * self.p.scale as f32
    }

    fn landing(&self) -> Option<(f32, f32)> {
        let s = self.p.scale as f32;
        self.landing.map(|(x, y)| (x * s, y * s))
    }

    fn open(&self, x: f32, y: f32) -> bool {
        x >= 0.0 && y >= 0.0 && (x as usize) < self.w && (y as usize) < self.h && self.agar[y as usize * self.w + x as usize]
    }

    fn stop_births(&mut self) {
        self.p.birth_energy = f32::INFINITY;
    }

    fn food_deposit(&self) -> f32 {
        self.p.food_deposit
    }

    fn hash(&self) -> u64 {
        BigDish::hash(self)
    }

    fn calm(&self, at: &[(f32, f32)], r: f32) -> Vec<u32> {
        self.gassed_near(at, r)
    }

    fn mould_grid(&self, bin: usize) -> Option<Vec<u32>> {
        let (gw, gh) = (self.w.div_ceil(bin), self.h.div_ceil(bin));
        let mut out = vec![0u32; gw * gh];
        for (i, &n) in self.occupancy().iter().enumerate() {
            if n > 0 {
                out[(i / self.w / bin) * gw + (i % self.w) / bin] += n as u32;
            }
        }
        Some(out)
    }
}

/// **The race world** on the CPU (owner, 14:54Z: the mould starts in one
/// spot and must expand, with food for a long game; an idle player should
/// lose to a strengthening mould): the W1b world's rect, 23 crystals that
/// run dry at charge 512 a cell, rock and wet ground, and W0's growth from a
/// blob of `walkers` on the home crystal, with room for ten times as many.
/// `scale` 2 is the 512 x 320 world, 1 a 256 x 160 one for tests.
pub fn race_world(seed: u64, scale: usize, walkers: usize) -> BigDish {
    BigDish::new(race_params(seed, scale, walkers))
}

/// The mould's turn from fresh gunfire, an opt-in (`flee=W` in the census
/// and the hole test), not the race world's default. With the alarm field on
/// (`alarm_keep` 0.98, the Swarmer's) and read at weight -1, the alarm each
/// hit lays (`squad::SHOT_ALARM`, out to twice the strike's radius) is
/// ground the mould backs away from while it is fresh. It answers the
/// owner's 04:39Z bar ("A group of soldiers shooting the slime should start
/// carving out a bigger and bigger hole that doesn't refill while being
/// shot at") in the hole test (`examples/larderhole.rs`, 16 seeds): 102
/// dish cells after 20 s of four soldiers' fire against 29 without it. It
/// was the default for a few hours on 2026-11-12 and failed the census gate
/// (race seeds 5-20, free shots): random wiped the mould on 12 of 16 seeds
/// (2 without it) and beat farm, wall and rush 13-3, because random's
/// Attack orders hold the core once the mould no longer overruns soldiers
/// there, and a mould without births starves; the first touch on a squad
/// came later too (farm +36 s median). `Reports/dead-ends.md` has the
/// condition: a flee that holds the core (or the mould's own crystals)
/// could pass.
pub const FLEE_KEEP: f32 = 0.98;
pub const FLEE_WEIGHT: f32 = -1.0;

pub fn race_params(seed: u64, scale: usize, walkers: usize) -> BigParams {
    BigParams {
        scale,
        substeps: scale,
        walkers,
        capacity: walkers * 10,
        scent_offset: 18.0 * scale as f32,
        scent_coarse: false,
        piles: 23,
        rect: true,
        crystal_charge: 512.0,
        crystal_eat: 0.005,
        terrain_rock: 0.2,
        terrain_wet: 0.2,
        upkeep: 0.001,
        seed,
        ..BigParams::today()
    }
}

/// Every number in the game layer: M3's (`larder::Rules`) where the rule
/// carried over, so a census can be read against M3's.
#[derive(Clone, Copy, Debug)]
pub struct Rules {
    /// Ticks a game second (M3: 15, an eighth of the dish's 120).
    pub tps: u32,
    /// Ticks a squad frame: the squad reads and acts this often.
    pub frame_ticks: u32,
    /// Dish cells a game second.
    pub speed: f32,
    /// How far a soldier sees and fires, dish cells.
    pub range: f32,
    /// Radius of a shot's strike, dish cells (at most `squad::R_MAX` fine).
    pub strike: f32,
    /// Game seconds between one soldier's shots.
    pub shot_every: f32,
    /// Food a shot costs. 0 since 2026-11-12: the owner's playtest (04:13Z)
    /// ran the bank dry with soldiers left, who could then neither fight
    /// nor harvest past the mould ("energy required for shooting isn't
    /// fun"); a soldier's fire rate (`shot_every`) is the limit. Was 10
    /// (M3's), against 30 food a second a harvester takes.
    pub price: f32,
    pub hp: f32,
    /// Health a walker touching a soldier takes a game second.
    pub drain: f32,
    /// Walkers within this many dish cells touch a soldier.
    pub touch: f32,
    /// A soldier's scent, as a share of a full pile's, laid on the agar
    /// within `smell_r` dish cells of it.
    pub smell: f32,
    pub smell_r: f32,
    /// Food a soldier takes from a crystal a game second, how much it can
    /// carry, and the charge one food is.
    pub harvest: f32,
    pub carry: f32,
    pub charge: f32,
    /// Defend fires on mould this near, dish cells.
    pub defend: f32,
    /// Walkers the aimed 4 x 4 block must hold before a soldier fires at
    /// it. 1 (any): on rush seed 2 over 200 s, 1 wasted 1 shot of 200 and
    /// killed 1,600 walkers, 2 killed 1,486 and 4 killed 1,452, once the
    /// sight check stopped shots into rock (before it, 104 of 200 shots
    /// hit nothing at 1 and 2 of 199 at 4).
    pub aim_min: u32,
    pub core_r: f32,
    pub core_secs: f32,
    /// The cap, game seconds (owner, 04:04Z: 15 minutes).
    pub cap: f32,
    pub start_bank: f32,
    pub squad: usize,
    pub soldier_cost: f32,
    pub max_squad: usize,
    /// Food a dish cell of wall costs, and its width in dish cells.
    pub wall_cost: f32,
    pub wall_width: f32,
    /// The larder smells of the bank, `hoard` times a full pile per 10,000
    /// food (M2), and walkers touching it take `steal` food a game second.
    pub hoard: f32,
    pub steal: f32,
    /// A carrier whose crystal has run dry (the frame gave under half its
    /// ask and the whole crystal holds less than one frame's ask) walks
    /// home with what it has, banks it and holds there (`Soldier::dry`). On:
    /// under score v2 (`Game::score`) farm and wall are even with it off
    /// (2026-11-12). Under W2b's score it cost farm 10 of 10 (WM1), because
    /// that score counted a crystal with crumbs left as held.
    pub home_dry: bool,
    /// **Larder overrun** (campaign thread, the owner's pick 2026-11-12
    /// 04:44Z): the round is lost (`End::Overrun`) once the mould has held
    /// the larder (a walker within its 3 dish cells) with the bank under 1
    /// food for this many game seconds straight. 0, the default, is off; a
    /// campaign mission turns it on.
    pub overrun: f32,
    /// **Reserve recruit** (the same pick): while fewer than 2 soldiers live
    /// and the bank can't buy one, the larder makes a free soldier after
    /// each this many game seconds of it, unless the mould holds the larder.
    /// With it on, a wiped squad is no loss while a recruit can still come.
    /// 0, the default, is off.
    pub reserve: f32,
    /// A carrier back from a dry crystal with nothing queued walks on to
    /// the nearest crystal with charge left (`Game::seek`), the mould's
    /// home excepted, rather than holding at the larder (owner, 11-12
    /// 04:22Z: better orders for harvesting). Only to a safe crystal
    /// (`Game::seek`). On, for the window and the census alike (one game):
    /// on seeds 5-20 the gate holds, farm 10 higher and 5 lower than with
    /// it off (p 0.30), wall 4 and 11 (p 0.12). The first version (nearest
    /// to the soldier, any crystal) cost wall 14 of 16 (p 0.004) and was
    /// replaced (`Reports/dead-ends.md`).
    pub seek: bool,
}

impl Default for Rules {
    fn default() -> Self {
        Rules {
            tps: 15,
            frame_ticks: 1,
            speed: 10.0,
            range: 15.0,
            strike: 2.5,
            shot_every: 1.0,
            price: 0.0,
            hp: 100.0,
            drain: 1.0,
            touch: 1.5,
            smell: 0.5,
            smell_r: 2.0,
            harvest: 30.0,
            carry: 300.0,
            charge: 64.0,
            defend: 7.5,
            aim_min: 1,
            core_r: 5.0,
            core_secs: 5.0,
            cap: 900.0,
            start_bank: 2000.0,
            squad: 4,
            soldier_cost: 400.0,
            max_squad: 40,
            wall_cost: 20.0,
            wall_width: 2.0,
            hoard: 1.0,
            steal: 1.0,
            home_dry: true,
            overrun: 0.0,
            reserve: 0.0,
            seek: true,
        }
    }
}

pub use super::larder::{End, Order, Stance};

/// Game seconds between rebuilds of the walking maps.
const PATHS_EVERY: f32 = 30.0;

/// Walking maps: for each goal cell, every open cell's distance to it
/// (`distance_map`), built when a soldier first needs it and dropped when a
/// wall goes up or `PATHS_EVERY` passes.
#[derive(Default)]
struct Paths {
    maps: std::collections::HashMap<usize, Vec<u32>>,
    at: u64,
}

fn cell(w: usize, h: usize, p: (f32, f32)) -> usize {
    let x = (p.0.max(0.0) as usize).min(w - 1);
    let y = (p.1.max(0.0) as usize).min(h - 1);
    y * w + x
}

/// Distances to `goal` over open cells, 5 a side step and 7 a diagonal
/// (a diagonal only when both side cells are open, so no corner is cut);
/// `u32::MAX` where the goal cannot be reached.
pub fn distance_map(w: usize, h: usize, goal: usize, open: impl Fn(usize) -> bool) -> Vec<u32> {
    use std::cmp::Reverse;
    use std::collections::BinaryHeap;
    let ok: Vec<bool> = (0..w * h).map(&open).collect();
    let mut dist = vec![u32::MAX; w * h];
    let mut heap = BinaryHeap::new();
    dist[goal] = 0;
    heap.push(Reverse((0u32, goal)));
    while let Some(Reverse((d, i))) = heap.pop() {
        if d > dist[i] {
            continue;
        }
        let (x, y) = ((i % w) as i64, (i / w) as i64);
        for (dx, dy) in [(-1i64, 0i64), (1, 0), (0, -1), (0, 1), (-1, -1), (1, -1), (-1, 1), (1, 1)] {
            let (nx, ny) = (x + dx, y + dy);
            if nx < 0 || ny < 0 || nx >= w as i64 || ny >= h as i64 {
                continue;
            }
            let j = ny as usize * w + nx as usize;
            if !ok[j] {
                continue;
            }
            let diag = dx != 0 && dy != 0;
            if diag && (!ok[y as usize * w + nx as usize] || !ok[ny as usize * w + x as usize]) {
                continue;
            }
            let nd = d + if diag { 7 } else { 5 };
            if nd < dist[j] {
                dist[j] = nd;
                heap.push(Reverse((nd, j)));
            }
        }
    }
    dist
}

/// The neighbour of `i` nearest the goal on `map` (`i` itself at the goal,
/// or when no neighbour is nearer).
fn downhill(w: usize, h: usize, map: &[u32], i: usize) -> usize {
    let (x, y) = ((i % w) as i64, (i / w) as i64);
    let mut best = i;
    for (dx, dy) in [(-1i64, 0i64), (1, 0), (0, -1), (0, 1), (-1, -1), (1, -1), (-1, 1), (1, 1)] {
        let (nx, ny) = (x + dx, y + dy);
        if nx < 0 || ny < 0 || nx >= w as i64 || ny >= h as i64 {
            continue;
        }
        let j = ny as usize * w + nx as usize;
        if map[j] < map[best] {
            best = j;
        }
    }
    best
}

/// A player's command; `Game::apply` keeps the log. As M3's, with a wall in
/// place of the turret and the bait. `Order::Clear` names a crystal by a
/// point inside it.
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum Cmd {
    Order(usize, Order),
    Stance(usize, Stance),
    Buy,
    /// A wall between two points, fine cells.
    Wall((f32, f32), (f32, f32)),
    /// Shift + an order: soldier `k` does it after the ones it has
    /// (`Soldier::queue`); a soldier holding starts it at once.
    Queue(usize, Order),
    /// The larder's rally point (`Game::rally`): what a bought soldier is
    /// first ordered to do; `Order::Hold` clears it.
    Rally(Order),
}

impl Cmd {
    pub fn to_line(self) -> String {
        match self {
            Cmd::Order(k, o) => super::larder::Cmd::Order(k, o).to_line(),
            Cmd::Stance(k, s) => super::larder::Cmd::Stance(k, s).to_line(),
            Cmd::Buy => "buy".into(),
            Cmd::Wall(a, b) => format!("wall {} {} {} {}", a.0, a.1, b.0, b.1),
            Cmd::Queue(k, o) => format!("queue{}", &super::larder::Cmd::Order(k, o).to_line()["order".len()..]),
            Cmd::Rally(o) => format!("rally{}", &super::larder::Cmd::Order(0, o).to_line()["order 0".len()..]),
        }
    }

    pub fn parse(line: &str) -> Option<Cmd> {
        let w: Vec<&str> = line.split_whitespace().collect();
        if *w.first()? == "wall" {
            let f = |i: usize| w.get(i).and_then(|v| v.parse::<f32>().ok());
            return Some(Cmd::Wall((f(1)?, f(2)?), (f(3)?, f(4)?)));
        }
        if let Some(rest) = line.trim_start().strip_prefix("rally ") {
            return match super::larder::Cmd::parse(&format!("order 0 {rest}"))? {
                super::larder::Cmd::Order(_, o) => Some(Cmd::Rally(o)),
                _ => None,
            };
        }
        if let Some(rest) = line.trim_start().strip_prefix("queue ") {
            return match super::larder::Cmd::parse(&format!("order {rest}"))? {
                super::larder::Cmd::Order(k, o) => Some(Cmd::Queue(k, o)),
                _ => None,
            };
        }
        match super::larder::Cmd::parse(line)? {
            super::larder::Cmd::Order(k, o) => Some(Cmd::Order(k, o)),
            super::larder::Cmd::Stance(k, s) => Some(Cmd::Stance(k, s)),
            super::larder::Cmd::Buy => Some(Cmd::Buy),
            _ => None,
        }
    }
}

/// Orders a soldier can hold in its queue after the one it is doing.
pub const QUEUE: usize = 8;

#[derive(Clone, Copy, Debug)]
pub struct Soldier {
    /// Fine cells.
    pub x: f32,
    pub y: f32,
    pub hp: f32,
    pub order: Order,
    pub stance: Stance,
    pub carry: f32,
    pub homing: bool,
    /// Walking home from a crystal that ran dry (`Rules::home_dry`): once
    /// it banks, it holds at the larder instead of walking back for crumbs.
    pub dry: bool,
    /// Orders to do after this one (Shift, `Cmd::Queue`), first
    /// `queued` of them; fixed size so a soldier stays `Copy`.
    pub queue: [Order; QUEUE],
    pub queued: u8,
    pub alive: bool,
    /// Tick of its next shot.
    ready: u64,
    /// Which point of a burn line it fires at next.
    beat: u32,
    /// What it read last frame: walkers touching, its densest mould in sight.
    pub touching: u32,
    pub aim: Option<(f32, f32)>,
    /// Walkers in the block it aims at.
    pub dense: u32,
    /// Its smell, x `Rules::smell` (1). The squad toolkit's mask (0: the
    /// mould can't smell it, to sneak to a crystal) and reek (over 1: a
    /// decoy that pulls the mould off the harvesters), `mould::kit`.
    pub smell: f32,
}

/// One shot as the window draws it: from, where it stopped (None: a wall or
/// rock stopped it, or it flew its range), and walkers killed.
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct Shot {
    pub from: (f32, f32),
    pub to: (f32, f32),
    pub hit: Option<(f32, f32)>,
    pub killed: u32,
}

pub struct Game<W: World> {
    pub world: W,
    pub rules: Rules,
    pub soldiers: Vec<Soldier>,
    pub bank: f32,
    pub spent: f32,
    pub harvested: f32,
    pub stolen: f32,
    pub shots: u32,
    pub killed: u32,
    pub walls: u32,
    /// The squad's base, fine cells: where bought soldiers stand and carried
    /// food goes.
    pub larder: (f32, f32),
    /// The mould's home crystal, fine cells.
    pub core: (f32, f32),
    pub core_down: Option<u64>,
    core_clear: u64,
    pub first_touch: Option<f32>,
    /// Walkers touching the larder at the last frame (it is held while any
    /// do).
    pub larder_touch: u32,
    /// **Stuck seconds** (design review, 2026-11-12: no soft lock), counted
    /// once a game second: a soldier alive, no shot fired and no food taken
    /// (harvested plus carried did not rise) since the last count, and the
    /// bank below a soldier's price. `stuck` is the round's total, `stuck_run`
    /// the seconds in a row now (0 once a second isn't stuck). The census
    /// (`examples/larderworld.rs`) counted this itself until 2026-11-12; the
    /// campaign's mission layer reads it here.
    pub stuck: u32,
    pub stuck_run: u32,
    stuck_mark: (u32, f32),
    /// Ticks the larder has been held with the bank empty (`Rules::overrun`),
    /// and ticks toward the next reserve recruit (`Rules::reserve`).
    held_ticks: u64,
    reserve_ticks: u64,
    pub end: Option<End>,
    /// The last frame's shots, for the window's tracers.
    pub last_shots: Vec<Shot>,
    /// Walkers alive at the last count (once a game second).
    pub mould: usize,
    /// Each crystal's charge at the start and the charge the squad took
    /// from it, so the cap's share can tell the mould's eating from ours.
    start_charge: Vec<f32>,
    taken: Vec<f32>,
    /// Walkers at the start, for the stalemate's size term (`Game::score`).
    pub start_mould: usize,
    /// Walkers per open fine cell at the start (`start_mould` over the cells
    /// `World::open` holds). The unit for any threshold written as a walker
    /// count, so it keeps its meaning from the CPU's 5,000 walkers to the
    /// GPU's 100,000 (integration thread, 2026-11-12, from the technical
    /// lead's review: counts like 6 or 80 walkers broke at GPU scale).
    pub density: f32,
    /// W2b's held share at the cap (`Game::share`), kept so a census can
    /// print W2b's score beside v2's.
    pub cap_held: Option<f32>,
    /// Walking round rock and walls (`Paths`).
    paths: Paths,
    pub rng: Rng,
    pub log: Vec<(u64, Cmd)>,
    /// The larder's rally point: the order a bought soldier starts with
    /// (`Cmd::Rally`; the window's right click with the larder selected).
    pub rally: Order,
    /// The step handed to the world and not yet finished (`begin_step`).
    flight: Option<Flight>,
}

/// A step between `Game::begin_step` and `Game::finish_step`: the tick it
/// began on, its frame, which soldier (None the larder) and crystal each
/// unit stands for, and the results if the world gave them at once.
struct Flight {
    t: u64,
    f: Frame,
    who: Vec<Option<usize>>,
    from: Vec<Option<usize>>,
    out: Option<FrameOut>,
}

/// The squad's base on a world: the crystal farthest from the home one
/// (crystal 0), so the race starts at opposite ends.
pub fn far_crystal(world: &impl World) -> (f32, f32) {
    let cs = world.crystals();
    let home = cs[0].0;
    cs.iter().map(|c| c.0).max_by(|a, b| (a.0 - home.0).hypot(a.1 - home.1).total_cmp(&(b.0 - home.0).hypot(b.1 - home.1))).unwrap_or(home)
}

/// Soldier `k`'s place round the point it was sent to, dish cells (M3's
/// sunflower: a squad sent to one point spreads instead of stacking).
fn slot(k: usize) -> (f32, f32) {
    if k == 0 {
        return (0.0, 0.0);
    }
    let r = 1.5 * (k as f32).sqrt();
    let a = k as f32 * 2.399_963;
    (r * a.cos(), r * a.sin())
}

impl<W: World> Game<W> {
    /// The squad at the crystal farthest from the mould's home, the mould on
    /// its home crystal (crystal 0).
    pub fn new(world: W, rules: Rules, seed: u64) -> Self {
        let larder = world.landing().unwrap_or_else(|| far_crystal(&world));
        let core = world.crystals()[0].0;
        let mould = world.living();
        let (w, h) = world.size();
        let open = (0..w * h).filter(|&i| world.open((i % w) as f32 + 0.5, (i / w) as f32 + 0.5)).count();
        let density = mould as f32 / open.max(1) as f32;
        let start_charge: Vec<f32> = world.crystals().iter().map(|c| c.1).collect();
        let taken = vec![0.0; start_charge.len()];
        let mut g = Game {
            world,
            rules,
            soldiers: Vec::new(),
            bank: rules.start_bank,
            spent: 0.0,
            harvested: 0.0,
            stolen: 0.0,
            shots: 0,
            killed: 0,
            walls: 0,
            larder,
            core,
            core_down: None,
            core_clear: 0,
            first_touch: None,
            larder_touch: 0,
            stuck: 0,
            stuck_run: 0,
            stuck_mark: (0, 0.0),
            held_ticks: 0,
            reserve_ticks: 0,
            end: None,
            last_shots: Vec::new(),
            mould,
            start_charge,
            taken,
            start_mould: mould,
            density,
            cap_held: None,
            paths: Paths::default(),
            rng: Rng::new(seed ^ 0x001a_2de2_501d),
            log: Vec::new(),
            flight: None,
            rally: Order::Hold,
        };
        for k in 0..rules.squad {
            let a = k as f32 / rules.squad.max(1) as f32 * std::f32::consts::TAU;
            g.add_soldier(a);
        }
        g
    }

    fn s(&self) -> f32 {
        self.world.scale()
    }

    fn add_soldier(&mut self, a: f32) {
        let r = 3.0 * self.s();
        let (x, y) = (self.larder.0 + r * a.cos(), self.larder.1 + r * a.sin());
        let (x, y) = if self.world.open(x, y) { (x, y) } else { self.larder };
        self.soldiers.push(Soldier {
            x,
            y,
            hp: self.rules.hp,
            order: Order::Hold,
            stance: Stance::Defend,
            carry: 0.0,
            homing: false,
            dry: false,
            queue: [Order::Hold; QUEUE],
            queued: 0,
            alive: true,
            ready: 0,
            beat: 0,
            touching: 0,
            aim: None,
            dense: 0,
            smell: 1.0,
        });
    }

    /// Game seconds since the start.
    pub fn time(&self) -> f32 {
        self.world.ticks() as f32 / self.rules.tps as f32
    }

    fn secs(&self, s: f32) -> u64 {
        (s * self.rules.tps as f32).round() as u64
    }

    pub fn alive(&self) -> usize {
        self.soldiers.iter().filter(|s| s.alive).count()
    }

    /// Carry out a command and log it at this tick. True if it took effect.
    /// A step still running finishes first (`finish_step`), so a command
    /// lands between whole steps, where a replay puts it.
    pub fn apply(&mut self, c: Cmd) -> bool {
        self.finish_step();
        self.log.push((self.world.ticks(), c));
        match c {
            Cmd::Order(k, o) if k < self.soldiers.len() => {
                self.soldiers[k].order = o;
                self.soldiers[k].homing = false;
                self.soldiers[k].queued = 0;
                true
            }
            Cmd::Queue(k, o) if k < self.soldiers.len() => {
                let s = &mut self.soldiers[k];
                if s.order == Order::Hold && s.queued == 0 {
                    s.order = o;
                    s.homing = false;
                    true
                } else if (s.queued as usize) < QUEUE {
                    s.queue[s.queued as usize] = o;
                    s.queued += 1;
                    true
                } else {
                    false
                }
            }
            Cmd::Stance(k, st) if k < self.soldiers.len() => {
                self.soldiers[k].stance = st;
                true
            }
            Cmd::Buy => self.buy(),
            Cmd::Wall(a, b) => self.wall(a, b),
            Cmd::Rally(o) => {
                self.rally = o;
                true
            }
            _ => false,
        }
    }

    pub fn buy(&mut self) -> bool {
        if self.bank < self.rules.soldier_cost || self.alive() >= self.rules.max_squad {
            return false;
        }
        self.bank -= self.rules.soldier_cost;
        self.spent += self.rules.soldier_cost;
        let a = self.rng.f32() * std::f32::consts::TAU;
        self.add_soldier(a);
        if let Some(s) = self.soldiers.last_mut() {
            s.order = self.rally;
        }
        true
    }

    /// Within range of a live soldier.
    pub fn reachable(&self, at: (f32, f32)) -> bool {
        let r = self.rules.range * self.s();
        self.soldiers.iter().any(|s| s.alive && (s.x - at.0).hypot(s.y - at.1) <= r)
    }

    /// The price of a wall from `a` to `b` (fine cells).
    pub fn wall_price(&self, a: (f32, f32), b: (f32, f32)) -> f32 {
        ((a.0 - b.0).hypot(a.1 - b.1) / self.s()).max(1.0) * self.rules.wall_cost
    }

    /// A wall, if both ends are within a soldier's range and it is paid for.
    pub fn wall(&mut self, a: (f32, f32), b: (f32, f32)) -> bool {
        let price = self.wall_price(a, b);
        if self.bank < price || !self.reachable(a) || !self.reachable(b) {
            return false;
        }
        self.bank -= price;
        self.spent += price;
        self.walls += 1;
        let s = self.s();
        self.world.build_wall((a.0 / s, a.1 / s), (b.0 / s, b.1 / s), self.rules.wall_width);
        self.paths.maps.clear();
        true
    }

    /// The crystal a point is inside (or nearest, within its radius plus 2
    /// dish cells): its index and centre.
    pub fn crystal_at(&self, p: (f32, f32)) -> Option<(usize, (f32, f32))> {
        let sc = self.s();
        self.world.crystals().iter().enumerate().map(|(i, c)| (i, c.0)).filter(|&(i, c)| (c.0 - p.0).hypot(c.1 - p.1) <= self.world.crystal_r(i) + 2.0 * sc).min_by(|a, b| {
            let d = |c: (f32, f32)| (c.0 - p.0).hypot(c.1 - p.1);
            d(a.1).total_cmp(&d(b.1))
        })
    }

    /// Where soldier `k` is walking to now, fine cells, if anywhere.
    fn goal(&self, k: usize) -> Option<(f32, f32)> {
        let s = self.soldiers[k];
        let sc = self.s();
        let off = |p: (f32, f32)| {
            let o = slot(k);
            (p.0 + o.0 * sc, p.1 + o.1 * sc)
        };
        match s.order {
            Order::Hold => None,
            Order::Move(x, y) | Order::Fight(x, y) => Some(off((x, y))),
            Order::Clear(x, y) => {
                if s.homing {
                    Some(off(self.larder))
                } else {
                    self.crystal_at((x, y)).map(|(_, c)| off(c))
                }
            }
            Order::Burn(a, b) => {
                // Stand short of the line's middle, on the side it was
                // ordered from, so the whole line is in range.
                let m = ((a.0 + b.0) / 2.0, (a.1 + b.1) / 2.0);
                let (dx, dy) = (s.x - m.0, s.y - m.1);
                let d = dx.hypot(dy);
                let keep = 0.6 * self.rules.range * sc;
                if d <= keep {
                    None
                } else {
                    Some((m.0 + dx / d * keep, m.1 + dy / d * keep))
                }
            }
            Order::Attack => Some(off(self.core)),
        }
    }

    /// Whether every cell on the line from `a` to `b` is open ground.
    pub fn clear_line(&self, a: (f32, f32), b: (f32, f32)) -> bool {
        let (w, h) = self.world.size();
        squad::path(w, h, a, b).into_iter().all(|i| self.world.open((i as usize % w) as f32 + 0.5, (i as usize / w) as f32 + 0.5))
    }

    /// Walk soldier `k` toward its goal for `dt` game seconds: straight
    /// when the line is open, else down the goal's distance map (`Paths`),
    /// and along one axis when even that step is blocked. (The first
    /// version only slid along an axis: on seed 1 three of four rushing
    /// soldiers stood 50 cells short of the core behind rock for the
    /// whole game, firing 196 of 200 shots into the rock.)
    fn walk(&mut self, k: usize, dt: f32) {
        let Some(g) = self.goal(k) else { return };
        let s = self.soldiers[k];
        let d = (g.0 - s.x).hypot(g.1 - s.y);
        if d < 0.5 || self.engaged(k) {
            return;
        }
        let g = if self.clear_line((s.x, s.y), g) {
            g
        } else {
            let now = self.world.ticks();
            if now >= self.paths.at + self.secs(PATHS_EVERY) {
                // Walls wear to rubble: rebuild the maps now and then.
                self.paths.maps.clear();
                self.paths.at = now;
            }
            let (w, h) = self.world.size();
            let gi = cell(w, h, g);
            if !self.paths.maps.contains_key(&gi) {
                let world = &self.world;
                let map = distance_map(w, h, gi, |i| world.open((i % w) as f32 + 0.5, (i / w) as f32 + 0.5));
                self.paths.maps.insert(gi, map);
            }
            let map = &self.paths.maps[&gi];
            // A few cells down the map, so the step heads along the path
            // rather than zig-zagging cell to cell.
            let mut at = cell(w, h, (s.x, s.y));
            for _ in 0..3 {
                at = downhill(w, h, map, at);
            }
            if at == gi {
                g
            } else {
                ((at % w) as f32 + 0.5, (at / w) as f32 + 0.5)
            }
        };
        let (dx, dy) = (g.0 - s.x, g.1 - s.y);
        let d = dx.hypot(dy).max(1e-6);
        let step = (self.rules.speed * self.s() * dt).min(d);
        let (nx, ny) = (s.x + dx / d * step, s.y + dy / d * step);
        let to = if self.world.open(nx, ny) {
            Some((nx, ny))
        } else if self.world.open(nx, s.y) {
            Some((nx, s.y))
        } else if self.world.open(s.x, ny) {
            Some((s.x, ny))
        } else {
            None
        };
        if let Some((x, y)) = to {
            self.soldiers[k].x = x;
            self.soldiers[k].y = y;
        }
    }

    /// An attack-moving soldier (`Order::Fight`) with mould it can shoot in
    /// sight: it stands and fights instead of walking on.
    fn engaged(&self, k: usize) -> bool {
        let s = self.soldiers[k];
        let Order::Fight(..) = s.order else { return false };
        let Some(aim) = s.aim else { return false };
        s.dense >= self.rules.aim_min && (aim.0 - s.x).hypot(aim.1 - s.y) <= self.rules.range * self.s() && self.clear_line((s.x, s.y), aim)
    }

    /// Soldier `k` takes the next order in its queue, if it has one.
    fn next(&mut self, k: usize) -> bool {
        let s = &mut self.soldiers[k];
        if s.queued == 0 {
            return false;
        }
        s.order = s.queue[0];
        s.queue.copy_within(1.., 0);
        s.queued -= 1;
        s.homing = false;
        true
    }

    /// The crystal a harvester from `skip` goes on to (`Rules::seek`), its
    /// centre, fine cells; None sends it home to hold. Only a safe one: at
    /// least a frame's harvest left, never the mould's home (crystal 0),
    /// none the mould has eaten from (as `share` counts it), and none
    /// nearer the core than the larder (the squad's side of the map). Of
    /// those, the nearest the larder that no other soldier is harvesting,
    /// else the nearest the larder. (The first version took the nearest to
    /// the soldier and only preferred uneaten crystals: the wall hand
    /// scored lower with it on 14 of 16 seeds, its farmers walking out past
    /// its wall.)
    pub fn seek(&self, k: usize, skip: Option<usize>) -> Option<(f32, f32)> {
        let r = self.rules;
        let ask = r.harvest * r.frame_ticks as f32 / r.tps as f32 * r.charge;
        let (l, core) = (self.larder, self.core);
        let d = |a: (f32, f32), b: (f32, f32)| (a.0 - b.0).hypot(a.1 - b.1);
        let busy: Vec<usize> = self
            .soldiers
            .iter()
            .enumerate()
            .filter(|&(q, s)| q != k && s.alive)
            .filter_map(|(_, s)| if let Order::Clear(x, y) = s.order { self.crystal_at((x, y)).map(|c| c.0) } else { None })
            .collect();
        self.world
            .crystals()
            .into_iter()
            .enumerate()
            .filter(|&(i, (c, charge))| i != 0 && Some(i) != skip && charge >= ask && d(c, l) < d(c, core))
            .filter(|&(i, (_, charge))| self.start_charge[i] - charge - self.taken[i] <= 0.01 * self.start_charge[i])
            .min_by(|a, b| busy.contains(&a.0).cmp(&busy.contains(&b.0)).then(d(a.1 .0, l).total_cmp(&d(b.1 .0, l))))
            .map(|(_, (c, _))| c)
    }

    /// Soldier `k` is done with a crystal that ran dry (`skip`): its next
    /// queued order, else (`Rules::seek`) the next crystal, else hold.
    fn after_dry(&mut self, k: usize, skip: Option<usize>) {
        if self.next(k) {
            return;
        }
        let o = match self.rules.seek.then(|| self.seek(k, skip)).flatten() {
            Some(c) => Order::Clear(c.0, c.1),
            None => Order::Hold,
        };
        self.soldiers[k].order = o;
    }

    /// Where soldier `k` fires this frame, if anywhere (fine cells).
    fn target(&mut self, k: usize) -> Option<(f32, f32)> {
        let s = self.soldiers[k];
        let sc = self.s();
        if let Order::Burn(a, b) = s.order {
            // Along the line in turn, five points.
            let t = (s.beat % 5) as f32 / 4.0;
            self.soldiers[k].beat += 1;
            return Some((a.0 + (b.0 - a.0) * t, a.1 + (b.1 - a.1) * t));
        }
        let aim = s.aim?;
        // Only at mould that shows as slime, and that the soldier can see
        // (no rock or wall on the line: a shot into rock is paid for and
        // hits nothing).
        if s.dense < self.rules.aim_min || !self.clear_line((s.x, s.y), aim) {
            return None;
        }
        let near = |p: (f32, f32), r: f32| (p.0 - aim.0).hypot(p.1 - aim.1) <= r;
        let stance = if matches!(s.order, Order::Attack | Order::Fight(..)) { Stance::Free } else { s.stance };
        match stance {
            Stance::HoldFire => None,
            Stance::Free => Some(aim),
            Stance::Defend => {
                let mut ok = s.touching > 0 || near((s.x, s.y), self.rules.defend * sc);
                if let Order::Clear(x, y) = s.order {
                    if let Some((i, c)) = self.crystal_at((x, y)) {
                        ok |= near(c, self.world.crystal_r(i) + 2.0 * sc);
                    }
                }
                ok.then_some(aim)
            }
        }
    }

    /// One squad frame (`Rules::frame_ticks` ticks) with the squad's turn
    /// around it. Does nothing once the round has ended. It is
    /// `begin_step` then `finish_step` (2026-11-12): the window calls the
    /// two apart, so the GPU runs the frame while the window draws.
    pub fn step(&mut self) {
        self.begin_step();
        self.finish_step();
    }

    /// Whether a step has begun and not finished.
    pub fn in_flight(&self) -> bool {
        self.flight.is_some()
    }

    /// The world's ticks as of the last finished step: while a step runs,
    /// the tick it began on. A reader keyed on game seconds (the
    /// campaign's tracker in the window) keys on this, so it reads whole
    /// steps, as the census does.
    pub fn settled_ticks(&self) -> u64 {
        self.flight.as_ref().map_or_else(|| self.world.ticks(), |f| f.t)
    }

    /// Whether `finish_step` would return without waiting on the world.
    pub fn step_ready(&self) -> bool {
        self.flight.as_ref().is_none_or(|f| f.out.is_some() || self.world.frame_ready())
    }

    /// The first half of `step`: the squad's turn (walking, banking, aims
    /// and shots) and the frame handed to the world, which may run it
    /// without waiting (`World::frame_start`). A step still running
    /// finishes first. Until `finish_step` the soldiers, bank and counts
    /// are the last step's, bar what this half changed.
    pub fn begin_step(&mut self) {
        self.finish_step();
        if self.end.is_some() {
            return;
        }
        let r = self.rules;
        let sc = self.s();
        let t = self.world.ticks();
        let dt = r.frame_ticks as f32 / r.tps as f32;
        let strike = (r.strike * sc).min(squad::R_MAX);
        let mut units = Vec::new();
        let mut who = Vec::new();
        let mut from = Vec::new();
        let mut shots = Vec::new();
        for k in 0..self.soldiers.len() {
            if !self.soldiers[k].alive {
                continue;
            }
            self.walk(k, dt);
            // A move that has arrived starts the next queued order (with
            // none queued it stays, as before: the window shows where).
            if self.soldiers[k].queued > 0 {
                let arrived = match self.soldiers[k].order {
                    Order::Hold => true,
                    Order::Move(..) | Order::Fight(..) => self.goal(k).is_none_or(|g| (g.0 - self.soldiers[k].x).hypot(g.1 - self.soldiers[k].y) <= 1.5 * sc),
                    _ => false,
                };
                if arrived {
                    self.next(k);
                }
            }
            let s = self.soldiers[k];
            // Harvest: at its crystal and not full, a clearing soldier takes;
            // full, it walks home; home, it banks.
            let mut take = 0.0;
            let mut at_crystal = None;
            if let Order::Clear(x, y) = s.order {
                if s.homing {
                    if (s.x - self.larder.0).hypot(s.y - self.larder.1) <= 3.0 * sc + 1.5 * (k as f32).sqrt() * sc {
                        self.bank += s.carry;
                        self.harvested += s.carry;
                        self.soldiers[k].carry = 0.0;
                        self.soldiers[k].homing = false;
                        if self.soldiers[k].dry {
                            self.soldiers[k].dry = false;
                            let skip = self.crystal_at((x, y)).map(|c| c.0);
                            self.after_dry(k, skip);
                        }
                    }
                } else if s.carry >= r.carry {
                    self.soldiers[k].homing = true;
                } else if let Some((i, c)) = self.crystal_at((x, y)) {
                    if (s.x - c.0).hypot(s.y - c.1) <= self.world.crystal_r(i) + 1.0 * sc {
                        take = (r.harvest * dt).min(r.carry - s.carry) * r.charge;
                        at_crystal = Some(i);
                    }
                }
            }
            if s.ready <= t && self.bank >= r.price {
                if let Some(at) = self.target(k) {
                    let (dx, dy) = (at.0 - s.x, at.1 - s.y);
                    let d = dx.hypot(dy).max(0.1);
                    let reach = r.range * sc;
                    if d <= reach + 4.0 {
                        shots.push(ShotIn { from: (s.x, s.y), to: (s.x + dx / d * reach, s.y + dy / d * reach), r: strike });
                        self.soldiers[k].ready = t + self.secs(r.shot_every);
                        self.bank -= r.price;
                        self.spent += r.price;
                    }
                }
            }
            units.push(UnitIn {
                x: s.x,
                y: s.y,
                touch: r.touch * sc,
                sight: r.range * sc,
                take_r: at_crystal.map_or(4.0 * sc, |i| self.world.crystal_r(i).min(4.0 * sc)),
                take,
                smell_r: r.smell_r * sc,
                smell: self.world.food_deposit() * r.smell * s.smell * r.frame_ticks as f32,
            });
            who.push(Some(k));
            from.push(at_crystal);
        }
        // The player's own mould's gardeners (`own_mould`; nothing here
        // unless the world has one).
        let tended = self.world.tend(&self.soldiers, self.bank, r.frame_ticks);
        self.bank -= tended;
        self.spent += tended;
        // The larder: smells of the bank, and reads who touches it.
        units.push(UnitIn {
            x: self.larder.0,
            y: self.larder.1,
            touch: 3.0 * sc,
            sight: 0.0,
            take_r: 0.0,
            take: 0.0,
            smell_r: 3.0 * sc,
            smell: self.world.food_deposit() * r.hoard * self.bank.max(0.0) / 10_000.0 * r.frame_ticks as f32,
        });
        who.push(None);
        from.push(None);
        let f = Frame { units, shots, slime: squad::slime_trail(6.0) };
        let out = self.world.frame_start(&f, r.frame_ticks);
        self.flight = Some(Flight { t, f, who, from, out });
    }

    /// The second half of `step`: the frame's results, waited for if the
    /// world has not given them (`World::frame_finish`), and what they do
    /// (kills, harvest, touches, theft, the core, the end). Nothing when no
    /// step is running.
    pub fn finish_step(&mut self) {
        let Some(Flight { t, f, who, from, out }) = self.flight.take() else { return };
        let out = out.unwrap_or_else(|| self.world.frame_finish());
        let r = self.rules;
        let sc = self.s();
        let dt = r.frame_ticks as f32 / r.tps as f32;
        self.shots += f.shots.len() as u32;
        self.last_shots = f.shots.iter().zip(&out.shots).map(|(i, o)| Shot { from: i.from, to: i.to, hit: o.hit, killed: o.killed }).collect();
        self.killed += out.shots.iter().map(|o| o.killed).sum::<u32>();
        // Gassed mould sleeps and does not bite (the squad toolkit's sleep
        // gas): its walkers are taken off each unit's touch.
        let calm = self.world.calm(&f.units.iter().map(|u| (u.x, u.y)).collect::<Vec<_>>(), r.touch * sc);
        let out = FrameOut { units: out.units.iter().zip(&calm).map(|(u, &c)| UnitOut { touching: u.touching.saturating_sub(c), ..*u }).collect(), ..out };
        let left: Vec<f32> = if r.home_dry { self.world.crystals().iter().map(|c| c.1).collect() } else { Vec::new() };
        let mut empty = Vec::new();
        for (((u, k), c), ask) in out.units.iter().zip(&who).zip(&from).zip(&f.units) {
            if let Some(c) = *c {
                self.taken[c] += u.took;
            }
            match *k {
                Some(k) => {
                    let s = &mut self.soldiers[k];
                    s.touching = u.touching;
                    s.aim = u.target;
                    s.dense = u.dense;
                    s.carry += u.took / r.charge;
                    // A crystal run dry sends its carrier home with what it
                    // has (`Rules::home_dry`, on): the frame gave under half
                    // its ask and the whole crystal holds less than one
                    // frame's ask (it never reaches 0 exactly: 0.8 charge
                    // can stay on its cells). Once it banks it holds at the
                    // larder (`Soldier::dry`) rather than walk back for
                    // crumbs. Without it (W2b, M3) a carrier went home only
                    // full: on the v1 Fork (WM1) a soldier emptied a crystal
                    // of 12,032 charge (188 food) and stood there for a 300
                    // load that never came. Four rules were tried under W2b's
                    // score and each cost the planned hands (the narrowest,
                    // this one: farm lower on 10 of 10), because that score
                    // counted a crystal with crumbs left as held while a
                    // soldier stood on it (`Reports/dead-ends.md`). Under
                    // score v2 (`Game::score`, charge taken) it is even: farm
                    // 5 higher and 8 lower of 16, wall 7 and 9. With the hold
                    // and a farm hand that skips dry crystals, the hand
                    // harvests twice the food (22,875 to 46,364 a game).
                    if let (true, Some(i)) = (r.home_dry, *c) {
                        if u.took < 0.5 * ask.take && left[i] < ask.take {
                            if s.carry > 0.0 {
                                s.homing = true;
                                s.dry = true;
                            } else if r.seek || s.queued > 0 {
                                // Empty-handed at a dry crystal (sent to
                                // one, or it ran dry on the first take):
                                // on at once, nothing to bank.
                                empty.push((k, i));
                            }
                        }
                    }
                    if u.touching > 0 {
                        if self.first_touch.is_none() {
                            self.first_touch = Some(self.world.ticks() as f32 / r.tps as f32);
                        }
                        s.hp -= r.drain * u.touching as f32 * dt;
                        if s.hp <= 0.0 {
                            s.alive = false;
                            s.hp = 0.0;
                            s.carry = 0.0;
                        }
                    }
                }
                None => {
                    let take = (r.steal * u.touching as f32 * dt).min(self.bank.max(0.0));
                    self.bank -= take;
                    self.stolen += take;
                    self.larder_touch = u.touching;
                }
            }
        }
        for (k, i) in empty {
            self.after_dry(k, Some(i));
        }
        self.check_core();
        let now = self.world.ticks();
        if now / r.tps as u64 != t / r.tps as u64 {
            self.mould = self.world.living();
            self.count_stuck();
        }
        self.campaign_rules(now - t);
        self.check_end();
    }

    /// Once a game second: `Game::stuck`.
    fn count_stuck(&mut self) {
        let taken = self.harvested + self.soldiers.iter().map(|s| s.carry).sum::<f32>();
        if self.alive() > 0 && self.shots == self.stuck_mark.0 && taken <= self.stuck_mark.1 && self.bank < self.rules.soldier_cost {
            self.stuck += 1;
            self.stuck_run += 1;
        } else {
            self.stuck_run = 0;
        }
        self.stuck_mark = (self.shots, taken);
    }

    /// `Rules::overrun` and `Rules::reserve`, each off at 0.
    fn campaign_rules(&mut self, ticks: u64) {
        let r = self.rules;
        let held = self.larder_touch > 0;
        if r.overrun > 0.0 {
            self.held_ticks = if held && self.bank < 1.0 { self.held_ticks + ticks } else { 0 };
            if self.held_ticks >= self.secs(r.overrun) && self.end.is_none() {
                self.end = Some(End::Overrun(self.time()));
            }
        }
        if r.reserve > 0.0 {
            if self.alive() < 2 && self.bank < r.soldier_cost && !held {
                self.reserve_ticks += ticks;
                if self.reserve_ticks >= self.secs(r.reserve) {
                    self.reserve_ticks = 0;
                    let a = self.soldiers.len() as f32 * 2.399_963;
                    self.add_soldier(a);
                }
            } else {
                self.reserve_ticks = 0;
            }
        }
    }

    fn check_core(&mut self) {
        if self.core_down.is_some() {
            return;
        }
        let sc = self.s();
        let r = self.rules.core_r * sc;
        let on = self.soldiers.iter().any(|s| s.alive && (s.x - self.core.0).hypot(s.y - self.core.1) <= r);
        // No walker near: the touch of the soldiers on it reads 0.
        let clear = on && self.soldiers.iter().filter(|s| s.alive && (s.x - self.core.0).hypot(s.y - self.core.1) <= r).all(|s| s.touching == 0 && s.aim.is_none_or(|a| (a.0 - self.core.0).hypot(a.1 - self.core.1) > 2.0 * r));
        if clear {
            self.core_clear += self.rules.frame_ticks as u64;
            if self.core_clear >= self.secs(self.rules.core_secs) {
                self.core_down = Some(self.world.ticks());
                self.world.stop_births();
            }
        } else {
            self.core_clear = 0;
        }
    }

    fn check_end(&mut self) {
        if self.end.is_some() {
            return;
        }
        let t = self.time();
        // With the reserve on, a wiped squad waits for its recruit unless
        // the mould holds the larder.
        let recruit = self.rules.reserve > 0.0 && self.larder_touch == 0;
        if self.alive() == 0 && self.bank < self.rules.soldier_cost && !recruit {
            self.end = Some(End::SquadWiped(t));
        } else if self.mould == 0 {
            self.end = Some(End::MouldWiped(t));
        } else if t >= self.rules.cap {
            self.cap_held = Some(self.share());
            self.end = Some(End::Cap(self.take_share()));
        }
    }

    /// **Share of the crystals held**, the HUD's and W2b's score's: of the
    /// crystals with charge left that either side holds, the fraction the
    /// squad holds. The squad holds one with a soldier on it and no mould
    /// touching that soldier; the mould holds the others it has eaten from
    /// (charge down by more than 1% of its start beyond what the squad
    /// took) with no soldier on it. One half when neither holds any. (The
    /// first version gave the mould every crystal without a soldier,
    /// reached or not, so an idle squad scored as if overrun from the first
    /// second.)
    pub fn share(&self) -> f32 {
        let (mut ours, mut theirs) = (0, 0);
        for (i, (c, charge)) in self.world.crystals().into_iter().enumerate() {
            if charge <= 0.0 {
                continue;
            }
            let cr = self.world.crystal_r(i);
            let eaten = self.start_charge[i] - charge - self.taken[i] > 0.01 * self.start_charge[i];
            let on: Vec<&Soldier> = self.soldiers.iter().filter(|s| s.alive && (s.x - c.0).hypot(s.y - c.1) <= cr + 2.0 * self.s()).collect();
            if !on.is_empty() && on.iter().all(|s| s.touching == 0) {
                ours += 1;
            } else if on.is_empty() && eaten {
                theirs += 1;
            }
        }
        if ours + theirs == 0 {
            0.5
        } else {
            ours as f32 / (ours + theirs) as f32
        }
    }

    /// **Share of the crystals' charge taken** (score v2's stalemate): of
    /// the charge gone from the crystals, the fraction the squad took, less
    /// the food the mould stole from the larder (which counts as the
    /// mould's). One half when nothing is gone. A sum over the whole game,
    /// so it rewards holding early (the AI thread's census, 2026-11-12: read
    /// at the cap, a style that grabbed crystals in the last 90 s tied one
    /// that held from 10:00, 7-7) and gives nothing for guarding a crystal
    /// with crumbs left (WM1: every rule that brought a carrier home from a
    /// dry crystal cost the planned hands under `share`, which counts a
    /// crystal with 0.8 charge left as one held). Weighing `share` by the
    /// charge left was tried first and put farm below idle on race seed 5
    /// (-0.265 against -0.232): harvesting empties the crystals you hold.
    pub fn take_share(&self) -> f32 {
        let cs = self.world.crystals();
        let ours: f64 = self.taken.iter().map(|&t| t as f64).sum::<f64>();
        let gone: f64 = cs.iter().enumerate().map(|(i, c)| (self.start_charge[i] - c.1).max(0.0) as f64).sum();
        let mould = (gone - ours).max(0.0);
        let stolen = (self.stolen * self.rules.charge) as f64;
        let (ours, mould) = ((ours - stolen).max(0.0), mould + stolen);
        if ours + mould <= 0.0 {
            0.5
        } else {
            (ours / (ours + mould)) as f32
        }
    }

    /// **One ordered number a round** (score v2, 2026-11-12). Wins and losses
    /// are `End::score`'s bands. A stalemate at the cap, in (-0.5, 0.5), is
    /// half the charge share (`take_share`), less a half, plus a size
    /// term, `0.25 (1 - m) / (1 + m)` with `m` the walkers left over the
    /// walkers at the start: +0.25 for a mould cut to nothing, 0 for one
    /// the size it started, toward -0.25 as it grows. (W2b's stalemate was
    /// the share at the cap alone, so a strike that took the core and left
    /// 7 to 574 walkers scored as a farm that left 9,000: the AI thread's
    /// census.)
    pub fn score(&self) -> Option<f32> {
        self.end.map(|e| match e {
            End::Cap(took) => {
                let m = self.mould as f32 / self.start_mould.max(1) as f32;
                0.5 * (took - 0.5) + 0.25 * (1.0 - m) / (1.0 + m)
            }
            _ => e.score(self.alive(), self.rules.cap),
        })
    }

    /// W2b's score (the crystals held at the cap, no size term), so a
    /// census can still be paired with W2b's and the AI's earlier ones.
    pub fn score_w2b(&self) -> Option<f32> {
        self.end.map(|e| match e {
            End::Cap(_) => End::Cap(self.cap_held.unwrap_or(0.5)).score(self.alive(), self.rules.cap),
            _ => e.score(self.alive(), self.rules.cap),
        })
    }

    /// Run to the end, with `plan` called once a game second before the
    /// frame.
    pub fn play(&mut self, mut plan: impl FnMut(&mut Game<W>)) -> End {
        let tps = self.rules.tps as u64;
        let mut last = u64::MAX;
        while self.end.is_none() {
            let sec = self.world.ticks() / tps;
            if sec != last {
                plan(self);
                last = sec;
            }
            self.step();
        }
        self.end.expect("ended")
    }

    /// The log as a replay file: a header with the seed and the end tick,
    /// then one command a line after its tick.
    pub fn replay_text(&self, seed: u64) -> String {
        let mut out = format!("larderworld replay seed {seed} end {}\n", self.world.ticks());
        for (t, c) in &self.log {
            out += &format!("{t} {}\n", c.to_line());
        }
        out
    }

    /// Play a replay's commands on `game` (fresh, same seed and rules): the
    /// same world reaches the same hash. Returns false on a bad file.
    pub fn replay(game: &mut Game<W>, text: &str) -> bool {
        let mut lines = text.lines();
        let Some(head) = lines.next() else { return false };
        let head: Vec<&str> = head.split_whitespace().collect();
        let Some(end) = head.get(5).and_then(|v| v.parse::<u64>().ok()) else { return false };
        let mut cmds = Vec::new();
        for l in lines {
            let Some((t, rest)) = l.split_once(' ') else { return false };
            let (Ok(t), Some(c)) = (t.parse::<u64>(), Cmd::parse(rest)) else { return false };
            cmds.push((t, c));
        }
        let mut next = 0;
        while game.world.ticks() < end && game.end.is_none() {
            while next < cmds.len() && cmds[next].0 == game.world.ticks() {
                game.apply(cmds[next].1);
                next += 1;
            }
            game.step();
        }
        while next < cmds.len() && cmds[next].0 == game.world.ticks() {
            game.apply(cmds[next].1);
            next += 1;
        }
        true
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn small(seed: u64) -> Game<BigDish> {
        Game::new(race_world(seed, 1, 1_000), Rules::default(), seed)
    }

    #[test]
    fn the_walking_map_goes_round_a_wall_through_its_gap() {
        // 20 x 10, a wall down x = 10 with one gap at y = 8.
        let (w, h) = (20, 10);
        let open = |i: usize| i % w != 10 || i / w == 8;
        let goal = 2 * w + 15;
        let map = distance_map(w, h, goal, open);
        let start = 2 * w + 5;
        assert_eq!(map[goal], 0);
        assert!(map[start] < u32::MAX, "reachable through the gap");
        // Straight would be 10 side steps (50); round the gap is longer.
        assert!(map[start] > 50, "not through the wall: {}", map[start]);
        let mut at = start;
        let mut steps = 0;
        while at != goal && steps < 100 {
            let next = downhill(w, h, &map, at);
            assert!(open(next) && next != at, "every step is open and goes downhill");
            at = next;
            steps += 1;
        }
        assert_eq!(at, goal);
        // Shut the gap: the goal is out of reach and the map says so.
        let sealed = distance_map(w, h, goal, |i| i % w != 10);
        assert_eq!(sealed[start], u32::MAX);
    }

    #[test]
    fn commands_round_trip_as_lines() {
        for c in [
            Cmd::Order(2, Order::Clear(10.5, 20.25)),
            Cmd::Order(0, Order::Burn((1.0, 2.0), (3.0, 4.5))),
            Cmd::Stance(1, Stance::Free),
            Cmd::Buy,
            Cmd::Wall((100.125, 40.0), (100.125, 80.5)),
        ] {
            assert_eq!(Cmd::parse(&c.to_line()), Some(c), "{}", c.to_line());
        }
    }

    #[test]
    fn the_squad_starts_far_from_the_mould() {
        let g = small(1);
        let d = (g.larder.0 - g.core.0).hypot(g.larder.1 - g.core.1);
        assert!(d > 100.0, "larder {:?} core {:?}", g.larder, g.core);
        assert_eq!(g.alive(), 4);
        assert!(g.soldiers.iter().all(|s| g.world.open(s.x, s.y)));
    }

    #[test]
    fn a_clearing_soldier_carries_charge_home() {
        // One soldier on the squad's own crystal: it fills its carry
        // (300 food at 30 a second) and banks it. Four on one crystal
        // empty the cells under them faster than the charge flows back in.
        let mut g = small(2);
        let c = g.larder;
        g.apply(Cmd::Order(0, Order::Clear(c.0, c.1)));
        g.apply(Cmd::Stance(0, Stance::HoldFire));
        let (k, _) = g.crystal_at(c).unwrap();
        let before = g.world.crystals()[k].1;
        for _ in 0..(g.rules.tps * 30) {
            g.step();
        }
        assert!(g.harvested >= g.rules.carry, "banked {} in 30 s", g.harvested);
        // Equal to f32 rounding (with `Rules::home_dry` a carrier banks
        // part loads).
        let books = g.rules.start_bank + g.harvested - g.spent - g.stolen;
        assert!((g.bank - books).abs() < 0.01, "bank {} books {books}", g.bank);
        // From the cells: the pile's total is summed a tick late (the next
        // tick's eating), and the frame's last take is after the ticks.
        let d = &g.world;
        let now: f64 = d.pile_cells.iter().filter(|&&i| d.pile_of[i as usize] as usize == k + 1).map(|&i| d.charge[i as usize] as f64).sum();
        let gone = before - now as f32;
        let held = (g.harvested + g.soldiers[0].carry) * g.rules.charge;
        assert!(gone >= held * 0.999, "the crystal lost {gone}, the squad holds {held}");
    }

    #[test]
    fn a_crystal_run_dry_sends_its_carrier_home_to_hold() {
        // The v1 Fork's case (WM1): a crystal with less than a load in it,
        // all within a soldier's reach (the v1 crystals were 3 dish cells
        // across the radius, under the 4 a soldier takes from). Its soldier
        // empties it, walks home with what it took, banks it and holds
        // there, rather than wait for a full load or walk back for crumbs.
        let mut g = small(2);
        // Holding there is the rule without `seek` (with it the soldier
        // walks on: `a_harvester_at_a_dry_crystal_goes_on_to_the_next`).
        g.rules.seek = false;
        let (k, c) = g.crystal_at(g.larder).unwrap();
        let d = &mut g.world;
        let s = d.p.scale as f32;
        let mut sum = 0.0;
        for q in 0..d.pile_cells.len() {
            let i = d.pile_cells[q] as usize;
            if d.pile_of[i] as usize == k + 1 {
                let (x, y) = ((i % d.w) as f32 + 0.5, (i / d.w) as f32 + 0.5);
                d.charge[i] = if (x - c.0).hypot(y - c.1) <= 3.0 * s { 20.0 } else { 0.0 };
                sum += d.charge[i];
            }
        }
        d.pile_charge[k] = sum;
        assert!(sum / g.rules.charge < 0.5 * g.rules.carry, "{sum} charge is not a short crystal");
        g.apply(Cmd::Order(0, Order::Clear(c.0, c.1)));
        g.apply(Cmd::Stance(0, Stance::HoldFire));
        for _ in 0..(g.rules.tps * 30) {
            g.step();
        }
        assert!(g.harvested >= 0.9 * sum / g.rules.charge, "banked {} of the crystal's {}, carrying {}", g.harvested, sum / g.rules.charge, g.soldiers[0].carry);
        assert!(g.soldiers[0].carry < 1.0, "still carrying {}", g.soldiers[0].carry);
        assert_eq!(g.soldiers[0].order, Order::Hold, "it holds at the larder");
        // With the rule off it waits on the crystal with its part load (W2b).
        let mut w = small(2);
        w.rules.home_dry = false;
        let d = &mut w.world;
        for q in 0..d.pile_cells.len() {
            let i = d.pile_cells[q] as usize;
            if d.pile_of[i] as usize == k + 1 {
                let (x, y) = ((i % d.w) as f32 + 0.5, (i / d.w) as f32 + 0.5);
                d.charge[i] = if (x - c.0).hypot(y - c.1) <= 3.0 * s { 20.0 } else { 0.0 };
            }
        }
        d.pile_charge[k] = sum;
        w.apply(Cmd::Order(0, Order::Clear(c.0, c.1)));
        w.apply(Cmd::Stance(0, Stance::HoldFire));
        for _ in 0..(w.rules.tps * 30) {
            w.step();
        }
        assert!(w.harvested == 0.0 && w.soldiers[0].carry > 0.5 * sum / w.rules.charge, "without the rule: banked {}, carrying {}", w.harvested, w.soldiers[0].carry);
    }

    #[test]
    fn the_stalemate_scores_the_charge_taken_and_the_moulds_size() {
        // Score v2: a harvesting squad takes a share of the charge gone that
        // an idle one doesn't, and at one share a smaller mould scores
        // higher, inside the stalemate band. W2b's score is kept beside it.
        let mut idle = small(2);
        let mut farm = small(2);
        assert_eq!(farm.take_share(), 0.5, "nothing gone yet");
        let c = farm.larder;
        farm.apply(Cmd::Order(0, Order::Clear(c.0, c.1)));
        farm.apply(Cmd::Stance(0, Stance::HoldFire));
        for _ in 0..(farm.rules.tps * 20) {
            idle.step();
            farm.step();
        }
        let (i, f) = (idle.take_share(), farm.take_share());
        assert!(i < 0.01, "the idle squad took {i} of the charge gone");
        assert!(f > 0.2, "the harvesting squad took only {f}");
        let start = farm.start_mould;
        let score = |g: &mut Game<BigDish>, took: f32, mould: usize| {
            g.end = Some(End::Cap(took));
            g.mould = mould;
            g.cap_held = Some(0.5);
            g.score().unwrap()
        };
        let small_mould = score(&mut farm, 0.6, start / 4);
        let same = score(&mut farm, 0.6, start);
        let big_mould = score(&mut farm, 0.6, start * 3);
        assert!(small_mould > same && same > big_mould, "{small_mould} {same} {big_mould}");
        assert!((same - 0.05).abs() < 1e-6, "a mould its starting size adds nothing: {same}");
        assert!(score(&mut farm, 1.0, 1) < 0.5 && score(&mut farm, 0.0, start * 1000) > -0.5, "inside the stalemate band");
        assert_eq!(farm.score_w2b(), Some(0.0), "W2b's score is the held share less a half");
    }

    /// A world that gives a frame's results only when asked for them, as
    /// the GPU world does, to test the step's halves on the CPU. Its
    /// results are in on every third poll (`frame_ready`), so a paced
    /// window finishes some steps when ready and the rest when the next
    /// is due.
    struct Later {
        d: BigDish,
        out: Option<FrameOut>,
        polls: std::cell::Cell<u32>,
    }

    fn later(seed: u64) -> Game<Later> {
        Game::new(Later { d: race_world(seed, 1, 1_000), out: None, polls: Default::default() }, Rules::default(), seed)
    }

    impl World for Later {
        fn size(&self) -> (usize, usize) {
            World::size(&self.d)
        }
        fn scale(&self) -> f32 {
            World::scale(&self.d)
        }
        fn ticks(&self) -> u64 {
            World::ticks(&self.d)
        }
        fn frame(&mut self, f: &Frame, ticks: u32) -> FrameOut {
            World::frame(&mut self.d, f, ticks)
        }
        fn frame_start(&mut self, f: &Frame, ticks: u32) -> Option<FrameOut> {
            assert!(self.out.is_none(), "a frame already running");
            self.out = Some(World::frame(&mut self.d, f, ticks));
            None
        }
        fn frame_finish(&mut self) -> FrameOut {
            self.out.take().expect("a frame running")
        }
        fn frame_ready(&self) -> bool {
            self.polls.set(self.polls.get() + 1);
            self.polls.get().is_multiple_of(3)
        }
        fn build_wall(&mut self, a: (f32, f32), b: (f32, f32), width: f32) {
            assert!(self.out.is_none(), "a wall built while a frame runs");
            World::build_wall(&mut self.d, a, b, width)
        }
        fn living(&self) -> usize {
            World::living(&self.d)
        }
        fn crystals(&self) -> Vec<((f32, f32), f32)> {
            World::crystals(&self.d)
        }
        fn crystal_r(&self, c: usize) -> f32 {
            World::crystal_r(&self.d, c)
        }
        fn landing(&self) -> Option<(f32, f32)> {
            World::landing(&self.d)
        }
        fn open(&self, x: f32, y: f32) -> bool {
            World::open(&self.d, x, y)
        }
        fn stop_births(&mut self) {
            World::stop_births(&mut self.d)
        }
        fn food_deposit(&self) -> f32 {
            World::food_deposit(&self.d)
        }
        fn hash(&self) -> u64 {
            World::hash(&self.d)
        }
        fn calm(&self, at: &[(f32, f32)], r: f32) -> Vec<u32> {
            self.d.calm(at, r)
        }
        fn mould_grid(&self, bin: usize) -> Option<Vec<u32>> {
            self.d.mould_grid(bin)
        }
    }

    /// **A step in halves plays as a whole one** (2026-11-12): the window
    /// begins a step, draws, and finishes it on a later frame; a command
    /// in between finishes it first. On a world that holds results back
    /// until asked, that game equals `step`'s, command for command: the same
    /// log (so the same replay), world hash, soldiers, bank and kills, with
    /// sleep gas on the mould at a soldier (the toolkit's calm touch, read
    /// in `finish_step` after the frame, as `step` read it).
    #[test]
    fn a_step_in_halves_plays_as_a_whole_one() {
        let seed = 5;
        let mut a = small(seed);
        let mut b = later(seed);
        let tps = a.rules.tps as u64;
        let (x, y) = a.larder;
        // Commands at game ticks: attack, buy, a wall, free fire, a buy.
        let cmd = |t: u64| match t {
            15 => vec![Cmd::Order(0, Order::Attack), Cmd::Order(1, Order::Attack)],
            40 => vec![Cmd::Buy],
            70 => vec![Cmd::Wall((x - 8.0, y - 4.0), (x - 8.0, y + 4.0))],
            100 => vec![Cmd::Stance(2, Stance::Free), Cmd::Order(2, Order::Attack)],
            300 => vec![Cmd::Buy, Cmd::Order(3, Order::Attack)],
            _ => Vec::new(),
        };
        let (mut running, mut gassed, mut calm) = (0, 0, 0);
        for n in 0..(tps * 40) {
            a.step();
            b.begin_step();
            assert!(b.in_flight() || b.end.is_some());
            assert!(!b.in_flight() || b.settled_ticks() == n * a.rules.frame_ticks as u64, "a running step's tick is the one it began on");
            for c in cmd(n + 1) {
                assert_eq!(a.apply(c), b.apply(c), "command {c:?} at {}", n + 1);
                assert!(!b.in_flight(), "a command finishes the running step");
            }
            // Sleep gas on the mould round the first soldier it touches
            // (the squad toolkit): the gassed walkers come off the touch in
            // `finish_step`.
            if let (0, Some(k)) = (gassed, a.soldiers.iter().position(|s| s.alive && s.touching > 0)) {
                b.finish_step();
                let (x, y) = (a.soldiers[k].x, a.soldiers[k].y);
                let r = 4.0 * a.rules.touch * a.world.scale();
                gassed = a.world.gas(x, y, r, 600);
                assert_eq!(b.world.d.gas(x, y, r, 600), gassed);
            }
            if gassed > 0 {
                let at: Vec<(f32, f32)> = a.soldiers.iter().map(|s| (s.x, s.y)).collect();
                calm += a.world.gassed_near(&at, a.rules.touch * a.world.scale()).iter().sum::<u32>();
            }
            // Some steps finish at once, the rest when the next begins.
            if n % 3 == 0 {
                b.finish_step();
            } else if b.in_flight() {
                running += 1;
            }
        }
        b.finish_step();
        assert!(running > 300, "steps left running into the next: {running}");
        assert!(gassed > 0 && calm > 0, "the gas slept walkers ({gassed}) by the soldiers ({calm} calm touches)");
        assert_eq!(a.log, b.log, "the same commands at the same ticks");
        assert_eq!(a.world.hash(), b.world.hash(), "the same world");
        assert_eq!(format!("{:?}", a.soldiers), format!("{:?}", b.soldiers));
        assert_eq!((a.bank, a.killed, a.shots, a.harvested, a.walls, a.mould), (b.bank, b.killed, b.shots, b.harvested, b.walls, b.mould));
        assert!(a.killed > 0 && a.walls == 1 && a.soldiers.len() > 4, "the commands acted: killed {}, walls {}, soldiers {}", a.killed, a.walls, a.soldiers.len());
    }

    /// The window's pacing (`App::step` in `larder-world`) for `secs` game
    /// seconds at `fps` frames a real second: each frame owes game time, a
    /// running step finishes once its results are in or when the next step
    /// is due, then the next begins. `click(g, from, to)` is the input of the
    /// frame from real second `from` to `to` (the window's clicks); `due(g)`
    /// runs just before each step begins (a paced replay's commands). Ends
    /// on whole steps.
    fn paced(g: &mut Game<Later>, fps: f32, secs: f32, mut click: impl FnMut(&mut Game<Later>, f32, f32), mut due: impl FnMut(&mut Game<Later>)) {
        let (dt, step) = (1.0 / fps, g.rules.frame_ticks as f32 / g.rules.tps as f32);
        let end = (secs * g.rules.tps as f32) as u64;
        let (mut owed, mut frame) = (0.0f32, 0u32);
        while g.world.ticks() < end && g.end.is_none() {
            click(g, frame as f32 * dt, (frame + 1) as f32 * dt);
            if g.in_flight() && g.step_ready() {
                g.finish_step();
            }
            owed += dt;
            while owed >= step && g.world.ticks() < end && g.end.is_none() {
                g.finish_step();
                due(g);
                g.begin_step();
                owed -= step;
            }
            frame += 1;
        }
        g.finish_step();
        due(g);
    }

    /// **A replay is the same game at any frame rate** (the technical lead's
    /// ask on the step split, 2026-11-12): a command is logged at the game
    /// tick it lands on, not the frame it was clicked in. Rounds played
    /// through the window's pacing at 60 and 23 frames a second, with the
    /// same clicks at the same real times, each replay (`Game::replay`, whole
    /// steps) to their own world hash, bank and soldiers; and the 60 fps log
    /// played back through the window's pacing at 23 and 144 frames a second
    /// gives that hash too.
    #[test]
    fn a_replay_is_the_same_game_at_any_frame_rate() {
        let seed = 7;
        let probe = small(seed);
        let (x, y) = probe.larder;
        let clicks = [
            (1.0, vec![Cmd::Order(0, Order::Attack), Cmd::Order(1, Order::Attack)]),
            (2.71, vec![Cmd::Buy]),
            (4.33, vec![Cmd::Wall((x - 8.0, y - 4.0), (x - 8.0, y + 4.0))]),
            (6.1, vec![Cmd::Stance(2, Stance::Free), Cmd::Order(2, Order::Attack)]),
            (11.47, vec![Cmd::Buy, Cmd::Order(3, Order::Attack)]),
        ];
        let secs = 20.0;
        let record = |fps: f32| {
            let mut g = later(seed);
            paced(&mut g, fps, secs, |g, from, to| {
                for (_, cs) in clicks.iter().filter(|(t, _)| *t >= from && *t < to) {
                    for &c in cs {
                        g.apply(c);
                    }
                }
            }, |_| {});
            g
        };
        let same = |a: &Game<Later>, b_hash: u64, b_bank: f32, b_soldiers: &str, what: &str| {
            assert_eq!(a.world.hash(), b_hash, "{what}: the world hash");
            assert_eq!(a.bank.to_bits(), b_bank.to_bits(), "{what}: the bank");
            assert_eq!(format!("{:?}", a.soldiers), b_soldiers, "{what}: the soldiers");
        };
        let mut ticks = Vec::new();
        for fps in [60.0, 23.0] {
            let a = record(fps);
            assert_eq!(a.log.len(), 8, "{fps} fps: every click logged ({:?}, ended {:?})", a.log, a.end.map(|e| e.band()));
            assert!(a.walls == 1 && a.soldiers.len() > 4, "{fps} fps: the clicks acted");
            ticks.push(a.log.iter().map(|l| l.0).collect::<Vec<u64>>());
            let mut b = small(seed);
            assert!(Game::replay(&mut b, &a.replay_text(seed)));
            assert_eq!(a.world.ticks(), b.world.ticks(), "{fps} fps: the replay's end");
            same(&a, b.world.hash(), b.bank, &format!("{:?}", b.soldiers), &format!("recorded at {fps} fps, replayed in whole steps"));
            if fps == 60.0 {
                for back in [23.0, 144.0] {
                    let mut c = later(seed);
                    let mut next = 0;
                    paced(&mut c, back, secs, |_, _, _| {}, |g| {
                        while next < a.log.len() && a.log[next].0 == g.world.ticks() {
                            g.apply(a.log[next].1);
                            next += 1;
                        }
                    });
                    assert_eq!(next, a.log.len(), "played back at {back} fps: every command");
                    assert_eq!(c.log, a.log, "played back at {back} fps: the same log");
                    same(&c, a.world.hash(), a.bank, &format!("{:?}", a.soldiers), &format!("played back at {back} fps"));
                }
            }
        }
        // 60 fps: [15, 15, 40, 64, 91, 91, 172, 172]; 23 fps: 14 and 171
        // for the first and last clicks (2026-11-12).
        assert_ne!(ticks[0], ticks[1], "the pacing put some clicks on other ticks");
    }

    #[test]
    fn stuck_seconds_count_a_squad_that_can_do_nothing() {
        let mut g = small(1);
        g.bank = 0.0;
        for k in 0..g.soldiers.len() {
            g.apply(Cmd::Stance(k, Stance::HoldFire));
        }
        for _ in 0..(g.rules.tps * 10) {
            g.step();
        }
        assert!(g.stuck >= 9 && g.stuck_run == g.stuck, "stuck {} run {}", g.stuck, g.stuck_run);
        // A bank that can buy a soldier is a way out.
        g.bank = g.rules.soldier_cost;
        for _ in 0..g.rules.tps {
            g.step();
        }
        assert_eq!(g.stuck_run, 0);
    }

    #[test]
    fn the_campaign_switches_are_off_by_default_and_do_what_they_say() {
        let r = Rules::default();
        assert!(r.overrun == 0.0 && r.reserve == 0.0);
        // Overrun: the larder held with the bank empty for 2 s ends the round.
        let mut g = small(1);
        g.rules.overrun = 2.0;
        g.bank = 0.0;
        g.larder_touch = 3;
        for _ in 0..29 {
            g.campaign_rules(1);
        }
        assert!(g.end.is_none());
        g.campaign_rules(1);
        assert!(matches!(g.end, Some(End::Overrun(_))));
        assert_eq!(g.end.unwrap().band(), "larder overrun");
        assert!(g.end.unwrap().score(0, g.rules.cap) < -1.0, "an overrun is a loss");
        // Reserve: a wiped squad with an empty bank waits, and the larder
        // sends one soldier every 10 s until two stand.
        let mut g = small(1);
        g.rules.reserve = 10.0;
        g.bank = 0.0;
        for s in g.soldiers.iter_mut() {
            s.alive = false;
        }
        for _ in 0..(g.rules.tps * 25) {
            g.step();
        }
        assert!(g.end.is_none(), "{:?}", g.end);
        assert_eq!(g.alive(), 2);
        for _ in 0..(g.rules.tps * 15) {
            g.step();
        }
        assert_eq!(g.alive(), 2, "two soldiers stand, so no more come");
        // Without it the same squad has lost.
        let mut g = small(1);
        g.bank = 0.0;
        for s in g.soldiers.iter_mut() {
            s.alive = false;
        }
        g.step();
        assert!(matches!(g.end, Some(End::SquadWiped(_))));
    }

    #[test]
    fn density_holds_from_small_worlds_to_big_ones() {
        // The same world at twice the scale has four times the open cells;
        // with four times the walkers its density is the same.
        let (a, b) = (small(1), Game::new(race_world(1, 2, 4_000), Rules::default(), 1));
        assert!(a.density > 0.0 && a.start_mould == 1_000 && b.start_mould == 4_000);
        assert!((b.density / a.density - 1.0).abs() < 0.05, "{} against {}", b.density, a.density);
        let (w, h) = a.world.size();
        let open = (0..w * h).filter(|&i| a.world.open((i % w) as f32 + 0.5, (i / w) as f32 + 0.5)).count();
        assert!(open < w * h, "rock and the rim are not open ground");
    }

    #[test]
    fn soldiers_on_free_fire_kill_mould_and_pay_for_it() {
        // Shots are free by default (2026-11-12); a price still comes out
        // of the bank shot by shot.
        let mut g = small(3);
        g.rules.price = 10.0;
        let core = g.core;
        for k in 0..4 {
            g.apply(Cmd::Order(k, Order::Attack));
        }
        for _ in 0..(g.rules.tps * 60) {
            g.step();
            if g.end.is_some() {
                break;
            }
        }
        assert!(g.shots > 0 && g.killed > 0, "shots {} killed {}", g.shots, g.killed);
        assert!(g.spent > 0.0 && (g.spent - g.shots as f32 * g.rules.price).abs() < 1e-3);
        let _ = core;
    }

    #[test]
    fn a_wall_costs_by_length_and_needs_a_soldier_near() {
        let mut g = small(4);
        let (x, y) = g.larder;
        let far = (g.core.0, g.core.1 + 30.0);
        assert!(!g.apply(Cmd::Wall(far, (far.0, far.1 + 10.0))), "a wall far from every soldier");
        let bank = g.bank;
        assert!(g.apply(Cmd::Wall((x - 10.0, y - 5.0), (x - 10.0, y + 5.0))));
        assert!((bank - g.bank - g.wall_price((x - 10.0, y - 5.0), (x - 10.0, y + 5.0))).abs() < 1e-3);
        assert!(!g.world.wall.is_empty() && g.world.wall.iter().any(|&w| w));
    }

    #[test]
    fn same_commands_same_hash() {
        let mut a = small(5);
        let plan = |g: &mut Game<BigDish>| {
            let s = g.time() as u32;
            if s == 1 {
                let c = g.core;
                g.apply(Cmd::Order(0, Order::Move(c.0, c.1)));
                g.apply(Cmd::Stance(0, Stance::Free));
            }
            if s == 3 {
                g.apply(Cmd::Buy);
            }
        };
        for _ in 0..(a.rules.tps * 8) {
            if a.world.ticks().is_multiple_of(a.rules.tps as u64) {
                plan(&mut a);
            }
            a.step();
        }
        let text = a.replay_text(5);
        let mut b = small(5);
        assert!(Game::replay(&mut b, &text));
        assert_eq!(a.world.ticks(), b.world.ticks());
        assert_eq!(a.world.hash(), b.world.hash());
        assert_eq!(a.bank.to_bits(), b.bank.to_bits());
    }

    #[test]
    fn queued_orders_run_in_turn_and_read_back_from_a_replay() {
        let mut g = small(6);
        let (x, y) = g.larder;
        let (a, b) = ((x - 8.0, y), (x - 8.0, y - 8.0));
        // Holding: a queued order starts at once; the next one waits.
        assert!(g.apply(Cmd::Queue(0, Order::Move(a.0, a.1))));
        assert!(g.apply(Cmd::Queue(0, Order::Fight(b.0, b.1))));
        assert_eq!(g.soldiers[0].order, Order::Move(a.0, a.1));
        assert_eq!(g.soldiers[0].queued, 1);
        for _ in 0..(g.rules.tps * 10) {
            g.step();
        }
        assert_eq!(g.soldiers[0].order, Order::Fight(b.0, b.1), "the move arrived and the queue moved on");
        assert_eq!(g.soldiers[0].queued, 0);
        // A plain order drops the queue.
        g.apply(Cmd::Queue(0, Order::Move(a.0, a.1)));
        g.apply(Cmd::Order(0, Order::Hold));
        assert_eq!(g.soldiers[0].queued, 0);
        g.apply(Cmd::Rally(Order::Clear(a.0, a.1)));
        g.bank = 1e6;
        assert!(g.apply(Cmd::Buy));
        assert_eq!(g.soldiers.last().unwrap().order, Order::Clear(a.0, a.1), "a bought soldier takes the rally order");
        for c in [Cmd::Rally(Order::Move(5.5, 6.0)), Cmd::Rally(Order::Hold), Cmd::Queue(3, Order::Fight(1.5, 2.25)), Cmd::Queue(1, Order::Clear(3.0, 4.0)), Cmd::Order(2, Order::Fight(7.0, 8.5))] {
            assert_eq!(Cmd::parse(&c.to_line()), Some(c), "{}", c.to_line());
        }
    }

    #[test]
    fn a_harvester_at_a_dry_crystal_goes_on_to_the_next() {
        let mut g = small(7);
        g.rules.seek = true;
        // Soldier 0 is sent to a crystal; then that crystal's charge reads
        // as gone, so the next frame's take finds it dry.
        let target = g.seek(0, None).expect("a crystal with charge");
        let (i, _) = g.crystal_at(target).unwrap();
        g.apply(Cmd::Order(0, Order::Clear(target.0, target.1)));
        for _ in 0..(g.rules.tps * 60) {
            g.step();
            if g.soldiers[0].carry > 0.0 {
                break;
            }
        }
        assert!(g.soldiers[0].carry > 0.0, "it reached the crystal and took");
        let d = &mut g.world;
        for c in 0..d.charge.len() {
            if d.pile_of[c] as usize == i + 1 {
                d.charge[c] = 0.0;
            }
        }
        d.pile_charge[i] = 0.0;
        let mut went_home = false;
        for _ in 0..(g.rules.tps * 90) {
            g.step();
            went_home |= g.soldiers[0].homing;
            if let Order::Clear(x, y) = g.soldiers[0].order {
                if g.crystal_at((x, y)).map(|c| c.0) != Some(i) {
                    break;
                }
            }
        }
        assert!(went_home, "it banked what it had first");
        let Order::Clear(x, y) = g.soldiers[0].order else { panic!("{:?}", g.soldiers[0].order) };
        let next = g.crystal_at((x, y)).unwrap().0;
        assert!(next != i && next != 0, "on to another crystal, not the mould's home: {next}");
        // With the rule off it holds at the larder, as before.
        let mut h = small(7);
        h.rules.seek = false;
        h.apply(Cmd::Order(0, Order::Clear(target.0, target.1)));
        h.after_dry(0, Some(i));
        assert_eq!(h.soldiers[0].order, Order::Hold);
    }

    #[test]
    fn moving_on_skips_crystals_the_mould_ate_from_or_a_soldier_works() {
        let mut g = small(8);
        g.rules.seek = true;
        let first = g.seek(0, None).expect("a safe crystal");
        let (i, _) = g.crystal_at(first).unwrap();
        // Another soldier harvesting it: soldier 0 takes another.
        g.apply(Cmd::Order(1, Order::Clear(first.0, first.1)));
        let other = g.seek(0, None).expect("a second safe crystal");
        assert_ne!(other, first, "not the one soldier 1 works");
        // The mould ate a tenth of it: never chosen, even with no one on it.
        g.apply(Cmd::Order(1, Order::Hold));
        g.world.pile_charge[i] *= 0.9;
        let c = g.seek(0, None).expect("a safe crystal");
        assert_ne!(g.crystal_at(c).unwrap().0, i, "not a crystal the mould ate from");
        // Every pick is on the squad's side: nearer the larder than the core.
        let d = |a: (f32, f32), b: (f32, f32)| (a.0 - b.0).hypot(a.1 - b.1);
        assert!(d(c, g.larder) < d(c, g.core));
    }
}
