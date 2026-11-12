//! **The campaign census** (ROADMAP W5; design
//! `Reports/larder-campaign-2026-11-12.md`): plays a mission's map on the
//! CPU big dish with idle, random and the player AI's hands over seeds, and
//! follows each round with the mission layer (`mould::campaign::Tracker`):
//! the verdict, stars, the bar's best, dead time by phase, the longest dead
//! and stuck runs, and "decided at".
//!
//! cargo run --release --example campaign -- mission=1-landfall seeds=1-8 [hands=idle,random,boom,claim,late,fort,strike scale=2 walkers=5000 threads=4]
//!
//! The mould's tier is the mission's when `mould::tiers::Tier` has it; the
//! ladder names not yet built (drifter, sprawler, trunk) play the plain
//! mould, and the output says so.

use slime_mould::mould::big::BigDish;
use slime_mould::mould::campaign::{decided_at, Goal, Mission, Tracker, BIN};
use slime_mould::mould::larder_world::{race_params, Cmd, Game, Order, Rules, Stance, World};
use slime_mould::mould::larder_world_ai;
use slime_mould::mould::map::Map;
use slime_mould::mould::tiers::Tier;
use std::path::Path;

struct Row {
    hand: String,
    seed: u64,
    cause: String,
    at: f32,
    stars: u8,
    best: f32,
    dead: f32,
    phase_dead: [f32; 4],
    longest_dead: u32,
    longest_stuck: u32,
    longest_stall: u32,
    decided: usize,
    alive: usize,
    harvested: f32,
    killed: u32,
    /// The goal's count over the round (crystals clear, or the share of
    /// walkers outside a contain goal's region): its least and its most.
    count: (f32, f32),
}

fn main() {
    let (mut mission, mut seeds, mut hands, mut scale, mut walkers, mut threads) = (String::from("1-landfall"), (1u64, 8u64), String::from("idle,random,boom,claim,late,fort,strike"), 2usize, 5000usize, 4usize);
    for a in std::env::args().skip(1) {
        let (k, v) = a.split_once('=').unwrap_or((&a, ""));
        match k {
            "mission" => mission = v.into(),
            "seeds" => {
                let (a, b) = v.split_once('-').unwrap_or((v, v));
                seeds = (a.parse().unwrap(), b.parse().unwrap());
            }
            "hands" => hands = v.into(),
            "scale" => scale = v.parse().unwrap(),
            "walkers" => walkers = v.parse().unwrap(),
            "threads" => threads = v.parse().unwrap(),
            _ => panic!("unknown argument {a}"),
        }
    }
    let root = Path::new(env!("CARGO_MANIFEST_DIR"));
    let m = Mission::parse(&std::fs::read_to_string(root.join(format!("Reports/larder-campaign-2026-11-12/missions/{mission}.txt"))).unwrap()).unwrap();
    let map = Map::load(&root.join("Reports/larder-maps-2026-11-11/maps"), &m.map).unwrap();
    let tier = m.tier.as_deref().and_then(Tier::from_name);
    println!(
        "mission {} ({}): \"{}\", map {}, tier {} {}, seeds {}-{}, scale {scale}, walkers {walkers}",
        mission,
        m.name,
        m.says,
        m.map,
        m.tier.as_deref().unwrap_or("none"),
        if m.tier.is_some() && tier.is_none() { "(not built: plain mould)" } else { "" },
        seeds.0,
        seeds.1
    );
    let jobs: Vec<(String, u64)> = hands.split(',').flat_map(|h| (seeds.0..=seeds.1).map(move |s| (h.to_string(), s))).collect();
    let next = std::sync::atomic::AtomicUsize::new(0);
    let rows = std::sync::Mutex::new(Vec::new());
    std::thread::scope(|sc| {
        for _ in 0..threads {
            sc.spawn(|| loop {
                let i = next.fetch_add(1, std::sync::atomic::Ordering::Relaxed);
                let Some((h, s)) = jobs.get(i) else { break };
                let r = play(&m, &map, tier, *s, scale, walkers, h);
                eprintln!("  {h} s{s}: {} {:.0}s {}*", r.cause, r.at, r.stars);
                rows.lock().unwrap().push(r);
            });
        }
    });
    let mut rows = rows.into_inner().unwrap();
    rows.sort_by(|a, b| a.hand.cmp(&b.hand).then(a.seed.cmp(&b.seed)));
    println!("\nhand      seed  cause       at    stars best  dead  dead by phase (land/contact/pressure/climax)  longest dead/stuck/stall  decided  alive harvested killed  count least/most");
    for r in &rows {
        println!(
            "{:<9} {:>4}  {:<10} {:>5.0}  {:>5} {:>4.2}  {:>4.0}%  {:>4.0}% {:>4.0}% {:>4.0}% {:>4.0}%                          {:>4} {:>4} {:>4}    {:>5}  {:>5} {:>9.0} {:>6}  {:.3}/{:.3}",
            r.hand,
            r.seed,
            r.cause,
            r.at,
            r.stars,
            r.best,
            100.0 * r.dead,
            100.0 * r.phase_dead[0],
            100.0 * r.phase_dead[1],
            100.0 * r.phase_dead[2],
            100.0 * r.phase_dead[3],
            r.longest_dead,
            r.longest_stuck,
            r.longest_stall,
            r.decided,
            r.alive,
            r.harvested,
            r.killed,
            r.count.0,
            r.count.1
        );
    }
    println!("\nby hand: goal met / seeds, median stars, mean best, mean dead share, worst stuck run, worst stall");
    let mut names: Vec<&str> = rows.iter().map(|r| r.hand.as_str()).collect();
    names.dedup();
    for h in names {
        let rs: Vec<&Row> = rows.iter().filter(|r| r.hand == h).collect();
        let mut stars: Vec<u8> = rs.iter().map(|r| r.stars).collect();
        stars.sort();
        let met = rs.iter().filter(|r| r.cause == "Goal").count();
        println!(
            "  {h:<9} {met}/{}  stars {}  best {:.2}  dead {:.0}%  stuck {} s  stall {} s",
            rs.len(),
            stars[stars.len() / 2],
            rs.iter().map(|r| r.best).sum::<f32>() / rs.len() as f32,
            100.0 * rs.iter().map(|r| r.dead).sum::<f32>() / rs.len() as f32,
            rs.iter().map(|r| r.longest_stuck).max().unwrap_or(0),
            rs.iter().map(|r| r.longest_stall).max().unwrap_or(0)
        );
    }
}

