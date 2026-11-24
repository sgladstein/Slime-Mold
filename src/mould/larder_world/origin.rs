//! **The origin: the slime's heart on its start crystal, which you kill to
//! win the planet** (2026-11-22, Milestone 2's win; the owner's idea of
//! 11-16 04:58Z, "every planet there is a more boss like entity at the
//! slime start crystal that you need to kill to win. The origin of the
//! slime of the planet"; the design in
//! `Reports/origin-design-2026-11-22/README.md`, which Direction approved
//! that morning with its three questions settled: a die-back of about ten
//! seconds, draining its crystal stops its regrowth, and the last walker
//! still wins as a fallback).
//!
//! The origin is the last-stand heart (`stand.rs`) from the landing: a lump
//! of the slime's own armoured walkers held round its start crystal
//! (`Game::core`), drawn with the heart's look. It is a win condition, not
//! a king (the owner's 11-12 "headless" ruling stands): the rest of the
//! slime does not need it to make walkers.
//!
//! - **It grows from its crystal.** Every `OriginRules::grow_every` seconds
//!   it takes in the free walkers round its crystal (a reach that widens
//!   from the crystal's middle until it holds its planet's `size`), up to
//!   `most` times that, but only while its crystal has charge. Only the
//!   squad empties a crystal (the owner, 11-21 06:30Z), so draining or
//!   pumping its crystal stops it regrowing.
//! - **It fights back.** It lunges at a soldier within `near` of its
//!   crystal (the heart's bout, with its cap on the health one bout may
//!   take), and when it loses members to shots it calls the body home: a
//!   raid on the soldiers striking it, at most every `call_every` seconds.
//! - **Its death wins the planet.** Cut under `floor` of the most it held
//!   (or `floor_min` members), it breaks: births stop, the slime stops
//!   eating (the engine's `starve`) and its upkeep goes up `die_upkeep`
//!   times, so the rest of it starves where it stands;
//!   after `die_back` seconds the planet is won (`End::MouldWiped`, which
//!   the run counts as the goal), or sooner at the last walker.
//! - **The late last stand gathers into it** (`time_stand`): late in a
//!   planet against an army, every free walker on the map streams to the
//!   origin instead of forming a second heart, so a planet ends in one
//!   fight at one place.
//!
//! GPU only (the owner's 11-17 card): a world that cannot sow never forms
//! one, and the planet is won at the last walker as before.

use super::{End, Game, World};
use crate::mould::big::{Lump, LUMPS};

