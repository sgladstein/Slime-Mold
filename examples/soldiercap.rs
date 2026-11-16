//! **What a soldier costs the game's CPU step** (soldier-cap thread,
//! 2026-11-16): `cargo run --release --example soldiercap -- [depth=1]
//! [seed=1] [n=40,80,120,160,240] [secs=60] [squads=8]`.
//!
//! Plays a run planet (`levelgen::level`) on the CPU big dish in squad
//! mode (the window's rules for `mode=squads`), buys `n` soldiers at once
//! into `squads` squads, and sends half the squads to fight at crystals
//! and half to clear crystals, a new pick every 15 game seconds so the
//! walking maps keep changing. Each step's time is split into the world's
//! frame (`World::frame`, the GPU's work in the window) and the rest: the
//! soldiers' turn (walking, maps, aims, shots, squads, buildings), which
//! the window runs on the CPU at any engine. Prints mean, p95 and max of
//! the rest, ms a step, for each `n`.
//!
//! `fight=1` sends every squad to fight at the slime's thickest blocks.
//! `squad::UNITS_MAX` caps the units the frame reads (256: 200 soldiers,
//! the home and 40 buildings; 96 before 11-16, when a soldier past about
//! 55 dropped out silently). Report: `Reports/soldier-cap-2026-11-16/`.
use slime_mould::mould::big::BigDish;
use slime_mould::mould::larder::Order;
use slime_mould::mould::larder_world::{race_params, Cmd, Game, Rules, Soldier, World};
use slime_mould::mould::levelgen::level;
use slime_mould::mould::squad::{self, Frame, FrameOut};
use std::time::Instant;

/// The big dish with the time spent in its frames counted.
struct Timed {
    d: BigDish,
    frame_s: f64,
}

impl World for Timed {
    fn size(&self) -> (usize, usize) {
        World::size(&self.d)
    }
    fn scale(&self) -> f32 {
        World::scale(&self.d)
    }
    fn ticks(&self) -> u64 {
        World::ticks(&self.d)
    }
    fn frame(&mut self, f: &Frame, ticks: u32) -> FrameOut {
        let t = Instant::now();
        let out = World::frame(&mut self.d, f, ticks);
        self.frame_s += t.elapsed().as_secs_f64();
        out
    }
    fn build_wall(&mut self, a: (f32, f32), b: (f32, f32), width: f32) {
        World::build_wall(&mut self.d, a, b, width)
    }
    fn living(&self) -> usize {
        World::living(&self.d)
    }
    fn mass(&self) -> usize {
        World::mass(&self.d)
    }
    fn own_living(&self) -> usize {
        World::own_living(&self.d)
    }
    fn crystals(&self) -> Vec<((f32, f32), f32)> {
        World::crystals(&self.d)
    }
    fn crystal_r(&self, c: usize) -> f32 {
        World::crystal_r(&self.d, c)
    }
    fn landing(&self) -> Option<(f32, f32)> {
        World::landing(&self.d)
    }
    fn open(&self, x: f32, y: f32) -> bool {
        World::open(&self.d, x, y)
    }
    fn open_cells(&self) -> Vec<bool> {
        World::open_cells(&self.d)
    }
    fn clear(&self, x: f32, y: f32) -> bool {
        World::clear(&self.d, x, y)
    }
    fn stop_births(&mut self) {
        World::stop_births(&mut self.d)
    }
    fn upkeep(&self) -> f32 {
        World::upkeep(&self.d)
    }
    fn set_upkeep(&mut self, up: f32) {
        World::set_upkeep(&mut self.d, up)
    }
    fn set_panic_r(&mut self, r: f32) {
        World::set_panic_r(&mut self.d, r)
    }
    fn surge_every(&self) -> u32 {
        World::surge_every(&self.d)
    }
    fn set_surge_at(&mut self, at: u32) {
        World::set_surge_at(&mut self.d, at)
    }
    fn set_raid(&mut self, at: u32, to: (f32, f32), r: f32, share: f32, field: Vec<u32>) {
        World::set_raid(&mut self.d, at, to, r, share, field)
    }
    fn raid_ground(&self) -> Option<(Vec<bool>, usize, usize)> {
        World::raid_ground(&self.d)
    }
    fn food_deposit(&self) -> f32 {
        World::food_deposit(&self.d)
    }
    fn fine_steps(&self) -> u32 {
        World::fine_steps(&self.d)
    }
    fn hash(&self) -> u64 {
        World::hash(&self.d)
    }
    fn mould_grid(&self, bin: usize) -> Option<Vec<u32>> {
        World::mould_grid(&self.d, bin)
    }
    fn calm(&self, at: &[(f32, f32)], r: f32) -> Vec<u32> {
        World::calm(&self.d, at, r)
    }
    fn tend(&mut self, s: &[Soldier], bank: f32, ticks: u32) -> f32 {
        World::tend(&mut self.d, s, bank, ticks)
    }
    fn burst(&mut self, b: &squad::Burst, k: &squad::BurstRules) -> squad::BurstOut {
        World::burst(&mut self.d, b, k)
    }
    fn burned(&mut self) -> u32 {
        World::burned(&mut self.d)
    }
    fn has_gas(&self) -> bool {
        World::has_gas(&self.d)
    }
    fn has_germ(&self) -> bool {
        World::has_germ(&self.d)
    }
    fn walkers_in(&self, at: (f32, f32), r: f32) -> u32 {
        World::walkers_in(&self.d, at, r)
    }
}

