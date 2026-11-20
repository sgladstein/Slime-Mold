//! **The GPU engine**: the big dish's step (`crate::mould::big`, the `nose`
//! scaling rule) on the GPU, and its drawing into a dish-sized frame buffer.
//! Moved here from `src/bin/mouldgpu.rs` on 2026-11-11 so that a game (the
//! Larder world) and the sandbox share one engine; the sandbox's panel,
//! presets and window stay in the binary, and so do the twin tests (they
//! use its presets; CI runs them with `MOULDGPU_TEST_GPU=1`).
//!
//! **One tick is the timing test's four compute passes** (`examples/gputime.rs`,
//! whose shader this one extends; that file stays as it was measured on the
//! GTX 1070): `sense` (a thread a walker: sensors, turn, aim, claim with
//! `atomicMin` of a keyed draw), `walk` (the winner moves and counts a
//! deposit), `trail` and `scent` (a thread a cell: 3x3 mean over agar,
//! kept). A frame is `ticks` ticks (2, the game's `STEPS_PER_FRAME`, by
//! default), then the drawing passes. The shader is `engine.wgsl`; what it
//! adds to the timing test's step is in `src/bin/mouldgpu.rs`'s header.
//!
//! wgpu comes through `pixels::wgpu`, the GPU `pixels` already opens for a
//! window, so no new dependency. Not built for wasm32 (the browser crate
//! builds the library).

use std::sync::atomic::{AtomicBool, Ordering};
use std::sync::Arc;

use crate::mould::big::{BigDish, BigParams, SpeciesSet, LIFE_DEAD};
use crate::mould::map::Map;
use crate::mould::squad;
use crate::mould::STEPS_PER_SECOND;
use pixels::wgpu;

pub mod larder_world;

/// The engine's shader (it was the sandbox's `SHADER` constant).
pub const SHADER: &str = include_str!("engine.wgsl");

/// The CPU's `keyed` stream key for a use (`mould::keyed`'s first line).
pub fn stream(seed: u64, use_: u32) -> u32 {
    (seed as u32) ^ ((seed >> 32) as u32) ^ use_.wrapping_mul(0x9E37_79B9)
}

// Draw uses, as in big.rs (106 is the CPU's respawn).
pub const DRAW_TURN: u32 = 101;
pub const DRAW_BLOCKED: u32 = 102;
pub const CLAIM_MOVE: u32 = 103;
pub const DRAW_JUMP: u32 = 106;
pub const DRAW_PULL: u32 = 107;
/// The sensing floor's and wander's draws (big.rs `DRAW_WANDER`).
pub const DRAW_WANDER: u32 = 108;
/// The outside rules' draws: the chance turn's, each walker's size and
/// explorer stage, and the blocked walker's reset (big.rs).
pub const DRAW_CHANCE: u32 = 109;
pub const DRAW_SIZE: u32 = 110;
pub const DRAW_KEEP: u32 = 111;
/// Growth's draws (big.rs `DRAW_STARVE`, `DRAW_BIRTH`, `CLAIM_BIRTH`).
pub const DRAW_STARVE: u32 = 401;
pub const DRAW_BIRTH: u32 = 402;
pub const CLAIM_BIRTH: u32 = 403;
/// A cut-off walker's withering (big.rs `DRAW_WITHER`).
pub const DRAW_WITHER: u32 = 404;
/// The specks' draw (big.rs `DRAW_ROAM`; condensing's split until 11-17).
pub const DRAW_ROAM: u32 = 405;
/// A raid's draw (big.rs `DRAW_RAID`).
pub const DRAW_RAID: u32 = 406;

/// The life cycle's (big.rs `DRAW_FRUIT` and on).
pub const DRAW_FRUIT: u32 = 114;
pub const DRAW_SLEEP: u32 = 115;
pub const CLAIM_LAND: u32 = 116;
pub const DRAW_SPORE_H: u32 = 117;
pub const DRAW_SLEEP_DIE: u32 = 118;

/// Bytes in `Params` (308 words).
pub const PARAMS_BYTES: usize = 1232;
/// Where one body's row sits in `Params` (2026-11-12, after the soil row:
/// `energy_flow`, `hunger_scent`, `hunger_at`; then the far smell,
/// `far_smell`, S2, in the word that held `fed_full` and then
/// `fire_weight`, both deleted 11-17).
pub const BODY_AT: usize = 848;
/// Where the crater's row sits in `Params` (2026-11-12, after one body's:
/// `scorch_keep`, `scorch_full`, the steps a shot's crater lasts, and
/// whether the crater counts down at all). `set_squad` writes its last two
/// words alone, as it does the squad's.
pub const CRATER_AT: usize = 864;
/// Where the lone step's row sat in `Params` (2026-11-12, after the
/// crater's). **Two words are free** (written 0): two of the lone step's
/// three (`lone_step`, `lone_floor`, `lone_r`; f32 880 and 884, u32 888)
/// since the GPU's half of that rule was deleted on 2026-11-17 (the Enemy
/// lane's switch cleanup; `Reports/dead-ends.md`), and 892, `fed_crowd`'s
/// until the same day. The CPU engine keeps `BigParams::lone_step`. Word
/// 888 is the fort's flag since 2026-11-18 (`Engine::forted`), and 892
/// the tether (`Settings::tether`) since the same day.
pub const LONE_AT: usize = 880;
/// Where the fling row sits in `Params` (11-14, after the lone step's:
/// `fling_deposit`, then three spare words).
pub const FLING_AT: usize = 896;
/// The breakaway row (`BigParams::roam`, 11-14).
pub const ROAM_AT: usize = 912;
/// The ground row (new ground kinds, 11-14; Params grew from 928 to 944):
/// deep water's step (`BigParams::deep_step`), deep water as a wall
/// (`Settings::deep_wall`, 11-20), then two words kept for the next ground
/// kinds.
pub const GROUND_AT: usize = 928;
/// The surge row (`BigParams::surge_every`, 11-15, the slime-enemy thread;
/// Params grew from 944 to 960): the steps between surges, the creep's and
/// the lunge's steps in one word (low and high half), the step the game's
/// last surge began on (`surge_at`), the lunge's haste.
pub const SURGE_AT: usize = 944;
/// The spray row (gun look, 2026-11-15; Params grew from 960 to 976, after
/// the surge row): a shot's spray (`squad::Frame::spray`): its longest
/// flight in fine steps (0 off), its first speed and the flight's
/// reciprocal; then the fine steps a tick (the low byte), so `fly` can give
/// the view each flung walker's flight a tick, and above it the share of
/// the kills that spray (of 65,536).
pub const SPRAY_AT: usize = 960;
/// The raid row (`BigParams::raid_at`, 2026-11-15, the defenses thread;
/// Params grew from 976 to 1008, after the spray row): the raid's first
/// step, its target (a fine cell, x | y << 16), its reach squared in whole
/// fine cells, the stream of its draw; then its share, its lean's and
/// run's steps (low and high half), its haste and a spare word.
pub const RAID_AT: usize = 976;
/// The lump rows (`BigParams::lumps`, 2026-11-16, the mini-boss thread;
/// Params grew from 1008 to 1152, after the raid row): whether any lump is
/// on and three spare words, then each lump's aim, pull, leash
/// (`Lump::leash_k`), haste, armour, bite and ring, and a spare word.
pub const LUMP_AT: usize = 1008;
/// The Mind row (the gun cards' Mind family, 2026-11-16; Params grew from
/// 1152 to 1168, after the lump rows): whether any walker has had a Mind
/// status (`Engine::minds`; `sense` reads none before), a spinning
/// walker's curl a step (`BigParams::spin_curl`), and two spare words.
pub const MIND_AT: usize = 1152;
/// The scar row (the Grudge's habituation on the GPU, 2026-11-17, the
/// simulation-use plan's step 0; Params grew from 1168 to 1184, after the
/// Mind row): `scar_keep` (0 with the scar off: no alarm field, or the key
/// 0), `scar_gain`, `scar_weight`, and `fire_dry` | `fire_fed` << 1 (its
/// spare word until the plan's C1, 11-17). The field itself is
/// binding 47's second block, kept for it since 2026-11-12.
pub const SCAR_AT: usize = 1168;
/// The crust row (the simulation-use plan's E1a, 2026-11-17; Params grew
/// from 1184 to 1200): `crust_rise` (0 with the crust off), `crust_keep`,
/// `crust_trail`, and the word in `news` where the crust's block starts
/// (`Engine::crust_at` / 4; the field rides in `news` because every
/// binding a stage has is taken).
pub const CRUST_AT: usize = 1184;
/// The crust's effects row (the plan's E1b and C2, 2026-11-17; Params grew
/// from 1200 to 1216): `crust_weight`, `crust_fast` and `crust_fuel` (each 0
/// on a dish without the crust), and `char_barren` (0 without plants).
pub const CRUST_FX_AT: usize = 1200;
/// The setting row (the plan's E2a, 2026-11-17; Params grew from 1216 to
/// 1232): `set_scar` (0 off, and 0 on a dish without the crust or the
/// scar), `set_wake`, `set_crust` and `set_armour`. Each slot's set word is
/// a block of `news` right after the crust's (`Engine::set_of`).
pub const SET_AT: usize = 1216;

/// The raid row's words (`RAID_AT`) from the settings: the reach squared
/// in whole fine cells, as the CPU's `raid_of` counts it.
fn raid_row(s: &Settings, scale: f32, field: bool) -> [u32; 8] {
    let sm = crate::mould::big::SURGE_MAX;
    let r = (s.raid_r.max(0.0) * scale).min(60_000.0);
    let on = s.raid_at > 0 && s.raid_r > 0.0 && s.raid_share > 0.0;
    [
        if on { s.raid_at } else { 0 },
        s.raid_to,
        (r * r).floor() as u32,
        stream(s.seed, DRAW_RAID),
        s.raid_share.to_bits(),
        s.raid_lean.min(sm) | s.raid_len.min(sm) << 16,
        s.raid_haste.max(crate::mould::big::PANIC_CREEP_MIN).to_bits(),
        field as u32,
    ]
}
/// Where the strike's row sits in `Params` (`Engine::strike` writes it
/// alone), before the soil row (2026-11-12).
pub const STRIKE_AT: usize = 800;
/// Where the squad's four words sit in `Params` (`set_squad` writes them
/// alone). Fixed: the look's hits row and the species block (2026-11-11)
/// come after them.
pub const SQUAD_AT: usize = 416;

/// The Larder World's switches in `Params::world` (engine.wgsl).
pub const WORLD_CRYSTALS: u32 = 1;
pub const WORLD_WETS: u32 = 2;
pub const WORLD_WALLS: u32 = 4;
pub const WORLD_TERRAIN: u32 = 8;
/// The dish has soil (`BigDish::soil`, WM3): the trail, life and breed
/// passes read a cell's kind from its ground word.
pub const WORLD_SOIL: u32 = 16;
/// The gardeners' world (`own_mould`; the shader's `GARDEN`): species 3 is the player's.
pub const WORLD_GARDEN: u32 = 32;
/// The dish has plants (`plant_words`; the shader's `PLANTS`), for the
/// units' fire reads.
pub const WORLD_PLANTS: u32 = 64;
/// A cell's `ground` bits (engine.wgsl): its crystal's pile (index + 1) in
/// the low byte, then wet, wall, a wall that lets scent through, rock, and
/// the soil kind in bits 12-15 (the map's G byte is bits 8-15, so its high
/// nibble lands there as it is). Agreed with the graphics thread
/// (2026-11-11): mould-made ground takes bits 16-23, height 24-31.
pub const GROUND_PILE: u32 = 0xFF;
pub const GROUND_WET: u32 = 0x100;
pub const GROUND_WALL: u32 = 0x200;
pub const GROUND_SCENT_WALL: u32 = 0x400;
pub const GROUND_ROCK: u32 = 0x800;
pub const GROUND_SOIL: u32 = 0xF000;
pub const GROUND_SOIL_SHIFT: u32 = 12;
/// Rounds of `evict` a wall runs between checks, and at most: walkers that
/// lost a free cell to another try the next nearest, until none finds one.
/// (Four rounds in all left 24 of 178 covered walkers inside on the twin
/// test's packed home crystal, which the CPU had all moved.)
pub const EVICT_ROUNDS: usize = 4;
pub const EVICT_MAX: usize = 64;

/// Species the sandbox draws (a colour each, as `style=species`).
pub const MAX_SPECIES: u8 = 4;

/// Piles the GPU tracks (the dish's seven and food dropped with F); F does
/// nothing once they are all used.
pub const MAX_PILES: usize = 64;
const _: () = assert!(MAX_PILES == crate::mould::big::PANIC_PILES, "a panicking walker looks over the same piles on both engines");

/// The wall counters' first bytes (`Engine::wstat`): bites, broken cells,
/// the eviction's moves, the struck, the living walkers, the own mould's,
/// the cap's two counts, then the condensed strikers' counts (words 8-10,
/// `Engine::condense`, 2026-11-20; word 8 held the old condensing's
/// walkers 11-15 to 11-17) and one spare. The
/// mould grid follows (`Engine::record_count`), room for squares down to a
/// game cell or `GRID_MIN` dish cells, whichever is smaller
/// (`Engine::grid_max`).
pub const WSTAT_BYTES: usize = 48;

/// The smallest mould grid square the wall counters have room for, in
/// dish cells, where the scale is bigger: the campaign's and the player
/// AI's squares of 4 (2026-11-12). Room down to a game cell (the scale) at
/// scale 8 fitted 40,960 squares, not the 163,840 squares of 4 of its 2048
/// x 1280 dish, so every ask there read all 10.5 MB of `occ` back.
/// A smaller square still reads back (`GpuWorld::mould_grid`).
pub const GRID_MIN: usize = 4;

/// The share of walkers the pull moves a tick (0.2%: at 100,000 walkers, a
/// quarter of them in about a second of holding).
pub const PULL: f32 = 0.002;

#[derive(Clone, Copy, PartialEq, Debug)]
pub enum Colour {
    Amber,
    Grey,
}

#[derive(Clone, Copy, PartialEq, Debug)]
pub enum Field {
    Trail,
    Walkers,
}

#[derive(Clone, Copy, PartialEq, Debug)]
pub enum Curve {
    Game,
    Soft,
}

/// Where the walkers start: spread over the dish, packed at home (today's
/// inoculum), or a disc in the middle (`BigParams::blob_r`, the looks
/// report's trunk start).
#[derive(Clone, Copy, PartialEq, Debug)]
pub enum Start {
    Spread,
    Home,
    Disc,
}

/// Every control. Angles in degrees, distances in fine cells.
#[derive(Clone, PartialEq, Debug)]
pub struct Settings {
    pub walkers: usize,
    /// One walker a cell (crowd 1); dropped when there are more walkers than
    /// half the agar cells.
    pub crowd: bool,
    pub sensor_angle: f32,
    pub turn: f32,
    pub reach: f32,
    pub step: f32,
    /// Trail laid a step by 100,000 walkers: the deposit is
    /// `laid * 100,000 / walkers`.
    pub laid: f32,
    pub keep: f32,
    pub respawn: f32,
    /// The dish's seven piles (dropped food stays either way).
    pub food: bool,
    /// How far out food is smelled (`BigParams::scent_offset`).
    pub smell: f32,
    pub colour: Colour,
    pub field: Field,
    pub curve: Curve,
    pub knee: f32,
    pub gamma: f32,
    /// Ticks a frame.
    pub ticks: u32,
    pub start: Start,
    /// Jenson's looks (`Reports/jenson-looks-2026-11-11.md`, `BigParams`).
    /// The disc start's radius, and the share of walkers spread over the
    /// dish instead with a disc or home start (`haze`).
    pub disc: f32,
    pub haze: f32,
    /// Each start an organic outline (`BigParams::start_shape`).
    pub start_shape: bool,
    /// Settings set by the trail a walker stands in (36 Points):
    /// s = min(trail x `trail_scale`, 1), 0 off; each setting is its base
    /// plus amp x s^power, `[amp, power]` (angles in degrees).
    pub trail_scale: f32,
    pub reach_by: [f32; 2],
    pub angle_by: [f32; 2],
    pub turn_by: [f32; 2],
    pub step_by: [f32; 2],
    /// The capped square-root deposit (`deposit_cap`), 0 off.
    pub cap: u16,
    /// The sensing floor and wander (chance a look), 0 off.
    pub floor: f32,
    pub wander: f32,
    /// Contagion, off when `sick` is 0: sick and immune for so many ticks,
    /// caught in germ over `catch_at`; a sick walker lays `germ` a tick
    /// (kept `germ_keep` a tick as it spreads) and moves, lays trail,
    /// senses and turns so many times as much.
    pub sick: u32,
    pub immune: u32,
    pub catch_at: f32,
    pub germ: f32,
    pub germ_keep: f32,
    pub sick_speed: f32,
    pub sick_trail: f32,
    pub sick_reach: f32,
    pub sick_turn: f32,
    /// A spinning walker's curl (`BigParams::spin_curl`, radians, 0 none),
    /// set by each Mind burst.
    pub spin_curl: f32,
    /// A sick walker pays this times the upkeep (`BigParams::sick_upkeep`;
    /// 1 costs nothing more).
    pub sick_upkeep: f32,
    /// The walkers within `seed_r` of the middle fall sick `seed_at`
    /// seconds in (0: at the start, `BigParams::sick_seed_r`).
    pub seed_at: f32,
    pub seed_r: f32,
    /// The outside rules (`BigParams` of PR #55,
    /// `Reports/outside-rules-2026-11-11.md`), off by default. Species
    /// (0 or 1 is none, up to 4): walker k is species k % n, smelling its
    /// own trail x (1 + `avoid`) less all trail x `avoid`; `zones` starts
    /// each in its own band of a spread start; `cycle` (3 or more) weighs
    /// the next species' trail by `next` and the previous one's by `prev`.
    pub species: u8,
    pub avoid: f32,
    pub zones: bool,
    pub cycle: bool,
    pub next: f32,
    pub prev: f32,
    /// Explorers: a walker younger than `explore` ticks steers to the
    /// lowest reading and lays `explorer_trail` of the deposit; with
    /// `explore_spread` each walker's stage is 0-2x as long.
    pub explore: u16,
    pub explorer_trail: f32,
    pub explore_spread: f32,
    /// A blocked walker lays `blocked_lay` of the deposit where it stands
    /// (not with the cap); with `blocked_keep` it keeps its heading, bar
    /// a reset with chance `blocked_reset` a tick.
    pub blocked_lay: f32,
    pub blocked_keep: bool,
    pub blocked_reset: f32,
    /// The chance turn's power (MCPM), 0 off; each walker's size spread
    /// (reach and step x 2^(spread x u)); the area sensors' radius.
    pub chance: f32,
    pub size_spread: f32,
    pub sensor_r: u8,
    /// PR #56 (`Reports/creatures-2026-11-11.md`): a walker joins a
    /// species whose trail at its cell outweighs its own `convert` times
    /// (0 off); grazing: grass starts at 1, each walker eats `graze_eat` a
    /// tick from its cell, it regrows by `graze_regrow` x (1 - grass), and
    /// a sensor adds `graze` x grass (0 off).
    pub convert: f32,
    pub graze: f32,
    pub graze_eat: f32,
    pub graze_regrow: f32,
    /// PR #58: species 0 lays this many times the deposit (1 even).
    pub boost: f32,
    /// Species 0's share of the walkers (`BigParams::species_share`, 0
    /// even), and conversion's square (`species_convert_r`: each species'
    /// trail summed over the (2r+1)-square round the walker, 0 the cell).
    /// The share is fixed for the dish's life (it places the walkers); the
    /// radius is live.
    pub species_share: f32,
    pub convert_r: u32,
    /// Mixed walkers (`BigParams::mix_*`): walker k is the second kind with
    /// chance `mix_share` (fixed for the dish's life, as the slots are);
    /// the second kind's reach, sensor angle, turn and step are times the
    /// others, and what it lays times `mix_deposit`.
    pub mix_share: f32,
    pub mix_reach: f32,
    pub mix_angle: f32,
    pub mix_turn: f32,
    pub mix_step: f32,
    pub mix_deposit: f32,
    /// Each species' own settings (`BigParams::species_sets`), set with
    /// `s0.reach=40` and the like: a field left unset is the global one.
    /// Only with two species or more; all unset is off.
    pub sets: [SpeciesKnobs; MAX_SPECIES as usize],
    /// The life cycle (`BigParams::spore_rate` and below, the tiers'
    /// Sporer, Sleeper and Swarmer; each means what the `BigParams` field
    /// of its name means): spores, dormancy and the alarm, all off at 0.
    /// The alarm field is there only if `alarm_keep` is over 0 when the
    /// dish is made, as on the CPU; the rest are live. Once any of spores
    /// or dormancy is on, or a strike has landed (`Engine::strike`), the
    /// walkers' life states stay on, as the CPU's do.
    pub spore_rate: f32,
    pub spore_scent: f32,
    pub spore_alarm: f32,
    pub spore_burst: f32,
    pub spore_steps: u16,
    pub spore_speed: f32,
    pub spore_reach: f32,
    pub spore_deposit: f32,
    pub spore_land: f32,
    pub spore_rest: u16,
    pub sleep_rate: f32,
    pub sleep_deposit: f32,
    pub wake_scent: f32,
    pub wake_alarm: f32,
    pub wake_steps: u16,
    pub sleep_rest: u16,
    pub sleep_death: f32,
    /// A gassed sleeper's chance a tick of dying (`BigParams::gas_death`;
    /// the toolkit's `BurstRules` sets it with the first gas).
    pub gas_death: f32,
    /// A flung walker's speed x `step` (`BigParams::fling_speed`; the
    /// rounds' `BurstRules` sets it with the first fling).
    pub fling_speed: f32,
    /// A flung walker's trail a step x `deposit` (`BigParams::fling_deposit`).
    pub fling_deposit: f32,
    /// The gas cloud (`BigParams::gas_keep`, `gas_sleep`, `gas_linger`;
    /// `Engine::lay_gas` lays it): its keep a step, the cloud a walker
    /// sleeps over, and the steps it stays gassed after.
    pub gas_keep: f32,
    pub gas_sleep: f32,
    pub gas_linger: u16,
    pub alarm_keep: f32,
    pub alarm_weight: f32,
    pub alarm_relay: f32,
    /// The scar (`BigParams::scar_keep`, `scar_gain`, `scar_weight`; the
    /// Grudge's habituation, on the GPU since 2026-11-17, the
    /// simulation-use plan's step 0): a slow per-cell memory of the alarm,
    /// read by the sensors beside it. Off with `scar_keep` 0.
    pub scar_keep: f32,
    pub scar_gain: f32,
    pub scar_weight: f32,
    /// The crust (`BigParams::crust_rise`, `crust_keep`, `crust_trail`; the
    /// simulation-use plan's E1a, 2026-11-17): ground that remembers where
    /// the slime has lived. Off with `crust_rise` 0.
    pub crust_rise: f32,
    pub crust_keep: f32,
    pub crust_trail: f32,
    /// The crust's effects (`BigParams::crust_weight`, `crust_fast`,
    /// `crust_fuel`, `char_barren`; the plan's E1b and C2).
    pub crust_weight: f32,
    pub crust_fast: f32,
    pub crust_fuel: f32,
    pub char_barren: f32,
    /// Setting (`BigParams::set_scar`, `set_wake`, `set_crust`,
    /// `set_armour`; the plan's E2a).
    pub set_scar: f32,
    pub set_wake: f32,
    pub set_crust: f32,
    pub set_armour: u32,
    /// Dry slime burns (`BigParams::fire_dry`, the plan's C1), and fed
    /// slime is fireproof (`fire_fed`).
    pub fire_dry: bool,
    pub fire_fed: bool,
    /// Fire lays alarm where it burns (`BigParams::fire_alarm`, the plan's
    /// C4; 0 off), carried in the plants' word 44.
    pub fire_alarm: f32,
    /// Gas ignites (`BigParams::gas_burn`, the plan's C5; 0 off), carried
    /// in the plants' word 43.
    pub gas_burn: f32,
    /// A far smell (`BigParams::far_smell`, the plan's S2; 0 off), in one
    /// body's row's last word (`Params::far`).
    pub far_smell: f32,
    /// **The lure's pull** (`lure_pull=W`, 2026-11-18, the Enemy lane; 0
    /// off, as here): a calm walker inside a lure's reach leans down the
    /// walking distance to it, x this (engine.wgsl's `lure_dist`, the
    /// lure block in `sense`). It is this engine's own rule with no
    /// `BigParams` field: the owner's card (11-17 19:21Z) builds new slime
    /// rules on the GPU only. A run's planets set it (`run::LURE_PULL`).
    /// The field is the game's (`Game::time_lure_paths`,
    /// `Engine::write_lure_field`); the pull is sent only while one is
    /// written (`Engine::lured`), in the soil row's last word.
    pub lure_pull: f32,
    /// **The tether** (`tether=N`, 2026-11-18, the Enemy lane, from the
    /// Lab's one-body prototype; 0 off, as here): a walker on its own way
    /// out from the body (a breakaway, a cut-off walker making for food;
    /// not a raider, not a charge at a unit, not a lump's member) steps
    /// only into a cell whose 8 neighbours, its own aside, hold N walkers
    /// (engine.wgsl's `tethered`). This engine's own rule with no
    /// `BigParams` field (the owner's card, 11-17 19:21Z: no CPU twin). A
    /// run's planets set it (`run::TETHER`). Params word 892, the lone
    /// step row's last.
    pub tether: u32,
    /// **Deep water is a wall to the slime** (`deep_wall=0` turns it off;
    /// on, as here, 2026-11-20, the owner: "Deep water shouldn't be
    /// traversable by player or slime"). A walker on dry or shallow ground
    /// never steps or jumps into deep water (`big::SOIL_DEEP`) and its
    /// sensors read it as rock, so the slime goes round a river by its
    /// fords and bridges as soldiers do; one already in it (flung, or put
    /// there by a map) swims out at `deep_step`. The slime's ways (raids,
    /// charges, the far smell, the lure) are built with it shut
    /// (`GpuWorld::raid_ground`). This engine's own rule with no
    /// `BigParams` field (the owner's card, 11-17 19:21Z: no CPU twin, so
    /// the CPU dish's slime still crawls through and the twin test turns
    /// it off). Params word 929, the ground row's second.
    pub deep_wall: bool,
    /// The flinch (`BigParams::flinch`): folded into `alarm_keep` and
    /// `alarm_weight` when the world is made (`Settings::flinched`).
    pub flinch: f32,
    /// A tier's resistance to the germ (`BigParams::germ_resist`, 1 none;
    /// 2026-11-17): a healthy walker catches it only over `catch_at` x
    /// this (`write_params` sends the product, so the passes are the
    /// CPU's).
    pub germ_resist: f32,
    /// A tier's frailty to the germ (`BigParams::germ_upkeep`, 1 none;
    /// 2026-11-17): a sick walker pays `sick_upkeep` x this
    /// (`write_params` sends the product).
    pub germ_upkeep: f32,
    /// Growth (Larder World W0, `BigParams::upkeep`; 0 off): energy a
    /// walker pays a tick, what it starts with, what it divides at, what a
    /// division costs, the chance a tick that a walker at zero dies, and
    /// the slots (0: `walkers`; the first `walkers` start alive). Growth
    /// and the slots are fixed for a dish's life.
    pub upkeep: f32,
    pub start_energy: f32,
    pub birth_energy: f32,
    pub birth_cost: f32,
    pub starve: f32,
    pub capacity: usize,
    /// Most living walkers each species may hold (`BigParams::species_cap`;
    /// empty is off). On the GPU only in a garden (the player's own mould,
    /// 2026-11-14), where the enemy (species 0) and the player's mould
    /// (species 3) are counted each tick before the births; a cap of 0
    /// stops a species' births. Written to each species `Set`'s `cap` word
    /// (its old `_p`), u32::MAX for no cap.
    pub species_cap: Vec<usize>,
    /// The Larder World (W1b, 2026-11-11; `BigParams`'s names in `set`):
    /// the dish's piles and its shape (`rect`); crystal charge a cell (0
    /// off), eaten in equal shares, flowing inside a crystal and regrowing
    /// as on the CPU; rock and wet ground (`terrain_rock`, `terrain_wet`,
    /// 0 off) and the wet step; walls' health (0: they never break), the
    /// bite a blocked walker takes, and the scent they let through.
    pub piles: usize,
    pub rect: bool,
    pub crystal: f32,
    pub crystal_eat: f32,
    pub crystal_flow: f32,
    pub crystal_regrow: f32,
    pub rock: f32,
    pub wet: f32,
    pub wet_step: f32,
    pub terrain_blob: f32,
    pub wall_hp: f32,
    pub wall_bite: f32,
    pub wall_scent: f32,
    /// One body (2026-11-12, `BigParams::energy_flow`, `hunger_scent`,
    /// `hunger_at`; Reports/one-body-2026-11-12.md): walkers' energy evens
    /// out between neighbouring cells, and a hungry walker follows the
    /// scent harder. 0 off; both need growth.
    pub energy_flow: f32,
    pub hunger_scent: f32,
    pub hunger_at: f32,
    /// A part cut off from its food withers (2026-11-13,
    /// `BigParams::wither` and its clock, `wither_every`, `wither_lag`,
    /// `wither_reach`): news of food travels through the body and each
    /// walker carries the last it heard; one that has heard none for
    /// `wither_lag` steps dies with chance `wither` at each check. 0 off;
    /// needs growth.
    pub wither: f32,
    pub wither_every: u32,
    pub wither_lag: u32,
    pub wither_reach: u8,
    /// A cut-off part panics (2026-11-13, `BigParams::panic`, `panic_r`):
    /// a walker whose news is older than `wither_lag` runs for the nearest
    /// unit within `panic_r` dish cells, else the nearest crystal with food
    /// left, `panic` times as fast, by its sensors' direction alone. 0 off;
    /// needs growth.
    pub panic: f32,
    pub panic_r: f32,
    /// Steps after the lag runs out that a cut-off walker leans: steers
    /// for its target at its own pace before it bolts
    /// (`BigParams::panic_lean`, at most 127).
    pub panic_lean: u32,
    /// The lean's pace, times a walker's step (`BigParams::panic_creep`).
    pub panic_creep: f32,
    /// Steps a charge at a unit bolts before it creeps again
    /// (`BigParams::panic_dash`, 0 never).
    pub panic_dash: u32,
    /// A speck's ripeness (`BigParams::condense_crowd`, `condense_r`): the
    /// walkers within the radius that let it leave. Condensing
    /// (`condense`) was deleted 11-17.
    pub condense_crowd: u32,
    pub condense_r: u8,
    /// Slime breaks away (`BigParams::roam`, `roam_far`, `roam_share`;
    /// 11-14): steps between chances to leave, 0 off (the specks, 11-16).
    pub roam: u32,
    pub roam_far: f32,
    pub roam_share: f32,
    /// The body surges (`BigParams::surge_every`, `surge_lean`,
    /// `surge_dash`, `surge_haste`; 11-15): steps between surges (0 off),
    /// the creep's and the lunge's steps, the lunge's haste; the reach is
    /// `panic_r`.
    pub surge_every: u32,
    pub surge_lean: u32,
    pub surge_dash: u32,
    pub surge_haste: f32,
    /// The step the game's last surge began on (`BigParams::surge_at`; 0
    /// the clock above): set by the game (`World::set_surge_at`), not a key.
    pub surge_at: u32,
    /// A raid (`BigParams::raid_at` and the rest, 11-15): the game sets
    /// the first step, target, reach and share (`World::set_raid`); the
    /// lean, run and haste are keys.
    pub raid_at: u32,
    pub raid_to: u32,
    pub raid_r: f32,
    pub raid_share: f32,
    pub raid_lean: u32,
    pub raid_len: u32,
    pub raid_haste: f32,
    /// The lumps (`BigParams::lumps`, held bodies, 2026-11-16): set by the
    /// game (`World::set_lump`), not keys.
    pub lumps: [crate::mould::big::Lump; crate::mould::big::LUMPS],
    /// The food scent's keep a step and a reached pile's share of it
    /// (`BigParams::scent_keep`, `fed_scent`; keys since 11-15, the whole-
    /// game review's W44, so a film can turn scent up).
    pub scent_keep: f32,
    pub fed_scent: f32,
    /// The crater's cut, length of full bite and cone (`BigParams::
    /// scorch_keep`, `scorch_full`, `scorch_cone`), for the look A/B of a
    /// shot's hole (`larder-world scorch_cone=1 ...`).
    pub scorch_keep: f32,
    pub scorch_full: u16,
    pub scorch_cone: bool,
    /// Plants on a map that has none (`plants::stand_in`, 11-14): until
    /// levelgen writes each biome's, the game grows the stand-in on any
    /// designed map or planet without plants of its own (`plants=0` off).
    pub plants: bool,
    pub seed: u64,
}

