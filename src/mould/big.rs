//! A mould dish of any size: the engine alone, for the GPU plan's phase 2
//! (`Reports/gpu-plan-2026-11-10.md`). Jones 2010's walkers lay trail and
//! steer by it and by food scent, with the rules phase 1 chose for a GPU, so
//! the scaling rule and the look of a dense network can be judged on the CPU
//! before any GPU code exists.
//!
//! What is left out on purpose: the game's economy (its shared store and
//! food flowing through tubes), the census, tools, levels and the wild
//! mould. The walker count is fixed, as in Jones 2010 and in Sage Jenson's
//! sketches, unless growth is on (`BigParams::upkeep`, Larder World's W0,
//! 2026-11-11): then each walker has its own energy, gains what it eats,
//! pays upkeep, starves, and divides into a free slot. The owner ruled
//! on 2026-11-10 (20:55Z) that today's dishes are unlikely to be the true
//! games, so this is judged by engine measures (`BigMeasure`), not stars.
//!
//! The rules phase 1 settled (2026-11-10):
//! - Walkers move all at once, as a GPU thread per walker would
//!   (`Params::all_at_once`): every walker senses the trail as it stood
//!   before anyone moved; with a crowd limit of 1 a walker may only enter a
//!   cell that was empty when the pass began, and the lower claim key wins
//!   (`keyed`); random draws are keyed on seed, step and walker.
//! - Trail and scent are `f32`. Fixed point erased a smell too faint to
//!   draw that the mould steers by (`Reports/dead-ends.md`,
//!   `Params::fixed_point`).
//! - Scent may live on today's grid while trail and walkers are finer
//!   (`BigParams::scent_coarse`), one of the review's scaling-rule options.
//!
//! Units: a *cell* is a fine cell of this dish; a *dish cell* is one of
//! today's 256 x 160 cells, `scale` cells across. A *tick* is one of today's
//! steps (`STEPS_PER_SECOND` a second); it holds `substeps` fine steps.

use std::f32::consts::TAU;

use super::{keyed, unit, H, STEPS_PER_SECOND, W};
use super::map::{self, Map, G_ROCK, G_WET, MAP_SCALE};

/// What a keyed draw is for here (`keyed`): its own streams, apart from the
/// game's.
const DRAW_TURN: u32 = 101;
const DRAW_BLOCKED: u32 = 102;
const CLAIM_MOVE: u32 = 103;
const DRAW_START: u32 = 104;
const DRAW_PILES: u32 = 105;
const DRAW_RESPAWN: u32 = 106;
const DRAW_HAZE: u32 = 107;
const DRAW_WANDER: u32 = 108;
const DRAW_CHANCE: u32 = 109;
const DRAW_SIZE: u32 = 110;
const DRAW_KEEP: u32 = 111;
/// Claims in `blocked_retry` round r use CLAIM_RETRY + r.
const CLAIM_RETRY: u32 = 112;
/// Growth (`upkeep`): past CLAIM_RETRY's 255 rounds, so no stream is shared.
const DRAW_STARVE: u32 = 401;
const DRAW_BIRTH: u32 = 402;
const CLAIM_BIRTH: u32 = 403;
const DRAW_MIX: u32 = 113;
const DRAW_FRUIT: u32 = 114;
const DRAW_SLEEP: u32 = 115;
const CLAIM_LAND: u32 = 116;
const DRAW_SPORE_H: u32 = 117;
const DRAW_SLEEP_DIE: u32 = 118;

/// Life states (`BigParams::spore_rate` and below, `BigDish::life`).
pub const LIFE_WALKER: u8 = 0;
pub const LIFE_SPORE: u8 = 1;
pub const LIFE_DORMANT: u8 = 2;
pub const LIFE_DEAD: u8 = 3;

/// One species' settings (`BigParams::species_sets`): each field means
/// what the `BigParams` field of the same name means, for that species'
/// walkers (`keep` for its own trail field).
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct SpeciesSet {
    pub sensor_angle: f32,
    pub sensor_offset: f32,
    pub rotate_angle: f32,
    pub step: f32,
    pub deposit: f32,
    pub keep: f32,
    pub crowd: u8,
    pub blocked_deposit: f32,
    pub graze_weight: f32,
    pub explore_steps: u16,
}

impl SpeciesSet {
    /// The global settings as a set: what every species uses when
    /// `species_sets` is empty.
    pub fn from_params(p: &BigParams) -> SpeciesSet {
        SpeciesSet {
            sensor_angle: p.sensor_angle,
            sensor_offset: p.sensor_offset,
            rotate_angle: p.rotate_angle,
            step: p.step,
            deposit: p.deposit,
            keep: p.keep,
            crowd: p.crowd,
            blocked_deposit: p.blocked_deposit,
            graze_weight: p.graze_weight,
            explore_steps: p.explore_steps,
        }
    }
}
const DRAW_TERRAIN: u32 = 140;
const DRAW_HOLD: u32 = 150;
/// Soil kinds in `BigDish::soil` (`BigParams::sand_keep`).
pub const SOIL_SAND: u8 = 1;
pub const SOIL_CLAY: u8 = 2;
pub const SOIL_BARREN: u8 = 3;

#[derive(Clone, Debug)]
pub struct BigParams {
    /// Fine cells per dish cell, each way: 1 is today's grid, 4 is 1024 x 640.
    pub scale: usize,
    /// Fine steps a tick. With `scale` 4 and `substeps` 4 a walker crosses a
    /// dish cell a tick, as today's do.
    pub substeps: usize,
    pub walkers: usize,
    pub sensor_angle: f32,
    pub rotate_angle: f32,
    /// How far ahead a walker senses, in fine cells.
    pub sensor_offset: f32,
    /// How far ahead a walker smells scent, in fine cells; 0 is
    /// `sensor_offset`. Trail sensed near sets the mesh (Jones's network
    /// size follows the sensor offset), scent smelled far sets how far off
    /// food pulls: on a finer grid one offset cannot keep both.
    pub scent_offset: f32,
    /// How far a walker moves a fine step, in fine cells.
    pub step: f32,
    pub deposit: f32,
    /// Trail kept each fine step, after the 3x3 mean.
    pub keep: f32,
    /// 3x3 mean passes on the trail each fine step (`keep` applied once).
    /// One pass spreads trail a quarter as far in dish terms on a grid four
    /// times finer, even with four fine steps a tick; `scale` passes a fine
    /// step give today's spread a tick (variance adds: `scale`^2 passes of a
    /// fine cell's 2/3 equal one pass of a dish cell's).
    pub trail_passes: usize,
    /// Fine steps between a walker's looks: it senses and turns on one, then
    /// walks straight. With `substeps` here a walker turns once a tick and
    /// walks today's path on the fine grid; with 1, a walker turns up to
    /// `rotate_angle` every fine step, `substeps` times as often a dish cell
    /// as today's.
    pub sense_every: usize,
    /// Walkers one cell may hold: 1 is today's rule, 0 no limit (as in
    /// Jenson's sketches, where particles pass through each other).
    pub crowd: u8,
    /// Scent on today's grid, stepped once a tick as today (true), or on the
    /// fine grid, stepped every fine step with the same numbers (false: the
    /// naive port, whose smell reaches `scale` times less far in dish terms).
    pub scent_coarse: bool,
    pub scent_keep: f32,
    /// Scent a food pile lays on each of its cells a scent step, until the
    /// network reaches it; then `fed_scent` of that, as today.
    pub food_deposit: f32,
    pub fed_scent: f32,
    /// Food piles, the first one home (the inoculum sits on it).
    pub piles: usize,
    /// Pile radius, in dish cells.
    pub pile_r: f32,
    /// Walkers start in a blob on the home pile (true) or spread over the
    /// whole dish (false, Jones's start).
    pub blob: bool,
    /// Chance a fine step that a walker jumps to a random agar cell with a
    /// new heading (0: never, today's rule). Jenson's later work (*36
    /// Points*, as Bleuje's port reads it) respawns each particle about
    /// every 1,000 steps (0.001); added 2026-11-10 to see whether it gives
    /// the haze of free walkers in Jenson's clips.
    pub respawn: f32,
    /// With `blob`, the starting blob's radius in dish cells, centred on the
    /// dish, walkers drawn evenly over it; 0 is today's packed inoculum on
    /// the home pile. Added 2026-11-11 for Jenson's network_2, which starts
    /// from a dense disc in the middle of the dish.
    pub blob_r: f32,
    /// With `blob`, the share of walkers that start spread evenly over the
    /// whole dish instead (a haze around the seed); 0 is today's.
    pub haze: f32,
    /// Trail-dependent settings (Jenson's *36 Points*, as Bleuje's port
    /// reads it). 0 is off (today's). Otherwise each walker, when it looks,
    /// reads the trail at its own cell as the pass began and takes
    /// `s = min(trail * sense_scale, 1)`; then, with each `*_mod` = `[amp,
    /// exp]`, sensor offset = `sensor_offset + amp * s^exp`, likewise sensor
    /// angle and rotate angle (radians) and step (fine cells). The scent
    /// reach is not changed. A step under 0 is taken as 0.
    pub sense_scale: f32,
    pub offset_mod: [f32; 2],
    pub angle_mod: [f32; 2],
    pub turn_mod: [f32; 2],
    pub step_mod: [f32; 2],
    /// Capped square-root deposit (Bleuje's port of *36 Points*). 0 is
    /// today's: each walker that moves lays `deposit` where it lands.
    /// Otherwise, after all walkers have moved, each cell holding n walkers
    /// (moved or not) gets `deposit * sqrt(min(n, deposit_cap))`, so a crowd
    /// lays less trail a walker than a lone one.
    pub deposit_cap: u16,
    /// Sensing floor. 0 is off (today's). Otherwise a walker whose three
    /// sensors all read under `sense_floor` (trail plus scent) ignores them
    /// and turns `rotate_angle` left or right at random (a keyed draw), so
    /// trail too faint to matter leaves it wandering. Added 2026-11-11: with
    /// no floor any haze, however thin, organises on its own, because the
    /// turn rule compares sensors only.
    pub sense_floor: f32,
    /// Chance a look that a walker ignores its sensors and takes a random
    /// heading (a keyed draw); 0 is never (today's). Lets walkers leak out of
    /// tubes into a glow around them.
    pub wander: f32,
    /// Contagion (Jenson's 2019 post: an infected particle lays an infected
    /// trail, which spreads to other particles). Off when 0. Each walker
    /// holds a counter c: 0 is healthy, over `immune_steps` is sick,
    /// 1..=`immune_steps` is immune. A healthy walker whose cell's germ
    /// field (as the pass began) is over `sick_catch` sets c =
    /// `sick_steps + immune_steps`. A sick walker moves `sick_speed` times
    /// as far and deposits `sick_trail` times as much trail, and lays
    /// `sick_deposit` germ on its cell; it senses `sick_reach` times as far
    /// and turns `sick_turn` times as sharply (after `sense_scale`). c falls by one each fine step after
    /// the move, to 0. The germ field gets one 3x3 mean a fine step, times
    /// `sick_keep`. Walkers within `sick_seed_r` dish cells of the dish
    /// centre start sick (`BigDish::infect` seeds it later).
    pub sick_steps: u16,
    pub immune_steps: u16,
    pub sick_catch: f32,
    pub sick_deposit: f32,
    pub sick_keep: f32,
    pub sick_speed: f32,
    pub sick_trail: f32,
    pub sick_reach: f32,
    pub sick_turn: f32,
    pub sick_seed_r: f32,
    /// A sick walker's upkeep is x this (1: sickness costs nothing). The
    /// squad toolkit's germ (`mould::kit`, 2026-11-12): contagion that only
    /// changed steering left no mark by 20 s (`Reports/dead-ends.md`, the
    /// contagion entry: "re-test when sickness changes trail or food"), so
    /// the germ's sickness burns energy, and a wave that runs along the
    /// tubes leaves them thinned where the walkers starve.
    pub sick_upkeep: f32,
    /// Area sensors (Lague's Slime-Simulation): a sensor reads the sum over
    /// the (2r+1) x (2r+1) square of agar cells centred where it points,
    /// not one cell. 0 is one cell (today's). The square's centre cell
    /// decides the wall; scent is still read at one point. Added 2026-11-11.
    pub sensor_r: u8,
    /// Species (Lague): walker k belongs to species k % `species`, each with
    /// its own trail field, laid, spread and kept as the trail is; `trail`
    /// is then their sum, for drawing and the measures. A walker senses
    /// own - `species_avoid` x others = (1 + avoid) x own - avoid x total
    /// (Lague's mask gives 1). 0 or 1 is one species (today's). Added
    /// 2026-11-11 for the RTS direction: moulds that share one dish.
    pub species: u8,
    pub species_avoid: f32,
    /// Conversion between species (0 is off; added 2026-11-11 for species
    /// that fight over a border). Each step, after moving, a walker whose
    /// cell holds more than `species_convert` times its own species' trail
    /// of another species' trail joins that species (the largest other,
    /// lowest index on ties). Values above 1 leave a margin, so a border
    /// moves only where one side clearly outweighs the other. With
    /// `species_cycle` only the previous species (the one that chases you)
    /// can take you, as in rock, paper, scissors. Walkers keep their species
    /// through respawn.
    pub species_convert: f32,
    /// Conversion compares each species' trail summed over the
    /// (2r+1) x (2r+1) square round the walker's cell (clipped at the dish
    /// edge) instead of the cell alone (0 is the cell; added 2026-11-11 so a
    /// bigger body takes a smaller one where they touch: at a contact the
    /// side with more trail within r wins, and a small body is outweighed
    /// across its whole width). Built from one summed-area table a species
    /// each step (f64, so the sums are exact enough at 1M walkers).
    pub species_convert_r: u32,
    /// Conversion weighs each species' trail divided by that species'
    /// deposit (its own `SpeciesSet::deposit`, else the global), so it
    /// counts walkers' presence rather than how much each lays (added
    /// 2026-11-12 for the player's own mould: with raw trail, a form that
    /// lays three times the deposit converted the race network whole within
    /// 60 s, so deposit was a stat that won every fight). Off by default; CPU
    /// only, not yet on the GPU engine.
    pub species_convert_per: bool,
    /// Species 0 lays this many times the deposit (1 is even; added
    /// 2026-11-11 to give one side an edge in a fight).
    pub species_boost: f32,
    /// Species 0's share of the walkers (0 is even, k % n). Otherwise
    /// walker k is species 0 when (k x share) mod 1 + share >= 1, so the
    /// shares interleave down the walker list, and the rest are species
    /// 1 + k % (n - 1). With `species_zones` the bands stay equal, so
    /// a bigger share is a *denser* side, not more ground (equal density on
    /// wider ground would just be a bigger, equally strong territory).
    /// Added 2026-11-11: an edge in numbers.
    pub species_share: f32,
    /// Grazing (0 is off; added 2026-11-11 to make bodies roam, after
    /// Primordia's "Symbiosis" mode). A grass field starts at 1 on every agar
    /// cell; each step every walker eats `graze_eat` from its cell (not below
    /// 0) and grass regrows by `graze_regrow` x (1 - grass). A sensor adds
    /// `graze_weight` x grass at the cell it reads, so a body eats the ground
    /// under it bare and leans toward fresh ground.
    pub graze_weight: f32,
    pub graze_eat: f32,
    pub graze_regrow: f32,
    /// With `species` > 1 and a spread start (`blob` false), species s
    /// starts in its own vertical band of the dish, s/n to (s+1)/n of the
    /// width (false: all mixed), so moulds begin as neighbours.
    pub species_zones: bool,
    /// Chance-weighted turn (MCPM, Elek et al. 2022). 0 is off (today's
    /// Jones rule). Otherwise, at a look, the walker picks the left or right
    /// sensor by a keyed draw and turns `rotate_angle` toward it with
    /// probability g^e / (f^e + g^e), f the front sensor and g that side
    /// (each floored at 0; 1/2 if both are 0), e = `chance_exp`; else it
    /// goes straight. The wander and floor rules still come first.
    pub chance_exp: f32,
    /// Cyclic species (Primordia's "Chasing Waves", MIT): with `species` 3
    /// or more and this on, a walker of species s weighs species s+1's
    /// trail by `species_next` and species s-1's by `species_prev` (mod n)
    /// instead of -`species_avoid`; any others stay at -`species_avoid`.
    /// Primordia's preset: own 1, next 0.6, previous -1.
    pub species_cycle: bool,
    pub species_next: f32,
    pub species_prev: f32,
    /// Explorers and networkers (Reginato, Proverbio and Giordano 2024;
    /// Defei-Wang's two deposits). 0 is off. Otherwise each walker holds an
    /// age, reset to 0 at the start and on `respawn`, counted up a fine step.
    /// While its age is under `explore_steps` it is an explorer: it steers
    /// by the same rule toward the *lowest* sensor (walls still read as
    /// walls) and lays `explore_deposit` times the deposit; after, it is an
    /// ordinary walker.
    pub explore_steps: u16,
    pub explore_deposit: f32,
    /// Spread of the explorer stage (0 is off): walker k explores for
    /// explore_steps x (1 + spread x (2u - 1)) steps, u in [0, 1) from a
    /// keyed draw on k alone (`DRAW_SIZE` with the top bit set), so walkers
    /// turn into networkers one by one rather than all at once. Those that
    /// turn early build a network behind the explorers still pushing out.
    pub explore_spread: f32,
    /// Per-walker size (Primordia's "Dendrites", MIT). 0 is off. Otherwise
    /// walker k's sensor offset and step are both multiplied by 2^(spread x
    /// u), u in [-1, 1] from a keyed draw on k alone, fixed for its life, so
    /// big walkers see and move far and small ones near.
    pub size_spread: f32,
    /// Mixed walkers in one mould (added 2026-11-11, owner: "what if some of
    /// the walkers had different properties than other ones?"). Walker k is
    /// the second kind when unit(keyed(seed, 0, DRAW_MIX, k)) < `mix_share`
    /// (0 is off: every walker is the first kind). Both kinds lay and read
    /// the same trail; the second kind's sensor offset, sensor angle, turn
    /// and step are the first kind's times `mix_reach`, `mix_angle`,
    /// `mix_turn` and `mix_step`, and everything it lays (moving, blocked or
    /// capped) is times `mix_deposit` (1 is the same). The kind is fixed for
    /// the walker's life, through respawn. Scent is read at `scent_offset`
    /// for both kinds: `mix_reach` does not make a walker smell farther.
    pub mix_share: f32,
    pub mix_reach: f32,
    pub mix_angle: f32,
    pub mix_turn: f32,
    pub mix_step: f32,
    pub mix_deposit: f32,
    /// Each species' own settings (empty is off: every species uses the
    /// global fields). Added 2026-11-11 after the independent review
    /// (`Reports/research-review-2026-11-11.md`): with one global set, a
    /// droplet species (crowd 1, wide sensors) and a network species (crowd
    /// 0) could not share a dish, which the Larder World ladder needs. When
    /// not empty it holds one `SpeciesSet` per species (`species` of them),
    /// and a walker of species s uses `species_sets[s]` in place of the
    /// global field of the same name; every other field stays global. A
    /// walker that converts takes its new species' set from the next step.
    /// The crowd rule counts every walker on a cell, whatever its species: a
    /// walker of a species with crowd c may enter a cell only while it holds
    /// fewer than c walkers of any species, and crowd 0 enters any cell.
    /// Only walkers of a species with `graze_weight` not 0 eat grass.
    pub species_sets: Vec<SpeciesSet>,
    /// Blocked walkers keep their heading (Jones and Adamatzky 2012, the
    /// amoeboid model) rather than drawing a new one, except with chance
    /// `blocked_reset` a step, when they draw one as today. False is today's
    /// rule. Only matters with a crowd limit.
    pub blocked_keep: bool,
    pub blocked_reset: f32,
    /// Retry rounds a step for blocked walkers (0 is today). In each round
    /// every still-blocked walker tries the same step again into a cell
    /// that is now below the crowd limit; ties go to the lowest keyed draw
    /// (`CLAIM_RETRY` + round), as in the first claim. Losers stay blocked.
    /// Only matters with a crowd limit.
    pub blocked_retry: u8,
    /// The share of the deposit a blocked walker lays where it stands (0 is
    /// today: only a walker that moves lays trail). Packed walkers then
    /// keep their own crowd smelling of trail, so a blob holds together
    /// instead of its edge, the only part moving, drawing the crowd out.
    pub blocked_deposit: f32,
    /// Growth and starving (Larder World W0, 2026-11-11). 0 is off: the
    /// walker count is fixed. Otherwise each walker holds energy, starting
    /// at `start_energy`. A fine step it gains what it eats (its equal share
    /// of the grass its cell lost, `graze_eat`, so the walkers on a cell
    /// share alike whatever their order) and pays `upkeep`. At zero it
    /// starves: it dies with chance `starve` a step, unless it eats first,
    /// so a starving mould thins rather than vanishing at once. A walker
    /// with `birth_energy` or more divides: its child lands in a free agar
    /// cell beside it (one of the eight, by a keyed draw; with a crowd
    /// limit, ties go to the lowest claim) and each keeps half of what is
    /// left after `birth_cost`. Walkers hold fixed slots, `capacity` of
    /// them (0: `walkers`), of which the first `walkers` placed start
    /// alive, so a walker's keyed draws stay its own for its life, as a GPU
    /// thread per slot needs. A child takes slot k xor m of its parent k, if
    /// that slot is empty, with m a keyed draw a step: xor with one mask
    /// pairs the slots, one parent a slot, so no order decides who is born,
    /// and a parent finds an empty slot as often as slots are empty. (A
    /// first try cycled m through the single bits; with the living in the
    /// first slots most masks paired the living with each other, so a small
    /// mould could breed only about one step in five.)
    pub upkeep: f32,
    pub start_energy: f32,
    pub birth_energy: f32,
    pub birth_cost: f32,
    pub starve: f32,
    pub capacity: usize,
    /// Energy that flows through the mould (2026-11-11, after the Larder
    /// thread's sizing runs; 0 is off, CPU only so far). Each step after eating, the
    /// walkers' mean energy on each cell evens out with the eight cells
    /// round it that hold walkers: a cell gains `energy_flow` x the sum of
    /// (neighbour mean - own mean), shared alike by its walkers. Pairs give
    /// and take the same, so energy is kept. Why: with births only into a
    /// free cell beside the parent, walkers packed on a crystal fill up
    /// (mean energy 30 at 300 s on charge 512) and never pass it on, while
    /// children born at the edge get about 0.4 and starve; the mould
    /// levelled off at 18-24k walkers whatever the charge. Flowing energy
    /// lets a crystal's food reach the front through the tubes, so food
    /// can become size, and a cut tube starves what is beyond it. Stable
    /// up to 1/8 (eight neighbours, one walker a cell).
    ///
    /// Measured (Reports/mould-shape-2026-11-11.md, *On crystals*, seed 1,
    /// 300 s): 0.05 turns the hoard into walkers (mean energy 0.66 against
    /// 26-30; 24,206 / 22,515 alive against 21,015 / 18,200 at charge 512 /
    /// 4096) but size still does not follow charge: the crystals' area sets
    /// the intake. 0.12 evens energy below `birth_energy` and fewer divide
    /// (20,782). Off by default: a cut tube's effect is untested.
    pub energy_flow: f32,
    /// **Hunger** (2026-11-12, the one-body thread; 0 is off, needs growth).
    /// A walker's scent reading is multiplied by 1 + `hunger_scent` x its
    /// hunger, where hunger is 1 - energy / `hunger_at` (0 at or above
    /// `hunger_at`): a hungry walker follows food harder than a fed one.
    pub hunger_scent: f32,
    pub hunger_at: f32,
    /// A starving walker (hunger `hunt_at` or more) smells further: its
    /// scent sensors reach `scent_offset` x (1 + `hunger_reach` x hunger)
    /// (0 is off). So a cut-off hunk with no food in reach widens its
    /// search as it starves. (Applied at any hunger, `hunger_reach` 3 broke
    /// the fed mould: walkers smelled past their own crystal and the mould
    /// fell to 0-1,900 walkers at 150 s against H's 6,600-11,900.)
    pub hunger_reach: f32,
    /// The hunger at which a walker hunts (`hunger_reach`, and `tether`
    /// when this is over 0); 0: every walker, at any hunger.
    pub hunt_at: f32,
    /// **Body pull** (2026-11-12, the one-body thread; 0 is off). Owner,
    /// 04:13-04:18Z: the mould "didn't feel like one cohesive organism"; a
    /// cut-off part should "feel like a cut off hunk and not a bunch of
    /// individuals". Each sensor also reads `body_pull` x the walkers in
    /// the square of radius `body_r` fine cells round its point, counted as
    /// the step began, scaled by the walker's own loneliness: 1 - n /
    /// `body_lone`, with n the other walkers in the same square round its
    /// own cell (1 when `body_lone` is 0). So a walker inside the mass
    /// follows trail and scent as before, and a straggler is drawn back to
    /// the crowd: streams of single walkers pull into a mass.
    pub body_pull: f32,
    pub body_r: u8,
    pub body_lone: f32,
    /// **Tether** (2026-11-12, the one-body thread; 0 is off). A walker may
    /// step into another cell only if that cell has at least `tether`
    /// other walkers among its eight neighbours (the mover not counted), or,
    /// if it has fewer than `tether` others round its own cell, at least as
    /// many as there: no walker steps away from the crowd it touches, so
    /// the mould crawls as a connected mass and cannot thin into streams
    /// of single walkers. A walker touching none moves freely and sticks to
    /// the first walkers it meets. A walker that cannot step is blocked.
    /// (A first try let any walker under `tether` move freely: the
    /// stragglers kept the streams, and the crystals' lumps came apart.)
    pub tether: u8,
    /// With `tether`: the chance a walker holds still a step (a keyed draw),
    /// and then movers count only the holders round a cell, so every
    /// mover lands touching a walker that stays (0: everyone may move, and
    /// counts everyone). Without it two touching walkers can step apart in
    /// the same step, each counting the other where it stood.
    pub tether_hold: f32,
    /// Upkeep times this for a walker with no other walker on the eight
    /// cells round it (1 is off; growth only): a walker cut off from the
    /// body is not the organism, and it starves fast unless it eats.
    pub lone_upkeep: f32,
    /// The square's radius, fine cells, in which `lone_upkeep` looks for
    /// another walker (1: the eight cells round it).
    pub lone_r: u8,
    /// Life cycle (added 2026-11-11 for the Larder ladder's Sporer and
    /// Sleeper tiers, and for how a mould answers the player's attack,
    /// `BigDish::strike`). Each walker holds a life state and a timer
    /// (`BigDish::life`, `BigDish::life_t`), allocated only when a rule
    /// below is on or a strike lands: 0 is an ordinary walker, 1 a spore in
    /// flight, 2 dormant, 3 dead. An ordinary or dormant walker's timer
    /// counts down by one each fine step, after the ordinary walkers have
    /// moved and before that step's changes of state are applied; while an
    /// ordinary walker's timer is above 0 it can neither fruit nor fall
    /// dormant. Spores and the dead hold no cell; the dormant keep theirs.
    ///
    /// **Spores** (off when `spore_rate` and `spore_burst` are 0; after
    /// Physarum's fruiting, which starvation and light trigger, and the
    /// expander review's Sporer, which jumps between crystals). At the start
    /// of a step, an ordinary walker with timer 0 becomes a spore with chance
    /// `spore_rate` if the scent on its cell (as the pass began) is over
    /// `spore_scent` (so a mould can fruit only where it feeds), plus
    /// `spore_burst` if the alarm there is over
    /// `spore_alarm` (a keyed draw, `DRAW_FRUIT`). It leaves its cell (the
    /// cell is free from the next step), takes a keyed random heading
    /// (`DRAW_SPORE_H`) and its timer becomes `spore_steps`. After the
    /// ordinary walkers have moved, each spore that was one when the step
    /// began, with timer T (before this step's count): in the first half of
    /// its flight (T over `spore_steps` / 2, integer division) it flies
    /// straight and cannot land early, so it leaves the crystal it fruited
    /// on; in the second half (T from `spore_steps` / 2 down to 1) it steers
    /// by scent alone with three sensors `spore_reach` fine cells out
    /// (`sensor_angle` apart, a sensor off the agar reads as a wall), turning
    /// `rotate_angle` toward the larger as the trail rule does (straight on
    /// if `spore_reach` is 0); with T at 0 it drifts straight, so a spore over
    /// a full crystal drifts off it to a free cell instead of circling. It
    /// moves `spore_speed` x `step` ignoring the crowd, or, if that point is
    /// off the agar, stays and takes a new keyed heading; lays
    /// `spore_deposit` x `deposit` on its cell (a faint streak, so the jump
    /// shows); counts its timer down. With the timer now at 0, or in the
    /// second half with the scent on its cell over `spore_land` (when that is
    /// over 0), it lands: it claims its cell if the cell is under the crowd
    /// limit, lowest keyed claim winning (`CLAIM_LAND`); the winner is an
    /// ordinary walker again with timer `spore_rest`, a loser flies on.
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
    /// **Dormancy** (off when 0; after the sclerotium, Jones 2015 survey
    /// pp. 23-26, and the expander review's Sleeper). At the start of a step
    /// an ordinary walker with timer 0 falls dormant with chance
    /// `sleep_rate` (keyed, `DRAW_SLEEP`) if the scent on its cell is under
    /// `wake_scent` and the alarm under `wake_alarm` (fields as the pass
    /// began). A dormant walker keeps its cell, neither senses nor moves, and
    /// each step after the ordinary walkers have moved lays `sleep_deposit`
    /// x `deposit` on its cell; then, if the scent there is over
    /// `wake_scent` or the alarm over `wake_alarm`, it wakes: an ordinary
    /// walker with timer `wake_steps`, and it adds `alarm_relay` to the alarm
    /// on its cell, once, so a wake can run through a dormant crust. A walker
    /// that falls dormant sets its timer to `sleep_rest`; while a dormant
    /// walker's timer is over 0 it counts down a step and alarm cannot wake
    /// it (scent still can): after a wake has run through it, a crust is deaf
    /// to alarm for that long, the window a bait strike opens.
    pub sleep_rate: f32,
    pub sleep_deposit: f32,
    pub wake_scent: f32,
    pub wake_alarm: f32,
    pub wake_steps: u16,
    pub sleep_rest: u16,
    /// Each step a dormant walker dies with this chance (keyed,
    /// `DRAW_SLEEP_DIE`): its cell is freed, and with growth on its slot
    /// too; 0 (the default) keeps it for ever. A free crust never dies,
    /// which froze a world whose crystals ran dry (review 1 of 2026-11-11:
    /// the whole body asleep and unchanged from 30 s to 90 s). Paying
    /// upkeep while asleep was tried first and lost the bait counter in the
    /// world (`Reports/dead-ends.md`, `sleep_upkeep`): the crust starved in
    /// seconds, since walkers fall asleep with little energy left. A death
    /// chance keeps the crust whole through a bait's few seconds and lets an
    /// unfed one fade over a minute or two.
    pub sleep_death: f32,
    /// A gassed walker (`BigDish::gas`, the squad toolkit's sleep gas)
    /// dies with this chance each step while its gas lasts (0 the default).
    pub gas_death: f32,
    /// **Alarm** (off when `alarm_keep` is 0): a field on the fine grid that
    /// a strike lays (`BigDish::strike`) and a waking walker relays. Each
    /// fine step it gets one 3x3 mean over the agar times `alarm_keep`, as
    /// the germ does. A sensor adds `alarm_weight` x alarm at the cell it
    /// reads (with the trail and any grass): over 0 the mould swarms to a
    /// wound, under 0 it flees one, as Jones and Adamatzky's light pushes
    /// their blob (2012).
    pub alarm_keep: f32,
    pub alarm_weight: f32,
    pub alarm_relay: f32,
    /// Trail above this counts as tube in the measures (today's `tube`).
    pub tube: f32,
    /// Larder World crystals (W1 of `Reports/larder-world-roadmap-2026-11-11.md`;
    /// 0 is off). Every fine agar cell of a pile holds this much charge at
    /// the start and none is ever added: a crystal runs dry. A pile's scent
    /// is `food_deposit` x the share of its charge left, so a drained
    /// crystal stops calling, and the picture draws the charge itself (the
    /// glow is the number, not an effect). The owner's rulings, 11-11 14:06Z:
    /// food becomes energy crystals that run out, which forces expansion.
    pub crystal_charge: f32,
    /// Charge a walker takes from its cell a fine step. The n walkers on a
    /// cell take equal shares, min(`crystal_eat`, charge / n), so the take
    /// does not depend on walker order (a count per cell, as on a GPU); the
    /// GPU thread's growth step (W0) credits it to each walker's energy.
    pub crystal_eat: f32,
    /// Crystals that refill (0 is off: they run dry). Each fine step a
    /// crystal cell regains this share of what it is missing. The owner,
    /// 11-11 14:54Z: crystals that run out go with the race start (one
    /// spot, it must expand); a landing on a world mostly mould probably
    /// wants crystals that don't run out. Both are tested; 14:55Z: "just my
    /// initial thought", to be settled by playtests.
    pub crystal_regrow: f32,
    /// Charge that flows inside a crystal (0 is off). Each fine step every
    /// cell of a pile moves this share of the way to the pile's mean, so
    /// the total is kept and a tube that touches one edge of a disc drains
    /// all of it, and the whole disc dims together. Why: the GPU thread's
    /// first growth run on run-dry crystals (11-11 15:27Z, one seed) saw
    /// the mould eat only the cells its tubes cross and starve with 61% of
    /// the charge still there; the owner wants an idle player to lose to
    /// a mould that keeps growing (14:54Z). On a GPU it is a per-pile sum
    /// (an atomic add per cell into a few dozen slots) and a second pass.
    ///
    /// 0.01 (worldwall, 11-11, 512 x 320, 20k walkers, 23 crystals that
    /// run dry): the share of the other crystals' charge eaten by 40 s goes
    /// from 0.476 to 0.829 (mean of 12 seeds), higher on 12 of 12 (sign
    /// test p 0.0005, survives Holm over 3 flows); 0.05 and 0.2 do no
    /// better (0.749, 0.800 on 6 seeds). The cost is within noise (7.8 ms
    /// a tick with and without, 4 threads).
    pub crystal_flow: f32,
    /// Terrain (0 is off): a smooth keyed noise on today's grid, blobs about
    /// `terrain_blob` dish cells across, the same layout at every scale.
    /// Where the first channel is under `terrain_rock` the ground is rock:
    /// not agar, so no walker, trail or scent enters it. Where the second
    /// is under `terrain_wet` it is wet: a walker's step there is x
    /// `wet_step`. Pile discs are kept clear of rock.
    pub terrain_rock: f32,
    pub terrain_wet: f32,
    pub wet_step: f32,
    pub terrain_blob: f32,
    /// Soil kinds (Larder World maps, WM3; off while `BigDish::soil` is
    /// empty, which it is unless a map or a harness fills it): a fine
    /// cell's kind, SOIL_SAND, SOIL_CLAY or SOIL_BARREN (0 is plain). After
    /// each fine step's trail spread, trail on sand is x `sand_keep` (it
    /// fades fast, so tubes there are thin) and on clay x `clay_keep` (it
    /// lasts). On barren ground upkeep is x `barren_upkeep` and no child is
    /// born there. Why these three: the brainstorm ranked ground by the
    /// model term it changes, and these touch trail kept and growth, the
    /// terms that steer and size the mould, where wet ground (the step)
    /// had no measured effect (Reports/larder-maps-2026-11-11.md).
    pub sand_keep: f32,
    pub clay_keep: f32,
    pub barren_upkeep: f32,
    /// Trail on a scorched cell (`BigDish::scorch_disc`, a blast's crater in
    /// the squad toolkit) is x this after each fine step's spread while the
    /// scorch lasts. Why a timed field and not a soil kind: a crater must
    /// fade back to the map's own ground, and the map's soil stays as drawn.
    /// Why scorch at all (owner, 2026-11-12 04:25Z: "do we need scortch when
    /// we have walls?"): a wall stops walkers, soldiers and shots; a crater
    /// stops only the mould's trail from holding, so a blast's hole or a
    /// det cord's cut stays open instead of re-knitting in seconds (the
    /// knife's 4 s heal, `Reports/dead-ends.md`, `STORM_CUTS`).
    /// 0.1 since 2026-11-12, when shots began to lay it (`Rules::crater`):
    /// the hole under a squad's fire was +28 dish cells over no crater at
    /// 0.1 and +19 at 0.5 (race world, 8 seeds, 8 of 8 larger each); the
    /// toolkit thread's probes at 0.1 kept a blast's disc as empty at 20 s
    /// and a det cord's cut cleaner (trail 67 against 137 at 20 s). Was 0.5.
    pub scorch_keep: f32,
    /// Fine steps of scorch at which a cell bites at full strength: below
    /// it the cut is `scorch_keep`'s scaled by steps left / `scorch_full`,
    /// so every crater fades out instead of stopping at once, and a writer
    /// sets both strength and length by the steps it writes (a blast's deep
    /// crater, a shot's faint stamp; the owner's "stamps that fade",
    /// 2026-11-12 04:38Z, on the graphics thread's A/B). 30 (one game
    /// second at scale 2) since 2026-11-12: at 300 a shot's 10 s crater
    /// bit at full strength only as it was laid (hole +15 against +19 at
    /// 30), and a blast's 600 steps sit at full either way. Was 300.
    pub scorch_full: u16,
    /// A rectangular world, three dish cells in from the edge, instead of
    /// the dish's ellipse: a world, not a dish (owner, 11-11 14:01Z).
    pub rect: bool,
    /// Wall health a fine cell (0: walls never break). Owner, 11-11 15:24Z:
    /// the mould must be able to damage walls or grow over them, with a
    /// big slowdown, or a wall round a crystal shuts it out for good. Each
    /// fine step every walker blocked by a wall cell it tried to enter
    /// takes `wall_bite` from that cell; at 0 the cell is ground again, as
    /// rubble: wet (the step x `wet_step`) and open to scent. The picture
    /// draws a wall's brightness from its health.
    ///
    /// 400, a first number for play (worldwall, 11-11, 512 x 320, 20k
    /// walkers, 3 seeds): a wall 2 dish cells thick round the whole home
    /// blob, the hardest the mould can press, loses its first cell at
    /// 3.4-4.7 s and lets 6-660 walkers out by 20 s, 2,000-2,900 by 40 s.
    /// At 1,600 the first cell goes at 13.6-15.9 s and none are out by
    /// 40 s; at 6,400 nothing breaks. A tube passing a ringed crystal
    /// bites it 0-76 times in 35 s (5 seeds), so only a crowd breaks a
    /// wall. Recalibrate once the mould grows (W0).
    pub wall_hp: f32,
    pub wall_bite: f32,
    /// The share of scent a walled scent cell keeps each scent step (0:
    /// walls stop scent). Scent spreads into a wall and fades there, so a
    /// little gets across and a walled crystal still calls, faintly. Why:
    /// with walls that stop scent, a ring round a crystal hides it, and the
    /// mould never presses on the ring however weak it is (worldwall ring
    /// arm, 11-11: no bite and no broken cell in 35 s at health 30 or 300,
    /// 2 seeds of 2 valid; a fixed-size mould presses nowhere).
    ///
    /// 0.9 (11-11, with the GPU thread's growth, scratch; 512 x 320, charge
    /// 512, a health-400 ring round the nearest unreached crystal at 5 s,
    /// 90 s, 8 seeds): the ring was bitten on 2 / 4 / 7 seeds of 8 at
    /// 0 / 0.5 / 0.9, broken through in places on 1 / 3 / 6, and the
    /// crystal inside eaten on 0 / 0 / 2 (both at 86 s). Bites, 0.9 against
    /// sealed: more on 6 of 7 untied seeds (p 0.125); not significant, but
    /// the trend is steady and the owner's ruling is that a walled crystal
    /// must be reachable (15:24Z). Sealed, the mould walks past a ringed
    /// crystal as if it were not there.
    pub wall_scent: f32,
    pub seed: u64,
}

