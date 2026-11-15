//! Technical review tests (2026-11-15), in a copy of 4494ad4f only.
use super::*;
use crate::mould::big::BigDish;
use crate::mould::map::{Crystal, Items, Map, G_ROCK};
use std::time::Instant;

/// A world `w` x `h` map cells; with `bar`, a rock bar at map x 400-447,
/// y 120-519, between the landing (300, 320) and open ground east of it.
fn test_map(w: usize, h: usize, bar: bool) -> Map {
    let mut ground = vec![0u8; w * h];
    if bar {
        for y in 120..520 {
            for x in 400..448 {
                ground[y * w + x] = G_ROCK;
            }
        }
    }
    let items = Items {
        crystals: vec![Crystal { x: 900.0, y: 80.0, r: 20.0, charge: 384.0 }, Crystal { x: 200.0, y: 560.0, r: 20.0, charge: 384.0 }],
        moulds: vec![(960.0, 600.0)],
        landing: Some((300.0, 320.0)),
        walls: Vec::new(),
        moisture: None,
    };
    Map { w, h, ground, plants: Vec::new(), items }
}

fn game(w: usize, h: usize, bar: bool, whole: bool) -> Game<BigDish> {
    let m = test_map(w, h, bar);
    let mut rules = Rules { build: BuildRules { raise: 0.0, ..BuildRules::default() }, ..Rules::default() };
    rules.squads = true;
    if whole {
        rules.map_pops = usize::MAX;
    }
    Game::new(BigDish::with_map(race_params(1, 2, 300), &m), rules, 1)
}

/// A squad steered east into the bar: how long until its middle is past
/// the bar (map x 480), at 1x and 6x, sliced maps and whole ones.
#[test]
fn review_steered_squad_round_rock() {
    for (w, h, whole) in [(1024, 640, false), (1024, 640, true), (2512, 1568, false), (2512, 1568, true)] {
        let mut g = game(w, h, true, whole);
        for _ in 0..4 {
            g.apply(Cmd::Buy);
        }
        g.apply(Cmd::Active(0));
        let tps = g.rules.tps;
        for _ in 0..(2 * tps) {
            g.step();
        }
        g.apply(Cmd::Steer(0, (1.0, 0.0)));
        let (mut past, mut still, mut waiting) = (None, 0, 0);
        let (mut most, t0) = (0.0f64, Instant::now());
        let n = 60 * tps;
        for i in 0..n {
            let before = g.squads[0].anchor;
            let t = Instant::now();
            g.step();
            most = most.max(t.elapsed().as_secs_f64() * 1e3);
            if g.squads[0].anchor == before {
                still += 1;
            }
            if g.paths.building.is_some() || !g.paths.asked.is_empty() {
                waiting += 1;
            }
            let m = g.middle(0).unwrap();
            if past.is_none() && m.0 * 2.0 > 480.0 {
                past = Some(i as f32 / tps as f32);
                break;
            }
        }
        let m = g.middle(0).unwrap();
        eprintln!(
            "STEER {w}x{h} whole={whole}: past the bar at {past:?} s; anchor still {still} steps, a map building or asked {waiting} steps; middle at map ({:.0}, {:.0}); step max {most:.1} ms; wall {:.1} s",
            m.0 * 2.0,
            m.1 * 2.0,
            t0.elapsed().as_secs_f64()
        );
    }
}

/// `n` live goals asked every step (soldiers whose straight walks are
/// shut): after each is first served, how often it is not served again.
#[test]
fn review_walking_map_cache_thrash() {
    for (w, h) in [(1024, 640), (2512, 1568)] {
        for n in [8usize, 16, 17, 18, 24, 32] {
            let mut g = game(w, h, false, false);
            let (fw, fh) = g.world.size();
            let goals: Vec<(f32, f32)> = (0..n).map(|k| (30.0 + (k % 8) as f32 * (fw as f32 - 60.0) / 7.0, 30.0 + (k / 8) as f32 * 40.0)).collect();
            let near = MAP_NEAR * g.s();
            let (mut first, mut lost, mut pump_max, mut pump_sum) = (vec![None; n], 0usize, 0.0f64, 0.0f64);
            let steps = 1500;
            for step in 0..steps {
                let t = Instant::now();
                g.pump_maps();
                let ms = t.elapsed().as_secs_f64() * 1e3;
                pump_max = pump_max.max(ms);
                pump_sum += ms;
                for (k, &p) in goals.iter().enumerate() {
                    let gi = cell(fw, fh, p);
                    match g.map_for(gi, p, near) {
                        Some(_) => {
                            if first[k].is_none() {
                                first[k] = Some(step);
                            }
                        }
                        None => {
                            if first[k].is_some() {
                                lost += 1;
                            }
                        }
                    }
                }
            }
            let most = ((16usize << 20) / (fw * fh)).clamp(4, MAPS_MAX);
            let all = first.iter().map(|f| f.unwrap_or(usize::MAX)).max().unwrap();
            eprintln!(
                "THRASH {w}x{h} fine {fw}x{fh} cache {most}: {n} goals, all served by step {all}, then unserved {lost} goal-steps of {} ({:.1}%); pump mean {:.2} ms max {pump_max:.1} ms",
                n * steps,
                100.0 * lost as f64 / (n * steps) as f64,
                pump_sum / steps as f64
            );
        }
    }
}

