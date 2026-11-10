//! The GPU plan's phase 2 timing test: the big dish's step as wgpu compute
//! passes, timed on whatever GPU the machine has (the owner's GTX 1070 is
//! the target; Reports/gpu-plan-2026-11-10.md, *Phases*).
//!
//! ```text
//! cargo run --release --example gputime
//! cargo run --release --example gputime -- walkers=100000 seconds=20 look=out/
//! ```
//!
//! It runs `BigDish` (`src/mould/big.rs`) under the chosen scaling rule
//! (`nose`: 1024 x 640, one fine step a tick, scent on the fine grid smelled
//! 36 fine cells out) with the rules phase 1 chose for a GPU. One tick is
//! four compute passes:
//!
//! 1. `sense` (a thread a walker): three sensors, turn, aim at a cell, and
//!    claim it with `atomicMin` of a keyed draw if it was empty when the
//!    pass began (crowd 1).
//! 2. `walk` (a thread a walker): the claim's winner moves and counts a
//!    deposit with `atomicAdd`; a loser or a walker facing a wall stays and
//!    draws a new heading.
//! 3. `trail` (a thread a cell): deposits added, 3x3 mean over agar, kept.
//! 4. `scent` (a thread a cell): the piles lay scent, 3x3 mean, kept; the
//!    deposit counts and claims are cleared and the tick counter advances.
//!
//! The random draws are the CPU's (`keyed`, lowbias32 in 32-bit steps), so a
//! GPU dish can be compared with the CPU's. It is not bit for bit the CPU's:
//! the GPU's `sin`, `cos` and sums round differently, and a pile's scent
//! drops to `fed_scent` at the end of the round of ticks in which the tube
//! reached it (the CPU checks every tick; the check reads the trail back
//! between rounds, outside the timing).
//!
//! Prints the adapter, then for each walker count the wall time a tick over
//! rounds of `batch` ticks per submit (the first round is warm-up), the GPU
//! time of each pass when the adapter has timestamp queries, and the
//! engine's measures read back at the end (`BigMeasure`) beside what the
//! CPU dish gives for the same seed. With more walkers than half the agar
//! cells the crowd limit is dropped (crowd 0, Jenson's rule).

use std::future::Future;
use std::sync::Arc;
use std::task::{Context, Poll, Wake, Waker};
use std::time::Instant;

use pixels::wgpu;
use slime_mould::mould::big::{BigDish, BigParams};
use slime_mould::mould::STEPS_PER_SECOND;

const SHADER: &str = r#"
struct Params {
    w: u32, h: u32, n: u32, crowd: u32,
    s_turn: u32, s_blocked: u32, s_claim: u32, _pad: u32,
    sa: f32, ra: f32, so: f32, sso: f32,
    st: f32, deposit: f32, keep: f32, scent_keep: f32,
};
struct State { t: u32 };

@group(0) @binding(0) var<uniform> P: Params;
@group(0) @binding(1) var<storage, read_write> S: State;
@group(0) @binding(2) var<storage, read_write> walkers: array<vec4<f32>>;
@group(0) @binding(3) var<storage, read_write> intent: array<vec4<f32>>;
@group(0) @binding(4) var<storage, read_write> icell: array<u32>;
@group(0) @binding(5) var<storage, read_write> occ: array<atomic<u32>>;
@group(0) @binding(6) var<storage, read_write> claim: array<atomic<u32>>;
@group(0) @binding(7) var<storage, read_write> dep: array<atomic<u32>>;
@group(0) @binding(8) var<storage, read> agar: array<u32>;
@group(0) @binding(9) var<storage, read> lay: array<f32>;
@group(0) @binding(10) var<storage, read_write> trail_in: array<f32>;
@group(0) @binding(11) var<storage, read_write> trail_out: array<f32>;
@group(0) @binding(12) var<storage, read_write> scent_in: array<f32>;
@group(0) @binding(13) var<storage, read_write> scent_out: array<f32>;

const NONE: u32 = 0xFFFFFFFFu;
const TAU: f32 = 6.2831855;

fn lowbias32(v: u32) -> u32 {
    var x = v;
    x ^= x >> 16u; x *= 0x7FEB352Du; x ^= x >> 15u; x *= 0x846CA68Bu;
    return x ^ (x >> 16u);
}
// The CPU's `keyed`, with the seed and use folded into `s` on the CPU.
fn keyed(s: u32, t: u32, walker: u32) -> u32 {
    return lowbias32(walker ^ lowbias32(lowbias32(s) ^ t));
}
fn unit(x: u32) -> f32 { return f32(x >> 8u) / 16777216.0; }

