//! **The campaign census** (ROADMAP W5; design
//! `Reports/larder-campaign-2026-11-12.md`): plays a mission's map on the
//! CPU big dish with idle, random and the player AI's hands over seeds, and
//! follows each round with the mission layer (`mould::campaign::Tracker`):
//! the verdict, stars, the bar's best, dead time by phase, the longest dead
//! and stuck runs, and "decided at".
//!
//! cargo run --release --example campaign -- mission=1-landfall seeds=1-8 [hands=idle,random,boom,claim,late,fort,strike scale=2 walkers=5000 threads=4]
//!
//! A hand `PLAN+STYLE` plays a style with a scripted plan on top
//! (`gaps`, or one of `mould::plans::named`: `cut+boom`, `guard+boom`).
//!
//! `gpu=1` plays each round on the GPU engine (`gpu::larder_world::GpuWorld`,
//! the window's world) instead of the CPU big dish, and `gpu=both` on both,
//! the rows marked by engine and the summary given for each (GPU thread,
//! 2026-11-12: the Rim leaked in the GPU window where the CPU census held).
//!
//! The mould's tier is the mission's when `mould::tiers::Tier` has it; the
//! ladder names not yet built (drifter, sprawler, trunk) play the plain
//! mould, and the output says so.

use pixels::wgpu;
use slime_mould::gpu::block_on;
use slime_mould::gpu::larder_world::{race_settings, GpuWorld};
use slime_mould::mould::big::BigDish;
use slime_mould::mould::campaign::{self, decided_at, Goal, Mission, Tracker, BIN};
use slime_mould::mould::larder_world::{race_params, Cmd, Game, Order, Rules, Stance, World};
use slime_mould::mould::larder_world_ai;
use slime_mould::mould::map::Map;
use slime_mould::mould::own_mould::{self, Task};
use slime_mould::mould::plans;
use slime_mould::mould::tiers::{self, Tier};
use std::path::Path;

struct Row {
    hand: String,
    seed: u64,
    /// Played on the GPU engine.
    gpu: bool,
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
    /// Contain goals: the most 4x4-cell bins outside the region holding at
    /// least 1, 4 and 16 walkers in any one second, and the seconds with
    /// any bin of 4+ outside (what a player sees as mould out of the Rim).
    out_bins: [u32; 3],
    seen_out: u32,
    /// Crystals the mould reached in the round (map order), as a bit set.
    reached: u64,
}

fn main() {
    let (mut mission, mut seeds, mut hands, mut scale, mut walkers, mut threads) = (String::from("1-landfall"), (1u64, 8u64), String::from("idle,random,boom,claim,late,fort,strike"), 2usize, 5000usize, 4usize);
    let mut engines = vec![false];
    let mut keys: Vec<(String, String)> = Vec::new();
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
            // `keys=energy_flow:0.05+wither:0.01`: mould keys over the tier
            // (`tiers::set_key`, CPU only), to try a tier change first.
            "keys" => keys = v.split('+').filter(|k| !k.is_empty()).map(|kv| kv.split_once(':').map(|(a, b)| (a.to_string(), b.to_string())).expect("keys=K:V+K:V")).collect(),
            "gpu" => {
                engines = match v {
                    "0" => vec![false],
                    "1" => vec![true],
                    "both" => vec![false, true],
                    _ => panic!("gpu=0, 1 or both"),
                }
            }
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
    if !keys.is_empty() {
        println!("keys over the tier: {keys:?}");
    }
    let mut jobs: Vec<(String, u64, bool)> = Vec::new();
    for h in hands.split(',') {
        for s in seeds.0..=seeds.1 {
            for &g in &engines {
                jobs.push((h.to_string(), s, g));
            }
        }
    }
    println!("on {}", engines.iter().map(|&g| if g { "the GPU engine" } else { "the CPU" }).collect::<Vec<_>>().join(" and "));
    let next = std::sync::atomic::AtomicUsize::new(0);
    let rows = std::sync::Mutex::new(Vec::new());
    std::thread::scope(|sc| {
        for _ in 0..threads {
            sc.spawn(|| {
                // Each thread its own GPU device, once it plays a GPU round.
                let mut dev: Option<(wgpu::Device, wgpu::Queue)> = None;
                loop {
                    let i = next.fetch_add(1, std::sync::atomic::Ordering::Relaxed);
                    let Some((h, s, on_gpu)) = jobs.get(i) else { break };
                    if *on_gpu && dev.is_none() {
                        dev = Some(gpu());
                    }
                    let r = play(&m, &map, tier, &keys, *s, scale, walkers, h, dev.as_ref().filter(|_| *on_gpu));
                    eprintln!("  {h} s{s}{}: {} {:.0}s {}*", if *on_gpu { " GPU" } else { "" }, r.cause, r.at, r.stars);
                    rows.lock().unwrap().push(r);
                }
            });
        }
    });
    let mut rows = rows.into_inner().unwrap();
    rows.sort_by(|a, b| a.hand.cmp(&b.hand).then(a.seed.cmp(&b.seed)).then(a.gpu.cmp(&b.gpu)));
    let both = engines.len() > 1;
    println!("\nhand      seed  cause       at    stars best  dead  dead by phase (land/contact/pressure/climax)  longest dead/stuck/stall  decided  alive harvested killed  count least/most  out bins >=1/4/16, secs seen out  crystals reached");
    for r in &rows {
        println!(
            "{:<9} {:>4}{}  {:<10} {:>5.0}  {:>5} {:>4.2}  {:>4.0}%  {:>4.0}% {:>4.0}% {:>4.0}% {:>4.0}%                          {:>4} {:>4} {:>4}    {:>5}  {:>5} {:>9.0} {:>6}  {:.3}/{:.3}  {}/{}/{} {}  {:?}",
            r.hand,
            r.seed,
            if both { if r.gpu { " GPU" } else { " CPU" } } else { "" },
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
            r.count.1,
            r.out_bins[0],
            r.out_bins[1],
            r.out_bins[2],
            r.seen_out,
            (0..64).filter(|c| r.reached >> c & 1 == 1).collect::<Vec<_>>()
        );
    }
    println!("\nby hand: goal met / seeds, median stars, mean best, mean dead share, worst stuck run, worst stall");
    let mut names: Vec<&str> = rows.iter().map(|r| r.hand.as_str()).collect();
    names.dedup();
    for h in names {
        for &on_gpu in &engines {
            let rs: Vec<&Row> = rows.iter().filter(|r| r.hand == h && r.gpu == on_gpu).collect();
            summary(h, if both { if on_gpu { " GPU" } else { " CPU" } } else { "" }, &rs);
        }
    }
}

