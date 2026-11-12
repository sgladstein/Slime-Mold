//! **The player's own mould: do body forms counter each other?** (the
//! player's-own-mould thread, 2026-11-12; proposal
//! https://claude.ai/artifact/LBDQgb8x9RoH6bE11G8H3m).
//!
//! Two species on a mirrored Larder world, each a different *form* (its
//! own `SpeciesSet`), growing from mirrored home crystals, converting each
//! other where one outweighs the other (`species_convert`,
//! `species_convert_r`). No soldiers and no player: this asks whether the
//! model itself gives forms that beat one form and lose to another, which
//! would let a player's choice of form be a strategy rather than a stat.
//!
//! ```text
//! cargo run --release --example ownmould -- census seeds=1-16 [forms=net,hunk,scout,heavy secs=180 walkers=5000 threads=4]
//! cargo run --release --example ownmould -- look=DIR seeds=1 forms=net,hunk [every=30]
//! ```
//!
//! **The world** (per seed): the race world (`larder_world::race_params`,
//! scale 2) taken as a map, its left half mirrored onto the right
//! (ground and crystals; crystals within their radius of the middle line
//! are dropped), and the mould's two starts on the left crystal nearest
//! (x 1/4, y 1/2) and its mirror. Each start gets half the walkers; the
//! first half is species 0 (left), the second species 1 (right).
//!
//! **A game** of forms A and B is played twice, A left and A right, and
//! A's measure is the mean of the two, so neither side's draws favour a
//! form. **Deciding measure, named before the run:** A's share of the
//! crystal charge the two species ate over the game (each walker's take,
//! `BigDish::took`, summed by its species). A beats B when its share is
//! above 0.5 on at least 14 of 16 seeds (two-sided sign test p 0.004,
//! which survives Bonferroni over 6 pairings). Also printed: walkers of
//! each species at the end, and crystals each holds (a crystal is held by
//! a species with at least twice the other's walkers on it, and 20 or
//! more).
//!
//! **Forms** (settings beyond the race mould's; units as `SpeciesSet`):
//! - `net`: the race mould unchanged (network with growth).
//! - `hunk`: grazing droplets that crawl (the species-sets Drifter: crowd 1,
//!   sensor 90, turn 45, offset 9, deposit 5, keep 0.93, blocked lay 1;
//!   not grazing: with grazing on and growth on, grass fed it to the
//!   slot cap, 49,744 walkers by 3:00 on seed 1, a race no form could run).
//! - `scout`: explorers (`explore_steps` 60): each walker runs straight
//!   for 60 steps before it networks.
//! - `heavy`: three times the deposit at half the step.
//! - `wide`: sensor 45, turn 45 (a coarse, fast-closing net).
//!
//! Every setting is off by default in the library; this example only sets
//! them. Fight keys: `convert=1.2 convert_r=4 avoid=0 per=1`
//! (`per=1` is `species_convert_per`: conversion counts walkers' presence,
//! trail over each species' deposit; with raw trail, `heavy` converted
//! the whole network on seed 1 within 60 s).
use std::sync::atomic::{AtomicUsize, Ordering};
use std::sync::Mutex;

use slime_mould::mould::big::{BigDish, BigParams, SpeciesSet};
use slime_mould::mould::larder_world::{race_params, race_world};
use slime_mould::mould::map::{Crystal, Map, MAP_H, MAP_W};

const TPS: u32 = 15;

#[derive(Clone)]
struct Opts {
    seeds: Vec<u64>,
    forms: Vec<String>,
    secs: u32,
    walkers: usize,
    threads: usize,
    convert: f32,
    convert_r: u32,
    avoid: f32,
    per: bool,
    look: Option<String>,
    every: u32,
}

fn parse_seeds(s: &str) -> Vec<u64> {
    s.split(',')
        .flat_map(|p| match p.split_once('-') {
            Some((a, b)) => (a.parse::<u64>().unwrap()..=b.parse::<u64>().unwrap()).collect::<Vec<_>>(),
            None => vec![p.parse().unwrap()],
        })
        .collect()
}

