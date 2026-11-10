//! The engine at any scale (`slime_mould::mould::big`): the GPU plan's
//! phase 2 offline dish, and phase 1's scaling-rule arms.
//!
//! ```text
//! cargo build --release --example bigdish
//! ./target/release/examples/bigdish preset=smell seeds=1 seconds=60 threads=4
//! ./target/release/examples/bigdish preset=dense gif=out/ every=12 down=2
//! ./target/release/examples/bigdish preset=reach tag=reach-blob look=out/ at=5,15,30,60
//! ```
//!
//! Presets (all from today's numbers, `BigParams::today`). From `naive`
//! on, each adds one part of the scaling rule to the one before:
//! - `today`: today's grid, 256 x 160, 2,000 walkers.
//! - `naive`: 1024 x 640 and 32,000 walkers (today's density), every number
//!   as today but in fine cells: a step a tick, scent on the fine grid. A
//!   walker crosses a quarter of a dish cell a tick and smells a quarter as
//!   far.
//! - `steps`: four fine steps a tick (today's speed), with deposit, trail
//!   kept, scent laid and scent kept scaled so a tick fades as today's.
//! - `smell`: scent back on today's grid, stepped once a tick (today's
//!   smell range).
//! - `reach`: sensors 36 fine cells out (today's 9 dish cells).
//! - `spread`: four 3x3 passes on the trail a fine step (today's trail
//!   spread a tick).
//! - `turn`: walkers sense and turn once a tick, then walk four fine steps
//!   straight: today's paths. Every number is today's in dish units.
//! - `dense`: `smell` with 100,000 walkers (about Jones's density).
//!
//! Any `BigParams` number can follow: `walkers= keep= substeps= scale=
//! crowd= sensor_offset= sensor_angle= (degrees) rotate_angle= (degrees)
//! scent_coarse=0|1 blob=0|1 piles= deposit= step= food_deposit= scent_keep=
//! fed_scent= pile_r=`.
//!
//! Prints the engine's measures (`BigMeasure`) every `report` seconds and
//! the simulation's cost a tick.

use std::time::Instant;

use slime_mould::mould::big::{BigDish, BigParams};
use slime_mould::mould::STEPS_PER_SECOND;

fn preset(name: &str) -> BigParams {
    let t = BigParams::today();
    let naive = BigParams { scale: 4, walkers: 32_000, scent_coarse: false, ..t.clone() };
    let steps = BigParams {
        substeps: 4,
        deposit: t.deposit / 4.0,
        keep: t.keep.powf(0.25),
        food_deposit: t.food_deposit / 4.0,
        scent_keep: t.scent_keep.powf(0.25),
        ..naive.clone()
    };
    let smell = BigParams { scent_coarse: true, food_deposit: t.food_deposit, scent_keep: t.scent_keep, ..steps.clone() };
    let reach = BigParams { sensor_offset: t.sensor_offset * 4.0, ..smell.clone() };
    match name {
        "today" => t,
        "naive" => naive,
        "steps" => steps,
        "smell" => smell,
        "spread" => BigParams { trail_passes: 4, ..reach.clone() },
        "turn" => BigParams { trail_passes: 4, sense_every: 4, ..reach },
        "reach" => reach.clone(),
        "dense" => BigParams { walkers: 100_000, ..smell },
        _ => panic!("unknown preset {name}: today, naive, steps, smell, reach, spread, turn, dense"),
    }
}

fn write_gif(path: &str, shots: Vec<image::RgbaImage>, ms: u32) {
    let delay = image::Delay::from_numer_denom_ms(ms, 1);
    let frames: Vec<image::Frame> = shots.into_iter().map(|img| image::Frame::from_parts(img, 0, 0, delay)).collect();
    let file = std::fs::File::create(path).expect("gif file");
    let mut enc = image::codecs::gif::GifEncoder::new_with_speed(file, 10);
    enc.set_repeat(image::codecs::gif::Repeat::Infinite).unwrap();
    enc.encode_frames(frames).expect("gif encode");
}

