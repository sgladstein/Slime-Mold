//! **Larder M1: soldiers and rules in the harness** (build plan,
//! `Reports/larder-build-plan-2026-11-11.md`, revised after three reviews).
//!
//! ```text
//! cargo run --release --example larderm1 -- census seeds=17-32
//! cargo run --release --example larderm1 -- look=DIR seeds=1 [plan=hunter]
//! cargo run --release --example larderm1 -- cost
//! ```
//!
//! The dish is the 23-pile field (`skirmish::FIELD`), one mould started on
//! the left home, the squad of four on the right home (the larder). The game
//! is `slime_mould::mould::larder`; a round ends when either side is wiped
//! out or at the 15-minute cap.
//!
//! **Plans, each a fixed rule with named settings** (method review,
//! finding 3; tuned on seeds 1-16 only, gated on 17-32):
//! - **idle**: no orders. Soldiers stand at the larder on the default
//!   stance (defend: they fire only at mould within 4 cells).
//! - **random**, spend-matched (method review, finding 5): no stance fire;
//!   each game second each soldier fires at a random point in its range
//!   with the chance that spends what the median of fortress, farmer and
//!   hunter spent on that seed, and every `WANDER` game seconds walks to a
//!   random point within `WANDER_R` of the larder.
//! - **fortress** (`FORT_R`): three soldiers hold a ring of burn lines,
//!   one chord each, round the larder; the fourth clears and harvests the
//!   larder inside it. (First look, seed 1: a ring with no income spent
//!   the 2,000 bank by 68 s and the squad died.)
//! - **farmer** (`FARM_PILES`): soldier k clears and harvests the k-th of
//!   the larder and its nearest piles, round robin, carrying food home.
//! - **hunter** (`HUNT_EVERY`): the squad together clears the mould-eaten
//!   pile nearest the larder, keeps it while it has food, and harvests it;
//!   with none eaten, it walks on the core. At 0 s the only eaten pile is
//!   the mould's home, so as written it marches on the core at once.
//!
//! **Deciding measure** (named before the run): the round's score,
//! `larder::End::score` (win in (1, 3], cap in [-0.5, 0.5], loss in
//! [-2, -1)). **Gate:** fortress, farmer and hunter each beat idle and
//! spend-matched random on 15 or more of 16 held-out seeds, after Holm over
//! the six comparisons; no band holds 14 or more of all runs; a front
//! travels 10 game seconds or more before it touches a soldier on 14 or
//! more of 16; the soldier layer costs under 0.1 ms a step.
use slime_mould::mould::larder::{Game, Order, Rules, Stance, GAME_SECOND};
use slime_mould::mould::skirmish::{FIELD, HOMES};
use slime_mould::mould::{Dish, Ground, H, W};

const FORT_R: f32 = 10.0;
const FARM_PILES: usize = 3;
const HUNT_EVERY: u64 = 5;
const WANDER: u64 = 20;
const WANDER_R: f32 = 30.0;

#[derive(Clone, Copy, Debug, PartialEq)]
enum Plan {
    Idle,
    Random(f32),
    Fortress,
    Farmer,
    Hunter,
}

impl Plan {
    fn name(self) -> &'static str {
        match self {
            Plan::Idle => "idle",
            Plan::Random(_) => "random",
            Plan::Fortress => "fortress",
            Plan::Farmer => "farmer",
            Plan::Hunter => "hunter",
        }
    }
    fn parse(s: &str) -> Plan {
        match s {
            "idle" => Plan::Idle,
            "random" => Plan::Random(1.0),
            "fortress" => Plan::Fortress,
            "farmer" => Plan::Farmer,
            "hunter" => Plan::Hunter,
            _ => panic!("plan: idle, random, fortress, farmer, hunter"),
        }
    }
}

#[derive(Clone, Copy, Debug, Default)]
struct Out {
    score: f32,
    band: &'static str,
    end_t: f32,
    left: usize,
    shots: u32,
    spent: f32,
    harvested: f32,
    first_touch: Option<f32>,
    /// Game second the mould first ate at the larder.
    larder_bite: Option<f32>,
    /// Game second the mould first had walkers 40 cells from its home: a
    /// front on the move.
    front_out: Option<f32>,
    core_down: Option<f32>,
    walkers: usize,
}

fn arg<'a>(args: &'a [String], k: &str) -> Option<&'a str> {
    args.iter()
        .find_map(|a| a.strip_prefix(k).and_then(|r| r.strip_prefix('=')))
}

