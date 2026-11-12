//! **The gardeners' first census** (ROADMAP L-O; `mould::own_mould`): the
//! Larder world's game (`larder_world::Game`) on a `Garden`, where the
//! player owns a second mould on the larder crystal and the soldiers can
//! lay its roads, feed it and cut it.
//!
//! ```text
//! cargo run --release --example garden -- census seeds=1-32 [hands=idle,random,farm,road,feed,grow cap=300 walkers=2500 threads=4]
//! cargo run --release --example garden -- look=DIR seeds=1 hands=road [every=60]
//! cargo run --release --example garden -- front seeds=1-32 [at=150 arms=none,sow,shoot,road,cut look=DIR]
//! ```
//!
//! **The front probe** (`front`, after the designer's review of #119: two
//! moulds holding a line is an RTS front; does each action move it?): every
//! arm plays `sow2` to `at` seconds, when the two moulds meet, then the
//! squad farms and soldiers 0 and 1 take the arm's action for 40 s:
//! **none** (back to the larder), **sow** (Feed at the front), **shoot**
//! (Free fire at the front), **road** (Road 15 cells past the front, into
//! the enemy), **cut** (Cut at the front: the player's own line should
//! fall back, a check that the measure sees a move). The front is the
//! player's cell (own trail over 1 and over the enemy's) nearest the
//! enemy's core. **Deciding measure, named before the run:** the player's
//! cells (own trail over 1 and over the enemy's) 20 s after the branch,
//! less the none arm's on the same seed, sign test, Holm over the four
//! arms and both measures; second measure the enemy's cells the same way.
//! Printed at 20 and 40 s.
//!
//! **Deciding measure, named before the run:** the player's share of the
//! crystal charge gone over the game, both of the player's eaters
//! counted: the squad's harvest (`Rules::charge` a food) and the player's
//! mould's part of the rest, split from the enemy's by their walkers'
//! `took`. Each hand
//! against idle and against random, paired by seed, two-sided sign test,
//! with the median difference; Holm over every comparison. Second
//! measure (named before the second census, after the first showed the
//! share is the squad's harvest whatever the gardeners do): the moulds'
//! balance at the cap, the player's walkers less the enemy's, paired
//! against idle, random and farm. Also printed:
//! each mould's walkers at the cap, and what the gardeners did.
//!
//! **Hands** (fixed rules):
//! - **idle**: never gives an order or a task.
//! - **random**: every 5 game seconds one random soldier gets a random
//!   move or crystal and a random task (none, road, feed, cut).
//! - **farm**: the scripted farm (`examples/larderworld.rs`'s, without
//!   buying past 8): soldier k clears the k-th nearest crystal; no tasks.
//! - **road**: farm, and soldier 0 walks Road from the larder to the
//!   nearest crystal no soldier is clearing, then the next, so the
//!   player's mould is led to food the squad leaves.
//! - **feed**: farm, and soldier 0 stands on the larder crystal on Feed,
//!   spending the bank over `RESERVE` into the player's mould.
//! - **grow**: road, then once the player's mould holds more walkers than
//!   it started with, soldier 0 feeds at its front (the road's end).
//! - **sow**, **sow2**: farm, and soldier 0 (and 1) walks from the larder to
//!   the nearest crystal no one clears and back on Feed while the bank is
//!   over `RESERVE` (else Road), sowing charge thin along the way. Added
//!   after the first census: feeding in one disc piled charge faster than
//!   the walkers there could eat it, while the random hand's scattered
//!   feeding grew the player's mould to 4 times idle's.
use std::sync::atomic::{AtomicUsize, Ordering};
use std::sync::Mutex;

use slime_mould::mould::larder_world::{Cmd, End, Game, Order, Rules, Stance, World};
use slime_mould::mould::own_mould::{garden_world, Garden, Task, OWN};

const RESERVE: f32 = 600.0;
const FARM_MAX: usize = 8;

#[derive(Clone)]
struct Opts {
    seeds: Vec<u64>,
    hands: Vec<String>,
    cap: f32,
    walkers: usize,
    threads: usize,
    look: Option<String>,
    every: u32,
}

