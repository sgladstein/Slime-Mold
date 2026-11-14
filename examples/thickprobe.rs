//! **Soldiers in thick slime** (owner, 2026-11-12 22:18Z, playing shape C:
//! "The soldiers have too much health or the slime doesn't hurt them
//! enough. I can walk my group of soliders into the thickest part of the
//! slime and kill most of the slime and harvest the whole crystal without
//! losing a single soldier (the whole game)").
//!
//! ```text
//! cargo run --release --example thickprobe -- [seeds=1-4 shape=c drain=1,2,4,8 n=40 at=240 for=60 order=fight threads=4 wade=6]
//! ```
//!
//! The CPU race world (`larder_world::race_world`, scale 2) with one of the
//! shapes page's dense moulds (`shape=c|d`: 25,000 walkers, upkeep 0.0002,
//! capacity 50,000; D adds sensors at 22.5 degrees, 5 cells out) grows for
//! `at` game seconds while a squad of `n` stands at the larder holding fire
//! and taking no harm. Then the squad, on its default stance (Defend), is
//! ordered into the thickest part of the mould (the 13 x 13 fine-cell block
//! holding the most walkers), as an attack-move (`order=fight`) or a plain
//! move (`order=move`), with `Rules::drain` at each of `drain=` times its
//! default, and followed for `for` game seconds. (The first runs, before
//! the bite went to 2 a walker at any mass, multiplied 1 x mass 0.2:
//! their x10 is today's default.)
//!
//! **Measures, named before the run:**
//! - **deciding:** soldiers lost of `n` at the end, by drain, on seeds 1-4,
//!   against the owner's aim as this thread put it to them (22:19Z): 40
//!   walked into C's thickest part should lose about a quarter;
//! - beside it: when the first one dies, walkers killed, the mould left,
//!   and the mean and most walkers touching a live soldier (whether the
//!   gap is the drain or the touching).

use std::sync::mpsc;

use slime_mould::mould::big::BigDish;
use slime_mould::mould::larder_world::{race_params, Cmd, Game, Order, Rules, Stance};

struct Row {
    seed: u64,
    drain: f32,
    walkers: usize,
    lost: Vec<(f32, usize)>,
    first: Option<f32>,
    killed: u32,
    end: usize,
    touch_mean: f32,
    touch_max: u32,
    arrived: Option<f32>,
    /// Soldier-frames by walkers touching: 0, 1-4, 5-9, 10-19, 20 or more.
    bins: [u64; 5],
}

/// The centre of the 13 x 13 fine-cell block holding the most walkers
/// whose centre a soldier can stand on (`BigDish`'s grid is fine cells). (The densest blocks are
/// often crystals, which are solid: a first version ordered the squad onto
/// one and on seed 3 it never met the mould.)
fn thickest(g: &Game<BigDish>) -> (f32, f32) {
    let d = &g.world;
    let occ = d.occupancy();
    let (w, h) = (d.w, d.h);
    let r = 6usize;
    let mut blocks = Vec::new();
    for y in (r..h.saturating_sub(r)).step_by(2) {
        for x in (r..w.saturating_sub(r)).step_by(2) {
            let mut n = 0u32;
            for yy in y - r..=y + r {
                for xx in x - r..=x + r {
                    n += occ[yy * w + xx] as u32;
                }
            }
            blocks.push((n, x, y));
        }
    }
    blocks.sort_by_key(|b| std::cmp::Reverse(b.0));
    blocks.iter().map(|&(_, x, y)| (x as f32, y as f32)).find(|&(x, y)| g.standable(x, y)).unwrap_or(g.larder)
}

#[allow(clippy::too_many_arguments)]
fn run(seed: u64, shape: &str, drain: f32, n: usize, at: f32, secs: f32, fight: bool, wade: f32) -> Row {
    let walkers = 25_000;
    let mut p = race_params(seed, 2, walkers);
    p.upkeep = 0.0002;
    p.capacity = 50_000;
    if shape == "d" {
        p.sensor_angle = 22.5f32.to_radians();
        p.sensor_offset = 5.0;
    }
    // No rations and no overrun (both later than the first runs): a squad
    // held at the larder for 240 s would otherwise starve, or lose the
    // round to the mould at the larder before the order (seeds 1, 2 and
    // 4 with 10 soldiers, 2026-11-13). The probe measures the bite.
    let rules = Rules { mass: Rules::mass_for(walkers, 2), squad: n, max_squad: n.max(40), wade, ration: 0.0, overrun: 0.0, ..Rules::default() };
    let base = rules.drain;
    let mut g = Game::new(BigDish::new(p), rules, seed);
    g.rules.drain = 0.0;
    for k in 0..g.soldiers.len() {
        g.apply(Cmd::Stance(k, Stance::HoldFire));
    }
    while g.time() < at && g.end.is_none() {
        g.step();
    }
    let to = thickest(&g);
    let sc = g.world.p.scale as f32;
    g.rules.drain = base * drain;
    for k in 0..g.soldiers.len() {
        g.soldiers[k].hp = g.rules.hp;
        g.apply(Cmd::Stance(k, Stance::Defend));
        g.apply(Cmd::Order(k, if fight { Order::Fight(to.0, to.1) } else { Order::Move(to.0, to.1) }));
    }
    let (start, killed0) = (g.mould, g.killed);
    let mut row = Row { seed, drain, walkers: start, lost: Vec::new(), first: None, killed: 0, end: 0, touch_mean: 0.0, touch_max: 0, arrived: None, bins: [0; 5] };
    let (mut tsum, mut tn) = (0u64, 0u64);
    let mut next = at + 10.0;
    while g.time() < at + secs && g.end.is_none() {
        g.step();
        let alive: Vec<_> = g.soldiers.iter().filter(|s| s.alive).collect();
        let dead = n - alive.len();
        if dead > 0 && row.first.is_none() {
            row.first = Some(g.time() - at);
        }
        for s in &alive {
            tsum += s.touching as u64;
            tn += 1;
            row.touch_max = row.touch_max.max(s.touching);
            row.bins[match s.touching {
                0 => 0,
                1..=4 => 1,
                5..=9 => 2,
                10..=19 => 3,
                _ => 4,
            }] += 1;
        }
        if row.arrived.is_none() && !alive.is_empty() {
            let (mx, my) = (alive.iter().map(|s| s.x).sum::<f32>() / alive.len() as f32, alive.iter().map(|s| s.y).sum::<f32>() / alive.len() as f32);
            if (mx - to.0).hypot(my - to.1) < 6.0 * sc {
                row.arrived = Some(g.time() - at);
            }
        }
        if g.time() >= next {
            row.lost.push((next - at, dead));
            next += 10.0;
        }
    }
    row.killed = g.killed - killed0;
    row.end = g.mould;
    row.touch_mean = tsum as f32 / tn.max(1) as f32;
    row
}

