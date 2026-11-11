// The camera's view pass (src/world/view.rs): a world-sized RGBA frame
// (any producer; the GPU engine's `frame`) drawn onto a screen-sized
// buffer through the camera, with the minimap, the game's marks and the
// screen-space overlay (HUD text) on top. Three passes: `base` (a thread a
// screen pixel), `draw_marks` (a workgroup a mark, claiming pixels), and
// `compose` (a thread a pixel: the marks' colours and the overlay).

struct V {
    sw: u32, sh: u32, stride: u32, ww: u32,
    wh: u32, n_marks: u32, mask: u32, overlay: u32,
    cx: f32, cy: f32, zoom: f32, _p: f32,
    // The minimap's rectangle on screen (mw 0: none).
    mx: u32, my: u32, mw: u32, mh: u32,
};

@group(0) @binding(0) var<uniform> P: V;
@group(0) @binding(1) var<storage, read> world: array<u32>;
@group(0) @binding(2) var<storage, read> overlay: array<u32>;
@group(0) @binding(3) var<storage, read> marks: array<u32>;
@group(0) @binding(4) var<storage, read_write> out: array<u32>;
// The visibility hook (fog of war, not decided): a world-sized 0-1 field
// the view darkens by when `mask` is 1. Nothing fills it yet.
@group(0) @binding(5) var<storage, read> vis: array<f32>;
// Which mark each pixel shows (index + 1, 0 none); `compose` clears it.
@group(0) @binding(6) var<storage, read_write> top: array<atomic<u32>>;

fn unpack(c: u32) -> vec3<f32> {
    return vec3<f32>(f32(c & 255u), f32((c >> 8u) & 255u), f32((c >> 16u) & 255u));
}

fn pack(c: vec3<f32>) -> u32 {
    let q = vec3<u32>(clamp(c, vec3<f32>(0.0), vec3<f32>(255.0)) + vec3<f32>(0.5));
    return q.x | (q.y << 8u) | (q.z << 16u) | (255u << 24u);
}

fn cell(x: f32, y: f32) -> vec3<f32> {
    let ix = min(u32(max(x, 0.0)), P.ww - 1u);
    let iy = min(u32(max(y, 0.0)), P.wh - 1u);
    var c = unpack(world[iy * P.ww + ix]);
    if (P.mask == 1u) { c *= 0.3 + 0.7 * clamp(vis[iy * P.ww + ix], 0.0, 1.0); }
    return c;
}

// The mean of k x k samples spread over a box `size` cells wide from (x, y).
fn boxed(x: f32, y: f32, size: f32, k: u32) -> vec3<f32> {
    var sum = vec3<f32>(0.0);
    let d = size / f32(k);
    for (var j = 0u; j < k; j++) {
        for (var i = 0u; i < k; i++) {
            sum += cell(x + (f32(i) + 0.5) * d, y + (f32(j) + 0.5) * d);
        }
    }
    return sum / f32(k * k);
}

fn in_minimap(x: u32, y: u32) -> bool {
    return P.mw > 0u && x + 1u >= P.mx && x <= P.mx + P.mw && y + 1u >= P.my && y <= P.my + P.mh;
}

const BACK: vec3<f32> = vec3<f32>(6.0, 8.0, 10.0);

