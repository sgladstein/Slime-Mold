//! The bulwark (the simulation-use plan's E2b, 2026-11-17; owner 01:13Z:
//! set slime blocks "Both" shots and soldiers). Shots are soaked by the set
//! walkers' armour (E2a, in the engines); this is the soldiers' half: fine
//! cells the set slime holds are no ground to their feet.
//!
//! A cell is in the bulwark when its 3 x 3 holds `Rules::set_block` set
//! walkers or more (`World::set_cells`; a fine cell holds at most one
//! walker, so a lone set walker is never a wall), and leaves it when the
//! count falls under half of that, so the edge doesn't flicker. Read once
//! a game second. `Game::ground` reads it at once (the walk, the steer and
//! the squad's anchor: a soldier won't head into or stand in it, and one
//! caught inside as it forms walks out the old way, `walk`'s "shut"; the
//! squad-movement thread, 11-17). The walking maps (`standable_cells`) read
//! it when they are built, and are made stale for it at most every
//! `BULWARK_STALE` seconds and only when `BULWARK_MOVED` cells or more
//! changed: every map is a whole-world Dijkstra that takes 6-7 steps at 6x,
//! and staling them each second would keep them stale in every fight and
//! halve how fast orders answer (the levelgen thread, 11-17). A point check
//! alone was tried first: a squad ordered past a band of set slime walked
//! its map into the band and stood there (the technical lead's review:
//! the stuck-squad bug of #252 again).
//!
//! **On on the Trunk's planet since 2026-11-17** (`run::does_to_you`,
//! `set_block` 5), the Enemy lane's pick of the block-or-trap A/B: the
//! owner's ruling was that bulwarks block soldiers, and a wall is something
//! a squad's orders can see and go round. The other side, set slime that
//! held a soldier in it to a twentieth of its speed while it was bitten
//! (`Rules::set_hold`, the game designer's alternative), was deleted the
//! same day: the walking maps went through it, so a move order across the
//! map walked a squad into it (`Reports/dead-ends.md`).
//!
//! **The fort** (2026-11-18, the Enemy lane). The owner, 11-18 03:11Z, on
//! walls that formed only where the Trunk was shot: "when it was original
//! suggested, I thought the walls would form protecting the core of the
//! slime or the slime crystals giving them a base of sorts". So the slime
//! walls what it holds. Every `FORT_EVERY` game seconds the game looks at
//! each crystal: one whose cells are a quarter covered by walkers is held
//! (`FORT_HOLD`), until it is dry or the cover falls under a twentieth
//! (`FORT_LOSE`). The held crystals' rings (`fort_mask`) are the mask the
//! engine sets walkers on (`World::set_fort`; engine.wgsl's `fort_at`):
//! slime standing on the ring sets with no scar and stays set, and the
//! rule above makes it a wall. A crystal that is lost or dry drops out of
//! the mask and its walls wake. Shot slime still sets where it is hit.
//! GPU engine only (the owner's card, 11-17 19:21Z: no new CPU twin).

use super::*;

/// Game seconds between the walking maps' refreshes for the bulwark.
pub const BULWARK_STALE: u64 = 5;
/// Cells the bulwark must have changed by before the maps are refreshed.
pub const BULWARK_MOVED: usize = 16;
/// Game seconds between the fort's looks at who holds each crystal.
pub const FORT_EVERY: u64 = 2;
/// The share of a crystal's cells with a walker on them at which the
/// slime holds it, and the share under which it has lost it.
pub const FORT_HOLD: f32 = 0.25;
pub const FORT_LOSE: f32 = 0.05;
/// A ring's gap from its crystal's edge, its gates and a gate's width, in
/// dish cells: a court for the slime inside, and ways in and out for it
/// and for a squad.
pub const FORT_GAP: f32 = 3.0;
pub const FORT_GATES: usize = 4;
pub const FORT_GATE: f32 = 4.0;

/// One held crystal's ring: its centre, the ring's inner radius (fine
/// cells) and the angle its first gate faces.
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct Ring {
    pub at: (f32, f32),
    pub r: f32,
    pub gate: f32,
}

