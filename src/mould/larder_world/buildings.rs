//! **Buildings** for the Larder world (owner, 2026-11-12 04:23Z: "turrets,
//! walls, and other building"). Each building works through something the
//! mould already does, not a drawn-on effect (the owner's bar: the mould is
//! integral to play, and several plans should counter each other):
//!
//! - **Turret**: a gun that does not move and fires at the densest mould
//!   it can see, at its own rate, paid per shot like a soldier's
//!   (`Rules::price`). Its hits lay the Swarmer's alarm (the squad frame's
//!   `SHOT_ALARM`), so against a Swarmer it calls the swarm onto itself.
//! - **Lure**: a beacon that lays food scent, the field the mould climbs,
//!   and burns bank food each second it is on (`Cmd::Toggle` switches it).
//!   It pulls the mould into a turret line, off a farmed crystal or away
//!   from the larder, at the price of a slower economy.
//! - **Pump**: stands on a crystal and drains its charge into the bank
//!   with no carriers. It is food on a crystal to the mould, so a target.
//! - **Wall**: today's wall (`BigDish::build_wall`) as one kind, so the
//!   soldier controls thread's build orders raise walls and turrets alike.
//!
//! **Why stationary units.** Turret, lure and pump are units in the squad
//! frame (`squad::Frame`): a turret is a unit's sight and touch plus a
//! `ShotIn`, a lure a unit's smell, a pump a unit's take. The frame is
//! already one tested path on the CPU and the GPU (W2a: one GPU frame
//! equals `squad::frame` exactly), so the buildings play the same on both
//! worlds without a new engine pass or binding. The cost is units: the
//! frame takes at most `squad::UNITS_MAX` (64), so with 40 soldiers and the
//! larder there is room for `BuildRules::max` (20) buildings.
//!
//! **Raised over time** (owner's card, 2026-11-12 04:41Z: "Soldiers build
//! over time"). A building is placed as a ghost with `built` 0 to 1 and
//! acts only once whole. Soldiers on `Order::Build` stand at it and add to
//! `built` (`Game::build_turn`), `BuildRules::raise` seconds for a lone
//! builder, builders adding up; an idle soldier in range takes up a ghost
//! by itself. A ghost
//! holds a tenth of its health and gains the rest as it rises, and the
//! mould wears a ghost as it wears a building. A wall writes its cells into
//! the world, at full health, when it is whole.

use super::{Game, Shot, World};
use crate::mould::squad::{Burst, BurstRules, ShotIn, UnitIn, UnitOut};

/// What a building is. A wall carries its far end (fine cells); its near
/// end is the building's `at`.
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum Kind {
    Turret,
    Lure,
    Pump,
    /// A second drop-off for carriers (`Game::drop_offs`). It smells of the
    /// bank as the larder does, at `outpost_hoard` of its strength, and
    /// walkers touching it steal from the bank as at the larder.
    Outpost,
    /// An alarm on a pole: while on, every `siren_every` seconds it lays
    /// `siren_alarm` over `siren_r` dish cells through the squad toolkit's
    /// burst (`World::burst`, no kill, the alarm field's one writer). The
    /// mould tiers answer the alarm (`mould::tiers`): a Sleeper wakes, a
    /// Sporer bursts its spores, a Swarmer floods the place. The plain mould
    /// has no alarm sense and ignores it. A way to call the mould where the
    /// player wants it (a turret line, away from a crystal) or to wake a
    /// Sleeper on the player's own clock.
    Siren,
    Wall { to: (f32, f32) },
}

impl Kind {
    pub fn name(self) -> &'static str {
        match self {
            Kind::Turret => "turret",
            Kind::Lure => "lure",
            Kind::Pump => "pump",
            Kind::Outpost => "outpost",
            Kind::Siren => "siren",
            Kind::Wall { .. } => "wall",
        }
    }
}

#[derive(Clone, Copy, Debug, PartialEq)]
pub struct Building {
    pub kind: Kind,
    /// Fine cells.
    pub at: (f32, f32),
    pub hp: f32,
    pub max_hp: f32,
    /// 0 a ghost, 1 whole; it acts only when whole.
    pub built: f32,
    pub alive: bool,
    /// A lure or siren that is on (the player's switch); every other kind
    /// ignores it.
    pub on: bool,
    /// Doing its job this frame: a turret that fired, a lure laying scent,
    /// a pump that drew charge. For the window's marks.
    pub working: bool,
    /// The pump's crystal.
    pub crystal: Option<usize>,
    /// Tick of a turret's next shot or a siren's next call.
    pub(super) ready: u64,
    /// What it read last frame.
    pub touching: u32,
    pub aim: Option<(f32, f32)>,
    pub dense: u32,
}

impl Building {
    /// Whole and standing.
    pub fn up(&self) -> bool {
        self.alive && self.built >= 1.0
    }
}

/// How far out a turret's shot starts, in its touch radii: the tip of the
/// barrels the window draws (`Marks::turret`, the head one touch radius
/// with barrels to 1.7).
pub const TURRET_MUZZLE: f32 = 1.7;