fn play(m: &Mission, map: &Map, tier: Option<Tier>, seed: u64, scale: usize, walkers: usize, hand: &str) -> Row {
    let mut p = race_params(seed, scale, walkers);
    if let Some(t) = tier {
        t.apply(&mut p);
    }
    let mut world = BigDish::with_map(p, map);
    world.threads = 1;
    let mut g = Game::new(world, m.rules(Rules::default()), seed);
    let mut tr = Tracker::new(m.clone(), &g);
    let mut ai = larder_world_ai::Hand::named(hand, &g);
    let tps = g.rules.tps as u64;
    let mut count = (f32::INFINITY, f32::NEG_INFINITY);
    let v = loop {
        match hand {
            "random" => random_hand(&mut g),
            _ => {
                ai.as_mut().unwrap_or_else(|| panic!("unknown hand {hand}")).play(&mut g);
            }
        }
        let s = g.world.ticks() / tps;
        while g.world.ticks() / tps == s && g.end.is_none() {
            g.step();
        }
        let grid = g.world.mould_grid(BIN);
        let v = tr.second(&g, grid.as_deref(), 1.0);
        if let Some(c) = tr.count {
            let c = if matches!(m.goal, Goal::Contain { .. }) { c as f32 / g.mould.max(1) as f32 } else { c as f32 };
            count = (count.0.min(c), count.1.max(c));
        }
        if let Some(v) = v {
            break v;
        }
    };
    let share = |p: usize| if tr.secs[p] == 0 { 0.0 } else { tr.dead[p] as f32 / tr.secs[p] as f32 };
    Row {
        hand: hand.into(),
        seed,
        cause: format!("{:?}", v.cause),
        at: v.at,
        stars: v.stars,
        best: v.best,
        dead: tr.dead_share(),
        phase_dead: [share(0), share(1), share(2), share(3)],
        longest_dead: tr.longest_dead,
        longest_stuck: tr.longest_stuck,
        longest_stall: tr.longest_stall,
        decided: decided_at(&tr.bars, v.stars),
        alive: g.alive(),
        harvested: g.harvested,
        killed: g.killed,
        count,
    }
}

/// The census's random hand (as `examples/larderworld.rs`): every 5 s one
/// soldier gets a random order and stance; sometimes a buy or a wall.
fn random_hand(g: &mut Game<BigDish>) {
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
    let point = |g: &mut Game<BigDish>| loop {
        let p = (g.rng.f32() * w as f32, g.rng.f32() * h as f32);
        if g.world.open(p.0, p.1) {
            break p;
        }
    };
    let order = match g.rng.below(4) {
        0 => {
            let p = point(g);
            Order::Move(p.0, p.1)
        }
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
}