@compute @workgroup_size(16, 16)
fn base(@builtin(global_invocation_id) gid: vec3<u32>) {
    let x = gid.x;
    let y = gid.y;
    if (x >= P.sw || y >= P.sh) { return; }
    var c = BACK;
    if (in_minimap(x, y)) {
        // A one-pixel frame round it, the world inside, the camera's view
        // as a white outline.
        if (x + 1u == P.mx || x == P.mx + P.mw || y + 1u == P.my || y == P.my + P.mh) {
            c = vec3<f32>(120.0, 126.0, 132.0);
        } else {
            let per = f32(P.ww) / f32(P.mw);
            let u = f32(x - P.mx);
            let v = f32(y - P.my);
            c = boxed(u * per, v * per, per, min(u32(ceil(per)), 4u));
            let s = f32(P.mw) / f32(P.ww);
            let x0 = P.cx * s;
            let y0 = P.cy * s;
            let x1 = (P.cx + f32(P.sw) / P.zoom) * s;
            let y1 = (P.cy + f32(P.sh) / P.zoom) * s;
            let inside = u + 1.0 >= x0 && u <= x1 && v + 1.0 >= y0 && v <= y1;
            let edge = abs(u - x0) < 1.0 || abs(u - x1) < 1.0 || abs(v - y0) < 1.0 || abs(v - y1) < 1.0;
            if (inside && edge) { c = vec3<f32>(235.0, 235.0, 235.0); }
        }
    } else {
        let wx = P.cx + f32(x) / P.zoom;
        let wy = P.cy + f32(y) / P.zoom;
        let fx = P.cx + (f32(x) + 0.5) / P.zoom;
        let fy = P.cy + (f32(y) + 0.5) / P.zoom;
        if (fx >= 0.0 && fy >= 0.0 && fx < f32(P.ww) && fy < f32(P.wh)) {
            if (P.zoom >= 1.0) {
                c = cell(fx, fy);
            } else {
                // Zoomed out: the mean of the cells under the pixel (up to
                // 4 x 4 samples), so thin tubes fade rather than flicker.
                let size = 1.0 / P.zoom;
                c = boxed(wx, wy, size, min(u32(ceil(size)), 4u));
            }
        }
    }
    out[y * P.stride + x] = pack(c);
}

const DISC: u32 = 0u;
const RING: u32 = 1u;
const RECT: u32 = 2u;
const FRAME: u32 = 3u;
const BAR: u32 = 4u;
const GLYPH: u32 = 5u;
const LINE: u32 = 6u;

// Marks claim pixels: each covered pixel keeps the highest mark index that
// covers it (`atomicMax`), so a later mark is drawn over an earlier one
// whatever order the workgroups run in (a bar over its unit's disc, a ring
// over both); `compose` then colours each claimed pixel from its mark.
fn claim(x: i32, y: i32, k: u32, minimap_ok: bool) {
    if (x < 0 || y < 0 || u32(x) >= P.sw || u32(y) >= P.sh) { return; }
    if (!minimap_ok && in_minimap(u32(x), u32(y))) { return; }
    atomicMax(&top[u32(y) * P.stride + u32(x)], k + 1u);
}

fn to_screen(x: f32, y: f32) -> vec2<f32> {
    return vec2<f32>((x - P.cx) * P.zoom, (y - P.cy) * P.zoom);
}

fn anchor(b: u32) -> vec2<f32> {
    return to_screen(bitcast<f32>(marks[b]), bitcast<f32>(marks[b + 1u])) + vec2<f32>(bitcast<f32>(marks[b + 2u]), bitcast<f32>(marks[b + 3u]));
}

// A mark's half-extents in pixels.
fn half(b: u32) -> vec2<f32> {
    var h = vec2<f32>(bitcast<f32>(marks[b + 4u]), bitcast<f32>(marks[b + 5u]));
    if ((marks[b + 10u] & 1u) != 0u) { h = max(h * P.zoom, vec2<f32>(2.0)); }
    return h;
}