fn opts() -> Opts {
    let mut o = Opts {
        seeds: (1..=32).collect(),
        hands: ["idle", "random", "farm", "road", "feed", "grow"].iter().map(|s| s.to_string()).collect(),
        cap: 300.0,
        walkers: 2500,
        threads: 4,
        look: None,
        every: 60,
    };
    for a in std::env::args().skip(1) {
        let Some((k, v)) = a.split_once('=') else { continue };
        match k {
            "seeds" => {
                o.seeds = v
                    .split(',')
                    .flat_map(|p| match p.split_once('-') {
                        Some((a, b)) => (a.parse::<u64>().unwrap()..=b.parse().unwrap()).collect::<Vec<_>>(),
                        None => vec![p.parse().unwrap()],
                    })
                    .collect()
            }
            "hands" => o.hands = v.split(',').map(|s| s.to_string()).collect(),
            "cap" => o.cap = v.parse().unwrap(),
            "walkers" => o.walkers = v.parse().unwrap(),
            "threads" => o.threads = v.parse().unwrap(),
            "look" => o.look = Some(v.to_string()),
            "every" => o.every = v.parse().unwrap(),
            "at" | "arms" => {}
            _ => panic!("unknown key {k}"),
        }
    }
    o
}

/// Crystals with charge, nearest the larder first, the enemy's core left out.
fn by_distance(g: &Game<Garden>) -> Vec<(f32, f32)> {
    let l = g.larder;
    let r = g.rules;
    let dry = r.harvest * r.frame_ticks as f32 / r.tps as f32 * r.charge;
    let mut cs: Vec<(f32, f32)> = g.world.crystals().iter().filter(|c| c.1 >= dry).map(|c| c.0).filter(|c| *c != g.core && *c != l).collect();
    cs.sort_by(|a, b| (a.0 - l.0).hypot(a.1 - l.1).total_cmp(&(b.0 - l.0).hypot(b.1 - l.1)));
    cs
}

/// The scripted farm; soldiers in `skip` are left to their gardening.
fn farm(g: &mut Game<Garden>, skip: &[usize]) -> Vec<(f32, f32)> {
    let cs = by_distance(g);
    if g.bank > g.rules.soldier_cost + RESERVE && g.alive() < FARM_MAX {
        g.apply(Cmd::Buy);
    }
    let mut taken: Vec<(f32, f32)> = Vec::new();
    for k in 0..g.soldiers.len() {
        let s = g.soldiers[k];
        if !s.alive || skip.contains(&k) {
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
            g.apply(Cmd::Stance(k, Stance::Defend));
        }
    }
    cs.into_iter().filter(|c| !taken.contains(c)).collect()
}

/// Soldier 0 walks a road to `to`; true when it is there.
fn road_to(g: &mut Game<Garden>, to: (f32, f32)) -> bool {
    let s = g.soldiers[0];
    g.world.set_task(0, Task::Road);
    if s.order != Order::Move(to.0, to.1) {
        g.apply(Cmd::Order(0, Order::Move(to.0, to.1)));
    }
    (s.x - to.0).hypot(s.y - to.1) < 3.0 * g.world.scale()
}

#[derive(Default)]
struct Hand {
    /// The road hand's current target, and whether it is walking back.
    target: Option<(f32, f32)>,
    back: bool,
}

