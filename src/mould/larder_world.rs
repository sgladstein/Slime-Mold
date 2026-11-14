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

mod buildings;
mod world;
mod rules;
mod paths;
mod cmd;
mod raising;
mod shots;
mod turn;
mod score;
mod shapes;
mod squads;
// The moved modules' items keep their old paths (`larder_world::World`,
// `larder_world::Rules`, ...).
pub use cmd::*;
pub use paths::*;
pub use raising::*;
pub use rules::*;
pub use world::*;
pub use shapes::shape_keys;
pub use squads::{Formation, LeadRules, Squad, SQUADS_MAX};
pub use buildings::{BuildRules, Building, Kind, TURRET_MUZZLE};
use buildings::Who;

use super::big::{BigDish, BigParams, LIFE_DEAD};
use super::squad::{self, Frame, FrameOut, ShotIn, UnitIn, UnitOut};
use super::Rng;

pub use super::larder::{End, Order, Stance};

/// Walking maps: for each goal cell, every open cell's distance to it
/// (`distance_map`), built when a soldier first needs it and dropped when a
/// wall goes up or `PATHS_EVERY` passes.
#[derive(Default)]
struct Paths {
    maps: std::collections::HashMap<usize, Vec<u32>>,
    at: u64,
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
    /// Carry taken up by tools it holds (`mould::kit`'s carry rule): a
    /// soldier carrying blasts brings home less food. 0 by default.
    pub load: f32,
    /// Its squad (`squads`), an index into `Game::squads`.
    pub squad: u8,
}

/// One shot as the window draws it: from, where it stopped (None: a wall or
/// rock stopped it, or it flew its range), and walkers killed.
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct Shot {
    pub from: (f32, f32),
    pub to: (f32, f32),
    pub hit: Option<(f32, f32)>,
    pub killed: u32,
    /// The soldier that fired it (None: a turret).
    pub by: Option<usize>,
    /// Fired this frame (a bullet's first leg, or an instant ray): the
    /// muzzle flash and the gunshot's sound; later legs are flight.
    pub fired: bool,
}

/// One take from a crystal in the last frame, as the window draws it
/// (`Game::last_takes`; art direction 2026-11-14, harvesting that looks
/// busy): who took (a soldier, or None for a building such as a pump),
/// from which crystal, where the taker stands (fine cells) and how much
/// charge. The soldier's working pulse and the crystal's working spot
/// both read this one list, so they light together.
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct Take {
    pub by: Option<usize>,
    pub crystal: usize,
    pub at: (f32, f32),
    pub took: f32,
}

/// A shot in flight (`Rules::bullet_speed`): where it is, its heading, the
/// fine cells of its range left, its strike radius, and who fired it.
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct Bullet {
    pub at: (f32, f32),
    pub dir: (f32, f32),
    pub left: f32,
    pub r: f32,
    pub by: Option<usize>,
    pub fresh: bool,
    /// Slime hits it flies on through (each one strikes as a shot does).
    pub pierce: u8,
    /// Times it splits where it finally lands (`Rules::split_n`); its
    /// pieces split one fewer time and do not pierce.
    pub split: u8,
}

/// What every soldier's bullet carries when fired: the rounds' flight
/// cards (`rounds::Rounds::arm`). Turret shots carry none.
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub struct BulletMods {
    pub pierce: u8,
    pub split: u8,
}

pub struct Game<W: World> {
    pub world: W,
    pub rules: Rules,
    pub soldiers: Vec<Soldier>,
    pub bank: f32,
    pub spent: f32,
    pub harvested: f32,
    pub stolen: f32,
    /// Food the squad has eaten (`Rules::ration`).
    pub eaten: f32,
    /// The bank couldn't pay the last step's rations: the squad is losing
    /// hit points (`Rules::hunger`).
    pub hungry: bool,
    /// Shots fired (with bullets, once a bullet, not once a leg).
    pub shots: u32,
    pub killed: u32,
    pub walls: u32,
    /// Turrets, lures, pumps and walls placed (`buildings`); a wall's
    /// health lives in the world's cells, not here.
    pub buildings: Vec<Building>,
    /// Food burnt by lures, drawn by pumps, and buildings the mould wore
    /// down.
    pub lured: f32,
    pub pumped: f32,
    pub lost_buildings: u32,
    /// Calls the sirens made (`Kind::Siren`).
    pub sirens: u32,
    /// The squad's base, fine cells: where bought soldiers stand and carried
    /// food goes.
    pub larder: (f32, f32),
    /// The mould's home crystal, fine cells.
    pub core: (f32, f32),
    pub core_down: Option<u64>,
    core_clear: u64,
    /// What the core is (`Core`): `Core::of(&rules)` unless a tier says
    /// otherwise (`with_tier`).
    pub core_rule: Core,
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
    /// The last frame's takes from crystals (`Take`), for the window.
    pub last_takes: Vec<Take>,
    /// Shots in flight (`Rules::bullet_speed`), oldest first; at most
    /// `squad::SHOTS_MAX`, and a shot fired past that is dropped (the
    /// newest go: the old ones are about to land).
    pub bullets: Vec<Bullet>,
    /// The flight cards on soldiers' bullets, and bullets a split could
    /// not add past `squad::SHOTS_MAX` (dropped, newest first, as fired
    /// shots are).
    pub bullet_mods: BulletMods,
    pub bullets_dropped: u32,
    /// Walkers alive at the last count (once a game second).
    pub mould: usize,
    /// The most walkers the mould has had at a count (`Rules::broken`).
    pub peak: usize,
    /// When the mould broke (`Rules::broken`), and the upkeep it had then,
    /// given back if it recovers; None while it is whole.
    pub broken: Option<(u64, f32)>,
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
    /// Crystals with charge left, centre and radius, fine cells: the ground
    /// soldiers can't stand on (`Rules::crystal_solid`), kept each step.
    solid: Vec<((f32, f32), f32)>,
    /// Each crystal's charge at the start, for the shrinking solid disc.
    crystal_full: Vec<f32>,
    pub rng: Rng,
    pub log: Vec<(u64, Cmd)>,
    /// The larder's rally point: the order a bought soldier starts with
    /// (`Cmd::Rally`; the window's right click with the larder selected).
    pub rally: Order,
    /// Every squad slot (`squads`), `SQUADS_MAX` of them; a slot with no
    /// live soldier is free.
    pub squads: Vec<Squad>,
    /// The squad the player controls (`Cmd::Active`), if any.
    pub active: Option<usize>,
    /// Each soldier's place in its squad's formation (`Game::rank_squad`),
    /// and the squad's size when they were matched: a squad whose size has
    /// changed since ranks its members by index until it is matched again.
    pub(super) ranks: Vec<u16>,
    pub(super) ranked: [u16; SQUADS_MAX],
    /// Crystals guarded at the last count (`Rules::larder` off), and what
    /// guarding has paid in all.
    pub guarded: Vec<usize>,
    pub guard_paid: f32,
    /// The step handed to the world and not yet finished (`begin_step`).
    flight: Option<Flight>,
}

