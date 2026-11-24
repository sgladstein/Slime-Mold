//! LAB (never merges): **the rescue planet** (Direction's brief, 2026-11-24
//! 17:24Z: every planet today is the same job, kill the heart; prototype one
//! with a different goal that comes from the model).
//!
//! `LAB_RESCUE=X,Y[,N[,HIDE[,OUT[,ALARM[,RING[,WOUND[,THICK]]]]]]]]`
//! strands a squad of N soldiers (3) at (X, Y), world units as `centre=`,
//! deep in the slime from the game's first second.
//!
//! - **Hidden.** The slime cannot smell them (the toolkit's mask, smell 0;
//!   the origin does not lunge at a soldier it cannot smell) and they fire
//!   only at slime that comes near them (Defend), for HIDE game seconds
//!   (300). They take no orders until reached (`Game::apply`).
//! - **Dug in.** A ring of eight doors RING (7) dish cells round them,
//!   raised whole and THICK (1) levels thick, outward: soldiers walk
//!   through, the slime cannot, and it gnaws them as any wall
//!   (`bite_walls`). The slime inside is killed for the first 3 s (the
//!   walls' own evict pushes some inward).
//! - **Found.** At HIDE, or the moment a soldier of yours comes within
//!   8 cells of one of them, they smell OUT (3) of a soldier (the
//!   toolkit's reek) and fire freely. Reached, they join that soldier's
//!   squad, and with ALARM (1) the slime's strikers turn on against the
//!   march home whatever its size (`StrikerRules`, as the viewer forces
//!   them). They are wounded: WOUND (0.3) of a soldier's pace, and a squad
//!   keeps to its slowest (`Game::pace`).
//! - **Won** when one from every dug-out has stood within 14 cells of the
//!   larder; **lost** when every soldier of any one dug-out is dead. The
//!   heart's win is dropped (Direction 19:37Z): the origin does not break,
//!   it still stands and bites, and only the rescue wins.
//!
//! Tried and changed on the way (planet 1, run seed 1, 11-24):
//! - v1, no hiding (smell 0.3, free fire), no ring, no alarm: dead in 28 s
//!   alone; the rescue hand reached them at 0:15 and was home at 0:31.
//! - Hold fire inside a wall ring: the ring closed slime in with them
//!   (two of three dead by 0:03); cleared first, the origin hunted them
//!   from 50 cells and killed them by 2:20; deep in the body they died in
//!   10 to 40 s to slime born or evicted inside. Defend fixed both.
//! - Plain walls: reached, they could not walk out (the last died in the
//!   ring at 3:06 with the squad home), so the ring is doors.
//! - Thickened toward the larder, the bands filled the ring (dead by 0:07).
//! - Without `wound` the march home took 14 s.

use super::{End, Game, Kind, World};

/// One dug-out.
#[derive(Clone, Debug)]
pub struct Site {
    pub at: (f32, f32),
    pub members: Vec<usize>,
    /// The game second they were found (reached, or HIDE ran out).
    pub found: Option<f32>,
    /// The game second a soldier of yours reached them.
    pub reached: Option<f32>,
    /// The game second one of them stood at the larder.
    pub home: Option<f32>,
    /// How many were alive at the last second, for the log.
    pub left: usize,
}

#[derive(Clone, Debug)]
pub struct LabRescue {
    pub sites: Vec<Site>,
    pub hide: f32,
    pub alarm: bool,
    pub out: f32,
    /// The found stranded's walking pace (`Game::wade`), 0..1.
    pub wound: f32,
    /// The ring's radius, dish cells (0 none), and the second it was dug.
    pub ring: f32,
    pub since: f32,
}

impl LabRescue {
    /// Soldier `k` is one of the stranded not yet reached (no orders).
    pub fn pinned(&self, k: usize) -> bool {
        self.sites.iter().any(|s| s.reached.is_none() && s.members.contains(&k))
    }
    /// Soldier `k` is one of the stranded, reached (wounded).
    pub fn carried(&self, k: usize) -> bool {
        self.sites.iter().any(|s| s.reached.is_some() && s.members.contains(&k))
    }
    /// The first dug-out not yet reached, if any.
    pub fn next(&self) -> Option<(f32, f32)> {
        self.sites.iter().find(|s| s.reached.is_none()).map(|s| s.at)
    }
}

impl<W: World> Game<W> {
    /// Soldier `k`'s pace as one of the found stranded (`LabRescue::wound`), else 1.
    pub(super) fn lab_wound(&self, k: usize) -> f32 {
        match &self.lab_rescue {
            Some(r) if r.carried(k) => r.wound,
            _ => 1.0,
        }
    }

    /// A rescue planet (the origin does not break: only the rescue wins).
    pub(super) fn lab_rescue_planet(&self) -> bool {
        self.lab_rescue.is_some()
    }

