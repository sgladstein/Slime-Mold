//! **The squad's hooks into the world** (Larder World W2a, 2026-11-11):
//! what soldiers do to the big dish and read from it each frame, as plain
//! functions on `BigDish`. They are the CPU twins of the GPU engine's
//! `shots`, `strike`, `cut`, `smell`, `units` and `harvest` passes
//! (`gpu::Engine::set_squad`), which the one-frame twin test in
//! `src/bin/mouldgpu.rs` holds to these. Plan: the roadmap report's
//! *W2 plan* (`Reports/larder-world-roadmap-2026-11-11.md`).
//!
//! **A frame**, on both sides, in this order:
//! 1. Shots. Each shot walks its line (`path`) cell by cell on the trail
//!    as the frame starts, and stops at the first cell that looks like
//!    slime (trail `slime` or more: `slime_trail`, the drawn thickness
//!    26/255) or at the first cell off the agar (a wall or rock). The owner,
//!    2026-11-11 16:48Z: "Gun fire should hit the first slime in its
//!    path". Every hit then strikes: the living walkers on cells within its
//!    radius die (`BigDish::kill`; a walker in two shots' discs counts for
//!    the first), and the trail there is cleared. Every shot reads the
//!    trail before any strike, so shots in one frame don't see each other.
//!    With the alarm field on (a mould tier, `mould::tiers`), a hit also
//!    lays `SHOT_ALARM` within twice its radius (on the GPU the `shout`
//!    pass, shots in order).
//! 2. Smell. Each soldier adds `smell` to the scent on the agar cells of
//!    its disc, so the mould grows toward the squad (M3's rule).
//! 3. The frame's ticks.
//! 4. Reads. Each unit counts the walkers on the cells within `touch` (the
//!    mould's bite) and finds the 4 x 4 block of cells with the most
//!    walkers whose centre is within `sight` (where it aims). Then, in unit
//!    order, each unit with `take` takes up to that much charge from the
//!    crystal cells within `take_r`, cell by cell (harvest).
//!
//! **Exact on both sides.** Every disc is a set of whole cells: the unit's
//! cell (its position, floored) and the cells whose offset (dx, dy) has
//! dx² + dy² at most the squared radius, floored to an integer. A shot's
//! cells are listed here and sent to the GPU. So the CPU and the GPU test
//! the same integers and agree to the bit, where float distances could
//! differ in the last place.

use super::big::{BigDish, LIFE_DEAD, LIFE_SPORE};

/// Most units, shots and shot cells in one frame (the GPU's buffer sizes).
pub const UNITS_MAX: usize = 96;
pub const SHOTS_MAX: usize = 256;
pub const PATH_MAX: usize = 16_384;
/// The side of the blocks a unit aims at, in cells.
pub const BIN: usize = 4;
/// The widest strike, in cells: the GPU clears a shot's trail over the
/// 15 x 15 box round its hit (one workgroup a shot), so the cost is the
/// shots', not the world's. Radii above it are cut to it on both sides.
pub const R_MAX: f32 = 7.0;
/// The drawn thickness that reads as slime (the graphics thread, 2026-11-11
/// 18:57Z: "on screen a cell reads as slime from about 26/255"; a test at
/// 60/255 let 98 of 120 shots fly through visible slime).
pub const SLIME_LOOK: f32 = 26.0 / 255.0;
const NONE: u32 = u32::MAX;
/// The alarm a hit lays on each agar cell within `SHOT_ALARM_R` times its
/// radius, when the alarm field is on: `examples/bigdish.rs`'s strike
/// default (`strike_alarm=50`, reaching twice the strike's radius), the
/// one the tiers were measured with (`Reports/mould-tiers-2026-11-11.md`).
pub const SHOT_ALARM: f32 = 50.0;
pub const SHOT_ALARM_R: f32 = 2.0;

