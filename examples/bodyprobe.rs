//! **The body probe** (mini-boss thread, 2026-11-16; owner 03:41Z: "The
//! slime forms a stronger, more aggressive, sub-unit that leaves to seek out
//! the player"). Every mini-boss option on the round-1 page needs a held
//! body: a few hundred walkers that travel to a target as one lump, where
//! today's break-aways and raids stream there as single specks
//! (`Reports/mini-boss-2026-11-16/`).
//!
//! ```text
//! cargo run --release --example bodyprobe -- [seeds=1-4 n=600 dist=70 secs=40]
//!     [arm=NAME:key=v+key=v ...] [look=DIR every=0.5] [threads=4]
//! ```
//!
//! A blank map (no rock, no water), one crystal as the target `dist` dish
//! cells east of a disc of `n` walkers, no growth (upkeep 0, so the count
//! is fixed and only the rules move it), the CPU engine at the race world's
//! scale 2 and 15 ticks a game second. Each arm sets any `BigParams` key
//! (`tiers::set_key`'s, plus the one-body rules below). Only one population
//! is on the dish, so a global rule here is the body's own rule: this asks
//! which rules make a body hold and travel, before asking how to give them
//! to a sub-population of the main slime.
//!
//! **Measures, named before the first run** (every game second):
//! - `held`: the largest part's share of the walkers (8-connected over
//!   cells holding walkers, gaps up to `gap` fine cells bridged, default 2
//!   dish cells = 4 fine). **Deciding: `held` at 10 s and 20 s, and at
//!   arrival.** 1 is one lump; low is scattered specks.
//! - `compact`: walkers' rms distance from their centre over a full disc's
//!   of the same count at one walker a cell (1 is a packed disc).
//! - `aspect`: the square root of the ratio of the walkers' covariance
//!   eigenvalues (1 round, 3 a sausage three times as long as wide).
//! - `went`: the centre's distance travelled toward the target, dish cells;
//!   `arrive`: first second the centre is within the crystal's radius + 4.
//!
//! With `look=DIR`: each arm's first seed as PNG frames every `every`
//! game seconds (walkers bright, trail amber, the crystal cyan), for a clip.
//!
//! `walk=VX,VY` moves the aim point (lump 0's `aim`) that many dish cells
//! a game second from the crystal, as a squad walking past (drawn as a
//! white ring), and `went` then still counts eastward travel; `circle=R,P`
//! walks it round the crystal instead, R dish cells out, once every P
//! game seconds.

use std::collections::VecDeque;
use std::sync::mpsc;

use slime_mould::mould::big::{BigDish, BigParams, LUMP_SHIFT};
use slime_mould::mould::larder_world::race_params;
use slime_mould::mould::map::{Crystal, Map};
use slime_mould::mould::tiers::set_key;

const TPS: u64 = 15;
const SCALE: usize = 2;
/// The blank map, dish cells.
const DW: usize = 200;
const DH: usize = 100;

fn key(p: &mut BigParams, k: &str, v: &str) {
    let f = || v.parse::<f32>().unwrap_or_else(|_| panic!("{k}={v}"));
    match k {
        "lone_step" => p.lone_step = f(),
        "lone_floor" => p.lone_floor = f(),
        "body_r" => p.body_r = f() as u8,
        "blocked_keep" => p.blocked_keep = f() != 0.0,
        "blocked_reset" => p.blocked_reset = f(),
        "blocked_retry" => p.blocked_retry = f() as u8,
        "graze_weight" => p.graze_weight = f(),
        "graze_eat" => p.graze_eat = f(),
        "graze_regrow" => p.graze_regrow = f(),
        "crowd" => p.crowd = f() as u8,
        "deposit" => p.deposit = f(),
        "hunger_reach" => p.hunger_reach = f(),
        // The probe's one lump (`BigParams::lumps`, lump 0: every walker).
        "aim_weight" => p.lumps[0].pull = f(),
        "leash" => p.lumps[0].leash = f(),
        "haste" => p.lumps[0].haste = f(),
        _ => set_key(p, k, v).unwrap_or_else(|e| panic!("{e}")),
    }
}

struct Arm {
    name: String,
    keys: Vec<(String, String)>,
}

#[derive(Default, Clone)]
struct Sample {
    held: f32,
    compact: f32,
    aspect: f32,
    went: f32,
}

struct Row {
    arm: usize,
    seed: u64,
    series: Vec<Sample>,
    arrive: Option<u32>,
    ms: f64,
}

