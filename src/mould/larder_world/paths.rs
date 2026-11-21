//! Walking maps and the soldiers' movement over them. Moved out of
//! `larder_world.rs` unchanged (C9, 2026-11-14).

use super::*;

/// Game seconds between rebuilds of the walking maps.
pub(super) const PATHS_EVERY: f32 = 30.0;

/// Most walking maps kept at once, and the most cells they hold together
/// (64 MB of distances): past either the map read longest ago goes. A map
/// is the whole world (512 x 320 fine cells at the window's scale 2, 655
/// KB; 3.9 MB on a 6x planet), and a squad's goals move with its anchor,
/// so a squad pressed against rock could build a new one every step and
/// keep them all for `PATHS_EVERY` (bug sweep, 2026-11-14). The cache
/// started again from empty at 48 before 11-14, when every soldier then
/// built its map again at once.
pub(super) const MAPS_MAX: usize = 48;
/// 32M cells (128 MB of distances, 34 maps at 6x). At 16M a 6x planet
/// kept 17 maps, and 18 or more live goals thrashed the cache: with 24, a
/// quarter of goal-steps went unserved while their maps were rebuilt for
/// ever (technical review, 2026-11-15).
const MAP_CELLS_MAX: usize = 32 << 20;

/// Cells taken off a walking map's queue in one step (`MapBuild`), across
/// the maps being built: a step's fixed cost, whatever the number of
/// soldiers. About 4 ms on the cloud CPU; a 1x world's map is one step or
/// two, a 6x planet's six or seven. Before 11-14 a soldier's new map was
/// built whole in the step it was asked for, so a squad's new order cost
/// a map a soldier in one step: a step's max on the 1070 grew from 22 ms
/// at today's area to 210 ms at 6x (technical lead: "a fixed cost per
/// frame, whatever the number of soldiers"). One whole map a step was
/// tried first: 28 ms a step at 6x on the cloud CPU, still over a frame.
pub(super) const MAP_POPS: usize = 1 << 17;
/// A step's cells for a stale map while no goal waits (`pump_maps`).
const STALE_POPS: usize = MAP_POPS / 4;

/// How far from a soldier's goal another goal's walking map serves it,
/// dish cells, when that goal is in a straight walk of its own: a squad's
/// formation slots share one map instead of building one a slot. About a
/// formation of 25's reach from its middle (12.5 dish cells). At 4, a
/// squad of 25 needed a map for each part of its formation: on a 6x core
/// world 8 squads of 25 asked for 45 goal cells a step, more maps than
/// the cache holds at 6x (34), and 47% of map lookups went unserved, the
/// soldier waiting; at 12, 19%, mostly the few seconds after all 8 squads
/// were sent at once (soldier cap 200, 2026-11-16; `examples/soldiercap`,
/// `depth=8 n=200 secs=30`).
pub(super) const MAP_NEAR: f32 = 12.0;

/// How far from a steered squad's moving goal another goal's walking map
/// serves it, dish cells (`guide`).
pub(super) const STEER_NEAR: f32 = 10.0;

/// How far past a solid crystal's rim a soldier stands to harvest or
/// bank, dish cells (`Rules::crystal_solid`).
pub(super) const RIM: f32 = 1.0;

/// Dish cells between soldiers' spots round a crystal (`Game::rim_spots`):
/// a drawn figure's width (`soldier_r`, 1.15 touch radii a side), so each
/// stands clear of the next at zoom 12.
pub(super) const SPOT: f32 = 3.5;
/// Rings of spots round a crystal at most (only the first is in a
/// harvester's reach).
const RIM_RINGS: usize = 4;

/// How far from a goal in rock or a wall `Game::goal` looks for ground,
/// and how far a soldier shut in a wall looks for a way out
/// (`Game::walk`), dish cells.
pub(super) const GOAL_SEARCH: f32 = 8.0;

pub(super) fn cell(w: usize, h: usize, p: (f32, f32)) -> usize {
    let x = (p.0.max(0.0) as usize).min(w - 1);
    let y = (p.1.max(0.0) as usize).min(h - 1);
    y * w + x
}

/// Distances to `goal` over open cells, 5 a side step and 7 a diagonal
/// (a diagonal only when both side cells are open, so no corner is cut);
/// `u32::MAX` where the goal cannot be reached. Built whole (`MapBuild`
/// builds one a slice at a time).
pub fn distance_map(w: usize, h: usize, goal: usize, open: impl Fn(usize) -> bool) -> Vec<u32> {
    let mut b = MapBuild::new(w, h, goal, (0..w * h).map(&open).collect());
    b.advance(w, h, usize::MAX);
    b.map()
}

/// A walking map being built (`distance_map`), any number of cells at a
/// time (`advance`), so a step can build a slice of one (`MAP_POPS`).
/// Dijkstra over a ring of 8 buckets, one a distance mod 8 (steps cost at
/// most 7, so the ring holds only distances `d` to `d + 7` and bucket
/// `d % 8` only `d`), each cell taken in distance order with no heap: the
/// same distances a binary heap gives (`heap_distance_map` in the tests),
/// in about a third of the time on a 6x planet's world (11-14), where a
/// soldier's new map was a step's 210 ms max on the 1070.
pub struct MapBuild {
    pub(super) goal: usize,
    ok: Vec<bool>,
    dist: Vec<u32>,
    ring: [Vec<u32>; 8],
    pending: usize,
    d: u32,
}

impl MapBuild {
    /// A map to `goal` over the cells `ok` marks open.
    pub fn new(w: usize, h: usize, goal: usize, ok: Vec<bool>) -> Self {
        let mut dist = vec![u32::MAX; w * h];
        let mut ring: [Vec<u32>; 8] = Default::default();
        dist[goal] = 0;
        ring[0].push(goal as u32);
        MapBuild { goal, ok, dist, ring, pending: 1, d: 0 }
    }

