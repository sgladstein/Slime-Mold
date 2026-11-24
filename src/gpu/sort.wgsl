// **The walkers sorted by place** (2026-11-24, Milestone 2 test 5; the
// Lab's prototype, 1070: a step's GPU time 9.46 -> 5.27 ms on planet 8,
// `sense` 3.33 -> 0.84). Every slot's state is moved so the living sit in
// the order of their 8x8-cell tile, and the empty slots after them, so a
// warp's walkers read neighbouring memory in `sense`, `egain` and the
// rest. A stable LSD radix sort on the GPU, four bits a pass, then a
// gather of every per-slot word through the order (`sort.rs`). Stable and
// free of atomics in what it writes, so the same world sorts the same way
// on every run (saves and replays stay exact). Its own bind groups: the
// engine's 48 storage bindings are all taken.

struct SortParams {
    n: u32,
    w: u32,
    h: u32,
    // Tiles a row (`w` / 8 rounded up), and in all.
    tx: u32,
    tiles: u32,
    // The digit's shift in this pass.
    shift: u32,
    // Workgroups of 256 slots.
    blocks: u32,
    // 0: the living first, in their old order (packing only); 1: by tile.
    mode: u32,
    // Growth on: a slot with energy under 0 is empty (`lives`).
    grows: u32,
    // A gather: words a slot, where they start in the source and in the
    // scratch buffer (words).
    words: u32,
    src_off: u32,
    dst_off: u32,
}

@group(0) @binding(0) var<uniform> SP: SortParams;
@group(0) @binding(1) var<storage, read> walkers: array<vec4<f32>>;
@group(0) @binding(2) var<storage, read> energy: array<f32>;
@group(0) @binding(3) var<storage, read> keys_in: array<u32>;
@group(0) @binding(4) var<storage, read> idx_in: array<u32>;
@group(0) @binding(5) var<storage, read_write> keys_out: array<u32>;
@group(0) @binding(6) var<storage, read_write> idx_out: array<u32>;
@group(0) @binding(7) var<storage, read_write> hist: array<u32>;

// Each slot's key: its tile (or 0 when packing), the out-of-place after
// the living and the empty last.
@compute @workgroup_size(256)
fn sort_keys(@builtin(global_invocation_id) gid: vec3<u32>) {
    let i = gid.x;
    if (i >= SP.n) { return; }
    let alive = SP.grows == 0u || energy[i] >= 0.0;
    let p = walkers[i];
    let x = u32(max(p.x, 0.0));
    let y = u32(max(p.y, 0.0));
    let inside = x < SP.w && y < SP.h;
    var key: u32;
    if (SP.mode == 0u) {
        key = select(1u, 0u, alive && inside);
    } else if (!alive) {
        key = SP.tiles + 1u;
    } else if (!inside) {
        key = SP.tiles;
    } else {
        key = (y / 8u) * SP.tx + x / 8u;
    }
    keys_out[i] = key;
    idx_out[i] = i;
}

// A slot's digit one-hot in sixteen 16-bit lanes (a block holds 256, so a
// lane never carries over), and the workgroup's inclusive scan of them.
var<workgroup> lo: array<vec4<u32>, 256>;
var<workgroup> hi: array<vec4<u32>, 256>;

fn one_hot(d: u32) -> array<vec4<u32>, 2> {
    var o = array<vec4<u32>, 2>(vec4<u32>(0u), vec4<u32>(0u));
    if (d < 16u) {
        let lane = d / 2u;
        let v = 1u << (16u * (d % 2u));
        if (lane < 4u) { o[0][lane] = v; } else { o[1][lane - 4u] = v; }
    }
    return o;
}

fn lane_of(a: vec4<u32>, b: vec4<u32>, d: u32) -> u32 {
    let lane = d / 2u;
    var w: u32;
    if (lane < 4u) { w = a[lane]; } else { w = b[lane - 4u]; }
    return (w >> (16u * (d % 2u))) & 0xFFFFu;
}

fn scan_block(t: u32, d: u32) {
    let o = one_hot(d);
    lo[t] = o[0];
    hi[t] = o[1];
    for (var off = 1u; off < 256u; off = off * 2u) {
        workgroupBarrier();
        var a = vec4<u32>(0u);
        var b = vec4<u32>(0u);
        if (t >= off) { a = lo[t - off]; b = hi[t - off]; }
        workgroupBarrier();
        lo[t] = lo[t] + a;
        hi[t] = hi[t] + b;
    }
    workgroupBarrier();
}

fn digit(i: u32) -> u32 {
    if (i >= SP.n) { return 16u; }
    return (keys_in[i] >> SP.shift) & 15u;
}

// Each block's count of each digit, digit-major (`hist[d * blocks + b]`),
// so one scan gives every slot's place.
@compute @workgroup_size(256)
fn sort_count(@builtin(workgroup_id) wid: vec3<u32>, @builtin(local_invocation_id) lid: vec3<u32>) {
    let b = wid.x;
    let t = lid.x;
    scan_block(t, digit(b * 256u + t));
    if (t < 16u) {
        hist[t * SP.blocks + b] = lane_of(lo[255], hi[255], t);
    }
}

// The counts' exclusive prefix, in place, by one workgroup: each thread a
// run of them.
var<workgroup> sums: array<u32, 256>;

@compute @workgroup_size(256)
fn sort_scan(@builtin(local_invocation_id) lid: vec3<u32>) {
    let t = lid.x;
    let len = 16u * SP.blocks;
    let run = (len + 255u) / 256u;
    let a = min(t * run, len);
    let e = min(a + run, len);
    var s = 0u;
    for (var i = a; i < e; i = i + 1u) { s = s + hist[i]; }
    sums[t] = s;
    for (var off = 1u; off < 256u; off = off * 2u) {
        workgroupBarrier();
        var v = 0u;
        if (t >= off) { v = sums[t - off]; }
        workgroupBarrier();
        sums[t] = sums[t] + v;
    }
    workgroupBarrier();
    var at = sums[t] - s;
    for (var i = a; i < e; i = i + 1u) {
        let c = hist[i];
        hist[i] = at;
        at = at + c;
    }
}

// Each slot to its place: its digit's start in its block, plus the slots
// before it in the block with the same digit (stable).
@compute @workgroup_size(256)
fn sort_scatter(@builtin(workgroup_id) wid: vec3<u32>, @builtin(local_invocation_id) lid: vec3<u32>) {
    let b = wid.x;
    let t = lid.x;
    let i = b * 256u + t;
    let d = digit(i);
    scan_block(t, d);
    if (d < 16u) {
        let rank = lane_of(lo[t], hi[t], d) - 1u;
        let to = hist[d * SP.blocks + b] + rank;
        keys_out[to] = keys_in[i];
        idx_out[to] = idx_in[i];
    }
}

// The gather (a bind group of its own): each slot's `words` words from the
// slot the order names, into the scratch buffer, which `sort.rs` copies
// back over the source.
@group(0) @binding(8) var<storage, read> order: array<u32>;
@group(0) @binding(9) var<storage, read> src: array<u32>;
@group(0) @binding(10) var<storage, read_write> dst: array<u32>;

@compute @workgroup_size(256)
fn sort_gather(@builtin(global_invocation_id) gid: vec3<u32>, @builtin(num_workgroups) nw: vec3<u32>) {
    let i = gid.x + gid.y * nw.x * 256u;
    if (i >= SP.n * SP.words) { return; }
    let k = i / SP.words;
    let j = i % SP.words;
    dst[SP.dst_off + i] = src[SP.src_off + order[k] * SP.words + j];
}