fn main() {
    let (mut seeds, mut shape, mut drains, mut n) = ((1u64..=4).collect::<Vec<_>>(), "c".to_string(), vec![1.0f32, 2.0, 4.0, 8.0], 40usize);
    let (mut at, mut secs, mut fight, mut threads) = (240.0f32, 60.0f32, true, 4usize);
    let mut wade = Rules::default().wade;
    for a in std::env::args().skip(1) {
        let (k, v) = a.split_once('=').expect("key=value");
        match k {
            "seeds" => {
                let (a, b) = v.split_once('-').unwrap_or((v, v));
                seeds = (a.parse().unwrap()..=b.parse().unwrap()).collect();
            }
            "shape" => shape = v.to_string(),
            "drain" => drains = v.split(',').map(|s| s.parse().unwrap()).collect(),
            "n" => n = v.parse().unwrap(),
            "at" => at = v.parse().unwrap(),
            "for" => secs = v.parse().unwrap(),
            "order" => fight = v == "fight",
            "threads" => threads = v.parse().unwrap(),
            "wade" => wade = v.parse().unwrap(),
            _ => panic!("unknown {k}"),
        }
    }
    let r = Rules::default();
    println!(
        "thickprobe shape {shape}, seeds {seeds:?}, drain x {drains:?} (default {} hp/s a walker, hp {}, mass {}), squad {n}, into the thickest part at {at} s by {} for {secs} s; wade {wade}; CPU race world scale 2, 25,000 walkers",
        r.drain,
        r.hp,
        Rules::mass_for(25_000, 2),
        if fight { "attack-move" } else { "move" }
    );
    let jobs: Vec<(u64, f32)> = seeds.iter().flat_map(|&s| drains.iter().map(move |&d| (s, d))).collect();
    let (tx, rx) = mpsc::channel();
    let queue = std::sync::Arc::new(std::sync::Mutex::new(jobs));
    let mut hs = Vec::new();
    for _ in 0..threads {
        let (tx, queue, shape) = (tx.clone(), queue.clone(), shape.clone());
        hs.push(std::thread::spawn(move || loop {
            let job = queue.lock().unwrap().pop();
            let Some((s, d)) = job else { break };
            tx.send(run(s, &shape, d, n, at, secs, fight, wade)).unwrap();
        }));
    }
    drop(tx);
    let mut rows: Vec<Row> = Vec::new();
    for r in rx {
        let lost = r.lost.iter().map(|(t, d)| format!("+{t:.0}s {d}")).collect::<Vec<_>>().join(" ");
        println!(
            "seed {:>2} drain x{:<4}: mould {} at the order; arrived {}; first death {}; lost {lost}; killed {}, mould left {}; touching mean {:.1}, most {}; soldier-frames touched 0/1-4/5-9/10-19/20+ {:?}",
            r.seed,
            r.drain,
            r.walkers,
            r.arrived.map_or("never".into(), |t| format!("+{t:.0} s")),
            r.first.map_or("none".into(), |t| format!("+{t:.1} s")),
            r.killed,
            r.end,
            r.touch_mean,
            r.touch_max,
            r.bins
        );
        rows.push(r);
    }
    for h in hs {
        h.join().unwrap();
    }
    println!("\nsummary, soldiers lost of {n} at +{secs:.0} s:");
    for &d in &drains {
        let mine: Vec<&Row> = seeds.iter().filter_map(|&s| rows.iter().find(|r| r.seed == s && r.drain == d)).collect();
        let lost: Vec<usize> = mine.iter().map(|r| r.lost.last().map_or(0, |l| l.1)).collect();
        let mean = |f: &dyn Fn(&Row) -> f32| mine.iter().map(|r| f(r)).sum::<f32>() / mine.len().max(1) as f32;
        println!(
            "  drain x{d:<4}: lost {lost:?} (mean {:.1}); killed {:.0}, mould left {:.0} of {:.0}; touching mean {:.1}",
            lost.iter().sum::<usize>() as f32 / lost.len().max(1) as f32,
            mean(&|r| r.killed as f32),
            mean(&|r| r.end as f32),
            mean(&|r| r.walkers as f32),
            mean(&|r| r.touch_mean)
        );
    }
}