fn act(name: &str, g: &mut Game<Garden>, h: &mut Hand, start_own: usize) {
    if !g.soldiers[0].alive && name != "idle" && name != "random" {
        farm(g, &[]);
        return;
    }
    match name {
        "idle" => {}
        "random" => {
            if !g.world.ticks().is_multiple_of(5 * g.rules.tps as u64) {
                return;
            }
            let live: Vec<usize> = (0..g.soldiers.len()).filter(|&k| g.soldiers[k].alive).collect();
            if live.is_empty() {
                return;
            }
            let k = live[g.rng.below(live.len() as u32) as usize];
            let (w, hh) = g.world.size();
            let order = if g.rng.below(2) == 0 {
                Order::Move(g.rng.f32() * w as f32, g.rng.f32() * hh as f32)
            } else {
                let cs = g.world.crystals();
                let c = cs[g.rng.below(cs.len() as u32) as usize].0;
                Order::Clear(c.0, c.1)
            };
            g.apply(Cmd::Order(k, order));
            let t = [Task::None, Task::Road, Task::Feed, Task::Cut][g.rng.below(4) as usize];
            g.world.set_task(k, t);
        }
        "farm" => {
            farm(g, &[]);
        }
        "road" | "grow" => {
            let free = farm(g, &[0]);
            let grown = g.world.counts_by_side().0 > start_own;
            if let (true, true, Some(t), false) = (name == "grow", grown, h.target, h.back) {
                // Feed at the road's end once the mould has grown.
                if road_to(g, t) {
                    let spare = g.bank - RESERVE;
                    g.world.set_task(0, if spare > 0.0 { Task::Feed } else { Task::Road });
                }
                return;
            }
            if h.back {
                let l = g.larder;
                if road_to(g, l) {
                    h.back = false;
                    h.target = None;
                }
                return;
            }
            let t = match h.target {
                Some(t) if free.contains(&t) => t,
                _ => match free.first() {
                    Some(&t) => {
                        h.target = Some(t);
                        t
                    }
                    None => return,
                },
            };
            if road_to(g, t) {
                h.back = true;
            }
        }
        "sow" | "sow2" => {
            // Soldiers 0 (and 1 for sow2) walk from the larder to the
            // nearest crystal no one is clearing and back, on Feed, so the
            // charge is sown thin along the way instead of piled in one disc.
            let sowers: &[usize] = if name == "sow2" { &[0, 1] } else { &[0] };
            let free = farm(g, sowers);
            for &k in sowers {
                if k >= g.soldiers.len() || !g.soldiers[k].alive {
                    continue;
                }
                let s = g.soldiers[k];
                let to = if h.back { g.larder } else { h.target.or(free.get(k % free.len().max(1)).copied()).unwrap_or(g.larder) };
                if h.target.is_none() && !h.back {
                    h.target = Some(to);
                }
                if s.order != Order::Move(to.0, to.1) {
                    g.apply(Cmd::Order(k, Order::Move(to.0, to.1)));
                }
                g.world.set_task(k, if g.bank > RESERVE { Task::Feed } else { Task::Road });
                if k == 0 && (s.x - to.0).hypot(s.y - to.1) < 3.0 * g.world.scale() {
                    if h.back {
                        h.back = false;
                        h.target = None;
                    } else {
                        h.back = true;
                    }
                }
            }
        }
        "feed" => {
            farm(g, &[0]);
            let l = g.larder;
            if g.soldiers[0].order != Order::Move(l.0, l.1) {
                g.apply(Cmd::Order(0, Order::Move(l.0, l.1)));
            }
            g.world.set_task(0, if g.bank > RESERVE { Task::Feed } else { Task::None });
        }
        _ => panic!("unknown hand {name}"),
    }
}

#[derive(Clone, Copy, Debug)]
struct Out {
    share: f64,
    own: usize,
    enemy: usize,
    soldiers: usize,
    end: &'static str,
    roads: u64,
    fed: f32,
    cut: u32,
}

/// Called with the game and its second (`look=`).
type Look<'a> = &'a mut dyn FnMut(&Game<Garden>, u32);

fn play(o: &Opts, seed: u64, hand: &str, mut look: Option<Look>) -> Out {
    let rules = Rules { cap: o.cap, ..Rules::default() };
    let mut g = Game::new(garden_world(seed, 2, o.walkers, None), rules, seed);
    let start_own = g.world.counts_by_side().0;
    let start: f64 = g.world.dish.charge.iter().map(|&c| c as f64).sum();
    let mut h = Hand::default();
    let mut next = 0u32;
    while g.end.is_none() {
        act(hand, &mut g, &mut h, start_own);
        g.step();
        if let Some(f) = look.as_mut() {
            let t = g.time() as u32;
            if t >= next {
                f(&g, t);
                next = t + o.every;
            }
        }
    }
    // The charge gone from the crystals is exact; what the squad took is
    // its harvest; the moulds ate the rest, split between them by their
    // walkers' `took` (sampled once a game tick, the same for both sides,
    // so the split holds while the totals would miss the other substeps).
    // Fed charge lands on the ground (crystal cells among it), so it adds
    // to what can be gone.
    let left: f64 = g.world.dish.charge.iter().map(|&c| c as f64).sum();
    let gone = (start + g.world.fed as f64 * g.rules.charge as f64 - left).max(0.0);
    let squad = (g.harvested as f64 * g.rules.charge as f64).min(gone);
    let moulds = gone - squad;
    let own_frac = g.world.own_ate / (g.world.own_ate + g.world.enemy_ate).max(1e-9);
    let share = (squad + moulds * own_frac) / gone.max(1e-9);
    let (own, enemy) = g.world.counts_by_side();
    let end = match g.end {
        Some(End::SquadWiped(_)) => "squad wiped",
        Some(End::MouldWiped(_)) => "enemy wiped",
        _ => "cap",
    };
    Out { share, own, enemy, soldiers: g.alive(), end, roads: g.world.roads, fed: g.world.fed, cut: g.world.cut }
}

