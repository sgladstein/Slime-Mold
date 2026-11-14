//! **The food probe** (2026-11-14, the game designer's review of the C4
//! strategy check, PR #213: "size follows food held"). Does shape C's size
//! follow the crystals it holds, or does a body on one or two crystals keep
//! thousands of walkers?
//!
//! ```text
//! cargo run --release --example foodprobe -- seeds=1-4 [grow=180 keep=2,4,8 after=120 keys=k:v+k:v]
//! ```
//!
//! Grows the CPU race world with shape C's keys (`capacity` 50,000,
//! `upkeep` 0.0002, `energy_flow` 0.05, `wither` 0.01, `panic` 2, then
//! `keys=`) for `grow` game seconds, printing its food budget every 30 s:
//! walkers, their mean energy, what they eat a step (`BigDish::took`)
//! against what they pay (`upkeep` each), and the crystals they eat from.
//! Then, per `keep`, from a copy of that dish: every crystal is emptied
//! (its charge set to 0, as a squad's harvest leaves it) except the `keep`
//! the mould eats most from, and the walkers are counted every 10 s for
//! `after` s. **Deciding measure, named before the run:** walkers `after`
//! s on, as a share of the walkers before. Size follows food when it falls
//! with fewer crystals kept, and a crystal's worth of walkers is about
//! `capacity` / the crystals the full body eats from.
use slime_mould::mould::big::BigDish;
use slime_mould::mould::larder_world::race_params;
use slime_mould::mould::tiers::set_key;

const SCALE: usize = 2;
const TPS: f32 = 15.0;

fn main() {
    let mut seeds: Vec<u64> = (1..=4).collect();
    let (mut grow, mut after, mut keep, mut keys) = (180.0f32, 120.0f32, vec![2usize, 4, 8], Vec::<(String, String)>::new());
    for a in std::env::args().skip(1) {
        let (k, v) = a.split_once('=').expect("key=value");
        match k {
            "seeds" => {
                let (a, b) = v.split_once('-').unwrap_or((v, v));
                seeds = (a.parse().unwrap()..=b.parse().unwrap()).collect();
            }
            "grow" => grow = v.parse().unwrap(),
            "after" => after = v.parse().unwrap(),
            "keep" => keep = v.split(',').map(|s| s.parse().unwrap()).collect(),
            "keys" => keys = v.split('+').map(|kv| kv.split_once(':').map(|(a, b)| (a.to_string(), b.to_string())).expect("keys=k:v+k:v")).collect(),
            _ => panic!("unknown {k}"),
        }
    }
    let base: Vec<(String, String)> = [("capacity", "50000"), ("energy_flow", "0.05"), ("wither", "0.01"), ("panic", "2")].iter().map(|(a, b)| (a.to_string(), b.to_string())).collect();
    println!("foodprobe: race world scale {SCALE}, 25000 walkers, upkeep 0.0002, grow {grow} s, keys {base:?} then {keys:?}");
    for &seed in &seeds {
        let mut p = race_params(seed, SCALE, 25_000);
        p.upkeep = 0.0002;
        for (k, v) in base.iter().chain(&keys) {
            set_key(&mut p, k, v).unwrap();
        }
        let mut d = BigDish::new(p);
        d.threads = 4;
        // What each pile fed this run, summed over the last 30 s.
        let mut fed = vec![0f64; d.piles.len()];
        let ticks = (grow * TPS) as u32;
        for t in 1..=ticks {
            d.tick();
            for k in 0..d.walkers.len() {
                let took = d.took.get(k).copied().unwrap_or(0.0);
                if took > 0.0 {
                    let (x, y, _) = d.walkers[k];
                    let i = (y as usize) * d.w + x as usize;
                    let pi = d.pile_of[i] as usize;
                    if pi > 0 {
                        fed[pi - 1] += took as f64;
                    }
                }
            }
            if t % (30.0 * TPS) as u32 == 0 {
                budget(&d, seed, t as f32 / TPS, &fed);
                if t < ticks {
                    fed.iter_mut().for_each(|f| *f = 0.0);
                }
            }
        }
        // The crystals the mould eats most from, most first.
        let mut order: Vec<usize> = (0..d.piles.len()).collect();
        order.sort_by(|&a, &b| fed[b].partial_cmp(&fed[a]).unwrap());
        let before = living(&d);
        for &h in &keep {
            let mut e = d.clone();
            for &pi in &order[h.min(order.len())..] {
                for &i in &e.pile_cells.clone() {
                    if e.pile_of[i as usize] as usize == pi + 1 {
                        e.charge[i as usize] = 0.0;
                    }
                }
            }
            let mut line = format!("seed {seed}: keep {h:>2} crystals ({:.0}% of the food it ate):", 100.0 * order[..h.min(order.len())].iter().map(|&i| fed[i]).sum::<f64>() / fed.iter().sum::<f64>().max(1e-9));
            for t in 1..=(after * TPS) as u32 {
                e.tick();
                if t % (10.0 * TPS) as u32 == 0 {
                    line += &format!(" {:.0}", living(&e));
                }
            }
            let n = living(&e);
            println!("{line}; {n} of {before} ({:.0}%)", 100.0 * n as f32 / before as f32);
        }
    }
}

fn living(d: &BigDish) -> usize {
    (0..d.walkers.len()).filter(|&k| d.lives(k)).count()
}

fn budget(d: &BigDish, seed: u64, secs: f32, fed: &[f64]) {
    let live: Vec<usize> = (0..d.walkers.len()).filter(|&k| d.lives(k)).collect();
    let n = live.len();
    let e: f64 = live.iter().map(|&k| d.energy[k] as f64).sum();
    let eat: f64 = live.iter().map(|&k| d.took.get(k).copied().unwrap_or(0.0) as f64).sum();
    let on = fed.iter().filter(|&&f| f > 0.0).count();
    let mut top: Vec<f64> = fed.to_vec();
    top.sort_by(|a, b| b.partial_cmp(a).unwrap());
    // `took` is read once a tick, after its last fine step: a step's take.
    let steps = 30.0 * TPS as f64;
    println!(
        "seed {seed} at {secs:>3.0} s: {n} walkers, mean energy {:.2}; eats {eat:.2} a step against upkeep {:.2}; eats from {on} crystals, the top five {:?} a step; crystals left {:.3}",
        e / n.max(1) as f64,
        n as f64 * d.p.upkeep as f64,
        top.iter().take(5).map(|f| (f / steps * 100.0).round() / 100.0).collect::<Vec<_>>(),
        d.crystals_left()
    );
}