impl Settings {
    /// These settings with the flinch folded into the alarm
    /// (`big::flinch_alarm`), as `BigDish` folds its params when made.
    pub fn flinched(mut self) -> Self {
        (self.alarm_keep, self.alarm_weight) = crate::mould::big::flinch_alarm(self.alarm_keep, self.alarm_weight, self.flinch);
        self
    }

    /// The phase 2 big dish: the `nose` rule at 100,000 walkers, food on,
    /// the game's look (examples/bigdish.rs preset `dense`, spread start).
    pub fn big_dish() -> Self {
        let t = BigParams::today();
        Settings {
            walkers: 100_000,
            crowd: true,
            sensor_angle: t.sensor_angle.to_degrees(),
            turn: t.rotate_angle.to_degrees(),
            reach: t.sensor_offset,
            step: t.step,
            laid: t.deposit,
            keep: t.keep,
            respawn: 0.0,
            food: true,
            smell: t.sensor_offset * 4.0,
            colour: Colour::Amber,
            field: Field::Trail,
            curve: Curve::Game,
            knee: 6.0,
            gamma: 0.6,
            ticks: 2,
            start: Start::Spread,
            disc: 180.0,
            haze: 0.0,
            start_shape: false,
            trail_scale: 0.0,
            reach_by: [0.0, 1.0],
            angle_by: [0.0, 1.0],
            turn_by: [0.0, 1.0],
            step_by: [0.0, 1.0],
            cap: 0,
            floor: 0.0,
            wander: 0.0,
            sick: 0,
            immune: 480,
            catch_at: 0.5,
            germ: 1.0,
            germ_keep: 0.9,
            sick_speed: 1.0,
            sick_trail: 1.0,
            sick_reach: 1.0,
            sick_turn: 1.0,
            sick_upkeep: 1.0,
            spin_curl: t.spin_curl,
            seed_at: 4.0,
            seed_r: 40.0,
            species: 0,
            avoid: t.species_avoid,
            zones: false,
            cycle: false,
            next: t.species_next,
            prev: t.species_prev,
            explore: 0,
            explorer_trail: t.explore_deposit,
            explore_spread: 0.0,
            blocked_lay: 0.0,
            blocked_keep: false,
            blocked_reset: t.blocked_reset,
            chance: 0.0,
            size_spread: 0.0,
            sensor_r: 0,
            convert: t.species_convert,
            graze: t.graze_weight,
            graze_eat: t.graze_eat,
            graze_regrow: t.graze_regrow,
            boost: t.species_boost,
            species_share: t.species_share,
            convert_r: t.species_convert_r,
            mix_share: t.mix_share,
            mix_reach: t.mix_reach,
            mix_angle: t.mix_angle,
            mix_turn: t.mix_turn,
            mix_step: t.mix_step,
            mix_deposit: t.mix_deposit,
            sets: [SpeciesKnobs::default(); MAX_SPECIES as usize],
            spore_rate: t.spore_rate,
            spore_scent: t.spore_scent,
            spore_alarm: t.spore_alarm,
            spore_burst: t.spore_burst,
            spore_steps: t.spore_steps,
            spore_speed: t.spore_speed,
            spore_reach: t.spore_reach,
            spore_deposit: t.spore_deposit,
            spore_land: t.spore_land,
            spore_rest: t.spore_rest,
            sleep_rate: t.sleep_rate,
            sleep_deposit: t.sleep_deposit,
            wake_scent: t.wake_scent,
            wake_alarm: t.wake_alarm,
            wake_steps: t.wake_steps,
            sleep_rest: t.sleep_rest,
            sleep_death: t.sleep_death,
            gas_death: t.gas_death,
            fling_speed: t.fling_speed,
            fling_deposit: t.fling_deposit,
            gas_keep: t.gas_keep,
            gas_sleep: t.gas_sleep,
            gas_linger: t.gas_linger,
            alarm_keep: t.alarm_keep,
            alarm_weight: t.alarm_weight,
            alarm_relay: t.alarm_relay,
            scar_keep: t.scar_keep,
            scar_gain: t.scar_gain,
            scar_weight: t.scar_weight,
            crust_rise: t.crust_rise,
            crust_keep: t.crust_keep,
            crust_trail: t.crust_trail,
            crust_weight: t.crust_weight,
            crust_fast: t.crust_fast,
            crust_fuel: t.crust_fuel,
            char_barren: t.char_barren,
            set_scar: t.set_scar,
            set_wake: t.set_wake,
            set_crust: t.set_crust,
            set_armour: t.set_armour,
            fire_dry: t.fire_dry,
            fire_fed: t.fire_fed,
            fire_alarm: t.fire_alarm,
            gas_burn: t.gas_burn,
            far_smell: t.far_smell,
            lure_pull: 0.0,
            tether: 0,
            deep_wall: true,
            flinch: t.flinch,
            germ_resist: t.germ_resist,
            germ_upkeep: t.germ_upkeep,
            upkeep: t.upkeep,
            start_energy: t.start_energy,
            birth_energy: t.birth_energy,
            birth_cost: t.birth_cost,
            starve: t.starve,
            capacity: t.capacity,
            species_cap: t.species_cap.clone(),
            piles: t.piles,
            rect: t.rect,
            crystal: t.crystal_charge,
            crystal_eat: t.crystal_eat,
            crystal_flow: t.crystal_flow,
            crystal_regrow: t.crystal_regrow,
            rock: t.terrain_rock,
            wet: t.terrain_wet,
            wet_step: t.wet_step,
            terrain_blob: t.terrain_blob,
            wall_hp: t.wall_hp,
            wall_bite: t.wall_bite,
            wall_scent: t.wall_scent,
            energy_flow: t.energy_flow,
            hunger_scent: t.hunger_scent,
            hunger_at: t.hunger_at,
            wither: t.wither,
            wither_every: t.wither_every,
            wither_lag: t.wither_lag,
            wither_reach: t.wither_reach,
            panic: t.panic,
            panic_r: t.panic_r,
            panic_lean: t.panic_lean,
            panic_creep: t.panic_creep,
            panic_dash: t.panic_dash,
            condense_crowd: t.condense_crowd,
            condense_r: t.condense_r,
            roam: t.roam,
            roam_far: t.roam_far,
            roam_share: t.roam_share,
            surge_every: t.surge_every,
            surge_lean: t.surge_lean,
            surge_dash: t.surge_dash,
            surge_haste: t.surge_haste,
            surge_at: t.surge_at,
            raid_at: t.raid_at,
            raid_to: t.raid_to,
            raid_r: t.raid_r,
            raid_share: t.raid_share,
            raid_lean: t.raid_lean,
            raid_len: t.raid_len,
            raid_haste: t.raid_haste,
            lumps: t.lumps,
            scent_keep: t.scent_keep,
            fed_scent: t.fed_scent,
            scorch_keep: t.scorch_keep,
            scorch_full: t.scorch_full,
            scorch_cone: t.scorch_cone,
            plants: true,
            seed: 1,
        }
    }

    /// Whether contagion is on.
    pub fn sick_on(&self) -> bool {
        self.sick > 0
    }

    /// Species fields the dish holds (0 for none).
    pub fn fields(&self) -> u32 {
        if self.species > 1 {
            self.species.min(MAX_SPECIES) as u32
        } else {
            0
        }
    }

    /// Whether the species have their own settings (`sets`): two species
    /// or more, and any field set.
    pub fn per_species(&self) -> bool {
        self.fields() > 0 && self.sets[..self.fields() as usize].iter().any(|k| *k != SpeciesKnobs::default())
    }

    /// Whether any walker explores: the global stage or a species' own.
    pub fn explores(&self) -> bool {
        self.explore > 0 || (self.per_species() && species_sets(self, true).iter().any(|t| t.explore_steps > 0))
    }
}

/// One species' own settings (`Settings::sets`; `None` is the global
/// field). Angles in degrees and `laid` per 100,000 walkers, as the
/// globals.
#[derive(Clone, Copy, PartialEq, Debug, Default)]
pub struct SpeciesKnobs {
    pub sensor_angle: Option<f32>,
    pub turn: Option<f32>,
    pub reach: Option<f32>,
    pub step: Option<f32>,
    pub laid: Option<f32>,
    pub keep: Option<f32>,
    pub crowd: Option<bool>,
    pub blocked_lay: Option<f32>,
    pub graze: Option<f32>,
    pub explore: Option<u16>,
}

/// The CPU's `species_sets` from the settings: empty unless
/// `per_species`; a species' crowd limit only where `crowd_ok`.
pub fn species_sets(s: &Settings, crowd_ok: bool) -> Vec<SpeciesSet> {
    if !s.per_species() {
        return Vec::new();
    }
    let per = 100_000.0 / s.walkers.max(1) as f32;
    s.sets[..s.fields() as usize]
        .iter()
        .map(|k| SpeciesSet {
            sensor_angle: k.sensor_angle.unwrap_or(s.sensor_angle).to_radians(),
            sensor_offset: k.reach.unwrap_or(s.reach),
            rotate_angle: k.turn.unwrap_or(s.turn).to_radians(),
            step: k.step.unwrap_or(s.step),
            deposit: k.laid.unwrap_or(s.laid) * per,
            keep: k.keep.unwrap_or(s.keep),
            crowd: (k.crowd.unwrap_or(s.crowd) && crowd_ok) as u8,
            blocked_deposit: k.blocked_lay.unwrap_or(s.blocked_lay),
            graze_weight: k.graze.unwrap_or(s.graze),
            explore_steps: k.explore.unwrap_or(s.explore),
        })
        .collect()
}

/// A switch on the command line: 1/on/true/yes or 0/off/false/no (until the
/// review of 2026-11-11 anything but 0 or off meant on, so `crowd=false`
/// turned the limit on).
pub fn on_off(key: &str, v: &str) -> Result<bool, String> {
    match v {
        "1" | "on" | "true" | "yes" => Ok(true),
        "0" | "off" | "false" | "no" => Ok(false),
        _ => Err(format!("{key}={v}: 1 or 0 (on, off)")),
    }
}

/// Sets a control from a `key=value` argument.
pub fn set(s: &mut Settings, key: &str, v: &str) -> Result<(), String> {
    let f = || v.parse::<f32>().map_err(|e| format!("{key}={v}: {e}"));
    let int = || v.parse::<u32>().map_err(|e| format!("{key}={v}: {e}"));
    // `amp` or `amp:power`, as examples/bigdish.rs takes the `*_mod`s.
    let pair = || -> Result<[f32; 2], String> {
        let (a, e) = v.split_once(':').unwrap_or((v, "1"));
        Ok([a.parse().map_err(|e| format!("{key}={v}: {e}"))?, e.parse().map_err(|e| format!("{key}={v}: {e}"))?])
    };
    let b = || on_off(key, v);
    let pick = |names: &[&str]| names.iter().position(|&n| n == v).ok_or_else(|| format!("{key}={v}: one of {}", names.join(", ")));
    // A species' own setting: `s0.reach=40` (species 0 to 3).
    if let Some((sp, field)) = key.split_once('.').and_then(|(a, f)| Some((a.strip_prefix('s')?.parse::<usize>().ok()?, f))) {
        let k = s.sets.get_mut(sp).ok_or_else(|| format!("{key}: species 0 to {}", MAX_SPECIES - 1))?;
        match field {
            "sensor_angle" => k.sensor_angle = Some(f()?),
            "turn" | "rotate_angle" => k.turn = Some(f()?),
            "reach" | "sensor_offset" => k.reach = Some(f()?),
            "step" => k.step = Some(f()?),
            "laid" => k.laid = Some(f()?),
            "keep" => k.keep = Some(f()?),
            "crowd" => k.crowd = Some(b()?),
            "blocked_lay" | "blocked_deposit" => k.blocked_lay = Some(f()?),
            "graze" | "graze_weight" => k.graze = Some(f()?),
            "explore" | "explore_steps" => k.explore = Some(int()?.min(u16::MAX as u32) as u16),
            _ => return Err(format!("unknown species control {key}")),
        }
        return Ok(());
    }
    match key {
        "walkers" => s.walkers = v.parse().map_err(|e| format!("{key}={v}: {e}"))?,
        "crowd" => s.crowd = b()?,
        "sensor_angle" => s.sensor_angle = f()?,
        "turn" | "rotate_angle" => s.turn = f()?,
        "reach" | "sensor_offset" => s.reach = f()?,
        "step" => s.step = f()?,
        "laid" => s.laid = f()?,
        "keep" => s.keep = f()?,
        "respawn" => s.respawn = f()?,
        "food" => s.food = b()?,
        "smell" | "scent_offset" => s.smell = f()?,
        "colour" | "color" => s.colour = [Colour::Amber, Colour::Grey, Colour::Grey][pick(&["amber", "grey", "gray"])?],
        "field" => s.field = [Field::Trail, Field::Walkers, Field::Walkers][pick(&["trail", "walkers", "count"])?],
        "curve" => s.curve = [Curve::Game, Curve::Soft][pick(&["game", "soft"])?],
        "knee" => s.knee = f()?,
        "gamma" => s.gamma = f()?,
        "ticks" => s.ticks = v.parse::<u32>().map_err(|e| format!("{key}={v}: {e}"))?.clamp(1, 32),
        "start" => s.start = [Start::Spread, Start::Home, Start::Home, Start::Disc][pick(&["spread", "blob", "home", "disc"])?],
        "disc" => s.disc = f()?,
        "haze" => s.haze = f()?,
        "trail_scale" | "sense_scale" => s.trail_scale = f()?,
        "reach_by" | "offset_mod" => s.reach_by = pair()?,
        "angle_by" | "angle_mod" => s.angle_by = pair()?,
        "turn_by" | "turn_mod" => s.turn_by = pair()?,
        "step_by" | "step_mod" => s.step_by = pair()?,
        "cap" | "deposit_cap" => s.cap = int()?.min(u16::MAX as u32) as u16,
        "floor" | "sense_floor" => s.floor = f()?,
        "wander" => s.wander = f()?,
        "sick" | "sick_steps" => s.sick = int()?.min(30_000),
        "immune" | "immune_steps" => s.immune = int()?.min(30_000),
        "catch" | "sick_catch" => s.catch_at = f()?,
        "germ" | "sick_deposit" => s.germ = f()?,
        "germ_keep" | "sick_keep" => s.germ_keep = f()?,
        "sick_speed" => s.sick_speed = f()?,
        "sick_trail" => s.sick_trail = f()?,
        "sick_reach" => s.sick_reach = f()?,
        "sick_turn" => s.sick_turn = f()?,
        "spin_curl" => s.spin_curl = f()?,
        "sick_upkeep" => s.sick_upkeep = f()?,
        "seed_at" => s.seed_at = f()?,
        "seed_r" => s.seed_r = f()?,
        // The outside rules, with the CPU's names too.
        "species" => s.species = int()?.min(MAX_SPECIES as u32) as u8,
        "avoid" | "species_avoid" => s.avoid = f()?,
        "zones" | "species_zones" => s.zones = b()?,
        "cycle" | "species_cycle" => s.cycle = b()?,
        "next" | "species_next" => s.next = f()?,
        "prev" | "species_prev" => s.prev = f()?,
        "explore" | "explore_steps" => s.explore = int()?.min(u16::MAX as u32) as u16,
        "explorer_trail" | "explore_deposit" => s.explorer_trail = f()?,
        "explore_spread" => s.explore_spread = f()?,
        "blocked_lay" | "blocked_deposit" => s.blocked_lay = f()?,
        "blocked_keep" => s.blocked_keep = b()?,
        "blocked_reset" => s.blocked_reset = f()?,
        "chance" | "chance_exp" => s.chance = f()?,
        "size_spread" => s.size_spread = f()?,
        "sensor_r" => s.sensor_r = int()?.min(8) as u8,
        "convert" | "species_convert" => s.convert = f()?,
        "graze" | "graze_weight" => s.graze = f()?,
        "graze_eat" => s.graze_eat = f()?,
        "graze_regrow" => s.graze_regrow = f()?,
        "boost" | "species_boost" => s.boost = f()?,
        "share" | "species_share" => s.species_share = f()?,
        // At most 64: the rows pass keeps 64 cells either side of its
        // piece of a row (engine.wgsl's `ROW_R`).
        "convert_r" | "species_convert_r" => s.convert_r = int()?.min(64),
        "mix_share" => s.mix_share = f()?,
        "mix_reach" => s.mix_reach = f()?,
        "mix_angle" => s.mix_angle = f()?,
        "mix_turn" => s.mix_turn = f()?,
        "mix_step" => s.mix_step = f()?,
        "mix_deposit" => s.mix_deposit = f()?,
        // The life cycle.
        "spore_rate" => s.spore_rate = f()?,
        "spore_scent" => s.spore_scent = f()?,
        "spore_alarm" => s.spore_alarm = f()?,
        "spore_burst" => s.spore_burst = f()?,
        "spore_steps" => s.spore_steps = int()?.min(65535) as u16,
        "spore_speed" => s.spore_speed = f()?,
        "spore_reach" => s.spore_reach = f()?,
        "spore_deposit" => s.spore_deposit = f()?,
        "spore_land" => s.spore_land = f()?,
        "spore_rest" => s.spore_rest = int()?.min(65535) as u16,
        "sleep_rate" => s.sleep_rate = f()?,
        "sleep_deposit" => s.sleep_deposit = f()?,
        "wake_scent" => s.wake_scent = f()?,
        "wake_alarm" => s.wake_alarm = f()?,
        "wake_steps" => s.wake_steps = int()?.min(65535) as u16,
        "sleep_rest" => s.sleep_rest = int()?.min(65535) as u16,
        "sleep_death" => s.sleep_death = f()?,
        "gas_death" => s.gas_death = f()?,
        "fling_speed" => s.fling_speed = f()?,
        "fling_deposit" => s.fling_deposit = f()?,
        "gas_keep" => s.gas_keep = f()?,
        "gas_sleep" => s.gas_sleep = f()?,
        "alarm_keep" => s.alarm_keep = f()?,
        "alarm_weight" => s.alarm_weight = f()?,
        "flinch" => s.flinch = f()?.max(0.0),
        "germ_resist" => s.germ_resist = f()?.max(0.0),
        "germ_upkeep" => s.germ_upkeep = f()?.max(0.0),
        "alarm_relay" => s.alarm_relay = f()?,
        "scar_keep" => s.scar_keep = f()?,
        "scar_gain" => s.scar_gain = f()?,
        "scar_weight" => s.scar_weight = f()?,
        // The window's crust at once (`crust=1`; 0 turns it off): a cell
        // fills in a minute of steady living and halves in two minutes
        // with none, at the window's 30 fine steps a second (scale 2).
        // With it, the crust burns (C2, 2026-11-17): worth 3 plants a unit,
        // so a fire lit on ground the slime left runs across it (planet 3,
        // seed 1: 4,382 cells burning 60 s after a 4-cell light, against
        // 30 at 1 and none left at 0) and stops at the fed body; char of
        // 0.3 or more is barren. Its pull (`crust_weight`) and fast ground
        // stay 0 until the owner picks a sign.
        "crust" => {
            let on = b()?;
            (s.crust_rise, s.crust_keep, s.crust_trail) = if on { (1.0 / 1800.0, 0.5f32.powf(1.0 / 3600.0), 1.0) } else { (0.0, 1.0, 1.0) };
            (s.crust_fuel, s.char_barren) = if on { (3.0, 0.3) } else { (0.0, 0.0) };
        }
        "crust_rise" => s.crust_rise = f()?.max(0.0),
        "crust_keep" => s.crust_keep = f()?.clamp(0.0, 1.0),
        "crust_trail" => s.crust_trail = f()?,
        // The crust's effects (E1b, C2): its pull on the sensors (under 0 a
        // push), fast ground, fuel for fire and barren char.
        "crust_weight" => s.crust_weight = f()?,
        "crust_fast" => s.crust_fast = f()?.max(0.0),
        "crust_fuel" => s.crust_fuel = f()?.max(0.0),
        "char_barren" => s.char_barren = f()?.max(0.0),
        // Setting (E2a): the scar a walker on crust sets at (0 off), the
        // scar it wakes under, the crust it needs, its armour in blows.
        "set_scar" => s.set_scar = f()?.max(0.0),
        "set_wake" => s.set_wake = f()?.max(0.0),
        "set_crust" => s.set_crust = f()?,
        "set_armour" => s.set_armour = int()?.clamp(1, 255),
        // Dry slime burns (`fire_dry=1`, the plan's C1): the trail wets a
        // cell against fire only where the slime is fed; cut-off and set
        // slime burn. Needs the wither's news on.
        "fire_dry" => s.fire_dry = b()?,
        // Fed slime is fireproof (`fire_fed=1`): a cell by fresh news is wet
        // against fire whatever its trail.
        "fire_fed" => s.fire_fed = b()?,
        // Fire scares the slime a little (`fire_alarm=F`, the plan's C4):
        // burning cells add F x their fire to the alarm field (shape C's
        // flinch reads it).
        "fire_alarm" => s.fire_alarm = f()?,
        // Gas ignites (`gas_burn=G`, the plan's C5): cloud over G lights
        // from a light or a burning neighbour and is used up.
        "gas_burn" => s.gas_burn = f()?,
        // A far smell (`far_smell=W`, the plan's S2): hungry walkers lean
        // down the walking distance to untouched crystals; specks walk it.
        "far_smell" => s.far_smell = f()?,
        // The lure's pull (`lure_pull=W`, 2026-11-18): calm slime inside a
        // lure's reach leans down the walking distance to it. The lab's
        // rig: 10 a steady stream with the body at home, 30 emptied a
        // colony in 45 s (Reports/lab-steering-2026-11-17).
        "lure_pull" => s.lure_pull = f()?,
        // The tether (`tether=N`, 2026-11-18): slime on its way out from
        // the body steps only where it touches N others (Lab, one body:
        // Reports/lab-body-2026-11-18; 2 its pick).
        "tether" => s.tether = int()?.min(8),
        // Deep water as a wall to the slime (`deep_wall=0` off; on by
        // default, 2026-11-20).
        "deep_wall" => s.deep_wall = int()? != 0,
        // Setting at once (`setting=1`; 0 turns it off): the crust if it is
        // off, the scar as a plain memory (keep 0.999, gain 0.01, no pull)
        // if it is off, and a walker on crust of 0.2 sets where its cell's
        // scar is 0.05, wakes under 0.01 and dies to its third blow. Planet 3,
        // seed 1, a standing rifle squad: 217 walkers set at 60 s, the squad
        // killed 265 of the slime in the first minute against 366 without;
        // a set_scar of 0.2 set 150 and 1 set none (the scar is over 1 only
        // on the cells shot most, which the walkers flinch off).
        "setting" => {
            if b()? {
                if s.crust_rise == 0.0 {
                    set(s, "crust", "1")?;
                }
                if s.scar_keep == 0.0 {
                    (s.scar_keep, s.scar_gain) = (0.999, 0.01);
                }
                (s.set_scar, s.set_wake, s.set_crust, s.set_armour) = (0.05, 0.01, 0.2, 3);
            } else {
                s.set_scar = 0.0;
            }
        }
        // A mould tier's keys at once (`mould::tiers::Tier`, as
        // `examples/bigdish.rs`'s `tier=`): `tier=sleeper`.
        "tier" => crate::mould::tiers::Tier::from_name(v).ok_or_else(|| format!("tier={v}: one of {}", crate::mould::tiers::Tier::ALL.map(|t| t.name()).join(", ")))?.settings(s)?,
        "upkeep" => s.upkeep = f()?,
        "start_energy" => s.start_energy = f()?,
        "birth_energy" => s.birth_energy = f()?,
        "birth_cost" => s.birth_cost = f()?,
        "starve" => s.starve = f()?,
        "capacity" | "slots" => s.capacity = v.parse().map_err(|e| format!("{key}={v}: {e}"))?,
        // The Larder World.
        "piles" => s.piles = int()?.min(MAX_PILES as u32) as usize,
        "rect" => s.rect = b()?,
        "crystal" | "crystal_charge" => s.crystal = f()?,
        "crystal_eat" => s.crystal_eat = f()?,
        "crystal_flow" => s.crystal_flow = f()?,
        "crystal_regrow" => s.crystal_regrow = f()?,
        "rock" | "terrain_rock" => s.rock = f()?,
        "wet" | "terrain_wet" => s.wet = f()?,
        "wet_step" => s.wet_step = f()?,
        "terrain_blob" => s.terrain_blob = f()?,
        "wall_hp" => s.wall_hp = f()?,
        "wall_bite" => s.wall_bite = f()?,
        "wall_scent" => s.wall_scent = f()?,
        "energy_flow" => s.energy_flow = f()?,
        "hunger_scent" => s.hunger_scent = f()?,
        "hunger_at" => s.hunger_at = f()?,
        "wither" => s.wither = f()?,
        "wither_every" => s.wither_every = int()?.clamp(1, 255),
        "wither_lag" => s.wither_lag = int()?.min(0xFFFF),
        "wither_reach" => s.wither_reach = int()?.clamp(1, 255) as u8,
        "panic" => s.panic = f()?,
        "panic_r" => s.panic_r = f()?,
        "panic_lean" => s.panic_lean = int()?.min(127),
        "panic_creep" => s.panic_creep = f()?,
        "panic_dash" => s.panic_dash = int()?,
        "condense_crowd" => s.condense_crowd = int()?,
        "condense_r" => s.condense_r = int()?.clamp(1, 8) as u8,
        "surge" => s.surge_every = int()?,
        "surge_lean" => s.surge_lean = int()?.min(crate::mould::big::SURGE_MAX),
        "surge_dash" => s.surge_dash = int()?.min(crate::mould::big::SURGE_MAX),
        "surge_haste" => s.surge_haste = f()?,
        "raid_lean" => s.raid_lean = int()?.min(crate::mould::big::SURGE_MAX),
        "raid_len" => s.raid_len = int()?.min(crate::mould::big::SURGE_MAX),
        "raid_haste" => s.raid_haste = f()?,
        "scent_keep" => s.scent_keep = f()?.clamp(0.0, 1.0),
        "fed_scent" => s.fed_scent = f()?.max(0.0),
        "roam" => s.roam = int()?.min(crate::mould::big::RIPE),
        "roam_far" => s.roam_far = f()?,
        "roam_share" => s.roam_share = f()?,
        "scorch_keep" => s.scorch_keep = f()?,
        "scorch_full" => s.scorch_full = int()?.clamp(1, 65535) as u16,
        "scorch_cone" => s.scorch_cone = b()?,
        "plants" => s.plants = b()?,
        "seed" => s.seed = v.parse().map_err(|e| format!("{key}={v}: {e}"))?,
        _ => return Err(format!("unknown control {key}")),
    }
    Ok(())
}

