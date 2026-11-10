//! The field skirmish: the RTS thread's playable demo (owner, 2026-11-10
//! 20:33Z: "lets focus on actually building playable demos").
//!
//! You and a rival mould share a field of 23 small food piles, mirrored so
//! neither side has the better map, with more food landing from 0:20;
//! whoever eats more of the food by the bell wins
//! (`Reports/rts-skirmish-2026-11-10.md`).
//!
//! **Version 2, the budget skirmish** (this file). Each side sets a tithe:
//! a fraction of every bite goes to its bank instead of its mould
//! (`Dish::tithe`), so money is walkers not grown. It spends the bank on
//! beacons (a pile your mould alone craves, `Dish::beacons`), spores (a
//! colony planted on a pile, `Dish::plant`) and habits. Nine scripted
//! plans (`Plan`) play either side. In a 576-game round robin one of them,
//! soft (beacons and spores on the rival's thinnest pile), won 127 of 128
//! games and the rest stood on a ladder with no cycle: the strategy is
//! shallow as built.
//!
//! **Version 1, set aside.** Habits on keys 1-4 (on this layout hunt beat
//! maze 32/32, maze cord 30/32, cord hunt 25/32), two salt walls and the
//! lamp, against a rival that answered your habit after a delay. The rival
//! beat an idle hand 16/16, and a hand pressing the counter at once beat it
//! 14/16. The owner played it (21:14Z): "user input seems limited and
//! doesn't lead to any complex stratgy". Its code is at 6c28e01.
//!
//! **Tried in version 2 and dropped:** a bank paid a tenth of all food
//! eaten without taking it from the mould. Money for nothing made every
//! purchase pure gain (random spending beat idle 4/4), so the tithe
//! replaced it.

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
/// A rally: price, seconds and gain (`Verb::Rally`).
pub const RALLY_COST: f32 = 150.0;
pub const RALLY_SECONDS: u64 = 5;
pub const RALLY_GAIN: f32 = 10.0;
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

/// The prices and reaches above, as one value a skirmish carries, so the
/// harness can try others (`Rules::set`). `Rules::default()` is the game.
#[derive(Clone, Debug, PartialEq)]
pub struct Rules {
    pub beacon_cost: f32,
    pub beacon_seconds: u64,
    pub beacon_gain: f32,
    pub spore_cost: f32,
    pub spore_walkers: usize,
    pub habit_cost: f32,
    /// How far from its own fed tube a side may plant a spore, in cells
    /// from the pile's centre (infinite: anywhere).
    pub spore_reach: f32,
    /// Seconds a spore lies dormant before it wakes (0: it lands awake).
    /// Dormant walkers are the model's sleepers (`Walker::slept`): they do
    /// not move, eat or pay upkeep, they hold their cells, and in a fight
    /// they count as unfed, so a spore dropped among the rival's fed
    /// walkers is eaten into before it wakes.
    pub spore_dormant: u64,
    /// Dormancy only for a spore landing on a pile the other side holds
    /// (more of its walkers eating there): one planted at home wakes at
    /// once.
    pub dormant_abroad_only: bool,
    /// Each spore costs this much more, as a fraction of `spore_cost`, for
    /// every spore the same side bought in the last 30 s (0: a flat price).
    pub spore_ramp: f32,
    /// A rally: its price, seconds and gain (a beacon's, but stronger and
    /// shorter, and only on a pile the side is eating at).
    pub rally_cost: f32,
    pub rally_seconds: u64,
    pub rally_gain: f32,
    /// The two moulds take turns moving first (`Dish::take_turns`). Off,
    /// yours always moves first, and won 14 of 16 mirror games; rounds 1-3
    /// ran with it off.
    pub take_turns: bool,
}

impl Default for Rules {
    fn default() -> Rules {
        Rules {
            beacon_cost: BEACON_COST,
            beacon_seconds: BEACON_SECONDS,
            beacon_gain: BEACON_GAIN,
            spore_cost: SPORE_COST,
            spore_walkers: SPORE_WALKERS,
            habit_cost: HABIT_COST,
            spore_reach: f32::INFINITY,
            spore_dormant: 0,
            dormant_abroad_only: false,
            spore_ramp: 0.0,
            rally_cost: RALLY_COST,
            rally_seconds: RALLY_SECONDS,
            rally_gain: RALLY_GAIN,
            take_turns: true,
        }
    }
}

