//! **Charges round walls** (the simulation-use plan's S1, 2026-11-17;
//! `BigDish::units_field`): a surge at a unit behind a wall, steering
//! straight at it against going down the units' walking distance.
//!
//! ```text
//! cargo run --release --example wallprobe -- [seeds=1-8 grow=63 gap=8 wall=8 depth=6 watch=15 look=DIR]
//! ```
//!
//! Shape C at its 25,000 walkers on the CPU race world (scale 2), grown
//! `grow` s with the surges held, `panic_r` 29 dish cells (the game's). A
//! unit stands `gap` dish cells clear of the slime with the most slime
//! within 20 (as `mindprobe`'s), and a cup of walls 1.5 dish cells thick
//! round it: its back `wall` dish cells each way across the line from the
//! slime, 3 dish cells in front of the unit, and its sides running from the
//! back's ends `depth` dish cells past the unit, so the way in opens away
//! from the slime (a first try, a straight wall alone, was flanked: the
//! body round the unit came from every side, 135 straight against 122). A surge starts 1 s in; the arms are the same
//! dish steering straight at the unit (the field empty) and down the
//! units' walking distance (`units_field_for` from the unit, built once:
//! the unit stands still), and a third with no wall.
//!
//! **Deciding measure, named before the run:** walkers that come within 3
//! dish cells of the unit in `watch` s (each counted once), the field
//! against straight, paired by seed. Also: the walkers piled against the
//! wall's slime side at the end (within 4 dish cells of its middle).
//!
//! `look=DIR` films the first seed's arms (`straight-NNN.png`,
//! `field-NNN.png`, `open-NNN.png`), a frame every third of a second.
use slime_mould::mould::big::{units_field_for, BigDish, BigParams, SURGE_WAIT};
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
    // Clear of the dish's rim by 30 dish cells, so the cup's open side is
    // not against the rock border (a first pick stood the unit 22 cells
    // from the west edge, its cup closed by the rock).
    let edge = 30 * d.p.scale;
    for y in (edge..d.h.saturating_sub(edge)).step_by(4) {
        for x in (edge..d.w.saturating_sub(edge)).step_by(4) {
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
        if std::env::var("WALLPROBE_DEBUG").is_ok() && t.is_multiple_of(TPS) {
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
    let (mut seeds, mut grow, mut gap, mut wall, mut depth, mut watch) = ((1u64..=8).collect::<Vec<_>>(), 63u64, 8.0f32, 8.0f32, 6.0f32, 15u64);
    let mut look = None::<std::path::PathBuf>;
    for a in std::env::args().skip(1) {
        let (k, v) = a.split_once('=').expect("key=value");
        match k {
            "seeds" => {
                let (a, b) = v.split_once('-').unwrap_or((v, v));
                seeds = (a.parse().unwrap()..=b.parse().unwrap()).collect();
            }
            "grow" => grow = v.parse().unwrap(),
            "gap" => gap = v.parse().unwrap(),
            "wall" => wall = v.parse().unwrap(),
            "depth" => depth = v.parse().unwrap(),
            "watch" => watch = v.parse().unwrap(),
            "look" => look = Some(v.into()),
            _ => panic!("unknown {k} (seeds, grow, gap, wall, depth, watch, look)"),
        }
    }
    let (walkers, keys) = shape_keys("c").expect("shape C");
    let s = SCALE as f32;
    println!("wallprobe: race world, shape C, scale {SCALE}, {walkers} walkers; grow {grow} s, unit {gap} clear of the slime, wall {wall} each way, watch {watch} s");
    println!("seed | unit (x, y), wall cells | came within {TOUCH}: no wall, straight, field | piled at the wall: straight, field");
    let mut rows: Vec<[usize; 3]> = Vec::new();
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
        let mut d = BigDish::new(p);
        d.threads = 4;
        for _ in 0..grow * TPS {
            d.tick();
        }
        let Some((unit, c)) = pick(&d, gap * s, 20.0 * s) else {
            println!("{seed}: no stand clear of the slime and of crystals");
            continue;
        };
        // The wall across the line from the slime to the unit, 3 dish
        // cells in front of the unit.
        let (dx, dy) = (unit.0 - c.0, unit.1 - c.1);
        let l = dx.hypot(dy).max(1e-3);
        let (ux, uy) = (dx / l, dy / l);
        let m = ((unit.0 - ux * 3.0 * s) / s, (unit.1 - uy * 3.0 * s) / s);
        let mut open = d.clone();
        open.p.surge_at = open.steps as u32 + 1 + 30;
        let mut walled = open.clone();
        // A cup: the back across the line, and two sides from its ends past
        // the unit, so the way in opens away from the slime.
        let (ax, ay, bx, by) = (m.0 - uy * wall, m.1 + ux * wall, m.0 + uy * wall, m.1 - ux * wall);
        let deep = 3.0 + depth;
        let mut cells = walled.build_wall_hp(ax, ay, bx, by, 1.5, 0.0);
        cells += walled.build_wall_hp(ax, ay, ax + ux * deep, ay + uy * deep, 1.5, 0.0);
        cells += walled.build_wall_hp(bx, by, bx + ux * deep, by + uy * deep, 1.5, 0.0);
        let mut field = walled.clone();
        field.units_field = units_field_for(&field.agar, field.w, field.h, &[unit.0 as u32 | (unit.1 as u32) << 16]);
        let none = vec![false; d.walkers.len()];
        let film = |name: &'static str| {
            let dir = look.as_deref().filter(|_| seed == seeds[0])?;
            std::fs::create_dir_all(dir).expect("look dir");
            Some((dir, name, ((c.0 + unit.0) / 2.0, (c.1 + unit.1) / 2.0), ((m.0 * s, m.1 * s), 0.0)))
        };
        let (o, _) = follow(&mut open, unit, watch * TPS, &none, film("open"));
        let (a, _) = follow(&mut walled, unit, watch * TPS, &none, film("straight"));
        let (b, _) = follow(&mut field, unit, watch * TPS, &none, film("field"));
        // Piled on the slime's side of the wall, within 4 dish cells of
        // its middle.
        let piled = |w: &BigDish| (0..w.walkers.len()).filter(|&k| ordinary(w, k) && { let (x, y) = (w.walkers[k].0 - m.0 * s, w.walkers[k].1 - m.1 * s); x.hypot(y) <= 4.0 * s && x * ux + y * uy < 0.0 }).count();
        println!("{seed} | ({:.0}, {:.0}), {cells} | {o}, {a}, {b} | {}, {}", unit.0, unit.1, piled(&walled), piled(&field));
        rows.push([o, a, b]);
    }
    let more = rows.iter().filter(|r| r[2] > r[1]).count();
    let less = rows.iter().filter(|r| r[2] < r[1]).count();
    let sum = |j: usize| rows.iter().map(|r| r[j]).sum::<usize>();
    println!("came in all: no wall {}, straight {}, field {}; field against straight: more {more}, less {less}, sign p {:.3}", sum(0), sum(1), sum(2), sign_p(more, less));
}