/// The origin's numbers. Times in game seconds, distances in dish cells.
/// Off by default (the campaign and the sandbox are frozen); a run turns
/// it on (`run::ENEMY_RULES`) and sets its size by planet
/// (`run::ORIGIN_SIZE`). Armour, bite, steering and the lunge are the
/// last-stand heart's (`StandRules`).
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct OriginRules {
    pub on: bool,
    /// The members it gathers at the landing and regrows to.
    pub size: f32,
    /// A take from its crystal comes only while it holds under this,
    /// times `size`. 1.5 until 11-22: Engine's siege on planet 1 (seed 2)
    /// met an origin of 1,120 against a size of 700, which it wore down
    /// only 10 a minute; at 1, with each take's reach cut to its shortfall,
    /// it regrows only to about its size.
    pub most: f32,
    /// Seconds between takes from its crystal, and how far (dish cells)
    /// the reach widens each time it is short.
    pub grow_every: f32,
    pub widen: f32,
    /// It breaks under this share of the most it held, or this many
    /// members.
    pub floor: f32,
    pub floor_min: f32,
    /// It lunges at, and calls the body home against, soldiers within this
    /// of its crystal (dish cells, past the crystal's rim).
    pub near: f32,
    /// Members lost in a second that call the body home (a share of the
    /// most it held), and the least time between calls.
    pub call_loss: f32,
    pub call_every: f32,
    /// The die-back's length and the upkeep's factor in it.
    pub die_back: f32,
    pub die_upkeep: f32,
    /// Its members' blows and extra bite (`big::Lump`, 15 at most), and
    /// its bouts: seconds between lunges, a lunge's length, and the share
    /// of the squad's health a bout may take (at most `cap_most`
    /// soldiers'). Tougher than the last stand's (4 and 1, every 20 s,
    /// a quarter): with those, 44 soldiers walking straight at it won
    /// planet 1 at 1:23 and planet 2 at 1:12 with none lost (run seed 1,
    /// lavapipe, 11-22), and Direction asked that a straight rush cost
    /// real losses and take minutes.
    pub armour: u32,
    pub bite: u32,
    pub every: f32,
    pub lunge: f32,
    pub cap: f32,
    pub cap_most: f32,
    /// How far its members may run from its centre in a lunge (times its
    /// radius; the last stand's is 2.5).
    pub lunge_leash: f32,
    /// It regrows only after this many seconds with no loss of `hurt` of
    /// the most it held in a second (11-22, Engine's siege on #699: on
    /// planet 2, run seed 1, it sat at 1,900 to 2,200 all game against 52
    /// rifles, refilling from its charged crystal as fast as they cut it,
    /// and the planet was lost at 11:29). A sustained fight now wears it
    /// down; a strike and a pull-back lets it refill. At 10 s and 0.5% a
    /// 48-soldier rush won at 1:16 with none lost; at 4 s and 2%, at 1:39
    /// with 10 lost (52: 1:36, 7 lost); with no pause, 44 at 2:05 with 34
    /// lost and 24 wiped (planet 2, size 700, run seed 1, lavapipe).
    pub quiet: f32,
    pub hurt: f32,
}

impl Default for OriginRules {
    fn default() -> Self {
        OriginRules { on: false, size: 400.0, most: 1.0, grow_every: 2.0, widen: 0.5, floor: 0.1, floor_min: 50.0, near: 30.0, call_loss: 0.02, call_every: 20.0, die_back: 10.0, die_upkeep: 40.0, armour: 12, bite: 2, every: 12.0, lunge: 8.0, cap: 0.4, cap_most: 20.0, lunge_leash: 2.5, quiet: 4.0, hurt: 0.02 }
    }
}

/// The origin's state beside the heart it is (`Stand::live` with `origin`).
#[derive(Clone, Copy, Debug, Default)]
pub struct OriginState {
    /// It formed this planet.
    pub formed: bool,
    /// The tick it formed, and when it broke (the die-back's start).
    pub born: Option<u64>,
    pub fell: Option<u64>,
    /// The upkeep the die-back holds.
    up: f32,
    /// How far it reaches round its crystal's middle now (dish cells).
    pub reach: f32,
    /// The tick of its last take and its last call home; its calls.
    grown: u64,
    called: Option<u64>,
    pub calls: u32,
    /// Its members a second ago.
    last: u32,
    /// The tick it last lost members to a fight (`OriginRules::hurt`).
    pub struck: Option<u64>,
}

impl<W: World> Game<W> {
    /// Once a game second, before the last stand: the origin forms at the
    /// landing, and once it has broken the die-back runs to the win.
    pub(super) fn time_origin(&mut self) {
        let r = self.rules.origin;
        if !r.on || !self.world.can_sow() {
            return;
        }
        let now = self.world.ticks();
        let tps = self.rules.tps as f32;
        if let Some(fell) = self.origin.fell {
            self.world.set_upkeep(self.origin.up);
            if now >= fell + (r.die_back.max(0.0) * tps) as u64 && self.end.is_none() {
                self.end = Some(End::MouldWiped(self.time()));
            }
            return;
        }
        if !self.origin.formed && self.mould > 0 {
            self.start_origin();
        }
    }

