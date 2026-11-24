//! **Trodden ground, a squad's firebreak** (squad thread, 2026-11-24;
//! Direction's option A, `Rules::trample_to`).
//!
//! A soldier holding its ground (within `trample_speed`, 2 dish cells, of
//! where it stood a second before) treads the plants within `trample_r`
//! dish cells of it down to `trample_to`, once a game second, the kind
//! kept. The ground a squad holds turns to short
//! plants, which the picture shows as smaller, thinner crowns (the plants'
//! cover goes with their density, `view.wgsl` `plant_cover`), and fire
//! hardly crosses it: a fire's catch goes with the plants on the cell, and
//! its heat tops out at the plants over `fire_full` (0.8), so at 0.1 a
//! soldier standing there takes at most an eighth of the harm.
//!
//! Burning ground is never trodden out (the IGNITE pass's mode 2 skips a
//! cell with heat), so a squad walking into fire, or one already standing
//! in it, burns as before; a moving soldier treads nothing.
//!
//! Why: planet 3 is jungle, a blast's crater lights it (`fire_light_blast`:
//! Needle Blast's, and a falling star's until 11-24, `StarRules`) and Tinder
//! lays dry brush along each shot, so fire ran in to a squad
//! holding a crystal and burned it: the run bot's Tinder rush on map 3 lost
//! 26 soldiers to 7 in 15 s, 60 burned in the game, against 7 with no
//! cards. Rejected before building (11-24, Direction): Tinder brush that
//! burns nobody of its own squad, a rule that reads as a cheat; a wider
//! `trail_fuel_gap` (12: 45 and 42 burned on maps 3 and 4, from 60 and 33),
//! and less fuel (0.5: 41 and 7), each of which weakens the card instead of
//! giving the squad a way to hold ground.
//!
//! GPU only (the 11-17 rule: new slime and plant rules on the GPU engine);
//! the CPU world's `World::trample` does nothing.
use super::*;

impl<W: World> Game<W> {
    /// Once a game second: each living soldier within `trample_speed` dish
    /// cells of where it stood a second ago treads the ground round it
    /// (`World::trample`). The test is the second's net move, not the
    /// frame's speed: a squad holding a crystal under attack shuffles 1 to
    /// 3 cells a second as it shoots and makes room, and the run bot's
    /// squads on planet 3 never stood a whole second under 1 cell a second
    /// while they fought (first build, 11-24: discs only while they waited
    /// for the landing, none in the burning from 60 s on). Cost: one IGNITE
    /// pass over the dish a second for every three treading soldiers who
    /// don't stand within a dish cell of another's disc.
    pub(super) fn trample(&mut self, dt: f32) {
        let r = self.rules;
        if r.trample_to <= 0.0 || !r.squads {
            return;
        }
        self.trample_clock += dt;
        if self.trample_clock < 1.0 {
            return;
        }
        self.trample_clock -= 1.0;
        let sc = self.s();
        self.still.resize(self.soldiers.len(), (f32::NAN, f32::NAN));
        let mut discs: Vec<(f32, f32, f32, f32)> = Vec::new();
        for (s, was) in self.soldiers.iter().zip(self.still.iter_mut()) {
            let (x, y) = (s.x / sc, s.y / sc);
            let held = s.alive && (x - was.0).hypot(y - was.1) <= r.trample_speed;
            *was = if s.alive { (x, y) } else { (f32::NAN, f32::NAN) };
            // A blob of soldiers a cell apart treads one disc, not one
            // each: the discs overlap by more than half anyway.
            if !held || discs.iter().any(|d| (d.0 - x).hypot(d.1 - y) < 1.0) {
                continue;
            }
            discs.push((x, y, r.trample_r, r.trample_to));
        }
        self.world.trample(&discs);
    }
}
