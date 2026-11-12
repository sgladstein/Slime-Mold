//! **The cut probe** (owner, 2026-11-12 04:13-04:18Z: "the slime didn't feel
//! like one cohesive organism"; a cut-off part "can act as its own smaller
//! organism ... attack or find its own food source or try to rejoin the main
//! slime before starving", "but it should still feel like a cut off hunk and
//! not a bunch of individuals").
//!
//! ```text
//! cargo run --release --example cutprobe -- seeds=1-8 [cuts=3 grow=60 after=120 scale=2 walkers=5000]
//!     [arm=NAME:key=v+key=v ...] [look=DIR] [series=FILE]
//! ```
//!
//! Grows the CPU race world (`larder_world::race_world`) for `grow` game
//! seconds (15 ticks each, as the game runs), then for each of `cuts` far
//! tips of the mould cuts across its tube `L` dish cells back from the tip
//! (the walkers on a band two dish cells wide across the tube killed and
//! its trail cleared, as a burn line would), and follows the cut-off piece
//! for `after` game seconds. Each cut runs on its own clone of the grown
//! world, so the cuts of one seed do not meet. Every arm grows its own
//! world with its rules on from the start (the game would have them on), so
//! cuts are chosen by the same rule but land on that arm's mould.
//!
//! **Measures, named before the first run:**
//! - `frag`: pieces the cut piece's walkers form (8-connected after
//!   bridging gaps of up to 2 dish cells), counting only parts with at
//!   least 5% of its walkers, mean over its life; `lcs`: the largest part's
//!   share of its walkers, mean over its life. **The deciding measure for
//!   "one hunk" is `lcs` at 20 s after the cut** (1 is one lump; low is
//!   scattered individuals).
//! - `rms`: walkers' root-mean-square distance from their centre, dish
//!   cells, 20 s after the cut; `compact` the same over a disc's of the
//!   same walker count (1 is a round lump, a thin arm is far above).
//! - `life`: game seconds until under 10% of the walkers at the cut are
//!   left (the run's end if never).
//! - `fate`: what happens first: `rejoin` (5 or more of its walkers touch
//!   the main body), `crystal` (it eats crystal it was not eating in its
//!   first second), `starve` (all dead), else `alive` at the end; with
//!   `fed` when it was eating at the cut (a piece cut with its crystal).
//!   Soldiers are not in this probe.
//!
//! With `look=DIR` it writes the world at the cut and every 10 s after
//! (walkers of the piece pink, the rest yellow); with `CUTPROBE_FILM=1` in
//! the environment, a frame every third of a game second too.
//!
//! Arms default to `base` (the race world as the game has it). Each `arm=`
//! adds one: `name:key=value+key=value` over `BigParams` fields named in
//! `set`. Paired by seed and cut index; the summary gives each arm's
//! paired count against the first arm on `lcs` at 20 s, `life` and
//! `compact`, with a two-sided sign test.
// Index loops over walker slots read several parallel arrays at once.
#![allow(clippy::needless_range_loop, clippy::too_many_arguments)]
use slime_mould::mould::big::{BigDish, BigParams};
use slime_mould::mould::larder_world::race_params;
use std::collections::VecDeque;

const TPS: u64 = 15;

fn set(p: &mut BigParams, k: &str, v: &str) {
    let f = || v.parse::<f32>().unwrap_or_else(|_| panic!("{k}={v}"));
    match k {
        "energy_flow" => p.energy_flow = f(),
        "blocked_keep" => p.blocked_keep = f() != 0.0,
        "blocked_reset" => p.blocked_reset = f(),
        "blocked_retry" => p.blocked_retry = f() as u8,
        "blocked_deposit" => p.blocked_deposit = f(),
        "upkeep" => p.upkeep = f(),
        "starve" => p.starve = f(),
        "keep" => p.keep = f(),
        "deposit" => p.deposit = f(),
        "sensor_offset" => p.sensor_offset = f(),
        "sensor_angle" => p.sensor_angle = f().to_radians(),
        "rotate_angle" => p.rotate_angle = f().to_radians(),
        "scent_offset" => p.scent_offset = f(),
        "fed_scent" => p.fed_scent = f(),
        "crowd" => p.crowd = f() as u8,
        "step" => p.step = f(),
        "substeps" => p.substeps = f() as usize,
        "hunger_scent" => p.hunger_scent = f(),
        "hunger_at" => p.hunger_at = f(),
        "body_pull" => p.body_pull = f(),
        "body_r" => p.body_r = f() as u8,
        "body_lone" => p.body_lone = f(),
        "tether" => p.tether = f() as u8,
        "lone_upkeep" => p.lone_upkeep = f(),
        "lone_r" => p.lone_r = f() as u8,
        "tether_hold" => p.tether_hold = f(),
        _ => panic!("unknown key {k}"),
    }
}

