//! Squads through narrow gaps (owner, 2026-11-15: "it gets awkward moving
//! with large squads through narrow areas").

use super::*;
use crate::mould::big::BigDish;
use crate::mould::map;

/// A test map: the river map's crystals, landing (dish 75, 125) and mould,
/// with a band of rock from dish x 120 to `120 + long` and one door
/// through it `wide` dish cells wide centred on dish y `door`.
pub(super) fn door_map(wide: f32, long: f32, door: f32) -> map::Map {
    let mut m = map::river(false, false);
    let k = map::MAP_SCALE;
    for y in 0..m.h {
        for x in 0..m.w {
            let (dx, dy) = ((x as f32 + 0.5) / k, (y as f32 + 0.5) / k);
            m.ground[y * m.w + x] = if (120.0..120.0 + long).contains(&dx) && (dy - door).abs() > wide / 2.0 { map::G_ROCK } else { 0 };
        }
    }
    m
}

/// The narrow places the tests and films cross: (name, map, goal in dish
/// cells).
pub(super) fn gaps() -> Vec<(&'static str, map::Map, (f32, f32))> {
    vec![
        ("bridge8", map::river(false, true), (175.0, 114.0)),
        ("door6long", door_map(6.0, 30.0, 114.0), (190.0, 114.0)),
        ("door8slant", door_map(8.0, 10.0, 95.0), (175.0, 130.0)),
    ]
}

/// The river test map's world (bridge 8 dish cells wide at dish y 110-118,
/// x 120-136), squad mode, `n` soldiers in squad 0, no slime near.
pub(super) fn bridge_game(seed: u64, n: usize) -> Game<BigDish> {
    gap_game(&map::river(false, true), seed, n)
}

/// `m`'s world in the squad mode, `n` soldiers in squad 0.
pub(super) fn gap_game(m: &map::Map, seed: u64, n: usize) -> Game<BigDish> {
    let d = BigDish::with_map(race_params(seed, 1, 500), m);
    let funnel = std::env::var("NARROW_FUNNEL").map_or(true, |v| v != "0");
    let mut g = Game::new(d, Rules { squads: true, funnel, max_squad: 64, ..Rules::default() }, seed);
    g.bank = 1e6;
    while g.alive() < n {
        assert!(g.apply(Cmd::Buy));
    }
    for _ in 0..2 * g.rules.tps / g.rules.frame_ticks {
        g.step();
    }
    g
}

/// Per step: (time, positions of squad 0).
pub(super) fn cross(g: &mut Game<BigDish>, to: (f32, f32), secs: f32) -> Vec<(f32, Vec<(f32, f32)>)> {
    g.apply(Cmd::SquadOrder(0, Order::Move(to.0, to.1)));
    let mut out = Vec::new();
    let t0 = g.time();
    let steps = (secs * g.rules.tps as f32 / g.rules.frame_ticks as f32) as usize;
    for _ in 0..steps {
        g.step();
        let m = g.members(0);
        out.push((g.time() - t0, m.iter().map(|&k| (g.soldiers[k].x, g.soldiers[k].y)).collect()));
    }
    out
}