fn opts() -> Opts {
    let mut o = Opts {
        seeds: (1..=16).collect(),
        forms: ["net", "hunk", "scout", "heavy"].iter().map(|s| s.to_string()).collect(),
        secs: 180,
        walkers: 5000,
        threads: 4,
        convert: 1.2,
        convert_r: 4,
        avoid: 0.0,
        per: true,
        look: None,
        every: 30,
    };
    for a in std::env::args().skip(1) {
        let Some((k, v)) = a.split_once('=') else { continue };
        match k {
            "seeds" => o.seeds = parse_seeds(v),
            "forms" => o.forms = v.split(',').map(|s| s.to_string()).collect(),
            "secs" => o.secs = v.parse().unwrap(),
            "walkers" => o.walkers = v.parse().unwrap(),
            "threads" => o.threads = v.parse().unwrap(),
            "convert" => o.convert = v.parse().unwrap(),
            "convert_r" => o.convert_r = v.parse().unwrap(),
            "avoid" => o.avoid = v.parse().unwrap(),
            "per" => o.per = v == "1",
            "look" => o.look = Some(v.to_string()),
            "every" => o.every = v.parse().unwrap(),
            _ => panic!("unknown key {k}"),
        }
    }
    o
}

/// A form's settings, from the race mould's.
fn form(name: &str, p: &BigParams) -> SpeciesSet {
    let mut s = SpeciesSet::from_params(p);
    match name {
        "net" => {}
        "hunk" => {
            s.crowd = 1;
            s.sensor_angle = 90.0;
            s.rotate_angle = 45.0;
            s.sensor_offset = 9.0;
            s.deposit = 5.0;
            s.keep = 0.93;
            s.blocked_deposit = 1.0;
        }
        "scout" => s.explore_steps = 60,
        "heavy" => {
            s.deposit *= 3.0;
            s.step *= 0.5;
        }
        "wide" => {
            s.sensor_angle = 45.0;
            s.rotate_angle = 45.0;
        }
        _ => panic!("unknown form {name}"),
    }
    s
}

/// The race world of `seed` as a map, its left half mirrored onto the right.
fn duel_map(seed: u64, walkers: usize) -> Map {
    let m = Map::of_dish(&race_world(seed, 2, walkers));
    let mut ground = m.ground.clone();
    for y in 0..MAP_H {
        for x in MAP_W / 2..MAP_W {
            ground[y * MAP_W + x] = m.ground[y * MAP_W + (MAP_W - 1 - x)];
        }
    }
    let mid = MAP_W as f32 / 2.0;
    let left: Vec<Crystal> = m.items.crystals.iter().copied().filter(|c| c.x + c.r < mid - 4.0).collect();
    let mut crystals = left.clone();
    crystals.extend(left.iter().map(|c| Crystal { x: MAP_W as f32 - c.x, ..*c }));
    let aim = (MAP_W as f32 / 4.0, MAP_H as f32 / 2.0);
    let home = *left.iter().min_by(|a, b| (a.x - aim.0).hypot(a.y - aim.1).total_cmp(&(b.x - aim.0).hypot(b.y - aim.1))).expect("a left crystal");
    let mut items = m.items.clone();
    items.crystals = crystals;
    items.moulds = vec![(home.x, home.y), (MAP_W as f32 - home.x, home.y)];
    items.landing = None;
    Map { ground, items }
}

fn duel(o: &Opts, seed: u64, left: &str, right: &str) -> BigDish {
    let mut p = race_params(seed, 2, o.walkers);
    p.species = 2;
    p.species_convert = o.convert;
    p.species_convert_r = o.convert_r;
    p.species_avoid = o.avoid;
    p.species_convert_per = o.per;
    p.species_sets = vec![form(left, &p), form(right, &p)];
    let mut d = BigDish::with_map(p, &duel_map(seed, o.walkers));
    // The first home's walkers come first (`place_walkers`); they are
    // species 0, the second home's species 1.
    let n = d.walkers.len();
    let first = o.walkers / 2 + o.walkers % 2;
    for k in 0..n {
        d.spec[k] = (k >= first) as u8;
    }
    d
}

/// One game of a census: the pairing, the seed, the side, and A's numbers
/// then B's.
struct Row {
    i: usize,
    j: usize,
    seed: u64,
    share: f64,
    walkers: [usize; 2],
    held: [usize; 2],
    turned: [u64; 2],
}

#[derive(Clone, Copy, Default, Debug)]
struct Out {
    ate: [f64; 2],
    walkers: [usize; 2],
    held: [usize; 2],
    /// Walkers that changed species (to 0, to 1) over the game.
    turned: [u64; 2],
}

