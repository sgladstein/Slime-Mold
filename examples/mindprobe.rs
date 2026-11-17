//! **Provoke and Still** (the simulation-use plan's B1 and B2, 2026-11-17;
//! `big::MIND_DEAF`, `big::MIND_STILL`, the toolkit's `Tool::Provoke` and
//! `Tool::Still`). Films and counts a provoked charge and a stilled surge.
//!
//! ```text
//! cargo run --release --example mindprobe -- [seeds=1-8 grow=63 r=12 gap=15 provoke=12 still=10 watch=15 hold=30 look=DIR]
//! ```
//!
//! For each seed: the default slime (shape C: `shape_keys("c")`, its
//! wither, panic and surge) on the CPU race world at scale 2, grown `grow`
//! game seconds with the surges held (`surge_at` = `SURGE_WAIT`, so no
//! clock surge falls in a window). `panic_r` is the game's 29 dish cells
//! (the gun's 24 plus `Rules::panic_margin`). Then a spot on the body is
//! picked: the walker with the most walkers within `r` dish cells, at
//! least `r` from every crystal's centre. A unit (`BigDish::threats`, as a
//! standing soldier is to the slime; it does not shoot) stands `gap` dish
//! cells beyond the spot's farthest walker in `r`, on the side of the
//! dish's middle. Three tests, each paired against the same state without
//! the status:
//!
//! - **provoke**: the disc of `r` round the spot deaf for `provoke` s (the
//!   toolkit's 12), the surges still held; followed `watch` s.
//! - **still**: a surge started 1 s on (`surge_at`); the disc of `r` round
//!   the body's walker nearest the unit still for `still` s, stamped at the
//!   same tick; followed `watch` s.
//! - **hold**: the disc round the crystal nearest the spot still, stamped
//!   again every `still` s for `hold` s, and the whole body likewise, the
//!   surges held (`only=hold` runs this alone). Does a held part starve
//!   where it stands? (First run: the disc round the spot itself, where no
//!   walker eats, changed nothing on 8 of 8 seeds.)
//!
//! **Deciding measures, named before the first run:** provoke: walkers
//! that come within 3 dish cells of the unit in `watch` s (each counted
//! once), provoke against plain; still: the same during the surge, still
//! against plain; hold: walkers alive in the disc's set at the end, still
//! against plain (and living walkers, the whole body still against plain).
//! Sign tests on the pairs (two-sided).
//!
//! `look=DIR` writes a frame every third of a second of the first seed's
//! provoke and still tests (`provoke-ARM-NNN.png`, `still-ARM-NNN.png`):
//! the trail (pale yellow), walkers whose news is stale ash grey, crystals
//! cyan; the unit a white square with its 3-cell reach a grey ring, the
//! disc a faint ring (marks for the reader, not the simulation).
use slime_mould::mould::big::{BigDish, BigParams, MIND_DEAF, MIND_STILL, SURGE_WAIT};
use slime_mould::mould::larder_world::{race_params, shape_keys};
use slime_mould::mould::tiers;

const TPS: u64 = 15;
const SCALE: usize = 2;
const TOUCH: f32 = 3.0;