/// The buildings' numbers. Distances in dish cells, times in game seconds,
/// food from the bank, as `Rules`. First numbers for play, to be set by
/// the census: a turret is a tougher, faster-firing soldier that cannot
/// walk or harvest and does not count against `max_squad`; a pump draws a
/// sixth of a soldier's harvest rate but never walks; a lure smells twice
/// a soldier.
#[derive(Clone, Copy, Debug)]
pub struct BuildRules {
    /// Buildings standing at once, walls not counted (the squad frame's
    /// 96 units, less 40 soldiers and the larder, leave 55).
    pub max: usize,
    /// No building within this of another, dish cells.
    pub gap: f32,
    /// Seconds a lone builder takes to raise a building (a wall: per
    /// `WALL_RAISE_LEN` dish cells of length); 0 is whole when placed.
    /// 8 (2026-11-12, seeds 5-12, cap 900, against 0, the hands sending
    /// their nearest soldier to raise what they place): every plan still
    /// beats idle and random 8-0; turtle is lower on 7 of 8 (p 0.016),
    /// guard and pumps on 7 (p 0.07), wall, ring and killzone even. The
    /// cost falls on the turret plans, which had the edge on every tier.
    pub raise: f32,
    pub turret_cost: f32,
    pub turret_hp: f32,
    pub turret_range: f32,
    pub turret_every: f32,
    pub lure_cost: f32,
    pub lure_hp: f32,
    /// The lure's scent, as a multiple of a soldier's (`Rules::smell`, a
    /// share of a full pile's), over a disc of `lure_r` dish cells.
    ///
    /// 8 over 6 cells (`examples/worldbuild.rs`, 2026-11-12, race world at
    /// scale 2, 5,000 walkers, the lure 0.35 of the way from the core, 8
    /// seeds, on against off): walkers within 8 cells of it at 20 s were
    /// more on 8 of 8 (p 0.008; 26-93 against 7-34), and the mould then
    /// wore the lure down on 3 of 8 seeds within the minute, after which
    /// the crowd thinned. The first numbers, 2 over 3 cells, drew more on 7
    /// of 8 at 20 s with 1 tie but by a few walkers (1-42 against 0-34),
    /// too few to see on screen. The measure named before the run, the
    /// count at 60 s, was not significant for either (6-2 and 7-1, p 0.29
    /// and 0.07): by then a strong lure is eaten. A lure is bait the mould
    /// takes, not a standing beacon.
    pub lure_smell: f32,
    pub lure_r: f32,
    /// Food a second a lure burns while on. 0 since 2026-11-14: the owner
    /// ruled "There should be no maintenance costs in this game" (04:53Z),
    /// so a lure costs its price once. 2 was the old burn.
    pub lure_burn: f32,
    pub pump_cost: f32,
    pub pump_hp: f32,
    /// Food a second a pump draws from its crystal.
    pub pump_rate: f32,
    /// A working pump's scent, as a multiple of a soldier's, over
    /// `lure_r` (0: none beyond its crystal's own). The pump as bait.
    pub pump_smell: f32,
    pub outpost_cost: f32,
    pub outpost_hp: f32,
    /// The outpost's bank smell as a share of the larder's (`Rules::hoard`).
    pub outpost_hoard: f32,
    pub siren_cost: f32,
    pub siren_hp: f32,
    /// Seconds between a siren's calls.
    pub siren_every: f32,
    /// The alarm a call lays (`squad::SHOT_ALARM` is one hit's) over
    /// `siren_r` dish cells, and the alarm field's keep if no tier has
    /// turned it on (the toolkit's 0.95). First numbers: three hits' alarm
    /// over a turret's reach, every 4 s, a call loud and wide enough that a
    /// Swarmer's flood is seen across the screen.
    pub siren_alarm: f32,
    pub siren_r: f32,
    pub siren_keep: f32,
    /// How near the mould must be to wear a building, dish cells (its
    /// footprint).
    pub touch: f32,
    /// Health a walker touching a building takes a game second, times
    /// `Rules::mass`. 1, what soldiers and buildings shared (`Rules::drain`)
    /// until soldiers' bite went to 2 a walker whatever the mass
    /// (2026-11-12, the owner: walking into the slime "should be very
    /// deadly"); buildings were not part of that ask.
    pub drain: f32,
    /// A building may be placed anywhere, not only within a soldier's
    /// range (`Rules::range`): the window sends the selected soldiers, or
    /// with none selected the nearest free one (`Cmd::Send`), to walk
    /// there and raise it. Owner's playtest, 2026-11-12 22:06Z: "having
    /// to be near the build location to build is annoying". Off is the
    /// old rule, for replays and census hands that relied on it.
    pub anywhere: bool,
}

impl Default for BuildRules {
    fn default() -> Self {
        BuildRules {
            max: 40,
            gap: 2.5,
            raise: 8.0,
            turret_cost: 500.0,
            turret_hp: 300.0,
            turret_range: 15.0,
            turret_every: 0.5,
            lure_cost: 300.0,
            lure_hp: 150.0,
            lure_smell: 8.0,
            lure_r: 6.0,
            lure_burn: 0.0,
            pump_cost: 500.0,
            pump_hp: 200.0,
            pump_rate: 5.0,
            pump_smell: 0.0,
            outpost_cost: 400.0,
            outpost_hp: 300.0,
            outpost_hoard: 0.25,
            siren_cost: 300.0,
            siren_hp: 150.0,
            siren_every: 4.0,
            siren_alarm: 150.0,
            siren_r: 15.0,
            siren_keep: 0.95,
            touch: 2.0,
            drain: 1.0,
            anywhere: true,
        }
    }
}

/// A ghost's health, as a share of the whole building's.
const GHOST_HP: f32 = 0.1;

