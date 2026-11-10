//! Red-team scratch harness. Links the slime-mould library; changes nothing in the repo.
//! Modes:
//!   grid SELF RIVAL SEED      one Wild One game, symmetric (400 v 400, repel 1.0 both), grains per side
//!   taut SEED ANGLE SCENT     at 60 s clone; set this mould's fed_scent; compare network over 30 s
//!   wave SEED CHARGE_S        flow on; at 60 s clone; inject CHARGE_S seconds of home-pile output as sap
//!   speed SEED ANGLE          tag walkers at 60 s; displacement after 1, 2, 5 s
use slime_mould::mould::level::DISHES;
use slime_mould::mould::{disc, Dish, Params, Phase, STEPS_PER_SECOND, W, H};

const SPS: u64 = STEPS_PER_SECOND as u64;

fn grain(name: &str, p: &mut Params) {
    let (sa, ra, so) = match name {
        "mesh" => (22.5, 45.0, 9.0),
        "cord" => (45.0, 45.0, 9.0),
        "maze" => (67.5, 67.5, 9.0),
        "bead" => (112.5, 112.5, 9.0),
        "runner" => (22.5, 45.0, 15.0),
        "wide" => (90.0, 45.0, 9.0),
        "hunter" => (22.5, 45.0, 9.0),
        "shy" => (22.5, 45.0, 9.0),
        _ => panic!("grain {name}"),
    };
    p.sensor_angle = (sa as f32).to_radians();
    p.rotate_angle = (ra as f32).to_radians();
    p.sensor_offset = so;
    p.foe_repel = match name {
        "hunter" => -1.0,
        "shy" => 3.0,
        _ => 1.0,
    };
}

fn wild_params() -> Params {
    DISHES[4].params_for()
}

/// The Wild One made symmetric: 400 walkers each side, both foe_repel 1.0.
fn sym_dish(seed: u64, a: &Params, b: &Params) -> Dish {
    let mut d = Dish::from_level_with(&DISHES[4], seed, a.clone());
    let p6 = (d.piles[6].x, d.piles[6].y);
    d.add_rival_of(p6, 400);
    if let Some(r) = d.rival.as_mut() {
        r.params = b.clone();
    }
    d
}

fn run_to(d: &mut Dish, steps: u64) {
    while d.t < steps && d.phase != Phase::Over {
        d.step();
    }
}

fn tube_cells(d: &Dish) -> usize {
    d.trail.iter().filter(|&&t| t > d.params.tube).count()
}

fn near_piles(d: &Dish) -> f32 {
    let n = d.walkers.len().max(1) as f32;
    let k = d
        .walkers
        .iter()
        .filter(|w| d.piles.iter().any(|p| (p.x - w.x).hypot(p.y - w.y) <= p.r + 4.0))
        .count() as f32;
    k / n
}

fn still(d: &Dish, path: &str) {
    use slime_mould::mould::render::{self, View};
    let scale = 3;
    let (w, h) = render::frame_size(scale);
    let mut buf = vec![0u8; (w * h * 4) as usize];
    let view = View { keys: false, style: slime_mould::mould::look::Style::GAME, ..View::default() };
    render::draw(d, &view, scale, &mut buf);
    image::RgbaImage::from_raw(w, h, buf).unwrap().save(path).unwrap();
}