fn census_of(d: &BigDish) -> ([usize; 2], [usize; 2]) {
    let mut n = [0usize; 2];
    let np = d.piles.len();
    let mut on = vec![[0usize; 2]; np];
    let w = d.w;
    for k in 0..d.walkers.len() {
        if !d.lives(k) {
            continue;
        }
        let sp = d.spec[k] as usize & 1;
        n[sp] += 1;
        if d.pile_of.is_empty() {
            continue;
        }
        let (x, y) = (d.walkers[k].0 as usize, d.walkers[k].1 as usize);
        let i = y.min(d.h - 1) * w + x.min(w - 1);
        let pi = d.pile_of[i] as usize;
        if pi > 0 && pi <= np {
            on[pi - 1][sp] += 1;
        }
    }
    let mut held = [0usize; 2];
    for c in on {
        for s in 0..2 {
            if c[s] >= 20 && c[s] >= 2 * c[1 - s] {
                held[s] += 1;
            }
        }
    }
    (n, held)
}

/// Called with the dish and the game second (`look=`).
type Look<'a> = &'a mut dyn FnMut(&BigDish, u32);

fn play(o: &Opts, seed: u64, left: &str, right: &str, mut look: Option<Look>) -> Out {
    let mut d = duel(o, seed, left, right);
    let mut out = Out::default();
    let ticks = o.secs * TPS;
    for t in 1..=ticks {
        let before = d.spec.clone();
        let was: Vec<bool> = (0..d.walkers.len()).map(|k| d.lives(k)).collect();
        d.tick();
        for (k, (&a, &b)) in before.iter().zip(&d.spec).enumerate() {
            // Alive before and after: a child born into a slot is not a turn.
            if a != b && was[k] && d.lives(k) {
                out.turned[b as usize & 1] += 1;
            }
        }
        for k in 0..d.walkers.len() {
            if !d.took.is_empty() && d.lives(k) {
                out.ate[d.spec[k] as usize & 1] += d.took[k] as f64;
            }
        }
        if let Some(f) = look.as_mut() {
            if t % (o.every * TPS) == 0 || t == TPS {
                f(&d, t / TPS);
            }
        }
    }
    let (n, held) = census_of(&d);
    out.walkers = n;
    out.held = held;
    out
}

/// Species 0's trail in one colour, species 1's in another, rock grey.
fn picture(d: &BigDish) -> image::RgbaImage {
    let (w, h) = (d.w, d.h);
    image::RgbaImage::from_fn(w as u32, h as u32, |x, y| {
        let i = y as usize * w + x as usize;
        let f = |v: f32| (1.0 - (-v / 6.0).exp()).clamp(0.0, 1.0);
        let a = f(d.species_trail[0][i]);
        let b = f(d.species_trail[1][i]);
        let rock = !d.rock.is_empty() && d.rock[i];
        let crystal = !d.pile_of.is_empty() && d.pile_of[i] > 0 && d.charge.get(i).copied().unwrap_or(0.0) > 0.0;
        let base = if rock {
            [60.0, 60.0, 64.0]
        } else if crystal {
            [40.0, 70.0, 90.0]
        } else {
            [16.0, 16.0, 20.0]
        };
        let px = [
            base[0] + a * (255.0 - base[0]) * 0.95 + b * 30.0,
            base[1] + a * 150.0 + b * 120.0,
            base[2] + a * 20.0 + b * (255.0 - base[2]) * 0.95,
        ];
        image::Rgba([px[0].min(255.0) as u8, px[1].min(255.0) as u8, px[2].min(255.0) as u8, 255])
    })
}

fn sign_p(wins: usize, losses: usize) -> f64 {
    // Two-sided binomial sign test, ties dropped.
    let n = wins + losses;
    if n == 0 {
        return 1.0;
    }
    let k = wins.min(losses);
    let mut c = 1.0f64;
    let mut tail = 0.0;
    for i in 0..=k {
        if i > 0 {
            c = c * (n - i + 1) as f64 / i as f64;
        }
        tail += c;
    }
    (2.0 * tail / 2f64.powi(n as i32)).min(1.0)
}

