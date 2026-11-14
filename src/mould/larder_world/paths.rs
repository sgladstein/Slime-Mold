//! Walking maps and the soldiers' movement over them. Moved out of
//! `larder_world.rs` unchanged (C9, 2026-11-14).

use super::*;

/// Game seconds between rebuilds of the walking maps.
pub(super) const PATHS_EVERY: f32 = 30.0;

/// How far past a solid crystal's rim a soldier stands to harvest or
/// bank, dish cells (`Rules::crystal_solid`).
pub(super) const RIM: f32 = 1.0;

pub(super) fn cell(w: usize, h: usize, p: (f32, f32)) -> usize {
    let x = (p.0.max(0.0) as usize).min(w - 1);
    let y = (p.1.max(0.0) as usize).min(h - 1);
    y * w + x
}

/// Distances to `goal` over open cells, 5 a side step and 7 a diagonal
/// (a diagonal only when both side cells are open, so no corner is cut);
/// `u32::MAX` where the goal cannot be reached.
pub fn distance_map(w: usize, h: usize, goal: usize, open: impl Fn(usize) -> bool) -> Vec<u32> {
    use std::cmp::Reverse;
    use std::collections::BinaryHeap;
    let ok: Vec<bool> = (0..w * h).map(&open).collect();
    let mut dist = vec![u32::MAX; w * h];
    let mut heap = BinaryHeap::new();
    dist[goal] = 0;
    heap.push(Reverse((0u32, goal)));
    while let Some(Reverse((d, i))) = heap.pop() {
        if d > dist[i] {
            continue;
        }
        let (x, y) = ((i % w) as i64, (i / w) as i64);
        for (dx, dy) in [(-1i64, 0i64), (1, 0), (0, -1), (0, 1), (-1, -1), (1, -1), (-1, 1), (1, 1)] {
            let (nx, ny) = (x + dx, y + dy);
            if nx < 0 || ny < 0 || nx >= w as i64 || ny >= h as i64 {
                continue;
            }
            let j = ny as usize * w + nx as usize;
            if !ok[j] {
                continue;
            }
            let diag = dx != 0 && dy != 0;
            if diag && (!ok[y as usize * w + nx as usize] || !ok[ny as usize * w + x as usize]) {
                continue;
            }
            let nd = d + if diag { 7 } else { 5 };
            if nd < dist[j] {
                dist[j] = nd;
                heap.push(Reverse((nd, j)));
            }
        }
    }
    dist
}

/// The neighbour of `i` nearest the goal on `map` (`i` itself at the goal,
/// or when no neighbour is nearer).
pub(super) fn downhill(w: usize, h: usize, map: &[u32], i: usize) -> usize {
    let (x, y) = ((i % w) as i64, (i / w) as i64);
    let mut best = i;
    for (dx, dy) in [(-1i64, 0i64), (1, 0), (0, -1), (0, 1), (-1, -1), (1, -1), (-1, 1), (1, 1)] {
        let (nx, ny) = (x + dx, y + dy);
        if nx < 0 || ny < 0 || nx >= w as i64 || ny >= h as i64 {
            continue;
        }
        let j = ny as usize * w + nx as usize;
        if map[j] < map[best] {
            best = j;
        }
    }
    best
}

/// The point of the segment `a`-`b` nearest `p`.
pub(super) fn nearest_on(a: (f32, f32), b: (f32, f32), p: (f32, f32)) -> (f32, f32) {
    let (dx, dy) = (b.0 - a.0, b.1 - a.1);
    let l2 = dx * dx + dy * dy;
    let t = if l2 > 0.0 { (((p.0 - a.0) * dx + (p.1 - a.1) * dy) / l2).clamp(0.0, 1.0) } else { 0.0 };
    (a.0 + dx * t, a.1 + dy * t)
}

/// Soldier `k`'s place round the point it was sent to, dish cells (M3's
/// sunflower: a squad sent to one point spreads instead of stacking).
pub(super) fn slot(k: usize) -> (f32, f32) {
    if k == 0 {
        return (0.0, 0.0);
    }
    let r = 1.5 * (k as f32).sqrt();
    let a = k as f32 * 2.399_963;
    (r * a.cos(), r * a.sin())
}

impl<W: World> Game<W> {
    /// Where soldier `k` is walking to now, fine cells, if anywhere.
    pub(super) fn goal(&self, k: usize) -> Option<(f32, f32)> {
        let s = self.soldiers[k];
        self.goal_raw(k).map(|g| self.rim(g, (s.x, s.y)))
    }

