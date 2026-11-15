//! **The roguelike run** (owner's notes, 2026-11-14 15:06Z; answers
//! 15:21Z): a separate mode beside the campaign, which it does not touch.
//! A run is planet after planet on the way to the slime's home world (the
//! owner's story, 15:25Z). Each level is one map with one slime to beat;
//! the slime gets harder (the tier ladder, more walkers) the deeper the
//! run goes; between levels a shop spends the points a win earned on the
//! squad (armour, shots, range, a bigger landing), unlocks and upgrades
//! buildings and tools, and buys gun cards that get better with depth. A
//! lost level ends the run.
//!
//! **What is this module's and what is not.** The level's geometry (ground,
//! crystals, mould starts, landing, walls, the map's size) is the level
//! generator's (`levelgen::level(seed, depth) -> Map`, its own thread,
//! agreed 2026-11-14 15:25Z, #277): a level is planet `depth` of the run's
//! seed (`Ground::Planet`), its mission's map `planet:DEPTH:SEED` as the
//! window's `map=` reads it. Everything else about a level is here:
//! the tier, the walker count, the goal and the rules the upgrades set.
//! Like `campaign`, a level is a `Mission` over the shared game, so the
//! window plays it with the mission tracker and its end screen; nothing in
//! `larder_world` changes for a run.
//!
//! **Guns and cards.** A gun is the gun thread's `guns::Gun` (owner,
//! 16:46Z: "The squad owns the gun"; its budget and slots are what its
//! cards may cost and hold). The run owns guns in an armory and carries
//! one (`Run::carried`), which every soldier lands with: they all start in
//! squad 0 and a split carries it on. Each shop sells `GUN_OFFER` guns
//! rolled for the coming planet (`Gun::roll(seed, depth)`), so deeper
//! shops sell bigger budgets. Cards are the rounds' cards, bought into a
//! bag and put into the carried gun up to its budget and slots
//! (`rounds::fit_gun`). Before the guns (#283) the run had one gun whose
//! budget grew with a "Bigger gun" upgrade (5, +2 a level); a rolled gun
//! replaces it.
//!
//! Rules-free and window-free, like the HUD: the window turns `Run::shop`
//! rows into the start menu's shop page and a pick back into `Run::pick`.

use super::campaign::{Bonus, Goal, Mission};
use super::guns::{Gun, GunKind};
use super::kit::KitRules;
use super::larder_world::Rules;
use super::levelgen::{self, Planet};
use super::map::{Map, G_ROCK, MAP_H, MAP_W};
use super::rounds::{self, Ammo, Card, Kind, Set, Shell};
use super::tiers::Tier;

/// Where a level's ground comes from.
#[derive(Clone, Debug, PartialEq)]
pub enum Ground {
    /// A hand-made map by name (`Reports/larder-maps-2026-11-11/maps`).
    /// The first build cycled the five (Flats first) until the generator
    /// landed (#277, 2026-11-14); kept for designed planets.
    Named(String),
    /// A generated planet: `levelgen::level(seed, depth)`, built when the
    /// window starts the level (about 55 ms), not when the shop shows it.
    Planet { seed: u64, depth: u32 },
    /// A map already made.
    Made(Map),
}

impl Ground {
    /// The map, built or loaded (`dir`: the hand-made maps' folder).
    pub fn map(&self, dir: &std::path::Path) -> Result<Map, String> {
        match self {
            Ground::Named(n) => Map::load(dir, n),
            Ground::Planet { seed, depth } => Ok(levelgen::level(*seed, *depth)),
            Ground::Made(m) => Ok(m.clone()),
        }
    }
}

/// The slime by depth: the campaign's ladder, gentlest first. Past its
/// end the hardest three take turns. The Grudge joins once the GPU engine
/// takes its scar keys (`Tier::settings` refuses it today; the test below
/// keeps every rung playable).
pub const LADDER: [Tier; 6] = [Tier::Drifter, Tier::Sprawler, Tier::Trunk, Tier::Swarmer, Tier::Sporer, Tier::Sleeper];

/// One level: what the window needs to start it.
#[derive(Clone, Debug, PartialEq)]
pub struct Level {
    pub depth: u32,
    /// The world's seed (the run's, moved by depth).
    pub seed: u64,
    pub ground: Ground,
    pub tier: Tier,
    /// The walkers the slime starts with, as a factor on the window's own
    /// count (1 on the first planet).
    pub walkers: f32,
    /// The goal, the bonus and the rules, as a campaign mission.
    pub mission: Mission,
    /// Game seconds the slime grows alone before the squad lands
    /// (`head_start`); 0 on planets no bigger than today's map.
    pub grow: f32,
}

/// Planets in a run; the last is the slime's home world (the core world,
/// the level generator's biggest map). Told to the level-gen thread
/// 2026-11-14: the owner's guess is 20 minutes to over an hour a run.
pub const PLANETS: u32 = 8;

/// Walkers grow this much a planet (a fifth more each), times the
/// planet's area over today's map (`MAP_W` x `MAP_H`).
pub const WALKERS_PER_DEPTH: f32 = 0.2;

/// The open share (cells not rock) of a typical planet: planets 1-8 of
/// seeds 1-6 measured 0.84 to 0.98, mean 0.93 (11-14, before #289's
/// openness dial). `Level::walkers` assumes it; `open_scale` corrects it.
pub const REF_OPEN: f32 = 0.93;

/// The level's walkers, taken from the planet's area, put onto its open
/// cells instead (the technical lead, 11-14 19:23Z: walkers per open
/// cell, as the density rule works, so a rocky planet is not crowded and
/// the count thresholds keep their meaning): this times the area count.
pub fn open_scale(m: &Map) -> f32 {
    let open = m.ground.iter().filter(|&&g| g & G_ROCK == 0).count() as f32;
    open / (m.ground.len().max(1) as f32 * REF_OPEN)
}

/// Each planet's slime is this factor bigger or smaller than its depth and
/// area give, rolled from the level's seed: 0.8 to 1.25, even on a log
/// scale (so as often a fifth smaller as a quarter bigger). The owner,
/// 11-15 08:34Z, on how big a planet's slime should be: "It should vary
/// and be more later on". The depth's fifth a planet and the area keep it
/// more later on; this makes two planets at one depth differ.
pub fn size_roll(seed: u64) -> f32 {
    let u = (mix(seed ^ 0x5112_E5EE) >> 11) as f32 / (1u64 << 53) as f32;
    (0.8f32.ln() + u * (1.25f32.ln() - 0.8f32.ln())).exp()
}

/// The planet's area as a share of today's map (`MAP_W` x `MAP_H`): 0.6 on
/// planet 1, 6 on the core world (`levelgen::AREA`).
pub fn area_of(p: &Planet) -> f32 {
    (p.size.0 * p.size.1) as f32 / (MAP_W * MAP_H) as f32
}

/// The level's clock (`Rules::cap`, game seconds) for a planet of `area`
/// (`area_of`): 15 minutes on planet 1's 0.6 and 7.5 minutes more for
/// each whole map of area, in half minutes: 15:00, 16:30, 18:00, 21:00,
/// 25:30, 31:30, 40:30 and 55:30 on today's eight planets.
///
/// Why (the planet review, 11-15, finding 6): the clock was 900 s on
/// every planet, while the walk to the far nest grows from 14 s to 60 s
/// and the slime about 1.6x a planet, so the owner's "15/20 minutes to 1
/// hour+" a level (11-14 15:33Z) was not wired up. Linear in area, not in
/// its square root (the walk): the slime and the crystals scale with
/// area, and the clock is for clearing them, not for crossing the map.
pub fn clock(area: f32) -> f32 {
    let secs = 900.0 + 450.0 * (area - 0.6).max(0.0);
    (secs / 30.0).round() * 30.0
}

/// Game seconds the slime grows alone before the squad lands on a planet
/// of `area` (`area_of`): none up to today's map, then a minute for each
/// time the map's width doubles (60 x (sqrt(area) - 1)): 11, 25, 40, 60
/// and 87 s on planets 4-8.
///
/// Why (the planet review, 11-15, finding 5): the core world's slime
/// covered 3% of the map at 60 s against 24-26% on planet 1, so the
/// biggest planet looked the emptiest. A front spreads at the same speed
/// on any map, so the cover a map shows at a given time falls with its
/// width; growing it first, by the width, lands the squad on a planet
/// whose slime has already spread, laid its trail and burnt its plants
/// ("the core world arrives already digested"). Planets 1-3 land on a
/// fresh slime as before.
pub fn head_start(area: f32) -> f32 {
    (60.0 * (area.sqrt() - 1.0)).max(0.0).round()
}

/// The goal: the slime down to this share of its peak, on every planet.
///
/// 1% since the owner's 11-15 18:02Z ("We are still winning the game with
/// too many slime left. Drop it to 1% of peak"); 2% before.
///
/// Why 2% (owner, 11-15 03:02Z: "right now how it's set up I feel like I
/// keep winning before I've actually killed everything"): it was a
/// quarter on planet 1 down to a tenth from planet 6, so a planet was won
/// with a tenth to a quarter of the slime still on the map. 2% is under
/// the broken line (`Rules::broken`, 3% of the game's peak, which counts
/// from the landing and so sits higher than this goal's peak, counted from
/// 60 s): before the goal is met the slime is broken and its stragglers
/// off food starve in a second or two (the owner's earlier "having to kill
/// the few random stragglers spread across the map to win is annoying",
/// 11-12 22:06Z), so what is left to shoot is the slime still on its
/// crystals. Difficulty by depth comes from the tier and the walkers now,
/// not from a gentler goal.
pub fn shrink_goal(_depth: u32) -> f32 {
    WIN_SHARE
}

