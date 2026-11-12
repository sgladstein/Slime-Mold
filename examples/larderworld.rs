//! **Larder in the world, the gate** (ROADMAP W2b): idle and random hands
//! must lose to the mould, and a plan must beat both on paired seeds.
//!
//! ```text
//! cargo run --release --example larderworld -- census seeds=1-16 [scale=2 walkers=5000 hands=idle,random,farm,rush,wall cap=600 threads=4]
//! cargo run --release --example larderworld -- look=DIR seeds=1 hands=farm [every=60]
//! cargo run --release --example larderworld -- census map=fork ...
//! cargo run --release --example larderworld -- census gpu=both seeds=1-32 threads=1 ...
//! ```
//!
//! **On the GPU engine** (`gpu=1`, or `gpu=both` for each game on both;
//! 2026-11-12, the technical lead review): the same hands, seeds and rules
//! on `gpu::larder_world::GpuWorld`, the engine the window plays. The two
//! run the same rules (the twin tests) but add floats in other orders, so
//! one seed's games part after a few seconds; what must agree is the spread
//! of outcomes. With `both` each hand's scores on the two are compared by a
//! two-sided Mann-Whitney test (Holm over the hands), and the paired sign
//! tests are printed for each engine, so a plan that wins on the CPU and
//! not on the GPU shows. A GPU game makes its own device on each thread;
//! on software Vulkan use `threads=1` (it already uses every core).
//!
//! The game is `mould::larder_world` on the CPU race world
//! (`larder_world::race_world`): 23 crystals that run dry, rock, wet
//! ground, and a growing mould that starts as a blob on the home crystal
//! with the squad of 4 on the crystal farthest from it. With `map=NAME`
//! (a map in `Reports/larder-maps-2026-11-11/maps`, or `map=DIR/NAME`) it
//! is that designed map instead (`BigDish::with_map`, ROADMAP WM1), on the
//! race world's mould settings; the seed then varies the mould, not the map.
//!
//! **Deciding measure, named before the run:** the game's one ordered
//! number a round (`Game::score`, v2 since 2026-11-12: a squad win beats a
//! stalemate at the cap beats the squad's loss; a win or loss is sooner and
//! more soldiers left; a stalemate is the squad's share of the charge taken
//! from the crystals and the mould's size against its start), paired by
//! seed, each plan against idle and against random, two-sided sign test,
//! Holm over every comparison made. Each row also prints `score1`, W2b's
//! score (`Game::score_w2b`: the crystals counted at the cap), to pair with
//! older censuses. `home_dry=1` turns on `Rules::home_dry`. `seek=0` turns off `Rules::seek` (a harvester back from a dry crystal walks on to the next safe one).
//!
//! **Hands** (fixed rules; settings named here):
//! - **idle**: never gives an order. The soldiers stand at the larder on
//!   Defend.
//! - **random**: every 5 game seconds one random live soldier gets a random
//!   order (a move to a random open point, a clear of a random crystal, a
//!   burn line through a random point, or an attack) and a random stance;
//!   with chance 1/4 it buys, and with 1/8 it lays a wall of random angle
//!   10 dish cells long at a random point in range.
//! - **farm**: soldier k clears the k-th nearest crystal with charge to the
//!   larder (the squad's own first), on Defend; every bank over
//!   `RESERVE` buys a soldier for the next crystal, up to `FARM_MAX`.
//! - **rush**: the whole squad attacks the mould's core at once, buying
//!   soldiers with whatever the bank holds over `RESERVE` and sending them
//!   too.
//! - **wall**: farm, and from 5 s until it stands a wall `WALL_LEN` dish cells long
//!   across the line from the core to the larder, `WALL_AT` of the way to
//!   the larder (the expander review: a wall across the leader's road
//!   was the one tool measured to turn games).
//! - Any other name is the player AI (`larder_world_ai::Style::named`):
//!   boom, claim, late, fort, strike, with knobs after a colon
//!   (`claim:claim_at=200+keep=4`). Each is also paired with farm, the
//!   best scripted hand.
use pixels::wgpu;
use slime_mould::gpu::block_on;
use slime_mould::gpu::larder_world::{race_settings, GpuWorld};
use slime_mould::mould::big::BigDish;
use slime_mould::mould::big::BigParams;
use slime_mould::mould::larder_world::{race_params, Cmd, End, Game, Order, Rules, Stance, World, FLEE_KEEP};
use slime_mould::mould::larder_world_ai;
use slime_mould::mould::map::Map;
use slime_mould::mould::tiers::Tier;

const RESERVE: f32 = 600.0;
const FARM_MAX: usize = 8;
const WALL_LEN: f32 = 26.0;
const WALL_AT: f32 = 0.6;