#[test]
#[ignore]
fn measure_bridge() {
    for (name, map, goal) in gaps().into_iter().filter(|g| std::env::var("NARROW_MAP").map_or(true, |m| m == g.0)) {
    for n in std::env::var("NARROW_NS").map(|v| v.split(',').map(|x| x.parse().unwrap()).collect::<Vec<usize>>()).unwrap_or(vec![8, 16, 24, 32]) {
        for seed in 1..=2u64 {
            let mut g = gap_game(&map, seed, n);
            let sc = g.s();
            let m0 = g.middle(0).unwrap();
            let to = (goal.0 * sc, goal.1 * sc);
            let tr = cross(&mut g, to, 60.0);
            // All past the river's far bank (dish x 137).
            let over = tr.iter().find(|(_, p)| p.iter().all(|q| q.0 > 152.0 * sc)).map(|t| t.0);
            // All within 3 gaps of the goal's blob.
            let settled = tr.iter().find(|(_, p)| p.iter().all(|q| (q.0 - to.0).hypot(q.1 - to.1) < (g.gap() * (n as f32).sqrt() + 2.0) * sc)).map(|t| t.0);
            // Walking: total path, total backward (away from goal x) path.
            let (mut walked, mut back) = (0.0, 0.0);
            for w in tr.windows(2) {
                for (a, b) in w[0].1.iter().zip(&w[1].1) {
                    walked += (b.0 - a.0).hypot(b.1 - a.1);
                    if b.0 < a.0 { back += a.0 - b.0; }
                }
            }
            let mut stuck = 0usize; // soldier-steps not moving while not yet settled
            for w in tr.windows(2) {
                if settled.is_some_and(|s| w[1].0 > s) { break; }
                for (a, b) in w[0].1.iter().zip(&w[1].1) {
                    if (b.0 - a.0).hypot(b.1 - a.1) < 1e-3 * sc { stuck += 1; }
                }
            }
            println!("{name} n={n} seed={seed} start=({:.0},{:.0}) over={over:?} settled={settled:?} walked/soldier={:.0} back/soldier={:.1} still-steps/soldier={:.0} jam={:.2} crowd={}", m0.0 / sc, m0.1 / sc, walked / n as f32 / sc, back / n as f32 / sc, stuck as f32 / n as f32, jam(&tr, g.gap() * sc), crowd(&tr, g.gap() * sc));
        }
    }
    }
}

/// Worst crowding at the gap: the most soldier pairs closer than half a
/// gap at once among those within 12 dish cells of dish x 128, per soldier.
pub(super) fn jam(tr: &[(f32, Vec<(f32, f32)>)], gap: f32) -> f32 {
    let sc = gap / 2.52;
    tr.iter().map(|(_, p)| {
        let n = p.len();
        let p: Vec<(f32, f32)> = p.iter().copied().filter(|q| (q.0 / sc - 128.0).abs() < 20.0).collect();
        let mut c = 0;
        for i in 0..p.len() { for j in i + 1..p.len() { if (p[i].0 - p[j].0).hypot(p[i].1 - p[j].1) < 0.5 * gap { c += 1; } } }
        c as f32 / n as f32
    }).fold(0.0, f32::max)
}

/// Writes squad 0's walk over the bridge as CSV (t, k, x, y in dish cells)
/// to `$NARROW_CSV`, for plotting.
#[test]
#[ignore]
fn dump_bridge() {
    let n: usize = std::env::var("NARROW_N").ok().and_then(|v| v.parse().ok()).unwrap_or(24);
    let which = std::env::var("NARROW_MAP").unwrap_or("bridge8".into());
    let (_, map, goal) = gaps().into_iter().find(|g| g.0 == which).unwrap();
    let mut g = gap_game(&map, 1, n);
    let sc = g.s();
    let to = (goal.0 * sc, goal.1 * sc);
    let tr = cross(&mut g, to, 22.0);
    let mut s = String::from("t,k,x,y\n");
    for (t, p) in tr.iter().step_by(2) {
        for (k, q) in p.iter().enumerate() {
            s += &format!("{t:.3},{k},{:.2},{:.2}\n", q.0 / sc, q.1 / sc);
        }
    }
    let path = std::env::var("NARROW_CSV").unwrap();
    std::fs::write(&path, s).unwrap();
    // Standable dish cells, one row of 0/1 a line.
    let (w, h) = g.world.size();
    let mut m = String::new();
    for y in 0..h {
        for x in 0..w {
            m.push(if g.standable(x as f32 + 0.5, y as f32 + 0.5) { '1' } else { '0' });
        }
        m.push('\n');
    }
    std::fs::write(format!("{path}.ground"), m).unwrap();
}

/// Saves the narrow places as map files (`map=DIR/NAME` in the window) to
/// `$NARROW_MAPS`.
#[test]
#[ignore]
fn save_gap_maps() {
    let dir = std::path::PathBuf::from(std::env::var("NARROW_MAPS").unwrap());
    for (name, m, _) in gaps() {
        m.save(&dir, name).unwrap();
    }
}

