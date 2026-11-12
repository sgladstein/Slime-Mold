//! **Rounds census** (`mould::rounds`, 2026-11-12): every soldier carries
//! the same round (a loadout of up to four cards) while one hand plays, for
//! each mould tier, on the CPU race world.
//!
//! ```text
//! cargo run --release --example rounds -- seeds=5-20 [tiers=none,swarmer,... loads=none,blast,... cap=600 threads=4 look=DIR]
//! ```
//!
//! **Deciding measure, named before the run:** `Game::score` (v2) a game,
//! paired by seed. Every arm plays the same hand (`examples/kitplans.rs`'s
//! farm hand on Defend), so the round is all that differs. Three readings,
//! named before the run (the technical lead's and game designer's reviews,
//! 11-12 15:31Z):
//! 1. *Each card against no card*: each single-card round (`blast`, `gas`,
//!    `germ`) and each modifier or trigger on a blast (`wide+blast`, ...)
//!    against `none`, and the modifier rounds against plain `blast`.
//! 2. *Planned against random*: each planned loadout against `rand` (a
//!    random round of 1-4 cards drawn from the seed, the same draw for
//!    every tier), sign test, Holm over every comparison in the table.
//! 3. *Counter-play*: the best loadout by mean differs between tiers, and
//!    no loadout is within noise of the best on every tier.
//!
//! **Loadouts** (`loads=`, any `card+card` string): `none`; one card each
//! (`blast`, `gas`, `germ`); a blast with each modifier and trigger;
//! planned: `trap` (thick+wide+blast: a mine the mould sets off), `lull`
//! (quiet+gas+quiet+blast: put the front to sleep, then a silent blast),
//! `plague` (germ+delay+blast), `shock` (scatter+blast+wide+blast); and
//! `rand`.
use slime_mould::mould::big::BigDish;
use slime_mould::mould::larder_world::{race_params, Cmd, Game, Order, Rules, Stance, World};
use slime_mould::mould::larder_world_ai::Hand;
use slime_mould::mould::rounds::{self, Card, RoundRules, Rounds};
use slime_mould::mould::tiers::Tier;
use slime_mould::mould::Rng;

const RESERVE: f32 = 600.0;
const FARM_MAX: usize = 8;