/// Cells within `g` fine cells (Chebyshev) of a marked cell.
fn dilate(m: &[bool], w: usize, h: usize, g: usize) -> Vec<bool> {
    let mut row = vec![false; w * h];
    for y in 0..h {
        let mut last = usize::MAX;
        // Distance to the nearest mark leftward then rightward.
        for x in 0..w {
            if m[y * w + x] {
                last = x;
            }
            if last != usize::MAX && x - last <= g {
                row[y * w + x] = true;
            }
        }
        last = usize::MAX;
        for x in (0..w).rev() {
            if m[y * w + x] {
                last = x;
            }
            if last != usize::MAX && last - x <= g {
                row[y * w + x] = true;
            }
        }
    }
    let mut out = vec![false; w * h];
    for x in 0..w {
        let mut last = usize::MAX;
        for y in 0..h {
            if row[y * w + x] {
                last = y;
            }
            if last != usize::MAX && y - last <= g {
                out[y * w + x] = true;
            }
        }
        last = usize::MAX;
        for y in (0..h).rev() {
            if row[y * w + x] {
                last = y;
            }
            if last != usize::MAX && last - y <= g {
                out[y * w + x] = true;
            }
        }
    }
    out
}

/// 8-connected labels (0 none, 1..) and the count.
fn label(m: &[bool], w: usize, h: usize) -> (Vec<u32>, u32) {
    let mut lab = vec![0u32; w * h];
    let mut n = 0;
    let mut q = VecDeque::new();
    for s in 0..w * h {
        if !m[s] || lab[s] != 0 {
            continue;
        }
        n += 1;
        lab[s] = n;
        q.push_back(s);
        while let Some(i) = q.pop_front() {
            let (x, y) = ((i % w) as i64, (i / w) as i64);
            for dy in -1..=1 {
                for dx in -1..=1 {
                    let (nx, ny) = (x + dx, y + dy);
                    if nx < 0 || ny < 0 || nx >= w as i64 || ny >= h as i64 {
                        continue;
                    }
                    let j = ny as usize * w + nx as usize;
                    if m[j] && lab[j] == 0 {
                        lab[j] = n;
                        q.push_back(j);
                    }
                }
            }
        }
    }
    (lab, n)
}

fn cell_of(d: &BigDish, k: usize) -> usize {
    let (x, y, _) = d.walkers[k];
    y as usize * d.w + x as usize
}

/// Walkers alive and ordinary-or-dormant (holding a cell).
fn holds(d: &BigDish, k: usize) -> bool {
    d.lives(k) && (d.life.is_empty() || d.life[k] == 0 || d.life[k] == 2)
}

struct Cut {
    /// Fine cells cut.
    cells: Vec<usize>,
    /// A fine cell of the tip, to name the piece.
    tip: usize,
    /// Dish cells from the tip.
    back: f32,
}