/// The CPU dish a sandbox dish starts from (walkers, agar, piles).
pub fn big_params(s: &Settings, scale: usize, crowd: bool) -> BigParams {
    let t = BigParams::today();
    BigParams {
        scale,
        walkers: s.walkers,
        sensor_angle: s.sensor_angle.to_radians(),
        rotate_angle: s.turn.to_radians(),
        sensor_offset: s.reach,
        scent_offset: s.smell,
        step: s.step,
        deposit: s.laid * 100_000.0 / s.walkers.max(1) as f32,
        keep: s.keep,
        crowd: crowd as u8,
        scent_coarse: false,
        blob: s.start != Start::Spread,
        respawn: s.respawn,
        blob_r: if s.start == Start::Disc { s.disc / scale as f32 } else { 0.0 },
        haze: if s.start == Start::Spread { 0.0 } else { s.haze },
        start_shape: s.start_shape,
        sense_scale: s.trail_scale,
        offset_mod: s.reach_by,
        angle_mod: [s.angle_by[0].to_radians(), s.angle_by[1]],
        turn_mod: [s.turn_by[0].to_radians(), s.turn_by[1]],
        step_mod: s.step_by,
        deposit_cap: s.cap,
        sense_floor: s.floor,
        wander: s.wander,
        sick_steps: s.sick as u16,
        immune_steps: s.immune as u16,
        sick_catch: s.catch_at,
        sick_deposit: s.germ,
        sick_keep: s.germ_keep,
        sick_speed: s.sick_speed,
        sick_trail: s.sick_trail,
        sick_reach: s.sick_reach,
        sick_turn: s.sick_turn,
        sick_upkeep: s.sick_upkeep,
        spin_curl: s.spin_curl,
        sick_seed_r: if s.seed_at <= 0.0 { s.seed_r / scale as f32 } else { 0.0 },
        sensor_r: s.sensor_r,
        species: s.fields() as u8,
        species_avoid: s.avoid,
        species_zones: s.zones,
        chance_exp: s.chance,
        species_cycle: s.cycle,
        species_next: s.next,
        species_prev: s.prev,
        explore_steps: s.explore,
        explore_deposit: s.explorer_trail,
        explore_spread: s.explore_spread,
        size_spread: s.size_spread,
        blocked_keep: s.blocked_keep,
        blocked_reset: s.blocked_reset,
        blocked_deposit: s.blocked_lay,
        species_convert: s.convert,
        graze_weight: s.graze,
        graze_eat: s.graze_eat,
        graze_regrow: s.graze_regrow,
        species_boost: s.boost,
        species_share: s.species_share,
        species_convert_r: s.convert_r,
        mix_share: s.mix_share,
        mix_reach: s.mix_reach,
        mix_angle: s.mix_angle,
        mix_turn: s.mix_turn,
        mix_step: s.mix_step,
        mix_deposit: s.mix_deposit,
        // With the global limit off the caller's `crowd` says nothing of
        // whether one can hold; `Engine::new` passes its own.
        species_sets: species_sets(s, crowd || !s.crowd),
        spore_rate: s.spore_rate,
        spore_scent: s.spore_scent,
        spore_alarm: s.spore_alarm,
        spore_burst: s.spore_burst,
        spore_steps: s.spore_steps,
        spore_speed: s.spore_speed,
        spore_reach: s.spore_reach,
        spore_deposit: s.spore_deposit,
        spore_land: s.spore_land,
        spore_rest: s.spore_rest,
        sleep_rate: s.sleep_rate,
        sleep_deposit: s.sleep_deposit,
        wake_scent: s.wake_scent,
        wake_alarm: s.wake_alarm,
        wake_steps: s.wake_steps,
        sleep_rest: s.sleep_rest,
        sleep_death: s.sleep_death,
        gas_death: s.gas_death,
        fling_speed: s.fling_speed,
        fling_deposit: s.fling_deposit,
        gas_keep: s.gas_keep,
        gas_sleep: s.gas_sleep,
        gas_linger: s.gas_linger,
        alarm_keep: s.alarm_keep,
        alarm_weight: s.alarm_weight,
        alarm_relay: s.alarm_relay,
        scar_keep: s.scar_keep,
        scar_gain: s.scar_gain,
        scar_weight: s.scar_weight,
        crust_rise: s.crust_rise,
        crust_keep: s.crust_keep,
        crust_trail: s.crust_trail,
        crust_weight: s.crust_weight,
        crust_fast: s.crust_fast,
        crust_fuel: s.crust_fuel,
        char_barren: s.char_barren,
        set_scar: s.set_scar,
        set_wake: s.set_wake,
        set_crust: s.set_crust,
        set_armour: s.set_armour,
        fire_dry: s.fire_dry,
        fire_fed: s.fire_fed,
        fire_alarm: s.fire_alarm,
        gas_burn: s.gas_burn,
        far_smell: s.far_smell,
        flinch: s.flinch,
        germ_resist: s.germ_resist,
        germ_upkeep: s.germ_upkeep,
        upkeep: s.upkeep,
        start_energy: s.start_energy,
        birth_energy: s.birth_energy,
        birth_cost: s.birth_cost,
        starve: s.starve,
        capacity: s.capacity,
        species_cap: s.species_cap.clone(),
        piles: s.piles,
        rect: s.rect,
        crystal_charge: s.crystal,
        crystal_eat: s.crystal_eat,
        crystal_flow: s.crystal_flow,
        crystal_regrow: s.crystal_regrow,
        terrain_rock: s.rock,
        terrain_wet: s.wet,
        wet_step: s.wet_step,
        terrain_blob: s.terrain_blob,
        wall_hp: s.wall_hp,
        wall_bite: s.wall_bite,
        wall_scent: s.wall_scent,
        energy_flow: s.energy_flow,
        hunger_scent: s.hunger_scent,
        hunger_at: s.hunger_at,
        wither: s.wither,
        wither_every: s.wither_every,
        wither_lag: s.wither_lag,
        wither_reach: s.wither_reach,
        panic: s.panic,
        panic_r: s.panic_r,
        panic_lean: s.panic_lean,
        panic_creep: s.panic_creep,
        panic_dash: s.panic_dash,
        condense_crowd: s.condense_crowd,
        condense_r: s.condense_r,
        roam: s.roam,
        roam_far: s.roam_far,
        roam_share: s.roam_share,
        surge_every: s.surge_every,
        surge_lean: s.surge_lean,
        surge_dash: s.surge_dash,
        surge_haste: s.surge_haste,
        surge_at: s.surge_at,
        raid_at: s.raid_at,
        raid_to: s.raid_to,
        raid_r: s.raid_r,
        raid_share: s.raid_share,
        raid_lean: s.raid_lean,
        raid_len: s.raid_len,
        raid_haste: s.raid_haste,
        lumps: s.lumps,
        scent_keep: s.scent_keep,
        fed_scent: s.fed_scent,
        scorch_keep: s.scorch_keep,
        scorch_full: s.scorch_full,
        scorch_cone: s.scorch_cone,
        // The race world's plant step, every game tick (`race_params`).
        plant_every: 1,
        seed: s.seed,
        ..t
    }
}

pub fn f32_bytes(v: &[f32]) -> Vec<u8> {
    v.iter().flat_map(|x| x.to_le_bytes()).collect()
}

pub fn u32_bytes(v: &[u32]) -> Vec<u8> {
    v.iter().flat_map(|x| x.to_le_bytes()).collect()
}

pub fn floats(bytes: &[u8]) -> Vec<f32> {
    bytes.chunks_exact(4).map(|c| f32::from_le_bytes([c[0], c[1], c[2], c[3]])).collect()
}

/// What the mouse is doing this frame, in fine cells.
#[derive(Clone, Copy, Default)]
pub struct Hand {
    pub at: Option<(f32, f32)>,
    pub pull: bool,
    pub wipe: bool,
    /// Make the walkers under the mouse sick (middle drag, or I once).
    pub infect: bool,
}

/// The shader, its layout and its ten pipelines, twice: made once, kept
/// across restarts.
pub struct Shared {
    pub bgl: wgpu::BindGroupLayout,
    /// sense, walk, trail, scent, histogram, top, draw, jump, land,
    /// convert, and the Larder World's eat, pilesum, flow, walls, build,
    /// evict, settle, unclaim.
    pub pipes: Vec<wgpu::ComputePipeline>,
    /// The same with the outside rules compiled out (`OUTSIDE` false), for
    /// a dish with all of them off.
    pub plain: Vec<wgpu::ComputePipeline>,
}

pub const BINDINGS: u32 = 49;
/// The most shot hits a frame (`Engine::set_hits`).
pub const MAX_HITS: usize = 256;
pub const SENSE: usize = 0;
pub const WALK: usize = 1;
pub const TRAIL: usize = 2;
pub const SCENT: usize = 3;
pub const HISTOGRAM: usize = 4;
pub const TOP: usize = 5;
pub const DRAW: usize = 6;
pub const JUMP: usize = 7;
pub const LAND: usize = 8;
pub const CONVERT: usize = 9;
pub const LIFE: usize = 10;
pub const BREED: usize = 11;
pub const BORN: usize = 12;
pub const EAT: usize = 13;
pub const PILESUM: usize = 14;
pub const FLOW: usize = 15;
pub const WALLS: usize = 16;
pub const BUILD: usize = 17;
pub const EVICT: usize = 18;
pub const SETTLE: usize = 19;
pub const UNCLAIM: usize = 20;
/// The squad (Larder World W2a, `set_squad`): before a frame's first tick,
/// then after its last.
pub const SHOTS: usize = 21;
pub const STRIKE: usize = 22;
pub const CUT: usize = 23;
pub const SMELL: usize = 24;
pub const UNITS: usize = 25;
pub const HARVEST: usize = 26;
/// Conversion's square (`species_convert_r`): each species' trail summed
/// along each cell's row, before `convert`.
pub const ROWS: usize = 27;
/// The life cycle (`BigParams::spore_rate` and below), after the move:
/// timers, fruiting, dormancy and waking (`fate`), then the spores' flight
/// (`fly`) and landing (`alight`). A strike (`Engine::strike`) is `hit`
/// (its cells) and `kill` (its walkers).
pub const FATE: usize = 28;
pub const FLY: usize = 29;
pub const ALIGHT: usize = 30;
pub const HIT: usize = 31;
pub const KILL: usize = 32;
/// A squad shot's alarm (`squad::SHOT_ALARM`), after `cut`, with the alarm
/// field on.
pub const SHOUT: usize = 33;
/// One body (`energy_flow`): each cell's energy and walkers summed, then
/// each walker's gain, after eating and before `life`.
pub const ESUM: usize = 34;
pub const EGAIN: usize = 35;
/// The living walkers counted into the wall counters' word 4
/// (`Engine::record_count`), for the game's packed readout.
pub const COUNT: usize = 36;
/// The squad's unit passes on one thread, as before 2026-11-12
/// (`Engine::squad_one`).
pub const SMELL1: usize = 37;
pub const UNITS1: usize = 38;
pub const HARVEST1: usize = 39;
/// The gardeners (`own_mould`, world bit `WORLD_GARDEN`): the tend entries
/// before the shots, and the player's walkers a cell counted into `gcount`
/// before the shots and before the reads.
pub const TEND: usize = 40;
pub const OWNCLEAR: usize = 41;
pub const OWNCOUNT: usize = 42;
/// A part cut off from its food withers (`Settings::wither`, binding 48):
/// each walker's news into its cell (`ngather`), spread through the body
/// `wither_reach` times (`nspread` from the gathered block, then `nab` and
/// `nba` turn about), and each walker takes its cell's (`nhear`).
pub const NGATHER: usize = 43;
pub const NSPREAD: usize = 44;
pub const NAB: usize = 45;
pub const NBA: usize = 46;
pub const NHEAR: usize = 47;
pub const CAPCLEAR: usize = 48;
pub const CAPCOUNT: usize = 49;
/// Sleep gas (in binding 48 after food's news, 11-14): `gas` puts the walkers in a disc to
/// sleep (`Engine::gas`), `calm` counts gassed sleepers near points
/// (`Engine::calm`).
pub const GAS: usize = 50;
pub const CALM: usize = 51;
pub const GASLAY: usize = 52;
pub const GASP: usize = 53;
/// An explosion's fling (11-14, `Engine::fling`).
pub const FLING: usize = 54;
/// The specks before `life` (`BigParams::roam`), and the lumps' bite a
/// cell before the frame's reads. Condensing's mass passes (`massclear`,
/// `masscount`, 56 and 57) went with it on 11-17 and the passes after
/// moved down two.
pub const RIPEN: usize = 55;
pub const DCLEAR: usize = 56;
pub const DCOUNT: usize = 57;
/// Bait laid (11-14, `Engine::lay_scent`).
pub const BAITLAY: usize = 58;
/// Plants and their fire (11-14, `mould::plants`; engine.wgsl's `plant`
/// and on).
pub const PLANT: usize = 59;
pub const FIRE: usize = 60;
pub const PCOPY: usize = 61;
pub const FIREKILL: usize = 62;
pub const IGNITE: usize = 63;
/// Lumps (held bodies, 2026-11-16; engine.wgsl's `lumpclear` and on).
pub const LUMPCLEAR: usize = 64;
pub const LUMPSUM: usize = 65;
pub const LUMPFORM: usize = 66;
/// A Mind status stamped (2026-11-16, `Engine::mind_pass`).
pub const MINDLAY: usize = 67;
/// A reader's kill (2026-11-16, `Engine::read_pass`).
pub const READKILL: usize = 68;
/// The set walkers a cell (E2b, `BigDish::set_cells`): cleared, then counted.
pub const SETCLEAR: usize = 69;
pub const SETCOUNT: usize = 70;
/// Condensed strikers (2026-11-20, `Engine::condense`): the edge facing
/// the squad condenses, then each block's leader takes what it took in.
pub const CONDENSE: usize = 71;
pub const CONDLEAD: usize = 72;
/// Every entry point, in pipeline order.
pub const PASSES: [&str; 73] = [
    "sense", "walk", "trail", "scent", "histogram", "top", "draw", "jump", "land", "convert", "life", "breed", "born", "eat", "pilesum", "flow", "walls", "build", "evict", "settle", "unclaim", "shots", "strike", "cut", "smell", "units", "harvest", "rows", "fate", "fly", "alight", "hit", "kill", "shout", "esum", "egain", "count", "smell1", "units1", "harvest1", "tend", "ownclear", "owncount", "ngather", "nspread", "nab", "nba", "nhear", "capclear", "capcount", "gas", "calm", "gaslay", "gasp", "fling", "ripen", "dclear", "dcount", "baitlay", "plant", "fire", "pcopy", "firekill", "ignite", "lumpclear", "lumpsum", "lumpform", "mindlay", "readkill", "setclear", "setcount", "condense", "condlead",
];

impl Shared {
    pub fn new(dev: &wgpu::Device) -> Self {
        let module = dev.create_shader_module(wgpu::ShaderModuleDescriptor { label: Some("sandbox"), source: wgpu::ShaderSource::Wgsl(SHADER.into()) });
        let entry = |binding: u32, ty: wgpu::BufferBindingType| wgpu::BindGroupLayoutEntry {
            binding,
            visibility: wgpu::ShaderStages::COMPUTE,
            ty: wgpu::BindingType::Buffer { ty, has_dynamic_offset: false, min_binding_size: None },
            count: None,
        };
        let rw = wgpu::BufferBindingType::Storage { read_only: false };
        let ro = wgpu::BufferBindingType::Storage { read_only: true };
        let mut entries = vec![entry(0, wgpu::BufferBindingType::Uniform)];
        for b in 1..BINDINGS {
            entries.push(entry(b, if matches!(b, 9 | 16 | 17 | 38 | 41 | 44) { ro } else { rw }));
        }
        let bgl = dev.create_bind_group_layout(&wgpu::BindGroupLayoutDescriptor { label: Some("sandbox"), entries: &entries });
        let layout = dev.create_pipeline_layout(&wgpu::PipelineLayoutDescriptor { label: Some("sandbox"), bind_group_layouts: &[Some(&bgl)], immediate_size: 0 });
        let build = |constants: &[(&str, f64)]| {
            PASSES
                .iter()
                .map(|entry_point| {
                    dev.create_compute_pipeline(&wgpu::ComputePipelineDescriptor {
                        label: Some(entry_point),
                        layout: Some(&layout),
                        module: &module,
                        entry_point: Some(entry_point),
                        compilation_options: wgpu::PipelineCompilationOptions { constants, ..Default::default() },
                        cache: None,
                    })
                })
                .collect()
        };
        Shared { bgl, pipes: build(&[]), plain: build(&[("OUTSIDE", 0.0)]) }
    }
}

/// One dish on the GPU.
pub struct Engine {
    pub device: wgpu::Device,
    pub queue: wgpu::Queue,
    /// Bind groups for even and odd ticks (trail and scent swap).
    pub groups: [wgpu::BindGroup; 2],
    pub params: wgpu::Buffer,
    pub walkers: wgpu::Buffer,
    /// Kept for the twin tests' `inject` and checks (the sandbox itself
    /// only binds them).
    pub state: wgpu::Buffer,
    pub occ: wgpu::Buffer,
    /// Agar a cell: written by walls (`build`, and `walls` when one breaks).
    pub agar: wgpu::Buffer,
    pub trail: [wgpu::Buffer; 2],
    pub scent: [wgpu::Buffer; 2],
    pub lay: wgpu::Buffer,
    pub pile: wgpu::Buffer,
    pub reached: wgpu::Buffer,
    pub frame: wgpu::Buffer,
    pub hud: wgpu::Buffer,
    /// Contagion: each walker's counter, and the germ field.
    pub sick: wgpu::Buffer,
    pub germ: [wgpu::Buffer; 2],
    /// The outside rules: each species' trail field (a block of cells a
    /// species, swapped as the trail is) and each walker's age.
    pub species: [wgpu::Buffer; 2],
    pub age: wgpu::Buffer,
    /// PR #56: grass a cell, and each walker's species.
    pub grass: wgpu::Buffer,
    pub spec: wgpu::Buffer,
    /// Growth: each slot's energy, below 0 for an empty slot; and whether
    /// growth is on (fixed for the dish's life, as the slots are).
    pub energy: wgpu::Buffer,
    pub grows: bool,
    /// What each walker on a cell took from its crystal this tick (the
    /// CPU's `took`, a cell not a slot; growth's `life` adds it).
    pub share: wgpu::Buffer,
    /// One body's sums (binding 46: energy in fixed point a cell, then
    /// walkers a cell) and whether its passes run (growth and
    /// `energy_flow` on).
    pub body: wgpu::Buffer,
    pub energy_flows: bool,
    /// Food's news (binding 48, `Settings::wither`): the step each slot
    /// last heard of food, then three blocks of cells (gathered, and the
    /// spread's two turns). `withers` while its passes run (growth and
    /// `wither` on); `reach` its spread's passes.
    pub news: wgpu::Buffer,
    /// Sleep gas, in `news` after its blocks from `gas_at` (bytes;
    /// `engine.wgsl`'s `gas_base`): a header, the calm's points and
    /// counts, then the step each slot's gas runs out at (`gas_words`).
    /// `gas_until` is the last step any gas lasts to, so `calm` costs
    /// nothing once it has passed.
    pub gas_at: u64,
    pub gas_until: u64,
    /// **Sleep wears off** (2026-11-18, `squad::BurstRules::gas_immune`):
    /// fine steps a walker that woke from gas cannot be gassed again, and
    /// while gassed no gas or cloud lengthens its sleep; 0 the old rule.
    /// Written to the gas header's word 6 with each gas and cloud laid.
    pub gas_immune: u32,
    /// **Statuses wear off** (2026-11-18, `squad::BurstRules::mind_rest`):
    /// fine steps after a Mind status ends before the same status takes
    /// that walker again; 0 the old rule. The gas header's word 7, written
    /// with each Mind status laid.
    pub mind_rest: u32,
    /// The cloud at each point of the last `calm`, in sleep levels (the
    /// points' fourth words; `Rules::doze`). Empty when `calm` read none.
    pub clouds: std::cell::RefCell<Vec<f32>>,
    /// Whether any walker has had a Mind status (`mind_pass`, or a dish
    /// loaded with one), and the last step any lasts to: the Mind row's
    /// first word (`minds_on`), so `sense` reads the words (after the
    /// gas's in `news`, `gas_words`) only while one may last. Each word
    /// carries its own end, so a run-out status reads as none either way
    /// (the slime-enemy thread's note on #403: without the end, every
    /// walker read its word every step for the rest of the planet).
    pub minds: bool,
    pub mind_until: u64,
    /// A lure's field is written and reaches somewhere
    /// (`write_lure_field`): only then is `Settings::lure_pull` sent, so
    /// with no lure standing a calm walker reads no word for it.
    pub lured: bool,
    /// The slime's wave field is written and reaches somewhere
    /// (`write_wave_field`, 2026-11-20): only then is its pull (`wave_w`)
    /// sent, in the lone step row's first word, so with no wave a calm
    /// walker reads no word for it.
    pub waved: bool,
    pub wave_w: f32,
    /// **The heartbeat** (2026-11-20, `Game::time_heart`, engine.wgsl's
    /// `heart_beat`): where it beats from (fine cells), its lean, its
    /// period in steps and its wavelength in fine cells; None off. Sent in
    /// the ground row's last two words, the lone step row's second and the
    /// fling row's second.
    pub heart: Option<Heart>,
    /// **Condensed strikers** (2026-11-20, `Game::time_strikers`,
    /// engine.wgsl's `condense`): a condensed walker's haste as it charges;
    /// None off (no walker is condensed, and the bite cells are counted
    /// only for lumps). Sent in the lump row's second word.
    pub strikers: Option<f32>,
    /// The walkers the condensed strikers held at the last `condense`
    /// (`World::mass` adds them back).
    pub striker_held: u32,
    /// A fort mask is written and marks some block (`write_fort`,
    /// 2026-11-18): only then is the fort's flag sent (Params word 888),
    /// so with no fort a walker reads no word for it.
    pub forted: bool,
    /// The gas cloud (`lay_gas`): on for good once laid (its keep is sent
    /// and `gasp` runs every tick), the step `calm` must look until, and a
    /// bound on the cloud's top at step `cloud_at`. A dish loaded with a
    /// cloud is looked at for good (`u64::MAX`).
    pub clouded: bool,
    pub cloud_until: u64,
    pub cloud_top: f32,
    pub cloud_at: u64,
    pub withers: bool,
    pub reach: u32,
    /// The dense words (specks and lumps) sit in `news` from `dense_at`
    /// (bytes; `dense_words`). `specks` runs `ripen` (`Settings::roam` with
    /// growth on). Until 11-17 `condenses` also ran it for condensing and
    /// thickening, `dense_walkers` counted condensed walkers a cell and
    /// `ripens` the mass; all three went with them.
    pub specks: bool,
    /// A lump is on (`Settings::lumps`): `lumpclear` and `lumpsum` start
    /// each tick.
    pub lumps_on: bool,
    /// A planted bug for the twin (`mouldgpu`'s
    /// `the_dense_passes_start_clean_when_they_come_back`), false in play:
    /// the next frame that counts the dense cells leaves out their clear
    /// (`dclear`), so it counts on top of whatever the last count left.
    pub plant_no_clear: bool,
    pub dense_at: u64,
    /// Plants (`plant_words`): their words sit in `news` from `plant_at`
    /// (bytes); `plants` when the dish has any, `fire_on` once lit (from
    /// then the fire passes run every plant step, as the CPU's), and a
    /// plant step every `plant_every` game ticks of `plant_sub` engine
    /// ticks each (the GPU world's `scale` fine steps a game tick).
    pub plant_at: u64,
    /// The crust (`BigDish::crust`): its block of cells sits in `news` from
    /// `crust_at` (bytes), 0 when the dish has none.
    pub crust_at: u64,
    pub plants: bool,
    pub fire_on: bool,
    /// Setting runs (`set_scar` over 0 with the crust and the scar), so the
    /// set walkers are counted a cell each game tick (`set_passes`).
    pub setting: bool,
    pub plant_every: u64,
    pub plant_sub: u64,
    /// The damage memory (binding 47, the squad toolkit's layout): the
    /// crater countdown a cell in fine steps (whole numbers in f32; the
    /// CPU's `BigDish::scorch`), then the scar a cell (the Grudge's threat
    /// memory, `BigDish::scar`; on the GPU since 2026-11-17, the trail pass
    /// keeps it and the sensors read it). `scorched` once any crater was laid: from then on the trail
    /// pass counts it down and cuts the trail under it, as the CPU's
    /// `tick` does once `scorch` is made. `crater` is the steps a shot's
    /// crater lasts this frame (`squad::Frame::crater`).
    pub crater: wgpu::Buffer,
    pub crater_steps: u32,
    pub scorched: bool,
    /// The gardeners' world (`GpuWorld::garden`): species 3 is the player's
    /// own mould, which the squad's passes leave alone, and eating runs on
    /// every cell. `tends` is this frame's tend entries (`squad::TEND`),
    /// `ntend` how many the params say.
    pub garden: bool,
    pub tends: Vec<[u32; 8]>,
    pub ntend: u32,
    /// The Larder World (engine.wgsl's bindings 34-41): ground bits, crystal
    /// charge, wall health, the bites walls took this tick, the crystal
    /// cells pile by pile, each pile's run and sums, the wall counters, and
    /// the wall being built.
    pub ground: wgpu::Buffer,
    pub charge: wgpu::Buffer,
    pub health: wgpu::Buffer,
    pub bites: wgpu::Buffer,
    pub pcell: wgpu::Buffer,
    pub piles: wgpu::Buffer,
    pub wstat: wgpu::Buffer,
    pub wb: wgpu::Buffer,
    /// Crystal cells listed (`pcell`) and the dish's piles; whether a wall
    /// has been built (the wall passes then run every tick).
    pub npc: u32,
    pub npiles: u32,
    pub walled: bool,
    /// The world bits the dish's own ground asks for (`ground_bits`): a
    /// map's rock and wet with the noise's `terrain_rock` and `terrain_wet`
    /// at 0.
    pub ground_world: u32,
    /// From the last `write_params`: whether crystal charge flows or
    /// regrows (the flow pass and its first sum then run).
    pub flows: bool,
    /// The squad (Larder World W2a): the frame's units and shots in
    /// `squad::pack`'s layout, each shot's kills, this frame's counts and
    /// slime trail (`set_squad`), and whether the next `record` runs them.
    pub sq: wgpu::Buffer,
    pub kills: wgpu::Buffer,
    pub squad: (u32, u32, f32),
    /// The squad frame's sink, squared (`squad::Frame::sink`; 0 off).
    pub shot_sink: f32,
    /// The squad frame's spray (`squad::Frame::spray`): its longest
    /// flight in fine steps (0 off), its first speed, and the share of the
    /// kills that spray (`big::spray_share_q`, of 65,536).
    pub spray: (u32, f32, u32),
    pub squad_due: bool,
    /// The squad's unit passes on one thread, as before 2026-11-12
    /// (`smell1`, `units1`, `harvest1`), to compare the two on a GPU.
    pub squad_one: bool,
    /// The squad frame a share opened and the last share will close
    /// (`record_share`).
    squad_on: bool,
    /// The mould grid's square, fine cells, that `record_count` sums (0:
    /// none), written with the params.
    pub grid: u32,
    /// This frame's shot hits (`set_hits`), and how many: the world look's
    /// stand-in fight (bindings 44, 45 after the Larder World's 32-43).
    pub hits: wgpu::Buffer,
    pub n_hits: u32,
    /// The look a cell (thickness, walkers, wound, scent; a byte each),
    /// written by the trail and draw passes, read by the view's shading.
    pub look: wgpu::Buffer,
    /// From the last `write_params`: whether the conversion pass runs, and
    /// whether any outside rule is on (else the plain pipelines run).
    pub converts: bool,
    pub boxes: bool,
    pub outside: bool,
    /// Species fields (0 for none), fixed for the dish's life.
    pub nsp: u32,
    /// Walker kinds (`mix_share`): 2 with the mix on, else 1; fixed for the
    /// dish's life. A walker's kind is bit 8 of its `spec` word, and the
    /// counts hold a block of cells a species and kind.
    pub kinds: u32,
    /// Whether the last frame had explorers on (switching them on starts
    /// every walker's stage again).
    pub exploring: bool,
    /// Whether the timed seeding (`Settings::seed_at`) has happened.
    pub seeded: bool,
    /// The CPU dish it started from: agar, piles, and the measures.
    pub dish: BigDish,
    /// Food dropped with F, in fine cells.
    pub dropped: Vec<(f32, f32)>,
    pub n: u32,
    pub cells: u32,
    /// Ticks run.
    pub done: u64,
    /// Whether the crowd limit can hold (`crowd_holds`: the start walkers
    /// at most half the agar).
    pub crowd_ok: bool,
    /// From the last `write_params`: whether the jump passes run (respawn
    /// or the pull on), and with the claim (`land`) too.
    pub jumps: bool,
    pub crowd: bool,
    /// From the last `write_params`: whether the species have their own
    /// settings (`Settings::per_species`).
    pub per: bool,
    /// The life cycle: whether the walkers' life states are on (once on,
    /// on for good: a spore holds no cell, so switching them off would
    /// lose it), and whether the dish has the alarm field (`alarm_keep`
    /// over 0 when it was made, as the CPU's `alarm`).
    pub cycles: bool,
    pub alarmed: bool,
}