/// `shrink_goal` on every planet.
pub const WIN_SHARE: f32 = 0.01;

/// The slime's surges by planet, 1 to 8 (past the end the last): the food
/// it eats to build one, as a share of the sandbox's (`SURGE_FOOD`, 150,
/// times the level's walkers), the longest wait between them
/// (`BigParams::surge_every`, steps, 30 a game second), its lunge
/// (`surge_dash`, steps) and its thickening threshold (`thicken`, a
/// birth's energy). The sandbox's shape C is 1 (150 food), 1800 (60 s), 30
/// and 2. Until food-driven surges (later on 11-15) the surges came on a
/// fixed beat of half these waits (40 s on planet 1 to 15 s on planet 8);
/// the food shares keep that curve's ratios to the sandbox's 30 s.
const SURGE_FOOD_SHARE: [f32; 8] = [1.33, 1.33, 1.0, 1.0, 0.83, 0.67, 0.67, 0.5];
const LONGEST_WAIT: [u32; 8] = [2400, 2400, 1800, 1800, 1500, 1200, 1200, 900];
const SURGE_DASH: [u32; 8] = [30, 30, 30, 36, 42, 45, 45, 54];
const THICKEN: [f32; 8] = [2.0, 2.0, 1.8, 1.7, 1.6, 1.5, 1.4, 1.3];
/// **Breakaway from planet 2** (the whole-game review's W4 and D2; the
/// owner agreed 2026-11-15 21:41Z: "5-7) agreed", D2 being RP's option, on
/// from planet 2): dense slime condenses (`condense`, steps a crowded or
/// feeding walker takes to condense; 300 is 10 game seconds) and
/// condensed or thickened slime breaks away for a fresh crystal far off
/// (`roam`, steps between its chances). Both are needed: on planet 2 (run
/// seeds 1 and 2, squad hand, to 3:00) `roam` alone over shape C's
/// thickening moved almost nothing (19,093 walkers at 2:00 against 19,103
/// off), most likely because a thickened walker sits inside the body
/// with every cell round it full; condensing absorbs half the ripe, so
/// the pieces have room to leave
/// (`Reports/breakaway-run-2026-11-15/README.md`). Planet 1 keeps the
/// plain body, so the first planet teaches the slime before it spreads.
const CONDENSE: [u32; 8] = [0, 300, 300, 300, 300, 300, 300, 300];
const ROAM: [u32; 8] = [0, 300, 300, 300, 300, 300, 300, 300];

/// **The slime gets fiercer deeper** (2026-11-15, the slime-enemy thread;
/// the run thread agreed the curve): engine keys over shape C's on a run's
/// planet, before the tier's and the menu's own (`run_shape`). The slime
/// surges more often and lunges longer, and thickens sooner, so it presses
/// harder and gets tougher faster from what it holds; from planet 2 it
/// condenses and sends pieces off to fresh crystals (`CONDENSE`, `ROAM`,
/// 11-15 D2). Planet 1's slime
/// needs a third more food for a surge than the sandbox's and waits up to
/// 80 s; the core world's half as much, waiting at most 30 s and lunging
/// 1.8 s. Why: the owner, 11-15 08:30Z, wants "more interesting
/// progression in the enemy ... and increase in challenge ... especially
/// when your guns get stronger"; until now depth grew only the walkers
/// and changed the tier. The surge's reach follows the guns already (the
/// gun's range plus `Rules::panic_margin`).
pub fn enemy_keys(depth: u32) -> Vec<(&'static str, String)> {
    let i = depth_index(depth);
    let mut k = vec![("surge", LONGEST_WAIT[i].to_string()), ("surge_dash", SURGE_DASH[i].to_string()), ("thicken", format!("{}", THICKEN[i]))];
    // Breakaway (`CONDENSE`, `ROAM`) only where it is on, so planet 1
    // keeps whatever shape C and the menu set.
    if CONDENSE[i] > 0 {
        k.extend([("condense", CONDENSE[i].to_string()), ("roam", ROAM[i].to_string())]);
    }
    k
}

fn depth_index(depth: u32) -> usize {
    (depth.max(1) as usize - 1).min(LONGEST_WAIT.len() - 1)
}

/// The food a planet's slime eats to build a surge (`Rules::surge_food`):
/// its share of the sandbox's, times the level's walkers (`Level::walkers`,
/// as the slime's slots are), since a bigger slime on a bigger planet
/// eats more and would otherwise surge at the floor all game.
pub fn surge_food(depth: u32, walkers: f32) -> f32 {
    SURGE_FOOD_SHARE[depth_index(depth)] * crate::mould::larder_world::SURGE_FOOD * walkers.max(0.1)
}

/// A planet's longest wait between surges in game seconds (`enemy_keys`),
/// for the shop.
pub fn surge_secs(depth: u32) -> f32 {
    LONGEST_WAIT[depth_index(depth)] as f32 / 30.0
}

/// What a tier does to you: one thing each rung of `LADDER` does to the
/// player, on top of its shape (owner, 2026-11-15 21:41Z, the whole-game
/// review's D3, "Agreed. We can test it out"; the first-time player's
/// table). A test: each row is game rules (`Rules::set` names, set in
/// `Run::rules`) and engine keys (`gpu::set` names, set over the tier's own
/// in the window), measured by the films in
/// `Reports/run-progression-2026-11-15/does-to-you.md`. New engine rules
/// for a row belong to the slime-enemy thread; the numbers here are this
/// thread's.
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct DoesToYou {
    /// One sentence for the planet card.
    pub says: &'static str,
    pub rules: &'static [(&'static str, f32)],
    pub keys: &'static [(&'static str, &'static str)],
}

/// `DoesToYou` by tier (the plain mould and the Grudge do nothing extra).
pub fn does_to_you(tier: Tier) -> DoesToYou {
    let none = DoesToYou { says: "", rules: &[], keys: &[] };
    match tier {
        // A pump smells of food 20 times a soldier's scent over 15 dish
        // cells (`BuildRules::pump_smell`, `lure_r`, which only pumps use in
        // a run: lures are not sold). Planet 1 seed 1, pumps hand: the far
        // pump stood at 197 of 200 hp at 2:00 with no smell, and was worn
        // to 23 at 2:00 and down by 2:21 with it; pump_smell 2 to 8 was too
        // weak to see.
        Tier::Drifter => DoesToYou { says: "It smells your pumps and comes to eat their crystals", rules: &[("pump_smell", 20.0), ("lure_r", 15.0)], keys: &[] },
        Tier::Sprawler => DoesToYou { says: "Its fast threads smell your soldiers from far off", rules: SPRAWLER_RULES, keys: &[] },
        // The same smell on planet 3 (pumps hand): the first two pumps at
        // 185 and 167 hp at 2:30 without it, 160 and 160 with it (modest:
        // the Trunk's cords are slow to turn).
        Tier::Trunk => DoesToYou { says: "It runs its cords to your pumps", rules: &[("pump_smell", 20.0), ("lure_r", 15.0)], keys: &[] },
        Tier::Swarmer => DoesToYou { says: "Its surges charge your squad from far off, and fast", rules: SWARMER_RULES, keys: SWARMER_KEYS },
        Tier::Sporer => DoesToYou { says: "Struck, it scatters spores that start new slime", rules: &[], keys: SPORER_KEYS },
        Tier::Sleeper => DoesToYou { says: "It wakes at your first shots, and all of it comes", rules: &[], keys: SLEEPER_KEYS },
        _ => none,
    }
}

// The films behind the rows below: run seed 1, one seed each, health is
// the squad's mean at 2:00 (`does-to-you.md`).
// Sprawler, planet 2, idle: a soldier smells 4 over 4 dish cells
// (`Rules::smell` 0.5, `smell_r` 2): 2 soldiers at 13 health at 2:00 ->
// the squad gone by 1:30.
const SPRAWLER_RULES: &[(&str, f32)] = &[("smell", 4.0), ("smell_r", 4.0)];
// Swarmer, planet 4, attack-move: a surge reaches 25 cells past the gun's
// range (`Rules::panic_margin` 5) and lunges at 3x (`surge_haste` 2):
// health 74 -> 29. Tried and backfired: the alarm louder (alarm_relay 20,
// alarm_weight 6), health 74 -> 95 and kills 4,537 -> 878, the body
// drawn to the wounds instead of the squad.
const SWARMER_RULES: &[(&str, f32)] = &[("panic_margin", 25.0)];
const SWARMER_KEYS: &[(&str, &str)] = &[("surge_haste", "3")];
// Sporer, planet 5, attack-move: bursts at an alarm of 0.3, not the
// tier's 1, at 0.2 a step, not 0.05: 4 soldiers at 27 health -> none.
// Tried: burst 0.2 alone (health 20, 3 soldiers: weak); burst 0.5 with
// spore_rate 1e-4 (health 74: backfired, the spores left the fight).
const SPORER_KEYS: &[(&str, &str)] = &[("spore_burst", "0.2"), ("spore_alarm", "0.3")];
// Sleeper, planet 6, attack-move: wakes at an alarm of 0.3, not 1: one
// soldier at 14 health -> the squad gone by 1:30. Tried and backfired:
// wake_alarm 3 (a higher bar is a deeper sleep: 4 soldiers at 99).
const SLEEPER_KEYS: &[(&str, &str)] = &[("wake_alarm", "0.3")];

/// Points a won level pays: more the deeper it was and the more stars.
pub fn reward(depth: u32, stars: u8) -> u32 {
    150 + 50 * depth + 50 * u32::from(stars.saturating_sub(1))
}

/// What a shop row is.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Group {
    Squad,
    Build,
    Tools,
    /// Guns owned and for sale.
    Guns,
    /// Gun cards: in the gun, in the bag, for sale, and more on offer.
    Cards,
    Dev,
}

