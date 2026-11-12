//! **The hole test** (owner, 2026-11-12 04:39Z, on the shot look: "A group
//! of soldiers shooting the slime should start carving out a bigger and
//! bigger hole that doesn't refill while being shot at").
//!
//! ```text
//! cargo run --release --example larderhole -- seeds=1-8 arms=base,flee:1,flee:3 [grow=60 fire=20 rest=10 squad=4 look=DIR every=5]
//! ```
//!
//! Each run is one Larder World game on the CPU race world
//! (`larder_world::race_world`, scale 2, 5,000 walkers, today's rules). The
//! mould grows for `grow` game seconds with the squad held at the larder
//! (no drain while it grows, so every run starts the test with the whole
//! squad). Then the squad of `squad` soldiers is set down in a line across
//! the ray from the mould's core toward the larder, `STAND` dish cells past
//! the body's edge on that ray (`place`), on Hold. It fires freely for
//! `fire` seconds and then holds fire for `rest`. The same seed is also run
//! with the squad holding fire throughout (the control): the mould moves
//! toward the soldiers' scent on its own, and the control shows how much.
//!
//! **Deciding measure, named before the run:** the hole after `fire`
//! seconds: the dish cells within the soldiers' range of the squad's
//! centre that held a living walker when the squad was set down and hold
//! none now. Each arm against `base` on paired seeds, two-sided sign test,
//! Holm over the arms. Beside it, per second: the mould's cells in range,
//! the walkers in range, how far the hits land from the squad (a hole that
//! deepens pushes the hits back), kills, and the soldiers alive.
//!
//! **Arms** (the mould's settings; the rules stay the game's):
//! - `base`: the race world's mould as it is (no alarm field; for a few
//!   hours on 2026-11-12 it fled gunfire by default, and
//!   `playtest/hole-1-16-default-sporer.txt` was run then).
//! - `off`: no alarm field.
//! - `flee:W[:K]`: the alarm field on (`alarm_keep` K, default 0.98, the
//!   Swarmer's), read by the sensors at weight -W, so a hit's alarm
//!   (`squad::SHOT_ALARM`, twice the strike's radius) is ground the mould
//!   turns away from until it fades. Fire keeps it fresh.
//! - `swarm`: the Swarmer tier (alarm weight +3), the opposite pull.
//! - `clock:N`: N fine steps a tick in place of the race world's 2 (a
//!   slower or faster mould, as the graphics thread timed it).
//! - `tier:NAME`: a mould tier (`mould::tiers`), after the parts before it:
//!   `flee:1+tier:sporer` is the Sporer on a mould that flees gunfire.
//! - `crater:S[:F[:K]]`: each hit lays S game seconds of crater on its
//!   strike's cells (`Rules::crater`, the squad toolkit's scorch field),
//!   biting at full strength for its last F fine steps
//!   (`BigParams::scorch_full`, default 30) and keeping K of the trail a
//!   fine step there (`scorch_keep`, default 0.1).
use slime_mould::mould::big::{BigDish, BigParams};
use slime_mould::mould::larder_world::{race_params, Cmd, Game, Order, Rules, Stance, World};
use slime_mould::mould::tiers::Tier;

/// Dish cells past the body's edge where the squad stands: about half a
/// soldier's range (15), so the first hits land at the body's rim and the
/// rest of the range reaches into it.
const STAND: f32 = 8.0;
/// Walkers within 3 dish cells that count as body on the ray.
const BODY: usize = 20;

/// One arm: its parts, joined by `+` and applied in order
/// (`flee:1+tier:sporer` is the Sporer on a mould that flees gunfire).
#[derive(Clone, Debug)]
struct Arm(Vec<Part>);

#[derive(Clone, Debug)]
enum Part {
    Base,
    Off,
    Flee(f32, f32),
    Swarm,
    Clock(u32),
    Tier(Tier),
    Crater(f32, Option<u16>, Option<f32>),
}