impl BigParams {
    /// Today's numbers on today's grid: the base every scaled dish is judged
    /// against. 2,000 walkers is about today's peak.
    pub fn today() -> Self {
        BigParams {
            scale: 1,
            substeps: 1,
            walkers: 2_000,
            sensor_angle: std::f32::consts::FRAC_PI_4,
            rotate_angle: std::f32::consts::FRAC_PI_4,
            sensor_offset: 9.0,
            scent_offset: 0.0,
            step: 1.0,
            deposit: 5.0,
            keep: 0.9,
            trail_passes: 1,
            sense_every: 1,
            crowd: 1,
            scent_coarse: true,
            scent_keep: 0.97,
            food_deposit: 20.0,
            fed_scent: 0.15,
            piles: 7,
            pile_r: 6.0,
            blob: true,
            respawn: 0.0,
            blob_r: 0.0,
            haze: 0.0,
            sense_scale: 0.0,
            offset_mod: [0.0, 1.0],
            angle_mod: [0.0, 1.0],
            turn_mod: [0.0, 1.0],
            step_mod: [0.0, 1.0],
            deposit_cap: 0,
            sense_floor: 0.0,
            wander: 0.0,
            sick_steps: 0,
            immune_steps: 0,
            sick_catch: 1.0,
            sick_deposit: 1.0,
            sick_keep: 0.9,
            sick_speed: 1.0,
            sick_trail: 1.0,
            sick_reach: 1.0,
            sick_turn: 1.0,
            sick_seed_r: 0.0,
            sick_upkeep: 1.0,
            sensor_r: 0,
            species: 0,
            species_avoid: 1.0,
            species_convert: 0.0,
            species_convert_r: 0,
            species_convert_per: false,
            species_boost: 1.0,
            species_share: 0.0,
            graze_weight: 0.0,
            graze_eat: 0.05,
            graze_regrow: 0.002,
            species_zones: false,
            chance_exp: 0.0,
            species_cycle: false,
            species_next: 0.6,
            species_prev: -1.0,
            explore_steps: 0,
            explore_spread: 0.0,
            explore_deposit: 0.05,
            size_spread: 0.0,
            mix_share: 0.0,
            mix_reach: 1.0,
            mix_angle: 1.0,
            mix_turn: 1.0,
            mix_step: 1.0,
            mix_deposit: 1.0,
            species_sets: Vec::new(),
            blocked_keep: false,
            blocked_reset: 0.01,
            blocked_retry: 0,
            blocked_deposit: 0.0,
            upkeep: 0.0,
            start_energy: 0.5,
            birth_energy: 1.0,
            birth_cost: 0.2,
            starve: 0.01,
            capacity: 0,
            energy_flow: 0.0,
            hunger_scent: 0.0,
            hunger_at: 1.0,
            hunger_reach: 0.0,
            hunt_at: 0.0,
            body_pull: 0.0,
            body_r: 2,
            body_lone: 0.0,
            tether: 0,
            tether_hold: 0.0,
            lone_upkeep: 1.0,
            lone_r: 1,
            spore_rate: 0.0,
            spore_scent: 0.0,
            spore_alarm: 0.0,
            spore_burst: 0.0,
            spore_steps: 240,
            spore_speed: 3.0,
            spore_reach: 0.0,
            spore_deposit: 0.0,
            spore_land: 0.0,
            spore_rest: 1200,
            sleep_rate: 0.0,
            sleep_deposit: 1.0,
            wake_scent: 1.0,
            wake_alarm: 1.0,
            wake_steps: 240,
            sleep_rest: 0,
            sleep_death: 0.0,
            gas_death: 0.0,
            alarm_keep: 0.0,
            alarm_weight: 0.0,
            alarm_relay: 0.0,
            tube: 1.0,
            crystal_charge: 0.0,
            crystal_eat: 0.05,
            crystal_regrow: 0.0,
            crystal_flow: 0.01,
            terrain_rock: 0.0,
            terrain_wet: 0.0,
            wet_step: 0.5,
            terrain_blob: 12.0,
            sand_keep: 0.8,
            clay_keep: 1.05,
            barren_upkeep: 3.0,
            scorch_keep: 0.1,
            scorch_full: 30,
            rect: false,
            wall_hp: 400.0,
            wall_bite: 1.0,
            wall_scent: 0.9,
            seed: 1,
        }
    }

    pub fn w(&self) -> usize {
        W * self.scale
    }

    pub fn h(&self) -> usize {
        H * self.scale
    }
}

/// The engine's numbers at one moment (`BigDish::measure`), in dish cells
/// and dish seconds so dishes of different scales compare.
#[derive(Clone, Debug, Default)]
pub struct BigMeasure {
    pub seconds: f32,
    /// Area over `tube`, in dish cells.
    pub tube_area: f32,
    /// Holes in the tube of at least one dish cell: the network's loops.
    pub loops: usize,
    /// Mean tube width, in dish cells: twice the area over the edge length.
    pub width: f32,
    /// Farthest tube from the home pile's centre, in dish cells.
    pub reach: f32,
    /// Piles the tube has reached, home included.
    pub reached: usize,
    /// Share of walkers standing where the trail is under `tube`.
    pub strays: f32,
    /// Living walkers, and their mean energy (0 with growth off).
    pub alive: usize,
    pub energy: f32,
}

#[derive(Clone)]
pub struct BigDish {
    pub p: BigParams,
    pub w: usize,
    pub h: usize,
    /// Ticks run.
    pub ticks: u64,
    /// Fine steps run: the step the keyed draws use.
    pub steps: u64,
    /// Fine grid: true where there is agar.
    pub agar: Vec<bool>,
    pub trail: Vec<f32>,
    scratch: Vec<f32>,
    /// The scent grid: today's (`scent_coarse`) or the fine one.
    pub sw: usize,
    pub sh: usize,
    pub scent_agar: Vec<bool>,
    pub scent: Vec<f32>,
    scent_scratch: Vec<f32>,
    /// x, y (fine cells), heading.
    pub walkers: Vec<(f32, f32, f32)>,
    occ: Vec<u16>,
    claim: Vec<u32>,
    /// Contagion (`sick_steps` > 0): each walker's counter, and the germ
    /// field on the fine grid. Both empty when contagion is off.
    pub sick: Vec<u16>,
    pub germ: Vec<f32>,
    /// One trail field per species (`species` > 1), else empty.
    pub species_trail: Vec<Vec<f32>>,
    /// Each walker's age in fine steps (`explore_steps` > 0), else empty.
    pub age: Vec<u16>,
    /// Each walker's species (`species` > 1), else empty. Starts k % n.
    pub spec: Vec<u8>,
    /// Each walker's kind, 0 or 1 (`mix_share` > 0), else empty.
    pub kind: Vec<u8>,
    /// Grass (`graze_weight` > 0, or growth on without crystals) a cell,
    /// else empty.
    pub grass: Vec<f32>,
    /// Growth (`upkeep` > 0): whether each slot holds a living walker and
    /// its energy; both empty with growth off, when every walker lives. A
    /// dead slot is left as it was and skipped by every rule.
    pub alive: Vec<bool>,
    pub energy: Vec<f32>,
    /// Births and starvings so far (growth on).
    pub births: u64,
    pub starved: u64,
    /// Each walker's life state (`LIFE_WALKER` ..= `LIFE_DEAD`) and timer
    /// (`BigParams::spore_rate` and below), else empty. Spores and the dead
    /// hold no cell (`occ`); the dormant keep theirs.
    pub life: Vec<u8>,
    pub life_t: Vec<u16>,
    /// The alarm field (`alarm_keep` > 0) on the fine grid, else empty.
    pub alarm: Vec<f32>,
    /// Pile centres in dish cells, and whether the tube has reached each.
    pub piles: Vec<(f32, f32)>,
    pub reached: Vec<bool>,
    /// Each pile's radius, dish cells, and its crystal's charge a full
    /// cell: `pile_r` and `crystal_charge` for every pile, unless a map
    /// (`with_map`, ROADMAP WM1) gives each crystal its own.
    pub pile_rs: Vec<f32>,
    pub pile_fill: Vec<f32>,
    /// Where the mould's blobs start and the squad lands, dish cells: pile
    /// 0's centre and none, unless a map places them. A map may give
    /// several starts (the Passes' two rivals); the walkers are shared
    /// among them, and pile 0 is the crystal nearest the first.
    pub homes: Vec<(f32, f32)>,
    pub landing: Option<(f32, f32)>,
    /// Crystals (`crystal_charge` > 0), else empty: charge a fine cell,
    /// which pile owns it (index + 1, 0 none), each pile's charge left and
    /// at the start, each walker's take this fine step, and all eaten.
    pub charge: Vec<f32>,
    pub pile_of: Vec<u16>,
    pub pile_charge: Vec<f32>,
    pub pile_full: Vec<f32>,
    /// The fine cells of every pile's crystal, pile by pile (a wall can
    /// take one out of its pile: its `pile_of` is then 0).
    pub pile_cells: Vec<u32>,
    pub took: Vec<f32>,
    pub eaten: f64,
    /// Terrain (`terrain_rock` or `terrain_wet` > 0), else empty: rock and
    /// wet a fine cell.
    pub rock: Vec<bool>,
    pub wet: Vec<bool>,
    /// Soil kind a fine cell (`BigParams::sand_keep`), else empty.
    pub soil: Vec<u8>,
    /// Fine steps of scorch left a fine cell (`scorch_disc`), else empty.
    pub scorch: Vec<u16>,
    /// Fine steps of gas left a walker (`gas`), else empty: while over 0 a
    /// dormant walker is not woken by scent, only by alarm.
    pub gassed: Vec<u16>,
    /// Walls the player built (`build_wall`), a fine cell; empty until the
    /// first. A wall is ground the mould cannot enter, the same channel as
    /// the rim and rock.
    pub wall: Vec<bool>,
    /// Each wall cell's health (`wall_hp` > 0), else empty; and cells broken.
    pub wall_health: Vec<f32>,
    pub broken: usize,
    /// Walker bites on walls so far (a measure, not state).
    pub wall_hits: u64,
    /// Scent cells inside a wall (`wall_scent` > 0), else empty.
    pub scent_wall: Vec<bool>,
    /// Holders a cell this step (`tether_hold`), else empty.
    holds_n: Vec<u16>,
    /// Threads for the field passes. Each cell is computed from the last
    /// field alone, so the result does not depend on it.
    pub threads: usize,
}

