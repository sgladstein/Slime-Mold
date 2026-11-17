//! **Dry slime burns** (the simulation-use plan's C1, 2026-11-17;
//! `BigParams::fire_dry`). The owner's ruling (11-17 01:13Z): fed slime is
//! fireproof, starving slime burns. This probe measures it on a cut part.
//!
//! ```text
//! cargo run --release --example dryprobe -- [seeds=1-8 depth=2 grow=60 r=10 margin=25 gap=8 alone=1 grass=1 lit=15 burn=20 scale=2 walkers=25000 look=DIR]
//! ```
//!
//! For each seed: a generated planet (`levelgen::level`) under full-grown
//! meadow (`grass=1`; 0 keeps the planet's own patches), the default slime (shape C: `shape_keys("c")`, with its
//! wither and panic) on the CPU race world, grown `grow` game seconds. Then
//! a disc of `r` dish cells on the body is picked: the most walkers in a
//! disc, with no crystal within `r` + `margin`. From that state six arms run:
//!
//! - **cut**: a ring `gap` dish cells wide round the disc is struck once
//!   (walkers killed, trail cleared, no alarm) and, with `alone=1` (the
//!   default), every walker outside the disc is removed, so the part is
//!   cut off from food and can't rejoin; `fire_dry` on.
//! - **joined**: the same disc, not cut; `fire_dry` on.
//! - **cut, old rule** and **joined, old rule**: the same with `fire_dry`
//!   off (the trail wets all slime).
//! - **cut, fed** and **joined, fed**: the same with `fire_fed` on too (fed
//!   slime wet whatever its trail), added after the first look: on full
//!   meadow the old rule burnt 65% of the joined disc in 20 s (seed 1).
//!
//! Once half the disc's walkers (20 or more) have stale news, or at `lit`
//! seconds after the cut if they never do, every plant in the disc is lit
//! (`ignite`, share 1), at the same tick in every arm; `burn` seconds later
//! the probe counts what the fire killed (`fire_killed` over the world,
//! nearly all of it in the disc), per walker in the disc at the light.
//!
//! **The deciding measure, named before the first run:** fire kills per
//! walker in the disc at the light, cut against joined, paired by seed.
//! The guard (the owner's ruling): joined slime with `fire_dry` must burn no
//! more than with it off. The old-rule arms say how much of the burning is
//! the rule.
//!
//! Prints a line an arm a seed, and the paired counts with a two-sided sign
//! test. `look=DIR` writes a still of the disc every third of a second
//! from the light (`seedN-ARM-NNN.png`).
use slime_mould::mould::big::{BigDish, BigParams};
use slime_mould::mould::larder_world::{race_params, shape_keys};
use slime_mould::mould::{levelgen, plants, tiers};

const TPS: u64 = 15;

fn sign_p(more: usize, less: usize) -> f64 {
    // Two-sided exact binomial at one half, ties dropped.
    let n = more + less;
    if n == 0 {
        return 1.0;
    }
    let k = more.min(less);
    let mut c = 1.0f64;
    let mut tail = 0.0;
    for i in 0..=k {
        if i > 0 {
            c *= (n - i + 1) as f64 / i as f64;
        }
        tail += c;
    }
    (2.0 * tail / 2f64.powi(n as i32)).min(1.0)
}

fn living(d: &BigDish) -> usize {
    (0..d.walkers.len()).filter(|&k| d.lives(k) && d.life.get(k).is_none_or(|&l| l == slime_mould::mould::big::LIFE_WALKER)).count()
}

/// Walkers in the disc (dish cells) and how many of them have stale news.
fn in_disc(d: &BigDish, c: (f32, f32), r: f32) -> (usize, usize) {
    let s = d.p.scale as f32;
    let (mut n, mut stale) = (0, 0);
    for k in 0..d.walkers.len() {
        if !d.lives(k) || d.life.get(k).is_some_and(|&l| l != slime_mould::mould::big::LIFE_WALKER) {
            continue;
        }
        let (x, y, _) = d.walkers[k];
        if (x / s - c.0).hypot(y / s - c.1) <= r {
            n += 1;
            if d.news.get(k).is_some_and(|&v| d.steps > v + d.p.wither_lag as u64) {
                stale += 1;
            }
        }
    }
    (n, stale)
}

