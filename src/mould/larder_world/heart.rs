//! **The heartbeat** (2026-11-20, the thread "Make the slime a more
//! interesting enemy", its third rule after fruiting and waves). The owner,
//! 11-20 05:46Z: "once I get strong enough there is just lots of shooting
//! through masses of uniform orange".
//!
//! Real slime mould pulses: its cytoplasm streams out and back through the
//! whole body every minute or two, in waves you can watch. Here the slime
//! beats from its heart, the home crystal (`Game::core`) while it holds it,
//! else the crystal it holds with the most slime: every `period` seconds a
//! beat runs outward through the body as a ring `len` dish cells long. As
//! a beat passes, a calm walker leans away from the heart and steps faster
//! (engine.wgsl's `heart_beat`), so the body's edge swells toward the
//! squad, then ebbs as it leans home. The picture is the simulation: the
//! swell is the walkers themselves.
//!
//! What it does to the game: the front is not a still wall to shoot into.
//! It comes at you in pulses you can see coming and time, and between them
//! it draws back.
//!
//! GPU only (the owner's 11-17 19:21Z card): `World::set_heart` does
//! nothing on the CPU engine.

use super::{Game, World};

/// The heartbeat's numbers. Off by default (the campaign and the sandbox
/// are frozen); a run turns it on for every planet (`run::ENEMY_RULES`).
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct HeartRules {
    pub on: bool,
    /// A calm walker's lean outward on the beat (and home between), in the
    /// readings' units (the lure's pull is 10).
    pub lean: f32,
    /// Game seconds a beat, and its wavelength in dish cells.
    pub period: f32,
    pub len: f32,
    /// The home crystal is the heart while this many walkers stand on it
    /// (within its radius); else the crystal with the most.
    pub held: f32,
}

impl Default for HeartRules {
    fn default() -> Self {
        HeartRules { on: false, lean: 25.0, period: 6.0, len: 80.0, held: 30.0 }
    }
}

impl<W: World> Game<W> {
    /// Once a game second: where the heart is, sent to the world (or none).
    pub(super) fn time_heart(&mut self) {
        let r = self.rules.heart;
        if !r.on || r.lean <= 0.0 || r.period <= 0.0 || r.len <= 0.0 {
            if self.heart_at.take().is_some() {
                self.world.set_heart(None, 0.0, 0, 0);
            }
            return;
        }
        // A beat runs out from one place: the heart moves at most once a
        // beat. Late in a planet (planet 2 map 1, lavapipe, the strategy
        // bot) the crystals' slime swung from 214 to 8 in two seconds as
        // the squad and the surges went over them, so the fullest changed
        // nearly every second and every walker's beat and lean turned
        // with it: 33 moves in the last 90 s, and half the slime of the
        // fullest (`pick_heart`) alone still left 36.
        let now = self.world.ticks();
        let beat = (r.period * self.rules.tps as f32) as u64;
        let at = if self.heart_at.is_some() && now < self.heart_since + beat { self.heart_at } else { self.heart_site() };
        if at != self.heart_at {
            self.heart_since = now;
        }
        let sc = self.world.scale();
        let steps = (r.period * self.rules.tps as f32 * self.world.fine_steps().max(1) as f32).round() as u32;
        self.world.set_heart(at, r.lean, steps.max(2), (r.len * sc).round().max(1.0) as u32);
        self.heart_at = at;
    }

    /// The home crystal while the slime holds it, else the crystal the
    /// heart beats from now while that still holds half the slime of the
    /// fullest, else the crystal it holds with the most walkers (fine
    /// cells); None with no slime on any (`pick_heart`).
    pub fn heart_site(&self) -> Option<(f32, f32)> {
        let sc = self.world.scale();
        let bin = (sc as usize).max(1);
        let grid = self.world.mould_grid(bin)?;
        let (w, _) = self.world.size();
        let gw = w.div_ceil(bin);
        let gh = grid.len() / gw.max(1);
        let on = |at: (f32, f32), r: f32| -> u32 {
            let mid = |i: usize| (i as f32 + 0.5) * bin as f32;
            let span = |c: f32, n: usize| ((((c - r) / bin as f32).floor().max(0.0)) as usize)..((((c + r) / bin as f32).ceil().max(0.0)) as usize + 1).min(n);
            span(at.1, gh).flat_map(|y| span(at.0, gw).map(move |x| (x, y))).filter(|&(x, y)| (mid(x) - at.0).hypot(mid(y) - at.1) <= r).map(|(x, y)| grid[y * gw + x]).sum()
        };
        let crystals = self.world.crystals();
        let counts: Vec<((f32, f32), u32)> = crystals.iter().enumerate().map(|(c, &(at, _))| (at, on(at, self.world.crystal_r(c).max(sc)))).collect();
        let near = |p: (f32, f32)| counts.iter().position(|(at, _)| (at.0 - p.0).hypot(at.1 - p.1) < sc);
        let n: Vec<u32> = counts.iter().map(|c| c.1).collect();
        pick_heart(&n, near(self.core), self.heart_at.and_then(near), self.rules.heart.held).map(|i| counts[i].0)
    }
}

/// Which crystal the heart beats from, of crystals holding `n` walkers
/// each: `home` while it holds `held`; else `was` (the one it beats from
/// now) while it holds at least half what the fullest does; else the
/// fullest. None with no slime on any.
///
/// Why `was` (the bug check, 2026-11-20): picked afresh each second, the
/// fullest crystal changed with every surge once the slime lost its home,
/// and every walker's beat and lean turned to the new centre at once. On
/// planet 2 map 1 (lavapipe, strikers off, the strategy bot) the heart
/// stayed home for 3:35, then moved 33 times in the last 90 s, most often
/// after one or two seconds: a 6 s beat never ran out from one place.
fn pick_heart(n: &[u32], home: Option<usize>, was: Option<usize>, held: f32) -> Option<usize> {
    if let Some(h) = home.filter(|&h| n[h] as f32 >= held) {
        return Some(h);
    }
    let best = (0..n.len()).filter(|&i| n[i] > 0).max_by_key(|&i| n[i])?;
    match was {
        Some(w) if n[w] > 0 && 2 * n[w] >= n[best] => Some(w),
        _ => Some(best),
    }
}

#[cfg(test)]
mod tests {
    use super::pick_heart;

    #[test]
    fn the_heart_stays_where_it_beats_until_another_crystal_holds_twice_the_slime() {
        // Home while it holds `held` (30), whatever else holds more.
        assert_eq!(pick_heart(&[30, 500, 0], Some(0), Some(1), 30.0), Some(0));
        // Home lost: the fullest, then kept while it holds half the fullest.
        assert_eq!(pick_heart(&[10, 200, 150], Some(0), None, 30.0), Some(1));
        assert_eq!(pick_heart(&[10, 200, 300], Some(0), Some(1), 30.0), Some(1));
        assert_eq!(pick_heart(&[10, 200, 400], Some(0), Some(1), 30.0), Some(1));
        // Under half, or empty: to the fullest.
        assert_eq!(pick_heart(&[10, 199, 400], Some(0), Some(1), 30.0), Some(2));
        assert_eq!(pick_heart(&[1, 0, 4], Some(0), Some(1), 30.0), Some(2));
        // No slime on any crystal: no heart.
        assert_eq!(pick_heart(&[0, 0], Some(0), Some(1), 30.0), None);
        assert_eq!(pick_heart(&[], None, None, 30.0), None);
    }
}