#[derive(Clone, Default)]
struct Row {
    /// Score v2 (`Game::score`) and W2b's (`Game::score_w2b`).
    score: f32,
    score1: f32,
    end: String,
    t: f32,
    alive: usize,
    mould: usize,
    bank: f32,
    harvested: f32,
    stolen: f32,
    shots: u32,
    killed: u32,
    crystals_left: f32,
    ms_tick: f64,
    /// The AI's mean and slowest think, microseconds.
    think_mean: f64,
    think_us: f64,
    /// Does the mould come for the squad (owner's playtest, 2026-11-12:
    /// "the slime mostly ignored my soldiers")? Living walkers within
    /// `NEAR` dish cells of any live soldier, the mean over the game's
    /// seconds while a soldier lives (CPU only: 0 on the GPU engine, which
    /// reads no walkers back); and the first second a walker touches a
    /// soldier (the cap if none ever does).
    near: f64,
    touch_at: f32,
    /// Stuck seconds (design review, 2026-11-12: no soft lock): soldiers
    /// alive, none fired or took food that second, and the bank cannot buy
    /// a soldier. The owner's playtest hit this when shots cost food.
    stuck: u32,
}

/// Dish cells round a soldier that count as near (`Row::near`): 10, two
/// thirds of a soldier's range.
const NEAR: f32 = 10.0;

/// The mould's settings over the race world's, from the command line.
#[derive(Clone, Copy, Default)]
struct Mould {
    /// A mould tier (`tier=swarmer`: the alarm that shots lay draws the
    /// mould to the fight). CPU only.
    tier: Option<Tier>,
    /// The mould's turn from gunfire, opt-in (`larder_world::FLEE_KEEP`
    /// and `FLEE_WEIGHT`): `flee=W` turns the alarm field on and reads it
    /// at -W; `flee=0` (the default) leaves it off. Applied before `tier=`,
    /// so a tier's own alarm keys win.
    flee: Option<f32>,
    /// `flee_keep=K`: the alarm field's keep a fine step for the plain
    /// mould (`FLEE_KEEP`, 0.98); a tier's own keep wins.
    flee_keep: Option<f32>,
    /// `scorch_full=F` and `scorch_keep=K`: the crater's strength
    /// (`BigParams::scorch_full`, `scorch_keep`), for `crater=S`.
    scorch_full: Option<u16>,
    scorch_keep: Option<f32>,
}

impl Mould {
    /// The alarm's keep and weight these settings give, from `(keep, weight)`.
    fn alarm(&self, (keep, weight): (f32, f32)) -> (f32, f32) {
        let (mut k, w) = match self.flee {
            Some(w) if w <= 0.0 => (0.0, 0.0),
            Some(w) => (FLEE_KEEP, -w),
            None => (keep, weight),
        };
        if let Some(fk) = self.flee_keep.filter(|_| k > 0.0) {
            k = fk;
        }
        (k, w)
    }
    fn apply(&self, p: &mut BigParams) {
        (p.alarm_keep, p.alarm_weight) = self.alarm((p.alarm_keep, p.alarm_weight));
        if let Some(t) = self.tier {
            t.apply(p);
        }
        if let Some(f) = self.scorch_full {
            p.scorch_full = f;
        }
        if let Some(k) = self.scorch_keep {
            p.scorch_keep = k;
        }
    }
}

