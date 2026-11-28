//! **The rescue** (2026-11-28, Enemy; Direction's call 11-24, moved to the
//! star map 11-27 23:41Z; a contract by the owner's word, 11-28 03:38Z:
//! "The rescue is one of the optional objectives/bonuses that we already
//! have in the star map"). The Lab built and timed it on its own
//! branch (`Reports/lab-rescue-2026-11-24/`, code on
//! `claude/project-thread-2o4cic`): every planet was the same job, and this
//! one asks for a timed push into the body and a slow escort home, against
//! a slime that turns on the march.
//!
//! In about one run in three, one planet at the step that meets the Trunk
//! (`route::RESCUE_STEP`) carries the rescue contract
//! (`route::Contract::Rescue`) on its card; the run turns `Rules::rescue`
//! on there (`run::Run::rescue`). The planet is won as any other, by wiping
//! out the slime; the rescue pays the contract. At the first game
//! second two squads of three of yours are stranded deep in the slime:
//!
//! - **Picked from the ground.** The Lab placed them by hand on run seed 1;
//!   a star-map planet is any world, so each dug-out is the thickest slime
//!   whose ring lies on open ground a soldier can walk to from the landing
//!   and clear of every crystal (`DropGround::fits`, the starfall's test),
//!   at least 40% of the farthest such spot from the larder, and the second
//!   at least 30% of it from the first.
//! - **Hidden.** The slime cannot smell them (smell 0, the toolkit's mask;
//!   a heart does not lunge at a soldier it cannot smell, `origin_near`)
//!   and they fire only at slime that comes near them (Defend). They take
//!   no orders until reached (`Game::rescue_holds`).
//! - **Dug in.** A ring of eight doors `ring` dish cells round them, raised
//!   whole: your soldiers walk through, the slime cannot and gnaws them as
//!   any wall. The slime inside is killed for the first 3 s (the walls' own
//!   evict pushes some inward).
//! - **Found** at `hide` (5:00), or the moment one of yours comes within
//!   `reach` cells: they reek (smell `out`) and fire freely. Reached, they
//!   join that soldier's squad, walk at `wound` of a soldier's pace
//!   (`Soldier::pace`, so their squad keeps to it), and the slime's
//!   strikers turn on against the march whatever its size (`alarm`).
//! - **Home** when one of a dug-out stands within `home` cells of the
//!   larder. With every dug-out home the contract is kept
//!   (`Game::rescued`), paid when the planet is won.
//! - **A dug-out wiped** ends the rescue (`Rescue::failed`): no contract.
//!   The Lab's rule was a lost planet (in a run, the run's end) and its win
//!   was the rescue; the owner's card (11-28 03:38Z) first chose "goes on,
//!   with a penalty", and a minute later he made it a contract, which is
//!   that: the penalty is the contract's pay not earned.
//!
//! The Lab's planet 3 numbers (GTX 1070, run seed 1, two dug-outs): idle
//! and greedy lost at 5:06 and 5:25 as the rings fell; a rescue from 1:00
//! won at 3:57, from 4:00 at 6:23. Tried and changed on the way (planet 1):
//! no hiding and free fire died in 28 s; hold fire inside a wall ring
//! walled slime in with them; plain walls held them in once reached (so
//! doors); walls thickened inward filled the ring (so outward); without the
//! wound the march home took 14 s.
//!
//! GPU only in play; the rule itself is plain game code (bursts, walls,
//! smell), so it runs on the CPU engine too. Once a game second.

use super::{Game, Kind, StrikerRules, World};
use crate::mould::larder::Stance;

/// Kill bursts a dug-out's clearing takes (`Game::rescue_clear`): more
/// than the most blows a walker of any level takes (`levels.rs`).
const CLEAR_BLOWS: u32 = 16;

/// A dug-out ring's cell health at `thick` 1: the 400 the Lab timed them
/// at, whatever the planet's player walls hold (`Rules::wall_hp`).
const RING_HP: f32 = 400.0;