fn seeds(s: Option<&str>) -> Vec<u64> {
    let s = s.unwrap_or("1-16");
    if let Some((a, b)) = s.split_once('-') {
        (a.parse().unwrap()..=b.parse().unwrap()).collect()
    } else {
        s.split(',').map(|v| v.parse().unwrap()).collect()
    }
}

fn game(seed: u64, rules: Rules) -> Game {
    let mut d = Dish::from_level_with(&FIELD, seed, FIELD.params_for());
    // One mould: no rival, no level bell, no storm.
    d.rival = None;
    d.level = None;
    d.storm_at = u64::MAX;
    Game::new(d, HOMES[1], HOMES[0], rules, seed)
}

/// The larder and its nearest piles, nearest first.
fn farm(g: &Game) -> Vec<(f32, f32)> {
    let mut by: Vec<(f32, (f32, f32))> = g
        .dish
        .piles
        .iter()
        .map(|p| ((p.x - g.larder.0).hypot(p.y - g.larder.1), (p.x, p.y)))
        .collect();
    by.sort_by(|a, b| a.0.total_cmp(&b.0));
    by.iter().take(FARM_PILES).map(|b| b.1).collect()
}

fn setup(g: &mut Game, plan: Plan) {
    let n = g.soldiers.len();
    match plan {
        Plan::Idle | Plan::Hunter => {}
        Plan::Random(_) => (0..n).for_each(|k| g.stance(k, Stance::HoldFire)),
        Plan::Fortress => {
            // One soldier farms the larder inside the ring; the rest hold
            // its chords. A ring with no income spent the bank by 68 s and
            // the squad died (seed 1, first look), so the ring guards food
            // it is eating.
            let ring = (n - 1).max(3).min(n);
            for k in 0..ring {
                let a0 = k as f32 / ring as f32 * std::f32::consts::TAU;
                let a1 = (k + 1) as f32 / ring as f32 * std::f32::consts::TAU;
                let p = |a: f32| (g.larder.0 + FORT_R * a.cos(), g.larder.1 + FORT_R * a.sin());
                g.order(k, Order::Burn(p(a0), p(a1)));
            }
            for k in ring..n {
                g.order(k, Order::Clear(g.larder.0, g.larder.1));
            }
        }
        Plan::Farmer => {
            let piles = farm(g);
            for k in 0..n {
                let p = piles[k % piles.len()];
                g.order(k, Order::Clear(p.0, p.1));
            }
        }
    }
}

fn each_second(g: &mut Game, plan: Plan) {
    let sec = g.dish.t / GAME_SECOND;
    match plan {
        Plan::Random(rate) => {
            let n = g.soldiers.len();
            for k in 0..n {
                if !g.soldiers[k].alive {
                    continue;
                }
                if g.rng.f32() < rate {
                    let s = g.soldiers[k];
                    let a = g.rng.f32() * std::f32::consts::TAU;
                    let r = g.rng.f32() * g.rules.range;
                    g.shoot(k, (s.x + r * a.cos(), s.y + r * a.sin()));
                }
                if sec.is_multiple_of(WANDER) {
                    let a = g.rng.f32() * std::f32::consts::TAU;
                    let r = g.rng.f32() * WANDER_R;
                    let to = (
                        (g.larder.0 + r * a.cos()).clamp(1.0, W as f32 - 1.0),
                        (g.larder.1 + r * a.sin()).clamp(1.0, H as f32 - 1.0),
                    );
                    g.order(k, Order::Move(to.0, to.1));
                }
            }
        }
        Plan::Hunter if sec.is_multiple_of(HUNT_EVERY) => {
            let alive: Vec<usize> = (0..g.soldiers.len()).filter(|&k| g.soldiers[k].alive).collect();
            if alive.is_empty() {
                return;
            }
            // Outward from the larder: the mould-eaten pile nearest it, so
            // the squad takes ground it can carry home from. (Nearest the
            // squad's centre led it across the field on seed 1 with no
            // income; the bank ran dry at 200 shots and it died at 69 s.)
            // Keep the pile being cleared while it has food: the squad
            // clears it, then harvests it.
            let keep = alive.iter().find_map(|&k| match g.soldiers[k].order {
                Order::Clear(x, y) => g
                    .pile_at((x, y))
                    .filter(|&i| g.dish.piles[i].food > 0.0)
                    .map(|_| (x, y)),
                _ => None,
            });
            if keep.is_some() {
                return;
            }
            let c = g.larder;
            let target = g
                .dish
                .piles
                .iter()
                .filter(|p| p.eaters > 0 && p.food > 0.0)
                .min_by(|a, b| (a.x - c.0).hypot(a.y - c.1).total_cmp(&(b.x - c.0).hypot(b.y - c.1)))
                .map(|p| (p.x, p.y));
            for &k in &alive {
                match target {
                    Some(p) => {
                        if g.soldiers[k].order != Order::Clear(p.0, p.1) {
                            g.order(k, Order::Clear(p.0, p.1));
                        }
                    }
                    None => g.order(k, Order::Attack),
                }
            }
        }
        _ => {}
    }
}