/// One hand's line of the summary, over `rs`.
fn summary(h: &str, engine: &str, rs: &[&Row]) {
    let mut stars: Vec<u8> = rs.iter().map(|r| r.stars).collect();
    stars.sort();
    let met = rs.iter().filter(|r| r.cause == "Goal").count();
    println!(
        "  {h:<9}{engine} {met}/{}  stars {}  best {:.2}  dead {:.0}%  stuck {} s  stall {} s",
        rs.len(),
        stars[stars.len() / 2],
        rs.iter().map(|r| r.best).sum::<f32>() / rs.len() as f32,
        100.0 * rs.iter().map(|r| r.dead).sum::<f32>() / rs.len() as f32,
        rs.iter().map(|r| r.longest_stuck).max().unwrap_or(0),
        rs.iter().map(|r| r.longest_stall).max().unwrap_or(0)
    );
}

/// A GPU device with the adapter's limits (as `examples/larderworld.rs`).
fn gpu() -> (wgpu::Device, wgpu::Queue) {
    let instance = wgpu::Instance::new(wgpu::InstanceDescriptor::new_without_display_handle_from_env());
    let adapter = block_on(instance.request_adapter(&wgpu::RequestAdapterOptions { power_preference: wgpu::PowerPreference::HighPerformance, ..Default::default() })).expect("no GPU adapter");
    eprintln!("campaign: GPU {}", adapter.get_info().name);
    block_on(adapter.request_device(&wgpu::DeviceDescriptor { label: Some("campaign"), required_limits: adapter.limits(), ..Default::default() })).expect("GPU device")
}

/// One round on the CPU big dish, or with `gpu` on the GPU engine (the
/// same map, params and tier as the window's `GpuWorld::with_map`).
#[allow(clippy::too_many_arguments)]
fn play(m: &Mission, map: &Map, tier: Option<Tier>, keys: &[(String, String)], seed: u64, scale: usize, walkers: usize, hand: &str, gpu: Option<&(wgpu::Device, wgpu::Queue)>) -> Row {
    let rules = m.rules(Rules::default());
    match gpu {
        None => {
            let mut p = race_params(seed, scale, walkers);
            if let Some(t) = tier {
                t.apply(&mut p);
            }
            for (k, v) in keys {
                tiers::set_key(&mut p, k, v).unwrap();
            }
            if m.own {
                // The player's own mould (mission 8): `walkers` a side.
                let mut world = own_mould::garden_on_map(p, map, walkers).unwrap();
                world.dish.threads = 1;
                return play_on(Game::new(world, rules, seed), m, seed, hand, false);
            }
            let mut world = BigDish::with_map(p, map);
            world.threads = 1;
            play_on(Game::new(world, rules, seed), m, seed, hand, false)
        }
        Some((device, queue)) => {
            let mut s = race_settings(seed, scale, walkers);
            if let Some(t) = tier {
                t.settings(&mut s).unwrap();
            }
            assert!(keys.is_empty(), "keys= is CPU only");
            let world = if m.own { GpuWorld::garden_on_map(device, queue, s, scale, map, walkers).unwrap() } else { GpuWorld::with_map(device, queue, s, scale, map) };
            play_on(Game::new(world, rules, seed), m, seed, hand, true)
        }
    }
}

