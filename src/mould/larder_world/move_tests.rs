//! **How a squad moves, measured** (squad movement rework, 2026-11-16;
//! owner: "It is still clunky overall"). Scripted moves on the CPU world,
//! every soldier's walk step by step, and the numbers that say what a
//! player sees as clunky: soldiers that stop and start, snap their way,
//! walk back when the squad stops, or pile up. `measure_moves` prints them
//! (ignored, run by hand); the tests below keep the rework's gains.

use super::narrow_tests::{door_map, gaps};
use super::*;
use crate::mould::big::BigDish;
use crate::mould::map;

/// `m`'s world at the window's scale (2), squad mode, `n` soldiers in
/// squad 0, settled for 2 s at the landing.
pub(super) fn move_game(m: &map::Map, seed: u64, n: usize) -> Game<BigDish> {
    let d = BigDish::with_map(race_params(seed, 2, 500), m);
    let on = |k: &str| std::env::var(k).map_or(true, |v| v != "0");
    let mut g = Game::new(d, Rules { squads: true, max_squad: n.max(40), steer: on("MOVE_STEER"), funnel: on("MOVE_FUNNEL"), ..Rules::default() }, seed);
    g.bank = 1e9;
    while g.alive() < n {
        assert!(g.apply(Cmd::Buy));
    }
    for _ in 0..2 * g.rules.tps / g.rules.frame_ticks {
        g.step();
    }
    g
}

/// A scripted move: at each time (game seconds from the start), a command.
pub(super) type Script = Vec<(f32, Cmd)>;

/// Every step: squad 0's members' positions and whether it moves as one
/// (steered or sent).
pub(super) struct Track {
    pub t: Vec<f32>,
    pub pos: Vec<Vec<(f32, f32)>>,
    pub moving: Vec<bool>,
    pub steer: Vec<(f32, f32)>,
    /// Steps a member's walk waited on a walking map being built.
    pub waits: usize,
    /// The slowest step's movement turn (squads, elbow room, walks), ms.
    pub worst_ms: f64,
    pub mean_ms: f64,
    pub p95_ms: f64,
}

pub(super) fn play(g: &mut Game<BigDish>, script: &Script, secs: f32) -> Track {
    let dt = g.rules.frame_ticks as f32 / g.rules.tps as f32;
    let steps = (secs / dt) as usize;
    let mut tr = Track { t: Vec::new(), pos: Vec::new(), moving: Vec::new(), steer: Vec::new(), waits: 0, worst_ms: 0.0, mean_ms: 0.0, p95_ms: 0.0 };
    let mut next = set_scene(g, script);
    let m = g.members(0);
    let mut total = 0.0;
    let mut all = Vec::with_capacity(steps);
    for i in 0..steps {
        let t = i as f32 * dt;
        while next < script.len() && script[next].0 <= t + 1e-4 {
            g.apply(script[next].1);
            next += 1;
        }
        let (ms, _) = movement_cost(g, dt);
        tr.worst_ms = tr.worst_ms.max(ms);
        total += ms;
        all.push(ms);
        let w0 = g.map_waits;
        g.step();
        tr.waits += (g.map_waits - w0) as usize;
        tr.t.push(t + dt);
        tr.pos.push(m.iter().map(|&k| (g.soldiers[k].x, g.soldiers[k].y)).collect());
        let sq = g.squads[0];
        tr.moving.push(sq.steer != (0.0, 0.0) || sq.dest.is_some());
        tr.steer.push(sq.steer);
    }
    tr.mean_ms = total / steps.max(1) as f64;
    all.sort_by(f64::total_cmp);
    tr.p95_ms = all.get(all.len() * 95 / 100).copied().unwrap_or(0.0);
    tr
}

/// Plays a script's commands before time 0 (they set the scene: a split,
/// a formation), unrecorded; the index of the first command at 0 or after.
pub(super) fn set_scene(g: &mut Game<BigDish>, script: &Script) -> usize {
    let dt = g.rules.frame_ticks as f32 / g.rules.tps as f32;
    let mut next = 0;
    let mut t = script.first().map_or(0.0, |c| c.0.min(0.0));
    while t < -1e-4 {
        while next < script.len() && script[next].0 <= t + 1e-4 {
            g.apply(script[next].1);
            next += 1;
        }
        g.step();
        t += dt;
    }
    next
}

