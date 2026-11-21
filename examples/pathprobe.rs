//! **How squads find their way across a run planet** (pathfinding thread,
//! 2026-11-20): `cargo run --release --example pathprobe -- [depth=3]
//! [seed=1795038793] [n=40] [squads=4] [secs=180] [every=20] [out=FILE]
//! [walkers=8000] [orders=mfc]`.
//!
//! Plays a run planet (`levelgen::level(seed, depth)`, the map
//! `run=SEED:DEPTH` lands on) on the CPU big dish in squad mode, buys `n`
//! soldiers into `squads` squads and moves them the ways a player does:
//! squad 0 is steered with eight-way keys at a far crystal, the next pick
//! once it is there; the others are sent (right click) to a new crystal
//! every `every` game seconds, by turns a move, an attack-move and a
//! drain (`orders`: m, f and c, taken in turn); at 60 s squad 1 splits and both halves are sent on, at 120 s
//! they join again. Few walkers (`walkers`), so the soldiers live to walk.
//! Every step goes to `out` as `Game::path_rows` (read by
//! `Reports/pathfinding-2026-11-20/tools/paths.py`).
use slime_mould::mould::big::BigDish;
use slime_mould::mould::larder::Order;
use slime_mould::mould::larder_world::{race_params, Cmd, Game, Rules, World};
use slime_mould::mould::levelgen::level;