fn cell(x: f32, y: f32) -> u32 {
    if (x < 0.0 || y < 0.0) { return NONE; }
    let cx = u32(x); let cy = u32(y);
    if (cx >= P.w || cy >= P.h) { return NONE; }
    return cy * P.w + cx;
}
fn scent_at(x: f32, y: f32) -> f32 {
    let i = cell(x, y);
    if (i == NONE) { return 0.0; }
    return scent_in[i];
}
fn sense_at(x: f32, y: f32, a: f32) -> f32 {
    let c = cos(a); let s = sin(a);
    let i = cell(x + c * P.so, y + s * P.so);
    if (i == NONE || agar[i] == 0u) { return -1.0e6; }
    return trail_in[i] + scent_at(x + c * P.sso, y + s * P.sso);
}

@compute @workgroup_size(256)
fn sense(@builtin(global_invocation_id) gid: vec3<u32>) {
    let k = gid.x;
    if (k >= P.n) { return; }
    let t = S.t;
    let wk = walkers[k];
    var h = wk.z;
    let f = sense_at(wk.x, wk.y, h);
    let fl = sense_at(wk.x, wk.y, h - P.sa);
    let fr = sense_at(wk.x, wk.y, h + P.sa);
    if (f > fl && f > fr) {
    } else if (f < fl && f < fr) {
        if ((keyed(P.s_turn, t, k) & 1u) == 0u) { h += P.ra; } else { h -= P.ra; }
    } else if (fl < fr) {
        h += P.ra;
    } else if (fr < fl) {
        h -= P.ra;
    }
    let nx = wk.x + cos(h) * P.st;
    let ny = wk.y + sin(h) * P.st;
    let here = cell(wk.x, wk.y);
    let there = cell(nx, ny);
    var aim = NONE;
    if (there != NONE) {
        if (there == here) {
            aim = there;
        } else if (agar[there] != 0u && (P.crowd == 0u || atomicLoad(&occ[there]) < P.crowd)) {
            aim = there;
            if (P.crowd > 0u) { atomicMin(&claim[there], keyed(P.s_claim, t, k)); }
        }
    }
    intent[k] = vec4<f32>(nx, ny, h, 0.0);
    icell[k] = aim;
}

@compute @workgroup_size(256)
fn walk(@builtin(global_invocation_id) gid: vec3<u32>) {
    let k = gid.x;
    if (k >= P.n) { return; }
    let t = S.t;
    let aim = icell[k];
    let wk = walkers[k];
    let here = cell(wk.x, wk.y);
    var wins = false;
    if (aim != NONE) {
        wins = aim == here || P.crowd == 0u || atomicLoad(&claim[aim]) == keyed(P.s_claim, t, k);
    }
    if (wins) {
        if (aim != here) {
            atomicSub(&occ[here], 1u);
            atomicAdd(&occ[aim], 1u);
        }
        walkers[k] = intent[k];
        atomicAdd(&dep[aim], 1u);
    } else {
        walkers[k] = vec4<f32>(wk.x, wk.y, unit(keyed(P.s_blocked, t, k)) * TAU, 0.0);
    }
}

@compute @workgroup_size(256)
fn trail(@builtin(global_invocation_id) gid: vec3<u32>) {
    let i = gid.x;
    if (i >= P.w * P.h) { return; }
    let x = i % P.w; let y = i / P.w;
    if (agar[i] == 0u || x == 0u || y == 0u || x + 1u == P.w || y + 1u == P.h) { trail_out[i] = 0.0; return; }
    var sum = 0.0; var n = 0.0;
    for (var dy = 0u; dy < 3u; dy++) {
        for (var dx = 0u; dx < 3u; dx++) {
            let j = (y + dy - 1u) * P.w + x + dx - 1u;
            if (agar[j] != 0u) {
                sum += trail_in[j] + P.deposit * f32(atomicLoad(&dep[j]));
                n += 1.0;
            }
        }
    }
    trail_out[i] = sum / n * P.keep;
}

