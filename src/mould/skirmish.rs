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
use super::{Dish, Params, Phase, INOCULUM, STEPS_PER_SECOND};

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

/// The two homes: yours (gold, left) and the rival's (violet, right).
pub const HOMES: [(f32, f32); 2] = [(40.0, 80.0), (216.0, 80.0)];

/// **The tithe and the bank.** A side sets what fraction of every bite its
/// mould takes goes to its bank instead (`Dish::tithe`): one of these. The
/// bank is food really taken from the mould, so money is walkers not grown.
pub const TITHES: [f32; 4] = [0.0, 0.1, 0.25, 0.4];
/// A beacon: what it costs, how long it burns, how many of a side's can
/// burn at once (a fourth replaces the oldest), and how strongly the pile
/// then smells to that side's mould alone, in full piles.
pub const BEACON_COST: f32 = 300.0;
pub const BEACON_SECONDS: u64 = 20;
pub const BEACONS_MAX: usize = 3;
pub const BEACON_GAIN: f32 = 3.0;
/// A spore: what it costs and the colony it plants on a pile. Each walker
/// carries what an inoculum walker starts with (`START_STORE / INOCULUM`),
/// so the colony itself is worth 150 x (1 + 3.75) = 712 food (a birth costs
/// 1): the rest is the price of putting it where you want.
pub const SPORE_COST: f32 = 1_000.0;
pub const SPORE_WALKERS: usize = 150;
pub const SPORE_FOOD: f32 = super::START_STORE / INOCULUM as f32;
/// A habit switch: its price, and seconds before the next.
pub const HABIT_COST: f32 = 200.0;
pub const SWITCH_COOLDOWN: u64 = 10;
/// **Late food**: from 0:20, every 15 s, either one pile on the centre line
/// or a mirrored pair, somewhere new each dish (`late_food`). Without it the
/// moulds hold every pile by 0:10 and there is nothing to expand into
/// (`Reports/rts-skirmish-2026-11-10.md`, *The first budget round robin*).
pub const LATE_FIRST: u64 = 20;
pub const LATE_EVERY: u64 = 15;
pub const LATE_COUNT: u64 = 7;
pub const LATE_FOOD: f32 = 12_000.0;

pub static FIELD: Level = Level {
    key: "field",
    name: "THE FIELD",
    goal: [
        "EAT MORE OF THE FOOD THAN THE RIVAL.",
        "SPEND WHAT YOU EAT: BEACONS, SPORES, HABITS.",
    ],
    twist: "FOOD YOU EAT FILLS YOUR BANK.",
    piles: &PILES,
    salt: &[],
    tools: Tools {
        oats: 0,
        oats_regrow: 0,
        salt: 0,
        snips: 0,
        water: 0,
        water_regrow: 0,
    },
    seconds: 150,
    sensor_deg: 22.5,
    rival: Some(22),
    // Provisional: a share above half means you out-ate it.
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

/// What a side can buy. Piles are indices into `Dish::piles`.
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum Verb {
    Beacon(usize),
    Spore(usize),
    Habit(Habit),
}

impl Verb {
    pub fn price(self) -> f32 {
        match self {
            Verb::Beacon(_) => BEACON_COST,
            Verb::Spore(_) => SPORE_COST,
            Verb::Habit(_) => HABIT_COST,
        }
    }
}

/// **Scripted strategies**, for the rival and for the harness's
/// round robin. Each thinks once a second and sees what a player sees: the
/// piles, who is eating each, late food once it lands, its own bank and the
/// other side's habit. Each sets a tithe (`TITHES`) and buys with it.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Plan {
    /// No tithe, buys nothing: the thickest mould, and the control.
    Idle,
    /// Buys something at random about one second in four, if it can, and
    /// now and then sets a random tithe.
    Random,
    /// Expand (tithe 25%): races for the late food, nearest home first,
    /// with a beacon then spores on each late pile it does not yet hold;
    /// never attacks the rival's first piles.
    Boom,
    /// Attack (tithe 40%): hunt, then beacons and spores on the rival's
    /// piles nearest home.
    Raid,
    /// Pick off (25%): beacons and spores on the rival's pile with the
    /// fewest of its walkers eating, without changing habit.
    Soft,
    /// Take the middle (25%): the piles within 40 cells of the centre line
    /// it does not hold, nearest the line first; then expand.
    Centre,
    /// Defend: a 10% tithe saved for when the rival eats at one of its
    /// piles, then 25% and reinforce that pile, most rival eaters first;
    /// otherwise light the nearest free pile.
    Fort,
    /// Save at 40% until 1:20, then drop to 10% and raid with everything.
    Hoard,
    /// Habits only (10%): answer the rival's habit with the one that beats
    /// it.
    Counter,
}