    pub(super) fn solid_now(&self) -> Vec<((f32, f32), f32)> {
        if !self.rules.crystal_solid {
            return Vec::new();
        }
        // A disc holding the charge left: soldiers take from the rim
        // (`squad`'s take disc reaches 4 dish cells in), so it shrinks as
        // they eat in and they step in after it. A fixed disc left a
        // crystal's middle out of reach once its rim was eaten.
        let min = self.s();
        self.world
            .crystals()
            .iter()
            .enumerate()
            .filter(|(_, c)| c.1 > 0.0)
            .map(|(i, c)| {
                let full = self.crystal_full.get(i).copied().unwrap_or(c.1).max(c.1);
                (c.0, self.world.crystal_r(i) * (c.1 / full.max(1e-6)).sqrt())
            })
            .filter(|&(_, r)| r >= min)
            .collect()
    }

    /// The crystal (charge left) whose disc holds `p`, if crystals are
    /// solid (`Rules::crystal_solid`): its centre and radius.
    pub(super) fn solid_at(&self, p: (f32, f32)) -> Option<((f32, f32), f32)> {
        self.solid.iter().copied().find(|&(c, r)| (c.0 - p.0).hypot(c.1 - p.1) < r)
    }

    /// How near `p` a soldier must stand to work at it, fine cells: `near`
    /// dish cells, or from the rim if `p` is on a solid crystal (a larder,
    /// outpost or ghost placed on one).
    pub(super) fn reach(&self, p: (f32, f32), near: f32) -> f32 {
        let sc = self.s();
        self.solid_at(p).map_or(near * sc, |(_, r)| (r + (RIM + 2.0) * sc).max(near * sc))
    }

    /// Ground a soldier may stand on: open, and not on a crystal with
    /// charge left (`Rules::crystal_solid`).
    pub fn standable(&self, x: f32, y: f32) -> bool {
        self.world.open(x, y) && self.solid_at((x, y)).is_none()
    }

    /// `p`, or if it is on a solid crystal, the point just past that
    /// crystal's rim on the side toward `from`.
    pub fn rim(&self, p: (f32, f32), from: (f32, f32)) -> (f32, f32) {
        let Some((c, r)) = self.solid_at(p) else { return p };
        let (dx, dy) = (from.0 - c.0, from.1 - c.1);
        let d = dx.hypot(dy);
        let (ux, uy) = if d > 1e-3 { (dx / d, dy / d) } else { (1.0, 0.0) };
        let out = r + RIM * self.s();
        (c.0 + ux * out, c.1 + uy * out)
    }

    /// Where soldier `k`'s order points, before a crystal moves it to its
    /// rim (`goal`).
    pub(super) fn goal_raw(&self, k: usize) -> Option<(f32, f32)> {
        let s = self.soldiers[k];
        let sc = self.s();
        let off = |p: (f32, f32)| {
            let o = slot(k);
            (p.0 + o.0 * sc, p.1 + o.1 * sc)
        };
        match s.order {
            Order::Hold => None,
            Order::Move(x, y) | Order::Fight(x, y) => Some(off((x, y))),
            Order::Build(b) => {
                let p = self.site(k, b)?;
                let q = off(p);
                // A builder stands at the building's edge, not on it, so the
                // rising ghost and its bar stay in sight (2026-11-12, the
                // build-anywhere look: a lone builder hid its turret). Walls
                // keep the point on the line nearest the soldier.
                if matches!(self.buildings[b].kind, Kind::Wall { .. }) {
                    return Some(q);
                }
                let edge = (self.rules.build.touch + 1.0) * sc;
                let (dx, dy) = (q.0 - p.0, q.1 - p.1);
                if dx.hypot(dy) >= edge {
                    return Some(q);
                }
                let (fx, fy) = if dx.hypot(dy) > 1e-3 { (dx, dy) } else { (s.x - p.0, s.y - p.1) };
                let d = fx.hypot(fy);
                let (ux, uy) = if d > 1e-3 { (fx / d, fy / d) } else { (1.0, 0.0) };
                Some((p.0 + ux * edge, p.1 + uy * edge))
            }
            Order::Clear(x, y) => {
                if s.homing {
                    Some(off(self.drop_off((s.x, s.y))))
                } else {
                    self.crystal_at((x, y)).map(|(_, c)| off(c))
                }
            }
            Order::Burn(a, b) => {
                // Stand short of the line's middle, on the side it was
                // ordered from, so the whole line is in range.
                let m = ((a.0 + b.0) / 2.0, (a.1 + b.1) / 2.0);
                let (dx, dy) = (s.x - m.0, s.y - m.1);
                let d = dx.hypot(dy);
                let keep = 0.6 * self.rules.range * sc;
                if d <= keep {
                    None
                } else {
                    Some((m.0 + dx / d * keep, m.1 + dy / d * keep))
                }
            }
            Order::Attack => Some(off(self.core)),
        }
    }