/// The movement part of a step, timed on a copy of the game's movers (the
/// real step runs it again): ms, and how many walks and anchor steps
/// waited on a walking map (`Game::map_waits`) in the real step. The
/// walking maps it asks for are taken back too, or the real step's
/// `pump_maps` starts them a step early and fewer walks wait (review,
/// 2026-11-16).
fn movement_cost(g: &mut Game<BigDish>, dt: f32) -> (f64, usize) {
    let asked = g.paths.asked.clone();
    let saved = (g.squads.clone(), g.columns.clone(), g.soldiers.clone(), g.ranks.clone(), g.ranked, g.map_waits);
    let t0 = std::time::Instant::now();
    g.solid = g.solid_now();
    g.squad_turn(dt);
    if g.rules.squads && g.rules.steer {
        g.steer_all(dt);
    } else {
        g.elbow_room(dt);
        for k in 0..g.soldiers.len() {
            if g.soldiers[k].alive {
                g.walk(k, dt);
            }
        }
    }
    let ms = t0.elapsed().as_secs_f64() * 1e3;
    let before;
    (g.squads, g.columns, g.soldiers, g.ranks, g.ranked, before) = saved;
    g.map_waits = before;
    g.paths.asked = asked;
    (ms, 0)
}

/// What a track says, over the steps it moves as one (`Track::moving`)
/// unless said otherwise.
#[derive(Debug, Default)]
pub(super) struct Feel {
    /// Share of soldier-steps a member barely moved (under a quarter of a
    /// full walk step) while its squad moved: stop-and-go.
    pub still: f32,
    /// Mean change of a member's step from the one before, in full walk
    /// steps: jerk (a smooth walk is near 0, a stop or a snap 1).
    pub jerk: f32,
    /// Mean turn of a walking member's heading step to step, degrees.
    pub turn: f32,
    /// The squad's middle's speed over a soldier's full speed.
    pub pace: f32,
    /// Root mean square distance of the members from the middle, dish cells.
    pub spread: f32,
    /// After the squad stopped moving as one: dish cells a member walked
    /// back against the last way it went, mean.
    pub back: f32,
    /// Seconds from the squad stopping to every member standing still.
    pub settle: f32,
    /// Seconds from the squad starting to move as one to its stopping, if
    /// it did (without it, `back` and `settle` read 0: it never stopped;
    /// review, 2026-11-16).
    pub arrive: Option<f32>,
    /// Seconds from the squad starting to move until its middle first
    /// walks at half a soldier's speed: how soon a key or a click shows.
    pub start: Option<f32>,
}

