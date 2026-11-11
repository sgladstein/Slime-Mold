//! **Larder's three cheap tests** (`Reports/mould-shooter-2026-11-11.md`,
//! *What must be true, and the first tests*): the mould as the enemy of a
//! squad that shoots it, measured on today's dish before anything is built.
//!
//! ```text
//! cargo run --release --example larderprobe -- fire seeds=1-16 [gun=snip|clear] [food=1]
//! cargo run --release --example larderprobe -- smell seeds=1-16
//! cargo run --release --example larderprobe -- lean seeds=1-16
//! cargo run --release --example larderprobe -- look=DIR seeds=1 [hand=aim] [gain=G] [food=F]
//! ```
//!
//! The dish is the field skirmish's 23-pile field (`skirmish::FIELD`) with
//! one mould only, started on the left home. The other home, (216, 80), is
//! **the larder**: the squad's food, which the mould steals by eating it.
//! No clock, no storm; each run is `SECONDS` long.
//!
//! `fire` (test 1): does fire near the larder protect it? Hands, each
//! firing one 24-cell line every `EVERY` steps (four a second) while any
//! mould is within `NEAR` cells of the larder: **aim** across the thickest
//! tube crossing a guard ring `RING` cells from the larder's centre (a
//! supply line in), **near** at a random spot on the same ring, along it,
//! **rand** anywhere on the dish, **pile** straight through the larder at a
//! random angle (at the mould eating it), and **idle**. `gun=snip` is the
//! existing snip (kills walkers, clears trail, wounds the ground 4 s, but
//! never a pile's own cells, so on the larder it only kills), so
//! fire every quarter second holds a cut; `gun=clear` only zeroes trail on
//! the line, every step, until the next shot. Deciding measure: food stolen
//! from the larder by the end, paired by seed.
//!
//! First look (seed 1): the mould covers the field and bites the larder by
//! about 4 s, and a closed ring leaves a self-fed island on the larder, so
//! `pile` was added after looking, before the census.
//!
//! `smell` (test 2): the larder gives off scent like a beacon of gain
//! 0.3, 1 and 3 times a full pile's (`Dish::beacons`), as a larder holding
//! little, some or a lot would; idle hand. Deciding measure: seconds to the
//! mould's first bite of the larder.
//!
//! `lean` (test 3): every pile but the larder holds `LEAN` of its food
//! against full. Counts loops (holes in the tube) and walkers at 45 s, then
//! the aim-versus-idle stolen difference at each, paired by seed.
use slime_mould::mould::skirmish::FIELD;
use slime_mould::mould::{Dish, Ground, Tool, H, STEPS_PER_SECOND, W};

const SECONDS: u64 = 90;
/// Steps between shots: four a second.
const EVERY: u64 = SPS / 4;
const NEAR: f32 = 50.0;
/// The guard ring: cells from the larder's centre where shots land.
const RING: f32 = 14.0;
const RING_POINTS: usize = 96;
/// The fortress ring as arcs: five 24-cell lines, one along each side of
/// a pentagon round the ring of 14 (half a side is 10.2, inside the line's
/// 12), close it.
const FORT_ARCS: u64 = 5;
/// The rival's home on the field (moved half a cell in on 11-11; it was
/// (216, 80) for the first three tests).
const LARDER: (f32, f32) = slime_mould::mould::skirmish::HOMES[1];
const LINE: f32 = 24.0;
const LEAN: f32 = 0.25;
/// When loops are counted: before the field's food runs low.
const LOOPS_AT: u64 = 45;
const SPS: u64 = STEPS_PER_SECOND as u64;

fn arg<'a>(args: &'a [String], k: &str) -> Option<&'a str> {
    args.iter()
        .find_map(|a| a.strip_prefix(k).and_then(|r| r.strip_prefix('=')))
}

fn seeds(s: Option<&str>) -> Vec<u64> {
    let s = s.unwrap_or("1-16");
    match s.split_once('-') {
        Some((a, b)) => (a.parse().unwrap()..=b.parse().unwrap()).collect(),
        None => s.split(',').map(|v| v.parse().unwrap()).collect(),
    }
}

#[derive(Clone, Copy, PartialEq, Debug)]
enum Hand {
    Idle,
    Aim,
    Near,
    Rand,
    /// Fire through the larder itself, at a random angle: at the mould
    /// eating it.
    Pile,
    /// A fortress: the whole guard ring held from the first second, one of
    /// `FORT_ARCS` arcs a shot in turn, whether or not mould is near.
    Fort,
}

