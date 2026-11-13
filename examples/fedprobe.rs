//! **Does holding crystals make the mould stronger?** (owner, 2026-11-12
//! 22:22Z, after playing shape C: "what is going on with the slime is
//! unclear to the player ... I cannot tell if it is getting stronger by
//! holding the crystals. Right now it is mostly moving around and I kill
//! it.")
//!
//! ```text
//! cargo run --release --example fedprobe -- [seeds=1-4 eat=default,0 crowd=0,1 full=2 flow=0 until=300 threads=4 look=DIR]
//! ```
//!
//! Shape C on the CPU race world (scale 2: 25,000 walkers, upkeep 0.0002,
//! capacity 50,000), an idle squad, with `BigParams::crystal_eat` at each
//! of `eat=` (`default`, or a number; 0: crystals feed nothing),
//! `BigParams::fed_crowd` at each of `crowd=` (a walker holding `full=`
//! energy or more packs that many more to a cell) and `energy_flow` at
//! `flow=`. `look=DIR` saves the trail at `until` (log scale, crystals
//! ringed) as `DIR/s{seed}-eat{e}-crowd{c}.png`.
//!
//! The first arm tried was a fed walker laying more trail (`fed_trail`,
//! removed; `BigParams::fed_crowd` has its numbers).
//!
//! **Measures, named before the run:** walkers every 30 s (**deciding:**
//! at `until`, paired by seed against the first arm: if crystals that feed
//! nothing leave the mould about as big, holding them does not make it
//! stronger, whatever the screen shows); crystals reached; the share of
//! living walkers within twice a crystal's radius of one, and their mean
//! energy against the rest's (where the food sits). At the end: walkers
//! over open cells, and the share boxed in (no free neighbour, so the
//! one-a-cell rule lets them neither step nor breed).

use std::sync::mpsc;

use slime_mould::mould::big::BigDish;
use slime_mould::mould::larder_world::{race_params, Game, Rules, World};

struct Row {
    seed: u64,
    arm: usize,

    /// Mean trail on cells within twice a crystal's radius of one, and on
    /// the rest of the cells holding walkers.
    trail: (f32, f32),
    /// At the end: living walkers over open cells, and the share of living
    /// walkers whose eight neighbouring cells are all full or not agar (the
    /// one-a-cell rule lets them neither step nor breed).
    packed: (f32, f32),
    series: Vec<(f32, usize, usize, f32, f32, f32)>,
}

fn sample(g: &Game<BigDish>, start: &[f32]) -> (usize, usize, f32, f32, f32) {
    let d = &g.world;
    let near: Vec<((f32, f32), f32)> = d.crystals().iter().enumerate().map(|(c, x)| (x.0, 2.0 * d.crystal_r(c))).collect();
    let reached = d.crystals().iter().zip(start).filter(|(c, &s)| c.1 < s * 0.999).count();
    let (mut n_on, mut e_on, mut n_off, mut e_off) = (0usize, 0f64, 0usize, 0f64);
    for k in 0..d.walkers.len() {
        if !d.lives(k) {
            continue;
        }
        let (x, y, _) = d.walkers[k];
        let e = d.energy.get(k).copied().unwrap_or(0.0) as f64;
        if near.iter().any(|&((cx, cy), r)| (x - cx).hypot(y - cy) <= r) {
            n_on += 1;
            e_on += e;
        } else {
            n_off += 1;
            e_off += e;
        }
    }
    let all = (n_on + n_off).max(1);
    (g.mould, reached, n_on as f32 / all as f32, (e_on / n_on.max(1) as f64) as f32, (e_off / n_off.max(1) as f64) as f32)
}

fn image(d: &BigDish) -> image::RgbaImage {
    let (w, h) = (d.w, d.h);
    let mut img = image::RgbaImage::new(w as u32, h as u32);
    let top = d.trail.iter().copied().fold(0.0f32, f32::max).max(1e-6);
    for i in 0..w * h {
        let v = ((1.0 + d.trail[i]).ln() / (1.0 + top).ln()).clamp(0.0, 1.0);
        let c = if !d.agar[i] { [40u8, 40, 48] } else { [(30.0 + 225.0 * v) as u8, (25.0 + 170.0 * v) as u8, (20.0 + 60.0 * v) as u8] };
        img.put_pixel((i % w) as u32, (i / w) as u32, image::Rgba([c[0], c[1], c[2], 255]));
    }
    for (c, x) in d.crystals().iter().enumerate() {
        let (cx, cy, r) = (x.0 .0, x.0 .1, d.crystal_r(c));
        for a in 0..96 {
            let th = a as f32 / 96.0 * std::f32::consts::TAU;
            let (px, py) = (cx + r * th.cos(), cy + r * th.sin());
            if px >= 0.0 && py >= 0.0 && (px as usize) < w && (py as usize) < h {
                img.put_pixel(px as u32, py as u32, image::Rgba([80, 220, 255, 255]));
            }
        }
    }
    img
}

