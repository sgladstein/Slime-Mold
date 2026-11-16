//! Soldiers on the right side of the wall they build (owner's playtest,
//! 2026-11-16 01:45Z: "some soldiers always end up on the wrong side,
//! trapped with the slime").

use super::*;
use crate::mould::big::BigDish;

/// A squad-mode game, 8 soldiers off the start crystal, slime called near
/// (`CoverRules::near` 0, so defenders come out as by the mould).
fn eight(seed: u64) -> Game<BigDish> {
    let mut g = Game::new(race_world(seed, 1, 1_000), Rules { start_bank: 1e6, squads: true, ..Rules::default() }, seed);
    while g.members(0).len() < 8 {
        assert!(g.apply(Cmd::Buy));
    }
    g.rules.cover.near = 0;
    g
}

/// Signed distance of `p` from the line `a`-`b`, positive on the side of `home`.
fn side(a: (f32, f32), b: (f32, f32), home: (f32, f32), p: (f32, f32)) -> f32 {
    let (dx, dy) = (b.0 - a.0, b.1 - a.1);
    let l = dx.hypot(dy);
    let n = (-dy / l, dx / l);
    let f = |q: (f32, f32)| (q.0 - a.0) * n.0 + (q.1 - a.1) * n.1;
    f(p) * f(home).signum()
}

/// One wall: the squad walks to open ground, a wall goes up `ahead` dish
/// cells from its middle toward the mould's core, square to that way,
/// and the squad is ordered to build it. (soldiers on the core's side
/// once it stands and 5 s after, soldiers shut in it) or None when the
/// wall can't be placed on this seed.
fn one(seed: u64, ahead: f32) -> Option<(usize, usize)> {
    let mut g = eight(seed);
    let sc = g.s();
    // Off the crystal, on open ground toward the core.
    let m = g.middle(0)?;
    let (cx, cy) = (g.core.0 - m.0, g.core.1 - m.1);
    let l = cx.hypot(cy);
    let u = (cx / l, cy / l);
    let to = (m.0 + u.0 * 20.0 * sc, m.1 + u.1 * 20.0 * sc);
    let to = g.nearest_standable(to, 10.0 * sc)?;
    g.apply(Cmd::SquadOrder(0, Order::Move(to.0, to.1)));
    for _ in 0..(10 * g.rules.tps / g.rules.frame_ticks) {
        g.step();
    }
    let m = g.middle(0)?;
    let c = (m.0 + u.0 * ahead * sc, m.1 + u.1 * ahead * sc);
    let p = (-u.1, u.0);
    let a = (c.0 - p.0 * 10.0 * sc, c.1 - p.1 * 10.0 * sc);
    let b = (c.0 + p.0 * 10.0 * sc, c.1 + p.1 * 10.0 * sc);
    let home = (c.0 - u.0 * 10.0 * sc, c.1 - u.1 * 10.0 * sc);
    if !g.apply(Cmd::Wall(a, b)) {
        return None;
    }
    let w = g.buildings.len() - 1;
    g.apply(Cmd::SquadOrder(0, Order::Build(w)));
    let mut t = 0;
    while !g.buildings[w].up() && t < 60 * g.rules.tps / g.rules.frame_ticks {
        g.step();
        t += 1;
    }
    if !g.buildings[w].up() {
        return None;
    }
    let shut = g.soldiers.iter().filter(|s| s.alive && !g.world.open(s.x, s.y)).count();
    for _ in 0..(5 * g.rules.tps / g.rules.frame_ticks) {
        g.step();
    }
    let far = g.soldiers.iter().filter(|s| s.alive && side(a, b, home, (s.x, s.y)) < 0.0).count();
    Some((far, shut))
}

/// The measure: soldiers past the wall, seeds 1-8, the wall 4 cells in
/// front of the squad's middle and through it.
#[test]
#[ignore]
fn measure_wall_sides() {
    for ahead in [4.0, 0.0] {
        let mut sum = (0, 0, 0);
        for seed in 1..9 {
            let r = one(seed, ahead);
            println!("ahead {ahead} seed {seed}: (past, shut) {r:?}");
            if let Some((f, s)) = r {
                sum = (sum.0 + f, sum.1 + s, sum.2 + 1);
            }
        }
        println!("ahead {ahead}: past {} shut {} over {} walls", sum.0, sum.1, sum.2);
    }
}

/// The fix: nobody past the wall or shut in it, the wall in front of the
/// squad or through it. On main (a119ce9) 9 of 64 were past the wall in
/// front (a defender each seed) and 35 of 64 past the one through it.
#[test]
fn a_squad_ends_on_its_own_side_of_the_wall_it_builds() {
    let mut walls = 0;
    for ahead in [4.0, 0.0] {
        for seed in 1..5 {
            let Some((past, shut)) = one(seed, ahead) else { continue };
            walls += 1;
            assert_eq!((past, shut), (0, 0), "ahead {ahead} seed {seed}: (past, shut)");
        }
    }
    assert!(walls >= 6, "{walls} of 8 walls stood");
}

/// A wall goes up against the slime: its home side is the one with less.
#[test]
fn a_walls_home_side_is_away_from_the_slime() {
    let g = eight(3);
    let sc = g.s();
    let m = g.middle(0).unwrap();
    // A line 12 cells short of the core, square to the way from the
    // squad: the core's slime on its far side.
    let (cx, cy) = (g.core.0 - m.0, g.core.1 - m.1);
    let l = cx.hypot(cy);
    let (u, p) = ((cx / l, cy / l), (-cy / l, cx / l));
    let c = (g.core.0 - u.0 * 12.0 * sc, g.core.1 - u.1 * 12.0 * sc);
    let (a, b) = ((c.0 - p.0 * 8.0 * sc, c.1 - p.1 * 8.0 * sc), (c.0 + p.0 * 8.0 * sc, c.1 + p.1 * 8.0 * sc));
    let n = g.wall_home(a, b);
    let toward = n.0 * u.0 + n.1 * u.1;
    let (plus, minus) = (g.world.walkers_in((c.0 + u.0 * 8.0 * sc, c.1 + u.1 * 8.0 * sc), 8.0 * sc), g.world.walkers_in((c.0 - u.0 * 8.0 * sc, c.1 - u.1 * 8.0 * sc), 8.0 * sc));
    // Short of the core, toward the squad: the core's side has the slime.
    assert!(plus > minus + WALL_SLIME, "slime by the core {plus}, on the squad's side {minus}");
    assert!(toward < -0.99, "home faces the squad, away from the core's slime: {toward}");
}