impl Arm {
    fn parse(s: &str) -> Arm {
        Arm(s
            .split('+')
            .map(|s| {
                let mut it = s.split(':');
                match it.next().unwrap() {
                    "base" => Part::Base,
                    "off" => Part::Off,
                    "flee" => {
                        let w = it.next().map_or(1.0, |v| v.parse().unwrap());
                        let k = it.next().map_or(0.98, |v| v.parse().unwrap());
                        Part::Flee(w, k)
                    }
                    "swarm" => Part::Swarm,
                    "clock" => Part::Clock(it.next().unwrap().parse().unwrap()),
                    "crater" => Part::Crater(it.next().unwrap().parse().unwrap(), it.next().map(|v| v.parse().unwrap()), it.next().map(|v| v.parse().unwrap())),
                    "tier" => {
                        let n = it.next().unwrap();
                        Part::Tier(Tier::from_name(n).unwrap_or_else(|| panic!("no tier {n}")))
                    }
                    o => panic!("no arm {o}"),
                }
            })
            .collect())
    }
    fn apply(&self, p: &mut BigParams) {
        for part in &self.0 {
            match *part {
                Part::Base => {}
                Part::Off => (p.alarm_keep, p.alarm_weight) = (0.0, 0.0),
                Part::Flee(w, k) => {
                    p.alarm_keep = k;
                    p.alarm_weight = -w;
                }
                Part::Swarm => Tier::from_name("swarmer").unwrap().apply(p),
                Part::Clock(n) => p.substeps = n as usize,
                Part::Tier(t) => t.apply(p),
                Part::Crater(_, f, k) => {
                    if let Some(f) = f {
                        p.scorch_full = f;
                    }
                    if let Some(k) = k {
                        p.scorch_keep = k;
                    }
                }
            }
        }
    }
    /// The game's rules for this arm.
    fn rules(&self, mut r: Rules) -> Rules {
        for part in &self.0 {
            if let Part::Crater(s, ..) = *part {
                r.crater = s;
            }
        }
        r
    }
    /// Only what a hit does changes (the alarm's pull, the crater):
    /// holding fire, it plays base's game.
    fn alarm_only(&self) -> bool {
        self.0.iter().all(|p| matches!(p, Part::Base | Part::Off | Part::Flee(..) | Part::Swarm | Part::Crater(..)))
    }
}

/// One second of the test.
#[derive(Clone, Copy, Default)]
struct Sec {
    hole: usize,
    cells: usize,
    walkers: usize,
    hit_d: f32,
    hits: u32,
    shots: u32,
    killed: u32,
    alive: usize,
}

struct Run {
    secs: Vec<Sec>,
    start_cells: usize,
    start_walkers: usize,
    placed: bool,
    ms_tick: f64,
}