impl Rules {
    /// Change one rule by name ("spore_cost", ...); false for a name it
    /// does not know or a value it cannot read.
    pub fn set(&mut self, name: &str, value: &str) -> bool {
        let Ok(v) = value.parse::<f32>() else { return false };
        match name {
            "beacon_cost" => self.beacon_cost = v,
            "beacon_seconds" => self.beacon_seconds = v as u64,
            "beacon_gain" => self.beacon_gain = v,
            "spore_cost" => self.spore_cost = v,
            "spore_walkers" => self.spore_walkers = v as usize,
            "habit_cost" => self.habit_cost = v,
            "spore_reach" => self.spore_reach = v,
            "spore_dormant" => self.spore_dormant = v as u64,
            "dormant_abroad_only" => self.dormant_abroad_only = v != 0.0,
            "spore_ramp" => self.spore_ramp = v,
            "rally_cost" => self.rally_cost = v,
            "rally_seconds" => self.rally_seconds = v as u64,
            "rally_gain" => self.rally_gain = v,
            "take_turns" => self.take_turns = v != 0.0,
            _ => return false,
        }
        true
    }
}

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
    /// A short, strong beacon on a pile you are eating at: defence.
    Rally(usize),
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
    /// Defend: a 10% tithe saved for when the rival lands a spore on, or
    /// eats at, one of its piles, then 25% and reinforce that pile (a
    /// landing spore first, then most rival eaters); otherwise light the
    /// nearest free pile.
    Fort,
    /// Save at 40% until 1:20, then drop to 10% and raid with everything.
    Hoard,
    /// Habits only (10%): answer the rival's habit with the one that beats
    /// it.
    Counter,
    /// Pick off on the cheap (10%): soft's target, beacons only.
    Lean,
    /// Guard (10%): beacons on its own border piles (a rival pile within 50
    /// cells) with the fewest of its walkers, before anyone attacks; a spore
    /// on any of its piles a rival spore lands on or the rival eats at.
    Guard,
    /// Answer in kind (25%): saves until the rival attacks one of its piles
    /// (a rival spore landing there, or a rival beacon or rally lit where it
    /// eats), then strikes the rival's thinnest pile, as soft does, for 10 s.
    Trade,
    /// Soft with a shield (25%): a rally on any of its piles a rival spore
    /// is landing on, before anything else; otherwise soft.
    Keep,
    /// Shield only (10%): a rally on any of its piles a rival spore is
    /// landing on or the rival has lit; nothing else.
    Ward,
}

impl Plan {
    pub const ALL: [Plan; 14] = [
        Plan::Idle,
        Plan::Random,
        Plan::Boom,
        Plan::Raid,
        Plan::Soft,
        Plan::Centre,
        Plan::Fort,
        Plan::Hoard,
        Plan::Counter,
        Plan::Lean,
        Plan::Guard,
        Plan::Trade,
        Plan::Keep,
        Plan::Ward,
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
            Plan::Lean => "lean",
            Plan::Guard => "guard",
            Plan::Trade => "trade",
            Plan::Keep => "keep",
            Plan::Ward => "ward",
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
    pub rallies: u32,
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
    /// Rallies, the same way.
    pub rallies: Vec<(usize, u64)>,
    /// Trade answers an attack until this step.
    pub answer_until: u64,
    /// The steps this side bought its spores in the last 30 s
    /// (`Rules::spore_ramp`).
    pub spore_times: Vec<u64>,
    /// `None`: a person plays this side.
    pub plan: Option<Plan>,
    pub tally: Tally,
    tithed_seen: f32,
    rng: u64,
}

/// A spore that has landed and not yet woken: whose, on which pile, the
/// step it landed (its walkers' `slept`) and the step it wakes.
#[derive(Clone, Copy, Debug)]
pub struct Spore {
    pub side: usize,
    pub pile: usize,
    pub landed: u64,
    pub wakes: u64,
}

/// A plan's question about one pile, and its search for the nearest pile
/// that passes one.
type PileTest<'a> = dyn Fn(&Skirmish, usize) -> bool + 'a;
type Nearest<'a> = dyn Fn(&Skirmish, &PileTest<'_>) -> Option<usize> + 'a;

