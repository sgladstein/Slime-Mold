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
    // The look (`View::set_look`): 1 shades the world from the engine's
    // look and ground fields, with every walker a speck; 0 the frame only.
    shade: u32, n_walk: u32, _l0: u32, _l1: u32,
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
// The look: a u32 a cell from the engine (mould thickness, walkers on it,
// wound, scent; a byte each), the ground a cell from the game (kind, value
// such as a wall's health or a crystal's charge), the walkers themselves
// (x, y, heading, _ in cells), and walkers a screen pixel (`specks`, which
// `base` clears as it reads).
@group(0) @binding(7) var<storage, read> look: array<u32>;
@group(0) @binding(8) var<storage, read> ground: array<u32>;
@group(0) @binding(9) var<storage, read> walkers: array<vec4<f32>>;
@group(0) @binding(10) var<storage, read_write> specks: array<atomic<u32>>;

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


// ---- The look: the world shaded from the simulation's own fields ----
//
// Height is what is there: mould thickness (the trail's brightness, the
// same number the colour ramp uses), rock, a wall's health, a crystal's
// charge, rubble. Light comes from the top left; slopes of that height
// field catch it or turn away, thick mould is glossy, tall things cast a
// short shadow. Nothing is drawn that is not a field: no outlines, no
// textures, no thresholds on graded quantities (owner, 2026-11-09/10).

const AGAR: u32 = 0u;
const WET: u32 = 1u;
const ROCK: u32 = 2u;
const WALL: u32 = 3u;
const RUBBLE: u32 = 4u;
const CRYSTAL: u32 = 5u;
const LIGHT: vec3<f32> = vec3<f32>(-0.42, -0.58, 0.70);

fn cidx(ix: i32, iy: i32) -> u32 {
    let x = u32(clamp(ix, 0, i32(P.ww) - 1));
    let y = u32(clamp(iy, 0, i32(P.wh) - 1));
    return y * P.ww + x;
}

fn byte(v: u32, k: u32) -> f32 { return f32((v >> (8u * k)) & 255u) / 255.0; }

// A cell's height in cells: off the world is rock.
fn cell_height(ix: i32, iy: i32) -> f32 {
    if (ix < 0 || iy < 0 || ix >= i32(P.ww) || iy >= i32(P.wh)) { return 3.0; }
    let i = cidx(ix, iy);
    let g = ground[i];
    let kind = g & 255u;
    let v = byte(g, 1u);
    var h = 4.0 * pow(byte(look[i], 0u), 0.65);
    if (kind == ROCK) { h = 3.0; }
    else if (kind == WALL) { h = 0.6 + 2.6 * v; }
    else if (kind == RUBBLE) { h = 0.2 + 0.6 * v; }
    else if (kind == CRYSTAL) { h = max(h, (0.6 + 9.0 * byte(g, 2u)) * v); }
    return h;
}

fn height(x: f32, y: f32) -> f32 {
    let fx = x - 0.5; let fy = y - 0.5;
    let ix = i32(floor(fx)); let iy = i32(floor(fy));
    let tx = fx - floor(fx); let ty = fy - floor(fy);
    let a = mix(cell_height(ix, iy), cell_height(ix + 1, iy), tx);
    let b = mix(cell_height(ix, iy + 1), cell_height(ix + 1, iy + 1), tx);
    return mix(a, b, ty);
}

struct Mat { albedo: vec3<f32>, glow: vec3<f32>, gloss: f32, wound: f32, gem: f32 };