/// The rescue's numbers (`Rules::rescue`). Off here; the run turns it on
/// for a Rescue node.
#[derive(Clone, Copy, Debug, PartialEq)]
#[derive(serde::Serialize, serde::Deserialize)]
pub struct RescueRules {
    pub on: bool,
    /// Dug-outs, and soldiers in each.
    pub sites: u32,
    pub n: u32,
    /// Game seconds they stay hidden unless reached first.
    pub hide: f32,
    /// Their smell once found (the toolkit's reek is 3).
    pub out: f32,
    /// 0 off; 1 the strikers forced on for good once one is reached; more
    /// than 1, forced for that many seconds and then the planet's own.
    pub alarm: f32,
    /// The ring's radius, dish cells, and its wall levels (1 to
    /// `WALL_LEVELS`).
    pub ring: f32,
    pub thick: u32,
    /// The found's walking pace, 0..1.
    pub wound: f32,
    /// Dish cells: a soldier this near reaches them; one of them this near
    /// the larder is home.
    pub reach: f32,
    pub home: f32,
}

impl Default for RescueRules {
    fn default() -> Self {
        RescueRules { on: false, sites: 2, n: 3, hide: 300.0, out: 3.0, alarm: 1.0, ring: 7.0, thick: 1, wound: 0.3, reach: 8.0, home: 14.0 }
    }
}

/// One dug-out.
#[derive(Clone, Debug, PartialEq)]
#[derive(serde::Serialize, serde::Deserialize)]
pub struct Site {
    /// Fine cells.
    pub at: (f32, f32),
    pub members: Vec<usize>,
    /// Game seconds: found (reached, or the hiding ran out), reached by a
    /// soldier of yours, and one of them stood at the larder.
    pub found: Option<f32>,
    pub reached: Option<f32>,
    pub home: Option<f32>,
    /// Alive at the last second, for the log.
    pub left: usize,
}

/// The rescue under way (`Game::rescue`).
#[derive(Clone, Debug, Default)]
#[derive(serde::Serialize, serde::Deserialize)]
pub struct Rescue {
    pub sites: Vec<Site>,
    /// The game second they were dug in.
    pub since: f32,
    /// A timed alarm: the second it ends and the planet's own strikers.
    pub calm: Option<(f32, StrikerRules)>,
    /// The game second a dug-out was wiped and the rescue ended.
    pub failed: Option<f32>,
    /// The game second every dug-out was home (the contract kept).
    pub done: Option<f32>,
    /// Seconds tried without finding ground (it gives up after ten).
    pub tries: u32,
}

impl Rescue {
    /// Soldier `k` is one of the stranded not yet reached (takes no orders).
    pub fn pinned(&self, k: usize) -> bool {
        self.failed.is_none() && self.sites.iter().any(|s| s.reached.is_none() && s.members.contains(&k))
    }

    /// The first dug-out not yet reached, if any.
    pub fn next(&self) -> Option<(f32, f32)> {
        self.sites.iter().find(|s| s.reached.is_none()).map(|s| s.at)
    }

    /// Dug-outs reached, and home, of all.
    pub fn counts(&self) -> (usize, usize, usize) {
        (self.sites.iter().filter(|s| s.reached.is_some()).count(), self.sites.iter().filter(|s| s.home.is_some()).count(), self.sites.len())
    }

    /// Under way: dug in, not failed, not done.
    pub fn live(&self) -> bool {
        !self.sites.is_empty() && self.failed.is_none() && self.done.is_none()
    }
}

impl<W: World> Game<W> {
    /// Every dug-out was brought home (`route::Contract::Rescue`).
    pub fn rescued(&self) -> bool {
        self.rescue.as_ref().is_some_and(|r| r.done.is_some())
    }

    /// Command `c` would order a stranded soldier not yet reached, or its
    /// squad: refused, they hold their dug-out.
    pub(super) fn rescue_holds(&self, c: &super::Cmd) -> bool {
        use super::Cmd;
        let Some(r) = self.rescue.as_ref().filter(|r| r.live()) else { return false };
        let squad = |q: u8| self.soldiers.iter().enumerate().any(|(k, s)| s.alive && s.squad == q && r.pinned(k));
        match *c {
            Cmd::Order(k, _) | Cmd::Queue(k, _) | Cmd::Stance(k, _) => r.pinned(k),
            Cmd::Join(k, q) => r.pinned(k) || squad(q),
            Cmd::Merge(a, b) | Cmd::Link(a, b) => squad(a) || squad(b),
            Cmd::Steer(q, _) | Cmd::SquadOrder(q, _) | Cmd::SquadQueue(q, _) | Cmd::Split(q, _) | Cmd::Formation(q, ..) | Cmd::Unlink(q) => squad(q),
            _ => false,
        }
    }

