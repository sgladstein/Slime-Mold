//! The game's tests. Moved out of `larder_world.rs` unchanged (C9,
//! 2026-11-14).

use super::*;

fn small(seed: u64) -> Game<BigDish> {
    // Buildings whole when placed (raising has its own tests).
    Game::new(race_world(seed, 1, 1_000), Rules { build: BuildRules { raise: 0.0, ..BuildRules::default() }, ..Rules::default() }, seed)
}

#[test]
fn bullets_fly_over_frames_and_land_later() {
    // A bullet's first leg is fired, later legs fly on from where the
    // last ended, and a hit ends it; nothing flies past its range.
    let mut g = small(3);
    for k in 0..g.soldiers.len() {
        g.apply(Cmd::Order(k, Order::Attack));
    }
    let range = g.rules.range * g.s();
    let (mut fired, mut flown, mut late_hits) = (0, 0, 0);
    for _ in 0..(40 * g.rules.tps) {
        let before = g.bullets.clone();
        g.step();
        for s in &g.last_shots {
            if s.fired {
                fired += 1;
            } else {
                flown += 1;
                late_hits += s.hit.is_some() as u32;
                // A later leg starts where a bullet in flight was.
                assert!(before.iter().any(|b| b.at == s.from), "a leg from nowhere: {s:?}");
            }
        }
        assert!(g.bullets.len() <= squad::SHOTS_MAX);
        assert!(g.bullets.iter().all(|b| b.left > 0.0 && b.left <= range + 1.0 && !b.fresh));
        if g.end.is_some() {
            break;
        }
    }
    assert!(fired > 0 && flown > 0 && late_hits > 0, "fired {fired} flown {flown} late hits {late_hits}");
    assert_eq!(g.shots, fired, "shots counts bullets fired, not legs");
}

#[test]
fn bullets_cap_at_the_shot_buffer_and_stop_at_walls() {
    let mut g = small(1);
    let sc = g.s();
    // Past `SHOTS_MAX` in flight the newest are dropped, so a frame
    // never hands the engines more legs than they take.
    let from = g.soldiers[0];
    let many: Vec<ShotIn> = (0..squad::SHOTS_MAX + 40).map(|_| ShotIn { from: (from.x, from.y), to: (from.x + 10.0 * sc, from.y), r: 1.0, over: false }).collect();
    let by = vec![(Some(0), 0, None); many.len()];
    let (legs, who) = g.fly(many, &by, &[], 1.0 / 15.0);
    assert_eq!((legs.len(), who.len(), g.bullets.len()), (squad::SHOTS_MAX, squad::SHOTS_MAX, squad::SHOTS_MAX));
    assert_eq!(g.bullets_dropped, 40);
    // A leg ends at the first closed cell on its way, and the bullet
    // with it.
    g.bullets.clear();
    let (w, h) = g.world.size();
    let wall = (0..w * h).map(|i| ((i % w) as f32 + 0.5, (i / w) as f32 + 0.5)).find(|&(x, y)| !g.world.open(x, y) && g.world.open(x - 3.0, y) && g.world.open(x - 2.0, y) && g.world.open(x - 1.0, y) && x > 4.0).expect("a wall or rock cell with open ground west of it");
    let start = (wall.0 - 3.0, wall.1);
    let (legs, _) = g.fly(vec![ShotIn { from: start, to: (start.0 + 20.0, start.1), r: 1.0, over: false }], &[(None, 0, None)], &[], 1.0);
    assert!((legs[0].to.0 - wall.0).abs() <= 1.0, "leg {:?} wall {:?}", legs[0], wall);
    g.landed(&legs, &[squad::ShotOut { hit: None, killed: 0 }]);
    assert!(g.bullets.is_empty(), "the bullet flew on past the wall");
}

/// A seeking bullet (`squad::Frame::catch`, the owner's pick 2026-11-16
/// 19:38Z) stops at live walkers on every leg (`squad::SEEKS`); its catch
/// beside the line rides only the legs near its aim point (`Bullet::aim`)
/// and its last, the frame it ends its range or meets a wall: elsewhere a
/// leg short of the slime is no miss, and a catch there struck the fringe
/// beside the line (`Game::catches`). Its width is `seek_catch` dish cells
/// a card; no seeker, no list.
#[test]
fn a_seekers_catch_rides_its_aim_and_last_leg() {
    let mut g = small(2);
    let r = g.rules;
    let sc = g.s();
    let dt = r.frame_ticks as f32 / r.tps as f32;
    let leg = r.bullet_speed * sc * dt;
    assert!(r.bullet_speed > 0.0 && leg > 1.0, "bullets fly in legs");
    let b = Bullet { at: (10.0, 10.0), dir: (1.0, 0.0), left: 10.0 * leg, r: 1.0, by: None, turret: None, fresh: false, pierce: 0, split: 0, cast: 0, speed: 1.0, carry: 0, expire: false, seek: 1, flown: 0.0, aim: f32::INFINITY, over: false };
    g.bullets = vec![b, Bullet { left: 0.5 * leg, ..b }, Bullet { left: 0.5 * leg, seek: 2, ..b }, Bullet { left: 0.5 * leg, seek: 0, ..b }];
    let one = (r.seek_catch * sc).round() as u16;
    let s = squad::SEEKS;
    assert_eq!(g.catches(&[], dt), vec![s, s | one, s | (2.0 * r.seek_catch * sc).round() as u16, 0]);
    // Near its aim point, mid-flight: this leg reaches it, or ends within
    // the catch of it; one aimed farther on gets none yet.
    let near = r.seek_catch * sc;
    g.bullets = vec![Bullet { aim: 0.5 * leg, ..b }, Bullet { aim: leg + 0.5 * near, ..b }, Bullet { aim: 5.0 * leg + near, ..b }, Bullet { aim: 0.5 * leg, flown: 2.0 * leg + near, ..b }];
    assert_eq!(g.catches(&[], dt), vec![s | one, s | one, s, s]);
    g.bullets = vec![Bullet { seek: 0, ..b }, Bullet { seek: 0, left: 0.5 * leg, ..b }];
    assert!(g.catches(&[], dt).is_empty(), "no seeker: no list");
}

#[test]
fn pierce_flies_on_past_a_hit_and_split_leaves_fanned_pieces() {
    let mut g = small(1);
    let sc = g.s();
    let r = g.rules;
    let leg = |from: (f32, f32)| ShotIn { from, to: (from.0 + 4.0, from.1), r: 2.0, over: false };
    let hit = |at: (f32, f32)| squad::ShotOut { hit: Some(at), killed: 1 };
    let b = Bullet { at: (50.0, 50.0), dir: (1.0, 0.0), left: 30.0, r: 2.0, by: Some(0), turret: None, fresh: true, pierce: 1, split: 1, cast: 0, speed: 1.0, carry: 0, expire: false, seek: 0, flown: 0.0, aim: f32::INFINITY, over: false };
    // Pierce: on past the hit by the gap, one pierce fewer, range spent
    // to there; the split waits for the bullet's last hit.
    g.bullets = vec![b];
    g.landed(&[leg(b.at)], &[hit((52.0, 50.0))]);
    let gap = r.pierce_gap * 2.0 + 1.0;
    assert_eq!(g.bullets.len(), 1);
    let p = g.bullets[0];
    assert_eq!((p.pierce, p.split, p.fresh), (0, 1, false));
    assert!((p.at.0 - (52.0 + gap)).abs() < 1e-4 && (p.left - (30.0 - 2.0 - gap)).abs() < 1e-4, "{p:?}");
    // Split: the spent bullet goes, `split_n` pieces fan round its
    // heading from the hit, each `split_range` long and smaller.
    g.landed(&[leg(p.at)], &[hit((60.0, 50.0))]);
    assert_eq!(g.bullets.len(), r.split_n as usize);
    for (j, q) in g.bullets.iter().enumerate() {
        assert_eq!((q.split, q.pierce, q.by, q.fresh), (0, 0, Some(0), false));
        assert!((q.left - r.split_range * sc).abs() < 1e-4 && (q.r - 2.0 * r.split_r).abs() < 1e-4);
        let a = q.dir.1.atan2(q.dir.0);
        assert!(a.abs() <= r.split_fan + 1e-4 && q.at.0 > 60.0, "piece {j} {q:?}");
    }
    assert!(g.bullets[0].dir.1 < 0.0 && g.bullets[r.split_n as usize - 1].dir.1 > 0.0, "fanned both ways");
    // A piece that hits makes nothing more, and a miss on a wall (no
    // hit) never splits.
    let n = g.bullets.len();
    let legs: Vec<ShotIn> = g.bullets.iter().map(|q| leg(q.at)).collect();
    let outs: Vec<squad::ShotOut> = (0..n).map(|i| if i == 0 { hit(legs[0].from) } else { squad::ShotOut { hit: None, killed: 0 } }).collect();
    g.landed(&legs, &outs);
    assert_eq!(g.bullets.len(), n - 1);
}

/// **A carrier lets its cast out** (2026-11-16, step B of the gun-system
/// review; `rounds::Card::Trigger`): at its first slime hit, as that cast's
/// own bullets fired from there along the carrier's heading (a buckshot
/// cast is four pellets fanned round it), while the carrier pierces on
/// empty. With `Expire` it keeps its cast through a hit it pierces and lets
/// it out where it stops: flying on from the end of its range, or thrown
/// back off a wall.
#[test]
fn a_carrier_lets_its_cast_out_where_it_hits_or_stops() {
    use super::super::rounds::{parse, Deck};
    let mut g = small(1);
    let sc = g.s();
    let q = g.soldiers[0].squad as usize;
    let load = |g: &mut Game<BigDish>, s: &str| g.apply(Cmd::Load(q as u8, Some(Deck::new(&parse(s).unwrap()))));
    load(&mut g, "trigger+plain+ammo/plain/buckshot/1/100/100/100/100/1");
    let m = g.mods(q);
    assert_eq!((m.casts, m.cycle[0], m.child[0], m.child[1]), (1, 1, 2, 0), "the buckshot is carried, not fired in turn");
    let (w, h) = g.world.size();
    // Open ground with room ahead, and a wall with open ground before it.
    let open = (0..w * h).map(|i| ((i % w) as f32 + 0.5, (i / w) as f32 + 0.5)).find(|&(x, y)| (0..12).all(|j| g.world.clear(x + j as f32, y))).expect("open ground");
    let wall = (0..w * h).map(|i| ((i % w) as f32 + 0.5, (i / w) as f32 + 0.5)).find(|&(x, y)| !g.world.clear(x, y) && (1..=3).all(|j| g.world.clear(x - j as f32, y)) && g.world.clear(x - 1.0, y + 1.0) && g.world.clear(x - 1.0, y - 1.0) && !g.world.clear(x, y + 1.0) && !g.world.clear(x, y - 1.0) && x > 4.0).expect("a wall face with open ground west of it");
    let carrier = Bullet { at: open, dir: (1.0, 0.0), left: 30.0, r: 2.0, by: Some(0), turret: None, fresh: false, pierce: 1, split: 0, cast: 1, speed: 1.0, carry: 2, expire: false, seek: 0, flown: 0.0, aim: f32::INFINITY, over: false };
    let leg = |from: (f32, f32), len: f32| ShotIn { from, to: (from.0 + len, from.1), r: 2.0, over: false };
    let hit = |at: (f32, f32)| squad::ShotOut { hit: Some(at), killed: 1 };
    let miss = squad::ShotOut { hit: None, killed: 0 };
    let h0 = (open.0 + 2.0, open.1);
    g.bullets = vec![carrier];
    g.landed(&[leg(open, 4.0)], &[hit(h0)]);
    assert_eq!((g.releases, g.bullets.len()), (1, 5));
    assert_eq!((g.bullets[0].carry, g.bullets[0].pierce, g.bullets[0].cast), (0, 0, 1), "the carrier pierces on empty");
    let ammo = m.ammo[1];
    let shape = ammo.set.shape();
    for p in &g.bullets[1..] {
        assert_eq!((p.cast, p.carry, p.at, p.fresh), (2, 0, h0, false));
        assert!(p.dir.1.atan2(p.dir.0).abs() <= shape.fan + 1e-4, "{p:?}");
        assert!((p.left - g.gun_range(q) * sc * ammo.range()).abs() < 1e-3 && (p.r - (g.strike_of(q) * ammo.reach()).min(squad::R_MAX) * shape.r).abs() < 1e-4, "{p:?}");
    }
    assert!(g.bullets[1].dir.1 < 0.0 && g.bullets[4].dir.1 > 0.0, "fanned both ways");
    // Expire: through the hit it pierces, still carrying.
    load(&mut g, "expire+plain+plain");
    let b = Bullet { carry: 2, expire: true, ..carrier };
    g.bullets = vec![b];
    g.landed(&[leg(open, 4.0)], &[hit(h0)]);
    assert_eq!((g.releases, g.bullets.len(), g.bullets[0].carry), (1, 1, 2));
    // Its range runs out on open ground: the cast flies on from there.
    g.bullets = vec![Bullet { at: open, left: 3.0, ..b }];
    g.landed(&[leg(open, 3.0)], &[miss]);
    assert_eq!((g.releases, g.bullets.len()), (2, 1));
    let p = g.bullets[0];
    assert_eq!((p.cast, p.at, p.dir), (2, (open.0 + 3.0, open.1), (1.0, 0.0)));
    // A wall: it heads back out the way it came, from a cell back.
    let start = (wall.0 - 3.0, wall.1);
    g.bullets.clear();
    let (legs, _) = g.fly(vec![ShotIn { from: start, to: (start.0 + 20.0, start.1), r: 1.0, over: false }], &[(Some(0), 1, None)], &[], 1.0);
    assert_eq!((g.bullets[0].carry, g.bullets[0].expire), (2, true), "the fired bullet takes its cast's carry");
    g.landed(&legs, &[miss]);
    assert_eq!((g.releases, g.bullets.len()), (3, 1));
    let p = g.bullets[0];
    assert!(p.dir.0 < -0.99 && g.world.clear(p.at.0, p.at.1) && p.at.0 < legs[0].to.0, "thrown back off the wall: {p:?} leg {:?}", legs[0]);
}

#[test]
fn split_pieces_cap_at_the_shot_buffer_in_order() {
    // The flood: a full buffer of splitting bullets all hit at once.
    // Pieces go to the end in their parents' order until the buffer
    // is full, the rest are counted, never more than `SHOTS_MAX` fly.
    let mut g = small(1);
    let b = |i: usize| Bullet { at: (10.0 + i as f32, 40.0), dir: (1.0, 0.0), left: 30.0, r: 1.0, by: Some(i % 4), turret: None, fresh: false, pierce: 0, split: 2, cast: 0, speed: 1.0, carry: 0, expire: false, seek: 0, flown: 0.0, aim: f32::INFINITY, over: false };
    g.bullets = (0..squad::SHOTS_MAX).map(b).collect();
    let legs: Vec<ShotIn> = g.bullets.iter().map(|q| ShotIn { from: q.at, to: (q.at.0 + 1.0, q.at.1), r: 1.0, over: false }).collect();
    // Every second one hits; the others fly on and keep their places.
    let outs: Vec<squad::ShotOut> = (0..legs.len()).map(|i| squad::ShotOut { hit: (i % 2 == 0).then_some(legs[i].to), killed: 0 }).collect();
    g.landed(&legs, &outs);
    let kept = squad::SHOTS_MAX / 2;
    let n = g.rules.split_n as usize;
    assert_eq!(g.bullets.len(), squad::SHOTS_MAX);
    assert_eq!(g.bullets_dropped as usize, kept * n - (squad::SHOTS_MAX - kept));
    assert!(g.bullets[..kept].iter().enumerate().all(|(i, q)| q.split == 2 && q.at.0 == 10.0 + (2 * i + 1) as f32 + 1.0));
    // The pieces after them are the first parents' pieces, in order.
    for (j, q) in g.bullets[kept..].iter().enumerate() {
        let parent = 2 * (j / n);
        assert_eq!((q.split, q.by), (1, Some(parent % 4)), "piece {j}");
    }
}

#[test]
fn split_and_pierce_bullets_hit_more_and_play_the_same_twice() {
    // In a real fight: pieces and pierced legs land hits of their own,
    // the buffer is never overrun, and the same seed plays the same.
    let run = |mods: BulletMods| {
        let mut g = small(3);
        g.bullet_mods = mods;
        for k in 0..g.soldiers.len() {
            g.apply(Cmd::Order(k, Order::Attack));
        }
        let mut later = 0u32;
        for _ in 0..(30 * g.rules.tps) {
            g.step();
            assert!(g.bullets.len() <= squad::SHOTS_MAX);
            later += g.last_shots.iter().filter(|s| !s.fired && s.hit.is_some()).count() as u32;
            if g.end.is_some() {
                break;
            }
        }
        (g.shots, later, g.killed, g.mould)
    };
    let both = BulletMods { pierce: 1, split: 2, ..BulletMods::default() };
    let a = run(both);
    assert_eq!(a, run(both), "the same seed played twice");
    let plain = run(BulletMods::default());
    assert!(a.1 > plain.1, "later hits: mods {a:?} plain {plain:?}");
}

#[test]
fn the_walking_map_goes_round_a_wall_through_its_gap() {
    // 20 x 10, a wall down x = 10 with one gap at y = 8.
    let (w, h) = (20, 10);
    let open = |i: usize| i % w != 10 || i / w == 8;
    let goal = 2 * w + 15;
    let map = distance_map(w, h, goal, open);
    let start = 2 * w + 5;
    assert_eq!(map[goal], 0);
    assert!(map[start] < u32::MAX, "reachable through the gap");
    // Straight would be 10 side steps (50); round the gap is longer.
    assert!(map[start] > 50, "not through the wall: {}", map[start]);
    let mut at = start;
    let mut steps = 0;
    while at != goal && steps < 100 {
        let next = downhill(w, h, &map, at, true);
        assert!(open(next) && next != at, "every step is open and goes downhill");
        at = next;
        steps += 1;
    }
    assert_eq!(at, goal);
    // Shut the gap: the goal is out of reach and the map says so.
    let sealed = distance_map(w, h, goal, |i| i % w != 10);
    assert_eq!(sealed[start], u32::MAX);
}

/// The binary-heap Dijkstra `distance_map` was until 11-14, kept as the
/// reference for the bucket ring that replaced it.
fn heap_distance_map(w: usize, h: usize, goal: usize, open: impl Fn(usize) -> bool) -> Vec<u32> {
    use std::cmp::Reverse;
    use std::collections::BinaryHeap;
    let ok: Vec<bool> = (0..w * h).map(&open).collect();
    let mut dist = vec![u32::MAX; w * h];
    let mut heap = BinaryHeap::new();
    dist[goal] = 0;
    heap.push(Reverse((0u32, goal)));
    while let Some(Reverse((d, i))) = heap.pop() {
        if d > dist[i] {
            continue;
        }
        let (x, y) = ((i % w) as i64, (i / w) as i64);
        for (dx, dy) in [(-1i64, 0i64), (1, 0), (0, -1), (0, 1), (-1, -1), (1, -1), (-1, 1), (1, 1)] {
            let (nx, ny) = (x + dx, y + dy);
            if nx < 0 || ny < 0 || nx >= w as i64 || ny >= h as i64 {
                continue;
            }
            let j = ny as usize * w + nx as usize;
            let diag = dx != 0 && dy != 0;
            if !ok[j] || (diag && (!ok[y as usize * w + nx as usize] || !ok[ny as usize * w + x as usize])) {
                continue;
            }
            let nd = d + if diag { 7 } else { 5 };
            if nd < dist[j] {
                dist[j] = nd;
                heap.push(Reverse((nd, j)));
            }
        }
    }
    dist
}

/// The bucket ring gives the heap's distances on mazes of every density,
/// goals in the open, in a pocket and on rock.
#[test]
fn the_walking_map_ring_matches_the_heap() {
    let mut seed = 7u64;
    let mut next = || {
        seed = seed.wrapping_mul(6_364_136_223_846_793_005).wrapping_add(1_442_695_040_888_963_407);
        (seed >> 33) as usize
    };
    for (w, h, rock) in [(40, 25, 0), (40, 25, 5), (64, 48, 3), (31, 57, 2), (1, 9, 4)] {
        let open: Vec<bool> = (0..w * h).map(|_| rock == 0 || next() % rock != 0).collect();
        for _ in 0..6 {
            let goal = next() % (w * h);
            let ring = distance_map(w, h, goal, |i| open[i]);
            assert_eq!(ring, heap_distance_map(w, h, goal, |i| open[i]), "{w} x {h}, one in {rock} rock, goal {goal}");
            // Built a slice at a time, as `Game::pump_maps` does: the same map.
            for pops in [1, 7, 100] {
                let mut b = MapBuild::new(w, h, goal, open.clone());
                let mut slices = 0;
                while !b.done() {
                    assert!(b.advance(w, h, pops) <= pops);
                    slices += 1;
                    assert!(slices <= 9 * w * h, "a build ends");
                }
                assert_eq!(b.map(), ring, "{pops} cells a slice");
            }
        }
    }
}

/// The walking grid a map is built on is `standable` at each cell's
/// middle, with crystals solid, at the start and as they are eaten.
#[test]
fn standable_cells_are_standable_at_each_middle() {
    let mut g = small(2);
    assert!(!g.solid.is_empty(), "crystals are solid");
    for round in 0..3 {
        let (w, h) = g.world.size();
        let cells = g.standable_cells();
        let each: Vec<bool> = (0..w * h).map(|i| g.standable((i % w) as f32 + 0.5, (i / w) as f32 + 0.5)).collect();
        assert_eq!(cells, each, "round {round}");
        let open: Vec<bool> = (0..w * h).map(|i| g.world.open((i % w) as f32 + 0.5, (i / w) as f32 + 0.5)).collect();
        assert_eq!(g.world.open_cells(), open, "the dish's own grid is `open` at each middle");
        assert!(cells.iter().any(|&c| !c) && cells.iter().any(|&c| c));
        for _ in 0..(g.rules.tps * 10) {
            g.step();
        }
    }
    // A planet with deep water: the dish's grid leaves it out as `open` does.
    let m = (1..20).map(|seed| crate::mould::levelgen::level(seed, 4)).find(|m| m.ground.contains(&crate::mould::map::G_DEEP)).expect("a river valley with deep water");
    let d = BigDish::with_map(race_params(1, 1, 1_000), &m);
    let (w, h) = World::size(&d);
    let open: Vec<bool> = (0..w * h).map(|i| World::open(&d, (i % w) as f32 + 0.5, (i / w) as f32 + 0.5)).collect();
    assert!((0..w * h).any(|i| d.deep(i)), "deep cells on the dish");
    assert_eq!(World::open_cells(&d), open);
}

#[test]
fn commands_round_trip_as_lines() {
    for c in [
        Cmd::Order(2, Order::Clear(10.5, 20.25)),
        Cmd::Order(0, Order::Burn((1.0, 2.0), (3.0, 4.5))),
        Cmd::Stance(1, Stance::Free),
        Cmd::Buy,
        Cmd::Wall((100.125, 40.0), (100.125, 80.5)),
    ] {
        assert_eq!(Cmd::parse(&c.to_line()), Some(c), "{}", c.to_line());
    }
}

/// The core's pull (`Core::pull`): a soldier within twice `core_r` of
/// the core lays `pull` times its scent over a lure's reach; one far
/// from it lays its own; a headless core pulls the same (the default
/// since 11-12 20:21Z), and a pull of 0 pulls nothing.
#[test]
fn a_soldier_at_the_core_smells_like_a_lure() {
    let units = |kind: CoreKind, pull: f32| {
        let mut g = Game::new(race_world(1, 1, 1_000), Rules { core_pull: 8.0, build: BuildRules { raise: 0.0, ..BuildRules::default() }, ..Rules::default() }, 1);
        (g.core_rule.kind, g.core_rule.pull) = (kind, pull);
        let sc = g.s();
        (g.soldiers[0].x, g.soldiers[0].y) = (g.core.0 + g.rules.core_r * sc, g.core.1);
        g.begin_step();
        let f = &g.flight.as_ref().expect("a frame in flight").f;
        (f.units[0].smell, f.units[0].smell_r, f.units[1].smell, f.units[1].smell_r, g.rules, sc)
    };
    for kind in [CoreKind::Fixed, CoreKind::Headless] {
        let (near, near_r, far, far_r, r, sc) = units(kind, 8.0);
        assert!(far > 0.0 && (near - 8.0 * far).abs() <= 1e-3 * near, "{kind:?}: near the core {near}, far {far}");
        assert_eq!((near_r, far_r), (r.build.lure_r * sc, r.smell_r * sc));
    }
    let (near, near_r, far, _, r, sc) = units(CoreKind::Headless, 0.0);
    assert_eq!((near, near_r), (far, r.smell_r * sc), "a pull of 0 pulls nothing");
}

/// A fixed core falls once held clear for `Core::secs` (births stop);
/// a headless one never does, so only a wipe ends that mould.
#[test]
fn a_held_core_falls_unless_the_mould_is_headless() {
    let hold = |kind: CoreKind| {
        let mut g = small(1);
        g.core_rule.kind = kind;
        for k in 0..g.soldiers.len() {
            (g.soldiers[k].x, g.soldiers[k].y) = g.core;
            (g.soldiers[k].touching, g.soldiers[k].aim) = (0, None);
        }
        for _ in 0..=g.secs(g.core_rule.secs) {
            g.check_core();
        }
        (g.core_down.is_some(), g.world.p.birth_energy.is_finite())
    };
    assert_eq!(hold(CoreKind::Fixed), (true, false), "held 5 s: the core falls and births stop");
    assert_eq!(hold(CoreKind::Headless), (false, true), "no core to hold");
}