fn run(seed: u64, plan: Plan, rules: Rules, look: Option<&str>) -> Out {
    let mut g = game(seed, rules);
    setup(&mut g, plan);
    let li = g.pile_at(g.larder).expect("larder pile");
    let home = HOMES[0];
    let mut o = Out::default();
    let shot_every = 60 * GAME_SECOND;
    while g.end.is_none() {
        if g.dish.t.is_multiple_of(GAME_SECOND) {
            each_second(&mut g, plan);
            let t = g.time();
            if o.larder_bite.is_none() && g.dish.piles[li].eaters > 0 {
                o.larder_bite = Some(t);
            }
            if o.front_out.is_none() && g.dish.walkers.iter().any(|w| (w.x - home.0).hypot(w.y - home.1) > 40.0) {
                o.front_out = Some(t);
            }
        }
        if let Some(dir) = look {
            if g.dish.t.is_multiple_of(shot_every) {
                save(
                    &g,
                    &format!("{dir}/m1-s{seed}-{}-{:03}.png", plan.name(), g.dish.t / GAME_SECOND),
                );
            }
        }
        g.step();
    }
    let end = g.end.unwrap();
    if let Some(dir) = look {
        save(&g, &format!("{dir}/m1-s{seed}-{}-end.png", plan.name()));
    }
    o.score = g.score().unwrap();
    o.band = end.band();
    o.end_t = g.time();
    o.left = g.alive();
    o.shots = g.shots;
    o.spent = g.spent;
    o.harvested = g.harvested;
    o.first_touch = g.first_touch;
    o.core_down = g.core_down.map(|t| t as f32 / GAME_SECOND as f32);
    o.walkers = g.dish.walkers.len();
    o
}

/// The dish as `render` draws it, with each soldier stamped on top as a
/// 3x3 mark (white alive, dark red dead) and the core ringed once taken.
/// The marks are game pieces, not effects (design review, finding 10).
fn save(g: &Game, path: &str) {
    use slime_mould::mould::render::{self, View};
    let scale = 3;
    let (w, h) = render::frame_size(scale);
    let mut buf = vec![0u8; (w * h * 4) as usize];
    let view = View {
        keys: false,
        bare: true,
        style: slime_mould::mould::look::Style::GAME,
        ..View::default()
    };
    render::draw(&g.dish, &view, scale, &mut buf);
    for s in &g.soldiers {
        let (cx, cy) = ((s.x * scale as f32) as i32, (s.y * scale as f32) as i32);
        let col = if s.alive {
            [255, 255, 255, 255]
        } else {
            [140, 20, 20, 255]
        };
        for dy in -2..=2 {
            for dx in -2..=2 {
                let (x, y) = (cx + dx, cy + dy);
                if x >= 0 && y >= 0 && (x as u32) < w && (y as u32) < h {
                    let i = ((y as u32 * w + x as u32) * 4) as usize;
                    buf[i..i + 4].copy_from_slice(&col);
                }
            }
        }
    }
    image::RgbaImage::from_raw(w, h, buf)
        .expect("frame")
        .save(path)
        .expect("save png");
}

fn sign_p(wins: usize, n: usize) -> f64 {
    if n == 0 {
        return 1.0;
    }
    let k = wins.min(n - wins);
    let mut c = 1.0f64;
    let mut tail = 0.0f64;
    for i in 0..=k {
        if i > 0 {
            c = c * (n - i + 1) as f64 / i as f64;
        }
        tail += c;
    }
    (2.0 * tail / 2f64.powi(n as i32)).min(1.0)
}