fn main() {
    let (mut depth, mut seed, mut secs, mut nsq) = (1u32, 1u64, 60.0f32, 8usize);
    // `fight=1`: every squad fights at the slime's thickest blocks.
    let mut fight = false;
    // `greedy=1`: the window's `greedy` hand from the normal bank.
    let mut greedy = false;
    let mut ns = vec![40usize, 80, 120, 160, 240];
    for a in std::env::args().skip(1) {
        let (k, v) = a.split_once('=').expect("key=value");
        match k {
            "depth" => depth = v.parse().unwrap(),
            "seed" => seed = v.parse().unwrap(),
            "secs" => secs = v.parse().unwrap(),
            "squads" => nsq = v.parse().unwrap(),
            "fight" => fight = v == "1",
            "greedy" => greedy = v == "1",
            "n" => ns = v.split(',').map(|s| s.parse().unwrap()).collect(),
            _ => panic!("no key {k}"),
        }
    }
    println!("soldiercap: fight {fight}, planet depth {depth} seed {seed}, {secs} game s, {nsq} squads, UNITS_MAX {}, SHOTS_MAX {}", squad::UNITS_MAX, squad::SHOTS_MAX);
    for &n in &ns {
        let map = level(seed, depth);
        let d = BigDish::with_map(race_params(seed, 2, 25_000), &map);
        let (w, h) = d.size();
        let world = Timed { d, frame_s: 0.0 };
        let bank = if greedy { Rules::default().start_bank } else { 1e9 };
        let mut rules = Rules { squads: true, larder: false, seek: false, overrun: 0.0, max_squad: n, start_bank: bank, ..Rules::default() };
        rules.mass = Rules::mass_for(25_000, 2);
        let mut g = Game::new(world, rules, seed);
        if greedy {
            greedy_run(&mut g, secs);
            continue;
        }
        while g.buy() {}
        let live: Vec<usize> = (0..g.soldiers.len()).filter(|&k| g.soldiers[k].alive).collect();
        for (i, &k) in live.iter().enumerate() {
            g.apply(Cmd::Join(k, (i * nsq / live.len()) as u8));
        }
        let crystals = g.world.crystals();
        let steps = (secs * g.rules.tps as f32) as usize;
        let (mut rest, mut frames) = (Vec::with_capacity(steps), 0.0f64);
        let mut pick = 0usize;
        let mut peak = 0usize;
        for s in 0..steps {
            if s % (15 * g.rules.tps as usize) == 0 {
                // The slime's thickest blocks (fine cells), for `fight`.
                let bin = 8usize;
                let thick: Vec<(f32, f32)> = World::mould_grid(&g.world, bin).map_or_else(Vec::new, |m| {
                    let gw = w.div_ceil(bin);
                    let mut ix: Vec<usize> = (0..m.len()).collect();
                    ix.sort_by_key(|&i| std::cmp::Reverse(m[i]));
                    ix.iter().take(nsq * 4).map(|&i| (((i % gw) * bin + bin / 2) as f32, ((i / gw) * bin + bin / 2) as f32)).collect()
                });
                for q in 0..nsq {
                    pick = (pick * 7 + 3 + q) % crystals.len();
                    let (x, y) = crystals[pick].0;
                    let o = if fight && !thick.is_empty() {
                        let t = thick[(q * 4) % thick.len()];
                        Order::Fight(t.0, t.1)
                    } else if q % 2 == 0 {
                        Order::Fight(x, y)
                    } else {
                        Order::Clear(x, y)
                    };
                    g.apply(Cmd::SquadOrder(q as u8, o));
                }
            }
            let f0 = g.world.frame_s;
            let t = Instant::now();
            g.step();
            let all = t.elapsed().as_secs_f64();
            let f = g.world.frame_s - f0;
            frames += f;
            peak = peak.max(g.bullets.len());
            rest.push((all - f) * 1e3);
            if g.end.is_some() {
                break;
            }
        }
        let k = rest.len();
        let mean = rest.iter().sum::<f64>() / k as f64;
        let mut sorted = rest.clone();
        sorted.sort_by(f64::total_cmp);
        let p95 = sorted[(k * 95 / 100).min(k - 1)];
        let max = sorted[k - 1];
        println!(
            "  n {n:>4}: world {w}x{h}, {k} steps, {} alive at end, {} squads live, {} shots, {} bullets in flight at most, {} dropped | soldiers' turn ms/step mean {mean:.3} p95 {p95:.3} max {max:.2} | frame (CPU world) mean {:.2}",
            g.alive(),
            g.live_squads().len(),
            g.shots,
            peak,
            g.bullets_dropped,
            frames * 1e3 / k as f64
        );
    }
}

