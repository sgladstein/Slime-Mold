//! **Larder M2: the mould answers, and spending** (build plan,
//! `Reports/larder-build-plan-2026-11-11.md`, revised).
//!
//! ```text
//! cargo run --release --example larderm2 -- census seeds=17-32 [off=hoard|hunger|drops]
//! cargo run --release --example larderm2 -- cost
//! cargo run --release --example larderm2 -- look=DIR seeds=1 [plan=reader] [lean=1]
//! ```
//!
//! M1's game (`slime_mould::mould::larder`) with M2's spending (buy a
//! soldier, lay bait, build a turret) and the mould's three answers, on by
//! default: the larder smells of the bank (`Rules::hoard`), a starving
//! mould drains and hunts soldiers harder (`Rules::hunger`), and timed
//! drops land every 60 game seconds, smelling 10 s ahead, none after 10:00
//! (`Rules::drop_every`).
//!
//! **The question** (method review, finding 1: in a solo game, plans
//! counter each other when the best plan depends on a state the player can
//! see): does the best plan cross over between a **fed** field (every pile
//! full: a big, meshed mould) and a **lean** one (every pile but the
//! larder at a quarter: a small, tree-shaped mould, 16 of 16 in the first
//! tests)? And does a **reader**, which picks from what it sees, beat
//! every fixed plan in both?
//!
//! **Plans** (fixed rules, settings named here; first looked at on seeds
//! 1-4, gated on 17-32):
//! - **idle**, and **random** spend-matched: fires at random points in
//!   range at the rate that spends what the median working plan spent on
//!   that seed (capped at one shot a soldier a second).
//! - **fortress**: M1's ring of three burn lines and one farmer on the
//!   larder; surplus over `RESERVE` builds turrets on a ring `TURRET_R` round
//!   the larder, up to `TURRETS`.
//! - **farmer**: soldier k clears and harvests the k-th nearest pile to the
//!   larder; surplus over `RESERVE` buys a soldier for the next pile, up to
//!   `FARM_MAX` piles.
//! - **hunter**: spends the starting bank on soldiers at once, then the
//!   whole squad clears the mould-eaten pile nearest the larder (the core
//!   last), free fire, harvesting what it clears; surplus buys soldiers.
//! - **baiter**: farms like the farmer with three piles; whenever mould is
//!   within `BAIT_NEAR` of the larder and no bait is uneaten, lays
//!   `BAIT_FOOD` of bait `BAIT_AWAY` cells from the larder toward the mould.
//! - **reader**: farms and buys while the mould is big; hunts once the
//!   mould is small (fewer than `SMALL` walkers, read from `READ_AFTER`
//!   game seconds on) or the squad has `ARMY`
//!   soldiers. What it reads is on screen: the mould's size and its own
//!   squad.
use slime_mould::mould::larder::{field_game, Game, Order, Rules, Stance, GAME_SECOND};
use slime_mould::mould::skirmish::HOMES;
use slime_mould::mould::{H, W};

const FORT_R: f32 = 10.0;
const TURRET_R: f32 = 16.0;
const TURRETS: usize = 6;
const RESERVE: f32 = 600.0;
const FARM_MAX: usize = 8;
const HUNT_EVERY: u64 = 5;
const BAIT_NEAR: f32 = 50.0;
const BAIT_AWAY: f32 = 35.0;
const BAIT_FOOD: f32 = 300.0;
const SMALL: usize = 3000;
const ARMY: usize = 16;
const READ_AFTER: u64 = 120;
const LEAN: f32 = 0.25;

#[derive(Clone, Copy, Debug, PartialEq)]
enum Plan {
    Idle,
    Random(f32),
    Fortress,
    Farmer,
    Hunter,
    Baiter,
    Reader,
}

const WORKING: [Plan; 5] = [Plan::Fortress, Plan::Farmer, Plan::Hunter, Plan::Baiter, Plan::Reader];

impl Plan {
    fn name(self) -> &'static str {
        match self {
            Plan::Idle => "idle",
            Plan::Random(_) => "random",
            Plan::Fortress => "fortress",
            Plan::Farmer => "farmer",
            Plan::Hunter => "hunter",
            Plan::Baiter => "baiter",
            Plan::Reader => "reader",
        }
    }
    fn parse(s: &str) -> Plan {
        [Plan::Idle, Plan::Random(0.5), Plan::Fortress, Plan::Farmer, Plan::Hunter, Plan::Baiter, Plan::Reader]
            .into_iter()
            .find(|p| p.name() == s)
            .expect("plan")
    }
}