fn main() {
    let mut seeds: Vec<u64> = (1..=4).collect();
    let (mut scale, mut walkers, mut threads, mut cap) = (2usize, 5_000usize, 4usize, None::<f32>);
    let mut hands: Vec<String> = ["idle", "random", "farm", "rush", "wall"].iter().map(|s| s.to_string()).collect();
    let (mut look, mut every, mut census) = (None::<String>, 60.0f32, false);
    let mut rules = Rules::default();
    let mut m = Mould::default();
    let mut map = None::<(String, Map)>;
    // The engines each game runs on: false the CPU big dish, true the GPU.
    let mut engines = vec![false];
    for a in std::env::args().skip(1) {
        if a == "census" {
            census = true;
            continue;
        }
        let (k, v) = a.split_once('=').expect("key=value");
        match k {
            "seeds" => {
                let (a, b) = v.split_once('-').unwrap_or((v, v));
                seeds = (a.parse().unwrap()..=b.parse().unwrap()).collect();
            }
            "scale" => scale = v.parse().unwrap(),
            "walkers" => walkers = v.parse().unwrap(),
            "threads" => threads = v.parse().unwrap(),
            "cap" => cap = Some(v.parse().unwrap()),
            "hands" => hands = v.split(',').map(String::from).collect(),
            "look" => look = Some(v.into()),
            "every" => every = v.parse().unwrap(),
            "aim_min" => rules.aim_min = v.parse().unwrap(),
            "price" => rules.price = v.parse().unwrap(),
            "smell" => rules.smell = v.parse().unwrap(),
            "smell_r" => rules.smell_r = v.parse().unwrap(),
            "tier" => m.tier = Some(Tier::from_name(v).unwrap_or_else(|| panic!("no tier {v}"))),
            "flee" => m.flee = Some(v.parse().unwrap()),
            "flee_keep" => m.flee_keep = Some(v.parse().unwrap()),
            // `crater=S`: game seconds of crater each hit lays
            // (`Rules::crater`).
            "crater" => rules.crater = v.parse().unwrap(),
            "scorch_full" => m.scorch_full = Some(v.parse().unwrap()),
            "scorch_keep" => m.scorch_keep = Some(v.parse().unwrap()),
            "strike" => rules.strike = v.parse().unwrap(),
            "home_dry" => rules.home_dry = v == "1",
            "seek" => rules.seek = v == "1",
            "gpu" => {
                engines = match v {
                    "0" => vec![false],
                    "1" => vec![true],
                    "both" => vec![false, true],
                    _ => panic!("gpu=0, 1 or both"),
                }
            }
            "map" => {
                let path = std::path::Path::new(v);
                let dir = match path.parent() {
                    Some(d) if !d.as_os_str().is_empty() => d.to_path_buf(),
                    _ => std::path::Path::new(env!("CARGO_MANIFEST_DIR")).join("Reports/larder-maps-2026-11-11/maps"),
                };
                let name = path.file_name().unwrap().to_str().unwrap().to_string();
                map = Some((name.clone(), Map::load(&dir, &name).unwrap_or_else(|e| panic!("{e}"))));
            }
            _ => panic!("unknown {k}"),
        }
    }
    if let Some(c) = cap {
        rules.cap = c;
    }
    let world = map.as_ref().map_or("the race world".to_string(), |(n, _)| format!("map {n}"));
    let names = engines.iter().map(|&g| engine_name(g)).collect::<Vec<_>>().join(" and ");
    println!(
        "larderworld {world}, scale {scale} walkers {walkers} cap {} s, tps {}, seeds {:?}; on {names}; tier {}; flee {}{}{}; rules {rules:?}",
        rules.cap,
        rules.tps,
        seeds,
        m.tier.map_or("none", |t| t.name()),
        m.flee.map_or("0 (default)".to_string(), |w| w.to_string()),
        m.flee_keep.map_or(String::new(), |k| format!(", keep {k}")),
        match (m.scorch_full, m.scorch_keep) {
            (None, None) => String::new(),
            (f, k) => format!("; scorch full {f:?} keep {k:?}"),
        }
    );
    assert!(m.tier.is_none() || engines == [false], "tier= plays on the CPU only");
    let map = map.as_ref().map(|(_, m)| m);
    if let Some(dir) = look {
        assert!(engines == [false], "look= draws the CPU world only");
        std::fs::create_dir_all(&dir).expect("look dir");
        for &seed in &seeds {
            for hand in &hands {
                let r = play(seed, scale, walkers, rules, m, hand, map, Some((&dir, every)), None);
                println!("{hand} seed {seed}: {} at {:.0} s, score {:+.3}", r.end, r.t, r.score);
            }
        }
        return;
    }
    if !census {
        eprintln!("census or look=DIR");
        return;
    }
    // Every (seed, hand, engine) game on `threads` threads, each thread with
    // its own GPU device once it plays a GPU game.
    let mut jobs: Vec<(u64, String, bool)> = Vec::new();
    for &s in &seeds {
        for h in &hands {
            for &g in &engines {
                jobs.push((s, h.clone(), g));
            }
        }
    }
    let next = std::sync::atomic::AtomicUsize::new(0);
    let out = std::sync::Mutex::new(vec![Row::default(); jobs.len()]);
    std::thread::scope(|sc| {
        for _ in 0..threads {
            sc.spawn(|| {
                let mut dev: Option<(wgpu::Device, wgpu::Queue)> = None;
                loop {
                    let j = next.fetch_add(1, std::sync::atomic::Ordering::Relaxed);
                    if j >= jobs.len() {
                        break;
                    }
                    let (seed, hand, on_gpu) = &jobs[j];
                    if *on_gpu && dev.is_none() {
                        dev = Some(gpu());
                    }
                    let r = play(*seed, scale, walkers, rules, m, hand, map, None, dev.as_ref().filter(|_| *on_gpu));
                    println!(
                        "{hand:>6} seed {seed:>2}{}: {:>11} at {:>5.0} s, score {:+.3}; soldiers {:>2}, mould {:>6}, bank {:>6.0}, harvested {:>6.0}, stolen {:>5.0}, shots {:>5}, killed {:>6}, crystals left {:.3}; {:.2} ms a tick, think {:.0} us mean, {:.0} at most; score1 {:+.3}; near {:.0}, touched at {:.0} s, stuck {} s",
                        if engines.len() > 1 { format!(" {}", engine_name(*on_gpu)) } else { String::new() },
                        r.end, r.t, r.score, r.alive, r.mould, r.bank, r.harvested, r.stolen, r.shots, r.killed, r.crystals_left, r.ms_tick, r.think_mean, r.think_us, r.score1, r.near, r.touch_at, r.stuck
                    );
                    out.lock().unwrap()[j] = r;
                }
            });
        }
    });
    let out = out.into_inner().unwrap();
    for &on_gpu in &engines {
        if engines.len() > 1 {
            println!("\n==== on {} ====", engine_name(on_gpu));
        }
        let get = |seed: u64, hand: &str| out[jobs.iter().position(|(s, h, g)| *s == seed && h == hand && *g == on_gpu).unwrap()].clone();
        report(&hands, &seeds, &get);
    }
    if engines.len() > 1 {
        let get = |seed: u64, hand: &str, g: bool| out[jobs.iter().position(|(s, h, e)| *s == seed && h == hand && *e == g).unwrap()].clone();
        compare(&hands, &seeds, &get);
    }
}