fn world(seed: u64, n: usize, dist: f32, arm: &Arm) -> (BigDish, (f32, f32), f32) {
    let (sx, sy) = (40.0f32, DH as f32 / 2.0);
    let (tx, ty) = (sx + dist, sy);
    let tr = 5.0;
    let mut m = Map::blank(DW * 4, DH * 4);
    m.items.moulds.push((sx * 4.0, sy * 4.0));
    m.items.crystals.push(Crystal { x: tx * 4.0, y: ty * 4.0, r: tr * 4.0, charge: 512.0 });
    let mut p = BigParams { upkeep: 0.0, terrain_rock: 0.0, terrain_wet: 0.0, start_shape: false, piles: 1, ..race_params(seed, SCALE, n) };
    let s = SCALE as f32;
    p.lumps[0].aim = cell_of((tx * s, ty * s));
    for (k, v) in &arm.keys {
        key(&mut p, k, v);
    }
    let mut d = BigDish::with_map(p, &m);
    // Every walker is in lump 0 (`big::LUMP_SHIFT`), so the whole dish is
    // the body. (Condensed too until condensing was deleted, 11-17.)
    d.dense = vec![1 << LUMP_SHIFT; d.walkers.len()];
    (d, (tx * s, ty * s), tr * s)
}

/// A fine cell as the lumps' aim holds it, x | y << 16.
fn cell_of((x, y): (f32, f32)) -> u32 {
    (x.max(0.0).floor() as u32).min(0xFFFF) | (y.max(0.0).floor() as u32).min(0xFFFF) << 16
}

/// The lump's aim, the middle of its cell (fine cells).
fn aim_of(d: &BigDish) -> (f32, f32) {
    let a = d.p.lumps[0].aim;
    ((a & 0xFFFF) as f32 + 0.5, (a >> 16) as f32 + 0.5)
}

/// The measures over the walkers now (fine cells).
fn sample(d: &BigDish, start_x: f32, gap: usize) -> Sample {
    let pts: Vec<(f32, f32)> = d.walkers.iter().enumerate().filter(|(k, _)| d.alive.is_empty() || d.alive[*k]).map(|(_, w)| (w.0, w.1)).collect();
    let n = pts.len().max(1) as f32;
    let (cx, cy) = pts.iter().fold((0.0, 0.0), |a, p| (a.0 + p.0, a.1 + p.1));
    let (cx, cy) = (cx / n, cy / n);
    let (mut sxx, mut syy, mut sxy) = (0.0f32, 0.0f32, 0.0f32);
    for &(x, y) in &pts {
        sxx += (x - cx) * (x - cx);
        syy += (y - cy) * (y - cy);
        sxy += (x - cx) * (y - cy);
    }
    let (sxx, syy, sxy) = (sxx / n, syy / n, sxy / n);
    let rms = (sxx + syy).sqrt();
    // A full disc of n cells: radius sqrt(n / pi), rms r / sqrt(2).
    let disc = (n / std::f32::consts::PI).sqrt() / std::f32::consts::SQRT_2;
    let tr = sxx + syy;
    let det = sxx * syy - sxy * sxy;
    let l1 = tr / 2.0 + ((tr * tr / 4.0 - det).max(0.0)).sqrt();
    let l2 = (tr - l1).max(1e-6);
    // Largest part: cells with walkers, dilated by `gap`, flood-filled.
    let (w, h) = (d.w, d.h);
    let mut has = vec![false; w * h];
    for &(x, y) in &pts {
        let (xi, yi) = (x as usize, y as usize);
        if xi < w && yi < h {
            has[yi * w + xi] = true;
        }
    }
    let g = gap as i64;
    let mut grown = vec![false; w * h];
    for y in 0..h as i64 {
        for x in 0..w as i64 {
            if has[y as usize * w + x as usize] {
                for dy in -g..=g {
                    for dx in -g..=g {
                        let (u, v) = (x + dx, y + dy);
                        if u >= 0 && v >= 0 && (u as usize) < w && (v as usize) < h {
                            grown[v as usize * w + u as usize] = true;
                        }
                    }
                }
            }
        }
    }
    let mut label = vec![0u32; w * h];
    let mut sizes = vec![0usize];
    for i in 0..w * h {
        if grown[i] && label[i] == 0 {
            let id = sizes.len() as u32;
            sizes.push(0);
            let mut q = VecDeque::from([i]);
            label[i] = id;
            while let Some(c) = q.pop_front() {
                let (x, y) = ((c % w) as i64, (c / w) as i64);
                for dy in -1..=1 {
                    for dx in -1..=1 {
                        let (u, v) = (x + dx, y + dy);
                        if u >= 0 && v >= 0 && (u as usize) < w && (v as usize) < h {
                            let j = v as usize * w + u as usize;
                            if grown[j] && label[j] == 0 {
                                label[j] = id;
                                q.push_back(j);
                            }
                        }
                    }
                }
            }
        }
    }
    for &(x, y) in &pts {
        let (xi, yi) = (x as usize, y as usize);
        if xi < w && yi < h {
            sizes[label[yi * w + xi] as usize] += 1;
        }
    }
    let big = *sizes.iter().max().unwrap_or(&0) as f32;
    Sample { held: big / n, compact: rms / disc, aspect: (l1 / l2).sqrt(), went: (cx - start_x) / SCALE as f32 }
}

