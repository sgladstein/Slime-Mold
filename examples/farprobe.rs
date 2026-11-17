//! **A far smell** (the simulation-use plan's S2, 2026-11-17;
//! `BigParams::far_smell`): the walking distance to the nearest fresh
//! crystal, read by hungry walkers and by specks.
//!
//! ```text
//! cargo run --release --example farprobe -- [seeds=1-8 depths=1,2,3 secs=150 smells=0,0.5 scale=2 every=2 look=DIR]
//! ```
//!
//! For each seed and depth: a generated planet (`levelgen::level`), the
//! default slime (shape C) with the run's enemy keys for that depth
//! (`run::enemy_keys`: surges, specks), on the CPU race world, no squad.
//! The far smell's field is rebuilt every `every` game seconds from the
//! fresh crystals, as the game does (`Game::time_food_paths`, set the
//! next time round). Played `secs` game seconds, once per `far_smell`.
//!
//! **Deciding measure, named before the run:** crystals the slime has
//! reached (any charge eaten: under `FAR_FRESH` left) at the end,
//! `far_smell` on against 0, paired by seed and depth. (The first look
//! counted crystals eaten under `ROAM_FRESH`: none were in 90 s, as the
//! slime eats slowly.) Also: the living at the end,
//! specks sent, landed and given up (`SPECK_TRIP`), and the walkers at
//! least `FAR_HUNGER` hungry at 30 s.
//!
//! `look=DIR` writes a still a second of the first seed and depth's arms
//! (`farS-NNN.png`): the slime by its trail, specks white, fresh crystals
//! cyan, reached ones dark blue.
use slime_mould::mould::big::{food_field_for, BigDish, BigParams, FAR_FRESH, FAR_HUNGER, RIPE, SPECK_TRIP};
use slime_mould::mould::larder_world::{race_params, shape_keys};
use slime_mould::mould::{levelgen, run, tiers};

const TPS: u64 = 15;

fn sign_p(more: usize, less: usize) -> f64 {
    // Two-sided exact binomial at one half, ties dropped.
    let n = more + less;
    if n == 0 {
        return 1.0;
    }
    let k = more.min(less);
    let mut c = 1.0f64;
    let mut tail = 0.0;
    for i in 0..=k {
        if i > 0 {
            c *= (n - i + 1) as f64 / i as f64;
        }
        tail += c;
    }
    (2.0 * tail / 2f64.powi(n as i32)).min(1.0)
}

fn living(d: &BigDish) -> usize {
    (0..d.walkers.len()).filter(|&k| d.lives(k) && d.life.get(k).is_none_or(|&l| l == slime_mould::mould::big::LIFE_WALKER)).count()
}

/// Crystals the slime has reached: any of their charge eaten (under
/// `FAR_FRESH` left).
fn touched(d: &BigDish) -> usize {
    (0..d.piles.len()).filter(|&j| d.crystal_left(j) < FAR_FRESH).count()
}

/// A still of the whole dish, a pixel a fine cell.
fn still(d: &BigDish, path: &std::path::Path) {
    let mut img = image::RgbImage::new(d.w as u32, d.h as u32);
    for i in 0..d.w * d.h {
        let (x, y) = ((i % d.w) as u32, (i / d.w) as u32);
        let c = if !d.agar[i] {
            [38, 37, 43]
        } else if !d.pile_of.is_empty() && d.pile_of[i] != 0 {
            if d.crystal_left(d.pile_of[i] as usize - 1) >= FAR_FRESH { [120, 214, 230] } else { [40, 70, 140] }
        } else {
            let v = (d.trail[i] / 6.0).min(1.0);
            [(70.0 + 146.0 * v) as u8, (64.0 + 140.0 * v) as u8, (56.0 + 34.0 * v) as u8]
        };
        img.put_pixel(x, y, image::Rgb(c));
    }
    for k in 0..d.walkers.len() {
        if d.lives(k) && d.roam_of(k).is_some() {
            let (x, y, _) = d.walkers[k];
            let (cx, cy) = (x.floor() as u32, y.floor() as u32);
            if (cx as usize) < d.w && (cy as usize) < d.h {
                img.put_pixel(cx, cy, image::Rgb([255, 255, 255]));
            }
        }
    }
    img.save(path).expect("still");
}

struct Arm {
    eaten: usize,
    living: usize,
    sent: usize,
    landed: usize,
    gave_up: usize,
    hungry: usize,
}