#[derive(Clone, Copy, Debug, Default)]
struct Out {
    score: f32,
    band: &'static str,
    end_t: f32,
    left: usize,
    most: usize,
    shots: u32,
    spent: f32,
    harvested: f32,
    bought: u32,
    turrets: usize,
    baited: f32,
    core_down: Option<f32>,
    walkers_at_read: usize,
    walkers: usize,
    hunger_mean: f32,
}

fn arg<'a>(args: &'a [String], k: &str) -> Option<&'a str> {
    args.iter().find_map(|a| a.strip_prefix(k).and_then(|r| r.strip_prefix('=')))
}

fn seeds(s: Option<&str>) -> Vec<u64> {
    let s = s.unwrap_or("1-16");
    if let Some((a, b)) = s.split_once('-') {
        (a.parse().unwrap()..=b.parse().unwrap()).collect()
    } else {
        s.split(',').map(|v| v.parse().unwrap()).collect()
    }
}

fn game(seed: u64, rules: Rules, lean: bool) -> Game {
    let mut g = field_game(seed, rules);
    if lean {
        let li = g.pile_at(g.larder).expect("larder");
        for (k, p) in g.dish.piles.iter_mut().enumerate() {
            if k != li {
                p.food *= LEAN;
                p.full *= LEAN;
            }
        }
    }
    g
}

/// Piles by distance from the larder, nearest first (the larder itself
/// first), leaving out the mould's core.
fn by_distance(g: &Game) -> Vec<(f32, f32)> {
    let mut by: Vec<(f32, (f32, f32))> = g
        .dish
        .piles
        .iter()
        .filter(|p| (p.x - g.core.0).hypot(p.y - g.core.1) > 1.0 && p.food > 0.0)
        .map(|p| ((p.x - g.larder.0).hypot(p.y - g.larder.1), (p.x, p.y)))
        .collect();
    by.sort_by(|a, b| a.0.total_cmp(&b.0));
    by.into_iter().map(|b| b.1).collect()
}

fn alive(g: &Game) -> Vec<usize> {
    (0..g.soldiers.len()).filter(|&k| g.soldiers[k].alive).collect()
}

fn farm_step(g: &mut Game, max_piles: usize, buy: bool) {
    let piles = by_distance(g);
    let squad = alive(g);
    if buy && g.bank > RESERVE + g.rules.soldier_cost && squad.len() < max_piles {
        g.buy();
    }
    let squad = alive(g);
    for (j, &k) in squad.iter().enumerate() {
        let want = piles.get(j % max_piles.min(piles.len()).max(1)).copied();
        if let Some(p) = want {
            let busy = matches!(g.soldiers[k].order, Order::Clear(x, y) if g.pile_at((x, y)).is_some_and(|i| g.dish.piles[i].food > 0.0));
            if !busy {
                g.order(k, Order::Clear(p.0, p.1));
            }
        }
    }
}

fn hunt_step(g: &mut Game) {
    while g.bank > RESERVE + g.rules.soldier_cost && g.buy() {}
    let squad = alive(g);
    if squad.is_empty() {
        return;
    }
    for &k in &squad {
        g.stance(k, Stance::Free);
    }
    // Keep the pile being cleared while it has food.
    let keep = squad.iter().find_map(|&k| match g.soldiers[k].order {
        Order::Clear(x, y) => g.pile_at((x, y)).filter(|&i| g.dish.piles[i].food > 0.0 && g.dish.piles[i].eaters > 0).map(|_| (x, y)),
        _ => None,
    });
    let target = keep.or_else(|| {
        g.dish
            .piles
            .iter()
            .filter(|p| p.eaters > 0 && p.food > 0.0 && (p.x - g.core.0).hypot(p.y - g.core.1) > 1.0)
            .min_by(|a, b| {
                (a.x - g.larder.0)
                    .hypot(a.y - g.larder.1)
                    .total_cmp(&(b.x - g.larder.0).hypot(b.y - g.larder.1))
            })
            .map(|p| (p.x, p.y))
    });
    for &k in &squad {
        match target {
            Some(p) => {
                if g.soldiers[k].order != Order::Clear(p.0, p.1) {
                    g.order(k, Order::Clear(p.0, p.1));
                }
            }
            // Nothing eaten but the core: walk on it only once the mould
            // is small; otherwise hold at the larder. (As in M1, a rule of
            // "no target, attack the core" marched the squad across the
            // field at 0 s, when the core is the only pile eaten, and it
            // died at 34-38 s on seeds 1-4.)
            None if g.dish.walkers.len() < SMALL => {
                if g.soldiers[k].order != Order::Attack {
                    g.order(k, Order::Attack);
                }
            }
            None => {
                if g.soldiers[k].order != Order::Clear(g.larder.0, g.larder.1) {
                    g.order(k, Order::Clear(g.larder.0, g.larder.1));
                }
            }
        }
    }
}