/// Who a unit in the frame is.
#[derive(Clone, Copy, Debug, PartialEq)]
pub(super) enum Who {
    Soldier(usize),
    Larder,
    Building(usize),
}

impl<W: World> Game<W> {
    /// The food a building of `kind` costs (a wall by its length).
    pub fn build_price(&self, kind: Kind, at: (f32, f32)) -> f32 {
        let b = self.rules.build;
        match kind {
            Kind::Turret => b.turret_cost,
            Kind::Lure => b.lure_cost,
            Kind::Pump => b.pump_cost,
            Kind::Outpost => b.outpost_cost,
            Kind::Siren => b.siren_cost,
            Kind::Wall { to } => self.wall_price(at, to),
        }
    }

    /// Buildings standing or being raised.
    pub fn standing(&self) -> usize {
        self.buildings.iter().filter(|b| b.alive && !matches!(b.kind, Kind::Wall { .. })).count()
    }

    /// Why a building can't go at `at` now, or None if it can.
    pub fn build_blocked(&self, kind: Kind, at: (f32, f32)) -> Option<&'static str> {
        let sc = self.s();
        if self.bank < self.build_price(kind, at) {
            return Some("not enough food");
        }
        if self.rules.build.anywhere {
            if !self.soldiers.iter().any(|s| s.alive) {
                return Some("no soldier to build it");
            }
        } else if !self.reachable(at) {
            return Some("no soldier in range");
        }
        if let Kind::Wall { to } = kind {
            return (!self.rules.build.anywhere && !self.reachable(to)).then_some("no soldier in range");
        }
        if !self.world.open(at.0, at.1) {
            return Some("not open ground");
        }
        if self.standing() >= self.rules.build.max {
            return Some("too many buildings");
        }
        let gap = self.rules.build.gap * sc;
        if self.buildings.iter().any(|b| b.alive && !matches!(b.kind, Kind::Wall { .. }) && (b.at.0 - at.0).hypot(b.at.1 - at.1) < gap) {
            return Some("too close to another building");
        }
        if kind == Kind::Pump && self.pump_crystal(at).is_none() {
            return Some("a pump goes on a crystal");
        }
        None
    }

    /// The crystal a pump at `at` would stand on: one whose disc holds the
    /// point.
    fn pump_crystal(&self, at: (f32, f32)) -> Option<usize> {
        self.world.crystals().iter().enumerate().find(|(i, c)| (c.0 .0 - at.0).hypot(c.0 .1 - at.1) <= self.world.crystal_r(*i)).map(|(i, _)| i)
    }

    /// Place a building of `kind` at `at` (fine cells), paid now. True if
    /// placed. With `BuildRules::raise` 0 it is whole at once.
    pub fn build(&mut self, kind: Kind, at: (f32, f32)) -> bool {
        if self.build_blocked(kind, at).is_some() {
            return false;
        }
        let price = self.build_price(kind, at);
        self.bank -= price;
        self.spent += price;
        let b = self.rules.build;
        let hp = match kind {
            Kind::Turret => b.turret_hp,
            Kind::Lure => b.lure_hp,
            Kind::Pump => b.pump_hp,
            Kind::Outpost => b.outpost_hp,
            Kind::Siren => b.siren_hp,
            Kind::Wall { .. } => self.world_wall_hp(),
        };
        let crystal = if kind == Kind::Pump { self.pump_crystal(at) } else { None };
        // A ghost starts at a tenth of its health and gains the rest as it
        // rises (`raise`), so a half-built building is weaker.
        self.buildings.push(Building { kind, at, hp: GHOST_HP * hp, max_hp: hp, built: 0.0, alive: true, on: true, working: false, crystal, ready: 0, touching: 0, aim: None, dense: 0 });
        if b.raise <= 0.0 {
            let k = self.buildings.len() - 1;
            self.raise(k, 1.0);
        }
        true
    }

    /// The nominal health of a wall cell, for the window (the world keeps
    /// each cell's own).
    fn world_wall_hp(&self) -> f32 {
        400.0
    }

    /// Add `amount` (0..1 of the whole) to building `k`; at 1 it stands,
    /// and a wall writes its cells into the world. The soldier controls
    /// thread's build order calls this.
    pub fn raise(&mut self, k: usize, amount: f32) {
        let Some(b) = self.buildings.get(k).copied() else { return };
        if !b.alive || b.built >= 1.0 {
            return;
        }
        let built = (b.built + amount).min(1.0);
        self.buildings[k].built = built;
        self.buildings[k].hp = (b.hp + (1.0 - GHOST_HP) * b.max_hp * (built - b.built)).min(b.max_hp);
        if built >= 1.0 {
            if let Kind::Wall { to } = b.kind {
                self.wall_cells(b.at, to);
            }
        }
    }

    /// Switch lure or siren `k` on or off. True if it is one.
    pub fn toggle(&mut self, k: usize) -> bool {
        match self.buildings.get_mut(k) {
            Some(b) if b.alive && matches!(b.kind, Kind::Lure | Kind::Siren) => {
                b.on = !b.on;
                true
            }
            _ => false,
        }
    }

    /// The buildings' turn before the frame: their units (and a turret's
    /// shot) into the frame, a lure's food out of the bank.
    pub(super) fn building_units(&mut self, units: &mut Vec<UnitIn>, shots: &mut Vec<ShotIn>, who: &mut Vec<Who>, from: &mut Vec<Option<usize>>, dt: f32) {
        let r = self.rules;
        let b = r.build;
        let sc = self.s();
        let t = self.world.ticks();
        let strike = (r.strike * sc).min(crate::mould::squad::R_MAX);
        for k in 0..self.buildings.len() {
            let bd = self.buildings[k];
            self.buildings[k].working = false;
            if !bd.alive || matches!(bd.kind, Kind::Wall { .. }) {
                continue;
            }
            let mut u = UnitIn { x: bd.at.0, y: bd.at.1, touch: b.touch * sc, ..UnitIn::default() };
            if !bd.up() {
                // A ghost does nothing yet, but the mould can wear it.
                units.push(u);
                who.push(Who::Building(k));
                from.push(None);
                continue;
            }
            let mut crystal = None;
            match bd.kind {
                Kind::Turret => {
                    u.sight = b.turret_range * sc;
                    if bd.ready <= t && self.bank >= r.price && bd.dense >= r.aim_min {
                        if let Some(aim) = bd.aim {
                            let (dx, dy) = (aim.0 - bd.at.0, aim.1 - bd.at.1);
                            let d = dx.hypot(dy).max(0.1);
                            let reach = b.turret_range * sc;
                            if d <= reach + 4.0 && self.clear_line(bd.at, aim) {
                                // From the barrel tip, not the turret's middle
                                // (art director, 11-14: the drawn head's barrels
                                // reach 1.7 touch radii), capped at half the way
                                // to the aim so mould at its foot is still hit.
                                // The line was checked clear from the middle,
                                // which covers the barrel.
                                let m = (TURRET_MUZZLE * b.touch * sc).min(0.5 * d);
                                let from = (bd.at.0 + dx / d * m, bd.at.1 + dy / d * m);
                                shots.push(ShotIn { from, to: (bd.at.0 + dx / d * reach, bd.at.1 + dy / d * reach), r: strike });
                                self.buildings[k].ready = t + self.secs(b.turret_every);
                                self.buildings[k].working = true;
                                self.bank -= r.price;
                                self.spent += r.price;
                            }
                        }
                    }
                }
                Kind::Lure => {
                    let burn = b.lure_burn * dt;
                    if bd.on && self.bank >= burn {
                        self.bank -= burn;
                        self.spent += burn;
                        self.lured += burn;
                        u.smell_r = b.lure_r * sc;
                        u.smell = self.world.food_deposit() * r.smell * b.lure_smell * r.frame_ticks as f32;
                        self.buildings[k].working = true;
                    }
                }
                Kind::Pump => {
                    if let Some(c) = bd.crystal {
                        u.take = b.pump_rate * dt * r.charge;
                        u.take_r = self.world.crystal_r(c).min(4.0 * sc);
                        u.smell_r = b.lure_r * sc;
                        u.smell = self.world.food_deposit() * r.smell * b.pump_smell * r.frame_ticks as f32;
                        crystal = Some(c);
                    }
                }
                Kind::Siren => {
                    // Before the frame starts, so the world is idle: the
                    // call goes through the burst, the alarm's one writer.
                    if bd.on && bd.ready <= t {
                        let burst = Burst { at: (bd.at.0 / sc, bd.at.1 / sc), alarm: b.siren_alarm, alarm_r: b.siren_r, ..Burst::default() };
                        self.world.burst(&burst, &BurstRules { alarm_keep: b.siren_keep, ..BurstRules::default() });
                        self.buildings[k].ready = t + self.secs(b.siren_every);
                        self.buildings[k].working = true;
                        self.sirens += 1;
                    }
                }
                Kind::Outpost => {
                    u.smell_r = 3.0 * sc;
                    u.smell = self.world.food_deposit() * r.hoard * b.outpost_hoard * self.bank.max(0.0) / 10_000.0 * r.frame_ticks as f32;
                }
                Kind::Wall { .. } => unreachable!(),
            }
            units.push(u);
            who.push(Who::Building(k));
            from.push(crystal);
        }
    }

    /// The buildings' turn after the frame: what each read, a pump's take
    /// into the bank, and the mould's wear.
    pub(super) fn building_out(&mut self, k: usize, u: &UnitOut, dt: f32) {
        let r = self.rules;
        let bd = &mut self.buildings[k];
        bd.touching = u.touching;
        bd.aim = u.target;
        bd.dense = u.dense;
        let up = bd.up();
        if up && bd.kind == Kind::Pump && u.took > 0.0 {
            bd.working = true;
            let food = u.took / r.charge;
            self.bank += food;
            self.harvested += food;
            self.pumped += food;
        }
        // By mass, as the larder's steal: the bank's loss follows the body's cover, not its walker count.
        if up && bd.kind == Kind::Outpost && u.touching > 0 {
            let take = (r.steal * r.mass * u.touching as f32 * dt).min(self.bank.max(0.0));
            self.bank -= take;
            self.stolen += take;
        }
        let bd = &mut self.buildings[k];
        if u.touching > 0 {
            // By mass (`BuildRules::drain` 1 x mass, #174): a building outlasts a thick body for a while; walls go "slowly" (owner, 11-11 15:24Z).
            bd.hp -= r.build.drain * r.mass * u.touching as f32 * dt;
            if bd.hp <= 0.0 {
                bd.hp = 0.0;
                bd.alive = false;
                bd.working = false;
                self.lost_buildings += 1;
            }
        }
    }

    /// Where carriers can bank: the larder first, then each standing
    /// outpost (for the soldier controls thread's nearest drop-off).
    pub fn drop_offs(&self) -> Vec<(f32, f32)> {
        std::iter::once(self.larder).chain(self.buildings.iter().filter(|b| b.kind == Kind::Outpost && b.up()).map(|b| b.at)).collect()
    }

    /// The turrets' shots of the last frame, for the window's tracers (they
    /// are in `last_shots` with the soldiers'; a shot no soldier fired is a
    /// turret's, which also finds a turret bullet's later legs).
    pub fn turret_shots(&self) -> impl Iterator<Item = &Shot> {
        self.last_shots.iter().filter(|s| s.by.is_none())
    }
}