/// The agar cells of the dish `make` builds from `s` (one walker placed,
/// so it costs a grid, not a mould).
pub fn agar_cells(s: &Settings, scale: usize, make: impl Fn(BigParams) -> BigDish) -> usize {
    make(BigParams { walkers: 1, ..big_params(s, scale, false) }).agar.iter().filter(|&&a| a).count()
}

/// Whether the one-walker-a-cell limit holds for `walkers` starting on a
/// dish of `agar_cells`: each start walker needs a free cell, which the
/// placement finds quickly while they are at most half the agar. Growth
/// needs no room of its own: with the limit on, a birth into a full cell
/// is refused (`born` here, `BigDish`'s births on the CPU), so the slots
/// (`capacity`) can never all be filled. Until 2026-11-12 this counted the
/// slots too (`walkers.max(capacity)`, since the growth port of 11-11),
/// which dropped the limit without a word on every race world over about
/// 7,300 walkers (the slots are 10 x walkers there) while the CPU census
/// kept it: the slime-enemy thread's 25,000 walkers on the Fork ran as
/// ropes on the GPU, a sheet with the limit held.
pub fn crowd_holds(walkers: usize, agar_cells: usize) -> bool {
    walkers <= agar_cells / 2
}

/// Whether `s` asks for a crowd limit anywhere: the dish's, or a species'
/// own (`sN.crowd`).
pub fn wants_crowd(s: &Settings) -> bool {
    species_sets(s, true).iter().any(|t| t.crowd > 0) || s.crowd
}

impl Engine {
    pub fn new(device: &wgpu::Device, queue: &wgpu::Queue, shared: &Shared, s: &Settings, scale: usize) -> Self {
        Self::made(device, queue, shared, s, scale, BigDish::new)
    }

    /// The engine on a designed world (`BigDish::with_map`, ROADMAP WM1).
    pub fn with_map(device: &wgpu::Device, queue: &wgpu::Queue, shared: &Shared, s: &Settings, scale: usize, m: &Map) -> Self {
        Self::made(device, queue, shared, s, scale, |p| BigDish::with_map(p, m))
    }

    /// The engine on the dish `make` builds from the settings' params: a
    /// designed world with its own start, such as the gardeners'
    /// (`own_mould::garden_dish`), whose walkers' species the dish sets.
    pub fn made_with(device: &wgpu::Device, queue: &wgpu::Queue, shared: &Shared, s: &Settings, scale: usize, make: impl Fn(BigParams) -> BigDish) -> Self {
        Self::made(device, queue, shared, s, scale, make)
    }

    fn made(device: &wgpu::Device, queue: &wgpu::Queue, shared: &Shared, s: &Settings, scale: usize, make: impl Fn(BigParams) -> BigDish) -> Self {
        let agar_cells = agar_cells(s, scale, &make);
        let crowd_ok = crowd_holds(s.walkers, agar_cells);
        if !crowd_ok && wants_crowd(s) {
            // Loud, since the game it plays is not the one asked for (the
            // slime-enemy thread found it silent, 2026-11-12).
            eprintln!("gpu: the one-walker-a-cell limit is OFF: {} walkers start on {} agar cells, and it holds only up to half ({})", s.walkers, agar_cells, agar_cells / 2);
        }
        let d = make(BigParams { species_sets: species_sets(s, crowd_ok), ..big_params(s, scale, s.crowd && crowd_ok) });
        let dev = device;
        let cells = (d.w * d.h) as u32;
        let n = d.walkers.len() as u32;
        let storage = wgpu::BufferUsages::STORAGE | wgpu::BufferUsages::COPY_DST | wgpu::BufferUsages::COPY_SRC;
        let buf = |label: &str, bytes: &[u8], usage: wgpu::BufferUsages| {
            note_bytes(bytes.len().max(16) as u64);
            let b = dev.create_buffer(&wgpu::BufferDescriptor { label: Some(label), size: bytes.len().max(16) as u64, usage, mapped_at_creation: false });
            queue.write_buffer(&b, 0, bytes);
            b
        };
        let zeros = |label: &str, bytes: usize| {
            note_bytes(bytes.max(16) as u64);
            dev.create_buffer(&wgpu::BufferDescriptor { label: Some(label), size: bytes.max(16) as u64, usage: storage, mapped_at_creation: false })
        };
        let params = buf("params", &[0u8; PARAMS_BYTES], wgpu::BufferUsages::UNIFORM | wgpu::BufferUsages::COPY_DST);
        let state = buf("state", &u32_bytes(&[0, 1f32.to_bits(), 0, 0]), storage);
        let flat: Vec<f32> = d.walkers.iter().flat_map(|&(x, y, h)| [x, y, h, 0.0]).collect();
        let walkers = buf("walkers", &f32_bytes(&flat), storage);
        let intent = zeros("intent", n as usize * 16);
        let icell = zeros("icell", n as usize * 4);
        let occ = buf("occ", &u32_bytes(&occupancy(&d)), storage);
        let claim = buf("claim", &u32_bytes(&vec![u32::MAX; cells as usize]), storage);
        // A block of counts a species and kind (`dep`, `sdep`), two for
        // explorers' and blocked walkers' classes (`edep`, `bdep`).
        let nsp = s.fields();
        let kinds = if d.kind.is_empty() { 1 } else { 2 };
        let blocks = nsp.max(1) as usize * kinds as usize;
        let dep = zeros("dep", blocks * cells as usize * 4);
        let agar: Vec<u32> = d.agar.iter().map(|&a| a as u32).collect();
        let agar = buf("agar", &u32_bytes(&agar), storage);
        let lay = zeros("lay", cells as usize * 4);
        let pile = zeros("pile", cells as usize * 4);
        let mut reached = vec![0u32; MAX_PILES];
        for (r, &hit) in reached.iter_mut().zip(&d.reached) {
            *r = hit as u32;
        }
        let reached = buf("reached", &u32_bytes(&reached), storage);
        let trail = [zeros("trail0", cells as usize * 4), zeros("trail1", cells as usize * 4)];
        let scent = [zeros("scent0", cells as usize * 4), zeros("scent1", cells as usize * 4)];
        let hist = zeros("hist", 512 * 4);
        let frame = zeros("frame", cells as usize * 4);
        let hud = zeros("hud", cells as usize * 4);
        // Contagion's buffers are always there (a u32 a walker and three
        // fields), so it can be switched on as the dish runs.
        let counters: Vec<u32> = if d.sick.is_empty() { vec![0; n as usize] } else { d.sick.iter().map(|&c| c as u32).collect() };
        let sick = buf("sick", &u32_bytes(&counters), storage);
        let sdep = zeros("sdep", blocks * cells as usize * 4);
        // The life cycle's share of them (always there, as a strike can
        // switch it on): the germ fields' second block is the alarm,
        // `gdep`'s second and third the relays and the dormant a cell, and
        // `bdep`'s third and fourth halves spores' and sleepers' deposits.
        let gdep = zeros("gdep", 3 * cells as usize * 4);
        // The third block (11-14) is the gas cloud.
        let germ = [zeros("germ0", 3 * cells as usize * 4), zeros("germ1", 3 * cells as usize * 4)];
        let edep = zeros("edep", 2 * blocks * cells as usize * 4);
        let bdep = zeros("bdep", 4 * blocks * cells as usize * 4);
        let species = [zeros("species0", nsp as usize * cells as usize * 4), zeros("species1", nsp as usize * cells as usize * 4)];
        let age = zeros("age", n as usize * 4);
        // Grass starts full on the agar (the CPU's, made only with grazing
        // on; here always, so grazing can be switched on as the dish runs);
        // each walker's species and kind are the CPU dish's.
        let grass = buf("grass", &f32_bytes(&grass_of(&d)), storage);
        let gcount = zeros("gcount", cells as usize * 4);
        let spec = buf("spec", &u32_bytes(&spec_words(&d)), storage);
        let energy = buf("energy", &f32_bytes(&energies(&d)), storage);
        let share = zeros("share", cells as usize * 4);
        // The Larder World's fields, always there (a wall can go up on any
        // dish); the crystal list only as long as the dish's crystals.
        let wd = world_data(&d);
        let ground = buf("ground", &u32_bytes(&wd.ground), storage);
        let charge = buf("charge", &f32_bytes(&wd.charge), storage);
        let health = buf("health", &f32_bytes(&wd.health), storage);
        let bites = zeros("bites", cells as usize * 4);
        let pcell = buf("pcell", &u32_bytes(&wd.pcell), storage);
        let piles = buf("piles", &u32_bytes(&wd.piles), storage);
        // Bites, broken cells, the eviction's moves, the struck, and the
        // living walkers (`record_count`); three words spare; then the
        // mould grid, for squares down to a dish cell.
        let g = d.p.scale.clamp(1, GRID_MIN);
        let grid_max = d.w.div_ceil(g) * d.h.div_ceil(g);
        let wstat = zeros("wstat", WSTAT_BYTES + 4 * grid_max);
        let wb = zeros("wall build", 48);
        let sq = zeros("squad", squad::ALL * 4);
        let kills = zeros("kills", squad::SHOTS_MAX * 4);
        let hits = zeros("hits", MAX_HITS * 16);
        let look = zeros("look", cells as usize * 4);
        let body = zeros("body", 2 * cells as usize * 4);
        let crater = zeros("crater", 2 * cells as usize * 4);
        // Food's news, then the sleep gas's words (no binding of their
        // own: the device's 48 storage buffers a stage are all bound).
        let news_len = news_words(&d, cells as usize).len();
        let gas_len = gas_words(&d).len();
        let dense_len = dense_words(&d, cells as usize).len();
        let plant_len = plant_words(&d).len();
        // The crust's block (`BigDish::crust`) after the plants', when the
        // dish has one.
        // Each slot's set word (`BigDish::set`) right after it.
        let crust_words: Vec<u32> = d.crust.iter().map(|v| v.to_bits()).chain(d.set.iter().copied()).chain(d.set_cells.iter().copied()).collect();
        let news = buf("news", &u32_bytes(&[news_words(&d, cells as usize), gas_words(&d), dense_words(&d, cells as usize), plant_words(&d), crust_words].concat()), storage);
        let group = |even: bool| {
            let (a, b) = if even { (0, 1) } else { (1, 0) };
            let bufs: [&wgpu::Buffer; BINDINGS as usize] = [
                &params, &state, &walkers, &intent, &icell, &occ, &claim, &dep, &agar, &lay, &trail[a], &trail[b], &scent[a], &scent[b], &hist, &frame, &hud, &pile, &reached, &sick, &sdep, &gdep, &germ[a], &germ[b], &edep, &bdep, &species[a],
                &species[b], &age, &grass, &gcount, &spec, &energy, &share, &ground, &charge, &health, &bites, &pcell, &piles, &wstat, &wb, &sq, &kills, &hits, &look, &body, &crater, &news,
            ];
            let entries: Vec<wgpu::BindGroupEntry> = bufs.iter().enumerate().map(|(i, b)| wgpu::BindGroupEntry { binding: i as u32, resource: b.as_entire_binding() }).collect();
            dev.create_bind_group(&wgpu::BindGroupDescriptor { label: Some("sandbox"), layout: &shared.bgl, entries: &entries })
        };
        let groups = [group(true), group(false)];
        let (cycles, alarmed) = (!d.life.is_empty(), !d.alarm.is_empty());
        let mut e = Engine {
            device: device.clone(),
            queue: queue.clone(),
            groups,
            params,
            walkers,
            state,
            occ,
            agar,
            trail,
            scent,
            lay,
            pile,
            reached,
            frame,
            hud,
            sick,
            germ,
            species,
            age,
            grass,
            spec,
            grows: !d.energy.is_empty(),
            energy,
            share,
            body,
            energy_flows: false,
            news,
            gas_at: 4 * news_len as u64,
            gas_until: gas_until(&d),
            gas_immune: 0,
            mind_rest: 0,
            clouds: Default::default(),
            minds: !d.mind.is_empty(),
            lured: false,
            waved: false,
            wave_w: 0.0,
            heart: None,
            strikers: None,
            striker_held: 0,
            forted: false,
            mind_until: mind_until(&d),
            clouded: false,
            cloud_until: 0,
            cloud_top: 0.0,
            cloud_at: 0,
            withers: false,
            reach: 1,
            specks: false,
            lumps_on: false,
            plant_no_clear: false,
            dense_at: 4 * (news_len + gas_len) as u64,
            plant_at: 4 * (news_len + gas_len + dense_len) as u64,
            crust_at: if d.crust.is_empty() { 0 } else { 4 * (news_len + gas_len + dense_len + plant_len) as u64 },
            plants: !d.plant.is_empty(),
            fire_on: !d.fire.is_empty(),
            setting: false,
            plant_every: d.p.plant_every.max(1) as u64,
            plant_sub: d.p.substeps.max(1) as u64,
            crater,
            crater_steps: 0,
            scorched: false,
            garden: false,
            tends: Vec::new(),
            ntend: 0,
            ground,
            charge,
            health,
            bites,
            pcell,
            piles,
            wstat,
            wb,
            npc: wd.pcell.len() as u32,
            npiles: wd.npiles,
            walled: !d.wall.is_empty(),
            ground_world: ground_bits(&d),
            flows: false,
            sq,
            kills,
            squad: (0, 0, 0.0),
            shot_sink: 0.0,
            spray: (0, 0.0, 65536),
            squad_due: false,
            squad_one: false,
            squad_on: false,
            grid: 0,
            hits,
            n_hits: 0,
            look,
            converts: false,
            boxes: false,
            outside: false,
            nsp,
            kinds,
            exploring: s.explores(),
            seeded: false,
            dish: d,
            dropped: Vec::new(),
            n,
            cells,
            done: 0,
            crowd_ok,
            jumps: false,
            crowd: false,
            per: false,
            cycles,
            alarmed,
        };
        e.lay_food(s.food);
        e
    }

    /// Writes the scent the piles lay at full strength, and which pile each
    /// cell's food is: the dish's own if `food`, plus any dropped with F.
    /// The scent pass lays `fed_scent` of it once a pile is reached (the
    /// CPU's `lay_scent`); dish piles are marked even with `food` off, for
    /// the measures. Piles sit 28 dish cells apart, so their discs never
    /// overlap; dropped food that overlaps a pile takes its cells.
    pub fn lay_food(&mut self, food: bool) {
        let d = &self.dish;
        let mut lay = vec![0.0f32; d.w * d.h];
        let mut pile = vec![0u32; d.w * d.h];
        let s = d.p.scale as f32;
        let dish = d.piles.iter().zip(&d.pile_rs).map(|(&(x, y), &r)| (x * s, y * s, r * s, food));
        let dropped = self.dropped.iter().map(|&(x, y)| (x, y, d.p.pile_r * s, true));
        for (pi, (cx, cy, r, lays)) in dish.chain(dropped).enumerate().take(MAX_PILES) {
            let n = r.ceil() as i32;
            for dy in -n..=n {
                for dx in -n..=n {
                    let (x, y) = (cx + dx as f32, cy + dy as f32);
                    // A crystal's footprint is its hexagon (`BigDish::in_pile`).
                    if !d.in_pile(dx as f32, dy as f32, r) || x < 0.0 || y < 0.0 || x >= d.w as f32 || y >= d.h as f32 {
                        continue;
                    }
                    // Every cell of the footprint: the scent pass lays only
                    // where scent spreads (agar, or a wall that lets it
                    // through), as the CPU's `lay_scent` lays on its
                    // `scent_agar`, so rubble a broken wall leaves lays again.
                    let i = y as usize * d.w + x as usize;
                    if lays {
                        lay[i] += d.p.food_deposit;
                    }
                    pile[i] = pi as u32 + 1;
                }
            }
        }
        self.queue.write_buffer(&self.lay, 0, &f32_bytes(&lay));
        self.queue.write_buffer(&self.pile, 0, &u32_bytes(&pile));
    }

    /// Whether F can drop another pile.
    pub fn can_drop(&self) -> bool {
        self.dish.piles.len() + self.dropped.len() < MAX_PILES
    }

    /// Loads a CPU dish's state (walkers, trail, scent, the scent its piles
    /// lay, and its step for the keyed draws) in place of the GPU's, for the
    /// twin test: one GPU tick from it should match one `BigDish::tick`.
    /// The dish must have this engine's settings.
    pub fn inject(&mut self, d: &BigDish) {
        assert_eq!((d.w, d.h, d.walkers.len() as u32, d.sw, d.sh), (self.dish.w, self.dish.h, self.n, d.w, d.h), "a dish of this engine's size, on the fine scent grid");
        let flat: Vec<f32> = d.walkers.iter().flat_map(|&(x, y, h)| [x, y, h, 0.0]).collect();
        self.queue.write_buffer(&self.walkers, 0, &f32_bytes(&flat));
        self.queue.write_buffer(&self.occ, 0, &u32_bytes(&occupancy(d)));
        assert_eq!(d.energy.is_empty(), !self.grows, "a dish with this engine's growth");
        self.queue.write_buffer(&self.energy, 0, &f32_bytes(&energies(d)));
        // The tick's bind group reads side `done % 2`.
        let side = (d.steps % 2) as usize;
        self.queue.write_buffer(&self.trail[side], 0, &f32_bytes(&d.trail));
        self.queue.write_buffer(&self.scent[side], 0, &f32_bytes(&d.scent));
        self.queue.write_buffer(&self.state, 0, &u32_bytes(&[d.steps as u32, 1f32.to_bits(), 0, 0]));
        let mut reached = vec![0u32; MAX_PILES];
        for (r, &hit) in reached.iter_mut().zip(&d.reached) {
            *r = hit as u32;
        }
        self.queue.write_buffer(&self.reached, 0, &u32_bytes(&reached));
        let counters: Vec<u32> = if d.sick.is_empty() { vec![0; self.n as usize] } else { d.sick.iter().map(|&c| c as u32).collect() };
        self.queue.write_buffer(&self.sick, 0, &u32_bytes(&counters));
        let mut germ = if d.germ.is_empty() { vec![0.0; self.cells as usize] } else { d.germ.clone() };
        germ.extend(if d.alarm.is_empty() { vec![0.0; self.cells as usize] } else { d.alarm.clone() });
        germ.extend(if d.gas_cloud.is_empty() { vec![0.0; self.cells as usize] } else { d.gas_cloud.clone() });
        self.queue.write_buffer(&self.germ[side], 0, &f32_bytes(&germ));
        // The scar (the damage memory's second block), so a twin tick
        // starts from the CPU's (2026-11-17: the Grudge on the GPU).
        let scar = if d.scar.is_empty() { vec![0.0; self.cells as usize] } else { d.scar.clone() };
        self.queue.write_buffer(&self.crater, self.cells as u64 * 4, &f32_bytes(&scar));
        // The life states come with the spec words below; a dish that has
        // them switches them on here.
        self.cycles |= !d.life.is_empty();
        self.alarmed |= !d.alarm.is_empty();
        // The cloud's linger is in the gas words below (`gas_words`).
        if !d.gas_cloud.is_empty() {
            self.clouded = true;
            self.cloud_until = u64::MAX;
        }
        assert_eq!(d.species_trail.len(), self.nsp as usize, "a dish with this engine's species");
        if self.nsp > 0 {
            let fields: Vec<f32> = d.species_trail.iter().flatten().copied().collect();
            self.queue.write_buffer(&self.species[side], 0, &f32_bytes(&fields));
        }
        let ages: Vec<u32> = if d.age.is_empty() { vec![0; self.n as usize] } else { d.age.iter().map(|&a| a as u32).collect() };
        self.queue.write_buffer(&self.age, 0, &u32_bytes(&ages));
        self.exploring = !d.age.is_empty();
        self.queue.write_buffer(&self.grass, 0, &f32_bytes(&grass_of(d)));
        // The Larder World: the agar too, since walls change it.
        let agar: Vec<u32> = d.agar.iter().map(|&a| a as u32).collect();
        self.queue.write_buffer(&self.agar, 0, &u32_bytes(&agar));
        let wd = world_data(d);
        assert_eq!(wd.pcell.len() as u32, self.npc, "a dish with this engine's crystals");
        self.queue.write_buffer(&self.ground, 0, &u32_bytes(&wd.ground));
        self.queue.write_buffer(&self.charge, 0, &f32_bytes(&wd.charge));
        self.queue.write_buffer(&self.health, 0, &f32_bytes(&wd.health));
        self.queue.write_buffer(&self.pcell, 0, &u32_bytes(&wd.pcell));
        self.queue.write_buffer(&self.piles, 0, &u32_bytes(&wd.piles));
        self.queue.write_buffer(&self.bites, 0, &vec![0u8; self.cells as usize * 4]);
        self.queue.write_buffer(&self.wstat, 0, &[0u8; 16]);
        self.walled = !d.wall.is_empty();
        self.ground_world = ground_bits(d);
        assert_eq!(d.kind.is_empty(), self.kinds == 1, "a dish with this engine's mix");
        self.queue.write_buffer(&self.spec, 0, &u32_bytes(&spec_words(d)));
        self.queue.write_buffer(&self.news, 0, &u32_bytes(&news_words(d, self.cells as usize)));
        self.queue.write_buffer(&self.news, self.gas_at, &u32_bytes(&gas_words(d)));
        self.queue.write_buffer(&self.news, self.dense_at, &u32_bytes(&dense_words(d, self.cells as usize)));
        // The raid's way round walls goes with it (`raid_row` reads
        // whether the mirror holds one).
        self.dish.raid_field = d.raid_field.clone();
        self.dish.units_field = d.units_field.clone();
        self.dish.food_field = d.food_field.clone();
        assert_eq!(d.plant.is_empty(), !self.plants, "a dish with this engine's plants");
        if self.plants {
            self.queue.write_buffer(&self.news, self.plant_at, &u32_bytes(&plant_words(d)));
        }
        assert_eq!(d.crust.is_empty(), self.crust_at == 0, "a dish with this engine's crust");
        if self.crust_at > 0 {
            self.queue.write_buffer(&self.news, self.crust_at, &f32_bytes(&d.crust));
            self.queue.write_buffer(&self.news, self.crust_at + 4 * self.cells as u64, &u32_bytes(&d.set));
            self.queue.write_buffer(&self.news, self.crust_at + 4 * (self.cells + self.n) as u64, &u32_bytes(&d.set_cells));
        }
        self.fire_on = !d.fire.is_empty();
        self.gas_until = gas_until(d);
        self.minds = !d.mind.is_empty();
        self.mind_until = mind_until(d);
        // The dense words above cleared the lure's blocks (a CPU dish has
        // none); the game writes them again within a game second. The
        // fort's mask the same (`Game::time_fort` writes it again).
        self.lured = false;
        self.waved = false;
        self.forted = false;
        self.withers = self.grows && (d.p.wither > 0.0 || d.p.panic > 0.0);
        self.specks = self.grows && d.p.roam > 0;
        self.done = d.steps;
        self.dish = d.clone();
        self.dropped.clear();
        self.lay_food(true);
    }