/// Worst knot at the gap: the most other soldiers within one gap of any
/// soldier within 20 dish cells of dish x 128.
pub(super) fn crowd(tr: &[(f32, Vec<(f32, f32)>)], gap: f32) -> usize {
    let sc = gap / 2.52;
    tr.iter().map(|(_, p)| {
        p.iter().filter(|q| (q.0 / sc - 128.0).abs() < 20.0).map(|a| p.iter().filter(|b| (b.0 - a.0).hypot(b.1 - a.1) < gap).count() - 1).max().unwrap_or(0)
    }).max().unwrap_or(0)
}

/// Furthest member of squad 0 from its middle, dish cells.
fn spread(g: &Game<BigDish>) -> f32 {
    let m = g.middle(0).unwrap();
    g.members(0).iter().map(|&k| (g.soldiers[k].x - m.0).hypot(g.soldiers[k].y - m.1)).fold(0.0, f32::max) / g.s()
}

/// **A big squad funnels through a narrow door** (owner, 2026-11-15). 24
/// soldiers sent through a door 6 cells wide and 30 long: before, the
/// formation's places in the rock all fell back to the anchor and the
/// squad went through as one knot (10 others within a gap of one soldier
/// at worst, seeds 1 and 2: 10 and 9); funnelling, it queues (6 and 6), is
/// all through within 16 s and stands as a blob again past the door.
///
/// With the squad movement rework's walk (`Rules::steer`, 2026-11-16) a
/// squad queues without the column too (worst knot 6 off, 7 on, seed 1),
/// since soldiers brake behind slower ones instead of shoving, but it
/// stalls in the door: the column is what gets it through first. The old
/// walk's comparison (fewer knots with the column) is kept for it.
#[test]
fn a_big_squad_funnels_through_a_door_and_forms_up_after() {
    for steer in [false, true] {
        for seed in 1..=2 {
            let mut worst = [0usize; 2];
            let mut after = [0.0f32; 2];
            let mut over = [None; 2];
            for (i, funnel) in [false, true].into_iter().enumerate() {
                let (_, m, goal) = gaps().into_iter().find(|g| g.0 == "door6long").unwrap();
                let mut g = gap_game(&m, seed, 24);
                g.rules.funnel = funnel;
                g.rules.steer = steer;
                let sc = g.s();
                let tr = cross(&mut g, (goal.0 * sc, goal.1 * sc), 20.0);
                worst[i] = crowd(&tr, g.gap() * sc);
                after[i] = spread(&g);
                over[i] = tr.iter().find(|(_, p)| p.iter().all(|q| q.0 > 152.0 * sc)).map(|t| t.0);
                if funnel {
                    assert!(over[i].is_some_and(|t| t < 16.0), "steer {steer} seed {seed}: through at {:?}", over[i]);
                    assert!(!g.columns[0].on && g.squads[0].dest.is_none(), "steer {steer} seed {seed}: arrived and out of the column");
                }
            }
            assert!(worst[1] <= 7, "steer {steer} seed {seed}: worst knot {worst:?} (off, on)");
            if steer {
                assert!(over[0].is_none_or(|t| over[1].is_some_and(|u| u < t)), "seed {seed}: through at {over:?} (off, on)");
            } else {
                assert!(worst[1] < worst[0], "seed {seed}: worst knot {worst:?} (off, on)");
            }
            assert!(after[1] < after[0] + 1.0, "steer {steer} seed {seed}: stands {after:?} cells wide after (off, on)");
        }
    }
}