pub(super) fn feel(g: &Game<BigDish>, tr: &Track) -> Feel {
    let sc = g.s();
    let dt = g.rules.frame_ticks as f32 / g.rules.tps as f32;
    let full = g.rules.speed * sc * dt;
    let n = tr.pos[0].len();
    let mut f = Feel::default();
    let (mut ss, mut sn, mut jn, mut tn, mut tc, mut pn, mut sp) = (0.0f32, 0.0f32, 0.0f32, 0.0f32, 0.0f32, 0.0f32, 0.0f32);
    for i in 2..tr.pos.len() {
        if !tr.moving[i] || !tr.moving[i - 1] {
            continue;
        }
        let mid = |p: &[(f32, f32)]| {
            let (x, y) = p.iter().fold((0.0, 0.0), |a, q| (a.0 + q.0, a.1 + q.1));
            (x / n as f32, y / n as f32)
        };
        let (m0, m1) = (mid(&tr.pos[i - 1]), mid(&tr.pos[i]));
        f.pace += (m1.0 - m0.0).hypot(m1.1 - m0.1) / full;
        pn += 1.0;
        sp += (tr.pos[i].iter().map(|q| (q.0 - m1.0).powi(2) + (q.1 - m1.1).powi(2)).sum::<f32>() / n as f32).sqrt() / sc;
        for k in 0..n {
            let v0 = (tr.pos[i - 1][k].0 - tr.pos[i - 2][k].0, tr.pos[i - 1][k].1 - tr.pos[i - 2][k].1);
            let v1 = (tr.pos[i][k].0 - tr.pos[i - 1][k].0, tr.pos[i][k].1 - tr.pos[i - 1][k].1);
            let (l0, l1) = (v0.0.hypot(v0.1), v1.0.hypot(v1.1));
            ss += (l1 < 0.25 * full) as u32 as f32;
            sn += 1.0;
            f.jerk += (v1.0 - v0.0).hypot(v1.1 - v0.1) / full;
            jn += 1.0;
            if l0 > 0.25 * full && l1 > 0.25 * full {
                let c = ((v0.0 * v1.0 + v0.1 * v1.1) / (l0 * l1)).clamp(-1.0, 1.0);
                tc += c.acos().to_degrees();
                tn += 1.0;
            }
        }
    }
    // The first step moving as one, and the first its middle walks at half
    // a soldier's speed after it.
    let first = tr.moving.iter().position(|&m| m);
    if let Some(a) = first {
        let mid = |p: &[(f32, f32)]| {
            let (x, y) = p.iter().fold((0.0, 0.0), |s, q| (s.0 + q.0, s.1 + q.1));
            (x / n as f32, y / n as f32)
        };
        f.start = (a.max(1)..tr.pos.len()).find(|&i| {
            let (m0, m1) = (mid(&tr.pos[i - 1]), mid(&tr.pos[i]));
            (m1.0 - m0.0).hypot(m1.1 - m0.1) >= 0.5 * full
        }).map(|i| (i - a) as f32 * dt);
    }
    f.still = ss / sn.max(1.0);
    f.jerk /= jn.max(1.0);
    f.turn = tc / tn.max(1.0);
    f.pace /= pn.max(1.0);
    f.spread = sp / pn.max(1.0);
    // The first stop after moving: walking back and settling.
    if let Some(stop) = (1..tr.moving.len()).find(|&i| tr.moving[i - 1] && !tr.moving[i] || tr.steer[i - 1] != (0.0, 0.0) && tr.steer[i] == (0.0, 0.0)) {
        let dir = {
            let a = &tr.pos[stop.saturating_sub(8)];
            let b = &tr.pos[stop];
            let (x, y) = a.iter().zip(b).fold((0.0, 0.0), |s, (p, q)| (s.0 + q.0 - p.0, s.1 + q.1 - p.1));
            let l = x.hypot(y).max(1e-6);
            (x / l, y / l)
        };
        let mut back = 0.0;
        for i in stop + 1..tr.pos.len() {
            for k in 0..n {
                let v = (tr.pos[i][k].0 - tr.pos[i - 1][k].0, tr.pos[i][k].1 - tr.pos[i - 1][k].1);
                back += (-(v.0 * dir.0 + v.1 * dir.1)).max(0.0);
            }
        }
        f.back = back / n as f32 / sc;
        let quiet = |i: usize| (0..n).all(|k| (tr.pos[i][k].0 - tr.pos[i - 1][k].0).hypot(tr.pos[i][k].1 - tr.pos[i - 1][k].1) < 0.05 * full);
        let last_busy = (stop + 1..tr.pos.len()).rev().find(|&i| !quiet(i)).unwrap_or(stop);
        f.settle = (last_busy - stop) as f32 * dt;
        f.arrive = first.map(|a| (stop - a) as f32 * dt);
    }
    f
}

