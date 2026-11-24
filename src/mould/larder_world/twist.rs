//! **Planet twists** (2026-11-24, Direction's job for Milestone 2's test 2,
//! "a run he wants to replay"; the Lab's prototype,
//! `Reports/lab-twists-2026-11-24/`): one condition a run can roll on a
//! planet, a change to the slime's own walk, so the picture is the change.
//!
//! - **The cold snap** (`Twist::Cold`): from `start` for `len` seconds the
//!   slime's step is `cold_step` times its own (0.3): its streams break
//!   into beads and crawl, and that minute is the one to push. The Lab's
//!   planet 1 (run seed 1, idle squad): at 0:20 the body reached 105 cells
//!   from its centre against 137, and the squad shot about 30% fewer
//!   walkers while it lasted.
//! - **The wind** (`Twist::Wind`): all planet, every step a walker takes is
//!   also pushed `wind` times its own step towards `wind_angle` degrees (0
//!   east, 90 south): the body leans and creeps downwind and lays its trail
//!   there. The Lab's planet 1: the body's centre 8.5 cells east in 50 s,
//!   and it swallowed the north-east crystal it otherwise reached with a
//!   thin arm; wind towards the squad killed 60% more by 1:30 (idle).
//!
//! Rejected for the game (the Lab's third): night, the sensors reaching
//! 2.5 times as far, which only makes the slime better at what it does
//! with no new choice for the player; it stays on the Lab's branch.
//!
//! The run rolls the twist (`run::Run::twist`); here it is driven once a
//! game second (`time_twist`), stateless, so a loaded game is driven the
//! same. GPU only (`World::set_twist`; the CPU engine takes no new rules).

use super::{Game, World};

/// A planet's twist (`TwistRules::kind`).
#[derive(Clone, Copy, Debug, PartialEq, Eq, Default)]
#[derive(serde::Serialize, serde::Deserialize)]
pub enum Twist {
    #[default]
    None,
    Cold,
    Wind,
}

/// The twist's numbers (`Rules::twist`). Times in game seconds.
#[derive(Clone, Copy, Debug, PartialEq)]
#[derive(serde::Serialize, serde::Deserialize)]
pub struct TwistRules {
    pub kind: Twist,
    /// The cold snap: from `start` for `len` seconds, the step times
    /// `cold_step`.
    pub start: f32,
    pub len: f32,
    pub cold_step: f32,
    /// The wind: a step's push, times the slime's own step, towards
    /// `wind_angle` degrees (0 east, 90 south). The Lab's 0.25.
    pub wind: f32,
    pub wind_angle: f32,
}

impl Default for TwistRules {
    fn default() -> Self {
        TwistRules { kind: Twist::None, start: 20.0, len: 60.0, cold_step: 0.3, wind: 0.25, wind_angle: 0.0 }
    }
}

/// What the HUD tells of a planet's twist now (`Game::twist_now`).
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum TwistNow {
    /// The cold snap comes in this many seconds.
    ColdIn(f32),
    /// The cold snap holds for this many seconds more.
    Cold(f32),
    /// The cold snap is over.
    ColdOver,
    /// The wind blows towards this many degrees (0 east, 90 south).
    Wind(f32),
}

impl TwistRules {
    /// The cold snap holds at game second `t`.
    pub fn cold_at(&self, t: f32) -> bool {
        self.kind == Twist::Cold && t >= self.start && t < self.start + self.len
    }
}

impl<W: World> Game<W> {
    /// Once a game second: the twist's step and wind to the world.
    pub(super) fn time_twist(&mut self) {
        let r = self.rules.twist;
        let (step, wind) = match r.kind {
            Twist::None => return,
            Twist::Cold => (if r.cold_at(self.time()) { r.cold_step.max(0.01) } else { 1.0 }, (0.0, 0.0)),
            Twist::Wind => {
                let a = r.wind_angle.to_radians();
                (1.0, (r.wind * a.cos(), r.wind * a.sin()))
            }
        };
        self.world.set_twist(step, wind);
    }

    /// The planet's twist as the HUD tells it (None without one).
    pub fn twist_now(&self) -> Option<TwistNow> {
        let r = self.rules.twist;
        let t = self.time();
        match r.kind {
            Twist::None => None,
            Twist::Wind => Some(TwistNow::Wind(r.wind_angle)),
            Twist::Cold if t < r.start => Some(TwistNow::ColdIn(r.start - t)),
            Twist::Cold if r.cold_at(t) => Some(TwistNow::Cold(r.start + r.len - t)),
            Twist::Cold => Some(TwistNow::ColdOver),
        }
    }
}