/// **A column files through** (the Formation page, owner 2026-11-21
/// 05:04Z: "good formation options"). The same 24 soldiers through the long
/// 6-cell door and over the 8-cell bridge in `Formation::Column`: two
/// abreast all the way, so they are all through about as soon as the blob
/// that funnels, with no worse a knot, and stand as a column past it.
#[test]
fn a_column_files_through_a_door_and_a_bridge() {
    for name in ["door6long", "bridge8"] {
        for seed in 1..=2 {
            let mut over = [None; 2];
            let mut worst = [0usize; 2];
            for (i, f) in [Formation::Blob, Formation::Column].into_iter().enumerate() {
                let (_, m, goal) = gaps().into_iter().find(|g| g.0 == name).unwrap();
                let mut g = gap_game(&m, seed, 24);
                g.apply(Cmd::Formation(0, f));
                let sc = g.s();
                let tr = cross(&mut g, (goal.0 * sc, goal.1 * sc), 24.0);
                worst[i] = crowd(&tr, g.gap() * sc);
                // Past dish x 136 (the bridge's end, the long door's middle):
                // a column at rest stands 30 cells long back from its goal,
                // so its tail is not past the door's far end.
                over[i] = tr.iter().find(|(_, p)| p.iter().all(|q| q.0 > 136.0 * sc)).map(|t| t.0);
            }
            println!("{name} seed {seed}: all through at {over:?} s, worst knot {worst:?} (blob, column)");
            assert!(over[1].is_some_and(|t| over[0].is_none_or(|u| t <= u + 2.0)), "{name} seed {seed}: through at {over:?} (blob, column)");
            assert!(worst[1] <= worst[0].max(4), "{name} seed {seed}: worst knot {worst:?} (blob, column)");
        }
    }
}

/// A squad steered (WASD) at a bridge from off to one side crosses it
/// funnelling, and one steered along the river bank does not funnel for
/// good: open ground on its far side is room enough.
#[test]
fn a_steered_squad_crosses_a_bridge_and_open_ground_stays_a_blob() {
    let mut g = bridge_game(3, 24);
    let sc = g.s();
    // Toward the bridge's mouth, then straight on east.
    g.apply(Cmd::Steer(0, (1.0, -0.2)));
    let mut funnelled = false;
    for _ in 0..(14.0 * g.rules.tps as f32 / g.rules.frame_ticks as f32) as usize {
        let m = g.middle(0).unwrap();
        if m.0 > 110.0 * sc {
            g.apply(Cmd::Steer(0, (1.0, 0.0)));
        }
        g.step();
        funnelled |= g.columns[0].on;
    }
    let m = g.middle(0).unwrap();
    assert!(funnelled, "funnelled at the bridge");
    assert!(m.0 > 140.0 * sc, "over the bridge: middle at x {}", m.0 / sc);
    // Steered south along the far bank, the river at its side.
    g.apply(Cmd::Steer(0, (0.0, 1.0)));
    let mut on = 0;
    for _ in 0..(5.0 * g.rules.tps as f32 / g.rules.frame_ticks as f32) as usize {
        g.step();
        on += g.columns[0].on as usize;
    }
    let steps = (5.0 * g.rules.tps as f32 / g.rules.frame_ticks as f32) as usize;
    assert!(on < steps / 2, "funnelled {on} of {steps} steps along an open bank");
}

/// Frame cost: the squads' turn (`Game::squad_turn`, which funnels) for
/// 32 soldiers crossing the long door, funnel on and off, mean per step.
#[test]
#[ignore]
fn time_funnel() {
    let (_, m, goal) = gaps().into_iter().find(|g| g.0 == "door6long").unwrap();
    for funnel in [false, true, false, true] {
        let mut g = gap_game(&m, 1, 32);
        g.rules.funnel = funnel;
        let sc = g.s();
        g.apply(Cmd::SquadOrder(0, Order::Move(goal.0 * sc, goal.1 * sc)));
        let dt = g.rules.frame_ticks as f32 / g.rules.tps as f32;
        let (mut t, mut n, mut on) = (0.0f64, 0, 0);
        for _ in 0..(16.0 / dt) as usize {
            // The turn alone on a copy of the step's start: the real step
            // runs it again, so time it on the squads' state as it stands.
            let saved = (g.squads.clone(), g.columns.clone(), g.soldiers.clone(), g.ranks.clone(), g.ranked);
            let t0 = std::time::Instant::now();
            g.squad_turn(dt);
            t += t0.elapsed().as_secs_f64();
            n += 1;
            on += g.columns[0].on as usize;
            (g.squads, g.columns, g.soldiers, g.ranks, g.ranked) = saved;
            g.step();
        }
        println!("funnel={funnel} squad_turn mean {:.1} us over {n} steps ({on} funnelling)", t / n as f64 * 1e6);
    }
}
