//! **Plants, looked at** (`mould::plants`, 2026-11-14): a generated planet
//! with a stand-in start for the plants (levelgen writes the real one per
//! biome), the slime on it, three craters at 20 s, and stills of the plant
//! field with the slime over it.
//!
//! `cargo run --release --example plantprobe -- [seed=1] [depth=1] [secs=90] [out=DIR] [at=0,20,30,60,90]`
//!
//! Prints a line a still: seconds, the plant field's sum and the share of
//! open cells with plants on, each kind's cells, and the CPU cost of one
//! plant tick (median of the ticks timed).
//!
//! The stand-in start: reeds on wet ground; elsewhere a low-frequency value
//! noise picks jungle in its high part, bare ground in its low part and
//! meadow between, and only one cell in 40 starts planted, so the stills
//! show the fronts filling in.

use std::path::PathBuf;
use std::time::Instant;

use slime_mould::mould::big::BigDish;
use slime_mould::mould::larder_world::race_params;
use slime_mould::mould::levelgen;
use slime_mould::mould::map::{Map, G_ROCK, G_WET, MAP_H, MAP_W};
use slime_mould::mould::plants::{self, JUNGLE, KINDS, MEADOW, REEDS, SCRUB};

fn hash(x: u32, y: u32, s: u32) -> u32 {
    let mut h = x.wrapping_mul(0x9E37_79B1) ^ y.wrapping_mul(0x85EB_CA77) ^ s.wrapping_mul(0xC2B2_AE3D);
    h ^= h >> 15;
    h = h.wrapping_mul(0x2C1B_3C6D);
    h ^= h >> 12;
    h
}

fn unit(h: u32) -> f32 {
    (h >> 8) as f32 / 16_777_216.0
}

/// Value noise on a `cell`-cell lattice, 0-1.
fn noise(x: usize, y: usize, cell: usize, s: u32) -> f32 {
    let (gx, gy) = ((x / cell) as u32, (y / cell) as u32);
    let (fx, fy) = ((x % cell) as f32 / cell as f32, (y % cell) as f32 / cell as f32);
    let v = |a: u32, b: u32| unit(hash(a, b, s));
    let top = v(gx, gy) * (1.0 - fx) + v(gx + 1, gy) * fx;
    let bot = v(gx, gy + 1) * (1.0 - fx) + v(gx + 1, gy + 1) * fx;
    top * (1.0 - fy) + bot * fy
}

fn stand_in(m: &mut Map, seed: u32) {
    m.plants = (0..MAP_W * MAP_H)
        .map(|i| {
            let (x, y) = (i % MAP_W, i / MAP_W);
            let g = m.ground[i];
            if g & G_ROCK != 0 || unit(hash(x as u32, y as u32, seed ^ 77)) > 1.0 / 40.0 {
                return 0;
            }
            if g & G_WET != 0 {
                return plants::pack(REEDS, 1.0);
            }
            let n = noise(x, y, 96, seed);
            if n > 0.62 {
                plants::pack(JUNGLE, 1.0)
            } else if n < 0.3 {
                0
            } else if g >> 4 == 1 {
                plants::pack(SCRUB, 1.0)
            } else {
                plants::pack(MEADOW, 1.0)
            }
        })
        .collect();
}

/// The plant kinds' colours (a stand-in for the look, which is the graphics
/// lead's): meadow yellow-green, jungle deep green, reeds blue-green, scrub
/// olive brown.
const COL: [[f32; 3]; KINDS] = [[150.0, 200.0, 70.0], [30.0, 120.0, 45.0], [70.0, 170.0, 150.0], [150.0, 140.0, 70.0]];

fn still(d: &BigDish, path: &PathBuf) {
    let img = image::RgbImage::from_fn(d.w as u32, d.h as u32, |x, y| {
        let i = y as usize * d.w + x as usize;
        let mut c = if !d.agar[i] {
            [40.0, 40.0, 44.0]
        } else if !d.wet.is_empty() && d.wet[i] {
            [70.0, 80.0, 95.0]
        } else {
            [95.0, 85.0, 70.0]
        };
        if !d.pile_of.is_empty() && d.pile_of[i] != 0 {
            c = [120.0, 220.0, 255.0];
        }
        if !d.plant.is_empty() {
            let (p, k) = (d.plant[i], d.plant_kind[i] as usize);
            for (v, t) in c.iter_mut().zip(COL[k]) {
                *v += (t - *v) * p;
            }
        }
        if !d.scorch.is_empty() && d.scorch[i] > 0 {
            c = [c[0] * 0.4, c[1] * 0.4, c[2] * 0.4];
        }
        let t = (d.trail[i] / 4.0).min(1.0);
        for (v, s) in c.iter_mut().zip([250.0, 220.0, 60.0]) {
            *v += (s - *v) * t;
        }
        image::Rgb(c.map(|v| v.clamp(0.0, 255.0) as u8))
    });
    let img = image::imageops::resize(&img, d.w as u32 * 2, d.h as u32 * 2, image::imageops::FilterType::Nearest);
    img.save(path).unwrap();
}

