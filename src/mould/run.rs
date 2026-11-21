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
/// end the hardest three take turns. The Grudge joined at planet 7 on
/// 2026-11-17, once the GPU engine took its scar keys (the simulation-use
/// plan's step 0, `Reports/sim-use-2026-11-17/plan.md`), so planet 8 mixes
/// the Sporer, Sleeper and Grudge; the test below keeps every rung
/// playable.
pub const LADDER: [Tier; 7] = [Tier::Drifter, Tier::Sprawler, Tier::Trunk, Tier::Swarmer, Tier::Sporer, Tier::Sleeper, Tier::Grudge];

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

/// Each tier's slime as a factor on the size its depth, area and roll give
/// (`Run::level`); a tuning number, Direction's to set (the 11-17 plan).
///
/// The Sprawler 0.6 (Direction, 11-18, on the Enemy lane's planet 2 look,
/// `Reports/planet2-sprawler-2026-11-18/`): at full size planet 2's slime
/// is 23,000 to 25,000 walkers against planet 1's 13,000 and holds every
/// crystal over 130,000 charge, so on run seeds 1, 3 and 4 the run bot,
/// landing with 9 soldiers and nothing bought, had 3 to 9 soldiers at 2:30.
/// At 0.6 it had 18 to 24 on all three; at 0.8 only seed 1 held (14);
/// seed 2, which lands by a free crystal, had 30 and 28. A bot that backs
/// off farther, or the Sprawler's smell for soldiers turned down, gained a
/// little on 2 of 3 seeds and built up on none, so the size is the lever.
/// Its threads and its fire shyness (`does_to_you`) stay what planet 2
/// does to you; per cell it is now thinner than planet 1 (0.72 of it), so
/// the depth rule's "more later on" is kept apart from this factor
/// (`levels_get_harder_and_the_same_seed_gives_the_same_run`). Retune from
/// the owner's play.
pub fn tier_size(tier: Tier) -> f32 {
    match tier {
        Tier::Sprawler => 0.6,
        _ => 1.0,
    }
}

/// The planet's area as a share of today's map (`MAP_W` x `MAP_H`): 0.6 on
/// planet 1, 6 on the core world (`levelgen::AREA`).
pub fn area_of(p: &Planet) -> f32 {
    (p.size.0 * p.size.1) as f32 / (MAP_W * MAP_H) as f32
}

/// The level's old countdown (game seconds) for a planet of `area`
/// (`area_of`): 15 minutes on planet 1's 0.6 and 7.5 minutes more for
/// each whole map of area, in half minutes: 15:00, 16:30, 18:00, 21:00,
/// 25:30, 31:30, 40:30 and 55:30 on today's eight planets. Since
/// 2026-11-20 the clock counts up (owner, 14:10Z: "there should not be a
/// countdown clock"), and this is the measure `limit` and `par` scale from.
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

/// The speed star's time on planet `depth` of `area` (`Rules::par`, game
/// seconds): the goal met before it earns a star, and the clock counts up
/// to it. 8:00 on planet 1 and 10:00 on planets 2 and 3 (Direction,
/// 2026-11-20, for the owner's count-up clock: "stars ... are for being
/// under a certain time", 14:10Z; in line with Milestone 1's planets of
/// 8 to 12 minutes, about 30 for the three). Deeper planets scale planet
/// 3's 10:00 by their old clock, in half minutes: 11:30, 14:00, 17:30,
/// 22:30 and 31:00 on planets 4-8. Before, the star came at four fifths of
/// the countdown (12:00 on planet 1), which the balance review (11-20)
/// found near free: his planets took a median 4:05 on planet 1.
pub fn par(depth: u32, area: f32) -> f32 {
    match depth {
        0 | 1 => 480.0,
        2 | 3 => 600.0,
        _ => (600.0 * clock(area) / clock(levelgen::AREA[2]) / 30.0).round() * 30.0,
    }
}

