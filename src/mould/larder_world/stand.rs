//! **The last-stand heart** (2026-11-20, the thread "Make the slime a more
//! interesting enemy"; D of the balance thread's brief "Keeping a big squad
//! under threat", its order on the owner's pick that day, "Condensed
//! strikers + late lunge": A and B first, then D "as every planet's end,
//! once A works").
//!
//! The owner, 11-20 14:10Z: once the squad is big enough "the slime cannot
//! touch them". 14:16Z: hardened slime "doesn't actually make this more
//! challenging because hardened slime doesn't attack you. It makes it
//! easier." So the end of a planet against an army: once the slime is
//! under `at` of its peak and the squad has at least `army` soldiers, what
//! is left of it gathers into one heart (a lump, `big::Lump`, the
//! mini-bosses' and the fruit's machinery) on its heart crystal (the
//! heartbeat's, `Game::heart_site`). Every walker on the map not in another
//! lump streams to it, abandoning the crystals it held, so the remnant
//! reads as one living thing the army must go and kill, not stragglers to
//! mop up. Each member takes `armour` blows and bites as `1 + bite`
//! walkers. And it attacks: every `every` seconds the whole heart lunges at
//! the nearest soldier at `haste` times its pace for `lunge` seconds, then
//! draws back to its crystal. Cut under `last` of the most it held, or
//! `life` seconds old, it breaks and is slime again, so the end of a
//! planet never waits on a few armoured walkers. Condensed strikers (`strikers.rs`) never join
//! it, so its strikers still charge round it.
//!
//! No new slime: the heart is the slime's own walkers, and the planet is
//! won as before, when the last of them dies.
//!
//! The picture is the simulation: the heart is its members packed past the
//! crowd limit, drawn with a look of its own (`LumpLook::Heart`, a deep
//! gold over a hot white-gold core), its strength its members against the
//! most it held, so it dims as you cut it down.
//!
//! GPU only (the owner's 11-17 19:21Z card): a world that cannot sow
//! (`World::can_sow`, the CPU engine; it takes no new slime rules) never
//! gathers one.

use super::{Game, World};
use crate::mould::big::{Lump, LUMPS};

/// The heart's numbers. Times in game seconds, distances in dish cells.
/// Off by default (the campaign and the sandbox are frozen); a run turns it
/// on from its planet (`run::ENEMY_RULES`, `run::FIRST_PLANET`).
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct StandRules {
    pub on: bool,
    /// The slime gathers once its mass is under this share of its peak.
    pub at: f32,
    /// Only against a squad of at least this many (the strikers' `full`):
    /// a smaller squad still mops up as before. At 12, on planet 2 map 2
    /// (lavapipe, the strat bot) a heart of 3,578 formed against 14
    /// soldiers and the planet was lost at 5:40.
    pub army: f32,
    /// Each member's blows and extra bite (`big::Lump`).
    pub armour: u32,
    pub bite: u32,
    /// Its steering: the pull to its aim and the leash (times its radius),
    /// at rest and lunging.
    pub pull: f32,
    pub leash: f32,
    pub lunge_leash: f32,
    /// Seconds between lunges (the first comes this long after it forms),
    /// a lunge's length, and its members' pace in one.
    pub every: f32,
    pub lunge: f32,
    pub haste: f32,
    /// Every `grow_every` seconds it takes in the slime grown since.
    pub grow_every: f32,
    /// Cut under this share of the most it held, or this many seconds
    /// after it formed, it breaks and is slime again, which starves off
    /// its food as a broken slime does (`Rules::broken`). Kept to the end,
    /// a heart cut low was the stragglers the owner found "annoying"
    /// (11-12 22:06Z), now armoured (lavapipe, the strat bot): on planet 2
    /// map 5 one cut from 955 to 30 sat on its crystal for 130 s and the
    /// planet ran from 3:31 to 7:19 with no soldier lost; a last charge
    /// until it died, tried next, held 29 walkers pressed on the ground
    /// between it and the squad for 200 s on map 2 until the game's cap.
    pub last: f32,
    pub life: f32,
}

impl Default for StandRules {
    fn default() -> Self {
        StandRules { on: false, at: 0.2, army: 24.0, armour: 4, bite: 1, pull: 40.0, leash: 1.0, lunge_leash: 2.5, every: 20.0, lunge: 10.0, haste: 3.0, grow_every: 3.0, last: 0.15, life: 150.0 }
    }
}

/// The heart.
#[derive(Clone, Copy, Debug)]
pub struct Heart {
    /// The lump it is (`World::grow_lump`).
    pub lump: usize,
    /// Its crystal (fine cells) and the tick it formed.
    pub home: (f32, f32),
    pub born: u64,
    /// Its members at the last look and the most it has had, and its
    /// centre (fine cells).
    pub members: u32,
    pub most: u32,
    pub centre: (f32, f32),
    /// Lunging until this tick (None at rest); the next lunge's tick.
    pub lunging: Option<u64>,
    pub next: u64,
    grown: u64,
}

/// The game's last stand.
#[derive(Debug, Default)]
pub struct Stand {
    /// The heart, while it lives.
    pub live: Option<Heart>,
    /// It formed this planet (once a planet), and its lunges.
    pub done: bool,
    pub lunges: u32,
    /// The tick it died or broke, and the most it held.
    pub fell: Option<(u64, u32)>,
}

