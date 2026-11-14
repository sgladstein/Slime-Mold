//! **Rounds** (2026-11-12, the owner's "Build it" on the Flashes of Chaos
//! research, `Reports/flashes-of-chaos-guns-2026-11-12.md`): a soldier's
//! ammunition as up to four cards in order, after the ammo-card guns of
//! Flashes of Chaos and the spell decks of Noita's wands. Every card works
//! through a field the mould already reads (the kill and its crater, the
//! alarm, sleep, the germ), so what a round does is drawn by the mould
//! itself, not painted on top.
//!
//! **Cards**, four kinds:
//! - *Payloads* are the squad toolkit's tools (`mould::kit`), cast where a
//!   soldier's shot hit: `Blast` (kill, crater, alarm), `Gas` (sleep),
//!   `Germ` (contagion). Firing one is free: "The bank is just to build
//!   the gun. How many and which cards" (owner 18:38Z). Until then each
//!   cast was paid from the bank at the toolkit's price (blast 60, gas
//!   150, germ 300, bomb 60), skipped when the bank was short; with the
//!   recharge gone eight soldiers on `lull` spent 373 a second, a 2,000
//!   start bank in about 5 s. The reviewers' call (game designer and technical
//!   lead, 15:31Z): no third weapons system, the payloads are the toolkit.
//!   `Bomb` (2026-11-14, the owner's "Do we have explosions yet. That
//!   should also be a thing", after Noita's splash,
//!   `Reports/noita-spell-look-2026-11-14.md` step 3) is the one payload
//!   that is the rounds' own: a smaller kill and crater than a blast, and a
//!   ring out to `bomb_ring` x its radius whose live mould is thrown
//!   outward as real walkers (`squad::Burst::fling`) that land and live
//!   on. It cuts the body and spreads it, a trade against blast and germ.
//! - *Modifiers* change the next payload: `Wide` (radius x `wide`),
//!   `Scatter` (`scatter_n` smaller bursts round the hit), `Quiet` (no
//!   alarm, so a Sleeper or a flee tier is not woken or warned).
//! - *Triggers* decide when the next payload goes off: `Delay` (it lodges
//!   and fires `delay_secs` later, once the mould has flowed back over the
//!   hole) and `Thick` (it waits up to `thick_secs` until `thick_n` walkers
//!   stand within `thick_r`: the mould sets off its own trap).
//! - *Flight* cards (2026-11-13, the owner's "Why doesn't ours change the
//!   flight too?" and yes to moving bullets then pierce and split) change
//!   every bullet the soldier fires, not a cast: `Pierce` flies on through
//!   one more slime hit, each striking; `Split` breaks the spent bullet
//!   into `Rules::split_n` pieces fanned on past the hit. They need
//!   bullets (`Rules::bullet_speed` above 0) and are game state, so both
//!   worlds fly them alike (`Game::landed`). Like the others they cost
//!   budget and fill a slot.
//!
//! **How a round fires** (Noita's deck): the cards are read left to right
//! into casts, each a run of modifiers and at most one trigger ending in a
//! payload; cards after the last payload do nothing. Every shot a soldier
//! fires carries the next cast in turn (`Soldier::next`, `Bullet::cast`),
//! back to the first after the last, at the gun's own rate of fire, and
//! the cast goes off where the bullet hits slime; a bullet that hits none
//! casts nothing. There is no recharge (owner 18:05Z: "Why does the gun
//! have a recharge at all. That shouldn't be a thing. It has a rate of
//! fire. It never has to stop to recharge or switch to regular bullets").
//! Until then a gun recharged `RoundRules::recharge` 2 s plus each card's
//! weight (a payload 1 s, Wide 1 s, any other 0.5 s) after its last cast,
//! firing plain shots meanwhile, and a hit, not a shot, took the next
//! cast. What limits a round is its gun's budget and slots
//! (`fit_gun`).
//!
//! **Bursts.** A cast compiles to a list of `squad::Burst`s (centre,
//! radius, kill, alarm, crater, gas, germ), applied by `World::burst`, so
//! rounds run on either world. Until 2026-11-12 evening this module had its
//! own `Burst` and a CPU-only `apply` with the same fields (agreed with the
//! squad toolkit thread so the switch was an import). On a world without
//! gas or germ (`World::has_gas`, `has_germ`; the GPU world has the gas
//! since 11-14, the germ to come) those payloads are not cast and cost nothing,
//! counted in `Rounds::unsupported`. Soldiers only (turret rounds later).
//!
//! **Caps** (technical lead): at most `PENDING_MAX` lodged casts at once
//! (a new one past the cap is dropped and counted in `Rounds::dropped`),
//! and at most `scatter_n` bursts a cast, so one frame casts at most a
//! burst list of soldiers x `scatter_n`.
use super::kit::KitRules;
use super::larder_world::{BulletMods, Game, World};
pub use super::squad::Burst;
use super::squad::BurstRules;

/// Most lodged casts (Delay and Thick) waiting at once.
pub const PENDING_MAX: usize = 64;
/// Cards the standard rifle holds (`guns::Gun::standard`); a rolled gun
/// holds its own `Gun::slots`, up to `MAX_SLOTS`.
pub const SLOTS: usize = 4;
/// Most cards any gun holds (`guns::SLOTS_MAX`): what a round reads.
pub const MAX_SLOTS: usize = super::guns::SLOTS_MAX as usize;

#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub enum Card {
    Blast,
    Bomb,
    Gas,
    Germ,
    Wide,
    Scatter,
    Quiet,
    Delay,
    Thick,
    Pierce,
    Split,
    Scorch,
    Bait,
    Mist,
    Range,
    Rate,
    Thrift,
    Smother,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Kind {
    Payload,
    Modifier,
    Trigger,
    Flight,
}

impl Card {
    pub const ALL: [Card; 18] = [Card::Blast, Card::Bomb, Card::Gas, Card::Germ, Card::Wide, Card::Scatter, Card::Quiet, Card::Delay, Card::Thick, Card::Pierce, Card::Split, Card::Scorch, Card::Bait, Card::Mist, Card::Range, Card::Rate, Card::Thrift, Card::Smother];

    pub fn kind(self) -> Kind {
        match self {
            Card::Blast | Card::Bomb | Card::Gas | Card::Germ => Kind::Payload,
            Card::Wide | Card::Scatter | Card::Quiet => Kind::Modifier,
            Card::Delay | Card::Thick => Kind::Trigger,
            Card::Pierce | Card::Split | Card::Scorch | Card::Bait | Card::Mist | Card::Range | Card::Rate | Card::Smother => Kind::Flight,
            Card::Thrift => Kind::Modifier,
        }
    }

