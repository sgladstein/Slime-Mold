//! A mould dish of any size: the engine alone, for the GPU plan's phase 2
//! (`Reports/gpu-plan-2026-11-10.md`). Jones 2010's walkers lay trail and
//! steer by it and by food scent, with the rules phase 1 chose for a GPU, so
//! the scaling rule and the look of a dense network can be judged on the CPU
//! before any GPU code exists.
//!
//! What is left out on purpose: the economy (eating, upkeep, births,
//! starving), the census, tools, levels and the wild mould. The walker count
//! is fixed, as in Jones 2010 and in Sage Jenson's sketches. The owner ruled
//! on 2026-11-10 (20:55Z) that today's dishes are unlikely to be the true
//! games, so this is judged by engine measures (`BigMeasure`), not stars.
//!
//! The rules phase 1 settled (2026-11-10):
//! - Walkers move all at once, as a GPU thread per walker would
//!   (`Params::all_at_once`): every walker senses the trail as it stood
//!   before anyone moved; with a crowd limit of 1 a walker may only enter a
//!   cell that was empty when the pass began, and the lower claim key wins
//!   (`keyed`); random draws are keyed on seed, step and walker.
//! - Trail and scent are `f32`. Fixed point erased a smell too faint to
//!   draw that the mould steers by (`Reports/dead-ends.md`,
//!   `Params::fixed_point`).
//! - Scent may live on today's grid while trail and walkers are finer
//!   (`BigParams::scent_coarse`), one of the review's scaling-rule options.
//!
//! Units: a *cell* is a fine cell of this dish; a *dish cell* is one of
//! today's 256 x 160 cells, `scale` cells across. A *tick* is one of today's
//! steps (`STEPS_PER_SECOND` a second); it holds `substeps` fine steps.

use std::f32::consts::TAU;

use super::{keyed, unit, H, STEPS_PER_SECOND, W};

/// What a keyed draw is for here (`keyed`): its own streams, apart from the
/// game's.
const DRAW_TURN: u32 = 101;
const DRAW_BLOCKED: u32 = 102;
const CLAIM_MOVE: u32 = 103;
const DRAW_START: u32 = 104;
const DRAW_PILES: u32 = 105;

#[derive(Clone, Debug)]
pub struct BigParams {
    /// Fine cells per dish cell, each way: 1 is today's grid, 4 is 1024 x 640.
    pub scale: usize,
    /// Fine steps a tick. With `scale` 4 and `substeps` 4 a walker crosses a
    /// dish cell a tick, as today's do.
    pub substeps: usize,
    pub walkers: usize,
    pub sensor_angle: f32,
    pub rotate_angle: f32,
    /// How far ahead a walker senses, in fine cells.
    pub sensor_offset: f32,
    /// How far ahead a walker smells scent, in fine cells; 0 is
    /// `sensor_offset`. Trail sensed near sets the mesh (Jones's network
    /// size follows the sensor offset), scent smelled far sets how far off
    /// food pulls: on a finer grid one offset cannot keep both.
    pub scent_offset: f32,
    /// How far a walker moves a fine step, in fine cells.
    pub step: f32,
    pub deposit: f32,
    /// Trail kept each fine step, after the 3x3 mean.
    pub keep: f32,
    /// 3x3 mean passes on the trail each fine step (`keep` applied once).
    /// One pass spreads trail a quarter as far in dish terms on a grid four
    /// times finer, even with four fine steps a tick; `scale` passes a fine
    /// step give today's spread a tick (variance adds: `scale`^2 passes of a
    /// fine cell's 2/3 equal one pass of a dish cell's).
    pub trail_passes: usize,
    /// Fine steps between a walker's looks: it senses and turns on one, then
    /// walks straight. With `substeps` here a walker turns once a tick and
    /// walks today's path on the fine grid; with 1, a walker turns up to
    /// `rotate_angle` every fine step, `substeps` times as often a dish cell
    /// as today's.
    pub sense_every: usize,
    /// Walkers one cell may hold: 1 is today's rule, 0 no limit (as in
    /// Jenson's sketches, where particles pass through each other).
    pub crowd: u8,
    /// Scent on today's grid, stepped once a tick as today (true), or on the
    /// fine grid, stepped every fine step with the same numbers (false: the
    /// naive port, whose smell reaches `scale` times less far in dish terms).
    pub scent_coarse: bool,
    pub scent_keep: f32,
    /// Scent a food pile lays on each of its cells a scent step, until the
    /// network reaches it; then `fed_scent` of that, as today.
    pub food_deposit: f32,
    pub fed_scent: f32,
    /// Food piles, the first one home (the inoculum sits on it).
    pub piles: usize,
    /// Pile radius, in dish cells.
    pub pile_r: f32,
    /// Walkers start in a blob on the home pile (true) or spread over the
    /// whole dish (false, Jones's start).
    pub blob: bool,
    /// Trail above this counts as tube in the measures (today's `tube`).
    pub tube: f32,
    pub seed: u64,
}