    /// Where the dug-outs go: the thickest slime on open, reachable ground
    /// clear of crystals, deep from the larder (see the module's head).
    /// Fine cells; fewer than asked (or none) where the ground allows no
    /// more.
    fn rescue_spots(&self, want: usize, ring: f32) -> Vec<(f32, f32)> {
        let sc = self.s();
        let bin = (sc as usize).max(1);
        let Some(grid) = self.world.mould_grid(bin) else { return Vec::new() };
        let (w, h) = self.world.size();
        let (gw, gh) = (w.div_ceil(bin), h.div_ceil(bin));
        if grid.len() < gw * gh {
            return Vec::new();
        }
        let ground = self.drop_ground();
        // Walkers within three rings' radius: how deep in the body.
        let span = (3.0 * ring / bin as f32).ceil() as i64;
        let thick = |p: (f32, f32)| -> u32 {
            let (cx, cy) = ((p.0 / bin as f32) as i64, (p.1 / bin as f32) as i64);
            let mut n = 0;
            for y in (cy - span).max(0)..=(cy + span).min(gh as i64 - 1) {
                for x in (cx - span).max(0)..=(cx + span).min(gw as i64 - 1) {
                    if (x - cx).pow(2) + (y - cy).pow(2) <= span * span {
                        n += grid[y as usize * gw + x as usize];
                    }
                }
            }
            n
        };
        let step = 4.0 * sc;
        let l = self.larder;
        let mut c: Vec<((f32, f32), f32, u32)> = Vec::new();
        let mut y = step / 2.0;
        while y < h as f32 {
            let mut x = step / 2.0;
            while x < w as f32 {
                let p = (x, y);
                if ground.fits(p, ring) {
                    let n = thick(p);
                    if n > 0 {
                        c.push((p, (p.0 - l.0).hypot(p.1 - l.1), n));
                    }
                }
                x += step;
            }
            y += step;
        }
        let far = c.iter().map(|e| e.1).fold(0.0, f32::max);
        let mut out: Vec<(f32, f32)> = Vec::new();
        for _ in 0..want {
            let apart = |p: (f32, f32), out: &[(f32, f32)]| out.iter().all(|o| (o.0 - p.0).hypot(o.1 - p.1) >= 0.3 * far);
            let best = c.iter().filter(|e| e.1 >= 0.4 * far && apart(e.0, &out)).max_by_key(|e| e.2).or_else(|| c.iter().filter(|e| apart(e.0, &out)).max_by_key(|e| e.2));
            match best {
                Some(e) => out.push(e.0),
                None => break,
            }
        }
        out
    }

    /// Dig one dug-out: `n` soldiers in a new squad behind a ring of doors
    /// `ring_cells` dish cells round `at`.
    fn rescue_dig(&mut self, at: (f32, f32), n: usize, ring_cells: f32, thick: u32) -> Option<Site> {
        let q = self.free_squad()?;
        let sc = self.s();
        // The slime inside the ring is cleared first (a kill burst, as a
        // shell's), or the walls close it in with them (Lab v3: two of
        // three dead by 0:03 inside a whole ring of slime).
        if ring_cells > 0.0 {
            let clear = super::squad::Burst { at: (at.0 / sc, at.1 / sc), r: ring_cells, kill: true, ..Default::default() };
            self.rescue_clear(&clear);
        }
        let mut members = Vec::new();
        for i in 0..n {
            let a = i as f32 * std::f32::consts::TAU / n as f32;
            self.add_soldier_at(at, a);
            let k = self.soldiers.len() - 1;
            // Close together in the middle, not wherever `add_soldier_at`
            // finds standing room (Lab v3 put one on rock outside the ring).
            let (x, y) = (at.0 + 1.5 * sc * a.cos(), at.1 + 1.5 * sc * a.sin());
            if self.standable(x, y) {
                let s = &mut self.soldiers[k];
                (s.x, s.y, s.was) = (x, y, (x, y));
            }
            let s = &mut self.soldiers[k];
            s.squad = q as u8;
            s.smell = 0.0;
            s.stance = Stance::Defend;
            members.push(k);
        }
        self.squads[q].dest = Some(at);
        let ring = ring_cells * sc;
        if ring > 0.0 {
            let p = |i: usize| {
                let a = i as f32 * std::f32::consts::TAU / 8.0 + std::f32::consts::TAU / 16.0;
                (at.0 + ring * a.cos(), at.1 + ring * a.sin())
            };
            for i in 0..8 {
                let (a, b) = (p(i), p(i + 1));
                let kind = Kind::Wall { to: b };
                let n0 = self.buildings.len();
                // Free: the price is put in the bank and taken back by the
                // build.
                let price = self.build_price(kind, a);
                self.bank += price;
                if self.build(kind, a) {
                    // Doors, not plain walls (`Building::door`): the slime
                    // can't cross them and bites them as walls, but soldiers
                    // walk through (Lab v4, plain walls: reached, they could
                    // not walk out).
                    self.buildings[n0].door = true;
                    // Thickened outward, never toward the ring's middle
                    // (Lab: inward, the bands filled the ring and evicted
                    // the slime in them onto the soldiers, dead by 0:07).
                    let m = ((a.0 + b.0) / 2.0 - at.0, (a.1 + b.1) / 2.0 - at.1);
                    let l = m.0.hypot(m.1).max(1e-6);
                    self.buildings[n0].home = (m.0 / l, m.1 / l);
                    self.raise(n0, 1.0);
                    // Its own health, not the planet's (`Rules::wall_hp`,
                    // `run::WALL_HP`, a quarter of this and less deeper
                    // since 2026-11-28): the Lab timed the rings at 4 to 5
                    // minutes at 400 a cell, and Enemy keeps them there.
                    // `thick` levels harden it as the player's upgrade
                    // does (until 11-28 they added bands outward).
                    let lv = thick.clamp(1, u32::from(super::WALL_LEVELS)) as f32;
                    self.harden_to(n0, RING_HP * (1.0 + self.rules.wall_harden * (lv - 1.0)));
                    self.buildings[n0].level = lv as u8;
                    let wl = &mut self.buildings[n0];
                    wl.hp = wl.max_hp;
                } else {
                    self.bank -= price;
                }
            }
        }
        Some(Site { at, members, found: None, reached: None, home: None, left: n })
    }