/// The trail that the game curve draws at `SLIME_LOOK` (`1 - exp(-v /
/// knee)`, the GPU's `bright` with `curve` 0).
pub fn slime_trail(knee: f32) -> f32 {
    -knee * (1.0 - SLIME_LOOK).ln()
}

/// One unit for a frame, in fine cells. A radius of 0 is its own cell
/// only; `take` 0 harvests nothing and `smell` 0 lays nothing.
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct UnitIn {
    pub x: f32,
    pub y: f32,
    pub touch: f32,
    pub sight: f32,
    pub take_r: f32,
    pub take: f32,
    pub smell_r: f32,
    pub smell: f32,
}

/// One shot: its line and the radius it strikes at the first slime (at
/// most `R_MAX`).
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct ShotIn {
    pub from: (f32, f32),
    pub to: (f32, f32),
    pub r: f32,
}

#[derive(Clone, Debug, Default, PartialEq)]
pub struct Frame {
    pub units: Vec<UnitIn>,
    pub shots: Vec<ShotIn>,
    /// The trail that counts as slime (`slime_trail`).
    pub slime: f32,
    /// Fine steps of crater each hit lays on the agar cells of its strike
    /// disc (`BigDish::scorch_disc`'s field: the trail there is cut each
    /// fine step while it lasts, `BigParams::scorch_keep`). 0 lays none.
    /// Why (owner, 2026-11-12 04:39Z): "A group of soldiers shooting the
    /// slime should start carving out a bigger and bigger hole that doesn't
    /// refill while being shot at". A shot stops at the first cell that
    /// reads as slime, so while the craters keep the trail under that, the
    /// next shots fly on into the body and the hole deepens; once fire
    /// stops, the craters fade and the mould grows back. On the GPU the
    /// `cut` pass lays it into the damage memory (binding 47).
    pub crater: u16,
}

#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct UnitOut {
    /// Walkers on the cells within `touch`.
    pub touching: u32,
    /// The centre of the block with the most walkers within `sight`.
    pub target: Option<(f32, f32)>,
    /// Charge harvested.
    pub took: f32,
    /// Walkers in that block, so a game can hold fire at a thin one
    /// (`larder_world::Rules::aim_min`).
    pub dense: u32,
}

#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct ShotOut {
    /// The centre of the slime cell it hit (none: it hit a wall or rock,
    /// left the world, or found no slime on its line).
    pub hit: Option<(f32, f32)>,
    pub killed: u32,
}

#[derive(Clone, Debug, Default, PartialEq)]
pub struct FrameOut {
    pub units: Vec<UnitOut>,
    pub shots: Vec<ShotOut>,
}

/// The largest r with r² at most `r2`.
pub fn isqrt(r2: u32) -> i64 {
    let mut r = 0i64;
    while (r + 1) * (r + 1) <= r2 as i64 {
        r += 1;
    }
    r
}

/// A radius as the integer its square is tested against.
pub fn r2(r: f32) -> u32 {
    (r.max(0.0) * r.max(0.0)).floor() as u32
}

/// The cell a point is in, clamped to the world.
pub fn cell_of(w: usize, h: usize, x: f32, y: f32) -> (i64, i64) {
    ((x.floor() as i64).clamp(0, w as i64 - 1), (y.floor() as i64).clamp(0, h as i64 - 1))
}

/// The cells of a disc round cell (`cx`, `cy`), in rows from the top and
/// left to right in a row (the GPU's order), clipped to the world.
pub fn disc(w: usize, h: usize, cx: i64, cy: i64, r2: u32) -> impl Iterator<Item = usize> {
    let r = isqrt(r2);
    let (w, h) = (w as i64, h as i64);
    (-r..=r).flat_map(move |dy| {
        (-r..=r).filter_map(move |dx| {
            let (x, y) = (cx + dx, cy + dy);
            (x >= 0 && y >= 0 && x < w && y < h && dx * dx + dy * dy <= r2 as i64).then_some((y * w + x) as usize)
        })
    })
}