    /// The distances so far (all of them once `done`).
    pub fn map(self) -> Vec<u32> {
        self.dist
    }

    /// Whether every reachable cell has its distance.
    pub fn done(&self) -> bool {
        self.pending == 0
    }

    /// Take up to `pops` cells off the queue; how many were taken.
    ///
    /// A cell off the world's edge checks its neighbours one by one; one
    /// inside it (nearly all) takes them by fixed offsets with no bounds
    /// checks. The per-neighbour checks were most of a map's cost: a whole
    /// 512 x 320 map took 5.3 to 6.2 ms on the cloud CPU, a step's spike
    /// when a squad was sent somewhere new or the maps went stale (Engine's
    /// bench, 2026-11-21: `pump_maps` 4.2 to 5.8 ms on planet 3).
    pub fn advance(&mut self, w: usize, h: usize, pops: usize) -> usize {
        let mut n = 0;
        // Side steps cost 5, diagonals 7; a diagonal only past two open
        // side cells (`distance_map`).
        let side = [1isize, -1, w as isize, -(w as isize)];
        let diag = [(-1isize, -1isize), (1, -1), (-1, 1), (1, 1)];
        while self.pending > 0 && n < pops {
            let b = (self.d % 8) as usize;
            let Some(i) = self.ring[b].pop() else {
                self.d += 1;
                continue;
            };
            let i = i as usize;
            self.pending -= 1;
            n += 1;
            if self.dist[i] != self.d {
                continue;
            }
            let (x, y) = (i % w, i / w);
            if x == 0 || y == 0 || x + 1 == w || y + 1 == h {
                self.edge(w, h, i);
                continue;
            }
            let ns = self.d + 5;
            let nd = self.d + 7;
            for o in side {
                let j = (i as isize + o) as usize;
                if self.ok[j] && ns < self.dist[j] {
                    self.dist[j] = ns;
                    self.ring[(ns % 8) as usize].push(j as u32);
                    self.pending += 1;
                }
            }
            for (dx, dy) in diag {
                let sx = (i as isize + dx) as usize;
                let sy = (i as isize + dy * w as isize) as usize;
                let j = (sy as isize + dx) as usize;
                if self.ok[j] && self.ok[sx] && self.ok[sy] && nd < self.dist[j] {
                    self.dist[j] = nd;
                    self.ring[(nd % 8) as usize].push(j as u32);
                    self.pending += 1;
                }
            }
        }
        n
    }

    /// `advance`'s step out of cell `i` on the world's edge, checking each
    /// neighbour is in the world.
    fn edge(&mut self, w: usize, h: usize, i: usize) {
        let (x, y) = ((i % w) as i64, (i / w) as i64);
        for (dx, dy) in [(-1i64, 0i64), (1, 0), (0, -1), (0, 1), (-1, -1), (1, -1), (-1, 1), (1, 1)] {
            let (nx, ny) = (x + dx, y + dy);
            if nx < 0 || ny < 0 || nx >= w as i64 || ny >= h as i64 {
                continue;
            }
            let j = ny as usize * w + nx as usize;
            if !self.ok[j] {
                continue;
            }
            let diag = dx != 0 && dy != 0;
            if diag && (!self.ok[y as usize * w + nx as usize] || !self.ok[ny as usize * w + x as usize]) {
                continue;
            }
            let nd = self.d + if diag { 7 } else { 5 };
            if nd < self.dist[j] {
                self.dist[j] = nd;
                self.ring[(nd % 8) as usize].push(j as u32);
                self.pending += 1;
            }
        }
    }
}

/// The neighbour of `i` nearest the goal on `map` (`i` itself at the goal,
/// or when no neighbour is nearer); with `corners` (`Rules::corners`) a
/// diagonal only past two open side cells.
pub(super) fn downhill(w: usize, h: usize, map: &[u32], i: usize, corners: bool) -> usize {
    let (x, y) = ((i % w) as i64, (i / w) as i64);
    let mut best = i;
    for (dx, dy) in [(-1i64, 0i64), (1, 0), (0, -1), (0, 1), (-1, -1), (1, -1), (-1, 1), (1, 1)] {
        let (nx, ny) = (x + dx, y + dy);
        if nx < 0 || ny < 0 || nx >= w as i64 || ny >= h as i64 {
            continue;
        }
        let j = ny as usize * w + nx as usize;
        // A diagonal only past two open side cells, as `distance_map` walks
        // (2026-11-14): a step cutting a wall's corner was blocked in
        // `Game::walk`, and a soldier at a wall's end stood there for good.
        if corners && dx != 0 && dy != 0 && (map[y as usize * w + nx as usize] == u32::MAX || map[ny as usize * w + x as usize] == u32::MAX) {
            continue;
        }
        if map[j] < map[best] {
            best = j;
        }
    }
    best
}

/// `i`, or if walking map `map` doesn't reach it, the nearest cell to it
/// that the map does (within `ONTO_MAP` fine cells; nearest the goal of
/// those as near); None if none is.
/// A point on open ground can sit in a cell the map has no way from: a
/// cell whose middle is in a crystal's disc while the point is just past
/// its rim, or a crystal eaten smaller since the map was built (maps are
/// built every `PATHS_EVERY`). Taken as off the map, a squad sent past a
/// crystal it stood beside stood there for good while its members walked
/// on without it, and a soldier at a crystal's rim stood 77 s 28 cells
/// from its steered squad (pathfinding thread, 2026-11-20: planet 3, run
/// seed 1).
pub(super) fn onto_map(w: usize, h: usize, map: &[u32], i: usize) -> Option<usize> {
    if map[i] != u32::MAX {
        return Some(i);
    }
    let (x, y) = ((i % w) as i64, (i / w) as i64);
    // Rings outward, the first with a cell on the map; in it, the cell
    // nearest the goal (ties to the lower cell).
    for r in 1..=ONTO_MAP {
        let mut best: Option<(u32, usize)> = None;
        for dy in -r..=r {
            for dx in -r..=r {
                if dx.abs().max(dy.abs()) != r {
                    continue;
                }
                let (nx, ny) = (x + dx, y + dy);
                if nx < 0 || ny < 0 || nx >= w as i64 || ny >= h as i64 {
                    continue;
                }
                let j = ny as usize * w + nx as usize;
                if map[j] != u32::MAX && best.is_none_or(|b| (map[j], j) < b) {
                    best = Some((map[j], j));
                }
            }
        }
        if let Some(b) = best {
            return Some(b.1);
        }
    }
    None
}