// A cell's material: the engine's colour for agar (mould, scent, species,
// germ, grass all in it), the ground kinds in their own colours, scaled by
// their value where it is graded.
fn cell_mat(ix: i32, iy: i32) -> Mat {
    var m: Mat;
    if (ix < 0 || iy < 0 || ix >= i32(P.ww) || iy >= i32(P.wh)) {
        m.albedo = vec3<f32>(52.0, 54.0, 60.0); m.glow = vec3<f32>(0.0); m.gloss = 0.05; m.wound = 0.0; m.gem = 0.0;
        return m;
    }
    let i = cidx(ix, iy);
    let g = ground[i];
    let kind = g & 255u;
    let v = byte(g, 1u);
    let lk = look[i];
    var c = unpack(world[i]);
    if (P.mask == 1u) { c *= 0.3 + 0.7 * clamp(vis[i], 0.0, 1.0); }
    // The mould: the engine's colour carries the hue (species, germ); the
    // look carries it at full strength wherever there is mould, by the
    // thickness, so a thin film reads as slime, not as dim ground.
    let th = byte(lk, 0u);
    let tint = max(c - vec3<f32>(12.0, 22.0, 26.0), vec3<f32>(0.0));
    let hue = tint / max(max(tint.x, max(tint.y, tint.z)), 1.0) * 240.0;
    m.albedo = mix(c, hue * (0.55 + 0.45 * th), clamp(1.0 - exp(-5.0 * th), 0.0, 1.0) * 0.85);
    m.glow = vec3<f32>(0.0);
    m.gloss = 0.08 + 0.7 * sqrt(th);
    m.wound = byte(lk, 2u);
    m.gem = 0.0;
    if (kind == WET) {
        m.albedo = c * vec3<f32>(0.70, 0.82, 1.0) + vec3<f32>(0.0, 4.0, 12.0);
        m.gloss = max(m.gloss, 0.5);
    } else if (kind == ROCK) {
        m.albedo = vec3<f32>(58.0, 60.0, 66.0); m.gloss = 0.04;
    } else if (kind == WALL) {
        // Health is the wall's brightness as well as its height.
        m.albedo = mix(vec3<f32>(70.0, 58.0, 48.0), vec3<f32>(168.0, 156.0, 132.0), v); m.gloss = 0.1;
    } else if (kind == RUBBLE) {
        m.albedo = vec3<f32>(92.0, 82.0, 70.0) * (0.7 + 0.3 * v); m.gloss = 0.05;
    } else if (kind == CRYSTAL) {
        // The charge left is the glow, brightest deep inside; mould
        // feeding on it lies over it in its own colour.
        let dp = byte(g, 2u);
        m.albedo = mix(vec3<f32>(26.0, 36.0, 44.0), vec3<f32>(30.0, 120.0, 140.0), v);
        m.glow = vec3<f32>(10.0, 70.0, 75.0) * v * (0.25 + 0.75 * dp);
        m.albedo = mix(m.albedo, hue * (0.55 + 0.45 * th), clamp(1.0 - exp(-5.0 * th), 0.0, 1.0) * 0.7);
        m.gloss = 1.0;
        m.gem = 1.0;
    }
    return m;
}

fn mat_at(x: f32, y: f32) -> Mat {
    let fx = x - 0.5; let fy = y - 0.5;
    let ix = i32(floor(fx)); let iy = i32(floor(fy));
    let tx = fx - floor(fx); let ty = fy - floor(fy);
    let a = cell_mat(ix, iy); let b = cell_mat(ix + 1, iy);
    let c = cell_mat(ix, iy + 1); let d = cell_mat(ix + 1, iy + 1);
    var m: Mat;
    m.albedo = mix(mix(a.albedo, b.albedo, tx), mix(c.albedo, d.albedo, tx), ty);
    m.glow = mix(mix(a.glow, b.glow, tx), mix(c.glow, d.glow, tx), ty);
    m.gloss = mix(mix(a.gloss, b.gloss, tx), mix(c.gloss, d.gloss, tx), ty);
    m.wound = mix(mix(a.wound, b.wound, tx), mix(c.wound, d.wound, tx), ty);
    m.gem = mix(mix(a.gem, b.gem, tx), mix(c.gem, d.gem, tx), ty);
    return m;
}

// The shaded colour at world point (x, y); `e` is the slope's sample step.
fn shaded(x: f32, y: f32, e: f32) -> vec3<f32> {
    let m = mat_at(x, y);
    let h = height(x, y);
    let dx = height(x + e, y) - height(x - e, y);
    let dy = height(x, y + e) - height(x, y - e);
    var n = normalize(vec3<f32>(-dx / (2.0 * e), -dy / (2.0 * e), 1.0));
    // A crystal's cone cut into six faces: its slope turned to the
    // nearest sixth, so it catches light face by face.
    if (m.gem > 0.5 && length(n.xy) > 0.05) {
        let az = atan2(n.y, n.x);
        let q = round(az / 1.0471976) * 1.0471976;
        let sl = length(n.xy);
        n = normalize(vec3<f32>(cos(q) * sl, sin(q) * sl, n.z));
    }
    let l = normalize(LIGHT);
    var shade = 1.0 + 1.25 * (dot(n, l) - l.z);
    // A short cast shadow: something taller toward the light.
    let toward = height(x + l.x * 3.0, y + l.y * 3.0);
    shade *= 1.0 - 0.35 * clamp((toward - h - 0.3) * 0.5, 0.0, 1.0);
    let half_v = normalize(l + vec3<f32>(0.0, 0.0, 1.0));
    let spec = pow(max(dot(n, half_v), 0.0), 28.0) * m.gloss;
    var c = m.albedo * max(shade, 0.15) + vec3<f32>(255.0, 248.0, 230.0) * spec * 0.7 + m.glow;
    // The wound: where a shot took mould away, the thickness it took, in
    // raw red, fading as the engine fades it.
    // A fresh wound glows like an ember and chars as it fades.
    let wd = clamp(m.wound * 1.3, 0.0, 1.0);
    let burn = mix(vec3<f32>(46.0, 22.0, 14.0), vec3<f32>(255.0, 120.0, 30.0), wd * wd);
    c = mix(c, burn, clamp(wd * 1.6, 0.0, 0.92));
    return c;
}