/// The cells a shot's line crosses, in order, each once: the line sampled
/// every half cell (as `world::look::first_hit` marches it), stopping at
/// the world's edge.
pub fn path(w: usize, h: usize, from: (f32, f32), to: (f32, f32)) -> Vec<u32> {
    let (dx, dy) = (to.0 - from.0, to.1 - from.1);
    let n = (dx.hypot(dy) * 2.0).ceil().max(1.0) as usize;
    let mut cells: Vec<u32> = Vec::new();
    for k in 0..=n {
        let t = k as f32 / n as f32;
        let (x, y) = (from.0 + dx * t, from.1 + dy * t);
        if x < 0.0 || y < 0.0 || x >= w as f32 || y >= h as f32 {
            break;
        }
        let i = (y as usize * w + x as usize) as u32;
        if cells.last() != Some(&i) {
            cells.push(i);
        }
    }
    cells
}

/// Step 1: the shots (see the module's comment). Returns each shot's hit
/// and kills.
pub fn shots(d: &mut BigDish, shots: &[ShotIn], slime: f32, crater: u16) -> Vec<ShotOut> {
    let (w, h) = (d.w, d.h);
    let shots = &shots[..shots.len().min(SHOTS_MAX)];
    let mut used = 0;
    let hits: Vec<u32> = shots
        .iter()
        .map(|s| {
            let cells = path(w, h, s.from, s.to);
            let cells = &cells[..cells.len().min(PATH_MAX - used)];
            used += cells.len();
            for &i in cells {
                if !d.agar[i as usize] {
                    return NONE;
                }
                if d.trail[i as usize] >= slime {
                    return i;
                }
            }
            NONE
        })
        .collect();
    let r2s: Vec<u32> = shots.iter().map(|s| r2(s.r.min(R_MAX))).collect();
    let inside = |c: usize, q: usize| {
        let hit = hits[q];
        if hit == NONE {
            return false;
        }
        let (dx, dy) = ((c % w) as i64 - (hit as usize % w) as i64, (c / w) as i64 - (hit as usize / w) as i64);
        dx * dx + dy * dy <= r2s[q] as i64
    };
    let mut killed = vec![0u32; shots.len()];
    for k in 0..d.walkers.len() {
        if !d.lives(k) || (!d.life.is_empty() && matches!(d.life[k], LIFE_SPORE | LIFE_DEAD)) {
            continue;
        }
        let (x, y, _) = d.walkers[k];
        let c = y as usize * w + x as usize;
        if let Some(q) = (0..shots.len()).find(|&q| inside(c, q)) {
            d.kill(k);
            killed[q] += 1;
        }
    }
    for q in 0..shots.len() {
        if hits[q] != NONE {
            let (hx, hy) = ((hits[q] as usize % w) as i64, (hits[q] as usize / w) as i64);
            for c in disc(w, h, hx, hy, r2s[q]) {
                d.trail[c] = 0.0;
            }
            // The crater (`Frame::crater`) on the same whole cells as the
            // strike, so a GPU twin can match it to the bit.
            if crater > 0 {
                if d.scorch.is_empty() {
                    d.scorch = vec![0; w * h];
                }
                for c in disc(w, h, hx, hy, r2s[q]) {
                    if d.agar[c] {
                        d.scorch[c] = d.scorch[c].max(crater);
                    }
                }
            }
            // A hit is a strike, so with the alarm field on (the mould tiers,
            // `mould::tiers`) it lays the alarm the tiers answer: the Sporer
            // scatters, the Sleeper wakes, the Swarmer floods the place.
            // Off by default (no alarm field). The GPU's `shout` pass lays
            // it too (2026-11-12), shots in order; `pack` sends the radius
            // squared as `r2` of SHOT_ALARM_R x r, since for a radius that
            // is not whole that differs from four times the strike's.
            if !d.alarm.is_empty() {
                for c in disc(w, h, hx, hy, r2(SHOT_ALARM_R * shots[q].r.min(R_MAX))) {
                    if d.agar[c] {
                        d.alarm[c] += SHOT_ALARM;
                    }
                }
            }
        }
    }
    hits.iter()
        .zip(killed)
        .map(|(&hit, killed)| ShotOut { hit: (hit != NONE).then(|| ((hit as usize % w) as f32 + 0.5, (hit as usize / w) as f32 + 0.5)), killed })
        .collect()
}

