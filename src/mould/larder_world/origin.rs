//! **Crystal hearts: a mini-boss that grows on a crystal the slime holds
//! thick** (2026-11-26, the owner's ruling of 11-26 00:08Z: "slime hearts
//! stay, but they are not the win condition. Basically anytime a slime
//! maintains a density of slime on a crystal for a 2 minutes a heart form.
//! It is basically just another mini-boss/way to make it harder to capture
//! core slime crystals.").
//!
//! Until then this was **the origin** (2026-11-22, Milestone 2's win; the
//! design in `Reports/origin-design-2026-11-22/README.md`): one heart
//! formed at the landing on the start crystal (`Game::core`), and breaking
//! it stopped births, starved the slime and won the planet after a
//! ten-second die-back. The owner, 11-25 23:57Z: "the Slime heart makes
//! the game even easier" (a snipe ended a planet in a minute or two); the
//! planet is won again at its last walker (`check_end`, `run::WIN_SHARE`).
//!
//! A heart is a lump of the slime's own armoured walkers (`stand.rs`'s
//! `Heart`, the last stand's machinery, drawn with its look):
//!
//! - **It forms where the slime holds a crystal thick.** Once a second each
//!   charged crystal's slime (walkers within its rim) is weighed against
//!   its disc: at `dense` walkers a fine cell or more (and at least
//!   `2 x floor_min` walkers) its clock runs, under it the clock falls
//!   back three seconds a second, so a surge's flicker does not reset it
//!   but thinning it out with fire does. At `after` seconds the heart
//!   forms on that crystal. One lives at a time (the four lumps are shared
//!   with mini-bosses, fruit and marches); another crystal's clock waits
//!   full and forms its heart when the first one breaks.
//! - **It grows from its crystal.** Every `grow_every` seconds it takes in
//!   the free walkers round its crystal (a reach that widens from the
//!   crystal's middle until it holds its planet's `size`), up to `most`
//!   times that, but only while its crystal has charge, so draining or
//!   pumping the crystal stops it regrowing.
//! - **It fights back.** It lunges at a soldier within `near` of its
//!   crystal (the heart's bout, with its cap on the health one bout may
//!   take), and when it loses members to shots it calls the body home: a
//!   raid on the soldiers striking it, at most every `call_every` seconds.
//! - **It breaks**, cut under `floor` of the most it held (or `floor_min`
//!   members): its lump lets go and what is left is ordinary slime again,
//!   and its crystal's clock starts from nothing. Nothing else happens: the
//!   planet is not won and the slime does not starve, so a crystal is
//!   taken by breaking its heart and then thinning or draining it before
//!   two more minutes are up.
//! - **The late last stand gathers into a live heart** (`time_stand`):
//!   late in a planet against an army, free walkers stream to it instead
//!   of forming a second heart.
//!
//! GPU only (the owner's 11-17 card): a world that cannot sow never forms
//! one.

use super::{Game, World};
use crate::mould::big::{Lump, LUMPS};

/// The crystal heart's numbers (the key prefix `origin_` is kept from the
/// origin). Times in game seconds, distances in dish cells.
/// Off by default (the campaign and the sandbox are frozen); a run turns
/// it on (`run::ENEMY_RULES`) and sets its size by planet
/// (`run::ORIGIN_SIZE`). Steering is the last-stand heart's (`StandRules`).
#[derive(Clone, Copy, Debug, PartialEq)]
#[derive(serde::Serialize, serde::Deserialize)]
pub struct OriginRules {
    pub on: bool,
    /// A heart forms on a charged crystal whose slime has held `dense`
    /// walkers a fine cell of its disc for `after` seconds (the owner's
    /// "a 2 minutes"). Free walkers stand about one a fine cell at most
    /// (the crowd limit): the start crystal reads 0.95 to 0.98 from the
    /// landing on (planet 1, run seed 1, lavapipe), so 0.85 is a crystal
    /// the slime has packed, and fire that holes it stops the clock.
    pub dense: f32,
    pub after: f32,
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
    /// The late last stand (`StandRules`, from planet 4) gathers free
    /// walkers into it only until it holds this times its size: each take
    /// is the widest disc round its centre holding no more slime than that
    /// (`gather_disc`). Until 11-24 it took every free walker on the map at
    /// once: Engine's siege on Scott's maps (GTX 1070, 720 s) met planet
    /// 7's origin at 79,940 and 66,426 members against a size of 4,200, the
    /// whole body, which no squad wore down to its floor (one open at
    /// 12:00, one lost), and planet 8's at 3,285 against 2,400. A first cap
    /// the same day (#756), a disc widening 3 dish cells a take cut to the
    /// square root of its shortfall, still overshot: on planet 7 map C it
    /// held 13,699 against 2,100 (the disc kept widening over ground the
    /// slime had left until it reached a packed body, and took it whole).
    pub gather_most: f32,
}