fn packed(d: &BigDish) -> (f32, f32) {
    let occ = d.occupancy();
    let open = d.agar.iter().filter(|&&a| a).count().max(1);
    let (mut live, mut boxed) = (0usize, 0usize);
    for k in 0..d.walkers.len() {
        if !d.lives(k) {
            continue;
        }
        live += 1;
        let (x, y) = (d.walkers[k].0 as i64, d.walkers[k].1 as i64);
        let free = (-1..=1i64).flat_map(|dy| (-1..=1i64).map(move |dx| (dx, dy))).filter(|&(dx, dy)| (dx, dy) != (0, 0)).any(|(dx, dy)| {
            let (nx, ny) = (x + dx, y + dy);
            if nx < 0 || ny < 0 || nx >= d.w as i64 || ny >= d.h as i64 {
                return false;
            }
            let j = ny as usize * d.w + nx as usize;
            d.agar[j] && occ[j] == 0
        });
        if !free {
            boxed += 1;
        }
    }
    (live as f32 / open as f32, boxed as f32 / live.max(1) as f32)
}

fn trail_split(d: &BigDish) -> (f32, f32) {
    let near: Vec<((f32, f32), f32)> = d.crystals().iter().enumerate().map(|(c, x)| (x.0, 2.0 * d.crystal_r(c))).collect();
    let occ = d.occupancy();
    let (mut a, mut na, mut b, mut nb) = (0f64, 0usize, 0f64, 0usize);
    for (i, &n) in occ.iter().enumerate() {
        if n == 0 {
            continue;
        }
        let (x, y) = ((i % d.w) as f32 + 0.5, (i / d.w) as f32 + 0.5);
        if near.iter().any(|&((cx, cy), r)| (x - cx).hypot(y - cy) <= r) {
            a += d.trail[i] as f64;
            na += 1;
        } else {
            b += d.trail[i] as f64;
            nb += 1;
        }
    }
    ((a / na.max(1) as f64) as f32, (b / nb.max(1) as f64) as f32)
}

#[allow(clippy::too_many_arguments)]
fn run(seed: u64, arm: usize, eat: Option<f32>, crowd: u8, full: f32, flow: f32, until: f32, look: Option<&str>, name: &str) -> Row {
    let walkers = 25_000;
    let mut p = race_params(seed, 2, walkers);
    p.upkeep = 0.0002;
    p.capacity = 50_000;
    if let Some(e) = eat {
        p.crystal_eat = e;
    }
    p.energy_flow = flow;
    p.fed_crowd = crowd;
    p.fed_full = full;
    let rules = Rules { mass: Rules::mass_for(walkers, 2), ..Rules::default() };
    let mut g = Game::new(BigDish::new(p), rules, seed);
    let start: Vec<f32> = g.world.crystals().iter().map(|c| c.1).collect();
    let mut row = Row { seed, arm, trail: (0.0, 0.0), packed: (0.0, 0.0), series: Vec::new() };
    let mut next = 30.0;
    while g.time() < until && g.end.is_none() {
        g.step();
        if g.time() >= next {
            let (n, c, share, e_on, e_off) = sample(&g, &start);
            row.series.push((next, n, c, share, e_on, e_off));
            next += 30.0;
        }
    }
    row.trail = trail_split(&g.world);
    row.packed = packed(&g.world);
    if let Some(dir) = look {
        image(&g.world).save(format!("{dir}/s{seed}-eat{name}-crowd{crowd}.png")).expect("png");
    }
    row
}