fn main() {
    let args: Vec<String> = std::env::args().skip(1).collect();
    match args[0].as_str() {
        "grid" => {
            let (ga, gb, seed) = (&args[1], &args[2], args[3].parse::<u64>().unwrap());
            let mut a = wild_params();
            let mut b = wild_params();
            grain(ga, &mut a);
            grain(gb, &mut b);
            let mut d = sym_dish(seed, &a, &b);
            let t0 = std::time::Instant::now();
            run_to(&mut d, 150 * SPS + 2);
            let r = d.rival.as_ref().unwrap();
            let share = d.ate / (d.ate + d.wild_ate).max(1.0);
            println!(
                "GRID self={ga} rival={gb} seed={seed} share={share:.3} walkers={}/{} fought={}/{} ate={:.0}/{:.0} ({:.1}s)",
                d.walkers.len(),
                r.walkers.len(),
                d.counts.fought,
                r.counts.fought,
                d.ate,
                d.wild_ate,
                t0.elapsed().as_secs_f32()
            );
        }
        "taut" => {
            let seed = args[1].parse::<u64>().unwrap();
            let angle: f32 = args[2].parse().unwrap();
            let scent: f32 = args[3].parse().unwrap();
            let mut p = wild_params();
            p.sensor_angle = angle.to_radians();
            let mut d = Dish::from_level_with(&DISHES[4], seed, p);
            run_to(&mut d, 60 * SPS);
            let mut c = d.clone();
            let mut t = d.clone();
            t.params.fed_scent = scent;
            let (ca0, cw0) = (c.ate, c.wild_ate);
            let (ta0, tw0) = (t.ate, t.wild_ate);
            let m0 = c.measure();
            println!(
                "TAUT seed={seed} angle={angle} scent={scent} t=60 len={:.1} tube={} near={:.2} walkers={}",
                m0.length,
                tube_cells(&c),
                near_piles(&c),
                c.walkers.len()
            );
            for &s in &[2u64, 5, 10, 20, 30] {
                run_to(&mut c, (60 + s) * SPS);
                run_to(&mut t, (60 + s) * SPS);
                if let Ok(dir) = std::env::var("STILL") {
                    if s == 5 || s == 20 {
                        still(&c, &format!("{dir}/taut-s{seed}-a{angle}-control-{s}s.png"));
                        still(&t, &format!("{dir}/taut-s{seed}-a{angle}-scent{scent}-{s}s.png"));
                    }
                }
                let (mc, mt) = (c.measure(), t.measure());
                let sc = (c.ate - ca0) / ((c.ate - ca0) + (c.wild_ate - cw0)).max(1.0);
                let st = (t.ate - ta0) / ((t.ate - ta0) + (t.wild_ate - tw0)).max(1.0);
                println!(
                    "  +{s:>2}s control len={:.1} tube={} near={:.2} walkers={} share_since={sc:.3} | taut len={:.1} tube={} near={:.2} walkers={} share_since={st:.3} reach {}/{}",
                    mc.length,
                    tube_cells(&c),
                    near_piles(&c),
                    c.walkers.len(),
                    mt.length,
                    tube_cells(&t),
                    near_piles(&t),
                    t.walkers.len(),
                    mc.reach,
                    mt.reach
                );
            }
        }
        "wave" => {
            let seed = args[1].parse::<u64>().unwrap();
            let charge: f32 = args[2].parse().unwrap();
            let mut p = wild_params();
            p.flow = true;
            if let Some(s) = args.get(3) {
                p.stream = s.parse().unwrap();
            }
            let mut d = Dish::from_level_with(&DISHES[4], seed, p.clone());
            run_to(&mut d, 60 * SPS);
            let pk: usize = std::env::var("PILE").ok().and_then(|v| v.parse().ok()).unwrap_or(0);
            let home = d.piles[pk].clone();
            let per_step = home.bite(p.table, p.eat);
            let amount = per_step * charge * SPS as f32;
            let mut c = d.clone();
            let mut s = d.clone();
            let rim: Vec<usize> = disc(home.r + 1.5)
                .filter_map(|(ox, oy)| {
                    let (x, y) = (home.x + ox, home.y + oy);
                    if x >= 0.0 && y >= 0.0 && (x as usize) < W && (y as usize) < H {
                        Some(y as usize * W + x as usize)
                    } else {
                        None
                    }
                })
                .collect();
            for &i in &rim {
                s.sap[i] += amount / rim.len() as f32;
            }
            println!(
                "WAVE seed={seed} charge={charge}s amount={amount:.0} (home bite/step {per_step:.3}, eaters {}), stream={}",
                home.eaters, p.stream
            );
            let bins = W / 16;
            let col = |d: &Dish| {
                let mut v = vec![0.0f32; bins];
                let mut n = vec![0i64; bins];
                for (i, &f) in d.sap.iter().enumerate() {
                    v[(i % W) / 16] += f;
                }
                for w in &d.walkers {
                    let b = (w.x as usize / 16).min(bins - 1);
                    v[b] += w.food;
                    n[b] += 1;
                }
                (v, n)
            };
            for &ms in &[250u64, 500, 1000, 2000, 3000, 5000, 8000, 12000] {
                let at = 60 * SPS + ms * SPS / 1000;
                run_to(&mut c, at);
                run_to(&mut s, at);
                let (vc, nc) = col(&c);
                let (vs, ns) = col(&s);
                let dv: Vec<String> = vc.iter().zip(&vs).map(|(a, b)| format!("{:>6.0}", b - a)).collect();
                let dn: Vec<String> = nc.iter().zip(&ns).map(|(a, b)| format!("{:>4}", b - a)).collect();
                println!("  +{:>5}ms food diff by x/16: {}", ms, dv.join(""));
                println!("           walker diff by x/16: {}  (walkers {} v {}, share {:.3} v {:.3})", dn.join(""), c.walkers.len(), s.walkers.len(), c.ate / (c.ate + c.wild_ate), s.ate / (s.ate + s.wild_ate));
            }
            let (vc, _) = col(&c);
            let wcols: Vec<String> = (0..bins).map(|b| format!("{:>6}", b * 16)).collect();
            println!("  x columns:                    {}", wcols.join(""));
            let base: Vec<String> = vc.iter().map(|a| format!("{:>6.0}", a)).collect();
            println!("  control food by x/16 at end:  {}", base.join(""));
        }
        "look" => {
            let (g, seed, secs) = (&args[1], args[2].parse::<u64>().unwrap(), args[3].parse::<u64>().unwrap());
            let mut a = wild_params();
            grain(g, &mut a);
            let mut d = sym_dish(seed, &a, &a);
            run_to(&mut d, secs * SPS);
            let dir = std::env::var("STILL").unwrap();
            still(&d, &format!("{dir}/grain-{g}-s{seed}-{secs}s.png"));
            println!("LOOK {g} seed {seed} at {secs}s: walkers {}/{}", d.walkers.len(), d.rival.as_ref().unwrap().walkers.len());
        }
        "molt" => {
            // At 60 s, switch this mould's grain (Grain's molt, Bloomfront's Temper); compare paired clone.
            let (seed, g) = (args[1].parse::<u64>().unwrap(), &args[2]);
            let mut d = Dish::from_level_with(&DISHES[4], seed, wild_params());
            run_to(&mut d, 60 * SPS);
            let mut c = d.clone();
            let mut t = d.clone();
            grain(g, &mut t.params);
            let (ca0, cw0, ta0, tw0) = (c.ate, c.wild_ate, t.ate, t.wild_ate);
            for &s in &[5u64, 15, 30, 60, 90] {
                run_to(&mut c, (60 + s) * SPS);
                run_to(&mut t, (60 + s) * SPS);
                let sc = (c.ate - ca0) / ((c.ate - ca0) + (c.wild_ate - cw0)).max(1.0);
                let st = (t.ate - ta0) / ((t.ate - ta0) + (t.wild_ate - tw0)).max(1.0);
                println!("MOLT seed={seed} to={g} +{s}s share_since control {sc:.3} molt {st:.3} | walkers {}/{} v {}/{} | len {:.0} v {:.0}", c.walkers.len(), c.rival.as_ref().map_or(0, |r| r.walkers.len()), t.walkers.len(), t.rival.as_ref().map_or(0, |r| r.walkers.len()), c.measure().length, t.measure().length);
                if let Ok(dir) = std::env::var("STILL") {
                    if s == 5 && seed == 1 {
                        still(&t, &format!("{dir}/molt-s{seed}-{g}-{s}s.png"));
                    }
                }
            }
        }
        "speed" => {
            let seed = args[1].parse::<u64>().unwrap();
            let angle: f32 = args[2].parse().unwrap();
            let mut p = wild_params();
            p.sensor_angle = angle.to_radians();
            let mut d = Dish::from_level_with(&DISHES[4], seed, p);
            run_to(&mut d, 60 * SPS);
            let start: Vec<(f32, f32)> = d.walkers.iter().map(|w| (w.x, w.y)).collect();
            for (k, w) in d.walkers.iter_mut().enumerate() {
                w.woke = k as u64; // tag: k < t - wake_grace, so just_woke() stays false
            }
            for &s in &[1u64, 2, 5, 10] {
                run_to(&mut d, (60 + s) * SPS);
                let mut ds: Vec<f32> = d
                    .walkers
                    .iter()
                    .filter(|w| (w.woke as usize) < start.len())
                    .map(|w| {
                        let (x, y) = start[w.woke as usize];
                        (w.x - x).hypot(w.y - y)
                    })
                    .collect();
                ds.sort_by(|a, b| a.total_cmp(b));
                let q = |f: f32| ds[((ds.len() - 1) as f32 * f) as usize];
                println!(
                    "SPEED seed={seed} angle={angle} +{s}s tagged alive {} of {}: displacement p10 {:.0} median {:.0} p90 {:.0} max {:.0} cells",
                    ds.len(),
                    start.len(),
                    q(0.1),
                    q(0.5),
                    q(0.9),
                    ds[ds.len() - 1]
                );
            }
        }
        m => panic!("mode {m}"),
    }
}