@compute @workgroup_size(64)
fn draw_marks(@builtin(workgroup_id) wg: vec3<u32>, @builtin(local_invocation_index) li: u32) {
    let k = wg.x;
    if (k >= P.n_marks) { return; }
    let b = k * 12u;
    let s = anchor(b);
    let shape = marks[b + 6u];
    let col2 = marks[b + 8u];
    let fill = bitcast<f32>(marks[b + 9u]);
    let flags = marks[b + 10u];
    let extra = marks[b + 11u];
    // A dot on the minimap: four threads, a 2 x 2 dot.
    if ((flags & 2u) != 0u && P.mw > 0u && li < 4u) {
        let ms = f32(P.mw) / f32(P.ww);
        let mx = i32(f32(P.mx) + bitcast<f32>(marks[b]) * ms) + i32(li & 1u);
        let my = i32(f32(P.my) + bitcast<f32>(marks[b + 1u]) * ms) + i32(li >> 1u);
        if (mx < i32(P.mx + P.mw) && my < i32(P.my + P.mh)) { claim(mx, my, k, true); }
    }
    if (shape == LINE) {
        let e = to_screen(bitcast<f32>(marks[b + 4u]), bitcast<f32>(marks[b + 5u]));
        let len = distance(s, e);
        let n = u32(ceil(len));
        let dash = u32(max(fill, 0.0));
        for (var i = li; i <= n; i += 64u) {
            if (dash > 0u && (i / dash) % 2u == 1u) { continue; }
            let p = mix(s, e, f32(i) / f32(max(n, 1u)));
            claim(i32(floor(p.x)), i32(floor(p.y)), k, false);
        }
        return;
    }
    let h = half(b);
    var x0 = floor(s.x - h.x);
    var y0 = floor(s.y - h.y);
    var x1 = ceil(s.x + h.x);
    var y1 = ceil(s.y + h.y);
    if (shape == GLYPH) {
        x0 = floor(s.x);
        y0 = floor(s.y);
        x1 = x0 + 5.0 * h.x - 1.0;
        y1 = y0 + 7.0 * h.x - 1.0;
    }
    // Clipped to the screen first, so a big mark zoomed in costs no more
    // than the screen.
    x0 = max(x0, 0.0);
    y0 = max(y0, 0.0);
    x1 = min(x1, f32(P.sw) - 1.0);
    y1 = min(y1, f32(P.sh) - 1.0);
    if (x1 < x0 || y1 < y0) { return; }
    let w = u32(x1 - x0) + 1u;
    let total = w * (u32(y1 - y0) + 1u);
    for (var j = li; j < total; j += 64u) {
        let qx = x0 + f32(j % w);
        let qy = y0 + f32(j / w);
        let d = vec2<f32>(qx + 0.5, qy + 0.5) - s;
        var on = false;
        if (shape == DISC) {
            on = length(d) <= h.x;
        } else if (shape == RING) {
            let r = length(d);
            on = r <= h.x && r >= h.x - 1.5;
        } else if (shape == RECT || shape == BAR) {
            on = abs(d.x) <= h.x && abs(d.y) <= h.y;
        } else if (shape == FRAME) {
            on = abs(d.x) <= h.x + 0.5 && abs(d.y) <= h.y + 0.5 && (abs(d.x) >= h.x - 0.5 || abs(d.y) >= h.y - 0.5);
        } else if (shape == GLYPH) {
            let gx = u32(floor((qx - floor(s.x)) / h.x));
            let gy = u32(floor((qy - floor(s.y)) / h.x));
            if (gx < 5u && gy < 7u) {
                let bit = gy * 5u + gx;
                if (bit < 32u) { on = ((col2 >> bit) & 1u) == 1u; } else { on = ((extra >> (bit - 32u)) & 1u) == 1u; }
            }
        }
        if (on) { claim(i32(qx), i32(qy), k, false); }
    }
}

// Marks, then the overlay (premultiplied RGBA, screen-sized), over the
// world; each pixel's mark claim is cleared for the next frame.
@compute @workgroup_size(16, 16)
fn compose(@builtin(global_invocation_id) gid: vec3<u32>) {
    let x = gid.x;
    let y = gid.y;
    if (x >= P.sw || y >= P.sh) { return; }
    let i = y * P.stride + x;
    var c = unpack(out[i]);
    let t = atomicExchange(&top[i], 0u);
    if (t > 0u) {
        let b = (t - 1u) * 12u;
        var col = marks[b + 7u];
        if (in_minimap(x, y)) {
            col = col | (255u << 24u);
        } else if (marks[b + 6u] == BAR) {
            // Left of the fill in the colour, right of it in the second.
            let h = half(b);
            let d = f32(x) + 0.5 - anchor(b).x;
            if (d + h.x > bitcast<f32>(marks[b + 9u]) * 2.0 * h.x) { col = marks[b + 8u]; }
        }
        let a = f32(col >> 24u) / 255.0;
        c = c * (1.0 - a) + unpack(col) * a;
    }
    if (P.overlay == 1u) {
        let o = overlay[y * P.sw + x];
        let oa = f32(o >> 24u) / 255.0;
        c = unpack(o) + c * (1.0 - oa);
    }
    out[i] = pack(c);
}