/// Choose up to `n` cuts: far tips of the body (geodesic from home), each
/// cut `back` dish cells behind its tip, across the tube: the body cells
/// within one dish cell of that geodesic level and 12 dish cells of the
/// cut point. A cut is kept if the piece beyond it holds 100 or more walkers
/// and under 40% of the mould, and no longer joins home.
fn choose_cuts(d: &BigDish, n: usize, cutw: f32) -> Vec<Cut> {
    let (w, h, s) = (d.w, d.h, d.p.scale);
    let occ: Vec<bool> = d.occupancy().iter().map(|&o| o > 0).collect();
    let body = dilate(&occ, w, h, s);
    let home = {
        let (hx, hy) = d.homes[0];
        (hy * s as f32) as usize * w + (hx * s as f32) as usize
    };
    // Geodesic distance from the body cell nearest home.
    let start = (0..w * h).filter(|&i| body[i]).min_by_key(|&i| {
        let (dx, dy) = ((i % w) as i64 - (home % w) as i64, (i / w) as i64 - (home / w) as i64);
        dx * dx + dy * dy
    });
    let Some(start) = start else { return Vec::new() };
    let mut dist = vec![u32::MAX; w * h];
    let mut parent = vec![usize::MAX; w * h];
    let mut q = VecDeque::new();
    dist[start] = 0;
    q.push_back(start);
    while let Some(i) = q.pop_front() {
        let (x, y) = ((i % w) as i64, (i / w) as i64);
        for dy in -1..=1 {
            for dx in -1..=1 {
                let (nx, ny) = (x + dx, y + dy);
                if nx < 0 || ny < 0 || nx >= w as i64 || ny >= h as i64 {
                    continue;
                }
                let j = ny as usize * w + nx as usize;
                if body[j] && dist[j] == u32::MAX {
                    dist[j] = dist[i] + 1;
                    parent[j] = i;
                    q.push_back(j);
                }
            }
        }
    }
    let total = (0..d.walkers.len()).filter(|&k| holds(d, k)).count();
    let mut used = vec![false; w * h];
    let mut cuts = Vec::new();
    for _ in 0..n * 4 {
        if cuts.len() >= n {
            break;
        }
        let Some(tip) = (0..w * h).filter(|&i| body[i] && dist[i] != u32::MAX && !used[i]).max_by_key(|&i| (dist[i], i)) else { break };
        let mut kept = None;
        for back in [15.0f32, 25.0, 35.0, 50.0] {
            let steps = (back * s as f32) as u32;
            if dist[tip] <= steps + 10 * s as u32 {
                break;
            }
            let mut c = tip;
            while dist[c] > dist[tip] - steps {
                c = parent[c];
            }
            // A straight band across the path at c: its direction from the
            // path 5 dish cells either side, the band `cutw` dish cells
            // either side of c along it and 12 across, whatever lies there
            // (a burn line, so the walkers cannot step round its ends).
            let (cx, cy) = ((c % w) as f32, (c / w) as f32);
            let mut ahead = tip;
            while dist[ahead] > dist[c] + 5 * s as u32 {
                ahead = parent[ahead];
            }
            let mut behind = c;
            for _ in 0..5 * s {
                if parent[behind] != usize::MAX {
                    behind = parent[behind];
                }
            }
            let (dx, dy) = ((ahead % w) as f32 - (behind % w) as f32, (ahead / w) as f32 - (behind / w) as f32);
            let dl = dx.hypot(dy).max(1.0);
            let (ux, uy) = (dx / dl, dy / dl);
            let rloc = 12.0 * s as f32;
            let cells: Vec<usize> = (0..w * h)
                .filter(|&i| {
                    let (px, py) = ((i % w) as f32 - cx, (i / w) as f32 - cy);
                    d.agar[i] && (px * ux + py * uy).abs() <= cutw * s as f32 && (px * uy - py * ux).abs() <= rloc
                })
                .collect();
            let mut b2 = body.clone();
            for &i in &cells {
                b2[i] = false;
            }
            let (lab, _) = label(&b2, w, h);
            let pl = lab[tip];
            if pl == 0 || lab[start] == pl {
                continue;
            }
            let size = (0..d.walkers.len()).filter(|&k| holds(d, k) && lab[cell_of(d, k)] == pl).count();
            if size >= 100 && size * 10 < total * 4 {
                kept = Some((Cut { cells, tip, back }, lab, pl));
                break;
            }
        }
        // Mark this tip's neighbourhood used, kept or not.
        match kept {
            Some((cut, lab, pl)) => {
                for i in 0..w * h {
                    if lab[i] == pl {
                        used[i] = true;
                    }
                }
                cuts.push(cut);
            }
            None => {
                let (tx, ty) = ((tip % w) as f32, (tip / w) as f32);
                for i in 0..w * h {
                    if ((i % w) as f32 - tx).hypot((i / w) as f32 - ty) <= 20.0 * s as f32 {
                        used[i] = true;
                    }
                }
            }
        }
    }
    cuts
}


