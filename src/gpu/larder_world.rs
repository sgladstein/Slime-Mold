//! **The Larder World on the GPU engine** (ROADMAP W2b): `World` over
//! `Engine`, so `larder_world::Game` plays on the engine as it plays on the
//! CPU big dish, for the window (W2c) and censuses on the GTX 1070.
//!
//! **One game second is the same on both.** The CPU race world takes
//! `substeps = scale` fine steps a tick, and everything that moves, eats or
//! grows runs once a fine step (`BigDish::tick`); the engine takes one fine
//! step a tick. So the GPU world runs `scale` engine ticks for each game
//! tick, and its `ticks()` counts game ticks.
//!
//! **The same mould.** `race_settings` gives the engine the settings whose
//! `big_params` are the CPU race world's `race_params` (bar `substeps`), so
//! both start from the same dish and run the same rules; a test checks it.
//!
//! The engine keeps a CPU mirror of its dish (`Engine::dish`), which the
//! game reads for open ground (`World::open`): walls are built on both, so
//! the mirror's agar follows them. A wall worn to rubble on the GPU is read
//! back into the mirror when the readout's broken count moves (until
//! 2026-11-12 it stayed a wall there, and soldiers walked round rubble).
//!
//! **One wait a step** (2026-11-12, the technical lead review): a frame's
//! results come back in one packed buffer its own command buffer fills
//! (`Readout`). Before, a step made four blocking read-backs, each on a new
//! staging buffer (the squad's words, the kills, the piles, the wall
//! counters), and once a game second two more of every slot's energy and
//! `spec` for the mould's size (8 MB at 100k walkers' 1M slots), which the
//! engine now counts itself (`Engine::record_count`). `World::frame_start`
//! and `frame_finish` split a frame so the window draws while the GPU runs
//! it (`Game::begin_step`, `finish_step`).

mod snap;

use std::cell::{Cell, RefCell};
use std::collections::VecDeque;
use std::sync::atomic::{AtomicBool, Ordering};
use std::sync::Arc;
use std::time::Instant;

use super::{read_back, Curve, Engine, Hand, Settings, Shared, Stamps, Start, MAX_PILES, PILE_WORDS, WSTAT_BYTES};
use pixels::wgpu;
use crate::mould::big::{BigDish, BigParams};
use crate::mould::larder_world::{race_params, Soldier, World};
use crate::mould::map::Map;
use crate::mould::own_mould::{self, Task, Tend};
use crate::mould::squad::{self, Frame, FrameOut, SHOTS_MAX, UNITS_MAX};

/// The CPU race world's settings on the engine (`larder_world::race_params`).
pub fn race_settings(seed: u64, scale: usize, walkers: usize) -> Settings {
    let p = race_params(seed, scale, walkers);
    Settings {
        walkers: p.walkers,
        crowd: p.crowd > 0,
        sensor_angle: p.sensor_angle.to_degrees(),
        turn: p.rotate_angle.to_degrees(),
        reach: p.sensor_offset,
        step: p.step,
        laid: p.deposit * p.walkers as f32 / 100_000.0,
        keep: p.keep,
        respawn: p.respawn,
        smell: p.scent_offset,
        start: if p.blob { Start::Home } else { Start::Spread },
        haze: p.haze,
        start_shape: p.start_shape,
        // Contagion is off (`sick` 0); these only make the params equal.
        immune: p.immune_steps as u32,
        catch_at: p.sick_catch,
        sick_upkeep: p.sick_upkeep,
        upkeep: p.upkeep,
        start_energy: p.start_energy,
        birth_energy: p.birth_energy,
        birth_cost: p.birth_cost,
        starve: p.starve,
        capacity: p.capacity,
        piles: p.piles,
        rect: p.rect,
        crystal: p.crystal_charge,
        crystal_eat: p.crystal_eat,
        crystal_flow: p.crystal_flow,
        crystal_regrow: p.crystal_regrow,
        rock: p.terrain_rock,
        wet: p.terrain_wet,
        wet_step: p.wet_step,
        terrain_blob: p.terrain_blob,
        wall_hp: p.wall_hp,
        wall_bite: p.wall_bite,
        wall_scent: p.wall_scent,
        alarm_keep: p.alarm_keep,
        alarm_weight: p.alarm_weight,
        energy_flow: p.energy_flow,
        hunger_scent: p.hunger_scent,
        hunger_at: p.hunger_at,
        // The crater's shape, as the CPU's race world sets it (the cone,
        // `race_params`).
        scorch_keep: p.scorch_keep,
        scorch_full: p.scorch_full,
        scorch_cone: p.scorch_cone,
        seed,
        ..Settings::big_dish()
    }
}

// The readout's byte offsets: the squad's unit outputs (`squad::U_OUT`),
// its shot outputs (`squad::S_OUT`), each shot's kills, every pile's run and
// sums, the wall counters with the living walkers, then the mould grid
// (10 KB at scale 1 in squares of 4 dish cells, 40 KB at scale 2).
const RO_UNITS: u64 = 0;
const RO_SHOTS: u64 = RO_UNITS + (UNITS_MAX * squad::U_OUT_W * 4) as u64;
const RO_KILLS: u64 = RO_SHOTS + (SHOTS_MAX * 16) as u64;
const RO_PILES: u64 = RO_KILLS + (SHOTS_MAX * 4) as u64;
// Each lump's sums (`dense_base` header words 4-15, `Engine::lump_sums`).
const RO_LUMPS: u64 = RO_PILES + (MAX_PILES * PILE_WORDS * 4) as u64;
const RO_WSTAT: u64 = RO_LUMPS + 48;
const RO_GRID: u64 = RO_WSTAT + WSTAT_BYTES as u64;

/// Game ticks between the mould grid's sums once a grid is asked for
/// (`World::mould_grid`): a game second at the Rules' 15 steps. The tech
/// lead review asked for it (2026-11-12): at the campaign's squares of 4
/// dish cells the grid is 40 KB at scale 2, 160 KB at scale 4 and 655 KB
/// at scale 8, too much to copy back every step for goals a second apart. The sums fall on
/// the frames that reach a multiple of it, so the grid at a whole game
/// second is that second's; a new square is summed on the next frame.
pub const GRID_EVERY: u64 = 15;

/// A frame's results in one buffer that is used again each frame: about
/// 8 KB, and once a game second the mould grid (`GRID_EVERY`), copied
/// after the frame's last pass in its own command buffer.
struct Readout {
    buf: wgpu::Buffer,
    ready: Arc<AtomicBool>,
    /// Copied into and asked for, not yet taken (`GpuWorld::frame_finish`).
    busy: bool,
    /// The submission that fills it, to wait for that one alone (in a
    /// cell: a frame's last share may be sent from `&self`, `flush`).
    sub: RefCell<Option<wgpu::SubmissionIndex>>,
}

/// `GpuWorld::setc`: the staging buffer (sized on first use), whether the
/// running step fills it, whether it mapped, and the cells taken with the
/// tick they are of.
#[derive(Default)]
struct SetCells {
    buf: Option<wgpu::Buffer>,
    due: Cell<bool>,
    ready: Arc<AtomicBool>,
    got: RefCell<Option<(u64, Vec<u32>)>>,
    /// Asks answered from the step's copy.
    hits: Cell<u32>,
}

/// What a readout holds, unpacked.
struct Read {
    out: FrameOut,
    /// Each pile's charge and cells still crystal.
    sums: Vec<(f32, u32)>,
    /// The piles' running sums of what the slime ate (`crystal_keep`), in
    /// 4096ths, added up (wrapping).
    ate: u32,
    /// Crystal cells with charge and a walker on them, pile by pile.
    covered: Vec<usize>,
    broken: u32,
    living: u32,
    /// The player's own mould's living walkers (a garden's; 0 otherwise).
    own: u32,
    /// The walkers asleep (`wstat[12]`) and on full scar (`wstat[13]`).
    asleep: u32,
    scarred: u32,
    /// The mould grid, if the frame summed one.
    grid: Option<Vec<u32>>,
    /// The engine's kill counter at the frame's end (`Engine::struck`,
    /// `wstat[3]`), which the plants' `firekill` pass adds to.
    struck: u32,
    /// Each lump's sums as the frame's last tick began (`Engine::lump_sums`).
    lumps: [(u32, u32, u32); crate::mould::big::LUMPS],
}

/// The engine as a Larder world.
pub struct GpuWorld {
    pub e: Engine,
    pub shared: Shared,
    pub s: Settings,
    /// Engine ticks a game tick (the scale).
    sub: u32,
    /// The crystals and the living walkers as of the last frame's readout.
    crystals: Vec<((f32, f32), f32)>,
    living: usize,
    /// What the slime ate without emptying a crystal (`Settings::
    /// crystal_keep`), charge in all, and the piles' running sum (in
    /// 4096ths) at the last readout.
    ate: f64,
    ate_mark: Option<u32>,
    /// Crystal cells with charge and a walker on them at the last readout,
    /// pile by pile (the slime's birth ceiling, `World::crystal_covered`).
    covered: Vec<usize>,
    /// Each pile's cells at the last readout (`World::crystal_cells_each`).
    cells: Vec<usize>,
    /// The gardeners' state (`GpuWorld::garden`), None in the plain game.
    pub garden: Option<GpuGarden>,
    /// The player's own mould's living walkers at the last readout.
    own: usize,
    /// The walkers asleep and those on full scar at the last readout
    /// (`GpuWorld::asleep`, `GpuWorld::scarred`).
    asleep: usize,
    scarred: usize,
    /// Each lump's sums at the last readout (`World::lumps`).
    lumps: [(u32, u32, u32); crate::mould::big::LUMPS],
    /// Wall cells broken as of the mirror's last agar read.
    broken: u32,
    /// The mirror's wall cells and a gather of their words, so `keep`
    /// reads back only their agar when one breaks; None until the first
    /// break, and again after a wall goes up (its cells are new).
    walls: Option<(Vec<u32>, super::Gather)>,
    /// A wall went up since `walls` was made.
    walls_new: bool,
    ro: Readout,
    /// **The set walkers a fine cell, copied out with a step** (performance
    /// thread, 2026-11-22): on a step that crosses a game second
    /// (`GRID_EVERY`) with setting on, the step's own command buffer copies
    /// `set_cells_of`'s range into this staging buffer, mapped with the
    /// readout, and `take` keeps it with the tick it is of. `set_cells`
    /// (`Game::mark_bulwark`, once a game second) then returns it without a
    /// read-back of its own. That read-back waited for everything queued on
    /// the GPU, the window's frames drawn since the step too: on the
    /// owner's 4K screen (GTX 1070, 11-22) it took 3 to 10 ms a call and
    /// up to 76 ms, once a second, on planet 3. Same data: nothing counts
    /// the set walkers between a step's last pass and the game's call.
    setc: SetCells,
    /// The mould grid's square the game last asked for (`World::mould_grid`),
    /// which the GPU then sums once a game second (`GRID_EVERY`), and the
    /// last sum: its square, its game tick and the grid.
    want: Cell<usize>,
    grid: Option<(usize, u64, Vec<u32>)>,
    /// GPU timestamps of each frame's ticks and drawing (`Stamps`: index 0
    /// to 1 the ticks, 1 to 2 the drawing), if the caller sets them.
    pub stamps: Option<Stamps>,
    /// While Some, every pass of each frame with ticks is timed on its own
    /// (`Engine::record_timed`), and the frame waits for its stamps: the
    /// bench's `passes=N` (2026-11-12). A frame's passes in order, GPU ms
    /// each.
    pub profile: Option<Vec<Vec<(&'static str, f64)>>>,
    /// Over how many frames a step's ticks spread (2026-11-12; 0 and 1
    /// record them all at `frame_start`, as before). With n, the step is
    /// recorded at `frame_start` in up to n shares of its engine ticks
    /// (`Engine::record_share`): the first, with the squad's start, is
    /// sent at once, one more at each `frame_more`, and the last carries
    /// the squad's end and the readout. Each share ends with the drawing,
    /// so the window shows the mould every frame, not every step. The
    /// passes and their order are the whole step's, so the game is the
    /// same (`a_step_in_shares_is_the_same_game`); only when the GPU runs
    /// them moves. At scale 8 a step is 8 engine ticks, about 17 ms on the
    /// 1070; at 60 frames a second (4 a step) the window spreads it over
    /// 4, 2 ticks a frame. No frame timestamps while a step is in shares.
    pub spread: u32,
    /// The running step's shares not yet sent; the last fills the readout.
    later: RefCell<VecDeque<wgpu::CommandBuffer>>,
    /// Frames whose results were not in when asked for, and the real ms
    /// spent waiting for them (`frame_finish`).
    pub waits: u64,
    pub wait_ms: f64,
    /// The engine's kill counter as last read (`Engine::struck`), so
    /// `burst` returns its own kills.
    struck: u32,
    /// Kills on that counter that are fire's (the plants' `firekill`, and
    /// burns), not yet told (`World::burned`). The counter is shared, so
    /// until 2026-11-15 the next kill burst claimed them all as its own
    /// (bug check 11-15): now every frame's readout carries the counter,
    /// what moved during a frame is fire's (bursts run between frames,
    /// never in one), and a kill burst after a burn reads the counter
    /// first so the burn's kills stay fire's.
    fire_new: u32,
    /// A burn has run since `struck` was read: its kills are on the
    /// counter but not yet in `fire_new`.
    burn_dirty: bool,
    /// The bursts' counter snaps (`World::bursts`, `Engine::snap`), kept
    /// from batch to batch and grown as a batch needs.
    snaps: Option<wgpu::Buffer>,
}

/// The gardeners on the GPU world (`own_mould`'s `Garden`, 2026-11-12):
/// the soldiers' tasks and what they did.
#[derive(Clone, Debug, Default)]
#[derive(serde::Serialize, serde::Deserialize)]
pub struct GpuGarden {
    pub tasks: Vec<Task>,
    pub tend: Tend,
    pub roads: u64,
    pub fed: f32,
}

impl GpuGarden {
    pub fn set_task(&mut self, k: usize, t: Task) {
        if self.tasks.len() <= k {
            self.tasks.resize(k + 1, Task::None);
        }
        self.tasks[k] = t;
    }
}

/// The garden's settings: the race world's for both moulds' walkers, with
/// four species slots, the enemy blind to the player's trail and conversion
/// as the CPU garden's (`own_mould::garden_world`). The CPU weighs
/// conversion by presence (`species_convert_per`), which the engine lacks;
/// with both moulds laying the same deposit it compares the same sums.
pub fn garden_settings(seed: u64, scale: usize, walkers: usize) -> Settings {
    // Half the slots a side, as the CPU garden (`own_mould::garden_world`;
    // on the GPU since 11-14, so a mission plays the same on both engines).
    garden_settings_on(race_settings(seed, scale, 2 * walkers), walkers)
}

/// `garden_settings` over any race settings `s` (a map's, a tier's keys
/// on top), with `side` walkers a side: what `own_mould::garden_params`
/// does to the CPU's params.
pub fn garden_settings_on(s: Settings, side: usize) -> Settings {
    let s = Settings { walkers: 2 * side, ..s };
    let half = s.capacity.max(s.walkers) / 2;
    let species_cap = (0..own_mould::SPECIES as usize).map(|sp| if sp == 0 || sp == own_mould::OWN as usize { half } else { 0 }).collect();
    Settings { species: own_mould::SPECIES, avoid: 0.0, convert: own_mould::CONVERT, convert_r: own_mould::CONVERT_R, species_cap, ..s }
}

impl GpuWorld {
    /// Every living walker as a line (a trace for a replay, `LARDER_STUCK`;
    /// 2026-11-23, the owner's walkers stuck in walls): its fine-cell
    /// place, its `spec` word, its cell's agar, ground word and count, and
    /// how many of the 8 cells round it are agar. Reads four buffers back;
    /// not for play.
    pub fn walker_dump(&self) -> Vec<String> {
        let e = &self.e;
        let u32s = |b: &wgpu::Buffer| -> Vec<u32> { read_back(&e.device, &e.queue, b).chunks_exact(4).map(|c| u32::from_le_bytes(c.try_into().unwrap())).collect() };
        let wk = super::floats(&read_back(&e.device, &e.queue, &e.walkers));
        let en = super::floats(&read_back(&e.device, &e.queue, &e.energy));
        let (spec, agar, ground, occ) = (u32s(&e.spec), u32s(&e.agar), u32s(&e.ground), u32s(&e.occ));
        let (w, h) = (e.dish.w, e.dish.h);
        let k = e.dish.p.scale as f32;
        let mut out = Vec::new();
        for q in 0..e.n as usize {
            if e.grows && en[q] < 0.0 {
                continue;
            }
            let st = (spec.get(q).copied().unwrap_or(0) >> 9) & 3;
            if st == 3 {
                continue;
            }
            let (x, y) = (wk[4 * q], wk[4 * q + 1]);
            let (cx, cy) = ((x.max(0.0) as usize).min(w - 1), (y.max(0.0) as usize).min(h - 1));
            let i = cy * w + cx;
            let open = (-1i32..=1).flat_map(|dy| (-1i32..=1).map(move |dx| (dx, dy))).filter(|&(dx, dy)| (dx, dy) != (0, 0)).filter(|&(dx, dy)| {
                let (nx, ny) = (cx as i32 + dx, cy as i32 + dy);
                nx >= 0 && ny >= 0 && (nx as usize) < w && (ny as usize) < h && agar[ny as usize * w + nx as usize] != 0
            }).count();
            out.push(format!("walker {q} at={:.1},{:.1} game={:.1},{:.1} spec={:#x} agar={} ground={:#x} occ={} open8={open}", x, y, x / k, y / k, spec.get(q).copied().unwrap_or(0), agar[i], ground[i], occ[i]));
        }
        out
    }
    /// **The gardeners' world on the GPU**: `own_mould::garden_world`'s
    /// start (the same map and walkers), with the squad's passes seeing
    /// the enemy alone and the soldiers' tasks as tend entries. `s` is
    /// `garden_settings` with any changes; a grazing world is refused (the
    /// player's walkers a cell are counted into the grazing count).
    pub fn garden(device: &wgpu::Device, queue: &wgpu::Queue, s: Settings, seed: u64, scale: usize, walkers: usize) -> Result<Self, String> {
        let s = s.flinched();
        if s.graze != 0.0 {
            return Err(format!("a garden can't graze (graze {}): its count of the player's walkers uses the grazing count", s.graze));
        }
        if s.species != own_mould::SPECIES {
            return Err(format!("a garden runs {} species slots, not {}", own_mould::SPECIES, s.species));
        }
        let m = own_mould::garden_map(seed, scale, walkers);
        let shared = Shared::new(device);
        let mut e = Engine::made_with(device, queue, &shared, &s, scale, |p| own_mould::garden_dish(p, &m, walkers));
        e.garden = true;
        let mut g = Self::of(e, shared, s, scale);
        g.garden = Some(GpuGarden::default());
        g.read_now();
        Ok(g)
    }

    /// **The garden on a designed map on the GPU** (campaign mission 8,
    /// 2026-11-14): `own_mould::garden_on_map`'s start. `s` is the race
    /// world's settings with any tier's keys; the garden's species slots,
    /// conversion and caps go on top (`garden_settings_on`), with
    /// `walkers` a side rounded as `own_mould::garden_map_on`.
    pub fn garden_on_map(device: &wgpu::Device, queue: &wgpu::Queue, s: Settings, scale: usize, m: &Map, walkers: usize) -> Result<Self, String> {
        if s.graze != 0.0 {
            return Err(format!("a garden can't graze (graze {}): its count of the player's walkers uses the grazing count", s.graze));
        }
        let (m, side) = own_mould::garden_map_on(m, walkers)?;
        let s = garden_settings_on(s, side).flinched();
        let shared = Shared::new(device);
        // `made_with` takes a dish, not a Result: a refused start is kept
        // here and returned once the engine is built. Its first call is a
        // one-walker probe of the agar (`agar_cells`), which is only a map.
        let refused = RefCell::new(None);
        let mut e = Engine::made_with(device, queue, &shared, &s, scale, |p| {
            if p.walkers != 2 * side {
                return BigDish::with_map(p, &m);
            }
            own_mould::garden_dish_on(p.clone(), &m, side).unwrap_or_else(|err| {
                *refused.borrow_mut() = Some(err);
                BigDish::with_map(p, &m)
            })
        });
        if let Some(err) = refused.into_inner() {
            return Err(err);
        }
        e.garden = true;
        let mut g = Self::of(e, shared, s, scale);
        g.garden = Some(GpuGarden::default());
        g.read_now();
        Ok(g)
    }

    /// The player's own mould's and the enemy's living walkers (a garden's;
    /// the player's is 0 in the plain game).
    pub fn counts_by_side(&self) -> (usize, usize) {
        (self.own, self.living - self.own)
    }

    /// The walkers asleep (dormant: the Sleeper's body, and any slime's
    /// with the life cycle on) at the last frame's end, from the engine's
    /// per-frame count (no read-back of its own). For the interface's
    /// counters (Look's ask, 2026-11-24).
    pub fn asleep(&self) -> usize {
        self.asleep
    }

    /// The walkers standing on full scar (the Grudge's scar tissue at its
    /// strongest: a blow there misses with chance `scar_tough`) at the last
    /// frame's end; 0 with scar tissue off. Same count as `asleep`.
    pub fn scarred(&self) -> usize {
        self.scarred
    }

    pub fn new(device: &wgpu::Device, queue: &wgpu::Queue, s: Settings, scale: usize) -> Self {
        // The flinch folds into the alarm here, as `BigDish` folds its
        // params when made, so the keys may come in any order.
        let s = s.flinched();
        let shared = Shared::new(device);
        let e = Engine::new(device, queue, &shared, &s, scale);
        Self::of(e, shared, s, scale)
    }

    /// On a designed world (`Engine::with_map`, ROADMAP WM1).
    pub fn with_map(device: &wgpu::Device, queue: &wgpu::Queue, s: Settings, scale: usize, m: &Map) -> Self {
        let s = s.flinched();
        let shared = Shared::new(device);
        let e = Engine::with_map(device, queue, &shared, &s, scale, m);
        Self::of(e, shared, s, scale)
    }

    fn of(mut e: Engine, shared: Shared, s: Settings, scale: usize) -> Self {
        // A game tick is `scale` engine ticks (the module's head), so the
        // plants step at the same game ticks as the CPU's.
        e.plant_sub = scale as u64;
        // No timed seeding of the germ, ever: the dish's `seed_at` (4 s)
        // would infect a `seed_r` (40-cell) disc at the map's middle once
        // contagion is on, a germ the CPU world never has (it seeds only
        // with `seed_at` 0, `BigParams::sick_seed_r`). Found 2026-11-17: a
        // germ deep in the Sleeper's sleeping crust killed none there yet
        // the body lost 22% (16,998 against 21,705 at 40 s); with the
        // seeding off, 0%. Every GPU germ from 11-14 to then carried it, so
        // one round "took 76% of shape C" where it took 48%
        // (`Reports/gun-route-2026-11-17/`). It was first turned off at the
        // first germ (`burst_passes`); here since the technical lead's
        // review of #416, so no other way of turning contagion on (a
        // `sick=` key) can set it off either.
        e.seeded = true;
        let size = RO_GRID + 4 * e.grid_max() as u64;
        let buf = e.device.create_buffer(&wgpu::BufferDescriptor { label: Some("readout"), size, usage: wgpu::BufferUsages::MAP_READ | wgpu::BufferUsages::COPY_DST, mapped_at_creation: false });
        let ro = Readout { buf, ready: Arc::new(AtomicBool::new(false)), busy: false, sub: RefCell::new(None) };
        let mut g = GpuWorld { e, shared, s, sub: scale as u32, crystals: Vec::new(), living: 0, ate: 0.0, ate_mark: None, covered: Vec::new(), cells: Vec::new(), garden: None, own: 0, asleep: 0, scarred: 0, lumps: [(0, 0, 0); crate::mould::big::LUMPS], broken: 0, walls: None, walls_new: false, ro, setc: SetCells::default(), want: Cell::new(0), grid: None, stamps: None, profile: None, spread: 0, later: RefCell::new(VecDeque::new()), waits: 0, wait_ms: 0.0, struck: 0, fire_new: 0, burn_dirty: false, snaps: None };
        g.read_now();
        g
    }

    /// The readout's copies into `enc`, after the frame's last pass.
    fn record_readout(&self, enc: &mut wgpu::CommandEncoder) {
        let e = &self.e;
        self.e.record_count(&self.shared, enc);
        let b = &self.ro.buf;
        enc.copy_buffer_to_buffer(&e.sq, (squad::U_OUT * 4) as u64, b, RO_UNITS, RO_SHOTS - RO_UNITS);
        enc.copy_buffer_to_buffer(&e.sq, (squad::S_OUT * 4) as u64, b, RO_SHOTS, RO_KILLS - RO_SHOTS);
        enc.copy_buffer_to_buffer(&e.kills, 0, b, RO_KILLS, RO_PILES - RO_KILLS);
        enc.copy_buffer_to_buffer(&e.piles, 0, b, RO_PILES, RO_LUMPS - RO_PILES);
        enc.copy_buffer_to_buffer(&e.news, e.dense_at + 16, b, RO_LUMPS, RO_WSTAT - RO_LUMPS);
        let (gw, gh) = e.grid_size();
        enc.copy_buffer_to_buffer(&e.wstat, 0, b, RO_WSTAT, (WSTAT_BYTES + 4 * gw * gh) as u64);
        if let (true, Some(sb)) = (self.setc.due.get(), self.setc.buf.as_ref()) {
            enc.copy_buffer_to_buffer(&e.news, e.crust_at + 4 * (e.cells + e.n) as u64, sb, 0, 4 * e.cells as u64);
        }
    }

    /// The square the next frame's grid sums, if `due`: the one last asked
    /// for, if the engine has room for it (else `mould_grid` reads it
    /// back), or none.
    fn grid_square(&self, due: bool) -> u32 {
        let g = self.want.get();
        let (w, h) = (self.e.dish.w, self.e.dish.h);
        if due && g > 0 && w.div_ceil(g) * h.div_ceil(g) <= self.e.grid_max() {
            g as u32
        } else {
            0
        }
    }

    /// The mould grid summed on the CPU from the cells' walkers read back
    /// (`occ`), as `BigDish`'s: the first ask, before a frame has summed it.
    fn grid_now(&self, bin: usize) -> Vec<u32> {
        self.flush();
        let (w, h) = (self.e.dish.w, self.e.dish.h);
        let gw = w.div_ceil(bin);
        let mut out = vec![0u32; gw * h.div_ceil(bin)];
        for (i, b) in read_back(&self.e.device, &self.e.queue, &self.e.occ).chunks_exact(4).take(w * h).enumerate() {
            let n = u32::from_le_bytes(b.try_into().unwrap());
            if n > 0 {
                out[(i / w / bin) * gw + (i % w) / bin] += n;
            }
        }
        out
    }

    /// `grid`, summed in squares of `b` cells, added up in squares of
    /// `bin` (a whole number of `b`): what `grid_now(bin)` reads, as of the
    /// same tick.
    fn coarser(&self, grid: &[u32], b: usize, bin: usize) -> Vec<u32> {
        let (w, h) = (self.e.dish.w, self.e.dish.h);
        let (fw, k) = (w.div_ceil(b), bin / b);
        let gw = w.div_ceil(bin);
        let mut out = vec![0u32; gw * h.div_ceil(bin)];
        for (i, &n) in grid.iter().enumerate() {
            out[(i / fw / k) * gw + (i % fw) / k] += n;
        }
        out
    }

    /// Submits `enc` and asks for the readout it fills.
    fn submit(&mut self, enc: wgpu::CommandEncoder) {
        assert!(!self.ro.busy, "a frame's readout is still out (frame_finish first)");
        self.ro.busy = true;
        self.send(enc.finish(), true);
    }

    /// Submits a frame's share; the `last` fills the readout, which is then
    /// asked for.
    fn send(&self, cb: wgpu::CommandBuffer, last: bool) {
        let index = self.e.queue.submit([cb]);
        if last {
            *self.ro.sub.borrow_mut() = Some(index);
            let ready = Arc::clone(&self.ro.ready);
            self.ro.buf.map_async(wgpu::MapMode::Read, .., move |r| {
                if r.is_ok() {
                    ready.store(true, Ordering::Release);
                }
            });
            if let (true, Some(sb)) = (self.setc.due.get(), self.setc.buf.as_ref()) {
                let ready = Arc::clone(&self.setc.ready);
                sb.map_async(wgpu::MapMode::Read, .., move |r| {
                    if r.is_ok() {
                        ready.store(true, Ordering::Release);
                    }
                });
            }
        }
    }

    /// Sends the running step's shares not yet sent: before the results
    /// are taken, and before a read-back of the engine's own buffers
    /// (which would otherwise see the step half run).
    fn flush(&self) {
        let mut later = self.later.borrow_mut();
        while let Some(cb) = later.pop_front() {
            self.send(cb, later.is_empty());
        }
    }

    /// The running step's next share sent, if it has one left (`spread`):
    /// the window calls this once a frame while a step runs.
    pub fn frame_more(&mut self) {
        let next = self.later.borrow_mut().pop_front();
        if let Some(cb) = next {
            let last = self.later.borrow().is_empty();
            self.send(cb, last);
        }
    }

    /// The running step's shares still to send.
    pub fn shares_left(&self) -> usize {
        self.later.borrow().len()
    }

    /// Takes the readout, waiting for it if it is not in yet.
    fn take(&mut self) -> Read {
        assert!(self.ro.busy, "no frame running");
        self.flush();
        if !self.frame_ready() {
            let t0 = Instant::now();
            let index = self.ro.sub.borrow_mut().take();
            self.e.device.poll(wgpu::PollType::Wait { submission_index: index, timeout: None }).expect("poll");
            self.waits += 1;
            self.wait_ms += t0.elapsed().as_secs_f64() * 1e3;
        }
        assert!(self.ro.ready.load(Ordering::Acquire), "the readout mapped");
        let words: Vec<u32> = self.ro.buf.get_mapped_range(..).chunks_exact(4).map(|b| u32::from_le_bytes(b.try_into().unwrap())).collect();
        self.ro.buf.unmap();
        self.ro.ready.store(false, Ordering::Release);
        self.ro.busy = false;
        let index = self.ro.sub.borrow_mut().take();
        if self.setc.due.replace(false) {
            if let Some(sb) = self.setc.buf.as_ref() {
                // Mapped with the same submission as the readout; its
                // callback may come a poll later.
                if !self.setc.ready.load(Ordering::Acquire) {
                    self.e.device.poll(wgpu::PollType::Wait { submission_index: index, timeout: None }).expect("poll");
                }
                if self.setc.ready.swap(false, Ordering::AcqRel) {
                    let cells: Vec<u32> = sb.get_mapped_range(..).chunks_exact(4).map(|b| u32::from_le_bytes(b.try_into().unwrap())).collect();
                    sb.unmap();
                    *self.setc.got.borrow_mut() = Some((self.ticks(), cells));
                }
            }
        }
        if let Some(st) = self.stamps.as_mut() {
            st.poll();
        }
        let at = |o: u64| (o / 4) as usize;
        // `squad::unpack` reads the outputs at their places in the squad's
        // words; only those two blocks are filled.
        let mut sq = vec![0u32; squad::PATH];
        sq[squad::U_OUT..squad::S_IN].copy_from_slice(&words[at(RO_UNITS)..at(RO_SHOTS)]);
        sq[squad::S_OUT..squad::PATH].copy_from_slice(&words[at(RO_SHOTS)..at(RO_KILLS)]);
        let (nunits, nshots, _) = self.e.squad;
        let out = squad::unpack(self.e.dish.w, &sq, &words[at(RO_KILLS)..at(RO_PILES)], nunits as usize, nshots as usize);
        let piles = &words[at(RO_PILES)..at(RO_LUMPS)];
        let ls = &words[at(RO_LUMPS)..at(RO_WSTAT)];
        let lumps = std::array::from_fn(|b| (ls[3 * b], ls[3 * b + 1], ls[3 * b + 2]));
        let sums = (0..self.e.npiles as usize).map(|p| (f32::from_bits(piles[p * PILE_WORDS + 4]), piles[p * PILE_WORDS + 2])).collect();
        let st = &words[at(RO_WSTAT)..];
        let (gw, gh) = self.e.grid_size();
        let grid = (gw * gh > 0).then(|| words[at(RO_GRID)..at(RO_GRID) + gw * gh].to_vec());
        let ate = (0..self.e.npiles as usize).fold(0u32, |a, p| a.wrapping_add(piles[p * PILE_WORDS + 7]));
        let covered = (0..self.e.npiles as usize).map(|p| piles[p * PILE_WORDS + 8] as usize).collect();
        Read { out, sums, ate, covered, broken: st[1], living: st[4], own: st[5], asleep: st[12], scarred: st[13], grid, struck: st[3], lumps }
    }

    /// Keeps a readout's world counts: the crystals, the living walkers,
    /// and the mirror's open ground once more wall cells have broken.
    fn keep(&mut self, r: &mut Read) {
        let k = self.e.dish.p.scale as f32;
        self.crystals = self.e.dish.piles.iter().zip(&r.sums).map(|(&(x, y), &(sum, _))| ((x * k, y * k), sum)).collect();
        self.living = r.living as usize;
        self.covered = std::mem::take(&mut r.covered);
        self.cells = r.sums.iter().map(|s| s.1 as usize).collect();
        // The slime's eating with `crystal_keep` (the piles' last words): a
        // world that flows sums twice a tick (`pilesum` before and after
        // `flow`). A count that went back (a fresh dish) starts again.
        if self.s.crystal_keep {
            if let Some(m) = self.ate_mark {
                let moved = r.ate.wrapping_sub(m);
                if moved < 1 << 31 {
                    self.ate += moved as f64 / 4096.0 / if self.e.flows { 2.0 } else { 1.0 };
                }
            }
            self.ate_mark = Some(r.ate);
        }
        self.own = r.own as usize;
        self.asleep = r.asleep as usize;
        self.scarred = r.scarred as usize;
        self.lumps = r.lumps;
        // What the frame (and any burn before it) killed on the shared
        // counter is fire's. A counter behind ours (a fresh dish, an
        // inject) starts the count again.
        let moved = r.struck.wrapping_sub(self.struck);
        if moved < 1 << 31 {
            self.fire_new += moved;
        }
        self.struck = r.struck;
        self.burn_dirty = false;
        if let Some(g) = r.grid.take() {
            self.grid = Some((self.e.grid as usize, self.ticks(), g));
        }
        if r.broken != self.broken {
            // Rare (a cell turns to rubble once), so a read-back of its own,
            // of the wall cells' agar alone: only the walls pass opens agar
            // on the GPU, and only at a wall cell. Until 2026-11-15 this
            // read the whole buffer (3.9 MB at 6x, about 3 ms on the 1070 by
            // the technical review's estimate, its finding 6) whenever the
            // count moved, and the core world rings every nest with hedges.
            if self.walls_new || self.walls.is_none() {
                let d = &self.e.dish;
                let cells: Vec<u32> = (0..d.w * d.h).filter(|&i| d.wall.get(i).copied().unwrap_or(false)).map(|i| i as u32).collect();
                let old = self.walls.take();
                self.walls = super::Gather::new(&self.e.device, &self.e.queue, &cells, old.as_ref().map(|w| &w.1)).map(|g| (cells, g));
                self.walls_new = false;
            }
            if let Some((cells, g)) = &self.walls {
                let (agar, _) = g.read(&self.e.device, &self.e.queue, &self.e.agar, &self.e.agar);
                for (&i, b) in cells.iter().zip(agar.chunks_exact(4)) {
                    self.e.dish.agar[i as usize] = u32::from_le_bytes(b.try_into().unwrap()) != 0;
                }
            }
            self.broken = r.broken;
        }
    }