/// Walkers bright on dark ground, trail amber, the crystal cyan, up 3x.
fn picture(d: &BigDish, target: (f32, f32), tr: f32) -> image::RgbImage {
    let aim = aim_of(d);
    let up = 3u32;
    let (w, h) = (d.w as u32, d.h as u32);
    let mut img = image::RgbImage::new(w * up, h * up);
    let mut count = vec![0u16; d.w * d.h];
    for (k, wk) in d.walkers.iter().enumerate() {
        if !d.alive.is_empty() && !d.alive[k] {
            continue;
        }
        let (x, y) = (wk.0 as usize, wk.1 as usize);
        if x < d.w && y < d.h {
            count[y * d.w + x] += 1;
        }
    }
    for y in 0..h {
        for x in 0..w {
            let i = (y * w + x) as usize;
            let t = 1.0 - (-d.trail[i] / 6.0).exp();
            let mut c = [18.0 + 120.0 * t, 16.0 + 70.0 * t, 12.0 + 20.0 * t];
            if (x as f32 - target.0).hypot(y as f32 - target.1) < tr {
                c = [40.0, 150.0, 170.0];
            }
            let ra = (x as f32 - aim.0).hypot(y as f32 - aim.1);
            if (aim.0 - target.0).hypot(aim.1 - target.1) > 1.0 && (3.0..4.5).contains(&ra) {
                c = [235.0, 240.0, 245.0];
            }
            if count[i] > 0 {
                let b = (count[i] as f32).min(3.0) / 3.0;
                c = [230.0 + 25.0 * b, 170.0 + 60.0 * b, 70.0 + 120.0 * b];
            }
            let px = image::Rgb([c[0].min(255.0) as u8, c[1].min(255.0) as u8, c[2].min(255.0) as u8]);
            for dy in 0..up {
                for dx in 0..up {
                    img.put_pixel(x * up + dx, y * up + dy, px);
                }
            }
        }
    }
    img
}