/// **How fast the greedy hand grows an army** (the game designer's ask on
/// #377, 11-16: the cap of 40 was the snowball's only brake): the window's
/// `greedy` hand (`hands.rs`), once a game second: buy whenever the bank
/// can pay, and every soldier not on a crystal with charge left drains the
/// biggest one. Prints the game time each army size is first reached, and
/// the army and bank every minute.
fn greedy_run(g: &mut Game<Timed>, secs: f32) {
    let marks = [10usize, 20, 40, 80, 120, 150, 200];
    let mut first: Vec<Option<f32>> = vec![None; marks.len()];
    let tps = g.rules.tps as u64;
    let mut minute = 0;
    while g.end.is_none() && g.time() < secs {
        if g.world.ticks().is_multiple_of(tps) {
            while g.bank >= g.rules.soldier_cost && g.alive() < g.rules.max_squad {
                if !g.apply(Cmd::Buy) {
                    break;
                }
            }
            let cs = g.world.crystals();
            if let Some(c) = (1..cs.len()).filter(|&c| cs[c].1 > 0.0).max_by(|&a, &b| cs[a].1.total_cmp(&cs[b].1)).map(|c| cs[c].0) {
                for k in 0..g.soldiers.len() {
                    let on = matches!(g.soldiers[k].order, Order::Clear(x, y) if g.crystal_at((x, y)).is_some_and(|(i, _)| cs[i].1 > 0.0));
                    if g.soldiers[k].alive && !on {
                        g.apply(Cmd::Order(k, Order::Clear(c.0, c.1)));
                    }
                }
            }
        }
        g.step();
        let n = g.alive();
        for (i, &m) in marks.iter().enumerate() {
            if n >= m && first[i].is_none() {
                first[i] = Some(g.time());
            }
        }
        if g.time() >= 60.0 * (minute + 1) as f32 {
            minute += 1;
            println!("    {minute}:00  soldiers {n}, bank {:.0}, harvested {:.0}, slime {}", g.bank, g.harvested, g.world.living());
        }
    }
    let at: Vec<String> = marks.iter().zip(&first).map(|(m, t)| format!("{m}: {}", t.map_or("-".into(), |t| format!("{}:{:02}", t as u32 / 60, t as u32 % 60)))).collect();
    println!("  greedy, cap {}: first reached {} | at the end ({:.0} s) {} alive, {} lost", g.rules.max_squad, at.join(", "), g.time(), g.alive(), g.soldiers.iter().filter(|s| !s.alive).count());
}
