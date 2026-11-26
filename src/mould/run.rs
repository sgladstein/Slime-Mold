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
use super::bodies::{Body, BodyKind};
use super::guns::{Gun, GunKind};
use super::kit::KitRules;
use super::larder_world::{Rules, Twist};
use super::levelgen::{self, Planet};
use super::map::{Map, G_ROCK, MAP_H, MAP_W};
use super::rounds::{self, Ammo, Card, Kind, RoundRules, Set, Shell};
use super::route::{self, Contract, Node, Outcome, Pip, StarMap};
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
/// playable. Since 2026-11-24 planet 8 has its own, the Dozer (Direction:
/// "P8 = the Swarmer that naps"), and past it the Sleeper, Grudge and
/// Dozer take turns.
pub const LADDER: [Tier; 8] = [Tier::Drifter, Tier::Sprawler, Tier::Trunk, Tier::Swarmer, Tier::Sporer, Tier::Sleeper, Tier::Grudge, Tier::Dozer];

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
    /// Its place on the tuned eight-planet curve (`Run::curve`): the
    /// step's (`curve_depth`) moved by the star map node's pip.
    pub curve: f32,
}

/// Planets in a run; the last is the slime's home world (the core world,
/// the level generator's biggest map). Told to the level-gen thread
/// 2026-11-14: the owner's guess is 20 minutes to over an hour a run.
/// Twelve since the star map (2026-11-26, the owner's pick of a route
/// with forks, 11-25 23:45Z, and his "at least 2 hours", 11-24 03:14Z):
/// the eight tuned planets spread over twelve steps (`curve_depth`), each
/// step a fork of planets of clearly different difficulty (`route`).
pub const PLANETS: u32 = 12;

