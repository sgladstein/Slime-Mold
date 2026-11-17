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
//! gas or germ (`World::has_gas`, `has_germ`; the GPU world has both
//! since 11-14) those payloads are not cast and cost nothing,
//! counted in `Rounds::unsupported`. Soldiers only (turret rounds later).
//!
//! **Caps** (technical lead): at most `PENDING_MAX` lodged casts at once
//! (a new one past the cap is dropped and counted in `Rounds::dropped`),
//! and at most `scatter_n` bursts a cast, so one frame casts at most a
//! burst list of soldiers x `scatter_n`.
pub use super::bullets::{Ammo, Set, Shell};
use super::big;
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
    /// The standard plain bullet: its strike and nothing more (a payload,
    /// since the payloads are bullets, owner 18:05Z).
    Plain,
    /// A rolled bullet (`bullets`): a payload, a set, damage and speed,
    /// range and spread multipliers, and its own cost.
    Ammo(Ammo),
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
    /// **Trigger** (2026-11-16, step B of the gun-system review; the owner's
    /// 11-14 15:06Z notes, "trigger next card on hit", and 11-16 02:34Z: "I
    /// thought we were planning on building a trigger card"): the bullet of
    /// the cast it is in carries the next cast and releases it where it
    /// hits slime (`Cast::child`, `Game::landed`).
    Trigger,
    /// **On-expire** (2026-11-16, step B): as `Trigger`, and it releases
    /// where its bullet stops without a hit too (a wall, the end of its
    /// range).
    Expire,
    /// **The shove cards** (owner 2026-11-16 05:10Z: "The slug looks fun";
    /// 05:11Z: "There should be ways to modify the pushback. I could image
    /// a gun that lets you corral and push the slime onto areas. Into a
    /// kill zone"). Each gives the next bullet a shove if its set has none
    /// (`RoundRules::shove`; the slug's is its own, `bullets::Shape`).
    /// `Heave`: the shove reaches and throws `RoundRules::heave` times as
    /// far, a card.
    Heave,
    /// The shove pushes the slime toward the nearest turret within
    /// `RoundRules::corral_reach`, as a block (`squad::Burst::push`); with
    /// no turret there it shoves away from the gun as usual.
    Corral,
    /// The shove drags the slime toward the gun, as a block: into the
    /// squad's crossfire, and nearer its soldiers.
    Pull,
    /// **Multicast** (2026-11-16, step B of the gun-system review: "the
    /// next two or three groups as one volley"): the shot that fires the
    /// cast it is in fires the next cast too, side by side in the same
    /// shot (`Cast::with`); a second multicast in that cast adds a third.
    Multicast,
    /// **Seek** (2026-11-16, step C of the gun-system review, the owner's
    /// 02:34Z "homing"; homing in the shot trace, the owner's 19:38Z pick):
    /// a bullet that would pass beside slime strikes it. On the leg that
    /// passes its aim point, or its last, the shot trace catches live slime
    /// or walkers within `Rules::seek_catch` cells a card of its line
    /// (`Game::catches`, `squad::trace`), in both engines. The first tries
    /// turned the bullet toward slime ahead no bullet had picked
    /// (`Game::steer`, `Rules::seek_turn`; deleted 11-17): it read slime a
    /// second old and missed as often (Reports/gun-seek-2026-11-16/).
    Seek,
    /// **The Mind bullets** (2026-11-16, step D of the gun-system review,
    /// its first family: "Mind is the family only this game can have",
    /// Jones's patterns change with how walkers sense). Each is a payload
    /// that kills nothing: the walkers within `RoundRules::mind_r` of its
    /// hit sense differently for `RoundRules::mind_secs`
    /// (`squad::Burst::mind`, `BigDish::mind`), and the network's texture
    /// shows it. `Blind`: sensors read nothing but walls, so the slime
    /// walks straight, into water and fire, and turns only off walls.
    Blind,
    /// Trail-following flips (`big::MIND_SOUR`): the slime turns from its
    /// own trail, so the network frays into fog and knits back after.
    Sour,
    /// Curls `RoundRules::spin_curl` degrees more each fine step
    /// (`big::MIND_SPIN`): it circles in place and stops advancing, so a
    /// vein breaks into a chain of knots.
    Spin,
    /// **The readers** (2026-11-16, step D of the gun-system review, its
    /// second family, "readers and marks": a bullet that cashes in a state
    /// another put on the slime). Each is a payload that kills only the
    /// walkers in the state it reads (`squad::Burst::read`,
    /// `BigDish::read_strike`), so it pays only after another card's work.
    /// `Shatter`: the walkers asleep from gas within `RoundRules::read_r`.
    Shatter,
    /// The sick walkers (`big::READ_SICK`) within `RoundRules::detonate_r`,
    /// wider, as the germ's sickness spreads along the veins from its hit;
    /// the germ is this family's mark, so there is no mark card.
    Detonate,
    /// **The route family** (2026-11-17, step D of the gun-system review,
    /// its third: "trail, food scent, soldier scent, alarm; the slime
    /// reroutes"). `Flare` is a payload that kills nothing and lays the
    /// alarm a strike lays (`squad::Burst::alarm`), wider and stronger,
    /// and burns: it lays it again every `RoundRules::flare_every` for
    /// `flare_secs` (`Rounds::burning`). What the slime does with an alarm
    /// is its tier's own rule: a flinching mould (shape C on planets 1-3,
    /// `BigParams::flinch`) backs off the flare while it burns, the
    /// Swarmer floods to it, the Sleeper's crust wakes and rushes it (the
    /// tiers' decoy, `Reports/mould-tiers-2026-11-11.md`). One shot's
    /// alarm on a flinching mould is gone in a few fine steps, hence the
    /// burn.
    Flare,
    /// **The element family** (2026-11-17, step D of the gun-system
    /// review, its fourth: "fire, water, plants; the ground the slime
    /// stands on"). `Torch` is a payload that kills nothing itself and
    /// lights every plant within `RoundRules::torch_r` (`squad::Burst::
    /// light`, `BigDish::ignite`): the fire does the killing, on whatever
    /// thin slime stands in it or walks into it, and spreads through the
    /// plants by the ground's moisture, so a planet's biome sets what one
    /// torch does (a meadow or a jungle burns; barrens, fens and caves
    /// hardly). Living slime thick enough to be damp does not burn, so the
    /// torch eats an edge, not a body, and fire hurts soldiers who stand in
    /// it (`Rules::fire_hurt`).
    Torch,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Kind {
    Payload,
    Modifier,
    Trigger,
    Flight,
}

impl Card {
    /// The fixed cards (not `Ammo`, which is rolled).
    pub const ALL: [Card; 33] = [Card::Plain, Card::Blast, Card::Bomb, Card::Gas, Card::Germ, Card::Wide, Card::Scatter, Card::Quiet, Card::Delay, Card::Thick, Card::Pierce, Card::Split, Card::Scorch, Card::Bait, Card::Mist, Card::Range, Card::Rate, Card::Thrift, Card::Smother, Card::Trigger, Card::Expire, Card::Heave, Card::Corral, Card::Pull, Card::Multicast, Card::Seek, Card::Blind, Card::Sour, Card::Spin, Card::Shatter, Card::Detonate, Card::Flare, Card::Torch];

    pub fn kind(self) -> Kind {
        match self {
            Card::Plain | Card::Ammo(_) | Card::Blast | Card::Bomb | Card::Gas | Card::Germ | Card::Blind | Card::Sour | Card::Spin | Card::Shatter | Card::Detonate | Card::Flare | Card::Torch => Kind::Payload,
            Card::Wide | Card::Scatter | Card::Quiet | Card::Heave | Card::Corral | Card::Pull => Kind::Modifier,
            Card::Delay | Card::Thick | Card::Trigger | Card::Expire | Card::Multicast => Kind::Trigger,
            Card::Pierce | Card::Split | Card::Scorch | Card::Bait | Card::Mist | Card::Range | Card::Rate | Card::Smother | Card::Seek => Kind::Flight,
            Card::Thrift => Kind::Modifier,
        }
    }

    /// Its kind's name (a rolled bullet's is "ammo"; `title` and `word`
    /// give its own).
    pub fn name(self) -> &'static str {
        match self {
            Card::Plain => "plain",
            Card::Ammo(_) => "ammo",
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
            Card::Trigger => "trigger",
            Card::Expire => "expire",
            Card::Multicast => "multicast",
            Card::Seek => "seek",
            Card::Heave => "heave",
            Card::Corral => "corral",
            Card::Pull => "pull",
            Card::Blind => "blind",
            Card::Sour => "sour",
            Card::Spin => "spin",
            Card::Shatter => "shatter",
            Card::Detonate => "detonate",
            Card::Flare => "flare",
            Card::Torch => "torch",
        }
    }

    /// Its name to show: "Needle gas" for a rolled bullet, else its name.
    pub fn title(self) -> String {
        match self {
            Card::Ammo(a) => a.title(),
            c => {
                let n = c.name();
                n[..1].to_uppercase() + &n[1..]
            }
        }
    }

    /// Its text form in a round (`parse`, `name`): its name, or a rolled
    /// bullet's `Ammo::word`.
    pub fn word(self) -> String {
        match self {
            Card::Ammo(a) => a.word(),
            c => c.name().to_string(),
        }
    }

    /// The bullet it fires: a payload's standard one, a rolled bullet's
    /// own. None for a card that is no bullet.
    pub fn ammo(self) -> Option<Ammo> {
        match self {
            Card::Ammo(a) => Some(a),
            c => Shell::of(c).map(Ammo::standard),
        }
    }

    /// What the card does, in a few words (the shop and the HUD say its
    /// kind beside it, and the window's `card_face` puts in the numbers).
    /// The owner, 11-15 02:57Z: "The text for all the gun cards is too
    /// much. They should be simplified and clarified"; until then each
    /// line led with its kind's jargon (payload, modifier, trigger, flight).
    pub fn about(self) -> &'static str {
        match self {
            Card::Plain => "Just its hit.",
            Card::Ammo(a) => match a.set {
                Set::Standard => "Its own damage, speed, range and spread.",
                Set::Slug => "Its hit shoves the slime back.",
                Set::Needle => "Hits up to 3 times in a line.",
                Set::Buckshot => "Fires 4 small pellets in a fan.",
            },
            Card::Blast => "Kills slime where it hits and leaves a crater.",
            Card::Gas => "Puts slime where it hits to sleep.",
            Card::Germ => "Infects slime where it hits; the sickness spreads.",
            Card::Bomb => "Throws the slime it hits outward; kills little.",
            Card::Wide => "Its hit is wider.",
            Card::Scatter => "Its hit becomes three smaller ones.",
            Card::Quiet => "It doesn't alarm the slime.",
            Card::Delay => "It goes off a few seconds after landing.",
            Card::Thick => "It waits until slime crowds round it.",
            Card::Pierce => "Bullets go through one slime hit.",
            Card::Split => "Bullets split where they land.",
            Card::Scorch => "Bullets burn a line the slime avoids.",
            Card::Bait => "Bullets leave a scent line the slime follows.",
            Card::Mist => "Bullets leave a thin sleeping gas.",
            Card::Range => "Shoots twice as far.",
            Card::Rate => "Fires twice as often.",
            Card::Thrift => "The next card costs less budget.",
            Card::Smother => "Bullets wipe the scent slime follows to food.",
            Card::Trigger => "Carries the next bullet and lets it out where it hits.",
            Card::Expire => "Carries the next bullet and lets it out where it stops.",
            Card::Multicast => "Fires the next bullet with it, in the same shot.",
            Card::Seek => "Bullets that just miss strike slime beside them.",
            Card::Blind => "Blinds slime where it hits: it walks straight on.",
            Card::Sour => "Slime where it hits turns from its own trail.",
            Card::Spin => "Slime where it hits turns in tight rings.",
            Card::Shatter => "Kills sleeping slime where it hits.",
            // "Let it spread": the game designer, 11-17 08:22Z: detonate
            // late, once the germ's wave has spread, is the card's lesson (2 s
            // after the germ it cut the wave short: the body 19,590 at 40 s
            // against 17,628, `Reports/gun-readers-2026-11-17/`).
            Card::Detonate => "Kills sick slime where it hits; let it spread first.",
            Card::Flare => "Burns where it hits, alarming slime; kills none.",
            Card::Torch => "Sets the plants where it hits on fire.",
            Card::Heave => "Its hit shoves the slime, farther with each heave.",
            Card::Corral => "Its hit shoves the slime toward your nearest turret.",
            Card::Pull => "Its hit drags the slime toward the gun.",
        }
    }

    /// What the card takes of a gun's budget (`RoundRules::budget`; the
    /// owner's notes, 11-14 15:06Z: "Each card should have a cost and guns
    /// have x amount of bank to spend", and their card at 15:34Z: the
    /// budget is a capacity checked when the gun is loaded). A payload is
    /// the most, a card that changes every bullet next, a tweak least.
    pub fn cost(self) -> u32 {
        match self {
            Card::Ammo(a) => a.cost as u32,
            Card::Plain => 1,
            Card::Bomb | Card::Germ => 4,
            Card::Blast | Card::Gas | Card::Scorch | Card::Mist | Card::Multicast => 3,
            Card::Wide | Card::Scatter | Card::Pierce | Card::Split | Card::Bait | Card::Range | Card::Rate | Card::Smother | Card::Trigger | Card::Expire | Card::Heave | Card::Corral | Card::Seek => 2,
            // Kills nothing, as the gas (3) doesn't; a few seconds' status.
            Card::Blind | Card::Sour | Card::Spin => 2,
            // Kills only what another card set up, which paid for it.
            Card::Shatter | Card::Detonate => 2,
            // Kills nothing; moves the slime, as the Mind bullets do.
            Card::Flare => 2,
            // Kills only by the fire, which needs plants and spares a damp
            // body: below the blast (3), with the flare.
            Card::Torch => 2,
            Card::Thrift => 0,
            Card::Quiet | Card::Delay | Card::Thick | Card::Pull => 1,
        }
    }

    /// How rare the card is: 1 common, 2 uncommon, 3 rare. A run hands
    /// out higher tiers deeper in (the roguelike thread's rewards).
    pub fn tier(self) -> u8 {
        match self {
            Card::Ammo(a) => a.shell.card().tier(),
            Card::Bomb | Card::Germ | Card::Multicast => 3,
            Card::Scatter | Card::Thick | Card::Split | Card::Scorch | Card::Mist | Card::Thrift | Card::Smother | Card::Trigger | Card::Expire | Card::Heave | Card::Corral | Card::Pull | Card::Seek => 2,
            Card::Blind | Card::Sour | Card::Spin | Card::Shatter | Card::Flare | Card::Torch => 2,
            Card::Detonate => 3,
            _ => 1,
        }
    }

    pub fn from_name(s: &str) -> Option<Card> {
        Card::ALL.into_iter().find(|c| c.name() == s)
    }

    /// A card from its text form (`word`): a fixed card's name or a rolled
    /// bullet's `ammo/...`.
    pub fn from_word(s: &str) -> Option<Card> {
        Card::from_name(s).or_else(|| Ammo::from_word(s).map(Card::Ammo))
    }
}