    pub fn name(self) -> &'static str {
        match self {
            Card::Blast => "blast",
            Card::Bomb => "bomb",
            Card::Gas => "gas",
            Card::Germ => "germ",
            Card::Wide => "wide",
            Card::Scatter => "scatter",
            Card::Quiet => "quiet",
            Card::Delay => "delay",
            Card::Thick => "thick",
            Card::Pierce => "pierce",
            Card::Split => "split",
            Card::Scorch => "scorch",
            Card::Bait => "bait",
            Card::Mist => "mist",
            Card::Range => "range",
            Card::Rate => "rate",
            Card::Thrift => "thrift",
            Card::Smother => "smother",
        }
    }

    /// What the card does, in a line (the HUD's loadout panel).
    pub fn about(self) -> &'static str {
        match self {
            Card::Blast => "Payload: kills the mould in a disc, leaves a crater and lays alarm",
            Card::Gas => "Payload: puts the mould in a disc to sleep; sleepers in gas may die",
            Card::Germ => "Payload: infects the mould in a disc; the sickness spreads through it",
            Card::Bomb => "Payload: kills a small core and throws the live mould round it outward",
            Card::Wide => "Modifier: the next payload reaches 1.6 times as far",
            Card::Scatter => "Modifier: the next payload lands as three smaller ones round the hit",
            Card::Quiet => "Modifier: the next payload lays no alarm",
            Card::Delay => "Trigger: the next payload lodges and goes off 3 s later",
            Card::Thick => "Trigger: the next payload lodges until the mould is thick round it",
            Card::Pierce => "Flight: every bullet flies on through one slime hit",
            Card::Split => "Flight: every bullet splits where it lands",
            Card::Scorch => "Trail: every bullet burns the mould along its flight and leaves a line of crater it shuns",
            Card::Bait => "Trail: every bullet leaves a line of food scent that every mould follows",
            Card::Mist => "Trail: every bullet leaves a thin gas cloud that puts the mould to sleep",
            Card::Range => "Flight: every soldier sees and shoots twice as far a card",
            Card::Rate => "Flight: every soldier fires twice as often a card",
            Card::Thrift => "Modifier: the next card costs 2 less of the gun's budget",
            Card::Smother => "Trail: every bullet wipes the food scent along its flight, so the mould loses the way",
        }
    }

    /// What the card takes of a gun's budget (`RoundRules::budget`; the
    /// owner's notes, 11-14 15:06Z: "Each card should have a cost and guns
    /// have x amount of bank to spend", and their card at 15:34Z: the
    /// budget is a capacity checked when the gun is loaded). A payload is
    /// the most, a card that changes every bullet next, a tweak least.
    pub fn cost(self) -> u32 {
        match self {
            Card::Bomb | Card::Germ => 4,
            Card::Blast | Card::Gas | Card::Scorch | Card::Mist => 3,
            Card::Wide | Card::Scatter | Card::Pierce | Card::Split | Card::Bait | Card::Range | Card::Rate | Card::Smother => 2,
            Card::Thrift => 0,
            Card::Quiet | Card::Delay | Card::Thick => 1,
        }
    }

    /// How rare the card is: 1 common, 2 uncommon, 3 rare. A run hands
    /// out higher tiers deeper in (the roguelike thread's rewards).
    pub fn tier(self) -> u8 {
        match self {
            Card::Bomb | Card::Germ => 3,
            Card::Scatter | Card::Thick | Card::Split | Card::Scorch | Card::Mist | Card::Thrift | Card::Smother => 2,
            _ => 1,
        }
    }

    pub fn from_name(s: &str) -> Option<Card> {
        Card::ALL.into_iter().find(|c| c.name() == s)
    }
}

/// A bullet's leg this step: from, to (its hit or the leg's end).
type Leg = ((f32, f32), (f32, f32));

/// A squad's own cards (`Squad::deck`; owner 11-14 16:46Z, "The squad owns
/// the gun"): up to `MAX_SLOTS`, held as a value so a squad, a split half
/// and a replay line (`Cmd::Load`) carry it. A squad with none fires the
/// army's round (`Rounds::round`, the H loadout).
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq, Hash)]
pub struct Deck {
    cards: [Option<Card>; MAX_SLOTS],
}

impl Deck {
    /// The first `MAX_SLOTS` of `cards`.
    pub fn new(cards: &[Card]) -> Deck {
        let mut d = Deck::default();
        for (slot, &c) in d.cards.iter_mut().zip(cards) {
            *slot = Some(c);
        }
        d
    }

    pub fn cards(&self) -> Vec<Card> {
        self.cards.iter().flatten().copied().collect()
    }

    pub fn is_empty(&self) -> bool {
        self.cards[0].is_none()
    }
}

/// A round written as `blast` or `quiet+gas+germ`.
pub fn parse(s: &str) -> Option<Vec<Card>> {
    if s == "none" || s.is_empty() {
        return Some(Vec::new());
    }
    let v: Option<Vec<Card>> = s.split('+').map(Card::from_name).collect();
    v.filter(|v| v.len() <= MAX_SLOTS)
}

/// A round's cost against a gun's budget (`Card::cost`). A `Thrift`
/// takes `THRIFT` off the card after it (not below 0); thrift after thrift
/// stacks on the next card that is not one.
pub fn cost(round: &[Card]) -> u32 {
    let mut cut = 0;
    let mut sum = 0;
    for &c in round {
        if c == Card::Thrift {
            cut += THRIFT;
        } else {
            sum += c.cost().saturating_sub(cut);
            cut = 0;
        }
    }
    sum
}

/// What a `Thrift` takes off the next card's cost.
pub const THRIFT: u32 = 2;

/// A set of cards that does not fit a gun: what it costs, the budget, and
/// how many cards against `SLOTS`.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct Over {
    pub cost: u32,
    pub budget: u32,
    pub cards: usize,
}

/// The cards, if they fit a gun of `budget`: their cost within it and at
/// most `SLOTS` of them (the standard rifle's slots).
pub fn fit(round: Vec<Card>, budget: u32) -> Result<Vec<Card>, Over> {
    fit_in(round, budget, SLOTS)
}

/// The cards, if they fit `gun` (owner 11-14 16:46Z: "Each gun defines
/// the budget and slot count"): cost within its budget, no more cards
/// than its slots.
pub fn fit_gun(round: Vec<Card>, gun: &super::guns::Gun) -> Result<Vec<Card>, Over> {
    fit_in(round, gun.budget, gun.slots as usize)
}

fn fit_in(round: Vec<Card>, budget: u32, slots: usize) -> Result<Vec<Card>, Over> {
    let c = cost(&round);
    if c <= budget && round.len() <= slots.min(MAX_SLOTS) {
        Ok(round)
    } else {
        Err(Over { cost: c, budget, cards: round.len() })
    }
}

pub fn name(round: &[Card]) -> String {
    if round.is_empty() {
        return "none".into();
    }
    round.iter().map(|c| c.name()).collect::<Vec<_>>().join("+")
}