    /// Writes the controls for the next frame of `ticks` ticks.
    pub fn write_params(&mut self, s: &Settings, hand: Hand, ticks: u32) {
        let seed = s.seed;
        let (w, h) = (self.dish.w as u32, self.dish.h as u32);
        let crowd = (s.crowd && self.crowd_ok) as u32;
        // Per walker as the CPU's `big_params`: the starting walkers, not
        // the slots (they are the same with growth off).
        let deposit = s.laid * 100_000.0 / s.walkers.max(1) as f32;
        let (bx, by) = hand.at.unwrap_or((-1.0e6, -1.0e6));
        let br = 6.0 * self.dish.p.scale as f32;
        let pull = if hand.pull && hand.at.is_some() { PULL } else { 0.0 };
        let wipe = (hand.wipe && hand.at.is_some()) as u32;
        let t = &self.dish.p;
        let mut b = u32_bytes(&[w, h, self.n, crowd, stream(seed, DRAW_TURN), stream(seed, DRAW_BLOCKED), stream(seed, CLAIM_MOVE), stream(seed, DRAW_JUMP)]);
        // Food smelled at the reach when `smell` is 0, as `BigParams::scent_offset`.
        let smell = if s.smell > 0.0 { s.smell } else { s.reach };
        b.extend(f32_bytes(&[s.sensor_angle.to_radians(), s.turn.to_radians(), s.reach, smell, s.step, deposit, s.keep, t.scent_keep]));
        b.extend(f32_bytes(&[s.respawn, pull, bx, by, br, s.knee, s.gamma]));
        b.extend(u32_bytes(&[stream(seed, DRAW_PULL), wipe, (s.colour == Colour::Grey) as u32, (s.field == Field::Walkers) as u32, (s.curve == Curve::Soft) as u32]));
        b.extend(f32_bytes(&[t.tube, t.fed_scent]));
        b.extend(u32_bytes(&[s.cap as u32, stream(seed, DRAW_WANDER)]));
        // The looks: angles in radians, as `BigParams`.
        let (am, tm) = ([s.angle_by[0].to_radians(), s.angle_by[1]], [s.turn_by[0].to_radians(), s.turn_by[1]]);
        // The catch level times the tier's germ resistance (2026-11-17),
        // the CPU's `sick_catch * germ_resist`; the dish's own draw then
        // shows the germ in red against that mould's level.
        b.extend(f32_bytes(&[s.trail_scale, s.reach_by[0], s.reach_by[1], am[0], am[1], tm[0], tm[1], s.step_by[0], s.step_by[1], s.floor, s.wander, s.catch_at * s.germ_resist]));
        // Infect: the mouse's disc, or the middle once `seed_at` has come.
        let (mut infect, mut ix, mut iy, mut ir) = (0u32, 0.0f32, 0.0f32, 0.0f32);
        if let (true, Some((x, y))) = (hand.infect, hand.at) {
            (infect, ix, iy, ir) = (1, x, y, br);
        } else if s.sick_on() && s.seed_at > 0.0 && !self.seeded && ticks > 0 && self.done as f32 >= s.seed_at * STEPS_PER_SECOND as f32 {
            (infect, ix, iy, ir) = (1, w as f32 / 2.0, h as f32 / 2.0, s.seed_r);
            self.seeded = true;
        }
        // The cloud's keep goes only once a cloud is laid (`clouded`), so
        // no tick spreads an empty one.
        let cloud_keep = if self.clouded { s.gas_keep } else { 0.0 };
        b.extend(u32_bytes(&[s.sick, s.immune, infect, cloud_keep.to_bits()]));
        // What the sick pay times the tier's frailty (2026-11-17), the
        // CPU's `sick_upkeep * germ_upkeep`.
        b.extend(f32_bytes(&[s.germ, s.germ_keep, s.sick_speed, s.sick_trail, s.sick_reach, s.sick_turn, ix, iy, ir, s.sick_upkeep * s.germ_upkeep, s.gas_death, s.step * s.fling_speed]));
        // The outside rules. Switching explorers on starts every walker's
        // stage again, so the switch shows (ages are only counted while on).
        if s.explores() && !self.exploring {
            self.queue.write_buffer(&self.age, 0, &vec![0u8; self.n as usize * 4]);
        }
        self.exploring = s.explores();
        // Each species' own settings (`species_sets`). With them the global
        // explorer stage, blocked share and grazing weight below are only
        // switches (on if any species has one); each walker reads its own
        // species' in the species block at the end.
        let sets = species_sets(s, self.crowd_ok);
        let any = |f: fn(&SpeciesSet) -> f32| sets.iter().map(f).find(|&v| v != 0.0).unwrap_or(0.0);
        let (explore, blocked, graze) = if sets.is_empty() {
            (s.explore as u32, s.blocked_lay, s.graze)
        } else {
            (sets.iter().map(|t| t.explore_steps as u32).max().unwrap_or(0), sets.iter().map(|t| t.blocked_deposit).fold(0.0, f32::max), any(|t| t.graze_weight))
        };
        b.extend(u32_bytes(&[self.nsp, s.cycle as u32, explore, stream(seed, DRAW_SIZE)]));
        b.extend(f32_bytes(&[s.avoid, s.next, s.prev, s.explorer_trail, s.explore_spread, blocked]));
        b.extend(u32_bytes(&[s.blocked_keep as u32]));
        b.extend(f32_bytes(&[s.blocked_reset, s.chance, s.size_spread]));
        b.extend(u32_bytes(&[s.sensor_r as u32, stream(seed, DRAW_CHANCE), stream(seed, DRAW_KEEP)]));
        b.extend(f32_bytes(&[s.convert, graze, s.graze_eat, s.graze_regrow, s.boost]));
        // Growth: its upkeep as the dish was made with (off stays off).
        let upkeep = if self.grows { s.upkeep.max(f32::MIN_POSITIVE) } else { 0.0 };
        b.extend(f32_bytes(&[upkeep, s.birth_energy, s.birth_cost, s.starve]));
        let mask = self.n.max(2).next_power_of_two() - 1;
        b.extend(u32_bytes(&[stream(seed, DRAW_STARVE), stream(seed, DRAW_BIRTH), stream(seed, CLAIM_BIRTH), mask]));
        // Withering's chance and draw borrow the growth row's two spare
        // words (360, 364; the slime-enemy thread, 2026-11-13).
        b.extend(f32_bytes(&[s.wither]));
        b.extend(u32_bytes(&[stream(seed, DRAW_WITHER)]));
        // The Larder World: what the dish was made with (crystals, terrain,
        // full charge and health), what has been built (walls), and the
        // rates as set now.
        let mut world = 0;
        if self.npc > 0 {
            world |= WORLD_CRYSTALS;
        }
        if t.terrain_wet > 0.0 || self.walled {
            world |= WORLD_WETS;
        }
        if self.walled {
            world |= WORLD_WALLS;
        }
        if t.terrain_rock > 0.0 || t.terrain_wet > 0.0 {
            world |= WORLD_TERRAIN;
        }
        world |= self.ground_world;
        if self.garden {
            world |= WORLD_GARDEN;
        }
        if self.plants {
            world |= WORLD_PLANTS;
        }
        b.extend(u32_bytes(&[world, self.npiles, self.npc, self.grid]));
        b.extend(f32_bytes(&[t.crystal_charge, s.crystal_eat, s.crystal_flow, s.crystal_regrow, s.wet_step, s.wall_bite, s.wall_scent, t.wall_hp]));
        // Real asserts, not debug ones (2026-11-12, game integration): six
        // threads append rows here, CI and the census run release builds,
        // and a row that slips moves every offset after it without a
        // failing test. Three length checks a frame cost nothing measurable.
        // New rows go after the soil row, in the merge order in ROADMAP.md
        // ("Larder World: shared rules and merge order").
        assert_eq!(b.len(), SQUAD_AT, "Params rows before the squad row moved");
        b.extend(u32_bytes(&[self.squad.0, self.squad.1, self.squad.2.to_bits(), squad::SHOT_ALARM.to_bits()]));
        b.extend(u32_bytes(&[self.n_hits, self.ntend, (squad::TEND - squad::PATH) as u32, self.shot_sink.to_bits()]));
        // The species block (2026-11-11): whether each species has its own
        // settings, the walker kinds, conversion's square; the second
        // kind's multipliers; then a set a species (`Set` in engine.wgsl),
        // each with its scent reach as the CPU takes it (`scent_offset`, or
        // the species' own sensor reach when that is 0).
        // A cut-off part's panic borrows the row's spare words 460 (the
        // charge's squared radius, fine cells, as the CPU's
        // `panic_target`) and 484 (the haste; 0 off, and off without
        // growth, as no news runs then), 488 the lean's pace (`panic_creep`,
        // clamped as the CPU's) and 492 the dash (`panic_dash`, steps, a
        // u32): the slime-enemy thread, 11-13.
        let pr = s.panic_r * self.dish.p.scale as f32;
        let panic_r2 = (pr.max(0.0) * pr.max(0.0)).floor() as u32;
        let panic = if self.grows { s.panic } else { 0.0 };
        b.extend(u32_bytes(&[!sets.is_empty() as u32, self.kinds, s.convert_r, panic_r2]));
        b.extend(f32_bytes(&[s.mix_reach, s.mix_angle, s.mix_turn, s.mix_step, s.mix_deposit, panic, s.panic_creep.max(crate::mould::big::PANIC_CREEP_MIN)]));
        b.extend(u32_bytes(&[s.panic_dash.min(crate::mould::big::PANIC_DASH_MAX)]));
        // Each species' birth cap borrows its set's spare word (+44, `cap`;
        // the player's own mould thread, 11-14): u32::MAX is none.
        let cap_of = |sp: usize| s.species_cap.get(sp).map_or(u32::MAX, |&c| c.min(u32::MAX as usize - 1) as u32);
        for sp in 0..MAX_SPECIES as usize {
            let Some(t) = sets.get(sp) else {
                b.extend([0u8; 44]);
                b.extend(u32_bytes(&[cap_of(sp)]));
                continue;
            };
            let sso = if s.smell > 0.0 { s.smell } else { t.sensor_offset };
            b.extend(f32_bytes(&[t.sensor_angle, t.sensor_offset, t.rotate_angle, t.step, sso, t.deposit, t.keep, t.blocked_deposit, t.graze_weight]));
            b.extend(u32_bytes(&[t.crowd as u32, t.explore_steps as u32, cap_of(sp)]));
        }
        // The life cycle: on for good once spores or dormancy are, as the
        // CPU's states (a strike switches it on too); the alarm field once
        // `alarm_keep` is over 0, cleared when it comes (on the CPU it is
        // made with the dish).
        self.cycles |= s.spore_rate > 0.0 || s.spore_burst > 0.0 || s.sleep_rate > 0.0;
        if s.alarm_keep > 0.0 && !self.alarmed {
            for g in &self.germ {
                self.queue.write_buffer(g, self.cells as u64 * 4, &vec![0u8; self.cells as usize * 4]);
            }
            self.alarmed = true;
        }
        b.extend(u32_bytes(&[self.cycles as u32, self.alarmed as u32, stream(seed, DRAW_FRUIT), stream(seed, DRAW_SLEEP)]));
        b.extend(u32_bytes(&[stream(seed, CLAIM_LAND), stream(seed, DRAW_SPORE_H), s.spore_steps as u32, s.spore_rest as u32, s.wake_steps as u32, s.sleep_rest as u32, stream(seed, DRAW_SLEEP_DIE), wither_clock(s)]));
        b.extend(f32_bytes(&[s.spore_rate, s.spore_scent, s.spore_alarm, s.spore_burst, s.step * s.spore_speed, s.spore_reach, s.spore_deposit, s.spore_land]));
        b.extend(f32_bytes(&[s.sleep_rate, s.sleep_deposit, s.wake_scent, s.wake_alarm, s.alarm_keep, s.alarm_weight, s.alarm_relay, s.sleep_death]));
        assert_eq!(b.len(), STRIKE_AT, "Params rows before the strike row moved");
        b.extend([0u8; 32]);
        // Soil (WM3, 2026-11-12): the dish's own rates, as the CPU reads
        // them from its `BigParams` (read only with `WORLD_SOIL`). The last
        // word (f32, byte 844) is the lure's pull (`Settings::lure_pull`,
        // 2026-11-18, the Enemy lane), 0 until a lure's field is written
        // (`lured`). It was one body's lossy sharing (`eloss`, `flow_loss`)
        // from 11-13 until the GPU's half of that rule was deleted on 11-17
        // (`Reports/dead-ends.md`).
        b.extend(f32_bytes(&[t.sand_keep, t.clay_keep, t.barren_upkeep, if self.lured { s.lure_pull } else { 0.0 }]));
        assert_eq!(b.len(), BODY_AT, "Params rows before the one body row moved");
        // The row's last word is the far smell (`far_smell`, S2, 11-17; it
        // held `fed_full`, then heat in the sensors, `fire_weight`, both
        // deleted 11-17).
        b.extend(f32_bytes(&[s.energy_flow, s.hunger_scent, s.hunger_at, s.far_smell]));
        assert_eq!(b.len(), CRATER_AT, "Params rows before the crater row moved");
        b.extend(f32_bytes(&[t.scorch_keep, t.scorch_full.max(1) as f32, self.crater_steps as f32]));
        b.extend(u32_bytes(&[self.crater_flags()]));
        assert_eq!(b.len(), LONE_AT, "Params rows before the lone step row moved");
        // The lone step's row, free since 11-17 (`LONE_AT`): its first
        // word the wave's pull (`waved`, 2026-11-20), its third the fort's
        // flag (`forted`, 11-18), its last the tether since 11-18
        // (`Settings::tether`).
        let wave = if self.waved { self.wave_w } else { 0.0 };
        let heart = self.heart.map_or(0.0, |h| h.lean);
        b.extend(u32_bytes(&[wave.to_bits(), heart.to_bits(), self.forted as u32, s.tether]));
        assert_eq!(b.len(), FLING_AT, "Params rows before the fling row moved");
        // The fling row's spare words, condensing's (11-14, the condense
        // thread): the first is free since 11-17 (`condense`, deleted), then
        // the specks' ripe crowd and radius (`ccrowd`: bits 0-15 and 16-23;
        // 24-31 were `thicken`'s, free since 11-17) and their draw's stream.
        let r = s.condense_r.clamp(1, 8) as u32;
        assert!(r <= 0xFF, "condense_r fits ccrowd's bits 16-23");
        b.extend(f32_bytes(&[s.fling_deposit]));
        // The first is the heartbeat's clock (2026-11-20): its period in
        // steps and its wavelength in fine cells, 16 bits each.
        let clock = self.heart.map_or(0, |h| h.period.min(0xFFFF) | (h.len.min(0xFFFF) << 16));
        b.extend(u32_bytes(&[clock, s.condense_crowd.min(0xFFFF) | (r << 16), stream(seed, DRAW_ROAM)]));
        // The breakaway row (11-14, the condense thread; Params grew from
        // 912 to 928 for it): steps between chances to leave (0 off), how
        // far the crystal must be and how near a landing is, in whole fine
        // cells as the CPU's `roam_pick` counts them, and the share sent.
        assert_eq!(b.len(), ROAM_AT, "Params rows before the breakaway row moved");
        let sc = self.dish.p.scale as f32;
        // Any slime may break away (11-16, the CPU's `roam_of`); `ripen`
        // runs for it.
        let roam = if self.grows { s.roam.min(crate::mould::big::RIPE) } else { 0 };
        self.specks = roam > 0;
        let far = (s.roam_far.max(0.0) * sc).floor() as u32;
        let land = (crate::mould::big::ROAM_LAND * sc).floor() as u32;
        b.extend(u32_bytes(&[roam, far, land]));
        b.extend(f32_bytes(&[s.roam_share]));
        assert_eq!(b.len(), GROUND_AT, "Params rows before the ground row moved");
        // Its second word is deep water as a wall (`Settings::deep_wall`,
        // 2026-11-20).
        b.extend(f32_bytes(&[t.deep_step]));
        b.extend(u32_bytes(&[s.deep_wall as u32]));
        // The last two, the heartbeat's source (2026-11-20).
        let at = self.heart.map_or((0.0, 0.0), |h| h.at);
        b.extend(f32_bytes(&[at.0, at.1]));
        assert_eq!(b.len(), SURGE_AT, "Params rows before the surge row moved");
        let sm = crate::mould::big::SURGE_MAX;
        b.extend(u32_bytes(&[s.surge_every, s.surge_lean.min(sm) | s.surge_dash.min(sm) << 16, s.surge_at]));
        b.extend(f32_bytes(&[s.surge_haste]));
        assert_eq!(b.len(), SPRAY_AT, "Params rows before the spray row moved");
        b.extend(u32_bytes(&self.spray_row()));
        assert_eq!(b.len(), RAID_AT, "Params rows before the raid row moved");
        b.extend(u32_bytes(&raid_row(s, self.dish.p.scale as f32, !self.dish.raid_field.is_empty())));
        assert_eq!(b.len(), LUMP_AT, "Params rows before the lump rows moved");
        self.lumps_on = s.lumps.iter().any(crate::mould::big::Lump::on);
        // The condense pass's reach and share are written by `condense`
        // alone; a step reads only the haste.
        b.extend(u32_bytes(&[(self.lumps_on || self.strikers.is_some()) as u32, self.strikers.unwrap_or(0.0).to_bits(), 0, 0]));
        for l in &s.lumps {
            b.extend(u32_bytes(&[l.aim, l.pull.to_bits(), l.leash_k().to_bits(), l.haste.to_bits(), l.armour(), l.bite, l.ring.to_bits(), l.spin.to_bits()]));
        }
        assert_eq!(b.len(), MIND_AT, "Params rows before the Mind row moved");
        b.extend(u32_bytes(&[self.minds_on() as u32, s.spin_curl.to_bits(), 0, 0]));
        assert_eq!(b.len(), SCAR_AT, "Params rows before the scar row moved");
        // The CPU makes the scar only with the alarm field (`BigDish::new`).
        let scars = s.alarm_keep > 0.0 && s.scar_keep > 0.0;
        b.extend(f32_bytes(&[if scars { s.scar_keep } else { 0.0 }, s.scar_gain, s.scar_weight]));
        // Its spare word: dry slime burns (`fire_dry`, the plan's C1, bit 0)
        // and fed slime is fireproof (`fire_fed`, bit 1).
        b.extend(u32_bytes(&[s.fire_dry as u32 | (s.fire_fed as u32) << 1]));
        assert_eq!(b.len(), CRUST_AT, "Params rows before the crust row moved");
        // The crust runs only on a dish made with it (its block in `news`).
        let rise = if self.crust_at > 0 { s.crust_rise } else { 0.0 };
        b.extend(f32_bytes(&[rise, s.crust_keep, s.crust_trail]));
        b.extend(u32_bytes(&[(self.crust_at / 4) as u32]));
        assert_eq!(b.len(), CRUST_FX_AT, "Params rows before the crust's effects moved");
        // As the CPU: a dish without the crust reads none, and without
        // plants it has no char.
        let crusts = self.crust_at > 0;
        let fx = |v: f32| if crusts { v } else { 0.0 };
        b.extend(f32_bytes(&[fx(s.crust_weight), fx(s.crust_fast), fx(s.crust_fuel), if self.plants { s.char_barren } else { 0.0 }]));
        assert_eq!(b.len(), SET_AT, "Params rows before the setting row moved");
        // Setting needs the crust and the scar, as the CPU's `sets`.
        b.extend(f32_bytes(&[if crusts && scars { s.set_scar } else { 0.0 }, s.set_wake, s.set_crust]));
        b.extend(u32_bytes(&[s.set_armour]));
        self.setting = crusts && scars && s.set_scar > 0.0;
        self.energy_flows = self.grows && s.energy_flow > 0.0;
        // Switching withering on starts every walker as if it heard of food
        // now, as the CPU's first `hear_food` does: a mould is not cut off
        // at its first step.
        // The news runs for panic too (`BigParams::panic`).
        let withers = self.grows && (s.wither > 0.0 || s.panic > 0.0);
        if withers && !self.withers {
            let mut words = vec![self.done as u32 + 1; self.n as usize];
            words.resize(self.n as usize + 3 * self.cells as usize, 0);
            self.queue.write_buffer(&self.news, 0, &u32_bytes(&words));
        }
        self.withers = withers;
        self.reach = s.wither_reach.max(1) as u32;
        self.flows = s.crystal_flow > 0.0 || s.crystal_regrow > 0.0;
        assert_eq!(b.len(), PARAMS_BYTES, "Params is not PARAMS_BYTES long");
        self.converts = s.convert > 0.0 && self.nsp > 0;
        self.boxes = self.converts && s.convert_r > 0;
        self.outside = self.nsp > 0
            || self.kinds > 1
            || s.explores()
            || blocked > 0.0
            || s.blocked_keep
            || s.chance > 0.0
            || s.size_spread > 0.0
            || s.sensor_r > 0
            || graze != 0.0
            || self.grows
            || self.cycles
            || self.alarmed;
        self.queue.write_buffer(&self.params, 0, &b);
        self.jumps = s.respawn > 0.0 || pull > 0.0;
        self.crowd = crowd > 0 || sets.iter().any(|t| t.crowd > 0);
        self.per = !sets.is_empty();
    }

    /// Shots that land in the next frame (`write_params` sends the count):
    /// discs (x, y, radius) in fine cells that cut trail as the wipe does,
    /// leaving a wound the look shows. At most `MAX_HITS`.
    pub fn set_hits(&mut self, hits: &[(f32, f32, f32)]) {
        let n = hits.len().min(MAX_HITS);
        if n > 0 {
            let flat: Vec<f32> = hits[..n].iter().flat_map(|&(x, y, r)| [x, y, r, 0.0]).collect();
            self.queue.write_buffer(&self.hits, 0, &f32_bytes(&flat));
        }
        self.n_hits = n as u32;
    }

    /// Makes fine cells not agar (a wall the game built): walkers, trail
    /// and scent cannot enter them. Also changes `dish.agar`.
    pub fn block(&mut self, cells: &[usize]) {
        for &i in cells {
            if i < self.dish.agar.len() {
                self.dish.agar[i] = false;
            }
        }
        let agar: Vec<u32> = self.dish.agar.iter().map(|&a| a as u32).collect();
        self.queue.write_buffer(&self.agar, 0, &u32_bytes(&agar));
    }

    /// Puts `cells` back on the agar (a held cut that ends, `larder-world
    /// cut=T,X,Y,R,HOLD`): the caller passes only cells `block` took off.
    pub fn unblock(&mut self, cells: &[usize]) {
        for &i in cells {
            if i < self.dish.agar.len() {
                self.dish.agar[i] = true;
            }
        }
        let agar: Vec<u32> = self.dish.agar.iter().map(|&a| a as u32).collect();
        self.queue.write_buffer(&self.agar, 0, &u32_bytes(&agar));
    }

    /// Records a frame: `ticks` ticks, then the drawing. With `stamps`
    /// (and at least one tick), GPU timestamps at the first tick's start
    /// (0), the last tick's end (1) and the drawing's end (2).
    pub fn record(&mut self, shared: &Shared, enc: &mut wgpu::CommandEncoder, ticks: u32, soft: bool, stamps: Option<&wgpu::QuerySet>) {
        self.record_share(shared, enc, ticks, (true, true), soft, stamps);
    }

    /// A share of a frame (`GpuWorld::spread`, 2026-11-12): with `open`
    /// the squad's start, then `ticks` ticks, with `close` the squad's
    /// end, then the drawing. A frame recorded in shares, the first open
    /// and the last closed, runs the same passes in the same order as
    /// `record` (bar the drawing, which every share ends with); stamps as
    /// `record`'s.
    pub fn record_share(&mut self, shared: &Shared, enc: &mut wgpu::CommandEncoder, ticks: u32, (open, close): (bool, bool), soft: bool, stamps: Option<&wgpu::QuerySet>) {
        let stamps = if ticks > 0 { stamps } else { None };
        let (passes, ticked) = self.frame_passes(ticks, soft, open, close);
        let pipes = if self.outside { &shared.pipes } else { &shared.plain };
        for (q, &(p, groups, side)) in passes.iter().enumerate() {
            let begin = (q == ticked.start).then_some(0);
            let end = if q + 1 == ticked.end {
                Some(1)
            } else {
                (p == DRAW).then_some(2)
            };
            let writes = stamps.and_then(|qs| (begin.is_some() || end.is_some()).then_some(wgpu::ComputePassTimestampWrites { query_set: qs, beginning_of_pass_write_index: begin, end_of_pass_write_index: end }));
            let mut pass = enc.begin_compute_pass(&wgpu::ComputePassDescriptor { label: None, timestamp_writes: writes });
            pass.set_pipeline(&pipes[p]);
            pass.set_bind_group(0, &self.groups[side], &[]);
            pass.dispatch_workgroups(groups, 1, 1);
        }
    }

    /// The frame `record` records, with every pass timed on its own,
    /// submitted and waited for (so for measuring only, 2026-11-12): GPU ms
    /// a pass, by name, in order. The frame runs: the dish moves on.
    pub fn record_timed(&mut self, shared: &Shared, ticks: u32, soft: bool) -> Vec<(&'static str, f64)> {
        let (passes, _) = self.frame_passes(ticks, soft, true, true);
        self.time_passes(shared, &passes)
    }

    /// A frame's passes in order, each with its workgroups and the side of
    /// the bind group it reads: the squad's start if one is due (shots,
    /// their strikes, then the smell, on the trail and scent the first
    /// tick reads: `squad.rs`'s frame), `ticks` ticks, the squad's end
    /// (the reads, then the harvest), then the drawing; and where the
    /// ticks' passes are in it. Moves `done` on by `ticks`; `open` takes
    /// the squad's frame and its start, `close` its end (a frame in
    /// shares, `record_share`).
    fn frame_passes(&mut self, ticks: u32, soft: bool, open: bool, close: bool) -> (Vec<(usize, u32, usize)>, std::ops::Range<usize>) {
        let (nunits, nshots, _) = self.squad;
        if open {
            self.squad_on = std::mem::take(&mut self.squad_due);
        }
        let due = self.squad_on;
        let side = |done: u64| (done % 2) as usize;
        let mut out = Vec::new();
        if open && due {
            let s = side(self.done);
            // The gardeners' tend entries first (the CPU's `World::tend`
            // runs before the frame), then the player's walkers counted for
            // the shots' walker-in-a-crater stop.
            if self.garden {
                if self.ntend > 0 {
                    out.push((TEND, 1, s));
                }
                out.extend([(OWNCLEAR, self.cells.div_ceil(256), s), (OWNCOUNT, self.n.div_ceil(256), s)]);
            }
            if nshots > 0 {
                out.extend([(SHOTS, nshots.div_ceil(256), s), (STRIKE, self.n.div_ceil(256), s), (CUT, nshots, s)]);
                if self.alarmed {
                    out.push((SHOUT, 1, s));
                }
            }
            if nunits > 0 {
                out.push((if self.squad_one { SMELL1 } else { SMELL }, 1, s));
            }
        }
        let tick = self.tick_passes();
        let start = out.len();
        for _ in 0..ticks {
            let s = side(self.done);
            out.extend(tick.iter().map(|&(p, g)| (p, g, s)));
            out.extend(self.plant_passes(self.done).into_iter().map(|(p, g)| (p, g, s)));
            out.extend(self.set_passes(self.done).into_iter().map(|(p, g)| (p, g, s)));
            self.done += 1;
        }
        let ticked = start..out.len();
        let s = side(self.done);
        if close && due && nunits > 0 {
            if self.garden {
                out.extend([(OWNCLEAR, self.cells.div_ceil(256), s), (OWNCOUNT, self.n.div_ceil(256), s)]);
            }
            // The lumps' members' bite a cell. Only with a lump on (11-16:
            // on the core world these passes and condensing's mass cost
            // 0.36 ms a step). Skipped, the cells keep the last count:
            // `dclear` always runs before `dcount`, so the first frame they
            // come back on (a lump formed) starts from clear cells, and the
            // units read the cells only under the same gate (`P.lump_on`;
            // 11-16, the tech lead's review).
            if self.lumps_on || self.strikers.is_some() {
                if !std::mem::take(&mut self.plant_no_clear) {
                    out.push((DCLEAR, self.cells.div_ceil(256), s));
                }
                out.push((DCOUNT, self.n.div_ceil(256), s));
            }
            // A workgroup a unit (`engine.wgsl`'s `units`, 2026-11-12).
            if self.squad_one {
                out.extend([(UNITS1, squad::UNITS_MAX.div_ceil(64) as u32, s), (HARVEST1, 1, s)]);
            } else {
                out.extend([(UNITS, nunits, s), (HARVEST, 1, s)]);
            }
        }
        if close {
            self.squad_on = false;
        }
        out.extend(self.draw_passes(soft).into_iter().map(|(p, g)| (p, g, s)));
        (out, ticked)
    }

    /// A tick's passes and their workgroups: conversion only with species
    /// and `convert` on, the jump passes only while respawn or the pull is
    /// on, `land` only with the crowd limit.
    pub fn tick_passes(&self) -> Vec<(usize, u32)> {
        let (n, cells) = (self.n.div_ceil(256), self.cells.div_ceil(256));
        let mut tick = Vec::new();
        // Each lump's centre as the step begins (the CPU's `sum_lumps`).
        if self.lumps_on {
            tick.extend([(LUMPCLEAR, 1), (LUMPSUM, n)]);
        }
        tick.extend([(SENSE, n), (WALK, n)]);
        // The Larder World, in the CPU's order: bites on walls (in the
        // move), then eating; both before growth, which eats what they give.
        if self.walled {
            tick.push((WALLS, cells));
        }
        // The life cycle, as the CPU's `life_step` after the move and the
        // wall bites, before eating. The move's claims are still in
        // `claim`; a spore's landing claim never meets one, since every
        // claimed cell now holds its winner and the engine's crowd limit is
        // at most 1 (with a limit of 2 they would need clearing first with
        // `unclaim`, as the deleted `fed_crowd` did after the walk).
        if self.cycles {
            tick.extend([(FATE, n), (FLY, n), (ALIGHT, n)]);
        }
        if self.npc > 0 {
            let pc = self.npc.div_ceil(256);
            // A garden eats on every cell (fed charge lies off the crystals).
            tick.push((EAT, if self.garden { cells } else { pc }));
            if self.flows {
                tick.extend([(PILESUM, self.npiles), (FLOW, pc)]);
            }
            tick.push((PILESUM, self.npiles));
        }
        // Food's news, as the CPU's `hear_food` after `eat_crystals` and
        // before `live_and_die`: gathered, spread `reach` times (the first
        // from the gathered block, then the two others in turn; `nhear`
        // reads the last written and clears the gathered block).
        if self.withers {
            tick.extend([(NGATHER, n), (NSPREAD, cells)]);
            for r in 1..self.reach {
                tick.push((if r % 2 == 1 { NAB } else { NBA }, cells));
            }
            tick.push((NHEAR, n));
        }
        // One body's flow, as the CPU's `flow_energy` after `eat_crystals`
        // and before `live_and_die` (`life` clears its sums).
        if self.energy_flows {
            tick.extend([(ESUM, n), (EGAIN, n)]);
        }
        if self.grows {
            // The specks before `life` (`ripen`).
            if self.specks {
                tick.push((RIPEN, n));
            }
            tick.push((LIFE, n));
            // A garden's birth caps count each side first (`capcount`).
            if self.garden {
                tick.extend([(CAPCLEAR, 1), (CAPCOUNT, n)]);
            }
            tick.extend([(BREED, n), (BORN, n)]);
        }
        if self.converts {
            // Conversion's square: the rows' sums first (`rows`), the
            // columns in `convert`.
            if self.boxes {
                // A workgroup a 256-cell piece of a row (engine.wgsl's
                // `ROW_CHUNK`).
                tick.push((ROWS, (self.dish.w as u32).div_ceil(256) * self.dish.h as u32));
            }
            tick.push((CONVERT, n));
        }
        if self.jumps {
            tick.push((JUMP, n));
            // With species' own settings every jump lands in `land`.
            if self.crowd || self.per {
                tick.push((LAND, n));
            }
        }
        tick.extend([(TRAIL, cells), (SCENT, cells)]);
        if self.clouded {
            tick.push((GASP, n));
        }
        tick
    }