/// Wading (`Rules::wade`): with 6 walkers touching a soldier walks at
/// half speed, with 12 at a fifth; open ground and `wade` 0 at full.
#[test]
fn thick_slime_slows_a_soldier() {
    let moved = |touching: u32, wade: f32| {
        let mut g = small(1);
        g.rules.wade = wade;
        let (x, y) = (g.soldiers[0].x, g.soldiers[0].y);
        let sc = g.s();
        // A goal well past one step, on open ground beside the larder.
        let to = (0..16).map(|k| k as f32 / 16.0 * std::f32::consts::TAU).map(|a| (x + 6.0 * sc * a.cos(), y + 6.0 * sc * a.sin())).find(|&p| g.clear_line((x, y), p)).expect("open ground by the larder");
        g.soldiers[0].order = Order::Move(to.0, to.1);
        g.soldiers[0].touching = touching;
        g.walk(0, 1.0 / g.rules.tps as f32);
        (g.soldiers[0].x - x).hypot(g.soldiers[0].y - y)
    };
    let free = moved(0, 6.0);
    assert!(free > 0.0, "it walked");
    for (touching, share) in [(2, 0.9), (6, 0.5), (12, 0.2)] {
        let d = moved(touching, 6.0);
        assert!((d / free - share).abs() < 0.01, "{touching} touching: {d} against {free}");
    }
    assert_eq!(moved(12, 0.0), free, "wade 0 is off");
    // Whole walkers at any density (2026-11-13): shape C's mass of 0.2
    // once needed 30 touching for half speed.
    let mut g = small(1);
    g.rules.mass = 0.2;
    let (x, y) = (g.soldiers[0].x, g.soldiers[0].y);
    let sc = g.s();
    let to = (0..16).map(|k| k as f32 / 16.0 * std::f32::consts::TAU).map(|a| (x + 6.0 * sc * a.cos(), y + 6.0 * sc * a.sin())).find(|&p| g.clear_line((x, y), p)).expect("open ground by the larder");
    g.soldiers[0].order = Order::Move(to.0, to.1);
    g.soldiers[0].touching = 6;
    g.walk(0, 1.0 / g.rules.tps as f32);
    let d = (g.soldiers[0].x - x).hypot(g.soldiers[0].y - y);
    assert!((d / free - 0.5).abs() < 0.01, "mass 0.2, 6 touching: {d} against {free}");
}

/// A tier with no core of its own keeps the rules' (`Core::of`).
#[test]
fn a_tier_without_a_core_keeps_the_rules_core() {
    let r = Rules { core_pull: 3.0, core_secs: 7.0, ..Rules::default() };
    for t in super::super::tiers::Tier::ALL {
        let g = Game::new(race_world(1, 1, 1_000), r, 1).with_tier(Some(t));
        assert_eq!(g.core_rule, t.core().unwrap_or(Core { kind: CoreKind::Headless, secs: 7.0, pull: 3.0 }), "{}", t.name());
    }
    assert_eq!(Game::new(race_world(1, 1, 1_000), r, 1).with_tier(None).core_rule, Core::of(&r));
}

/// The broken mould (`Rules::broken`): under 3% of its peak, its upkeep
/// is 50 times, and it gets its upkeep back once it is over the line.
#[test]
fn a_broken_mould_starves_until_it_grows_back() {
    let mut g = small(1);
    let up = g.world.upkeep();
    assert!(up > 0.0 && g.broken.is_none());
    g.peak = g.mould * 100;
    g.check_broken();
    assert_eq!((g.broken.map(|b| b.1), g.world.upkeep()), (Some(up), up * g.rules.broken_starve));
    g.check_broken();
    assert_eq!(g.world.upkeep(), up * g.rules.broken_starve, "it starves once, not 50 x 50");
    g.peak = g.mould;
    g.check_broken();
    assert_eq!((g.broken, g.world.upkeep()), (None, up));
    let mut off = small(1);
    off.rules.broken = 0.0;
    off.peak = off.mould * 100;
    off.check_broken();
    assert_eq!((off.broken, off.world.upkeep()), (None, up));
}

/// A scared piece's reach follows the guns (`Rules::panic_margin`): set
/// when the game starts and kept once a game second, so a longer range
/// moves it; a negative margin leaves the world's own.
#[test]
fn a_scared_piece_sees_past_the_guns() {
    let mut g = small(1);
    assert_eq!(g.world.p.panic_r, g.rules.range + g.rules.panic_margin);
    assert!(g.world.p.panic_r > g.rules.range);
    g.rules.range = 36.0;
    let t = g.world.ticks();
    while g.world.ticks() < t + g.rules.tps as u64 {
        g.step();
    }
    assert_eq!(g.world.p.panic_r, 36.0 + g.rules.panic_margin, "a longer range moves it at the next count");
    // A rounds `Range` card stretches the gun (`gun_range`), and the reach with it.
    g.bullet_mods.reach = 1;
    let t = g.world.ticks();
    while g.world.ticks() < t + g.rules.tps as u64 {
        g.step();
        g.bullet_mods.reach = 1;
    }
    assert!(g.gun_range(0) > 36.0);
    assert_eq!(g.world.p.panic_r, g.gun_range(0) + g.rules.panic_margin, "a range card moves it too");
    let mut keep = race_world(1, 1, 1_000);
    keep.p.panic_r = 7.0;
    let g = Game::new(keep, Rules { panic_margin: -1.0, ..Rules::default() }, 1);
    assert_eq!(g.world.p.panic_r, 7.0);
}

/// No king by default (owner, 11-12 20:21Z): a squad holding the plain
/// mould's home stops nothing, and births go on.
#[test]
fn taking_the_plain_moulds_home_stops_nothing() {
    let mut g = small(1);
    assert_eq!(g.core_rule.kind, CoreKind::Headless);
    for k in 0..g.soldiers.len() {
        (g.soldiers[k].x, g.soldiers[k].y) = g.core;
        (g.soldiers[k].touching, g.soldiers[k].aim) = (0, None);
    }
    for _ in 0..=g.secs(g.core_rule.secs) * 2 {
        g.check_core();
    }
    assert!(g.core_down.is_none() && g.world.p.birth_energy.is_finite());
}

/// A head start (`head_start`, the run's big planets, 11-15): the slime
/// grows alone first, the world's ticks run on, and the game's clock still
/// starts at 0:00 on landing, so the cap and the goals count from there.
#[test]
fn a_head_start_grows_the_slime_and_the_clock_starts_on_landing() {
    let r = Rules { build: BuildRules { raise: 0.0, ..BuildRules::default() }, ..Rules::default() };
    let fresh = race_world(3, 1, 1_000);
    let mut grown = race_world(3, 1, 1_000);
    head_start(&mut grown, 10.0, &r);
    assert_eq!(grown.ticks(), 10 * u64::from(r.tps));
    assert_ne!(grown.living(), fresh.living(), "the slime lived on alone");
    let mut g = Game::new(grown, Rules { cap: 2.0, ..r }, 3);
    assert_eq!(g.time(), 0.0);
    while g.end.is_none() {
        g.step();
    }
    assert!(matches!(g.end, Some(End::Cap(_))));
    assert!((g.time() - 2.0).abs() < 0.1, "the cap counts from landing, not from the world's start: {}", g.time());
}

#[test]
fn the_squad_starts_far_from_the_mould() {
    let g = small(1);
    let d = (g.larder.0 - g.core.0).hypot(g.larder.1 - g.core.1);
    assert!(d > 100.0, "larder {:?} core {:?}", g.larder, g.core);
    assert_eq!(g.alive(), 4);
    assert!(g.soldiers.iter().all(|s| g.world.open(s.x, s.y)));
}

#[test]
fn a_clearing_soldier_carries_charge_home() {
    // One soldier on the squad's own crystal: it fills its carry
    // (300 food at 30 a second) and banks it. Four on one crystal
    // empty the cells under them faster than the charge flows back in.
    let mut g = small(2);
    let c = g.larder;
    g.apply(Cmd::Order(0, Order::Clear(c.0, c.1)));
    g.apply(Cmd::Stance(0, Stance::HoldFire));
    let (k, _) = g.crystal_at(c).unwrap();
    let before = g.world.crystals()[k].1;
    for _ in 0..(g.rules.tps * 30) {
        g.step();
    }
    assert!(g.harvested >= g.rules.carry, "banked {} in 30 s", g.harvested);
    // Equal to f32 rounding (with `Rules::home_dry` a carrier banks
    // part loads).
    let books = g.rules.start_bank + g.harvested - g.spent - g.stolen;
    assert!((g.bank - books).abs() < 0.1, "bank {} books {books}", g.bank);
    // From the cells: the pile's total is summed a tick late (the next
    // tick's eating), and the frame's last take is after the ticks.
    let d = &g.world;
    let now: f64 = d.pile_cells.iter().filter(|&&i| d.pile_of[i as usize] as usize == k + 1).map(|&i| d.charge[i as usize] as f64).sum();
    let gone = before - now as f32;
    let held = (g.harvested + g.soldiers[0].carry) * g.rules.charge;
    assert!(gone >= held * 0.999, "the crystal lost {gone}, the squad holds {held}");
}

#[test]
fn takes_are_listed_each_step_for_the_harvest_look() {
    // `Game::last_takes` is what the harvest look draws from (the soldier's
    // working pose and pulse, the crystal's reaction): this step's takes
    // only, by whom and from which crystal, summing (in charge, `Rules::charge`
    // a unit of food) to what was carried in.
    let mut g = small(2);
    let c = g.larder;
    g.apply(Cmd::Order(0, Order::Clear(c.0, c.1)));
    g.apply(Cmd::Stance(0, Stance::HoldFire));
    let (k, _) = g.crystal_at(c).unwrap();
    let mut seen = 0;
    for _ in 0..(g.rules.tps * 5) {
        let held = g.soldiers[0].carry + g.harvested;
        g.step();
        let took: f32 = g.last_takes.iter().filter(|t| t.by == Some(0)).map(|t| t.took).sum::<f32>() / g.rules.charge;
        assert!(g.last_takes.iter().all(|t| t.took > 0.0 && t.crystal == k), "{:?}", g.last_takes.len());
        let gained = g.soldiers[0].carry + g.harvested - held;
        assert!((gained - took).abs() < 1e-3, "carried in {gained}, listed {took}");
        seen += (took > 0.0) as usize;
    }
    assert!(seen > 0, "no take listed in 5 s on the crystal");
}

#[test]
fn soldiers_stand_round_a_crystal_not_on_it() {
    // Owner, 11-12 14:43Z: soldiers "shouldn't be able to stand on top
    // of the crystal". They start outside the larder's crystal, a move
    // onto a crystal stops at its rim, and harvesters still bank.
    let mut g = small(2);
    let (_, c) = g.crystal_at(g.larder).unwrap();
    let sc = g.s();
    for s in &g.soldiers {
        assert!(g.standable(s.x, s.y), "spawned on the crystal at {:?}", (s.x, s.y));
    }
    g.apply(Cmd::Stance(0, Stance::HoldFire));
    g.apply(Cmd::Stance(1, Stance::HoldFire));
    g.apply(Cmd::Order(0, Order::Move(c.0, c.1)));
    g.apply(Cmd::Order(1, Order::Clear(c.0, c.1)));
    for _ in 0..(g.rules.tps * 40) {
        g.step();
        for s in g.soldiers.iter().filter(|s| s.alive) {
            assert!(g.solid_at((s.x, s.y)).is_none(), "a soldier on a crystal at {:?}", (s.x, s.y));
        }
    }
    let s = g.soldiers[0];
    let (_, r) = g.solid_at(c).expect("the crystal still has charge");
    let d = (s.x - c.0).hypot(s.y - c.1);
    assert!(d >= r && d <= r + 3.0 * sc, "the mover stands {d} from the centre, rim {r}");
    assert!(g.harvested > 0.0, "nothing banked from the rim");
    // With the rule off the mover walks onto the crystal's middle.
    let mut w = small(2);
    w.rules.crystal_solid = false;
    w.apply(Cmd::Stance(0, Stance::HoldFire));
    w.apply(Cmd::Order(0, Order::Move(c.0, c.1)));
    for _ in 0..(w.rules.tps * 40) {
        w.step();
    }
    let s = w.soldiers[0];
    assert!((s.x - c.0).hypot(s.y - c.1) < 2.0 * sc, "off, it stands {} from the centre", (s.x - c.0).hypot(s.y - c.1));
}

#[test]
fn a_crystal_run_dry_sends_its_carrier_home_to_hold() {
    // The v1 Fork's case (WM1): a crystal with less than a load in it,
    // all within a soldier's reach (the v1 crystals were 3 dish cells
    // across the radius, under the 4 a soldier takes from). Its soldier
    // empties it, walks home with what it took, banks it and holds
    // there, rather than wait for a full load or walk back for crumbs.
    let mut g = small(2);
    // Holding there is the rule without `seek` (with it the soldier
    // walks on: `a_harvester_at_a_dry_crystal_goes_on_to_the_next`).
    g.rules.seek = false;
    let (k, c) = g.crystal_at(g.larder).unwrap();
    let d = &mut g.world;
    let s = d.p.scale as f32;
    let mut sum = 0.0;
    for q in 0..d.pile_cells.len() {
        let i = d.pile_cells[q] as usize;
        if d.pile_of[i] as usize == k + 1 {
            let (x, y) = ((i % d.w) as f32 + 0.5, (i / d.w) as f32 + 0.5);
            d.charge[i] = if (x - c.0).hypot(y - c.1) <= 3.0 * s { 20.0 } else { 0.0 };
            sum += d.charge[i];
        }
    }
    d.pile_charge[k] = sum;
    assert!(sum / g.rules.charge < 0.5 * g.rules.carry, "{sum} charge is not a short crystal");
    g.apply(Cmd::Order(0, Order::Clear(c.0, c.1)));
    g.apply(Cmd::Stance(0, Stance::HoldFire));
    for _ in 0..(g.rules.tps * 30) {
        g.step();
    }
    assert!(g.harvested >= 0.9 * sum / g.rules.charge, "banked {} of the crystal's {}, carrying {}", g.harvested, sum / g.rules.charge, g.soldiers[0].carry);
    assert!(g.soldiers[0].carry < 1.0, "still carrying {}", g.soldiers[0].carry);
    assert_eq!(g.soldiers[0].order, Order::Hold, "it holds at the larder");
    // With the rule off it waits on the crystal with its part load (W2b).
    let mut w = small(2);
    w.rules.home_dry = false;
    let d = &mut w.world;
    for q in 0..d.pile_cells.len() {
        let i = d.pile_cells[q] as usize;
        if d.pile_of[i] as usize == k + 1 {
            let (x, y) = ((i % d.w) as f32 + 0.5, (i / d.w) as f32 + 0.5);
            d.charge[i] = if (x - c.0).hypot(y - c.1) <= 3.0 * s { 20.0 } else { 0.0 };
        }
    }
    d.pile_charge[k] = sum;
    w.apply(Cmd::Order(0, Order::Clear(c.0, c.1)));
    w.apply(Cmd::Stance(0, Stance::HoldFire));
    for _ in 0..(w.rules.tps * 30) {
        w.step();
    }
    assert!(w.harvested == 0.0 && w.soldiers[0].carry > 0.5 * sum / w.rules.charge, "without the rule: banked {}, carrying {}", w.harvested, w.soldiers[0].carry);
}

#[test]
fn the_stalemate_scores_the_charge_taken_and_the_moulds_size() {
    // Score v2: a harvesting squad takes a share of the charge gone that
    // an idle one doesn't, and at one share a smaller mould scores
    // higher, inside the stalemate band. W2b's score is kept beside it.
    let mut idle = small(2);
    let mut farm = small(2);
    assert_eq!(farm.take_share(), 0.5, "nothing gone yet");
    let c = farm.larder;
    farm.apply(Cmd::Order(0, Order::Clear(c.0, c.1)));
    farm.apply(Cmd::Stance(0, Stance::HoldFire));
    for _ in 0..(farm.rules.tps * 20) {
        idle.step();
        farm.step();
    }
    let (i, f) = (idle.take_share(), farm.take_share());
    assert!(i < 0.01, "the idle squad took {i} of the charge gone");
    assert!(f > 0.2, "the harvesting squad took only {f}");
    let start = farm.start_mould;
    let score = |g: &mut Game<BigDish>, took: f32, mould: usize| {
        g.end = Some(End::Cap(took));
        g.mould = mould;
        g.cap_held = Some(0.5);
        g.score().unwrap()
    };
    let small_mould = score(&mut farm, 0.6, start / 4);
    let same = score(&mut farm, 0.6, start);
    let big_mould = score(&mut farm, 0.6, start * 3);
    assert!(small_mould > same && same > big_mould, "{small_mould} {same} {big_mould}");
    assert!((same - 0.05).abs() < 1e-6, "a mould its starting size adds nothing: {same}");
    assert!(score(&mut farm, 1.0, 1) < 0.5 && score(&mut farm, 0.0, start * 1000) > -0.5, "inside the stalemate band");
    assert_eq!(farm.score_w2b(), Some(0.0), "W2b's score is the held share less a half");
}

/// A world that gives a frame's results only when asked for them, as
/// the GPU world does, to test the step's halves on the CPU. Its
/// results are in on every third poll (`frame_ready`), so a paced
/// window finishes some steps when ready and the rest when the next
/// is due.
struct Later {
    d: BigDish,
    out: Option<FrameOut>,
    polls: std::cell::Cell<u32>,
    /// Every unit reads this many walkers on it (`None`: the dish's).
    touch: Option<u32>,
}

impl Later {
    fn touched(&self, mut o: FrameOut) -> FrameOut {
        if let Some(n) = self.touch {
            o.units.iter_mut().for_each(|u| u.touching = n);
        }
        o
    }
}

fn later(seed: u64) -> Game<Later> {
    // As `small`'s rules (buildings whole when placed).
    Game::new(Later { d: race_world(seed, 1, 1_000), out: None, polls: Default::default(), touch: None }, Rules { build: BuildRules { raise: 0.0, ..BuildRules::default() }, ..Rules::default() }, seed)
}

impl World for Later {
    fn size(&self) -> (usize, usize) {
        World::size(&self.d)
    }
    fn scale(&self) -> f32 {
        World::scale(&self.d)
    }
    fn ticks(&self) -> u64 {
        World::ticks(&self.d)
    }
    fn frame(&mut self, f: &Frame, ticks: u32) -> FrameOut {
        let o = World::frame(&mut self.d, f, ticks);
        self.touched(o)
    }
    fn frame_start(&mut self, f: &Frame, ticks: u32) -> Option<FrameOut> {
        assert!(self.out.is_none(), "a frame already running");
        self.out = Some(World::frame(&mut self.d, f, ticks));
        None
    }
    fn frame_finish(&mut self) -> FrameOut {
        let o = self.out.take().expect("a frame running");
        self.touched(o)
    }
    fn frame_ready(&self) -> bool {
        self.polls.set(self.polls.get() + 1);
        self.polls.get().is_multiple_of(3)
    }
    fn build_wall(&mut self, a: (f32, f32), b: (f32, f32), width: f32) {
        assert!(self.out.is_none(), "a wall built while a frame runs");
        World::build_wall(&mut self.d, a, b, width)
    }
    fn burst(&mut self, b: &squad::Burst, k: &squad::BurstRules) -> squad::BurstOut {
        World::burst(&mut self.d, b, k)
    }
    fn living(&self) -> usize {
        World::living(&self.d)
    }
    fn crystals(&self) -> Vec<((f32, f32), f32)> {
        World::crystals(&self.d)
    }
    fn crystal_r(&self, c: usize) -> f32 {
        World::crystal_r(&self.d, c)
    }
    fn landing(&self) -> Option<(f32, f32)> {
        World::landing(&self.d)
    }
    fn open(&self, x: f32, y: f32) -> bool {
        World::open(&self.d, x, y)
    }
    fn wall_cell(&self, i: usize) -> bool {
        World::wall_cell(&self.d, i)
    }
    fn stop_births(&mut self) {
        World::stop_births(&mut self.d)
    }
    fn upkeep(&self) -> f32 {
        World::upkeep(&self.d)
    }
    fn set_upkeep(&mut self, up: f32) {
        World::set_upkeep(&mut self.d, up)
    }
    fn set_panic_r(&mut self, r: f32) {
        World::set_panic_r(&mut self.d, r)
    }
    fn surge_every(&self) -> u32 {
        World::surge_every(&self.d)
    }
    fn set_surge_at(&mut self, at: u32) {
        World::set_surge_at(&mut self.d, at)
    }
    fn set_units_field(&mut self, field: Vec<u32>) {
        World::set_units_field(&mut self.d, field)
    }
    fn far_smell(&self) -> bool {
        World::far_smell(&self.d)
    }
    fn fresh_crystals(&self) -> Vec<u32> {
        World::fresh_crystals(&self.d)
    }
    fn set_food_field(&mut self, field: Vec<u32>) {
        World::set_food_field(&mut self.d, field)
    }
    fn set_raid(&mut self, at: u32, to: (f32, f32), r: f32, share: f32, field: Vec<u32>) {
        World::set_raid(&mut self.d, at, to, r, share, field)
    }
    fn raid_ground(&self) -> Option<(Vec<bool>, usize, usize)> {
        World::raid_ground(&self.d)
    }
    fn food_deposit(&self) -> f32 {
        World::food_deposit(&self.d)
    }
    fn fine_steps(&self) -> u32 {
        World::fine_steps(&self.d)
    }
    fn hash(&self) -> u64 {
        World::hash(&self.d)
    }
    fn calm(&self, at: &[(f32, f32)], r: f32) -> Vec<u32> {
        self.d.calm(at, r)
    }
    fn mould_grid(&self, bin: usize) -> Option<Vec<u32>> {
        self.d.mould_grid(bin)
    }
    fn form_lump(&mut self, b: usize, at: Option<((f32, f32), f32)>) {
        World::form_lump(&mut self.d, b, at)
    }
    fn grow_lump(&mut self, b: usize, at: ((f32, f32), f32)) {
        World::grow_lump(&mut self.d, b, at)
    }
    fn set_lump(&mut self, b: usize, l: crate::mould::big::Lump) {
        World::set_lump(&mut self.d, b, l)
    }
    fn lumps(&self) -> [Option<(f32, f32, u32)>; crate::mould::big::LUMPS] {
        World::lumps(&self.d)
    }
}

/// **Each rule that reads the walkers on a unit, at mass 1 and 0.2**
/// (the integration thread's audit, 2026-11-13; `Rules::mass` is 0.2 for
/// shape C's 25,000 walkers). Ten walkers on every unit for two frames:
/// - whole: a soldier's bite (`drain`), and its wading in the second
///   frame (`wade`: 10 on it is 1 / (1 + (10/6)²) of its open stride);
/// - by mass: the larder's and an outpost's steal, and a turret's wear;
/// - presence: the larder held (any walker at all);
/// - the AI's reads of the body (`larder_world_ai::View`) are whole
///   walkers, the same at either mass.
#[test]
fn walkers_count_whole_on_soldiers_and_by_mass_on_the_larder_and_buildings() {
    use crate::mould::larder_world_ai::{Side, View};
    let n = 10u32;
    let run = |mass: f32, touch: u32| {
        let mut g = later(1);
        (g.rules.mass, g.rules.overrun) = (mass, 0.0);
        let open = |g: &Game<Later>, p: (f32, f32)| {
            (0..40 * 16)
                .map(|i| {
                    let a = (i % 16) as f32 / 16.0 * std::f32::consts::TAU;
                    (p.0 + (i / 16) as f32 * a.cos(), p.1 + (i / 16) as f32 * a.sin())
                })
                .find(|q| g.world.open(q.0, q.1))
                .expect("open ground")
        };
        let (t, o) = (open(&g, (g.larder.0 + 8.0, g.larder.1)), open(&g, (g.larder.0 - 8.0, g.larder.1)));
        assert!(g.build(Kind::Turret, t) && g.build(Kind::Outpost, o));
        let to = (g.core.0, g.core.1);
        g.apply(Cmd::Order(0, Order::Move(to.0, to.1)));
        g.world.touch = Some(touch);
        let v = View::of(&g, &Side::whole(&g));
        let body: Vec<u32> = v.crystals.iter().flat_map(|c| [c.on, c.near]).collect();
        g.step();
        let at = (g.soldiers[0].x, g.soldiers[0].y);
        g.step();
        let stride = (g.soldiers[0].x - at.0).hypot(g.soldiers[0].y - at.1);
        (g.soldiers.iter().map(|s| s.hp).collect::<Vec<_>>(), stride, g.stolen, g.buildings[0].hp, g.buildings[0].max_hp, g.larder_touch, body)
    };
    let r = Rules::default();
    let dt = r.frame_ticks as f32 / r.tps as f32;
    let (_, open_stride, ..) = run(1.0, 0);
    assert!(open_stride > 0.0, "the soldier walks");
    let mut bodies = Vec::new();
    for mass in [1.0f32, 0.2] {
        let (hp, stride, stolen, turret, max, held, body) = run(mass, n);
        let bite = 2.0 * r.drain * n as f32 * dt;
        assert!(hp.iter().all(|&h| (r.hp - h - bite).abs() < 1e-3), "mass {mass}: whole bite {bite}, health {hp:?}");
        let slow = open_stride / (1.0 + (n as f32 / r.wade).powi(2));
        assert!((stride - slow).abs() < 0.02 * open_stride, "mass {mass}: whole wading, stride {stride} against {slow}");
        // The larder and the outpost, two frames each.
        let steal = 2.0 * 2.0 * r.steal * mass * n as f32 * dt;
        assert!((stolen - steal).abs() < 1e-3, "mass {mass}: stolen {stolen} against {steal}");
        let wear = 2.0 * r.build.drain * mass * n as f32 * dt;
        assert!((max - turret - wear).abs() < 1e-3, "mass {mass}: turret wear {} against {wear}", max - turret);
        assert_eq!(held, n, "the larder reads them, held at any mass");
        bodies.push(body);
    }
    assert_eq!(bodies[0], bodies[1], "the AI reads whole walkers at either mass");
}