fn engine_name(on_gpu: bool) -> &'static str {
    if on_gpu {
        "the GPU engine"
    } else {
        "the CPU"
    }
}

/// A GPU device with the adapter's limits (the engine binds 48 buffers).
fn gpu() -> (wgpu::Device, wgpu::Queue) {
    let instance = wgpu::Instance::new(wgpu::InstanceDescriptor::new_without_display_handle_from_env());
    let adapter = block_on(instance.request_adapter(&wgpu::RequestAdapterOptions { power_preference: wgpu::PowerPreference::HighPerformance, ..Default::default() })).expect("no GPU adapter");
    eprintln!("larderworld: GPU {}", adapter.get_info().name);
    block_on(adapter.request_device(&wgpu::DeviceDescriptor { label: Some("larderworld"), required_limits: adapter.limits(), ..Default::default() })).expect("GPU device")
}

/// **The two engines' spreads** (`gpu=both`): each hand's scores on the
/// CPU against the GPU engine, a two-sided Mann-Whitney test with ties
/// shared, Holm over the hands; the medians and the median paired
/// difference beside it (pairs only share a start: games part within
/// seconds).
fn compare(hands: &[String], seeds: &[u64], get: &dyn Fn(u64, &str, bool) -> Row) {
    println!("\nCPU against GPU, by hand (score: median CPU, median GPU, median GPU - CPU by seed; Mann-Whitney two-sided, Holm over {}):", hands.len());
    let mut tests = Vec::new();
    for h in hands {
        let a: Vec<f64> = seeds.iter().map(|&s| get(s, h, false).score as f64).collect();
        let b: Vec<f64> = seeds.iter().map(|&s| get(s, h, true).score as f64).collect();
        let d: Vec<f64> = a.iter().zip(&b).map(|(x, y)| y - x).collect();
        tests.push((h.clone(), median(&a), median(&b), median(&d), mann_whitney_p(&a, &b)));
    }
    let holds = holm(&tests.iter().map(|t| t.4).collect::<Vec<_>>());
    for (i, (h, ma, mb, md, p)) in tests.iter().enumerate() {
        println!("  {h:>16}: CPU {ma:+.3}, GPU {mb:+.3}, difference {md:+.3}; p {p:.4}{}", if holds[i] { ", differs" } else { "" });
    }
    let ends = |g: bool, b: &str| -> usize { hands.iter().map(|h| seeds.iter().filter(|&&s| get(s, h, g).end == b).count()).sum() };
    println!("  ends, CPU / GPU: mould wiped {} / {}, cap {} / {}, squad wiped {} / {}", ends(false, "mould wiped"), ends(true, "mould wiped"), ends(false, "cap"), ends(true, "cap"), ends(false, "squad wiped"), ends(true, "squad wiped"));
}

fn median(v: &[f64]) -> f64 {
    let mut v = v.to_vec();
    v.sort_by(f64::total_cmp);
    match v.len() {
        0 => 0.0,
        n if n % 2 == 1 => v[n / 2],
        n => (v[n / 2 - 1] + v[n / 2]) / 2.0,
    }
}

