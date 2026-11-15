//! **The squad toolkit, tool by tool** (`mould::kit`, 2026-11-12): each
//! tool against no tool (and against the same tool without the part that
//! is new) on the CPU race world, paired by seed.
//!
//! ```text
//! cargo run --release --example kitprobe -- test=blast seeds=1-8 [warm=60 threads=4 look=DIR]
//! ```
//!
//! Every test grows the race world's mould for `warm` game seconds with the
//! squad on Hold Fire, so nothing but the tool touches it, then uses the
//! tool at a point picked from the mould (below) and follows it.
//!
//! **Deciding measures, named before the runs:**
//! - `blast` (arms none, blast without crater, blast; at the densest 8 x 8
//!   block): walkers within the blast radius 10 s after, crater against no
//!   crater. Also at 2, 5 and 20 s.
//! - `fire` (the owner, 04:39Z: "A group of soldiers shooting the slime
//!   should start carving out a bigger and bigger hole that doesn't refill
//!   while being shot at"; arms none, fire, fire with a crater at each hit):
//!   four soldiers placed 12 dish cells from the densest block fire at it
//!   (Free stance, free shots) for 20 s; the hole is the agar cells within
//!   12 dish cells of the block with no walker; deciding: hole at 20 s,
//!   crater against none; then 10 s after fire stops.
//! - `germ` (arms none, steering-only germ with `sick_upkeep` 1, germ):
//!   living walkers 30 s after, germ against none and against steering-only.
//! - `gas` (arms none, gas, gas with a blast 10 dish cells off at +5 s):
//!   walkers asleep 10 s after, gas against gas-and-noise; charge eaten from
//!   the crystals under the gas over 20 s, gas against none.
//! - `cord` (arms none, cord without crater, cord; a 30-dish-cell line
//!   across the busiest arm 20-40 dish cells from the core, square to it):
//!   trail on the line's cells 10 s after (does the cut stay open?), crater
//!   against none; walkers beyond the line at 30 s.
//! - `mine` (arms none, three mines on the line from the core to the
//!   larder, a third, a half and two thirds of the way): walkers within 20
//!   dish cells of the larder at +60 s, mines against none; kills and mines
//!   gone off.
//!
//! Sign test on the pairs (two-sided); the tests are listed with their p,
//! and Holm is applied over every deciding comparison of the run.
use slime_mould::mould::big::{BigDish, LIFE_DORMANT};
use slime_mould::mould::kit::{Kit, KitRules, Tool};
use slime_mould::mould::larder_world::{race_world, Cmd, Game, Order, Rules, Stance, World};

#[derive(Clone, Debug, Default)]
struct Out {
    /// Named measures, in order.
    m: Vec<(String, f64)>,
}

fn sign_p(w: usize, n: usize) -> f64 {
    if n == 0 {
        return 1.0;
    }
    let k = w.min(n - w);
    let mut c = 1.0f64;
    let mut tail = 0.0;
    for i in 0..=k {
        if i > 0 {
            c = c * (n - i + 1) as f64 / i as f64;
        }
        tail += c;
    }
    (2.0 * tail / 2f64.powi(n as i32)).min(1.0)
}

/// The centre (fine cells) of the 8 x 8 block with the most walkers,
/// among blocks whose centre passes `ok`.
fn densest(g: &Game<BigDish>, ok: impl Fn((f32, f32)) -> bool) -> (f32, f32) {
    let b = 8;
    let grid = g.world.mould_grid(b).unwrap();
    let gw = g.world.w.div_ceil(b);
    let mut best = (0u32, (0.0, 0.0));
    for (i, &n) in grid.iter().enumerate() {
        let c = (((i % gw) * b + b / 2) as f32, ((i / gw) * b + b / 2) as f32);
        if n > best.0 && ok(c) && g.world.open(c.0, c.1) {
            best = (n, c);
        }
    }
    best.1
}

