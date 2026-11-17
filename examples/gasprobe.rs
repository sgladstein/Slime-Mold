//! **Gas ignites** (the simulation-use plan's C5, 2026-11-17;
//! `BigParams::gas_burn`): sleep gas laid on the body, then a small fire
//! lit at the cloud's edge. With `gas_burn` the fire flashes across the
//! cloud, damp body or not, and burns the sleepers; without it the damp
//! body puts the fire out.
//!
//! ```text
//! cargo run --release --example gasprobe -- [seeds=1-8 depth=2 grow=60 r=10 margin=25 watch=15 burns=0,0.2 scale=2 look=DIR]
//! ```
//!
//! For each seed: a generated planet (`levelgen::level`) under full-grown
//! meadow, the default slime (shape C, `fire_dry` on) on the CPU race
//! world, grown `grow` game seconds with the surges held. On a disc of
//! the body as `dryprobe`'s (`r`, crystals `margin` clear), the kit's sleep
//! gas (`Tool::Gas`: `gas_r` 12, 20 s, its cloud) is laid, and 1 s later a
//! fire of 2 dish cells is lit at the cloud's edge (`gas_r` - 2 east of the
//! centre, share 1), then watched `watch` seconds, once per `gas_burn`.
//!
//! **Deciding measure, named before the run:** walkers killed by fire in
//! `watch` seconds, `gas_burn` 0.2 (the kit's `gas_sleep`, so the cloud
//! burns where it sleeps) against 0, paired by seed. Also: the walkers
//! asleep at the light, the cloud summed and its cells over 0.2 at the
//! light and at the end, the living.
//!
//! `look=DIR` writes a still every third of a second of the first seed's
//! arms (`bG-NNN.png`), the cloud tinted violet.
use slime_mould::mould::big::{BigDish, BigParams, SURGE_WAIT};
use slime_mould::mould::larder_world::{race_params, shape_keys};
use slime_mould::mould::kit::KitRules;
use slime_mould::mould::rounds::burst_rules;
use slime_mould::mould::squad::{self, Burst};
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
        if !d.gas_cloud.is_empty() && d.gas_cloud[i] > 0.0 {
            let g = (d.gas_cloud[i] * 1.5).min(0.6);
            for (v, t) in col.iter_mut().zip([170.0, 110.0, 230.0]) {
                *v += (t - *v) * g;
            }
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
    let (mut seeds, mut depth, mut grow, mut r, mut margin, mut watch, mut scale) = ((1u64..=8).collect::<Vec<_>>(), 2u32, 60u64, 10.0f32, 25.0f32, 15u64, 2usize);
    let mut burns = vec![0.0f32, 0.2];
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
            "burns" => burns = v.split(',').map(|x| x.parse().unwrap()).collect(),
            "scale" => scale = v.parse().unwrap(),
            "walkers" => walkers = v.parse().unwrap(),
            "look" => look = Some(v.into()),
            _ => panic!("unknown {k} (seeds, depth, grow, r, margin, watch, burns, scale, walkers, look)"),
        }
    }
    let kit = KitRules::default();
    println!("gasprobe: planet depth {depth} under meadow, shape C, scale {scale}, {walkers} walkers; grow {grow} s, disc r {r}, margin {margin}, gas r {}, watch {watch} s, gas_burn {burns:?}", kit.gas_r);
    println!("seed gas_burn | centre (x, y) | asleep at the light | fire killed | cloud summed, cells over 0.2: at the light -> at the end | living");
    let mut rows: Vec<Vec<u64>> = Vec::new();
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
        let fine = TPS as f32 * d.p.substeps.max(1) as f32;
        let rules = burst_rules(&kit, fine);
        let gas = Burst { at: c, r: kit.gas_r, gas: (kit.gas_secs * fine) as u16, ..Burst::default() };
        let mut row = Vec::new();
        for &gb in &burns {
            let mut w = d.clone();
            w.p.gas_burn = gb;
            squad::burst(&mut w, &gas, &rules);
            for _ in 0..TPS {
                w.tick();
            }
            let asleep = (0..w.walkers.len()).filter(|&k| w.lives(k) && !w.gassed.is_empty() && w.gassed[k] > 0).count();
            let cloud = |w: &BigDish| (w.gas_cloud.iter().map(|&v| v as f64).sum::<f64>(), w.gas_cloud.iter().filter(|&&v| v > 0.2).count());
            let at = cloud(&w);
            let before = w.fire_killed;
            w.ignite(c.0 + kit.gas_r - 2.0, c.1, 2.0, 1.0);
            for q in 0..watch * TPS {
                if let Some(dir) = look.as_ref().filter(|_| seed == seeds[0]) {
                    if q.is_multiple_of(5) {
                        std::fs::create_dir_all(dir).expect("look dir");
                        still(&w, c, kit.gas_r + 8.0, &dir.join(format!("b{gb}-{:03}.png", q / 5)));
                    }
                }
                w.tick();
            }
            let killed = w.fire_killed - before;
            let end = cloud(&w);
            println!("{seed} {gb} | ({:.0}, {:.0}) | {asleep} | {killed} | {:.0}, {} -> {:.0}, {} | {}", c.0, c.1, at.0, at.1, end.0, end.1, living(&w));
            row.push(killed);
        }
        rows.push(row);
    }
    let base = burns.iter().position(|&a| a == 0.0);
    for (j, &gb) in burns.iter().enumerate() {
        let sum: u64 = rows.iter().map(|r| r[j]).sum();
        match base.filter(|&b| b != j) {
            Some(b) => {
                let more = rows.iter().filter(|r| r[j] > r[b]).count();
                let less = rows.iter().filter(|r| r[j] < r[b]).count();
                println!("gas_burn {gb}: fire killed {sum} in all; against 0, more {more}, less {less}, sign p {:.3}", sign_p(more, less));
            }
            None => println!("gas_burn {gb}: fire killed {sum} in all"),
        }
    }
}
