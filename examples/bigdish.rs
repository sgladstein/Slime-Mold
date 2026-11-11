//! The engine at any scale (`slime_mould::mould::big`): the GPU plan's
//! phase 2 offline dish, and phase 1's scaling-rule arms.
//!
//! ```text
//! cargo build --release --example bigdish
//! ./target/release/examples/bigdish preset=nose seeds=1 seconds=60 threads=4
//! ./target/release/examples/bigdish preset=dense gif=out/ every=12 down=2
//! ./target/release/examples/bigdish preset=dense frames=out/ every=24   # PNGs for ffmpeg
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
//! - `nose`: `naive`, but scent smelled 36 fine cells out (today's 9 dish
//!   cells) while trail is sensed 9 fine cells out: the census found the
//!   naive port's fine mesh and today's pace come from different reaches.
//! - `dense`: `nose` with 100,000 walkers (about Jones's density), the
//!   phase 2 dish.
//!
//! Any `BigParams` number can follow: `walkers= keep= substeps= scale=
//! crowd= sensor_offset= sensor_angle= (degrees) rotate_angle= (degrees)
//! scent_coarse=0|1 blob=0|1 piles= deposit= step= food_deposit= scent_keep=
//! fed_scent= pile_r= respawn=`.
//!
//! Prints the engine's measures (`BigMeasure`) every `report` seconds and
//! the simulation's cost a tick.
//!
//! `style=` sets how frames are drawn (2026-11-10, comparing with Jenson's
//! clips): `amber` (default) is `BigDish::image`, the game's look; `grey`
//! is the same trail grey on black with brightness 1 - exp(-trail/`knee`)
//! (default knee 6, the game's); `count` draws walkers a cell the same way,
//! as Bleuje's port of Jenson's later work draws particle counts. A list
//! (`style=amber,grey`) saves each style's still; frames and GIFs use the
//! first. `raw` in the list also saves the trail and walkers a cell as raw
//! f32 (`.f32`, w x h each) for drawing offline.
//!
//! Added 2026-11-11 for the looks the comparison could not reproduce
//! (`Reports/jenson-looks-2026-11-11.md`): `style=soft` draws the trail
//! over its 99.9th percentile to the power `gamma` (default 0.6), the
//! comparison's soft curve; `style=sick` is `soft` with the germ field in
//! red, full at four times `sick_catch`. New keys: `blob_r= haze= sense_scale= deposit_cap= sense_floor= wander= sick_steps=
//! immune_steps= sick_catch= sick_deposit= sick_keep= sick_speed=
//! sick_trail= sick_reach= sick_turn= sick_seed_r=`, and `offset_mod= angle_mod= turn_mod=
//! step_mod=` as `amp:exp` (angles in degrees). Events: `infect=T` makes
//! walkers within `infect_r` (default 10) dish cells of the centre sick at
//! T seconds; `cut=T` clears trail and germ in a band `cut_w` (default 2)
//! dish cells wide at dish x `cut_x` (default 160) at T seconds, as the
//! knife does. With contagion on, the report adds the share sick, the share
//! ever sick, the sick front (90th percentile distance of sick walkers from
//! the centre, dish cells) and the share of walkers beyond `cut_x` ever
//! sick.

use std::time::Instant;

use slime_mould::mould::big::{BigDish, BigParams};
use slime_mould::mould::{H, STEPS_PER_SECOND, W};

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
        "naive" => naive.clone(),
        "nose" => BigParams { scent_offset: t.sensor_offset * 4.0, ..naive },
        "steps" => steps,
        "smell" => smell,
        "spread" => BigParams { trail_passes: 4, ..reach.clone() },
        "turn" => BigParams { trail_passes: 4, sense_every: 4, ..reach },
        "reach" => reach.clone(),
        "dense" => BigParams { walkers: 100_000, scent_offset: t.sensor_offset * 4.0, ..naive },
        _ => panic!("unknown preset {name}: today, naive, steps, smell, reach, spread, turn, nose, dense"),
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