fn main() {
    let o = opts();
    println!(
        "ownmould: secs {} walkers {} convert {} convert_r {} avoid {} per {} forms {:?} seeds {:?}",
        o.secs, o.walkers, o.convert, o.convert_r, o.avoid, o.per, o.forms, o.seeds
    );
    if let Some(dir) = o.look.clone() {
        std::fs::create_dir_all(&dir).unwrap();
        let (a, b) = (o.forms[0].clone(), o.forms.get(1).cloned().unwrap_or_else(|| o.forms[0].clone()));
        for &seed in &o.seeds {
            let mut save = |d: &BigDish, s: u32| {
                let (n, held) = census_of(d);
                let path = format!("{dir}/{a}-{b}-s{seed}-t{s:03}.png");
                picture(d).save(&path).unwrap();
                println!("look {a}(left) vs {b}(right) seed {seed} t {s}s walkers {:?} held {:?} -> {path}", n, held);
            };
            let out = play(&o, seed, &a, &b, Some(&mut save));
            println!("look end seed {seed}: ate {:.0} / {:.0}, walkers {:?}, held {:?}, turned to each {:?}", out.ate[0], out.ate[1], out.walkers, out.held, out.turned);
        }
        return;
    }
    // Every pairing (A before B in the list), each seed played both ways.
    let mut jobs = Vec::new();
    // A form against itself is no control here: its two sides are one
    // game mirrored, so their mean is 0.5 by construction.
    for i in 0..o.forms.len() {
        for j in i + 1..o.forms.len() {
            for &s in &o.seeds {
                jobs.push((i, j, s, false));
                jobs.push((i, j, s, true));
            }
        }
    }
    let results = Mutex::new(Vec::new());
    let next = AtomicUsize::new(0);
    std::thread::scope(|sc| {
        for _ in 0..o.threads {
            sc.spawn(|| loop {
                let k = next.fetch_add(1, Ordering::SeqCst);
                if k >= jobs.len() {
                    break;
                }
                let (i, j, s, flip) = jobs[k];
                let (a, b) = (&o.forms[i], &o.forms[j]);
                let out = if flip { play(&o, s, b, a, None) } else { play(&o, s, a, b, None) };
                // A's numbers first.
                let (ia, ib) = if flip { (1, 0) } else { (0, 1) };
                let share = out.ate[ia] / (out.ate[0] + out.ate[1]).max(1e-9);
                println!(
                    "raw {} vs {} seed {} {}: share {:.3} ate {:.0}/{:.0} walkers {}/{} held {}/{} turned to {}/{}",
                    a,
                    b,
                    s,
                    if flip { "A-right" } else { "A-left" },
                    share,
                    out.ate[ia],
                    out.ate[ib],
                    out.walkers[ia],
                    out.walkers[ib],
                    out.held[ia],
                    out.held[ib],
                    out.turned[ia],
                    out.turned[ib]
                );
                results.lock().unwrap().push(Row {
                    i,
                    j,
                    seed: s,
                    share,
                    walkers: [out.walkers[ia], out.walkers[ib]],
                    held: [out.held[ia], out.held[ib]],
                    turned: [out.turned[ia], out.turned[ib]],
                });
            });
        }
    });
    let r = results.into_inner().unwrap();
    println!("\nsummary: A's share of the charge eaten, mean of both sides, by seed");
    for i in 0..o.forms.len() {
        for j in i + 1..o.forms.len() {
            let mut shares = Vec::new();
            let (mut wa, mut wb, mut ha, mut hb, mut ta, mut tb) = (0usize, 0usize, 0usize, 0usize, 0u64, 0u64);
            for &s in &o.seeds {
                let g: Vec<_> = r.iter().filter(|x| x.i == i && x.j == j && x.seed == s).collect();
                let m = g.iter().map(|x| x.share).sum::<f64>() / g.len() as f64;
                shares.push(m);
                for x in &g {
                    wa += x.walkers[0];
                    wb += x.walkers[1];
                    ha += x.held[0];
                    hb += x.held[1];
                    ta += x.turned[0];
                    tb += x.turned[1];
                }
            }
            let wins = shares.iter().filter(|&&v| v > 0.5).count();
            let losses = shares.iter().filter(|&&v| v < 0.5).count();
            let mut sorted = shares.clone();
            sorted.sort_by(|a, b| a.total_cmp(b));
            let med = sorted[sorted.len() / 2];
            let games = (o.seeds.len() * 2) as f64;
            println!(
                "{:>6} vs {:<6} A ahead on {:>2} of {} (p {:.4}), median share {:.3}, range {:.3}-{:.3}; mean walkers {:.0}/{:.0}, held {:.1}/{:.1}, turned to {:.0}/{:.0}",
                o.forms[i],
                o.forms[j],
                wins,
                shares.len(),
                sign_p(wins, losses),
                med,
                sorted[0],
                sorted[sorted.len() - 1],
                wa as f64 / games,
                wb as f64 / games,
                ha as f64 / games,
                hb as f64 / games,
                ta as f64 / games,
                tb as f64 / games
            );
        }
    }
}