fn main() {
    let (mut depth, mut seed, mut n, mut nsq, mut secs, mut every, mut walkers) = (3u32, 1_795_038_793u64, 40usize, 4usize, 180.0f32, 20.0f32, 8000usize);
    let mut out = String::from("pathprobe.csv");
    let mut orders = String::from("mfc");
    for a in std::env::args().skip(1) {
        let (k, v) = a.split_once('=').expect("key=value");
        match k {
            "depth" => depth = v.parse().unwrap(),
            "seed" => seed = v.parse().unwrap(),
            "n" => n = v.parse().unwrap(),
            "squads" => nsq = v.parse().unwrap(),
            "secs" => secs = v.parse().unwrap(),
            "every" => every = v.parse().unwrap(),
            "walkers" => walkers = v.parse().unwrap(),
            "out" => out = v.to_string(),
            "orders" => orders = v.to_string(),
            _ => panic!("no key {k}"),
        }
    }
    let map = level(seed, depth);
    let d = BigDish::with_map(race_params(seed, 2, walkers), &map);
    let mut rules = Rules { squads: true, larder: false, seek: false, overrun: 0.0, max_squad: n, start_bank: 1e9, ..Rules::default() };
    rules.mass = Rules::mass_for(walkers, 2);
    let mut g = Game::new(d, rules, seed);
    while g.alive() < n && g.buy() {}
    let live: Vec<usize> = (0..g.soldiers.len()).filter(|&k| g.soldiers[k].alive).collect();
    for (i, &k) in live.iter().enumerate() {
        g.apply(Cmd::Join(k, (i * nsq / live.len()) as u8));
    }
    let s = g.world.scale();
    let crystals: Vec<(f32, f32)> = g.world.crystals().iter().map(|c| c.0).collect();
    let tps = g.rules.tps as usize;
    let steps = (secs * tps as f32) as usize;
    let mut f = std::io::BufWriter::new(std::fs::File::create(&out).expect("out"));
    // The ground as a picture (`OUT.pgm`, fine cells): 255 standable,
    // 120 shallow-free rock or deep water, for the track plots.
    {
        let (w, h) = g.world.size();
        let ok = g.standable_cells();
        let mut p = format!("P5 {w} {h} 255\n").into_bytes();
        p.extend(ok.iter().map(|&o| if o { 255u8 } else { 60 }));
        std::fs::write(format!("{out}.pgm"), p).expect("pgm");
    }
    let mut rng = seed ^ 0x9E37_79B9_7F4A_7C15;
    let mut next = |m: usize| {
        rng ^= rng << 13;
        rng ^= rng >> 7;
        rng ^= rng << 17;
        (rng % m as u64) as usize
    };
    // The steered squad's target: the crystal furthest from it first.
    let far = |g: &Game<BigDish>, q: usize, pick: &mut dyn FnMut(usize) -> usize| -> (f32, f32) {
        let mid = g.middle(q).unwrap_or((0.0, 0.0));
        let mut by: Vec<(f32, f32)> = crystals.clone();
        by.sort_by(|a, b| (b.0 - mid.0).hypot(b.1 - mid.1).total_cmp(&(a.0 - mid.0).hypot(a.1 - mid.1)));
        by[pick(by.len().div_ceil(2))]
    };
    let mut target = far(&g, 0, &mut next);
    println!("pathprobe: planet {depth} seed {seed}, {} soldiers in {nsq} squads, world {:?} at scale {s}, {} crystals", g.alive(), g.world.size(), crystals.len());
    let mut sent = 0usize;
    let mut split: Option<usize> = None;
    let mut held = 0.0f32;
    for step in 0..steps {
        let t = step as f32 / tps as f32;
        // Squad 0: eight-way steering at its target, as WASD gives.
        if let Some(mid) = g.middle(0) {
            let (dx, dy) = (target.0 - mid.0, target.1 - mid.1);
            if dx.hypot(dy) < 12.0 * s {
                println!("  t {t:.1}: squad 0 reached {:.0},{:.0}", target.0 / s, target.1 / s);
                target = far(&g, 0, &mut next);
            }
            // A key held until the way is more than 30 degrees off it (a
            // player doesn't change keys at every step on the 22.5-degree
            // line between two).
            let a = dy.atan2(dx);
            let off = (a - held).sin().atan2((a - held).cos()).abs();
            if g.squads[0].steer == (0.0, 0.0) || off > 30f32.to_radians() {
                held = (a / std::f32::consts::FRAC_PI_4).round() * std::f32::consts::FRAC_PI_4;
            }
            g.apply(Cmd::Steer(0, (held.cos(), held.sin())));
        }
        if step % (every as usize * tps) == 0 {
            for q in g.live_squads().into_iter().filter(|&q| q != 0) {
                let c = crystals[next(crystals.len())];
                let o = match orders.as_bytes()[sent % orders.len()] {
                    b'm' => Order::Move(c.0, c.1),
                    b'f' => Order::Fight(c.0, c.1),
                    _ => Order::Clear(c.0, c.1),
                };
                sent += 1;
                g.apply(Cmd::SquadOrder(q as u8, o));
                println!("  t {t:.1}: squad {q} ({} soldiers) sent {:?} to {:.0},{:.0}", g.members(q).len(), o, c.0 / s, c.1 / s);
            }
        }
        if step == 60 * tps && g.live_squads().contains(&1) {
            let before = g.live_squads();
            let p = g.middle(1).unwrap_or((0.0, 0.0));
            g.apply(Cmd::Split(1, (p.0 + 10.0 * s, p.1)));
            split = g.live_squads().into_iter().find(|q| !before.contains(q));
            println!("  t {t:.1}: squad 1 split -> {split:?}");
        }
        if step == 120 * tps {
            if let Some(b) = split {
                // Join: send both to one point, then merge once they touch.
                let c = crystals[next(crystals.len())];
                g.apply(Cmd::SquadOrder(1, Order::Move(c.0, c.1)));
                g.apply(Cmd::SquadOrder(b as u8, Order::Move(c.0, c.1)));
                println!("  t {t:.1}: squads 1 and {b} sent to {:.0},{:.0} to join", c.0 / s, c.1 / s);
            }
        }
        if step > 120 * tps && step % tps == 0 {
            // Only once they overlap, as the window's J asks.
            let near = split.and_then(|b| Some((g.middle(1)?, g.middle(b)?))).is_some_and(|(a, c)| (a.0 - c.0).hypot(a.1 - c.1) < 12.0 * s);
            if let Some(b) = split.filter(|_| near) {
                if g.apply(Cmd::Merge(1, b as u8)) {
                    println!("  t {t:.1}: squads 1 and {b} merged");
                    split = None;
                }
            }
        }
        g.step();
        g.path_rows(&mut f).expect("write");
        if g.end.is_some() {
            println!("  t {t:.1}: ended");
            break;
        }
    }
    println!("  end: {} alive, {} map waits", g.alive(), g.map_waits);
}
