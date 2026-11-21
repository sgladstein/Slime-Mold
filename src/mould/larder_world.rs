//! **Larder in the world** (ROADMAP W2b; the plan is the *W2 plan* section of
//! `Reports/larder-world-roadmap-2026-11-11.md`). M3's squad (`larder.rs`)
//! on the Larder World: a growing mould that starts on one crystal, crystals
//! that run dry, rock, wet ground and walls the mould wears down. No window
//! here; `examples/larderworld.rs` plays idle, random and planned hands on
//! paired seeds, and the window (W2c) drives the same `Game`.
//!
//! **Two worlds, one game.** The rules run over `World`, which the CPU big
//! dish implements here (tests, CI and small censuses). The GPU engine is
//! to implement it next (play, and censuses on the GTX 1070); it first
//! needs the CPU race world's pace, since the race world takes `scale`
//! fine steps a tick and the engine one. Everything
//! the squad does to the mould and reads from it goes through one squad
//! frame (`squad::Frame`, W2a): shots that stop at the first slime and
//! strike there, the soldiers' smell, then the ticks, then each soldier's
//! touch, its densest mould in sight and its harvest. So the game never
//! reads the world's fields itself, and the GPU needs one small read-back a
//! frame.
//!
//! **What changed from M3**, and why:
//! - Food is crystal charge for both sides (the plan): a clear order
//!   harvests charge from the crystal's cells into the soldier's carry,
//!   walked home to the larder. What the squad takes the mould can't eat.
//!   The bank counts food, `Rules::charge` charge to one food, so M3's
//!   prices carry over.
//! - A shot is a ray from the soldier toward its target that stops at the
//!   first cell that looks like slime, or at rock or a wall, and strikes a
//!   disc there (owner, 16:48Z: "Gun fire should hit the first slime in its
//!   path"). M3's shot was a line laid across the target point.
//! - The aim is the densest 4 x 4 block in sight from the frame before
//!   (`UnitOut::target`), so a soldier fires on the mould's thickest part
//!   it can see, and the ray stops on whatever slime is in front of it.
//! - Soldiers walk round rock and walls on a distance map to their goal
//!   (`distance_map`), and fire only along a clear line (`clear_line`).
//!   M3's dish had no rock. Without either, on seed 1 three of four
//!   rushing soldiers stood behind rock 50 cells short of the core and put
//!   196 of 200 shots into it; with both they reached the core and killed
//!   1,282 walkers with 200 shots, against 13.
//! - The mould steals from the larder: walkers touching it take
//!   `Rules::steal` food a game second from the bank. In M3 the larder was
//!   a pile the mould ate; here the bank is not on the ground, and without
//!   a cost at home an idle squad behind a wall loses nothing.
//! - Not yet ported: turrets, bait, hunger (it barely acted in M2) and
//!   bodies as crystals (they need a spare crystal slot on both worlds).
//!
//! Distances in `Rules` are dish cells, as in M3 (the 512 x 320 world at
//! scale 2 is M3's 256 x 160 dish); the game works in fine cells and
//! converts through `World::scale`. Time is game seconds of `Rules::tps`
//! ticks.

mod buildings;
mod world;
mod raids;
mod bosses;
mod fruit;
mod waves;
mod heart;
mod stand;
mod strikers;
mod starfall;
mod cores;
mod force;
mod rules;
mod paths;
mod cmd;
mod raising;
mod shots;
mod turn;
mod score;
mod shapes;
mod squads;
mod funnel;
mod steer;
mod cover;
mod doors;
mod bulwark;
mod fireway;
mod gear;
// The moved modules' items keep their old paths (`larder_world::World`,
// `larder_world::Rules`, ...).
pub use cmd::*;
pub use paths::*;
pub use raising::*;
pub use rules::*;
pub use world::*;
pub use shapes::shape_keys;
pub use squads::{Formation, LeadRules, Squad, NO_LINK, SQUADS_MAX, SQUAD_QUEUE};
pub use cover::{Cover, CoverRules, HealRules};
pub use doors::{DOOR_WIDTHS, WALL_LEVELS};
pub use raids::{LureMark, March, MarchEnd, MarchHow, Raid, RaidAim, RaidRules, Raids};
pub use fruit::{Fruit, FruitEnd, FruitRules, Fruits};
pub use waves::{Wave, WavePhase, WaveRules, Waves};
pub use heart::HeartRules;
pub use stand::{Heart, Stand, StandRules};
pub use strikers::{StrikerRules, Strikers};
pub use starfall::{fall_count, Cashed, CoreFrom, Drop, DropKind, StarRules, Starfall};
pub use cores::Cores;
pub use bosses::{parts, Boss, BossCause, BossEnd, BossKind, BossPhase, BossRules, Bosses, Part, PART_BIN};
pub use buildings::{BuildRules, Building, Kind, Shell, PUMP_LEVELS, TURRET_DECK, TURRET_LEVELS, TURRET_MUZZLE};
use buildings::{wall_at, Who};

use super::big::{BigDish, BigParams, LIFE_DEAD, SURGE_WAIT};
use super::squad::{self, Frame, FrameOut, ShotIn, UnitIn, UnitOut};
use super::Rng;
pub use super::guns::Gun;
pub use super::rounds::Deck;
use super::kit::Kit;
use super::rounds::Rounds;

pub use super::larder::{End, Order, Stance};