fn main() {
    let mut seeds: Vec<u64> = (1..=8).collect();
    let (mut scale, mut walkers, mut threads) = (2usize, 5_000usize, 4usize);
    let (mut grow, mut fire, mut rest, mut squad) = (60.0f32, 20.0f32, 10.0f32, 4usize);
    let mut names: Vec<String> = vec!["base".into(), "flee:1".into(), "flee:3".into()];
    let mut arms: Vec<Arm> = names.iter().map(|s| Arm::parse(s)).collect();
    let mut look = None::<String>;
    // Seconds between pictures with `look`.
    let mut every = 5.0f32;
    let mut rules = Rules::default();
    for a in std::env::args().skip(1) {
        let (k, v) = a.split_once('=').expect("key=value");
        match k {
            "seeds" => {
                let (a, b) = v.split_once('-').unwrap_or((v, v));
                seeds = (a.parse().unwrap()..=b.parse().unwrap()).collect();
            }
            "scale" => scale = v.parse().unwrap(),
            "walkers" => walkers = v.parse().unwrap(),
            "threads" => threads = v.parse().unwrap(),
            "grow" => grow = v.parse().unwrap(),
            "fire" => fire = v.parse().unwrap(),
            "rest" => rest = v.parse().unwrap(),
            "squad" => squad = v.parse().unwrap(),
            "smell" => rules.smell = v.parse().unwrap(),
            "smell_r" => rules.smell_r = v.parse().unwrap(),
            "strike" => rules.strike = v.parse().unwrap(),
            "shot_every" => rules.shot_every = v.parse().unwrap(),
            "look" => look = Some(v.into()),
            "every" => every = v.parse().unwrap(),
            "arms" => {
                names = v.split(',').map(String::from).collect();
                arms = names.iter().map(|s| Arm::parse(s)).collect();
            }
            _ => panic!("unknown {k}"),
        }
    }
    println!("larderhole race world, scale {scale} walkers {walkers}, grow {grow} s, fire {fire} s, rest {rest} s, squad {squad}, stand {STAND} dish cells past the edge, seeds {seeds:?}; rules {rules:?}");
    if let Some(d) = &look {
        std::fs::create_dir_all(d).expect("look dir");
    }
    // Every (seed, arm, fire?) run on `threads` threads. Holding fire, an
    // alarm arm plays base's game to the bit (no shot, no alarm, and the
    // sensors add weight x 0; checked on seed 1), so an alarm arm's control
    // is base's, run once.
    let base = names.iter().position(|n| n == "base");
    let hold_of = |a: usize| match base {
        Some(b) if arms[a].alarm_only() => b,
        _ => a,
    };
    let jobs: Vec<(u64, usize, bool)> = seeds.iter().flat_map(|&s| (0..arms.len()).flat_map(move |a| [(s, a, true), (s, a, false)])).filter(|&(_, a, f)| f || hold_of(a) == a).collect();
    let next = std::sync::atomic::AtomicUsize::new(0);
    let out = std::sync::Mutex::new((0..jobs.len()).map(|_| None::<Run>).collect::<Vec<_>>());
    std::thread::scope(|sc| {
        for _ in 0..threads {
            sc.spawn(|| loop {
                let j = next.fetch_add(1, std::sync::atomic::Ordering::Relaxed);
                if j >= jobs.len() {
                    break;
                }
                let (seed, a, shoot) = jobs[j];
                let pics = look.as_deref().filter(|_| seed == seeds[0]).map(|d| (d, names[a].replace(':', "-")));
                let r = run(seed, scale, walkers, rules, &arms[a], shoot, grow, fire, rest, squad, pics.as_ref().map(|(d, n)| (*d, n.as_str(), every)));
                let s = |f: &dyn Fn(&Sec) -> String| r.secs.iter().map(f).collect::<Vec<_>>().join(" ");
                println!(
                    "{:>10} {} seed {seed:>2}: placed {}, at the start {} cells {} walkers in range; {:.2} ms a tick\n    hole    {}\n    cells   {}\n    walkers {}\n    hit at  {}\n    hits    {}\n    killed  {}\n    alive   {}",
                    names[a],
                    if shoot { "fire" } else { "hold" },
                    r.placed,
                    r.start_cells,
                    r.start_walkers,
                    r.ms_tick,
                    s(&|x| x.hole.to_string()),
                    s(&|x| x.cells.to_string()),
                    s(&|x| x.walkers.to_string()),
                    s(&|x| if x.hits > 0 { format!("{:.1}", x.hit_d) } else { "-".into() }),
                    s(&|x| format!("{}/{}", x.hits, x.shots)),
                    s(&|x| x.killed.to_string()),
                    s(&|x| x.alive.to_string()),
                );
                out.lock().unwrap()[j] = Some(r);
            });
        }
    });
    let out: Vec<Run> = out.into_inner().unwrap().into_iter().map(|r| r.unwrap()).collect();
    let squad = squad;
    let get = |seed: u64, a: usize, shoot: bool| {
        let a = if shoot { a } else { hold_of(a) };
        &out[jobs.iter().position(|&(s, b, f)| s == seed && b == a && f == shoot).unwrap()]
    };
    let marks: Vec<usize> = [5.0, 10.0, fire, fire + 2.0, fire + 5.0, fire + rest]
        .iter()
        .map(|&t| (t as usize).saturating_sub(1))
        .filter(|&i| i < (fire + rest) as usize)
        .collect();
    let at = (fire as usize).saturating_sub(1);
    let median = |mut v: Vec<f32>| {
        v.sort_by(f32::total_cmp);
        if v.is_empty() {
            f32::NAN
        } else {
            v[v.len() / 2]
        }
    };
    println!("\nmedian over seeds, hole in dish cells (fire / hold) at {:?} s after the squad is set down; firing stops at {fire} s:", marks.iter().map(|i| i + 1).collect::<Vec<_>>());
    for (a, n) in names.iter().enumerate() {
        let row = |shoot: bool| marks.iter().map(|&i| format!("{:>4.0}", median(seeds.iter().map(|&s| get(s, a, shoot).secs[i].hole as f32).collect()))).collect::<Vec<_>>().join(" ");
        let refill = median(seeds.iter().map(|&s| {
            let h = &get(s, a, true).secs;
            let end = h[at].hole.max(1) as f32;
            h[(at + 5).min(h.len() - 1)].hole as f32 / end
        }).collect());
        let kills = median(seeds.iter().map(|&s| get(s, a, true).secs[..=at].iter().map(|x| x.killed).sum::<u32>() as f32).collect());
        let alive = seeds.iter().map(|&s| get(s, a, true).secs.last().unwrap().alive).sum::<usize>();
        let ms = median(seeds.iter().map(|&s| get(s, a, true).ms_tick as f32).collect());
        let share = median(seeds.iter().map(|&s| {
            let r = get(s, a, true);
            r.secs[at].hole as f32 / r.start_cells.max(1) as f32
        }).collect());
        let deep = median(seeds.iter().map(|&s| {
            let h = &get(s, a, true).secs[at.saturating_sub(4)..=at];
            let n = h.iter().map(|x| x.hits).sum::<u32>();
            h.iter().map(|x| x.hit_d * x.hits as f32).sum::<f32>() / n.max(1) as f32
        }).collect());
        println!("  {n:>10}: fire {}  | hold {}  | at {fire:.0} s the hole is {:.0}% of the mould in range at the start; hits land {deep:.1} dish cells out in the last 5 s of fire; 5 s after the last shot {:.0}% of the hole is left; {kills:.0} killed under fire; {alive} of {} soldiers alive at the end; {ms:.2} ms a tick", row(true), row(false), share * 100.0, refill * 100.0, seeds.len() * squad);
    }
    // Each arm against base: the hole at the end of fire, paired by seed.
    let Some(b) = base else { return };
    let mut tests = Vec::new();
    for (a, n) in names.iter().enumerate().filter(|(a, _)| *a != b) {
        let (mut w, mut l) = (0, 0);
        let mut diffs = Vec::new();
        for &s in &seeds {
            let (x, y) = (get(s, a, true).secs[at].hole, get(s, b, true).secs[at].hole);
            diffs.push(x as f32 - y as f32);
            if x > y {
                w += 1;
            } else if x < y {
                l += 1;
            }
        }
        tests.push((n.clone(), w, l, sign_p(w, w + l), median(diffs)));
    }
    let m = tests.len();
    let mut order: Vec<usize> = (0..m).collect();
    order.sort_by(|&x, &y| tests[x].3.total_cmp(&tests[y].3));
    let mut holds = vec![false; m];
    for (rank, &i) in order.iter().enumerate() {
        if tests[i].3 * (m - rank) as f64 <= 0.05 {
            holds[i] = true;
        } else {
            break;
        }
    }
    println!("\nthe hole at {fire} s of fire against base, paired by seed (larger / smaller, ties left out; sign test, Holm over {m}):");
    for (i, (n, w, l, p, d)) in tests.iter().enumerate() {
        println!("  {n:>10}: {w} larger, {l} smaller, median difference {d:+.0} dish cells, p {p:.4}{}", if holds[i] { ", holds" } else { "" });
    }
}

