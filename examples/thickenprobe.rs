//! **The thickening probe** (2026-11-15, the slime-enemy thread). Does a
//! slime that holds its crystals keep getting stronger, and faster the more
//! crystals it holds? The owner, 11-15 08:34Z: it should "keep getting
//! stronger by holding onto its crystals and getting more of them (which
//! increases how fast it multiplies)".
//!
//! ```text
//! cargo run --release --example thickenprobe -- seeds=1-2 [scale=1 grow=480 keep=0,4 keys=thicken:2]
//! ```
//!
//! Grows the CPU race world with shape C's keys (`capacity` twice the start
//! walkers, `upkeep` 0.0002, `energy_flow` 0.05, `wither` 0.01, `panic` 2,
//! then `keys=`) from C's walkers per open cell (25,000 at scale 2, 6,250
//! at scale 1) for `grow` game seconds, and prints every 30 s: walkers,
//! condensed walkers, mass (a condensed one twice), mean energy, what they
//! eat a step against their upkeep, and the crystals they eat from. Per
//! `keep` (0 is every crystal): every crystal but the `keep` nearest the
//! start is emptied before the first step, so the slime holds that many.
//! **Deciding measure, named before the run:** mass at the last line,
//! against the slots; stronger means mass past the slots, and "more
//! crystals multiply it faster" means a higher mass at every line with
//! more crystals kept.
use slime_mould::mould::big::BigDish;
use slime_mould::mould::larder_world::race_params;
use slime_mould::mould::tiers::set_key;

const TPS: f32 = 15.0;

fn main() {
    let mut seeds: Vec<u64> = (1..=2).collect();
    let (mut scale, mut grow, mut keep, mut keys) = (1usize, 480.0f32, vec![0usize], Vec::<(String, String)>::new());
    for a in std::env::args().skip(1) {
        let (k, v) = a.split_once('=').expect("key=value");
        match k {
            "seeds" => {
                let (a, b) = v.split_once('-').unwrap_or((v, v));
                seeds = (a.parse().unwrap()..=b.parse().unwrap()).collect();
            }
            "scale" => scale = v.parse().unwrap(),
            "grow" => grow = v.parse().unwrap(),
            "keep" => keep = v.split(',').map(|s| s.parse().unwrap()).collect(),
            "keys" => {
                keys = v
                    .split('+')
                    .map(|kv| {
                        kv.split_once(':')
                            .map(|(a, b)| (a.to_string(), b.to_string()))
                            .expect("keys=k:v+k:v")
                    })
                    .collect()
            }
            _ => panic!("unknown {k}"),
        }
    }
    let walkers = 25_000 * scale * scale / 4;
    let cap = format!("{}", 2 * walkers);
    let base: Vec<(String, String)> = [
        ("capacity", cap.as_str()),
        ("energy_flow", "0.05"),
        ("wither", "0.01"),
        ("panic", "2"),
    ]
    .iter()
    .map(|(a, b)| (a.to_string(), b.to_string()))
    .collect();
    println!("thickenprobe: race world scale {scale}, {walkers} walkers, upkeep 0.0002, grow {grow} s, keys {base:?} then {keys:?}");
    for &seed in &seeds {
        for &kept in &keep {
            let mut p = race_params(seed, scale, walkers);
            p.upkeep = 0.0002;
            for (k, v) in base.iter().chain(&keys) {
                set_key(&mut p, k, v).unwrap();
            }
            let mut d = BigDish::new(p);
            d.threads = 4;
            if kept > 0 {
                // The start's middle, and the crystals nearest it kept.
                let live: Vec<usize> = (0..d.walkers.len()).filter(|&k| d.lives(k)).collect();
                let (mx, my) = live
                    .iter()
                    .fold((0.0, 0.0), |a, &k| (a.0 + d.walkers[k].0, a.1 + d.walkers[k].1));
                let (mx, my) = (mx / live.len() as f32, my / live.len() as f32);
                let mut order: Vec<(f32, usize)> = (0..d.piles.len())
                    .map(|j| {
                        let c = d.pile_centre(j);
                        let (x, y) = ((c & 0xFFFF) as f32, (c >> 16) as f32);
                        ((x - mx).hypot(y - my), j)
                    })
                    .collect();
                order.sort_by(|a, b| a.0.partial_cmp(&b.0).unwrap());
                let gone: Vec<usize> = order[kept.min(order.len())..].iter().map(|o| o.1 + 1).collect();
                for i in d.pile_cells.clone() {
                    if gone.contains(&(d.pile_of[i as usize] as usize)) {
                        d.charge[i as usize] = 0.0;
                    }
                }
            }
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
                    line(&d, seed, kept, t as f32 / TPS, &fed);
                    fed.iter_mut().for_each(|f| *f = 0.0);
                }
            }
        }
    }
}

fn line(d: &BigDish, seed: u64, kept: usize, secs: f32, fed: &[f64]) {
    let live: Vec<usize> = (0..d.walkers.len()).filter(|&k| d.lives(k)).collect();
    let n = live.len();
    let dense = live.iter().filter(|&&k| d.is_dense(k)).count();
    let e: f64 = live.iter().map(|&k| d.energy[k] as f64).sum();
    let eat: f64 = live.iter().map(|&k| d.took.get(k).copied().unwrap_or(0.0) as f64).sum();
    let up: f64 = (n + dense) as f64 * d.p.upkeep as f64;
    let on = fed.iter().filter(|&&f| f > 0.0).count();
    println!(
        "seed {seed} keep {kept:>2} at {secs:>3.0} s: {n} walkers, {dense} thick, mass {} of {} slots; mean energy {:.2}; eats {eat:.2} a step against upkeep {up:.2}; eats from {on} crystals; thickened {}",
        n + dense,
        d.walkers.len(),
        e / n.max(1) as f64,
        d.thickened
    );
}