    /// The plants' passes after engine tick `done` (none unless it ends a
    /// game tick the CPU's `plant_tick` steps on: tick `done / plant_sub`
    /// a multiple of `plant_every`).
    pub fn plant_passes(&self, done: u64) -> Vec<(usize, u32)> {
        let (sub, every) = (self.plant_sub.max(1), self.plant_every.max(1));
        if !self.plants || done % sub != sub - 1 || !(done / sub).is_multiple_of(every) {
            return Vec::new();
        }
        let cells = self.cells.div_ceil(256);
        let mut v = vec![(PLANT, cells)];
        if self.fire_on {
            v.push((FIRE, cells));
        }
        v.push((PCOPY, cells));
        if self.fire_on {
            v.push((FIREKILL, self.n.div_ceil(256)));
        }
        v
    }

    /// The set walkers a cell after engine tick `done`, as the CPU's
    /// `count_set` at the end of a game tick (after the plants' passes):
    /// none while setting is off, so the cells keep their last count (all 0
    /// if it never ran).
    pub fn set_passes(&self, done: u64) -> Vec<(usize, u32)> {
        let sub = self.plant_sub.max(1);
        if !self.setting || done % sub != sub - 1 {
            return Vec::new();
        }
        vec![(SETCLEAR, self.cells.div_ceil(256)), (SETCOUNT, self.n.div_ceil(256))]
    }

    /// The set walkers a fine cell (`BigDish::set_cells`), read back; empty
    /// on a dish without the crust.
    pub fn set_cells_of(&self) -> Vec<u32> {
        if self.crust_at == 0 {
            return Vec::new();
        }
        let at = self.crust_at + 4 * (self.cells + self.n) as u64;
        read_back_range(&self.device, &self.queue, &self.news, at, 4 * self.cells as u64).chunks_exact(4).map(|b| u32::from_le_bytes(b.try_into().unwrap())).collect()
    }

    /// Lights the plants within `r` dish cells of (`x`, `y`) (dish cells),
    /// as the CPU's `BigDish::ignite`; nothing without plants. Runs on its
    /// own, between ticks.
    pub fn ignite(&mut self, shared: &Shared, lights: &[(f32, f32, f32, f32)]) {
        if !self.plants || lights.is_empty() {
            return;
        }
        self.discs(shared, lights, 0);
        // Kills mark the dead as the CPU's do (its kill makes `life`).
        self.cycles = true;
        self.fire_on = true;
    }

    /// Lays dry fuel within `r` dish cells of (`x`, `y`) (dish cells), each
    /// `(x, y, r, fuel)`, as the CPU's `BigDish::lay_fuel` (the gun cards'
    /// Tinder, 2026-11-17); nothing without plants. Runs on its own,
    /// between ticks: the IGNITE pass with word 49's mode byte 1.
    pub fn lay(&mut self, shared: &Shared, discs: &[(f32, f32, f32, f32)]) {
        if !self.plants || discs.is_empty() {
            return;
        }
        self.discs(shared, discs, 1);
    }

    /// The IGNITE pass over `discs`, three a run: word 49 the count, with
    /// `mode` in its second byte (0 light, 1 lay fuel), then x, y, r and the
    /// fourth word from 50.
    fn discs(&mut self, shared: &Shared, discs: &[(f32, f32, f32, f32)], mode: u32) {
        let k = self.dish.p.scale as f32;
        for chunk in discs.chunks(3) {
            let mut w = vec![chunk.len() as u32 | mode << 8];
            for &(x, y, r, share) in chunk {
                w.extend([(x * k).to_bits(), (y * k).to_bits(), (r * k).to_bits(), share.to_bits()]);
            }
            self.queue.write_buffer(&self.news, self.plant_at + 4 * 49, &u32_bytes(&w));
            self.run(shared, &[(IGNITE, self.cells.div_ceil(256))]);
        }
    }

    /// The plants now (density, kind, fire's heat, char), read back: the
    /// CPU's `plant`, `plant_kind`, `fire`, `char`. Empty without plants.
    pub fn plants_of(&self) -> (Vec<f32>, Vec<u8>, Vec<f32>, Vec<f32>) {
        if !self.plants {
            return Default::default();
        }
        let b = read_back(&self.device, &self.queue, &self.news);
        let c = self.cells as usize;
        let at = self.plant_at as usize + 4 * PLANT_HEAD;
        let word = |blk: usize, i: usize| u32::from_le_bytes(b[at + 4 * (blk * c + i)..at + 4 * (blk * c + i) + 4].try_into().unwrap());
        let f = |blk: usize| (0..c).map(|i| f32::from_bits(word(blk, i))).collect::<Vec<f32>>();
        (f(2), (0..c).map(|i| word(1, i) as u8).collect(), f(4), f(6))
    }

    /// The drawing's passes: the soft curve's histogram and reference first.
    pub fn draw_passes(&self, soft: bool) -> Vec<(usize, u32)> {
        let cells = self.cells.div_ceil(256);
        let mut draw = Vec::new();
        if soft {
            draw.extend([(HISTOGRAM, cells), (TOP, 1)]);
        }
        draw.push((DRAW, cells));
        draw
    }

    /// One tick and the drawing with every pass timed on its own (each
    /// pass's start and end stamped), submitted and waited for: GPU ms a
    /// pass, by name. It runs the tick, so the dish moves on by one.
    pub fn profile(&mut self, shared: &Shared, soft: bool) -> Vec<(&'static str, f64)> {
        let tick = self.tick_passes();
        let passes: Vec<(usize, u32, usize)> = tick.iter().map(|&(p, g)| (p, g, (self.done % 2) as usize)).chain(self.draw_passes(soft).into_iter().map(|(p, g)| (p, g, ((self.done + 1) % 2) as usize))).collect();
        self.done += 1;
        self.time_passes(shared, &passes)
    }

    /// `passes` (pass, workgroups, side), each stamped at its start and
    /// end, submitted and waited for: GPU ms a pass, by name.
    fn time_passes(&self, shared: &Shared, passes: &[(usize, u32, usize)]) -> Vec<(&'static str, f64)> {
        let pipes = if self.outside { &shared.pipes } else { &shared.plain };
        let count = 2 * passes.len() as u32;
        let qs = self.device.create_query_set(&wgpu::QuerySetDescriptor { label: Some("profile"), ty: wgpu::QueryType::Timestamp, count });
        let resolve = self.device.create_buffer(&wgpu::BufferDescriptor { label: Some("profile"), size: count as u64 * 8, usage: wgpu::BufferUsages::QUERY_RESOLVE | wgpu::BufferUsages::COPY_SRC, mapped_at_creation: false });
        let mut enc = self.device.create_command_encoder(&wgpu::CommandEncoderDescriptor { label: None });
        for (q, &(p, groups, side)) in passes.iter().enumerate() {
            let writes = wgpu::ComputePassTimestampWrites { query_set: &qs, beginning_of_pass_write_index: Some(2 * q as u32), end_of_pass_write_index: Some(2 * q as u32 + 1) };
            let mut pass = enc.begin_compute_pass(&wgpu::ComputePassDescriptor { label: None, timestamp_writes: Some(writes) });
            pass.set_pipeline(&pipes[p]);
            pass.set_bind_group(0, &self.groups[side], &[]);
            pass.dispatch_workgroups(groups, 1, 1);
        }
        enc.resolve_query_set(&qs, 0..count, &resolve, 0);
        self.queue.submit([enc.finish()]);
        let t: Vec<u64> = read_back(&self.device, &self.queue, &resolve).chunks_exact(8).map(|c| u64::from_le_bytes(c.try_into().unwrap())).collect();
        let period = self.queue.get_timestamp_period() as f64;
        passes.iter().enumerate().map(|(q, &(p, ..))| (PASSES[p], t[2 * q + 1].saturating_sub(t[2 * q]) as f64 * period / 1e6)).collect()
    }

    /// Reads the dish back into the CPU dish and measures it.
    pub fn measure(&mut self) -> crate::mould::big::BigMeasure {
        let side = (self.done % 2) as usize;
        self.dish.trail = floats(&read_back(&self.device, &self.queue, &self.trail[side]));
        self.dish.walkers = floats(&read_back(&self.device, &self.queue, &self.walkers)).chunks_exact(4).map(|c| (c[0], c[1], c[2])).collect();
        if self.grows {
            let e = floats(&read_back(&self.device, &self.queue, &self.energy));
            self.dish.alive = e.iter().take(self.n as usize).map(|&v| v >= 0.0).collect();
            self.dish.energy = e.iter().take(self.n as usize).map(|&v| v.max(0.0)).collect();
        }
        self.dish.ticks = self.done;
        if self.specks {
            self.dish.dense = self.dense_of();
        }
        // The life states, so the measure leaves the dead out.
        if self.cycles {
            (self.dish.life, self.dish.life_t) = self.life_of();
            if !self.dish.flung.is_empty() {
                self.dish.flung = self.flung_of();
            }
            if !self.dish.doomed.is_empty() {
                self.dish.doomed = self.marked(0x4000);
            }
        }
        // The Larder World: the crystals as the GPU holds them, so the
        // measure's crystal share left is the GPU's.
        if self.npiles > 0 {
            self.dish.charge = floats(&read_back(&self.device, &self.queue, &self.charge));
            let (sums, ..) = self.world_counts();
            for (p, (sum, cells)) in sums.into_iter().enumerate() {
                self.dish.pile_charge[p] = sum;
                self.dish.pile_full[p] = self.dish.pile_fill[p] * cells as f32;
            }
        }
        // The GPU marks a pile from the trail a tick late (see `trail`), so
        // the CPU marks any the last tick's trail reached, as it would.
        let reached = read_back(&self.device, &self.queue, &self.reached);
        for (r, c) in self.dish.reached.iter_mut().zip(reached.chunks_exact(4)) {
            *r = u32::from_le_bytes(c.try_into().unwrap()) != 0;
        }
        self.dish.update_reached();
        self.dish.measure()
    }

    /// The living walkers now, with growth on (a read-back of the energy;
    /// a dead slot holds -1) and without the struck dead (life cycle).
    /// With both off, every walker.
    pub fn alive(&self) -> usize {
        let dead = if self.cycles { self.life_of().0.iter().filter(|&&l| l == LIFE_DEAD).count() } else { 0 };
        if !self.grows {
            return self.n as usize - dead;
        }
        // With growth a struck walker's slot is empty, so not counted twice.
        floats(&read_back(&self.device, &self.queue, &self.energy)).iter().take(self.n as usize).filter(|&&v| v >= 0.0).count()
    }

    /// The share of walkers sick now (counter over `immune`).
    pub fn sick_share(&self, s: &Settings) -> f64 {
        let c = read_back(&self.device, &self.queue, &self.sick);
        let sick = c.chunks_exact(4).take(self.n as usize).filter(|b| u32::from_le_bytes((*b).try_into().unwrap()) > s.immune).count();
        sick as f64 / self.n.max(1) as f64
    }

    /// A wall from (`ax`, `ay`) to (`bx`, `by`), dish cells, `width` dish
    /// cells thick, as `BigDish::build_wall` builds it, on the GPU: the
    /// `build` pass over the wall's box, then `EVICT_ROUNDS` rounds that
    /// move the walkers it covers to the nearest free cells (the CPU moves
    /// them one by one in walker order, so where they land can differ).
    /// Writes the params from `s` first, as the passes read them. The CPU
    /// mirror (`dish`) is not changed.
    #[allow(clippy::too_many_arguments)]
    pub fn build_wall(&mut self, shared: &Shared, s: &Settings, ax: f32, ay: f32, bx: f32, by: f32, width: f32) {
        let k = self.dish.p.scale as f32;
        let (w, h) = (self.dish.w, self.dish.h);
        let (ax, ay, bx, by, r) = (ax * k, ay * k, bx * k, by * k, (width * k / 2.0).max(0.5));
        let (x0, x1) = ((ax.min(bx) - r).floor().max(0.0) as usize, ((ax.max(bx) + r).ceil() as usize).min(w - 1));
        let (y0, y1) = ((ay.min(by) - r).floor().max(0.0) as usize, ((ay.max(by) + r).ceil() as usize).min(h - 1));
        if x1 < x0 || y1 < y0 {
            return;
        }
        let (bw, bh) = ((x1 - x0 + 1) as u32, (y1 - y0 + 1) as u32);
        let hp = if self.dish.p.wall_hp > 0.0 { self.dish.p.wall_hp } else { f32::INFINITY };
        let reach = (16.0 * k).ceil() as u32;
        let mut wb = f32_bytes(&[ax, ay, bx, by, r, hp]);
        wb.extend(u32_bytes(&[x0 as u32, y0 as u32, bw, bh, (s.wall_scent > 0.0) as u32, reach]));
        self.queue.write_buffer(&self.wb, 0, &wb);
        self.walled = true;
        self.write_params(s, Hand::default(), 0);
        let pipes = if self.outside { &shared.pipes } else { &shared.plain };
        let group = &self.groups[(self.done % 2) as usize];
        let (n, cells) = (self.n.div_ceil(256), self.cells.div_ceil(256));
        let submit = |passes: &[(usize, u32)]| {
            let mut enc = self.device.create_command_encoder(&wgpu::CommandEncoderDescriptor { label: Some("wall") });
            for &(p, groups) in passes {
                let mut pass = enc.begin_compute_pass(&wgpu::ComputePassDescriptor { label: None, timestamp_writes: None });
                pass.set_pipeline(&pipes[p]);
                pass.set_bind_group(0, group, &[]);
                pass.dispatch_workgroups(groups, 1, 1);
            }
            self.queue.submit([enc.finish()]);
        };
        submit(&[(BUILD, (bw * bh).div_ceil(256))]);
        let round = [(EVICT, n), (SETTLE, n), (UNCLAIM, cells)];
        for _ in 0..EVICT_MAX / EVICT_ROUNDS {
            self.queue.write_buffer(&self.wstat, 8, &[0u8; 4]);
            submit(&round.repeat(EVICT_ROUNDS));
            let st = read_back(&self.device, &self.queue, &self.wstat);
            if u32::from_le_bytes(st[8..12].try_into().unwrap()) == 0 {
                break;
            }
        }
    }

    /// Submits `passes` (pass, workgroups) on the current side, apart from
    /// a tick (the tick count does not move): for tests of one pass.
    pub fn run(&self, shared: &Shared, passes: &[(usize, u32)]) {
        let pipes = if self.outside { &shared.pipes } else { &shared.plain };
        let group = &self.groups[(self.done % 2) as usize];
        let mut enc = self.device.create_command_encoder(&wgpu::CommandEncoderDescriptor { label: Some("run") });
        for &(p, groups) in passes {
            let mut pass = enc.begin_compute_pass(&wgpu::ComputePassDescriptor { label: None, timestamp_writes: None });
            pass.set_pipeline(&pipes[p]);
            pass.set_bind_group(0, group, &[]);
            pass.dispatch_workgroups(groups, 1, 1);
        }
        self.queue.submit([enc.finish()]);
    }

    /// The player's attack, as the CPU's `BigDish::strike` (the tiers'
    /// test of how each mould answers it; dish cells): within `r` of
    /// (`x`, `y`) the trail (each species' too) is cleared and every
    /// walker on the ground dies (spores in flight are missed; with growth
    /// the slot is freed), and within `alarm_r` `alarm` is added to the
    /// alarm field if the dish has one. With `r` 0 it is only a shock.
    /// Switches the life states on. Runs on its own, between ticks;
    /// `struck` reads back the walkers killed so far.
    #[allow(clippy::too_many_arguments)]
    pub fn strike(&mut self, shared: &Shared, s: &Settings, x: f32, y: f32, r: f32, alarm: f32, alarm_r: f32) {
        self.burst(shared, s, x, y, r, alarm, alarm_r, 0.0, 0, 0.0);
    }

    /// **Sleep gas**, as the CPU's `BigDish::gas` (dish cells, the
    /// squad toolkit's sleep gas on the engine, 11-14): every living
    /// ordinary or dormant walker within `r` of (`x`, `y`) falls dormant
    /// with its timer at 0, deaf to scent for `steps` fine steps (alarm
    /// still wakes it), and while gassed dies asleep at `s.gas_death` if
    /// that is over `sleep_death`. Switches the life states on. Runs on
    /// its own, between ticks; returns the walkers gassed (a read-back).
    pub fn gas(&mut self, shared: &Shared, s: &Settings, x: f32, y: f32, r: f32, steps: u16) -> u32 {
        let before = self.gas_word(2);
        self.gas_pass(shared, s, x, y, r, steps);
        self.gas_word(2).wrapping_sub(before)
    }

    /// `gas` with no read-back: its count is on the `news` gas word 2
    /// (`snap` copies it), for a batch of bursts read back at once
    /// (`GpuWorld::bursts`, 2026-11-16).
    pub fn gas_pass(&mut self, shared: &Shared, s: &Settings, x: f32, y: f32, r: f32, steps: u16) {
        let k = self.dish.p.scale as f32;
        self.cycles = true;
        self.write_params(s, Hand::default(), 0);
        self.queue.write_buffer(&self.params, STRIKE_AT as u64, &f32_bytes(&[x * k, y * k, r * k, 0.0, 0.0, 0.0, steps as f32, 0.0]));
        self.queue.write_buffer(&self.news, self.gas_at, &u32_bytes(&[self.done as u32]));
        self.queue.write_buffer(&self.news, self.gas_at + 24, &u32_bytes(&[self.gas_immune]));
        self.run(shared, &[(GAS, self.n.div_ceil(256))]);
        if steps > 0 {
            self.gas_until = self.gas_until.max(self.done + steps as u64);
        }
    }

    /// **A Mind status** (the CPU's `BigDish::mind`, dish cells): every
    /// living ordinary or dormant walker within `r` of (`x`, `y`) takes
    /// status `kind` for `steps` fine steps from now (`mindlay`), and
    /// `sense` reads the Mind words from the next tick on (`minds`).
    #[allow(clippy::too_many_arguments)]
    pub fn mind_pass(&mut self, shared: &Shared, s: &Settings, x: f32, y: f32, r: f32, kind: u32, steps: u16) {
        let k = self.dish.p.scale as f32;
        self.minds = true;
        self.mind_until = self.mind_until.max(self.done + steps as u64);
        self.write_params(s, Hand::default(), 0);
        self.queue.write_buffer(&self.params, STRIKE_AT as u64, &f32_bytes(&[x * k, y * k, r * k, 0.0, 0.0, 0.0, steps as f32, kind as f32]));
        self.queue.write_buffer(&self.news, self.gas_at, &u32_bytes(&[self.done as u32]));
        self.queue.write_buffer(&self.news, self.gas_at + 28, &u32_bytes(&[self.mind_rest]));
        self.run(shared, &[(MINDLAY, self.n.div_ceil(256))]);
    }

    /// **A reader's kill** (the CPU's `BigDish::read_strike`, dish cells):
    /// every living ordinary or dormant walker within `r` of (`x`, `y`) in
    /// the state `kind` reads (`big::READ_SLEEP`, `READ_SICK`) takes a
    /// blow (`readkill`); the kills count on the strike's counter.
    pub fn read_pass(&mut self, shared: &Shared, s: &Settings, x: f32, y: f32, r: f32, kind: u8) {
        let k = self.dish.p.scale as f32;
        self.write_params(s, Hand::default(), 0);
        self.queue.write_buffer(&self.params, STRIKE_AT as u64, &f32_bytes(&[x * k, y * k, r * k, 0.0, 0.0, 0.0, 0.0, kind as f32]));
        self.queue.write_buffer(&self.news, self.gas_at, &u32_bytes(&[self.done as u32]));
        self.run(shared, &[(READKILL, self.n.div_ceil(256))]);
    }

    /// Whether `sense` reads the Mind words this tick: once any walker has
    /// had a status, until the last one runs out (`mind_until`; the
    /// frame's params are written before its ticks, so a step at or past
    /// the end has none left).
    pub fn minds_on(&self) -> bool {
        self.minds && self.done <= self.mind_until
    }

    /// Each slot's Mind word (the CPU's `BigDish::mind`), read back.
    pub fn mind_words(&self) -> Vec<u32> {
        let at = self.gas_at + 4 * (GAS_SLOTS as u64 + self.n as u64);
        let b = read_back_range(&self.device, &self.queue, &self.news, at, 4 * self.n as u64);
        b.chunks_exact(4).map(|c| u32::from_le_bytes(c.try_into().unwrap())).collect()
    }

    /// The engine's three burst counters as they will stand once the GPU
    /// reaches this point (the walkers struck, gassed and flung: `wstat`
    /// word 3 and the `news` gas words 2 and 3), copied on the GPU into
    /// slot `slot` of `dst` (16 bytes a slot) with no wait: a batch of
    /// bursts snaps after each and reads `dst` back once (`read_snaps`).
    pub fn snap(&self, dst: &wgpu::Buffer, slot: u64) {
        let mut enc = self.device.create_command_encoder(&wgpu::CommandEncoderDescriptor { label: Some("snap") });
        enc.copy_buffer_to_buffer(&self.wstat, 12, dst, 16 * slot, 4);
        enc.copy_buffer_to_buffer(&self.news, self.gas_at + 8, dst, 16 * slot + 4, 8);
        self.queue.submit([enc.finish()]);
    }

    /// The first `n` slots `snap` wrote: (struck, gassed, flung) each, in
    /// one read-back.
    pub fn read_snaps(&self, dst: &wgpu::Buffer, n: usize) -> Vec<[u32; 3]> {
        let b = read_back_range(&self.device, &self.queue, dst, 0, 16 * n as u64);
        b.chunks_exact(16).map(|c| [0, 1, 2].map(|i| u32::from_le_bytes(c[4 * i..4 * i + 4].try_into().unwrap()))).collect()
    }

    /// **The gas cloud**, as the CPU's `BigDish::lay_gas` (dish cells, as
    /// `gas`): `dose` of cloud on each agar cell within `r` of (`x`, `y`),
    /// which from the next tick spreads at `s.gas_keep` and puts to sleep
    /// every walker over `s.gas_sleep`, gassed `s.gas_linger` steps more
    /// (`gasp`, a pass a tick from the first cloud on). Runs on its own,
    /// between ticks.
    /// Food scent laid within `r` of (`x`, `y`) (dish cells): the CPU's
    /// `BigDish::lay_bait` (the rounds' `Bait` trail, 11-14).
    pub fn lay_scent(&mut self, shared: &Shared, s: &Settings, x: f32, y: f32, r: f32, (keep, amount): (f32, f32)) {
        let k = self.dish.p.scale as f32;
        self.write_params(s, Hand::default(), 0);
        self.queue.write_buffer(&self.params, STRIKE_AT as u64, &f32_bytes(&[x * k, y * k, r * k, amount, keep, 0.0, 0.0, 0.0]));
        self.run(shared, &[(BAITLAY, self.cells.div_ceil(256))]);
    }

    pub fn lay_gas(&mut self, shared: &Shared, s: &Settings, x: f32, y: f32, r: f32, dose: f32) {
        let k = self.dish.p.scale as f32;
        self.cycles = true;
        self.clouded = true;
        self.write_params(s, Hand::default(), 0);
        self.queue.write_buffer(&self.params, STRIKE_AT as u64, &f32_bytes(&[x * k, y * k, r * k, dose, 0.0, 0.0, 0.0, 0.0]));
        self.queue.write_buffer(&self.news, self.gas_at + 16, &u32_bytes(&[s.gas_linger as u32, s.gas_sleep.to_bits(), self.gas_immune]));
        self.run(shared, &[(GASLAY, self.cells.div_ceil(256))]);
        // How long `calm` must keep looking: the cloud's top can only fall
        // by `gas_keep` a step once laid (a 3x3 mean never raises it), so
        // from a bound on it, until it is under `gas_sleep`, then the
        // linger.
        let keep = s.gas_keep.clamp(0.0, 0.999_999);
        let since = self.done.saturating_sub(self.cloud_at) as i32;
        self.cloud_top = self.cloud_top * keep.powi(since.min(1 << 20)) + dose.max(0.0);
        self.cloud_at = self.done;
        if self.cloud_until != u64::MAX {
            let left = if self.cloud_top <= s.gas_sleep || keep <= 0.0 { 0 } else { ((s.gas_sleep / self.cloud_top).ln() / keep.ln()).ceil() as u64 + 1 };
            self.cloud_until = self.cloud_until.max(self.done + left + s.gas_linger as u64 + 1);
        }
    }

    fn gas_word(&self, i: usize) -> u32 {
        let b = read_back_range(&self.device, &self.queue, &self.news, self.gas_at + 4 * i as u64, 4);
        u32::from_le_bytes(b[..4].try_into().unwrap())
    }

    /// For each point (fine cells, at most `squad::UNITS_MAX`), the gassed
    /// dormant walkers within `r` of it now: the CPU's
    /// `BigDish::gassed_near`. Zeros with no read-back once every gas has
    /// run out (`gas_until`).
    pub fn calm(&self, shared: &Shared, at: &[(f32, f32)], r: f32) -> Vec<u32> {
        let n = at.len().min(squad::UNITS_MAX);
        if self.done >= self.gas_until.max(self.cloud_until) || n == 0 {
            self.clouds.borrow_mut().clear();
            return vec![0; at.len()];
        }
        let mut words = vec![self.done as u32, n as u32, 0, 0];
        words.resize(GAS_HEAD + 4 * squad::UNITS_MAX + squad::UNITS_MAX, 0);
        for (u, &(x, y)) in at.iter().take(n).enumerate() {
            words[GAS_HEAD + 4 * u..GAS_HEAD + 4 * u + 3].copy_from_slice(&[x.to_bits(), y.to_bits(), r.to_bits()]);
        }
        // Word 2 (walkers gassed so far) is kept.
        // Words 2-7 (walkers gassed so far, the flung count, the cloud's
        // linger and sleep level) are kept.
        self.queue.write_buffer(&self.news, self.gas_at, &u32_bytes(&words[..2]));
        self.queue.write_buffer(&self.news, self.gas_at + 4 * GAS_HEAD as u64, &u32_bytes(&words[GAS_HEAD..]));
        self.run(shared, &[(CALM, self.n.div_ceil(256))]);
        // One read-back: the points (the cloud in each one's fourth word,
        // 2026-11-18) and the counts after them.
        let pts = self.gas_at + 4 * GAS_HEAD as u64;
        let b = read_back_range(&self.device, &self.queue, &self.news, pts, 4 * (4 * squad::UNITS_MAX + n) as u64);
        let word = |i: usize| u32::from_le_bytes(b[4 * i..4 * i + 4].try_into().unwrap());
        let mut clouds: Vec<f32> = (0..n).map(|u| f32::from_bits(word(4 * u + 3))).collect();
        clouds.resize(at.len(), 0.0);
        *self.clouds.borrow_mut() = clouds;
        let mut out: Vec<u32> = (0..n).map(|u| word(4 * squad::UNITS_MAX + u)).collect();
        out.resize(at.len(), 0);
        out
    }

    /// Fine steps of gas left each slot now (the CPU's `gassed`), read back.
    pub fn gassed_of(&self) -> Vec<u16> {
        let b = read_back(&self.device, &self.queue, &self.news);
        b[self.gas_at as usize + GAS_SLOTS * 4..].chunks_exact(4).take(self.n as usize).map(|c| u32::from_le_bytes(c.try_into().unwrap()).saturating_sub(self.done as u32).min(u16::MAX as u32) as u16).collect()
    }