/// A step between `Game::begin_step` and `Game::finish_step`: the tick it
/// began on, its frame, which soldier (None the larder) and crystal each
/// unit stands for, and the results if the world gave them at once.
struct Flight {
    t: u64,
    f: Frame,
    /// Each of `f.shots`: who fired it and whether it was fired this
    /// frame; with bullets, leg q is `bullets[q]`'s.
    by: Vec<(Option<usize>, bool)>,
    /// Shots fired this frame.
    fired: u32,
    who: Vec<Who>,
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
            eaten: 0.0,
            hungry: false,
            shots: 0,
            killed: 0,
            walls: 0,
            buildings: Vec::new(),
            lured: 0.0,
            pumped: 0.0,
            lost_buildings: 0,
            sirens: 0,
            larder,
            core,
            core_down: None,
            core_clear: 0,
            core_rule: Core::of(&rules),
            first_touch: None,
            larder_touch: 0,
            stuck: 0,
            stuck_run: 0,
            stuck_mark: (0, 0.0),
            held_ticks: 0,
            reserve_ticks: 0,
            end: None,
            last_shots: Vec::new(),
            last_takes: Vec::new(),
            bullets: Vec::new(),
            bullet_mods: BulletMods::default(),
            bullets_dropped: 0,
            mould,
            peak: mould,
            broken: None,
            start_charge,
            taken,
            start_mould: mould,
            density,
            cap_held: None,
            paths: Paths::default(),
            solid: Vec::new(),
            crystal_full: Vec::new(),
            rng: Rng::new(seed ^ 0x001a_2de2_501d),
            log: Vec::new(),
            flight: None,
            rally: Order::Hold,
            squads: vec![Squad::default(); SQUADS_MAX],
            active: None,
            ranks: Vec::new(),
            ranked: [0; SQUADS_MAX],
            guarded: Vec::new(),
            guard_paid: 0.0,
        };
        g.crystal_full = g.world.crystals().iter().map(|c| c.1).collect();
        g.solid = g.solid_now();
        for k in 0..rules.squad {
            let a = k as f32 / rules.squad.max(1) as f32 * std::f32::consts::TAU;
            g.add_soldier(a);
        }
        g
    }

    /// Gives the game a tier's core (`Tier::core`), read once here: None,
    /// or a tier with no core of its own, keeps `Core::of(&rules)`.
    pub fn with_tier(mut self, tier: Option<super::tiers::Tier>) -> Self {
        if let Some(c) = tier.and_then(|t| t.core()) {
            self.core_rule = c;
        }
        self
    }

    fn s(&self) -> f32 {
        self.world.scale()
    }

    fn add_soldier(&mut self, a: f32) {
        self.add_soldier_at(self.larder, a);
    }

    fn add_soldier_at(&mut self, at: (f32, f32), a: f32) {
        // Round the point (the larder, or a squad), out past a crystal it
        // sits on.
        let sc = self.s();
        let (x, y) = (0..30)
            .map(|n| (3.0 + n as f32) * sc)
            .map(|r| (at.0 + r * a.cos(), at.1 + r * a.sin()))
            .find(|&(x, y)| self.standable(x, y))
            .unwrap_or(at);
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
            load: 0.0,
            squad: 0,
        });
    }

    /// Game seconds since the start.
    pub fn time(&self) -> f32 {
        self.world.ticks() as f32 / self.rules.tps as f32
    }

    /// How far the mould is through overrunning the larder (`Rules::overrun`),
    /// 0-1: 0 while it is not held with the bank dry (or the rule is off),
    /// 1 as the round is lost. The window darkens the larder's sprite by it.
    pub fn overrun_share(&self) -> f32 {
        if self.rules.overrun <= 0.0 {
            return 0.0;
        }
        (self.held_ticks as f32 / self.secs(self.rules.overrun).max(1) as f32).min(1.0)
    }

    fn secs(&self, s: f32) -> u64 {
        (s * self.rules.tps as f32).round() as u64
    }

    pub fn alive(&self) -> usize {
        self.soldiers.iter().filter(|s| s.alive).count()
    }

}

#[cfg(test)]
mod tests;
