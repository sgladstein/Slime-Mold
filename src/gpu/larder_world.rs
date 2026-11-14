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

use std::cell::{Cell, RefCell};
use std::collections::VecDeque;
use std::sync::atomic::{AtomicBool, Ordering};
use std::sync::Arc;
use std::time::Instant;

use super::{read_back, Curve, Engine, Hand, Settings, Shared, Stamps, Start, MAX_PILES, WSTAT_BYTES};
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
const RO_SHOTS: u64 = RO_UNITS + (UNITS_MAX * 16) as u64;
const RO_KILLS: u64 = RO_SHOTS + (SHOTS_MAX * 16) as u64;
const RO_PILES: u64 = RO_KILLS + (SHOTS_MAX * 4) as u64;
const RO_WSTAT: u64 = RO_PILES + (MAX_PILES * 32) as u64;
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

/// What a readout holds, unpacked.
struct Read {
    out: FrameOut,
    /// Each pile's charge and cells still crystal.
    sums: Vec<(f32, u32)>,
    broken: u32,
    living: u32,
    /// The player's own mould's living walkers (a garden's; 0 otherwise).
    own: u32,
    /// The mould grid, if the frame summed one.
    grid: Option<Vec<u32>>,
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
    /// The gardeners' state (`GpuWorld::garden`), None in the plain game.
    pub garden: Option<GpuGarden>,
    /// The player's own mould's living walkers at the last readout.
    own: usize,
    /// Wall cells broken as of the mirror's last agar read.
    broken: u32,
    ro: Readout,
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
    /// Walkers bursts have killed so far (`Engine::struck`), so `burst`
    /// returns its own.
    struck: u32,
}