/// The Tools group's unlocks that are devices (`Run::has_device`): the
/// squad mode's device slot sets them off (`Game::device`).
pub const DEVICES: [&str; 4] = ["blast", "mine", "cord", "gas"];

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
///
/// 0.45 since its threads (2026-11-24, Direction: land them, retune planet
/// 2's size if harder): lanes let up to 3 more walkers share a cell, so the
/// threaded body filled its birth ceiling where the packed one was held by
/// room. Run seed 1, idle squad at 2:00: 14,317 walkers off, 26,725 with
/// threads at 0.6, 15,593 at 0.45 (landing 13,845 and 10,384); 4, 3 and 4
/// soldiers left. The spokes read the same at 0.45.
pub fn tier_size(tier: Tier) -> f32 {
    match tier {
        Tier::Sprawler => 0.45,
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
/// **The origin's members by planet**, 1 to 8 (`OriginRules::size`,
/// `origin.rs`, 2026-11-22): the body it gathers at the landing and
/// regrows to, half as much again at most (`OriginRules::most`). The
/// design's first numbers (400, 600, 900) took about 900, 1,300 and 1,900
/// at their first wide takes anyway, over the most they could regrow to,
/// so a struck origin never regrew and 44 soldiers walking straight at it
/// won planet 1 at 1:35 with none lost (run seed 1, lavapipe, 11-22).
/// Second pass the same morning (44 soldiers, `hand=origin`): planet 2 at
/// 1,200 wiped them at 1:50, at 800 wiped them too, at 700 they won at
/// 2:05 with 34 lost (24 were wiped), at 600 won at 1:45 with 10 lost; its
/// strikers and Mortar fight there too. Planet 1 lost none at any try
/// (bite 3, armour 15, a lunge reach of 4 or 6 radii, the flinch off): 44
/// guns cut a lunge down before it touches them. So planet 1 is bigger,
/// 1,200, for time (won at 2:54 against 2:13 at 800), and planet 2 lower.
/// Then Engine's siege (the run bot round the origin, seeds 1 to 3 on
/// planet 1, after Squad's near-first rule of #715, which keeps a soldier
/// on the slime 5 cells off before an origin 20 off) wore it down only 50
/// to 70 a minute and never broke it in 12 min (it held 1,929 to 2,118).
/// Direction's pick (11-22 08:42Z, of a near origin beating that rule or a
/// smaller one): planet 1 at 700 and planet 3 at 1,000, so the siege wins
/// planets 1 to 3 in under 10 min. With each take cut to its shortfall
/// (`origin.rs`), the siege at these: planet 1 won seeds 1-3 at 4:23, 4:42
/// and 7:57; planet 3 at 4:00, 4:37 and 9:14; planet 2 won seed 1 at 5:59
/// but was wiped on seeds 2 and 3 (both won at 4:37 and 5:04 with the
/// origin off, and seed 3 was wiped at 600 too), so planet 2 is 500: won
/// seeds 1-3 at 5:49, 5:16 and 5:42, losing 0, 8 and 7.
/// Planets 4 to 8 at half their first sizes (Direction, 11-24 05:25Z, as
/// first numbers): Engine's siege on Scott's three maps (GTX 1070, plan
/// `bot,want=torch.standard|rush,aim=6,cards=0`, 720 s) won 10 of 15 at
/// half against 6 of 15 at 2,400 to 4,800 (sign test p 0.22). Planet 4
/// won all three at 1,200 (5:13, 9:26, 8:07) and lost all three at 2,400;
/// planet 6 won all three both ways. Planet 7's origin swallowed the whole
/// body on two maps at either size (the last stand's gather, now capped:
/// `OriginRules::gather_most`), so its size is not yet measured.
/// Planets 5, 6 and 8 raised half as much again (Direction 11-24: a late
/// planet's win should cost more than an early one's; the numbers are this
/// lane's): Engine's batch s (GTX 1070, run bot with `storm` 0.5, the owner's
/// three maps and run seeds 1-3, 720 s) as is against 1.5x. Planet 5 at
/// 2,250 won 3 of 6 against 4 (one win became open, another open became a
/// win); planet 6 at 2,700 won 5 of 6 both ways but slower (map A 5:53 to
/// 8:00, B 6:10 to 7:11, seed 3 4:24 to 9:50); planet 8 (the Dozer) won 6 of
/// 6 both ways in 5:07 to 7:05, the origin's size making no visible
/// difference there. Planet 7 stays 2,100: the siege won 3 of 6 there, the
/// hardest planet, and a gather cap of 1.25 sizes instead of 2 made it
/// harder still (0 of 6), so the cap stays 2.
pub const ORIGIN_SIZE: [f32; 8] = [700.0, 500.0, 1000.0, 1200.0, 2250.0, 2700.0, 2100.0, 3600.0];
/// **The heart's armour by planet** (`OriginRules::armour`, the blows a
/// member takes, 15 at most; D7, the owner's card of 11-24 14:52Z,
/// "Tougher heart by planet": a snipe works early and costs more later,
/// and the siege still wins under 12 minutes). The measure: the smallest
/// squad that breaks the heart (Engine's `hand=origin2`, which closes in
/// as the siege does, banks of 1,600, 4,800 and 11,200 for squads of
/// about 8, 16 and 32, run seeds 1-3, GTX 1070). As it was (armour 12
/// everywhere; batches u2 and y2, after #824 and #830 kept the Dozer's
/// and the Sporer's hearts from dozing or bursting out of their count):
/// planet 2 broke to 16 (0, 2, 3 of 3), 3 to 32 (0, 0, 2), 4 to 16 (0, 2,
/// 3), 6 to 32 (0, 0, 2; at 11,200 all three hearts fell under 300), 7 to
/// 32 (0, 0, 2), 8 to 32 once in three (0, 0, 1). So planets 6 to 8
/// already ask twice planet 2's squad, and planet 4 was the dip: at 15
/// it breaks only to 32 (0, 0, 3; batch y1), and Engine's siege (storm
/// 0.5 and from 8:00, six maps, 720 s, batch z4) won 5 of 6 against 4 of
/// 6 as it was, the heart falling 50 to 77 s later on three of the four
/// maps both won, with 9 to 24 more soldiers lost there. Planet 5 stays
/// at 12: once its heart stopped bursting into spores (#830) no squad of
/// 32 broke it (batch y3: 0 of 9, the deepest at 1,457 of 2,086), and the
/// siege won 2 of 6 as it is against 1 of 6 at 15.
pub const ORIGIN_ARMOUR: [u32; 8] = [12, 12, 12, 15, 12, 12, 12, 12];
const STRIKER_SHARE: [f32; 8] = [0.25, 0.6, 1.0, 1.0, 1.0, 1.0, 1.0, 1.0];
/// The slime's room to regrow over its landing, by planet
/// (`Rules::ceiling_grow`; 2026-11-22, the thread "Set difficulty between
/// last two playtests"). Every planet at the 1.5 the owner's "in between"
/// asked for (his 11-21 game regrew to 2, his 11-22 one to 1). A row of
/// its own so planet 1 can move alone: Squad's #708 (free fire shoots
/// what is near first) made planet 1 easier in the gun lab (36 soldiers
/// lived against 22), and Direction's plan is to raise planet 1's alone
/// (to about 2) if his next planet 1 log still reads easy.
const CEILING_GROW: [f32; 8] = [1.5; 8];
/// **Harder from planet 2** (2026-11-22, the thread "Make the run harder
/// after full-game win"). The owner won all 8 planets on 11-22 losing 0 to
/// 1 soldiers on 7 of them ("Still too easy"), then: "Don't redo
/// everything and make it too hard again but scale it up. You decide what
/// is best". Planet 1 keeps every number (Direction: he lost half his
/// planet 1 games on 11-20 and 11-21).
///
/// - `RAID_GAP_MIN`: raids never closer than this (`RaidRules::gap_min`):
///   45 s since #652 (his 11-21 planet 1 had raids 26 s apart); 35 from
///   planet 2, partway back.
/// - `STRIKER_ARMY`: the strikers' rate cap grows with an army past 24
///   soldiers, to this many times (`StrikerRules::army_most`): 2.5 times
///   at 60 or more alive.
/// - `SOLDIER_STEP`: the soldier's price climbs this share of its base
///   for each soldier alive over 30 (`Rules::soldier_step`): 2x at 70,
///   2.5x at 90. Off on planet 1, where armies stay small.
const RAID_GAP_MIN: [f32; 8] = [45.0, 35.0, 35.0, 35.0, 35.0, 35.0, 35.0, 35.0];
const STRIKER_ARMY: [f32; 8] = [1.0, 2.5, 2.5, 2.5, 2.5, 2.5, 2.5, 2.5];
const SOLDIER_STEP: [f32; 8] = [0.0, 40.0, 40.0, 40.0, 40.0, 40.0, 40.0, 40.0];
/// **Sooner from planet 2** (2026-11-22 evening, the same thread). His
/// next run on #721 (build 0509374): planets 1-4 won in 3:51 to 5:02 with
/// 14 to 35 soldiers, 0 to 3 lost a planet; raids 0, 0, 1, 3 (the first
/// is drawn at 240 +/- 40 s, so a planet won by 4:00 met none) and the
/// army-sized rules above barely woke (under 30 alive on all but one).
///
/// - `RAID_FIRST`, `RAID_EVERY`: the first raid's mean time and the gap
///   at the landing (`RaidRules::first`, `every`; spreads kept at the same
///   share): 2:00 and 100 s from planet 2, so a 5-minute planet meets 2
///   or 3. Friday's hard planet 1 (11-21 04:49Z) had 14 in 16:33, one a
///   70 s; still under that.
/// - `DRAIN`: a touching walker's bite (`Rules::drain`, 2 since 11-12):
///   2.5 from planet 2, so walking into the slime costs a small squad too.
const RAID_FIRST: [f32; 8] = [240.0, 120.0, 120.0, 120.0, 120.0, 120.0, 120.0, 120.0];
const RAID_EVERY: [f32; 8] = [150.0, 100.0, 100.0, 100.0, 100.0, 100.0, 100.0, 100.0];
/// - `RAID_ARMOUR`: blows a marching raider takes (`RaidRules::
///   march_armour`) until the march meets the squad or its target; the
///   owner, same evening: "if the slime never reaches my soldiers,
///   increasing the bite doesn't matter. Tougher slime has a better
///   chance". 2 on planets 2-3, 3 on 4-6, 4 on 7-8 (mini-bosses take 4 to
///   15). The bite above stays: it is what a raid that arrives does.
const DRAIN: [f32; 8] = [2.0, 2.5, 2.5, 2.5, 2.5, 2.5, 2.5, 2.5];
const RAID_ARMOUR: [u32; 8] = [0, 2, 2, 3, 3, 3, 4, 4];
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
///
/// **The slime eats without emptying a crystal** (`crystal_keep`,
/// 2026-11-21, the thread "Make the slime a more interesting enemy"; the
/// owner, 11-21 06:30Z: "crystals never run out from eating or pumping,
/// and only your squad can empty one by draining it ... this 100%"). Walkers
/// still take their bite and the crystal keeps its charge; what they ate is
/// counted on the GPU (`pilesum`) so the surge clock (`time_surges`) runs
/// as before.
pub fn enemy_keys(e: f32) -> Vec<(&'static str, String)> {
    vec![
        ("surge", ((curve_at_u(&LONGEST_WAIT, e) as f32 * STRETCH).round() as u32).to_string()),
        ("surge_dash", curve_at_u(&SURGE_DASH, e).to_string()),
        ("roam", curve_at_u(&ROAM, e).to_string()),
        ("roam_share", format!("{}", curve_at(&ROAM_SHARE, e))),
        ("far_smell", "1".to_string()),
        ("lure_pull", format!("{LURE_PULL}")),
        ("tether", format!("{TETHER}")),
        // The Sporer's own spore numbers (`tiers::SPORE`) on every
        // planet: a fruit's spores homed by them until 11-21 (they no
        // longer home: engine.wgsl's `fly`), and no other spore flies on
        // these planets but the Sporer's, which already has them.
        ("spore_reach", "36".to_string()),
        ("spore_land", "1".to_string()),
        // Crystals never run out from the slime's eating (the owner,
        // 11-21 06:30Z): only the squad empties one, by draining it.
        ("crystal_keep", "1".to_string()),
        // A hit's hole holds for the first half of its crater (about
        // 1 s) before it closes from the rim (`crater_shut`, 11-21, the
        // Look lane's A2; the owner, 11-12: "a bigger and bigger hole that
        // doesn't refill while being shot at").
        ("crater_shut", "0.5".to_string()),
        // A mini-boss's walkers flinch out of a fresh hit for the first
        // half of its crater, as long as the hole is shut (`boss_flinch`,
        // 11-21, Direction's job from the Look lane's exit test 2; a
        // quarter dented the wanderer less, and the whole crater blew it
        // into specks that never regrouped).
        ("boss_flinch", "0.5".to_string()),
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

/// **The slime's least walkers a covered crystal cell** (`Rules::ceiling`,
/// 2026-11-21; `larder_world/ceiling.rs`): its ceiling is the crystal it
/// covers times its landing density, at least this. On planet 1 (run seeds
/// 1-3) it landed with 14 to 21 a cell, so this lifts only a thin landing.
pub const CEILING: f32 = 12.0;

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
pub const ENEMY_RULES: &[(&str, f32)] = &[("unit_paths", 1.0), ("fruit", 1.0), ("wave", 1.0), ("heart", 1.0), ("strikers", 1.0), ("late_at", 0.4), ("stand", 1.0), ("ceiling", CEILING), ("origin", 1.0)];

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
pub fn enemy_rules(e: f32) -> impl Iterator<Item = (&'static str, f32)> {
    ENEMY_RULES.iter().map(move |&(k, v)| (k, if e >= first_planet(k) as f32 { v } else { 0.0 }))
}

/// **The run's two length dials** (2026-11-24, Direction's ask after the
/// owner's answer that day, 03:14Z: "It should be longer, at least 2
/// hours, but we can also double the number of planets. We also don't
/// need to decide exactly now"). `PLANETS` is how many planets a run has;
/// the per-planet tables below hold eight values, tuned on eight planets,
/// and a run of another length spreads its planets over the same curve
/// (`curve_depth`, `curve_at`), so planet 1 and the last keep the first
/// and last values. `STRETCH` makes each planet longer: the clocks the
/// slime's threats keep from the landing (raids, surges, mini-bosses)
/// and the planet's star and far limit (`par`, `limit`) all run that many
/// times as long. At 8 planets and a stretch of 1 every number is as it
/// was (the test `the_length_dials_change_nothing_at_eight_planets`).
pub const STRETCH: f32 = 1.0;

/// Planet `depth` as a place on the eight-planet curve the tables were
/// tuned on: 1 to 8 at `PLANETS` 8, the same span for any other count.
pub fn curve_depth(depth: u32) -> f32 {
    curve_depth_of(depth, PLANETS)
}

fn curve_depth_of(depth: u32, planets: u32) -> f32 {
    if planets <= 1 {
        return 1.0;
    }
    let d = depth.clamp(1, planets);
    1.0 + (d - 1) as f32 * 7.0 / (planets - 1) as f32
}

/// A table's value at curve place `e`, read off its eight values between
/// the two nearest (exactly the value at a whole place on the curve).
fn curve_at(table: &[f32; 8], e: f32) -> f32 {
    let x = (e - 1.0).clamp(0.0, 7.0);
    let i = x.floor() as usize;
    let f = x - i as f32;
    if f == 0.0 {
        table[i]
    } else {
        table[i] + (table[i + 1] - table[i]) * f
    }
}

fn curve_at_u(table: &[u32; 8], e: f32) -> u32 {
    curve_at(&table.map(|v| v as f32), e).round() as u32
}

/// The planet of the eight tuned ones nearest curve place `e`, 1 to 8:
/// what rolls the shop, the species and the map's size and biome.
pub fn nearest(e: f32) -> u32 {
    (e.round() as u32).clamp(1, 8)
}

/// The food a planet's slime eats to build a surge (`Rules::surge_food`):
/// its share of the sandbox's, times the level's walkers (`Level::walkers`,
/// as the slime's slots are), since a bigger slime on a bigger planet
/// eats more and would otherwise surge at the floor all game.
pub fn surge_food(e: f32, walkers: f32) -> f32 {
    curve_at(&SURGE_FOOD_SHARE, e) * crate::mould::larder_world::SURGE_FOOD * walkers.max(0.1)
}

/// A planet's longest wait between surges in game seconds (`enemy_keys`),
/// for the shop.
pub fn surge_secs(e: f32) -> f32 {
    curve_at_u(&LONGEST_WAIT, e) as f32 * STRETCH / 30.0
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
        Tier::Drifter => DoesToYou { says: "It smells your pumps. Every pump you build calls it to come and eat", rules: &[("pump_smell", 20.0), ("lure_r", 15.0)], keys: &[] },
        Tier::Sprawler => DoesToYou { says: "It reaches out in threads that find your soldiers. It fears fire: bring Tinder and a Torch", rules: SPRAWLER_RULES, keys: SPRAWLER_KEYS },
        // Until 2026-11-17 the Trunk had the Drifter's row (the same pump
        // smell: the first two pumps at 185 and 167 hp at 2:30 without it,
        // 160 and 160 with it; modest, as its cords are slow to turn), so
        // planets 1 and 3 did the same thing to you. Now it sets
        // (`TRUNK_KEYS`), and since 11-18 it walls the crystals it holds
        // (`fort` in `TRUNK_RULES`).
        Tier::Trunk => DoesToYou { says: "It walls its crystals in amber and sets hard where you shoot it. Burn the walls", rules: TRUNK_RULES, keys: TRUNK_KEYS },
        Tier::Swarmer => DoesToYou { says: "Its surges charge you from far off. Cut it to a quarter and all of it comes", rules: SWARMER_RULES, keys: SWARMER_KEYS },
        Tier::Sporer => DoesToYou { says: "Struck, it scatters spores that grow new slime. Kill it fast, or fight its seedlings", rules: &[], keys: SPORER_KEYS },
        Tier::Sleeper => DoesToYou { says: "It grows while it sleeps. Every minute you farm, there is more to wake", rules: &[], keys: SLEEPER_KEYS },
        Tier::Grudge => DoesToYou { says: "Where you hurt it, it grows back tough. Don't fight on old ground", rules: &[], keys: GRUDGE_KEYS },
        Tier::Dozer => DoesToYou { says: "It sleeps off every charge. Each shot wakes it, and awake it charges", rules: SWARMER_RULES, keys: DOZER_KEYS },
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
//
// **Its body as threads** (2026-11-24; the Lab's prototype of that day,
// `Reports/lab-threads-2026-11-24/`, built in at Direction's word): lanes
// (3 more walkers on a cell holding as much trail as the walker's own) and
// the shuttle (a walker past 0.9 of its birth energy walks down the
// crystals' scent at 3, back up it when hungry), with the trail kept 0.92
// (its walk's 0.8) and sensors 6 fine cells out (its 18), GPU only
// (`gpu::Settings::thread_lanes`). Planet 2, seeds 1-3, at the off arm's
// walkers: the disc round each crystal opens into four to seven spokes,
// holes 25-29% of the body's outline against 0-8%. Neither rule alone
// does it; with its own walk (0.8, 18) both made only a rough cross.
const SPRAWLER_KEYS: &[(&str, &str)] = &[("fire_alarm", "50"), ("keep", "0.92"), ("sensor_offset", "6"), ("thread_lanes", "3"), ("thread_trail", "1.0"), ("thread_shuttle", "3"), ("thread_fed", "0.9")];

/// The Sprawler's thread keys, the GPU engine's alone (no CPU twin).
pub const THREAD_KEYS: [&str; 4] = ["thread_lanes", "thread_trail", "thread_shuttle", "thread_fed"];
/// The Grudge's scar tissue keys, the GPU engine's alone (no CPU twin).
pub const TOUGH_KEYS: [&str; 3] = ["scar_tough", "scar_tough_full", "scar_burn"];
/// The Sleeper's dreaming key, the GPU engine's alone (no CPU twin).
pub const DREAM_KEYS: [&str; 1] = ["dream"];
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
//
// **It grows while it sleeps** (2026-11-24, Direction's pick for planet 6's
// nameable thing; the Lab's prototype, `Reports/lab-sleeper-2026-11-24/`):
// a sleeping walker buds a sleeping child at 0.003 a step, under the birth
// ceiling and off the crystals' scent (`gpu::Settings::dream`), so the time
// you farm before it wakes is time it grows. 0.003 is the Lab's pick (one
// seed); its growth is bounded by the ceiling and its free edge.
const SLEEPER_KEYS: &[(&str, &str)] = &[("wake_alarm", "0.3"), ("dream", "0.003")];
// The Dozer, planet 8 (2026-11-24, Direction's card: "It sleeps off every
// charge. Each shot wakes it, and awake it charges"): the Swarmer's lunge
// and last stand (`SWARMER_RULES`) on a body that sleeps away from its
// crystals, woken at the Sleeper's planet 6 bar (wake_alarm 0.3). No
// dreaming: that is planet 6's.
const DOZER_KEYS: &[(&str, &str)] = &[("surge_haste", "3"), ("wake_alarm", "0.3")];
// Grudge, planet 7 (joined 2026-11-17, when the GPU took its scar): its
// memory of where it was hit pulls it back at twice the campaign tier's
// weight (scar_weight 1, not 0.5), so a firing line it was hurt from draws
// it back after the alarm has faded. A first number, not yet filmed on a
// planet (`Reports/sim-use-2026-11-17/step0-scar-on-gpu.md`).
//
// **Scar tissue** (2026-11-24, Direction's pick for planet 7's nameable
// thing; the Lab's prototype, `Reports/lab-scar-tissue-2026-11-24/`): a blow
// on a walker standing on scar misses with chance 0.5 x min(scar / 2,000,
// 1), so the ground the squad has fought over grows tough slime, and the
// pull above brings the slime back to it. Fire burns the scar off. 2,000 is
// about a minute of fire on one place (the Lab's median scar near a holding
// squad: 513 at 1:00, 3,500 at 1:30); P 0.75 took the run bot from 30
// soldiers to 4 by 2:00 against 21 at 0.5 and 30 off (one seed), so 0.5.
const GRUDGE_KEYS: &[(&str, &str)] = &[("scar_weight", "1"), ("scar_tough", "0.5"), ("scar_tough_full", "2000"), ("scar_burn", "1")];

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

/// A card only a gun's bullet uses (`Run::body_cards` leaves it off a
/// device): its flight, not what it casts.
pub fn gun_only(c: Card) -> bool {
    matches!(c, Card::Pierce | Card::Split | Card::Range | Card::Rate | Card::Seek)
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
///
/// A counter for every slime (Direction 11-24, Milestone 2; run seed 1,
/// greedy squad, paired films, `Reports/slime-counters-2026-11-24/`):
/// - The Swarmer and the Dozer: a Bomb. Its ring throws the charge back:
///   planet 4, 25 soldiers at 1:30 where plain shots were wiped at 0:44
///   (seed 2: 21 against 11); planet 8, 13 at 1:00 against wiped at 0:48.
///   A blast was wiped too (0:40).
/// - The Sporer: a Germ (frail 6, no resistance): its slime at 0:40 was
///   18,714 against 91,854 with plain shots, three seeds alike.
/// - The Sleeper: a Flare. The germ does nothing to its sleeping crust
///   (`tiers.rs`); a flare wakes it and a germ then takes it: 10 soldiers
///   at 1:00 against wiped at 0:38. The germ comes first, on planet 5.
/// - Not the Grudge: fire burns its scar off, but the scar takes over a
///   minute to grow, and no film yet shows the Torch answer it.
pub fn bring_shell(tier: Tier) -> Option<Shell> {
    match tier {
        Tier::Sprawler | Tier::Trunk => Some(Shell::Torch),
        Tier::Swarmer | Tier::Dozer => Some(Shell::Bomb),
        Tier::Sporer => Some(Shell::Germ),
        Tier::Sleeper => Some(Shell::Flare),
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

/// The dev skip (owner, 2026-11-22 21:36Z: "a dev way to play the later
/// planets of the run ... skip planets and get average amount of gold to
/// spend at the shop"): what a skipped planet pays, as an average win
/// there pays. Two parts, so a retune of `reward` carries over:
///
/// - `SKIP_STARS_X10`, the average stars of a win, in tenths. Planets 1 to
///   3: the bots that pick up drops in job 14-0-f (11-21, `drops=` 5, 20
///   and 60 pooled, 11 maps on planets 1 and 2, 9 on planet 3; 2.24, 2.57,
///   2.07 stars). Planets 4 on: the bot curve's wins on planets 4 and 5
///   (1.83 and 1.5 on few games), held at 1.8.
/// - `SKIP_FOUND`, the average points a win takes from starfall ore and
///   boss cores (`Run::found`). Planets 1 to 3: the same games (101, 197,
///   159). Planets 4 on: no bot data, so the same share of the planet's
///   pot those three took (36% of falls x ore worth + core pot,
///   `StarRules::for_depth`, about 4.2 falls a planet).
///
/// Bots that never chase drops take 20 to 30 a planet instead, about 100
/// to 170 less than this; a player who picks up drops is the one priced.
/// The home world (planet `PLANETS`) can't be skipped: winning it ends
/// the run. Rejected: the dev rows' flat 1,000 points, which is not what
/// a run would have had to spend.
pub const SKIP_STARS_X10: [u32; 7] = [22, 26, 21, 18, 18, 18, 18];
pub const SKIP_FOUND: [u32; 7] = [100, 200, 160, 210, 235, 265, 295];

/// The points skipping planet `depth` pays (`SKIP_STARS_X10`,
/// `SKIP_FOUND`), rounded to tens; 0 for the home world.
pub fn skip_pay(depth: u32) -> u32 {
    if depth == 0 || depth >= PLANETS {
        return 0;
    }
    // By the tuned planet nearest the step (twelve steps, 2026-11-26).
    let d = nearest(curve_depth(depth)).min(7);
    let i = d as usize - 1;
    let base = reward(d, 1) + 50 * SKIP_STARS_X10[i].saturating_sub(10) / 10;
    (base + SKIP_FOUND[i] + 5) / 10 * 10
}

/// What a shop row is.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Group {
    Squad,
    Build,
    Tools,
    /// Rare run-wide rules (gun step F), after planets 3, 5 and 7.
    Relics,
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
            Group::Relics => "Relics",
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
pub const ITEMS: [Item; 21] = [
    Item { key: "hp", name: "Armour", group: Group::Squad, about: "Soldiers take a quarter more biting before they fall", max: 4, cost: 120, needs: None },
    Item { key: "strike", name: "Heavier shots", group: Group::Squad, about: "Shots a fifth wider; past the widest, they fire faster", max: 4, cost: 120, needs: None },
    Item { key: "rate", name: "Faster trigger", group: Group::Squad, about: "Soldiers fire 15% sooner", max: 4, cost: 120, needs: None },
    Item { key: "range", name: "Longer barrels", group: Group::Squad, about: "3 more cells of sight and reach", max: 3, cost: 100, needs: None },
    Item { key: "squad", name: "Bigger landing", group: Group::Squad, about: "Two more soldiers land with you", max: 4, cost: 150, needs: None },
    Item { key: "bank", name: "Supply drop", group: Group::Squad, about: "Land with 500 more food in the bank", max: 4, cost: 100, needs: None },
    Item { key: "harvest", name: "Quick hands", group: Group::Squad, about: "Soldiers take a quarter more from a crystal each second", max: 3, cost: 100, needs: None },
    // Fire resistance (the owner, 2026-11-22 07:40Z: "you should be able to
    // upgrade fire resistance ... or even immunity"). Fire hurts only where
    // the squad's own fire cards lit it (Torch, Tinder:
    // `Rules::fire_hurt`), so this alone does nothing; it is the half of a
    // fire deck that makes the deck safe, the owner's Noita rule (11-18: no
    // single card is strong alone; strong builds are combinations). Halves
    // fire's harm a level and the third level is immunity, 600 points in
    // all, two to three planets' pay (`fireproof_share`; an immune squad
    // also walks straight through fire, `Game::fire_via`).
    Item { key: "fireproof", name: "Fireproof", group: Group::Squad, about: "Soldiers take half the harm from fire; at level 3, none", max: 3, cost: 100, needs: None },
    Item { key: "walls", name: "Walls", group: Group::Build, about: "Unlocks laying walls", max: 1, cost: 100, needs: None },
    Item { key: "turret", name: "Turrets", group: Group::Build, about: "Unlocks the turret, a gun that does not move", max: 1, cost: 200, needs: None },
    // The tree's first body (the turrets thread, owner 2026-11-17 00:47Z):
    // after turrets, as each next body will be after the one before.
    Item { key: "mortar", name: "Mortars", group: Group::Build, about: "Unlocks the mortar: the longest reach there is, lobbed over walls", max: 1, cost: 250, needs: Some("turret") },
    // The second body (Direction 11-24 11:42Z): a field layer, wants
    // field cards in the turrets' deck.
    Item { key: "sprayer", name: "Sprayers", group: Group::Build, about: "Unlocks the sprayer: lays the turrets' cards in a strip across the slime's way; wants gas, fire or crater cards, weak with Plain", max: 1, cost: 250, needs: Some("mortar") },
    // The third body (Direction 11-24 12:33Z): picky aim, slime on a
    // crystal first.
    Item { key: "hunter", name: "Hunters", group: Group::Build, about: "Unlocks the hunter: one fast flat shot at what matters most, the slime on a crystal first, then fruit, mini-bosses, raids and the heart", max: 1, cost: 250, needs: Some("sprayer") },
    Item { key: "turret+", name: "Turret power", group: Group::Build, about: "Turrets, mortars, sprayers and hunters: 30% tougher, 3 cells more reach, fire 15% sooner", max: 3, cost: 150, needs: Some("turret") },
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

/// The share of fire's harm (`Rules::fire_hurt`) a squad with `level`
/// levels of Fireproof takes: half a level, none at the third (immune).
pub fn fireproof_share(level: u8) -> f32 {
    if level >= 3 {
        0.0
    } else {
        0.5f32.powi(i32::from(level))
    }
}

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
/// **Relics** (gun step F, Direction 11-24 08:54Z and 09:51Z; the
/// gun-system plan's "rare run-wide rules"): one is for sale in the shops
/// of `RELIC_DEPTHS`, after planets 3, 5 and 7, at `Relic::price`; owned,
/// it holds for the rest of the run.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub enum Relic {
    /// Slime the squad kills draws calm slime to where it died
    /// (`RoundRules::carrion_secs`).
    Carrion,
    /// Sick slime lays double trail (the engine's `sick_trail`, 1 to 2;
    /// `Run::engine_keys`), so the germ's sickness runs farther down the
    /// tubes it thickens (Direction 11-24 11:28Z: built here, no shader
    /// change).
    Fever,
    /// A device's fire also lights the ground the squads left
    /// (`RoundRules::backdraft_gap`).
    Backdraft,
}

impl Relic {
    pub const ALL: [Relic; 3] = [Relic::Carrion, Relic::Fever, Relic::Backdraft];

    pub fn key(self) -> &'static str {
        match self {
            Relic::Carrion => "carrion",
            Relic::Fever => "fever",
            Relic::Backdraft => "backdraft",
        }
    }

    pub fn name(self) -> &'static str {
        match self {
            Relic::Carrion => "Carrion",
            Relic::Fever => "Fever",
            Relic::Backdraft => "Backdraft",
        }
    }

    pub fn about(self) -> &'static str {
        match self {
            Relic::Carrion => "For the rest of the run, where a soldier's shot kills slime, calm slime streams to the spot for 3 s: into your guns when you hold a line, onto you when you are overrun.",
            Relic::Fever => "For the rest of the run, sick slime lays twice the trail, so a germ's sickness runs much farther down the slime's tubes, and those tubes grow thicker.",
            Relic::Backdraft => "For the rest of the run, a device's fire also lights the ground your squads held and left: a wall of fire behind you, and a trap if you fall back onto it.",
        }
    }

    pub fn price(self) -> u32 {
        match self {
            Relic::Carrion => 300,
            Relic::Fever => 250,
            Relic::Backdraft => 300,
        }
    }

    pub fn of_key(k: &str) -> Option<Relic> {
        Relic::ALL.into_iter().find(|r| r.key() == k)
    }
}

/// The shops that offer a relic: after planets 3, 5 and 7.
pub const RELIC_DEPTHS: [u32; 3] = [4, 6, 8];

/// The first paid reroll's price (`Run::reroll_price`, gun step F; the
/// gun-system plan's "rerolls that cost more each time"; a tier-1 card is
/// 80): it doubles each time in one shop. A first guess.
pub const REROLL_PRICE: u32 = 25;

/// Device bodies for sale in each shop from planet 2 (`Run::body_rolls`).
pub const BODY_OFFER: usize = 2;

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

/// The first depth whose shop sells the cards that only alarm the slime
/// (the Flare bullet) or take the alarm away (Quiet): the first planet
/// whose slime may answer an alarm (`tiers::params`: the Sleeper, Swarmer
/// and Grudge walk to it, from planet 4; the planets before ignore it).
pub const ALARM_FROM: u32 = 4;

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
    // The dart (2026-11-24) from the start, as the Lure it is a form of:
    // every planet's slime answers a lure (`run::LURE_PULL`).
    let mut v = vec![Shell::Plain, Shell::Blast, Shell::Dart];
    if depth >= power_from(2) {
        v.extend([Shell::Gas, Shell::Blind, Shell::Sour, Shell::Spin, Shell::Provoke, Shell::Still, Shell::Shatter, Shell::Torch]);
    }
    // The flare kills nothing and only alarms, which no slime answers
    // before `ALARM_FROM` (2026-11-22, exit test 5): sold before it, its
    // ember burned on planets 1 to 3 and the slime walked on.
    if depth >= ALARM_FROM {
        v.push(Shell::Flare);
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

/// A card's family for the shop's mix (gun step F, `Run::luck`): what
/// it works on. Bullets (payloads) are not drawn among the cards.
pub fn family(c: Card) -> &'static str {
    match c {
        Card::Pierce | Card::Split | Card::Range | Card::Rate | Card::Seek => "flight",
        Card::Scorch | Card::Bait | Card::Mist | Card::Smother | Card::Tinder => "trail",
        Card::Wide | Card::Scatter | Card::Thick | Card::Delay | Card::Quiet | Card::Thrift => "shape",
        Card::Heave | Card::Pull => "force",
        Card::Trigger | Card::Expire | Card::Multicast => "rule",
        _ => "bullet",
    }
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
    /// The dev skip (`skip_pay`): this planet counted as won without
    /// playing it, its average pay added, and the next planet's shop.
    /// In every run's shop, dev mode or not, so it can be found.
    DevSkip,
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
    /// The devices carry the loadout shown's cards (`Run::device_loadout`,
    /// gun step E): the default for a body with no loadout of its own.
    DevicesUse,
    /// Body on offer `i` (gun step E3): bought, in place of the body of its
    /// kind owned.
    Body(usize),
    /// Owned body `i` into the loadout shown, which gives up its gun.
    CarryBody(usize),
    /// The relic on offer bought (gun step F).
    Relic,
    /// The card offers redrawn for `reroll_price` (gun step F): the guns
    /// and bodies on offer stay, the held card too.
    Reroll,
    /// The hold moves to the next card on offer, or off after the last
    /// (gun step F): a held card comes back in every shop until bought.
    Hold,
    /// Owned body `i`'s autocast switch flips (gun step E4,
    /// `Body::auto`); in the field too.
    BodyAuto(usize),
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
    /// Its device body, an index into `Run::bodies` (gun step E3): a
    /// loadout holds a gun or a body, and a body's carries its cards
    /// (`Run::body_cards`) fitted to the body's rack.
    pub body: Option<usize>,
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
    /// The loadout whose cards the devices carry (gun step E, 2026-11-24):
    /// the default for a body with no loadout of its own (`body_cards`).
    /// None, none, until the shop's "Devices use this" picks one.
    pub device_loadout: Option<usize>,
    /// The shop is the loadout panel opened on a planet (H): only the guns
    /// and cards owned, nothing for sale, the game paused under it.
    pub field: bool,
    /// The squad the panel was opened for and the loadout it uses, which
    /// `Pick::SquadUses` changes; the window applies it on closing.
    pub field_squad: Option<(u8, usize)>,
    /// This shop's guns for sale.
    pub gun_offers: Vec<Gun>,
    /// Device bodies owned (gun step E3, `bodies.rs`): at most one of each
    /// kind, each the tool its kind is.
    pub bodies: Vec<Body>,
    /// This shop's bodies for sale.
    pub body_offers: Vec<Body>,
    /// Stars won on each level so far.
    pub stars: Vec<u8>,
    /// The run is over (a level was lost) at this depth.
    pub over: bool,
    /// Dev mode: the shop has rows to jump planets, add points and redraw.
    pub dev: bool,
    /// Redraws so far (dev mode and `Pick::Reroll`), so each draws anew.
    rerolls: u64,
    /// **Luck's shop levers** (gun step F, Direction 11-24 08:54Z, first
    /// guesses): the card offer held for the next shops (`Pick::Hold`),
    /// paid rerolls in this shop (`Pick::Reroll`, `reroll_price`), and
    /// shops in a row with no tier-3 card on offer (`pity`).
    pub held: Option<Card>,
    pub paid_rerolls: u32,
    pub since_rare: u32,
    /// Relics owned, and the one this shop sells (`RELIC_DEPTHS`).
    pub relics: Vec<Relic>,
    pub relic_offer: Option<Relic>,
    /// The run ended on a lost level.
    beaten: bool,
    /// The gun range (`Run::range`): a free armory, not a run.
    pub range: bool,
    /// Real seconds spent playing the run's planets (Milestone 2 test 2,
    /// 11-24: the run's play time on the end page and "Back to the run"):
    /// unpaused, no menu or shop or loadout panel, fast forward counted as
    /// the real time it took. The window adds it up (`App::count_played`);
    /// saved in `run.txt`, so a quit mid-planet keeps it.
    pub played: f32,
    /// `played` as each planet ended (won, lost or skipped), from planet 1;
    /// below zero where it is unknown (a save from before these, 11-24).
    /// The Runs page's time on each planet (`planet_secs`) and best
    /// times per planet (Direction 11-24).
    pub splits: Vec<f32>,
    /// **The route taken on the star map** (2026-11-26, `route`): the node
    /// picked at each step so far, from step 1, and whether its heat was
    /// on. Shorter than `depth` while the player is choosing the next
    /// planet (`choosing`). A save from before it reads as node 0 (the
    /// Fair planet) everywhere, which is always a path.
    pub route: Vec<Leg>,
    /// Tests: the run read as tuned planet `tuned` of the eight, whatever
    /// its step (`Run::tuned`), so the tests written for eight planets
    /// still name them.
    #[cfg(test)]
    pub tuned: Option<f32>,
}

/// One step of the route (`Run::route`): the node picked (`route::Node`'s
/// `k`) and whether its **heat** was on (direction 4: a bigger, faster
/// slime for more pay, `HEAT_*`).
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub struct Leg {
    pub k: u8,
    pub heat: bool,
}

/// What heat does (first guesses, 2026-11-26; balance later): the slime a
/// quarter bigger, the first raid at 60% of its time and the raids after
/// at 80%, for half again the planet's pay.
pub const HEAT_WALKERS: f32 = 1.25;
pub const HEAT_RAID_FIRST: f32 = 0.6;
pub const HEAT_RAID_EVERY: f32 = 0.8;
pub const HEAT_PAY_X10: u32 = 15;

/// The depth the gun range rolls its guns and bullets for at first.
pub const RANGE_DEPTH: u32 = 3;

fn level_of(owned: &[u8; ITEMS.len()], key: &str) -> u8 {
    ITEMS.iter().position(|i| i.key == key).map_or(0, |k| owned[k])
}

/// **A planet's twist** (2026-11-24, Direction; `larder_world::twist`):
/// the cold snap or the wind, rolled from the run's seed and the planet
/// (`Run::twist`), so a saved run keeps its rolls. From planet
/// `TWIST_FROM`, never on two planets in a row; after a planet without
/// one, one in `TWIST_ODDS`, which comes to about one planet in three.
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct PlanetTwist {
    pub kind: Twist,
    /// The wind's way, degrees (0 east, 90 south): one of the eight
    /// compass points.
    pub angle: f32,
}

pub const TWIST_FROM: u32 = 2;
const TWIST_ODDS: u64 = 2;

/// A compass point's name for an angle (0 east, 90 south, as the map
/// draws it).
fn compass(deg: f32) -> &'static str {
    const NAMES: [&str; 8] = ["east", "south-east", "south", "south-west", "west", "north-west", "north", "north-east"];
    NAMES[((deg.rem_euclid(360.0) / 45.0).round() as usize) % 8]
}

impl PlanetTwist {
    /// Its name on the planet's card and the HUD.
    pub fn name(&self) -> &'static str {
        match self.kind {
            Twist::Cold => "Cold snap",
            Twist::Wind => "Wind",
            Twist::None => "",
        }
    }

    /// Its line on the planet's card (Direction's words, 11-24 17:24Z).
    pub fn card(&self) -> String {
        match self.kind {
            Twist::Cold => {
                let d = crate::mould::larder_world::TwistRules::default();
                let at = d.start as u32;
                format!("Cold snap at {}:{:02}: for a minute the slime crawls in beads. Push now.", at / 60, at % 60)
            }
            Twist::Wind => format!("Wind from the {}: the slime leans and creeps {} all planet. Stand upwind, or let it blow onto your guns.", compass(self.angle + 180.0), compass(self.angle)),
            Twist::None => String::new(),
        }
    }
}

/// The twist a run of `seed` rolls on planet `depth` (before the star
/// map; its Fair planets keep the odds, `Run::twist_of`).
#[cfg(test)]
fn twist_at(seed: u64, depth: u32) -> Option<PlanetTwist> {
    if depth < TWIST_FROM || twist_at(seed, depth - 1).is_some() {
        return None;
    }
    let z = mix(seed ^ 0x5457_4953_5400_0000 ^ u64::from(depth).wrapping_mul(0x2545_F491_4F6C_DD1D));
    if !z.is_multiple_of(TWIST_ODDS) {
        return None;
    }
    let kind = if (z >> 8).is_multiple_of(2) { Twist::Cold } else { Twist::Wind };
    Some(PlanetTwist { kind, angle: ((z >> 16) % 8) as f32 * 45.0 })
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
        Run { seed, depth: 1, points: 0, owned, cards: Vec::new(), offers: Vec::new(), guns: vec![first_gun()], loadouts: vec![Loadout { gun: Some(0), body: None, cards: Vec::new() }, Loadout::default()], editing: 0, turret_loadout: 1, device_loadout: None, field: false, field_squad: None, gun_offers: Vec::new(), bodies: Vec::new(), body_offers: Vec::new(), stars: Vec::new(), over: false, dev: false, rerolls: 0, held: None, paid_rerolls: 0, since_rare: 0, relics: Vec::new(), relic_offer: None, beaten: false, range: false, played: 0.0, splits: Vec::new(), route: vec![Leg::default()], #[cfg(test)] tuned: None }
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

    /// The current planet's twist (`PlanetTwist`), None on most.
    pub fn twist(&self) -> Option<PlanetTwist> {
        if self.range {
            return None;
        }
        self.twist_of(self.depth, &self.star_map())
    }

    /// The twist on the route's planet at `step` (star map, 2026-11-26): a
    /// Safe planet never has one and a Hard one always does; a Fair one
    /// rolls as every planet did (`twist_at`'s odds, never two in a row).
    fn twist_of(&self, step: u32, map: &StarMap) -> Option<PlanetTwist> {
        if step < TWIST_FROM {
            return None;
        }
        let n = self.node_in(map, step);
        let z = mix(n.seed ^ 0x5457_4953_5400_0000);
        let roll = || {
            let kind = if (z >> 8).is_multiple_of(2) { Twist::Cold } else { Twist::Wind };
            Some(PlanetTwist { kind, angle: ((z >> 16) % 8) as f32 * 45.0 })
        };
        match n.pip {
            Pip::Safe => None,
            Pip::Hard => roll(),
            Pip::Fair if self.twist_of(step - 1, map).is_some() || !z.is_multiple_of(TWIST_ODDS) => None,
            Pip::Fair => roll(),
        }
    }

    /// The run's star map (`route::star_map`), from its seed.
    pub fn star_map(&self) -> StarMap {
        route::star_map(self.seed, PLANETS, curve_depth)
    }

    /// The route's leg at the current step (node 0, no heat, until picked).
    pub fn leg(&self) -> Leg {
        self.route.get((self.depth as usize).saturating_sub(1)).copied().unwrap_or_default()
    }

    fn node_in(&self, map: &StarMap, step: u32) -> Node {
        let k = self.route.get((step as usize).saturating_sub(1)).map_or(0, |l| l.k);
        map.node(step, k).or_else(|| map.node(step, 0)).cloned().expect("every step has a node 0")
    }

    /// The route's node at the current step.
    pub fn node(&self) -> Node {
        self.node_in(&self.star_map(), self.depth.clamp(1, PLANETS))
    }

    /// The current planet's difficulty (Fair in the gun range).
    pub fn pip(&self) -> Pip {
        if self.range {
            Pip::Fair
        } else {
            self.node().pip
        }
    }

    /// The current planet's place on the tuned eight-planet curve: the
    /// step's (`curve_depth`) moved by its pip (`Pip::offset`), 1 to 8.
    /// Every per-planet table of the slime's is read here.
    pub fn curve(&self) -> f32 {
        (self.step_curve() + self.pip().offset()).clamp(1.0, 8.0)
    }

    /// The step's own place on the curve (`curve_depth`).
    fn step_curve(&self) -> f32 {
        #[cfg(test)]
        if let Some(t) = self.tuned {
            return t;
        }
        curve_depth(self.depth)
    }

    /// Tests: tuned planet `d` of the eight (`tuned`) at step `d`.
    #[cfg(test)]
    pub fn tuned(seed: u64, d: u32) -> Run {
        let mut r = Run::new(seed);
        r.set_tuned(d);
        r
    }

    #[cfg(test)]
    pub fn set_tuned(&mut self, d: u32) {
        self.depth = d;
        self.tuned = Some(d as f32);
    }

    /// The depth the level generator builds the current planet at (its
    /// size and biome): the tuned planet nearest `curve`, and the core
    /// world only at the last step.
    pub fn map_depth(&self) -> u32 {
        if self.depth >= PLANETS || self.step_curve() >= 8.0 {
            levelgen::CORE_DEPTH
        } else {
            nearest(self.curve()).min(levelgen::CORE_DEPTH - 1)
        }
    }

    /// The depth the shop rolls its guns, cards and bodies for: the tuned
    /// planet nearest the step, so twelve steps keep the eight-planet
    /// shop (the gun range's own depth there).
    pub fn roll_depth(&self) -> u32 {
        if self.range {
            self.depth
        } else {
            nearest(self.step_curve())
        }
    }

    /// The player is to pick the next planet on the star map: a fork,
    /// once a planet is won (`won` picks a step with one way on).
    pub fn choosing(&self) -> bool {
        !self.range && !self.over && self.route.len() < self.depth as usize
    }

    /// The planets the route can go to at the current step.
    pub fn choices(&self) -> Vec<Node> {
        let from = (self.depth as usize).checked_sub(2).and_then(|i| self.route.get(i)).map(|l| l.k);
        self.star_map().choices(self.depth, from).into_iter().cloned().collect()
    }

    /// Picks node `k` for the current step, with or without heat, and
    /// draws its shop (a Hard planet's rare offer comes with it); false if
    /// no pick is due or `k` is not a way on.
    pub fn choose(&mut self, k: u8, heat: bool) -> bool {
        if !self.choosing() || !self.choices().iter().any(|n| n.k == k) {
            return false;
        }
        self.route.resize(self.depth as usize - 1, Leg::default());
        self.route.push(Leg { k, heat });
        self.restock();
        true
    }

    /// The step's one way on, taken when it has no fork.
    fn take_only_way(&mut self) {
        if !self.choosing() {
            return;
        }
        let c = self.choices();
        if c.len() == 1 {
            self.route.resize(self.depth as usize - 1, Leg::default());
            self.route.push(Leg { k: c[0].k, heat: false });
        }
    }

    /// The points winning the current planet with `stars` pays: `reward`
    /// for the tuned planet nearest the step, times its pip's share
    /// (`Pip::pay_x10`) and its heat's (`HEAT_PAY_X10`), rounded to fives.
    pub fn pay(&self, stars: u8) -> u32 {
        let base = reward(self.roll_depth(), stars);
        if self.range {
            return base;
        }
        let heat = if self.leg().heat { HEAT_PAY_X10 } else { 10 };
        (base * self.pip().pay_x10() * heat + 250) / 500 * 5
    }

    /// The current planet's contracts, each kept or not by `o`, and its
    /// pay (paid into `points` for those kept). Before `won`.
    pub fn settle(&mut self, o: &Outcome) -> Vec<(Contract, bool, u32)> {
        if self.range {
            return Vec::new();
        }
        let e = self.curve();
        let out: Vec<(Contract, bool, u32)> = self.node().contracts.iter().map(|&c| (c, c.met(o), c.pay(e))).collect();
        self.points += out.iter().filter(|c| c.1).map(|c| c.2).sum::<u32>();
        out
    }

    /// A Hard planet's rare offer (the brief's "a rare card or gun
    /// offered in the next shop"): a tier-3 card the run doesn't own,
    /// drawn by the planet's seed, or None.
    pub fn rare_offer(&self) -> Option<Card> {
        if self.range || self.pip() != Pip::Hard {
            return None;
        }
        let owned: Vec<Card> = self.cards.iter().map(|c| c.0).collect();
        let pool: Vec<Card> = Card::ALL.into_iter().filter(|&c| c.kind() != Kind::Payload && c.tier() >= 3 && card_power(c).is_some() && !owned.contains(&c)).collect();
        (!pool.is_empty()).then(|| pool[(mix(self.node().seed ^ 0x5241_5245) % pool.len() as u64) as usize])
    }

    /// The planet card for the current depth (name, biome, size), with no
    /// map built.
    pub fn planet(&self) -> Planet {
        if self.range {
            return levelgen::planet(self.seed, self.depth);
        }
        levelgen::planet(self.node().seed, self.map_depth())
    }

    /// The level at the current depth.
    pub fn level(&self) -> Level {
        let d = self.depth;
        // The star map's node (2026-11-26): its world, its place on the
        // curve and the map depth the level generator builds it at.
        let node = self.node();
        let (e, md) = (self.curve(), self.map_depth());
        let seed = mix(node.seed ^ u64::from(d).wrapping_mul(0x5851_F42D));
        // The planet the level generator makes for this run and depth
        // (its name and biome come with it); `map` as `map=` names it, so
        // the window's title and alert read its name.
        let planet = self.planet();
        let map = format!("planet:{md}:{}", node.seed);
        // The species of the tuned planet nearest its curve place: each
        // shows up at more than one step, and a Hard fork meets a later
        // one than its Fair neighbour, a Safe fork an earlier one.
        let tier = LADDER[nearest(e) as usize - 1];
        let shrink = shrink_goal(d);
        let mission = Mission {
            name: format!("Planet {d} of {PLANETS}, {}", planet.name),
            says: if shrink > 0.0 { format!("Wipe out the slime: cut it to {:.0}% of its peak", 100.0 * shrink) } else { "Kill the slime's heart on its home crystal, or every last bit of slime".to_string() },
            map: map.clone(),
            tier: Some(tier.name().to_string()),
            goal: Goal::Shrink(shrink),
            verbs: Vec::new(),
            bonus: Some(Bonus::LoseAtMost(2)),
            // Free play's bite (2), not the missions' older 1: walking into
            // the slime is very deadly (owner, 11-12).
            // The clock counts up to the speed star (`par`); only a
            // stalemate meets the far limit (`limit`).
            rules: vec![("drain".into(), Rules::default().drain), ("cap".into(), limit(area_of(&planet)) * STRETCH), ("par".into(), par(md, area_of(&planet)) * STRETCH)],
            own: false,
        };
        // Walkers scale with the planet's area too (the level-gen thread,
        // 11-14 17:50Z: the same count covered a third of the smaller
        // planet 1 by 30 s, and would be a corner of a 6x core world).
        let area = area_of(&planet);
        let heat = if self.leg().heat { HEAT_WALKERS } else { 1.0 };
        let walkers = (1.0 + WALKERS_PER_DEPTH * (e - 1.0)) * area * size_roll(seed) * tier_size(tier) * heat;
        Level { depth: d, seed, ground: Ground::Planet { seed: node.seed, depth: md }, tier, walkers, mission, grow: head_start(area), curve: e }
    }

    /// The game's rules with the squad's and the buildings' upgrades.
    pub fn rules(&self, r: &mut Rules) {
        // What this planet's tier does to you (`does_to_you`); not in the
        // gun range, which plays no planet.
        if !self.range {
            // Mini-bosses (2026-11-16, `larder_world/bosses.rs`): their
            // chance rises with the planet (owner, 04:58Z: "Chance increases
            // with progression into the run"); a tier's own rules after.
            let e = self.curve();
            r.boss = crate::mould::larder_world::BossRules::for_curve(e);
            r.boss.every *= STRETCH;
            r.boss.every_sd *= STRETCH;
            r.boss.first *= STRETCH;
            // What every planet's slime does from its planet on
            // (`ENEMY_RULES`, `FIRST_PLANET`).
            for (k, v) in enemy_rules(e) {
                assert!(r.set(k, v), "ENEMY_RULES: no rule {k}");
            }
            // The planet's share scales the rate cap too, since the cap,
            // not the share, binds wherever the band is big (planet 1 run
            // seed 1: 16 soldiers lost and won at a quarter of the cap, 17
            // at the full cap, 43 and lost uncapped, 11 with no strikers).
            let at = |t: &[f32; 8]| curve_at(t, e);
            r.origin.size = at(&ORIGIN_SIZE);
            r.origin.armour = curve_at_u(&ORIGIN_ARMOUR, e);
            // The planet's twist (`twist`), the rest of its numbers the
            // twist's own.
            if let Some(t) = self.twist() {
                r.twist.kind = t.kind;
                r.twist.wind_angle = t.angle;
            }
            r.ceiling_grow = at(&CEILING_GROW);
            r.raid.gap_min = at(&RAID_GAP_MIN) * STRETCH;
            r.strikers.army_most = at(&STRIKER_ARMY);
            r.soldier_step = at(&SOLDIER_STEP);
            let d = crate::mould::larder_world::RaidRules::default();
            let (hf, he) = if self.leg().heat { (HEAT_RAID_FIRST, HEAT_RAID_EVERY) } else { (1.0, 1.0) };
            let (first, every) = (at(&RAID_FIRST) * STRETCH * hf, at(&RAID_EVERY) * STRETCH * he);
            r.raid.first = first;
            r.raid.first_sd = d.first_sd * first / d.first;
            r.raid.every = every;
            r.raid.every_sd = d.every_sd * every / d.every;
            r.drain = at(&DRAIN);
            r.raid.march_armour = curve_at_u(&RAID_ARMOUR, e);
            let k = at(&STRIKER_SHARE);
            r.strikers.share = k;
            r.strikers.rate = crate::mould::larder_world::StrikerRules::default().rate * k;
            for &(k, v) in does_to_you(self.level().tier).rules {
                assert!(r.set(k, v), "does_to_you: no rule {k}");
            }
            // Starfall and boss cores (2026-11-20, `larder_world/starfall.rs`;
            // the owner, 19:06Z: "I like starfall with boss cores"): shop
            // points the squad takes on the planet, on top of `reward`.
            r.star = crate::mould::larder_world::StarRules::for_depth(nearest(e));
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
        r.fire_hurt *= fireproof_share(self.level_of("fireproof"));
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
        // And so are sprayers and hunters (the shop's line said sprayers
        // since #796, but they were left out until the hunter came).
        b.sprayer_hp *= 1.0 + 0.3 * t;
        b.sprayer_range += 3.0 * t;
        b.sprayer_every *= 0.85f32.powf(t);
        b.hunter_hp *= 1.0 + 0.3 * t;
        b.hunter_range += 3.0 * t;
        b.hunter_every *= 0.85f32.powf(t);
    }

    /// The tools' prices with `tools+`.
    pub fn kit(&self, k: &mut KitRules) {
        let f = 0.8f32.powf(f32::from(self.level_of("tools+")));
        k.blast_price *= f;
        k.mine_price *= f;
        k.cord_price *= f;
        k.gas_price *= f;
        k.germ_price *= f;
        // The squad mode's device slot is free a throw (2026-11-21), so
        // the same upgrade shortens its cooldowns instead.
        k.blast_cd *= f;
        k.mine_cd *= f;
        k.cord_cd *= f;
        k.gas_cd *= f;
        k.germ_cd *= f;
        k.mind_cd *= f;
        // Each owned body's rolls (gun step E3): its reach, and its
        // cooldown as a share of its tool's.
        for b in &self.bodies {
            k.reaches[b.kind.tool()] = b.reach;
            match b.kind {
                BodyKind::Grenade => k.blast_cd *= b.cool,
                BodyKind::Mine => k.mine_cd *= b.cool,
                BodyKind::Line => k.cord_cd *= b.cool,
                BodyKind::Canister => k.gas_cd *= b.cool,
            }
        }
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
        if let Some(b) = self.loadouts.get(i).and_then(|l| l.body).and_then(|b| self.bodies.get(b)) {
            return b.rack();
        }
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

    /// Whether the run owns a device (the Tools group's unlocks).
    pub fn has_device(&self) -> bool {
        !self.bodies.is_empty() || DEVICES.iter().any(|k| self.has(k))
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
        // A loadout holds a gun or a body (gun step E3).
        self.loadouts[self.editing].body = None;
        self.refit();
    }

    /// Body `b` into the loadout shown, out of any other, and the shown
    /// one's gun back to the armory (a loadout holds a gun or a body); its
    /// cards refit to the body's rack. Not the squads' first loadout,
    /// which they land with. True if it changed.
    fn take_body(&mut self, b: usize) -> bool {
        if b >= self.bodies.len() || self.editing == 0 || self.loadouts[self.editing].body == Some(b) {
            return false;
        }
        for j in 0..self.loadouts.len() {
            if j != self.editing && self.loadouts[j].body == Some(b) {
                self.loadouts[j].body = None;
                self.refit_stored(j);
            }
        }
        self.loadouts[self.editing].gun = None;
        self.loadouts[self.editing].body = Some(b);
        self.refit();
        true
    }

    /// The bodies on autocast, as `Cmd::Auto`'s bits (`kit::TOOLS` order).
    pub fn autocast(&self) -> u8 {
        self.bodies.iter().filter(|b| b.auto).fold(0, |m, b| m | (1 << b.kind.tool()))
    }

    /// What the run's relics do to the gun cards' rules (gun step F):
    /// Carrion's marks (`RoundRules::carrion_secs`, 3 s) and Backdraft's
    /// fire on left ground (`backdraft_gap`, 8 dish cells).
    pub fn round_rules(&self, rr: &mut RoundRules) {
        if self.relics.contains(&Relic::Carrion) {
            rr.carrion_secs = 3.0;
        }
        if self.relics.contains(&Relic::Backdraft) {
            rr.backdraft_gap = 8.0;
        }
    }

    /// What the run's relics do to the slime's engine keys, set over the
    /// planet's and the tier's (the window's `start`): Fever's trail.
    pub fn engine_keys(&self) -> Vec<(&'static str, &'static str)> {
        let mut keys = Vec::new();
        if self.relics.contains(&Relic::Fever) {
            keys.push(("sick_trail", "2"));
        }
        keys
    }

    /// The body of `kind` owned, if any (its index in `bodies`).
    pub fn body_of(&self, kind: BodyKind) -> Option<usize> {
        self.bodies.iter().position(|b| b.kind == kind)
    }

    /// The cards the body of `kind` carries (gun step E3): those of the
    /// loadout holding it, else of the devices' default loadout
    /// (`device_loadout`, E2), as many as its rack takes; none without
    /// the body.
    pub fn body_cards(&self, kind: BodyKind) -> Vec<Card> {
        let Some(b) = self.body_of(kind) else { return Vec::new() };
        let l = self.loadouts.iter().position(|l| l.body == Some(b)).or(self.device_loadout);
        let Some(l) = l else { return Vec::new() };
        let rack = self.bodies[b].rack();
        // The bullet's flight cards are left on the gun: a device fires no
        // bullet, so Pierce, Split, Range, Rate and Seek on it did nothing
        // and took a slot and budget from a card that would (dead cards,
        // Direction 11-24 13:42Z). Trail cards stay: a device lays them
        // where it goes off (`Rounds::cast_device_at`).
        let mut cards: Vec<Card> = self.cards_of(l).into_iter().filter(|&c| !gun_only(c)).collect();
        while !cards.is_empty() && rounds::fit_gun(cards.clone(), &rack).is_err() {
            cards.pop();
        }
        cards
    }

    /// A body bought: in place of the one of its kind owned (keeping its
    /// index, so the loadout holding it holds the new one, its cards
    /// refitted), and its unlock owned, which the bots and the kit read
    /// (`has`).
    fn own_body(&mut self, body: Body) {
        let i = match self.body_of(body.kind) {
            Some(i) => {
                // The new body keeps the old one's autocast switch.
                self.bodies[i] = Body { auto: self.bodies[i].auto, ..body };
                i
            }
            None => {
                self.bodies.push(body);
                self.bodies.len() - 1
            }
        };
        if let Some(k) = ITEMS.iter().position(|it| it.key == body.kind.key()) {
            self.owned[k] = self.owned[k].max(1);
        }
        if let Some(j) = self.loadouts.iter().position(|l| l.body == Some(i)) {
            if j == self.editing {
                self.refit();
            } else {
                self.refit_stored(j);
            }
        }
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
            if self.held == Some(c) {
                self.held = None;
            }
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
                    let a = if set == Set::Standard { Ammo::standard(shell) } else { Ammo::roll_of(shell, set, mix(z ^ (16 * i + j) as u64), self.roll_depth()) };
                    self.offers.push(Card::Ammo(a));
                }
            }
            self.offers.extend(Card::ALL.into_iter().filter(|c| c.kind() != Kind::Payload));
            self.gun_offers = GunKind::ALL.iter().enumerate().map(|(k, &kind)| Gun::roll_kind(kind, mix(z ^ (0x6755_4E00 + k as u64)), self.roll_depth())).collect();
            self.body_offers = BodyKind::ALL.iter().enumerate().map(|(k, &kind)| Body::roll(kind, mix(z ^ (0xB0D1_0000 + k as u64)), self.roll_depth())).collect();
            return;
        }
        let n = OFFER + self.level_of("draw") as usize;
        let pool: Vec<Card> = Card::ALL.into_iter().filter(|&c| c.kind() != Kind::Payload && card_power(c).is_some_and(|p| power_from(p) <= self.roll_depth())).collect();
        let owned = self.all_cards();
        // A carrier (Trigger, Expire, Multicast) acts on a bullet card after
        // it, so none is sold while the run owns no bullet card (balance
        // review, 2026-11-20; restock already held back Shatter and
        // Detonate this way). Modifiers work on the plain bullet
        // (`rounds::casts`), so they stay in.
        //
        // And none while it owns only one (dead cards, Direction 11-24
        // 13:42Z): each needs a second cast after the one it sits on, and
        // a gun of one bullet card has none, so a Multicast fired nothing
        // beside it and a Trigger carried nothing. A plain bullet as the
        // missing cast was rejected: a bare plain cast casts nothing.
        let bullets = owned.iter().filter(|c| c.kind() == Kind::Payload).count();
        let has_bullet = bullets > 0;
        let pool: Vec<Card> = pool.into_iter().filter(|c| bullets >= 2 || c.kind() != Kind::Trigger || matches!(c, Card::Delay | Card::Thick)).collect();
        // Cards with nothing to act on yet are not sold (2026-11-22, exit
        // test 5: in the gun lab's arena on planet 1 each played the very
        // same game as no card). Thrift lowers the next card's cost, which
        // counts only once a bullet card fills the budget; Quiet takes a
        // blast's or flare's alarm away, which no slime answers before
        // `ALARM_FROM` (the drifter, sprawler and trunk ignore alarm,
        // `tiers::params`).
        let pool: Vec<Card> = pool.into_iter().filter(|&c| (c != Card::Thrift || has_bullet) && (c != Card::Quiet || self.roll_depth() >= ALARM_FROM)).collect();
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
        let mut shells: Vec<Shell> = shells_at(self.roll_depth()).into_iter().filter(|&s| match s {
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
            self.offers.push(Card::Ammo(Ammo::roll_of(shell, set, mix(z ^ 0xA330), self.roll_depth())));
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
        z = self.luck(&pool, &owned, bullets, wanted, z);
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
                (!kinds.is_empty()).then(|| Gun::roll_kind(kinds.remove((s % kinds.len() as u64) as usize), s, self.roll_depth()))
            })
            .filter(|g| !self.guns.contains(g))
            .collect();
        self.body_offers = self.body_rolls(z);
        // A Hard planet's rare card (star map, 2026-11-26), in the last
        // card's place so the shop's rows keep their count.
        if let Some(c) = self.rare_offer().filter(|c| !self.offers.contains(c)) {
            match self.offers.last_mut() {
                Some(last) => *last = c,
                None => self.offers.push(c),
            }
        }
    }

    /// **Luck's levers on the card offers** (gun step F, Direction 11-24
    /// 08:54Z; the gun-system plan's shop levers, first guesses): the cards
    /// on offer from `bullets` on, drawn from `pool`, change so that
    ///
    /// - **mix**: about a third (`n / 3`, at least one) are of a family the
    ///   run owns a card of (`family`), and at least one of a family it
    ///   owns none of, where the pool has them;
    /// - **pity**: once tier-3 cards are sold (`power_from(3)`), a shop
    ///   after `since_rare` shops with none shows one with chance
    ///   `since_rare` in 3, so surely by the fourth;
    /// - **hold**: the held card (`Pick::Hold`), if not on offer, takes the
    ///   last card's place.
    ///
    /// The card the planet asks for (`wanted`) keeps its slot. Returns the
    /// draw's seed moved on.
    fn luck(&mut self, pool: &[Card], owned: &[Card], bullets: usize, wanted: Option<Card>, mut z: u64) -> u64 {
        let fams: Vec<&str> = owned.iter().filter(|c| c.kind() != Kind::Payload).map(|&c| family(c)).collect();
        let ours = |c: &Card| fams.contains(&family(*c));
        // A card that may be swapped out: drawn, not the planet's.
        let free = |offers: &[Card], i: usize| i >= bullets && Some(offers[i]) != wanted;
        let n = self.offers.len().saturating_sub(bullets);
        // Swaps slot `at` for a card of `pool` that `fit` takes, new to
        // the offer and unowned if any is; false if none.
        let swap = |offers: &mut Vec<Card>, at: usize, fit: &dyn Fn(&Card) -> bool, z: &mut u64| {
            let mut c: Vec<Card> = pool.iter().copied().filter(|c| fit(c) && !offers.contains(c) && !owned.contains(c)).collect();
            if c.is_empty() {
                c = pool.iter().copied().filter(|c| fit(c) && !offers.contains(c)).collect();
            }
            if c.is_empty() {
                return false;
            }
            *z = mix(*z);
            offers[at] = c[(*z % c.len() as u64) as usize];
            true
        };
        let mut offers = std::mem::take(&mut self.offers);
        if !fams.is_empty() && n > 0 {
            let want_ours = (n / 3).max(1);
            // One of a family the run owns none of.
            if !offers[bullets..].iter().any(|c| !ours(c)) {
                if let Some(at) = (bullets..offers.len()).rev().find(|&i| free(&offers, i)) {
                    swap(&mut offers, at, &|c| !ours(c), &mut z);
                }
            }
            // About a third of the run's own families.
            while offers[bullets..].iter().filter(|c| ours(c)).count() < want_ours {
                let theirs = offers[bullets..].iter().filter(|c| !ours(c)).count();
                let Some(at) = (bullets..offers.len()).rev().find(|&i| free(&offers, i) && !ours(&offers[i]) && theirs > 1) else { break };
                if !swap(&mut offers, at, &|c| ours(c), &mut z) {
                    break;
                }
            }
        }
        // Pity: a tier-3 card after shops without one.
        if self.roll_depth() >= power_from(3) && self.since_rare > 0 && !offers[bullets..].iter().any(|c| c.tier() >= 3) {
            z = mix(z);
            if (z >> 8) % 3 < u64::from(self.since_rare) {
                // In place of a card of the same side of the mix, so the
                // mix holds where it can.
                let side = |c: &Card| ours(c);
                let tier3 = |c: &Card| c.tier() >= 3;
                let slots: Vec<usize> = (bullets..offers.len()).rev().filter(|&i| free(&offers, i)).collect();
                let done = slots.iter().any(|&at| {
                    let was = side(&offers[at]);
                    swap(&mut offers, at, &|c| tier3(c) && side(c) == was, &mut z)
                });
                if !done {
                    if let Some(&at) = slots.first() {
                        swap(&mut offers, at, &tier3, &mut z);
                    }
                }
            }
        }
        // The held card back, in the last card's place.
        if let Some(h) = self.held.filter(|h| !offers.contains(h)) {
            match (bullets..offers.len()).rev().find(|&i| free(&offers, i)) {
                Some(at) => offers[at] = h,
                None => offers.push(h),
            }
        }
        self.offers = offers;
        z
    }

    /// This shop's bodies (gun step E3, Direction 11-24, first guesses):
    /// on planet 1 a plain Grenade, the old Blasts unlock at its price,
    /// unless one is owned; from planet 2 two rolled bodies of two kinds,
    /// beside the guns (the guns on offer are as many as before).
    fn body_rolls(&self, z: u64) -> Vec<Body> {
        if self.roll_depth() <= 1 {
            return if self.body_of(BodyKind::Grenade).is_some() { Vec::new() } else { vec![Body::plain(BodyKind::Grenade)] };
        }
        let mut kinds = BodyKind::ALL.to_vec();
        (0..BODY_OFFER as u64)
            .map(|k| {
                let s = mix(z ^ (0xB0D1_0000 + k));
                Body::roll(kinds.remove((s % kinds.len() as u64) as usize), s, self.roll_depth())
            })
            .collect()
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
        self.split();
        self.points += self.pay(stars);
        self.stars.push(stars);
        if self.depth >= PLANETS {
            self.over = true;
            return;
        }
        self.depth += 1;
        self.take_only_way();
        self.next_shop();
    }

    /// A new planet's shop (`won`, `skip`): its draw, the paid rerolls
    /// back to the first price, and the pity count (`since_rare`) on by
    /// one when this shop shows no tier-3 card (gun step F).
    fn next_shop(&mut self) {
        self.paid_rerolls = 0;
        self.restock();
        // One unowned relic, drawn by the run's seed and the planet.
        let left: Vec<Relic> = Relic::ALL.into_iter().filter(|r| !self.relics.contains(r)).collect();
        let draw = mix(self.seed ^ u64::from(self.depth).wrapping_mul(0x9E37_79B9_7F4A_7C15)) as usize;
        // At the first step on each of the tuned planets `RELIC_DEPTHS`
        // names (twelve steps, 2026-11-26: steps 5, 9 and 12).
        let at = self.roll_depth();
        let first = self.depth <= 1 || nearest(curve_depth(self.depth - 1)) != at;
        self.relic_offer = (RELIC_DEPTHS.contains(&at) && first && !left.is_empty()).then(|| left[draw % left.len()]);
        let rare = self.offers.iter().any(|&c| c.kind() != Kind::Payload && c.tier() >= 3);
        self.since_rare = if rare || at < power_from(3) { 0 } else { self.since_rare + 1 };
    }

    /// What `Pick::Reroll` costs now: `REROLL_PRICE`, doubling each time
    /// in one shop.
    pub fn reroll_price(&self) -> u32 {
        REROLL_PRICE << self.paid_rerolls.min(8)
    }

    /// The dev skip (`Pick::DevSkip`): the current planet counted as won
    /// with no stars (the end card's count shows it was skipped), its
    /// average pay (`skip_pay`) added and the next planet's shop drawn.
    /// False on the home world, which can't be skipped.
    pub fn skip(&mut self) -> bool {
        if self.over || self.range || self.field || self.depth >= PLANETS {
            return false;
        }
        self.split();
        self.points += skip_pay(self.depth);
        self.stars.push(0);
        self.depth += 1;
        self.take_only_way();
        self.next_shop();
        true
    }

    /// The current planet's end in `splits`: those before it it lacks
    /// marked unknown, then the play time now.
    fn split(&mut self) {
        let at = self.depth.saturating_sub(1) as usize;
        self.splits.resize(at, -1.0);
        self.splits.push(self.played);
    }

    /// Real seconds played on planet `depth` (from 1), when both its end
    /// and the one before are known (`splits`).
    pub fn planet_secs(&self, depth: u32) -> Option<f32> {
        let i = (depth as usize).checked_sub(1)?;
        let end = *self.splits.get(i).filter(|t| **t >= 0.0)?;
        let start = if i == 0 { 0.0 } else { *self.splits.get(i - 1).filter(|t| **t >= 0.0)? };
        Some((end - start).max(0.0))
    }

    /// The run ended on the home world's defeat.
    pub fn victory(&self) -> bool {
        self.over && !self.beaten
    }

    /// The current level was lost: the run is over.
    pub fn lost(&mut self) {
        self.split();
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
        // The device unlocks are sold as bodies since gun step E3
        // (`body_offers`, below).
        for (i, it) in ITEMS.iter().enumerate().filter(|(_, it)| !self.range && !self.field && BodyKind::of_key(it.key).is_none()) {
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
        // Device bodies (gun step E3): those owned, each a click into the
        // loadout shown (not the squads' first), then those for sale, in
        // the Tools tab where the unlocks they replace were.
        for (i, b) in self.bodies.iter().enumerate() {
            let on = self.loadouts.get(self.editing).is_some_and(|l| l.body == Some(i));
            let holder = self.loadouts.iter().position(|l| l.body == Some(i));
            let (about, state) = match holder {
                _ if on => (format!("{}; in {shown}, which carries its cards.", b.kind.about()), "carried".to_string()),
                _ if self.editing == 0 => (format!("{}; show another loadout to give it cards (squads land with {shown}).", b.kind.about()), holder.map_or("owned".to_string(), |j| self.loadout_name(j))),
                Some(j) => (format!("{}; in {}: click to move it to {shown}, which gives up its gun.", b.kind.about(), self.loadout_name(j)), self.loadout_name(j)),
                None => (format!("{}; click to put it in {shown}, which gives up its gun, to carry its cards.", b.kind.about()), "owned".to_string()),
            };
            rows.push(Row { group: Group::Tools, face: Face::Item(b.kind.key()), on, name: title(&b.title()), about, price: String::new(), state, ready: !on && self.editing != 0, pick: Pick::CarryBody(i) });
            // Its autocast switch (gun step E4), free, in the field too.
            let (now, then) = if b.auto { ("on", "off") } else { ("off", "on") };
            let about = format!("{} on autocast goes off on its own whenever the squad's device slot is ready and thick slime is in reach, at least {:.0} cells from every soldier of the squad. Click to turn it {then}.", b.kind.name(), KitRules::default().auto_gap);
            rows.push(Row { group: Group::Tools, face: Face::Item(b.kind.key()), on: b.auto, name: format!("{} autocast: {now}", b.kind.name()), about, price: String::new(), state: format!("autocast {now}"), ready: true, pick: Pick::BodyAuto(i) });
        }
        // Relics (gun step F): those owned, and this shop's for sale.
        for rl in self.relics.iter() {
            rows.push(Row { group: Group::Relics, face: Face::Item(rl.key()), on: true, name: rl.name().into(), about: rl.about().into(), price: String::new(), state: "owned".into(), ready: false, pick: Pick::Relic });
        }
        if let Some(rl) = self.relic_offer.filter(|_| !self.field && !self.range) {
            rows.push(Row { group: Group::Relics, face: Face::Item(rl.key()), on: false, name: rl.name().into(), about: format!("A relic, rare: {}", rl.about()), price: rl.price().to_string(), state: "for sale".into(), ready: rl.price() <= self.points, pick: Pick::Relic });
        }
        for (i, b) in self.body_offers.iter().enumerate().filter(|_| !self.field) {
            let p = b.price();
            let (price, state) = if self.range { (String::new(), "free".into()) } else { (p.to_string(), "for sale".into()) };
            let swap = if self.body_of(b.kind).is_some() { format!(" Replaces your {}.", b.kind.name()) } else { String::new() };
            rows.push(Row { group: Group::Tools, face: Face::Item(b.kind.key()), on: false, name: title(&b.title()), about: format!("{}.{swap}", b.kind.about()), price, state, ready: self.range || p <= self.points, pick: Pick::Body(i) });
        }
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
            let held = self.held == Some(c) && !self.range;
            let (price, state) = if self.range { (String::new(), "free".into()) } else { (p.to_string(), if held { "held".into() } else { "for sale".into() }) };
            let about = if held { format!("{} Held: it comes back in every shop until you buy it.", c.about()) } else { c.about().into() };
            rows.push(Row { group: Group::Cards, face: Face::Card(c), on: false, name: card_title(c), about, price, state, ready: self.range || p <= self.points, pick: Pick::Offer(i) });
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
                if self.device_loadout == Some(i) && self.has_device() {
                    users.push("devices carry its cards".to_string());
                }
                if let Some((q, _)) = self.field_squad.filter(|f| f.1 == i) {
                    users.push(format!("Squad {} uses it", q + 1));
                }
                let users = if users.is_empty() { String::new() } else { format!(" ({})", users.join("; ")) };
                let n = self.cards_of(i).len();
                let about = format!("{gun}, {n} card{}{users}. Click to show it in the gun slots.", if n == 1 { "" } else { "s" });
                rows.push(bar(format!("{}: {gun}", i + 1), about, i == self.editing, i != self.editing, Pick::Edit(i)));
            }
            // Luck's shop levers (gun step F): a paid reroll of the card
            // offers, and the hold, which steps along the cards on offer.
            if !self.field && !self.offers.is_empty() {
                let price = self.reroll_price();
                let again = if self.paid_rerolls > 0 { " again" } else { "" };
                rows.push(Row { price: price.to_string(), ..bar(format!("Reroll cards{again} ({price})"), format!("New cards on offer for {price} points (the guns and bodies stay); the price doubles with each reroll in this shop."), false, price <= self.points, Pick::Reroll) });
                let at = self.held.and_then(|h| self.offers.iter().position(|&c| c == h));
                let next = at.map_or(self.offers.first(), |i| self.offers.get(i + 1));
                let name = match self.held {
                    Some(h) => format!("Hold: {}", card_title(h)),
                    None => "Hold a card".to_string(),
                };
                let about = match next {
                    Some(&c) => format!("Keep a card on offer for the next shops, free, until you buy it. Click to hold {} instead.", card_title(c)),
                    None => "Keep a card on offer for the next shops, free, until you buy it. Click to hold none.".to_string(),
                };
                rows.push(bar(name, about, self.held.is_some(), true, Pick::Hold));
            }
            if self.loadouts.len() < LOADOUTS_MAX {
                rows.push(bar("New loadout".into(), "An empty loadout with no gun: put a gun and cards in it, then give it to a squad or turrets.".into(), false, true, Pick::NewLoadout));
            }
            if self.has("turret") && self.turret_loadout != self.editing {
                rows.push(bar("Turrets use this".into(), format!("New turrets use {shown}; each turret can be changed in the game."), false, true, Pick::TurretsUse));
            }
            if self.has_device() && self.device_loadout != Some(self.editing) {
                rows.push(bar("Devices use this".into(), format!("Every device with no loadout of its own also casts {shown}'s cards (as many as its body holds) where it goes off, as a hit there would."), false, true, Pick::DevicesUse));
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
        // Behind Dev mode since 11-24 (polish review finding 8: a "Dev
        // mode" tab in every run's shop); Options' "Start on planet" still
        // skips with the same pay without it. Scott asked for "a dev way"
        // (11-22 21:36Z).
        if self.dev && !self.range && !self.field && !self.over && self.depth < PLANETS {
            let about = format!("Don't play planet {}: get the {} points an average win there pays, and shop for planet {}", self.depth, skip_pay(self.depth), self.depth + 1);
            rows.push(Row { group: Group::Dev, face: Face::Dev, on: false, name: format!("Skip planet {}", self.depth), about, price: String::new(), state: "dev".into(), ready: true, pick: Pick::DevSkip });
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
                // A device unlock (gone from the shop since gun step E3;
                // the bots still buy by key) is the plain body of it.
                if let Some(kind) = BodyKind::of_key(ITEMS[i].key) {
                    if self.body_of(kind).is_some() {
                        return false;
                    }
                    self.points -= price;
                    self.own_body(Body::plain(kind));
                    return true;
                }
                self.points -= price;
                self.owned[i] += 1;
                true
            }
            Pick::Body(i) => {
                let Some(&b) = self.body_offers.get(i) else { return false };
                let price = if self.range { 0 } else { b.price() };
                if price > self.points || self.field {
                    return false;
                }
                self.points -= price;
                self.body_offers.remove(i);
                self.own_body(b);
                true
            }
            Pick::CarryBody(i) => self.take_body(i),
            Pick::BodyAuto(i) => match self.bodies.get_mut(i) {
                Some(b) => {
                    b.auto = !b.auto;
                    true
                }
                None => false,
            },
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
                // The jump takes the Fair planets (`route`).
                self.route.resize(self.depth as usize, Leg::default());
                self.restock();
                true
            }
            Pick::DevSkip => self.skip(),
            Pick::Relic => {
                let Some(rl) = self.relic_offer.filter(|_| !self.field && !self.range) else { return false };
                if rl.price() > self.points {
                    return false;
                }
                self.points -= rl.price();
                self.relics.push(rl);
                self.relic_offer = None;
                true
            }
            Pick::Reroll => {
                let price = self.reroll_price();
                if self.range || self.field || price > self.points {
                    return false;
                }
                self.points -= price;
                self.paid_rerolls += 1;
                self.rerolls += 1;
                let (guns, bodies) = (std::mem::take(&mut self.gun_offers), std::mem::take(&mut self.body_offers));
                self.restock();
                (self.gun_offers, self.body_offers) = (guns, bodies);
                true
            }
            Pick::Hold => {
                if self.range || self.field || self.offers.is_empty() {
                    return false;
                }
                let at = self.held.and_then(|h| self.offers.iter().position(|&c| c == h));
                self.held = match at {
                    None => self.offers.first().copied(),
                    Some(i) => self.offers.get(i + 1).copied(),
                };
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
            Pick::DevicesUse => {
                let done = self.has_device() && self.device_loadout != Some(self.editing);
                if done {
                    self.device_loadout = Some(self.editing);
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

/// **The run's save file** (owner, 2026-11-22 04:40Z: "a run should always
/// be autosaving and saving the run between closing the program", after a
/// night's run was lost on closing the window; until then a run lived only
/// in the window's memory and there was no save at all, Milestone 2's "save
/// and resume" not yet built).
///
/// `key=value` lines, one per field, the cards by their text form
/// (`Card::word`, as replays carry them) and the guns as
/// `kind/depth/range/rate/spread/shots/budget/slots/grade` (`gun_word`;
/// Rust prints an `f32` so it reads back exactly). Upgrades are saved by
/// their key, not their place in `ITEMS`. A line it does not know is
/// skipped and a missing one keeps `Run::new`'s value, and a card or an
/// upgrade a later build renamed or removed is dropped with the rest kept,
/// so a save outlives the builds `play.cmd` pulls between sittings. Only
/// the head (`SAVE_HEAD`), the seed and the depth are required.
///
/// What it holds is the run in the shop, or as it landed on the planet
/// being played: a planet's own state (the slime, the squads, the
/// buildings) lives on the GPU and is not saved, so a run closed mid-planet
/// lands on the same planet (the same seed, the same map) fresh.
pub const SAVE_HEAD: &str = "larder-world run save 1";

/// A gun's text form in a save (see `SAVE_HEAD`).
pub fn gun_word(g: &Gun) -> String {
    format!("{}/{}/{}/{}/{}/{}/{}/{}/{}", g.kind.key(), g.depth, g.range, g.rate, g.spread, g.shots, g.budget, g.slots, g.grade)
}

/// A gun from its text form (`gun_word`).
pub fn gun_from_word(s: &str) -> Option<Gun> {
    let w: Vec<&str> = s.split('/').collect();
    if w.len() != 9 {
        return None;
    }
    let f = |i: usize| w[i].parse::<f32>().ok().filter(|v| v.is_finite());
    Some(Gun { kind: GunKind::parse(w[0])?, depth: w[1].parse().ok()?, range: f(2)?, rate: f(3)?, spread: f(4)?, shots: w[5].parse().ok()?, budget: w[6].parse().ok()?, slots: w[7].parse().ok()?, grade: f(8)? })
}

impl Run {
    /// The run as its save file's text (`SAVE_HEAD`). The loadout panel's
    /// state (`field`, `field_squad`) is the window's while it is open and
    /// is not saved.
    pub fn save_text(&self) -> String {
        let words = |cs: &[Card]| cs.iter().map(|c| c.word()).collect::<Vec<_>>().join(" ");
        let guns = |gs: &[Gun]| gs.iter().map(gun_word).collect::<Vec<_>>().join(" ");
        let flag = |b: bool| if b { "1" } else { "0" };
        let mut s = format!("{SAVE_HEAD}\n");
        let mut put = |k: &str, v: String| s.push_str(&format!("{k}={v}\n"));
        put("seed", self.seed.to_string());
        put("depth", self.depth.to_string());
        put("points", self.points.to_string());
        put("owned", ITEMS.iter().zip(self.owned).filter(|(_, l)| *l > 0).map(|(it, l)| format!("{}:{l}", it.key)).collect::<Vec<_>>().join(" "));
        put("cards", self.cards.iter().map(|(c, on)| format!("{}{}", if *on { "+" } else { "-" }, c.word())).collect::<Vec<_>>().join(" "));
        put("offers", words(&self.offers));
        put("guns", guns(&self.guns));
        put("gun_offers", guns(&self.gun_offers));
        put("bodies", self.bodies.iter().map(Body::word).collect::<Vec<_>>().join(" "));
        put("body_offers", self.body_offers.iter().map(Body::word).collect::<Vec<_>>().join(" "));
        // A loadout: its gun's index or `-` (`-/B` for body B's, gun step
        // E3; an older build reads that as no gun), then its cards, `,`
        // between.
        let head = |l: &Loadout| match l.body {
            Some(b) => format!("-/{b}"),
            None => l.gun.map_or("-".into(), |g| g.to_string()),
        };
        put("loadouts", self.loadouts.iter().map(|l| std::iter::once(head(l)).chain(l.cards.iter().map(|c| c.word())).collect::<Vec<_>>().join(",")).collect::<Vec<_>>().join(" "));
        put("editing", self.editing.to_string());
        put("turret_loadout", self.turret_loadout.to_string());
        put("device_loadout", self.device_loadout.map_or("-".into(), |i| i.to_string()));
        put("stars", self.stars.iter().map(|s| s.to_string()).collect::<Vec<_>>().join(" "));
        put("over", flag(self.over).into());
        put("beaten", flag(self.beaten).into());
        put("dev", flag(self.dev).into());
        put("rerolls", self.rerolls.to_string());
        put("held", self.held.map_or(String::new(), |c| c.word()));
        put("paid_rerolls", self.paid_rerolls.to_string());
        put("since_rare", self.since_rare.to_string());
        put("relics", self.relics.iter().map(|r| r.key()).collect::<Vec<_>>().join(" "));
        put("relic_offer", self.relic_offer.map_or("", |r| r.key()).to_string());
        put("played", format!("{:.1}", self.played));
        put("splits", self.splits.iter().map(|t| format!("{t:.1}")).collect::<Vec<_>>().join(" "));
        // The star map's route (2026-11-26): each step's node, `h` after
        // it where the heat was on.
        put("route", self.route.iter().map(|l| format!("{}{}", l.k, if l.heat { "h" } else { "" })).collect::<Vec<_>>().join(" "));
        s
    }

    /// A run from its save file's text (`save_text`), or why not.
    pub fn from_save(text: &str) -> Result<Run, String> {
        let mut lines = text.lines();
        if lines.next().map(str::trim) != Some(SAVE_HEAD) {
            return Err(format!("not a run save (its first line is not \"{SAVE_HEAD}\")"));
        }
        let kv: Vec<(&str, &str)> = lines.filter_map(|l| l.split_once('=')).map(|(k, v)| (k.trim(), v.trim())).collect();
        let get = |k: &str| kv.iter().rev().find(|(n, _)| *n == k).map(|(_, v)| *v);
        let seed: u64 = get("seed").and_then(|v| v.parse().ok()).ok_or("no seed")?;
        let mut r = Run::new(seed);
        r.depth = get("depth").and_then(|v| v.parse().ok()).filter(|d| (1..=PLANETS).contains(d)).ok_or("no depth, or one past the last planet")?;
        let num = |k: &str| get(k).and_then(|v| v.parse::<u64>().ok());
        let flag = |k: &str| get(k).is_some_and(|v| v == "1");
        let list = |k: &str| get(k).map(|v| v.split_whitespace().collect::<Vec<_>>()).unwrap_or_default();
        r.points = num("points").map_or(0, |v| v.min(u32::MAX as u64) as u32);
        if get("owned").is_some() {
            r.owned = [0; ITEMS.len()];
            for w in list("owned") {
                let Some((k, l)) = w.split_once(':') else { continue };
                if let (Some(i), Ok(l)) = (ITEMS.iter().position(|it| it.key == k), l.parse::<u8>()) {
                    r.owned[i] = l.min(ITEMS[i].max);
                }
            }
        }
        r.cards = list("cards").into_iter().filter_map(|w| Some((Card::from_word(w.get(1..)?)?, w.starts_with('+')))).collect();
        r.offers = list("offers").into_iter().filter_map(Card::from_word).collect();
        let guns: Vec<Gun> = list("guns").into_iter().filter_map(gun_from_word).collect();
        if !guns.is_empty() {
            r.guns = guns;
        }
        r.gun_offers = list("gun_offers").into_iter().filter_map(gun_from_word).collect();
        for b in list("bodies").into_iter().filter_map(Body::from_word) {
            if r.body_of(b.kind).is_none() {
                r.bodies.push(b);
            }
        }
        r.body_offers = list("body_offers").into_iter().filter_map(Body::from_word).collect();
        // A save from before bodies (gun step E3, Direction 11-24) that
        // owned a tool's unlock gets that tool's plain body, so the run
        // keeps its tools.
        for kind in BodyKind::ALL {
            if r.body_of(kind).is_none() && r.has(kind.key()) {
                r.bodies.push(Body::plain(kind));
            }
        }
        let bodies = r.bodies.len();
        let loadouts: Vec<Loadout> = list("loadouts")
            .into_iter()
            .map(|l| {
                let mut parts = l.split(',');
                let head = parts.next().unwrap_or("-");
                let (g, b) = head.split_once('/').unwrap_or((head, ""));
                let gun = g.parse::<usize>().ok().filter(|&g| g < r.guns.len());
                let body = b.parse::<usize>().ok().filter(|&b| b < bodies);
                Loadout { gun: if body.is_some() { None } else { gun }, body, cards: parts.filter_map(Card::from_word).collect() }
            })
            .take(LOADOUTS_MAX)
            .collect();
        if !loadouts.is_empty() {
            r.loadouts = loadouts;
        }
        let last = r.loadouts.len() - 1;
        r.editing = num("editing").map_or(0, |v| (v as usize).min(last));
        r.turret_loadout = num("turret_loadout").map_or(r.turret_loadout, |v| v as usize).min(last);
        r.device_loadout = num("device_loadout").map(|v| (v as usize).min(last));
        r.stars = list("stars").into_iter().filter_map(|v| v.parse().ok()).collect();
        r.over = flag("over");
        r.beaten = flag("beaten");
        r.dev = flag("dev");
        r.rerolls = num("rerolls").unwrap_or(0);
        r.held = get("held").and_then(Card::from_word);
        r.paid_rerolls = num("paid_rerolls").map_or(0, |v| v as u32);
        r.since_rare = num("since_rare").map_or(0, |v| v as u32);
        r.relics = list("relics").into_iter().filter_map(Relic::of_key).collect();
        r.relic_offer = get("relic_offer").and_then(Relic::of_key);
        // A save from before 11-24 has none: its time so far is unknown.
        r.played = get("played").and_then(|v| v.parse::<f32>().ok()).filter(|v| v.is_finite() && *v >= 0.0).unwrap_or(0.0);
        // (One unreadable reads as unknown, so the rest keep their planets.)
        r.splits = list("splits").into_iter().map(|v| v.parse::<f32>().ok().filter(|t| t.is_finite()).unwrap_or(-1.0)).collect();
        // A save from before the star map: node 0 (Fair) at every step so
        // far. A leg past the map's ways is dropped with the rest after
        // it, so the player picks again there.
        r.route = match get("route") {
            None => vec![Leg::default(); r.depth as usize],
            Some(_) => {
                let map = r.star_map();
                let mut legs: Vec<Leg> = Vec::new();
                for w in list("route") {
                    let heat = w.ends_with('h');
                    let Ok(k) = w.trim_end_matches('h').parse::<u8>() else { break };
                    let step = legs.len() as u32 + 1;
                    let from = legs.last().map(|l| l.k);
                    if step > r.depth || !map.choices(step, from).iter().any(|n| n.k == k) {
                        break;
                    }
                    legs.push(Leg { k, heat });
                }
                legs
            }
        };
        r.take_only_way();
        Ok(r)
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

    /// **Planet twists** (`PlanetTwist`): none on planet 1, never two
    /// planets in a row, about one in three from planet 2 (over 2,000
    /// runs), both kinds and every wind direction rolled, the same seed the
    /// same rolls, and the run's rules carry them.
    #[test]
    fn a_run_rolls_a_twist_on_about_one_planet_in_three() {
        let (mut rolled, mut planets, mut cold, mut wind) = (0, 0, 0, 0);
        let mut ways = std::collections::HashSet::new();
        for seed in 0..2_000u64 {
            let mut last = None;
            for d in 1..=PLANETS {
                let t = twist_at(seed, d);
                assert_eq!(t, twist_at(seed, d), "the same seed rolls the same");
                if d == 1 {
                    assert!(t.is_none(), "none on planet 1");
                    continue;
                }
                assert!(!(t.is_some() && last.is_some()), "seed {seed}: twists on planets {} and {d}", d - 1);
                planets += 1;
                if let Some(t) = t {
                    rolled += 1;
                    match t.kind {
                        Twist::Cold => cold += 1,
                        Twist::Wind => {
                            wind += 1;
                            ways.insert(t.angle as u32);
                        }
                        Twist::None => panic!("a rolled twist is one"),
                    }
                }
                last = t;
            }
        }
        let share = rolled as f32 / planets as f32;
        eprintln!("twists: {rolled} of {planets} planets ({share:.3}), {cold} cold, {wind} wind, {} ways", ways.len());
        assert!((0.28..0.38).contains(&share), "about one in three: {share}");
        assert!(cold > rolled / 3 && wind > rolled / 3, "both kinds: {cold} cold, {wind} wind");
        assert_eq!(ways.len(), 8, "every wind way");
        // The run's rules carry the roll; the shop's range rolls none.
        let mut run = Run::new(0);
        let seed = (0..200).find(|&s| twist_at(s, 2).is_some_and(|t| t.kind == Twist::Wind)).unwrap();
        run.seed = seed;
        run.depth = 2;
        let mut r = Rules::default();
        run.rules(&mut r);
        assert_eq!(r.twist.kind, Twist::Wind);
        assert_eq!(r.twist.wind_angle, twist_at(seed, 2).unwrap().angle);
        let t = run.twist().unwrap();
        assert!(t.card().starts_with("Wind from the ") && t.card().contains("all planet"), "{}", t.card());
        assert!(Run::range(seed).twist().is_none());
        let cold = PlanetTwist { kind: Twist::Cold, angle: 0.0 };
        assert_eq!(cold.card(), "Cold snap at 0:20: for a minute the slime crawls in beads. Push now.");
        let west = PlanetTwist { kind: Twist::Wind, angle: 0.0 };
        assert_eq!(west.card(), "Wind from the west: the slime leans and creeps east all planet. Stand upwind, or let it blow onto your guns.");
    }

    /// Each late threat waits for its planet (`FIRST_PLANET`, the owner
    /// 11-20 23:32Z): planet 1 has fruiting and the heartbeat but no
    /// strikers, waves, late lunge or last-stand heart; each joins on its
    /// planet and stays on deeper; every scheduled rule is one of
    /// `ENEMY_RULES` and every rule plays by planet 4.
    /// **The length dials change nothing at eight planets** (2026-11-24,
    /// Direction's ask): with `PLANETS` 8 and `STRETCH` 1 every table reads
    /// its own value on its own planet, the mini-bosses are `for_depth`'s,
    /// and each late threat starts on its `FIRST_PLANET`. With sixteen
    /// planets the same curve spreads: planet 1 and 16 read the first and
    /// last values, and the places between rise in order.
    #[test]
    fn the_length_dials_change_nothing_at_eight_planets() {
        // Twelve steps since the star map (2026-11-26); the curve's whole
        // places are still the eight tuned planets exactly.
        assert_eq!((PLANETS, STRETCH), (12, 1.0), "the dials as set");
        assert_eq!((curve_depth(1), curve_depth(PLANETS)), (1.0, 8.0));
        let f32s: [&[f32; 8]; 11] = [&SURGE_FOOD_SHARE, &ORIGIN_SIZE, &STRIKER_SHARE, &CEILING_GROW, &RAID_GAP_MIN, &STRIKER_ARMY, &SOLDIER_STEP, &RAID_FIRST, &RAID_EVERY, &DRAIN, &ROAM_SHARE];
        let u32s: [&[u32; 8]; 5] = [&LONGEST_WAIT, &RAID_ARMOUR, &SURGE_DASH, &ROAM, &ORIGIN_ARMOUR];
        for d in 1..=8 {
            assert_eq!(curve_depth_of(d, 8), d as f32);
            for t in f32s {
                assert_eq!(curve_at(t, d as f32).to_bits(), t[d as usize - 1].to_bits(), "planet {d}: {t:?}");
            }
            for t in u32s {
                assert_eq!(curve_at_u(t, d as f32), t[d as usize - 1], "planet {d}: {t:?}");
            }
            assert_eq!(crate::mould::larder_world::BossRules::for_curve(d as f32), crate::mould::larder_world::BossRules::for_depth(d));
            for (k, v) in enemy_rules(d as f32) {
                assert_eq!(v != 0.0, d >= first_planet(k) && ENEMY_RULES.iter().any(|&(n, w)| n == k && w != 0.0), "{k} on planet {d}");
            }
        }
        // Sixteen planets: the same span, in order.
        let e: Vec<f32> = (1..=16).map(|d| curve_depth_of(d, 16)).collect();
        assert_eq!((e[0], e[15]), (1.0, 8.0));
        assert!(e.windows(2).all(|w| w[1] > w[0]), "{e:?}");
        let mid = curve_at(&ORIGIN_SIZE, e[9]);
        assert!(ORIGIN_SIZE[4] < mid && mid < ORIGIN_SIZE[5], "planet 10 of 16 between planets 5 and 6 of 8: {mid}");
    }

    #[test]
    fn late_threats_wait_for_their_planet() {
        for &(k, d) in FIRST_PLANET {
            assert!(ENEMY_RULES.iter().any(|&(n, _)| n == k), "FIRST_PLANET names {k}, not one of ENEMY_RULES");
            assert!((1..=4).contains(&d), "{k} first on planet {d}");
        }
        let rules = |d: u32| {
            let mut r = Rules::default();
            Run::tuned(1, d).rules(&mut r);
            r
        };
        // Wherever the strikers can be on, their share and cap are not 0.
        assert!(STRIKER_SHARE.iter().all(|&k| k > 0.0), "a 0 share leaves strikers on and doing nothing");
        // The slime's room to regrow reaches every planet's rules, at least
        // its landing (under 1 a slime would start over its ceiling).
        for d in 1..=8 {
            assert_eq!(rules(d).ceiling_grow, CEILING_GROW[d as usize - 1], "planet {d}'s ceiling_grow");
        }
        assert!(CEILING_GROW.iter().all(|&g| g >= 1.0), "a growth under 1 starves the slime at landing");
        // Planet 1 keeps its numbers; planets 2 on are harder (11-22).
        let (one, two) = (rules(1), rules(2));
        assert!(one.raid.gap_min == 45.0 && one.strikers.army_most == 1.0 && one.soldier_step == 0.0, "planet 1 as before");
        assert!(two.raid.gap_min < 45.0 && two.strikers.army_most > 1.0 && two.soldier_step > 0.0, "planet 2 on: closer raids, more strikers on a big army, a climbing price");
        assert!((one.raid.first, one.raid.every, one.drain) == (240.0, 150.0, 2.0), "planet 1's raid clock and bite as before");
        assert!(one.raid.march_armour == 0 && two.raid.march_armour > 1, "planet 2 on: a raid's marchers take more than one blow");
        assert!(two.raid.first < one.raid.first && two.raid.every < one.raid.every && two.drain > one.drain, "planet 2 on: the first raid sooner, the next ones closer, a harder bite");
        let p1 = rules(1);
        assert!(p1.heart.on && p1.unit_paths, "planet 1 keeps the heartbeat and charges round walls");
        assert!(!p1.fruit.on && !p1.strikers.on && !p1.wave.on && p1.late_at == 0.0 && !p1.stand.on, "planet 1 has none of the late threats");
        assert_eq!(p1.boss.every, 0.0, "nor mini-bosses");
        assert!(p1.star.on, "planet 1 keeps starfall and cores");
        let p2 = rules(2);
        assert!(p2.fruit.on && p2.strikers.on && p2.strikers.share > 0.0 && !p2.wave.on && p2.late_at == 0.0 && !p2.stand.on);
        let p3 = rules(3);
        assert!(p3.strikers.on && p3.wave.on && p3.late_at > 0.0 && !p3.stand.on);
        for d in 4..=8 {
            let r = rules(d);
            assert!(r.strikers.on && r.wave.on && r.late_at > 0.0 && r.stand.on && r.fruit.on && r.heart.on, "planet {d}");
        }
    }

    /// Deeper, the slime surges no less often, lunges no shorter and
    /// sends specks no more often; planet 1 is gentler than the sandbox's shape C
    /// and the core world fiercer, and every key is one the engine takes.
    #[test]
    fn the_slime_gets_fiercer_deeper() {
        let get = |d: u32, k: &str| enemy_keys(curve_depth(d)).into_iter().find(|(n, _)| *n == k).map(|(_, v)| v.parse::<f32>().unwrap()).unwrap();
        for d in 1..=10 {
            assert!(get(d + 1, "surge") <= get(d, "surge") && get(d + 1, "surge_dash") >= get(d, "surge_dash"), "planet {d}");
            // Specks no more often deeper (owner 11-16: "Specks more often
            // early game"): chances no closer together, no larger share.
            assert!(get(d + 1, "roam") >= get(d, "roam") && get(d + 1, "roam_share") <= get(d, "roam_share"), "planet {d}");
            assert_eq!(surge_secs(curve_depth(d)), get(d, "surge") / 30.0);
            let mut p = crate::mould::big::BigParams::today();
            for (k, v) in enemy_keys(curve_depth(d)) {
                // The lure's pull is the GPU engine's alone (the owner's
                // card, 11-17 19:21Z: no CPU twin): the engine takes it,
                // and `BigParams` has no field for it.
                if k == "lure_pull" || k == "tether" || k == "crystal_keep" || k == "crater_shut" || k == "boss_flinch" {
                    let mut s = crate::gpu::Settings::big_dish();
                    crate::gpu::set(&mut s, k, &v).unwrap();
                    assert!(s.lure_pull == LURE_PULL || s.tether == TETHER || s.crystal_keep || s.crater_shut > 0.0 || s.boss_flinch > 0.0, "planet {d}: {k}");
                    continue;
                }
                crate::mould::tiers::set_key(&mut p, k, &v).unwrap();
            }
            // No bronze (owner 11-16: "Remove the bronze"), specks on.
            assert!(p.surge_every > 0 && p.surge_dash > 0 && p.roam > 0 && p.roam_share > 0.0);
            assert!(surge_food(curve_depth(d + 1), 1.0) <= surge_food(curve_depth(d), 1.0) && surge_food(curve_depth(d), 2.0) == 2.0 * surge_food(curve_depth(d), 1.0), "planet {d}");
        }
        let c_food = crate::mould::larder_world::SURGE_FOOD;
        assert!(surge_food(curve_depth(1), 1.0) > c_food && surge_food(curve_depth(PLANETS), 1.0) < c_food);
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
        for d in 1..=PLANETS {
            r.depth = d;
            let l = r.level();
            // A cell's walkers grow deeper, so a smaller planet (a breather)
            // can have fewer in all; the tier's own factor (`tier_size`, the
            // Sprawler's 0.45) is a tuning number kept apart from that rule.
            let dense = l.walkers / area_of(&r.planet()) / size_roll(l.seed) / tier_size(l.tier);
            assert!((0.8..=1.25).contains(&size_roll(l.seed)));
            assert!(dense > walkers, "more walkers a cell deeper");
            assert!(shrink_goal(d) <= goals);
            (walkers, goals) = (dense, shrink_goal(d));
            assert_eq!(l, Run { depth: d, ..Run::new(7) }.level());
            let n = r.node();
            assert_eq!(l.ground, Ground::Planet { seed: n.seed, depth: r.map_depth() });
            assert_eq!(levelgen::spec_planet(&l.mission.map, 0), Some(Ok(levelgen::planet(n.seed, r.map_depth()))), "the window reads the planet back from the mission's map");
        }
        r.depth = 1;
        assert_eq!(r.level().tier, Tier::Drifter);
        // Planet 2's Sprawler at 0.45 of its size (the planet 2 look, 11-18;
        // 0.45 with its threads, 11-24).
        r.depth = 2;
        let (l, area) = (r.level(), area_of(&r.planet()));
        assert_eq!(l.tier, Tier::Sprawler);
        assert!((l.walkers - 0.45 * (1.0 + 0.2 * (curve_depth(2) - 1.0)) * area * size_roll(l.seed)).abs() < 1e-4);
        // The level generator's core world is the run's last planet, and
        // only the last.
        r.depth = PLANETS;
        assert_eq!(r.map_depth(), levelgen::CORE_DEPTH);
        assert!((1..PLANETS).all(|d| Run { depth: d, ..Run::new(7) }.map_depth() < levelgen::CORE_DEPTH));
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

    /// Exit test 5 (2026-11-22): a card with nothing to act on is not
    /// sold. No Quiet or Flare before `ALARM_FROM`, and no Thrift while the
    /// run owns no bullet card.
    #[test]
    fn the_shop_holds_back_cards_with_nothing_to_act_on() {
        assert!(!shells_at(ALARM_FROM - 1).contains(&Shell::Flare));
        assert!(shells_at(ALARM_FROM).contains(&Shell::Flare));
        let mut quiet = 0;
        for seed in 1..=20 {
            let mut r = Run::new(seed);
            for d in 1..=ALARM_FROM + 1 {
                r.depth = d;
                for k in 0..5 {
                    r.rerolls = k;
                    r.restock();
                    assert!(!r.offers.contains(&Card::Thrift), "seed {seed} depth {d}: a Thrift with no bullet owned");
                    assert!(d >= ALARM_FROM || !r.offers.contains(&Card::Quiet), "seed {seed} depth {d}: a Quiet before the alarm");
                    quiet += usize::from(r.offers.contains(&Card::Quiet));
                }
            }
        }
        assert!(quiet > 0, "Quiet is sold from ALARM_FROM");
    }

    /// Balance review, 2026-11-20: no carrier is sold while the run owns
    /// fewer than two bullet cards (one since 11-24: with one, a Multicast
    /// or Trigger has no second cast to fire), and an owned card can come
    /// back in the last slot.
    #[test]
    fn the_shop_holds_back_carriers_and_sells_owned_cards_again() {
        let carrier = |c: &Card| matches!(c, Card::Trigger | Card::Expire | Card::Multicast);
        let (mut again, mut carriers) = (0, 0);
        for seed in 1..=20 {
            let mut r = Run::new(seed);
            for d in 1..=4 {
                r.set_tuned(d);
                for k in 0..5 {
                    r.rerolls = k;
                    r.restock();
                    assert!(!r.offers.iter().any(carrier), "seed {seed} depth {d}: a carrier with no bullet owned: {:?}", r.offers);
                }
            }
            r.cards = vec![(Card::Blast, true), (Card::Wide, false), (Card::Pierce, false)];
            for d in 1..=4 {
                r.set_tuned(d);
                for k in 0..5 {
                    r.rerolls = k;
                    r.restock();
                    assert!(!r.offers.iter().any(carrier), "seed {seed} depth {d}: a carrier with one bullet owned: {:?}", r.offers);
                }
            }
            r.cards = vec![(Card::Blast, true), (Card::Gas, false), (Card::Wide, false), (Card::Pierce, false)];
            for d in 1..=4 {
                r.set_tuned(d);
                for k in 0..5 {
                    r.rerolls = k;
                    r.restock();
                    carriers += r.offers.iter().filter(|c| carrier(c)).count();
                    again += r.offers.iter().filter(|c| matches!(c, Card::Wide | Card::Pierce)).count();
                }
            }
        }
        assert!(carriers > 0, "carriers sold once two bullets are owned");
        assert!(again > 0, "owned cards come back into the draw");
    }

    /// Dead cards, 11-24: a device carries no card only a bullet uses
    /// (`gun_only`), so its slots go to cards it casts.
    #[test]
    fn a_device_leaves_the_bullets_flight_cards_on_the_gun() {
        let mut r = Run::new(1);
        // A rack of 2 slots and budget 4: Range and Pierce (2 each) filled
        // it before, and the Blast (3) after them was popped off.
        r.bodies.push(Body { slots: 2, budget: 4, ..Body::roll(BodyKind::Grenade, 7, 3) });
        r.cards = vec![(Card::Range, true), (Card::Pierce, true), (Card::Blast, true)];
        r.device_loadout = Some(0);
        let d = r.body_cards(BodyKind::Grenade);
        assert!(d.contains(&Card::Blast) && !d.iter().any(|&c| gun_only(c)), "{d:?} from {:?}, rack {:?}", r.cards_of(0), r.bodies[0].rack());
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
                // (Twelve steps: the Sprawler on steps 2 and 3, the Trunk
                // on 4.)
                while r.level().tier != tier {
                    r.won(2);
                }
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
        // Later planets ask for their counter (11-24), each a shell their
        // shop sells; the Drifter (no shop before it) and the Grudge ask
        // for none.
        for (depth, tier) in (1..).zip(LADDER) {
            match bring_shell(tier) {
                Some(s) => assert!(depth > 1 && shells_at(depth).contains(&s), "{tier:?}: {s:?} not sold at planet {depth}"),
                None => assert!(matches!(tier, Tier::Drifter | Tier::Grudge), "{tier:?}"),
            }
        }
    }

    #[test]
    fn the_shop_sells_what_the_points_buy_and_no_more() {
        let mut r = Run::new(1);
        assert!(r.has("walls") && !r.has("turret"));
        assert!(r.shop().iter().all(|row| !row.ready || matches!(row.pick, Pick::Card(_) | Pick::Edit(_) | Pick::NewLoadout | Pick::DevSkip)), "no points, nothing to buy");
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
                    r.set_tuned(d);
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
    fn a_skipped_planet_pays_an_average_win_and_the_home_world_cannot_be_skipped() {
        // The pay rises from planet 2 on and stays near a 2-star win plus drops.
        let pays: Vec<u32> = (1..=PLANETS).map(skip_pay).collect();
        assert_eq!(pays, [360, 510, 510, 470, 530, 530, 580, 580, 630, 690, 690, 0]);
        let mut r = Run::new(3);
        r.restock();
        assert!(!r.shop().iter().any(|row| row.pick == Pick::DevSkip), "not without dev mode (11-24)");
        r.dev = true;
        assert!(r.shop().iter().any(|row| row.pick == Pick::DevSkip), "in a dev run's shop");
        let mut paid = 0;
        while r.pick(Pick::DevSkip) {
            paid += skip_pay(r.depth - 1);
        }
        assert_eq!((r.depth, r.points, r.stars.len(), r.over), (PLANETS, paid, PLANETS as usize - 1, false));
        assert!(!r.shop().iter().any(|row| row.pick == Pick::DevSkip));
        assert!(!r.offers.is_empty(), "the home world's shop is drawn");
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
            let run = Run::tuned(1, d as u32 + 1);
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
                assert!(set.is_ok() || *t == Tier::Trunk || THREAD_KEYS.contains(&k) || TOUGH_KEYS.contains(&k) || DREAM_KEYS.contains(&k), "{} key {k}: {set:?}", t.name());
            }
            let shy = if *t == Tier::Sprawler { 50.0 } else { 0.0 };
            assert_eq!((s.fire_alarm, p.fire_alarm), (shy, shy), "{}", t.name());
            // The Trunk sets, walls soldiers out, walls the crystals it
            // holds and is fireproof while fed; the Swarmer makes a last
            // stand; no other rung does either.
            let trunk = *t == Tier::Trunk;
            assert_eq!((s.set_scar > 0.0, s.crust_rise > 0.0, s.fire_fed, r.set_block > 0.0, r.fort > 0.0), (trunk, trunk, trunk, trunk, trunk), "{}", t.name());
            assert_eq!(r.raid.last_stand, matches!(*t, Tier::Swarmer | Tier::Dozer), "{}", t.name());
            // Every planet: the far smell, and charges that go round walls.
            assert!(r.unit_paths, "{}", t.name());
            assert!(enemy_keys(curve_depth(d as u32 + 1)).iter().any(|(k, v)| *k == "far_smell" && v == "1"));
            // Every planet: a lure pulls (the owner's card, 11-18 03:03Z).
            assert!(enemy_keys(curve_depth(d as u32 + 1)).iter().any(|(k, v)| *k == "lure_pull" && v.parse::<f32>().unwrap() > 0.0));
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

    /// **Fireproof** (owner, 2026-11-22 07:40Z: "upgrade fire resistance
    /// ... or even immunity"): each level halves fire's harm and the third
    /// is immunity, at 100, 200 and 300 points.
    #[test]
    fn fireproof_halves_fire_then_makes_the_squad_immune() {
        let mut r = Run::new(1);
        r.points = 10_000;
        let k = ITEMS.iter().position(|i| i.key == "fireproof").unwrap();
        let base = Rules::default().fire_hurt;
        let hurt = |r: &Run| {
            let mut rules = Rules::default();
            r.rules(&mut rules);
            rules.fire_hurt
        };
        assert_eq!(hurt(&r), base);
        for (level, share, price) in [(1, 0.5, 100), (2, 0.25, 200), (3, 0.0, 300)] {
            let before = r.points;
            assert!(r.pick(Pick::Item(k)));
            assert_eq!((r.level_of("fireproof"), before - r.points), (level, price));
            assert_eq!(hurt(&r), base * share, "level {level}");
        }
        assert!(!r.pick(Pick::Item(k)), "three levels at most");
    }

    /// **Loadouts** (owner, 2026-11-19 19:22Z): a gun and its cards; a
    /// gun or a card is in one loadout at a time; a loadout that loses its
    /// gun fits the turrets' rack and gives back what no longer fits; the
    /// field panel sells nothing and gives the panel's squad a loadout.
    /// **A run saves and loads whole** (owner 2026-11-22 04:40Z, a run
    /// lost on closing the window): a run that has shopped, won planets
    /// and built loadouts reads back from its save text as it was, every
    /// card kind and a rolled bullet and gun with it; a save from an older
    /// or newer build keeps what it can read.
    #[test]
    fn a_run_saves_and_loads_whole() {
        let mut r = Run::new(1_763_786_000);
        r.points = 5_000;
        r.restock();
        for i in 0..ITEMS.len() {
            r.pick(Pick::Item(i));
        }
        r.pick(Pick::Offer(0));
        r.pick(Pick::Gun(0));
        r.cards.extend(Card::ALL.into_iter().map(|c| (c, false)));
        r.cards.push((Card::Ammo(Ammo::roll(7, 3)), false));
        r.pick(Pick::NewLoadout);
        let gas = r.cards.iter().position(|c| c.0 == Card::Gas).unwrap();
        r.pick(Pick::Card(gas));
        r.pick(Pick::Edit(0));
        r.won(3);
        r.won(1);
        r.dev = true;
        r.pick(Pick::DevReroll);
        r.owned[1] = 2;
        r.guns.push(Gun::roll(3, 2));
        r.gun_offers.push(GunKind::Long.average_at(3));
        r.loadouts.push(Loadout { gun: Some(r.guns.len() - 1), body: None, cards: vec![Card::Blast, Card::Ammo(Ammo::roll(9, 2))] });
        r.own_body(Body::roll(BodyKind::Canister, 5, 2));
        r.loadouts.push(Loadout { gun: None, body: r.body_of(BodyKind::Canister), cards: vec![Card::Torch] });
        r.body_offers.push(Body::roll(BodyKind::Line, 6, 2));
        r.offers.push(Card::Pierce);
        r.played = 1234.5;
        (r.held, r.paid_rerolls, r.since_rare) = (Some(Card::Pierce), 2, 1);
        assert!(r.owned.iter().any(|&l| l > 1) && r.guns.len() > 1 && r.loadouts.len() > 2 && !r.offers.is_empty());
        let text = r.save_text();
        assert_eq!(Run::from_save(&text), Ok(r.clone()), "{text}");
        // The loadout panel's state is the window's, not the save's.
        let mut f = r.clone();
        (f.field, f.field_squad) = (true, Some((1, 2)));
        assert_eq!(Run::from_save(&f.save_text()), Ok(r.clone()));
        // A renamed card and upgrade, a line from a later build and a lost
        // line: the rest loads.
        let odd = text.replace("cards=", "cards=+no-such-card ").replace("owned=", "owned=no_such_item:3 ") + "a_later_field=7\n";
        let odd: String = odd.lines().filter(|l| !l.starts_with("rerolls=")).map(|l| format!("{l}\n")).collect();
        let back = Run::from_save(&odd).unwrap();
        assert_eq!((back.cards.clone(), back.owned, back.loadouts.clone(), back.rerolls), (r.cards.clone(), r.owned, r.loadouts.clone(), 0));
        // Not a save, or no planet to play: an error, never a panic.
        assert!(Run::from_save("").is_err() && Run::from_save("hello\nseed=1").is_err());
        assert!(Run::from_save(&format!("{SAVE_HEAD}\nseed=1\ndepth=99\n")).is_err());
        let bare = Run::from_save(&format!("{SAVE_HEAD}\nseed=4\ndepth=2\n")).unwrap();
        assert_eq!((bare.seed, bare.depth, bare.guns.clone(), bare.loadouts.len()), (4, 2, vec![first_gun()], 2));
    }

    /// Each planet's time (the Runs page, 11-24): won, skipped and lost
    /// planets each end a split; a save without them reads as unknown, and
    /// a planet after an unknown one is unknown too.
    #[test]
    fn a_run_keeps_each_planets_time() {
        let mut r = Run::new(3);
        r.played = 300.0;
        r.won(2);
        r.played = 300.0;
        assert!(r.skip());
        r.played = 750.5;
        r.lost();
        assert_eq!(r.splits, vec![300.0, 300.0, 750.5]);
        assert_eq!((r.planet_secs(1), r.planet_secs(2), r.planet_secs(3), r.planet_secs(4)), (Some(300.0), Some(0.0), Some(450.5), None));
        let back = Run::from_save(&r.save_text()).unwrap();
        assert_eq!(back.splits, r.splits);
        // From an older save on planet 3: its first two unknown.
        let mut old = Run::from_save(&format!("{SAVE_HEAD}\nseed=4\ndepth=3\nplayed=900\n")).unwrap();
        old.won(1);
        assert_eq!(old.splits, vec![-1.0, -1.0, 900.0]);
        assert_eq!((old.planet_secs(2), old.planet_secs(3)), (None, None));
    }

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

    /// **Devices carry a loadout's cards** (gun step E, 2026-11-24): no
    /// "Devices use this" before a body is owned; once one is, the pick
    /// gives the bodies the shown loadout's cards, as many as each body's
    /// rack takes (the plain Grenade's 2 slots and budget of 4), and the
    /// run's save keeps it.
    #[test]
    fn devices_carry_the_loadout_they_use() {
        let mut r = Run::new(1);
        assert!(r.edit(1));
        r.cards = vec![(Card::Torch, true), (Card::Wide, true), (Card::Gas, true)];
        assert!(!r.shop().iter().any(|w| w.pick == Pick::DevicesUse) && !r.pick(Pick::DevicesUse), "no device owned");
        assert!(r.body_cards(BodyKind::Grenade).is_empty());
        r.own_body(Body::plain(BodyKind::Grenade));
        assert!(r.shop().iter().any(|w| w.pick == Pick::DevicesUse));
        assert!(r.pick(Pick::DevicesUse) && r.device_loadout == Some(1));
        assert!(!r.shop().iter().any(|w| w.pick == Pick::DevicesUse), "loadout 2 is already the devices'");
        let d = r.body_cards(BodyKind::Grenade);
        assert!(!d.is_empty() && d.len() <= 2 && rounds::fit_gun(d.clone(), &Body::plain(BodyKind::Grenade).rack()).is_ok(), "{d:?}");
        assert_eq!(d[0], Card::Torch);
        assert!(r.body_cards(BodyKind::Mine).is_empty(), "no mine owned");
        assert!(r.shop().iter().any(|w| w.pick == Pick::Edit(1) && w.about.contains("devices carry its cards")));
        assert_eq!(Run::from_save(&r.save_text()).map(|f| (f.device_loadout, f.body_cards(BodyKind::Grenade))), Ok((Some(1), d)));
    }

    /// **Luck's shop levers** (gun step F, Direction 11-24 08:54Z): the
    /// hold keeps a card on offer shop after shop until it is bought; a
    /// reroll redraws the cards for 25, then 50, keeping the guns and
    /// bodies, and costs 25 again in the next shop; every shop's cards mix
    /// the run's own families and one it owns none of; after shops with no
    /// tier-3 card one is on offer by the fourth; Trigger and Expire are
    /// tier 3.
    #[test]
    fn the_shops_luck_levers_hold_reroll_mix_and_pity() {
        assert_eq!((Card::Trigger.tier(), Card::Expire.tier(), Card::Multicast.tier()), (3, 3, 3));
        // Hold.
        let mut r = Run::new(1);
        r.restock();
        assert!(r.shop().iter().any(|w| w.pick == Pick::Hold && w.name == "Hold a card"));
        assert!(r.pick(Pick::Hold) && r.held == Some(r.offers[0]));
        let h = r.held.unwrap();
        assert!(r.pick(Pick::Hold) && r.held == Some(r.offers[1]), "the hold steps along");
        r.held = Some(h);
        assert!(r.shop().iter().any(|w| w.pick == Pick::Offer(0) && w.state == "held"));
        for _ in 0..3 {
            r.won(3);
            assert!(r.offers.contains(&h), "held into planet {}'s shop: {:?}", r.depth, r.offers);
        }
        r.points = 10_000;
        let at = r.offers.iter().position(|&c| c == h).unwrap();
        assert!(r.pick(Pick::Offer(at)) && r.held.is_none(), "bought, the hold is off");
        // Reroll.
        let (guns, bodies, offers, pts) = (r.gun_offers.clone(), r.body_offers.clone(), r.offers.clone(), r.points);
        assert_eq!(r.reroll_price(), REROLL_PRICE);
        assert!(r.pick(Pick::Reroll) && r.points == pts - 25 && r.reroll_price() == 50);
        assert!(r.pick(Pick::Reroll) && r.points == pts - 75 && r.reroll_price() == 100);
        assert_eq!((r.gun_offers.clone(), r.body_offers.clone()), (guns, bodies), "the guns and bodies stay");
        assert_ne!(r.offers, offers, "new cards");
        r.won(3);
        assert_eq!(r.reroll_price(), 25, "each shop starts at the first price");
        r.points = 10;
        assert!(!r.pick(Pick::Reroll), "too dear");
        // Mix: with Wide owned (shape), planets 2 and 3's cards hold one
        // shape card and one of another family, over 40 seeds and rerolls.
        let (mut ours, mut theirs, mut shops) = (0, 0, 0);
        for seed in 1..=40 {
            let mut r = Run::new(seed);
            r.cards = vec![(Card::Wide, true), (Card::Blast, true)];
            for d in 2..=3 {
                r.depth = d;
                for k in 0..3 {
                    r.rerolls = k;
                    r.restock();
                    let cards: Vec<Card> = r.offers.iter().copied().filter(|c| c.kind() != Kind::Payload).collect();
                    shops += 1;
                    ours += usize::from(cards.iter().any(|&c| family(c) == "shape"));
                    theirs += usize::from(cards.iter().any(|&c| family(c) != "shape"));
                }
            }
        }
        assert_eq!((ours, theirs), (shops, shops), "every shop mixes the run's family and another");
        // Pity: three shops with no tier-3 card, the fourth shows one.
        for seed in 1..=40 {
            let mut r = Run::new(seed);
            r.cards = vec![(Card::Blast, true)];
            r.depth = 5;
            r.since_rare = 3;
            r.restock();
            assert!(r.offers.iter().any(|c| c.kind() != Kind::Payload && c.tier() == 3), "seed {seed}: {:?}", r.offers);
        }
        let mut rare = 0;
        for seed in 1..=40 {
            let mut r = Run::new(seed);
            r.cards = vec![(Card::Blast, true)];
            r.depth = 5;
            r.restock();
            rare += usize::from(r.offers.iter().any(|c| c.kind() != Kind::Payload && c.tier() == 3));
        }
        eprintln!("planet 5's first shop shows a tier-3 card in {rare} of 40 seeds without pity");
    }

    /// **Relics** (gun step F): Carrion is for sale after planets 3, 5
    /// and 7 only, once; bought, it is owned for the run, sets
    /// `carrion_secs` and stays in the save.
    #[test]
    fn carrion_is_a_relic_sold_after_planets_3_5_and_7() {
        let mut r = Run::new(1);
        let mut sold = Vec::new();
        for _ in 1..PLANETS {
            r.won(3);
            if r.relic_offer.is_some() {
                sold.push(r.depth);
            }
        }
        // Twelve steps (2026-11-26): the first step on tuned planets 4, 6
        // and 8.
        assert_eq!(sold, vec![5, 9, 12], "after steps 4, 8 and 11");
        let mut r = Run::new(1);
        for _ in 0..3 {
            r.won(3);
        }
        r.relic_offer = Some(Relic::Carrion);
        let row = r.shop().into_iter().find(|w| w.pick == Pick::Relic).expect("a relic row");
        assert_eq!((row.group, row.name.as_str(), row.price.as_str()), (Group::Relics, "Carrion", "300"));
        r.points = 299;
        assert!(!r.pick(Pick::Relic));
        let mut rr = RoundRules::default();
        r.round_rules(&mut rr);
        assert_eq!(rr.carrion_secs, 0.0);
        r.points = 300;
        assert!(r.pick(Pick::Relic) && r.relics == vec![Relic::Carrion] && r.points == 0 && r.relic_offer.is_none());
        r.round_rules(&mut rr);
        assert_eq!(rr.carrion_secs, 3.0);
        assert_eq!(Run::from_save(&r.save_text()).map(|f| f.relics), Ok(vec![Relic::Carrion]));
        // The three shops each offer an unowned relic, drawn by the seed:
        // across seeds every relic comes first somewhere, and one bought
        // is never offered again.
        let mut first = std::collections::HashSet::new();
        for seed in 0..30 {
            let mut r = Run::new(seed);
            let mut seen = Vec::new();
            for _ in 1..PLANETS {
                r.won(3);
                if let Some(o) = r.relic_offer {
                    assert!(!r.relics.contains(&o));
                    seen.push(o);
                    r.relics.push(o);
                }
            }
            first.insert(seen[0]);
            assert_eq!(seen.len(), 3);
        }
        assert_eq!(first.len(), 3, "each relic is some seed's first");
        // Fever sets the slime's sick trail; Backdraft the left ground's fire.
        let mut r = Run::new(1);
        assert!(r.engine_keys().is_empty());
        r.relics = vec![Relic::Fever, Relic::Backdraft];
        assert_eq!(r.engine_keys(), vec![("sick_trail", "2")]);
        let mut rr = RoundRules::default();
        r.round_rules(&mut rr);
        assert_eq!((rr.backdraft_gap, rr.carrion_secs), (8.0, 0.0));
        // All three owned: the shop of step 9 (a relic's) sells none.
        r.relics.push(Relic::Carrion);
        while r.depth < 9 {
            r.won(3);
        }
        assert_eq!((r.depth, r.relic_offer), (9, None), "every relic owned: none more for sale");
    }

    /// **Bodies** (gun step E3, Direction 11-24): planet 1's shop sells
    /// a plain Grenade at the old Blasts price and no Tools unlocks; planet
    /// 2's sells two rolled bodies of two kinds and as many guns as before;
    /// a body bought replaces the one of its kind, in the loadout holding
    /// it; a loadout holding a body carries its cards on that body alone,
    /// fitted to its rack; and a save that owned an old unlock loads with
    /// its plain body.
    #[test]
    fn bodies_are_bought_carried_and_loaded_from_old_saves() {
        let mut r = Run::new(1);
        r.restock();
        let rows = r.shop();
        assert!(!rows.iter().any(|w| matches!(w.pick, Pick::Item(i) if DEVICES.contains(&ITEMS[i].key))), "the old unlocks are gone");
        let g = rows.iter().find(|w| w.pick == Pick::Body(0)).expect("a Grenade on sale");
        assert!(g.name.contains("Grenade") && g.price == "150", "{} {}", g.name, g.price);
        r.points = 10_000;
        assert!(r.pick(Pick::Body(0)) && r.has("blast") && r.bodies == vec![Body::plain(BodyKind::Grenade)]);
        assert!(r.body_offers.is_empty() && !r.shop().iter().any(|w| matches!(w.pick, Pick::Body(_))));
        r.won(3);
        assert_eq!(r.gun_offers.len(), GUN_OFFER, "the guns on offer are as many as before");
        assert_eq!(r.body_offers.len(), BODY_OFFER);
        assert_ne!(r.body_offers[0].kind, r.body_offers[1].kind);
        assert!(r.body_offers.iter().all(|b| b.rolled && b.depth == 2));
        // A rolled Grenade in place of the plain one, held by loadout 2.
        r.pick(Pick::NewLoadout);
        assert!(r.edit(1));
        assert!(r.pick(Pick::CarryBody(0)) && r.loadouts[1].body == Some(0) && r.loadouts[1].gun.is_none());
        assert!(!r.edit(0) || !r.pick(Pick::CarryBody(0)), "loadout 1 holds no body");
        assert!(r.edit(1));
        let big = Body { slots: 3, budget: 6, ..Body::roll(BodyKind::Grenade, 3, 2) };
        r.body_offers = vec![big];
        assert!(r.pick(Pick::Body(0)) && r.bodies == vec![big] && r.loadouts[1].body == Some(0));
        r.cards = vec![(Card::Torch, true), (Card::Wide, true), (Card::Gas, true), (Card::Bomb, true)];
        let d = r.body_cards(BodyKind::Grenade);
        assert!(!d.is_empty() && rounds::fit_gun(d.clone(), &big.rack()).is_ok(), "{d:?}");
        assert_eq!(r.rack_of(1).budget, 6, "the shown loadout's rack is the body's");
        // Autocast (gun step E4): off as bought, a free switch, kept
        // through a buy of the same kind and the save.
        assert_eq!(r.autocast(), 0);
        let sw = r.shop().iter().position(|w| w.pick == Pick::BodyAuto(0)).expect("an autocast switch");
        assert!(r.shop()[sw].name.ends_with("off") && r.pick(Pick::BodyAuto(0)) && r.autocast() == 1);
        r.body_offers = vec![Body::roll(BodyKind::Grenade, 4, 2)];
        assert!(r.pick(Pick::Body(0)) && r.autocast() == 1, "a new Grenade keeps the switch");
        assert_eq!(Run::from_save(&r.save_text()).map(|f| f.autocast()), Ok(1));
        r.field = true;
        assert!(r.pick(Pick::BodyAuto(0)) && r.autocast() == 0, "the field panel switches it too");
        r.field = false;
        // An old save: the Blasts and Sleep gas unlocks, no bodies.
        let mut old = Run::new(2);
        for k in ["blast", "gas"] {
            old.owned[ITEMS.iter().position(|i| i.key == k).unwrap()] = 1;
        }
        let text: String = old.save_text().lines().filter(|l| !l.starts_with("bodies") && !l.starts_with("body_offers")).map(|l| format!("{l}\n")).collect();
        let f = Run::from_save(&text).unwrap();
        assert_eq!(f.bodies, vec![Body::plain(BodyKind::Grenade), Body::plain(BodyKind::Canister)]);
        let mut k = KitRules::default();
        f.kit(&mut k);
        assert_eq!((k.reach_of(crate::mould::kit::Tool::Blast((0.0, 0.0))), k.blast_cd), (24.0, KitRules::default().blast_cd), "the plain body plays as the old tool");
        let mut k = KitRules::default();
        r.kit(&mut k);
        assert_eq!((k.reach_of(crate::mould::kit::Tool::Blast((0.0, 0.0))), k.blast_cd), (r.bodies[0].reach, KitRules::default().blast_cd * r.bodies[0].cool), "a rolled body's reach and cooldown");
    }

}