/// Fine cells round a cell off a walking map that `onto_map` looks for one
/// on it: the band a crystal eaten since the map was built leaves (a
/// standing squad eats one by a few cells in `PATHS_EVERY`; within two
/// cells, at first, a soldier in that band at a crystal's rim stood 24 s).
const ONTO_MAP: i64 = 8;

/// The point of the segment `a`-`b` nearest `p`.
pub(super) fn nearest_on(a: (f32, f32), b: (f32, f32), p: (f32, f32)) -> (f32, f32) {
    let (dx, dy) = (b.0 - a.0, b.1 - a.1);
    let l2 = dx * dx + dy * dy;
    let t = if l2 > 0.0 { (((p.0 - a.0) * dx + (p.1 - a.1) * dy) / l2).clamp(0.0, 1.0) } else { 0.0 };
    (a.0 + dx * t, a.1 + dy * t)
}

/// Soldier `k`'s place round the point it was sent to, dish cells (M3's
/// sunflower: a squad sent to one point spreads instead of stacking),
/// `ring` dish cells a ring (`Game::slot`).
pub(super) fn slot(k: usize, ring: f32) -> (f32, f32) {
    if k == 0 {
        return (0.0, 0.0);
    }
    let r = ring * (k as f32).sqrt();
    let a = k as f32 * 2.399_963;
    (r * a.cos(), r * a.sin())
}

impl<W: World> Game<W> {
    /// Soldier `k`'s sunflower slot (`slot`), dish cells. The ring is 1.5,
    /// or with elbow room (`Rules::elbow`, 2026-11-14) a little over the
    /// separation (1.6 x touch 1.5 = 2.4, so 2.52), so the nearest two
    /// slots (0 and 1) sit as far apart as elbow room asks and it never
    /// pushes a squad at rest off its slots. With elbow room off the slots
    /// are as before, so the engine twins (which turn it off) play as they
    /// did: 2.5 there moved seed 3's core twin 8.6% apart at 25 s.
    pub(super) fn slot(&self, k: usize) -> (f32, f32) {
        slot(k, (self.rules.elbow * self.rules.touch * 1.05).max(1.5))
    }

    /// Where soldier `k` is walking to now, fine cells, if anywhere.
    pub(super) fn goal(&self, k: usize) -> Option<(f32, f32)> {
        self.goal_ground(k).map(|g| g.0)
    }

    /// `goal`, and whether it was moved off rock or a wall onto ground.
    pub(super) fn goal_ground(&self, k: usize) -> Option<((f32, f32), bool)> {
        let s = self.soldiers[k];
        let raw = self.goal_raw(k)?;
        // Round a crystal: its own spot on the rim (`rim_spots`), if it
        // has one for this crystal.
        let spot = self.paths.rim_spot.get(k).copied().flatten().filter(|&p| self.solid_at(raw).is_some_and(|(c, r)| (p.0 - c.0).hypot(p.1 - c.1) < r + (RIM + 4.0 * SPOT) * self.s()));
        let g = spot.unwrap_or_else(|| self.rim(raw, (s.x, s.y)));
        // A goal in rock or a wall (a click on one, or a squad slot past a
        // wall's edge) is the nearest ground a soldier can stand on. Before
        // (2026-11-14), the walking map from a walled-in goal reached no
        // cell, so the soldier froze where it stood.
        if self.standable(g.0, g.1) {
            return Some((g, false));
        }
        Some(self.nearest_standable(g, GOAL_SEARCH * self.s()).map_or((g, false), |q| (q, true)))
    }

    /// The centre of the standable cell nearest `p` within `r` fine cells,
    /// if any (rings of cells outward, so the first ring with one wins).
    pub fn nearest_standable(&self, p: (f32, f32), r: f32) -> Option<(f32, f32)> {
        let (w, h) = self.world.size();
        let (cx, cy) = (p.0.floor() as i64, p.1.floor() as i64);
        let mut best: Option<((f32, f32), f32)> = None;
        for ring in 0..=(r.ceil() as i64) {
            for dy in -ring..=ring {
                for dx in -ring..=ring {
                    if dx.abs().max(dy.abs()) != ring {
                        continue;
                    }
                    let (x, y) = (cx + dx, cy + dy);
                    if x < 0 || y < 0 || x >= w as i64 || y >= h as i64 {
                        continue;
                    }
                    let q = (x as f32 + 0.5, y as f32 + 0.5);
                    let d = (q.0 - p.0).hypot(q.1 - p.1);
                    if d <= r + 0.71 && self.standable(q.0, q.1) && best.is_none_or(|b| d < b.1) {
                        best = Some((q, d));
                    }
                }
            }
            if best.is_some() {
                break;
            }
        }
        best.map(|b| b.0)
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
        // A box first: every standable check (every cell of every
        // straight-walk line) asks this of every crystal, and with `hypot`
        // alone it was most of a 200-soldier step's walking (squad
        // movement rework, 2026-11-16). Then `hypot`, not a squared test,
        // which can differ at a rim's edge in f32: walking maps
        // (`standable_cells`) test the same way, and old replays stay
        // exact (review, 2026-11-16).
        self.solid.iter().copied().find(|&(c, r)| {
            let (dx, dy) = (c.0 - p.0, c.1 - p.1);
            dx.abs() < r && dy.abs() < r && dx.hypot(dy) < r
        })
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
        self.ground(x, y) && self.solid_at((x, y)).is_none()
    }