impl Group {
    pub fn name(self) -> &'static str {
        match self {
            Group::Squad => "Squad",
            Group::Build => "Buildings",
            Group::Tools => "Tools",
            Group::Guns => "Guns",
            Group::Cards => "Cards",
            Group::Dev => "Dev mode",
        }
    }
}

/// Something the shop sells by levels (an unlock is one level).
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct Item {
    pub key: &'static str,
    pub name: &'static str,
    pub group: Group,
    /// What one level does.
    pub about: &'static str,
    pub max: u8,
    /// The first level's price; each next level costs this much more.
    pub cost: u32,
    /// Another item that must be owned first.
    pub needs: Option<&'static str>,
}

/// Everything the shop sells but cards. Unlock keys (`max` 1) are what
/// `Run::has` answers for the window's command card.
/// The lure, the siren and the outpost are not sold (owner, 2026-11-15
/// 21:41Z, the whole-game review's D10): the outpost is out of the run,
/// and the lure and siren stay out until they visibly draw the slime. The
/// window hides them from a run's build card (`run_hidden`).
pub const ITEMS: [Item; 17] = [
    Item { key: "hp", name: "Armour", group: Group::Squad, about: "Soldiers take a quarter more biting before they fall", max: 4, cost: 120, needs: None },
    Item { key: "strike", name: "Heavier shots", group: Group::Squad, about: "Each shot kills a fifth more slime", max: 4, cost: 120, needs: None },
    Item { key: "rate", name: "Faster trigger", group: Group::Squad, about: "Soldiers fire 15% sooner", max: 4, cost: 120, needs: None },
    Item { key: "range", name: "Longer barrels", group: Group::Squad, about: "3 more cells of sight and reach", max: 3, cost: 100, needs: None },
    Item { key: "squad", name: "Bigger landing", group: Group::Squad, about: "Two more soldiers land with you", max: 4, cost: 150, needs: None },
    Item { key: "bank", name: "Supply drop", group: Group::Squad, about: "Land with 500 more food in the bank", max: 4, cost: 100, needs: None },
    Item { key: "harvest", name: "Quick hands", group: Group::Squad, about: "Soldiers take a quarter more from a crystal each second", max: 3, cost: 100, needs: None },
    Item { key: "walls", name: "Walls", group: Group::Build, about: "Unlocks laying walls", max: 1, cost: 100, needs: None },
    Item { key: "turret", name: "Turrets", group: Group::Build, about: "Unlocks the turret, a gun that does not move", max: 1, cost: 200, needs: None },
    Item { key: "turret+", name: "Turret power", group: Group::Build, about: "Turrets: 30% tougher, 3 cells more reach, fire 15% sooner", max: 3, cost: 150, needs: Some("turret") },
    // Pumps are the main income and never locked (owner, 2026-11-15:
    // "pumps do not start locked"); the shop sells their power instead.
    Item { key: "pump+", name: "Pump power", group: Group::Build, about: "Pumps draw a quarter more from their crystals each second", max: 3, cost: 150, needs: None },
    Item { key: "blast", name: "Blasts", group: Group::Tools, about: "Unlocks the thrown blast and its crater", max: 1, cost: 150, needs: None },
    Item { key: "mine", name: "Mines", group: Group::Tools, about: "Unlocks the mine, a blast the slime sets off", max: 1, cost: 150, needs: None },
    Item { key: "cord", name: "Det cord", group: Group::Tools, about: "Unlocks det cord, a line of blasts", max: 1, cost: 200, needs: None },
    Item { key: "gas", name: "Sleep gas", group: Group::Tools, about: "Unlocks sleep gas", max: 1, cost: 200, needs: None },
    Item { key: "tools+", name: "Cheaper tools", group: Group::Tools, about: "Every tool costs a fifth less", max: 3, cost: 120, needs: None },
    Item { key: "draw", name: "More cards on offer", group: Group::Cards, about: "One more gun card on offer in every shop", max: 2, cost: 100, needs: None },
];

/// What the run owns at the start: walls (the plainest defence).
pub const START: [&str; 1] = ["walls"];
/// The gun the run starts with: the average rifle (free play's gun,
/// budget 10 and 4 slots). The first build started on a budget of 5 that
/// "Bigger gun" raised; with rolled guns the budget grows by buying one.
pub fn first_gun() -> Gun {
    GunKind::Rifle.average_at(1)
}

/// Guns for sale in each shop.
pub const GUN_OFFER: usize = 2;

/// A gun's price: dearer the deeper it was found and the better it rolled
/// (`Gun::grade`, 1 average, squared so a lucky roll costs what it gives),
/// in tens.
pub fn gun_price(g: &Gun) -> u32 {
    let p = (80.0 + 40.0 * g.depth as f32) * g.grade.max(0.5).powi(2);
    ((p / 10.0).round() as u32).max(1) * 10
}
/// Cards on offer in a shop before `draw`: modifiers, triggers and
/// flight cards (the bullets are `AMMO_OFFER` rolled ones).
pub const OFFER: usize = 3;
/// Rolled bullets on offer in a shop (the gun thread's `Ammo::roll`,
/// #291: a shell, a set, damage, speed, range and spread, and a budget
/// cost that follows the roll). Before them the shop sold the fixed
/// payload cards (blast, gas, bomb) like any other.
pub const AMMO_OFFER: usize = 2;

/// The shells a shop's bullet may carry at `depth`: plain, blast and gas
/// from the start, the bomb from planet 4 (its old card's rarity), never
/// the germ (the GPU world casts none yet).
pub fn shells_at(depth: u32) -> Vec<Shell> {
    let mut v = vec![Shell::Plain, Shell::Blast, Shell::Gas];
    if depth >= power_from(3) {
        v.push(Shell::Bomb);
    }
    v
}

/// A rolled bullet's price in points: its shell's budget cost, how well
/// it rolled (`Ammo::grade`) and how deep it was found, in tens.
pub fn ammo_price(a: &Ammo) -> u32 {
    let p = (40.0 + 20.0 * a.shell.base() as f32) * a.grade().max(0.4).powf(1.5) * (1.0 + 0.1 * (a.depth.max(1) - 1) as f32);
    ((p / 10.0).round() as u32).max(1) * 10
}

/// A card's price in points: a rolled bullet's own, else its rarity's.
pub fn card_price(c: Card) -> u32 {
    match c {
        Card::Ammo(a) => ammo_price(&a),
        c => card_cost(card_power(c).unwrap_or(1)),
    }
}

/// A gun card's power, 1-3: the gun thread's `Card::tier` (common,
/// uncommon, rare), which depth first offers it and its price. None:
/// never sold (the germ: the GPU world casts none yet).
pub fn card_power(c: Card) -> Option<u8> {
    (c != Card::Germ).then(|| c.tier())
}

/// The first depth whose shop offers a card of this power.
pub fn power_from(power: u8) -> u32 {
    match power {
        1 => 1,
        2 => 2,
        _ => 4,
    }
}

pub fn card_cost(power: u8) -> u32 {
    [0, 80, 150, 250][power.min(3) as usize]
}

/// A row of the shop, for the menu to draw.
#[derive(Clone, Debug, PartialEq)]
pub struct Row {
    pub group: Group,
    /// What it shows: the window picks its icon from this.
    pub face: Face,
    /// The carried gun, or a card in it.
    pub on: bool,
    pub name: String,
    pub about: String,
    /// "150", or "" when it can't be bought again.
    pub price: String,
    /// "2/4", "owned", "in the gun", "in the bag".
    pub state: String,
    /// A pick does something now.
    pub ready: bool,
    pub pick: Pick,
}