/// A frame in `style` (see the header): the brightest fine cell of each
/// `down` x `down` block, with a smooth knee and no edge drawn.
fn picture(d: &BigDish, style: &str, knee: f32, down: usize) -> image::RgbaImage {
    if style == "amber" {
        return d.image(down);
    }
    let field: Vec<f32> = match style {
        "grey" => d.trail.clone(),
        "count" => {
            let mut c = vec![0.0f32; d.w * d.h];
            for &(x, y, _) in &d.walkers {
                c[y as usize * d.w + x as usize] += 1.0;
            }
            c
        }
        "soft" | "sick" => d.trail.clone(),
        _ => panic!("style={style}: amber, grey, count, soft or sick"),
    };
    let soft = style == "soft" || style == "sick";
    // The soft curve's reference: the trail's 99.9th percentile over the agar.
    let refv = if soft { pct(&field, 0.999) } else { 1.0 };
    // Germ on a fixed scale: full red at four times the catch level.
    let germ_ref = 4.0 * d.p.sick_catch.max(1e-6);
    let down = down.max(1);
    let (iw, ih) = ((d.w / down) as u32, (d.h / down) as u32);
    let mut img = image::RgbaImage::new(iw, ih);
    for py in 0..ih {
        for px in 0..iw {
            let mut v = 0.0f32;
            for dy in 0..down {
                for dx in 0..down {
                    v = v.max(field[(py as usize * down + dy) * d.w + px as usize * down + dx]);
                }
            }
            if soft {
                let g = (v / refv).clamp(0.0, 1.0).powf(knee);
                let mut germ = 0.0f32;
                if style == "sick" && !d.germ.is_empty() {
                    for dy in 0..down {
                        for dx in 0..down {
                            germ = germ.max(d.germ[(py as usize * down + dy) * d.w + px as usize * down + dx]);
                        }
                    }
                    germ = (germ / germ_ref).clamp(0.0, 1.0).powf(0.5);
                }
                let r = (255.0 * g.max(germ)) as u8;
                let gb = (255.0 * g * (1.0 - 0.8 * germ)) as u8;
                img.put_pixel(px, py, image::Rgba([r, gb, gb, 255]));
                continue;
            }
            let g = (255.0 * (1.0 - (-v / knee).exp())).clamp(0.0, 255.0) as u8;
            img.put_pixel(px, py, image::Rgba([g, g, g, 255]));
        }
    }
    img
}

/// The `q` quantile of the positive values of `v` (1 if none).
fn pct(v: &[f32], q: f32) -> f32 {
    let mut pos: Vec<f32> = v.iter().copied().filter(|&x| x > 0.0).collect();
    if pos.is_empty() {
        return 1.0;
    }
    let k = ((pos.len() - 1) as f32 * q) as usize;
    *pos.select_nth_unstable_by(k, |a, b| a.total_cmp(b)).1
}