/// Step 2: each unit's smell on the agar cells of its disc.
pub fn smell(d: &mut BigDish, units: &[UnitIn]) {
    assert!(!d.p.scent_coarse, "the squad smells on the fine scent grid");
    let (w, h) = (d.w, d.h);
    for u in &units[..units.len().min(UNITS_MAX)] {
        if u.smell <= 0.0 {
            continue;
        }
        let (cx, cy) = cell_of(w, h, u.x, u.y);
        for i in disc(w, h, cx, cy, r2(u.smell_r)) {
            if d.agar[i] {
                d.scent[i] += u.smell;
            }
        }
    }
}

/// Step 4's reads: each unit's walkers within `touch`, the block it aims
/// at (the block index, row-major, `BIN` cells a side) and that block's
/// walkers.
pub fn read(d: &BigDish, units: &[UnitIn]) -> Vec<(u32, Option<usize>, u32)> {
    let (w, h) = (d.w, d.h);
    let occ = d.occupancy();
    let (nbw, nbh) = (w.div_ceil(BIN) as i64, h.div_ceil(BIN) as i64);
    units[..units.len().min(UNITS_MAX)]
        .iter()
        .map(|u| {
            let (cx, cy) = cell_of(w, h, u.x, u.y);
            let touching = disc(w, h, cx, cy, r2(u.touch)).map(|i| occ[i] as u32).sum();
            let s2 = r2(u.sight) as i64;
            let r = isqrt(r2(u.sight)) + 2;
            let (by0, by1) = ((cy - r).max(0) / BIN as i64, ((cy + r) / BIN as i64).min(nbh - 1));
            let (bx0, bx1) = ((cx - r).max(0) / BIN as i64, ((cx + r) / BIN as i64).min(nbw - 1));
            let mut best = (0u32, None);
            for by in by0..=by1 {
                for bx in bx0..=bx1 {
                    let (mx, my) = (bx * BIN as i64 + BIN as i64 / 2 - cx, by * BIN as i64 + BIN as i64 / 2 - cy);
                    if mx * mx + my * my > s2 {
                        continue;
                    }
                    let mut n = 0u32;
                    for y in (by as usize * BIN)..((by as usize + 1) * BIN).min(h) {
                        for x in (bx as usize * BIN)..((bx as usize + 1) * BIN).min(w) {
                            n += occ[y * w + x] as u32;
                        }
                    }
                    if n > best.0 {
                        best = (n, Some((by * nbw + bx) as usize));
                    }
                }
            }
            (touching, best.1, best.0)
        })
        .collect()
}

/// Step 4's harvest, in unit order: what each unit took.
pub fn harvest(d: &mut BigDish, units: &[UnitIn]) -> Vec<f32> {
    let (w, h) = (d.w, d.h);
    units[..units.len().min(UNITS_MAX)]
        .iter()
        .map(|u| {
            if u.take <= 0.0 || d.charge.is_empty() {
                return 0.0;
            }
            let (cx, cy) = cell_of(w, h, u.x, u.y);
            let mut rem = u.take;
            for i in disc(w, h, cx, cy, r2(u.take_r)) {
                if d.charge[i] > 0.0 {
                    let t = d.charge[i].min(rem);
                    d.charge[i] -= t;
                    rem -= t;
                    if rem <= 0.0 {
                        break;
                    }
                }
            }
            u.take - rem
        })
        .collect()
}