#[derive(Clone, Copy, Debug)]
pub struct RoundRules {
    /// `Wide`: radius factor.
    pub wide: f32,
    /// `Scatter`: bursts, each at radius x `scatter_r`, set round the hit
    /// `scatter_d` radii out.
    pub scatter_n: usize,
    pub scatter_r: f32,
    pub scatter_d: f32,
    pub delay_secs: f32,
    /// `Thick`: walkers within `thick_r` dish cells that set it off, and how
    /// long it waits before it goes off anyway.
    pub thick_n: u32,
    pub thick_r: f32,
    pub thick_secs: f32,
    /// `Bomb`: its kill radius (dish cells), the fling ring's outer radius
    /// x that, how long the walkers nearest the kill fly (game seconds;
    /// the ring's outer edge barely moves), a flung walker's speed x
    /// `step` a fine step, and its crater (radius, seconds).
    /// At the race world's 30 fine steps a second and step 1, 0.5 s at 1.5
    /// throws the inner edge 22 fine cells, 11 dish cells.
    pub bomb_r: f32,
    pub bomb_ring: f32,
    pub bomb_secs: f32,
    pub fling_speed: f32,
    pub bomb_crater_r: f32,
    pub bomb_crater_secs: f32,
    /// **Trails** (the flight cards `Scorch`, `Bait`, `Mist`, 2026-11-14,
    /// after Noita's burning and gunpowder trails: step 5 of
    /// `Reports/noita-spell-look-2026-11-14.md`): each soldier bullet's leg
    /// a frame lays bursts every `trail_r` along it, radius `trail_r` dish
    /// cells (x the square root of the copies of the card): crater for
    /// `trail_crater_secs`, food scent `trail_bait` x a soldier's frame of
    /// smell, or `trail_cloud` of gas cloud. Free: the cards cost budget.
    ///
    /// `Bait` replaced `Alarm` on 2026-11-14 (owner's card, 09:02Z): the
    /// alarm trail laid `trail_alarm` 0.5 x a blast's alarm, which the
    /// default mould ignores (`alarm_weight` 0); only the Sleeper, Swarmer
    /// and Grudge walked to it, so on most maps the card did nothing.
    pub trail_r: f32,
    pub trail_crater_secs: f32,
    pub trail_bait: f32,
    pub trail_cloud: f32,
    /// `Scorch` also burns: kills the walkers within its crater radius
    /// along the flight (owner, 2026-11-14 08:41Z, after "how are scorch
    /// and alarm different. It just makes a virtual wall for the slime?").
    /// Off, scorch is the crater wall alone, as #244 built it.
    pub trail_burn: bool,
    /// A gun's budget (`Card::cost`, `fit`, `Rounds::build`): 10 fits
    /// every test loadout the window's H cycles (the dearest, shock, is
    /// 10). A run may grow it (the roguelike thread).
    pub budget: u32,
    /// The toolkit's numbers for the payloads (radii, prices, crater).
    pub kit: KitRules,
}

impl Default for RoundRules {
    fn default() -> Self {
        RoundRules {
            wide: 1.6,
            scatter_n: 3,
            scatter_r: 0.6,
            scatter_d: 1.5,
            delay_secs: 3.0,
            thick_n: 40,
            thick_r: 5.0,
            thick_secs: 30.0,
            bomb_r: 4.0,
            bomb_ring: 2.5,
            bomb_secs: 0.5,
            fling_speed: 1.5,
            bomb_crater_r: 4.0,
            bomb_crater_secs: 6.0,
            trail_r: 2.0,
            trail_crater_secs: 4.0,
            trail_bait: 40.0,
            trail_cloud: 0.4,
            trail_burn: true,
            budget: 10,
            kit: KitRules::default(),
        }
    }
}

/// A payload with what its modifiers and trigger made of it.
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct Cast {
    pub payload: Card,
    pub wide: u32,
    pub scatter: bool,
    pub quiet: bool,
    pub trigger: Option<Card>,
}

/// The round's casts, left to right. Modifiers and triggers apply to the
/// next payload; a second trigger before it replaces the first.
pub fn casts(round: &[Card]) -> Vec<Cast> {
    let mut out = Vec::new();
    let (mut wide, mut scatter, mut quiet, mut trigger) = (0, false, false, None);
    for &c in round.iter().take(MAX_SLOTS) {
        match c {
            Card::Wide => wide += 1,
            Card::Scatter => scatter = true,
            Card::Quiet => quiet = true,
            Card::Delay | Card::Thick => trigger = Some(c),
            // Flight cards ride on every bullet, not on a cast (`flight`).
            Card::Pierce | Card::Split | Card::Scorch | Card::Bait | Card::Mist | Card::Range | Card::Rate | Card::Smother | Card::Thrift => {}
            Card::Blast | Card::Bomb | Card::Gas | Card::Germ => {
                out.push(Cast { payload: c, wide, scatter, quiet, trigger });
                (wide, scatter, quiet, trigger) = (0, false, false, None);
            }
        }
    }
    out
}

/// The round's flight cards as every soldier bullet carries them: each
/// `pierce` one more slime hit flown through, each `split` one more
/// generation of pieces; and its casts, one a payload, which its shots
/// carry in turn.
pub fn flight(round: &[Card]) -> BulletMods {
    let cards = || round.iter().take(MAX_SLOTS);
    let n = |k: Card| cards().filter(|&&c| c == k).count() as u8;
    let casts = cards().filter(|c| c.kind() == Kind::Payload).count() as u8;
    BulletMods { pierce: n(Card::Pierce), split: n(Card::Split), reach: n(Card::Range), rate: n(Card::Rate), casts }
}

/// The round's trail cards as every soldier bullet lays them: copies of
/// `Scorch`, `Bait`, `Mist` and `Smother`.
pub fn trails(round: &[Card]) -> [u8; 4] {
    let n = |k: Card| round.iter().take(MAX_SLOTS).filter(|&&c| c == k).count() as u8;
    [n(Card::Scorch), n(Card::Bait), n(Card::Mist), n(Card::Smother)]
}

/// The bursts a trail lays along one bullet leg from `from` to `to` (fine
/// cells): a disc every `trail_r` from the leg's start, each with every
/// trail the round has. None without trail cards. `smell` is a soldier's
/// frame of food scent (`Rules::smell` x the world's `food_deposit` x
/// `frame_ticks`), the bait's unit.
pub fn trail_bursts(n: [u8; 4], from: (f32, f32), to: (f32, f32), r: &RoundRules, scale: f32, fine: f32, smell: f32) -> Vec<Burst> {
    if n == [0; 4] {
        return Vec::new();
    }
    let (a, b) = ((from.0 / scale, from.1 / scale), (to.0 / scale, to.1 / scale));
    let len = (b.0 - a.0).hypot(b.1 - a.1);
    let gap = r.trail_r.max(0.25);
    let discs = (len / gap).ceil().max(1.0) as usize;
    let f = |c: u8| (c as f32).sqrt();
    let rr = r.trail_r * f(n.into_iter().max().unwrap_or(0));
    let crater = if n[0] > 0 { (r.trail_crater_secs * fine).round().clamp(0.0, u16::MAX as f32) as u16 } else { 0 };
    let bait = r.trail_bait * smell * n[1] as f32;
    let cloud = r.trail_cloud * n[2] as f32;
    (1..=discs)
        .map(|j| {
            let t = (j as f32 * gap).min(len) / len.max(1e-6);
            let at = (a.0 + (b.0 - a.0) * t, a.1 + (b.1 - a.1) * t);
            Burst { at, r: rr, burn: n[0] > 0 && r.trail_burn, crater, crater_r: if crater > 0 { r.trail_r * f(n[0]) } else { 0.0 }, bait, smother: n[3] > 0, cloud, ..Burst::default() }
        })
        .collect()
}