/// The disc to cut: the most walkers within `r` (dish cells), no crystal
/// within `r` + `margin`, its middle on a walker.
fn pick(d: &BigDish, r: f32, margin: f32) -> Option<(f32, f32)> {
    let s = d.p.scale as f32;
    let crystals: Vec<(f32, f32)> = (0..d.w * d.h).filter(|&i| !d.pile_of.is_empty() && d.pile_of[i] != 0).map(|i| ((i % d.w) as f32 / s, (i / d.w) as f32 / s)).collect();
    let live: Vec<(f32, f32)> = (0..d.walkers.len()).filter(|&k| d.lives(k)).map(|k| (d.walkers[k].0 / s, d.walkers[k].1 / s)).collect();
    let mut best = None;
    let mut most = 0;
    for &c in live.iter().step_by(37) {
        if crystals.iter().any(|&q| (q.0 - c.0).hypot(q.1 - c.1) <= r + margin) {
            continue;
        }
        let n = live.iter().filter(|&&q| (q.0 - c.0).hypot(q.1 - c.1) <= r).count();
        if n > most {
            (most, best) = (n, Some(c));
        }
    }
    best
}

/// A still of the box `half` dish cells round `c`, 4 px a fine cell:
/// ground and plants under the slime's trail (pale yellow, by its trail as
/// the game's sheet thickens), walkers whose news is stale ash grey, fire
/// orange over all.
fn still(d: &BigDish, c: (f32, f32), half: f32, path: &std::path::Path) {
    const PX: u32 = 4;
    let s = d.p.scale as f32;
    let (x0, y0) = (((c.0 - half) * s).max(0.0) as usize, ((c.1 - half) * s).max(0.0) as usize);
    let side = (2.0 * half * s) as usize;
    let (x1, y1) = ((x0 + side).min(d.w), (y0 + side).min(d.h));
    let (bw, bh) = (x1 - x0, y1 - y0);
    let mut stale = vec![false; d.w * d.h];
    for k in 0..d.walkers.len() {
        if d.lives(k) && d.news.get(k).is_some_and(|&v| d.steps > v + d.p.wither_lag as u64) {
            let (x, y) = (d.walkers[k].0 as usize, d.walkers[k].1 as usize);
            if x < d.w && y < d.h {
                stale[y * d.w + x] = true;
            }
        }
    }
    let img = image::RgbImage::from_fn(bw as u32 * PX, bh as u32 * PX, |px, py| {
        let (x, y) = (x0 + (px / PX) as usize, y0 + (py / PX) as usize);
        let i = y * d.w + x;
        let mut col = if !d.agar[i] { [40.0, 40.0, 44.0] } else { [95.0, 85.0, 70.0] };
        if !d.pile_of.is_empty() && d.pile_of[i] != 0 {
            col = [120.0, 220.0, 255.0];
        }
        if !d.plant.is_empty() {
            let pl = d.plant[i].min(1.0);
            for (v, t) in col.iter_mut().zip([90.0, 150.0, 60.0]) {
                *v += (t - *v) * pl;
            }
        }
        if !d.char.is_empty() {
            let k = 1.0 - 0.75 * d.char[i];
            col = [col[0] * k, col[1] * k, col[2] * k];
        }
        let tr = (d.trail[i] / 6.0).min(1.0);
        let slime = if stale[i] { [150.0, 145.0, 140.0] } else { [240.0, 215.0, 90.0] };
        for (v, t) in col.iter_mut().zip(slime) {
            *v += (t - *v) * tr * 0.85;
        }
        if !d.fire.is_empty() && d.fire[i] > 0.0 {
            let f = d.fire[i].min(1.0);
            for (v, t) in col.iter_mut().zip([255.0, 100.0, 20.0]) {
                *v += (t - *v) * f;
            }
        }
        image::Rgb(col.map(|v| v.clamp(0.0, 255.0) as u8))
    });
    img.save(path).expect("still");
}