    /// `standable` at every cell's middle, as a walking map reads it: open
    /// ground, then each solid crystal's disc cleared over its own box, by
    /// `solid_at`'s own test, so the same cells. Testing every crystal at
    /// every cell made a walking map grow with the planet times its
    /// crystals: a step's max on the 1070 was 22 ms at today's area and
    /// 210 ms at 6x when a soldier needed a new map (11-14).
    pub fn standable_cells(&self) -> Vec<bool> {
        let (w, h) = self.world.size();
        let mut ok = self.world.open_cells();
        // A door's cells are walls in the world and ground to soldiers.
        for (o, &d) in ok.iter_mut().zip(&self.doors) {
            *o |= d;
        }
        // Set slime's bulwark (E2b) is no ground: maps route round it as
        // of their build (`Game::mark_bulwark` stales them when it moved).
        for (o, &b) in ok.iter_mut().zip(&self.bulwark) {
            *o &= !b;
        }
        for &(c, r) in &self.solid {
            let span = |m: f32, n: usize| ((m - r - 1.0).floor().max(0.0) as usize, ((m + r + 1.0).ceil().max(0.0) as usize).min(n));
            let ((x0, x1), (y0, y1)) = (span(c.0, w), span(c.1, h));
            for y in y0..y1 {
                for x in x0..x1 {
                    let p = (x as f32 + 0.5, y as f32 + 0.5);
                    if (c.0 - p.0).hypot(c.1 - p.1) < r {
                        ok[y * w + x] = false;
                    }
                }
            }
        }
        ok
    }

    /// The walking map that leads to goal `g` (cell `gi`), by its key in
    /// `Paths::maps`: `gi`'s own, or one to a goal within `near` fine cells
    /// from which `g` is a straight walk. None while it is being built:
    /// the goal is asked for (`pump_maps` builds it) and the caller waits.
    pub(super) fn map_for(&mut self, gi: usize, g: (f32, f32), near: f32) -> Option<usize> {
        let (w, _) = self.world.size();
        let step = self.paths.step;
        let mid = |k: usize| ((k % w) as f32 + 0.5, (k / w) as f32 + 0.5);
        let by = |k: usize| {
            let c = mid(k);
            (c.0 - g.0).hypot(c.1 - g.1)
        };
        if self.rules.map_pops == usize::MAX {
            // The old pace (`Rules::map_pops`): each goal's own map, built
            // whole the step it is asked for.
            if !self.paths.maps.contains_key(&gi) {
                let (w, h) = self.world.size();
                let ok = self.standable_cells();
                let map = distance_map(w, h, gi, |i| ok[i]);
                self.paths.maps.insert(gi, (map, step));
                self.trim_maps();
            }
            self.paths.maps.get_mut(&gi).expect("a built map").1 = step;
            return Some(gi);
        }
        // The goal's own map first. A goal on a cell's edge is as near the
        // next cell's middle, and that map's straight walk can fail (a wall
        // went up in the line): taking only the nearest map, as at first,
        // left the goal asked for and rebuilt every step while its own map
        // sat in the cache (bug check, 2026-11-15).
        if let Some(m) = self.paths.maps.get_mut(&gi) {
            m.1 = step;
            return Some(gi);
        }
        let close = |k: usize| k == gi || (by(k) <= near && self.walk_line(mid(k), g));
        // The nearest built map that serves, ties to the lower cell (the
        // cache is a hash map: its order is not the same run to run).
        let mut near_maps: Vec<usize> = self.paths.maps.keys().copied().filter(|&k| by(k) <= near).collect();
        near_maps.sort_unstable_by(|&a, &b| by(a).total_cmp(&by(b)).then(a.cmp(&b)));
        let built = near_maps.into_iter().find(|&k| close(k));
        if let Some(k) = built {
            self.paths.maps.get_mut(&k).expect("a built map").1 = step;
            return Some(k);
        }
        if self.paths.building.as_ref().is_some_and(|b| close(b.goal)) {
            return None;
        }
        match self.paths.asked.iter().position(|&(k, _)| close(k)) {
            Some(i) => self.paths.asked[i].1 = step,
            None => self.paths.asked.push((gi, step)),
        }
        None
    }

