//! **Plants, looked at** (`mould::plants`, 2026-11-14): a generated planet
//! with a stand-in start for the plants (levelgen writes the real one per
//! biome), the slime on it, three craters at 20 s, and stills of the plant
//! field with the slime over it.
//!
//! `cargo run --release --example plantprobe -- [seed=1] [depth=1] [secs=90] [out=DIR] [at=0,20,30,60,90] [fire=40] [save=DIR]`
//!
//! `save=DIR` writes the planet with its stand-in plants as a map and stops.
//!
//! Prints a line a still: seconds, the plant field's sum and the share of
//! open cells with plants on, each kind's cells, and the CPU cost of one
//! plant tick (median of the ticks timed).
//!
//! The start is `plants::stand_in` with one cell in 40 planted, so the
//! stills show the fronts filling in.

use std::path::PathBuf;
use std::time::Instant;

use slime_mould::mould::big::BigDish;
use slime_mould::mould::larder_world::race_params;
use slime_mould::mould::levelgen;
use slime_mould::mould::plants::{self, KINDS};

/// The plant kinds' colours (a stand-in for the look, which is the graphics
/// lead's): meadow yellow-green, jungle deep green, reeds blue-green, scrub
/// olive brown.
const COL: [[f32; 3]; KINDS] = [
    [150.0, 200.0, 70.0],
    [30.0, 120.0, 45.0],
    [70.0, 170.0, 150.0],
    [150.0, 140.0, 70.0],
];

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
        if !d.char.is_empty() {
            let k = 1.0 - 0.75 * d.char[i];
            c = [c[0] * k, c[1] * k, c[2] * k];
        }
        if !d.fire.is_empty() && d.fire[i] > 0.0 {
            let f = d.fire[i];
            for (v, t) in c.iter_mut().zip([255.0, 110.0, 20.0]) {
                *v += (t - *v) * f;
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
    let img = image::imageops::resize(
        &img,
        d.w as u32 * 2,
        d.h as u32 * 2,
        image::imageops::FilterType::Nearest,
    );
    img.save(path).unwrap();
}

fn main() -> Result<(), String> {
    let mut fire_at = 40u32;
    let mut save: Option<PathBuf> = None;
    let (mut seed, mut depth, mut secs, mut out, mut at) = (
        1u64,
        1u32,
        120u32,
        PathBuf::from("plantprobe"),
        vec![0u32, 20, 30, 40, 41, 42, 44, 47, 50, 60, 90, 120],
    );
    for a in std::env::args().skip(1) {
        let (k, v) = a.split_once('=').ok_or(format!("expected key=value, got {a}"))?;
        let num = |v: &str| v.parse::<u64>().map_err(|e| format!("{k}: {e}"));
        match k {
            "seed" => seed = num(v)?,
            "depth" => depth = num(v)? as u32,
            "secs" => secs = num(v)? as u32,
            "out" => out = PathBuf::from(v),
            "fire" => fire_at = num(v)? as u32,
            "save" => save = Some(PathBuf::from(v)),
            "at" => {
                at = v
                    .split(',')
                    .map(|s| s.parse().map_err(|e| format!("at: {e}")))
                    .collect::<Result<_, String>>()?
            }
            _ => return Err(format!("unknown key {k} (seed, depth, secs, out, at, fire, save)")),
        }
    }
    std::fs::create_dir_all(&out).map_err(|e| e.to_string())?;
    let mut m = levelgen::level(seed, depth);
    // A saved map starts full grown; the probe's run from one cell in 40.
    plants::stand_in(&mut m, seed as u32, if save.is_some() { 1.0 } else { 1.0 / 40.0 });
    // `save=DIR`: the planet with its stand-in plants as the loader's
    // files, for the game (`larder-world map=DIR/plants-DEPTH-SEED`).
    if let Some(dir) = &save {
        std::fs::create_dir_all(dir).map_err(|e| e.to_string())?;
        m.save(dir, &format!("plants-{depth}-{seed}"))?;
        println!("saved {}", dir.join(format!("plants-{depth}-{seed}.png")).display());
        return Ok(());
    }
    let mut d = BigDish::with_map(race_params(seed, 2, 3000), &m);
    let open = d.agar.iter().filter(|&&a| a).count().max(1);
    println!(
        "planet {} ({}), {} x {} fine cells, {open} open",
        depth,
        levelgen::planet(seed, depth).name,
        d.w,
        d.h
    );
    println!(
        "secs sum cover% | meadow jungle reeds scrub | plant step ms (every {} ticks)",
        d.p.plant_every
    );
    // The race world's game ticks a second (`larder_world::Rules::tps`).
    let tps = slime_mould::mould::larder_world::Rules::default().tps;
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
            if s == fire_at {
                // A ring of eight lights 25 dish cells round the slime's
                // middle: whatever stands in plants there burns toward it.
                let sc = d.p.scale as f32;
                let live: Vec<(f32, f32)> = (0..d.walkers.len())
                    .filter(|&k| d.alive.get(k).copied().unwrap_or(true))
                    .map(|k| (d.walkers[k].0 / sc, d.walkers[k].1 / sc))
                    .collect();
                let n = live.len().max(1) as f32;
                let (cx, cy) = live.iter().fold((0.0, 0.0), |a, w| (a.0 + w.0 / n, a.1 + w.1 / n));
                let mut lit = 0;
                for k in 0..8 {
                    let a = k as f32 * std::f32::consts::FRAC_PI_4;
                    lit += d.ignite(cx + 25.0 * a.cos(), cy + 25.0 * a.sin(), 3.0);
                }
                let mut tr: Vec<f32> = live
                    .iter()
                    .map(|&(x, y)| d.trail[(y * sc) as usize * d.w + (x * sc) as usize])
                    .collect();
                tr.sort_by(f32::total_cmp);
                let q = |f: f32| tr.get((f * tr.len() as f32) as usize).copied().unwrap_or(0.0);
                println!("fire at {s} s round ({cx:.0}, {cy:.0}): {lit} cells lit; trail under walkers p10 {:.2} p25 {:.2} p50 {:.2} p90 {:.2}", q(0.1), q(0.25), q(0.5), q(0.9));
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
                let living = (0..d.walkers.len())
                    .filter(|&k| d.alive.get(k).copied().unwrap_or(true))
                    .count();
                println!(
                    "{s} {:.0} {:.1} | {} {} {} {} | {ms:.3} | burning {} fire-killed {} living {living}",
                    d.plant.iter().sum::<f32>(),
                    100.0 * on as f32 / open as f32,
                    kinds[0],
                    kinds[1],
                    kinds[2],
                    kinds[3],
                    d.burning,
                    d.fire_killed
                );
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
                let g = plants::Ground {
                    w,
                    h,
                    cap: &d.plant_cap,
                    agar: &d.agar,
                    pile_of: &d.pile_of,
                    wet: &d.wet,
                    scorch: &d.scorch,
                };
                let r = plants::Rates {
                    grow: d.p.plant_grow,
                    spread: d.p.plant_spread,
                    wet: d.p.plant_wet,
                    scorch: d.p.plant_scorch,
                    kinds: d.p.plant_kinds,
                };
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