fn main() {
    let mut seeds = 1..=4u64;
    let (mut n, mut dist, mut secs, mut threads, mut gap) = (600usize, 70.0f32, 40u32, 4usize, 4usize);
    let (mut look, mut every) = (None::<String>, 0.5f32);
    let mut walk = (0.0f32, 0.0f32);
    let mut circle = (0.0f32, 1.0f32);
    let mut arms: Vec<Arm> = Vec::new();
    for a in std::env::args().skip(1) {
        let (k, v) = a.split_once('=').unwrap_or_else(|| panic!("{a}"));
        match k {
            "seeds" => {
                let (lo, hi) = v.split_once('-').unwrap_or((v, v));
                seeds = lo.parse().unwrap()..=hi.parse().unwrap();
            }
            "n" => n = v.parse().unwrap(),
            "dist" => dist = v.parse().unwrap(),
            "secs" => secs = v.parse().unwrap(),
            "threads" => threads = v.parse().unwrap(),
            "gap" => gap = v.parse().unwrap(),
            "look" => look = Some(v.to_string()),
            "every" => every = v.parse().unwrap(),
            "circle" => {
                let (a, b) = v.split_once(',').unwrap();
                circle = (a.parse().unwrap(), b.parse().unwrap());
            }
            "walk" => {
                let (a, b) = v.split_once(',').unwrap();
                walk = (a.parse().unwrap(), b.parse().unwrap());
            }
            "arm" => {
                let (name, rest) = v.split_once(':').unwrap_or((v, ""));
                let keys = rest.split('+').filter(|s| !s.is_empty()).map(|kv| {
                    let (k, v) = kv.split_once('=').unwrap_or_else(|| panic!("{kv}"));
                    (k.to_string(), v.to_string())
                });
                arms.push(Arm { name: name.to_string(), keys: keys.collect() });
            }
            _ => panic!("unknown argument {k}"),
        }
    }
    if arms.is_empty() {
        arms.push(Arm { name: "plain".into(), keys: Vec::new() });
    }
    let jobs: Vec<(usize, u64)> = (0..arms.len()).flat_map(|a| seeds.clone().map(move |s| (a, s))).collect();
    let first_seed = *seeds.start();
    let (tx, rx) = mpsc::channel();
    let next = std::sync::atomic::AtomicUsize::new(0);
    std::thread::scope(|sc| {
        for _ in 0..threads {
            let (tx, jobs, arms, next, look, walk) = (tx.clone(), &jobs, &arms, &next, &look, walk);
            let circle = circle;
            sc.spawn(move || loop {
                let j = next.fetch_add(1, std::sync::atomic::Ordering::Relaxed);
                let Some(&(a, seed)) = jobs.get(j) else { break };
                let (mut d, target, tr) = world(seed, n, dist, &arms[a]);
                let start_x = 40.0 * SCALE as f32;
                let mut series = vec![sample(&d, start_x, gap)];
                let mut arrive = None;
                let t0 = std::time::Instant::now();
                let film = look.as_ref().filter(|_| seed == first_seed);
                let frame_every = ((every * TPS as f32).round() as u64).max(1);
                if let Some(dir) = film {
                    std::fs::create_dir_all(dir).unwrap();
                    picture(&d, target, tr).save(format!("{dir}/{}-f0000.png", arms[a].name)).unwrap();
                }
                for tk in 1..=secs as u64 * TPS {
                    if circle.0 > 0.0 {
                        let a = tk as f32 / TPS as f32 / circle.1 * std::f32::consts::TAU + std::f32::consts::PI;
                        let r = circle.0 * SCALE as f32;
                        d.p.lumps[0].aim = cell_of((target.0 + r * a.cos(), target.1 + r * a.sin()));
                    } else if walk != (0.0, 0.0) {
                        let t = tk as f32 / TPS as f32;
                        let sc = SCALE as f32;
                        d.p.lumps[0].aim = cell_of((target.0 + walk.0 * t * sc, target.1 + walk.1 * t * sc));
                    }
                    d.tick();
                    if let Some(dir) = film {
                        if tk.is_multiple_of(frame_every) {
                            picture(&d, target, tr).save(format!("{dir}/{}-f{:04}.png", arms[a].name, tk / frame_every)).unwrap();
                        }
                    }
                    if tk.is_multiple_of(TPS) {
                        let s = sample(&d, start_x, gap);
                        if arrive.is_none() && s.went * SCALE as f32 >= target.0 - start_x - tr - 4.0 * SCALE as f32 {
                            arrive = Some((tk / TPS) as u32);
                        }
                        series.push(s);
                    }
                }
                let ms = t0.elapsed().as_secs_f64() * 1000.0 / (secs as f64 * TPS as f64);
                tx.send(Row { arm: a, seed, series, arrive, ms }).unwrap();
            });
        }
    });
    drop(tx);
    let mut rows: Vec<Row> = rx.into_iter().collect();
    rows.sort_by_key(|r| (r.arm, r.seed));
    println!("bodyprobe n={n} dist={dist} secs={secs} gap={gap} seeds={first_seed}-{}", seeds.end());
    println!("{:<14} {:>4} {:>16} {:>16} {:>16} {:>14} {:>7} {:>7}", "arm", "seed", "held 5/10/20s", "compact 5/10/20", "aspect 5/10/20", "went 10/20/end", "arrive", "ms/tk");
    let at = |r: &Row, s: usize| r.series.get(s).cloned().unwrap_or_else(|| r.series.last().cloned().unwrap());
    for r in &rows {
        let (a, b, c) = (at(r, 5), at(r, 10), at(r, 20));
        let e = r.series.last().unwrap();
        println!(
            "{:<14} {:>4} {:>5.2}/{:.2}/{:.2} {:>5.2}/{:.2}/{:.2} {:>5.1}/{:.1}/{:.1} {:>4.0}/{:.0}/{:.0} {:>7} {:>7.2}",
            arms[r.arm].name, r.seed, a.held, b.held, c.held, a.compact, b.compact, c.compact, a.aspect, b.aspect, c.aspect, b.went, c.went, e.went,
            r.arrive.map_or("-".into(), |t| format!("{t}s")), r.ms
        );
    }
    // Arm means.
    println!("\nmeans");
    for (ai, arm) in arms.iter().enumerate() {
        let rs: Vec<&Row> = rows.iter().filter(|r| r.arm == ai).collect();
        let m = |f: &dyn Fn(&Row) -> f32| rs.iter().map(|r| f(r)).sum::<f32>() / rs.len() as f32;
        let arrived = rs.iter().filter(|r| r.arrive.is_some()).count();
        println!(
            "{:<14} held10 {:.2} held20 {:.2} compact20 {:.2} aspect20 {:.1} went20 {:.0} arrived {}/{} keys {}",
            arm.name,
            m(&|r| at(r, 10).held),
            m(&|r| at(r, 20).held),
            m(&|r| at(r, 20).compact),
            m(&|r| at(r, 20).aspect),
            m(&|r| at(r, 20).went),
            arrived,
            rs.len(),
            arm.keys.iter().map(|(k, v)| format!("{k}={v}")).collect::<Vec<_>>().join(" ")
        );
    }
}