/// Walking maps: for each goal cell, every open cell's distance to it
/// (`distance_map`) and the step it was last walked, asked for when a
/// soldier first needs one (`Game::map_for`) and built a slice a step
/// (`Game::pump_maps`).
#[derive(Default)]
struct Paths {
    maps: std::collections::HashMap<usize, (Vec<u32>, u64)>,
    /// The tick of the last `PATHS_EVERY` rebuild, and its step.
    at: u64,
    went: u64,
    /// Steps of the game so far, as `pump_maps` counts them.
    step: u64,
    /// Goal cells asked for, oldest first, each with the step it was last
    /// asked at.
    asked: Vec<(usize, u64)>,
    /// The map being built.
    building: Option<MapBuild>,
    /// Built maps to build again (a wall went up, or `PATHS_EVERY`).
    stale: Vec<usize>,
    /// Whether the last map started was an asked goal's: with both asked
    /// and stale maps waiting, starts take turns (`pump_maps`).
    asked_last: bool,
    /// Whether the map being built went stale while it was built (a wall
    /// went up): it joins `stale` when done.
    build_stale: bool,
    /// Which coarse squares a soldier can mostly stand on (`fireway.rs`),
    /// until the maps go stale.
    coarse: Option<Vec<bool>>,
    /// Fire seen lately: where, how far round (fine cells), and the tick
    /// it cools (`Game::mark_hot`).
    hot: Vec<((f32, f32), f32, u64)>,
}

/// Orders a soldier can hold in its queue after the one it is doing.
pub const QUEUE: usize = 8;
/// What a Guard order keeps round a crystal's rim or a building's
/// footprint, dish cells (`Game::zone_at`): slime this near what is
/// guarded is coming onto it.
pub const GUARD_RIM: f32 = 3.0;

#[derive(Clone, Copy, Debug)]
pub struct Soldier {
    /// Fine cells.
    pub x: f32,
    pub y: f32,
    pub hp: f32,
    pub order: Order,
    pub stance: Stance,
    pub carry: f32,
    pub homing: bool,
    /// Walking home from a crystal that ran dry (`Rules::home_dry`): once
    /// it banks, it holds at the larder instead of walking back for crumbs.
    pub dry: bool,
    /// Orders to do after this one (Shift, `Cmd::Queue`), first
    /// `queued` of them; fixed size so a soldier stays `Copy`.
    pub queue: [Order; QUEUE],
    pub queued: u8,
    pub alive: bool,
    /// Tick of its next shot.
    ready: u64,
    /// Which point of a burn line it fires at next.
    beat: u32,
    /// What it read last frame: walkers touching, its densest mould in sight.
    pub touching: u32,
    pub aim: Option<(f32, f32)>,
    /// Walkers in the block it aims at.
    pub dense: u32,
    /// Its smell, x `Rules::smell` (1). The squad toolkit's mask (0: the
    /// mould can't smell it, to sneak to a crystal) and reek (over 1: a
    /// decoy that pulls the mould off the harvesters), `mould::kit`.
    pub smell: f32,
    /// Carry taken up by tools it holds (`mould::kit`'s carry rule): a
    /// soldier carrying blasts brings home less food. 0 by default.
    pub load: f32,
    /// Its squad (`squads`), an index into `Game::squads`.
    pub squad: u8,
    /// Standing guard while the rest of its squad works (`Game::cover_turn`).
    pub cover: bool,
    /// Game seconds it has stood still, unbitten and not firing: it heals
    /// once this reaches `HealRules::wait` (`Game::heal`).
    pub rest: f32,
    /// The cast its next shot carries, an index into its squad's casts
    /// (`BulletMods::casts`): each shot fires the next card in turn and the
    /// gun never stops to recharge (owner 18:05Z).
    pub next: u8,
    /// Its velocity, fine cells a game second (`steer`: a soldier speeds
    /// up and slows down instead of starting and stopping dead).
    pub vx: f32,
    pub vy: f32,
    /// Where it stood when this step's walk began, fine cells: the window
    /// draws it between there and `x`, `y` (`Game::drawn_between`), so it
    /// glides at the screen's frame rate instead of jumping 15 times a
    /// second.
    pub was: (f32, f32),
    /// The sleep gas cloud where it stood last frame, in the levels that
    /// put the slime to sleep (`World::clouds`); at `Rules::doze` or over
    /// it dozes (`Game::dozing`).
    pub doze: f32,
}

/// One shot as the window draws it: from, where it stopped (None: a wall or
/// rock stopped it, or it flew its range), and walkers killed.
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct Shot {
    pub from: (f32, f32),
    pub to: (f32, f32),
    pub hit: Option<(f32, f32)>,
    pub killed: u32,
    /// The soldier that fired it (None: a turret).
    pub by: Option<usize>,
    /// The turret that fired it (`Game::buildings` index), if one did.
    pub turret: Option<usize>,
    /// Fired this frame (a bullet's first leg, or an instant ray): the
    /// muzzle flash and the gunshot's sound; later legs are flight.
    pub fired: bool,
    /// The cast it carries (`Bullet::cast`): 0 plain.
    pub cast: u8,
    /// Its strike's radius, fine cells (`Bullet::r`): what the hit kills,
    /// and so how far the window's light of it reaches (owner 20:22Z).
    pub r: f32,
}

/// One take from a crystal in the last frame, as the window draws it
/// (`Game::last_takes`; art direction 2026-11-14, harvesting that looks
/// busy): who took (a soldier, or None for a building such as a pump),
/// from which crystal, where the taker stands (fine cells) and how much
/// charge. The soldier's working pulse and the crystal's working spot
/// both read this one list, so they light together.
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct Take {
    pub by: Option<usize>,
    pub crystal: usize,
    pub at: (f32, f32),
    pub took: f32,
}

/// A leg's shooter (None: a turret), whether it was fired this frame, the
/// cast it carries (`Bullet::cast`) and the turret that fired it
/// (`Bullet::turret`).
type LegBy = (Option<usize>, bool, u8, Option<usize>);