/// The fort's mask over a `w` x `h` world: a word a `big::RAID_BLOCK`
/// block, row by row, 1 on the rings. A block is on a ring when its centre
/// is within `thick` fine cells outside the nearest ring's inner radius
/// and inside no other ring, so crystals close together share one outline;
/// and not in one of that ring's `FORT_GATES` gates (`gate` fine cells
/// wide, evenly round it from `Ring::gate`).
pub fn fort_mask(w: usize, h: usize, rings: &[Ring], thick: f32, gate: f32) -> Vec<u32> {
    let b = crate::mould::big::RAID_BLOCK;
    let (bw, bh) = (w.div_ceil(b), h.div_ceil(b));
    let mut mask = vec![0u32; bw * bh];
    if rings.is_empty() || thick <= 0.0 {
        return mask;
    }
    let turn = std::f32::consts::TAU / FORT_GATES as f32;
    for by in 0..bh {
        for bx in 0..bw {
            let (x, y) = ((bx * b) as f32 + 0.5 * b as f32, (by * b) as f32 + 0.5 * b as f32);
            // The nearest ring by distance past its inner radius: under 0
            // is inside a ring (its court), so no wall.
            let Some((past, d, ring)) = rings.iter().map(|g| (g.at.0 - x).hypot(g.at.1 - y)).zip(rings).map(|(d, g)| (d - g.r, d, g)).min_by(|p, q| p.0.total_cmp(&q.0)) else {
                continue;
            };
            if !(0.0..thick).contains(&past) {
                continue;
            }
            // The angle from the nearest gate's middle, as an arc at this
            // distance.
            let a = ((y - ring.at.1).atan2(x - ring.at.0) - ring.gate).rem_euclid(turn);
            let off = a.min(turn - a) * d;
            if off >= 0.5 * gate {
                mask[by * bw + bx] = 1;
            }
        }
    }
    mask
}

impl<W: World> Game<W> {
    /// Whether the fine cell at `(x, y)` is in the bulwark.
    pub fn bulwark_at(&self, x: f32, y: f32) -> bool {
        if self.bulwark.is_empty() || x < 0.0 || y < 0.0 {
            return false;
        }
        let (w, h) = self.world.size();
        let (cx, cy) = (x as usize, y as usize);
        cx < w && cy < h && self.bulwark[cy * w + cx]
    }

    /// The bulwark from the set walkers now (`Rules::set_block`); cleared
    /// with the rule off or setting off. Once a game second.
    pub(super) fn mark_bulwark(&mut self) {
        let on = self.rules.set_block;
        let cells = if on > 0.0 { self.world.set_cells() } else { None };
        let (w, h) = self.world.size();
        let Some(c) = cells.filter(|c| c.len() == w * h) else {
            self.bulwark.clear();
            self.stale_for_bulwark();
            return;
        };
        let was = std::mem::take(&mut self.bulwark);
        let mut b = vec![false; w * h];
        let mut any = false;
        for y in 0..h {
            for x in 0..w {
                let mut n = 0u32;
                for yy in y.saturating_sub(1)..(y + 2).min(h) {
                    for xx in x.saturating_sub(1)..(x + 2).min(w) {
                        n += c[yy * w + xx];
                    }
                }
                let i = y * w + x;
                let held = was.get(i).copied().unwrap_or(false);
                let n = n as f32;
                b[i] = n >= on || (held && n >= 0.5 * on);
                any |= b[i];
            }
        }
        if any {
            self.bulwark = b;
        }
        self.stale_for_bulwark();
    }