@compute @workgroup_size(256)
fn scent(@builtin(global_invocation_id) gid: vec3<u32>) {
    let i = gid.x;
    if (i >= P.w * P.h) { return; }
    atomicStore(&dep[i], 0u);
    atomicStore(&claim[i], NONE);
    if (i == 0u) { S.t = S.t + 1u; }
    let x = i % P.w; let y = i / P.w;
    if (agar[i] == 0u || x == 0u || y == 0u || x + 1u == P.w || y + 1u == P.h) { scent_out[i] = 0.0; return; }
    var sum = 0.0; var n = 0.0;
    for (var dy = 0u; dy < 3u; dy++) {
        for (var dx = 0u; dx < 3u; dx++) {
            let j = (y + dy - 1u) * P.w + x + dx - 1u;
            if (agar[j] != 0u) {
                sum += scent_in[j] + lay[j];
                n += 1.0;
            }
        }
    }
    scent_out[i] = sum / n * P.scent_keep;
}
"#;

/// The CPU's `keyed` stream key for a use (`mould::keyed`'s first line).
fn stream(seed: u64, use_: u32) -> u32 {
    (seed as u32) ^ ((seed >> 32) as u32) ^ use_.wrapping_mul(0x9E37_79B9)
}

// Draw uses, as in big.rs.
const DRAW_TURN: u32 = 101;
const DRAW_BLOCKED: u32 = 102;
const CLAIM_MOVE: u32 = 103;

/// The chosen scaling rule (`examples/bigdish.rs` preset `nose`).
fn nose(walkers: usize, seed: u64) -> BigParams {
    let t = BigParams::today();
    BigParams { scale: 4, walkers, scent_coarse: false, scent_offset: t.sensor_offset * 4.0, blob: false, seed, ..t }
}