impl BigParams {
    /// Today's numbers on today's grid: the base every scaled dish is judged
    /// against. 2,000 walkers is about today's peak.
    pub fn today() -> Self {
        BigParams {
            scale: 1,
            substeps: 1,
            walkers: 2_000,
            sensor_angle: std::f32::consts::FRAC_PI_4,
            rotate_angle: std::f32::consts::FRAC_PI_4,
            sensor_offset: 9.0,
            scent_offset: 0.0,
            step: 1.0,
            deposit: 5.0,
            keep: 0.9,
            trail_passes: 1,
            sense_every: 1,
            crowd: 1,
            scent_coarse: true,
            scent_keep: 0.97,
            food_deposit: 20.0,
            fed_scent: 0.15,
            piles: 7,
            pile_r: 6.0,
            blob: true,
            tube: 1.0,
            seed: 1,
        }
    }

    pub fn w(&self) -> usize {
        W * self.scale
    }

    pub fn h(&self) -> usize {
        H * self.scale
    }
}

/// The engine's numbers at one moment (`BigDish::measure`), in dish cells
/// and dish seconds so dishes of different scales compare.
#[derive(Clone, Debug, Default)]
pub struct BigMeasure {
    pub seconds: f32,
    /// Area over `tube`, in dish cells.
    pub tube_area: f32,
    /// Holes in the tube of at least one dish cell: the network's loops.
    pub loops: usize,
    /// Mean tube width, in dish cells: twice the area over the edge length.
    pub width: f32,
    /// Farthest tube from the home pile's centre, in dish cells.
    pub reach: f32,
    /// Piles the tube has reached, home included.
    pub reached: usize,
    /// Share of walkers standing where the trail is under `tube`.
    pub strays: f32,
}

#[derive(Clone)]
pub struct BigDish {
    pub p: BigParams,
    pub w: usize,
    pub h: usize,
    /// Ticks run.
    pub ticks: u64,
    /// Fine steps run: the step the keyed draws use.
    pub steps: u64,
    /// Fine grid: true where there is agar.
    pub agar: Vec<bool>,
    pub trail: Vec<f32>,
    scratch: Vec<f32>,
    /// The scent grid: today's (`scent_coarse`) or the fine one.
    pub sw: usize,
    pub sh: usize,
    pub scent_agar: Vec<bool>,
    pub scent: Vec<f32>,
    scent_scratch: Vec<f32>,
    /// x, y (fine cells), heading.
    pub walkers: Vec<(f32, f32, f32)>,
    occ: Vec<u16>,
    claim: Vec<u32>,
    /// Pile centres in dish cells, and whether the tube has reached each.
    pub piles: Vec<(f32, f32)>,
    pub reached: Vec<bool>,
    /// Threads for the field passes. Each cell is computed from the last
    /// field alone, so the result does not depend on it.
    pub threads: usize,
}

