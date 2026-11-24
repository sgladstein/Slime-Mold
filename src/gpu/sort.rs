//! **The walkers sorted by place on the GPU** (2026-11-24, Milestone 2
//! test 5, Direction after the Lab's prototype): `sort.wgsl`'s stable
//! radix sort of the slots by 8x8-cell tile, the living first, then every
//! per-slot word gathered through the order. Nothing comes back to the CPU
//! (the Lab's CPU sort read the walkers back and took 86 to 91 ms a second
//! on the 1070), and the order is the same on every run of the same world,
//! so saves and replays stay exact.
//!
//! Why by tile and not only the living first (`Order::Pack`, one pass, not
//! four): on the 1070, planet 8, the Lab's CPU forms of each (11-24) took
//! a step's GPU time from 9.36 ms to 7.35 packed (`sense` 3.29 -> 1.93)
//! and to 5.27 by tile (`sense` 0.84): packing alone is half the win.
//!
//! What moves with a slot (every word the engine keeps for a walker from
//! one step to the next): `walkers` (4 words), `energy`, `spec`, `sick`,
//! `age`, and in `news` its food-news word, its gas word, its Mind word,
//! its dense word (lump and hits) and, with the crust, its set word.
//! `intent` and `icell` live within a tick and are not kept. A slot's
//! random draws follow its number (`keyed(.., k)`, and a child's slot is
//! `child_of(k, t)`), so a sorted world plays a different game of the same
//! rules: births land as often on average (a parent's child slot is
//! uniform over the slots whatever the order), but neighbours now share
//! draws of nearby numbers. A new per-slot word in `engine.wgsl` must be
//! added to `Sorter::new`'s list, or it stays with the slot's old number.

use super::Engine;
use pixels::wgpu;

const SHADER: &str = include_str!("sort.wgsl");

/// Uniform rows a sort uses, one a dispatch kind (`min_uniform_buffer_
/// offset_alignment` is at most 256).
const ROW: u64 = 256;

/// How the slots are ordered.
#[derive(Clone, Copy, Debug, PartialEq, Eq, serde::Serialize, serde::Deserialize)]
pub enum Order {
    /// The living first, in their old order.
    Pack,
    /// The living by 8x8 tile, row by row; the old order within a tile.
    Tile,
}

/// A gather: the buffer, words a slot, the slots' first word in it.
struct Gather {
    group: wgpu::BindGroup,
    buf: wgpu::Buffer,
    words: u32,
    at: u64,
    row: u32,
}

/// The sort's pipelines, scratch buffers and bind groups for one engine.
pub struct Sorter {
    keys: wgpu::ComputePipeline,
    count: wgpu::ComputePipeline,
    scan: wgpu::ComputePipeline,
    scatter: wgpu::ComputePipeline,
    gather: wgpu::ComputePipeline,
    /// Held for the bind groups that read it.
    _params: wgpu::Buffer,
    /// Bind groups reading the first key/index pair and writing the
    /// second, and the other way.
    ab: wgpu::BindGroup,
    ba: wgpu::BindGroup,
    gathers: Vec<Gather>,
    scratch: wgpu::Buffer,
    n: u32,
    blocks: u32,
    passes: u32,
    pub order: Order,
}

fn words(v: &[u32]) -> Vec<u8> {
    v.iter().flat_map(|w| w.to_le_bytes()).collect()
}