/// Who fires a shot this frame: the soldier (None: not a soldier), the cast
/// it carries, and the turret (`Game::buildings` index) when a turret
/// fired it (the turrets thread, 2026-11-16: the independent review's
/// shooter id, so a turret's cast kills, its Pull and its Render credit
/// find the turret that fired).
pub(super) type Firer = (Option<usize>, u8, Option<usize>);

/// A shot in flight (`Rules::bullet_speed`): where it is, its heading, the
/// fine cells of its range left, its strike radius, who fired it and the
/// cast it carries.
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct Bullet {
    pub at: (f32, f32),
    pub dir: (f32, f32),
    pub left: f32,
    pub r: f32,
    pub by: Option<usize>,
    /// The turret that fired it (`Game::buildings` index): its cast, Pull
    /// and kills go to that turret (`Shot::turret`).
    pub turret: Option<usize>,
    pub fresh: bool,
    /// Slime hits it flies on through (each one strikes as a shot does).
    pub pierce: u8,
    /// Times it splits where it finally lands (`Rules::split_n`); its
    /// pieces split one fewer time and do not pierce.
    pub split: u8,
    /// The cast it carries: 0 plain (a turret's, a split piece), else 1 +
    /// the index of the cast in its soldier's squad's cards when it was
    /// fired (`Soldier::next`). Every slime hit casts it (`rounds`), so a
    /// piercing bullet casts at each strike.
    pub cast: u8,
    /// Its speed, times `Rules::bullet_speed` (its bullet card's,
    /// `bullets::Ammo::speed`; 1 for a turret's).
    pub speed: f32,
    /// The cast it carries to let out (`rounds::Card::Trigger`, 2026-11-16;
    /// `BulletMods::child`): 0 none, else that cast's id. It is let out as
    /// that cast's own bullets would be fired, from where this one hits
    /// slime (`Game::landed`).
    pub carry: u8,
    /// It lets its carried cast out where it stops instead of at its first
    /// hit (`rounds::Card::Expire`): spent on slime, at a wall (thrown back
    /// off it) or at the end of its range.
    pub expire: bool,
    /// Seek cards it flies with (`BulletMods::seek`), and how far it has
    /// flown (fine cells), which places its aim point (`aim`) for the
    /// catch.
    pub seek: u8,
    pub flown: f32,
    /// How far from its muzzle it reaches the point its soldier aimed at
    /// (fine cells; infinite for a turret's, a split piece or a let-out
    /// cast): a seeker's catch looks beside its line there
    /// (`Game::catches`), as a soldier's bullet flies on to its full range.
    pub aim: f32,
    /// It flies over walls (`ShotIn::over`): a turret's (owner,
    /// 2026-11-16 19:50Z: "turrets can fire over walls ... but regular
    /// guns shots do not"). Rock still ends it.
    pub over: bool,
}

/// What every soldier's bullet carries when fired: the rounds' flight
/// cards (`rounds::Rounds::arm`). Turret shots carry none.
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub struct BulletMods {
    pub pierce: u8,
    pub split: u8,
    /// Copies of the `Range` card: each stretches every soldier's sight
    /// and reach by `Rules::range_card` of `Rules::range`
    /// (`Game::gun_range`).
    pub reach: u8,
    /// Copies of the `Rate` card: each fires every soldier's gun
    /// `Rules::rate_card` more times as often (`Game::shot_gap`).
    pub rate: u8,
    /// Casts the gun fires in its own turn (`rounds::cycle`): each shot
    /// carries the next of them (`Soldier::next`), the cast id in `cycle`.
    pub casts: u8,
    /// The cast ids (1 + the index in `rounds::casts`) of the gun's turn,
    /// the first `casts` of them. A carried cast (`Card::Trigger`) is not
    /// among them: its carrier's bullet lets it out.
    pub cycle: [u8; super::rounds::MAX_SLOTS],
    /// Each cast's bullet (`bullets::Ammo`), by its index in
    /// `rounds::casts`: the shot's damage (its strike), speed, range and
    /// spread.
    pub ammo: [super::bullets::Ammo; super::rounds::MAX_SLOTS],
    /// Each cast's carried cast id (0 none, `rounds::Cast::child`), and
    /// whether it lets it out where it stops without a hit too.
    pub child: [u8; super::rounds::MAX_SLOTS],
    pub expire: [bool; super::rounds::MAX_SLOTS],
    /// Each cast's cast id fired beside it in the same shot (0 none,
    /// `rounds::Cast::with`, `Card::Multicast`).
    pub with: [u8; super::rounds::MAX_SLOTS],
    /// Copies of the `Seek` card: every bullet catches live slime beside
    /// its line, `Rules::seek_catch` cells a copy (`Game::catches`). Its
    /// first steering (`Game::steer`, `seek_turn`) was deleted 11-17.
    pub seek: u8,
}