/// The centre of block `b` (`read`'s target).
pub fn block_centre(w: usize, b: usize) -> (f32, f32) {
    let nbw = w.div_ceil(BIN);
    (((b % nbw) * BIN) as f32 + BIN as f32 / 2.0, ((b / nbw) * BIN) as f32 + BIN as f32 / 2.0)
}

/// A whole frame on the CPU: shots, smell, `ticks` ticks, reads, harvest.
pub fn frame(d: &mut BigDish, f: &Frame, ticks: u32) -> FrameOut {
    let shots = shots(d, &f.shots, f.slime, f.crater);
    smell(d, &f.units);
    for _ in 0..ticks {
        d.tick();
    }
    let reads = read(d, &f.units);
    let took = harvest(d, &f.units);
    let w = d.w;
    FrameOut {
        units: reads.into_iter().zip(took).map(|((touching, b, dense), took)| UnitOut { touching, target: b.map(|b| block_centre(w, b)), took, dense }).collect(),
        shots,
    }
}

// ------------------------------------------------- the GPU's buffer layout

/// Word offsets in the GPU's squad buffer: units in (8 words each), units
/// out (4), shots in (4), shots out (4), then the shots' cells.
pub const U_IN: usize = 0;
pub const U_OUT: usize = U_IN + UNITS_MAX * 8;
pub const S_IN: usize = U_OUT + UNITS_MAX * 4;
pub const S_OUT: usize = S_IN + SHOTS_MAX * 4;
pub const PATH: usize = S_OUT + SHOTS_MAX * 4;
pub const WORDS: usize = PATH + PATH_MAX;

/// A frame as the GPU's squad buffer holds it (outputs zeroed), with its
/// unit and shot counts.
pub fn pack(w: usize, h: usize, f: &Frame) -> (Vec<u32>, u32, u32) {
    let units = &f.units[..f.units.len().min(UNITS_MAX)];
    let shots = &f.shots[..f.shots.len().min(SHOTS_MAX)];
    let mut b = vec![0u32; PATH];
    for (k, u) in units.iter().enumerate() {
        let (cx, cy) = cell_of(w, h, u.x, u.y);
        b[U_IN + 8 * k..U_IN + 8 * k + 8].copy_from_slice(&[cx as u32, cy as u32, r2(u.touch), r2(u.sight), r2(u.take_r), u.take.to_bits(), r2(u.smell_r), u.smell.to_bits()]);
    }
    for k in 0..UNITS_MAX {
        b[U_OUT + 4 * k + 1] = NONE;
    }
    let mut cells: Vec<u32> = Vec::new();
    for (q, s) in shots.iter().enumerate() {
        let p = path(w, h, s.from, s.to);
        let p = &p[..p.len().min(PATH_MAX - cells.len())];
        // The fourth word is the alarm's squared radius (`SHOT_ALARM_R`).
        b[S_IN + 4 * q..S_IN + 4 * q + 4].copy_from_slice(&[cells.len() as u32, p.len() as u32, r2(s.r.min(R_MAX)), r2(SHOT_ALARM_R * s.r.min(R_MAX))]);
        cells.extend_from_slice(p);
    }
    for q in 0..SHOTS_MAX {
        b[S_OUT + 4 * q] = NONE;
    }
    b.extend(cells);
    (b, units.len() as u32, shots.len() as u32)
}

