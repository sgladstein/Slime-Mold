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
//!   `Germ` (contagion). Each is paid from the bank at the toolkit's price
//!   when it is cast. The reviewers' call (game designer and technical
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
//!   worlds fly them alike (`Game::landed`). Like the others they weigh
//!   `card_weight` on the recharge and fill a slot.
//!
//! **How a round fires** (Noita's deck): the cards are read left to right
//! into casts, each a run of modifiers and at most one trigger ending in a
//! payload; cards after the last payload do nothing. Each of a soldier's
//! shots that hits slime casts the next cast at the hit. After the last
//! cast the gun recharges for `RoundRules::recharge` plus each card's
//! weight (`Card::weight`) before the next cast; the shots meanwhile are
//! plain. The fire rate is untouched (shots stay free, owner 11-12): the
//! budget is the recharge and the four slots, not food per shot.
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
/// Most cards a round holds.
pub const SLOTS: usize = 4;

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
    /// budget is a capacity checked when the gun is loaded, and equipped
    /// cards fire on the recharge as before). A payload is the most, a
    /// card that changes every bullet next, a tweak least.
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

    /// Game seconds the card adds to its round's recharge.
    pub fn weight(self, r: &RoundRules) -> f32 {
        match self {
            Card::Wide => r.wide_weight,
            k if k.kind() == Kind::Payload => r.payload_weight,
            _ => r.card_weight,
        }
    }
}