impl Plan {
    pub const ALL: [Plan; 9] = [
        Plan::Idle,
        Plan::Random,
        Plan::Boom,
        Plan::Raid,
        Plan::Soft,
        Plan::Centre,
        Plan::Fort,
        Plan::Hoard,
        Plan::Counter,
    ];

    pub fn name(self) -> &'static str {
        match self {
            Plan::Idle => "idle",
            Plan::Random => "random",
            Plan::Boom => "boom",
            Plan::Raid => "raid",
            Plan::Soft => "soft",
            Plan::Centre => "centre",
            Plan::Fort => "fort",
            Plan::Hoard => "hoard",
            Plan::Counter => "counter",
        }
    }

    pub fn parse(s: &str) -> Option<Plan> {
        Plan::ALL.into_iter().find(|p| p.name() == s)
    }
}

/// What a side bought over a game.
#[derive(Clone, Debug, Default)]
pub struct Tally {
    pub banked: f32,
    pub spent: f32,
    pub beacons: u32,
    pub spores: u32,
    pub switches: u32,
}

/// One side of the skirmish: 0 is you (the dish's own mould), 1 the rival
/// (`Dish::rival`).
#[derive(Clone, Debug)]
pub struct Side {
    pub habit: Habit,
    /// The tithe this side has set, one of `TITHES`.
    pub tithe: f32,
    /// The step this side's next habit switch is allowed.
    pub ready_at: u64,
    pub bank: f32,
    /// Lit beacons: pile index and the step it goes out.
    pub beacons: Vec<(usize, u64)>,
    /// `None`: a person plays this side.
    pub plan: Option<Plan>,
    pub tally: Tally,
    tithed_seen: f32,
    rng: u64,
}

pub struct Skirmish {
    pub dish: Dish,
    pub sides: [Side; 2],
}

impl Skirmish {
    /// A skirmish on dish `seed`; `plans` says who plays each side (`None`
    /// a person).
    pub fn new(seed: u64, plans: [Option<Plan>; 2]) -> Skirmish {
        let mut dish = Dish::from_level_with(&FIELD, seed, FIELD.params_for());
        // An even start: the level gives a wild rival 600 walkers and a
        // bolder repel (The Wild One's); here it is your equal.
        dish.add_rival_of(HOMES[1], INOCULUM);
        let p = dish.params.clone();
        if let Some(r) = dish.rival.as_mut() {
            r.params = p;
        }
        late_food(&mut dish, seed);
        let side = |k: u64, plan| Side {
            habit: Habit::Spread,
            tithe: 0.0,
            ready_at: 0,
            bank: 0.0,
            beacons: Vec::new(),
            plan,
            tally: Tally::default(),
            tithed_seen: 0.0,
            rng: (seed ^ (0x9E37_79B9_7F4A_7C15u64.wrapping_mul(k + 1))) | 1,
        };
        Skirmish {
            dish,
            sides: [side(0, plans[0]), side(1, plans[1])],
        }
    }

    fn mould(&self, s: usize) -> &Dish {
        if s == 0 {
            &self.dish
        } else {
            self.dish.rival.as_ref().expect("the skirmish has a rival")
        }
    }

    /// Side `s` sets its tithe (any fraction; the page offers `TITHES`).
    pub fn set_tithe(&mut self, s: usize, tithe: f32) {
        let tithe = tithe.clamp(0.0, 0.9);
        self.sides[s].tithe = tithe;
        self.mould_mut(s).tithe = tithe;
    }

    fn mould_mut(&mut self, s: usize) -> &mut Dish {
        if s == 0 {
            &mut self.dish
        } else {
            self.dish.rival.as_mut().expect("the skirmish has a rival")
        }
    }