    /// The readout now, outside a frame (the start, a wall built).
    fn read_now(&mut self) {
        // The count reads the walker count from the params, which the
        // start has not written yet.
        self.e.grid = self.grid_square(true);
        self.e.write_params(&self.s, Hand::default(), 0);
        let mut enc = self.e.device.create_command_encoder(&wgpu::CommandEncoderDescriptor { label: Some("readout") });
        self.record_readout(&mut enc);
        self.submit(enc);
        let mut r = self.take();
        self.keep(&mut r);
    }

    /// One burst's passes with no read-back (`World::bursts` counts
    /// them): the kill, clear, alarm and crater, the fire, the fling, the
    /// gas and its cloud, the scent and the germ, in the CPU's order.
    fn burst_passes(&mut self, b: &squad::Burst, k: &squad::BurstRules) {
        if (b.alarm > 0.0 || b.gas > 0) && self.s.alarm_keep <= 0.0 {
            self.s.alarm_keep = k.alarm_keep;
        }
        if b.kill || b.burn || b.alarm > 0.0 || b.crater > 0 {
            let r = if b.kill || b.burn { b.r } else { 0.0 };
            self.e.burst(&self.shared, &self.s, b.at.0, b.at.1, r, b.alarm, b.alarm_r, b.crater_r, b.crater, 0.0);
        }
        // A reader after the strike and crater, as on the CPU (11-16).
        if b.read > 0 {
            self.e.read_pass(&self.shared, &self.s, b.at.0, b.at.1, b.r, b.read);
        }
        // Fire in the plants, after the strike and crater as on the CPU
        // (`squad::burst`): a share of a burn's disc, a smaller share of a
        // blast's out to a dish cell past its crater. Laid fuel first
        // (Tinder, 11-17), as on the CPU, so a burn on the same disc
        // lights it.
        if b.fuel > 0.0 {
            self.e.lay(&self.shared, &[(b.at.0, b.at.1, b.r, b.fuel)]);
        }
        let mut lights = Vec::new();
        let p = &self.e.dish.p;
        if b.burn {
            lights.push((b.at.0, b.at.1, b.r, p.fire_light_burn));
        }
        if b.crater > 0 && !b.unlit {
            lights.push((b.at.0, b.at.1, b.crater_r + 1.0, p.fire_light_blast));
        }
        // A torch (`rounds::Card::Torch`, 11-17), after the blast's as on
        // the CPU.
        if b.light > 0.0 {
            lights.push((b.at.0, b.at.1, b.r, b.light));
        }
        self.e.ignite(&self.shared, &lights);
        // An explosion's fling (`Engine::fling`, 11-14), after the kill as
        // on the CPU.
        if let Some((r, rr, inv)) = b.fling_ring(self.e.dish.p.scale as f32) {
            self.s.fling_speed = k.fling_speed;
            let sc = self.e.dish.p.scale as f32;
            self.e.fling_pass(&self.shared, &self.s, b.at.0, b.at.1, r / sc, rr / sc, b.fling_steps, inv, b.push);
        }
        // Sleep wears off (2026-11-18): the gas header's word 6, written
        // with the gas and the cloud below.
        self.e.gas_immune = k.gas_immune as u32;
        if b.gas > 0 {
            self.s.gas_death = k.gas_death;
            self.e.gas_pass(&self.shared, &self.s, b.at.0, b.at.1, b.r, b.gas);
            // The cloud after the gas, as `squad::burst` lays it.
            if k.gas_keep > 0.0 {
                (self.s.gas_keep, self.s.gas_sleep, self.s.gas_linger) = (k.gas_keep, k.gas_sleep, k.gas_linger);
                self.e.lay_gas(&self.shared, &self.s, b.at.0, b.at.1, b.r, k.gas_dose);
            }
        }
        if b.bait > 0.0 || b.smother {
            self.e.lay_scent(&self.shared, &self.s, b.at.0, b.at.1, b.r, (if b.smother { 0.0 } else { 1.0 }, b.bait));
        }
        if b.cloud > 0.0 && k.gas_keep > 0.0 {
            (self.s.gas_keep, self.s.gas_sleep, self.s.gas_linger) = (k.gas_keep, k.gas_sleep, k.gas_linger);
            self.e.lay_gas(&self.shared, &self.s, b.at.0, b.at.1, b.r, b.cloud);
        }
        // The germ last, as on the CPU: a burst of its own that kills,
        // clears and lays nothing, only infects within `r`.
        if b.germ {
            if self.s.sick == 0 {
                assert!(k.sick_steps > 0, "the germ needs sick_steps");
                // The dish's timed seeding is off from the start
                // (`GpuWorld::of`). A plain assert, not a debug one (the
                // technical lead's review of #423): CI and every census
                // run in release. One check a burst, as #94's Params
                // lengths.
                assert!(self.e.seeded, "the GPU dish's timed seeding is on in a Larder world: the first germ would infect the map's middle too");
                self.s.sick = k.sick_steps as u32;
                self.s.immune = k.immune_steps as u32;
                self.s.sick_upkeep = k.sick_upkeep;
                self.s.catch_at = k.sick_catch;
                self.s.germ = k.sick_deposit;
                self.s.germ_keep = k.sick_keep;
            }
            self.e.burst(&self.shared, &self.s, b.at.0, b.at.1, 0.0, 0.0, 0.0, 0.0, 0, b.r);
        }
        // A Mind status last, as on the CPU (2026-11-16).
        if b.mind > 0 {
            self.s.spin_curl = k.spin_curl;
            self.e.mind_rest = k.mind_rest as u32;
            self.e.mind_pass(&self.shared, &self.s, b.at.0, b.at.1, b.r, b.mind as u32, k.mind_steps_for(b.mind as u32));
        }
    }
}

impl World for GpuWorld {
    fn save_state(&mut self) -> Result<Vec<u8>, String> {
        self.save_bytes()
    }

    fn load_state(&mut self, bytes: &[u8]) -> Result<(), String> {
        self.load_bytes(bytes)
    }

    fn size(&self) -> (usize, usize) {
        (self.e.dish.w, self.e.dish.h)
    }

    fn scale(&self) -> f32 {
        self.e.dish.p.scale as f32
    }

    fn ticks(&self) -> u64 {
        self.e.done / self.sub as u64
    }

    fn frame(&mut self, f: &Frame, ticks: u32) -> FrameOut {
        self.frame_start(f, ticks);
        self.frame_finish()
    }

    /// Records and submits the frame and returns without waiting for it
    /// (None): `frame_finish` gives its results. The world's counts stay
    /// those of the last frame until then.
    fn frame_start(&mut self, f: &Frame, ticks: u32) -> Option<FrameOut> {
        let n = ticks * self.sub;
        self.e.set_squad(f);
        let (t0, t1) = (self.ticks(), self.ticks() + ticks as u64);
        let fresh = matches!(&self.grid, Some((b, _, _)) if *b == self.want.get());
        self.e.grid = self.grid_square(!fresh || t0 / GRID_EVERY != t1 / GRID_EVERY);
        // The set walkers with this step, if the game will ask for them
        // after it (`setc`).
        let due = self.e.setting && self.e.crust_at != 0 && self.e.cells > 0 && t0 / GRID_EVERY != t1 / GRID_EVERY;
        if due && self.setc.buf.as_ref().is_none_or(|b| b.size() != 4 * self.e.cells as u64) {
            self.setc.buf = Some(self.e.device.create_buffer(&wgpu::BufferDescriptor { label: Some("set cells out"), size: 4 * self.e.cells as u64, usage: wgpu::BufferUsages::MAP_READ | wgpu::BufferUsages::COPY_DST, mapped_at_creation: false }));
        }
        self.setc.due.set(due);
        *self.setc.got.borrow_mut() = None;
        self.e.write_params(&self.s, Hand::default(), n);
        if n > 0 && self.profile.is_some() {
            let t = self.e.record_timed(&self.shared, n, self.s.curve == Curve::Soft);
            self.profile.as_mut().unwrap().push(t);
            let mut enc = self.e.device.create_command_encoder(&wgpu::CommandEncoderDescriptor { label: Some("larder world readout") });
            self.record_readout(&mut enc);
            self.submit(enc);
            return None;
        }
        let soft = self.s.curve == Curve::Soft;
        let shares = self.spread.clamp(1, n.max(1));
        if shares > 1 {
            assert!(!self.ro.busy, "a frame's readout is still out (frame_finish first)");
            self.ro.busy = true;
            let mut sent = 0;
            for k in 0..shares {
                let ticks = n * (k + 1) / shares - sent;
                sent += ticks;
                let last = k + 1 == shares;
                let mut enc = self.e.device.create_command_encoder(&wgpu::CommandEncoderDescriptor { label: Some("larder world share") });
                self.e.record_share(&self.shared, &mut enc, ticks, (k == 0, last), soft, None);
                if last {
                    self.record_readout(&mut enc);
                }
                if k == 0 {
                    self.send(enc.finish(), false);
                } else {
                    self.later.borrow_mut().push_back(enc.finish());
                }
            }
            return None;
        }
        let mut enc = self.e.device.create_command_encoder(&wgpu::CommandEncoderDescriptor { label: Some("larder world") });
        let qs = self.stamps.as_ref().and_then(|st| st.take());
        self.e.record(&self.shared, &mut enc, n, soft, qs);
        let stamped = qs.is_some() && n > 0;
        if stamped {
            self.stamps.as_ref().unwrap().resolve(&mut enc);
        }
        self.record_readout(&mut enc);
        self.submit(enc);
        if stamped {
            self.stamps.as_mut().unwrap().map();
        }
        None
    }

    /// Checks the GPU without blocking (never ready while shares wait to
    /// be sent).
    fn frame_ready(&self) -> bool {
        if !self.later.borrow().is_empty() {
            return false;
        }
        let _ = self.e.device.poll(wgpu::PollType::Poll);
        self.ro.ready.load(Ordering::Acquire)
    }

    fn frame_finish(&mut self) -> FrameOut {
        let mut r = self.take();
        self.keep(&mut r);
        r.out
    }

    /// Trodden ground (`Engine::trample`), between steps as a burst is.
    fn trample(&mut self, discs: &[(f32, f32, f32, f32)]) {
        if discs.is_empty() {
            return;
        }
        self.flush();
        self.e.trample(&self.shared, discs);
    }

    /// Fire's kills since the last call: the plants' `firekill` and the
    /// burns (`Burst::burn`), which `burst` does not count here.
    fn burned(&mut self) -> u32 {
        // Fire's kills in the frames came with their readouts; only a burn
        // since then needs a read-back (the Scorch trail's, once a frame).
        if self.burn_dirty {
            self.flush();
            let now = self.e.struck();
            self.fire_new += now.wrapping_sub(self.struck);
            self.struck = now;
            self.burn_dirty = false;
        }
        std::mem::take(&mut self.fire_new)
    }

    /// The kill, clear, alarm and crater in the engine's strike passes (a
    /// submit of its own, after any frame in flight), then one read-back for
    /// the walkers killed: bursts are a player's tool uses, a few a second
    /// at most, not a pass a frame. Then an explosion's fling
    /// (`Engine::fling`), the sleep gas (`Engine::gas`, 11-14), in the
    /// CPU's order (`squad::burst`), and the germ (11-14):
    /// contagion turns on with the first germ (its settings from `k`, as
    /// `squad::burst` sets the CPU's; sickness changes no step, trail,
    /// reach or turn, as there), and the strike's `kill` pass infects. The
    /// alarm field turns on with the first alarm or gas, as `BigDish::
    /// enable_alarm` does.
    fn burst(&mut self, b: &squad::Burst, k: &squad::BurstRules) -> squad::BurstOut {
        // A shock or a crater alone (the rounds' trails, a burst a bullet
        // a frame) and a burn count nothing now: no read-back. A burn's
        // kills wait for `burned`, once a frame. A reader's kills are its
        // own (2026-11-16), so it reads back as a kill does.
        if !b.kill && b.read == 0 && b.fling_ring(self.e.dish.p.scale as f32).is_none() && b.gas == 0 {
            self.flush();
            self.burst_passes(b, k);
            self.burn_dirty |= b.burn;
            return squad::BurstOut::default();
        }
        self.bursts(&[(*b, *k)]).remove(0)
    }

    /// The bursts' passes in order with a `snap` of the engine's counters
    /// before the first and after each (no wait), then one read-back of
    /// the snaps: each burst's kills, gassed and flung are what its own
    /// passes moved. Until 2026-11-16 each killing burst read the kill
    /// counter back, each fling and gas its count too (`Engine::struck`,
    /// `fling`, `gas`): a volley of eight bombs was 16 stalls.
    fn bursts(&mut self, list: &[(squad::Burst, squad::BurstRules)]) -> Vec<squad::BurstOut> {
        if list.is_empty() {
            return Vec::new();
        }
        // A step in shares (`spread`) runs whole before the bursts, so they
        // land between steps as on the CPU.
        self.flush();
        let need = 16 * (list.len() as u64 + 1);
        if self.snaps.as_ref().is_none_or(|b| b.size() < need) {
            let size = need.next_power_of_two().max(256);
            self.snaps = Some(self.e.device.create_buffer(&wgpu::BufferDescriptor { label: Some("burst snaps"), size, usage: wgpu::BufferUsages::COPY_SRC | wgpu::BufferUsages::COPY_DST, mapped_at_creation: false }));
        }
        let snaps = self.snaps.take().unwrap();
        self.e.snap(&snaps, 0);
        for (i, (b, k)) in list.iter().enumerate() {
            self.burst_passes(b, k);
            self.e.snap(&snaps, i as u64 + 1);
        }
        let w = self.e.read_snaps(&snaps, list.len() + 1);
        self.snaps = Some(snaps);
        // What moved the kill counter before the batch (a burn, a frame
        // still counting) is fire's, and so is a burn's within it.
        self.fire_new += w[0][0].wrapping_sub(self.struck);
        let outs = list
            .iter()
            .zip(w.windows(2))
            .map(|((b, _), d)| {
                let [struck, gassed, flung] = [0, 1, 2].map(|i| d[1][i].wrapping_sub(d[0][i]));
                // A reader's kills are its own, as a strike's.
                let own = b.kill || b.read > 0;
                if !own {
                    self.fire_new += struck;
                }
                squad::BurstOut { killed: if own { struck } else { 0 }, flung, gassed }
            })
            .collect();
        self.struck = w[list.len()][0];
        self.burn_dirty = false;
        outs
    }

    /// The gassed sleepers near each point (`Engine::calm`; the CPU's
    /// `gassed_near`): a pass and a read-back after the frame, only while
    /// some gas lasts.
    fn calm(&self, at: &[(f32, f32)], r: f32) -> Vec<u32> {
        self.e.calm(&self.shared, at, r)
    }

    fn clouds(&self) -> Vec<f32> {
        self.e.clouds.borrow().clone()
    }

    fn has_gas(&self) -> bool {
        true
    }

    /// The germ runs here (2026-11-14): the burst's `kill` pass infects,
    /// and the `life` pass charges the sick `sick_upkeep`.
    fn has_germ(&self) -> bool {
        true
    }

    fn build_wall(&mut self, a: (f32, f32), b: (f32, f32), width: f32) {
        self.e.build_wall(&self.shared, &self.s, a.0, a.1, b.0, b.1, width);
        self.e.dish.build_wall(a.0, a.1, b.0, b.1, width);
        self.walls_new = true;
        // Evicted walkers can die, and the wall's cells are no longer open.
        self.read_now();
    }

    /// The living walkers at the last frame's end (its readout's count);
    /// in a garden the enemy's alone (to the game, the mould).
    fn living(&self) -> usize {
        self.living - self.own
    }

    fn own_living(&self) -> usize {
        self.own
    }

    /// The enemy's living, from the same readout, so no read-back (until
    /// 11-17 plus its condensed walkers, `wstat[8]`).
    /// With the walkers condensed strikers hold (`Engine::striker_held`,
    /// as the last `condense` read them; the review lane, 11-20: without
    /// them the HUD, the end card, fruit's floor and the wave's trigger saw
    /// up to four times less slime than there is).
    fn mass(&self) -> usize {
        self.living - self.own + self.e.striker_held as usize
    }

    fn set_task(&mut self, k: usize, t: Task) {
        if let Some(gd) = self.garden.as_mut() {
            gd.set_task(k, t);
        }
    }

    /// The gardeners' tasks (`own_mould::Garden::tend`), as tend entries
    /// the frame's `tend` pass applies before its shots; the food spent.
    /// Road and Feed as on the CPU; Cut is the det cord here (a burst), so
    /// a Cut task does nothing.
    fn tend(&mut self, soldiers: &[Soldier], bank: f32, ticks: u32) -> f32 {
        let Some(g) = self.garden.as_mut() else { return 0.0 };
        let d = &self.e.dish;
        let (w, h) = (d.w, d.h);
        let s = d.p.scale as f32;
        let secs = ticks as f32 / g.tend.tps as f32;
        let r2 = squad::r2(g.tend.r * s);
        let mut spent = 0.0;
        for (k, so) in soldiers.iter().enumerate() {
            let t = g.tasks.get(k).copied().unwrap_or_default();
            if !so.alive || !matches!(t, Task::Road | Task::Feed) || self.e.tends.len() >= squad::TEND_MAX {
                continue;
            }
            let (cx, cy) = squad::cell_of(w, h, so.x, so.y);
            let cells = squad::disc(w, h, cx, cy, r2).filter(|&i| d.agar[i]).count();
            match t {
                Task::Road => {
                    let a = g.tend.road * secs;
                    self.e.tends.push([cx as u32, cy as u32, 1, r2, a.to_bits(), 0, 0, 0]);
                    g.roads += 1;
                }
                _ => {
                    let cost = (g.tend.feed * secs).min((bank - spent).max(0.0));
                    if cost <= 0.0 || cells == 0 {
                        continue;
                    }
                    let each = cost * g.tend.charge / cells as f32;
                    self.e.tends.push([cx as u32, cy as u32, 2, r2, 0, each.to_bits(), 0, 0]);
                    g.fed += cost;
                    spent += cost;
                }
            }
        }
        spent
    }

    fn crystals(&self) -> Vec<((f32, f32), f32)> {
        self.crystals.clone()
    }

    fn crystal_r(&self, c: usize) -> f32 {
        self.e.dish.pile_rs[c] * self.e.dish.p.scale as f32
    }

    fn landing(&self) -> Option<(f32, f32)> {
        let s = self.e.dish.p.scale as f32;
        self.e.dish.landing.map(|(x, y)| (x * s, y * s))
    }

    fn open(&self, x: f32, y: f32) -> bool {
        World::open(&self.e.dish, x, y)
    }

    fn open_cells(&self) -> Vec<bool> {
        World::open_cells(&self.e.dish)
    }

    fn clear(&self, x: f32, y: f32) -> bool {
        World::clear(&self.e.dish, x, y)
    }

    fn wall_cell(&self, i: usize) -> bool {
        World::wall_cell(&self.e.dish, i)
    }

    fn stop_births(&mut self) {
        // In a garden only the enemy's births stop (its cap goes to 0), as
        // the CPU's `Garden::stop_births`; the player's mould keeps growing.
        if self.garden.is_some() {
            if let Some(c) = self.s.species_cap.get_mut(0) {
                *c = 0;
                return;
            }
        }
        self.s.birth_energy = f32::INFINITY;
    }

    /// Written to the GPU with the next step's params, as births are.
    fn upkeep(&self) -> f32 {
        self.s.upkeep
    }

    fn set_upkeep(&mut self, up: f32) {
        self.s.upkeep = up;
    }

    /// Written to the GPU with the next step's params (`panic_r2`).
    fn set_panic_r(&mut self, r: f32) {
        self.s.panic_r = r;
    }

    fn surge_haste(&self) -> f32 {
        self.s.surge_haste
    }

    /// Written to the GPU with the next step's params, and kept on the
    /// mirror dish, as `set_surge_at`.
    fn set_surge_haste(&mut self, h: f32) {
        self.s.surge_haste = h;
        self.e.dish.p.surge_haste = h;
    }

    fn surge_every(&self) -> u32 {
        self.s.surge_every
    }

    /// Written to the GPU with the next step's params, and kept on the
    /// mirror dish's params so a twin's CPU copy reads the same.
    fn set_surge_at(&mut self, at: u32) {
        self.s.surge_at = at;
        self.e.dish.p.surge_at = at;
    }

    /// As `set_surge_at`: written with the next step's params, and kept on
    /// the mirror dish.
    fn set_raid(&mut self, at: u32, to: (f32, f32), r: f32, share: f32, field: Vec<u32>) {
        let cell = crate::mould::larder_world::raid_cell(to, (self.e.dish.w, self.e.dish.h));
        (self.s.raid_at, self.s.raid_to, self.s.raid_r, self.s.raid_share) = (at, cell, r, share);
        let p = &mut self.e.dish.p;
        (p.raid_at, p.raid_to, p.raid_r, p.raid_share) = (at, cell, r, share);
        // The way round walls (`big::raid_field_for`, built by the game
        // on `raid_ground`).
        self.e.dish.raid_field = field;
        self.e.write_raid_field();
    }

    /// The units' walking distance (S1), written after the slot words.
    fn set_units_field(&mut self, field: Vec<u32>) {
        self.e.dish.units_field = field;
        self.e.write_units_field();
    }

    fn far_smell(&self) -> bool {
        self.s.far_smell > 0.0
    }

    /// From the last readout's crystal sums against the mirror's full
    /// charges (the CPU's `crystal_left`).
    fn fresh_crystals(&self) -> Vec<u32> {
        let d = &self.e.dish;
        self.crystals
            .iter()
            .enumerate()
            .take(crate::mould::big::PANIC_PILES)
            .filter(|&(j, c)| d.pile_full.get(j).is_none_or(|&f| f <= 0.0 || c.1 / f >= crate::mould::big::FAR_FRESH))
            .map(|(j, _)| d.pile_centre(j))
            .collect()
    }

    /// The far smell's field (S2), written after the units' walking
    /// distance.
    fn set_food_field(&mut self, field: Vec<u32>) {
        self.e.dish.food_field = field;
        self.e.write_food_field();
    }

    fn lure_pull(&self) -> bool {
        self.s.lure_pull > 0.0
    }

    fn set_lure_field(&mut self, field: Vec<u32>) {
        self.e.write_lure_field(&field);
    }

    /// Where setting runs (`Engine::setting`: the crust, the scar and
    /// `set_scar`), a walker can set on a fort's mask.
    fn forts(&self) -> bool {
        self.e.setting
    }

    fn set_fort(&mut self, mask: Vec<u32>) {
        self.e.write_fort(&mask);
    }

    /// Runs `lumpform` at once, after the running step's shares (the CPU's
    /// `form_lump`; the mirror's dense words are not kept).
    fn form_lump(&mut self, b: usize, at: Option<((f32, f32), f32)>) {
        self.flush();
        let d = &self.e.dish;
        let at = at.map(|(c, r)| crate::mould::larder_world::lump_reach(c, r * d.p.scale as f32, (d.w, d.h)));
        self.e.form_lump(&self.shared, b, at);
    }

    /// Runs `lumpform`'s grow at once, as `form_lump`.
    fn grow_lump(&mut self, b: usize, (c, r): ((f32, f32), f32)) {
        self.flush();
        let d = &self.e.dish;
        let at = crate::mould::larder_world::lump_reach(c, r * d.p.scale as f32, (d.w, d.h));
        self.e.grow_lump(&self.shared, b, at, None);
    }

    fn rouse(&mut self) {
        self.flush();
        self.e.rouse(&self.shared);
    }

    fn grow_lump_most(&mut self, b: usize, (c, r): ((f32, f32), f32), room: u32) {
        self.flush();
        let d = &self.e.dish;
        let at = crate::mould::larder_world::lump_reach(c, r * d.p.scale as f32, (d.w, d.h));
        self.e.grow_lump(&self.shared, b, at, Some(room));
    }

    /// Written with the next step's params, and kept on the mirror dish.
    fn set_lump(&mut self, b: usize, l: crate::mould::big::Lump) {
        self.s.lumps[b] = l;
        self.e.dish.p.lumps[b] = l;
    }

    /// From the last frame's readout (`RO_LUMPS`), so no read-back.
    fn lumps(&self) -> [Option<(f32, f32, u32)>; crate::mould::big::LUMPS] {
        // The sums are summed only while a lump is on (`lumps_on`), so with
        // none on the last ones stay in the readout: none then, as the
        // CPU's `lump_of` (review of the mini-boss PR).
        let on = self.s.lumps.iter().any(crate::mould::big::Lump::on);
        std::array::from_fn(|b| {
            let (sx, sy, n) = self.lumps[b];
            (on && n > 0).then(|| ((sx / n) as f32 + 0.5, (sy / n) as f32 + 0.5, n))
        })
    }

    /// A ripe fruit's burst (`fruit.rs`), at once, as `form_lump`.
    fn sow(&mut self, at: (f32, f32), r: f32, steps: u16) {
        self.flush();
        let sc = self.e.dish.p.scale as f32;
        self.e.sow_pass(&self.shared, &self.s, at.0 / sc, at.1 / sc, r, steps);
    }

    fn spore_flight(&self) -> f32 {
        self.s.step * self.s.spore_speed
    }

    fn slime_ate(&self) -> f64 {
        self.ate
    }

    fn crystal_covered_each(&self) -> Vec<usize> {
        self.covered.clone()
    }

    fn crystal_cells_each(&self) -> Vec<usize> {
        self.cells.clone()
    }

    fn slots(&self) -> usize {
        self.e.n as usize
    }

    /// Written to the GPU with the next step's params (species 0's cap).
    fn set_ceiling(&mut self, n: u32) {
        self.s.ceiling = n;
    }

    /// Written to the GPU with the next step's params (`Settings::no_eat`).
    fn set_starve(&mut self, on: bool) {
        self.s.no_eat = on;
    }

    /// Each of the pile's cells written 0 (the mirror's list of them).
    fn empty_crystal(&mut self, c: usize) {
        let d = &self.e.dish;
        for &i in &d.pile_cells {
            if d.pile_of.get(i as usize).is_some_and(|&p| p as usize == c + 1) {
                self.e.queue.write_buffer(&self.e.charge, i as u64 * 4, &0f32.to_le_bytes());
            }
        }
    }

    fn can_sow(&self) -> bool {
        true
    }

    fn set_wave_field(&mut self, field: Vec<u32>, pull: f32) {
        self.e.write_wave_field(&field, pull);
    }

    fn can_wave(&self) -> bool {
        true
    }

    fn set_heart(&mut self, at: Option<(f32, f32)>, lean: f32, period: u32, len: u32) {
        self.e.heart = at.map(|at| crate::gpu::Heart { at, lean, period, len });
    }

    fn set_strikers(&mut self, haste: Option<f32>) {
        self.e.strikers = haste;
        if haste.is_none() {
            self.e.striker_held = 0;
        }
    }

    /// At once, after the running step's shares, as `form_lump`.
    fn condense(&mut self, reach: f32, share: f32, most: u32) -> Option<(u32, u32, u32, u32)> {
        self.e.strikers?;
        self.flush();
        Some(self.e.condense(&self.shared, reach, share, most))
    }

    /// The mirror's ground, which the world's walls keep (`build_wall`).
    fn raid_ground(&self) -> Option<(Vec<bool>, usize, usize)> {
        // Deep water is a wall to this engine's slime (`Settings::
        // deep_wall`, 2026-11-20), so the ways it walks (a raid's, a
        // charge's, the far smell's, the lure's) go round by the fords and
        // bridges rather than pressing on the bank.
        let d = &self.e.dish;
        let ground = if self.s.deep_wall { (0..d.w * d.h).map(|i| d.agar[i] && !d.deep(i)).collect() } else { d.agar.clone() };
        Some((ground, d.w, d.h))
    }

    fn food_deposit(&self) -> f32 {
        self.e.dish.p.food_deposit
    }

    fn fine_steps(&self) -> u32 {
        self.sub
    }

    fn walker_step(&self) -> f32 {
        self.s.step
    }

    /// As of the last whole game second (`GRID_EVERY`): once asked for,
    /// the GPU sums the square into the readout once a game second; a
    /// first ask, or a new square, is read back once. The CPU world's grid
    /// is the tick's own, so a goal read off it lands up to a second later
    /// here.
    ///
    /// A square a whole number of times the summed one is added up from
    /// it, and leaves the GPU's square as it was (the bug hunt, 11-21): the
    /// game's rules ask at `scale` and the bots at 4 x `scale`, and while
    /// each ask moved the square, every rule's ask after a bot's missed and
    /// read the cells back fresh, so a bot's game saw a fresher grid than
    /// play does (one `walkers_in` call more turned a planet 2 loss on run
    /// seed 1 into a win).
    fn mould_grid(&self, bin: usize) -> Option<Vec<u32>> {
        match &self.grid {
            Some((b, _, g)) if *b == bin => return Some(g.clone()),
            Some((b, _, g)) if *b > 0 && bin.is_multiple_of(*b) => return Some(self.coarser(g, *b, bin)),
            _ => {}
        }
        self.want.set(bin);
        Some(self.grid_now(bin))
    }

    /// Read back once a game second while a rule wants it (`Game::mark_bulwark`).
    fn set_cells(&self) -> Option<Vec<u32>> {
        self.e.setting.then(|| {
            // The copy the step brought (`setc`), if it is of now.
            if let Some((t, cells)) = self.setc.got.borrow_mut().take() {
                if t == self.ticks() && cells.len() == self.e.cells as usize {
                    self.setc.hits.set(self.setc.hits.get() + 1);
                    // The tests read it back too: the step's copy must be
                    // the read-back's cells.
                    #[cfg(test)]
                    {
                        self.flush();
                        assert_eq!(cells, self.e.set_cells_of(), "the step's copy of the set cells is the read-back's");
                    }
                    return cells;
                }
            }
            self.flush();
            self.e.set_cells_of()
        })
    }