    /// A strike that also lays a crater of `steps` fine steps within
    /// `crater_r` dish cells (the squad toolkit's burst, `squad::Burst`; the
    /// CPU's `strike` then `scorch_disc`), in the same two passes: the
    /// `hit` pass writes the damage memory's first block (binding 47) as it
    /// clears the trail. From then on the trail pass counts the crater down.
    /// With contagion on (`Settings::sick`), the `kill` pass also makes
    /// every living walker within `germ_r` dish cells sick (the toolkit's
    /// germ, the CPU's `BigDish::infect`), whatever `r` kills.
    #[allow(clippy::too_many_arguments)]
    pub fn burst(&mut self, shared: &Shared, s: &Settings, x: f32, y: f32, r: f32, alarm: f32, alarm_r: f32, crater_r: f32, steps: u16, germ_r: f32) {
        let k = self.dish.p.scale as f32;
        self.cycles = true;
        self.scorched |= steps > 0;
        self.write_params(s, Hand::default(), 0);
        self.queue.write_buffer(&self.params, STRIKE_AT as u64, &f32_bytes(&[x * k, y * k, r * k, alarm, alarm_r * k, crater_r * k, steps as f32, germ_r * k]));
        self.run(shared, &[(HIT, self.cells.div_ceil(256)), (KILL, self.n.div_ceil(256))]);
    }

    /// Walkers killed by strikes (`strike`) since the dish was made or
    /// injected.
    pub fn struck(&self) -> u32 {
        let st = read_back(&self.device, &self.queue, &self.wstat);
        u32::from_le_bytes(st[12..16].try_into().unwrap())
    }

    /// Each slot's life state and timer (the CPU's `life`, `life_t`), read
    /// back from the `spec` words.
    pub fn life_of(&self) -> (Vec<u8>, Vec<u16>) {
        let w: Vec<u32> = read_back(&self.device, &self.queue, &self.spec).chunks_exact(4).take(self.n as usize).map(|c| u32::from_le_bytes(c.try_into().unwrap())).collect();
        (w.iter().map(|&w| ((w >> 9) & 3) as u8).collect(), w.iter().map(|&w| (w >> 16) as u16).collect())
    }

    /// Each slot's flung mark (the CPU's `flung`, engine.wgsl's `FLUNG`),
    /// read back from the `spec` words.
    pub fn flung_of(&self) -> Vec<bool> {
        self.marked(0x2000)
    }

    /// Whether each slot's `spec` word has `bit` (`FLUNG` 0x2000, `DOOM`
    /// 0x4000: the CPU's `flung` and `doomed`), read back.
    fn marked(&self, bit: u32) -> Vec<bool> {
        read_back(&self.device, &self.queue, &self.spec).chunks_exact(4).take(self.n as usize).map(|c| u32::from_le_bytes(c.try_into().unwrap()) & bit != 0).collect()
    }

    /// The spray row (`SPRAY_AT`): the longest flight, the first speed and
    /// the flight's reciprocal (`big::spray_inv`), then the fine steps a
    /// tick in the last word's low byte and the spray's share
    /// (`big::spray_share_q`, at most 65,536) above it.
    fn spray_row(&self) -> [u32; 4] {
        let inv = crate::mould::big::spray_inv(self.spray.0 as u16);
        let ticks = self.dish.p.substeps.clamp(1, 255) as u32;
        assert!(self.spray.2 <= 65536, "the spray's share is a fraction of 65,536");
        [self.spray.0, self.spray.1.to_bits(), inv.to_bits(), ticks | (self.spray.2 << 8)]
    }

    /// **An explosion's fling** (the CPU's `BigDish::fling`; dish cells,
    /// the gun rounds' `Bomb`, 11-14): every living ordinary or dormant
    /// walker between `r` and `rr` of (`x`, `y`) leaves its cell and flies
    /// straight out for up to `steps` fine steps at `s.fling_speed`, then
    /// lands and lives on. `inv` is 1 / (`rr`² - `r`²) in fine cells
    /// (`squad::Burst::fling_ring`). Switches the life states on. Runs on
    /// its own, between ticks; returns the walkers flung (a read-back).
    #[allow(clippy::too_many_arguments)]
    /// With `push` (a heading, radians) every walker in the ring flies
    /// along it for the whole `steps` (`BigDish::fling`): the row's `kcr`
    /// holds the heading and `kir` 1.
    #[allow(clippy::too_many_arguments)]
    pub fn fling(&mut self, shared: &Shared, s: &Settings, x: f32, y: f32, r: f32, rr: f32, steps: u16, inv: f32, push: Option<f32>) -> u32 {
        let before = self.gas_word(3);
        self.fling_pass(shared, s, x, y, r, rr, steps, inv, push);
        self.gas_word(3).wrapping_sub(before)
    }

    /// `fling` with no read-back: its count is on the `news` gas word 3
    /// (`snap` copies it), as `gas_pass`.
    #[allow(clippy::too_many_arguments)]
    pub fn fling_pass(&mut self, shared: &Shared, s: &Settings, x: f32, y: f32, r: f32, rr: f32, steps: u16, inv: f32, push: Option<f32>) {
        let k = self.dish.p.scale as f32;
        self.cycles = true;
        // So `measure` reads the marks back.
        if self.dish.flung.is_empty() {
            self.dish.flung = vec![false; self.n as usize];
        }
        self.write_params(s, Hand::default(), 0);
        // The strike row as the `fling` pass reads it: `kalarm` holds `inv`.
        let (head, pushed) = push.map_or((0.0, 0.0), |a| (a, 1.0));
        self.queue.write_buffer(&self.params, STRIKE_AT as u64, &f32_bytes(&[x * k, y * k, r * k, inv, rr * k, head, steps as f32, pushed]));
        self.run(shared, &[(FLING, self.n.div_ceil(256))]);
    }

    /// **A sowing** (2026-11-20, a ripe fruit bursting,
    /// `larder_world::fruit`): every living ordinary or dormant walker
    /// within `rr` dish cells of (`x`, `y`) (dish cells) leaves as a homing
    /// spore (`SOWN` in engine.wgsl), thrown straight out from the centre
    /// for `steps` steps: straight while its timer is over the dish's
    /// `spore_steps` / 2, then by scent, coming down on food (`spore_land`)
    /// or where its timer runs out. The fling's pass with `kir` 2; counted
    /// on the `news` gas word 3, as a fling. GPU only (the owner's 11-17
    /// 19:21Z card: new slime rules on the GPU engine, no CPU twin).
    pub fn sow_pass(&mut self, shared: &Shared, s: &Settings, x: f32, y: f32, rr: f32, steps: u16) {
        let k = self.dish.p.scale as f32;
        self.cycles = true;
        self.write_params(s, Hand::default(), 0);
        let rr = rr * k;
        let inv = 1.0 / (rr * rr).max(1e-6);
        self.queue.write_buffer(&self.params, STRIKE_AT as u64, &f32_bytes(&[x * k, y * k, 0.0, inv, rr, 0.0, steps.max(2) as f32, 2.0]));
        self.run(shared, &[(FLING, self.n.div_ceil(256))]);
    }

    /// The alarm field now (zeros without one).
    /// The crater countdown a cell (the damage memory's first block), read
    /// back: the CPU's `BigDish::scorch`.
    pub fn crater_of(&self) -> Vec<f32> {
        let g = floats(&read_back(&self.device, &self.queue, &self.crater));
        g[..self.cells as usize].to_vec()
    }

    /// The scar a cell (the damage memory's second block), read back: the
    /// CPU's `BigDish::scar`.
    pub fn scar_of(&self) -> Vec<f32> {
        let g = floats(&read_back(&self.device, &self.queue, &self.crater));
        g[self.cells as usize..2 * self.cells as usize].to_vec()
    }

    /// The crust a cell, read back (the CPU's `BigDish::crust`; empty
    /// without one).
    pub fn crust_of(&self) -> Vec<f32> {
        if self.crust_at == 0 {
            return Vec::new();
        }
        floats(&read_back_range(&self.device, &self.queue, &self.news, self.crust_at, 4 * self.cells as u64))
    }

    /// Each slot's set word, read back (the CPU's `BigDish::set`; empty
    /// without the crust).
    pub fn set_of(&self) -> Vec<u32> {
        if self.crust_at == 0 {
            return Vec::new();
        }
        let b = read_back_range(&self.device, &self.queue, &self.news, self.crust_at + 4 * self.cells as u64, 4 * self.n as u64);
        b.chunks_exact(4).map(|c| u32::from_le_bytes(c.try_into().unwrap())).collect()
    }

    /// Species `sp`'s trail a cell now, read back (the CPU's
    /// `species_trail[sp]`).
    pub fn species_trail_of(&self, sp: usize) -> Vec<f32> {
        let g = floats(&read_back(&self.device, &self.queue, &self.species[(self.done % 2) as usize]));
        let c = self.cells as usize;
        g[sp * c..(sp + 1) * c].to_vec()
    }

    /// Each cell's charge now, read back (the CPU's `charge`).
    pub fn charge_of(&self) -> Vec<f32> {
        floats(&read_back(&self.device, &self.queue, &self.charge))[..self.cells as usize].to_vec()
    }

    /// The gas cloud now (the CPU's `gas_cloud`), read back.
    pub fn gas_cloud_of(&self) -> Vec<f32> {
        let g = floats(&read_back(&self.device, &self.queue, &self.germ[(self.done % 2) as usize]));
        g[2 * self.cells as usize..3 * self.cells as usize].to_vec()
    }

    /// The food scent now (the CPU's `scent`), read back.
    pub fn scent_of(&self) -> Vec<f32> {
        floats(&read_back(&self.device, &self.queue, &self.scent[(self.done % 2) as usize]))
    }

    pub fn alarm_of(&self) -> Vec<f32> {
        let g = floats(&read_back(&self.device, &self.queue, &self.germ[(self.done % 2) as usize]));
        g[self.cells as usize..2 * self.cells as usize].to_vec()
    }

    /// Counts the living walkers (the CPU's `living`: a slot alive and not
    /// dead) into `wstat` word 4, and with `grid` set the mould grid (the
    /// CPU's `World::mould_grid` of `grid`-cell squares) into the words
    /// after `WSTAT_BYTES`, in `enc` after a frame's ticks: the game's mould
    /// size and the AI's eyes with no read-back of their own (`GpuWorld`'s
    /// readout copies them). Before 2026-11-12 the game read the whole
    /// energy buffer and the `spec` words back once a game second for the
    /// count (`alive`), 8 MB at 100k walkers' 1M slots, and the GPU world
    /// had no grid. The params must be written (`write_params`) first.
    pub fn record_count(&self, shared: &Shared, enc: &mut wgpu::CommandEncoder) {
        let (gw, gh) = self.grid_size();
        assert!(gw * gh <= self.grid_max(), "a mould grid square under a dish cell");
        // Words 4 (the living) and 5 (the player's own mould's, in a garden).
        enc.clear_buffer(&self.wstat, 16, Some(8));
        if gw * gh > 0 {
            enc.clear_buffer(&self.wstat, WSTAT_BYTES as u64, Some(4 * (gw * gh) as u64));
        }
        let pipes = if self.outside { &shared.pipes } else { &shared.plain };
        let mut pass = enc.begin_compute_pass(&wgpu::ComputePassDescriptor { label: Some("count"), timestamp_writes: None });
        pass.set_pipeline(&pipes[COUNT]);
        pass.set_bind_group(0, &self.groups[(self.done % 2) as usize], &[]);
        let threads = if self.grid > 0 { self.n.max(self.cells) } else { self.n };
        pass.dispatch_workgroups(threads.div_ceil(256), 1, 1);
    }

    /// The mould grid's columns and rows (`grid`; 0 x 0 with none).
    pub fn grid_size(&self) -> (usize, usize) {
        match self.grid as usize {
            0 => (0, 0),
            g => (self.dish.w.div_ceil(g), self.dish.h.div_ceil(g)),
        }
    }

    /// The most squares the mould grid has room for (a dish cell each).
    pub fn grid_max(&self) -> usize {
        (self.wstat.size() as usize - WSTAT_BYTES) / 4
    }

    /// The squad's frame (Larder World W2a; `squad.rs` has the CPU twins and
    /// the order): the next `record` runs its shots, strikes and smell before
    /// its first tick, and its reads and harvest after its last. Shots need
    /// growth on (a struck walker's slot is emptied). `squad_out` reads the
    /// results back.
    pub fn set_squad(&mut self, f: &squad::Frame) {
        assert!(f.shots.is_empty() || self.grows, "shots kill by emptying slots: growth must be on");
        let (words, nunits, nshots) = squad::pack(self.dish.w, self.dish.h, f);
        self.queue.write_buffer(&self.sq, 0, &u32_bytes(&words));
        // The gardeners' entries (`GpuWorld::tend` set them; taken here, so
        // a frame without tending writes none).
        let tends = std::mem::take(&mut self.tends);
        assert!(tends.len() <= squad::TEND_MAX, "{} tend entries", tends.len());
        if !tends.is_empty() {
            let flat: Vec<u32> = tends.iter().flatten().copied().collect();
            self.queue.write_buffer(&self.sq, (squad::TEND * 4) as u64, &u32_bytes(&flat));
        }
        self.ntend = tends.len() as u32;
        self.queue.write_buffer(&self.kills, 0, &[0u8; squad::SHOTS_MAX * 4]);
        self.squad = (nunits, nshots, f.slime);
        self.squad_due = true;
        self.queue.write_buffer(&self.params, SQUAD_AT as u64, &u32_bytes(&[nunits, nshots, f.slime.to_bits(), squad::SHOT_ALARM.to_bits()]));
        // The sink squared, as `squad::sink_r2` multiplies it (0 off).
        self.shot_sink = if f.sink > 0.0 { f.sink * f.sink } else { 0.0 };
        self.queue.write_buffer(&self.params, SQUAD_AT as u64 + 28, &u32_bytes(&[self.shot_sink.to_bits()]));
        // The spray (gun look, 2026-11-15). A droplet is a flung spore, so
        // the life states go on with the first frame that has shots, as
        // `squad::shots` turns the CPU's on.
        self.spray = if f.spray > 0 { (f.spray as u32, f.spray_speed, crate::mould::big::spray_share_q(f.spray_share)) } else { (0, 0.0, 65536) };
        if self.spray.0 > 0 && nshots > 0 {
            self.cycles = true;
            if self.dish.flung.is_empty() {
                self.dish.flung = vec![false; self.n as usize];
            }
            if self.dish.doomed.is_empty() {
                self.dish.doomed = vec![false; self.n as usize];
            }
        }
        self.queue.write_buffer(&self.params, SPRAY_AT as u64, &u32_bytes(&self.spray_row()));
        self.crater_steps = f.crater as u32;
        self.scorched |= f.crater > 0;
        self.queue.write_buffer(&self.params, CRATER_AT as u64 + 8, &u32_bytes(&[(f.crater as f32).to_bits(), self.crater_flags()]));
    }

    /// The last squad frame's results (`set_squad`), read back.
    /// The crater row's flag word: bit 0 `scorched` (the trail pass counts
    /// down and cuts from then on), bit 1 the cone (`BigParams::
    /// scorch_cone`, read by the `cut` pass).
    fn crater_flags(&self) -> u32 {
        self.scorched as u32 | (self.dish.p.scorch_cone as u32) << 1
    }

    pub fn squad_out(&self) -> squad::FrameOut {
        let words: Vec<u32> = read_back(&self.device, &self.queue, &self.sq).chunks_exact(4).take(squad::PATH).map(|b| u32::from_le_bytes(b.try_into().unwrap())).collect();
        let kills: Vec<u32> = read_back(&self.device, &self.queue, &self.kills).chunks_exact(4).map(|b| u32::from_le_bytes(b.try_into().unwrap())).collect();
        squad::unpack(self.dish.w, &words, &kills, self.squad.0 as usize, self.squad.1 as usize)
    }

    /// The Larder World read back: each pile's charge and cells still
    /// crystal (from the last tick's sum), and walls' bites and cells
    /// broken so far.
    pub fn world_counts(&self) -> (Vec<(f32, u32)>, u32, u32) {
        let piles = read_back(&self.device, &self.queue, &self.piles);
        let word = |b: &[u8], i: usize| u32::from_le_bytes(b[4 * i..4 * i + 4].try_into().unwrap());
        let sums = (0..self.npiles as usize).map(|p| (f32::from_bits(word(&piles, p * 8 + 4)), word(&piles, p * 8 + 2))).collect();
        let st = read_back(&self.device, &self.queue, &self.wstat);
        (sums, word(&st, 0), word(&st, 1))
    }

    pub fn picture(&self) -> image::RgbaImage {
        let bytes = read_back(&self.device, &self.queue, &self.frame);
        image::RgbaImage::from_raw(self.dish.w as u32, self.dish.h as u32, bytes).expect("frame size")
    }
}

/// The living walkers a cell of a CPU dish (its `occ`).
pub fn occupancy(d: &BigDish) -> Vec<u32> {
    // The CPU's own counts: with the life cycle spores and the dead hold
    // no cell, so counting the living walkers' places (as before
    // 2026-11-11) counted every spore in flight too.
    d.occupancy().iter().map(|&c| c as u32).collect()
}

/// Each slot's energy as the GPU keeps it: -1 for an empty slot (0s with
/// growth off, never read).
/// Each slot's `spec` word: its species (0 with none), with the mix on its
/// kind in bit 8, and with the life cycle its state in bits 9-10 and its
/// timer in bits 16-31 (engine.wgsl's `L_*`).
pub fn spec_words(d: &BigDish) -> Vec<u32> {
    (0..d.walkers.len())
        .map(|k| {
            d.spec.get(k).map_or(0, |&c| c as u32) | d.kind.get(k).map_or(0, |&c| (c as u32) << 8) | d.life.get(k).map_or(0, |&c| (c as u32) << 9) | d.life_t.get(k).map_or(0, |&c| (c as u32) << 16) | d.flung.get(k).map_or(0, |&f| (f as u32) << 13) | d.doomed.get(k).map_or(0, |&f| (f as u32) << 14)
        })
        .collect()
}

/// Food's news a CPU dish starts the GPU's with (binding 48): each slot's
/// step it last heard of food (`BigDish::news`), or the next step for all
/// before its first `hear_food`, as the CPU starts them; then the three
/// blocks of cells, clear.
/// Words before the slots in the sleep gas's words (after food's news,
/// `engine.wgsl`'s `GAS_HEAD`, `GAS_UNITS`, `GAS_CALM`, `GAS_SLOTS`).
pub const GAS_HEAD: usize = 8;
pub const GAS_SLOTS: usize = GAS_HEAD + 5 * squad::UNITS_MAX;

/// The sleep gas's buffer a CPU dish starts the GPU's with: a header (zero but words 4 and 5, the cloud's linger and sleep level)
/// and calm block, then each slot's step its gas runs out at (the dish's
/// step plus its `gassed`; 0, never, where it has none).
/// Since 2026-11-16 the Mind words follow, one a slot (`BigDish::mind`,
/// `engine.wgsl`'s `mind_of`), so the block is `GAS_SLOTS` + 2 x slots.
pub fn gas_words(d: &BigDish) -> Vec<u32> {
    let n = d.walkers.len();
    let mut w = vec![0u32; GAS_SLOTS + 2 * n];
    if !d.mind.is_empty() {
        w[GAS_SLOTS + n..].copy_from_slice(&d.mind);
    }
    w[4] = d.p.gas_linger as u32;
    w[5] = d.p.gas_sleep.to_bits();
    for (k, &g) in d.gassed.iter().enumerate() {
        if g > 0 {
            w[GAS_SLOTS + k] = d.steps as u32 + g as u32;
        }
    }
    w
}

/// The last step any of a dish's Mind statuses lasts to (its words hold
/// the low 29 bits of their end step, `MIND_T`), 0 when none lasts.
fn mind_until(d: &BigDish) -> u64 {
    use crate::mould::big::{mind_kind, MIND_T};
    let now = d.steps as u32 & MIND_T;
    // None lasting: 0 (`minds_on` then reads at most at step 0, harmless).
    let left = d.mind.iter().filter(|&&w| mind_kind(w, d.steps) != 0).map(|&w| (w & MIND_T).wrapping_sub(now) & MIND_T).max();
    left.map_or(0, |l| d.steps + l as u64)
}

/// The last step any of a dish's gas lasts to.
fn gas_until(d: &BigDish) -> u64 {
    d.steps + d.gassed.iter().copied().max().unwrap_or(0) as u64
}

/// Dense slime's words a CPU dish starts the GPU's with (binding 48, after
/// the sleep gas's; `engine.wgsl`'s `dense_base`): a header of
/// `DENSE_HEAD` words, each slot's word (`BigDish::dense`, 0 while it has
/// none), then a word a cell, clear (free since 11-17: condensed walkers
/// a cell), the raid's field and the lumps' bite cells.
pub const DENSE_HEAD: usize = 16;

pub fn dense_words(d: &BigDish, cells: usize) -> Vec<u32> {
    let mut w = vec![0u32; DENSE_HEAD];
    w.extend((0..d.walkers.len()).map(|k| d.dense.get(k).copied().unwrap_or(0)));
    // The units' walking distance in the cells' word (free since 11-17;
    // `BigDish::units_field`, S1; engine.wgsl's `units_dist`): each block
    // its distance + 1, 0 where none reaches or with no field.
    // The far smell's (`BigDish::food_field`, S2; `food_dist`) follows in
    // the same region, a block's words on: blocks are a quarter of the
    // cells, so both fit. The lure's field (`Engine::write_lure_field`,
    // `lure_dist`) is the third quarter, clear here: a CPU dish has none.
    w.extend(block_words(&d.units_field, d.w, d.h));
    w.extend(block_words(&d.food_field, d.w, d.h));
    w.resize(DENSE_HEAD + d.walkers.len() + cells, 0);
    // A raid's walking distance after the dense cells (`BigDish::
    // raid_field`, 11-15; engine.wgsl's `raid_dist`), zeros with none.
    // Blocks (`RAID_BLOCK`), fewer than the cells the region holds.
    w.extend(d.raid_field.iter().take(cells));
    w.resize(DENSE_HEAD + d.walkers.len() + 2 * cells, 0);
    // Each cell's lump members' bite (`squad::bite_cells`; `dcount`).
    w.resize(DENSE_HEAD + d.walkers.len() + 3 * cells, 0);
    w
}

/// A walking-distance field (`BigDish::units_field`, `food_field`) as the
/// GPU holds it: one word a `RAID_BLOCK` block, the distance + 1, 0 where
/// none reaches or with no field.
fn block_words(field: &[u32], w: usize, h: usize) -> Vec<u32> {
    let b = crate::mould::big::RAID_BLOCK;
    let nb = w.div_ceil(b) * h.div_ceil(b);
    let mut v: Vec<u32> = field.iter().take(nb).map(|&d| d.wrapping_add(1)).collect();
    v.resize(nb, 0);
    v
}

/// **The heartbeat** (2026-11-20; `Engine::heart`).
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct Heart {
    /// Where it beats from, fine cells.
    pub at: (f32, f32),
    /// A calm walker's lean outward on the beat (and home between).
    pub lean: f32,
    /// Steps a beat, and its wavelength in fine cells.
    pub period: u32,
    pub len: u32,
}

/// Each slot's condensing word as the GPU holds it (`BigDish::dense`).
impl Engine {
    /// Writes the mirror dish's raid walking distance (`BigDish::
    /// raid_field`) to `news` after the dense cells; zeros with none.
    ///
    /// Only its blocks (`RAID_BLOCK`, a quarter of the region): the
    /// region's second quarter is the wave's field (`write_wave_field`,
    /// 2026-11-20), which a raid must not clear.
    pub fn write_raid_field(&self) {
        let cells = self.cells as usize;
        let b = crate::mould::big::RAID_BLOCK;
        let nb = (self.dish.w.div_ceil(b) * self.dish.h.div_ceil(b)).min(cells);
        let at = self.dense_at + 4 * (DENSE_HEAD + self.n as usize + cells) as u64;
        let mut w: Vec<u32> = self.dish.raid_field.iter().copied().take(nb).collect();
        w.resize(nb, 0);
        self.queue.write_buffer(&self.news, at, &u32_bytes(&w));
    }

    /// Writes the mirror dish's units' walking distance (`BigDish::
    /// units_field`, S1) after the slot words, each block's distance + 1
    /// (0 where none reaches); zeros with none. One word a block (a
    /// quarter of the region): the first build wrote the whole region,
    /// a word a fine cell, 4x the bytes once a second.
    pub fn write_units_field(&self) {
        let at = self.dense_at + 4 * (DENSE_HEAD + self.n as usize) as u64;
        self.queue.write_buffer(&self.news, at, &u32_bytes(&block_words(&self.dish.units_field, self.dish.w, self.dish.h)));
    }

    /// Writes the mirror dish's far smell (`BigDish::food_field`, S2)
    /// after the units' walking distance, one word a block as that.
    pub fn write_food_field(&self) {
        let b = crate::mould::big::RAID_BLOCK;
        let nb = self.dish.w.div_ceil(b) * self.dish.h.div_ceil(b);
        let at = self.dense_at + 4 * (DENSE_HEAD + self.n as usize + nb) as u64;
        self.queue.write_buffer(&self.news, at, &u32_bytes(&block_words(&self.dish.food_field, self.dish.w, self.dish.h)));
    }

    /// **Writes a lure's field** (engine.wgsl's `lure_dist`; 2026-11-18,
    /// the Enemy lane): each block's walking distance to the nearest lure
    /// (`big::units_field_for`'s blocks, `u32::MAX` where none reaches),
    /// after the far smell's in the cells' region, one word a block as
    /// that. An empty field clears it. The mirror dish holds no copy: the
    /// rule has no CPU twin. Nothing is written while there was no lure
    /// and there is none (the game calls this once a game second).
    pub fn write_lure_field(&mut self, field: &[u32]) {
        let reaches = field.iter().any(|&d| d != u32::MAX);
        if !reaches && !self.lured {
            return;
        }
        let b = crate::mould::big::RAID_BLOCK;
        let nb = self.dish.w.div_ceil(b) * self.dish.h.div_ceil(b);
        // The third quarter of the region; a dish of odd size (none of
        // the game's: maps are multiples of 4) has no room for it.
        if 3 * nb > self.cells as usize {
            return;
        }
        let at = self.dense_at + 4 * (DENSE_HEAD + self.n as usize + 2 * nb) as u64;
        self.queue.write_buffer(&self.news, at, &u32_bytes(&block_words(field, self.dish.w, self.dish.h)));
        self.lured = reaches;
    }

    /// **Writes the slime's wave field** (engine.wgsl's `wave_dist`;
    /// 2026-11-20, `Game::time_waves`): each block's walking distance to
    /// the nearest of the wave's points (`big::units_field_for`'s blocks,
    /// `u32::MAX` where none reaches), in the second quarter of the raid's
    /// region (after the raid's own blocks), and the pull a calm walker
    /// leans down it with. An empty field clears it. No CPU twin (the
    /// owner's card, 11-17 19:21Z). Nothing is written while there was no
    /// wave and there is none.
    pub fn write_wave_field(&mut self, field: &[u32], pull: f32) {
        let reaches = field.iter().any(|&d| d != u32::MAX);
        if !reaches && !self.waved {
            return;
        }
        let b = crate::mould::big::RAID_BLOCK;
        let nb = self.dish.w.div_ceil(b) * self.dish.h.div_ceil(b);
        if 2 * nb > self.cells as usize {
            return;
        }
        let at = self.dense_at + 4 * (DENSE_HEAD + self.n as usize + self.cells as usize + nb) as u64;
        self.queue.write_buffer(&self.news, at, &u32_bytes(&block_words(field, self.dish.w, self.dish.h)));
        self.waved = reaches;
        self.wave_w = pull;
    }

    /// **Writes the fort's mask** (engine.wgsl's `fort_at`; 2026-11-18,
    /// the Enemy lane): a word a `RAID_BLOCK` block, row by row, not 0
    /// where a walker sets with no scar, after the lure's field in the
    /// cells' region (its fourth quarter). An empty or all-zero mask
    /// clears it. The mirror dish holds no copy: the rule has no CPU
    /// twin. Nothing is written while there was no fort and there is
    /// none.
    pub fn write_fort(&mut self, mask: &[u32]) {
        let any = mask.iter().any(|&m| m != 0);
        if !any && !self.forted {
            return;
        }
        let b = crate::mould::big::RAID_BLOCK;
        let nb = self.dish.w.div_ceil(b) * self.dish.h.div_ceil(b);
        // The fourth quarter of the region; a dish of odd size (none of
        // the game's: maps are multiples of 4) has no room for it.
        if 4 * nb > self.cells as usize {
            return;
        }
        let at = self.dense_at + 4 * (DENSE_HEAD + self.n as usize + 3 * nb) as u64;
        let mut words: Vec<u32> = mask.iter().copied().take(nb).collect();
        words.resize(nb, 0);
        self.queue.write_buffer(&self.news, at, &u32_bytes(&words));
        self.forted = any;
    }

