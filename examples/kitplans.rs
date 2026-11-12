//! **The squad toolkit's plans** (`mould::kit`, 2026-11-12): four plans,
//! each built on a different tool pair, against idle, random and the farm
//! hand, for each mould tier, on the CPU race world (or a map).
//!
//! ```text
//! cargo run --release --example kitplans -- seeds=1-8 [tiers=none,swarmer,sleeper,sporer hands=... cap=600 threads=4 map=fork look=DIR]
//! ```
//!
//! **Deciding measure, named before the run:** `Game::score` (v2) a game,
//! paired by seed. Two gates, named before the run (the technical lead's
//! review, 04:40Z): (1) each plan beats idle and random, the random hand
//! with the tools too (`randkit`), on each tier, sign test, Holm over the
//! comparisons made; (2) the payoff table, plan by tier: passes when the
//! best plan (highest mean score) is not the same in every column and no
//! plan is within noise of the best everywhere (not lower than the best on
//! a significant count of pairs in any column).
//!
//! **Hands** (each also farms as `farm` does: soldier k clears the k-th
//! nearest crystal with charge, buying a soldier over `RESERVE` up to
//! `FARM_MAX`; tools are paid from the same bank, and only above
//! `TOOL_RESERVE`):
//! - **idle**, **random**: as `examples/larderworld.rs`.
//! - **randkit**: random, and every 5 s with chance 1/2 a random tool at a
//!   random open point within reach of a random live soldier.
//! - **farm**: the farm hand alone, on Defend.
//! - **loud** (blast, mine, reek): a reeking soldier holds at the larder
//!   with four mines round it (8 dish cells out), re-laid as they go off;
//!   once a second, a blast on the densest 8 x 8 block in reach of any
//!   soldier if it holds 12 walkers; and a runner (as plague's) carries a
//!   blast to the mould's densest block every 3 s. Free stance.
//! - **quiet** (mask, gas): every soldier masked, on Hold Fire; a
//!   harvester with mould within 10 dish cells gasses itself (no two gas
//!   clouds within 12 dish cells in 8 s).
//! - **plague** (germ): Defend; when no walker is sick and the last
//!   wave's immunity is over, a runner (the first live soldier) walks to
//!   within reach of the mould's densest 8 x 8 block anywhere, germs it,
//!   and goes back to farming. (A first version germed only mould already
//!   in reach of the farmers: no block there ever held 24 walkers, so it
//!   never fired in 240 s on two seeds.)
//! - **siege** (det cord): Defend; every 20 s a cord 24 dish cells long
//!   across the mould at the aim nearest the larder, square to the line
//!   from the core.
use slime_mould::mould::big::BigDish;
use slime_mould::mould::kit::{Kit, KitRules, Tool};
use slime_mould::mould::larder_world::{race_params, Cmd, Game, Order, Rules, Stance, World};
use slime_mould::mould::map::Map;
use slime_mould::mould::tiers::Tier;

const RESERVE: f32 = 600.0;
const TOOL_RESERVE: f32 = 200.0;
const FARM_MAX: usize = 8;

#[derive(Clone, Default)]
struct Row {
    score: f32,
    end: String,
    t: f32,
    alive: usize,
    mould: usize,
    harvested: f32,
    uses: [u32; 6],
    killed: [usize; 6],
    shots: u32,
    ms_tick: f64,
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

fn by_distance(g: &Game<BigDish>) -> Vec<(f32, f32)> {
    let l = g.larder;
    let r = g.rules;
    let dry = r.harvest * r.frame_ticks as f32 / r.tps as f32 * r.charge;
    let mut cs: Vec<(f32, f32)> = g.world.crystals().iter().filter(|c| c.1 >= dry).map(|c| c.0).filter(|c| *c != g.core).collect();
    cs.sort_by(|a, b| (a.0 - l.0).hypot(a.1 - l.1).total_cmp(&(b.0 - l.0).hypot(b.1 - l.1)));
    cs
}

/// `examples/larderworld.rs`'s farm hand; `skip` soldiers are left alone
/// (the loud plan's decoy), `stance` is set on each farmer once.
fn farm_hand(g: &mut Game<BigDish>, farmed: &mut Vec<usize>, skip: &[usize], stance: Stance) {
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
            if !farmed.contains(&k) {
                farmed.push(k);
                g.apply(Cmd::Stance(k, stance));
            }
        }
    }
}