/// A bullet's leg this step: from, to (its hit or the leg's end).
type Leg = ((f32, f32), (f32, f32));

/// A bullet that hit slime this step and carries a cast: the soldier
/// (None: a turret, 2026-11-16), the hit (fine cells), the cast (`Bullet::cast`), the bullet's
/// heading (a unit vector) and its strike radius (fine cells).
type Hit = (Option<usize>, (f32, f32), u8, (f32, f32), f32, Option<usize>);

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
    let v: Option<Vec<Card>> = s.split('+').map(Card::from_word).collect();
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
    round.iter().map(|c| c.word()).collect::<Vec<_>>().join("+")
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
    ///
    /// **Blast and bomb apart** (2026-11-16, the gun-system review's step
    /// A; the owner, 02:34Z: "two of them are just two different sized
    /// explosions"): the blast holds ground (it kills a 6-cell disc and its
    /// crater keeps the slime off for `KitRules::crater_secs`, 20 s), the
    /// bomb throws and kills little: a 2.5-cell kill, the ring out to the
    /// same 10 cells thrown for 0.7 s (the inner edge about 15 dish cells),
    /// and a 1 s crater, so the slime flows back. Until then the bomb killed
    /// 4 cells, threw for 0.5 s and held a 4-cell crater 6 s: the throw
    /// landed round the hole, which filled back in about a second
    /// (`Reports/explosions-2026-11-14/README.md`), and the two read as a
    /// big and a small hole.
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
    /// **The shove cards** (`Card::Heave`, `Corral`, `Pull`, 2026-11-16): a
    /// bullet whose set has no shove of its own shoves `shove` strike radii
    /// past its hit for `shove_secs` (the slug's own is 3 radii, 0.25 s);
    /// each `Heave` reaches and throws `heave` times as far; a corral
    /// pushes toward a turret within `corral_reach` dish cells of the hit.
    pub shove: f32,
    pub shove_secs: f32,
    pub heave: f32,
    pub corral_reach: f32,
    /// **Pull stops short** (owner 2026-11-16 19:40Z, on the card "Stop it
    /// short of the squad"): a pull's slime lands no nearer than this many
    /// dish cells to the shooter; a hit already that near pulls nothing.
    /// Dragged right onto the squad, pull+plain lost all 8 soldiers in
    /// 10 s (`Reports/gun-shove-2026-11-16/`).
    pub pull_short: f32,
    /// **The Mind bullets** (`Card::Blind` and on, 2026-11-16): their
    /// status's reach in dish cells (x the bullet's area, as the gas's),
    /// how long it lasts, and a spinning walker's curl in degrees a fine
    /// step (`BigParams::spin_curl`).
    /// `set=` keys (`RoundRules::set`) tune them by eye.
    pub mind_r: f32,
    pub mind_secs: f32,
    pub spin_curl: f32,
    /// **The readers** (`Card::Shatter`, `Detonate`, 2026-11-16): their
    /// reach in dish cells (x the bullet's area). Shatter's covers a gas
    /// hit's (`KitRules::gas_r`, 12); Detonate's is twice that, for the
    /// sickness that has spread from a germ's hit (5). `set=` keys.
    pub read_r: f32,
    pub detonate_r: f32,
    /// **The route family** (`Card::Flare`, 2026-11-17): a flare's alarm a
    /// lay (a blast's is the toolkit's `blast_alarm`, 50, and every shot's
    /// `squad::SHOT_ALARM`, 50) over `flare_r` dish cells (x the bullet's
    /// area), laid again every `flare_every` seconds for `flare_secs`.
    /// `set=` keys.
    pub flare_alarm: f32,
    pub flare_r: f32,
    pub flare_secs: f32,
    pub flare_every: f32,
    /// **The element family** (`Card::Torch`, 2026-11-17): a torch lights
    /// this share (`torch_light`, 1: all) of the cells with fuel within
    /// `torch_r` dish cells (x the bullet's area): a blast's crater (7)
    /// lights 0.12 out to a cell past it, a scorch line 0.4 of its disc.
    /// `set=` keys.
    pub torch_r: f32,
    pub torch_light: f32,
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
            bomb_r: 2.5,
            bomb_ring: 4.0,
            bomb_secs: 0.7,
            fling_speed: 1.5,
            bomb_crater_r: 2.5,
            bomb_crater_secs: 1.0,
            trail_r: 2.0,
            trail_crater_secs: 4.0,
            trail_bait: 40.0,
            trail_cloud: 0.4,
            trail_burn: true,
            shove: 2.0,
            shove_secs: 0.25,
            heave: 1.5,
            corral_reach: 40.0,
            pull_short: 6.0,
            mind_r: 8.0,
            mind_secs: 6.0,
            spin_curl: 35.0,
            read_r: 12.0,
            detonate_r: 24.0,
            flare_alarm: 100.0,
            flare_r: 16.0,
            flare_secs: 3.0,
            flare_every: 0.2,
            torch_r: 6.0,
            torch_light: 1.0,
            budget: 10,
            // The germ round's sick pay a sixth of the toolkit's upkeep
            // (10 against 60; 2026-11-17, the game designer's call, 05:40Z:
            // "retune it now. It's numbers, so it doesn't wait for the
            // balance pass"). At 60 one germ round took 48% of plain C's
            // walkers in 20 s (race world, seed 0, one cast on the Fork lobe
            // at 20 s, walkers at 40 s; 76% in runs that carried the GPU
            // world's middle-seeding bug, fixed in `GpuWorld`'s burst). The
            // planets' spread comes from each tier's `germ_resist` and
            // `germ_upkeep` (the strong planets' sick pay 6 times this, the
            // toolkit's 60): `tiers.rs`, shape C's in `larder_world::shapes`,
            // `Reports/gun-route-2026-11-17/`.
            kit: KitRules { sick_upkeep: 10.0, ..KitRules::default() },
        }
    }
}

impl RoundRules {
    /// The bursts' shared settings (`burst_rules`, the toolkit's) with
    /// the rounds' own: the bomb's fling speed, and the Mind bullets'
    /// length and settings (2026-11-16).
    pub fn burst_rules(&self, fine: f32) -> BurstRules {
        let mind_steps = (self.mind_secs * fine).round().clamp(1.0, u16::MAX as f32) as u16;
        BurstRules { fling_speed: self.fling_speed, mind_steps, spin_curl: self.spin_curl.to_radians(), ..burst_rules(&self.kit, fine) }
    }

    /// A rule by name (`larder-world set=NAME=V`, as `Rules::set`), false
    /// for none: the Mind bullets' and the readers' numbers, to tune by eye.
    pub fn set(&mut self, name: &str, v: f32) -> bool {
        let r = match name {
            "mind_r" => &mut self.mind_r,
            "mind_secs" => &mut self.mind_secs,
            "spin_curl" => &mut self.spin_curl,
            "read_r" => &mut self.read_r,
            "detonate_r" => &mut self.detonate_r,
            "flare_alarm" => &mut self.flare_alarm,
            "flare_r" => &mut self.flare_r,
            "flare_secs" => &mut self.flare_secs,
            "flare_every" => &mut self.flare_every,
            "torch_r" => &mut self.torch_r,
            "torch_light" => &mut self.torch_light,
            // The germ round's own contagion (2026-11-17), in its kit so
            // the toolkit's germ dish keeps `KitRules::default`'s
            // (the rounds' upkeep is 10, the toolkit's 60).
            "sick_secs" => &mut self.kit.sick_secs,
            "sick_upkeep" => &mut self.kit.sick_upkeep,
            "sick_catch" => &mut self.kit.sick_catch,
            _ => return false,
        };
        *r = v;
        true
    }
}

/// A payload with what its modifiers and trigger made of it.
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct Cast {
    /// What it casts where it hits: `Card::Plain` (nothing), `Blast`,
    /// `Bomb`, `Gas` or `Germ`, a rolled bullet's by its shell.
    pub payload: Card,
    /// The bullet that carries it: its damage and multipliers (a fixed
    /// card's is its standard bullet).
    pub ammo: Ammo,
    pub wide: u32,
    pub scatter: bool,
    pub quiet: bool,
    pub trigger: Option<Card>,
    /// The cast its bullet carries and lets out (`Card::Trigger`,
    /// `Card::Expire`): its index in the round's casts. A carried cast is
    /// fired only by its carrier, not in the gun's own turn (`cycle`).
    pub child: Option<u8>,
    /// Its bullet lets the child out where it stops without a hit too
    /// (`Card::Expire`).
    pub expire: bool,
    /// `Heave` cards on it, and the way its shove pushes (`Card::Corral`
    /// or `Card::Pull`; None: away from the gun). Any of them gives the
    /// bullet a shove (`Rounds::shove`).
    pub heave: u8,
    pub herd: Option<Card>,
    /// The cast fired beside it in the same shot (`Card::Multicast`): its
    /// index in the round's casts. Like a carried cast it leaves the gun's
    /// own turn (`cycle`); every shot of this cast fires it too.
    pub with: Option<u8>,
}

