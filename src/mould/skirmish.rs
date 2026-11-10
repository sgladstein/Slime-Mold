//! The field skirmish: the RTS thread's first playable demo (owner,
//! 2026-11-10 20:33Z: "lets focus on actually building playable demos").
//!
//! You and a rival mould share a field of 23 small food piles, mirrored so
//! neither side has the better map; whoever eats more of the food by the
//! bell wins. Everything here was measured first, in the RTS red team's
//! harness (`Reports/rts-design-2026-11-10/field/README.md`):
//!
//! - **The field.** The Wild One's 7 piles made a dish where the opening
//!   decided everything and no verb moved much. 23 smaller piles grow about
//!   6,600 walkers by 60 s (Jones's density), where the moulds' growth
//!   habits beat each other in a cycle and a wall across the rival's road
//!   moves the food share.
//! - **Habits** (keys 1-4). How your walkers sense: spread (the shipped
//!   22.5 degrees), hunt (drawn to the rival's trail), maze (67.5) and cord
//!   (45). On this layout, with 32 games a pairing: hunt beat maze 32/32,
//!   maze beat cord 30/32, cord beat hunt 25/32, and all three beat spread
//!   28-31 of 32. A switch shows: the mould's texture changes within
//!   seconds, and the rival's habit is named on screen.
//! - **Salt** across the rival's road: on the field it moved share by 0.02 on
//!   14 of 16 seeds and beat a randomly placed wall on 14 of 16. Two walls,
//!   one coming back every 30 s.
//! - **The lamp** pushes both moulds; it moved share a little (12 of 16 up).
//!
//! The rival is a scripted hand (`Ai`): it answers your habit with the one
//! that beats it after a reaction delay, and walls your road when it is
//! behind. Left out: Taut's pull (always best at full, so not a choice yet),
//! oats and the knife (they lost share or moved nothing on the field).

use super::level::{Level, PileSpec, Tools, Win};
use super::{Dish, Params, Phase, Tool, INOCULUM, STEPS_PER_SECOND};

const SPS: u64 = STEPS_PER_SECOND as u64;

const fn small(x: f32, y: f32) -> PileSpec {
    PileSpec {
        x,
        y,
        r: 4.0,
        food: 15_000.0,
        arrives: None,
    }
}

const fn home(x: f32, y: f32) -> PileSpec {
    PileSpec {
        x,
        y,
        r: 6.0,
        food: 30_000.0,
        arrives: None,
    }
}

/// The scatter layout (the field sampler's seed 7, at least 28 cells
/// apart): nine piles a side mirrored about x = 128, three on the centre
/// line, and the two homes. 375k food.
static PILES: [PileSpec; 23] = [
    home(40.0, 80.0),
    small(78.0, 20.0),
    small(178.0, 20.0),
    small(67.0, 61.0),
    small(189.0, 61.0),
    small(18.0, 71.0),
    small(238.0, 71.0),
    small(56.0, 126.0),
    small(200.0, 126.0),
    small(110.0, 17.0),
    small(146.0, 17.0),
    small(98.0, 51.0),
    small(158.0, 51.0),
    small(92.0, 108.0),
    small(164.0, 108.0),
    small(96.0, 142.0),
    small(160.0, 142.0),
    small(65.0, 96.0),
    small(191.0, 96.0),
    small(128.0, 40.0),
    small(128.0, 80.0),
    small(128.0, 120.0),
    home(216.0, 80.0),
];

/// The rival's home, the last pile.
const RIVAL_HOME: (f32, f32) = (216.0, 80.0);
const MY_HOME: (f32, f32) = (40.0, 80.0);

/// Salt walls in stock at the start, and the most you can hold.
pub const SALT_STOCK: u32 = 2;
/// Seconds per salt wall coming back, up to `SALT_STOCK`.
pub const SALT_REGROW: u64 = 30;
/// Seconds between habit switches, for you and the rival alike.
pub const SWITCH_COOLDOWN: u64 = 10;

pub static FIELD: Level = Level {
    key: "field",
    name: "THE FIELD",
    goal: [
        "EAT MORE OF THE FOOD THAN THE RIVAL.",
        "KEYS 1-4 CHANGE HOW YOUR MOULD GROWS.",
    ],
    twist: "HUNT BEATS MAZE, MAZE CORD, CORD HUNT.",
    piles: &PILES,
    salt: &[],
    tools: Tools {
        oats: 0,
        oats_regrow: 0,
        salt: SALT_STOCK,
        snips: 0,
        water: 0,
        water_regrow: 0,
    },
    seconds: 150,
    sensor_deg: 22.5,
    rival: Some(22),
    // Provisional, not yet measured against hands: a share above half
    // means you out-ate it.
    win: Win::Rival { bars: [0.55, 0.62] },
};

/// How a mould's walkers sense, as the harness's grains (`grain` in the
/// red team's `main.rs`).
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Habit {
    Spread,
    Hunt,
    Maze,
    Cord,
}