/// Two-sided Mann-Whitney U test, normal approximation with the tie
/// correction and a continuity correction (fine from about 8 a side).
fn mann_whitney_p(a: &[f64], b: &[f64]) -> f64 {
    let (n1, n2) = (a.len() as f64, b.len() as f64);
    let mut all: Vec<(f64, bool)> = a.iter().map(|&x| (x, false)).chain(b.iter().map(|&x| (x, true))).collect();
    all.sort_by(|x, y| x.0.total_cmp(&y.0));
    let n = all.len();
    let (mut r1, mut ties) = (0.0, 0.0);
    let mut i = 0;
    while i < n {
        let mut j = i;
        while j + 1 < n && all[j + 1].0 == all[i].0 {
            j += 1;
        }
        let rank = (i + j) as f64 / 2.0 + 1.0;
        let t = (j - i + 1) as f64;
        ties += t * t * t - t;
        r1 += all[i..=j].iter().filter(|x| !x.1).count() as f64 * rank;
        i = j + 1;
    }
    let u = r1 - n1 * (n1 + 1.0) / 2.0;
    let mean = n1 * n2 / 2.0;
    let nn = n1 + n2;
    let var = n1 * n2 / 12.0 * ((nn + 1.0) - ties / (nn * (nn - 1.0)));
    if var <= 0.0 {
        return 1.0;
    }
    let z = ((u - mean).abs() - 0.5).max(0.0) / var.sqrt();
    (2.0 * (1.0 - normal_cdf(z))).min(1.0)
}

fn normal_cdf(z: f64) -> f64 {
    // Abramowitz and Stegun 7.1.26 for erf, |error| under 1.5e-7.
    let x = z / std::f64::consts::SQRT_2;
    let t = 1.0 / (1.0 + 0.327_591_1 * x.abs());
    let y = 1.0 - (((((1.061_405_429 * t - 1.453_152_027) * t) + 1.421_413_741) * t - 0.284_496_736) * t + 0.254_829_592) * t * (-x * x).exp();
    0.5 * (1.0 + if x >= 0.0 { y } else { -y })
}

/// Holm's step-down at 0.05: which of `ps` hold.
fn holm(ps: &[f64]) -> Vec<bool> {
    let mut order: Vec<usize> = (0..ps.len()).collect();
    order.sort_by(|&a, &b| ps[a].total_cmp(&ps[b]));
    let m = ps.len();
    let mut holds = vec![false; m];
    for (rank, &i) in order.iter().enumerate() {
        if ps[i] * (m - rank) as f64 <= 0.05 {
            holds[i] = true;
        } else {
            break;
        }
    }
    holds
}

/// One engine's census: each hand's mean and ends, then each plan paired by
/// seed against idle, random and farm (sign test, Holm).
fn report(hands: &[String], seeds: &[u64], get: &dyn Fn(u64, &str) -> Row) {
    println!("\nby hand (mean score; ends):");
    for h in hands {
        let rows: Vec<Row> = seeds.iter().map(|&s| get(s, h)).collect();
        let mean = rows.iter().map(|r| r.score).sum::<f32>() / rows.len() as f32;
        let ends = |b: &str| rows.iter().filter(|r| r.end == b).count();
        let near = rows.iter().map(|r| r.near).sum::<f64>() / rows.len() as f64;
        let touch = rows.iter().map(|r| r.touch_at).sum::<f32>() / rows.len() as f32;
        let stuck = rows.iter().map(|r| r.stuck as f32).sum::<f32>() / rows.len() as f32;
        let stuck_games = rows.iter().filter(|r| r.stuck >= 60).count();
        println!("  {h:>6}: mean {mean:+.3}; mould wiped {}, cap {}, squad wiped {}; near {near:.0} walkers, first touch {touch:.0} s; stuck {stuck:.0} s a game, {stuck_games} games stuck a minute or more", ends("mould wiped"), ends("cap"), ends("squad wiped"));
    }
    let mut tests = Vec::new();
    for h in hands.iter().filter(|h| *h != "idle" && *h != "random") {
        let scripted = ["farm", "rush", "wall"].contains(&h.as_str());
        for base in ["idle", "random", "farm"] {
            if base == "farm" && scripted {
                continue;
            }
            if !hands.iter().any(|x| x == base) {
                continue;
            }
            let (mut w, mut l) = (0, 0);
            for &s in seeds {
                let (a, b) = (get(s, h).score, get(s, base).score);
                if a > b {
                    w += 1;
                } else if a < b {
                    l += 1;
                }
            }
            tests.push((format!("{h} vs {base}"), w, l, sign_p(w, w + l)));
        }
    }
    // Holm over every comparison.
    let mut order: Vec<usize> = (0..tests.len()).collect();
    order.sort_by(|&a, &b| tests[a].3.total_cmp(&tests[b].3));
    let m = tests.len();
    let mut holds = vec![false; m];
    for (rank, &i) in order.iter().enumerate() {
        if tests[i].3 * (m - rank) as f64 <= 0.05 {
            holds[i] = true;
        } else {
            break;
        }
    }
    println!("\npaired by seed (score higher / lower, ties left out; sign test, Holm over {m}):");
    for (i, (name, w, l, p)) in tests.iter().enumerate() {
        println!("  {name:>16}: {w} higher, {l} lower, p {p:.4}{}", if holds[i] { ", holds" } else { "" });
    }
}