pub struct Game<W: World> {
    pub world: W,
    pub rules: Rules,
    pub soldiers: Vec<Soldier>,
    pub bank: f32,
    pub spent: f32,
    pub harvested: f32,
    pub stolen: f32,
    /// Shots fired (with bullets, once a bullet, not once a leg).
    pub shots: u32,
    pub killed: u32,
    /// Walkers the plants' fire killed (`World::burned`; in `killed` too).
    pub fire_killed: u32,
    /// Walkers the turrets' shots killed (shots with no soldier, `Shot::by`
    /// None): the shove cards' kill zone measure (2026-11-16).
    pub turret_killed: u32,
    /// Casts fired beside another in the same shot (`Card::Multicast`).
    pub multicast: u32,
    /// Soldier bullets' slime hits, and the dish cells they hit counted
    /// once a frame: how widely a shot's hits spread (the seek film's
    /// measure, 2026-11-16).
    pub hits: u32,
    pub hit_cells: u32,
    /// Soldier bullets that ended with no slime struck on their last leg,
    /// at the end of their range or at a wall or rock (the owner's 11-16
    /// 14:24Z question: "how often do soldiers you're not controlling miss
    /// their target"). A piercing bullet that hit before counts too.
    pub missed_range: u32,
    pub missed_wall: u32,
    pub walls: u32,
    /// The doors' cells (`doors`), row by row over the world's cells:
    /// ground to soldiers though the world holds them as wall. Empty while
    /// there is no door.
    pub doors: Vec<bool>,
    /// The bulwark (`Rules::set_block`): fine cells set slime holds,
    /// no ground to soldiers' feet. Empty while there is none.
    pub bulwark: Vec<bool>,
    /// The bulwark as the walking maps were last made stale for it, and the
    /// tick that was (`Game::mark_bulwark`).
    pub bulwark_maps: Vec<bool>,
    pub bulwark_staled: u64,
    /// The crystals the slime holds now, each one's ring in the fort's
    /// mask (`Rules::fort`, `Game::time_fort`). Empty until the first look.
    pub fort_held: Vec<bool>,
    /// Turrets, lures, pumps and walls placed (`buildings`); a wall's
    /// health lives in the world's cells, not here.
    pub buildings: Vec<Building>,
    /// Food drawn by pumps, and buildings the mould wore down.
    pub pumped: f32,
    pub lost_buildings: u32,
    /// Calls the sirens made (`Kind::Siren`).
    pub sirens: u32,
    /// The squad's base, fine cells: where bought soldiers stand and carried
    /// food goes.
    pub larder: (f32, f32),
    /// The mould's home crystal, fine cells.
    pub core: (f32, f32),
    pub core_down: Option<u64>,
    core_clear: u64,
    /// What the core is (`Core`): `Core::of(&rules)` unless a tier says
    /// otherwise (`with_tier`).
    pub core_rule: Core,
    pub first_touch: Option<f32>,
    /// The world's ticks when the game began: 0 unless the slime grew
    /// alone first (`head_start`). `time` counts from here, so the clock,
    /// the goals and the end screen start at 0:00 on landing.
    pub landed: u64,
    /// Walkers touching the larder at the last frame (it is held while any
    /// do).
    pub larder_touch: u32,
    /// **Stuck seconds** (design review, 2026-11-12: no soft lock), counted
    /// once a game second: a soldier alive, no shot fired and no food taken
    /// (harvested plus carried did not rise) since the last count, no
    /// soldier walked, no pump stands, nothing is being built and not
    /// (free shots and a crystal with charge left), and the
    /// bank below a soldier's price (the walking, pump and building
    /// exceptions since the whole-game review's W17, 11-15: a run ended
    /// "stuck" at 1:37 with four soldiers at full health and both pumps
    /// standing). `stuck` is the round's total, `stuck_run`
    /// the seconds in a row now (0 once a second isn't stuck). The census
    /// (`examples/larderworld.rs`) counted this itself until 2026-11-12; the
    /// campaign's mission layer reads it here.
    pub stuck: u32,
    pub stuck_run: u32,
    stuck_mark: (u32, f32),
    /// Each soldier's place at the last stuck count: a soldier that walked
    /// since is doing something (whole-game review W17, 11-15).
    stuck_at: Vec<(f32, f32)>,
    /// Ticks the larder has been held with the bank empty (`Rules::overrun`),
    /// and ticks toward the next reserve recruit (`Rules::reserve`).
    held_ticks: u64,
    reserve_ticks: u64,
    pub end: Option<End>,
    /// The last frame's shots, for the window's tracers.
    pub last_shots: Vec<Shot>,
    /// The last frame's takes from crystals (`Take`), for the window.
    pub last_takes: Vec<Take>,
    /// Shots in flight (`Rules::bullet_speed`), oldest first; at most
    /// `squad::SHOTS_MAX`, and a shot fired past that is dropped (the
    /// newest go: the old ones are about to land).
    pub bullets: Vec<Bullet>,
    /// Mortar shells in the air (`buildings::Shell`), oldest first.
    pub shells: Vec<Shell>,
    /// Shells fired, and those that have come down.
    pub shells_fired: u32,
    pub shells_landed: u32,
    /// Walkers mortar shells' strikes killed (in `turret_killed` too; a
    /// shell's cast's kills go only there, as a turret's do).
    pub mortar_killed: u32,
    /// The bullets as they flew the last step's legs, one each in the order
    /// of `last_shots` (empty while shots are rays): what the window draws
    /// gliding along each leg between steps (gun look, 2026-11-15). No
    /// rule reads it.
    pub flown: Vec<Bullet>,
    /// The flight cards on soldiers' bullets, and bullets a split could
    /// not add past `squad::SHOTS_MAX` (dropped, newest first, as fired
    /// shots are).
    pub bullet_mods: BulletMods,
    /// The cards every turret fires (`Cmd::Load(TURRET_DECK, ..)`; the
    /// turrets thread, 2026-11-16): None, plain strikes as before. One deck
    /// for every turret, as one gun is a squad's (the proposal's option A:
    /// no clicking round twenty turrets), and a turret shot carrying a cast
    /// is told from a soldier's by `by` None (`turret_mods`).
    pub turret_deck: Option<Deck>,
    pub bullets_dropped: u32,
    /// Carried casts let out (`Bullet::carry`), each one release, however
    /// many bullets its set fires.
    pub releases: u32,
    /// Walkers alive at the last count (once a game second).
    pub mould: usize,
    /// The most walkers the mould has had at a count (`Rules::broken`).
    pub peak: usize,
    /// When the mould broke (`Rules::broken`), and the upkeep it had then,
    /// given back if it recovers; None while it is whole.
    pub broken: Option<(u64, f32)>,
    /// The slime has fallen under `Rules::late_at` of its peak: its
    /// charges reach and lunge as a cornered slime's for the rest of the
    /// planet (`Game::keep_panic_reach`).
    pub late: bool,
    /// Soldiers killed by the slime's touch (`Rules::drain`), by
    /// standing in fire (`Rules::fire_hurt`) and by a falling star's blast
    /// (`StarRules::hurt`, 11-20); the squad's own blasts' kills are
    /// `Rounds::blast_kills`. The 11-20 late-threat measure counts the
    /// slime's own kills apart from the squad's.
    pub fell_bitten: u32,
    /// Health the slime's touch has taken from soldiers (`Rules::drain`;
    /// the last-stand heart's lunge cap, `StandRules::cap`).
    pub bitten_hp: f32,
    pub fell_burned: u32,
    pub fell_starfall: u32,
    /// Each crystal's charge at the start and the charge the squad took
    /// from it, so the cap's share can tell the mould's eating from ours.
    start_charge: Vec<f32>,
    taken: Vec<f32>,
    /// Walkers at the start, for the stalemate's size term (`Game::score`).
    pub start_mould: usize,
    /// Walkers per open fine cell at the start (`start_mould` over the cells
    /// `World::open` holds). The unit for any threshold written as a walker
    /// count, so it keeps its meaning from the CPU's 5,000 walkers to the
    /// GPU's 100,000 (integration thread, 2026-11-12, from the technical
    /// lead's review: counts like 6 or 80 walkers broke at GPU scale).
    pub density: f32,
    /// W2b's held share at the cap (`Game::share`), kept so a census can
    /// print W2b's score beside v2's.
    pub cap_held: Option<f32>,
    /// Walking round rock and walls (`Paths`).
    paths: Paths,
    /// Crystals with charge left, centre and radius, fine cells: the ground
    /// soldiers can't stand on (`Rules::crystal_solid`), kept each step.
    solid: Vec<((f32, f32), f32)>,
    /// Each crystal's charge at the start, for the shrinking solid disc.
    crystal_full: Vec<f32>,
    pub rng: Rng,
    pub log: Vec<(u64, Cmd)>,
    /// The larder's rally point: the order a bought soldier starts with
    /// (`Cmd::Rally`; the window's right click with the larder selected).
    pub rally: Order,
    /// Every squad slot (`squads`), `SQUADS_MAX` of them; a slot with no
    /// live soldier is free.
    pub squads: Vec<Squad>,
    /// The squad the player controls (`Cmd::Active`), if any.
    pub active: Option<usize>,
    /// Each soldier's place in its squad's formation (`Game::rank_squad`),
    /// and the squad's size when they were matched: a squad whose size has
    /// changed since ranks its members by index until it is matched again.
    pub(super) ranks: Vec<u16>,
    pub(super) ranked: [u16; SQUADS_MAX],
    /// The walking map `Game::guide` went by last, if any (`Squad::map`).
    guided: Option<usize>,
    /// Walks and anchor steps that waited on a walking map being built,
    /// in all (`move_tests` counts them; the pathing thread's measure).
    pub map_waits: u64,
    /// Each squad's column through narrow places (`funnel`).
    pub(super) columns: Vec<funnel::Column>,
    /// Crystals guarded at the last count (`Rules::larder` off), and what
    /// guarding has paid in all.
    pub guarded: Vec<usize>,
    pub guard_paid: f32,
    /// Health soldiers have healed resting by a held crystal (`HealRules`).
    pub healed: f32,
    /// Food-driven surges (`Rules::surge_food`, `Game::time_surges`): food
    /// the slime has eaten since the last surge (the meter), surges the
    /// game has started, the fine step the last began on (or the landing),
    /// whether the game times them, and the crystals' charge at the last
    /// frame.
    pub surge_meter: f32,
    pub surges: u32,
    /// Food the slime has eaten from its crystals since the landing, as
    /// the meter counts it (with surges timed or not).
    pub slime_ate: f32,
    surge_last: u64,
    surge_timed: bool,
    charge_was: f64,
    /// Raids (`Rules::raid`, `raids.rs`): the meter and every raid sent.
    pub raids: Raids,
    /// Lures with no building (`raids::LureMark`; the lure's field,
    /// `time_lure_paths`), each until its tick.
    pub lure_marks: Vec<LureMark>,
    /// Mini-bosses (`Rules::boss`, `bosses.rs`): the live ones and the ended.
    pub bosses: Bosses,
    /// Fruiting (`Rules::fruit`, `fruit.rs`): the fruit ripening and the
    /// ended.
    pub fruits: Fruits,
    pub waves: Waves,
    /// Where the slime's heart beats from now (`heart.rs`), fine cells,
    /// and the tick it began beating there.
    pub heart_at: Option<(f32, f32)>,
    pub heart_since: u64,
    /// The condensed strikers (`strikers.rs`).
    pub strikers: Strikers,
    /// Starfall and boss cores (`Rules::star`, `starfall.rs`): the falls
    /// due, what is down and the shop points taken.
    pub starfall: Starfall,
    /// Boss cores that re-form their boss, and raids' cores (`cores.rs`).
    pub cores: Cores,
    /// The last-stand heart (`stand.rs`).
    pub stand: Stand,
    /// Food the kill box has paid (`BuildRules::render_food`).
    pub rendered: f32,
    /// The squad's tools (`mould::kit`) and the army's gun cards
    /// (`mould::rounds`), run after every finished step (`gear.rs`). Until
    /// 2026-11-20 the window's `App` held and ran both, so a game played
    /// without the window (the library's tests, the census) fired its gun
    /// with no card cast and never restocked a tool (Engine and tools, job
    /// 6). Set a planet's up from its run with `Game::kit_out`.
    pub kit: Kit,
    pub rounds: Rounds,
    /// Soldiers the kit restocks to its full carry (`KitRules::carry_max`)
    /// rather than `carry_each`: the window's selection, copied in each
    /// frame. Empty without a window.
    pub carry_full: Vec<usize>,
    /// Whether the game runs its kit and rounds itself after each step
    /// (`Game::gear_turn`): always in play. False only for a test that
    /// runs a `Rounds` of its own to look between a step and its casts
    /// (the walkers before a shove, the pops a step let out).
    pub gear: bool,
    /// The CPU time of each part of a step, when on (`Game::costed`): name,
    /// total ms, the longest ms and the calls. None in play; the bench
    /// turns it on and prints it.
    pub costs: Option<Vec<(&'static str, f64, f64, u32)>>,
    /// The step handed to the world and not yet finished (`begin_step`).
    flight: Option<Flight>,
}

/// A step between `Game::begin_step` and `Game::finish_step`: the tick it
/// began on, its frame, which soldier (None the larder) and crystal each
/// unit stands for, and the results if the world gave them at once.
struct Flight {
    t: u64,
    f: Frame,
    /// Each of `f.shots`: who fired it, whether it was fired this frame
    /// and the cast it carries; with bullets, leg q is `bullets[q]`'s.
    by: Vec<LegBy>,
    /// Shots fired this frame.
    fired: u32,
    who: Vec<Who>,
    from: Vec<Option<usize>>,
    out: Option<FrameOut>,
}

/// Grows the slime alone on `world` for `secs` game seconds at `rules`'
/// pace, before a `Game` lands on it: frames with no unit and no shot,
/// so it spreads, lays its trail, eats crystals and burns plants as it
/// would with nobody there. `Game::new` then counts its clock from the
/// world's ticks (`Game::landed`). The roguelike run's big planets
/// (`run::head_start`, the planet review's finding 5, 11-15).
///
/// A replay (`Game::replay`) needs the same head start on its fresh world:
/// its command ticks are the world's.
pub fn head_start<W: World>(world: &mut W, secs: f32, rules: &Rules) {
    let frames = (secs.max(0.0) * rules.tps as f32 / rules.frame_ticks.max(1) as f32).round() as u32;
    for _ in 0..frames {
        let f = Frame { units: Vec::new(), shots: Vec::new(), slime: squad::slime_trail(6.0), crater: 0, sink: rules.sink, ..Frame::default() };
        world.frame(&f, rules.frame_ticks);
    }
}

/// The squad's base on a world: the crystal farthest from the home one
/// (crystal 0), so the race starts at opposite ends.
pub fn far_crystal(world: &impl World) -> (f32, f32) {
    let cs = world.crystals();
    let home = cs[0].0;
    cs.iter().map(|c| c.0).max_by(|a, b| (a.0 - home.0).hypot(a.1 - home.1).total_cmp(&(b.0 - home.0).hypot(b.1 - home.1))).unwrap_or(home)
}

impl<W: World> Game<W> {
    /// The squad at the crystal farthest from the mould's home, the mould on
    /// its home crystal (crystal 0).
    pub fn new(world: W, rules: Rules, seed: u64) -> Self {
        let larder = world.landing().unwrap_or_else(|| far_crystal(&world));
        let core = world.crystals()[0].0;
        let mould = world.living();
        let (w, h) = world.size();
        let open = (0..w * h).filter(|&i| world.open((i % w) as f32 + 0.5, (i / w) as f32 + 0.5)).count();
        let density = mould as f32 / open.max(1) as f32;
        let start_charge: Vec<f32> = world.crystals().iter().map(|c| c.1).collect();
        let taken = vec![0.0; start_charge.len()];
        let world_ticks = world.ticks();
        let mut g = Game {
            world,
            rules,
            soldiers: Vec::new(),
            bank: rules.start_bank,
            spent: 0.0,
            harvested: 0.0,
            stolen: 0.0,
            shots: 0,
            killed: 0,
            fire_killed: 0,
            turret_killed: 0,
            multicast: 0,
            hits: 0,
            hit_cells: 0,
            missed_range: 0,
            missed_wall: 0,
            walls: 0,
            doors: Vec::new(),
            bulwark: Vec::new(),
            bulwark_maps: Vec::new(),
            bulwark_staled: 0,
            fort_held: Vec::new(),
            buildings: Vec::new(),
            pumped: 0.0,
            lost_buildings: 0,
            sirens: 0,
            larder,
            core,
            core_down: None,
            core_clear: 0,
            core_rule: Core::of(&rules),
            first_touch: None,
            landed: world_ticks,
            larder_touch: 0,
            stuck: 0,
            stuck_run: 0,
            stuck_mark: (0, 0.0),
            stuck_at: Vec::new(),
            held_ticks: 0,
            reserve_ticks: 0,
            end: None,
            last_shots: Vec::new(),
            last_takes: Vec::new(),
            bullets: Vec::new(),
            shells: Vec::new(),
            shells_fired: 0,
            shells_landed: 0,
            mortar_killed: 0,
            flown: Vec::new(),
            bullet_mods: BulletMods::default(),
            turret_deck: None,
            bullets_dropped: 0,
            releases: 0,
            mould,
            peak: mould,
            broken: None,
            late: false,
            fell_bitten: 0,
            bitten_hp: 0.0,
            fell_burned: 0,
            fell_starfall: 0,
            start_charge,
            taken,
            start_mould: mould,
            density,
            cap_held: None,
            paths: Paths::default(),
            solid: Vec::new(),
            crystal_full: Vec::new(),
            rng: Rng::new(seed ^ 0x001a_2de2_501d),
            log: Vec::new(),
            flight: None,
            rally: Order::Hold,
            squads: vec![Squad::default(); SQUADS_MAX],
            active: None,
            ranks: Vec::new(),
            ranked: [0; SQUADS_MAX],
            guided: None,
            map_waits: 0,
            columns: vec![funnel::Column::default(); SQUADS_MAX],
            guarded: Vec::new(),
            healed: 0.0,
            guard_paid: 0.0,
            surge_meter: 0.0,
            surges: 0,
            slime_ate: 0.0,
            surge_last: 0,
            surge_timed: false,
            charge_was: 0.0,
            raids: Raids::new(seed),
            lure_marks: Vec::new(),
            bosses: Bosses::new(seed),
            fruits: Fruits::default(),
            waves: Waves::default(),
            heart_at: None,
            heart_since: 0,
            strikers: Strikers::default(),
            starfall: Starfall::new(seed),
            cores: Cores::default(),
            stand: Stand::default(),
            rendered: 0.0,
            kit: Kit::new(Default::default()),
            rounds: Rounds::new(Vec::new(), Default::default()),
            carry_full: Vec::new(),
            gear: true,
            costs: None,
        };
        g.crystal_full = g.world.crystals().iter().map(|c| c.1).collect();
        g.charge_was = g.crystal_full.iter().map(|&c| c as f64).sum();
        g.take_surges();
        g.solid = g.solid_now();
        g.keep_panic_reach();
        for k in 0..rules.squad {
            let a = k as f32 / rules.squad.max(1) as f32 * std::f32::consts::TAU;
            g.add_soldier(a);
        }
        g
    }

