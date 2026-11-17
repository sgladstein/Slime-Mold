//! **Fire scares it, a little** (the simulation-use plan's C4, 2026-11-17;
//! `BigParams::fire_alarm`): burning cells lay alarm, which shape C's
//! flinch reads as ground to back off. This probe measures what that does
//! to a fire lit on the fed body.
//!
//! ```text
//! cargo run --release --example fearprobe -- [seeds=1-8 depth=2 grow=60 r=10 margin=25 watch=30 alarms=0,5,50,200 scale=2 look=DIR]
//! ```
//!
//! For each seed: a generated planet (`levelgen::level`) under full-grown
//! meadow, the default slime (shape C, its flinch at keep 0.9, `fire_dry`
//! on) on the CPU race world, grown `grow` game seconds with the surges
//! held (`surge_at` = `SURGE_WAIT`). A disc of `r` dish cells on the body
//! with no crystal within `r` + `margin` (as `dryprobe`'s) is lit (every
//! plant in it, share 1) and followed `watch` seconds, once per
//! `fire_alarm` in `alarms`. The fire runs on across the meadow round the
//! body; the slime's trail keeps it off the fed tubes.
//!
//! **Deciding measure, named before the run:** fire kills per walker in
//! the disc at the light, `fire_alarm` 5 against 0, paired by seed. Also:
//! the walkers within 2 `r` of the disc's centre, how far their mean moved
//! from the centre by the end (the body stepping back), and the living.
//!
//! `look=DIR` writes a still every third of a second of the first seed's
//! arms (`aF-NNN.png`).
use slime_mould::mould::big::{BigDish, BigParams, SURGE_WAIT};
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

/// The walkers within `r` dish cells of `c` and their mean's distance
/// from `c` (dish cells).
fn round(d: &BigDish, c: (f32, f32), r: f32) -> (usize, f32) {
    let s = d.p.scale as f32;
    let (mut n, mut sx, mut sy) = (0usize, 0.0f32, 0.0f32);
    for k in 0..d.walkers.len() {
        if !d.lives(k) || d.life.get(k).is_some_and(|&l| l != slime_mould::mould::big::LIFE_WALKER) {
            continue;
        }
        let (x, y) = (d.walkers[k].0 / s, d.walkers[k].1 / s);
        if (x - c.0).hypot(y - c.1) <= r {
            (n, sx, sy) = (n + 1, sx + x, sy + y);
        }
    }
    if n == 0 {
        return (0, 0.0);
    }
    (n, (sx / n as f32 - c.0).hypot(sy / n as f32 - c.1))
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
    let (mut seeds, mut depth, mut grow, mut r, mut margin, mut watch, mut scale) = ((1u64..=8).collect::<Vec<_>>(), 2u32, 60u64, 10.0f32, 25.0f32, 30u64, 2usize);
    let mut alarms = vec![0.0f32, 5.0, 50.0, 200.0];
    let (cw, keys) = shape_keys("c").expect("shape C");
    let mut walkers = cw;
    let mut look = None::<std::path::PathBuf>;
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
            "margin" => margin = v.parse().unwrap(),
            "watch" => watch = v.parse().unwrap(),
            "alarms" => alarms = v.split(',').map(|x| x.parse().unwrap()).collect(),
            "scale" => scale = v.parse().unwrap(),
            "walkers" => walkers = v.parse().unwrap(),
            "look" => look = Some(v.into()),
            _ => panic!("unknown {k} (seeds, depth, grow, r, margin, watch, alarms, scale, walkers, look)"),
        }
    }
    println!("fearprobe: planet depth {depth} under meadow, shape C, scale {scale}, {walkers} walkers; grow {grow} s, disc r {r}, margin {margin}, watch {watch} s, fire_alarm {alarms:?}");
    println!("seed fire_alarm | disc (x, y), walkers at the light | fire killed, per walker | within 2r: walkers, mean's offset at the light -> at the end | living");
    // Kills a walker, by seed and arm.
    let mut rows: Vec<Vec<f64>> = Vec::new();
    for &seed in &seeds {
        let mut m = levelgen::level(seed, depth);
        m.plants = vec![plants::pack(0, 1.0); m.ground.len()];
        let mut p: BigParams = race_params(seed, scale, walkers);
        for &(k, v) in keys {
            match k {
                "upkeep" => p.upkeep = v.parse().unwrap(),
                "capacity" => p.capacity = v.parse().unwrap(),
                _ => tiers::set_key(&mut p, k, v).unwrap_or_else(|e| panic!("{e}")),
            }
        }
        p.fire_dry = true;
        p.surge_at = SURGE_WAIT;
        let mut d = BigDish::with_map(p, &m);
        d.threads = 4;
        for _ in 0..grow * TPS {
            d.tick();
        }
        let Some(c) = pick(&d, r, margin) else {
            println!("{seed} - | no disc clear of crystals");
            continue;
        };
        let (n, _) = round(&d, c, r);
        let mut row = Vec::new();
        for &fa in &alarms {
            let mut w = d.clone();
            w.p.fire_alarm = fa;
            let (_, at) = round(&w, c, 2.0 * r);
            let before = w.fire_killed;
            w.ignite(c.0, c.1, r, 1.0);
            for q in 0..watch * TPS {
                if let Some(dir) = look.as_ref().filter(|_| seed == seeds[0]) {
                    if q.is_multiple_of(5) {
                        std::fs::create_dir_all(dir).expect("look dir");
                        still(&w, c, 2.0 * r + 6.0, &dir.join(format!("a{fa}-{:03}.png", q / 5)));
                    }
                }
                w.tick();
            }
            let killed = w.fire_killed - before;
            let per = killed as f64 / n.max(1) as f64;
            let (near, end) = round(&w, c, 2.0 * r);
            println!("{seed} {fa} | ({:.0}, {:.0}), {n} | {killed}, {per:.3} | {near}, {at:.2} -> {end:.2} | {}", c.0, c.1, living(&w));
            row.push(per);
        }
        rows.push(row);
    }
    let base = alarms.iter().position(|&a| a == 0.0);
    for (j, &fa) in alarms.iter().enumerate() {
        let mean = rows.iter().map(|r| r[j]).sum::<f64>() / rows.len().max(1) as f64;
        match base.filter(|&b| b != j) {
            Some(b) => {
                let more = rows.iter().filter(|r| r[j] > r[b]).count();
                let less = rows.iter().filter(|r| r[j] < r[b]).count();
                println!("fire_alarm {fa}: kills a walker {mean:.3}; against 0, more {more}, less {less}, sign p {:.3}", sign_p(more, less));
            }
            None => println!("fire_alarm {fa}: kills a walker {mean:.3}"),
        }
    }
}