/// The GPU's results: the squad buffer's words up to `PATH` and the kills.
pub fn unpack(w: usize, words: &[u32], kills: &[u32], nunits: usize, nshots: usize) -> FrameOut {
    FrameOut {
        units: (0..nunits)
            .map(|k| {
                let o = &words[U_OUT + 4 * k..U_OUT + 4 * k + 4];
                UnitOut { touching: o[0], target: (o[1] != NONE).then(|| block_centre(w, o[1] as usize)), took: f32::from_bits(o[2]), dense: o[3] }
            })
            .collect(),
        shots: (0..nshots)
            .map(|q| {
                let hit = words[S_OUT + 4 * q];
                ShotOut { hit: (hit != NONE).then(|| ((hit as usize % w) as f32 + 0.5, (hit as usize / w) as f32 + 0.5)), killed: kills[q] }
            })
            .collect(),
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::mould::big::BigParams;

    #[test]
    fn discs_are_whole_cells_in_the_gpus_order() {
        assert_eq!(isqrt(0), 0);
        assert_eq!(isqrt(8), 2);
        assert_eq!(isqrt(9), 3);
        let d: Vec<usize> = disc(10, 10, 5, 5, 1).collect();
        assert_eq!(d, vec![45, 54, 55, 56, 65]);
        // Clipped at a corner: (0,0), (1,0), (0,1), (1,1) of radius² 2.
        let c: Vec<usize> = disc(10, 10, 0, 0, 2).collect();
        assert_eq!(c, vec![0, 1, 10, 11]);
        assert_eq!(r2(2.5), 6);
        assert_eq!(cell_of(10, 10, -3.0, 12.5), (0, 9));
    }

    #[test]
    fn a_path_lists_each_cell_once_and_stops_at_the_edge() {
        let p = path(10, 10, (0.5, 0.5), (3.5, 0.5));
        assert_eq!(p, vec![0, 1, 2, 3]);
        let q = path(10, 10, (8.5, 5.5), (14.0, 5.5));
        assert_eq!(q, vec![58, 59]);
    }

    #[test]
    fn slime_is_the_trail_the_game_curve_draws_at_26() {
        let knee = 3.0;
        let v = slime_trail(knee);
        assert!(((1.0 - (-v / knee).exp()) - SLIME_LOOK).abs() < 1e-6);
    }

    fn world() -> BigDish {
        let mut p = BigParams::today();
        p.scale = 2;
        p.scent_coarse = false;
        p.walkers = 3000;
        p.crystal_charge = 64.0;
        p.crystal_eat = 0.005;
        p.seed = 3;
        let mut d = BigDish::new(p);
        for _ in 0..40 {
            d.tick();
        }
        d
    }

    #[test]
    fn a_shot_stops_at_the_first_slime_and_kills_there() {
        let mut d = world();
        let slime = slime_trail(3.0);
        let (w, h) = (d.w, d.h);
        // A line from a cell that is not slime into the thickest cell.
        let thick = (0..w * h).max_by(|&a, &b| d.trail[a].total_cmp(&d.trail[b])).unwrap();
        let (tx, ty) = ((thick % w) as f32 + 0.5, (thick / w) as f32 + 0.5);
        let from = (0..w * h)
            .find(|&i| {
                let r = ((i % w) as f32 - tx).hypot((i / w) as f32 - ty);
                d.agar[i] && d.trail[i] < slime / 4.0 && (12.0..30.0).contains(&r)
            })
            .expect("a clear cell near the mould");
        let from = ((from % w) as f32 + 0.5, (from / w) as f32 + 0.5);
        let alive = |d: &BigDish| (0..d.walkers.len()).filter(|&k| d.lives(k) && (d.life.is_empty() || d.life[k] != LIFE_DEAD)).count();
        let before = alive(&d);
        // The first slime cell on the line, not the thickest.
        let cells = path(w, h, from, (tx, ty));
        let first = cells.iter().map(|&i| i as usize).take_while(|&i| d.agar[i]).find(|&i| d.trail[i] >= slime).expect("the line reaches slime");
        assert_ne!(first, thick, "this line meets thinner slime first");
        let out = shots(&mut d, &[ShotIn { from, to: (tx, ty), r: 3.0 }], slime, 0);
        let hit = out[0].hit.expect("the line reaches slime");
        assert_eq!(((first % w) as f32 + 0.5, (first / w) as f32 + 0.5), hit);
        assert!(out[0].killed > 0, "walkers die where it hits");
        assert_eq!(before - alive(&d), out[0].killed as usize);
        let (hx, hy) = (hit.0 as i64, hit.1 as i64);
        for i in disc(w, h, hx, hy, r2(3.0)) {
            assert_eq!(d.trail[i], 0.0, "the strike clears the trail");
        }
        assert!(d.alarm.is_empty(), "no alarm field unless a tier turns it on");
    }

    #[test]
    fn with_the_alarm_on_a_hit_lays_it_for_the_tiers() {
        let mut d = world();
        let (w, h) = (d.w, d.h);
        // The Swarmer's field, turned on mid-game as `BigDish::new` would.
        d.p.alarm_keep = 0.98;
        d.alarm = vec![0.0; w * h];
        let thick = (0..w * h).max_by(|&a, &b| d.trail[a].total_cmp(&d.trail[b])).unwrap();
        let at = ((thick % w) as f32 + 0.5, (thick / w) as f32 + 0.5);
        // A shot that starts on slime hits its first cell.
        let out = shots(&mut d, &[ShotIn { from: at, to: (at.0 + 5.0, at.1), r: 3.0 }], slime_trail(3.0), 0);
        let hit = out[0].hit.expect("it starts on slime");
        let (hx, hy) = (hit.0 as i64, hit.1 as i64);
        let near: Vec<usize> = disc(w, h, hx, hy, r2(SHOT_ALARM_R * 3.0)).collect();
        for &i in &near {
            assert_eq!(d.alarm[i], if d.agar[i] { SHOT_ALARM } else { 0.0 });
        }
        let total: f32 = d.alarm.iter().sum();
        let laid: f32 = near.iter().map(|&i| d.alarm[i]).sum();
        assert_eq!(total, laid, "alarm only within twice the radius");
        assert!(laid > 0.0);
    }

    #[test]
    fn a_hit_lays_its_crater_on_the_strikes_cells_and_the_next_shot_flies_on() {
        let mut d = world();
        let (w, h) = (d.w, d.h);
        let slime = slime_trail(3.0);
        let thick = (0..w * h).max_by(|&a, &b| d.trail[a].total_cmp(&d.trail[b])).unwrap();
        let at = ((thick % w) as f32 + 0.5, (thick / w) as f32 + 0.5);
        let shot = ShotIn { from: at, to: (at.0 + 12.0, at.1), r: 2.0 };
        // Without a crater no field appears.
        let mut plain = d.clone();
        shots(&mut plain, &[shot], slime, 0);
        assert!(plain.scorch.is_empty(), "crater 0 lays none");
        let out = shots(&mut d, &[shot], slime, 90);
        let hit = out[0].hit.expect("it starts on slime");
        let (hx, hy) = (hit.0 as i64, hit.1 as i64);
        let cells: Vec<usize> = disc(w, h, hx, hy, r2(2.0)).collect();
        for i in 0..w * h {
            let want = if cells.contains(&i) && d.agar[i] { 90 } else { 0 };
            assert_eq!(d.scorch[i], want, "cell {i}");
        }
        // A longer crater keeps the longer; a shorter one does not cut it.
        shots(&mut d, &[shot], slime, 30);
        assert!(cells.iter().all(|&i| !d.agar[i] || d.scorch[i] == 90));
        // With the struck cells' trail held under slime, a second shot down
        // the same line lands farther on: the hole deepens.
        let mut a = d.clone();
        for _ in 0..4 {
            a.tick();
        }
        let next = shots(&mut a, &[shot], slime, 90)[0].hit;
        let mut b = plain.clone();
        for _ in 0..4 {
            b.tick();
        }
        let again = shots(&mut b, &[shot], slime, 0)[0].hit;
        let far = |h: Option<(f32, f32)>| h.map_or(f32::INFINITY, |(x, _)| x - at.0);
        assert!(far(next) > far(again), "crater {next:?} against none {again:?}");
    }
}