    /// The fort's mask from the crystals the slime holds now (the module's
    /// notes). Every `FORT_EVERY` game seconds, and written only when a
    /// crystal was taken, lost or run dry. The home crystal's first gate
    /// faces the middle of the map and every other crystal's faces home,
    /// where its tube runs (`open_gate` turns a ring whose gates would
    /// face rock).
    pub(super) fn time_fort(&mut self) {
        let k = self.world.scale();
        let thick = self.rules.fort * k;
        if thick <= 0.0 || !self.world.forts() {
            // The rule went off under a fort: its walls wake.
            if self.fort_held.iter().any(|&h| h) {
                self.fort_held.clear();
                self.world.set_fort(Vec::new());
            }
            return;
        }
        if !(self.world.ticks() / self.rules.tps as u64).is_multiple_of(FORT_EVERY) {
            return;
        }
        // The walkers on each crystal, from one copy of the mould's grid
        // (squares of a dish cell, as `World::walkers_in` sums; asked for
        // once, since a world may read it back for each ask).
        let bin = (k as usize).max(1);
        let Some(grid) = self.world.mould_grid(bin) else { return };
        let (w, h) = self.world.size();
        let gw = w.div_ceil(bin);
        let gh = grid.len() / gw.max(1);
        let on = |at: (f32, f32), r: f32| -> u32 {
            let mid = |i: usize| (i as f32 + 0.5) * bin as f32;
            // Only the squares the crystal's box covers.
            let span = |c: f32, n: usize| ((((c - r) / bin as f32).floor().max(0.0)) as usize)..((((c + r) / bin as f32).ceil().max(0.0)) as usize + 1).min(n);
            span(at.1, gh).flat_map(|y| span(at.0, gw).map(move |x| (x, y))).filter(|&(x, y)| (mid(x) - at.0).hypot(mid(y) - at.1) <= r).map(|(x, y)| grid[y * gw + x]).sum()
        };
        let crystals = self.world.crystals();
        let mut held = std::mem::take(&mut self.fort_held);
        held.resize(crystals.len(), false);
        let mut changed = false;
        for (c, &(at, left)) in crystals.iter().enumerate() {
            let r = self.world.crystal_r(c);
            let share = on(at, r) as f32 / (std::f32::consts::PI * r * r).max(1.0);
            let now = left > 0.0 && share >= if held[c] { FORT_LOSE } else { FORT_HOLD };
            changed |= now != held[c];
            held[c] = now;
        }
        self.fort_held = held;
        if !changed {
            return;
        }
        let rings: Vec<Ring> = crystals
            .iter()
            .enumerate()
            .filter(|&(c, _)| self.fort_held[c])
            .map(|(c, &(at, _))| {
                let to = if (at.0 - self.core.0).hypot(at.1 - self.core.1) < 1.0 { (0.5 * w as f32, 0.5 * h as f32) } else { self.core };
                let r = self.world.crystal_r(c) + FORT_GAP * k;
                Ring { at, r, gate: self.open_gate(at, r + 0.5 * thick, (to.1 - at.1).atan2(to.0 - at.0)) }
            })
            .collect();
        self.world.set_fort(fort_mask(w, h, &rings, thick, FORT_GATE * k));
    }

    /// The angle a ring's first gate faces: `toward`, unless fewer than two
    /// of its gates would then open on ground (a crystal in a pocket of
    /// rock or by deep water), when the ring is turned by eighths of a gate
    /// spacing to the turn with the most gates on ground. A ring whose
    /// gates all faced rock would shut the slime from its own crystal and
    /// a squad inside from the map. `d` is the gates' distance from `at`.
    fn open_gate(&self, at: (f32, f32), d: f32, toward: f32) -> f32 {
        let turn = std::f32::consts::TAU / FORT_GATES as f32;
        let open = |a: f32| (0..FORT_GATES).filter(|&g| self.world.open(at.0 + d * (a + g as f32 * turn).cos(), at.1 + d * (a + g as f32 * turn).sin())).count();
        let mut best = (open(toward), toward);
        for j in 1..8 {
            if best.0 >= 2 {
                break;
            }
            let a = toward + j as f32 * turn / 8.0;
            let n = open(a);
            if n > best.0 {
                best = (n, a);
            }
        }
        best.1
    }

