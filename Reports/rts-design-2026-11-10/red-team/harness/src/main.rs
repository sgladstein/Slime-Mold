//! Red-team scratch harness. Links the slime-mould library; changes nothing in the repo.
//! Modes:
//!   grid SELF RIVAL SEED      one Wild One game, symmetric (400 v 400, repel 1.0 both), grains per side
//!   taut SEED ANGLE SCENT     at 60 s clone; set this mould's fed_scent; compare network over 30 s (ON_FIELD=1: on the field)
//!   wave SEED CHARGE_S        flow on; at 60 s clone; inject CHARGE_S seconds of home-pile output as sap
//!   speed SEED ANGLE          tag walkers at 60 s; displacement after 1, 2, 5 s
//!   field SELF RIVAL SEED     the grid game on the field (added after the owner's 2026-11-10 19:24Z
//!                             remark that 7 piles limit the options): 23 smaller mirrored piles, same grid
//!   flook SELF RIVAL SEED SECS  a still of a field game (STILL=DIR)
//!   verbs SEED                the paired-clone verb census (DISH=wild|field, FIELD=layout): at 60 s
//!                             clone the dish, apply one verb in one copy, planned or at random, and
//!                             compare 30 s against the untouched copy
//!   comeback SEED             can a hand overturn the opening? (DISH=wild|field; wild is the symmetric
//!                             Wild One): mesh v mesh to 60 s, then give the side behind one verb in
//!                             one copy and run every copy to the bell against the untouched one
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

/// The field: The Wild One's grid and homes (400 walkers each side), but its
/// seven piles replaced by many smaller ones, mirrored about x = 128. Each
/// pile feeds at most `table * eat / upkeep` = 300 walkers, so more piles let
/// the moulds grow denser.
///
/// Default, the scatter: a home pile each side (r 6, 30k food), nine piles a
/// side at fixed, irregular spots (drawn once by a Poisson-disc sampler, at
/// least 28 cells apart) and three on the centre line, r 4 and 15k each: 23
/// piles, 375k food (The Wild One: 7 piles, 260k). `FIELD=lattice` is the
/// first try, looked at and set aside: a 5 x 5 grid plus four by the homes,
/// 31 piles of 18k, which grew a carpet over the whole dish (4,000-6,000
/// walkers a side) rather than a network.
fn field_dish(seed: u64, a: &Params, b: &Params) -> Dish {
    let mut d = Dish::from_level_with(&DISHES[4], seed, a.clone());
    let t = d.piles[0].clone();
    let mk = |x: f32, y: f32, r: f32, food: f32| slime_mould::mould::Pile { x, y, r, food, full: food, ..t.clone() };
    let mut piles = vec![mk(40.0, 80.0, 6.0, 30_000.0)];
    if std::env::var("FIELD").as_deref() == Ok("lattice") {
        for &x in &[64.0f32, 96.0, 128.0, 160.0, 192.0] {
            for &y in &[24.0f32, 52.0, 80.0, 108.0, 136.0] {
                piles.push(mk(x, y, 4.0, 18_000.0));
            }
        }
        for &(x, y) in &[(40.0f32, 40.0f32), (40.0, 120.0), (216.0, 40.0), (216.0, 120.0)] {
            piles.push(mk(x, y, 4.0, 18_000.0));
        }
    } else {
        // `FIELD=scatter2`: a second draw of the same sampler (seed 11 not 7),
        // to check that a result is not one layout's. `FIELD_LEFT="x,y x,y
        // ..."` gives any other draw (the layout sweep; the sampler is
        // `field/sweep/layouts.py`, whose seeds 7 and 11 print these two).
        let left: Vec<(f32, f32)> = if let Ok(l) = std::env::var("FIELD_LEFT") {
            l.split_whitespace()
                .map(|xy| {
                    let (x, y) = xy.split_once(',').unwrap();
                    (x.parse().unwrap(), y.parse().unwrap())
                })
                .collect()
        } else if std::env::var("FIELD").as_deref() == Ok("scatter2") {
            vec![(105.0f32, 75.0f32), (64.0, 92.0), (76.0, 121.0), (110.0, 145.0), (59.0, 49.0), (21.0, 98.0), (90.0, 48.0), (100.0, 16.0), (44.0, 121.0)]
        } else {
            vec![(78.0f32, 20.0f32), (67.0, 61.0), (18.0, 71.0), (56.0, 126.0), (110.0, 17.0), (98.0, 51.0), (92.0, 108.0), (96.0, 142.0), (65.0, 96.0)]
        };
        for &(x, y) in &left {
            piles.push(mk(x, y, 4.0, 15_000.0));
            piles.push(mk(256.0 - x, y, 4.0, 15_000.0));
        }
        for &y in &[40.0f32, 80.0, 120.0] {
            piles.push(mk(128.0, y, 4.0, 15_000.0));
        }
    }
    piles.push(mk(216.0, 80.0, 6.0, 30_000.0));
    d.piles = piles;
    d.add_rival_of((216.0, 80.0), 400);
    if let Some(r) = d.rival.as_mut() {
        r.params = b.clone();
    }
    d
}