    /// Gives the game a tier's core (`Tier::core`), read once here: None,
    /// or a tier with no core of its own, keeps `Core::of(&rules)`.
    pub fn with_tier(mut self, tier: Option<super::tiers::Tier>) -> Self {
        if let Some(c) = tier.and_then(|t| t.core()) {
            self.core_rule = c;
        }
        self
    }

    fn s(&self) -> f32 {
        self.world.scale()
    }

    /// How far squad `q`'s soldiers see and fire, dish cells: its gun's
    /// range (`Gun::range`; the standard rifle's is `Rules::range`)
    /// stretched by the rounds' `Range` cards (owner, 2026-11-14 08:39Z:
    /// "there should be a range modifer card too"). Free fire, aims, the
    /// burn line's stand-off and the squad mode's volleys all read this;
    /// building reach and turrets keep `Rules::range` and `turret_range`.
    pub fn gun_range(&self, q: usize) -> f32 {
        self.gun(q).range * (1.0 + self.rules.range_card * self.mods(q).reach as f32)
    }

    /// What squad `q`'s bullets carry (`rounds::flight`): from its own
    /// cards (`Squad::deck`) if it has them, else the army's
    /// (`bullet_mods`, set by `Rounds::arm` from the H loadout).
    pub fn mods(&self, q: usize) -> BulletMods {
        match self.squads.get(q).and_then(|s| s.deck) {
            Some(d) => super::rounds::flight(&d.cards()),
            None => self.bullet_mods,
        }
    }