fn random_point(g: &mut Game<BigDish>) -> (f32, f32) {
    let (w, h) = g.world.size();
    loop {
        let p = (g.rng.f32() * w as f32, g.rng.f32() * h as f32);
        if g.world.open(p.0, p.1) {
            break p;
        }
    }
}

fn random_hand(g: &mut Game<BigDish>) {
    if !(g.time() as u64).is_multiple_of(5) {
        return;
    }
    let live: Vec<usize> = (0..g.soldiers.len()).filter(|&k| g.soldiers[k].alive).collect();
    if live.is_empty() {
        g.apply(Cmd::Buy);
        return;
    }
    let k = live[g.rng.below(live.len() as u32) as usize];
    let order = match g.rng.below(4) {
        0 => {
            let p = random_point(g);
            Order::Move(p.0, p.1)
        }
        1 => {
            let cs = g.world.crystals();
            let c = cs[g.rng.below(cs.len() as u32) as usize].0;
            Order::Clear(c.0, c.1)
        }
        2 => {
            let p = random_point(g);
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

fn random_kit(g: &mut Game<BigDish>, kit: &mut Kit) {
    if !(g.time() as u64).is_multiple_of(5) || g.rng.below(2) == 0 || g.bank < TOOL_RESERVE {
        return;
    }
    let live: Vec<usize> = (0..g.soldiers.len()).filter(|&k| g.soldiers[k].alive).collect();
    if live.is_empty() {
        return;
    }
    let k = live[g.rng.below(live.len() as u32) as usize];
    let s = g.soldiers[k];
    let reach = kit.rules.reach * g.world.scale();
    let near = |g: &mut Game<BigDish>| loop {
        let (a, r) = (g.rng.f32() * std::f32::consts::TAU, g.rng.f32() * reach);
        let p = (s.x + r * a.cos(), s.y + r * a.sin());
        if g.world.open(p.0, p.1) {
            break p;
        }
    };
    let p = near(g);
    let t = match g.rng.below(6) {
        0 => Tool::Blast(p),
        1 => Tool::Mine(p),
        2 => {
            let q = near(g);
            Tool::Cord(p, q)
        }
        3 => Tool::Germ(p),
        4 => Tool::Gas(p),
        _ => Tool::Smell(k, [0.0, 1.0, kit.rules.reek][g.rng.below(3) as usize]),
    };
    kit.use_tool(g, t);
}

/// Live soldiers' aims within reach, with the walkers in each.
fn aims(g: &Game<BigDish>, kit: &Kit) -> Vec<(usize, (f32, f32), u32)> {
    let reach = kit.rules.reach * g.world.scale();
    (0..g.soldiers.len())
        .filter_map(|k| {
            let s = g.soldiers[k];
            let a = s.aim?;
            (s.alive && (a.0 - s.x).hypot(a.1 - s.y) <= reach).then_some((k, a, s.dense))
        })
        .collect()
}

/// The centre (fine cells) of the 8 x 8 block with the most walkers
/// within reach of a live soldier, and its walkers (the first scripted
/// plans aimed at each soldier's own aim, a 4 x 4 block of at most 16
/// walkers: 136 blasts in 240 s killed 398, and a germ there reached no
/// body).
fn densest_in_reach(g: &Game<BigDish>, kit: &Kit) -> Option<((f32, f32), u32)> {
    let b = 8;
    let grid = g.world.mould_grid(b)?;
    let gw = g.world.w.div_ceil(b);
    let reach = kit.rules.reach * g.world.scale();
    let mut best: Option<((f32, f32), u32)> = None;
    for (i, &n) in grid.iter().enumerate() {
        if n == 0 || best.is_some_and(|x| x.1 >= n) {
            continue;
        }
        let c = (((i % gw) * b + b / 2) as f32, ((i / gw) * b + b / 2) as f32);
        if g.world.open(c.0, c.1) && g.soldiers.iter().any(|s| s.alive && (s.x - c.0).hypot(s.y - c.1) <= reach) {
            best = Some((c, n));
        }
    }
    best
}

#[derive(Default)]
struct Plan {
    farmed: Vec<usize>,
    decoy: Option<usize>,
    last_blast: f32,
    gas: Vec<((f32, f32), f32)>,
    last_cord: f32,
    started: bool,
    runner: Option<usize>,
}

fn loud(g: &mut Game<BigDish>, kit: &mut Kit, st: &mut Plan) {
    let s = g.world.scale();
    if st.decoy.is_none() {
        let k = (0..g.soldiers.len()).find(|&k| g.soldiers[k].alive);
        if let Some(k) = k {
            st.decoy = Some(k);
            let l = g.larder;
            g.apply(Cmd::Order(k, Order::Move(l.0, l.1)));
            g.apply(Cmd::Stance(k, Stance::Free));
            kit.use_tool(g, Tool::Smell(k, kit.rules.reek));
        }
    }
    let skip: Vec<usize> = st.decoy.into_iter().chain(st.runner).collect();
    farm_hand(g, &mut st.farmed, &skip, Stance::Free);
    // A runner carries blasts to the mould's body, one each 3 s.
    if g.time() - st.last_cord >= 3.0 && g.bank > TOOL_RESERVE + kit.rules.blast_price && sortie(g, kit, st, Tool::Blast) {
        st.last_cord = g.time();
    }
    // The minefield round the larder: four places, each re-laid when empty.
    let l = g.larder;
    for j in 0..4 {
        let a = j as f32 * std::f32::consts::FRAC_PI_2 + std::f32::consts::FRAC_PI_4;
        let m = (l.0 + 8.0 * s * a.cos(), l.1 + 8.0 * s * a.sin());
        let laid = kit.mines.iter().any(|q| (q.0 - m.0).hypot(q.1 - m.1) < 1.0);
        if !laid && g.bank > TOOL_RESERVE && g.world.open(m.0, m.1) {
            kit.use_tool(g, Tool::Mine(m));
        }
    }
    // A blast a second at most, on the densest mould in reach once it is
    // thick (12 of an 8 x 8 block's 64 cells; at 24 none fired in 240 s).
    if g.time() - st.last_blast >= 1.0 && g.bank > TOOL_RESERVE {
        if let Some((a, n)) = densest_in_reach(g, kit) {
            if n >= 12 && kit.use_tool(g, Tool::Blast(a)) {
                st.last_blast = g.time();
            }
        }
    }
}

fn quiet(g: &mut Game<BigDish>, kit: &mut Kit, st: &mut Plan) {
    let s = g.world.scale();
    for k in 0..g.soldiers.len() {
        if g.soldiers[k].alive && g.soldiers[k].smell != 0.0 {
            kit.use_tool(g, Tool::Smell(k, 0.0));
        }
    }
    farm_hand(g, &mut st.farmed, &[], Stance::HoldFire);
    let t = g.time();
    st.gas.retain(|&(_, at)| t - at < kit.rules.gas_secs);
    for k in 0..g.soldiers.len() {
        let x = g.soldiers[k];
        let Some(a) = x.aim else { continue };
        if !x.alive || !(x.touching > 0 || (x.dense >= 8 && (a.0 - x.x).hypot(a.1 - x.y) <= 10.0 * s)) {
            continue;
        }
        let p = (x.x, x.y);
        if st.gas.iter().any(|&(q, _)| (q.0 - p.0).hypot(q.1 - p.1) < 12.0 * s) || g.bank < TOOL_RESERVE {
            continue;
        }
        if kit.use_tool(g, Tool::Gas(p)) {
            st.gas.push((p, t));
        }
    }
}

fn plague(g: &mut Game<BigDish>, kit: &mut Kit, st: &mut Plan) {
    let skip: Vec<usize> = st.runner.into_iter().collect();
    farm_hand(g, &mut st.farmed, &skip, Stance::Defend);
    let sick = !g.world.sick.is_empty() && g.world.sick.iter().any(|&c| c > g.world.p.immune_steps);
    if sick || g.bank < TOOL_RESERVE + kit.rules.germ_price {
        return;
    }
    // Wait out the immunity of the last wave: a germ on immune walkers is
    // spent for nothing.
    if st.started && g.time() - st.last_cord < kit.rules.sick_secs + kit.rules.immune_secs {
        return;
    }
    if sortie(g, kit, st, Tool::Germ) {
        st.started = true;
        st.last_cord = g.time();
    }
}

/// A runner (the first live soldier not the decoy) walks to within reach
/// of the mould's densest 8 x 8 block anywhere and uses `tool` there, then
/// goes back to farming. Returns whether the tool was used this call.
fn sortie(g: &mut Game<BigDish>, kit: &mut Kit, st: &mut Plan, tool: fn((f32, f32)) -> Tool) -> bool {
    let Some((a, _)) = densest(g) else { return false };
    let Some(k) = st.runner.filter(|&k| g.soldiers[k].alive).or_else(|| (0..g.soldiers.len()).find(|&k| g.soldiers[k].alive && Some(k) != st.decoy)) else { return false };
    st.runner = Some(k);
    // With the carry rule an empty runner fetches tools from the larder
    // first (`play` restocks it there to `carry_max`).
    if kit.rules.carry_max > 0 && kit.held(k) == 0 {
        let l = g.larder;
        if g.soldiers[k].order != Order::Move(l.0, l.1) {
            g.apply(Cmd::Order(k, Order::Move(l.0, l.1)));
        }
        return false;
    }
    if kit.use_tool(g, tool(a)) {
        if kit.held(k) > 0 {
            return true;
        }
        st.runner = None;
        // Back to farming: the farm hand gives it a crystal again.
        g.apply(Cmd::Order(k, Order::Hold));
        st.farmed.retain(|&x| x != k);
        true
    } else {
        // Walk toward it, stopping short so the mould's edge is in reach.
        let x = g.soldiers[k];
        let (dx, dy) = (a.0 - x.x, a.1 - x.y);
        let d = dx.hypot(dy).max(1.0);
        let stop = 0.7 * kit.rules.reach * g.world.scale();
        let to = if d > stop { (a.0 - dx / d * stop, a.1 - dy / d * stop) } else { (x.x, x.y) };
        if g.soldiers[k].order != Order::Move(to.0, to.1) {
            g.apply(Cmd::Order(k, Order::Move(to.0, to.1)));
        }
        false
    }
}

/// The centre (fine cells) of the 8 x 8 block with the most walkers
/// anywhere, and its walkers.
fn densest(g: &Game<BigDish>) -> Option<((f32, f32), u32)> {
    let b = 8;
    let grid = g.world.mould_grid(b)?;
    let gw = g.world.w.div_ceil(b);
    let mut best: Option<((f32, f32), u32)> = None;
    for (i, &n) in grid.iter().enumerate() {
        let c = (((i % gw) * b + b / 2) as f32, ((i / gw) * b + b / 2) as f32);
        if n > 0 && best.is_none_or(|x| n > x.1) && g.world.open(c.0, c.1) {
            best = Some((c, n));
        }
    }
    best
}

fn siege(g: &mut Game<BigDish>, kit: &mut Kit, st: &mut Plan) {
    farm_hand(g, &mut st.farmed, &[], Stance::Defend);
    if g.time() - st.last_cord < 20.0 && st.started {
        return;
    }
    let (l, c, s) = (g.larder, g.core, g.world.scale());
    let Some(&(_, a, _)) = aims(g, kit).iter().min_by(|x, y| (x.1 .0 - l.0).hypot(x.1 .1 - l.1).total_cmp(&(y.1 .0 - l.0).hypot(y.1 .1 - l.1))) else { return };
    let (dx, dy) = (a.0 - c.0, a.1 - c.1);
    let d = dx.hypot(dy).max(1.0);
    let half = 12.0 * s;
    let (p, q) = ((a.0 - dy / d * half, a.1 + dx / d * half), (a.0 + dy / d * half, a.1 - dx / d * half));
    if g.bank > TOOL_RESERVE && kit.use_tool(g, Tool::Cord(p, q)) {
        st.started = true;
        st.last_cord = g.time();
    }
}

fn play(seed: u64, tier: &str, hand: &str, rules: Rules, kr: KitRules, map: Option<&Map>, look: Option<&str>) -> Row {
    let mut p = race_params(seed, 2, 5000);
    if let Some(t) = Tier::from_name(tier) {
        t.apply(&mut p);
    }
    // Germ resistance (round 2), a mould trait tried on any tier: GERM_RESIST=
    // and SICK_TRAIL= in the environment.
    if let Ok(v) = std::env::var("GERM_RESIST") {
        p.germ_resist = v.parse().unwrap();
    }
    if let Ok(v) = std::env::var("SICK_TRAIL") {
        p.sick_trail = v.parse().unwrap();
    }
    let mut world = match map {
        Some(m) => BigDish::with_map(p, m),
        None => BigDish::new(p),
    };
    world.threads = 1;
    let mut g = Game::new(world, rules, seed);
    let mut kit = Kit::new(kr);
    let mut st = Plan::default();
    let start = std::time::Instant::now();
    let tps = g.rules.tps as u64;
    let mut last = u64::MAX;
    let mut shot = -60.0f32;
    while g.end.is_none() {
        let sec = g.world.ticks() / tps;
        if sec != last {
            last = sec;
            // The carry rule: every soldier at the larder takes one tool,
            // a runner as many as it may hold (the tools are paid for when
            // used). Smell needs none.
            if kit.rules.carry_max > 0 {
                for k in 0..g.soldiers.len() {
                    let n = if st.runner == Some(k) || st.decoy == Some(k) { kit.rules.carry_max } else { kit.rules.carry_each };
                    if n > 0 {
                        kit.restock(&mut g, k, n);
                    }
                }
            }
            match hand {
                "idle" => {}
                "random" => random_hand(&mut g),
                "randkit" => {
                    random_hand(&mut g);
                    random_kit(&mut g, &mut kit);
                }
                "farm" => farm_hand(&mut g, &mut st.farmed, &[], Stance::Defend),
                "loud" => loud(&mut g, &mut kit, &mut st),
                "quiet" => quiet(&mut g, &mut kit, &mut st),
                "plague" => plague(&mut g, &mut kit, &mut st),
                "siege" => siege(&mut g, &mut kit, &mut st),
                _ => panic!("unknown hand {hand}"),
            }
            if let Some(dir) = look {
                if g.time() - shot >= 60.0 {
                    shot = g.time();
                    g.world.image(1).save(format!("{dir}/{tier}-{hand}-s{seed}-{:04.0}s.png", g.time())).unwrap();
                }
            }
        }
        kit.frame(&mut g);
        g.step();
    }
    let end = g.end.unwrap();
    Row {
        score: g.score().expect("ended"),
        end: end.band().into(),
        t: g.time(),
        alive: g.alive(),
        mould: g.world.living(),
        harvested: g.harvested,
        uses: kit.uses,
        killed: kit.killed,
        shots: g.shots,
        ms_tick: start.elapsed().as_secs_f64() * 1000.0 / g.world.ticks().max(1) as f64,
    }
}

fn main() {
    let mut seeds: Vec<u64> = (1..=8).collect();
    let mut tiers: Vec<String> = ["none", "swarmer", "sleeper", "sporer"].iter().map(|s| s.to_string()).collect();
    let mut hands: Vec<String> = ["idle", "random", "randkit", "farm", "loud", "quiet", "plague", "siege"].iter().map(|s| s.to_string()).collect();
    let (mut threads, mut look) = (4usize, None::<String>);
    let mut rules = Rules { cap: 600.0, ..Rules::default() };
    let mut map = None::<(String, Map)>;
    let mut kr = KitRules::default();
    for a in std::env::args().skip(1) {
        let (k, v) = a.split_once('=').expect("key=value");
        match k {
            "seeds" => {
                let (a, b) = v.split_once('-').unwrap_or((v, v));
                seeds = (a.parse().unwrap()..=b.parse().unwrap()).collect();
            }
            "tiers" => tiers = v.split(',').map(String::from).collect(),
            "hands" => hands = v.split(',').map(String::from).collect(),
            "threads" => threads = v.parse().unwrap(),
            "cap" => rules.cap = v.parse().unwrap(),
            "price" => rules.price = v.parse().unwrap(),
            "look" => look = Some(v.into()),
            "reach" => kr.reach = v.parse().unwrap(),
            "blast_price" => kr.blast_price = v.parse().unwrap(),
            "blast_r" => kr.blast_r = v.parse().unwrap(),
            "germ_price" => kr.germ_price = v.parse().unwrap(),
            "sick_upkeep" => kr.sick_upkeep = v.parse().unwrap(),
            "gas_price" => kr.gas_price = v.parse().unwrap(),
            "sick_catch" => kr.sick_catch = v.parse().unwrap(),
            "carry" => kr.carry_max = v.parse().unwrap(),
            "carry_each" => kr.carry_each = v.parse().unwrap(),
            "tool_weight" => kr.tool_weight = v.parse().unwrap(),
            "map" => {
                let dir = std::path::Path::new(env!("CARGO_MANIFEST_DIR")).join("Reports/larder-maps-2026-11-11/maps");
                map = Some((v.to_string(), Map::load(&dir, v).unwrap_or_else(|e| panic!("{e}"))));
            }
            _ => panic!("unknown {k}"),
        }
    }
    if let Some(d) = &look {
        std::fs::create_dir_all(d).unwrap();
    }
    let world = map.as_ref().map_or("the race world".to_string(), |(n, _)| format!("map {n}"));
    println!("kitplans {world}, cap {} s, seeds {seeds:?}, tiers {tiers:?}; rules {rules:?}; kit {:?}", rules.cap, kr);
    let mapr = map.as_ref().map(|(_, m)| m);
    let mut jobs: Vec<(String, u64, String)> = Vec::new();
    for t in &tiers {
        for &s in &seeds {
            for h in &hands {
                jobs.push((t.clone(), s, h.clone()));
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
                let (tier, seed, hand) = &jobs[j];
                let lk = if *seed == seeds[0] { look.as_deref() } else { None };
                let r = play(*seed, tier, hand, rules, kr, mapr, lk);
                println!(
                    "{tier:>8} {hand:>7} seed {seed:>2}: {:>11} at {:>4.0} s, score {:+.3}; soldiers {:>2}, mould {:>6}, harvested {:>6.0}, shots {:>5}; uses {:?}, tool kills {:?}; {:.2} ms a tick",
                    r.end, r.t, r.score, r.alive, r.mould, r.harvested, r.shots, r.uses, r.killed, r.ms_tick
                );
                out.lock().unwrap()[j] = r;
            });
        }
    });
    let out = out.into_inner().unwrap();
    let get = |t: &str, s: u64, h: &str| out[jobs.iter().position(|(a, b, c)| a == t && *b == s && c == h).unwrap()].clone();
    println!("\npayoff table (mean score; ends mould wiped / cap / squad wiped):");
    print!("{:>8}", "");
    for t in &tiers {
        print!(" {t:>22}");
    }
    println!();
    for h in &hands {
        print!("{h:>8}");
        for t in &tiers {
            let rows: Vec<Row> = seeds.iter().map(|&s| get(t, s, h)).collect();
            let mean = rows.iter().map(|r| r.score).sum::<f32>() / rows.len() as f32;
            let e = |b: &str| rows.iter().filter(|r| r.end == b).count();
            print!(" {mean:>+8.3} ({}/{}/{})     ", e("mould wiped"), e("cap"), e("squad wiped"));
        }
        println!();
    }
    // Gate 1: each plan against idle, random and randkit, per tier.
    let plans: Vec<&String> = hands.iter().filter(|h| ["loud", "quiet", "plague", "siege", "farm"].contains(&h.as_str())).collect();
    let mut tests = Vec::new();
    for t in &tiers {
        for h in &plans {
            for base in ["idle", "random", "randkit"] {
                if !hands.iter().any(|x| x == base) {
                    continue;
                }
                let (mut w, mut l) = (0, 0);
                for &s in &seeds {
                    let (a, b) = (get(t, s, h).score, get(t, s, base).score);
                    if a > b {
                        w += 1;
                    } else if a < b {
                        l += 1;
                    }
                }
                tests.push((format!("{t} {h} vs {base}"), w, l, sign_p(w, w + l)));
            }
        }
    }
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
    println!("\ngate 1, paired by seed (higher / lower; sign test, Holm over {m}):");
    for (i, (name, w, l, p)) in tests.iter().enumerate() {
        println!("  {name:>28}: {w} / {l}, p {p:.4}{}", if holds[i] { ", holds" } else { "" });
    }
    // Gate 2: per tier, the best plan, and each other plan against it.
    println!("\ngate 2, per tier: the best plan by mean, and each plan against it (higher / lower, p):");
    for t in &tiers {
        let mean = |h: &str| seeds.iter().map(|&s| get(t, s, h).score).sum::<f32>() / seeds.len() as f32;
        let best = plans.iter().max_by(|a, b| mean(a).total_cmp(&mean(b))).unwrap();
        let line: Vec<String> = plans
            .iter()
            .filter(|h| h != &best)
            .map(|h| {
                let (mut w, mut l) = (0, 0);
                for &s in &seeds {
                    let (a, b) = (get(t, s, h).score, get(t, s, best).score);
                    if a > b {
                        w += 1;
                    } else if a < b {
                        l += 1;
                    }
                }
                format!("{h} {w}/{l} p {:.3}", sign_p(w, w + l))
            })
            .collect();
        println!("  {t:>8}: best {best} ({:+.3}); {}", mean(best), line.join(", "));
    }
}