    /// The walking maps made stale for the bulwark (`BULWARK_STALE`,
    /// `BULWARK_MOVED`).
    fn stale_for_bulwark(&mut self) {
        let now = self.world.ticks();
        if now < self.bulwark_staled + BULWARK_STALE * self.rules.tps as u64 && !self.bulwark_maps.is_empty() {
            return;
        }
        let at = |v: &[bool], i: usize| v.get(i).copied().unwrap_or(false);
        let n = self.bulwark.len().max(self.bulwark_maps.len());
        let moved = (0..n).filter(|&i| at(&self.bulwark, i) != at(&self.bulwark_maps, i)).count();
        if moved >= BULWARK_MOVED {
            self.stale_maps();
            self.bulwark_maps = self.bulwark.clone();
            self.bulwark_staled = now;
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    /// The fort's mask is a ring `thick` wide outside each ring's radius,
    /// open at four gates from the angle the ring gives, and two rings
    /// that overlap share one outline with no wall in either's court.
    #[test]
    fn the_forts_mask_rings_a_crystal_with_gates() {
        let b = crate::mould::big::RAID_BLOCK;
        let (w, h) = (120usize, 80usize);
        let bw = w.div_ceil(b);
        let on = |m: &[u32], x: f32, y: f32| m[(y as usize / b) * bw + x as usize / b] != 0;
        assert!(fort_mask(w, h, &[], 4.0, 8.0).iter().all(|&v| v == 0), "no ring, no mask");
        let one = [Ring { at: (40.0, 40.0), r: 16.0, gate: 0.0 }];
        let m = fort_mask(w, h, &one, 4.0, 8.0);
        assert_eq!(m.len(), bw * h.div_ceil(b));
        // Along the diagonal: court, wall, open ground.
        let d = std::f32::consts::FRAC_1_SQRT_2;
        assert!(!on(&m, 40.0 + 12.0 * d, 40.0 + 12.0 * d), "the court is open");
        assert!(on(&m, 40.0 + 18.0 * d, 40.0 + 18.0 * d), "the ring is wall");
        assert!(!on(&m, 40.0 + 24.0 * d, 40.0 + 24.0 * d), "past the ring is open");
        // The four gates, from angle 0: east, south, west and north.
        for (gx, gy) in [(1.0, 0.0), (0.0, 1.0), (-1.0, 0.0), (0.0, -1.0)] {
            assert!(!on(&m, 40.5 + 18.0 * gx, 40.5 + 18.0 * gy), "a gate at ({gx}, {gy})");
        }
        // The ring is most of its band: the gates take 32 of about 113
        // cells of its middle circle.
        let cells = b * b * m.iter().filter(|&&v| v != 0).count();
        let band = std::f32::consts::PI * (20.0 * 20.0 - 16.0 * 16.0);
        assert!((cells as f32) > 0.5 * band && (cells as f32) < 0.9 * band, "{cells} cells of a band of {band}");
        // A gate turned by the ring's angle.
        let turned = fort_mask(w, h, &[Ring { gate: std::f32::consts::FRAC_PI_4, ..one[0] }], 4.0, 8.0);
        assert!(on(&turned, 58.5, 40.5) && !on(&turned, 40.0 + 18.0 * d, 40.0 + 18.0 * d), "the gates turn with the ring");
        // Two rings that overlap: no wall where one's band is inside the
        // other's court, and wall on the far sides.
        let two = [one[0], Ring { at: (64.0, 40.0), r: 12.0, gate: 0.0 }];
        let m = fort_mask(w, h, &two, 4.0, 8.0);
        assert!(!on(&m, 54.0, 40.0) && !on(&m, 57.5, 40.5), "one outline: no wall inside the other ring");
        assert!(on(&m, 40.0 - 18.0 * d, 40.0 - 18.0 * d) && on(&m, 64.0 + 14.0 * d, 40.0 - 14.0 * d), "wall on the far sides");
    }

    /// The bulwark marks a cell whose 3 x 3 holds `set_block` set walkers,
    /// holds it down to half of that, and only `ground` reads it: the
    /// walking maps' cells (`standable_cells`) stay terrain.
    #[test]
    fn set_slime_is_no_ground_to_soldiers_and_holds_to_half() {
        let mut g = Game::new(race_world(1, 1, 1_000), Rules { set_block: 5.0, ..Rules::default() }, 1);
        let (w, h) = g.world.size();
        let (cx, cy) = (w / 2, h / 2);
        assert!(g.ground(cx as f32 + 0.5, cy as f32 + 0.5), "open ground at the middle");
        g.mark_bulwark();
        assert!(g.bulwark.is_empty(), "no setting, no bulwark");
        g.world.p.set_scar = 0.1;
        if g.world.scar.is_empty() {
            g.world.scar = vec![0.0; w * h];
        }
        g.world.set_cells = vec![0; w * h];
        let put = |g: &mut Game<BigDish>, n: usize| {
            g.world.set_cells.iter_mut().for_each(|c| *c = 0);
            for d in 0..n {
                g.world.set_cells[(cy - 1 + d / 3) * w + cx - 1 + d % 3] = 1;
            }
            g.mark_bulwark();
        };
        let maps = g.standable_cells();
        put(&mut g, 9);
        let mid = (cx as f32 + 0.5, cy as f32 + 0.5);
        assert!(!g.ground(mid.0, mid.1) && !g.standable(mid.0, mid.1), "nine set walkers are a wall to feet");
        assert!(g.ground(mid.0 + 4.0, mid.1), "two cells past them is ground");
        let now = g.standable_cells();
        assert!(now != maps && !now[cy * w + cx], "the walking maps route round it");
        put(&mut g, 3);
        assert!(!g.ground(mid.0, mid.1), "three hold it (half of five)");
        put(&mut g, 2);
        assert!(g.ground(mid.0, mid.1) && g.bulwark.is_empty(), "two let it go");
        put(&mut g, 4);
        assert!(g.ground(mid.0, mid.1), "four don't raise it");
    }

    /// The technical lead's ask (11-17): a soldier ordered past a band of
    /// set slime goes round it and arrives, never standing in it. A 3-cell
    /// band 21 cells long across its straight way; open both ends.
    #[test]
    fn a_soldier_ordered_past_set_slime_goes_round_it() {
        let mut g = Game::new(race_world(1, 1, 1_000), Rules { set_block: 5.0, ..Rules::default() }, 1);
        let (w, h) = g.world.size();
        g.world.p.set_scar = 0.1;
        if g.world.scar.is_empty() {
            g.world.scar = vec![0.0; w * h];
        }
        g.world.set_cells = vec![0; w * h];
        let from = (g.soldiers[0].x, g.soldiers[0].y);
        let far = 24.0;
        let to = (0..32)
            .map(|k| k as f32 / 32.0 * std::f32::consts::TAU)
            .map(|a| (from.0 + far * a.cos(), from.1 + far * a.sin()))
            .find(|&p| g.walk_line(from, p))
            .expect("open ground round the larder");
        let (d, n) = (((to.0 - from.0) / far, (to.1 - from.1) / far), (-(to.1 - from.1) / far, (to.0 - from.0) / far));
        let mid = ((from.0 + to.0) / 2.0, (from.1 + to.1) / 2.0);
        for t in -10..=10 {
            for a in -1..=1 {
                let (x, y) = (mid.0 + n.0 * t as f32 + d.0 * a as f32, mid.1 + n.1 * t as f32 + d.1 * a as f32);
                if x >= 0.0 && y >= 0.0 && (x as usize) < w && (y as usize) < h {
                    g.world.set_cells[y as usize * w + x as usize] = 1;
                }
            }
        }
        g.mark_bulwark();
        assert!(!g.ground(mid.0, mid.1) && !g.walk_line(from, to), "the band cuts the straight way");
        g.soldiers[0].order = Order::Move(to.0, to.1);
        let dt = 1.0 / g.rules.tps as f32;
        let mut took = None;
        for step in 0..30 * g.rules.tps {
            g.pump_maps();
            g.walk(0, dt);
            let s = g.soldiers[0];
            assert!(!g.bulwark_at(s.x, s.y), "a soldier in set slime at ({}, {})", s.x, s.y);
            if (s.x - to.0).hypot(s.y - to.1) < 1.0 {
                took = Some(step as f32 * dt);
                break;
            }
        }
        let took = took.expect("the soldier got past the band");
        eprintln!("round the band in {took:.1} s");
    }
}