    /// What every turret's bullets carry (`turret_deck`): nothing without
    /// a deck, so plain turrets fly as before.
    pub fn turret_mods(&self) -> BulletMods {
        self.turret_deck.map_or_else(BulletMods::default, |d| super::rounds::flight(&d.cards()))
    }

    /// The cards turret `b` fires: its own (`Building::deck`), else the
    /// turrets' deck; None (a turret's shot with no turret known) the
    /// turrets' deck.
    pub fn turret_cards(&self, b: Option<usize>) -> Option<Deck> {
        b.and_then(|b| self.buildings.get(b)).and_then(|b| b.deck).or(self.turret_deck)
    }

    /// What turret `b`'s bullets carry (`turret_cards`).
    pub fn turret_mods_of(&self, b: Option<usize>) -> BulletMods {
        self.turret_cards(b).map_or_else(BulletMods::default, |d| super::rounds::flight(&d.cards()))
    }

    /// What the bullets of shooter `by` carry: its squad's (`mods`), or
    /// for None turret `turret`'s (`turret_mods_of`).
    pub fn mods_by(&self, by: Option<usize>, turret: Option<usize>) -> BulletMods {
        match by {
            Some(k) => self.soldiers.get(k).map_or_else(BulletMods::default, |s| self.mods(s.squad as usize)),
            None => self.turret_mods_of(turret),
        }
    }