    /// Food side `s` has eaten.
    pub fn ate(&self, s: usize) -> f32 {
        if s == 0 {
            self.dish.ate
        } else {
            self.dish.wild_ate
        }
    }

    /// Side `s`'s walkers.
    pub fn walkers(&self, s: usize) -> usize {
        if s == 0 {
            self.dish.walkers.len()
        } else {
            self.dish.wild_left
        }
    }

    /// Side `s`'s walkers eating at pile `i`.
    pub fn eaters(&self, s: usize, i: usize) -> u32 {
        let p = &self.dish.piles[i];
        if s == 0 {
            p.eaters
        } else {
            p.foe_eaters
        }
    }

    /// Your share of the food eaten so far (half before anyone eats).
    pub fn share(&self) -> f32 {
        let all = self.dish.ate + self.dish.wild_ate;
        if all < 1.0 {
            0.5
        } else {
            self.dish.ate / all
        }
    }

    /// Seconds until side `s` may switch habit (0: now).
    pub fn cooldown_s(&self, s: usize) -> f32 {
        self.sides[s].ready_at.saturating_sub(self.dish.t) as f32 / SPS as f32
    }

    /// Whether side `s` could buy `v` now.
    pub fn can(&self, s: usize, v: Verb) -> bool {
        if self.dish.phase != Phase::Growing || self.sides[s].bank < v.price() {
            return false;
        }
        match v {
            Verb::Beacon(i) | Verb::Spore(i) => self.dish.piles.get(i).is_some_and(|p| p.food > 0.0 && !p.oats),
            Verb::Habit(h) => h != self.sides[s].habit && self.dish.t >= self.sides[s].ready_at,
        }
    }

    /// Side `s` buys `v`; false (and nothing paid) when it cannot.
    pub fn act(&mut self, s: usize, v: Verb) -> bool {
        if !self.can(s, v) {
            return false;
        }
        let t = self.dish.t;
        match v {
            Verb::Beacon(i) => {
                let side = &mut self.sides[s];
                side.beacons.retain(|b| b.0 != i);
                if side.beacons.len() >= BEACONS_MAX {
                    side.beacons.remove(0);
                }
                side.beacons.push((i, t + BEACON_SECONDS * SPS));
                side.tally.beacons += 1;
                self.light(s);
            }
            Verb::Spore(i) => {
                let at = (self.dish.piles[i].x, self.dish.piles[i].y);
                let placed = if s == 0 {
                    let r = self.dish.rival.take();
                    let others = r.as_ref().map_or(&[][..], |r| &r.walkers[..]);
                    let n = self.dish.plant(at, SPORE_WALKERS, SPORE_FOOD, others);
                    self.dish.rival = r;
                    n
                } else {
                    let n = self.dish.plant_rival(at, SPORE_WALKERS, SPORE_FOOD);
                    self.dish.wild_left = self.dish.rival.as_ref().map_or(0, |r| r.walkers.len());
                    n
                };
                if placed == 0 {
                    return false;
                }
                self.sides[s].tally.spores += 1;
            }
            Verb::Habit(h) => {
                h.apply(&mut self.mould_mut(s).params);
                let side = &mut self.sides[s];
                side.habit = h;
                side.ready_at = t + SWITCH_COOLDOWN * SPS;
                side.tally.switches += 1;
            }
        }
        let side = &mut self.sides[s];
        side.bank -= v.price();
        side.tally.spent += v.price();
        true
    }

    /// Side `s`'s lit beacons onto its mould.
    fn light(&mut self, s: usize) {
        let lit: Vec<(f32, f32, f32)> = self.sides[s]
            .beacons
            .iter()
            .map(|&(i, _)| (self.dish.piles[i].x, self.dish.piles[i].y, BEACON_GAIN))
            .collect();
        self.mould_mut(s).beacons = lit;
    }