/// Ring cuts: up to `n` hunks of the body, each the walkers within `r`
/// dish cells of a centre 30 or more dish cells (by the body) from home,
/// cut off by killing every walker on the ring from `r` to `r` + `gap`
/// (and clearing its trail). Centres hold the most walkers in their disc,
/// 30 dish cells apart. A clean cut: whatever happens next is the piece's
/// own doing.
fn choose_rings(d: &BigDish, n: usize, r: f32, gap: f32, unfed: bool) -> Vec<Cut> {
    let (w, h, s) = (d.w, d.h, d.p.scale as f32);
    let occ: Vec<bool> = d.occupancy().iter().map(|&o| o > 0).collect();
    let body = dilate(&occ, w, h, d.p.scale);
    let home = {
        let (hx, hy) = d.homes[0];
        ((hx * s) as usize, (hy * s) as usize)
    };
    let mut cand: Vec<(usize, usize)> = Vec::new();
    let step = d.p.scale * 2;
    for y in (0..h).step_by(step) {
        for x in (0..w).step_by(step) {
            let i = y * w + x;
            if !occ[i] || ((x as f32 - home.0 as f32).hypot(y as f32 - home.1 as f32)) < 30.0 * s {
                continue;
            }
            let rr = (r * s) as i64;
            let (mut c, mut food) = (0, false);
            let rf = rr + 4 * d.p.scale as i64;
            for dy in -rf..=rf {
                for dx in -rf..=rf {
                    let (nx, ny) = (x as i64 + dx, y as i64 + dy);
                    if nx >= 0 && ny >= 0 && (nx as usize) < w && (ny as usize) < h && dx * dx + dy * dy <= (rr + 4 * d.p.scale as i64).pow(2) {
                        let j = ny as usize * w + nx as usize;
                        // With `unfed`, no crystal within 4 dish cells of the disc.
                        if unfed && !d.charge.is_empty() && d.pile_of[j] > 0 && d.charge[j] > 0.0 {
                            food = true;
                        }
                        if dx * dx + dy * dy <= rr * rr {
                            c += d.occupancy()[j] as usize;
                        }
                    }
                }
            }
            if !food {
                cand.push((c, i));
            }
        }
    }
    let _ = body;
    cand.sort_by(|a, b| b.cmp(a));
    let mut cuts: Vec<Cut> = Vec::new();
    for (c, i) in cand {
        if cuts.len() >= n || c < 20 {
            break;
        }
        let (x, y) = ((i % w) as f32, (i / w) as f32);
        if cuts.iter().any(|k| ((k.tip % w) as f32 - x).hypot((k.tip / w) as f32 - y) < 30.0 * s) {
            continue;
        }
        let (r0, r1) = (r * s, (r + gap) * s);
        let cells = (0..w * h)
            .filter(|&j| {
                let dd = ((j % w) as f32 - x).hypot((j / w) as f32 - y);
                dd > r0 && dd <= r1
            })
            .collect();
        cuts.push(Cut { cells, tip: i, back: c as f32 });
    }
    cuts
}

#[derive(Clone, Default, Debug)]
struct Sample {
    t: f32,
    n: usize,
    frag: usize,
    lcs: f32,
    rms: f32,
    compact: f32,
    cx: f32,
    cy: f32,
    touching: usize,
    took: f32,
    energy: f32,
}

/// 1 = piece, 2 = main body, 0 none.
fn sample(d: &BigDish, tag: &[u8], t: f32) -> Sample {
    let (w, h, s) = (d.w, d.h, d.p.scale);
    let mut piece = vec![false; w * h];
    let mut main = vec![false; w * h];
    let (mut n, mut sx, mut sy, mut took, mut en) = (0usize, 0f64, 0f64, 0f32, 0f64);
    for k in 0..d.walkers.len() {
        if !holds(d, k) {
            continue;
        }
        let i = cell_of(d, k);
        if tag[k] == 1 {
            piece[i] = true;
            n += 1;
            sx += d.walkers[k].0 as f64;
            sy += d.walkers[k].1 as f64;
            if !d.took.is_empty() {
                took += d.took[k];
            }
            if !d.energy.is_empty() {
                en += d.energy[k] as f64;
            }
        } else if tag[k] == 2 {
            main[i] = true;
        }
    }
    if n == 0 {
        return Sample { t, ..Default::default() };
    }
    let (cx, cy) = ((sx / n as f64) as f32, (sy / n as f64) as f32);
    let mut r2 = 0f64;
    for k in 0..d.walkers.len() {
        if holds(d, k) && tag[k] == 1 {
            let (x, y, _) = d.walkers[k];
            r2 += ((x - cx).powi(2) + (y - cy).powi(2)) as f64;
        }
    }
    let rms = (r2 / n as f64).sqrt() as f32;
    let disc = (n as f32 / std::f32::consts::PI).sqrt() / std::f32::consts::SQRT_2;
    let (lab, nl) = label(&dilate(&piece, w, h, s), w, h);
    let mut count = vec![0usize; nl as usize + 1];
    for k in 0..d.walkers.len() {
        if holds(d, k) && tag[k] == 1 {
            count[lab[cell_of(d, k)] as usize] += 1;
        }
    }
    let min = (n / 20).max(1);
    let frag = count[1..].iter().filter(|&&c| c >= min).count();
    let lcs = *count[1..].iter().max().unwrap_or(&0) as f32 / n as f32;
    let near_main = dilate(&main, w, h, s);
    let touching = (0..d.walkers.len()).filter(|&k| holds(d, k) && tag[k] == 1 && near_main[cell_of(d, k)]).count();
    Sample { t, n, frag, lcs, rms: rms / s as f32, compact: rms / disc.max(0.5), cx: cx / s as f32, cy: cy / s as f32, touching, took, energy: (en / n as f64) as f32 }
}