/// **A step in halves plays as a whole one** (2026-11-12): the window
/// begins a step, draws, and finishes it on a later frame; a command
/// in between finishes it first. On a world that holds results back
/// until asked, that game equals `step`'s, command for command: the same
/// log (so the same replay), world hash, soldiers, bank and kills, with
/// sleep gas on the mould at a soldier (the toolkit's calm touch, read
/// in `finish_step` after the frame, as `step` read it).
#[test]
fn a_step_in_halves_plays_as_a_whole_one() {
    let seed = 5;
    let mut a = small(seed);
    let mut b = later(seed);
    let tps = a.rules.tps as u64;
    let (x, y) = a.larder;
    // Commands at game ticks: attack, buy, a wall, free fire, a buy.
    let cmd = |t: u64| match t {
        15 => vec![Cmd::Order(0, Order::Attack), Cmd::Order(1, Order::Attack)],
        40 => vec![Cmd::Buy],
        70 => vec![Cmd::Wall((x - 8.0, y - 4.0), (x - 8.0, y + 4.0))],
        100 => vec![Cmd::Stance(2, Stance::Free), Cmd::Order(2, Order::Attack)],
        300 => vec![Cmd::Buy, Cmd::Order(3, Order::Attack)],
        _ => Vec::new(),
    };
    let (mut running, mut gassed, mut calm) = (0, 0, 0);
    for n in 0..(tps * 40) {
        a.step();
        b.begin_step();
        assert!(b.in_flight() || b.end.is_some());
        assert!(!b.in_flight() || b.settled_ticks() == n * a.rules.frame_ticks as u64, "a running step's tick is the one it began on");
        for c in cmd(n + 1) {
            assert_eq!(a.apply(c), b.apply(c), "command {c:?} at {}", n + 1);
            assert!(!b.in_flight(), "a command finishes the running step");
        }
        // Sleep gas on the mould round the first soldier it touches
        // (the squad toolkit): the gassed walkers come off the touch in
        // `finish_step`. With 3 walkers on it, not 1: once aims had to
        // see their slime (2026-11-13), the first touch on seed 5 was one
        // walker on an attack-moving soldier, which walked on from the 5
        // it slept and left no calm touch to read; at 3 it is in the body.
        if let (0, Some(k)) = (gassed, a.soldiers.iter().position(|s| s.alive && s.touching >= 3)) {
            b.finish_step();
            let (x, y) = (a.soldiers[k].x, a.soldiers[k].y);
            let r = 4.0 * a.rules.touch * a.world.scale();
            gassed = a.world.gas(x, y, r, 600);
            assert_eq!(b.world.d.gas(x, y, r, 600), gassed);
        }
        if gassed > 0 {
            let at: Vec<(f32, f32)> = a.soldiers.iter().map(|s| (s.x, s.y)).collect();
            calm += a.world.gassed_near(&at, a.rules.touch * a.world.scale()).iter().sum::<u32>();
        }
        // Some steps finish at once, the rest when the next begins.
        if n % 3 == 0 {
            b.finish_step();
        } else if b.in_flight() {
            running += 1;
        }
    }
    b.finish_step();
    assert!(running > 300, "steps left running into the next: {running}");
    assert!(gassed > 0 && calm > 0, "the gas slept walkers ({gassed}) by the soldiers ({calm} calm touches)");
    assert_eq!(a.log, b.log, "the same commands at the same ticks");
    assert_eq!(a.world.hash(), b.world.hash(), "the same world");
    assert_eq!(format!("{:?}", a.soldiers), format!("{:?}", b.soldiers));
    assert_eq!((a.bank, a.killed, a.shots, a.harvested, a.walls, a.mould), (b.bank, b.killed, b.shots, b.harvested, b.walls, b.mould));
    assert!(a.killed > 0 && a.walls == 1 && a.soldiers.len() > 4, "the commands acted: killed {}, walls {}, soldiers {}", a.killed, a.walls, a.soldiers.len());
}

/// The window's pacing (`App::step` in `larder-world`) for `secs` game
/// seconds at `fps` frames a real second: each frame owes game time, a
/// running step finishes once its results are in or when the next step
/// is due, then the next begins. `click(g, from, to)` is the input of the
/// frame from real second `from` to `to` (the window's clicks); `due(g)`
/// runs just before each step begins (a paced replay's commands). Ends
/// on whole steps.
fn paced(g: &mut Game<Later>, fps: f32, secs: f32, mut click: impl FnMut(&mut Game<Later>, f32, f32), mut due: impl FnMut(&mut Game<Later>)) {
    let (dt, step) = (1.0 / fps, g.rules.frame_ticks as f32 / g.rules.tps as f32);
    let end = (secs * g.rules.tps as f32) as u64;
    let (mut owed, mut frame) = (0.0f32, 0u32);
    while g.world.ticks() < end && g.end.is_none() {
        click(g, frame as f32 * dt, (frame + 1) as f32 * dt);
        if g.in_flight() && g.step_ready() {
            g.finish_step();
        }
        owed += dt;
        while owed >= step && g.world.ticks() < end && g.end.is_none() {
            g.finish_step();
            due(g);
            g.begin_step();
            owed -= step;
        }
        frame += 1;
    }
    g.finish_step();
    due(g);
}

/// **A replay is the same game at any frame rate** (the technical lead's
/// ask on the step split, 2026-11-12): a command is logged at the game
/// tick it lands on, not the frame it was clicked in. Rounds played
/// through the window's pacing at 60 and 23 frames a second, with the
/// same clicks at the same real times, each replay (`Game::replay`, whole
/// steps) to their own world hash, bank and soldiers; and the 60 fps log
/// played back through the window's pacing at 23 and 144 frames a second
/// gives that hash too.
#[test]
fn a_replay_is_the_same_game_at_any_frame_rate() {
    let seed = 7;
    let probe = small(seed);
    let (x, y) = probe.larder;
    let clicks = [
        (1.0, vec![Cmd::Order(0, Order::Attack), Cmd::Order(1, Order::Attack)]),
        (2.71, vec![Cmd::Buy]),
        (4.33, vec![Cmd::Wall((x - 8.0, y - 4.0), (x - 8.0, y + 4.0))]),
        (6.1, vec![Cmd::Stance(2, Stance::Free), Cmd::Order(2, Order::Attack)]),
        (11.47, vec![Cmd::Buy, Cmd::Order(3, Order::Attack)]),
    ];
    let secs = 20.0;
    let record = |fps: f32| {
        let mut g = later(seed);
        paced(&mut g, fps, secs, |g, from, to| {
            for (_, cs) in clicks.iter().filter(|(t, _)| *t >= from && *t < to) {
                for &c in cs {
                    g.apply(c);
                }
            }
        }, |_| {});
        g
    };
    let same = |a: &Game<Later>, b_hash: u64, b_bank: f32, b_soldiers: &str, what: &str| {
        assert_eq!(a.world.hash(), b_hash, "{what}: the world hash");
        assert_eq!(a.bank.to_bits(), b_bank.to_bits(), "{what}: the bank");
        assert_eq!(format!("{:?}", a.soldiers), b_soldiers, "{what}: the soldiers");
    };
    let mut ticks = Vec::new();
    for fps in [60.0, 23.0] {
        let a = record(fps);
        assert_eq!(a.log.len(), 8, "{fps} fps: every click logged ({:?}, ended {:?})", a.log, a.end.map(|e| e.band()));
        assert!(a.walls == 1 && a.soldiers.len() > 4, "{fps} fps: the clicks acted");
        ticks.push(a.log.iter().map(|l| l.0).collect::<Vec<u64>>());
        let mut b = small(seed);
        assert!(Game::replay(&mut b, &a.replay_text(seed)));
        assert_eq!(a.world.ticks(), b.world.ticks(), "{fps} fps: the replay's end");
        same(&a, b.world.hash(), b.bank, &format!("{:?}", b.soldiers), &format!("recorded at {fps} fps, replayed in whole steps"));
        if fps == 60.0 {
            for back in [23.0, 144.0] {
                let mut c = later(seed);
                let mut next = 0;
                paced(&mut c, back, secs, |_, _, _| {}, |g| {
                    while next < a.log.len() && a.log[next].0 == g.world.ticks() {
                        g.apply(a.log[next].1);
                        next += 1;
                    }
                });
                assert_eq!(next, a.log.len(), "played back at {back} fps: every command");
                assert_eq!(c.log, a.log, "played back at {back} fps: the same log");
                same(&c, a.world.hash(), a.bank, &format!("{:?}", a.soldiers), &format!("played back at {back} fps"));
            }
        }
    }
    // 60 fps: [15, 15, 40, 64, 91, 91, 172, 172]; 23 fps: 14 and 171
    // for the first and last clicks (2026-11-12).
    assert_ne!(ticks[0], ticks[1], "the pacing put some clicks on other ticks");
}

#[test]
fn stuck_seconds_count_a_squad_that_can_do_nothing() {
    let mut g = small(1);
    g.bank = 0.0;
    // Shots that cost food: with free shots and charge left there is
    // always a way on (`a_walking_squad_or_a_standing_pump_is_not_stuck`).
    g.rules.price = 10.0;
    for k in 0..g.soldiers.len() {
        g.apply(Cmd::Stance(k, Stance::HoldFire));
    }
    for _ in 0..(g.rules.tps * 10) {
        g.step();
    }
    assert!(g.stuck >= 9 && g.stuck_run == g.stuck, "stuck {} run {}", g.stuck, g.stuck_run);
    // A bank that can buy a soldier is a way out.
    g.bank = g.rules.soldier_cost;
    for _ in 0..g.rules.tps {
        g.step();
    }
    assert_eq!(g.stuck_run, 0);
}

#[test]
fn a_walking_squad_or_a_standing_pump_is_not_stuck() {
    // Whole-game review W17 (11-15): a run ended "stuck" with four soldiers
    // at full health walking and two pumps standing.
    let stuck_for = |g: &mut Game<BigDish>| {
        g.bank = 0.0;
        g.rules.price = 10.0;
        for _ in 0..(g.rules.tps * 10) {
            g.step();
        }
        g.stuck_run
    };
    let hold = |g: &mut Game<BigDish>| {
        for k in 0..g.soldiers.len() {
            g.apply(Cmd::Stance(k, Stance::HoldFire));
        }
    };
    // Walking: orders across the dish and back, so they keep moving.
    let mut g = small(1);
    hold(&mut g);
    let (w, h) = g.world.size();
    let s0 = (g.soldiers[0].x, g.soldiers[0].y);
    let far = (w as f32 - s0.0, h as f32 - s0.1);
    g.apply(Cmd::Order(0, Order::Move(far.0, far.1)));
    assert_eq!(stuck_for(&mut g), 0, "a soldier walking is doing something");
    // A standing pump.
    let mut g = small(1);
    hold(&mut g);
    let at = (g.soldiers[0].x, g.soldiers[0].y);
    g.buildings.push(Building { kind: Kind::Pump, at, hp: 100.0, max_hp: 100.0, built: 1.0, alive: true, on: true, working: false, crystal: None, ready: 0, touching: 0, aim: None, dense: 0, drew: 0.0, level: 1, home: (0.0, 0.0), door: false, next: 0 });
    assert_eq!(stuck_for(&mut g), 0, "a standing pump is something");
    // Worn down, it no longer is.
    g.buildings[0].alive = false;
    assert!(stuck_for(&mut g) >= 9);
    // Free shots with charge left: never stuck, standing or not.
    let mut g = small(1);
    hold(&mut g);
    g.bank = 0.0;
    for _ in 0..(g.rules.tps * 10) {
        g.step();
    }
    assert_eq!(g.stuck_run, 0, "free shots and a charged crystal are a way on");
}

#[test]
fn the_campaign_switches_do_what_they_say() {
    // The reserve is off by default; the overrun is on since PR 4.
    let r = Rules::default();
    assert!(r.overrun == 20.0 && r.reserve == 0.0);
    // Overrun: the larder held with the bank empty for 2 s ends the round.
    let mut g = small(1);
    g.rules.overrun = 2.0;
    g.bank = 0.0;
    g.larder_touch = 3;
    for _ in 0..29 {
        g.campaign_rules(1);
    }
    assert!(g.end.is_none());
    g.campaign_rules(1);
    assert!(matches!(g.end, Some(End::Overrun(_))));
    assert_eq!(g.end.unwrap().band(), "larder overrun");
    assert!(g.end.unwrap().score(0, g.rules.cap) < -1.0, "an overrun is a loss");
    // Reserve: a wiped squad with an empty bank waits, and the larder
    // sends one soldier every 10 s until two stand.
    let mut g = small(1);
    g.rules.reserve = 10.0;
    g.bank = 0.0;
    for s in g.soldiers.iter_mut() {
        s.alive = false;
    }
    for _ in 0..(g.rules.tps * 25) {
        g.step();
    }
    assert!(g.end.is_none(), "{:?}", g.end);
    assert_eq!(g.alive(), 2);
    for _ in 0..(g.rules.tps * 15) {
        g.step();
    }
    assert_eq!(g.alive(), 2, "two soldiers stand, so no more come");
    // Without it the same squad has lost.
    let mut g = small(1);
    g.bank = 0.0;
    for s in g.soldiers.iter_mut() {
        s.alive = false;
    }
    g.step();
    assert!(matches!(g.end, Some(End::SquadWiped(_))));
}

/// **A run is lost when its last soldier falls** (owner, 2026-11-18
/// 15:56Z: he lost every soldier on planet 2 and they came back; "you
/// should lose if all your soliders die"): with a run's rules, food in the
/// bank for several soldiers and the reserve on, the round still ends at
/// once, and no soldier comes back after.
#[test]
fn a_runs_round_ends_when_its_last_soldier_falls() {
    let mut g = small(1);
    crate::mould::run::Run::new(1).rules(&mut g.rules);
    assert_eq!(g.rules.wipe_ends, 1.0, "a run turns it on");
    g.rules.reserve = 10.0;
    g.bank = 10.0 * g.rules.soldier_cost;
    for s in g.soldiers.iter_mut() {
        s.alive = false;
    }
    g.step();
    assert!(matches!(g.end, Some(End::SquadWiped(_))), "{:?}", g.end);
    assert!(!g.apply(Cmd::Buy) && g.alive() == 0, "a soldier was bought after the end");
    for _ in 0..(g.rules.tps * 15) {
        g.step();
    }
    assert_eq!(g.alive(), 0, "no recruit after the end");
    // Free play keeps the old rule: the same bank buys on.
    let mut g = small(1);
    g.bank = 10.0 * g.rules.soldier_cost;
    for s in g.soldiers.iter_mut() {
        s.alive = false;
    }
    g.step();
    assert!(g.end.is_none(), "{:?}", g.end);
}

/// No upkeep (owner, 2026-11-14: soldiers must not need feeding, 01:49Z;
/// "There should be no maintenance costs in this game", 04:53Z): a squad
/// standing idle for 20 s spends nothing and keeps its health, so the
/// bank moves only by what the mould eats from the larder. Rations and
/// hunger (`Rules::ration`, `Rules::hunger`) were removed for this.
#[test]
fn soldiers_cost_nothing_to_keep() {
    let mut g = small(7);
    g.rules.overrun = 0.0;
    for k in 0..g.soldiers.len() {
        g.apply(Cmd::Stance(k, Stance::HoldFire));
    }
    let (bank, n) = (g.bank, g.alive());
    let mut bitten = vec![false; g.soldiers.len()];
    // 20 s: the mould reaches the larder later than that on this seed.
    for _ in 0..(g.rules.tps * 20) {
        g.step();
        for (b, s) in bitten.iter_mut().zip(&g.soldiers) {
            *b |= s.touching > 0;
        }
    }
    assert_eq!((g.spent, g.harvested), (0.0, 0.0), "nothing bought, nothing banked");
    assert!((g.bank - (bank - g.stolen)).abs() < 0.1, "bank {} from {bank}, the mould ate {}", g.bank, g.stolen);
    assert_eq!(g.alive(), n, "the squad lived");
    // Health goes only to the mould's bite.
    let unbitten: Vec<f32> = g.soldiers.iter().zip(&bitten).filter(|(_, &b)| !b).map(|(s, _)| s.hp).collect();
    assert!(!unbitten.is_empty() && unbitten.iter().all(|&hp| hp == g.rules.hp), "unbitten soldiers' health {unbitten:?}");
}

/// A reserve recruit on an empty bank comes in whole and can harvest.
#[test]
fn a_reserve_recruit_comes_whole_and_harvests() {
    let mut g = small(7);
    (g.rules.reserve, g.rules.overrun, g.bank) = (10.0, 0.0, 0.0);
    for s in g.soldiers.iter_mut() {
        (s.alive, s.hp) = (false, 0.0);
    }
    while g.alive() == 0 && g.time() < 12.0 {
        g.step();
    }
    assert!(g.end.is_none() && g.alive() == 1, "a recruit came");
    let k = g.soldiers.iter().position(|s| s.alive).unwrap();
    assert_eq!(g.soldiers[k].hp, g.rules.hp, "whole");
    let target = g.seek(k, None).expect("a crystal with charge");
    g.apply(Cmd::Order(k, Order::Clear(target.0, target.1)));
    let t0 = g.time();
    while g.time() < t0 + 60.0 && g.soldiers[k].alive && g.bank <= 0.0 {
        g.step();
    }
    assert!(g.bank > 0.0 && g.soldiers[k].alive, "the recruit banked food");
}

#[test]
fn density_holds_from_small_worlds_to_big_ones() {
    // The same world at twice the scale has four times the open cells;
    // with four times the walkers its density is the same.
    let (a, b) = (small(1), Game::new(race_world(1, 2, 4_000), Rules::default(), 1));
    assert!(a.density > 0.0 && a.start_mould == 1_000 && b.start_mould == 4_000);
    assert!((b.density / a.density - 1.0).abs() < 0.05, "{} against {}", b.density, a.density);
    let (w, h) = a.world.size();
    let open = (0..w * h).filter(|&i| a.world.open((i % w) as f32 + 0.5, (i / w) as f32 + 0.5)).count();
    assert!(open < w * h, "rock and the rim are not open ground");
}

#[test]
fn soldiers_on_free_fire_kill_mould_and_pay_for_it() {
    // Shots are free by default (2026-11-12); a price still comes out
    // of the bank shot by shot.
    let mut g = small(3);
    g.rules.price = 10.0;
    let core = g.core;
    for k in 0..4 {
        g.apply(Cmd::Order(k, Order::Attack));
    }
    for _ in 0..(g.rules.tps * 60) {
        g.step();
        if g.end.is_some() {
            break;
        }
    }
    assert!(g.shots > 0 && g.killed > 0, "shots {} killed {}", g.shots, g.killed);
    assert!(g.spent > 0.0 && (g.spent - g.shots as f32 * g.rules.price).abs() < 1e-3);
    let _ = core;
}

#[test]
fn a_wall_costs_by_length_and_needs_a_soldier_near() {
    let mut g = small(4);
    let (x, y) = g.larder;
    let far = (g.core.0, g.core.1 + 30.0);
    // With `BuildRules::anywhere` (the default) a far wall is a ghost a
    // soldier walks to; the old rule wants both ends in range.
    g.rules.build.anywhere = false;
    assert!(!g.apply(Cmd::Wall(far, (far.0, far.1 + 10.0))), "a wall far from every soldier");
    g.rules.build.anywhere = true;
    let bank = g.bank;
    assert!(g.apply(Cmd::Wall((x - 10.0, y - 5.0), (x - 10.0, y + 5.0))));
    assert!((bank - g.bank - g.wall_price((x - 10.0, y - 5.0), (x - 10.0, y + 5.0))).abs() < 1e-3);
    assert!(!g.world.wall.is_empty() && g.world.wall.iter().any(|&w| w));
}

#[test]
fn same_commands_same_hash() {
    let mut a = small(5);
    let plan = |g: &mut Game<BigDish>| {
        let s = g.time() as u32;
        if s == 1 {
            let c = g.core;
            g.apply(Cmd::Order(0, Order::Move(c.0, c.1)));
            g.apply(Cmd::Stance(0, Stance::Free));
        }
        if s == 3 {
            g.apply(Cmd::Buy);
        }
    };
    for _ in 0..(a.rules.tps * 8) {
        if a.world.ticks().is_multiple_of(a.rules.tps as u64) {
            plan(&mut a);
        }
        a.step();
    }
    let text = a.replay_text(5);
    let mut b = small(5);
    assert!(Game::replay(&mut b, &text));
    assert_eq!(a.world.ticks(), b.world.ticks());
    assert_eq!(a.world.hash(), b.world.hash());
    assert_eq!(a.bank.to_bits(), b.bank.to_bits());
}

#[test]
fn queued_orders_run_in_turn_and_read_back_from_a_replay() {
    let mut g = small(6);
    let (x, y) = g.larder;
    let (a, b) = ((x - 8.0, y), (x - 8.0, y - 8.0));
    // Holding: a queued order starts at once; the next one waits.
    assert!(g.apply(Cmd::Queue(0, Order::Move(a.0, a.1))));
    assert!(g.apply(Cmd::Queue(0, Order::Fight(b.0, b.1))));
    assert_eq!(g.soldiers[0].order, Order::Move(a.0, a.1));
    assert_eq!(g.soldiers[0].queued, 1);
    for _ in 0..(g.rules.tps * 10) {
        g.step();
    }
    assert_eq!(g.soldiers[0].order, Order::Fight(b.0, b.1), "the move arrived and the queue moved on");
    assert_eq!(g.soldiers[0].queued, 0);
    // A plain order drops the queue.
    g.apply(Cmd::Queue(0, Order::Move(a.0, a.1)));
    g.apply(Cmd::Order(0, Order::Hold));
    assert_eq!(g.soldiers[0].queued, 0);
    g.apply(Cmd::Rally(Order::Clear(a.0, a.1)));
    g.bank = 1e6;
    assert!(g.apply(Cmd::Buy));
    assert_eq!(g.soldiers.last().unwrap().order, Order::Clear(a.0, a.1), "a bought soldier takes the rally order");
    for c in [Cmd::Rally(Order::Move(5.5, 6.0)), Cmd::Rally(Order::Hold), Cmd::Queue(3, Order::Fight(1.5, 2.25)), Cmd::Queue(1, Order::Clear(3.0, 4.0)), Cmd::Order(2, Order::Fight(7.0, 8.5))] {
        assert_eq!(Cmd::parse(&c.to_line()), Some(c), "{}", c.to_line());
    }
}

#[test]
fn a_harvester_at_a_dry_crystal_goes_on_to_the_next() {
    let mut g = small(7);
    g.rules.seek = true;
    // Soldier 0 is sent to a crystal; then that crystal's charge reads
    // as gone, so the next frame's take finds it dry.
    let target = g.seek(0, None).expect("a crystal with charge");
    let (i, _) = g.crystal_at(target).unwrap();
    g.apply(Cmd::Order(0, Order::Clear(target.0, target.1)));
    for _ in 0..(g.rules.tps * 60) {
        g.step();
        if g.soldiers[0].carry > 0.0 {
            break;
        }
    }
    assert!(g.soldiers[0].carry > 0.0, "it reached the crystal and took");
    let d = &mut g.world;
    for c in 0..d.charge.len() {
        if d.pile_of[c] as usize == i + 1 {
            d.charge[c] = 0.0;
        }
    }
    d.pile_charge[i] = 0.0;
    let mut went_home = false;
    for _ in 0..(g.rules.tps * 90) {
        g.step();
        went_home |= g.soldiers[0].homing;
        if let Order::Clear(x, y) = g.soldiers[0].order {
            if g.crystal_at((x, y)).map(|c| c.0) != Some(i) {
                break;
            }
        }
    }
    assert!(went_home, "it banked what it had first");
    let Order::Clear(x, y) = g.soldiers[0].order else { panic!("{:?}", g.soldiers[0].order) };
    let next = g.crystal_at((x, y)).unwrap().0;
    assert!(next != i && next != 0, "on to another crystal, not the mould's home: {next}");
    // With the rule off it holds at the larder, as before.
    let mut h = small(7);
    h.rules.seek = false;
    h.apply(Cmd::Order(0, Order::Clear(target.0, target.1)));
    h.after_dry(0, Some(i));
    assert_eq!(h.soldiers[0].order, Order::Hold);
}

#[test]
fn moving_on_skips_crystals_the_mould_ate_from_or_a_soldier_works() {
    let mut g = small(8);
    g.rules.seek = true;
    let first = g.seek(0, None).expect("a safe crystal");
    let (i, _) = g.crystal_at(first).unwrap();
    // Another soldier harvesting it: soldier 0 takes another.
    g.apply(Cmd::Order(1, Order::Clear(first.0, first.1)));
    let other = g.seek(0, None).expect("a second safe crystal");
    assert_ne!(other, first, "not the one soldier 1 works");
    // The mould ate a tenth of it: never chosen, even with no one on it.
    g.apply(Cmd::Order(1, Order::Hold));
    g.world.pile_charge[i] *= 0.9;
    let c = g.seek(0, None).expect("a safe crystal");
    assert_ne!(g.crystal_at(c).unwrap().0, i, "not a crystal the mould ate from");
    // Every pick is on the squad's side: nearer the larder than the core.
    let d = |a: (f32, f32), b: (f32, f32)| (a.0 - b.0).hypot(a.1 - b.1);
    assert!(d(c, g.larder) < d(c, g.core));
}

/// An aim spread (`Rules::spread`) turns each shot's line from dead on
/// by at most the spread at the target; with none, nothing is drawn, so
/// the same frame fires the same lines.
#[test]
fn an_aim_spread_turns_the_shots_within_the_spread() {
    let (mut a, mut b) = (small(3), small(3));
    // Dead on (the default until 11-15, now the rifle's 0.5) against 2.5.
    a.rules.spread = 0.0;
    b.rules.spread = 2.5;
    for g in [&mut a, &mut b] {
        for k in 0..g.soldiers.len() {
            g.apply(Cmd::Order(k, Order::Attack));
        }
    }
    let mut turned = 0;
    for _ in 0..15 * 60 {
        // A shot aims where its soldier's last read put the target.
        let aims: Vec<_> = a.soldiers.iter().map(|s| s.aim).collect();
        a.step();
        b.step();
        if !a.last_shots.is_empty() {
            // The first volley: the same soldiers fire from the same
            // places, each line as long, turned by up to the spread.
            assert_eq!(a.last_shots.len(), b.last_shots.len());
            let sc = a.s();
            for (p, q) in a.last_shots.iter().zip(&b.last_shots) {
                assert_eq!(p.from, q.from);
                let (u, v) = ((p.to.0 - p.from.0, p.to.1 - p.from.1), (q.to.0 - q.from.0, q.to.1 - q.from.1));
                assert!((u.0.hypot(u.1) - v.0.hypot(v.1)).abs() < 1e-3, "as long");
                let turn = (u.0 * v.1 - u.1 * v.0).atan2(u.0 * v.0 + u.1 * v.1).abs();
                let k = a.soldiers.iter().position(|s| (s.x, s.y) == p.from).expect("its soldier");
                let at = aims[k].expect("its target");
                let d = (at.0 - p.from.0).hypot(at.1 - p.from.1).max(0.1);
                assert!(turn <= (2.5 * sc / d).atan() + 1e-4, "turned {turn} at {d}");
                turned += (turn > 1e-4) as u32;
            }
            break;
        }
    }
    assert!(turned > 0, "the spread turned some shot");
}