    /// Forms lump `b` (the CPU's `BigDish::form_lump`, `lumpform`): the
    /// ordinary walkers within reach squared `r2` (whole fine cells) of
    /// fine cell `at` (x | y << 16) join it; None releases it, every slot
    /// (a spore in flight too). On its own,
    /// between ticks.
    pub fn form_lump(&self, shared: &Shared, b: usize, at: Option<(u32, u32)>) {
        assert!(b < crate::mould::big::LUMPS, "lump {b}");
        let (c, r2) = at.unwrap_or((u32::MAX, 0));
        self.queue.write_buffer(&self.news, self.dense_at + 4, &u32_bytes(&[b as u32 + 1, c, r2]));
        self.run(shared, &[(LUMPFORM, self.n.div_ceil(256))]);
    }

    /// Lump `b` takes in the ordinary walkers within `at` in no lump, its
    /// members kept (the CPU's `grow_lump`): `lumpform` with bit 8 of the
    /// lump word set.
    pub fn grow_lump(&self, shared: &Shared, b: usize, at: (u32, u32)) {
        assert!(b < crate::mould::big::LUMPS, "lump {b}");
        self.queue.write_buffer(&self.news, self.dense_at + 4, &u32_bytes(&[(b as u32 + 1) | 1 << 8, at.0, at.1]));
        self.run(shared, &[(LUMPFORM, self.n.div_ceil(256))]);
    }

    /// **Condenses the edge facing the squad** (2026-11-20, engine.wgsl's
    /// `condense` and `condlead`; `Game::time_strikers`), at once between
    /// steps: each ordinary walker within `reach` fine cells of a soldier
    /// (walking, where the units' walking distance is kept) tries with
    /// chance `share`, and in each block the first takes in up
    /// to three of the rest. Needs `strikers` on (and the params written
    /// with it). Returns the condensed walkers living before it ran, what
    /// they held, and the walkers it took in. GPU only (the owner's 11-17
    /// 19:21Z card): no CPU twin.
    pub fn condense(&mut self, shared: &Shared, reach: f32, share: f32, most: u32) -> (u32, u32, u32) {
        let Some(haste) = self.strikers else { return (0, 0, 0) };
        let b = crate::mould::big::RAID_BLOCK;
        let nb = self.dish.w.div_ceil(b) * self.dish.h.div_ceil(b);
        // Its words are the raid region's third quarter.
        if 3 * nb > self.cells as usize {
            return (0, 0, 0);
        }
        // Bit 31: by the units' walking distance, where it is kept.
        let walk = if self.dish.units_field.is_empty() { 0 } else { 1 << 31 };
        let r2 = ((reach.max(0.0) * reach.max(0.0)) as u32).min(0xFF_FFFF) | most.clamp(1, 15) << 24 | walk;
        self.queue.write_buffer(&self.params, LUMP_AT as u64, &u32_bytes(&[1, haste.to_bits(), r2, share.clamp(0.0, 1.0).to_bits()]));
        self.queue.write_buffer(&self.wstat, 32, &[0u8; 12]);
        self.run(shared, &[(CONDENSE, self.n.div_ceil(256)), (CONDLEAD, (nb as u32).div_ceil(256))]);
        let w = read_back_range(&self.device, &self.queue, &self.wstat, 32, 12);
        let v: Vec<u32> = w.chunks_exact(4).map(|c| u32::from_le_bytes(c.try_into().unwrap())).collect();
        // What they hold now: what they held, and what this pass took in.
        self.striker_held = v[1] + v[2];
        (v[0], v[1], v[2])
    }

    /// Each lump's (x, y, members) sums as the last tick began (header
    /// words 4-15; the CPU's `lump_sums`), read back.
    pub fn lump_sums(&self) -> [(u32, u32, u32); crate::mould::big::LUMPS] {
        let b = read_back(&self.device, &self.queue, &self.news);
        let at = self.dense_at as usize + 16;
        let w: Vec<u32> = b[at..at + 48].chunks_exact(4).map(|c| u32::from_le_bytes(c.try_into().unwrap())).collect();
        std::array::from_fn(|i| (w[3 * i], w[3 * i + 1], w[3 * i + 2]))
    }

    pub fn dense_of(&self) -> Vec<u32> {
        let b = read_back(&self.device, &self.queue, &self.news);
        let at = self.dense_at as usize + 4 * DENSE_HEAD;
        b[at..at + 4 * self.n as usize].chunks_exact(4).map(|c| u32::from_le_bytes(c.try_into().unwrap())).collect()
    }
}

/// The plants' words (engine.wgsl's `plant_base` on): `PLANT_HEAD` words of
/// numbers, then seven blocks a cell (soil capacity, 0 on crystals; kind;
/// plants A and B; fire A and B; char), or none without plants. Rates are
/// a plant step's, as the CPU's `plant_rates` and `fire_rates` give them.
pub const PLANT_HEAD: usize = 64;
pub fn plant_words(d: &BigDish) -> Vec<u32> {
    let mut w = vec![0u32; PLANT_HEAD];
    if d.plant.is_empty() {
        return w;
    }
    let p = &d.p;
    let k = p.plant_every.max(1) as f32;
    let head = [p.plant_grow * k, p.plant_spread * k, p.plant_wet, (p.plant_scorch * k).min(1.0), p.fire_heat, p.fire_eat, p.fire_spread, p.fire_fuel, p.fire_damp, p.fire_kill, p.char_gain, p.char_keep, p.char_clear, p.fire_out, p.fire_smoulder, p.fire_cool];
    for (o, v) in head.iter().enumerate() {
        w[o] = v.to_bits();
    }
    for (q, kd) in p.plant_kinds.iter().enumerate() {
        (w[16 + q], w[20 + q], w[24 + q], w[28 + q]) = (kd.cap.to_bits(), kd.pace.to_bits(), kd.dry.to_bits(), kd.burn.to_bits());
    }
    // Word 45: fire steps run (`BigDish::fire_steps`), which keys each
    // cell's chance; the `firekill` pass counts it on.
    w[45] = d.fire_steps;
    // Words 46-47: fire's flicker and ember keep.
    (w[46], w[47]) = (p.fire_flicker.to_bits(), p.fire_ember.to_bits());
    // Word 48: the plants at which a fire can burn at full heat.
    w[48] = p.fire_full.to_bits();
    // Word 44: the alarm fire lays a step (`fire_alarm`, C4); word 43 the
    // gas that burns (`gas_burn`, C5). (Words 32-42 are free; 49-61 hold
    // `Engine::ignite`'s and `lay`'s discs.)
    w[44] = p.fire_alarm.to_bits();
    w[43] = p.gas_burn.to_bits();
    // Word 62: what plants over a cell's capacity (laid fuel, Tinder) lose
    // a plant step (the CPU's `Rates::rot`).
    w[62] = (p.plant_rot * k).to_bits();
    let c = d.w * d.h;
    let crystal = |i: usize| !d.pile_of.is_empty() && d.pile_of[i] != 0;
    w.extend((0..c).map(|i| if crystal(i) { 0.0f32 } else { d.plant_cap[i] }.to_bits()));
    w.extend(d.plant_kind.iter().map(|&k| k as u32));
    w.extend(d.plant.iter().map(|v| v.to_bits()));
    w.extend(std::iter::repeat_n(0u32, c));
    let fire = |v: &[f32]| if v.is_empty() { vec![0u32; c] } else { v.iter().map(|x| x.to_bits()).collect() };
    w.extend(fire(&d.fire));
    w.extend(std::iter::repeat_n(0u32, c));
    w.extend(fire(&d.char));
    // Block 7: moisture (`plants::moisture`).
    w.extend(fire(&d.moist));
    // Block 8: how fresh each cell's fire is (engine.wgsl's `PB_FRESH`,
    // for the view's advancing edge; the CPU keeps no copy).
    w.extend(std::iter::repeat_n(0u32, c));
    w
}

pub fn news_words(d: &BigDish, cells: usize) -> Vec<u32> {
    let mut words: Vec<u32> = if d.news.len() == d.walkers.len() { d.news.iter().map(|&t| t as u32).collect() } else { vec![d.steps as u32 + 1; d.walkers.len()] };
    words.resize(d.walkers.len() + 3 * cells, 0);
    words
}

/// Withering's clock in one word (the life-cycle row's spare word 732,
/// borrowed by the slime-enemy thread, 2026-11-13): `wither_lag` in the
/// low 16 bits, `wither_every` in the next 8, and whether the spread ends
/// in its second block (`wither_reach` even) in bit 24.
pub fn wither_clock(s: &Settings) -> u32 {
    let reach = s.wither_reach.max(1) as u32;
    // Bits 25-31: a cut-off part's lean (`panic_lean`, 2026-11-13).
    s.wither_lag.min(0xFFFF) | (s.wither_every.clamp(1, 255) << 16) | ((reach.is_multiple_of(2) as u32) << 24) | (s.panic_lean.min(crate::mould::big::PANIC_LEAN_MAX) << 25)
}

pub fn energies(d: &BigDish) -> Vec<f32> {
    if d.energy.is_empty() {
        return vec![0.0; d.walkers.len()];
    }
    d.energy.iter().zip(&d.alive).map(|(&e, &a)| if a { e } else { -1.0 }).collect()
}

/// The grass a CPU dish starts the GPU's with: its own, or with none full
/// on the agar (so grazing can be switched on as the dish runs), but none
/// with crystals on, where the CPU makes no grass unless grazing is set
/// (growth would otherwise eat grass the CPU does not have).
pub fn grass_of(d: &BigDish) -> Vec<f32> {
    if !d.grass.is_empty() {
        d.grass.clone()
    } else if d.p.crystal_charge > 0.0 {
        vec![0.0; d.w * d.h]
    } else {
        d.agar.iter().map(|&a| a as u32 as f32).collect()
    }
}

/// A CPU dish's Larder World as the GPU holds it (engine.wgsl, bindings
/// 34-39): ground bits, charge and health a cell, the crystal cells pile
/// by pile (`BigDish::pile_cells`), and each pile's run and sums, 8 words
/// a pile for `MAX_PILES` piles.
pub struct WorldData {
    pub ground: Vec<u32>,
    pub charge: Vec<f32>,
    pub health: Vec<f32>,
    pub pcell: Vec<u32>,
    pub piles: Vec<u32>,
    /// Piles with crystal (the dish's piles with crystals on, else 0).
    pub npiles: u32,
}

/// The world bits a dish's ground needs drawn and run whatever its
/// params: terrain if it has rock or wet, the wet pass if wet, soil if any
/// cell is sand, clay or barren.
pub fn ground_bits(d: &BigDish) -> u32 {
    let wet = d.wet.iter().any(|&v| v);
    let rock = d.rock.iter().any(|&v| v);
    let soil = d.soil.iter().any(|&k| k != 0);
    (if wet { WORLD_WETS } else { 0 }) | (if wet || rock { WORLD_TERRAIN } else { 0 }) | (if soil { WORLD_SOIL } else { 0 })
}

pub fn world_data(d: &BigDish) -> WorldData {
    let cells = d.w * d.h;
    let on = |v: &[bool], i: usize| !v.is_empty() && v[i];
    // The scent grid is the fine one here (`Engine::inject` checks), so
    // `scent_wall` is a cell's.
    let ground = (0..cells)
        .map(|i| {
            let mut g = if d.pile_of.is_empty() { 0 } else { d.pile_of[i] as u32 & GROUND_PILE };
            if on(&d.wet, i) {
                g |= GROUND_WET;
            }
            if on(&d.rock, i) {
                g |= GROUND_ROCK;
            }
            if !d.soil.is_empty() {
                g |= (d.soil[i] as u32) << GROUND_SOIL_SHIFT & GROUND_SOIL;
            }
            if on(&d.wall, i) {
                g |= GROUND_WALL;
                if on(&d.scent_wall, i) {
                    g |= GROUND_SCENT_WALL;
                }
            }
            g
        })
        .collect();
    let charge = if d.charge.is_empty() { vec![0.0; cells] } else { d.charge.clone() };
    // A wall built with `wall_hp` 0 never breaks: infinite health.
    let health = (0..cells).map(|i| if !on(&d.wall, i) { 0.0 } else if d.wall_health.is_empty() { f32::INFINITY } else { d.wall_health[i] }).collect();
    let mut piles = vec![0u32; MAX_PILES * 8];
    for p in 0..MAX_PILES {
        piles[p * 8 + 5] = 1f32.to_bits();
    }
    let npiles = if d.charge.is_empty() { 0 } else { d.piles.len().min(MAX_PILES) };
    // Each pile's run of the list: the cells still in it bound it (one a
    // wall took out of its pile stays listed, skipped by its 0).
    let (mut start, mut end) = (vec![u32::MAX; MAX_PILES], vec![0u32; MAX_PILES]);
    for (q, &i) in d.pile_cells.iter().enumerate() {
        let p = d.pile_of[i as usize] as usize;
        if p > 0 && p <= MAX_PILES {
            start[p - 1] = start[p - 1].min(q as u32);
            end[p - 1] = end[p - 1].max(q as u32 + 1);
        }
    }
    for p in 0..npiles {
        if start[p] != u32::MAX {
            piles[p * 8] = start[p];
            piles[p * 8 + 1] = end[p] - start[p];
        }
        // The pile's centre, a panicking walker's target (`pile_centre`).
        piles[p * 8 + 3] = d.pile_centre(p);
        piles[p * 8 + 4] = d.pile_charge[p].to_bits();
        piles[p * 8 + 5] = d.crystal_left(p).to_bits();
        piles[p * 8 + 6] = d.pile_fill[p].to_bits();
    }
    WorldData { ground, charge, health, pcell: d.pile_cells.clone(), piles, npiles: npiles as u32 }
}

thread_local! {
    static READS: std::cell::Cell<usize> = const { std::cell::Cell::new(0) };
}

/// Blocking read-backs this thread has made (`read_back`), so a run or a
/// test can say how often it waited on the GPU (`larder-world -- bench`).
pub fn reads() -> usize {
    READS.with(|r| r.get())
}

thread_local! {
    static BYTES: std::cell::Cell<(u64, u64)> = const { std::cell::Cell::new((0, 0)) };
}

/// Counts `n` bytes of a storage buffer made for the world (the engine's,
/// the view's, the canvas's, the ground look's), for `buffer_bytes`.
pub fn note_bytes(n: u64) {
    BYTES.with(|b| b.set((b.get().0 + n, b.get().1.max(n))));
}

/// Bytes of the world's storage buffers this thread has made: the cell
/// budget's measure as planets grow (levelgen report, *Bigger planets*;
/// `larder-world -- bench` prints it). A count at creation, not an
/// allocator report, which wgpu does not give on every backend; screen
/// buffers and textures the size of the window are not in it.
pub fn buffer_bytes() -> u64 {
    BYTES.with(|b| b.get().0)
}

/// The biggest one of them, bytes: the buffer that meets the adapter's
/// `max_storage_buffer_binding_size` first.
pub fn biggest_buffer() -> u64 {
    BYTES.with(|b| b.get().1)
}

/// `src` copied to a new staging buffer and waited for: it empties the
/// GPU's queue, so the game's step reads through `GpuWorld`'s one packed
/// readout instead (2026-11-12).
pub fn read_back(device: &wgpu::Device, queue: &wgpu::Queue, src: &wgpu::Buffer) -> Vec<u8> {
    READS.with(|r| r.set(r.get() + 1));
    let size = src.size();
    let staging = device.create_buffer(&wgpu::BufferDescriptor { label: Some("read back"), size, usage: wgpu::BufferUsages::MAP_READ | wgpu::BufferUsages::COPY_DST, mapped_at_creation: false });
    let mut enc = device.create_command_encoder(&wgpu::CommandEncoderDescriptor { label: None });
    enc.copy_buffer_to_buffer(src, 0, &staging, 0, size);
    queue.submit([enc.finish()]);
    staging.map_async(wgpu::MapMode::Read, .., |r| r.expect("map"));
    device.poll(wgpu::PollType::wait_indefinitely()).expect("poll");
    let bytes = staging.get_mapped_range(..).to_vec();
    staging.unmap();
    bytes
}

/// `len` bytes of `src` from `at` (both multiples of 4), as `read_back`:
/// for a few words of a big buffer. The gas and fling counts and `calm`
/// read the whole `news` buffer first, which holds three words a dish cell
/// and three a walker: at 6x a planet's area one gas press took 115 ms on
/// software Vulkan and the every-command bench's frames without a step
/// 37.8 ms on the 1070 (2026-11-14), nearly all of it these copies.
pub fn read_back_range(device: &wgpu::Device, queue: &wgpu::Queue, src: &wgpu::Buffer, at: u64, len: u64) -> Vec<u8> {
    READS.with(|r| r.set(r.get() + 1));
    let size = len.max(4).next_multiple_of(4);
    let staging = device.create_buffer(&wgpu::BufferDescriptor { label: Some("read back"), size, usage: wgpu::BufferUsages::MAP_READ | wgpu::BufferUsages::COPY_DST, mapped_at_creation: false });
    let mut enc = device.create_command_encoder(&wgpu::CommandEncoderDescriptor { label: None });
    enc.copy_buffer_to_buffer(src, at, &staging, 0, size);
    queue.submit([enc.finish()]);
    staging.map_async(wgpu::MapMode::Read, .., |r| r.expect("map"));
    device.poll(wgpu::PollType::wait_indefinitely()).expect("poll");
    let bytes = staging.get_mapped_range(..).to_vec();
    staging.unmap();
    bytes[..len as usize].to_vec()
}

/// Chosen 4-byte words of two storage buffers gathered on the GPU into
/// one small buffer and read back in one copy: Larder World's ground
/// refresh reads its crystal and wall cells' charge and health this way,
/// once a game second (2026-11-14: the whole charge buffer read back and
/// a CPU pass over every cell took 23.6 ms at 4x a planet's area and
/// 32.9 ms at 6x on the 1070). A copy per row of cells came first and was
/// dropped: 2,914 copies at 6x took 17-21 ms on software Vulkan, nearly
/// all of the refresh, where this one pass and one copy took 1.3-1.4 ms.
pub struct Gather {
    pipe: wgpu::ComputePipeline,
    bgl: wgpu::BindGroupLayout,
    idx: wgpu::Buffer,
    out: wgpu::Buffer,
    staging: wgpu::Buffer,
    n: u32,
}

const GATHER: &str = r#"
@group(0) @binding(0) var<storage, read> idx: array<u32>;
@group(0) @binding(1) var<storage, read> a: array<u32>;
@group(0) @binding(2) var<storage, read> b: array<u32>;
@group(0) @binding(3) var<storage, read_write> out: array<u32>;

@compute @workgroup_size(64)
fn gather(@builtin(global_invocation_id) id: vec3<u32>, @builtin(num_workgroups) groups: vec3<u32>) {
    let n = arrayLength(&idx);
    let i = id.x + id.y * groups.x * 64u;
    if (i >= n) {
        return;
    }
    out[i] = a[idx[i]];
    out[n + i] = b[idx[i]];
}
"#;

impl Gather {
    /// A gather of the words at `cells` (none: None), with `old`'s
    /// pipeline if there is one (a wall going up renews the cells, and a
    /// pipeline built each time would cost the frame more).
    pub fn new(device: &wgpu::Device, queue: &wgpu::Queue, cells: &[u32], old: Option<&Gather>) -> Option<Self> {
        if cells.is_empty() {
            return None;
        }
        let (pipe, bgl) = match old {
            Some(o) => (o.pipe.clone(), o.bgl.clone()),
            None => Self::pipeline(device),
        };
        let bytes = cells.len() as u64 * 4;
        let idx = device.create_buffer(&wgpu::BufferDescriptor { label: Some("gather cells"), size: bytes, usage: wgpu::BufferUsages::STORAGE | wgpu::BufferUsages::COPY_DST, mapped_at_creation: false });
        queue.write_buffer(&idx, 0, &u32_bytes(cells));
        let out = device.create_buffer(&wgpu::BufferDescriptor { label: Some("gathered"), size: 2 * bytes, usage: wgpu::BufferUsages::STORAGE | wgpu::BufferUsages::COPY_SRC, mapped_at_creation: false });
        let staging = device.create_buffer(&wgpu::BufferDescriptor { label: Some("gathered read"), size: 2 * bytes, usage: wgpu::BufferUsages::MAP_READ | wgpu::BufferUsages::COPY_DST, mapped_at_creation: false });
        note_bytes(3 * bytes);
        Some(Gather { pipe, bgl, idx, out, staging, n: cells.len() as u32 })
    }

    fn pipeline(device: &wgpu::Device) -> (wgpu::ComputePipeline, wgpu::BindGroupLayout) {
        let module = device.create_shader_module(wgpu::ShaderModuleDescriptor { label: Some("gather"), source: wgpu::ShaderSource::Wgsl(GATHER.into()) });
        let entry = |binding: u32, read_only: bool| wgpu::BindGroupLayoutEntry {
            binding,
            visibility: wgpu::ShaderStages::COMPUTE,
            ty: wgpu::BindingType::Buffer { ty: wgpu::BufferBindingType::Storage { read_only }, has_dynamic_offset: false, min_binding_size: None },
            count: None,
        };
        let bgl = device.create_bind_group_layout(&wgpu::BindGroupLayoutDescriptor { label: Some("gather"), entries: &[entry(0, true), entry(1, true), entry(2, true), entry(3, false)] });
        let layout = device.create_pipeline_layout(&wgpu::PipelineLayoutDescriptor { label: Some("gather"), bind_group_layouts: &[Some(&bgl)], immediate_size: 0 });
        let pipe = device.create_compute_pipeline(&wgpu::ComputePipelineDescriptor { label: Some("gather"), layout: Some(&layout), module: &module, entry_point: Some("gather"), compilation_options: Default::default(), cache: None });
        (pipe, bgl)
    }

    /// `a`'s and `b`'s words at the cells, in the cells' order, waited for.
    pub fn read(&self, device: &wgpu::Device, queue: &wgpu::Queue, a: &wgpu::Buffer, b: &wgpu::Buffer) -> (Vec<u8>, Vec<u8>) {
        READS.with(|r| r.set(r.get() + 1));
        let group = device.create_bind_group(&wgpu::BindGroupDescriptor {
            label: Some("gather"),
            layout: &self.bgl,
            entries: &[
                wgpu::BindGroupEntry { binding: 0, resource: self.idx.as_entire_binding() },
                wgpu::BindGroupEntry { binding: 1, resource: a.as_entire_binding() },
                wgpu::BindGroupEntry { binding: 2, resource: b.as_entire_binding() },
                wgpu::BindGroupEntry { binding: 3, resource: self.out.as_entire_binding() },
            ],
        });
        // Workgroups in rows of at most 65,535 (a dispatch's limit).
        let groups = self.n.div_ceil(64);
        let (gx, gy) = (groups.min(65_535), groups.div_ceil(65_535));
        let mut enc = device.create_command_encoder(&wgpu::CommandEncoderDescriptor { label: Some("gather") });
        {
            let mut pass = enc.begin_compute_pass(&wgpu::ComputePassDescriptor { label: Some("gather"), timestamp_writes: None });
            pass.set_pipeline(&self.pipe);
            pass.set_bind_group(0, &group, &[]);
            pass.dispatch_workgroups(gx, gy, 1);
        }
        let bytes = self.n as u64 * 4;
        enc.copy_buffer_to_buffer(&self.out, 0, &self.staging, 0, 2 * bytes);
        queue.submit([enc.finish()]);
        self.staging.map_async(wgpu::MapMode::Read, .., |r| r.expect("map"));
        device.poll(wgpu::PollType::wait_indefinitely()).expect("poll");
        let all = self.staging.get_mapped_range(..).to_vec();
        self.staging.unmap();
        let (x, y) = all.split_at(bytes as usize);
        (x.to_vec(), y.to_vec())
    }
}

/// GPU timestamps read back without stalling: resolved into `read` on a
/// frame when it is free, mapped, and picked up on a later frame.
pub struct Stamps {
    pub qs: wgpu::QuerySet,
    pub resolve: wgpu::Buffer,
    pub read: wgpu::Buffer,
    pub ready: Arc<AtomicBool>,
    pub busy: bool,
    /// Nanoseconds a tick of the GPU's timestamp counter.
    pub period: f64,
    /// Smoothed readings, ms: the ticks, the drawing, and the camera's
    /// view pass (with `view`, else 0).
    pub sim_ms: f64,
    pub draw_ms: f64,
    pub view_ms: f64,
    /// Every reading, for the screenshot hook's median.
    pub all: Vec<(f64, f64, f64)>,
    /// A fourth stamp, at the view pass's end (`world::View::record`).
    pub view: bool,
}

impl Stamps {
    pub fn new(device: &wgpu::Device, queue: &wgpu::Queue) -> Self {
        Self::with_view(device, queue, false)
    }

    /// With `view`, a fourth stamp (index 3) the view pass writes.
    pub fn with_view(device: &wgpu::Device, queue: &wgpu::Queue, view: bool) -> Self {
        let n = 3 + view as u32;
        let qs = device.create_query_set(&wgpu::QuerySetDescriptor { label: Some("frame"), ty: wgpu::QueryType::Timestamp, count: n });
        let resolve = device.create_buffer(&wgpu::BufferDescriptor { label: Some("resolve"), size: n as u64 * 8, usage: wgpu::BufferUsages::QUERY_RESOLVE | wgpu::BufferUsages::COPY_SRC, mapped_at_creation: false });
        let read = device.create_buffer(&wgpu::BufferDescriptor { label: Some("stamps"), size: n as u64 * 8, usage: wgpu::BufferUsages::MAP_READ | wgpu::BufferUsages::COPY_DST, mapped_at_creation: false });
        Stamps { qs, resolve, read, ready: Arc::new(AtomicBool::new(false)), busy: false, period: queue.get_timestamp_period() as f64, sim_ms: 0.0, draw_ms: 0.0, view_ms: 0.0, all: Vec::new(), view }
    }

    /// The query set to write this frame, if the last reading is in.
    pub fn take(&self) -> Option<&wgpu::QuerySet> {
        (!self.busy).then_some(&self.qs)
    }

    pub fn resolve(&self, enc: &mut wgpu::CommandEncoder) {
        let n = 3 + self.view as u32;
        enc.resolve_query_set(&self.qs, 0..n, &self.resolve, 0);
        enc.copy_buffer_to_buffer(&self.resolve, 0, &self.read, 0, n as u64 * 8);
    }

    /// After the submit: ask for the reading.
    pub fn map(&mut self) {
        let ready = Arc::clone(&self.ready);
        self.read.map_async(wgpu::MapMode::Read, .., move |r| {
            if r.is_ok() {
                ready.store(true, Ordering::Release);
            }
        });
        self.busy = true;
    }

    /// Picks up a reading if one has arrived.
    pub fn poll(&mut self) {
        if !self.busy || !self.ready.load(Ordering::Acquire) {
            return;
        }
        let t: Vec<u64> = self.read.get_mapped_range(..).chunks_exact(8).map(|c| u64::from_le_bytes(c.try_into().unwrap())).collect();
        self.read.unmap();
        self.ready.store(false, Ordering::Release);
        self.busy = false;
        let ms = |a: u64, b: u64| b.saturating_sub(a) as f64 * self.period / 1e6;
        let (sim, draw) = (ms(t[0], t[1]), ms(t[1], t[2]));
        let view = if self.view { ms(t[2], t[3]) } else { 0.0 };
        // Smoothed, so the panel's number can be read; seeded with the
        // first reading (it started at 0 until the review of 2026-11-11, so
        // early screenshot figures read low).
        let a = if self.all.is_empty() { 1.0 } else { 0.1 };
        self.sim_ms += a * (sim - self.sim_ms);
        self.draw_ms += a * (draw - self.draw_ms);
        self.view_ms += a * (view - self.view_ms);
        if self.all.len() < 100_000 {
            self.all.push((sim, draw, view));
        }
    }

    /// The median reading so far: the ticks, the drawing and the view, ms,
    /// and how many readings.
    pub fn median(&self) -> (f64, f64, f64, usize) {
        let mid = |mut v: Vec<f64>| {
            v.sort_by(|a, b| a.partial_cmp(b).unwrap());
            v.get(v.len() / 2).copied().unwrap_or(0.0)
        };
        (mid(self.all.iter().map(|r| r.0).collect()), mid(self.all.iter().map(|r| r.1).collect()), mid(self.all.iter().map(|r| r.2).collect()), self.all.len())
    }
}

pub fn block_on<F: std::future::Future>(f: F) -> F::Output {
    use std::task::{Context, Poll, Wake, Waker};
    struct Unpark(std::thread::Thread);
    impl Wake for Unpark {
        fn wake(self: Arc<Self>) {
            self.0.unpark();
        }
    }
    let waker = Waker::from(Arc::new(Unpark(std::thread::current())));
    let mut cx = Context::from_waker(&waker);
    let mut f = std::pin::pin!(f);
    loop {
        if let Poll::Ready(v) = f.as_mut().poll(&mut cx) {
            return v;
        }
        std::thread::park();
    }
}