fn main() -> Result<(), String> {
    let (mut seed, mut depth, mut secs, mut out, mut at) = (1u64, 1u32, 120u32, PathBuf::from("plantprobe"), vec![0u32, 20, 25, 30, 60, 90, 120]);
    for a in std::env::args().skip(1) {
        let (k, v) = a.split_once('=').ok_or(format!("expected key=value, got {a}"))?;
        let num = |v: &str| v.parse::<u64>().map_err(|e| format!("{k}: {e}"));
        match k {
            "seed" => seed = num(v)?,
            "depth" => depth = num(v)? as u32,
            "secs" => secs = num(v)? as u32,
            "out" => out = PathBuf::from(v),
            "at" => at = v.split(',').map(|s| s.parse().map_err(|e| format!("at: {e}"))).collect::<Result<_, String>>()?,
            _ => return Err(format!("unknown key {k} (seed, depth, secs, out, at)")),
        }
    }
    std::fs::create_dir_all(&out).map_err(|e| e.to_string())?;
    let mut m = levelgen::level(seed, depth);
    stand_in(&mut m, seed as u32);
    let mut d = BigDish::with_map(race_params(seed, 2, 3000), &m);
    let open = d.agar.iter().filter(|&&a| a).count().max(1);
    println!("planet {} ({}), {} x {} fine cells, {open} open", depth, levelgen::planet(seed, depth).name, d.w, d.h);
    println!("secs sum cover% | meadow jungle reeds scrub | plant step ms (every {} ticks)", d.p.plant_every);
    let tps = slime_mould::mould::STEPS_PER_SECOND;
    let mut costs = Vec::new();
    for t in 0..=secs * tps {
        if t % tps == 0 {
            let s = t / tps;
            if s == 20 {
                // Three craters across the middle: bare scars that green
                // back from their rims once the scorch is gone.
                for k in 1..=3 {
                    d.scorch_disc(64.0 * k as f32 + 20.0, 80.0, 8.0, 2400);
                }
            }
            if at.contains(&s) {
                let on = d.plant.iter().filter(|&&p| p > 0.05).count();
                let mut kinds = [0usize; KINDS];
                for (k, &p) in d.plant_kind.iter().zip(&d.plant) {
                    if p > 0.05 {
                        kinds[*k as usize] += 1;
                    }
                }
                costs.sort_by(f64::total_cmp);
                let ms = costs.get(costs.len() / 2).copied().unwrap_or(0.0);
                println!("{s} {:.0} {:.1} | {} {} {} {} | {ms:.3}", d.plant.iter().sum::<f32>(), 100.0 * on as f32 / open as f32, kinds[0], kinds[1], kinds[2], kinds[3]);
                still(&d, &out.join(format!("plants-{depth}-{seed}-{s:03}s.png")));
            }
        }
        if t < secs * tps {
            let t0 = Instant::now();
            d.tick();
            // The plant tick alone, once in 30: a second copy stepped on
            // the side so the timing doesn't move the run.
            if t % 30 == 0 {
                let (w, h) = (d.w, d.h);
                let g = plants::Ground { w, h, cap: &d.plant_cap, agar: &d.agar, pile_of: &d.pile_of, wet: &d.wet, scorch: &d.scorch };
                let r = plants::Rates { grow: d.p.plant_grow, spread: d.p.plant_spread, wet: d.p.plant_wet, scorch: d.p.plant_scorch, kinds: d.p.plant_kinds };
                // (One step's cost; the world runs it every `plant_every` ticks.)
                let (mut to, mut kind) = (vec![0.0; w * h], d.plant_kind.clone());
                let t1 = Instant::now();
                plants::step(&g, r, &d.plant, &mut to, &mut kind);
                costs.push(t1.elapsed().as_secs_f64() * 1000.0);
            }
            let _ = t0;
        }
    }
    Ok(())
}