/// Elbow room (`Rules::elbow`, art review 2026-11-14): a squad stacked on
/// one point fans out until every pair is the separation apart, never onto
/// unstandable ground; a squad sent to one cell ends that far apart too;
/// with the rule off a stack stays a stack.
#[test]
fn stacked_soldiers_step_apart_onto_open_ground() {
    let spread = |elbow: f32, send: bool| {
        let mut g = small(1);
        g.rules.elbow = elbow;
        while g.soldiers.len() < 8 {
            g.bank = g.bank.max(g.rules.soldier_cost);
            g.apply(Cmd::Buy);
        }
        let (x, y) = (g.soldiers[0].x, g.soldiers[0].y);
        let sc = g.s();
        let to = (0..16).map(|k| k as f32 / 16.0 * std::f32::consts::TAU).map(|a| (x + 12.0 * sc * a.cos(), y + 12.0 * sc * a.sin())).find(|&p| g.clear_line((x, y), p)).expect("open ground by the larder");
        for k in 0..g.soldiers.len() {
            g.apply(Cmd::Stance(k, Stance::HoldFire));
            (g.soldiers[k].x, g.soldiers[k].y) = (x, y);
            if send {
                g.apply(Cmd::Order(k, Order::Move(to.0, to.1)));
            }
        }
        for _ in 0..(g.rules.tps * 8) {
            g.step();
            for s in g.soldiers.iter().filter(|s| s.alive) {
                assert!(g.standable(s.x, s.y), "a soldier on unstandable ground at {:?}", (s.x, s.y));
            }
        }
        let live: Vec<_> = g.soldiers.iter().filter(|s| s.alive).map(|s| (s.x, s.y)).collect();
        assert!(live.len() >= 8, "the squad lived");
        let mut near = f32::MAX;
        for (i, a) in live.iter().enumerate() {
            for b in &live[i + 1..] {
                near = near.min((a.0 - b.0).hypot(a.1 - b.1));
            }
        }
        near / (g.rules.touch * sc)
    };
    for send in [false, true] {
        let near = spread(1.6, send);
        assert!(near >= 1.6 * 0.95, "sent {send}: nearest pair {near} touch radii apart");
    }
    assert!(spread(0.0, false) < 0.01, "off, the stack stays");
}

/// A burning soldier fires along its line once the line is in reach, and
/// while it still walks there it shoots what reaches it, by its own aim
/// and stance (2026-11-14: on shape C the `volley` squad, short of its
/// line, fired nothing for 2 s with up to 17 walkers on each soldier and
/// died). Defend still holds fire at slime that is neither on it nor near.
#[test]
fn a_soldier_walking_to_a_burn_line_shoots_what_reaches_it() {
    let mut g = small(1);
    let sc = g.s();
    let reach = g.rules.range * sc;
    let (x, y) = (g.soldiers[0].x, g.soldiers[0].y);
    let near = (x + 2.0 * sc, y);
    assert!(g.clear_line((x, y), near), "open ground by the larder");
    let set = |g: &mut Game<BigDish>, line: f32, touching: u32, aim: (f32, f32)| {
        let s = &mut g.soldiers[0];
        s.order = Order::Burn((x + line, y - sc), (x + line, y + sc));
        s.stance = Stance::Defend;
        s.touching = touching;
        s.aim = Some(aim);
        s.dense = 10;
    };
    // A line in reach: its points, whatever touches the soldier.
    set(&mut g, 0.5 * reach, 3, near);
    for _ in 0..5 {
        let p = g.target(0).expect("a line in reach is fired at");
        assert!((p.0 - (x + 0.5 * reach)).abs() < 1e-3, "{p:?}");
    }
    // A line far out of reach, walkers on the soldier: its own aim.
    set(&mut g, 4.0 * reach, 3, near);
    for _ in 0..5 {
        assert_eq!(g.target(0), Some(near), "a soldier on its way shoots what reaches it");
    }
    // Far line, nothing on it and its densest slime far off: Defend holds.
    let far = (x + 0.9 * reach, y);
    if g.clear_line((x, y), far) {
        set(&mut g, 4.0 * reach, 0, far);
        assert!(g.target(0).is_none(), "Defend does not fire at far slime on the way");
    }
}

// Squads (`squads`, owner 2026-11-14).

/// `small` in the squad mode.
fn squads(seed: u64) -> Game<BigDish> {
    let mut g = small(seed);
    g.rules.squads = true;
    g
}

/// Furthest any of squad `q` stands from its middle, dish cells.
fn spread_of(g: &Game<BigDish>, q: usize) -> f32 {
    let m = g.middle(q).unwrap();
    g.members(q).iter().map(|&k| (g.soldiers[k].x - m.0).hypot(g.soldiers[k].y - m.1)).fold(0.0, f32::max) / g.s()
}

/// Walks squad 0 off the crystal it starts round (its members start on
/// every side of it) onto open ground and lets it settle.
fn off_the_crystal(g: &mut Game<BigDish>) {
    let m = g.middle(0).unwrap();
    let sc = g.s();
    let to = (0..64)
        .map(|i| i as f32 / 64.0 * std::f32::consts::TAU)
        .map(|a| (m.0 + 25.0 * sc * a.cos(), m.1 + 25.0 * sc * a.sin()))
        .find(|&p| g.standable(p.0, p.1) && g.solid_at(p).is_none() && (0..8).all(|j| { let a = j as f32 * 0.785; g.standable(p.0 + 6.0 * sc * a.cos(), p.1 + 6.0 * sc * a.sin()) }))
        .expect("open ground near the start");
    g.apply(Cmd::SquadOrder(0, Order::Move(to.0, to.1)));
    for _ in 0..8 * g.rules.tps / g.rules.frame_ticks {
        g.step();
    }
}

#[test]
fn a_steered_squad_moves_together_and_settles_when_let_go() {
    let mut g = squads(2);
    for _ in 0..4 {
        g.apply(Cmd::Buy);
    }
    g.apply(Cmd::Active(0));
    off_the_crystal(&mut g);
    let settled = spread_of(&g, 0);
    // A blob of 8 has its last place gap x sqrt(7) out (`Game::gap`).
    assert!(settled < 3.0 * g.gap(), "a settled blob of 8 stands {settled} cells wide (gap {})", g.gap());
    let start = g.middle(0).unwrap();
    // Steer toward the mould's home for 6 s.
    let to = g.core;
    let d = ((to.0 - start.0), (to.1 - start.1));
    g.apply(Cmd::Steer(0, d));
    let mut worst: f32 = 0.0;
    for i in 0..6 * g.rules.tps / g.rules.frame_ticks {
        g.step();
        if i > 15 {
            worst = worst.max(spread_of(&g, 0));
        }
    }
    let moved = g.middle(0).unwrap();
    let gone = (moved.0 - start.0).hypot(moved.1 - start.1) / g.s();
    assert!(gone > 10.0, "the squad moved {gone} cells");
    assert!(worst < 4.5 * g.gap(), "the squad spread to {worst} cells while steered");
    g.apply(Cmd::Steer(0, (0.0, 0.0)));
    for _ in 0..3 * g.rules.tps / g.rules.frame_ticks {
        g.step();
    }
    let still = g.middle(0).unwrap();
    for _ in 0..g.rules.tps / g.rules.frame_ticks {
        g.step();
    }
    let after = g.middle(0).unwrap();
    assert!((after.0 - still.0).hypot(after.1 - still.1) < 0.5 * g.s(), "a let-go squad stays put");
}

#[test]
fn a_squad_sent_somewhere_goes_at_the_pace_of_its_slowest() {
    // Hold one member back as the slime would (its touch is set each
    // frame): the rest must not run ahead of it by much more than the lead.
    let mut g = squads(4);
    for _ in 0..3 {
        g.apply(Cmd::Buy);
    }
    let start = g.middle(0).unwrap();
    let dest = (start.0 + 40.0 * g.s(), start.1);
    let dest = if g.standable(dest.0, dest.1) { dest } else { (start.0 - 40.0 * g.s(), start.1) };
    g.apply(Cmd::SquadOrder(0, Order::Move(dest.0, dest.1)));
    g.rules.wade = 6.0;
    off_the_crystal(&mut g);
    let start = g.middle(0).unwrap();
    let dest = (start.0 + 40.0 * g.s(), start.1);
    let dest = if g.standable(dest.0, dest.1) { dest } else { (start.0 - 40.0 * g.s(), start.1) };
    g.apply(Cmd::SquadOrder(0, Order::Move(dest.0, dest.1)));
    let slow = g.members(0)[0];
    for _ in 0..3 * g.rules.tps / g.rules.frame_ticks {
        // Walkers on it each frame (`finish_step` reads the real touch
        // back after the walk): wade 6 holds it to a fifth of its speed.
        g.soldiers[slow].touching = 12;
        g.step();
        g.soldiers[slow].hp = g.rules.hp;
    }
    let m = g.middle(0).unwrap();
    let s = g.soldiers[slow];
    let gap = (m.0 - s.x).hypot(m.1 - s.y) / g.s();
    eprintln!("slow soldier {gap} cells from the middle; squad spread {}", spread_of(&g, 0));
    assert!(gap < STRAY_TEST, "the squad left its slow soldier {gap} cells behind");
}

const STRAY_TEST: f32 = 12.0;

#[test]
fn a_volley_fires_every_member_at_the_point_at_once_and_the_active_squad_faster() {
    let mut g = squads(5);
    for _ in 0..4 {
        g.apply(Cmd::Buy);
    }
    let m = g.middle(0).unwrap();
    let p = (m.0 + 10.0 * g.s(), m.1);
    g.apply(Cmd::Fire(0, Some(p)));
    g.step();
    let n = g.members(0).len();
    let fired: Vec<_> = g.last_shots.iter().filter(|s| s.fired && s.by.is_some()).collect();
    assert_eq!(fired.len(), n, "every member fires on the same step");
    // Each flies toward the point, within the rifle's aim spread.
    let sc = g.s();
    for s in &fired {
        let a = (p.1 - s.from.1).atan2(p.0 - s.from.0);
        let b = (s.to.1 - s.from.1).atan2(s.to.0 - s.from.0);
        let d = (p.0 - s.from.0).hypot(p.1 - s.from.1).max(0.1);
        let off = (g.gun(0).spread * sc / d).atan();
        assert!((a - b).abs() < 0.05 + off || s.hit.is_some(), "a volley shot off the aim");
    }
    // Volleys over 6 s, not controlled and controlled.
    let count = |g: &mut Game<BigDish>| {
        let mut v = 0;
        for _ in 0..6 * g.rules.tps / g.rules.frame_ticks {
            g.step();
            v += g.last_shots.iter().filter(|s| s.fired && s.by.is_some()).count();
        }
        v
    };
    let plain = count(&mut g);
    g.apply(Cmd::Active(0));
    let lead = count(&mut g);
    // Twice the rate, less the first wait on the old beat.
    assert!(lead as f32 >= 1.6 * plain as f32, "controlled {lead} shots against {plain}");
    // Let go of fire: no more volleys.
    g.apply(Cmd::Fire(0, None));
    assert!(g.squads[0].fire.is_none());
}

#[test]
fn split_join_and_merge_move_soldiers_between_squads() {
    let mut g = squads(6);
    for _ in 0..5 {
        g.apply(Cmd::Buy);
    }
    g.apply(Cmd::Active(0));
    off_the_crystal(&mut g);
    let n = g.members(0).len();
    let m = g.middle(0).unwrap();
    let p = (m.0 + 30.0, m.1);
    assert!(g.apply(Cmd::Split(0, p)));
    let b = g.active.unwrap();
    assert_ne!(b, 0, "control goes to the half split off");
    assert_eq!(g.members(0).len() + g.members(b).len(), n);
    assert_eq!(g.members(b).len(), n / 2);
    // The new half is the one nearer the point.
    let d = |q: usize| g.middle(q).map(|c| (c.0 - p.0).hypot(c.1 - p.1)).unwrap();
    assert!(d(b) <= d(0));
    // A bought soldier joins the controlled squad.
    g.bank = 1e6;
    g.apply(Cmd::Buy);
    assert_eq!(g.soldiers.last().unwrap().squad as usize, b);
    // Pull one out into a fresh squad, then merge everything back.
    let c = g.free_squad().unwrap();
    let k = g.members(0)[0];
    assert!(g.apply(Cmd::Join(k, c as u8)));
    assert_eq!(g.live_squads().len(), 3);
    assert!(g.apply(Cmd::Merge(0, b as u8)));
    assert!(g.apply(Cmd::Merge(0, c as u8)));
    assert_eq!(g.live_squads(), vec![0]);
    assert_eq!(g.active, Some(0), "merging the controlled squad keeps control");
    for _ in 0..4 * g.rules.tps / g.rules.frame_ticks {
        g.step();
    }
    assert!(spread_of(&g, 0) < 4.0 * g.gap(), "the merged squad gathers into one blob: {}", spread_of(&g, 0));
}

#[test]
fn formations_place_every_member_apart() {
    for f in Formation::ALL {
        for n in [1, 2, 5, 12, 30] {
            let pts: Vec<_> = (0..n).map(|r| f.offset(r, n, 1.6)).collect();
            for i in 0..n {
                for j in 0..i {
                    let d = (pts[i].0 - pts[j].0).hypot(pts[i].1 - pts[j].1);
                    assert!(d >= 0.9, "{f:?} with {n}: places {i} and {j} {d} apart");
                }
            }
        }
    }
}

#[test]
fn squad_commands_round_trip_as_lines() {
    let cs = [
        Cmd::Steer(2, (0.6, -0.8)),
        Cmd::SquadOrder(1, Order::Fight(3.5, 4.0)),
        Cmd::SquadOrder(0, Order::Clear(10.0, 2.0)),
        Cmd::Fire(3, Some((1.0, 2.0))),
        Cmd::Fire(3, None),
        Cmd::Face(0, (5.0, 6.0)),
        Cmd::Split(1, (7.0, 8.0)),
        Cmd::Join(12, 4),
        Cmd::Merge(0, 5),
        Cmd::Active(2),
        Cmd::Formation(1, Formation::Wedge),
        Cmd::SquadQueue(2, Order::Move(1.5, 2.5)),
        Cmd::SquadQueue(0, Order::Build(3)),
        Cmd::Link(0, 3),
        Cmd::Unlink(3),
        Cmd::Cover(1, Cover::More),
        Cmd::Cover(1, Cover::Never),
        Cmd::Arm(2, None),
        Cmd::Arm(1, Some(Gun::roll(11, 3))),
        Cmd::Arm(0, Some(crate::mould::guns::GunKind::Scatter.average())),
    ];
    for c in cs {
        assert_eq!(Cmd::parse(&c.to_line()), Some(c), "{}", c.to_line());
    }
}

/// The squad mode with no larder: drain or guard.
fn no_larder(seed: u64) -> Game<BigDish> {
    let mut g = squads(seed);
    (g.rules.larder, g.rules.seek, g.rules.overrun) = (false, false, 0.0);
    g
}

/// A crystal off the mould's home with charge, nearest the squad.
fn near_crystal(g: &Game<BigDish>) -> (usize, (f32, f32)) {
    let m = g.middle(0).unwrap();
    g.world.crystals().iter().enumerate().skip(1).filter(|c| c.1 .1 > 0.0).map(|(i, c)| (i, c.0)).min_by(|a, b| (a.1 .0 - m.0).hypot(a.1 .1 - m.1).total_cmp(&(b.1 .0 - m.0).hypot(b.1 .1 - m.1))).unwrap()
}

#[test]
fn with_no_larder_a_drained_crystal_banks_at_once_and_a_guarded_one_pays_without_running_down() {
    // Drain: soldiers harvesting bank what they take; nobody carries.
    let mut g = no_larder(7);
    let (i, c) = near_crystal(&g);
    g.apply(Cmd::SquadOrder(0, Order::Clear(c.0, c.1)));
    let bank0 = g.bank;
    for _ in 0..30 * g.rules.tps / g.rules.frame_ticks {
        g.step();
        assert!(g.soldiers.iter().all(|s| s.carry == 0.0 && !s.homing), "nothing is carried");
        if g.soldiers.iter().all(|s| matches!(s.order, Order::Clear(..))) {
            assert!(!g.guarded.contains(&i), "a crystal being drained is not guarded");
        }
    }
    assert!(g.harvested > 0.0 && g.bank > bank0, "draining banked {} (bank {bank0} -> {})", g.harvested, g.bank);
    // Guard: the same crystal held, nobody harvesting, pays each second and
    // keeps its charge.
    let mut g = no_larder(7);
    g.apply(Cmd::SquadOrder(0, Order::Move(c.0, c.1)));
    for _ in 0..20 * g.rules.tps / g.rules.frame_ticks {
        g.step();
    }
    let charge = g.world.crystals()[i].1;
    // Each second it is guarded (no walker on it) it pays a `guard`-th of
    // its charge.
    let (mut seconds, mut checked) = (0, 0);
    for _ in 0..20 * g.rules.tps / g.rules.frame_ticks {
        let paid0 = g.guard_paid;
        g.step();
        if g.guard_paid > paid0 && g.guarded == vec![i] {
            seconds += 1;
            let want = g.world.crystals()[i].1 / g.rules.charge / g.rules.guard;
            let got = g.guard_paid - paid0;
            assert!((got - want).abs() < 0.01 * want + 1e-4, "guarding paid {got} in a second, want {want}");
            checked += 1;
        }
    }
    assert!(seconds >= 5 && checked > 0, "the squad by the crystal guarded it {seconds} of 20 s");
    // The squad takes none of it (the mould may eat some).
    let _ = charge;
    assert_eq!(g.harvested, 0.0);
}

/// Owner's playtest (2026-11-14 04:46Z): "soldier get stuck on the wall
/// they build". Builders stood on the wall's line, so the risen wall shut
/// them in and the walking map from inside reached nothing; and one at a
/// wall's end was stepped diagonally across its corner, which `walk` then
/// refused, so it stood there for good. Now they stand beside it and step
/// round its ends.
#[test]
fn builders_stand_beside_their_wall_and_walk_on_once_it_stands() {
    let mut walled = 0;
    for seed in 1..6 {
        let mut g = Game::new(race_world(seed, 1, 1_000), Rules::default(), seed);
        let (x, y) = g.larder;
        let n = g.soldiers.len();
        // Across the squad's way west, so builders come up to its line.
        if !g.apply(Cmd::Wall((x - 12.0, y - 10.0), (x - 12.0, y + 10.0))) {
            continue;
        }
        let w = g.buildings.len() - 1;
        for k in 0..n {
            g.apply(Cmd::Order(k, Order::Build(w)));
        }
        let mut t = 0;
        while g.buildings[w].built < 1.0 && t < g.rules.tps * 60 {
            g.step();
            t += 1;
        }
        assert!(g.buildings[w].built >= 1.0, "seed {seed}: the wall rose");
        walled += 1;
        for (k, s) in g.soldiers.iter().enumerate() {
            assert!(g.standable(s.x, s.y), "seed {seed}: builder {k} shut in its wall at ({}, {})", s.x, s.y);
        }
        // West past the wall, round its ends.
        let to = (x - 25.0, y);
        for k in 0..n {
            g.apply(Cmd::Order(k, Order::Move(to.0, to.1)));
        }
        for _ in 0..(g.rules.tps * 20) {
            g.step();
        }
        for (k, s) in g.soldiers.iter().enumerate() {
            let d = (s.x - to.0).hypot(s.y - to.1);
            assert!(d < 12.0, "seed {seed}: soldier {k} {d} cells short at ({}, {})", s.x, s.y);
        }
    }
    assert!(walled >= 4, "{walled} of 5 seeds built the wall");
}

/// A soldier inside a wall (one that rose on it) walks out to the nearest
/// ground and goes where it is sent; one sent into rock or a wall stops at
/// the nearest ground instead of freezing where it stands.
#[test]
fn a_soldier_shut_in_a_wall_walks_out_and_a_move_into_one_stops_beside_it() {
    let mut g = small(2);
    let (x, y) = g.larder;
    let at = (x - 12.0, y);
    for k in 0..g.soldiers.len() {
        g.apply(Cmd::Order(k, Order::Hold));
    }
    g.soldiers[0].x = at.0;
    g.soldiers[0].y = at.1;
    g.wall_cells((at.0, at.1 - 6.0), (at.0, at.1 + 6.0));
    assert!(!g.world.open(at.0, at.1), "the soldier stands in the wall");
    g.apply(Cmd::Order(0, Order::Move(x - 24.0, y)));
    for _ in 0..(g.rules.tps * 15) {
        g.step();
    }
    let s = g.soldiers[0];
    assert!(g.standable(s.x, s.y) && (s.x - (x - 24.0)).hypot(s.y - y) < 4.0, "out and there: ({}, {})", s.x, s.y);
    // Sent into the wall's middle: it ends beside it, not frozen.
    let start = (s.x, s.y);
    g.apply(Cmd::Order(0, Order::Move(at.0, at.1)));
    for _ in 0..(g.rules.tps * 10) {
        g.step();
    }
    let s = g.soldiers[0];
    assert!(g.standable(s.x, s.y) && (s.x - at.0).hypot(s.y - at.1) < 4.0, "beside the wall: ({}, {}) from {start:?}", s.x, s.y);
}

/// Owner's playtest (2026-11-14 04:44Z): four soldiers sent to harvest a
/// far crystal walked back to the larder while the rest went. They had
/// been selected at the larder, so the window's kit gave each 5 tools,
/// and at 50 carry a tool (`KitRules::tool_weight`) they were full after
/// 50 food. Tools weigh nothing now, and nobody walks home empty-handed.
#[test]
fn a_soldier_holding_tools_goes_to_the_crystal_it_is_sent_to() {
    use crate::mould::kit::{Kit, KitRules};
    let mut g = small(3);
    let home = g.larder;
    let mut kit = Kit::new(KitRules::default());
    g.soldiers[0].x = home.0 + 2.0;
    g.soldiers[0].y = home.1 + 2.0;
    assert!(kit.restock(&mut g, 0, kit.rules.carry_max));
    assert_eq!(kit.held(0), 5);
    // A part load from its last crystal.
    g.soldiers[0].carry = 80.0;
    let far = g
        .world
        .crystals()
        .iter()
        .map(|c| c.0)
        .filter(|c| (c.0 - home.0).hypot(c.1 - home.1) > 30.0)
        .min_by(|a, b| (a.0 - home.0).hypot(a.1 - home.1).total_cmp(&(b.0 - home.0).hypot(b.1 - home.1)))
        .expect("a crystal away from the larder");
    let d0 = (g.soldiers[0].x - far.0).hypot(g.soldiers[0].y - far.1);
    g.apply(Cmd::Order(0, Order::Clear(far.0, far.1)));
    for _ in 0..(g.rules.tps * 3) {
        g.step();
    }
    let s = g.soldiers[0];
    let d = (s.x - far.0).hypot(s.y - far.1);
    assert!(!s.homing && d < d0 - 5.0, "walked {} toward the crystal, homing {}", d0 - d, s.homing);
    // A load heavier than the whole carry: an empty soldier still goes out
    // rather than shuttling to the larder and back.
    let mut g = small(3);
    g.soldiers[0].load = 2.0 * g.rules.carry;
    g.apply(Cmd::Order(0, Order::Clear(far.0, far.1)));
    for _ in 0..(g.rules.tps * 3) {
        g.step();
    }
    assert!(!g.soldiers[0].homing, "home with nothing to bank");
}

/// Owner's playtest (2026-11-14 08:01Z): "My unit got stuck and cannot
/// move anymore". A squad steered into rock or the map's edge sent its
/// anchor on in, more than `LEAD` ahead of soldiers who could not follow,
/// and the anchor then waited for them for good: steered back the other
/// way it moved 0 cells (15 of 32 squads, 8 headings on seeds 1-4, 25 s).
#[test]
fn a_squad_steered_into_rock_or_the_edge_comes_back_out() {
    let mut stuck = Vec::new();
    for seed in 1..4u64 {
        for dir in 0..8 {
            let mut g = squads(seed);
            for _ in 0..4 {
                g.apply(Cmd::Buy);
            }
            g.apply(Cmd::Active(0));
            off_the_crystal(&mut g);
            let sc = g.s();
            let a = dir as f32 * std::f32::consts::FRAC_PI_4;
            let d = (a.cos(), a.sin());
            let frames = |s: u32, g: &Game<BigDish>| s * g.rules.tps / g.rules.frame_ticks;
            g.apply(Cmd::Steer(0, d));
            for _ in 0..frames(20, &g) {
                g.step();
            }
            let pressed = g.middle(0).unwrap();
            assert!(g.squads[0].anchor.is_some_and(|p| g.world.open(p.0, p.1)), "seed {seed} heading {dir}: the anchor left the ground");
            g.apply(Cmd::Steer(0, (-d.0, -d.1)));
            for _ in 0..frames(4, &g) {
                g.step();
            }
            let back = g.middle(0).unwrap();
            let moved = (back.0 - pressed.0).hypot(back.1 - pressed.1) / sc;
            if moved < 5.0 {
                stuck.push((seed, dir, moved));
            }
        }
    }
    assert!(stuck.is_empty(), "squads that did not come back (seed, heading, cells): {stuck:?}");
}

/// Owner's playtest (2026-11-14 08:01Z): "The soldiers are constantly
/// shuffeling around in their formation". A formation turned with the
/// controlled squad's facing, which follows the cursor, and ranked its
/// places by soldier index: the cursor circling a resting blob walked its 8
/// soldiers 850 cells in 5 s over seeds 1-4, and a death 241 (a wedge
/// 216). Now a blob doesn't turn and every soldier takes the nearest free
/// place: 0 and 35 (a wedge's death 48). A line or wedge still turns to
/// face the cursor, as it should. Every squad must have arrived first: a
/// wedge sent round rock waited for its own tail (its middle stands behind
/// its anchor) and was still walking at 8 s on seed 3 and for good on
/// seed 4, so its member's death walked it 113 cells (2026-11-14, after
/// sent squads took the walking map round walls); now 36 and 22.
#[test]
fn a_formation_holds_its_places_when_the_cursor_circles_or_a_member_falls() {
    for f in [Formation::Blob, Formation::Wedge] {
        let (mut circling, mut death) = (0.0, 0.0);
        for seed in 1..5u64 {
            let mut g = squads(seed);
            for _ in 0..4 {
                g.apply(Cmd::Buy);
            }
            g.apply(Cmd::Active(0));
            g.apply(Cmd::Formation(0, f));
            off_the_crystal(&mut g);
            assert!(g.squads[0].dest.is_none(), "{f:?} seed {seed}: still walking 8 s after it was sent 25 cells");
            let sc = g.s();
            let frames = 5 * g.rules.tps / g.rules.frame_ticks;
            let m = g.middle(0).unwrap();
            let walked = |g: &mut Game<BigDish>, face: bool| {
                let mut w = 0.0;
                for i in 0..frames {
                    if face {
                        let a = i as f32 / 75.0 * std::f32::consts::TAU;
                        g.apply(Cmd::Face(0, (m.0 + 20.0 * sc * a.cos(), m.1 + 20.0 * sc * a.sin())));
                    }
                    let before: Vec<_> = g.soldiers.iter().map(|s| (s.x, s.y)).collect();
                    g.step();
                    w += g.soldiers.iter().zip(&before).filter(|(s, _)| s.alive).map(|(s, b)| (s.x - b.0).hypot(s.y - b.1)).sum::<f32>();
                }
                w / sc
            };
            if f == Formation::Blob {
                circling += walked(&mut g, true);
            }
            let k = g.members(0)[0];
            g.soldiers[k].alive = false;
            death += walked(&mut g, false);
        }
        assert!(circling < 1.0, "{f:?}: the cursor circling walked the squad {circling} cells");
        assert!(death < 100.0, "{f:?}: a death walked the squad {death} cells");
    }
}