/// The bursts one cast makes at `at` (fine cells); `scale` fine cells a
/// dish cell and `fine` fine steps a game second.
pub fn bursts(c: &Cast, at: (f32, f32), r: &RoundRules, scale: f32, fine: f32) -> Vec<Burst> {
    let k = &r.kit;
    let f = r.wide.powi(c.wide as i32);
    let steps = |secs: f32| (secs * fine).round().clamp(0.0, u16::MAX as f32) as u16;
    let at = (at.0 / scale, at.1 / scale);
    let alarm = if c.quiet { 0.0 } else { k.blast_alarm };
    let none = Burst { at, r: 0.0, kill: false, burn: false, alarm: 0.0, alarm_r: 0.0, crater: 0, crater_r: 0.0, gas: 0, germ: false, bait: 0.0, smother: false, fling: 0.0, fling_steps: 0, cloud: 0.0 };
    let one = match c.payload {
        Card::Blast => Burst { r: k.blast_r * f, kill: true, alarm, alarm_r: 2.0 * k.blast_r * f, crater: steps(k.crater_secs), crater_r: k.crater_r * f, ..none },
        Card::Bomb => {
            let br = r.bomb_r * f;
            Burst { r: br, kill: true, alarm, alarm_r: 2.0 * br, crater: steps(r.bomb_crater_secs), crater_r: r.bomb_crater_r * f, fling: br * r.bomb_ring, fling_steps: steps(r.bomb_secs).max(1), ..none }
        }
        Card::Gas => Burst { r: k.gas_r * f, gas: steps(k.gas_secs), ..none },
        _ => Burst { r: k.germ_r * f, germ: true, ..none },
    };
    if !c.scatter {
        return vec![one];
    }
    let n = r.scatter_n.max(1);
    let d = one.r * r.scatter_d;
    (0..n)
        .map(|j| {
            let a = j as f32 * std::f32::consts::TAU / n as f32;
            let q = r.scatter_r;
            Burst { at: (at.0 + d * a.cos(), at.1 + d * a.sin()), r: one.r * q, alarm_r: one.alarm_r * q, crater_r: one.crater_r * q, fling: one.fling * q, ..one }
        })
        .collect()
}

/// The bursts' shared settings from the toolkit's rules, as `Kit` makes
/// them; `fine` fine steps a game second. The rounds' own `Bomb` adds the
/// fling's speed (`Rounds::fire`).
pub fn burst_rules(k: &KitRules, fine: f32) -> BurstRules {
    let sick_steps = (k.sick_secs * fine).round().max(1.0) as u16;
    let (gas_keep, gas_linger) = k.gas_cloud(fine);
    BurstRules {
        alarm_keep: k.alarm_keep,
        gas_death: k.gas_death,
        gas_keep,
        gas_dose: k.gas_dose,
        gas_sleep: k.gas_sleep,
        gas_linger,
        sick_steps,
        immune_steps: (k.immune_secs * fine).round().min(u16::MAX as f32 - sick_steps as f32) as u16,
        sick_upkeep: k.sick_upkeep,
        sick_catch: k.sick_catch,
        sick_deposit: k.sick_deposit,
        sick_keep: k.sick_keep,
        fling_speed: 0.0,
    }
}

/// Whether a world can cast this payload.
fn castable<W: World>(w: &W, p: Card) -> bool {
    match p {
        Card::Gas => w.has_gas(),
        Card::Germ => w.has_germ(),
        _ => true,
    }
}

#[derive(Clone, Copy, Debug, PartialEq)]
struct Pending {
    cast: Cast,
    at: (f32, f32),
    /// Tick it goes off by (Delay) or gives up waiting and goes off (Thick).
    due: u64,
}

/// One burst as the window or a page draws it.
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct Pop {
    pub burst: Burst,
    pub payload: Card,
    pub tick: u64,
    pub killed: usize,
    pub flung: usize,
}

/// The army's rounds: one loadout for every squad without its own cards
/// (the game designer: loadouts per squad, set at the larder or between
/// missions, not per soldier mid-fight), and what was cast. Which cast a
/// shot carries is game state (`Soldier::next`, `Bullet::cast`).
#[derive(Clone, Debug, Default)]
pub struct Rounds {
    pub rules: RoundRules,
    pub round: Vec<Card>,
    casts: Vec<Cast>,
    pending: Vec<Pending>,
    /// Casts by payload (blast, gas, germ, bomb), walkers killed by
    /// rounds, walkers flung by bombs, and casts dropped at `PENDING_MAX`.
    pub cast: [u32; 4],
    pub killed: usize,
    pub flung: usize,
    pub dropped: u32,
    /// Casts skipped because this world has no gas or germ yet.
    pub unsupported: u32,
    /// Trail bursts laid (`trail_bursts`), all kinds.
    pub trailed: u32,
    /// Walkers the `Scorch` trail burned (in `killed` too).
    pub burned: usize,
    pub pops: Vec<Pop>,
}

fn slot(p: Card) -> usize {
    match p {
        Card::Blast => 0,
        Card::Gas => 1,
        Card::Bomb => 3,
        _ => 2,
    }
}

impl Rounds {
    /// A gun with these cards, unchecked (the window's test loadouts).
    pub fn new(round: Vec<Card>, rules: RoundRules) -> Self {
        let casts = casts(&round);
        Rounds { rules, round, casts, ..Default::default() }
    }

    /// A gun with these cards if they fit `rules.budget` (`fit`): how a
    /// run equips the cards a player owns.
    pub fn build(round: Vec<Card>, rules: RoundRules) -> Result<Self, Over> {
        fit(round, rules.budget).map(|r| Rounds::new(r, rules))
    }

    /// Puts the round's flight cards and its count of casts on the game's
    /// soldier bullets (also done by every `after_step`, so a game rounds
    /// join late still flies them from the next shot).
    pub fn arm<W: World>(&self, g: &mut Game<W>) {
        g.bullet_mods = flight(&self.round);
    }

    /// Lodged casts waiting.
    pub fn waiting(&self) -> usize {
        self.pending.len()
    }

    /// The round's casts, left to right.
    pub fn casts(&self) -> &[Cast] {
        &self.casts
    }

    /// The cards squad `q` fires: its own (`Squad::deck`, `Cmd::Load`) or
    /// the army's round.
    pub fn cards_of<W: World>(&self, g: &Game<W>, q: usize) -> Vec<Card> {
        match g.squads.get(q).and_then(|s| s.deck) {
            Some(d) => d.cards(),
            None => self.round.clone(),
        }
    }

    /// Squad `q`'s casts, left to right (`casts` of `cards_of`).
    pub fn casts_of<W: World>(&self, g: &Game<W>, q: usize) -> Vec<Cast> {
        match g.squads.get(q).and_then(|s| s.deck) {
            Some(d) => casts(&d.cards()),
            None => self.casts.clone(),
        }
    }