/// Agar inside a rectangle `margin` cells in from the edge (`rect`).
fn rect(w: usize, h: usize, margin: f32) -> Vec<bool> {
    let m = margin as usize;
    (0..w * h).map(|i| {
        let (x, y) = (i % w, i / w);
        x >= m && y >= m && x + m < w && y + m < h
    }).collect()
}

/// The terrain's noise (`place_terrain`'s channels: 0 rock, 1 wet), for
/// drawing the same land the dish places: the world look's elevation is
/// channel 0 turned over, so rock is the high ground.
pub fn terrain_noise(seed: u64, channel: u32, x: f32, y: f32, cell: f32) -> f32 {
    noise(seed, channel, x, y, cell)
}

/// Smooth value noise in [0, 1] at a dish-cell position: keyed lattice
/// values `cell` dish cells apart, blended with a smoothstep. Keyed on the
/// seed and the lattice point alone, so every scale draws the same map.
fn noise(seed: u64, channel: u32, x: f32, y: f32, cell: f32) -> f32 {
    let (gx, gy) = (x / cell, y / cell);
    let (ix, iy) = (gx.floor(), gy.floor());
    let (fx, fy) = (gx - ix, gy - iy);
    let (sx, sy) = (fx * fx * (3.0 - 2.0 * fx), fy * fy * (3.0 - 2.0 * fy));
    let v = |a: f32, b: f32| {
        let k = ((a as i32 + 64) as u32 & 0xFFFF) | (((b as i32 + 64) as u32 & 0x7FFF) << 16);
        unit(keyed(seed, channel as u64, DRAW_TERRAIN, k))
    };
    let top = v(ix, iy) + (v(ix + 1.0, iy) - v(ix, iy)) * sx;
    let bot = v(ix, iy + 1.0) + (v(ix + 1.0, iy + 1.0) - v(ix, iy + 1.0)) * sx;
    top + (bot - top) * sy
}

/// Agar inside an ellipse three dish cells in from the edge, as today's
/// `agar_ground`, at any size.
fn ellipse(w: usize, h: usize, margin: f32) -> Vec<bool> {
    let (cx, cy) = (w as f32 / 2.0, h as f32 / 2.0);
    let (rx, ry) = (cx - margin, cy - margin);
    let mut g = vec![false; w * h];
    for y in 0..h {
        for x in 0..w {
            let dx = (x as f32 + 0.5 - cx) / rx;
            let dy = (y as f32 + 0.5 - cy) / ry;
            g[y * w + x] = dx * dx + dy * dy <= 1.0;
        }
    }
    g
}

/// The 3x3 mean over agar neighbours, times `keep`, for rows `y0..y1` of
/// `to` (which holds just those rows). Today's `spread` without the lamp.
///
/// Branchless (2026-11-12, when the alarm became the plain mould's default
/// and this ran on the whole race world every fine step): each neighbour
/// adds its value times its agar mask (1 or 0) in the same order as the
/// first version (`spread_rows_reference` in the tests), so the result is
/// bit for bit the same: adding `0.0 * v` leaves a sum of fields that are
/// never negative unchanged, and the GPU's diffuse (which sums in this
/// order) stays an exact twin. Rows are sliced so the compiler can drop
/// the bounds checks.
fn spread_rows(agar: &[bool], from: &[f32], to: &mut [f32], w: usize, h: usize, keep: f32, y0: usize) {
    let rows = to.len() / w;
    for r in 0..rows {
        let y = y0 + r;
        let out = &mut to[r * w..(r + 1) * w];
        if y == 0 || y + 1 == h || w < 3 {
            out.fill(0.0);
            continue;
        }
        let (f0, a0) = (&from[(y - 1) * w..y * w], &agar[(y - 1) * w..y * w]);
        let (f1, a1) = (&from[y * w..(y + 1) * w], &agar[y * w..(y + 1) * w]);
        let (f2, a2) = (&from[(y + 1) * w..(y + 2) * w], &agar[(y + 1) * w..(y + 2) * w]);
        out[0] = 0.0;
        out[w - 1] = 0.0;
        for x in 1..w - 1 {
            let (mut sum, mut n) = (0.0f32, 0.0f32);
            for (f, a) in [(f0, a0), (f1, a1), (f2, a2)] {
                for k in [x - 1, x, x + 1] {
                    let m = a[k] as u8 as f32;
                    sum += f[k] * m;
                    n += m;
                }
            }
            out[x] = if a1[x] { sum / n * keep } else { 0.0 };
        }
    }
}

/// `spread_rows` over the whole field, split into bands across `threads`.
fn spread(agar: &[bool], from: &[f32], to: &mut [f32], w: usize, h: usize, keep: f32, threads: usize) {
    let threads = threads.clamp(1, h);
    if threads == 1 {
        spread_rows(agar, from, to, w, h, keep, 0);
        return;
    }
    let band = h.div_ceil(threads);
    std::thread::scope(|s| {
        for (k, chunk) in to.chunks_mut(band * w).enumerate() {
            s.spawn(move || spread_rows(agar, from, chunk, w, h, keep, k * band));
        }
    });
}

impl BigDish {
    pub fn new(p: BigParams) -> Self {
        Self::build(p, None)
    }

    /// A designed world (`map::Map`, ROADMAP WM1): the map's crystals in
    /// place of the keyed piles, each its own size and charge, the one
    /// nearest the mould's start first (pile 0, the mould's home); rock and
    /// wet from the map's ground in place of the noise; its walls built
    /// with their health; and the mould's blob on its start. `p` still
    /// switches each system on: crystals need `crystal_charge` > 0 (the
    /// map's charges replace it), and the rest of `p` is the world's as
    /// before. Soil kinds (G's high nibble) fill `soil`, which stays empty
    /// on a map without any (WM3's rules act through `sand_keep` and the
    /// rest; the GPU engine does not read soil yet).
    pub fn with_map(p: BigParams, m: &Map) -> Self {
        Self::build(p, Some(m))
    }

    fn build(p: BigParams, map: Option<&Map>) -> Self {
        assert!(
            p.species_sets.is_empty() || p.species_sets.len() >= p.species.max(1) as usize,
            "species_sets needs one set per species"
        );
        let (w, h) = (p.w(), p.h());
        let s = p.scale as f32;
        let ground = |w: usize, h: usize, m: f32| if p.rect { rect(w, h, m) } else { ellipse(w, h, m) };
        let agar = ground(w, h, 3.0 * s);
        let (sw, sh) = if p.scent_coarse { (W, H) } else { (w, h) };
        let scent_agar = if p.scent_coarse { ground(W, H, 3.0) } else { agar.clone() };
        let mut d = BigDish {
            w,
            h,
            ticks: 0,
            steps: 0,
            trail: vec![0.0; w * h],
            scratch: vec![0.0; w * h],
            agar,
            sw,
            sh,
            scent: vec![0.0; sw * sh],
            scent_scratch: vec![0.0; sw * sh],
            scent_agar,
            walkers: Vec::with_capacity(p.walkers),
            occ: vec![0; w * h],
            claim: vec![u32::MAX; w * h],
            sick: Vec::new(),
            germ: Vec::new(),
            species_trail: Vec::new(),
            age: Vec::new(),
            spec: Vec::new(),
            kind: Vec::new(),
            grass: Vec::new(),
            alive: Vec::new(),
            energy: Vec::new(),
            births: 0,
            starved: 0,
            life: Vec::new(),
            life_t: Vec::new(),
            alarm: Vec::new(),
            piles: Vec::new(),
            reached: Vec::new(),
            pile_rs: Vec::new(),
            pile_fill: Vec::new(),
            homes: Vec::new(),
            landing: None,
            charge: Vec::new(),
            pile_of: Vec::new(),
            pile_charge: Vec::new(),
            pile_full: Vec::new(),
            pile_cells: Vec::new(),
            took: Vec::new(),
            eaten: 0.0,
            rock: Vec::new(),
            wet: Vec::new(),
            soil: Vec::new(),
            scorch: Vec::new(),
            gassed: Vec::new(),
            wall: Vec::new(),
            wall_health: Vec::new(),
            broken: 0,
            wall_hits: 0,
            scent_wall: Vec::new(),
            holds_n: Vec::new(),
            threads: 1,
            p,
        };
        match map {
            Some(m) => {
                d.map_piles(m);
                d.map_terrain(m);
            }
            None => {
                d.place_piles();
                d.place_terrain();
            }
        }
        d.place_crystals();
        if let Some(m) = map {
            // Before the walkers, so none starts inside a wall.
            for wl in &m.items.walls {
                let (a, b) = (Map::dish(wl.a), Map::dish(wl.b));
                d.build_wall_hp(a.0, a.1, b.0, b.1, wl.thick / MAP_SCALE, wl.hp);
            }
        }
        d.place_walkers();
        if d.p.upkeep > 0.0 {
            // The living first, then empty slots at the centre (any cell
            // inside the dish would do: a dead slot is never read).
            let n = d.walkers.len();
            let cap = if d.p.capacity > 0 { d.p.capacity.max(n) } else { n };
            let (cx, cy) = (d.w as f32 / 2.0 + 0.5, d.h as f32 / 2.0 + 0.5);
            d.walkers.resize(cap, (cx, cy, 0.0));
            d.alive = (0..cap).map(|k| k < n).collect();
            d.energy = (0..cap).map(|k| if k < n { d.p.start_energy } else { 0.0 }).collect();
        }
        if d.p.mix_share > 0.0 {
            let (seed, share) = (d.p.seed, d.p.mix_share);
            d.kind = (0..d.walkers.len()).map(|k| u8::from(unit(keyed(seed, 0, DRAW_MIX, k as u32)) < share)).collect();
        }
        if d.p.explore_steps > 0 || d.p.species_sets.iter().any(|s| s.explore_steps > 0) {
            d.age = vec![0; d.walkers.len()];
        }
        // Growth eats grass unless the world feeds it crystals instead
        // (with crystals on, grass only if a `graze_weight` asks for it).
        let grazes = d.p.graze_weight != 0.0 || d.p.species_sets.iter().any(|s| s.graze_weight != 0.0);
        if grazes || (d.p.upkeep > 0.0 && d.p.crystal_charge <= 0.0) {
            d.grass = d.agar.iter().map(|&a| if a { 1.0 } else { 0.0 }).collect();
        }
        if d.p.spore_rate > 0.0 || d.p.spore_burst > 0.0 || d.p.sleep_rate > 0.0 {
            d.life = vec![LIFE_WALKER; d.walkers.len()];
            d.life_t = vec![0; d.walkers.len()];
        }
        if d.p.alarm_keep > 0.0 {
            d.alarm = vec![0.0; w * h];
        }
        if d.p.species > 1 {
            d.species_trail = vec![vec![0.0; w * h]; d.p.species as usize];
            let n = d.p.species as usize;
            d.spec = (0..d.walkers.len()).map(|k| d.species_of(k) as u8).collect();
            debug_assert!(d.spec.iter().all(|&s| (s as usize) < n));
        }
        if d.p.sick_steps > 0 {
            d.sick = vec![0; d.walkers.len()];
            d.germ = vec![0.0; w * h];
            let (cx, cy) = (W as f32 / 2.0, H as f32 / 2.0);
            d.infect(cx, cy, d.p.sick_seed_r);
        }
        d
    }

    /// Contagion: every walker within `r` dish cells of (`x`, `y`, dish
    /// cells) falls sick, immune or not. Does nothing with contagion off.
    pub fn infect(&mut self, x: f32, y: f32, r: f32) {
        if self.sick.is_empty() || r <= 0.0 {
            return;
        }
        let s = self.p.scale as f32;
        let c = self.p.sick_steps + self.p.immune_steps;
        for (k, &(wx, wy, _)) in self.walkers.iter().enumerate() {
            if self.lives(k) && (wx / s - x).hypot(wy / s - y) <= r {
                self.sick[k] = c;
            }
        }
    }

    /// Whether slot `k` holds a living walker (always, with growth off).
    pub fn lives(&self, k: usize) -> bool {
        self.alive.is_empty() || self.alive[k]
    }

    /// Walkers on each fine cell (the crowd count), for the Larder world's
    /// squad reads (`squad`, W2a).
    pub fn occupancy(&self) -> &[u16] {
        &self.occ
    }

    /// Kills walker `k` as `strike` does (it leaves its cell; with growth
    /// its slot is freed for births, and with life states, made here if
    /// growth is off, it is dead): the
    /// Larder world's shots (`squad::shots`, W2a). The caller checks that it
    /// lives and holds a cell.
    pub fn kill(&mut self, k: usize) {
        if self.alive.is_empty() && self.life.is_empty() {
            self.life = vec![LIFE_WALKER; self.walkers.len()];
            self.life_t = vec![0; self.walkers.len()];
        }
        let (x, y, _) = self.walkers[k];
        let i = self.cell(x, y).expect("walker inside the dish");
        self.occ[i] -= 1;
        if !self.alive.is_empty() {
            self.alive[k] = false;
        }
        if !self.life.is_empty() {
            self.life[k] = LIFE_DEAD;
        }
    }

    fn cell(&self, x: f32, y: f32) -> Option<usize> {
        if x < 0.0 || y < 0.0 {
            return None;
        }
        let (cx, cy) = (x as usize, y as usize);
        (cx < self.w && cy < self.h).then(|| cy * self.w + cx)
    }

    /// Home on the left, the rest spread by keyed draws, at least 28 dish
    /// cells apart and clear of the rim: the same layout at every scale for
    /// one seed, so arms pair.
    fn place_piles(&mut self) {
        let seed = self.p.seed;
        let r = self.p.pile_r;
        let (cx, cy, rx, ry) = (W as f32 / 2.0, H as f32 / 2.0, W as f32 / 2.0 - 3.0, H as f32 / 2.0 - 3.0);
        let square = self.p.rect;
        let inside = |x: f32, y: f32| {
            let (dx, dy) = ((x - cx) / (rx - r - 4.0), (y - cy) / (ry - r - 4.0));
            if square { dx.abs() <= 1.0 && dy.abs() <= 1.0 } else { dx * dx + dy * dy <= 1.0 }
        };
        self.piles.push((W as f32 * 0.22, H as f32 * 0.5));
        let mut k = 0u32;
        while self.piles.len() < self.p.piles && k < 100_000 {
            let x = unit(keyed(seed, 0, DRAW_PILES, 2 * k)) * W as f32;
            let y = unit(keyed(seed, 0, DRAW_PILES, 2 * k + 1)) * H as f32;
            k += 1;
            if inside(x, y) && self.piles.iter().all(|&(px, py)| (px - x).hypot(py - y) >= 28.0) {
                self.piles.push((x, y));
            }
        }
        self.reached = vec![false; self.piles.len()];
        self.reached[0] = true;
        self.pile_rs = vec![r; self.piles.len()];
        self.pile_fill = vec![self.p.crystal_charge; self.piles.len()];
        self.homes = vec![self.piles[0]];
    }

    /// A map's crystals as the piles, the one nearest the mould's start
    /// first, the rest in the map's order; the start and landing from the
    /// map (the start on the first crystal without one).
    fn map_piles(&mut self, m: &Map) {
        let mut cs = m.items.crystals.clone();
        assert!(!cs.is_empty(), "a map needs a crystal");
        let start = m.items.moulds.first().copied().unwrap_or((cs[0].x, cs[0].y));
        let d2 = |c: &map::Crystal| (c.x - start.0).hypot(c.y - start.1);
        let near = (0..cs.len()).min_by(|&a, &b| d2(&cs[a]).total_cmp(&d2(&cs[b]))).unwrap();
        let first = cs.remove(near);
        cs.insert(0, first);
        self.piles = cs.iter().map(|c| Map::dish((c.x, c.y))).collect();
        self.pile_rs = cs.iter().map(|c| c.r / MAP_SCALE).collect();
        self.pile_fill = cs.iter().map(|c| if self.p.crystal_charge > 0.0 { c.charge } else { 0.0 }).collect();
        self.reached = vec![false; self.piles.len()];
        self.reached[0] = true;
        self.homes = if m.items.moulds.is_empty() { vec![Map::dish(start)] } else { m.items.moulds.iter().map(|&p| Map::dish(p)).collect() };
        self.landing = m.items.landing.map(Map::dish);
    }

    /// A map's ground: rock leaves the agar on the fine and scent grids,
    /// wet lies on the agar, and the soil kind is G's high nibble (the
    /// map's cell at each cell's centre, `Map::ground_fine`). The noise's
    /// `terrain_rock` and `terrain_wet` are not read.
    fn map_terrain(&mut self, m: &Map) {
        let (w, h, s) = (self.w, self.h, self.p.scale);
        self.rock = vec![false; w * h];
        self.wet = vec![false; w * h];
        for i in 0..w * h {
            let g = m.ground_fine(s, i % w, i / w);
            if g >> 4 != 0 {
                if self.soil.is_empty() {
                    self.soil = vec![0; w * h];
                }
                self.soil[i] = g >> 4;
            }
            if g & G_ROCK != 0 {
                self.rock[i] = true;
                self.agar[i] = false;
            } else if g & G_WET != 0 && self.agar[i] {
                self.wet[i] = true;
            }
        }
        let k = if self.p.scent_coarse { 1 } else { s };
        for i in 0..self.sw * self.sh {
            if m.ground_fine(k, i % self.sw, i / self.sw) & G_ROCK != 0 {
                self.scent_agar[i] = false;
            }
        }
    }

    /// Terrain (`terrain_rock`, `terrain_wet`): rock leaves the agar, on
    /// the fine grid and the scent grid alike; pile discs stay clear.
    fn place_terrain(&mut self) {
        let (rock_t, wet_t) = (self.p.terrain_rock, self.p.terrain_wet);
        if rock_t <= 0.0 && wet_t <= 0.0 {
            return;
        }
        let (seed, cell, s) = (self.p.seed, self.p.terrain_blob.max(1.0), self.p.scale as f32);
        let piles: Vec<(f32, f32, f32)> = self.piles.iter().zip(&self.pile_rs).map(|(&(x, y), &r)| (x, y, r + 4.0)).collect();
        let is_rock = |x: f32, y: f32| {
            rock_t > 0.0 && noise(seed, 0, x, y, cell) < rock_t && piles.iter().all(|&(px, py, clear)| (px - x).hypot(py - y) > clear)
        };
        let (w, h) = (self.w, self.h);
        self.rock = vec![false; w * h];
        self.wet = vec![false; w * h];
        for i in 0..w * h {
            let (x, y) = (((i % w) as f32 + 0.5) / s, ((i / w) as f32 + 0.5) / s);
            if is_rock(x, y) {
                self.rock[i] = true;
                self.agar[i] = false;
            } else if wet_t > 0.0 && self.agar[i] && noise(seed, 1, x, y, cell) < wet_t {
                self.wet[i] = true;
            }
        }
        let k = if self.p.scent_coarse { 1.0 } else { s };
        for i in 0..self.sw * self.sh {
            let (x, y) = (((i % self.sw) as f32 + 0.5) / k, ((i / self.sw) as f32 + 0.5) / k);
            if is_rock(x, y) {
                self.scent_agar[i] = false;
            }
        }
    }

    /// A wall from (`ax`, `ay`) to (`bx`, `by`), dish cells, `width` dish
    /// cells thick: those cells leave the agar (fine and scent grids) and
    /// lose their trail, scent and charge. The Larder World's first player
    /// tool (roadmap reviews, 11-11: a salt wall across the leader's road
    /// turned 6 of 12 lost skirmish games and beat a random wall 16/16).
    /// A walker standing where the wall goes is pushed to the nearest free
    /// agar cell, searching outward ring by ring, or stays if none is free
    /// within 16 dish cells (a packed blob has few free cells near). Returns the fine cells walled.
    pub fn build_wall(&mut self, ax: f32, ay: f32, bx: f32, by: f32, width: f32) -> usize {
        self.build_wall_hp(ax, ay, bx, by, width, self.p.wall_hp)
    }