fn picture(g: &Game<Garden>) -> image::RgbaImage {
    let d = &g.world.dish;
    let (w, h) = (d.w, d.h);
    let f = |v: f32| (1.0 - (-v / 6.0).exp()).clamp(0.0, 1.0);
    let mut img = image::RgbaImage::from_fn(w as u32, h as u32, |x, y| {
        let i = y as usize * w + x as usize;
        let own = f(d.species_trail[OWN as usize][i]);
        let enemy = f(d.trail[i] - d.species_trail[OWN as usize][i]);
        let rock = !d.rock.is_empty() && d.rock[i];
        let crystal = !d.pile_of.is_empty() && d.pile_of[i] > 0 && d.charge.get(i).copied().unwrap_or(0.0) > 0.0;
        let base = if rock {
            [60.0, 60.0, 64.0]
        } else if crystal {
            [40.0, 70.0, 90.0]
        } else {
            [16.0, 16.0, 20.0]
        };
        // The player's mould in green, the enemy's in amber.
        let px = [base[0] + enemy * 220.0 + own * 40.0, base[1] + enemy * 140.0 + own * 210.0, base[2] + enemy * 20.0 + own * 90.0];
        image::Rgba([px[0].min(255.0) as u8, px[1].min(255.0) as u8, px[2].min(255.0) as u8, 255])
    });
    for s in g.soldiers.iter().filter(|s| s.alive) {
        for dy in -1i32..=1 {
            for dx in -1i32..=1 {
                let (x, y) = (s.x as i32 + dx, s.y as i32 + dy);
                if x >= 0 && y >= 0 && (x as usize) < w && (y as usize) < h {
                    img.put_pixel(x as u32, y as u32, image::Rgba([255, 255, 255, 255]));
                }
            }
        }
    }
    img
}

fn sign_p(a: usize, b: usize) -> f64 {
    let n = a + b;
    if n == 0 {
        return 1.0;
    }
    let k = a.min(b);
    let (mut c, mut tail) = (1.0f64, 0.0f64);
    for i in 0..=k {
        if i > 0 {
            c = c * (n - i + 1) as f64 / i as f64;
        }
        tail += c;
    }
    (2.0 * tail / 2f64.powi(n as i32)).min(1.0)
}

/// The player's and the enemy's cells: trail over 1 and over the other's.
fn cells(g: &Game<Garden>) -> (usize, usize) {
    let d = &g.world.dish;
    let (mut own, mut enemy) = (0, 0);
    for i in 0..d.w * d.h {
        let o = d.species_trail[OWN as usize][i];
        let e = d.trail[i] - o;
        if o > 1.0 && o > e {
            own += 1;
        } else if e > 1.0 && e > o {
            enemy += 1;
        }
    }
    (own, enemy)
}

/// The player's cell nearest the enemy's core, and the point `past` cells
/// beyond it towards the core.
fn front_of(g: &Game<Garden>, past: f32) -> (f32, f32) {
    let d = &g.world.dish;
    let c = g.core;
    let mut best = (f32::MAX, g.larder);
    for i in 0..d.w * d.h {
        let o = d.species_trail[OWN as usize][i];
        if o > 1.0 && o > d.trail[i] - o && d.agar[i] {
            let (x, y) = ((i % d.w) as f32 + 0.5, (i / d.w) as f32 + 0.5);
            let r = (x - c.0).hypot(y - c.1);
            if r < best.0 {
                best = (r, (x, y));
            }
        }
    }
    let (r, (x, y)) = best;
    if r == f32::MAX || r < 1.0 {
        return (x, y);
    }
    let t = (past / r).min(1.0);
    (x + (c.0 - x) * t, y + (c.1 - y) * t)
}