fn main() {
    let mut name = "nose".to_string();
    // Names the run in its output lines and files; the preset's name if not given.
    let mut tag: Option<String> = None;
    let mut seeds = vec![1u64];
    let (mut seconds, mut report, mut threads) = (60.0f32, 10.0f32, 1usize);
    let (mut gif, mut look): (Option<String>, Option<String>) = (None, None);
    // Full-size PNG frames every `every` ticks, for a video encoder.
    let mut frames: Option<String> = None;
    let (mut every, mut down) = (12u64, 1usize);
    let (mut style, mut knee) = ("amber".to_string(), 6.0f32);
    let mut at: Vec<f32> = Vec::new();
    let (mut infect, mut infect_r, mut cut, mut cut_x, mut cut_w) = (-1.0f32, 10.0f32, -1.0f32, 160.0f32, 2.0f32);
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
            "frames" => frames = Some(v.into()),
            "look" => look = Some(v.into()),
            "every" => every = v.parse().unwrap(),
            "down" => down = v.parse().unwrap(),
            "style" => style = v.into(),
            "knee" => knee = v.parse().unwrap(),
            "gamma" => knee = v.parse().unwrap(),
            "infect" => infect = v.parse().unwrap(),
            "infect_r" => infect_r = v.parse().unwrap(),
            "cut" => cut = v.parse().unwrap(),
            "cut_x" => cut_x = v.parse().unwrap(),
            "cut_w" => cut_w = v.parse().unwrap(),
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
            "scent_offset" => p.scent_offset = f(),
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
            "respawn" => p.respawn = f(),
            "blob_r" => p.blob_r = f(),
            "haze" => p.haze = f(),
            "sense_scale" => p.sense_scale = f(),
            "offset_mod" | "angle_mod" | "turn_mod" | "step_mod" => {
                let (a, e) = v.split_once(':').unwrap_or_else(|| panic!("{k}=amp:exp"));
                let (mut a, e): (f32, f32) = (a.parse().unwrap(), e.parse().unwrap());
                if k != "offset_mod" && k != "step_mod" {
                    a = a.to_radians();
                }
                let m = [a, e];
                match k.as_str() {
                    "offset_mod" => p.offset_mod = m,
                    "angle_mod" => p.angle_mod = m,
                    "turn_mod" => p.turn_mod = m,
                    _ => p.step_mod = m,
                }
            }
            "deposit_cap" => p.deposit_cap = f() as u16,
            "sense_floor" => p.sense_floor = f(),
            "wander" => p.wander = f(),
            "sick_steps" => p.sick_steps = f() as u16,
            "immune_steps" => p.immune_steps = f() as u16,
            "sick_catch" => p.sick_catch = f(),
            "sick_deposit" => p.sick_deposit = f(),
            "sick_keep" => p.sick_keep = f(),
            "sick_speed" => p.sick_speed = f(),
            "sick_trail" => p.sick_trail = f(),
            "sick_reach" => p.sick_reach = f(),
            "sick_turn" => p.sick_turn = f(),
            "sick_seed_r" => p.sick_seed_r = f(),
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
        let to_tick = |t: f32| if t < 0.0 { u64::MAX } else { (t * STEPS_PER_SECOND as f32).round() as u64 };
        let (infect_tick, cut_tick) = (to_tick(infect), to_tick(cut));
        let mut ever = vec![false; d.walkers.len()];
        let mut stills: Vec<u64> = at.iter().map(|s| (s * STEPS_PER_SECOND as f32).round() as u64).collect();
        stills.sort_unstable();
        while d.ticks < ticks {
            let t0 = Instant::now();
            d.tick();
            spent += t0.elapsed().as_secs_f64();
            if d.ticks == infect_tick {
                d.infect(W as f32 / 2.0, H as f32 / 2.0, infect_r);
            }
            if d.ticks == cut_tick {
                // The knife: clear trail and germ in a band across the dish.
                let s = d.p.scale as f32;
                let (x0, x1) = (((cut_x - cut_w / 2.0) * s) as usize, ((cut_x + cut_w / 2.0) * s) as usize);
                for y in 0..d.h {
                    for x in x0..x1.min(d.w) {
                        d.trail[y * d.w + x] = 0.0;
                        if !d.germ.is_empty() {
                            d.germ[y * d.w + x] = 0.0;
                        }
                    }
                }
            }
            for (e, &c) in ever.iter_mut().zip(&d.sick) {
                *e |= c > d.p.immune_steps;
            }
            if gif.is_some() && d.ticks.is_multiple_of(every) {
                shots.push(picture(&d, style.split(',').next().unwrap(), knee, down));
            }
            if let Some(dir) = &frames {
                if d.ticks.is_multiple_of(every) {
                    picture(&d, style.split(',').next().unwrap(), knee, 1).save(format!("{dir}/{name}-s{seed}-{:04}.png", d.ticks / every)).unwrap();
                }
            }
            if let Some(dir) = &look {
                if stills.contains(&d.ticks) {
                    // One still a style when several are listed (style=amber,grey).
                    let secs = d.ticks / STEPS_PER_SECOND as u64;
                    for st in style.split(',') {
                        if st == "raw" {
                            // The trail and walkers a cell as f32 little-endian, w x h each,
                            // for drawing offline with other curves.
                            let mut bytes = Vec::with_capacity(8 * d.w * d.h);
                            let mut count = vec![0.0f32; d.w * d.h];
                            for &(x, y, _) in &d.walkers {
                                count[y as usize * d.w + x as usize] += 1.0;
                            }
                            for v in d.trail.iter().chain(&count) {
                                bytes.extend_from_slice(&v.to_le_bytes());
                            }
                            std::fs::write(format!("{dir}/big-{name}-s{seed}-{secs:03}s.f32"), bytes).unwrap();
                            continue;
                        }
                        let file = if style.contains(',') {
                            format!("{dir}/big-{name}-s{seed}-{secs:03}s-{st}.png")
                        } else {
                            format!("{dir}/big-{name}-s{seed}-{secs:03}s.png")
                        };
                        picture(&d, st, knee, 1).save(file).unwrap();
                    }
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
                if !d.sick.is_empty() {
                    let s = d.p.scale as f32;
                    let (cx, cy) = (W as f32 / 2.0, H as f32 / 2.0);
                    let mut dist: Vec<f32> = d
                        .walkers
                        .iter()
                        .zip(&d.sick)
                        .filter(|(_, &c)| c > d.p.immune_steps)
                        .map(|(&(x, y, _), _)| (x / s - cx).hypot(y / s - cy))
                        .collect();
                    let n = d.walkers.len() as f32;
                    let front = if dist.is_empty() {
                        0.0
                    } else {
                        let k = ((dist.len() - 1) as f32 * 0.9) as usize;
                        *dist.select_nth_unstable_by(k, |a, b| a.total_cmp(b)).1
                    };
                    let beyond: Vec<bool> = d.walkers.iter().zip(&ever).filter(|(w, _)| w.0 / s > cut_x + cut_w / 2.0).map(|(_, &e)| e).collect();
                    let far = beyond.iter().filter(|&&e| e).count() as f32 / beyond.len().max(1) as f32;
                    println!(
                        "seed {seed} {name} t {:5.1}s  sick {:.3}  ever {:.3}  front {:6.1}  far {:.3}",
                        m.seconds,
                        dist.len() as f32 / n,
                        ever.iter().filter(|&&e| e).count() as f32 / n,
                        front,
                        far
                    );
                }
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