fn main() {
    let (mut seeds, mut depth, mut grow, mut r, mut lit, mut burn, mut scale) = ((1u64..=8).collect::<Vec<_>>(), 2u32, 60u64, 10.0f32, 15u64, 20u64, 2usize);
    let (cw, keys) = shape_keys("c").expect("shape C");
    let mut walkers = cw;
    // A margin of 4 dish cells: the lone part reached a crystal within
    // seconds and never went stale (seed 1: 0 of 1,395 at 14 s).
    let (mut gap, mut alone, mut margin) = (8.0f32, true, 25.0f32);
    let mut look = None::<std::path::PathBuf>;
    let mut grass = true;
    for a in std::env::args().skip(1) {
        let (k, v) = a.split_once('=').expect("key=value");
        match k {
            "seeds" => {
                let (a, b) = v.split_once('-').unwrap_or((v, v));
                seeds = (a.parse().unwrap()..=b.parse().unwrap()).collect();
            }
            "depth" => depth = v.parse().unwrap(),
            "grow" => grow = v.parse().unwrap(),
            "r" => r = v.parse().unwrap(),
            "lit" => lit = v.parse().unwrap(),
            "burn" => burn = v.parse().unwrap(),
            "gap" => gap = v.parse().unwrap(),
            "alone" => alone = v == "1",
            "margin" => margin = v.parse().unwrap(),
            "scale" => scale = v.parse().unwrap(),
            "walkers" => walkers = v.parse().unwrap(),
            "look" => look = Some(v.into()),
            "grass" => grass = v == "1",
            _ => panic!("unknown {k} (seeds, depth, grow, r, margin, gap, alone, grass, lit, burn, scale, walkers, look)"),
        }
    }
    println!("dryprobe: planet depth {depth}, shape C, scale {scale}, {walkers} walkers; grow {grow} s, disc r {r}, margin {margin}, gap {gap}, alone {alone}, grass {grass}, lit {lit} s after the cut, counted {burn} s after");
    println!("seed arm | disc (x, y) | at the light (s after the cut): walkers, stale | fire killed, per walker | living");
    // (cut, joined; each with the old rule; each fed-fireproof) kills per walker, by seed
    let mut rows: Vec<[f64; 6]> = Vec::new();
    for &seed in &seeds {
        let mut m = levelgen::level(seed, depth);
        // On grass (the plan's measure): every cell meadow, full grown. The
        // planet's own plants are patches; in them a lit disc on seed 1
        // burnt two small patches out within 2 s either way.
        if grass {
            m.plants = vec![plants::pack(0, 1.0); m.ground.len()];
        } else {
            plants::stand_in(&mut m, seed as u32, 1.0);
        }
        let mut p: BigParams = race_params(seed, scale, walkers);
        for &(k, v) in keys {
            match k {
                "upkeep" => p.upkeep = v.parse().unwrap(),
                "capacity" => p.capacity = v.parse().unwrap(),
                _ => tiers::set_key(&mut p, k, v).unwrap_or_else(|e| panic!("{e}")),
            }
        }
        p.fire_dry = true;
        let mut d = BigDish::with_map(p, &m);
        for _ in 0..grow * TPS {
            d.tick();
        }
        if std::env::var("DRYPROBE_DEBUG").is_ok() {
            let cr = (0..d.pile_of.len()).filter(|&i| d.pile_of[i] != 0).count();
            let ch = d.charge.iter().filter(|&&v| v > 0.0).count();
            let took = d.took.iter().filter(|&&v| v > 0.0).count();
            println!("debug: pile_of {} ({cr} crystal), charge {} ({ch} > 0), took {took} walkers, piles {}", d.pile_of.len(), d.charge.len(), d.piles.len());
        }
        let Some(c) = pick(&d, r, margin) else {
            println!("{seed} - | no disc clear of crystals");
            continue;
        };
        let mut row = [0.0; 6];
        // The ring: strikes `gap` dish cells across every dish cell round a
        // circle half that past the disc's edge, once. Held open every tick
        // instead (as soldiers firing into the gap would), the part
        // panicked out through it and died there: no walker left in the
        // disc by 10 s on seed 1. Struck once and lit at a fixed 10 s, the
        // body had closed the gap and none was stale (0 of 1,116).
        // A ring 3 dish cells wide, struck once: the body closed it and
        // nothing went stale by 15 s either (seed 1: 0 of 1,154), so the
        // ring is `gap` wide.
        let rr = r + 0.5 * gap;
        let ring = |w: &mut BigDish| {
            let steps = (std::f32::consts::TAU * rr).ceil() as usize;
            for q in 0..steps {
                let t = q as f32 / steps as f32 * std::f32::consts::TAU;
                w.strike(c.0 + rr * t.cos(), c.1 + rr * t.sin(), 0.5 * gap, 0.0, 0.0);
            }
        };
        // The tick the cut arm lit at; the others light at the same.
        let mut light_at = None::<u64>;
        for (a, (name, cut, dry, fed)) in [("cut", true, true, false), ("joined", false, true, false), ("cut-old", true, false, false), ("joined-old", false, false, false), ("cut-fed", true, true, true), ("joined-fed", false, true, true)].into_iter().enumerate() {
            let mut w = d.clone();
            (w.p.fire_dry, w.p.fire_fed) = (dry, fed);
            if cut {
                ring(&mut w);
                // `alone`: the rest of the mould removed too, so the part
                // can't rejoin: with only the ring (even 8 dish cells wide)
                // the oldest news anywhere 15 s after the cut was 83 steps,
                // under the lag of 150 (seed 1, grown 30 s).
                if alone {
                    let s = w.p.scale as f32;
                    for k in 0..w.walkers.len() {
                        let (x, y, _) = w.walkers[k];
                        if w.lives(k) && w.life.get(k).is_none_or(|&l| l == slime_mould::mould::big::LIFE_WALKER) && (x / s - c.0).hypot(y / s - c.1) > r {
                            w.kill(k);
                        }
                    }
                }
            }
            let mut t = 0u64;
            loop {
                let (n, stale) = in_disc(&w, c, r);
                let due = match light_at {
                    Some(at) => t >= at,
                    None => (n >= 20 && stale * 2 >= n) || t >= lit * TPS,
                };
                if due {
                    break;
                }
                if std::env::var("DRYPROBE_DEBUG").is_ok() && t.is_multiple_of(TPS) {
                    let all = (0..w.walkers.len()).filter(|&k| w.lives(k)).count();
                    let old = (0..w.walkers.len()).filter(|&k| w.lives(k) && w.news.get(k).is_some_and(|&v| w.steps > v + w.p.wither_lag as u64)).count();
                    println!("debug {name} {} s: disc {n}, {stale} stale; world {all}, {old} stale", t / TPS);
                }
                w.tick();
                t += 1;
            }
            let at = *light_at.get_or_insert(t);
            if std::env::var("DRYPROBE_DEBUG").is_ok() {
                let ages: Vec<u64> = (0..w.walkers.len()).filter(|&k| w.lives(k)).map(|k| w.steps.saturating_sub(w.news.get(k).copied().unwrap_or(0))).collect();
                let old = ages.iter().filter(|&&a| a > w.p.wither_lag as u64).count();
                println!("debug: steps {} lag {} wither {} news {} heard {} energy {}; {old} of {} walkers stale, oldest {}", w.steps, w.p.wither_lag, w.p.wither, w.news.len(), w.heard.len(), w.energy.len(), ages.len(), ages.iter().max().unwrap_or(&0));
            }
            let (n, stale) = in_disc(&w, c, r);
            let before = w.fire_killed;
            w.ignite(c.0, c.1, r, 1.0);
            for q in 0..burn * TPS {
                if let Some(dir) = &look {
                    if q.is_multiple_of(5) {
                        std::fs::create_dir_all(dir).expect("look dir");
                        still(&w, c, r + gap + 4.0, &dir.join(format!("seed{seed}-{name}-{:03}.png", q / 5)));
                    }
                }
                w.tick();
            }
            let killed = w.fire_killed - before;
            let per = killed as f64 / n.max(1) as f64;
            row[a] = per;
            println!("{seed} {name} | ({:.0}, {:.0}) | {:.1} s: {n}, {stale} | {killed}, {per:.3} | {}", c.0, c.1, at as f32 / TPS as f32, living(&w));
        }
        rows.push(row);
    }
    let pair = |a: usize, b: usize| {
        let more = rows.iter().filter(|r| r[a] > r[b]).count();
        let less = rows.iter().filter(|r| r[a] < r[b]).count();
        (more, less, sign_p(more, less))
    };
    let mean = |a: usize| rows.iter().map(|r| r[a]).sum::<f64>() / rows.len().max(1) as f64;
    println!("mean kills a walker: cut {:.3}, joined {:.3}; with the old rule cut {:.3}, joined {:.3}; fed fireproof too cut {:.3}, joined {:.3}", mean(0), mean(1), mean(2), mean(3), mean(4), mean(5));
    let (m, l, p) = pair(0, 1);
    println!("cut over joined: {m} more, {l} less, sign p {p:.3}");
    let (m, l, p) = pair(0, 2);
    println!("cut, dry rule over old rule: {m} more, {l} less, sign p {p:.3}");
    let (m, l, p) = pair(1, 3);
    println!("joined, dry rule over old rule (the guard): {m} more, {l} less, sign p {p:.3}");
    let (m, l, p) = pair(4, 5);
    println!("fed fireproof too, cut over joined: {m} more, {l} less, sign p {p:.3}");
    let (m, l, p) = pair(5, 1);
    println!("joined, fed fireproof over dry rule alone: {m} more, {l} less, sign p {p:.3}");
}