/// New slots alive since the last tick take the tag of a tagged walker on
/// or beside their cell (a child lands beside its parent).
fn tag_births(d: &BigDish, tag: &mut [u8], was: &mut [bool], grid: &mut [u8]) {
    let w = d.w;
    grid.iter_mut().for_each(|g| *g = 0);
    for k in 0..d.walkers.len() {
        if holds(d, k) && was[k] && tag[k] > 0 {
            grid[cell_of(d, k)] = tag[k];
        }
    }
    for k in 0..d.walkers.len() {
        let now = d.lives(k);
        if now && !was[k] {
            let i = cell_of(d, k);
            let (x, y) = ((i % w) as i64, (i / w) as i64);
            let mut t = 0;
            'f: for r in 1..=2i64 {
                for dy in -r..=r {
                    for dx in -r..=r {
                        let (nx, ny) = (x + dx, y + dy);
                        if nx >= 0 && ny >= 0 && (nx as usize) < w && (ny as usize) < d.h && grid[ny as usize * w + nx as usize] > 0 {
                            t = grid[ny as usize * w + nx as usize];
                            break 'f;
                        }
                    }
                }
            }
            tag[k] = if t == 0 { 2 } else { t };
        }
        was[k] = now;
    }
}

fn image(d: &BigDish, tag: &[u8]) -> image::RgbaImage {
    let (w, h) = (d.w, d.h);
    let mut img = image::RgbaImage::new(w as u32, h as u32);
    for i in 0..w * h {
        let mut c = if !d.agar[i] { [0u8, 0, 0] } else { [18, 16, 14] };
        if !d.rock.is_empty() && d.rock[i] {
            c = [70, 66, 60];
        }
        if !d.wet.is_empty() && d.wet[i] {
            c = [16, 24, 36];
        }
        if !d.charge.is_empty() && d.pile_of[i] > 0 && d.charge[i] > 0.0 {
            let f = (d.charge[i] / d.pile_fill[d.pile_of[i] as usize - 1]).min(1.0);
            c = [(30.0 + 40.0 * f) as u8, (60.0 + 120.0 * f) as u8, (80.0 + 170.0 * f) as u8];
        }
        let t = (d.trail[i] / 40.0).min(1.0);
        for (ch, add) in c.iter_mut().zip([110.0, 90.0, 40.0]) {
            *ch = (*ch as f32 + add * t).min(255.0) as u8;
        }
        img.put_pixel((i % w) as u32, (i / w) as u32, image::Rgba([c[0], c[1], c[2], 255]));
    }
    for k in 0..d.walkers.len() {
        if !holds(d, k) {
            continue;
        }
        let i = cell_of(d, k);
        let c = match tag[k] {
            1 => [255, 70, 200],
            _ => [250, 210, 90],
        };
        img.put_pixel((i % w) as u32, (i / w) as u32, image::Rgba([c[0], c[1], c[2], 255]));
    }
    img
}

#[derive(Clone, Debug)]
struct Run {
    n0: usize,
    back: f32,
    fed: bool,
    lcs20: f32,
    frag_mean: f32,
    lcs_mean: f32,
    rms20: f32,
    compact20: f32,
    life: f32,
    fate: String,
    fate_t: f32,
    moved: f32,
    react: f32,
}