fn walkers_within(d: &BigDish, p: (f32, f32), r: f32) -> usize {
    (0..d.walkers.len()).filter(|&k| d.lives(k) && (d.life.is_empty() || d.life[k] != slime_mould::mould::big::LIFE_DEAD)).filter(|&k| (d.walkers[k].0 - p.0).hypot(d.walkers[k].1 - p.1) <= r).count()
}

fn hole(d: &BigDish, p: (f32, f32), r: f32) -> usize {
    let occ = d.occupancy();
    let n = r.ceil() as i64;
    let mut c = 0;
    for dy in -n..=n {
        for dx in -n..=n {
            if ((dx * dx + dy * dy) as f32) > r * r {
                continue;
            }
            let (x, y) = (p.0 as i64 + dx, p.1 as i64 + dy);
            if x < 0 || y < 0 || x as usize >= d.w || y as usize >= d.h {
                continue;
            }
            let i = y as usize * d.w + x as usize;
            if d.agar[i] && occ[i] == 0 {
                c += 1;
            }
        }
    }
    c
}

fn secs(g: &mut Game<BigDish>, kit: &mut Kit, s: f32, mut each: impl FnMut(&mut Game<BigDish>, &mut Kit)) {
    let end = g.time() + s;
    while g.time() < end - 1e-4 {
        kit.frame(g);
        g.step();
        each(g, kit);
    }
}

fn picture(g: &Game<BigDish>, kit: &Kit, path: &str) {
    let mut img = g.world.image(1);
    let (w, h) = (img.width() as i32, img.height() as i32);
    let mut put = |x: i32, y: i32, c: [u8; 4]| {
        if x >= 0 && y >= 0 && x < w && y < h {
            img.put_pixel(x as u32, y as u32, image::Rgba(c));
        }
    };
    for s in g.soldiers.iter().filter(|s| s.alive) {
        for dy in -1..=1 {
            for dx in -1..=1 {
                put(s.x as i32 + dx, s.y as i32 + dy, [255, 255, 255, 255]);
            }
        }
    }
    for &(x, y) in &kit.mines {
        put(x as i32, y as i32, [255, 220, 0, 255]);
    }
    img.save(path).expect("save");
    // The walkers themselves, one dot a living walker (the graphics thread
    // draws the slime's thickness from living walkers): a hole the trail
    // picture blurs over shows here as empty ground.
    let d = &g.world;
    let mut occ = image::RgbaImage::from_pixel(d.w as u32, d.h as u32, image::Rgba([12, 16, 22, 255]));
    for (i, &n) in d.occupancy().iter().enumerate() {
        if !d.agar[i] {
            occ.put_pixel((i % d.w) as u32, (i / d.w) as u32, image::Rgba([40, 34, 30, 255]));
        } else if n > 0 {
            occ.put_pixel((i % d.w) as u32, (i / d.w) as u32, image::Rgba([240, 190, 110, 255]));
        }
    }
    if !d.scorch.is_empty() {
        for (i, &c) in d.scorch.iter().enumerate() {
            if c > 0 && d.occupancy()[i] == 0 && d.agar[i] {
                let v = (30.0 + 50.0 * (c as f32 / d.p.scorch_full as f32).min(1.0)) as u8;
                occ.put_pixel((i % d.w) as u32, (i / d.w) as u32, image::Rgba([v, 18, 14, 255]));
            }
        }
    }
    occ.save(path.replace(".png", "-occ.png")).expect("save");
}

fn game(seed: u64, warm: f32) -> (Game<BigDish>, Kit) {
    let world = race_world(seed, 2, 5000);
    let rules = Rules { price: 0.0, ..Rules::default() };
    let mut g = Game::new(world, rules, seed);
    // The squad stands by on Hold Fire and cannot die: only the tool acts.
    for k in 0..g.soldiers.len() {
        g.apply(Cmd::Stance(k, Stance::HoldFire));
        g.soldiers[k].hp = 1e9;
    }
    // Tool by tool: the tool, not the carry rule (that is kitplans' work),
    // so tools come from the bank wherever a soldier stands.
    let mut kit = Kit::new(KitRules { carry_max: 0, ..KitRules::default() });
    g.bank = 1e9;
    secs(&mut g, &mut kit, warm, |_, _| {});
    (g, kit)
}