fn setup(g: &mut Game, plan: Plan) {
    let n = g.soldiers.len();
    match plan {
        Plan::Random(_) => (0..n).for_each(|k| g.stance(k, Stance::HoldFire)),
        Plan::Fortress => {
            let ring = (n - 1).clamp(3, n);
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
        _ => {}
    }
}

fn each_second(g: &mut Game, plan: Plan) {
    let sec = g.dish.t / GAME_SECOND;
    match plan {
        Plan::Idle => {}
        Plan::Random(rate) => {
            for k in alive(g) {
                if g.rng.f32() < rate {
                    let s = g.soldiers[k];
                    let a = g.rng.f32() * std::f32::consts::TAU;
                    let r = g.rng.f32() * g.rules.range;
                    g.shoot(k, (s.x + r * a.cos(), s.y + r * a.sin()));
                }
            }
        }
        Plan::Fortress => {
            if g.bank > RESERVE + g.rules.turret_cost && g.turrets.len() < TURRETS {
                let k = g.turrets.len() as f32;
                let a = k / TURRETS as f32 * std::f32::consts::TAU + 0.3;
                let at = (g.larder.0 + TURRET_R * a.cos(), g.larder.1 + TURRET_R * a.sin());
                g.build_turret(at);
            }
        }
        Plan::Farmer => farm_step(g, FARM_MAX, true),
        Plan::Hunter => {
            if sec.is_multiple_of(HUNT_EVERY) {
                hunt_step(g);
            }
        }
        Plan::Baiter => {
            farm_step(g, 3, false);
            let near = g
                .dish
                .walkers
                .iter()
                .filter(|w| (w.x - g.larder.0).hypot(w.y - g.larder.1) < BAIT_NEAR)
                .fold((0.0, 0.0, 0usize), |a, w| (a.0 + w.x, a.1 + w.y, a.2 + 1));
            let uneaten = g.dish.piles.iter().skip(23).any(|p| p.food > 0.0 && p.eaters == 0 && p.full == BAIT_FOOD);
            if near.2 > 20 && !uneaten {
                let c = (near.0 / near.2 as f32, near.1 / near.2 as f32);
                let (dx, dy) = (c.0 - g.larder.0, c.1 - g.larder.1);
                let d = dx.hypot(dy).max(0.1);
                // Toward the mould, at the edge of a soldier's range from
                // the nearest soldier to that point.
                let at = (g.larder.0 + dx / d * BAIT_AWAY, g.larder.1 + dy / d * BAIT_AWAY);
                if let Some(&k) = alive(g).first() {
                    let s = g.soldiers[k];
                    let (ex, ey) = (at.0 - s.x, at.1 - s.y);
                    let e = ex.hypot(ey).max(0.1);
                    let reach = e.min(g.rules.range - 1.0);
                    g.bait((s.x + ex / e * reach, s.y + ey / e * reach), BAIT_FOOD);
                }
            }
        }
        Plan::Reader => {
            // Until it has seen the mould grow (`READ_AFTER`), it baits and
            // farms three piles, as the baiter does. Then a small mould
            // (fewer than `SMALL` walkers) is a lean one: expand and buy as
            // the farmer does, and hunt once the squad is `ARMY` strong.
            // A big mould: keep baiting.
            let army = alive(g).len();
            let small = sec >= READ_AFTER && g.dish.walkers.len() < SMALL;
            if small && army >= ARMY {
                if sec.is_multiple_of(HUNT_EVERY) {
                    hunt_step(g);
                }
            } else if small {
                // Tried and reverted (seeds 1-16): buying past one soldier
                // a pile, toward `ARMY`, to spend the fed reader's unspent
                // ~12000 food. Fed did not move (median -0.214, 13 of 16
                // over the farmer either way) and lean collapsed (median
                // 1.431 -> -1.651, squad wiped 16 of 16): extra soldiers
                // walk into a starving mould and feed it.
                farm_step(g, FARM_MAX, true);
            } else {
                each_second(g, Plan::Baiter);
            }
        }
    }
}

fn run(seed: u64, plan: Plan, rules: Rules, lean: bool, look: Option<&str>) -> Out {
    let mut g = game(seed, rules, lean);
    setup(&mut g, plan);
    let mut o = Out::default();
    let mut hunger_sum = 0.0;
    let mut secs = 0u32;
    while g.end.is_none() {
        if g.dish.t.is_multiple_of(GAME_SECOND) {
            each_second(&mut g, plan);
            o.most = o.most.max(g.alive());
            hunger_sum += g.hunger;
            secs += 1;
            if g.dish.t == READ_AFTER * GAME_SECOND {
                o.walkers_at_read = g.dish.walkers.len();
            }
            if let Some(dir) = look {
                if g.dish.t.is_multiple_of(60 * GAME_SECOND) {
                    save(&g, &format!("{dir}/m2-s{seed}-{}-{}-{:03}.png", if lean { "lean" } else { "fed" }, plan.name(), g.dish.t / GAME_SECOND));
                }
            }
        }
        g.step();
    }
    if let Some(dir) = look {
        save(&g, &format!("{dir}/m2-s{seed}-{}-{}-end.png", if lean { "lean" } else { "fed" }, plan.name()));
    }
    let end = g.end.unwrap();
    o.score = g.score().unwrap();
    o.band = end.band();
    o.end_t = g.time();
    o.left = g.alive();
    o.shots = g.shots;
    o.spent = g.spent;
    o.harvested = g.harvested;
    o.bought = g.bought;
    o.turrets = g.turrets.len();
    o.baited = g.baited;
    o.core_down = g.core_down.map(|t| t as f32 / GAME_SECOND as f32);
    o.walkers = g.dish.walkers.len();
    o.hunger_mean = hunger_sum / secs.max(1) as f32;
    o
}

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
    let mut stamp = |x: f32, y: f32, r: i32, col: [u8; 4]| {
        let (cx, cy) = ((x * scale as f32) as i32, (y * scale as f32) as i32);
        for dy in -r..=r {
            for dx in -r..=r {
                let (px, py) = (cx + dx, cy + dy);
                if px >= 0 && py >= 0 && (px as u32) < w && (py as u32) < h {
                    let i = ((py as u32 * w + px as u32) * 4) as usize;
                    buf[i..i + 4].copy_from_slice(&col);
                }
            }
        }
    };
    for t in &g.turrets {
        stamp(t.x, t.y, 3, [90, 170, 255, 255]);
    }
    for s in &g.soldiers {
        stamp(s.x, s.y, 2, if s.alive { [255, 255, 255, 255] } else { [140, 20, 20, 255] });
    }
    image::RgbaImage::from_raw(w, h, buf).expect("frame").save(path).expect("save png");
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

/// Wins of `a` over `b`, ties dropped: (wins, n).
fn wins(a: &[Out], b: &[Out]) -> (usize, usize) {
    let (mut w, mut n) = (0, 0);
    for (x, y) in a.iter().zip(b) {
        if (x.score - y.score).abs() < 1e-4 {
            continue;
        }
        n += 1;
        if x.score > y.score {
            w += 1;
        }
    }
    (w, n)
}

type Results = Vec<(Plan, Vec<Out>)>;

fn condition(seeds: &[u64], rules: Rules, lean: bool) -> Results {
    let mut results: Results = [Plan::Idle].iter().chain(WORKING.iter()).map(|&p| (p, par(seeds, |s| run(s, p, rules, lean, None)))).collect();
    let rates: Vec<f32> = (0..seeds.len())
        .map(|i| {
            let per: Vec<f32> = results[1..]
                .iter()
                .map(|(_, o)| o[i].spent / o[i].end_t.max(1.0) / rules.price / rules.squad as f32)
                .collect();
            median(per).min(1.0)
        })
        .collect();
    let random = par(seeds, |s| {
        let i = seeds.iter().position(|&x| x == s).unwrap();
        run(s, Plan::Random(rates[i]), rules, lean, None)
    });
    results.insert(1, (Plan::Random(0.0), random));
    results
}

fn print(name: &str, seeds: &[u64], results: &Results) {
    println!("\n==== {name}");
    for (plan, o) in results {
        println!("{}:", plan.name());
        for (s, r) in seeds.iter().zip(o) {
            println!(
                "  seed {s:2}: score {:6.3}  {:11}  at {:4.0}  left {:2} most {:2}  shots {:5}  spent {:6.0}  harvested {:6.0}  bought {:2}  turrets {}  bait {:5.0}  core {:>4}  walkers@read {:5}  end walkers {:5}  hunger {:.2}",
                r.score,
                r.band,
                r.end_t,
                r.left,
                r.most,
                r.shots,
                r.spent,
                r.harvested,
                r.bought,
                r.turrets,
                r.baited,
                r.core_down.map(|t| format!("{t:.0}")).unwrap_or("-".into()),
                r.walkers_at_read,
                r.walkers,
                r.hunger_mean
            );
        }
        let bands = ["mould wiped", "cap", "squad wiped"]
            .iter()
            .map(|b| format!("{b} {}", o.iter().filter(|r| r.band == *b).count()))
            .collect::<Vec<_>>()
            .join(", ");
        println!(
            "  median score {:.3}; {bands}; median end {:.0} s",
            median(o.iter().map(|r| r.score).collect()),
            median(o.iter().map(|r| r.end_t).collect())
        );
    }
    // Best plan per seed among the working plans.
    print!("  best working plan per seed:");
    for i in 0..seeds.len() {
        let best = results[2..]
            .iter()
            .max_by(|a, b| a.1[i].score.total_cmp(&b.1[i].score))
            .map(|(p, _)| p.name())
            .unwrap();
        print!(" {best}");
    }
    println!();
}

fn holm(tests: &mut [(String, usize, usize, f64, bool)]) {
    let mut order: Vec<usize> = (0..tests.len()).collect();
    order.sort_by(|&a, &b| tests[a].3.total_cmp(&tests[b].3));
    let m = tests.len();
    let mut holding = true;
    for (rank, &i) in order.iter().enumerate() {
        holding = holding && tests[i].3 <= 0.05 / (m - rank) as f64;
        tests[i].4 = holding;
    }
}

fn census(seeds: &[u64], rules: Rules, label: &str) {
    println!(
        "Larder M2 census{label}: {} seeds ({}-{}), squad {} growing to {}, cap {} game s; hoard {}, hunger {}, drops every {} s",
        seeds.len(),
        seeds[0],
        seeds[seeds.len() - 1],
        rules.squad,
        rules.max_squad,
        rules.cap,
        rules.hoard,
        rules.hunger,
        rules.drop_every
    );
    let fed = condition(seeds, rules, false);
    print("FED field (every pile full)", seeds, &fed);
    let lean = condition(seeds, rules, true);
    print("LEAN field (every pile but the larder at a quarter)", seeds, &lean);

    // The family, named before the run: (1) every ordered pair of the four
    // fixed working plans in each condition, for the crossover; (2) the
    // reader against each fixed plan and the two controls in each
    // condition. Holm over all of them.
    let fixed = [Plan::Fortress, Plan::Farmer, Plan::Hunter, Plan::Baiter];
    let get = |r: &Results, p: Plan| r.iter().find(|(q, _)| q.name() == p.name()).map(|x| x.1.clone()).unwrap();
    let mut tests: Vec<(String, usize, usize, f64, bool)> = Vec::new();
    for (cname, r) in [("fed", &fed), ("lean", &lean)] {
        for (i, &a) in fixed.iter().enumerate() {
            for &b in &fixed[i + 1..] {
                let (w, n) = wins(&get(r, a), &get(r, b));
                // Two-sided: report as the winner's count.
                let (name, ww) = if 2 * w >= n { (format!("{cname}: {} > {}", a.name(), b.name()), w) } else { (format!("{cname}: {} > {}", b.name(), a.name()), n - w) };
                tests.push((name, ww, n, sign_p(w, n), false));
            }
        }
        for &b in [Plan::Idle, Plan::Random(0.0)].iter().chain(fixed.iter()) {
            let (w, n) = wins(&get(r, Plan::Reader), &get(r, b));
            tests.push((format!("{cname}: reader > {}", b.name()), w, n, sign_p(w, n), false));
        }
    }
    holm(&mut tests);
    println!("\nComparisons (paired by seed, ties dropped; Holm over {}):", tests.len());
    for (name, w, n, p, ok) in &tests {
        println!("  {name:32} {w:2} of {n:2}  p {p:.5}  {}", if *ok { "survives Holm" } else { "-" });
    }
    // The crossover: a pair whose winner flips between the conditions,
    // each way on 15+ seeds and surviving Holm.
    println!("\nCrossovers (A > B fed and B > A lean, or the reverse, 15+ each way, both survive Holm):");
    let mut any = false;
    for (i, &a) in fixed.iter().enumerate() {
        for &b in &fixed[i + 1..] {
            let f = tests.iter().find(|t| t.0.starts_with("fed: ") && (t.0.ends_with(&format!("{} > {}", a.name(), b.name())) || t.0.ends_with(&format!("{} > {}", b.name(), a.name()))));
            let l = tests.iter().find(|t| t.0.starts_with("lean: ") && (t.0.ends_with(&format!("{} > {}", a.name(), b.name())) || t.0.ends_with(&format!("{} > {}", b.name(), a.name()))));
            if let (Some(f), Some(l)) = (f, l) {
                let flipped = f.0[5..] != l.0[6..];
                if flipped {
                    let pass = f.1 >= 15 && l.1 >= 15 && f.4 && l.4;
                    any = true;
                    println!("  {} ({} of {}) / {} ({} of {}): {}", f.0, f.1, f.2, l.0, l.1, l.2, if pass { "PASS" } else { "flips, not significant" });
                }
            }
        }
    }
    if !any {
        println!("  none: no pair of fixed plans changes winner between fed and lean");
    }
    let reader_ok = tests.iter().filter(|t| t.0.contains("reader >")).all(|t| t.1 >= 15 && t.4);
    println!("Reader beats every fixed plan and both controls in both conditions (15+ of 16, Holm): {}", if reader_ok { "PASS" } else { "FAIL" });
}

/// The game layer's cost a step with every M2 answer on (hoard smell,
/// hunger, drops) and six turrets firing: the whole `Game::step` against a
/// bare dish step on the same seed (the M1 method, `larderm1 cost`).
fn cost(rules: Rules) {
    for n in [4usize, 40] {
        let rules = Rules { squad: n, start_bank: 1e9, ..rules };
        let steps = 120 * GAME_SECOND;
        let mut g = game(1, rules, false);
        (0..n).for_each(|k| g.stance(k, Stance::Free));
        for k in 0..TURRETS {
            let a = k as f32 / TURRETS as f32 * std::f32::consts::TAU;
            let s = g.soldiers[0];
            g.build_turret((s.x + 8.0 * a.cos(), s.y + 8.0 * a.sin()));
        }
        let t0 = std::time::Instant::now();
        for _ in 0..steps {
            g.step();
        }
        let game_ns = t0.elapsed().as_nanos();
        let mut bare = game(1, Rules { squad: 0, ..rules }, false);
        let t0 = std::time::Instant::now();
        for _ in 0..steps {
            bare.dish.step();
        }
        let dish_ns = t0.elapsed().as_nanos();
        let per = |ns: u128| ns as f64 / steps as f64 / 1e6;
        println!(
            "{n:2} soldiers, {} turrets: game step {:.3} ms, bare dish step {:.3} ms, game layer about {:.3} ms a step",
            g.turrets.len(),
            per(game_ns),
            per(dish_ns),
            per(game_ns) - per(dish_ns)
        );
    }
}

fn main() {
    let args: Vec<String> = std::env::args().skip(1).collect();
    let seeds = seeds(arg(&args, "seeds"));
    let mut rules = Rules::default();
    let mut label = String::new();
    if let Some(off) = arg(&args, "off") {
        for o in off.split(',') {
            match o {
                "hoard" => rules.hoard = 0.0,
                "hunger" => rules.hunger = 0.0,
                "drops" => rules.drop_every = 0.0,
                _ => panic!("off=hoard|hunger|drops"),
            }
        }
        label = format!(" (off: {off})");
    }
    let t0 = std::time::Instant::now();
    if let Some(dir) = arg(&args, "look") {
        std::fs::create_dir_all(dir).expect("look dir");
        let lean = arg(&args, "lean") == Some("1");
        let plans: Vec<Plan> = match arg(&args, "plan") {
            Some(p) => vec![Plan::parse(p)],
            None => WORKING.to_vec(),
        };
        for &seed in &seeds {
            for &p in &plans {
                let o = run(seed, p, rules, lean, Some(dir));
                println!("seed {seed} {} {}: {o:?}", if lean { "lean" } else { "fed" }, p.name());
            }
        }
    } else if args.first().map(String::as_str) == Some("cost") {
        cost(rules);
    } else if args.first().map(String::as_str) == Some("census") {
        census(&seeds, rules, &label);
    } else {
        panic!("census | cost | look=DIR");
    }
    let _ = (HOMES, W, H);
    println!("({:.1} s)", t0.elapsed().as_secs_f32());
}