    /// The walkers, trail and energy read back, hashed (a replay's check;
    /// it waits for the GPU, so the game calls it only at the end).
    fn hash(&self) -> u64 {
        self.flush();
        let side = (self.e.done % 2) as usize;
        let mut h = 0xcbf2_9ce4_8422_2325u64;
        for buf in [&self.e.walkers, &self.e.trail[side], &self.e.energy] {
            for b in read_back(&self.e.device, &self.e.queue, buf) {
                h = (h ^ b as u64).wrapping_mul(0x100_0000_01b3);
            }
        }
        h
    }
}

/// The params the engine runs for settings `s` at `scale` (its CPU
/// mirror's), for comparing a GPU world with its CPU twin.
pub fn engine_params(s: &Settings, scale: usize) -> BigParams {
    super::big_params(s, scale, s.crowd)
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::gpu::{block_on, reads};
    use crate::mould::larder_world::{Cmd, End, Game, Order, Rules};
    use crate::mould::tiers::Tier;

    /// Whether the tests run on a real card, not software Vulkan (llvmpipe in
    /// CI). Short twins that match exactly on llvmpipe are the bug detectors
    /// there and keep their exact bound; a card orders atomics and float
    /// sums its own way (the GTX 1070, 2026-11-14, HANDOFF decision 6), so
    /// it gets the slack it measured (technical lead, #261).
    static ON_CARD: std::sync::atomic::AtomicBool = std::sync::atomic::AtomicBool::new(false);

    /// `exact` on software Vulkan, `card` on a real GPU (`ON_CARD`).
    fn tol<T>(exact: T, card: T) -> T {
        if ON_CARD.load(std::sync::atomic::Ordering::Relaxed) { card } else { exact }
    }

    /// A GPU, or None (the test passes, saying so) where there is none; CI
    /// sets `MOULDGPU_TEST_GPU=1` (Mesa's software Vulkan), so there a
    /// missing GPU fails. As `mouldgpu`'s `test_gpu`.
    fn test_gpu() -> Option<(wgpu::Device, wgpu::Queue)> {
        let required = std::env::var("MOULDGPU_TEST_GPU").is_ok_and(|v| v == "1");
        let instance = wgpu::Instance::new(wgpu::InstanceDescriptor::new_without_display_handle_from_env());
        let adapter = match block_on(instance.request_adapter(&wgpu::RequestAdapterOptions::default())) {
            Ok(a) => a,
            Err(e) => {
                assert!(!required, "MOULDGPU_TEST_GPU=1 but no GPU adapter: {e}");
                eprintln!("no GPU adapter ({e}): skipped");
                return None;
            }
        };
        let info = adapter.get_info();
        ON_CARD.store(info.device_type != wgpu::DeviceType::Cpu, std::sync::atomic::Ordering::Relaxed);
        let desc = wgpu::DeviceDescriptor { label: Some("readout"), required_limits: adapter.limits(), ..Default::default() };
        Some(block_on(adapter.request_device(&desc)).expect("device"))
    }

    /// The GPU world with every frame checked against the read-backs the
    /// readout replaced (`Engine::squad_out`, `world_counts`, `alive`).
    struct Checked {
        g: GpuWorld,
        /// The mould grid's square (the player AI's 4 dish cells).
        bin: usize,
        frames: u32,
        /// Frames that summed the grid.
        sums: u32,
        hits: u32,
        touched: u32,
        dead: usize,
    }

    impl World for Checked {
        fn size(&self) -> (usize, usize) {
            self.g.size()
        }
        fn scale(&self) -> f32 {
            self.g.scale()
        }
        fn ticks(&self) -> u64 {
            self.g.ticks()
        }
        fn frame(&mut self, f: &Frame, ticks: u32) -> FrameOut {
            let (r0, w0) = (reads(), self.g.waits);
            let out = self.g.frame(f, ticks);
            assert_eq!(reads(), r0, "frame {}: no read-back of its own", self.frames);
            assert!(self.g.waits <= w0 + 1, "frame {}: one wait at most", self.frames);
            assert_eq!(out, self.g.e.squad_out(), "frame {}: the squad's output", self.frames);
            let k = self.g.scale();
            let sums: Vec<((f32, f32), f32)> = self.g.e.dish.piles.iter().zip(self.g.e.world_counts().0).map(|(&(x, y), (sum, _))| ((x * k, y * k), sum)).collect();
            assert_eq!(self.g.crystals(), sums, "frame {}: the crystals", self.frames);
            assert_eq!(self.g.living(), self.g.e.alive(), "frame {}: the living walkers", self.frames);
            let (bin, t) = (self.bin, self.g.ticks());
            let &(b, at, _) = self.g.grid.as_ref().expect("a grid summed on the GPU");
            assert!(b == bin && at <= t && t - at < GRID_EVERY, "frame {}: the grid's square {b} and tick {at} at tick {t}", self.frames);
            if at == t {
                self.sums += 1;
                assert_eq!(self.g.mould_grid(bin), Some(self.g.grid_now(bin)), "frame {}: the mould grid", self.frames);
            }
            if self.g.e.cycles {
                let (life, _) = self.g.e.life_of();
                let (dead, living) = (life.iter().filter(|&&l| l == crate::mould::big::LIFE_DEAD).count(), self.g.living());
                self.dead = self.dead.max(dead);
                // With growth a dead slot's energy is -1, so `alive` skips
                // it as the count does.
                assert!(living + dead <= self.g.e.n as usize, "frame {}", self.frames);
            }
            self.frames += 1;
            self.hits += out.shots.iter().filter(|s| s.killed > 0).count() as u32;
            self.touched += out.units.iter().map(|u| u.touching).sum::<u32>();
            out
        }
        fn build_wall(&mut self, a: (f32, f32), b: (f32, f32), width: f32) {
            self.g.build_wall(a, b, width)
        }
        fn burst(&mut self, b: &squad::Burst, k: &squad::BurstRules) -> squad::BurstOut {
            self.g.burst(b, k)
        }
        fn bursts(&mut self, list: &[(squad::Burst, squad::BurstRules)]) -> Vec<squad::BurstOut> {
            self.g.bursts(list)
        }
        fn has_germ(&self) -> bool {
            self.g.has_germ()
        }
        fn living(&self) -> usize {
            self.g.living()
        }
        fn mass(&self) -> usize {
            self.g.mass()
        }
        fn own_living(&self) -> usize {
            self.g.own_living()
        }
        fn set_task(&mut self, k: usize, t: Task) {
            World::set_task(&mut self.g, k, t);
        }
        fn crystals(&self) -> Vec<((f32, f32), f32)> {
            self.g.crystals()
        }
        fn crystal_r(&self, c: usize) -> f32 {
            self.g.crystal_r(c)
        }
        fn landing(&self) -> Option<(f32, f32)> {
            self.g.landing()
        }
        fn open(&self, x: f32, y: f32) -> bool {
            self.g.open(x, y)
        }
        fn clear(&self, x: f32, y: f32) -> bool {
            self.g.clear(x, y)
        }
        fn wall_cell(&self, i: usize) -> bool {
            self.g.wall_cell(i)
        }
        fn stop_births(&mut self) {
            self.g.stop_births()
        }
        fn upkeep(&self) -> f32 {
            self.g.upkeep()
        }
        fn set_upkeep(&mut self, up: f32) {
            self.g.set_upkeep(up)
        }
        fn set_panic_r(&mut self, r: f32) {
            self.g.set_panic_r(r)
        }
        fn surge_haste(&self) -> f32 {
            self.g.surge_haste()
        }
        fn set_surge_haste(&mut self, h: f32) {
            self.g.set_surge_haste(h)
        }
        fn surge_every(&self) -> u32 {
            self.g.surge_every()
        }
        fn set_surge_at(&mut self, at: u32) {
            self.g.set_surge_at(at)
        }
        fn set_raid(&mut self, at: u32, to: (f32, f32), r: f32, share: f32, field: Vec<u32>) {
            self.g.set_raid(at, to, r, share, field)
        }
        fn set_units_field(&mut self, field: Vec<u32>) {
            self.g.set_units_field(field)
        }
        fn far_smell(&self) -> bool {
            self.g.far_smell()
        }
        fn fresh_crystals(&self) -> Vec<u32> {
            self.g.fresh_crystals()
        }
        fn set_food_field(&mut self, field: Vec<u32>) {
            self.g.set_food_field(field)
        }
        fn lure_pull(&self) -> bool {
            self.g.lure_pull()
        }
        fn set_lure_field(&mut self, field: Vec<u32>) {
            self.g.set_lure_field(field)
        }
        fn forts(&self) -> bool {
            self.g.forts()
        }
        fn set_fort(&mut self, mask: Vec<u32>) {
            self.g.set_fort(mask)
        }
        fn raid_ground(&self) -> Option<(Vec<bool>, usize, usize)> {
            self.g.raid_ground()
        }
        fn food_deposit(&self) -> f32 {
            self.g.food_deposit()
        }
        fn fine_steps(&self) -> u32 {
            self.g.fine_steps()
        }
        fn walker_step(&self) -> f32 {
            self.g.walker_step()
        }
        fn hash(&self) -> u64 {
            self.g.hash()
        }
        fn set_cells(&self) -> Option<Vec<u32>> {
            self.g.set_cells()
        }
        fn mould_grid(&self, bin: usize) -> Option<Vec<u32>> {
            self.g.mould_grid(bin)
        }
        fn form_lump(&mut self, b: usize, at: Option<((f32, f32), f32)>) {
            self.g.form_lump(b, at)
        }
        fn grow_lump(&mut self, b: usize, at: ((f32, f32), f32)) {
            self.g.grow_lump(b, at)
        }
        fn grow_lump_most(&mut self, b: usize, at: ((f32, f32), f32), room: u32) {
            self.g.grow_lump_most(b, at, room)
        }
        fn rouse(&mut self) {
            self.g.rouse()
        }
        fn set_lump(&mut self, b: usize, l: crate::mould::big::Lump) {
            self.g.set_lump(b, l)
        }
        fn lumps(&self) -> [Option<(f32, f32, u32)>; crate::mould::big::LUMPS] {
            self.g.lumps()
        }
        fn sow(&mut self, at: (f32, f32), r: f32, steps: u16) {
            self.g.sow(at, r, steps)
        }
        fn spore_flight(&self) -> f32 {
            self.g.spore_flight()
        }
        fn slime_ate(&self) -> f64 {
            self.g.slime_ate()
        }
        fn crystal_covered_each(&self) -> Vec<usize> {
            self.g.crystal_covered_each()
        }
        fn crystal_cells_each(&self) -> Vec<usize> {
            self.g.crystal_cells_each()
        }
        fn slots(&self) -> usize {
            self.g.slots()
        }
        fn set_ceiling(&mut self, n: u32) {
            self.g.set_ceiling(n)
        }
        fn set_starve(&mut self, on: bool) {
            self.g.set_starve(on)
        }
        fn empty_crystal(&mut self, c: usize) {
            self.g.empty_crystal(c)
        }
        fn can_sow(&self) -> bool {
            self.g.can_sow()
        }
        fn set_wave_field(&mut self, field: Vec<u32>, pull: f32) {
            self.g.set_wave_field(field, pull)
        }
        fn can_wave(&self) -> bool {
            self.g.can_wave()
        }
        fn set_heart(&mut self, at: Option<(f32, f32)>, lean: f32, period: u32, len: u32) {
            self.g.set_heart(at, lean, period, len)
        }
        fn set_strikers(&mut self, haste: Option<f32>) {
            self.g.set_strikers(haste)
        }
        fn condense(&mut self, reach: f32, share: f32, most: u32) -> Option<(u32, u32, u32, u32)> {
            self.g.condense(reach, share, most)
        }
    }

    /// **The packed readout gives what the four read-backs gave**
    /// (2026-11-12): every frame's squad output, crystals and living walkers
    /// equal `squad_out`, `world_counts` and `alive` read back after it, and
    /// its mould grid the one summed from the cells read back, with no
    /// read-back and at most one wait in the frame. A squad attacks
    /// on a small race world (scale 1) until its shots kill and the mould
    /// touches it, on the plain mould and on the Sleeper (the life cycle,
    /// dying asleep), whose dead the count must leave out.
    #[test]
    fn the_readout_matches_the_read_backs() {
        let Some((device, queue)) = test_gpu() else { return };
        for tier in [None, Some(Tier::Sleeper)] {
            let (seed, scale, walkers) = (5, 1, 2_000);
            let mut s = race_settings(seed, scale, walkers);
            if let Some(t) = tier {
                t.settings(&mut s).unwrap();
            }
            let g = GpuWorld::new(&device, &queue, s, scale);
            let bin = 4 * scale;
            assert_eq!(g.mould_grid(bin), Some(g.grid_now(bin)), "the first ask is read back");
            assert!(g.grid.is_none() && g.want.get() == bin);
            let rules = Rules { cap: 45.0, ..Rules::default() };
            assert_eq!(g.living(), g.e.alive(), "the count at the start");
            let mut game = Game::new(Checked { g, bin, frames: 0, sums: 0, hits: 0, touched: 0, dead: 0 }, rules, seed);
            for k in 0..game.soldiers.len() {
                game.apply(Cmd::Order(k, Order::Attack));
            }
            while game.end.is_none() {
                game.step();
            }
            let c = &game.world;
            let name = tier.map_or("plain", |t| t.name());
            eprintln!("{name}: {} frames, {} grid sums, {} shots that killed, {} touches, {} waits ({:.1} ms), up to {} dead", c.frames, c.sums, c.hits, c.touched, c.g.waits, c.g.wait_ms, c.dead);
            // The first frame (a new square), then each whole game second.
            assert_eq!(c.sums as u64, 1 + c.frames as u64 / GRID_EVERY, "{name}: the grid summed once a game second");
            assert!(c.frames > 100 && c.hits > 0 && c.touched > 0, "{name}: the squad fought ({} frames, {} hits, {} touches)", c.frames, c.hits, c.touched);
            assert!(tier.is_none() || c.dead > 0, "{name}: walkers died asleep");
        }
    }

    /// **A planet saved mid-game loads as it was** (save and resume A,
    /// Milestone 2, 2026-11-24): five seconds in, the game is saved, read
    /// onto a world built fresh, and saved again: the two saves are the
    /// same bytes (every engine buffer read back, every CPU field, the
    /// game's whole state). A save for another planet is refused. Then
    /// both play on two seconds and still save the same bytes (the engine
    /// is deterministic on one device), so nothing a step reads was left
    /// out.
    #[test]
    fn a_planet_saved_mid_game_loads_as_it_was() {
        let Some((device, queue)) = test_gpu() else { return };
        let (seed, scale, walkers) = (5, 1, 2_000);
        let fresh = || GpuWorld::new(&device, &queue, race_settings(seed, scale, walkers), scale);
        let rules = || Rules { cap: 30.0, ..Rules::default() };
        let mut g = Game::new(fresh(), rules(), seed);
        while g.time() < 5.0 && g.end.is_none() {
            g.step();
        }
        let saved = g.save("race 5").unwrap();
        let mut back = Game::from_save(&saved, "race 5", fresh()).unwrap();
        assert_eq!((back.time(), back.mould, back.killed), (g.time(), g.mould, g.killed));
        let again = back.save("race 5").unwrap();
        assert_eq!(again.len(), saved.len());
        assert!(again == saved, "the loaded game saves the same bytes");
        assert!(Game::from_save(&saved, "race 6", fresh()).is_err(), "another planet's save");
        for _ in 0..60 {
            g.step();
            back.step();
        }
        assert_eq!((g.world.ticks(), g.mould), (back.world.ticks(), back.mould));
        let (a, b) = (g.save("race 5").unwrap(), back.save("race 5").unwrap());
        assert!(a == b, "played on, the two games save the same bytes: {} differ", a.iter().zip(&b).filter(|(x, y)| x != y).count());
    }

    /// **A bot's coarser ask leaves the rules' grid alone** (the bug hunt,
    /// 11-21): asked for squares four times the summed ones, the grid is
    /// added up from the summed one, equal to the cells' own sum as of its
    /// tick, and the GPU goes on summing the rules' square, so the rules'
    /// next ask is still the summed grid and reads nothing back.
    #[test]
    fn a_coarser_grid_is_added_up_from_the_summed_one() {
        let Some((device, queue)) = test_gpu() else { return };
        let (seed, scale, walkers) = (5, 1, 2_000);
        let mut g = Game::new(GpuWorld::new(&device, &queue, race_settings(seed, scale, walkers), scale), Rules { cap: 30.0, ..Rules::default() }, seed);
        let mut checked = 0;
        while g.time() < 6.0 && g.end.is_none() {
            let _ = g.world.mould_grid(1);
            g.step();
            let w = &g.world;
            let Some((b, at, _)) = w.grid.as_ref().map(|(b, at, v)| (*b, *at, v.len())) else { continue };
            assert_eq!(b, 1, "the rules' square is summed");
            if at != w.ticks() {
                continue;
            }
            let coarse = w.mould_grid(4).unwrap();
            assert_eq!(coarse, w.grid_now(4), "at {at}: the coarse grid is the cells' sum");
            assert_eq!(w.want.get(), 1, "the coarse ask left the square");
            checked += 1;
        }
        assert!(checked >= 3, "{checked} whole seconds checked");
    }

    /// **A turret sees and fires over a wall on both engines** (owner,
    /// 2026-11-16 19:50Z: "turrets can fire over walls, lobbed fire goes
    /// over walls, but regular guns shots do not"): a box of wall round a
    /// unit on the race world's spread walkers, one unit and shot plain,
    /// one `over`. The plain unit sees nothing past the box and the plain
    /// shot, fired from the wall's inner face, stops in it; the `over`
    /// ones see and hit past it, the GPU as the CPU (the same block, hit
    /// cell and kills).
    #[test]
    fn a_turret_fires_over_a_wall_on_both_engines() {
        use crate::mould::larder_world::race_world;
        use crate::mould::squad::{ShotIn, UnitIn};
        let Some((device, queue)) = test_gpu() else { return };
        let (seed, scale, walkers) = (5, 1, 1_000);
        let mut gpu = GpuWorld::new(&device, &queue, race_settings(seed, scale, walkers), scale);
        let mut cpu = race_world(seed, scale, walkers);
        let (w, h) = World::size(&cpu);
        let f0 = Frame { slime: 1.0, ..Frame::default() };
        let _ = (gpu.frame(&f0, 1), cpu.frame(&f0, 1));
        // A spot with open ground 8 cells round, no walker within 8 cells
        // and the most from 8 to 24.
        let occ = cpu.occupancy();
        let around = |p: (i32, i32), r0: i32, r1: i32| -> usize {
            (-r1..=r1).flat_map(|dy| (-r1..=r1).map(move |dx| (dx, dy))).filter(|&(dx, dy)| (r0 * r0..=r1 * r1).contains(&(dx * dx + dy * dy))).map(|(dx, dy)| (p.0 + dx, p.1 + dy)).filter(|&(x, y)| x >= 0 && y >= 0 && x < w as i32 && y < h as i32).map(|(x, y)| occ[y as usize * w + x as usize] as usize).sum()
        };
        let open = |p: (i32, i32)| (-8..=8).all(|dy| (-8..=8).all(|dx| World::clear(&cpu, (p.0 + dx) as f32 + 0.5, (p.1 + dy) as f32 + 0.5)));
        let (ux, uy) = (8..h as i32 - 8).step_by(2).flat_map(|y| (8..w as i32 - 8).step_by(2).map(move |x| (x, y))).filter(|&p| around(p, 0, 8) == 0 && open(p)).max_by_key(|&p| around(p, 8, 24)).expect("open ground");
        let u = (ux as f32 + 0.5, uy as f32 + 0.5);
        // The box: four walls 5 cells out, 2 thick.
        let r = 5.0;
        for (a, b) in [((-r, -r), (r, -r)), ((r, -r), (r, r)), ((r, r), (-r, r)), ((-r, r), (-r, -r))] {
            let (a, b) = ((u.0 + a.0, u.1 + a.1), (u.0 + b.0, u.1 + b.1));
            World::build_wall(&mut gpu, a, b, 2.0);
            World::build_wall(&mut cpu, a, b, 2.0);
        }
        let wall = |i: usize| squad::over_wall(&cpu.wall, &cpu.agar, i);
        let cell = |p: (f32, f32)| p.1 as usize * w + p.0 as usize;
        // East, from the first wall cell on the way out.
        let face = (0..8).map(|k| (u.0 + k as f32, u.1)).find(|&p| wall(cell(p))).expect("the box's east wall");
        let unit = |over: bool| UnitIn { x: u.0, y: u.1, touch: 2.0, sight: 30.0, over, ..UnitIn::default() };
        let shot = |over: bool| ShotIn { from: face, to: (face.0 + 20.0, face.1), r: 3.0, over };
        let f = Frame { units: vec![unit(false), unit(true)], shots: vec![shot(false), shot(true)], slime: 0.0, ..Frame::default() };
        let (g, k) = (gpu.frame(&f, 0), cpu.frame(&f, 0));
        let past = |p: (f32, f32)| (p.0 - u.0).abs().max((p.1 - u.1).abs()) > r + 1.0;
        assert_eq!(k.units[0].target.filter(|&p| past(p)), None, "the plain unit sees over the wall");
        assert!(k.units[1].target.is_some_and(past), "the turret sees past the wall: {:?}", k.units[1].target);
        assert_eq!(k.shots[0].hit, None, "the plain shot flew through the wall");
        assert!(k.shots[1].hit.is_some_and(past), "the turret's shot hit past the wall: {:?}", k.shots[1]);
        for q in 0..2 {
            assert_eq!(g.units[q].target, k.units[q].target, "unit {q}: GPU and CPU aim apart");
            assert_eq!((g.shots[q].hit, g.shots[q].killed), (k.shots[q].hit, k.shots[q].killed), "shot {q}: GPU and CPU apart");
        }
        eprintln!("turret aims {:?}, its shot hits {:?} and kills {}", k.units[1].target, k.shots[1].hit, k.shots[1].killed);
    }

    /// **A wall worn to rubble opens in the mirror** (2026-11-15): a wall
    /// built on a race world, bitten to nothing on the GPU, opens there,
    /// and the frame's `keep` reads the wall cells' agar back (a gather of
    /// those cells alone, the technical review's finding 6), so the
    /// mirror's open ground equals the whole buffer read back. A second
    /// wall renews the gathered cells.
    #[test]
    fn a_wall_worn_to_rubble_opens_in_the_mirror() {
        let Some((device, queue)) = test_gpu() else { return };
        let (seed, scale) = (5, 1);
        let mut s = race_settings(seed, scale, 500);
        (s.wall_hp, s.wall_bite) = (2.0, 1.0);
        let g = GpuWorld::new(&device, &queue, s, scale);
        let (w, h) = g.size();
        let mut game = Game::new(g, Rules::default(), seed);
        let gpu_agar = |g: &GpuWorld| read_back(&g.e.device, &g.e.queue, &g.e.agar).chunks_exact(4).map(|b| u32::from_le_bytes(b.try_into().unwrap()) != 0).collect::<Vec<bool>>();
        for (k, y) in [(0, 0.25), (1, 0.75)] {
            let g = &mut game.world;
            let walls = |g: &GpuWorld| (0..w * h).filter(|&i| g.e.dish.wall.get(i).copied().unwrap_or(false)).collect::<Vec<usize>>();
            let before = walls(g);
            let (x, y) = (w as f32 * 0.5, h as f32 * y);
            g.build_wall((x - 8.0, y), (x + 8.0, y), 2.0);
            let new: Vec<usize> = walls(g).into_iter().filter(|i| !before.contains(i)).collect();
            assert!(!new.is_empty() && new.iter().all(|&i| !g.e.dish.agar[i]), "wall {k}: built and shut");
            let mut bites = vec![0u32; w * h];
            for &i in &new {
                bites[i] = 1_000;
            }
            g.e.queue.write_buffer(&g.e.bites, 0, &super::super::u32_bytes(&bites));
            game.step();
            let g = &game.world;
            let opened = new.iter().filter(|&&i| g.e.dish.agar[i]).count();
            assert_eq!(opened, new.len(), "wall {k}: every bitten cell opened in the mirror");
            assert!(gpu_agar(g) == g.e.dish.agar, "wall {k}: the mirror's agar is the GPU's");
        }
    }

    /// **A step in shares is the same game** (`GpuWorld::spread`,
    /// 2026-11-12): one round, each step 4 engine ticks (scale 2, two game
    /// ticks a step), played with the ticks recorded whole, in 2 shares
    /// and in 4. The shares are sent one a `frame_more`, with 0 to 3 calls
    /// between steps by turns (so some steps finish with shares unsent,
    /// which `frame_finish` sends), and a first mould-grid ask and a hash
    /// fall mid-step (their read-backs send the rest). Every step's counts
    /// and the hash at the end match the whole steps'.
    #[test]
    fn a_step_in_shares_is_the_same_game() {
        let Some((device, queue)) = test_gpu() else { return };
        let (seed, scale, walkers) = (5, 2, 5_000);
        let rules = Rules { cap: 20.0, frame_ticks: 2, ..Rules::default() };
        let play = |shares: u32| {
            let mut g = GpuWorld::new(&device, &queue, race_settings(seed, scale, walkers), scale);
            g.spread = shares;
            let mut game = Game::new(g, rules, seed);
            for k in 0..game.soldiers.len() {
                game.apply(Cmd::Order(k, Order::Attack));
            }
            let (mut log, mut split) = (Vec::new(), 0);
            let mut n = 0u32;
            while game.end.is_none() && game.time() < 20.0 {
                game.begin_step();
                split = split.max(game.world.shares_left());
                for _ in 0..n % 4 {
                    game.world.frame_more();
                }
                if n == 9 {
                    std::hint::black_box(game.world.mould_grid(8));
                }
                if n % 13 == 5 {
                    std::hint::black_box(game.world.hash());
                }
                game.finish_step();
                log.push((game.world.ticks(), game.mould, game.world.living(), game.harvested.to_bits(), game.shots, game.killed, game.alive()));
                n += 1;
            }
            (log, game.world.hash(), split)
        };
        let (whole, hash, split) = play(1);
        assert_eq!(split, 0, "whole steps leave no shares");
        let (shots, killed) = (whole.last().unwrap().4, whole.last().unwrap().5);
        eprintln!("shares: {} steps, {shots} shots, {killed} killed, hash {hash:016x}", whole.len());
        assert!(whole.len() > 100 && shots > 0 && killed > 0, "the squad fought ({} steps, {shots} shots, {killed} killed)", whole.len());
        for shares in [2, 4] {
            let (log, h, split) = play(shares);
            assert_eq!(split as u32, shares - 1, "{shares} shares: the later ones wait for frame_more");
            assert_eq!(log, whole, "{shares} shares: the same steps");
            assert_eq!(h, hash, "{shares} shares: the same world at the end");
        }
    }

    /// **The squad toolkit runs on the GPU world** (2026-11-12): a kit blast
    /// at the start kills the same walkers there as on the CPU race world
    /// (the same mould, before any tick) and lays its crater; a det cord
    /// lays one too; the gas (11-14) is paid and sleeps the mould. Then, as
    /// the game runs, a mine at the mould's home goes off from the frames'
    /// grid. The germ has its own test (`the_germ_runs_on_the_gpu_world`).
    #[test]
    fn the_kit_runs_on_the_gpu_world() {
        use crate::mould::kit::{Kit, KitRules, Tool};
        use crate::mould::larder_world::race_world;
        let Some((device, queue)) = test_gpu() else { return };
        let (seed, scale, walkers) = (5, 1, 2_000);
        let rules = Rules { cap: 30.0, ..Rules::default() };
        let mut gpu = Game::new(GpuWorld::new(&device, &queue, race_settings(seed, scale, walkers), scale), rules, seed);
        let mut cpu = Game::new(race_world(seed, scale, walkers), rules, seed);
        let home = gpu.world.crystals()[0].0;
        assert_eq!(home, cpu.world.crystals()[0].0);
        let free = KitRules { carry_max: 0, reach: 1_000.0, ..KitRules::default() };
        let (mut kg, mut kc) = (Kit::new(free), Kit::new(free));
        for g in [&mut gpu.bank, &mut cpu.bank] {
            *g = 10_000.0;
        }
        assert!(kg.use_tool(&mut gpu, Tool::Blast(home)) && kc.use_tool(&mut cpu, Tool::Blast(home)));
        // Within 1%: the kill test is a float distance (the GPU's sqrt, the
        // CPU's hypot), so a walker on the rim can fall either way (113
        // against 112 on seed 5). The crater is whole cells, so exact.
        assert!(kc.killed[0] > 50 && kg.killed[0].abs_diff(kc.killed[0]) * 100 <= kc.killed[0], "the blast killed {} on the GPU, {} on the CPU", kg.killed[0], kc.killed[0]);
        let cratered = |g: &Game<GpuWorld>| g.world.e.crater_of().iter().filter(|&&c| c > 0.0).count();
        let after_blast = cratered(&gpu);
        assert_eq!(after_blast, cpu.world.scorch.iter().filter(|&&c| c > 0).count(), "the blast's crater");
        let bank = gpu.bank;
        // The sleep gas works since 11-14 (`Engine::gas`) and is paid.
        assert!(kg.use_tool(&mut gpu, Tool::Gas(home)) && gpu.bank < bank && kg.gassed > 0, "the gas on the GPU: {} gassed", kg.gassed);
        let k = gpu.world.scale();
        assert!(kg.use_tool(&mut gpu, Tool::Cord((home.0 + 20.0 * k, home.1 - 10.0 * k), (home.0 + 20.0 * k, home.1 + 10.0 * k))));
        assert!(cratered(&gpu) > after_blast, "the cord's craters");
        // A mine on a fresh world, on the mould's home: the frames' grid
        // (one square a dish cell) sets it off.
        let mut g2 = Game::new(GpuWorld::new(&device, &queue, race_settings(seed, scale, walkers), scale), rules, seed);
        g2.bank = 10_000.0;
        let mut km = Kit::new(free);
        assert!(km.use_tool(&mut g2, Tool::Mine(home)));
        let mut fired = 0;
        while g2.end.is_none() && fired == 0 {
            g2.world.mould_grid(scale);
            g2.step();
            fired = km.frame(&mut g2);
        }
        assert_eq!(fired, 1, "the mine (trigger {}) went off by {:.0} s", km.mine_trigger(&g2), g2.time());
        eprintln!("GPU kit: blast {} killed, mine {} killed at {:.1} s", kg.killed[0], km.killed[1], g2.time());
    }

    /// **A batch of bursts counts each as alone, with one read-back**
    /// (2026-11-16, the gun-card plan's batching): a blast, a bomb's
    /// throw, a push, a gas and a burn on the mould's home, run as one
    /// `bursts` batch on one GPU world and one at a time (`burst`) on a
    /// twin: the same counts, burst by burst, and the same fire's kills;
    /// the batch reads back once where the singles read back once a
    /// counting burst. Against the CPU race world within 1% (the float
    /// distance at the rim, as the kit's blast).
    #[test]
    fn a_batch_of_bursts_counts_each_as_alone_with_one_read_back() {
        use crate::mould::kit::KitRules;
        use crate::mould::larder_world::race_world;
        let Some((device, queue)) = test_gpu() else { return };
        let (seed, scale, walkers) = (5, 1, 4_000);
        let mut batch = GpuWorld::new(&device, &queue, race_settings(seed, scale, walkers), scale);
        let mut one = GpuWorld::new(&device, &queue, race_settings(seed, scale, walkers), scale);
        let mut cpu = race_world(seed, scale, walkers);
        let home = batch.crystals()[0].0;
        let sc = batch.scale();
        let at = (home.0 / sc, home.1 / sc);
        let fine = 30.0 * batch.fine_steps() as f32;
        let k = squad::BurstRules { fling_speed: 1.5, ..crate::mould::rounds::burst_rules(&KitRules::default(), fine) };
        let off = |dx: f32, dy: f32| (at.0 + dx, at.1 + dy);
        let list = [
            squad::Burst { at: off(-8.0, 0.0), r: 4.0, kill: true, crater: 30, crater_r: 5.0, ..Default::default() },
            squad::Burst { at: off(6.0, 4.0), r: 2.0, fling: 8.0, fling_steps: 20, ..Default::default() },
            squad::Burst { at: off(0.0, -8.0), r: 1e-3, fling: 6.0, fling_steps: 15, push: Some(0.7), ..Default::default() },
            squad::Burst { at: off(-4.0, 8.0), r: 4.0, gas: 60, ..Default::default() },
            squad::Burst { at: off(10.0, -6.0), r: 3.0, burn: true, ..Default::default() },
            squad::Burst { at: off(2.0, 2.0), r: 3.0, kill: true, ..Default::default() },
        ];
        let list: Vec<(squad::Burst, squad::BurstRules)> = list.iter().map(|&b| (b, k)).collect();
        let r0 = crate::gpu::reads();
        let outs = batch.bursts(&list);
        let reads = crate::gpu::reads() - r0;
        let r1 = crate::gpu::reads();
        let singles: Vec<squad::BurstOut> = list.iter().map(|(b, k)| one.burst(b, k)).collect();
        let single_reads = crate::gpu::reads() - r1;
        let on_cpu: Vec<squad::BurstOut> = list.iter().map(|(b, k)| cpu.burst(b, k)).collect();
        eprintln!("batch {outs:?} ({reads} read-backs), one at a time {singles:?} ({single_reads}), CPU {on_cpu:?}");
        assert_eq!(outs, singles, "a batch counts each burst as it would alone");
        assert_eq!(reads, 1, "one read-back for the batch");
        assert!(single_reads >= 4, "one at a time reads back each counting burst: {single_reads}");
        // A burn's kills come with `burned` on the GPU (the CPU's burst
        // counts them itself).
        let fire = batch.burned();
        assert_eq!(fire, one.burned(), "the burn's kills are fire's either way");
        assert!(fire.abs_diff(on_cpu[4].killed) * 100 <= on_cpu[4].killed + 1 && fire > 10, "the burn killed {fire} on the GPU, {} on the CPU", on_cpu[4].killed);
        assert!(outs[0].killed > 20 && outs[1].flung > 20 && outs[2].flung > 10 && outs[3].gassed > 10, "each burst did something: {outs:?}");
        for ((g, c), (b, _)) in outs.iter().zip(&on_cpu).zip(&list) {
            let killed = if b.burn { 0 } else { c.killed };
            for (a, b) in [(g.killed, killed), (g.flung, c.flung), (g.gassed, c.gassed)] {
                assert!(a.abs_diff(b) * 100 <= b.max(1) + 1, "the GPU's {g:?} against the CPU's {c:?}");
            }
        }
    }

    /// **Sleep wears off** (2026-11-18, `BurstRules::gas_immune`; the
    /// owner's playtest: Mist on the turrets "basically stopped
    /// everything"): on the mould's home, a second gas while the first's
    /// sleep runs puts no walker to sleep again (it would have restamped
    /// every one), a gas after they wake finds them resting, and once the
    /// rest is over gas takes them again. With `gas_immune` 0 every gas
    /// takes them, the old rule.
    #[test]
    fn sleep_wears_off_on_the_gpu_world() {
        use crate::mould::kit::KitRules;
        let Some((device, queue)) = test_gpu() else { return };
        let (seed, scale, walkers) = (5, 1, 4_000);
        let rules = Rules { cap: 30.0, ..Rules::default() };
        let run = |immune: u16| -> Vec<u32> {
            let mut g = Game::new(GpuWorld::new(&device, &queue, race_settings(seed, scale, walkers), scale), rules, seed);
            let home = g.world.crystals()[0].0;
            let sc = g.world.scale();
            let fine = g.world.fine_steps() as u64;
            let k = squad::BurstRules { gas_immune: immune, ..crate::mould::rounds::burst_rules(&KitRules { gas_half: 0.0, ..KitRules::default() }, 30.0 * fine as f32) };
            let gas = squad::Burst { at: (home.0 / sc, home.1 / sc), r: 5.0, gas: 60, ..Default::default() };
            let mut out = vec![g.world.burst(&gas, &k).gassed, g.world.burst(&gas, &k).gassed];
            // Past the sleep (60 fine steps), inside the rest.
            for _ in 0..(90 / fine + 1) {
                g.step();
            }
            out.push(g.world.burst(&gas, &k).gassed);
            // Past the rest too.
            for _ in 0..(200 / fine + 1) {
                g.step();
            }
            out.push(g.world.burst(&gas, &k).gassed);
            out
        };
        let (old, new) = (run(0), run(120));
        eprintln!("gassed each time: rest 0 {old:?}, rest 120 fine steps {new:?}");
        assert!(old.iter().all(|&n| n > 20), "with no rest every gas takes them: {old:?}");
        assert!(new[0] > 20 && new[1] == 0, "a second gas while asleep takes none: {new:?}");
        assert!(new[2] * 2 < old[2], "a gas just after they wake finds them resting: {new:?} against {old:?}");
        assert!(new[3] > 20, "after the rest gas takes them again: {new:?}");
    }

    /// **Asleep in your own gas** (2026-11-18, `Rules::doze`): a thick
    /// sleep cloud laid on a soldier puts it to sleep the next frame: it
    /// fires no shot but still walks where it is sent (since 11-19), and
    /// once the cloud thins it wakes. With `doze` 0 it never sleeps.
    #[test]
    fn a_soldier_sleeps_in_its_own_gas_cloud() {
        use crate::mould::kit::KitRules;
        use crate::mould::larder_world::{Cmd, Order};
        let Some((device, queue)) = test_gpu() else { return };
        let (seed, scale, walkers) = (5, 1, 2_000);
        let run = |doze: f32| {
            let rules = Rules { cap: 30.0, doze, ..Rules::default() };
            let mut g = Game::new(GpuWorld::new(&device, &queue, race_settings(seed, scale, walkers), scale), rules, seed);
            let sc = g.world.scale();
            let fine = 30.0 * g.world.fine_steps() as f32;
            let k = crate::mould::rounds::burst_rules(&KitRules::default(), fine);
            let at = (g.soldiers[0].x, g.soldiers[0].y);
            g.world.burst(&squad::Burst { at: (at.0 / sc, at.1 / sc), r: 6.0, cloud: 4.0, ..Default::default() }, &k);
            g.step();
            g.step();
            let asleep = g.dozing(0);
            let level = g.soldiers[0].doze;
            // Sent to the crystal: one dozing still walks (11-19).
            let goal = g.world.crystals()[1].0;
            assert!(g.apply(Cmd::Order(0, Order::Move(goal.0, goal.1))));
            let before = (g.soldiers[0].x, g.soldiers[0].y);
            for _ in 0..5 {
                g.step();
            }
            let moved = (g.soldiers[0].x - before.0).hypot(g.soldiers[0].y - before.1);
            while g.time() < 60.0 && g.dozing(0) {
                g.step();
            }
            (asleep, level, moved, g.dozing(0), g.time())
        };
        let (asleep, level, moved, still, woke) = run(1.0);
        eprintln!("cloud on the soldier {level:.1} sleep levels; asleep {asleep}, moved {moved:.2} fine cells asleep; awake again at {woke:.1} s");
        assert!(asleep && level >= 1.0 && moved > 0.1, "asleep in the cloud yet walking: level {level}, moved {moved}");
        assert!(!still && woke < 60.0, "it wakes when the cloud thins");
        let (asleep, _, moved, _, _) = run(0.0);
        assert!(!asleep && moved > 0.1, "doze 0: it walks on ({moved})");
    }

    /// **The germ runs on the GPU world** (2026-11-14, cloud plan C5): a
    /// kit germ at the mould's home makes the same walkers sick there as
    /// on the CPU race world (the same mould, before any tick; within 1%,
    /// the float distance at the rim as the blast's), and then, as the
    /// game runs, sickness spreads through the germ field and its upkeep
    /// thins the mould against the same world with no germ, on both.
    #[test]
    fn the_germ_runs_on_the_gpu_world() {
        use crate::mould::kit::{Kit, KitRules, Tool};
        use crate::mould::big::BigDish;
        use crate::mould::larder_world::race_world;
        let Some((device, queue)) = test_gpu() else { return };
        let (seed, scale, walkers) = (5, 1, 2_000);
        let rules = Rules { cap: 60.0, ..Rules::default() };
        let gpu_world = || Game::new(GpuWorld::new(&device, &queue, race_settings(seed, scale, walkers), scale), rules, seed);
        let (mut gpu, mut gpu_none) = (gpu_world(), gpu_world());
        let (mut cpu, mut cpu_none) = (Game::new(race_world(seed, scale, walkers), rules, seed), Game::new(race_world(seed, scale, walkers), rules, seed));
        let home = gpu.world.crystals()[0].0;
        let free = KitRules { carry_max: 0, reach: 1_000.0, ..KitRules::default() };
        let (mut kg, mut kc) = (Kit::new(free), Kit::new(free));
        gpu.bank = 10_000.0;
        cpu.bank = 10_000.0;
        assert!(gpu.world.has_germ());
        assert!(kg.use_tool(&mut gpu, Tool::Germ(home)) && kc.use_tool(&mut cpu, Tool::Germ(home)));
        let immune = gpu.world.s.immune;
        let gpu_sick = |g: &Game<GpuWorld>| read_back(&g.world.e.device, &g.world.e.queue, &g.world.e.sick).chunks_exact(4).take(g.world.e.n as usize).filter(|c| u32::from_le_bytes((*c).try_into().unwrap()) > immune).count();
        let cpu_sick = |g: &Game<BigDish>| g.world.sick.iter().filter(|&&c| c as u32 > g.world.p.immune_steps as u32).count();
        let (sg, sc) = (gpu_sick(&gpu), cpu_sick(&cpu));
        eprintln!("germ: {sg} sick on the GPU, {sc} on the CPU");
        assert!(sc > 50 && sg.abs_diff(sc) * 100 <= sc, "the germ made {sg} sick on the GPU, {sc} on the CPU");
        assert_eq!((immune, gpu.world.s.sick_upkeep), (cpu.world.p.immune_steps as u32, cpu.world.p.sick_upkeep), "the same contagion settings");
        // Twenty game seconds: the sick pay 60 times the upkeep and die,
        // and the germ they lay catches more.
        let secs = 20.0;
        let mut most = (sg, sc);
        // Each world's own time too: with contagion on the germ field
        // spreads every tick (software Vulkan, so only a ratio).
        let mut took = [0.0f64; 2];
        while gpu.time() < secs {
            for (j, g) in [&mut gpu, &mut gpu_none].into_iter().enumerate() {
                let t = Instant::now();
                g.world.mould_grid(scale);
                g.step();
                took[j] += t.elapsed().as_secs_f64();
            }
            most.0 = most.0.max(gpu_sick(&gpu));
        }
        eprintln!("germ: GPU world {:.2} s with contagion on, {:.2} s off, for {secs} game s", took[0], took[1]);
        while cpu.time() < secs {
            cpu.step();
            cpu_none.step();
            most.1 = most.1.max(cpu_sick(&cpu));
        }
        let (lg, lgn, lc, lcn) = (gpu.world.living(), gpu_none.world.living(), cpu.world.living(), cpu_none.world.living());
        eprintln!("germ after {secs} s: GPU {lg} living against {lgn} with none (most sick {}), CPU {lc} against {lcn} (most sick {})", most.0, most.1);
        assert!(most.0 > sg && most.1 > sc, "the sickness spread (most sick {most:?}, from {sg} and {sc})");
        assert!(lg * 10 < lgn * 9 && lc * 10 < lcn * 9, "the germ thinned the mould: GPU {lg} against {lgn}, CPU {lc} against {lcn}");
    }

    /// **A germ infects only where it lands on the GPU world** (2026-11-17):
    /// a kit germ cast at 15 s into the race world's empty corner (scale
    /// 2, 25,000 walkers) makes no one sick 2 s later, on the GPU as on the
    /// CPU. Before the fix the GPU's first germ also seeded a 40-cell disc
    /// at the map's middle once the engine's tick count passed the dish's
    /// `Settings::seed_at` (4 s of fine steps: 16 s of the game's, as the
    /// world counts its ticks); the test sets that disc over the whole map,
    /// so a planted copy of the old code made 26,443 sick here.
    #[test]
    fn a_germ_infects_only_where_it_lands() {
        use crate::mould::kit::{Kit, KitRules, Tool};
        use crate::mould::big::BigDish;
        use crate::mould::larder_world::race_world;
        let Some((device, queue)) = test_gpu() else { return };
        let (seed, scale, walkers) = (0, 2, 25_000);
        let rules = Rules { cap: 60.0, ..Rules::default() };
        // The dish's timed seeding over the whole map, so a germ that set
        // it off would make everyone sick.
        let s = Settings { seed_r: 1.0e4, ..race_settings(seed, scale, walkers) };
        let mut gpu = Game::new(GpuWorld::new(&device, &queue, s, scale), rules, seed);
        let mut cpu = Game::new(race_world(seed, scale, walkers), rules, seed);
        let run = |gpu: &mut Game<GpuWorld>, cpu: &mut Game<BigDish>, to: f32| {
            while gpu.time() < to {
                gpu.world.mould_grid(scale);
                gpu.step();
            }
            while cpu.time() < to {
                cpu.step();
            }
        };
        run(&mut gpu, &mut cpu, 15.0);
        let corner = (6.0, 6.0);
        let kit = KitRules { carry_max: 0, reach: 1_000.0, ..KitRules::default() };
        let (mut kg, mut kc) = (Kit::new(kit), Kit::new(kit));
        gpu.bank = 10_000.0;
        cpu.bank = 10_000.0;
        assert!(kg.use_tool(&mut gpu, Tool::Germ(corner)) && kc.use_tool(&mut cpu, Tool::Germ(corner)));
        assert!(gpu.world.s.sick > 0, "the germ turned contagion on");
        run(&mut gpu, &mut cpu, 17.0);
        assert!(gpu.world.e.done as f32 >= gpu.world.s.seed_at * crate::mould::STEPS_PER_SECOND as f32, "past the old seeding's time");
        let immune = gpu.world.s.immune;
        let sg = read_back(&gpu.world.e.device, &gpu.world.e.queue, &gpu.world.e.sick).chunks_exact(4).take(gpu.world.e.n as usize).filter(|c| u32::from_le_bytes((*c).try_into().unwrap()) > immune).count();
        let sc = cpu.world.sick.iter().filter(|&&c| c as u32 > cpu.world.p.immune_steps as u32).count();
        eprintln!("germ in the corner at 15 s, 2 s on: {sg} sick on the GPU, {sc} on the CPU");
        assert_eq!((sg, sc), (0, 0), "a germ on empty ground makes no one sick");
    }

    /// **The first germ off the middle of a planet infects as on the CPU**
    /// (the technical lead's review of #416, 2026-11-17: the race world's
    /// twins cast near its middle or on empty ground, so the GPU-only germ
    /// at the map's middle went unseen). Planet 1:2, which is not square,
    /// warmed 12 s; the first germ round (`rounds::bursts`, the round's
    /// rules) cast on the living walker farthest from the map's middle,
    /// with slime at the middle; then on to past the dish's old timed
    /// seeding (`Settings::seed_at`, 4 s of fine steps, 480 engine ticks:
    /// 32 s of the game's at scale 1). The sick walkers
    /// match the CPU's slot for slot, each game second (a little slack on
    /// a card): 25 at the cast, 108 a second on, none 7 s on. With the old
    /// code (no `seeded` set) it failed 21 s on: 2,973 sick on the GPU
    /// (2,008 within 40 cells of the middle), none on the CPU. The other
    /// burst and round paths take their place from the cast; only the
    /// timed seeding used the middle.
    #[test]
    fn the_first_germ_off_a_planets_middle_infects_as_on_the_cpu() {
        use crate::mould::big::BigDish;
        use crate::mould::rounds;
        let Some((device, queue)) = test_gpu() else { return };
        let (seed, scale, walkers) = (2, 1, 8_000);
        let (_, m) = crate::mould::levelgen::spec("planet:1:2", seed).unwrap().unwrap();
        let mut cpu = BigDish::with_map(race_params(seed, scale, walkers), &m);
        let mut gpu = GpuWorld::with_map(&device, &queue, race_settings(seed, scale, walkers), scale, &m);
        assert!(cpu.w != cpu.h, "a planet that is not square: {} x {}", cpu.w, cpu.h);
        let f = Frame::default();
        let second = |cpu: &mut BigDish, gpu: &mut GpuWorld| {
            for w in [cpu as &mut dyn World, gpu as &mut dyn World] {
                w.frame(&f, 15);
            }
        };
        for _ in 0..12 {
            second(&mut cpu, &mut gpu);
        }
        let mid = (cpu.w as f32 / 2.0, cpu.h as f32 / 2.0);
        let living: Vec<usize> = (0..cpu.walkers.len()).filter(|&k| cpu.lives(k)).collect();
        let off = |k: usize| (cpu.walkers[k].0 - mid.0).hypot(cpu.walkers[k].1 - mid.1);
        let near_mid = living.iter().filter(|&&k| off(k) <= 20.0).count();
        let far = *living.iter().max_by(|&&a, &&b| off(a).total_cmp(&off(b))).unwrap();
        let at = (cpu.walkers[far].0, cpu.walkers[far].1);
        eprintln!("planet 1:2 {} x {} fine cells: the germ at {at:?}, {:.0} cells off the middle; {near_mid} walkers within 20 of the middle", cpu.w, cpu.h, off(far));
        assert!(near_mid > 20, "slime at the middle, for the old seeding to infect");
        assert!(off(far) > 0.3 * mid.0.min(mid.1), "the germ well off the middle");
        let rr = rounds::RoundRules::default();
        let fine = 15.0 * World::fine_steps(&cpu) as f32;
        let b = rounds::bursts(&rounds::casts(&rounds::parse("germ").unwrap())[0], at, &rr, scale as f32, fine)[0];
        let k = rr.burst_rules(fine);
        World::burst(&mut cpu, &b, &k);
        World::burst(&mut gpu, &b, &k);
        let sick_sets = |cpu: &BigDish, gpu: &GpuWorld| {
            let immune = gpu.s.immune;
            let n = gpu.e.n as usize;
            // Living walkers only, as the readers' films count them: a
            // dead walker keeps its count on the GPU (its slot free, its
            // energy below 0, or its state dead).
            let words = |b: &wgpu::Buffer| -> Vec<u32> { read_back(&gpu.e.device, &gpu.e.queue, b).chunks_exact(4).take(n).map(|c| u32::from_le_bytes(c.try_into().unwrap())).collect() };
            let (ill, spec) = (words(&gpu.e.sick), words(&gpu.e.spec));
            let en: Vec<f32> = words(&gpu.e.energy).into_iter().map(f32::from_bits).collect();
            let g: Vec<usize> = (0..n).filter(|&j| ill[j] > immune && (spec[j] >> 9) & 3 != 3 && en[j] >= 0.0).collect();
            let c: Vec<usize> = cpu.sick.iter().enumerate().filter(|&(j, &v)| v as u32 > cpu.p.immune_steps as u32 && cpu.lives(j)).map(|(j, _)| j).collect();
            (c, g)
        };
        let (c, g) = sick_sets(&cpu, &gpu);
        eprintln!("at the cast: {} sick on the CPU, {} on the GPU", c.len(), g.len());
        assert!(!c.is_empty() && c == g, "the cast's sick, slot for slot");
        let to = gpu.s.seed_at * crate::mould::STEPS_PER_SECOND as f32;
        let mut sec = 0;
        while (gpu.e.done as f32) < to + crate::mould::STEPS_PER_SECOND as f32 || sec < 4 {
            second(&mut cpu, &mut gpu);
            sec += 1;
            let (c, g) = sick_sets(&cpu, &gpu);
            let near = |v: &[usize], x: &dyn Fn(usize) -> (f32, f32)| v.iter().filter(|&&j| { let p = x(j); (p.0 - mid.0).hypot(p.1 - mid.1) <= 40.0 }).count();
            let wk: Vec<f32> = read_back(&gpu.e.device, &gpu.e.queue, &gpu.e.walkers).chunks_exact(4).map(|c| f32::from_le_bytes(c.try_into().unwrap())).collect();
            let (cm, gm) = (near(&c, &|j| (cpu.walkers[j].0, cpu.walkers[j].1)), near(&g, &|j| (wk[4 * j], wk[4 * j + 1])));
            eprintln!("{sec} s on: {} sick on the CPU ({cm} within 40 of the middle), {} on the GPU ({gm})", c.len(), g.len());
            let diff = c.iter().filter(|j| g.binary_search(j).is_err()).count() + g.iter().filter(|j| c.binary_search(j).is_err()).count();
            assert!(diff <= tol(0, 10 + c.len() / 20), "{sec} s on: the sick differ in {diff} slots (CPU {}, GPU {}; within 40 of the middle CPU {cm}, GPU {gm})", c.len(), g.len());
            assert!(sec < 40, "the old seeding's time never came");
        }
        assert!(gpu.e.done as f32 >= to, "past the old seeding's time");
    }

    /// **A frail mould dies of the germ on both engines** (2026-11-17,
    /// `germ_upkeep`, the Sleeper's): the same kit germ at the race
    /// world's home, at the germ round's upkeep of 10, with the mould's
    /// frailty 1 and 6. The frail mould pays 60 and thins; at 1 the sick
    /// mostly live; on both engines.
    #[test]
    fn a_frail_mould_dies_of_the_germ_on_both_engines() {
        use crate::mould::kit::{Kit, KitRules, Tool};
        use crate::mould::larder_world::race_world;
        let Some((device, queue)) = test_gpu() else { return };
        let (seed, scale, walkers) = (5, 1, 2_000);
        let rules = Rules { cap: 60.0, ..Rules::default() };
        let kit = KitRules { carry_max: 0, reach: 1_000.0, sick_upkeep: 10.0, ..KitRules::default() };
        let mut living = Vec::new();
        for frail in [1.0, 6.0] {
            let mut gpu = Game::new(GpuWorld::new(&device, &queue, Settings { germ_upkeep: frail, ..race_settings(seed, scale, walkers) }, scale), rules, seed);
            let mut dish = race_world(seed, scale, walkers);
            dish.p.germ_upkeep = frail;
            let mut cpu = Game::new(dish, rules, seed);
            let home = gpu.world.crystals()[0].0;
            let (mut kg, mut kc) = (Kit::new(kit), Kit::new(kit));
            gpu.bank = 10_000.0;
            cpu.bank = 10_000.0;
            assert!(kg.use_tool(&mut gpu, Tool::Germ(home)) && kc.use_tool(&mut cpu, Tool::Germ(home)));
            assert_eq!((gpu.world.s.sick_upkeep, gpu.world.s.germ_upkeep), (cpu.world.p.sick_upkeep, cpu.world.p.germ_upkeep), "the same contagion settings");
            while gpu.time() < 20.0 {
                gpu.world.mould_grid(scale);
                gpu.step();
            }
            while cpu.time() < 20.0 {
                cpu.step();
            }
            living.push((gpu.world.living(), cpu.world.living()));
        }
        let [(g1, c1), (g6, c6)] = [living[0], living[1]];
        eprintln!("frailty 1: GPU {g1} living, CPU {c1}; frailty 6: GPU {g6}, CPU {c6}");
        assert!(g6 * 10 < g1 * 9 && c6 * 10 < c1 * 9, "the frail mould thinned: GPU {g6} against {g1}, CPU {c6} against {c1}");
        // On software Vulkan both agreed to the walker (666 and 479); a
        // planted bug, the GPU sending `sick_upkeep` alone, left the frail
        // GPU mould at 666.
        // On a card a twelfth (the GTX 1070, 11-21: GPU 638 and 493, CPU
        // 666 and 479, a small dying mould's chaos in the card's order),
        // which still catches that bug (666 against 479).
        let part = tol(50, 12);
        assert!(g6.abs_diff(c6) * part <= c6 && g1.abs_diff(c1) * part <= c1, "the engines agree: GPU {g1} and {g6}, CPU {c1} and {c6}");
    }

    /// The engine runs the CPU race world's mould: the same params, bar
    /// the fine steps a tick, which the GPU world makes up by running
    /// `scale` engine ticks a game tick.
    #[test]
    fn race_settings_give_the_race_worlds_params() {
        for (seed, scale, walkers) in [(1, 2, 5_000), (7, 1, 1_000), (3, 4, 20_000)] {
            let cpu = race_params(seed, scale, walkers);
            let gpu = BigParams { substeps: cpu.substeps, ..engine_params(&race_settings(seed, scale, walkers), scale) };
            assert_eq!(format!("{gpu:?}"), format!("{cpu:?}"), "seed {seed} scale {scale}");
        }
    }

    /// **A lure draws the calm slime down its field** (`Settings::
    /// lure_pull`, `Game::time_lure_paths`; 2026-11-18, the Enemy lane). A
    /// rule of the GPU engine alone (the owner's card, 11-17 19:21Z: no CPU
    /// twin), so its test is what it does on the software GPU. Three race
    /// worlds alike: pull 10 with a lure mark 25 dish cells from the
    /// mould's home for 12 game seconds, pull 10 with no lure, and pull 0
    /// with the mark.
    ///
    /// - The field is set a game second after it is started (so the game
    ///   does not depend on the worker thread): no pull in the engine
    ///   before second 2, the pull and the field after, 0 walking at the
    ///   mark's block and nothing past the reach.
    /// - At 12 s the lured slime has several times the walkers round the
    ///   mark that the unlured has.
    /// - The mark runs out at 12 s; by 14 s the field is cleared and the
    ///   pull is off again.
    /// - Pull 0 builds no field, and plays the unlured game to the bit.
    #[test]
    fn a_lure_draws_the_calm_slime_down_its_field() {
        use crate::gpu::{floats, DENSE_HEAD};
        use crate::mould::big::RAID_BLOCK;
        use crate::mould::larder_world::LureMark;
        let Some((device, queue)) = test_gpu() else { return };
        let (seed, scale, walkers) = (5, 1, 2_000);
        let k = scale as f32;
        let play = |pull: f32, mark: bool| {
            let s = Settings { lure_pull: pull, ..race_settings(seed, scale, walkers) };
            let mut g = Game::new(GpuWorld::new(&device, &queue, s, scale), Rules::default(), seed);
            let home = g.world.crystals()[0].0;
            let (w, _) = g.world.size();
            // Toward the middle of the world, so the mark is on it.
            let at = (if home.0 < w as f32 / 2.0 { home.0 + 25.0 * k } else { home.0 - 25.0 * k }, home.1);
            let tps = g.rules.tps as u64;
            if mark {
                g.lure_marks.push(LureMark { at, until: 12 * tps });
            }
            (g, at)
        };
        let to = |g: &mut Game<GpuWorld>, secs: f32| {
            while g.time() < secs {
                g.step();
            }
        };
        let near = |g: &Game<GpuWorld>, at: (f32, f32), r: f32| {
            let e = &g.world.e;
            let wk = floats(&read_back(&e.device, &e.queue, &e.walkers));
            let en = floats(&read_back(&e.device, &e.queue, &e.energy));
            (0..e.n as usize).filter(|&q| (!e.grows || en[q] >= 0.0) && (wk[4 * q] - at.0).hypot(wk[4 * q + 1] - at.1) <= r).count()
        };
        // The lure's blocks as the engine holds them (the cells' region's
        // third quarter, engine.wgsl's `lure_dist`).
        let blocks = |g: &Game<GpuWorld>| {
            let e = &g.world.e;
            let nb = e.dish.w.div_ceil(RAID_BLOCK) * e.dish.h.div_ceil(RAID_BLOCK);
            let from = e.dense_at as usize + 4 * (DENSE_HEAD + e.n as usize + 2 * nb);
            read_back(&e.device, &e.queue, &e.news)[from..from + 4 * nb].chunks_exact(4).map(|b| u32::from_le_bytes(b.try_into().unwrap())).collect::<Vec<u32>>()
        };
        let (mut lured, at) = play(10.0, true);
        let (mut bare, _) = play(10.0, false);
        let (mut off, _) = play(0.0, true);
        assert!(lured.world.lure_pull() && !off.world.lure_pull());
        // Started at the first second, set at the second.
        to(&mut lured, 1.5);
        assert!(!lured.world.e.lured, "the field is not set in its first second");
        to(&mut lured, 2.5);
        assert!(lured.world.e.lured, "the field is set a second after it was started");
        let f = blocks(&lured);
        let bw = lured.world.e.dish.w.div_ceil(RAID_BLOCK);
        let q = (at.1 as usize / RAID_BLOCK) * bw + at.0 as usize / RAID_BLOCK;
        let reach = (lured.rules.lure_reach * k) as u32;
        let (reached, far) = (f.iter().filter(|&&v| v > 0).count(), f.iter().copied().max().unwrap_or(0));
        eprintln!("lure field: {reached} of {} blocks reached, {} at the mark, the farthest {far} (reach {reach})", f.len(), f[q]);
        assert!(f[q] >= 1 && f[q] <= 2 * RAID_BLOCK as u32 * crate::mould::big::UNITS_REACH as u32 + 4, "0 walking at the mark's block, or the open block beside it ({})", f[q]);
        assert!(reached > 50 && reached < f.len() && far <= reach + 1, "the field stops at the lure's reach");
        // Twelve seconds of pull.
        for g in [&mut lured, &mut bare, &mut off] {
            to(g, 12.0);
        }
        let r = 8.0 * k;
        let (with, without) = (near(&lured, at, r), near(&bare, at, r));
        eprintln!("lure: at 12 s {with} walkers within {r} of the mark with it, {without} without ({} and {} alive)", lured.world.e.alive(), bare.world.e.alive());
        assert!(with >= 3 * without + 30, "the lure draws the slime: {with} walkers round it against {without}");
        // The mark has run out: cleared a second after the last field.
        to(&mut lured, 14.5);
        assert!(lured.lure_marks.is_empty() && !lured.world.e.lured, "the field is cleared when the last lure goes");
        assert!(blocks(&lured).iter().all(|&v| v == 0), "and its blocks are clear");
        // Pull 0: no field, and the unlured game to the bit (on software
        // Vulkan; a card orders its float sums its own way).
        assert!(!off.world.e.lured && blocks(&off).iter().all(|&v| v == 0), "pull 0 builds no field");
        if !ON_CARD.load(std::sync::atomic::Ordering::Relaxed) {
            assert_eq!(off.world.hash(), bare.world.hash(), "pull 0 is the game with no lure");
        }
    }

    /// **The wave's field draws the calm slime, and a raid leaves it**
    /// (`Engine::write_wave_field`, `Game::time_waves`; 2026-11-20). A rule
    /// of the GPU engine alone, so its test is what it does on the software
    /// GPU. Two race worlds alike, each given at 1 s a field to a point 25
    /// dish cells from the mould's home, one at pull 40 and one at pull 0.
    ///
    /// - With pull 40 the field is held and its pull is sent; at pull 0 too
    ///   the blocks are written but no walker leans.
    /// - At 11 s the pulled slime has several times the walkers round the
    ///   point that the other has.
    /// - A raid's field written after it (`write_raid_field`) leaves the
    ///   wave's blocks as they were (they share the raid's region).
    /// - An empty field clears it.
    #[test]
    fn a_wave_field_draws_the_calm_slime_and_a_raid_leaves_it() {
        use crate::gpu::{floats, DENSE_HEAD};
        use crate::mould::big::RAID_BLOCK;
        let Some((device, queue)) = test_gpu() else { return };
        let (seed, scale, walkers) = (5, 1, 2_000);
        let k = scale as f32;
        let new = || {
            let s = race_settings(seed, scale, walkers);
            Game::new(GpuWorld::new(&device, &queue, s, scale), Rules::default(), seed)
        };
        let to = |g: &mut Game<GpuWorld>, secs: f32| {
            while g.time() < secs {
                g.step();
            }
        };
        let near = |g: &Game<GpuWorld>, at: (f32, f32), r: f32| {
            let e = &g.world.e;
            let wk = floats(&read_back(&e.device, &e.queue, &e.walkers));
            let en = floats(&read_back(&e.device, &e.queue, &e.energy));
            (0..e.n as usize).filter(|&q| (!e.grows || en[q] >= 0.0) && (wk[4 * q] - at.0).hypot(wk[4 * q + 1] - at.1) <= r).count()
        };
        // The wave's blocks as the engine holds them (the raid's region,
        // after its own blocks; engine.wgsl's `wave_dist`).
        let blocks = |g: &Game<GpuWorld>| {
            let e = &g.world.e;
            let nb = e.dish.w.div_ceil(RAID_BLOCK) * e.dish.h.div_ceil(RAID_BLOCK);
            let from = e.dense_at as usize + 4 * (DENSE_HEAD + e.n as usize + e.cells as usize + nb);
            read_back(&e.device, &e.queue, &e.news)[from..from + 4 * nb].chunks_exact(4).map(|b| u32::from_le_bytes(b.try_into().unwrap())).collect::<Vec<u32>>()
        };
        let (mut pulled, mut still) = (new(), new());
        assert!(pulled.world.can_wave());
        let home = pulled.world.crystals()[0].0;
        let (w, _) = pulled.world.size();
        let at = (if home.0 < w as f32 / 2.0 { home.0 + 25.0 * k } else { home.0 - 25.0 * k }, home.1);
        let cell = (at.0 as u32) | ((at.1 as u32) << 16);
        for (g, pull) in [(&mut pulled, 40.0), (&mut still, 0.0)] {
            to(g, 1.0);
            let (agar, gw, gh) = g.world.raid_ground().expect("the race world has ground");
            g.world.set_wave_field(crate::mould::big::units_field_for(&agar, gw, gh, &[cell]), pull);
        }
        assert!(pulled.world.e.waved && pulled.world.e.wave_w == 40.0);
        let before = blocks(&pulled);
        assert!(before.iter().filter(|&&v| v > 0).count() > 50, "the field is written");
        pulled.world.e.write_raid_field();
        assert_eq!(blocks(&pulled), before, "a raid's field leaves the wave's blocks");
        to(&mut pulled, 11.0);
        to(&mut still, 11.0);
        let r = 8.0 * k;
        let (with, without) = (near(&pulled, at, r), near(&still, at, r));
        eprintln!("wave: at 11 s {with} walkers within {r} of the point with pull 40, {without} with 0 ({} and {} alive)", pulled.world.e.alive(), still.world.e.alive());
        assert!(with >= 3 * without + 30, "the wave draws the slime: {with} walkers round its point against {without}");
        pulled.world.set_wave_field(Vec::new(), 0.0);
        assert!(!pulled.world.e.waved && blocks(&pulled).iter().all(|&v| v == 0), "an empty field clears it");
    }

    /// **The slime's edge swells and ebbs with its heartbeat**
    /// (`Engine::heart`, `Game::time_heart`; 2026-11-20). Two race worlds
    /// alike, one given a heart at the mould's home at 1 s (lean 25, a beat
    /// every 4 game seconds, 80 fine cells long), the other none. Over the
    /// next 8 s, the walkers' mean distance from the home (read every half
    /// second) swings at least twice as far with the heart as without.
    #[test]
    fn the_slime_edge_swells_and_ebbs_with_its_heartbeat() {
        use crate::gpu::floats;
        let Some((device, queue)) = test_gpu() else { return };
        let (seed, scale, walkers) = (5, 1, 2_000);
        let new = || Game::new(GpuWorld::new(&device, &queue, race_settings(seed, scale, walkers), scale), Rules::default(), seed);
        let mean = |g: &Game<GpuWorld>, at: (f32, f32)| {
            let e = &g.world.e;
            let wk = floats(&read_back(&e.device, &e.queue, &e.walkers));
            let en = floats(&read_back(&e.device, &e.queue, &e.energy));
            let ds: Vec<f32> = (0..e.n as usize).filter(|&q| !e.grows || en[q] >= 0.0).map(|q| (wk[4 * q] - at.0).hypot(wk[4 * q + 1] - at.1)).collect();
            ds.iter().sum::<f32>() / ds.len().max(1) as f32
        };
        let swing = |beats: bool| {
            let mut g = new();
            let home = g.world.crystals()[0].0;
            while g.time() < 1.0 {
                g.step();
            }
            if beats {
                let steps = 4 * g.rules.tps * g.world.fine_steps().max(1);
                g.world.set_heart(Some(home), 25.0, steps, 80);
            }
            let mut m: Vec<f32> = Vec::new();
            for k in 1..=16 {
                while g.time() < 1.0 + 0.5 * k as f32 {
                    g.step();
                }
                m.push(mean(&g, home));
            }
            m.iter().copied().fold(f32::MIN, f32::max) - m.iter().copied().fold(f32::MAX, f32::min)
        };
        let (with, without) = (swing(true), swing(false));
        eprintln!("heart: the mean distance from home swings {with:.2} fine cells with a heart, {without:.2} without");
        assert!(with >= 2.0 * without && with > 0.5, "the heart moves the body: {with:.2} against {without:.2}");
    }

    /// **Condensed strikers take in their neighbours, charge, and take
    /// more blows** (`Engine::condense`, `Game::time_strikers`;
    /// 2026-11-20). A rule of the GPU engine alone (the owner's card, 11-17
    /// 19:21Z), so its test is what it does on the software GPU. A race
    /// world with one soldier standing 25 fine cells east of the mould's
    /// home (it neither shoots nor moves); at 2 s the edge within 20 cells
    /// of it condenses (every walker there tries).
    ///
    /// - The slime loses exactly the walkers taken in, and the condensed
    ///   hold them all: no walker is made or lost.
    /// - Over the next 6 s the condensed walkers close on the soldier (their
    ///   mean distance at least halves) while the plain slime does not.
    /// - A strike that kills every plain walker in its disc leaves most
    ///   condensed walkers in it alive, each holding one walker fewer.
    #[test]
    fn condensed_strikers_take_in_their_neighbours_charge_and_take_more_blows() {
        use crate::gpu::floats;
        let Some((device, queue)) = test_gpu() else { return };
        let (seed, scale, walkers) = (5, 1, 2_000);
        let mut w = GpuWorld::new(&device, &queue, race_settings(seed, scale, walkers), scale);
        let home = w.crystals()[0].0;
        let at = (home.0 + 25.0, home.1);
        let f = Frame { units: vec![squad::UnitIn { x: at.0, y: at.1, touch: 1.5, ..Default::default() }], ..Frame::default() };
        // The living walkers: (x, y, condensed, held).
        let read = |w: &GpuWorld| -> Vec<(f32, f32, bool, u32)> {
            let e = &w.e;
            let wk = floats(&read_back(&e.device, &e.queue, &e.walkers));
            let en = floats(&read_back(&e.device, &e.queue, &e.energy));
            let life = e.life_of().0;
            let dw = e.dense_of();
            (0..e.n as usize)
                .filter(|&q| (!e.grows || en[q] >= 0.0) && (!e.cycles || life[q] != crate::mould::big::LIFE_DEAD))
                .map(|q| (wk[4 * q], wk[4 * q + 1], dw[q] & 0x10000 != 0, dw[q] >> 28))
                .collect()
        };
        let mean = |v: &[(f32, f32, bool, u32)], cond: bool| {
            let d: Vec<f32> = v.iter().filter(|p| p.2 == cond).map(|p| (p.0 - at.0).hypot(p.1 - at.1)).collect();
            d.iter().sum::<f32>() / d.len().max(1) as f32
        };
        for _ in 0..2 {
            w.frame(&f, 15);
        }
        assert_eq!(w.condense(20.0, 1.0, 3), None, "no condensing with the strikers off");
        w.set_strikers(Some(1.5));
        w.frame(&f, 1);
        let before = read(&w).len();
        let (_, _, took, band) = w.condense(20.0, 1.0, 3).expect("the GPU world condenses");
        let v = read(&w);
        let (live, held) = (v.iter().filter(|p| p.2).count(), v.iter().filter(|p| p.2).map(|p| p.3).sum::<u32>());
        eprintln!("strikers: {before} walkers, {took} taken in by {live} condensed holding {held}, {} after", v.len());
        assert!(took >= 30 && live >= 10, "the edge condenses: {took} taken in by {live}");
        assert!(band >= took + live as u32, "the band counts every walker that could try: {band}, {took} taken in by {live}");
        assert_eq!(before - v.len(), took as usize, "the slime loses only the walkers taken in");
        assert_eq!(held, took, "and the condensed hold them all");
        w.frame(&f, 1);
        assert_eq!(w.mass(), w.living() + held as usize, "the slime's mass counts what they hold");
        assert!(v.iter().filter(|p| p.2).all(|p| (1..=3).contains(&p.3)), "each holds one to three");
        let (c0, p0) = (mean(&v, true), mean(&v, false));
        for _ in 0..6 {
            w.frame(&f, 15);
        }
        let v = read(&w);
        let (c1, p1) = (mean(&v, true), mean(&v, false));
        eprintln!("strikers: mean distance to the soldier, condensed {c0:.1} -> {c1:.1}, plain {p0:.1} -> {p1:.1}");
        assert!(c1 <= 0.5 * c0, "the condensed charge the soldier: {c0:.1} -> {c1:.1}");
        assert!(p1 >= 0.8 * p0, "the plain slime stays: {p0:.1} -> {p1:.1}");
        // A strike over the soldier.
        let r = 6.0;
        let inside = |v: &[(f32, f32, bool, u32)], cond: bool| v.iter().filter(|p| p.2 == cond && (p.0 - at.0).hypot(p.1 - at.1) <= r - 1.0).count();
        let (cin, pin) = (inside(&v, true), inside(&v, false));
        let held0: u32 = v.iter().filter(|p| p.2 && (p.0 - at.0).hypot(p.1 - at.1) <= r - 1.0).map(|p| p.3).sum();
        w.burst(&squad::Burst { at, r, kill: true, ..Default::default() }, &squad::BurstRules::default());
        let v = read(&w);
        let (cout, pout) = (inside(&v, true), inside(&v, false));
        let held1: u32 = v.iter().filter(|p| p.2 && (p.0 - at.0).hypot(p.1 - at.1) <= r - 1.0).map(|p| p.3).sum();
        eprintln!("strikers: a strike over the soldier leaves {cout} of {cin} condensed (holding {held0} -> {held1}) and {pout} of {pin} plain");
        assert!(cin >= 5 && cout == cin && held1 + cin as u32 == held0, "every condensed walker in it takes the blow and lives, holding one fewer");
        assert!(pout * 10 <= pin, "the plain walkers in it die: {pout} of {pin} left");
        // A lump formed over all of them takes no striker (joining cleared
        // the walkers a striker held, so a mini-boss or a fruit formed by
        // strikers took that slime off the map).
        let held2: u32 = v.iter().filter(|p| p.2).map(|p| p.3).sum();
        w.form_lump(0, Some((at, 400.0)));
        w.frame(&f, 1);
        let v = read(&w);
        let held3: u32 = v.iter().filter(|p| p.2).map(|p| p.3).sum();
        assert_eq!(held3, held2, "a lump formed over the strikers leaves them and what they hold");
        w.form_lump(0, None);
    }

    /// **A walker born where a striker took one in is counted** (the bug
    /// check, 2026-11-20). Condensing marks the walkers it takes in dead,
    /// as a strike does; a child born into such a slot takes its parent's
    /// life state only with the life states on (`born`), so until this the
    /// child of a world that condensed before anything turned them on
    /// (every pass that kills does; in a game the first shot's spray does)
    /// kept the dead mark: it lived and moved but was never counted. A
    /// nearly full world, so the births land in those slots.
    #[test]
    fn walkers_born_where_strikers_took_walkers_in_are_counted() {
        use crate::gpu::floats;
        let Some((device, queue)) = test_gpu() else { return };
        let (seed, scale, walkers) = (5, 1, 2_000);
        let s = Settings { capacity: 2_100, ..race_settings(seed, scale, walkers) };
        let mut w = GpuWorld::new(&device, &queue, s, scale);
        let home = w.crystals()[0].0;
        let at = (home.0 + 25.0, home.1);
        let f = Frame { units: vec![squad::UnitIn { x: at.0, y: at.1, touch: 1.5, ..Default::default() }], ..Frame::default() };
        for _ in 0..2 {
            w.frame(&f, 15);
        }
        w.set_strikers(Some(1.5));
        // By position, not by pattern: the slime thread's #598 adds a
        // fourth number (the band) and either may land first.
        let took = w.condense(20.0, 1.0, 3).expect("the GPU world condenses").2;
        assert!(took >= 30, "the edge condenses: {took} taken in");
        for _ in 0..12 {
            w.frame(&f, 15);
        }
        let e = &w.e;
        let en = floats(&read_back(&e.device, &e.queue, &e.energy));
        let life = e.life_of().0;
        let live = (0..e.n as usize).filter(|&q| en[q] >= 0.0).count();
        let marked = (0..e.n as usize).filter(|&q| en[q] >= 0.0 && life[q] == crate::mould::big::LIFE_DEAD).count();
        eprintln!("strikers on a full world: {took} taken in; {live} slots live, {marked} of them marked dead, counted {}", w.living());
        assert!(live > 2_000 - took as usize, "the slime was born into the room: {live} live");
        assert_eq!(marked, 0, "no walker born into a taken-in walker's slot keeps its dead mark");
        assert_eq!(w.living(), live, "so the count sees every one");
    }

    /// **The last-stand heart** (2026-11-20, `larder_world/stand.rs`), a
    /// rule of the GPU engine alone (the owner's card, 11-17 19:21Z), so its
    /// test is what it does on the software GPU: a race world with its
    /// soldiers, the heart's share and army set so it gathers at once, a
    /// lunge every 4 s for 3 s.
    ///
    /// - It forms on the heart crystal and takes in nearly every walker.
    /// - Its members gather: their spread round its centre shrinks.
    /// - Lunging, its centre closes on the nearest soldier; at rest after,
    ///   it draws back toward its crystal.
    #[test]
    fn the_last_stand_heart_gathers_the_slime_and_lunges() {
        use crate::mould::larder_world::StandRules;
        let Some((device, queue)) = test_gpu() else { return };
        let (seed, scale, walkers) = (5, 1, 3_000);
        let mut rules = Rules { cap: 120.0, ..Rules::default() };
        rules.stand = StandRules { on: true, at: 2.0, army: 1.0, every: 4.0, lunge: 8.0, ..StandRules::default() };
        let mut g = Game::new(GpuWorld::new(&device, &queue, race_settings(seed, scale, walkers), scale), rules, seed);
        // Its soldiers stand still (no orders), so the lunge's aim holds.
        let near = |g: &Game<GpuWorld>, c: (f32, f32)| g.soldiers.iter().filter(|s| s.alive).map(|s| (s.x - c.0).hypot(s.y - c.1)).fold(f32::INFINITY, f32::min);
        while g.stand.live.is_none_or(|h| h.most == 0) && g.time() < 10.0 {
            g.step();
        }
        let h = g.stand.live.expect("the heart forms");
        let home = h.home;
        let first = (h.members, (h.centre.0 - home.0).hypot(h.centre.1 - home.1));
        let mass0 = g.mould;
        // Gathering, up to its first lunge.
        while g.stand.live.is_some_and(|h| h.lunging.is_none()) && g.time() < 30.0 {
            g.step();
        }
        let h = g.stand.live.expect("the heart lives");
        let (rest_c, rest_n) = (h.centre, h.members);
        let d0 = near(&g, rest_c);
        while g.stand.live.is_some_and(|h| h.lunging.is_some()) && g.time() < 50.0 {
            g.step();
        }
        let h = g.stand.live.expect("the heart lives through its lunge");
        let d1 = near(&g, h.centre);
        let out = (h.centre.0 - home.0).hypot(h.centre.1 - home.1);
        for _ in 0..(3 * g.rules.tps) {
            g.step();
        }
        let back = g.stand.live.map(|h| (h.centre.0 - home.0).hypot(h.centre.1 - home.1)).unwrap_or(0.0);
        eprintln!("heart: formed with {} of {mass0} walkers {:.1} fine cells from its crystal; {rest_n} members at its first lunge; nearest soldier {d0:.1} -> {d1:.1} lunging; {out:.1} from its crystal after, {back:.1} 3 s later; {} lunges", first.0, first.1, g.stand.lunges);
        assert!(rest_n as f32 >= 0.8 * mass0 as f32, "it takes in nearly all the slime: {rest_n} of {mass0}");
        assert!(d1 < 0.8 * d0, "lunging, it closes on the soldier: {d0:.1} -> {d1:.1}");
        assert!(back < out, "at rest, it draws back home: {out:.1} -> {back:.1}");
    }

    /// **A lunge's cap** (`StandRules::cap`, 2026-11-20 evening, the
    /// balance thread): the same race world, soldiers standing still, the
    /// heart gathering at once and lunging after 4 s for 30 s (it needs
    /// about 10 s to reach them). Uncapped (`cap` 1, `cap_most` 1000: the
    /// whole squad, and never sated) its lunge bites more of the squad to
    /// death than capped at a quarter; capped, the lunge draws back once it
    /// has taken a quarter of the squad's health (before its 30 s are up),
    /// sated for 5 s, and at most one soldier more than its cap dies.
    #[test]
    fn a_heart_lunge_draws_back_once_it_has_bitten_its_cap() {
        use crate::mould::larder_world::StandRules;
        let Some((device, queue)) = test_gpu() else { return };
        let (seed, scale, walkers) = (5, 1, 3_000);
        let play = |cap: f32, most: f32, sated: f32| {
            let mut rules = Rules { cap: 120.0, ..Rules::default() };
            rules.stand = StandRules { on: true, at: 2.0, army: 1.0, every: 4.0, lunge: 30.0, cap, cap_most: most, sated, ..StandRules::default() };
            let mut g = Game::new(GpuWorld::new(&device, &queue, race_settings(seed, scale, walkers), scale), rules, seed);
            let squad = g.soldiers.iter().filter(|s| s.alive).count();
            while g.stand.live.is_none_or(|h| h.lunging.is_none()) && g.time() < 30.0 {
                g.step();
            }
            let (start, bit0) = (g.time(), g.fell_bitten);
            while g.stand.live.is_some_and(|h| h.lunging.is_some()) && g.time() < start + 31.0 {
                g.step();
            }
            let drew_back = g.time() - start;
            while g.time() < start + 32.0 {
                g.step();
            }
            (squad, g.fell_bitten - bit0, drew_back)
        };
        let (squad, capped, drew_back) = play(0.25, 20.0, 5.0);
        let (_, uncapped, lunged) = play(1.0, 1000.0, 0.0);
        let cap = ((0.25 * squad as f32).ceil() as u32).max(1);
        eprintln!("heart lunge: {squad} soldiers; capped at {cap}: {capped} bitten to death, drew back after {drew_back:.1} s; uncapped: {uncapped}, lunged {lunged:.1} s");
        assert!(capped <= cap + 1, "capped, it takes about its cap: {capped} against a cap of {cap}");
        assert!(uncapped > capped, "uncapped, it takes more: {uncapped} against {capped}");
        assert!(drew_back < 30.0, "it drew back before its lunge was up: {drew_back:.1} s");
    }

    /// **Deep water is a wall to the slime** (`Settings::deep_wall`,
    /// 2026-11-20, the owner: "Deep water shouldn't be traversable by
    /// player or slime"). A rule of the GPU engine alone (the owner's card,
    /// 11-17 19:21Z: no CPU twin), so its test is what it does on the
    /// software GPU. The river map (`map::river`, a deep band 16 dish cells
    /// wide with a ford) with the slime's start moved to 15 dish cells from
    /// its east bank, so its edge starts in the water; on and off alike for 20
    /// game seconds:
    ///
    /// - On, no walker goes into the deep water (the count in it never
    ///   rises) and those that start in it swim out (most are out by 20 s);
    ///   off, the slime stays in it and spreads through it.
    /// - On, the slime's ways (`raid_ground`) are shut over the deep water
    ///   and open over the ford.
    #[test]
    fn deep_water_is_a_wall_to_the_slime() {
        use crate::gpu::floats;
        let Some((device, queue)) = test_gpu() else { return };
        let (seed, scale, walkers) = (5, 1, 4_000);
        let mut m = crate::mould::map::river(true, false);
        m.items.moulds = vec![(604.0, 200.0)];
        let play = |on: bool| {
            let s = Settings { deep_wall: on, ..race_settings(seed, scale, walkers) };
            Game::new(GpuWorld::with_map(&device, &queue, s, scale, &m), Rules::default(), seed)
        };
        let in_deep = |g: &Game<GpuWorld>| {
            let e = &g.world.e;
            let wk = floats(&read_back(&e.device, &e.queue, &e.walkers));
            let en = floats(&read_back(&e.device, &e.queue, &e.energy));
            let d = &e.dish;
            (0..e.n as usize)
                .filter(|&q| (!e.grows || en[q] >= 0.0) && d.deep((wk[4 * q + 1] as usize).min(d.h - 1) * d.w + (wk[4 * q] as usize).min(d.w - 1)))
                .count()
        };
        let (mut on, mut off) = (play(true), play(false));
        let start = in_deep(&on);
        assert!(start >= 50 && in_deep(&off) == start, "the slime's edge starts in the water: {start} walkers");
        // A map cell's fine cell (a dish cell is 4 map cells, at scale 1).
        let (ground, w, _) = on.world.raid_ground().unwrap();
        let at = |x: usize, y: usize| (y / 4) * w + x / 4;
        assert!(!ground[at(512, 200)] && ground[at(512, 320)] && ground[at(460, 200)], "the ways are shut over deep water, open over the ford and the bank");
        assert!(off.world.raid_ground().unwrap().0[at(512, 200)], "off, the ways cross it as before");
        let (mut last, mut without) = (start, 0);
        for secs in [2.0, 5.0, 10.0, 15.0, 20.0] {
            for g in [&mut on, &mut off] {
                while g.time() < secs {
                    g.step();
                }
            }
            let (a, b) = (in_deep(&on), in_deep(&off));
            eprintln!("deep water at {secs} s: {a} walkers in it with the wall, {b} without (of {start} at the start; {} and {} alive)", on.world.living(), off.world.living());
            assert!(a <= last, "with the wall none goes into the water: {a} at {secs} s, {last} before");
            (last, without) = (a, b);
        }
        assert!(last * 2 <= start, "with the wall most swim out: {last} of {start} left");
        assert!(without >= start, "without the wall the slime stays and spreads in the water: {without} against {start}");
    }

    /// **The tether** (`Settings::tether`, 2026-11-18, the Enemy lane, from
    /// the Lab's one-body prototype; the owner's play notes that day: slime
    /// that leaves the blob as "individual walkers ... is problematic"). A
    /// rule of the GPU engine alone, so its test is what it does on the
    /// software GPU. Planet 1's slime (shape C on the race world with a
    /// run's planet keys, so breakaways), tether 2 and 0, alike.
    ///
    /// - A raid is not tethered (it goes as one body by its march, and a
    ///   tethered file stays home): a raid sent at one game second leaves
    ///   the two games the same to the bit (on software Vulkan) until the
    ///   first breakaway.
    /// - With breakaways on, the tether changes the game by 20 s. What it
    ///   does to the picture is a film's: on this small world's slime a
    ///   breakaway is loose about one time in a hundred either way.
    #[test]
    fn a_tethered_breakaway_goes_out_touching_the_body() {
        use crate::mould::larder_world::shape_keys;
        let Some((device, queue)) = test_gpu() else { return };
        let (seed, scale, walkers) = (5, 2, 5_000);
        let k = scale as f32;
        let play = |tether: u32, roam: bool| {
            let mut s = race_settings(seed, scale, walkers);
            for (key, v) in shape_keys("c").unwrap().1 {
                crate::gpu::set(&mut s, key, v).unwrap();
            }
            for (key, v) in crate::mould::run::enemy_keys(1) {
                crate::gpu::set(&mut s, key, &v).unwrap();
            }
            s.tether = tether;
            if !roam {
                s.roam = 0;
            }
            Game::new(GpuWorld::new(&device, &queue, s, scale), Rules::default(), seed)
        };
        let to = |g: &mut Game<GpuWorld>, secs: f32| {
            while g.time() < secs {
                g.step();
            }
        };
        // A raid with no breakaway: the same game either way.
        let (mut held, mut free) = (play(2, false), play(0, false));
        for g in [&mut held, &mut free] {
            to(g, 1.0);
            let home = g.world.crystals()[0].0;
            let (w, h) = g.world.size();
            let at = (if home.0 < w as f32 / 2.0 { home.0 + 40.0 * k } else { home.0 - 40.0 * k }, home.1);
            let (agar, gw, gh) = g.world.raid_ground().unwrap();
            let field = crate::mould::big::raid_field_for(&agar, gw, gh, crate::mould::larder_world::raid_cell(at, (w, h)));
            let fine = g.world.fine_steps().max(1) as u64;
            g.world.set_raid((g.world.ticks() * fine + 1) as u32, at, 60.0, 0.33, field);
            to(g, 4.0);
        }
        if !ON_CARD.load(std::sync::atomic::Ordering::Relaxed) {
            assert_eq!(held.world.hash(), free.world.hash(), "a raid is not tethered");
        }
        // With breakaways it acts (what it does to the picture is filmed:
        // Reports/raid-march-2026-11-18).
        let (mut held, mut free) = (play(2, true), play(0, true));
        to(&mut held, 20.0);
        to(&mut free, 20.0);
        assert_ne!(held.world.hash(), free.world.hash(), "the tether holds breakaways");
    }

    /// **The set walkers come with the step** (performance thread,
    /// 2026-11-22, `GpuWorld::setc`): with setting on, each game second's
    /// `set_cells` is answered from the step's own copy, never a read-back
    /// that waits for the whole GPU queue, and (`set_cells` checks it in
    /// the tests) the copy is the read-back's cells.
    #[test]
    fn the_set_cells_come_with_the_step() {
        let Some((device, queue)) = test_gpu() else { return };
        let (seed, scale, walkers) = (5, 1, 2_000);
        let mut s = race_settings(seed, scale, walkers);
        crate::gpu::set(&mut s, "alarm_keep", "0.9").unwrap();
        crate::gpu::set(&mut s, "setting", "1").unwrap();
        crate::gpu::set(&mut s, "crust_rise", "0.02").unwrap();
        crate::gpu::set(&mut s, "crust_trail", "0").unwrap();
        let mut g = Game::new(GpuWorld::new(&device, &queue, s, scale), Rules { set_block: 5.0, ..Rules::default() }, seed);
        // Until walkers have set for a few seconds, so the copies checked
        // hold some.
        let mut set_secs = 0;
        while set_secs < 5 && g.time() < 90.0 {
            let sec = g.time() as u32;
            g.step();
            if g.time() as u32 != sec && g.world.e.set_cells_of().iter().any(|&n| n > 0) {
                set_secs += 1;
            }
        }
        let (hits, t) = (g.world.setc.hits.get(), g.time());
        assert!(set_secs >= 5, "walkers set by {t:.0} s");
        assert!(hits as f32 >= t - 2.0, "the set cells came with the step {hits} times in {t:.0} game seconds");
    }

    /// **The fort** (`Rules::fort`, 2026-11-18, the Enemy lane; the
    /// owner's note that day that he had thought the walls "would form
    /// protecting the core of the slime or the slime crystals") is a rule
    /// of the GPU engine alone, so its test is what it does on the
    /// software GPU. Two race worlds alike with setting on and `set_block`
    /// 5, one with `fort` 2 and one without; nobody shoots, so no scar.
    ///
    /// - With the fort the slime holds its home crystal, the engine has
    ///   its ring, and by 20 s walkers have set, every one of them on the
    ///   ring, enough that the bulwark has cells. Without it none sets.
    /// - The walls cost the slime little: it is no smaller than nine
    ///   tenths of the slime with no fort.
    /// - With the rule off the mask is cleared and the walls wake.
    #[test]
    fn the_slime_walls_the_crystal_it_holds() {
        use crate::gpu::{floats, DENSE_HEAD};
        use crate::mould::big::RAID_BLOCK;
        let Some((device, queue)) = test_gpu() else { return };
        let (seed, scale, walkers) = (5, 1, 2_000);
        let play = |fort: f32| {
            // Setting needs the scar, which needs the alarm (the game's
            // shape has both; a race world has neither).
            let mut s = race_settings(seed, scale, walkers);
            crate::gpu::set(&mut s, "alarm_keep", "0.9").unwrap();
            crate::gpu::set(&mut s, "setting", "1").unwrap();
            // A thin race slime moves on before the crust it sets on has
            // grown (the game's blanket stands on its cells): a fast crust
            // under any trail.
            crate::gpu::set(&mut s, "crust_rise", "0.02").unwrap();
            crate::gpu::set(&mut s, "crust_trail", "0").unwrap();
            Game::new(GpuWorld::new(&device, &queue, s, scale), Rules { set_block: 5.0, fort, ..Rules::default() }, seed)
        };
        let to = |g: &mut Game<GpuWorld>, secs: f32| {
            while g.time() < secs {
                g.step();
            }
        };
        // The fort's blocks as the engine holds them (the cells' region's
        // fourth quarter, engine.wgsl's `fort_at`).
        let mask = |g: &Game<GpuWorld>| {
            let e = &g.world.e;
            let nb = e.dish.w.div_ceil(RAID_BLOCK) * e.dish.h.div_ceil(RAID_BLOCK);
            let from = e.dense_at as usize + 4 * (DENSE_HEAD + e.n as usize + 3 * nb);
            read_back(&e.device, &e.queue, &e.news)[from..from + 4 * nb].chunks_exact(4).map(|b| u32::from_le_bytes(b.try_into().unwrap())).collect::<Vec<u32>>()
        };
        // The living set walkers, and how many of them stand off the mask.
        let set = |g: &Game<GpuWorld>, m: &[u32]| {
            let e = &g.world.e;
            let wk = floats(&read_back(&e.device, &e.queue, &e.walkers));
            let en = floats(&read_back(&e.device, &e.queue, &e.energy));
            let bw = e.dish.w.div_ceil(RAID_BLOCK);
            let words = e.set_of();
            let on: Vec<usize> = (0..e.n as usize).filter(|&q| words[q] > 0 && en[q] >= 0.0).collect();
            let off = on.iter().filter(|&&q| m[(wk[4 * q + 1] as usize / RAID_BLOCK) * bw + wk[4 * q] as usize / RAID_BLOCK] == 0).count();
            (on.len(), off)
        };
        let (mut fort, mut bare) = (play(2.0), play(0.0));
        for g in [&mut fort, &mut bare] {
            to(g, 20.0);
        }
        assert!(fort.world.forts(), "setting runs, so the world can hold a fort");
        let m = mask(&fort);
        let ring = m.iter().filter(|&&v| v != 0).count();
        let (walls, stray) = set(&fort, &m);
        let wall_cells = fort.bulwark.iter().filter(|&&b| b).count();
        eprintln!("fort: held {:?}, {ring} blocks in the ring, {walls} walkers set ({stray} off the ring), bulwark {wall_cells} cells; {} alive against {} with no fort", fort.fort_held, fort.world.e.alive(), bare.world.e.alive());
        assert!(fort.fort_held.first() == Some(&true) && fort.world.e.forted && ring > 20, "the slime holds its home crystal and the engine has its ring");
        assert!(walls >= 40 && stray == 0, "slime on the ring sets, and only there: {walls} set, {stray} off the ring");
        assert!(wall_cells >= 20, "the ring is a wall to feet: {wall_cells} cells");
        assert!(!bare.world.e.forted && set(&bare, &mask(&bare)).0 == 0 && bare.bulwark.is_empty(), "no fort, nothing sets unshot");
        assert!(10 * fort.world.e.alive() >= 9 * bare.world.e.alive(), "the walls do not starve the slime");
        // The rule off: the mask is cleared and the walls wake.
        fort.rules.fort = 0.0;
        to(&mut fort, 24.0);
        let m = mask(&fort);
        assert!(!fort.world.e.forted && m.iter().all(|&v| v == 0) && fort.fort_held.is_empty(), "the mask is cleared with the rule off");
        assert_eq!(set(&fort, &m).0, 0, "and the walls wake");
    }

    /// **The slime eats without emptying a crystal, and its size follows
    /// the crystal it covers** (`crystal_keep`, `Rules::ceiling`; the
    /// owner, 2026-11-21 06:30Z and 06:34Z). Engine rules with no CPU twin
    /// (his card, 11-17 19:21Z), so the test is what they do on the
    /// software GPU, in three race worlds alike for 12 game seconds:
    ///
    /// - With the keep the crystals hold every bit of their charge while
    ///   the slime eats, and the engine counts what it ate (the surge
    ///   meter's food) within a fifth of what the same world without the
    ///   keep took out of its crystals.
    /// - The engine counts covered crystal cells, a few walkers to one.
    /// - A ceiling from the crystals it holds (at the landing's walkers a
    ///   unit of held charge) with its roof at the landing walkers:
    ///   births stop there, past it by at most a step's births, while the
    ///   kept world without it grows on.
    /// - The crystal it covers most emptied (as a squad that drained it):
    ///   the charge it holds falls by that crystal's, and the ceiling with
    ///   it (under the roof).
    #[test]
    fn the_slime_eats_without_emptying_a_crystal_and_stops_at_its_ceiling() {
        let Some((device, queue)) = test_gpu() else { return };
        let (seed, scale, walkers) = (3, 1, 2_000);
        let play = |keep: bool, ceiling: f32| {
            let mut s = race_settings(seed, scale, walkers);
            crate::gpu::set(&mut s, "crystal_keep", if keep { "1" } else { "0" }).unwrap();
            Game::new(GpuWorld::new(&device, &queue, s, scale), Rules { ceiling, ceiling_roof: 1.0, ceiling_upkeep: 1.0, ..Rules::default() }, seed)
        };
        let charge = |g: &Game<GpuWorld>| g.world.crystals().iter().map(|c| c.1 as f64).sum::<f64>();
        let (mut kept, mut eaten, mut capped) = (play(true, 0.0), play(false, 0.0), play(true, 1.0));
        // The crystals as the first frame's readout gives them.
        for g in [&mut kept, &mut eaten, &mut capped] {
            g.step();
        }
        let full = charge(&kept);
        assert!(full > 0.0 && (charge(&eaten) - full).abs() < 1e-3 * full, "the worlds start alike");
        while kept.time() < 12.0 {
            for g in [&mut kept, &mut eaten, &mut capped] {
                g.step();
            }
        }
        let took = full - charge(&eaten);
        let ate = kept.world.slime_ate();
        let covered = kept.world.crystal_covered();
        let start = capped.ceiling.start;
        eprintln!("keep: charge {:.1} of {full:.1}, ate {ate:.1}; no keep: took {took:.1}; covered {covered} cells, {} alive; capped at {} (landed {start}), {} alive against {} uncapped", charge(&kept), kept.world.e.alive(), capped.ceiling.now, capped.world.e.alive(), kept.world.e.alive());
        assert!((charge(&kept) - full).abs() < 1e-4 * full, "the keep holds every crystal's charge");
        assert!(took > 100.0 && (ate - took).abs() < 0.2 * took, "the keep counts what the slime ate ({ate:.1}) as the crystals lose it without ({took:.1})");
        assert!(covered > 20 && covered < kept.world.e.alive(), "a few walkers to a covered cell: {covered}");
        let ceil = capped.ceiling.now as usize;
        assert!(ceil <= start && ceil + start / 10 >= start, "the landing's density over more covered cells, under a roof at the landing walkers: ceiling {ceil}, landed {start}");
        let alive = capped.world.e.alive();
        assert!(alive <= ceil + ceil / 100, "births stop at the ceiling: {alive} alive, ceiling {ceil}");
        assert!(kept.world.e.alive() > alive + start / 50, "the world without a ceiling grows on: {} against {alive}", kept.world.e.alive());
        let cov = capped.world.crystal_covered_each();
        let most = (0..cov.len()).max_by_key(|&j| cov[j]).unwrap();
        let (held, share) = (capped.ceiling.held, capped.world.crystals()[most].1 as f64);
        // What it could take meanwhile: the crystals it did not hold.
        let free: f64 = capped.world.crystals().iter().zip(&capped.ceiling.holding).filter(|(_, &h)| !h).map(|(c, _)| c.1 as f64).sum();
        capped.world.empty_crystal(most);
        while capped.time() < 15.0 {
            capped.step();
        }
        let c = &capped.ceiling;
        eprintln!("drained crystal {most} ({share:.0} of {held:.0} held): holds {:.0}, ceiling {ceil} to {}", c.held, c.now);
        assert!(share > 0.0 && capped.ceiling.holding[most] && capped.world.crystals()[most].1 == 0.0, "it held the drained crystal, now empty");
        assert!(c.held <= held - share + free + 1.0, "the charge it holds lost the drained crystal's: {held:.0} to {:.0} ({free:.0} it could take)", c.held);
        let want = (c.held * c.density).min(start as f64).round();
        // (No lower: a falling ceiling chases the slime down, `CEILING_FALL`.)
        assert!(c.now as f64 >= want - 1.0 && c.now as usize <= ceil, "the ceiling follows it, under the roof: {} against {want}", c.now);
    }

    /// **The origin** (`origin.rs`, 2026-11-22, Milestone 2's win; the
    /// owner's idea of 11-16, Direction's go 11-22). Engine-side it is the
    /// lumps the mini-bosses use, so the test is what it does on the
    /// software GPU, in two race worlds alike: it forms at the landing on
    /// the start crystal; with that crystal drained at once (as a squad
    /// that drained it) it does not regrow, and in the fed world it holds
    /// more; cut under its floor it breaks, births stop, and after the
    /// die-back the planet is won while slime still lived when it broke.
    #[test]
    fn the_origin_forms_on_its_crystal_starves_when_drained_and_its_death_wins() {
        let Some((device, queue)) = test_gpu() else { return };
        let (seed, scale, walkers) = (3, 1, 4_000);
        let play = || {
            let s = race_settings(seed, scale, walkers);
            let mut rules = Rules::default();
            rules.origin.on = true;
            rules.origin.size = 150.0;
            rules.origin.die_upkeep = std::env::var("DU").ok().and_then(|v| v.parse().ok()).unwrap_or(rules.origin.die_upkeep);
            Game::new(GpuWorld::new(&device, &queue, s, scale), rules, seed)
        };
        let (mut fed, mut dry) = (play(), play());
        let home = fed.core;
        let c = fed.crystal_at(home).map(|(c, _)| c).expect("the slime's start crystal");
        dry.world.empty_crystal(c);
        while fed.time() < 14.0 {
            for g in [&mut fed, &mut dry] {
                g.step();
            }
        }
        let members = |g: &Game<GpuWorld>| g.origin_hud().map_or(0, |o| o.0);
        let (a, b) = (members(&fed), members(&dry));
        eprintln!("origin: fed {a} members (reach {:.1}), drained {b} (reach {:.1})", fed.origin.reach, dry.origin.reach);
        assert!(fed.origin.formed && fed.stand.live.is_some_and(|h| h.origin), "it formed at the landing");
        assert!(a >= 50, "it gathered round its fed crystal: {a}");
        assert!(a > b + 30, "a drained crystal does not feed it: {a} against {b}");
        // Its floor over its members: it breaks, and the die-back runs.
        // It must have held a body first (`2 x floor_min`); its takes are
        // cut to its shortfall (11-22), so it may get there after 14 s.
        while fed.stand.live.is_some_and(|h| (h.most as f32) < 2.0 * fed.rules.origin.floor_min) && fed.time() < 40.0 {
            fed.step();
        }
        fed.rules.origin.floor = 2.0;
        let mass = fed.mould;
        let broke_at = fed.time();
        while fed.end.is_none() && fed.time() < broke_at + 20.0 {
            fed.step();
        }
        eprintln!("broke at {broke_at:.1} s with {mass} walkers; ended {:?} with {} left", fed.end, fed.mould);
        assert!(fed.origin.fell.is_some() && fed.stand.live.is_none(), "it broke under its floor");
        assert!(matches!(fed.end, Some(End::MouldWiped(_))), "the planet is won: {:?}", fed.end);
        assert!(fed.time() <= broke_at + fed.rules.origin.die_back + 2.0, "within the die-back: {:.1} s", fed.time() - broke_at);
    }

    /// **The last stand gathers only up to twice the origin's size**
    /// (`OriginRules::gather_most`, 11-24): Engine's siege met planet 7's
    /// origin at 79,940 members against a size of 4,200, the late last
    /// stand having taken every free walker on the map at once. Here an
    /// origin of 150 in a race world of 4,000 walkers, set gathering once
    /// it holds its size, holds more than its size and no more than twice
    /// it (and 5%) 20 s later: a disc sized to the slime it held still
    /// took 446 and 570 here, so the engine caps the joiners
    /// (`grow_lump_most`).
    #[test]
    fn the_last_stand_gathers_the_origin_only_to_twice_its_size() {
        let Some((device, queue)) = test_gpu() else { return };
        let (seed, scale, walkers) = (3, 1, 4_000);
        let s = race_settings(seed, scale, walkers);
        let mut rules = Rules::default();
        rules.origin.on = true;
        rules.origin.size = 150.0;
        let mut g = Game::new(GpuWorld::new(&device, &queue, s, scale), rules, seed);
        let members = |g: &Game<GpuWorld>| g.origin_hud().map_or(0, |o| o.0);
        while (members(&g) as f32) < 0.9 * g.rules.origin.size && g.time() < 40.0 {
            g.step();
        }
        let before = members(&g);
        let gathered_at = g.time();
        g.stand.live.as_mut().expect("the origin formed").gather = true;
        let mut most = 0;
        while g.time() < gathered_at + 20.0 {
            g.step();
            most = most.max(members(&g));
        }
        let after = members(&g);
        eprintln!("gather: {before} members at {gathered_at:.1} s, {after} 20 s later (most {most}), slime {}", g.mould);
        assert!(g.mould > 2_000, "the slime is far bigger than the cap: {}", g.mould);
        assert!(after as f32 > 1.3 * g.rules.origin.size, "it gathered more than its size: {after}");
        assert!(most as f32 <= 2.05 * g.rules.origin.size, "but not the body: {most}");
    }

    /// **A hit's hole stays open while it is fresh** (`crater_shut`,
    /// 2026-11-21, the Look lane's A2). Engine rule with no CPU twin, so
    /// the test is what it does on the software GPU: two race worlds
    /// alike, a kill and a crater as long as the guns' on the home crystal's slime, one
    /// with the first half of the crater shut. Walkers within the hole's
    /// inner two thirds: none back in the shut world while the whole hole
    /// is shut (a quarter of the crater), and fewer than in the open world
    /// at its half; both refill once it opens.
    #[test]
    fn a_hits_hole_stays_open_while_it_is_fresh() {
        use crate::gpu::floats;
        use crate::mould::kit::KitRules;
        let Some((device, queue)) = test_gpu() else { return };
        let (seed, scale, walkers) = (5, 1, 4_000);
        let play = |shut: &str| {
            let mut s = race_settings(seed, scale, walkers);
            crate::gpu::set(&mut s, "crater_shut", shut).unwrap();
            let mut g = Game::new(GpuWorld::new(&device, &queue, s, scale), Rules::default(), seed);
            while g.time() < 20.0 {
                g.step();
            }
            g
        };
        let (mut open, mut shut) = (play("0"), play("0.5"));
        let home = open.world.crystals()[0].0;
        let sc = open.world.scale();
        let at = (home.0 / sc, home.1 / sc);
        let fine = open.world.fine_steps() as f32;
        let k = crate::mould::rounds::burst_rules(&KitRules::default(), 30.0 * fine);
        // The guns' own crater (fine steps, from the game's frame), as a
        // shot's crater is shut and a longer one is not.
        let cs = open.world.e.crater_steps;
        assert!(cs >= 8, "the game lays craters: {cs}");
        let hit = squad::Burst { at, r: 6.0, kill: true, crater: cs as _, crater_r: 6.0, ..Default::default() };
        // The walkers within 4 of the hit's centre (fine cells).
        let inside = |g: &Game<GpuWorld>| {
            let e = &g.world.e;
            let wk = floats(&read_back(&e.device, &e.queue, &e.walkers));
            let en = floats(&read_back(&e.device, &e.queue, &e.energy));
            let (cx, cy, r) = (at.0 * e.dish.p.scale as f32, at.1 * e.dish.p.scale as f32, 4.0 * e.dish.p.scale as f32);
            (0..e.n as usize).filter(|&q| en[q] >= 0.0 && (wk[4 * q] - cx).hypot(wk[4 * q + 1] - cy) <= r).count()
        };
        let before = inside(&open);
        let mut counts = Vec::new();
        for g in [&mut open, &mut shut] {
            g.world.burst(&hit, &k);
            let mut c = Vec::new();
            for quarter in 1..=4 {
                let until = g.world.ticks() + (cs as f32 / 4.0 / fine).ceil() as u64;
                while g.world.ticks() < until {
                    g.step();
                }
                c.push((quarter, inside(g)));
            }
            counts.push(c);
        }
        eprintln!("hole: {before} walkers before the hit; open {:?}, shut {:?} (quarters of the crater)", counts[0], counts[1]);
        assert!(before > 30, "the hit lands in the slime: {before}");
        assert_eq!(counts[1][0].1, 0, "nothing steps into the shut hole");
        assert!(counts[1][1].1 < counts[0][1].1, "fewer back at the crater's half: {:?} against {:?}", counts[1][1], counts[0][1]);
        assert!(counts[1][3].1 > 0, "the shut hole refills once it opens");
    }

    /// **The Sprawler's threads stack its walkers into lanes** (the Lab's
    /// prototype, 11-24). With `thread_lanes` a calm walker may step into
    /// a full cell whose trail beats its own, and with `thread_shuttle` a
    /// fed walker reads the scent backwards, so the body packs into
    /// threads with holes between: more of its walkers share a cell than
    /// without, on the same seed.
    #[test]
    fn the_sprawlers_threads_stack_walkers_into_lanes() {
        let Some((device, queue)) = test_gpu() else { return };
        let (seed, scale, walkers) = (5, 1, 5_000);
        let play = |keys: &[(&str, &str)]| {
            let mut s = race_settings(seed, scale, walkers);
            for &(k, v) in [("keep", "0.92"), ("sensor_offset", "6")].iter().chain(keys) {
                crate::gpu::set(&mut s, k, v).unwrap();
            }
            let mut g = Game::new(GpuWorld::new(&device, &queue, s, scale), Rules::default(), seed);
            while g.time() < 30.0 {
                g.step();
            }
            let e = &g.world.e;
            let occ: Vec<u32> = read_back(&e.device, &e.queue, &e.occ).chunks_exact(4).map(|b| u32::from_le_bytes([b[0], b[1], b[2], b[3]])).collect();
            let all: u32 = occ.iter().sum();
            let stacked: u32 = occ.iter().filter(|&&c| c >= 2).sum();
            // The crystal cells under the slime, which set its ceiling
            // (`ceiling.rs`): the Lab feared fed shuttles leave a crystal's
            // middle bare.
            (all, stacked as f32 / all.max(1) as f32, g.world.crystal_covered())
        };
        let off = play(&[]);
        let on = play(&[("thread_lanes", "3"), ("thread_trail", "1.0"), ("thread_shuttle", "3"), ("thread_fed", "0.9")]);
        eprintln!("threads: off {} walkers, {:.3} stacked, {} crystal cells covered; on {} walkers, {:.3} stacked, {} covered", off.0, off.1, off.2, on.0, on.1, on.2);
        assert!(off.0 > 1_000 && on.0 > 1_000, "both slimes live: {off:?} {on:?}");
        assert!(on.1 > off.1 + 0.05, "threads stack more walkers a cell: {on:?} against {off:?}");
    }

    /// **Scar tissue shrugs off blows** (`scar_tough`, the Grudge's,
    /// 2026-11-24; GPU only). Two race worlds alike with the scar on, one
    /// with scar tissue at 0.5; the scar set to twice `scar_tough_full`
    /// everywhere, then one killing burst on the home crystal. With scar
    /// tissue about half the walkers it hit live; without, none.
    #[test]
    fn scar_tissue_shrugs_off_half_the_blows() {
        use crate::gpu::{f32_bytes, floats};
        use crate::mould::kit::KitRules;
        let Some((device, queue)) = test_gpu() else { return };
        let (seed, scale, walkers) = (5, 1, 4_000);
        let play = |tough: &str| -> (usize, usize, usize) {
            let mut s = race_settings(seed, scale, walkers);
            for (k, v) in [("alarm_keep", "0.95"), ("scar_keep", "0.9999"), ("scar_gain", "0.01"), ("scar_tough", tough), ("scar_tough_full", "2000")] {
                crate::gpu::set(&mut s, k, v).unwrap();
            }
            let mut g = Game::new(GpuWorld::new(&device, &queue, s, scale), Rules::default(), seed);
            while g.time() < 10.0 {
                g.step();
            }
            let home = g.world.crystals()[0].0;
            let sc = g.world.scale();
            let at = (home.0 / sc, home.1 / sc);
            let (cx, cy, r) = {
                let e = &g.world.e;
                let ds = e.dish.p.scale as f32;
                (at.0 * ds, at.1 * ds, 5.0 * ds)
            };
            let near = |g: &Game<GpuWorld>| {
                let e = &g.world.e;
                let wk = floats(&read_back(&e.device, &e.queue, &e.walkers));
                let en = floats(&read_back(&e.device, &e.queue, &e.energy));
                (0..e.n as usize).filter(|&q| en[q] >= 0.0 && (wk[4 * q] - cx).hypot(wk[4 * q + 1] - cy) <= r).count()
            };
            {
                let e = &g.world.e;
                e.queue.write_buffer(&e.crater, e.cells as u64 * 4, &f32_bytes(&vec![4_000.0; e.cells as usize]));
            }
            let before = near(&g);
            let fine = g.world.fine_steps() as f32;
            let k = crate::mould::rounds::burst_rules(&KitRules::default(), 30.0 * fine);
            g.world.burst(&squad::Burst { at, r: 6.0, kill: true, ..Default::default() }, &k);
            let after = near(&g);
            // The interface's count of walkers on full scar (every cell's
            // scar is over full here): all of the living with scar tissue
            // on, none with it off.
            g.step();
            (before, after, g.world.scarred())
        };
        let off = play("0");
        let on = play("0.5");
        eprintln!("scar tissue: off {} -> {}, on {} -> {} ({} on full scar)", off.0, off.1, on.0, on.1, on.2);
        assert!(off.2 == 0 && on.2 > walkers / 2, "the walkers on full scar counted: {off:?} {on:?}");
        assert!(off.0 > 50 && on.0 > 50, "slime on the crystal: {off:?} {on:?}");
        assert!(off.1 * 10 < off.0, "without scar tissue the burst kills nearly all: {off:?}");
        let kept = on.1 as f32 / on.0 as f32;
        assert!((0.3..0.7).contains(&kept), "scar tissue keeps about half: {on:?}");
    }

    /// **The Sleeper grows while it sleeps** (`dream`, 2026-11-24; GPU
    /// only). Two race worlds alike with the Sleeper's life cycle and a
    /// birth ceiling of twice the walkers, one dreaming at 0.003 a step:
    /// at 40 s the dreaming one has more walkers asleep, and never more
    /// than its ceiling (a step's births over it at most).
    #[test]
    fn the_sleeper_grows_while_it_sleeps() {
        use crate::mould::tiers::Tier;
        let Some((device, queue)) = test_gpu() else { return };
        let (seed, scale, walkers) = (5, 1, 4_000);
        let play = |dream: &str| -> (usize, usize) {
            let mut s = race_settings(seed, scale, walkers);
            for (k, v) in Tier::Sleeper.keys() {
                crate::gpu::set(&mut s, k, v).unwrap();
            }
            crate::gpu::set(&mut s, "dream", dream).unwrap();
            s.ceiling = 2 * walkers as u32;
            let mut g = Game::new(GpuWorld::new(&device, &queue, s, scale), Rules::default(), seed);
            while g.time() < 40.0 {
                g.step();
            }
            let (st, _) = g.world.e.life_of();
            let en = crate::gpu::floats(&read_back(&g.world.e.device, &g.world.e.queue, &g.world.e.energy));
            let alive: Vec<u8> = st.iter().zip(&en).filter(|(_, &e)| e >= 0.0).map(|(&s, _)| s).collect();
            let asleep = alive.iter().filter(|&&s| s == 2).count();
            // The interface's per-frame count of them agrees.
            assert_eq!(g.world.asleep(), asleep, "the asleep counter");
            (alive.len(), asleep)
        };
        let off = play("0");
        let on = play("0.003");
        eprintln!("dreaming: off {} alive, {} asleep; on {} alive, {} asleep", off.0, off.1, on.0, on.1);
        assert!(off.1 > 100, "the Sleeper sleeps: {off:?}");
        assert!(on.1 > off.1 + off.1 / 5, "dreaming grows the sleeping body: {on:?} against {off:?}");
        assert!(on.0 <= 2 * walkers + walkers / 4, "under its ceiling: {on:?}");
    }

    /// **The last stand wakes the Dozer** (`World::rouse`, 2026-11-24; GPU
    /// only). A race world with the Dozer's keys sleeps away from its
    /// crystals by 40 s; `rouse` wakes every sleeper at once, so its last
    /// stand sends all of it.
    #[test]
    fn the_last_stand_wakes_the_dozer() {
        use crate::mould::tiers::Tier;
        let Some((device, queue)) = test_gpu() else { return };
        let (seed, scale, walkers) = (5, 1, 4_000);
        let mut s = race_settings(seed, scale, walkers);
        for (k, v) in Tier::Dozer.keys() {
            crate::gpu::set(&mut s, k, v).unwrap();
        }
        let mut g = Game::new(GpuWorld::new(&device, &queue, s, scale), Rules::default(), seed);
        while g.time() < 40.0 {
            g.step();
        }
        let asleep = |g: &Game<GpuWorld>| {
            let (st, _) = g.world.e.life_of();
            let en = crate::gpu::floats(&read_back(&g.world.e.device, &g.world.e.queue, &g.world.e.energy));
            st.iter().zip(&en).filter(|(&s, &e)| e >= 0.0 && s == 2).count()
        };
        let before = asleep(&g);
        g.world.rouse();
        let after = asleep(&g);
        eprintln!("the Dozer: {before} asleep, {after} after the rouse");
        assert!(before > 100, "the Dozer sleeps: {before}");
        assert_eq!(after, 0, "the rouse wakes them all");
    }

    /// **A blast's long crater is not shut** (the review lane, 11-21: det
    /// cord stood as a wall for 19 s). Only a crater no longer than the
    /// guns' (`crater`) is shut: a kill with a crater ten times as long on
    /// a world with the first half of the guns' crater shut refills in its
    /// first quarter-second as an open world's hole does.
    #[test]
    fn a_blasts_long_crater_is_not_shut() {
        use crate::gpu::floats;
        use crate::mould::kit::KitRules;
        let Some((device, queue)) = test_gpu() else { return };
        let (seed, scale, walkers) = (5, 1, 4_000);
        let mut s = race_settings(seed, scale, walkers);
        crate::gpu::set(&mut s, "crater_shut", "0.5").unwrap();
        let mut g = Game::new(GpuWorld::new(&device, &queue, s, scale), Rules::default(), seed);
        while g.time() < 20.0 {
            g.step();
        }
        let home = g.world.crystals()[0].0;
        let sc = g.world.scale();
        let at = (home.0 / sc, home.1 / sc);
        let fine = g.world.fine_steps() as f32;
        let k = crate::mould::rounds::burst_rules(&KitRules::default(), 30.0 * fine);
        let cs = g.world.e.crater_steps;
        g.world.burst(&squad::Burst { at, r: 6.0, kill: true, crater: (10 * cs) as _, crater_r: 6.0, ..Default::default() }, &k);
        for _ in 0..2 {
            let until = g.world.ticks() + (cs as f32 / 4.0 / fine).ceil() as u64;
            while g.world.ticks() < until {
                g.step();
            }
        }
        let e = &g.world.e;
        let wk = floats(&read_back(&e.device, &e.queue, &e.walkers));
        let en = floats(&read_back(&e.device, &e.queue, &e.energy));
        let (cx, cy, r) = (at.0 * e.dish.p.scale as f32, at.1 * e.dish.p.scale as f32, 4.0 * e.dish.p.scale as f32);
        let back = (0..e.n as usize).filter(|&q| en[q] >= 0.0 && (wk[4 * q] - cx).hypot(wk[4 * q + 1] - cy) <= r).count();
        eprintln!("blast hole: {back} walkers back at half the guns' crater");
        assert!(back > 0, "a blast's crater stays open to walkers");
    }

    /// **A mini-boss's walkers flinch out of a hit** (`boss_flinch`,
    /// 2026-11-21, Direction's job from the Look lane's exit test 2; GPU
    /// only). Two race worlds alike, a lump of armoured members held in
    /// place by its pull, and a hit at its centre that hurts them and
    /// leaves a crater as long as the guns'; one world with members flinching for the
    /// first half of it, both with its first half shut (`crater_shut`, as
    /// in a run). A quarter of the crater on, the flinching lump
    /// holds fewer members within 3 cells of the hit (its hole) than the
    /// one that stood (llvmpipe 11-21: 51 against 103 of 108), and as many
    /// members in all (a flinch kills none).
    #[test]
    fn a_mini_bosss_walkers_flinch_out_of_a_hit() {
        use crate::gpu::floats;
        use crate::mould::big::{Lump, LUMP_BITS, LUMP_SHIFT};
        use crate::mould::kit::KitRules;
        use crate::mould::larder_world::{raid_cell, BossRules, World};
        let Some((device, queue)) = test_gpu() else { return };
        let (seed, scale, walkers) = (5, 1, 4_000);
        let play = |flinch: &str| -> (usize, usize, usize) {
            let mut s = race_settings(seed, scale, walkers);
            crate::gpu::set(&mut s, "boss_flinch", flinch).unwrap();
            crate::gpu::set(&mut s, "crater_shut", "0.5").unwrap();
            let rules = Rules { boss: BossRules { every: 0.0, first: 0.0, ..BossRules::default() }, ..Rules::default() };
            let mut g = Game::new(GpuWorld::new(&device, &queue, s, scale), rules, seed);
            while g.time() < 20.0 {
                g.step();
            }
            let home = g.world.crystals()[0].0;
            let sc = g.world.scale();
            let at = (home.0 / sc, home.1 / sc);
            let (w, h) = g.world.size();
            let b = BossRules::default();
            g.world.form_lump(0, Some((at, 8.0)));
            g.world.set_lump(0, Lump { aim: raid_cell(at, (w, h)), pull: b.pull, leash: b.leash_at(b.clench, 0.5), armour: 15, ..Lump::default() });
            for _ in 0..30 {
                g.step();
            }
            // Members in all, and within 3 cells of the hit.
            let members = |g: &Game<GpuWorld>| {
                let e = &g.world.e;
                let wk = floats(&read_back(&e.device, &e.queue, &e.walkers));
                let en = floats(&read_back(&e.device, &e.queue, &e.energy));
                let news: Vec<u32> = read_back(&e.device, &e.queue, &e.news).chunks_exact(4).map(|c| u32::from_le_bytes([c[0], c[1], c[2], c[3]])).collect();
                let base = e.dense_at as usize / 4 + crate::gpu::DENSE_HEAD;
                let (cx, cy, r) = (at.0 * e.dish.p.scale as f32, at.1 * e.dish.p.scale as f32, 3.0 * e.dish.p.scale as f32);
                let m: Vec<usize> = (0..e.n as usize).filter(|&q| en[q] >= 0.0 && (news[base + q] & LUMP_BITS) >> LUMP_SHIFT == 1).collect();
                let near = m.iter().filter(|&&q| (wk[4 * q] - cx).hypot(wk[4 * q + 1] - cy) <= r).count();
                (m.len(), near)
            };
            let (_, before) = members(&g);
            let fine = g.world.fine_steps() as f32;
            let k = crate::mould::rounds::burst_rules(&KitRules::default(), 30.0 * fine);
            let cs = g.world.e.crater_steps;
            g.world.burst(&squad::Burst { at, r: 5.0, kill: true, crater: cs as _, crater_r: 5.0, ..Default::default() }, &k);
            let until = g.world.ticks() + (cs as f32 / 4.0 / fine).ceil() as u64;
            while g.world.ticks() < until {
                g.step();
            }
            let (all, near) = members(&g);
            (before, all, near)
        };
        let (stood, flinched) = (play("0"), play("0.5"));
        eprintln!("flinch: (near before, members, near after) stood {stood:?}, flinched {flinched:?}");
        assert!(stood.0 > 10, "the hit lands in the lump: {stood:?}");
        assert!(flinched.2 * 3 < stood.2 * 2, "the flinching lump opens a hole: {flinched:?} against {stood:?}");
        assert!(flinched.1 + flinched.1 / 20 >= stood.1, "a flinch kills none: {flinched:?} against {stood:?}");
    }

    /// **A shot stops at set slime** (the owner's planet 3 round,
    /// 2026-11-19 20:46Z: "slime that i couldn't kill they were not moving
    /// but all my soliders would shoot them over and over and nothing hurt
    /// them"). A set walker lays no trail, and a shot stopped only where
    /// the trail looked like slime, so once the live slime round a wall
    /// had gone its trail faded and every bullet flew through it: that
    /// round ended on 41 set walkers that 3,200 shots in 85 s never hit.
    /// The fort's world of the test above at 20 s, its trail then wiped
    /// as if the live slime had long gone: a shot at a set walker's cell
    /// hits it, and it dies within its armour's blows.
    #[test]
    fn a_shot_stops_at_set_slime_with_no_trail() {
        use crate::gpu::floats;
        use crate::mould::squad::{slime_trail, ShotIn};
        let Some((device, queue)) = test_gpu() else { return };
        let (seed, scale, walkers) = (5, 1, 2_000);
        let mut s = race_settings(seed, scale, walkers);
        crate::gpu::set(&mut s, "alarm_keep", "0.9").unwrap();
        crate::gpu::set(&mut s, "setting", "1").unwrap();
        crate::gpu::set(&mut s, "crust_rise", "0.02").unwrap();
        crate::gpu::set(&mut s, "crust_trail", "0").unwrap();
        let armour = s.set_armour;
        let mut g = Game::new(GpuWorld::new(&device, &queue, s, scale), Rules { set_block: 5.0, fort: 2.0, ..Rules::default() }, seed);
        while g.time() < 20.0 {
            g.step();
        }
        let slime = slime_trail(6.0);
        let e = &g.world.e;
        let w = e.dish.w;
        // The trail faded: both sides wiped.
        let none = crate::gpu::f32_bytes(&vec![0.0; w * e.dish.h]);
        for side in &e.trail {
            e.queue.write_buffer(side, 0, &none);
        }
        let wk = floats(&read_back(&e.device, &e.queue, &e.walkers));
        let en = floats(&read_back(&e.device, &e.queue, &e.energy));
        let words = e.set_of();
        let set: Vec<usize> = (0..e.n as usize).filter(|&q| words[q] > 0 && en[q] >= 0.0).map(|q| wk[4 * q + 1] as usize * w + wk[4 * q] as usize).collect();
        assert!(!set.is_empty(), "nothing set (the case is not made)");
        let c = set[0];
        let at = ((c % w) as f32 + 0.5, (c / w) as f32 + 0.5);
        let shot = Frame { shots: vec![ShotIn { from: at, to: at, r: 0.0, over: false }], slime, ..Frame::default() };
        let mut killed = 0;
        for n in 0..=armour {
            let out = g.world.frame(&shot, 0);
            assert!(out.shots[0].hit.is_some(), "shot {n} flew through the set walker at {at:?}");
            killed += out.shots[0].killed;
        }
        assert!(killed >= 1, "{} shots at a set walker killed none", armour + 1);
    }

    /// **Plants and their fire run on the GPU as on the CPU** (2026-11-14,
    /// `mould::plants`): a map with plants of all four kinds, sparse so
    /// they spread, run side by side; a burn lit at 3 s. The plants,
    /// kinds, fire and char match at every check (exact on llvmpipe, a
    /// little slack on a card), and fire kills count in `burned` on both.
    /// **A hurt slime fruits** (2026-11-20, `larder_world/fruit.rs`; the
    /// owner, 11-20 05:46Z, on the slime late in a planet: "lots of
    /// shooting through masses of uniform orange"). With fruiting on and
    /// the hurt test at 0 (so it may fruit at once) on a race world:
    /// - a knot of lump members forms out of the guns' reach and swells;
    /// - left alone it bursts when ripe: its walkers leave as spores in the
    ///   air, nearly all come down living, and they land farther from the
    ///   knot than they stood;
    /// - struck down under its quorum it breaks instead, and sows nothing.
    #[test]
    fn a_fruit_ripens_and_sows_or_breaks() {
        use crate::gpu::floats;
        use crate::mould::big::LIFE_SPORE;
        use crate::mould::larder_world::{FruitEnd, FruitRules};
        let Some((device, queue)) = test_gpu() else { return };
        let (seed, scale, walkers) = (5, 1, 2_000);
        let fruit = FruitRules { on: true, first: 2.0, hurt: 0.0, least: 50.0, size: 0.15, ripen: 6.0, clear: 0.0, ..FruitRules::default() };
        let play = || {
            let mut s = race_settings(seed, scale, walkers);
            // The run's keys for the spores' homing and landing.
            crate::gpu::set(&mut s, "spore_reach", "36").unwrap();
            crate::gpu::set(&mut s, "spore_land", "1").unwrap();
            Game::new(GpuWorld::new(&device, &queue, s, scale), Rules { fruit, ..Rules::default() }, seed)
        };
        let to = |g: &mut Game<GpuWorld>, secs: f32| {
            while g.time() < secs {
                g.step();
            }
        };
        // The living walkers' mean distance from `at`.
        let spread = |g: &Game<GpuWorld>, at: (f32, f32)| {
            let e = &g.world.e;
            let wk = floats(&read_back(&e.device, &e.queue, &e.walkers));
            let en = floats(&read_back(&e.device, &e.queue, &e.energy));
            let d: Vec<f32> = (0..e.n as usize).filter(|&q| !e.grows || en[q] >= 0.0).map(|q| (wk[4 * q] - at.0).hypot(wk[4 * q + 1] - at.1)).collect();
            d.iter().sum::<f32>() / d.len().max(1) as f32
        };
        // Ripe, it bursts.
        let mut g = play();
        to(&mut g, 4.5);
        let f = g.fruits.live.expect("the slime may fruit from 2 s and it is hurt enough (0)");
        assert!(f.members >= 50, "a knot of the slime's walkers: {} members", f.members);
        let (at, alive, near) = (f.centre, g.world.e.alive(), spread(&g, f.centre));
        while g.fruits.last.is_none() && g.time() < 15.0 {
            g.step();
        }
        assert_eq!(g.fruits.last.map(|l| l.1), Some(FruitEnd::Burst), "left alone it bursts when ripe");
        let sown = g.fruits.sown;
        let flying = g.world.e.life_of().0.iter().filter(|&&l| l == LIFE_SPORE).count();
        let t = g.time();
        to(&mut g, t + 12.0);
        let (after, far) = (g.world.e.alive(), spread(&g, at));
        eprintln!("fruit: {} members at 4.5 s (most {}), {sown} sown, {flying} in the air just after; alive {alive} -> {after}; mean distance from the knot {near:.1} -> {far:.1}", f.members, g.fruits.last.unwrap().2);
        assert!(sown >= 50 && flying as u64 >= sown / 2, "its walkers leave as spores: {sown} sown, {flying} flying");
        assert!(10 * after >= 8 * alive, "and come down living: {alive} -> {after}");
        assert!(far > near, "farther from the knot than they stood: {near:.1} -> {far:.1}");
        // Struck down, it breaks.
        let mut g = play();
        to(&mut g, 4.5);
        let f = g.fruits.live.expect("a fruit as before");
        let sc = g.world.scale();
        let r = 2.0 * (f.members as f32 / std::f32::consts::PI).sqrt() / sc + 4.0;
        for _ in 0..3 {
            g.world.burst(&squad::Burst { at: (f.centre.0 / sc, f.centre.1 / sc), r, kill: true, ..Default::default() }, &squad::BurstRules::default());
            g.step();
        }
        while g.fruits.last.is_none() && g.time() < 15.0 {
            g.step();
        }
        assert_eq!(g.fruits.last.map(|l| l.1), Some(FruitEnd::Broken), "shot down it breaks");
        assert_eq!(g.fruits.sown, 0, "and sows nothing");
        // A burst lands short of the squad (2026-11-21, `Game::sow_steps`):
        // the throw stops the disc and its flight short of the guns' reach
        // and `clear` from the nearest living soldier.
        let mut g = play();
        g.step();
        assert!(!g.soldiers.is_empty(), "the race world lands a squad");
        let (step, throw) = (g.world.spore_flight(), g.rules.fruit.throw);
        assert!(step > 0.0, "a world that sows says how far a spore flies a step");
        let far = (g.gun_range_max() + g.rules.fruit.clear) * g.world.scale();
        let at = (g.soldiers[0].x, g.soldiers[0].y);
        for s in g.soldiers.iter_mut().skip(1) {
            s.alive = false;
        }
        let mut steps = |d: f32| {
            g.soldiers[0].x = at.0 + d;
            g.sow_steps(at, 4.0)
        };
        assert_eq!(steps(0.5 * far), 2, "a squad in reach of the pod gets it back about where it stood");
        let d = far + 4.0 + 50.5 * step;
        let n = steps(d);
        assert!(n == 50 && 4.0 + n as f32 * step <= d - far, "it comes down short of the squad: {n} steps of {step} with the squad {d} off, reach {far}");
        assert_eq!(steps(1.0e6), throw, "a squad far off sees the whole throw");
        g.soldiers[0].alive = false;
        assert_eq!(g.sow_steps(at, 4.0), throw, "and no squad the whole throw");
    }

    /// **A pinwheel's arms turn** (2026-11-20, `Lump::spin`, the Pinwheel
    /// mini-boss; GPU only). A lump of about 400 walkers on a race world,
    /// held in place by its pull, run 4 s with and without `spin`: with it
    /// the members gather into three arms (the angular harmonic of order
    /// `PIN_ARMS` round their centre, `h3`, is well above the still
    /// lump's), and the arms turn (its phase keeps moving one way). On
    /// llvmpipe (11-20): h3 0.159 spun against 0.064 still, 350 members;
    /// the phase moved 9.7 rad in 3 s (0.2 turns a second x 3 arms is
    /// 11.3). Walking to an aim it keeps most of a still lump's pace (7.5
    /// cells in 4 s against 11.5 at haste 2; at 1, 5.5).
    #[test]
    fn a_spinning_lump_gathers_into_turning_arms() {
        use crate::gpu::floats;
        use crate::mould::big::{Lump, LUMP_BITS, LUMP_SHIFT};
        use crate::mould::larder_world::{raid_cell, BossRules, World};
        let Some((device, queue)) = test_gpu() else { return };
        let (seed, scale, walkers) = (5, 1, 6_000);
        // Per 0.25 s: the members' h3 (size, phase).
        let run = |turns: f32, walk: f32| -> (Vec<(f32, f32)>, f32) {
            let rules = Rules { boss: BossRules { every: 0.0, first: 0.0, ..BossRules::default() }, ..Rules::default() };
            let mut g = Game::new(GpuWorld::new(&device, &queue, race_settings(seed, scale, walkers), scale), rules, seed);
            while g.time() < 2.0 {
                g.step();
            }
            // The most crowded 12-cell square.
            let e = &g.world.e;
            let wk = floats(&read_back(&e.device, &e.queue, &e.walkers));
            let en = floats(&read_back(&e.device, &e.queue, &e.energy));
            let (w, h) = g.world.size();
            let (bw, bh) = (w / 12 + 1, h / 12 + 1);
            let mut bins = vec![0u32; bw * bh];
            for q in 0..e.n as usize {
                let (x, y) = (wk[4 * q], wk[4 * q + 1]);
                if (!e.grows || en[q] >= 0.0) && x >= 0.0 && y >= 0.0 && (x as usize) < w && (y as usize) < h {
                    bins[(y as usize / 12) * bw + x as usize / 12] += 1;
                }
            }
            let top = (0..bins.len()).max_by_key(|&b| bins[b]).unwrap();
            let at = ((top % bw) as f32 * 12.0 + 6.0, (top / bw) as f32 * 12.0 + 6.0);
            let b = BossRules::default();
            g.world.form_lump(0, Some((at, 12.0)));
            let per_step = std::f32::consts::TAU / (15.0 * g.world.fine_steps().max(1) as f32);
            let to = ((at.0 + walk).min(w as f32 - 2.0), at.1);
            g.world.set_lump(0, Lump { aim: raid_cell(to, (w, h)), pull: b.pull, leash: b.leash_at(b.clench, 0.5), spin: turns * per_step, haste: if turns != 0.0 { 2.0 } else { 1.0 }, ..Lump::default() });
            let mut out = Vec::new();
            let start = g.world.lumps()[0].map_or(at, |l| (l.0, l.1));
            let t0 = g.time();
            while g.time() < t0 + 4.0 {
                for _ in 0..4 {
                    g.step();
                }
                let e = &g.world.e;
                let wk = floats(&read_back(&e.device, &e.queue, &e.walkers));
                let news: Vec<u32> = read_back(&e.device, &e.queue, &e.news).chunks_exact(4).map(|c| u32::from_le_bytes([c[0], c[1], c[2], c[3]])).collect();
                let base = e.dense_at as usize / 4 + crate::gpu::DENSE_HEAD;
                let m: Vec<(f32, f32)> = (0..e.n as usize).filter(|&q| (news[base + q] & LUMP_BITS) >> LUMP_SHIFT == 1).map(|q| (wk[4 * q], wk[4 * q + 1])).collect();
                let n = m.len().max(1) as f32;
                let (cx, cy) = (m.iter().map(|p| p.0).sum::<f32>() / n, m.iter().map(|p| p.1).sum::<f32>() / n);
                let (mut re, mut im) = (0.0f32, 0.0f32);
                for (x, y) in &m {
                    let a = 3.0 * (y - cy).atan2(x - cx);
                    re += a.cos();
                    im += a.sin();
                }
                out.push(((re * re + im * im).sqrt() / n, im.atan2(re)));
            }
            let end = g.world.lumps()[0].map_or(start, |l| (l.0, l.1));
            (out, end.0 - start.0)
        };
        let (still, _) = run(0.0, 0.0);
        let (spun, _) = run(0.2, 0.0);
        // Walking to an aim 40 cells off, it keeps most of a still lump's pace.
        let (_, plain) = run(0.0, 40.0);
        let (_, turning) = run(0.2, 40.0);
        eprintln!("pinwheel: walked {turning:.1} cells in 4 s turning, {plain:.1} plain");
        let mean = |v: &[(f32, f32)]| v[4..].iter().map(|p| p.0).sum::<f32>() / (v.len() - 4) as f32;
        // How far the phase went, unwrapped, after the first second.
        let turned = spun[4..].windows(2).map(|p| {
            let mut d = p[1].1 - p[0].1;
            while d > std::f32::consts::PI { d -= std::f32::consts::TAU; }
            while d < -std::f32::consts::PI { d += std::f32::consts::TAU; }
            d
        }).sum::<f32>();
        eprintln!("pinwheel: h3 still {:.3}, spun {:.3}; phase moved {turned:.2} rad in 3 s", mean(&still), mean(&spun));
        assert!(mean(&spun) > 1.6 * mean(&still) && mean(&spun) > 0.1, "three arms: h3 {:.3} spun against {:.3} still", mean(&spun), mean(&still));
        assert!(turned.abs() > 2.0, "and they turn: the phase moved {turned:.2} rad in 3 s");
        assert!(turning > 0.5 * plain && plain > 5.0, "and it still walks: {turning:.1} cells turning against {plain:.1} plain");
    }

    #[test]
    fn plants_and_fire_run_on_the_gpu_as_on_the_cpu() {
        use crate::mould::big::BigDish;
        use crate::mould::plants::pack;
        use std::path::Path;
        let Some((device, queue)) = test_gpu() else { return };
        let (seed, scale, walkers) = (3, 1, 2_000);
        let dir = Path::new(env!("CARGO_MANIFEST_DIR")).join("Reports/larder-maps-2026-11-11/maps");
        let mut m = Map::load(&dir, "flats").unwrap();
        // Full cover in kind bands (sparse plants, every 7th cell, went out within
        // a second once fire caught by chance, 11-14).
        // No reeds by the burn (they never catch).
        m.plants = (0..m.ground.len()).map(|i| pack([0, 1, 3, 0][(i / m.w / 160).min(3)], 1.0)).collect();
        let mut cpu = BigDish::with_map(race_params(seed, scale, walkers), &m);
        let mut gpu = GpuWorld::with_map(&device, &queue, race_settings(seed, scale, walkers), scale, &m);
        assert!(!cpu.plant.is_empty() && gpu.e.plants);
        let home = cpu.homes[0];
        let burn = squad::Burst { at: (home.0 + 25.0, home.1), r: 6.0, burn: true, ..Default::default() };
        let k = squad::BurstRules::default();
        let (mut killed, mut fired) = ((0, 0), 0);
        let mut hot = 0.0f32;
        // Steps of 15 ticks (a game second at 15 tps), so the soldier
        // reads the fire while it burns.
        for sec in 1..=24 {
            // A soldier standing at the burn reads its heat (`UnitOut::heat`,
            // fire hurts soldiers, owner 11-14 19:18Z).
            // Just inside the burn's edge: its middle is scorched bare.
            let at = (burn.at.0 + 5.0, burn.at.1);
            let f = Frame { units: vec![squad::UnitIn { x: at.0, y: at.1, touch: 1.5, ..Default::default() }], ..Frame::default() };
            let outs: Vec<f32> = [&mut cpu as &mut dyn World, &mut gpu as &mut dyn World].into_iter().map(|w| w.frame(&f, 15).units[0].heat).collect();
            // Exact on llvmpipe; on a card (`tol`) within 1e-3: on the GTX
            // 1070 (2026-11-21, Reports/pc-batch-1070-m1-2026-11-21.md) the
            // heat at 14 s was 0.11107785 against 0.111077845, one ulp of
            // the card's float order.
            assert!(outs[0].to_bits() == outs[1].to_bits() || (outs[0] - outs[1]).abs() <= tol(-1.0, 1e-3), "{sec} s: heat under the soldier CPU {} GPU {}", outs[0], outs[1]);
            hot = hot.max(outs[0]);
            if sec == 12 {
                // A burn's own kills: the CPU's in its out, the GPU's
                // with fire's in `burned`.
                killed.0 += World::burst(&mut cpu, &burn, &k).killed;
                World::burst(&mut gpu, &burn, &k);
                // Two ticks on, the soldier stands in the burn's flames (its
                // cells here hold 0.17 of plants, out within a second).
                let outs: Vec<f32> = [&mut cpu as &mut dyn World, &mut gpu as &mut dyn World].into_iter().map(|w| w.frame(&f, 2).units[0].heat).collect();
                assert!(outs[0].to_bits() == outs[1].to_bits() || (outs[0] - outs[1]).abs() <= tol(-1.0, 1e-3), "after the burn: heat under the soldier CPU {} GPU {}", outs[0], outs[1]);
                hot = hot.max(outs[0]);
            }
            killed.0 += World::burned(&mut cpu);
            killed.1 += World::burned(&mut gpu);
            let (p, kd, fire, ch) = gpu.e.plants_of();
            let diff = |a: &[f32], b: &[f32]| a.iter().zip(b).map(|(x, y)| (x - y).abs()).fold(0.0f32, f32::max);
            let same_kind = kd.iter().zip(&cpu.plant_kind).filter(|(a, b)| a == b).count() as f32 / kd.len() as f32;
            let cfire = if cpu.fire.is_empty() { vec![0.0; p.len()] } else { cpu.fire.clone() };
            let cchar = if cpu.char.is_empty() { vec![0.0; p.len()] } else { cpu.char.clone() };
            let (dp, df, dc) = (diff(&p, &cpu.plant), diff(&fire, &cfire), diff(&ch, &cchar));
            fired = fired.max(cpu.burning);
            eprintln!("plants {sec} s: sum CPU {:.1} GPU {:.1}; max diff plants {dp:.2e} fire {df:.2e} char {dc:.2e}; kinds same {same_kind:.5}; burning {}; killed {killed:?}", cpu.plant.iter().sum::<f32>(), p.iter().sum::<f32>(), cpu.burning);
            assert!(dp <= tol(1e-5, 0.05) && df <= tol(1e-5, 0.2) && dc <= tol(1e-5, 0.2), "{sec} s: plants {dp} fire {df} char {dc}");
            assert!(same_kind >= tol(1.0, 0.99), "{sec} s: kinds {same_kind}");
        }
        assert!(fired > 0, "the burn lit the plants");
        assert!(hot > 0.0, "the soldier at the burn stood in fire");
        assert!(killed.0.abs_diff(killed.1) <= tol(0, 5 + killed.0 / 10), "fire's kills: CPU {} GPU {}", killed.0, killed.1);
    }

    /// **Still soldiers tread a firebreak** (`Game::trample`, 11-24; GPU
    /// only, so no CPU twin): the IGNITE pass's mode 2 cuts the plants in
    /// the disc to its cap and keeps their kind, leaves the plants outside
    /// it and any under it already short, and doesn't tread out a fire.
    #[test]
    fn trodden_ground_is_short_plants_and_fire_is_not_trodden_out() {
        use crate::mould::plants::pack;
        use std::path::Path;
        let Some((device, queue)) = test_gpu() else { return };
        let (seed, scale, walkers) = (3, 1, 500);
        let dir = Path::new(env!("CARGO_MANIFEST_DIR")).join("Reports/larder-maps-2026-11-11/maps");
        let mut m = Map::load(&dir, "flats").unwrap();
        // Grass at full, but a short strip at 0.05 through the disc (the
        // map is four times the dish a side).
        m.plants = (0..m.ground.len()).map(|i| pack(1, if i / m.w / 4 == 30 { 0.05 } else { 1.0 })).collect();
        let mut gpu = GpuWorld::with_map(&device, &queue, race_settings(seed, scale, walkers), scale, &m);
        let w = World::size(&gpu).0;
        let (a0, k0, _, _) = gpu.e.plants_of();
        // Taller than the cap (the soil caps some grass short of 1).
        let open = |i: usize| a0[i] > 0.1;
        // Fire lit at (120, 30) first, then two discs trodden: one over
        // open grass and the strip, one over the fire.
        gpu.e.ignite(&gpu.shared, &[(120.0, 30.0, 2.0, 1.0)]);
        let (_, _, fire0, _) = gpu.e.plants_of();
        World::trample(&mut gpu, &[(60.0, 30.0, 4.0, 0.1), (120.0, 30.0, 4.0, 0.1)]);
        let (a, k, fire, _) = gpu.e.plants_of();
        let (mut trod, mut kept, mut burning) = (0, 0, 0);
        for y in 20..40 {
            for x in 50..130 {
                let i = y * w + x;
                let near = |cx: f32| (x as f32 - cx).hypot(y as f32 - 30.0) <= 4.0;
                if near(60.0) && open(i) {
                    assert!((a[i] - 0.1).abs() < 1e-6, "trodden to 0.1 at ({x}, {y}): {}", a[i]);
                    assert_eq!(k[i], k0[i], "the kind kept");
                    trod += 1;
                } else if fire0[i] > 0.0 {
                    assert_eq!(a[i], a0[i], "a burning cell isn't trodden out ({x}, {y})");
                    assert!(fire[i] > 0.0);
                    burning += 1;
                } else if !near(60.0) && !near(120.0) || !open(i) {
                    assert_eq!(a[i], a0[i], "untouched at ({x}, {y})");
                    kept += 1;
                }
            }
        }
        eprintln!("trodden {trod}, burning left {burning}, untouched {kept}");
        assert!(trod > 30 && burning > 5 && kept > 1000);
        assert_eq!(World::size(&gpu), (m.w / 4, m.h / 4), "the dish a quarter of the map a side");
    }

    /// **A falling star's crater lights no plants** (`Burst::unlit`,
    /// 11-24): on grass a crater lights the plants round it, the same
    /// crater marked unlit none, on both worlds.
    #[test]
    fn an_unlit_crater_lights_no_plants() {
        use crate::mould::big::BigDish;
        use crate::mould::plants::pack;
        use std::path::Path;
        let Some((device, queue)) = test_gpu() else { return };
        let (seed, scale, walkers) = (3, 1, 500);
        let dir = Path::new(env!("CARGO_MANIFEST_DIR")).join("Reports/larder-maps-2026-11-11/maps");
        let mut m = Map::load(&dir, "flats").unwrap();
        m.plants = vec![pack(1, 1.0); m.ground.len()];
        let k = squad::BurstRules::default();
        for unlit in [false, true] {
            let mut gpu = GpuWorld::with_map(&device, &queue, race_settings(seed, scale, walkers), scale, &m);
            let mut cpu = BigDish::with_map(race_params(seed, scale, walkers), &m);
            let b = squad::Burst { at: (60.0, 30.0), r: 3.0, kill: true, crater: 300, crater_r: 6.0, unlit, ..squad::Burst::default() };
            World::burst(&mut gpu, &b, &k);
            let lit0 = cpu.burning;
            World::burst(&mut cpu, &b, &k);
            let (_, _, fire, _) = gpu.e.plants_of();
            let lit = (cpu.burning - lit0, fire.iter().filter(|&&v| v > 0.0).count());
            eprintln!("unlit {unlit}: lit {} on the CPU, {} on the GPU", lit.0, lit.1);
            if unlit {
                assert_eq!(lit, (0, 0), "an unlit crater lit plants");
            } else {
                assert!(lit.0 > 0 && lit.1 > 0, "a crater lights the grass: {lit:?}");
            }
        }
    }

    /// **A squad walking into fire still burns; one holding still is
    /// spared most of it** (`Game::trample`, 11-24, Direction's check): on
    /// grass, eight soldiers stand 3 s, then either walk east through a
    /// fire lit across their way (`fireway` off, so straight in) or hold
    /// while four fires are lit 7 cells round them, to 18 s. Trodden ground
    /// changes nothing for the walkers (fire's harm within a fifth either
    /// way; 14 and 14 health on llvmpipe), and the held squad loses under
    /// half the health it does without it (3 against 20).
    #[test]
    fn a_squad_walking_into_fire_still_burns_and_one_holding_still_is_spared() {
        use crate::mould::plants::pack;
        use std::path::Path;
        let Some((device, queue)) = test_gpu() else { return };
        let (seed, scale, walkers) = (3, 1, 200);
        let dir = Path::new(env!("CARGO_MANIFEST_DIR")).join("Reports/larder-maps-2026-11-11/maps");
        let mut m = Map::load(&dir, "flats").unwrap();
        m.plants = vec![pack(1, 1.0); m.ground.len()];
        let play = |to: f32, walk: bool| -> f32 {
            let w = GpuWorld::with_map(&device, &queue, race_settings(seed, scale, walkers), scale, &m);
            let mut rules = Rules { squads: true, larder: false, seek: false, overrun: 0.0, max_squad: 8, start_bank: 1e9, fireway: false, trample_to: to, ..Rules::default() };
            rules.mass = Rules::mass_for(walkers, scale);
            let mut g = Game::new(w, rules, seed);
            while g.alive() < 8 && g.buy() {}
            for k in 0..g.soldiers.len() {
                g.apply(Cmd::Join(k, 0));
            }
            while g.time() < 3.0 {
                g.step();
            }
            let s = g.world.scale();
            let (x, y) = g.middle(0).unwrap();
            let (x, y) = (x / s, y / s);
            if walk {
                g.world.e.ignite(&g.world.shared, &[(x + 12.0, y, 4.0, 1.0)]);
                g.apply(Cmd::SquadOrder(0, Order::Move((x + 30.0) * s, y * s)));
            } else {
                let ring: Vec<_> = [(7.0, 0.0), (-7.0, 0.0), (0.0, 7.0), (0.0, -7.0)].iter().map(|d| (x + d.0, y + d.1, 3.0, 1.0)).collect();
                g.world.e.ignite(&g.world.shared, &ring);
            }
            let (hp0, bit0) = (g.soldiers.iter().map(|q| q.hp.max(0.0)).sum::<f32>(), g.bitten_hp);
            while g.time() < 18.0 && g.end.is_none() {
                g.step();
            }
            let hp1 = g.soldiers.iter().map(|q| q.hp.max(0.0)).sum::<f32>();
            hp0 - hp1 - (g.bitten_hp - bit0)
        };
        let (walk_off, walk_on, hold_off, hold_on) = (play(0.0, true), play(0.1, true), play(0.0, false), play(0.1, false));
        eprintln!("fire's harm: walking {walk_off:.0} untrodden, {walk_on:.0} trodden; holding {hold_off:.0} untrodden, {hold_on:.0} trodden");
        assert!(walk_off > 5.0, "the walk crossed the fire: {walk_off}");
        assert!((walk_on - walk_off).abs() <= 0.2 * walk_off, "walking into fire still burns: {walk_on} against {walk_off}");
        assert!(hold_off > 10.0, "the fire reached the held squad: {hold_off}");
        assert!(hold_on < 0.5 * hold_off, "the trodden ground spared the held squad: {hold_on} against {hold_off}");
    }

    /// **A Torch Grenade burns the squad that throws it at its own feet**
    /// (gun step E3; Direction 11-24 07:01Z, the owner's Noita rule: a
    /// very strong card is also dangerous): on grass, a held squad with
    /// the trodden firebreak on, a Grenade carrying a Torch thrown on its
    /// own middle costs it health to fire, one thrown 16 cells off costs
    /// it little, and Fireproof's first level halves the harm at its feet
    /// (llvmpipe: 78 of 800 health at its feet, 20 at 8 cells, none from
    /// 12; 39 with Fireproof; trodden or not the same, as the Torch brings
    /// its brush, `RoundRules::device_brush`; before it, 7 trodden and 28
    /// untrodden at its feet).
    #[test]
    fn a_torch_grenade_at_the_squads_feet_burns_it_and_one_thrown_off_does_not() {
        use crate::mould::kit::Tool;
        use crate::mould::larder_world::DEVICE_DECKS;
        use crate::mould::plants::pack;
        use crate::mould::rounds::{parse, Deck};
        use std::path::Path;
        let Some((device, queue)) = test_gpu() else { return };
        let (seed, scale, walkers) = (3, 1, 200);
        let dir = Path::new(env!("CARGO_MANIFEST_DIR")).join("Reports/larder-maps-2026-11-11/maps");
        let mut m = Map::load(&dir, "flats").unwrap();
        m.plants = vec![pack(1, 1.0); m.ground.len()];
        let play = |off: f32, share: f32| -> f32 {
            let w = GpuWorld::with_map(&device, &queue, race_settings(seed, scale, walkers), scale, &m);
            let mut rules = Rules { squads: true, larder: false, seek: false, overrun: 0.0, max_squad: 8, start_bank: 1e9, fireway: false, ..Rules::default() };
            rules.mass = Rules::mass_for(walkers, scale);
            rules.fire_hurt *= share;
            let mut g = Game::new(w, rules, seed);
            while g.alive() < 8 && g.buy() {}
            for k in 0..g.soldiers.len() {
                g.apply(Cmd::Join(k, 0));
            }
            assert!(g.apply(Cmd::Load(DEVICE_DECKS, Some(Deck::new(&parse("torch").unwrap())))));
            while g.time() < 3.0 {
                g.step();
            }
            let s = g.world.scale();
            let (x, y) = g.middle(0).unwrap();
            assert!(g.device(0, Tool::Blast((x + off * s, y))), "the grenade went off");
            let (hp0, bit0) = (g.soldiers.iter().map(|q| q.hp.max(0.0)).sum::<f32>(), g.bitten_hp);
            while g.time() < 18.0 && g.end.is_none() {
                g.step();
            }
            let hp1 = g.soldiers.iter().map(|q| q.hp.max(0.0)).sum::<f32>();
            hp0 - hp1 - (g.bitten_hp - bit0)
        };
        let (feet, off, proof) = (play(0.0, 1.0), play(16.0, 1.0), play(0.0, crate::mould::run::fireproof_share(1)));
        eprintln!("a torch grenade's fire harm: at the squad's feet {feet:.0}, 16 cells off {off:.0}, at its feet with Fireproof 1 {proof:.0}");
        assert!(feet > 40.0, "a torch at its own feet burned the squad: {feet}");
        assert!(off < 0.25 * feet, "a torch thrown off spared it: {off} against {feet}");
        assert!(proof < 0.75 * feet, "Fireproof cut the harm: {proof} against {feet}");
    }

    /// **A torch lights the plants on the GPU as on the CPU** (the gun
    /// cards' element family, `rounds::Card::Torch`, 2026-11-17): the
    /// plant twin's map, the torch's burst from its card at 10 s on the
    /// slime in the thickest grass by the home; the cells it lights, the
    /// fire field after, the fire's kills and the living walkers match
    /// (exact on llvmpipe, a little slack on a card). It ran with heat in
    /// the sensors too (`fire_weight` -20) until that was deleted 11-17
    /// for `fire_alarm` (Reports/dead-ends.md).
    #[test]
    fn a_torch_lights_the_plants_on_the_gpu_as_on_the_cpu() {
        use crate::mould::big::BigDish;
        use crate::mould::plants::pack;
        use crate::mould::rounds;
        use std::path::Path;
        let Some((device, queue)) = test_gpu() else { return };
        let (seed, scale, walkers) = (3, 1, 2_000);
        let dir = Path::new(env!("CARGO_MANIFEST_DIR")).join("Reports/larder-maps-2026-11-11/maps");
        let mut m = Map::load(&dir, "flats").unwrap();
        m.plants = (0..m.ground.len()).map(|i| pack([0, 1, 3, 0][(i / m.w / 160).min(3)], 1.0)).collect();
        let mut cpu = BigDish::with_map(race_params(seed, scale, walkers), &m);
        let mut gpu = GpuWorld::with_map(&device, &queue, race_settings(seed, scale, walkers), scale, &m);
        let k = squad::BurstRules::default();
        let f = Frame::default();
        for _ in 0..10 {
            for w in [&mut cpu as &mut dyn World, &mut gpu as &mut dyn World] {
                w.frame(&f, 15);
            }
        }
        // The CPU's walker in the thickest grass within 40 dish cells of
        // the home (the rounds' own torch test's pick).
        let home = cpu.homes[0];
        let s = cpu.p.scale as f32;
        let fuel = |p: (f32, f32)| {
            let (cx, cy) = (p.0 as i64, p.1 as i64);
            let mut f = 0.0f32;
            for y in (cy - 3).max(0)..=(cy + 3).min(cpu.h as i64 - 1) {
                for x in (cx - 3).max(0)..=(cx + 3).min(cpu.w as i64 - 1) {
                    f += cpu.plant[y as usize * cpu.w + x as usize];
                }
            }
            f
        };
        let at = (0..cpu.walkers.len())
            .filter(|&k| cpu.lives(k))
            .map(|k| (cpu.walkers[k].0, cpu.walkers[k].1))
            .filter(|p| (p.0 / s - home.0).hypot(p.1 / s - home.1) <= 40.0)
            .max_by(|a, b| fuel(*a).total_cmp(&fuel(*b)))
            .unwrap();
        let cast = rounds::casts(&rounds::parse("torch").unwrap())[0];
        let b = rounds::bursts(&cast, at, &rounds::RoundRules::default(), s, 15.0)[0];
        assert!(b.light > 0.0 && !b.kill && !b.burn);
        let lit0 = cpu.burning;
        World::burst(&mut cpu, &b, &k);
        World::burst(&mut gpu, &b, &k);
        let lit = cpu.burning - lit0;
        let (_, _, fire, _) = gpu.e.plants_of();
        let gpu_lit = fire.iter().filter(|&&v| v >= 1.0).count();
        eprintln!("the torch lit {lit} cells on the CPU, {gpu_lit} at full heat on the GPU");
        assert!(lit > 50, "it lit the grass: {lit}");
        assert!(lit.abs_diff(gpu_lit) <= tol(0, 2 + lit / 20), "lit: CPU {lit} GPU {gpu_lit}");
        let mut killed = (0, 0);
        for sec in 1..=4 {
            for w in [&mut cpu as &mut dyn World, &mut gpu as &mut dyn World] {
                w.frame(&f, 15);
            }
            killed.0 += World::burned(&mut cpu);
            killed.1 += World::burned(&mut gpu);
            let (_, _, fire, _) = gpu.e.plants_of();
            let cfire = if cpu.fire.is_empty() { vec![0.0; fire.len()] } else { cpu.fire.clone() };
            let df = fire.iter().zip(&cfire).map(|(x, y)| (x - y).abs()).fold(0.0f32, f32::max);
            let living = (World::living(&cpu), World::living(&gpu));
            eprintln!("torch {sec} s: burning {}; max diff fire {df:.2e}; killed {killed:?}; living {living:?}", cpu.burning);
            assert!(df <= tol(1e-5, 0.2), "{sec} s: fire {df}");
            assert!(living.0.abs_diff(living.1) <= tol(0, 10 + living.0 / 50), "{sec} s: living CPU {} GPU {}", living.0, living.1);
        }
        assert!(killed.0 > 0, "the fire killed walkers");
        assert!(killed.0.abs_diff(killed.1) <= tol(0, 5 + killed.0 / 10), "fire's kills: CPU {} GPU {}", killed.0, killed.1);
    }

    /// **Tinder lays and burns on the GPU as on the CPU** (the gun cards'
    /// element family, 2026-11-17; `squad::Burst::fuel`, `Engine::lay`): the
    /// flats map planted in its top band only, its bottom 30 rows barren
    /// soil (where nothing grows, so laid fuel withers by `plant_rot` to 0);
    /// three tinder lines, one across bare ground from 15 dish cells out to
    /// the slime's edge (scrub over its capacity), one through the meadow
    /// (over a meadow's), one on the barren rows; a torch at the first
    /// line's outer end. The
    /// plants and kinds match exactly after the laying, and plants, fire,
    /// the fire's kills and the living match every second for 10 s.
    #[test]
    fn tinder_lays_and_burns_on_the_gpu_as_on_the_cpu() {
        use crate::mould::big::{BigDish, SOIL_BARREN};
        use crate::mould::plants::pack;
        use crate::mould::rounds;
        use std::path::Path;
        let Some((device, queue)) = test_gpu() else { return };
        let (seed, scale, walkers) = (3, 1, 2_000);
        let dir = Path::new(env!("CARGO_MANIFEST_DIR")).join("Reports/larder-maps-2026-11-11/maps");
        let mut m = Map::load(&dir, "flats").unwrap();
        m.plants = (0..m.ground.len()).map(|i| if i / m.w < 40 { pack(0, 1.0) } else { 0 }).collect();
        let rows = m.ground.len() / m.w;
        for i in 0..m.ground.len() {
            if i / m.w >= rows - 30 {
                m.ground[i] = (m.ground[i] & 0x0F) | (SOIL_BARREN << 4);
            }
        }
        let mut cpu = BigDish::with_map(race_params(seed, scale, walkers), &m);
        let mut gpu = GpuWorld::with_map(&device, &queue, race_settings(seed, scale, walkers), scale, &m);
        let k = squad::BurstRules::default();
        let f = Frame::default();
        for _ in 0..10 {
            for w in [&mut cpu as &mut dyn World, &mut gpu as &mut dyn World] {
                w.frame(&f, 15);
            }
        }
        let home = cpu.homes[0];
        let s = cpu.p.scale as f32;
        let far = |p: &(f32, f32)| (p.0 / s - home.0).hypot(p.1 / s - home.1);
        let edge = (0..cpu.walkers.len())
            .filter(|&k| cpu.lives(k))
            .map(|k| (cpu.walkers[k].0, cpu.walkers[k].1))
            .filter(|p| far(p) <= 40.0)
            .max_by(|a, b| far(a).total_cmp(&far(b)))
            .unwrap();
        let (dx, dy) = (edge.0 / s - home.0, edge.1 / s - home.1);
        let d = dx.hypot(dy).max(1e-3);
        let out = (edge.0 + 15.0 * s * dx / d, edge.1 + 15.0 * s * dy / d);
        // Whole lines (`trail_bursts` lays no gap; play's is `keep_clear`).
        let r = rounds::RoundRules::default();
        let n = rounds::trails(&[rounds::Card::Tinder]);
        let mut discs = rounds::trail_bursts(n, out, edge, &r, s, 15.0, 1.0);
        discs.extend(rounds::trail_bursts(n, (20.0 * s, 20.0 * s), (60.0 * s, 20.0 * s), &r, s, 15.0, 1.0));
        let low = (rows - 15) as f32 * s;
        discs.extend(rounds::trail_bursts(n, (20.0 * s, low), (60.0 * s, low), &r, s, 15.0, 1.0));
        for b in &discs {
            World::burst(&mut cpu, b, &k);
            World::burst(&mut gpu, b, &k);
        }
        let (plants, kinds, _, _) = gpu.e.plants_of();
        let laid = cpu.plant.iter().filter(|&&p| p >= r.trail_fuel).count();
        let dp = plants.iter().zip(&cpu.plant).map(|(x, y)| (x - y).abs()).fold(0.0f32, f32::max);
        let dk = kinds.iter().zip(&cpu.plant_kind).filter(|(a, b)| a != b).count();
        eprintln!("tinder: {} discs, {laid} cells laid on the CPU; max diff plants {dp:.2e}, kinds differing {dk}", discs.len());
        assert!(laid > 100, "both lines laid: {laid}");
        // Exact on llvmpipe; a card's float order leaves an ulp (the GTX
        // 1070, 11-21: 1.9e-9, kinds all the same).
        assert!(dp <= tol(0.0, 1e-6) && dk == 0, "laid: plants {dp}, kinds {dk}");
        let cast = rounds::casts(&rounds::parse("torch").unwrap())[0];
        let b = rounds::bursts(&cast, out, &r, s, 15.0)[0];
        World::burst(&mut cpu, &b, &k);
        World::burst(&mut gpu, &b, &k);
        let mut killed = (0, 0);
        for sec in 1..=10 {
            for w in [&mut cpu as &mut dyn World, &mut gpu as &mut dyn World] {
                w.frame(&f, 15);
            }
            killed.0 += World::burned(&mut cpu);
            killed.1 += World::burned(&mut gpu);
            let (plants, _, fire, _) = gpu.e.plants_of();
            let cfire = if cpu.fire.is_empty() { vec![0.0; fire.len()] } else { cpu.fire.clone() };
            let df = fire.iter().zip(&cfire).map(|(x, y)| (x - y).abs()).fold(0.0f32, f32::max);
            let dp = plants.iter().zip(&cpu.plant).map(|(x, y)| (x - y).abs()).fold(0.0f32, f32::max);
            let living = (World::living(&cpu), World::living(&gpu));
            let laid = cpu.plant.iter().filter(|&&p| p > 0.8).count();
            eprintln!("tinder {sec} s: burning {}, over 0.8 {laid}; max diff plants {dp:.2e} fire {df:.2e}; killed {killed:?}; living {living:?}", cpu.burning);
            assert!(dp <= tol(1e-5, 0.2) && df <= tol(1e-5, 0.2), "{sec} s: plants {dp}, fire {df}");
            assert!(living.0.abs_diff(living.1) <= tol(0, 10 + living.0 / 50), "{sec} s: living CPU {} GPU {}", living.0, living.1);
        }
        assert!(killed.0 > 0, "the fire down the line killed walkers");
        assert!(killed.0.abs_diff(killed.1) <= tol(0, 5 + killed.0 / 10), "fire's kills: CPU {} GPU {}", killed.0, killed.1);
    }

    /// **Crust burns, and its char is barren, on the GPU as on the CPU**
    /// (the simulation-use plan's C2, 2026-11-17; `BigParams::crust_fuel`,
    /// `char_barren`): the plant twin's map with the crust on and worth a
    /// plant a unit, char over 0.3 barren, the crust's pull and fast ground
    /// on too; a burn lit at 12 s. Plants, fire, char and crust match at
    /// every check, and the walkers' count. The plants are under the fire's
    /// fuel, so a fire still burning at 24 s, with barren char, ran on the
    /// crust.
    #[test]
    fn crust_burns_on_the_gpu_as_on_the_cpu() {
        use crate::mould::big::BigDish;
        use crate::mould::plants::pack;
        use std::path::Path;
        let Some((device, queue)) = test_gpu() else { return };
        let (seed, scale, walkers) = (3, 1, 2_000);
        let dir = Path::new(env!("CARGO_MANIFEST_DIR")).join("Reports/larder-maps-2026-11-11/maps");
        let mut m = Map::load(&dir, "flats").unwrap();
        // Thin plants (0.04 a cell, under the fire's fuel of 0.05), so only
        // the crust carries a fire.
        m.plants = (0..m.ground.len()).map(|i| pack([0, 1, 3, 0][(i / m.w / 160).min(3)], 0.04)).collect();
        let (rise, keep, weight, fast, fuel, barren) = (0.02, 0.999, 0.5, 0.5, 1.0, 0.3);
        let p = BigParams { crust_rise: rise, crust_keep: keep, crust_trail: 1.0, crust_weight: weight, crust_fast: fast, crust_fuel: fuel, char_barren: barren, ..race_params(seed, scale, walkers) };
        let s = Settings { crust_rise: rise, crust_keep: keep, crust_trail: 1.0, crust_weight: weight, crust_fast: fast, crust_fuel: fuel, char_barren: barren, ..race_settings(seed, scale, walkers) };
        let mut cpu = BigDish::with_map(p, &m);
        let mut gpu = GpuWorld::with_map(&device, &queue, s, scale, &m);
        assert!(!cpu.plant.is_empty() && gpu.e.plants && !cpu.crust.is_empty() && gpu.e.crust_at > 0);
        let home = cpu.homes[0];
        let burn = squad::Burst { at: (home.0 + 25.0, home.1), r: 10.0, burn: true, ..Default::default() };
        let k = squad::BurstRules::default();
        let (mut burning, mut barren_cells) = (0, 0);
        for sec in 1..=24 {
            let f = Frame::default();
            for w in [&mut cpu as &mut dyn World, &mut gpu as &mut dyn World] {
                w.frame(&f, 15);
            }
            if sec == 12 {
                World::burst(&mut cpu, &burn, &k);
                World::burst(&mut gpu, &burn, &k);
                for w in [&mut cpu as &mut dyn World, &mut gpu as &mut dyn World] {
                    w.frame(&f, 30);
                }
            }
            World::burned(&mut cpu);
            World::burned(&mut gpu);
            let (p, _, fire, ch) = gpu.e.plants_of();
            let crust = gpu.e.crust_of();
            let diff = |a: &[f32], b: &[f32]| a.iter().zip(b).map(|(x, y)| (x - y).abs()).fold(0.0f32, f32::max);
            let cfire = if cpu.fire.is_empty() { vec![0.0; p.len()] } else { cpu.fire.clone() };
            let cchar = if cpu.char.is_empty() { vec![0.0; p.len()] } else { cpu.char.clone() };
            let (dp, df, dc, dk) = (diff(&p, &cpu.plant), diff(&fire, &cfire), diff(&ch, &cchar), diff(&crust, &cpu.crust));
            let (nc, ng) = (World::living(&cpu), World::living(&gpu));
            let barren = cchar.iter().filter(|&&c| c >= barren).count();
            eprintln!("crust fire {sec} s: max diff plants {dp:.2e} fire {df:.2e} char {dc:.2e} crust {dk:.2e}; burning {}; {barren} barren cells; walkers CPU {nc} GPU {ng}", cpu.burning);
            // On a card the crust parts most (the GTX 1070, 11-21: 0.12 at
            // 17 s from ulps in plants and fire of 2e-8 and 1e-7).
            assert!(dp <= tol(1e-5, 0.05) && df <= tol(1e-5, 0.2) && dc <= tol(1e-5, 0.2) && dk <= tol(1e-5, 0.25), "{sec} s: plants {dp} fire {df} char {dc} crust {dk}");
            assert!(nc.abs_diff(ng) <= tol(0, 5 + nc / 50), "{sec} s: walkers CPU {nc} GPU {ng}");
            (burning, barren_cells) = (cpu.burning, barren);
        }
        assert!(burning > 50 && barren_cells > 50, "the crust carried the fire: {burning} burning, {barren_cells} barren at 24 s");
    }

    /// **Dry slime burns, and fed slime is fireproof, on the GPU as on the
    /// CPU** (the simulation-use plan's C1, 2026-11-17; `BigParams::
    /// fire_dry`, `fire_fed`): the plant twin's map
    /// in full plants, the food news on (a wither too rare to kill anyone)
    /// with a short lag, so the body away from its crystal is stale, and
    /// every plant within 20 dish cells of home lit at 12 s. Plants, fire and char match at every
    /// check, and the walkers' count, with `fire_dry` on and with both.
    /// Planted: a GPU that ignores either must fail it. And the fire burns
    /// into stale slime (more fire kills with `fire_dry` than without) and
    /// spares the fed (fewer with `fire_fed` too).
    #[test]
    fn dry_slime_burns_on_the_gpu_as_on_the_cpu() {
        use crate::mould::big::BigDish;
        use crate::mould::plants::pack;
        use std::path::Path;
        let Some((device, queue)) = test_gpu() else { return };
        let (seed, scale, walkers) = (3, 1, 2_000);
        let dir = Path::new(env!("CARGO_MANIFEST_DIR")).join("Reports/larder-maps-2026-11-11/maps");
        let mut m = Map::load(&dir, "flats").unwrap();
        m.plants = (0..m.ground.len()).map(|i| pack([0, 1, 3, 0][(i / m.w / 160).min(3)], 1.0)).collect();
        let (wither, lag) = (1e-12, 2);
        // (largest fire difference over the run, CPU fire kills, stale
        // walkers on the CPU at the burn)
        // Each engine's (fire_dry, fire_fed).
        let play = |(dry, fed): (bool, bool), (gpu_dry, gpu_fed): (bool, bool)| -> (f32, u64, usize) {
            let p = BigParams { wither, wither_lag: lag, fire_dry: dry, fire_fed: fed, ..race_params(seed, scale, walkers) };
            let s = Settings { wither, wither_lag: lag, fire_dry: gpu_dry, fire_fed: gpu_fed, ..race_settings(seed, scale, walkers) };
            let mut cpu = BigDish::with_map(p, &m);
            let mut gpu = GpuWorld::with_map(&device, &queue, s, scale, &m);
            assert!(!cpu.plant.is_empty() && gpu.e.plants && gpu.e.withers);
            let home = cpu.homes[0];
            let start: f64 = cpu.plant.iter().map(|&v| v as f64).sum();
            let (mut worst, mut stale) = (0.0f32, 0);
            for sec in 1..=24 {
                let f = Frame::default();
                for w in [&mut cpu as &mut dyn World, &mut gpu as &mut dyn World] {
                    w.frame(&f, 15);
                }
                if sec == 12 {
                    let now = cpu.steps;
                    stale = cpu.heard.iter().filter(|&&v| v > 0 && v + (lag as u64) < now).count();
                    let wet = cpu.trail.iter().filter(|&&t| t >= cpu.p.fire_damp).count();
                    let dried = cpu.slime_fire().iter().filter(|&&d| d == crate::mould::plants::SLIME_DRY).count();
                    let fuel = (0..cpu.plant.len()).filter(|&i| cpu.plant[i] >= cpu.p.fire_fuel && cpu.trail[i] >= cpu.p.fire_damp).count();
                    eprintln!("12 s: {wet} cells wet by trail, {dried} marked dry, {fuel} wet with fuel; living {}", World::living(&cpu));
                    // Every cell with fuel within 20 dish cells of home
                    // lit, wet or not: the fire step puts out what is wet.
                    cpu.ignite(home.0, home.1, 20.0, 1.0);
                    gpu.e.ignite(&gpu.shared, &[(home.0, home.1, 20.0, 1.0)]);
                    for w in [&mut cpu as &mut dyn World, &mut gpu as &mut dyn World] {
                        w.frame(&f, 30);
                    }
                }
                World::burned(&mut cpu);
                World::burned(&mut gpu);
                let (pl, _, fire, ch) = gpu.e.plants_of();
                let diff = |a: &[f32], b: &[f32]| a.iter().zip(b).map(|(x, y)| (x - y).abs()).fold(0.0f32, f32::max);
                let cfire = if cpu.fire.is_empty() { vec![0.0; pl.len()] } else { cpu.fire.clone() };
                let cchar = if cpu.char.is_empty() { vec![0.0; pl.len()] } else { cpu.char.clone() };
                let (dp, df, dc) = (diff(&pl, &cpu.plant), diff(&fire, &cfire), diff(&ch, &cchar));
                let (nc, ng) = (World::living(&cpu), World::living(&gpu));
                if (dry, fed) == (gpu_dry, gpu_fed) {
                    if ON_CARD.load(std::sync::atomic::Ordering::Relaxed) {
                        // On a card the fire front drifts a cell from the
                        // CPU's as the burn goes on, so a cell-by-cell bound
                        // only grows (the GTX 1070, 11-21: plants 0.018 at
                        // 19 s, 0.074 at 21 s; fire 0.26, char 0.19): it
                        // judges the burn's totals instead. On llvmpipe,
                        // 11-21: an engine with the other `fire_fed` burns
                        // 2.5 to 3 times the plants and char of the CPU's
                        // (92 plants against 33 at 24 s; char 62 against
                        // 24), the same rules but a different burn 5 to 10%
                        // apart; the bounds sit between.
                        let tot = |a: &[f32]| a.iter().map(|&v| v as f64).sum::<f64>();
                        let near = |a: f64, b: f64, rel: f64| (a - b).abs() <= rel * b.abs() + 1.0;
                        let (burnt, gburnt) = (start - tot(&cpu.plant), start - tot(&pl));
                        let (cf, gf, cc, gc) = (tot(&cfire), tot(&fire), tot(&cchar), tot(&ch));
                        assert!(near(gburnt, burnt, 0.25) && near(gf, cf, 0.4) && near(gc, cc, 0.25), "{sec} s: plants burnt CPU {burnt:.1} GPU {gburnt:.1}, fire {cf:.1} {gf:.1}, char {cc:.1} {gc:.1}");
                    } else {
                        assert!(dp <= 1e-5 && df <= 1e-5 && dc <= 1e-5, "{sec} s: plants {dp} fire {df} char {dc}");
                    }
                    assert!(nc.abs_diff(ng) <= tol(0, 5 + nc / 50), "{sec} s: walkers CPU {nc} GPU {ng}");
                }
                worst = worst.max(df);
            }
            (worst, cpu.fire_killed, stale)
        };
        let (df, dry_kills, stale) = play((true, false), (true, false));
        eprintln!("dry slime burns: largest fire difference {df:.2e}; {stale} cells of stale news at the burn; fire killed {dry_kills}");
        assert!(stale > 100, "the news went stale: {stale} cells");
        let (_, wet_kills, _) = play((false, false), (false, false));
        eprintln!("fed and stale alike wet: fire killed {wet_kills}");
        assert!(dry_kills > wet_kills, "fire burns into stale slime: {dry_kills} against {wet_kills}");
        let (df, _, _) = play((true, false), (false, false));
        eprintln!("planted, the GPU ignores fire_dry: largest fire difference {df:.2e}");
        assert!(df > 0.2, "the twin catches a GPU that ignores fire_dry");
        let (df, fed_kills, _) = play((true, true), (true, true));
        eprintln!("fed slime fireproof too: largest fire difference {df:.2e}; fire killed {fed_kills}");
        assert!(fed_kills < dry_kills, "fed slime is fireproof: {fed_kills} against {dry_kills}");
        let (df, _, _) = play((true, true), (true, false));
        eprintln!("planted, the GPU ignores fire_fed: largest fire difference {df:.2e}");
        assert!(df > 0.2, "the twin catches a GPU that ignores fire_fed");
    }

    /// **Fire scares it** (the simulation-use plan's C4, `fire_alarm`), on
    /// the C1 twin's dish with the flinch on (keep 0.9, as shape C): every
    /// plant within 20 dish cells of home lit at 12 s. The alarm field,
    /// plants, fire and char agree at every second; fire lays alarm (the
    /// alarm over the dish rises against `fire_alarm` 0). Planted: a GPU
    /// with `fire_alarm` 0 must be caught by the alarm.
    #[test]
    fn fire_lays_alarm_on_the_gpu_as_on_the_cpu() {
        use crate::mould::big::BigDish;
        use crate::mould::plants::pack;
        use std::path::Path;
        let Some((device, queue)) = test_gpu() else { return };
        let (seed, scale, walkers) = (3, 1, 2_000);
        let dir = Path::new(env!("CARGO_MANIFEST_DIR")).join("Reports/larder-maps-2026-11-11/maps");
        let mut m = Map::load(&dir, "flats").unwrap();
        m.plants = (0..m.ground.len()).map(|i| pack([0, 1, 3, 0][(i / m.w / 160).min(3)], 1.0)).collect();
        // (largest alarm L1 over the run, the CPU's alarm summed 1 s after the light)
        let play = |fa: f32, gpu_fa: f32| -> (f64, f64) {
            let p = BigParams { flinch: 0.9, fire_alarm: fa, ..race_params(seed, scale, walkers) };
            let s = Settings { flinch: 0.9, fire_alarm: gpu_fa, ..race_settings(seed, scale, walkers) };
            let mut cpu = BigDish::with_map(p, &m);
            let mut gpu = GpuWorld::with_map(&device, &queue, s, scale, &m);
            assert!(!cpu.plant.is_empty() && gpu.e.plants && !cpu.alarm.is_empty());
            let home = cpu.homes[0];
            let (mut worst, mut after) = (0.0f64, 0.0f64);
            for sec in 1..=16 {
                let f = Frame::default();
                for w in [&mut cpu as &mut dyn World, &mut gpu as &mut dyn World] {
                    w.frame(&f, 15);
                }
                if sec == 12 {
                    cpu.ignite(home.0, home.1, 20.0, 1.0);
                    gpu.e.ignite(&gpu.shared, &[(home.0, home.1, 20.0, 1.0)]);
                    for w in [&mut cpu as &mut dyn World, &mut gpu as &mut dyn World] {
                        w.frame(&f, 30);
                    }
                }
                World::burned(&mut cpu);
                World::burned(&mut gpu);
                let ga = gpu.e.alarm_of();
                let l1: f64 = ga.iter().zip(&cpu.alarm).map(|(a, b)| (a - b).abs() as f64).sum();
                if sec == 13 {
                    after = cpu.alarm.iter().map(|&v| v as f64).sum();
                }
                let (pl, _, fire, _) = gpu.e.plants_of();
                let diff = |a: &[f32], b: &[f32]| a.iter().zip(b).map(|(x, y)| (x - y).abs()).fold(0.0f32, f32::max);
                let cfire = if cpu.fire.is_empty() { vec![0.0; pl.len()] } else { cpu.fire.clone() };
                if fa == gpu_fa {
                    let (nc, ng) = (World::living(&cpu), World::living(&gpu));
                    let (dp, df) = (diff(&pl, &cpu.plant), diff(&fire, &cfire));
                    assert!(dp <= tol(1e-5, 0.05) && df <= tol(1e-5, 0.2), "{sec} s: plants {dp} fire {df}, alarm L1 {l1:.3e}");
                    assert!(nc.abs_diff(ng) <= tol(0, 5 + nc / 50), "{sec} s: walkers CPU {nc} GPU {ng}");
                    assert!(l1 <= tol(1e-2, 50.0), "{sec} s: alarm L1 {l1:.3e}");
                }
                worst = worst.max(l1);
            }
            (worst, after)
        };
        let (l0, off) = play(0.0, 0.0);
        eprintln!("without fire alarm: largest alarm L1 {l0:.2e}");
        let (l1, on) = play(5.0, 5.0);
        eprintln!("fire lays alarm: largest alarm L1 {l1:.2e}; alarm summed 1 s after the light {on:.1} against {off:.1} without");
        assert!(on > off + 100.0, "fire lays alarm: {on:.1} against {off:.1}");
        let (l1, _) = play(5.0, 0.0);
        eprintln!("planted, the GPU lays none: largest alarm L1 {l1:.2e}");
        assert!(l1 > 100.0, "the twin catches a GPU fire that lays no alarm");
    }

    /// **Gas ignites** (the simulation-use plan's C5, `gas_burn`): the kit's
    /// sleep gas laid over the grown body on the flats map planted whole,
    /// then a small fire lit at the cloud's edge. On the CPU the flash must
    /// run through the cloud (damp body or not), burn the sleepers and use
    /// the cloud up, where with `gas_burn` 0 the body puts the fire out.
    /// With a GPU, its fire, cloud and living must match the CPU's each
    /// second; planted, a GPU with `gas_burn` 0 must be caught.
    #[test]
    fn gas_ignites_on_the_gpu_as_on_the_cpu() {
        use crate::mould::kit::KitRules;
        use crate::mould::plants::pack;
        use crate::mould::rounds::burst_rules;
        use std::path::Path;
        let (seed, scale, walkers) = (3, 1, 2_000);
        let dir = Path::new(env!("CARGO_MANIFEST_DIR")).join("Reports/larder-maps-2026-11-11/maps");
        let mut m = Map::load(&dir, "flats").unwrap();
        m.plants = (0..m.ground.len()).map(|i| pack([0, 1, 3, 0][(i / m.w / 160).min(3)], 1.0)).collect();
        let kit = KitRules::default();
        // The CPU alone, or beside a GPU with its own `gas_burn`: (fire
        // kills in the 3 s after the light, the cloud summed at the light
        // and 3 s on, the largest fire, cloud and living gaps).
        let play = |gb: f32, gpu: Option<(&wgpu::Device, &wgpu::Queue, f32)>| -> (u64, f64, f64, f32, f64, usize) {
            let p = BigParams { gas_burn: gb, ..race_params(seed, scale, walkers) };
            let mut cpu = BigDish::with_map(p, &m);
            let mut gw = gpu.map(|(d, q, ggb)| GpuWorld::with_map(d, q, Settings { gas_burn: ggb, ..race_settings(seed, scale, walkers) }, scale, &m));
            let home = cpu.homes[0];
            let rules = burst_rules(&kit, 15.0 * World::fine_steps(&cpu) as f32);
            let gas = squad::Burst { at: home, r: kit.gas_r, gas: (kit.gas_secs * 15.0 * World::fine_steps(&cpu) as f32) as u16, ..squad::Burst::default() };
            let (mut killed, mut cloud0, mut cloud1) = (0u64, 0.0f64, 0.0f64);
            let (mut df, mut dc, mut dn) = (0.0f32, 0.0f64, 0usize);
            for sec in 1..=16 {
                let f = Frame::default();
                World::frame(&mut cpu, &f, 15);
                if let Some(g) = gw.as_mut() {
                    World::frame(g, &f, 15);
                }
                if sec == 12 {
                    World::burst(&mut cpu, &gas, &rules);
                    if let Some(g) = gw.as_mut() {
                        World::burst(g, &gas, &rules);
                    }
                }
                if sec == 13 {
                    cloud0 = cpu.gas_cloud.iter().map(|&v| v as f64).sum();
                    killed = cpu.fire_killed;
                    let at = (home.0 + kit.gas_r - 2.0, home.1);
                    cpu.ignite(at.0, at.1, 2.0, 1.0);
                    if let Some(g) = gw.as_mut() {
                        g.e.ignite(&g.shared, &[(at.0, at.1, 2.0, 1.0)]);
                    }
                }
                if sec == 16 {
                    killed = cpu.fire_killed - killed;
                    cloud1 = cpu.gas_cloud.iter().map(|&v| v as f64).sum();
                }
                World::burned(&mut cpu);
                if let Some(g) = gw.as_mut() {
                    World::burned(g);
                    let (_, _, fire, _) = g.e.plants_of();
                    let cfire = if cpu.fire.is_empty() { vec![0.0; fire.len()] } else { cpu.fire.clone() };
                    df = df.max(fire.iter().zip(&cfire).map(|(a, b)| (a - b).abs()).fold(0.0f32, f32::max));
                    let gc = g.e.gas_cloud_of();
                    if !cpu.gas_cloud.is_empty() {
                        dc = dc.max(gc.iter().zip(&cpu.gas_cloud).map(|(a, b)| (a - b).abs() as f64).sum());
                    }
                    dn = dn.max(World::living(&cpu).abs_diff(World::living(g)));
                }
            }
            (killed, cloud0, cloud1, df, dc, dn)
        };
        let (k0, c0, c0e, ..) = play(0.0, None);
        let (k1, c1, c1e, ..) = play(0.2, None);
        eprintln!("gas ignites (CPU): fire kills in 3 s {k1} against {k0} without; cloud at the light {c1:.0} -> {c1e:.0} (without {c0:.0} -> {c0e:.0})");
        assert!(k1 > k0 + 50 && k1 > 2 * k0, "the flash burns the sleepers: {k1} against {k0}");
        assert!(c1e < 0.5 * c0e, "the flash uses the cloud: {c1e:.0} against {c0e:.0}");
        let Some((device, queue)) = test_gpu() else { return };
        let (k, _, _, df, dc, dn) = play(0.2, Some((&device, &queue, 0.2)));
        eprintln!("gas ignites, twin: largest fire gap {df:.2e}, cloud L1 {dc:.2e}, living {dn} (CPU kills {k})");
        assert!(df <= tol(1e-5, 0.2) && dc <= tol(1e-2, 50.0), "fire {df}, cloud L1 {dc:.3e}");
        assert!(dn <= tol(0, 5 + walkers / 50), "living off by {dn}");
        let (_, _, _, df, dc, dn) = play(0.2, Some((&device, &queue, 0.0)));
        eprintln!("planted, the GPU's gas never burns: fire gap {df:.2e}, cloud L1 {dc:.2e}, living {dn}");
        assert!(df > 0.5 && dc > 50.0, "the twin catches a GPU whose gas never burns");
    }

    /// The CPU census's world (`race_params`, a tier's `apply`) and the GPU
    /// world (`race_settings`, `Tier::settings`, `Engine::made`) play the
    /// same crowd rule on every map, tier and scale the game ships, and on
    /// the 25,000 walkers at scale 2 that dropped it on the GPU alone until
    /// 2026-11-12 (`crowd_holds` counted the slots). No GPU needed.
    #[test]
    fn the_cpu_census_and_the_gpu_world_play_the_same_crowd_rule() {
        use crate::gpu::{agar_cells, crowd_holds, wants_crowd};
        use crate::mould::big::BigDish;
        use std::path::Path;
        let dir = Path::new(env!("CARGO_MANIFEST_DIR")).join("Reports/larder-maps-2026-11-11/maps");
        let maps: Vec<(&str, Option<Map>)> = std::iter::once(("the noise world", None)).chain(["fork", "nest", "rim", "flats", "passes"].into_iter().map(|n| (n, Some(Map::load(&dir, n).unwrap())))).collect();
        let tiers: Vec<Option<Tier>> = std::iter::once(None).chain(Tier::ALL.iter().copied().map(Some)).collect();
        let mut checked = 0;
        for (scale, walkers) in [(1, 1250), (2, 5000), (2, 25000), (4, 20000), (8, 80000), (8, 100000)] {
            for (name, m) in &maps {
                let make = |q| match m {
                    Some(m) => BigDish::with_map(q, m),
                    None => BigDish::new(q),
                };
                let agar = agar_cells(&race_settings(1, scale, walkers), scale, make);
                for &tier in &tiers {
                    let mut p = race_params(1, scale, walkers);
                    let mut s = race_settings(1, scale, walkers);
                    if let Some(t) = tier {
                        t.apply(&mut p);
                        if t.settings(&mut s).is_err() {
                            continue; // a tier the GPU world refuses never plays there
                        }
                    }
                    let cpu = p.crowd > 0 || p.species_sets.iter().any(|t| t.crowd > 0);
                    let gpu = wants_crowd(&s) && crowd_holds(s.walkers, agar);
                    assert_eq!(cpu, gpu, "{name}, scale {scale}, {walkers} walkers ({} slots, {agar} agar), tier {:?}: the CPU's crowd {cpu}, the GPU's {gpu}", s.capacity, tier.map(|t| t.name()));
                    checked += 1;
                }
            }
        }
        assert!(checked > 6 * 6 * 3, "only {checked} cases");
    }

    /// **The gardeners on the GPU** (2026-11-12): the GPU garden starts as
    /// the CPU's (the same map, the same walkers a side), and to the game
    /// the mould is the enemy alone.
    #[test]
    fn the_gpu_garden_starts_as_the_cpu_garden() {
        use crate::mould::own_mould::garden_world;
        let Some((device, queue)) = test_gpu() else { return };
        let (seed, scale, walkers) = (5, 1, 1_000);
        let gpu = GpuWorld::garden(&device, &queue, garden_settings(seed, scale, walkers), seed, scale, walkers).unwrap();
        let cpu = garden_world(seed, scale, walkers, None);
        assert_eq!(gpu.counts_by_side(), cpu.counts_by_side());
        assert_eq!(gpu.e.dish.landing, cpu.dish.landing);
        let game = Game::new(gpu, Rules::default(), seed);
        assert_eq!(game.mould, walkers, "the game's mould is the enemy's");
        // The plain world has no player's mould.
        let plain = GpuWorld::new(&device, &queue, race_settings(seed, scale, walkers), scale);
        assert_eq!(plain.counts_by_side(), (0, walkers));
    }

    /// Each side keeps its own birth reserve on both engines
    /// (`species_cap`, 11-14; the tech lead's ask for the campaign's
    /// own-mould mission): with the enemy capped at its start, it stays
    /// within a few dozen of the cap (conversion, not birth) while the
    /// player's mould keeps its own; the two engines count the same each
    /// frame; a
    /// taken core (`stop_births`) stops only the enemy's births. Without
    /// the cap the same enemy outgrows it (else this proves nothing).
    #[test]
    fn gpu_garden_keeps_each_sides_birth_reserve_as_the_cpu() {
        use crate::mould::own_mould::garden_world;
        let Some((device, queue)) = test_gpu() else { return };
        let (seed, scale, walkers) = (5, 1, 1_000);
        let f = Frame { slime: 1.0, ..Frame::default() };
        let run = |cap: Option<usize>, stop: bool| {
            let mut s = garden_settings(seed, scale, walkers);
            let mut cpu = garden_world(seed, scale, walkers, None);
            if let Some(c) = cap {
                s.species_cap[0] = c;
                cpu.dish.p.species_cap[0] = c;
            }
            let mut gpu = GpuWorld::garden(&device, &queue, s, seed, scale, walkers).unwrap();
            if stop {
                gpu.stop_births();
                cpu.stop_births();
            }
            let mut most = 0;
            for _ in 0..20 {
                let _ = gpu.frame(&f, 15);
                let _ = cpu.frame(&f, 15);
                // Exact on llvmpipe; on a card (`tol`) each side and the whole within
                // 2% (+2; 1% until the 11-21 batch's (1001, 1074) against
                // (988, 1087)): on the
                // GTX 1070 (11-14, 004b7cb) the sides came out (988, 1068)
                // against the CPU's (992, 1064), the same 2,056 walkers with
                // four on the other side, where llvmpipe matched exactly; the
                // births race on the card's atomics in another order.
                let (g, c) = (gpu.counts_by_side(), cpu.counts_by_side());
                let near = |a: usize, b: usize| a.abs_diff(b) <= tol(0, b / 50 + 2);
                assert!(near(g.0, c.0) && near(g.1, c.1) && near(g.0 + g.1, c.0 + c.1), "the engines part: {g:?} on the GPU, {c:?} on the CPU");
                most = most.max(gpu.counts_by_side().1);
            }
            (gpu.counts_by_side(), most)
        };
        let ((own_free, _), most_free) = run(None, false);
        assert!(most_free > walkers + 50, "the uncapped enemy grew only to {most_free}");
        let ((own, _), most) = run(Some(walkers), false);
        assert!(most <= walkers + walkers / 10, "capped at {walkers}, the enemy reached {most}");
        // Seed 5 at 20 frames: free 1,116; capped 1,027; stopped 1,027 (the
        // last few dozen came over by conversion, not birth: the player's
        // mould fell from 1,000 to 980 in every run, as it isn't sown here).
        assert!(most_free > most + 50, "the cap held nothing back ({most_free} free, {most} capped)");
        // 15, was 5: with organic starts (2026-11-15, `start_shape`) the
        // two moulds meet along a different edge, and conversion there
        // differs between the capped and free runs by 13 of 1,000 on seed 5
        // (994 against 1,007); the cap itself takes nothing of the player's.
        assert!(own + 15 >= own_free, "the player's mould lost by the enemy's cap ({own} against {own_free})");
        let ((own_stop, _), most_stop) = run(None, true);
        assert!(most_stop <= walkers + walkers / 10, "with its births stopped the enemy reached {most_stop}");
        // The cap's overshoot is bounded by one step's births (the count
        // runs before them): what the capped enemy gained over the
        // births-stopped one (conversion alike in both) is a step's parents
        // at most; at seed 5 it is 0.
        assert!(most <= most_stop + walkers / 20, "the capped enemy passed its cap by {} over the stopped one", most - most_stop.min(most));
        // 15 as above (organic starts: 994 against 1,007 at seed 5).
        assert!(own_stop + 15 >= own_free, "stopping the enemy's births stopped the player's ({own_stop})");
    }

    /// **The garden on a designed map, both engines** (mission 8, the tech
    /// lead's review, 11-14): on the Passes the GPU's dish starts as the
    /// CPU's, walker for walker (positions and sides), and the two count
    /// the same walkers a side each of the first frames. Planted: one
    /// walker's side swapped on the CPU makes the count part at once, so
    /// the check would see a swapped start.
    #[test]
    fn the_gpu_garden_on_a_map_starts_and_runs_as_the_cpu() {
        use crate::mould::own_mould::{garden_on_map, OWN};
        let Some((device, queue)) = test_gpu() else { return };
        let dir = std::path::Path::new(env!("CARGO_MANIFEST_DIR")).join("Reports/larder-maps-2026-11-11/maps");
        let m = Map::load(&dir, "passes").unwrap();
        let (seed, scale, walkers) = (3, 1, 1_000);
        let mut gpu = GpuWorld::garden_on_map(&device, &queue, race_settings(seed, scale, walkers), scale, &m, walkers).unwrap();
        let mut cpu = garden_on_map(race_params(seed, scale, walkers), &m, walkers).unwrap();
        let (g, c) = (&gpu.e.dish, &cpu.dish);
        assert_eq!(g.walkers.len(), c.walkers.len());
        assert_eq!(g.spec, c.spec, "each walker's side");
        assert!(g.walkers.iter().zip(&c.walkers).all(|(a, b)| a == b), "each walker's place");
        assert_eq!(g.landing, c.landing);
        assert_eq!(gpu.counts_by_side(), (walkers, walkers));
        assert_eq!(gpu.counts_by_side(), cpu.counts_by_side());
        let f = Frame { slime: 1.0, ..Frame::default() };
        for i in 0..5 {
            let _ = gpu.frame(&f, 15);
            let _ = cpu.frame(&f, 15);
            assert_eq!(gpu.counts_by_side(), cpu.counts_by_side(), "frame {i}: the engines part");
        }
        // Planted: a swapped walker on one engine is seen.
        let k = (0..cpu.dish.walkers.len()).find(|&k| cpu.dish.lives(k) && cpu.dish.spec[k] == OWN).unwrap();
        cpu.dish.spec[k] = 0;
        assert_ne!(gpu.counts_by_side(), cpu.counts_by_side(), "a swapped side went unseen");
    }

    /// A grazing world is refused, with a reason, not run wrong.
    #[test]
    fn a_garden_refuses_a_grazing_world() {
        let Some((device, queue)) = test_gpu() else { return };
        let s = Settings { graze: 1.0, ..garden_settings(5, 1, 500) };
        let e = GpuWorld::garden(&device, &queue, s, 5, 1, 500).err().expect("refused");
        assert!(e.contains("graze"), "{e}");
    }

    /// Shots fly over the player's mould and strike the enemy's alone, on
    /// both engines: a shot from the larder across the player's mould hits
    /// nothing; one into the enemy's home kills only the enemy's walkers.
    #[test]
    fn gpu_garden_shots_spare_the_players_mould() {
        use crate::mould::own_mould::garden_world;
        use crate::mould::squad::ShotIn;
        let Some((device, queue)) = test_gpu() else { return };
        let (seed, scale, walkers) = (5, 1, 1_000);
        let mut gpu = GpuWorld::garden(&device, &queue, garden_settings(seed, scale, walkers), seed, scale, walkers).unwrap();
        let mut cpu = garden_world(seed, scale, walkers, None);
        let l = cpu.landing().unwrap();
        let home = cpu.crystals()[0].0;
        // A few ticks so both moulds lay trail.
        let f0 = Frame { slime: 1.0, ..Frame::default() };
        let _ = gpu.frame(&f0, 15);
        let _ = cpu.frame(&f0, 15);
        assert_eq!(gpu.counts_by_side().0, cpu.counts_by_side().0);
        let over = Frame { shots: vec![ShotIn { from: l, to: (l.0 + 12.0, l.1), r: 3.0, over: false }], slime: 1.0, ..Frame::default() };
        // The line does cross the player's slime (else this proves nothing).
        let (w, h) = cpu.size();
        let crossed = squad::path(w, h, l, (l.0 + 12.0, l.1)).iter().filter(|&&i| cpu.dish.species_trail[crate::mould::own_mould::OWN as usize][i as usize] >= 1.0).count();
        assert!(crossed > 0, "the shot's line crosses no slime of the player's");
        let (g, c) = (gpu.frame(&over, 0), cpu.frame(&over, 0));
        assert_eq!(g.shots[0].hit, None, "the GPU's shot stopped on the player's mould");
        assert_eq!(c.shots[0].hit, None, "the CPU's shot stopped on the player's mould");
        let own = gpu.counts_by_side().0;
        // From just outside the home's middle: since starts are organic
        // (2026-11-15, `start_shape`) a shot from 30 cells out can stop on
        // the start's thin fraying rim, where trail lies but no walker is
        // within its reach (0 killed on both engines, seed 5).
        let into = Frame { shots: vec![ShotIn { from: (home.0 - 4.0, home.1), to: home, r: 3.0, over: false }], slime: 1.0, ..Frame::default() };
        let (g, c) = (gpu.frame(&into, 0), cpu.frame(&into, 0));
        assert!(g.shots[0].killed > 0 && c.shots[0].killed > 0, "kills: GPU {}, CPU {}", g.shots[0].killed, c.shots[0].killed);
        assert_eq!(g.shots[0].hit, c.shots[0].hit, "the same hit cell");
        assert_eq!(gpu.counts_by_side().0, own, "the player's mould untouched");
    }

    /// Road lays the player's trail and Feed spreads charge from the bank
    /// where the soldier stands, the same on both engines.
    #[test]
    fn gpu_garden_road_and_feed_as_the_cpu() {
        use crate::mould::own_mould::{garden_world, Task, OWN};
        let Some((device, queue)) = test_gpu() else { return };
        let (seed, scale, walkers) = (5, 1, 1_000);
        let mut gpu = Game::new(GpuWorld::garden(&device, &queue, garden_settings(seed, scale, walkers), seed, scale, walkers).unwrap(), Rules::default(), seed);
        let mut cpu = Game::new(garden_world(seed, scale, walkers, None), Rules::default(), seed);
        // Soldier 0 lays a road, soldier 1 feeds, standing still.
        gpu.world.garden.as_mut().unwrap().set_task(0, Task::Road);
        gpu.world.garden.as_mut().unwrap().set_task(1, Task::Feed);
        cpu.world.set_task(0, Task::Road);
        cpu.world.set_task(1, Task::Feed);
        gpu.bank = 1_000.0;
        cpu.bank = 1_000.0;
        let c0g: f64 = gpu.world.e.charge_of().iter().map(|&c| c as f64).sum();
        let c0c: f64 = cpu.world.dish.charge.iter().map(|&c| c as f64).sum();
        gpu.step();
        cpu.step();
        let fed = gpu.world.garden.as_ref().unwrap().fed;
        assert!((fed - cpu.world.fed).abs() < 1e-3 && fed > 0.0, "fed: GPU {fed}, CPU {}", cpu.world.fed);
        assert_eq!(gpu.world.garden.as_ref().unwrap().roads, cpu.world.roads);
        let s0 = gpu.soldiers[0];
        let (w, h) = gpu.world.size();
        let (cx, cy) = squad::cell_of(w, h, s0.x, s0.y);
        let i = cy as usize * w + cx as usize;
        let own = gpu.world.e.species_trail_of(OWN as usize);
        assert!(own[i] > 1.0, "the road's trail: {}", own[i]);
        // The charge laid less what was eaten: both gained about the feed.
        let c1g: f64 = gpu.world.e.charge_of().iter().map(|&c| c as f64).sum();
        let c1c: f64 = cpu.world.dish.charge.iter().map(|&c| c as f64).sum();
        let (dg, dc) = (c1g - c0g, c1c - c0c);
        assert!((dg - dc).abs() < 0.02 * dc.abs().max(100.0), "charge change: GPU {dg:.1}, CPU {dc:.1}");
    }

    /// **A wall holds until it is worn through** (the owner, 2026-11-21:
    /// "Sometimes slime is able to get through my walls even when they
    /// have no burrowed a full hole yet"). A walker's step only tested the
    /// cell it lands in, so a step longer than the wall is thick (a surge,
    /// a raid, a striker or a heartbeat at 2 to 3 times the pace) jumped
    /// it, and a step slanting past a corner slipped between two wall cells
    /// that touch only at it. Now every cell the step crosses must be open.
    ///
    /// A closed ring of wall 2 cells thick (scale 1, the game's wall
    /// width) round an emptied middle, unbreakable, in slime that steps 3
    /// cells: no walker gets in. Each count is a game second; with the old
    /// rule (`solid_walls=0`) walkers come in within seconds.
    #[test]
    fn slime_does_not_step_over_or_between_wall_cells() {
        use crate::gpu::floats;
        let Some((device, queue)) = test_gpu() else { return };
        let (seed, scale, walkers) = (5, 1, 6_000);
        let inside_after = |solid: bool| {
            let s = Settings { step: 3.0, respawn: 0.0, solid_walls: solid, ..race_settings(seed, scale, walkers) };
            let g = GpuWorld::new(&device, &queue, s, scale);
            let (w, h) = g.size();
            let mut game = Game::new(g, Rules::default(), seed);
            let (cx, cy, r) = (w as f32 * 0.5, h as f32 * 0.5, 14.0f32);
            // A ring of 24 straight walls whose ends meet: some lie
            // slanted, so their cells touch at corners.
            let n = 24;
            for q in 0..n {
                let (a0, a1) = (q as f32 / n as f32 * std::f32::consts::TAU, (q + 1) as f32 / n as f32 * std::f32::consts::TAU);
                game.world.build_wall((cx + r * a0.cos(), cy + r * a0.sin()), (cx + r * a1.cos(), cy + r * a1.sin()), 2.0);
            }
            // Empty the middle.
            let GpuWorld { e, shared, s, .. } = &mut game.world;
            e.strike(shared, s, cx, cy, r - 1.5, 0.0, 0.0);
            let count = |g: &GpuWorld| {
                let e = &g.e;
                let wk = floats(&read_back(&e.device, &e.queue, &e.walkers));
                let en = floats(&read_back(&e.device, &e.queue, &e.energy));
                (0..e.n as usize).filter(|&q| (!e.grows || en[q] >= 0.0) && (wk[4 * q] - cx).hypot(wk[4 * q + 1] - cy) < r - 1.5).count()
            };
            let mut seen = vec![count(&game.world)];
            for secs in 1..=12 {
                while game.time() < secs as f32 {
                    game.step();
                }
                seen.push(count(&game.world));
            }
            seen
        };
        let (on, off) = (inside_after(true), inside_after(false));
        eprintln!("walkers inside the ring each second: solid {on:?}, old rule {off:?}");
        assert!(on.iter().all(|&c| c == 0), "none gets in: {on:?}");
        assert!(off.iter().any(|&c| c > 0), "the old rule lets them in (the test's control): {off:?}");
    }

    /// **A wall built on packed slime leaves no walker inside it** (the
    /// owner, 2026-11-22: "sometimes walkers get stuck in walls/doors when
    /// building and it becomes a problem for winning"). A wall moves the
    /// walkers it covers to the nearest free cell (`evict`), but with one
    /// walker a cell it looked only 16 cells out and kept the crowd limit,
    /// so in slime packed wider than that a walker found nowhere and stayed
    /// in the wall, where no gun's shot reaches it (`stops`) and, with
    /// `solid_walls`, a step of one cell can't take it out of a wall
    /// thicker than that: the planet's mass never reached 0.
    ///
    /// A disc of slime packed one a cell 40 out, then a wall 4 cells thick
    /// (a thickened wall at scale 1) through its middle: no living walker
    /// is left on a wall cell, none is lost, and a game later they are
    /// still all out.
    #[test]
    fn a_wall_built_on_packed_slime_leaves_no_walker_inside() {
        use crate::gpu::{floats, Start};
        let Some((device, queue)) = test_gpu() else { return };
        let (seed, scale) = (5, 1);
        let s = Settings { start: Start::Disc, disc: 30.0, haze: 0.0, walkers: 4_500, respawn: 0.0, ..race_settings(seed, scale, 4_500) };
        let g = GpuWorld::new(&device, &queue, s, scale);
        let (w, h) = g.size();
        let mut game = Game::new(g, Rules::default(), seed);
        // Living walkers, and those on a cell that is not agar.
        let count = |g: &GpuWorld| {
            let e = &g.e;
            let wk = floats(&read_back(&e.device, &e.queue, &e.walkers));
            let en = floats(&read_back(&e.device, &e.queue, &e.energy));
            let agar: Vec<u32> = read_back(&e.device, &e.queue, &e.agar).chunks_exact(4).map(|b| u32::from_le_bytes(b.try_into().unwrap())).collect();
            let live: Vec<usize> = (0..e.n as usize).filter(|&q| !e.grows || en[q] >= 0.0).collect();
            let inside = live.iter().filter(|&&q| agar[(wk[4 * q + 1] as usize).min(h - 1) * w + (wk[4 * q] as usize).min(w - 1)] == 0).count();
            (live.len(), inside)
        };
        let (cx, cy) = (w as f32 * 0.5, h as f32 * 0.5);
        let (n0, in0) = count(&game.world);
        game.world.build_wall((cx - 36.0, cy), (cx + 36.0, cy), 4.0);
        let (n1, in1) = count(&game.world);
        for _ in 0..20 {
            game.step();
        }
        let (n2, in2) = count(&game.world);
        eprintln!("packed slime: {n0} walkers ({in0} on walls) before the wall, {n1} ({in1} inside) after it, {n2} ({in2} inside) 20 steps on");
        assert_eq!(in0, 0, "none on a wall before");
        assert_eq!(n1, n0, "no walker lost to the wall");
        assert_eq!(in1, 0, "{in1} walkers left inside the wall");
        assert_eq!(in2, 0, "{in2} walkers inside the wall 20 steps on");
    }

    /// **A wall built under spores in flight leaves none over it** (the
    /// owner's planet 5, the Sporer, 2026-11-23: the slime stayed at 26
    /// for the last 2:40 of a cleared planet). A spore steps only onto
    /// agar with no wall between, so one a new wall covered never moved,
    /// never landed and was never shot (shots miss spores), and the mass
    /// held. Now the wall's last round moves spores off it too, and a spore
    /// whose flight ends on a wall dies there.
    ///
    /// The race world with spores every 100 ticks a walker and flights of
    /// 40 ticks; a thick wall across the middle when the air is full of
    /// them. Right after it and every step for 4 s on: no living spore on a
    /// wall cell.
    #[test]
    fn a_wall_built_under_flying_spores_leaves_none_over_it() {
        use crate::gpu::{floats, set};
        let Some((device, queue)) = test_gpu() else { return };
        let (seed, scale) = (5, 1);
        let mut s = Settings { respawn: 0.0, ..race_settings(seed, scale, 6_000) };
        for (k, v) in [("spore_rate", "0.01"), ("spore_scent", "-1"), ("spore_steps", "40"), ("spore_reach", "8"), ("spore_land", "1"), ("spore_rest", "10")] {
            set(&mut s, k, v).unwrap();
        }
        let g = GpuWorld::new(&device, &queue, s, scale);
        let (w, h) = g.size();
        let mut game = Game::new(g, Rules::default(), seed);
        // Living spores, and those over a cell that is not agar.
        let spores = |g: &GpuWorld| {
            let e = &g.e;
            let u32s = |b: &wgpu::Buffer| -> Vec<u32> { read_back(&e.device, &e.queue, b).chunks_exact(4).map(|c| u32::from_le_bytes(c.try_into().unwrap())).collect() };
            let wk = floats(&read_back(&e.device, &e.queue, &e.walkers));
            let en = floats(&read_back(&e.device, &e.queue, &e.energy));
            let (spec, agar) = (u32s(&e.spec), u32s(&e.agar));
            let flying: Vec<usize> = (0..e.n as usize).filter(|&q| (!e.grows || en[q] >= 0.0) && (spec[q] >> 9) & 3 == 1).collect();
            let over = flying.iter().filter(|&&q| agar[(wk[4 * q + 1] as usize).min(h - 1) * w + (wk[4 * q] as usize).min(w - 1)] == 0).count();
            (flying.len(), over)
        };
        for _ in 0..10 {
            game.step();
        }
        let (n0, _) = spores(&game.world);
        // Through the home blob, where they fly from.
        let p = game.world.e.dish.homes[0];
        let (cx, cy) = (p.0 * scale as f32, p.1 * scale as f32);
        for dy in [-3.0, 0.0, 3.0] {
            game.world.build_wall((cx - 40.0, cy + dy), (cx + 40.0, cy + dy), 4.0);
        }
        let (n1, over1) = spores(&game.world);
        let mut worst = 0;
        while game.time() < 4.0 {
            game.step();
            worst = worst.max(spores(&game.world).1);
        }
        eprintln!("spores in flight {n0} before the wall, {n1} after ({over1} over it); at most {worst} over a wall in the 4 s after");
        assert!(n0 > 100, "spores in the air: {n0}");
        assert_eq!(over1, 0, "{over1} spores left over the wall");
        assert_eq!(worst, 0, "{worst} spores over a wall later");
    }
}