/// What a row is, for its icon and its face on the page.
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum Face {
    /// `ITEMS` by key.
    Item(&'static str),
    Card(Card),
    Gun(Gun),
    Dev,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Pick {
    /// `ITEMS[i]`'s next level.
    Item(usize),
    /// Card on offer `i`.
    Offer(usize),
    /// Owned card `i`: into the gun or out of it, free.
    Card(usize),
    /// Owned card `i` into gun slot `slot` (0 the first the gun fires),
    /// the cards from there on one slot later; from the bag only if it
    /// fits (owner 20:32Z: "How do I actually build and adjust my guns ...
    /// I don't see how to move cards around").
    Move(usize, usize),
    /// Gun on offer `i`: bought, into the armory and carried.
    Gun(usize),
    /// Owned gun `i` carried instead, free.
    Carry(usize),
    /// Dev mode (owner, 11-14 15:33Z: "a dev mode to control what level
    /// planet i go to", unlimited money, auto reroll): the next planet one
    /// deeper or shallower.
    DevDepth(i32),
    /// Dev mode: 1,000 more points.
    DevPoints,
    /// Dev mode: a fresh draw of cards.
    DevReroll,
    /// The gun range: every card in the gun back to the bag.
    EmptyGun,
    /// The gun range: the bag emptied (the gun keeps its cards).
    EmptyBag,
}

/// One run.
#[derive(Clone, Debug, PartialEq)]
pub struct Run {
    pub seed: u64,
    /// The level being played or next to play, from 1.
    pub depth: u32,
    pub points: u32,
    /// Levels owned of each of `ITEMS`.
    pub owned: [u8; ITEMS.len()],
    /// Cards owned, and whether each is in the gun.
    pub cards: Vec<(Card, bool)>,
    /// This shop's cards on offer (a bought one is taken off).
    pub offers: Vec<Card>,
    /// Guns owned (the armory); never empty.
    pub guns: Vec<Gun>,
    /// The one carried: `guns[carry]`.
    pub carry: usize,
    /// This shop's guns for sale.
    pub gun_offers: Vec<Gun>,
    /// Stars won on each level so far.
    pub stars: Vec<u8>,
    /// The run is over (a level was lost) at this depth.
    pub over: bool,
    /// Dev mode: the shop has rows to jump planets, add points and redraw.
    pub dev: bool,
    /// Redraws so far (dev mode), so each draws anew.
    rerolls: u64,
    /// The run ended on a lost level.
    beaten: bool,
    /// The gun range (`Run::range`): a free armory, not a run.
    pub range: bool,
}

/// The depth the gun range rolls its guns and bullets for at first.
pub const RANGE_DEPTH: u32 = 3;

fn level_of(owned: &[u8; ITEMS.len()], key: &str) -> u8 {
    ITEMS.iter().position(|i| i.key == key).map_or(0, |k| owned[k])
}

/// A small seeded generator (splitmix64): the shop's draw must be the same
/// on every machine, so no floats.
fn mix(mut z: u64) -> u64 {
    z = z.wrapping_add(0x9E37_79B9_7F4A_7C15);
    z = (z ^ (z >> 30)).wrapping_mul(0xBF58_476D_1CE4_E5B9);
    z = (z ^ (z >> 27)).wrapping_mul(0x94D0_49BB_1331_11EB);
    z ^ (z >> 31)
}

impl Run {
    pub fn new(seed: u64) -> Run {
        let mut owned = [0; ITEMS.len()];
        for k in START {
            if let Some(i) = ITEMS.iter().position(|i| i.key == k) {
                owned[i] = 1;
            }
        }
        Run { seed, depth: 1, points: 0, owned, cards: Vec::new(), offers: Vec::new(), guns: vec![first_gun()], carry: 0, gun_offers: Vec::new(), stars: Vec::new(), over: false, dev: false, rerolls: 0, beaten: false, range: false }
    }

    /// The gun range (owner, 11-15 02:55Z: "I need a test environment
    /// built where I can play with the guns and cards and bullets"): a run
    /// whose shop is a free armory. Every gun kind is owned (its average),
    /// with one of each rolled for the depth on the rack; the shelf holds
    /// every card that is no bullet and a bullet of every payload and set
    /// (the standard set exact, the others rolled), free and never sold
    /// out. No upgrades, no planets: the window plays it on the sandbox.
    pub fn range(seed: u64) -> Run {
        let mut r = Run::new(seed);
        r.range = true;
        r.depth = RANGE_DEPTH;
        r.guns = GunKind::ALL.iter().map(|k| k.average()).collect();
        r.restock();
        r
    }

    /// Levels owned of the item `key` (0 for an unknown key).
    pub fn level_of(&self, key: &str) -> u8 {
        level_of(&self.owned, key)
    }

    /// The unlock `key` is owned (the window's lock on the command card).
    pub fn has(&self, key: &str) -> bool {
        self.level_of(key) > 0
    }

    /// The planet card for the current depth (name, biome, size), with no
    /// map built.
    pub fn planet(&self) -> Planet {
        levelgen::planet(self.seed, self.depth)
    }

    /// The level at the current depth.
    pub fn level(&self) -> Level {
        let d = self.depth;
        let seed = mix(self.seed ^ u64::from(d).wrapping_mul(0x5851_F42D));
        // The planet the level generator makes for this run and depth
        // (its name and biome come with it); `map` as `map=` names it, so
        // the window's title and alert read its name.
        let planet = self.planet();
        let map = format!("planet:{d}:{}", self.seed);
        let tier = match LADDER.get(d as usize - 1) {
            Some(&t) => t,
            None => LADDER[LADDER.len() - 3 + (mix(seed) % 3) as usize],
        };
        let shrink = shrink_goal(d);
        let mission = Mission {
            name: format!("Planet {d} of {PLANETS}, {}", planet.name),
            says: format!("Wipe out the slime: cut it to {:.0}% of its peak", 100.0 * shrink),
            map: map.clone(),
            tier: Some(tier.name().to_string()),
            goal: Goal::Shrink(shrink),
            verbs: Vec::new(),
            bonus: Some(Bonus::LoseAtMost(2)),
            // Free play's bite (2), not the missions' older 1: walking into
            // the slime is very deadly (owner, 11-12).
            // The clock grows with the planet (`clock`).
            rules: vec![("drain".into(), Rules::default().drain), ("cap".into(), clock(area_of(&planet)))],
            own: false,
        };
        // Walkers scale with the planet's area too (the level-gen thread,
        // 11-14 17:50Z: the same count covered a third of the smaller
        // planet 1 by 30 s, and would be a corner of a 6x core world).
        let area = area_of(&planet);
        let walkers = (1.0 + WALKERS_PER_DEPTH * (d - 1) as f32) * area * size_roll(seed);
        Level { depth: d, seed, ground: Ground::Planet { seed: self.seed, depth: d }, tier, walkers, mission, grow: head_start(area) }
    }

    /// The game's rules with the squad's and the buildings' upgrades.
    pub fn rules(&self, r: &mut Rules) {
        // What this planet's tier does to you (`does_to_you`); not in the
        // gun range, which plays no planet.
        if !self.range {
            for &(k, v) in does_to_you(self.level().tier).rules {
                assert!(r.set(k, v), "does_to_you: no rule {k}");
            }
        }
        let l = |k| f32::from(self.level_of(k));
        r.hp *= 1.0 + 0.25 * l("hp");
        r.strike *= 1.0 + 0.2 * l("strike");
        r.shot_every *= 0.85f32.powf(l("rate"));
        r.range += 3.0 * l("range");
        r.squad += 2 * self.level_of("squad") as usize;
        r.start_bank += 500.0 * l("bank");
        r.harvest *= 1.0 + 0.25 * l("harvest");
        let t = l("turret+");
        let p = l("pump+");
        let b = &mut r.build;
        b.pump_share *= 1.0 + 0.25 * p;
        b.pump_rate *= 1.0 + 0.25 * p;
        b.turret_hp *= 1.0 + 0.3 * t;
        b.turret_range += 3.0 * t;
        b.turret_every *= 0.85f32.powf(t);
    }

    /// The tools' prices with `tools+`.
    pub fn kit(&self, k: &mut KitRules) {
        let f = 0.8f32.powf(f32::from(self.level_of("tools+")));
        k.blast_price *= f;
        k.mine_price *= f;
        k.cord_price *= f;
        k.gas_price *= f;
        k.germ_price *= f;
    }

    /// The gun every soldier lands with.
    pub fn carried(&self) -> Gun {
        self.guns.get(self.carry).copied().unwrap_or_else(first_gun)
    }

    /// The carried gun as the squad fires it: its range and rate with the
    /// squad's "Longer barrels" and "Faster trigger" (`rules` puts the same
    /// levels on `Rules::range` and `shot_every`, which only the standard
    /// rifle reads; a squad armed with its own gun reads the gun's, so
    /// until 11-15 those two upgrades did nothing in a run).
    pub fn armed(&self) -> Gun {
        let l = |k| f32::from(self.level_of(k));
        let g = self.carried();
        Gun { range: g.range + 3.0 * l("range"), rate: g.rate / 0.85f32.powf(l("rate")), ..g }
    }

    /// The carried gun's card budget (`rounds::fit_gun`).
    pub fn budget(&self) -> u32 {
        self.carried().budget
    }

    /// The gun as it sits fits its budget and slots.
    fn fits_now(&self) -> bool {
        rounds::fit_gun(self.gun(), &self.carried()).is_ok()
    }

    /// Card `c` would fit in the gun beside what it holds, at its end. The
    /// cost depends on the order (a `Thrift` cuts the card after it), so a
    /// card put anywhere else is checked again where it lands (`put_in`,
    /// `move_to`).
    pub fn fits(&self, c: Card) -> bool {
        let mut g = self.gun();
        g.push(c);
        rounds::fit_gun(g, &self.carried()).is_ok()
    }

    /// The cards in the gun fit it: the last card in the gun that is no
    /// bullet comes out first, then the last bullet, so a smaller gun keeps
    /// something to fire. (Before the gun had an order of its own, 11-14,
    /// the last bought came out.)
    fn refit(&mut self) {
        while rounds::fit_gun(self.gun(), &self.carried()).is_err() {
            let last = |bullet: bool| self.cards.iter().rposition(|c| c.1 && (c.0.kind() == Kind::Payload) == bullet);
            match last(false).or_else(|| last(true)) {
                Some(i) => self.cards[i].1 = false,
                None => break,
            }
        }
    }

    /// The gun: the cards in it in the order they sit in `cards`, which is
    /// the order it fires them (a modifier works on the bullet after it).
    /// The player arranges it (`Pick::Move`); a card put in goes where
    /// `put_in` places it. Until 11-14 the gun was sorted, flight cards
    /// first, then the modifiers and triggers, then the payloads (#280),
    /// and the player could not change it; once every shot fired the next
    /// card in turn (#288) the owner asked how to move cards (20:32Z).
    pub fn gun(&self) -> Vec<Card> {
        self.cards.iter().filter(|c| c.1).map(|c| c.0).collect()
    }

    /// Where a card goes into the gun when nobody says: after the cards
    /// of its rank and before the first of a later one (flight cards,
    /// then modifiers and triggers, then bullets), as the sorted gun put
    /// it, so a bought `wide` still widens the first bullet. Returns
    /// where it went.
    fn put_in(&mut self, i: usize) -> usize {
        let rank = |c: Card| match c.kind() {
            Kind::Flight => 0,
            Kind::Modifier | Kind::Trigger => 1,
            Kind::Payload => 2,
        };
        let (c, _) = self.cards.remove(i);
        let at = match self.cards.iter().position(|o| o.1 && rank(o.0) > rank(c)) {
            Some(k) => k,
            None => self.cards.iter().rposition(|o| o.1).map_or(self.cards.len(), |k| k + 1),
        };
        self.cards.insert(at, (c, true));
        // Checked where it went: in front of a `Thrift`'s card it takes
        // the cut from that card (Thrift, Bomb, Germ on a budget of 7 took
        // a Quiet to cost 8). Over the budget it goes to the bag.
        if !self.fits_now() {
            self.cards[at].1 = false;
        }
        at
    }

    /// Offer `k` bought: straight into the gun when it fits the budget,
    /// else into the bag. Returns where it went in `cards`, or None when
    /// the points do not reach.
    fn buy(&mut self, k: usize) -> Option<usize> {
        let &c = self.offers.get(k)?;
        // The range's shelf is free and never sold out.
        if !self.range {
            let price = card_price(c);
            if price > self.points {
                return None;
            }
            self.points -= price;
            self.offers.remove(k);
        }
        let room = self.fits(c);
        self.cards.push((c, false));
        let last = self.cards.len() - 1;
        Some(if room { self.put_in(last) } else { last })
    }

    /// Offer `k` dragged from the shelf onto gun slot `slot`: bought and
    /// put there, or in the bag when the gun has no room for it. False
    /// when it can't be bought.
    pub fn buy_into(&mut self, k: usize, slot: usize) -> bool {
        let Some(i) = self.buy(k) else { return false };
        self.move_to(i, slot);
        true
    }

    /// Owned card `i` into gun slot `slot`, the rest after it moved on one;
    /// a slot past the last card is the end.
    fn move_to(&mut self, i: usize, slot: usize) -> bool {
        let Some(&(c, on)) = self.cards.get(i) else { return false };
        if !on && !self.fits(c) {
            return false;
        }
        let was = self.cards.clone();
        self.cards.remove(i);
        let ons: Vec<usize> = self.cards.iter().enumerate().filter(|(_, o)| o.1).map(|(k, _)| k).collect();
        let at = match ons.get(slot) {
            Some(&k) => k,
            None => ons.last().map_or(self.cards.len(), |&k| k + 1),
        };
        self.cards.insert(at, (c, true));
        // A move that takes a card out from behind a `Thrift`, or puts one
        // there, changes the cost: one that leaves the gun over its budget
        // does not happen.
        if !self.fits_now() {
            self.cards = was;
            return false;
        }
        true
    }

    /// The gun as the window's loadout text (`rounds::parse` reads it).
    pub fn gun_text(&self) -> String {
        rounds::name(&self.gun())
    }

    /// Fills the shop's offer for the current depth: `OFFER` cards plus
    /// `draw`'s, of the powers this depth has reached, not already owned
    /// (a card may be offered again once owned only if every card is).
    pub fn restock(&mut self) {
        if self.range {
            let z = mix(mix(self.seed ^ 0x7A46_E000) ^ u64::from(self.depth)) ^ self.rerolls;
            self.offers.clear();
            for (i, &shell) in Shell::ALL.iter().enumerate() {
                for (j, &set) in Set::ALL.iter().enumerate() {
                    let a = if set == Set::Standard { Ammo::standard(shell) } else { Ammo::roll_of(shell, set, mix(z ^ (16 * i + j) as u64), self.depth) };
                    self.offers.push(Card::Ammo(a));
                }
            }
            self.offers.extend(Card::ALL.into_iter().filter(|c| c.kind() != Kind::Payload));
            self.gun_offers = GunKind::ALL.iter().enumerate().map(|(k, &kind)| Gun::roll_kind(kind, mix(z ^ (0x6755_4E00 + k as u64)), self.depth)).collect();
            return;
        }
        let n = OFFER + self.level_of("draw") as usize;
        let pool: Vec<Card> = Card::ALL.into_iter().filter(|&c| c.kind() != Kind::Payload && card_power(c).is_some_and(|p| power_from(p) <= self.depth)).collect();
        let fresh: Vec<Card> = pool.iter().copied().filter(|c| !self.cards.iter().any(|o| o.0 == *c)).collect();
        let mut from = if fresh.is_empty() { pool } else { fresh };
        let mut z = mix(mix(mix(self.seed ^ 0xC0FF_EE00) ^ u64::from(self.depth)) ^ self.rerolls);
        self.offers.clear();
        // Rolled bullets first, each its own seed.
        let shells = shells_at(self.depth);
        for _ in 0..AMMO_OFFER {
            z = mix(z);
            let shell = shells[(z % shells.len() as u64) as usize];
            let set = Set::ALL[((z >> 20) % Set::ALL.len() as u64) as usize];
            self.offers.push(Card::Ammo(Ammo::roll_of(shell, set, mix(z ^ 0xA330), self.depth)));
        }
        while self.offers.len() < n + AMMO_OFFER && !from.is_empty() {
            z = mix(z);
            self.offers.push(from.remove((z % from.len() as u64) as usize));
        }
        // Guns for the planet ahead, each its own seed.
        self.gun_offers = (0..GUN_OFFER as u64).map(|k| Gun::roll(mix(z ^ (0x6755_4E00 + k)), self.depth)).filter(|g| !self.guns.contains(g)).collect();
    }

    /// The current level was won with `stars` (1-3): its points, the next
    /// depth, and a fresh shop. Winning the last planet wins the run.
    pub fn won(&mut self, stars: u8) {
        self.points += reward(self.depth, stars);
        self.stars.push(stars);
        if self.depth >= PLANETS {
            self.over = true;
            return;
        }
        self.depth += 1;
        self.restock();
    }

    /// The run ended on the home world's defeat.
    pub fn victory(&self) -> bool {
        self.over && !self.beaten
    }

    /// The current level was lost: the run is over.
    pub fn lost(&mut self) {
        self.over = true;
        self.beaten = true;
    }

    fn price(&self, i: usize) -> Option<u32> {
        let it = ITEMS[i];
        let l = self.owned[i];
        (l < it.max).then_some(it.cost * (u32::from(l) + 1))
    }

    /// The shop's rows: the items, the cards on offer, the cards owned;
    /// the gun range's has no items, a free shelf, and its own rows.
    pub fn shop(&self) -> Vec<Row> {
        let mut rows = Vec::new();
        for (i, it) in ITEMS.iter().enumerate().filter(|_| !self.range) {
            let l = self.owned[i];
            let price = self.price(i);
            let needs = it.needs.is_some_and(|k| !self.has(k));
            let state = if it.max == 1 {
                if l > 0 { "owned".to_string() } else { "locked".to_string() }
            } else {
                format!("{l}/{}", it.max)
            };
            let about = if needs { format!("{} (needs {})", it.about, it.needs.and_then(|k| ITEMS.iter().find(|x| x.key == k)).map_or("", |x| x.name)) } else { it.about.to_string() };
            rows.push(Row { group: it.group, face: Face::Item(it.key), on: false, name: it.name.into(), about, price: price.map_or(String::new(), |p| p.to_string()), state, ready: !needs && price.is_some_and(|p| p <= self.points), pick: Pick::Item(i) });
        }
        for (i, g) in self.guns.iter().enumerate() {
            let on = i == self.carry;
            rows.push(Row {
                group: Group::Guns,
                face: Face::Gun(*g),
                on,
                name: title(&g.title()),
                about: if on { "Your squad carries it.".into() } else { "Click to carry it instead.".into() },
                price: String::new(),
                state: if on { "carried".into() } else { "armory".into() },
                ready: !on,
                pick: Pick::Carry(i),
            });
        }
        for (i, g) in self.gun_offers.iter().enumerate() {
            let p = gun_price(g);
            let (price, state) = if self.range { (String::new(), "free".into()) } else { (p.to_string(), "for sale".into()) };
            rows.push(Row { group: Group::Guns, face: Face::Gun(*g), on: false, name: title(&g.title()), about: format!("From planet {}.", g.depth), price, state, ready: self.range || p <= self.points, pick: Pick::Gun(i) });
        }
        for (i, &c) in self.offers.iter().enumerate() {
            let p = card_price(c);
            let (price, state) = if self.range { (String::new(), "free".into()) } else { (p.to_string(), "for sale".into()) };
            rows.push(Row { group: Group::Cards, face: Face::Card(c), on: false, name: card_title(c), about: c.about().into(), price, state, ready: self.range || p <= self.points, pick: Pick::Offer(i) });
        }
        let mut slot = 0;
        for (i, &(c, on)) in self.cards.iter().enumerate() {
            let fits = self.fits(c);
            slot += usize::from(on);
            rows.push(Row {
                group: Group::Cards,
                face: Face::Card(c),
                on,
                name: card_title(c),
                // Its words only; how to click and drag is on the shop's
                // hint line (owner 11-15 02:57Z: the cards' text "is too
                // much"). Its slot is its state.
                about: if on || fits { c.about().into() } else { format!("{} Too big for the gun now.", c.about()) },
                price: String::new(),
                state: if on { format!("slot {slot}") } else { "in the bag".into() },
                ready: on || fits,
                pick: Pick::Card(i),
            });
        }
        if self.range {
            let row = |name: &str, about: String, pick| Row { group: Group::Dev, face: Face::Dev, on: false, name: name.into(), about, price: String::new(), state: String::new(), ready: true, pick };
            rows.push(row("New rolls", "Roll the shelf's bullets and the rack's guns again.".into(), Pick::DevReroll));
            rows.push(row("Deeper rolls", format!("Roll them as on planet {} (now {}).", (self.depth + 1).min(PLANETS), self.depth), Pick::DevDepth(1)));
            rows.push(row("Shallower rolls", format!("Roll them as on planet {} (now {}).", self.depth.saturating_sub(1).max(1), self.depth), Pick::DevDepth(-1)));
            rows.push(row("Empty the gun", "Every card in the gun back to the bag.".into(), Pick::EmptyGun));
            rows.push(row("Empty the bag", "Throw away the cards in the bag.".into(), Pick::EmptyBag));
        }
        if self.dev && !self.range {
            let dev = |name: &str, about: String, pick| Row { group: Group::Dev, face: Face::Dev, on: false, name: name.into(), about, price: String::new(), state: "dev".into(), ready: true, pick };
            rows.push(dev("Next planet deeper", format!("Land on planet {} instead", (self.depth + 1).min(PLANETS)), Pick::DevDepth(1)));
            rows.push(dev("Next planet shallower", format!("Land on planet {} instead", self.depth.saturating_sub(1).max(1)), Pick::DevDepth(-1)));
            rows.push(dev("1,000 points", "Free points".into(), Pick::DevPoints));
            rows.push(dev("Redraw the cards", "A fresh offer of gun cards".into(), Pick::DevReroll));
        }
        rows
    }

    /// A shop pick; false when it can't be done (too dear, maxed, locked).
    pub fn pick(&mut self, p: Pick) -> bool {
        match p {
            Pick::Item(i) => {
                let Some(price) = self.price(i).filter(|&c| c <= self.points) else { return false };
                if ITEMS[i].needs.is_some_and(|k| !self.has(k)) {
                    return false;
                }
                self.points -= price;
                self.owned[i] += 1;
                true
            }
            Pick::Offer(i) => self.buy(i).is_some(),
            Pick::Gun(i) => {
                let Some(&g) = self.gun_offers.get(i) else { return false };
                // The range swaps a rack gun for the owned one of its kind,
                // which goes on the rack (four of each, either way back).
                if let Some(k) = self.range.then(|| self.guns.iter().position(|o| o.kind == g.kind)).flatten() {
                    self.gun_offers[i] = std::mem::replace(&mut self.guns[k], g);
                    self.carry = k;
                    self.refit();
                    return true;
                }
                let price = gun_price(&g);
                if price > self.points {
                    return false;
                }
                self.points -= price;
                self.gun_offers.remove(i);
                self.guns.push(g);
                self.carry = self.guns.len() - 1;
                self.refit();
                true
            }
            Pick::Carry(i) => {
                if i >= self.guns.len() || i == self.carry {
                    return false;
                }
                self.carry = i;
                self.refit();
                true
            }
            Pick::DevDepth(by) if self.dev || self.range => {
                self.depth = (self.depth as i32 + by).clamp(1, PLANETS as i32) as u32;
                self.restock();
                true
            }
            Pick::DevPoints if self.dev => {
                self.points += 1000;
                true
            }
            Pick::DevReroll if self.dev || self.range => {
                self.rerolls += 1;
                self.restock();
                true
            }
            Pick::DevDepth(_) | Pick::DevPoints | Pick::DevReroll => false,
            Pick::EmptyGun | Pick::EmptyBag if !self.range => false,
            Pick::EmptyGun => {
                let any = self.cards.iter().any(|c| c.1);
                // In the bag in their order, after what was there.
                let (gun, bag): (Vec<_>, Vec<_>) = self.cards.drain(..).partition(|c| c.1);
                self.cards = bag.into_iter().chain(gun.into_iter().map(|c| (c.0, false))).collect();
                any
            }
            Pick::EmptyBag => {
                let n = self.cards.len();
                self.cards.retain(|c| c.1);
                self.cards.len() < n
            }
            Pick::Card(i) => {
                let room = self.cards.get(i).is_some_and(|c| self.fits(c.0));
                match self.cards.get(i) {
                    Some(c) if c.1 => {
                        self.cards[i].1 = false;
                        // A `Thrift` out puts its card back at full cost.
                        self.refit();
                        true
                    }
                    Some(_) if room => {
                        self.put_in(i);
                        true
                    }
                    _ => false,
                }
            }
            Pick::Move(i, slot) => self.move_to(i, slot),
        }
    }

    /// One line on the run so far, for the shop and the end.
    pub fn line(&self) -> String {
        let stars: u32 = self.stars.iter().map(|&s| u32::from(s)).sum();
        format!("{} planet{} freed, {stars} star{}", self.stars.len(), if self.stars.len() == 1 { "" } else { "s" }, if stars == 1 { "" } else { "s" })
    }
}

/// A card's name in the shop: a rolled bullet's own ("Needle gas").
fn card_title(c: Card) -> String {
    match c {
        Card::Ammo(a) => a.title(),
        c => title(c.name()),
    }
}

fn title(s: &str) -> String {
    let mut c = s.chars();
    c.next().map_or(String::new(), |f| f.to_uppercase().chain(c).collect())
}

#[cfg(test)]
mod tests {
    use super::*;

    /// Deeper, the slime surges no less often, lunges no shorter and
    /// thickens no later; planet 1 is gentler than the sandbox's shape C
    /// and the core world fiercer, and every key is one the engine takes.
    #[test]
    fn the_slime_gets_fiercer_deeper() {
        let get = |d: u32, k: &str| enemy_keys(d).into_iter().find(|(n, _)| *n == k).map(|(_, v)| v.parse::<f32>().unwrap()).unwrap();
        for d in 1..=10 {
            assert!(get(d + 1, "surge") <= get(d, "surge") && get(d + 1, "surge_dash") >= get(d, "surge_dash") && get(d + 1, "thicken") <= get(d, "thicken"), "planet {d}");
            assert_eq!(surge_secs(d), get(d, "surge") / 30.0);
            let mut p = crate::mould::big::BigParams::today();
            for (k, v) in enemy_keys(d) {
                crate::mould::tiers::set_key(&mut p, k, &v).unwrap();
            }
            assert!(p.surge_every > 0 && p.surge_dash > 0 && p.thicken > 0.0);
            assert!(surge_food(d + 1, 1.0) <= surge_food(d, 1.0) && surge_food(d, 2.0) == 2.0 * surge_food(d, 1.0), "planet {d}");
        }
        let c_food = crate::mould::larder_world::SURGE_FOOD;
        assert!(surge_food(1, 1.0) > c_food && surge_food(PLANETS, 1.0) < c_food);
        let (_, c) = crate::mould::larder_world::shape_keys("c").unwrap();
        let c_surge: f32 = c.iter().find(|(k, _)| *k == "surge").unwrap().1.parse().unwrap();
        assert!(get(1, "surge") > c_surge && get(PLANETS, "surge") < c_surge);
        assert!(get(PLANETS, "surge") < get(1, "surge") && get(PLANETS, "surge_dash") > get(1, "surge_dash") && get(PLANETS, "thicken") < get(1, "thicken"));
        // Breakaway from planet 2 on (D2): condensing and roaming together,
        // as roaming alone barely leaves a full body; none on planet 1.
        let has = |d: u32, k: &str| enemy_keys(d).iter().any(|(n, _)| *n == k);
        assert!(!has(1, "condense") && !has(1, "roam"));
        for d in 2..=PLANETS {
            assert!(get(d, "condense") > 0.0 && get(d, "roam") > 0.0, "planet {d}");
        }
    }

    #[test]
    fn levels_get_harder_and_the_same_seed_gives_the_same_run() {
        let mut r = Run::new(7);
        let mut walkers = 0.0;
        let mut goals = 1.0;
        for d in 1..=10 {
            r.depth = d;
            let l = r.level();
            // A cell's walkers grow deeper, so a smaller planet (a breather)
            // can have fewer in all.
            let dense = l.walkers / area_of(&r.planet()) / size_roll(l.seed);
            assert!((0.8..=1.25).contains(&size_roll(l.seed)));
            assert!(dense > walkers, "more walkers a cell deeper");
            assert!(shrink_goal(d) <= goals);
            (walkers, goals) = (dense, shrink_goal(d));
            assert_eq!(l, Run { depth: d, ..Run::new(7) }.level());
            assert_eq!(l.ground, Ground::Planet { seed: 7, depth: d });
            assert_eq!(levelgen::spec_planet(&l.mission.map, 0), Some(Ok(levelgen::planet(7, d))), "the window reads the planet back from the mission's map");
        }
        r.depth = 1;
        assert_eq!(r.level().tier, Tier::Drifter);
        // The level generator's core world is the run's last planet.
        assert_eq!(levelgen::CORE_DEPTH, PLANETS);
    }

    /// The planet review's finding 6 and 5 (11-15): the clock grows with
    /// the planet's area from 15 minutes, and the slime grows first on
    /// planets bigger than today's map.
    #[test]
    fn bigger_planets_get_a_longer_clock_and_a_grown_slime() {
        let at: Vec<(f32, f32)> = levelgen::AREA.iter().map(|&a| (clock(a), head_start(a))).collect();
        assert_eq!(at, [(900.0, 0.0), (990.0, 0.0), (1080.0, 0.0), (1260.0, 11.0), (1530.0, 25.0), (1890.0, 40.0), (2430.0, 60.0), (3330.0, 87.0)]);
        let mut r = Run::new(3);
        for d in 1..=PLANETS {
            r.depth = d;
            let (l, a) = (r.level(), area_of(&r.planet()));
            let cap = l.mission.rules(Rules::default()).cap;
            assert_eq!((cap, l.grow), (clock(a), head_start(a)), "planet {d}: the level's rules carry its clock");
        }
    }

    /// Planets at one depth differ in slime (owner, 11-15 08:34Z: "It
    /// should vary and be more later on"): the roll spans 0.8-1.25 over
    /// seeds, half of them each side of 1.
    #[test]
    fn a_planets_slime_size_varies_by_seed() {
        let rolls: Vec<f32> = (0..400).map(size_roll).collect();
        assert!(rolls.iter().all(|r| (0.8..=1.25).contains(r)));
        let (lo, hi) = (rolls.iter().cloned().fold(9.0, f32::min), rolls.iter().cloned().fold(0.0, f32::max));
        assert!(lo < 0.82 && hi > 1.22, "{lo} {hi}");
        let under = rolls.iter().filter(|&&r| r < 1.0).count();
        assert!((160..=240).contains(&under), "{under} of 400 under 1");
    }

    #[test]
    fn a_level_is_a_mission_the_campaign_would_take() {
        let l = Run::new(1).level();
        let text = format!("name {}\nsays {}\nmap {}\ntier {}\ngoal shrink {}\nbonus lose 2\nrule drain {}\nrule cap {}\n", l.mission.name, l.mission.says, l.mission.map, l.tier.name(), shrink_goal(1), Rules::default().drain, clock(0.6));
        assert_eq!(Mission::parse(&text).unwrap(), l.mission);
    }

    #[test]
    fn the_shop_sells_what_the_points_buy_and_no_more() {
        let mut r = Run::new(1);
        assert!(r.has("walls") && !r.has("turret"));
        assert!(r.shop().iter().all(|row| !row.ready || matches!(row.pick, Pick::Card(_))), "no points, nothing to buy");
        r.won(1);
        assert_eq!((r.depth, r.points), (2, reward(1, 1)));
        assert_eq!(r.offers.len(), OFFER + AMMO_OFFER);
        let turret = ITEMS.iter().position(|i| i.key == "turret").unwrap();
        let power = ITEMS.iter().position(|i| i.key == "turret+").unwrap();
        assert!(!r.pick(Pick::Item(power)), "turret power needs turrets");
        assert!(r.pick(Pick::Item(turret)));
        assert!(r.has("turret") && !r.pick(Pick::Item(turret)), "an unlock is bought once");
        assert_eq!(r.points, reward(1, 1) - 200);
        assert_eq!(reward(1, 1), 200, "the first win buys one unlock");
        assert!(!r.pick(Pick::Item(power)), "too dear now");
        // Levels rise in price.
        let hp = ITEMS.iter().position(|i| i.key == "hp").unwrap();
        r.points = 1000;
        assert!(r.pick(Pick::Item(hp)) && r.pick(Pick::Item(hp)));
        assert_eq!(r.points, 1000 - 120 - 240);
        let mut rules = Rules::default();
        r.rules(&mut rules);
        assert_eq!(rules.hp, 1.5 * Rules::default().hp);
    }

    /// The owner, 20:32Z: "How do I actually build and adjust my guns.
    /// When I buy a card it goes into the gun but I don't see how to move
    /// cards around." A card moves to any slot, the gun fires in slot
    /// order, a bag card drops into a slot only if it fits, and the shop
    /// numbers the slots.
    #[test]
    fn cards_move_to_any_slot_and_the_gun_fires_them_in_slot_order() {
        let mut r = Run::new(3);
        r.guns = vec![Gun { budget: 20, slots: 5, ..first_gun() }];
        r.cards = vec![(Card::Wide, true), (Card::Blast, true), (Card::Gas, true), (Card::Quiet, false)];
        assert_eq!(r.gun_text(), "wide+blast+gas");
        // Gas to the front: wide now widens the blast's next cast only
        // after gas.
        assert!(r.pick(Pick::Move(2, 0)));
        assert_eq!(r.gun_text(), "gas+wide+blast");
        // A slot past the end is the end; moving to its own slot changes nothing.
        let wide = r.cards.iter().position(|c| c.0 == Card::Wide).unwrap();
        assert!(r.pick(Pick::Move(wide, 9)));
        assert_eq!(r.gun_text(), "gas+blast+wide");
        let blast = r.cards.iter().position(|c| c.0 == Card::Blast).unwrap();
        assert!(r.pick(Pick::Move(blast, 1)));
        assert_eq!(r.gun_text(), "gas+blast+wide");
        // From the bag straight into slot 1, the rest moved on.
        let quiet = r.cards.iter().position(|c| c.0 == Card::Quiet).unwrap();
        assert!(r.pick(Pick::Move(quiet, 1)));
        assert_eq!(r.gun_text(), "gas+quiet+blast+wide");
        assert_eq!(rounds::casts(&r.gun()).len(), 2, "gas, then quiet blast; the trailing wide waits for a bullet");
        // A bag card that does not fit stays in the bag.
        r.guns = vec![Gun { budget: 9, slots: 5, ..first_gun() }];
        r.cards.push((Card::Bomb, false));
        assert!(!r.pick(Pick::Move(r.cards.len() - 1, 0)) && r.gun_text() == "gas+quiet+blast+wide");
        // The shop numbers the slots in firing order and says how to move.
        let rows: Vec<Row> = r.shop().into_iter().filter(|row| row.on && matches!(row.pick, Pick::Card(_))).collect();
        assert_eq!(rows.len(), 4);
        assert_eq!((rows[1].state.as_str(), rows[1].about.as_str()), ("slot 2", Card::Quiet.about()));
        assert!(rows.iter().map(|row| row.name.clone()).eq(["Gas", "Quiet", "Blast", "Wide"].map(String::from)));
        // A card for sale dragged onto slot 1: bought and put there; one
        // the gun has no room for is bought into the bag.
        r.guns = vec![Gun { budget: 20, slots: 6, ..first_gun() }];
        (r.offers, r.points) = (vec![Card::Pierce, Card::Bomb], 10_000);
        assert!(r.buy_into(0, 0));
        assert_eq!(r.gun_text(), "pierce+gas+quiet+blast+wide");
        r.guns = vec![Gun { budget: 9, slots: 6, ..first_gun() }];
        let n = r.cards.len();
        assert!(r.buy_into(0, 0) && r.offers.is_empty() && r.cards.len() == n + 1 && !r.cards[n].1, "bought into the bag");
        r.points = 0;
        r.offers = vec![Card::Wide];
        assert!(!r.buy_into(0, 0) && r.offers.len() == 1, "no points, no card");
    }

    /// The gun range (owner, 11-15 02:55Z: "I need a test environment
    /// built where I can play with the guns and cards and bullets"): every
    /// gun kind owned and one of each rolled on the rack, a bullet of every
    /// payload and set and every other card on a free shelf that never
    /// sells out, no upgrades; its rows roll again, roll deeper and empty
    /// the gun and the bag. A run has none of that.
    #[test]
    fn the_gun_range_is_a_free_armory() {
        let mut r = Run::range(7);
        assert!(r.guns.iter().map(|g| g.kind).eq(GunKind::ALL) && r.gun_offers.len() == GunKind::ALL.len());
        let others = Card::ALL.iter().filter(|c| c.kind() != Kind::Payload).count();
        assert_eq!(r.offers.len(), Shell::ALL.len() * Set::ALL.len() + others);
        for shell in Shell::ALL {
            for set in Set::ALL {
                assert!(r.offers.iter().any(|c| matches!(c, Card::Ammo(a) if a.shell == shell && a.set == set)), "{shell:?} {set:?}");
            }
        }
        assert_eq!(r.offers[0], Card::Ammo(Ammo::standard(Shell::Plain)), "the standard set is exact");
        let rows = r.shop();
        assert!(rows.iter().all(|row| !matches!(row.pick, Pick::Item(_)) && row.price.is_empty() && (row.ready || row.on)), "no upgrades, nothing to pay");
        // The same card twice, free, and still on the shelf.
        let n = r.offers.len();
        assert!(r.pick(Pick::Offer(0)) && r.pick(Pick::Offer(0)));
        assert_eq!((r.offers.len(), r.cards.len(), r.points), (n, 2, 0));
        let (owned, rack) = (r.guns[0], r.gun_offers[0]);
        assert!(r.pick(Pick::Gun(0)) && r.carry == 0 && r.guns[0] == rack && r.gun_offers[0] == owned, "a rack gun swaps for the owned one of its kind, free, and is carried");
        // New rolls change the rolled bullets, not the exact ones.
        let before = r.offers.clone();
        assert!(r.pick(Pick::DevReroll) && before != r.offers && before[0] == r.offers[0]);
        assert!(r.pick(Pick::DevDepth(1)) && r.depth == RANGE_DEPTH + 1);
        assert!(r.pick(Pick::EmptyGun) && r.gun().is_empty() && r.cards.len() == 2);
        assert!(r.pick(Pick::EmptyBag) && r.cards.is_empty());
        let mut run = Run::new(7);
        run.cards = vec![(Card::Wide, true)];
        assert!(!run.pick(Pick::EmptyGun) && !run.pick(Pick::DevReroll) && run.gun_text() == "wide");
        assert!(!run.shop().iter().any(|row| matches!(row.pick, Pick::EmptyGun | Pick::EmptyBag)));
    }

    #[test]
    fn cards_fill_the_gun_to_its_budget_and_get_better_deeper() {
        let mut r = Run::new(5);
        r.restock();
        assert!(r.offers.iter().all(|&c| card_power(c) == Some(1)), "planet 1 offers only the plainest cards");
        r.depth = 4;
        r.restock();
        assert!(r.offers.iter().all(|&c| c != Card::Germ));
        // Blast (3) and Wide (2) fill a budget of 5; Thick (1) waits in the bag.
        r.guns = vec![Gun { budget: 5, slots: 3, ..first_gun() }];
        r.points = 10_000;
        r.offers = vec![Card::Blast, Card::Wide, Card::Thick];
        for _ in 0..3 {
            assert!(r.pick(Pick::Offer(0)));
        }
        assert_eq!(r.gun_text(), "wide+blast", "a bought modifier goes in before the bullet");
        let at = |r: &Run, c: Card| r.cards.iter().position(|o| o.0 == c).unwrap();
        assert!(!r.cards[at(&r, Card::Thick)].1 && !r.pick(Pick::Card(at(&r, Card::Thick))), "over the budget");
        assert!(r.pick(Pick::Card(at(&r, Card::Wide))) && r.pick(Pick::Card(at(&r, Card::Thick))), "wide out, thick in");
        assert_eq!(r.gun_text(), "thick+blast");
        // A bigger gun bought is carried, and holds all three.
        r.gun_offers = vec![Gun { budget: 7, slots: 4, ..first_gun() }];
        let before = r.points;
        assert!(r.pick(Pick::Gun(0)) && r.carry == 1 && r.budget() == 7);
        assert_eq!(r.points, before - gun_price(&r.carried()));
        assert!(r.pick(Pick::Card(at(&r, Card::Wide))), "a bigger gun holds all three");
        assert!(rounds::fit_gun(r.gun(), &r.carried()).is_ok() && rounds::parse(&r.gun_text()).is_some(), "{}", r.gun_text());
        assert_eq!(r.gun_text(), "thick+wide+blast");
        // Back to the small gun: the last card that is no bullet comes out
        // (wide), so the gun keeps its blast.
        assert!(r.pick(Pick::Carry(0)) && !r.pick(Pick::Carry(0)));
        assert_eq!(r.gun_text(), "thick+blast");
        // Slots count too: three 1-cost cards and a fourth do not fit 3 slots.
        r.guns[0] = Gun { budget: 20, slots: 3, ..first_gun() };
        r.cards = vec![(Card::Thick, true), (Card::Quiet, true), (Card::Delay, true), (Card::Blast, false)];
        assert!(!r.fits(Card::Blast));
        // The gun fires its cards in the order they sit, which the player
        // sets (`Pick::Move`).
        r.cards = vec![(Card::Blast, true), (Card::Wide, true), (Card::Pierce, true)];
        assert_eq!(r.gun_text(), "blast+wide+pierce");
        // The deep shops reach the bomb.
        let deep = (4..40).any(|d| {
            let mut r = Run::new(d);
            r.depth = d as u32;
            r.restock();
            r.offers.iter().any(|c| matches!(c, Card::Ammo(a) if a.shell == Shell::Bomb))
        });
        assert!(deep);
    }

    #[test]
    fn deeper_shops_sell_guns_with_bigger_budgets() {
        let mean = |d: u32| {
            let budgets: Vec<f32> = (0..40u64)
                .flat_map(|seed| {
                    let mut r = Run::new(seed);
                    r.depth = d;
                    r.restock();
                    assert_eq!(r.gun_offers.len(), GUN_OFFER);
                    assert!(r.gun_offers.iter().all(|g| g.depth == d));
                    r.gun_offers.iter().map(|g| g.budget as f32 / g.kind.average().budget as f32).collect::<Vec<_>>()
                })
                .collect();
            budgets.iter().sum::<f32>() / budgets.len() as f32
        };
        assert!(mean(6) > 1.3 * mean(1), "{} vs {}", mean(6), mean(1));
        // The two on offer differ, and the same run sells the same guns.
        let mut a = Run::new(3);
        a.restock();
        assert_ne!(a.gun_offers[0], a.gun_offers[1]);
        let mut b = Run::new(3);
        b.restock();
        assert_eq!(a.gun_offers, b.gun_offers);
        // A better roll costs more.
        let g = GunKind::Carbine.average_at(3);
        assert!(gun_price(&Gun { grade: 1.3, ..g }) > gun_price(&g));
        assert!(Run::new(1).shop().iter().any(|r| r.on && r.face == Face::Gun(first_gun())), "the first gun is carried");
    }

    #[test]
    fn every_shop_rolls_bullets_and_prices_them_by_the_roll() {
        for d in 1..=8 {
            for seed in 0..20u64 {
                let mut r = Run::new(seed);
                r.depth = d;
                r.restock();
                let ammo: Vec<Ammo> = r.offers.iter().filter_map(|c| if let Card::Ammo(a) = c { Some(*a) } else { None }).collect();
                assert_eq!(ammo.len(), AMMO_OFFER);
                assert!(ammo.iter().all(|a| a.shell != Shell::Germ && (d >= 4 || a.shell != Shell::Bomb)), "{ammo:?}");
                assert!(r.offers.iter().all(|c| matches!(c, Card::Ammo(_)) || c.kind() != Kind::Payload), "the fixed payload cards are no longer sold");
                assert_eq!(r.offers.len(), AMMO_OFFER + OFFER);
            }
        }
        // A better roll of the same bullet costs more points.
        let a = Ammo::roll_of(Shell::Blast, Set::Needle, 1, 3);
        let better = (0..200u64).map(|k| Ammo::roll_of(Shell::Blast, Set::Needle, k, 3)).find(|b| b.grade() > a.grade() * 1.2);
        if let Some(b) = better {
            assert!(ammo_price(&b) >= ammo_price(&a));
        }
        // Bought, it goes into the gun like any card, by its own cost.
        let mut r = Run::new(4);
        r.restock();
        r.points = 10_000;
        let first = r.offers[0];
        assert!(r.pick(Pick::Offer(0)) && r.cards[0] == (first, true));
        assert!(rounds::parse(&r.gun_text()).is_some(), "a saved gun reads back: {}", r.gun_text());
    }

    #[test]
    fn dev_mode_jumps_planets_and_the_home_world_ends_the_run() {
        let mut r = Run::new(2);
        assert!(!r.pick(Pick::DevPoints), "not without dev mode");
        r.dev = true;
        let before = r.offers.clone();
        r.restock();
        assert!(r.pick(Pick::DevReroll) && r.pick(Pick::DevPoints) && r.points == 1000);
        assert!(r.offers != before || r.offers.is_empty());
        for _ in 0..20 {
            r.pick(Pick::DevDepth(1));
        }
        assert_eq!(r.depth, PLANETS);
        r.won(3);
        assert!(r.over && r.victory());
        let mut r = Run::new(2);
        r.lost();
        assert!(r.over && !r.victory());
    }

    #[test]
    fn every_ladder_tier_sets_on_the_gpu_engine() {
        for t in LADDER {
            let mut s = crate::gpu::larder_world::race_settings(1, 2, 5000);
            assert!(t.settings(&mut s).is_ok(), "{}", t.name());
        }
    }

    /// Every rung does something to you (`does_to_you`, owner 11-15
    /// 21:41Z), with rules the game takes and keys the GPU engine takes,
    /// and the run sets the rules on its planet but not in the gun range.
    #[test]
    fn every_ladder_tier_does_something_to_you() {
        for (d, t) in LADDER.iter().enumerate() {
            let does = does_to_you(*t);
            assert!(!does.says.is_empty() && (!does.rules.is_empty() || !does.keys.is_empty()), "{}", t.name());
            let mut s = crate::gpu::larder_world::race_settings(1, 2, 5000);
            t.settings(&mut s).unwrap();
            for &(k, v) in does.keys {
                assert!(crate::gpu::set(&mut s, k, v).is_ok(), "{} key {k}", t.name());
            }
            let mut run = Run::new(1);
            run.depth = d as u32 + 1;
            assert_eq!(run.level().tier, *t);
            let mut r = Rules::default();
            run.rules(&mut r);
            if *t == Tier::Drifter {
                assert_eq!((r.build.pump_smell, r.build.lure_r), (20.0, 15.0));
            }
        }
        let mut r = Rules::default();
        Run::range(1).rules(&mut r);
        assert_eq!(r.build.pump_smell, Rules::default().build.pump_smell);
    }

    /// The outpost, lure and siren are not sold in a run (owner 11-15
    /// 21:41Z, D10).
    #[test]
    fn the_run_shop_sells_no_outpost_lure_or_siren() {
        for k in ["outpost", "lure", "siren"] {
            assert!(ITEMS.iter().all(|i| i.key != k), "{k}");
        }
    }

    #[test]
    fn a_thrift_cannot_take_the_gun_over_its_budget() {
        let mut r = Run::new(1);
        r.guns[0].budget = 7;
        r.points = 9999;
        r.cards = vec![(Card::Thrift, true), (Card::Bomb, true), (Card::Germ, true)];
        r.offers = vec![Card::Quiet];
        r.pick(Pick::Offer(0));
        assert!(rounds::fit_gun(r.gun(), &r.carried()).is_ok(), "bought in: {:?} costs {}", r.gun(), rounds::cost(&r.gun()));
        let mut r = Run::new(1);
        r.guns[0].budget = 6;
        r.cards = vec![(Card::Thrift, true), (Card::Bomb, true), (Card::Germ, true)];
        r.pick(Pick::Card(0));
        assert!(rounds::fit_gun(r.gun(), &r.carried()).is_ok(), "thrift out: {:?} costs {}", r.gun(), rounds::cost(&r.gun()));
        // Dragged to the end the Thrift cuts nothing: the move is refused.
        let mut r = Run::new(1);
        r.guns[0].budget = 6;
        r.cards = vec![(Card::Thrift, true), (Card::Bomb, true), (Card::Germ, true)];
        assert!(!r.pick(Pick::Move(0, 9)));
        assert_eq!(r.gun(), vec![Card::Thrift, Card::Bomb, Card::Germ]);
    }

    #[test]
    fn barrels_and_trigger_reach_the_carried_gun() {
        let mut r = Run::new(1);
        let g = r.carried();
        r.points = 99_999;
        for k in ["range", "rate"] {
            let i = ITEMS.iter().position(|it| it.key == k).unwrap();
            assert!(r.pick(Pick::Item(i)));
        }
        let a = r.armed();
        assert!((a.range - (g.range + 3.0)).abs() < 1e-4, "{} then {}", g.range, a.range);
        assert!((a.rate - g.rate / 0.85).abs() < 1e-4, "{} then {}", g.rate, a.rate);
    }
}