    /// The origin gathering now round the start crystal; false with no
    /// free lump (none is taken at the landing) or no slime there.
    pub(super) fn start_origin(&mut self) -> bool {
        let mut r = self.rules.stand;
        (r.armour, r.bite, r.every) = (self.rules.origin.armour, self.rules.origin.bite, self.rules.origin.every);
        let Some(lump) = (0..LUMPS).find(|&l| !self.lump_taken(l)) else { return false };
        let home = self.core;
        let reach = self.rules.origin.widen.max(0.5);
        self.world.grow_lump(lump, (home, reach));
        let size = self.world.size();
        self.world.set_lump(lump, Lump { aim: super::raid_cell(home, size), pull: r.pull, leash: r.leash, armour: r.armour, bite: r.bite, ..Lump::default() });
        let now = self.world.ticks();
        let tps = self.rules.tps as f32;
        let heart = super::Heart::origin(lump, home, now, now + (r.every.max(1.0) * tps) as u64);
        self.stand.live = Some(heart);
        self.origin = OriginState { formed: true, born: Some(now), reach, grown: now, ..OriginState::default() };
        true
    }

    /// Whether the origin's crystal still has charge (only the squad
    /// empties one).
    fn origin_fed(&self) -> bool {
        let home = self.core;
        let sc = self.world.scale();
        self.world.crystals().iter().filter(|c| (c.0 .0 - home.0).hypot(c.0 .1 - home.1) < sc).any(|c| c.1 > 0.0)
    }

    /// Once a second from `keep_heart` while the heart is the origin: its
    /// regrowth from its crystal and its call home. Returns whether it
    /// broke (the heart's own keeping then stops).
    pub(super) fn keep_origin(&mut self, h: &mut super::Heart) -> bool {
        let r = self.rules.origin;
        let now = self.world.ticks();
        let tps = self.rules.tps as f32;
        // The lump's sums lag a step behind its forming (as the fruit's):
        // no floor until it has held a body.
        let floor = (r.floor * h.most as f32).max(r.floor_min);
        if h.most as f32 >= 2.0 * r.floor_min && (h.members as f32) < floor {
            self.break_origin(h.lump);
            return true;
        }
        // Hurt this second: no regrowth for `quiet` seconds.
        let lost = self.origin.last.saturating_sub(h.members);
        if lost > 0 && lost as f32 >= r.hurt * h.most as f32 {
            self.origin.struck = Some(now);
        }
        let calm = self.origin.struck.is_none_or(|t| now >= t + (r.quiet.max(0.0) * tps) as u64);
        // Its regrowth: from its crystal while it has charge, the reach
        // widening while it is short of its size; or, gathering (the late
        // last stand), every free walker on the map. Neither while it is
        // being cut down (`OriginRules::quiet`).
        if calm && now >= self.origin.grown + (r.grow_every.max(0.5) * tps) as u64 {
            self.origin.grown = now;
            if h.gather {
                let whole = self.whole_map();
                self.world.grow_lump(h.lump, (h.centre, whole));
            } else if self.origin_fed() && (h.members as f32) < r.most * r.size {
                let cr = self.crystal_at(self.core).map_or(4.0 * self.world.scale(), |(c, _)| self.world.crystal_r(c)) / self.world.scale().max(1.0);
                // The reach widens while it is short of its size; a take
                // reaches only the share of it the shortfall asks (its
                // square root, for a disc's area), so a take on a crystal
                // the slime has packed again stays near its size. A take
                // gets every free walker in its disc, as many as the packed
                // slime holds: Engine's siege, 11-22, planet 1, seed 3, at
                // 6:15, 4 short of 700 at a reach of 5.5, it went from 696
                // to 1,095 in one take (and seed 2 formed at 1,120 against
                // 700 with `most` 1.5). A cap on the disc a body of its size
                // fills (a walker a fine cell) did not bind: the packed
                // slime there held about 1.6 a fine cell.
                let short = (r.size - h.members as f32).max(0.0) / r.size.max(1.0);
                if short > 0.0 {
                    self.origin.reach = (self.origin.reach + r.widen.max(0.1)).min(cr + 2.0);
                }
                let take = (self.origin.reach * short.sqrt()).max(0.5);
                self.world.grow_lump(h.lump, (self.core, take));
            }
        }
        // The call home: struck hard enough, with soldiers near it.
        self.origin.last = h.members;
        let due = self.origin.called.is_none_or(|t| now >= t + (r.call_every.max(1.0) * tps) as u64);
        if due && h.most > 0 && lost as f32 >= r.call_loss * h.most as f32 && lost > 0 {
            if let Some(k) = self.origin_near() {
                let to = (self.soldiers[k].x, self.soldiers[k].y);
                let raid = self.rules.raid;
                if self.call_home(k, to, raid.size, raid.share) {
                    self.origin.called = Some(now);
                    self.origin.calls += 1;
                }
            }
        }
        false
    }