    /// `build_wall` with its cells' health `hp` (0: it never breaks), as a
    /// map's walls are built.
    pub fn build_wall_hp(&mut self, ax: f32, ay: f32, bx: f32, by: f32, width: f32, hp: f32) -> usize {
        let s = self.p.scale as f32;
        let (w, h) = (self.w, self.h);
        if self.wall.is_empty() {
            self.wall = vec![false; w * h];
        }
        let (ax, ay, bx, by, r) = (ax * s, ay * s, bx * s, by * s, (width * s / 2.0).max(0.5));
        let (dx, dy) = (bx - ax, by - ay);
        let len2 = (dx * dx + dy * dy).max(1e-6);
        let near = |x: f32, y: f32| {
            let t = (((x - ax) * dx + (y - ay) * dy) / len2).clamp(0.0, 1.0);
            (x - ax - t * dx).hypot(y - ay - t * dy) <= r
        };
        let (x0, x1) = ((ax.min(bx) - r).floor().max(0.0) as usize, ((ax.max(bx) + r).ceil() as usize).min(w - 1));
        let (y0, y1) = ((ay.min(by) - r).floor().max(0.0) as usize, ((ay.max(by) + r).ceil() as usize).min(h - 1));
        let mut cells = Vec::new();
        for y in y0..=y1 {
            for x in x0..=x1 {
                let i = y * w + x;
                if self.agar[i] && near(x as f32 + 0.5, y as f32 + 0.5) {
                    cells.push(i);
                }
            }
        }
        if hp > 0.0 && self.wall_health.is_empty() {
            self.wall_health = vec![0.0; w * h];
        }
        for &i in &cells {
            self.agar[i] = false;
            self.wall[i] = true;
            if !self.wall_health.is_empty() {
                self.wall_health[i] = hp;
            }
            self.trail[i] = 0.0;
            for f in self.species_trail.iter_mut() {
                f[i] = 0.0;
            }
            if !self.charge.is_empty() && self.pile_of[i] > 0 {
                self.pile_charge[self.pile_of[i] as usize - 1] -= self.charge[i];
                self.pile_full[self.pile_of[i] as usize - 1] -= self.pile_fill[self.pile_of[i] as usize - 1];
                self.charge[i] = 0.0;
                self.pile_of[i] = 0;
            }
        }
        // The scent grid: a scent cell is walled when its centre is.
        let k = if self.p.scent_coarse { 1.0 } else { s };
        for i in 0..self.sw * self.sh {
            let (x, y) = (((i % self.sw) as f32 + 0.5) / k * s, ((i / self.sw) as f32 + 0.5) / k * s);
            if self.scent_agar[i] && near(x, y) {
                if self.p.wall_scent > 0.0 {
                    if self.scent_wall.is_empty() {
                        self.scent_wall = vec![false; self.sw * self.sh];
                    }
                    self.scent_wall[i] = true;
                } else {
                    self.scent_agar[i] = false;
                }
                self.scent[i] = 0.0;
            }
        }
        // Walkers inside move out, in walker order (a CPU rule; the GPU
        // port does this with a claim pass).
        let reach = (16.0 * s).ceil() as i32;
        for k in 0..self.walkers.len() {
            // An empty slot (growth) holds a stale place and no count.
            if !self.lives(k) {
                continue;
            }
            let (x, y, hd) = self.walkers[k];
            let i = self.cell(x, y).expect("walker inside the dish");
            // Spores and the dead hold no cell (`BigDish::life`): a spore
            // over the wall flies on and cannot land on it.
            if self.agar[i] || matches!(self.life.get(k), Some(&(LIFE_SPORE | LIFE_DEAD))) {
                continue;
            }
            let (cx, cy) = (x as i32, y as i32);
            'ring: for d in 1..=reach {
                for oy in -d..=d {
                    for ox in -d..=d {
                        if ox.abs() != d && oy.abs() != d {
                            continue;
                        }
                        let (nx, ny) = (cx + ox, cy + oy);
                        if nx < 0 || ny < 0 || nx as usize >= w || ny as usize >= h {
                            continue;
                        }
                        let j = ny as usize * w + nx as usize;
                        if self.agar[j] && (self.p.crowd == 0 || self.occ[j] < self.p.crowd as u16) {
                            self.occ[i] -= 1;
                            self.occ[j] += 1;
                            self.walkers[k] = (nx as f32 + 0.5, ny as f32 + 0.5, hd);
                            break 'ring;
                        }
                    }
                }
            }
        }
        cells.len()
    }

    /// Blocked walkers bite the wall cell they tried to enter (`wall_hp`).
    /// Damage is summed first and applied after, so walker order does not
    /// matter; a cell at 0 becomes rubble: ground again, wet, scent open.
    fn bite_walls(&mut self, blocked: &[(u32, f32, f32)]) {
        let bite = self.p.wall_bite;
        let mut hit: Vec<usize> = Vec::new();
        for &(k, h, st) in blocked {
            let (x, y, _) = self.walkers[k as usize];
            if let Some(j) = self.cell(x + h.cos() * st, y + h.sin() * st) {
                if self.wall[j] {
                    hit.push(j);
                }
            }
        }
        self.wall_hits += hit.len() as u64;
        for &j in &hit {
            self.wall_health[j] -= bite;
        }
        let s = self.p.scale as f32;
        let k = if self.p.scent_coarse { 1.0 / s } else { 1.0 };
        for &j in &hit {
            if self.wall[j] && self.wall_health[j] <= 0.0 {
                self.wall[j] = false;
                self.wall_health[j] = 0.0;
                self.agar[j] = true;
                if self.wet.is_empty() {
                    self.wet = vec![false; self.w * self.h];
                    if self.rock.is_empty() {
                        self.rock = vec![false; self.w * self.h];
                    }
                }
                self.wet[j] = true;
                let (fx, fy) = ((j % self.w) as f32 * k, (j / self.w) as f32 * k);
                let si = fy as usize * self.sw + fx as usize;
                if si < self.scent_agar.len() {
                    self.scent_agar[si] = true;
                    if !self.scent_wall.is_empty() {
                        self.scent_wall[si] = false;
                    }
                }
                self.broken += 1;
            }
        }
    }

    /// Crystals (`crystal_charge`): every fine agar cell in a pile's disc
    /// holds the full charge.
    fn place_crystals(&mut self) {
        if self.p.crystal_charge <= 0.0 {
            return;
        }
        let s = self.p.scale as f32;
        self.charge = vec![0.0; self.w * self.h];
        self.pile_of = vec![0; self.w * self.h];
        self.pile_full = vec![0.0; self.piles.len()];
        self.pile_cells.clear();
        for pi in 0..self.piles.len() {
            let (r, full) = (self.pile_rs[pi] * s, self.pile_fill[pi]);
            let n = r.ceil() as i32;
            let (cx, cy) = (self.piles[pi].0 * s, self.piles[pi].1 * s);
            for dy in -n..=n {
                for dx in -n..=n {
                    if !self.in_pile(dx as f32, dy as f32, r) {
                        continue;
                    }
                    if let Some(i) = self.cell(cx + dx as f32, cy + dy as f32) {
                        if self.agar[i] && self.pile_of[i] == 0 {
                            self.charge[i] = full;
                            self.pile_of[i] = pi as u16 + 1;
                            self.pile_full[pi] += full;
                            self.pile_cells.push(i as u32);
                        }
                    }
                }
            }
        }
        self.pile_charge = self.pile_full.clone();
    }

    /// Whether the offset (`dx`, `dy`) from a pile's centre is inside it,
    /// for a pile of radius `r` on whatever grid the caller uses. With
    /// crystals on, a pile is a pointy-top hexagon of circumradius `r`
    /// (the owner, 11-11 16:48Z: "The crystals should have hexagonal
    /// outline, not circle"; the picture is the data, so the crystal's
    /// cells, its scent and where it counts as reached are all the
    /// hexagon, about 0.83 of the disc's area). With crystals off it is
    /// the disc it always was, so older dishes do not change.
    pub fn in_pile(&self, dx: f32, dy: f32, r: f32) -> bool {
        if self.p.crystal_charge > 0.0 {
            let ax = dx.abs();
            ax <= r * 0.866_025_4 && dy.abs() <= r - ax * 0.577_350_3
        } else {
            dx * dx + dy * dy <= r * r
        }
    }

    /// The share of a pile's crystal left (1 with crystals off).
    pub fn crystal_left(&self, pi: usize) -> f32 {
        if self.pile_full.is_empty() || self.pile_full[pi] <= 0.0 {
            return 1.0;
        }
        (self.pile_charge[pi] / self.pile_full[pi]).max(0.0)
    }

    /// Crystal charge left over all piles, as a share of the start.
    pub fn crystals_left(&self) -> f32 {
        let full: f32 = self.pile_full.iter().sum();
        if full <= 0.0 { 1.0 } else { self.pile_charge.iter().sum::<f32>().max(0.0) / full }
    }

    /// Eating (`crystal_charge`): the n walkers on a cell take equal shares.
    fn eat_crystals(&mut self) {
        if self.charge.is_empty() {
            return;
        }
        let eat = self.p.crystal_eat;
        if self.took.len() != self.walkers.len() {
            self.took = vec![0.0; self.walkers.len()];
        }
        // Each walker's share, from the charge as the pass began.
        // Only ordinary walkers eat (`BigDish::life`): spores and the dead
        // hold no cell, and the dormant neither sense nor move. A dormant
        // walker still counts in `occ`, so it shrinks the share of an
        // ordinary one on its cell; with one walker a cell that never
        // arises. An empty slot (growth) takes nothing either.
        for k in 0..self.walkers.len() {
            if !self.active(k) {
                self.took[k] = 0.0;
                continue;
            }
            let (x, y, _) = self.walkers[k];
            let i = self.cell(x, y).expect("walker inside the dish");
            let n = self.occ[i].max(1) as f32;
            self.took[k] = eat.min(self.charge[i] / n);
        }
        for k in 0..self.walkers.len() {
            let t = self.took[k];
            if t <= 0.0 {
                continue;
            }
            let (x, y, _) = self.walkers[k];
            let i = self.cell(x, y).expect("walker inside the dish");
            self.charge[i] = (self.charge[i] - t).max(0.0);
            self.eaten += t as f64;
        }
        // Pile totals are summed afresh from the cells, in f64. (They were
        // a running f32 total until 11-11: at charge 512 a pile holds about
        // 180,000, a 0.005 bite is below that total's f32 step of 0.016,
        // so the totals stopped moving, crystal_left read 1.000 and the
        // piles called at full scent while the mould ate them.)
        let np = self.piles.len();
        let sums = |d: &Self| {
            let (mut sum, mut cells) = (vec![0.0f64; np], vec![0u32; np]);
            for &i in &d.pile_cells {
                let pi = d.pile_of[i as usize] as usize;
                if pi > 0 {
                    sum[pi - 1] += d.charge[i as usize] as f64;
                    cells[pi - 1] += 1;
                }
            }
            (sum, cells)
        };
        let f = self.p.crystal_flow;
        if f > 0.0 {
            let (sum, cells) = sums(self);
            for &i in &self.pile_cells {
                let pi = self.pile_of[i as usize] as usize;
                if pi > 0 {
                    let mean = (sum[pi - 1] / cells[pi - 1] as f64) as f32;
                    let c = &mut self.charge[i as usize];
                    *c += f * (mean - *c);
                }
            }
        }
        let g = self.p.crystal_regrow;
        if g > 0.0 {
            for &i in &self.pile_cells {
                let i = i as usize;
                if self.pile_of[i] > 0 {
                    let full = self.pile_fill[self.pile_of[i] as usize - 1];
                    if self.charge[i] < full {
                        self.charge[i] += g * (full - self.charge[i]);
                    }
                }
            }
        }
        let (sum, _) = sums(self);
        for (pc, s) in self.pile_charge.iter_mut().zip(sum) {
            *pc = s as f32;
        }
    }

    fn place_walkers(&mut self) {
        let (seed, n, s) = (self.p.seed, self.p.walkers, self.p.scale as f32);
        let homes = self.homes.clone();
        let m = homes.len().max(1);
        let mut k = 0u32;
        for (j, &(x, y)) in homes.iter().enumerate() {
            // Each start an equal share, the first the remainder too.
            let share = n / m + if j == 0 { n % m } else { 0 };
            let target = self.walkers.len() + share;
            // A blob as dense as today's inoculum: about 0.9 of the cells in it.
            let r = (share as f32 / (0.9 * std::f32::consts::PI)).sqrt();
            let (hx, hy, mut r) = if self.p.blob_r > 0.0 { (self.w as f32 / 2.0, self.h as f32 / 2.0, self.p.blob_r * s) } else { (x * s, y * s, r) };
            let mut budget = k + 50 * share as u32 + 1000;
            // A blob that cannot fit (a map's start near the rim or rock:
            // the Fork's, 12 dish cells from the edge, took 3,752 of 5,000)
            // widens by a quarter and goes on, up to the world's size. The
            // first round is the one every dish had, so a blob that fits
            // draws the same.
            loop {
                self.place_round(seed, target, hx, hy, r, &mut k, budget);
                if self.walkers.len() >= target || !self.p.blob || r > self.w.max(self.h) as f32 {
                    break;
                }
                r *= 1.25;
                budget += 50 * share as u32 + 1000;
            }
        }
    }

    #[allow(clippy::too_many_arguments)]
    fn place_round(&mut self, seed: u64, n: usize, hx: f32, hy: f32, r: f32, k: &mut u32, budget: u32) {
        while self.walkers.len() < n && *k < budget {
            let k0 = *k;
            let (a, b, c) = (
                unit(keyed(seed, 1, DRAW_START, 3 * k0)),
                unit(keyed(seed, 1, DRAW_START, 3 * k0 + 1)),
                unit(keyed(seed, 1, DRAW_START, 3 * k0 + 2)),
            );
            *k += 1;
            let k = *k;
            let hazy = self.p.haze > 0.0 && unit(keyed(seed, 1, DRAW_HAZE, k)) < self.p.haze;
            let (x, y) = if self.p.blob && !hazy {
                let (ang, dist) = (a * TAU, b.sqrt() * r);
                (hx + ang.cos() * dist, hy + ang.sin() * dist)
            } else if self.p.species_zones && self.p.species > 1 {
                // Walker number `len` (the next pushed) is species_of(len);
                // bands are equal whatever the shares (see `species_share`).
                let n = self.p.species as f32;
                let sp = self.species_of(self.walkers.len()) as f32;
                ((sp + a) / n * self.w as f32, b * self.h as f32)
            } else {
                (a * self.w as f32, b * self.h as f32)
            };
            let Some(i) = self.cell(x, y) else { continue };
            // The crowd limit of the species this walker will be.
            let crowd = if self.p.species_sets.is_empty() { self.p.crowd } else { self.p.species_sets[self.species_of(self.walkers.len())].crowd };
            if !self.agar[i] || (crowd > 0 && self.occ[i] >= crowd as u16) {
                continue;
            }
            self.occ[i] += 1;
            self.walkers.push((x, y, c * TAU));
        }
    }

    /// Scent under a fine position, from whichever grid holds it.
    fn scent_at(&self, x: f32, y: f32) -> f32 {
        let (x, y) = if self.p.scent_coarse {
            (x / self.p.scale as f32, y / self.p.scale as f32)
        } else {
            (x, y)
        };
        if x < 0.0 || y < 0.0 || x >= self.sw as f32 || y >= self.sh as f32 {
            return 0.0;
        }
        self.scent[y as usize * self.sw + x as usize]
    }

    /// Trail at `so` and scent at `sso` along heading `a`: off the agar
    /// (by the trail sensor) reads as a wall.
    /// `sw` weighs the scent (1, or more with `hunger_scent`); `body`, the
    /// square sums of walkers as the step began (`body_pull`), with `bw`
    /// their weight.
    #[allow(clippy::too_many_arguments)]
    fn sense(&self, (x, y): (f32, f32), a: f32, so: f32, sso: f32, sp: usize, gw: f32, sw: f32, body: &[u32], bw: f32) -> f32 {
        let (c, s) = (a.cos(), a.sin());
        match self.cell(x + c * so, y + s * so) {
            Some(i) if self.agar[i] => {
                let g = if self.grass.is_empty() { 0.0 } else { gw * self.grass[i] };
                let al = if self.alarm.is_empty() { 0.0 } else { self.p.alarm_weight * self.alarm[i] };
                let b = if body.is_empty() { 0.0 } else { bw * body[i] as f32 };
                let sc = self.scent_at(x + c * sso, y + s * sso);
                // sw 1 keeps the sum's order, so older runs keep their hashes.
                let sc = if sw == 1.0 { sc } else { sw * sc };
                self.trail_seen(i, sp) + sc + g + al + b
            }
            _ => -1.0e6,
        }
    }

    /// The trail a walker of species `sp` reads at cell `i`: one cell or a
    /// square (`sensor_r`), own trail weighed against others' (`species`).
    fn trail_seen(&self, i: usize, sp: usize) -> f32 {
        let at = |j: usize| {
            if self.species_trail.is_empty() {
                self.trail[j]
            } else {
                let a = self.p.species_avoid;
                let n = self.species_trail.len();
                if self.p.species_cycle && n >= 3 {
                    let (own, next, prev) = (self.species_trail[sp][j], self.species_trail[(sp + 1) % n][j], self.species_trail[(sp + n - 1) % n][j]);
                    own + self.p.species_next * next + self.p.species_prev * prev - a * (self.trail[j] - own - next - prev)
                } else {
                    (1.0 + a) * self.species_trail[sp][j] - a * self.trail[j]
                }
            }
        };
        let r = self.p.sensor_r as usize;
        if r == 0 {
            return at(i);
        }
        let (x, y) = (i % self.w, i / self.w);
        let mut sum = 0.0;
        for yy in y.saturating_sub(r)..(y + r + 1).min(self.h) {
            for xx in x.saturating_sub(r)..(x + r + 1).min(self.w) {
                let j = yy * self.w + xx;
                if self.agar[j] {
                    sum += at(j);
                }
            }
        }
        sum
    }

    /// Lay `amount` of trail on cell `j` for walker `k` (and its species).
    fn lay(&mut self, k: usize, j: usize, amount: f32) {
        let amount = if !self.kind.is_empty() && self.kind[k] == 1 { amount * self.p.mix_deposit } else { amount };
        self.trail[j] += amount;
        if !self.species_trail.is_empty() {
            let sp = self.spec[k] as usize;
            let a = if sp == 0 { amount * self.p.species_boost } else { amount };
            self.trail[j] += a - amount;
            self.species_trail[sp][j] += a;
        }
    }

    /// The settings species `sp` uses: its own (`species_sets`) or the
    /// global fields.
    fn set_of(&self, sp: usize) -> SpeciesSet {
        if self.p.species_sets.is_empty() {
            SpeciesSet::from_params(&self.p)
        } else {
            self.p.species_sets[sp]
        }
    }

    /// Walker `k`'s species (0 with one species).
    fn sp_of(&self, k: usize) -> usize {
        if self.spec.is_empty() {
            0
        } else {
            self.spec[k] as usize
        }
    }

    /// One fine step of the walkers: sense, turn, claim, move, deposit.
    fn move_walkers(&mut self) {
        let p = self.p.clone();
        let nsets = if p.species_sets.is_empty() { 1 } else { p.species_sets.len() };
        let sets: Vec<SpeciesSet> = (0..nsets).map(|s| self.set_of(s)).collect();
        let per = sets.len() > 1;
        // Any species with a crowd limit makes claims this step.
        let any_crowd = sets.iter().any(|s| s.crowd > 0);
        let respawn = p.respawn;
        let (ss, cap, contagion) = (p.sense_scale, p.deposit_cap, !self.sick.is_empty());
        let (sick_at, catch_c) = (p.immune_steps, p.sick_steps + p.immune_steps);
        let (seed, t) = (p.seed, self.steps);
        // (walker, from, to, x, y, heading); from == to for a move within a cell.
        let mut moves: Vec<(u32, u32, u32, f32, f32, f32, f32)> = Vec::with_capacity(self.walkers.len());
        // (walker, heading, step) of each walker that could not move.
        let mut blocked: Vec<(u32, f32, f32)> = Vec::new();
        let look = t.is_multiple_of(p.sense_every.max(1) as u64);
        let nsp = self.species_trail.len().max(1);
        // Life cycle (`spore_rate` and below): walkers that were spores or
        // dormant as the step began, and ordinary walkers that fruit or fall
        // dormant this step (applied after the moves, so every claim sees the
        // cells as the pass began).
        let life_on = !self.life.is_empty();
        let (mut flying, mut sleeping, mut fruit, mut doze) = (Vec::new(), Vec::new(), Vec::new(), Vec::new());
        // Body pull: walkers in each cell's square, as the step began.
        let body = if p.body_pull > 0.0 && look { self.body_counts(p.body_r as usize) } else { Vec::new() };
        // Tether's holders (`tether_hold`): who stays this step, and how
        // many stay on each cell.
        self.holds_n.clear();
        let hold_on = p.tether > 0 && p.tether_hold > 0.0;
        if hold_on {
            self.holds_n.resize(self.w * self.h, 0);
            for k in 0..self.walkers.len() {
                if self.active(k) && unit(keyed(seed, t, DRAW_HOLD, k as u32)) < p.tether_hold {
                    let (x, y, _) = self.walkers[k];
                    let i = self.cell(x, y).expect("walker inside the dish");
                    self.holds_n[i] += 1;
                }
            }
        }
        for k in 0..self.walkers.len() {
            if !self.lives(k) {
                continue;
            }
            let sp = if nsp > 1 { self.spec[k] as usize } else { 0 };
            let set = sets[if per { sp } else { 0 }];
            let crowd = set.crowd;
            let (x, y, mut h) = self.walkers[k];
            let here = self.cell(x, y).expect("walker inside the dish");
            if life_on {
                match self.life[k] {
                    LIFE_SPORE => {
                        flying.push(k);
                        continue;
                    }
                    LIFE_DORMANT => {
                        sleeping.push(k);
                        continue;
                    }
                    LIFE_DEAD => continue,
                    _ => {}
                }
                if self.life_t[k] == 0 {
                    let alarm = if self.alarm.is_empty() { 0.0 } else { self.alarm[here] };
                    let mut c = if self.scent_at(x, y) > p.spore_scent { p.spore_rate } else { 0.0 };
                    if p.spore_burst > 0.0 && alarm > p.spore_alarm {
                        c += p.spore_burst;
                    }
                    if c > 0.0 && unit(keyed(seed, t, DRAW_FRUIT, k as u32)) < c {
                        fruit.push(k);
                        continue;
                    }
                    if p.sleep_rate > 0.0 && self.scent_at(x, y) < p.wake_scent && alarm < p.wake_alarm && unit(keyed(seed, t, DRAW_SLEEP, k as u32)) < p.sleep_rate {
                        doze.push(k);
                        continue;
                    }
                }
            }
            if hold_on && unit(keyed(seed, t, DRAW_HOLD, k as u32)) < p.tether_hold {
                continue;
            }
            // Trail-dependent settings (`sense_scale`), read where it stands.
            let (mut so, mut sa, mut ra, mut st) = (set.sensor_offset, set.sensor_angle, set.rotate_angle, set.step);
            let mut sso = if p.scent_offset > 0.0 { p.scent_offset } else { set.sensor_offset };
            if p.hunger_reach > 0.0 {
                let g = self.hunger(k);
                if g >= p.hunt_at {
                    sso *= 1.0 + p.hunger_reach * g;
                }
            }
            if !self.kind.is_empty() && self.kind[k] == 1 {
                so *= p.mix_reach;
                sa *= p.mix_angle;
                ra *= p.mix_turn;
                st *= p.mix_step;
            }
            if p.size_spread > 0.0 {
                let u = 2.0 * unit(keyed(seed, 0, DRAW_SIZE, k as u32)) - 1.0;
                let sc = (p.size_spread * u).exp2();
                so *= sc;
                st *= sc;
            }
            let explorer = self.explores(k);
            if !self.wet.is_empty() && self.wet[here] {
                st *= p.wet_step;
            }
            if ss > 0.0 {
                let v = (self.trail[here] * ss).min(1.0);
                let m = |b: f32, [amp, e]: [f32; 2]| b + amp * v.powf(e);
                so = m(so, p.offset_mod);
                sa = m(sa, p.angle_mod);
                ra = m(ra, p.turn_mod);
                st = m(st, p.step_mod).max(0.0);
            }
            if contagion {
                if self.sick[k] == 0 && self.germ[here] > p.sick_catch {
                    self.sick[k] = catch_c;
                }
                if self.sick[k] > sick_at {
                    st *= p.sick_speed;
                    so *= p.sick_reach;
                    ra *= p.sick_turn;
                }
            }
            if look {
                let sw = if p.hunger_scent > 0.0 && !self.energy.is_empty() {
                    1.0 + p.hunger_scent * self.hunger(k)
                } else {
                    1.0
                };
                let bw = if body.is_empty() {
                    0.0
                } else if p.body_lone > 0.0 {
                    let near = body[here].saturating_sub(1) as f32;
                    p.body_pull * (1.0 - near / p.body_lone).max(0.0)
                } else {
                    p.body_pull
                };
                let (mut f, mut fl, mut fr) = (
                    self.sense((x, y), h, so, sso, sp, set.graze_weight, sw, &body, bw),
                    self.sense((x, y), h - sa, so, sso, sp, set.graze_weight, sw, &body, bw),
                    self.sense((x, y), h + sa, so, sso, sp, set.graze_weight, sw, &body, bw),
                );
                if explorer {
                    // Toward the lowest reading; a wall stays a wall.
                    for v in [&mut f, &mut fl, &mut fr] {
                        if *v > -1.0e5 {
                            *v = -*v;
                        }
                    }
                }
                if p.wander > 0.0 && unit(keyed(seed, t, DRAW_WANDER, k as u32 | 0x8000_0000)) < p.wander {
                    h = unit(keyed(seed, t, DRAW_WANDER, k as u32 | 0x4000_0000)) * TAU;
                } else if p.sense_floor > 0.0 && f.max(fl).max(fr) < p.sense_floor {
                    h += if keyed(seed, t, DRAW_WANDER, k as u32) & 1 == 0 { ra } else { -ra };
                } else if p.chance_exp > 0.0 {
                    let left = keyed(seed, t, DRAW_CHANCE, k as u32) & 1 == 0;
                    let (fe, ge) = (f.max(0.0).powf(p.chance_exp), (if left { fl } else { fr }).max(0.0).powf(p.chance_exp));
                    let turn = if fe + ge > 0.0 { ge / (fe + ge) } else { 0.5 };
                    if unit(keyed(seed, t, DRAW_CHANCE, k as u32 | 0x8000_0000)) < turn {
                        h += if left { -ra } else { ra };
                    }
                } else if f > fl && f > fr {
                } else if f < fl && f < fr {
                    h += if keyed(seed, t, DRAW_TURN, k as u32) & 1 == 0 { ra } else { -ra };
                } else if fl < fr {
                    h += ra;
                } else if fr < fl {
                    h -= ra;
                }
            }
            let (nx, ny) = (x + h.cos() * st, y + h.sin() * st);
            match self.cell(nx, ny) {
                Some(there) if there == here || (self.agar[there] && (crowd == 0 || self.occ[there] < crowd as u16) && self.tethered(k, here, there)) => {
                    if there != here && crowd > 0 {
                        let key = keyed(seed, t, CLAIM_MOVE, k as u32);
                        self.claim[there] = self.claim[there].min(key);
                    }
                    moves.push((k as u32, here as u32, there as u32, nx, ny, h, st));
                }
                _ => blocked.push((k as u32, h, st)),
            }
        }
        for &(k, from, to, nx, ny, h, st) in &moves {
            let (i, j) = (from as usize, to as usize);
            let set = sets[if per { self.sp_of(k as usize) } else { 0 }];
            let (crowd, dep) = (set.crowd, set.deposit);
            let wins = i == j || crowd == 0 || self.claim[j] == keyed(seed, t, CLAIM_MOVE, k);
            if wins {
                self.occ[i] -= 1;
                self.occ[j] += 1;
                self.walkers[k as usize] = (nx, ny, h);
                if cap == 0 {
                    let sick = contagion && self.sick[k as usize] > sick_at;
                    let amount = if sick { dep * self.p.sick_trail } else { dep };
                    self.lay(k as usize, j, amount * self.explore_share(k as usize));
                }
            } else {
                blocked.push((k, h, st));
            }
        }
        if any_crowd {
            for &(_, _, to, ..) in &moves {
                self.claim[to as usize] = u32::MAX;
            }
        }
        // Retry rounds (`blocked_retry`): a blocked walker tries its step
        // again into cells emptied this step, so a packed crowd moving one
        // way moves together, as in Jones's random sequential update,
        // instead of only its front row moving.
        for r in 0..if any_crowd { p.blocked_retry } else { 0 } {
            let mut tries: Vec<(u32, u32, u32, f32, f32, f32, f32)> = Vec::new();
            let mut still: Vec<(u32, f32, f32)> = Vec::new();
            for &(k, h, st) in &blocked {
                let crowd = sets[if per { self.sp_of(k as usize) } else { 0 }].crowd;
                let (x, y, _) = self.walkers[k as usize];
                let here = self.cell(x, y).expect("walker inside the dish");
                let (nx, ny) = (x + h.cos() * st, y + h.sin() * st);
                match self.cell(nx, ny) {
                    Some(there) if there != here && self.agar[there] && self.occ[there] < crowd as u16 && self.tethered(k as usize, here, there) => {
                        let key = keyed(seed, t, CLAIM_RETRY + r as u32, k);
                        self.claim[there] = self.claim[there].min(key);
                        tries.push((k, here as u32, there as u32, nx, ny, h, st));
                    }
                    _ => still.push((k, h, st)),
                }
            }
            if tries.is_empty() {
                break;
            }
            for &(k, from, to, nx, ny, h, st) in &tries {
                let (i, j) = (from as usize, to as usize);
                if self.claim[j] == keyed(seed, t, CLAIM_RETRY + r as u32, k) {
                    self.occ[i] -= 1;
                    self.occ[j] += 1;
                    self.walkers[k as usize] = (nx, ny, h);
                    if cap == 0 {
                        let dep = sets[if per { self.sp_of(k as usize) } else { 0 }].deposit;
                        let sick = contagion && self.sick[k as usize] > sick_at;
                        let amount = if sick { dep * self.p.sick_trail } else { dep };
                        self.lay(k as usize, j, amount * self.explore_share(k as usize));
                    }
                } else {
                    still.push((k, h, st));
                }
            }
            for &(_, _, to, ..) in &tries {
                self.claim[to as usize] = u32::MAX;
            }
            blocked = still;
        }
        // Blocked: Jones's rule, stay put and pick a new heading; or
        // (`blocked_keep`) keep the heading it chose, bar a rare reset.
        if sets.iter().any(|s| s.blocked_deposit > 0.0) && cap == 0 {
            for &(k, ..) in &blocked {
                let set = sets[if per { self.sp_of(k as usize) } else { 0 }];
                if set.blocked_deposit <= 0.0 {
                    continue;
                }
                let (x, y, _) = self.walkers[k as usize];
                let i = self.cell(x, y).expect("walker inside the dish");
                self.lay(k as usize, i, set.deposit * set.blocked_deposit * self.explore_share(k as usize));
            }
        }
        if !self.wall_health.is_empty() {
            self.bite_walls(&blocked);
        }
        for &(k, h, _) in &blocked {
            let keep = p.blocked_keep && unit(keyed(seed, t, DRAW_KEEP, k)) >= p.blocked_reset;
            self.walkers[k as usize].2 = if keep { h } else { unit(keyed(seed, t, DRAW_BLOCKED, k)) * TAU };
        }
        if life_on {
            self.life_step(&flying, &sleeping, &fruit, &doze);
        }
        if cap > 0 {
            // Capped square-root deposit: per cell, from the walkers on it.
            let sick_trail = self.p.sick_trail;
            for k in 0..self.walkers.len() {
                if !self.active(k) {
                    continue;
                }
                let (x, y, _) = self.walkers[k];
                let i = self.cell(x, y).expect("walker inside the dish");
                let n = self.occ[i].min(cap) as f32;
                // Each of the n walkers on the cell lays its share of
                // deposit * sqrt(n): sqrt(n) / n of a lone walker's.
                let dep = sets[if per { self.sp_of(k) } else { 0 }].deposit;
                let share = dep * n.sqrt() / self.occ[i] as f32;
                let sick = contagion && self.sick[k] > sick_at;
                let amount = if sick { share * sick_trail } else { share };
                self.lay(k, i, amount * self.explore_share(k));
            }
        }
        if contagion {
            for k in 0..self.walkers.len() {
                if self.sick[k] > sick_at && self.active(k) {
                    let (x, y, _) = self.walkers[k];
                    let i = self.cell(x, y).expect("walker inside the dish");
                    self.germ[i] += self.p.sick_deposit;
                }
                self.sick[k] = self.sick[k].saturating_sub(1);
            }
        }
        for k in 0..self.age.len() {
            if self.lives(k) {
                self.age[k] = self.age[k].saturating_add(1);
            }
        }
        if !self.grass.is_empty() {
            // Who eats grass: every living walker when growth feeds on it
            // (no crystals), else only species with a `graze_weight`
            // (`species_sets`; without sets, everyone, as grass exists only
            // when grazing is on).
            let growth_grazes = p.upkeep > 0.0 && p.crystal_charge <= 0.0;
            let eats = |d: &Self, k: usize| d.active(k) && (growth_grazes || p.species_sets.is_empty() || p.species_sets[d.sp_of(k)].graze_weight != 0.0);
            if !self.energy.is_empty() {
                // Each walker's equal share of what its cell loses below:
                // min(grass, n x eat) over the n walkers on it. (With
                // species sets where some species do not graze, n still
                // counts every walker on the cell: a grazer beside a
                // non-grazer is credited a little less than its cell
                // loses. Untested together; growth with species sets is
                // not yet on the GPU either.)
                for k in 0..self.walkers.len() {
                    if !eats(self, k) {
                        continue;
                    }
                    let (x, y, _) = self.walkers[k];
                    let i = self.cell(x, y).expect("walker inside the dish");
                    let n = self.occ[i] as f32;
                    self.energy[k] += (self.grass[i] / n).min(p.graze_eat);
                }
            }
            for k in 0..self.walkers.len() {
                if !eats(self, k) {
                    continue;
                }
                let (x, y, _) = self.walkers[k];
                let i = self.cell(x, y).expect("walker inside the dish");
                self.grass[i] = (self.grass[i] - p.graze_eat).max(0.0);
            }
            for (g, &a) in self.grass.iter_mut().zip(&self.agar) {
                if a {
                    *g += p.graze_regrow * (1.0 - *g);
                }
            }
        }
        self.eat_crystals();
        if self.p.energy_flow > 0.0 && !self.energy.is_empty() {
            self.flow_energy();
        }
        if !self.energy.is_empty() {
            self.live_and_die();
        }
        if p.species_convert > 0.0 && !self.spec.is_empty() {
            let n = self.species_trail.len();
            let (w, h, r) = (self.w, self.h, p.species_convert_r as usize);
            // Summed-area tables, (w+1) x (h+1), when comparing squares.
            let sat: Vec<Vec<f64>> = if r > 0 {
                self.species_trail
                    .iter()
                    .map(|f| {
                        let mut t = vec![0.0f64; (w + 1) * (h + 1)];
                        for y in 0..h {
                            let mut row = 0.0f64;
                            for x in 0..w {
                                row += f[y * w + x] as f64;
                                t[(y + 1) * (w + 1) + x + 1] = t[y * (w + 1) + x + 1] + row;
                            }
                        }
                        t
                    })
                    .collect()
            } else {
                Vec::new()
            };
            // Each species' weight a unit of trail (`species_convert_per`).
            let per: Vec<f64> = (0..n).map(|s| if p.species_convert_per { 1.0 / (self.set_of(s).deposit as f64).max(1e-9) } else { 1.0 }).collect();
            let weigh = |s: usize, cx: usize, cy: usize, i: usize| -> f64 {
                if r == 0 {
                    return self.species_trail[s][i] as f64 * per[s];
                }
                let (x0, y0) = (cx.saturating_sub(r), cy.saturating_sub(r));
                let (x1, y1) = ((cx + r + 1).min(w), (cy + r + 1).min(h));
                let t = &sat[s];
                (t[y1 * (w + 1) + x1] - t[y0 * (w + 1) + x1] - t[y1 * (w + 1) + x0] + t[y0 * (w + 1) + x0]) * per[s]
            };
            let mut next = self.spec.clone();
            for (k, slot) in next.iter_mut().enumerate() {
                if !self.lives(k) {
                    continue;
                }
                let (x, y, _) = self.walkers[k];
                let i = self.cell(x, y).expect("walker inside the dish");
                let (cx, cy) = (i % w, i / w);
                let own = self.spec[k] as usize;
                let mut best = own;
                if p.species_cycle && n >= 3 {
                    // Cyclic: only the species that chases you can take you.
                    best = (own + n - 1) % n;
                } else {
                    for s in 0..n {
                        if s != own && weigh(s, cx, cy, i) > weigh(best, cx, cy, i) {
                            best = s;
                        }
                    }
                }
                // r 0 keeps the f32 product, so older runs keep their hashes.
                let wins = if r == 0 && !p.species_convert_per {
                    self.species_trail[best][i] > p.species_convert * self.species_trail[own][i]
                } else {
                    weigh(best, cx, cy, i) > p.species_convert as f64 * weigh(own, cx, cy, i)
                };
                if best != own && wins {
                    *slot = best as u8;
                }
            }
            // Conversion reads only trails, so writing to a copy is the same
            // as converting in place.
            self.spec = next;
        }
        if respawn > 0.0 {
            self.respawn_walkers(respawn, &sets);
        }
    }

    /// Others (`occ`, less the mover on `here`) on the eight cells round
    /// fine cell `i`, and whether `tether` lets a walker on `here` step to
    /// `there`: always with `tether` 0, or when it is a straggler (under
    /// `tether` others round `here`), else when `there` has `tether` or more
    /// others round it.
    fn others_round(&self, i: usize, here: usize) -> u32 {
        let (w, h) = (self.w as i64, self.h as i64);
        let (x, y) = ((i % self.w) as i64, (i / self.w) as i64);
        let mut n = 0u32;
        for dy in -1..=1 {
            for dx in -1..=1 {
                let (nx, ny) = (x + dx, y + dy);
                if (dx, dy) == (0, 0) || nx < 0 || ny < 0 || nx >= w || ny >= h {
                    continue;
                }
                let j = (ny * w + nx) as usize;
                n += if self.holds_n.is_empty() { self.occ[j] as u32 - (j == here) as u32 } else { self.holds_n[j] as u32 };
            }
        }
        n
    }

    /// Walkers in the square of radius `r` round fine cell `i`, less the
    /// cell itself (`occ`).
    fn occ_round(&self, i: usize, r: i64) -> u32 {
        let (w, h) = (self.w as i64, self.h as i64);
        let (x, y) = ((i % self.w) as i64, (i / self.w) as i64);
        let mut n = 0u32;
        for dy in -r..=r {
            for dx in -r..=r {
                let (nx, ny) = (x + dx, y + dy);
                if (dx, dy) != (0, 0) && nx >= 0 && ny >= 0 && nx < w && ny < h {
                    n += self.occ[(ny * w + nx) as usize] as u32;
                }
            }
        }
        n
    }

    /// Walker `k`'s hunger, 0 (fed at `hunger_at` or more) to 1 (empty); 0
    /// with growth off.
    fn hunger(&self, k: usize) -> f32 {
        if self.energy.is_empty() || self.p.hunger_at <= 0.0 {
            return 0.0;
        }
        (1.0 - self.energy[k] / self.p.hunger_at).max(0.0)
    }

    fn tethered(&self, k: usize, here: usize, there: usize) -> bool {
        // With `hunt_at` over 0 only hunting walkers are tethered: a fed
        // body spreads as today, a starving hunk keeps together.
        let t = if self.p.hunt_at > 0.0 && self.hunger(k) < self.p.hunt_at { 0 } else { self.p.tether as u32 };
        t == 0 || self.others_round(there, here) >= t.min(self.others_round(here, here))
    }

    /// Walkers (`occ`) in the square of radius `r` round each fine cell, by
    /// a summed-area table: the body pull's reading (`body_pull`).
    fn body_counts(&self, r: usize) -> Vec<u32> {
        let (w, h) = (self.w, self.h);
        let mut t = vec![0u32; (w + 1) * (h + 1)];
        for y in 0..h {
            let mut row = 0u32;
            for x in 0..w {
                row += self.occ[y * w + x] as u32;
                t[(y + 1) * (w + 1) + x + 1] = t[y * (w + 1) + x + 1] + row;
            }
        }
        let mut out = vec![0u32; w * h];
        for y in 0..h {
            let (y0, y1) = (y.saturating_sub(r), (y + r + 1).min(h));
            for x in 0..w {
                let (x0, x1) = (x.saturating_sub(r), (x + r + 1).min(w));
                out[y * w + x] = t[y1 * (w + 1) + x1] + t[y0 * (w + 1) + x0] - t[y0 * (w + 1) + x1] - t[y1 * (w + 1) + x0];
            }
        }
        out
    }

    /// `energy_flow`: walkers' mean energy evens out between neighbouring
    /// cells that hold walkers. Sums in f64 over slots in order, so the
    /// result does not depend on how walkers sit in their slots beyond
    /// rounding; a GPU port would sum in fixed point. Only ordinary walkers
    /// (`active`) share: a spore or a dormant walker keeps its energy, as
    /// it pays no upkeep (`live_and_die`), so each cell counts its own
    /// sharers rather than `occ`.
    fn flow_energy(&mut self) {
        let d = self.p.energy_flow as f64;
        let (w, h) = (self.w, self.h);
        let mut mean = vec![0f64; w * h];
        let mut n = vec![0u32; w * h];
        let mut cells = vec![usize::MAX; self.walkers.len()];
        for (k, c) in cells.iter_mut().enumerate() {
            if !self.active(k) {
                continue;
            }
            let (x, y, _) = self.walkers[k];
            let i = self.cell(x, y).expect("walker inside the dish");
            *c = i;
            mean[i] += self.energy[k] as f64;
            n[i] += 1;
        }
        for (m, &c) in mean.iter_mut().zip(&n) {
            if c > 0 {
                *m /= c as f64;
            }
        }
        let mut gain = vec![0f64; w * h];
        for y in 0..h {
            for x in 0..w {
                let i = y * w + x;
                if n[i] == 0 {
                    continue;
                }
                let mut f = 0.0;
                for (dx, dy) in [(1i64, 0i64), (1, 1), (0, 1), (-1, 1), (-1, 0), (-1, -1), (0, -1), (1, -1)] {
                    let (nx, ny) = (x as i64 + dx, y as i64 + dy);
                    if nx < 0 || ny < 0 || nx >= w as i64 || ny >= h as i64 {
                        continue;
                    }
                    let j = ny as usize * w + nx as usize;
                    if n[j] > 0 {
                        f += mean[j] - mean[i];
                    }
                }
                gain[i] = d * f / n[i] as f64;
            }
        }
        for (k, &i) in cells.iter().enumerate() {
            if i != usize::MAX {
                self.energy[k] = (self.energy[k] as f64 + gain[i]).max(0.0) as f32;
            }
        }
    }

    /// Growth's step (`upkeep`), after eating: upkeep and starving, then
    /// births. Deaths come first, so a cell a walker starved on is free for
    /// a birth in the same step; births only see the dish as it stood after
    /// the deaths (cells free then, the claims among themselves), as a GPU
    /// pass would.
    ///
    /// With the life cycle (`BigDish::life`), only ordinary walkers pay
    /// upkeep, starve and divide: a spore or a dormant walker keeps its
    /// energy until it lands or wakes (a choice, untested in play; a
    /// dormant upkeep could make sleeping cost something). A struck walker
    /// frees its slot (`strike`), so a birth can reuse it, and a child is
    /// an ordinary walker with its timer at 0.
    fn live_and_die(&mut self) {
        let p = self.p.clone();
        let (seed, t) = (p.seed, self.steps);
        // A parent's crowd limit is its species' (`species_sets`) if set.
        let crowd_of = |d: &Self, k: usize| if p.species_sets.is_empty() { p.crowd } else { p.species_sets[d.sp_of(k)].crowd };
        let n = self.walkers.len();
        for k in 0..n {
            if !self.active(k) {
                continue;
            }
            // Its crystal take this step (`took`, Larder World) counts as
            // food like its grass share.
            let took = if self.took.is_empty() { 0.0 } else { self.took[k] };
            let mut up = if self.soil.is_empty() {
                p.upkeep
            } else {
                let (x, y, _) = self.walkers[k];
                let i = self.cell(x, y).expect("walker inside the dish");
                if self.soil[i] == SOIL_BARREN { p.upkeep * p.barren_upkeep } else { p.upkeep }
            };
            if p.lone_upkeep != 1.0 {
                let (x, y, _) = self.walkers[k];
                let i = self.cell(x, y).expect("walker inside the dish");
                if self.occ_round(i, p.lone_r.max(1) as i64) == 0 {
                    up *= p.lone_upkeep;
                }
            }
            let up = if !self.sick.is_empty() && self.sick[k] > p.immune_steps { up * p.sick_upkeep } else { up };
            let e = self.energy[k] + took - up;
            if e > 0.0 {
                self.energy[k] = e;
                continue;
            }
            self.energy[k] = 0.0;
            if unit(keyed(seed, t, DRAW_STARVE, k as u32)) < p.starve {
                let (x, y, _) = self.walkers[k];
                let i = self.cell(x, y).expect("walker inside the dish");
                self.occ[i] -= 1;
                self.alive[k] = false;
                self.starved += 1;
            }
        }
        // Births: parent k's child takes slot k ^ m if that slot is empty
        // and k ^ m < n; m is a keyed draw a step, below the power of two
        // at or over n.
        let mask = n.max(2).next_power_of_two() - 1;
        let m = keyed(seed, t, DRAW_BIRTH, u32::MAX) as usize & mask;
        const NEXT: [(f32, f32); 8] = [(1.0, 0.0), (1.0, 1.0), (0.0, 1.0), (-1.0, 1.0), (-1.0, 0.0), (-1.0, -1.0), (0.0, -1.0), (1.0, -1.0)];
        // (parent, child slot, cell, x, y)
        let mut tries: Vec<(u32, u32, u32, f32, f32)> = Vec::new();
        for k in 0..n {
            let b = k ^ m;
            if !self.active(k) || self.energy[k] < p.birth_energy || b >= n || self.alive[b] {
                continue;
            }
            let (x, y, _) = self.walkers[k];
            let (dx, dy) = NEXT[(keyed(seed, t, DRAW_BIRTH, k as u32) % 8) as usize];
            let (nx, ny) = (x + dx, y + dy);
            let Some(j) = self.cell(nx, ny) else { continue };
            let crowd = crowd_of(self, k);
            if !self.agar[j] || (crowd > 0 && self.occ[j] >= crowd as u16) || (!self.soil.is_empty() && self.soil[j] == SOIL_BARREN) {
                continue;
            }
            if crowd > 0 {
                let key = keyed(seed, t, CLAIM_BIRTH, k as u32);
                self.claim[j] = self.claim[j].min(key);
            }
            tries.push((k as u32, b as u32, j as u32, nx, ny));
        }
        for &(k, b, j, nx, ny) in &tries {
            let (k, b, j) = (k as usize, b as usize, j as usize);
            if crowd_of(self, k) > 0 && self.claim[j] != keyed(seed, t, CLAIM_BIRTH, k as u32) {
                continue;
            }
            let half = ((self.energy[k] - p.birth_cost) * 0.5).max(0.0);
            self.energy[k] = half;
            self.energy[b] = half;
            self.alive[b] = true;
            self.occ[j] += 1;
            self.walkers[b] = (nx, ny, unit(keyed(seed, t, DRAW_BIRTH, k as u32 | 0x8000_0000)) * TAU);
            if !self.age.is_empty() {
                self.age[b] = 0;
            }
            if !self.sick.is_empty() {
                self.sick[b] = 0;
            }
            if !self.gassed.is_empty() {
                self.gassed[b] = 0;
            }
            if !self.spec.is_empty() {
                self.spec[b] = self.spec[k];
            }
            // A child is its parent's kind (`mix_share`), as it is its
            // species.
            if !self.kind.is_empty() {
                self.kind[b] = self.kind[k];
            }
            if !self.life.is_empty() {
                self.life[b] = LIFE_WALKER;
                self.life_t[b] = 0;
            }
            self.births += 1;
        }
        for &(_, _, j, ..) in &tries {
            self.claim[j as usize] = u32::MAX;
        }
    }

    /// Whether slot `k` holds an ordinary walker: a living one (`lives`,
    /// growth) that is not a spore, dormant or dead (life cycle); always,
    /// with both off.
    fn active(&self, k: usize) -> bool {
        self.lives(k) && (self.life.is_empty() || self.life[k] == LIFE_WALKER)
    }

    /// The life cycle's part of a fine step, after the ordinary walkers have
    /// moved and laid (`BigParams::spore_rate` and below): walkers that fruit
    /// or fall dormant this step change state, the dormant lay and may wake,
    /// and spores fly and may land.
    fn life_step(&mut self, flying: &[usize], sleeping: &[usize], fruit: &[usize], doze: &[usize]) {
        let p = self.p.clone();
        let (seed, t, dep, crowd) = (p.seed, self.steps, p.deposit, p.crowd);
        for k in 0..self.walkers.len() {
            if matches!(self.life[k], LIFE_WALKER | LIFE_DORMANT) {
                self.life_t[k] = self.life_t[k].saturating_sub(1);
            }
        }
        for &k in fruit {
            let (x, y, _) = self.walkers[k];
            let i = self.cell(x, y).expect("walker inside the dish");
            self.occ[i] -= 1;
            self.life[k] = LIFE_SPORE;
            self.life_t[k] = p.spore_steps;
            self.walkers[k].2 = unit(keyed(seed, t, DRAW_SPORE_H, k as u32)) * TAU;
        }
        for &k in doze {
            self.life[k] = LIFE_DORMANT;
            self.life_t[k] = p.sleep_rest;
        }
        // Dormant: lay, then wake on scent or alarm. Every wake is decided
        // before any relay is added, so the order of walkers does not matter.
        let mut woke = Vec::new();
        for &k in sleeping {
            let (x, y, _) = self.walkers[k];
            let i = self.cell(x, y).expect("walker inside the dish");
            let gas = !self.gassed.is_empty() && self.gassed[k] > 0;
            let death = if gas { p.gas_death.max(p.sleep_death) } else { p.sleep_death };
            if death > 0.0 && unit(keyed(seed, t, DRAW_SLEEP_DIE, k as u32)) < death {
                self.occ[i] -= 1;
                self.life[k] = LIFE_DEAD;
                // With growth on, the slot is freed for a birth to reuse.
                if !self.alive.is_empty() {
                    self.alive[k] = false;
                    self.starved += 1;
                }
                continue;
            }
            if p.sleep_deposit > 0.0 {
                self.lay(k, i, dep * p.sleep_deposit);
            }
            let alarm = if self.alarm.is_empty() { 0.0 } else { self.alarm[i] };
            if (!gas && self.scent_at(x, y) > p.wake_scent) || (alarm > p.wake_alarm && self.life_t[k] == 0) {
                woke.push((k, i));
            }
        }
        for g in self.gassed.iter_mut() {
            *g = g.saturating_sub(1);
        }
        for &(k, i) in &woke {
            self.life[k] = LIFE_WALKER;
            self.life_t[k] = p.wake_steps;
            if !self.alarm.is_empty() {
                self.alarm[i] += p.alarm_relay;
            }
        }
        // Spores: steer by scent, fly, lay a streak, land.
        let (so, sa, ra, st) = (p.spore_reach, p.sensor_angle, p.rotate_angle, p.step * p.spore_speed);
        let mut landing = Vec::new();
        for &k in flying {
            let (x, y, mut h) = self.walkers[k];
            let tt = self.life_t[k];
            let homing = tt > 0 && tt <= p.spore_steps / 2;
            if so > 0.0 && homing {
                let read = |a: f32| {
                    let (px, py) = (x + a.cos() * so, y + a.sin() * so);
                    match self.cell(px, py) {
                        Some(i) if self.agar[i] => self.scent_at(px, py),
                        _ => -1.0e6,
                    }
                };
                let (f, fl, fr) = (read(h), read(h - sa), read(h + sa));
                if f > fl && f > fr {
                } else if f < fl && f < fr {
                    h += if keyed(seed, t, DRAW_SPORE_H, k as u32 | 0x8000_0000) & 1 == 0 { ra } else { -ra };
                } else if fl < fr {
                    h += ra;
                } else if fr < fl {
                    h -= ra;
                }
            }
            let (nx, ny) = (x + h.cos() * st, y + h.sin() * st);
            self.walkers[k] = match self.cell(nx, ny) {
                Some(j) if self.agar[j] => (nx, ny, h),
                _ => (x, y, unit(keyed(seed, t, DRAW_SPORE_H, k as u32 | 0x4000_0000)) * TAU),
            };
            let (x, y, _) = self.walkers[k];
            let i = self.cell(x, y).expect("walker inside the dish");
            if p.spore_deposit > 0.0 {
                self.lay(k, i, dep * p.spore_deposit);
            }
            self.life_t[k] = self.life_t[k].saturating_sub(1);
            let lands = self.life_t[k] == 0 || (homing && p.spore_land > 0.0 && self.scent_at(x, y) > p.spore_land);
            // The crowd limit is the spore's own species' (`species_sets`),
            // and a cell a wall has taken since it flew there is no landing.
            let crowd = if p.species_sets.is_empty() { crowd } else { p.species_sets[self.sp_of(k)].crowd };
            if lands && self.agar[i] && (crowd == 0 || self.occ[i] < crowd as u16) {
                self.claim[i] = self.claim[i].min(keyed(seed, t, CLAIM_LAND, k as u32));
                landing.push((k, i, crowd));
            }
        }
        for &(k, i, crowd) in &landing {
            if crowd == 0 || self.claim[i] == keyed(seed, t, CLAIM_LAND, k as u32) {
                self.occ[i] += 1;
                self.life[k] = LIFE_WALKER;
                self.life_t[k] = p.spore_rest;
            }
        }
        for &(_, i, _) in &landing {
            self.claim[i] = u32::MAX;
        }
    }

    /// The player's attack (added 2026-11-11, to test how each tier answers
    /// it): within `r` dish cells of (`x`, `y`, dish cells) it clears the
    /// trail (each species' too) and kills every walker on the ground
    /// (spores in flight are missed); within `alarm_r` dish cells it adds
    /// `alarm` to every agar cell of the alarm field, if there is one (the
    /// shock reaches past the kill: one 3x3 mean a step spreads alarm only a
    /// few fine cells a second). With `r` 0 it is only a shock (a bait):
    /// nothing is cleared or killed. Allocates the life states if they are
    /// off. Returns the walkers killed.
    pub fn strike(&mut self, x: f32, y: f32, r: f32, alarm: f32, alarm_r: f32) -> usize {
        if self.life.is_empty() {
            self.life = vec![LIFE_WALKER; self.walkers.len()];
            self.life_t = vec![0; self.walkers.len()];
        }
        let s = self.p.scale as f32;
        let (cx, cy, rr, ar) = (x * s, y * s, r * s, alarm_r * s);
        let n = rr.max(ar).ceil() as i64;
        for dy in -n..=n {
            for dx in -n..=n {
                let d2 = (dx * dx + dy * dy) as f32;
                let Some(i) = self.cell(cx + dx as f32, cy + dy as f32) else { continue };
                if rr > 0.0 && d2 <= rr * rr {
                    self.trail[i] = 0.0;
                    for f in self.species_trail.iter_mut() {
                        f[i] = 0.0;
                    }
                }
                if d2 <= ar * ar && !self.alarm.is_empty() && self.agar[i] {
                    self.alarm[i] += alarm;
                }
            }
        }
        let mut killed = 0;
        for k in 0..self.walkers.len() {
            let (wx, wy, _) = self.walkers[k];
            if !self.lives(k) || rr <= 0.0 || (wx - cx).hypot(wy - cy) > rr || matches!(self.life[k], LIFE_SPORE | LIFE_DEAD) {
                continue;
            }
            let i = self.cell(wx, wy).expect("walker inside the dish");
            self.occ[i] -= 1;
            self.life[k] = LIFE_DEAD;
            // With growth on, the slot is freed for a birth to reuse.
            if !self.alive.is_empty() {
                self.alive[k] = false;
            }
            killed += 1;
        }
        killed
    }

    /// Walkers in each life state: ordinary, spores, dormant, dead. With
    /// growth on, an empty slot counts in none, and a struck walker's slot
    /// is empty (`strike`), so the dead stay 0.
    pub fn life_counts(&self) -> [usize; 4] {
        let mut c = [0usize; 4];
        for k in 0..self.walkers.len() {
            if self.lives(k) {
                c[self.life.get(k).copied().unwrap_or(LIFE_WALKER) as usize] += 1;
            }
        }
        c
    }

    /// The species walker number `k` starts as (`species`, `species_share`).
    fn species_of(&self, k: usize) -> usize {
        let n = self.p.species.max(1) as usize;
        let sh = self.p.species_share;
        if sh <= 0.0 || n < 2 {
            return k % n;
        }
        // Walker k is species 0 when floor((k+1) x sh) > floor(k x sh): an
        // even spread with exactly the share over any long run.
        let (a, b) = ((k as f64 * sh as f64).floor(), ((k + 1) as f64 * sh as f64).floor());
        if b > a {
            0
        } else {
            1 + k % (n - 1)
        }
    }

    /// Whether walker `k` is still an explorer (`explore_steps`, `explore_spread`).
    fn explores(&self, k: usize) -> bool {
        if self.age.is_empty() {
            return false;
        }
        let mut n = if self.p.species_sets.is_empty() { self.p.explore_steps } else { self.p.species_sets[self.sp_of(k)].explore_steps } as f32;
        if self.p.explore_spread > 0.0 {
            let u = unit(keyed(self.p.seed, 0, DRAW_SIZE, k as u32 | 0x8000_0000));
            n *= 1.0 + self.p.explore_spread * (2.0 * u - 1.0);
        }
        (self.age[k] as f32) < n
    }

    /// The share of the deposit walker `k` lays: `explore_deposit` while it
    /// explores, else 1.
    fn explore_share(&self, k: usize) -> f32 {
        if self.explores(k) {
            self.p.explore_deposit
        } else {
            1.0
        }
    }

    /// `respawn`: each walker, by its own keyed draws, may jump to a random
    /// agar cell with a new heading; a jump onto rim or a full cell is lost.
    fn respawn_walkers(&mut self, respawn: f32, sets: &[SpeciesSet]) {
        let (seed, t) = (self.p.seed, self.steps);
        let (w, h) = (self.w as f32, self.h as f32);
        for k in 0..self.walkers.len() {
            let key = 4 * k as u32;
            if unit(keyed(seed, t, DRAW_RESPAWN, key)) >= respawn || !self.active(k) {
                continue;
            }
            let x = unit(keyed(seed, t, DRAW_RESPAWN, key + 1)) * w;
            let y = unit(keyed(seed, t, DRAW_RESPAWN, key + 2)) * h;
            let Some(j) = self.cell(x, y) else { continue };
            let crowd = sets[if sets.len() > 1 { self.sp_of(k) } else { 0 }].crowd;
            if !self.agar[j] || (crowd > 0 && self.occ[j] >= crowd as u16) {
                continue;
            }
            let (ox, oy, _) = self.walkers[k];
            let i = self.cell(ox, oy).expect("walker inside the dish");
            self.occ[i] -= 1;
            self.occ[j] += 1;
            self.walkers[k] = (x, y, unit(keyed(seed, t, DRAW_RESPAWN, key + 3)) * TAU);
            if !self.age.is_empty() {
                self.age[k] = 0;
            }
        }
    }

    /// Piles lay scent on the scent grid: full until the tube reaches them,
    /// then `fed_scent` of that, as today's `deposit_food` does.
    fn lay_scent(&mut self) {
        let k = if self.p.scent_coarse { 1.0 } else { self.p.scale as f32 };
        for (pi, &(px, py)) in self.piles.iter().enumerate() {
            let r = self.pile_rs[pi] * k;
            let n = r.ceil() as i32;
            let strength = self.p.food_deposit * if self.reached[pi] { self.p.fed_scent } else { 1.0 } * self.crystal_left(pi);
            let (cx, cy) = (px * k, py * k);
            for dy in -n..=n {
                for dx in -n..=n {
                    if !self.in_pile(dx as f32, dy as f32, r) {
                        continue;
                    }
                    let (x, y) = (cx + dx as f32, cy + dy as f32);
                    if x < 0.0 || y < 0.0 || x >= self.sw as f32 || y >= self.sh as f32 {
                        continue;
                    }
                    let i = y as usize * self.sw + x as usize;
                    if self.scent_agar[i] {
                        self.scent[i] += strength;
                    }
                }
            }
        }
    }

    fn scent_step(&mut self) {
        self.lay_scent();
        spread(&self.scent_agar, &self.scent, &mut self.scent_scratch, self.sw, self.sh, self.p.scent_keep, self.threads);
        std::mem::swap(&mut self.scent, &mut self.scent_scratch);
        if !self.scent_wall.is_empty() {
            let keep = self.p.wall_scent;
            for (v, &w) in self.scent.iter_mut().zip(&self.scent_wall) {
                if w {
                    *v *= keep;
                }
            }
        }
    }

    /// The scent the piles lay on each scent cell in one scent step, as
    /// `lay_scent` adds it, for the piles reached so far. The GPU timing
    /// harness (`examples/gputime.rs`) uploads it once.
    pub fn scent_laid(&self) -> Vec<f32> {
        let mut out = vec![0.0f32; self.sw * self.sh];
        let k = if self.p.scent_coarse { 1.0 } else { self.p.scale as f32 };
        for (pi, &(px, py)) in self.piles.iter().enumerate() {
            let r = self.pile_rs[pi] * k;
            let n = r.ceil() as i32;
            let strength = self.p.food_deposit * if self.reached[pi] { self.p.fed_scent } else { 1.0 } * self.crystal_left(pi);
            let (cx, cy) = (px * k, py * k);
            for dy in -n..=n {
                for dx in -n..=n {
                    if !self.in_pile(dx as f32, dy as f32, r) {
                        continue;
                    }
                    let (x, y) = (cx + dx as f32, cy + dy as f32);
                    if x < 0.0 || y < 0.0 || x >= self.sw as f32 || y >= self.sh as f32 {
                        continue;
                    }
                    let i = y as usize * self.sw + x as usize;
                    if self.scent_agar[i] {
                        out[i] += strength;
                    }
                }
            }
        }
        out
    }

    /// A pile counts as reached once tube lies on it.
    pub fn update_reached(&mut self) {
        let s = self.p.scale as f32;
        for pi in 0..self.piles.len() {
            if self.reached[pi] {
                continue;
            }
            let r = self.pile_rs[pi] * s;
            let n = r.ceil() as i32;
            let (cx, cy) = (self.piles[pi].0 * s, self.piles[pi].1 * s);
            'pile: for dy in -n..=n {
                for dx in -n..=n {
                    if !self.in_pile(dx as f32, dy as f32, r) {
                        continue;
                    }
                    if let Some(i) = self.cell(cx + dx as f32, cy + dy as f32) {
                        if self.trail[i] > self.p.tube {
                            self.reached[pi] = true;
                            break 'pile;
                        }
                    }
                }
            }
        }
    }

    /// One tick: `substeps` fine steps of walkers and trail, and the scent
    /// once (on today's grid) or each fine step (on the fine grid).
    pub fn tick(&mut self) {
        for _ in 0..self.p.substeps {
            self.move_walkers();
            if self.species_trail.is_empty() {
                for pass in 0..self.p.trail_passes.max(1) {
                    let keep = if pass == 0 { self.p.keep } else { 1.0 };
                    spread(&self.agar, &self.trail, &mut self.scratch, self.w, self.h, keep, self.threads);
                    std::mem::swap(&mut self.trail, &mut self.scratch);
                }
            } else {
                // Each species' field spreads as the trail would; the trail
                // is their sum (the spread is linear, so the same field).
                for sp in 0..self.species_trail.len() {
                    // Each species' own keep (`species_sets`), else the global.
                    let own = if self.p.species_sets.is_empty() { self.p.keep } else { self.p.species_sets[sp].keep };
                    for pass in 0..self.p.trail_passes.max(1) {
                        let keep = if pass == 0 { own } else { 1.0 };
                        spread(&self.agar, &self.species_trail[sp], &mut self.scratch, self.w, self.h, keep, self.threads);
                        std::mem::swap(&mut self.species_trail[sp], &mut self.scratch);
                    }
                }
                for (i, v) in self.trail.iter_mut().enumerate() {
                    *v = self.species_trail.iter().map(|f| f[i]).sum();
                }
            }
            if !self.soil.is_empty() {
                let (sand, clay) = (self.p.sand_keep, self.p.clay_keep);
                let soil = &self.soil;
                for f in std::iter::once(&mut self.trail).chain(self.species_trail.iter_mut()) {
                    for (v, &k) in f.iter_mut().zip(soil) {
                        match k {
                            SOIL_SAND => *v *= sand,
                            SOIL_CLAY => *v *= clay,
                            _ => {}
                        }
                    }
                }
            }
            if !self.scorch.is_empty() {
                let (cut, full) = (1.0 - self.p.scorch_keep, self.p.scorch_full.max(1) as f32);
                let scorch = &mut self.scorch;
                for f in std::iter::once(&mut self.trail).chain(self.species_trail.iter_mut()) {
                    for (v, &c) in f.iter_mut().zip(scorch.iter()) {
                        if c > 0 {
                            *v *= 1.0 - cut * (c as f32 / full).min(1.0);
                        }
                    }
                }
                for c in scorch.iter_mut() {
                    *c = c.saturating_sub(1);
                }
            }
            if !self.germ.is_empty() {
                spread(&self.agar, &self.germ, &mut self.scratch, self.w, self.h, self.p.sick_keep, self.threads);
                std::mem::swap(&mut self.germ, &mut self.scratch);
            }
            if !self.alarm.is_empty() {
                spread(&self.agar, &self.alarm, &mut self.scratch, self.w, self.h, self.p.alarm_keep, self.threads);
                std::mem::swap(&mut self.alarm, &mut self.scratch);
            }
            if !self.p.scent_coarse {
                self.scent_step();
            }
            self.steps += 1;
        }
        if self.p.scent_coarse {
            self.scent_step();
        }
        self.update_reached();
        self.ticks += 1;
    }

    pub fn seconds(&self) -> f32 {
        self.ticks as f32 / STEPS_PER_SECOND as f32
    }

    pub fn measure(&self) -> BigMeasure {
        let (w, h) = (self.w, self.h);
        let s = self.p.scale as f32;
        let tube: Vec<bool> = self.trail.iter().zip(&self.agar).map(|(&v, &a)| a && v > self.p.tube).collect();
        let area = tube.iter().filter(|&&t| t).count();
        // Edge: tube cells beside a cell that is not tube.
        let mut edge = 0usize;
        let (hx, hy) = (self.piles[0].0 * s, self.piles[0].1 * s);
        let mut reach = 0.0f32;
        for y in 1..h - 1 {
            for x in 1..w - 1 {
                let i = y * w + x;
                if !tube[i] {
                    continue;
                }
                if !(tube[i - 1] && tube[i + 1] && tube[i - w] && tube[i + w]) {
                    edge += 1;
                }
                reach = reach.max((x as f32 + 0.5 - hx).hypot(y as f32 + 0.5 - hy));
            }
        }
        // Loops: 4-connected pieces of agar that is not tube, not touching
        // the rim, of at least one dish cell.
        let mut seen = vec![false; w * h];
        let mut loops = 0;
        let min = (s * s) as usize;
        let mut stack = Vec::new();
        for start in 0..w * h {
            if seen[start] || tube[start] || !self.agar[start] {
                continue;
            }
            seen[start] = true;
            stack.push(start);
            let (mut size, mut open) = (0usize, false);
            while let Some(i) = stack.pop() {
                size += 1;
                let (x, y) = (i % w, i / w);
                for (ok, j) in [(x > 0, i.wrapping_sub(1)), (x + 1 < w, i + 1), (y > 0, i.wrapping_sub(w)), (y + 1 < h, i + w)] {
                    if !ok {
                        open = true;
                        continue;
                    }
                    if !self.agar[j] {
                        open = true;
                    } else if !tube[j] && !seen[j] {
                        seen[j] = true;
                        stack.push(j);
                    }
                }
            }
            if !open && size >= min {
                loops += 1;
            }
        }
        // Living: a walker in its slot (growth) that has not been struck
        // dead (life cycle); spores and the dormant count.
        let living = |k: usize| self.lives(k) && (self.life.is_empty() || self.life[k] != LIFE_DEAD);
        let strays = (0..self.walkers.len())
            .filter(|&k| living(k))
            .filter(|&k| self.cell(self.walkers[k].0, self.walkers[k].1).is_some_and(|i| self.trail[i] <= self.p.tube))
            .count();
        let alive = (0..self.walkers.len()).filter(|&k| living(k)).count();
        let energy = if self.energy.is_empty() { 0.0 } else { (0..self.walkers.len()).filter(|&k| living(k)).map(|k| self.energy[k]).sum::<f32>() / alive.max(1) as f32 };
        BigMeasure {
            seconds: self.seconds(),
            tube_area: area as f32 / (s * s),
            loops,
            width: if edge > 0 { 2.0 * area as f32 / edge as f32 / s } else { 0.0 },
            reach: reach / s,
            reached: self.reached.iter().filter(|&&r| r).count(),
            strays: strays as f32 / alive.max(1) as f32,
            alive,
            energy,
        }
    }

    /// The trail as a picture, one pixel a fine cell, `down` fine cells to a
    /// pixel each way (the brightest of them). Brightness follows the trail
    /// smoothly, with no tube edge drawn (the owner's rule: the picture is
    /// the data); the piles show as their scent's faint glow.
    pub fn image(&self, down: usize) -> image::RgbaImage {
        let down = down.max(1);
        let (iw, ih) = ((self.w / down) as u32, (self.h / down) as u32);
        let mut img = image::RgbaImage::new(iw, ih);
        for py in 0..ih {
            for px in 0..iw {
                let (mut v, mut inside, mut ch, mut rock, mut wet, mut wall) = (0.0f32, false, 0.0f32, false, false, false);
                let mut wall_left = 1.0f32;
                for dy in 0..down {
                    for dx in 0..down {
                        let i = (py as usize * down + dy) * self.w + px as usize * down + dx;
                        v = v.max(self.trail[i]);
                        inside |= self.agar[i];
                        if !self.charge.is_empty() && self.pile_of[i] > 0 {
                            ch = ch.max(self.charge[i] / self.pile_fill[self.pile_of[i] as usize - 1]);
                        }
                        if !self.rock.is_empty() {
                            rock |= self.rock[i];
                            wet |= self.wet[i];
                        }
                        if !self.wall.is_empty() && self.wall[i] {
                            wall = true;
                            if !self.wall_health.is_empty() {
                                wall_left = wall_left.min(self.wall_health[i] / self.p.wall_hp);
                            }
                        }
                    }
                }
                let c = (px as f32 + 0.5) * down as f32;
                let r = (py as f32 + 0.5) * down as f32;
                let sc = self.scent_at(c, r);
                let px_rgb = if !inside {
                    // Rock is drawn apart from the world's edge: it is ground.
                    if wall {
                        // A worn wall dims toward the rock colour.
                        let a = 0.25 + 0.75 * wall_left.max(0.0);
                        [(34.0 + 116.0 * a) as u8, (30.0 + 120.0 * a) as u8, (28.0 + 130.0 * a) as u8]
                    } else if rock {
                        [34, 30, 28]
                    } else {
                        [8, 14, 18]
                    }
                } else {
                    // Trail: a soft knee, so faint wisps show and tubes glow.
                    let a = 1.0 - (-v / 6.0).exp();
                    let g = 1.0 - (-sc / 60.0).exp();
                    let mut base = [12.0 + 30.0 * g, 22.0 + 26.0 * g, 26.0 + 10.0 * g];
                    if wet {
                        base = [base[0] * 0.7, base[1] * 0.9, base[2] * 1.0 + 16.0];
                    }
                    // A crystal's glow is its charge: full is bright cyan,
                    // drained is ground again, with nothing drawn in between.
                    let crystal = [140.0, 235.0, 255.0];
                    for k in 0..3 {
                        base[k] += (crystal[k] - base[k]) * ch;
                    }
                    let hot = [255.0, 190.0, 90.0];
                    let mut out = [0u8; 3];
                    for k in 0..3 {
                        out[k] = (base[k] + (hot[k] - base[k]) * a).clamp(0.0, 255.0) as u8;
                    }
                    out
                };
                img.put_pixel(px, py, image::Rgba([px_rgb[0], px_rgb[1], px_rgb[2], 255]));
            }
        }
        img
    }

    /// A hash of the walkers and the trail, for the tests: the same seed and
    /// numbers must give the same dish, whatever `threads` is.
    pub fn hash(&self) -> u64 {
        let mut h: u64 = 0xCBF2_9CE4_8422_2325;
        let mut eat = |v: u32| {
            for b in v.to_le_bytes() {
                h ^= b as u64;
                h = h.wrapping_mul(0x0100_0000_01B3);
            }
        };
        for &(x, y, a) in &self.walkers {
            eat(x.to_bits());
            eat(y.to_bits());
            eat(a.to_bits());
        }
        for v in self.trail.iter().chain(&self.scent).chain(&self.germ).chain(&self.grass).chain(&self.charge).chain(&self.wall_health).chain(self.species_trail.iter().flatten()) {
            eat(v.to_bits());
        }
        for &c in self.spec.iter().chain(&self.kind) {
            eat(c as u32);
        }
        for &c in self.sick.iter().chain(&self.age).chain(&self.life_t) {
            eat(c as u32);
        }
        for &a in &self.alive {
            eat(a as u32);
        }
        for &e in &self.energy {
            eat(e.to_bits());
        }
        for &c in &self.life {
            eat(c as u32);
        }
        for v in &self.alarm {
            eat(v.to_bits());
        }
        for &c in self.scorch.iter().chain(&self.gassed) {
            eat(c as u32);
        }
        h
    }

    /// Turns on the life cycle's states (as a strike does), so walkers can be
    /// put to sleep or killed by the squad's tools. Does nothing if on.
    pub fn enable_life(&mut self) {
        if self.life.is_empty() {
            self.life = vec![LIFE_WALKER; self.walkers.len()];
            self.life_t = vec![0; self.walkers.len()];
        }
    }

    /// Turns on the alarm field with `keep` (if it is off), so blasts lay
    /// alarm and gunfire can wake gassed walkers. `alarm_weight` is left as
    /// it is: at 0 the alarm steers nothing, it only wakes.
    pub fn enable_alarm(&mut self, keep: f32) {
        if self.alarm.is_empty() {
            self.alarm = vec![0.0; self.w * self.h];
            self.p.alarm_keep = keep;
        }
    }

    /// Turns on contagion with the current `sick_*` settings (`sick_steps`
    /// must be over 0); nobody is sick until `infect`.
    pub fn enable_contagion(&mut self) {
        assert!(self.p.sick_steps > 0, "contagion needs sick_steps");
        if self.sick.is_empty() {
            self.sick = vec![0; self.walkers.len()];
            self.germ = vec![0.0; self.w * self.h];
        }
    }

    /// Scorches the agar within `r` fine cells of (`x`, `y`, fine cells) for
    /// `steps` fine steps (`BigParams::scorch_keep`, fading over the last
    /// `scorch_full`); a cell keeps the longer of its scorch and this one.
    /// The one crater writer: blasts, det cord and (Larder thread) shots.
    /// Returns cells scorched.
    pub fn scorch_disc(&mut self, x: f32, y: f32, r: f32, steps: u16) -> usize {
        if self.scorch.is_empty() {
            self.scorch = vec![0; self.w * self.h];
        }
        let n = r.ceil() as i64;
        let mut c = 0;
        for dy in -n..=n {
            for dx in -n..=n {
                if (dx * dx + dy * dy) as f32 > r * r {
                    continue;
                }
                let Some(i) = self.cell(x + dx as f32, y + dy as f32) else { continue };
                if self.agar[i] {
                    self.scorch[i] = self.scorch[i].max(steps);
                    c += 1;
                }
            }
        }
        c
    }

    /// **Sleep gas** (the squad toolkit): every ordinary walker within `r`
    /// fine cells of (`x`, `y`, fine cells) falls dormant at once and stays
    /// deaf to scent for `steps` fine steps; alarm still wakes it (its
    /// timer is set to 0), so gunfire or a blast nearby ends the gas.
    /// Returns walkers gassed.
    pub fn gas(&mut self, x: f32, y: f32, r: f32, steps: u16) -> usize {
        self.enable_life();
        if self.gassed.is_empty() {
            self.gassed = vec![0; self.walkers.len()];
        }
        let mut n = 0;
        for k in 0..self.walkers.len() {
            let (wx, wy, _) = self.walkers[k];
            if self.lives(k) && matches!(self.life[k], LIFE_WALKER | LIFE_DORMANT) && (wx - x).hypot(wy - y) <= r {
                self.life[k] = LIFE_DORMANT;
                self.life_t[k] = 0;
                self.gassed[k] = steps;
                n += 1;
            }
        }
        n
    }

    /// For each point (fine cells), the gassed dormant walkers within `r`
    /// of it: the sleeping mould that does not bite a soldier.
    pub fn gassed_near(&self, at: &[(f32, f32)], r: f32) -> Vec<u32> {
        let mut out = vec![0; at.len()];
        if self.gassed.is_empty() {
            return out;
        }
        for k in 0..self.walkers.len() {
            if self.gassed[k] == 0 || !self.lives(k) || self.life[k] != LIFE_DORMANT {
                continue;
            }
            let (wx, wy, _) = self.walkers[k];
            for (o, &(x, y)) in out.iter_mut().zip(at) {
                if (wx - x).hypot(wy - y) <= r {
                    *o += 1;
                }
            }
        }
        out
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    /// The first `spread_rows` (branches on each neighbour's agar), kept as
    /// the reference the branchless one must match bit for bit.
    fn spread_rows_reference(agar: &[bool], from: &[f32], to: &mut [f32], w: usize, h: usize, keep: f32, y0: usize) {
        let rows = to.len() / w;
        for r in 0..rows {
            let y = y0 + r;
            for x in 0..w {
                let i = y * w + x;
                let o = r * w + x;
                if !agar[i] || x == 0 || y == 0 || x + 1 == w || y + 1 == h {
                    to[o] = 0.0;
                    continue;
                }
                let mut sum = 0.0;
                let mut n = 0.0;
                for row in [i - w, i, i + w] {
                    for j in [row - 1, row, row + 1] {
                        if agar[j] {
                            sum += from[j];
                            n += 1.0;
                        }
                    }
                }
                to[o] = sum / n * keep;
            }
        }
    }

    #[test]
    fn the_branchless_spread_matches_the_first_one_bit_for_bit() {
        // Random agar with holes (rock, walls), values on non-agar cells
        // too (a wall built over alarm keeps its value for a step), zeros,
        // tiny and large values, several bands.
        let mut z = 0x9E37_79B9_7F4A_7C15u64;
        let mut next = || {
            z ^= z << 13;
            z ^= z >> 7;
            z ^= z << 17;
            z
        };
        for &(w, h) in &[(3, 3), (5, 4), (37, 23), (128, 80)] {
            let agar: Vec<bool> = (0..w * h).map(|_| next() % 5 != 0).collect();
            let from: Vec<f32> = (0..w * h)
                .map(|_| match next() % 4 {
                    0 => 0.0,
                    1 => (next() % 1000) as f32 * 1e-7,
                    _ => (next() % 100_000) as f32 / 37.0,
                })
                .collect();
            for &keep in &[0.98f32, 0.95, 1.0] {
                let mut a = vec![1.0; w * h];
                let mut b = vec![2.0; w * h];
                spread_rows_reference(&agar, &from, &mut a, w, h, keep, 0);
                spread_rows(&agar, &from, &mut b, w, h, keep, 0);
                assert!(a.iter().zip(&b).all(|(x, y)| x.to_bits() == y.to_bits()), "{w}x{h} keep {keep}");
                for threads in [2, 3, 7] {
                    let mut c = vec![3.0; w * h];
                    spread(&agar, &from, &mut c, w, h, keep, threads);
                    assert!(a.iter().zip(&c).all(|(x, y)| x.to_bits() == y.to_bits()), "{w}x{h} keep {keep}, {threads} threads");
                }
            }
        }
    }

    #[test]
    fn respawn_keeps_walkers_and_the_crowd_rule() {
        let mut d = BigDish::new(BigParams { respawn: 0.2, blob: false, ..small(1, 1) });
        let n = d.walkers.len();
        let before = d.walkers.clone();
        for _ in 0..5 {
            d.tick();
        }
        assert_eq!(d.walkers.len(), n);
        let mut occ = vec![0u16; d.w * d.h];
        for &(x, y, _) in &d.walkers {
            let i = d.cell(x, y).unwrap();
            assert!(d.agar[i]);
            occ[i] += 1;
        }
        assert!(occ.iter().all(|&c| c <= 1));
        assert_eq!(occ, d.occ);
        // About a fifth of them jump each step: most end far from their start.
        let far = d.walkers.iter().zip(&before).filter(|(a, b)| (a.0 - b.0).hypot(a.1 - b.1) > 20.0).count();
        assert!(far > n / 2, "{far} of {n} moved far");
    }

    fn small(scale: usize, substeps: usize) -> BigParams {
        BigParams { scale, substeps, walkers: 600 * scale * scale, seed: 3, ..BigParams::today() }
    }

    /// Threads split the field passes by rows; the dish must not care.
    #[test]
    fn threads_change_nothing() {
        let mut a = BigDish::new(small(2, 2));
        let mut b = a.clone();
        b.threads = 4;
        for _ in 0..60 {
            a.tick();
            b.tick();
        }
        assert_eq!(a.hash(), b.hash());
        assert_eq!(a.hash(), {
            let mut c = BigDish::new(small(2, 2));
            for _ in 0..60 {
                c.tick();
            }
            c.hash()
        });
    }

    /// With a crowd limit of 1 no cell ever holds two walkers, and no claim
    /// is left behind.
    #[test]
    fn one_walker_a_cell() {
        let mut d = BigDish::new(small(2, 2));
        for _ in 0..120 {
            d.tick();
            let mut seen = vec![0u8; d.w * d.h];
            for &(x, y, _) in &d.walkers {
                let i = d.cell(x, y).unwrap();
                assert!(d.agar[i]);
                seen[i] += 1;
                assert!(seen[i] == 1, "two walkers in one cell");
            }
            assert!(d.occ.iter().zip(&seen).all(|(&o, &s)| o == s as u16));
            assert!(d.claim.iter().all(|&c| c == u32::MAX));
        }
    }

    /// The scaling options keep the dish deterministic whatever the threads.
    #[test]
    fn scaling_options_ignore_threads() {
        let p = BigParams { trail_passes: 2, sense_every: 2, scent_offset: 18.0, ..small(2, 2) };
        let mut a = BigDish::new(p);
        let mut b = a.clone();
        b.threads = 3;
        for _ in 0..40 {
            a.tick();
            b.tick();
        }
        assert_eq!(a.hash(), b.hash());
    }

    /// A scent reach of 0 means the trail sensor's reach, bit for bit.
    #[test]
    fn scent_offset_zero_is_the_sensor_offset() {
        let p = small(2, 2);
        let mut a = BigDish::new(p.clone());
        let mut b = BigDish::new(BigParams { scent_offset: p.sensor_offset, ..p });
        for _ in 0..30 {
            a.tick();
            b.tick();
        }
        assert_eq!(a.hash(), b.hash());
    }

    /// Between looks a walker keeps its heading; only a blocked walker,
    /// which stays put, draws a new one.
    #[test]
    fn walkers_walk_straight_between_looks() {
        // Spread start: a packed blob leaves most walkers blocked.
        let mut d = BigDish::new(BigParams { sense_every: 2, blob: false, ..small(2, 2) });
        for _ in 0..20 {
            d.tick();
        }
        assert_eq!(d.steps % 2, 0);
        d.steps += 1;
        let before = d.walkers.clone();
        d.move_walkers();
        let mut moved = 0;
        for (a, b) in before.iter().zip(&d.walkers) {
            if (a.0, a.1) != (b.0, b.1) {
                assert_eq!(a.2, b.2, "turned between looks");
                moved += 1;
            }
        }
        assert!(moved > before.len() / 2, "{moved} of {} moved", before.len());
    }

    /// The 2026-11-11 options (Jenson's looks) keep the dish deterministic
    /// whatever the threads, and a sickness seeded in the middle spreads.
    #[test]
    fn jenson_options_ignore_threads_and_spread() {
        let p = BigParams {
            crowd: 0,
            blob_r: 20.0,
            haze: 0.3,
            sense_scale: 0.05,
            step_mod: [1.0, 1.0],
            turn_mod: [-0.2, 2.0],
            deposit_cap: 50,
            sense_floor: 0.5,
            wander: 0.01,
            sick_steps: 60,
            immune_steps: 60,
            sick_catch: 0.2,
            sick_reach: 0.5,
            sick_turn: 2.0,
            sick_trail: 3.0,
            sick_seed_r: 6.0,
            ..small(2, 1)
        };
        let mut a = BigDish::new(p);
        let seeded = a.sick.iter().filter(|&&c| c > 0).count();
        assert!(seeded > 0);
        let mut b = a.clone();
        b.threads = 3;
        let mut ever = vec![false; a.walkers.len()];
        for _ in 0..60 {
            a.tick();
            b.tick();
            for (e, &c) in ever.iter_mut().zip(&a.sick) {
                *e |= c > a.p.immune_steps;
            }
        }
        assert_eq!(a.hash(), b.hash());
        let caught = ever.iter().filter(|&&e| e).count();
        assert!(caught > 2 * seeded, "{caught} caught from {seeded} seeded");
    }

    /// With every new option at its default the dish is today's, and the
    /// contagion state is not even allocated.
    #[test]
    fn new_options_default_off() {
        let d = BigDish::new(small(2, 2));
        assert!(d.charge.is_empty() && d.rock.is_empty() && d.took.is_empty());
        assert!(d.sick.is_empty() && d.germ.is_empty());
        assert!(d.species_trail.is_empty() && d.age.is_empty());
        assert!(d.kind.is_empty() && d.grass.is_empty() && d.p.species_sets.is_empty());
    }

    /// Species sets that copy the global fields change nothing: the same
    /// dish, bit for bit, as the global fields alone.
    #[test]
    fn species_sets_equal_to_globals_change_nothing() {
        for p in [
            BigParams { species: 2, species_zones: true, ..small(2, 1) },
            BigParams { species: 2, species_convert: 1.2, graze_weight: 10.0, explore_steps: 30, ..small(2, 1) },
            BigParams { species: 2, blocked_keep: true, blocked_retry: 2, blocked_deposit: 1.0, keep: 0.93, ..small(2, 1) },
            BigParams { species: 2, crowd: 0, deposit_cap: 4, ..small(2, 1) },
        ] {
            let same = SpeciesSet::from_params(&p);
            let mut a = BigDish::new(p.clone());
            let mut b = BigDish::new(BigParams { species_sets: vec![same; 2], ..p });
            for _ in 0..40 {
                a.tick();
                b.tick();
            }
            assert_eq!(a.hash(), b.hash());
        }
    }

    /// A droplet species (crowd 1, grazing, wide sensors) and a network
    /// species (crowd 0, explorers first) on one dish: only the grazing
    /// species eats grass, and only grazers' cells lose it.
    #[test]
    fn species_sets_differ_by_species() {
        let base = BigParams { species: 2, species_zones: true, ..small(2, 1) };
        let drop = SpeciesSet { crowd: 1, sensor_angle: 1.5, rotate_angle: 0.8, deposit: 5.0, keep: 0.93, blocked_deposit: 1.0, graze_weight: 10.0, explore_steps: 0, ..SpeciesSet::from_params(&base) };
        let net = SpeciesSet { crowd: 0, keep: 0.6, graze_weight: 0.0, explore_steps: 20, ..SpeciesSet::from_params(&base) };
        let mut d = BigDish::new(BigParams { species_sets: vec![drop, net], ..base });
        assert!(!d.grass.is_empty() && !d.age.is_empty());
        let mut been = vec![false; d.w * d.h];
        for _ in 0..30 {
            d.tick();
            for (k, &(x, y, _)) in d.walkers.iter().enumerate() {
                if d.spec[k] == 0 {
                    been[d.cell(x, y).unwrap()] = true;
                }
            }
        }
        let eaten: Vec<usize> = (0..d.w * d.h).filter(|&i| d.agar[i] && d.grass[i] < 1.0).collect();
        assert!(!eaten.is_empty());
        assert!(eaten.iter().all(|&i| been[i]), "grass eaten where no grazer stood");
        // No cell holds two walkers of the crowd-1 species.
        let mut n0 = vec![0u16; d.w * d.h];
        for (k, &(x, y, _)) in d.walkers.iter().enumerate() {
            if d.spec[k] == 0 {
                n0[d.cell(x, y).unwrap()] += 1;
            }
        }
        assert!(n0.iter().all(|&n| n <= 1));
    }

    /// Each species' field decays at its own keep.
    #[test]
    fn species_keep_is_per_species() {
        let base = BigParams { species: 2, ..small(2, 1) };
        let a = SpeciesSet { deposit: 0.0, keep: 0.9, ..SpeciesSet::from_params(&base) };
        let b = SpeciesSet { keep: 0.5, ..a };
        let mut d = BigDish::new(BigParams { species_sets: vec![a, b], respawn: 0.0, ..base });
        for i in 0..d.w * d.h {
            if d.agar[i] {
                d.species_trail[0][i] = 1.0;
                d.species_trail[1][i] = 1.0;
                d.trail[i] = 2.0;
            }
        }
        let before: f32 = d.species_trail[0].iter().sum();
        d.tick();
        let (s0, s1): (f32, f32) = (d.species_trail[0].iter().sum(), d.species_trail[1].iter().sum());
        assert!((s0 / before - 0.9).abs() < 0.01, "{}", s0 / before);
        assert!((s1 / before - 0.5).abs() < 0.01, "{}", s1 / before);
    }

    /// `species_convert_per` weighs trail over each species' deposit: a
    /// species laying three times as much no longer converts the other by
    /// that alone (raw trail converts nearly every walker of the other).
    #[test]
    fn conversion_per_deposit_counts_presence() {
        let ones = |per: bool| {
            let base = BigParams { species: 2, species_convert: 1.2, species_convert_r: 2, species_avoid: 0.0, ..small(2, 1) };
            let heavy = SpeciesSet { deposit: 3.0 * base.deposit, ..SpeciesSet::from_params(&base) };
            let p = BigParams { species_sets: vec![heavy, SpeciesSet::from_params(&base)], species_convert_per: per, ..base };
            let mut d = BigDish::new(p);
            for i in 0..d.w * d.h {
                if d.agar[i] {
                    d.species_trail[0][i] = 3.0;
                    d.species_trail[1][i] = 1.0;
                    d.trail[i] = 4.0;
                }
            }
            let n1 = |d: &BigDish| d.spec.iter().filter(|&&s| s == 1).count();
            let before = n1(&d);
            d.tick();
            (before, n1(&d))
        };
        let (b, raw) = ones(false);
        let (_, per) = ones(true);
        assert!(raw * 10 < b, "raw trail: {raw} of {b} species-1 walkers left");
        assert!(per * 10 > 9 * b, "per deposit: {per} of {b} left");
    }

    /// The outside rules (2026-11-11) are keyed draws too: threads change
    /// nothing, and the species fields add up to the shared trail.
    #[test]
    fn outside_rules_ignore_threads() {
        for p in [
            BigParams { sensor_r: 1, chance_exp: 4.0, size_spread: 0.6, ..small(2, 1) },
            BigParams { species: 2, species_zones: true, explore_steps: 30, ..small(2, 1) },
            BigParams { species: 3, species_cycle: true, species_convert: 1.0, ..small(2, 1) },
            BigParams { species: 2, species_convert: 1.5, species_boost: 1.5, graze_weight: 10.0, ..small(2, 1) },
            BigParams { species: 2, species_zones: true, species_convert: 1.2, species_convert_r: 3, species_share: 0.6, ..small(2, 1) },
            BigParams { mix_share: 0.3, mix_reach: 3.0, mix_angle: 0.5, mix_turn: 2.0, mix_step: 1.5, mix_deposit: 0.5, ..small(2, 1) },
            BigParams { blocked_keep: true, blocked_reset: 0.1, blocked_retry: 3, blocked_deposit: 1.0, ..small(2, 1) },
            BigParams { explore_steps: 30, explore_spread: 1.0, ..small(2, 1) },
            {
                let base = BigParams { species: 2, species_zones: true, species_convert: 1.2, ..small(2, 1) };
                let drop = SpeciesSet { crowd: 1, sensor_angle: 1.5, deposit: 5.0, blocked_deposit: 1.0, graze_weight: 10.0, ..SpeciesSet::from_params(&base) };
                let net = SpeciesSet { crowd: 0, keep: 0.8, explore_steps: 20, ..SpeciesSet::from_params(&base) };
                BigParams { species_sets: vec![drop, net], blocked_retry: 2, ..base }
            },
        ] {
            let mut a = BigDish::new(p);
            let mut b = a.clone();
            b.threads = 3;
            for _ in 0..40 {
                a.tick();
                b.tick();
            }
            assert_eq!(a.hash(), b.hash());
            let n = a.p.species as usize;
            assert_eq!(a.species_trail.len(), if n > 1 { n } else { 0 });
            if n > 1 {
                for (i, &t) in a.trail.iter().enumerate() {
                    let sum: f32 = a.species_trail.iter().map(|f| f[i]).sum();
                    assert!((t - sum).abs() <= 1e-3 * t.abs().max(1.0), "cell {i}: {t} vs {sum}");
                }
            }
        }
    }

    /// Growth (Larder World W0): threads change nothing, a cell's count is
    /// its living walkers, the crowd limit holds through births, and food
    /// decides the size: with the grass regrowing the mould outgrows its
    /// start, with none it eats the dish bare and dies down.
    #[test]
    fn growth_follows_food() {
        let grow = |regrow: f32| BigParams { upkeep: 0.01, capacity: 4 * 600, graze_weight: 10.0, graze_regrow: regrow, ..small(1, 1) };
        let mut a = BigDish::new(grow(0.002));
        let mut b = a.clone();
        b.threads = 3;
        let start = a.measure().alive;
        assert_eq!((start, a.walkers.len()), (600, 2400));
        let mut peak = 0;
        for _ in 0..600 {
            a.tick();
            b.tick();
            peak = peak.max(a.measure().alive);
        }
        assert_eq!(a.hash(), b.hash());
        let mut occ = vec![0u16; a.w * a.h];
        for k in 0..a.walkers.len() {
            if a.lives(k) {
                let (x, y, _) = a.walkers[k];
                let i = a.cell(x, y).unwrap();
                assert!(a.agar[i]);
                occ[i] += 1;
            }
        }
        assert!(occ.iter().all(|&c| c <= 1));
        assert_eq!(occ, a.occ);
        let fed = a.measure().alive;
        assert!(a.births > 0 && a.starved > 0 && fed > 2 * start, "fed: {fed} alive of {start}, peak {peak}");
        // The dish holds about 30,000 grass; 2,400 walkers burn 24 a step.
        let mut c = BigDish::new(grow(0.0));
        for _ in 0..3000 {
            c.tick();
        }
        let starved = c.measure().alive;
        assert!(starved < start / 2, "starved: {starved} alive of {start}");
    }

    /// Growth on Larder's crystals: a walker's crystal take (`took`) is its
    /// food, so with no births and no starving the energy gained is the
    /// charge eaten less upkeep; an empty slot takes nothing; and with
    /// crystals on there is no grass unless `graze_weight` asks for it.
    #[test]
    fn growth_eats_crystals() {
        let upkeep = 1e-4;
        let p = BigParams { crystal_charge: 2.0, crystal_eat: 0.05, upkeep, birth_energy: 1e9, starve: 0.0, capacity: 1200, ..small(1, 1) };
        let mut d = BigDish::new(p.clone());
        assert!(d.grass.is_empty(), "no grass with crystals");
        // Park the empty slots on the home crystal, where a share is there
        // to take.
        let (px, py) = d.piles[0];
        for k in 600..1200 {
            d.walkers[k] = (px, py, 0.0);
        }
        let alive = d.measure().alive;
        assert_eq!(alive, 600);
        for _ in 0..20 {
            let (e0, eaten0): (f64, f64) = (d.energy.iter().map(|&e| e as f64).sum(), d.eaten);
            d.tick();
            let e1: f64 = d.energy.iter().map(|&e| e as f64).sum();
            let want = (d.eaten - eaten0) - upkeep as f64 * alive as f64;
            assert!((e1 - e0 - want).abs() < 1e-3, "gained {} against eaten less upkeep {want}", e1 - e0);
            assert!((600..1200).all(|k| d.took[k] == 0.0), "an empty slot took a share");
        }
        assert!(d.eaten > 0.0, "the home crystal fed the mould");
        assert!(!BigDish::new(BigParams { graze_weight: 10.0, ..p }).grass.is_empty());
    }

    /// `energy_flow` keeps energy (pairs give what they take), evens it out
    /// and leaves empty slots alone.
    #[test]
    fn energy_flow_keeps_and_evens() {
        let p = BigParams { upkeep: 0.01, energy_flow: 0.1, capacity: 1200, ..small(1, 1) };
        let mut d = BigDish::new(p);
        for _ in 0..30 {
            d.tick();
        }
        for k in 0..d.walkers.len() {
            if d.lives(k) {
                d.energy[k] = (k % 7) as f32 * 0.3;
            }
        }
        let empty: Vec<f32> = (0..d.walkers.len()).filter(|&k| !d.lives(k)).map(|k| d.energy[k]).collect();
        let spread = |d: &BigDish| {
            let e: Vec<f64> = (0..d.walkers.len()).filter(|&k| d.lives(k)).map(|k| d.energy[k] as f64).collect();
            let m = e.iter().sum::<f64>() / e.len() as f64;
            (e.iter().sum::<f64>(), e.iter().map(|x| (x - m) * (x - m)).sum::<f64>())
        };
        let (sum0, var0) = spread(&d);
        d.flow_energy();
        let (sum1, var1) = spread(&d);
        assert!((sum1 - sum0).abs() < 1e-3, "energy kept: {sum0} -> {sum1}");
        assert!(var1 < 0.9 * var0, "energy evens out: {var0} -> {var1}");
        let after: Vec<f32> = (0..d.walkers.len()).filter(|&k| !d.lives(k)).map(|k| d.energy[k]).collect();
        assert_eq!(empty, after, "empty slots untouched");
    }

    /// The one-body rules (hunger, body pull, tether, lone upkeep): threads
    /// change nothing, the crowd limit holds, and each one changes the dish
    /// (so it is wired in), while left at its default it changes nothing.
    #[test]
    fn one_body_rules_ignore_threads_and_act() {
        let grow = BigParams { upkeep: 0.01, capacity: 1200, crystal_charge: 64.0, crystal_eat: 0.02, ..small(2, 1) };
        let base_hash = {
            let mut d = BigDish::new(grow.clone());
            for _ in 0..40 {
                d.tick();
            }
            d.hash()
        };
        for p in [
            BigParams { hunger_scent: 6.0, hunger_at: 1.0, energy_flow: 0.05, ..grow.clone() },
            BigParams { body_pull: 3.0, body_r: 2, body_lone: 6.0, ..grow.clone() },
            BigParams { tether: 2, ..grow.clone() },
            BigParams { tether: 1, tether_hold: 0.3, ..grow.clone() },
            BigParams { lone_upkeep: 5.0, lone_r: 2, ..grow.clone() },
            BigParams { hunger_reach: 3.0, hunt_at: 0.3, tether: 1, ..grow.clone() },
        ] {
            let mut a = BigDish::new(p);
            let mut b = a.clone();
            b.threads = 3;
            for _ in 0..40 {
                a.tick();
                b.tick();
            }
            assert_eq!(a.hash(), b.hash());
            assert_ne!(a.hash(), base_hash, "the rule changes the dish: {:?}", (a.p.hunger_scent, a.p.body_pull, a.p.tether, a.p.lone_upkeep));
            for (i, &n) in a.occupancy().iter().enumerate() {
                assert!(n <= 1, "cell {i} holds {n}");
            }
        }
    }

    /// Tether with holders: a mover always lands touching a walker that
    /// stays, so far fewer walkers lose touch with the crowd in a step than
    /// without it (a holder can still be left alone when all round it move).
    #[test]
    fn tether_keeps_contact() {
        let lost = |tether: u8| {
            let mut d = BigDish::new(BigParams { tether, tether_hold: 0.5, walkers: 600, ..small(2, 1) });
            for _ in 0..20 {
                d.tick();
            }
            let mut lost = 0;
            for _ in 0..20 {
                let touch = |d: &BigDish| -> Vec<bool> {
                    (0..d.walkers.len())
                        .map(|k| {
                            let (x, y, _) = d.walkers[k];
                            d.occ_round(y as usize * d.w + x as usize, 1) > 0
                        })
                        .collect()
                };
                let before = touch(&d);
                d.tick();
                let after = touch(&d);
                lost += before.iter().zip(&after).filter(|(b, a)| **b && !**a).count();
            }
            lost
        };
        let (on, off) = (lost(1), lost(0));
        assert!(on * 2 < off, "lost touch: {on} tethered against {off} free");
    }

    /// With growth off nothing is kept for it: every slot lives.
    #[test]
    fn growth_off_keeps_no_slots() {
        let d = BigDish::new(small(1, 1));
        assert!(d.alive.is_empty() && d.energy.is_empty() && d.grass.is_empty());
        assert_eq!(d.measure().alive, d.walkers.len());
    }

    /// The life cycle (2026-11-11): threads change nothing; the crowd rule
    /// and `occ` hold with spores in flight, the dormant and the dead; spores
    /// leave, fly and land; the dormant do not move; a strike kills and
    /// frees cells; and with every rule off nothing is allocated.
    #[test]
    fn life_cycle_keeps_the_crowd_rule_and_ignores_threads() {
        let d = BigDish::new(small(2, 1));
        assert!(d.life.is_empty() && d.life_t.is_empty() && d.alarm.is_empty());
        for p in [
            BigParams { spore_rate: 0.01, spore_scent: -1.0, spore_steps: 20, spore_reach: 8.0, spore_land: 1.0, spore_deposit: 0.2, spore_rest: 10, ..small(2, 1) },
            BigParams { sleep_rate: 0.05, wake_scent: 50.0, alarm_keep: 0.95, alarm_relay: 20.0, wake_alarm: 1.0, alarm_weight: 2.0, sleep_rest: 10, wake_steps: 10, ..small(2, 1) },
            BigParams { spore_burst: 0.5, spore_alarm: 1.0, spore_steps: 20, alarm_keep: 0.98, alarm_weight: -5.0, ..small(2, 1) },
        ] {
            let mut a = BigDish::new(p);
            let mut b = a.clone();
            b.threads = 3;
            let mut flew = 0;
            let mut landed = 0;
            for t in 0..60 {
                if t == 20 {
                    let (x, y) = (a.walkers[0].0 / 2.0, a.walkers[0].1 / 2.0);
                    let ka = a.strike(x, y, 4.0, 30.0, 8.0);
                    let kb = b.strike(x, y, 4.0, 30.0, 8.0);
                    assert_eq!(ka, kb);
                    assert!(ka > 0, "the strike killed nothing");
                }
                let before = a.life.clone();
                let pos = a.walkers.clone();
                a.tick();
                b.tick();
                for k in 0..a.walkers.len() {
                    if before[k] == LIFE_DORMANT && a.life[k] == LIFE_DORMANT {
                        assert_eq!((pos[k].0, pos[k].1), (a.walkers[k].0, a.walkers[k].1), "a dormant walker moved");
                    }
                    if before[k] == LIFE_DEAD {
                        assert_eq!(a.life[k], LIFE_DEAD, "the dead came back");
                    }
                    flew += usize::from(before[k] != LIFE_SPORE && a.life[k] == LIFE_SPORE);
                    landed += usize::from(before[k] == LIFE_SPORE && a.life[k] == LIFE_WALKER);
                }
                let mut occ = vec![0u16; a.w * a.h];
                for (k, &(x, y, _)) in a.walkers.iter().enumerate() {
                    if matches!(a.life[k], LIFE_WALKER | LIFE_DORMANT) {
                        let i = a.cell(x, y).unwrap();
                        assert!(a.agar[i]);
                        occ[i] += 1;
                    }
                }
                assert_eq!(occ, a.occ);
                assert!(occ.iter().all(|&c| c <= 1));
                assert!(a.claim.iter().all(|&c| c == u32::MAX));
            }
            assert_eq!(a.hash(), b.hash());
            if a.p.spore_rate > 0.0 || a.p.spore_burst > 0.0 {
                assert!(flew > 0 && landed > 0, "{flew} flew, {landed} landed");
            }
            if a.p.sleep_rate > 0.0 {
                assert!(a.life_counts()[2] > 0, "nobody fell dormant");
            }
        }
    }

    /// Growth with the life cycle (merged 2026-11-11): threads change
    /// nothing; `occ` counts the living ordinary and dormant walkers; a
    /// spore or a dormant walker keeps its energy (no upkeep, no grass); a
    /// strike frees its slots, so the dead count stays 0 and no living
    /// slot is dead; and births go on after it, some into struck slots.
    #[test]
    fn growth_with_the_life_cycle() {
        let p = BigParams {
            upkeep: 0.01,
            capacity: 4 * 600,
            graze_weight: 10.0,
            graze_regrow: 0.002,
            spore_rate: 0.01,
            spore_scent: -1.0,
            spore_steps: 20,
            spore_reach: 8.0,
            spore_land: 1.0,
            spore_rest: 10,
            sleep_rate: 0.05,
            wake_scent: 50.0,
            sleep_rest: 10,
            wake_steps: 10,
            ..small(1, 1)
        };
        let mut a = BigDish::new(p);
        let mut b = a.clone();
        b.threads = 3;
        let mut struck = Vec::new();
        let (mut paused, mut reused, mut births_after) = (0usize, 0usize, 0u64);
        for t in 0..300 {
            if t == 100 {
                let before: Vec<bool> = (0..a.walkers.len()).map(|k| a.lives(k)).collect();
                let (x, y) = (a.walkers[0].0, a.walkers[0].1);
                let ka = a.strike(x, y, 6.0, 0.0, 0.0);
                assert_eq!(ka, b.strike(x, y, 6.0, 0.0, 0.0));
                assert!(ka > 0, "the strike killed nothing");
                struck = (0..a.walkers.len()).filter(|&k| before[k] && !a.lives(k)).collect();
                assert_eq!(struck.len(), ka);
                births_after = a.births;
            }
            let (life, energy) = (a.life.clone(), a.energy.clone());
            a.tick();
            b.tick();
            for k in 0..a.walkers.len() {
                if a.lives(k) && matches!(life[k], LIFE_SPORE | LIFE_DORMANT) && a.life[k] == life[k] {
                    assert_eq!(a.energy[k], energy[k], "a spore or a sleeper paid upkeep or ate");
                    paused += 1;
                }
                assert!(!a.lives(k) || a.life[k] != LIFE_DEAD, "a living slot is dead");
            }
            let mut occ = vec![0u16; a.w * a.h];
            for (k, &(x, y, _)) in a.walkers.iter().enumerate() {
                if a.lives(k) && matches!(a.life[k], LIFE_WALKER | LIFE_DORMANT) {
                    let i = a.cell(x, y).unwrap();
                    assert!(a.agar[i]);
                    occ[i] += 1;
                }
            }
            assert_eq!(occ, a.occ);
            assert!(occ.iter().all(|&c| c <= 1));
        }
        reused += struck.iter().filter(|&&k| a.lives(k)).count();
        assert_eq!(a.hash(), b.hash());
        assert_eq!(a.life_counts()[3], 0);
        assert!(paused > 0, "no spore or sleeper was seen");
        assert!(a.births > births_after && reused > 0, "{} births after the strike, {reused} struck slots reused", a.births - births_after);
    }

    /// The life cycle with Larder World's crystals and walls (merged
    /// 2026-11-11): a wall built over spores, sleepers and the dead keeps
    /// `occ` true, nobody lands on a wall, only ordinary walkers eat, and the
    /// eaten total still matches what left the piles.
    #[test]
    fn life_cycle_with_walls_and_crystals() {
        let p = BigParams {
            crystal_charge: 2.0,
            crystal_eat: 0.2,
            spore_rate: 0.02,
            spore_scent: -1.0,
            spore_steps: 20,
            spore_reach: 8.0,
            spore_land: 1.0,
            sleep_rate: 0.05,
            wake_scent: 50.0,
            alarm_keep: 0.95,
            alarm_relay: 20.0,
            sleep_rest: 10,
            wake_steps: 10,
            ..small(2, 2)
        };
        let mut d = BigDish::new(p);
        let full: f32 = d.pile_full.iter().sum();
        for t in 0..60 {
            if t == 15 {
                let (hx, hy) = d.piles[0];
                assert!(d.build_wall(hx - 3.0, hy - 10.0, hx - 3.0, hy + 10.0, 2.0) > 0);
            }
            if t == 25 {
                let (hx, hy) = d.piles[0];
                d.strike(hx, hy, 3.0, 30.0, 6.0);
            }
            d.tick();
            let mut occ = vec![0u16; d.w * d.h];
            for (k, &(x, y, _)) in d.walkers.iter().enumerate() {
                if matches!(d.life[k], LIFE_WALKER | LIFE_DORMANT) {
                    let i = d.cell(x, y).unwrap();
                    assert!(d.agar[i], "a grounded walker on a wall or rim");
                    occ[i] += 1;
                }
                if !matches!(d.life[k], LIFE_WALKER) {
                    assert_eq!(d.took[k], 0.0, "only ordinary walkers eat");
                }
            }
            assert_eq!(occ, d.occ);
        }
        let counts = d.life_counts();
        assert!(counts[1] + counts[2] + counts[3] > 0, "{counts:?}");
        let gone = full - d.pile_charge.iter().sum::<f32>();
        assert!((gone as f64 - d.eaten).abs() < 1e-2 * full as f64);
    }

    /// Crystals run dry: charge only falls, the eaten total matches what
    /// left the piles, and a drained pile lays less scent. Terrain is the
    /// same map at every scale and keeps rock off the piles.
    /// Soil (Larder World maps): sand thins the trail, clay keeps it, and
    /// barren ground costs upkeep and takes no births; an empty soil field
    /// changes nothing.
    #[test]
    fn soil_kinds_act_on_their_terms() {
        let p = BigParams { upkeep: 0.001, capacity: 1200, ..small(1, 1) };
        let run = |kind: u8| {
            let mut d = BigDish::new(p.clone());
            if kind > 0 {
                d.soil = vec![kind; d.w * d.h];
            }
            for _ in 0..30 {
                d.tick();
            }
            (d.trail.iter().sum::<f32>(), d.alive.iter().filter(|&&a| a).count(), d.hash())
        };
        let (plain, sand, clay, barren) = (run(0), run(SOIL_SAND), run(SOIL_CLAY), run(SOIL_BARREN));
        assert!(sand.0 < plain.0 * 0.5, "sand {} plain {}", sand.0, plain.0);
        assert!(clay.0 > plain.0, "clay {} plain {}", clay.0, plain.0);
        assert!(barren.1 <= plain.1 && barren.2 != plain.2);
        // Soil all plain (0) is the same dish as no soil field.
        let mut d = BigDish::new(p.clone());
        d.soil = vec![0; d.w * d.h];
        for _ in 0..30 {
            d.tick();
        }
        assert_eq!(d.hash(), plain.2);
    }

    #[test]
    fn crystals_run_dry_and_terrain_pairs() {
        let p = BigParams { crystal_charge: 2.0, crystal_eat: 0.2, terrain_rock: 0.3, terrain_wet: 0.3, rect: true, ..small(2, 2) };
        let mut d = BigDish::new(p.clone());
        let full: f32 = d.pile_full.iter().sum();
        assert!(full > 0.0 && d.crystals_left() == 1.0);
        let mut last = 1.0;
        for _ in 0..60 {
            d.tick();
            let left = d.crystals_left();
            assert!(left <= last + 1e-6);
            last = left;
        }
        assert!(last < 1.0, "the home crystal is eaten");
        let gone = full - d.pile_charge.iter().sum::<f32>();
        assert!((gone as f64 - d.eaten).abs() < 1e-2 * full as f64);
        assert!(d.crystal_left(0) < 1.0);
        let mut t = BigDish::new(p.clone());
        t.threads = 4;
        let mut e = BigDish::new(p.clone());
        for _ in 0..10 {
            t.tick();
            e.tick();
        }
        assert_eq!(t.hash(), e.hash(), "threads change nothing");
        // The terrain map pairs across scales: rock at the same dish cells.
        let (a, b) = (BigDish::new(BigParams { scale: 1, substeps: 1, ..p.clone() }), BigDish::new(BigParams { scale: 2, ..p.clone() }));
        let mut same = 0;
        for y in 0..a.h {
            for x in 0..a.w {
                if a.rock[y * a.w + x] == b.rock[(2 * y) * b.w + 2 * x] {
                    same += 1;
                }
            }
        }
        assert!(same as f32 > 0.97 * (a.w * a.h) as f32);
        assert!(a.rock.iter().any(|&r| r) && a.wet.iter().any(|&w| w));
        // Crystals that refill end fuller than ones that run dry.
        let mut r = BigDish::new(BigParams { crystal_regrow: 0.05, ..p.clone() });
        let mut n = BigDish::new(p.clone());
        for _ in 0..60 {
            r.tick();
            n.tick();
        }
        assert!(r.crystals_left() > n.crystals_left());
        // A wall leaves the agar, clears its trail and moves walkers out.
        let mut d = BigDish::new(BigParams { crystal_charge: 0.0, terrain_rock: 0.0, terrain_wet: 0.0, wall_hp: 0.0, ..p.clone() });
        for _ in 0..20 {
            d.tick();
        }
        let (hx, hy) = d.piles[0];
        let n = d.build_wall(hx - 3.0, hy - 10.0, hx - 3.0, hy + 10.0, 2.0);
        assert!(n > 0);
        for (i, &wl) in d.wall.iter().enumerate() {
            if wl {
                assert!(!d.agar[i] && d.trail[i] == 0.0);
            }
        }
        let inside = d.walkers.iter().filter(|&&(x, y, _)| d.wall[y as usize * d.w + x as usize]).count();
        assert!(inside < 5, "{inside} walkers left inside the wall");
        for _ in 0..30 {
            d.tick();
        }
        for (i, &wl) in d.wall.iter().enumerate() {
            if wl {
                assert!(d.trail[i] == 0.0, "no trail enters a wall");
            }
        }
        assert_eq!(d.broken, 0, "walls without health never break");
        // With health, the crowd pressing on a wall wears it through.
        let mut b = BigDish::new(BigParams { crystal_charge: 0.0, terrain_rock: 0.0, terrain_wet: 0.0, wall_hp: 2.0, wall_bite: 1.0, ..p.clone() });
        for _ in 0..20 {
            b.tick();
        }
        let (hx, hy) = b.piles[0];
        b.build_wall(hx - 3.0, hy - 10.0, hx - 3.0, hy + 10.0, 2.0);
        for _ in 0..60 {
            b.tick();
        }
        assert!(b.broken > 0, "the mould breaks a weak wall");
        for (i, &wl) in b.wall.iter().enumerate() {
            if !wl && b.wall_health[i] == 0.0 && b.wet[i] {
                assert!(b.agar[i], "rubble is ground again");
            }
        }
        for &(px, py) in &a.piles {
            assert!(!a.rock[py as usize * a.w + px as usize]);
        }
    }

    /// The same layout of piles at every scale, so arms pair.
    #[test]
    fn crystal_flow_keeps_the_total_and_evens_a_disc() {
        // Pile 1 with half its cells emptied by hand, and no eating: without
        // flow it stays half empty, with flow the cells even out and still
        // add up to the pile's charge.
        let p = BigParams { crystal_charge: 2.0, crystal_eat: 0.0, crystal_flow: 0.0, ..small(2, 2) };
        let spread = |d: &BigDish| {
            let cells: Vec<f32> = d.pile_cells.iter().filter(|&&i| d.pile_of[i as usize] == 2).map(|&i| d.charge[i as usize]).collect();
            let (lo, hi) = cells.iter().fold((f32::MAX, f32::MIN), |(lo, hi), &c| (lo.min(c), hi.max(c)));
            (hi - lo, cells.iter().sum::<f32>())
        };
        let half = |d: &mut BigDish| {
            let cells: Vec<u32> = d.pile_cells.iter().copied().filter(|&i| d.pile_of[i as usize] == 2).collect();
            for &i in &cells[..cells.len() / 2] {
                d.pile_charge[1] -= d.charge[i as usize];
                d.charge[i as usize] = 0.0;
            }
        };
        let mut still = BigDish::new(p.clone());
        let mut flow = BigDish::new(BigParams { crystal_flow: 0.2, ..p });
        half(&mut still);
        half(&mut flow);
        for _ in 0..30 {
            still.tick();
            flow.tick();
        }
        let ((s0, _), (s1, sum1)) = (spread(&still), spread(&flow));
        assert!(s0 > 1.9, "without flow the disc stays uneven ({s0})");
        assert!(s1 < 0.01, "flow evens it ({s1})");
        assert!((sum1 - flow.pile_charge[1]).abs() < 1e-3 * flow.pile_full[1], "the total is kept");
        assert!((flow.crystal_left(1) - 0.5).abs() < 0.05);
    }

    #[test]
    fn crystals_are_hexagons() {
        // Pointy-top, circumradius pile_r: the top vertex is in, the same
        // distance out sideways is not (a flat side is at 0.87 r), and the
        // area is the hexagon's, not the disc's.
        let d = BigDish::new(BigParams { crystal_charge: 1.0, terrain_rock: 0.0, ..small(2, 2) });
        let s = d.p.scale as f32;
        let r = d.p.pile_r * s;
        let (cx, cy) = (d.piles[1].0 * s, d.piles[1].1 * s);
        let at = |x: f32, y: f32| d.pile_of[d.cell(x, y).unwrap()] == 2;
        assert!(at(cx, cy - r + 1.0) && at(cx, cy + r - 1.0), "top and bottom vertices");
        assert!(!at(cx + 0.95 * r, cy) && !at(cx - 0.95 * r, cy), "outside the flat sides");
        assert!(at(cx + 0.8 * r, cy));
        let cells = d.pile_of.iter().filter(|&&p| p == 2).count() as f32;
        let hex = 2.598 * r * r;
        assert!((cells - hex).abs() < 0.1 * hex, "{cells} cells against a hexagon's {hex}");
    }

    #[test]
    fn deep_crystals_still_count_down() {
        // Charge 512 a cell: a pile holds far more than a bite's f32 step,
        // and its total must still fall as the walkers eat (a running f32
        // total stood still here).
        let mut d = BigDish::new(BigParams { crystal_charge: 512.0, crystal_eat: 0.005, ..small(2, 2) });
        let full: f64 = d.pile_full.iter().map(|&v| v as f64).sum();
        for _ in 0..60 {
            d.tick();
        }
        let gone = full - d.pile_charge.iter().map(|&v| v as f64).sum::<f64>();
        assert!(d.eaten > 0.0 && d.crystal_left(0) < 1.0);
        assert!((gone - d.eaten).abs() < 0.01 * d.eaten, "gone {gone} eaten {}", d.eaten);
    }

    #[test]
    fn leaky_walls_let_a_little_scent_across() {
        // A ring round pile 1 (dish cells; the scent grid is today's).
        // Outside it, 3 cells past the ring on the far side from home, a
        // sealed ring leaves less scent than a leaky one, and the leaky
        // one less than no ring.
        let p = BigParams { walkers: 50, ..small(2, 2) };
        let at = |ring: Option<f32>| {
            let mut d = BigDish::new(BigParams { wall_scent: ring.unwrap_or(0.0), ..p.clone() });
            let ((hx, hy), (cx, cy)) = (d.piles[0], d.piles[1]);
            let r = d.p.pile_r + 3.0;
            if ring.is_some() {
                for q in 0..24 {
                    let (a0, a1) = (q as f32 / 24.0 * std::f32::consts::TAU, (q + 1) as f32 / 24.0 * std::f32::consts::TAU);
                    d.build_wall(cx + r * a0.cos(), cy + r * a0.sin(), cx + r * a1.cos(), cy + r * a1.sin(), 2.0);
                }
            }
            for _ in 0..200 {
                d.tick();
            }
            let a = (cy - hy).atan2(cx - hx);
            let (x, y) = (cx + (r + 3.0) * a.cos(), cy + (r + 3.0) * a.sin());
            d.scent[y as usize * d.sw + x as usize]
        };
        let (open, leaky, sealed) = (at(None), at(Some(0.3)), at(Some(0.0)));
        assert!(sealed < leaky && leaky < open, "sealed {sealed} leaky {leaky} open {open}");
    }

    #[test]
    fn piles_pair_across_scales() {
        let a = BigDish::new(small(1, 1));
        let b = BigDish::new(small(4, 4));
        assert_eq!(a.piles, b.piles);
        assert!(a.piles.len() >= 5);
    }

    /// On today's grid the network grows out from home and reaches food.
    #[test]
    fn the_mould_spreads_and_finds_food() {
        let mut d = BigDish::new(BigParams { seed: 2, ..BigParams::today() });
        for _ in 0..30 * STEPS_PER_SECOND {
            d.tick();
        }
        let m = d.measure();
        assert!(m.tube_area > 500.0, "{m:?}");
        assert!(m.reached >= 2, "{m:?}");
    }
}