fn block_on<F: Future>(f: F) -> F::Output {
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

fn f32_bytes(v: &[f32]) -> Vec<u8> {
    v.iter().flat_map(|x| x.to_le_bytes()).collect()
}

fn u32_bytes(v: &[u32]) -> Vec<u8> {
    v.iter().flat_map(|x| x.to_le_bytes()).collect()
}

struct Gpu {
    device: wgpu::Device,
    queue: wgpu::Queue,
    timestamps: bool,
}

struct Run {
    pipes: [wgpu::ComputePipeline; 4],
    /// Bind groups for even and odd ticks (trail and scent swap).
    groups: [wgpu::BindGroup; 2],
    walkers: wgpu::Buffer,
    trail: [wgpu::Buffer; 2],
    scent: [wgpu::Buffer; 2],
    n: u32,
    cells: u32,
    lay: wgpu::Buffer,
}

impl Run {
    fn new(gpu: &Gpu, d: &BigDish) -> Self {
        let dev = &gpu.device;
        let p = &d.p;
        let cells = (d.w * d.h) as u32;
        let n = d.walkers.len() as u32;
        let storage = wgpu::BufferUsages::STORAGE | wgpu::BufferUsages::COPY_DST | wgpu::BufferUsages::COPY_SRC;
        let buf = |label: &str, bytes: &[u8], usage: wgpu::BufferUsages| {
            let b = dev.create_buffer(&wgpu::BufferDescriptor { label: Some(label), size: bytes.len().max(16) as u64, usage, mapped_at_creation: false });
            gpu.queue.write_buffer(&b, 0, bytes);
            b
        };
        let sso = if p.scent_offset > 0.0 { p.scent_offset } else { p.sensor_offset };
        let crowd = p.crowd as u32;
        let mut params = u32_bytes(&[d.w as u32, d.h as u32, n, crowd, stream(p.seed, DRAW_TURN), stream(p.seed, DRAW_BLOCKED), stream(p.seed, CLAIM_MOVE), 0]);
        params.extend(f32_bytes(&[p.sensor_angle, p.rotate_angle, p.sensor_offset, sso, p.step, p.deposit, p.keep, p.scent_keep]));
        let params = buf("params", &params, wgpu::BufferUsages::UNIFORM | wgpu::BufferUsages::COPY_DST);
        let state = buf("state", &u32_bytes(&[d.steps as u32, 0, 0, 0]), storage);
        let flat: Vec<f32> = d.walkers.iter().flat_map(|&(x, y, h)| [x, y, h, 0.0]).collect();
        let walkers = buf("walkers", &f32_bytes(&flat), storage);
        let intent = buf("intent", &vec![0u8; n as usize * 16], storage);
        let icell = buf("icell", &vec![0u8; n as usize * 4], storage);
        let mut occ = vec![0u32; cells as usize];
        for &(x, y, _) in &d.walkers {
            occ[y as usize * d.w + x as usize] += 1;
        }
        let occ = buf("occ", &u32_bytes(&occ), storage);
        let claim = buf("claim", &u32_bytes(&vec![u32::MAX; cells as usize]), storage);
        let dep = buf("dep", &vec![0u8; cells as usize * 4], storage);
        let agar: Vec<u32> = d.agar.iter().map(|&a| a as u32).collect();
        let agar = buf("agar", &u32_bytes(&agar), storage);
        let lay = buf("lay", &f32_bytes(&d.scent_laid()), storage);
        let trail = [buf("trail0", &f32_bytes(&d.trail), storage), buf("trail1", &f32_bytes(&d.trail), storage)];
        let scent = [buf("scent0", &f32_bytes(&d.scent), storage), buf("scent1", &f32_bytes(&d.scent), storage)];

        let module = dev.create_shader_module(wgpu::ShaderModuleDescriptor { label: Some("big dish"), source: wgpu::ShaderSource::Wgsl(SHADER.into()) });
        // One explicit layout shared by all four entry points, so one bind
        // group serves every pass.
        let entry = |binding: u32, ty: wgpu::BufferBindingType| wgpu::BindGroupLayoutEntry {
            binding,
            visibility: wgpu::ShaderStages::COMPUTE,
            ty: wgpu::BindingType::Buffer { ty, has_dynamic_offset: false, min_binding_size: None },
            count: None,
        };
        let rw = wgpu::BufferBindingType::Storage { read_only: false };
        let ro = wgpu::BufferBindingType::Storage { read_only: true };
        let mut entries = vec![entry(0, wgpu::BufferBindingType::Uniform)];
        for b in 1..=13 {
            entries.push(entry(b, if b == 8 || b == 9 { ro } else { rw }));
        }
        let bgl = dev.create_bind_group_layout(&wgpu::BindGroupLayoutDescriptor { label: Some("big dish"), entries: &entries });
        let layout = dev.create_pipeline_layout(&wgpu::PipelineLayoutDescriptor { label: Some("big dish"), bind_group_layouts: &[Some(&bgl)], immediate_size: 0 });
        let pipe = |entry_point: &str| {
            dev.create_compute_pipeline(&wgpu::ComputePipelineDescriptor {
                label: Some(entry_point),
                layout: Some(&layout),
                module: &module,
                entry_point: Some(entry_point),
                compilation_options: Default::default(),
                cache: None,
            })
        };
        let pipes = [pipe("sense"), pipe("walk"), pipe("trail"), pipe("scent")];
        let group = |even: bool| {
            let (a, b) = if even { (0, 1) } else { (1, 0) };
            let bufs: [&wgpu::Buffer; 14] = [&params, &state, &walkers, &intent, &icell, &occ, &claim, &dep, &agar, &lay, &trail[a], &trail[b], &scent[a], &scent[b]];
            let entries: Vec<wgpu::BindGroupEntry> = bufs.iter().enumerate().map(|(i, b)| wgpu::BindGroupEntry { binding: i as u32, resource: b.as_entire_binding() }).collect();
            dev.create_bind_group(&wgpu::BindGroupDescriptor { label: Some("big dish"), layout: &bgl, entries: &entries })
        };
        let groups = [group(true), group(false)];
        Run { pipes, groups, walkers, trail, scent, n, cells, lay }
    }

    /// Records `ticks` ticks starting at tick `from` (which sets the
    /// ping-pong side), with GPU timestamps around each pass if `stamps`.
    fn record(&self, enc: &mut wgpu::CommandEncoder, from: u64, ticks: u64, stamps: Option<&wgpu::QuerySet>) {
        for t in from..from + ticks {
            let group = &self.groups[(t % 2) as usize];
            let sizes = [self.n, self.n, self.cells, self.cells];
            for (p, pipe) in self.pipes.iter().enumerate() {
                let timestamp_writes = stamps.map(|qs| wgpu::ComputePassTimestampWrites {
                    query_set: qs,
                    beginning_of_pass_write_index: Some(2 * p as u32),
                    end_of_pass_write_index: Some(2 * p as u32 + 1),
                });
                let mut pass = enc.begin_compute_pass(&wgpu::ComputePassDescriptor { label: None, timestamp_writes });
                pass.set_pipeline(pipe);
                pass.set_bind_group(0, group, &[]);
                pass.dispatch_workgroups(sizes[p].div_ceil(256), 1, 1);
            }
        }
    }
}

fn read_back(gpu: &Gpu, src: &wgpu::Buffer) -> Vec<u8> {
    let size = src.size();
    let staging = gpu.device.create_buffer(&wgpu::BufferDescriptor { label: Some("read back"), size, usage: wgpu::BufferUsages::MAP_READ | wgpu::BufferUsages::COPY_DST, mapped_at_creation: false });
    let mut enc = gpu.device.create_command_encoder(&wgpu::CommandEncoderDescriptor { label: None });
    enc.copy_buffer_to_buffer(src, 0, &staging, 0, size);
    gpu.queue.submit([enc.finish()]);
    staging.map_async(wgpu::MapMode::Read, .., |r| r.expect("map"));
    gpu.device.poll(wgpu::PollType::wait_indefinitely()).expect("poll");
    let bytes = staging.get_mapped_range(..).to_vec();
    staging.unmap();
    bytes
}

fn floats(bytes: &[u8]) -> Vec<f32> {
    bytes.chunks_exact(4).map(|c| f32::from_le_bytes([c[0], c[1], c[2], c[3]])).collect()
}

fn main() {
    let mut counts = vec![32_000usize, 100_000, 1_000_000];
    let (mut seconds, mut batch, mut seed) = (10.0f32, 120u64, 1u64);
    let mut look: Option<String> = None;
    let mut cpu = true;
    for arg in std::env::args().skip(1) {
        let (k, v) = arg.split_once('=').unwrap_or_else(|| panic!("expected key=value, got {arg}"));
        match k {
            "walkers" => counts = v.split(',').map(|s| s.parse().unwrap()).collect(),
            "seconds" => seconds = v.parse().unwrap(),
            "batch" => batch = v.parse().unwrap(),
            "seed" => seed = v.parse().unwrap(),
            "look" => look = Some(v.into()),
            "cpu" => cpu = v != "0",
            _ => panic!("unknown key {k}: walkers= seconds= batch= seed= look= cpu=0|1"),
        }
    }
    let instance = wgpu::Instance::new(wgpu::InstanceDescriptor::new_without_display_handle_from_env());
    let adapter = block_on(instance.request_adapter(&wgpu::RequestAdapterOptions { power_preference: wgpu::PowerPreference::HighPerformance, ..Default::default() })).expect("no GPU adapter");
    let info = adapter.get_info();
    println!("adapter: {} ({:?}, {:?}, driver {} {})", info.name, info.backend, info.device_type, info.driver, info.driver_info);
    let timestamps = adapter.features().contains(wgpu::Features::TIMESTAMP_QUERY);
    let required_features = if timestamps { wgpu::Features::TIMESTAMP_QUERY } else { wgpu::Features::empty() };
    let (device, queue) = block_on(adapter.request_device(&wgpu::DeviceDescriptor { label: Some("gputime"), required_features, required_limits: adapter.limits(), ..Default::default() })).expect("no device");
    let gpu = Gpu { device, queue, timestamps };
    println!("timestamp queries: {}", if gpu.timestamps { "yes" } else { "no (wall time only)" });

    for &n in &counts {
        let mut p = nose(n, seed);
        let agar_cells = BigDish::new(BigParams { walkers: 1, ..p.clone() }).agar.iter().filter(|&&a| a).count();
        if n > agar_cells / 2 {
            p.crowd = 0;
        }
        let mut d = BigDish::new(p.clone());
        println!("== {} walkers on {} x {} (crowd {}), seed {seed}", d.walkers.len(), d.w, d.h, p.crowd);
        let run = Run::new(&gpu, &d);
        let ticks = (seconds * STEPS_PER_SECOND as f32).round() as u64;
        let mut per_tick = Vec::new();
        let mut done = 0u64;
        while done < ticks {
            let k = batch.min(ticks - done);
            let mut enc = gpu.device.create_command_encoder(&wgpu::CommandEncoderDescriptor { label: None });
            run.record(&mut enc, done, k, None);
            let t0 = Instant::now();
            gpu.queue.submit([enc.finish()]);
            gpu.device.poll(wgpu::PollType::wait_indefinitely()).expect("poll");
            per_tick.push(t0.elapsed().as_secs_f64() * 1e3 / k as f64);
            done += k;
            // Between rounds, untimed: read the trail back and drop the
            // scent of piles the tube has reached, as the CPU does each tick.
            if d.reached.iter().any(|&r| !r) {
                d.trail = floats(&read_back(&gpu, &run.trail[(done % 2) as usize]));
                let before = d.reached.clone();
                d.update_reached();
                if d.reached != before {
                    gpu.queue.write_buffer(&run.lay, 0, &f32_bytes(&d.scent_laid()));
                }
            }
        }
        let timed = if per_tick.len() > 1 { &per_tick[1..] } else { &per_tick[..] };
        let mut sorted = timed.to_vec();
        sorted.sort_by(|a, b| a.partial_cmp(b).unwrap());
        let median = sorted[sorted.len() / 2];
        println!(
            "wall ms a tick over {} rounds of {batch}: median {median:.3}, best {:.3}, worst {:.3} (first round, warm-up: {:.3}); a 60 Hz frame of two ticks {:.3} ms",
            timed.len(),
            sorted[0],
            sorted[sorted.len() - 1],
            per_tick[0],
            2.0 * median
        );
        if gpu.timestamps {
            let qs = gpu.device.create_query_set(&wgpu::QuerySetDescriptor { label: Some("passes"), ty: wgpu::QueryType::Timestamp, count: 8 });
            let resolve = gpu.device.create_buffer(&wgpu::BufferDescriptor { label: Some("resolve"), size: 64, usage: wgpu::BufferUsages::QUERY_RESOLVE | wgpu::BufferUsages::COPY_SRC, mapped_at_creation: false });
            let mut enc = gpu.device.create_command_encoder(&wgpu::CommandEncoderDescriptor { label: None });
            // One more tick, profiled; it advances the dish, so the
            // measures below are at `seconds` plus one tick.
            run.record(&mut enc, done, 1, Some(&qs));
            done += 1;
            enc.resolve_query_set(&qs, 0..8, &resolve, 0);
            gpu.queue.submit([enc.finish()]);
            let raw = read_back(&gpu, &resolve);
            let stamp: Vec<u64> = raw.chunks_exact(8).map(|c| u64::from_le_bytes(c.try_into().unwrap())).collect();
            let ns = gpu.queue.get_timestamp_period() as f64;
            let names = ["sense", "walk", "trail", "scent"];
            let parts: Vec<String> = (0..4).map(|p| format!("{} {:.3}", names[p], stamp[2 * p + 1].saturating_sub(stamp[2 * p]) as f64 * ns / 1e6)).collect();
            println!("GPU ms a pass, one tick: {}", parts.join(", "));
        }
        // Read the dish back and measure it as the CPU does.
        let side = (done % 2) as usize;
        d.trail = floats(&read_back(&gpu, &run.trail[side]));
        d.scent = floats(&read_back(&gpu, &run.scent[side]));
        d.walkers = floats(&read_back(&gpu, &run.walkers)).chunks_exact(4).map(|c| (c[0], c[1], c[2])).collect();
        d.ticks = done;
        d.update_reached();
        let m = d.measure();
        println!(
            "gpu dish at {:.1}s: tube {:.1}  loops {}  width {:.2}  reach {:.1}  piles {}/{}  strays {:.3}",
            m.seconds, m.tube_area, m.loops, m.width, m.reach, m.reached, d.piles.len(), m.strays
        );
        if let Some(dir) = &look {
            let path = format!("{dir}/gpu-{}-s{seed}-{:03}s.png", n, (m.seconds.round()) as u32);
            d.image(1).save(&path).unwrap();
            println!("wrote {path}");
        }
        if cpu && n <= 100_000 {
            let mut c = BigDish::new(p.clone());
            c.threads = std::thread::available_parallelism().map(|n| n.get()).unwrap_or(1);
            let t0 = Instant::now();
            while c.ticks < done {
                c.tick();
            }
            let cm = c.measure();
            println!(
                "cpu dish at {:.1}s: tube {:.1}  loops {}  width {:.2}  reach {:.1}  piles {}/{}  strays {:.3}  ({:.2} ms a tick, {} threads)",
                cm.seconds,
                cm.tube_area,
                cm.loops,
                cm.width,
                cm.reach,
                cm.reached,
                c.piles.len(),
                cm.strays,
                t0.elapsed().as_secs_f64() * 1e3 / done as f64,
                c.threads
            );
        }
    }
}