/// A wall going up every `period` steps (`stale_maps`) while one goal
/// waits for its map at 6x: does it ever get one?
#[test]
fn review_walls_in_succession_restart_the_build() {
    for (w, h) in [(1024, 640), (2512, 1568)] {
        for period in [2u64, 4, 6, 8, 10, 16] {
            let mut g = game(w, h, false, false);
            let (fw, fh) = g.world.size();
            let p = (fw as f32 - 40.0, fh as f32 - 40.0);
            let gi = cell(fw, fh, p);
            let near = MAP_NEAR * g.s();
            let mut got = None;
            for step in 0..600u64 {
                g.pump_maps();
                if g.map_for(gi, p, near).is_some() {
                    got = Some(step);
                    break;
                }
                if step % period == period - 1 {
                    g.stale_maps();
                }
            }
            eprintln!("WALLS {w}x{h}: a wall every {period} steps ({:.2} s): first map at step {got:?}", period as f32 / g.rules.tps as f32);
        }
    }
}

/// A stale map (a wall went up) while another goal keeps being asked (a
/// steered squad's goal moves each time it is served, `guide`'s near is
/// half a step): when is the stale one rebuilt?
#[test]
fn review_stale_maps_wait_behind_asks() {
    for mover in [false, true] {
        let mut g = game(2512, 1568, false, false);
        let (fw, fh) = g.world.size();
        let near = MAP_NEAR * g.s();
        let a = (100.0, 100.0);
        let ai = cell(fw, fh, a);
        while g.map_for(ai, a, near).is_none() {
            g.pump_maps();
        }
        let mut m = (600.0f32, 600.0f32);
        let mut moves = 0;
        let tick = |g: &mut Game<BigDish>, moves: &mut i32, m: &mut (f32, f32)| {
            g.pump_maps();
            g.map_for(ai, a, near);
            if mover {
                let mi = cell(fw, fh, *m);
                if g.map_for(mi, *m, 0.5).is_some() {
                    m.0 += 1.0;
                    *moves += 1;
                }
            }
        };
        // The steered squad is already moving when the wall goes up.
        for _ in 0..40 {
            tick(&mut g, &mut moves, &mut m);
        }
        let before = moves;
        g.stale_maps();
        assert!(g.paths.stale.contains(&ai));
        let old = g.paths.maps[&ai].0.as_ptr();
        let mut rebuilt = None;
        for step in 0..1500 {
            tick(&mut g, &mut moves, &mut m);
            if g.paths.maps.get(&ai).is_some_and(|mm| mm.0.as_ptr() != old) {
                rebuilt = Some(step);
                break;
            }
        }
        eprintln!("STALE 6x mover={mover}: moved {before} times in 40 steps before the wall; stale map rebuilt at step {rebuilt:?} after it (the mover moved {} times meanwhile)", moves - before);
    }
}

/// The game's own walking grid (`standable_cells`, walls shut) on planets
/// whose checks pass only through the map's outer 12 cells: how many
/// crystals and nests the landing reaches. Planet 8 seed 1 is the control.
#[test]
fn review_planets_through_the_rim() {
    for (seed, depth) in [(1u64, 8u32), (19, 8), (39, 5), (21, 4)] {
        for rim_open in [false, true] {
        let m = crate::mould::levelgen::level(seed, depth);
        let mut world = BigDish::with_map(race_params(seed, 2, 500), &m);
        if rim_open {
            // The dish's rim (no agar within 3 dish cells of the edge) as
            // the map's own ground there.
            let (fw, fh) = (world.w, world.h);
            for i in 0..fw * fh {
                let (x, y) = (i % fw, i / fw);
                if (x < 6 || y < 6 || x + 6 >= fw || y + 6 >= fh) && !world.rock[i] && !world.wall[i] {
                    world.agar[i] = true;
                }
            }
        }
        let g = Game::new(world, Rules::default(), seed);
        let (w, h) = g.world.size();
        let ok = g.standable_cells();
        let from = cell(w, h, g.larder);
        let d = distance_map(w, h, from, |i| ok[i]);
        let reached = |c: (f32, f32), r: f32| {
            let rr = r + 3.0 * g.s();
            let (x0, x1) = (((c.0 - rr).max(0.0)) as usize, ((c.0 + rr) as usize).min(w - 1));
            let (y0, y1) = (((c.1 - rr).max(0.0)) as usize, ((c.1 + rr) as usize).min(h - 1));
            (y0..=y1).any(|y| (x0..=x1).any(|x| d[y * w + x] != u32::MAX))
        };
        let cs = g.world.crystals();
        let got = (0..cs.len()).filter(|&i| reached(cs[i].0, g.world.crystal_r(i))).count();
        let open = d.iter().filter(|&&v| v != u32::MAX).count();
        let edge = (0..w * h).filter(|&i| d[i] != u32::MAX && { let (x, y) = (i % w, i / w); x < 8 || y < 8 || x + 8 >= w || y + 8 >= h }).count();
        let s = crate::mould::levelgen::stats(&crate::mould::levelgen::planet(seed, depth), &m);
        eprintln!("RIMGAME planet {depth} seed {seed} rim_open={rim_open}: the game's grid reaches {got} of {} crystals without breaking soft rock ({} cells open from the landing, {edge} of them within 4 dish cells of the edge); levelgen says open charge {:.0}%, unreachable {}", cs.len(), open, s.open_share * 100.0, s.unreachable);
        }
    }
}
