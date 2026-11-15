//! Larder World W1: does a player's wall change what the mould takes?
//! (`Reports/larder-world-roadmap-2026-11-11.md`; the expander review
//! named walls the first tool because a salt wall across the leader's road
//! was the one tool measured to turn skirmish games.)
//!
//! Big dish, rectangular world, finite crystals, the mould starting as a
//! blob on the home crystal (left). At 5 s one wall of fixed length goes
//! in: `none`, `road` (across the line from home to the nearest
//! untouched crystals, 6 dish cells ahead of the mould's front there) or `random` (same
//! length and width, keyed position and angle). Deciding measure, named
//! before the run: charge drained from the non-home crystals by 30 s, paired
//! by seed, road against none and against random (sign test).
//!
//! Breakable walls (owner, 11-11 15:24Z) add two arms: `ring` (at 5 s a
//! closed wall round the nearest crystal the mould has not reached; when
//! the mould first eats from it) and `cage` (at the start a closed wall
//! round the whole home blob, the hardest the mould can press; when the
//! first cell breaks and how many walkers are outside at the end).
//!
//!   cargo run --release --example worldwall -- seeds=1-8 [scale=2 walkers=20000 seconds=30]
//!     [arms=none,road,random,ring,cage wall_hp= wall_bite= wall_scent= crystal_flow= look=DIR]

use slime_mould::mould::big::{BigDish, BigParams};