/// Agar inside an ellipse three dish cells in from the edge, as today's
/// `agar_ground`, at any size.
fn ellipse(w: usize, h: usize, margin: f32) -> Vec<bool> {
    let (cx, cy) = (w as f32 / 2.0, h as f32 / 2.0);
    let (rx, ry) = (cx - margin, cy - margin);
    let mut g = vec![false; w * h];
    for y in 0..h {
        for x in 0..w {
            let dx = (x as f32 + 0.5 - cx) / rx;
            let dy = (y as f32 + 0.5 - cy) / ry;
            g[y * w + x] = dx * dx + dy * dy <= 1.0;
        }
    }
    g
}

/// The 3x3 mean over agar neighbours, times `keep`, for rows `y0..y1` of
/// `to` (which holds just those rows). Today's `spread` without the lamp.
fn spread_rows(agar: &[bool], from: &[f32], to: &mut [f32], w: usize, h: usize, keep: f32, y0: usize) {
    let rows = to.len() / w;
    for r in 0..rows {
        let y = y0 + r;
        for x in 0..w {
            let i = y * w + x;
            let o = r * w + x;
            if !agar[i] || x == 0 || y == 0 || x + 1 == w || y + 1 == h {
                to[o] = 0.0;
                continue;
            }
            let mut sum = 0.0;
            let mut n = 0.0;
            for row in [i - w, i, i + w] {
                for j in [row - 1, row, row + 1] {
                    if agar[j] {
                        sum += from[j];
                        n += 1.0;
                    }
                }
            }
            to[o] = sum / n * keep;
        }
    }
}

/// `spread_rows` over the whole field, split into bands across `threads`.
fn spread(agar: &[bool], from: &[f32], to: &mut [f32], w: usize, h: usize, keep: f32, threads: usize) {
    let threads = threads.clamp(1, h);
    if threads == 1 {
        spread_rows(agar, from, to, w, h, keep, 0);
        return;
    }
    let band = h.div_ceil(threads);
    std::thread::scope(|s| {
        for (k, chunk) in to.chunks_mut(band * w).enumerate() {
            s.spawn(move || spread_rows(agar, from, chunk, w, h, keep, k * band));
        }
    });
}

impl BigDish {
    pub fn new(p: BigParams) -> Self {
        let (w, h) = (p.w(), p.h());
        let s = p.scale as f32;
        let agar = ellipse(w, h, 3.0 * s);
        let (sw, sh) = if p.scent_coarse { (W, H) } else { (w, h) };
        let scent_agar = if p.scent_coarse { ellipse(W, H, 3.0) } else { agar.clone() };
        let mut d = BigDish {
            w,
            h,
            ticks: 0,
            steps: 0,
            trail: vec![0.0; w * h],
            scratch: vec![0.0; w * h],
            agar,
            sw,
            sh,
            scent: vec![0.0; sw * sh],
            scent_scratch: vec![0.0; sw * sh],
            scent_agar,
            walkers: Vec::with_capacity(p.walkers),
            occ: vec![0; w * h],
            claim: vec![u32::MAX; w * h],
            piles: Vec::new(),
            reached: Vec::new(),
            threads: 1,
            p,
        };
        d.place_piles();
        d.place_walkers();
        d
    }

    fn cell(&self, x: f32, y: f32) -> Option<usize> {
        if x < 0.0 || y < 0.0 {
            return None;
        }
        let (cx, cy) = (x as usize, y as usize);
        (cx < self.w && cy < self.h).then(|| cy * self.w + cx)
    }