/// When a stalemate ends a planet (`Rules::cap`, game seconds): twice the
/// old countdown, 30:00 on planet 1. The clock counts up and nothing shows
/// this until its last two minutes, so it ends only a planet neither side
/// can win: a slime the squad cannot finish and that cannot finish the
/// squad (owner's 11-19 planet 3 sat 18 minutes on 40 walkers it could not
/// kill). It is graded as the countdown was: cut to half the peak, 1 star
/// and the run goes on; else lost. The choice was Direction's to give
/// this thread (2026-11-20): a far limit, not none, so no planet can run
/// forever and the run bot's checks still end.
pub fn limit(area: f32) -> f32 {
    2.0 * clock(area)
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
/// 0 since 2026-11-18, so a planet is won when its last walker dies
/// (`End::MouldWiped`, which the campaign counts as the goal met). The
/// owner's play that day: "The game still ends too quick when you get
/// close to winning. There were still slime left." At 1% a planet 1 slime
/// of 13,000 was won with about 130 walkers on the map. The broken line
/// below still starves the stragglers off food, so what is left to shoot
/// is the slime on its crystals; the Trunk's walls are set walkers and
/// count. Direction's tuning number; the origin (Milestone 2) may become
/// the win instead.
///
/// 1% from the owner's 11-15 18:02Z ("We are still winning the game with
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
pub const WIN_SHARE: f32 = 0.0;

/// The slime's surges by planet, 1 to 8 (past the end the last): the food
/// it eats to build one, as a share of the sandbox's (`SURGE_FOOD`, 150,
/// times the level's walkers), the longest wait between them
/// (`BigParams::surge_every`, steps, 30 a game second), its lunge
/// (`surge_dash`, steps). The sandbox's shape C is 1 (150 food), 1800
/// (60 s) and 30. Until 11-16 the slime also thickened from 2 down to 1.3
/// births' energy deeper (`thicken`, the bronze); the owner, 11-16 04:58Z:
/// "Remove the bronze, keep the specks". Until food-driven surges (later on 11-15) the surges came on a
/// fixed beat of half these waits (40 s on planet 1 to 15 s on planet 8);
/// the food shares keep that curve's ratios to the sandbox's 30 s.
const SURGE_FOOD_SHARE: [f32; 8] = [1.33, 1.33, 1.0, 1.0, 0.83, 0.67, 0.67, 0.5];
const LONGEST_WAIT: [u32; 8] = [2400, 2400, 1800, 1800, 1500, 1200, 1200, 900];
/// The strikers' share by planet (`StrikerRules::share`; 2026-11-20 night,
/// Direction's numbers): 0.6 on planet 2, all of it from planet 3, as the
/// big-army problem is planets 2 and up. It scales the rate cap
/// (`StrikerRules::rate`) as well: about 90 strikers a second on planet 2,
/// 150 from planet 3, and a quarter (about 37 a second) on planet 1. Since
/// the same night the strikers wait for planet 2 (`FIRST_PLANET`), so
/// planet 1's quarter plays only when they are turned on there
/// (`set=strikers=1`, or `FIRST_PLANET` moved back); a 0 here would leave
/// them on and doing nothing (the bug check's note on #606).
const STRIKER_SHARE: [f32; 8] = [0.25, 0.6, 1.0, 1.0, 1.0, 1.0, 1.0, 1.0];
const SURGE_DASH: [u32; 8] = [30, 30, 30, 36, 42, 45, 45, 54];
/// **Specks: the slime sends pieces of itself off to fresh crystals,
/// most often early in the run** (2026-11-16, the slime-enemy thread).
/// The owner, 04:58Z, on the mini-boss questions: "Remove the bronze, keep
/// the specks (specks and mini-boss are separate but both happen. Specks
/// more often early game)". Breakaway (`roam`, steps between a walker's
/// chances; `roam_share`, the share of chances taken) now takes ordinary
/// walkers crowded or feeding at a crystal (`BigParams::roam`), so it
/// needs no condensing. Until 11-16 (#370, the whole-game review's
/// D2) planets 2-8 condensed (`condense` 300) and broke away bronze
/// pieces; breakaway alone over thickening moved nothing there
/// (`Reports/breakaway-run-2026-11-15/README.md`). Now every planet
/// sends specks, the first planets most: mini-bosses take over deeper.
/// A walker gets a chance every `ROAM` steps (300, 10 s, on every
/// planet) and a twentieth of the chances leave on planets 1-2, down to
/// a fiftieth on the core world. Measured on planet 2 of run 2 and planet
/// 1 of run 1 (squad hand, `Reports/specks-2026-11-16/README.md`): at
/// 0.05 about 1,000 walkers (6%) are on their way at once from 0:20, and
/// the slime is 7% (planet 2) and 10% (planet 1) heavier at 5:00 than
/// with none; at 0.1 about 1,900 on their way; at 0.5, the first try,
/// 4,600-9,500 (and 12,300 of 17,406 before a speck gave up after
/// `big::SPECK_TRIP`), most of the body.
const ROAM: [u32; 8] = [300; 8];
const ROAM_SHARE: [f32; 8] = [0.05, 0.05, 0.04, 0.04, 0.03, 0.03, 0.02, 0.02];

/// **The slime gets fiercer deeper** (2026-11-15, the slime-enemy thread;
/// the run thread agreed the curve): engine keys over shape C's on a run's
/// planet, before the tier's and the menu's own (`run_shape`). The slime
/// surges more often and lunges longer, so it presses harder; it sends
/// specks off to fresh crystals, most often on the first planets (`ROAM`,
/// `ROAM_SHARE`, 11-16). Planet 1's slime
/// needs a third more food for a surge than the sandbox's and waits up to
/// 80 s; the core world's half as much, waiting at most 30 s and lunging
/// 1.8 s. Why: the owner, 11-15 08:30Z, wants "more interesting
/// progression in the enemy ... and increase in challenge ... especially
/// when your guns get stronger"; until now depth grew only the walkers
/// and changed the tier. The surge's reach follows the guns already (the
/// gun's range plus `Rules::panic_margin`).
///
/// **The far smell is on on every planet** (`far_smell` 1, 2026-11-17,
/// the Enemy lane; the plan the owner approved at 19:07Z: "On by default now,
/// you veto in play"). Hungry slime leans down the walking distance to
/// crystals nothing has touched, and specks walk it, so the slime goes for
/// food it cannot yet smell instead of waiting in its bowl. With no squad
/// on planet 3 it reached 59 crystals in 2:30 against 47 (more on 7 of 8
/// seeds, none fewer) and ended bigger on 8 of 8, by up to 45%
/// (`Reports/sim-use-2026-11-17/s2-far-smell.md`): a harder planet, to be
/// retuned by the numbers above, not by turning this off. Until now it was
/// a key in `play-new-slime.cmd` only. Its field is built on a worker
/// thread every 2 game seconds; a hungry walker reads three more words a
/// step.
///
/// **A lure pulls on every planet** (`lure_pull` `LURE_PULL`, 2026-11-18,
/// the Enemy lane; the owner's card, 11-18 03:03Z: "Lure now, in the three
/// planets"). Calm slime within `Rules::lure_reach` of a Lure that is on
/// leans down the walking distance to it (`raids.rs`'s `time_lure_paths`).
///
/// **The slime holds together on its way out** (`tether` `TETHER`,
/// 2026-11-18, the Enemy lane; the owner's play notes that day): a
/// breakaway or a cut-off walker making for food steps only where it
/// touches others (engine.wgsl's `tethered`); a raid goes as one body by
/// its march (`RaidRules::march`).
pub fn enemy_keys(depth: u32) -> Vec<(&'static str, String)> {
    let i = depth_index(depth);
    vec![
        ("surge", LONGEST_WAIT[i].to_string()),
        ("surge_dash", SURGE_DASH[i].to_string()),
        ("roam", ROAM[i].to_string()),
        ("roam_share", format!("{}", ROAM_SHARE[i])),
        ("far_smell", "1".to_string()),
        ("lure_pull", format!("{LURE_PULL}")),
        ("tether", format!("{TETHER}")),
        // The Sporer's own spore numbers (`tiers::SPORE`) on every
        // planet: a fruit's spores homed by them until 11-21 (they no
        // longer home: engine.wgsl's `fly`), and no other spore flies on
        // these planets but the Sporer's, which already has them.
        ("spore_reach", "36".to_string()),
        ("spore_land", "1".to_string()),
    ]
}

/// **The walkers a marcher's next cell must touch** (the engine's
/// `tether`, 2026-11-18, the Enemy lane). The Lab's pick (one body,
/// `Reports/lab-body-2026-11-18`): on planet 1, run seed 1, 2 halved the
/// loose walkers in a calm breakaway (407 to 192); 3, 4 or side contact
/// did no better. On raiders it kept the raid home, so they are left out.
pub const TETHER: u32 = 2;

/// **How hard a lure pulls** (the engine's `lure_pull`), one number for
/// every tier, Direction's to tune. 10 is the lab's recommended start:
/// on planet 1 with three rendering turrets it made a steady stream into
/// the guns while the body stayed on its crystal (6,024 killed by 1:30
/// against 1,623 with no lure); 30 emptied the home colony in about 45 s
/// (`Reports/lab-steering-2026-11-17`). Those were measured with the far
/// smell off; here both are on.
pub const LURE_PULL: f32 = 10.0;

/// **Game rules every planet's slime plays by** (`Rules::set` names), set
/// in `Run::rules` before the tier's own (`does_to_you`). Today one:
/// charges go round walls (`unit_paths`, the simulation-use plan's S1; on
/// on every planet since 2026-11-17, with the far smell above). A surge or
/// a panic charge walks down the units' walking distance instead of
/// steering straight at a soldier, so a wall bends a charge to its gap
/// where it used to pile the slime against the wall: behind a cup of
/// walls half as many walkers piled at the wall and as many reached the
/// soldier (`Reports/sim-use-2026-11-17/s1-unit-paths.md`). The field is
/// built on a worker thread once a game second (2.0 ms on planet 3, 7.4 ms
/// on planet 8, off the frame).
///
/// **The slime fruits when it is hurt** (`fruit`, 2026-11-20, the thread
/// "Make the slime a more interesting enemy"; `larder_world/fruit.rs`). The
/// owner, 11-20 05:46Z: the slime is interesting early in a planet "when I
/// am poor and weak but once I get strong enough there is just lots of
/// shooting through masses of uniform orange". From 1:30 after the
/// landing, once the squad has killed a fifth of the slime's mass in 30 s,
/// the slime streams a twelfth of itself (at most 2,500 walkers)
/// into a dark knot out of the guns' reach. Break it within 30 s or it
/// bursts and throws its walkers over the planet as spores that land on
/// food as new slime. One at a time, 40 s apart at least. GPU only.
///
/// **The slime comes in waves when it is losing** (`wave`, 2026-11-20, the
/// same thread; `larder_world/waves.rs`). From 1:30, once the squad has
/// killed an eighth of the slime's mass in 20 s and the slime is shrinking,
/// the slime near the squad pulls back past the guns' reach to two or
/// three points round it, masses there for 12 s, and comes in from all of
/// them at once for 8 s. 45 s apart at least. GPU only.
///
/// **The slime beats** (`heart`, 2026-11-20, the same thread;
/// `larder_world/heart.rs`): every 6 s a beat runs out from its home
/// crystal (else the crystal it holds most) through the body, and the
/// calm slime leans out and quickens as it passes, so the edge swells
/// toward you and ebbs. GPU only.
///
/// **The slime condenses into strikers late in a planet** (`strikers`,
/// 2026-11-20, the owner's pick that day; `larder_world/strikers.rs`):
/// under 60% of its peak (retuned that evening from 40%), its edge within
/// the guns' reach and 30 dish cells of a soldier condenses once a second
/// against a squad of more than 8 (in full from 24), each condensing
/// walker taking in up to three neighbours; a condensed walker takes a
/// blow for each, bites once more for each, and charges the nearest
/// soldier at three times its pace. GPU only.
///
/// **A cornered slime lunges** (`late_at`, 2026-11-20, the balance thread,
/// the owner's same pick; `Game::keep_panic_reach`): under 40% of its
/// peak, for the rest of the planet its charges reach 25 dish cells past
/// the guns and its surges lunge at 3x, the Swarmer's numbers (on the
/// Swarmer it changes nothing). On with the strikers, at the same share,
/// as one cornered phase. Measured on the strategy league
/// (`Reports/late-threat-2026-11-20`): with it the slime bit no more of a
/// big army than without (planet 2: up on 1 map, down on 1, tied on 7;
/// planet 3: up on 1, down on 2, tied on 6), and nor did the strikers; on
/// with that said, for the owner to judge in play.
///
/// **The last-stand heart** (`stand`, 2026-11-20, the same thread;
/// `larder_world/stand.rs`): under 20% of its peak, against a squad of 24
/// or more, every walker left gathers into one armoured heart on its heart
/// crystal, which lunges at the nearest soldier every 20 s, until it is cut
/// under 15% of the most it held or is 150 s old. GPU only.
pub const ENEMY_RULES: &[(&str, f32)] = &[("unit_paths", 1.0), ("fruit", 1.0), ("wave", 1.0), ("heart", 1.0), ("strikers", 1.0), ("late_at", 0.4), ("stand", 1.0)];

/// **The planet each of `ENEMY_RULES` first plays on** (2026-11-20, the
/// thread "Pace threats across the planets"; the owner that night, after a
/// planet 1 on a good map: "We don't need all these mini-bosses, strikers,
/// etc. in planet one. We should save some thing for later planets", and
/// the strikers "still fucked my shit up (on planet 1)"). Every rule stays
/// on in the game; each waits for its planet, so each planet brings
/// something new:
///
/// - planet 1, the Drifter: the slime itself (surges, raids, specks, the
///   lure, the heartbeat);
/// - planet 2, the Sprawler: mini-bosses (`BossRules::for_depth`, since
///   11-18), fruiting, and the strikers, at `STRIKER_SHARE`'s 0.6;
/// - planet 3, the Trunk: waves and the late lunge, the slime fighting
///   back as a whole when it is losing, the Pinwheel (`BossRules`), and
///   the strikers in full;
/// - planet 4, the Swarmer: the slime's mortar (`BossRules`) and the
///   last-stand heart.
///
/// Before, all seven played from planet 1, so planet 1 had the strikers,
/// the waves, the late lunge and the heart that were built for the big
/// armies of planets 2 and up (`Reports/late-threat-2026-11-20`). A rule
/// missing here plays from planet 1. Direction's numbers to move.
///
/// Fruiting joined planet 2 on 11-21 (his play that night, 05:21Z, on a
/// build with the fruit fixes of #608: "I just played planet 1 and their
/// was mini-bosses. There were these events where the slime dispersed
/// across the map and surrounded me randomly, halving the squad each
/// time"). His log: at 8:44 the slime's mass rose 14,045 to 17,936 in 3 s
/// as a fruit burst, and 21 of 28 soldiers died in those 3 s; the dark
/// knot it grows from reads as a mini-boss.
pub const FIRST_PLANET: &[(&str, u32)] = &[("fruit", 2), ("strikers", 2), ("wave", 3), ("late_at", 3), ("stand", 4)];

/// The planet `rule` (one of `ENEMY_RULES`) first plays on (`FIRST_PLANET`).
pub fn first_planet(rule: &str) -> u32 {
    FIRST_PLANET.iter().find(|&&(k, _)| k == rule).map_or(1, |&(_, d)| d)
}

/// `ENEMY_RULES` on planet `depth`: each at its value from its planet
/// (`FIRST_PLANET`), 0 (off) before it.
pub fn enemy_rules(depth: u32) -> impl Iterator<Item = (&'static str, f32)> {
    ENEMY_RULES.iter().map(move |&(k, v)| (k, if depth.max(1) >= first_planet(k) { v } else { 0.0 }))
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

/// `DoesToYou` by tier: every tier does one thing (the plain mould is no
/// tier; the Grudge's row since 2026-11-17).
pub fn does_to_you(tier: Tier) -> DoesToYou {
    match tier {
        // A pump smells of food 20 times a soldier's scent over 15 dish
        // cells (`BuildRules::pump_smell`, `lure_r`, which only pumps use in
        // a run: lures are not sold). Planet 1 seed 1, pumps hand: the far
        // pump stood at 197 of 200 hp at 2:00 with no smell, and was worn
        // to 23 at 2:00 and down by 2:21 with it; pump_smell 2 to 8 was too
        // weak to see.
        Tier::Drifter => DoesToYou { says: "It smells your pumps and comes to eat their crystals", rules: &[("pump_smell", 20.0), ("lure_r", 15.0)], keys: &[] },
        Tier::Sprawler => DoesToYou { says: "Its fast threads smell your soldiers from far off, but it shies from fire. Bring Tinder and a Torch, and keep your soldiers out of the flames: fire burns them too", rules: SPRAWLER_RULES, keys: SPRAWLER_KEYS },
        // Until 2026-11-17 the Trunk had the Drifter's row (the same pump
        // smell: the first two pumps at 185 and 167 hp at 2:30 without it,
        // 160 and 160 with it; modest, as its cords are slow to turn), so
        // planets 1 and 3 did the same thing to you. Now it sets
        // (`TRUNK_KEYS`), and since 11-18 it walls the crystals it holds
        // (`fort` in `TRUNK_RULES`).
        Tier::Trunk => DoesToYou { says: "It walls the crystals it holds in amber, and sets where you shoot it. The walls take three hits and stop your soldiers. Fire burns them: bring a Torch", rules: TRUNK_RULES, keys: TRUNK_KEYS },
        Tier::Swarmer => DoesToYou { says: "Its surges charge your squad from far off, and fast. Cut down to a quarter, all of it rushes you", rules: SWARMER_RULES, keys: SWARMER_KEYS },
        Tier::Sporer => DoesToYou { says: "Struck, it scatters spores that start new slime", rules: &[], keys: SPORER_KEYS },
        Tier::Sleeper => DoesToYou { says: "It wakes at your first shots, and all of it comes", rules: &[], keys: SLEEPER_KEYS },
        Tier::Grudge => DoesToYou { says: "It remembers where you hurt it, and comes back there", rules: &[], keys: GRUDGE_KEYS },
    }
}

// The films behind the rows below: run seed 1, one seed each, health is
// the squad's mean at 2:00 (`does-to-you.md`).
// Sprawler, planet 2, idle: a soldier smells 4 over 4 dish cells
// (`Rules::smell` 0.5, `smell_r` 2): 2 soldiers at 13 health at 2:00 ->
// the squad gone by 1:30.
const SPRAWLER_RULES: &[(&str, f32)] = &[("smell", 4.0), ("smell_r", 4.0)];
// And it shies from fire (`fire_alarm` 50, the simulation-use plan's C4;
// the game designer 11-17 12:10Z and 14:38Z: as a switch for every slime
// it made every fire a fence, 0 burned while lit against 119, so it is one
// tier's trait, a weakness the player can use on the Sprawler's planet).
// At 50 the slime rings a fire and the fire eats holes in its edge (C4:
// 13% more kills on fearprobe, Reports/sim-use-2026-11-17/c4-fire-alarm.md).
// Run seed 1's planet 2 has nothing a plain light burns; the designer
// (15:26Z) keeps it here for that: Tinder, which brings its own fuel, is the
// answer, so the card says to bring it.
const SPRAWLER_KEYS: &[(&str, &str)] = &[("fire_alarm", "50")];
// **The Trunk sets** (2026-11-17, the Enemy lane; planet 3). The
// simulation-use plan's E1 and E2 were built on 11-17 behind four switches
// (the crust and its pull, setting, set slime blocking or trapping, fed
// slime fireproof), each waiting on its own blind film. The plan the owner
// approved at 19:07Z has the Enemy lane pick one configuration and give it
// to one tier as its trait, as fire shyness became the Sprawler's. This is
// it, on the planet its films were made on:
// - `crust=1`: ground the slime has held keeps a crust (full after a
//   minute of living, half gone two minutes after it leaves), which burns
//   as fuel (`crust_fuel` 3) and leaves barren char (`char_barren` 0.3).
// - `crust_weight` 8 and `crust_fast` 1: the Trunk keeps to its old ground
//   and steps faster on it (the +8 side of the pull's film: 43,366 walkers
//   and 73,244 cells ever held at 5:00, against 39,999 and 72,507 with no
//   pull; the -8 side spread it as a thin sheet over 36% more ground,
//   which is the Sprawler's shape, not the Trunk's).
// - `setting=1`: a walker on crust where the scar says it was hurt stops,
//   stops breeding and takes three blows (`set_armour` 3), and wakes when
//   the fire has been off it for about a minute. A standing rifle squad
//   killed 39% less in 45-75 s, a squad that moved to a fresh front 34%
//   less, and soldiers lived longer (7 at 90 s against 5 and 4), since set
//   slime does not come on: the Trunk is the slime that holds its ground
//   (`Reports/sim-use-2026-11-17/e2a-setting.md`).
// - `set_block` 5: set slime is a wall to soldiers (`larder_world/bulwark.rs`;
//   the owner, 11-17 01:13Z: bulwarks block "Both" shots and soldiers). The
//   trap (`set_hold`) lost the A/B and was deleted.
// - `fire_fed=1`: the fed body is fireproof whatever its trail (the owner
//   agreed to the plan's "fed slime is fireproof, starving slime burns" in
//   the same answer); set and cut-off slime still burn (`fire_dry`), so
//   fire is the answer to a wall your shots made. A fed disc lost 24-65% to
//   fire in 20 s without it and nothing on 7 of 8 seeds with it
//   (`c1-dry-burns.md`). Checked on planet 3, run seed 1: two Torch shots
//   on a 1,212-cell wall left 679 cells two seconds later, against 1,284
//   with no fire, and the fed body beside it did not catch
//   (`Reports/enemy-switches-2026-11-17/README.md`).
// GPU engine keys only (`gpu::set`'s `crust` and `setting` set several
// numbers at once; the CPU setter has no such names, and new slime work is
// GPU only since the owner's 11-17 19:21Z answer).
const TRUNK_RULES: &[(&str, f32)] = &[("set_block", 5.0), ("fort", 3.0)];
const TRUNK_KEYS: &[(&str, &str)] = &[("crust", "1"), ("crust_weight", "8"), ("crust_fast", "1"), ("setting", "1"), ("fire_fed", "1")];
// Swarmer, planet 4, attack-move: a surge reaches 25 cells past the gun's
// range (`Rules::panic_margin` 5) and lunges at 3x (`surge_haste` 2):
// health 74 -> 29. Tried and backfired: the alarm louder (alarm_relay 20,
// alarm_weight 6), health 74 -> 95 and kills 4,537 -> 878, the body
// drawn to the wounds instead of the squad.
// And its last stand (`RaidRules::last_stand`, 2026-11-17, the Enemy
// lane): once, when it falls under a quarter of its peak
// (`last_stand_at`), every walker it has left rushes the thickest knot of
// soldiers. Built 11-15 as a switch for every slime and left off (the owner,
// 21:15Z: "a slime that is losing might rush you will everything it has.
// Not sure if this is a good idea or not"); it is the charging slime's
// way to lose, so it is this tier's and no other's.
const SWARMER_RULES: &[(&str, f32)] = &[("panic_margin", 25.0), ("last_stand", 1.0)];
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
// Grudge, planet 7 (joined 2026-11-17, when the GPU took its scar): its
// memory of where it was hit pulls it back at twice the campaign tier's
// weight (scar_weight 1, not 0.5), so a firing line it was hurt from draws
// it back after the alarm has faded. A first number, not yet filmed on a
// planet (`Reports/sim-use-2026-11-17/step0-scar-on-gpu.md`).
const GRUDGE_KEYS: &[(&str, &str)] = &[("scar_weight", "1")];

/// The card a tier's planet asks you to bring, if any: the card that
/// answers its trait (the game designer, 2026-11-17 15:26Z, with the guns
/// thread's rule). The Sprawler shies from fire (`fire_alarm`, #434), but
/// run seed 1's planet 2 has nothing a plain light burns, so its trait only
/// shows against fire that brings its own fuel: Tinder. When the next
/// planet names a card and you don't own it, it takes one of the shop's
/// card slots (`Run::restock`), so the planet card's advice can be taken.
pub fn bring(tier: Tier) -> Option<Card> {
    match tier {
        Tier::Sprawler => Some(Card::Tinder),
        _ => None,
    }
}

/// The bullet a tier's planet asks you to bring, if any (2026-11-18, the
/// Squad and arsenal lane: the shops before planets 2 and 3 should make
/// those planets play differently). When you own no bullet of that shell
/// it is the shop's first bullet on every reroll (`Run::restock`).
///
/// - The Sprawler: a Torch. Tinder lays brush and kills nothing until a
///   fire runs along it, and the run starts with plain bullets: over run
///   seeds 1-12 the shop that pushed Tinder sold a fire (a torch or a
///   blast) on 3, so on 9 the planet's own advice was a dead buy.
/// - The Trunk: a Torch. Shot, it sets into amber walls that take three
///   hits each and that fire burns (the Enemy lane, 11-17: two torch shots
///   cut a 1,212-cell wall to 679 in 2 s).
pub fn bring_shell(tier: Tier) -> Option<Shell> {
    match tier {
        Tier::Sprawler | Tier::Trunk => Some(Shell::Torch),
        _ => None,
    }
}

/// Points a won level pays: more the deeper it was and the more stars.
///
/// 175 + 25 a planet since the starfall (Direction, 2026-11-20): ore
/// worth 30 + 10 a planet falls at least three times a planet, about 4.2
/// on average (`StarRules::for_depth`, `fall_count`), so what grows with
/// depth is now mostly what the squad goes out and takes. Planet 1 still
/// pays 200, which the first unlock and the Torch-and-Tinder pair before
/// planet 2 are priced on (`ammo_price`); from planet 2 the base pays 25
/// less a planet than the old 150 + 50 a planet (planet 3: 250, not 300;
/// planet 8: 375, not 550). Rejected: the brainstorm's 100 + 25 a planet,
/// which takes the planet 1 floor under both buys while no bot collects
/// drops, so nothing yet says how much of the ore a player takes.
pub fn reward(depth: u32, stars: u8) -> u32 {
    175 + 25 * depth + 50 * u32::from(stars.saturating_sub(1))
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
/// The siren and the outpost are not sold (owner, 2026-11-15 21:41Z, the
/// whole-game review's D10): the outpost is out of the run, and the siren
/// stays out until it visibly draws the slime. The window hides them from
/// a run's build card (`run_hidden`). The lure was out with them until the
/// Enemy lane's lure field met that condition for it (the lab's steering
/// prototype; owner's card, 2026-11-18 03:03Z: "Lure now, in the three
/// planets"). It is not sold either: for an hour it was an unlock here
/// (Lures, 150), which left planet 1 without it, so Direction made it free
/// from the first planet (11-18 08:38Z). The window never locks it
/// (`run_lock`), as it never locks the pump.
pub const ITEMS: [Item; 18] = [
    Item { key: "hp", name: "Armour", group: Group::Squad, about: "Soldiers take a quarter more biting before they fall", max: 4, cost: 120, needs: None },
    Item { key: "strike", name: "Heavier shots", group: Group::Squad, about: "Shots a fifth wider; past the widest, they fire faster", max: 4, cost: 120, needs: None },
    Item { key: "rate", name: "Faster trigger", group: Group::Squad, about: "Soldiers fire 15% sooner", max: 4, cost: 120, needs: None },
    Item { key: "range", name: "Longer barrels", group: Group::Squad, about: "3 more cells of sight and reach", max: 3, cost: 100, needs: None },
    Item { key: "squad", name: "Bigger landing", group: Group::Squad, about: "Two more soldiers land with you", max: 4, cost: 150, needs: None },
    Item { key: "bank", name: "Supply drop", group: Group::Squad, about: "Land with 500 more food in the bank", max: 4, cost: 100, needs: None },
    Item { key: "harvest", name: "Quick hands", group: Group::Squad, about: "Soldiers take a quarter more from a crystal each second", max: 3, cost: 100, needs: None },
    Item { key: "walls", name: "Walls", group: Group::Build, about: "Unlocks laying walls", max: 1, cost: 100, needs: None },
    Item { key: "turret", name: "Turrets", group: Group::Build, about: "Unlocks the turret, a gun that does not move", max: 1, cost: 200, needs: None },
    // The tree's first body (the turrets thread, owner 2026-11-17 00:47Z):
    // after turrets, as each next body will be after the one before.
    Item { key: "mortar", name: "Mortars", group: Group::Build, about: "Unlocks the mortar: the longest reach there is, lobbed over walls", max: 1, cost: 250, needs: Some("turret") },
    Item { key: "turret+", name: "Turret power", group: Group::Build, about: "Turrets and mortars: 30% tougher, 3 cells more reach, fire 15% sooner", max: 3, cost: 150, needs: Some("turret") },
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
/// from the start, the bomb and the germ from planet 4 (their old cards'
/// rarity). The germ was never sold until 2026-11-16 ("the GPU world casts
/// none yet"), though the GPU world has cast it since 11-14
/// (`GpuWorld::has_germ`; the gun-system review's step A). The Mind
/// bullets (blind, sour, spin; step D, 2026-11-16) from planet 2, their
/// cards' rarity, and the readers with what they read: shatter with the
/// Mind bullets (gas is sold from the start), detonate with the germ. The
/// flare and the torch (step D's route and element families, 11-17) from
/// planet 2, their cards' rarity, and so Provoke and Still (the Mind
/// family's last two, 11-17).
///
/// Gas from planet 2 since 2026-11-18 (Direction: its card is uncommon now,
/// `Card::tier`); it was sold from the start.
pub fn shells_at(depth: u32) -> Vec<Shell> {
    let mut v = vec![Shell::Plain, Shell::Blast];
    if depth >= power_from(2) {
        v.extend([Shell::Gas, Shell::Blind, Shell::Sour, Shell::Spin, Shell::Provoke, Shell::Still, Shell::Shatter, Shell::Flare, Shell::Torch]);
    }
    if depth >= power_from(3) {
        v.extend([Shell::Bomb, Shell::Germ, Shell::Detonate]);
    }
    v
}

/// A rolled bullet's price in points: its shell's budget cost, how well
/// it rolled (`Ammo::grade`) and how deep it was found, in tens.
///
/// A Torch never costs more than `TORCH_MOST` (Direction, 2026-11-18): the
/// shop before planet 2 pushes Tinder and a Torch together (`bring`,
/// `bring_shell`), and the pair must be affordable on the least a won
/// planet 1 pays (`reward(1, 1)`, 200). A Torch rolled 40 to 150 before.
pub fn ammo_price(a: &Ammo) -> u32 {
    let p = (40.0 + 20.0 * a.shell.base() as f32) * a.grade().max(0.4).powf(1.5) * (1.0 + 0.1 * (a.depth.max(1) - 1) as f32);
    let p = ((p / 10.0).round() as u32).max(1) * 10;
    if a.shell == Shell::Torch {
        p.min(TORCH_MOST)
    } else {
        p
    }
}

/// The most a Torch bullet costs, and Tinder's price (it was its rarity's
/// 150): Direction's first numbers, 2026-11-18 (`ammo_price`).
pub const TORCH_MOST: u32 = 100;
pub const TINDER_PRICE: u32 = 100;

/// A card's price in points: a rolled bullet's own, else its rarity's.
pub fn card_price(c: Card) -> u32 {
    match c {
        Card::Ammo(a) => ammo_price(&a),
        Card::Tinder => TINDER_PRICE,
        c => card_cost(card_power(c).unwrap_or(1)),
    }
}

/// A gun card's power, 1-3: the gun thread's `Card::tier` (common,
/// uncommon, rare), which depth first offers it and its price. None was
/// never sold: the germ, until 2026-11-16 (`shells_at`).
pub fn card_power(c: Card) -> Option<u8> {
    Some(c.tier())
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
    /// The shop's gun slots switch between the squad's gun and the
    /// turrets' deck (the turrets thread, 2026-11-16): the same bag feeds
    /// both, so a card on the turrets is one the squad doesn't fire. Since
    /// loadouts (2026-11-19) it switches between loadout 1 and the one the
    /// turrets use (`Run::turret_loadout`).
    EditTurrets,
    /// The gun slots show loadout `i` (`Run::loadouts`).
    Edit(usize),
    /// A new, empty loadout with no gun, shown in the gun slots.
    NewLoadout,
    /// New turrets use the loadout shown (`Run::turret_loadout`).
    TurretsUse,
    /// In the field (`Run::field`), the squad the panel was opened for uses
    /// the loadout shown (`Run::field_squad`).
    SquadUses,
}

/// A loadout (owner, 2026-11-19 19:22Z): a gun, or none, and its cards.
/// Each squad and each turret uses one, and a change to it changes all of
/// them. A gun and a card are in one loadout at a time; a turret fires its
/// cards and has no use for the gun (its body sets its range and rate).
#[derive(Clone, Debug, Default, PartialEq)]
pub struct Loadout {
    /// Its gun, an index into `Run::guns`; None fits its cards to the
    /// turrets' rack (`Run::turret_rack`), and a squad given it fires the
    /// standard rifle.
    pub gun: Option<usize>,
    /// Its cards in firing order, while another loadout is shown in the
    /// gun slots (the one shown keeps them in `Run::cards`).
    cards: Vec<Card>,
}

/// The most loadouts a run holds.
pub const LOADOUTS_MAX: usize = 6;

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
    /// The loadouts (`Loadout`): squads land on the first; never empty.
    pub loadouts: Vec<Loadout>,
    /// The loadout the shop's gun slots show; its cards are `cards`' `on`.
    pub editing: usize,
    /// The loadout new turrets use (owner 11-19: "a default loadout for
    /// turrets", then each turret changed one by one in the game).
    pub turret_loadout: usize,
    /// The shop is the loadout panel opened on a planet (H): only the guns
    /// and cards owned, nothing for sale, the game paused under it.
    pub field: bool,
    /// The squad the panel was opened for and the loadout it uses, which
    /// `Pick::SquadUses` changes; the window applies it on closing.
    pub field_squad: Option<(u8, usize)>,
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
        Run { seed, depth: 1, points: 0, owned, cards: Vec::new(), offers: Vec::new(), guns: vec![first_gun()], loadouts: vec![Loadout { gun: Some(0), cards: Vec::new() }, Loadout::default()], editing: 0, turret_loadout: 1, field: false, field_squad: None, gun_offers: Vec::new(), stars: Vec::new(), over: false, dev: false, rerolls: 0, beaten: false, range: false }
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
        // The range's one squad fires one loadout.
        r.loadouts.truncate(1);
        r.turret_loadout = 0;
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
            says: if shrink > 0.0 { format!("Wipe out the slime: cut it to {:.0}% of its peak", 100.0 * shrink) } else { "Wipe out the slime: kill every last walker".to_string() },
            map: map.clone(),
            tier: Some(tier.name().to_string()),
            goal: Goal::Shrink(shrink),
            verbs: Vec::new(),
            bonus: Some(Bonus::LoseAtMost(2)),
            // Free play's bite (2), not the missions' older 1: walking into
            // the slime is very deadly (owner, 11-12).
            // The clock counts up to the speed star (`par`); only a
            // stalemate meets the far limit (`limit`).
            rules: vec![("drain".into(), Rules::default().drain), ("cap".into(), limit(area_of(&planet))), ("par".into(), par(d, area_of(&planet)))],
            own: false,
        };
        // Walkers scale with the planet's area too (the level-gen thread,
        // 11-14 17:50Z: the same count covered a third of the smaller
        // planet 1 by 30 s, and would be a corner of a 6x core world).
        let area = area_of(&planet);
        let walkers = (1.0 + WALKERS_PER_DEPTH * (d - 1) as f32) * area * size_roll(seed) * tier_size(tier);
        Level { depth: d, seed, ground: Ground::Planet { seed: self.seed, depth: d }, tier, walkers, mission, grow: head_start(area) }
    }

    /// The game's rules with the squad's and the buildings' upgrades.
    pub fn rules(&self, r: &mut Rules) {
        // What this planet's tier does to you (`does_to_you`); not in the
        // gun range, which plays no planet.
        if !self.range {
            // Mini-bosses (2026-11-16, `larder_world/bosses.rs`): their
            // chance rises with the planet (owner, 04:58Z: "Chance increases
            // with progression into the run"); a tier's own rules after.
            r.boss = crate::mould::larder_world::BossRules::for_depth(self.depth);
            // What every planet's slime does from its planet on
            // (`ENEMY_RULES`, `FIRST_PLANET`).
            for (k, v) in enemy_rules(self.depth) {
                assert!(r.set(k, v), "ENEMY_RULES: no rule {k}");
            }
            // The planet's share scales the rate cap too, since the cap,
            // not the share, binds wherever the band is big (planet 1 run
            // seed 1: 16 soldiers lost and won at a quarter of the cap, 17
            // at the full cap, 43 and lost uncapped, 11 with no strikers).
            let k = STRIKER_SHARE[depth_index(self.depth)];
            r.strikers.share = k;
            r.strikers.rate = crate::mould::larder_world::StrikerRules::default().rate * k;
            for &(k, v) in does_to_you(self.level().tier).rules {
                assert!(r.set(k, v), "does_to_you: no rule {k}");
            }
            // Starfall and boss cores (2026-11-20, `larder_world/starfall.rs`;
            // the owner, 19:06Z: "I like starfall with boss cores"): shop
            // points the squad takes on the planet, on top of `reward`.
            r.star = crate::mould::larder_world::StarRules::for_depth(self.depth);
        }
        // A run is lost when its last soldier falls, food in the bank or
        // not (owner, 2026-11-18 15:56Z; `Rules::wipe_ends`).
        r.wipe_ends = 1.0;
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
        // Mortars are turrets' bodies: the same power (the turrets thread).
        b.mortar_hp *= 1.0 + 0.3 * t;
        b.mortar_range += 3.0 * t;
        b.mortar_every *= 0.85f32.powf(t);
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

    /// The gun shown in the shop's gun slots (the edited loadout's), else
    /// the first loadout's, else the rifle the run starts with.
    pub fn carried(&self) -> Gun {
        self.gun_of(self.editing).or_else(|| self.gun_of(0)).unwrap_or_else(first_gun)
    }

    /// The edited loadout's gun, an index into `guns`.
    pub fn carry(&self) -> Option<usize> {
        self.loadouts.get(self.editing).and_then(|l| l.gun)
    }

    /// Loadout `i`'s gun, as rolled.
    pub fn gun_of(&self, i: usize) -> Option<Gun> {
        self.loadouts.get(i).and_then(|l| l.gun).and_then(|g| self.guns.get(g)).copied()
    }

    /// Loadout `i`'s gun as the squad fires it: its range and rate with
    /// the squad's "Longer barrels" and "Faster trigger" (`rules` puts the
    /// same levels on `Rules::range` and `shot_every`, which only the
    /// standard rifle reads; a squad armed with its own gun reads the
    /// gun's, so until 11-15 those two upgrades did nothing in a run).
    /// None: the standard rifle, which reads the rules.
    pub fn armed_of(&self, i: usize) -> Option<Gun> {
        let l = |k| f32::from(self.level_of(k));
        self.gun_of(i).map(|g| Gun { range: g.range + 3.0 * l("range"), rate: g.rate / 0.85f32.powf(l("rate")), ..g })
    }

    /// Loadout `i`'s gun as a squad fires it, a loadout with no gun the
    /// run's first rifle with the upgrades, as a squad lands (`armed`).
    /// Never None: a squad armed with None took a merged squad's gun
    /// (`Cmd::Merge` keeps either's), mixing two loadouts (review,
    /// 2026-11-20).
    pub fn squad_gun(&self, i: usize) -> Gun {
        let l = |k| f32::from(self.level_of(k));
        let g = self.gun_of(i).unwrap_or_else(first_gun);
        Gun { range: g.range + 3.0 * l("range"), rate: g.rate / 0.85f32.powf(l("rate")), ..g }
    }

    /// Loadout `i`'s cards as a turret fires them: as many as fit the
    /// turrets' rack, which "Turret power" grows, whatever gun the loadout
    /// holds (a turret has no use for it; with its rack a bought gun
    /// gave turrets four cards with no Turret power, review 2026-11-20).
    pub fn turret_deck_of(&self, i: usize) -> Vec<Card> {
        let rack = self.turret_rack();
        let mut cards = self.cards_of(i);
        while !cards.is_empty() && rounds::fit_gun(cards.clone(), &rack).is_err() {
            cards.pop();
        }
        cards
    }

    /// The first loadout's gun as the squad lands with it (`armed_of`).
    pub fn armed(&self) -> Gun {
        let l = |k| f32::from(self.level_of(k));
        let g = self.gun_of(0).unwrap_or_else(first_gun);
        Gun { range: g.range + 3.0 * l("range"), rate: g.rate / 0.85f32.powf(l("rate")), ..g }
    }

    /// The first loadout's card budget (`rounds::fit_gun`): the army's
    /// round, which squads with no deck of their own fire.
    pub fn budget(&self) -> u32 {
        self.rack_of(0).budget
    }

    /// The turrets' deck as a gun to fit cards in (the turrets thread,
    /// 2026-11-16, after the independent review: a free turret deck was a
    /// pure buff to staying home): 2 slots and a budget of 4, one more slot
    /// and 2 more budget a level of "Turret power". Every loadout with no
    /// gun fits its cards to it.
    pub fn turret_rack(&self) -> Gun {
        let l = u32::from(self.level_of("turret+"));
        Gun { slots: (2 + l).min(rounds::MAX_SLOTS as u32), budget: 4 + 2 * l, ..first_gun() }
    }

    /// What loadout `i`'s cards fit against: its gun, else the turrets'
    /// rack.
    pub fn rack_of(&self, i: usize) -> Gun {
        self.gun_of(i).unwrap_or_else(|| self.turret_rack())
    }

    /// What the shop's gun slots fit cards against.
    pub fn rack(&self) -> Gun {
        self.rack_of(self.editing)
    }

    /// The gun slots show the turrets' loadout (and it is not the
    /// squads' first).
    pub fn editing_turrets(&self) -> bool {
        self.editing == self.turret_loadout && self.editing != 0
    }

    /// Loadout `i`'s cards, in firing order.
    pub fn cards_of(&self, i: usize) -> Vec<Card> {
        if i == self.editing {
            self.gun()
        } else {
            self.loadouts.get(i).map(|l| l.cards.clone()).unwrap_or_default()
        }
    }

    /// The first loadout's cards, in firing order: the army's round.
    pub fn squad_cards(&self) -> Vec<Card> {
        self.cards_of(0)
    }

    /// The turrets' deck (`Game::turret_deck`), in firing order: empty
    /// without turrets.
    pub fn turret_cards(&self) -> Vec<Card> {
        if !self.has("turret") {
            return Vec::new();
        }
        self.turret_deck_of(self.turret_loadout)
    }

    /// Every card owned: in the bag, shown, and in the other loadouts.
    pub fn all_cards(&self) -> Vec<Card> {
        let mut v: Vec<Card> = self.cards.iter().map(|c| c.0).collect();
        for (i, l) in self.loadouts.iter().enumerate() {
            if i != self.editing {
                v.extend(l.cards.iter().copied());
            }
        }
        v
    }

    /// Loadout `i`'s name on the shop's buttons and the HUD.
    pub fn loadout_name(&self, i: usize) -> String {
        format!("Loadout {}", i + 1)
    }

    /// The shop's gun slots show loadout `i`: the shown cards go back to
    /// their loadout, and `i`'s come out to the slots. True if it changed.
    pub fn edit(&mut self, i: usize) -> bool {
        if i == self.editing || i >= self.loadouts.len() {
            return false;
        }
        let (out, bag): (Vec<_>, Vec<_>) = self.cards.drain(..).partition(|c| c.1);
        self.loadouts[self.editing].cards = out.into_iter().map(|c| c.0).collect();
        let back = std::mem::take(&mut self.loadouts[i].cards);
        self.cards = back.into_iter().map(|c| (c, true)).chain(bag).collect();
        self.editing = i;
        true
    }

    /// The shop's gun slots show the turrets' loadout (`on`) or the first.
    /// True if it changed.
    pub fn edit_turrets(&mut self, on: bool) -> bool {
        if on && !self.has("turret") {
            return false;
        }
        self.edit(if on { self.turret_loadout } else { 0 })
    }

    /// Gun `g` into the loadout shown, out of any other: that one goes on
    /// with no gun, its cards refitted to the turrets' rack.
    fn take_gun(&mut self, g: usize) {
        for j in 0..self.loadouts.len() {
            if j != self.editing && self.loadouts[j].gun == Some(g) {
                self.loadouts[j].gun = None;
                self.refit_stored(j);
            }
        }
        self.loadouts[self.editing].gun = Some(g);
        self.refit();
    }

    /// A loadout not shown fits its gun: its last cards back to the bag
    /// (the same order as `refit`).
    fn refit_stored(&mut self, j: usize) {
        let rack = self.rack_of(j);
        while rounds::fit_gun(self.loadouts[j].cards.clone(), &rack).is_err() {
            let cards = &self.loadouts[j].cards;
            let last = |bullet: bool| cards.iter().rposition(|c| (c.kind() == Kind::Payload) == bullet);
            match last(false).or_else(|| last(true)) {
                Some(i) => {
                    let c = self.loadouts[j].cards.remove(i);
                    self.cards.push((c, false));
                }
                None => break,
            }
        }
    }

    /// The gun as it sits fits its budget and slots.
    fn fits_now(&self) -> bool {
        rounds::fit_gun(self.gun(), &self.rack()).is_ok()
    }

    /// Card `c` would fit in the gun beside what it holds, at its end. The
    /// cost depends on the order (a `Thrift` cuts the card after it), so a
    /// card put anywhere else is checked again where it lands (`put_in`,
    /// `move_to`).
    pub fn fits(&self, c: Card) -> bool {
        let mut g = self.gun();
        g.push(c);
        rounds::fit_gun(g, &self.rack()).is_ok()
    }

    /// The cards in the gun fit it: the last card in the gun that is no
    /// bullet comes out first, then the last bullet, so a smaller gun keeps
    /// something to fire. (Before the gun had an order of its own, 11-14,
    /// the last bought came out.)
    fn refit(&mut self) {
        while rounds::fit_gun(self.gun(), &self.rack()).is_err() {
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
    /// but for the last slot, which may sell a second copy of one.
    pub fn restock(&mut self) {
        if self.range {
            let z = mix(mix(self.seed ^ 0x7A46_E000) ^ u64::from(self.depth)) ^ self.rerolls;
            self.offers.clear();
            // The Mind bullets (2026-11-16) and the readers in the standard
            // set only (`Shell::special`): one of each in every set put the
            // armory's bullets on four pages at 200% on 1080 lines, and
            // what they do is the status or the read, not the set's shape.
            for (i, &shell) in Shell::ALL.iter().enumerate() {
                let sets: &[Set] = if shell.special() { &[Set::Standard] } else { &Set::ALL };
                for (j, &set) in sets.iter().enumerate() {
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
        let owned = self.all_cards();
        // A carrier (Trigger, Expire, Multicast) acts on a bullet card after
        // it, so none is sold while the run owns no bullet card (balance
        // review, 2026-11-20; restock already held back Shatter and
        // Detonate this way). Modifiers work on the plain bullet
        // (`rounds::casts`), so they stay in.
        let has_bullet = owned.iter().any(|c| c.kind() == Kind::Payload);
        let pool: Vec<Card> = pool.into_iter().filter(|c| has_bullet || c.kind() != Kind::Trigger || matches!(c, Card::Delay | Card::Thick)).collect();
        let fresh: Vec<Card> = pool.iter().copied().filter(|c| !owned.contains(c)).collect();
        // The cards owned come back into the draw for the last card slot,
        // so a second loadout can buy its own copy (balance review,
        // 2026-11-20); the other slots are cards not yet owned, as before.
        let mut again: Vec<Card> = pool.iter().copied().filter(|c| owned.contains(c)).collect();
        let mut from = if fresh.is_empty() { std::mem::take(&mut again) } else { fresh };
        let mut z = mix(mix(mix(self.seed ^ 0xC0FF_EE00) ^ u64::from(self.depth)) ^ self.rerolls);
        self.offers.clear();
        // The card the planet ahead asks for (`bring`), when not owned,
        // takes the first card slot on every reroll, whatever its tier's
        // depth; the draw fills the rest. Drawn by chance, as before, Tinder
        // was in 23 of 200 planet-2 shops (run seeds 1-20, rerolls 0-9) and
        // not in seed 1's first.
        let wanted = bring(self.level().tier).filter(|c| !owned.contains(c));
        if let Some(c) = wanted {
            from.retain(|&f| f != c);
        }
        // Rolled bullets first, each its own seed. What a shop's bullets
        // may not be (2026-11-18, from the first two shops of run seeds
        // 1-12, where each of these happened on 3 to 5 of the 24): the same
        // shell twice; a reader with nothing to read (a Shatter with no gas
        // bullet owned, a Detonate with no germ: alone each kills nothing);
        // a plain bullet of the standard set, which is the bullet the run
        // starts with. And the first is the planet's (`bring_shell`) while
        // none of that shell is owned.
        let owns = |s: Shell| owned.iter().any(|c| matches!(c, Card::Ammo(a) if a.shell == s));
        let mut shells: Vec<Shell> = shells_at(self.depth).into_iter().filter(|&s| match s {
            Shell::Shatter => owns(Shell::Gas),
            Shell::Detonate => owns(Shell::Germ),
            _ => true,
        }).collect();
        let mut asked = bring_shell(self.level().tier).filter(|&s| !owns(s));
        for _ in 0..AMMO_OFFER {
            z = mix(z);
            let shell = match asked.take() {
                Some(s) => {
                    shells.retain(|&x| x != s);
                    s
                }
                None if shells.is_empty() => break,
                None => shells.remove((z % shells.len() as u64) as usize),
            };
            let mut set = Set::ALL[((z >> 20) % Set::ALL.len() as u64) as usize];
            if (shell, set) == (Shell::Plain, Set::Standard) {
                set = Set::ALL[1 + ((z >> 24) % (Set::ALL.len() as u64 - 1)) as usize];
            }
            self.offers.push(Card::Ammo(Ammo::roll_of(shell, set, mix(z ^ 0xA330), self.depth)));
        }
        let bullets = self.offers.len();
        self.offers.extend(wanted);
        while self.offers.len() < n + bullets && !from.is_empty() {
            // The last slot draws from the new cards and the owned alike.
            if self.offers.len() + 1 == n + bullets && !again.is_empty() {
                from.append(&mut again);
            }
            z = mix(z);
            self.offers.push(from.remove((z % from.len() as u64) as usize));
        }
        if self.offers.len() < n + bullets && !again.is_empty() {
            z = mix(z);
            self.offers.push(again.remove((z % again.len() as u64) as usize));
        }
        // Guns for the planet ahead, each its own seed: two kinds, and
        // neither the kind in hand (2026-11-18: a gun of another kind is
        // the buy that most changes how the squad plays, and of seeds
        // 1-12's first two shops 7 sold the same kind twice and 7 sold the
        // rifle the run starts with).
        let held = self.gun_of(0);
        let mut kinds: Vec<GunKind> = GunKind::ALL.into_iter().filter(|&k| held.is_none_or(|g| g.kind != k)).collect();
        self.gun_offers = (0..GUN_OFFER as u64)
            .filter_map(|k| {
                let s = mix(z ^ (0x6755_4E00 + k));
                (!kinds.is_empty()).then(|| Gun::roll_kind(kinds.remove((s % kinds.len() as u64) as usize), s, self.depth))
            })
            .filter(|g| !self.guns.contains(g))
            .collect();
    }

    /// Shop points the squad took on the planet (`Game::starfall`: ore
    /// from the sky and boss cores), kept whether it is won or lost.
    /// Before `won`, so the end screen's sum is the shop's.
    pub fn found(&mut self, points: u32) {
        self.points += points;
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
        // The loadout panel on a planet (`field`) sells nothing.
        for (i, it) in ITEMS.iter().enumerate().filter(|_| !self.range && !self.field) {
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
        let shown = self.loadout_name(self.editing);
        for (i, g) in self.guns.iter().enumerate() {
            let on = self.carry() == Some(i);
            let other = self.loadouts.iter().position(|l| l.gun == Some(i)).filter(|_| !on);
            let (about, state) = match other {
                _ if on => (format!("In {shown}."), "carried".to_string()),
                Some(j) => (format!("In {}: click to move it to {shown}, which leaves {} with no gun.", self.loadout_name(j), self.loadout_name(j)), self.loadout_name(j)),
                None => (format!("Click to put it in {shown}."), "armory".to_string()),
            };
            rows.push(Row { group: Group::Guns, face: Face::Gun(*g), on, name: title(&g.title()), about, price: String::new(), state, ready: !on, pick: Pick::Carry(i) });
        }
        for (i, g) in self.gun_offers.iter().enumerate().filter(|_| !self.field) {
            let p = gun_price(g);
            let (price, state) = if self.range { (String::new(), "free".into()) } else { (p.to_string(), "for sale".into()) };
            rows.push(Row { group: Group::Guns, face: Face::Gun(*g), on: false, name: title(&g.title()), about: format!("From planet {}.", g.depth), price, state, ready: self.range || p <= self.points, pick: Pick::Gun(i) });
        }
        for (i, &c) in self.offers.iter().enumerate().filter(|_| !self.field) {
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
        // The loadouts (owner, 2026-11-19 19:22Z), on the bottom bar: one
        // button each to show it in the gun slots, a new one, and which
        // the turrets (and, on a planet, the squad) use.
        if !self.range {
            let bar = |name: String, about: String, on: bool, ready: bool, pick| Row { group: Group::Dev, face: Face::Dev, on, name, about, price: String::new(), state: String::new(), ready, pick };
            for i in 0..self.loadouts.len() {
                let gun = self.gun_of(i).map_or("no gun".to_string(), |g| title(&g.title()));
                let mut users = Vec::new();
                if i == 0 {
                    users.push("squads land with it".to_string());
                }
                if i == self.turret_loadout && self.has("turret") {
                    users.push("new turrets use it".to_string());
                }
                if let Some((q, _)) = self.field_squad.filter(|f| f.1 == i) {
                    users.push(format!("Squad {} uses it", q + 1));
                }
                let users = if users.is_empty() { String::new() } else { format!(" ({})", users.join("; ")) };
                let n = self.cards_of(i).len();
                let about = format!("{gun}, {n} card{}{users}. Click to show it in the gun slots.", if n == 1 { "" } else { "s" });
                rows.push(bar(format!("{}: {gun}", i + 1), about, i == self.editing, i != self.editing, Pick::Edit(i)));
            }
            if self.loadouts.len() < LOADOUTS_MAX {
                rows.push(bar("New loadout".into(), "An empty loadout with no gun: put a gun and cards in it, then give it to a squad or turrets.".into(), false, true, Pick::NewLoadout));
            }
            if self.has("turret") && self.turret_loadout != self.editing {
                rows.push(bar("Turrets use this".into(), format!("New turrets use {shown}; each turret can be changed in the game."), false, true, Pick::TurretsUse));
            }
            if let Some((q, l)) = self.field_squad.filter(|f| f.1 != self.editing) {
                rows.push(bar(format!("Squad {} uses this", q + 1), format!("Squad {} fires {shown} (now {}).", q + 1, self.loadout_name(l)), false, true, Pick::SquadUses));
            }
        }
        if self.range {
            let row = |name: &str, about: String, pick| Row { group: Group::Dev, face: Face::Dev, on: false, name: name.into(), about, price: String::new(), state: String::new(), ready: true, pick };
            rows.push(row("New rolls", "Roll the shelf's bullets and the rack's guns again.".into(), Pick::DevReroll));
            rows.push(row("Deeper rolls", format!("Roll them as on planet {} (now {}).", (self.depth + 1).min(PLANETS), self.depth), Pick::DevDepth(1)));
            rows.push(row("Shallower rolls", format!("Roll them as on planet {} (now {}).", self.depth.saturating_sub(1).max(1), self.depth), Pick::DevDepth(-1)));
            rows.push(row("Empty the gun", "Every card in the gun back to the bag.".into(), Pick::EmptyGun));
            rows.push(row("Empty the bag", "Throw away the cards in the bag.".into(), Pick::EmptyBag));
        }
        if self.dev && !self.range && !self.field {
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
            Pick::Item(_) | Pick::Offer(_) | Pick::Gun(_) if self.field => false,
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
                // A new gun goes in the loadout shown (to the squads' first
                // from the turrets', as before loadouts).
                if self.editing_turrets() {
                    self.edit(0);
                }
                // The range swaps a rack gun for the owned one of its kind,
                // which goes on the rack (four of each, either way back).
                if let Some(k) = self.range.then(|| self.guns.iter().position(|o| o.kind == g.kind)).flatten() {
                    self.gun_offers[i] = std::mem::replace(&mut self.guns[k], g);
                    self.take_gun(k);
                    return true;
                }
                let price = gun_price(&g);
                if price > self.points {
                    return false;
                }
                self.points -= price;
                self.gun_offers.remove(i);
                self.guns.push(g);
                self.take_gun(self.guns.len() - 1);
                true
            }
            Pick::Carry(i) => {
                if i >= self.guns.len() || self.carry() == Some(i) {
                    return false;
                }
                self.take_gun(i);
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
            Pick::EditTurrets => self.edit_turrets(!self.editing_turrets()),
            Pick::Edit(i) => self.edit(i),
            Pick::NewLoadout => {
                if self.loadouts.len() >= LOADOUTS_MAX {
                    return false;
                }
                self.loadouts.push(Loadout::default());
                self.edit(self.loadouts.len() - 1)
            }
            Pick::TurretsUse => {
                let done = self.has("turret") && self.turret_loadout != self.editing;
                if done {
                    self.turret_loadout = self.editing;
                }
                done
            }
            Pick::SquadUses => match self.field_squad {
                Some((q, l)) if l != self.editing => {
                    self.field_squad = Some((q, self.editing));
                    true
                }
                _ => false,
            },
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

    /// Each late threat waits for its planet (`FIRST_PLANET`, the owner
    /// 11-20 23:32Z): planet 1 has fruiting and the heartbeat but no
    /// strikers, waves, late lunge or last-stand heart; each joins on its
    /// planet and stays on deeper; every scheduled rule is one of
    /// `ENEMY_RULES` and every rule plays by planet 4.
    #[test]
    fn late_threats_wait_for_their_planet() {
        for &(k, d) in FIRST_PLANET {
            assert!(ENEMY_RULES.iter().any(|&(n, _)| n == k), "FIRST_PLANET names {k}, not one of ENEMY_RULES");
            assert!((1..=4).contains(&d), "{k} first on planet {d}");
        }
        let rules = |d: u32| {
            let mut r = Rules::default();
            Run { depth: d, ..Run::new(1) }.rules(&mut r);
            r
        };
        // Wherever the strikers can be on, their share and cap are not 0.
        assert!(STRIKER_SHARE.iter().all(|&k| k > 0.0), "a 0 share leaves strikers on and doing nothing");
        let p1 = rules(1);
        assert!(p1.heart.on && p1.unit_paths, "planet 1 keeps the heartbeat and charges round walls");
        assert!(!p1.fruit.on && !p1.strikers.on && !p1.wave.on && p1.late_at == 0.0 && !p1.stand.on, "planet 1 has none of the late threats");
        assert_eq!(p1.boss.every, 0.0, "nor mini-bosses");
        assert!(p1.star.on, "planet 1 keeps starfall and cores");
        let p2 = rules(2);
        assert!(p2.fruit.on && p2.strikers.on && p2.strikers.share > 0.0 && !p2.wave.on && p2.late_at == 0.0 && !p2.stand.on);
        let p3 = rules(3);
        assert!(p3.strikers.on && p3.wave.on && p3.late_at > 0.0 && !p3.stand.on);
        for d in 4..=PLANETS {
            let r = rules(d);
            assert!(r.strikers.on && r.wave.on && r.late_at > 0.0 && r.stand.on && r.fruit.on && r.heart.on, "planet {d}");
        }
    }

    /// Deeper, the slime surges no less often, lunges no shorter and
    /// sends specks no more often; planet 1 is gentler than the sandbox's shape C
    /// and the core world fiercer, and every key is one the engine takes.
    #[test]
    fn the_slime_gets_fiercer_deeper() {
        let get = |d: u32, k: &str| enemy_keys(d).into_iter().find(|(n, _)| *n == k).map(|(_, v)| v.parse::<f32>().unwrap()).unwrap();
        for d in 1..=10 {
            assert!(get(d + 1, "surge") <= get(d, "surge") && get(d + 1, "surge_dash") >= get(d, "surge_dash"), "planet {d}");
            // Specks no more often deeper (owner 11-16: "Specks more often
            // early game"): chances no closer together, no larger share.
            assert!(get(d + 1, "roam") >= get(d, "roam") && get(d + 1, "roam_share") <= get(d, "roam_share"), "planet {d}");
            assert_eq!(surge_secs(d), get(d, "surge") / 30.0);
            let mut p = crate::mould::big::BigParams::today();
            for (k, v) in enemy_keys(d) {
                // The lure's pull is the GPU engine's alone (the owner's
                // card, 11-17 19:21Z: no CPU twin): the engine takes it,
                // and `BigParams` has no field for it.
                if k == "lure_pull" || k == "tether" {
                    let mut s = crate::gpu::Settings::big_dish();
                    crate::gpu::set(&mut s, k, &v).unwrap();
                    assert!(s.lure_pull == LURE_PULL || s.tether == TETHER, "planet {d}: {k}");
                    continue;
                }
                crate::mould::tiers::set_key(&mut p, k, &v).unwrap();
            }
            // No bronze (owner 11-16: "Remove the bronze"), specks on.
            assert!(p.surge_every > 0 && p.surge_dash > 0 && p.roam > 0 && p.roam_share > 0.0);
            assert!(surge_food(d + 1, 1.0) <= surge_food(d, 1.0) && surge_food(d, 2.0) == 2.0 * surge_food(d, 1.0), "planet {d}");
        }
        let c_food = crate::mould::larder_world::SURGE_FOOD;
        assert!(surge_food(1, 1.0) > c_food && surge_food(PLANETS, 1.0) < c_food);
        let (_, c) = crate::mould::larder_world::shape_keys("c").unwrap();
        let c_surge: f32 = c.iter().find(|(k, _)| *k == "surge").unwrap().1.parse().unwrap();
        assert!(get(1, "surge") > c_surge && get(PLANETS, "surge") < c_surge);
        assert!(get(PLANETS, "surge") < get(1, "surge") && get(PLANETS, "surge_dash") > get(1, "surge_dash"));
        // Specks clearly more often on planet 1 than on the core world.
        assert!(get(1, "roam_share") / get(1, "roam") > 2.0 * get(PLANETS, "roam_share") / get(PLANETS, "roam"));
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
            // can have fewer in all; the tier's own factor (`tier_size`, the
            // Sprawler's 0.6) is a tuning number kept apart from that rule.
            let dense = l.walkers / area_of(&r.planet()) / size_roll(l.seed) / tier_size(l.tier);
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
        // Planet 2's Sprawler at 0.6 of its size (the planet 2 look, 11-18).
        r.depth = 2;
        let (l, area) = (r.level(), area_of(&r.planet()));
        assert_eq!(l.tier, Tier::Sprawler);
        assert!((l.walkers - 0.6 * 1.2 * area * size_roll(l.seed)).abs() < 1e-5);
        // The level generator's core world is the run's last planet.
        assert_eq!(levelgen::CORE_DEPTH, PLANETS);
    }

    /// The planet review's finding 6 and 5 (11-15): the clock grows with
    /// the planet's area from 15 minutes, and the slime grows first on
    /// planets bigger than today's map. Since 2026-11-20 the clock counts
    /// up: the level carries the speed star's time (`par`) and twice the
    /// old clock as a stalemate's limit (`limit`).
    #[test]
    fn bigger_planets_get_a_longer_clock_and_a_grown_slime() {
        let at: Vec<(f32, f32)> = levelgen::AREA.iter().map(|&a| (clock(a), head_start(a))).collect();
        assert_eq!(at, [(900.0, 0.0), (990.0, 0.0), (1080.0, 0.0), (1260.0, 11.0), (1530.0, 25.0), (1890.0, 40.0), (2430.0, 60.0), (3330.0, 87.0)]);
        let pars: Vec<f32> = levelgen::AREA.iter().zip(1..).map(|(&a, d)| par(d, a)).collect();
        assert_eq!(pars, [480.0, 600.0, 600.0, 690.0, 840.0, 1050.0, 1350.0, 1860.0], "8:00, then 10:00, then scaled by the old clock");
        let mut r = Run::new(3);
        for d in 1..=PLANETS {
            r.depth = d;
            let (l, a) = (r.level(), area_of(&r.planet()));
            let rules = l.mission.rules(Rules::default());
            assert_eq!((rules.cap, rules.par, l.grow), (limit(a), par(d, a), head_start(a)), "planet {d}: the level's rules carry its limit and par");
            assert!(rules.par < rules.cap / 2.0, "planet {d}: the stalemate limit is far past the star");
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
        let text = format!("name {}\nsays {}\nmap {}\ntier {}\ngoal shrink {}\nbonus lose 2\nrule drain {}\nrule cap {}\nrule par {}\n", l.mission.name, l.mission.says, l.mission.map, l.tier.name(), shrink_goal(1), Rules::default().drain, limit(0.6), par(1, 0.6));
        assert_eq!(Mission::parse(&text).unwrap(), l.mission);
    }

    /// Balance review, 2026-11-20: no carrier is sold while the run owns
    /// no bullet card, and an owned card can come back in the last slot.
    #[test]
    fn the_shop_holds_back_carriers_and_sells_owned_cards_again() {
        let carrier = |c: &Card| matches!(c, Card::Trigger | Card::Expire | Card::Multicast);
        let (mut again, mut carriers) = (0, 0);
        for seed in 1..=20 {
            let mut r = Run::new(seed);
            for d in 1..=4 {
                r.depth = d;
                for k in 0..5 {
                    r.rerolls = k;
                    r.restock();
                    assert!(!r.offers.iter().any(carrier), "seed {seed} depth {d}: a carrier with no bullet owned: {:?}", r.offers);
                }
            }
            r.cards = vec![(Card::Blast, true), (Card::Wide, false), (Card::Pierce, false)];
            for d in 1..=4 {
                r.depth = d;
                for k in 0..5 {
                    r.rerolls = k;
                    r.restock();
                    carriers += r.offers.iter().filter(|c| carrier(c)).count();
                    again += r.offers.iter().filter(|c| matches!(c, Card::Wide | Card::Pierce)).count();
                }
            }
        }
        assert!(carriers > 0, "carriers sold once a bullet is owned");
        assert!(again > 0, "owned cards come back into the draw");
    }

    #[test]
    fn the_shop_before_the_sprawler_offers_tinder() {
        // Run seed 1's planet 2 is the Sprawler, which shies from fire that
        // brings its own fuel: its shop offers Tinder on every reroll.
        let mut r = Run::new(1);
        r.won(1);
        assert_eq!(r.level().tier, Tier::Sprawler);
        assert_eq!(bring(Tier::Sprawler), Some(Card::Tinder));
        for _ in 0..8 {
            assert!(r.offers.contains(&Card::Tinder), "{:?}", r.offers);
            assert_eq!(r.offers.len(), OFFER + AMMO_OFFER + r.level_of("draw") as usize, "it takes a slot, not an extra one");
            r.rerolls += 1;
            r.restock();
        }
        // Owned, it is no longer pushed.
        r.cards.push((Card::Tinder, false));
        r.restock();
        assert!(!r.offers.contains(&Card::Tinder));
        // No other rung asks for a card yet.
        assert!(LADDER.iter().all(|&t| t == Tier::Sprawler || bring(t).is_none()));
    }

    /// **The first two shops change how the gun plays** (Milestone 1's
    /// exit test 5; 2026-11-18). On run seeds 1-12, every reroll: the shop
    /// before the Sprawler and the one before the Trunk sell a Torch
    /// bullet first while none is owned (`bring_shell`: Tinder's brush
    /// needs a fire, and the Trunk's amber walls burn); the two bullets
    /// are never one shell, never the starting bullet, never a reader with
    /// nothing to read; the two guns are two kinds, neither the one
    /// carried.
    #[test]
    fn the_first_shops_sell_the_planets_bullet_and_no_doubles() {
        for seed in 1..=12 {
            let mut r = Run::new(seed);
            for tier in [Tier::Sprawler, Tier::Trunk] {
                r.won(2);
                assert_eq!((r.level().tier, bring_shell(tier)), (tier, Some(Shell::Torch)));
                for _ in 0..6 {
                    let shells: Vec<(Shell, Set)> = r.offers.iter().filter_map(|c| if let Card::Ammo(a) = c { Some((a.shell, a.set)) } else { None }).collect();
                    assert_eq!((shells.len(), shells[0].0), (AMMO_OFFER, Shell::Torch), "seed {seed}, {tier:?}: {:?}", r.offers);
                    assert_ne!(shells[0].0, shells[1].0, "seed {seed}: one shell twice");
                    assert!(shells.iter().all(|&s| s != (Shell::Plain, Set::Standard) && !matches!(s.0, Shell::Shatter | Shell::Detonate)), "seed {seed}: {shells:?}");
                    assert_eq!(r.offers.len(), OFFER + AMMO_OFFER, "the planet's bullet takes a slot, not an extra one");
                    // Tinder and the Torch together cost no more than the
                    // least a won planet 1 pays (Direction, 2026-11-18).
                    assert!(card_price(r.offers[0]) + card_price(Card::Tinder) <= reward(1, 1), "seed {seed}: a torch at {}", card_price(r.offers[0]));
                    let kinds: Vec<GunKind> = r.gun_offers.iter().map(|g| g.kind).collect();
                    assert!(kinds.len() == GUN_OFFER && kinds[0] != kinds[1] && !kinds.contains(&r.guns[r.carry().unwrap()].kind), "seed {seed}: {kinds:?} with a {:?} carried", r.guns[r.carry().unwrap()].kind);
                    r.rerolls += 1;
                    r.restock();
                }
            }
        }
        // With a torch owned the planet's bullet is no longer pushed, and
        // with a gas bullet owned a Shatter may be sold.
        let mut r = Run::new(1);
        r.won(2);
        r.cards.push((Card::Ammo(Ammo::standard(Shell::Torch)), false));
        r.cards.push((Card::Ammo(Ammo::standard(Shell::Gas)), false));
        let (mut torch_first, mut shatter) = (0, 0);
        for _ in 0..40 {
            r.rerolls += 1;
            r.restock();
            let shells: Vec<Shell> = r.offers.iter().filter_map(|c| if let Card::Ammo(a) = c { Some(a.shell) } else { None }).collect();
            torch_first += usize::from(shells[0] == Shell::Torch);
            shatter += usize::from(shells.contains(&Shell::Shatter));
        }
        assert!(torch_first < 12 && shatter > 0, "torch first in {torch_first} of 40, shatter in {shatter}");
        // The other rungs ask for no bullet yet.
        assert!(LADDER.iter().all(|&t| matches!(t, Tier::Sprawler | Tier::Trunk) || bring_shell(t).is_none()));
    }

    #[test]
    fn the_shop_sells_what_the_points_buy_and_no_more() {
        let mut r = Run::new(1);
        assert!(r.has("walls") && !r.has("turret"));
        assert!(r.shop().iter().all(|row| !row.ready || matches!(row.pick, Pick::Card(_) | Pick::Edit(_) | Pick::NewLoadout)), "no points, nothing to buy");
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
        assert_eq!(rounds::casts(&r.gun()).len(), 3, "gas, then quiet blast, then a wide plain bullet");
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
    /// payload and set (the Mind bullets and readers standard only) and every other card on a free shelf that never
    /// sells out, no upgrades; its rows roll again, roll deeper and empty
    /// the gun and the bag. A run has none of that.
    #[test]
    fn the_gun_range_is_a_free_armory() {
        let mut r = Run::range(7);
        assert!(r.guns.iter().map(|g| g.kind).eq(GunKind::ALL) && r.gun_offers.len() == GunKind::ALL.len());
        let others = Card::ALL.iter().filter(|c| c.kind() != Kind::Payload).count();
        // The Mind bullets and the readers in the standard set only
        // (2026-11-16, `restock`).
        let special = Shell::ALL.iter().filter(|s| s.special()).count();
        assert_eq!(r.offers.len(), (Shell::ALL.len() - special) * Set::ALL.len() + special + others);
        for shell in Shell::ALL {
            for set in Set::ALL {
                let on = r.offers.iter().any(|c| matches!(c, Card::Ammo(a) if a.shell == shell && a.set == set));
                assert_eq!(on, !shell.special() || set == Set::Standard, "{shell:?} {set:?}");
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
        assert!(r.pick(Pick::Gun(0)) && r.carry() == Some(0) && r.guns[0] == rack && r.gun_offers[0] == owned, "a rack gun swaps for the owned one of its kind, free, and is carried");
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
        assert!(r.pick(Pick::Gun(0)) && r.carry() == Some(1) && r.budget() == 7);
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
                assert!(ammo.iter().all(|a| d >= 4 || !matches!(a.shell, Shell::Bomb | Shell::Germ)), "{ammo:?}");
                assert!(r.offers.iter().all(|c| matches!(c, Card::Ammo(_)) || c.kind() != Kind::Payload), "the fixed payload cards are no longer sold");
                assert_eq!(r.offers.len(), AMMO_OFFER + OFFER);
            }
        }
        // The germ is sold from planet 4, as the bomb is (2026-11-16).
        let germs = (0..60u64)
            .filter(|&seed| {
                let mut r = Run::new(seed);
                r.depth = 5;
                r.restock();
                r.offers.iter().any(|c| matches!(c, Card::Ammo(a) if a.shell == Shell::Germ))
            })
            .count();
        assert!(germs > 0, "no germ bullet in 60 planet-5 shops");
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
            // The fire-shy Sprawler, on both engines; no other rung is.
            // The CPU setter takes the keys it has: the Trunk's are the GPU
            // engine's alone (`TRUNK_KEYS`).
            let mut p = crate::mould::larder_world::race_params(1, 2, 5000);
            for &(k, v) in does.keys {
                let set = crate::mould::tiers::set_key(&mut p, k, v);
                assert!(set.is_ok() || *t == Tier::Trunk, "{} key {k}: {set:?}", t.name());
            }
            let shy = if *t == Tier::Sprawler { 50.0 } else { 0.0 };
            assert_eq!((s.fire_alarm, p.fire_alarm), (shy, shy), "{}", t.name());
            // The Trunk sets, walls soldiers out, walls the crystals it
            // holds and is fireproof while fed; the Swarmer makes a last
            // stand; no other rung does either.
            let trunk = *t == Tier::Trunk;
            assert_eq!((s.set_scar > 0.0, s.crust_rise > 0.0, s.fire_fed, r.set_block > 0.0, r.fort > 0.0), (trunk, trunk, trunk, trunk, trunk), "{}", t.name());
            assert_eq!(r.raid.last_stand, *t == Tier::Swarmer, "{}", t.name());
            // Every planet: the far smell, and charges that go round walls.
            assert!(r.unit_paths, "{}", t.name());
            assert!(enemy_keys(d as u32 + 1).iter().any(|(k, v)| *k == "far_smell" && v == "1"));
            // Every planet: a lure pulls (the owner's card, 11-18 03:03Z).
            assert!(enemy_keys(d as u32 + 1).iter().any(|(k, v)| *k == "lure_pull" && v.parse::<f32>().unwrap() > 0.0));
        }
        let mut r = Rules::default();
        Run::range(1).rules(&mut r);
        assert_eq!(r.build.pump_smell, Rules::default().build.pump_smell);
    }

    /// The outpost and siren are not sold in a run (owner 11-15 21:41Z,
    /// D10). Nor is the lure: since 2026-11-18 it is free from the first
    /// planet and needs no unlock (the window's `run_lock`).
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

    /// **The turrets' deck comes out of the same bag** (the turrets
    /// thread, 2026-11-16, after the independent review): with turrets
    /// owned, the shop's gun slots switch to the turrets' rack (2 slots,
    /// budget 4), a card put there leaves the bag and the squad's gun, both
    /// decks survive the switch either way, and without turrets there is no
    /// switch and no turret deck.
    #[test]
    fn the_turrets_take_cards_from_the_squads_bag() {
        let mut r = Run::new(1);
        r.cards = vec![(Card::Wide, true), (Card::Blast, false), (Card::Gas, false), (Card::Bomb, false)];
        assert!(!r.pick(Pick::EditTurrets), "no turrets, no switch");
        assert!(!r.shop().iter().any(|w| w.pick == Pick::TurretsUse));
        assert!(r.turret_cards().is_empty());
        r.owned[ITEMS.iter().position(|i| i.key == "turret").unwrap()] = 1;
        assert!(r.shop().iter().any(|w| w.pick == Pick::Edit(1)), "the turrets' loadout has its button");
        assert!(r.pick(Pick::EditTurrets) && r.editing_turrets());
        assert_eq!((r.rack().slots, r.rack().budget), (2, 4));
        assert!(r.gun().is_empty(), "the slots now show the empty turret deck");
        assert_eq!(r.squad_cards(), vec![Card::Wide], "the squad keeps its gun");
        // Gas into the turrets: out of the bag.
        let gas = r.cards.iter().position(|c| c.0 == Card::Gas).unwrap();
        assert!(r.pick(Pick::Card(gas)));
        assert_eq!(r.turret_cards(), vec![Card::Gas]);
        assert_eq!(r.cards.iter().filter(|c| !c.1).count(), 2, "the bag gave it up");
        // Back to the gun: each deck where it was, the bag shared.
        assert!(r.pick(Pick::EditTurrets) && !r.editing_turrets());
        assert_eq!((r.squad_cards(), r.turret_cards()), (vec![Card::Wide], vec![Card::Gas]));
        assert_eq!(r.gun(), vec![Card::Wide]);
        assert!(r.cards.iter().all(|c| c.0 != Card::Gas), "gas is on the turrets, not in the bag");
        // Turret power grows the rack: a slot and 2 budget a level.
        r.owned[ITEMS.iter().position(|i| i.key == "turret+").unwrap()] = 2;
        assert_eq!((r.turret_rack().slots, r.turret_rack().budget), (4, 8));
    }

    /// **Loadouts** (owner, 2026-11-19 19:22Z): a gun and its cards; a
    /// gun or a card is in one loadout at a time; a loadout that loses its
    /// gun fits the turrets' rack and gives back what no longer fits; the
    /// field panel sells nothing and gives the panel's squad a loadout.
    #[test]
    fn loadouts_hold_their_own_gun_and_cards() {
        let mut r = Run::new(1);
        r.cards = vec![(Card::Wide, true), (Card::Blast, false), (Card::Gas, false), (Card::Bomb, false)];
        r.guns.push(GunKind::Scatter.average());
        assert_eq!(r.loadouts.len(), 2);
        // A new loadout, shown, with no gun: the turrets' rack.
        assert!(r.pick(Pick::NewLoadout) && r.editing == 2 && r.carry().is_none());
        assert_eq!(r.rack(), r.turret_rack());
        // The scattergun into it, then a card.
        assert!(r.pick(Pick::Carry(1)) && r.gun_of(2) == Some(r.guns[1]));
        let gas = r.cards.iter().position(|c| c.0 == Card::Gas).unwrap();
        assert!(r.pick(Pick::Card(gas)));
        assert_eq!((r.cards_of(0), r.cards_of(2)), (vec![Card::Wide], vec![Card::Gas]));
        // Back to the first: each keeps its own, and the gas is not in the bag.
        assert!(r.pick(Pick::Edit(0)));
        assert_eq!((r.gun(), r.cards_of(2)), (vec![Card::Wide], vec![Card::Gas]));
        assert!(!r.cards.iter().any(|c| c.0 == Card::Gas) && r.all_cards().contains(&Card::Gas));
        // Taking loadout 3's gun into the first leaves it with none, its
        // cards refitted to the rack.
        assert!(r.pick(Pick::Carry(1)) && r.carry() == Some(1) && r.gun_of(2).is_none());
        assert!(rounds::fit_gun(r.cards_of(2), &r.turret_rack()).is_ok());
        assert_eq!(r.gun_of(0), Some(r.guns[1]));
        // A row says which loadout holds each gun.
        assert!(r.shop().iter().any(|w| w.pick == Pick::Carry(0) && w.state == "armory"));
        // The field panel: nothing for sale, and the squad's loadout.
        r.points = 10_000;
        r.restock();
        r.field = true;
        r.field_squad = Some((3, 0));
        let rows = r.shop();
        assert!(!rows.iter().any(|w| matches!(w.pick, Pick::Item(_) | Pick::Offer(_) | Pick::Gun(_))), "nothing for sale on a planet");
        assert!(!r.pick(Pick::Item(0)) && !r.pick(Pick::Offer(0)));
        assert!(!rows.iter().any(|w| w.pick == Pick::SquadUses), "squad 4 already uses the one shown");
        assert!(r.pick(Pick::Edit(2)) && r.shop().iter().any(|w| w.pick == Pick::SquadUses));
        assert!(r.pick(Pick::SquadUses) && r.field_squad == Some((3, 2)));
        // At most LOADOUTS_MAX.
        while r.pick(Pick::NewLoadout) {}
        assert_eq!(r.loadouts.len(), LOADOUTS_MAX);
    }

    /// A turret fires as many of its loadout's cards as the turrets' rack
    /// takes, whatever gun the loadout holds; a squad on a loadout with no
    /// gun lands with the run's rifle (review, 2026-11-20).
    #[test]
    fn turrets_fire_their_rack_and_gunless_squads_the_rifle() {
        let mut r = Run::new(1);
        r.owned[ITEMS.iter().position(|i| i.key == "turret").unwrap()] = 1;
        r.guns.push(Gun { slots: 4, budget: 12, ..GunKind::Long.average() });
        assert!(r.pick(Pick::Carry(1)));
        r.cards = vec![(Card::Wide, true), (Card::Blast, true), (Card::Gas, true), (Card::Bomb, true)];
        assert!(rounds::fit_gun(r.gun(), &r.carried()).is_ok());
        assert!(r.pick(Pick::TurretsUse));
        let t = r.turret_cards();
        assert!(t.len() < 4 && rounds::fit_gun(t.clone(), &r.turret_rack()).is_ok(), "{t:?} on a rack of {}", r.turret_rack().slots);
        assert_eq!(r.squad_gun(1).kind, first_gun().kind, "loadout 2 has no gun: the rifle");
    }
}