#[cfg(test)]
mod tests {
    use super::super::*;
    use super::*;

    fn small(seed: u64) -> Game<BigDish> {
        // Whole when placed: these tests are of what buildings do; raising
        // them has its own tests (`BuildRules::raise` set in each).
        Game::new(race_world(seed, 1, 1_000), Rules { start_bank: 1e6, build: BuildRules { raise: 0.0, ..BuildRules::default() }, ..Rules::default() }, seed)
    }

    /// A soldier at `at`, so a building there is in reach.
    fn stand(g: &mut Game<BigDish>, at: (f32, f32)) {
        g.soldiers[0].x = at.0;
        g.soldiers[0].y = at.1;
    }

    /// An open cell near `p`.
    fn open_near(g: &Game<BigDish>, p: (f32, f32)) -> (f32, f32) {
        for r in 0..40 {
            for k in 0..16 {
                let a = k as f32 / 16.0 * std::f32::consts::TAU;
                let q = (p.0 + r as f32 * a.cos(), p.1 + r as f32 * a.sin());
                if g.world.open(q.0, q.1) {
                    return q;
                }
            }
        }
        panic!("no open cell near {p:?}");
    }

    #[test]
    fn building_commands_round_trip_as_lines() {
        for c in [Cmd::Build(Kind::Turret, (10.5, 20.25)), Cmd::Build(Kind::Lure, (1.0, 2.0)), Cmd::Build(Kind::Pump, (3.0, 4.5)), Cmd::Build(Kind::Wall { to: (7.0, 8.5) }, (5.0, 6.0)), Cmd::Toggle(3)] {
            assert_eq!(Cmd::parse(&c.to_line()), Some(c), "{}", c.to_line());
        }
    }