    /// Home on the left, the rest spread by keyed draws, at least 28 dish
    /// cells apart and clear of the rim: the same layout at every scale for
    /// one seed, so arms pair.
    fn place_piles(&mut self) {
        let seed = self.p.seed;
        let r = self.p.pile_r;
        let (cx, cy, rx, ry) = (W as f32 / 2.0, H as f32 / 2.0, W as f32 / 2.0 - 3.0, H as f32 / 2.0 - 3.0);
        let inside = |x: f32, y: f32| {
            let (dx, dy) = ((x - cx) / (rx - r - 4.0), (y - cy) / (ry - r - 4.0));
            dx * dx + dy * dy <= 1.0
        };
        self.piles.push((W as f32 * 0.22, H as f32 * 0.5));
        let mut k = 0u32;
        while self.piles.len() < self.p.piles && k < 100_000 {
            let x = unit(keyed(seed, 0, DRAW_PILES, 2 * k)) * W as f32;
            let y = unit(keyed(seed, 0, DRAW_PILES, 2 * k + 1)) * H as f32;
            k += 1;
            if inside(x, y) && self.piles.iter().all(|&(px, py)| (px - x).hypot(py - y) >= 28.0) {
                self.piles.push((x, y));
            }
        }
        self.reached = vec![false; self.piles.len()];
        self.reached[0] = true;
    }

    fn place_walkers(&mut self) {
        let (seed, n, s) = (self.p.seed, self.p.walkers, self.p.scale as f32);
        let (hx, hy) = (self.piles[0].0 * s, self.piles[0].1 * s);
        // A blob as dense as today's inoculum: about 0.9 of the cells in it.
        let r = (n as f32 / (0.9 * std::f32::consts::PI)).sqrt();
        let mut k = 0u32;
        while self.walkers.len() < n && k < 50 * n as u32 + 1000 {
            let (a, b, c) = (
                unit(keyed(seed, 1, DRAW_START, 3 * k)),
                unit(keyed(seed, 1, DRAW_START, 3 * k + 1)),
                unit(keyed(seed, 1, DRAW_START, 3 * k + 2)),
            );
            k += 1;
            let (x, y) = if self.p.blob {
                let (ang, dist) = (a * TAU, b.sqrt() * r);
                (hx + ang.cos() * dist, hy + ang.sin() * dist)
            } else {
                (a * self.w as f32, b * self.h as f32)
            };
            let Some(i) = self.cell(x, y) else { continue };
            if !self.agar[i] || (self.p.crowd > 0 && self.occ[i] >= self.p.crowd as u16) {
                continue;
            }
            self.occ[i] += 1;
            self.walkers.push((x, y, c * TAU));
        }
    }

    /// Scent under a fine position, from whichever grid holds it.
    fn scent_at(&self, x: f32, y: f32) -> f32 {
        let (x, y) = if self.p.scent_coarse {
            (x / self.p.scale as f32, y / self.p.scale as f32)
        } else {
            (x, y)
        };
        if x < 0.0 || y < 0.0 || x >= self.sw as f32 || y >= self.sh as f32 {
            return 0.0;
        }
        self.scent[y as usize * self.sw + x as usize]
    }

    /// Trail at `so` and scent at `sso` along heading `a`: off the agar
    /// (by the trail sensor) reads as a wall.
    fn sense(&self, x: f32, y: f32, a: f32, so: f32, sso: f32) -> f32 {
        let (c, s) = (a.cos(), a.sin());
        match self.cell(x + c * so, y + s * so) {
            Some(i) if self.agar[i] => self.trail[i] + self.scent_at(x + c * sso, y + s * sso),
            _ => -1.0e6,
        }
    }