fn follow(grown: &BigDish, cut: &Cut, after: u64, wound: u64, half: bool, look: Option<(&str, String)>, series: &mut Option<std::fs::File>, key: &str) -> Run {
    let mut d = grown.clone();
    let (w, h) = (d.w, d.h);
    let mut cutm = vec![false; w * h];
    for &i in &cut.cells {
        cutm[i] = true;
        d.trail[i] = 0.0;
    }
    for k in 0..d.walkers.len() {
        if holds(&d, k) && cutm[cell_of(&d, k)] {
            d.kill(k);
        }
    }
    // Tag: the component of the tip after the cut.
    let occ: Vec<bool> = d.occupancy().iter().map(|&o| o > 0).collect();
    let (lab, _) = label(&dilate(&occ, w, h, d.p.scale), w, h);
    let pl = lab[cut.tip];
    let mut tag: Vec<u8> = (0..d.walkers.len()).map(|k| if !d.lives(k) { 0 } else if pl > 0 && lab[cell_of(&d, k)] == pl { 1 } else { 2 }).collect();
    let mut was: Vec<bool> = (0..d.walkers.len()).map(|k| d.lives(k)).collect();
    let mut grid = vec![0u8; w * h];
    let s0 = sample(&d, &tag, 0.0);
    let n0 = s0.n;
    let mut samples = vec![s0.clone()];
    if let Some((dir, name)) = &look {
        image(&d, &tag).save(format!("{dir}/{name}-t000.png")).expect("png");
    }
    let mut first_took = 0.0f32;
    // A wound (`wound` game seconds): the cut cells are off the agar, so
    // no walker or trail crosses until it heals.
    // With `half`, only the half of a ring facing home closes: the hunk is
    // cut from the body but free to go anywhere else.
    let (hx, hy) = (d.homes[0].0 * d.p.scale as f32, d.homes[0].1 * d.p.scale as f32);
    let (tx, ty) = ((cut.tip % w) as f32, (cut.tip / w) as f32);
    let healed: Vec<usize> = cut
        .cells
        .iter()
        .copied()
        .filter(|&i| d.agar[i] && (!half || ((i % w) as f32 - tx) * (hx - tx) + ((i / w) as f32 - ty) * (hy - ty) > 0.0))
        .collect();
    if wound > 0 {
        for &i in &healed {
            d.agar[i] = false;
        }
    }
    for tk in 1..=after * TPS {
        if wound > 0 && tk == wound * TPS {
            for &i in &healed {
                d.agar[i] = true;
            }
        }
        d.tick();
        tag_births(&d, &mut tag, &mut was, &mut grid);
        // A film (`CUTPROBE_FILM=1`): a frame every third of a game second.
        if let Some((dir, name)) = &look {
            if std::env::var("CUTPROBE_FILM").is_ok() && tk.is_multiple_of(TPS / 3) {
                image(&d, &tag).save(format!("{dir}/{name}-f{:04}.png", tk / (TPS / 3))).expect("png");
            }
        }
        if tk <= TPS && !d.took.is_empty() {
            first_took += (0..d.walkers.len()).filter(|&k| tag[k] == 1 && holds(&d, k)).map(|k| d.took[k]).sum::<f32>();
        }
        if tk.is_multiple_of(TPS) {
            let t = (tk / TPS) as f32;
            let s = sample(&d, &tag, t);
            let dead = s.n == 0;
            samples.push(s);
            if let Some((dir, name)) = &look {
                if (tk / TPS).is_multiple_of(10) {
                    image(&d, &tag).save(format!("{dir}/{name}-t{:03}.png", tk / TPS)).expect("png");
                }
            }
            if dead {
                break;
            }
        }
    }
    let fed = first_took > 0.0;
    let alive: Vec<&Sample> = samples.iter().filter(|s| s.n > 0).collect();
    let at = |t: f32| samples.iter().find(|s| s.t >= t).cloned().unwrap_or_default();
    let s20 = at(20.0);
    let life = samples.iter().find(|s| s.n * 10 < n0).map_or(after as f32, |s| s.t);
    let mut fate = ("alive".to_string(), after as f32);
    let mut prev_touch = false;
    // Reaction: the first second its centre has moved 3 dish cells.
    let react = samples.iter().find(|x| x.n > 0 && (x.cx - s0.cx).hypot(x.cy - s0.cy) >= 3.0).map_or(-1.0, |x| x.t);
    for s in &samples[1..] {
        if s.n == 0 {
            fate = ("starve".into(), s.t);
            break;
        }
        if s.touching >= (s.n / 20).max(5) && prev_touch {
            fate = ("rejoin".into(), s.t);
            break;
        }
        prev_touch = s.touching >= (s.n / 20).max(5);
        if !fed && s.took > 0.0 && s.t >= 2.0 {
            fate = ("crystal".into(), s.t);
            break;
        }
    }
    if let Some(f) = series {
        use std::io::Write;
        for s in &samples {
            writeln!(f, "{key},{},{},{},{:.3},{:.2},{:.2},{:.1},{:.1},{},{:.3},{:.3}", s.t, s.n, s.frag, s.lcs, s.rms, s.compact, s.cx, s.cy, s.touching, s.took, s.energy).unwrap();
        }
    }
    let last = alive.last().cloned().cloned().unwrap_or_default();
    Run {
        n0,
        back: cut.back,
        fed,
        lcs20: s20.lcs,
        frag_mean: alive.iter().map(|s| s.frag as f32).sum::<f32>() / alive.len().max(1) as f32,
        lcs_mean: alive.iter().map(|s| s.lcs).sum::<f32>() / alive.len().max(1) as f32,
        rms20: s20.rms,
        compact20: s20.compact,
        life,
        fate: if fed { format!("{}+fed", fate.0) } else { fate.0 },
        fate_t: fate.1,
        moved: (last.cx - s0.cx).hypot(last.cy - s0.cy),
        react,
    }
}