/// Most carriers in a chain: a carried cast may carry one more, and a
/// third carrier's cast is fired in turn instead (the technical lead's caps
/// in the plan: depth 2).
pub const CARRY_DEPTH: usize = 2;

/// The angle between the bullets a multicast shot fires side by side
/// (`Card::Multicast`), radians: about a cell apart at 12 cells, so each
/// finds its own slime and the eye sees two shots.
pub const VOLLEY_FAN: f32 = 0.08;

/// The round's casts, left to right. Modifiers and triggers apply to the
/// next payload; a second trigger before it replaces the first.
///
/// A `Trigger` or `Expire` makes its cast carry the next one (`Cast::child`),
/// up to `CARRY_DEPTH` deep; a carrier with no cast after it carries none.
/// A `Multicast` makes its cast fire the next one not yet taken beside it
/// (`Cast::with`), each copy one more; links of both kinds count toward
/// the same depth.
/// The turrets' casts (`Game::turret_deck`), left to right; none without
/// a deck, so plain turrets cast nothing.
pub fn turret_casts<W: World>(g: &Game<W>) -> Vec<Cast> {
    g.turret_deck.map(|d| casts(&d.cards())).unwrap_or_default()
}

pub fn casts(round: &[Card]) -> Vec<Cast> {
    let mut out: Vec<Cast> = Vec::new();
    let (mut wide, mut scatter, mut quiet, mut trigger, mut carry) = (0, false, false, None, None);
    let (mut heave, mut herd, mut multi) = (0, None, 0u8);
    let mut multis: Vec<u8> = Vec::new();
    for &c in round.iter().take(MAX_SLOTS) {
        match c {
            Card::Wide => wide += 1,
            Card::Scatter => scatter = true,
            Card::Quiet => quiet = true,
            Card::Delay | Card::Thick => trigger = Some(c),
            Card::Trigger | Card::Expire => carry = Some(c == Card::Expire || carry == Some(true)),
            Card::Heave => heave += 1,
            Card::Corral | Card::Pull => herd = Some(c),
            Card::Multicast => multi += 1,
            // Flight cards ride on every bullet, not on a cast (`flight`).
            Card::Pierce | Card::Split | Card::Scorch | Card::Bait | Card::Mist | Card::Range | Card::Rate | Card::Smother | Card::Thrift | Card::Seek => {}
            Card::Plain | Card::Ammo(_) | Card::Blast | Card::Bomb | Card::Gas | Card::Germ | Card::Blind | Card::Sour | Card::Spin | Card::Shatter | Card::Detonate | Card::Flare | Card::Torch => {
                let ammo = c.ammo().unwrap_or_default();
                out.push(Cast { payload: ammo.shell.card(), ammo, wide, scatter, quiet, trigger, child: None, expire: carry == Some(true), heave, herd, with: None });
                multis.push(multi);
                // Marks a carrier for now: linked below.
                if carry.is_some() {
                    out.last_mut().unwrap().child = Some(u8::MAX);
                }
                (wide, scatter, quiet, trigger, carry) = (0, false, false, None, None);
                (heave, herd, multi) = (0, None, 0);
            }
        }
    }
    // Each carrier takes the cast after it, and each multicast the next
    // cast not yet taken (one more a copy, each fired beside the last),
    // unless that would make a chain deeper than `CARRY_DEPTH`.
    let mut depth = vec![0usize; out.len()];
    let mut taken = vec![false; out.len()];
    for i in 0..out.len() {
        if out[i].child.is_some() {
            if i + 1 < out.len() && !taken[i + 1] && depth[i] < CARRY_DEPTH {
                out[i].child = Some((i + 1) as u8);
                depth[i + 1] = depth[i] + 1;
                taken[i + 1] = true;
            } else {
                (out[i].child, out[i].expire) = (None, false);
            }
        }
        let mut at = i;
        for _ in 0..multis[i] {
            let Some(j) = (i + 1..out.len()).find(|&j| !taken[j]) else { break };
            if depth[at] >= CARRY_DEPTH {
                break;
            }
            out[at].with = Some(j as u8);
            depth[j] = depth[at] + 1;
            taken[j] = true;
            at = j;
        }
    }
    out
}

/// The casts one shot of cast `i` fires (indices in `casts`): `i` and the
/// casts beside it (`Cast::with`), in order.
pub fn volley(casts: &[Cast], i: usize) -> Vec<usize> {
    let mut v = vec![i];
    while let Some(j) = casts.get(*v.last().unwrap()).and_then(|c| c.with) {
        if v.contains(&(j as usize)) {
            break;
        }
        v.push(j as usize);
    }
    v
}

/// The casts a gun fires in its own turn, left to right, as cast ids (1 +
/// the index in `casts`): every cast no other carries.
pub fn cycle(casts: &[Cast]) -> Vec<u8> {
    let carried: Vec<u8> = casts.iter().flat_map(|c| c.child.into_iter().chain(c.with)).collect();
    (0..casts.len() as u8).filter(|i| !carried.contains(i)).map(|i| i + 1).collect()
}