    /// One fine step of the walkers: sense, turn, claim, move, deposit.
    fn move_walkers(&mut self) {
        let p = &self.p;
        let (sa, ra, so, st, dep, crowd) = (p.sensor_angle, p.rotate_angle, p.sensor_offset, p.step, p.deposit, p.crowd);
        let sso = if p.scent_offset > 0.0 { p.scent_offset } else { so };
        let (seed, t) = (p.seed, self.steps);
        // (walker, from, to, x, y, heading); from == to for a move within a cell.
        let mut moves: Vec<(u32, u32, u32, f32, f32, f32)> = Vec::with_capacity(self.walkers.len());
        let mut blocked: Vec<u32> = Vec::new();
        let look = t.is_multiple_of(p.sense_every.max(1) as u64);
        for k in 0..self.walkers.len() {
            let (x, y, mut h) = self.walkers[k];
            if look {
                let f = self.sense(x, y, h, so, sso);
                let fl = self.sense(x, y, h - sa, so, sso);
                let fr = self.sense(x, y, h + sa, so, sso);
                if f > fl && f > fr {
                } else if f < fl && f < fr {
                    h += if keyed(seed, t, DRAW_TURN, k as u32) & 1 == 0 { ra } else { -ra };
                } else if fl < fr {
                    h += ra;
                } else if fr < fl {
                    h -= ra;
                }
            }
            let (nx, ny) = (x + h.cos() * st, y + h.sin() * st);
            let here = self.cell(x, y).expect("walker inside the dish");
            match self.cell(nx, ny) {
                Some(there) if there == here || (self.agar[there] && (crowd == 0 || self.occ[there] < crowd as u16)) => {
                    if there != here && crowd > 0 {
                        let key = keyed(seed, t, CLAIM_MOVE, k as u32);
                        self.claim[there] = self.claim[there].min(key);
                    }
                    moves.push((k as u32, here as u32, there as u32, nx, ny, h));
                }
                _ => blocked.push(k as u32),
            }
        }
        for &(k, from, to, nx, ny, h) in &moves {
            let (i, j) = (from as usize, to as usize);
            let wins = i == j || crowd == 0 || self.claim[j] == keyed(seed, t, CLAIM_MOVE, k);
            if wins {
                self.occ[i] -= 1;
                self.occ[j] += 1;
                self.walkers[k as usize] = (nx, ny, h);
                self.trail[j] += dep;
            } else {
                blocked.push(k);
            }
        }
        if crowd > 0 {
            for &(_, _, to, ..) in &moves {
                self.claim[to as usize] = u32::MAX;
            }
        }
        // Blocked: Jones's rule, stay put and pick a new heading.
        for &k in &blocked {
            self.walkers[k as usize].2 = unit(keyed(seed, t, DRAW_BLOCKED, k)) * TAU;
        }
    }

    /// Piles lay scent on the scent grid: full until the tube reaches them,
    /// then `fed_scent` of that, as today's `deposit_food` does.
    fn lay_scent(&mut self) {
        let k = if self.p.scent_coarse { 1.0 } else { self.p.scale as f32 };
        let r = self.p.pile_r * k;
        let n = r.ceil() as i32;
        for (pi, &(px, py)) in self.piles.iter().enumerate() {
            let strength = self.p.food_deposit * if self.reached[pi] { self.p.fed_scent } else { 1.0 };
            let (cx, cy) = (px * k, py * k);
            for dy in -n..=n {
                for dx in -n..=n {
                    if ((dx * dx + dy * dy) as f32) > r * r {
                        continue;
                    }
                    let (x, y) = (cx + dx as f32, cy + dy as f32);
                    if x < 0.0 || y < 0.0 || x >= self.sw as f32 || y >= self.sh as f32 {
                        continue;
                    }
                    let i = y as usize * self.sw + x as usize;
                    if self.scent_agar[i] {
                        self.scent[i] += strength;
                    }
                }
            }
        }
    }

    fn scent_step(&mut self) {
        self.lay_scent();
        spread(&self.scent_agar, &self.scent, &mut self.scent_scratch, self.sw, self.sh, self.p.scent_keep, self.threads);
        std::mem::swap(&mut self.scent, &mut self.scent_scratch);
    }

