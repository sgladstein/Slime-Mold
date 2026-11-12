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

/// The life cycle's (big.rs `DRAW_FRUIT` and on).
pub const DRAW_FRUIT: u32 = 114;
pub const DRAW_SLEEP: u32 = 115;
pub const CLAIM_LAND: u32 = 116;
pub const DRAW_SPORE_H: u32 = 117;
pub const DRAW_SLEEP_DIE: u32 = 118;

/// Bytes in `Params` (220 words).
pub const PARAMS_BYTES: usize = 880;
/// Where one body's row sits in `Params` (2026-11-12, after the soil row:
/// `energy_flow`, `hunger_scent`, `hunger_at`).
pub const BODY_AT: usize = 848;
/// Where the crater's row sits in `Params` (2026-11-12, after one body's:
/// `scorch_keep`, `scorch_full`, the steps a shot's crater lasts, and
/// whether the crater counts down at all). `set_squad` writes its last two
/// words alone, as it does the squad's.
pub const CRATER_AT: usize = 864;
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

/// The wall counters' first bytes (`Engine::wstat`): bites, broken cells,
/// the eviction's moves, the struck, the living walkers, three spare. The
/// mould grid follows (`Engine::record_count`), room for squares down to a
/// game cell or `GRID_MIN` dish cells, whichever is smaller
/// (`Engine::grid_max`).
pub const WSTAT_BYTES: usize = 32;

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
    pub alarm_keep: f32,
    pub alarm_weight: f32,
    pub alarm_relay: f32,
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
    /// The crater's cut, length of full bite and cone (`BigParams::
    /// scorch_keep`, `scorch_full`, `scorch_cone`), for the look A/B of a
    /// shot's hole (`larder-world scorch_cone=1 ...`).
    pub scorch_keep: f32,
    pub scorch_full: u16,
    pub scorch_cone: bool,
    pub seed: u64,
}