    /// Build walking maps for `MAP_POPS` cells: the one being built, then
    /// at most one more started, goals asked for before maps gone stale.
    /// Once at the start of each step. Every `PATHS_EVERY` (walls wear to
    /// rubble, crystals are eaten) and when a wall goes up (`stale_maps`)
    /// the maps go stale: each is rebuilt in turn and walked meanwhile, so
    /// no soldier stands waiting for a map it had.
    pub(super) fn pump_maps(&mut self) {
        self.paths.step += 1;
        let step = self.paths.step;
        let now = self.world.ticks();
        if now >= self.paths.at + self.secs(PATHS_EVERY) {
            // Maps nobody walked since the last time go; the rest go stale.
            let went = self.paths.went;
            self.paths.maps.retain(|_, m| m.1 >= went);
            self.stale_maps();
            self.paths.at = now;
            self.paths.went = step;
        }
        // An ask not made again last step is dropped: its soldier arrived,
        // died, or moved its goal on.
        self.paths.asked.retain(|&(_, t)| t + 1 >= step);
        let (w, h) = self.world.size();
        let mut pops = self.rules.map_pops;
        // A stale map is walked while it is rebuilt, so with no goal asked
        // for it builds at a quarter of the pace: every `PATHS_EVERY` the
        // maps went stale together and were rebuilt one a step, a run of
        // 5 to 6 ms steps on planet 3 (cloud CPU; Engine's bench,
        // 2026-11-21). A goal someone waits for keeps the whole pace.
        let slow = |g: &Self| g.paths.asked.is_empty() && !g.paths.asked_last;
        if self.paths.building.is_some() && slow(self) {
            pops = pops.min(STALE_POPS);
        }
        let mut started = false;
        loop {
            if self.paths.building.is_none() {
                if started {
                    break;
                }
                // Asked goals and stale maps take turns while both wait:
                // asks first always, as at first, starved the stale maps in
                // a long fight whose goals move every step, so a map made
                // stale by a new wall kept routing soldiers into it (bug
                // check, 2026-11-15).
                let ask = !self.paths.asked.is_empty() && (self.paths.stale.is_empty() || !self.paths.asked_last);
                let next = if ask {
                    Some(self.paths.asked.remove(0).0)
                } else {
                    // The stale map walked last first, ties to the lower cell.
                    let maps = &self.paths.maps;
                    let at = (0..self.paths.stale.len()).max_by_key(|&i| {
                        let k = self.paths.stale[i];
                        (maps.get(&k).map_or(0, |m| m.1), std::cmp::Reverse(k))
                    });
                    at.map(|i| self.paths.stale.swap_remove(i))
                };
                let Some(gi) = next else { break };
                self.paths.asked_last = ask;
                if slow(self) {
                    pops = pops.min(STALE_POPS);
                }
                started = true;
                self.paths.building = Some(MapBuild::new(w, h, gi, self.standable_cells()));
            }
            let b = self.paths.building.as_mut().expect("a map being built");
            pops -= b.advance(w, h, pops);
            if !b.done() {
                break;
            }
            let b = self.paths.building.take().expect("a map being built");
            let read = self.paths.maps.get(&b.goal).map_or(step, |m| m.1);
            self.paths.stale.retain(|&k| k != b.goal);
            if std::mem::take(&mut self.paths.build_stale) {
                self.paths.stale.push(b.goal);
            }
            self.paths.maps.insert(b.goal, (b.dist, read));
            self.trim_maps();
            if pops == 0 {
                break;
            }
        }
    }

    /// Past the cache's size (`MAPS_MAX` maps, `MAP_CELLS_MAX` cells), the
    /// map read longest ago goes, ties to the lower cell. At the old pace
    /// too (`Rules::map_pops`), which had lost its trim (bug check,
    /// 2026-11-15).
    fn trim_maps(&mut self) {
        let (w, h) = self.world.size();
        let most = (MAP_CELLS_MAX / (w * h).max(1)).clamp(4, MAPS_MAX);
        while self.paths.maps.len() > most {
            let old = self.paths.maps.iter().map(|(&k, m)| (m.1, k)).min().expect("maps").1;
            self.paths.maps.remove(&old);
            self.paths.stale.retain(|&k| k != old);
        }
    }

    /// Every walking map goes stale (a wall went up, or `PATHS_EVERY`):
    /// each is rebuilt in turn by `pump_maps` and walked meanwhile; a map
    /// being built starts again on the new ground.
    pub(super) fn stale_maps(&mut self) {
        let mut keys: Vec<usize> = self.paths.maps.keys().copied().collect();
        keys.sort_unstable();
        self.paths.stale = keys;
        self.paths.coarse = None;
        // A map being built finishes on the ground it started on and goes
        // stale then, walked meanwhile like the rest. Starting it again,
        // as at first, starved it: at 6x a wall going up every 6 steps or
        // less left a waiting goal with no map in 600 steps (technical
        // review, 2026-11-15).
        if self.paths.building.is_some() {
            self.paths.build_stale = true;
        }
    }