    /// The scent the piles lay on each scent cell in one scent step, as
    /// `lay_scent` adds it, for the piles reached so far. The GPU timing
    /// harness (`examples/gputime.rs`) uploads it once.
    pub fn scent_laid(&self) -> Vec<f32> {
        let mut out = vec![0.0f32; self.sw * self.sh];
        let k = if self.p.scent_coarse { 1.0 } else { self.p.scale as f32 };
        let r = self.p.pile_r * k;
        let n = r.ceil() as i32;
        for (pi, &(px, py)) in self.piles.iter().enumerate() {
            let strength = self.p.food_deposit * if self.reached[pi] { self.p.fed_scent } else { 1.0 };
            let (cx, cy) = (px * k, py * k);
            for dy in -n..=n {
                for dx in -n..=n {
                    if ((dx * dx + dy * dy) as f32) > r * r {
                        continue;
                    }
                    let (x, y) = (cx + dx as f32, cy + dy as f32);
                    if x < 0.0 || y < 0.0 || x >= self.sw as f32 || y >= self.sh as f32 {
                        continue;
                    }
                    let i = y as usize * self.sw + x as usize;
                    if self.scent_agar[i] {
                        out[i] += strength;
                    }
                }
            }
        }
        out
    }

    /// A pile counts as reached once tube lies on it.
    pub fn update_reached(&mut self) {
        let s = self.p.scale as f32;
        let r = self.p.pile_r * s;
        let n = r.ceil() as i32;
        for pi in 0..self.piles.len() {
            if self.reached[pi] {
                continue;
            }
            let (cx, cy) = (self.piles[pi].0 * s, self.piles[pi].1 * s);
            'pile: for dy in -n..=n {
                for dx in -n..=n {
                    if ((dx * dx + dy * dy) as f32) > r * r {
                        continue;
                    }
                    if let Some(i) = self.cell(cx + dx as f32, cy + dy as f32) {
                        if self.trail[i] > self.p.tube {
                            self.reached[pi] = true;
                            break 'pile;
                        }
                    }
                }
            }
        }
    }

    /// One tick: `substeps` fine steps of walkers and trail, and the scent
    /// once (on today's grid) or each fine step (on the fine grid).
    pub fn tick(&mut self) {
        for _ in 0..self.p.substeps {
            self.move_walkers();
            for pass in 0..self.p.trail_passes.max(1) {
                let keep = if pass == 0 { self.p.keep } else { 1.0 };
                spread(&self.agar, &self.trail, &mut self.scratch, self.w, self.h, keep, self.threads);
                std::mem::swap(&mut self.trail, &mut self.scratch);
            }
            if !self.p.scent_coarse {
                self.scent_step();
            }
            self.steps += 1;
        }
        if self.p.scent_coarse {
            self.scent_step();
        }
        self.update_reached();
        self.ticks += 1;
    }

    pub fn seconds(&self) -> f32 {
        self.ticks as f32 / STEPS_PER_SECOND as f32
    }

    pub fn measure(&self) -> BigMeasure {
        let (w, h) = (self.w, self.h);
        let s = self.p.scale as f32;
        let tube: Vec<bool> = self.trail.iter().zip(&self.agar).map(|(&v, &a)| a && v > self.p.tube).collect();
        let area = tube.iter().filter(|&&t| t).count();
        // Edge: tube cells beside a cell that is not tube.
        let mut edge = 0usize;
        let (hx, hy) = (self.piles[0].0 * s, self.piles[0].1 * s);
        let mut reach = 0.0f32;
        for y in 1..h - 1 {
            for x in 1..w - 1 {
                let i = y * w + x;
                if !tube[i] {
                    continue;
                }
                if !(tube[i - 1] && tube[i + 1] && tube[i - w] && tube[i + w]) {
                    edge += 1;
                }
                reach = reach.max((x as f32 + 0.5 - hx).hypot(y as f32 + 0.5 - hy));
            }
        }
        // Loops: 4-connected pieces of agar that is not tube, not touching
        // the rim, of at least one dish cell.
        let mut seen = vec![false; w * h];
        let mut loops = 0;
        let min = (s * s) as usize;
        let mut stack = Vec::new();
        for start in 0..w * h {
            if seen[start] || tube[start] || !self.agar[start] {
                continue;
            }
            seen[start] = true;
            stack.push(start);
            let (mut size, mut open) = (0usize, false);
            while let Some(i) = stack.pop() {
                size += 1;
                let (x, y) = (i % w, i / w);
                for (ok, j) in [(x > 0, i.wrapping_sub(1)), (x + 1 < w, i + 1), (y > 0, i.wrapping_sub(w)), (y + 1 < h, i + w)] {
                    if !ok {
                        open = true;
                        continue;
                    }
                    if !self.agar[j] {
                        open = true;
                    } else if !tube[j] && !seen[j] {
                        seen[j] = true;
                        stack.push(j);
                    }
                }
            }
            if !open && size >= min {
                loops += 1;
            }
        }
        let strays = self
            .walkers
            .iter()
            .filter(|&&(x, y, _)| self.cell(x, y).is_some_and(|i| self.trail[i] <= self.p.tube))
            .count();
        BigMeasure {
            seconds: self.seconds(),
            tube_area: area as f32 / (s * s),
            loops,
            width: if edge > 0 { 2.0 * area as f32 / edge as f32 / s } else { 0.0 },
            reach: reach / s,
            reached: self.reached.iter().filter(|&&r| r).count(),
            strays: strays as f32 / self.walkers.len().max(1) as f32,
        }
    }

    /// The trail as a picture, one pixel a fine cell, `down` fine cells to a
    /// pixel each way (the brightest of them). Brightness follows the trail
    /// smoothly, with no tube edge drawn (the owner's rule: the picture is
    /// the data); the piles show as their scent's faint glow.
    pub fn image(&self, down: usize) -> image::RgbaImage {
        let down = down.max(1);
        let (iw, ih) = ((self.w / down) as u32, (self.h / down) as u32);
        let mut img = image::RgbaImage::new(iw, ih);
        for py in 0..ih {
            for px in 0..iw {
                let (mut v, mut inside) = (0.0f32, false);
                for dy in 0..down {
                    for dx in 0..down {
                        let i = (py as usize * down + dy) * self.w + px as usize * down + dx;
                        v = v.max(self.trail[i]);
                        inside |= self.agar[i];
                    }
                }
                let c = (px as f32 + 0.5) * down as f32;
                let r = (py as f32 + 0.5) * down as f32;
                let sc = self.scent_at(c, r);
                let px_rgb = if !inside {
                    [8, 14, 18]
                } else {
                    // Trail: a soft knee, so faint wisps show and tubes glow.
                    let a = 1.0 - (-v / 6.0).exp();
                    let g = 1.0 - (-sc / 60.0).exp();
                    let base = [12.0 + 30.0 * g, 22.0 + 26.0 * g, 26.0 + 10.0 * g];
                    let hot = [255.0, 190.0, 90.0];
                    let mut out = [0u8; 3];
                    for k in 0..3 {
                        out[k] = (base[k] + (hot[k] - base[k]) * a).clamp(0.0, 255.0) as u8;
                    }
                    out
                };
                img.put_pixel(px, py, image::Rgba([px_rgb[0], px_rgb[1], px_rgb[2], 255]));
            }
        }
        img
    }

    /// A hash of the walkers and the trail, for the tests: the same seed and
    /// numbers must give the same dish, whatever `threads` is.
    pub fn hash(&self) -> u64 {
        let mut h: u64 = 0xCBF2_9CE4_8422_2325;
        let mut eat = |v: u32| {
            for b in v.to_le_bytes() {
                h ^= b as u64;
                h = h.wrapping_mul(0x0100_0000_01B3);
            }
        };
        for &(x, y, a) in &self.walkers {
            eat(x.to_bits());
            eat(y.to_bits());
            eat(a.to_bits());
        }
        for v in self.trail.iter().chain(&self.scent) {
            eat(v.to_bits());
        }
        h
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn small(scale: usize, substeps: usize) -> BigParams {
        BigParams { scale, substeps, walkers: 600 * scale * scale, seed: 3, ..BigParams::today() }
    }

    /// Threads split the field passes by rows; the dish must not care.
    #[test]
    fn threads_change_nothing() {
        let mut a = BigDish::new(small(2, 2));
        let mut b = a.clone();
        b.threads = 4;
        for _ in 0..60 {
            a.tick();
            b.tick();
        }
        assert_eq!(a.hash(), b.hash());
        assert_eq!(a.hash(), {
            let mut c = BigDish::new(small(2, 2));
            for _ in 0..60 {
                c.tick();
            }
            c.hash()
        });
    }

    /// With a crowd limit of 1 no cell ever holds two walkers, and no claim
    /// is left behind.
    #[test]
    fn one_walker_a_cell() {
        let mut d = BigDish::new(small(2, 2));
        for _ in 0..120 {
            d.tick();
            let mut seen = vec![0u8; d.w * d.h];
            for &(x, y, _) in &d.walkers {
                let i = d.cell(x, y).unwrap();
                assert!(d.agar[i]);
                seen[i] += 1;
                assert!(seen[i] == 1, "two walkers in one cell");
            }
            assert!(d.occ.iter().zip(&seen).all(|(&o, &s)| o == s as u16));
            assert!(d.claim.iter().all(|&c| c == u32::MAX));
        }
    }

    /// The scaling options keep the dish deterministic whatever the threads.
    #[test]
    fn scaling_options_ignore_threads() {
        let p = BigParams { trail_passes: 2, sense_every: 2, scent_offset: 18.0, ..small(2, 2) };
        let mut a = BigDish::new(p);
        let mut b = a.clone();
        b.threads = 3;
        for _ in 0..40 {
            a.tick();
            b.tick();
        }
        assert_eq!(a.hash(), b.hash());
    }

    /// A scent reach of 0 means the trail sensor's reach, bit for bit.
    #[test]
    fn scent_offset_zero_is_the_sensor_offset() {
        let p = small(2, 2);
        let mut a = BigDish::new(p.clone());
        let mut b = BigDish::new(BigParams { scent_offset: p.sensor_offset, ..p });
        for _ in 0..30 {
            a.tick();
            b.tick();
        }
        assert_eq!(a.hash(), b.hash());
    }

    /// Between looks a walker keeps its heading; only a blocked walker,
    /// which stays put, draws a new one.
    #[test]
    fn walkers_walk_straight_between_looks() {
        // Spread start: a packed blob leaves most walkers blocked.
        let mut d = BigDish::new(BigParams { sense_every: 2, blob: false, ..small(2, 2) });
        for _ in 0..20 {
            d.tick();
        }
        assert_eq!(d.steps % 2, 0);
        d.steps += 1;
        let before = d.walkers.clone();
        d.move_walkers();
        let mut moved = 0;
        for (a, b) in before.iter().zip(&d.walkers) {
            if (a.0, a.1) != (b.0, b.1) {
                assert_eq!(a.2, b.2, "turned between looks");
                moved += 1;
            }
        }
        assert!(moved > before.len() / 2, "{moved} of {} moved", before.len());
    }

    /// The same layout of piles at every scale, so arms pair.
    #[test]
    fn piles_pair_across_scales() {
        let a = BigDish::new(small(1, 1));
        let b = BigDish::new(small(4, 4));
        assert_eq!(a.piles, b.piles);
        assert!(a.piles.len() >= 5);
    }

    /// On today's grid the network grows out from home and reaches food.
    #[test]
    fn the_mould_spreads_and_finds_food() {
        let mut d = BigDish::new(BigParams { seed: 2, ..BigParams::today() });
        for _ in 0..30 * STEPS_PER_SECOND {
            d.tick();
        }
        let m = d.measure();
        assert!(m.tube_area > 500.0, "{m:?}");
        assert!(m.reached >= 2, "{m:?}");
    }
}
