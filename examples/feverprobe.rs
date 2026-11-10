//! **Two Fevers version 3 checks** (`Reports/two-fevers-2026-11-10/v3-design.md`,
//! *Risks*): the three cheap tests named before any v3 build.
//!
//! ```text
//! cargo run --release --example feverprobe -- trail seeds=1-4
//! cargo run --release --example feverprobe -- gap seeds=1-16 [tube=T] [look=DIR]
//! cargo run --release --example feverprobe -- speed seeds=1-16 [tube=T]
//! cargo run --release --example feverprobe -- call seeds=1-16 [w=W] [reach=R] [look=DIR]
//! cargo run --release --example feverprobe -- pinch seeds=1-16 [look=DIR]
//! ```
//!
//! `trail`: trail under walkers after the warm-up (what "tube" is).
//! `gap`: a 4-cell band down the middle has its trail wiped every step (a
//! cut); seed on the left, count carriers and owned walkers right of the
//! band after 20 s, fever by tube (`tube=`) against the shipped spread.
//! `speed`: one seed in the middle; for every walker caught in 15 s, its
//! distance over the time it took, split by the trail where it was caught.
//! `call`: Ember owns the walkers left of x = 100; one call on the pile at
//! (162, 51); walkers Ember owns within 10 cells of it, and how far right
//! its body reaches, at +5/+10/+20 s, against the same clone with no call.
//! `pinch`: Ember owns the host; a pinch on the thickest tube cell away
//! from piles; trail there each second for 8 s, against a clone without.
use slime_mould::mould::fever::{Fevers, Phase, Rules, EMBER, H, STEPS_PER_SECOND, W};

fn arg<'a>(args: &'a [String], k: &str) -> Option<&'a str> {
    args.iter()
        .find_map(|a| a.strip_prefix(k).and_then(|r| r.strip_prefix('=')))
}

fn seeds(s: Option<&str>) -> Vec<u64> {
    let s = s.unwrap_or("1-16");
    match s.split_once('-') {
        Some((a, b)) => (a.parse().unwrap()..=b.parse().unwrap()).collect(),
        None => s.split(',').map(|v| v.parse().unwrap()).collect(),
    }
}

fn save(g: &Fevers, path: &str) {
    let mut buf = vec![0u8; W * H * 4];
    g.paint(&mut buf);
    let img = image::RgbaImage::from_raw(W as u32, H as u32, buf).unwrap();
    let img = image::imageops::resize(&img, W as u32 * 3, H as u32 * 3, image::imageops::FilterType::Nearest);
    img.save(path).unwrap();
}

fn warm(seed: u64, r: Rules) -> Fevers {
    let mut g = Fevers::new(seed, r);
    while g.phase() == Phase::Warm {
        g.step();
    }
    g
}

fn secs(g: &mut Fevers, s: f32, mut each: impl FnMut(&mut Fevers)) {
    for _ in 0..(s * STEPS_PER_SECOND as f32) as u32 {
        each(g);
        g.step();
    }
}

fn pct(v: &mut [f32], p: f32) -> f32 {
    v.sort_by(f32::total_cmp);
    if v.is_empty() {
        0.0
    } else {
        v[((v.len() - 1) as f32 * p) as usize]
    }
}

