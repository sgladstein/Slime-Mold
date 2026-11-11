// The UI canvas (src/world/canvas.rs): screen-space shapes drawn over the
// view's finished frame, in order, with real blending, so text lies on a
// panel and a panel on the world. The CPU bins each shape into the 16 x 16
// tiles it touches; a workgroup is a tile and walks only its own list, so
// a screen with no UI over a tile costs that tile one read.
//
// Why not marks: a mark claims a pixel outright (the highest index wins),
// which is right for units over the world but cannot put anti-aliased text
// over a translucent panel; the edge pixels would blend with the world.

struct P {
    sw: u32, sh: u32, stride: u32, tiles_x: u32,
    tiles_y: u32, n: u32, aw: u32, _p: u32,
};

@group(0) @binding(0) var<uniform> U: P;
// 16 words a shape (see canvas.rs `Prim`).
@group(0) @binding(1) var<storage, read> prims: array<u32>;
// Tile t's shapes are items[offsets[t] .. offsets[t + 1]], in draw order.
@group(0) @binding(2) var<storage, read> offsets: array<u32>;
@group(0) @binding(3) var<storage, read> items: array<u32>;
// The atlas: coverage 0-255, four texels a word, `aw` texels a row.
@group(0) @binding(4) var<storage, read> atlas: array<u32>;
@group(0) @binding(5) var<storage, read_write> out: array<u32>;

const RRECT: u32 = 0u;
const SHADOW: u32 = 1u;
const MASK: u32 = 2u;
const BAR: u32 = 3u;
const RING: u32 = 4u;

fn unpack(c: u32) -> vec4<f32> {
    return vec4<f32>(f32(c & 255u), f32((c >> 8u) & 255u), f32((c >> 16u) & 255u), f32(c >> 24u)) / 255.0;
}

fn pack(c: vec3<f32>) -> u32 {
    let q = vec3<u32>(clamp(c, vec3<f32>(0.0), vec3<f32>(1.0)) * 255.0 + vec3<f32>(0.5));
    return q.x | (q.y << 8u) | (q.z << 16u) | (255u << 24u);
}

fn f(b: u32, k: u32) -> f32 {
    return bitcast<f32>(prims[b + k]);
}

// Signed distance to a rounded box centred at c, half-size h, radius r.
fn sd_rrect(p: vec2<f32>, c: vec2<f32>, h: vec2<f32>, r: f32) -> f32 {
    let q = abs(p - c) - h + vec2<f32>(r);
    return length(max(q, vec2<f32>(0.0))) + min(max(q.x, q.y), 0.0) - r;
}

fn texel(x: u32, y: u32) -> f32 {
    let i = y * U.aw + x;
    return f32((atlas[i >> 2u] >> ((i & 3u) * 8u)) & 255u) / 255.0;
}

fn over(dst: vec3<f32>, src: vec3<f32>, a: f32) -> vec3<f32> {
    return src * a + dst * (1.0 - a);
}

@compute @workgroup_size(16, 16)
fn draw(@builtin(global_invocation_id) gid: vec3<u32>, @builtin(workgroup_id) wg: vec3<u32>) {
    let t = wg.y * U.tiles_x + wg.x;
    let a0 = offsets[t];
    let a1 = offsets[t + 1u];
    if (a0 == a1) { return; }
    let x = gid.x;
    let y = gid.y;
    if (x >= U.sw || y >= U.sh) { return; }
    let i = y * U.stride + x;
    var c = unpack(out[i]).rgb;
    let p = vec2<f32>(f32(x) + 0.5, f32(y) + 0.5);
    for (var k = a0; k < a1; k++) {
        let b = items[k] * 16u;
        let kind = prims[b];
        let lo = vec2<f32>(f(b, 1u), f(b, 2u));
        let hi = vec2<f32>(f(b, 3u), f(b, 4u));
        let ctr = (lo + hi) * 0.5;
        let hs = (hi - lo) * 0.5;
        let r = f(b, 5u);
        let col = unpack(prims[b + 6u]);
        let col2 = unpack(prims[b + 7u]);
        if (kind == RRECT || kind == BAR) {
            let d = sd_rrect(p, ctr, hs, r);
            var cover = clamp(0.5 - d, 0.0, 1.0);
            // A hole (words 14-15: its left, top, right, bottom as 16-bit
            // pixels) the panel leaves open: the minimap's frame.
            let h0 = prims[b + 14u];
            let h1 = prims[b + 15u];
            if (h1 != 0u) {
                let hl = vec2<f32>(f32(h0 & 65535u), f32(h0 >> 16u));
                let hh = vec2<f32>(f32(h1 & 65535u), f32(h1 >> 16u));
                if (p.x > hl.x && p.y > hl.y && p.x < hh.x && p.y < hh.y) { cover = 0.0; }
            }
            if (cover <= 0.0) { continue; }
            var fill = col;
            if (kind == RRECT) {
                // A vertical gradient from colour (top) to colour2 (bottom);
                // colour2 alpha 0 means flat.
                if (col2.a > 0.0) {
                    let v = clamp((p.y - lo.y) / max(hi.y - lo.y, 1.0), 0.0, 1.0);
                    fill = mix(col, col2, v);
                }
            } else {
                // A bar: colour left of the fill, colour2 right of it.
                // The fill's end anti-aliased like the box's edges.
                let edge = lo.x + (hi.x - lo.x) * f(b, 13u);
                fill = mix(col2, col, clamp(edge - p.x + 0.5, 0.0, 1.0));
            }
            c = over(c, fill.rgb, fill.a * cover);
            // A border `bw` wide inside the edge, in word 12's colour.
            let bw = f(b, 8u);
            if (bw > 0.0) {
                let bc = unpack(prims[b + 12u]);
                let inner = clamp(0.5 - (-d - bw), 0.0, 1.0);
                c = over(c, bc.rgb, bc.a * cover * inner);
            }
        } else if (kind == SHADOW) {
            // A soft shadow: the rounded box blurred `soft` pixels.
            let soft = max(f(b, 9u), 0.5);
            let d = sd_rrect(p, ctr, hs, r);
            let a = col.a * (1.0 - smoothstep(-soft, soft, d));
            c = over(c, col.rgb, a);
        } else if (kind == RING) {
            // A ring `bw` wide at the box's edge (a progress or cooldown
            // sweep when fill < 1: the part past the fill's angle is skipped).
            let d = sd_rrect(p, ctr, hs, r);
            let bw = f(b, 8u);
            let cover = clamp(0.5 - abs(d + bw * 0.5) + bw * 0.5, 0.0, 1.0);
            let q = p - ctr;
            let ang = fract(atan2(q.x, -q.y) / 6.2831853 + 1.0);
            if (ang > f(b, 13u)) { continue; }
            c = over(c, col.rgb, col.a * cover);
        } else if (kind == MASK) {
            // Atlas coverage at whole pixels (glyphs and icons are placed on
            // the pixel grid), tinted.
            if (p.x < lo.x || p.y < lo.y || p.x >= hi.x || p.y >= hi.y) { continue; }
            let u = prims[b + 10u] + u32(p.x - lo.x);
            let v = prims[b + 11u] + u32(p.y - lo.y);
            let a = texel(u, v) * col.a;
            c = over(c, col.rgb, a);
        }
    }
    out[i] = pack(c);
}
