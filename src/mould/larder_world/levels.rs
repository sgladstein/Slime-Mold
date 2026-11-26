//! **The slime's levels** (2026-11-26, the thread "Make the slime's
//! progression much stronger"). The owner, 08:24Z: "some toughness scaling
//! and speed scaling is needed to make the slime more dangerous and
//! scarier. You could change slime color so their are 3-8 levels of slime.
//! The slimes could be mixed to so one area on a later planet is the easier
//! to kill orange slime and this other crystal is full of stronger more
//! dangerous slime."
//!
//! Each crystal has a level, 0 to `max` (the run's by planet,
//! `run::LEVEL_MAX`): the slime's home crystal (crystal 0) the highest, the
//! rest rolled from the planet's seed, lower levels likelier. Slime born on
//! a crystal takes its level, and its children keep it as they spread, so
//! a crystal's slime is its colour and a border between two is where they
//! mix (engine.wgsl's `GRADE_BITS`; the view's `level_rgb`, orange at level
//! 0 through red and crimson to violet). A level takes more blows, steps
//! faster and bites harder (`gpu::Settings::level_blows`, `level_speed`,
//! `level_bite`, set by the run's `enemy_keys`).
//!
//! Driven once a game second (`time_levels`) and before a head start
//! (`apply_levels`), stateless, so a loaded game is driven the same. GPU
//! only (`World::set_levels`; the CPU engine takes no new rules).

use super::{Game, Rules, World};

/// The levels' numbers (`Rules::levels`).
#[derive(Clone, Copy, Debug, PartialEq, Default)]
#[derive(serde::Serialize, serde::Deserialize)]
pub struct LevelRules {
    /// The highest level on the planet (0: every crystal level 0, off).
    pub max: u32,
    /// The roll's seed (the level's).
    pub seed: u64,
    /// How the roll leans low: a crystal's level is `max` x u^`lean` for
    /// a uniform u, rounded down (1 even; 1.5 a third more at the bottom).
    pub lean: f32,
}

/// Each of `n` crystals' level under `r`: the home crystal (0) at `max`,
/// the rest rolled.
pub fn crystal_levels(r: &LevelRules, n: usize) -> Vec<u32> {
    (0..n)
        .map(|c| {
            if r.max == 0 {
                return 0;
            }
            if c == 0 {
                return r.max;
            }
            let h = crate::mould::run::mix(r.seed ^ (c as u64).wrapping_mul(0x9E37_79B9_7F4A_7C15));
            let u = (h >> 11) as f32 / (1u64 << 53) as f32;
            ((r.max + 1) as f32 * u.powf(r.lean.max(0.1))).floor().min(r.max as f32) as u32
        })
        .collect()
}

/// The crystals' levels to `world` (before the game starts: a head start).
pub fn apply_levels<W: World>(world: &mut W, rules: &Rules) {
    if rules.levels.max > 0 {
        let n = world.crystals().len();
        world.set_levels(&crystal_levels(&rules.levels, n));
    }
}

impl<W: World> Game<W> {
    /// Once a game second: the crystals' levels to the world (it writes
    /// them only when they change).
    pub(super) fn time_levels(&mut self) {
        apply_levels(&mut self.world, &self.rules);
    }

    /// Each crystal's level, in `World::crystals`' order.
    pub fn crystal_levels(&self) -> Vec<u32> {
        crystal_levels(&self.rules.levels, self.world.crystals().len())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn the_home_crystal_is_the_highest_and_the_rest_vary() {
        let r = LevelRules { max: 5, seed: 7, lean: 1.5 };
        let l = crystal_levels(&r, 40);
        assert_eq!(l[0], 5);
        assert!(l.iter().all(|&v| v <= 5));
        assert!(l.contains(&0) && l[1..].iter().any(|&v| v >= 3), "a mix: {l:?}");
        assert_eq!(crystal_levels(&LevelRules::default(), 5), vec![0; 5], "off: all level 0");
    }
}