/// The verb census's measures, read from a copy and its control.
fn tube_mask(d: &Dish) -> (Vec<bool>, Vec<bool>) {
    let r = d.rival.as_ref().unwrap();
    let t = d.params.tube;
    (d.trail.iter().map(|&v| v > t).collect(), r.trail.iter().map(|&v| v > t).collect())
}

/// Cells whose tube state differs between two copies, over the control's
/// tube cells (both moulds): how much of the picture the verb changed.
///
/// The dish is chaotic: any change, however small, reorders the walkers'
/// turns, so the whole picture drifts apart within seconds (a one-walker
/// nudge does it too; the census's `nudge` row measures that drift). So
/// `at` limits the count to cells within 24 of where the verb acted, and the
/// count is of cells, not a ratio, since a spot on bare agar has no tube to
/// divide by. The whole dish when `at` is None.
fn visible_change(c: &Dish, v: &Dish, at: Option<(f32, f32)>) -> f32 {
    let (ca, cb) = tube_mask(c);
    let (va, vb) = tube_mask(v);
    let near = |i: usize| at.is_none_or(|(x, y)| ((i % W) as f32 + 0.5 - x).hypot((i / W) as f32 + 0.5 - y) <= 24.0);
    (0..W * H).filter(|&i| near(i)).map(|i| (ca[i] != va[i]) as usize + (cb[i] != vb[i]) as usize).sum::<usize>() as f32
}

/// Territory: this mould's share of the tube cells where one mould's trail
/// is above the tube threshold and above the other's.
fn territory(d: &Dish) -> f32 {
    let r = d.rival.as_ref().unwrap();
    let t = d.params.tube;
    let (mut a, mut b) = (0usize, 0usize);
    for (x, y) in d.trail.iter().zip(&r.trail) {
        if *x > t && x > y {
            a += 1;
        } else if *y > t && y > x {
            b += 1;
        }
    }
    a as f32 / (a + b).max(1) as f32
}

/// Dish piles with food left that each mould holds (more eaters on it).
fn piles_held(d: &Dish) -> (usize, usize) {
    let live = d.piles.iter().filter(|p| !p.oats && p.food > 0.0);
    let mine = live.clone().filter(|p| p.eaters > p.foe_eaters).count();
    let theirs = live.filter(|p| p.foe_eaters > p.eaters).count();
    (mine, theirs)
}

struct Xs(u64);
impl Xs {
    fn f(&mut self) -> f32 {
        self.0 ^= self.0 << 13;
        self.0 ^= self.0 >> 7;
        self.0 ^= self.0 << 17;
        (self.0 >> 40) as f32 / (1u64 << 24) as f32
    }
    /// A random open-dish point.
    fn agar(&mut self, d: &Dish) -> (f32, f32) {
        loop {
            let (x, y) = (self.f() * W as f32, self.f() * H as f32);
            if d.ground[(y as usize) * W + x as usize] == slime_mould::mould::Ground::Agar {
                return (x, y);
            }
        }
    }
}