/// Puts soldier 0 (alive) within reach of `p` so the tool may be used.
fn stand_near(g: &mut Game<BigDish>, p: (f32, f32)) {
    let k = (0..g.soldiers.len()).find(|&k| g.soldiers[k].alive).expect("a soldier");
    g.soldiers[k].x = p.0;
    g.soldiers[k].y = p.1;
    g.soldiers[k].hp = 1e9;
    g.apply(Cmd::Order(k, Order::Hold));
}

fn run(test: &str, arm: &str, seed: u64, warm: f32, look: Option<&str>) -> Out {
    let (mut g, mut kit) = game(seed, warm);
    // The crater's strength and fade, for the Larder thread's shot
    // craters (keep 0.1, full 30 asked 2026-11-12): KEEP= and FULL= in the
    // environment.
    if let Ok(v) = std::env::var("KEEP") {
        g.world.p.scorch_keep = v.parse().unwrap();
    }
    if let Ok(v) = std::env::var("FULL") {
        g.world.p.scorch_full = v.parse().unwrap();
    }
    // Germ resistance (round 2; amputation by steering, tried first, did
    // nothing: see `BigParams::germ_resist`): GERM_RESIST= and SICK_TRAIL=
    // in the environment.
    if let Ok(v) = std::env::var("GERM_RESIST") {
        g.world.p.germ_resist = v.parse().unwrap();
    }
    if let Ok(v) = std::env::var("SICK_TRAIL") {
        g.world.p.sick_trail = v.parse().unwrap();
    }
    if let Ok(v) = std::env::var("SICK_CATCH") {
        kit.rules.sick_catch = v.parse().unwrap();
    }
    let s = g.world.p.scale as f32;
    let mut o = Out::default();
    let snap = |g: &Game<BigDish>, kit: &Kit, tag: &str| {
        if let Some(dir) = look {
            picture(g, kit, &format!("{dir}/{test}-{arm}-s{seed}-{tag}.png"));
        }
    };
    let core = g.core;
    match test {
        "blast" => {
            let p = densest(&g, |_| true);
            let r = kit.rules.blast_r * s;
            if arm == "nocrater" {
                kit.rules.crater_secs = 0.0;
            }
            snap(&g, &kit, "0before");
            o.m.push(("before".into(), walkers_within(&g.world, p, r) as f64));
            if arm != "none" {
                stand_near(&mut g, p);
                assert!(kit.use_tool(&mut g, Tool::Blast(p)));
            }
            o.m.push(("killed".into(), kit.killed[0] as f64));
            let mut t = 0.0;
            for (at, tag) in [(2.0, "2s"), (5.0, "5s"), (10.0, "10s"), (20.0, "20s")] {
                secs(&mut g, &mut kit, at - t, |_, _| {});
                t = at;
                o.m.push((format!("in disc {tag}"), walkers_within(&g.world, p, r) as f64));
                o.m.push((format!("hole {tag}"), hole(&g.world, p, r) as f64));
                snap(&g, &kit, tag);
            }
        }
        "fire" => {
            let p = densest(&g, |_| true);
            let crater = arm == "crater";
            for k in 0..g.soldiers.len().min(4) {
                let a = k as f32 * std::f32::consts::FRAC_PI_2;
                let mut q = (p.0 + 12.0 * s * a.cos(), p.1 + 12.0 * s * a.sin());
                if !g.world.open(q.0, q.1) {
                    q = p;
                }
                g.soldiers[k].x = q.0;
                g.soldiers[k].y = q.1;
                g.soldiers[k].hp = 1e9;
                g.apply(Cmd::Order(k, Order::Hold));
                g.apply(Cmd::Stance(k, if arm == "none" { Stance::HoldFire } else { Stance::Free }));
            }
            let r = 12.0 * s;
            o.m.push(("hole 0s".into(), hole(&g.world, p, r) as f64));
            snap(&g, &kit, "0before");
            let steps = (kit.rules.crater_secs * 0.5 * g.rules.tps as f32 * g.world.p.substeps as f32) as u16;
            let mut t = 0.0;
            for at in [5.0f32, 10.0, 20.0] {
                secs(&mut g, &mut kit, at - t, |g, _| {
                    if crater {
                        let hits: Vec<(f32, f32)> = g.last_shots.iter().filter_map(|x| x.hit).collect();
                        let rr = g.rules.strike * 2.0 * g.world.p.scale as f32;
                        for h in hits {
                            g.world.scorch_disc(h.0, h.1, rr, steps);
                        }
                    }
                });
                t = at;
                o.m.push((format!("hole {at:.0}s"), hole(&g.world, p, r) as f64));
                snap(&g, &kit, &format!("{at:.0}s"));
            }
            o.m.push(("shots".into(), g.shots as f64));
            o.m.push(("killed".into(), g.killed as f64));
            for k in 0..g.soldiers.len() {
                g.apply(Cmd::Stance(k, Stance::HoldFire));
            }
            secs(&mut g, &mut kit, 10.0, |_, _| {});
            o.m.push(("hole 10s after".into(), hole(&g.world, p, r) as f64));
            snap(&g, &kit, "30s");
        }
        "germ" => {
            let p = densest(&g, |_| true);
            if arm == "steer" {
                kit.rules.sick_upkeep = 1.0;
            }
            o.m.push(("before".into(), g.world.living() as f64));
            snap(&g, &kit, "0before");
            if arm != "none" {
                stand_near(&mut g, p);
                assert!(kit.use_tool(&mut g, Tool::Germ(p)));
            }
            let mut ever = vec![false; g.world.walkers.len()];
            let mut t = 0.0;
            for at in [5.0f32, 10.0, 30.0, 60.0] {
                secs(&mut g, &mut kit, at - t, |g, kit| {
                    let d = &g.world;
                    if !d.sick.is_empty() {
                        for (e, &c) in ever.iter_mut().zip(&d.sick) {
                            *e |= c > d.p.immune_steps;
                        }
                    }
                    let _ = kit;
                });
                t = at;
                o.m.push((format!("living {at:.0}s"), g.world.living() as f64));
                snap(&g, &kit, &format!("{at:.0}s"));
            }
            o.m.push(("ever sick".into(), ever.iter().filter(|&&e| e).count() as f64));
        }
        "gas" => {
            let p = densest(&g, |_| true);
            let r = kit.rules.gas_r * s;
            let under: Vec<usize> = g.world.crystals().iter().enumerate().filter(|(_, c)| (c.0 .0 - p.0).hypot(c.0 .1 - p.1) <= r + 4.0 * s).map(|(i, _)| i).collect();
            let charge = |g: &Game<BigDish>| -> f64 { let c = g.world.crystals(); under.iter().map(|&i| c[i].1 as f64).sum() };
            let c0 = charge(&g);
            snap(&g, &kit, "0before");
            if arm != "none" {
                stand_near(&mut g, p);
                assert!(kit.use_tool(&mut g, Tool::Gas(p)));
            }
            o.m.push(("gassed".into(), kit.gassed as f64));
            let asleep = |g: &Game<BigDish>| if g.world.life.is_empty() { 0 } else { (0..g.world.walkers.len()).filter(|&k| g.world.lives(k) && g.world.life[k] == LIFE_DORMANT).count() };
            secs(&mut g, &mut kit, 5.0, |_, _| {});
            o.m.push(("asleep 5s".into(), asleep(&g) as f64));
            if arm == "noise" {
                let q = (p.0 + 10.0 * s, p.1);
                let q = if g.world.open(q.0, q.1) { q } else { (p.0 - 10.0 * s, p.1) };
                kit.use_tool(&mut g, Tool::Blast(q));
            }
            secs(&mut g, &mut kit, 5.0, |_, _| {});
            o.m.push(("asleep 10s".into(), asleep(&g) as f64));
            snap(&g, &kit, "10s");
            secs(&mut g, &mut kit, 10.0, |_, _| {});
            o.m.push(("asleep 20s".into(), asleep(&g) as f64));
            o.m.push(("eaten 20s".into(), c0 - charge(&g)));
            o.m.push(("crystals under".into(), under.len() as f64));
            snap(&g, &kit, "20s");
        }
        "cord" => {
            let p = densest(&g, |c| {
                let d = (c.0 - core.0).hypot(c.1 - core.1) / s;
                (20.0..=40.0).contains(&d)
            });
            let (dx, dy) = (p.0 - core.0, p.1 - core.1);
            let d = dx.hypot(dy).max(1.0);
            let (ux, uy) = (dx / d, dy / d);
            let half = 15.0 * s;
            let (a, b) = ((p.0 - uy * half, p.1 + ux * half), (p.0 + uy * half, p.1 - ux * half));
            let line: Vec<usize> = {
                let n = (2.0 * half) as usize;
                (0..=n).filter_map(|j| {
                    let f = j as f32 / n as f32;
                    let (x, y) = (a.0 + (b.0 - a.0) * f, a.1 + (b.1 - a.1) * f);
                    g.world.open(x, y).then(|| y as usize * g.world.w + x as usize)
                }).collect()
            };
            let on_line = |g: &Game<BigDish>| -> f64 { line.iter().map(|&i| g.world.trail[i] as f64).sum() };
            let beyond = |g: &Game<BigDish>| -> usize {
                let w = &g.world;
                (0..w.walkers.len()).filter(|&k| w.lives(k) && (w.life.is_empty() || w.life[k] != slime_mould::mould::big::LIFE_DEAD)).filter(|&k| {
                    let (x, y, _) = w.walkers[k];
                    (x - p.0) * ux + (y - p.1) * uy > 0.0
                }).count()
            };
            if arm == "nocrater" {
                kit.rules.crater_secs = 0.0;
            }
            o.m.push(("line before".into(), on_line(&g)));
            o.m.push(("beyond before".into(), beyond(&g) as f64));
            snap(&g, &kit, "0before");
            if arm != "none" {
                stand_near(&mut g, p);
                assert!(kit.use_tool(&mut g, Tool::Cord(a, b)));
            }
            o.m.push(("killed".into(), kit.killed[2] as f64));
            let mut t = 0.0;
            for at in [2.0f32, 5.0, 10.0, 20.0, 30.0] {
                secs(&mut g, &mut kit, at - t, |_, _| {});
                t = at;
                o.m.push((format!("line {at:.0}s"), on_line(&g)));
                o.m.push((format!("beyond {at:.0}s"), beyond(&g) as f64));
                snap(&g, &kit, &format!("{at:.0}s"));
            }
        }
        "mine" => {
            // Four mines in a ring 8 dish cells round the larder, where the
            // squad's smell draws the mould (the core-to-larder line was
            // tried first: the mould's road is not straight, and the three
            // mines there killed a mean of 6 walkers in 60 s on 8 seeds).
            let l = g.larder;
            if arm != "none" {
                for j in 0..4 {
                    let a = j as f32 * std::f32::consts::FRAC_PI_2 + std::f32::consts::FRAC_PI_4;
                    let m = (l.0 + 8.0 * s * a.cos(), l.1 + 8.0 * s * a.sin());
                    if g.world.open(m.0, m.1) {
                        kit.use_tool(&mut g, Tool::Mine(m));
                    }
                }
            }
            o.m.push(("mines".into(), kit.mines.len() as f64));
            snap(&g, &kit, "0before");
            let mut t = 0.0;
            for at in [20.0f32, 60.0, 120.0] {
                secs(&mut g, &mut kit, at - t, |_, _| {});
                t = at;
                o.m.push((format!("near larder {at:.0}s"), walkers_within(&g.world, l, 12.0 * s) as f64));
                snap(&g, &kit, &format!("{at:.0}s"));
            }
            o.m.push(("gone off".into(), kit.bangs.len() as f64));
            o.m.push(("killed".into(), kit.killed[1] as f64));
            o.m.push(("stolen".into(), g.stolen as f64));
            o.m.push(("living".into(), g.world.living() as f64));
        }
        _ => panic!("unknown test {test}"),
    }
    o
}