    #[test]
    fn a_building_needs_a_soldier_near_food_and_room() {
        let mut g = small(1);
        let far = open_near(&g, g.core);
        assert_eq!(g.build_blocked(Kind::Turret, far), None, "anywhere: a soldier walks there");
        g.rules.build.anywhere = false;
        assert_eq!(g.build_blocked(Kind::Turret, far), Some("no soldier in range"));
        g.rules.build.anywhere = true;
        let at = open_near(&g, (g.larder.0 + 8.0, g.larder.1));
        let bank = g.bank;
        assert!(g.apply(Cmd::Build(Kind::Turret, at)));
        assert_eq!(bank - g.bank, g.rules.build.turret_cost);
        assert!(g.buildings[0].up(), "whole at once while `raise` is 0");
        assert_eq!(g.build_blocked(Kind::Lure, (at.0 + 1.0, at.1)), Some("too close to another building"));
        // Off every crystal, in reach, away from the turret.
        let off = (0..64)
            .map(|i| {
                let a = i as f32 * 0.7;
                (g.larder.0 + (4.0 + i as f32 * 0.2) * a.cos(), g.larder.1 + (4.0 + i as f32 * 0.2) * a.sin())
            })
            .find(|&p| g.world.open(p.0, p.1) && g.reachable(p) && g.crystal_at(p).is_none_or(|(i, c)| (c.0 - p.0).hypot(c.1 - p.1) > g.world.crystal_r(i)) && (p.0 - at.0).hypot(p.1 - at.1) > 3.0)
            .expect("open ground off the crystals");
        assert_eq!(g.build_blocked(Kind::Pump, off), Some("a pump goes on a crystal"));
        g.bank = 10.0;
        assert_eq!(g.build_blocked(Kind::Lure, open_near(&g, (g.larder.0 - 8.0, g.larder.1))), Some("not enough food"));
    }

    #[test]
    fn a_ghost_acts_only_once_raised() {
        let mut g = small(2);
        g.rules.build.raise = 10.0;
        let at = open_near(&g, (g.larder.0 + 8.0, g.larder.1));
        let to = (at.0, at.1 + 8.0);
        assert!(g.build(Kind::Wall { to }, at));
        assert_eq!(g.walls, 0, "a ghost wall writes no cells");
        assert!(!g.world.wall.iter().any(|&w| w));
        g.raise(0, 0.6);
        assert_eq!(g.walls, 0);
        g.raise(0, 0.6);
        assert_eq!((g.walls, g.buildings[0].built), (1, 1.0));
        assert!(g.world.wall.iter().any(|&w| w), "whole: its cells are in the world");
    }

