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
        let at = self.heart_site();
        let sc = self.world.scale();
        let steps = (r.period * self.rules.tps as f32 * self.world.fine_steps().max(1) as f32).round() as u32;
        self.world.set_heart(at, r.lean, steps.max(2), (r.len * sc).round().max(1.0) as u32);
        self.heart_at = at;
    }

    /// The home crystal while the slime holds it, else the crystal it holds
    /// with the most walkers (fine cells); None with no slime on any.
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
        let home = counts.iter().find(|(at, _)| (at.0 - self.core.0).hypot(at.1 - self.core.1) < sc);
        if let Some(&(at, n)) = home {
            if n as f32 >= self.rules.heart.held {
                return Some(at);
            }
        }
        counts.iter().filter(|c| c.1 > 0).max_by_key(|c| c.1).map(|c| c.0)
    }
}