fn par<T: Send + Default + Clone, F: Fn(u64) -> T + Sync>(seeds: &[u64], f: F) -> Vec<T> {
    let threads = std::thread::available_parallelism().map(|n| n.get()).unwrap_or(1);
    let mut out = vec![T::default(); seeds.len()];
    let size = seeds.len().div_ceil(threads).max(1);
    std::thread::scope(|sc| {
        let f = &f;
        for (o, s) in out.chunks_mut(size).zip(seeds.chunks(size)) {
            sc.spawn(move || {
                for (slot, &seed) in o.iter_mut().zip(s) {
                    *slot = f(seed);
                }
            });
        }
    });
    out
}

fn median(mut v: Vec<f32>) -> f32 {
    v.sort_by(|a, b| a.total_cmp(b));
    if v.is_empty() {
        f32::NAN
    } else {
        v[v.len() / 2]
    }
}

fn opt(v: Option<f32>) -> String {
    v.map(|t| format!("{t:.0}")).unwrap_or("-".into())
}

fn census(seeds: &[u64], rules: Rules) {
    println!(
        "Larder M1 census: {} seeds ({}-{}), squad {}, cap {} game s, a game second = {} steps",
        seeds.len(),
        seeds[0],
        seeds[seeds.len() - 1],
        rules.squad,
        rules.cap,
        GAME_SECOND
    );
    let fixed = [Plan::Idle, Plan::Fortress, Plan::Farmer, Plan::Hunter];
    let mut results: Vec<(Plan, Vec<Out>)> = fixed
        .iter()
        .map(|&p| (p, par(seeds, |s| run(s, p, rules, None))))
        .collect();
    // Spend-matched random: per seed, the median spend rate of the three
    // working plans, as a chance per soldier per game second.
    let rates: Vec<f32> = (0..seeds.len())
        .map(|i| {
            let per: Vec<f32> = results[1..]
                .iter()
                .map(|(_, o)| o[i].shots as f32 / o[i].end_t.max(1.0) / rules.squad as f32)
                .collect();
            median(per).min(1.0)
        })
        .collect();
    let random: Vec<Out> = par(seeds, |s| {
        let i = seeds.iter().position(|&x| x == s).unwrap();
        run(s, Plan::Random(rates[i]), rules, None)
    });
    results.insert(1, (Plan::Random(0.0), random));

    for (plan, o) in &results {
        println!("\n{}:", plan.name());
        for (s, r) in seeds.iter().zip(o) {
            println!(
                "  seed {s:2}: score {:6.3}  {:11}  at {:4.0}  left {}  shots {:5}  spent {:6.0}  harvested {:6.0}  front out {:>4}  first touch {:>4}  larder bite {:>4}  core down {:>4}  walkers {:5}",
                r.score,
                r.band,
                r.end_t,
                r.left,
                r.shots,
                r.spent,
                r.harvested,
                opt(r.front_out),
                opt(r.first_touch),
                opt(r.larder_bite),
                opt(r.core_down),
                r.walkers
            );
        }
        let bands = ["mould wiped", "cap", "squad wiped"]
            .iter()
            .map(|b| format!("{b} {}", o.iter().filter(|r| r.band == *b).count()))
            .collect::<Vec<_>>()
            .join(", ");
        println!(
            "  median score {:.3}; {bands}; median end {:.0} s; median shots {:.0}",
            median(o.iter().map(|r| r.score).collect()),
            median(o.iter().map(|r| r.end_t).collect()),
            median(o.iter().map(|r| r.shots as f32).collect()),
        );
    }

    // The gate: each working plan against idle and random, Holm over six.
    println!("\nGate comparisons (paired by seed, ties dropped, Holm over 6):");
    let mut tests: Vec<(String, usize, usize, f64)> = Vec::new();
    for (plan, o) in &results[2..] {
        for (ctl, c) in &results[..2] {
            let (mut w, mut n) = (0, 0);
            for (a, b) in o.iter().zip(c) {
                if (a.score - b.score).abs() < 1e-4 {
                    continue;
                }
                n += 1;
                if a.score > b.score {
                    w += 1;
                }
            }
            tests.push((format!("{} > {}", plan.name(), ctl.name()), w, n, sign_p(w, n)));
        }
    }
    let mut order: Vec<usize> = (0..tests.len()).collect();
    order.sort_by(|&a, &b| tests[a].3.total_cmp(&tests[b].3));
    let m = tests.len();
    let mut holding = true;
    let mut survives = vec![false; m];
    for (rank, &i) in order.iter().enumerate() {
        holding = holding && tests[i].3 <= 0.05 / (m - rank) as f64;
        survives[i] = holding;
    }
    for (i, (name, w, n, p)) in tests.iter().enumerate() {
        let pass = *w >= 15 && survives[i];
        println!(
            "  {name:20} {w:2} of {n:2}  p {p:.5}  Holm {}  gate (15+ of 16) {}",
            if survives[i] { "survives" } else { "fails" },
            if pass { "PASS" } else { "FAIL" }
        );
    }
    let all: Vec<&Out> = results.iter().flat_map(|(_, o)| o.iter()).collect();
    for b in ["mould wiped", "cap", "squad wiped"] {
        let n = all.iter().filter(|r| r.band == b).count();
        println!(
            "  band {b:11}: {n} of {} runs ({:.0}%)",
            all.len(),
            100.0 * n as f32 / all.len() as f32
        );
    }
    let travel: Vec<Option<f32>> = results[0]
        .1
        .iter()
        .map(|r| Some(r.first_touch.or(r.larder_bite)? - r.front_out?))
        .collect();
    let ok = travel.iter().filter(|t| t.is_some_and(|t| t >= 10.0)).count();
    println!(
        "  front travel (idle: front out to first touch or larder bite) >= 10 s on {ok} of {}; median {:.0} s",
        travel.len(),
        median(travel.iter().flatten().copied().collect())
    );
}

