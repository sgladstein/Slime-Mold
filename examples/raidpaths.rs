//! Times a raid's walking distance (`BigDish::raid_paths`, 11-15) on a
//! planet: `cargo run --release --example raidpaths -- DEPTH SEED`. Prints
//! the world's size, the build's time for targets at a few crystals (on
//! the game's worker thread since 11-15), and what the frame itself pays:
//! copying the ground and starting the thread. Then the units' walking
//! distance (`big::units_field_for`, S1, 2026-11-17) from up to 40 sources
//! (the crystals, standing in for units).
use slime_mould::mould::big::BigDish;
use slime_mould::mould::larder_world::{race_params, raid_cell, World};
use slime_mould::mould::levelgen::level;
use std::time::Instant;

fn main() {
    let a: Vec<u64> = std::env::args().skip(1).filter_map(|v| v.parse().ok()).collect();
    let (depth, seed) = (a.first().copied().unwrap_or(8) as u32, a.get(1).copied().unwrap_or(1));
    let map = level(seed, depth);
    let mut d = BigDish::with_map(race_params(seed, 2, 5_000), &map);
    let cs = d.crystals();
    println!("planet {depth} seed {seed}: {} x {} fine cells, {} crystals", d.w, d.h, cs.len());
    for &(at, _) in cs.iter().take(5) {
        d.p.raid_at = 1;
        d.p.raid_to = raid_cell(at, (d.w, d.h));
        let t = Instant::now();
        d.raid_paths();
        let ms = t.elapsed().as_secs_f64() * 1e3;
        let reach = d.raid_field.iter().filter(|&&v| v < u32::MAX).count();
        println!("  target {:.0},{:.0}: {ms:.1} ms, {reach} of {} blocks reached", at.0, at.1, d.raid_field.len());
    }
    let (to, mut worst) = (d.p.raid_to, 0.0f64);
    for _ in 0..20 {
        let t = Instant::now();
        let (agar, w, h) = World::raid_ground(&d).unwrap();
        let j = std::thread::spawn(move || slime_mould::mould::big::raid_field_for(&agar, w, h, to));
        worst = worst.max(t.elapsed().as_secs_f64() * 1e3);
        j.join().unwrap();
    }
    println!("  on the frame (copy the ground, start the thread): worst of 20 {worst:.2} ms");
    let units: Vec<u32> = cs.iter().take(40).map(|&(at, _)| raid_cell(at, (d.w, d.h))).collect();
    let t = Instant::now();
    let f = slime_mould::mould::big::units_field_for(&d.agar, d.w, d.h, &units);
    let reach = f.iter().filter(|&&v| v < u32::MAX).count();
    println!("  units' walking distance from {} sources: {:.1} ms, {reach} of {} blocks reached", units.len(), t.elapsed().as_secs_f64() * 1e3, f.len());
}
