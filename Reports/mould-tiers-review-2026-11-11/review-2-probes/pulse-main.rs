#![allow(dead_code, clippy::all)]
//! Scratch prototype (not in the repo): does a relay pulse with a one-shot
//! directed burst move a whole body toward a periodic pacemaker?
pub mod big;
use big::{BigDish, BigParams, Pulse};
pub const W: usize = 256;
pub const H: usize = 160;
pub const STEPS_PER_SECOND: u32 = 120;
fn lowbias32(mut x: u32) -> u32 { x ^= x >> 16; x = x.wrapping_mul(0x7FEB_352D); x ^= x >> 15; x = x.wrapping_mul(0x846C_A68B); x ^ (x >> 16) }
pub fn keyed(seed: u64, t: u64, use_: u32, walker: u32) -> u32 {
    let s = (seed as u32) ^ ((seed >> 32) as u32) ^ use_.wrapping_mul(0x9E37_79B9);
    let step = lowbias32(lowbias32(s) ^ t as u32);
    lowbias32(walker ^ step)
}
pub fn unit(x: u32) -> f32 { (x >> 8) as f32 / (1u32 << 24) as f32 }

fn main() {
    let mut kv = std::collections::HashMap::new();
    for a in std::env::args().skip(1) { let (k, v) = a.split_once('=').unwrap(); kv.insert(k.to_string(), v.to_string()); }
    let g = |k: &str, d: f32| kv.get(k).map(|v| v.parse::<f32>().unwrap()).unwrap_or(d);
    let seed = g("seed", 1.0) as u64;
    let t = BigParams::today();
    let p = BigParams { scale: 4, walkers: g("walkers", 20000.0) as usize, scent_coarse: false, scent_offset: 36.0, blob: true, blob_r: g("blob_r", 40.0), piles: g("piles", 7.0) as usize, seed,
        upkeep: g("upkeep", 0.0), capacity: g("capacity", 0.0) as usize, graze_weight: g("graze_weight", 0.0), ..t };
    let mut d = BigDish::new(p);
    let mode = g("pulse", 0.0);
    if mode > 0.0 {
        d.pulse_on(Pulse { keep: g("keep", 0.98), fire: g("fire", 1.0), relay: g("relay", 20.0), refr: g("refr", 600.0) as u16,
            burst: g("burst", 60.0) as u16, dir_r: g("dir_r", 12.0), pace_scent: g("pace_scent", 0.0), pace_rate: g("pace_rate", 0.0), weight: g("weight", 0.0), burst_dep: g("burst_dep", 1.0) });
    }
    let (bx, by, br, bamp) = (g("beat_x", 95.0) * 4.0, g("beat_y", 80.0) * 4.0, g("beat_r", 8.0) * 4.0, g("beat_amp", 50.0));
    let (b0, bper) = (g("beat0", 4.0), g("beat_every", 6.0));
    let secs = g("seconds", 30.0);
    let tag = kv.get("tag").cloned().unwrap_or("run".into());
    let ticks = (secs * 120.0) as u64;
    let t0 = std::time::Instant::now();
    while d.ticks < ticks {
        d.tick();
        let s = d.ticks as f32 / 120.0;
        if mode > 0.0 && bper > 0.0 && s >= b0 && ((s - b0) * 120.0).round() as u64 % (bper * 120.0) as u64 == 0 {
            d.pulse_add(bx, by, br, bamp);
        }
        if d.ticks % 240 == 0 {
            let mut xs: Vec<f32> = (0..d.walkers.len()).filter(|&k| d.lives(k)).map(|k| d.walkers[k].0 / 4.0).collect();
            let n = xs.len();
            xs.sort_by(|a, b| a.total_cmp(b));
            let mean: f32 = xs.iter().sum::<f32>() / n as f32;
            let q = |f: f32| xs[((n as f32 * f) as usize).min(n - 1)];
            let m = d.measure();
            println!("{tag} seed {seed} t {:5.1} alive {n} meanx {:.1} p10 {:.1} p50 {:.1} p90 {:.1} piles {} fired {} ms/tick {:.1}", s, mean, q(0.1), q(0.5), q(0.9), m.reached, d.fired, t0.elapsed().as_secs_f64() * 1e3 / d.ticks as f64);
            d.fired = 0;
        }
    }
}