/// The `Range` card (owner, 2026-11-14 08:39Z): slime between the plain
/// range and the stretched one is out of reach without the card and in
/// reach with it, and a fired leg is as long as the stretched range.
#[test]
fn a_range_card_lets_soldiers_engage_further_and_shoot_further() {
    let mut g = small(1);
    let sc = g.s();
    let (x, y) = (g.soldiers[0].x, g.soldiers[0].y);
    let d = 1.25 * g.rules.range * sc;
    let far = [(1.0, 0.0), (-1.0, 0.0), (0.0, 1.0), (0.0, -1.0), (0.7, 0.7), (-0.7, 0.7), (0.7, -0.7), (-0.7, -0.7)]
        .into_iter()
        .map(|(u, v): (f32, f32)| (x + u * d, y + v * d))
        .find(|&p| g.clear_line((x, y), p))
        .expect("an open line 30 cells out");
    {
        let s = &mut g.soldiers[0];
        s.order = Order::Fight(far.0, far.1);
        s.aim = Some(far);
        s.dense = 50;
    }
    assert!(!g.engaged(0), "30 cells is past the plain range");
    g.bullet_mods = BulletMods { reach: 1, ..BulletMods::default() };
    assert!((g.gun_range(0) - 48.0).abs() < 1e-4, "one card doubles the range (owner: 2-4x)");
    assert!(g.engaged(0), "one Range card reaches 48 cells");
    // Fired legs: free fire with the card flies out to 48 cells.
    let mut g = small(3);
    g.bullet_mods = BulletMods { reach: 1, ..BulletMods::default() };
    for k in 0..g.soldiers.len() {
        g.apply(Cmd::Order(k, Order::Attack));
    }
    let reach = g.gun_range(0) * g.s();
    let mut longest: f32 = 0.0;
    for _ in 0..(30 * g.rules.tps) {
        g.step();
        assert!(g.bullets.iter().all(|b| b.left <= reach + 1.0));
        longest = g.bullets.iter().map(|b| b.left).fold(longest, f32::max);
        if g.end.is_some() {
            break;
        }
    }
    assert!(longest > g.rules.range * g.s(), "a bullet flew past the plain range: {longest} of {reach}");
}

// Building (2026-11-14, owner 08:10Z: queue builds; start raising once in
// range instead of walking to the ghost; keep the soldiers on one side of
// a turret raised by the enemy).

fn frames(g: &Game<BigDish>, secs: f32) -> u32 {
    (secs * g.rules.tps as f32 / g.rules.frame_ticks as f32) as u32
}

/// Standable ground near `p` with room round it (6 dish cells clear).
fn room_near(g: &Game<BigDish>, p: (f32, f32)) -> (f32, f32) {
    let sc = g.s();
    (0..40)
        .flat_map(|r| (0..16).map(move |k| (r, k)))
        .map(|(r, k)| {
            let a = k as f32 / 16.0 * std::f32::consts::TAU;
            (p.0 + r as f32 * sc * a.cos(), p.1 + r as f32 * sc * a.sin())
        })
        .find(|&q| g.build_blocked(Kind::Turret, q).is_none() && (0..8).all(|j| { let a = j as f32 * 0.785; g.standable(q.0 + 6.0 * sc * a.cos(), q.1 + 6.0 * sc * a.sin()) }))
        .expect("open ground")
}

#[test]
fn builders_raise_from_reach_and_stay_on_the_side_they_came_from() {
    let mut g = small(15);
    g.rules.start_bank = 1e6;
    g.bank = 1e6;
    g.rules.build.raise = 6.0;
    for _ in 0..4 {
        g.apply(Cmd::Buy);
    }
    let sc = g.s();
    let from = g.larder;
    let dir = {
        let (dx, dy) = (g.core.0 - from.0, g.core.1 - from.1);
        let d = dx.hypot(dy);
        (dx / d, dy / d)
    };
    let at = room_near(&g, (from.0 + dir.0 * 30.0 * sc, from.1 + dir.1 * 30.0 * sc));
    assert!(g.apply(Cmd::Build(Kind::Turret, at)));
    let n = g.soldiers.len();
    for k in 0..n {
        g.apply(Cmd::Order(k, Order::Build(0)));
    }
    let reach = g.rules.build.reach * sc;
    let mut first = None;
    for _ in 0..frames(&g, 60.0) {
        g.step();
        if first.is_none() && g.buildings[0].built > 0.0 {
            // The nearest builder when it began: about `reach` out, not at the ghost.
            first = g.soldiers.iter().filter(|s| s.alive).map(|s| (s.x - at.0).hypot(s.y - at.1)).reduce(f32::min);
        }
        if g.buildings[0].up() {
            break;
        }
    }
    assert!(g.buildings[0].up(), "the squad raised it");
    let first = first.unwrap();
    assert!(first > reach - 1.5 * sc, "raising began {:.1} cells out (reach {})", first / sc, g.rules.build.reach);
    // Every builder on the side it came from (toward the larder), none
    // past the ghost or round it toward the mould, and none on it.
    let back = (from.0 - at.0, from.1 - at.1);
    for (k, s) in g.soldiers.iter().enumerate().filter(|(_, s)| s.alive) {
        let d = (s.x - at.0).hypot(s.y - at.1);
        let side = ((s.x - at.0) * back.0 + (s.y - at.1) * back.1) / back.0.hypot(back.1);
        assert!(side > 0.0, "soldier {k} stands {:.1} cells on the far side", -side / sc);
        assert!(d >= (g.rules.build.touch + 1.0) * sc - 0.5, "soldier {k} on the ghost ({:.1} cells)", d / sc);
    }
}

#[test]
fn shift_queued_builds_rise_in_turn_by_one_builder() {
    let mut g = small(16);
    g.bank = 1e6;
    g.rules.build.raise = 2.0;
    let sc = g.s();
    let (x, y) = g.larder;
    for k in 1..g.soldiers.len() {
        g.apply(Cmd::Order(k, Order::Move(x - 30.0 * sc, y)));
    }
    let mut sites = Vec::new();
    for i in 0..3 {
        let at = room_near(&g, (x + (12.0 + 8.0 * i as f32) * sc, y + 10.0 * sc));
        assert!(g.apply(Cmd::Build(Kind::Turret, at)), "site {i}");
        assert!(g.apply(Cmd::Queue(0, Order::Build(i))));
        sites.push(at);
    }
    assert_eq!((g.soldiers[0].order, g.soldiers[0].queued), (Order::Build(0), 2), "the first at once, two queued");
    // The others go far off, out of the idle soldiers' pickup range.
    let mut done = Vec::new();
    for _ in 0..frames(&g, 60.0) {
        g.step();
        for b in 0..3 {
            if g.buildings[b].up() && !done.contains(&b) {
                done.push(b);
            }
        }
        if done.len() == 3 {
            break;
        }
    }
    assert_eq!(done, vec![0, 1, 2], "raised in the order queued");
    g.step();
    assert_eq!(g.soldiers[0].order, Order::Hold, "then it holds");
}

#[test]
fn a_squad_sent_off_keeps_a_build_queued_for_when_it_gets_there() {
    let mut g = squads(17);
    g.bank = 1e6;
    g.rules.build.raise = 2.0;
    g.apply(Cmd::Active(0));
    off_the_crystal(&mut g);
    let sc = g.s();
    let m = g.middle(0).unwrap();
    let to = (0..32)
        .map(|i| i as f32 / 32.0 * std::f32::consts::TAU)
        .map(|a| room_near(&g, (m.0 + 20.0 * sc * a.cos(), m.1 + 20.0 * sc * a.sin())))
        .find(|q| (q.0 - m.0).hypot(q.1 - m.1) > 15.0 * sc)
        .expect("somewhere to send it");
    g.apply(Cmd::SquadOrder(0, Order::Move(to.0, to.1)));
    g.step();
    let at = room_near(&g, (to.0, to.1 + 9.0 * sc));
    assert!(g.apply(Cmd::Build(Kind::Turret, at)));
    for k in g.members(0) {
        assert!(g.apply(Cmd::Queue(k, Order::Build(0))));
    }
    for _ in 0..3 {
        g.step();
    }
    assert!(g.squads[0].dest.is_some(), "still on its way");
    assert!(g.members(0).iter().all(|&k| g.soldiers[k].queued == 1), "the build stays queued while it walks");
    for _ in 0..frames(&g, 30.0) {
        g.step();
        if g.buildings[0].up() {
            break;
        }
    }
    assert!(g.buildings[0].up(), "the queued build rose once the squad got there");
    // And a build placed without Shift while it walks stops the walk.
    let far = room_near(&g, (to.0 - 25.0 * sc, to.1));
    g.apply(Cmd::SquadOrder(0, Order::Move(far.0, far.1)));
    g.step();
    let at2 = room_near(&g, (to.0 + 6.0 * sc, to.1 - 9.0 * sc));
    assert!(g.apply(Cmd::Build(Kind::Turret, at2)));
    assert!(g.apply(Cmd::SquadOrder(0, Order::Build(1))));
    for _ in 0..frames(&g, 30.0) {
        g.step();
        if g.buildings[1].up() {
            break;
        }
    }
    assert!(g.buildings[1].up(), "the squad turned to the build");
}

#[test]
fn a_ghost_called_off_pays_back_its_price_and_its_builders_move_on() {
    let mut g = small(18);
    g.bank = 5_000.0;
    g.rules.build.raise = 20.0;
    let sc = g.s();
    let (x, y) = g.larder;
    let a = room_near(&g, (x + 12.0 * sc, y));
    assert!(g.apply(Cmd::Build(Kind::Turret, a)));
    let b = room_near(&g, (x - 12.0 * sc, y));
    assert!(g.apply(Cmd::Build(Kind::Turret, b)));
    assert!(g.apply(Cmd::Queue(0, Order::Build(0))));
    assert!(g.apply(Cmd::Queue(0, Order::Build(1))));
    for _ in 0..frames(&g, 6.0) {
        g.step();
    }
    assert!(g.buildings[0].built > 0.0 && !g.buildings[0].up(), "half up: {}", g.buildings[0].built);
    let before = g.bank;
    assert_eq!(g.ghost_at((a.0 + 0.5 * sc, a.1)), Some(0));
    assert!(g.apply(Cmd::Unbuild(0)));
    assert_eq!(g.bank, before + g.rules.build.turret_cost, "the whole price back");
    assert!(!g.buildings[0].alive);
    assert_eq!(g.ghost_at(a), None, "gone");
    assert!(!g.apply(Cmd::Unbuild(0)), "only once");
    g.step();
    assert_eq!(g.soldiers[0].order, Order::Build(1), "its builder goes on down its queue");
    assert_eq!(Cmd::parse(&Cmd::Unbuild(3).to_line()), Some(Cmd::Unbuild(3)));
    // A standing building is not sold back.
    g.rules.build.raise = 0.0;
    let c = room_near(&g, (x, y + 12.0 * sc));
    assert!(g.apply(Cmd::Build(Kind::Turret, c)));
    let n = g.buildings.len() - 1;
    assert!(g.buildings[n].up());
    assert!(!g.apply(Cmd::Unbuild(n)));
    // ... it is pulled down instead, for half its price (`Cmd::Demolish`,
    // the building card; half, owner 2026-11-15 21:37Z).
    assert_eq!(g.building_at(c), Some(n), "a click on it finds it, standing");
    let before = g.bank;
    assert!(g.apply(Cmd::Demolish(n)));
    assert_eq!(g.bank, before + 0.5 * g.rules.build.turret_cost);
    assert!(!g.buildings[n].alive && !g.apply(Cmd::Demolish(n)), "once");
    assert_eq!(Cmd::parse(&Cmd::Demolish(4).to_line()), Some(Cmd::Demolish(4)));
    // A wall stands in the world's cells: not pulled down.
    let w0 = room_near(&g, (x - 12.0 * sc, y + 12.0 * sc));
    assert!(g.apply(Cmd::Wall(w0, (w0.0 + 4.0 * sc, w0.1))));
    let w = g.buildings.len() - 1;
    assert!(g.buildings[w].up() && !g.apply(Cmd::Demolish(w)));
}

/// **Guard** (owner 2026-11-15 19:51Z, in Burn a line's place): a soldier
/// guarding a spot shoots slime in its zone that defend alone would leave,
/// and its zone round a crystal is the crystal's rim and `GUARD_RIM`.
#[test]
fn a_guard_shoots_slime_on_what_it_guards() {
    let mut g = small(1);
    let sc = g.s();
    let s0 = g.soldiers[0];
    // Slime 10 cells off: past defend's 7.5, inside a guard zone round it.
    let slime = (s0.x + 10.0 * sc, s0.y);
    let set = |g: &mut Game<BigDish>| {
        let s = &mut g.soldiers[0];
        s.aim = Some(slime);
        s.dense = 50;
        s.touching = 0;
    };
    g.soldiers[0].order = Order::Hold;
    set(&mut g);
    assert!(g.clear_line((s0.x, s0.y), slime), "open ground by the larder");
    assert_eq!(g.target(0), None, "defend leaves slime 10 cells off");
    g.soldiers[0].order = Order::Guard(slime.0 + 2.0 * sc, slime.1);
    set(&mut g);
    assert_eq!(g.target(0), Some(slime), "a guard shoots it on the spot it guards");
    g.soldiers[0].stance = Stance::HoldFire;
    assert_eq!(g.target(0), None, "not on hold fire");
    // Round a crystal: its rim and GUARD_RIM.
    let (c, _) = g.world.crystals()[0];
    let ((m, r), i) = (g.zone_at(c), 0);
    assert_eq!(m, c);
    assert!((r - (g.world.crystal_r(i) + GUARD_RIM * sc)).abs() < 1e-3);
    assert_eq!(Cmd::parse(&Cmd::Order(2, Order::Guard(1.5, 3.0)).to_line()), Some(Cmd::Order(2, Order::Guard(1.5, 3.0))));
}

/// **A squad sent past a dead end goes round it** (2026-11-14, the squad
/// thread's film: a squad sent from inside the Fork's V wall to a crystal
/// beyond it stood at the wall for good). A cup of walls opens toward the
/// squad with the point it is sent to beyond it: the anchor's straight
/// step led into the cup and its path step back out, turn about.
#[test]
fn a_squad_sent_past_a_dead_end_goes_round_it() {
    let mut tried = Vec::new();
    for seed in 1..5u64 {
        let mut g = squads(seed);
        g.rules.build.raise = 0.0;
        for _ in 0..4 {
            g.apply(Cmd::Buy);
        }
        off_the_crystal(&mut g);
        g.bank = 1e6;
        let sc = g.s();
        let m = g.middle(0).unwrap();
        let mut sent = None;
        for dir in 0..8 {
            let a = dir as f32 * std::f32::consts::FRAC_PI_4;
            let (d, p) = ((a.cos(), a.sin()), (-a.sin(), a.cos()));
            let at = |f: f32, s: f32| (m.0 + (d.0 * f + p.0 * s) * sc, m.1 + (d.1 * f + p.1 * s) * sc);
            let to = at(30.0, 0.0);
            if !g.standable(to.0, to.1) || g.solid_at(to).is_some() {
                continue;
            }
            let n = g.buildings.len();
            let walls = [(at(14.0, -8.0), at(14.0, 8.0)), (at(6.0, -8.0), at(14.0, -8.0)), (at(6.0, 8.0), at(14.0, 8.0))];
            if walls.iter().all(|&(a, b)| g.apply(Cmd::Wall(a, b))) {
                sent = Some(to);
                break;
            }
            // Not all three stood: take down what did and try the next way.
            for b in &mut g.buildings[n..] {
                b.alive = false;
            }
        }
        let Some(to) = sent else { continue };
        g.apply(Cmd::SquadOrder(0, Order::Move(to.0, to.1)));
        let mut best = f32::MAX;
        for _ in 0..40 * g.rules.tps / g.rules.frame_ticks {
            g.step();
            let mid = g.middle(0).unwrap();
            best = best.min((mid.0 - to.0).hypot(mid.1 - to.1) / sc);
        }
        tried.push((seed, best));
    }
    assert!(tried.len() >= 2, "the cup stood on too few seeds: {tried:?}");
    assert!(tried.iter().all(|&(_, best)| best < 6.0), "(seed, nearest the squad came in cells): {tried:?}");
}

/// The `Rate` card (owner's notes, 11-14 15:06Z: "2-4x ... rate of
/// fire"): each copy fires every soldier's gun once more as often.
#[test]
fn a_rate_card_fires_twice_as_often() {
    let mut g = small(1);
    let plain = g.shot_gap(0);
    g.bullet_mods = BulletMods { rate: 1, ..BulletMods::default() };
    assert_eq!(g.shot_gap(0), (plain as f32 / 2.0).round().max(1.0) as u64);
    g.bullet_mods = BulletMods { rate: 3, ..BulletMods::default() };
    assert_eq!(g.shot_gap(0), (plain as f32 / 4.0).round().max(1.0) as u64, "three cards, four times");
}

/// Open ground `d` dish cells from squad 0's middle.
fn open_near(g: &Game<BigDish>, d: f32) -> (f32, f32) {
    let m = g.middle(0).unwrap();
    let sc = g.s();
    (0..64)
        .map(|i| i as f32 / 64.0 * std::f32::consts::TAU)
        .map(|a| (m.0 + d * sc * a.cos(), m.1 + d * sc * a.sin()))
        .find(|&p| g.standable(p.0, p.1) && g.solid_at(p).is_none())
        .expect("open ground")
}

#[test]
fn a_squad_does_its_queued_orders_in_turn() {
    let mut g = squads(7);
    for _ in 0..3 {
        g.apply(Cmd::Buy);
    }
    off_the_crystal(&mut g);
    let a = open_near(&g, 14.0);
    let b = open_near(&g, 26.0);
    g.apply(Cmd::SquadOrder(0, Order::Move(a.0, a.1)));
    assert!(g.apply(Cmd::SquadQueue(0, Order::Move(b.0, b.1))));
    assert_eq!(g.squads[0].queued, 1);
    let mut went_a = false;
    for _ in 0..16 * g.rules.tps / g.rules.frame_ticks {
        g.step();
        let m = g.middle(0).unwrap();
        went_a |= (m.0 - a.0).hypot(m.1 - a.1) < 3.0 * g.s();
    }
    let m = g.middle(0).unwrap();
    assert!(went_a, "the squad never reached its first point");
    assert_eq!(g.squads[0].queued, 0, "the queued move started");
    assert!((m.0 - b.0).hypot(m.1 - b.1) < 4.0 * g.s(), "the squad stands {} cells from its queued point", (m.0 - b.0).hypot(m.1 - b.1) / g.s());
    // A plain order drops the queue; an idle squad starts a queued one at once.
    g.apply(Cmd::SquadQueue(0, Order::Move(a.0, a.1)));
    assert_eq!((g.squads[0].queued, g.squads[0].dest), (0, Some(a)));
    g.apply(Cmd::SquadQueue(0, Order::Move(b.0, b.1)));
    g.apply(Cmd::SquadOrder(0, Order::Move(a.0, a.1)));
    assert_eq!(g.squads[0].queued, 0);
}

#[test]
fn linked_squads_take_orders_together_and_keep_their_own_settings() {
    let mut g = squads(8);
    for _ in 0..5 {
        g.apply(Cmd::Buy);
    }
    g.apply(Cmd::Active(0));
    off_the_crystal(&mut g);
    let m = g.middle(0).unwrap();
    assert!(g.apply(Cmd::Split(0, (m.0 + 30.0, m.1))));
    let b = g.active.unwrap();
    assert_eq!(g.squads[b].link, NO_LINK, "a split half starts unlinked");
    assert!(!g.leads(0), "the half left behind has no lead edge");
    assert!(g.apply(Cmd::Link(b as u8, 0)));
    assert_eq!(g.linked(0), vec![0, b]);
    assert!(g.leads(0), "linked with the controlled squad: the lead's edge");
    // Settings stay each squad's own.
    g.apply(Cmd::Formation(0, Formation::Line));
    g.apply(Cmd::Cover(b as u8, Cover::Never));
    assert_eq!((g.squads[0].formation, g.squads[b].formation), (Formation::Line, Formation::Blob));
    assert_eq!((g.squads[0].cover, g.squads[b].cover), (Cover::Some, Cover::Never));
    // One order moves both, each by where it stands in the link.
    let (m0, mb) = (g.middle(0).unwrap(), g.middle(b).unwrap());
    let to = open_near(&g, 20.0);
    g.apply(Cmd::SquadOrder(b as u8, Order::Move(to.0, to.1)));
    let (d0, db) = (g.squads[0].dest.unwrap(), g.squads[b].dest.unwrap());
    let apart = |p: (f32, f32), q: (f32, f32)| (p.0 - q.0, p.1 - q.1);
    let (was, now) = (apart(m0, mb), apart(d0, db));
    assert!((was.0 - now.0).abs() < 1e-3 && (was.1 - now.1).abs() < 1e-3, "the link keeps its shape: {was:?} then {now:?}");
    g.apply(Cmd::Steer(0, (1.0, 0.0)));
    assert_eq!(g.squads[b].steer, (1.0, 0.0), "steering one steers its link");
    g.apply(Cmd::Steer(0, (0.0, 0.0)));
    // A link is commanded as one: no picking a squad inside it.
    assert!(!g.apply(Cmd::Active(0)));
    assert_eq!(g.active, Some(b));
    // Unlinked, an order moves one only.
    assert!(g.apply(Cmd::Unlink(0)));
    assert!(g.apply(Cmd::Active(0)), "unlinked, it can be commanded alone");
    assert_eq!((g.squads[0].link, g.squads[b].link), (NO_LINK, NO_LINK));
    g.apply(Cmd::SquadOrder(0, Order::Move(to.0, to.1)));
    assert_eq!(g.squads[0].dest, Some(to));
    assert_ne!(g.squads[b].dest, Some(to));
}

#[test]
fn a_working_squad_posts_defenders_when_slime_is_near_and_works_a_little_slower() {
    let mut g = no_larder(9);
    g.bank = 1e6;
    while g.members(0).len() < 8 {
        assert!(g.apply(Cmd::Buy));
    }
    let (_, c) = near_crystal(&g);
    let n = g.members(0).len();
    assert_eq!(n, 8);
    // Any slime counts as near (the dish's own may be far from this crystal).
    g.rules.cover.near = 0;
    g.apply(Cmd::SquadOrder(0, Order::Clear(c.0, c.1)));
    assert_eq!(g.squads[0].work, Some(Order::Clear(c.0, c.1)));
    let guards = |g: &Game<BigDish>| g.members(0).into_iter().filter(|&k| g.soldiers[k].cover).count();
    assert_eq!(guards(&g), 3, "a third of 8 on guard");
    for k in g.members(0) {
        let s = g.soldiers[k];
        assert_eq!(s.cover, matches!(s.order, Order::Fight(..) | Order::Move(..)), "a defender walks to its post and fights there, a worker drains");
    }
    let worker = g.members(0).into_iter().find(|&k| !g.soldiers[k].cover).unwrap();
    let boost = g.work_boost(worker);
    // 8 x (1 - 0.6 x 3/8) / 5: the squad's rate falls 22.5%, not 37.5%.
    assert!((boost - 1.24).abs() < 1e-4, "workers' boost {boost}");
    g.apply(Cmd::Cover(0, Cover::More));
    assert_eq!(guards(&g), 4);
    g.apply(Cmd::Cover(0, Cover::Never));
    assert_eq!(guards(&g), 0);
    assert_eq!(g.work_boost(worker), 1.0);
    // With nothing near (the default threshold, set out of reach), nobody guards.
    g.apply(Cmd::Cover(0, Cover::Some));
    assert_eq!(guards(&g), 3);
    g.rules.cover.near = u32::MAX;
    g.apply(Cmd::Cover(0, Cover::Some));
    assert_eq!(guards(&g), 0, "no slime near: everyone works");
    // Defenders stand at the work's edge, with the squad, never out in the
    // slime (posts out in its film cost soldiers, 2026-11-15).
    g.rules.cover.near = 0;
    g.apply(Cmd::Cover(0, Cover::Some));
    let (ci, cc) = g.crystal_at(c).unwrap();
    let edge = g.world.crystal_r(ci) + RIM * g.s();
    for k in g.members(0).into_iter().filter(|&k| g.soldiers[k].cover) {
        let (Order::Fight(x, y) | Order::Move(x, y)) = g.soldiers[k].order else { unreachable!() };
        let d = (x - cc.0).hypot(y - cc.1);
        assert!(d <= edge + 6.0 * g.s(), "a post {:.1} cells out, the edge at {:.1}", d / g.s(), edge / g.s());
    }
    // Sent elsewhere, the work ends and the defenders come in.
    let to = open_near(&g, 20.0);
    g.apply(Cmd::SquadOrder(0, Order::Move(to.0, to.1)));
    assert_eq!(g.squads[0].work, None);
    assert_eq!(guards(&g), 0);
}