    /// A dug-out's slime killed: the burst `CLEAR_BLOWS` times over, since
    /// a kill burst is one blow and the slime takes several by planet
    /// (`run::blows`, levels, #862/#864). One burst (the Lab's, before
    /// those) on step 3 of run seed 3 left the first dug-out's three dead
    /// at 0:03, in slime the burst had only wounded.
    fn rescue_clear(&mut self, b: &super::squad::Burst) {
        for _ in 0..CLEAR_BLOWS {
            self.world.burst(b, &super::squad::BurstRules::default());
        }
    }

    /// Once a game second.
    pub(super) fn time_rescue(&mut self) {
        let r = self.rules.rescue;
        if !r.on || self.end.is_some() {
            return;
        }
        let t = self.time();
        let Some(mut res) = self.rescue.take() else {
            if t < 1.0 {
                return;
            }
            let spots = self.rescue_spots(r.sites.max(1) as usize, r.ring.max(1.0) * self.s());
            let sites: Vec<Site> = spots.into_iter().filter_map(|at| self.rescue_dig(at, r.n.max(1) as usize, r.ring.max(0.0), r.thick)).collect();
            self.rescue = Some(Rescue { sites, since: t, tries: 1, ..Rescue::default() });
            return;
        };
        if res.sites.is_empty() {
            // No ground yet (the slime's grid not read back): try again
            // for ten seconds, then it is an ordinary planet.
            if res.tries < 10 {
                let spots = self.rescue_spots(r.sites.max(1) as usize, r.ring.max(1.0) * self.s());
                res.sites = spots.into_iter().filter_map(|at| self.rescue_dig(at, r.n.max(1) as usize, r.ring.max(0.0), r.thick)).collect();
                res.since = t;
                res.tries += 1;
            }
            self.rescue = Some(res);
            return;
        }
        if !res.live() {
            self.time_rescue_calm(&mut res, t);
            self.rescue = Some(res);
            return;
        }
        let sc = self.s();
        // The walls' own evict (#739) pushes the slime they were built over
        // out of their cells, half of it inward: cleared again at first.
        if r.ring > 0.0 && t <= res.since + 3.0 {
            for s in &res.sites {
                let clear = super::squad::Burst { at: (s.at.0 / sc, s.at.1 / sc), r: (r.ring - self.rules.wall_width).max(1.0), kill: true, ..Default::default() };
                self.rescue_clear(&clear);
            }
        }
        let mut alarm = false;
        for i in 0..res.sites.len() {
            let mut s = res.sites[i].clone();
            if s.home.is_some() {
                continue;
            }
            let alive: Vec<usize> = s.members.iter().copied().filter(|&k| self.soldiers[k].alive).collect();
            s.left = alive.len();
            if alive.is_empty() {
                res.failed = Some(t);
                // The found are ordinary soldiers again.
                for site in &res.sites {
                    for &m in &site.members {
                        self.soldiers[m].pace = 1.0;
                    }
                }
                res.sites[i] = s;
                break;
            }
            let find = |g: &mut Self, s: &mut Site| {
                if s.found.is_none() {
                    s.found = Some(t);
                    for &m in &alive {
                        g.soldiers[m].smell = r.out;
                        g.soldiers[m].stance = Stance::Free;
                    }
                }
            };
            if s.reached.is_none() {
                if t >= r.hide {
                    find(self, &mut s);
                }
                // They hold where they were left, whatever their squad is
                // told (the Lab's greedy hand sent them out to the ring).
                let q = self.soldiers[alive[0]].squad as usize;
                self.squads[q].dest = Some(s.at);
                for &m in &alive {
                    let so = &mut self.soldiers[m];
                    let off = (so.x - s.at.0).hypot(so.y - s.at.1) > 2.5 * sc;
                    let there = matches!(so.order, super::Order::Move(x, y) if (x - s.at.0).abs() < 0.5 && (y - s.at.1).abs() < 0.5);
                    if off && !there {
                        so.order = super::Order::Move(s.at.0, s.at.1);
                        so.queued = 0;
                    } else if !off && !matches!(so.order, super::Order::Hold) {
                        so.order = super::Order::Hold;
                        so.queued = 0;
                    }
                }
                let reach = r.reach * sc;
                let by = (0..self.soldiers.len()).find(|&k| {
                    let so = &self.soldiers[k];
                    so.alive && !res.pinned(k) && alive.iter().any(|&m| (self.soldiers[m].x - so.x).hypot(self.soldiers[m].y - so.y) <= reach)
                });
                if let Some(k) = by {
                    let q = self.soldiers[k].squad;
                    for &m in &alive {
                        let so = &mut self.soldiers[m];
                        so.squad = q;
                        so.pace = r.wound.clamp(0.05, 1.0);
                        so.order = super::Order::Hold;
                        so.queued = 0;
                    }
                    find(self, &mut s);
                    alarm |= r.alarm > 0.0;
                    s.reached = Some(t);
                }
            } else {
                let l = self.larder;
                if alive.iter().any(|&m| (self.soldiers[m].x - l.0).hypot(self.soldiers[m].y - l.1) <= r.home * sc) {
                    s.home = Some(t);
                    for &m in &alive {
                        self.soldiers[m].pace = 1.0;
                    }
                }
            }
            res.sites[i] = s;
        }
        if alarm && res.failed.is_none() {
            let was = res.calm.map_or(self.rules.strikers, |c| c.1);
            if r.alarm > 1.0 {
                res.calm = Some((t + r.alarm, was));
            }
            self.rules.strikers = StrikerRules { on: true, below: 2.0, few: -1.0, full: 0.0, ..was };
        }
        self.time_rescue_calm(&mut res, t);
        if res.failed.is_none() && res.sites.iter().all(|s| s.home.is_some()) {
            res.done = Some(t);
        }
        self.rescue = Some(res);
    }