fn sign_p(a: usize, b: usize) -> f64 {
    // Two-sided binomial sign test, p = 0.5, ties dropped.
    let n = a + b;
    if n == 0 {
        return 1.0;
    }
    let k = a.min(b);
    let mut c = 1f64;
    let mut sum = 0f64;
    for i in 0..=n {
        if i > 0 {
            c *= (n - i + 1) as f64 / i as f64;
        }
        if i <= k {
            sum += c;
        }
    }
    (2.0 * sum / 2f64.powi(n as i32)).min(1.0)
}

fn main() {
    let mut seeds: Vec<u64> = (1..=4).collect();
    let (mut scale, mut walkers, mut grow, mut after, mut ncuts, mut cutw, mut wound, mut ring) = (2usize, 5_000usize, 60u64, 120u64, 3usize, 3.0f32, 0u64, 0.0f32);
    let (mut unfed, mut half) = (false, false);
    let mut arms: Vec<(String, Vec<(String, String)>)> = Vec::new();
    let (mut look, mut series) = (None::<String>, None::<std::fs::File>);
    for a in std::env::args().skip(1) {
        let (k, v) = a.split_once('=').expect("key=value");
        match k {
            "seeds" => {
                let (a, b) = v.split_once('-').unwrap_or((v, v));
                seeds = (a.parse().unwrap()..=b.parse().unwrap()).collect();
            }
            "scale" => scale = v.parse().unwrap(),
            "walkers" => walkers = v.parse().unwrap(),
            "grow" => grow = v.parse().unwrap(),
            "after" => after = v.parse().unwrap(),
            "cuts" => ncuts = v.parse().unwrap(),
            "cutw" => cutw = v.parse().unwrap(),
            "wound" => wound = v.parse().unwrap(),
            "ring" => ring = v.parse().unwrap(),
            "unfed" => unfed = v == "1",
            "half" => half = v == "1",
            "look" => look = Some(v.into()),
            "series" => {
                let mut f = std::fs::File::create(v).expect("series file");
                use std::io::Write;
                writeln!(f, "key,t,n,frag,lcs,rms,compact,cx,cy,touching,took,energy").unwrap();
                series = Some(f);
            }
            "arm" => {
                let (name, kv) = v.split_once(':').unwrap_or((v, ""));
                let kvs = kv.split('+').filter(|s| !s.is_empty()).map(|s| {
                    let (a, b) = s.split_once('=').expect("key=value in arm");
                    (a.to_string(), b.to_string())
                });
                arms.push((name.into(), kvs.collect()));
            }
            _ => panic!("unknown {k}"),
        }
    }
    if arms.is_empty() {
        arms.push(("base".into(), Vec::new()));
    }
    if let Some(d) = &look {
        std::fs::create_dir_all(d).expect("look dir");
    }
    println!("cutprobe race world scale {scale} walkers {walkers}, grow {grow} s, follow {after} s, {ncuts} cuts a seed, seeds {seeds:?}");
    println!("arms: {arms:?}");
    println!("arm seed cut | n0 back fed | lcs20 frag_mean lcs_mean rms20 compact20 | life fate at moved | grown cut_ms");
    // results[arm][(seed, cut)]
    let mut results: Vec<Vec<((u64, usize), Run)>> = vec![Vec::new(); arms.len()];
    for &seed in &seeds {
        for (ai, (name, kvs)) in arms.iter().enumerate() {
            let mut p = race_params(seed, scale, walkers);
            for (k, v) in kvs {
                set(&mut p, k, v);
            }
            let mut d = BigDish::new(p);
            d.threads = 4;
            let t0 = std::time::Instant::now();
            for _ in 0..grow * TPS {
                d.tick();
            }
            let grow_ms = t0.elapsed().as_secs_f64() * 1000.0 / (grow * TPS) as f64;
            let alive = (0..d.walkers.len()).filter(|&k| holds(&d, k)).count();
            // The whole mould as grown: its parts (as for a piece, 1% or
            // more of it) and the largest part's share; crystals it eats.
            {
                let all: Vec<u8> = (0..d.walkers.len()).map(|k| if holds(&d, k) { 1 } else { 0 }).collect();
                let mut dd = d.clone();
                dd.p.energy_flow = 0.0;
                let s = sample(&d, &all, 0.0);
                let (lab, nl) = label(&dilate(&d.occupancy().iter().map(|&o| o > 0).collect::<Vec<_>>(), d.w, d.h, d.p.scale), d.w, d.h);
                let mut count = vec![0usize; nl as usize + 1];
                for k in 0..d.walkers.len() {
                    if holds(&d, k) {
                        count[lab[cell_of(&d, k)] as usize] += 1;
                    }
                }
                let parts = count[1..].iter().filter(|&&c| c * 100 >= alive).count();
                let fed = (0..d.pile_charge.len()).filter(|&i| d.pile_charge[i] < 0.99 * d.pile_full[i]).count();
                let lone = (0..d.walkers.len()).filter(|&k| holds(&d, k) && count[lab[cell_of(&d, k)] as usize] < 5).count();
                println!("{name} {seed} grown | alive {alive} parts1% {parts} lcs {:.2} lone {:.2} rms {:.1} | crystals eaten from {fed} of {}", s.lcs, lone as f32 / alive as f32, s.rms, d.pile_charge.len());
            }
            let cuts = if ring > 0.0 { choose_rings(&d, ncuts, ring, cutw, unfed) } else { choose_cuts(&d, ncuts, cutw) };
            if let Some(dir) = &look {
                let tag: Vec<u8> = vec![2; d.walkers.len()];
                image(&d, &tag).save(format!("{dir}/{name}-s{seed}-grown.png")).expect("png");
            }
            for (ci, cut) in cuts.iter().enumerate() {
                let key = format!("{name},{seed},{ci}");
                let lk = look.as_ref().map(|dir| (dir.as_str(), format!("{name}-s{seed}-c{ci}")));
                println!("cut {name} {seed} {ci} at fine cell ({}, {})", cut.tip % d.w, cut.tip / d.w);
                let r = follow(&d, cut, after, wound, half, lk, &mut series, &key);
                println!(
                    "{name} {seed} {ci} | {} {:.0} {} | {:.2} {:.2} {:.2} {:.1} {:.2} | {:.0} {} {:.0} {:.1} react {:.0} | {alive} {grow_ms:.2}",
                    r.n0, r.back, r.fed as u8, r.lcs20, r.frag_mean, r.lcs_mean, r.rms20, r.compact20, r.life, r.fate, r.fate_t, r.moved, r.react
                );
                results[ai].push(((seed, ci), r));
            }
        }
    }
    // Summary: each arm's means, then paired against the first arm.
    println!("\nsummary (mean over cuts)");
    for (ai, (name, _)) in arms.iter().enumerate() {
        let rs = &results[ai];
        let m = |f: &dyn Fn(&Run) -> f32| rs.iter().map(|(_, r)| f(r)).sum::<f32>() / rs.len().max(1) as f32;
        let mut fates = std::collections::BTreeMap::new();
        for (_, r) in rs {
            *fates.entry(r.fate.clone()).or_insert(0) += 1;
        }
        println!(
            "{name}: cuts {} n0 {:.0} lcs20 {:.2} frag {:.2} lcs {:.2} rms20 {:.1} compact20 {:.2} life {:.0} moved {:.1} fates {fates:?}",
            rs.len(),
            m(&|r| r.n0 as f32),
            m(&|r| r.lcs20),
            m(&|r| r.frag_mean),
            m(&|r| r.lcs_mean),
            m(&|r| r.rms20),
            m(&|r| r.compact20),
            m(&|r| r.life),
            m(&|r| r.moved)
        );
        if ai == 0 {
            continue;
        }
        // Pair by (seed, cut index): the cut lands on each arm's own mould.
        for (label, f) in [("lcs20", (|r: &Run| r.lcs20) as fn(&Run) -> f32), ("life", |r: &Run| r.life), ("compact20", |r: &Run| -r.compact20)] {
            let (mut up, mut down) = (0, 0);
            for (k, r) in rs {
                if let Some((_, b)) = results[0].iter().find(|(kk, _)| kk == k) {
                    let (x, y) = (f(r), f(b));
                    if x > y {
                        up += 1;
                    } else if x < y {
                        down += 1;
                    }
                }
            }
            println!("  vs {}: {label} better {up} worse {down}, p {:.4}", arms[0].0, sign_p(up, down));
        }
    }
}