    /// Once a frame, before the dish steps: banks fill, beacons go out, and
    /// each scripted side thinks once a second (the rival half a second
    /// after you).
    pub fn tick(&mut self) {
        if self.dish.phase != Phase::Growing {
            return;
        }
        let t = self.dish.t;
        for s in 0..2 {
            let tithed = self.mould(s).tithed;
            let side = &mut self.sides[s];
            let new = (tithed - side.tithed_seen).max(0.0);
            side.tithed_seen = tithed;
            side.bank += new;
            side.tally.banked += new;
            let before = side.beacons.len();
            side.beacons.retain(|b| b.1 > t);
            if side.beacons.len() != before {
                self.light(s);
            }
        }
        for s in 0..2 {
            if let Some(plan) = self.sides[s].plan {
                if t % SPS == (s as u64) * (SPS / 2) {
                    self.think(s, plan);
                }
            }
        }
    }

    /// A scripted side's move, once a second: its tithe, then what it buys.
    fn think(&mut self, s: usize, plan: Plan) {
        let o = 1 - s;
        let t = self.dish.t;
        let home = HOMES[s];
        let n = self.dish.piles.len();
        let live = |sk: &Skirmish, i: usize| sk.dish.piles[i].food > 0.0;
        let dist = |sk: &Skirmish, i: usize| {
            let p = &sk.dish.piles[i];
            (p.x - home.0).hypot(p.y - home.1)
        };
        let nearest = |sk: &Skirmish, f: &dyn Fn(&Skirmish, usize) -> bool| {
            (0..n)
                .filter(|&i| live(sk, i) && f(sk, i))
                .min_by(|&a, &b| dist(sk, a).total_cmp(&dist(sk, b)))
        };
        let lit = |sk: &Skirmish, i: usize| sk.sides[s].beacons.iter().any(|b| b.0 == i);
        let free = |sk: &Skirmish, i: usize| sk.eaters(s, i) == 0 && sk.eaters(o, i) == 0;
        let theirs = |sk: &Skirmish, i: usize| sk.eaters(o, i) > sk.eaters(s, i);
        let bank = self.sides[s].bank;
        let tithe = match plan {
            Plan::Idle => 0.0,
            Plan::Random => self.sides[s].tithe,
            Plan::Counter | Plan::Fort => TITHES[1],
            Plan::Boom | Plan::Soft | Plan::Centre => TITHES[2],
            Plan::Raid => TITHES[3],
            Plan::Hoard => {
                if t < 80 * SPS {
                    TITHES[3]
                } else {
                    TITHES[1]
                }
            }
        };
        self.set_tithe(s, tithe);
        match plan {
            Plan::Idle => {}
            Plan::Random => {
                if self.roll(s) < 0.25 {
                    let i = (self.roll(s) * n as f32) as usize % n;
                    let k = self.roll(s);
                    let v = if k < 0.45 {
                        Verb::Beacon(i)
                    } else if k < 0.8 {
                        Verb::Spore(i)
                    } else {
                        Verb::Habit(Habit::ALL[(self.roll(s) * 4.0) as usize % 4])
                    };
                    self.act(s, v);
                }
                if self.roll(s) < 0.05 {
                    let k = (self.roll(s) * TITHES.len() as f32) as usize % TITHES.len();
                    self.set_tithe(s, TITHES[k]);
                }
            }
            Plan::Boom => {
                let first = PILES.len();
                if let Some(i) = nearest(self, &|sk: &Skirmish, i: usize| i >= first && sk.eaters(s, i) <= sk.eaters(o, i)) {
                    self.strike(s, i, &lit);
                } else {
                    self.expand(s, &nearest, &lit, &free, bank);
                }
            }
            Plan::Raid | Plan::Hoard => {
                if plan == Plan::Hoard && t < 80 * SPS {
                    return;
                }
                if plan == Plan::Raid && self.sides[s].habit != Habit::Hunt {
                    self.act(s, Verb::Habit(Habit::Hunt));
                    return;
                }
                if let Some(i) = nearest(self, &theirs) {
                    self.strike(s, i, &lit);
                }
            }
            Plan::Soft => {
                let target = (0..n)
                    .filter(|&i| live(self, i) && theirs(self, i))
                    .min_by_key(|&i| self.eaters(o, i));
                if let Some(i) = target {
                    self.strike(s, i, &lit);
                }
            }
            Plan::Centre => {
                let band = |sk: &Skirmish, i: usize| {
                    let p = &sk.dish.piles[i];
                    (p.x - 128.0).abs() <= 40.0 && sk.eaters(s, i) <= sk.eaters(o, i)
                };
                let target = (0..n)
                    .filter(|&i| live(self, i) && band(self, i))
                    .min_by(|&a, &b| {
                        let d = |i: usize| (self.dish.piles[i].x - 128.0).abs();
                        d(a).total_cmp(&d(b))
                    });
                match target {
                    Some(i) => self.strike(s, i, &lit),
                    None => self.expand(s, &nearest, &lit, &free, bank),
                }
            }
            Plan::Fort => {
                // Its own piles the rival is eating at, most rival eaters
                // first; a tithe only while there is one to defend.
                let contested = (0..n)
                    .filter(|&i| live(self, i) && self.eaters(s, i) > 0 && self.eaters(o, i) > 0)
                    .max_by_key(|&i| self.eaters(o, i));
                match contested {
                    Some(i) => {
                        self.set_tithe(s, TITHES[2]);
                        self.strike(s, i, &lit);
                    }
                    None => {
                        if let Some(i) = nearest(self, &free) {
                            if !lit(self, i) {
                                self.act(s, Verb::Beacon(i));
                            }
                        }
                    }
                }
            }
            Plan::Counter => {
                let want = self.sides[o].habit.counter();
                self.act(s, Verb::Habit(want));
            }
        }
    }

