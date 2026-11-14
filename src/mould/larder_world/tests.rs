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
    let many: Vec<ShotIn> = (0..squad::SHOTS_MAX + 40).map(|_| ShotIn { from: (from.x, from.y), to: (from.x + 10.0 * sc, from.y), r: 1.0 }).collect();
    let by = vec![Some(0); many.len()];
    let (legs, who) = g.fly(many, &by, 1.0 / 15.0);
    assert_eq!((legs.len(), who.len(), g.bullets.len()), (squad::SHOTS_MAX, squad::SHOTS_MAX, squad::SHOTS_MAX));
    assert_eq!(g.bullets_dropped, 40);
    // A leg ends at the first closed cell on its way, and the bullet
    // with it.
    g.bullets.clear();
    let (w, h) = g.world.size();
    let wall = (0..w * h).map(|i| ((i % w) as f32 + 0.5, (i / w) as f32 + 0.5)).find(|&(x, y)| !g.world.open(x, y) && g.world.open(x - 3.0, y) && g.world.open(x - 2.0, y) && g.world.open(x - 1.0, y) && x > 4.0).expect("a wall or rock cell with open ground west of it");
    let start = (wall.0 - 3.0, wall.1);
    let (legs, _) = g.fly(vec![ShotIn { from: start, to: (start.0 + 20.0, start.1), r: 1.0 }], &[None], 1.0);
    assert!((legs[0].to.0 - wall.0).abs() <= 1.0, "leg {:?} wall {:?}", legs[0], wall);
    g.landed(&legs, &[squad::ShotOut { hit: None, killed: 0 }]);
    assert!(g.bullets.is_empty(), "the bullet flew on past the wall");
}