    /// Dig one dug-out: N soldiers in a new squad behind a ring of doors.
    fn lab_dig(&mut self, at: (f32, f32), n: usize, ring_cells: f32, thick: u8) -> Option<Site> {
        let q = self.free_squad()?;
        let sc = self.s();
        // Their dug-out: the slime inside the ring is cleared first (a
        // kill burst, as a shell's), or the walls close it in with them
        // (v3, planet 1 at 110,215: two of three dead by 0:03 inside
        // a whole ring of slime).
        if ring_cells > 0.0 {
            let clear = super::squad::Burst { at: (at.0 / sc, at.1 / sc), r: ring_cells, kill: true, ..Default::default() };
            self.world.burst(&clear, &super::squad::BurstRules::default());
        }
        let mut members = Vec::new();
        for i in 0..n {
            self.add_soldier_at(at, i as f32 * std::f32::consts::TAU / n as f32);
            let k = self.soldiers.len() - 1;
            // Close together in the middle, not wherever `add_soldier_at`
            // finds standing room (v3 put one on the rock outside the ring).
            let a = i as f32 * std::f32::consts::TAU / n as f32;
            let (x, y) = (at.0 + 1.5 * sc * a.cos(), at.1 + 1.5 * sc * a.sin());
            if self.standable(x, y) {
                self.soldiers[k].x = x;
                self.soldiers[k].y = y;
                self.soldiers[k].was = (x, y);
            }
            self.soldiers[k].squad = q as u8;
            self.soldiers[k].smell = 0.0;
            self.soldiers[k].stance = crate::mould::larder::Stance::Defend;
            members.push(k);
        }
        self.squads[q].dest = Some(at);
        // Dug in: an octagon of doors, raised whole (as `stand` in the
        // window's headless runs places a building).
        let ring = ring_cells * sc;
        if ring > 0.0 {
            let p = |i: usize| {
                let a = i as f32 * std::f32::consts::TAU / 8.0 + std::f32::consts::TAU / 16.0;
                (at.0 + ring * a.cos(), at.1 + ring * a.sin())
            };
            let mut up = 0;
            for i in 0..8 {
                let (a, b) = (p(i), p(i + 1));
                let kind = Kind::Wall { to: b };
                let n0 = self.buildings.len();
                self.bank += self.build_price(kind, a);
                if self.build(kind, a) {
                    // Doors, not plain walls (`Building::door`): the slime
                    // can't cross them and bites them as walls, but
                    // soldiers walk through. As plain walls (v4) the
                    // rescuers reached them and the found walked nowhere:
                    // the ring held them in, and the last one died in it
                    // at 3:06 with the squad home.
                    self.buildings[n0].door = true;
                    // Thickened outward, not on the side toward the larder
                    // (`wall_home`): thickened inward, the bands filled the
                    // ring and evicted the slime in them onto the soldiers
                    // (dead by 0:07 at 40,150).
                    let m = ((a.0 + b.0) / 2.0 - at.0, (a.1 + b.1) / 2.0 - at.1);
                    let l = m.0.hypot(m.1).max(1e-6);
                    self.buildings[n0].home = (m.0 / l, m.1 / l);
                    self.raise(n0, 1.0);
                    // THICK levels in all (`Game::thicken`, the walls' own
                    // upgrade), so how long it holds is a dial.
                    for _ in 1..thick {
                        self.thicken(n0);
                    }
                    let w = &mut self.buildings[n0];
                    w.hp = w.max_hp;
                    up += 1;
                } else {
                    self.bank -= self.build_price(kind, a);
                    eprintln!("  lab rescue: wall {i} not built: {}", self.build_blocked(kind, a).unwrap_or("blocked"));
                }
            }
            eprintln!("  lab rescue: {up} of 8 walls round {:.0},{:.0}", at.0, at.1);
        }
        eprintln!("  lab rescue t {:.0} s: {n} soldiers stranded at {:.0},{:.0} in squad {q}", self.time(), at.0, at.1);
        Some(Site { at, members, found: None, reached: None, home: None, left: n })
    }

