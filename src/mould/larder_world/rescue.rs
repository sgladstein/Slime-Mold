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
//! - **Won** when one of them stands within 14 cells of the larder;
//!   **lost** when all of them are dead. The heart's own win still stands.
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

#[derive(Clone, Debug)]
pub struct LabRescue {
    pub at: (f32, f32),
    pub hide: f32,
    pub alarm: bool,
    /// The game second they were found (reached, or HIDE ran out).
    pub found: Option<f32>,
    pub members: Vec<usize>,
    /// The game second a soldier of yours reached them.
    pub reached: Option<f32>,
    pub out: f32,
    /// How many were alive at the last second, for the log.
    pub left: usize,
    /// The found stranded's walking pace (`Game::wade`), 0..1.
    pub wound: f32,
    /// The ring's radius, dish cells (0 none), and the second it was dug.
    pub ring: f32,
    pub since: f32,
}

impl<W: World> Game<W> {
    /// Soldier `k`'s pace as one of the found stranded (`LabRescue::wound`), else 1.
    pub(super) fn lab_wound(&self, k: usize) -> f32 {
        match &self.lab_rescue {
            Some(r) if r.reached.is_some() && r.members.contains(&k) => r.wound,
            _ => 1.0,
        }
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
            let (at, n) = ((f[0], f[1]), f.get(2).map_or(3, |&n| n.max(1.0) as usize));
            let (hide, out, alarm) = (f.get(3).copied().unwrap_or(300.0), f.get(4).copied().unwrap_or(3.0), f.get(5).copied().unwrap_or(1.0) > 0.0);
            let Some(q) = self.free_squad() else { return };
            let sc = self.s();
            let ring_cells = f.get(6).copied().unwrap_or(7.0);
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
                // Close together in the middle, not wherever
                // `add_soldier_at` finds standing room (v3 put one on the
                // rock outside the ring).
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
            // Dug in: an octagon of walls, raised whole (as `stand` in the
            // window's headless runs places a building).
            let ring = ring_cells * sc;
            let thick = (f.get(8).copied().unwrap_or(1.0) as u8).clamp(1, super::doors::WALL_LEVELS);
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
                        // Doors, not plain walls (`Building::door`): the
                        // slime can't cross them and bites them as walls,
                        // but soldiers walk through. As plain walls (v4)
                        // the rescuers reached them and the found walked
                        // nowhere: the ring held them in, and the last
                        // one died in it at 3:06 with the squad home.
                        self.buildings[n0].door = true;
                        // Thickened outward, not on the side toward the
                        // larder (`wall_home`): thickened inward, the bands
                        // filled the ring and evicted the slime in them
                        // onto the soldiers (dead by 0:07 at 40,150).
                        let m = ((a.0 + b.0) / 2.0 - at.0, (a.1 + b.1) / 2.0 - at.1);
                        let l = m.0.hypot(m.1).max(1e-6);
                        self.buildings[n0].home = (m.0 / l, m.1 / l);
                        self.raise(n0, 1.0);
                        // THICK levels in all (`Game::thicken`, the walls'
                        // own upgrade), so how long it holds is a dial.
                        for _ in 1..thick {
                            self.thicken(n0);
                        }
                        let w = &mut self.buildings[n0];
                        w.hp = w.max_hp;
                        up += 1;
                    } else {
                        eprintln!("  lab rescue: wall {i} not built: {}", self.build_blocked(kind, a).unwrap_or("blocked"));
                    }
                }
                eprintln!("  lab rescue: {up} of 8 walls round them");
            }
            eprintln!("  lab rescue t {t:.0} s: {n} soldiers stranded at {:.0},{:.0} in squad {q}", at.0, at.1);
            self.lab_rescue = Some(LabRescue { at, hide, alarm, found: None, members, reached: None, out, left: n, wound: f.get(7).copied().unwrap_or(0.3).clamp(0.05, 1.0), ring: ring_cells.max(0.0), since: t });
            return;
        };
        // The walls' own evict (#739) pushes the slime they were built
        // over out of their cells, half of it inward: cleared again for
        // the first seconds.
        if r.found.is_none() && r.ring > 0.0 && t <= r.since + 3.0 {
            let sc = self.s();
            let clear = super::squad::Burst { at: (r.at.0 / sc, r.at.1 / sc), r: (r.ring - self.rules.wall_width).max(1.0), kill: true, ..Default::default() };
            self.world.burst(&clear, &super::squad::BurstRules::default());
        }
        let alive: Vec<usize> = r.members.iter().copied().filter(|&k| self.soldiers[k].alive).collect();
        if alive.len() < r.left {
            let near = |m: usize| (0..self.soldiers.len()).filter(|&k| self.soldiers[k].alive && !r.members.contains(&k)).map(|k| (self.soldiers[k].x - self.soldiers[m].x).hypot(self.soldiers[k].y - self.soldiers[m].y)).fold(f32::INFINITY, f32::min);
            let dead: Vec<String> = r.members.iter().filter(|&&m| !self.soldiers[m].alive).map(|&m| format!("{:.0},{:.0} ({:.0} from help)", self.soldiers[m].x, self.soldiers[m].y, near(m))).collect();
            eprintln!("  lab rescue t {t:.0} s: {} of {} alive; dead at {}", alive.len(), r.members.len(), dead.join(", "));
            r.left = alive.len();
        }
        if alive.is_empty() {
            eprintln!("  lab rescue t {t:.0} s: lost, every stranded soldier died{}", if r.reached.is_some() { " on the way home" } else { "" });
            self.end = Some(End::SquadWiped(t));
            self.lab_rescue = Some(r);
            return;
        }
        let sc = self.world.scale();
        // Found: they reek and fight for their lives.
        let find = |g: &mut Self, r: &mut LabRescue, alive: &[usize], why: &str| {
            if r.found.is_none() {
                r.found = Some(t);
                for &m in alive {
                    g.soldiers[m].smell = r.out;
                    g.soldiers[m].stance = crate::mould::larder::Stance::Free;
                }
                eprintln!("  lab rescue t {t:.0} s: found ({why}), {} alive", alive.len());
            }
        };
        if r.reached.is_none() && t >= r.hide {
            find(self, &mut r, &alive, "hiding ran out");
        }
        match r.reached {
            None => {
                // They hold where they were left, whatever the squad's
                // orders (the greedy hand sent them out to the ring and
                // the slime at 0:18 and 0:27).
                let q = self.soldiers[alive[0]].squad as usize;
                self.squads[q].dest = Some(r.at);
                for &m in &alive {
                    let s = &mut self.soldiers[m];
                    let off = (s.x - r.at.0).hypot(s.y - r.at.1) > 2.5 * sc;
                    let home = matches!(s.order, super::Order::Move(x, y) if (x - r.at.0).abs() < 0.5 && (y - r.at.1).abs() < 0.5);
                    if off && !home {
                        s.order = super::Order::Move(r.at.0, r.at.1);
                        s.queued = 0;
                    } else if !off && !matches!(s.order, super::Order::Hold) {
                        s.order = super::Order::Hold;
                        s.queued = 0;
                    }
                }
                let reach = 8.0 * sc;
                let by = (0..self.soldiers.len()).find(|&k| {
                    let s = &self.soldiers[k];
                    s.alive && !r.members.contains(&k) && alive.iter().any(|&m| (self.soldiers[m].x - s.x).hypot(self.soldiers[m].y - s.y) <= reach)
                });
                if let Some(k) = by {
                    let q = self.soldiers[k].squad;
                    for &m in &alive {
                        self.soldiers[m].squad = q;
                    }
                    find(self, &mut r, &alive, "reached");
                    if r.alarm {
                        let was = self.rules.strikers;
                        self.rules.strikers = super::StrikerRules { on: true, below: 2.0, few: -1.0, full: 0.0, ..was };
                    }
                    r.reached = Some(t);
                    eprintln!("  lab rescue t {t:.0} s: reached, {} of {} alive, into squad {q}", alive.len(), r.members.len());
                }
            }
            Some(_) => {
                let home = 14.0 * sc;
                let l = self.larder;
                if alive.iter().any(|&m| (self.soldiers[m].x - l.0).hypot(self.soldiers[m].y - l.1) <= home) {
                    eprintln!("  lab rescue t {t:.0} s: won, {} of {} brought home", alive.len(), r.members.len());
                    self.end = Some(End::MouldWiped(t));
                }
            }
        }
        self.lab_rescue = Some(r);
    }
}
