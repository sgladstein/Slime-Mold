//! **Condensed strikers** (2026-11-20, the thread "Make the slime a more
//! interesting enemy", the owner's pick on the balance thread's card that
//! day: "Condensed strikers + late lunge").
//!
//! The owner, 11-20 14:10Z: once the squad is big enough "the slime cannot
//! touch them, so it doesn't matter how much slime their is ... we could go
//! back to our early feature we had where the slime can condense and get
//! tougher. Although the visuals and mechanics of that would need to be
//! much improved from what we had before." And 14:16Z, of a hardening
//! idea: hardened slime "doesn't actually make this more challenging
//! because hardened slime doesn't attack you. It makes it easier."
//!
//! So late in a planet (under `below` of its peak), the slime's edge facing
//! the squad condenses: once a game second each ordinary walker within the
//! guns' reach and `band` dish cells of a soldier tries with chance
//! `share`, and in each block of fine cells the first that tries takes in
//! up to three of the rest (engine.wgsl's `condense`). A condensed walker
//! counts once, takes a blow for each walker it took in (and one more to
//! die), and bites once more for each. Mass and bite stay what they were:
//! no new slime. And it charges: every condensed walker runs for the
//! nearest soldier at `haste` of its pace, down the units' walking
//! distance, and never sits on a crystal. A shot's strike kills every
//! walker in its disc, so a quarter as many die a shot, and the same mass
//! crosses the guns' band four times as tough.
//!
//! The look is the simulation: a condensed walker is a bright bead in the
//! slime's gold, larger by what it holds, with a slow pulse, so where they
//! bunch they read as hot cores (view.wgsl's `draw_specks`). No bronze, no
//! outline.
//!
//! The old condensing (11-14 to 11-17, Reports/dead-ends.md) packed thick
//! slime anywhere: its lobes sat on their crystals and thick walkers got
//! stuck in the body. This one condenses only the edge facing the squad,
//! and every condensed walker charges.
//!
//! GPU only (the owner's 11-17 19:21Z card): a world that cannot condense
//! (`World::condense`, the CPU engine) never does.

use super::{Game, World};

/// The strikers' numbers. Off by default (the campaign and the sandbox are
/// frozen); a run turns it on for every planet (`run::ENEMY_RULES`).
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct StrikerRules {
    pub on: bool,
    /// The slime condenses while its mass is under this share of its peak.
    pub below: f32,
    /// How far past the guns' reach the condensing edge runs (dish cells).
    pub band: f32,
    /// The chance a walker in the band tries, once a game second.
    pub share: f32,
    /// A condensed walker's pace as it charges, times its own.
    pub haste: f32,
    /// The most walkers one takes in (1 to 15): it takes that many blows
    /// more and bites that many times more.
    pub most: u32,
}

impl Default for StrikerRules {
    fn default() -> Self {
        StrikerRules { on: false, below: 0.4, band: 12.0, share: 0.5, haste: 1.5, most: 3 }
    }
}

/// What the strikers have done.
#[derive(Clone, Debug, Default)]
pub struct Strikers {
    /// The engine's rule is on (from the first second the slime is under
    /// `below` of its peak; it stays on, so condensed walkers keep charging
    /// if the slime grows back).
    pub on: bool,
    /// The condensed walkers living at the last condensing, and the
    /// walkers they still held.
    pub live: u32,
    pub held: u32,
    /// The walkers taken in, all told.
    pub took: u64,
}

impl<W: World> Game<W> {
    /// Once a game second: condense the edge facing the squad while the
    /// slime is under `below` of its peak.
    pub(super) fn time_strikers(&mut self) {
        let r = self.rules.strikers;
        if !r.on || r.haste <= 0.0 {
            if std::mem::take(&mut self.strikers.on) {
                self.world.set_strikers(None);
            }
            return;
        }
        let low = self.peak > 0 && (self.mould as f32) < r.below * self.peak as f32;
        if !low && !self.strikers.on {
            return;
        }
        self.world.set_strikers(Some(r.haste));
        self.strikers.on = true;
        let share = if low && self.soldiers.iter().any(|s| s.alive) { r.share } else { 0.0 };
        let reach = (self.gun_range_max() + r.band).max(0.0) * self.world.scale();
        if let Some((live, held, took)) = self.world.condense(reach, share, r.most) {
            (self.strikers.live, self.strikers.held) = (live, held);
            self.strikers.took += took as u64;
        }
    }
}
