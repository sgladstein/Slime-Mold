//! **Lone walkers at density** (owner, 2026-11-12 22:06-22:07Z, playing
//! shape D: "after killing almost all the mold, having the kill the few
//! random stragglers spread across the map to win is annoying"; "There are
//! still clearly lone walkers moving around").
//!
//! ```text
//! cargo run --release --example loneprobe -- [seeds=1-4 shape=d lone=1,3,10 lone_r=1 sweep=180 keep=0.02 until=300 threads=4]
//! ```
//!
//! The CPU race world (`larder_world::race_world`, scale 2) with one of the
//! shapes page's dense moulds (`shape=c|d`: 25,000 walkers, upkeep 0.0002,
//! capacity 50,000; D adds sensors at 22.5 degrees, 5 cells out), an idle
//! squad, and `BigParams::lone_upkeep` at each of `lone=` (1 is off): a
//! walker with no other walker in the square of radius `lone_r` round its
//! cell pays that many times the upkeep, so a straggler starves in seconds
//! unless it eats or rejoins. At `sweep` game seconds every walker near a
//! crystal is killed and of the rest all but a `keep` share picked at
//! random, as a player's sweep that holds the crystals and leaves
//! stragglers everywhere else, and the remnant is followed to `until`.
//! `broken=B starve_x=X` tries the broken mould (`Rules::broken`): under B
//! x its peak walkers, upkeep is X times.
//!
//! The dead end it re-tests (`Reports/dead-ends.md`, `lone_upkeep` for one
//! body): at today's 5,000 walkers the scouts that find crystals are lone,
//! so 5-20 stopped the spread; re-test when the body spreads another way.
//! At 25,000 walkers with one walker a cell the front is a sheet.
//!
//! **Measures, named before the run:**
//! - before the sweep, every 30 s: walkers, `lone` (the share of walkers
//!   with no other walker on the 8 cells round them, the look measure),
//!   `reached` (crystals the mould has eaten from);
//! - **deciding for the spread:** crystals reached at the sweep, paired by
//!   seed against `lone=1`;
//! - **deciding for the stragglers:** game seconds after the sweep until
//!   under 1% of the walkers at the sweep are left (`clear`), and walkers at
//!   `until` (whether the remnant regrows from the crystals it sits on).

use std::sync::mpsc;

use slime_mould::mould::big::BigDish;
use slime_mould::mould::larder_world::{race_params, Game, Rules, World};

struct Arm {
    lone: f32,
    /// The broken mould (`Rules::broken`, `broken_starve`): under `broken` x
    /// its peak walkers, upkeep is `starve_x` times (0: off).
    broken: f32,
    starve_x: f32,
}

struct Row {
    seed: u64,
    lone: f32,
    series: Vec<(f32, usize, f32, usize)>,
    at_sweep: (usize, usize),
    clear: Option<f32>,
    after: Vec<(f32, usize, f32)>,
    end: usize,
}

fn lone_share(d: &BigDish) -> f32 {
    let occ = d.occupancy();
    let (w, h) = (d.w as i64, d.h as i64);
    let (mut n, mut lone) = (0usize, 0usize);
    for k in 0..d.walkers.len() {
        if !d.lives(k) {
            continue;
        }
        let (x, y, _) = d.walkers[k];
        let (cx, cy) = (x as i64, y as i64);
        let mut others = 0u32;
        for dy in -1..=1 {
            for dx in -1..=1 {
                let (nx, ny) = (cx + dx, cy + dy);
                if nx >= 0 && ny >= 0 && nx < w && ny < h {
                    others += occ[(ny * w + nx) as usize] as u32;
                }
            }
        }
        n += 1;
        if others <= 1 {
            lone += 1;
        }
    }
    if n == 0 {
        0.0
    } else {
        lone as f32 / n as f32
    }
}

fn reached(g: &Game<BigDish>, start: &[f32]) -> usize {
    g.world.crystals().iter().zip(start).filter(|(c, &s)| c.1 < s * 0.999).count()
}

