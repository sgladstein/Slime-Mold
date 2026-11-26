//! **The hunt** (2026-11-26, the Lab; brief
//! `Reports/review-2026-11-25/lab-brief-slime-reach.md`): the slime follows
//! the squad's trail.
//!
//! The 11-25 review found the body could not reach a squad that keeps
//! walking: its own pull on the squad was the panic charge, the guns'
//! reach plus 5 cells, and a kiting squad of 15 on planet 2 lost no
//! soldier by 5:00 on 5 of 6 run seeds with the scheduled threats off.
//! The soldiers' smell is in the food scent, which keeps 0.97 a step and
//! so is gone about a second after the squad has walked on.
//!
//! The trail here is the brief's third direction, a prey scent that leads:
//! once a game second the places the soldiers stand are kept for as long
//! as they are warm, and the slime's field is the walking distance to the
//! nearest of them with each place's age added as distance
//! (`Rules::hunt_lag` dish cells a second). With the lag faster than the
//! squad walks, a place a second newer is always nearer down the field
//! than the one behind it, so a walker that meets the path goes along it
//! toward the squad, the way an animal follows a scent trail and not
//! straight at its prey. The field stops at `Rules::hunt_reach`, so a
//! place older than reach / lag seconds pulls nothing: a squad that gets
//! far enough ahead leaves the slime a cold trail and it stops, which is
//! the brief's fourth direction (giving up) without a clock. Calm walkers
//! lean down it by `gpu::Settings::hunt_pull` (engine.wgsl's `hunt_dist`).
//!
//! Built off the frame and set a game second later, as the lure's field
//! (`time_lure_paths`), so the game is the same however fast the thread.

use super::raids::{round_walk_from, Way};
use super::*;

/// The squad's warm places and the trail being built.
#[derive(Default)]
pub(super) struct Hunt {
    /// Each game second's soldier cells (fine cells, x | y << 16), oldest
    /// first, with the second they were taken.
    places: std::collections::VecDeque<(u64, Vec<u32>)>,
    /// The trail started a second ago.
    way: Option<Way>,
    /// Whether the world holds a trail to clear once none is warm.
    set: bool,
}

impl<W: World> Game<W> {
    /// Once a game second: the trail started a second ago is set, today's
    /// places are kept, the cold ones dropped, and a new trail is started.
    pub(super) fn time_hunt(&mut self) {
        if !self.world.hunt_pull() {
            return;
        }
        if let Some(way) = self.hunt.way.take() {
            self.world.set_hunt_field(way.finish());
        } else if self.hunt.set {
            self.world.set_hunt_field(Vec::new());
            self.hunt.set = false;
        }
        let r = self.rules;
        let sc = self.world.scale();
        let now = self.world.ticks() / r.tps.max(1) as u64;
        let (w, h) = self.world.size();
        let cell = |x: f32, y: f32| (x.clamp(0.0, w as f32 - 1.0) as u32) | ((y.clamp(0.0, h as f32 - 1.0) as u32) << 16);
        // A masked soldier (the toolkit's mask, smell 0) leaves no trail.
        let here: Vec<u32> = self.soldiers.iter().filter(|s| s.alive && s.smell > 0.0).map(|s| cell(s.x, s.y)).collect();
        self.hunt.places.push_back((now, here));
        let lag = r.hunt_lag.max(0.0) * sc;
        let reach = r.hunt_reach.max(0.0) * sc;
        // Places whose age alone takes them past the reach are cold.
        let warm = |at: u64| (now.saturating_sub(at) as f32) * lag <= reach;
        while self.hunt.places.front().is_some_and(|p| !warm(p.0)) {
            self.hunt.places.pop_front();
        }
        let from: Vec<(u32, u32)> = self.hunt.places.iter().flat_map(|(at, cells)| {
            let age = ((now - at) as f32 * lag).round() as u32;
            cells.iter().map(move |&c| (c, age))
        }).collect();
        if from.is_empty() {
            return;
        }
        let Some((agar, gw, gh)) = self.slime_ground() else { return };
        let limit = reach as u32;
        self.hunt.way = Some(Way::start_with(move || round_walk_from(&agar, gw, gh, &from, limit)));
        self.hunt.set = true;
    }
}