/// A round written as `blast` or `quiet+gas+germ`.
pub fn parse(s: &str) -> Option<Vec<Card>> {
    if s == "none" || s.is_empty() {
        return Some(Vec::new());
    }
    let v: Option<Vec<Card>> = s.split('+').map(Card::from_name).collect();
    v.filter(|v| v.len() <= SLOTS)
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
/// most `SLOTS` of them.
pub fn fit(round: Vec<Card>, budget: u32) -> Result<Vec<Card>, Over> {
    let c = cost(&round);
    if c <= budget && round.len() <= SLOTS {
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
    /// Game seconds of recharge after a round's last cast, before card
    /// weights.
    pub recharge: f32,
    pub payload_weight: f32,
    pub card_weight: f32,
    pub wide_weight: f32,
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
    /// `step` a fine step, its crater (radius, seconds) and its price.
    /// At the race world's 30 fine steps a second and step 1, 0.5 s at 1.5
    /// throws the inner edge 22 fine cells, 11 dish cells.
    pub bomb_r: f32,
    pub bomb_ring: f32,
    pub bomb_secs: f32,
    pub fling_speed: f32,
    pub bomb_crater_r: f32,
    pub bomb_crater_secs: f32,
    pub bomb_price: f32,
    /// **Trails** (the flight cards `Scorch`, `Bait`, `Mist`, 2026-11-14,
    /// after Noita's burning and gunpowder trails: step 5 of
    /// `Reports/noita-spell-look-2026-11-14.md`): each soldier bullet's leg
    /// a frame lays bursts every `trail_r` along it, radius `trail_r` dish
    /// cells (x the square root of the copies of the card): crater for
    /// `trail_crater_secs`, food scent `trail_bait` x a soldier's frame of
    /// smell, or `trail_cloud` of gas cloud. Free: the cards weigh on the
    /// recharge.
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
            recharge: 2.0,
            payload_weight: 1.0,
            card_weight: 0.5,
            wide_weight: 1.0,
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
            bomb_price: 60.0,
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
    /// Payloads' bank price.
    pub price: f32,
}

/// The round's casts, left to right. Modifiers and triggers apply to the
/// next payload; a second trigger before it replaces the first.
pub fn casts(round: &[Card], r: &RoundRules) -> Vec<Cast> {
    let mut out = Vec::new();
    let (mut wide, mut scatter, mut quiet, mut trigger) = (0, false, false, None);
    for &c in round.iter().take(SLOTS) {
        match c {
            Card::Wide => wide += 1,
            Card::Scatter => scatter = true,
            Card::Quiet => quiet = true,
            Card::Delay | Card::Thick => trigger = Some(c),
            // Flight cards ride on every bullet, not on a cast (`flight`).
            Card::Pierce | Card::Split | Card::Scorch | Card::Bait | Card::Mist | Card::Range | Card::Rate | Card::Smother | Card::Thrift => {}
            Card::Blast | Card::Bomb | Card::Gas | Card::Germ => {
                let price = match c {
                    Card::Blast => r.kit.blast_price,
                    Card::Bomb => r.bomb_price,
                    Card::Gas => r.kit.gas_price,
                    _ => r.kit.germ_price,
                };
                out.push(Cast { payload: c, wide, scatter, quiet, trigger, price });
                (wide, scatter, quiet, trigger) = (0, false, false, None);
            }
        }
    }
    out
}

/// The round's flight cards as every soldier bullet carries them: each
/// `pierce` one more slime hit flown through, each `split` one more
/// generation of pieces.
pub fn flight(round: &[Card]) -> BulletMods {
    let n = |k: Card| round.iter().take(SLOTS).filter(|&&c| c == k).count() as u8;
    BulletMods { pierce: n(Card::Pierce), split: n(Card::Split), reach: n(Card::Range), rate: n(Card::Rate) }
}

/// The round's trail cards as every soldier bullet lays them: copies of
/// `Scorch`, `Bait`, `Mist` and `Smother`.
pub fn trails(round: &[Card]) -> [u8; 4] {
    let n = |k: Card| round.iter().take(SLOTS).filter(|&&c| c == k).count() as u8;
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

/// Game seconds a round recharges after its last cast.
pub fn recharge(round: &[Card], r: &RoundRules) -> f32 {
    r.recharge + round.iter().take(SLOTS).map(|c| c.weight(r)).sum::<f32>()
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

#[derive(Clone, Copy, Debug, Default, PartialEq)]
struct Gun {
    next: usize,
    /// Tick the gun may cast again (after a recharge).
    ready: u64,
}

/// The squad's rounds: one loadout for every soldier (the game designer:
/// loadouts per squad, set at the larder or between missions, not per
/// soldier mid-fight), each soldier's gun state, and what was cast.
#[derive(Clone, Debug, Default)]
pub struct Rounds {
    pub rules: RoundRules,
    pub round: Vec<Card>,
    casts: Vec<Cast>,
    guns: Vec<Gun>,
    pending: Vec<Pending>,
    /// Casts by payload (blast, gas, germ, bomb), walkers killed by
    /// rounds, walkers flung by bombs, bank spent, casts dropped at
    /// `PENDING_MAX`, and casts skipped for want of food.
    pub cast: [u32; 4],
    pub killed: usize,
    pub flung: usize,
    pub spent: f32,
    pub dropped: u32,
    pub broke: u32,
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
        let casts = casts(&round, &rules);
        Rounds { rules, round, casts, ..Default::default() }
    }

    /// A gun with these cards if they fit `rules.budget` (`fit`): how a
    /// run equips the cards a player owns.
    pub fn build(round: Vec<Card>, rules: RoundRules) -> Result<Self, Over> {
        fit(round, rules.budget).map(|r| Rounds::new(r, rules))
    }

    /// Puts the round's flight cards on the game's soldier bullets (also
    /// done by every `after_step`, so a game rounds join late still flies
    /// them from the next shot).
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

    /// What soldier `k`'s next hit casts, and whether its gun is still
    /// recharging at tick `t` (a hit then casts nothing). None when the
    /// round has no payload.
    pub fn next_cast(&self, k: usize, t: u64) -> Option<(Cast, bool)> {
        let g = self.guns.get(k).copied().unwrap_or_default();
        self.casts.get(g.next).map(|&c| (c, g.ready > t))
    }

    /// Soldier `k`'s gun: the index (into `casts`) of the cast its next
    /// hit makes, and the tick its recharge ends (0 or past: ready).
    pub fn gun(&self, k: usize) -> Option<(usize, u64)> {
        let g = self.guns.get(k).copied().unwrap_or_default();
        (!self.casts.is_empty()).then_some((g.next, g.ready))
    }

    /// Lodged casts: what, where (fine cells), and the tick each goes off
    /// by.
    pub fn lodged(&self) -> impl Iterator<Item = (Cast, (f32, f32), u64)> + '_ {
        self.pending.iter().map(|p| (p.cast, p.at, p.due))
    }

    /// After each `Game::step`: each soldier's shot that hit slime this
    /// step casts its gun's next cast, and lodged casts that are due go
    /// off. A shot is a soldier's by `Shot::by` (turret shots have none).
    pub fn after_step<W: World>(&mut self, g: &mut Game<W>) {
        self.arm(g);
        self.lay_trails(g);
        let t = g.world.ticks();
        let tps = g.rules.tps;
        if !self.casts.is_empty() {
            if self.guns.len() < g.soldiers.len() {
                self.guns.resize(g.soldiers.len(), Gun::default());
            }
            // Each hit by the soldier that fired it (`Shot::by`; until the
            // shots flew as bullets, 11-13, matched by where it left from).
            let hits: Vec<(usize, (f32, f32))> = g.last_shots.iter().filter_map(|s| s.hit.zip(s.by)).map(|(h, k)| (k, h)).collect();
            for (k, hit) in hits {
                if k >= g.soldiers.len() || !g.soldiers[k].alive {
                    continue;
                }
                if self.guns[k].ready > t {
                    continue;
                }
                let c = self.casts[self.guns[k].next];
                if !castable(&g.world, c.payload) {
                    self.unsupported += 1;
                } else if g.bank < c.price {
                    self.broke += 1;
                    continue;
                } else {
                    g.bank -= c.price;
                    g.spent += c.price;
                    self.spent += c.price;
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
                let gun = &mut self.guns[k];
                gun.next += 1;
                if gun.next == self.casts.len() {
                    gun.next = 0;
                    gun.ready = t + (recharge(&self.round, &self.rules) * tps as f32).round() as u64;
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
        let mut n = trails(&self.round);
        if !g.world.has_gas() {
            n[2] = 0;
        }
        if n == [0; 4] {
            return;
        }
        let s = g.world.scale();
        let fine = g.rules.tps as f32 * g.world.fine_steps() as f32;
        let k = burst_rules(&self.rules.kit, fine);
        let smell = g.world.food_deposit() * g.rules.smell * g.rules.frame_ticks as f32;
        let mut burned = 0;
        let legs: Vec<((f32, f32), (f32, f32))> = g.last_shots.iter().filter(|s| s.by.is_some()).map(|s| (s.from, s.hit.unwrap_or(s.to))).collect();
        for (from, to) in legs {
            for b in trail_bursts(n, from, to, &self.rules, s, fine, smell) {
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
        assert!(casts(&round, &r).is_empty());
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
        assert_eq!((rd.cast, rd.spent), ([0; 4], 0.0), "a trail casts nothing and costs no food");
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

    #[test]
    fn flight_cards_ride_every_bullet_and_cast_nothing() {
        let r = RoundRules::default();
        let round = parse("pierce+split+split+blast").unwrap();
        assert_eq!(flight(&round), BulletMods { pierce: 1, split: 2, reach: 0, rate: 0 });
        assert_eq!(flight(&parse("range+range+pierce").unwrap()), BulletMods { pierce: 1, split: 0, reach: 2, rate: 0 });
        assert_eq!(casts(&round, &r).len(), 1);
        assert_eq!(casts(&parse("split").unwrap(), &r), vec![]);
        assert!((recharge(&parse("pierce").unwrap(), &r) - (r.recharge + r.card_weight)).abs() < 1e-6);
        let mut g = Game::new(BigDish::new(race_params(1, 1, 500)), Rules::default(), 1);
        Rounds::new(round, r).arm(&mut g);
        assert_eq!(g.bullet_mods, BulletMods { pierce: 1, split: 2, reach: 0, rate: 0 });
        assert_eq!(g.gun_range(), g.rules.range);
        Rounds::new(parse("range+blast").unwrap(), r).arm(&mut g);
        assert!((g.gun_range() - g.rules.range * (1.0 + g.rules.range_card)).abs() < 1e-4, "one Range card stretches the reach");
    }

    #[test]
    fn modifiers_and_triggers_go_to_the_next_payload() {
        let r = RoundRules::default();
        let round = parse("wide+blast+thick+gas").unwrap();
        let c = casts(&round, &r);
        assert_eq!(c.len(), 2);
        assert_eq!((c[0].payload, c[0].wide, c[0].trigger), (Card::Blast, 1, None));
        assert_eq!((c[1].payload, c[1].wide, c[1].trigger), (Card::Gas, 0, Some(Card::Thick)));
        // Cards after the last payload do nothing but still weigh.
        assert_eq!(casts(&parse("blast+quiet").unwrap(), &r).len(), 1);
        assert!(recharge(&parse("blast+quiet").unwrap(), &r) > recharge(&parse("blast").unwrap(), &r));
        assert!(parse("blast+blast+blast+blast+blast").is_none());
        assert_eq!(name(&round), "wide+blast+thick+gas");
    }

    #[test]
    fn scatter_splits_a_payload_and_quiet_lays_no_alarm() {
        let r = RoundRules::default();
        let c = casts(&parse("scatter+quiet+blast").unwrap(), &r)[0];
        let b = bursts(&c, (100.0, 100.0), &r, 2.0, 30.0);
        assert_eq!(b.len(), r.scatter_n);
        assert!(b.iter().all(|b| b.alarm == 0.0 && b.r < r.kit.blast_r && b.kill));
        let loud = bursts(&casts(&parse("blast").unwrap(), &r)[0], (100.0, 100.0), &r, 2.0, 30.0);
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
        let c = casts(&[Card::Blast], &r)[0];
        let k = burst_rules(&r.kit, fine);
        let killed: usize = bursts(&c, p, &r, b.world.p.scale as f32, fine).iter().map(|x| b.world.burst(x, &k).killed as usize).sum();
        assert_eq!(killed, kit.killed[0]);
        // The larder smells of the bank, so both pay the price.
        b.bank -= c.price;
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
        let c = casts(&[Card::Bomb], &r)[0];
        assert_eq!(c.price, r.bomb_price);
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

    #[test]
    fn rounds_cast_on_hits_and_recharge() {
        // Soldiers fire at the mould; a blast round casts on their hits,
        // then waits out its recharge.
        // On the attack, so the squad meets the mould whatever its shots'
        // timing (with bullets, 11-13, Defend on seed 2 hit only the thin
        // edge: ten casts, none killing).
        let attack = |g: &mut Game<BigDish>| {
            g.bank = 1.0e9;
            for k in 0..g.soldiers.len() {
                g.apply(Cmd::Order(k, Order::Attack));
            }
        };
        let mut g = game(2);
        attack(&mut g);
        let mut rd = Rounds::new(vec![Card::Blast], RoundRules::default());
        let mut lodged = Rounds::new(parse("delay+blast").unwrap(), RoundRules::default());
        let mut h = game(2);
        attack(&mut h);
        for _ in 0..15 * 60 {
            g.step();
            rd.after_step(&mut g);
            h.step();
            lodged.after_step(&mut h);
        }
        assert!(rd.cast[0] > 0, "no blast cast in 60 s");
        assert!(rd.killed > 0, "cast {:?} pops {:?}", rd.cast, &rd.pops[..rd.pops.len().min(3)]);
        // Recharge 3 s a cast: at most one cast per soldier per 3 s.
        assert!(rd.cast[0] as usize <= g.soldiers.len() * 41, "{} casts", rd.cast[0]);
        assert!(lodged.cast[0] > 0 && lodged.pops.iter().all(|p| p.payload == Card::Blast));
        // Every lodged cast went off at least `delay_secs` after it was cast.
        assert!(lodged.pops.len() as u32 + lodged.waiting() as u32 + lodged.dropped >= lodged.cast[0]);
    }
}