impl Hand {
    fn parse(s: &str) -> Hand {
        match s {
            "aim" => Hand::Aim,
            "near" => Hand::Near,
            "rand" => Hand::Rand,
            "pile" => Hand::Pile,
            "fort" => Hand::Fort,
            _ => Hand::Idle,
        }
    }
}

#[derive(Clone, Copy, PartialEq)]
enum Gun {
    Snip,
    Clear,
}

#[derive(Clone, Copy)]
struct Setup {
    hand: Hand,
    gun: Gun,
    /// Beacon gain on the larder, or 0 for a plain pile.
    gain: f32,
    /// Food in every pile but the larder, as a fraction of the field's.
    food: f32,
    /// Steps between shots.
    every: u64,
}

#[derive(Clone, Copy, Debug, Default)]
struct Out {
    stolen: f32,
    ate: f32,
    walkers: usize,
    first_bite: Option<f32>,
    shots: u32,
    loops45: usize,
    walkers45: usize,
}

fn larder_index(d: &Dish) -> usize {
    d.piles
        .iter()
        .position(|p| (p.x - LARDER.0).abs() < 0.5 && (p.y - LARDER.1).abs() < 0.5)
        .expect("the larder pile")
}

fn dish(seed: u64, s: Setup) -> Dish {
    let mut d = Dish::from_level_with(&FIELD, seed, FIELD.params_for());
    // One mould: the level's rival goes, and with it the level's bell, so
    // the dish is a free one that never ends or storms.
    d.rival = None;
    d.level = None;
    d.storm_at = u64::MAX;
    d.snips_left = u32::MAX;
    let li = larder_index(&d);
    for (k, p) in d.piles.iter_mut().enumerate() {
        if k != li {
            p.food *= s.food;
            p.full *= s.food;
        }
    }
    if s.gain > 0.0 {
        d.beacons.push((LARDER.0, LARDER.1, s.gain));
    }
    d
}

struct Rng(u64);
impl Rng {
    fn f(&mut self) -> f32 {
        self.0 ^= self.0 << 13;
        self.0 ^= self.0 >> 7;
        self.0 ^= self.0 << 17;
        (self.0 >> 40) as f32 / (1u64 << 24) as f32
    }
}

/// Inside the dish: agar, or ground a shot has wounded (a blind hand fires
/// at wounded ground too; demanding agar hung the ring hand once the whole
/// ring was wounded).
fn on_agar(d: &Dish, x: f32, y: f32) -> bool {
    x >= 0.0
        && y >= 0.0
        && (x as usize) < W
        && (y as usize) < H
        && matches!(d.ground[y as usize * W + x as usize], Ground::Agar | Ground::Wound)
}

/// The line the hand fires this second, if any: centre and unit direction.
fn aim(d: &Dish, hand: Hand, every: u64, rng: &mut Rng) -> Option<((f32, f32), (f32, f32))> {
    if hand == Hand::Idle {
        return None;
    }
    if hand == Hand::Fort {
        // Arc k of the ring, chosen by the shot count, fired along its chord.
        let k = (d.t / every) % FORT_ARCS;
        let a = (k as f32 + 0.5) / FORT_ARCS as f32 * std::f32::consts::TAU;
        return Some((
            (LARDER.0 + RING * a.cos(), LARDER.1 + RING * a.sin()),
            (-a.sin(), a.cos()),
        ));
    }
    let near: Vec<(f32, f32)> = d
        .walkers
        .iter()
        .filter(|w| (w.x - LARDER.0).hypot(w.y - LARDER.1) < NEAR)
        .map(|w| (w.x, w.y))
        .collect();
    if near.is_empty() {
        return None;
    }
    let ang = rng.f() * std::f32::consts::PI;
    let random_dir = (ang.cos(), ang.sin());
    match hand {
        Hand::Aim => {
            // The thickest tube crossing the guard ring round the larder:
            // a supply line in. Fire across it (along the ring).
            let mut best: Option<(f32, f32)> = None;
            for k in 0..RING_POINTS {
                let a = k as f32 / RING_POINTS as f32 * std::f32::consts::TAU;
                let (x, y) = (LARDER.0 + RING * a.cos(), LARDER.1 + RING * a.sin());
                if !on_agar(d, x, y) {
                    continue;
                }
                let tr = d.trail[y as usize * W + x as usize];
                if tr > d.params.tube && best.is_none_or(|b| tr > b.0) {
                    best = Some((tr, a));
                }
            }
            let (_, a) = best?;
            Some((
                (LARDER.0 + RING * a.cos(), LARDER.1 + RING * a.sin()),
                (-a.sin(), a.cos()),
            ))
        }
        Hand::Near => loop {
            // The same ring, blind: a random spot on it, fired along it.
            let a = rng.f() * std::f32::consts::TAU;
            let at = (LARDER.0 + RING * a.cos(), LARDER.1 + RING * a.sin());
            if on_agar(d, at.0, at.1) {
                return Some((at, (-a.sin(), a.cos())));
            }
        },
        Hand::Pile => Some((LARDER, random_dir)),
        Hand::Rand => loop {
            let at = (rng.f() * W as f32, rng.f() * H as f32);
            if on_agar(d, at.0, at.1) {
                return Some((at, random_dir));
            }
        },
        Hand::Idle | Hand::Fort => None,
    }
}