fn sign_p(more: usize, less: usize) -> f64 {
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

fn ordinary(d: &BigDish, k: usize) -> bool {
    d.lives(k) && d.life.get(k).is_none_or(|&l| l == slime_mould::mould::big::LIFE_WALKER)
}

fn stale(d: &BigDish, k: usize) -> bool {
    d.news.get(k).is_some_and(|&v| d.steps > v + d.p.wither_lag as u64)
}

/// Fine-cell positions of the crystals' centres.
fn crystals(d: &BigDish) -> Vec<(f32, f32)> {
    (0..d.piles.len())
        .map(|j| d.pile_centre(j))
        .map(|c| ((c & 0xFFFF) as f32 + 0.5, (c >> 16) as f32 + 0.5))
        .collect()
}

/// The unit's stand and the spot: open ground with no walker within
/// `clear` fine cells and the most walkers within `reach`, every crystal
/// at least `reach` from it (so the slime there is the body, not a
/// crystal's crowd); the spot is the mean of those walkers, the body's near
/// side. A grid point every 4 fine cells.
fn pick(d: &BigDish, clear: f32, reach: f32) -> Option<((f32, f32), (f32, f32))> {
    let cs = crystals(d);
    let live: Vec<(f32, f32)> = (0..d.walkers.len())
        .filter(|&k| ordinary(d, k))
        .map(|k| (d.walkers[k].0, d.walkers[k].1))
        .collect();
    let mut best = None;
    let mut most = 0;
    for y in (8..d.h.saturating_sub(8)).step_by(4) {
        for x in (8..d.w.saturating_sub(8)).step_by(4) {
            let u = (x as f32 + 0.5, y as f32 + 0.5);
            if !d.agar[y * d.w + x] || cs.iter().any(|&q| (q.0 - u.0).hypot(q.1 - u.1) < reach) {
                continue;
            }
            let (mut n, mut sx, mut sy, mut ok) = (0usize, 0.0f32, 0.0f32, true);
            for &q in &live {
                let r = (q.0 - u.0).hypot(q.1 - u.1);
                if r < clear {
                    ok = false;
                    break;
                }
                if r <= reach {
                    (n, sx, sy) = (n + 1, sx + q.0, sy + q.1);
                }
            }
            if ok && n > most {
                (most, best) = (n, Some((u, (sx / n as f32, sy / n as f32))));
            }
        }
    }
    best
}

/// One frame of the box `half` fine cells round `c`, 3 px a fine cell.
fn frame(d: &BigDish, c: (f32, f32), half: f32, unit: (f32, f32), disc: ((f32, f32), f32), path: &std::path::Path) {
    const PX: u32 = 3;
    let (x0, y0) = ((c.0 - half).max(0.0) as usize, (c.1 - half).max(0.0) as usize);
    let side = (2.0 * half) as usize;
    let (x1, y1) = ((x0 + side).min(d.w), (y0 + side).min(d.h));
    let mut grey = vec![false; d.w * d.h];
    for k in 0..d.walkers.len() {
        if ordinary(d, k) && stale(d, k) {
            let (x, y) = (d.walkers[k].0 as usize, d.walkers[k].1 as usize);
            if x < d.w && y < d.h {
                grey[y * d.w + x] = true;
            }
        }
    }
    let mut img = image::RgbImage::from_fn((x1 - x0) as u32 * PX, (y1 - y0) as u32 * PX, |px, py| {
        let (x, y) = (x0 + (px / PX) as usize, y0 + (py / PX) as usize);
        let i = y * d.w + x;
        let mut col = if !d.agar[i] {
            [40.0, 40.0, 44.0]
        } else {
            [70.0, 66.0, 58.0]
        };
        if !d.pile_of.is_empty() && d.pile_of[i] != 0 {
            col = [120.0, 220.0, 255.0];
        }
        let tr = (d.trail[i] / 6.0).min(1.0);
        let slime = if grey[i] {
            [150.0, 145.0, 140.0]
        } else {
            [240.0, 215.0, 90.0]
        };
        for (v, t) in col.iter_mut().zip(slime) {
            *v += (t - *v) * tr * 0.85;
        }
        image::Rgb(col.map(|v| v.clamp(0.0, 255.0) as u8))
    });
    let mut put = |x: f32, y: f32, col: [u8; 3]| {
        let (px, py) = ((x - x0 as f32) * PX as f32, (y - y0 as f32) * PX as f32);
        if px >= 0.0 && py >= 0.0 && (px as u32) < img.width() && (py as u32) < img.height() {
            img.put_pixel(px as u32, py as u32, image::Rgb(col));
        }
    };
    for q in 0..256 {
        let a = q as f32 / 256.0 * std::f32::consts::TAU;
        put(
            unit.0 + a.cos() * TOUCH * SCALE as f32,
            unit.1 + a.sin() * TOUCH * SCALE as f32,
            [170, 180, 185],
        );
        put(
            disc.0 .0 + a.cos() * disc.1,
            disc.0 .1 + a.sin() * disc.1,
            [110, 110, 120],
        );
    }
    for dy in -4..=4 {
        for dx in -4..=4 {
            put(
                unit.0 + dx as f32 / PX as f32,
                unit.1 + dy as f32 / PX as f32,
                [255, 255, 255],
            );
        }
    }
    img.save(path).expect("frame");
}

/// A film's folder, name, box centre and the disc's ring.
type Film<'a> = Option<(&'a std::path::Path, &'a str, (f32, f32), ((f32, f32), f32))>;

/// Follows `d` for `ticks`, the unit standing at `unit`: the walkers that
/// came within `TOUCH` dish cells of it (each once), and how many of them
/// were in `set`.
fn follow(d: &mut BigDish, unit: (f32, f32), ticks: u64, set: &[bool], film: Film) -> (usize, usize) {
    let reach = TOUCH * SCALE as f32;
    let mut came = vec![false; d.walkers.len()];
    for t in 0..ticks {
        if let Some((dir, name, c, disc)) = film {
            if t.is_multiple_of(5) {
                frame(d, c, 90.0, unit, disc, &dir.join(format!("{name}-{:03}.png", t / 5)));
            }
        }
        d.threats = vec![(unit.0.floor() as i64, unit.1.floor() as i64)];
        if std::env::var("MINDPROBE_DEBUG").is_ok() && t.is_multiple_of(TPS) {
            let live = |f: &dyn Fn(usize) -> bool| (0..d.walkers.len()).filter(|&k| ordinary(d, k) && f(k)).count();
            let near = live(&|k| (d.walkers[k].0 - unit.0).hypot(d.walkers[k].1 - unit.1) <= 29.0 * SCALE as f32);
            println!(
                "debug {} s: {} with a status, {} stale, {} panicking, {near} within panic reach of the unit",
                t / TPS,
                live(&|k| d.mind_of(k) != 0),
                live(&|k| stale(d, k)),
                live(&|k| d.panics(k))
            );
        }
        d.tick();
        for k in 0..d.walkers.len().min(came.len()) {
            if ordinary(d, k) && (d.walkers[k].0 - unit.0).hypot(d.walkers[k].1 - unit.1) <= reach {
                came[k] = true;
            }
        }
    }
    let n = came.iter().filter(|&&c| c).count();
    let from = came.iter().zip(set).filter(|(&c, &s)| c && s).count();
    (n, from)
}

fn main() {
    let (mut seeds, mut grow, mut r, mut gap) = ((1u64..=8).collect::<Vec<_>>(), 63u64, 12.0f32, 8.0f32);
    let (mut provoke, mut still, mut watch, mut hold) = (12.0f32, 10.0f32, 15u64, 30u64);
    let mut look = None::<std::path::PathBuf>;
    // `only=hold`: skip the provoke and still tests (their counts read 0).
    let mut only = None::<String>;
    for a in std::env::args().skip(1) {
        let (k, v) = a.split_once('=').expect("key=value");
        match k {
            "seeds" => {
                let (a, b) = v.split_once('-').unwrap_or((v, v));
                seeds = (a.parse().unwrap()..=b.parse().unwrap()).collect();
            }
            "grow" => grow = v.parse().unwrap(),
            "r" => r = v.parse().unwrap(),
            "gap" => gap = v.parse().unwrap(),
            "provoke" => provoke = v.parse().unwrap(),
            "still" => still = v.parse().unwrap(),
            "watch" => watch = v.parse().unwrap(),
            "hold" => hold = v.parse().unwrap(),
            "look" => look = Some(v.into()),
            "only" => only = Some(v.to_string()),
            _ => panic!("unknown {k} (seeds, grow, r, gap, provoke, still, watch, hold, look, only)"),
        }
    }
    let (walkers, keys) = shape_keys("c").expect("shape C");
    let s = SCALE as f32;
    let fine = TPS as f32 * 2.0;
    let steps = |secs: f32| (secs * fine).round() as u16;
    println!("mindprobe: race world, shape C, scale {SCALE}, {walkers} walkers; grow {grow} s, disc r {r}, unit {gap} beyond it, provoke {provoke} s, still {still} s, watch {watch} s, hold {hold} s");
    // Per seed: (plain, provoke) came, (plain, still) came, (plain, still) alive in the held disc.
    let mut rows: Vec<[usize; 8]> = Vec::new();
    for &seed in &seeds {
        let mut p: BigParams = race_params(seed, SCALE, walkers);
        for &(k, v) in keys {
            match k {
                "upkeep" => p.upkeep = v.parse().unwrap(),
                "capacity" => p.capacity = v.parse().unwrap(),
                _ => tiers::set_key(&mut p, k, v).unwrap_or_else(|e| panic!("{e}")),
            }
        }
        p.panic_r = 29.0;
        p.surge_at = SURGE_WAIT;
        assert_eq!(p.substeps, 2, "a game second is 30 fine steps");
        let mut d = BigDish::new(p);
        d.threads = 4;
        for _ in 0..grow * TPS {
            d.tick();
        }
        let rr = r * s;
        // The unit stands `gap` dish cells clear of the slime, with the
        // most of it within 20 dish cells (inside the panic reach of 29).
        let Some((unit, c)) = pick(&d, gap * s, 20.0 * s) else {
            println!("{seed}: no stand clear of the slime and of crystals");
            continue;
        };
        let in_r = |d: &BigDish, at: (f32, f32)| {
            (0..d.walkers.len())
                .map(|k| ordinary(d, k) && (d.walkers[k].0 - at.0).hypot(d.walkers[k].1 - at.1) <= rr)
                .collect::<Vec<bool>>()
        };
        let set = in_r(&d, c);
        let n_set = set.iter().filter(|&&b| b).count();
        let film = |name: &'static str, at: (f32, f32)| {
            let dir = look.as_deref().filter(|_| seed == seeds[0])?;
            std::fs::create_dir_all(dir).expect("look dir");
            Some((dir, name, ((at.0 + unit.0) / 2.0, (at.1 + unit.1) / 2.0), (at, rr)))
        };
        // Provoke.
        let skip = only.as_deref() == Some("hold");
        let (mut pc, mut vc, mut sc, mut tc) = (0, 0, 0, 0);
        if !skip {
            let mut plain = d.clone();
            let pf;
            (pc, pf) = follow(&mut plain, unit, watch * TPS, &set, film("provoke-plain", c));
            let mut prov = d.clone();
            let hit = prov.mind(c.0, c.1, rr, MIND_DEAF, steps(provoke));
            let vf;
            (vc, vf) = follow(&mut prov, unit, watch * TPS, &set, film("provoke-deaf", c));
            let lost = (0..set.len()).filter(|&k| set[k] && !prov.lives(k)).count();
            let plain_lost = (0..set.len()).filter(|&k| set[k] && !plain.lives(k)).count();
            println!("{seed} provoke | spot ({:.0}, {:.0}), unit ({:.0}, {:.0}), {n_set} in the disc, {hit} stamped | came within {TOUCH}: plain {pc} ({pf} from the disc), provoked {vc} ({vf}) | the disc's walkers dead after {watch} s: plain {plain_lost}, provoked {lost}", c.0, c.1, unit.0, unit.1);
            // Still, on a surge.
            let near = (0..d.walkers.len()).filter(|&k| ordinary(&d, k)).min_by(|&a, &b| {
                let da = (d.walkers[a].0 - unit.0).hypot(d.walkers[a].1 - unit.1);
                let db = (d.walkers[b].0 - unit.0).hypot(d.walkers[b].1 - unit.1);
                da.total_cmp(&db)
            });
            let at = near.map(|k| (d.walkers[k].0, d.walkers[k].1)).unwrap_or(c);
            let held = in_r(&d, at);
            let mut surge = d.clone();
            surge.p.surge_at = surge.steps as u32 + 1 + 30;
            let mut stilled = surge.clone();
            let sf;
            (sc, sf) = follow(&mut surge, unit, watch * TPS, &held, film("still-plain", at));
            let hit_s = stilled.mind(at.0, at.1, rr, MIND_STILL, steps(still));
            let tf;
            (tc, tf) = follow(&mut stilled, unit, watch * TPS, &held, film("still-held", at));
            println!("{seed} still | disc ({:.0}, {:.0}), {hit_s} stamped, surge from 1 s | came within {TOUCH}: surge {sc} ({sf} from the disc), still {tc} ({tf})", at.0, at.1);
        }
        // Hold: a disc still for `hold` s, stamped again each `still` s,
        // the surges held. First run (11-17): on the spot, which is clear
        // of crystals, it changed nothing on 8 of 8 seeds (no walker there
        // eats, panics or surges, so still and plain tick alike). So the
        // disc is the crystal nearest the spot, and a third arm holds the
        // whole body still.
        let cs = crystals(&d);
        let hc = cs
            .iter()
            .copied()
            .min_by(|a, b| (a.0 - c.0).hypot(a.1 - c.1).total_cmp(&(b.0 - c.0).hypot(b.1 - c.1)))
            .unwrap_or(c);
        let hset = in_r(&d, hc);
        let (mut a0, mut a1, mut a2) = (d.clone(), d.clone(), d.clone());
        let (eat0, all0) = (d.eaten, (0..d.walkers.len()).filter(|&k| ordinary(&d, k)).count());
        let mut t = 0u64;
        while t < hold * TPS {
            if t.is_multiple_of((still * TPS as f32) as u64) {
                a1.mind(hc.0, hc.1, rr, MIND_STILL, steps(still));
                let big = (a2.w + a2.h) as f32;
                a2.mind(a2.w as f32 / 2.0, a2.h as f32 / 2.0, big, MIND_STILL, steps(still));
            }
            for w in [&mut a0, &mut a1, &mut a2] {
                w.threats = vec![(unit.0.floor() as i64, unit.1.floor() as i64)];
                w.tick();
            }
            t += 1;
        }
        let alive = |w: &BigDish| (0..hset.len()).filter(|&k| hset[k] && ordinary(w, k)).count();
        let grey = |w: &BigDish| (0..w.walkers.len()).filter(|&k| ordinary(w, k) && stale(w, k)).count();
        let all = |w: &BigDish| (0..w.walkers.len()).filter(|&k| ordinary(w, k)).count();
        let n_h = hset.iter().filter(|&&b| b).count();
        println!(
            "{seed} hold | crystal disc ({:.0}, {:.0}), {n_h} walkers; after {hold} s: the disc's alive plain {}, still {}, all still {}; stale walkers anywhere plain {}, still {}, all still {}; living {all0} -> plain {}, still {}, all still {}; eaten plain {:.0}, still {:.0}, all still {:.0}",
            hc.0, hc.1, alive(&a0), alive(&a1), alive(&a2), grey(&a0), grey(&a1), grey(&a2), all(&a0), all(&a1), all(&a2), a0.eaten - eat0, a1.eaten - eat0, a2.eaten - eat0
        );
        rows.push([pc, vc, sc, tc, alive(&a0), alive(&a1), all(&a0), all(&a2)]);
    }
    let pair = |a: usize, b: usize| {
        let more = rows.iter().filter(|r| r[b] > r[a]).count();
        let less = rows.iter().filter(|r| r[b] < r[a]).count();
        (more, less, sign_p(more, less))
    };
    let mean = |a: usize| rows.iter().map(|r| r[a] as f64).sum::<f64>() / rows.len().max(1) as f64;
    let (m, l, p) = pair(0, 1);
    println!(
        "provoke: came within {TOUCH}, mean plain {:.0}, provoked {:.0}; provoked more {m}, less {l}, sign p {p:.3}",
        mean(0),
        mean(1)
    );
    let (m, l, p) = pair(2, 3);
    println!(
        "still: came within {TOUCH} in a surge, mean plain {:.0}, still {:.0}; still more {m}, less {l}, sign p {p:.3}",
        mean(2),
        mean(3)
    );
    let (m, l, p) = pair(4, 5);
    println!("hold: the crystal disc's walkers alive, mean plain {:.0}, still {:.0}; still more {m}, less {l}, sign p {p:.3}", mean(4), mean(5));
    let (m, l, p) = pair(6, 7);
    println!(
        "hold: living walkers, mean plain {:.0}, all still {:.0}; all still more {m}, less {l}, sign p {p:.3}",
        mean(6),
        mean(7)
    );
}