impl Stand {
    /// Whether lump `l` is the heart's.
    pub fn holds(&self, l: usize) -> bool {
        self.live.is_some_and(|h| h.lump == l)
    }
}

impl<W: World> Game<W> {
    /// Once a game second: gather the heart late in a planet against an
    /// army, and keep it, lunging, until it dies.
    pub(super) fn time_stand(&mut self) {
        let r = self.rules.stand;
        if !r.on || !self.world.can_sow() {
            return;
        }
        if self.stand.live.is_some() {
            self.keep_heart();
            return;
        }
        if self.stand.done || self.peak == 0 || self.mould == 0 || (self.mould as f32) >= r.at * self.peak as f32 {
            return;
        }
        if (self.soldiers.iter().filter(|s| s.alive).count() as f32) < r.army {
            return;
        }
        self.start_stand();
    }

    /// The heart gathering now on the heart's crystal; false with no free
    /// lump (a mini-boss, a march or a fruit ends in seconds, so play
    /// waits for one) or no slime on a crystal. Called by `time_stand`,
    /// and by the slime viewer whenever it is asked (`Game::force_stand`).
    pub(super) fn start_stand(&mut self) -> bool {
        let r = self.rules.stand;
        let Some(lump) = (0..LUMPS).find(|&l| !self.lump_taken(l)) else { return false };
        let Some(home) = self.heart_site() else { return false };
        self.world.grow_lump(lump, (home, self.whole_map()));
        let size = self.world.size();
        self.world.set_lump(lump, Lump { aim: super::raid_cell(home, size), pull: r.pull, leash: r.leash, armour: r.armour, bite: r.bite, ..Lump::default() });
        let now = self.world.ticks();
        let tps = self.rules.tps as f32;
        self.stand.live = Some(Heart { lump, home, born: now, members: 0, most: 0, centre: home, lunging: None, next: now + (r.every.max(1.0) * tps) as u64, grown: now });
        self.stand.done = true;
        true
    }

    /// A reach (dish cells) round any point that holds the whole map.
    fn whole_map(&self) -> f32 {
        let (w, h) = self.world.size();
        (w.max(h) as f32 * 1.5) / self.world.scale().max(1.0)
    }

    fn keep_heart(&mut self) {
        let Some(mut h) = self.stand.live else { return };
        let r = self.rules.stand;
        let tps = self.rules.tps as f32;
        let now = self.world.ticks();
        let (centre, n) = self.world.lumps()[h.lump].map_or((h.centre, 0), |(x, y, n)| ((x, y), n));
        (h.centre, h.members) = (centre, n);
        h.most = h.most.max(n);
        // The lump's sums lag a step behind its forming (as the fruit's).
        let gone = if h.most == 0 { now > h.born + (3.0 * tps) as u64 } else { (n as f32) < r.last * h.most as f32 || now > h.born + (r.life.max(1.0) * tps) as u64 };
        if gone {
            self.world.form_lump(h.lump, None);
            self.stand.fell = Some((now, h.most));
            self.stand.live = None;
            return;
        }
        // It takes in the slime grown since (from crystals still eating).
        if now >= h.grown + (r.grow_every.max(0.5) * tps) as u64 {
            self.world.grow_lump(h.lump, (h.centre, self.whole_map()));
            h.grown = now;
        }
        let size = self.world.size();
        let rest = Lump { aim: super::raid_cell(h.home, size), pull: r.pull, leash: r.leash, armour: r.armour, bite: r.bite, ..Lump::default() };
        match h.lunging {
            Some(until) if now >= until => {
                self.world.set_lump(h.lump, rest);
                h.lunging = None;
                h.next = now + (r.every.max(1.0) * tps) as u64;
            }
            Some(_) => self.aim_heart(&h),
            None if now >= h.next && self.soldiers.iter().any(|s| s.alive) => {
                h.lunging = Some(now + (r.lunge.max(1.0) * tps) as u64);
                self.stand.lunges += 1;
                self.aim_heart(&h);
            }
            None => {}
        }
        self.stand.live = Some(h);
    }

    /// The lunge: the heart's aim on the soldier nearest its centre.
    fn aim_heart(&mut self, h: &Heart) {
        let r = self.rules.stand;
        let near = self.soldiers.iter().filter(|s| s.alive).min_by(|a, b| (a.x - h.centre.0).hypot(a.y - h.centre.1).total_cmp(&(b.x - h.centre.0).hypot(b.y - h.centre.1)));
        let Some(s) = near else { return };
        let size = self.world.size();
        self.world.set_lump(h.lump, Lump { aim: super::raid_cell((s.x, s.y), size), pull: r.pull, leash: r.lunge_leash, haste: r.haste, armour: r.armour, bite: r.bite, ..Lump::default() });
    }

    /// What the view draws of the heart: its lump, centre (fine cells) and
    /// strength 0-1, its members against the most it held
    /// (`LumpLook::Heart`).
    pub fn heart_looks(&self) -> Option<(usize, (f32, f32), f32)> {
        self.stand.live.map(|h| (h.lump, h.centre, if h.most > 0 { h.members as f32 / h.most as f32 } else { 1.0 }))
    }
}
