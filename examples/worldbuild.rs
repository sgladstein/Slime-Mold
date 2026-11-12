//! **Does a lure pull the mould?** (Larder world buildings, 2026-11-12.)
//!
//! ```text
//! cargo run --release --example worldbuild -- seeds=1-8 [scale=2 walkers=5000 at=0.35 secs=60]
//! ```
//!
//! On the race world, a lure `at` of the way from the mould's home crystal
//! to the larder, with one soldier on Hold fire standing on it in both arms
//! (so the soldier's own smell is the same), on and switched off, paired by
//! seed. **Deciding measure, named before the run:** walkers within
//! `R` dish cells of the lure at `secs` game seconds, lure on against off,
//! sign test. Printed every 10 s as well, with the lure's food burnt.
use slime_mould::mould::larder_world::{race_world, Cmd, Game, Kind, Rules, Stance, World};

const R: f32 = 8.0;

fn main() {
    let (mut seeds, mut scale, mut walkers, mut at, mut secs) = ((1u64..=8).collect::<Vec<_>>(), 2usize, 5_000usize, 0.35f32, 60u32);
    let mut rules = Rules { start_bank: 1e6, ..Rules::default() };
    for a in std::env::args().skip(1) {
        let (k, v) = a.split_once('=').expect("key=value");
        match k {
            "seeds" => {
                let (a, b) = v.split_once('-').unwrap_or((v, v));
                seeds = (a.parse().unwrap()..=b.parse().unwrap()).collect();
            }
            "scale" => scale = v.parse().unwrap(),
            "walkers" => walkers = v.parse().unwrap(),
            "at" => at = v.parse().unwrap(),
            "secs" => secs = v.parse().unwrap(),
            "lure_smell" => rules.build.lure_smell = v.parse().unwrap(),
            "lure_r" => rules.build.lure_r = v.parse().unwrap(),
            _ => panic!("unknown {k}"),
        }
    }
    println!("worldbuild: lure smell {} (x a soldier's), radius {} dish cells", rules.build.lure_smell, rules.build.lure_r);
    println!("worldbuild: lure {at} of the way from the core to the larder, walkers within {R} dish cells, scale {scale}, walkers {walkers}, {secs} s");
    let (mut more, mut less) = (0, 0);
    for &seed in &seeds {
        let mut row = Vec::new();
        for on in [true, false] {
            let mut g = Game::new(race_world(seed, scale, walkers), rules, seed);
            let s = g.world.scale();
            let (c, l) = (g.core, g.larder);
            let p = (c.0 + (l.0 - c.0) * at, c.1 + (l.1 - c.1) * at);
            let site = (0..40).flat_map(|r| (0..16).map(move |k| (r, k))).map(|(r, k)| {
                let a = k as f32 / 16.0 * std::f32::consts::TAU;
                (p.0 + r as f32 * a.cos(), p.1 + r as f32 * a.sin())
            }).find(|q| g.world.open(q.0, q.1)).expect("open ground");
            g.soldiers[0].x = site.0;
            g.soldiers[0].y = site.1;
            g.apply(Cmd::Stance(0, Stance::HoldFire));
            assert!(g.apply(Cmd::Build(Kind::Lure, site)));
            if !on {
                g.apply(Cmd::Toggle(0));
            }
            let mut near = Vec::new();
            for t in 1..=secs {
                for _ in 0..g.rules.tps {
                    g.step();
                }
                if t % 10 == 0 {
                    let r2 = (R * s) * (R * s);
                    let n = g.world.walkers.iter().enumerate().filter(|(k, w)| g.world.lives(*k) && (w.0 - site.0).powi(2) + (w.1 - site.1).powi(2) <= r2).count();
                    near.push(n);
                }
            }
            println!("  seed {seed} lure {}: near the lure every 10 s {near:?}; burnt {:.0} food; mould {}", if on { "on " } else { "off" }, g.lured, g.world.living());
            row.push(*near.last().unwrap());
        }
        if row[0] > row[1] {
            more += 1;
        } else if row[0] < row[1] {
            less += 1;
        }
    }
    let n = more + less;
    let k = more.min(less);
    let mut p = 0.0f64;
    for i in 0..=k {
        p += (1..=i).fold(1.0f64, |acc, j| acc * (n - j + 1) as f64 / j as f64) / 2f64.powi(n as i32);
    }
    println!("lure on against off at {secs} s: more walkers near it on {more}, fewer on {less} (two-sided sign test p {:.4})", (2.0 * p).min(1.0));
}
