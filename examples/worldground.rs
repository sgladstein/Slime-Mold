//! Larder World maps (WM3): which kinds of ground does the mould feel?
//! (`Reports/larder-maps-2026-11-11.md`, *Which ground the mould feels*.)
//!
//! The owner, 11-11 20:06Z, asked how much ground such as height really
//! changes the mould. So before any ground goes on a map, this test runs
//! it. One world (the growth-sizing world of the Larder roadmap: 512 x
//! 320, 23 crystals of charge 512 eaten at 0.005, 5,000 walkers growing
//! into 50,000 slots, upkeep 0.001), and one patch layout a seed (keyed blobs over
//! about 30% of the ground, clear of the crystals). The arms differ only
//! in what the patch is:
//!   plain   the same cells, no effect (the control);
//!   wet     the step there x `wet_step` (0.5), the speed rule the world
//!           already has, standing in for any slowing ground such as a slope;
//!   sand    trail there x `sand_keep` each fine step (fades fast);
//!   clay    trail there x `clay_keep` (lasts);
//!   barren  upkeep there x `barren_upkeep`, and no births there.
//! Deciding measures, named before the run, each paired by seed against
//! plain with a sign test: (1) the share of living walkers on patch cells
//! at 30 s and 60 s (does the mould avoid or seek it?); (2) walkers alive
//! at 60 s (does it change the mould's size?); (3) crystal charge eaten by
//! 60 s. (A first look at crystal_eat 0.05, the default, ate 98-99% of
//! the charge by 60 s and filled all 50,000 slots by 30 s on every arm,
//! so size could not differ; 0.005 is the growth-sizing world's eat.)
//!
//!   cargo run --release --example worldground -- seeds=1-16 [seconds=60 cover=0.3
//!     arms=plain,wet,sand,clay,barren sand_keep= clay_keep= barren_upkeep= look=DIR]

use slime_mould::mould::big::{BigDish, BigParams, SOIL_BARREN, SOIL_CLAY, SOIL_SAND};

/// Smooth keyed value noise in [0, 1] on dish cells, blobs `cell` across.
fn patch_noise(seed: u64, x: f32, y: f32, cell: f32) -> f32 {
    let lattice = |i: i64, j: i64| {
        let mut z = seed ^ 0x9E37_79B9_7F4A_7C15 ^ (i as u64).wrapping_mul(0xBF58_476D_1CE4_E5B9) ^ (j as u64).wrapping_mul(0x94D0_49BB_1331_11EB);
        z = (z ^ (z >> 30)).wrapping_mul(0xBF58_476D_1CE4_E5B9);
        z = (z ^ (z >> 27)).wrapping_mul(0x94D0_49BB_1331_11EB);
        ((z ^ (z >> 31)) >> 40) as f32 / (1u64 << 24) as f32
    };
    let (gx, gy) = (x / cell, y / cell);
    let (i, j) = (gx.floor() as i64, gy.floor() as i64);
    let (fx, fy) = (gx - gx.floor(), gy - gy.floor());
    let (sx, sy) = (fx * fx * (3.0 - 2.0 * fx), fy * fy * (3.0 - 2.0 * fy));
    let a = lattice(i, j) + (lattice(i + 1, j) - lattice(i, j)) * sx;
    let b = lattice(i, j + 1) + (lattice(i + 1, j + 1) - lattice(i, j + 1)) * sx;
    a + (b - a) * sy
}