    #[test]
    fn a_turret_fires_on_mould_in_sight_and_kills() {
        let mut g = small(3);
        let at = open_near(&g, (g.core.0 + 10.0, g.core.1));
        stand(&mut g, at);
        g.soldiers[0].stance = Stance::HoldFire;
        assert!(g.build(Kind::Turret, at));
        for _ in 0..(g.rules.tps * 10) {
            g.step();
        }
        let b = g.buildings[0];
        assert!(g.shots > 5 && g.killed > 0, "a turret by the core: {} shots, {} killed", g.shots, g.killed);
        assert!(b.hp < b.max_hp || !b.alive, "the mould wore it: {} of {}", b.hp, b.max_hp);
        // Shots cost what a soldier's do.
        assert!((g.spent - g.rules.build.turret_cost - g.shots as f32 * g.rules.price).abs() < 1e-2);
    }

    #[test]
    fn a_turret_shot_leaves_from_its_barrel_tip() {
        // The window draws the barrels to `TURRET_MUZZLE` touch radii; the
        // shot starts there (art director, 11-14), on the line to its aim,
        // and never past half the way, so mould at its foot is still hit.
        let mut g = small(1);
        let at = open_near(&g, (g.core.0 + 10.0, g.core.1));
        stand(&mut g, at);
        g.soldiers[0].stance = Stance::HoldFire;
        assert!(g.build(Kind::Turret, at));
        let tip = TURRET_MUZZLE * g.rules.build.touch * g.s();
        let mut seen = 0;
        for _ in 0..(g.rules.tps * 10) {
            g.step();
            for s in g.turret_shots().filter(|s| s.fired) {
                let out = (s.from.0 - at.0).hypot(s.from.1 - at.1);
                let along = ((s.from.0 - at.0) * (s.to.0 - at.0) + (s.from.1 - at.1) * (s.to.1 - at.1)) / ((s.to.0 - at.0).hypot(s.to.1 - at.1) * out.max(1e-6));
                assert!(out <= tip + 1e-3 && out > 0.0 && along > 0.999, "from {out} out (tip {tip}), along {along}");
                seen += 1;
            }
        }
        assert!(seen > 3, "{seen} turret shots");
    }

    #[test]
    fn a_siren_calls_with_the_alarm_every_few_seconds_while_on() {
        let run = |on: bool| {
            let mut g = small(4);
            let at = open_near(&g, (0.6 * g.larder.0 + 0.4 * g.core.0, 0.6 * g.larder.1 + 0.4 * g.core.1));
            stand(&mut g, at);
            assert!(g.apply(Cmd::Build(Kind::Siren, at)));
            if !on {
                assert!(g.apply(Cmd::Toggle(0)));
            }
            let mould = g.world.living();
            for _ in 0..(g.rules.tps * 9) {
                g.step();
            }
            let i = at.1 as usize * g.world.w + at.0 as usize;
            (g.sirens, g.world.alarm.get(i).copied().unwrap_or(0.0), g.world.alarm.len(), mould)
        };
        // Calls at 0, 4 and 8 s; the plain mould has no alarm until then.
        let (calls, alarm, len, _) = run(true);
        assert_eq!(calls, 3);
        assert!(len > 0 && alarm > 0.0, "the siren's alarm on its cell: {alarm}");
        let (calls, _, len, _) = run(false);
        assert_eq!((calls, len), (0, 0), "a siren switched off calls nothing and turns no alarm on");
        // It kills nothing: the call is a burst with no kill.
        let mut g = small(4);
        let at = open_near(&g, g.core);
        stand(&mut g, at);
        let before = g.world.living();
        assert!(g.build(Kind::Siren, at));
        g.begin_step();
        assert_eq!(g.world.living(), before, "the call killed walkers");
        assert!(Cmd::parse(&Cmd::Build(Kind::Siren, at).to_line()) == Some(Cmd::Build(Kind::Siren, at)));
    }

    #[test]
    fn a_lure_burns_food_and_lays_scent_while_on() {
        let run = |on: bool| {
            let mut g = small(4);
            // The burn is off by default (no upkeep); on, it still works.
            g.rules.build.lure_burn = 2.0;
            // Open ground between the bases, away from the crystals' scent.
            let at = open_near(&g, (0.6 * g.larder.0 + 0.4 * g.core.0, 0.6 * g.larder.1 + 0.4 * g.core.1));
            stand(&mut g, at);
            assert!(g.build(Kind::Lure, at));
            if !on {
                assert!(g.apply(Cmd::Toggle(0)));
            }
            for _ in 0..(g.rules.tps * 4) {
                g.step();
            }
            let i = at.1 as usize * g.world.w + at.0 as usize;
            (g.world.scent[i], g.lured)
        };
        let (scent_on, lured) = run(true);
        let (scent_off, lured_off) = run(false);
        assert!((lured - 4.0 * 2.0).abs() < 0.2 && lured_off == 0.0, "burnt {lured} food in 4 s on, {lured_off} off");
        assert!(scent_on > 2.0 * scent_off.max(1e-6), "scent at the lure {scent_on}, off {scent_off}");
    }

    /// No upkeep (owner, 2026-11-14 04:53Z: "There should be no
    /// maintenance costs in this game"): a lure left on costs nothing once
    /// it is paid for, and nothing else standing drains the bank.
    #[test]
    fn standing_buildings_cost_nothing_to_keep() {
        let mut g = small(4);
        let at = open_near(&g, (0.6 * g.larder.0 + 0.4 * g.core.0, 0.6 * g.larder.1 + 0.4 * g.core.1));
        stand(&mut g, at);
        assert!(g.build(Kind::Lure, at));
        let siren = open_near(&g, (at.0 + 8.0, at.1));
        stand(&mut g, siren);
        assert!(g.build(Kind::Siren, siren));
        for k in 0..g.soldiers.len() {
            g.apply(Cmd::Stance(k, Stance::HoldFire));
        }
        let spent = g.spent;
        let mut working = false;
        for _ in 0..(g.rules.tps * 6) {
            g.step();
            working |= g.buildings[0].working;
        }
        assert!(working, "the lure gave off scent");
        assert_eq!((g.lured, g.eaten, g.spent - spent), (0.0, 0.0, 0.0), "upkeep paid");
    }