    /// Light pile `i` if it is not lit, else plant a spore on it.
    fn strike(&mut self, s: usize, i: usize, lit: &dyn Fn(&Skirmish, usize) -> bool) {
        if !lit(self, i) && self.act(s, Verb::Beacon(i)) {
            return;
        }
        self.act(s, Verb::Spore(i));
    }

    /// Expansion, for Boom and Centre once the middle is held: a spore on
    /// the free pile nearest home, else (with a spore's worth to spare) a
    /// beacon on the nearest free pile not yet lit.
    fn expand(
        &mut self,
        s: usize,
        nearest: &dyn Fn(&Skirmish, &dyn Fn(&Skirmish, usize) -> bool) -> Option<usize>,
        lit: &dyn Fn(&Skirmish, usize) -> bool,
        free: &dyn Fn(&Skirmish, usize) -> bool,
        bank: f32,
    ) {
        if let Some(i) = nearest(self, free) {
            if self.act(s, Verb::Spore(i)) {
                return;
            }
        }
        if bank >= SPORE_COST + BEACON_COST {
            if let Some(i) = nearest(self, &|sk: &Skirmish, i: usize| free(sk, i) && !lit(sk, i)) {
                self.act(s, Verb::Beacon(i));
            }
        }
    }

    /// A number in [0, 1) from side `s`'s own stream (xorshift), so the
    /// random plan never draws from the dish's.
    fn roll(&mut self, s: usize) -> f32 {
        let r = &mut self.sides[s].rng;
        *r ^= *r << 13;
        *r ^= *r >> 7;
        *r ^= *r << 17;
        (*r >> 40) as f32 / (1u64 << 24) as f32
    }
}