fn main() {
    let mut seeds: Vec<u64> = (1..=16).collect();
    let (mut seconds, mut cover, mut threads, mut look) = (60u32, 0.3f32, 4usize, None::<String>);
    let mut arms: Vec<String> = ["plain", "wet", "sand", "clay", "barren"].iter().map(|s| s.to_string()).collect();
    let t = BigParams::today();
    let (mut sand_keep, mut clay_keep, mut barren_upkeep) = (t.sand_keep, t.clay_keep, t.barren_upkeep);
    for a in std::env::args().skip(1) {
        let (k, v) = a.split_once('=').expect("key=value");
        match k {
            "seeds" => {
                let (a, b) = v.split_once('-').unwrap_or((v, v));
                seeds = (a.parse().unwrap()..=b.parse().unwrap()).collect();
            }
            "seconds" => seconds = v.parse().unwrap(),
            "cover" => cover = v.parse().unwrap(),
            "threads" => threads = v.parse().unwrap(),
            "look" => look = Some(v.into()),
            "arms" => arms = v.split(',').map(String::from).collect(),
            "sand_keep" => sand_keep = v.parse().unwrap(),
            "clay_keep" => clay_keep = v.parse().unwrap(),
            "barren_upkeep" => barren_upkeep = v.parse().unwrap(),
            _ => panic!("unknown {k}"),
        }
    }
    println!("worldground seconds {seconds} cover {cover} sand_keep {sand_keep} clay_keep {clay_keep} barren_upkeep {barren_upkeep}");
    let ticks_per_s = slime_mould::mould::STEPS_PER_SECOND as u64;
    for &seed in &seeds {
        for arm in arms.iter().map(String::as_str) {
            let p = BigParams {
                scale: 2,
                substeps: 2,
                walkers: 5_000,
                capacity: 50_000,
                upkeep: 0.001,
                scent_offset: 36.0,
                scent_coarse: false,
                piles: 23,
                rect: true,
                crystal_charge: 512.0,
                crystal_eat: 0.005,
                sand_keep,
                clay_keep,
                barren_upkeep,
                seed,
                ..BigParams::today()
            };
            let mut d = BigDish::new(p);
            d.threads = threads;
            let (w, h, s) = (d.w, d.h, d.p.scale as f32);
            let clear = d.p.pile_r + 4.0;
            // The patch: the lowest `cover` share of the noise (by a
            // threshold found on this seed's cells), clear of every crystal.
            let vals: Vec<f32> = (0..w * h)
                .map(|i| {
                    let (x, y) = (((i % w) as f32 + 0.5) / s, ((i / w) as f32 + 0.5) / s);
                    if !d.agar[i] || d.piles.iter().any(|&(px, py)| (px - x).hypot(py - y) <= clear) {
                        f32::INFINITY
                    } else {
                        patch_noise(seed, x, y, 16.0)
                    }
                })
                .collect();
            let mut sorted: Vec<f32> = vals.iter().copied().filter(|v| v.is_finite()).collect();
            sorted.sort_by(f32::total_cmp);
            let agar_n = d.agar.iter().filter(|&&a| a).count();
            let cut = sorted[((cover * agar_n as f32) as usize).min(sorted.len() - 1)];
            let patch: Vec<bool> = vals.iter().map(|&v| v < cut).collect();
            let kind = match arm {
                "sand" => SOIL_SAND,
                "clay" => SOIL_CLAY,
                "barren" => SOIL_BARREN,
                _ => 0,
            };
            if kind > 0 {
                d.soil = patch.iter().map(|&b| if b { kind } else { 0 }).collect();
            }
            if arm == "wet" {
                d.wet = patch.clone();
            }
            let on_patch = |d: &BigDish| {
                let (mut on, mut all) = (0usize, 0usize);
                for (k, &(x, y, _)) in d.walkers.iter().enumerate() {
                    if !d.alive.is_empty() && !d.alive[k] {
                        continue;
                    }
                    let (cx, cy) = (x as usize, y as usize);
                    if cx < w && cy < h {
                        all += 1;
                        on += patch[cy * w + cx] as usize;
                    }
                }
                (on as f32 / all.max(1) as f32, all)
            };
            let mut share = Vec::new();
            let mut alive = Vec::new();
            let start = std::time::Instant::now();
            for t in 0..seconds as u64 * ticks_per_s {
                d.tick();
                if (t + 1) % (30 * ticks_per_s) == 0 {
                    let (sh, n) = on_patch(&d);
                    share.push(sh);
                    alive.push(n);
                    if let Some(dir) = &look {
                        std::fs::create_dir_all(dir).unwrap();
                        let mut img = d.image(1);
                        // Mark the patch faintly, so the picture shows where it is.
                        for (i, px) in img.pixels_mut().enumerate() {
                            if i < patch.len() && patch[i] {
                                px.0[2] = px.0[2].saturating_add(40);
                            }
                        }
                        img.save(format!("{dir}/ground-s{seed}-{arm}-{}s.png", (t + 1) / ticks_per_s)).unwrap();
                    }
                }
            }
            let eaten: f32 = d.pile_full.iter().zip(&d.pile_charge).map(|(f, c)| f - c).sum();
            let full: f32 = d.pile_full.iter().sum();
            println!(
                "seed {seed:2} {arm:6} patch {:.3} of agar  on-patch share {}  alive {}  eaten {:.4}  ms/tick {:.2}  hash {:016x}",
                patch.iter().filter(|&&b| b).count() as f32 / agar_n as f32,
                share.iter().map(|v| format!("{v:.3}")).collect::<Vec<_>>().join(" "),
                alive.iter().map(|v| v.to_string()).collect::<Vec<_>>().join(" "),
                eaten / full,
                start.elapsed().as_secs_f32() * 1000.0 / (seconds as u64 * ticks_per_s) as f32,
                d.hash()
            );
        }
    }
}