    #[test]
    fn a_pump_draws_its_crystal_into_the_bank() {
        let mut g = small(5);
        let (c, centre) = g.crystal_at(g.larder).expect("the larder stands on a crystal");
        let charge = g.world.crystals()[c].1;
        assert!(g.build(Kind::Pump, centre) || g.build(Kind::Pump, open_near(&g, centre)));
        let bank = g.bank;
        for _ in 0..(g.rules.tps * 10) {
            g.step();
        }
        let want = 10.0 * g.rules.build.pump_rate;
        assert!((g.pumped - want).abs() < 0.05 * want, "pumped {} in 10 s", g.pumped);
        // The squad's rations (`Rules::ration`) come out of the same bank.
        assert!(g.bank + g.eaten > bank && charge - g.world.crystals()[c].1 > 0.9 * want * g.rules.charge);
        assert!(g.take_share() > 0.5, "the pump's take is the squad's");
    }

    #[test]
    fn a_ghost_gains_health_as_it_rises() {
        let mut g = small(7);
        g.rules.build.raise = 10.0;
        let at = open_near(&g, (g.larder.0 + 8.0, g.larder.1));
        assert!(g.build(Kind::Turret, at));
        let max = g.buildings[0].max_hp;
        assert_eq!(g.buildings[0].hp, 0.1 * max);
        g.raise(0, 0.5);
        assert!((g.buildings[0].hp - 0.55 * max).abs() < 1e-3);
        g.raise(0, 0.5);
        assert!((g.buildings[0].hp - max).abs() < 1e-3 && g.buildings[0].up());
    }

    #[test]
    fn an_outpost_is_a_drop_off() {
        let mut g = small(8);
        let at = open_near(&g, (g.larder.0 + 8.0, g.larder.1 + 4.0));
        assert_eq!(g.drop_offs(), vec![g.larder]);
        assert!(g.build(Kind::Outpost, at));
        assert_eq!(g.drop_offs(), vec![g.larder, at]);
        assert!(Cmd::parse(&Cmd::Build(Kind::Outpost, at).to_line()) == Some(Cmd::Build(Kind::Outpost, at)));
    }

    /// 40 soldiers, the larder and `BuildRules::max` buildings fit the squad
    /// frame (`squad::UNITS_MAX`), which drops units past it silently.
    #[test]
    fn a_full_squad_and_every_building_fit_the_frame() {
        let r = Rules::default();
        assert!(r.max_squad + 1 + r.build.max <= crate::mould::squad::UNITS_MAX);
        let mut g = small(9);
        g.rules.build.gap = 0.0;
        while g.buy() {}
        assert_eq!(g.alive(), r.max_squad);
        let mut placed = 0;
        for i in 0..400 {
            let a = i as f32 * 2.4;
            let at = (g.larder.0 + (2.0 + 0.05 * i as f32) * a.cos(), g.larder.1 + (2.0 + 0.05 * i as f32) * a.sin());
            if g.build(Kind::Lure, at) {
                placed += 1;
            }
        }
        assert_eq!(placed, r.build.max, "the cap stops the 41st");
    }

    #[test]
    fn same_buildings_same_hash() {
        let plan = |g: &mut Game<BigDish>| {
            if g.time() as u32 == 1 {
                let at = open_near(g, (g.larder.0 + 8.0, g.larder.1));
                g.apply(Cmd::Build(Kind::Turret, at));
                let at = open_near(g, (g.larder.0 - 8.0, g.larder.1));
                g.apply(Cmd::Build(Kind::Lure, at));
            }
            if g.time() as u32 == 3 {
                g.apply(Cmd::Toggle(1));
            }
        };
        let mut a = small(6);
        for _ in 0..(a.rules.tps * 6) {
            if a.world.ticks().is_multiple_of(a.rules.tps as u64) {
                plan(&mut a);
            }
            a.step();
        }
        let text = a.replay_text(6);
        let mut b = small(6);
        assert!(Game::replay(&mut b, &text));
        assert_eq!(a.world.hash(), b.world.hash());
        assert_eq!(a.buildings, b.buildings);
    }