fn main() {
    let mut seeds: Vec<u64> = (1..=8).collect();
    let (mut scale, mut walkers, mut seconds, mut threads, mut look) = (2usize, 20_000usize, 30u32, 4usize, None::<String>);
    let mut arms: Vec<String> = vec!["none".into(), "road".into(), "random".into()];
    // Defaults follow `BigParams::today`.
    let t = BigParams::today();
    let (mut wall_hp, mut wall_bite, mut crystal_flow, mut wall_scent) = (t.wall_hp, t.wall_bite, t.crystal_flow, t.wall_scent);
    for a in std::env::args().skip(1) {
        let (k, v) = a.split_once('=').expect("key=value");
        match k {
            "seeds" => {
                let (a, b) = v.split_once('-').unwrap_or((v, v));
                seeds = (a.parse().unwrap()..=b.parse().unwrap()).collect();
            }
            "scale" => scale = v.parse().unwrap(),
            "walkers" => walkers = v.parse().unwrap(),
            "seconds" => seconds = v.parse().unwrap(),
            "threads" => threads = v.parse().unwrap(),
            "look" => look = Some(v.into()),
            "arms" => arms = v.split(',').map(String::from).collect(),
            "wall_hp" => wall_hp = v.parse().unwrap(),
            "wall_bite" => wall_bite = v.parse().unwrap(),
            "crystal_flow" => crystal_flow = v.parse().unwrap(),
            "wall_scent" => wall_scent = v.parse().unwrap(),
            _ => panic!("unknown {k}"),
        }
    }
    println!("worldwall scale {scale} walkers {walkers} seconds {seconds} wall_hp {wall_hp} wall_bite {wall_bite} crystal_flow {crystal_flow} wall_scent {wall_scent}");
    for &seed in &seeds {
        let mut out = Vec::new();
        for arm in arms.iter().map(String::as_str) {
            let p = BigParams {
                scale,
                substeps: scale,
                walkers,
                scent_offset: 36.0,
                scent_coarse: false,
                piles: 23,
                rect: true,
                crystal_charge: 1.0,
                crystal_eat: 0.005,
                wall_hp,
                wall_bite,
                crystal_flow,
                wall_scent,
                seed,
                ..BigParams::today()
            };
            let mut d = BigDish::new(p);
            d.threads = threads;
            let ticks_per_s = slime_mould::mould::STEPS_PER_SECOND as u64;
            let mut wall_cells = 0;
            // `ring`: a closed wall round a crystal, and the game second
            // the mould first eats from it. The crystal is picked at 5 s, the
            // same way in every arm (the arms share their first 5 s): the
            // one nearest home that is still full and has no walker within
            // the ring (0 if there is none: the seed is skipped for `ring`).
            // (A first try took the nearest crystal at the start; the home
            // blob already covered it. A second fell back to ringing home.)
            let mut ring_pile = 0usize;
            let mut first_bite: Option<f32> = None;
            // `cage`: at the start, a ring round the whole home blob (its
            // farthest walker plus 6 dish cells), so the mould presses on a
            // wall as hard as it can; the game second the first cell breaks.
            let mut first_break: Option<f32> = None;
            let mut outside_every_10s: Vec<usize> = Vec::new();
            let mut cage_r = 0.0f32;
            if arm == "cage" {
                let (hx, hy) = d.piles[0];
                let s = scale as f32;
                cage_r = d.walkers.iter().map(|&(x, y, _)| (x / s - hx).hypot(y / s - hy)).fold(0.0f32, f32::max) + 6.0;
                let n = 48;
                for q in 0..n {
                    let (a0, a1) = (q as f32 / n as f32 * std::f32::consts::TAU, (q + 1) as f32 / n as f32 * std::f32::consts::TAU);
                    let r = cage_r;
                    wall_cells += d.build_wall(hx + r * a0.cos(), hy + r * a0.sin(), hx + r * a1.cos(), hy + r * a1.sin(), 2.0);
                }
            }
            for t in 0..seconds as u64 * ticks_per_s {
                if t == 5 * ticks_per_s {
                    let (hx, hy) = d.piles[0];
                    let (s, r) = (scale as f32, d.p.pile_r + 6.0);
                    ring_pile = (1..d.piles.len())
                        .filter(|&i| d.crystal_left(i) >= 0.999)
                        .filter(|&i| !d.walkers.iter().any(|&(x, y, _)| (x / s - d.piles[i].0).hypot(y / s - d.piles[i].1) < r))
                        .min_by(|&a, &b| (d.piles[a].0 - hx).hypot(d.piles[a].1 - hy).total_cmp(&(d.piles[b].0 - hx).hypot(d.piles[b].1 - hy)))
                        .unwrap_or(0);
                }
                if t == 5 * ticks_per_s && arm == "ring" && ring_pile == 0 {
                    break;
                }
                if t == 5 * ticks_per_s && arm == "ring" {
                    let (cx, cy) = d.piles[ring_pile];
                    let r = d.p.pile_r + 4.0;
                    let n = 16;
                    for q in 0..n {
                        let (a0, a1) = (q as f32 / n as f32 * std::f32::consts::TAU, (q + 1) as f32 / n as f32 * std::f32::consts::TAU);
                        wall_cells += d.build_wall(cx + r * a0.cos(), cy + r * a0.sin(), cx + r * a1.cos(), cy + r * a1.sin(), 2.0);
                    }
                } else if t == 5 * ticks_per_s && (arm == "road" || arm == "random") {
                    let (hx, hy) = d.piles[0];
                    let (len, width) = (60.0f32, 3.0f32);
                    let (cx, cy, ang) = if arm == "road" {
                        // Toward the mean of the three nearest other crystals.
                        let mut o: Vec<(f32, f32)> = d.piles[1..].to_vec();
                        o.sort_by(|a, b| (a.0 - hx).hypot(a.1 - hy).total_cmp(&(b.0 - hx).hypot(b.1 - hy)));
                        let (mx, my) = o.iter().take(3).fold((0.0, 0.0), |s, p| (s.0 + p.0 / 3.0, s.1 + p.1 / 3.0));
                        let a = (my - hy).atan2(mx - hx);
                        // Just ahead of the mould's front on that line: the
                        // farthest walker within 10 dish cells of the line,
                        // plus 6. (A first run put the wall 12 cells from
                        // home, inside the 20k-walker blob, and it did
                        // nothing: road against none 5 up, 3 down of 8.)
                        let s = scale as f32;
                        let (ux, uy) = (a.cos(), a.sin());
                        let front = d.walkers.iter().map(|&(x, y, _)| ((x / s - hx) * ux + (y / s - hy) * uy, ((x / s - hx) * -uy + (y / s - hy) * ux).abs()))
                            .filter(|&(_, off)| off < 10.0).map(|(along, _)| along).fold(0.0f32, f32::max);
                        let r = front + 6.0;
                        (hx + r * ux, hy + r * uy, a + std::f32::consts::FRAC_PI_2)
                    } else {
                        let u = |k: u64| ((seed.wrapping_mul(6364136223846793005).wrapping_add(k * 1442695040888963407)) >> 40) as f32 / (1u64 << 24) as f32;
                        (20.0 + u(1) * 216.0, 20.0 + u(2) * 120.0, u(3) * std::f32::consts::TAU)
                    };
                    let (dx, dy) = (ang.cos() * len / 2.0, ang.sin() * len / 2.0);
                    wall_cells = d.build_wall(cx - dx, cy - dy, cx + dx, cy + dy, width);
                }
                d.tick();
                if arm == "cage" && (t + 1) % (10 * ticks_per_s) == 0 {
                    let ((hx, hy), s) = (d.piles[0], scale as f32);
                    outside_every_10s.push(d.walkers.iter().filter(|&&(x, y, _)| (x / s - hx).hypot(y / s - hy) > cage_r + 1.0).count());
                }
                if first_break.is_none() && d.broken > 0 {
                    first_break = Some(d.seconds());
                }
                if ring_pile > 0 && first_bite.is_none() && d.crystal_left(ring_pile) < 0.999 {
                    first_bite = Some(d.seconds());
                }
            }
            let others: f32 = (1..d.piles.len()).map(|i| d.pile_full[i] - d.pile_charge[i]).sum();
            let full: f32 = d.pile_full[1..].iter().sum();
            if arm == "cage" {
                println!(
                    "seed {seed} cage   wall {wall_cells:5} first break {}  broken {} of {wall_cells}  wall bites {}  walkers outside at 10 s, 20 s.. {:?}  hash {:016x}",
                    first_break.map_or("never".to_string(), |s| format!("{s:.1}s")),
                    d.broken,
                    d.wall_hits,
                    outside_every_10s,
                    d.hash()
                );
                if let Some(dir) = &look {
                    std::fs::create_dir_all(dir).unwrap();
                    d.image(1).save(format!("{dir}/wall-s{seed}-cage.png")).unwrap();
                }
                continue;
            }
            if arm == "ring" && ring_pile == 0 {
                println!("seed {seed} ring   no crystal unreached at 5 s: skipped");
                continue;
            }
            println!(
                "seed {seed} {arm:6} wall {wall_cells:5} drained {:.4} of others  home left {:.3}  ring pile bitten {}  left {:.3}  broken {}  wall bites {}  hash {:016x}",
                others / full,
                d.crystal_left(0),
                first_bite.map_or("never".to_string(), |s| format!("{s:.1}s")),
                d.crystal_left(ring_pile),
                d.broken,
                d.wall_hits,
                d.hash()
            );
            if let Some(dir) = &look {
                std::fs::create_dir_all(dir).unwrap();
                d.image(1).save(format!("{dir}/wall-s{seed}-{arm}.png")).unwrap();
            }
            out.push(others / full);
        }
        if out.len() == 3 {
            println!("seed {seed} road-none {:+.4} road-random {:+.4}", out[1] - out[0], out[1] - out[2]);
        }
    }
}