/// The squad's line: `n` spots 2 dish cells apart across the ray from the
/// core toward the larder, `STAND` dish cells past the last point on the
/// ray with `BODY` walkers within 3 dish cells (the body's edge; the scan
/// stops after 6 dish cells with none). If a spot is off the open agar or
/// already in the mould, the ray turns by 20 degrees at a time, either
/// side. Returns the centre, the ray, and the spots (fine cells).
/// Where the squad stands: its centre, the ray (a unit vector) and the spots.
type Line = ((f32, f32), (f32, f32), Vec<(f32, f32)>);

fn place(g: &Game<BigDish>, n: usize) -> Option<Line> {
    let sc = g.world.scale();
    let d = &g.world;
    let live: Vec<(f32, f32)> = (0..d.walkers.len()).filter(|&k| d.lives(k)).map(|k| (d.walkers[k].0, d.walkers[k].1)).collect();
    let (c, l) = (g.core, g.larder);
    let base = (l.1 - c.1).atan2(l.0 - c.0);
    for turn in [0i32, 1, -1, 2, -2, 3, -3, 4, -4, 5, -5, 6, -6, 7, -7, 8, -8, 9] {
        let a = base + turn as f32 * 20f32.to_radians();
        let (ux, uy) = (a.cos(), a.sin());
        let (mut edge, mut gap, mut r) = (0.0f32, 0.0f32, 0.0f32);
        while r < 150.0 {
            let p = (c.0 + ux * r * sc, c.1 + uy * r * sc);
            let k = live.iter().filter(|q| (q.0 - p.0).hypot(q.1 - p.1) <= 3.0 * sc).count();
            if k >= BODY {
                edge = r;
                gap = 0.0;
            } else {
                gap += 0.5;
                if gap >= 6.0 {
                    break;
                }
            }
            r += 0.5;
        }
        let at = edge + STAND;
        let centre = (c.0 + ux * at * sc, c.1 + uy * at * sc);
        let spots: Vec<(f32, f32)> = (0..n)
            .map(|k| {
                let o = (k as f32 - (n as f32 - 1.0) / 2.0) * 2.0 * sc;
                (centre.0 - uy * o, centre.1 + ux * o)
            })
            .collect();
        let free = |p: &(f32, f32)| d.open(p.0, p.1) && !live.iter().any(|q| (q.0 - p.0).hypot(q.1 - p.1) <= 2.0 * sc);
        if spots.iter().all(free) {
            return Some((centre, (ux, uy), spots));
        }
    }
    None
}