fn snip(d: &mut Dish, (c, u): ((f32, f32), (f32, f32))) {
    let h = LINE / 2.0;
    if d.begin_stroke(Tool::Snip, c.0 - u.0 * h, c.1 - u.1 * h) {
        d.drag_stroke(c.0 + u.0 * h, c.1 + u.1 * h);
        d.end_stroke();
    }
}

fn clear(d: &mut Dish, (c, u): ((f32, f32), (f32, f32))) {
    let n = (LINE * 2.0) as i32;
    for k in 0..=n {
        let s = -LINE / 2.0 + LINE * k as f32 / n as f32;
        let (x, y) = (c.0 + u.0 * s, c.1 + u.1 * s);
        for (ox, oy) in [(0.0, 0.0), (1.0, 0.0), (-1.0, 0.0), (0.0, 1.0), (0.0, -1.0)] {
            let (px, py) = (x + ox, y + oy);
            if px >= 0.0 && py >= 0.0 && (px as usize) < W && (py as usize) < H {
                d.trail[py as usize * W + px as usize] = 0.0;
            }
        }
    }
}

/// Loops: holes in the tube (cells over `Params::tube`), as 4-connected
/// regions of non-tube agar of at least 9 cells that do not reach the rim.
fn loops(d: &Dish) -> usize {
    let tube = d.params.tube;
    let is_tube = |i: usize| d.ground[i] == Ground::Agar && d.trail[i] > tube;
    let mut seen = vec![false; W * H];
    let mut holes = 0;
    let mut stack = Vec::new();
    for s in 0..W * H {
        if seen[s] || is_tube(s) {
            continue;
        }
        seen[s] = true;
        stack.push(s);
        let (mut size, mut open) = (0usize, false);
        while let Some(i) = stack.pop() {
            size += 1;
            if d.ground[i] == Ground::Rim {
                open = true;
            }
            let (x, y) = (i % W, i / W);
            if x == 0 || y == 0 || x == W - 1 || y == H - 1 {
                open = true;
            }
            let mut push = |j: usize| {
                if !seen[j] && !is_tube(j) {
                    seen[j] = true;
                    stack.push(j);
                }
            };
            if x > 0 {
                push(i - 1);
            }
            if x + 1 < W {
                push(i + 1);
            }
            if y > 0 {
                push(i - W);
            }
            if y + 1 < H {
                push(i + W);
            }
        }
        if !open && size >= 9 {
            holes += 1;
        }
    }
    holes
}