/// One game: on the CPU big dish, or with `gpu` on the GPU engine
/// (`GpuWorld`, the race world's settings and the same map).
#[allow(clippy::too_many_arguments)]
fn play(seed: u64, scale: usize, walkers: usize, rules: Rules, mould: Mould, hand: &str, map: Option<&Map>, look: Option<(&str, f32)>, gpu: Option<&(wgpu::Device, wgpu::Queue)>) -> Row {
    if let Some((device, queue)) = gpu {
        let mut s = race_settings(seed, scale, walkers);
        (s.alarm_keep, s.alarm_weight) = mould.alarm((s.alarm_keep, s.alarm_weight));
        let world = match map {
            Some(m) => GpuWorld::with_map(device, queue, s, scale, m),
            None => GpuWorld::new(device, queue, s, scale),
        };
        return play_on(Game::new(world, rules, seed), seed, hand, None, &|w: &GpuWorld| w.e.dish.pile_full.iter().sum(), &|_, _, _| None);
    }
    let mut p = race_params(seed, scale, walkers);
    mould.apply(&mut p);
    let mut world = match map {
        Some(m) => BigDish::with_map(p, m),
        None => BigDish::new(p),
    };
    world.threads = 1;
    let draw = |g: &Game<BigDish>, path: &str| {
        picture(g, path);
        counts(g, hand, seed);
    };
    let near = |d: &BigDish, live: &[(f32, f32)], r: f32| Some((0..d.walkers.len()).filter(|&k| d.lives(k) && live.iter().any(|&(x, y)| (d.walkers[k].0 - x).hypot(d.walkers[k].1 - y) <= r)).count());
    play_on(Game::new(world, rules, seed), seed, hand, look.map(|(dir, every)| -> Look<BigDish> { (dir, every, &draw) }), &|w: &BigDish| w.pile_full.iter().sum(), &near)
}

/// Living walkers within a radius (fine cells) of any of the points, where
/// the world can count them (`Row::near`).
type Near<'a, W> = &'a dyn Fn(&W, &[(f32, f32)], f32) -> Option<usize>;

/// Pictures: the folder, game seconds between them, and what draws one.
type Look<'a, W> = (&'a str, f32, &'a dyn Fn(&Game<W>, &str));

/// `play`'s game run to its end with `hand`, on any world; with `look`, a
/// picture every so many game seconds and at the end. `full` is the
/// crystals' full charge (`pile_full`: the GPU world's from its CPU mirror,
/// which builds the same walls).
fn play_on<W: World>(mut g: Game<W>, seed: u64, hand: &str, look: Option<Look<W>>, full: &dyn Fn(&W) -> f32, near: Near<W>) -> Row {
    let start = std::time::Instant::now();
    let mut shot = -1.0f32;
    let mut farmed: Vec<usize> = Vec::new();
    let mut ai = larder_world_ai::Hand::named(hand, &g);
    let (mut think_us, mut think_sum, mut thinks) = (0.0f64, 0.0f64, 0u32);
    let (mut near_sum, mut near_n, mut touch_at) = (0.0f64, 0u32, None::<f32>);
    let end = g.play(|g| {
        let t = g.time();
        {
            let live: Vec<(f32, f32)> = g.soldiers.iter().filter(|s| s.alive).map(|s| (s.x, s.y)).collect();
            if !live.is_empty() {
                if let Some(n) = near(&g.world, &live, NEAR * g.world.scale()) {
                    near_sum += n as f64;
                    near_n += 1;
                }
            }
            if touch_at.is_none() && g.soldiers.iter().any(|s| s.alive && s.touching > 0) {
                touch_at = Some(t);
            }
        }
        if let Some((dir, every, draw)) = look {
            if t - shot >= every || shot < 0.0 {
                shot = t;
                draw(g, &format!("{dir}/{hand}-s{seed}-{:04.0}s.png", t));
            }
        }
        match hand {
            "idle" => {}
            "random" => random_hand(g),
            "farm" => farm_hand(g, &mut farmed),
            "rush" => rush_hand(g),
            "wall" => {
                // Until the wall stands, one soldier walks to it and the
                // rest farm (the first version tried once at 5 s, failed
                // out of range, and the farm hand took the walker back:
                // on seeds 1 and 3 no wall was ever built).
                if g.walls == 0 && t >= 5.0 {
                    wall_road(g);
                }
                farm_hand(g, &mut farmed);
            }
            _ => {
                let t0 = std::time::Instant::now();
                ai.as_mut().unwrap_or_else(|| panic!("unknown hand {hand}")).play(g);
                let us = t0.elapsed().as_secs_f64() * 1e6;
                think_us = think_us.max(us);
                think_sum += us;
                thinks += 1;
            }
        }
    });
    if let Some((dir, _, draw)) = look {
        draw(&g, &format!("{dir}/{hand}-s{seed}-end-{:04.0}s.png", g.time()));
    }
    let ms = start.elapsed().as_secs_f64() * 1000.0 / g.world.ticks().max(1) as f64;
    let left: f32 = g.world.crystals().iter().map(|c| c.1).sum::<f32>() / full(&g.world).max(1.0);
    Row {
        score: g.score().expect("ended"),
        score1: g.score_w2b().expect("ended"),
        end: end.band().into(),
        t: match end {
            End::SquadWiped(t) | End::MouldWiped(t) | End::Overrun(t) => t,
            End::Cap(_) => g.rules.cap,
        },
        alive: g.alive(),
        mould: g.world.living(),
        bank: g.bank,
        harvested: g.harvested,
        stolen: g.stolen,
        shots: g.shots,
        killed: g.killed,
        crystals_left: left,
        ms_tick: ms,
        think_mean: think_sum / thinks.max(1) as f64,
        think_us,
        near: near_sum / near_n.max(1) as f64,
        touch_at: touch_at.unwrap_or(g.rules.cap),
        // Counted by the game since 2026-11-12 (`Game::stuck`), the same
        // test the census ran here (it can count the round's last second,
        // which the plan never saw).
        stuck: g.stuck,
    }
}