fn main() {
    let args: Vec<String> = std::env::args().skip(1).collect();
    let what = args.first().map(String::as_str).unwrap_or("trail");
    let tube: f32 = arg(&args, "tube").map_or(6.0, |v| v.parse().unwrap());
    let cap: f32 = arg(&args, "cap").map_or(1.0, |v| v.parse().unwrap());
    let look = arg(&args, "look");
    if let Some(d) = look {
        std::fs::create_dir_all(d).unwrap();
    }
    match what {
        "trail" => {
            for seed in seeds(arg(&args, "seeds")) {
                let g = warm(seed, Rules::default());
                let mut v: Vec<f32> = g
                    .host
                    .iter()
                    .map(|h| g.trail[h.y as usize * W + h.x as usize])
                    .collect();
                let mut all: Vec<f32> = g.trail.iter().copied().filter(|&t| t > 0.5).collect();
                println!(
                    "seed {seed}: trail under walkers p10 {:.1} p50 {:.1} p90 {:.1}; cells above 0.5: {} (p50 {:.1})",
                    pct(&mut v, 0.1),
                    pct(&mut v, 0.5),
                    pct(&mut v, 0.9),
                    all.len(),
                    pct(&mut all, 0.5)
                );
            }
        }
        "gap" => {
            println!("seed  by-tube: right-carriers right-owned | spread: right-carriers right-owned");
            for seed in seeds(arg(&args, "seeds")) {
                let mut line = format!("{seed}");
                for (k, t) in [tube, 0.0].into_iter().enumerate() {
                    let r = Rules {
                        tube_fever: t,
                        tube_cap: cap,
                        match_seconds: 600.0,
                        ..Rules::default()
                    };
                    let mut g = warm(seed, r);
                    let cut = |g: &mut Fevers| {
                        for y in 0..H {
                            for x in 126..130 {
                                g.trail[y * W + x] = 0.0;
                            }
                        }
                    };
                    secs(&mut g, 3.0, cut);
                    // Seed on the walker nearest (80, 80).
                    let w = *g
                        .host
                        .iter()
                        .min_by(|p, q| {
                            (p.x - 80.0)
                                .hypot(p.y - 80.0)
                                .total_cmp(&(q.x - 80.0).hypot(q.y - 80.0))
                        })
                        .unwrap();
                    g.spores[EMBER] = 1;
                    g.seed(EMBER, w.x, w.y);
                    secs(&mut g, 20.0, cut);
                    let rc = g
                        .host
                        .iter()
                        .filter(|h| h.x > 132.0 && h.carrying() == Some(EMBER))
                        .count();
                    let ro = g
                        .host
                        .iter()
                        .filter(|h| h.x > 132.0 && h.owner() == Some(EMBER))
                        .count();
                    let lo = g
                        .host
                        .iter()
                        .filter(|h| h.x < 124.0 && h.owner() == Some(EMBER))
                        .count();
                    line += &format!(
                        "{}{rc:5} {ro:5} (left owned {lo:5})",
                        if k == 0 { "   " } else { " | " }
                    );
                    if let (Some(d), 0) = (look, k) {
                        save(&g, &format!("{d}/gap-{seed}.png"));
                    }
                }
                println!("{line}");
            }
        }
        "speed" => {
            for t in [tube, 0.0] {
                let (mut thick, mut thin, mut reach) = (Vec::new(), Vec::new(), Vec::new());
                for seed in seeds(arg(&args, "seeds")) {
                    let r = Rules {
                        tube_fever: t,
                        tube_cap: cap,
                        match_seconds: 600.0,
                        ..Rules::default()
                    };
                    let mut g = warm(seed, r);
                    let w = *g
                        .host
                        .iter()
                        .min_by(|p, q| {
                            (p.x - 128.0)
                                .hypot(p.y - 80.0)
                                .total_cmp(&(q.x - 128.0).hypot(q.y - 80.0))
                        })
                        .unwrap();
                    g.spores[EMBER] = 1;
                    g.seed(EMBER, w.x, w.y);
                    let t0 = g.t;
                    let mut caught = vec![false; g.host.len()];
                    for (k, h) in g.host.iter().enumerate() {
                        caught[k] = h.carrying().is_some();
                    }
                    let mut far: f32 = 0.0;
                    secs(&mut g, 15.0, |g| {
                        for (k, h) in g.host.iter().enumerate() {
                            if !caught[k] && h.carrying().is_some() {
                                caught[k] = true;
                                let d = (h.x - w.x).hypot(h.y - w.y);
                                let dt = (g.t - t0) as f32 / STEPS_PER_SECOND as f32;
                                let tr = g.trail[h.y as usize * W + h.x as usize];
                                if d > 10.0 {
                                    if tr >= 7.0 {
                                        thick.push(d / dt);
                                    } else if tr < 4.0 {
                                        thin.push(d / dt);
                                    }
                                }
                                far = far.max(d);
                            }
                        }
                    });
                    reach.push(far);
                }
                println!(
                    "tube_fever {t}: cells/s to catch, thick tube (trail>=7) median {:.1} (n {}), thin (<4) median {:.1} (n {}); farthest catch median {:.0}",
                    pct(&mut thick, 0.5),
                    thick.len(),
                    pct(&mut thin, 0.5),
                    thin.len(),
                    pct(&mut reach, 0.5)
                );
            }
        }
        "call" => {
            let w: f32 = arg(&args, "w").map_or(30.0, |v| v.parse().unwrap());
            let reach: f32 = arg(&args, "reach").map_or(60.0, |v| v.parse().unwrap());
            let (px, py) = (162.0f32, 51.0f32);
            println!("seed  call: near@5 near@10 near@20 p90x@20 | none: near@5 near@10 near@20 p90x@20");
            let mut more = 0;
            let list = seeds(arg(&args, "seeds"));
            for &seed in &list {
                let mut line = format!("{seed}");
                let mut near20 = [0usize; 2];
                for (k, on) in [true, false].into_iter().enumerate() {
                    let r = Rules {
                        call_weight: if on { w } else { 0.0 },
                        call_reach: reach,
                        match_seconds: 600.0,
                        ..Rules::default()
                    };
                    let mut g = warm(seed, r);
                    for h in g.host.iter_mut() {
                        if h.x < 100.0 {
                            h.own = 1;
                        }
                    }
                    g.set_calls(EMBER, &[(px, py)]);
                    let near = |g: &Fevers| {
                        g.host
                            .iter()
                            .filter(|h| h.owner() == Some(EMBER) && (h.x - px).hypot(h.y - py) <= 10.0)
                            .count()
                    };
                    let mut out = Vec::new();
                    for (n, s) in [(5, 5.0), (10, 5.0), (20, 10.0)] {
                        secs(&mut g, s, |_| {});
                        out.push(near(&g));
                        if let (Some(d), 0) = (look, k) {
                            save(&g, &format!("{d}/call-{seed}-{n}s.png"));
                        }
                    }
                    let mut xs: Vec<f32> = g
                        .host
                        .iter()
                        .filter(|h| h.owner() == Some(EMBER))
                        .map(|h| h.x)
                        .collect();
                    near20[k] = out[2];
                    // Tube left: cells of trail 2 or more, and the owned
                    // walkers within 20 cells of the call (a collapse piles
                    // them there).
                    let tube = g.trail.iter().filter(|&&t| t >= 2.0).count();
                    let heap = g
                        .host
                        .iter()
                        .filter(|h| h.owner() == Some(EMBER) && (h.x - px).hypot(h.y - py) <= 20.0)
                        .count();
                    line += &format!(
                        "{}{:5} {:5} {:5} {:5.0} tube {:5} heap {:5}",
                        if k == 0 { "   " } else { " | " },
                        out[0],
                        out[1],
                        out[2],
                        pct(&mut xs, 0.9),
                        tube,
                        heap
                    );
                }
                more += (near20[0] > near20[1]) as usize;
                println!("{line}");
            }
            println!("call more owned walkers on the pile at +20 s: {more}/{}", list.len());
        }
        "pinch" => {
            println!("seed  trail at the pinch each second 0..8 | same without the pinch");
            let list = seeds(arg(&args, "seeds"));
            let mut cut = 0;
            for &seed in &list {
                let r = Rules {
                    match_seconds: 600.0,
                    ..Rules::default()
                };
                let mut g = warm(seed, r);
                for h in g.host.iter_mut() {
                    h.own = 1;
                }
                // The thickest cell at least 15 cells from every pile.
                let far = |i: usize| {
                    g.piles
                        .iter()
                        .all(|p| ((i % W) as f32 - p.x).hypot((i / W) as f32 - p.y) > 15.0)
                };
                let i = (0..W * H)
                    .filter(|&i| far(i))
                    .max_by(|&a, &b| g.trail[a].total_cmp(&g.trail[b]))
                    .unwrap();
                let (x, y) = ((i % W) as f32 + 0.5, (i / W) as f32 + 0.5);
                let mut c = g.clone();
                g.pinch(EMBER, x, y);
                let mut a = Vec::new();
                let mut b = Vec::new();
                for sec in 0..=8 {
                    // The mean over the pinch's inner disc (radius 2).
                    let m = |g: &Fevers| {
                        let mut s = 0.0;
                        let mut n = 0.0;
                        for dy in -2i32..=2 {
                            for dx in -2i32..=2 {
                                if dx * dx + dy * dy <= 4 {
                                    s += g.trail[(i as i32 + dy * W as i32 + dx) as usize];
                                    n += 1.0;
                                }
                            }
                        }
                        s / n
                    };
                    a.push(m(&g));
                    b.push(m(&c));
                    if let (Some(d), 5) = (look, sec) {
                        save(&g, &format!("{d}/pinch-{seed}.png"));
                        save(&c, &format!("{d}/pinch-{seed}-none.png"));
                    }
                    if sec < 8 {
                        secs(&mut g, 1.0, |_| {});
                        secs(&mut c, 1.0, |_| {});
                    }
                }
                cut += (a[5] < 1.0 && a[6] < 1.0) as usize;
                let f = |v: &[f32]| v.iter().map(|t| format!("{t:5.1}")).collect::<Vec<_>>().join("");
                println!("{seed} at ({x:.0},{y:.0}): {} | {}", f(&a), f(&b));
            }
            println!("pinch below trail 1.0 at +5 s and +6 s: {cut}/{}", list.len());
        }
        _ => panic!("unknown probe {what}"),
    }
}
