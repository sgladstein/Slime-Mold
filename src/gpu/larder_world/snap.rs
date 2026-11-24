//! **Save and resume A** (Milestone 2, Direction 2026-11-24): the GPU
//! world written out mid-planet and read back into a fresh world built for
//! the same planet, so a quit keeps the planet as it was, not only the run
//! (step C relanded it fresh).
//!
//! The engine's buffers are read back whole and written again with
//! `write_buffer`; its CPU-side fields go through serde one by one, in a
//! fixed order (`snap_fields!`). The macro destructures the struct with no
//! `..`, so a field added to `Engine` or `GpuWorld` does not compile until
//! it is listed as a buffer, a handle (rebuilt with the world, not saved)
//! or a value.
//!
//! A save is only read by the build that wrote it (`Game::save`'s header
//! carries the build's commit): bincode has no field names, so any change
//! to a saved struct reads as garbage, and a save from another build falls
//! back to step C's reland.

use super::super::{read_back, Engine};
use super::GpuWorld;
use pixels::wgpu;
use serde::{de::DeserializeOwned, Serialize};

/// One value, appended.
pub fn put<T: Serialize + ?Sized>(out: &mut Vec<u8>, v: &T) -> Result<(), String> {
    bincode::serialize_into(out, v).map_err(|e| format!("save: {e}"))
}

/// The next value, read off the front of `r`.
pub fn get<T: DeserializeOwned>(r: &mut &[u8]) -> Result<T, String> {
    bincode::deserialize_from(r).map_err(|e| format!("load: {e}"))
}

/// A buffer's bytes, or None for one the GPU can't copy out (a uniform
/// with no COPY_SRC, refilled every frame anyway).
fn buffer_bytes(device: &wgpu::Device, queue: &wgpu::Queue, b: &wgpu::Buffer) -> Option<Vec<u8>> {
    b.usage().contains(wgpu::BufferUsages::COPY_SRC).then(|| read_back(device, queue, b))
}

fn write_back(queue: &wgpu::Queue, b: &wgpu::Buffer, bytes: Option<Vec<u8>>, name: &str) -> Result<(), String> {
    let Some(bytes) = bytes else { return Ok(()) };
    if bytes.len() as u64 != b.size() {
        return Err(format!("load: buffer {name} is {} bytes, the save has {}", b.size(), bytes.len()));
    }
    if !b.usage().contains(wgpu::BufferUsages::COPY_DST) {
        return Err(format!("load: buffer {name} can't be written"));
    }
    queue.write_buffer(b, 0, &bytes);
    Ok(())
}

/// `save` and `load` for a struct: `buffers` read back, `pairs` the
/// ping-pong pairs (`[wgpu::Buffer; 2]`), `skip` not saved, `values`
/// through serde. Every field must be named once.
macro_rules! snap_fields {
    ($ty:ident { buffers: [$($b:ident),* $(,)?], pairs: [$($p:ident),* $(,)?], skip: [$($h:ident),* $(,)?], values: [$($v:ident),* $(,)?] $(,)? }) => {
        impl $ty {
            #[allow(clippy::redundant_locals)]
            fn snap_save(&self, device: &wgpu::Device, queue: &wgpu::Queue, out: &mut Vec<u8>) -> Result<(), String> {
                let _ = (device, queue);
                let $ty { $($b,)* $($p,)* $($h: _,)* $($v,)* } = self;
                $( put(out, &buffer_bytes(device, queue, $b))?; )*
                $( for b in $p.iter() { put(out, &buffer_bytes(device, queue, b))?; } )*
                $( put(out, $v)?; )*
                Ok(())
            }
            fn snap_load(&mut self, queue: &wgpu::Queue, r: &mut &[u8]) -> Result<(), String> {
                let _ = queue;
                let $ty { $($b,)* $($p,)* $($h: _,)* $($v,)* } = self;
                $( write_back(queue, $b, get(r)?, stringify!($b))?; )*
                $( for b in $p.iter() { write_back(queue, b, get(r)?, stringify!($p))?; } )*
                $( *$v = get(r)?; )*
                Ok(())
            }
        }
    };
}

snap_fields!(Engine {
    buffers: [
        params, walkers, state, occ, agar, lay, pile, reached, frame, hud, sick, age, grass, spec, energy, share, body, news, crater, ground, charge,
        health, bites, pcell, piles, wstat, wb, sq, kills, hits, look,
    ],
    pairs: [trail, scent, germ, species],
    skip: [device, queue, groups],
    values: [
        grows, energy_flows, gas_at, gas_until, gas_immune, mind_rest, clouds, minds, mind_until, lured, waved, wave_w, heart, strikers, striker_held,
        forted, clouded, cloud_until, cloud_top, cloud_at, withers, reach, specks, lumps_on, plant_no_clear, dense_at, plant_at, crust_at, plants,
        fire_on, setting, plant_every, plant_sub, crater_steps, scorched, garden, ceiling_on, tends, ntend, npc, npiles, walled, ground_world, flows,
        squad, shot_sink, spray, squad_due, squad_one, squad_on, grid, n_hits, converts, boxes, outside, nsp, kinds, exploring, seeded, dish,
        dropped, n, cells, done, crowd_ok, jumps, crowd, per, cycles, alarmed,
    ],
});

snap_fields!(GpuWorld {
    buffers: [],
    pairs: [],
    // `e` is saved on its own (it holds the device). The readout, the
    // cell staging, the wall gather, the burst snaps and the command
    // buffers held back are a frame's machinery: idle between frames
    // (`GpuWorld::save_state` flushes first) and built again as needed.
    // The wall cells are found again from the dish (`walls_new`, set
    // by the load). The wait counters are the window's timing, not the
    // game's.
    skip: [e, shared, ro, setc, walls, walls_new, stamps, profile, later, snaps, waits, wait_ms],
    values: [
        s, sub, crystals, living, ate, ate_mark, covered, cells, garden, own, lumps, broken, want, grid, spread, struck,
        fire_new, burn_dirty,
    ],
});

impl GpuWorld {
    /// Every share of the running step sent, and an error if a frame's
    /// results are still out (a save is taken between frames).
    fn settle(&self) -> Result<(), String> {
        self.flush();
        if self.ro.busy || self.setc.due.get() {
            return Err("save: a frame is still running".into());
        }
        Ok(())
    }

    /// The world's whole state, between frames.
    pub fn save_bytes(&mut self) -> Result<Vec<u8>, String> {
        self.settle()?;
        let mut out = Vec::new();
        let (device, queue) = (self.e.device.clone(), self.e.queue.clone());
        self.e.snap_save(&device, &queue, &mut out)?;
        self.snap_save(&device, &queue, &mut out)?;
        Ok(out)
    }

    /// Reads `save_bytes`' output into this world, which must be built for
    /// the same planet (the buffers' sizes are checked).
    pub fn load_bytes(&mut self, mut r: &[u8]) -> Result<(), String> {
        self.settle()?;
        let queue = self.e.queue.clone();
        self.e.snap_load(&queue, &mut r)?;
        self.snap_load(&queue, &mut r)?;
        // The wall gather is built again from the loaded dish.
        self.walls = None;
        self.walls_new = true;
        queue.submit([]);
        if !r.is_empty() {
            return Err(format!("load: {} bytes left over", r.len()));
        }
        Ok(())
    }
}