fn random_hand<W: World>(g: &mut Game<W>) {
    if !(g.time() as u64).is_multiple_of(5) {
        return;
    }
    let live: Vec<usize> = (0..g.soldiers.len()).filter(|&k| g.soldiers[k].alive).collect();
    if live.is_empty() {
        g.apply(Cmd::Buy);
        return;
    }
    let (w, h) = g.world.size();
    let k = live[g.rng.below(live.len() as u32) as usize];
    let point = |g: &mut Game<W>| loop {
        let p = (g.rng.f32() * w as f32, g.rng.f32() * h as f32);
        if g.world.open(p.0, p.1) {
            break p;
        }
    };
    let order = match g.rng.below(4) {
        0 => Order::Move(point(g).0, point(g).1),
        1 => {
            let cs = g.world.crystals();
            let c = cs[g.rng.below(cs.len() as u32) as usize].0;
            Order::Clear(c.0, c.1)
        }
        2 => {
            let p = point(g);
            let a = g.rng.f32() * std::f32::consts::TAU;
            let l = 5.0 * g.world.scale();
            Order::Burn((p.0 - a.cos() * l, p.1 - a.sin() * l), (p.0 + a.cos() * l, p.1 + a.sin() * l))
        }
        _ => Order::Attack,
    };
    g.apply(Cmd::Order(k, order));
    let stance = [Stance::HoldFire, Stance::Defend, Stance::Free][g.rng.below(3) as usize];
    g.apply(Cmd::Stance(k, stance));
    if g.rng.below(4) == 0 {
        g.apply(Cmd::Buy);
    }
    if g.rng.below(8) == 0 {
        let s = g.soldiers[k];
        let r = g.rules.range * g.world.scale() * g.rng.f32();
        let (a, b) = (g.rng.f32() * std::f32::consts::TAU, g.rng.f32() * std::f32::consts::TAU);
        let m = (s.x + r * a.cos(), s.y + r * a.sin());
        let l = 5.0 * g.world.scale();
        g.apply(Cmd::Wall((m.0 - l * b.cos(), m.1 - l * b.sin()), (m.0 + l * b.cos(), m.1 + l * b.sin())));
    }
}

/// Crystals with charge, nearest the larder first (the core last of all).
fn by_distance<W: World>(g: &Game<W>) -> Vec<(f32, f32)> {
    let l = g.larder;
    // A crystal under one frame's full ask is dry: the soldier the game
    // sent home from it (`Rules::home_dry`) is not sent back for crumbs.
    let r = g.rules;
    let dry = r.harvest * r.frame_ticks as f32 / r.tps as f32 * r.charge;
    let mut cs: Vec<(f32, f32)> = g.world.crystals().iter().filter(|c| c.1 >= dry).map(|c| c.0).filter(|c| *c != g.core).collect();
    cs.sort_by(|a, b| (a.0 - l.0).hypot(a.1 - l.1).total_cmp(&(b.0 - l.0).hypot(b.1 - l.1)));
    cs
}