impl Default for OriginRules {
    fn default() -> Self {
        OriginRules { on: false, dense: 0.85, after: 120.0, size: 400.0, most: 1.0, grow_every: 2.0, widen: 0.5, floor: 0.1, floor_min: 50.0, near: 30.0, call_loss: 0.02, call_every: 20.0, armour: 12, bite: 2, every: 12.0, lunge: 8.0, cap: 0.4, cap_most: 20.0, lunge_leash: 2.5, quiet: 4.0, hurt: 0.02, gather_most: 2.0 }
    }
}

/// The crystal hearts' state: the live one's (`Stand::live` with
/// `origin`), and each crystal's clock.
#[derive(Clone, Debug, Default)]
#[derive(serde::Serialize, serde::Deserialize)]
pub struct OriginState {
    /// Hearts formed and broken this planet.
    pub formed: u32,
    pub broken: u32,
    /// The live heart's crystal (fine cells) and the tick it formed; the
    /// tick the last one broke.
    pub home: (f32, f32),
    pub born: Option<u64>,
    pub fell: Option<u64>,
    /// Each charged crystal's seconds held thick (`OriginRules::dense`),
    /// by its middle (fine cells).
    pub thick: Vec<((f32, f32), f32)>,
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
    /// Once a game second, before the last stand: each charged crystal's
    /// clock runs while the slime holds it thick, and a heart forms on the
    /// crystal whose clock is full first, when no heart lives.
    pub(super) fn time_origin(&mut self) {
        let r = self.rules.origin;
        if !r.on || !self.world.can_sow() {
            return;
        }
        let Some(counts) = self.crystal_walkers() else { return };
        let charged: Vec<f32> = self.world.crystals().iter().map(|c| c.1).collect();
        let sc = self.world.scale();
        let live = self.stand.live.is_some_and(|h| h.origin).then_some(self.origin.home);
        let was = std::mem::take(&mut self.origin.thick);
        let full = r.after.max(1.0);
        for (c, &(at, n, rim)) in counts.iter().enumerate() {
            let held = was.iter().find(|w| (w.0 .0 - at.0).hypot(w.0 .1 - at.1) < sc).map_or(0.0, |w| w.1);
            // The live heart's own crystal holds no clock: it has its heart.
            let thick = charged.get(c).is_some_and(|&q| q > 0.0) && n as f32 >= 2.0 * r.floor_min && n as f32 >= r.dense * std::f32::consts::PI * rim * rim && live.is_none_or(|h| (h.0 - at.0).hypot(h.1 - at.1) >= sc);
            // Thinned, the clock falls back three seconds a second, so a
            // surge's flicker does not reset it but fire that keeps it
            // thin does.
            let t = if thick { (held + 1.0).min(full) } else { (held - 3.0).max(0.0) };
            if t > 0.0 {
                self.origin.thick.push((at, t));
            }
        }
        // Any other heart (the late last stand's) holds the slot too.
        if self.stand.live.is_some() || self.mould == 0 {
            return;
        }
        // Of full clocks, the crystal holding the most slime.
        let walkers = |at: (f32, f32)| counts.iter().find(|c| (c.0 .0 - at.0).hypot(c.0 .1 - at.1) < sc).map_or(0, |c| c.1);
        let ready = self.origin.thick.iter().filter(|w| w.1 >= full).map(|w| w.0).max_by_key(|&at| walkers(at));
        if let Some(home) = ready {
            self.start_origin(home);
        }
    }

    /// The radius (dish cells) of the smallest disc round `at` (fine cells)
    /// whose slime, members and free walkers both, comes to more than
    /// `most` (the whole slime's, if it holds less), from the mould grid's
    /// squares a dish cell wide, ring by ring; 0 with no grid. One pass over
    /// the grid, every `grow_every` while gathering.
    fn gather_disc(&self, at: (f32, f32), most: f32) -> f32 {
        let bin = (self.world.scale() as usize).max(1);
        let Some(g) = self.world.mould_grid(bin) else { return 0.0 };
        let gw = self.world.size().0.div_ceil(bin);
        let b = bin as f32;
        let mut rings: Vec<u64> = Vec::new();
        for (i, &n) in g.iter().enumerate().filter(|(_, &n)| n > 0) {
            let d = (((i % gw) as f32 + 0.5) * b - at.0).hypot(((i / gw) as f32 + 0.5) * b - at.1) / b;
            let k = d as usize;
            if rings.len() <= k {
                rings.resize(k + 1, 0);
            }
            rings[k] += n as u64;
        }
        let mut held = 0u64;
        for (k, &n) in rings.iter().enumerate() {
            held += n;
            if held as f32 > most {
                return (k + 1) as f32;
            }
        }
        rings.len() as f32
    }