fn hash(seed: u64, k: u64) -> f32 {
    let mut x = seed.wrapping_mul(0x9E37_79B9_7F4A_7C15) ^ k.wrapping_mul(0xBF58_476D_1CE4_E5B9);
    x ^= x >> 31;
    x = x.wrapping_mul(0x94D0_49BB_1331_11EB);
    x ^= x >> 29;
    (x >> 40) as f32 / (1u64 << 24) as f32
}

#[allow(clippy::too_many_arguments)]
fn run(seed: u64, shape: &str, arm: &Arm, lone_r: u8, sweep: f32, keep: f32, until: f32) -> Row {
    let walkers = 25_000;
    let mut p = race_params(seed, 2, walkers);
    p.upkeep = 0.0002;
    p.capacity = 50_000;
    if shape == "d" {
        p.sensor_angle = 22.5f32.to_radians();
        p.sensor_offset = 5.0;
    }
    p.lone_upkeep = arm.lone;
    p.lone_r = lone_r;
    // The broken mould is the game's own rule (`Rules::broken`, on by
    // default since this probe); `broken=0` is the control.
    let rules = Rules { mass: Rules::mass_for(walkers, 2), broken: arm.broken, broken_starve: arm.starve_x, ..Rules::default() };
    let mut g = Game::new(BigDish::new(p), rules, seed);
    let start: Vec<f32> = g.world.crystals().iter().map(|c| c.1).collect();
    let mut row = Row { seed, lone: arm.lone, series: Vec::new(), at_sweep: (0, 0), clear: None, after: Vec::new(), end: 0 };
    let mut next = 30.0;
    while g.time() < sweep && g.end.is_none() {
        g.step();
        if g.time() >= next {
            row.series.push((next, g.mould, lone_share(&g.world), reached(&g, &start)));
            next += 30.0;
        }
    }
    let before = g.mould;
    row.at_sweep = (before, reached(&g, &start));
    // The player's sweep as the owner played it (40 soldiers holding the
    // crystals, then hunting): every walker within twice a crystal's radius
    // of one dies, and of the rest all but `keep`, so the remnant is
    // stragglers off food. (A first version kept `keep` of every walker,
    // crystals too: the fed ones bred it back to 3,000 in 5 s, which is not
    // the endgame the owner met.)
    // Crystals, their radii and walkers are all fine cells (`BigDish`'s
    // grid is `scale` times today's).
    let near: Vec<((f32, f32), f32)> = g.world.crystals().iter().enumerate().map(|(c, x)| (x.0, 2.0 * g.world.crystal_r(c))).collect();
    for k in 0..g.world.walkers.len() {
        if !g.world.lives(k) {
            continue;
        }
        let (x, y, _) = g.world.walkers[k];
        let fed = near.iter().any(|&((cx, cy), r)| (x - cx).hypot(y - cy) <= r);
        if fed || hash(seed, k as u64) >= keep {
            g.world.kill(k);
        }
    }
    g.step();
    let mut next = sweep + 5.0;
    while g.time() < until && g.end.is_none() {
        g.step();
        if row.clear.is_none() && (g.mould as f32) < 0.01 * before as f32 {
            row.clear = Some(g.time() - sweep);
        }
        if g.time() >= next {
            row.after.push((next - sweep, g.mould, lone_share(&g.world)));
            next += 5.0;
        }
    }
    row.end = g.mould;
    row
}

