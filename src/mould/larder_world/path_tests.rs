//! **Where squads got stuck finding their way** (pathfinding thread,
//! 2026-11-20). `examples/pathprobe` plays 60 soldiers in four squads over
//! run planets, steered with eight-way keys, sent, split and joined, and
//! `Reports/pathfinding-2026-11-20/tools/paths.py` counts soldiers and
//! squads that stand short of where they head. These keep its fixes.

use super::move_tests::move_game;
use super::*;

/// Steers squad 0 with eight-way keys at `to` (dish cells) for up to
/// `secs` game seconds; the game second its middle came within 12 dish
/// cells of it, if it did.
fn steer_to(g: &mut Game<crate::mould::big::BigDish>, to: (f32, f32), secs: f32) -> Option<f32> {
    let sc = g.s();
    let to = (to.0 * sc, to.1 * sc);
    let t0 = g.time();
    let mut held: Option<f32> = None;
    while g.time() - t0 < secs {
        let m = g.middle(0)?;
        let (dx, dy) = (to.0 - m.0, to.1 - m.1);
        if dx.hypot(dy) < 12.0 * sc {
            return Some(g.time() - t0);
        }
        // A key held till the way is 30 degrees off it, as a player holds one.
        let a = dy.atan2(dx);
        if held.is_none_or(|h| (a - h).sin().atan2((a - h).cos()).abs() > 30f32.to_radians()) {
            held = Some((a / std::f32::consts::FRAC_PI_4).round() * std::f32::consts::FRAC_PI_4);
        }
        let h = held.unwrap_or(0.0);
        g.apply(Cmd::Steer(0, (h.cos(), h.sin())));
        g.step();
    }
    None
}

/// **Steered past a rock's tip, a squad goes round it** (run seed
/// 1795038793, planet 2: 15 steered south-east from the landing toward the
/// crystal at dish 86, 104). The point 40 cells ahead fell in rock, its
/// walking map reached no ground, and the squad stood at the tip of a bar
/// for good (175 s in the probe). Now the point backs off along its line to
/// ground (`Game::steer_point`): there in about 12 s.
#[test]
fn a_squad_steered_past_a_rocks_tip_goes_round_it() {
    let mut g = move_game(&crate::mould::levelgen::level(1_795_038_793, 2), 1_795_038_793, 15);
    let t = steer_to(&mut g, (86.0, 104.0), 40.0);
    assert!(t.is_some(), "never got there: middle at {:?}", g.middle(0).map(|m| (m.0 / g.s(), m.1 / g.s())));
}

/// **Steered along a slanting wall, a squad gets past it** (run seed
/// 1795038793, planet 3: 15 steered at the crystal at dish 169, 135). With
/// only the next step's line checked, its anchor stepped straight where
/// that was open and back up its map where it was not, and stood at the
/// wall shuffling for good. Now on a map it keeps to it until the whole
/// line is open (`Game::guide`): there in about 25 s.
#[test]
fn a_squad_steered_along_a_slanting_wall_gets_past_it() {
    let mut g = move_game(&crate::mould::levelgen::level(1_795_038_793, 3), 1_795_038_793, 15);
    let t = steer_to(&mut g, (169.0, 135.0), 60.0);
    assert!(t.is_some(), "never got there: middle at {:?}", g.middle(0).map(|m| (m.0 / g.s(), m.1 / g.s())));
}

/// `onto_map`: a cell the map has no way from (a crystal eaten since it
/// was built, or a cell's middle in a crystal's disc) takes the nearest
/// cell that has one, nearest the goal among those; none past `ONTO_MAP`.
#[test]
fn a_cell_off_a_walking_map_finds_the_nearest_on_it() {
    let (w, h) = (20, 20);
    // A disc of "crystal" round (10, 10), radius 3, shut; the goal at (0, 0).
    let open = |i: usize| ((i % w) as f32 - 10.0).hypot((i / w) as f32 - 10.0) > 3.0;
    let map = distance_map(w, h, 0, open);
    let mid = 10 * w + 10;
    assert_eq!(map[mid], u32::MAX);
    let on = onto_map(w, h, &map, mid).expect("a cell on the map");
    let (x, y) = ((on % w) as f32, (on / w) as f32);
    assert!((x - 10.0).abs().max((y - 10.0).abs()) <= 3.0, "the nearest ring with ground: ({x}, {y})");
    assert!(x < 10.0 && y < 10.0, "on the goal's side: ({x}, {y})");
    // Every cell shut but the goal: nothing within reach.
    let lone = distance_map(w, h, 0, |i| i == 0);
    assert_eq!(onto_map(w, h, &lone, mid), None);
    // A cell on the map is its own.
    assert_eq!(onto_map(w, h, &map, 5), Some(5));
}

/// Squad 0 sent 50 cells east on open ground past fire marked halfway
/// (radius 6 cells, marked again each second, as soldiers feeling its heat
/// would), with `Rules::fireway` as given: the nearest any member came to
/// the fire's middle (dish cells), and the game second it arrived, if it
/// did within 30 s.
fn past_fire(fireway: bool) -> (f32, Option<f32>) {
    let mut g = move_game(&super::narrow_tests::door_map(0.0, 0.0, 0.0), 3, 15);
    g.rules.fireway = fireway;
    let sc = g.s();
    let at = g.middle(0).unwrap();
    let to = (at.0 + 50.0 * sc, at.1);
    let fire = (at.0 + 25.0 * sc, at.1);
    g.apply(Cmd::SquadOrder(0, Order::Move(to.0, to.1)));
    let t0 = g.time();
    let mut near = f32::MAX;
    let mut marked = -1.0;
    while g.time() - t0 < 30.0 {
        if g.time() - marked >= 1.0 {
            g.mark_hot(fire, 6.0 * sc);
            marked = g.time();
        }
        g.step();
        for k in g.members(0) {
            near = near.min((g.soldiers[k].x - fire.0).hypot(g.soldiers[k].y - fire.1) / sc);
        }
        if g.squads[0].dest.is_none() {
            return (near, Some(g.time() - t0));
        }
    }
    (near, None)
}

/// **A squad sent past fire walks round it** (`fireway.rs`; Direction,
/// 2026-11-21: a move order's way avoids burning ground where there is a
/// way round). Without it the squad walks through the fire's middle; with
/// it the nearest member passes 4.5 cells from it (its flank may brush the
/// edge of the 6-cell fire: wider marks made longer ways round for no less
/// fire on the 24 maps).
#[test]
fn a_squad_sent_past_fire_walks_round_it() {
    let (through, _) = past_fire(false);
    assert!(through < 3.0, "without fireway it walks through: nearest {through:.1} cells");
    let (round, t) = past_fire(true);
    assert!(round > 4.0, "a member walked into the fire: nearest {round:.1} cells");
    assert!(t.is_some(), "never arrived (nearest {round:.1} cells)");
}