/// A move measured: name, map, its script (from the game at the start, in
/// fine cells) and how long it runs, seconds.
pub(super) type Move = (&'static str, map::Map, Box<dyn Fn(&Game<BigDish>) -> Script>, f32);

/// The moves measured (`Move`). The
/// planets are run seed `seed`'s (the first measures had both seeds on run
/// seed 1's planets, one sample; review, 2026-11-16).
pub(super) fn moves(seed: u64) -> Vec<Move> {
    let open = door_map(0.0, 0.0, 0.0);
    let mut v: Vec<Move> = Vec::new();
    // WASD: east, then north (a quarter turn), then west (about turn),
    // then let go.
    v.push(("steer", open.clone(), Box::new(|_| vec![(0.0, Cmd::Steer(0, (1.0, 0.0))), (4.0, Cmd::Steer(0, (0.0, -1.0))), (7.0, Cmd::Steer(0, (-1.0, 0.0))), (10.0, Cmd::Steer(0, (0.0, 0.0)))]), 15.0));
    // A right click 50 cells off on open ground, as a blob and in the
    // other formations.
    for (name, shape) in [("send", Formation::Blob), ("line", Formation::Line), ("wedge", Formation::Wedge), ("ring", Formation::Ring)] {
        v.push((name, open.clone(), Box::new(move |g| {
            let (sc, at) = (g.s(), g.middle(0).unwrap());
            vec![(-2.0, Cmd::Formation(0, shape)), (0.0, Cmd::SquadOrder(0, Order::Move(at.0 + 50.0 * sc, at.1 + 10.0 * sc)))]
        }), 15.0));
    }
    // Sent 50 cells east, then after 3 s back where it began.
    v.push(("reverse", open.clone(), Box::new(|g| {
        let (sc, at) = (g.s(), g.middle(0).unwrap());
        vec![(0.0, Cmd::SquadOrder(0, Order::Move(at.0 + 50.0 * sc, at.1))), (3.0, Cmd::SquadOrder(0, Order::Move(at.0, at.1)))]
    }), 15.0));
    // Two squads through each other over the 8-cell bridge: the squad
    // splits, the half nearer the far bank crosses first (30 s, unrecorded),
    // then each is sent to the other's side. Squad 0 is measured.
    v.push(("cross", map::river(false, true), Box::new(|g| {
        let (sc, at) = (g.s(), g.middle(0).unwrap());
        let far = (175.0 * sc, 114.0 * sc);
        vec![(-30.0, Cmd::Split(0, far)), (-30.0, Cmd::SquadOrder(1, Order::Move(far.0, far.1))), (0.0, Cmd::SquadOrder(0, Order::Move(far.0, far.1))), (0.0, Cmd::SquadOrder(1, Order::Move(at.0, at.1)))]
    }), 30.0));
    for (name, m, goal) in gaps() {
        let name: &'static str = match name {
            "bridge8" => "bridge8",
            "door6long" => "door6long",
            _ => "door8slant",
        };
        v.push((name, m, Box::new(move |g| vec![(0.0, Cmd::SquadOrder(0, Order::Move(goal.0 * g.s(), goal.1 * g.s())))]), 22.0));
    }
    // A run's planets: sent from the landing to the crystal furthest from
    // it within 120 dish cells as the crow flies (the walk is longer),
    // round whatever rock is in the way; the first planet and the last
    // (6x the area, where walking maps cost most).
    for (name, depth) in [("planet1", 1u32), ("planet8", 8)] {
        v.push((name, crate::mould::levelgen::level(seed, depth), Box::new(far_crystal), 40.0));
    }
    v
}

/// A move order to the crystal furthest from squad 0 within 120 dish
/// cells, just off its rim.
fn far_crystal(g: &Game<BigDish>) -> Script {
    let (sc, at) = (g.s(), g.middle(0).unwrap());
    let d = |c: (f32, f32)| (c.0 - at.0).hypot(c.1 - at.1);
    let c = g.world.crystals().into_iter().map(|c| c.0).filter(|&c| d(c) <= 120.0 * sc).max_by(|a, b| d(*a).total_cmp(&d(*b))).unwrap_or(at);
    vec![(0.0, Cmd::SquadOrder(0, Order::Move(c.0, c.1)))]
}

/// Prints the feel of every move for squads of `MOVE_NS` (default 12, 40,
/// 100, 200) on seeds 1-2: `MOVE=steer,send cargo test --release --lib
/// measure_moves -- --ignored --nocapture`.
#[test]
#[ignore]
fn measure_moves() {
    let pick = std::env::var("MOVE").ok();
    let ns: Vec<usize> = std::env::var("MOVE_NS").map(|v| v.split(',').map(|x| x.parse().unwrap()).collect()).unwrap_or(vec![12, 40, 100, 200]);
    let seeds: Vec<u64> = std::env::var("MOVE_SEEDS").map(|v| v.split(',').map(|x| x.parse().unwrap()).collect()).unwrap_or(vec![1, 2]);
    let names: Vec<&str> = moves(1).iter().map(|v| v.0).collect();
    for name in names {
        if pick.as_ref().is_some_and(|p| !p.split(',').any(|x| x == name)) {
            continue;
        }
        for &n in &ns {
            for &seed in &seeds {
                let (_, m, script, secs) = moves(seed).into_iter().find(|v| v.0 == name).unwrap();
                let mut g = move_game(&m, seed, n);
                let s = script(&g);
                let tr = play(&mut g, &s, secs);
                let f = feel(&g, &tr);
                println!(
                    "{name:>10} n={n:>3} seed={seed} still={:.2} jerk={:.2} turn={:>5.1} pace={:.2} spread={:>5.1} back={:>5.2} settle={:>4.1}s arrive={} start={} waits={} move ms mean={:.3} p95={:.2} worst={:.2}",
                    f.still,
                    f.jerk,
                    f.turn,
                    f.pace,
                    f.spread,
                    f.back,
                    f.settle,
                    f.arrive.map_or("never".into(), |a| format!("{a:>4.1}s")),
                    f.start.map_or("never".into(), |a| format!("{a:.2}s")),
                    tr.waits,
                    tr.mean_ms,
                    tr.p95_ms,
                    tr.worst_ms
                );
            }
        }
    }
}