    /// Once a game second.
    pub(super) fn time_lab_rescue(&mut self) {
        if self.end.is_some() {
            return;
        }
        let t = self.time();
        let Some(mut r) = self.lab_rescue.take() else {
            let Ok(v) = std::env::var("LAB_RESCUE") else { return };
            if t < 1.0 {
                return;
            }
            let f: Vec<f32> = v.split(',').filter_map(|x| x.parse().ok()).collect();
            if f.len() < 2 {
                return;
            }
            let n = f.get(2).map_or(3, |&n| n.max(1.0) as usize);
            let (hide, out, alarm) = (f.get(3).copied().unwrap_or(300.0), f.get(4).copied().unwrap_or(3.0), f.get(5).copied().unwrap_or(1.0) > 0.0);
            let ring_cells = f.get(6).copied().unwrap_or(7.0);
            let thick = (f.get(8).copied().unwrap_or(1.0) as u8).clamp(1, super::doors::WALL_LEVELS);
            let mut spots = vec![(f[0], f[1])];
            if let Ok(m) = std::env::var("LAB_RESCUE_MORE") {
                for s in m.split(';') {
                    let p: Vec<f32> = s.split(',').filter_map(|x| x.parse().ok()).collect();
                    if p.len() >= 2 {
                        spots.push((p[0], p[1]));
                    }
                }
            }
            let sites: Vec<Site> = spots.into_iter().filter_map(|at| self.lab_dig(at, n, ring_cells, thick)).collect();
            if sites.is_empty() {
                return;
            }
            self.lab_rescue = Some(LabRescue { sites, hide, alarm, out, wound: f.get(7).copied().unwrap_or(0.3).clamp(0.05, 1.0), ring: ring_cells.max(0.0), since: t });
            return;
        };
        let sc = self.world.scale();
        // The walls' own evict (#739) pushes the slime they were built over
        // out of their cells, half of it inward: cleared again for the
        // first seconds.
        if r.ring > 0.0 && t <= r.since + 3.0 {
            for s in &r.sites {
                let clear = super::squad::Burst { at: (s.at.0 / sc, s.at.1 / sc), r: (r.ring - self.rules.wall_width).max(1.0), kill: true, ..Default::default() };
                self.world.burst(&clear, &super::squad::BurstRules::default());
            }
        }
        let mut alarm_now = false;
        for i in 0..r.sites.len() {
            let mut s = r.sites[i].clone();
            if s.home.is_some() {
                continue;
            }
            let alive: Vec<usize> = s.members.iter().copied().filter(|&k| self.soldiers[k].alive).collect();
            if alive.len() < s.left {
                let near = |m: usize| (0..self.soldiers.len()).filter(|&k| self.soldiers[k].alive && !s.members.contains(&k)).map(|k| (self.soldiers[k].x - self.soldiers[m].x).hypot(self.soldiers[k].y - self.soldiers[m].y)).fold(f32::INFINITY, f32::min);
                let dead: Vec<String> = s.members.iter().filter(|&&m| !self.soldiers[m].alive).map(|&m| format!("{:.0},{:.0} ({:.0} from help)", self.soldiers[m].x, self.soldiers[m].y, near(m))).collect();
                eprintln!("  lab rescue t {t:.0} s: dug-out {i}: {} of {} alive; dead at {}", alive.len(), s.members.len(), dead.join(", "));
                s.left = alive.len();
            }
            if alive.is_empty() {
                eprintln!("  lab rescue t {t:.0} s: lost, every soldier of dug-out {i} died{}", if s.reached.is_some() { " on the way home" } else { "" });
                self.end = Some(End::SquadWiped(t));
                r.sites[i] = s;
                self.lab_rescue = Some(r);
                return;
            }
            // Found: they reek and fight for their lives.
            let find = |g: &mut Self, s: &mut Site, why: &str| {
                if s.found.is_none() {
                    s.found = Some(t);
                    for &m in &alive {
                        g.soldiers[m].smell = r.out;
                        g.soldiers[m].stance = crate::mould::larder::Stance::Free;
                    }
                    eprintln!("  lab rescue t {t:.0} s: dug-out {i} found ({why}), {} alive", alive.len());
                }
            };
            if s.reached.is_none() && t >= r.hide {
                find(self, &mut s, "hiding ran out");
            }
            if s.reached.is_none() {
                // They hold where they were left, whatever the squad's
                // orders (the greedy hand sent them out to the ring and the
                // slime at 0:18 and 0:27).
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
                let reach = 8.0 * sc;
                let by = (0..self.soldiers.len()).find(|&k| {
                    let so = &self.soldiers[k];
                    so.alive && !r.pinned(k) && alive.iter().any(|&m| (self.soldiers[m].x - so.x).hypot(self.soldiers[m].y - so.y) <= reach)
                });
                if let Some(k) = by {
                    let q = self.soldiers[k].squad;
                    for &m in &alive {
                        self.soldiers[m].squad = q;
                    }
                    find(self, &mut s, "reached");
                    alarm_now |= r.alarm;
                    s.reached = Some(t);
                    eprintln!("  lab rescue t {t:.0} s: dug-out {i} reached, {} of {} alive, into squad {q}", alive.len(), s.members.len());
                }
            } else {
                let l = self.larder;
                if alive.iter().any(|&m| (self.soldiers[m].x - l.0).hypot(self.soldiers[m].y - l.1) <= 14.0 * sc) {
                    s.home = Some(t);
                    eprintln!("  lab rescue t {t:.0} s: dug-out {i} home, {} of {} alive", alive.len(), s.members.len());
                }
            }
            r.sites[i] = s;
        }
        if alarm_now {
            let was = self.rules.strikers;
            self.rules.strikers = super::StrikerRules { on: true, below: 2.0, few: -1.0, full: 0.0, ..was };
        }
        if r.sites.iter().all(|s| s.home.is_some()) {
            eprintln!("  lab rescue t {t:.0} s: won, every dug-out brought home");
            self.end = Some(End::MouldWiped(t));
        }
        self.lab_rescue = Some(r);
    }
}
