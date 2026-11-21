//! **The slime's size follows the crystal it holds** (`Rules::ceiling`,
//! 2026-11-21, the thread "Make the slime a more interesting enemy").
//!
//! The owner, 11-21 06:30Z, on the rule that "crystals never run out from
//! eating or pumping, and only your squad can empty one by draining it":
//! "this 100%"; at 06:34Z he picked option 1 of three: the slime's size
//! follows the food it holds. With `crystal_keep` a crystal the slime sits
//! on feeds it for ever, so without a ceiling it refills its slots (twice
//! its landing walkers) on any crystal, and the squad taking crystals off
//! it changes nothing you can see.
//!
//! Once a game second the slime's ceiling is set from the crystal cells it
//! covers (with charge and a walker on them, the GPU's `pilesum`), smoothed
//! over `CEILING_SMOOTH` seconds, times its walkers a covered cell: its
//! density at landing, at least `Rules::ceiling` and at most
//! `CEILING_DENSITY_MOST`. So a planet starts with
//! the slime at or under its ceiling (its start plays as before), and the
//! ceiling then rises and falls with the crystal it covers, never over
//! `Rules::ceiling_roof` times its landing walkers or its slots.
//!
//! - Over the ceiling no walker is born (the engine's `CEIL`, species 0's
//!   birth cap).
//! - Over the ceiling its upkeep rises, to `Rules::ceiling_upkeep` times
//!   at `CEILING_OVER` over, so the walkers that lost their food starve
//!   within a minute or so and the slime shrinks to what it still holds;
//!   at or under it the upkeep is the shape's.
//!
//! Rejected (measured the same morning, planet 1, run seeds 1-3, greedy):
//! - The upkeep raised everywhere, all the time: x2 cut the slime at 1:00
//!   by 29% (7,959 against 11,261 on seed 1), x3 by 52%. At landing most of
//!   the slime is off its crystal on its start energy, so it starved.
//! - The floor at the landing walkers (the plan as first written): a slime
//!   that held any crystal at all regrew to its landing size, so draining
//!   its crystals did not shrink it. A floor on the density instead keeps
//!   the start and lets the size follow the crystal.
//! - A ceiling from the food it ate a second (its income) over the income
//!   at 1:00, enforced by upkeep alone (a prototype): income swings with
//!   surges and fights, so the ceiling swung with it, and upkeep never
//!   stopped births.

use super::{Game, World};

/// Seconds the covered cells are smoothed over: a surge or a shot that
/// empties a crystal's rim for a moment does not drop the ceiling.
pub const CEILING_SMOOTH: f64 = 5.0;

/// The most walkers a covered cell the landing density may take: a slime
/// that lands mostly off its crystal would otherwise get a ceiling at its
/// roof, and the rule would do nothing there (the review thread, 11-21;
/// planet 1 landed with 14 to 21).
pub const CEILING_DENSITY_MOST: f64 = 25.0;

/// How far over its ceiling the slime pays the whole of
/// `Rules::ceiling_upkeep` (a share; under it, in proportion).
pub const CEILING_OVER: f64 = 0.2;

#[derive(Clone, Debug, Default)]
pub struct Ceiling {
    /// The walkers the slime landed with, once read (0 before).
    pub start: usize,
    /// Its walkers a covered crystal cell: its landing density, at least
    /// `Rules::ceiling`.
    pub density: f64,
    /// Covered crystal cells, smoothed.
    pub covered: f64,
    /// The ceiling now (0 before the first second or with the rule off).
    pub now: u32,
    /// The shape's upkeep, before any raise.
    pub upkeep: f32,
    /// The upkeep's factor now (1 at or under the ceiling).
    pub factor: f32,
}

impl<W: World> Game<W> {
    /// Once a game second, after `check_broken`: the slime's birth ceiling
    /// from the crystal it covers, and its upkeep over it
    /// (`Rules::ceiling`).
    pub(super) fn time_ceiling(&mut self) {
        let r = self.rules;
        if r.ceiling <= 0.0 {
            return;
        }
        let covered = self.world.crystal_covered() as f64;
        let (slots, mould) = (self.world.slots(), self.mould);
        if self.ceiling.start == 0 {
            let c = &mut self.ceiling;
            c.start = mould.max(1);
            c.covered = covered;
            c.density = (mould as f64 / covered.max(1.0)).clamp(r.ceiling as f64, CEILING_DENSITY_MOST.max(r.ceiling as f64));
            c.upkeep = self.world.upkeep();
        }
        let c = &mut self.ceiling;
        c.covered += (covered - c.covered) / CEILING_SMOOTH;
        let mut ceil = (c.covered * c.density).min(c.start as f64 * r.ceiling_roof.max(1.0) as f64);
        if slots > 0 {
            ceil = ceil.min(slots as f64);
        }
        c.now = ceil.round().max(1.0) as u32;
        self.world.set_ceiling(c.now);
        // A broken slime starves on its own rule (`check_broken`), which
        // saves and gives back the upkeep it found.
        if self.broken.is_some() {
            return;
        }
        let over = ((mould as f64 / ceil.max(1.0) - 1.0) / CEILING_OVER).clamp(0.0, 1.0);
        c.factor = 1.0 + (r.ceiling_upkeep - 1.0).max(0.0) * over as f32;
        self.world.set_upkeep(c.upkeep * c.factor);
    }
}