impl Sorter {
    pub fn new(e: &Engine, order: Order) -> Self {
        let dev = &e.device;
        let module = dev.create_shader_module(wgpu::ShaderModuleDescriptor { label: Some("sort"), source: wgpu::ShaderSource::Wgsl(SHADER.into()) });
        let storage = |binding: u32, read_only: bool| wgpu::BindGroupLayoutEntry {
            binding,
            visibility: wgpu::ShaderStages::COMPUTE,
            ty: wgpu::BindingType::Buffer { ty: wgpu::BufferBindingType::Storage { read_only }, has_dynamic_offset: false, min_binding_size: None },
            count: None,
        };
        let uniform = wgpu::BindGroupLayoutEntry {
            binding: 0,
            visibility: wgpu::ShaderStages::COMPUTE,
            ty: wgpu::BindingType::Buffer { ty: wgpu::BufferBindingType::Uniform, has_dynamic_offset: true, min_binding_size: wgpu::BufferSize::new(48) },
            count: None,
        };
        let radix_bgl = dev.create_bind_group_layout(&wgpu::BindGroupLayoutDescriptor {
            label: Some("sort radix"),
            entries: &[uniform, storage(1, true), storage(2, true), storage(3, true), storage(4, true), storage(5, false), storage(6, false), storage(7, false)],
        });
        let gather_bgl = dev.create_bind_group_layout(&wgpu::BindGroupLayoutDescriptor { label: Some("sort gather"), entries: &[uniform, storage(8, true), storage(9, true), storage(10, false)] });
        let pipe = |bgl: &wgpu::BindGroupLayout, entry: &str| {
            let layout = dev.create_pipeline_layout(&wgpu::PipelineLayoutDescriptor { label: Some("sort"), bind_group_layouts: &[Some(bgl)], immediate_size: 0 });
            dev.create_compute_pipeline(&wgpu::ComputePipelineDescriptor { label: Some(entry), layout: Some(&layout), module: &module, entry_point: Some(entry), compilation_options: Default::default(), cache: None })
        };
        let n = e.n;
        let blocks = n.div_ceil(256);
        let (w, h) = (e.dish.w as u32, e.dish.h as u32);
        let tx = w.div_ceil(8);
        let tiles = tx * h.div_ceil(8);
        // Bits the largest key needs (the empty: tiles + 1), four a pass.
        let top = match order {
            Order::Pack => 1,
            Order::Tile => tiles + 1,
        };
        let passes = (32 - top.leading_zeros()).div_ceil(4).max(1);
        let usage = wgpu::BufferUsages::STORAGE | wgpu::BufferUsages::COPY_DST | wgpu::BufferUsages::COPY_SRC;
        let make = |label: &str, bytes: u64| dev.create_buffer(&wgpu::BufferDescriptor { label: Some(label), size: bytes.max(16), usage, mapped_at_creation: false });
        let slots = 4 * n as u64;
        let (keys_a, idx_a, keys_b, idx_b) = (make("sort keys a", slots), make("sort index a", slots), make("sort keys b", slots), make("sort index b", slots));
        let hist = make("sort counts", 4 * 16 * blocks as u64);
        // The per-slot words (see the module's note). Offsets in words.
        let cells = e.cells as u64;
        let gas_base = n as u64 + 3 * cells;
        let gas_slots = gas_base + super::GAS_SLOTS as u64;
        let mind = gas_slots + n as u64;
        let dense = e.dense_at / 4 + super::DENSE_HEAD as u64;
        let mut list: Vec<(&wgpu::Buffer, u32, u64)> = vec![(&e.walkers, 4, 0), (&e.energy, 1, 0), (&e.spec, 1, 0), (&e.sick, 1, 0), (&e.age, 1, 0), (&e.news, 1, 0), (&e.news, 1, gas_slots), (&e.news, 1, mind), (&e.news, 1, dense)];
        if e.crust_at > 0 {
            list.push((&e.news, 1, e.crust_at / 4 + cells));
        }
        let scratch = make("sort scratch", 16 * n as u64);
        // Rows: the keys, a pass each, a gather each.
        let base = [n, w, h, tx, tiles, 0, blocks, (order == Order::Tile) as u32, e.grows as u32, 0, 0, 0];
        let mut rows: Vec<[u32; 12]> = vec![base];
        for p in 0..passes {
            let mut r = base;
            r[5] = 4 * p;
            rows.push(r);
        }
        let first_gather = rows.len() as u32;
        for &(_, wds, at) in &list {
            let mut r = base;
            (r[9], r[10], r[11]) = (wds, at as u32, 0);
            rows.push(r);
        }
        let mut bytes = vec![0u8; rows.len() * ROW as usize];
        for (i, r) in rows.iter().enumerate() {
            bytes[i * ROW as usize..i * ROW as usize + 48].copy_from_slice(&words(r));
        }
        let params = dev.create_buffer(&wgpu::BufferDescriptor { label: Some("sort params"), size: bytes.len() as u64, usage: wgpu::BufferUsages::UNIFORM | wgpu::BufferUsages::COPY_DST, mapped_at_creation: false });
        e.queue.write_buffer(&params, 0, &bytes);
        let row = wgpu::BindingResource::Buffer(wgpu::BufferBinding { buffer: &params, offset: 0, size: wgpu::BufferSize::new(48) });
        let radix = |ki: &wgpu::Buffer, ii: &wgpu::Buffer, ko: &wgpu::Buffer, io: &wgpu::Buffer| {
            let entries = [
                wgpu::BindGroupEntry { binding: 0, resource: row.clone() },
                wgpu::BindGroupEntry { binding: 1, resource: e.walkers.as_entire_binding() },
                wgpu::BindGroupEntry { binding: 2, resource: e.energy.as_entire_binding() },
                wgpu::BindGroupEntry { binding: 3, resource: ki.as_entire_binding() },
                wgpu::BindGroupEntry { binding: 4, resource: ii.as_entire_binding() },
                wgpu::BindGroupEntry { binding: 5, resource: ko.as_entire_binding() },
                wgpu::BindGroupEntry { binding: 6, resource: io.as_entire_binding() },
                wgpu::BindGroupEntry { binding: 7, resource: hist.as_entire_binding() },
            ];
            dev.create_bind_group(&wgpu::BindGroupDescriptor { label: Some("sort radix"), layout: &radix_bgl, entries: &entries })
        };
        let ab = radix(&keys_a, &idx_a, &keys_b, &idx_b);
        let ba = radix(&keys_b, &idx_b, &keys_a, &idx_a);
        // The keys are written by `ba`'s outputs (a); an even count of
        // passes ends in a, an odd in b.
        let order_buf = if passes % 2 == 0 { &idx_a } else { &idx_b };
        let gathers = list
            .iter()
            .enumerate()
            .map(|(g, &(buf, wds, at))| {
                let entries = [
                    wgpu::BindGroupEntry { binding: 0, resource: row.clone() },
                    wgpu::BindGroupEntry { binding: 8, resource: order_buf.as_entire_binding() },
                    wgpu::BindGroupEntry { binding: 9, resource: buf.as_entire_binding() },
                    wgpu::BindGroupEntry { binding: 10, resource: scratch.as_entire_binding() },
                ];
                let group = dev.create_bind_group(&wgpu::BindGroupDescriptor { label: Some("sort gather"), layout: &gather_bgl, entries: &entries });
                Gather { group, buf: buf.clone(), words: wds, at, row: first_gather + g as u32 }
            })
            .collect();
        Sorter {
            keys: pipe(&radix_bgl, "sort_keys"),
            count: pipe(&radix_bgl, "sort_count"),
            scan: pipe(&radix_bgl, "sort_scan"),
            scatter: pipe(&radix_bgl, "sort_scatter"),
            gather: pipe(&gather_bgl, "sort_gather"),
            _params: params,
            ab,
            ba,
            gathers,
            scratch,
            n,
            blocks,
            passes,
            order,
        }
    }