    #[test]
    fn soldiers_raise_a_ghost_over_time_and_builders_add_up() {
        let mut g = small(11);
        g.rules.build.raise = 4.0;
        let at = open_near(&g, (g.larder.0 + 6.0, g.larder.1));
        assert!(g.apply(Cmd::Build(Kind::Turret, at)));
        assert!(!g.buildings[0].up(), "a ghost while it is raised");
        let ghost = g.buildings[0].hp;
        // One soldier sent: it walks there and raises it.
        g.apply(Cmd::Order(0, Order::Build(0)));
        for k in 1..g.soldiers.len() {
            g.apply(Cmd::Order(k, Order::Move(g.larder.0 - 20.0, g.larder.1)));
        }
        let mut half = None;
        for n in 0..(g.rules.tps * 30) {
            g.step();
            if half.is_none() && g.buildings[0].built >= 0.5 {
                half = Some((n, g.buildings[0].hp));
            }
            if g.buildings[0].up() {
                break;
            }
        }
        let (_, hp_half) = half.expect("it rose past half");
        assert!(hp_half > ghost && hp_half < g.buildings[0].max_hp, "half-built is weaker: {hp_half}");
        assert!(g.buildings[0].up(), "a lone builder raises it in its time");
        g.step();
        assert_eq!(g.soldiers[0].order, Order::Hold, "then it holds");
        // Two builders on the next: about twice as fast.
        let time = |g: &mut Game<BigDish>, n: usize| {
            let at2 = open_near(g, (g.larder.0, g.larder.1 + 8.0));
            assert!(g.apply(Cmd::Build(Kind::Turret, at2)));
            let b = g.buildings.len() - 1;
            for k in 0..n {
                g.soldiers[k].x = at2.0;
                g.soldiers[k].y = at2.1;
                g.apply(Cmd::Order(k, Order::Build(b)));
            }
            for k in n..g.soldiers.len() {
                g.apply(Cmd::Order(k, Order::Move(g.larder.0 - 20.0, g.larder.1)));
            }
            let mut t = 0;
            while !g.buildings[b].up() && t < 1000 {
                g.step();
                t += 1;
            }
            t
        };
        let mut a = small(12);
        a.rules.build.raise = 4.0;
        let mut b = small(12);
        b.rules.build.raise = 4.0;
        for g in [&mut a, &mut b] {
            g.rules.build.gap = 0.0;
        }
        let (one, two) = (time(&mut a, 1), time(&mut b, 2));
        assert!(two * 3 < one * 2, "two builders {two} ticks, one {one}");
        let _ = g;
    }

    #[test]
    fn an_idle_soldier_raises_a_ghost_in_range_and_a_wall_stands_only_when_whole() {
        let mut g = small(13);
        g.rules.build.raise = 2.0;
        let (x, y) = g.larder;
        assert!(g.apply(Cmd::Wall((x - 10.0, y - 5.0), (x - 10.0, y + 5.0))));
        assert!(!g.world.wall.iter().any(|&w| w), "no wall cells while a ghost");
        // Nobody ordered: the idle squad at the larder takes it up.
        for _ in 0..(g.rules.tps * 20) {
            g.step();
            if g.buildings[0].up() {
                break;
            }
        }
        assert!(g.buildings[0].up(), "raised by the idle squad");
        assert!(g.world.wall.iter().any(|&w| w), "its cells are in the world once whole");
        assert_eq!(Cmd::parse(&Cmd::Order(2, Order::Build(7)).to_line()), Some(Cmd::Order(2, Order::Build(7))));
    }

    #[test]
    fn a_build_far_away_sends_a_soldier_who_then_goes_back_to_work() {
        let mut g = small(14);
        g.rules.build.raise = 2.0;
        // Every soldier busy: walking away from the site.
        let (x, y) = g.larder;
        for k in 0..g.soldiers.len() {
            g.apply(Cmd::Order(k, Order::Move(x - 6.0, y)));
        }
        let far = open_near(&g, ((x + g.core.0) / 2.0, (y + g.core.1) / 2.0));
        assert!(!g.reachable(far), "the site is out of every soldier's range");
        assert!(g.apply(Cmd::Build(Kind::Turret, far)));
        assert_eq!(g.raising(0), (0, 0));
        assert_eq!(g.raise_left(0), None, "nobody raising it yet");
        assert_eq!(Cmd::parse(&Cmd::Send(0).to_line()), Some(Cmd::Send(0)));
        assert!(g.apply(Cmd::Send(0)));
        let k = (0..g.soldiers.len()).find(|&k| g.soldiers[k].order == Order::Build(0)).expect("one soldier sent");
        assert_eq!(g.raising(0), (0, 1), "on the way");
        assert_eq!((g.soldiers[k].queued, g.soldiers[k].queue[0]), (1, Order::Move(x - 6.0, y)), "its walk is kept for after");
        let mut seen = None;
        for _ in 0..(g.rules.tps * 120) {
            g.step();
            if seen.is_none() {
                seen = g.raise_left(0);
            }
            if g.buildings[0].up() {
                break;
            }
        }
        let left = seen.expect("a time left showed while it rose");
        assert!(left > 0.0 && left <= 2.0, "{left}");
        assert!(g.buildings[0].up(), "it walked there and raised it");
        g.step();
        assert_eq!(g.soldiers[k].order, Order::Move(x - 6.0, y), "then back to its order");
        assert!(!g.apply(Cmd::Send(0)), "nothing to send to once it stands");
    }

    #[test]
    fn an_idle_soldier_is_sent_before_a_busy_one() {
        let mut g = small(15);
        g.rules.build.raise = 2.0;
        let (x, y) = g.larder;
        for k in 1..g.soldiers.len() {
            g.apply(Cmd::Order(k, Order::Move(x - 6.0, y)));
        }
        let at = open_near(&g, (x + 8.0, y));
        assert!(g.apply(Cmd::Build(Kind::Turret, at)));
        assert_eq!(g.send(0), Some(0), "soldier 0 is the only idle one");
        assert_eq!(g.soldiers[0].queued, 0, "an idle soldier has nothing to go back to");
    }
}