/// One arm of the front probe: (cells at the branch, at +20 s, at +40 s).
fn front_arm(o: &Opts, seed: u64, arm: &str, at: f32) -> [(usize, usize); 3] {
    let rules = Rules { cap: at + 41.0, ..Rules::default() };
    let mut g = Game::new(garden_world(seed, 2, o.walkers, None), rules, seed);
    let start_own = g.world.counts_by_side().0;
    let mut h = Hand::default();
    while g.end.is_none() && g.time() < at {
        act("sow2", &mut g, &mut h, start_own);
        g.step();
    }
    let mut out = [cells(&g); 3];
    let to = front_of(&g, if arm == "road" { 15.0 } else { 0.0 });
    for k in [0usize, 1] {
        if k >= g.soldiers.len() || !g.soldiers[k].alive {
            continue;
        }
        let (dest, task, stance) = match arm {
            "none" => (g.larder, Task::None, Stance::Defend),
            "sow" => (to, Task::Feed, Stance::Defend),
            "shoot" => (to, Task::None, Stance::Free),
            "road" => (to, Task::Road, Stance::Defend),
            "cut" => (to, Task::Cut, Stance::Defend),
            _ => panic!("unknown arm {arm}"),
        };
        g.apply(Cmd::Order(k, Order::Move(dest.0, dest.1)));
        g.apply(Cmd::Stance(k, stance));
        g.world.set_task(k, task);
    }
    let t0 = g.time();
    let mut slot = 1;
    while g.end.is_none() && slot < 3 {
        farm(&mut g, &[0, 1]);
        g.step();
        if g.time() >= t0 + 20.0 * slot as f32 {
            out[slot] = cells(&g);
            if let (Some(dir), 1) = (&o.look, slot) {
                std::fs::create_dir_all(dir).unwrap();
                picture(&g).save(format!("{dir}/front-{arm}-s{seed}.png")).unwrap();
            }
            slot += 1;
        }
    }
    for s in out.iter_mut().skip(slot) {
        *s = cells(&g);
    }
    out
}

