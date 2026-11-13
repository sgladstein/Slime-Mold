//! The CPU cost of a squad read (`squad::read`), before and after aims
//! that see (2026-11-13). Grows shape C on the race world for `at` game
//! seconds with the squad holding fire and taking no harm (so the dish is
//! the same at either commit), then times `squad::read` for `n` units on
//! agar cells nearest the mould's walkers, `reps` times.
//!
//! cargo run --release --example aimcost -- [seeds=1-2 at=240 n=40 reps=400]
use std::time::Instant;

use slime_mould::mould::big::BigDish;
use slime_mould::mould::larder_world::{race_params, Cmd, Game, Rules, Stance};
use slime_mould::mould::squad::{self, UnitIn};

fn main() {
    let (mut seeds, mut at, mut n, mut reps) = (vec![1u64, 2], 240.0f32, 40usize, 400usize);
    for a in std::env::args().skip(1) {
        let (k, v) = a.split_once('=').expect("key=value");
        match k {
            "seeds" => {
                let (a, b) = v.split_once('-').unwrap_or((v, v));
                seeds = (a.parse().unwrap()..=b.parse().unwrap()).collect();
            }
            "at" => at = v.parse().unwrap(),
            "n" => n = v.parse().unwrap(),
            "reps" => reps = v.parse().unwrap(),
            _ => panic!("unknown key {k}"),
        }
    }
    for seed in seeds {
        let walkers = 25_000;
        let mut p = race_params(seed, 2, walkers);
        p.upkeep = 0.0002;
        p.capacity = 50_000;
        let rules = Rules { mass: Rules::mass_for(walkers, 2), ration: 0.0, overrun: 0.0, ..Rules::default() };
        let mut g = Game::new(BigDish::new(p), rules, seed);
        g.rules.drain = 0.0;
        for k in 0..g.soldiers.len() {
            g.apply(Cmd::Stance(k, Stance::HoldFire));
        }
        while g.time() < at && g.end.is_none() {
            g.step();
        }
        let d = &g.world;
        let (w, h) = (d.w, d.h);
        let occ = d.occupancy();
        // Units on agar cells 4-12 cells from a walker: the edge of the
        // body, where a squad stands and fires. Every 97th such cell.
        let mut at_edge = Vec::new();
        for y in 12..h - 12 {
            for x in 12..w - 12 {
                let i = y * w + x;
                if !d.agar[i] || occ[i] > 0 {
                    continue;
                }
                let near = (-12i64..=12).step_by(4).any(|dy| (-12i64..=12).step_by(4).any(|dx| {
                    let j = ((y as i64 + dy) as usize) * w + (x as i64 + dx) as usize;
                    (dx.abs().max(dy.abs()) >= 4) && occ[j] > 0
                }));
                if near {
                    at_edge.push((x, y));
                }
            }
        }
        let step = (at_edge.len() / n).max(1);
        let sc = 2.0;
        let r = Rules::default();
        let units: Vec<UnitIn> = at_edge.iter().step_by(step).take(n).map(|&(x, y)| UnitIn {
            x: x as f32 + 0.5,
            y: y as f32 + 0.5,
            touch: r.touch * sc,
            sight: r.range * sc,
            take_r: 4.0 * sc,
            take: 0.0,
            smell_r: r.smell_r * sc,
            smell: 0.0,
        }).collect();
        let mut out = squad::read(d, &units);
        let t = Instant::now();
        for _ in 0..reps {
            out = squad::read(d, &units);
        }
        let us = t.elapsed().as_secs_f64() * 1e6 / reps as f64;
        let aimed = out.iter().filter(|o| o.1.is_some()).count();
        let dense: u32 = out.iter().map(|o| o.2).sum();
        println!("seed {seed}: {} walkers at {at} s, {} units on the body's edge ({} candidate cells); squad::read {us:.1} us a frame ({:.2} us a unit); {aimed} aim, dense sum {dense}", g.mould, units.len(), at_edge.len(), us / units.len() as f64);
    }
}