fn main() {
    let mut name = "smell".to_string();
    // Names the run in its output lines and files; the preset's name if not given.
    let mut tag: Option<String> = None;
    let mut seeds = vec![1u64];
    let (mut seconds, mut report, mut threads) = (60.0f32, 10.0f32, 1usize);
    let (mut gif, mut look): (Option<String>, Option<String>) = (None, None);
    let (mut every, mut down) = (12u64, 1usize);
    let mut at: Vec<f32> = Vec::new();
    let mut overrides: Vec<(String, String)> = Vec::new();
    for arg in std::env::args().skip(1) {
        let (k, v) = arg.split_once('=').unwrap_or_else(|| panic!("expected key=value, got {arg}"));
        match k {
            "preset" => name = v.into(),
            "tag" => tag = Some(v.into()),
            "seeds" => seeds = v.split(',').map(|s| s.parse().unwrap()).collect(),
            "seconds" => seconds = v.parse().unwrap(),
            "report" => report = v.parse().unwrap(),
            "threads" => threads = v.parse().unwrap(),
            "gif" => gif = Some(v.into()),
            "look" => look = Some(v.into()),
            "every" => every = v.parse().unwrap(),
            "down" => down = v.parse().unwrap(),
            "at" => at = v.split(',').map(|s| s.parse().unwrap()).collect(),
            _ => overrides.push((k.into(), v.into())),
        }
    }
    let mut p = preset(&name);
    let name = tag.unwrap_or(name);
    for (k, v) in &overrides {
        let f = || v.parse::<f32>().unwrap_or_else(|_| panic!("{k}={v}: not a number"));
        match k.as_str() {
            "walkers" => p.walkers = f() as usize,
            "keep" => p.keep = f(),
            "trail_passes" => p.trail_passes = f() as usize,
            "sense_every" => p.sense_every = f() as usize,
            "substeps" => p.substeps = f() as usize,
            "scale" => p.scale = f() as usize,
            "crowd" => p.crowd = f() as u8,
            "sensor_offset" => p.sensor_offset = f(),
            "sensor_angle" => p.sensor_angle = f().to_radians(),
            "rotate_angle" => p.rotate_angle = f().to_radians(),
            "scent_coarse" => p.scent_coarse = f() != 0.0,
            "blob" => p.blob = f() != 0.0,
            "piles" => p.piles = f() as usize,
            "deposit" => p.deposit = f(),
            "step" => p.step = f(),
            "food_deposit" => p.food_deposit = f(),
            "scent_keep" => p.scent_keep = f(),
            "fed_scent" => p.fed_scent = f(),
            "pile_r" => p.pile_r = f(),
            _ => panic!("unknown key {k}"),
        }
    }
    println!("bigdish: preset {name} seeds {seeds:?} seconds {seconds} threads {threads}");
    println!("params {p:?}");
    let ticks = (seconds * STEPS_PER_SECOND as f32).round() as u64;
    let report_every = ((report * STEPS_PER_SECOND as f32).round() as u64).max(1);
    for &seed in &seeds {
        let mut d = BigDish::new(BigParams { seed, ..p.clone() });
        d.threads = threads;
        println!("== seed {seed}: {} x {}, {} walkers, {} piles", d.w, d.h, d.walkers.len(), d.piles.len());
        let mut shots = Vec::new();
        let mut spent = 0.0f64;
        let mut stills: Vec<u64> = at.iter().map(|s| (s * STEPS_PER_SECOND as f32).round() as u64).collect();
        stills.sort_unstable();
        while d.ticks < ticks {
            let t0 = Instant::now();
            d.tick();
            spent += t0.elapsed().as_secs_f64();
            if gif.is_some() && d.ticks.is_multiple_of(every) {
                shots.push(d.image(down));
            }
            if let Some(dir) = &look {
                if stills.contains(&d.ticks) {
                    d.image(1).save(format!("{dir}/big-{name}-s{seed}-{:03}s.png", d.ticks / STEPS_PER_SECOND as u64)).unwrap();
                }
            }
            if d.ticks.is_multiple_of(report_every) || d.ticks == ticks {
                let m = d.measure();
                println!(
                    "seed {seed} {name} t {:5.1}s  tube {:7.1}  loops {:4}  width {:.2}  reach {:6.1}  piles {}/{}  strays {:.3}  ms/tick {:.2}",
                    m.seconds,
                    m.tube_area,
                    m.loops,
                    m.width,
                    m.reach,
                    m.reached,
                    d.piles.len(),
                    m.strays,
                    spent * 1e3 / d.ticks as f64
                );
            }
        }
        println!("seed {seed} {name} hash={:016x}", d.hash());
        if let Some(dir) = &gif {
            // Frames `every` ticks apart, played at 3x real time.
            let ms = (every as f32 * 1000.0 / STEPS_PER_SECOND as f32 / 3.0).round() as u32;
            write_gif(&format!("{dir}/big-{name}-s{seed}.gif"), shots, ms.max(20));
        }
    }
}