    /// The live soldier nearest the origin's crystal within its reach
    /// (`OriginRules::near` past the crystal's rim) that the slime can
    /// smell (LAB: not the hidden stranded squad).
    pub(super) fn origin_near(&self) -> Option<usize> {
        let sc = self.world.scale();
        let home = self.core;
        let cr = self.crystal_at(home).map_or(0.0, |(c, _)| self.world.crystal_r(c));
        let reach = cr + self.rules.origin.near * sc;
        (0..self.soldiers.len()).filter(|&k| self.soldiers[k].alive && self.soldiers[k].smell > 0.0).map(|k| (k, (self.soldiers[k].x - home.0).hypot(self.soldiers[k].y - home.1))).filter(|&(_, d)| d <= reach).min_by(|a, b| a.1.total_cmp(&b.1)).map(|(k, _)| k)
    }

    /// Whether soldier `k` stands within the origin's reach (`origin_near`'s
    /// test for one soldier).
    pub(super) fn origin_reaches(&self, k: usize) -> bool {
        let sc = self.world.scale();
        let home = self.core;
        let cr = self.crystal_at(home).map_or(0.0, |(c, _)| self.world.crystal_r(c));
        let s = self.soldiers[k];
        // LAB (the rescue planet): a soldier the slime cannot smell
        // (smell 0, the hidden stranded squad) is not one it lunges at;
        // without this the origin hunted them down in 2:20 from 50 cells.
        s.alive && s.smell > 0.0 && (s.x - home.0).hypot(s.y - home.1) <= cr + self.rules.origin.near * sc
    }

    /// The origin breaks: its lump lets go, births stop and the slime's
    /// upkeep goes up (`OriginRules::die_upkeep`), so the rest of it dies
    /// back over `die_back` seconds to the win.
    fn break_origin(&mut self, lump: usize) {
        let r = self.rules.origin;
        self.world.form_lump(lump, None);
        self.world.stop_births();
        self.world.set_starve(true);
        let base = if self.ceiling.start > 0 { self.ceiling.upkeep } else if let Some((_, up)) = self.broken { up } else { self.world.upkeep() };
        self.origin.up = base * r.die_upkeep.max(1.0);
        self.world.set_upkeep(self.origin.up);
        self.origin.fell = Some(self.world.ticks());
        self.stand.live = None;
    }

    /// What the HUD shows of the origin: its members against the most it
    /// held, and the die-back's seconds left once it has broken.
    pub fn origin_hud(&self) -> Option<(u32, u32, Option<f32>)> {
        if !self.origin.formed {
            return None;
        }
        if let Some(fell) = self.origin.fell {
            let left = self.rules.origin.die_back - (self.world.ticks().saturating_sub(fell)) as f32 / self.rules.tps.max(1) as f32;
            return Some((0, 0, Some(left.max(0.0))));
        }
        self.stand.live.filter(|h| h.origin).map(|h| (h.members, h.most, None))
    }
}