/// A stroke of `tool` centred on `m`, along the unit direction `dir`.
fn stroke(d: &mut Dish, tool: slime_mould::mould::Tool, m: (f32, f32), dir: (f32, f32), len: f32) {
    let a = (m.0 - dir.0 * len / 2.0, m.1 - dir.1 * len / 2.0);
    let b = (m.0 + dir.0 * len / 2.0, m.1 + dir.1 * len / 2.0);
    if d.begin_stroke(tool, a.0, a.1) {
        d.drag_stroke(b.0, b.1);
        d.end_stroke();
    }
}

/// The comeback test's aim, for the side behind (`actor_self`: the self
/// mould; else the rival): the leader's most forward pile, the one it holds
/// (more of its eaters than the actor's) nearest the actor's home; else the
/// nearest pile the leader is on at all; else the centre. Returns the target
/// and the leader's home, whose road to the target the strokes cross.
fn front(d: &Dish, actor_self: bool) -> ((f32, f32), (f32, f32)) {
    let (mine, theirs) = if actor_self { ((40.0f32, 80.0f32), (216.0f32, 80.0f32)) } else { ((216.0, 80.0), (40.0, 80.0)) };
    let near = |q: &&slime_mould::mould::Pile| (q.x - mine.0).hypot(q.y - mine.1);
    // `eaters` are the self mould's, `foe_eaters` the rival's.
    let leader_on = |q: &slime_mould::mould::Pile| if actor_self { q.foe_eaters } else { q.eaters };
    let actor_on = |q: &slime_mould::mould::Pile| if actor_self { q.eaters } else { q.foe_eaters };
    let live = || d.piles.iter().filter(|q| !q.oats && q.food > 0.0);
    let target = live()
        .filter(|q| leader_on(q) > actor_on(q))
        .min_by(|a, b| near(a).total_cmp(&near(b)))
        .or_else(|| live().filter(|q| leader_on(q) > 0).min_by(|a, b| near(a).total_cmp(&near(b))))
        .map(|q| (q.x, q.y))
        .unwrap_or((128.0, 80.0));
    (target, theirs)
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
            // ON_FIELD=1: the same test on the field (`field_dish`), both
            // moulds on these numbers, after the owner's 19:24Z remark.
            let mut d = if std::env::var("ON_FIELD").is_ok() {
                field_dish(seed, &p, &p)
            } else {
                Dish::from_level_with(&DISHES[4], seed, p)
            };
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
        "field" | "flook" => {
            let (ga, gb, seed) = (&args[1], &args[2], args[3].parse::<u64>().unwrap());
            let mut a = wild_params();
            let mut b = wild_params();
            grain(ga, &mut a);
            grain(gb, &mut b);
            let mut d = field_dish(seed, &a, &b);
            if args[0] == "flook" {
                let dir = std::env::var("STILL").unwrap();
                for s in args[4].split(',') {
                    let s: u64 = s.parse().unwrap();
                    run_to(&mut d, s * SPS);
                    let layout = std::env::var("FIELD").unwrap_or_else(|_| "scatter".into());
                    still(&d, &format!("{dir}/field-{layout}-{ga}-{gb}-s{seed}-{s}s.png"));
                    let r = d.rival.as_ref().unwrap();
                    println!("FLOOK {ga} v {gb} seed {seed} at {s}s: walkers {}/{} fought {}/{} piles left {}", d.walkers.len(), r.walkers.len(), d.counts.fought, r.counts.fought, d.piles.iter().filter(|p| p.food > 0.0).count());
                }
                return;
            }
            let t0 = std::time::Instant::now();
            let sh = |d: &Dish| d.ate / (d.ate + d.wild_ate).max(1.0);
            run_to(&mut d, 30 * SPS);
            let s30 = sh(&d);
            run_to(&mut d, 60 * SPS);
            let s60 = sh(&d);
            let w60 = (d.walkers.len(), d.rival.as_ref().unwrap().walkers.len());
            run_to(&mut d, 150 * SPS + 2);
            let r = d.rival.as_ref().unwrap();
            let share = d.ate / (d.ate + d.wild_ate).max(1.0);
            println!(
                "FIELD layout={} self={ga} rival={gb} seed={seed} share={share:.3} share30={s30:.3} share60={s60:.3} walkers60={}/{} walkers={}/{} fought={}/{} ate={:.0}/{:.0} piles_left={} ({:.1}s)",
                std::env::var("FIELD").unwrap_or_else(|_| "scatter".into()),
                w60.0,
                w60.1,
                d.walkers.len(),
                r.walkers.len(),
                d.counts.fought,
                r.counts.fought,
                d.ate,
                d.wild_ate,
                d.piles.iter().filter(|p| p.food > 0.0).count(),
                t0.elapsed().as_secs_f32()
            );
        }
        "verbs" => {
            use slime_mould::mould::Tool;
            let seed = args[1].parse::<u64>().unwrap();
            let on_field = std::env::var("DISH").as_deref() == Ok("field");
            let p = wild_params();
            let mut d = if on_field { field_dish(seed, &p, &p) } else { Dish::from_level_with(&DISHES[4], seed, p) };
            run_to(&mut d, 60 * SPS);
            let t0 = d.t;
            let label = if on_field { std::env::var("FIELD").unwrap_or_else(|_| "scatter".into()) } else { "wild".into() };
            // The plan's target: the rival's most forward pile, the one it
            // holds (more of its eaters than ours) nearest our home; else
            // the nearest pile it is on at all; else the centre.
            let near = |q: &&slime_mould::mould::Pile| (q.x - 40.0).hypot(q.y - 80.0);
            let live = || d.piles.iter().filter(|q| !q.oats && q.food > 0.0);
            let target = live()
                .filter(|q| q.foe_eaters > q.eaters)
                .min_by(|a, b| near(a).total_cmp(&near(b)))
                .or_else(|| live().filter(|q| q.foe_eaters > 0).min_by(|a, b| near(a).total_cmp(&near(b))))
                .map(|q| (q.x, q.y))
                .unwrap_or((128.0, 80.0));
            let home = (216.0f32, 80.0f32);
            let (dx, dy) = (target.0 - home.0, target.1 - home.1);
            let n = dx.hypot(dy).max(1.0);
            let along = (dx / n, dy / n);
            let across = (-along.1, along.0);
            let mid = (home.0 + dx * 0.5, home.1 + dy * 0.5);
            let mut rng = Xs(seed.wrapping_mul(0x9E37_79B9_7F4A_7C15) | 1);
            let mut c = d.clone();
            let verbs: Vec<(&str, &str)> = vec![
                ("lamp", "plan"), ("lamp", "random"),
                ("oats", "plan"), ("oats", "random"),
                ("salt", "plan"), ("salt", "random"),
                ("snip", "plan"), ("snip", "random"),
                ("taut", "plan"),
                ("molt", "plan"), ("molt", "random"),
                ("nudge", "null"),
            ];
            let mut copies: Vec<Dish> = Vec::new();
            let mut spots: Vec<Option<(f32, f32)>> = Vec::new();
            for &(verb, mode) in &verbs {
                let mut v = d.clone();
                let spot = match (verb, mode) {
                    ("lamp", "plan") => { v.set_lamp(Some(target)); Some(target) }
                    ("lamp", _) => { let q = rng.agar(&v); v.set_lamp(Some(q)); Some(q) }
                    ("oats", "plan") => {
                        // Just past the rival's pile on our side of it
                        // (6 cells further from the rival's home), so
                        // our walkers reach the new food first. It lost
                        // share on both dishes: new food near the front
                        // feeds whoever is already there.
                        let q = (target.0 + along.0 * 6.0, target.1 + along.1 * 6.0);
                        v.place_oats(q.0, q.1);
                        Some(q)
                    }
                    ("oats", _) => { let q = rng.agar(&v); v.place_oats(q.0, q.1); Some(q) }
                    ("salt", "plan") => { stroke(&mut v, Tool::Salt, mid, across, 60.0); Some(mid) }
                    ("salt", _) => { let q = rng.agar(&v); let a = rng.f() * std::f32::consts::TAU; stroke(&mut v, Tool::Salt, q, (a.cos(), a.sin()), 60.0); Some(q) }
                    ("snip", "plan") => { stroke(&mut v, Tool::Snip, mid, across, 30.0); Some(mid) }
                    ("snip", _) => { let q = rng.agar(&v); let a = rng.f() * std::f32::consts::TAU; stroke(&mut v, Tool::Snip, q, (a.cos(), a.sin()), 30.0); Some(q) }
                    // Taut and molt act on the whole mould, so their 5 s
                    // change is counted over the whole dish (spot None),
                    // while the nudge's is counted near the target. That
                    // is not like for like: whole-dish change includes
                    // chaos, so their 5 s "seen" figures do not count.
                    ("taut", _) => { v.params.fed_scent = 1.0; None }
                    ("molt", "plan") => { grain("cord", &mut v.params); None }
                    ("molt", _) => { let g = ["cord", "maze", "runner", "hunter", "bead"][(rng.f() * 5.0) as usize % 5]; grain(g, &mut v.params); None }
                    // The chaos floor: turn one walker by a hundredth of a
                    // radian. Its 5 s change is measured near the plan's
                    // target, where the placed verbs' plans act.
                    ("nudge", _) => { if let Some(w) = v.walkers.first_mut() { w.heading += 0.01; } Some(target) }
                    _ => unreachable!(),
                };
                copies.push(v);
                spots.push(spot);
            }
            let (ca0, cw0) = (c.ate, c.wild_ate);
            let since = |x: &Dish| (x.ate - ca0) / ((x.ate - ca0) + (x.wild_ate - cw0)).max(1.0);
            let mut vis5 = vec![0.0f32; copies.len()];
            let mut sh5 = vec![0.0f32; copies.len()];
            run_to(&mut c, t0 + 5 * SPS);
            let c5 = since(&c);
            for (k, v) in copies.iter_mut().enumerate() {
                // The lamp is held 10 s, then let go.
                run_to(v, t0 + 5 * SPS);
                vis5[k] = visible_change(&c, v, spots[k]);
                sh5[k] = since(v);
            }
            run_to(&mut c, t0 + 30 * SPS);
            let (c30, ct30, cp30) = (since(&c), territory(&c), piles_held(&c));
            for (k, v) in copies.iter_mut().enumerate() {
                run_to(v, t0 + 10 * SPS);
                v.set_lamp(None);
                run_to(v, t0 + 30 * SPS);
                let (vp30, vt30) = (piles_held(v), territory(v));
                println!(
                    "VERB dish={label} seed={seed} verb={} mode={} share5={:.3} ctl5={c5:.3} share30={:.3} ctl30={c30:.3} vis5={:.0} terr30={vt30:.3} ctlterr30={ct30:.3} piles30={}/{} ctlpiles30={}/{} target={:.0},{:.0}",
                    verbs[k].0, verbs[k].1, sh5[k], since(v), vis5[k], vp30.0, vp30.1, cp30.0, cp30.1, target.0, target.1
                );
            }
        }
        "comeback" => {
            // Can a hand overturn the opening? With no hands the side ahead
            // at 60 s leads at the bell in about 4 games of 5 on the field
            // (field/README.md). Equal moulds (mesh v mesh) to 60 s; then the
            // side behind gets one verb in one copy, and every copy runs to
            // the bell (150 s, as the field games) against the untouched one.
            use slime_mould::mould::Tool;
            let seed = args[1].parse::<u64>().unwrap();
            let on_field = std::env::var("DISH").as_deref() == Ok("field");
            let p = wild_params();
            let mut d = if on_field { field_dish(seed, &p, &p) } else { sym_dish(seed, &p, &p) };
            let label = if on_field { std::env::var("FIELD").unwrap_or_else(|_| "scatter".into()) } else { "wild".into() };
            run_to(&mut d, 60 * SPS);
            let t0 = d.t;
            let self_share = |x: &Dish| x.ate / (x.ate + x.wild_ate).max(1.0);
            let s60 = self_share(&d);
            let actor_self = s60 < 0.5;
            let mine = |s: f32| if actor_self { s } else { 1.0 - s };
            let (a0, w0) = (d.ate, d.wild_ate);
            let since = |x: &Dish| mine((x.ate - a0) / ((x.ate - a0) + (x.wild_ate - w0)).max(1.0));
            // Aim at the leader's front: the target pile, and the middle of
            // the leader's road from its home to it, with the across unit.
            let aim = |v: &Dish| {
                let (target, home) = front(v, actor_self);
                let (dx, dy) = (target.0 - home.0, target.1 - home.1);
                let n = dx.hypot(dy).max(1.0);
                (target, (home.0 + dx * 0.5, home.1 + dy * 0.5), (-dy / n, dx / n))
            };
            fn actor_params(v: &mut Dish, actor_self: bool) -> &mut Params {
                if actor_self { &mut v.params } else { &mut v.rival.as_mut().unwrap().params }
            }
            let mut rng = Xs(seed.wrapping_mul(0x9E37_79B9_7F4A_7C15) | 1);
            // salt x6 and lamp x6 act again every 15 s, re-aimed each time:
            // a hand that keeps working, not one move. Their salt is not
            // limited by the dish's stock (`salt_left`), so they can.
            let verbs: Vec<(&str, &str)> = vec![
                ("none", "control"),
                ("salt", "plan"), ("salt", "random"), ("salt", "x6"),
                ("lamp", "plan"), ("lamp", "random"), ("lamp", "x6"),
                ("snip", "plan"),
                ("taut", "plan"),
                ("molt", "hunter"), ("molt", "random"),
            ];
            let salt_at = |v: &mut Dish, rng: &mut Xs, planned: bool| {
                v.salt_left = v.salt_left.max(1);
                if planned {
                    let (_, mid, across) = aim(v);
                    stroke(v, Tool::Salt, mid, across, 60.0);
                } else {
                    let q = rng.agar(v);
                    let a = rng.f() * std::f32::consts::TAU;
                    stroke(v, Tool::Salt, q, (a.cos(), a.sin()), 60.0);
                }
            };
            let mut copies: Vec<Dish> = Vec::new();
            for &(verb, mode) in &verbs {
                let mut v = d.clone();
                match (verb, mode) {
                    ("none", _) => {}
                    ("salt", m) => salt_at(&mut v, &mut rng, m != "random"),
                    ("lamp", "random") => { let q = rng.agar(&v); v.set_lamp(Some(q)); }
                    ("lamp", _) => { let (t, _, _) = aim(&v); v.set_lamp(Some(t)); }
                    ("snip", _) => { let (_, mid, across) = aim(&v); v.snips_left = v.snips_left.max(1); stroke(&mut v, Tool::Snip, mid, across, 30.0); }
                    ("taut", _) => actor_params(&mut v, actor_self).fed_scent = 1.0,
                    // Hunter beat mesh in 31 of 32 field games.
                    ("molt", "hunter") => grain("hunter", actor_params(&mut v, actor_self)),
                    ("molt", _) => { let g = ["cord", "maze", "runner", "hunter", "bead"][(rng.f() * 5.0) as usize % 5]; grain(g, actor_params(&mut v, actor_self)); }
                    _ => unreachable!(),
                }
                copies.push(v);
            }
            // Lamps are held 10 s at a time.
            for &(sec, act) in &[(10u64, false), (15, true), (25, false), (30, true), (40, false), (45, true), (55, false), (60, true), (70, false), (75, true), (85, false), (90, false)] {
                for (k, v) in copies.iter_mut().enumerate() {
                    run_to(v, t0 + sec * SPS);
                    match (verbs[k], act) {
                        (("lamp", "x6"), true) => { let (t, _, _) = aim(v); v.set_lamp(Some(t)); }
                        (("lamp", _), false) => v.set_lamp(None),
                        (("salt", "x6"), true) => salt_at(v, &mut rng, true),
                        _ => {}
                    }
                }
            }
            let ctl = &copies[0];
            let (cbell, csince) = (mine(self_share(ctl)), since(ctl));
            for (k, v) in copies.iter().enumerate() {
                println!(
                    "COMEBACK dish={label} seed={seed} actor={} behind60={:.3} verb={} mode={} bell={:.3} ctlbell={cbell:.3} since={:.3} ctlsince={csince:.3} t={:.0}",
                    if actor_self { "self" } else { "rival" },
                    mine(s60),
                    verbs[k].0,
                    verbs[k].1,
                    mine(self_share(v)),
                    since(v),
                    v.t as f32 / SPS as f32
                );
            }
        }
        m => panic!("mode {m}"),
    }
}
