//! **The retreat probe** (2026-11-13, the game designer's follow-up on the
//! panic wind-up, `BigParams::panic_creep`): can a squad that pulls back
//! when a cut-off piece starts to lean get clear, or does the charge always
//! catch it? "If it gets clear on most seeds, a player who reacts is
//! rewarded."
//!
//! ```text
//! cargo run --release --example retreatprobe -- seeds=1-4 [grow=90 keys=panic_dash:15+...] [look=DIR look_d0=24]
//! ```
//!
//! Grows the CPU race world with shape C's keys (`capacity` 50,000,
//! `upkeep` 0.0002, `energy_flow` 0.05, `wither` 0.01, `panic` 2, plus
//! `keys=`) for `grow` game seconds. Then, per seed, it picks a piece: a
//! walker at least 40 fine cells from every crystal's centre with 40 to 600 walkers
//! within 10 fine cells, those walkers kept and every other walker killed,
//! so no news of food reaches them; their news is set to run out on the next
//! step, so the lean starts at 0 s. A unit (a point, as `BigDish::threats`
//! has it) starts `D0` fine cells beyond the piece's edge, on the side
//! toward the dish's middle, and either stands or walks straight away at a
//! soldier's pace (`Rules::speed` 10 dish cells a second) from a reaction
//! time. **Deciding measure, named before the run:** caught or clear, where
//! caught is any living walker within a soldier's touch (1.5 dish cells)
//! within 10 s. The unit is not slowed by rock or by walkers, does not shoot
//! and the walkers do not die of its shots, so this is the charge's speed
//! against the squad's and nothing else.
use slime_mould::mould::big::BigDish;
use slime_mould::mould::larder_world::race_params;
use slime_mould::mould::tiers::set_key;

const SCALE: usize = 2;
const TPS: f32 = 15.0;
const SPEED: f32 = 10.0;
const TOUCH: f32 = 1.5;

fn main() {
    let mut seeds: Vec<u64> = (1..=4).collect();
    let (mut grow, mut keys) = (90.0f32, Vec::<(String, String)>::new());
    // `look=DIR`: frames of every seed's runs at `look_d0`, one a tick,
    // the dish as `BigDish::image` draws it cropped round the piece and the
    // unit as a white square with its touch as a grey ring (marks for the
    // reader, not the simulation).
    let (mut look, mut look_d0) = (None::<String>, 24.0f32);
    for a in std::env::args().skip(1) {
        let (k, v) = a.split_once('=').expect("key=value");
        match k {
            "seeds" => {
                let (a, b) = v.split_once('-').unwrap_or((v, v));
                seeds = (a.parse().unwrap()..=b.parse().unwrap()).collect();
            }
            "grow" => grow = v.parse().unwrap(),
            "look" => look = Some(v.to_string()),
            "look_d0" => look_d0 = v.parse().unwrap(),
            "keys" => keys = v.split('+').map(|kv| kv.split_once(':').map(|(a, b)| (a.to_string(), b.to_string())).expect("keys=k:v+k:v")).collect(),
            _ => panic!("unknown {k}"),
        }
    }
    let base: Vec<(String, String)> = [("capacity", "50000"), ("energy_flow", "0.05"), ("wither", "0.01"), ("panic", "2")].iter().map(|(a, b)| (a.to_string(), b.to_string())).collect();
    println!("retreatprobe: race world scale {SCALE}, 25000 walkers, grow {grow} s, keys {:?} then {keys:?}", base);
    let s = SCALE as f32;
    let (touch, step) = (TOUCH * s, SPEED * s / TPS);
    let mut tally = std::collections::BTreeMap::<(u32, String), (u32, u32)>::new();
    for &seed in &seeds {
        let mut p = race_params(seed, SCALE, 25_000);
        p.upkeep = 0.0002;
        for (k, v) in base.iter().chain(&keys) {
            set_key(&mut p, k, v).unwrap();
        }
        let mut d = BigDish::new(p);
        d.threads = 2;
        for _ in 0..(grow * TPS) as u32 {
            d.tick();
        }
        // The piece.
        let live: Vec<usize> = (0..d.walkers.len()).filter(|&k| d.lives(k)).collect();
        // Pile centres in fine cells (`BigDish::piles` holds dish cells).
        let centres: Vec<(f32, f32)> = (0..d.piles.len()).map(|j| d.pile_centre(j)).map(|c| ((c & 0xFFFF) as f32 + 0.5, (c >> 16) as f32 + 0.5)).collect();
        let far = |x: f32, y: f32| centres.iter().all(|&(px, py)| (px - x).hypot(py - y) >= 40.0);
        let near = |k0: usize| live.iter().copied().filter(|&k| (d.walkers[k].0 - d.walkers[k0].0).hypot(d.walkers[k].1 - d.walkers[k0].1) <= 10.0).collect::<Vec<_>>();
        let mut rng = seed.wrapping_mul(0x9E37_79B9_7F4A_7C15) | 1;
        let mut piece = Vec::new();
        for _ in 0..20_000 {
            rng ^= rng << 13;
            rng ^= rng >> 7;
            rng ^= rng << 17;
            let k0 = live[(rng % live.len() as u64) as usize];
            if !far(d.walkers[k0].0, d.walkers[k0].1) {
                continue;
            }
            let n = near(k0);
            if (40..=600).contains(&n.len()) {
                piece = n;
                break;
            }
        }
        assert!(!piece.is_empty(), "seed {seed}: no piece found");
        let keep: std::collections::HashSet<usize> = piece.iter().copied().collect();
        for &k in &live {
            if !keep.contains(&k) {
                d.kill(k);
            }
        }
        let lag = d.p.wither_lag as u64;
        for &k in &piece {
            d.news[k] = d.steps + 1 - lag;
        }
        let n = piece.len() as f32;
        let (cx, cy) = (piece.iter().map(|&k| d.walkers[k].0).sum::<f32>() / n, piece.iter().map(|&k| d.walkers[k].1).sum::<f32>() / n);
        let edge = piece.iter().map(|&k| (d.walkers[k].0 - cx).hypot(d.walkers[k].1 - cy)).fold(0.0f32, f32::max);
        let a = (d.h as f32 / 2.0 - cy).atan2(d.w as f32 / 2.0 - cx);
        let (ux, uy) = (a.cos(), a.sin());
        println!("seed {seed}: piece of {} walkers round ({cx:.0}, {cy:.0}), edge {edge:.1} fine cells; away along ({ux:.2}, {uy:.2})", piece.len());
        for d0 in [16.0f32, 24.0, 32.0] {
            for (name, react) in [("stands", f32::INFINITY), ("pulls back at 0 s", 0.0), ("pulls back at 0.5 s", 0.5)] {
                let mut e = d.clone();
                let (mut x, mut y) = (cx + ux * (edge + d0), cy + uy * (edge + d0));
                let (mut caught, mut least) = (None, f32::MAX);
                let film = look.as_ref().filter(|_| d0 == look_d0).map(|dir| format!("{dir}/s{seed}-{}", name.replace(' ', "-").replace('.', "")));
                if let Some(f) = &film {
                    std::fs::create_dir_all(f).unwrap();
                }
                let (fx, fy) = ((cx + ux * (edge + d0) * 0.5) as i64, (cy + uy * (edge + d0) * 0.5) as i64);
                for t in 0..(10.0 * TPS) as u32 {
                    let secs = t as f32 / TPS;
                    if secs >= react {
                        x = (x + ux * step).clamp(0.0, e.w as f32 - 1.0);
                        y = (y + uy * step).clamp(0.0, e.h as f32 - 1.0);
                    }
                    e.threats = vec![(x.floor() as i64, y.floor() as i64)];
                    e.tick();
                    let m = (0..e.walkers.len()).filter(|&k| e.lives(k)).map(|k| (e.walkers[k].0 - x).hypot(e.walkers[k].1 - y)).fold(f32::MAX, f32::min);
                    least = least.min(m);
                    if let Some(f) = &film {
                        frame(&e, (fx, fy), (x, y), touch, &format!("{f}/f{t:04}.png"));
                    }
                    if m <= touch && caught.is_none() {
                        caught = Some(secs + 1.0 / TPS);
                    }
                    // A film runs on 4 s past the catch.
                    if caught.is_some_and(|c| film.is_none() || secs >= c + 4.0) {
                        break;
                    }
                }
                let scared = (0..e.walkers.len()).filter(|&k| e.lives(k) && e.panics(k)).count();
                println!(
                    "  D0 {d0:>2}: {name:<20} {}; closest {least:.1}; {scared} scared at the end",
                    caught.map_or("clear".to_string(), |t| format!("caught at {t:.2} s"))
                );
                let c = tally.entry((d0 as u32, name.to_string())).or_default();
                if caught.is_some() {
                    c.0 += 1;
                } else {
                    c.1 += 1;
                }
            }
        }
    }
    println!("by distance and reaction (caught / clear over {} seeds):", seeds.len());
    for ((d0, name), (c, k)) in tally {
        println!("  D0 {d0:>2} {name:<20} caught {c}, clear {k}");
    }
}