/// The round's flight cards as every soldier bullet carries them: each
/// `pierce` one more slime hit flown through, each `split` one more
/// generation of pieces; and its casts, one a payload, which its shots
/// carry in turn.
pub fn flight(round: &[Card]) -> BulletMods {
    let cards = || round.iter().take(MAX_SLOTS);
    let n = |k: Card| cards().filter(|&&c| c == k).count() as u8;
    let all = casts(round);
    let mut m = BulletMods { pierce: n(Card::Pierce), split: n(Card::Split), reach: n(Card::Range), rate: n(Card::Rate), seek: n(Card::Seek), ..BulletMods::default() };
    for (i, c) in all.iter().enumerate() {
        m.ammo[i] = c.ammo;
        m.child[i] = c.child.map_or(0, |j| j + 1);
        m.expire[i] = c.expire;
        m.with[i] = c.with.map_or(0, |j| j + 1);
    }
    let turn = cycle(&all);
    m.casts = turn.len() as u8;
    m.cycle[..turn.len()].copy_from_slice(&turn);
    m
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
    if c.payload == Card::Plain {
        return Vec::new();
    }
    let k = &r.kit;
    // Wide, the bullet's damage (`Ammo::reach`: area as damage), and its
    // set's share (a buckshot pellet's, `bullets::Shape::r`).
    let f = r.wide.powi(c.wide as i32) * c.ammo.reach() * c.ammo.set.shape().r;
    let steps = |secs: f32| (secs * fine).round().clamp(0.0, u16::MAX as f32) as u16;
    let at = (at.0 / scale, at.1 / scale);
    let alarm = if c.quiet { 0.0 } else { k.blast_alarm };
    let none = Burst { at, r: 0.0, kill: false, burn: false, alarm: 0.0, alarm_r: 0.0, crater: 0, crater_r: 0.0, gas: 0, germ: false, bait: 0.0, smother: false, fling: 0.0, fling_steps: 0, push: None, cloud: 0.0, mind: 0, read: 0, light: 0.0 };
    let mind = |m: u32| Burst { r: r.mind_r * f, mind: m as u8, ..none };
    let one = match c.payload {
        Card::Blast => Burst { r: k.blast_r * f, kill: true, alarm, alarm_r: 2.0 * k.blast_r * f, crater: steps(k.crater_secs), crater_r: k.crater_r * f, ..none },
        Card::Bomb => {
            let br = r.bomb_r * f;
            Burst { r: br, kill: true, alarm, alarm_r: 2.0 * br, crater: steps(r.bomb_crater_secs), crater_r: r.bomb_crater_r * f, fling: br * r.bomb_ring, fling_steps: steps(r.bomb_secs).max(1), ..none }
        }
        Card::Gas => Burst { r: k.gas_r * f, gas: steps(k.gas_secs), ..none },
        Card::Germ => Burst { r: k.germ_r * f, germ: true, ..none },
        Card::Blind => mind(big::MIND_BLIND),
        Card::Sour => mind(big::MIND_SOUR),
        Card::Spin => mind(big::MIND_SPIN),
        Card::Shatter => Burst { r: r.read_r * f, read: big::READ_SLEEP, ..none },
        Card::Detonate => Burst { r: r.detonate_r * f, read: big::READ_SICK, ..none },
        // No kill (`r` 0): the alarm alone, unless Quiet took it.
        Card::Flare => Burst { alarm: if c.quiet { 0.0 } else { r.flare_alarm }, alarm_r: r.flare_r * f, ..none },
        // No kill and no alarm: the fire alone.
        Card::Torch => Burst { r: r.torch_r * f, light: r.torch_light, ..none },
        _ => return Vec::new(),
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
        mind_steps: 0,
        spin_curl: 0.0,
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
    /// The heading of the bullet that lodged it (a bomb throws away
    /// from the gun, `Rounds::fire`).
    dir: (f32, f32),
    /// Tick it goes off by (Delay) or gives up waiting and goes off (Thick).
    due: u64,
    /// A turret's: its kills are the turrets' (`Game::turret_killed`).
    turret: bool,
}

/// A burning flare (`Card::Flare`, 2026-11-17): its alarm burst, the tick
/// it lays it again and the tick it goes out.
#[derive(Clone, Copy, Debug, PartialEq)]
struct Burn {
    burst: Burst,
    next: u64,
    out: u64,
}

/// One burst as the window or a page draws it.
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct Pop {
    pub burst: Burst,
    /// A bomb's throw as its own burst, centred back toward the gun
    /// (`Rounds::fire`, 2026-11-16); `burst` is then its kill and crater
    /// alone. None: `burst` was the whole of it.
    pub throw: Option<Burst>,
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
    /// Flares still burning (`Card::Flare`), at most `PENDING_MAX`.
    burning: Vec<Burn>,
    /// Casts by payload (blast, gas, germ, bomb, the Mind bullets
    /// together and the readers together, since 2026-11-16, the flare and
    /// the torch, 11-17), walkers killed by rounds, walkers flung by bombs,
    /// and casts dropped at `PENDING_MAX` (lodged casts and burning flares
    /// each).
    pub cast: [u32; 8],
    pub killed: usize,
    pub flung: usize,
    pub dropped: u32,
    /// Casts skipped because this world has no gas or germ yet.
    pub unsupported: u32,
    /// Trail bursts laid (`trail_bursts`), all kinds.
    pub trailed: u32,
    /// Walkers the `Scorch` trail burned (in `killed` too).
    pub burned: usize,
    /// Slug shoves (`bullets::Shape::shove`) and the walkers they threw
    /// (in `flung` too).
    pub shoves: u32,
    pub shoved: usize,
    /// Of `shoves`, those that pushed one way (`Card::Corral`, `Pull`).
    pub pushes: u32,
    /// Pulls that hit within `RoundRules::pull_short` of the shooter (and a
    /// reach) and so pulled nothing.
    pub short: u32,
    /// Walkers put to sleep by rounds' gas (`squad::BurstOut::gassed`).
    pub gassed: usize,
    pub pops: Vec<Pop>,
    /// This step's payload and shove bursts, run as one batch at its end
    /// (`World::bursts`; `Rounds::flush`), with what each counts for.
    queued: Vec<(Burst, BurstRules, Due)>,
    /// Of `queued`, those a turret's cast laid (`Game::turret_killed`).
    queued_turret: Vec<bool>,
    /// The casts going off now are a turret's (`after_step`), so `fire`
    /// marks their bursts in `queued_turret`.
    by_turret: bool,
    /// Batches run (`flush`) and the bursts in them.
    pub batches: u32,
    pub batched: u32,
}

/// What a queued burst's counts go to: its pop (`Rounds::pops`), or a
/// shove.
#[derive(Clone, Copy, Debug, PartialEq)]
enum Due {
    Pop(usize),
    Shove,
    /// A burning flare's lay again: it kills nothing, so counts nothing.
    Burn,
}

/// A payload's place in `Rounds::cast`; plain bullets are not counted.
fn slot(p: Card) -> Option<usize> {
    match p {
        Card::Blast => Some(0),
        Card::Gas => Some(1),
        Card::Germ => Some(2),
        Card::Bomb => Some(3),
        Card::Blind | Card::Sour | Card::Spin => Some(4),
        Card::Shatter | Card::Detonate => Some(5),
        Card::Flare => Some(6),
        Card::Torch => Some(7),
        _ => None,
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

    /// Flares burning: where (dish cells), their reach (dish cells) and
    /// the ticks left.
    pub fn burning(&self, now: u64) -> impl Iterator<Item = ((f32, f32), f32, u64)> + '_ {
        self.burning.iter().map(move |b| (b.burst.at, b.burst.alarm_r, b.out.saturating_sub(now)))
    }

    /// Each burning flare lays its alarm again when it is due
    /// (`RoundRules::flare_every`), into this step's batch, and goes out
    /// at its end (`flare_secs`).
    fn burn<W: World>(&mut self, g: &mut Game<W>) {
        if self.burning.is_empty() {
            return;
        }
        let t = g.world.ticks();
        let every = ((self.rules.flare_every * g.rules.tps as f32).round() as u64).max(1);
        let k = self.rules.burst_rules(g.rules.tps as f32 * g.world.fine_steps() as f32);
        let mut lays = Vec::new();
        self.burning.retain_mut(|b| {
            if t >= b.out {
                return false;
            }
            if t >= b.next {
                lays.push(b.burst);
                b.next += every;
            }
            true
        });
        for b in lays {
            self.queued.push((b, k, Due::Burn));
            self.queued_turret.push(false);
        }
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
    /// The turrets' are `turret_casts`.
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
        let m = g.mods(s.squad as usize);
        let n = m.casts as usize;
        (n > 0).then(|| m.cycle[s.next as usize % n] as usize - 1)
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
        let hits: Vec<Hit> = g
            .last_shots
            .iter()
            .filter(|s| s.cast > 0)
            .filter_map(|s| {
                let (dx, dy) = (s.to.0 - s.from.0, s.to.1 - s.from.1);
                let d = dx.hypot(dy).max(1e-3);
                s.hit.map(|h| (s.by, h, s.cast, (dx / d, dy / d), s.r, s.turret))
            })
            .collect();
        // A turret's shot (`by` None) casts from the turrets' deck
        // (`Game::turret_deck`, 2026-11-16), in the slot past the squads'.
        by_squad.push(None);
        for (by, hit, cast, dir, hit_r, turret) in hits {
            // A bullet flies on when its soldier falls: it still casts. Its
            // gun is where a Pull drags toward: the soldier, or the turret
            // that fired it (`Shot::turret`).
            let (q, gun) = match by {
                Some(k) => match g.soldiers.get(k) {
                    Some(s) => (s.squad as usize, Some((s.x, s.y))),
                    None => continue,
                },
                None => (g.squads.len(), turret.and_then(|t| g.buildings.get(t)).map(|b| b.at)),
            };
            self.by_turret = by.is_none();
            if q >= by_squad.len() {
                continue;
            }
            let cs = by_squad[q].get_or_insert_with(|| if by.is_none() { turret_casts(g) } else { self.casts_of(g, q) });
            let Some(&c) = cs.get(cast as usize - 1) else { continue };
            // A slug shoves where it hits, whatever it carries, and so does
            // any bullet with a shove card.
            let shape = c.ammo.set.shape();
            if shape.shove > 0.0 || c.heave > 0 || c.herd.is_some() {
                self.shove(g, hit, dir, hit_r, shape, &c, gun);
            }
            // A plain bullet's strike was all it had (`Game::step`).
            let Some(sl) = slot(c.payload) else { continue };
            if !castable(&g.world, c.payload) {
                self.unsupported += 1;
            } else {
                self.cast[sl] += 1;
                match c.trigger {
                    None => self.fire(g, &c, hit, Some(dir)),
                    Some(tr) => {
                        if self.pending.len() >= PENDING_MAX {
                            self.dropped += 1;
                        } else {
                            let secs = if tr == Card::Delay { self.rules.delay_secs } else { self.rules.thick_secs };
                            self.pending.push(Pending { cast: c, at: hit, dir, due: t + (secs * tps as f32).round() as u64, turret: by.is_none() });
                        }
                    }
                }
            }
        }
        self.by_turret = false;
        self.burn(g);
        self.flush(g);
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
            self.by_turret = p.turret;
            self.fire(g, &p.cast, p.at, Some(p.dir));
            self.by_turret = false;
        }
        self.flush(g);
    }

    /// Runs the step's queued bursts as one batch (`World::bursts`: on the
    /// GPU one read-back for them all, 2026-11-16) and adds up what each
    /// did, to its pop or the shoves. Bursts in a step never read each
    /// other's counts, so batching moves no walker: the same bursts run in
    /// the same order.
    fn flush<W: World>(&mut self, g: &mut Game<W>) {
        if self.queued.is_empty() {
            return;
        }
        let q = std::mem::take(&mut self.queued);
        let tur = std::mem::take(&mut self.queued_turret);
        let list: Vec<(Burst, BurstRules)> = q.iter().map(|&(b, k, _)| (b, k)).collect();
        let outs = g.world.bursts(&list);
        self.batches += 1;
        self.batched += q.len() as u32;
        let sc = g.world.scale();
        let mut kills = Vec::new();
        for (j, ((b, _, due), out)) in q.iter().zip(outs).enumerate() {
            let (killed, flung) = (out.killed as usize, out.flung as usize);
            // A turret's cast kills are the turrets' (they were counted as
            // the soldiers' until the independent review, 2026-11-16), and
            // every cast's kills in a rendering turret's reach pay as a
            // shot's do (`Game::render_pay`, "whoever fired").
            if tur.get(j).copied().unwrap_or(false) {
                g.turret_killed += out.killed;
            }
            if out.killed > 0 {
                kills.push((Some((b.at.0 * sc, b.at.1 * sc)), out.killed));
            }
            self.killed += killed;
            self.flung += flung;
            self.gassed += out.gassed as usize;
            g.killed += out.killed;
            match *due {
                Due::Pop(i) => {
                    self.pops[i].killed += killed;
                    self.pops[i].flung += flung;
                }
                Due::Shove => self.shoved += flung,
                Due::Burn => {}
            }
        }
        g.render_pay(&kills);
    }

    /// Every cast of the round at once at `at` (fine cells), free and with
    /// no trigger: a staged scene for a film or a test (`larder-world
    /// boom=`), the same bursts a hit there would cast.
    pub fn cast_at<W: World>(&mut self, g: &mut Game<W>, at: (f32, f32)) {
        // The first squad's deck when it has one (`load=`, so a paired
        // film can boom the cards it loads; 2026-11-16), else the round.
        let cs = self.casts_of(g, 0);
        self.cast_all(g, &cs, at);
    }

    /// Every cast of `cards` at once at `at`, as `cast_at` the round's:
    /// `boom=T,X,Y,CARDS`, so a film can set a state off with one card and
    /// read it with another seconds later (the readers, 2026-11-16).
    pub fn cast_cards_at<W: World>(&mut self, g: &mut Game<W>, at: (f32, f32), cards: &[Card]) {
        self.cast_all(g, &casts(cards), at);
    }

    fn cast_all<W: World>(&mut self, g: &mut Game<W>, cs: &[Cast], at: (f32, f32)) {
        for c in cs {
            if let Some(sl) = slot(c.payload) {
                self.cast[sl] += 1;
                self.fire(g, c, at, None);
            }
        }
        self.flush(g);
    }

    /// Every soldier bullet's leg this step lays the round's trails
    /// (`trail_bursts`), up to its hit. A gas trail needs a world with gas.
    fn lay_trails<W: World>(&mut self, g: &mut Game<W>) {
        // Each leg lays its squad's trails (`cards_of`).
        let gas = g.world.has_gas();
        // The turrets' trails (`Game::turret_deck`) in the slot past the
        // squads'; with no deck they lay none, as before.
        let tq = g.squads.len();
        let mut of: Vec<Option<[u8; 4]>> = vec![None; tq + 1];
        let mut any = [0u8; 4];
        let legs: Vec<(Leg, [u8; 4])> = g
            .last_shots
            .iter()
            .map(|s| (s, s.by.map_or(tq, |k| g.soldiers.get(k).map_or(0, |x| x.squad as usize))))
            .map(|(s, q)| {
                let n = *of[q].get_or_insert_with(|| {
                    let mut n = trails(&if q == tq { g.turret_deck.map(|d| d.cards()).unwrap_or_default() } else { self.cards_of(g, q) });
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
        // The GPU's burns (the CPU counted its own above); the plants'
        // fire's kills were taken in `finish_step`, so none are here.
        if n[0] > 0 && self.rules.trail_burn {
            burned += g.world.burned() as usize;
        }
        self.killed += burned;
        self.burned += burned;
        g.killed += burned as u32;
    }

    /// A slug's shove (`bullets::Shape::shove`) where a bullet heading
    /// `dir` struck slime at `hit` with radius `r` (fine cells): a fling
    /// with no kill, centred a strike radius back toward the gun, so its
    /// ring lies round and past the hole and throws every walker in it away
    /// from the gun, those at the hole furthest. Centred on the hit, the
    /// ring's near half threw slime at the shooter.
    ///
    /// **The shove cards** (2026-11-16): a set with no shove shoves
    /// `RoundRules::shove` radii; each `Heave` multiplies the reach and the
    /// throw by `RoundRules::heave`. A card's shove is a push
    /// (`squad::Burst::push`): every walker in the disc of that reach round
    /// the hit flies one heading: straight on along the bullet's line, or
    /// with `Corral` toward the nearest turret within `corral_reach` of the
    /// hit (none: straight on), with `Pull` toward the gun that fired (`gun`: the soldier, or the turret). A slug
    /// with no `Corral` or `Pull` keeps its own throw out from behind the
    /// hit (step A), farther with each `Heave`.
    #[allow(clippy::too_many_arguments)]
    fn shove<W: World>(&mut self, g: &mut Game<W>, hit: (f32, f32), dir: (f32, f32), r: f32, shape: super::bullets::Shape, c: &Cast, gun: Option<(f32, f32)>) {
        let s = g.world.scale();
        let fine = g.rules.tps as f32 * g.world.fine_steps() as f32;
        let kr = self.rules.burst_rules(fine);
        let rr = &self.rules;
        let (shove, secs) = if shape.shove > 0.0 { (shape.shove, shape.shove_secs) } else { (rr.shove, rr.shove_secs) };
        let h = rr.heave.powi(c.heave as i32);
        let steps = (secs * h * fine).round().clamp(1.0, u16::MAX as f32) as u16;
        let reach = (1.0 + shove * h) * r;
        let toward = match c.herd {
            Some(Card::Pull) => gun,
            Some(Card::Corral) => {
                let far = rr.corral_reach * s;
                g.buildings.iter().filter(|b| b.up() && b.kind == super::larder_world::Kind::Turret).map(|b| b.at).filter(|t| (t.0 - hit.0).hypot(t.1 - hit.1) <= far).min_by(|a, b| (a.0 - hit.0).hypot(a.1 - hit.1).total_cmp(&(b.0 - hit.0).hypot(b.1 - hit.1)))
            }
            _ => None,
        };
        // A pull flies its slime only as far as `pull_short` dish cells
        // from the shooter, its nearest walker (a reach from the hit)
        // included; with no room, none.
        let steps = match (c.herd, toward) {
            (Some(Card::Pull), Some(t)) => {
                let room = (t.0 - hit.0).hypot(t.1 - hit.1) - reach - rr.pull_short * s;
                let per = (g.world.walker_step() * rr.fling_speed).max(1e-3);
                let most = (room / per).floor();
                if most < 1.0 {
                    self.short += 1;
                    return;
                }
                steps.min(most.min(u16::MAX as f32) as u16)
            }
            _ => steps,
        };
        let heading = match toward.filter(|t| (t.0 - hit.0).hypot(t.1 - hit.1) > 1e-3) {
            Some(t) => Some((t.1 - hit.1).atan2(t.0 - hit.0)),
            // A shove a card gave pushes straight on along the bullet's
            // line. The slug's own throws out from behind the hit (step A),
            // which a standard bullet's shove can't copy: thrown out from
            // behind, 61 of 89 went away from the gun (seed 2), the rest
            // sideways or back at the squad.
            None if shape.shove <= 0.0 => Some(dir.1.atan2(dir.0)),
            None => None,
        };
        let b = match heading {
            // A push: the disc round the hit, one heading.
            Some(a) => Burst { at: (hit.0 / s, hit.1 / s), r: 1e-3, fling: reach / s, fling_steps: steps, push: Some(a), ..Burst::default() },
            None => {
                let at = ((hit.0 - dir.0 * r) / s, (hit.1 - dir.1 * r) / s);
                Burst { at, r: r / s, fling: reach / s, fling_steps: steps, ..Burst::default() }
            }
        };
        if b.push.is_some() {
            self.pushes += 1;
        }
        self.shoves += 1;
        self.queued.push((b, kr, Due::Shove));
        self.queued_turret.push(self.by_turret);
    }

    /// Casts `c` at `at` (fine cells), from a bullet heading `dir` (None:
    /// a staged cast, `cast_at`).
    ///
    /// **A bomb throws away from the gun** (2026-11-16, step A): with a
    /// heading, its kill and crater stay on the hit and its throw is a
    /// burst of its own, the ring centred its kill radius back toward the
    /// gun and reaching as far past the hit, as a slug's shove is
    /// (`Rounds::shove`). Thrown all round the hit (the 11-14 bomb), the
    /// near half flew at the squad: with the step-A bomb's longer throw
    /// the squad's health at 33 s fell from 76 to 49 against main's bomb
    /// (the Fork's hold, one seed; `Reports/gun-shapes-2026-11-16/`).
    fn fire<W: World>(&mut self, g: &mut Game<W>, c: &Cast, at: (f32, f32), dir: Option<(f32, f32)>) {
        let s = g.world.scale();
        let t = g.world.ticks();
        let fine = g.rules.tps as f32 * g.world.fine_steps() as f32;
        let k = self.rules.burst_rules(fine);
        for b in bursts(c, at, &self.rules, s, fine) {
            let (burst, throw) = match dir.filter(|_| b.fling > b.r) {
                Some(d) => {
                    let back = (b.at.0 - d.0 * b.r, b.at.1 - d.1 * b.r);
                    (Burst { fling: 0.0, fling_steps: 0, ..b }, Some(Burst { at: back, r: b.r, fling: b.fling + b.r, fling_steps: b.fling_steps, ..Burst::default() }))
                }
                None => (b, None),
            };
            // Counted when the step's batch runs (`flush`).
            let i = self.pops.len();
            for p in std::iter::once(burst).chain(throw) {
                self.queued.push((p, k, Due::Pop(i)));
                self.queued_turret.push(self.by_turret);
            }
            self.pops.push(Pop { burst, throw, payload: c.payload, tick: t, killed: 0, flung: 0 });
            // A flare burns on (`burn`), its first lay this one.
            if c.payload == Card::Flare && burst.alarm > 0.0 {
                if self.burning.len() >= PENDING_MAX {
                    self.dropped += 1;
                } else {
                    let every = ((self.rules.flare_every * g.rules.tps as f32).round() as u64).max(1);
                    let out = t + (self.rules.flare_secs * g.rules.tps as f32).round() as u64;
                    self.burning.push(Burn { burst, next: t + every, out });
                }
            }
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

    /// The Mind bullets (2026-11-16, step D): each is a payload that casts
    /// one status burst at its hit (no kill, no alarm), wide by `Wide` as
    /// the gas, and counts in the Mind slot; the burst rules carry its
    /// length and the spin's curl in radians.
    #[test]
    fn a_mind_bullet_casts_its_status_and_kills_nothing() {
        let r = RoundRules::default();
        for (name, kind) in [("blind", big::MIND_BLIND), ("sour", big::MIND_SOUR), ("spin", big::MIND_SPIN)] {
            let c = Card::from_name(name).unwrap();
            assert_eq!((c.kind(), c.word(), slot(c)), (Kind::Payload, name.to_string(), Some(4)));
            let cast = casts(&parse(&format!("wide+{name}")).unwrap())[0];
            let b = bursts(&cast, (40.0, 20.0), &r, 2.0, 120.0);
            assert_eq!(b.len(), 1);
            assert_eq!((b[0].mind as u32, b[0].kill, b[0].alarm, b[0].at), (kind, false, 0.0, (20.0, 10.0)));
            assert!((b[0].r - r.mind_r * r.wide).abs() < 1e-4, "{name}: {} wide", b[0].r);
        }
        let k = r.burst_rules(120.0);
        assert_eq!(k.mind_steps, (r.mind_secs * 120.0) as u16);
        assert!((k.spin_curl - r.spin_curl.to_radians()).abs() < 1e-6);
        let mut tuned = r;
        assert!(tuned.set("spin_curl", 50.0) && tuned.spin_curl == 50.0 && !tuned.set("no_such", 1.0));
    }

    /// The readers (2026-11-16, step D): each is a payload that casts one
    /// read burst at its hit (no kill, alarm or crater), wide by `Wide`,
    /// Shatter over `read_r` and Detonate over `detonate_r`, counted in the
    /// readers' slot. On the CPU world a shatter alone kills nothing, and
    /// gas then shatter at one point kills what the gas put to sleep.
    #[test]
    fn a_reader_casts_its_read_and_kills_only_that_state() {
        let r = RoundRules::default();
        for (name, kind, reach) in [("shatter", big::READ_SLEEP, r.read_r), ("detonate", big::READ_SICK, r.detonate_r)] {
            let c = Card::from_name(name).unwrap();
            assert_eq!((c.kind(), c.word(), slot(c)), (Kind::Payload, name.to_string(), Some(5)));
            let b = bursts(&casts(&parse(&format!("wide+{name}")).unwrap())[0], (40.0, 20.0), &r, 2.0, 120.0);
            assert_eq!(b.len(), 1);
            assert_eq!((b[0].read, b[0].kill, b[0].alarm, b[0].crater, b[0].mind, b[0].at), (kind, false, 0.0, 0, 0, (20.0, 10.0)));
            assert!((b[0].r - reach * r.wide).abs() < 1e-4, "{name}: {} wide", b[0].r);
        }
        let mut tuned = r;
        assert!(tuned.set("read_r", 5.0) && tuned.set("detonate_r", 9.0) && (tuned.read_r, tuned.detonate_r) == (5.0, 9.0));
        use crate::mould::larder_world::race_world;
        let play = |round: &str| {
            let mut g = Game::new(race_world(1, 2, 5_000), Rules::default(), 1);
            for _ in 0..30 {
                g.step();
            }
            let living: Vec<usize> = (0..g.world.walkers.len()).filter(|&k| g.world.lives(k)).collect();
            let (x, y) = living.iter().fold((0.0, 0.0), |a, &k| (a.0 + g.world.walkers[k].0, a.1 + g.world.walkers[k].1));
            let at = (x / living.len() as f32, y / living.len() as f32);
            let mut rd = Rounds::new(parse(round).unwrap(), RoundRules::default());
            rd.cast_at(&mut g, at);
            (rd.gassed, rd.killed, rd.cast)
        };
        let (_, alone, cast) = play("shatter");
        assert_eq!((alone, cast[5]), (0, 1), "a shatter alone kills nothing");
        let (gassed, killed, cast) = play("gas+shatter");
        eprintln!("gas then shatter: {gassed} gassed, {killed} killed");
        assert!(gassed > 50 && killed == gassed && cast[1] == 1 && cast[5] == 1, "the shatter takes the gassed: {gassed}, {killed}");
    }

    /// **The route family** (2026-11-17, step D): a flare is a payload
    /// that kills nothing and lays the alarm (`flare_alarm` over
    /// `flare_r`, wide by `Wide`; none when Quiet); it counts in its own
    /// slot and burns, laying it again every `flare_every` until
    /// `flare_secs` are out.
    #[test]
    fn a_flare_burns_its_alarm_and_kills_nothing() {
        let r = RoundRules::default();
        let c = Card::Flare;
        assert_eq!((c.kind(), c.word(), slot(c), c.cost(), c.tier()), (Kind::Payload, "flare".to_string(), Some(6), 2, 2));
        let b = bursts(&casts(&parse("wide+flare").unwrap())[0], (40.0, 20.0), &r, 2.0, 120.0);
        assert_eq!(b.len(), 1);
        assert_eq!((b[0].kill, b[0].r, b[0].alarm, b[0].crater, b[0].at), (false, 0.0, r.flare_alarm, 0, (20.0, 10.0)));
        assert!((b[0].alarm_r - r.flare_r * r.wide).abs() < 1e-4, "{} wide", b[0].alarm_r);
        let hush = bursts(&casts(&parse("quiet+flare").unwrap())[0], (40.0, 20.0), &r, 2.0, 120.0);
        assert_eq!(hush[0].alarm, 0.0, "a quiet flare alarms nothing");
        let mut tuned = r;
        assert!(tuned.set("flare_secs", 1.0) && tuned.set("flare_r", 7.0) && (tuned.flare_secs, tuned.flare_r) == (1.0, 7.0));
        // The burn on the race world: one cast, then a step at a time.
        use crate::mould::larder_world::race_world;
        let mut g = Game::new(race_world(1, 2, 5_000), Rules::default(), 1);
        for _ in 0..10 {
            g.step();
        }
        let mut rd = Rounds::new(parse("flare").unwrap(), r);
        let at = (100.0, 100.0);
        rd.cast_at(&mut g, at);
        assert_eq!((rd.cast[6], rd.killed, rd.burning(g.world.ticks()).count()), (1, 0, 1));
        let tps = g.rules.tps as f32;
        let (every, secs) = ((r.flare_every * tps).round() as u32, (r.flare_secs * tps).round() as u32);
        let mut lays = 0;
        for _ in 0..secs + 2 {
            g.step();
            let before = rd.batched;
            rd.after_step(&mut g);
            lays += rd.batched - before;
        }
        eprintln!("a flare over {secs} ticks, every {every}: {lays} lays again");
        assert_eq!(lays, secs / every - 1 + u32::from(secs % every != 0), "it lays again every {every} ticks until it goes out");
        assert_eq!((rd.burning(g.world.ticks()).count(), rd.killed), (0, 0), "out, and it killed nothing");
    }

    /// **The element family** (2026-11-17, step D): a torch is a payload
    /// with no kill, alarm or crater of its own that lights the plants
    /// within `torch_r` (wide by `Wide`), all of them at `torch_light` 1,
    /// counted in its own slot. On the flats map fully planted, one torch
    /// on the slime in the thickest grass near the home lights the plants
    /// there and kills nothing at its cast; the fire it starts kills
    /// walkers in the seconds after (102 cells lit, 68 burned in 4 s, out
    /// within a second: the grass under a body is damp, `plants::
    /// moisture`). On the same world with no plants it lights nothing and
    /// nothing dies of fire.
    #[test]
    fn a_torch_lights_the_plants_and_the_fire_kills() {
        let r = RoundRules::default();
        let c = Card::Torch;
        assert_eq!((c.kind(), c.word(), slot(c), c.cost(), c.tier()), (Kind::Payload, "torch".to_string(), Some(7), 2, 2));
        assert_eq!(Shell::of(c).map(|s| s.special()), Some(true));
        let b = bursts(&casts(&parse("wide+torch").unwrap())[0], (40.0, 20.0), &r, 2.0, 120.0);
        assert_eq!(b.len(), 1);
        assert_eq!((b[0].kill, b[0].burn, b[0].alarm, b[0].crater, b[0].light, b[0].at), (false, false, 0.0, 0, r.torch_light, (20.0, 10.0)));
        assert!((b[0].r - r.torch_r * r.wide).abs() < 1e-4, "{} wide", b[0].r);
        let mut tuned = r;
        assert!(tuned.set("torch_r", 9.0) && tuned.set("torch_light", 0.5) && (tuned.torch_r, tuned.torch_light) == (9.0, 0.5));
        use crate::mould::larder_world::race_params;
        use crate::mould::map::Map;
        use crate::mould::plants::pack;
        let dir = std::path::Path::new(env!("CARGO_MANIFEST_DIR")).join("Reports/larder-maps-2026-11-11/maps");
        let play = |planted: bool| {
            let mut m = Map::load(&dir, "flats").unwrap();
            if planted {
                // Meadow, scrub and jungle in bands (the plants' twin test's
                // cover, with no reeds, which never catch).
                m.plants = (0..m.ground.len()).map(|i| pack([0, 1, 3, 0][(i / m.w / 160).min(3)], 1.0)).collect();
            }
            let mut g = Game::new(BigDish::with_map(race_params(3, 1, 2_000), &m), Rules::default(), 3);
            for _ in 0..10 * g.rules.tps {
                g.step();
            }
            // In thick grass: of the living walkers within 40 dish cells
            // of the home, the one with the most plants within 3 cells
            // (fire kills at `fire_kill` heat, which thin grass, or grass
            // the slime has worn, never reaches: its heat is capped by its
            // fuel, `plants::fire_step`).
            let home = g.world.homes[0];
            let s = g.world.scale();
            let w = &g.world;
            let fuel = |p: (f32, f32)| {
                let (cx, cy) = (p.0 as i64, p.1 as i64);
                let mut f = 0.0f32;
                for y in (cy - 3 * s as i64).max(0)..=(cy + 3 * s as i64).min(w.h as i64 - 1) {
                    for x in (cx - 3 * s as i64).max(0)..=(cx + 3 * s as i64).min(w.w as i64 - 1) {
                        f += w.plant.get(y as usize * w.w + x as usize).copied().unwrap_or(0.0);
                    }
                }
                f
            };
            let edge = (0..w.walkers.len())
                .filter(|&k| w.lives(k))
                .map(|k| (w.walkers[k].0, w.walkers[k].1))
                .filter(|p| (p.0 / s - home.0).hypot(p.1 / s - home.1) <= 40.0)
                .max_by(|a, b| fuel(*a).total_cmp(&fuel(*b)))
                .unwrap();
            let mut rd = Rounds::new(parse("torch").unwrap(), r);
            let lit0 = g.world.burning;
            rd.cast_at(&mut g, edge);
            let lit = g.world.burning - lit0;
            let killed_at_cast = rd.killed;
            let fire0 = g.world.fire_killed;
            for _ in 0..4 * g.rules.tps {
                g.step();
            }
            (rd.cast[7], lit, killed_at_cast, g.world.fire_killed - fire0)
        };
        let (cast, lit, at_cast, burned) = play(true);
        eprintln!("a torch on the planted flats: {lit} cells lit, {at_cast} killed at the cast, {burned} burned in 4 s");
        assert_eq!((cast, at_cast), (1, 0), "one torch, and no kill of its own");
        assert!(lit > 50, "it lit the plants: {lit}");
        assert!(burned > 0, "its fire killed walkers");
        let (cast, lit, _, burned) = play(false);
        assert_eq!((cast, lit, burned), (1, 0, 0), "no plants: nothing lit, nothing burned");
    }

    /// **A flare moves the slime by its own alarm rule** (the route
    /// family, 2026-11-17): on the race world warmed 20 s, a flare at the
    /// edge of the body. A flinching mould (`BigParams::flinch` 0.9, shape
    /// C's) has fewer walkers within 12 dish cells 2 s later than with no
    /// flare; a mould that walks to alarm (the Swarmer's `alarm_keep` 0.98,
    /// `alarm_weight` 3) has more.
    #[test]
    fn a_flare_pushes_a_flinching_mould_and_draws_a_swarmer() {
        use crate::mould::larder_world::race_world;
        let near = |flare: bool, swarm: bool| {
            let mut d = race_world(1, 2, 5_000);
            if swarm {
                (d.p.alarm_keep, d.p.alarm_weight) = (0.98, 3.0);
            } else {
                d.p.flinch = 0.9;
                d.p = d.p.flinched();
            }
            let mut g = Game::new(d, Rules::default(), 1);
            let tps = g.rules.tps;
            for _ in 0..20 * tps {
                g.step();
            }
            // Thin slime: of every 25th living walker, the one with
            // nearest 60 others within 12 dish cells (the same spot with
            // and without the flare, as the game is the same to here).
            let w = &g.world;
            let r = 12.0 * w.scale();
            let living: Vec<usize> = (0..w.walkers.len()).filter(|&k| w.lives(k)).collect();
            let around = |p: (f32, f32)| living.iter().filter(|&&k| (w.walkers[k].0 - p.0).hypot(w.walkers[k].1 - p.1) <= r).count();
            let at = living.iter().step_by(25).map(|&k| (w.walkers[k].0, w.walkers[k].1)).min_by_key(|&p| around(p).abs_diff(60)).unwrap();
            let mut rd = Rounds::new(parse("flare").unwrap(), RoundRules::default());
            if flare {
                rd.cast_at(&mut g, at);
            }
            for _ in 0..2 * tps {
                g.step();
                rd.after_step(&mut g);
            }
            let r = 12.0 * g.world.scale();
            let w = &g.world;
            (0..w.walkers.len()).filter(|&k| w.lives(k) && (w.walkers[k].0 - at.0).hypot(w.walkers[k].1 - at.1) <= r).count()
        };
        let (calm, flinch) = (near(false, false), near(true, false));
        let (still, swarm) = (near(false, true), near(true, true));
        eprintln!("within 12 cells 2 s on: flinching {calm} -> {flinch} with a flare, swarmer {still} -> {swarm}");
        assert!(flinch * 10 < calm * 9, "a flinching mould backs off a flare: {calm} -> {flinch}");
        assert!(swarm * 10 > still * 11, "a swarmer floods to it: {still} -> {swarm}");
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
        assert_eq!(rd.cast, [0; 8], "a trail casts nothing");
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
        let f = flight(&round);
        assert_eq!((f.pierce, f.split, f.reach, f.rate, f.casts), (1, 2, 0, 0, 1));
        assert_eq!(f.ammo[0], Ammo::standard(Shell::Blast), "the blast card fires a standard blast bullet");
        let f = flight(&parse("range+range+pierce").unwrap());
        assert_eq!((f.pierce, f.split, f.reach, f.rate, f.casts), (1, 0, 2, 0, 0));
        assert_eq!(casts(&round).len(), 1);
        assert_eq!(casts(&parse("split").unwrap()), vec![]);
        assert_eq!(flight(&parse("blast+wide+gas+quiet").unwrap()).casts as usize, casts(&parse("blast+wide+gas+quiet").unwrap()).len());
        let mut g = Game::new(BigDish::new(race_params(1, 1, 500)), Rules::default(), 1);
        Rounds::new(round, r).arm(&mut g);
        assert_eq!(g.bullet_mods, flight(&parse("pierce+split+split+blast").unwrap()));
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
        let k = r.burst_rules(fine);
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
        // 20 s, was 10: with organic starts (2026-11-15, `start_shape`) seed
        // 2's squad had cast no blast by 10 s, the mould's edge lying
        // elsewhere; the claim is about the bank, not the walk.
        for _ in 0..15 * 20 {
            p.step();
            free.after_step(&mut p);
        }
        assert!(free.cast[0] > 0, "an empty bank blasts");
        assert_eq!(p.bank, 0.0, "and pays nothing for it");
    }

    /// Rolled bullets (owner 16:46Z; payloads are bullets, 18:05Z): a
    /// bullet card's damage widens the strike (area as damage) and its
    /// payload's burst, its speed, range and spread scale the gun's, and
    /// it writes and reads back as text in a round.
    #[test]
    fn a_bullet_cards_numbers_ride_its_shots() {
        let slug = Ammo { shell: Shell::Plain, set: Set::Slug, depth: 1, damage: 200, speed: 50, range: 50, spread: 100, cost: 2 };
        let fast = Ammo { shell: Shell::Blast, set: Set::Needle, depth: 1, damage: 400, speed: 200, range: 150, spread: 50, cost: 5 };
        let round = vec![Card::Ammo(slug), Card::Wide, Card::Ammo(fast)];
        assert_eq!(parse(&name(&round)), Some(round.clone()), "{}", name(&round));
        assert_eq!(cost(&round), 2 + 2 + 5);
        let c = casts(&round);
        assert_eq!((c.len(), c[0].payload, c[1].payload, c[1].wide), (2, Card::Plain, Card::Blast, 1));
        let r = RoundRules::default();
        assert!(bursts(&c[0], (50.0, 50.0), &r, 2.0, 30.0).is_empty(), "a plain bullet casts nothing");
        let b = bursts(&c[1], (50.0, 50.0), &r, 2.0, 30.0);
        assert!((b[0].r - r.kit.blast_r * r.wide * 2.0).abs() < 1e-4, "damage 40: a blast twice as wide, then Wide: {}", b[0].r);
        // In play: the slug's bullets fly at half speed, half range, with
        // a wider strike; the blast needle's at twice the speed.
        let mut g = game(2);
        g.bank = 1.0e9;
        for k in 0..g.soldiers.len() {
            g.apply(Cmd::Order(k, Order::Attack));
        }
        g.apply(Cmd::Load(0, Some(Deck::new(&round))));
        let strike = (g.rules.strike * g.world.scale()).min(crate::mould::squad::R_MAX);
        let reach = g.gun_range(0) * g.world.scale();
        let mut seen = [0; 2];
        for _ in 0..15 * 20 {
            g.step();
            for b in g.bullets.iter().filter(|b| b.by.is_some()) {
                match b.cast {
                    1 => {
                        seen[0] += 1;
                        assert_eq!(b.speed, 0.5);
                        assert!((b.r - (strike * 2f32.sqrt()).min(crate::mould::squad::R_MAX)).abs() < 1e-4 && b.r > strike, "a wider strike: {} against {strike}", b.r);
                        assert!(b.left <= 0.5 * reach + 1e-3, "half the range: {} of {reach}", b.left);
                    }
                    2 => {
                        seen[1] += 1;
                        assert_eq!(b.speed, 2.0);
                    }
                    c => panic!("a plain bullet from a loaded squad: cast {c}"),
                }
            }
        }
        assert!(seen[0] > 0 && seen[1] > 0, "{seen:?}");
    }

    /// A plain bullet of `set` at standard numbers: only its shape differs.
    fn plain(set: Set) -> Card {
        Card::Ammo(Ammo { set, ..Ammo::standard(Shell::Plain) })
    }

    /// **A Pull on a turret's shot drags toward that turret** (2026-11-16,
    /// the independent review: with no shooter id it shoved away along the
    /// shot's line): the push heads from the hit to the gun it is given.
    #[test]
    fn a_pull_heads_for_the_gun_that_fired() {
        let mut g = game(1);
        let mut rd = Rounds::new(Vec::new(), RoundRules::default());
        let c = casts(&parse("pull+plain").unwrap())[0];
        let sc = g.world.scale();
        let hit = (40.0 * sc, 40.0 * sc);
        // A turret 30 cells straight "down" from the hit, the shot flying
        // across: the pull must head for the turret, not along the shot.
        let gun = (40.0 * sc, 70.0 * sc);
        rd.shove(&mut g, hit, (1.0, 0.0), 2.0, c.ammo.set.shape(), &c, Some(gun));
        let a = rd.queued.first().and_then(|q| q.0.push).expect("a push");
        assert!((a - std::f32::consts::FRAC_PI_2).abs() < 1e-3, "heading {a}");
        assert_eq!(rd.queued_turret, vec![false]);
    }

    fn attacking(seed: u64) -> Game<BigDish> {
        let mut g = game(seed);
        g.bank = 1.0e9;
        for k in 0..g.soldiers.len() {
            g.apply(Cmd::Order(k, Order::Attack));
        }
        g
    }

    /// **Each set has its own shape** (2026-11-16, the gun-system review's
    /// step A): a needle flies on through two more slime hits, buckshot
    /// fires a fan of four pellets half a strike's radius each, a slug
    /// shoves where it hits; the standard bullet is as it was.
    #[test]
    fn each_set_has_its_own_shape() {
        let r = RoundRules::default();
        let round: Vec<Card> = Set::ALL.into_iter().map(plain).collect();
        let mut g = attacking(2);
        g.apply(Cmd::Load(0, Some(Deck::new(&round))));
        let mut rd = Rounds::new(Vec::new(), r);
        let strike = (g.rules.strike * g.world.scale()).min(crate::mould::squad::R_MAX);
        // By set (cast 1 standard, 2 slug, 3 needle, 4 buckshot): shots
        // fired (a soldier's frame), bullets fired, slime hits.
        let (mut shots, mut bullets, mut hits) = ([0u32; 4], [0u32; 4], [0u32; 4]);
        for _ in 0..15 * 30 {
            g.step();
            let mut now = vec![[0u32; 4]; g.soldiers.len()];
            for s in g.last_shots.iter().filter(|s| s.by.is_some()) {
                let c = s.cast as usize - 1;
                if s.fired {
                    now[s.by.unwrap()][c] += 1;
                    bullets[c] += 1;
                    let want = if c == 3 { 0.5 * strike } else { strike };
                    assert!((s.r - want).abs() < 1e-4, "set {c}: r {} against {want}", s.r);
                }
                if s.hit.is_some() {
                    hits[c] += 1;
                }
            }
            for n in now {
                for c in 0..4 {
                    shots[c] += (n[c] > 0) as u32;
                    // A rifle fires one bullet a shot; buckshot four.
                    assert!(n[c] == 0 || n[c] == if c == 3 { 4 } else { 1 }, "set {c}: {} bullets in a shot", n[c]);
                }
            }
            for b in g.bullets.iter().filter(|b| b.by.is_some() && b.fresh) {
                assert_eq!(b.pierce, if b.cast == 3 { 2 } else { 0 }, "{b:?}");
            }
            rd.after_step(&mut g);
        }
        eprintln!("shots {shots:?}, bullets {bullets:?}, hits {hits:?}, shoves {} threw {}", rd.shoves, rd.shoved);
        assert!(shots.iter().all(|&n| n > 0), "{shots:?}");
        assert_eq!(bullets[3], 4 * shots[3]);
        // Each plain bullet hits once at most; a needle can hit three times.
        assert!(hits[0] <= bullets[0] && hits[1] <= bullets[1]);
        assert!(hits[2] > bullets[2], "a needle flies on: {} hits from {} bullets", hits[2], bullets[2]);
        // Every slug hit shoved, and only slug hits.
        assert!(hits[1] > 0 && rd.shoves == hits[1] && rd.shoved > 0, "{} shoves for {} slug hits", rd.shoves, hits[1]);
        assert_eq!(rd.killed, 0, "a shove kills none");
        // A buckshot payload's burst is a pellet's share of it.
        let c = casts(&[Card::Ammo(Ammo { set: Set::Buckshot, ..Ammo::standard(Shell::Blast) })])[0];
        let b = bursts(&c, (50.0, 50.0), &r, 2.0, 30.0);
        assert!((b[0].r - 0.5 * r.kit.blast_r).abs() < 1e-4, "{}", b[0].r);
    }

    /// A slug's shove throws the slime it hits away from the gun, and the
    /// shove kills none; a bomb's throw goes away from the gun too.
    #[test]
    fn a_slug_shoves_and_a_bomb_throws_away_from_the_gun() {
        for (card, seed) in [(plain(Set::Slug), 2), (plain(Set::Slug), 3), (Card::Bomb, 2), (Card::Bomb, 3)] {
            let slug = card != Card::Bomb;
            let mut g = attacking(seed);
            g.apply(Cmd::Load(0, Some(Deck::new(&[card]))));
            let mut rd = Rounds::new(Vec::new(), RoundRules::default());
            let mut checked = false;
            for _ in 0..15 * 30 {
                g.step();
                let hits: Vec<_> = g.last_shots.iter().filter(|s| s.by.is_some() && s.hit.is_some()).cloned().collect();
                let (before, was, killed) = (g.world.walkers.clone(), g.world.flung.clone(), g.killed);
                rd.after_step(&mut g);
                assert!(!slug || g.killed == killed, "a shove kills none");
                if hits.len() != 1 {
                    continue;
                }
                let thrown: Vec<usize> = (0..before.len()).filter(|&j| g.world.flung.get(j) == Some(&true) && was.get(j) != Some(&true)).collect();
                if thrown.len() < 40 {
                    continue;
                }
                let s = &hits[0];
                let (dx, dy) = (s.to.0 - s.from.0, s.to.1 - s.from.1);
                let d = dx.hypot(dy);
                let secs = if slug { crate::mould::bullets::SLUG_SHOVE_SECS } else { rd.rules.bomb_secs };
                for _ in 0..(secs * g.rules.tps as f32) as usize + 4 {
                    g.step();
                }
                let away = thrown.iter().filter(|&&j| (g.world.walkers[j].0 - before[j].0) * dx / d + (g.world.walkers[j].1 - before[j].1) * dy / d > 0.0).count();
                eprintln!("{card:?} seed {seed}: {} thrown, {away} away from the gun", thrown.len());
                assert!(away * 10 >= thrown.len() * 8, "{card:?} seed {seed}: {away} of {} thrown away from the gun", thrown.len());
                checked = true;
                break;
            }
            assert!(checked, "{card:?} seed {seed}: no lone hit that threw");
        }
    }

    /// **Carriers** (2026-11-16, step B): a `Trigger` or `Expire` makes its
    /// cast carry the next one, which leaves the gun's own turn; a chain
    /// stops at `CARRY_DEPTH`, and a carrier with nothing after it carries
    /// nothing.
    #[test]
    fn a_carrier_takes_the_next_cast_out_of_the_turn() {
        let c = casts(&parse("trigger+plain+blast").unwrap());
        assert_eq!((c[0].child, c[0].expire, c[1].child), (Some(1), false, None));
        assert_eq!(cycle(&c), vec![1]);
        let f = flight(&parse("trigger+plain+blast+gas").unwrap());
        assert_eq!((f.casts, &f.cycle[..2], &f.child[..3]), (2, &[1, 3][..], &[2, 0, 0][..]));
        assert_eq!(f.ammo[1], Ammo::standard(Shell::Blast), "ammo stays by cast index");
        let c = casts(&parse("expire+plain+blast").unwrap());
        assert!(c[0].expire && c[0].child == Some(1));
        // A chain: the plain carries the gas, which carries the blast.
        // (With six slots three carriers leave the third nothing to carry,
        // so `CARRY_DEPTH` is a guard for longer guns.)
        let c = casts(&parse("trigger+plain+expire+gas+blast").unwrap());
        assert_eq!(c.iter().map(|c| (c.child, c.expire)).collect::<Vec<_>>(), vec![(Some(1), false), (Some(2), true), (None, false)]);
        assert_eq!(cycle(&c), vec![1]);
        let c = casts(&parse("trigger+plain+trigger+gas+trigger+blast").unwrap());
        assert_eq!(c.iter().map(|c| c.child).collect::<Vec<_>>(), vec![Some(1), Some(2), None]);
        // Nothing after it.
        assert_eq!(casts(&parse("trigger+blast").unwrap())[0].child, None);
        assert_eq!(cycle(&casts(&parse("plain+trigger").unwrap())), vec![1]);
        for c in [Card::Trigger, Card::Expire] {
            assert_eq!((c.kind(), Card::from_word(c.name())), (Kind::Trigger, Some(c)));
        }
    }

    /// **A trigger's cast goes off where its bullet hit** (step B): with
    /// `trigger+plain+blast` the gun fires only the plain bullet, and every
    /// blast is let out where one of those hit slime, so each shot that
    /// lands blasts (a blast in turn only every second shot). The plain
    /// hit kills the slime there first, so the blast flies on through the
    /// hole it made and goes off at the next slime along the plain bullet's
    /// line: within 3 dish cells of the hit, or further straight on.
    #[test]
    fn a_trigger_blasts_where_its_bullet_hits() {
        let mut out = Vec::new();
        for round in ["trigger+plain+blast", "plain+blast"] {
            let mut g = attacking(2);
            g.apply(Cmd::Load(0, Some(Deck::new(&parse(round).unwrap()))));
            let mut rd = Rounds::new(Vec::new(), RoundRules::default());
            // Each plain hit and the bullet's heading.
            let mut hits: Vec<((f32, f32), (f32, f32))> = Vec::new();
            let (mut near, mut far, mut off) = (0, 0, 0);
            for _ in 0..15 * 30 {
                g.step();
                for s in g.last_shots.iter().filter(|s| s.by.is_some()) {
                    if s.fired && round.starts_with("trigger") {
                        assert_eq!(s.cast, 1, "the blast is never fired in turn");
                    }
                    if let (Some(h), 1) = (s.hit, s.cast) {
                        let d = (s.to.0 - s.from.0).hypot(s.to.1 - s.from.1).max(1e-3);
                        hits.push((h, ((s.to.0 - s.from.0) / d, (s.to.1 - s.from.1) / d)));
                    }
                }
                let before = rd.pops.len();
                rd.after_step(&mut g);
                let sc = g.world.scale();
                for p in rd.pops[before..].iter().filter(|p| p.payload == Card::Blast) {
                    let at = (p.burst.at.0 * sc, p.burst.at.1 * sc);
                    let ahead = |&(h, d): &((f32, f32), (f32, f32))| {
                        let v = ((at.0 - h.0) / sc, (at.1 - h.1) / sc);
                        let n = v.0.hypot(v.1);
                        (n <= 3.0, n <= g.rules.range && (v.0 * d.0 + v.1 * d.1) >= n * 0.35f32.cos())
                    };
                    match hits.iter().map(ahead).fold((false, false), |a, b| (a.0 || b.0, a.1 || b.1)) {
                        (true, _) => near += 1,
                        (false, true) => far += 1,
                        _ => off += 1,
                    }
                }
            }
            eprintln!("{round}: releases {}, blasts {}, killed {}, plain hits {}; blasts by a hit {near}, straight on from one {far}, neither {off}", g.releases, rd.cast[0], rd.killed, hits.len());
            if round.starts_with("trigger") {
                assert!(g.releases > 20 && rd.cast[0] > 20 && rd.cast[0] <= g.releases, "releases {} blasts {}", g.releases, rd.cast[0]);
                assert_eq!(off, 0, "every blast goes off by a plain hit or on along its line");
            } else {
                assert_eq!(g.releases, 0);
            }
            out.push((rd.cast[0], rd.killed));
        }
        assert!(out[0].0 > out[1].0, "a trigger blasts on every landing shot: {out:?}");
    }

    /// **Multicast links** (step B): a multicast's cast fires the next
    /// cast not yet taken beside it, a second copy a third; the casts
    /// beside it leave the gun's turn; a trigger in the same cast takes the
    /// cast after it first; past `CARRY_DEPTH` copies do nothing.
    #[test]
    fn a_multicast_fires_the_next_cast_beside_its_own() {
        let w = |r: &str| casts(&parse(r).unwrap()).iter().map(|c| c.with).collect::<Vec<_>>();
        assert_eq!(w("multicast+plain+blast"), vec![Some(1), None]);
        assert_eq!(cycle(&casts(&parse("multicast+plain+blast").unwrap())), vec![1]);
        assert_eq!(w("multicast+multicast+plain+blast+gas"), vec![Some(1), Some(2), None]);
        assert_eq!(volley(&casts(&parse("multicast+multicast+plain+blast+gas").unwrap()), 0), vec![0, 1, 2]);
        assert_eq!(w("multicast+multicast+multicast+plain+blast+gas"), vec![Some(1), Some(2), None], "a third copy is past the depth");
        // The trigger carries the blast; the multicast fires the gas beside.
        let c = casts(&parse("multicast+trigger+plain+blast+gas").unwrap());
        assert_eq!((c[0].child, c[0].with), (Some(1), Some(2)));
        assert_eq!(cycle(&c), vec![1]);
        // Nothing after it: a plain shot alone.
        assert_eq!(w("multicast+plain"), vec![None]);
        assert_eq!(cycle(&casts(&parse("plain+blast+multicast").unwrap())), vec![1, 2]);
        let f = flight(&parse("plain+multicast+blast+gas").unwrap());
        assert_eq!((f.casts, &f.cycle[..2], &f.with[..3]), (2, &[1, 2][..], &[0, 3, 0][..]));
        assert_eq!((Card::Multicast.kind(), Card::from_word("multicast"), Card::Multicast.tier()), (Kind::Trigger, Some(Card::Multicast), 3));
    }

    /// **A multicast shot fires both** (step B): with `multicast+plain+blast`
    /// every soldier shot fires a plain bullet and a blast side by side
    /// (`Game::multicast` one a shot), so blasts come every shot instead of
    /// every second one, and the step's bursts run as one batch.
    #[test]
    fn a_multicast_shot_fires_its_casts_side_by_side() {
        let mut out = Vec::new();
        for round in ["multicast+plain+blast", "plain+blast"] {
            let mut g = attacking(2);
            g.apply(Cmd::Load(0, Some(Deck::new(&parse(round).unwrap()))));
            let mut rd = Rounds::new(Vec::new(), RoundRules::default());
            let (mut fired, mut pairs) = ([0u32; 3], 0);
            for _ in 0..15 * 30 {
                g.step();
                let mut by: Vec<(usize, u8)> = g.last_shots.iter().filter(|s| s.fired).filter_map(|s| s.by.map(|k| (k, s.cast))).collect();
                for &(_, c) in &by {
                    fired[c as usize] += 1;
                }
                by.sort();
                pairs += by.windows(2).filter(|w| w[0].0 == w[1].0 && w[0].1 == 1 && w[1].1 == 2).count();
                rd.after_step(&mut g);
            }
            eprintln!("{round}: fired {fired:?}, side by side {pairs}, multicast {}, blasts {}, killed {}, batches {} of {} bursts", g.multicast, rd.cast[0], rd.killed, rd.batches, rd.batched);
            if round.starts_with("multicast") {
                assert!(fired[1] > 20 && fired[1] == fired[2] && g.multicast == fired[1], "every shot fires both: {fired:?}, multicast {}", g.multicast);
                assert_eq!(pairs as u32, fired[1], "each soldier's plain and blast go out in the same step");
            } else {
                assert_eq!(g.multicast, 0);
                assert!(fired[1].abs_diff(fired[2]) <= 8, "in turn: {fired:?}");
            }
            assert!(rd.batched >= rd.batches, "bursts run in batches");
            out.push((fired[2], rd.cast[0]));
        }
        assert!(out[0].0 > out[1].0 * 3 / 2, "a multicast fires a blast every shot: {out:?}");
    }

    /// **The shove cards** (owner 2026-11-16 05:11Z: "a gun that lets you
    /// corral and push the slime onto areas. Into a kill zone"): a heave
    /// gives a standard bullet a shove straight on along its line, and two
    /// throw it farther; a corral pushes the slime it hits toward the
    /// nearest turret, a pull toward the gun; each a block on one heading. Each
    /// on the first lone hit that throws 40 walkers or more, measured once
    /// the throw is over.
    #[test]
    fn shove_cards_push_the_slime_where_they_say() {
        use crate::mould::larder_world::Kind as Building;
        let mut thrown_far = Vec::new();
        for round in ["heave+plain", "heave+heave+plain", "corral+plain", "pull+plain"] {
            let mut g = attacking(2);
            g.apply(Cmd::Load(0, Some(Deck::new(&parse(round).unwrap()))));
            let sc = g.world.scale();
            // A turret beside the first soldier to hit slime, 12 cells off
            // its line, for the corral to push toward.
            let mut turret = None;
            // The squad attacks, so its hits soon lie past the default
            // reach of a turret it passed: here the corral reaches 100.
            let mut rd = Rounds::new(Vec::new(), RoundRules { corral_reach: 100.0, ..RoundRules::default() });
            let mut checked = false;
            for _ in 0..20 * 30 {
                g.step();
                let hits: Vec<_> = g.last_shots.iter().filter(|s| s.by.is_some() && s.hit.is_some()).cloned().collect();
                if turret.is_none() {
                    let Some(s) = hits.first() else {
                        rd.after_step(&mut g);
                        continue;
                    };
                    let d = (s.to.0 - s.from.0).hypot(s.to.1 - s.from.1);
                    let side = (-(s.to.1 - s.from.1) / d, (s.to.0 - s.from.0) / d);
                    let spot = (0..48)
                        .map(|i| {
                            let (off, f) = ((12.0 + (i / 2) as f32) * sc, if i % 2 == 0 { 1.0 } else { -1.0 });
                            (s.from.0 + side.0 * off * f, s.from.1 + side.1 * off * f)
                        })
                        .find(|&q| g.build_blocked(Building::Turret, q).is_none())
                        .expect("room for a turret");
                    assert!(g.build(Building::Turret, spot));
                    let t = g.buildings.len() - 1;
                    g.buildings[t].built = 1.0;
                    turret = Some(g.buildings[t].at);
                    rd.after_step(&mut g);
                    continue;
                }
                let turret = turret.unwrap();
                let (before, was) = (g.world.walkers.clone(), g.world.flung.clone());
                let pushes = rd.pushes;
                rd.after_step(&mut g);
                if hits.len() != 1 {
                    continue;
                }
                let thrown: Vec<usize> = (0..before.len()).filter(|&j| g.world.flung.get(j) == Some(&true) && was.get(j) != Some(&true)).collect();
                if thrown.len() < 40 {
                    continue;
                }
                let s = &hits[0];
                let h = s.hit.unwrap();
                let d = (s.to.0 - s.from.0).hypot(s.to.1 - s.from.1);
                let way = match round {
                    "corral+plain" => {
                        let n = (turret.0 - h.0).hypot(turret.1 - h.1);
                        ((turret.0 - h.0) / n, (turret.1 - h.1) / n)
                    }
                    "pull+plain" => (-(s.to.0 - s.from.0) / d, -(s.to.1 - s.from.1) / d),
                    _ => ((s.to.0 - s.from.0) / d, (s.to.1 - s.from.1) / d),
                };
                // A corral's hit past `corral_reach` of the turret pushes
                // straight on: wait for one in reach.
                if round.starts_with("corral") && (turret.0 - h.0).hypot(turret.1 - h.1) / sc > rd.rules.corral_reach {
                    continue;
                }
                let pushed = rd.pushes > pushes;
                assert!(pushed, "{round}: a card's shove is a push");
                let heaves = round.matches("heave").count() as i32;
                let secs = rd.rules.shove_secs * rd.rules.heave.powi(heaves);
                for _ in 0..(secs * g.rules.tps as f32) as usize + 4 {
                    g.step();
                }
                let moved: Vec<(f32, f32)> = thrown.iter().map(|&j| (g.world.walkers[j].0 - before[j].0, g.world.walkers[j].1 - before[j].1)).collect();
                let along = moved.iter().filter(|m| m.0 * way.0 + m.1 * way.1 > 0.0).count();
                // A push flies every one on the heading: of those that moved
                // a cell or more, 7 in 10 lie within 0.5 rad of it (a wall
                // turns some, and each walks a little once landed; a throw
                // out from a centre spreads them over half a circle).
                let free: Vec<&(f32, f32)> = moved.iter().filter(|m| m.0.hypot(m.1) >= 1.0).collect();
                let lined = free.iter().filter(|m| (m.0 * way.0 + m.1 * way.1) / m.0.hypot(m.1) >= 0.5f32.cos()).count();
                let mean = moved.iter().map(|m| m.0.hypot(m.1)).sum::<f32>() / moved.len() as f32 / sc;
                eprintln!("{round}: {} thrown, {along} the way it says, {lined} of {} free ones on its line, mean throw {mean:.1} cells", thrown.len(), free.len());
                assert!(along * 10 >= thrown.len() * 8, "{round}: {along} of {} thrown the way it says", thrown.len());
                if pushed {
                    assert!(lined * 10 >= free.len() * 7 && !free.is_empty(), "{round}: {lined} of {} on the push's line", free.len());
                }
                if round.starts_with("heave") {
                    thrown_far.push(mean);
                }
                checked = true;
                break;
            }
            assert!(checked, "{round}: no lone hit that threw");
        }
        assert!(thrown_far[1] > thrown_far[0], "two heaves throw farther than one: {thrown_far:?}");
        for c in [Card::Heave, Card::Corral, Card::Pull] {
            assert_eq!((c.kind(), Card::from_word(c.name())), (Kind::Modifier, Some(c)));
        }
    }

    /// Blast and bomb apart (2026-11-16): at one spot the blast kills more
    /// and throws none, the bomb kills little and throws more than it kills,
    /// and the blast's crater outlasts the bomb's many times over.
    #[test]
    fn a_blast_holds_and_a_bomb_throws() {
        let r = RoundRules::default();
        let (mut a, mut b) = (game(3), game(3));
        let d = &a.world;
        let (mut x, mut y, mut m) = (0.0, 0.0, 0.0);
        for (j, &(wx, wy, _)) in d.walkers.iter().enumerate() {
            if d.lives(j) {
                (x, y, m) = (x + wx, y + wy, m + 1.0);
            }
        }
        let p = (x / m, y / m);
        let fine = a.rules.tps as f32 * a.world.p.substeps as f32;
        let k = r.burst_rules(fine);
        let sc = a.world.p.scale as f32;
        let blast = bursts(&casts(&[Card::Blast])[0], p, &r, sc, fine)[0];
        let bomb = bursts(&casts(&[Card::Bomb])[0], p, &r, sc, fine)[0];
        let (ba, bb) = (a.world.burst(&blast, &k), b.world.burst(&bomb, &k));
        eprintln!("blast killed {} flung {}; bomb killed {} flung {}", ba.killed, ba.flung, bb.killed, bb.flung);
        assert!(ba.killed > 2 * bb.killed && ba.flung == 0, "blast {ba:?} bomb {bb:?}");
        assert!(bb.flung > bb.killed, "bomb {bb:?}");
        assert!(blast.crater >= 10 * bomb.crater && bomb.fling > blast.r, "{blast:?} {bomb:?}");
    }
}
