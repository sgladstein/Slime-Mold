// The camera's view pass (src/world/view.rs): a world-sized RGBA frame
// (any producer; the GPU engine's `frame`) drawn onto a screen-sized
// buffer through the camera, with the minimap, the game's marks and the
// screen-space overlay (HUD text) on top. Passes: `prep` (each shown cell once), `base` (a thread a
// screen pixel), `draw_marks` (a workgroup a mark, claiming pixels), and
// `compose` (a thread a pixel: the marks' colours and the overlay).

struct V {
    sw: u32, sh: u32, stride: u32, ww: u32,
    wh: u32, n_marks: u32, mask: u32, overlay: u32,
    // n_lights: the point lights this frame (`lights`, `View::set_lights`).
    cx: f32, cy: f32, zoom: f32, n_lights: u32,
    // The minimap's rectangle on screen (mw 0: none).
    mx: u32, my: u32, mw: u32, mh: u32,
    // The look (`View::set_look`): 1 shades the world from the engine's
    // look and ground fields, with every walker a speck; 0 the frame only.
    // lives 1: `alive` holds each walker's energy, and a slot below 0 (empty
    // or dead) draws no speck (`View::set_alive`).
    shade: u32, n_walk: u32, lives: u32, body_density: f32,
    // The rectangle of cells `prep` works out (origin ox, oy; rw x rh):
    // the cells the screen shows and a margin of 4 for the slope, shadow
    // and blend samples.
    ox: i32, oy: i32, rw: u32, rh: u32,
    // Food's news (`View::set_news`): 1 when the engine's news and step are
    // bound; a walker whose news is more than `lag` steps old is cut off
    // from food (it withers and, with panic on, runs).
    news_on: u32, lag: u32, lean: u32,
    // The crystals drawn as hexagons (`View::set_crystals`), 0 for none.
    n_gems: u32,
    // Fine cells a cell each way (`prep`'s grid; 1 or 2 by the zoom).
    fine: u32,
    // The crystals' live charge (`View::set_charge`): crystal cells listed,
    // the game seconds since the last frame, and 0 off, 1 on, 2 a first
    // frame that only reads the charge.
    n_list: u32, dt: f32, live: u32,
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
// The sprites (`src/world/sprites/`): 2048 x 160 RGBA, sixteen 128 px
// tiles (soldier body, helmet, flash, dead; six buildings, each body then
// lamp) above the same at 32 px.
@group(0) @binding(11) var<storage, read> atlas: array<u32>;
// Each shown cell's height and material, worked out once a frame by `prep`
// (rows `rw` apart from cell ox, oy). `base` reads these: a pixel needs 24
// heights (bilinear at six points) and four materials, and before this it
// worked each one out from the fields itself, which made the look cost
// 1.8-2.0 ms against 0.3 ms plain on the GTX 1070 at 1080p.
@group(0) @binding(12) var<storage, read_write> hcell: array<f32>;
@group(0) @binding(13) var<storage, read_write> mcell: array<vec2<u32>>;
@group(0) @binding(14) var<storage, read> alive: array<f32>;
// Food's news (`View::set_news`): the engine's binding 48 (the step each
// walker last heard of food, its first `n_walk` words) and its state (the
// step now, word 0); `cut` counts the cut-off walkers a world cell each
// frame (`draw_specks`), for `prep`.
@group(0) @binding(15) var<storage, read> news: array<u32>;
@group(0) @binding(16) var<storage, read> state: array<u32>;
@group(0) @binding(17) var<storage, read_write> fgrid: array<atomic<u32>>;
// The crystals (`View::set_crystals`): each one's centre and circumradius
// in cells (x, y, r, _), and a world cell's crystal (index + 1, 0 none)
// for the cells within one of a crystal's footprint, so a pixel finds the
// hexagon its edge cuts through.
@group(0) @binding(18) var<storage, read> gems: array<vec4<f32>>;
@group(0) @binding(19) var<storage, read> gem_cell: array<u32>;
// Point lights (`View::set_lights`, 2026-11-14; binding 20, after the
// graphics thread's 15-19): two vec4s each, (x, y,
// reach, lift) in world cells and (r, g, b, strength) in colour units.
// They are things that give off light in the game (a bullet in flight,
// a burst for its first tenth of a second, a muzzle as it fires), and
// they light the world's own relief in `shaded`: the mould's height and
// gloss catch them, bare agar dimly, as Noita lights its world from a
// spell's light (Reports/noita-spell-look-2026-11-14.md).
@group(0) @binding(20) var<storage, read> lights: array<vec4<f32>>;
// The crystals' live charge (`View::set_charge`): the engine's charge a
// cell; each listed crystal cell's charge last frame and how fast it is
// being taken (`drain`); the crystal cells listed.
@group(0) @binding(21) var<storage, read> charge: array<f32>;
@group(0) @binding(22) var<storage, read_write> heat: array<vec2<f32>>;
@group(0) @binding(24) var<storage, read> gem_list: array<u32>;
// Each shown cell's hard terrain (rock, wall, rubble), worked out by `prep`
// for `terrain_over` (whole cells, rows `rw / fine` apart from cell ox, oy; `tidx`): byte 0 the cells'
// occupancy smoothed over 3 x 3 (1 2 1), byte 1 the height (0-4 cells),
// byte 2 the kind, byte 3 its value (a wall's health; rock's elevation).
// Binding 23 (20 is the lights, 21, 22 and 24 the live charge).
@group(0) @binding(23) var<storage, read_write> tcell: array<u32>;

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

// How tall the land's full elevation stands, in cells: about a fat tube's
// height (4), so hills light and shade like the mould without hiding it.
// Maps leave elevation 0 until the simulation uses it (the Larder thread
// proposes it as the first new ground kind: slower uphill, wet hollows,
// scent drifting down), so the height drawn is the height the mould feels.
const ELEV: f32 = 5.0;
// Bare ground: damp low loam to dry high soil, by elevation. Dark, cool
// and low in saturation since 2026-11-14 (art direction rules 2 and 3:
// the ground is the darkest thing and slate, so the gold slime and the
// cyan crystals are the only colour); before, (30,34,28) to (84,74,58),
// an olive-brown whose middle (57,54,43) read as sand beside the slime.
const SOIL_LOW: vec3<f32> = vec3<f32>(18.0, 22.0, 27.0);
const SOIL_HIGH: vec3<f32> = vec3<f32>(52.0, 55.0, 58.0);
// Soil kinds (byte 0's high nibble, `look::Fields::soil`; the mould's
// `BigDish::soil`, Larder World maps): each changes the mould (sand: its
// trail fades, and it spreads further; clay: its trail lasts, and it is
// trapped; barren: no births, upkeep x 3; Reports/larder-maps-2026-11-11.md,
// *Which ground the mould feels*), so each reads as its own ground.
// Each still dark and muted beside plain ground (35,38,42 flat), told
// apart by value and a slight cast: sand paler and faintly warm, clay a
// dull red-brown, barren a flat ash grey. Before 2026-11-14 sand was
// (150,134,92), brighter than thin slime, and clay a saturated brick.
const SAND_RGB: vec3<f32> = vec3<f32>(70.0, 68.0, 60.0);
const CLAY_RGB: vec3<f32> = vec3<f32>(56.0, 38.0, 34.0);
const BARREN_RGB: vec3<f32> = vec3<f32>(50.0, 50.0, 50.0);
// Mould cut off from food fades towards this (`cell_mat`'s `cut`).
const ASH_RGB: vec3<f32> = vec3<f32>(120.0, 112.0, 104.0);
// The engine's colour for bare agar; what it adds above this (scent,
// species, germ, grass tints) is laid over the soil.
const AGAR_RGB: vec3<f32> = vec3<f32>(12.0, 22.0, 26.0);
// The mould's gold (`draw`'s (255, 190, 90) over agar) as a hue at 240.
const MOULD_HUE: vec3<f32> = vec3<f32>(240.0, 165.9, 63.2);

// The mould's thickness as drawn: the trail's brightness (the look's byte
// 0) times how many living walkers are near (byte 1 summed over the 5 x 5
// cells round it). The trail alone saturates (`bright` is 1 - exp(-v/knee))
// and diffuses back over a cut within a frame or two, so before 2026-11-12
// the body looked the same after shots killed 30% of its walkers (owner's
// playtest, 04:13Z: "visually it didn't impact much"); with the walkers in
// it, a patch they died in is thin until living ones come back. `BODY_N`
// is the walkers in that window that make a body two-thirds thick: in a
// rush on the fork map (5000 walkers on 512 x 320 cells, `BODY_DENSITY`
// walkers a cell) the median cell with trail of 0.5 or more had 14 near
// it, the tenth percentile 4. It scales with the world's living walkers a
// cell (`View::set_body_density`; 0 keeps the calibration), so a world
// with ten times the walkers does not saturate to the trail again and a
// sparse one does not read thin (tech lead review of #93, 04:48Z).
// Whole walkers against a window scaled by living density (linear, as
// `Rules::mass` is), so C's body reads as thick as the 5,000-walker one's.
const BODY_N: f32 = 6.0;
const BODY_DENSITY: f32 = 5000.0 / (512.0 * 320.0);

// The body from `n`, the walkers on the 5 x 5 cells round (ix, iy).
fn body(ix: i32, iy: i32, n: u32) -> f32 {
    var n0 = BODY_N;
    if (P.body_density > 0.0) { n0 = BODY_N * max(P.body_density / BODY_DENSITY, 0.05); }
    return byte(look[cidx(ix, iy)], 0u) * (1.0 - exp(-f32(n) / n0));
}

// A cell's height in cells: off the world is rock.
fn cell_height(ix: i32, iy: i32, th: f32) -> f32 {
    // Off the world, rock, walls and rubble stand at the land's height
    // here: `terrain_over` draws them, edges and shadows, at the pixel
    // (see *Hard terrain*). Their cell heights used to stand here, and the
    // slope between a tall cell and the ground made a stepped bevel and a
    // dark band a cell wide round every rock and wall at high zoom.
    if (ix < 0 || iy < 0 || ix >= i32(P.ww) || iy >= i32(P.wh)) { return 0.0; }
    let i = cidx(ix, iy);
    let g = ground[i];
    let kind = g & 15u;
    let v = byte(g, 1u);
    // The land (byte 3, the map's elevation) under everything on it.
    let land = ELEV * byte(g, 3u);
    // Thick veins stand higher than before (4 th^0.65) so they read as
    // raised tubes with a lit and a shaded flank (2026-11-14, material).
    // Tried 5 th^0.75: the dense body's trail swings showed as mottling.
    // A saturating curve, so the dense body's small swings of trail read
    // as a calm surface, not as sand grains (2026-11-14).
    var h = 5.0 * (1.0 - exp(-2.6 * th)) / (1.0 - exp(-2.6));
    // Hard terrain is drawn by `terrain_over`; its cells shade as soil.
    if (kind == ROCK || kind == WALL || kind == RUBBLE) { h = 0.0; }
    else if (kind == WET) { h = max(h - 0.3, 0.0); }
    else if (kind == CRYSTAL && P.n_gems == 0u) { h = max(h, (0.6 + 9.0 * byte(g, 2u)) * v); }
    return land + h;
}

// A fine cell's place in `hcell` and `mcell` (fine cells from the
// rectangle's corner, clamped to it).
fn ridx(ix: i32, iy: i32) -> u32 {
    let x = clamp(ix, 0, i32(P.rw) - 1);
    let y = clamp(iy, 0, i32(P.rh) - 1);
    return u32(y) * P.rw + u32(x);
}

// World point (x, y) in fine cells from the rectangle's corner, less a
// half, for bilinear samples of `hcell` and `mcell`.
fn fpos(x: f32, y: f32) -> vec2<f32> {
    let k = f32(P.fine);
    return vec2<f32>((x - f32(P.ox)) * k - 0.5, (y - f32(P.oy)) * k - 0.5);
}

fn hc(ix: i32, iy: i32) -> f32 { return hcell[ridx(ix, iy)]; }

// Crystals drawn at screen resolution (2026-11-14, the owner's playtest,
// 01:49Z: "The crystals are pixelated"). Before, a crystal stood in the
// cell field: its footprint and its pyramid were cells, bilinear between
// them, so at zoom 6 its slanted edges were a staircase of 6-pixel steps.
// Now a crystal is its own hexagon, the one the simulation's footprint is
// cut from (`look::dish_crystals`, pointy-top, circumradius r): its
// height is a pyramid on the exact hexagon, its faces flat with the
// normal worked out, not sampled, and its edge antialiased to a pixel.
// Its charge is still each cell's (the ground's byte 1), so a crystal
// fed on unevenly still shows it.

// How far (x, y) is inside crystal k's hexagon, in cells (below 0 outside),
// and the gradient of that distance.
fn gem_in(k: u32, x: f32, y: f32) -> vec3<f32> {
    let g = gems[k];
    let p = vec2<f32>(x - g.x, y - g.y);
    let a = g.z * 0.8660254;
    let q = vec2<f32>(abs(p.x), abs(p.y));
    let side = q.x;
    let slant = 0.5 * q.x + 0.8660254 * q.y;
    if (side >= slant) { return vec3<f32>(a - side, -sign(p.x), 0.0); }
    return vec3<f32>(a - slant, -0.5 * sign(p.x), -0.8660254 * sign(p.y));
}

// The crystal (index + 1) whose footprint is at or next to (x, y); 0 none.
fn gem_near(x: f32, y: f32) -> u32 {
    if (P.n_gems == 0u || x < 0.0 || y < 0.0 || x >= f32(P.ww) || y >= f32(P.wh)) { return 0u; }
    return gem_cell[u32(y) * P.ww + u32(x)];
}

// ---- The crystals' state: charge, taking, spent ----
//
// What a crystal shows is its charge, cell by cell (owner's playtest
// 11-14 04:53Z: "harvesting crystals could look way better"): it stands
// as tall as its charge and glows with it, so the side being taken sinks
// and dims first; a cell being taken glows white-hot by how fast it goes
// (`drain`), whether a soldier or the mould is taking it; and a dry
// crystal is a low grey stub of spent rock. With `View::set_charge` the
// charge is the engine's, live each frame; without, the ground's share
// (refreshed once a game second).

// A crystal cell's share of its full charge (cell i of crystal k).
fn cell_share(k: u32, i: u32) -> f32 {
    if (P.live != 0u) { return clamp(charge[i] / max(gems[k].w, 1e-6), 0.0, 1.0); }
    return byte(ground[i], 1u);
}

// How hot a crystal cell is: the share of its full charge taken a game
// second, smoothed (`drain`); 0 without the live charge.
fn cell_heat(i: u32) -> f32 {
    if (P.live == 0u) { return 0.0; }
    return heat[i].y;
}

// Crystal k's charge share and heat at (x, y): a cubic B-spline (`bc`) over
// its own cells of the 4 x 4 round the point, so the soil beside it does not
// dim its edge; at the edge, where none is crystal, the nearest cell in
// towards the centre. Bilinear, a crystal harvested row by row (the
// squad's take works through its cells in rows) showed each cell's step
// at zoom 12 (2026-11-14).
fn gem_state(k: u32, x: f32, y: f32) -> vec2<f32> {
    let ux = x - 0.5; let uy = y - 0.5;
    let ix = i32(floor(ux)); let iy = i32(floor(uy));
    let wx = bc(ux - floor(ux)); let wy = bc(uy - floor(uy));
    var acc = vec2<f32>(0.0);
    var wsum = 0.0;
    for (var j = 0; j < 4; j++) {
        for (var q = 0; q < 4; q++) {
            let cx = ix - 1 + q; let cy = iy - 1 + j;
            if (cx < 0 || cy < 0 || cx >= i32(P.ww) || cy >= i32(P.wh)) { continue; }
            let i = cidx(cx, cy);
            if ((ground[i] & 15u) != CRYSTAL) { continue; }
            let w = wx[q] * wy[j];
            acc += w * vec2<f32>(cell_share(k, i), cell_heat(i));
            wsum += w;
        }
    }
    if (wsum > 1e-3) { return acc / wsum; }
    let c = gems[k].xy;
    let d = c - vec2<f32>(x, y);
    let u = d / max(length(d), 1e-3);
    for (var s = 1; s <= 2; s++) {
        let q = vec2<f32>(x, y) + u * 0.9 * f32(s);
        let i = cidx(i32(floor(q.x)), i32(floor(q.y)));
        if ((ground[i] & 15u) == CRYSTAL) { return vec2<f32>(cell_share(k, i), cell_heat(i)); }
    }
    return vec2<f32>(0.0);
}

fn gem_charge(k: u32, x: f32, y: f32) -> f32 { return gem_state(k, x, y).x; }

// A spent crystal keeps this share of its height: a low stub of grey rock,
// not a hole.
const GEM_STUB: f32 = 0.15;
// The most a mould film on a crystal covers it (`shaded`): a thick mat
// still lets a fifth of the faces and glow through, so it reads as a
// crystal under mould.
const GEM_FILM: f32 = 0.75;
// The rate (share of a cell's full charge a game second) that lights a
// cell half way.
const GEM_HOT: f32 = 0.15;
// How long a cell's heat lasts after the taking stops, game seconds.
const GEM_TAU: f32 = 0.35;

fn gem_tall(v: f32) -> f32 { return GEM_STUB + (1.0 - GEM_STUB) * v; }

// A crystal's height at (x, y) above the ground (0 off any crystal).
fn gem_height(x: f32, y: f32) -> f32 {
    let k1 = gem_near(x, y);
    if (k1 == 0u) { return 0.0; }
    let d = gem_in(k1 - 1u, x, y).x;
    if (d <= 0.0) { return 0.0; }
    let depth = min(d / (gems[k1 - 1u].z * 0.8660254), 1.0);
    return (0.6 + 9.0 * depth) * gem_tall(gem_charge(k1 - 1u, x, y));
}

// Each listed crystal cell's heat: how fast its charge is going, a share of
// its full charge a game second, held for GEM_TAU after it stops. A thread
// a crystal cell; nothing changes on a frame with no game time.
@compute @workgroup_size(256)
fn drain(@builtin(global_invocation_id) gid: vec3<u32>) {
    let q = gid.x;
    if (q >= P.n_list) { return; }
    let i = gem_list[q];
    let c = charge[i];
    let hp = heat[i];
    if (P.live == 2u) { heat[i] = vec2<f32>(c, 0.0); return; }
    if (P.dt <= 0.0) { return; }
    let k1 = gem_cell[i];
    var full = 1.0;
    if (k1 > 0u) { full = max(gems[k1 - 1u].w, 1e-6); }
    let rate = max(hp.x - c, 0.0) / full / P.dt;
    heat[i] = vec2<f32>(c, max(rate, hp.y * exp(-P.dt / GEM_TAU)));
}

fn height(x: f32, y: f32) -> f32 {
    return max(cells_height(x, y), gem_height(x, y));
}

// The cells' height at (x, y): a cubic B-spline of the fine cells round
// (smooth in its slope too). Bilinear, the slope jumped at each fine cell's
// border, and the shading showed faint squares a cell wide at zoom 12-24.
fn bspline(t: f32) -> vec4<f32> {
    let t2 = t * t; let t3 = t2 * t;
    return vec4<f32>((1.0 - t) * (1.0 - t) * (1.0 - t), 3.0 * t3 - 6.0 * t2 + 4.0, -3.0 * t3 + 3.0 * t2 + 3.0 * t + 1.0, t3) / 6.0;
}

fn cells_height_lin(x: f32, y: f32) -> f32 {
    let f = fpos(x, y);
    let ix = i32(floor(f.x)); let iy = i32(floor(f.y));
    let tx = f.x - floor(f.x); let ty = f.y - floor(f.y);
    return mix(mix(hc(ix, iy), hc(ix + 1, iy), tx), mix(hc(ix, iy + 1), hc(ix + 1, iy + 1), tx), ty);
}

fn cells_height(x: f32, y: f32) -> f32 {
    let f = fpos(x, y);
    let ix = i32(floor(f.x)); let iy = i32(floor(f.y));
    let wx = bspline(f.x - floor(f.x)); let wy = bspline(f.y - floor(f.y));
    var h = 0.0;
    for (var j = 0; j < 4; j++) {
        let r = wx.x * hc(ix - 1, iy - 1 + j) + wx.y * hc(ix, iy - 1 + j) + wx.z * hc(ix + 1, iy - 1 + j) + wx.w * hc(ix + 2, iy - 1 + j);
        h += wy[j] * r;
    }
    return h;
}

// `body`: the wide walker count (`prep`'s `wide`, 0-1) in the byte the
// wound had (no wound is drawn, see `shaded`), for the specks' light.
struct Mat { albedo: vec3<f32>, glow: vec3<f32>, gloss: f32, body: f32, gem: f32 };

// A cell's material: the engine's colour for agar (mould, scent, species,
// germ, grass all in it), the ground kinds in their own colours, scaled by
// their value where it is graded.
// `cut`: the share of the walkers round the cell whose news of food is
// stale (`View::set_news`), 0 without it.
// The engine's colour for cell (ix, iy), as the look takes it.
fn cell_colour(ix: i32, iy: i32) -> vec3<f32> {
    let i = cidx(ix, iy);
    var c = unpack(world[i]);
    let kind = ground[i] & 15u;
    // With crystals drawn as hexagons (`set_crystals`), a crystal cell's
    // own colour (the engine's crystal cyan) would tint the mould on it and
    // show as a cyan rim round the hexagon. The engine draws a crystal cell
    // as base = agar + (cyan - agar) x share, then the mould over it as
    // base + (gold - base) x g (`engine.wgsl`, the game's amber); so the
    // mould's own g comes back from the red channel, where gold and the
    // crystal differ most, and the cell is drawn as that mould on bare
    // agar. Before 11-14 08:30Z the colour was taken from the nearest
    // non-crystal cell within 6, which a crystal 30 cells wide never
    // reached: its middle kept the cyan, and the mould on it read pale.
    if (kind == CRYSTAL && P.n_gems > 0u) {
        let sh = byte(ground[i], 1u);
        let base = AGAR_RGB + (vec3<f32>(140.0, 235.0, 255.0) - AGAR_RGB) * sh;
        let g = clamp((c.x - base.x) / max(255.0 - base.x, 1.0), 0.0, 1.0);
        c = AGAR_RGB + (vec3<f32>(255.0, 190.0, 90.0) - AGAR_RGB) * g;
    }
    if (P.mask == 1u) { c *= 0.3 + 0.7 * clamp(vis[i], 0.0, 1.0); }
    return c;
}

// The engine's colour at world point (x, y): a Gaussian of the 4 x 4
// cells round, for the fine cells (`prep`), so the mould's hue does not
// change in cell-sized squares when zoomed in.
fn colour_at(x: f32, y: f32) -> vec3<f32> {
    let ix = i32(floor(x - 0.5)) - 1; let iy = i32(floor(y - 0.5)) - 1;
    var c = vec3<f32>(0.0);
    var ws = 0.0;
    for (var dy = 0; dy < 4; dy++) {
        for (var dx = 0; dx < 4; dx++) {
            let d = vec2<f32>(f32(ix + dx) + 0.5 - x, f32(iy + dy) + 0.5 - y);
            let w = exp(-dot(d, d) / (2.0 * 0.55 * 0.55));
            c += w * cell_colour(ix + dx, iy + dy);
            ws += w;
        }
    }
    return c / ws;
}

fn cell_mat(ix: i32, iy: i32, th: f32, cut: f32) -> Mat {
    return cell_mat_c(ix, iy, th, cut, cell_colour(ix, iy));
}

// A cell's material with the engine's colour `c` (`cell_colour`, or the
// smoothed `colour_at` for a fine cell).
fn cell_mat_c(ix: i32, iy: i32, th: f32, cut: f32, c: vec3<f32>) -> Mat {
    var m: Mat;
    if (ix < 0 || iy < 0 || ix >= i32(P.ww) || iy >= i32(P.wh)) {
        m.albedo = mix(SOIL_LOW, SOIL_HIGH, 0.5); m.glow = vec3<f32>(0.0); m.gloss = 0.05; m.body = 0.0; m.gem = 0.0;
        return m;
    }
    let i = cidx(ix, iy);
    let g = ground[i];
    let kind = g & 15u;
    let v = byte(g, 1u);
    let lk = look[i];
    // The mould: the engine's colour carries the hue (species, germ); the
    // look carries it at full strength wherever there is mould, by the
    // thickness, so a thin film reads as slime, not as dim ground.
    let tint = max(c - AGAR_RGB, vec3<f32>(0.0));
    let tmax = max(tint.x, max(tint.y, tint.z));
    var hue = tint / max(tmax, 1.0) * 240.0;
    // Where walkers stand on little or no trail, the engine's colour has
    // next to no mould in it, and the line above blew that near-nothing
    // up to full strength: zero went black, a faint reddish ground tint
    // pure red. The owner (11-14 08:30Z, "Are these blackish spots in the
    // middle of the mold intentional?") saw black blobs with red rims in a
    // thick body: crater and scorch cells, whose trail is wiped while the
    // walkers stay (`look_trail` 0, `tm` from the walkers alone). There the
    // hue is the mould's own gold (the engine's `draw` gold over agar),
    // taking the engine's hue as its colour carries mould.
    hue = mix(MOULD_HUE, hue, smoothstep(6.0, 40.0, tmax));
    // A piece cut off from food (C8, 2026-11-14) goes ashen and dull as its
    // walkers' news goes stale, before they wither: the share of stale
    // walkers is the fade, so a piece greys as it starves, not on a timer.
    hue = mix(hue, ASH_RGB, 0.75 * cut);
    // Bare ground is soil by the land's elevation, with the engine's tints
    // over it; before 2026-11-11 19:29Z it was the engine's flat near-black.
    // Flat land (elevation 0, every map until the simulation uses height)
    // is the middle of the range.
    var base = mix(SOIL_LOW, SOIL_HIGH, 0.5 + 0.5 * byte(g, 3u));
    let sk = (g >> 4u) & 7u;
    if (sk == 1u) { base = SAND_RGB; } else if (sk == 2u) { base = CLAY_RGB; } else if (sk == 3u) { base = BARREN_RGB; }
    let soil = base + tint * 0.6;
    // The slime's material (art director, 2026-11-14: "it still reads as
    // sand against the olive ground"). Before, the body was the hue at 55%
    // to 100% by thickness, laid over the soil up to 85%: a thin film was
    // dim opaque paint and a thick vein a flat ochre. Now, by the same
    // thickness: a thin film is a gel the ground shows through (the soil
    // tinted by the hue, as through coloured glass), and the body turns
    // opaque and brighter gold as it thickens, its hue pushed away from
    // grey so it is the most saturated warm colour on screen; a starving
    // piece loses the push with its colour (`cut`). Every weight is a
    // smooth function of `th`, so no line is drawn where the film ends.
    let lum = dot(hue, vec3<f32>(0.299, 0.587, 0.114));
    let vivid = lum + (hue - vec3<f32>(lum)) * (1.0 + 0.45 * (1.0 - cut));
    let body_c = vivid * (0.95 + 0.3 * smoothstep(0.3, 1.0, th));
    let gel = soil * (0.55 + 0.7 * vivid / 255.0);
    let film = 1.0 - exp(-14.0 * th);
    let opaque = smoothstep(0.08, 0.55, th);
    m.albedo = mix(mix(soil, gel, film), body_c, opaque * 0.95);
    m.glow = vec3<f32>(0.0);
    m.gloss = (0.1 + 0.9 * sqrt(th)) * (1.0 - 0.6 * cut);
    m.body = 0.0;
    m.gem = 0.0;
    if (kind == WET) {
        // Standing water over the soil: dark blue-green, glossy (darker
        // since 2026-11-14 with the ground); mould in it keeps its own colour.
        m.albedo = mix(vec3<f32>(14.0, 30.0, 42.0) + tint * 0.4, m.albedo, clamp(1.0 - exp(-5.0 * th), 0.0, 1.0) * 0.85);
        m.gloss = max(m.gloss, 0.75);
    } else if (kind == ROCK || kind == WALL || kind == RUBBLE) {
        // Under hard terrain, the soil: `terrain_over` lays the rock, wall
        // or rubble over it with an edge of its own, so the cells next to
        // it blend toward soil, not toward a grey cell (`ter_cell` has
        // their colours).
        m.gloss = 0.05;
    } else if (kind == CRYSTAL && P.n_gems == 0u) {
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

// A material in two u32s: albedo and gloss, glow and body, a byte each
// (albedo and glow in colour units, gloss and body 0-1 as 0-255). `gem`
// is not drawn and not kept.
fn mat_pack(m: Mat) -> vec2<u32> {
    let a = vec4<u32>(vec4<f32>(clamp(m.albedo, vec3<f32>(0.0), vec3<f32>(255.0)), clamp(m.gloss, 0.0, 1.0) * 255.0) + 0.5);
    let g = vec4<u32>(vec4<f32>(clamp(m.glow, vec3<f32>(0.0), vec3<f32>(255.0)), clamp(m.body, 0.0, 1.0) * 255.0) + 0.5);
    return vec2<u32>(a.x | a.y << 8u | a.z << 16u | a.w << 24u, g.x | g.y << 8u | g.z << 16u | g.w << 24u);
}

fn mc(ix: i32, iy: i32) -> Mat {
    let p = mcell[ridx(ix, iy)];
    var m: Mat;
    m.albedo = unpack(p.x);
    m.gloss = byte(p.x, 3u);
    m.glow = unpack(p.y);
    m.body = byte(p.y, 3u);
    m.gem = 0.0;
    return m;
}

// Works out each shown cell's height and material once a frame (a thread
// a cell of the rectangle). The walkers for `body` come through a 20 x 20
// tile in workgroup memory (the 16 x 16 cells and a 2-cell rim), loaded once:
// reading the 25 cells from the look for each cell cost the 1070 about
// 0.5 ms at zoom 1 (1920 x 1080; 1.19 -> 1.68 ms, matched fast runs).
var<workgroup> crowd: array<u32, 576>;

// The slime below the cell (2026-11-14, the owner's playtest, 01:49Z: "The
// slime looks very blurry/out of focus"). Before, the body was one value a
// cell, from the walkers on the 5 x 5 cells round it, bilinear between
// cells and its slope sampled 1.5 cells apart: at zoom 6 its edge spread
// over some 30 pixels, at zoom 24 over 100. Now `prep` works on fine cells
// (`fine` a cell each way, 1 or 2 by the zoom) and the walkers are counted
// where they really are, a fine cell each (`draw_specks`), then summed
// round each fine cell with a Gaussian of SHARP_S cells (SHARP_S1 at one
// fine cell a cell), so the edge is as sharp as the walkers make it. The
// ground under it (kind, colour, trail) is still the cells', blended
// bilinear, so walls and soil look as before. A first step with the same
// cells and a 0.8-cell Gaussian and a 0.75-cell slope sharpened zoom 4-6
// but left zoom 12-24 as soft as it was (one cell 12-24 pixels).
const SHARP_S: f32 = 0.6;
const SHARP_S1: f32 = 0.8;
// The dense body is smoothed wider (BODY_S cells): grain only where the
// walkers really are sparse, at the frontier (the art director, 02:26Z:
// "sharp is right, lumpy is wrong"; the first fine try at 0.5 cells
// everywhere read as cauliflower inside the body).
const BODY_S: f32 = 1.1;

// The trail (the look's byte 0) at (x, y): a Gaussian of the 4 x 4 cells
// round, smooth in the position, so its slope has no crease at the cell
// borders (bilinear did: square facets a cell wide in the shading).
fn look_trail(x: f32, y: f32) -> f32 {
    let ix = i32(floor(x - 0.5)) - 1; let iy = i32(floor(y - 0.5)) - 1;
    var t = 0.0;
    var ws = 0.0;
    for (var dy = 0; dy < 4; dy++) {
        for (var dx = 0; dx < 4; dx++) {
            let cx = ix + dx; let cy = iy + dy;
            let d = vec2<f32>(f32(cx) + 0.5 - x, f32(cy) + 0.5 - y);
            let w = exp(-dot(d, d) / (2.0 * 0.55 * 0.55));
            t += w * byte(look[cidx(cx, cy)], 0u);
            ws += w;
        }
    }
    return t / ws;
}

@compute @workgroup_size(16, 16)
fn prep(@builtin(global_invocation_id) gid: vec3<u32>, @builtin(local_invocation_id) lid: vec3<u32>, @builtin(workgroup_id) wid: vec3<u32>, @builtin(local_invocation_index) li: u32) {
    // The walkers a fine cell (and, above bit 16, the cut-off ones) on a
    // 24 x 24 tile: the workgroup's 16 x 16 and a 4-cell rim.
    let bx = i32(wid.x * 16u) - 4;
    let by = i32(wid.y * 16u) - 4;
    for (var k = li; k < 576u; k += 256u) {
        let fx = bx + i32(k % 24u); let fy = by + i32(k / 24u);
        var v = 0u;
        if (fx >= 0 && fy >= 0 && fx < i32(P.rw) && fy < i32(P.rh)) { v = atomicLoad(&fgrid[u32(fy) * P.rw + u32(fx)]); }
        crowd[k] = v;
    }
    workgroupBarrier();
    if (gid.x >= P.rw || gid.y >= P.rh) { return; }
    let i = gid.y * P.rw + gid.x;
    let kf = f32(P.fine);
    var sig = SHARP_S * kf;
    if (P.fine == 1u) { sig = SHARP_S1; }
    let sigb = BODY_S * kf;
    var n = 0.0;
    var m = 0.0;
    var ws = 0.0;
    var nb = 0.0;
    var wb = 0.0;
    // Gaussians are separable: nine weights a side each, not 81 exps.
    var g1: array<f32, 9>;
    var g2: array<f32, 9>;
    for (var q = 0; q < 9; q++) {
        let d2 = f32((q - 4) * (q - 4));
        g1[q] = exp(-d2 / (2.0 * sig * sig));
        g2[q] = exp(-d2 / (2.0 * sigb * sigb));
    }
    for (var dy = 0u; dy < 9u; dy++) {
        for (var dx = 0u; dx < 9u; dx++) {
            let t = crowd[(lid.y + dy) * 24u + lid.x + dx];
            let c = f32(t & 0xFFFFu) / 15.0;
            let w = g1[dx] * g1[dy];
            n += w * c;
            m += w * f32(t >> 16u) / 15.0;
            ws += w;
            let v = g2[dx] * g2[dy];
            nb += v * c;
            wb += v;
        }
    }
    let cut_share = clamp(m / max(n, 1e-3), 0.0, 1.0);
    // Walkers a cell here, as the 5 x 5 window `BODY_N` was set on: near
    // (the sharp count) and wide (the body's).
    var n0 = BODY_N;
    if (P.body_density > 0.0) { n0 = BODY_N * max(P.body_density / BODY_DENSITY, 0.05); }
    let near = 1.0 - exp(-25.0 * n / ws * kf * kf / n0);
    let wide = 1.0 - exp(-25.0 * nb / wb * kf * kf / n0);
    // The fine cell's centre in the world, and the cells round it.
    let wx = f32(P.ox) + (f32(gid.x) + 0.5) / kf;
    let wy = f32(P.oy) + (f32(gid.y) + 0.5) / kf;
    // Where the wide count says body, its smooth value; towards the
    // frontier, the sharp one; a smooth blend between, so no contour.
    // The body's middle keeps 40% of the sharp count (owner's playtest,
    // 11-14 04:50Z: "The middle still looks blurry"): with the wide count
    // alone there the sheet was a smooth swell with no structure of its own;
    // the sharp share shows its walkers' own clumps and channels, at the
    // fine grid, while the wide share keeps them calm (all-sharp, at four
    // fine cells a cell, was lumpy squares).
    let dense = smoothstep(0.15, 0.45, wide) * 0.6;
    let th = look_trail(wx, wy) * mix(near, wide, dense);
    // The hard terrain a whole cell (`tcell`), from the fine cell at the
    // cell's corner.
    if (gid.x % P.fine == 0u && gid.y % P.fine == 0u) {
        tcell[(gid.y / P.fine) * (P.rw / P.fine) + gid.x / P.fine] = ter_prep(P.ox + i32(gid.x / P.fine), P.oy + i32(gid.y / P.fine));
    }
    // The colour's thickness counts the walkers on their own too (owner,
    // 11-14: lone walkers read dim): an explorer lays little trail, so by
    // the trail alone it was a dark film bead; where walkers are, it is
    // mould and takes the mould's colour. The height keeps `th`.
    let tm = max(th, 0.4 * near);
    if (P.fine == 1u) {
        let ix = i32(floor(wx)); let iy = i32(floor(wy));
        hcell[i] = cell_height(ix, iy, th);
        var m1 = cell_mat(ix, iy, tm, cut_share);
        m1.body = wide;
        mcell[i] = mat_pack(m1);
        return;
    }
    // The ground under it: the four cells round, bilinear (their kinds and
    // colours, this fine cell's body).
    let fx = wx - 0.5; let fy = wy - 0.5;
    let ix = i32(floor(fx)); let iy = i32(floor(fy));
    let tx = fx - floor(fx); let ty = fy - floor(fy);
    let w4 = vec4<f32>((1.0 - tx) * (1.0 - ty), tx * (1.0 - ty), (1.0 - tx) * ty, tx * ty);
    let o = array<vec2<i32>, 4>(vec2<i32>(0, 0), vec2<i32>(1, 0), vec2<i32>(0, 1), vec2<i32>(1, 1));
    var h = 0.0;
    var mt: Mat;
    mt.albedo = vec3<f32>(0.0); mt.glow = vec3<f32>(0.0); mt.gloss = 0.0; mt.body = wide; mt.gem = 0.0;
    let cs = colour_at(wx, wy);
    for (var q = 0; q < 4; q++) {
        let wq = w4[q];
        if (wq <= 0.0) { continue; }
        let cx = ix + o[q].x; let cy = iy + o[q].y;
        h += wq * cell_height(cx, cy, th);
        let mq = cell_mat_c(cx, cy, tm, cut_share, cs);
        mt.albedo += wq * mq.albedo; mt.glow += wq * mq.glow; mt.gloss += wq * mq.gloss;
    }
    hcell[i] = h;
    mcell[i] = mat_pack(mt);
}

fn mat_at(x: f32, y: f32) -> Mat {
    let f = fpos(x, y);
    let fx = f.x; let fy = f.y;
    let ix = i32(floor(fx)); let iy = i32(floor(fy));
    let tx = fx - floor(fx); let ty = fy - floor(fy);
    let a = mc(ix, iy); let b = mc(ix + 1, iy);
    let c = mc(ix, iy + 1); let d = mc(ix + 1, iy + 1);
    var m: Mat;
    m.albedo = mix(mix(a.albedo, b.albedo, tx), mix(c.albedo, d.albedo, tx), ty);
    m.glow = mix(mix(a.glow, b.glow, tx), mix(c.glow, d.glow, tx), ty);
    m.gloss = mix(mix(a.gloss, b.gloss, tx), mix(c.gloss, d.gloss, tx), ty);
    m.body = mix(mix(a.body, b.body, tx), mix(c.body, d.body, tx), ty);
    m.gem = mix(mix(a.gem, b.gem, tx), mix(c.gem, d.gem, tx), ty);
    return m;
}

// The wide walker count under the last `shaded` point, for the specks' light.
var<private> under_body: f32;

// The shaded colour at world point (x, y); `e` is the slope's sample step.
fn shaded(x: f32, y: f32, e: f32) -> vec3<f32> {
    let m = mat_at(x, y);
    under_body = m.body;
    // No wound is drawn (owner, 2026-11-12 04:38Z, on a blind A/B of the
    // old ember burn and a char where walkers died: "Neither. Both of
    // those look like stamps. Placed on the slime that fade"). A hit shows
    // as what it is: the hole in the body (`body`, the living walkers),
    // open until walkers flow back. The engine still keeps the wound byte
    // (`cut`, `set_hits`); a lasting hole is a rule (Larder thread) that
    // will come with its own field.
    // The slope from samples 2e apart (e: 0.75 / `fine` cells): the
    // B-spline's own slope showed the walkers' one-a-cell lattice as a fine
    // plaid in the dense body (2026-11-14); the wider difference smooths it.
    // A crystal's own faces are worked out in its block below.
    let hc0 = cells_height(x, y);
    let h = max(hc0, gem_height(x, y));
    // Forward differences: three B-spline samples, not five.
    let dx = cells_height(x + e, y) - hc0;
    let dy = cells_height(x, y + e) - hc0;
    // The slope capped at 0.7 (2026-11-14, owner: lone walkers looked
    // dim). The sharp count makes a lone walker a bump some 3 cells tall
    // and a cell wide; its flank away from the light shaded to the floor,
    // so explorers on bare ground read as black dots. Capped, the far
    // flank stays at about 0.4 of the light and a vein still has a lit and
    // a shaded side.
    var g = vec2<f32>(dx, dy) / e;
    let gl = length(g);
    if (gl > 0.7) { g *= 0.7 / gl; }
    let n = normalize(vec3<f32>(-g.x, -g.y, 1.0));
    // A crystal stands as a pyramid on its footprint (height from its
    // depth inside it), so a hexagonal crystal has six flat faces. An
    // earlier version turned each slope to the nearest sixth to fake faces
    // on round crystals; the hexagon footprint made that unneeded.
    let l = normalize(LIGHT);
    var shade = 1.0 + 1.25 * (dot(n, l) - l.z);
    // A short cast shadow: something taller toward the light.
    // (Bilinear: the shadow is soft anyway, and it saves a B-spline.)
    let tx = x + l.x * 3.0; let ty = y + l.y * 3.0;
    let toward = max(cells_height_lin(tx, ty), gem_height(tx, ty));
    shade *= 1.0 - 0.35 * clamp((toward - h - 0.3) * 0.5, 0.0, 1.0);
    let half_v = normalize(l + vec3<f32>(0.0, 0.0, 1.0));
    // A wet sheet: the glossier (thicker) the mould, the tighter and
    // brighter its highlight, so a raised vein carries a sharp white glint
    // along its lit flank and a thin film only a faint sheen.
    let spec = pow(max(dot(n, half_v), 0.0), 16.0 + 64.0 * m.gloss) * m.gloss * (0.5 + 0.9 * m.gloss);
    var c = m.albedo * max(shade, 0.15) + vec3<f32>(255.0, 250.0, 236.0) * spec + m.glow;
    // Each point light on this surface: diffuse on its albedo, a highlight
    // by its gloss, falling to nothing at the light's reach. The surface's
    // own height is in `to`, so a tube's flank toward a bullet lights and
    // the far flank stays dark.
    for (var j = 0u; j < P.n_lights; j++) {
        let a = lights[2u * j];
        let lc = lights[2u * j + 1u];
        let to = vec3<f32>(a.x - x, a.y - y, a.w - h);
        let d2 = dot(to.xy, to.xy);
        let rr = a.z * a.z;
        if (d2 >= rr) { continue; }
        let fall = 1.0 - d2 / rr;
        let att = fall * fall * lc.w;
        let ld = normalize(to);
        let diff = max(dot(n, ld), 0.0);
        let hv = normalize(ld + vec3<f32>(0.0, 0.0, 1.0));
        let sp = pow(max(dot(n, hv), 0.0), 24.0) * (0.15 + m.gloss);
        c += (m.albedo * (lc.xyz / 255.0) * diff * 1.6 + lc.xyz * sp * 0.9) * att;
    }
    let k1 = gem_near(x, y);
    if (k1 > 0u) {
        let k = k1 - 1u;
        let gi = gem_in(k, x, y);
        // Antialiased to a pixel: `zoom` pixels a cell.
        let cover = clamp(gi.x * P.zoom + 0.5, 0.0, 1.0);
        let st = gem_state(k, x, y);
        let v = st.x;
        // A cell being taken: white-hot by how fast its charge goes.
        let hot = 1.0 - exp(-0.693 * st.y / GEM_HOT);
        if (cover > 0.0) {
            let a = gems[k].z * 0.8660254;
            let depth = clamp(gi.x / a, 0.0, 1.0);
            // The pyramid's face: height (0.6 + 9 depth) tall(v), so its
            // slope is 9 tall(v) / a along the distance's gradient.
            let s = 9.0 * gem_tall(v) / a;
            let gn = normalize(vec3<f32>(-s * gi.y, -s * gi.z, 1.0));
            var gs = 1.0 + 1.25 * (dot(gn, l) - l.z);
            // Charged, it is glassy cyan lit from inside; spent, matte grey
            // rock (the only cyan in the world is charge).
            let gspec = pow(max(dot(gn, half_v), 0.0), 28.0) * (0.25 + 0.75 * v);
            var alb = mix(vec3<f32>(84.0, 88.0, 92.0), vec3<f32>(30.0, 120.0, 140.0), v);
            let glow = vec3<f32>(10.0, 70.0, 75.0) * v * (0.25 + 0.75 * depth) + vec3<f32>(150.0, 230.0, 255.0) * hot * (0.25 + 0.35 * v);
            // Mould on a crystal grows over it (owner, 11-14 08:15Z: "slime
            // shouldn't go under crystals, they should grow over it but it
            // needs to still clearly look like a crystal"). Before, the
            // hexagon was drawn over the slime with the mould mixed into its
            // albedo at most 70%, lit by the crystal's faces only, so the
            // mould read as sliding underneath. Now the slime there is a
            // film on the faces: its cover follows its real thickness (the
            // inverse of the cells' height curve), at most `GEM_FILM`, so a
            // thin film shows the faces through and a feeding mat covers
            // more; it is lit by the face it lies on with a little of its
            // own relief, so the six faces and their edges still read; the
            // crystal's glow shines through it, weakened by the cover; and
            // a mould that is draining it (`hot`, the same charge loss the
            // harvest glow shows) takes a cyan cast.
            let th = -log(1.0 - clamp(cells_height(x, y) * 0.2, 0.0, 0.999) * (1.0 - exp(-2.6))) / 2.6;
            let film = GEM_FILM * (1.0 - exp(-25.0 * th));
            let rock = alb * max(gs, 0.15) + vec3<f32>(255.0, 248.0, 230.0) * gspec * 0.7;
            // The film is the slime as shaded off the crystal (its own
            // bumps, gloss and glint), tilted by the face it lies on.
            let lum = dot(c, vec3<f32>(0.299, 0.587, 0.114));
            let fed = mix(c, vec3<f32>(0.45, 1.05, 1.2) * lum, clamp(0.35 * hot, 0.0, 0.35));
            let on = fed * clamp(0.3 + 0.7 * gs, 0.3, 1.5);
            // The silhouette stays crisp under the film: a dark line a pixel
            // and a half wide inside the hexagon's edge, where the faces
            // meet the ground (a hard shape's edge, art direction rule 4).
            let rim = 0.72 + 0.28 * smoothstep(0.0, 1.5, gi.x * P.zoom);
            // The charge's light comes up through the film, brightest deep in a
            // full crystal, so a covered crystal still glows cyan.
            let through = glow * (1.0 - 0.3 * film) + vec3<f32>(20.0, 95.0, 115.0) * v * depth * film;
            let gc = (mix(rock, on, film) + through) * rim;
            c = mix(c, gc, cover);
        }
        if (cover < 1.0 && hot > 0.0) {
            // Where its rim is being taken, the light spills a little onto
            // the ground and whatever stands there (half a cell).
            c += vec3<f32>(120.0, 220.0, 255.0) * hot * 0.6 * exp(min(gi.x, 0.0) / 0.5) * (1.0 - cover);
        }
    }
    return c;
}

// ---- Hard terrain: rock, walls, rubble ----
//
// The simulation's rock and walls are whole cells, and drawn as cells
// they stepped every `zoom` pixels with a dark band a cell wide (the art
// director's review, 2026-11-14: "drawn from the cell grid"). Here the
// cells' occupancy is a smooth field, a cubic B-spline over the 4 x 4
// cells round the point (`bc`: C2, so its outline has no corners a cell apart),
// and the edge is where it crosses `TER_ISO`, worked out at the screen
// pixel: the coverage is the signed distance to that crossing (the field
// over its gradient) in pixels, so every zoom gets a one-pixel
// antialiased edge. The B-spline of a straight run of cells crosses 0.5
// on the cells' boundary; a lone cell or a one-cell diagonal comes out
// rounder and thinner, so the level is 0.4, which puts a straight edge
// about a tenth of a cell outside the cells. The height (rock 3, a wall by its health,
// rubble low) and colour are the weighted means of the solid cells, so a
// worn wall still sinks and dims cell by cell. A short soft shadow falls
// away from the light; the old cast shadow came from cell heights three
// cells toward the light and read as a black band.
const TER_ISO: f32 = 0.4;
// How far in from the edge the top bevels (cells), and the shadow's reach
// a cell of the terrain's height.
const TER_BEVEL: f32 = 0.9;
const TER_SHADOW: f32 = 0.35;

// A cell's own terrain, packed as `tcell` holds it (byte 0 left 255 for
// solid, 0 open; `prep` smooths it). Off the world is rock, as before.
fn ter_pack(ix: i32, iy: i32) -> u32 {
    if (ix < 0 || iy < 0 || ix >= i32(P.ww) || iy >= i32(P.wh)) { return 255u | (191u << 8u) | (ROCK << 16u); }
    let g = ground[cidx(ix, iy)];
    let kind = g & 15u;
    let v = byte(g, 1u);
    var h = 0.0;
    var val = v;
    if (kind == ROCK) { h = 3.0; val = byte(g, 3u); }
    else if (kind == WALL) { h = 0.6 + 2.6 * v; }
    else if (kind == RUBBLE) { h = 0.2 + 0.6 * v; }
    else { return 0u; }
    return 255u | (u32(h / 4.0 * 255.0 + 0.5) << 8u) | (kind << 16u) | (u32(val * 255.0 + 0.5) << 24u);
}

// The smoothed terrain cell for `prep`: its own kind, height and value, and
// the 3 x 3 occupancy round it weighted 1 2 1 (x) 1 2 1. With the cubic
// spline over these, a straight edge stays on the cells' boundary and the
// maps' 2-cell staircases (their rock is drawn on the coarse grid) smooth
// out; the spline alone left a notch every step at zoom 12 and 24, and
// working the smoothing out a pixel at a time (four splines at +-0.75
// cells) cost 7 ms a frame more on llvmpipe at 1280 x 800 (2026-11-14).
fn ter_prep(ix: i32, iy: i32) -> u32 {
    var occ = 0u;
    for (var j = -1; j <= 1; j++) {
        for (var i = -1; i <= 1; i++) {
            if ((ter_pack(ix + i, iy + j) & 255u) != 0u) { occ += (2u - u32(abs(i))) * (2u - u32(abs(j))); }
        }
    }
    let own = ter_pack(ix, iy);
    return (own & 0xffffff00u) | ((occ * 255u + 8u) / 16u);
}

// A whole cell's place in `tcell` (clamped to `prep`'s rectangle).
fn tidx(ix: i32, iy: i32) -> u32 {
    let w = i32(P.rw / P.fine);
    let x = clamp(ix - P.ox, 0, w - 1);
    let y = clamp(iy - P.oy, 0, i32(P.rh / P.fine) - 1);
    return u32(y * w + x);
}

// A shown cell's terrain from its `tcell` word: colour and height (0: open).
fn ter_cell(p: u32) -> vec4<f32> {
    let kind = (p >> 16u) & 255u;
    let v = byte(p, 3u);
    let h = byte(p, 1u) * 4.0;
    if (kind == ROCK) { return vec4<f32>(mix(vec3<f32>(70.0, 72.0, 76.0), vec3<f32>(118.0, 116.0, 110.0), v), h); }
    // Health is the wall's brightness as well as its height.
    if (kind == WALL) { return vec4<f32>(mix(vec3<f32>(70.0, 58.0, 48.0), vec3<f32>(168.0, 156.0, 132.0), v), h); }
    if (kind == RUBBLE) { return vec4<f32>(vec3<f32>(92.0, 82.0, 70.0) * (0.7 + 0.3 * v), h); }
    return vec4<f32>(0.0);
}

// Cubic B-spline weights (cells -1..2 round floor(x - 0.5)) and their
// derivatives, at t = (x - 0.5) - floor(x - 0.5). The quadratic spline (3 x 3
// cells) left a diagonal ridge's staircase as a scalloped edge at zoom 12
// (2026-11-14); the cubic's 4 x 4 smooths it to a ripple under a pixel.
fn bc(t: f32) -> vec4<f32> {
    let s = 1.0 - t;
    return vec4<f32>(s * s * s, 3.0 * t * t * t - 6.0 * t * t + 4.0, -3.0 * t * t * t + 3.0 * t * t + 3.0 * t + 1.0, t * t * t) / 6.0;
}
fn bcd(t: f32) -> vec4<f32> {
    let s = 1.0 - t;
    return vec4<f32>(-s * s, 3.0 * t * t - 4.0 * t, -3.0 * t * t + 2.0 * t + 1.0, t * t) * 0.5;
}

struct Ter { s: f32, grad: vec2<f32>, h: f32, albedo: vec3<f32> };

// The terrain field at world point (x, y): occupancy, its gradient (per
// cell), and the solid cells' mean height and colour.
fn terrain(x: f32, y: f32) -> Ter {
    var t: Ter;
    t.s = 0.0; t.grad = vec2<f32>(0.0); t.h = 0.0; t.albedo = vec3<f32>(0.0);
    var hw = 0.0;
    let ux = x - 0.5; let uy = y - 0.5;
    let ix = i32(floor(ux)); let iy = i32(floor(uy));
    let fx = ux - floor(ux); let fy = uy - floor(uy);
    let wx = bc(fx); let wy = bc(fy);
    let dx = bcd(fx); let dy = bcd(fy);
    for (var j = 0; j < 4; j++) {
        for (var i = 0; i < 4; i++) {
            let p = tcell[tidx(ix - 1 + i, iy - 1 + j)];
            let o = byte(p, 0u);
            if (o > 0.0) {
                let w = wx[i] * wy[j];
                t.s += w * o;
                t.grad += o * vec2<f32>(dx[i] * wy[j], wx[i] * dy[j]);
                // Height and colour from the solid cells themselves.
                let c = ter_cell(p);
                if (c.w > 0.0) { t.h += w * c.w; t.albedo += w * c.xyz; hw += w; }
            }
        }
    }
    if (hw > 0.0) { t.h /= hw; t.albedo /= hw; } else { t.h = 0.0; }
    return t;
}

// The ground's colour `c` at world point (x, y) with the hard terrain over
// it: the shadow it casts, then its top, lit, with an antialiased edge.
fn terrain_over(c0: vec3<f32>, x: f32, y: f32) -> vec3<f32> {
    // No hard terrain within reach of this cell (`look::FAR`): nothing to
    // draw, at the cost of one read.
    if ((ground[cidx(i32(floor(x)), i32(floor(y)))] & 128u) != 0u) { return c0; }
    let l = normalize(LIGHT);
    let t = terrain(x, y);
    var cov = 0.0;
    var d = 0.0;
    let gl = max(length(t.grad), 1e-4);
    if (t.s > 0.0) {
        // Signed distance to the edge in cells (inside positive), and the
        // pixel's coverage.
        d = (t.s - TER_ISO) / gl;
        cov = clamp(0.5 + d * P.zoom, 0.0, 1.0);
    }
    var c = c0;
    if (cov < 1.0) {
        // The shadow on the ground: the terrain a short way toward the
        // light, as far as its height casts; soft over about a cell.
        let ld = normalize(l.xy);
        let q = terrain(x + ld.x * 1.2, y + ld.y * 1.2);
        if (q.s > 0.0) {
            let reach = clamp(q.h * TER_SHADOW, 0.0, 1.0);
            c *= 1.0 - 0.45 * reach * smoothstep(0.05, 0.6, q.s);
        }
    }
    if (cov <= 0.0) { return c; }
    // The top: flat, beveled down to the edge within TER_BEVEL, the bevel
    // a little steeper the taller the terrain.
    let inward = t.grad / gl;
    let k = 1.0 - smoothstep(0.0, TER_BEVEL, d);
    let n = normalize(vec3<f32>(-inward * k * (0.35 + 0.15 * t.h), 1.0));
    let shade = 1.0 + 1.25 * (dot(n, l) - l.z);
    let half_v = normalize(l + vec3<f32>(0.0, 0.0, 1.0));
    let spec = pow(max(dot(n, half_v), 0.0), 20.0) * 0.08;
    let top = t.albedo * max(shade, 0.15) + vec3<f32>(255.0, 248.0, 230.0) * spec;
    return mix(c, top, cov);
}

// Every walker a speck of light at its own position (one thread a walker),
// counted a screen pixel; `base` adds and clears them.
@compute @workgroup_size(256)
fn draw_specks(@builtin(global_invocation_id) gid: vec3<u32>) {
    let k = gid.x;
    if (k >= P.n_walk) { return; }
    if (P.lives != 0u && alive[k] < 0.0) { return; }
    let w = walkers[k];
    if (!(w.x >= 0.0 && w.y >= 0.0 && w.x < f32(P.ww) && w.y < f32(P.wh))) { return; }
    // A speck a walker, coloured by its news of food (C8, 2026-11-14): calm
    // (bits 0-9), leaning (10-19), bolting (20-31), the phases `panic_to`
    // in the engine steers by; it also counts the cut-off walkers a cell
    // for `prep`'s ashen fade. Each is a ramp of the walker's news age in
    // fifteenths, not a step at the lag (art director, 2026-11-14: a whole
    // piece crosses the lag within a tick or two, so a step snapped it from
    // gold to ash, then to red, at once). The ash runs from a third of the
    // lean before the lag to 4/3 lean past it (-1 s to +2 s at the 5 s lag
    // and 1.5 s lean), amber comes in over a third of the lean each side of
    // the lag, and red over a third each side of the lean's end; the fine
    // grid counts a walker 15 so its ash share keeps the ramp's steps.
    var lean = 0u;
    var bolt = 0u;
    var cnt = 15u;
    var cut = 0.0;
    if (P.news_on == 1u) {
        let heard = news[k];
        let now = state[0] + 1u;
        let age = f32(now - min(heard, now));
        let lag = f32(P.lag);
        let ln = max(f32(P.lean), 1.0);
        let a = smoothstep(lag - ln * 2.0 / 3.0, lag + ln * 4.0 / 3.0, age);
        let l = smoothstep(lag - ln / 3.0, lag + ln / 3.0, age);
        let b = smoothstep(lag + ln * 2.0 / 3.0, lag + ln * 4.0 / 3.0, age);
        cnt = 15u | (u32(round(a * 15.0)) << 16u);
        lean = u32(round(l * (1.0 - b) * 15.0));
        bolt = u32(round(b * 15.0));
        cut = l;
    }
    let add = select(1u, 0u, cut > 0.5) | (lean << 10u) | (bolt << 20u);
    // The walker in its fine cell for `prep`'s body (and, above bit 16,
    // counted cut off for its ashen fade); before the screen test, as
    // `prep` reads a rim past it.
    let kf = f32(P.fine);
    let gx = i32(floor((w.x - f32(P.ox)) * kf));
    let gy = i32(floor((w.y - f32(P.oy)) * kf));
    if (gx >= 0 && gy >= 0 && gx < i32(P.rw) && gy < i32(P.rh)) { atomicAdd(&fgrid[u32(gy) * P.rw + u32(gx)], cnt); }
    let s = vec2<f32>((w.x - P.cx) * P.zoom, (w.y - P.cy) * P.zoom);
    let px = i32(floor(s.x)); let py = i32(floor(s.y));
    if (px < 0 || py < 0 || px >= i32(P.sw) || py >= i32(P.sh)) { return; }
    // A walker an explosion threw (the engine's `fling`, 11-14) has 1 in
    // its fourth word while it flies: a hot speck, a pixel more each side
    // per 4 of zoom, so the throw shows as the mould itself in the air
    // and not only as the streak it lays. It counts 6 calm walkers a pixel
    // (the calm bits, 0-9, so it never reads as cut off).
    if (w.w > 0.5) {
        let r = i32(P.zoom / 4.0);
        for (var dy = -r; dy <= r; dy++) {
            for (var dx = -r; dx <= r; dx++) {
                let qx = px + dx; let qy = py + dy;
                if (qx < 0 || qy < 0 || qx >= i32(P.sw) || qy >= i32(P.sh)) { continue; }
                atomicAdd(&specks[u32(qy) * P.stride + u32(qx)], 6u);
            }
        }
        return;
    }
    // A cut-off walker's speck grows with the zoom (a pixel a walker was
    // lost at zoom 6: 195 of them in a cut piece did not read), up to 4 x 4
    // pixels.
    // Every speck grows with the zoom, to zoom / 3 pixels up to 4 (calm
    // ones too since the owner's playtest, 11-14: lone walkers read dim);
    // from 3 pixels its corners are left off, so it is a dot, not a square.
    let d = clamp(i32(round(P.zoom / 3.0)), 1, 4);
    for (var dy = 0; dy < d; dy++) {
        for (var dx = 0; dx < d; dx++) {
            if (d >= 3 && (dx == 0 || dx == d - 1) && (dy == 0 || dy == d - 1)) { continue; }
            let qx = px + dx - d / 2; let qy = py + dy - d / 2;
            if (qx >= 0 && qy >= 0 && qx < i32(P.sw) && qy < i32(P.sh)) { atomicAdd(&specks[u32(qy) * P.stride + u32(qx)], add); }
        }
    }
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
            c = shaded(fx, fy, 0.75 / f32(P.fine));
            c = terrain_over(c, fx, fy);
            let n = atomicExchange(&specks[y * P.stride + x], 0u);
            // Each calm walker a point of light (owner's playtest, 11-14
            // 04:50Z: "single individual walkers should be brighter"). It
            // was 0.048 of white added a walker, so a lone explorer on bare
            // ground was a faint pixel. Now its light is a share of the way
            // to the speck colour that falls with the square of the walkers
            // a cell round it (from the wide count, `prep`): about 0.5 for a
            // walker alone (0.13 a cell under a 1.1-cell Gaussian), under
            // 0.02 in the body (1 a cell and up). A share falling linearly
            // with the wide count left the body's middle a starfield. So a
            // lone walker reads plainly and the dense body keeps a calm,
            // moving grain (its flow) without going white.
            var n0 = BODY_N;
            if (P.body_density > 0.0) { n0 = BODY_N * max(P.body_density / BODY_DENSITY, 0.05); }
            let dens = -log(max(1.0 - under_body, 0.004)) * n0 / 25.0;
            let per = 0.95 / (1.0 + (dens / 0.35) * (dens / 0.35));
            let lit = 1.0 - pow(1.0 - per, f32(n & 1023u));
            c = mix(c, vec3<f32>(255.0, 222.0, 130.0), lit * 0.92);
            // Cut-off walkers' specks take their phase's colour outright
            // (added light was lost on the bright body: no pixel of a
            // bolting piece read red on the first film).
            c = mix(c, vec3<f32>(255.0, 178.0, 40.0), min(f32((n >> 10u) & 1023u) * (0.4 / 15.0), 0.75));
            c = mix(c, vec3<f32>(235.0, 52.0, 40.0), min(f32(n >> 20u) * (0.5 / 15.0), 0.85));
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
const BUILDING: u32 = 8u;
const STREAK: u32 = 9u;

// ---- The soldier ----
//
// First drawn from shapes in the shader (shadow, pack, shoulders, arms,
// rifle, helmet, lit by the world's light). The owner compared it blind
// against the same design as an SVG sprite and picked the sprite (A,
// 2026-11-11 16:48Z), so the figure is now the sprite atlas; the shapes'
// code is in git history (commit 105c358). The shadow stays a shape here.

fn sd_ellipse(p: vec2<f32>, c: vec2<f32>, r: vec2<f32>) -> f32 {
    let q = (p - c) / r;
    return (length(q) - 1.0) * min(r.x, r.y);
}

fn cover(sd: f32, aa: f32) -> f32 { return clamp(0.5 - sd / aa, 0.0, 1.0); }

// One texel of the atlas, premultiplied.
fn texel(x: i32, y: i32) -> vec4<f32> {
    let c = atlas[u32(clamp(y, 0, 159)) * 2048u + u32(clamp(x, 0, 2047))];
    let a = f32(c >> 24u) / 255.0;
    return vec4<f32>(unpack(c) * a, a);
}

// Tile `k` (0 body, 1 helmet, 2 flash, 3 dead; 4-15 the buildings, each
// body then lamp: turret, lure, pump, outpost, siren, larder) at point `p` in radii
// (-2..2 across the tile), bilinear, from the 128 px or the 32 px row.
fn sprite(k: u32, p: vec2<f32>, small: bool) -> vec4<f32> {
    var size = 128.0; var oy = 0.0;
    if (small) { size = 32.0; oy = 128.0; }
    let u = (p.x + 2.0) / 4.0 * size - 0.5;
    let v = (p.y + 2.0) / 4.0 * size - 0.5;
    if (u < -1.0 || v < -1.0 || u > size || v > size) { return vec4<f32>(0.0); }
    let ox = f32(k) * size;
    let x0 = floor(u); let y0 = floor(v);
    let tx = u - x0; let ty = v - y0;
    let lo = size - 1.0;
    let ix0 = i32(ox + clamp(x0, 0.0, lo)); let ix1 = i32(ox + clamp(x0 + 1.0, 0.0, lo));
    let iy0 = i32(oy + clamp(y0, 0.0, lo)); let iy1 = i32(oy + clamp(y0 + 1.0, 0.0, lo));
    let inside = vec4<f32>(f32(x0 >= 0.0 && y0 >= 0.0), f32(x0 + 1.0 <= lo && y0 >= 0.0), f32(x0 >= 0.0 && y0 + 1.0 <= lo), f32(x0 + 1.0 <= lo && y0 + 1.0 <= lo));
    let a = mix(texel(ix0, iy0) * inside.x, texel(ix1, iy0) * inside.y, tx);
    let b = mix(texel(ix0, iy1) * inside.z, texel(ix1, iy1) * inside.w, tx);
    return mix(a, b, ty);
}

// Premultiplied `top` over premultiplied `under`.
fn pm_over(under: vec4<f32>, top: vec4<f32>) -> vec4<f32> {
    return top + under * (1.0 - top.w);
}

// The soldier: the owner's pick of an SVG sprite (2026-11-11 16:48Z, A over
// the shader-drawn figure of the first look), rotated to its facing, with
// the helmet tinted the team's colour, the muzzle flash while `flash` > 0
// and the dead pose; the shadow is cast in screen space by the world's
// light. Returns straight colour and alpha.
//
// Read at zoom 6 (art review 2026-11-14, rule 6: an olive figure on ochre
// slime was lost): a living soldier has a dark outline a constant
// OUTLINE pixels wide round its silhouette (body and helmet), which
// parts it from the gold slime, and a cream rim light on the edges that
// face the world's light, which parts it from the dark ground. The dead
// get neither and no shadow: a flat grey shape lying on the ground.
const OUTLINE: f32 = 1.5;
// Sprite colours are 0-255, as the atlas's texels (`texel`).
const RIM: vec3<f32> = vec3<f32>(250.0, 244.0, 222.0);
const INK: vec3<f32> = vec3<f32>(14.0, 13.0, 10.0);

// The living soldier's silhouette (body and helmet) at `p`, 0-1.
fn silhouette(p: vec2<f32>, small: bool) -> f32 {
    return max(sprite(0u, p, small).w, sprite(1u, p, small).w);
}

// Harvesting (art direction, the owner's playtest 11-14 04:53Z:
// "harvesting crystals could look way better"): `carry` 0-1 is the share
// of a full load the soldier holds, shown as cyan charge filling the pack
// on its back from the bottom (sprite frame) with a glow round it that
// grows with the load, so a carrier walking home is a light you can
// follow; `take` 0-1 is a pulse at its hands while it takes from a crystal
// this moment (`Game::last_takes`). Both are game state, not decoration.
// The pack is the body tile's rectangle at x -0.72..-0.30, y -0.42..0.42.
const CHARGE: vec3<f32> = vec3<f32>(92.0, 224.0, 232.0);
const CHARGE_HOT: vec3<f32> = vec3<f32>(220.0, 255.0, 255.0);

fn soldier(d: vec2<f32>, r: f32, ang: f32, team: vec3<f32>, flash: f32, dead: bool, carry: f32, take: f32) -> vec4<f32> {
    let aa = 1.0 / max(r, 1.0);
    let l = normalize(LIGHT);
    let ca = cos(ang); let sa = sin(ang);
    let s = d / r;
    let p = vec2<f32>(s.x * ca + s.y * sa, -s.x * sa + s.y * ca);
    // Under 12 px across the radius the 32 px tiles, so a small soldier is
    // averaged rather than aliased.
    let small = r < 12.0;
    if (dead) {
        let c = sprite(3u, p, small);
        if (c.w <= 0.0) { return vec4<f32>(0.0); }
        return vec4<f32>(c.xyz / c.w, c.w * 0.92);
    }
    let sh = sd_ellipse(s, vec2<f32>(-l.x, -l.y) * 0.32, vec2<f32>(0.78, 0.78));
    var out = vec4<f32>(0.0, 0.0, 0.0, 0.38 * cover(sh, aa * 2.5));
    // The outline: the silhouette grown by OUTLINE pixels (eight taps, in
    // the sprite's own frame so it turns with the figure).
    let o = OUTLINE / r;
    var grown = 0.0;
    for (var i = 0u; i < 8u; i++) {
        let a = f32(i) * 0.785398;
        grown = max(grown, silhouette(p + vec2<f32>(cos(a), sin(a)) * o, small));
    }
    out = pm_over(out, vec4<f32>(INK * grown, grown));
    let body = sprite(0u, p, small);
    out = pm_over(out, body);
    let h = sprite(1u, p, small);
    out = pm_over(out, vec4<f32>(h.xyz * team / 255.0, h.w));
    // The rim: inside the silhouette, where one pixel toward the light
    // (screen space, turned into the sprite's frame) is outside it.
    let ls = vec2<f32>(l.x, l.y) / r;
    let lp = vec2<f32>(ls.x * ca + ls.y * sa, -ls.x * sa + ls.y * ca);
    let inside = max(body.w, h.w);
    let rim = clamp(inside - silhouette(p + lp * 1.2, small), 0.0, 1.0) * 0.75;
    out = vec4<f32>(mix(out.xyz, RIM * out.w, rim), out.w);
    if (carry > 0.0) {
        // The pack's charge, filled from its foot (+y) up, a hot core
        // where it is full.
        let inpack = p.x > -0.68 && p.x < -0.34 && abs(p.y) < 0.38;
        let level = 0.38 - 0.76 * carry;
        if (inpack && p.y >= level) {
            let c = mix(CHARGE, CHARGE_HOT, clamp(carry * 1.2 - 0.2, 0.0, 1.0) * 0.6);
            out = vec4<f32>(c, 1.0);
        }
        // The glow: light, so it adds; its reach grows with the load. Kept
        // inside the body's edge: each screen pixel shows one mark (the
        // highest that claims it), so a wide glow claimed a neighbour's
        // pixels and cut a dark bite out of it (first try, 0.06 + 0.12 *
        // carry, at the zoom 12 film).
        let q = vec2<f32>(clamp(p.x, -0.68, -0.34), clamp(p.y, -0.38, 0.38));
        let dd = length(p - q);
        let g = carry * exp(-dd * dd / (0.02 + 0.04 * carry)) * 0.6;
        out = vec4<f32>(out.xyz + CHARGE * g, min(out.w + g, 1.0));
    }
    if (take > 0.0) {
        // A pulse at the hands as charge comes in.
        let dh = length(p - vec2<f32>(0.66, 0.3));
        let g = take * exp(-dh * dh / 0.03);
        out = vec4<f32>(out.xyz + CHARGE_HOT * g, min(out.w + g, 1.0));
    }
    if (flash > 0.0) {
        let f = sprite(2u, p, small) * clamp(flash * 1.5, 0.0, 1.0);
        out = pm_over(out, f);
    }
    if (out.w <= 0.0) { return vec4<f32>(0.0); }
    return vec4<f32>(out.xyz / out.w, out.w);
}

// A building (`Marks::building`, the sprite atlas's tiles 4-15, drawn from
// src/world/sprites/buildings.svg in soldier A's style), with the soldier's
// cast shadow. What changes on it is the game's state, nothing else: it
// rises from its foot as it is built (the unbuilt part a faint ghost), it
// darkens and greys as its health goes, and its lamp is lit only while it
// works (a turret firing, a lure laying scent, a pump drawing charge, a
// siren while it is on).
// `pulse` 0-1 brightens the lamp for a moment: the larder's charge cells
// as a carrier banks its load (harvest look, 11-14).
fn building(d: vec2<f32>, r: f32, kind: u32, built: f32, health: f32, working: bool, pulse: f32) -> vec4<f32> {
    let aa = 1.0 / max(r, 1.0);
    let l = normalize(LIGHT);
    let p = d / r;
    let small = r < 12.0;
    let k = 4u + 2u * min(kind, 5u);
    var body = sprite(k, p, small);
    // The silhouette's edge, OUTLINE pixels out (eight taps): a dark
    // outline round a standing building, as round a soldier, so it parts
    // from the slime (art review 2026-11-14), and the cream line of a
    // ghost's plan.
    let o = OUTLINE / r;
    var grown = 0.0;
    for (var i = 0u; i < 8u; i++) {
        let a = f32(i) * 0.785398;
        grown = max(grown, sprite(k, p + vec2<f32>(cos(a), sin(a)) * o, small).w);
    }
    let edge = clamp(grown - body.w, 0.0, 1.0);
    // Built from the foot (screen down) up (owner's playtest 2026-11-12:
    // "a faint building ... that fills in as it builds"). Above the line
    // the plan: the body faint and its outline cream, so a ghost reads on
    // the dark ground too (11-14: a ghost's faint dark body was lost
    // there). Below it the risen part at nearly full colour, with a cream
    // seam where it is rising, so the progress is in the sprite itself.
    let line = 1.1 - 2.2 * built;
    let up = p.y >= line;
    if (built < 1.0) {
        if (!up) { body = body * 0.22; } else { body = body * 0.85; }
    }
    // Hurt (art review 2026-11-14: buildings show their state in the
    // sprite): darker and greyer with the health gone, and scorched, a
    // share of its texels charred about half the health lost.
    let grey = dot(body.xyz, vec3<f32>(0.3, 0.59, 0.11));
    // A ghost's low health is its progress, shown by the seam instead.
    let hurt = select(0.0, 1.0 - health, built >= 1.0);
    body = vec4<f32>(mix(body.xyz, vec3<f32>(grey), 0.6 * hurt) * (1.0 - 0.45 * hurt), body.w);
    let cellp = floor(p * 9.0);
    let n = fract(sin(dot(cellp, vec2<f32>(12.9898, 78.233))) * 43758.5453);
    if (n < 0.55 * hurt) { body = vec4<f32>(body.xyz * 0.3, body.w); }
    let sh = sd_ellipse(p, vec2<f32>(-l.x, -l.y) * 0.3, vec2<f32>(1.0, 1.0));
    var out = vec4<f32>(0.0, 0.0, 0.0, 0.32 * cover(sh, aa * 2.5) * max(built, 0.3));
    if (built >= 1.0) {
        out = pm_over(out, vec4<f32>(INK * edge, edge));
    } else if (!up) {
        out = pm_over(out, vec4<f32>(RIM * edge * 0.7, edge * 0.7));
    } else {
        out = pm_over(out, vec4<f32>(INK * edge, edge));
    }
    out = pm_over(out, body);
    if (built < 1.0) {
        let seam = clamp(1.0 - abs(p.y - line) * r / 1.2, 0.0, 1.0) * max(body.w, edge);
        out = pm_over(out, vec4<f32>(RIM * seam, seam));
    }
    if (working && built >= 1.0) { out = pm_over(out, sprite(k + 1u, p, small)); }
    if (pulse > 0.0 && built >= 1.0) {
        let lamp = sprite(k + 1u, p, small);
        let g = pulse * 1.5;
        out = vec4<f32>(out.xyz + lamp.xyz * g, min(out.w + lamp.w * g, 1.0));
        // A halo round the lit cells.
        let lit = sprite(k + 1u, p * 0.8, true).w;
        out = vec4<f32>(out.xyz + CHARGE * lit * pulse * 0.5, min(out.w + lit * pulse * 0.5, 1.0));
        // The whole store washed in charge light, on the body only (no
        // new alpha, so it claims no neighbour's pixels): the lit cells
        // alone were a few pixels at zoom 12 and lost under the carriers
        // walking in (first film, 11-14).
        let wash = pulse * exp(-dot(p, p) / 0.6) * 0.45;
        out = vec4<f32>(out.xyz + CHARGE * wash * out.w, out.w);
    }
    if (out.w <= 0.0) { return vec4<f32>(0.0); }
    return vec4<f32>(out.xyz / out.w, out.w);
}

fn building_at(b: u32, d: vec2<f32>, r: f32) -> vec4<f32> {
    let extra = marks[b + 11u];
    return building(d, r, u32(bitcast<f32>(marks[b + 9u])), f32(extra & 255u) / 255.0, f32((extra >> 8u) & 255u) / 255.0, (extra & 65536u) != 0u, f32((extra >> 17u) & 255u) / 255.0);
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

// Each mark gets SPLIT workgroups (wg.y), which share its box (or line)
// between them. One 64-thread workgroup a mark walked a big mark's whole
// box alone: one 480-pixel ring cost the view 0.7 ms on the GTX 1070 (the
// camera thread, 2026-11-11). Small marks leave the extra groups idle.
const SPLIT: u32 = 16u;

@compute @workgroup_size(64)
fn draw_marks(@builtin(workgroup_id) wg: vec3<u32>, @builtin(local_invocation_index) li: u32) {
    let k = wg.x;
    let t0 = li + 64u * wg.y;
    if (k >= P.n_marks) { return; }
    let b = k * 12u;
    let s = anchor(b);
    let shape = marks[b + 6u];
    let col2 = marks[b + 8u];
    let fill = bitcast<f32>(marks[b + 9u]);
    let flags = marks[b + 10u];
    let extra = marks[b + 11u];
    // A dot on the minimap: four threads, a 2 x 2 dot.
    if ((flags & 2u) != 0u && P.mw > 0u && wg.y == 0u && li < 4u) {
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
        for (var i = t0; i <= n; i += 64u * SPLIT) {
            if (dash > 0u && (i / dash) % 2u == 1u) { continue; }
            let p = mix(s, e, f32(i) / f32(max(n, 1u)));
            claim(i32(floor(p.x)), i32(floor(p.y)), k, false);
        }
        return;
    }
    if (shape == STREAK) {
        let e = to_screen(bitcast<f32>(marks[b + 4u]), bitcast<f32>(marks[b + 5u]));
        let rr = 3.0 * fill;
        let lo = max(min(s, e) - vec2<f32>(rr), vec2<f32>(0.0));
        let hi = min(max(s, e) + vec2<f32>(rr), vec2<f32>(f32(P.sw) - 1.0, f32(P.sh) - 1.0));
        if (hi.x < lo.x || hi.y < lo.y) { return; }
        let w = u32(hi.x - lo.x) + 1u;
        let total = w * (u32(hi.y - lo.y) + 1u);
        for (var j = t0; j < total; j += 64u * SPLIT) {
            let q = floor(lo) + vec2<f32>(f32(j % w), f32(j / w));
            if (streak_at(b, q + vec2<f32>(0.5)).x > 0.004) { claim(i32(q.x), i32(q.y), k, false); }
        }
        return;
    }
    let h = half(b);
    var x0 = floor(s.x - h.x);
    var y0 = floor(s.y - h.y);
    var x1 = ceil(s.x + h.x);
    var y1 = ceil(s.y + h.y);
    if (shape == SOLDIER || shape == BUILDING) {
        // Rifle, barrels and flash reach past the radius.
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
    for (var j = t0; j < total; j += 64u * SPLIT) {
        let qx = x0 + f32(j % w);
        let qy = y0 + f32(j / w);
        let d = vec2<f32>(qx + 0.5, qy + 0.5) - s;
        var on = false;
        if (shape == DISC) {
            on = length(d) <= h.x;
        } else if (shape == RING) {
            // `fill` pixels wide (`Marks::ring_thin`), else 1.5.
            let r = length(d);
            var wide = 1.5;
            if (fill > 0.0) { wide = fill; }
            on = r <= h.x && r >= h.x - wide;
        } else if (shape == RECT || shape == BAR) {
            on = abs(d.x) <= h.x && abs(d.y) <= h.y;
        } else if (shape == FRAME) {
            on = abs(d.x) <= h.x + 0.5 && abs(d.y) <= h.y + 0.5 && (abs(d.x) >= h.x - 0.5 || abs(d.y) >= h.y - 0.5);
        } else if (shape == SOLDIER) {
            on = soldier_at(b, d, h.x).w > 0.004;
        } else if (shape == BUILDING) {
            on = building_at(b, d, h.x).w > 0.004;
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

// A streak (`Marks::streak`): something bright in flight drawn along the
// way it moved this frame, as Noita draws a fast particle along its
// velocity (`draw_as_long`). The anchor is its head, (hw, hh) its tail in
// world cells, `fill` its core's radius in pixels. Returns (glow, core) at
// screen point p: a halo three cores wide and a hot core, both brightest
// at the head and gone at the tail, added to the world (light, not paint).
fn streak_at(b: u32, p: vec2<f32>) -> vec2<f32> {
    let head = anchor(b);
    let tail = to_screen(bitcast<f32>(marks[b + 4u]), bitcast<f32>(marks[b + 5u]));
    let r = max(bitcast<f32>(marks[b + 9u]), 0.5);
    let ab = head - tail;
    let l2 = max(dot(ab, ab), 1e-6);
    let t = clamp(dot(p - tail, ab) / l2, 0.0, 1.0);
    let d = distance(p, tail + ab * t);
    let along = t * t;
    let halo = exp(-(d * d) / (2.0 * r * r * 2.2)) * along;
    let core = clamp(r - d + 0.5, 0.0, 1.0) * mix(0.35, 1.0, along);
    return vec2<f32>(select(halo, 0.0, d > 3.0 * r), core);
}

fn soldier_at(b: u32, d: vec2<f32>, r: f32) -> vec4<f32> {
    let extra = marks[b + 11u];
    return soldier(d, r, bitcast<f32>(marks[b + 9u]), unpack(marks[b + 7u]), f32(extra & 255u) / 255.0, (extra & 256u) != 0u, f32((extra >> 9u) & 255u) / 255.0, f32((extra >> 17u) & 255u) / 255.0);
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
        if (marks[b + 6u] == STREAK && !in_minimap(x, y)) {
            let g = streak_at(b, vec2<f32>(f32(x) + 0.5, f32(y) + 0.5));
            let a = f32(col >> 24u) / 255.0;
            c = c + unpack(col) * g.x * a + unpack(marks[b + 8u]) * g.y;
        } else if (marks[b + 6u] == SOLDIER && !in_minimap(x, y)) {
            let d = vec2<f32>(f32(x) + 0.5, f32(y) + 0.5) - anchor(b);
            let f = soldier_at(b, d, half(b).x);
            c = c * (1.0 - f.w) + f.xyz * f.w;
        } else if (marks[b + 6u] == BUILDING && !in_minimap(x, y)) {
            let d = vec2<f32>(f32(x) + 0.5, f32(y) + 0.5) - anchor(b);
            let f = building_at(b, d, half(b).x);
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
