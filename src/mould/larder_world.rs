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
use super::squad::{self, Frame, FrameOut, ShotIn, UnitIn};
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
    /// Food a shot costs.
    pub price: f32,
    pub hp: f32,
    /// Health a walker touching a soldier takes a game second.
    pub drain: f32,
    /// Walkers within this many dish cells touch a soldier.
    pub touch: f32,
    /// A soldier's scent, as a share of a full pile's.
    pub smell: f32,
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
            price: 10.0,
            hp: 100.0,
            drain: 1.0,
            touch: 1.5,
            smell: 0.5,
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
}

impl Cmd {
    pub fn to_line(self) -> String {
        match self {
            Cmd::Order(k, o) => super::larder::Cmd::Order(k, o).to_line(),
            Cmd::Stance(k, s) => super::larder::Cmd::Stance(k, s).to_line(),
            Cmd::Buy => "buy".into(),
            Cmd::Wall(a, b) => format!("wall {} {} {} {}", a.0, a.1, b.0, b.1),
        }
    }

    pub fn parse(line: &str) -> Option<Cmd> {
        let w: Vec<&str> = line.split_whitespace().collect();
        if *w.first()? == "wall" {
            let f = |i: usize| w.get(i).and_then(|v| v.parse::<f32>().ok());
            return Some(Cmd::Wall((f(1)?, f(2)?), (f(3)?, f(4)?)));
        }
        match super::larder::Cmd::parse(line)? {
            super::larder::Cmd::Order(k, o) => Some(Cmd::Order(k, o)),
            super::larder::Cmd::Stance(k, s) => Some(Cmd::Stance(k, s)),
            super::larder::Cmd::Buy => Some(Cmd::Buy),
            _ => None,
        }
    }
}

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
    pub end: Option<End>,
    /// The last frame's shots, for the window's tracers.
    pub last_shots: Vec<Shot>,
    /// Walkers alive at the last count (once a game second).
    pub mould: usize,
    /// Each crystal's charge at the start and the charge the squad took
    /// from it, so the cap's share can tell the mould's eating from ours.
    start_charge: Vec<f32>,
    taken: Vec<f32>,
    /// Walking round rock and walls (`Paths`).
    paths: Paths,
    pub rng: Rng,
    pub log: Vec<(u64, Cmd)>,
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
            end: None,
            last_shots: Vec::new(),
            mould,
            start_charge,
            taken,
            paths: Paths::default(),
            rng: Rng::new(seed ^ 0x001a_2de2_501d),
            log: Vec::new(),
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
            alive: true,
            ready: 0,
            beat: 0,
            touching: 0,
            aim: None,
            dense: 0,
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
    pub fn apply(&mut self, c: Cmd) -> bool {
        self.log.push((self.world.ticks(), c));
        match c {
            Cmd::Order(k, o) if k < self.soldiers.len() => {
                self.soldiers[k].order = o;
                self.soldiers[k].homing = false;
                true
            }
            Cmd::Stance(k, st) if k < self.soldiers.len() => {
                self.soldiers[k].stance = st;
                true
            }
            Cmd::Buy => self.buy(),
            Cmd::Wall(a, b) => self.wall(a, b),
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
            Order::Move(x, y) => Some(off((x, y))),
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
        if d < 0.5 {
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
        let stance = if s.order == Order::Attack { Stance::Free } else { s.stance };
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
    /// around it. Does nothing once the round has ended.
    pub fn step(&mut self) {
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
                smell_r: 2.0 * sc,
                smell: self.world.food_deposit() * r.smell * r.frame_ticks as f32,
            });
            who.push(Some(k));
            from.push(at_crystal);
        }
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
        let out = self.world.frame(&f, r.frame_ticks);
        self.shots += f.shots.len() as u32;
        self.last_shots = f.shots.iter().zip(&out.shots).map(|(i, o)| Shot { from: i.from, to: i.to, hit: o.hit, killed: o.killed }).collect();
        self.killed += out.shots.iter().map(|o| o.killed).sum::<u32>();
        for ((u, k), c) in out.units.iter().zip(&who).zip(&from) {
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
                    // A carrier goes home only full, as in W2b and M3, so a
                    // soldier whose crystal runs dry waits there with what it
                    // has until it is ordered on (the window's "crystal ran
                    // dry" alert says when). The maps found it (WM1): on the
                    // v1 Fork a soldier emptied a crystal of 12,032 charge
                    // (188 food) and stood there for a 300 load that never
                    // came; on the v1 Nest the take dwindled as charge seeped
                    // in from the crystal's edge (128 food). Four rules that
                    // send it home were tried on race seeds 5-20 against
                    // W2b's census, and each cost the planned hands. The frame
                    // gave under half its ask: farm and wall lower on all 4
                    // games of seeds 5-6 (farm seed 5, -0.136 against
                    // -0.029). That, and the crystal can't fill the load: farm
                    // lower on 15 of 16 (mean -0.204 against -0.027), wall on
                    // 15, kills down a fifth. That, with a quarter-load floor:
                    // farm even (14 of 16 the same), wall lower on 8 and
                    // higher on none (-0.053 against -0.022). Half its ask
                    // with the whole crystal under one frame's ask: farm lower
                    // on 10 of 10, wall on 8 of 9 (-0.188 against -0.034).
                    // Each sent soldiers home off crystals with crumbs left,
                    // and back for more crumbs, and `share` counts a crystal
                    // with any charge as held while a soldier stands on it.
                    // A rule that keeps the hands' score needs `share` to
                    // weigh a crystal by its charge, or the hands to re-order
                    // a soldier as a player would (`Reports/dead-ends.md`).
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
                }
            }
        }
        self.check_core();
        let now = self.world.ticks();
        if now / r.tps as u64 != t / r.tps as u64 {
            self.mould = self.world.living();
        }
        self.check_end();
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
        let t = self.time();
        if self.alive() == 0 && self.bank < self.rules.soldier_cost {
            self.end = Some(End::SquadWiped(t));
        } else if self.mould == 0 {
            self.end = Some(End::MouldWiped(t));
        } else if t >= self.rules.cap {
            self.end = Some(End::Cap(self.share()));
        }
    }

    /// **Share of the crystals held**, for the cap: of the crystals with
    /// charge left that either side holds, the fraction the squad holds. The
    /// squad holds one with a soldier on it and no mould touching that
    /// soldier; the mould holds the others it has eaten from (charge down
    /// by more than 1% of its start beyond what the squad took) with no
    /// soldier on it. One half when neither holds any. (The first version
    /// gave the mould every crystal without a soldier, reached or not, so
    /// an idle squad scored as if overrun from the first second.)
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

    pub fn score(&self) -> Option<f32> {
        self.end.map(|e| e.score(self.alive(), self.rules.cap))
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
        // Equal to f32 rounding: a carrier that walks home whenever its
        // crystal runs low banks in many small loads.
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
    fn soldiers_on_free_fire_kill_mould_and_pay_for_it() {
        let mut g = small(3);
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
        assert!((g.spent - g.shots as f32 * g.rules.price).abs() < 1e-3);
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
}