/// Writes squad 0's walk for one move as CSV (t, k, x, y in dish cells) to
/// `$MOVE_CSV`, with the ground beside it (`.ground`), for plots:
/// `MOVE=steer MOVE_N=40`.
#[test]
#[ignore]
fn dump_move() {
    let which = std::env::var("MOVE").unwrap_or("steer".into());
    let n: usize = std::env::var("MOVE_N").ok().and_then(|v| v.parse().ok()).unwrap_or(40);
    let (_, m, script, secs) = moves(1).into_iter().find(|v| v.0 == which).unwrap();
    let mut g = move_game(&m, 1, n);
    let sc = g.s();
    let sl = script(&g);
    let tr = play(&mut g, &sl, secs);
    let mut s = String::from("t,k,x,y\n");
    for (t, p) in tr.t.iter().zip(&tr.pos) {
        for (k, q) in p.iter().enumerate() {
            s += &format!("{t:.3},{k},{:.2},{:.2}\n", q.0 / sc, q.1 / sc);
        }
    }
    let path = std::env::var("MOVE_CSV").unwrap();
    std::fs::write(&path, s).unwrap();
}

/// Prints squad 0's anchor, middle and stuck members each game second of
/// one move (`MOVE`, `MOVE_N`), for finding where a squad sticks.
#[test]
#[ignore]
fn trace_move() {
    let which = std::env::var("MOVE").unwrap_or("planet1".into());
    let n: usize = std::env::var("MOVE_N").ok().and_then(|v| v.parse().ok()).unwrap_or(40);
    let (_, m, script, secs) = moves(1).into_iter().find(|v| v.0 == which).unwrap();
    let mut g = move_game(&m, 1, n);
    let sc = g.s();
    let sl = script(&g);
    let mut next = set_scene(&mut g, &sl);
    let dt = g.rules.frame_ticks as f32 / g.rules.tps as f32;
    for i in 0..(secs / dt) as usize {
        while next < sl.len() && sl[next].0 <= i as f32 * dt + 1e-4 {
            g.apply(sl[next].1);
            next += 1;
        }
        let before: Vec<(f32, f32)> = g.soldiers.iter().map(|s| (s.x, s.y)).collect();
        g.step();
        if i % 15 == 0 {
            let sq = g.squads[0];
            let mid = g.middle(0).unwrap();
            let still = g.members(0).iter().filter(|&&k| (g.soldiers[k].x - before[k].0).hypot(g.soldiers[k].y - before[k].1) < 0.3).count();
            let k0 = g.members(0)[0];
            println!(
                "t={:.0} anchor={:?} mid=({:.0},{:.0}) dest={:?} map={:?} vel=({:.1},{:.1}) still={still}/{} waits={} k0=({:.0},{:.0}) v=({:.1},{:.1}) goal={:?}",
                i as f32 * dt,
                sq.anchor.map(|a| ((a.0 / sc) as i32, (a.1 / sc) as i32)),
                mid.0 / sc,
                mid.1 / sc,
                sq.dest.map(|a| ((a.0 / sc) as i32, (a.1 / sc) as i32)),
                sq.map,
                sq.vel.0,
                sq.vel.1,
                g.members(0).len(),
                g.map_waits,
                g.soldiers[k0].x / sc,
                g.soldiers[k0].y / sc,
                g.soldiers[k0].vx,
                g.soldiers[k0].vy,
                g.goal(k0).map(|a| ((a.0 / sc) as i32, (a.1 / sc) as i32))
            );
        }
    }
}