#[test]
fn pierce_flies_on_past_a_hit_and_split_leaves_fanned_pieces() {
    let mut g = small(1);
    let sc = g.s();
    let r = g.rules;
    let leg = |from: (f32, f32)| ShotIn { from, to: (from.0 + 4.0, from.1), r: 2.0 };
    let hit = |at: (f32, f32)| squad::ShotOut { hit: Some(at), killed: 1 };
    let b = Bullet { at: (50.0, 50.0), dir: (1.0, 0.0), left: 30.0, r: 2.0, by: Some(0), fresh: true, pierce: 1, split: 1 };
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

#[test]
fn split_pieces_cap_at_the_shot_buffer_in_order() {
    // The flood: a full buffer of splitting bullets all hit at once.
    // Pieces go to the end in their parents' order until the buffer
    // is full, the rest are counted, never more than `SHOTS_MAX` fly.
    let mut g = small(1);
    let b = |i: usize| Bullet { at: (10.0 + i as f32, 40.0), dir: (1.0, 0.0), left: 30.0, r: 1.0, by: Some(i % 4), fresh: false, pierce: 0, split: 2 };
    g.bullets = (0..squad::SHOTS_MAX).map(b).collect();
    let legs: Vec<ShotIn> = g.bullets.iter().map(|q| ShotIn { from: q.at, to: (q.at.0 + 1.0, q.at.1), r: 1.0 }).collect();
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
    let both = BulletMods { pierce: 1, split: 2 };
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
        let next = downhill(w, h, &map, at);
        assert!(open(next) && next != at, "every step is open and goes downhill");
        at = next;
        steps += 1;
    }
    assert_eq!(at, goal);
    // Shut the gap: the goal is out of reach and the map says so.
    let sealed = distance_map(w, h, goal, |i| i % w != 10);
    assert_eq!(sealed[start], u32::MAX);
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
    // Rations come off the bank every step, so the f32 rounding is
    // a few hundredths over 30 s (0.03; 0.01 held before them).
    let books = g.rules.start_bank + g.harvested - g.spent - g.stolen - g.eaten;
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
    fn stop_births(&mut self) {
        World::stop_births(&mut self.d)
    }
    fn upkeep(&self) -> f32 {
        World::upkeep(&self.d)
    }
    fn set_upkeep(&mut self, up: f32) {
        World::set_upkeep(&mut self.d, up)
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
        (g.rules.mass, g.rules.ration, g.rules.overrun) = (mass, 0.0, 0.0);
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
    // Rations off: they would eat the bank this test buys with.
    g.rules.ration = 0.0;
    g.bank = 0.0;
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

#[test]
fn rations_feed_the_squad_and_an_empty_bank_starves_it_until_food_comes() {
    let mut g = small(1);
    let n = g.alive() as f32;
    // Off by default since 2026-11-14 (the owner's playtest); the rule
    // still works when `ration=` turns it on.
    assert!(n >= 1.0 && g.rules.ration == 0.0 && g.rules.hunger == 1.0);
    g.rules.ration = 2.0;
    let hp = g.soldiers[0].hp;
    // Paid in full: the bank pays, nobody hungers.
    g.bank = 3.0 * n;
    g.feed(1.0);
    assert!((g.bank - n).abs() < 1e-4 && (g.eaten - 2.0 * n).abs() < 1e-4 && !g.hungry);
    assert_eq!(g.soldiers[0].hp, hp);
    // Half paid: the bank empties and each soldier loses half of a
    // second's hunger.
    g.feed(1.0);
    assert!(g.bank.abs() < 1e-4 && g.hungry);
    assert!((g.soldiers[0].hp - (hp - 0.5)).abs() < 1e-4, "{}", g.soldiers[0].hp);
    // A carrier banks: the squad eats again (no spiral: hunger costs
    // only hit points, so harvesters keep their speed).
    g.bank = 300.0;
    g.feed(1.0);
    assert!(!g.hungry && (g.soldiers[0].hp - (hp - 0.5)).abs() < 1e-4);
    // Starved long enough, a soldier dies, and the need falls with the
    // squad.
    g.bank = 0.0;
    g.feed(hp);
    assert_eq!(g.alive(), 0);
    assert!(!g.soldiers[0].alive && g.soldiers[0].carry == 0.0);
    g.feed(1.0);
    assert!(!g.hungry, "nobody left to feed");
    // Off at 0.
    let mut g = small(1);
    g.rules.ration = 0.0;
    g.bank = 0.0;
    g.feed(10.0);
    assert!(!g.hungry && g.eaten == 0.0);
}

/// A starving squad can feed itself (the design review, 11-12 21:51Z:
/// a squad that slowly dies on an empty bank and can do nothing about
/// it is the playtest's soft lock again). Hunger costs only hit points,
/// so starving harvesters walk and take step for step as fed ones do,
/// and a carrier's trip home pays the rations again. A reserve recruit
/// on an empty bank comes in whole and can do the same.
#[test]
fn a_starving_squad_harvests_as_a_fed_one_and_feeds_itself() {
    let sent = |hunger: f32| {
        let mut g = small(7);
        // Rations on (off by default since 2026-11-14): this tests them.
        (g.rules.ration, g.rules.hunger, g.rules.overrun, g.bank) = (1.0, hunger, 0.0, 0.0);
        let target = g.seek(0, None).expect("a crystal with charge");
        for k in 0..g.soldiers.len() {
            g.apply(Cmd::Order(k, Order::Clear(target.0, target.1)));
        }
        g
    };
    // Same rations, so the same bank; only the hunger differs.
    let (mut fed, mut starving) = (sent(0.0), sent(1.0));
    let at = |g: &Game<BigDish>| g.soldiers.iter().map(|s| (s.x, s.y, s.carry)).collect::<Vec<_>>();
    let (mut hungry, mut ate_again) = (false, false);
    for _ in 0..(starving.rules.tps * 60) {
        fed.step();
        starving.step();
        if fed.alive() != starving.alive() {
            break;
        }
        assert_eq!(at(&fed), at(&starving), "at {:.1} s", starving.time());
        assert_eq!(fed.harvested, starving.harvested);
        hungry |= starving.hungry;
        ate_again |= hungry && !starving.hungry;
    }
    assert!(hungry && ate_again && starving.harvested > 0.0, "it starved, banked a load and ate again");
    assert!(starving.soldiers.iter().zip(&fed.soldiers).any(|(s, f)| s.alive && s.hp < f.hp), "and the hunger showed on its health");
    // The reserve: a wiped squad on an empty bank gets a recruit at full
    // health, who harvests and pays its own ration.
    let mut g = small(7);
    (g.rules.ration, g.rules.reserve, g.rules.overrun, g.bank) = (1.0, 10.0, 0.0, 0.0);
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
    let mut fed_itself = false;
    while g.time() < t0 + 60.0 && g.soldiers[k].alive {
        g.step();
        fed_itself |= g.bank > 0.0 && !g.hungry;
    }
    assert!(fed_itself && g.soldiers[k].alive, "the recruit banked food and ate");
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