    /// A heart gathering now round the crystal at `home`; false with no
    /// free lump (a mini-boss, a march or a fruit ends in seconds, so its
    /// clock waits full for one).
    pub(super) fn start_origin(&mut self, home: (f32, f32)) -> bool {
        let mut r = self.rules.stand;
        (r.armour, r.bite, r.every) = (self.rules.origin.armour, self.rules.origin.bite, self.rules.origin.every);
        let Some(lump) = (0..LUMPS).find(|&l| !self.lump_taken(l)) else { return false };
        let reach = self.rules.origin.widen.max(0.5);
        self.world.grow_lump(lump, (home, reach));
        let size = self.world.size();
        self.world.set_lump(lump, Lump { aim: super::raid_cell(home, size), pull: r.pull, leash: r.leash, armour: r.armour, bite: r.bite, ..Lump::default() });
        let now = self.world.ticks();
        let tps = self.rules.tps as f32;
        let heart = super::Heart::origin(lump, home, now, now + (r.every.max(1.0) * tps) as u64);
        self.stand.live = Some(heart);
        let sc = self.world.scale();
        let thick = std::mem::take(&mut self.origin.thick).into_iter().filter(|w| (w.0 .0 - home.0).hypot(w.0 .1 - home.1) >= sc).collect();
        self.origin = OriginState { formed: self.origin.formed + 1, broken: self.origin.broken, home, born: Some(now), fell: self.origin.fell, thick, reach, grown: now, ..OriginState::default() };
        true
    }

    /// Whether the heart's crystal still has charge (only the squad
    /// empties one).
    fn origin_fed(&self) -> bool {
        let home = self.origin.home;
        let sc = self.world.scale();
        self.world.crystals().iter().filter(|c| (c.0 .0 - home.0).hypot(c.0 .1 - home.1) < sc).any(|c| c.1 > 0.0)
    }

    /// Once a second from `keep_heart` while the heart is a crystal heart:
    /// its regrowth from its crystal and its call home. Returns whether it
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
                // Only up to `gather_most` sizes, so it never swallows the
                // body.
                let most = r.gather_most.max(1.0) * r.size.max(1.0);
                if (h.members as f32) < most {
                    // The walkers round its crystal: the smallest disc
                    // there holding as much slime as `most` (members
                    // and free alike), and of those in it no more than its
                    // shortfall join (`grow_lump_most`). A disc counted to
                    // hold just the shortfall still overshot (the test: 446
                    // and 570 against 300), as the members do not stand in
                    // one disc: the lump's centre wandered 36 cells from the
                    // crystal in 2 s and they spread round it.
                    let take = self.gather_disc(self.origin.home, most).max(0.5);
                    self.origin.reach = take;
                    let room = (most - h.members as f32).max(0.0) as u32;
                    self.world.grow_lump_most(h.lump, (self.origin.home, take), room);
                }
            } else if self.origin_fed() && (h.members as f32) < r.most * r.size {
                let cr = self.crystal_at(self.origin.home).map_or(4.0 * self.world.scale(), |(c, _)| self.world.crystal_r(c)) / self.world.scale().max(1.0);
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
                self.world.grow_lump(h.lump, (self.origin.home, take));
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

    /// The live soldier nearest the heart's crystal within its reach
    /// (`OriginRules::near` past the crystal's rim).
    pub(super) fn origin_near(&self) -> Option<usize> {
        let sc = self.world.scale();
        let home = self.origin.home;
        let cr = self.crystal_at(home).map_or(0.0, |(c, _)| self.world.crystal_r(c));
        let reach = cr + self.rules.origin.near * sc;
        (0..self.soldiers.len()).filter(|&k| self.soldiers[k].alive).map(|k| (k, (self.soldiers[k].x - home.0).hypot(self.soldiers[k].y - home.1))).filter(|&(_, d)| d <= reach).min_by(|a, b| a.1.total_cmp(&b.1)).map(|(k, _)| k)
    }

    /// Whether soldier `k` stands within the origin's reach (`origin_near`'s
    /// test for one soldier).
    pub(super) fn origin_reaches(&self, k: usize) -> bool {
        let sc = self.world.scale();
        let home = self.origin.home;
        let cr = self.crystal_at(home).map_or(0.0, |(c, _)| self.world.crystal_r(c));
        let s = self.soldiers[k];
        s.alive && (s.x - home.0).hypot(s.y - home.1) <= cr + self.rules.origin.near * sc
    }

    /// The heart breaks: its lump lets go and what is left of it is
    /// ordinary slime again; its crystal's clock starts from nothing
    /// (`time_origin` keeps no clock on a live heart's crystal).
    fn break_origin(&mut self, lump: usize) {
        self.world.form_lump(lump, None);
        self.origin.fell = Some(self.world.ticks());
        self.origin.broken += 1;
        self.origin.born = None;
        self.stand.live = None;
    }

    /// What the HUD shows of the live crystal heart: its members against
    /// the most it held.
    pub fn origin_hud(&self) -> Option<(u32, u32)> {
        self.stand.live.filter(|h| h.origin).map(|h| (h.members, h.most))
    }
}