#[test]
fn a_squad_told_to_heal_heals_for_food_and_nobody_heals_unasked() {
    // `heal`: told to heal or not, with the heal rate at `rate`.
    let run = |rate: f32, heal: bool| {
        let mut g = no_larder(10);
        g.rules.heal.rate = rate;
        g.rules.heal.away = 0.0;
        g.bank = 1000.0;
        let k = g.members(0)[0];
        g.soldiers[k].hp = 50.0;
        if heal {
            assert!(g.apply(Cmd::Heal(0)));
        }
        for _ in 0..10 * g.rules.tps / g.rules.frame_ticks {
            g.step();
        }
        (g.soldiers[k].hp, g.healed, g.guarded.len(), g.squads[0].heal)
    };
    let (hp, healed, held, still) = run(HealRules::default().rate, true);
    let (hp0, healed0, ..) = run(0.0, true);
    let (hp_unasked, healed_unasked, ..) = run(HealRules::default().rate, false);
    eprintln!("healing: hp {hp} (rate 0 {hp0}, not told {hp_unasked}), healed {healed}, crystals held {held}");
    assert!(held > 0, "the squad holds the crystal it starts round");
    assert_eq!((hp0, healed0), (50.0, 0.0), "no healing with the rate at 0");
    // Owner 2026-11-15 07:41Z: "Healing should be a specific action".
    assert_eq!((hp_unasked, healed_unasked), (50.0, 0.0), "nobody heals without the order");
    assert!(hp >= 99.0, "told to heal, 10 s by a held crystal: {hp} hp");
    assert!(healed > 45.0);
    assert!(!still, "whole again, the squad stops healing");
    // Away from a held crystal, a sixth as fast; another order stops it.
    let mut g = no_larder(10);
    g.guarded.clear();
    g.rules.guard = 0.0;
    g.bank = 1000.0;
    let k = g.members(0)[0];
    g.soldiers[k].hp = 50.0;
    g.apply(Cmd::Heal(0));
    for _ in 0..10 * g.rules.tps / g.rules.frame_ticks {
        g.step();
    }
    let hp = g.soldiers[k].hp;
    assert!(g.guarded.is_empty());
    assert!(hp > 55.0 && hp < 70.0, "healed 10 s away from a held crystal: {hp} hp");
    assert!(g.squads[0].heal);
    let m = g.middle(0).unwrap();
    g.apply(Cmd::SquadOrder(0, Order::Move(m.0 + 4.0 * g.s(), m.1)));
    assert!(!g.squads[0].heal, "another order ends the healing");
}

/// Deep water (new ground kinds, 2026-11-14): no soldier stands in it, so
/// one sent across the river walks round to the ford and never sets foot
/// in the deep; shots fly over it; a bridge carries soldiers too.
#[test]
fn soldiers_cross_deep_water_only_by_ford_or_bridge() {
    for (ford, bridge, lane) in [(true, false, (280.0, 360.0)), (false, true, (440.0, 472.0))] {
        let m = crate::mould::map::river(ford, bridge);
        let mut g = Game::new(
            BigDish::with_map(race_params(1, 2, 500), &m),
            Rules {
                build: BuildRules {
                    raise: 0.0,
                    ..BuildRules::default()
                },
                ..Rules::default()
            },
            1,
        );
        // Map cells to fine cells at scale 2.
        let f = |p: (f32, f32)| (p.0 / 2.0, p.1 / 2.0);
        let (deep, west, east) = (f((510.0, 100.0)), f((400.0, 520.0)), f((640.0, 520.0)));
        assert!(
            !g.world.open(deep.0, deep.1) && g.world.clear(deep.0, deep.1),
            "deep water: no ground, but clear to fire over"
        );
        assert!(
            g.clear_line(west, east) && !g.walk_line(west, east),
            "a shot crosses the river, a soldier can't walk it"
        );
        // A bullet across the river flies its whole leg.
        g.bullets.clear();
        let (legs, _) = g.fly(
            vec![ShotIn {
                from: west,
                to: east,
                r: 1.0,
                over: false,
            }],
            &[(Some(0), 0, None)],
            &[],
            10.0,
        );
        assert_eq!(legs[0].to, east, "the bullet flew over the water");
        g.soldiers[0].x = west.0;
        g.soldiers[0].y = west.1;
        g.soldiers[0].order = Order::Move(east.0, east.1);
        let dt = 1.0 / g.rules.tps as f32;
        let mut crossed_at = None;
        for _ in 0..(120 * g.rules.tps) {
            // The step builds walking maps a slice at a time (`pump_maps`).
            g.pump_maps();
            g.walk(0, dt);
            let s = g.soldiers[0];
            assert!(g.world.open(s.x, s.y), "soldier in deep water at ({}, {})", s.x, s.y);
            if crossed_at.is_none() && s.x >= 240.0 && s.x < 272.0 {
                crossed_at = Some(s.y * 2.0);
            }
            if (s.x - east.0).hypot(s.y - east.1) < 2.0 {
                break;
            }
        }
        let s = g.soldiers[0];
        assert!(
            (s.x - east.0).hypot(s.y - east.1) < 2.0,
            "ford {ford} bridge {bridge}: the soldier got across, at ({}, {})",
            s.x,
            s.y
        );
        let y = crossed_at.expect("it crossed the river");
        assert!(
            y >= lane.0 && y < lane.1,
            "crossed at map y {y}, not on the ford or bridge {lane:?}"
        );
        // No building in deep water.
        assert!(!g.world.open(deep.0, deep.1));
    }
}

/// Walking maps (bug check, 2026-11-15): a goal's own built map serves it
/// before a neighbour's, and maps gone stale are rebuilt even while new
/// goals are asked for every step, as in a long fight.
#[test]
fn stale_walking_maps_are_rebuilt_while_goals_keep_coming() {
    let m = crate::mould::map::river(true, false);
    let mut g = Game::new(BigDish::with_map(race_params(1, 2, 500), &m), Rules::default(), 1);
    let (w, h) = g.world.size();
    let ok = g.standable_cells();
    let open: Vec<usize> = (0..w * h).filter(|&i| ok[i] && i % w > 2 && i % w < w - 2).collect();
    let mid = |k: usize| ((k % w) as f32 + 0.5, (k / w) as f32 + 0.5);

    // Own map first: with the cell to its left built too, a goal on the
    // edge between them (as near both middles; ties go to the lower cell)
    // gets its own.
    let gi = open[open.len() / 2];
    assert!(ok[gi - 1]);
    g.paths.maps.insert(gi - 1, (vec![0; w * h], 0));
    g.paths.maps.insert(gi, (vec![0; w * h], 0));
    let edge = ((gi % w) as f32, (gi / w) as f32 + 0.5);
    assert_eq!(g.map_for(gi, edge, super::paths::MAP_NEAR * g.s()), Some(gi));
    g.paths.maps.clear();

    // One map built, then stale; a new goal is asked for every step.
    let a = open[open.len() / 3];
    let mut built = false;
    for _ in 0..20 {
        built = g.map_for(a, mid(a), 0.0).is_some();
        if built {
            break;
        }
        g.pump_maps();
    }
    assert!(built, "the first map was built");
    g.stale_maps();
    assert_eq!(g.paths.stale, vec![a]);
    let mut rebuilt = None;
    for step in 0..40 {
        let b = open[(open.len() / 2 + step * 997) % open.len()];
        g.map_for(b, mid(b), 0.0);
        g.pump_maps();
        if !g.paths.stale.contains(&a) && g.paths.building.as_ref().is_none_or(|b| b.goal != a) {
            rebuilt = Some(step);
            break;
        }
    }
    assert!(rebuilt.is_some(), "the stale map was never rebuilt while goals kept coming");
}

// Guns (owner 2026-11-14 16:46Z: "The squad owns the gun. Each gun defines
// the budget and slot count and range, spread, rate of fire as actual
// numbers").

/// A squad with no gun of its own fires the standard rifle, which is the
/// game's own rules, so a game that hands out no guns plays as before.
#[test]
fn an_unarmed_squad_fires_the_standard_rifle_of_its_rules() {
    let mut g = squads(1);
    g.apply(Cmd::Active(u8::MAX));
    g.rules.range = 30.0;
    g.rules.shot_every = 0.5;
    assert_eq!(g.gun(0), Gun::standard(&g.rules));
    assert_eq!(g.gun_range(0), 30.0);
    assert_eq!(g.shot_gap(0), g.secs(0.5).max(1));
}

/// Each squad fires its own gun: range, rate, spread and bullets a shot.
/// Both halves of a split carry a copy (owner 18:05Z); merging hands a gun
/// to a squad without one; `Cmd::Arm` None returns the rifle.
#[test]
fn each_squad_fires_its_own_gun_and_both_halves_of_a_split_copy_it() {
    use crate::mould::guns::GunKind;
    let mut g = squads(2);
    for _ in 0..3 {
        g.apply(Cmd::Buy);
    }
    g.apply(Cmd::Active(u8::MAX));
    let (carbine, long) = (GunKind::Carbine.average(), GunKind::Long.average());
    assert!(g.apply(Cmd::Arm(0, Some(carbine))));
    assert_eq!((g.gun(0), g.gun_range(0)), (carbine, 15.0));
    let m = g.middle(0).unwrap();
    let b = g.split(0, (m.0 + 30.0, m.1)).expect("a split");
    assert_eq!(g.gun(0), carbine, "the half left behind keeps the gun");
    assert_eq!(g.gun(b), carbine, "the new half carries a copy (owner 18:05Z)");
    g.apply(Cmd::Arm(b as u8, Some(long)));
    assert_eq!(g.gun(0), carbine, "a copy: arming one half leaves the other's");
    assert_eq!(g.gun_range(b), 40.0);
    assert!(g.shot_gap(0) * 4 <= g.shot_gap(b), "the carbine fires 4.4 times as often as the long gun: {} against {}", g.shot_gap(0), g.shot_gap(b));
    assert_eq!(g.gun_range_max(), 40.0, "the mould's panic reach reads the farthest gun");
    g.apply(Cmd::Arm(b as u8, None));
    assert_eq!(g.gun(b), Gun::standard(&g.rules));
    assert!(!g.apply(Cmd::Arm(SQUADS_MAX as u8, Some(long))), "no squad there");
    // Merging: a squad with no gun of its own takes the joining one's.
    g.apply(Cmd::Arm(0, None));
    g.apply(Cmd::Arm(b as u8, Some(long)));
    assert!(g.apply(Cmd::Merge(0, b as u8)));
    assert_eq!(g.gun(0), long, "the long gun is not lost to the merge");
}

/// The scattergun fires its `shots` bullets a shot, fanned by its spread:
/// a soldier's shot leaves as that many legs.
#[test]
fn a_scattergun_fires_a_fan_of_bullets_a_shot() {
    use crate::mould::guns::GunKind;
    let fired = |shots: Option<u32>| {
        let mut g = small(3);
        if let Some(n) = shots {
            let gun = Gun { shots: n, range: g.rules.range, ..GunKind::Scatter.average() };
            g.arm(0, Some(gun));
        }
        for k in 0..g.soldiers.len() {
            g.apply(Cmd::Order(k, Order::Attack));
        }
        let (mut legs, mut shooters) = (0, 0);
        let mut dirs: Vec<(f32, f32)> = Vec::new();
        for _ in 0..(20 * g.rules.tps / g.rules.frame_ticks) {
            g.step();
            // This frame's first legs, by soldier: one shot each a frame
            // (a soldier fires at most once a frame at these rates).
            let fresh: Vec<&Shot> = g.last_shots.iter().filter(|s| s.fired && s.by.is_some()).collect();
            legs += fresh.len();
            let mut by: Vec<usize> = fresh.iter().filter_map(|s| s.by).collect();
            by.sort();
            by.dedup();
            shooters += by.len();
            dirs.extend(fresh.iter().map(|s| {
                let (dx, dy) = (s.to.0 - s.from.0, s.to.1 - s.from.1);
                let d = dx.hypot(dy).max(1e-3);
                (dx / d, dy / d)
            }));
            if g.end.is_some() {
                break;
            }
        }
        (legs, shooters, dirs)
    };
    let (legs1, shooters1, _) = fired(None);
    assert!(shooters1 > 0 && legs1 <= shooters1 + shooters1 / 10, "the rifle: one bullet a shot ({legs1} legs, {shooters1} shots)");
    let (legs4, shooters4, dirs) = fired(Some(4));
    assert!(shooters4 > 0, "the scattergun fired");
    assert!(legs4 >= 3 * shooters4, "four bullets a shot (some dropped at the cap or a wall): {legs4} legs from {shooters4} shots");
    let mut d = dirs.clone();
    d.dedup_by(|a, b| (a.0 - b.0).abs() < 1e-4 && (a.1 - b.1).abs() < 1e-4);
    assert!(d.len() > dirs.len() / 2, "the fan's bullets fly apart");
}

/// Each start is an organic shape, not a disc (planet review, 2026-11-15:
/// "every start is a perfect gold disc"): the 90th-percentile reach of the
/// walkers in each of 24 sectors round the start varies by far more than
/// the disc's on the same open world,
/// every walker is placed, and seeds differ.
#[test]
fn a_start_is_an_organic_shape_not_a_disc() {
    let reach = |p: crate::mould::big::BigParams| {
        let d = BigDish::new(p);
        // The living (the rest are empty slots), about their own middle.
        let live: Vec<(f32, f32)> = (0..d.walkers.len()).filter(|&k| d.lives(k)).map(|k| (d.walkers[k].0, d.walkers[k].1)).collect();
        let n = live.len() as f32;
        let (hx, hy) = (live.iter().map(|w| w.0).sum::<f32>() / n, live.iter().map(|w| w.1).sum::<f32>() / n);
        let mut by: Vec<Vec<f32>> = vec![Vec::new(); 24];
        for &(x, y) in &live {
            let (dx, dy) = (x - hx, y - hy);
            let a = ((dy.atan2(dx) / std::f32::consts::TAU + 1.0).fract() * 24.0) as usize % 24;
            by[a].push(dx.hypot(dy));
        }
        let far: Vec<f32> = by
            .iter_mut()
            .map(|v| {
                v.sort_by(f32::total_cmp);
                v.get(v.len() * 9 / 10).copied().unwrap_or(0.0)
            })
            .collect();
        let mean = far.iter().sum::<f32>() / 24.0;
        let sd = (far.iter().map(|f| (f - mean).powi(2)).sum::<f32>() / 24.0).sqrt();
        (sd / mean, live.len(), far)
    };
    // On open ground, so rock does not clip either.
    let open = |seed, shape| crate::mould::big::BigParams { start_shape: shape, terrain_rock: 0.0, terrain_wet: 0.0, haze: 0.0, ..race_params(seed, 1, 500) };
    for seed in [1, 2, 3] {
        let (cv, n, far) = reach(open(seed, true));
        let (cv_disc, n_disc, _) = reach(open(seed, false));
        eprintln!("seed {seed}: organic {cv:.3}, disc {cv_disc:.3}");
        assert_eq!(n, n_disc, "seed {seed}: every walker placed");
        assert!(cv > cv_disc + 0.1, "seed {seed}: the start's reach varies by {cv:.3} of its mean, the disc's {cv_disc:.3}: {far:?}");
    }
    assert_ne!(reach(open(1, true)).2, reach(open(2, true)).2, "two seeds, one shape");
}

// Walking maps at the core world's 6x (technical review, 2026-11-15):
// a 1x world is 1024 x 640 map cells, a 6x one 2512 x 1568.

/// A world `w` x `h` map cells with its landing at (300, 320); with `bar`,
/// a rock bar at map x 400-447, y 120-519, east of it.
fn bar_map(w: usize, h: usize, bar: bool) -> crate::mould::map::Map {
    let mut ground = vec![0u8; w * h];
    if bar {
        for y in 120..520 {
            for x in 400..448 {
                ground[y * w + x] = crate::mould::map::G_ROCK;
            }
        }
    }
    let items = crate::mould::map::Items {
        crystals: vec![crate::mould::map::Crystal { x: 900.0, y: 80.0, r: 20.0, charge: 384.0 }, crate::mould::map::Crystal { x: 200.0, y: 560.0, r: 20.0, charge: 384.0 }],
        moulds: vec![(960.0, 600.0)],
        landing: Some((300.0, 320.0)),
        walls: Vec::new(),
        moisture: None,
        flora: None,
    };
    crate::mould::map::Map { w, h, ground, plants: Vec::new(), items }
}

/// Squad mode on `bar_map`, with walking maps built whole at once if
/// `whole` (`Rules::map_pops`), else a slice a step as in play.
fn bar_game(w: usize, h: usize, bar: bool, whole: bool) -> Game<BigDish> {
    let m = bar_map(w, h, bar);
    let mut rules = Rules { build: BuildRules { raise: 0.0, ..BuildRules::default() }, ..Rules::default() };
    rules.squads = true;
    if whole {
        rules.map_pops = usize::MAX;
    }
    Game::new(BigDish::with_map(race_params(1, 2, 300), &m), rules, 1)
}

/// Game seconds until a squad of 4 steered east from the landing has its
/// middle past the bar (map x 480), or None in `secs`.
fn steer_past_bar(w: usize, h: usize, whole: bool, secs: u32) -> Option<f32> {
    let mut g = bar_game(w, h, true, whole);
    for _ in 0..4 {
        g.apply(Cmd::Buy);
    }
    g.apply(Cmd::Active(0));
    let tps = g.rules.tps;
    for _ in 0..2 * tps {
        g.step();
    }
    g.apply(Cmd::Steer(0, (1.0, 0.0)));
    for i in 0..secs * tps {
        g.step();
        // A fine cell is two map cells here (scale 2).
        if g.middle(0).unwrap().0 * 2.0 > 480.0 {
            return Some(i as f32 / tps as f32);
        }
    }
    None
}

/// **A squad steered into rock gets round it as fast with maps built a
/// slice a step as with whole ones** (`STEER_NEAR`). Near half a lead, at
/// first, every anchor move asked for a new map and the anchor waited for
/// it: 26.3 s against 12.9 s whole at 1x, and never in 60 s at 6x. Now
/// 13.6 s at 1x (2026-11-15).
#[test]
fn a_steered_squad_gets_round_rock_with_sliced_maps() {
    let whole = steer_past_bar(1024, 640, true, 60).expect("whole maps get past the bar");
    let sliced = steer_past_bar(1024, 640, false, 60).expect("sliced maps get past the bar");
    eprintln!("past the bar at 1x: whole {whole:.1} s, sliced {sliced:.1} s");
    assert!(sliced < 1.5 * whole, "sliced {sliced:.1} s against whole {whole:.1} s");
}

/// The same at the core world's 6x, where a map takes about 8 steps:
/// 16.4 s sliced against 12.9 s whole (2026-11-15).
#[test]
fn a_steered_squad_gets_round_rock_with_sliced_maps_at_6x() {
    let whole = steer_past_bar(2512, 1568, true, 60).expect("whole maps get past the bar");
    let sliced = steer_past_bar(2512, 1568, false, 60).expect("sliced maps get past the bar");
    eprintln!("past the bar at 6x: whole {whole:.1} s, sliced {sliced:.1} s");
    assert!(sliced < 1.5 * whole, "sliced {sliced:.1} s against whole {whole:.1} s");
}

/// **24 live goals at 6x keep their maps** (`MAP_CELLS_MAX`): each goal,
/// asked every step, is served on every step after its first. At 16M
/// cells the cache held 17 maps at 6x and lost 26% of goal-steps to
/// rebuilds with 24 goals.
#[test]
fn walking_maps_serve_24_goals_at_6x() {
    let mut g = bar_game(2512, 1568, false, false);
    let (fw, fh) = g.world.size();
    let n = 24;
    let goals: Vec<(f32, f32)> = (0..n).map(|k| (30.0 + (k % 8) as f32 * (fw as f32 - 60.0) / 7.0, 30.0 + (k / 8) as f32 * 40.0)).collect();
    let near = super::paths::MAP_NEAR * g.s();
    let (mut first, mut lost) = (vec![None; n], 0usize);
    for step in 0..600 {
        g.pump_maps();
        for (k, &p) in goals.iter().enumerate() {
            match g.map_for(cell(fw, fh, p), p, near) {
                Some(_) => first[k] = first[k].or(Some(step)),
                None if first[k].is_some() => lost += 1,
                None => {}
            }
        }
    }
    assert!(first.iter().all(Option::is_some), "every goal served in 600 steps: {first:?}");
    assert_eq!(lost, 0, "goal-steps unserved after a goal's first map");
}

/// **Walls going up every 2 steps still let a waiting goal's map finish**
/// (`stale_maps`): the map being built finishes, then goes stale. Started
/// again on each wall, at first, a goal at 6x got no map in 600 steps with
/// a wall every 6 steps or less.
#[test]
fn walls_in_quick_succession_let_a_walking_map_finish() {
    for (w, h) in [(1024, 640), (2512, 1568)] {
        let mut g = bar_game(w, h, false, false);
        let (fw, fh) = g.world.size();
        let p = (fw as f32 - 40.0, fh as f32 - 40.0);
        let near = super::paths::MAP_NEAR * g.s();
        let got = (0..60u32).find(|&step| {
            g.pump_maps();
            let served = g.map_for(cell(fw, fh, p), p, near).is_some();
            if step % 2 == 1 {
                g.stale_maps();
            }
            served
        });
        assert!(got.is_some_and(|s| s < 30), "{w}x{h}: first map at step {got:?}");
    }
    // A map finished after a wall went up mid-build is stale: walked until
    // it is built again on the new ground.
    let mut g = bar_game(2512, 1568, false, false);
    let (fw, fh) = g.world.size();
    let p = (fw as f32 - 40.0, fh as f32 - 40.0);
    let gi = cell(fw, fh, p);
    assert!(g.map_for(gi, p, 0.0).is_none());
    g.pump_maps();
    assert!(g.paths.building.is_some(), "a 6x map takes more than a step");
    g.stale_maps();
    while g.map_for(gi, p, 0.0).is_none() {
        g.pump_maps();
    }
    let again = g.paths.stale.contains(&gi) || g.paths.building.as_ref().is_some_and(|b| b.goal == gi);
    assert!(again, "built on the ground before the wall, so stale (or being built again)");
}

#[test]
fn a_link_whose_controlled_squad_dies_hands_control_to_the_rest() {
    let mut g = squads(8);
    for _ in 0..5 {
        g.apply(Cmd::Buy);
    }
    g.apply(Cmd::Active(0));
    off_the_crystal(&mut g);
    let m = g.middle(0).unwrap();
    assert!(g.apply(Cmd::Split(0, (m.0 + 30.0, m.1))));
    let b = g.active.unwrap();
    assert!(g.apply(Cmd::Link(b as u8, 0)));
    // The controlled squad is wiped out: the rest of its link takes over
    // (until 11-15 Active refused them as linked, a soft-lock).
    for k in g.members(b) {
        g.soldiers[k].alive = false;
    }
    assert!(g.members(b).is_empty());
    assert!(g.apply(Cmd::Active(0)), "the live squad of the link takes control");
    assert_eq!(g.active, Some(0));
}

/// **Letting go of every squad** (owner 2026-11-15 02:58Z: "a way to
/// deselect so that you're not actually controlling any specific squad"):
/// `Cmd::Active(u8::MAX)` stops the one let go (and its link) steering and
/// volleying, keeps its sent order, takes the lead edge from it, and a
/// soldier bought meanwhile still joins a squad (there is no larder).
#[test]
fn letting_go_stops_the_steer_keeps_orders_and_a_bought_soldier_joins_a_squad() {
    let mut g = squads(4);
    g.bank = 1e6;
    for _ in 0..3 {
        g.apply(Cmd::Buy);
    }
    g.apply(Cmd::Active(0));
    off_the_crystal(&mut g);
    let m = g.middle(0).unwrap();
    let b = g.split(0, (m.0 + 30.0, m.1)).expect("a split");
    assert_eq!(g.active, Some(b));
    let to = open_near(&g, 20.0);
    g.apply(Cmd::SquadOrder(0, Order::Move(to.0, to.1)));
    let sent = g.squads[0].dest;
    assert!(sent.is_some());
    g.apply(Cmd::Steer(b as u8, (1.0, 0.0)));
    g.apply(Cmd::Fire(b as u8, Some(m)));
    assert!(g.leads(b));
    assert!(g.apply(Cmd::Active(u8::MAX)));
    assert_eq!(g.active, None);
    assert_eq!(g.squads[b].steer, (0.0, 0.0), "let go, it stops steering");
    assert!(g.squads[b].fire.is_none(), "and stops volleying");
    assert!(!g.leads(b) && !g.leads(0), "no squad has the lead's edge");
    assert_eq!(g.squads[0].dest, sent, "the other keeps the order it was sent on");
    let n = g.alive();
    assert!(g.apply(Cmd::Buy));
    assert_eq!(g.alive(), n + 1);
    let k = g.soldiers.len() - 1;
    assert_eq!(g.soldiers[k].squad as usize, g.live_squads()[0], "the new soldier joins the lowest live squad");
    // A replay line round-trips.
    assert_eq!(Cmd::parse(&Cmd::Active(u8::MAX).to_line()), Some(Cmd::Active(u8::MAX)));
}

/// The rifle's base spread (owner 11-15 08:10Z) lets a bullet's spread
/// multiplier show: the same frame's shots, drawing the same numbers, turn
/// four times as far with a x4 spread bullet loaded (before, the rifle's
/// spread was 0 and no bullet turned a shot).
#[test]
fn a_spread_bullet_widens_the_rifles_aim() {
    use super::super::bullets::Ammo;
    use super::super::rounds::{Card, Deck};
    let (mut a, mut b) = (small(3), small(3));
    assert!(a.gun(0).spread > 0.0, "the rifle has a base spread");
    let wide = Ammo { spread: 400, ..Ammo::default() };
    assert!(b.apply(Cmd::Load(0, Some(Deck::new(&[Card::Ammo(wide)])))));
    assert!(a.apply(Cmd::Load(0, Some(Deck::new(&[Card::Ammo(Ammo::default())])))));
    for g in [&mut a, &mut b] {
        for k in 0..g.soldiers.len() {
            g.apply(Cmd::Order(k, Order::Attack));
        }
    }
    let turn = |p: &Shot, aim: (f32, f32)| {
        let (u, v) = ((p.to.0 - p.from.0, p.to.1 - p.from.1), (aim.0 - p.from.0, aim.1 - p.from.1));
        (u.0 * v.1 - u.1 * v.0).atan2(u.0 * v.0 + u.1 * v.1).abs()
    };
    for _ in 0..15 * 60 {
        let aims: Vec<_> = a.soldiers.iter().map(|s| s.aim).collect();
        a.step();
        b.step();
        if !a.last_shots.is_empty() {
            assert_eq!(a.last_shots.len(), b.last_shots.len());
            let mut wider = 0;
            for (p, q) in a.last_shots.iter().zip(&b.last_shots) {
                let k = a.soldiers.iter().position(|s| (s.x, s.y) == p.from).expect("its soldier");
                let at = aims[k].expect("its target");
                let (ta, tb) = (turn(p, at), turn(q, at));
                assert!(tb + 1e-4 >= ta, "a x4 bullet turned less: {tb} against {ta}");
                wider += (tb > 2.0 * ta && tb > 1e-3) as u32;
            }
            assert!(wider > 0, "no shot turned wider with the spread bullet");
            return;
        }
    }
    panic!("no volley");
}