/// After a move has ended: which members still walk, and toward what
/// (`MOVE`, `MOVE_N`), for finding what keeps a squad from settling.
#[test]
#[ignore]
fn trace_settle() {
    let which = std::env::var("MOVE").unwrap_or("planet8".into());
    let n: usize = std::env::var("MOVE_N").ok().and_then(|v| v.parse().ok()).unwrap_or(40);
    let (_, m, script, secs) = moves(1).into_iter().find(|v| v.0 == which).unwrap();
    let mut g = move_game(&m, 1, n);
    let sc = g.s();
    let sl = script(&g);
    let mut next = set_scene(&mut g, &sl);
    let dt = g.rules.frame_ticks as f32 / g.rules.tps as f32;
    let full = g.rules.speed * sc;
    for i in 0..(secs / dt) as usize {
        while next < sl.len() && sl[next].0 <= i as f32 * dt + 1e-4 {
            g.apply(sl[next].1);
            next += 1;
        }
        let before: Vec<(f32, f32)> = g.soldiers.iter().map(|s| (s.x, s.y)).collect();
        g.step();
        if g.squads[0].dest.is_none() && i % 30 == 0 {
            let movers: Vec<String> = g
                .members(0)
                .iter()
                .filter(|&&k| (g.soldiers[k].x - before[k].0).hypot(g.soldiers[k].y - before[k].1) > 0.05 * full * dt)
                .map(|&k| {
                    let s = g.soldiers[k];
                    let goal = g.goal(k).unwrap_or((s.x, s.y));
                    let o = g.place(k);
                    format!("k{k} at ({:.1},{:.1}) v=({:.1},{:.1}) goal d={:.1} slot|o|={:.1} order={:?} line={}", s.x / sc, s.y / sc, s.vx, s.vy, (goal.0 - s.x).hypot(goal.1 - s.y) / sc, o.0.hypot(o.1), s.order, g.walk_line((s.x, s.y), goal))
                })
                .collect();
            println!("t={:.1} movers {}: {}", i as f32 * dt, movers.len(), movers.iter().take(4).cloned().collect::<Vec<_>>().join(" | "));
        }
    }
}

/// One move's feel: squad `n` on seed `seed` (`moves`).
fn feel_of(name: &str, seed: u64, n: usize) -> (Feel, Track) {
    let (_, m, script, secs) = moves(seed).into_iter().find(|v| v.0 == name).unwrap();
    let mut g = move_game(&m, seed, n);
    let s = script(&g);
    let tr = play(&mut g, &s, secs);
    (feel(&g, &tr), tr)
}

/// **Steered with WASD, a squad walks in step and stops in its places**
/// (squad movement rework, 2026-11-16). 40 soldiers east, north, west, let
/// go: before, every member dashed and stood (jerk 0.04 but a walk drawn
/// in jumps), and on letting go they ran past their places and walked back
/// 0.36 cells each; now they stop where they are (0.00) within 0.3 s.
#[test]
fn a_steered_squad_walks_in_step_and_stops_in_its_places() {
    let (f, _) = feel_of("steer", 1, 40);
    assert!(f.jerk < 0.08 && f.still < 0.05, "{f:?}");
    assert!(f.back < 0.05 && f.settle < 1.0, "{f:?}");
}

/// **Two squads cross on a bridge** (review, 2026-11-16): 20 each way over
/// the 8-cell bridge. Braking alone they stood face to face for 14 s;
/// stepping aside they pass, the measured squad over in 17.5 s.
#[test]
fn two_squads_pass_each_other_on_a_bridge() {
    let (f, _) = feel_of("cross", 1, 40);
    assert!(f.arrive.is_some_and(|a| a < 24.0), "{f:?}");
}

/// **A squad crosses planet 8's caves to a crystal and settles** (squad
/// movement rework, 2026-11-16): 40 soldiers, run seed 1's eighth planet.
/// Before the fix to queuing, two of them pushed each other for 20 s after
/// arriving.
#[test]
fn a_squad_crosses_planet_8_and_settles() {
    let (f, tr) = feel_of("planet8", 1, 40);
    assert!(f.arrive.is_some() && f.settle < 10.0, "{f:?}");
    assert!(tr.waits < 200, "map waits {}", tr.waits);
}