#[allow(clippy::too_many_arguments)]
fn run(seed: u64, scale: usize, walkers: usize, rules: Rules, arm: &Arm, shoot: bool, grow: f32, fire: f32, rest: f32, n: usize, look: Option<(&str, &str, f32)>) -> Run {
    let mut p = race_params(seed, scale, walkers);
    arm.apply(&mut p);
    let rules = arm.rules(rules);
    let mut world = BigDish::new(p);
    world.threads = 1;
    let mut g = Game::new(world, rules, seed);
    // Grow: the squad holds fire at the larder and takes no drain.
    g.rules.drain = 0.0;
    for k in 0..g.soldiers.len() {
        g.apply(Cmd::Stance(k, Stance::HoldFire));
    }
    let tps = g.rules.tps;
    while g.time() < grow && g.end.is_none() {
        g.step();
    }
    g.rules.drain = rules.drain;
    let sc = g.world.scale();
    let placed = place(&g, n);
    let (centre, ray, spots) = placed.clone().unwrap_or((g.larder, (1.0, 0.0), vec![g.larder; n]));
    // The test's squad: `n` soldiers on the spots, the rest out of play
    // (buying more if the start had fewer).
    while g.soldiers.len() < n {
        g.bank = g.bank.max(g.rules.soldier_cost);
        if !g.apply(Cmd::Buy) {
            break;
        }
    }
    for (k, s) in g.soldiers.iter_mut().enumerate() {
        if k < n {
            s.x = spots[k].0;
            s.y = spots[k].1;
            s.alive = true;
            s.hp = rules.hp;
        } else {
            s.alive = false;
        }
        s.order = Order::Hold;
        s.stance = if shoot { Stance::Free } else { Stance::HoldFire };
    }
    let range = rules.range;
    // Dish cells within range of the centre, as offsets in a square grid.
    let side = 2 * range.ceil() as i32 + 1;
    let bins = |g: &Game<BigDish>| {
        let mut b = vec![0u32; (side * side) as usize];
        let d = &g.world;
        for k in 0..d.walkers.len() {
            if !d.lives(k) {
                continue;
            }
            let (dx, dy) = ((d.walkers[k].0 - centre.0) / sc, (d.walkers[k].1 - centre.1) / sc);
            if dx.hypot(dy) > range {
                continue;
            }
            let (i, j) = ((dx.floor() as i32 + side / 2), (dy.floor() as i32 + side / 2));
            if i >= 0 && j >= 0 && i < side && j < side {
                b[(j * side + i) as usize] += 1;
            }
        }
        b
    };
    let start = bins(&g);
    let start_cells = start.iter().filter(|&&v| v > 0).count();
    let start_walkers = start.iter().sum::<u32>() as usize;
    let t0 = g.time();
    let ticks0 = g.world.ticks();
    let clock = std::time::Instant::now();
    let mut secs = Vec::new();
    let (mut hit_sum, mut hits, mut shots, mut killed0) = (0.0f32, 0u32, 0u32, g.killed);
    let mut next_pic = 0.0f32;
    let total = fire + rest;
    let mut ceased = false;
    let mut tick = 0u32;
    while g.end.is_none() && (g.time() - t0) < total - 1e-3 {
        let el = g.time() - t0;
        if shoot && !ceased && el >= fire - 1e-3 {
            for k in 0..g.soldiers.len() {
                g.soldiers[k].stance = Stance::HoldFire;
            }
            ceased = true;
        }
        if let Some((dir, name, every)) = look {
            if el >= next_pic - 1e-3 {
                crop(&g, centre, ray, &format!("{dir}/{name}-{}-s{seed}-{:04.1}s.png", if shoot { "fire" } else { "hold" }, el));
                next_pic += every;
            }
        }
        g.step();
        tick += 1;
        for s in &g.last_shots {
            shots += 1;
            if let Some(h) = s.hit {
                hit_sum += (h.0 - centre.0).hypot(h.1 - centre.1) / sc;
                hits += 1;
            }
        }
        if tick.is_multiple_of(tps) {
            let b = bins(&g);
            secs.push(Sec {
                hole: start.iter().zip(&b).filter(|(a, b)| **a > 0 && **b == 0).count(),
                cells: b.iter().filter(|&&v| v > 0).count(),
                walkers: b.iter().sum::<u32>() as usize,
                hit_d: if hits > 0 { hit_sum / hits as f32 } else { 0.0 },
                hits,
                shots,
                killed: g.killed - killed0,
                alive: g.soldiers.iter().filter(|s| s.alive).count(),
            });
            (hit_sum, hits, shots, killed0) = (0.0, 0, 0, g.killed);
        }
    }
    if let Some((dir, name, _)) = look {
        crop(&g, centre, ray, &format!("{dir}/{name}-{}-s{seed}-{:02.0}s.png", if shoot { "fire" } else { "hold" }, g.time() - t0));
    }
    // A run cut short (the squad wiped, the mould gone) repeats its last
    // second, so every run has the same marks.
    let last = secs.last().copied().unwrap_or_default();
    while secs.len() < total as usize {
        secs.push(Sec { hits: 0, shots: 0, killed: 0, ..last });
    }
    let ms_tick = clock.elapsed().as_secs_f64() * 1000.0 / (g.world.ticks() - ticks0).max(1) as f64;
    Run { secs, start_cells, start_walkers, placed: placed.is_some(), ms_tick }
}