fn run(seed: u64, s: Setup, look: Option<&str>) -> Out {
    let mut d = dish(seed, s);
    let li = larder_index(&d);
    let full = d.piles[li].full;
    let mut rng = Rng(seed.wrapping_mul(0x9E37_79B9_7F4A_7C15) | 1);
    let mut out = Out::default();
    let mut held: Option<((f32, f32), (f32, f32))> = None;
    for t in 0..SECONDS * SPS {
        if t % s.every == 0 {
            held = aim(&d, s.hand, s.every, &mut rng);
            if let Some(line) = held {
                out.shots += 1;
                if s.gun == Gun::Snip {
                    snip(&mut d, line);
                }
            }
        }
        if s.gun == Gun::Clear {
            if let Some(line) = held {
                clear(&mut d, line);
            }
        }
        d.step();
        if t % (SPS / 2) == 0 && t < 8 * SPS && std::env::var("DEBUG").is_ok() {
            let wounds = d.ground.iter().filter(|g| **g == Ground::Wound).count();
            let inside = d
                .walkers
                .iter()
                .filter(|w| (w.x - LARDER.0).hypot(w.y - LARDER.1) < RING - 3.0)
                .count();
            eprintln!(
                "t {:.1} s: wound cells {wounds}, walkers inside ring {inside}, larder eaters {}",
                t as f32 / SPS as f32,
                d.piles[li].eaters
            );
        }
        if out.first_bite.is_none() && d.piles[li].eaters > 0 {
            out.first_bite = Some(t as f32 / SPS as f32);
        }
        if t + 1 == LOOPS_AT * SPS {
            out.loops45 = loops(&d);
            out.walkers45 = d.walkers.len();
        }
        if let Some(dir) = look {
            if (t + 1) % (10 * SPS) == 0 {
                save(&d, &format!("{dir}/s{seed}-{:?}-{:03}.png", s.hand, (t + 1) / SPS));
            }
        }
    }
    out.stolen = full - d.piles[li].food;
    // `Dish::ate` is kept only with a rival; with one mould every bite off
    // a pile is its own.
    out.ate = d
        .piles
        .iter()
        .enumerate()
        .filter(|&(k, _)| k != li)
        .map(|(_, p)| p.full - p.food)
        .sum();
    out.walkers = d.walkers.len();
    out
}

fn save(d: &Dish, path: &str) {
    use slime_mould::mould::render::{self, View};
    let scale = 3;
    let (w, h) = render::frame_size(scale);
    let mut buf = vec![0u8; (w * h * 4) as usize];
    let view = View {
        keys: false,
        ..View::default()
    };
    render::draw(d, &view, scale, &mut buf);
    image::RgbaImage::from_raw(w, h, buf)
        .expect("frame")
        .save(path)
        .expect("save png");
}

/// Two-sided sign test on `wins` of `n` (ties dropped by the caller).
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

/// Paired comparison of `a` against `b` on one measure; lower is better
/// when `low` is set. Returns "a better on W of N, p".
fn paired(name_a: &str, name_b: &str, a: &[f32], b: &[f32], low: bool) -> String {
    let (mut w, mut n) = (0, 0);
    for (x, y) in a.iter().zip(b) {
        if (x - y).abs() < 1e-3 {
            continue;
        }
        n += 1;
        if (x < y) == low {
            w += 1;
        }
    }
    format!("{name_a} better than {name_b} on {w} of {n} (p {:.4})", sign_p(w, n))
}