    /// The cast a shot of squad `q`'s carries (`Shot::cast`,
    /// `Bullet::cast`; 0 plain): its `cast - 1`th, None if plain or past
    /// the end (its cards changed in flight).
    pub fn cast_of<W: World>(&self, g: &Game<W>, q: usize, cast: u8) -> Option<Cast> {
        let i = (cast as usize).checked_sub(1)?;
        match g.squads.get(q).and_then(|s| s.deck) {
            Some(d) => casts(&d.cards()).get(i).copied(),
            None => self.casts.get(i).copied(),
        }
    }

    /// The index (into its squad's `casts_of`) of the cast soldier `k`'s
    /// next shot carries; None when its squad's cards have no payload.
    pub fn next<W: World>(&self, g: &Game<W>, k: usize) -> Option<usize> {
        let s = g.soldiers.get(k)?;
        let n = g.mods(s.squad as usize).casts as usize;
        (n > 0).then(|| s.next as usize % n)
    }

    /// Lodged casts: what, where (fine cells), and the tick each goes off
    /// by.
    pub fn lodged(&self) -> impl Iterator<Item = (Cast, (f32, f32), u64)> + '_ {
        self.pending.iter().map(|p| (p.cast, p.at, p.due))
    }

    /// After each `Game::step`: each soldier's bullet that hit slime this
    /// step casts the cast it carries (`Shot::cast`, its squad's cards),
    /// and lodged casts that are due go off. Turret shots and split pieces
    /// carry none.
    pub fn after_step<W: World>(&mut self, g: &mut Game<W>) {
        self.arm(g);
        self.lay_trails(g);
        let t = g.world.ticks();
        let tps = g.rules.tps;
        // Each squad casts its own cards (`casts_of`): the army's round, or
        // its own deck (`Cmd::Load`, the squad owns the gun, 11-14).
        let mut by_squad: Vec<Option<Vec<Cast>>> = vec![None; g.squads.len()];
        let hits: Vec<(usize, (f32, f32), u8)> = g.last_shots.iter().filter(|s| s.cast > 0).filter_map(|s| s.hit.zip(s.by).map(|(h, k)| (k, h, s.cast))).collect();
        for (k, hit, cast) in hits {
            // A bullet flies on when its soldier falls: it still casts.
            let Some(q) = g.soldiers.get(k).map(|s| s.squad as usize) else { continue };
            if q >= by_squad.len() {
                continue;
            }
            let cs = by_squad[q].get_or_insert_with(|| self.casts_of(g, q));
            let Some(&c) = cs.get(cast as usize - 1) else { continue };
            if !castable(&g.world, c.payload) {
                self.unsupported += 1;
            } else {
                self.cast[slot(c.payload)] += 1;
                match c.trigger {
                    None => self.fire(g, &c, hit),
                    Some(tr) => {
                        if self.pending.len() >= PENDING_MAX {
                            self.dropped += 1;
                        } else {
                            let secs = if tr == Card::Delay { self.rules.delay_secs } else { self.rules.thick_secs };
                            self.pending.push(Pending { cast: c, at: hit, due: t + (secs * tps as f32).round() as u64 });
                        }
                    }
                }
            }
        }
        if self.pending.is_empty() {
            return;
        }
        // Thick: walkers within `thick_r` dish cells, by the world's own
        // count (exact on the CPU, the frame's mould grid on the GPU; until
        // 2026-11-12 evening a scan of the CPU occupancy here).
        let rr = self.rules.thick_r * g.world.scale();
        let near = |w: &W, p: (f32, f32)| w.walkers_in(p, rr);
        let mut go = Vec::new();
        self.pending.retain(|p| {
            let fire = t >= p.due || (p.cast.trigger == Some(Card::Thick) && near(&g.world, p.at) >= self.rules.thick_n);
            if fire {
                go.push(*p);
            }
            !fire
        });
        for p in go {
            self.fire(g, &p.cast, p.at);
        }
    }

    /// Every cast of the round at once at `at` (fine cells), free and with
    /// no trigger: a staged scene for a film or a test (`larder-world
    /// boom=`), the same bursts a hit there would cast.
    pub fn cast_at<W: World>(&mut self, g: &mut Game<W>, at: (f32, f32)) {
        for c in self.casts.clone() {
            self.cast[slot(c.payload)] += 1;
            self.fire(g, &c, at);
        }
    }

    /// Every soldier bullet's leg this step lays the round's trails
    /// (`trail_bursts`), up to its hit. A gas trail needs a world with gas.
    fn lay_trails<W: World>(&mut self, g: &mut Game<W>) {
        // Each leg lays its squad's trails (`cards_of`).
        let gas = g.world.has_gas();
        let mut of: Vec<Option<[u8; 4]>> = vec![None; g.squads.len()];
        let mut any = [0u8; 4];
        let legs: Vec<(Leg, [u8; 4])> = g
            .last_shots
            .iter()
            .filter_map(|s| s.by.map(|k| (s, g.soldiers.get(k).map_or(0, |x| x.squad as usize))))
            .map(|(s, q)| {
                let n = *of[q].get_or_insert_with(|| {
                    let mut n = trails(&self.cards_of(g, q));
                    if !gas {
                        n[2] = 0;
                    }
                    n
                });
                for (a, b) in any.iter_mut().zip(n) {
                    *a = (*a).max(b);
                }
                ((s.from, s.hit.unwrap_or(s.to)), n)
            })
            .collect();
        if any == [0; 4] {
            return;
        }
        let s = g.world.scale();
        let fine = g.rules.tps as f32 * g.world.fine_steps() as f32;
        let k = burst_rules(&self.rules.kit, fine);
        let smell = g.world.food_deposit() * g.rules.smell * g.rules.frame_ticks as f32;
        let mut burned = 0;
        let n = any;
        for ((from, to), legn) in legs {
            if legn == [0; 4] {
                continue;
            }
            for b in trail_bursts(legn, from, to, &self.rules, s, fine, smell) {
                burned += g.world.burst(&b, &k).killed as usize;
                self.trailed += 1;
            }
        }
        if n[0] > 0 && self.rules.trail_burn {
            burned += g.world.burned() as usize;
        }
        self.killed += burned;
        self.burned += burned;
        g.killed += burned as u32;
    }

    fn fire<W: World>(&mut self, g: &mut Game<W>, c: &Cast, at: (f32, f32)) {
        let s = g.world.scale();
        let t = g.world.ticks();
        let fine = g.rules.tps as f32 * g.world.fine_steps() as f32;
        let k = BurstRules { fling_speed: self.rules.fling_speed, ..burst_rules(&self.rules.kit, fine) };
        for b in bursts(c, at, &self.rules, s, fine) {
            let out = g.world.burst(&b, &k);
            let killed = out.killed as usize;
            self.killed += killed;
            self.flung += out.flung as usize;
            g.killed += killed as u32;
            self.pops.push(Pop { burst: b, payload: c.payload, tick: t, killed, flung: out.flung as usize });
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    /// The budget (owner's card, 11-14 15:34Z: capacity at loadout): a set
    /// that costs more than the gun's budget, or has more than `SLOTS`
    /// cards, is refused and says by how much; one that fits builds.
    #[test]
    fn a_gun_takes_only_the_cards_its_budget_holds() {
        let r = RoundRules::default();
        let ok = parse("scatter+blast+wide+blast").unwrap();
        assert_eq!(cost(&ok), 10);
        assert!(Rounds::build(ok, r).is_ok(), "10 fits a 10 budget");
        let dear = vec![Card::Bomb, Card::Germ, Card::Blast];
        assert_eq!(Rounds::build(dear.clone(), r).err(), Some(Over { cost: 11, budget: 10, cards: 3 }));
        assert!(Rounds::build(dear, RoundRules { budget: 11, ..r }).is_ok(), "a bigger gun takes it");
        // Thrift (the owner's "reduce cost"): 2 off the next card.
        assert_eq!(cost(&parse("thrift+bomb+thrift+bomb").unwrap()), 4);
        assert_eq!(cost(&parse("thrift+thrift+bomb").unwrap()), 0, "two thrifts stack, not below 0");
        assert_eq!(cost(&parse("blast+thrift").unwrap()), 3, "a thrift with nothing after saves nothing");
        let five = vec![Card::Quiet; 5];
        assert_eq!(fit(five, 99).err().map(|o| o.cards), Some(5), "at most SLOTS cards");
        assert!(Card::ALL.iter().all(|c| (c.cost() >= 1 || *c == Card::Thrift) && (1..=3).contains(&c.tier())));
    }

    #[test]
    fn trail_cards_lay_discs_along_the_leg_and_cast_nothing() {
        let r = RoundRules::default();
        let round = parse("scorch+bait+bait+mist").unwrap();
        assert!(casts(&round).is_empty());
        assert_eq!(trails(&round), [1, 2, 1, 0]);
        // A 9-dish-cell leg at scale 2: a disc every 2 cells, the last on the end.
        let b = trail_bursts(trails(&round), (2.0, 4.0), (20.0, 4.0), &r, 2.0, 30.0, 1.0);
        assert_eq!(b.len(), 5);
        assert_eq!(b.last().unwrap().at, (10.0, 2.0));
        let one = b[0];
        assert!(!one.kill && one.burn && one.gas == 0 && one.fling == 0.0, "scorch burns, counted later");
        let cold = RoundRules { trail_burn: false, ..r };
        assert!(!trail_bursts([1, 0, 0, 0], (0.0, 0.0), (9.0, 0.0), &cold, 1.0, 30.0, 1.0)[0].burn);
        assert!(!trail_bursts([0, 1, 1, 0], (0.0, 0.0), (9.0, 0.0), &r, 1.0, 30.0, 1.0)[0].burn, "only scorch burns");
        assert_eq!(one.crater, (r.trail_crater_secs * 30.0) as u16);
        assert!((one.bait - 2.0 * r.trail_bait).abs() < 1e-4 && one.alarm == 0.0 && (one.r - r.trail_r * 2f32.sqrt()).abs() < 1e-5, "two baits: twice the scent, over the wider disc");
        assert!((one.cloud - r.trail_cloud).abs() < 1e-6 && one.crater_r == r.trail_r);
        assert!(trail_bursts([0; 4], (0.0, 0.0), (9.0, 0.0), &r, 1.0, 30.0, 1.0).is_empty());
    }

    /// A mist round on the CPU world: the bullets' flight lays a real cloud
    /// and nothing is cast or paid for (the race world has the kit's cloud
    /// only once a burst brings `BurstRules::gas_keep`, so the trail turns it on).
    #[test]
    fn a_mist_round_lays_a_cloud_along_the_flight() {
        use crate::mould::larder_world::{race_world, Cmd, Order};
        let mut g = Game::new(race_world(1, 2, 5_000), Rules::default(), 1);
        for k in 0..g.soldiers.len() {
            assert!(g.apply(Cmd::Order(k, Order::Attack)));
        }
        let mut rd = Rounds::new(parse("mist").unwrap(), RoundRules::default());
        while g.time() < 20.0 && rd.trailed == 0 {
            g.step();
            rd.after_step(&mut g);
        }
        assert!(rd.trailed > 0, "the squad's bullets laid a trail");
        assert!(g.world.gas_cloud.iter().any(|&v| v > 0.0), "the trail is a real cloud");
        assert_eq!(rd.cast, [0; 4], "a trail casts nothing");
    }

    /// Scorch burns (owner, 2026-11-14 08:41Z): the slime on a scorch
    /// bullet's flight dies, counted in the game's kills, and more dies
    /// than the plain bullets alone kill on the same seed.
    #[test]
    fn a_scorch_round_burns_the_slime_on_the_line() {
        use crate::mould::larder_world::{race_world, Cmd, Order};
        let run = |round: &str| {
            let mut g = Game::new(race_world(1, 2, 5_000), Rules::default(), 1);
            for k in 0..g.soldiers.len() {
                assert!(g.apply(Cmd::Order(k, Order::Attack)));
            }
            let mut rd = Rounds::new(parse(round).unwrap(), RoundRules::default());
            while g.time() < 20.0 && g.end.is_none() {
                g.step();
                rd.after_step(&mut g);
            }
            (rd.burned, g.killed)
        };
        let (burned, killed) = run("scorch");
        let (none, plain) = run("none");
        assert_eq!(none, 0);
        assert!(burned > 0 && killed > plain, "burned {burned}, killed {killed} vs plain {plain}");
    }

    /// The squad owns the gun and its cards (owner 11-14 16:46Z): a squad
    /// loaded with its own deck (`Cmd::Load`) casts and flies those cards
    /// and no other squad does; both halves of a split carry the cards
    /// (owner 18:05Z); unloaded, a squad fires the army's round again.
    #[test]
    fn a_squad_with_its_own_cards_casts_them_and_the_army_does_not() {
        use crate::mould::larder_world::{race_world, Cmd, Order};
        let mut g = Game::new(race_world(1, 2, 5_000), Rules { squads: true, ..Rules::default() }, 1);
        let rd = Rounds::new(Vec::new(), RoundRules::default());
        assert!(g.apply(Cmd::Load(0, Some(Deck::new(&parse("pierce+range+blast").unwrap())))));
        assert_eq!(rd.casts_of(&g, 0).len(), 1);
        assert_eq!((g.mods(0).pierce, g.mods(0).reach), (1, 1));
        assert!((g.gun_range(0) - 2.0 * g.rules.range).abs() < 1e-4, "its range card doubles its reach");
        let m = g.middle(0).unwrap();
        let b = g.split(0, (m.0 + 30.0, m.1)).expect("a split");
        assert_eq!(rd.cards_of(&g, 0), parse("pierce+range+blast").unwrap(), "the half left behind keeps the cards");
        assert_eq!(rd.cards_of(&g, b), parse("pierce+range+blast").unwrap(), "the new half carries a copy");
        assert!(g.apply(Cmd::Load(b as u8, None)));
        assert_eq!(rd.cards_of(&g, 0), parse("pierce+range+blast").unwrap(), "a copy: unloading one half leaves the other's");
        assert!(rd.casts_of(&g, b).is_empty() && g.mods(b) == BulletMods::default(), "unloaded: the army's plain round");
        assert_eq!(g.gun_range(b), g.rules.range);
        // In play: squad 0's hits cast blasts; with no deck nobody's do.
        let blasts = |own: bool| {
            let mut g = Game::new(race_world(1, 2, 5_000), Rules::default(), 1);
            let mut rd = Rounds::new(Vec::new(), RoundRules::default());
            if own {
                g.apply(Cmd::Load(0, Some(Deck::new(&[Card::Blast]))));
            }
            for k in 0..g.soldiers.len() {
                g.apply(Cmd::Order(k, Order::Attack));
            }
            while g.time() < 15.0 && g.end.is_none() {
                g.step();
                rd.after_step(&mut g);
            }
            rd.cast[0]
        };
        assert_eq!(blasts(false), 0);
        assert!(blasts(true) > 0, "the loaded squad blasts");
        assert_eq!(Cmd::parse(&Cmd::Load(3, Some(Deck::new(&[Card::Wide, Card::Blast]))).to_line()), Some(Cmd::Load(3, Some(Deck::new(&[Card::Wide, Card::Blast])))));
        assert_eq!(Cmd::parse(&Cmd::Load(3, None).to_line()), Some(Cmd::Load(3, None)));
        assert_eq!(Cmd::parse(&Cmd::Load(1, Some(Deck::default())).to_line()), Some(Cmd::Load(1, Some(Deck::default()))), "an empty deck: plain, not the army's");
    }

    #[test]
    fn flight_cards_ride_every_bullet_and_cast_nothing() {
        let r = RoundRules::default();
        let round = parse("pierce+split+split+blast").unwrap();
        assert_eq!(flight(&round), BulletMods { pierce: 1, split: 2, reach: 0, rate: 0, casts: 1 });
        assert_eq!(flight(&parse("range+range+pierce").unwrap()), BulletMods { pierce: 1, split: 0, reach: 2, rate: 0, casts: 0 });
        assert_eq!(casts(&round).len(), 1);
        assert_eq!(casts(&parse("split").unwrap()), vec![]);
        assert_eq!(flight(&parse("blast+wide+gas+quiet").unwrap()).casts as usize, casts(&parse("blast+wide+gas+quiet").unwrap()).len());
        let mut g = Game::new(BigDish::new(race_params(1, 1, 500)), Rules::default(), 1);
        Rounds::new(round, r).arm(&mut g);
        assert_eq!(g.bullet_mods, BulletMods { pierce: 1, split: 2, reach: 0, rate: 0, casts: 1 });
        assert_eq!(g.gun_range(0), g.rules.range);
        Rounds::new(parse("range+blast").unwrap(), r).arm(&mut g);
        assert!((g.gun_range(0) - g.rules.range * (1.0 + g.rules.range_card)).abs() < 1e-4, "one Range card stretches the reach");
    }

    #[test]
    fn modifiers_and_triggers_go_to_the_next_payload() {
        let round = parse("wide+blast+thick+gas").unwrap();
        let c = casts(&round);
        assert_eq!(c.len(), 2);
        assert_eq!((c[0].payload, c[0].wide, c[0].trigger), (Card::Blast, 1, None));
        assert_eq!((c[1].payload, c[1].wide, c[1].trigger), (Card::Gas, 0, Some(Card::Thick)));
        // Cards after the last payload do nothing.
        assert_eq!(casts(&parse("blast+quiet").unwrap()), casts(&parse("blast").unwrap()));
        // A round reads up to `MAX_SLOTS` (a rolled gun's most, 6); the
        // rifle's 4 is `fit`'s to check.
        assert!(parse("blast+blast+blast+blast+blast+blast+blast").is_none());
        let five = parse("blast+blast+blast+blast+blast").unwrap();
        assert_eq!(fit(five, 99).err().map(|o| o.cards), Some(5));
        assert_eq!(name(&round), "wide+blast+thick+gas");
    }

    #[test]
    fn scatter_splits_a_payload_and_quiet_lays_no_alarm() {
        let r = RoundRules::default();
        let c = casts(&parse("scatter+quiet+blast").unwrap())[0];
        let b = bursts(&c, (100.0, 100.0), &r, 2.0, 30.0);
        assert_eq!(b.len(), r.scatter_n);
        assert!(b.iter().all(|b| b.alarm == 0.0 && b.r < r.kit.blast_r && b.kill));
        let loud = bursts(&casts(&parse("blast").unwrap())[0], (100.0, 100.0), &r, 2.0, 30.0);
        assert_eq!(loud.len(), 1);
        assert!(loud[0].alarm > 0.0);
    }

    use crate::mould::big::BigDish;
    use crate::mould::kit::{Kit, Tool};
    use crate::mould::larder_world::{race_params, Cmd, Order, Rules};

    fn game(seed: u64) -> Game<BigDish> {
        let mut w = BigDish::new(race_params(seed, 2, 3000));
        w.threads = 1;
        let mut g = Game::new(w, Rules::default(), seed);
        for _ in 0..30 {
            g.step();
        }
        g
    }

    #[test]
    fn a_blast_burst_is_a_kit_blast() {
        // The squad toolkit thread's ask (11-12 16:15Z): a burst and a kit
        // blast leave the same dish on the same seed, so the switch to the
        // shared burst list can be checked.
        let (mut a, mut b) = (game(3), game(3));
        assert_eq!(a.world.hash(), b.world.hash(), "two games of one seed differ");
        let s = a.soldiers[0];
        let p = (s.x + 4.0, s.y);
        let mut kit = Kit::new(KitRules { carry_max: 0, ..KitRules::default() });
        assert!(kit.use_tool(&mut a, Tool::Blast(p)));
        let r = RoundRules { kit: kit.rules, ..RoundRules::default() };
        let fine = b.rules.tps as f32 * b.world.p.substeps as f32;
        let c = casts(&[Card::Blast])[0];
        let k = burst_rules(&r.kit, fine);
        let killed: usize = bursts(&c, p, &r, b.world.p.scale as f32, fine).iter().map(|x| b.world.burst(x, &k).killed as usize).sum();
        assert_eq!(killed, kit.killed[0]);
        // The larder smells of the bank, so both pay the kit's price (a
        // round's cast is free since 18:38Z; the kit's tools still cost).
        b.bank -= r.kit.blast_price;
        assert_eq!(a.bank, b.bank);
        assert_eq!(a.world.hash(), b.world.hash());
        assert_eq!(a.world.scorch, b.world.scorch);
        for _ in 0..20 {
            a.step();
            b.step();
        }
        assert_eq!(a.world.hash(), b.world.hash());
    }

    /// **A bomb** (11-14): it kills a small core, and the live mould in its
    /// ring flies straight out from the centre, lands within its flight,
    /// and lives on there: the body is cut and spread at once.
    #[test]
    fn a_bomb_flings_its_rim_and_the_flung_live_on() {
        use crate::mould::big::{LIFE_SPORE, LIFE_WALKER};
        let mut g = game(3);
        let d = &g.world;
        let (mut x, mut y, mut m) = (0.0, 0.0, 0.0);
        for (j, &(wx, wy, _)) in d.walkers.iter().enumerate() {
            if d.lives(j) {
                (x, y, m) = (x + wx, y + wy, m + 1.0);
            }
        }
        let p = (x / m, y / m);
        let r = RoundRules::default();
        let fine = g.rules.tps as f32 * g.world.p.substeps as f32;
        let c = casts(&[Card::Bomb])[0];
        let k = BurstRules { fling_speed: r.fling_speed, ..burst_rules(&r.kit, fine) };
        let b = bursts(&c, p, &r, g.world.p.scale as f32, fine);
        assert_eq!(b.len(), 1);
        let living = World::living(&g.world);
        let from = g.world.walkers.clone();
        let out = g.world.burst(&b[0], &k);
        let flung: Vec<usize> = (0..from.len()).filter(|&j| g.world.flung.get(j) == Some(&true)).collect();
        assert!(out.killed > 10 && out.flung as usize == flung.len() && flung.len() > 50, "killed {}, flung {}", out.killed, out.flung);
        assert!(flung.iter().all(|&j| g.world.life[j] == LIFE_SPORE), "the flung are in flight");
        let dist = |q: (f32, f32, f32)| (q.0 - p.0).hypot(q.1 - p.1);
        let ring = b[0].fling * g.world.p.scale as f32;
        // Their flight, then a few ticks to land on a free cell.
        for _ in 0..(r.bomb_secs * g.rules.tps as f32) as usize + 6 {
            g.step();
        }
        let landed = flung.iter().filter(|&&j| !g.world.flung[j] && g.world.life[j] == LIFE_WALKER).count();
        let out_far = flung.iter().filter(|&&j| dist(g.world.walkers[j]) > dist(from[j]) + 1.0).count();
        let beyond = flung.iter().filter(|&&j| g.world.lives(j) && dist(g.world.walkers[j]) > ring).count();
        eprintln!("bomb: killed {}, flung {}, landed {landed}, {out_far} further out, {beyond} beyond the ring; living {living} then {}", out.killed, flung.len(), World::living(&g.world));
        assert!(landed * 10 >= flung.len() * 9, "{landed} of {} landed", flung.len());
        assert!(out_far * 10 >= flung.len() * 8, "{out_far} of {} thrown outward", flung.len());
        assert!(beyond * 4 >= flung.len(), "{beyond} of {} landed beyond the ring", flung.len());
    }

    /// No recharge (owner 18:05Z: "It has a rate of fire. It never has to
    /// stop to recharge or switch to regular bullets"): every shot a
    /// soldier fires carries the next cast in turn, a scattergun's whole
    /// fan the same one, and every slime hit casts what its bullet
    /// carries. Lodged casts go off later.
    #[test]
    fn every_shot_fires_the_next_card_in_turn_and_the_gun_never_recharges() {
        use crate::mould::guns::GunKind;
        // On the attack, so the squad meets the mould whatever its shots'
        // timing (with bullets, 11-13, Defend on seed 2 hit only the thin
        // edge: ten casts, none killing).
        let attack = |g: &mut Game<BigDish>| {
            g.bank = 1.0e9;
            for k in 0..g.soldiers.len() {
                g.apply(Cmd::Order(k, Order::Attack));
            }
        };
        for scatter in [false, true] {
            let mut g = game(2);
            attack(&mut g);
            if scatter {
                g.apply(Cmd::Arm(0, Some(GunKind::Scatter.average())));
            }
            let mut rd = Rounds::new(parse("blast+wide+gas").unwrap(), RoundRules::default());
            rd.arm(&mut g);
            // Each soldier's shots, a frame at a time, by the cast they carry.
            let mut seq: Vec<Vec<u8>> = vec![Vec::new(); g.soldiers.len()];
            let mut hits = [0u32; 2];
            let (mut legs, mut fans) = (0, 0);
            for _ in 0..15 * 30 {
                g.step();
                rd.after_step(&mut g);
                let mut now: Vec<Vec<u8>> = vec![Vec::new(); g.soldiers.len()];
                for s in &g.last_shots {
                    if let (Some(k), true) = (s.by, s.fired) {
                        now[k].push(s.cast);
                    }
                    if s.hit.is_some() && s.by.is_some() && s.cast > 0 {
                        hits[s.cast as usize - 1] += 1;
                    }
                }
                for (k, v) in now.into_iter().enumerate().filter(|(_, v)| !v.is_empty()) {
                    assert!(v.iter().all(|&c| c == v[0]), "soldier {k}'s fan carries one cast: {v:?}");
                    legs += v.len();
                    fans += 1;
                    seq[k].push(v[0]);
                }
            }
            assert!(fans > 0 && rd.cast[0] > 0 && rd.cast[1] + rd.unsupported > 0, "cast {:?}", rd.cast);
            if scatter {
                assert!(legs >= 3 * fans, "the scattergun's fan: {legs} bullets in {fans} shots");
            }
            for (k, v) in seq.iter().enumerate() {
                assert!(v.iter().all(|&c| c == 1 || c == 2), "soldier {k}: never a plain shot: {v:?}");
                assert!(v.windows(2).all(|w| w[1] == 3 - w[0]), "soldier {k}: blast, gas, blast, ...: {v:?}");
            }
            // Every hit cast what its bullet carried, recharge or none.
            assert_eq!(rd.cast[0], hits[0], "a blast on every blast bullet's hit");
            assert_eq!(rd.cast[1] + rd.unsupported, hits[1]);
            assert!(rd.killed > 0, "cast {:?} pops {:?}", rd.cast, &rd.pops[..rd.pops.len().min(3)]);
            // The old recharge (2 s, plus 1 a payload and 1 for Wide) let
            // a soldier cast at most 2 a 5 s, 12 in these 30 s.
            if !scatter {
                assert!(rd.cast[0] + rd.cast[1] > 12 * g.soldiers.len() as u32 / 2, "{:?} casts from {} soldiers", rd.cast, g.soldiers.len());
            }
            eprintln!("scatter {scatter}: {fans} shots, {legs} bullets, cast {:?}, killed {}", rd.cast, rd.killed);
        }
        // A lodged cast goes off later, every one a blast.
        let mut h = game(2);
        attack(&mut h);
        let mut lodged = Rounds::new(parse("delay+blast").unwrap(), RoundRules::default());
        for _ in 0..15 * 30 {
            h.step();
            lodged.after_step(&mut h);
        }
        assert!(lodged.cast[0] > 0 && lodged.pops.iter().all(|p| p.payload == Card::Blast));
        assert!(lodged.pops.len() as u32 + lodged.waiting() as u32 + lodged.dropped >= lodged.cast[0]);
        // Firing is free (owner 18:38Z: "The bank is just to build the
        // gun"): an empty bank still blasts, and the round spends none.
        let mut p = game(2);
        attack(&mut p);
        p.bank = 0.0;
        let mut free = Rounds::new(vec![Card::Blast], RoundRules::default());
        for _ in 0..15 * 10 {
            p.step();
            free.after_step(&mut p);
        }
        assert!(free.cast[0] > 0, "an empty bank blasts");
        assert_eq!(p.bank, 0.0, "and pays nothing for it");
    }
}