/// One frame: 160 x 120 fine cells round `at`, four pixels a cell.
fn frame(d: &BigDish, at: (i64, i64), unit: (f32, f32), touch: f32, path: &str) {
    let img = d.image(1);
    let (w, h) = (160i64, 120i64);
    let k = 4u32;
    let mut out = image::RgbaImage::from_pixel(w as u32 * k, h as u32 * k, image::Rgba([17, 26, 31, 255]));
    for y in 0..h {
        for x in 0..w {
            let (sx, sy) = (at.0 - w / 2 + x, at.1 - h / 2 + y);
            if sx < 0 || sy < 0 || sx >= img.width() as i64 || sy >= img.height() as i64 {
                continue;
            }
            let px = *img.get_pixel(sx as u32, sy as u32);
            for dy in 0..k {
                for dx in 0..k {
                    out.put_pixel(x as u32 * k + dx, y as u32 * k + dy, px);
                }
            }
        }
    }
    let (ux, uy) = ((unit.0 - (at.0 - w / 2) as f32) * k as f32, (unit.1 - (at.1 - h / 2) as f32) * k as f32);
    for i in 0..64 {
        let a = i as f32 / 64.0 * std::f32::consts::TAU;
        let (rx, ry) = (ux + a.cos() * touch * k as f32, uy + a.sin() * touch * k as f32);
        if rx >= 0.0 && ry >= 0.0 && (rx as u32) < out.width() && (ry as u32) < out.height() {
            out.put_pixel(rx as u32, ry as u32, image::Rgba([170, 180, 185, 255]));
        }
    }
    for dy in -5i32..=5 {
        for dx in -5i32..=5 {
            let (px, py) = (ux as i32 + dx, uy as i32 + dy);
            if px >= 0 && py >= 0 && (px as u32) < out.width() && (py as u32) < out.height() {
                out.put_pixel(px as u32, py as u32, image::Rgba([255, 255, 255, 255]));
            }
        }
    }
    out.save(path).unwrap();
}