/// The plants' fire's kills count in the game's kills every frame (bug
/// check 11-15: they were counted only on a Scorch trail's frame, so a
/// game without one never counted them).
#[test]
fn the_plants_fire_kills_count_without_a_scorch_trail() {
    use crate::mould::map::Map;
    use crate::mould::plants::pack;
    use std::path::Path;
    let dir = Path::new(env!("CARGO_MANIFEST_DIR")).join("Reports/larder-maps-2026-11-11/maps");
    let mut m = Map::load(&dir, "flats").unwrap();
    // Full cover in kind bands, as the GPU twin test of the plants.
    m.plants = (0..m.ground.len()).map(|i| pack([0, 1, 3, 0][(i / m.w / 160).min(3)], 1.0)).collect();
    let mut g = Game::new(BigDish::with_map(race_params(3, 1, 2_000), &m), Rules::default(), 3);
    let home = g.world.homes[0];
    let burn = squad::Burst { at: (home.0 + 12.0, home.1), r: 6.0, burn: true, ..Default::default() };
    for sec in 0..40 {
        if sec == 10 {
            g.world.burst(&burn, &squad::BurstRules::default());
        }
        for _ in 0..g.rules.tps / g.rules.frame_ticks {
            g.step();
        }
    }
    eprintln!("t {} end {:?} burning {} fire_killed {} killed {}", g.time(), g.end, g.world.burning, g.world.fire_killed, g.killed);
    assert!(g.world.fire_killed > 0, "the fire killed walkers");
    assert_eq!(g.fire_killed as u64, g.world.fire_killed, "every fire kill counted");
    assert!(g.killed >= g.fire_killed);
}

/// The window's rule keys (`set=NAME=V`, `Rules::set`, 2026-11-15, W44):
/// a known rule takes the value, an unknown one is refused.
#[test]
fn rules_set_by_name() {
    let mut r = Rules::default();
    assert!(r.set("pump_smell", 2.0) && r.build.pump_smell == 2.0);
    assert!(r.set("panic_margin", 7.0) && r.panic_margin == 7.0);
    assert!(!r.set("no_such_rule", 1.0));
}

/// Food-driven surges (`Rules::surge_food`, `Game::time_surges`, 11-15):
/// the meter fills with what the slime eats from its crystals, a surge
/// starts once it is full and `surge_min` has passed, or once the world's
/// `surge_every` has whatever it ate, and with `surge_food` 0 the world
/// keeps its own clock.
#[test]
fn the_slime_surges_when_it_has_eaten() {
    let play = |food: f32, min: f32| {
        let mut d = race_world(1, 1, 1_000);
        d.p.surge_every = 20 * 15 * d.p.substeps as u32;
        let r = Rules { surge_food: food, surge_min: min, build: BuildRules { raise: 0.0, ..BuildRules::default() }, ..Rules::default() };
        let mut g = Game::new(d, r, 1);
        let held = g.world.p.surge_at;
        let (mut starts, mut last) = (Vec::new(), held);
        while g.time() < 70.0 && g.end.is_none() {
            g.step();
            if g.world.p.surge_at != last {
                last = g.world.p.surge_at;
                let fine = (g.rules.tps * g.world.fine_steps()) as f32;
                assert_eq!(last as u64, g.world.ticks() * g.world.fine_steps() as u64 + 1, "it starts on the next step");
                starts.push((last as f32 - 1.0) / fine);
            }
        }
        (g, held, starts)
    };
    let (g, held, starts) = play(0.0, 10.0);
    assert!(held == 0 && starts.is_empty() && g.surges == 0, "surge_food 0: the world's clock");
    assert!(g.slime_ate > 0.0, "the slime eats from its crystals: {}", g.slime_ate);
    let rate = g.slime_ate / g.time();
    // Fed: the meter fills in about a second, so every surge waits for the
    // floor (5 s).
    let (g, held, starts) = play(rate, 5.0);
    let gaps: Vec<f32> = std::iter::once(starts[0] - g.landed as f32 / g.rules.tps as f32).chain(starts.windows(2).map(|w| w[1] - w[0])).collect();
    eprintln!("eats {rate:.2} food/s; fed: {} surges, gaps {gaps:?}", g.surges);
    assert_eq!(held, SURGE_WAIT, "the game holds the body until its first surge");
    assert!(g.surges as usize == starts.len() && starts.len() >= 8, "{starts:?}");
    assert!(gaps.iter().all(|&s| (5.0..5.5).contains(&s)), "each at the floor: {gaps:?}");
    // Starved (a meter it can't fill): only the longest wait, 20 s.
    let (g, _, starts) = play(1.0e9, 5.0);
    let gaps: Vec<f32> = std::iter::once(starts[0] - g.landed as f32 / g.rules.tps as f32).chain(starts.windows(2).map(|w| w[1] - w[0])).collect();
    eprintln!("starved: gaps {gaps:?}");
    assert!(starts.len() >= 3 && gaps.iter().all(|&s| (20.0..20.5).contains(&s)), "each at the ceiling: {gaps:?}");
}

/// Raids (`Rules::raid`, `raids.rs`, 2026-11-15, the defenses thread): with
/// the meter full the game sends a raid at the pump drawing most, through
/// the world's raid words (target, reach, share, first step); a lure that
/// is on near the target takes it; `food` 0 sends none; the last stand
/// sends the whole slime once at the soldiers when it falls under its share
/// of its peak.
#[test]
fn the_slime_raids_the_richest_pump_and_a_lure_takes_it() {
    let play = |raid: RaidRules, lure: bool, secs: f32| {
        let mut g = small(1);
        g.rules.raid = raid;
        g.bank = 1.0e6;
        // Pumps on the two crystals farthest from the mould's home: the
        // larger draws more.
        let cs = g.world.crystals();
        let mut by_far: Vec<usize> = (0..cs.len()).collect();
        let home = g.core;
        by_far.sort_by(|&a, &b| ((cs[b].0 .0 - home.0).hypot(cs[b].0 .1 - home.1)).total_cmp(&(cs[a].0 .0 - home.0).hypot(cs[a].0 .1 - home.1)));
        for &c in &by_far[..2] {
            assert!(g.build(Kind::Pump, cs[c].0), "a pump on crystal {c}");
        }
        let top = (0..g.buildings.len()).max_by(|&a, &b| g.pump_draw(a).total_cmp(&g.pump_draw(b))).unwrap();
        if lure {
            let at = g.buildings[top].at;
            let spot = (0..64).map(|i| (at.0 + 8.0 * (i as f32 * 0.7).cos(), at.1 + 8.0 * (i as f32 * 0.7).sin())).find(|&p| g.build_blocked(Kind::Lure, p).is_none()).expect("room for a lure");
            assert!(g.build(Kind::Lure, spot));
        }
        while g.time() < secs && g.end.is_none() {
            g.step();
        }
        (g, top)
    };
    let fast = RaidRules { first: 2.0, first_sd: 0.0, min: 1.0, every: 5.0, every_sd: 0.0, size_sd: 0.0, ..RaidRules::default() };
    let (g, top) = play(fast, false, 14.0);
    eprintln!("raids: {:?}", g.raids.sent);
    assert!(g.raids.sent.len() >= 2, "a raid every 5 s: {}", g.raids.sent.len());
    assert!(g.raids.sent.iter().all(|d| d.size == fast.size), "no spread, the mean size");
    let d = g.raids.sent[0];
    assert_eq!(d.aim, RaidAim::Building(top), "the pump drawing most");
    assert!(d.reach > 0.0 && d.held > 0 && d.share == fast.share && !d.last_stand);
    let last = g.raids.sent.last().unwrap();
    assert_eq!(g.world.p.raid_to, raid_cell(last.to, (g.world.w, g.world.h)), "the world holds the target");
    assert_eq!((g.world.p.raid_r, g.world.p.raid_share), (last.reach, last.share));
    assert_eq!(g.world.p.raid_at as u64, last.tick * g.world.fine_steps() as u64 + 1, "it starts on the next step");
    // A lure that is on by the pump takes the raid.
    let (g, top) = play(fast, true, 8.0);
    let lure = g.buildings.iter().position(|b| b.kind == Kind::Lure).unwrap();
    assert!(g.buildings[lure].alive, "the lure still stands at the first raid");
    assert_eq!(g.raids.sent[0].aim, RaidAim::Lure { lure, from: Some(top) });
    assert_eq!(g.raids.sent[0].to, g.buildings[lure].at);
    // Off: none, and the world holds no raid.
    let (g, _) = play(RaidRules { every: 0.0, ..fast }, false, 14.0);
    assert!(g.raids.sent.is_empty() && g.world.p.raid_at == 0);
    // The last stand: once, the whole slime, at the soldiers.
    let mut g = small(1);
    g.rules.raid = RaidRules { last_stand: true, every: 0.0, ..RaidRules::default() };
    g.step();
    g.peak = g.mould * 10;
    g.time_raids();
    g.time_raids();
    assert_eq!(g.raids.sent.len(), 1, "once");
    let d = g.raids.sent[0];
    assert!(d.last_stand && d.share == 1.0 && matches!(d.aim, RaidAim::Soldiers(_)) && d.reach >= 1.0e3, "{d:?}");
    // Off by default.
    assert!(!RaidRules::default().last_stand);
    // Random (owner 2026-11-16 01:48Z): each raid draws its gap and its
    // size from a normal distribution, within the floor and the bounds.
    let spread = RaidRules { first: 2.0, first_sd: 1.0, min: 2.0, every: 4.0, every_sd: 2.0, size: 0.15, size_sd: 0.06, ..RaidRules::default() };
    let (g, _) = play(spread, false, 30.0);
    let r = &g.raids.sent;
    assert!(r.len() >= 4, "{} raids in 30 s", r.len());
    let gaps: Vec<u64> = r.windows(2).map(|w| w[1].tick - w[0].tick).collect();
    eprintln!("gaps {gaps:?}, sizes {:?}", r.iter().map(|d| d.size).collect::<Vec<_>>());
    assert!(gaps.iter().any(|&a| a != gaps[0]), "the gaps vary: {gaps:?}");
    assert!(gaps.iter().all(|&a| a >= 2 * g.rules.tps as u64), "never under the floor: {gaps:?}");
    assert!(r.iter().any(|d| d.size != r[0].size), "the sizes vary");
    assert!(r.iter().all(|d| (spread.size_min..=spread.size_max).contains(&d.size)));
}

/// The raids' normal draw has the mean and spread asked for.
#[test]
fn the_raids_draw_from_a_normal_distribution() {
    let mut r = Raids::new(7);
    let xs: Vec<f32> = (0..20_000).map(|_| r.normal(60.0, 20.0)).collect();
    let m = xs.iter().sum::<f32>() / xs.len() as f32;
    let sd = (xs.iter().map(|x| (x - m) * (x - m)).sum::<f32>() / xs.len() as f32).sqrt();
    assert!((m - 60.0).abs() < 0.6 && (sd - 20.0).abs() < 0.6, "mean {m}, sd {sd}");
    let within = xs.iter().filter(|&&x| (x - 60.0).abs() < 20.0).count() as f32 / xs.len() as f32;
    assert!((within - 0.683).abs() < 0.02, "{within} within one sd");
}

/// The kill box pays (`BuildRules::render_food`, 2026-11-15): only a turret
/// upgraded once renders; the slime killed within its range pays into the
/// bank and counts as income, and a kill outside it pays nothing.
#[test]
fn a_rendering_turret_pays_for_kills_in_its_range() {
    let mut g = small(1);
    g.bank = 1.0e5;
    let at = g.larder;
    let spot = (0..64).map(|i| (at.0 + 12.0 * (i as f32 * 0.7).cos(), at.1 + 12.0 * (i as f32 * 0.7).sin())).find(|&p| g.build_blocked(Kind::Turret, p).is_none()).expect("room for a turret");
    assert!(g.build(Kind::Turret, spot));
    let t = g.buildings.len() - 1;
    let near = (spot.0 + 3.0, spot.1);
    let far = (spot.0 + 10.0 * g.rules.build.turret_range, spot.1);
    assert_eq!(g.render_pay(&[(Some(near), 10)]), 0.0, "a plain turret renders nothing");
    assert_eq!(g.upgrade_price(t), Some(g.rules.build.render_cost));
    assert_eq!(g.turret_at(spot), Some(t));
    let bank = g.bank;
    assert!(g.apply(Cmd::Upgrade(t)));
    assert_eq!(g.bank, bank - g.rules.build.render_cost);
    assert_eq!(g.upgrade_price(t), None, "one level");
    let (before, income) = (g.bank, g.income());
    let paid = g.render_pay(&[(Some(near), 10), (Some(far), 50), (None, 7)]);
    assert_eq!(paid, 10.0 * g.rules.build.render_food, "only the kills in range");
    assert_eq!(g.bank, before + paid);
    assert_eq!(g.income(), income + paid);
}

/// **Mini-bosses** (2026-11-16, `bosses.rs`): the chance rises through a
/// run (owner 04:58Z, "Chance increases with progression into the run").
#[test]
fn mini_boss_chances_rise_with_the_planet() {
    let (a, b) = (BossRules::for_depth(1), BossRules::for_depth(8));
    assert!(a.every > b.every && a.cut < b.cut && a.first > b.first, "{a:?} {b:?}");
    assert_eq!((a.most, BossRules::for_depth(4).most), (1, 2));
    assert_eq!(Rules::default().boss.every, 0.0, "off outside a run");
    assert_eq!(Rules::default().boss.cut, 0.0);
    for d in 1..8 {
        let (p, q) = (BossRules::for_depth(d), BossRules::for_depth(d + 1));
        assert!(q.every <= p.every && q.cut >= p.cut && q.grade_lo >= p.grade_lo && q.grade_hi >= p.grade_hi, "planet {d}");
    }
    // Strength by grade (owner 05:23Z): much stronger than ordinary
    // slime's one blow, and a wide spread.
    let r = BossRules::default();
    assert_eq!((r.armour_at(0.0), r.armour_at(1.0)), (4, 15));
    assert_eq!((r.bite_at(0.0), r.bite_at(1.0)), (1, 4));
    assert!(r.reach_at(6.0, 1.0) > 1.5 * r.reach_at(6.0, 0.0) - 1.0 && r.haste_at(1.0) > r.haste_at(0.0));
}

/// The slime's parts (`bosses::parts`): a piece joined to no crystal with
/// food is cut off; a gap of one coarse square still joins (corners too).
#[test]
fn slime_parts_find_a_cut_off_piece() {
    let (gw, gh, bin) = (40, 20, 2);
    let mut g = vec![0u32; gw * gh];
    // A fed piece at squares 2-5, 2-5 by a crystal; a cut piece at 30-35,
    // 10-15, its two halves a coarse square apart (squares 30-31, 34-35:
    // coarse 15 and 17... joined through 16 only if it holds walkers) and
    // a corner-joined bit.
    for y in 2..6 {
        for x in 2..6 {
            g[y * gw + x] = 3;
        }
    }
    for y in 10..16 {
        for x in 30..36 {
            g[y * gw + x] = 5;
        }
    }
    g[16 * gw + 36] = 7; // corner to the cut piece's square (35, 15)
    let food = [((4.0 * bin as f32, 4.0 * bin as f32), 3.0)];
    let p = parts(&g, gw, gh, bin, &food);
    assert_eq!(p.len(), 2, "{p:?}");
    let fed = p.iter().find(|p| p.fed).expect("a fed part");
    let cut = p.iter().find(|p| !p.fed).expect("a cut part");
    assert_eq!(fed.walkers, 16 * 3);
    assert_eq!(cut.walkers, 36 * 5 + 7);
    // Its centre in fine cells, in the cut piece's squares (30-36 x bin).
    assert!(cut.centre.0 > 30.0 * bin as f32 && cut.centre.0 < 37.0 * bin as f32 && cut.centre.1 > 10.0 * bin as f32 && cut.centre.1 < 17.0 * bin as f32, "{cut:?}");
    // No food: both cut off.
    assert!(parts(&g, gw, gh, bin, &[]).iter().all(|p| !p.fed));
}

/// Squads shoot a mini-boss first (owner 04:58Z, question 6): one in reach
/// and in sight takes the shot over the slime the engine's aim found; one
/// out of reach leaves the aim alone; hold fire still holds. A soldier
/// with slime on it shoots that unless the mini-boss is within its defend
/// reach, and the reach is the next bullet's (review of the mini-boss PR).
#[test]
fn squads_shoot_a_mini_boss_first() {
    let mut g = small(1);
    let sc = g.s();
    let s0 = g.soldiers[0];
    let slime = (s0.x + 3.0 * sc, s0.y);
    let set = |g: &mut Game<BigDish>, touching: u32| {
        let s = &mut g.soldiers[0];
        s.aim = Some(slime);
        s.dense = 50;
        s.touching = touching;
        s.order = Order::Hold;
    };
    set(&mut g, 2);
    assert_eq!(g.target(0), Some(slime), "defend shoots the slime on it");
    let q = s0.squad as usize;
    let reach = g.gun_range(q) * sc;
    assert_eq!(g.shot_reach(0), g.gun_range(q), "a standard bullet flies the gun's reach");
    let r = (300.0f32 / std::f32::consts::PI).sqrt();
    let at = |d: f32| (s0.x, s0.y + d);
    // In open ground by the larder, in reach.
    assert!(g.clear_line((s0.x, s0.y), at(reach + r)), "open ground by the larder");
    // Its body's edge within the defend reach: the mini-boss first, slime
    // on the soldier or not.
    let close = at(r + 0.5 * g.rules.defend * sc);
    let boss = Boss { lump: 0, born: 0, at: close, cause: BossCause::Staged, phase: BossPhase::Hunt(Some(0)), first: 300, members: 300, centre: close, grade: 0.5, kind: BossKind::Hunter, stolen: 0.0, visited: 0 };
    g.bosses.live.push(boss);
    for touching in [0, 2] {
        set(&mut g, touching);
        assert_eq!(g.target(0), Some(close), "the mini-boss first, {touching} on it");
    }
    // Further, in reach: the boss's line when nothing is on the soldier,
    // the slime on it when something is.
    g.bosses.live[0].centre = at(0.9 * reach);
    set(&mut g, 0);
    assert_eq!(g.boss_aim(0), Some((at(0.9 * reach), r)), "in reach, nothing on it");
    set(&mut g, 2);
    assert_eq!(g.boss_aim(0), None, "slime on it, the boss further than its defend reach");
    assert_eq!(g.target(0), Some(slime), "so it shoots the slime on it");
    // Far out of reach: the engine's aim again.
    g.bosses.live[0].centre = at(3.0 * reach);
    set(&mut g, 2);
    assert_eq!(g.target(0), Some(slime), "out of reach: the slime on it");
    // Its body's edge in reach: shot at along the line, at the gun's reach.
    g.bosses.live[0].centre = at(reach + 0.5 * r);
    set(&mut g, 0);
    let (p, _) = g.boss_aim(0).expect("an edge in reach is shot at");
    assert!((p.0 - s0.x).abs() < 1e-3 && (p.1 - (s0.y + reach)).abs() < 1e-2, "{p:?}");
    // A short bullet (half range) next: that edge is out of its reach.
    g.bullet_mods.casts = 1;
    g.bullet_mods.ammo[0].range = 50;
    assert!(g.squads.get(q).is_none_or(|s| s.deck.is_none()), "the army's cards");
    assert!((g.shot_reach(0) - 0.5 * g.gun_range(q)).abs() < 1e-4);
    assert_eq!(g.boss_aim(0), None, "out of the bullet's reach");
    g.bosses.live[0].centre = at(0.5 * reach);
    let (p, _) = g.boss_aim(0).expect("in the bullet's reach");
    assert!((p.1 - (s0.y + 0.5 * reach)).abs() < 1e-2, "{p:?}");
    g.bullet_mods.casts = 0;
    g.bosses.live[0].centre = close;
    g.soldiers[0].stance = Stance::HoldFire;
    set(&mut g, 2);
    assert_eq!(g.target(0), None, "not on hold fire");
}

/// **How a mini-boss ends** (review of the mini-boss PR): one with too few
/// at its knot fizzles, one under its quorum breaks, one that hunted its
/// `life` is spent; each goes unsteered and unarmoured at once (its lump's
/// rules back to none). A fizzled one lets its members go at once; a
/// broken or spent one fades first (`BossRules::fade`, the look thread's
/// ask): its members stay marked, its lump is not reused and the view's
/// word says how far through, then it lets them go. Then no walker
/// carries its lump or its hits, and the world reads no centre for it.
#[test]
fn a_mini_boss_ends_and_lets_its_walkers_go() {
    let tps = small(0).rules.tps as usize;
    let cases = [
        (BossEnd::Fizzled, BossRules { least: 1_000_000, ..BossRules::default() }),
        (BossEnd::Broken, BossRules { least: 1, quorum: 2.0, ..BossRules::default() }),
        (BossEnd::Spent, BossRules { least: 1, gather: 1.0, life: 1.0, ..BossRules::default() }),
    ];
    for (end, rules) in cases {
        let mut g = small(2);
        g.rules.boss = rules;
        // Round the thickest of the slime.
        let w = &g.world;
        let living: Vec<usize> = (0..w.walkers.len()).filter(|&k| w.lives(k)).collect();
        let (x, y) = living.iter().fold((0.0, 0.0), |a, &k| (a.0 + w.walkers[k].0, a.1 + w.walkers[k].1));
        let at = (x / living.len() as f32, y / living.len() as f32);
        assert!(g.form_boss(at, 6.0, BossCause::Staged));
        let l = g.bosses.live[0].lump;
        let joined = (0..g.world.walkers.len()).filter(|&k| g.world.lives(k) && g.world.lump_of(k) == Some(l)).count();
        assert!(joined > 10, "{end:?}: {joined} joined");
        // A member hurt, to see its hits go.
        let hurt = (0..g.world.walkers.len()).find(|&k| g.world.lives(k) && g.world.lump_of(k) == Some(l)).unwrap();
        assert!(!g.world.hit(hurt) && g.world.dense[hurt] >> crate::mould::big::HURT_SHIFT == 1);
        for _ in 0..6 * tps {
            g.step();
            if !g.bosses.ended.is_empty() {
                break;
            }
        }
        let (b, how, _) = g.bosses.ended.first().copied().unwrap_or_else(|| panic!("{end:?}: it ended: {:?}", g.bosses.live));
        assert_eq!(how, end, "{b:?}");
        assert!(g.bosses.live.is_empty());
        assert_eq!(g.world.p.lumps[l], crate::mould::big::Lump::default(), "{end:?}: unsteered and unarmoured at once");
        let marked = |g: &Game<BigDish>| (0..g.world.walkers.len()).filter(|&k| g.world.dense.get(k).is_some_and(|&d| d & crate::mould::big::LUMP_BITS != 0)).count();
        if end == BossEnd::Fizzled {
            assert!(g.bosses.fading.is_empty() && g.lump_looks()[l] == 0, "{end:?}: no fade");
        } else {
            assert_eq!(g.bosses.fading.len(), 1, "{end:?}: it fades");
            assert!(marked(&g) > 0, "{end:?}: its members stay marked while it fades");
            let look = g.lump_looks()[l];
            assert!(look & 1 << 8 != 0 && look & 7 == g.bosses.fading[0].0.kind as u16, "{end:?}: the view's word says it fades, and its kind: {look:#x}");
            // Its lump is not taken while it fades.
            assert!(g.form_boss(at, 6.0, BossCause::Staged) && g.bosses.live[0].lump != l);
            let other = g.bosses.live.remove(0).lump;
            World::form_lump(&mut g.world, other, None);
            World::set_lump(&mut g.world, other, crate::mould::big::Lump::default());
            assert!(marked(&g) > 0, "{end:?}: still marked");
            let mut last = look >> 9;
            while !g.bosses.fading.is_empty() {
                g.step();
                if !g.bosses.fading.is_empty() {
                    let now = g.lump_looks()[l] >> 9;
                    assert!(now >= last, "{end:?}: the fade only goes on");
                    last = now;
                }
            }
            assert_eq!(last, 127, "{end:?}: through the whole fade");
        }
        // The lumps' sums are the last step's start: one more.
        g.step();
        let w = &g.world;
        assert!((0..w.walkers.len()).all(|k| w.dense.get(k).is_none_or(|&d| d & (crate::mould::big::LUMP_BITS | crate::mould::big::HURT_BITS) == 0)), "{end:?}: no slot keeps the lump or its hits");
        assert_eq!(w.p.lumps[l], crate::mould::big::Lump::default(), "{end:?}: its rules back to none");
        assert!(World::lumps(w)[l].is_none(), "{end:?}: no centre read");
    }
}

/// **The hunter lunges** (`BossRules::lunge_every`, the look thread's ask:
/// the pulse must come from the simulation): while it hunts a soldier,
/// its lump's pull and pace go up `lunge_pull` and `lunge_haste` times for
/// `lunge` seconds every `lunge_every`, and back between; off with
/// `lunge_every` 0.
#[test]
fn a_hunting_mini_boss_lunges() {
    for every in [3.0, 0.0] {
        let mut g = small(2);
        // A hunter (`kind` 1): the others go on errands and do not lunge.
        g.rules.boss = BossRules { least: 1, gather: 1.0, life: 30.0, quorum: 0.0, lunge_every: every, lunge: 1.0, lunge_pull: 2.0, lunge_haste: 3.0, kind: 1.0, ..BossRules::default() };
        let tps = g.rules.tps as usize;
        // Round the thickest of the slime.
        let w = &g.world;
        let living: Vec<usize> = (0..w.walkers.len()).filter(|&k| w.lives(k)).collect();
        let (x, y) = living.iter().fold((0.0, 0.0), |a, &k| (a.0 + w.walkers[k].0, a.1 + w.walkers[k].1));
        assert!(g.form_boss((x / living.len() as f32, y / living.len() as f32), 6.0, BossCause::Staged));
        let (l, grade) = (g.bosses.live[0].lump, g.bosses.live[0].grade);
        let mut seen = Vec::new();
        for _ in 0..12 * tps {
            g.step();
            if g.world.ticks().is_multiple_of(tps as u64) && g.bosses.live.first().is_some_and(|b| matches!(b.phase, BossPhase::Hunt(Some(_)))) {
                let p = g.world.p.lumps[l];
                seen.push((p.pull, p.haste));
            }
        }
        let r = g.rules.boss;
        assert!(!g.bosses.live.is_empty(), "it still hunts: {:?}", g.bosses.ended);
        let base = (r.pull, r.haste_at(grade));
        let up = seen.iter().filter(|s| s.0 == base.0 * 2.0 && (s.1 - base.1 * 3.0).abs() < 1e-5).count();
        let calm = seen.iter().filter(|s| s.0 == base.0 && (s.1 - base.1).abs() < 1e-5).count();
        eprintln!("lunge every {every}: {} hunting seconds, {up} lunging, {calm} calm", seen.len());
        assert_eq!(up + calm, seen.len(), "only the two paces: {seen:?}");
        if every > 0.0 {
            assert!(up >= 3 && calm >= 2 * up - 2, "a lunge every 3 s, a second long: {up} of {}", seen.len());
        } else {
            assert_eq!(up, 0);
        }
    }
}

