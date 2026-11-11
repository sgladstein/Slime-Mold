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
const DRAW_TERRAIN: u32 = 140;

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
    /// Species 0 lays this many times the deposit (1 is even; added
    /// 2026-11-11 to give one side an edge in a fight).
    pub species_boost: f32,
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
    /// Energy that flows through the mould (2026-11-11, scratch, after the
    /// Larder thread's sizing runs; 0 is off). Each step after eating, the
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
    pub energy_flow: f32,
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
            sensor_r: 0,
            species: 0,
            species_avoid: 1.0,
            species_convert: 0.0,
            species_boost: 1.0,
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
            tube: 1.0,
            crystal_charge: 0.0,
            crystal_eat: 0.05,
            crystal_regrow: 0.0,
            crystal_flow: 0.01,
            terrain_rock: 0.0,
            terrain_wet: 0.0,
            wet_step: 0.5,
            terrain_blob: 12.0,
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
    /// Grass (`graze_weight` > 0, or growth on) a cell, else empty.
    pub grass: Vec<f32>,
    /// Growth (`upkeep` > 0): whether each slot holds a living walker and
    /// its energy; both empty with growth off, when every walker lives. A
    /// dead slot is left as it was and skipped by every rule.
    pub alive: Vec<bool>,
    pub energy: Vec<f32>,
    /// Births and starvings so far (growth on).
    pub births: u64,
    pub starved: u64,
    /// Pile centres in dish cells, and whether the tube has reached each.
    pub piles: Vec<(f32, f32)>,
    pub reached: Vec<bool>,
    /// Crystals (`crystal_charge` > 0), else empty: charge a fine cell,
    /// which pile owns it (index + 1, 0 none), each pile's charge left and
    /// at the start, each walker's take this fine step, and all eaten.
    pub charge: Vec<f32>,
    pub pile_of: Vec<u16>,
    pub pile_charge: Vec<f32>,
    pub pile_full: Vec<f32>,
    /// The fine cells of every pile's crystal, pile by pile (a wall can
    /// take one out of its pile: its `pile_of` is then 0).
    pile_cells: Vec<u32>,
    pub took: Vec<f32>,
    pub eaten: f64,
    /// Terrain (`terrain_rock` or `terrain_wet` > 0), else empty: rock and
    /// wet a fine cell.
    pub rock: Vec<bool>,
    pub wet: Vec<bool>,
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
fn spread_rows(agar: &[bool], from: &[f32], to: &mut [f32], w: usize, h: usize, keep: f32, y0: usize) {
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
            grass: Vec::new(),
            alive: Vec::new(),
            energy: Vec::new(),
            births: 0,
            starved: 0,
            piles: Vec::new(),
            reached: Vec::new(),
            charge: Vec::new(),
            pile_of: Vec::new(),
            pile_charge: Vec::new(),
            pile_full: Vec::new(),
            pile_cells: Vec::new(),
            took: Vec::new(),
            eaten: 0.0,
            rock: Vec::new(),
            wet: Vec::new(),
            wall: Vec::new(),
            wall_health: Vec::new(),
            broken: 0,
            wall_hits: 0,
            scent_wall: Vec::new(),
            threads: 1,
            p,
        };
        d.place_piles();
        d.place_terrain();
        d.place_crystals();
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
        if d.p.explore_steps > 0 {
            d.age = vec![0; d.walkers.len()];
        }
        // Growth eats grass unless the world feeds it crystals instead
        // (with crystals on, grass only if `graze_weight` asks for it).
        if d.p.graze_weight != 0.0 || (d.p.upkeep > 0.0 && d.p.crystal_charge <= 0.0) {
            d.grass = d.agar.iter().map(|&a| if a { 1.0 } else { 0.0 }).collect();
        }
        if d.p.species > 1 {
            d.species_trail = vec![vec![0.0; w * h]; d.p.species as usize];
            let n = d.p.species as usize;
            d.spec = (0..d.walkers.len()).map(|k| (k % n) as u8).collect();
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
    }

    /// Terrain (`terrain_rock`, `terrain_wet`): rock leaves the agar, on
    /// the fine grid and the scent grid alike; pile discs stay clear.
    fn place_terrain(&mut self) {
        let (rock_t, wet_t) = (self.p.terrain_rock, self.p.terrain_wet);
        if rock_t <= 0.0 && wet_t <= 0.0 {
            return;
        }
        let (seed, cell, s) = (self.p.seed, self.p.terrain_blob.max(1.0), self.p.scale as f32);
        let clear = self.p.pile_r + 4.0;
        let piles = self.piles.clone();
        let is_rock = |x: f32, y: f32| {
            rock_t > 0.0 && noise(seed, 0, x, y, cell) < rock_t && piles.iter().all(|&(px, py)| (px - x).hypot(py - y) > clear)
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
        if self.p.wall_hp > 0.0 && self.wall_health.is_empty() {
            self.wall_health = vec![0.0; w * h];
        }
        for &i in &cells {
            self.agar[i] = false;
            self.wall[i] = true;
            if !self.wall_health.is_empty() {
                self.wall_health[i] = self.p.wall_hp;
            }
            self.trail[i] = 0.0;
            for f in self.species_trail.iter_mut() {
                f[i] = 0.0;
            }
            if !self.charge.is_empty() && self.pile_of[i] > 0 {
                self.pile_charge[self.pile_of[i] as usize - 1] -= self.charge[i];
                self.pile_full[self.pile_of[i] as usize - 1] -= self.p.crystal_charge;
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
            if self.agar[i] {
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
        let r = self.p.pile_r * s;
        let n = r.ceil() as i32;
        self.charge = vec![0.0; self.w * self.h];
        self.pile_of = vec![0; self.w * self.h];
        self.pile_full = vec![0.0; self.piles.len()];
        self.pile_cells.clear();
        for pi in 0..self.piles.len() {
            let (cx, cy) = (self.piles[pi].0 * s, self.piles[pi].1 * s);
            for dy in -n..=n {
                for dx in -n..=n {
                    if !self.in_pile(dx as f32, dy as f32, r) {
                        continue;
                    }
                    if let Some(i) = self.cell(cx + dx as f32, cy + dy as f32) {
                        if self.agar[i] && self.pile_of[i] == 0 {
                            self.charge[i] = self.p.crystal_charge;
                            self.pile_of[i] = pi as u16 + 1;
                            self.pile_full[pi] += self.p.crystal_charge;
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
    fn in_pile(&self, dx: f32, dy: f32, r: f32) -> bool {
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
        // Each walker's share, from the charge as the pass began. An empty
        // slot (growth) takes nothing.
        for k in 0..self.walkers.len() {
            if !self.lives(k) {
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
            let full = self.p.crystal_charge;
            for &i in &self.pile_cells {
                let i = i as usize;
                if self.pile_of[i] > 0 && self.charge[i] < full {
                    self.charge[i] += g * (full - self.charge[i]);
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
        let (hx, hy) = (self.piles[0].0 * s, self.piles[0].1 * s);
        // A blob as dense as today's inoculum: about 0.9 of the cells in it.
        let r = (n as f32 / (0.9 * std::f32::consts::PI)).sqrt();
        let (hx, hy, r) = if self.p.blob_r > 0.0 { (self.w as f32 / 2.0, self.h as f32 / 2.0, self.p.blob_r * s) } else { (hx, hy, r) };
        let mut k = 0u32;
        while self.walkers.len() < n && k < 50 * n as u32 + 1000 {
            let (a, b, c) = (
                unit(keyed(seed, 1, DRAW_START, 3 * k)),
                unit(keyed(seed, 1, DRAW_START, 3 * k + 1)),
                unit(keyed(seed, 1, DRAW_START, 3 * k + 2)),
            );
            k += 1;
            let hazy = self.p.haze > 0.0 && unit(keyed(seed, 1, DRAW_HAZE, k)) < self.p.haze;
            let (x, y) = if self.p.blob && !hazy {
                let (ang, dist) = (a * TAU, b.sqrt() * r);
                (hx + ang.cos() * dist, hy + ang.sin() * dist)
            } else if self.p.species_zones && self.p.species > 1 {
                // Walker number `len` (the next pushed) is species len % n.
                let n = self.p.species as f32;
                let sp = (self.walkers.len() % self.p.species as usize) as f32;
                ((sp + a) / n * self.w as f32, b * self.h as f32)
            } else {
                (a * self.w as f32, b * self.h as f32)
            };
            let Some(i) = self.cell(x, y) else { continue };
            if !self.agar[i] || (self.p.crowd > 0 && self.occ[i] >= self.p.crowd as u16) {
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
    fn sense(&self, x: f32, y: f32, a: f32, so: f32, sso: f32, sp: usize) -> f32 {
        let (c, s) = (a.cos(), a.sin());
        match self.cell(x + c * so, y + s * so) {
            Some(i) if self.agar[i] => {
                let g = if self.grass.is_empty() { 0.0 } else { self.p.graze_weight * self.grass[i] };
                self.trail_seen(i, sp) + self.scent_at(x + c * sso, y + s * sso) + g
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
        self.trail[j] += amount;
        if !self.species_trail.is_empty() {
            let sp = self.spec[k] as usize;
            let a = if sp == 0 { amount * self.p.species_boost } else { amount };
            self.trail[j] += a - amount;
            self.species_trail[sp][j] += a;
        }
    }

    /// One fine step of the walkers: sense, turn, claim, move, deposit.
    fn move_walkers(&mut self) {
        let p = self.p.clone();
        let (sa, ra, so, st, dep, crowd) = (p.sensor_angle, p.rotate_angle, p.sensor_offset, p.step, p.deposit, p.crowd);
        let respawn = p.respawn;
        let (ss, cap, contagion) = (p.sense_scale, p.deposit_cap, !self.sick.is_empty());
        let (sick_at, catch_c) = (p.immune_steps, p.sick_steps + p.immune_steps);
        let sso = if p.scent_offset > 0.0 { p.scent_offset } else { so };
        let (seed, t) = (p.seed, self.steps);
        // (walker, from, to, x, y, heading); from == to for a move within a cell.
        let mut moves: Vec<(u32, u32, u32, f32, f32, f32, f32)> = Vec::with_capacity(self.walkers.len());
        // (walker, heading, step) of each walker that could not move.
        let mut blocked: Vec<(u32, f32, f32)> = Vec::new();
        let look = t.is_multiple_of(p.sense_every.max(1) as u64);
        let nsp = self.species_trail.len().max(1);
        for k in 0..self.walkers.len() {
            if !self.lives(k) {
                continue;
            }
            let sp = if nsp > 1 { self.spec[k] as usize } else { 0 };
            let (x, y, mut h) = self.walkers[k];
            let here = self.cell(x, y).expect("walker inside the dish");
            // Trail-dependent settings (`sense_scale`), read where it stands.
            let (mut so, mut sa, mut ra, mut st) = (so, sa, ra, st);
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
                let (mut f, mut fl, mut fr) = (self.sense(x, y, h, so, sso, sp), self.sense(x, y, h - sa, so, sso, sp), self.sense(x, y, h + sa, so, sso, sp));
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
                Some(there) if there == here || (self.agar[there] && (crowd == 0 || self.occ[there] < crowd as u16)) => {
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
        if crowd > 0 {
            for &(_, _, to, ..) in &moves {
                self.claim[to as usize] = u32::MAX;
            }
        }
        // Retry rounds (`blocked_retry`): a blocked walker tries its step
        // again into cells emptied this step, so a packed crowd moving one
        // way moves together, as in Jones's random sequential update,
        // instead of only its front row moving.
        for r in 0..if crowd > 0 { p.blocked_retry } else { 0 } {
            let mut tries: Vec<(u32, u32, u32, f32, f32, f32, f32)> = Vec::new();
            let mut still: Vec<(u32, f32, f32)> = Vec::new();
            for &(k, h, st) in &blocked {
                let (x, y, _) = self.walkers[k as usize];
                let here = self.cell(x, y).expect("walker inside the dish");
                let (nx, ny) = (x + h.cos() * st, y + h.sin() * st);
                match self.cell(nx, ny) {
                    Some(there) if there != here && self.agar[there] && self.occ[there] < crowd as u16 => {
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
        if p.blocked_deposit > 0.0 && cap == 0 {
            for &(k, ..) in &blocked {
                let (x, y, _) = self.walkers[k as usize];
                let i = self.cell(x, y).expect("walker inside the dish");
                self.lay(k as usize, i, dep * p.blocked_deposit * self.explore_share(k as usize));
            }
        }
        if !self.wall_health.is_empty() {
            self.bite_walls(&blocked);
        }
        for &(k, h, _) in &blocked {
            let keep = p.blocked_keep && unit(keyed(seed, t, DRAW_KEEP, k)) >= p.blocked_reset;
            self.walkers[k as usize].2 = if keep { h } else { unit(keyed(seed, t, DRAW_BLOCKED, k)) * TAU };
        }
        if cap > 0 {
            // Capped square-root deposit: per cell, from the walkers on it.
            let sick_trail = self.p.sick_trail;
            for k in 0..self.walkers.len() {
                if !self.lives(k) {
                    continue;
                }
                let (x, y, _) = self.walkers[k];
                let i = self.cell(x, y).expect("walker inside the dish");
                let n = self.occ[i].min(cap) as f32;
                // Each of the n walkers on the cell lays its share of
                // deposit * sqrt(n): sqrt(n) / n of a lone walker's.
                let share = dep * n.sqrt() / self.occ[i] as f32;
                let sick = contagion && self.sick[k] > sick_at;
                let amount = if sick { share * sick_trail } else { share };
                self.lay(k, i, amount * self.explore_share(k));
            }
        }
        if contagion {
            for k in 0..self.walkers.len() {
                if !self.lives(k) {
                    continue;
                }
                if self.sick[k] > sick_at {
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
            if !self.energy.is_empty() {
                // Each walker's equal share of what its cell loses below:
                // min(grass, n x eat) over the n walkers on it.
                for k in 0..self.walkers.len() {
                    if !self.lives(k) {
                        continue;
                    }
                    let (x, y, _) = self.walkers[k];
                    let i = self.cell(x, y).expect("walker inside the dish");
                    let n = self.occ[i] as f32;
                    self.energy[k] += (self.grass[i] / n).min(p.graze_eat);
                }
            }
            for k in 0..self.walkers.len() {
                if !self.lives(k) {
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
            for k in 0..self.walkers.len() {
                if !self.lives(k) {
                    continue;
                }
                let (x, y, _) = self.walkers[k];
                let i = self.cell(x, y).expect("walker inside the dish");
                let own = self.spec[k] as usize;
                let mut best = own;
                if p.species_cycle && n >= 3 {
                    // Cyclic: only the species that chases you can take you.
                    best = (own + n - 1) % n;
                } else {
                    for s in 0..n {
                        if s != own && self.species_trail[s][i] > self.species_trail[best][i] {
                            best = s;
                        }
                    }
                }
                if best != own && self.species_trail[best][i] > p.species_convert * self.species_trail[own][i] {
                    self.spec[k] = best as u8;
                }
            }
        }
        if respawn > 0.0 {
            self.respawn_walkers(respawn, crowd);
        }
    }

    /// `energy_flow`: walkers' mean energy evens out between neighbouring
    /// cells that hold walkers. Sums in f64 over slots in order, so the
    /// result does not depend on how walkers sit in their slots beyond
    /// rounding; a GPU port would sum in fixed point.
    fn flow_energy(&mut self) {
        let d = self.p.energy_flow as f64;
        let (w, h) = (self.w, self.h);
        let mut mean = vec![0f64; w * h];
        let mut cells = vec![usize::MAX; self.walkers.len()];
        for k in 0..self.walkers.len() {
            if !self.lives(k) {
                continue;
            }
            let (x, y, _) = self.walkers[k];
            let i = self.cell(x, y).expect("walker inside the dish");
            cells[k] = i;
            mean[i] += self.energy[k] as f64;
        }
        for (i, m) in mean.iter_mut().enumerate() {
            if self.occ[i] > 0 {
                *m /= self.occ[i] as f64;
            }
        }
        let mut gain = vec![0f64; w * h];
        for y in 0..h {
            for x in 0..w {
                let i = y * w + x;
                if self.occ[i] == 0 {
                    continue;
                }
                let mut f = 0.0;
                for (dx, dy) in [(1i64, 0i64), (1, 1), (0, 1), (-1, 1), (-1, 0), (-1, -1), (0, -1), (1, -1)] {
                    let (nx, ny) = (x as i64 + dx, y as i64 + dy);
                    if nx < 0 || ny < 0 || nx >= w as i64 || ny >= h as i64 {
                        continue;
                    }
                    let j = ny as usize * w + nx as usize;
                    if self.occ[j] > 0 {
                        f += mean[j] - mean[i];
                    }
                }
                gain[i] = d * f / self.occ[i] as f64;
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
    fn live_and_die(&mut self) {
        let p = self.p.clone();
        let (seed, t, crowd) = (p.seed, self.steps, p.crowd);
        let n = self.walkers.len();
        for k in 0..n {
            if !self.alive[k] {
                continue;
            }
            // Its crystal take this step (`took`, Larder World) counts as
            // food like its grass share.
            let took = if self.took.is_empty() { 0.0 } else { self.took[k] };
            let e = self.energy[k] + took - p.upkeep;
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
            if !self.alive[k] || self.energy[k] < p.birth_energy || b >= n || self.alive[b] {
                continue;
            }
            let (x, y, _) = self.walkers[k];
            let (dx, dy) = NEXT[(keyed(seed, t, DRAW_BIRTH, k as u32) % 8) as usize];
            let (nx, ny) = (x + dx, y + dy);
            let Some(j) = self.cell(nx, ny) else { continue };
            if !self.agar[j] || (crowd > 0 && self.occ[j] >= crowd as u16) {
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
            if crowd > 0 && self.claim[j] != keyed(seed, t, CLAIM_BIRTH, k as u32) {
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
            if !self.spec.is_empty() {
                self.spec[b] = self.spec[k];
            }
            self.births += 1;
        }
        if crowd > 0 {
            for &(_, _, j, ..) in &tries {
                self.claim[j as usize] = u32::MAX;
            }
        }
    }

    /// The share of the deposit walker `k` lays: `explore_deposit` while it
    /// explores, else 1.
    /// Whether walker `k` is still an explorer (`explore_steps`, `explore_spread`).
    fn explores(&self, k: usize) -> bool {
        if self.age.is_empty() {
            return false;
        }
        let mut n = self.p.explore_steps as f32;
        if self.p.explore_spread > 0.0 {
            let u = unit(keyed(self.p.seed, 0, DRAW_SIZE, k as u32 | 0x8000_0000));
            n *= 1.0 + self.p.explore_spread * (2.0 * u - 1.0);
        }
        (self.age[k] as f32) < n
    }

    fn explore_share(&self, k: usize) -> f32 {
        if self.explores(k) {
            self.p.explore_deposit
        } else {
            1.0
        }
    }

    /// `respawn`: each walker, by its own keyed draws, may jump to a random
    /// agar cell with a new heading; a jump onto rim or a full cell is lost.
    fn respawn_walkers(&mut self, respawn: f32, crowd: u8) {
        let (seed, t) = (self.p.seed, self.steps);
        let (w, h) = (self.w as f32, self.h as f32);
        for k in 0..self.walkers.len() {
            let key = 4 * k as u32;
            if !self.lives(k) || unit(keyed(seed, t, DRAW_RESPAWN, key)) >= respawn {
                continue;
            }
            let x = unit(keyed(seed, t, DRAW_RESPAWN, key + 1)) * w;
            let y = unit(keyed(seed, t, DRAW_RESPAWN, key + 2)) * h;
            let Some(j) = self.cell(x, y) else { continue };
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
        let r = self.p.pile_r * k;
        let n = r.ceil() as i32;
        for (pi, &(px, py)) in self.piles.iter().enumerate() {
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
        let r = self.p.pile_r * k;
        let n = r.ceil() as i32;
        for (pi, &(px, py)) in self.piles.iter().enumerate() {
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
        let r = self.p.pile_r * s;
        let n = r.ceil() as i32;
        for pi in 0..self.piles.len() {
            if self.reached[pi] {
                continue;
            }
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
                    for pass in 0..self.p.trail_passes.max(1) {
                        let keep = if pass == 0 { self.p.keep } else { 1.0 };
                        spread(&self.agar, &self.species_trail[sp], &mut self.scratch, self.w, self.h, keep, self.threads);
                        std::mem::swap(&mut self.species_trail[sp], &mut self.scratch);
                    }
                }
                for (i, v) in self.trail.iter_mut().enumerate() {
                    *v = self.species_trail.iter().map(|f| f[i]).sum();
                }
            }
            if !self.germ.is_empty() {
                spread(&self.agar, &self.germ, &mut self.scratch, self.w, self.h, self.p.sick_keep, self.threads);
                std::mem::swap(&mut self.germ, &mut self.scratch);
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
        let strays = (0..self.walkers.len())
            .filter(|&k| self.lives(k))
            .filter(|&k| self.cell(self.walkers[k].0, self.walkers[k].1).is_some_and(|i| self.trail[i] <= self.p.tube))
            .count();
        let alive = (0..self.walkers.len()).filter(|&k| self.lives(k)).count();
        let energy = if self.energy.is_empty() { 0.0 } else { self.energy.iter().sum::<f32>() / alive.max(1) as f32 };
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
                        if !self.charge.is_empty() {
                            ch = ch.max(self.charge[i] / self.p.crystal_charge);
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
        for &c in &self.spec {
            eat(c as u32);
        }
        for &c in self.sick.iter().chain(&self.age) {
            eat(c as u32);
        }
        for &a in &self.alive {
            eat(a as u32);
        }
        for &e in &self.energy {
            eat(e.to_bits());
        }
        h
    }
}

#[cfg(test)]
mod tests {
    use super::*;

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
            BigParams { blocked_keep: true, blocked_reset: 0.1, blocked_retry: 3, blocked_deposit: 1.0, ..small(2, 1) },
            BigParams { explore_steps: 30, explore_spread: 1.0, ..small(2, 1) },
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

    /// With growth off nothing is kept for it: every slot lives.
    #[test]
    fn growth_off_keeps_no_slots() {
        let d = BigDish::new(small(1, 1));
        assert!(d.alive.is_empty() && d.energy.is_empty() && d.grass.is_empty());
        assert_eq!(d.measure().alive, d.walkers.len());
    }

    /// Crystals run dry: charge only falls, the eaten total matches what
    /// left the piles, and a drained pile lays less scent. Terrain is the
    /// same map at every scale and keeps rock off the piles.
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