fn main() {
    let (mut seeds, mut shape, mut lones, mut lone_r) = ((1u64..=4).collect::<Vec<_>>(), "d".to_string(), vec![1.0f32, 3.0, 10.0], 1u8);
    let (mut sweep, mut keep, mut until, mut threads) = (180.0f32, 0.02f32, 300.0f32, 4usize);
    let (mut broken, mut starve_x) = (Rules::default().broken, Rules::default().broken_starve);
    for a in std::env::args().skip(1) {
        let (k, v) = a.split_once('=').expect("key=value");
        match k {
            "seeds" => {
                let (a, b) = v.split_once('-').unwrap_or((v, v));
                seeds = (a.parse().unwrap()..=b.parse().unwrap()).collect();
            }
            "shape" => shape = v.to_string(),
            "lone" => lones = v.split(',').map(|s| s.parse().unwrap()).collect(),
            "lone_r" => lone_r = v.parse().unwrap(),
            "sweep" => sweep = v.parse().unwrap(),
            "keep" => keep = v.parse().unwrap(),
            "until" => until = v.parse().unwrap(),
            "threads" => threads = v.parse().unwrap(),
            "broken" => broken = v.parse().unwrap(),
            "starve_x" => starve_x = v.parse().unwrap(),
            _ => panic!("unknown {k}"),
        }
    }
    println!("loneprobe shape {shape}, seeds {seeds:?}, lone {lones:?}, lone_r {lone_r}, sweep at {sweep} s keeping {keep}, until {until} s; broken {broken} starve_x {starve_x}; idle squad, CPU race world scale 2, 25,000 walkers");
    let jobs: Vec<(u64, f32)> = seeds.iter().flat_map(|&s| lones.iter().map(move |&l| (s, l))).collect();
    let (tx, rx) = mpsc::channel();
    let queue = std::sync::Arc::new(std::sync::Mutex::new(jobs.clone()));
    let mut hs = Vec::new();
    for _ in 0..threads {
        let (tx, queue, shape) = (tx.clone(), queue.clone(), shape.clone());
        hs.push(std::thread::spawn(move || loop {
            let job = queue.lock().unwrap().pop();
            let Some((s, l)) = job else { break };
            tx.send(run(s, &shape, &Arm { lone: l, broken, starve_x }, lone_r, sweep, keep, until)).unwrap();
        }));
    }
    drop(tx);
    let mut rows: Vec<Row> = Vec::new();
    for r in rx {
        let series = r.series.iter().map(|(t, n, l, c)| format!("{t:.0}s {n} lone {:.3} reached {c}", l)).collect::<Vec<_>>().join("; ");
        let after = r.after.iter().map(|(t, n, l)| format!("+{t:.0}s {n} ({:.2})", l)).collect::<Vec<_>>().join(" ");
        println!("seed {:>2} lone {:>4}: {series}", r.seed, r.lone);
        println!("    sweep: {} walkers, {} crystals reached; clear {}; after: {after}; end {}", r.at_sweep.0, r.at_sweep.1, r.clear.map_or("never".to_string(), |c| format!("{c:.1} s")), r.end);
        rows.push(r);
    }
    for h in hs {
        h.join().unwrap();
    }
    println!("\nsummary (paired by seed against lone {}):", lones[0]);
    for &l in &lones {
        let mine: Vec<&Row> = seeds.iter().filter_map(|&s| rows.iter().find(|r| r.seed == s && r.lone == l)).collect();
        let base: Vec<&Row> = seeds.iter().filter_map(|&s| rows.iter().find(|r| r.seed == s && r.lone == lones[0])).collect();
        let mean = |f: &dyn Fn(&Row) -> f32| mine.iter().map(|r| f(r)).sum::<f32>() / mine.len().max(1) as f32;
        let lone_at = |r: &Row| r.series.last().map_or(0.0, |s| s.2);
        let (mut fewer, mut more) = (0, 0);
        for (a, b) in mine.iter().zip(&base) {
            if a.at_sweep.1 < b.at_sweep.1 {
                fewer += 1;
            } else if a.at_sweep.1 > b.at_sweep.1 {
                more += 1;
            }
        }
        let cleared = mine.iter().filter(|r| r.clear.is_some()).count();
        println!(
            "  lone {l:>4}: walkers at sweep {:.0}, lone share {:.3}, crystals reached {:.1} (fewer {fewer}, more {more} than lone {}); cleared {cleared}/{} in {:.1} s mean; walkers at {until:.0} s {:.0}",
            mean(&|r| r.at_sweep.0 as f32),
            mean(&lone_at),
            mean(&|r| r.at_sweep.1 as f32),
            lones[0],
            mine.len(),
            mine.iter().filter_map(|r| r.clear).sum::<f32>() / cleared.max(1) as f32,
            mean(&|r| r.end as f32),
        );
    }
}