// Every walker a speck of light at its own position (one thread a walker),
// counted a screen pixel; `base` adds and clears them.
@compute @workgroup_size(256)
fn draw_specks(@builtin(global_invocation_id) gid: vec3<u32>) {
    let k = gid.x;
    if (k >= P.n_walk) { return; }
    let w = walkers[k];
    if (!(w.x >= 0.0 && w.y >= 0.0 && w.x < f32(P.ww) && w.y < f32(P.wh))) { return; }
    let s = vec2<f32>((w.x - P.cx) * P.zoom, (w.y - P.cy) * P.zoom);
    let px = i32(floor(s.x)); let py = i32(floor(s.y));
    if (px < 0 || py < 0 || px >= i32(P.sw) || py >= i32(P.sh)) { return; }
    atomicAdd(&specks[u32(py) * P.stride + u32(px)], 4u);
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
        if (P.shade == 1u && P.zoom >= 1.0) {
            // Shaded from the fields, smooth between cells at any zoom.
            c = shaded(fx, fy, 1.5);
            let n = atomicExchange(&specks[y * P.stride + x], 0u);
            c += vec3<f32>(255.0, 236.0, 180.0) * min(f32(n) * 0.012, 0.25);
        } else if (fx >= 0.0 && fy >= 0.0 && fx < f32(P.ww) && fy < f32(P.wh)) {
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
const SOLDIER: u32 = 7u;

// ---- The soldier: a top-down figure from shapes, no sprite ----
//
// In the soldier's own frame (u along its facing, v across, in radii):
// helmet, shoulders, pack, two arms and a rifle held to the right, a
// shadow cast to the bottom right (the world's light), and a muzzle flash
// while `flash` (0-1, the shot's own timer) is above 0. Lit by the same
// light as the world. Returns colour and alpha (0: not covered).

fn sd_ellipse(p: vec2<f32>, c: vec2<f32>, r: vec2<f32>) -> f32 {
    let q = (p - c) / r;
    return (length(q) - 1.0) * min(r.x, r.y);
}

fn sd_capsule(p: vec2<f32>, a: vec2<f32>, b: vec2<f32>, r: f32) -> f32 {
    let pa = p - a; let ba = b - a;
    let h = clamp(dot(pa, ba) / dot(ba, ba), 0.0, 1.0);
    return length(pa - ba * h) - r;
}

fn sd_box(p: vec2<f32>, c: vec2<f32>, b: vec2<f32>, round: f32) -> f32 {
    let d = abs(p - c) - b + vec2<f32>(round);
    return length(max(d, vec2<f32>(0.0))) + min(max(d.x, d.y), 0.0) - round;
}

// A lit part: a dome of radius `r` round `c` (in radii), coloured `base`.
fn lit(p: vec2<f32>, c: vec2<f32>, r: f32, base: vec3<f32>, ang: f32, gloss: f32) -> vec3<f32> {
    let q = (p - c) / r;
    let z = sqrt(max(1.0 - dot(q, q), 0.0));
    // The light into the soldier's frame (it turns, the light does not).
    let ca = cos(ang); let sa = sin(ang);
    let l = normalize(LIGHT);
    let ll = vec3<f32>(l.x * ca + l.y * sa, -l.x * sa + l.y * ca, l.z);
    let n = normalize(vec3<f32>(q, max(z, 0.2)));
    let d = max(dot(n, ll), 0.0);
    let sp = pow(max(dot(n, normalize(ll + vec3<f32>(0.0, 0.0, 1.0))), 0.0), 24.0) * gloss;
    return base * (0.35 + 0.8 * d) + vec3<f32>(255.0) * sp * 0.6;
}

fn over(under: vec4<f32>, c: vec3<f32>, a: f32) -> vec4<f32> {
    let oa = a + under.w * (1.0 - a);
    if (oa <= 0.0) { return vec4<f32>(0.0); }
    return vec4<f32>((c * a + under.xyz * under.w * (1.0 - a)) / oa, oa);
}

fn cover(sd: f32, aa: f32) -> f32 { return clamp(0.5 - sd / aa, 0.0, 1.0); }

fn soldier(d: vec2<f32>, r: f32, ang: f32, team: vec3<f32>, flash: f32, dead: bool) -> vec4<f32> {
    let aa = 1.0 / max(r, 1.0);
    // The shadow, in screen space (the light does not turn with him).
    let l = normalize(LIGHT);
    var out = vec4<f32>(0.0);
    let sh = sd_ellipse(d / r, vec2<f32>(-l.x, -l.y) * 0.32, vec2<f32>(0.78, 0.78));
    out = over(out, vec3<f32>(0.0), 0.38 * cover(sh, aa * 2.5));
    // Into his frame: u along the facing, v across, in radii.
    let ca = cos(ang); let sa = sin(ang);
    let s = d / r;
    let p = vec2<f32>(s.x * ca + s.y * sa, -s.x * sa + s.y * ca);
    var dim = 1.0;
    if (dead) { dim = 0.45; }
    // Fatigues are olive drab leaning a quarter toward the team; the
    // helmet carries the team's colour whole, so it reads from far out.
    let drab = vec3<f32>(92.0, 98.0, 66.0);
    let uniform_c = mix(drab, team, 0.25) * dim;
    let pack_c = mix(drab, team, 0.12) * 0.62 * dim;
    let sleeve = mix(drab, team, 0.25) * 1.1 * dim;
    // Pack, then shoulders.
    let pk = sd_box(p, vec2<f32>(-0.48, 0.0), vec2<f32>(0.2, 0.4), 0.12);
    out = over(out, lit(p, vec2<f32>(-0.48, 0.0), 0.5, pack_c, ang, 0.1), cover(pk, aa));
    let body = sd_ellipse(p, vec2<f32>(-0.08, 0.0), vec2<f32>(0.38, 0.8));
    out = over(out, lit(p, vec2<f32>(-0.06, 0.0), 0.8, uniform_c, ang, 0.1), cover(body, aa));
    if (!dead) {
        // Arms out to the rifle, held to his right (+v).
        let arm_r = sd_capsule(p, vec2<f32>(-0.02, 0.58), vec2<f32>(0.5, 0.36), 0.15);
        let arm_l = sd_capsule(p, vec2<f32>(0.0, -0.56), vec2<f32>(0.78, 0.26), 0.14);
        out = over(out, lit(p, vec2<f32>(0.25, 0.47), 0.35, sleeve, ang, 0.1), cover(arm_r, aa));
        out = over(out, lit(p, vec2<f32>(0.39, -0.15), 0.45, sleeve, ang, 0.1), cover(arm_l, aa));
        // The rifle.
        let gun = sd_capsule(p, vec2<f32>(0.0, 0.3), vec2<f32>(1.45, 0.3), 0.1);
        out = over(out, vec3<f32>(34.0, 34.0, 38.0) + vec3<f32>(70.0) * clamp(0.35 - (p.y - 0.3) * 3.0, 0.0, 1.0), cover(gun, aa * 0.7));
    }
    // The helmet, the team's colour, glossy.
    let hc = vec2<f32>(0.04, 0.0);
    let helm = length(p - hc) - 0.34;
    out = over(out, lit(p, hc, 0.34, team * dim, ang, 0.7 * dim), cover(helm, aa));
    // The muzzle flash: the shot leaving, as long as the game says it lasts.
    if (flash > 0.0 && !dead) {
        let fl = sd_ellipse(p, vec2<f32>(1.62, 0.3), vec2<f32>(0.3 * flash + 0.08, 0.17 * flash + 0.05));
        let core = 1.0 - clamp(length((p - vec2<f32>(1.58, 0.3)) / vec2<f32>(0.3, 0.17)), 0.0, 1.0);
        out = over(out, mix(vec3<f32>(255.0, 170.0, 60.0), vec3<f32>(255.0, 250.0, 220.0), core), cover(fl, aa) * clamp(flash * 1.5, 0.0, 1.0));
    }
    return out;
}

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
    if (shape == SOLDIER) {
        // Rifle and flash reach past the radius.
        x0 = floor(s.x - 2.0 * h.x); x1 = ceil(s.x + 2.0 * h.x);
        y0 = floor(s.y - 2.0 * h.x); y1 = ceil(s.y + 2.0 * h.x);
    }
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
        } else if (shape == SOLDIER) {
            on = soldier_at(b, d, h.x).w > 0.004;
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

fn soldier_at(b: u32, d: vec2<f32>, r: f32) -> vec4<f32> {
    let extra = marks[b + 11u];
    return soldier(d, r, bitcast<f32>(marks[b + 9u]), unpack(marks[b + 7u]), f32(extra & 255u) / 255.0, (extra & 256u) != 0u);
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
        if (marks[b + 6u] == SOLDIER && !in_minimap(x, y)) {
            let d = vec2<f32>(f32(x) + 0.5, f32(y) + 0.5) - anchor(b);
            let f = soldier_at(b, d, half(b).x);
            c = c * (1.0 - f.w) + f.xyz * f.w;
        } else {
            let a = f32(col >> 24u) / 255.0;
            c = c * (1.0 - a) + unpack(col) * a;
        }
    }
    if (P.overlay == 1u) {
        let o = overlay[y * P.sw + x];
        let oa = f32(o >> 24u) / 255.0;
        c = unpack(o) + c * (1.0 - oa);
    }
    out[i] = pack(c);
}