fn play(mut d: BigDish, secs: u64, every: u64, look: Option<(&std::path::Path, &str)>) -> Arm {
    let mut a = Arm { eaten: 0, living: 0, sent: 0, landed: 0, gave_up: 0, hungry: 0 };
    let mut pending: Option<Vec<u32>> = None;
    for t in 0..secs * TPS {
        if t.is_multiple_of(every * TPS) && d.p.far_smell > 0.0 {
            // As the game: the field built now is set the next time round.
            if let Some(f) = pending.take() {
                d.food_field = f;
            }
            pending = Some(food_field_for(&d.agar, d.w, d.h, &d.fresh_crystals()));
        }
        if let Some((dir, name)) = look {
            if t.is_multiple_of(TPS) {
                still(&d, &dir.join(format!("{name}-{:03}.png", t / TPS)));
            }
        }
        if std::env::var("FARPROBE_DEBUG").is_ok() && t.is_multiple_of(10 * TPS) && !d.food_field.is_empty() {
            let sp: Vec<u32> = (0..d.walkers.len()).filter(|&k| d.lives(k) && d.roam_of(k).is_some()).map(|k| {
                let (x, y, _) = d.walkers[k];
                d.food_dist((y.floor() as usize).min(d.h - 1) * d.w + (x.floor() as usize).min(d.w - 1))
            }).collect();
            let none = sp.iter().filter(|&&v| v == u32::MAX).count();
            let mut v: Vec<u32> = sp.iter().copied().filter(|&v| v < u32::MAX).collect();
            v.sort_unstable();
            let q = |f: f32| v.get(((v.len() as f32 * f) as usize).min(v.len().saturating_sub(1))).copied().unwrap_or(0);
            println!("debug {} s: {} specks, {none} where no way reaches; distance 10/50/90%: {} {} {}; fresh crystals {}", t / TPS, sp.len(), q(0.1), q(0.5), q(0.9), d.fresh_crystals().len());
        }
        if std::env::var("FARPROBE_DEBUG").is_ok() && t == 30 * TPS && !d.energy.is_empty() {
            let mut e: Vec<f32> = (0..d.walkers.len()).filter(|&k| d.lives(k)).map(|k| d.energy[k]).collect();
            e.sort_by(|a, b| a.total_cmp(b));
            let q: Vec<String> = (0..=10).map(|i| format!("{:.2}", e[((e.len() - 1) * i) / 10])).collect();
            println!("debug energy deciles at 30 s (hunger_at {}): {}", d.p.hunger_at, q.join(" "));
        }
        if t == 30 * TPS {
            a.hungry = (0..d.walkers.len()).filter(|&k| d.lives(k) && !d.energy.is_empty() && d.p.hunger_at > 0.0 && 1.0 - d.energy[k] / d.p.hunger_at >= FAR_HUNGER).count();
        }
        let before: Vec<(bool, u32)> = (0..d.walkers.len()).map(|k| (d.lives(k) && d.roam_of(k).is_some(), d.dense.get(k).map_or(0, |w| w & RIPE))).collect();
        d.tick();
        for (k, &(was, trip)) in before.iter().enumerate() {
            let now = d.lives(k) && d.roam_of(k).is_some();
            if !was && now {
                a.sent += 1;
            } else if was && !now && d.lives(k) {
                if trip + 1 >= SPECK_TRIP { a.gave_up += 1 } else { a.landed += 1 }
            }
        }
    }
    a.eaten = touched(&d);
    a.living = living(&d);
    a
}

fn main() {
    let (mut seeds, mut depths, mut secs, mut scale, mut every) = ((1u64..=8).collect::<Vec<_>>(), vec![1u32, 2, 3], 150u64, 2usize, 2u64);
    let mut smells = vec![0.0f32, 0.5];
    let (cw, keys) = shape_keys("c").expect("shape C");
    let mut walkers = cw;
    let mut look = None::<std::path::PathBuf>;
    for a in std::env::args().skip(1) {
        let (k, v) = a.split_once('=').expect("key=value");
        match k {
            "seeds" => {
                let (a, b) = v.split_once('-').unwrap_or((v, v));
                seeds = (a.parse().unwrap()..=b.parse().unwrap()).collect();
            }
            "depths" => depths = v.split(',').map(|x| x.parse().unwrap()).collect(),
            "secs" => secs = v.parse().unwrap(),
            "smells" => smells = v.split(',').map(|x| x.parse().unwrap()).collect(),
            "scale" => scale = v.parse().unwrap(),
            "every" => every = v.parse().unwrap(),
            "walkers" => walkers = v.parse().unwrap(),
            "look" => look = Some(v.into()),
            _ => panic!("unknown {k} (seeds, depths, secs, smells, scale, every, walkers, look)"),
        }
    }
    println!("farprobe: shape C + the run's enemy keys, scale {scale}, {walkers} walkers, no squad; {secs} s, field every {every} s; far_smell {smells:?}");
    println!("seed depth far_smell | crystals | reached | living | specks sent, landed, gave up | hungry at 30 s");
    let mut pairs: Vec<Vec<usize>> = vec![Vec::new(); smells.len()];
    for &depth in &depths {
        for &seed in &seeds {
            let m = levelgen::level(seed, depth);
            let mut p: BigParams = race_params(seed, scale, walkers);
            for &(k, v) in keys {
                match k {
                    "upkeep" => p.upkeep = v.parse().unwrap(),
                    "capacity" => p.capacity = v.parse().unwrap(),
                    _ => tiers::set_key(&mut p, k, v).unwrap_or_else(|e| panic!("{e}")),
                }
            }
            for (k, v) in run::enemy_keys(depth) {
                tiers::set_key(&mut p, k, &v).unwrap_or_else(|e| panic!("{e}"));
            }
            let d0 = BigDish::with_map(p, &m);
            for (q, &fs) in smells.iter().enumerate() {
                let mut d = d0.clone();
                d.threads = 4;
                d.p.far_smell = fs;
                let name = format!("far{fs}");
                let film = look.as_deref().filter(|_| seed == seeds[0] && depth == depths[0]).map(|dir| {
                    std::fs::create_dir_all(dir).expect("look dir");
                    (dir, name.as_str())
                });
                let a = play(d, secs, every, film);
                println!("{seed} {depth} {fs} | {} | {} | {} | {}, {}, {} | {}", d0.piles.len(), a.eaten, a.living, a.sent, a.landed, a.gave_up, a.hungry);
                pairs[q].push(a.eaten);
            }
        }
    }
    for q in 1..smells.len() {
        let (more, less) = pairs[q].iter().zip(&pairs[0]).fold((0, 0), |(m, l), (a, b)| (m + (a > b) as usize, l + (a < b) as usize));
        println!("far_smell {} against 0: crystals reached {} against {}, more {more}, fewer {less}, sign p {:.3}", smells[q], pairs[q].iter().sum::<usize>(), pairs[0].iter().sum::<usize>(), sign_p(more, less));
    }
}