    /// Whether a soldier can walk straight from `a` to `b`: every cell on
    /// the line standable (`standable`; `clear_line` is the line of fire).
    pub(super) fn walk_line(&self, a: (f32, f32), b: (f32, f32)) -> bool {
        let (w, h) = self.world.size();
        squad::path(w, h, a, b).into_iter().all(|i| self.standable((i as usize % w) as f32 + 0.5, (i as usize / w) as f32 + 0.5))
    }

    /// Whether every cell on the line from `a` to `b` is open ground.
    pub fn clear_line(&self, a: (f32, f32), b: (f32, f32)) -> bool {
        let (w, h) = self.world.size();
        squad::path(w, h, a, b).into_iter().all(|i| self.world.open((i as usize % w) as f32 + 0.5, (i as usize / w) as f32 + 0.5))
    }

    /// Walk soldier `k` toward its goal for `dt` game seconds: straight
    /// when the line is open, else down the goal's distance map (`Paths`),
    /// and along one axis when even that step is blocked. (The first
    /// version only slid along an axis: on seed 1 three of four rushing
    /// soldiers stood 50 cells short of the core behind rock for the
    /// whole game, firing 196 of 200 shots into the rock.)
    pub(super) fn walk(&mut self, k: usize, dt: f32) {
        let Some(g) = self.goal(k) else { return };
        let s = self.soldiers[k];
        let d = (g.0 - s.x).hypot(g.1 - s.y);
        if d < 0.5 || self.engaged(k) {
            return;
        }
        // Caught on a crystal (a larder or ghost placed by one, a crystal
        // grown back): straight out to its near rim first, over anything.
        let caught = self.solid_at((s.x, s.y)).is_some();
        let g = if caught {
            self.rim((s.x, s.y), (s.x, s.y))
        } else if self.walk_line((s.x, s.y), g) {
            g
        } else {
            let now = self.world.ticks();
            if now >= self.paths.at + self.secs(PATHS_EVERY) {
                // Walls wear to rubble: rebuild the maps now and then.
                self.paths.maps.clear();
                self.paths.at = now;
            }
            let (w, h) = self.world.size();
            let gi = cell(w, h, g);
            if !self.paths.maps.contains_key(&gi) {
                let map = distance_map(w, h, gi, |i| self.standable((i % w) as f32 + 0.5, (i / w) as f32 + 0.5));
                self.paths.maps.insert(gi, map);
            }
            let map = &self.paths.maps[&gi];
            // A few cells down the map, so the step heads along the path
            // rather than zig-zagging cell to cell.
            let mut at = cell(w, h, (s.x, s.y));
            for _ in 0..3 {
                at = downhill(w, h, map, at);
            }
            if at == gi {
                g
            } else {
                ((at % w) as f32 + 0.5, (at / w) as f32 + 0.5)
            }
        };
        let (dx, dy) = (g.0 - s.x, g.1 - s.y);
        let d = dx.hypot(dy).max(1e-6);
        // Thick slime holds a soldier back (`Rules::wade`).
        let wade = if self.rules.wade > 0.0 {
            // Whole walkers: every one on a soldier holds its legs at any mass (owner, 11-13 15:55Z).
            let x = s.touching as f32 / self.rules.wade;
            1.0 / (1.0 + x * x)
        } else {
            1.0
        };
        let step = (self.rules.speed * self.s() * dt * wade).min(d);
        let (nx, ny) = (s.x + dx / d * step, s.y + dy / d * step);
        let ok = |x: f32, y: f32| if caught { self.world.open(x, y) } else { self.standable(x, y) };
        let to = if ok(nx, ny) {
            Some((nx, ny))
        } else if ok(nx, s.y) {
            Some((nx, s.y))
        } else if ok(s.x, ny) {
            Some((s.x, ny))
        } else {
            None
        };
        if let Some((x, y)) = to {
            self.soldiers[k].x = x;
            self.soldiers[k].y = y;
        }
    }

    /// An attack-moving soldier (`Order::Fight`) with mould it can shoot in
    /// sight: it stands and fights instead of walking on.
    pub(super) fn engaged(&self, k: usize) -> bool {
        let s = self.soldiers[k];
        let Order::Fight(..) = s.order else { return false };
        let Some(aim) = s.aim else { return false };
        s.dense >= self.rules.aim_min && (aim.0 - s.x).hypot(aim.1 - s.y) <= self.rules.range * self.s() && self.clear_line((s.x, s.y), aim)
    }

    /// The depot a carrier at `p` walks home to: the nearest of the larder
    /// and the standing outposts (`Game::drop_offs`).
    pub(super) fn drop_off(&self, p: (f32, f32)) -> (f32, f32) {
        let d = |a: (f32, f32)| (a.0 - p.0).hypot(a.1 - p.1);
        self.drop_offs().into_iter().min_by(|a, b| d(*a).total_cmp(&d(*b))).unwrap_or(self.larder)
    }
}