fn play_on<W: World>(mut g: Game<W>, m: &Mission, seed: u64, hand: &str, on_gpu: bool) -> Row {
    let mut tr = Tracker::new(m.clone(), &g);
    // `gaps+STYLE`: the Rim's wall plan (one soldier walls each gap of the
    // ring, then STYLE plays everyone); the mission 3 verb's own hand.
    // `PLAN+STYLE` for the other plans (`mould::plans::named`: ring, pumps,
    // guard, turtle, zone, cut): STYLE farms each second, then the plan
    // builds or gives its few soldiers their orders, keeping STYLE's
    // reserve.
    // (A style's own knobs are joined by `+` too, `strike:strike_at=180+keep=2`,
    // so only a known plan name before the first `+` is a plan.)
    let (pre, style) = match hand.split_once('+') {
        Some((p, st)) if p == "gaps" || plans::named(p).is_some() => (p, st),
        _ => ("", hand),
    };
    let gaps = pre == "gaps";
    let plan = plans::named(pre);
    let reserve = larder_world_ai::Style::named(style).map_or(400.0, |s| s.reserve);
    let mut ai = larder_world_ai::Hand::named(style, &g);
    let mut walled = [false; 4];
    let tps = g.rules.tps as u64;
    let mut count = (f32::INFINITY, f32::NEG_INFINITY);
    let (mut out_bins, mut seen_out, mut reached) = ([0u32; 3], 0u32, 0u64);
    let v = loop {
        match hand {
            "random" => random_hand(&mut g),
            _ if gaps && walled.iter().any(|w| !w) && g.time() < 120.0 => campaign::wall_rim_gaps(&mut g, &mut walled),
            _ => {
                ai.as_mut().unwrap_or_else(|| panic!("unknown hand {hand}")).play(&mut g);
                if let Some(p) = plan {
                    p.play(&mut g, reserve);
                }
            }
        }
        let s = g.world.ticks() / tps;
        while g.world.ticks() / tps == s && g.end.is_none() {
            g.step();
        }
        let grid = g.world.mould_grid(BIN);
        // `CAMPAIGN_TRACE=1`: each side's walkers, the bank and the
        // buildings every 30 s (mission 8: who is winning, and how).
        if std::env::var_os("CAMPAIGN_TRACE").is_some() && (g.time() as u64).is_multiple_of(30) {
            eprintln!("    {hand} s{seed} t {:.0}: own {} enemy {} soldiers {} bank {:.0} killed {} buildings {}", g.time(), g.world.own_living(), g.mould, g.alive(), g.bank, g.killed, g.buildings.iter().filter(|b| b.alive).count());
        }
        if let Some(gr) = grid.as_deref() {
            let gw = g.world.size().0.div_ceil(BIN);
            for (c, ((x, y), _)) in g.world.crystals().into_iter().enumerate().take(64) {
                let r = g.world.crystal_r(c);
                let (bx0, bx1) = (((x - r) / BIN as f32).max(0.0) as usize, ((x + r) / BIN as f32) as usize);
                let (by0, by1) = (((y - r) / BIN as f32).max(0.0) as usize, ((y + r) / BIN as f32) as usize);
                'c: for by in by0..=by1 {
                    for bx in bx0..=bx1.min(gw - 1) {
                        if gr.get(by * gw + bx).is_some_and(|&n| n > 0) {
                            reached |= 1 << c;
                            break 'c;
                        }
                    }
                }
            }
        }
        if let (Goal::Contain { rect, .. }, Some(gr)) = (&m.goal, grid.as_deref()) {
            let fine = g.world.scale() / slime_mould::mould::map::MAP_SCALE;
            let gw = g.world.size().0.div_ceil(BIN);
            let mut n = [0u32; 3];
            for (i, &c) in gr.iter().enumerate() {
                let (px, py) = (((i % gw) * BIN) as f32 + BIN as f32 / 2.0, ((i / gw) * BIN) as f32 + BIN as f32 / 2.0);
                if px < rect.0 * fine || px > rect.2 * fine || py < rect.1 * fine || py > rect.3 * fine {
                    for (k, t) in [1, 4, 16].iter().enumerate() {
                        n[k] += (c >= *t) as u32;
                    }
                }
            }
            for k in 0..3 {
                out_bins[k] = out_bins[k].max(n[k]);
            }
            seen_out += (n[1] > 0) as u32;
        }
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
        gpu: on_gpu,
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
        out_bins,
        seen_out,
        reached,
    }
}

/// The census's random hand (as `examples/larderworld.rs`): every 5 s one
/// soldier gets a random order and stance; sometimes a buy or a wall.
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
    // With a mould of the player's own (mission 8), a random gardener's
    // task too, as `examples/garden.rs`'s random hand (drawn only there, so
    // the other missions' random rounds are unchanged).
    if g.world.own_living() > 0 {
        let t = [Task::None, Task::Road, Task::Feed, Task::Cut][g.rng.below(4) as usize];
        g.world.set_task(k, t);
    }
}
