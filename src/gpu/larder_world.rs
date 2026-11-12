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
//! the mirror's agar follows them. A wall worn to rubble on the GPU stays a
//! wall in the mirror; soldiers then walk round rubble they could cross,
//! which only costs them a few steps.

use super::{read_back, Curve, Engine, Hand, Settings, Shared, Start};
use pixels::wgpu;
use crate::mould::big::BigParams;
use crate::mould::larder_world::{race_params, World};
use crate::mould::map::Map;
use crate::mould::squad::{Frame, FrameOut};

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
        energy_flow: p.energy_flow,
        hunger_scent: p.hunger_scent,
        hunger_at: p.hunger_at,
        seed,
        ..Settings::big_dish()
    }
}

/// The engine as a Larder world.
pub struct GpuWorld {
    pub e: Engine,
    pub shared: Shared,
    pub s: Settings,
    /// Engine ticks a game tick (the scale).
    sub: u32,
    /// The crystals as of the last frame (read back once a frame).
    crystals: Vec<((f32, f32), f32)>,
}

impl GpuWorld {
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
        let mut g = GpuWorld { e, shared, s, sub: scale as u32, crystals: Vec::new() };
        g.read_crystals();
        g
    }

    fn read_crystals(&mut self) {
        let k = self.e.dish.p.scale as f32;
        let (sums, ..) = self.e.world_counts();
        self.crystals = self.e.dish.piles.iter().zip(sums).map(|(&(x, y), (sum, _))| ((x * k, y * k), sum)).collect();
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
        let n = ticks * self.sub;
        self.e.set_squad(f);
        self.e.write_params(&self.s, Hand::default(), n);
        let mut enc = self.e.device.create_command_encoder(&wgpu::CommandEncoderDescriptor { label: Some("larder world") });
        self.e.record(&self.shared, &mut enc, n, self.s.curve == Curve::Soft, None);
        self.e.queue.submit([enc.finish()]);
        let out = self.e.squad_out();
        self.read_crystals();
        out
    }

    fn build_wall(&mut self, a: (f32, f32), b: (f32, f32), width: f32) {
        self.e.build_wall(&self.shared, &self.s, a.0, a.1, b.0, b.1, width);
        self.e.dish.build_wall(a.0, a.1, b.0, b.1, width);
    }

    fn living(&self) -> usize {
        self.e.alive()
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
        self.s.birth_energy = f32::INFINITY;
    }

    fn food_deposit(&self) -> f32 {
        self.e.dish.p.food_deposit
    }

    /// The walkers, trail and energy read back, hashed (a replay's check;
    /// it waits for the GPU, so the game calls it only at the end).
    fn hash(&self) -> u64 {
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
}