fn main() {
    let (mut seeds, mut eats, mut flow, mut until, mut threads) = ((1u64..=4).collect::<Vec<_>>(), vec![None, Some(0.0)], 0.0f32, 300.0f32, 4usize);
    let (mut crowds, mut full, mut look) = (vec![0u8], 2.0f32, None::<String>);
    for a in std::env::args().skip(1) {
        let (k, v) = a.split_once('=').expect("key=value");
        match k {
            "seeds" => {
                let (a, b) = v.split_once('-').unwrap_or((v, v));
                seeds = (a.parse().unwrap()..=b.parse().unwrap()).collect();
            }
            "eat" => eats = v.split(',').map(|s| if s == "default" { None } else { Some(s.parse().unwrap()) }).collect(),
            "flow" => flow = v.parse().unwrap(),
            "crowd" => crowds = v.split(',').map(|s| s.parse().unwrap()).collect(),
            "full" => full = v.parse().unwrap(),
            "look" => look = Some(v.to_string()),
            "until" => until = v.parse().unwrap(),
            "threads" => threads = v.parse().unwrap(),
            _ => panic!("unknown {k}"),
        }
    }
    let base = race_params(1, 2, 25_000).crystal_eat;
    let name = |e: &Option<f32>| e.map_or(format!("default {base}"), |e| e.to_string());
    println!("fedprobe shape C, seeds {seeds:?}, crystal_eat {:?}, fed_crowd {crowds:?} (fed_full {full}), energy_flow {flow}, until {until} s; idle squad, CPU race world scale 2, 25,000 walkers", eats.iter().map(name).collect::<Vec<_>>());
    // One arm a crystal_eat and fed_crowd pair; arm 0 is the control.
    let arms: Vec<(Option<f32>, u8)> = eats.iter().flat_map(|&e| crowds.iter().map(move |&c| (e, c))).collect();
    let label = |a: usize| format!("eat {} crowd {}", name(&arms[a].0), arms[a].1);
    let jobs: Vec<(u64, usize)> = seeds.iter().flat_map(|&s| (0..arms.len()).map(move |a| (s, a))).collect();
    let (tx, rx) = mpsc::channel();
    let queue = std::sync::Arc::new(std::sync::Mutex::new(jobs));
    let mut hs = Vec::new();
    for _ in 0..threads {
        let (tx, queue, arms, look) = (tx.clone(), queue.clone(), arms.clone(), look.clone());
        hs.push(std::thread::spawn(move || loop {
            let job = queue.lock().unwrap().pop();
            let Some((s, a)) = job else { break };
            let tag = arms[a].0.map_or("default".to_string(), |e| e.to_string());
            tx.send(run(s, a, arms[a].0, arms[a].1, full, flow, until, look.as_deref(), &tag)).unwrap();
        }));
    }
    drop(tx);
    let mut rows: Vec<Row> = Vec::new();
    for r in rx {
        let series = r.series.iter().map(|(t, n, c, sh, on, off)| format!("{t:.0}s {n} ({c} reached, {:.0}% near, energy {on:.2} near / {off:.2} off)", sh * 100.0)).collect::<Vec<_>>().join("; ");
        println!("seed {:>2} {}: {series}; trail at the end {:.2} near crystals / {:.2} elsewhere ({:.1}x); walkers {:.1}% of open cells, {:.1}% of them boxed in", r.seed, label(r.arm), r.trail.0, r.trail.1, r.trail.0 / r.trail.1.max(1e-6), 100.0 * r.packed.0, 100.0 * r.packed.1);
        rows.push(r);
    }
    for h in hs {
        h.join().unwrap();
    }
    println!("\nsummary, walkers at {until:.0} s (paired by seed against {}):", label(0));
    for a in 0..arms.len() {
        let end = |s: u64, a: usize| rows.iter().find(|r| r.seed == s && r.arm == a).and_then(|r| r.series.last()).map(|x| x.1).unwrap_or(0);
        let ns: Vec<usize> = seeds.iter().map(|&s| end(s, a)).collect();
        let (mut more, mut fewer) = (0, 0);
        for &s in &seeds {
            match end(s, a).cmp(&end(s, 0)) {
                std::cmp::Ordering::Greater => more += 1,
                std::cmp::Ordering::Less => fewer += 1,
                _ => {}
            }
        }
        let ratio: Vec<String> = seeds.iter().filter_map(|&s| rows.iter().find(|r| r.seed == s && r.arm == a)).map(|r| format!("{:.1}", r.trail.0 / r.trail.1.max(1e-6))).collect();
        println!("  {}: {ns:?}, mean {:.0} (more {more}, fewer {fewer}); trail near crystals / elsewhere {ratio:?}", label(a), ns.iter().sum::<usize>() as f32 / ns.len().max(1) as f32);
    }
}