fn front(o: &Opts) {
    let mut at = 150.0f32;
    let mut arms: Vec<String> = ["none", "sow", "shoot", "road", "cut"].iter().map(|s| s.to_string()).collect();
    for a in std::env::args().skip(1) {
        if let Some(v) = a.strip_prefix("at=") {
            at = v.parse().unwrap();
        }
        if let Some(v) = a.strip_prefix("arms=") {
            arms = v.split(',').map(|s| s.to_string()).collect();
        }
    }
    println!("front: branch at {at} s from sow2, arms {arms:?}, seeds {:?}", o.seeds);
    let jobs: Vec<(usize, u64)> = (0..arms.len()).flat_map(|a| o.seeds.iter().map(move |&s| (a, s))).collect();
    let next = AtomicUsize::new(0);
    let res = Mutex::new(Vec::new());
    std::thread::scope(|sc| {
        for _ in 0..o.threads {
            sc.spawn(|| loop {
                let k = next.fetch_add(1, Ordering::SeqCst);
                if k >= jobs.len() {
                    break;
                }
                let (a, s) = jobs[k];
                let out = front_arm(o, s, &arms[a], at);
                println!("raw {} seed {}: {:?}", arms[a], s, out);
                res.lock().unwrap().push((a, s, out));
            });
        }
    });
    let r = res.into_inner().unwrap();
    let get = |a: usize, s: u64| r.iter().find(|x| x.0 == a && x.1 == s).map(|x| x.2).unwrap();
    let Some(none) = arms.iter().position(|a| a == "none") else { return };
    let n = o.seeds.len() as f64;
    println!("\nsummary: mean cells (player / enemy) at the branch, +20 s, +40 s");
    for (a, name) in arms.iter().enumerate() {
        let m = |j: usize, side: usize| o.seeds.iter().map(|&s| { let c = get(a, s)[j]; if side == 0 { c.0 } else { c.1 } } as f64).sum::<f64>() / n;
        println!("{name:>6}: {:.0} / {:.0}, {:.0} / {:.0}, {:.0} / {:.0}", m(0, 0), m(0, 1), m(1, 0), m(1, 1), m(2, 0), m(2, 1));
    }
    let mut tests = Vec::new();
    for j in [1usize, 2] {
        for side in [0usize, 1] {
            for (a, name) in arms.iter().enumerate() {
                if a == none {
                    continue;
                }
                let mut d: Vec<f64> = o.seeds.iter().map(|&s| {
                    let (x, y) = (get(a, s)[j], get(none, s)[j]);
                    if side == 0 { x.0 as f64 - y.0 as f64 } else { x.1 as f64 - y.1 as f64 }
                }).collect();
                let hi = d.iter().filter(|&&v| v > 0.0).count();
                let lo = d.iter().filter(|&&v| v < 0.0).count();
                d.sort_by(|a, b| a.total_cmp(b));
                let label = format!("+{}s {} cells {name} vs none", 20 * j, if side == 0 { "player" } else { "enemy" });
                tests.push((label, hi, lo, sign_p(hi, lo), d[d.len() / 2], j == 1));
            }
        }
    }
    // Holm over the deciding family: the +20 s tests, both sides.
    let mut fam: Vec<usize> = (0..tests.len()).filter(|&k| tests[k].5).collect();
    fam.sort_by(|&a, &b| tests[a].3.total_cmp(&tests[b].3));
    let m = fam.len();
    let mut holm = vec![None; tests.len()];
    let mut still = true;
    for (rank, &k) in fam.iter().enumerate() {
        still = still && tests[k].3 <= 0.05 / (m - rank) as f64;
        holm[k] = Some(still);
    }
    println!("\npaired against none: higher / lower, p, median difference (Holm over the +20 s tests)");
    for (k, t) in tests.iter().enumerate() {
        let h = match holm[k] { Some(true) => "  holds under Holm", Some(false) => "", None => "  (not in the family)" };
        println!("{:>34}  {:>2} / {:>2}  p {:.4}  median {:+.0}{h}", t.0, t.1, t.2, t.3, t.4);
    }
}