/// Late food for dish `seed` (`LATE_FIRST`...): even turns one pile on the
/// centre line, odd turns a mirrored pair, each on open agar at least 20
/// cells from every other pile, early or late, and 30 from either home.
/// Seeded apart from the dish's own random stream.
fn late_food(dish: &mut Dish, seed: u64) {
    let mut r = seed.wrapping_mul(0xD1B5_4A32_D192_ED03) | 1;
    let mut roll = move || {
        r ^= r << 13;
        r ^= r >> 7;
        r ^= r << 17;
        (r >> 40) as f32 / (1u64 << 24) as f32
    };
    let mut taken: Vec<(f32, f32)> = dish.piles.iter().map(|p| (p.x, p.y)).collect();
    let template = dish.piles[1].clone();
    for k in 0..LATE_COUNT {
        let at = (LATE_FIRST + k * LATE_EVERY) * SPS;
        let pair = k % 2 == 1;
        for _ in 0..500 {
            let (x, y) = if pair {
                (16.0 + roll() * 96.0, 14.0 + roll() * 132.0)
            } else {
                (128.0, 14.0 + roll() * 132.0)
            };
            let spots: Vec<(f32, f32)> = if pair { vec![(x, y), (256.0 - x, y)] } else { vec![(x, y)] };
            let clear = spots.iter().all(|&(x, y)| {
                let agar = super::cell(x, y).is_some_and(|i| dish.ground[i] == super::Ground::Agar)
                    && super::cell(x, y - 5.0).is_some_and(|i| dish.ground[i] == super::Ground::Agar)
                    && super::cell(x, y + 5.0).is_some_and(|i| dish.ground[i] == super::Ground::Agar);
                agar && taken.iter().all(|q| (q.0 - x).hypot(q.1 - y) >= 20.0)
                    && HOMES.iter().all(|h| (h.0 - x).hypot(h.1 - y) >= 30.0)
            });
            if clear {
                for &(x, y) in &spots {
                    taken.push((x, y));
                    let p = super::Pile { x, y, food: LATE_FOOD, full: LATE_FOOD, ..template.clone() };
                    dish.pending.push((at, p));
                }
                break;
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
    let (mine, theirs) = if actor_self { (HOMES[0], HOMES[1]) } else { (HOMES[1], HOMES[0]) };
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

    fn run(s: &mut Skirmish, secs: u64) {
        let end = s.dish.t + secs * SPS;
        while s.dish.t < end {
            s.tick();
            s.dish.step();
        }
    }

    #[test]
    fn the_field_is_mirrored_and_even() {
        let s = Skirmish::new(1, [None, None]);
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
    fn eating_fills_the_bank_and_buying_spends_it() {
        let mut s = Skirmish::new(2, [None, None]);
        assert!(!s.act(0, Verb::Spore(20)), "bought with an empty bank");
        run(&mut s, 30);
        let bank = s.sides[0].bank;
        assert!(bank == 0.0, "a bank filled with no tithe");
        s.set_tithe(0, 0.25);
        run(&mut s, 10);
        let bank = s.sides[0].bank;
        assert!(bank > 0.0 && (bank - s.dish.tithed).abs() < 0.01 * bank, "bank {bank}, tithed {}", s.dish.tithed);
        s.sides[0].bank = SPORE_COST + BEACON_COST + HABIT_COST;
        s.sides[1].bank = SPORE_COST;
        let (mine, theirs) = (s.walkers(0), s.walkers(1));
        assert!(s.act(0, Verb::Spore(20)));
        assert!(s.act(1, Verb::Spore(20)));
        // A crowded pile takes fewer than the whole colony.
        assert!(s.walkers(0) > mine + SPORE_WALKERS / 2 && s.walkers(0) <= mine + SPORE_WALKERS);
        assert!(s.walkers(1) > theirs && s.walkers(1) <= theirs + SPORE_WALKERS);
        assert!(s.act(0, Verb::Beacon(19)));
        assert_eq!(s.dish.beacons.len(), 1);
        assert!(s.dish.rival.as_ref().unwrap().beacons.is_empty(), "a beacon lit for both");
        assert!(s.act(0, Verb::Habit(Habit::Cord)));
        assert!(s.sides[0].bank < 1.0);
        s.sides[0].bank = HABIT_COST;
        assert!(!s.act(0, Verb::Habit(Habit::Maze)), "a switch inside the cooldown");
        run(&mut s, BEACON_SECONDS + 1);
        assert!(s.dish.beacons.is_empty(), "the beacon never went out");
    }

    #[test]
    fn a_beacon_pulls_only_its_own_mould() {
        // The far centre pile, lit for you from the start: your scent there
        // is the beacon's, the rival's the pile's own.
        let mut s = Skirmish::new(3, [None, None]);
        s.sides[0].bank = BEACON_COST;
        assert!(s.act(0, Verb::Beacon(19)));
        s.dish.step();
        let p = &s.dish.piles[19];
        let i = p.y as usize * super::super::W + p.x as usize;
        let mine = s.dish.scent[i];
        let theirs = s.dish.rival.as_ref().unwrap().scent[i];
        assert!(mine > 2.0 * theirs, "beacon scent {mine} against {theirs}");
    }
}