/// The gardeners on the GPU world (`own_mould`'s `Garden`, 2026-11-12):
/// the soldiers' tasks and what they did.
#[derive(Clone, Debug, Default)]
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
    /// **The gardeners' world on the GPU**: `own_mould::garden_world`'s
    /// start (the same map and walkers), with the squad's passes seeing
    /// the enemy alone and the soldiers' tasks as tend entries. `s` is
    /// `garden_settings` with any changes; a grazing world is refused (the
    /// player's walkers a cell are counted into the grazing count).
    pub fn garden(device: &wgpu::Device, queue: &wgpu::Queue, s: Settings, seed: u64, scale: usize, walkers: usize) -> Result<Self, String> {
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
        let s = garden_settings_on(s, side);
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

    pub fn new(device: &wgpu::Device, queue: &wgpu::Queue, s: Settings, scale: usize) -> Self {
        let shared = Shared::new(device);
        let e = Engine::new(device, queue, &shared, &s, scale);
        Self::of(e, shared, s, scale)
    }

    /// On a designed world (`Engine::with_map`, ROADMAP WM1).
    pub fn with_map(device: &wgpu::Device, queue: &wgpu::Queue, s: Settings, scale: usize, m: &Map) -> Self {
        let shared = Shared::new(device);
        let e = Engine::with_map(device, queue, &shared, &s, scale, m);
        Self::of(e, shared, s, scale)
    }

    fn of(e: Engine, shared: Shared, s: Settings, scale: usize) -> Self {
        let size = RO_GRID + 4 * e.grid_max() as u64;
        let buf = e.device.create_buffer(&wgpu::BufferDescriptor { label: Some("readout"), size, usage: wgpu::BufferUsages::MAP_READ | wgpu::BufferUsages::COPY_DST, mapped_at_creation: false });
        let ro = Readout { buf, ready: Arc::new(AtomicBool::new(false)), busy: false, sub: RefCell::new(None) };
        let mut g = GpuWorld { e, shared, s, sub: scale as u32, crystals: Vec::new(), living: 0, garden: None, own: 0, broken: 0, ro, want: Cell::new(0), grid: None, stamps: None, profile: None, spread: 0, later: RefCell::new(VecDeque::new()), waits: 0, wait_ms: 0.0, struck: 0 };
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
        enc.copy_buffer_to_buffer(&e.piles, 0, b, RO_PILES, RO_WSTAT - RO_PILES);
        let (gw, gh) = e.grid_size();
        enc.copy_buffer_to_buffer(&e.wstat, 0, b, RO_WSTAT, (WSTAT_BYTES + 4 * gw * gh) as u64);
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
        *self.ro.sub.borrow_mut() = None;
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
        let piles = &words[at(RO_PILES)..at(RO_WSTAT)];
        let sums = (0..self.e.npiles as usize).map(|p| (f32::from_bits(piles[p * 8 + 4]), piles[p * 8 + 2])).collect();
        let st = &words[at(RO_WSTAT)..];
        let (gw, gh) = self.e.grid_size();
        let grid = (gw * gh > 0).then(|| words[at(RO_GRID)..at(RO_GRID) + gw * gh].to_vec());
        Read { out, sums, broken: st[1], living: st[4], own: st[5], grid }
    }

    /// Keeps a readout's world counts: the crystals, the living walkers,
    /// and the mirror's open ground once more wall cells have broken.
    fn keep(&mut self, r: &mut Read) {
        let k = self.e.dish.p.scale as f32;
        self.crystals = self.e.dish.piles.iter().zip(&r.sums).map(|(&(x, y), &(sum, _))| ((x * k, y * k), sum)).collect();
        self.living = r.living as usize;
        self.own = r.own as usize;
        if let Some(g) = r.grid.take() {
            self.grid = Some((self.e.grid as usize, self.ticks(), g));
        }
        if r.broken != self.broken {
            // Rare (a cell turns to rubble once), so a read-back of its own.
            let agar = read_back(&self.e.device, &self.e.queue, &self.e.agar);
            for (a, b) in self.e.dish.agar.iter_mut().zip(agar.chunks_exact(4)) {
                *a = u32::from_le_bytes(b.try_into().unwrap()) != 0;
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
}

impl World for GpuWorld {
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
        // A step in shares (`spread`) runs whole before the burst, so the
        // burst lands between steps as on the CPU.
        self.flush();
        if (b.alarm > 0.0 || b.gas > 0) && self.s.alarm_keep <= 0.0 {
            self.s.alarm_keep = k.alarm_keep;
        }
        let mut out = squad::BurstOut::default();
        if b.kill || b.alarm > 0.0 || b.crater > 0 {
            let r = if b.kill { b.r } else { 0.0 };
            self.e.burst(&self.shared, &self.s, b.at.0, b.at.1, r, b.alarm, b.alarm_r, b.crater_r, b.crater, 0.0);
            // Only a kill moves the count: a shock or a crater alone (the
            // rounds' trails, a burst a bullet a frame) skips the read-back.
            if b.kill {
                let now = self.e.struck();
                out.killed = now.wrapping_sub(self.struck);
                self.struck = now;
            }
        }
        // An explosion's fling (`Engine::fling`, 11-14), after the kill as
        // on the CPU.
        if let Some((r, rr, inv)) = b.fling_ring(self.e.dish.p.scale as f32) {
            self.s.fling_speed = k.fling_speed;
            let sc = self.e.dish.p.scale as f32;
            out.flung = self.e.fling(&self.shared, &self.s, b.at.0, b.at.1, r / sc, rr / sc, b.fling_steps, inv);
        }
        if b.gas > 0 {
            self.s.gas_death = k.gas_death;
            out.gassed = self.e.gas(&self.shared, &self.s, b.at.0, b.at.1, b.r, b.gas);
            // The cloud after the gas, as `squad::burst` lays it.
            if k.gas_keep > 0.0 {
                (self.s.gas_keep, self.s.gas_sleep, self.s.gas_linger) = (k.gas_keep, k.gas_sleep, k.gas_linger);
                self.e.lay_gas(&self.shared, &self.s, b.at.0, b.at.1, b.r, k.gas_dose);
            }
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
                self.s.sick = k.sick_steps as u32;
                self.s.immune = k.immune_steps as u32;
                self.s.sick_upkeep = k.sick_upkeep;
                self.s.catch_at = k.sick_catch;
                self.s.germ = k.sick_deposit;
                self.s.germ_keep = k.sick_keep;
            }
            self.e.burst(&self.shared, &self.s, b.at.0, b.at.1, 0.0, 0.0, 0.0, 0.0, 0, b.r);
        }
        out
    }

    /// The gassed sleepers near each point (`Engine::calm`; the CPU's
    /// `gassed_near`): a pass and a read-back after the frame, only while
    /// some gas lasts.
    fn calm(&self, at: &[(f32, f32)], r: f32) -> Vec<u32> {
        self.e.calm(&self.shared, at, r)
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
        let d = &self.e.dish;
        x >= 0.0 && y >= 0.0 && (x as usize) < d.w && (y as usize) < d.h && d.agar[y as usize * d.w + x as usize]
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

    fn food_deposit(&self) -> f32 {
        self.e.dish.p.food_deposit
    }

    fn fine_steps(&self) -> u32 {
        self.sub
    }

    /// As of the last whole game second (`GRID_EVERY`): once asked for,
    /// the GPU sums the square into the readout once a game second; a
    /// first ask, or a new square, is read back once. The CPU world's grid
    /// is the tick's own, so a goal read off it lands up to a second later
    /// here.
    fn mould_grid(&self, bin: usize) -> Option<Vec<u32>> {
        self.want.set(bin);
        match &self.grid {
            Some((b, _, g)) if *b == bin => Some(g.clone()),
            _ => Some(self.grid_now(bin)),
        }
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
    use crate::mould::larder_world::{Cmd, Game, Order, Rules};
    use crate::mould::tiers::Tier;

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
        fn has_germ(&self) -> bool {
            self.g.has_germ()
        }
        fn living(&self) -> usize {
            self.g.living()
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
        fn stop_births(&mut self) {
            self.g.stop_births()
        }
        fn upkeep(&self) -> f32 {
            self.g.upkeep()
        }
        fn set_upkeep(&mut self, up: f32) {
            self.g.set_upkeep(up)
        }
        fn food_deposit(&self) -> f32 {
            self.g.food_deposit()
        }
        fn fine_steps(&self) -> u32 {
            self.g.fine_steps()
        }
        fn hash(&self) -> u64 {
            self.g.hash()
        }
        fn mould_grid(&self, bin: usize) -> Option<Vec<u32>> {
            self.g.mould_grid(bin)
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
                // Each side within 1% and the whole within 1%, not exact: on the
                // GTX 1070 (11-14, 004b7cb) the sides came out (988, 1068)
                // against the CPU's (992, 1064), the same 2,056 walkers with
                // four on the other side, where llvmpipe matched exactly; the
                // births race on the card's atomics in another order.
                let (g, c) = (gpu.counts_by_side(), cpu.counts_by_side());
                let near = |a: usize, b: usize| a.abs_diff(b) <= b / 100 + 2;
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
        assert!(own + 5 >= own_free, "the player's mould lost by the enemy's cap ({own} against {own_free})");
        let ((own_stop, _), most_stop) = run(None, true);
        assert!(most_stop <= walkers + walkers / 10, "with its births stopped the enemy reached {most_stop}");
        // The cap's overshoot is bounded by one step's births (the count
        // runs before them): what the capped enemy gained over the
        // births-stopped one (conversion alike in both) is a step's parents
        // at most; at seed 5 it is 0.
        assert!(most <= most_stop + walkers / 20, "the capped enemy passed its cap by {} over the stopped one", most - most_stop.min(most));
        assert!(own_stop + 5 >= own_free, "stopping the enemy's births stopped the player's ({own_stop})");
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
        let over = Frame { shots: vec![ShotIn { from: l, to: (l.0 + 12.0, l.1), r: 3.0 }], slime: 1.0, ..Frame::default() };
        // The line does cross the player's slime (else this proves nothing).
        let (w, h) = cpu.size();
        let crossed = squad::path(w, h, l, (l.0 + 12.0, l.1)).iter().filter(|&&i| cpu.dish.species_trail[crate::mould::own_mould::OWN as usize][i as usize] >= 1.0).count();
        assert!(crossed > 0, "the shot's line crosses no slime of the player's");
        let (g, c) = (gpu.frame(&over, 0), cpu.frame(&over, 0));
        assert_eq!(g.shots[0].hit, None, "the GPU's shot stopped on the player's mould");
        assert_eq!(c.shots[0].hit, None, "the CPU's shot stopped on the player's mould");
        let own = gpu.counts_by_side().0;
        let into = Frame { shots: vec![ShotIn { from: (home.0 - 30.0, home.1), to: home, r: 3.0 }], slime: 1.0, ..Frame::default() };
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
}