    /// Squad `q` takes its own cards (`Cmd::Load`), None the army's;
    /// `TURRET_DECK` loads every turret (None: plain strikes).
    pub fn load(&mut self, q: usize, deck: Option<Deck>) -> bool {
        if q == TURRET_DECK as usize {
            self.turret_deck = deck;
            return true;
        }
        match self.squads.get_mut(q) {
            Some(s) => {
                s.deck = deck;
                true
            }
            None => false,
        }
    }

    /// The farthest any live squad fires (`gun_range`), or the standard
    /// rifle's with none: what the mould's panic reach reads.
    pub fn gun_range_max(&self) -> f32 {
        self.live_squads().into_iter().map(|q| self.gun_range(q)).fold(None, |m: Option<f32>, r| Some(m.map_or(r, |m| m.max(r)))).unwrap_or_else(|| self.gun_range(SQUADS_MAX))
    }

    /// Squad `q`'s gun (owner 11-14 16:46Z: "The squad owns the gun"): its
    /// own if it was armed (`arm`), else the standard rifle at these rules.
    pub fn gun(&self, q: usize) -> Gun {
        self.squads.get(q).and_then(|s| s.gun).unwrap_or_else(|| Gun::standard(&self.rules))
    }

    /// Squad `q` takes `gun` (`Cmd::Arm`); `None` gives back the standard
    /// rifle. Its soldiers fire it from their next shot.
    pub fn arm(&mut self, q: usize, gun: Option<Gun>) -> bool {
        match self.squads.get_mut(q) {
            Some(s) => {
                s.gun = gun;
                true
            }
            None => false,
        }
    }