/// **A big part cut off becomes one** (`BossRules::cut`), through the
/// game's second: not in the landing's grace (`first`), then at once with
/// chance 1, and not again where one just ended for `rest` seconds (review
/// of the mini-boss PR: its own remnant re-formed the next second with its
/// hits healed). Every crystal emptied, so all the slime is cut off.
#[test]
fn a_cut_off_part_becomes_a_mini_boss_after_the_grace() {
    let mut g = small(2);
    g.rules.boss = BossRules { cut: 1.0, cut_least: 50, first: 3.0, least: 1, quorum: 0.0, gather: 1.0, life: 1.0, rest: 4.0, ..BossRules::default() };
    let tps = g.rules.tps as u64;
    g.world.charge.iter_mut().for_each(|c| *c = 0.0);
    g.world.pile_charge.iter_mut().for_each(|c| *c = 0.0);
    assert!(g.slime_parts().iter().any(|p| !p.fed && p.walkers >= 50), "{:?}", g.slime_parts());
    let mut born = Vec::new();
    for _ in 0..12 * tps {
        g.step();
        for b in &g.bosses.live {
            if !born.iter().any(|&(t, _): &(u64, (f32, f32))| t == b.born) {
                born.push((b.born, b.at));
            }
        }
    }
    eprintln!("born {born:?}; ended {:?}", g.bosses.ended.iter().map(|e| (e.0.born, e.1, e.2)).collect::<Vec<_>>());
    assert!(g.bosses.ended.iter().all(|e| e.0.cause == BossCause::Cut) && !born.is_empty());
    let (t0, _) = born[0];
    assert!(t0 >= 3 * tps && t0 <= 4 * tps + 1, "not in the grace, then at once: {t0}");
    // None where one ended within its rest.
    for &(t, at) in &born {
        for e in g.bosses.ended.iter().filter(|e| e.2 <= t && t < e.2 + 4 * tps) {
            let d = (at.0 - e.0.centre.0).hypot(at.1 - e.0.centre.1);
            assert!(d > 6.0 * g.s(), "born at {t} {d:.1} cells from one ended at {}", e.2);
        }
    }
    assert!(born.len() >= 2, "one again after the rest: {born:?}");
}

/// The same seed gives the same mini-bosses: formed, hunted and ended
/// alike on two CPU games (their own draws, `Bosses::rng`).
#[test]
fn mini_bosses_replay_from_the_seed() {
    let run = || {
        let mut g = small(4);
        g.rules.boss = BossRules { every: 4.0, every_sd: 1.0, first: 2.0, first_sd: 0.5, min: 1.0, crowd: 0.0, least: 10, gather: 1.0, life: 3.0, cut: 0.5, cut_least: 100, ..BossRules::default() };
        for k in 0..g.soldiers.len() {
            g.apply(Cmd::Order(k, Order::Attack));
        }
        for _ in 0..15 * g.rules.tps {
            g.step();
        }
        (format!("{:?} {:?}", g.bosses.live, g.bosses.ended), World::hash(&g.world))
    };
    let (a, b) = (run(), run());
    assert!(a.0.contains("Boss {"), "some formed: {}", a.0);
    assert_eq!(a, b);
}

/// **Planet 8's mini-bosses, the smoke** (CLAUDE.md's checks): idle hands
/// on the run's last planet's rules see none in the first `min` seconds
/// (and no cut-off one before `first`), and the game does not end in under
/// a minute. The race world's 1,000 walkers never fill a knot to 0.8 or
/// cut off 400, so any knot and a cut-off part of 100 do here.
#[test]
fn planet_eight_bosses_leave_the_first_minute_playable() {
    let mut g = small(5);
    g.rules.boss = BossRules { crowd: 0.0, cut_least: 100, ..BossRules::for_depth(8) };
    let r = g.rules.boss;
    let tps = g.rules.tps as u64;
    let mut first = None;
    for _ in 0..60 * tps {
        g.step();
        if let Some(b) = g.bosses.live.first().or(g.bosses.ended.first().map(|e| &e.0)) {
            first.get_or_insert((b.born, b.cause));
        }
        assert!(g.end.is_none(), "the game ended at {} s", g.world.ticks() / tps);
    }
    eprintln!("planet 8, idle: first mini-boss {first:?}, {} in all", g.bosses.ended.len() + g.bosses.live.len());
    let (t, cause) = first.expect("one formed in the minute");
    assert!(t >= (r.min * tps as f32) as u64, "none in the first {} s: {t}", r.min);
    assert!(cause != BossCause::Cut || t >= (r.first * tps as f32) as u64, "no cut-off one in the grace: {t}");
}

/// A mini-boss forms by its rules at a crowded knot of the CPU dish,
/// clenches, then hunts the nearest soldier; staged ones count their
/// members a second on, and one that loses its body breaks.
#[test]
fn a_mini_boss_forms_at_a_crowd_and_hunts() {
    let mut g = small(2);
    g.rules.boss = BossRules { every: 5.0, every_sd: 0.0, first: 2.0, first_sd: 0.0, min: 1.0, crowd: 0.0, least: 10, gather: 2.0, ..BossRules::default() };
    let tps = g.rules.tps as usize;
    let mut formed = None;
    for _ in 0..10 * tps {
        g.step();
        if let Some(b) = g.bosses.live.first() {
            formed.get_or_insert(*b);
        }
        if g.bosses.live.first().is_some_and(|b| matches!(b.phase, BossPhase::Hunt(_))) {
            break;
        }
    }
    let b0 = formed.expect("a mini-boss formed");
    assert_eq!(b0.cause, BossCause::Crowd);
    let b = g.bosses.live.first().copied().or(g.bosses.ended.last().map(|e| e.0)).unwrap();
    assert!(b.first >= 10, "it held walkers: {b:?}, ended {:?}", g.bosses.ended);
    // Its grade tightens its clench and leash (`BossRules::tight`).
    let r = g.rules.boss;
    assert!(r.leash_at(r.leash, 1.0) < r.leash_at(r.leash, 0.0) && r.leash_at(r.leash, 0.0) == r.leash);
    assert!(matches!(b.phase, BossPhase::Hunt(_)), "it hunts after gathering: {b:?}");
    // Its members are marked in the dish and the world reads its centre.
    let l = b.lump;
    let marked = (0..g.world.walkers.len()).filter(|&k| g.world.lives(k) && g.world.lump_of(k) == Some(l)).count();
    assert!(marked as u32 >= b.members / 2 && marked > 0, "{marked} marked, {} members", b.members);
    assert!(g.world.lumps()[l].is_some());
    // The view's word for it: the hunter, hunting.
    assert_eq!(g.lump_looks()[l] & 15, 1 | 8);
}

/// For 200 soldiers in 8 squads of 25, each soldier's place in its squad's
/// formation (`place_at`) at one of 8 places across a 6x map, each asking
/// for its place's walking map every step with `near` dish cells
/// (`map_for`) for 600 steps: the places ever served, and the goal-steps
/// lost after a place's first map.
fn squad_goals_served(near: f32) -> (usize, usize) {
    let (w, h) = (2512, 1568);
    let m = bar_map(w, h, false);
    let rules = Rules { squads: true, larder: false, seek: false, overrun: 0.0, max_squad: 200, start_bank: 1e9, ..Rules::default() };
    let mut g = Game::new(BigDish::with_map(race_params(1, 2, 300), &m), rules, 1);
    while g.buy() {}
    let live: Vec<usize> = (0..g.soldiers.len()).filter(|&k| g.soldiers[k].alive).collect();
    assert_eq!(live.len(), 200, "200 soldiers bought");
    for (i, &k) in live.iter().enumerate() {
        g.apply(Cmd::Join(k, (i * 8 / live.len()) as u8));
    }
    let (fw, fh) = g.world.size();
    let dests: Vec<(f32, f32)> = (0..8).map(|q| (fw as f32 * (0.15 + 0.7 * (q % 4) as f32 / 3.0), fh as f32 * if q < 4 { 0.25 } else { 0.75 })).collect();
    // Each soldier's place in its squad's formation at its squad's place.
    let goals: Vec<(f32, f32)> = live.iter().map(|&k| g.place_at(k, dests[g.soldiers[k].squad as usize])).collect();
    assert_eq!(goals.len(), 200, "every soldier has a place to go");
    let near = near * g.s();
    let (mut first, mut lost) = (vec![false; goals.len()], 0usize);
    for _ in 0..600 {
        g.pump_maps();
        for (k, &p) in goals.iter().enumerate() {
            match g.map_for(cell(fw, fh, p), p, near) {
                Some(_) => first[k] = true,
                None if first[k] => lost += 1,
                None => {}
            }
        }
    }
    (first.iter().filter(|&&f| f).count(), lost)
}

/// **A squad's 25 formation places share their walking maps at 6x**
/// (`MAP_NEAR`, 2026-11-16, soldier cap 200). A formation of 25 spans 25
/// fine cells from its middle. At 4 dish cells (8 fine) only 89 of the 200
/// places got a map in 600 steps and 9,002 goal-steps were lost after a
/// first map, the cache (34 maps at 6x) thrashing; at 8, 13,405 lost; at
/// 12 and 16, none.
#[test]
fn squads_of_25_share_walking_maps_at_6x() {
    let (served, lost) = squad_goals_served(super::paths::MAP_NEAR);
    assert_eq!(served, 200, "every place served in 600 steps");
    assert_eq!(lost, 0, "goal-steps unserved after a place's first map");
}


/// A mini-boss staged round the slime's middle (the race world's slime
/// sits on crystal 0 at 56,80; the squad on crystal 21 at 236,99), with
/// these rules and forced to `kind`.
fn staged_kind(seed: u64, kind: BossKind, rules: BossRules) -> Game<BigDish> {
    let mut g = small(seed);
    g.rules.boss = BossRules { least: 1, gather: 1.0, life: 120.0, quorum: 0.0, kind: kind as u8 as f32, ..rules };
    let w = &g.world;
    let living: Vec<usize> = (0..w.walkers.len()).filter(|&k| w.lives(k)).collect();
    let (x, y) = living.iter().fold((0.0, 0.0), |a, &k| (a.0 + w.walkers[k].0, a.1 + w.walkers[k].1));
    assert!(g.form_boss((x / living.len() as f32, y / living.len() as f32), 6.0, BossCause::Staged));
    assert_eq!(g.bosses.live[0].kind, kind);
    g
}

/// **A wanderer goes to a far fresh crystal, roots, and moves on**
/// (`BossKind::Wanderer`): it sets off for the nearest crystal at least
/// `far` away, sits on it for `root` seconds, takes in the slime round it
/// (more members) and marks it visited, then goes for another.
#[test]
fn a_wanderer_roots_on_a_far_crystal_and_moves_on() {
    let mut g = staged_kind(2, BossKind::Wanderer, BossRules { far: 40.0, root: 10.0, ..BossRules::default() });
    let tps = g.rules.tps as usize;
    let start = g.bosses.live[0].centre;
    let mut goals: Vec<(f32, f32)> = Vec::new();
    let (mut sat, mut grew) = (None, 0);
    // Walkers round the far crystal (crystal 8, 88,51; 43 cells off), and
    // its charge, as it sits down and as it leaves.
    let round = |g: &Game<BigDish>| (0..g.world.walkers.len()).filter(|&k| g.world.lives(k) && (g.world.walkers[k].0 - 88.3).hypot(g.world.walkers[k].1 - 50.8) < 12.0).count();
    let (mut at_sit, mut at_leave) = ((0, 0.0), (0, 0.0));
    for s in 0..90 {
        for _ in 0..tps {
            g.step();
        }
        let Some(b) = g.bosses.live.first() else { break };
        match b.phase {
            BossPhase::Go(p) | BossPhase::Sit(p, _) if goals.last() != Some(&p) => goals.push(p),
            _ => {}
        }
        if matches!(b.phase, BossPhase::Sit(..)) && sat.is_none() {
            sat = Some((s, b.members));
            at_sit = (round(&g), g.world.crystals()[8].1);
        }
        if goals.len() >= 2 && b.visited != 0 {
            break;
        }
    }
    // The members as the next second starts (the sums are the step's).
    for _ in 0..tps {
        g.step();
    }
    if let (Some((_, m)), Some(b)) = (sat, g.bosses.live.first()) {
        grew = b.members as i64 - m as i64;
        at_leave = (round(&g), g.world.crystals()[8].1);
    }
    let b = g.bosses.live.first().copied().or(g.bosses.ended.last().map(|e| e.0)).unwrap();
    eprintln!("goals {goals:?}; sat {sat:?}; grew {grew}; round crystal 8 (walkers, charge) sitting {at_sit:?}, leaving {at_leave:?}, now {:?}; visited {:b}; ended {:?}", (round(&g), g.world.crystals()[8].1), b.visited, g.bosses.ended.iter().map(|e| e.1).collect::<Vec<_>>());
    let cs = g.world.crystals();
    let first = goals.first().copied().expect("it set off");
    let j = cs.iter().position(|c| c.0 == first).expect("for a crystal");
    assert!((first.0 - start.0).hypot(first.1 - start.1) >= 40.0, "a far one: {first:?} from {start:?}");
    assert!(sat.is_some(), "it got there and sat");
    assert!(b.visited & 1 << j != 0, "marked visited");
    assert!(goals.len() >= 2 && goals[1] != first, "then on to another: {goals:?}");
    assert!(grew > 0, "it took in the slime there");
}

/// **A sapper goes for a wall and eats through it** (`BossKind::Sapper`):
/// it sets off for the nearest point of a standing wall, presses into it
/// until the wall's cell there is gone, then goes on through and hunts;
/// with no building at all it hunts.
#[test]
fn a_sapper_goes_for_a_wall() {
    for wall in [true, false] {
        let mut g = staged_kind(2, BossKind::Sapper, BossRules::default());
        g.bank = 1e6;
        let (at, to) = ((95.0, 65.0), (95.0, 95.0));
        if wall {
            assert!(g.build(Kind::Wall { to }, at), "{:?}", g.build_blocked(Kind::Wall { to }, at));
        }
        let tps = g.rules.tps as usize;
        let mut phases = Vec::new();
        for _ in 0..100 {
            for _ in 0..tps {
                g.step();
            }
            let Some(b) = g.bosses.live.first() else { break };
            phases.push(b.phase);
            if b.visited == 2 {
                break;
            }

        }
        eprintln!("wall {wall}: {} s, last {:?}, {} wall bites, {} wall cells broken", phases.len(), phases.last(), g.world.wall_hits, g.world.broken);
        if wall {
            let to_wall = phases.iter().find_map(|p| if let BossPhase::Go(p) = p { Some(*p) } else { None }).expect("it set off");
            assert!((to_wall.0 - 95.0).abs() < 1e-3 && (65.0..=95.0).contains(&to_wall.1), "for the wall's nearest point: {to_wall:?}");
            assert!(phases.iter().any(|p| matches!(p, BossPhase::Sit(..))), "it reached the wall");
            assert!(g.world.wall_hits > 0, "and bit into it");
            // Holed where it sat, it goes through to the far side (the wall
            // is at x 95, it came from the west), then hunts.
            let sat = phases.iter().position(|p| matches!(p, BossPhase::Sit(..))).unwrap();
            let through = phases.iter().rposition(|p| matches!(p, BossPhase::Go(q) if q.0 > 95.0)).expect("through the hole");
            let b = g.bosses.live[0];
            eprintln!("sat at {sat} s, going through from {through} s, hunting at {} s from {:.1},{:.1}", phases.len(), b.centre.0, b.centre.1);
            assert!(through > sat && b.visited == 2 && b.centre.0 > 95.0, "through: {:?} at {:?}", b.phase, b.centre);
        } else {
            assert!(phases.iter().all(|p| matches!(p, BossPhase::Hunt(_))), "nothing to sap: it hunts");
        }
    }
}

/// **A leech sits on a pump and takes its pay** (`BossKind::Leech`): the
/// pump keeps working and stands, but the bank gets nothing from it while
/// the leech holds on; broken, the leech gives its take back.
#[test]
fn a_leech_takes_a_pumps_pay_and_gives_it_back_when_broken() {
    let mut g = staged_kind(2, BossKind::Leech, BossRules::default());
    // A pump on crystal 14 (88,80), 31 cells from the slime's middle.
    g.bank = 1e6;
    let at = g.world.crystals()[14].0;
    assert!(g.build(Kind::Pump, at), "{:?}", g.build_blocked(Kind::Pump, at));
    let pump = g.buildings.len() - 1;
    // The race world's slime reaches the rig before the leech does and
    // would eat it: made tough here, so the leech is what's measured.
    (g.buildings[pump].hp, g.buildings[pump].max_hp) = (1e9, 1e9);
    let tps = g.rules.tps as usize;
    let mut sat = None;
    for s in 0..60 {
        for _ in 0..tps {
            g.step();
        }
        let b = g.bosses.live[0];
        if matches!(b.phase, BossPhase::Sit(..)) {
            sat = Some(s);
            break;
        }
    }
    let b = g.bosses.live[0];
    eprintln!("sat {sat:?} at {:?}; pump at {at:?} hp {} of {}", b.phase, g.buildings[pump].hp, g.buildings[pump].max_hp);
    assert!(sat.is_some(), "it reached the pump");
    assert_eq!(g.leech_on(pump), Some(0));
    let (bank, pumped, hp) = (g.bank, g.pumped, g.buildings[pump].hp);
    for _ in 0..10 * tps {
        g.step();
    }
    let took = g.bosses.live[0].stolen;
    eprintln!("10 s on: stole {took}, bank {} -> {}, pumped {pumped} -> {}, hp {hp} -> {}", bank, g.bank, g.pumped, g.buildings[pump].hp);
    assert!(took > 0.5 * 10.0 * g.pump_draw(pump), "it takes the pump's pay: {took}");
    assert_eq!(g.pumped, pumped, "none of it banked");
    assert!(g.bank <= bank, "the bank gets nothing from the pump");
    assert!(g.buildings[pump].alive && g.buildings[pump].hp >= hp, "the rig stands under it");
    assert_eq!(g.bosses.stolen, took);
    // It rings the pump (`BossRules::ring`, look thread 11:56Z), outside
    // the rig (`pump_rig_r`, 1.3 x the crystal's radius) and round it.
    let (lump, cr) = (g.bosses.live[0].lump, g.world.crystal_r(14));
    let mut eighths = [0; 8];
    let mut d: Vec<f32> = Vec::new();
    for k in (0..g.world.walkers.len()).filter(|&k| g.world.lives(k) && g.world.lump_of(k) == Some(lump)) {
        let (dx, dy) = (g.world.walkers[k].0 - at.0, g.world.walkers[k].1 - at.1);
        d.push(dx.hypot(dy));
        eighths[((dy.atan2(dx) / std::f32::consts::TAU + 1.0) * 8.0) as usize % 8] += 1;
    }
    d.sort_by(f32::total_cmp);
    let (tenth, median) = (d[d.len() / 10], d[d.len() / 2]);
    eprintln!("ring {:.1} (crystal {cr:.1}): members from the pump {tenth:.1} (10%), {median:.1} (median); by eighths {eighths:?}", cr * g.rules.boss.ring);
    assert!(tenth > 1.3 * cr && eighths.iter().filter(|&&n| n > 0).count() >= 6, "a ring outside the rig, round it");
    // Broken (most of its members gone): its take goes back.
    let bank = g.bank;
    g.bosses.live[0].first = 100 * g.bosses.live[0].members.max(1);
    g.rules.boss.quorum = 0.5;
    for _ in 0..2 * tps {
        g.step();
    }
    let e = g.bosses.ended.last().expect("it broke");
    assert_eq!(e.1, BossEnd::Broken);
    assert!(g.bank >= bank + 0.99 * e.0.stolen && e.0.stolen >= took, "given back: bank {bank} -> {}, stolen {}", g.bank, e.0.stolen);
    assert_eq!(g.bosses.returned, e.0.stolen);
}

/// **Charges round walls** (`Rules::unit_paths`, the simulation-use plan's
/// S1, 2026-11-17): once a game second the game builds the units' walking
/// distance from where the last frame's units stood and sets it a second
/// later, so after two seconds the world holds one, zero at a soldier's
/// block and rising away from it; off (the default), none; a masked
/// soldier is no source.
#[test]
fn the_game_sets_the_units_walking_distance_once_a_second() {
    use crate::mould::big::RAID_BLOCK;
    let play = |on: bool, mask: bool| {
        let mut g = small(2);
        g.rules.unit_paths = on;
        if mask {
            for s in &mut g.soldiers {
                s.smell = 0.0;
            }
        }
        let secs = (2.5 * g.rules.tps as f32 / g.rules.frame_ticks as f32).ceil() as usize;
        for _ in 0..secs {
            g.step();
        }
        g
    };
    let g = play(true, false);
    let f = &g.world.units_field;
    let bw = g.world.w.div_ceil(RAID_BLOCK);
    let s = g.soldiers[0];
    let at = |x: f32, y: f32| f[(y as usize / RAID_BLOCK) * bw + x as usize / RAID_BLOCK];
    let reach = f.iter().filter(|&&v| v < u32::MAX).count();
    eprintln!("units field: {reach} of {} blocks reached; {} at the first soldier, {} 20 cells east", f.len(), at(s.x, s.y), at(s.x + 20.0, s.y));
    assert!(!f.is_empty() && reach * 2 > f.len(), "the field is set and reaches most of the world");
    assert!(at(s.x, s.y) <= 3 && at(s.x + 20.0, s.y) > at(s.x, s.y), "zero by a soldier, rising away");
    assert!(play(false, false).world.units_field.is_empty(), "off: none");
    // Masked (the toolkit's mask, smell 0), no soldier is a source.
    let m = play(true, true);
    let fm = &m.world.units_field;
    eprintln!("every soldier masked: {} at the first soldier", fm[(s.y as usize / RAID_BLOCK) * bw + s.x as usize / RAID_BLOCK]);
    assert!(fm.len() == f.len() && fm[(s.y as usize / RAID_BLOCK) * bw + s.x as usize / RAID_BLOCK] > 3, "a masked soldier is no source");
}

/// **The walking distances don't depend on the thread's speed** (the
/// technical lead, 2026-11-17 13:50Z): the units' walking distance
/// (`unit_paths`) is set a game second after it is started and the far
/// smell's (`far_smell`, S2) two seconds after, each joining the worker if
/// it is late, so a game whose workers each sleep 300 ms (more than a
/// frame's worth of game time here) plays the same, its fields and its
/// hash, every game second.
#[test]
fn walking_distances_are_set_at_a_fixed_step_however_slow_the_thread() {
    use std::sync::atomic::Ordering;
    let play = |delay: u64| {
        super::raids::WAY_DELAY_MS.store(delay, Ordering::Relaxed);
        let mut g = small(2);
        g.rules.unit_paths = true;
        g.world.p.far_smell = 1.0;
        let per_sec = (g.rules.tps as f32 / g.rules.frame_ticks as f32).ceil() as usize;
        let mut seen = Vec::new();
        for f in 0..6 * per_sec {
            g.step();
            if f % per_sec == per_sec - 1 {
                seen.push((g.world.hash(), g.world.units_field.clone(), g.world.food_field.clone()));
            }
        }
        super::raids::WAY_DELAY_MS.store(0, Ordering::Relaxed);
        seen
    };
    let (fast, slow) = (play(0), play(300));
    assert!(fast.iter().any(|(_, f, g)| !f.is_empty() && !g.is_empty()), "the fields were set");
    for (s, (a, b)) in fast.iter().zip(&slow).enumerate() {
        assert_eq!(a.1, b.1, "second {}: the same field", s + 1);
        assert_eq!(a.2, b.2, "second {}: the same far smell", s + 1);
        assert_eq!(a.0, b.0, "second {}: the same dish", s + 1);
    }
}

/// **The far smell's field** (`BigParams::far_smell`, the simulation-use
/// plan's S2, 2026-11-17): with the slime reading it, the game builds the
/// walking distance to the untouched crystals every
/// `raids::FOOD_PATHS_EVERY` game seconds and sets it the next time round:
/// zero by an untouched crystal, rising away; off, none.
#[test]
fn the_game_sets_the_far_smell_every_two_seconds() {
    use crate::mould::big::RAID_BLOCK;
    let play = |on: bool| {
        let mut g = small(2);
        g.world.p.far_smell = if on { 1.0 } else { 0.0 };
        let secs = (4.5 * g.rules.tps as f32 / g.rules.frame_ticks as f32).ceil() as usize;
        for _ in 0..secs {
            g.step();
        }
        g
    };
    let g = play(true);
    let f = &g.world.food_field;
    let fresh = g.world.fresh_crystals();
    let bw = g.world.w.div_ceil(RAID_BLOCK);
    let at = |c: u32| f[((c >> 16) as usize / RAID_BLOCK) * bw + (c & 0xFFFF) as usize / RAID_BLOCK];
    let reach = f.iter().filter(|&&v| v < u32::MAX).count();
    eprintln!("far smell: {reach} of {} blocks reached, {} untouched crystals, {:?} at the first", f.len(), fresh.len(), fresh.first().map(|&c| at(c)));
    assert!(!fresh.is_empty() && reach * 2 > f.len(), "the field is set and reaches most of the world");
    assert!(fresh.iter().all(|&c| at(c) <= 8), "near zero at each untouched crystal");
    assert!(play(false).world.food_field.is_empty(), "off: none");
}