    /// Each living soldier's own spot round the crystal its goal falls in
    /// (`Paths::rim_spot`), for `goal`: once a step, before the soldiers
    /// walk (`squad_turn`).
    ///
    /// Without them each goal in a crystal went to the rim on its
    /// soldier's own side (`rim`), so a squad harvesting, guarding or
    /// standing round a crystal pressed into one arc of the rim, each
    /// pulled in until its neighbours' elbow room held it: harvesters
    /// stood 2.1 dish cells from their nearest (median; planet 1, run seed
    /// 1, `hand=greedy`), where a drawn figure is 3.45 across, stacked at
    /// zoom 6 and 12 (Direction, from the Look lane's exit test 2,
    /// 2026-11-21). Now the soldiers at one crystal take spots on the ring
    /// just past the rim, `SPOT` apart or closer (down to elbow room) when
    /// the ring must hold them all, the nearest first to the side they
    /// stand on; past that, rings `SPOT` x 0.87 further out, every other
    /// one turned half a spot. They are handed out in the order the
    /// soldiers stand round the crystal, so none crosses another to its
    /// spot. A first try put a second ring out before the first went
    /// round: its soldiers stood out of a harvester's reach, and the
    /// greedy hand's harvest at 60 s fell from 4,081 to 2,040.
    pub(super) fn rim_spots(&mut self) {
        let n = self.soldiers.len();
        let mut spots = vec![None; n];
        if !self.rules.squads || self.solid.is_empty() {
            self.paths.rim_spot = spots;
            return;
        }
        let sc = self.s();
        let sep = SPOT * sc;
        // Who stands round which crystal: its goal falls in the disc.
        let mut at: Vec<Vec<usize>> = vec![Vec::new(); self.solid.len()];
        for k in 0..n {
            if !self.soldiers[k].alive {
                continue;
            }
            let Some(g) = self.goal_raw(k) else { continue };
            if let Some(i) = self.solid.iter().position(|&(c, r)| (g.0 - c.0).hypot(g.1 - c.1) < r) {
                at[i].push(k);
            }
        }
        for (i, ks) in at.iter().enumerate() {
            if ks.is_empty() {
                continue;
            }
            let (c, r) = self.solid[i];
            let ang = |p: (f32, f32)| (p.1 - c.1).atan2(p.0 - c.0);
            // The side they stand on: the way to their middle.
            let (mx, my) = ks.iter().fold((0.0, 0.0), |a, &k| (a.0 + self.soldiers[k].x, a.1 + self.soldiers[k].y));
            let side = (my / ks.len() as f32 - c.1).atan2(mx / ks.len() as f32 - c.0);
            let turn = |a: f32| {
                let d = (a - side).rem_euclid(std::f32::consts::TAU);
                if d > std::f32::consts::PI { d - std::f32::consts::TAU } else { d }
            };
            // Spots, ring by ring, scored by how far round and out. The
            // first ring is the one in a harvester's reach (`turn.rs`: the
            // crystal's whole radius and 2 cells), so it fills all the way
            // round before the next, its spots closing up to elbow room to
            // hold everyone if need be.
            let r0 = r + RIM * sc;
            let tau = std::f32::consts::TAU;
            let tight = self.rules.elbow * self.rules.touch * sc;
            let sep0 = (tau * r0 / ks.len() as f32).clamp(tight.min(sep), sep);
            let mut cand: Vec<(f32, f32, (f32, f32))> = Vec::new();
            for ring in 0..RIM_RINGS {
                let rr = r0 + ring as f32 * 0.87 * sep;
                let sep = if ring == 0 { sep0 } else { sep };
                let m = ((tau * rr / sep).floor() as usize).max(1);
                let step = std::f32::consts::TAU / m as f32;
                let half = if ring % 2 == 1 { 0.5 * step } else { 0.0 };
                for j in 0..m {
                    let a = side + half + j as f32 * step;
                    let p = (c.0 + rr * a.cos(), c.1 + rr * a.sin());
                    if !self.standable(p.0, p.1) {
                        continue;
                    }
                    let t = turn(a);
                    cand.push((t.abs() * r0 + ring as f32 * tau * rr, t, p));
                }
                if cand.len() >= 3 * ks.len() {
                    break;
                }
            }
            cand.sort_by(|a, b| a.0.total_cmp(&b.0));
            cand.truncate(ks.len());
            // In order round the crystal, from the side.
            cand.sort_by(|a, b| a.1.total_cmp(&b.1));
            let mut order: Vec<usize> = ks.clone();
            order.sort_by(|&a, &b| {
                let (pa, pb) = ((self.soldiers[a].x, self.soldiers[a].y), (self.soldiers[b].x, self.soldiers[b].y));
                turn(ang(pa)).total_cmp(&turn(ang(pb))).then(a.cmp(&b))
            });
            for (&k, spot) in order.iter().zip(&cand) {
                spots[k] = Some(spot.2);
            }
        }
        self.paths.rim_spot = spots;
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

    /// Soldier `k`'s place in its squad's formation round `p`, fine cells
    /// (`squads`; it was M3's sunflower by soldier index, `slot(k)`, before
    /// squads).
    pub(super) fn place_at(&self, k: usize, p: (f32, f32)) -> (f32, f32) {
        let o = self.place(k);
        let sc = self.s();
        let q = (p.0 + o.0 * sc, p.1 + o.1 * sc);
        // A formation place in rock or past a wall: the point itself, so
        // a squad squeezing past a wall is not stuck on places it can't
        // stand on (seen on the Fork: a squad steered into the V wall
        // stood still, its anchor waiting for members whose places were
        // in the wall).
        // A column's place (`funnel`) is on the way the anchor came, often
        // round a corner from it: no straight line asked.
        if self.rules.squads && !(self.standable(q.0, q.1) && (self.in_column(k) || self.walk_line(p, q))) {
            p
        } else {
            q
        }
    }

    /// Where soldier `k`'s order points, before a crystal moves it to its
    /// rim (`goal`).
    pub(super) fn goal_raw(&self, k: usize) -> Option<(f32, f32)> {
        let s = self.soldiers[k];
        let sc = self.s();
        // Its place in its squad's formation (`squads`; it was M3's
        // sunflower by soldier index, `slot(k)`, before squads).
        let off = |p: (f32, f32)| self.place_at(k, p);
        match s.order {
            Order::Hold => None,
            // A guard stands round what it guards as a move does: a crystal
            // there moves it to the rim (`goal`).
            Order::Move(x, y) | Order::Fight(x, y) | Order::Guard(x, y) => Some(off((x, y))),
            Order::Build(b) => {
                // Straight at the site until in reach (`BuildRules::reach`),
                // then stand and raise it from there, on the side it came
                // from: no place in a formation round the ghost, so a squad
                // building a turret by the mould stays on its own side of it
                // (owner, 2026-11-14 08:10Z). Before, each builder walked to
                // its sunflower place round the ghost's edge, and half a
                // squad walked round to the far side, toward the mould.
                let p = self.site(k, b)?;
                let (near, far) = self.build_band(b, p);
                let (dx, dy) = (s.x - p.0, s.y - p.1);
                let d = dx.hypot(dy);
                // A wall's builders stand on its home side (`Building::home`),
                // whichever side they came from, so the risen wall leaves
                // nobody with the slime (owner's playtest, 2026-11-16
                // 01:45Z: "some soldiers always end up on the wrong side").
                // Before, each stood on the side it reached the line from,
                // and a squad straddling the line was split by its wall.
                if let Some(sd) = self.wall_side(b, (s.x, s.y)) {
                    if sd < near {
                        let n = self.buildings[b].home;
                        let out = near + 0.5 * sc;
                        let q = (p.0 + n.0 * out, p.1 + n.1 * out);
                        // Only on open ground: `goal_ground` moves a goal
                        // in rock to the nearest standable cell, which can
                        // be past the line (levelgen thread, 11-16); there
                        // it raises from where it stands, as before.
                        if self.standable(q.0, q.1) && self.solid_at(q).is_none() {
                            return Some(q);
                        }
                        // Rock there: the nearest open home-side spot along
                        // the wall, a cell at a time either way (walls
                        // thread, 2026-11-16: with the new walk, seed 8 of
                        // `measure_wall_sides` left 3 builders past a wall
                        // through the squad, their home-side spot rock, so
                        // they raised it from the slime's side).
                        if let Kind::Wall { to } = self.buildings[b].kind {
                            let at = self.buildings[b].at;
                            let l = (to.0 - at.0).hypot(to.1 - at.1).max(1e-3);
                            let u = ((to.0 - at.0) / l, (to.1 - at.1) / l);
                            let here = (p.0 - at.0) * u.0 + (p.1 - at.1) * u.1;
                            for i in 1..=(l / sc).ceil() as i32 {
                                for t in [here - i as f32 * sc, here + i as f32 * sc] {
                                    if !(0.0..=l).contains(&t) {
                                        continue;
                                    }
                                    let q = (at.0 + u.0 * t + n.0 * out, at.1 + u.1 * t + n.1 * out);
                                    if self.standable(q.0, q.1) && self.solid_at(q).is_none() {
                                        return Some(q);
                                    }
                                }
                            }
                        }
                    }
                }
                if d > far {
                    return Some(p);
                }
                if d >= near {
                    return None;
                }
                // Too near: out to the edge on its own side, so the rising
                // ghost and its bar stay in sight (2026-11-12, the
                // build-anywhere look: a lone builder hid its turret), and
                // a wall's builder is not shut inside its own wall when it
                // rises (2026-11-14, owner: "soldier get stuck on the wall
                // they build"). A wall's nearest point makes the way out
                // square to the line.
                let (ux, uy) = if d > 1e-3 {
                    (dx / d, dy / d)
                } else if let Kind::Wall { to } = self.buildings[b].kind {
                    let at = self.buildings[b].at;
                    let l = (to.0 - at.0).hypot(to.1 - at.1).max(1e-3);
                    (-(to.1 - at.1) / l, (to.0 - at.0) / l)
                } else {
                    (1.0, 0.0)
                };
                let out = near + 0.5 * sc;
                Some((p.0 + ux * out, p.1 + uy * out))
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
                let keep = 0.6 * self.gun_range(s.squad as usize) * sc;
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
        // The cells of `squad::path`, in its order, walked without building
        // the list and stopping at the first that fails: every soldier asks
        // a few of these a step (squad movement rework, 2026-11-16).
        let (w, h) = self.world.size();
        let (dx, dy) = (b.0 - a.0, b.1 - a.1);
        let n = (dx.hypot(dy) * 2.0).ceil().max(1.0) as usize;
        let mut last = usize::MAX;
        for k in 0..=n {
            let t = k as f32 / n as f32;
            let (x, y) = (a.0 + dx * t, a.1 + dy * t);
            if x < 0.0 || y < 0.0 || x >= w as f32 || y >= h as f32 {
                break;
            }
            let i = y as usize * w + x as usize;
            if i != last {
                last = i;
                if !self.standable((i % w) as f32 + 0.5, (i / w) as f32 + 0.5) {
                    return false;
                }
            }
        }
        true
    }

    /// Whether every cell on the line from `a` to `b` is clear to fire
    /// across (`World::clear`: open ground or deep water).
    pub fn clear_line(&self, a: (f32, f32), b: (f32, f32)) -> bool {
        let (w, h) = self.world.size();
        squad::path(w, h, a, b).into_iter().all(|i| self.world.clear((i as usize % w) as f32 + 0.5, (i as usize / w) as f32 + 0.5))
    }

    /// Walk soldier `k` toward its goal for `dt` game seconds: straight
    /// when the line is open, else down the goal's distance map (`Paths`),
    /// and along one axis when even that step is blocked. (The first
    /// version only slid along an axis: on seed 1 three of four rushing
    /// soldiers stood 50 cells short of the core behind rock for the
    /// whole game, firing 196 of 200 shots into the rock.)
    pub(super) fn walk(&mut self, k: usize, dt: f32) {
        let Some((g, moved)) = self.goal_ground(k) else { return };
        let s = self.soldiers[k];
        let d = (g.0 - s.x).hypot(g.1 - s.y);
        // A goal moved off rock is one cell for every slot that fell in the
        // rock: near it is there, or those soldiers would shove for it
        // against their elbow room (`elbow_room`) without end.
        let there = if moved { (self.rules.elbow * self.rules.touch * self.s()).max(0.5) } else { 0.5 };
        if d < there || self.engaged(k) {
            return;
        }
        // Caught on a crystal (a larder or ghost placed by one, a crystal
        // grown back): straight out to its near rim first, over anything.
        let caught = self.solid_at((s.x, s.y)).is_some();
        // Shut in a wall or rock (a wall that rose on it, an old save):
        // straight out to the nearest standable ground, over anything.
        let shut = !caught && !self.ground(s.x, s.y);
        let out = if shut { self.nearest_standable((s.x, s.y), GOAL_SEARCH * self.s()) } else { None };
        let g = if let Some(o) = out {
            o
        } else if caught {
            self.rim((s.x, s.y), (s.x, s.y))
        } else if self.walk_line((s.x, s.y), g) {
            g
        } else {
            let (w, h) = self.world.size();
            let gi = cell(w, h, g);
            let Some(key) = self.map_for(gi, g, MAP_NEAR * self.s()) else {
                // Its map is being built (`pump_maps`): wait for it.
                self.map_waits += 1;
                return;
            };
            let map = &self.paths.maps[&key].0;
            // A few cells down the map, so the step heads along the path
            // rather than zig-zagging cell to cell.
            let mut at = cell(w, h, (s.x, s.y));
            for _ in 0..3 {
                at = downhill(w, h, map, at, self.rules.corners);
            }
            // At the map's goal: the soldier's own is a straight walk on.
            if at == key {
                g
            } else {
                ((at % w) as f32 + 0.5, (at / w) as f32 + 0.5)
            }
        };
        let (dx, dy) = (g.0 - s.x, g.1 - s.y);
        let d = dx.hypot(dy).max(1e-6);
        let step = (self.rules.speed * self.s() * dt * self.wade(k)).min(d);
        let (nx, ny) = (s.x + dx / d * step, s.y + dy / d * step);
        let ok = |x: f32, y: f32| if out.is_some() { true } else if caught { self.ground(x, y) } else { self.standable(x, y) };
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

    /// The share of full speed soldier `k` walks at: thick slime holds a
    /// soldier back (`Rules::wade`).
    pub fn wade(&self, k: usize) -> f32 {
        if self.rules.wade > 0.0 {
            // Whole walkers: every one on a soldier holds its legs at any mass (owner, 11-13 15:55Z).
            let x = self.soldiers[k].touching as f32 / self.rules.wade;
            1.0 / (1.0 + x * x)
        } else {
            1.0
        }
    }

    /// Elbow room (`Rules::elbow`, art review 2026-11-14): two living
    /// soldiers closer than `elbow` touch radii each step half the
    /// overlap apart, at most a walk step at their wading speed (at least
    /// half speed, so a knot in thick slime still opens), onto
    /// standable ground only (else along one axis, else not at all), so a
    /// squad walking in one column fans out and every soldier can be
    /// counted. Pushes come from the positions before the pass, so the
    /// result doesn't depend on visiting order; a pair on the very same
    /// point splits along a fixed angle from their indices. O(n²): 40
    /// soldiers are 780 pairs.
    pub(super) fn elbow_room(&mut self, dt: f32) {
        let r = self.rules;
        if r.elbow <= 0.0 {
            return;
        }
        let sc = self.s();
        let min = r.elbow * r.touch * sc;
        let n = self.soldiers.len();
        let mut push = vec![(0.0f32, 0.0f32); n];
        let mut any = false;
        for i in 0..n {
            let a = self.soldiers[i];
            if !a.alive {
                continue;
            }
            for j in i + 1..n {
                let b = self.soldiers[j];
                if !b.alive {
                    continue;
                }
                let (dx, dy) = (a.x - b.x, a.y - b.y);
                let d = dx.hypot(dy);
                if d >= min {
                    continue;
                }
                let (ux, uy) = if d > 1e-4 {
                    (dx / d, dy / d)
                } else {
                    let t = (i * 31 + j) as f32 * 2.399_963;
                    (t.cos(), t.sin())
                };
                let half = (min - d) / 2.0;
                push[i].0 += ux * half;
                push[i].1 += uy * half;
                push[j].0 -= ux * half;
                push[j].1 -= uy * half;
                any = true;
            }
        }
        if !any {
            return;
        }
        for (k, p) in push.into_iter().enumerate() {
            let s = self.soldiers[k];
            let len = p.0.hypot(p.1);
            // A soldier caught on a crystal walks out first (`walk`).
            if len <= 0.0 || !s.alive || self.solid_at((s.x, s.y)).is_some() {
                continue;
            }
            let cap = r.speed * sc * dt * self.wade(k).max(0.5);
            let f = if len > cap { cap / len } else { 1.0 };
            let (nx, ny) = (s.x + p.0 * f, s.y + p.1 * f);
            let to = if self.standable(nx, ny) {
                Some((nx, ny))
            } else if self.standable(nx, s.y) {
                Some((nx, s.y))
            } else if self.standable(s.x, ny) {
                Some((s.x, ny))
            } else {
                None
            };
            if let Some((x, y)) = to {
                self.soldiers[k].x = x;
                self.soldiers[k].y = y;
            }
        }
    }

    /// An attack-moving soldier (`Order::Fight`) with mould it can shoot in
    /// sight, in a squad that stands to fight (`Squad::holding`, decided in
    /// `Game::fight_turn`): it stands and fights instead of walking on.
    /// With too little slime in reach for its squad to stand, it walks on
    /// and shoots as it goes (its stance is free on an attack-move,
    /// `Game::stance_of`, and nothing holds a walking soldier's fire).
    pub(super) fn engaged(&self, k: usize) -> bool {
        self.in_contact(k) && (self.rules.fight_hold <= 0.0 || self.squads.get(self.soldiers[k].squad as usize).is_some_and(|q| q.holding))
    }

    /// Soldier `k` on an attack-move with slime it can shoot: in reach, on
    /// an open line. (Before 2026-11-19 this alone stood it still.)
    pub(super) fn in_contact(&self, k: usize) -> bool {
        let s = self.soldiers[k];
        let Order::Fight(..) = s.order else { return false };
        let Some(aim) = s.aim else { return false };
        s.dense >= self.rules.aim_min && (aim.0 - s.x).hypot(aim.1 - s.y) <= self.gun_range(s.squad as usize) * self.s() && self.clear_line((s.x, s.y), aim)
    }

    /// The depot a carrier at `p` walks home to: the nearest of the larder
    /// and the standing outposts (`Game::drop_offs`).
    pub(super) fn drop_off(&self, p: (f32, f32)) -> (f32, f32) {
        let d = |a: (f32, f32)| (a.0 - p.0).hypot(a.1 - p.1);
        self.drop_offs().into_iter().min_by(|a, b| d(*a).total_cmp(&d(*b))).unwrap_or(self.larder)
    }
}