fn main() {
    let mut seeds: Vec<u64> = (1..=8).collect();
    let (mut test, mut warm, mut threads, mut look) = ("blast".to_string(), 60.0f32, 4usize, None::<String>);
    for a in std::env::args().skip(1) {
        let (k, v) = a.split_once('=').expect("key=value");
        match k {
            "seeds" => {
                let (a, b) = v.split_once('-').unwrap_or((v, v));
                seeds = (a.parse().unwrap()..=b.parse().unwrap()).collect();
            }
            "test" => test = v.into(),
            "warm" => warm = v.parse().unwrap(),
            "threads" => threads = v.parse().unwrap(),
            "look" => look = Some(v.into()),
            _ => panic!("unknown {k}"),
        }
    }
    let arms: Vec<&str> = match test.as_str() {
        "blast" | "cord" => vec!["none", "nocrater", "crater"],
        "fire" => vec!["none", "fire", "crater"],
        "germ" => vec!["none", "steer", "germ"],
        "gas" => vec!["none", "gas", "noise"],
        "mine" => vec!["none", "mines"],
        _ => panic!("unknown test"),
    };
    if let Some(d) = &look {
        std::fs::create_dir_all(d).unwrap();
    }
    println!("kitprobe test={test} warm={warm} seeds {seeds:?}; kit {:?}", KitRules::default());
    let jobs: Vec<(u64, &str)> = seeds.iter().flat_map(|&s| arms.iter().map(move |&a| (s, a))).collect();
    let next = std::sync::atomic::AtomicUsize::new(0);
    let out = std::sync::Mutex::new(vec![Out::default(); jobs.len()]);
    let t0 = std::time::Instant::now();
    std::thread::scope(|sc| {
        for _ in 0..threads {
            sc.spawn(|| loop {
                let j = next.fetch_add(1, std::sync::atomic::Ordering::Relaxed);
                if j >= jobs.len() {
                    break;
                }
                let (seed, arm) = jobs[j];
                let lk = if seed == seeds[0] { look.as_deref() } else { None };
                let o = run(&test, arm, seed, warm, lk);
                let line: Vec<String> = o.m.iter().map(|(n, v)| format!("{n} {v:.0}")).collect();
                println!("{arm:>9} seed {seed:>2}: {}", line.join(", "));
                out.lock().unwrap()[j] = o;
            });
        }
    });
    let out = out.into_inner().unwrap();
    println!("({:.0} s)", t0.elapsed().as_secs_f64());
    let get = |s: u64, a: &str| &out[jobs.iter().position(|&(x, y)| x == s && y == a).unwrap()];
    println!("\nmeans:");
    for a in &arms {
        let names: Vec<String> = get(seeds[0], a).m.iter().map(|x| x.0.clone()).collect();
        let line: Vec<String> = names.iter().enumerate().map(|(i, n)| format!("{n} {:.0}", seeds.iter().map(|&s| get(s, a).m[i].1).sum::<f64>() / seeds.len() as f64)).collect();
        println!("  {a:>9}: {}", line.join(", "));
    }
    println!("\npairs (higher / lower / tied, two-sided sign p):");
    for i in 0..arms.len() {
        for j in 0..i {
            let (a, b) = (arms[i], arms[j]);
            let names: Vec<String> = get(seeds[0], a).m.iter().map(|x| x.0.clone()).collect();
            for (m, n) in names.iter().enumerate() {
                let (mut hi, mut lo) = (0, 0);
                for &s in &seeds {
                    let (x, y) = (get(s, a).m[m].1, get(s, b).m[m].1);
                    if x > y {
                        hi += 1;
                    } else if x < y {
                        lo += 1;
                    }
                }
                println!("  {a} vs {b}, {n}: {hi} / {lo} / {}, p {:.4}", seeds.len() - hi - lo, sign_p(hi, hi + lo));
            }
        }
    }
}