#[derive(Clone, Default)]
struct Row {
    score: f32,
    end: String,
    t: f32,
    alive: usize,
    mould: usize,
    cast: [u32; 3],
    killed: usize,
    round_killed: usize,
    spent: f32,
    broke: u32,
    dropped: u32,
    ms_tick: f64,
    load: String,
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

/// `examples/kitplans.rs`'s farm hand.
fn farm_hand(g: &mut Game<BigDish>, farmed: &mut Vec<usize>) {
    let l = g.larder;
    let r = g.rules;
    let dry = r.harvest * r.frame_ticks as f32 / r.tps as f32 * r.charge;
    let mut cs: Vec<(f32, f32)> = g.world.crystals().iter().filter(|c| c.1 >= dry).map(|c| c.0).filter(|c| *c != g.core).collect();
    cs.sort_by(|a, b| (a.0 - l.0).hypot(a.1 - l.1).total_cmp(&(b.0 - l.0).hypot(b.1 - l.1)));
    if g.bank > g.rules.soldier_cost + RESERVE && g.alive() < FARM_MAX {
        g.apply(Cmd::Buy);
    }
    let mut taken: Vec<(f32, f32)> = Vec::new();
    for k in 0..g.soldiers.len() {
        let s = g.soldiers[k];
        if !s.alive {
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

/// A random round of 1-4 cards, drawn from the seed alone.
fn random_round(seed: u64) -> Vec<Card> {
    let mut rng = Rng::new(seed.wrapping_mul(0x9E37_79B9_7F4A_7C15) ^ 0x5EED);
    let n = 1 + rng.below(4) as usize;
    (0..n).map(|_| Card::ALL[rng.below(Card::ALL.len() as u32) as usize]).collect()
}

fn loadout(name: &str, seed: u64) -> Vec<Card> {
    let s = match name {
        "rand" => return random_round(seed),
        "trap" => "thick+wide+blast",
        "lull" => "quiet+gas+quiet+blast",
        "plague" => "germ+delay+blast",
        "shock" => "scatter+blast+wide+blast",
        s => s,
    };
    rounds::parse(s).unwrap_or_else(|| panic!("bad loadout {s}"))
}

fn play(seed: u64, tier: &str, load: &str, hand: &str, rules: Rules, rr: RoundRules, look: Option<&str>) -> Row {
    let mut p = race_params(seed, 2, 5000);
    if let Some(t) = Tier::from_name(tier) {
        t.apply(&mut p);
    }
    let mut world = BigDish::new(p);
    world.threads = 1;
    let mut g = Game::new(world, rules, seed);
    let round = loadout(load, seed);
    let mut rd = Rounds::new(round.clone(), rr);
    let mut farmed = Vec::new();
    let mut brain = (hand != "farm").then(|| Hand::named(hand, &g).unwrap_or_else(|| panic!("unknown hand {hand}")));
    let start = std::time::Instant::now();
    let tps = g.rules.tps as u64;
    let mut last = u64::MAX;
    let mut frame = 0u64;
    while g.end.is_none() {
        let sec = g.world.ticks() / tps;
        if sec != last {
            last = sec;
            match brain.as_mut() {
                Some(b) => {
                    b.play(&mut g);
                }
                None => farm_hand(&mut g, &mut farmed),
            }
        }
        g.step();
        rd.after_step(&mut g);
        // Frames for a GIF: every 5th step (a third of a game second) for
        // 60 game seconds from LOOK_FROM (default 30), full size, each
        // living soldier marked by a white 3 x 3 square (a unit, not an
        // effect: what the rounds did is only in the mould).
        if let Some(dir) = look {
            let t = g.time();
            let from: f32 = std::env::var("LOOK_FROM").ok().and_then(|v| v.parse().ok()).unwrap_or(30.0);
            if (from..from + 60.0).contains(&t) {
                if frame.is_multiple_of(5) {
                    let mut im = g.world.image(1);
                    for s in g.soldiers.iter().filter(|s| s.alive) {
                        for dy in -1i32..=1 {
                            for dx in -1i32..=1 {
                                let (x, y) = (s.x as i32 + dx, s.y as i32 + dy);
                                if x >= 0 && y >= 0 && (x as u32) < im.width() && (y as u32) < im.height() {
                                    im.put_pixel(x as u32, y as u32, image::Rgba([255, 255, 255, 255]));
                                }
                            }
                        }
                    }
                    im.save(format!("{dir}/{tier}-{load}-s{seed}-{frame:05}.png")).unwrap();
                }
                frame += 1;
            }
        }
    }
    Row {
        score: g.score().expect("ended"),
        end: g.end.unwrap().band().into(),
        t: g.time(),
        alive: g.alive(),
        mould: g.world.living(),
        cast: rd.cast,
        killed: g.killed as usize,
        round_killed: rd.killed,
        spent: rd.spent,
        broke: rd.broke,
        dropped: rd.dropped,
        ms_tick: start.elapsed().as_secs_f64() * 1000.0 / g.world.ticks().max(1) as f64,
        load: rounds::name(&round),
    }
}

fn holm(tests: &[(String, usize, usize, f64)]) -> Vec<bool> {
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
    holds
}

fn main() {
    let mut seeds: Vec<u64> = (5..=20).collect();
    let mut tiers: Vec<String> = ["none", "swarmer", "sleeper", "sporer", "wary", "grudge"].iter().map(|s| s.to_string()).collect();
    let mut loads: Vec<String> = ["none", "blast", "gas", "germ", "wide+blast", "scatter+blast", "quiet+blast", "delay+blast", "thick+blast", "trap", "lull", "plague", "shock", "rand"].iter().map(|s| s.to_string()).collect();
    let (mut threads, mut look) = (4usize, None::<String>);
    let mut hand = "farm".to_string();
    let mut rules = Rules { cap: 600.0, ..Rules::default() };
    let mut rr = RoundRules::default();
    for a in std::env::args().skip(1) {
        let (k, v) = a.split_once('=').expect("key=value");
        match k {
            "seeds" => {
                let (a, b) = v.split_once('-').unwrap_or((v, v));
                seeds = (a.parse().unwrap()..=b.parse().unwrap()).collect();
            }
            "tiers" => tiers = v.split(',').map(String::from).collect(),
            "loads" => loads = v.split(',').map(String::from).collect(),
            "threads" => threads = v.parse().unwrap(),
            "cap" => rules.cap = v.parse().unwrap(),
            "look" => look = Some(v.into()),
            "hand" => hand = v.into(),
            "recharge" => rr.recharge = v.parse().unwrap(),
            "blast_price" => rr.kit.blast_price = v.parse().unwrap(),
            "gas_price" => rr.kit.gas_price = v.parse().unwrap(),
            "germ_price" => rr.kit.germ_price = v.parse().unwrap(),
            _ => panic!("unknown {k}"),
        }
    }
    if let Some(d) = &look {
        std::fs::create_dir_all(d).unwrap();
    }
    println!("rounds census, race world, hand {hand}, cap {} s, seeds {seeds:?}, tiers {tiers:?}, loads {loads:?}; round rules {rr:?}", rules.cap);
    let mut jobs: Vec<(String, u64, String)> = Vec::new();
    for t in &tiers {
        for &s in &seeds {
            for l in &loads {
                jobs.push((t.clone(), s, l.clone()));
            }
        }
    }
    let next = std::sync::atomic::AtomicUsize::new(0);
    let out = std::sync::Mutex::new(vec![Row::default(); jobs.len()]);
    std::thread::scope(|sc| {
        for _ in 0..threads {
            sc.spawn(|| loop {
                let j = next.fetch_add(1, std::sync::atomic::Ordering::Relaxed);
                if j >= jobs.len() {
                    break;
                }
                let (tier, seed, load) = &jobs[j];
                let lk = if *seed == seeds[0] { look.as_deref() } else { None };
                let r = play(*seed, tier, load, &hand, rules, rr, lk);
                println!(
                    "{tier:>8} {load:>24} seed {seed:>2}: {:>11} at {:>4.0} s, score {:+.3}; soldiers {:>2}, mould {:>6}; round {}, casts {:?}, round kills {} of {}, spent {:.0}, broke {}, dropped {}; {:.2} ms a tick",
                    r.end, r.t, r.score, r.alive, r.mould, r.load, r.cast, r.round_killed, r.killed, r.spent, r.broke, r.dropped, r.ms_tick
                );
                out.lock().unwrap()[j] = r;
            });
        }
    });
    let out = out.into_inner().unwrap();
    let get = |t: &str, s: u64, l: &str| out[jobs.iter().position(|(a, b, c)| a == t && *b == s && c == l).unwrap()].clone();
    let mean = |t: &str, l: &str| seeds.iter().map(|&s| get(t, s, l).score).sum::<f32>() / seeds.len() as f32;
    let pair = |t: &str, a: &str, b: &str| {
        let (mut w, mut l) = (0, 0);
        for &s in &seeds {
            let (x, y) = (get(t, s, a).score, get(t, s, b).score);
            if x > y {
                w += 1;
            } else if x < y {
                l += 1;
            }
        }
        (w, l)
    };
    println!("\npayoff table (mean score; ends mould wiped / cap / squad wiped):");
    print!("{:>24}", "");
    for t in &tiers {
        print!(" {t:>20}");
    }
    println!();
    for l in &loads {
        print!("{l:>24}");
        for t in &tiers {
            let rows: Vec<Row> = seeds.iter().map(|&s| get(t, s, l)).collect();
            let e = |b: &str| rows.iter().filter(|r| r.end == b).count();
            print!(" {:>+7.3} ({:>2}/{:>2}/{:>2})", mean(t, l), e("mould wiped"), e("cap"), e("squad wiped"));
        }
        println!();
    }
    // Readings 1 and 2, one Holm family.
    let has = |l: &str| loads.iter().any(|x| x == l);
    let mut tests = Vec::new();
    for t in &tiers {
        for l in &loads {
            let mut bases: Vec<&str> = Vec::new();
            if l != "none" && l != "rand" {
                bases.push("none");
            }
            if l.contains('+') && l.ends_with("+blast") && l.matches('+').count() == 1 {
                bases.push("blast");
            }
            if ["trap", "lull", "plague", "shock"].contains(&l.as_str()) {
                bases.push("rand");
            }
            for b in bases {
                if !has(b) {
                    continue;
                }
                let (w, lo) = pair(t, l, b);
                tests.push((format!("{t} {l} vs {b}"), w, lo, sign_p(w, w + lo)));
            }
        }
    }
    let holds = holm(&tests);
    println!("\nreadings 1-2, paired by seed (higher / lower; sign test, Holm over {}):", tests.len());
    for (i, (name, w, l, p)) in tests.iter().enumerate() {
        println!("  {name:>40}: {w:>2} / {l:>2}, p {p:.4}{}", if holds[i] { ", holds" } else { "" });
    }
    // Reading 3: the best loadout per tier, and each other against it.
    println!("\nreading 3, per tier: the best loadout by mean, and each other against it (higher / lower, p):");
    for t in &tiers {
        let best = loads.iter().max_by(|a, b| mean(t, a).total_cmp(&mean(t, b))).unwrap();
        let line: Vec<String> = loads
            .iter()
            .filter(|l| *l != best)
            .map(|l| {
                let (w, lo) = pair(t, l, best);
                format!("{l} {w}/{lo} p {:.3}", sign_p(w, w + lo))
            })
            .collect();
        println!("  {t:>8}: best {best} ({:+.3}); {}", mean(t, best), line.join(", "));
    }
    let rand_rounds: Vec<String> = seeds.iter().map(|&s| rounds::name(&random_round(s))).collect();
    println!("\nrand's round by seed: {rand_rounds:?}");
}