    /// Records the sort: the keys, the passes, then each gather and its
    /// copy back. Between frames only (a tick's `intent` and `icell` are
    /// not moved).
    pub fn record(&self, enc: &mut wgpu::CommandEncoder) {
        if self.n == 0 {
            return;
        }
        let at = |r: u32| r * ROW as u32;
        let run = |enc: &mut wgpu::CommandEncoder, pipe: &wgpu::ComputePipeline, group: &wgpu::BindGroup, row: u32, x: u32, y: u32| {
            let mut pass = enc.begin_compute_pass(&wgpu::ComputePassDescriptor { label: Some("sort"), timestamp_writes: None });
            pass.set_pipeline(pipe);
            pass.set_bind_group(0, group, &[at(row)]);
            pass.dispatch_workgroups(x, y, 1);
        };
        run(enc, &self.keys, &self.ba, 0, self.blocks, 1);
        for p in 0..self.passes {
            let g = if p % 2 == 0 { &self.ab } else { &self.ba };
            run(enc, &self.count, g, 1 + p, self.blocks, 1);
            run(enc, &self.scan, g, 1 + p, 1, 1);
            run(enc, &self.scatter, g, 1 + p, self.blocks, 1);
        }
        for g in &self.gathers {
            let total = self.n as u64 * g.words as u64;
            let groups = total.div_ceil(256) as u32;
            let (x, y) = (groups.min(65_535), groups.div_ceil(65_535));
            run(enc, &self.gather, &g.group, g.row, x, y);
            enc.copy_buffer_to_buffer(&self.scratch, 0, &g.buf, 4 * g.at, 4 * total);
        }
    }
}