fn farm_hand<W: World>(g: &mut Game<W>, farmed: &mut Vec<usize>) {
    let cs = by_distance(g);
    if g.bank > g.rules.soldier_cost + RESERVE && g.alive() < FARM_MAX {
        g.apply(Cmd::Buy);
    }
    // Each live soldier without a crystal, or on a dry one, takes the
    // nearest crystal no other soldier is clearing.
    let mut taken: Vec<(f32, f32)> = Vec::new();
    for k in 0..g.soldiers.len() {
        let s = g.soldiers[k];
        if !s.alive {
            continue;
        }
        // The wall hand's builder, until the wall stands.
        if matches!(s.order, Order::Move(..)) && g.walls == 0 {
            continue;
        }
        if let Order::Clear(x, y) = s.order {
            if let Some((_, c)) = g.crystal_at((x, y)) {
                if cs.contains(&c) && !taken.contains(&c) {
                    taken.push(c);
                    continue;
                }
            }
        }
        if let Some(&c) = cs.iter().find(|c| !taken.contains(c)) {
            taken.push(c);
            g.apply(Cmd::Order(k, Order::Clear(c.0, c.1)));
            if !farmed.contains(&k) {
                farmed.push(k);
                g.apply(Cmd::Stance(k, Stance::Defend));
            }
        }
    }
}

fn rush_hand<W: World>(g: &mut Game<W>) {
    while g.bank > g.rules.soldier_cost + RESERVE && g.alive() < g.rules.max_squad {
        if !g.apply(Cmd::Buy) {
            break;
        }
    }
    for k in 0..g.soldiers.len() {
        if g.soldiers[k].alive && g.soldiers[k].order != Order::Attack {
            g.apply(Cmd::Order(k, Order::Attack));
        }
    }
}

/// A wall across the core-to-larder line, `WALL_AT` of the way to the
/// larder, built by sending soldier 0 there first if it is out of range.
fn wall_road<W: World>(g: &mut Game<W>) {
    let (c, l) = (g.core, g.larder);
    let m = (c.0 + (l.0 - c.0) * WALL_AT, c.1 + (l.1 - c.1) * WALL_AT);
    let (dx, dy) = (l.0 - c.0, l.1 - c.1);
    let d = dx.hypot(dy).max(1.0);
    let h = WALL_LEN * g.world.scale() / 2.0;
    let (a, b) = ((m.0 - dy / d * h, m.1 + dx / d * h), (m.0 + dy / d * h, m.1 - dx / d * h));
    if !g.apply(Cmd::Wall(a, b)) {
        // Out of range: walk the first live soldier there (soldier 0 stands
        // on the point itself, the others round it) and try again next
        // second.
        // It stands 3 dish cells on the larder's side, off the wall's line,
        // with both ends in range (`WALL_LEN` is short of twice the range).
        let at = (m.0 + dx / d * 3.0 * g.world.scale(), m.1 + dy / d * 3.0 * g.world.scale());
        if let Some(k) = (0..g.soldiers.len()).find(|&k| g.soldiers[k].alive) {
            if g.soldiers[k].order != Order::Move(at.0, at.1) {
                g.apply(Cmd::Order(k, Order::Move(at.0, at.1)));
            }
        }
    }
}

/// The world as the engine draws it, with each live soldier as a white
/// square and the larder as a yellow ring: marks for the reader, not the
/// simulation.
/// The counts that go beside a picture.
fn counts(g: &Game<BigDish>, hand: &str, seed: u64) {
    println!(
        "  {hand} seed {seed} at {:.0} s: mould {}, soldiers {}, bank {:.0}, harvested {:.0}, killed {}, crystals left {:.3}",
        g.time(),
        g.world.living(),
        g.soldiers.iter().filter(|s| s.alive).count(),
        g.bank,
        g.harvested,
        g.killed,
        g.world.crystals_left()
    );
}

fn picture(g: &Game<BigDish>, path: &str) {
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
    // The last frame's shots: red to the slime they hit, dim where they
    // found none.
    for sh in &g.last_shots {
        let (end, c) = match sh.hit {
            Some(p) => (p, [255, 60, 60, 255]),
            None => (sh.to, [110, 40, 40, 255]),
        };
        let n = (end.0 - sh.from.0).hypot(end.1 - sh.from.1).ceil().max(1.0) as i32;
        for k in 0..=n {
            let t = k as f32 / n as f32;
            put((sh.from.0 + (end.0 - sh.from.0) * t) as i32, (sh.from.1 + (end.1 - sh.from.1) * t) as i32, c);
        }
    }
    let r = 4.0 * g.world.scale();
    for i in 0..64 {
        let a = i as f32 / 64.0 * std::f32::consts::TAU;
        put((g.larder.0 + r * a.cos()) as i32, (g.larder.1 + r * a.sin()) as i32, [255, 220, 60, 255]);
    }
    img.save(path).expect("save png");
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