impl Habit {
    pub const ALL: [Habit; 4] = [Habit::Spread, Habit::Hunt, Habit::Maze, Habit::Cord];

    pub fn name(self) -> &'static str {
        match self {
            Habit::Spread => "spread",
            Habit::Hunt => "hunt",
            Habit::Maze => "maze",
            Habit::Cord => "cord",
        }
    }

    /// Sensor angle, turn angle (degrees) and how the walkers take the
    /// other mould's trail (`Params::foe_repel`; below 0 drawn to it).
    fn numbers(self) -> (f32, f32, f32) {
        match self {
            Habit::Spread => (22.5, 45.0, 1.0),
            Habit::Hunt => (22.5, 45.0, -1.0),
            Habit::Maze => (67.5, 67.5, 1.0),
            Habit::Cord => (45.0, 45.0, 1.0),
        }
    }

    pub fn apply(self, p: &mut Params) {
        let (sensor, turn, repel) = self.numbers();
        p.sensor_angle = sensor.to_radians();
        p.rotate_angle = turn.to_radians();
        p.foe_repel = repel;
    }

    /// The habit that beats this one on the field.
    pub fn counter(self) -> Habit {
        match self {
            Habit::Spread | Habit::Maze => Habit::Hunt,
            Habit::Hunt => Habit::Cord,
            Habit::Cord => Habit::Maze,
        }
    }
}

/// The scripted rival.
#[derive(Clone, Debug)]
pub struct Ai {
    /// Steps it takes to answer a habit you switch to.
    pub react: u64,
    /// A switch it has decided on, and the step it makes it.
    pending: Option<(Habit, u64)>,
    ready_at: u64,
    salt_left: u32,
    next_salt: u64,
}

impl Ai {
    /// `react_s`: seconds to answer your habit (8 a fair rival).
    pub fn new(react_s: u64) -> Ai {
        Ai {
            react: react_s * SPS,
            pending: None,
            ready_at: 0,
            salt_left: SALT_STOCK,
            next_salt: 30 * SPS,
        }
    }
}

pub struct Skirmish {
    pub dish: Dish,
    pub mine: Habit,
    pub theirs: Habit,
    /// The step your next switch is allowed.
    pub ready_at: u64,
    pub ai: Ai,
    salt_back_at: u64,
}

impl Skirmish {
    pub fn new(seed: u64) -> Skirmish {
        let mut dish = Dish::from_level_with(&FIELD, seed, FIELD.params_for());
        // An even start: the level gives a wild rival 600 walkers and a
        // bolder repel (The Wild One's); here it is your equal.
        dish.add_rival_of(RIVAL_HOME, INOCULUM);
        let p = dish.params.clone();
        if let Some(r) = dish.rival.as_mut() {
            r.params = p;
        }
        Skirmish {
            dish,
            mine: Habit::Spread,
            theirs: Habit::Spread,
            ready_at: 0,
            ai: Ai::new(8),
            salt_back_at: SALT_REGROW * SPS,
        }
    }

    /// Seconds until you may switch again (0: now).
    pub fn cooldown_s(&self) -> f32 {
        self.ready_at.saturating_sub(self.dish.t) as f32 / SPS as f32
    }

    /// Switch your habit. False while cooling down, or when nothing changes.
    pub fn switch(&mut self, h: Habit) -> bool {
        if h == self.mine || self.dish.t < self.ready_at || self.dish.phase != Phase::Growing {
            return false;
        }
        h.apply(&mut self.dish.params);
        self.mine = h;
        self.ready_at = self.dish.t + SWITCH_COOLDOWN * SPS;
        true
    }

    /// Your share of the food eaten so far.
    pub fn share(&self) -> f32 {
        let all = self.dish.ate + self.dish.wild_ate;
        if all < 1.0 {
            0.5
        } else {
            self.dish.ate / all
        }
    }

    /// Once a frame, before the dish steps: salt comes back, the rival acts.
    pub fn tick(&mut self) {
        let t = self.dish.t;
        if self.dish.phase != Phase::Growing {
            return;
        }
        if t >= self.salt_back_at {
            if self.dish.salt_left < SALT_STOCK {
                self.dish.salt_left += 1;
            }
            self.salt_back_at = t + SALT_REGROW * SPS;
        }
        self.rival_acts();
    }