impl Settings {
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
            alarm_keep: t.alarm_keep,
            alarm_weight: t.alarm_weight,
            alarm_relay: t.alarm_relay,
            upkeep: t.upkeep,
            start_energy: t.start_energy,
            birth_energy: t.birth_energy,
            birth_cost: t.birth_cost,
            starve: t.starve,
            capacity: t.capacity,
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
            scorch_keep: t.scorch_keep,
            scorch_full: t.scorch_full,
            scorch_cone: t.scorch_cone,
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
        "alarm_keep" => s.alarm_keep = f()?,
        "alarm_weight" => s.alarm_weight = f()?,
        "alarm_relay" => s.alarm_relay = f()?,
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
        "scorch_keep" => s.scorch_keep = f()?,
        "scorch_full" => s.scorch_full = int()?.clamp(1, 65535) as u16,
        "scorch_cone" => s.scorch_cone = b()?,
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
        alarm_keep: s.alarm_keep,
        alarm_weight: s.alarm_weight,
        alarm_relay: s.alarm_relay,
        upkeep: s.upkeep,
        start_energy: s.start_energy,
        birth_energy: s.birth_energy,
        birth_cost: s.birth_cost,
        starve: s.starve,
        capacity: s.capacity,
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
        scorch_keep: s.scorch_keep,
        scorch_full: s.scorch_full,
        scorch_cone: s.scorch_cone,
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

pub const BINDINGS: u32 = 48;
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
/// Every entry point, in pipeline order.
pub const PASSES: [&str; 37] = [
    "sense", "walk", "trail", "scent", "histogram", "top", "draw", "jump", "land", "convert", "life", "breed", "born", "eat", "pilesum", "flow", "walls", "build", "evict", "settle", "unclaim", "shots", "strike", "cut", "smell", "units", "harvest", "rows", "fate", "fly", "alight", "hit", "kill", "shout", "esum", "egain", "count",
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
    /// The damage memory (binding 47, the squad toolkit's layout): the
    /// crater countdown a cell in fine steps (whole numbers in f32; the
    /// CPU's `BigDish::scorch`), then the tiers thread's scar, unused so
    /// far. `scorched` once any crater was laid: from then on the trail
    /// pass counts it down and cuts the trail under it, as the CPU's
    /// `tick` does once `scorch` is made. `crater` is the steps a shot's
    /// crater lasts this frame (`squad::Frame::crater`).
    pub crater: wgpu::Buffer,
    pub crater_steps: u32,
    pub scorched: bool,
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
    pub squad_due: bool,
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
    /// Whether the crowd limit can hold (walkers at most half the agar).
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

impl Engine {
    pub fn new(device: &wgpu::Device, queue: &wgpu::Queue, shared: &Shared, s: &Settings, scale: usize) -> Self {
        Self::made(device, queue, shared, s, scale, BigDish::new)
    }

    /// The engine on a designed world (`BigDish::with_map`, ROADMAP WM1).
    pub fn with_map(device: &wgpu::Device, queue: &wgpu::Queue, shared: &Shared, s: &Settings, scale: usize, m: &Map) -> Self {
        Self::made(device, queue, shared, s, scale, |p| BigDish::with_map(p, m))
    }

    fn made(device: &wgpu::Device, queue: &wgpu::Queue, shared: &Shared, s: &Settings, scale: usize, make: impl Fn(BigParams) -> BigDish) -> Self {
        let agar_cells = make(BigParams { walkers: 1, ..big_params(s, scale, false) }).agar.iter().filter(|&&a| a).count();
        let crowd_ok = s.walkers.max(s.capacity) <= agar_cells / 2;
        let d = make(BigParams { species_sets: species_sets(s, crowd_ok), ..big_params(s, scale, s.crowd && crowd_ok) });
        let dev = device;
        let cells = (d.w * d.h) as u32;
        let n = d.walkers.len() as u32;
        let storage = wgpu::BufferUsages::STORAGE | wgpu::BufferUsages::COPY_DST | wgpu::BufferUsages::COPY_SRC;
        let buf = |label: &str, bytes: &[u8], usage: wgpu::BufferUsages| {
            let b = dev.create_buffer(&wgpu::BufferDescriptor { label: Some(label), size: bytes.len().max(16) as u64, usage, mapped_at_creation: false });
            queue.write_buffer(&b, 0, bytes);
            b
        };
        let zeros = |label: &str, bytes: usize| dev.create_buffer(&wgpu::BufferDescriptor { label: Some(label), size: bytes.max(16) as u64, usage: storage, mapped_at_creation: false });
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
        let germ = [zeros("germ0", 2 * cells as usize * 4), zeros("germ1", 2 * cells as usize * 4)];
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
        let sq = zeros("squad", squad::WORDS * 4);
        let kills = zeros("kills", squad::SHOTS_MAX * 4);
        let hits = zeros("hits", MAX_HITS * 16);
        let look = zeros("look", cells as usize * 4);
        let body = zeros("body", 2 * cells as usize * 4);
        let crater = zeros("crater", 2 * cells as usize * 4);
        let group = |even: bool| {
            let (a, b) = if even { (0, 1) } else { (1, 0) };
            let bufs: [&wgpu::Buffer; BINDINGS as usize] = [
                &params, &state, &walkers, &intent, &icell, &occ, &claim, &dep, &agar, &lay, &trail[a], &trail[b], &scent[a], &scent[b], &hist, &frame, &hud, &pile, &reached, &sick, &sdep, &gdep, &germ[a], &germ[b], &edep, &bdep, &species[a],
                &species[b], &age, &grass, &gcount, &spec, &energy, &share, &ground, &charge, &health, &bites, &pcell, &piles, &wstat, &wb, &sq, &kills, &hits, &look, &body, &crater,
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
            crater,
            crater_steps: 0,
            scorched: false,
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
            squad_due: false,
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
        self.queue.write_buffer(&self.germ[side], 0, &f32_bytes(&germ));
        // The life states come with the spec words below; a dish that has
        // them switches them on here.
        self.cycles |= !d.life.is_empty();
        self.alarmed |= !d.alarm.is_empty();
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
        b.extend(f32_bytes(&[s.trail_scale, s.reach_by[0], s.reach_by[1], am[0], am[1], tm[0], tm[1], s.step_by[0], s.step_by[1], s.floor, s.wander, s.catch_at]));
        // Infect: the mouse's disc, or the middle once `seed_at` has come.
        let (mut infect, mut ix, mut iy, mut ir) = (0u32, 0.0f32, 0.0f32, 0.0f32);
        if let (true, Some((x, y))) = (hand.infect, hand.at) {
            (infect, ix, iy, ir) = (1, x, y, br);
        } else if s.sick_on() && s.seed_at > 0.0 && !self.seeded && ticks > 0 && self.done as f32 >= s.seed_at * STEPS_PER_SECOND as f32 {
            (infect, ix, iy, ir) = (1, w as f32 / 2.0, h as f32 / 2.0, s.seed_r);
            self.seeded = true;
        }
        b.extend(u32_bytes(&[s.sick, s.immune, infect, 0]));
        b.extend(f32_bytes(&[s.germ, s.germ_keep, s.sick_speed, s.sick_trail, s.sick_reach, s.sick_turn, ix, iy, ir, 0.0, 0.0, 0.0]));
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
        b.extend(u32_bytes(&[stream(seed, DRAW_STARVE), stream(seed, DRAW_BIRTH), stream(seed, CLAIM_BIRTH), mask, 0, 0]));
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
        b.extend(u32_bytes(&[self.n_hits, 0, 0, 0]));
        // The species block (2026-11-11): whether each species has its own
        // settings, the walker kinds, conversion's square; the second
        // kind's multipliers; then a set a species (`Set` in engine.wgsl),
        // each with its scent reach as the CPU takes it (`scent_offset`, or
        // the species' own sensor reach when that is 0).
        b.extend(u32_bytes(&[!sets.is_empty() as u32, self.kinds, s.convert_r, 0]));
        b.extend(f32_bytes(&[s.mix_reach, s.mix_angle, s.mix_turn, s.mix_step, s.mix_deposit, 0.0, 0.0, 0.0]));
        for sp in 0..MAX_SPECIES as usize {
            let Some(t) = sets.get(sp) else {
                b.extend([0u8; 48]);
                continue;
            };
            let sso = if s.smell > 0.0 { s.smell } else { t.sensor_offset };
            b.extend(f32_bytes(&[t.sensor_angle, t.sensor_offset, t.rotate_angle, t.step, sso, t.deposit, t.keep, t.blocked_deposit, t.graze_weight]));
            b.extend(u32_bytes(&[t.crowd as u32, t.explore_steps as u32, 0]));
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
        b.extend(u32_bytes(&[stream(seed, CLAIM_LAND), stream(seed, DRAW_SPORE_H), s.spore_steps as u32, s.spore_rest as u32, s.wake_steps as u32, s.sleep_rest as u32, stream(seed, DRAW_SLEEP_DIE), 0]));
        b.extend(f32_bytes(&[s.spore_rate, s.spore_scent, s.spore_alarm, s.spore_burst, s.step * s.spore_speed, s.spore_reach, s.spore_deposit, s.spore_land]));
        b.extend(f32_bytes(&[s.sleep_rate, s.sleep_deposit, s.wake_scent, s.wake_alarm, s.alarm_keep, s.alarm_weight, s.alarm_relay, s.sleep_death]));
        assert_eq!(b.len(), STRIKE_AT, "Params rows before the strike row moved");
        b.extend([0u8; 32]);
        // Soil (WM3, 2026-11-12): the dish's own rates, as the CPU reads
        // them from its `BigParams` (read only with `WORLD_SOIL`).
        b.extend(f32_bytes(&[t.sand_keep, t.clay_keep, t.barren_upkeep, 0.0]));
        assert_eq!(b.len(), BODY_AT, "Params rows before the one body row moved");
        b.extend(f32_bytes(&[s.energy_flow, s.hunger_scent, s.hunger_at, 0.0]));
        assert_eq!(b.len(), CRATER_AT, "Params rows before the crater row moved");
        b.extend(f32_bytes(&[t.scorch_keep, t.scorch_full.max(1) as f32, self.crater_steps as f32]));
        b.extend(u32_bytes(&[self.crater_flags()]));
        self.energy_flows = self.grows && s.energy_flow > 0.0;
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

    /// Records a frame: `ticks` ticks, then the drawing. With `stamps`
    /// (and at least one tick), GPU timestamps at the first tick's start
    /// (0), the last tick's end (1) and the drawing's end (2).
    pub fn record(&mut self, shared: &Shared, enc: &mut wgpu::CommandEncoder, ticks: u32, soft: bool, stamps: Option<&wgpu::QuerySet>) {
        let stamps = if ticks > 0 { stamps } else { None };
        let (passes, ticked) = self.frame_passes(ticks, soft);
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
        let (passes, _) = self.frame_passes(ticks, soft);
        self.time_passes(shared, &passes)
    }

    /// A frame's passes in order, each with its workgroups and the side of
    /// the bind group it reads: the squad's start if one is due (shots,
    /// their strikes, then the smell, on the trail and scent the first
    /// tick reads: `squad.rs`'s frame), `ticks` ticks, the squad's end
    /// (the reads, then the harvest), then the drawing; and where the
    /// ticks' passes are in it. Moves `done` on by `ticks` and takes the
    /// squad's frame.
    fn frame_passes(&mut self, ticks: u32, soft: bool) -> (Vec<(usize, u32, usize)>, std::ops::Range<usize>) {
        let (nunits, nshots, _) = self.squad;
        let due = std::mem::take(&mut self.squad_due);
        let side = |done: u64| (done % 2) as usize;
        let mut out = Vec::new();
        if due {
            let s = side(self.done);
            if nshots > 0 {
                out.extend([(SHOTS, nshots.div_ceil(256), s), (STRIKE, self.n.div_ceil(256), s), (CUT, nshots, s)]);
                if self.alarmed {
                    out.push((SHOUT, 1, s));
                }
            }
            if nunits > 0 {
                out.push((SMELL, 1, s));
            }
        }
        let tick = self.tick_passes();
        let start = out.len();
        for _ in 0..ticks {
            let s = side(self.done);
            out.extend(tick.iter().map(|&(p, g)| (p, g, s)));
            self.done += 1;
        }
        let ticked = start..out.len();
        let s = side(self.done);
        if due && nunits > 0 {
            // A workgroup a unit (`engine.wgsl`'s `units`, 2026-11-12).
            out.extend([(UNITS, nunits, s), (HARVEST, 1, s)]);
        }
        out.extend(self.draw_passes(soft).into_iter().map(|(p, g)| (p, g, s)));
        (out, ticked)
    }

    /// A tick's passes and their workgroups: conversion only with species
    /// and `convert` on, the jump passes only while respawn or the pull is
    /// on, `land` only with the crowd limit.
    pub fn tick_passes(&self) -> Vec<(usize, u32)> {
        let (n, cells) = (self.n.div_ceil(256), self.cells.div_ceil(256));
        let mut tick = vec![(SENSE, n), (WALK, n)];
        // The Larder World, in the CPU's order: bites on walls (in the
        // move), then eating; both before growth, which eats what they give.
        if self.walled {
            tick.push((WALLS, cells));
        }
        // The life cycle, as the CPU's `life_step` after the move and the
        // wall bites, before eating. The move's claims are still in
        // `claim`; a spore's landing claim never meets one, since every
        // claimed cell now holds its winner and the engine's crowd limit is
        // at most 1 (with a limit of 2 they would need clearing first).
        if self.cycles {
            tick.extend([(FATE, n), (FLY, n), (ALIGHT, n)]);
        }
        if self.npc > 0 {
            let pc = self.npc.div_ceil(256);
            tick.push((EAT, pc));
            if self.flows {
                tick.extend([(PILESUM, self.npiles), (FLOW, pc)]);
            }
            tick.push((PILESUM, self.npiles));
        }
        // One body's flow, as the CPU's `flow_energy` after `eat_crystals`
        // and before `live_and_die` (`life` clears its sums).
        if self.energy_flows {
            tick.extend([(ESUM, n), (EGAIN, n)]);
        }
        if self.grows {
            tick.extend([(LIFE, n), (BREED, n), (BORN, n)]);
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
        tick
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
        // The life states, so the measure leaves the dead out.
        if self.cycles {
            (self.dish.life, self.dish.life_t) = self.life_of();
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
        let k = self.dish.p.scale as f32;
        self.cycles = true;
        self.write_params(s, Hand::default(), 0);
        self.queue.write_buffer(&self.params, STRIKE_AT as u64, &f32_bytes(&[x * k, y * k, r * k, alarm, alarm_r * k, 0.0, 0.0, 0.0]));
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

    /// The alarm field now (zeros without one).
    /// The crater countdown a cell (the damage memory's first block), read
    /// back: the CPU's `BigDish::scorch`.
    pub fn crater_of(&self) -> Vec<f32> {
        let g = floats(&read_back(&self.device, &self.queue, &self.crater));
        g[..self.cells as usize].to_vec()
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
        enc.clear_buffer(&self.wstat, 16, Some(4));
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
        self.queue.write_buffer(&self.kills, 0, &[0u8; squad::SHOTS_MAX * 4]);
        self.squad = (nunits, nshots, f.slime);
        self.squad_due = true;
        self.queue.write_buffer(&self.params, SQUAD_AT as u64, &u32_bytes(&[nunits, nshots, f.slime.to_bits(), squad::SHOT_ALARM.to_bits()]));
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
            d.spec.get(k).map_or(0, |&c| c as u32) | d.kind.get(k).map_or(0, |&c| (c as u32) << 8) | d.life.get(k).map_or(0, |&c| (c as u32) << 9) | d.life_t.get(k).map_or(0, |&c| (c as u32) << 16)
        })
        .collect()
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