pub struct Skirmish {
    pub dish: Dish,
    pub sides: [Side; 2],
    pub rules: Rules,
    /// Spores still dormant (`Rules::spore_dormant`).
    pub spores: Vec<Spore>,
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
            rallies: Vec::new(),
            answer_until: 0,
            spore_times: Vec::new(),
            plan,
            tally: Tally::default(),
            tithed_seen: 0.0,
            rng: (seed ^ (0x9E37_79B9_7F4A_7C15u64.wrapping_mul(k + 1))) | 1,
        };
        Skirmish {
            dish,
            sides: [side(0, plans[0]), side(1, plans[1])],
            rules: Rules::default(),
            spores: Vec::new(),
        }
    }

    fn mould(&self, s: usize) -> &Dish {
        if s == 0 {
            &self.dish
        } else {
            self.dish.rival.as_ref().expect("the skirmish has a rival")
        }
    }

    /// Set a rule (`Rules::set`) or one of the model's numbers on both
    /// moulds at once: `fight`, `fight_fed`, `fight_unfed`, `upkeep`,
    /// `upkeep_size`, `hold`, `hold_flip`. False if `name` is neither or
    /// `value` is not a number.
    pub fn set(&mut self, name: &str, value: &str) -> bool {
        if self.rules.set(name, value) {
            return true;
        }
        let Ok(v) = value.parse::<f32>() else { return false };
        for s in 0..2 {
            let p = &mut self.mould_mut(s).params;
            match name {
                "fight" => p.fight = v,
                "fight_fed" => p.fight_fed = v,
                "fight_unfed" => p.fight_unfed = v,
                "upkeep" => p.upkeep = v,
                "upkeep_size" => p.upkeep_size = v,
                "hold" => p.hold = v,
                "hold_flip" => p.hold_flip = v,
                _ => return false,
            }
        }
        true
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

    /// What `v` costs side `s` now.
    pub fn price(&self, s: usize, v: Verb) -> f32 {
        match v {
            Verb::Beacon(_) => self.rules.beacon_cost,
            Verb::Spore(_) => {
                let recent = self.sides[s].spore_times.iter().filter(|&&t| t + 30 * SPS > self.dish.t).count();
                self.rules.spore_cost * (1.0 + self.rules.spore_ramp * recent as f32)
            }
            Verb::Habit(_) => self.rules.habit_cost,
            Verb::Rally(_) => self.rules.rally_cost,
        }
    }

    /// Whether side `s`'s fed tube comes within `reach` cells of (x, y).
    pub fn within_reach(&self, s: usize, (x, y): (f32, f32), reach: f32) -> bool {
        if !reach.is_finite() {
            return true;
        }
        let fed = &self.mould(s).fed;
        let r = reach.ceil() as i32;
        let (cx, cy) = (x as i32, y as i32);
        for dy in -r..=r {
            for dx in -r..=r {
                if dx * dx + dy * dy > r * r {
                    continue;
                }
                let (px, py) = (cx + dx, cy + dy);
                if px >= 0 && py >= 0 && (px as usize) < super::W && (py as usize) < super::H && fed[py as usize * super::W + px as usize] != u16::MAX {
                    return true;
                }
            }
        }
        false
    }

    /// Whether side `s` could buy `v` now.
    pub fn can(&self, s: usize, v: Verb) -> bool {
        if self.dish.phase != Phase::Growing || self.sides[s].bank < self.price(s, v) {
            return false;
        }
        match v {
            Verb::Beacon(i) => self.dish.piles.get(i).is_some_and(|p| p.food > 0.0 && !p.oats),
            Verb::Spore(i) => self
                .dish
                .piles
                .get(i)
                .is_some_and(|p| p.food > 0.0 && !p.oats && self.within_reach(s, (p.x, p.y), self.rules.spore_reach)),
            Verb::Habit(h) => h != self.sides[s].habit && self.dish.t >= self.sides[s].ready_at,
            Verb::Rally(i) => self.dish.piles.get(i).is_some_and(|p| p.food > 0.0 && !p.oats) && self.eaters(s, i) > 0,
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
                side.beacons.push((i, t + self.rules.beacon_seconds * SPS));
                side.tally.beacons += 1;
                self.light(s);
            }
            Verb::Rally(i) => {
                let side = &mut self.sides[s];
                side.rallies.retain(|b| b.0 != i);
                side.rallies.push((i, t + self.rules.rally_seconds * SPS));
                side.tally.rallies += 1;
                self.light(s);
            }
            Verb::Spore(i) => {
                let at = (self.dish.piles[i].x, self.dish.piles[i].y);
                let placed = if s == 0 {
                    let r = self.dish.rival.take();
                    let others = r.as_ref().map_or(&[][..], |r| &r.walkers[..]);
                    let n = self.dish.plant(at, self.rules.spore_walkers, SPORE_FOOD, others);
                    self.dish.rival = r;
                    n
                } else {
                    let n = self.dish.plant_rival(at, self.rules.spore_walkers, SPORE_FOOD);
                    self.dish.wild_left = self.dish.rival.as_ref().map_or(0, |r| r.walkers.len());
                    n
                };
                if placed == 0 {
                    return false;
                }
                let abroad = self.eaters(1 - s, i) > self.eaters(s, i);
                if self.rules.spore_dormant > 0 && (abroad || !self.rules.dormant_abroad_only) {
                    // The new walkers are the last ones in; they sleep
                    // from now, tagged with this step.
                    let walkers = &mut self.mould_mut(s).walkers;
                    let k = walkers.len() - placed;
                    for w in &mut walkers[k..] {
                        w.slept = t;
                    }
                    self.spores.push(Spore { side: s, pile: i, landed: t, wakes: t + self.rules.spore_dormant * SPS });
                }
                self.sides[s].tally.spores += 1;
                self.sides[s].spore_times.push(t);
            }
            Verb::Habit(h) => {
                h.apply(&mut self.mould_mut(s).params);
                let side = &mut self.sides[s];
                side.habit = h;
                side.ready_at = t + SWITCH_COOLDOWN * SPS;
                side.tally.switches += 1;
            }
        }
        let price = self.price(s, v);
        let side = &mut self.sides[s];
        side.bank -= price;
        side.tally.spent += price;
        true
    }

    /// Side `s`'s lit beacons onto its mould.
    fn light(&mut self, s: usize) {
        let side = &self.sides[s];
        let piles = &self.dish.piles;
        // A rallied pile takes the rally's gain even if it is also lit.
        let lit: Vec<(f32, f32, f32)> = side
            .rallies
            .iter()
            .map(|&(i, _)| (piles[i].x, piles[i].y, self.rules.rally_gain))
            .chain(
                side.beacons
                    .iter()
                    .filter(|b| !side.rallies.iter().any(|r| r.0 == b.0))
                    .map(|&(i, _)| (piles[i].x, piles[i].y, self.rules.beacon_gain)),
            )
            .collect();
        self.mould_mut(s).beacons = lit;
    }

    /// Once a frame, before the dish steps: banks fill, beacons go out, and
    /// each scripted side thinks once a second (the rival half a second
    /// after you).
    pub fn tick(&mut self) {
        self.dish.take_turns = self.rules.take_turns;
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
            let before = side.beacons.len() + side.rallies.len();
            side.beacons.retain(|b| b.1 > t);
            side.rallies.retain(|b| b.1 > t);
            side.spore_times.retain(|&at| at + 30 * SPS > t);
            if side.beacons.len() + side.rallies.len() != before {
                self.light(s);
            }
        }
        // Spores due wake: every walker still asleep since the step one
        // landed.
        let due: Vec<Spore> = self.spores.iter().filter(|p| p.wakes <= t).copied().collect();
        if !due.is_empty() {
            self.spores.retain(|p| p.wakes > t);
            for p in due {
                for w in self.mould_mut(p.side).walkers.iter_mut() {
                    if w.slept == p.landed {
                        w.slept = u64::MAX;
                        w.woke = t;
                    }
                }
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
        let nearest = |sk: &Skirmish, f: &PileTest<'_>| {
            (0..n)
                .filter(|&i| live(sk, i) && f(sk, i))
                .min_by(|&a, &b| dist(sk, a).total_cmp(&dist(sk, b)))
        };
        let lit = |sk: &Skirmish, i: usize| sk.sides[s].beacons.iter().any(|b| b.0 == i);
        let free = |sk: &Skirmish, i: usize| sk.eaters(s, i) == 0 && sk.eaters(o, i) == 0;
        let theirs = |sk: &Skirmish, i: usize| sk.eaters(o, i) > sk.eaters(s, i);
        let thinnest = |sk: &Skirmish| {
            (0..n)
                .filter(|&i| live(sk, i) && theirs(sk, i))
                .min_by_key(|&i| sk.eaters(o, i))
        };
        let rallied = |sk: &Skirmish, i: usize| sk.sides[s].rallies.iter().any(|b| b.0 == i);
        // An own pile a rival spore is landing on (asleep there now).
        let landing = |sk: &Skirmish| sk.spores.iter().find(|p| p.side == o && sk.eaters(s, p.pile) > 0).map(|p| p.pile);
        // That, or an own pile the rival has a beacon or rally lit on.
        let attacked = |sk: &Skirmish| {
            landing(sk).or_else(|| {
                let r = &sk.sides[o];
                r.rallies.iter().chain(r.beacons.iter()).map(|b| b.0).find(|&i| sk.eaters(s, i) > sk.eaters(o, i))
            })
        };
        let bank = self.sides[s].bank;
        let tithe = match plan {
            Plan::Idle => 0.0,
            Plan::Random => self.sides[s].tithe,
            Plan::Counter | Plan::Fort | Plan::Lean | Plan::Guard => TITHES[1],
            Plan::Ward => TITHES[1],
            Plan::Boom | Plan::Soft | Plan::Centre | Plan::Trade | Plan::Keep => TITHES[2],
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
                    // A rally is drawn as often as a habit; `act` refuses it
                    // on a pile the side is not eating at.
                    let v = if k < 0.4 {
                        Verb::Beacon(i)
                    } else if k < 0.7 {
                        Verb::Spore(i)
                    } else if k < 0.85 {
                        Verb::Rally(i)
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
                if let Some(i) = thinnest(self) {
                    self.strike(s, i, &lit);
                }
            }
            Plan::Trade => {
                if attacked(self).is_some() {
                    self.sides[s].answer_until = t + 10 * SPS;
                }
                if t < self.sides[s].answer_until {
                    if let Some(i) = thinnest(self) {
                        self.strike(s, i, &lit);
                    }
                }
            }
            Plan::Keep | Plan::Ward => {
                let shield = if plan == Plan::Ward { attacked(self) } else { landing(self) };
                if let Some(i) = shield {
                    if !rallied(self, i) && self.act(s, Verb::Rally(i)) {
                        return;
                    }
                    // Saving for the rally beats spending on the attack.
                    if !rallied(self, i) {
                        return;
                    }
                }
                if plan == Plan::Keep {
                    if let Some(i) = thinnest(self) {
                        self.strike(s, i, &lit);
                    }
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
                let landing = self.spores.iter().find(|p| p.side == o && self.eaters(s, p.pile) > 0).map(|p| p.pile);
                let contested = landing.or_else(|| {
                    (0..n)
                        .filter(|&i| live(self, i) && self.eaters(s, i) > 0 && self.eaters(o, i) > 0)
                        .max_by_key(|&i| self.eaters(o, i))
                });
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
            Plan::Lean => {
                let target = (0..n)
                    .filter(|&i| live(self, i) && theirs(self, i) && !lit(self, i))
                    .min_by_key(|&i| self.eaters(o, i));
                if let Some(i) = target {
                    if self.sides[s].beacons.len() < BEACONS_MAX {
                        self.act(s, Verb::Beacon(i));
                    }
                }
            }
            Plan::Guard => {
                let landing = self.spores.iter().find(|p| p.side == o && self.eaters(s, p.pile) > 0).map(|p| p.pile);
                let contested = landing.or_else(|| (0..n).find(|&i| live(self, i) && self.eaters(s, i) > 0 && self.eaters(o, i) > 0));
                if let Some(i) = contested {
                    if self.act(s, Verb::Spore(i)) {
                        return;
                    }
                }
                let border = |sk: &Skirmish, i: usize| {
                    let p = &sk.dish.piles[i];
                    sk.eaters(s, i) > sk.eaters(o, i)
                        && (0..n).any(|j| {
                            let q = &sk.dish.piles[j];
                            live(sk, j) && theirs(sk, j) && (q.x - p.x).hypot(q.y - p.y) < 50.0
                        })
                };
                let target = (0..n)
                    .filter(|&i| live(self, i) && border(self, i) && !lit(self, i))
                    .min_by_key(|&i| self.eaters(s, i));
                if let Some(i) = target {
                    if self.sides[s].beacons.len() < BEACONS_MAX {
                        self.act(s, Verb::Beacon(i));
                    }
                }
            }
        }
    }

    /// Light pile `i` if it is not lit, else plant a spore on it.
    fn strike(&mut self, s: usize, i: usize, lit: &PileTest<'_>) {
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
        nearest: &Nearest<'_>,
        lit: &PileTest<'_>,
        free: &PileTest<'_>,
        bank: f32,
    ) {
        if let Some(i) = nearest(self, free) {
            if self.act(s, Verb::Spore(i)) {
                return;
            }
        }
        if bank >= self.rules.spore_cost + self.rules.beacon_cost {
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
        // A crowded pile takes fewer than the whole colony: the centre pile
        // at 0:40 on dish 2 took 60 of 150 once the moulds took turns.
        assert!(s.walkers(0) > mine + SPORE_WALKERS / 4 && s.walkers(0) <= mine + SPORE_WALKERS, "{} placed", s.walkers(0) - mine);
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

    #[test]
    fn the_holder_bites_harder_and_loses_the_pile_only_when_outnumbered() {
        let mut p = super::super::Pile {
            eaters: 10,
            foe_eaters: 10,
            ..Skirmish::new(1, [None, None]).dish.piles[1].clone()
        };
        let (table, eat) = (40, 0.03);
        assert_eq!(p.bite_held(table, eat, 2.0), p.bite(table, eat), "nobody holds it yet");
        p.held = 1;
        assert!((p.bite_held(table, eat, 2.0) - 2.0 * p.bite_held(table, eat, 1.0) * 2.0 / 3.0).abs() < 1e-6);
        p.held = -1;
        assert!((p.bite_held(table, eat, 2.0) - p.bite(table, eat) * 2.0 / 3.0).abs() < 1e-6);
        // In a skirmish the home piles are held by their own mould, and a
        // plain head count marks nothing.
        let mut s = Skirmish::new(2, [None, None]);
        assert!(s.set("hold", "2"));
        run(&mut s, 5);
        assert_eq!(s.dish.piles[0].held, 1);
        assert_eq!(s.dish.piles[22].held, -1);
        assert_eq!(s.dish.rival.as_ref().unwrap().piles[22].held, 1);
        let mut s = Skirmish::new(2, [None, None]);
        run(&mut s, 5);
        assert!(s.dish.piles.iter().all(|p| p.held == 0));
        assert!(!s.set("no_such_rule", "1"));
    }

    #[test]
    fn upkeep_that_grows_with_size_shrinks_a_lone_mould() {
        let grow = |k: &str| {
            let mut s = Skirmish::new(4, [None, None]);
            assert!(s.set("upkeep_size", k));
            run(&mut s, 30);
            s.walkers(0)
        };
        let (flat, steep) = (grow("0"), grow("0.3"));
        assert!(steep < flat, "{steep} walkers with a size cost, {flat} without");
    }

    #[test]
    fn a_rally_needs_your_own_eaters_and_spores_cost_more_in_a_burst() {
        let mut s = Skirmish::new(2, [None, None]);
        s.rules.spore_ramp = 0.5;
        run(&mut s, 30);
        let mine = (0..s.dish.piles.len()).find(|&i| s.eaters(0, i) > 0 && s.eaters(1, i) == 0).unwrap();
        let theirs = (0..s.dish.piles.len()).find(|&i| s.eaters(1, i) > 0 && s.eaters(0, i) == 0).unwrap();
        s.sides[0].bank = 2.0 * RALLY_COST;
        assert!(!s.act(0, Verb::Rally(theirs)), "a rally where you do not eat");
        assert!(s.act(0, Verb::Rally(mine)));
        assert_eq!(s.dish.beacons.len(), 1);
        assert_eq!(s.dish.beacons[0].2, RALLY_GAIN);
        // A beacon on a rallied pile does not dim it.
        s.sides[0].bank = BEACON_COST;
        assert!(s.act(0, Verb::Beacon(mine)));
        assert_eq!(s.dish.beacons.len(), 1);
        assert_eq!(s.dish.beacons[0].2, RALLY_GAIN);
        run(&mut s, RALLY_SECONDS + 1);
        assert_eq!(s.dish.beacons[0].2, BEACON_GAIN, "the rally never went out");
        assert_eq!(s.price(0, Verb::Spore(theirs)), SPORE_COST);
        s.sides[0].bank = 10.0 * SPORE_COST;
        assert!(s.act(0, Verb::Spore(theirs)));
        assert!(s.act(0, Verb::Spore(theirs)));
        assert_eq!(s.price(0, Verb::Spore(theirs)), 2.0 * SPORE_COST);
        assert_eq!(s.price(1, Verb::Spore(mine)), SPORE_COST, "the rival pays for your burst");
        run(&mut s, 31);
        assert_eq!(s.price(0, Verb::Spore(theirs)), SPORE_COST, "the price never came back down");
    }
}