    /// A timed alarm's end: the planet's own strikers back.
    fn time_rescue_calm(&mut self, res: &mut Rescue, t: f32) {
        if let Some((end, was)) = res.calm {
            if t >= end {
                self.rules.strikers = was;
                res.calm = None;
            }
        }
    }

    /// The rescue's line for the HUD: what to do now and how long the
    /// rings are said to hold. None off a rescue planet or once it is over.
    pub fn rescue_now(&self) -> Option<String> {
        let r = self.rescue.as_ref()?;
        if r.sites.is_empty() {
            return None;
        }
        let t = self.time();
        if let Some(at) = r.done {
            return (t < at + 10.0).then(|| "Rescue done: the contract pays when you win".to_string());
        }
        if let Some(at) = r.failed {
            return (t < at + 10.0).then(|| "Rescue lost: a dug-out was wiped".to_string());
        }
        let (reached, home, all) = r.counts();
        let hide = self.rules.rescue.hide - self.time();
        Some(if reached < all {
            if hide > 0.0 {
                let n = hide.ceil() as u32;
                format!("Rescue: {reached} of {all} dug-outs reached; found in {}:{:02}", n / 60, n % 60)
            } else {
                format!("Rescue: {reached} of {all} dug-outs reached; the slime has found them")
            }
        } else {
            format!("Rescue: walk them home, {home} of {all} there")
        })
    }
}