fn par<F: Fn(u64) -> Out + Sync>(seeds: &[u64], f: F) -> Vec<Out> {
    assert!(!seeds.is_empty(), "no seeds");
    let threads = std::thread::available_parallelism().map(|n| n.get()).unwrap_or(1);
    let mut out = vec![Out::default(); seeds.len()];
    std::thread::scope(|sc| {
        let f = &f;
        let chunks: Vec<_> = out
            .chunks_mut(seeds.len().div_ceil(threads))
            .zip(seeds.chunks(seeds.len().div_ceil(threads)))
            .collect();
        for (o, s) in chunks {
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

fn show(name: &str, seeds: &[u64], o: &[Out]) {
    println!("{name}:");
    for (s, r) in seeds.iter().zip(o) {
        println!(
            "  seed {s:2}: stolen {:7.0}  ate {:8.0}  walkers {:5}  first bite {:>6}  shots {:3}  loops@45 {:3}  walkers@45 {:5}",
            r.stolen,
            r.ate,
            r.walkers,
            r.first_bite.map(|t| format!("{t:.1}")).unwrap_or("never".into()),
            r.shots,
            r.loops45,
            r.walkers45
        );
    }
    println!(
        "  median stolen {:.0}, ate {:.0}, walkers {:.0}, first bite {:.1} s, loops@45 {:.0}",
        median(o.iter().map(|r| r.stolen).collect()),
        median(o.iter().map(|r| r.ate).collect()),
        median(o.iter().map(|r| r.walkers as f32).collect()),
        median(o.iter().map(|r| r.first_bite.unwrap_or(SECONDS as f32)).collect()),
        median(o.iter().map(|r| r.loops45 as f32).collect()),
    );
}

// ------------------------------------------------------------ the economy

/// **The priced ring** (`econ`; the design review's test, owner's yes at
/// 2026-11-11 03:03Z). The mould is slowed: one game second is `SLOW_STEPS`
/// steps (an eighth of today's second), so it first reaches the larder at
/// about 30 game seconds instead of 4, and wounds last `Params::wound`
/// game seconds. The squad holds three piles: the larder and the two field
/// piles nearest it. Each game second it harvests `HARVEST` food from each
/// held pile with food left and **no mould eating there**; what it takes
/// comes off the pile, so the larder runs dry. Every shot costs `price`.
/// The score is the bank at the end: food harvested less food spent.
const SLOW_STEPS: u64 = SPS / 8;
const ROUND: u64 = 480;
const HARVEST: f32 = 100.0;
/// Seconds between ring shots: five arcs refreshed every 3 s, inside the
/// 4 s a wound lasts.
const RING_GAP: f32 = 0.6;

#[derive(Clone, Copy, PartialEq, Debug)]
enum Plan {
    Idle,
    /// One shot a game second anywhere on the dish.
    Random,
    /// The larder's ring from 0 s to the bell, whatever it costs.
    Ring,
    /// Fire through each held pile with mould eating it, once a second each.
    Mow,
    /// Ring on the larder and mow the two away piles.
    RingMow,
    /// Ring the larder only while it still holds food worth guarding
    /// (more than `RING_GAP` seconds of shots would cost), then mow.
    RingThenMow,
}

#[derive(Clone, Copy, Debug, Default)]
struct Econ {
    bank: f32,
    harvested: f32,
    spent: f32,
    shots: u32,
    mould_ate: f32,
    larder_left: f32,
    first_bite: Option<f32>,
}

fn held_piles(d: &Dish) -> Vec<usize> {
    let li = larder_index(d);
    let mut by: Vec<(f32, usize)> = d
        .piles
        .iter()
        .enumerate()
        .filter(|&(k, _)| k != li)
        .map(|(k, p)| ((p.x - LARDER.0).hypot(p.y - LARDER.1), k))
        .collect();
    by.sort_by(|a, b| a.0.total_cmp(&b.0));
    vec![li, by[0].1, by[1].1]
}

fn econ_run(seed: u64, plan: Plan, price: f32, look: Option<&str>) -> Econ {
    let mut d = dish(
        seed,
        Setup {
            hand: Hand::Idle,
            gun: Gun::Snip,
            gain: 0.0,
            food: 1.0,
            every: EVERY,
        },
    );
    // Wounds last 4 game seconds: `wound` is read in today's seconds.
    d.params.wound = 4.0 * SLOW_STEPS as f32 / SPS as f32;
    let held = held_piles(&d);
    let li = held[0];
    let start: Vec<f32> = d.piles.iter().map(|p| p.food).collect();
    let mut rng = Rng(seed.wrapping_mul(0x9E37_79B9_7F4A_7C15) | 1);
    let mut e = Econ::default();
    let ring_every = (RING_GAP * SLOW_STEPS as f32) as u64;
    let mut arc = 0u64;
    let mut squad_took = vec![0.0f32; d.piles.len()];
    for t in 0..ROUND * SLOW_STEPS {
        let second = t % SLOW_STEPS == 0;
        let fire = |d: &mut Dish, line: ((f32, f32), (f32, f32)), e: &mut Econ| {
            if e.bank >= price {
                e.bank -= price;
                e.spent += price;
                e.shots += 1;
                snip(d, line);
            }
        };
        let larder_worth = d.piles[li].food > HARVEST * 30.0;
        let ring_on = match plan {
            Plan::Ring | Plan::RingMow => true,
            Plan::RingThenMow => larder_worth,
            _ => false,
        };
        if ring_on && t % ring_every == 0 {
            let a = (arc % FORT_ARCS) as f32 + 0.5;
            let a = a / FORT_ARCS as f32 * std::f32::consts::TAU;
            arc += 1;
            fire(
                &mut d,
                (
                    (LARDER.0 + RING * a.cos(), LARDER.1 + RING * a.sin()),
                    (-a.sin(), a.cos()),
                ),
                &mut e,
            );
        }
        if second {
            let mow: &[usize] = match plan {
                Plan::Mow => &held,
                Plan::RingMow => &held[1..],
                Plan::RingThenMow => {
                    if ring_on {
                        &held[1..]
                    } else {
                        &held
                    }
                }
                _ => &[],
            };
            for &k in mow {
                if d.piles[k].eaters > 0 {
                    let ang = rng.f() * std::f32::consts::PI;
                    let at = (d.piles[k].x, d.piles[k].y);
                    fire(&mut d, (at, (ang.cos(), ang.sin())), &mut e);
                }
            }
            if plan == Plan::Random {
                let ang = rng.f() * std::f32::consts::PI;
                let at = loop {
                    let at = (rng.f() * W as f32, rng.f() * H as f32);
                    if on_agar(&d, at.0, at.1) {
                        break at;
                    }
                };
                fire(&mut d, (at, (ang.cos(), ang.sin())), &mut e);
            }
            // The harvest: none from a pile the mould is eating at.
            for &k in &held {
                let p = &mut d.piles[k];
                if p.eaters == 0 && p.food > 0.0 {
                    let take = HARVEST.min(p.food);
                    p.food -= take;
                    squad_took[k] += take;
                    e.bank += take;
                    e.harvested += take;
                }
            }
        }
        d.step();
        if e.first_bite.is_none() && d.piles[li].eaters > 0 {
            e.first_bite = Some(t as f32 / SLOW_STEPS as f32);
        }
        if let Some(dir) = look {
            if (t + 1) % (60 * SLOW_STEPS) == 0 {
                save(
                    &d,
                    &format!("{dir}/econ-s{seed}-{plan:?}-p{price}-{:03}.png", (t + 1) / SLOW_STEPS),
                );
            }
        }
    }
    e.mould_ate = d
        .piles
        .iter()
        .enumerate()
        .map(|(k, p)| start[k] - p.food - squad_took[k])
        .sum();
    e.larder_left = d.piles[li].food;
    e
}

fn econ_main(seeds: &[u64], prices: &[f32]) {
    const PLANS: [Plan; 6] = [
        Plan::Idle,
        Plan::Random,
        Plan::Ring,
        Plan::Mow,
        Plan::RingMow,
        Plan::RingThenMow,
    ];
    println!(
        "priced ring: round {ROUND} game s, a game second = {SLOW_STEPS} steps, harvest {HARVEST}/s from 3 held piles, {} seeds",
        seeds.len()
    );
    for &price in prices {
        println!("\n== shot price {price}");
        let threads = std::thread::available_parallelism().map(|n| n.get()).unwrap_or(1);
        let mut res: Vec<Vec<Econ>> = Vec::new();
        for &plan in &PLANS {
            let mut out = vec![Econ::default(); seeds.len()];
            std::thread::scope(|sc| {
                let n = seeds.len().div_ceil(threads);
                for (o, s) in out.chunks_mut(n).zip(seeds.chunks(n)) {
                    sc.spawn(move || {
                        for (slot, &seed) in o.iter_mut().zip(s) {
                            *slot = econ_run(seed, plan, price, None);
                        }
                    });
                }
            });
            let med = |f: &dyn Fn(&Econ) -> f32| median(out.iter().map(f).collect());
            println!(
                "  {:12} bank {:7.0}  harvested {:7.0}  spent {:6.0}  shots {:5.0}  mould ate {:7.0}  larder left {:6.0}  first bite {:5.1}",
                format!("{plan:?}"),
                med(&|e| e.bank),
                med(&|e| e.harvested),
                med(&|e| e.spent),
                med(&|e| e.shots as f32),
                med(&|e| e.mould_ate),
                med(&|e| e.larder_left),
                med(&|e| e.first_bite.unwrap_or(ROUND as f32)),
            );
            res.push(out);
        }
        // Who wins each seed, and each plan against idle and random.
        let mut wins = [0u32; PLANS.len()];
        // Each seed's best plan (indexing across plans, so a range loop).
        #[allow(clippy::needless_range_loop)]
        for k in 0..seeds.len() {
            let best = (0..PLANS.len())
                .max_by(|&a, &b| res[a][k].bank.total_cmp(&res[b][k].bank))
                .unwrap();
            wins[best] += 1;
        }
        let bank = |i: usize| res[i].iter().map(|e| e.bank).collect::<Vec<_>>();
        for (i, plan) in PLANS.iter().enumerate() {
            println!(
                "  {:12} best on {:2} seeds; {}; {}",
                format!("{plan:?}"),
                wins[i],
                paired("it", "idle", &bank(i), &bank(0), false),
                paired("it", "random", &bank(i), &bank(1), false)
            );
        }
        println!(
            "  ring v ring+mow: {}",
            paired("ring", "ring+mow", &bank(2), &bank(4), false)
        );
        println!("  ring v mow: {}", paired("ring", "mow", &bank(2), &bank(3), false));
        println!(
            "  ring v ring-then-mow: {}",
            paired("ring", "ring-then-mow", &bank(2), &bank(5), false)
        );
    }
}

fn main() {
    let args: Vec<String> = std::env::args().skip(1).collect();
    let seeds = seeds(arg(&args, "seeds"));
    let gun = if arg(&args, "gun") == Some("clear") {
        Gun::Clear
    } else {
        Gun::Snip
    };
    let food: f32 = arg(&args, "food").map(|v| v.parse().unwrap()).unwrap_or(1.0);
    let base = Setup {
        hand: Hand::Idle,
        gun,
        gain: 0.0,
        food,
        every: arg(&args, "every").map(|v| v.parse().unwrap()).unwrap_or(EVERY),
    };
    if let Some(dir) = arg(&args, "look") {
        std::fs::create_dir_all(dir).expect("look dir");
        let s = Setup {
            hand: Hand::parse(arg(&args, "hand").unwrap_or("idle")),
            gain: arg(&args, "gain").map(|v| v.parse().unwrap()).unwrap_or(0.0),
            ..base
        };
        for &seed in &seeds {
            let r = run(seed, s, Some(dir));
            println!("seed {seed} {:?}: {r:?}", s.hand);
        }
        return;
    }
    let t0 = std::time::Instant::now();
    match args.first().map(String::as_str) {
        Some("econ") => {
            let prices: Vec<f32> = arg(&args, "prices")
                .unwrap_or("10,40,120")
                .split(',')
                .map(|v| v.parse().unwrap())
                .collect();
            econ_main(&seeds, &prices);
        }
        Some("econlook") => {
            let dir = arg(&args, "dir").unwrap_or("econlook");
            std::fs::create_dir_all(dir).expect("dir");
            let price: f32 = arg(&args, "price").map(|v| v.parse().unwrap()).unwrap_or(40.0);
            for plan in [Plan::Idle, Plan::Ring, Plan::Mow] {
                let e = econ_run(seeds[0], plan, price, Some(dir));
                println!("{plan:?}: {e:?}");
            }
        }
        Some("fort") => {
            println!("fortress: ring held from 0 s, {SECONDS} s, {} seeds", seeds.len());
            let idle = par(&seeds, |sd| {
                run(
                    sd,
                    Setup {
                        hand: Hand::Idle,
                        ..base
                    },
                    None,
                )
            });
            let fort = par(&seeds, |sd| {
                run(
                    sd,
                    Setup {
                        hand: Hand::Fort,
                        ..base
                    },
                    None,
                )
            });
            show("Idle", &seeds, &idle);
            show("Fort", &seeds, &fort);
            let st = |o: &[Out]| o.iter().map(|r| r.stolen).collect::<Vec<_>>();
            println!("stolen: {}", paired("fort", "idle", &st(&fort), &st(&idle), true));
        }
        Some("fire") => {
            println!(
                "test 1, held fire: gun {}, food x{food}, {SECONDS} s, {} seeds",
                if gun == Gun::Snip { "snip" } else { "clear" },
                seeds.len()
            );
            let hands = [Hand::Idle, Hand::Aim, Hand::Near, Hand::Rand, Hand::Pile];
            let res: Vec<Vec<Out>> = hands
                .iter()
                .map(|&h| par(&seeds, |sd| run(sd, Setup { hand: h, ..base }, None)))
                .collect();
            for (h, o) in hands.iter().zip(&res) {
                show(&format!("{h:?}"), &seeds, o);
            }
            let st = |k: usize| res[k].iter().map(|r| r.stolen).collect::<Vec<_>>();
            let at = |k: usize| res[k].iter().map(|r| r.ate).collect::<Vec<_>>();
            println!("stolen (lower is better for the squad):");
            println!("  {}", paired("aim", "idle", &st(1), &st(0), true));
            println!("  {}", paired("aim", "near", &st(1), &st(2), true));
            println!("  {}", paired("aim", "rand", &st(1), &st(3), true));
            println!("  {}", paired("near", "idle", &st(2), &st(0), true));
            println!("  {}", paired("rand", "idle", &st(3), &st(0), true));
            println!("  {}", paired("pile", "idle", &st(4), &st(0), true));
            println!("  {}", paired("pile", "aim", &st(4), &st(1), true));
            println!("mould ate, all dish (lower is better for the squad):");
            println!("  {}", paired("aim", "idle", &at(1), &at(0), true));
            println!("  {}", paired("aim", "rand", &at(1), &at(3), true));
        }
        Some("smell") => {
            println!("test 2, larder smell: idle hand, {SECONDS} s, {} seeds", seeds.len());
            let gains = [0.0f32, 0.3, 1.0, 3.0];
            let res: Vec<Vec<Out>> = gains
                .iter()
                .map(|&g| par(&seeds, |sd| run(sd, Setup { gain: g, ..base }, None)))
                .collect();
            for (g, o) in gains.iter().zip(&res) {
                show(&format!("gain {g} (0 = plain pile)"), &seeds, o);
            }
            let fb = |k: usize| {
                res[k]
                    .iter()
                    .map(|r| r.first_bite.unwrap_or(SECONDS as f32))
                    .collect::<Vec<_>>()
            };
            let st = |k: usize| res[k].iter().map(|r| r.stolen).collect::<Vec<_>>();
            println!("first bite (lower = reached sooner):");
            println!("  {}", paired("gain 3", "gain 0.3", &fb(3), &fb(1), true));
            println!("  {}", paired("gain 1", "gain 0.3", &fb(2), &fb(1), true));
            println!("  {}", paired("gain 3", "gain 1", &fb(3), &fb(2), true));
            println!("stolen (higher = the hoard drew more theft):");
            println!("  {}", paired("gain 3", "gain 0.3", &st(3), &st(1), false));
        }
        Some("lean") => {
            println!(
                "test 3, lean v fat: piles x{LEAN} v x1, snip, {SECONDS} s, {} seeds",
                seeds.len()
            );
            let mk = |h: Hand, f: f32| Setup {
                hand: h,
                food: f,
                ..base
            };
            let fat_idle = par(&seeds, |sd| run(sd, mk(Hand::Idle, 1.0), None));
            let fat_aim = par(&seeds, |sd| run(sd, mk(Hand::Aim, 1.0), None));
            let lean_idle = par(&seeds, |sd| run(sd, mk(Hand::Idle, LEAN), None));
            let lean_aim = par(&seeds, |sd| run(sd, mk(Hand::Aim, LEAN), None));
            show("fat idle", &seeds, &fat_idle);
            show("fat aim", &seeds, &fat_aim);
            show("lean idle", &seeds, &lean_idle);
            show("lean aim", &seeds, &lean_aim);
            let lp = |o: &[Out]| o.iter().map(|r| r.loops45 as f32).collect::<Vec<_>>();
            println!(
                "loops at 45 s: {}",
                paired("fat", "lean", &lp(&fat_idle), &lp(&lean_idle), false).replace("better than", "more loops than")
            );
            // The aimed hand's saving, as a fraction of what idle lost, so a
            // lean mould that steals less to begin with is not favoured.
            let save = |i: &[Out], a: &[Out]| {
                i.iter()
                    .zip(a)
                    .map(|(i, a)| (i.stolen - a.stolen) / i.stolen.max(1.0))
                    .collect::<Vec<_>>()
            };
            let (sf, sl) = (save(&fat_idle, &fat_aim), save(&lean_idle, &lean_aim));
            println!(
                "aim's saving (fraction of idle's loss), lean v fat: {}",
                paired("lean", "fat", &sl, &sf, false).replace("better than", "saves more than")
            );
            println!("  median saving: fat {:.2}, lean {:.2}", median(sf), median(sl));
        }
        _ => {
            eprintln!("larderprobe fire|smell|lean seeds=A-B, or look=DIR");
            std::process::exit(2);
        }
    }
    println!("({:.1} s)", t0.elapsed().as_secs_f32());
}
