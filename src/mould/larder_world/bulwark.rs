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

use super::*;

/// Game seconds between the walking maps' refreshes for the bulwark.
pub const BULWARK_STALE: u64 = 5;
/// Cells the bulwark must have changed by before the maps are refreshed.
pub const BULWARK_MOVED: usize = 16;

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