fn main() {
    let o = opts();
    // `look=` with `front` saves each arm 20 s after the branch.
    if std::env::args().any(|a| a == "front") {
        front(&o);
        return;
    }
    println!("garden: cap {} s, walkers {} a side, hands {:?}, seeds {:?}", o.cap, o.walkers, o.hands, o.seeds);
    if let Some(dir) = o.look.clone() {
        std::fs::create_dir_all(&dir).unwrap();
        for &seed in &o.seeds {
            for hand in &o.hands {
                let mut save = |g: &Game<Garden>, t: u32| {
                    let (own, enemy) = g.world.counts_by_side();
                    let path = format!("{dir}/{hand}-s{seed}-t{t:03}.png");
                    picture(g).save(&path).unwrap();
                    println!("look {hand} seed {seed} t {t}s own {own} enemy {enemy} bank {:.0} -> {path}", g.bank);
                };
                let out = play(&o, seed, hand, Some(&mut save));
                println!("end {hand} seed {seed}: {out:?}");
            }
        }
        return;
    }
    let jobs: Vec<(usize, u64)> = (0..o.hands.len()).flat_map(|h| o.seeds.iter().map(move |&s| (h, s))).collect();
    let next = AtomicUsize::new(0);
    let res = Mutex::new(Vec::new());
    std::thread::scope(|sc| {
        for _ in 0..o.threads {
            sc.spawn(|| loop {
                let k = next.fetch_add(1, Ordering::SeqCst);
                if k >= jobs.len() {
                    break;
                }
                let (h, s) = jobs[k];
                let out = play(&o, s, &o.hands[h], None);
                println!("raw {} seed {}: {:?}", o.hands[h], s, out);
                res.lock().unwrap().push((h, s, out));
            });
        }
    });
    let r = res.into_inner().unwrap();
    let get = |h: usize, s: u64| r.iter().find(|x| x.0 == h && x.1 == s).map(|x| x.2).unwrap();
    println!("\nsummary: player's share of the charge eaten (squad + own mould), and walkers at the end");
    for (h, name) in o.hands.iter().enumerate() {
        let mut sh: Vec<f64> = o.seeds.iter().map(|&s| get(h, s).share).collect();
        sh.sort_by(|a, b| a.total_cmp(b));
        let n = o.seeds.len() as f64;
        let own = o.seeds.iter().map(|&s| get(h, s).own as f64).sum::<f64>() / n;
        let enemy = o.seeds.iter().map(|&s| get(h, s).enemy as f64).sum::<f64>() / n;
        let wiped = o.seeds.iter().filter(|&&s| get(h, s).end == "squad wiped").count();
        let soldiers = o.seeds.iter().map(|&s| get(h, s).soldiers as f64).sum::<f64>() / n;
        let roads = o.seeds.iter().map(|&s| get(h, s).roads as f64).sum::<f64>() / n;
        let fed = o.seeds.iter().map(|&s| get(h, s).fed as f64).sum::<f64>() / n;
        let cut = o.seeds.iter().map(|&s| get(h, s).cut as f64).sum::<f64>() / n;
        println!(
            "{name:>7}: median share {:.3} ({:.3}-{:.3}), mean own {own:.0} enemy {enemy:.0} soldiers {soldiers:.1}, squad wiped {wiped}; road frames {roads:.0}, fed {fed:.0}, cut {cut:.0}",
            sh[sh.len() / 2],
            sh[0],
            sh[sh.len() - 1]
        );
    }
    println!("\npaired (hand vs control): seeds where the hand's share is higher / lower, p, median difference");
    let mut tests = Vec::new();
    for (h, name) in o.hands.iter().enumerate() {
        for ctrl in ["idle", "random"] {
            let Some(c) = o.hands.iter().position(|x| x == ctrl) else { continue };
            if c == h {
                continue;
            }
            let mut d: Vec<f64> = o.seeds.iter().map(|&s| get(h, s).share - get(c, s).share).collect();
            let (up, down) = (d.iter().filter(|&&x| x > 0.0).count(), d.iter().filter(|&&x| x < 0.0).count());
            d.sort_by(|a, b| a.total_cmp(b));
            tests.push((name.clone(), ctrl, up, down, sign_p(up, down), d[d.len() / 2]));
        }
    }
    println!("\npaired on the moulds' balance at the cap (player's walkers less the enemy's): higher / lower, p, median difference");
    for (h, name) in o.hands.iter().enumerate() {
        for ctrl in ["idle", "random", "farm"] {
            let Some(c) = o.hands.iter().position(|x| x == ctrl) else { continue };
            if c == h {
                continue;
            }
            let bal = |h: usize, s: u64| get(h, s).own as f64 - get(h, s).enemy as f64;
            let mut d: Vec<f64> = o.seeds.iter().map(|&s| bal(h, s) - bal(c, s)).collect();
            let (up, down) = (d.iter().filter(|&&x| x > 0.0).count(), d.iter().filter(|&&x| x < 0.0).count());
            d.sort_by(|a, b| a.total_cmp(b));
            println!("{name:>7} vs {ctrl:<6} {up:>2} / {down:>2}  p {:.4}  median diff {:+.0}", sign_p(up, down), d[d.len() / 2]);
        }
    }
    let m = tests.len();
    let mut order: Vec<usize> = (0..m).collect();
    order.sort_by(|&a, &b| tests[a].4.total_cmp(&tests[b].4));
    let mut holm = vec![false; m];
    for (rank, &i) in order.iter().enumerate() {
        if tests[i].4 * (m - rank) as f64 <= 0.05 {
            holm[i] = true;
        } else {
            break;
        }
    }
    for (i, t) in tests.iter().enumerate() {
        println!("{:>7} vs {:<6} {:>2} / {:>2}  p {:.4}  median diff {:+.3}{}", t.0, t.1, t.2, t.3, t.4, t.5, if holm[i] { "  (Holm)" } else { "" });
    }
}