    fn add_soldier(&mut self, a: f32) {
        self.add_soldier_at(self.larder, a);
    }

    fn add_soldier_at(&mut self, at: (f32, f32), a: f32) {
        // Round the point (the larder, or a squad), out past a crystal it
        // sits on.
        let sc = self.s();
        let (x, y) = (0..30)
            .map(|n| (3.0 + n as f32) * sc)
            .map(|r| (at.0 + r * a.cos(), at.1 + r * a.sin()))
            .find(|&(x, y)| self.standable(x, y))
            .unwrap_or(at);
        self.soldiers.push(Soldier {
            x,
            y,
            hp: self.rules.hp,
            order: Order::Hold,
            stance: Stance::Defend,
            carry: 0.0,
            homing: false,
            dry: false,
            queue: [Order::Hold; QUEUE],
            queued: 0,
            alive: true,
            ready: 0,
            beat: 0,
            touching: 0,
            aim: None,
            dense: 0,
            smell: 1.0,
            load: 0.0,
            squad: 0,
            cover: false,
            rest: 0.0,
            next: 0,
            vx: 0.0,
            vy: 0.0,
            was: (x, y),
            doze: 0.0,
        });
    }

    /// Game seconds since the start (the landing, `Game::landed`).
    /// Food that has come in, in all: drained, pumped and guarded (the
    /// window's flow readout is its rate, so it is never negative: owner,
    /// 2026-11-15, "the monitor that says your energy flow should always
    /// be positive"; spending shows in the bank, not the flow).
    pub fn income(&self) -> f32 {
        self.harvested + self.guard_paid + self.rendered
    }

    /// Every living soldier and squad this step as CSV rows, dish cells
    /// (the path probe: `LARDER_PATHLOG` in the window's headless runs,
    /// `examples/pathprobe`; read by
    /// `Reports/pathfinding-2026-11-20/tools/paths.py`).
    /// `s,tick,k,squad,x,y,vx,vy,goal x,goal y,touching,order,cover,hp` and
    /// `q,tick,q,anchor x,anchor y,dest x,dest y,steer x,steer y,holding,column,fight`.
    pub fn path_rows(&self, f: &mut impl std::io::Write) -> std::io::Result<()> {
        let t = self.world.ticks();
        let s = self.s();
        for (k, so) in self.soldiers.iter().enumerate().filter(|(_, s)| s.alive) {
            let (gx, gy) = self.goal(k).unwrap_or((f32::NAN, f32::NAN));
            let o = match so.order {
                Order::Hold => 'h',
                Order::Move(..) => 'm',
                Order::Fight(..) => 'f',
                Order::Guard(..) => 'g',
                Order::Build(..) => 'b',
                Order::Clear(..) => 'c',
                _ => 'o',
            };
            writeln!(f, "s,{t},{k},{},{:.2},{:.2},{:.2},{:.2},{:.2},{:.2},{},{o},{},{:.0}", so.squad, so.x / s, so.y / s, so.vx / s, so.vy / s, gx / s, gy / s, so.touching, so.cover as u8, so.hp)?;
        }
        for q in self.live_squads() {
            let sq = self.squads[q];
            let (ax, ay) = sq.anchor.unwrap_or((f32::NAN, f32::NAN));
            let (dx, dy) = sq.dest.unwrap_or((f32::NAN, f32::NAN));
            let (vx, vy) = sq.via.unwrap_or((f32::NAN, f32::NAN));
            writeln!(f, "q,{t},{q},{:.2},{:.2},{:.2},{:.2},{:.2},{:.2},{},{},{},{:.2},{:.2}", ax / s, ay / s, dx / s, dy / s, sq.steer.0, sq.steer.1, sq.holding as u8, self.columns[q].on as u8, sq.fight as u8, vx / s, vy / s)?;
        }
        Ok(())
    }

    pub fn time(&self) -> f32 {
        (self.world.ticks() - self.landed) as f32 / self.rules.tps as f32
    }

    /// How far the mould is through overrunning the larder (`Rules::overrun`),
    /// 0-1: 0 while it is not held with the bank dry (or the rule is off),
    /// 1 as the round is lost. The window darkens the larder's sprite by it.
    pub fn overrun_share(&self) -> f32 {
        if self.rules.overrun <= 0.0 {
            return 0.0;
        }
        (self.held_ticks as f32 / self.secs(self.rules.overrun).max(1) as f32).min(1.0)
    }

    fn secs(&self, s: f32) -> u64 {
        (s * self.rules.tps as f32).round() as u64
    }

    pub fn alive(&self) -> usize {
        self.soldiers.iter().filter(|s| s.alive).count()
    }

}

#[cfg(test)]
mod tests;
#[cfg(test)]
mod narrow_tests;
#[cfg(test)]
mod wall_side_tests;
#[cfg(test)]
mod move_tests;
#[cfg(test)]
mod path_tests;