/// The soldier layer's cost a step: the same seed, played with no squad
/// and with `n` soldiers firing freely, 120 game seconds each; the
/// difference over the steps.
fn cost() {
    for n in [4usize, 40] {
        let rules = Rules {
            squad: n,
            start_bank: 1e9,
            ..Rules::default()
        };
        let steps = 120 * GAME_SECOND;
        let mut g = game(1, rules);
        (0..n).for_each(|k| g.stance(k, Stance::Free));
        let mut squad_ns = 0u128;
        let mut dish_ns = 0u128;
        for _ in 0..steps {
            // The squad's part of `Game::step` is everything but the dish
            // step: time the whole, then a bare dish step on a clone would
            // differ in state, so time the dish step directly inside.
            let t0 = std::time::Instant::now();
            g.step();
            let all = t0.elapsed().as_nanos();
            squad_ns += all;
            let _ = &mut dish_ns;
        }
        let mut bare = game(1, Rules { squad: 0, ..rules });
        let t0 = std::time::Instant::now();
        for _ in 0..steps {
            bare.dish.step();
        }
        dish_ns += t0.elapsed().as_nanos();
        let per = |ns: u128| ns as f64 / steps as f64 / 1e6;
        println!(
            "{n:2} soldiers: game step {:.3} ms, bare dish step {:.3} ms, squad layer about {:.3} ms a step ({} shots, {} walkers at the end vs {})",
            per(squad_ns),
            per(dish_ns),
            per(squad_ns) - per(dish_ns),
            g.shots,
            g.dish.walkers.len(),
            bare.dish.walkers.len()
        );
    }
}

fn main() {
    let args: Vec<String> = std::env::args().skip(1).collect();
    let seeds = seeds(arg(&args, "seeds"));
    // M1 as it was measured: none of M2's answers (hoard smell, hunger,
    // drops), so this census reruns to the same numbers.
    let rules = Rules {
        squad: arg(&args, "squad").map(|v| v.parse().unwrap()).unwrap_or(4),
        hoard: 0.0,
        hunger: 0.0,
        drop_every: 0.0,
        ..Rules::default()
    };
    let t0 = std::time::Instant::now();
    if let Some(dir) = arg(&args, "look") {
        std::fs::create_dir_all(dir).expect("look dir");
        let plans: Vec<Plan> = match arg(&args, "plan") {
            Some(p) => vec![Plan::parse(p)],
            None => vec![Plan::Idle, Plan::Fortress, Plan::Farmer, Plan::Hunter],
        };
        for &seed in &seeds {
            for &p in &plans {
                let o = run(seed, p, rules, Some(dir));
                println!("seed {seed} {}: {o:?}", p.name());
            }
        }
    } else {
        match args.first().map(String::as_str) {
            Some("census") => census(&seeds, rules),
            Some("cost") => cost(),
            _ => panic!("census | cost | look=DIR"),
        }
    }
    let _ = Ground::Agar;
    println!("({:.1} s)", t0.elapsed().as_secs_f32());
}