    fn rival_acts(&mut self) {
        let t = self.dish.t;
        let want = self.mine.counter();
        match self.ai.pending {
            Some((h, at)) if h != want => self.ai.pending = Some((want, at.max(t + self.ai.react / 2))),
            None if self.theirs != want => self.ai.pending = Some((want, t + self.ai.react)),
            _ => {}
        }
        if let Some((h, at)) = self.ai.pending {
            if h == self.theirs {
                self.ai.pending = None;
            } else if t >= at && t >= self.ai.ready_at {
                if let Some(r) = self.dish.rival.as_mut() {
                    h.apply(&mut r.params);
                }
                self.theirs = h;
                self.ai.ready_at = t + SWITCH_COOLDOWN * SPS;
                self.ai.pending = None;
            }
        }
        // Behind, it walls your road to your most forward pile.
        if t >= self.ai.next_salt && self.ai.salt_left > 0 {
            self.ai.next_salt = t + SALT_REGROW * SPS;
            if self.share() > 0.5 {
                let (target, from) = front(&self.dish, false);
                let (dx, dy) = (target.0 - from.0, target.1 - from.1);
                let n = dx.hypot(dy).max(1.0);
                let mid = (from.0 + dx * 0.5, from.1 + dy * 0.5);
                let across = (-dy / n, dx / n);
                // The dish's salt stock is yours; the rival's walls are
                // laid through the same stroke, its own stock kept here.
                let keep = self.dish.salt_left;
                self.dish.salt_left = 1;
                let half = 30.0;
                let a = (mid.0 - across.0 * half, mid.1 - across.1 * half);
                if self.dish.begin_stroke(Tool::Salt, a.0, a.1) {
                    self.dish.drag_stroke(mid.0 + across.0 * half, mid.1 + across.1 * half);
                    self.dish.end_stroke();
                    self.ai.salt_left -= 1;
                }
                self.dish.salt_left = keep;
            }
        }
    }
}

/// The aim for a hand acting for one side against the other's front:
/// `actor_self` is true for your hand, false for the rival's. The target is
/// the other side's most forward pile (the one it holds, more of its eaters
/// than the actor's, nearest the actor's home), else the nearest pile it is
/// on at all, else the centre. Returns the target and the other side's
/// home, whose road to the target a wall should cross.
pub fn front(d: &Dish, actor_self: bool) -> ((f32, f32), (f32, f32)) {
    let (mine, theirs) = if actor_self {
        (MY_HOME, RIVAL_HOME)
    } else {
        (RIVAL_HOME, MY_HOME)
    };
    let near = |q: &&super::Pile| (q.x - mine.0).hypot(q.y - mine.1);
    // `eaters` are your mould's, `foe_eaters` the rival's.
    let other_on = |q: &super::Pile| if actor_self { q.foe_eaters } else { q.eaters };
    let actor_on = |q: &super::Pile| if actor_self { q.eaters } else { q.foe_eaters };
    let live = || d.piles.iter().filter(|q| !q.oats && q.food > 0.0);
    let target = live()
        .filter(|q| other_on(q) > actor_on(q))
        .min_by(|a, b| near(a).total_cmp(&near(b)))
        .or_else(|| {
            live()
                .filter(|q| other_on(q) > 0)
                .min_by(|a, b| near(a).total_cmp(&near(b)))
        })
        .map(|q| (q.x, q.y))
        .unwrap_or((128.0, 80.0));
    (target, theirs)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn the_field_is_mirrored_and_even() {
        let s = Skirmish::new(1);
        assert_eq!(s.dish.piles.len(), 23);
        for p in &s.dish.piles {
            assert!(
                s.dish
                    .piles
                    .iter()
                    .any(|q| (q.x - (256.0 - p.x)).abs() < 1e-3 && q.y == p.y && q.food == p.food),
                "{p:?} has no mirror"
            );
        }
        let r = s.dish.rival.as_ref().unwrap();
        assert_eq!(s.dish.walkers.len(), r.walkers.len());
        assert_eq!(r.params.foe_repel, s.dish.params.foe_repel);
    }

    #[test]
    fn every_habit_is_beaten_by_another_and_the_cycle_closes() {
        for h in Habit::ALL {
            assert_ne!(h.counter(), h);
        }
        assert_eq!(Habit::Hunt.counter().counter().counter(), Habit::Hunt);
    }

    #[test]
    fn the_rival_answers_a_switch_after_its_delay_and_cooldowns_hold() {
        let mut s = Skirmish::new(2);
        let step = |s: &mut Skirmish, secs: u64| {
            let end = s.dish.t + secs * SPS;
            while s.dish.t < end {
                s.tick();
                s.dish.step();
            }
        };
        // It answers spread with hunt after its delay.
        step(&mut s, 9);
        assert_eq!(s.theirs, Habit::Hunt);
        assert!(s.switch(Habit::Cord));
        assert!(!s.switch(Habit::Maze), "a switch inside the cooldown");
        assert_eq!(s.dish.params.foe_repel, 1.0);
        step(&mut s, 4);
        assert_eq!(s.theirs, Habit::Hunt, "it answered before its delay");
        step(&mut s, 10);
        assert_eq!(s.theirs, Habit::Maze);
        assert!(s.switch(Habit::Hunt));
        assert_eq!(s.dish.params.foe_repel, -1.0);
    }
}