/// The 44 x 44 dish cells round the point `STAND` dish cells into the
/// mould from the squad, as the engine draws them, 3 pixels a fine cell;
/// soldiers white, the last frame's shots red to their hit.
fn crop(g: &Game<BigDish>, centre: (f32, f32), ray: (f32, f32), path: &str) {
    let sc = g.world.scale();
    let img = g.world.image(1);
    let half = 22.0 * sc;
    let mid = (centre.0 - ray.0 * STAND * sc, centre.1 - ray.1 * STAND * sc);
    let (x0, y0) = ((mid.0 - half).max(0.0) as u32, (mid.1 - half).max(0.0) as u32);
    let side = (2.0 * half) as u32;
    let (w, h) = (side.min(img.width() - x0), side.min(img.height() - y0));
    let mut out = image::imageops::crop_imm(&img, x0, y0, w, h).to_image();
    let mut put = |x: f32, y: f32, c: [u8; 4]| {
        let (px, py) = (x as i64 - x0 as i64, y as i64 - y0 as i64);
        if px >= 0 && py >= 0 && px < w as i64 && py < h as i64 {
            out.put_pixel(px as u32, py as u32, image::Rgba(c));
        }
    };
    for sh in &g.last_shots {
        let end = sh.hit.unwrap_or(sh.to);
        let n = (end.0 - sh.from.0).hypot(end.1 - sh.from.1).ceil().max(1.0) as i32;
        for k in 0..=n {
            let t = k as f32 / n as f32;
            put(sh.from.0 + (end.0 - sh.from.0) * t, sh.from.1 + (end.1 - sh.from.1) * t, [255, 70, 60, 255]);
        }
    }
    for s in g.soldiers.iter().filter(|s| s.alive) {
        for dy in -1..=1 {
            for dx in -1..=1 {
                put(s.x + dx as f32, s.y + dy as f32, [255, 255, 255, 255]);
            }
        }
    }
    let big = image::imageops::resize(&out, w * 3, h * 3, image::imageops::FilterType::Nearest);
    big.save(path).expect("save png");
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
