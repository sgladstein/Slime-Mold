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
    // Dense slime condenses (`View::set_dense`, 11-14): the word in `news`
    // where each walker's condensing word starts (0 off); `DENSE_BIT` marks
    // a condensed walker (the engine's `dense_word`); `ripe_at` the steps
    // a walker must be ripe to condense (`condense`); `fhalf` where the
    // fine grid's second half (the condensing weights) starts.
    // `shalf` where the specks' second half (the streaks) starts.
    dense_at: u32, ripe_at: u32, fhalf: u32, shalf: u32,
    // The engine's alarm field (`View::set_alarm`, 11-14): which germ field
    // is current (0 or 1), and the alarm that reads full (0: not shown).
    alarm_side: u32, alarm_full: f32,
    // The plants (`View::set_plants`, 11-14): the word in `news` where the
    // engine's plant blocks start (0 off; `mould::plants`).
    plant_at: u32,
    // The map's flora (`View::set_plants`, `map::Items::flora`): which
    // forms its plants take up close (`plant_layer`).
    flora: u32,
    // Two spare words. To 2026-11-17 they scaled the alarm's pale haze on
    // bare ground and on the slime (`View::set_alarm_look`, 11-15); the
    // haze is gone (`flush_at`).
    spare_a: u32, spare_b: u32,
    // The scent view (`View::scent`, F2 in Larder World, 11-15): 1 dims the
    // world and washes over it the two fields a walker's sensor adds up
    // (the engine's `sense_at`): the food scent (the look's byte 3) in
    // blue with its contour lines, and the mould's trail (byte 0) in pink.
    scent_view: u32,
    // The mini-bosses' bodies (`View::set_lumps`, 2026-11-16): how many
    // lumps have a look, and each lump's centre in cells (x, y), its look
    // (`LumpLook`, 0 plain) and its strength 0-1; then each lump's look
    // left as it fades (1 whole, 0 gone: `LumpBody::remain`) and what it
    // has drunk (0-1, a leech's take: `LumpBody::fill`).
    n_lumps: u32,
    lumps: array<vec4<f32>, 4>,
    lump_fade: vec4<f32>,
    lump_fill: vec4<f32>,
    // How far each lump's members' own density sets where its look shows
    // (`LumpBody::held`: 1 out on its errand, 0 gathering).
    lump_held: vec4<f32>,
    // The crust (`View::set_crust`, 2026-11-17): the word in `news` where
    // the engine's crust block starts (0 off); the set walkers a cell's
    // block (`View::set_setting`, E2b; 0 off); a spare word (set slime's
    // look to 2026-11-18).
    // The Lure's steering field (`View::set_steer`, 2026-11-18): the word
    // in `news` where its blocks start, 0 hidden; and how far it reaches in
    // cells (`View::set_steer_reach`), 0 unknown.
    crust_at: u32, setc_at: u32, steer_reach: f32, steer_at: u32,
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
// The sprites (`src/world/sprites/`): 2176 x 160 RGBA, seventeen 128 px
// tiles (soldier body, helmet, flash, dead; six buildings, each body then
// lamp; the turret's head) above the same at 32 px.
@group(0) @binding(11) var<storage, read> atlas: array<u32>;
// Each shown cell's height and material, worked out once a frame by `prep`
// (rows `rw` apart from cell ox, oy). `base` reads these: a pixel needs 24
// heights (bilinear at six points) and four materials, and before this it
// worked each one out from the fields itself, which made the look cost
// 1.8-2.0 ms against 0.3 ms plain on the GTX 1070 at 1080p.
@group(0) @binding(12) var<storage, read_write> hcell: array<f32>;
@group(0) @binding(13) var<storage, read_write> mcell: array<vec4<u32>>;
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
// The engine's two germ fields (`View::set_alarm`, 11-14; bindings agreed
// with the slime-look thread): three blocks of cells each, germ, alarm,
// gas; `alarm_side` says which is current.
@group(0) @binding(25) var<storage, read> germ0: array<f32>;
@group(0) @binding(26) var<storage, read> germ1: array<f32>;

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
            let sx = x + (f32(i) + 0.5) * d; let sy = y + (f32(j) + 0.5) * d;
            if (P.shade == 1u) { sum += far_cell(sx, sy); } else { sum += cell(sx, sy); }
        }
    }
    return sum / f32(k * k);
}

// The far view's colour of the cell at (x, y), with the look on: the
// shaded view's own materials a cell at a time, under flat light, for the
// whole map below zoom 1 and the minimap at every zoom. Before 11-15 both
// drew the engine's raw colour (navy ground, brown rock, crystals brighter
// than the slime), a different picture from the game's (the planet review,
// item 1).
fn far_cell(x: f32, y: f32) -> vec3<f32> {
    if (x < 0.0 || y < 0.0 || x >= f32(P.ww) || y >= f32(P.wh)) { return BACK; }
    let ix = i32(x); let iy = i32(y);
    let i = cidx(ix, iy);
    let g = ground[i];
    let kind = g & 15u;
    let th = byte(look[i], 0u);
    var m = cell_mat(ix, iy, th, 0.0, 0.0);
    var c = m.albedo;
    // Deep water's darkness, drawn at the pixel in the shaded view.
    if (kind == WET && ((g >> 4u) & 7u) == SOIL_DEEP) { c = mix(c, DEEP_RGB, 0.7 * m.vis); }
    // The plants over the ground, by their cover, as `plant_layer` lays them.
    let pd = plant_d(i);
    if (pd > 0.0) { c = mix(c, plant_rgb(plant_k(i)), plant_cover(pd) * m.vis); }
    // Hard terrain and crystals are drawn over the material in the shaded
    // view; here their own colours.
    if (kind == ROCK || kind == WALL || kind == DOOR) { c = ter_cell(ter_pack(ix, iy)).xyz; }
    if (kind == CRYSTAL && P.n_gems > 0u) {
        let v = byte(g, 1u);
        c = mix(c, mix(vec3<f32>(50.0, 50.0, 48.0), vec3<f32>(38.0, 142.0, 165.0) + vec3<f32>(10.0, 70.0, 75.0) * 0.6, v), 1.0 - 0.8 * (1.0 - m.vis));
    }
    if (P.mask == 1u) { c *= 0.3 + 0.7 * clamp(vis[i], 0.0, 1.0); }
    return c;
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
// Rubble (a wall cell the slime ate to 0) is ground in the simulation, so
// it is drawn as ground (`cell_mat_c`, `ground_cell`): gravel, a fine cell
// a stone, each lighter or darker, in the worn wall's warm grey, with the
// slime drawn over it as over any ground. Until 2026-11-16 it was hard
// terrain, a low wall with the wall's own outline drawn over the slime: a
// ring three-quarters eaten still looked whole while slime crossed it, so
// the slime seemed to climb over walls (Reports/walls-2026-11-16).
const RUBBLE_RGB: vec3<f32> = vec3<f32>(52.0, 47.0, 41.0);
fn rubble_rgb(ix: i32, iy: i32) -> vec3<f32> {
    return RUBBLE_RGB * (0.75 + 0.5 * hash2(vec2<f32>(f32(ix), f32(iy))));
}
const CRYSTAL: u32 = 5u;
// A door (`look::Ground::Door`): a wall's cells the soldiers walk through
// and the slime still has to eat. Hard terrain like a wall, lower, a
// timber brown that darkens with its health as a wall's does, with the
// plain bevel of rock rather than the hedge's grain and a grate of bars a
// cell apart (`terrain_top`), so it reads as a built gate in the wall
// (walls thread, 2026-11-16). A slate blue first read as water in a ring
// standing in a lake.
const DOOR: u32 = 6u;
const LIGHT: vec3<f32> = vec3<f32>(-0.42, -0.58, 0.70);

fn cidx(ix: i32, iy: i32) -> u32 {
    let x = u32(clamp(ix, 0, i32(P.ww) - 1));
    let y = u32(clamp(iy, 0, i32(P.wh) - 1));
    return y * P.ww + x;
}

fn byte(v: u32, k: u32) -> f32 { return f32((v >> (8u * k)) & 255u) / 255.0; }

// **Plants** (11-14, the plants thread to the graphics lead's spec): a
// ground layer between the soil and the slime, all from the engine's
// fields (`news` from `plant_at`: blocks of cells, the engine's
// `PB_KIND` 1, plants 2, fire's heat 4, char 6). Each kind's colour is
// desaturated, cool and darker than the slime, so the slime stays the most
// saturated warm thing on screen; the cover is 1 - exp(-k density), so no
// edge is drawn where plants thin out.
fn pblock(b: u32, i: u32) -> u32 { return news[P.plant_at + b * P.ww * P.wh + i]; }
fn plant_d(i: u32) -> f32 { if (P.plant_at == 0u) { return 0.0; } return bitcast<f32>(pblock(2u, i)); }
fn plant_k(i: u32) -> u32 { return pblock(1u, i) & 3u; }
fn plant_heat(i: u32) -> f32 { if (P.plant_at == 0u) { return 0.0; } return bitcast<f32>(pblock(4u, i)); }
fn plant_char(i: u32) -> f32 { if (P.plant_at == 0u) { return 0.0; } return bitcast<f32>(pblock(6u, i)); }
// How fresh a cell's fire is (the engine's `PB_FRESH`, 1 as it catches).
fn plant_fresh(i: u32) -> f32 { return bitcast<f32>(pblock(8u, i)); }
// The ground's moisture in cell (ix, iy) (`plants::moisture`, block 7):
// 1 at water and up to 0.4 wetter within 8 cells of it.
fn plant_moist(ix: i32, iy: i32) -> f32 { if (P.plant_at == 0u) { return 0.0; } return bitcast<f32>(pblock(7u, cidx(ix, iy))); }
// Heat and freshness at world point (x, y), bilinear: a crown's flame
// takes them at its centre (`plant_kind_layer`).
fn heat_lin(x: f32, y: f32) -> vec2<f32> {
    let fx = x - 0.5; let fy = y - 0.5;
    let ix = i32(floor(fx)); let iy = i32(floor(fy));
    let tx = fx - floor(fx); let ty = fy - floor(fy);
    let a = cidx(ix, iy); let b = cidx(ix + 1, iy); let c = cidx(ix, iy + 1); let d = cidx(ix + 1, iy + 1);
    let h = mix(mix(plant_heat(a), plant_heat(b), tx), mix(plant_heat(c), plant_heat(d), tx), ty);
    let f = mix(mix(plant_fresh(a), plant_fresh(b), tx), mix(plant_fresh(c), plant_fresh(d), tx), ty);
    return vec2<f32>(h, f);
}
// Muted dark blue-green, each kind told apart by value (graphics lead,
// 11-14: below the slime's thin film in value, darker and bluer than the
// team olive, no gold): jungle darkest (a raised mass whose lit crowns
// read), reeds teal, meadow a little lighter, scrub dusty. Before
// (18:20Z to 21:30Z) meadow was (68, 76, 48), an olive that the owner
// called "mostly just green blobs" (19:26Z). Green, hue about 130-150
// (graphics lead 21:56Z): the first per-pixel cut, jungle (18,42,40) and
// reeds (32,56,54), sat at hue ~175 by the crystals' cyan, which is
// theirs alone (art rule 3), and took their pop on the thumbnails.
fn plant_rgb(k: u32) -> vec3<f32> {
    if (k == 1u) { return vec3<f32>(20.0, 42.0, 28.0); }
    if (k == 2u) { return vec3<f32>(34.0, 56.0, 40.0); }
    if (k == 3u) { return vec3<f32>(54.0, 54.0, 42.0); }
    return vec3<f32>(40.0, 60.0, 40.0);
}
// Each kind's height at full density (jungle tall, meadow low).
// (Graphics lead's review 11-14 18:20Z: the first heights, 0.6, 0.4, 0.2
// and 0.15, read flat; at these the jungle gets a canopy texture.)
fn plant_h(k: u32) -> f32 {
    if (k == 1u) { return 1.2; }
    if (k == 2u) { return 0.8; }
    if (k == 3u) { return 0.4; }
    return 0.25;
}
fn plant_cover(d: f32) -> f32 { return 1.0 - exp(-3.0 * d); }
// Burnt ground: the soil toward a dark warm grey-brown by the char (the
// plants that burnt there, so jungle leaves darker ground than meadow).
// The first cut mixed toward (11, 11, 12) and read as holes, next to the
// black spots the owner already asked about in the mould.
// Graphics lead 11-14 21:26Z: the brown read as a muddy blur; char is a
// near-black grey with a crisp edge (`shaded`), lighter than the first
// cut's (11, 11, 12), which read as holes.
const CHAR_RGB: vec3<f32> = vec3<f32>(25.0, 24.0, 23.0);
// The plants' fields at world point (`x`, `y`) in cells: the cubic
// B-spline of the 4 x 4 cells round it (as `cells_height` samples the
// height), per pixel, so nothing is stair-stepped at the owner's zoom 24
// (11-14 20:21Z: "the graphics quality should match that"). `d` is each
// kind's density (meadow, jungle, reeds, scrub), with the fire's heat
// and the char in the same 16 reads. Before (18:20Z to 21:30Z) the plants
// were mixed into each cell's material through a 3 x 3 Gaussian and
// blended between cells: soft blobs with no texture.
struct PlantPx { d: vec4<f32>, heat: f32, char: f32, fresh: f32 }
fn plant_px(x: f32, y: f32) -> PlantPx {
    var o: PlantPx;
    o.d = vec4<f32>(0.0); o.heat = 0.0; o.char = 0.0; o.fresh = 0.0;
    if (P.plant_at == 0u) { return o; }
    let fx = x - 0.5; let fy = y - 0.5;
    let ix = i32(floor(fx)); let iy = i32(floor(fy));
    let wx = bspline(fx - floor(fx)); let wy = bspline(fy - floor(fy));
    for (var j = 0; j < 4; j++) {
        for (var q = 0; q < 4; q++) {
            let w = wx[q] * wy[j];
            let c = cidx(ix - 1 + q, iy - 1 + j);
            let d = plant_d(c);
            if (d > 0.0) {
                let k = plant_k(c);
                o.d += w * d * vec4<f32>(f32(k == 0u), f32(k == 1u), f32(k == 2u), f32(k == 3u));
            }
            let hc = plant_heat(c);
            o.heat += w * hc;
            if (hc > 0.0) { o.fresh += w * plant_fresh(c); }
            o.char += w * plant_char(c);
        }
    }
    return o;
}

// **Crust** (the simulation-use plan's E1a, 2026-11-17; `BigDish::crust`):
// ground that remembers the slime, 0 to 1 a cell. Drawn as satin patches
// on the bare ground (the readability review's pick: value, hue and the
// gold are all taken, gloss is free): a two-octave value noise about a
// cell across, and a pixel is in a patch where the noise is under the
// crust's level, so the crust sets the share of ground glazed, each patch
// edge is a pixel wide at any zoom, and no line is drawn at any crust
// level. Inside, the ground lifts a little and cools; the patch's rim
// toward the light catches a thin highlight and the far rim a shadow, as
// a varnish would. Zoomed out past the patches' size it fades to their
// mean, a faint cool lift by the crust.
const CRUST_FREQ: f32 = 0.8;
fn crust_c(ix: i32, iy: i32) -> f32 { return bitcast<f32>(news[P.crust_at + cidx(ix, iy)]); }
fn crust_lin(x: f32, y: f32) -> f32 {
    let fx = x - 0.5; let fy = y - 0.5;
    let ix = i32(floor(fx)); let iy = i32(floor(fy));
    let tx = fx - floor(fx); let ty = fy - floor(fy);
    return mix(mix(crust_c(ix, iy), crust_c(ix + 1, iy), tx), mix(crust_c(ix, iy + 1), crust_c(ix + 1, iy + 1), tx), ty);
}
fn crust_noise(p: vec2<f32>) -> f32 { return 0.65 * vnoise(p) + 0.35 * vnoise(p * 2.7 + vec2<f32>(17.0, 5.0)); }
// The ground's albedo `a` with the crust at (x, y) laid over it, `gv` how
// much of the bare ground shows (1 bare, 0 under the slime).
fn crust_over(a: vec3<f32>, x: f32, y: f32, gv: f32) -> vec3<f32> {
    if (P.crust_at == 0u || gv <= 0.0) { return a; }
    let cv = crust_lin(x, y);
    if (cv <= 0.01) { return a; }
    let satin = a * 1.12 + vec3<f32>(3.0, 5.0, 8.0);
    // Cells a pixel, against the patches' size: past about half a patch
    // a pixel, the mean cover only.
    let px = 1.0 / P.zoom;
    let far = smoothstep(0.25, 0.6, px * CRUST_FREQ);
    // The noise's spread is about 0.2-0.8 (mean 0.5): the crust maps onto
    // it, so a full crust glazes nearly all the ground and a thin one a few
    // flecks (from 0.15 up, the first try's floor, a crust under 0.2 drew
    // nothing).
    let t = 0.3 + 0.5 * cv;
    var o = a;
    if (far < 1.0) {
        let p = vec2<f32>(x, y) * CRUST_FREQ;
        let nz = crust_noise(p);
        let w = max(px * CRUST_FREQ * 0.6, 0.004);
        let inside = smoothstep(-w, w, t - nz);
        if (inside > 0.0) {
            // The rim: the band inside the edge some 3 pixels deep (at
            // most a fifth of a cell), lit where the patch's outward normal
            // (up the noise) faces the light.
            let d = 0.35;
            let g = vec2<f32>(crust_noise(p + vec2<f32>(d, 0.0)) - crust_noise(p - vec2<f32>(d, 0.0)), crust_noise(p + vec2<f32>(0.0, d)) - crust_noise(p - vec2<f32>(0.0, d)));
            let gn = g / max(length(g), 1e-5);
            let band = 1.0 - smoothstep(w, w + min(3.0 * px, 0.2) * CRUST_FREQ * 0.6, t - nz);
            let lit = dot(gn, normalize(LIGHT.xy));
            var c = satin;
            c += vec3<f32>(34.0, 34.0, 36.0) * band * max(lit, 0.0);
            c *= 1.0 - 0.18 * band * max(-lit, 0.0);
            o = mix(a, c, inside);
        }
    }
    let mean = mix(a, satin, clamp(1.1 * cv, 0.0, 1.0));
    return mix(mix(o, mean, far), a, 1.0 - gv);
}

// One kind's plants at (x, y) as round crowns on a jittered grid `s`
// cells apart (squashed `stretch` times along y for reeds), each crown's
// radius `r0` of the spacing times the square root of the kind's density
// there, so the share of ground a kind covers follows its density: a
// thin stand is scattered tufts with ground between, a full one closes
// up, and as fire eats the plants each crown shrinks and goes. The
// crown's own dome lights it (`lit`), so a canopy reads as a raised mass
// of lit crowns and shaded gaps. `a` is the crown's cover, antialiased
// over about a pixel; `tone` is the crown's own hash (a little value and
// hue each). No stamp or sprite: a crown's size is the field's value.
struct Crown { a: f32, lit: f32, tone: f32, dome: f32, at: vec2<f32> }
// One kind's form on one flora (`flora_form`): the grid's spacing `s`,
// the crown's radius `r0` of it, `stretch` along y, the outline's `lobe`
// and the dome's `relief`, its `shape` (0 a lobed dome, 1 a blade, 2 a
// thorny star, 3 a cap with a dark eye, 4 a ring), `clump` (how much
// the crowns gather into patches, by a slow noise at their centres, the
// cover staying with the density) and its colour; `wet` gathers the
// patches onto the wettest ground (the moisture field, so along the
// water) instead of the noise alone, and `tip` (when not zero) is the
// colour each crown's top turns toward.
struct Form { s: f32, r0: f32, stretch: f32, lobe: f32, relief: f32, shape: u32, clump: f32, col: vec3<f32>, wet: f32, tip: vec3<f32> }
// `lobe` ruffles each crown's outline (five to eight lobes, so a grass tuft is
// spiky and a tree's crown leafy, not a pebble: the first cut's round
// domes read as cobbles, and seven deep lobes on every tuft as stars);
// `relief` is how strongly its dome is lit.
// sin(n theta + phi) for the unit vector u = (cos theta, sin theta).
fn lobe_wave(u: vec2<f32>, n: u32, phi: f32) -> f32 {
    let u2 = vec2<f32>(u.x * u.x - u.y * u.y, 2.0 * u.x * u.y);
    let u4 = vec2<f32>(u2.x * u2.x - u2.y * u2.y, 2.0 * u2.x * u2.y);
    var z = vec2<f32>(u4.x * u.x - u4.y * u.y, u4.x * u.y + u4.y * u.x);
    if (n >= 6u) { z = vec2<f32>(u4.x * u2.x - u4.y * u2.y, u4.x * u2.y + u4.y * u2.x); }
    if (n >= 7u) { let z6 = z; z = vec2<f32>(z6.x * u.x - z6.y * u.y, z6.x * u.y + z6.y * u.x); }
    if (n >= 8u) { z = vec2<f32>(u4.x * u4.x - u4.y * u4.y, 2.0 * u4.x * u4.y); }
    return z.y * cos(phi) + z.x * sin(phi);
}

fn crowns(x: f32, y: f32, fm: Form, dk: f32, seed: f32) -> Crown {
    var o: Crown;
    o.a = 0.0; o.lit = 1.0; o.tone = 0.5; o.dome = 0.0; o.at = vec2<f32>(x, y);
    let s = fm.s;
    let q = vec2<f32>(x, y / fm.stretch) / s;
    let qi = floor(q);
    let rr = fm.r0 * sqrt(min(dk, 1.0));
    let aa = 1.2 / (P.zoom * s);
    let l = normalize(LIGHT);
    var top = -1.0;
    for (var j = -1; j <= 1; j++) {
        for (var i = -1; i <= 1; i++) {
            let c = qi + vec2<f32>(f32(i), f32(j));
            let h1 = hash2(c + seed);
            let h2 = hash2(c + seed + 17.3);
            let h3 = hash2(c + seed + 41.9);
            let ctr = c + 0.15 + 0.7 * vec2<f32>(h1, h2);
            var r = rr * (0.6 + 0.4 * h3);
            if (fm.clump > 0.0) {
                // Patches: big where a slow noise is high, none where low;
                // a denser stand fills more of the ground between.
                var g = vnoise(ctr * s / 5.0 + seed);
                if (fm.wet > 0.0) {
                    let mo = plant_moist(i32(floor(ctr.x * s)), i32(floor(ctr.y * s * fm.stretch)));
                    g = mix(g, smoothstep(0.55, 0.95, mo) + 0.3 * (g - 0.5), fm.wet);
                }
                r *= mix(1.0, 1.45 * smoothstep(0.3, 0.7, g + 0.5 * (dk - 0.5)), fm.clump);
            }
            if (r <= 0.0) { continue; }
            var rel = q - ctr;
            var reach = r * (1.0 + fm.lobe);
            if (fm.shape == 1u) {
                // A blade: a thin ellipse leaning up to 35 degrees.
                let t = (h1 - 0.5) * 1.2;
                let cs = cos(t); let sn = sin(t);
                rel = vec2<f32>(cs * rel.x - sn * rel.y, sn * rel.x + cs * rel.y) * vec2<f32>(2.2, 0.45);
                reach = r * 2.4;
            }
            let dd = length(rel);
            if (dd > reach + aa) { continue; }
            var re = r;
            if (fm.shape == 2u) {
                // Thorns: nine to twelve sharp points (atan2 and sin only
                // here, on the planets that grow thorns).
                let ang = atan2(rel.y, rel.x);
                let w = pow(0.5 + 0.5 * sin((9.0 + floor(4.0 * h2)) * ang + 6.2831853 * h1), 3.0);
                re = r * (1.0 - 0.5 * fm.lobe + 1.5 * fm.lobe * w);
            } else if (fm.lobe > 0.0) {
                // sin(n theta + phi) without the angle: (cos theta, sin theta)
                // to the n-th power as a complex number (`lobe_wave`); atan2 and
                // sin cost the 1070 0.03 ms at zoom 6 and 24 (performance
                // thread, 11-15), and the pixels match to 1/255.
                re = r * (1.0 + fm.lobe * lobe_wave(rel / max(dd, 1e-20), 5u + u32(floor(4.0 * h2)), 6.2831853 * h1));
            }
            var a = clamp((re - dd) / aa + 0.5, 0.0, 1.0);
            if (fm.shape == 4u) { a = min(a, clamp((dd - 0.5 * re) / aa + 0.5, 0.0, 1.0)); }
            if (a <= 0.0) { continue; }
            let u = min(dd / max(re, 1e-4), 1.0);
            var dome = sqrt(1.0 - u * u);
            if (fm.shape == 4u) { dome = sqrt(max(1.0 - pow((u - 0.75) / 0.25, 2.0), 0.0)); }
            o.a = max(o.a, a);
            // The highest dome over this point is the crown seen.
            if (dome > top) {
                top = dome;
                let n = normalize(vec3<f32>(rel / max(r, 1e-4), max(dome, 0.05)));
                o.lit = (1.0 + fm.relief * (dot(n, l) - l.z)) * (1.0 - 0.35 * fm.relief * (1.0 - dome));
                // A cap's dark eye (fungi, lichen rosettes).
                if (fm.shape == 3u) { o.lit *= 0.55 + 0.45 * smoothstep(0.12, 0.42, u); }
                o.tone = h3;
                o.dome = dome;
                o.at = vec2<f32>(ctr.x * s, ctr.y * s * fm.stretch);
            }
        }
    }
    return o;
}

// Each biome's plants have their own forms up close (art lead 11-15,
// planet review #3: only 4 of 8 biomes could be named at zoom 12; the
// plants are the biome's signature). The kinds and amounts are the
// simulation's; the flora (the map's `flora` line, from the level
// generator's biome) picks how each kind grows and its colour, all
// green except the caves' fungi and the core's alien growth, which are
// dark mauve and purple (never cyan: the crystals' alone, nor gold or
// red-orange: the slime's and the fire's).
fn flora_form(fl: u32, k: u32) -> Form {
    // Flora 0 (meadow), and every kind a flora leaves alone.
    var f: Form;
    f.stretch = 1.0; f.shape = 0u; f.clump = 0.0; f.col = plant_rgb(k); f.wet = 0.0; f.tip = vec3<f32>(0.0);
    if (k == 0u) { f.s = 0.45; f.r0 = 0.8; f.lobe = 0.2; f.relief = 0.35; }
    else if (k == 3u) { f.s = 1.1; f.r0 = 0.5; f.lobe = 0.25; f.relief = 0.6; }
    else if (k == 2u) { f.s = 0.4; f.r0 = 0.6; f.stretch = 2.4; f.lobe = 0.15; f.relief = 0.4; }
    else { f.s = 1.7; f.r0 = 0.85; f.lobe = 0.12; f.relief = 0.9; }
    if (fl == 1u) {
        // Barrens: dry grass blades; thorn bushes in patches; oasis palms.
        if (k == 0u) { f.shape = 1u; f.s = 0.55; f.r0 = 0.7; f.relief = 0.3; f.col = vec3<f32>(58.0, 60.0, 42.0); }
        else if (k == 3u) { f.shape = 2u; f.s = 1.3; f.r0 = 0.5; f.lobe = 0.45; f.clump = 0.8; f.col = vec3<f32>(58.0, 56.0, 40.0); }
        else if (k == 1u) { f.shape = 2u; f.s = 1.6; f.lobe = 0.35; f.col = vec3<f32>(30.0, 48.0, 30.0); }
    } else if (fl == 2u) {
        // Fen: wet moss hummocks, dark olive-brown with yellow-green tips,
        // gathered along the water (art lead 11-15: the fen's green round
        // clumps were the closest pair with the meadow's; in grey it is a
        // coarser, darker grain). Before: even green cushions 0.6 apart,
        // (34,58,36), clumped by noise alone.
        if (k == 0u) { f.s = 0.85; f.r0 = 0.95; f.lobe = 0.08; f.relief = 0.95; f.clump = 0.9; f.wet = 0.6; f.col = vec3<f32>(30.0, 29.0, 16.0); f.tip = vec3<f32>(62.0, 74.0, 28.0); }
        else if (k == 1u) { f.s = 1.1; f.r0 = 0.8; f.lobe = 0.15; f.col = vec3<f32>(24.0, 44.0, 30.0); }
    } else if (fl == 3u) {
        // River valley: long grass, blade by blade.
        if (k == 0u) { f.shape = 1u; f.s = 0.6; f.r0 = 0.95; f.relief = 0.3; f.col = vec3<f32>(46.0, 68.0, 42.0); }
    } else if (fl == 4u) {
        // Jungle: a broadleaf canopy of big crowns over ferns.
        if (k == 1u) { f.s = 2.3; f.r0 = 0.9; f.lobe = 0.18; f.relief = 1.0; f.col = vec3<f32>(18.0, 44.0, 26.0); }
        else if (k == 0u) { f.shape = 2u; f.s = 0.7; f.r0 = 0.7; f.lobe = 0.4; f.relief = 0.5; f.col = vec3<f32>(30.0, 58.0, 34.0); }
    } else if (fl == 5u) {
        // Highlands: lichen rosettes and cushion plants in patches; firs.
        if (k == 0u) { f.shape = 3u; f.s = 0.6; f.r0 = 0.6; f.lobe = 0.0; f.relief = 0.4; f.clump = 0.7; f.col = vec3<f32>(62.0, 64.0, 48.0); }
        else if (k == 3u) { f.s = 0.9; f.r0 = 0.55; f.lobe = 0.06; f.relief = 0.8; f.clump = 0.5; f.col = vec3<f32>(52.0, 58.0, 40.0); }
        else if (k == 1u) { f.shape = 2u; f.s = 1.4; f.r0 = 0.7; f.lobe = 0.3; f.col = vec3<f32>(20.0, 40.0, 30.0); }
    } else if (fl == 6u) {
        // Caves: fungi. Caps with a dark eye, giant ones for the jungle
        // kind, stalks for reeds; a dark mauve-grey.
        f.shape = 3u; f.lobe = 0.0;
        if (k == 0u) { f.s = 0.55; f.r0 = 0.65; f.relief = 0.6; f.col = vec3<f32>(70.0, 62.0, 72.0); }
        else if (k == 1u) { f.s = 1.9; f.r0 = 0.85; f.relief = 1.0; f.col = vec3<f32>(78.0, 66.0, 82.0); }
        else if (k == 3u) { f.s = 1.0; f.col = vec3<f32>(66.0, 60.0, 62.0); }
        else { f.shape = 1u; f.col = vec3<f32>(60.0, 56.0, 66.0); }
    } else if (fl == 7u) {
        // The core: alien growth. Coral rings, thorned crowns; purple-
        // magenta, hue 290-320, the only purple on the map (art lead
        // 11-15: red is danger's; before, wine at hue 315-340).
        if (k == 0u) { f.shape = 4u; f.s = 0.6; f.r0 = 0.7; f.lobe = 0.1; f.col = vec3<f32>(68.0, 40.0, 60.0); }
        else if (k == 1u) { f.shape = 2u; f.s = 1.8; f.r0 = 0.8; f.lobe = 0.4; f.relief = 0.9; f.col = vec3<f32>(52.0, 28.0, 48.0); }
        else if (k == 3u) { f.shape = 4u; f.s = 1.0; f.col = vec3<f32>(64.0, 44.0, 58.0); }
        else { f.shape = 1u; f.col = vec3<f32>(58.0, 38.0, 54.0); }
    }
    return f;
}

// The plant layer at (x, y): each kind's crowns, lower kinds first
// (meadow, scrub, reeds, jungle by `plant_h`), so a canopy stands over
// the grass. `rgb` is premultiplied by `cover`. Where a kind's crowns
// would be under 2-3 pixels (they would alias), they fade to the
// kind's mean cover and colour (the far view).
// `fl` and `fc` are the crowns' own flames (premultiplied, and their
// cover), `fw` how much of the cover is drawn as crowns (not the far
// view's mean), for `shaded`'s mix with the smooth flame.
struct PlantLayer { rgb: vec3<f32>, cover: f32, fl: vec3<f32>, fc: f32, fw: f32 }
fn plant_kind_layer(o: ptr<function, PlantLayer>, x: f32, y: f32, k: u32, dk: f32, hot: bool) {
    if (dk <= 0.003) { return; }
    let fm = flora_form(P.flora, k);
    let f = smoothstep(1.6, 3.2, fm.s * P.zoom);
    var a = plant_cover(dk);
    var col = fm.col;
    var fa = 0.0;
    var fl = vec3<f32>(0.0);
    if (f > 0.0) {
        let cr = crowns(x, y, fm, dk, f32(k) * 101.0 + 7.0);
        let tint = mix(vec3<f32>(1.06, 1.0, 0.9), vec3<f32>(0.92, 1.0, 1.08), cr.tone);
        a = mix(a, cr.a, f);
        var cc = col;
        if (fm.tip.x > 0.0) { cc = mix(col, fm.tip, smoothstep(0.8, 1.0, cr.dome)); }
        col = mix(col, cc * tint * cr.lit * (0.85 + 0.3 * cr.tone), f);
        // A burning crown: its flame from the heat at its centre, so each
        // crown flares, shrinks and goes out on its own and a front has
        // the plants' own tongues and gaps (graphics lead 21:26Z). A fresh
        // fire burns a step hotter (toward yellow), the crown's top
        // hottest.
        // A freshly caught crown burns white-hot at its top (`FLAME_CORE`,
        // luma ~240), above the thick slime's highlight (art lead 11-15,
        // planet review #11, rule 2: light is the top of the value
        // order; before, the front's luma p99 was 155 against the slime's
        // 240), and the core fades as the fire ages (`PB_FRESH`).
        if (hot && cr.a > 0.0) {
            let hf = heat_lin(cr.at.x, cr.at.y);
            let h = (hf.x + 0.3 * hf.y * step(0.01, hf.x)) * (0.8 + 0.25 * cr.dome);
            // sqrt: freshness falls x 0.85 a fire step, so a crown's core
            // stays white for about the half second it takes to catch
            // its neighbours (linear, the front's luma p99 was 186).
            let core = sqrt(clamp(hf.y, 0.0, 1.0)) * step(0.01, hf.x) * (0.5 + 0.5 * cr.dome);
            fa = cr.a * f * max(flame_cover(hf.x), core);
            fl = mix(flame_body(h), FLAME_CORE, core) * fa;
        }
    }
    (*o).rgb = (*o).rgb * (1.0 - a) + col * a;
    (*o).cover = (*o).cover + a * (1.0 - (*o).cover);
    (*o).fl = (*o).fl * (1.0 - a) + fl;
    (*o).fc = (*o).fc * (1.0 - a) + fa;
    (*o).fw = (*o).fw * (1.0 - a) + a * f;
}
fn plant_layer(x: f32, y: f32, d: vec4<f32>, hot: bool) -> PlantLayer {
    var o: PlantLayer;
    o.rgb = vec3<f32>(0.0); o.cover = 0.0; o.fl = vec3<f32>(0.0); o.fc = 0.0; o.fw = 0.0;
    // Meadow, scrub, reeds, jungle, each in its flora's form
    // (`flora_form`; on flora 0: fine spiky tufts half a cell apart,
    // bushes a cell apart with sand between, narrow stems long along y,
    // crowns 1.7 cells apart closing into a canopy).
    plant_kind_layer(&o, x, y, 0u, d.x, hot);
    plant_kind_layer(&o, x, y, 3u, d.w, hot);
    plant_kind_layer(&o, x, y, 2u, d.z, hot);
    plant_kind_layer(&o, x, y, 1u, d.y, hot);
    return o;
}

// The flames as light: one continuous blackbody ramp of the heat, so
// every pixel has its own value. Graphics lead 11-14 21:01Z: three layers
// with 0.03-wide steps (the deep-zoom cut of 20:40Z) read as lava or flat
// paint, and a step on a field is a thresholded edge (owner, 2026-11-09/10).
// The engine's heat follows the fuel left (flicker takes up to 35% off
// each step), so the fresh front is hottest. Smoothed, a burning band's
// heat sits mostly at 0.5-0.7 and maps to orange (luminance 100-140 with
// the ground under it, below the slime's highlight); yellow toward 0.85,
// near white only near 1, and the fading embers behind (heat * 0.95 a
// step) fall to dull red. With orange at 0.75 the bulk read red and dim
// (median luminance 82, 21:20Z cut).
// Before: `flame` mapped heat^1.5 ember-orange-yellow with yellow from
// 0.85, and the bulk read near white in grey (20:10Z review).
// The white-hot core of a freshly caught fire (luma about 240).
const FLAME_CORE: vec3<f32> = vec3<f32>(255.0, 242.0, 205.0);
fn flame_body(h: f32) -> vec3<f32> {
    let t = clamp(h, 0.0, 1.0);
    let ember = vec3<f32>(60.0, 12.0, 3.0);
    let red = vec3<f32>(160.0, 36.0, 8.0);
    let orange = vec3<f32>(222.0, 96.0, 20.0);
    let yellow = vec3<f32>(248.0, 172.0, 68.0);
    let white = vec3<f32>(255.0, 232.0, 190.0);
    if (t < 0.4) { return mix(ember, red, t / 0.4); }
    if (t < 0.62) { return mix(red, orange, (t - 0.4) / 0.22); }
    if (t < 0.85) { return mix(orange, yellow, (t - 0.62) / 0.23); }
    return mix(yellow, white, smoothstep(0.85, 1.0, t));
}

// How much the flame's body covers a pixel: a steep but continuous rise
// over heat 0.12-0.2, so at the owner's deep zoom (24 px a cell; 11-14
// 20:21Z: "the fire is blurry. The assumption should be that I am
// playing zoomed in") the flames' edge is a few pixels wide and the
// B-spline's lumps read as tongues. The cover from 0.08 to 0.38 (21:10Z
// cut) read soft and blobby at zoom 12.
fn flame_cover(h: f32) -> f32 { return smoothstep(0.12, 0.2, h); }

// The warm light a fire spills on the ground and slime around it: the
// same heat field's faint tail (the B-spline reaches about 2 cells past a
// burning cell), added as light, never as a drawn halo. From heat 0.04,
// so the dying embers behind a front (graphics lead 21:26Z: "a muddy
// brown blur") add next to none.
fn flame_spill(h: f32) -> vec3<f32> {
    return vec3<f32>(120.0, 44.0, 10.0) * (0.4 * smoothstep(0.04, 0.3, h));
}

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
// Plain soil at elevation `e` (-1 to 1), in its planet's own hue (`P.flora`,
// `levelgen::Biome as u8`): art lead 11-15, the biomes' plain soils were one
// slate grey and read apart by their plants alone. Each tint keeps the
// value about where it was (grey 34-41 at the middle elevation, inside the
// ground's 20-45 band) and moves the hue: barrens a sun-baked tan (about
// 30 degrees), caves a cold blue-grey (about 220), meadow a little brown,
// jungle a dark red-brown humus, fen olive mud, valley a warm silt;
// highlands keep the neutral stone grey.
fn biome_tint(fl: u32) -> vec3<f32> {
    if (fl == 1u) { return vec3<f32>(1.31, 1.04, 0.75); }
    if (fl == 2u) { return vec3<f32>(0.95, 1.0, 0.88); }
    if (fl == 3u) { return vec3<f32>(1.08, 1.0, 0.92); }
    if (fl == 4u) { return vec3<f32>(1.33, 0.88, 0.7); }
    if (fl == 5u) { return vec3<f32>(1.0, 1.0, 1.0); }
    if (fl == 6u) { return vec3<f32>(0.91, 0.99, 1.18); }
    return vec3<f32>(1.08, 1.0, 0.9);
}
fn plain_soil(e: f32) -> vec3<f32> {
    return mix(SOIL_LOW, SOIL_HIGH, 0.5 + 0.5 * e) * biome_tint(P.flora);
}
// Soil kinds (byte 0's high nibble, `look::Fields::soil`; the mould's
// `BigDish::soil`, Larder World maps): each changes the mould (sand: its
// trail fades, and it spreads further; clay: its trail lasts, and it is
// trapped; barren: no births, upkeep x 3; Reports/larder-maps-2026-11-11.md,
// *Which ground the mould feels*), so each reads as its own ground.
// Each still dark and muted beside plain ground (35,38,42 flat), told
// apart by value and its own texture (`soil_detail`), so a planet's
// biome reads from a grey thumbnail (art lead, 11-14): sand just above
// the ground with wind ripples, clay darker with crack lines, barren a
// lighter ash grey (still a clear step under rock) with clumped grains,
// water darker still. Before
// 2026-11-14 sand was (150,134,92), brighter than thin slime, and clay a
// saturated brick; #247's flat (70,68,60)/(56,38,34)/(50,50,50) read as
// one grey on the planets (Reports/soil-materials-2026-11-14).
const SAND_RGB: vec3<f32> = vec3<f32>(50.0, 48.0, 42.0);
const CLAY_RGB: vec3<f32> = vec3<f32>(38.0, 28.0, 26.0);
const BARREN_RGB: vec3<f32> = vec3<f32>(55.0, 55.0, 53.0);
// Standing water: a clear step darker than any ground, so it reads as a
// hollow in a grey thumbnail (art lead 11-14 18:56Z: wet was the ground's
// value at (10,24,34), and Fen read as one flat field); its lit shore and
// gloss say it is wet (`cell_mat_c`, `soil_detail`).
// Shallow water is a clear step lighter than deep (at least 20 in grey;
// the planet review 11-15: fords were 6-7 off deep water and could not be
// seen at zoom 1). It was (5, 14, 22).
const WET_RGB: vec3<f32> = vec3<f32>(14.0, 33.0, 41.0);
// Deep water (soil 4 with the wet bit, #285): darker still at its middle;
// a bridge (soil 5) is a plank deck. The owner (11-14 18:01Z) on the
// first river: "just a blue curved line on the map. It doesn't look
// professional."
const SOIL_DEEP: u32 = 4u;
const SOIL_BRIDGE: u32 = 5u;
const DEEP_RGB: vec3<f32> = vec3<f32>(1.0, 6.0, 14.0);
// Weathered grey-brown, its value between rock and soft rock: gold is
// the slime's hue (rule 3), and the first orange planks read as slime
// beside its front at zoom 6 (art lead, 11-14 19:23Z).
const WOOD_RGB: vec3<f32> = vec3<f32>(98.0, 90.0, 80.0);
// Mould cut off from food fades towards this (`cell_mat`'s `cut`).
const ASH_RGB: vec3<f32> = vec3<f32>(120.0, 112.0, 104.0);
// Condensed mould's colour (`cell_mat_c`'s `dense`): a deep bronze-brown,
// the gold body gone darker and denser, told from the panic's amber and
// the bolt's red by value and shine, not hue (art director, 11-14: the
// first copper, 215,88,30, sat between the two and read as panic at zoom
// 6). Its gloss is near full, so the highlight is tight and bright. A
// condensed walker's mark in its word (the engine's `DENSE_BIT`), an
// absorbed one's (`ABSORB_BIT`, its steps since in the low bits, for
// `ABSORB_STEPS`), and the light of fresh absorptions (`prep`).
const DENSE_RGB: vec3<f32> = vec3<f32>(92.0, 58.0, 34.0);
const DENSE_BIT: u32 = 0x10000u;
const ABSORB_BIT: u32 = 0x20000u;
const ABSORB_STEPS: f32 = 15.0;
const ABSORB_RGB: vec3<f32> = vec3<f32>(255.0, 232.0, 190.0);
// The fine grid's second half, a word a fine cell (`draw_specks`, read by
// `prep`): the condensed weights in bits 0-12 (15 a condensed walker, so
// 546 of them a fine cell; the engine's crowd cap keeps a cell to 255), a
// mini-boss's members in bits 13-21 (one each, 511), a flag a lump in bits
// 22-25 (set with `atomicOr`: which lumps have members there), fresh
// absorptions from bit 26 (63). Before the mini-bosses (2026-11-16) the
// absorptions sat at bit 16.
const FG_DENSE: u32 = 0x1FFFu;
const FG_LUMP: u32 = 13u;
const FG_LUMP_MASK: u32 = 0x1FFu;
const FG_FLAGS: u32 = 22u;
const FG_ABSORB: u32 = 26u;
// A mini-boss's member: its lump + 1 in its dense word's bits 25-27 (the
// engine's `lump_of`), the hits it has taken in bits 28-31.
const LUMP_SHIFT: u32 = 25u;
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
// The cell's height (x) and the mould's own share of it (y: no land, no
// plants), which `shaded` reads as the film's thickness. Before 11-14 the
// thickness was read back from the whole height, so high land or a plant
// patch (jungle stands 1.2) read as a thick film: the soil's material and
// the crisp edges were switched off over every plant patch and faded on
// high ground.
fn cell_height(ix: i32, iy: i32, th: f32) -> vec2<f32> {
    // Off the world, rock, walls and rubble stand at the land's height
    // here: `terrain_over` draws them, edges and shadows, at the pixel
    // (see *Hard terrain*). Their cell heights used to stand here, and the
    // slope between a tall cell and the ground made a stepped bevel and a
    // dark band a cell wide round every rock and wall at high zoom.
    if (ix < 0 || iy < 0 || ix >= i32(P.ww) || iy >= i32(P.wh)) { return vec2<f32>(0.0); }
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
    if (kind == ROCK || kind == WALL || kind == DOOR) { h = 0.0; }
    else if (kind == WET) { h = max(h - 0.3, 0.0); }
    else if (kind == CRYSTAL && P.n_gems == 0u) { h = max(h, (0.6 + 9.0 * byte(g, 2u)) * v); }
    let hm = h;
    // Plants stand by their density and kind; the shading makes their
    // texture from the field's own variation.
    if (kind != ROCK && kind != WALL && kind != DOOR) {
        // Through the same 3 x 3 Gaussian (1 2 1) as their colour
        // (as `plant_look` smoothed it then). Read cell by cell, a row of reeds along a bank,
        // its density varying cell to cell, stood as a string of beads,
        // each with a lit and a shaded flank: at zoom 12-24 the waterline
        // read as a dark stepped edge (owner, 11-14 20:21Z: "the edges
        // between different grounds is pixalated").
        if (P.plant_at != 0u) {
            var ph = 0.0;
            for (var dy = -1; dy <= 1; dy++) {
                for (var dx = -1; dx <= 1; dx++) {
                    let j = cidx(clamp(ix + dx, 0, i32(P.ww) - 1), clamp(iy + dy, 0, i32(P.wh) - 1));
                    let pd = plant_d(j);
                    if (pd > 0.0) { ph += f32((2 - abs(dx)) * (2 - abs(dy))) / 16.0 * pd * plant_h(plant_k(j)); }
                }
            }
            h += ph;
        }
    }
    return vec2<f32>(land + h, hm);
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
// The mould's own height a fine cell (`cell_height`'s y), in the buffer's
// second half (`View::cell_buffers` sizes it for both).
fn hm(ix: i32, iy: i32) -> f32 { return hcell[P.rw * P.rh + ridx(ix, iy)]; }
// The mould film's thickness at (x, y): the inverse of the height curve on
// the mould's own height, bilinear between fine cells.
fn film_h(x: f32, y: f32) -> f32 {
    let f = fpos(x, y);
    let ix = i32(floor(f.x)); let iy = i32(floor(f.y));
    let tx = f.x - floor(f.x); let ty = f.y - floor(f.y);
    return mix(mix(hm(ix, iy), hm(ix + 1, iy), tx), mix(hm(ix, iy + 1), hm(ix + 1, iy + 1), tx), ty);
}
fn film_th(x: f32, y: f32) -> f32 {
    return -log(1.0 - clamp(film_h(x, y) * 0.2, 0.0, 0.999) * (1.0 - exp(-2.6))) / 2.6;
}

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
const GEM_STUB: f32 = 0.1;
// The most a mould film on a crystal covers it (`shaded`): a thick mat
// still lets a fifth of the faces and glow through, so it reads as a
// crystal under mould.
const GEM_FILM: f32 = 0.92;
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
// `vis`: how much of the ground shows through the slime (1 bare, 0.05
// under an opaque body), so `shaded` lays the plants and char under it.
struct Mat { albedo: vec3<f32>, glow: vec3<f32>, gloss: f32, body: f32, gem: f32, vis: f32 };

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

fn cell_mat(ix: i32, iy: i32, th: f32, cut: f32, dense: f32) -> Mat {
    return cell_mat_c(ix, iy, th, cut, dense, cell_colour(ix, iy));
}

// A cell's material with the engine's colour `c` (`cell_colour`, or the
// smoothed `colour_at` for a fine cell).
fn cell_mat_c(ix: i32, iy: i32, th: f32, cut: f32, dense: f32, c: vec3<f32>) -> Mat {
    var m: Mat;
    if (ix < 0 || iy < 0 || ix >= i32(P.ww) || iy >= i32(P.wh)) {
        m.albedo = plain_soil(0.0); m.glow = vec3<f32>(0.0); m.gloss = 0.05; m.body = 0.0; m.gem = 0.0; m.vis = 1.0;
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
    var base = plain_soil(byte(g, 3u));
    let sk = (g >> 4u) & 7u;
    if (sk == 1u) { base = SAND_RGB; } else if (sk == 2u) { base = CLAY_RGB; } else if (sk == 3u) { base = BARREN_RGB; }
    if (kind == RUBBLE) { base = rubble_rgb(ix, iy); }
    // Burnt ground and the plants are laid per pixel in `shaded`
    // (`plant_px`), under the slime by `vis`.
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
    // The push eases off where the body is thickest (art director, 11-14
    // 08:15Z: the soldiers and the bullet light must still win the value
    // order), and the gel adds a little of the hue's own light, so a thin
    // film stays above the darker ground (crystals thread, 11-14) in grey.
    let thick = smoothstep(0.6, 1.0, th);
    let vivid = lum + (hue - vec3<f32>(lum)) * (1.0 + 0.45 * (1.0 - cut)) * (1.0 - 0.3 * thick);
    // Condensed mould (11-14, the condense thread): where its walkers have
    // condensed (`dense`, the share of the mass round the cell) the body
    // turns deep bronze and holds near-full gloss, a harder, denser body,
    // so the half of the cover it gave up and the strength it kept both
    // show. The share is the fade, as the ash's is. It is mixed after the
    // gold's saturation push: mixed into the hue before it, the push turned
    // the bronze orange, the panic's colour.
    let shade_th = 0.95 + 0.2 * smoothstep(0.3, 1.0, th);
    let body_c = mix(vivid * shade_th, DENSE_RGB * shade_th, dense * (1.0 - cut));
    var gel = soil * (0.55 + 0.7 * vivid / 255.0) + vivid * 0.06;
    // The film's inside lifted toward 95 in grey with its own light (art
    // lead 11-15: thin film from 80, above rock and soft rock), gated by
    // the thickness, which carries the walkers' count (`prep`), so the
    // faint tail past the last walkers keeps the ground's value and the
    // edge stays as sharp as its walkers. Lifted everywhere it widened
    // the edge from 0.96 to 1.74 cells (dead-ends 11-15).
    let gel_l = dot(gel, vec3<f32>(0.299, 0.587, 0.114));
    gel += vivid * max(95.0 - gel_l, 0.0) / max(lum, 1.0) * smoothstep(FILM_LIFT_A, FILM_LIFT_B, th);
    // Tried and reverted 11-15: lifting the gel to 100 grey with its own
    // light (rule 2, film from 80). It barely moved the review's film p10,
    // whose low end is the veins' shaded far flank (the shade floor in
    // `shaded` fixed that), and it widened the slime's edge from 0.96 to
    // 1.74 cells (the_slime_edge_is_as_sharp_as_its_walkers), because the
    // faint tail past the walkers lit up.
    // The gel comes in faster (was 1 - exp(-14 th)): a frontier film a
    // few hundredths thick was a third gel, 57 in grey over ground at 40,
    // under rock (the review 11-15: film 10th percentile 49-86).
    let film = 1.0 - exp(-40.0 * th);
    // Opaque from a third of full thickness (was 0.08 to 0.55). The fork's
    // start crystal stands on a disc of clay; the body over it was only
    // about half opaque, so the clay showed through as a flat orange-red
    // disc with the map patch's own edge (art director, 11-14, via the
    // crystals thread). The ground now shows through the frontier's thin
    // film only, where its thickness grades it.
    let opaque = smoothstep(0.05, 0.35, th);
    // The thin film's floor is absolute (art lead 11-15: over the fen's
    // dark mud the film fell to 73 in grey): wherever it has thickness,
    // the film is lifted with the slime's own light to at least 95 before
    // shading (80 on its shaded flank, the shade floor being 0.85) over any
    // ground, gated as the gel's lift so the tail past the walkers stays
    // ground.
    var thin = mix(soil, gel, film);
    let thin_l = dot(thin, vec3<f32>(0.299, 0.587, 0.114));
    thin += vivid / max(lum, 1.0) * max(95.0 * smoothstep(FILM_LIFT_A, FILM_LIFT_B, th) - thin_l, 0.0);
    m.albedo = mix(thin, body_c, opaque * 0.95);
    // How much of the ground's own detail (plants, char, soil texture)
    // shows through the slime here, gone by the thin film's middle (th
    // 0.05) by the colour's thickness, walkers counted (art lead 11-15,
    // planet review #8: under any slime only the slime's texture reads).
    // Before, 1 - 0.95 x opaque left the plants whole under a film of
    // explorers, whose height (`film_th`) is next to none.
    m.vis = 1.0 - smoothstep(0.004, 0.05, th);
    m.glow = vec3<f32>(0.0);
    m.gloss = mix((0.1 + 0.9 * sqrt(th)) * (1.0 - 0.6 * cut), 0.95, dense * (1.0 - cut));
    m.body = 0.0;
    m.gem = 0.0;
    if (kind == WET) {
        // Standing water over the soil: dark blue-green, glossy (darker
        // since 2026-11-14 with the ground); mould in it keeps its own colour.
        // Deep water's darkness is drawn at the pixel (`soil_detail`): a
        // darker cell colour here showed the cells' bilinear blend as a
        // stair inside the water.
        // Reeds (or any plant) on the water lie over it per pixel
        // (`shaded`), and dull its gloss by their cover.
        let water = WET_RGB + tint * 0.4;
        let over = clamp(1.0 - exp(-5.0 * th), 0.0, 1.0) * 0.85;
        m.albedo = mix(water, m.albedo, over);
        m.vis = min(m.vis, 1.0 - over);
        m.gloss = max(m.gloss, 0.75);
    } else if (kind == ROCK || kind == WALL || kind == DOOR) {
        // Under hard terrain, the soil: `terrain_over` lays the rock or
        // wall over it with an edge of its own, so the cells next to
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
    // Fire: its heat glows (an emissive, so it lights the dark).
    // The halo's colour, with the cell's own heat over it: the flames'
    // flicker and embers (`plants::Fire::flicker`, `ember`) show cell by
    // cell inside the glow (owner 11-14 19:26Z, "The fire should also look
    // better": with the halo alone a front read as a soft orange smear).
    // Fire is drawn per pixel in `shaded` (`plant_px`, `flame_body`): a
    // cell-level halo here (the 3 x 3 Gaussian, 18:20Z to 21:01Z) showed
    // as stair-steps at zoom 12 once the flame was light, not paint.
    return m;
}

// A material in two u32s: albedo and gloss, glow and body, a byte each
// (albedo and glow in colour units, gloss and body 0-1 as 0-255). `gem`
// is not drawn and not kept.
fn mat_pack(m: Mat) -> vec4<u32> {
    let a = vec4<u32>(vec4<f32>(clamp(m.albedo, vec3<f32>(0.0), vec3<f32>(255.0)), clamp(m.gloss, 0.0, 1.0) * 255.0) + 0.5);
    let g = vec4<u32>(vec4<f32>(clamp(m.glow, vec3<f32>(0.0), vec3<f32>(255.0)), clamp(m.body, 0.0, 1.0) * 255.0) + 0.5);
    let v = u32(clamp(m.vis, 0.0, 1.0) * 255.0 + 0.5);
    return vec4<u32>(a.x | a.y << 8u | a.z << 16u | a.w << 24u, g.x | g.y << 8u | g.z << 16u | g.w << 24u, v, 0u);
}

fn mc(ix: i32, iy: i32) -> Mat {
    let p = mcell[ridx(ix, iy)];
    var m: Mat;
    m.albedo = unpack(p.x);
    m.gloss = byte(p.x, 3u);
    m.glow = unpack(p.y);
    m.body = byte(p.y, 3u);
    m.gem = 0.0;
    m.vis = f32(p.z & 255u) / 255.0;
    return m;
}

// Works out each shown cell's height and material once a frame (a thread
// a cell of the rectangle). The walkers for `body` come through a 20 x 20
// tile in workgroup memory (the 16 x 16 cells and a 2-cell rim), loaded once:
// reading the 25 cells from the look for each cell cost the 1070 about
// 0.5 ms at zoom 1 (1920 x 1080; 1.19 -> 1.68 ms, matched fast runs).
var<workgroup> crowd: array<u32, 576>;
// The fine grid's second half on the same tile (`set_dense`).
var<workgroup> crowd2: array<u32, 576>;
// The tile's rows summed across (the Gaussians' first, horizontal pass;
// 24 rows of the workgroup's 16 columns): the sharp count, the cut-off
// count, the condensed count and the fresh absorptions under the sharp
// Gaussian, and the count under the body's.
var<workgroup> rows5: array<array<f32, 6>, 384>;

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
// The engine's scent at (x, y) (the look's byte 3, 1 - e^(-scent/60)),
// bilinear over the four cells round it.
fn look_scent(x: f32, y: f32) -> f32 {
    return look_byte(x, y, 3u);
}

// The look's byte k at (x, y), bilinear over the four cells round it.
fn look_byte(x: f32, y: f32, k: u32) -> f32 {
    let fx = x - 0.5; let fy = y - 0.5;
    let ix = i32(floor(fx)); let iy = i32(floor(fy));
    let tx = fx - f32(ix); let ty = fy - f32(iy);
    let a = mix(byte(look[cidx(ix, iy)], k), byte(look[cidx(ix + 1, iy)], k), tx);
    let b = mix(byte(look[cidx(ix, iy + 1)], k), byte(look[cidx(ix + 1, iy + 1)], k), tx);
    return mix(a, b, ty);
}

// The scent view (F2): what the mould smells, over the dimmed world. The
// food scent (byte 3, 1 - e^(-scent/60)) is a blue wash by its strength
// with a contour line every eighth of the way to full, a pixel wide at any
// zoom (the line's width is set in screen pixels from the field's slope,
// so it stays sharp close up, owner 11-14 20:21Z: play is zoomed in); the
// slime climbs those lines. The trail (byte 0, the engine's soft curve of
// it) is a pink wash: it shows trail where no walker stands, such as a
// road or a bait round's line, which the slime's own look (trail x
// walkers) does not draw. Nothing here is painted: both are the engine's
// fields as this frame's look holds them.
const SCENT_RGB = vec3<f32>(70.0, 170.0, 255.0);
const TRAIL_RGB = vec3<f32>(255.0, 90.0, 190.0);
const SCENT_LINES: f32 = 8.0;
fn scent_view(c0: vec3<f32>, x: f32, y: f32) -> vec3<f32> {
    var c = c0 * 0.32;
    let tr = look_byte(x, y, 0u);
    c = mix(c, TRAIL_RGB, clamp(tr, 0.0, 1.0) * 0.75);
    let s = look_byte(x, y, 3u);
    c = mix(c, SCENT_RGB, clamp(s, 0.0, 1.0) * 0.7);
    if (s > 0.01) {
        let h = 0.5;
        let gx = (look_byte(x + h, y, 3u) - look_byte(x - h, y, 3u)) / (2.0 * h);
        let gy = (look_byte(x, y + h, 3u) - look_byte(x, y - h, 3u)) / (2.0 * h);
        // The field's change a screen pixel, and the pixels to the nearest line.
        let per_px = max(length(vec2<f32>(gx, gy)) / P.zoom, 1.0e-5);
        let v = s * SCENT_LINES;
        let px = abs(fract(v + 0.5) - 0.5) / SCENT_LINES / per_px;
        let line = 1.0 - smoothstep(0.5, 1.5, px);
        c = mix(c, vec3<f32>(225.0, 245.0, 255.0), line * 0.85);
    }
    // The crystals stay as drawn (the food the scent comes from), their
    // hexagon's edge antialiased to a pixel: under the wash they vanished
    // and the scent's rings circled nothing (first film, 11-15).
    let k1 = gem_near(x, y);
    if (k1 > 0u) {
        let inside = clamp(gem_in(k1 - 1u, x, y).x * P.zoom + 0.5, 0.0, 1.0);
        c = mix(c, c0 * 0.9, inside);
    }
    return c;
}

// The engine's alarm on cell (ix, iy) (the germ fields' second block).
fn alarm_cell(ix: i32, iy: i32) -> f32 {
    let i = P.ww * P.wh + cidx(ix, iy);
    if (P.alarm_side == 0u) { return germ0[i]; }
    return germ1[i];
}

// Whether the engine keeps an alarm on cell (ix, iy): its agar, so not
// rock, wall or the world's rim (the engine's `trail` pass zeroes those).
fn alarm_open(ix: i32, iy: i32) -> f32 {
    if (ix <= 0 || iy <= 0 || ix + 1 >= i32(P.ww) || iy + 1 >= i32(P.wh)) { return 0.0; }
    let k = ground[cidx(ix, iy)] & 15u;
    return select(1.0, 0.0, k == ROCK || k == WALL || k == DOOR);
}

// The alarm at (x, y), bilinear over the open cells round it: x the
// alarm, y the open cells' share of the weight (0 deep in a wall).
// Plain bilinear (to 11-15) mixed a wall's 0 into the open cell beside
// it, so a plateau of alarm fell to nothing at every wall face and the
// haze's front term (it brightened the field's slope; gone 11-17) drew
// that fall as a bright straight line: the square edge of a blast's glow
// in the Fork's stem (the whole-game review's W27; the field itself is
// right, the walls hold none). The flush keeps the open weights, so slime
// against a wall flushes as the slime beside it does.
fn alarm_at(x: f32, y: f32) -> vec2<f32> {
    let fx = x - 0.5; let fy = y - 0.5;
    let ix = i32(floor(fx)); let iy = i32(floor(fy));
    let tx = fx - f32(ix); let ty = fy - f32(iy);
    let w = vec4<f32>((1.0 - tx) * (1.0 - ty) * alarm_open(ix, iy), tx * (1.0 - ty) * alarm_open(ix + 1, iy),
        (1.0 - tx) * ty * alarm_open(ix, iy + 1), tx * ty * alarm_open(ix + 1, iy + 1));
    let s = w.x + w.y + w.z + w.w;
    if (s <= 0.0) { return vec2<f32>(0.0, 0.0); }
    let a = vec4<f32>(alarm_cell(ix, iy), alarm_cell(ix + 1, iy), alarm_cell(ix, iy + 1), alarm_cell(ix + 1, iy + 1));
    return vec2<f32>(dot(w, a) / s, s);
}

// **The flinch, on the slime** (Look A, 2026-11-17). The alarm field is
// what a hit lays and the slime steers by (`BigParams::flinch`: it backs
// off ground a shot just struck; the Swarmer floods to it), so it is drawn
// where it acts, on the slime, as the slime's own colour: struck slime
// flushes hot, from its gold toward a deep orange with a little light of
// its own, by the alarm there (`FLUSH_KNEE` of the alarm that reads full
// is two-thirds of the way), for as long as the engine keeps the alarm (at
// the flinch's keep a shot's is gone in under a second). The hottest thing
// stays the crater's ember, the damage itself; the flush is the fainter
// warmth round it. It is worked out a fine cell in `prep`, with the
// slime's hue, and only where there is slime.
// Before: a pale cool haze by the alarm's strength with a brighter front
// on its slope, a pixel at a time in `shaded`, on bare ground and slime
// alike (11-14), scaled to a quarter on 11-15 (`set_alarm_look`, the
// whole-game review's W46: at full strength the fire zone washed mint),
// over a sharp green the engine painted on the same field. With the
// flinch on, every shot lays alarm, so the two together were a lime wash
// in cell-sized blobs over ground and slime round a firing squad, the
// largest thing on screen where the player aims (planning review, 11-17).
// Tried the same day and not kept: the slime blanching (toward pale
// straw; it washed the front out as the haze had) and dimming (pulling
// its light in; the fire zone read as a darker, dirtier slime).
// Bare ground shows no alarm now. A siren's call there (the siren is held
// out of the run, D10) shows by its lamp and by the slime it reaches.
const FLUSH_KNEE: f32 = 0.5;
const FLUSH_MUL: vec3<f32> = vec3<f32>(1.0, 0.76, 0.58);
const FLUSH_GLOW: vec3<f32> = vec3<f32>(24.0, 4.0, 0.0);
// How hot the slime at (x, y) is flushed, 0 to 1; `th` its thickness as
// its colour takes it (`prep`'s `tm`).
fn flush_at(x: f32, y: f32, th: f32) -> f32 {
    if (P.alarm_full <= 0.0 || th <= 0.004) { return 0.0; }
    let a = alarm_at(x, y);
    return (1.0 - exp(-a.x / (FLUSH_KNEE * P.alarm_full))) * a.y * smoothstep(0.004, 0.05, th);
}
fn flush_mat(m0: Mat, k: f32) -> Mat {
    if (k <= 0.0) { return m0; }
    var m = m0;
    m.albedo = mix(m.albedo, m.albedo * FLUSH_MUL, k);
    m.glow += FLUSH_GLOW * k;
    return m;
}

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

// ---- Mini-boss bodies (2026-11-16) ----
//
// A mini-boss is a lump of the slime's own walkers held together and
// steered as one (`mould::big` lumps). Its look is those walkers: `prep`
// counts them on their own (`lcb`), and where they are a share of the
// mould round a cell, the body carries the lump's material over the
// ordinary slime's, by that share, so there is no drawn outline: a lump
// forming among the sheet shows as its members' colour spreading through
// it, and a held body is a solid patch of it. The members' own density
// (under the body's wide Gaussian) is its shape: it stands up as a glossy
// dome where they are thick, with a darker, wetter membrane where their
// cover falls off, and it lights from inside where they are thickest, so a
// body that clenches or bunches as it lunges shows that as it happens.
// Every member is also its own dot (`draw_specks`), so the body's churn is
// its walkers moving. The owner (11-16): from the simulation, not drawn on
// top; not bronze; varied per kind; stronger ones read so.
//
// Tried and dropped (11-16, independent review): the core lit where the
// members' sharp count stood above their wide one (their clumps). At 0.5
// members a cell the sharp window holds about one member, so the ratio is
// shot noise: about 30% of the body crossed the threshold by chance and
// re-rolled every frame, and the body boiled. The wide count is steady.
struct Lump { share: f32, look: u32, strength: f32, core: f32, rise: f32, cover: f32, fill: f32 };

// Each look's skin (the body's albedo), core light (emissive, where its
// members are thickest) and hit colour (a member's dot as it takes hits).
// Violet is the hunter's: the one hue the screen did not use (the slime
// gold, bronze, the panic's amber and red, crystal cyan, the soldiers'
// blue, olive ground), the complement of the gold it forms inside, and it
// holds up for red-green colour blindness. Ember (crimson over orange) is
// kept for the A/B: its dots sat near the panic's amber and its red near
// the bolt's (review, 11-16). Ink is near-black over a pale lilac core, for
// kinds told apart by value rather than hue. Bone is a pale grey-lilac grub
// over a faint warm core: the palest thing on the slime, so it is told
// apart by value too; on so pale a skin a cold-white hit would not show,
// so its hits go rose, like a bruise. The game gives each kind its look
// (Larder World's `kind_look`): hunter violet, wanderer bone, sapper ember,
// leech ink.
fn lump_skin(k: u32) -> vec3<f32> {
    switch k {
        case 1u: { return vec3<f32>(96.0, 14.0, 26.0); }
        case 2u: { return vec3<f32>(120.0, 56.0, 190.0); }
        case 3u: { return vec3<f32>(26.0, 20.0, 34.0); }
        case 4u: { return vec3<f32>(140.0, 134.0, 152.0); }
        default: { return MOULD_HUE; }
    }
}
fn lump_core(k: u32) -> vec3<f32> {
    switch k {
        case 1u: { return vec3<f32>(255.0, 120.0, 60.0); }
        case 2u: { return vec3<f32>(220.0, 150.0, 255.0); }
        case 3u: { return vec3<f32>(200.0, 180.0, 255.0); }
        case 4u: { return vec3<f32>(120.0, 90.0, 74.0); }
        default: { return vec3<f32>(0.0); }
    }
}
// A hit member: cold white, away from the shot spray's warm white-hot.
fn lump_hit(k: u32) -> vec3<f32> {
    switch k {
        case 1u: { return vec3<f32>(255.0, 250.0, 235.0); }
        case 4u: { return vec3<f32>(235.0, 64.0, 118.0); }
        default: { return vec3<f32>(225.0, 245.0, 255.0); }
    }
}
// A lump's core colour as it has drunk (`fill`, a leech's take from the
// pump it sits on): toward the crystals' charge cyan, the colour of what
// it took (the only cyan in the world is charge).
fn lump_glow(k: u32, fill: f32) -> vec3<f32> {
    return mix(lump_core(k), CHARGE, clamp(fill, 0.0, 1.0));
}
const LUMP_GLOSS: f32 = 0.9;
const LUMP_RISE: f32 = 1.1;
// How bright the core light burns at full (0.9 in round 1). Since the
// mini-boss engine's tighter clench (11-16, round 3) a body that sits (a
// sapper at its wall, a wanderer rooted) packs past `LUMP_HOT` all over,
// and at 0.7 its core light swamped its skin: the ember sapper read salmon,
// the bone wanderer clipped white. Now the skin is the read and the light
// a glow under it; the members' own dots carry the bright points.
const LUMP_CORE: f32 = 0.4;
// A leech's glow by its fill, over `LUMP_CORE`'s (1.5 x 0.4: at 2.5 a
// leech two-thirds full washed its pump pale cyan and its ink was lost).
const LUMP_DRINK: f32 = 1.5;
// Members a cell at which a lump's own cover is 1 - 1/e (its dome). A
// held hunter of 420 is about 25 x 35 cells, 0.5-0.6 members a cell: far
// thinner than the sheet it left (`BODY_N`), so its cover is counted on
// its own scale; at 0.5 a cell the cover was 0.98 at 2 members a cell, so
// a denser body read the same.
const LUMP_N: f32 = 0.35;
// The cover over which a held lump's own members carry its look whatever
// the sheet round them (`lump_here`): 0.2 is 0.08 members a cell, 0.8 is
// 0.56.
const LUMP_OWN: vec2<f32> = vec2<f32>(0.2, 0.8);
// Members a cell (wide) where the core light starts and is full ((0.55,
// 1.3) in round 1, for a loose 0.5-a-cell body; raised with the clench,
// so a held body glows where it packs and a lunge's clench lights it).
const LUMP_HOT: vec2<f32> = vec2<f32>(0.65, 1.5);
// A member's energy at which its dot burns full, and how much of its core
// colour the dot lays over the body at full energy.
const LUMP_FULL: f32 = 1.5;
const LUMP_SPECK: f32 = 0.75;

// The lump at world point (x, y): `share` its members over all walkers
// (both under the body's wide Gaussian), `wide` its members a cell, and
// `flags` the lumps with members in the 5 x 5 fine cells round (bits 0-3,
// `FG_FLAGS`): the look is that lump's, or of the nearest flagged centre
// where two meet (so two lumps side by side meet where their members do,
// not on a line between their centres).
fn lump_here(x: f32, y: f32, share: f32, wide: f32, flags: u32) -> Lump {
    var l: Lump;
    l.share = 0.0; l.look = 0u; l.strength = 0.0; l.core = 0.0; l.rise = 0.0; l.cover = 0.0; l.fill = 0.0;
    if (P.n_lumps == 0u || wide <= 1e-3) { return l; }
    var best = 1e30;
    var bk = 0u;
    for (var k = 0u; k < 4u; k++) {
        let q = P.lumps[k];
        if (q.z < 0.5) { continue; }
        // Unflagged lumps only when no lump is flagged here (the wide
        // Gaussian's tail past the members' last cells).
        let far = select(0.0, 1e20, flags != 0u && (flags & (1u << k)) == 0u);
        let d = dot(q.xy - vec2<f32>(x, y), q.xy - vec2<f32>(x, y)) + far;
        if (d < best) { best = d; l.look = u32(q.z + 0.5); l.strength = q.w; bk = k; }
    }
    if (l.look == 0u) { return l; }
    // How much of its look is left (a broken or spent lump fading, its
    // members still marked): the light goes out first (by the square), the
    // body sinks and its colour drains back to the slime's with it, so it
    // comes apart into the sheet rather than popping back to gold.
    let rem = clamp(P.lump_fade[bk], 0.0, 1.0);
    l.fill = P.lump_fill[bk];
    // By the body's wide Gaussian: by the sharp one the skin's edge
    // changed within a fine cell and drew as stairs at zoom 24.
    l.cover = 1.0 - exp(-wide / LUMP_N);
    // A held lump (out on its errand) shows wherever its own members
    // stand thick enough (`LUMP_OWN`, by their cover), however thick the
    // sheet they stand in: a leech ringing its pump (mini-boss #396) puts
    // 123 members in a band 14-18 cells round the pump, about 0.36 a cell
    // in the thick sheet that grows there, and by its share of the walkers
    // alone it showed only as dark flecks in a raised gold collar. A
    // gathering one still seeps through by its share. A stray member (0.1
    // a cell, cover 0.25) adds nothing.
    let own = smoothstep(LUMP_OWN.x, LUMP_OWN.y, l.cover) * clamp(P.lump_held[bk], 0.0, 1.0);
    l.share = max(smoothstep(0.05, 0.7, clamp(share, 0.0, 1.0)), own) * rem;
    // A leech's drink lights the whole of it (`LUMP_DRINK`), wherever its
    // members are, not only where they pack: on its pump it sits under the
    // rig and on the crystal, and only a glow round them showed it there.
    // So a full one reads as worth breaking (its take goes back to the
    // bank).
    l.core = l.share * (smoothstep(LUMP_HOT.x, LUMP_HOT.y, wide) * (0.6 + 0.4 * l.strength) + LUMP_DRINK * l.fill) * rem;
    l.rise = LUMP_RISE * l.share * l.cover * l.cover * (0.7 + 0.3 * l.strength);
    return l;
}

// The material over a cell where a lump is (`lump_here`): its skin by its
// share, its gloss, and its core's light.
fn lump_mat(m0: Mat, l: Lump, th: f32) -> Mat {
    if (l.look == 0u || l.share <= 0.0) { return m0; }
    var m = m0;
    let s = l.share * smoothstep(0.02, 0.2, th);
    // Darker where its members thin out, so its edge is theirs: a dark,
    // wet membrane where their cover falls off (`rim`, its slope's middle),
    // lighter inside where they are thick.
    let rim = 4.0 * l.cover * (1.0 - l.cover);
    let skin = lump_skin(l.look) * (0.55 + 0.55 * smoothstep(0.3, 0.9, l.cover)) * (1.0 - 0.45 * rim);
    m.albedo = mix(m.albedo, skin, s);
    m.gloss = mix(m.gloss, min(LUMP_GLOSS + 0.1 * rim, 1.0), s);
    m.glow = m.glow * (1.0 - s) + lump_glow(l.look, l.fill) * LUMP_CORE * l.core;
    return m;
}

// **Set slime** (the simulation-use plan's E2b, 2026-11-17;
// `BigDish::set_cells`): walkers that stopped where they were hurt are drawn
// by their own state, a material over the body where they stand: amber, a
// deep resin with a low warm glow inside (set slime as hardened, dried
// sap). It was one side of a blind A/B to 2026-11-18, settled by the Look
// and interface lane from frames of planet 3's walls at zoom 6: the other
// side, a pale glass-glossy glaze toward (255, 238, 196), read as bare
// sand and is the team's cream and the wanderer's pale bone; and with no
// mark a wall that stops soldiers looked like slime they can walk
// (`Reports/dead-ends.md`). Not dark ridges (they read as the player's
// walls), not bronze (the owner turned it down). The body's opacity stays
// the sheet's; its edge is the set walkers' own, smoothed over the 4 x 4
// cells round (a lone set walker is a fleck, a set rim a band).
// It stands `SET_RISE` above the sheet (2026-11-18). To then it kept the
// sheet's height too (the graphics thread, 11-17), which was a detail while
// it formed only under fire; the Trunk's fort (Enemy, #473) made it a ring
// round every crystal the slime holds, and flat it read as a rust stain or
// a drawn target on the slime, not a wall a soldier cannot cross. Raised,
// the light gives it a lit flank and a shaded one, as a wall.
fn set_c(ix: i32, iy: i32) -> f32 { return f32(news[P.setc_at + cidx(ix, iy)]); }
fn set_share(x: f32, y: f32) -> f32 {
    if (P.setc_at == 0u) { return 0.0; }
    let ix = i32(floor(x - 0.5)) - 1; let iy = i32(floor(y - 0.5)) - 1;
    var c = 0.0;
    var ws = 0.0;
    for (var dy = 0; dy < 4; dy++) {
        for (var dx = 0; dx < 4; dx++) {
            let d = vec2<f32>(f32(ix + dx) + 0.5 - x, f32(iy + dy) + 0.5 - y);
            let w = exp(-dot(d, d) / (2.0 * 0.7 * 0.7));
            c += w * set_c(ix + dx, iy + dy);
            ws += w;
        }
    }
    return smoothstep(0.2, 0.6, c / ws);
}
const SET_AMBER: vec3<f32> = vec3<f32>(112.0, 46.0, 14.0);
const SET_AMBER_GLOW: vec3<f32> = vec3<f32>(255.0, 110.0, 30.0);
// How far set slime stands above the sheet: the dense body is at most 5.
const SET_RISE: f32 = 4.0;
fn set_rise(share: f32, th: f32) -> f32 { return SET_RISE * share * smoothstep(0.02, 0.2, th); }
fn set_mat(m0: Mat, share: f32, th: f32) -> Mat {
    if (share <= 0.0) { return m0; }
    var m = m0;
    let s = share * smoothstep(0.02, 0.2, th);
    m.albedo = mix(m.albedo, SET_AMBER, s);
    m.gloss = mix(m.gloss, 0.5, s);
    m.glow = m.glow + SET_AMBER_GLOW * 0.14 * s;
    return m;
}

// How far beyond the map's edge `prep` works the ground out, in cells.
const OFF_MAP: f32 = 8.0;

@compute @workgroup_size(16, 16)
fn prep(@builtin(global_invocation_id) gid: vec3<u32>, @builtin(local_invocation_id) lid: vec3<u32>, @builtin(workgroup_id) wid: vec3<u32>, @builtin(local_invocation_index) li: u32) {
    // The walkers a fine cell (and, above bit 16, the cut-off ones) on a
    // 24 x 24 tile: the workgroup's 16 x 16 and a 4-cell rim.
    let bx = i32(wid.x * 16u) - 4;
    let by = i32(wid.y * 16u) - 4;
    // Fine cells well beyond the map's edge (the dark round a map smaller
    // than the screen, which is rock) are never read: `base` draws a pixel
    // the rock covers whole as the rock's top alone, and the ground's
    // samples near the edge reach a few cells out at most (the slope 2,
    // the cast shadow 3, the sheen 1.5). Their terrain is still written
    // (`tcell`, which the rock's top reads); the rest is skipped, a whole
    // workgroup at a time before its tile. (Performance thread, 11-15: at
    // zoom 1 on the fork, three quarters of `prep` was off the map.)
    let kf0 = f32(P.fine);
    let lo = vec2<f32>(f32(P.ox), f32(P.oy)) + vec2<f32>(f32(wid.x * 16u), f32(wid.y * 16u)) / kf0;
    let hi = lo + 16.0 / kf0;
    let wgf = vec2<f32>(f32(P.ww), f32(P.wh));
    let group_off = any(hi < vec2<f32>(-OFF_MAP)) || any(lo > wgf + OFF_MAP);
    if (group_off) {
        if (gid.x >= P.rw || gid.y >= P.rh) { return; }
        if (gid.x % P.fine == 0u && gid.y % P.fine == 0u) {
            tcell[(gid.y / P.fine) * (P.rw / P.fine) + gid.x / P.fine] = ter_prep(P.ox + i32(gid.x / P.fine), P.oy + i32(gid.y / P.fine));
        }
        return;
    }
    for (var k = li; k < 576u; k += 256u) {
        let fx = bx + i32(k % 24u); let fy = by + i32(k / 24u);
        var v = 0u;
        var v2 = 0u;
        if (fx >= 0 && fy >= 0 && fx < i32(P.rw) && fy < i32(P.rh)) {
            v = atomicLoad(&fgrid[u32(fy) * P.rw + u32(fx)]);
            if (P.dense_at > 0u) { v2 = atomicLoad(&fgrid[P.fhalf + u32(fy) * P.rw + u32(fx)]); }
        }
        crowd[k] = v;
        crowd2[k] = v2;
    }
    workgroupBarrier();
    let kf = f32(P.fine);
    var sig = SHARP_S * kf;
    if (P.fine == 1u) { sig = SHARP_S1; }
    let sigb = BODY_S * kf;
    // Gaussians are separable: nine weights a side each, not 81 exps.
    var g1: array<f32, 9>;
    var g2: array<f32, 9>;
    var s1 = 0.0;
    var s2 = 0.0;
    for (var q = 0; q < 9; q++) {
        let d2 = f32((q - 4) * (q - 4));
        g1[q] = exp(-d2 / (2.0 * sig * sig));
        g2[q] = exp(-d2 / (2.0 * sigb * sigb));
        s1 += g1[q];
        s2 += g2[q];
    }
    // And applied separably: each of the tile's 24 rows summed across its
    // nine columns once (`rows5`), then nine rows down a cell. Before
    // (to 11-15) each fine cell took all 81 of its window's tile words, two
    // thirds of `prep` at zoom 1 on llvmpipe (33 of 59 ms, planet 8); the
    // sums differ only in the float rounding of their order.
    for (var k = li; k < 384u; k += 256u) {
        let r = k / 16u;
        let c0 = k % 16u;
        var a = array<f32, 6>(0.0, 0.0, 0.0, 0.0, 0.0, 0.0);
        for (var dx = 0u; dx < 9u; dx++) {
            let t = crowd[r * 24u + c0 + dx];
            let c = f32(t & 0xFFFFu) / 15.0;
            let t2 = crowd2[r * 24u + c0 + dx];
            a[0] += g1[dx] * c;
            a[1] += g1[dx] * f32(t >> 16u) / 15.0;
            a[2] += g1[dx] * f32(t2 & FG_DENSE) / 15.0;
            a[3] += g1[dx] * f32(t2 >> FG_ABSORB);
            a[4] += g2[dx] * c;
            // A mini-boss's members, under the body's Gaussian (`lump_here`).
            a[5] += g2[dx] * f32((t2 >> FG_LUMP) & FG_LUMP_MASK);
        }
        rows5[k] = a;
    }
    workgroupBarrier();
    if (gid.x >= P.rw || gid.y >= P.rh) { return; }
    let i = gid.y * P.rw + gid.x;
    var n = 0.0;
    var m = 0.0;
    var nb = 0.0;
    var dn = 0.0;
    var ab = 0.0;
    var lcb = 0.0;
    for (var dy = 0u; dy < 9u; dy++) {
        let a = rows5[(lid.y + dy) * 16u + lid.x];
        n += g1[dy] * a[0];
        m += g1[dy] * a[1];
        dn += g1[dy] * a[2];
        ab += g1[dy] * a[3];
        nb += g2[dy] * a[4];
        lcb += g2[dy] * a[5];
    }
    let ws = s1 * s1;
    let wb = s2 * s2;
    let cut_share = clamp(m / max(n, 1e-3), 0.0, 1.0);
    // The share of the mould's mass round the cell that has condensed (a
    // condensed walker counts two in `n`).
    let dense_share = clamp(2.0 * dn / max(n, 1e-3), 0.0, 1.0);
    // Fresh absorptions round the cell (the last 0.3 s, `draw_specks`)
    // light it warm-white by how many (art director, 11-14: a big
    // condensation flares, a trickle glints), a fine cell's worth of them
    // near full; it cools into the condensed colour as they age out.
    let flash = 1.0 - exp(-0.6 * ab / ws * kf * kf);
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
    let th0 = look_trail(wx, wy) * mix(near, wide, dense);
    // Condensed mould keeps tight, raised edges (art director, 11-14: it
    // should read tougher at a glance): where it has condensed the
    // thickness is pushed through a steep curve, so the body is a full,
    // smooth plateau with a short edge (its highlight sits tight on the
    // rim), and stands up to a half taller (`th_h`, the height only). Taking
    // more of the sharp count instead made the inside lumpy and glittering.
    let th = mix(th0, smoothstep(0.04, 0.3, th0), 0.45 * dense_share);
    // The height's thickness comes in with the wide count (art director,
    // 11-14 08:15Z: the thin frontier read as bubble wrap, each lone walker
    // a dome with a cast shadow and a glint). A walker alone (wide about
    // 0.4) is a point of light (its speck) on a flat film; the height
    // rises as walkers gather, full in the body (wide 0.6 and up).
    // Settled condensed mould keeps its veins (art director, 11-14
    // follow-up: the plateau read as a flat plate): its height takes the
    // raw thickness `th0`, veins and all, raised by up to 0.6, so the broad
    // sheen runs over it as over dense wet metal; only its colour takes the
    // plateau. The first cut fed the plateau to the height too.
    var th_h = mix(th, th0 * (1.0 + 0.6 * dense_share), dense_share) * smoothstep(0.3, 0.6, wide);
    // A mini-boss's body (2026-11-16): its members' share of the walkers
    // round the cell (a member counts once in `n` and once in `lc`), and the
    // nearest lump's look. Where its members are, it stands up from the
    // sheet as a raised, rounder body (by their share and how tightly they
    // pack, `near`), so it casts its own shadow and catches the sheen.
    // Which lumps have members in the 5 x 5 fine cells round (the tile's
    // flags, `FG_FLAGS`), read only where members are.
    var flags = 0u;
    if (lcb > 1e-3) {
        for (var fy = 2u; fy < 7u; fy++) {
            for (var fx = 2u; fx < 7u; fx++) {
                flags |= (crowd2[(lid.y + fy) * 24u + lid.x + fx] >> FG_FLAGS) & 15u;
            }
        }
    }
    let lump = lump_here(wx, wy, lcb / max(nb, 1e-3), lcb / wb * kf * kf, flags);
    th_h = th_h + lump.rise;
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
    // The alarm on the slime here (`flush_at`), under a mini-boss's skin
    // and the set slime's, which keep their own colours.
    let hot = flush_at(wx, wy, tm);
    let ss = set_share(wx, wy);
    if (P.fine == 1u) {
        let ix = i32(floor(wx)); let iy = i32(floor(wy));
        let hh = cell_height(ix, iy, th_h);
        hcell[i] = hh.x + set_rise(ss, tm);
        hcell[P.rw * P.rh + i] = hh.y;
        hcell[2u * P.rw * P.rh + i] = ground_pack(ground_cell(ix, iy).col);
        var m1 = cell_mat(ix, iy, tm, cut_share, dense_share);
        m1.body = wide;
        m1.glow += ABSORB_RGB * 0.45 * flash;
        m1 = flush_mat(m1, hot);
        m1 = lump_mat(m1, lump, tm);
        m1 = set_mat(m1, ss, tm);
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
    var h = vec2<f32>(0.0);
    var gcol = vec3<f32>(0.0);
    var mt: Mat;
    mt.albedo = vec3<f32>(0.0); mt.glow = vec3<f32>(0.0); mt.gloss = 0.0; mt.body = wide; mt.gem = 0.0; mt.vis = 0.0;
    let cs = colour_at(wx, wy);
    for (var q = 0; q < 4; q++) {
        let wq = w4[q];
        if (wq <= 0.0) { continue; }
        let cx = ix + o[q].x; let cy = iy + o[q].y;
        h += wq * cell_height(cx, cy, th_h);
        gcol += wq * ground_cell(cx, cy).col;
        let mq = cell_mat_c(cx, cy, tm, cut_share, dense_share, cs);
        mt.albedo += wq * mq.albedo; mt.glow += wq * mq.glow; mt.gloss += wq * mq.gloss; mt.vis += wq * mq.vis;
    }
    mt.glow += ABSORB_RGB * 0.45 * flash;
    mt = flush_mat(mt, hot);
    mt = lump_mat(mt, lump, tm);
    mt = set_mat(mt, ss, tm);
    hcell[i] = h.x + set_rise(ss, tm);
    hcell[P.rw * P.rh + i] = h.y;
    hcell[2u * P.rw * P.rh + i] = ground_pack(gcol);
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
    m.vis = mix(mix(a.vis, b.vis, tx), mix(c.vis, d.vis, tx), ty);
    return m;
}

// The wide walker count under the last `shaded` point, for the specks' light.
var<private> under_body: f32;

// The shaded colour at world point (x, y); `e` is the slope's sample step.
// ---- Soil materials (art lead, 11-14, review issue 7 on the planets) ----
//
// The soil kinds were flat colours, so the planets' biomes (built from
// them) looked alike in a grey thumbnail and zoom 6 was a flat field. Each
// soil now has a material read from the soil field at the pixel: the
// bilinear share of each kind over the four cells round it, so a patch's
// texture fades out over its own edge, not on a cell line. The patterns
// are fixed in world cells (they do not crawl with the camera) and fade
// out where they would alias (under about 3 pixels a feature) and under
// the mould, whose film covers them.
fn hash2(p: vec2<f32>) -> f32 {
    let q = fract(p * vec2<f32>(0.1031, 0.1030));
    let r = q + dot(q, q.yx + 33.33);
    return fract((r.x + r.y) * r.x);
}

// Smooth value noise in 0..1, one lattice point a unit.
fn vnoise(p: vec2<f32>) -> f32 {
    let i = floor(p);
    let f = p - i;
    let u = f * f * (3.0 - 2.0 * f);
    return mix(mix(hash2(i), hash2(i + vec2<f32>(1.0, 0.0)), u.x), mix(hash2(i + vec2<f32>(0.0, 1.0)), hash2(i + vec2<f32>(1.0, 1.0)), u.x), u.y);
}

// The share of sand, clay, barren and wet ground at (x, y), a cubic
// B-spline over the 4 x 4 cells round (no cell stair in a patch's edge),
// and the wet share's gradient (per cell), for the water's crisp edge.
struct Soil { w: vec4<f32>, wg: vec2<f32>, deep: f32, bridge: f32, bg: vec2<f32> };
fn soil_share(x: f32, y: f32) -> Soil {
    var o: Soil;
    o.w = vec4<f32>(0.0); o.wg = vec2<f32>(0.0);
    o.deep = 0.0; o.bridge = 0.0; o.bg = vec2<f32>(0.0);
    let ux = x - 0.5; let uy = y - 0.5;
    let ix = i32(floor(ux)); let iy = i32(floor(uy));
    let fx = ux - floor(ux); let fy = uy - floor(uy);
    let wx = bc(fx); let wy = bc(fy);
    let dx = bcd(fx); let dy = bcd(fy);
    for (var j = 0; j < 4; j++) {
        for (var i = 0; i < 4; i++) {
            let cx = clamp(ix - 1 + i, 0, i32(P.ww) - 1); let cy = clamp(iy - 1 + j, 0, i32(P.wh) - 1);
            let g = ground[cidx(cx, cy)];
            let k = g & 15u;
            let sk = (g >> 4u) & 7u;
            var e = vec4<f32>(0.0);
            if (k == WET) { e.w = 1.0; }
            else if (sk == 1u) { e.x = 1.0; } else if (sk == 2u) { e.y = 1.0; } else if (sk == 3u) { e.z = 1.0; }
            let wt = wx[i] * wy[j];
            let dw = vec2<f32>(dx[i] * wy[j], wx[i] * dy[j]);
            o.w += e * wt;
            o.wg += e.w * dw;
            // Deep water (soil 4 with the wet bit) and bridges (soil 5;
            // ground kinds thread, #285).
            if (k == WET && sk == SOIL_DEEP) { o.deep += wt; }
            if (k != WET && sk == SOIL_BRIDGE) { o.bridge += wt; o.bg += dw; }
        }
    }
    return o;
}

// The water at (x, y), bilinear over the 2 x 2 cells round it: the wet
// share (any water) and the deep share. For the plants over water
// (`shaded`); the water's own crisp edge is `soil_share`'s.
fn water_share(x: f32, y: f32) -> vec2<f32> {
    let ux = x - 0.5; let uy = y - 0.5;
    let ix = i32(floor(ux)); let iy = i32(floor(uy));
    let f = vec2<f32>(ux - floor(ux), uy - floor(uy));
    var o = vec2<f32>(0.0);
    for (var j = 0; j < 2; j++) {
        for (var i = 0; i < 2; i++) {
            let g = ground[cidx(clamp(ix + i, 0, i32(P.ww) - 1), clamp(iy + j, 0, i32(P.wh) - 1))];
            let wt = mix(1.0 - f.x, f.x, f32(i)) * mix(1.0 - f.y, f.y, f32(j));
            if ((g & 15u) == WET) {
                o.x += wt;
                if (((g >> 4u) & 7u) == SOIL_DEEP) { o.y += wt; }
            }
        }
    }
    return o;
}

// How much thinner plants are drawn over water (their density for the
// crowns' size, so the stems shrink and the water shows between).
const WATER_THIN: f32 = 0.65;
// And how much of their flat far-view cover is taken off over water.
const WATER_VEIL: f32 = 0.6;

// Is cell (ix, iy) deep water?
fn deep_at(ix: i32, iy: i32) -> f32 {
    let g = ground[cidx(clamp(ix, 0, i32(P.ww) - 1), clamp(iy, 0, i32(P.wh) - 1))];
    return select(0.0, 1.0, (g & 15u) == WET && ((g >> 4u) & 7u) == SOIL_DEEP);
}

// How far into deep water (x, y) is, 0 at its edge to 1 about five cells
// in: the share of deep cells on two rings round it (4 points at 2 cells,
// 8 at 5). The river reads as deepening toward its middle, not one flat
// band. Its cost on llvmpipe (River, 1280 x 800): three rings of 8 points
// worked out with sin and cos about 7 ms a frame; these 12 constants less
// (Reports/river-look-2026-11-14).
const DEEP_RING: array<vec2<f32>, 12> = array<vec2<f32>, 12>(vec2<f32>(1.84, 0.78), vec2<f32>(-0.78, 1.84), vec2<f32>(-1.84, -0.78), vec2<f32>(0.78, -1.84), vec2<f32>(5.00, 0.00), vec2<f32>(3.54, 3.54), vec2<f32>(0.00, 5.00), vec2<f32>(-3.54, 3.54), vec2<f32>(-5.00, 0.00), vec2<f32>(-3.54, -3.54), vec2<f32>(-0.00, -5.00), vec2<f32>(3.54, -3.54));
fn deep_depth(x: f32, y: f32) -> f32 {
    var n = 0.0;
    for (var k = 0; k < 12; k++) {
        let q = vec2<f32>(x, y) + DEEP_RING[k];
        n += deep_at(i32(floor(q.x)), i32(floor(q.y)));
    }
    return n / 12.0;
}

// The bare ground's colour at a cell as `cell_mat_c` makes it (soil by
// kind and elevation; water), with no tint. (Plants and char are drawn at
// the pixel over it since the plants thread's crowns.)
struct GroundB { col: vec3<f32>, elev: f32 };
fn ground_cell(ix: i32, iy: i32) -> GroundB {
    var o: GroundB;
    let cx = clamp(ix, 0, i32(P.ww) - 1); let cy = clamp(iy, 0, i32(P.wh) - 1);
    let i = cidx(cx, cy);
    let g = ground[i];
    let kind = g & 15u;
    let sk = (g >> 4u) & 7u;
    o.elev = byte(g, 3u);
    var base = plain_soil(o.elev);
    if (sk == 1u) { base = SAND_RGB; } else if (sk == 2u) { base = CLAY_RGB; } else if (sk == 3u) { base = BARREN_RGB; }
    if (kind == RUBBLE) { base = rubble_rgb(ix, iy); }
    if (kind == WET) { base = WET_RGB; }
    o.col = base;
    return o;
}

// `ground_cell` blended bilinearly between the four cells round (x, y),
// as the cells' material is.
// The bare ground's colour a fine cell, as `prep` blends `cell_mat_c`'s
// (`ground_cell`'s colour by the same weights), in the third grid of
// `hcell`: r, g, b packed as one whole number under 2^24, which an f32
// holds exactly. `soil_detail` reads it bilinear, as `mat_at` reads the
// material, so the difference it adds cancels the material's own cell
// steps exactly. (First cuts worked it out at the pixel: a bilinear of the
// four cells round left a cell-wide scallop along waterlines at zoom 12,
// since at `fine` 2 the material is a blend of blends; the exact weights,
// nine `ground_cell`s a pixel, cost 40% of the frame on llvmpipe.)
fn ground_pack(c: vec3<f32>) -> f32 {
    let q = vec3<u32>(clamp(c, vec3<f32>(0.0), vec3<f32>(255.0)) + 0.5);
    return f32(q.x * 65536u + q.y * 256u + q.z);
}
fn hg(ix: i32, iy: i32) -> vec3<f32> {
    let u = u32(hcell[2u * P.rw * P.rh + ridx(ix, iy)]);
    return vec3<f32>(f32(u >> 16u), f32((u >> 8u) & 255u), f32(u & 255u));
}
fn ground_est(x: f32, y: f32) -> vec3<f32> {
    let f = fpos(x, y);
    let ix = i32(floor(f.x)); let iy = i32(floor(f.y));
    let tx = f.x - floor(f.x); let ty = f.y - floor(f.y);
    return mix(mix(hg(ix, iy), hg(ix + 1, iy), tx), mix(hg(ix, iy + 1), hg(ix + 1, iy + 1), tx), ty);
}
// The land's elevation at (x, y), bilinear over the four cells round, for
// the plain soil's colour.
fn ground_elev(x: f32, y: f32) -> f32 {
    let fx = x - 0.5; let fy = y - 0.5;
    let ix = i32(floor(fx)); let iy = i32(floor(fy));
    let tx = fx - floor(fx); let ty = fy - floor(fy);
    var e = array<f32, 4>();
    for (var q = 0; q < 4; q++) {
        let i = cidx(clamp(ix + (q & 1), 0, i32(P.ww) - 1), clamp(iy + (q >> 1u), 0, i32(P.wh) - 1));
        e[q] = byte(ground[i], 3u);
    }
    return mix(mix(e[0], e[1], tx), mix(e[2], e[3], tx), ty);
}

// What each soil does to the ground's albedo at (x, y), `bare` the share
// not under mould: (multiplier, added light).
// The plain soil's micro-look on each biome (`soil_detail`), a signed
// share of its colour. Every grid is turned off the cells' axes (value
// noise on the axes shows squares at zoom 24) and costs one to three
// value noises a pixel; the highlands' gravel adds one 3 x 3 cell search.
fn plain_look(fl: u32, p: vec2<f32>) -> f32 {
    let q1 = vec2<f32>(0.8 * p.x - 0.6 * p.y, 0.6 * p.x + 0.8 * p.y);
    let q2 = vec2<f32>(0.28 * p.x + 0.96 * p.y, -0.96 * p.x + 0.28 * p.y);
    if (fl == 1u) {
        // Barrens: dry grit, fine pale and dark specks, and faint wind
        // streaks drawn out along one direction.
        let n = vnoise(q1 * 3.2 + 9.0);
        let speck = 0.10 * smoothstep(0.66, 0.86, n) - 0.08 * smoothstep(0.66, 0.86, 1.0 - n);
        let streak = 0.05 * (vnoise(vec2<f32>(q2.x * 0.25, q2.y * 2.2) + 4.0) - 0.5) * 2.0;
        // A hardpan of large faint polygons, about four cells across,
        // their seams a pixel or two (art lead 11-15).
        let g = p / 4.0;
        let gi = floor(g);
        var d1 = 9.0; var d2 = 9.0;
        for (var j = -1; j <= 1; j++) {
            for (var i = -1; i <= 1; i++) {
                let c = gi + vec2<f32>(f32(i), f32(j));
                let o = c + vec2<f32>(hash2(c * 1.9 + 0.7), hash2(c * 3.1 + 4.3)) * 0.8 + 0.1;
                let d = length(g - o);
                if (d < d1) { d2 = d1; d1 = d; } else if (d < d2) { d2 = d; }
            }
        }
        let seam = 1.0 - smoothstep(0.8, 2.2, (d2 - d1) * 4.0 * P.zoom);
        return 0.7 * speck + streak - 0.10 * seam;
    }
    if (fl == 2u) {
        // Fen: mud, darker wet blotches a couple of cells across, with a
        // few wet glints in them.
        let b = smoothstep(0.42, 0.72, vnoise(q1 / 1.5 + 3.0));
        let glint = smoothstep(0.86, 0.96, vnoise(q2 * 3.4 + 15.0));
        return -0.14 * b + 0.16 * glint * b;
    }
    if (fl == 3u) {
        // Valley: river silt laid in soft bands, bent, with pale pebbles.
        let band = 0.06 * sin(6.2831853 * (q2.x / 2.3 + 1.4 * vnoise(p / 9.0 + 2.0)));
        let peb = 0.12 * smoothstep(0.78, 0.9, vnoise(q1 * 1.4 + 40.0));
        return band + peb;
    }
    if (fl == 4u) {
        // Jungle: dark humus, a dense two-scale mottle of leaf litter.
        let m = 0.6 * vnoise(q1 * 0.9 + 1.0) + 0.4 * vnoise(q2 * 2.6 + 6.0);
        let fleck = smoothstep(0.8, 0.92, vnoise(q2 * 1.7 + 30.0));
        return 0.26 * (m - 0.5) - 0.12 * fleck;
    }
    if (fl == 5u) {
        // Highlands: gravel, stones about two thirds of a cell, each its
        // own shade, dark gaps between.
        let g = q1 / 0.7;
        let gi = floor(g);
        var d1 = 9.0; var d2 = 9.0; var c1 = gi;
        for (var j = -1; j <= 1; j++) {
            for (var i = -1; i <= 1; i++) {
                let c = gi + vec2<f32>(f32(i), f32(j));
                let o = c + vec2<f32>(hash2(c * 1.3 + 5.1), hash2(c * 2.9 + 1.7)) * 0.8 + 0.1;
                let d = length(g - o);
                if (d < d1) { d2 = d1; d1 = d; c1 = c; } else if (d < d2) { d2 = d; }
            }
        }
        let gap = (d2 - d1) * 0.7 * P.zoom;
        return 0.16 * (hash2(c1 * 4.1 + 2.3) - 0.5) - 0.22 * (1.0 - smoothstep(0.6, 2.0, gap));
    }
    if (fl == 6u) {
        // Caves: stone dust, a faint fine grain over broad pale and dark
        // drifts.
        // Smooth, with sparse pale grit and dark damp patches (art lead
        // 11-15: the first cut's even grain read as the barrens').
        let damp = smoothstep(0.55, 0.8, vnoise(q2 / 4.5 + 8.0));
        let grit = smoothstep(0.9, 0.97, vnoise(q1 * 3.3 + 13.0));
        return 0.04 * (vnoise(q1 / 2.0 + 2.0) - 0.5) - 0.12 * damp + 0.14 * grit * (1.0 - damp);
    }
    // Meadow and the core world: loam, soft crumbs at two scales.
    return 0.13 * (vnoise(q1 * 1.6 + 2.0) - 0.5) + 0.09 * (vnoise(q2 * 3.1 + 19.0) - 0.5);
}

fn soil_detail(x: f32, y: f32, bare: f32) -> vec4<f32> {
    // Under slime thick enough that the texture has faded out (most of a
    // big body), nothing to do (art lead 11-14: early-outs; cost both ways
    // in Reports/river-look-2026-11-14).
    if (bare <= 0.03) { return vec4<f32>(1.0, 0.0, 0.0, 0.0); }
    // Where the 4 x 4 cells the spline reads are one ground (kind and
    // soil), which is most pixels, the spline is skipped: its shares are
    // then that ground's alone, exactly. (Art lead, 11-14 22:15Z: the
    // ground had grown by about half on llvmpipe over four looks.) The
    // first skip (#305) tested only the 2 x 2 cells round the pixel, but
    // the spline reaches a cell further, and the bank, the shallow margin,
    // the bridge's shadow and the depth read its shares as a distance: at
    // every cell middle half a cell to a cell and a half off a shore or a
    // bridge they jumped (a hard step of 56 to 89 in brightness at 24 px a
    // cell beside the River's bridge, bug check 11-15).
    var so: Soil;
    let ux = i32(floor(x - 0.5)); let uy = i32(floor(y - 0.5));
    let g0 = ground[cidx(clamp(ux, 0, i32(P.ww) - 1), clamp(uy, 0, i32(P.wh) - 1))] & 127u;
    let k0 = g0 & 15u; let sk0 = g0 >> 4u;
    var one = !(k0 != WET && sk0 == SOIL_BRIDGE);
    for (var j = -1; j < 3 && one; j++) {
        for (var i = -1; i < 3; i++) {
            let c = cidx(clamp(ux + i, 0, i32(P.ww) - 1), clamp(uy + j, 0, i32(P.wh) - 1));
            if ((ground[c] & 127u) != g0) { one = false; break; }
        }
    }
    if (one) {
        so.w = vec4<f32>(0.0); so.wg = vec2<f32>(0.0); so.bridge = 0.0; so.bg = vec2<f32>(0.0);
        if (k0 == WET) { so.w.w = 1.0; } else if (sk0 == 1u) { so.w.x = 1.0; } else if (sk0 == 2u) { so.w.y = 1.0; } else if (sk0 == 3u) { so.w.z = 1.0; }
        so.deep = select(0.0, 1.0, k0 == WET && sk0 == SOIL_DEEP);
    } else {
        so = soil_share(x, y);
    }
    let w = so.w;
    if (dot(w, vec4<f32>(1.0)) + so.bridge <= 0.001) {
        // Plain soil alone (most of most maps): its biome's look only.
        let wp = bare * smoothstep(3.0, 8.0, P.zoom);
        if (wp > 0.001) { return vec4<f32>(1.0 + wp * PLAIN_AMP * plain_look(P.flora, vec2<f32>(x, y)), 0.0, 0.0, 0.0); }
        return vec4<f32>(1.0, 0.0, 0.0, 0.0);
    }
    var mul = 1.0;
    var add = vec3<f32>(0.0);
    // The water's crisp cover (its edge where the smooth wet share crosses
    // one half, antialiased over a pixel) and the signed distance to it.
    var cw = 0.0;
    var wd = -9.0;
    if (w.w >= 0.999) { cw = 1.0; wd = 9.0; }
    else if (w.w > 0.001) {
        wd = (w.w - 0.5) / max(length(so.wg), 1e-3);
        cw = clamp(0.5 + wd * P.zoom, 0.0, 1.0);
    }
    // The land's soils, crisp: each kind's smooth share (the cubic spline
    // over the cells) sharpened at the pixel into one kind or another, the
    // boundary where two shares are equal, antialiased over about a pixel
    // at any zoom (a softmax whose sharpness grows with the zoom). The
    // owner (11-14 20:21Z): "when I zoom in on the world, the edges between
    // different grounds is pixalated"; the material is blended between
    // cells, so at 24 px a cell every edge showed the cells' steps, and
    // the textures faded across two cells with the shares.
    let land_w = max(1.0 - w.w, 1e-3);
    let a = vec4<f32>(w.x, w.y, w.z, max(1.0 - w.x - w.y - w.z - w.w, 0.0)) / land_w;
    let sharp = clamp(3.4 * P.zoom, 2.0, 80.0);
    let am = max(max(a.x, a.y), max(a.z, a.w));
    let ea = exp((a - vec4<f32>(am)) * sharp);
    let ls = ea / dot(ea, vec4<f32>(1.0));
    // Each soil's texture by its crisp share on the land. (The plants'
    // crowns are drawn over it by their own cover, `plant_layer`.)
    let ws = ls.xyz * (1.0 - cw);
    // Sand: wind ripples about 1.6 cells apart, the flank toward the
    // light lighter. Low-frequency noise in their phase bends them and
    // stretches or squeezes their spacing (art lead, 11-14: straight even
    // diagonals read as a hatch at zoom 6 and a moire at zoom 1), and they
    // fade to flat sand below about zoom 3, where a ripple is under five
    // pixels and would alias.
    if (ws.x > 0.001) {
        let p = vec2<f32>(x, y);
        // Long dune swells about 14 cells apart, bent at their own scale,
        // which a whole-map view still shows,
        let big = vnoise(p / 24.0);
        var k = 0.12 * sin(6.2831853 * ((x * 0.55 + y * 0.3) / 14.0 + 1.6 * big));
        // and the ripples on them, bent by the same noise and one finer
        // octave (two vnoise a pixel; sums of sines cost the same on
        // llvmpipe and looked like wood grain).
        let fade = smoothstep(2.5, 4.0, P.zoom);
        if (fade > 0.0) {
            let bend = 4.0 * big + 1.2 * vnoise(p / 4.0 + 17.0);
            k += 0.14 * fade * sin(6.2831853 * ((x * 0.82 + y * 0.42) / 1.6 + bend));
        }
        mul *= 1.0 + k * ws.x * bare;
    }
    // Clay: dried into plates about 3 cells across, dark cracks between,
    // a pixel or so wide (the second-nearest minus nearest jittered point).
    // (Its cracks and plates are flat below zoom 1: skipped there.)
    if (ws.y > 0.001 && P.zoom > 1.0) {
        let p = vec2<f32>(x, y) / 3.0;
        let pi = floor(p);
        var d1 = 9.0; var d2 = 9.0;
        var c1 = pi;
        for (var j = -1; j <= 1; j++) {
            for (var i = -1; i <= 1; i++) {
                let c = pi + vec2<f32>(f32(i), f32(j));
                let o = c + vec2<f32>(hash2(c * 1.7 + 3.1), hash2(c * 2.3 + 7.9)) * 0.8 + 0.1;
                let d = length(p - o);
                if (d < d1) { d2 = d1; d1 = d; c1 = c; } else if (d < d2) { d2 = d; }
            }
        }
        let gap = (d2 - d1) * 3.0 * P.zoom;
        let crack = (1.0 - smoothstep(0.6, 1.6, gap)) * smoothstep(3.0, 8.0, 3.0 * P.zoom);
        // Each plate its own shade. (It was hashed on squares a seventh
        // of a plate across, a mottle of squares that read as pixels at
        // zoom 24; owner, 11-14 20:21Z.)
        let plate = 0.92 + 0.16 * hash2(c1 * 3.7 + 1.3);
        mul *= mix(1.0, (1.0 - 0.6 * crack) * mix(1.0, plate, smoothstep(3.0, 8.0, P.zoom)), ws.y * bare);
    }
    // Barren: dry ash, its grains soft and about a cell across, in clumps
    // a few cells wide with bare ash between (art lead, 11-14: a hashed
    // speckle half a cell across read as TV static at zoom 6).
    // (Its grains are flat below zoom 2: skipped there.)
    if (ws.z > 0.001 && P.zoom > 2.0) {
        let p = vec2<f32>(x, y);
        // Two octaves on turned grids, so a grain is round: one value
        // noise thresholded made squares on the cells' axes, which read as
        // pixels at zoom 12-24 (owner, 11-14 20:21Z).
        let q1 = vec2<f32>(0.8 * p.x - 0.6 * p.y, 0.6 * p.x + 0.8 * p.y);
        let q2 = vec2<f32>(0.28 * p.x + 0.96 * p.y, -0.96 * p.x + 0.28 * p.y);
        let n = 0.5 + 1.4 * (0.5 * (vnoise(q1 * 1.3 + 5.0) + vnoise(q2 * 1.5 + 21.0)) - 0.5);
        let clump = smoothstep(0.4, 0.7, vnoise(p / 3.5 + 11.0));
        let sp = (0.2 * smoothstep(0.6, 0.85, n) - 0.15 * smoothstep(0.6, 0.85, 1.0 - n)) * clump;
        let fade = smoothstep(2.0, 4.0, P.zoom);
        mul *= 1.0 + sp * ws.z * fade * bare;
    }
    // Plain soil: each planet's own ground up close (art lead's planet
    // review 11-15, item 3: the planets' plain ground was one flat colour
    // at zoom 12). The map's biome is `P.flora` (the plants thread's
    // per-map value, `levelgen::Biome as u8`; one uniform for both). A
    // signed change of at most about an eighth, so the ground keeps its
    // value band (20-45 in grey); it comes in from zoom 3 to 8, below
    // which it would alias, and lies under the slime by `bare`.
    let wl = ls.w * (1.0 - cw) * bare * smoothstep(3.0, 8.0, P.zoom);
    if (wl > 0.001) {
        mul *= 1.0 + wl * PLAIN_AMP * plain_look(P.flora, vec2<f32>(x, y));
    }
    // What the cells' material already gives here (`cell_mat_c`'s bare
    // ground, blended between the four cells round: `est`), and what this
    // pixel should be (`want`); the difference is added, so the soils' and
    // water's edges are redrawn at the pixel. (Plants and char are drawn
    // over all this in `shaded`.)
    let est = ground_est(x, y);
    let land = SAND_RGB * ls.x + CLAY_RGB * ls.y + BARREN_RGB * ls.z + plain_soil(ground_elev(x, y)) * ls.w;
    var want = land;
    // Wet: the water's edge is where the smooth wet share crosses one
    // half, worked out at the pixel (one pixel antialiased, as rock's), so
    // a pond is a curve, not the cells' stair the material's blend drew.
    // Out from it: a thin light waterline, then damp dark ground about a
    // cell wide. In from it: a shallow margin where the bed shows through,
    // then (deep water) darkening toward the middle, and a glitter of the
    // key light in streaks across its direction.
    if (w.w > 0.001) {
        let d = wd;
        var water = WET_RGB;
        var depth = 0.0;
        if (so.deep > 0.001) {
            depth = clamp(so.deep, 0.0, 1.0) * (0.35 + 0.65 * deep_depth(x, y));
            water = mix(WET_RGB, DEEP_RGB, smoothstep(0.0, 0.8, depth));
        }
        // The surface: a slow swell in the darkness, so open water isn't
        // flat. (Tried and dropped: streaks of glitter across the key light
        // read as rain at zoom 6 and 12; thresholded noise as angular chips
        // and square blobs.)
        let p = vec2<f32>(x, y);
        water *= 1.0 + 0.3 * (vnoise(p / 8.0 + 9.0) - 0.5) * (0.3 + 0.7 * depth);
        if (w.w >= 0.999) {
            want = water;
        } else {
            // The shallow margin: the bed (the land's colour, wet-dark)
            // through a little water, about a cell in.
            let shallow = mix(land * 0.7, vec3<f32>(30.0, 52.0, 56.0), 0.55);
            water = mix(water, shallow, exp(-max(d, 0.0) / 0.9) * 0.8);
            // The bank: damp darker ground out to about a cell, and a light
            // waterline right at the edge.
            let out_d = max(-d, 0.0);
            let bank = land * (1.0 - 0.4 * exp(-out_d / 0.9)) + vec3<f32>(30.0, 40.0, 42.0) * exp(-abs(d) / 0.12);
            want = mix(bank, water, cw);
        }
    }
    // A bridge: a plank deck across the water, planks laid across the way
    // over (the axis along which the deck meets no deep water), a seam
    // between planks, each plank its own shade, and a darker rail along
    // both edges with posts. Its edge is crisp like the water's.
    if (so.bridge > 0.001) {
        let gl = max(length(so.bg), 1e-3);
        let d = select((so.bridge - 0.5) / gl, 9.0, so.bridge >= 0.999);
        let cb = clamp(0.5 + d * P.zoom, 0.0, 1.0);
        if (cb > 0.0) {
            var sx = 0.0; var sy = 0.0;
            for (var k = 1; k <= 3; k++) {
                let r = f32(k) * 5.0;
                sx += deep_at(i32(floor(x + r)), i32(floor(y))) + deep_at(i32(floor(x - r)), i32(floor(y)));
                sy += deep_at(i32(floor(x)), i32(floor(y + r))) + deep_at(i32(floor(x)), i32(floor(y - r)));
            }
            let along = select(vec2<f32>(1.0, 0.0), vec2<f32>(0.0, 1.0), sx >= sy);
            let u = dot(vec2<f32>(x, y), along);
            let v = dot(vec2<f32>(x, y), vec2<f32>(along.y, along.x));
            let pw = 0.8;
            let f = fract(u / pw);
            let px = 1.0 / P.zoom;
            let seam = 1.0 - smoothstep(0.5 * px, 1.5 * px, min(f, 1.0 - f) * pw);
            let plank = floor(u / pw);
            let tint = 0.82 + 0.3 * hash2(vec2<f32>(plank, 3.0));
            let grain = 1.0 + 0.07 * sin(v * 5.0 + 9.0 * hash2(vec2<f32>(plank, 7.0)));
            var wood = WOOD_RGB * tint * grain * (1.0 - 0.6 * seam * smoothstep(3.0, 6.0, P.zoom));
            // The rails: a beam about 0.5 cell wide along each edge, a post
            // every 3 cells standing lighter.
            let rail = 1.0 - smoothstep(0.45, 0.45 + px, d);
            let post = rail * (1.0 - smoothstep(0.35, 0.35 + px, abs(fract(u / 3.0) - 0.5) * 3.0 - 1.05));
            // The deck's edge as a bevel: the rail facing the key light
            // lit, the far one shaded. (A deck height in `cell_height` was
            // tried: the shading reads height as mould, so the soil
            // texture faded off the deck.)
            let outn = -so.bg / gl;
            let facing = dot(outn, normalize(-LIGHT.xy));
            wood = mix(wood, vec3<f32>(58.0, 53.0, 47.0), rail);
            wood = mix(wood, vec3<f32>(118.0, 110.0, 99.0), post);
            wood *= 1.0 + 0.25 * facing * rail;
            want = mix(want, wood, cb);
        }
        // Its shadow on the water, on the side away from the key light,
        // about half a cell wide.
        let outn = -so.bg / gl;
        let away = max(dot(outn, normalize(LIGHT.xy)), 0.0);
        want *= 1.0 - 0.55 * away * exp(min(d, 0.0) / 0.5) * (1.0 - cb);
    }
    add += (want - est) * bare;
    return vec4<f32>(mul, add);
}

// The Lure's steering field as the engine holds it (2026-11-18; the lab's
// prototype of 11-17 made a rule): a word a 2 x 2 block of cells, the
// walking distance to the nearest lure in cells + 1, and 0 where the field
// does not reach (past its reach, behind rock, or no lure standing).
// Calm walkers lean down it; the engine rewrites it once a game second.
fn steer_w(bx: i32, by: i32) -> u32 {
    let bw = (P.ww + 1u) / 2u; let bh = (P.wh + 1u) / 2u;
    let x = u32(clamp(bx, 0, i32(bw) - 1)); let y = u32(clamp(by, 0, i32(bh) - 1));
    return news[P.steer_at + y * bw + x];
}
// (distance, cover) at a point: bilinear over the blocks the field
// reaches, so a line of equal distance is smooth at any zoom and stops
// where the field does.
fn steer_px(x: f32, y: f32) -> vec2<f32> {
    let fx = x / 2.0 - 0.5; let fy = y / 2.0 - 0.5;
    let ix = i32(floor(fx)); let iy = i32(floor(fy));
    let tx = fx - floor(fx); let ty = fy - floor(fy);
    var sum = 0.0; var ws = 0.0;
    for (var j = 0; j < 2; j++) {
        for (var i = 0; i < 2; i++) {
            let v = steer_w(ix + i, iy + j);
            if (v == 0u) { continue; }
            let w = select(1.0 - tx, tx, i == 1) * select(1.0 - ty, ty, j == 1);
            sum += w * f32(v - 1u); ws += w;
        }
    }
    if (ws <= 0.0) { return vec2<f32>(0.0, 0.0); }
    return vec2<f32>(sum / ws, ws);
}
// The rings' colour is your team's cream (art direction, rule 3: the Lure
// is your tool). Not the lab's pink (255, 110, 215): that hue is the core
// planet's flora, the only purple on the map. Not the Lure's old scent
// green either: on the meadow's green it was lost, and a green over the
// fight is what the flinch's wash was (`flush_at`). Tried on the lab's
// scene at zoom 6, planet 1, 2026-11-18.
const STEER_RGB: vec3<f32> = vec3<f32>(250.0, 242.0, 220.0);
// A ring every 20 cells of walking, and how bright a ring is at the lure.
const STEER_RING: f32 = 20.0;
const STEER_LINE: f32 = 0.42;

fn shaded(x: f32, y: f32, e: f32) -> vec3<f32> {
    var m = mat_at(x, y);
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
    // The soil's own material where the ground is bare (the mould's film,
    // thickness by the inverse of the height curve, covers it).
    let th_here = film_th(x, y);
    // Under any slime the ground's own detail (clay cracks, ripples, the
    // plants' crowns, char) goes fully under, gone by the thin film's
    // middle (th 0.05), so only the slime's texture reads where it lies
    // (art lead 11-15, planet review #8: cracks and crowns showed through
    // the film). Before, `exp(-30 th)` left 40% of it at th 0.03, and the
    // plants took the per-fine-cell `vis` alone, 1 under any thin film.
    let under = 1.0 - smoothstep(0.004, 0.05, th_here);
    let sd = soil_detail(x, y, exp(-30.0 * th_here) * under * m.vis);
    m.albedo = (m.albedo + sd.yzw) * sd.x;
    // Burnt ground, then the plants, at this pixel and under the slime
    // (`vis`): one B-spline of the plant fields (`plant_px`), which the
    // fire below shares.
    let pp = plant_px(x, y);
    var pl: PlantLayer;
    pl.fl = vec3<f32>(0.0); pl.fc = 0.0; pl.fw = 0.0;
    if (P.plant_at != 0u) {
        // Char: near-black grey with a crisp edge (a narrow rise of the
        // per-pixel char, a few pixels at zoom 24), a little ash grain.
        // Over water the plants thin out so the water shows between
        // them, and none lie over deep water (whole-game review, 11-15:
        // the rivers vanished under reeds within two minutes, the reeds
        // spreading over the shallows and the plants' spline bleeding
        // over the narrow deep channel). Only the look: the reeds still
        // grow there, and burn.
        // Zoomed out, where the crowns fade to their flat mean cover, that
        // cover is veiled over water too (the shallows read green at zoom
        // 4 with the thinning alone).
        var dp = pp.d;
        var ws = vec2<f32>(0.0);
        if (dot(dp, vec4<f32>(1.0)) > 0.003) {
            ws = water_share(x, y);
            dp *= (1.0 - WATER_THIN * ws.x) * (1.0 - smoothstep(0.25, 0.6, ws.y));
        }
        pl = plant_layer(x, y, dp, pp.heat > 0.002);
        if (ws.x > 0.0) {
            let far = 1.0 - clamp(pl.fw / max(pl.cover, 1e-3), 0.0, 1.0);
            let veil = 1.0 - WATER_VEIL * ws.x * far;
            pl.rgb *= veil; pl.cover *= veil;
        }
        let gv = m.vis * under;
        let pv = pl.cover * gv;
        m.albedo = m.albedo * (1.0 - pv) + pl.rgb * gv;
        m.gloss *= 1.0 - pv;
        // Over the plants too: what is left of a burnt stand is stubble.
        let ch = smoothstep(0.06, 0.3, pp.char) * (0.85 + 0.15 * smoothstep(0.3, 0.8, pp.char));
        let ash = CHAR_RGB * (0.9 + 0.2 * vnoise(vec2<f32>(x, y) * 2.3));
        m.albedo = mix(m.albedo, ash * (0.8 + 0.4 * (pl.rgb / max(pl.cover, 0.01)).g / 60.0 * pv), ch * gv);
    }
    // The crust over the bare ground and its plants, under the slime.
    m.albedo = crust_over(m.albedo, x, y, m.vis * under);
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
    // The highlight takes its normal from the height sampled wider than
    // the shading's (bilinear, four cheap samples) and a broad lobe (art director,
    // 11-14 08:15Z: at the fine slope and exponent 16-80 the dense body
    // glittered like crumpled gold foil): wet sheens along the veins, not
    // one glint a walker clump.
    // The spacing is some 9 screen pixels (1.5 cells at zoom 6, 0.75 at
    // 12): a fixed 1.5 cells made zoom 12 read soft again.
    let so = clamp(9.0 / P.zoom, 0.6, 1.5);
    // From the mould's own height (`hm`): the sheen is the wet film's and
    // the water's, which is flat. Taken from the whole height, a reed bed's
    // slope down into the water caught the highlight on the water's gloss,
    // a pale blurred band a cell wide along every bank (11-14).
    let sx = film_h(x + so, y) - film_h(x - so, y);
    let sy = film_h(x, y + so) - film_h(x, y - so);
    let ns = normalize(vec3<f32>(-sx / (2.0 * so), -sy / (2.0 * so), 1.0));
    let spec = pow(max(dot(ns, half_v), 0.0), 14.0 + 30.0 * m.gloss) * m.gloss * (0.5 + 0.7 * m.gloss);
    // The thin film's shaded flank keeps most of its light (rule 2: the
    // film from 80; the planet review 11-15 measured its 10th percentile
    // at 65-80 on slime-coloured pixels, the far flank of the frontier's
    // veins, which fell to 0.4 of the light). Thick veins keep their full
    // relief.
    let thin_floor = 0.15 + 0.7 * smoothstep(0.005, 0.04, th_here) * (1.0 - smoothstep(0.25, 0.6, th_here));
    var c = m.albedo * max(shade, thin_floor) + vec3<f32>(255.0, 250.0, 236.0) * spec + m.glow;
    // The fire's body, at this pixel (`flame_body` of the B-spline heat):
    // worked out in `prep` per fine cell it stair-stepped at the owner's
    // zoom 24 (11-14 20:21Z). The heat comes with the plants' own reads.
    if (P.plant_at != 0u) {
        // Emission over the darkened ground (graphics lead 21:01Z).
        // Zoomed in, the flames are the burning crowns' own (`plant_layer`)
        // and, where the plants are gone, ember points: the heat still in
        // the char as scattered glowing grains whose size follows the heat,
        // so they shrink and go as it fades (graphics lead 21:26Z: the
        // smooth heat alone read as a soft orange blob). Zoomed out (a crown
        // or grain under about 3 pixels) the smooth heat's flame stands in.
        let hf = pp.heat;
        let ez = smoothstep(2.5, 5.0, 0.6 * P.zoom);
        var fl = pl.fl;
        var fc = pl.fc;
        let rest = 1.0 - pl.fw;
        if (hf > 0.002 && rest > 0.0) {
            let sc = flame_cover(hf) * rest * (1.0 - ez);
            fl += mix(flame_body(hf), FLAME_CORE, 0.8 * clamp(pp.fresh, 0.0, 1.0)) * sc * (1.0 - fc);
            fc += sc * (1.0 - fc);
            if (ez > 0.0) {
                var ef: Form;
                ef.s = 0.6; ef.r0 = 0.35; ef.stretch = 1.0; ef.lobe = 0.0; ef.relief = 0.5; ef.shape = 0u; ef.clump = 0.0; ef.col = vec3<f32>(0.0); ef.wet = 0.0; ef.tip = vec3<f32>(0.0);
                let em = crowns(x, y, ef, hf, 913.0);
                let ea = em.a * ez * rest * smoothstep(0.02, 0.1, hf);
                fl += flame_body(min(hf * (0.9 + 0.4 * em.dome) + 0.3 * pp.fresh, 1.0)) * ea * (1.0 - fc);
                fc += ea * (1.0 - fc);
            }
        }
        c = c * (1.0 - 0.6 * fc) + fl + flame_spill(hf) * (1.0 - fc);
    }
    // Each point light on this surface: diffuse on its albedo, a highlight
    // by its gloss, falling to nothing at the light's reach. The surface's
    // own height is in `to`, so a tube's flank toward a bullet lights and
    // the far flank stays dark.
    for (var j = 0u; j < P.n_lights; j++) {
        let a = lights[2u * j];
        let lc = lights[2u * j + 1u];
        let to = vec3<f32>(a.x - x, a.y - y, abs(a.w) - h);
        let d2 = dot(to.xy, to.xy);
        let rr = a.z * a.z;
        if (d2 >= rr) { continue; }
        let fall = 1.0 - d2 / rr;
        var att = fall * fall * lc.w;
        // A scent light (lift below 0, a burning lure's, 11-14): lit only
        // as far as the engine's scent there (the look's byte 3), so what
        // glows is the scent the lure really lays, spreading while it
        // burns and fading as the field does once it stops.
        // The scent glows a little of itself too, as the gas cloud does,
        // so it shows on the dark bare ground the light alone barely lifts.
        if (a.w < 0.0) {
            // While the steering field's rings are drawn they are the
            // Lure's sign and the scent light is not lit: its green pool
            // turned the slime at the lure lime, and at cream it read as
            // the pale haze the alarm lost (11-18, the lab's scene).
            if (P.steer_at != 0u) { continue; }
            att *= look_scent(x, y);
            c += lc.xyz * att * 0.22;
        }
        let ld = normalize(to);
        let diff = max(dot(n, ld), 0.0);
        let hv = normalize(ld + vec3<f32>(0.0, 0.0, 1.0));
        let sp = pow(max(dot(n, hv), 0.0), 24.0) * (0.15 + m.gloss);
        c += (m.albedo * (lc.xyz / 255.0) * diff * 1.6 + lc.xyz * sp * 0.9) * att;
    }
    // (The alarm field's pale haze was laid here, to 2026-11-17: `flush_at`.)
    // The Lure's steering field, drawn as it is: hairlines of equal walking
    // distance to the lure every `STEER_RING` cells (they bend round rock
    // and walls as the field does), brightest at the lure and fading with
    // the distance, so what the rings cover is what the Lure reaches and
    // the slime leans across them toward the middle. Lines only: the lab's
    // prototype also filled the field with a haze and lit a core at the
    // lure, which was a wash over the kill zone the Lure is built to make;
    // the Lure building marks the middle. Under slime the lines drop to
    // 30%, so the slime stays the thing to read. They do not move: a
    // drifting ring would be a drawn pulse, not the field.
    if (P.steer_at != 0u) {
        let sp = steer_px(x, y);
        if (sp.y > 0.0) {
            let d = sp.x;
            let ring = STEER_RING * max(round(d / STEER_RING), 1.0);
            let dr = abs(d - ring);
            // No ring within half a ring of the field's reach. There the
            // field stops on its blocks' stepped edge, so a ring drew as a
            // dotted line round the pull's rim; the game's reach (60 dish
            // cells, 120 cells) is a multiple of the spacing, so its last
            // ring fell on that edge (seen on planets 2 and 3, 11-18). The
            // pull's rim is where the rings stop.
            let kept = select(1.0, 0.0, P.steer_reach > 0.0 && ring > P.steer_reach - 0.5 * STEER_RING);
            let line = (1.0 - smoothstep(0.6 / P.zoom, 1.6 / P.zoom, dr)) * kept;
            c += STEER_RGB * STEER_LINE * line * (0.25 + 0.75 * exp(-d / 110.0)) * sp.y * (0.3 + 0.7 * under);
        }
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
            // Spent, it is worn stone (art director, 11-14: a dry crystal
            // "reads like a UI block"): darker than the rock round it, matte,
            // and stepped in three hexagonal terraces from its depth field,
            // each step's riser (two pixels) tilted out so the light catches
            // one side and shades the other, as a worn step pyramid. The
            // steps come in as the charge goes, so a live crystal is clean
            // glass. Before: a flat (84,88,92) grey, steps tried as dark
            // seams alone read as a target.
            let spent = 1.0 - v;
            let tf = fract(depth * 3.0 + 0.5);
            let tw = 2.0 * 3.0 / max(a * P.zoom, 1.0);
            let riser = (1.0 - smoothstep(0.0, tw, min(tf, 1.0 - tf) * 2.0)) * spent;
            // The pyramid's face: height (0.6 + 9 depth) tall(v), so its
            // slope is 9 tall(v) / a along the distance's gradient; a riser
            // adds a steep outward tilt.
            let s = 9.0 * gem_tall(v) / a + 2.2 * riser;
            let gn = normalize(vec3<f32>(-s * gi.y, -s * gi.z, 1.0));
            var gs = 1.0 + 1.25 * (dot(gn, l) - l.z);
            // Charged, it is glassy cyan lit from inside; spent, matte stone
            // (the only cyan in the world is charge).
            let gspec = pow(max(dot(gn, half_v), 0.0), 28.0) * (0.08 + 0.92 * v);
            let worn = 0.6 + 0.3 * depth;
            // Charged faces from about 110 in grey (rule 2: above rock, soft
            // rock and the thin film; the review 11-15 measured 93-126).
            var alb = mix(vec3<f32>(66.0, 66.0, 64.0) * worn, vec3<f32>(38.0, 142.0, 165.0), v);
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
            let th = film_th(x, y);
            let film = GEM_FILM * (1.0 - exp(-40.0 * th));
            // The faces' light compressed toward the middle while charged
            // (a shaded face of a live crystal stays above 110 in grey;
            // the spent stone keeps the full relief of its steps).
            var rock = alb * max(mix(gs, 1.0 + 0.6 * (gs - 1.0), v), 0.15) + vec3<f32>(255.0, 248.0, 230.0) * gspec * 0.7;
            // And a floor: a face holding charge never reads under 112 in
            // grey with its glow (art lead 11-15: crystals are the
            // objective; the compression alone left meadow 2 at 101 and
            // caves at 105-109, shaded faces and part-charged cells). The
            // floor fades in with the charge, so spent stone stays dark.
            let rl = dot(rock + glow, vec3<f32>(0.299, 0.587, 0.114));
            let want = 112.0 * smoothstep(0.15, 0.45, v);
            rock *= max(1.0, (want - (rl - dot(rock, vec3<f32>(0.299, 0.587, 0.114)))) / max(dot(rock, vec3<f32>(0.299, 0.587, 0.114)), 1.0));
            // The film is the slime as shaded off the crystal (its own
            // bumps, gloss and glint), tilted by the face it lies on.
            let lum = dot(c, vec3<f32>(0.299, 0.587, 0.114));
            let fed = mix(c, vec3<f32>(0.45, 1.05, 1.2) * lum, clamp(0.2 * hot, 0.0, 0.2));
            let on = fed * clamp(0.5 + 0.5 * gs, 0.4, 1.4);
            // The silhouette stays crisp under the film: a dark line a pixel
            // and a half wide inside the hexagon's edge, where the faces
            // meet the ground (a hard shape's edge, art direction rule 4).
            let rim = 0.72 + 0.28 * smoothstep(0.0, 1.5, gi.x * P.zoom);
            // The charge's light comes up through the film, brightest deep in a
            // full crystal, so a covered crystal still glows cyan.
            let through = glow * (1.0 - 0.8 * film) + vec3<f32>(8.0, 40.0, 50.0) * v * depth * film;
            let gc = mix(rock * rim, on, film) + through;
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
    else if (kind == DOOR) { h = 0.4 + 1.4 * v; }
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
    // The value order (rule 2; the planet review 11-15, its luma table):
    // ground 20-45, rock 50-60, soft rock 60-72 at full health, then the
    // slime's thin film from 80, crystals from 110, thick slime from 140.
    // Rock was 98 and soft rock 107-123 in grey, above the thin film on
    // all 14 planet-seeds and above the crystals on 12.
    if (kind == ROCK) { return vec4<f32>(mix(vec3<f32>(50.0, 52.0, 56.0), vec3<f32>(66.0, 65.0, 62.0), v), h); }
    // Health is the wall's brightness as well as its height: a warm grey
    // just above rock at full health, darkening as it is worn. (Before:
    // up to (128, 118, 104), and its pale beige 168 before 11-14 18:56Z.)
    if (kind == WALL) { return vec4<f32>(mix(vec3<f32>(34.0, 30.0, 26.0), vec3<f32>(74.0, 67.0, 58.0), v), h); }
    if (kind == DOOR) { return vec4<f32>(mix(vec3<f32>(36.0, 26.0, 16.0), vec3<f32>(88.0, 62.0, 36.0), v), h); }
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

struct Ter { s: f32, grad: vec2<f32>, h: f32, albedo: vec3<f32>, wall: f32, wal: vec3<f32> };

// The terrain field at world point (x, y): occupancy, its gradient (per
// cell), and the solid cells' mean height and colour.
fn terrain(x: f32, y: f32) -> Ter {
    var t: Ter;
    t.s = 0.0; t.grad = vec2<f32>(0.0); t.h = 0.0; t.albedo = vec3<f32>(0.0); t.wall = 0.0; t.wal = vec3<f32>(0.0);
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
                if (c.w > 0.0) { t.h += w * c.w; t.albedo += w * c.xyz; hw += w; if (((p >> 16u) & 255u) == WALL) { t.wall += w; t.wal += w * c.xyz; } }
            }
        }
    }
    if (hw > 0.0) {
        // Where soft rock meets rock, each keeps its own colour up to a
        // crisp joint (the wall share's half, over a few pixels): the
        // weighted mean blurred the seam over 25 px (the review 11-15).
        let rk = t.albedo - t.wal;
        let wl = t.wall;
        t.h /= hw; t.wall = wl / hw;
        if (wl > 0.0 && hw - wl > 0.0) {
            let js = smoothstep(0.5 - 1.5 / P.zoom, 0.5 + 1.5 / P.zoom, t.wall);
            t.albedo = mix(rk / (hw - wl), t.wal / wl, js);
            t.wall = js;
        } else { t.albedo /= hw; }
    } else { t.h = 0.0; }
    return t;
}

// The hard terrain at world point (x, y) as `terrain_over` draws it: the
// field, the signed distance to its edge in cells (inside positive) and
// the pixel's coverage. `near` is false where no hard terrain is within
// reach of this cell (`look::FAR`), at the cost of one read.
struct TerAt { t: Ter, d: f32, cov: f32, near: bool, p: vec2<f32> };
fn terrain_at(x: f32, y: f32) -> TerAt {
    var o: TerAt;
    o.d = 0.0; o.cov = 0.0; o.p = vec2<f32>(x, y);
    o.near = (ground[cidx(i32(floor(x)), i32(floor(y)))] & 128u) == 0u;
    if (!o.near) { return o; }
    o.t = terrain(x, y);
    let gl = max(length(o.t.grad), 1e-4);
    if (o.t.s > 0.0) {
        o.d = (o.t.s - TER_ISO) / gl;
        // Soft rock (the hedges) is grown, not cut: its edge is pushed in
        // and out by a lumpy noise about a cell across, so it reads as a
        // broken, bushy line, not a tube (the planet review 11-15, item 10).
        if (o.t.wall > 0.0) {
            let q = vec2<f32>(0.8 * x - 0.6 * y, 0.6 * x + 0.8 * y);
            o.d += o.t.wall * 0.45 * (vnoise(q * 1.1 + 31.0) + 0.5 * vnoise(vec2<f32>(x, y) * 2.7 + 7.0) - 0.75);
        }
        o.cov = clamp(0.5 + o.d * P.zoom, 0.0, 1.0);
    }
    return o;
}

// The terrain's top at a covered point: flat, beveled down to the edge.
fn terrain_top(ta: TerAt) -> vec3<f32> {
    let t = ta.t;
    let d = ta.d;
    let l = normalize(LIGHT);
    let gl = max(length(t.grad), 1e-4);
    let inward = t.grad / gl;
    // At most 5 pixels wide, so at zoom 12-24 the shaded side is a crisp
    // band, not a cell of blur (the planet review 11-15).
    let k = 1.0 - smoothstep(0.0, min(TER_BEVEL, 5.0 / P.zoom), d);
    // Raised: the bevel tilts enough that the side toward the light is lit
    // and the far side shaded (11-14, art lead: rock read as a flat
    // cutout at barren's value; the tilt was 0.35 + 0.15 h), and a thin
    // rim along the lit edge catches the key light.
    // Soft rock takes no bevel (its tilt rounded it into a tube): its
    // relief is the grain below.
    let n = normalize(vec3<f32>(-inward * k * (0.8 + 0.2 * t.h) * (1.0 - 0.85 * t.wall), 1.0));
    let shade = 1.0 + 1.25 * (dot(n, l) - l.z);
    let half_v = normalize(l + vec3<f32>(0.0, 0.0, 1.0));
    let spec = pow(max(dot(n, half_v), 0.0), 20.0) * 0.08;
    let lit_edge = max(dot(-inward, normalize(l.xy)), 0.0) * (1.0 - smoothstep(0.0, 2.0, d * P.zoom));
    // The hedge's grain: clumps a cell and a half across, finer leaves
    // in them, darker where the cells are worn (their health is already
    // the albedo's value), and a dark fringe where the lumpy edge ends.
    var alb = t.albedo;
    if (t.wall > 0.0) {
        let p = ta.p;
        // (On turned grids: on the cells' axes value noise reads as
        // squares at zoom 24.)
        let q1 = vec2<f32>(0.8 * p.x - 0.6 * p.y, 0.6 * p.x + 0.8 * p.y);
        let q2 = vec2<f32>(0.28 * p.x + 0.96 * p.y, -0.96 * p.x + 0.28 * p.y);
        let gr = vnoise(q1 / 1.5 + 3.0) * 0.6 + vnoise(q2 * 2.3 + 11.0) * 0.4;
        alb *= mix(1.0, (0.72 + 0.56 * gr) * (0.8 + 0.2 * smoothstep(0.0, 0.6, d)), t.wall);
    }
    // A door's grate: bars on the cells' middles both ways, dark between,
    // a pixel of smoothing at any zoom. Read from the cell under the pixel
    // (one read, covered pixels only), not held in `Ter`, which costs
    // registers across `shaded` (view-cost-2026-11-15).
    let pc = ta.p;
    if ((ground[cidx(i32(floor(pc.x)), i32(floor(pc.y)))] & 15u) == DOOR) {
        let aa = 0.7 / P.zoom;
        let bx = 1.0 - smoothstep(0.13 - aa, 0.13 + aa, abs(fract(pc.x) - 0.5));
        let by = 1.0 - smoothstep(0.13 - aa, 0.13 + aa, abs(fract(pc.y) - 0.5));
        alb *= mix(0.55, 1.35, max(bx, by));
    }
    return alb * max(shade, 0.15) + vec3<f32>(255.0, 248.0, 230.0) * spec * (1.0 - t.wall) + vec3<f32>(60.0, 58.0, 52.0) * lit_edge * (1.0 - 0.7 * t.wall);
}

// The ground's colour `c` at world point (x, y) with the hard terrain over
// it (`terrain_at`): the shadow it casts, then its top, lit, with an
// antialiased edge. Where the terrain covers the pixel whole, `base` takes
// `terrain_top` alone and never shades the ground under it.
// It works the terrain out again rather than taking `base`'s: held across
// `shaded`, the terrain's fields cost the 1070 0.15-0.18 ms at zoom 6 and
// 24 on the fork (registers; 2011eb65, Reports/view-cost-2026-11-15).
fn terrain_over(c0: vec3<f32>, x: f32, y: f32) -> vec3<f32> {
    let ta = terrain_at(x, y);
    if (!ta.near) { return c0; }
    let l = normalize(LIGHT);
    let cov = ta.cov;
    var c = c0;
    if (cov < 1.0) {
        // The shadow on the ground: the terrain a short way toward the
        // light, as far as its height casts; soft over about a cell.
        let ld = normalize(l.xy);
        let q = terrain(x + ld.x * 1.2, y + ld.y * 1.2);
        if (q.s > 0.0) {
            let reach = clamp(q.h * TER_SHADOW, 0.0, 1.0);
            // Its edge is the offset terrain's own edge, antialiased over
            // two pixels (the planet review 11-15: the shaded side was a
            // soft band 20-25 px wide at zoom 24; it faded over the field's
            // 0.05-0.6, a cell or so).
            let qd = (q.s - TER_ISO) / max(length(q.grad), 1e-4);
            c *= 1.0 - 0.45 * reach * clamp(0.5 + 0.5 * qd * P.zoom, 0.0, 1.0);
        }
    }
    if (cov <= 0.0) { return c; }
    // The top: flat, beveled down to the edge within TER_BEVEL, the bevel
    // a little steeper the taller the terrain.
    return mix(c, terrain_top(ta), cov);
}

// An absorbed walker (the engine's `ABSORB_BIT`, 11-14; art director:
// the turn should be an implosion, the absorbed half streaming into their
// survivors, not vanishing). For `ABSORB_STEPS` (0.5 s) after it is
// absorbed its slot still holds where it died and the steps since; it is
// drawn as a short bright streak pulled from there toward the condensed
// walkers round it: the centre of the engine's per-cell condensed count
// (`dcount`, after the slots) in the 9 x 9 cells round, so it lands on its
// survivors and the patch draws in as it thickens. It speeds up as it
// goes (the square of its age) and dims. For its first 0.3 s it also
// counts a fresh absorption in its fine cell (the grid's second half,
// above bit 16) for `prep`'s warm-white flash. It adds nothing to the body.
fn draw_absorbed(k: u32, dw: u32) {
    let w = walkers[k];
    if (!(w.x >= 0.0 && w.y >= 0.0 && w.x < f32(P.ww) && w.y < f32(P.wh))) { return; }
    let age = f32(dw & 0xFFFFu);
    let u = clamp(age / ABSORB_STEPS, 0.0, 1.0);
    let cx = i32(floor(w.x)); let cy = i32(floor(w.y));
    var pull = vec2<f32>(0.0);
    var tot = 0.0;
    let cells = P.dense_at + P.n_walk;
    for (var dy = -4; dy <= 4; dy++) {
        for (var dx = -4; dx <= 4; dx++) {
            let x = cx + dx; let y = cy + dy;
            if (x < 0 || y < 0 || x >= i32(P.ww) || y >= i32(P.wh)) { continue; }
            let c = f32(news[cells + u32(y) * P.ww + u32(x)]);
            pull += c * (vec2<f32>(f32(x), f32(y)) + 0.5 - vec2<f32>(w.x, w.y));
            tot += c;
        }
    }
    // At least a cell and a half: inside an even body the centre is
    // close, and a pull that short drew a tick, not a stream.
    if (tot > 0.0) { pull /= tot; }
    let pl = length(pull);
    if (pl > 1e-3) { pull *= max(pl, 1.5) / pl; }
    let head = vec2<f32>(w.x, w.y) + pull * u * u;
    let tail = vec2<f32>(w.x, w.y) + pull * max(u - 0.3, 0.0) * max(u - 0.3, 0.0);
    if (age < 9.0) {
        let kf = f32(P.fine);
        let gx = i32(floor((w.x - f32(P.ox)) * kf));
        let gy = i32(floor((w.y - f32(P.oy)) * kf));
        if (gx >= 0 && gy >= 0 && gx < i32(P.rw) && gy < i32(P.rh)) { atomicAdd(&fgrid[P.fhalf + u32(gy) * P.rw + u32(gx)], 1u << FG_ABSORB); }
    }
    // The streak in screen pixels, tail to head, brightest at the head.
    let a = (tail - vec2<f32>(P.cx, P.cy)) * P.zoom;
    let b = (head - vec2<f32>(P.cx, P.cy)) * P.zoom;
    let len = max(abs(b.x - a.x), abs(b.y - a.y));
    let steps = clamp(i32(ceil(len)), 1, 48);
    // The streak buffer (the specks' second half, `base`): up to 3 a pixel
    // at the head, so a streak shows warm-white over the bright body, where
    // a calm speck's light is all but off.
    let bright = 3.0 * (1.0 - 0.6 * u);
    let th = clamp(i32(P.zoom / 6.0), 1, 3);
    for (var j = 0; j <= steps; j++) {
        let f = f32(j) / f32(steps);
        let q = mix(a, b, f);
        let v = u32(round(bright * (0.35 + 0.65 * f)));
        // A pixel wide per 6 of zoom (a hair at zoom 12 read as texture).
        for (var ty = 0; ty < th; ty++) {
            for (var tx = 0; tx < th; tx++) {
                let px = i32(floor(q.x)) + tx; let py = i32(floor(q.y)) + ty;
                if (px < 0 || py < 0 || px >= i32(P.sw) || py >= i32(P.sh)) { continue; }
                atomicAdd(&specks[P.shalf + u32(py) * P.stride + u32(px)], v);
            }
        }
    }
}

// Every walker a speck of light at its own position (one thread a walker),
// counted a screen pixel; `base` adds and clears them.
@compute @workgroup_size(256)
fn draw_specks(@builtin(global_invocation_id) gid: vec3<u32>) {
    let k = gid.x;
    if (k >= P.n_walk) { return; }
    var dw = 0u;
    if (P.dense_at > 0u) { dw = news[P.dense_at + k]; }
    if (P.lives != 0u && alive[k] < 0.0) {
        if ((dw & ABSORB_BIT) != 0u) { draw_absorbed(k, dw); }
        return;
    }
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
    // A mini-boss's member (2026-11-16): its lump + 1 in its word's bits
    // 25-27, when the game gives that lump a look. The engine spares members
    // withering and panic, so their news is not drawn (no ash, no amber or
    // red speck on a hunter crossing open ground); nor their condensing,
    // nor their ripeness (the engine keeps writing both): a member is the
    // lump's material and counts once, in `prep`'s body and its own count.
    let lump_k = (dw >> LUMP_SHIFT) & 7u;
    let member = P.n_lumps > 0u && lump_k != 0u && lump_k <= 4u && P.lumps[min(lump_k, 4u) - 1u].z >= 0.5;
    if (P.news_on == 1u && !member) {
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
    var add = select(1u, 0u, cut > 0.5) | (lean << 10u) | (bolt << 20u);
    // Condensing (`set_dense`, 11-14), all of it read off the walker's
    // word. A condensed walker is two walkers' worth: it counts twice in
    // the body and 15 in the fine grid's second half (`dcnt`), which
    // `prep` turns into the bronze's share. A ripening walker warms toward
    // it as its ripeness climbs (up to 4, about half the bronze at the
    // moment it condenses), so a body heats before it turns; one that
    // stops being ripe cools again. For 0.4 s after it condenses its speck
    // flares (`fresh`; 2 s at first read as noise, art director 11-14).
    var fresh = 0u;
    var dcnt = 0u;
    // A member is counted on its own (`FG_LUMP`, `prep`'s `lcb`), and its
    // lump's flag set in the cell (`FG_FLAGS`), so `prep` takes the look
    // of the lump that is there, not of the nearest centre.
    if (member) {
        dcnt = 1u << FG_LUMP;
    } else if (P.dense_at > 0u) {
        if ((dw & DENSE_BIT) != 0u) {
            cnt += 15u;
            dcnt += 15u;
            let age = f32(dw & 0xFFFFu);
            fresh = u32(round(4.0 * max(1.0 - age / 12.0, 0.0)));
        } else if (P.ripe_at > 0u) {
            let r = clamp(f32(dw & 0xFFFFu) / f32(P.ripe_at), 0.0, 1.0);
            dcnt = u32(round(4.0 * r * r));
        }
    }
    // The flare is calm brightness, which `base` lights as more walkers.
    add += fresh;
    // The walker in its fine cell for `prep`'s body (and, above bit 16,
    // counted cut off for its ashen fade); before the screen test, as
    // `prep` reads a rim past it.
    let kf = f32(P.fine);
    let gx = i32(floor((w.x - f32(P.ox)) * kf));
    let gy = i32(floor((w.y - f32(P.oy)) * kf));
    if (gx >= 0 && gy >= 0 && gx < i32(P.rw) && gy < i32(P.rh)) {
        atomicAdd(&fgrid[u32(gy) * P.rw + u32(gx)], cnt);
        if (dcnt > 0u) { atomicAdd(&fgrid[P.fhalf + u32(gy) * P.rw + u32(gx)], dcnt); }
        if (member) { atomicOr(&fgrid[P.fhalf + u32(gy) * P.rw + u32(gx)], 1u << (FG_FLAGS + lump_k - 1u)); }
    }
    let s = vec2<f32>((w.x - P.cx) * P.zoom, (w.y - P.cy) * P.zoom);
    let px = i32(floor(s.x)); let py = i32(floor(s.y));
    if (px < 0 || py < 0 || px >= i32(P.sw) || py >= i32(P.sh)) { return; }
    // A walker an explosion threw (the engine's `fling`, 11-14) has 1 in
    // its fourth word while it flies: a hot speck, a pixel more each side
    // per 4 of zoom, so the throw shows as the mould itself in the air
    // and not only as the streak it lays. It counts 6 calm walkers a pixel
    // (the calm bits, 0-9, so it never reads as cut off).
    // Since the gun look (2026-11-15) it is a droplet drawn out along its
    // flight (its heading, the third word): a round head zoom / 4 pixels
    // across the radius and a tail that thins behind it over 0.8 of the
    // cells it flies a tick (the fourth word, from the engine's `fly`; at
    // least half a cell, at most three), so a throw or a shot's spray reads
    // as mould flying out, its frames joined into one motion, and not as a
    // spray of squares (the square was 2 zoom / 4 + 1 pixels a side: 13 x
    // 13 at zoom 24). A shot's spray droplet slows to a stop, so its tail
    // shortens as it lands.
    // The bold spray (11-15, the owner 21:37Z on the share A/B: "I see no
    // difference"): the droplets added 6 calm walkers a pixel, and a calm
    // walker's light falls to under 0.02 in the body (`base`), so over the
    // slime a droplet all but vanished and the two shares differed in 5 of
    // 30 frames. Now each droplet is drawn in its own screen of words (the
    // specks' third, `atomicMax` of a layer and a heat): a head zoom / 3.2
    // pixels across the radius and its tail, inside a dark rim zoom / 12
    // pixels wide (at least one), so it reads over the bright body and
    // the dark ground alike. A shot's spray droplet (the engine writes its
    // flight below 0) is a killed walker blown out: hot while it is fast
    // (white-hot until 11-18, now the slime's gold glowing), cooling to
    // deep orange as it slows to land. A blast's throw is living mould and
    // keeps its gold.
    // In the slime's material (Look A3, 2026-11-18): the dark rim was an
    // ink outline, a cartoon chip beside a body drawn with light (the
    // 11-17 review: "outlined shapes in a different style from the slime
    // they come from"). Now a droplet is a bead in the slime's gold, lit by
    // the body's key light with its tight wet glint, so its upper left
    // catches the light and its lower right falls darker, and where the rim
    // stood it casts a soft shadow down and away from the light onto what
    // is under it. The shaded limb and the shadow keep it apart from the
    // bright body as the rim did.
    if (w.w != 0.0) {
        let spray = w.w < 0.0;
        let fl = abs(w.w);
        let rh = max(P.zoom / 3.2, 1.0);
        let len = max(clamp(0.8 * fl, 0.5, 3.0) * P.zoom, 2.0);
        let dir = vec2<f32>(cos(w.z), sin(w.z));
        let tail = s - dir * len;
        let l = normalize(LIGHT);
        // The shadow: the droplet's own shape, put DROP_SHADOW head radii
        // away from the light (screen pixels; the light's own direction on
        // the ground), at least a pixel.
        let sh = -normalize(l.xy) * max(DROP_SHADOW * rh, 1.0);
        // Heat by its flight a tick: about 0.9 cells for the fastest
        // droplet as it leaves (`spray_reach` 7 over 0.45 s; its tail at
        // zoom 12 about 9 pixels), under 0.2 as the slowest lands.
        // Four bits: 0 a blast's living throw (gold), 1-15 a spray's heat
        // from landing to leaving.
        var heat = 0.0;
        if (spray) { heat = clamp((fl - 0.2) / 0.6, 0.0, 1.0); }
        let pad = vec2<f32>(rh + 1.0);
        let lo = vec2<i32>(floor(min(s, tail) + min(sh, vec2<f32>(0.0)) - pad));
        let hi = vec2<i32>(ceil(max(s, tail) + max(sh, vec2<f32>(0.0)) + pad));
        for (var qy = max(lo.y, 0); qy <= min(hi.y, i32(P.sh) - 1); qy++) {
            for (var qx = max(lo.x, 0); qx <= min(hi.x, i32(P.sw) - 1); qx++) {
                let p = vec2<f32>(f32(qx) + 0.5, f32(qy) + 0.5);
                let t = clamp(dot(s - p, dir) / len, 0.0, 1.0);
                let r = rh * (1.0 - 0.7 * t);
                let on = s - dir * (t * len);
                let d = distance(p, on);
                // The layer: 2 the droplet (its heat in bits 4-7, the tail
                // cooler than the head; how squarely it faces the light in
                // bits 0-3), 1 its shadow.
                var v = 0u;
                if (d <= r) {
                    var h4 = 0u;
                    if (spray) { h4 = 1u + u32(round(14.0 * heat * (1.0 - 0.5 * t))); }
                    // A bead's normal: out from its axis, up by what is
                    // left of a sphere's radius.
                    let o = (p - on) / max(r, 0.5);
                    let n = normalize(vec3<f32>(o, sqrt(max(1.0 - dot(o, o), 0.0))));
                    let f = u32(round(15.0 * clamp((dot(n, l) + 0.3) / 1.3, 0.0, 1.0)));
                    v = (2u << 8u) | (h4 << 4u) | f;
                } else {
                    let ts = clamp(dot(s - (p - sh), dir) / len, 0.0, 1.0);
                    if (distance(p - sh, s - dir * (ts * len)) <= rh * (1.0 - 0.7 * ts)) { v = 1u << 8u; }
                }
                if (v != 0u) { atomicMax(&specks[2u * P.shalf + u32(qy) * P.stride + u32(qx)], v); }
            }
        }
        return;
    }
    // A mini-boss's member (2026-11-16) is drawn as itself, at every zoom:
    // a round dot in its lump's core colour (`base`, the specks' third
    // screen as layer 3, over the droplets), bright by its energy and
    // turning to its lump's hit colour by the hits it has taken. In a thick
    // body a calm speck's light is all but off, so members would not show
    // as ordinary specks. The word for `atomicMax`: hits on top (a hit
    // member's dot wins its pixel), then energy, the layer, the lump (bits
    // 4-5, for its fade and fill), the look.
    if (member) {
        let lk = min(u32(P.lumps[lump_k - 1u].z + 0.5), 15u);
        let hits = min(dw >> 28u, 15u);
        // Its energy (`alive`, with lives on) in eighths of `LUMP_FULL`:
        // a fed member burns bright, one run down far from food dims.
        var e8 = 8u;
        if (P.lives != 0u) { e8 = u32(round(8.0 * clamp(alive[k] / LUMP_FULL, 0.0, 1.0))); }
        let v = (hits << 14u) | (e8 << 10u) | (3u << 8u) | ((lump_k - 1u) << 4u) | lk;
        let r = max(P.zoom / 8.0, 0.6);
        let lo = vec2<i32>(floor(s - vec2<f32>(r + 1.0)));
        let hi = vec2<i32>(ceil(s + vec2<f32>(r + 1.0)));
        for (var qy = max(lo.y, 0); qy <= min(hi.y, i32(P.sh) - 1); qy++) {
            for (var qx = max(lo.x, 0); qx <= min(hi.x, i32(P.sw) - 1); qx++) {
                let p = vec2<f32>(f32(qx) + 0.5, f32(qy) + 0.5);
                if (distance(p, s) <= max(r, 0.71)) { atomicMax(&specks[2u * P.shalf + u32(qy) * P.stride + u32(qx)], v); }
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

// A flung walker's droplet (`draw_specks`, the bold spray, 11-15): a
// blast's living throw, and a shot's spray from fast (hot) to landing
// (cool). Since 11-18 a lit bead with a shadow, not a chip in a dark rim
// (`DROP_RIM` (34, 18, 6) at 0.8, a rim zoom / 12 pixels wide).
// The shadow: how far it falls from the droplet in head radii, and how
// much light it takes from what is under it.
const DROP_SHADOW: f32 = 0.9;
const DROP_SHADE: f32 = 0.5;
// The bead's shading: the body's curve (`shaded`, 1 + 1.25 (n.l - l.z),
// floor 0.3) made a bead a few pixels across mostly its dark limb, a
// brown pebble at zoom 6, so a gentler slope and a higher floor.
const DROP_SLOPE: f32 = 0.8;
const DROP_FLOOR: f32 = 0.55;
// A shot's spray: a killed walker blown out, the slime's gold while it is
// fast, glowing this share of the way to `DROP_HEAT` over its shading
// (so even the fastest keeps a shaded side), cooling to deep orange as it
// slows to land. A blast's throw is living mould: gold, no glow.
// (Tried 11-18 and not kept: the 11-15 white-hot (`DROP_HOT` (255, 246,
// 214)) at 0.6 and at 0.3 over a base a fifth to a third of the way to
// it: pale cream or grey-brown seeds, not the slime.)
const DROP_GLOW: f32 = 0.4;
const DROP_HEAT: vec3<f32> = vec3<f32>(255.0, 238.0, 180.0);
const DROP_GOLD: vec3<f32> = vec3<f32>(255.0, 196.0, 70.0);
const DROP_COOL: vec3<f32> = vec3<f32>(238.0, 118.0, 24.0);
const BACK: vec3<f32> = vec3<f32>(6.0, 8.0, 10.0);
// Off the map in the shaded view: darker than any ground or rock.
// The micro-look's strength: the first cut (one) read only as a faint
// grain at zoom 12 over ground at about 40 in grey.
const PLAIN_AMP: f32 = 2.0;
const FILM_LIFT_A: f32 = 0.01;
const FILM_LIFT_B: f32 = 0.05;
const OFF_MAP_RGB: vec3<f32> = vec3<f32>(12.0, 13.0, 16.0);

// A measuring aid (off in the game): with it on, each shaded pixel is drawn
// as its class instead (red: 10 soil, 20 rock, 30 soft rock, 40 rubble,
// 50 crystal, 70 water, 80 off the map; green: the slime film's thickness
// x 255; blue: soft rock's health x 255), so a frame from a build with it
// registers every pixel of the same frame from the game's build. Used for
// the value-order tables (Reports/value-order-2026-11-15).
const DEBUG_ID: bool = false;
fn debug_id(x: f32, y: f32) -> vec3<f32> {
    if (x < 0.0 || y < 0.0 || x >= f32(P.ww) || y >= f32(P.wh)) { return vec3<f32>(80.0, 0.0, 0.0); }
    let g = ground[cidx(i32(floor(x)), i32(floor(y)))];
    let k = g & 15u;
    var cls = 10.0;
    if (k == ROCK) { cls = 20.0; } else if (k == WALL) { cls = 30.0; } else if (k == RUBBLE) { cls = 40.0; } else if (k == DOOR) { cls = 60.0; }
    else if (k == CRYSTAL) { cls = 50.0; } else if (k == WET) { cls = 70.0; }
    return vec3<f32>(cls, clamp(film_th(x, y), 0.0, 1.0) * 255.0, byte(g, 1u) * 255.0);
}

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
            // Where rock or a wall covers the pixel whole, its top is all
            // that shows: the ground under it is not shaded (performance
            // thread, 11-15: rock and the dark beyond the map's edge, which
            // is rock, were shaded and then painted over; at zoom 1 that is
            // most of the screen on a map smaller than it).
            let ta = terrain_at(fx, fy);
            let n = atomicExchange(&specks[y * P.stride + x], 0u);
            if (ta.cov >= 1.0) {
                c = terrain_top(ta);
                // The specks' light needs the body count only where a
                // walker is.
                under_body = 0.0;
                if ((n & 1023u) != 0u) { under_body = mat_at(fx, fy).body; }
            } else {
                c = shaded(fx, fy, 0.75 / f32(P.fine));
                c = terrain_over(c, fx, fy);
            }
            // Off the map: its own dark value, under any rock (the planet
            // review 11-15: cave rock was the backdrop's grey, so the map's
            // edge vanished). Off-world cells still stand as rock, so the
            // map is rimmed by rock that fades into the dark over a cell.
            let outside = max(max(-fx, fx - f32(P.ww)), max(-fy, fy - f32(P.wh)));
            if (outside > 0.0) { c = mix(c, OFF_MAP_RGB, smoothstep(0.0, 1.5, outside)); }
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
            // An absorbed walker's streak (`draw_absorbed`) takes its
            // warm-white outright, as the phases below do.
            let hot = atomicExchange(&specks[P.shalf + y * P.stride + x], 0u);
            c = mix(c, ABSORB_RGB, min(f32(hot) * 0.3, 0.9));
            // A flung walker's droplet (`draw_specks`, the bold spray): its
            // shadow on what is under it, then the droplet, lit by the
            // body's key light with the dense body's wet glint (`shaded`:
            // exponent 14 + 30 x 0.95), a spray's colour by its heat.
            let drop = atomicExchange(&specks[2u * P.shalf + y * P.stride + x], 0u);
            if ((drop >> 8u) == 1u) { c *= 1.0 - DROP_SHADE; }
            if ((drop >> 8u) == 2u) {
                let h4 = (drop >> 4u) & 15u;
                var dc = DROP_GOLD;
                var hh = 0.0;
                if (h4 > 0u) {
                    hh = f32(h4 - 1u) / 14.0;
                    dc = mix(DROP_COOL, DROP_GOLD, hh);
                }
                let fl = f32(drop & 15u) / 15.0 * 1.3 - 0.3;
                let lz = normalize(LIGHT).z;
                let shade = clamp(1.0 + DROP_SLOPE * (fl - lz), DROP_FLOOR, 1.4);
                let spec = pow(max(fl, 0.0), 42.0) * 1.1;
                var lit = dc * shade + vec3<f32>(255.0, 250.0, 236.0) * spec;
                lit = mix(lit, DROP_HEAT, DROP_GLOW * hh * hh * (3.0 - 2.0 * hh));
                c = mix(c, min(lit, vec3<f32>(255.0)), 0.95);
            }
            // A mini-boss's member (`draw_specks`): its lump's core colour
            // (cyan as a leech drinks), white-hot by the hits it has taken,
            // going out with its lump's look as it fades.
            if (((drop >> 8u) & 3u) == 3u) {
                let lk = drop & 15u;
                let li = (drop >> 4u) & 3u;
                let hit = min(f32((drop >> 14u) & 15u) / 2.0, 1.0);
                let e = f32((drop >> 10u) & 15u) / 8.0;
                let mc = mix(lump_glow(lk, P.lump_fill[li]), lump_hit(lk), hit);
                c = mix(c, mc, mix(LUMP_SPECK * (0.3 + 0.7 * e), 0.95, hit) * clamp(P.lump_fade[li], 0.0, 1.0));
            }
            // Cut-off walkers' specks take their phase's colour outright
            // (added light was lost on the bright body: no pixel of a
            // bolting piece read red on the first film).
            c = mix(c, vec3<f32>(255.0, 178.0, 40.0), min(f32((n >> 10u) & 1023u) * (0.4 / 15.0), 0.75));
            c = mix(c, vec3<f32>(235.0, 52.0, 40.0), min(f32(n >> 20u) * (0.5 / 15.0), 0.85));
            // Rock and walls only dim: the scent there is 0, and its lines
            // would trace their faces.
            if (P.scent_view != 0u) {
                if (ta.cov >= 1.0) { c *= 0.32; } else { c = scent_view(c, fx, fy); }
            }
            if (DEBUG_ID) { c = debug_id(fx, fy); }
        } else if (fx >= 0.0 && fy >= 0.0 && fx < f32(P.ww) && fy < f32(P.wh)) {
            if (P.zoom >= 1.0) {
                c = cell(fx, fy);
            } else {
                // Zoomed out: the mean of the cells under the pixel (up to
                // 4 x 4 samples), so thin tubes fade rather than flicker.
                let size = 1.0 / P.zoom;
                c = boxed(wx, wy, size, min(u32(ceil(size)), 4u));
            }
            if (P.scent_view != 0u && P.shade == 1u) { c = scent_view(c, fx, fy); }
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
const FLASH: u32 = 10u;

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
    let c = atlas[u32(clamp(y, 0, 159)) * 2176u + u32(clamp(x, 0, 2175))];
    let a = f32(c >> 24u) / 255.0;
    return vec4<f32>(unpack(c) * a, a);
}

// Tile `k` (0 body, 1 helmet, 2 flash, 3 dead; 4-15 the buildings, each
// body then lamp: turret, lure, pump, outpost, siren, larder; 16 the
// turret's head) at point `p` in radii
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
//
// The turret (kind 0; owner's playtest 11-14 08:15Z: "turrets need to look
// better"): a base that stays put and a head (tile 16) turned to `facing`,
// the game's aim, so the gun points at what it shoots. `pulse` is its shot:
// the head kicks back along the barrels by up to 0.22 radii and the muzzle
// flash (its lamp tile) shows, both fading with the pulse. `state` lights
// the lamp on the head's back: 0 idle (dark, nothing in reach), 1 tracking
// (amber, mould in reach and food to fire), 2 dry (red, and the head
// greyed: the bank cannot pay for a shot). Every standing building gets
// the soldier's rim toward the light (#227).
fn turret_frame(q: vec2<f32>, ca: f32, sa: f32, recoil: f32) -> vec2<f32> {
    return vec2<f32>(q.x * ca + q.y * sa + recoil, -q.x * sa + q.y * ca);
}

// A mortar's tube (kind 7, the turrets thread 2026-11-17) at `h`, the
// head's frame (+x its facing, radii): a dark cradle, a short wide steel
// tube with its round mouth at the far end, the bore dark inside it, so
// from above it reads as a barrel tipped up at the sky. As shapes, crisp
// at any zoom. `recoil` slides the tube back into its cradle.
fn mortar_head(h: vec2<f32>, recoil: f32) -> vec4<f32> {
    let aa = 0.05;
    let cc = cover(length(h) - 0.44, aa);
    var out = vec4<f32>(STEEL_DARK * cc, cc);
    let c = vec2<f32>(0.22 - recoil, 0.0);
    let bx = abs(h - c) - vec2<f32>(0.46, 0.22);
    let dt = length(max(bx, vec2<f32>(0.0))) + min(max(bx.x, bx.y), 0.0) - 0.06;
    let ct = cover(dt, aa);
    let round = mix(STEEL, STEEL_DARK, clamp(abs(h.y) / 0.3, 0.0, 1.0) * 0.7);
    out = pm_over(out, vec4<f32>(round * ct, ct));
    let dm = length(h - vec2<f32>(0.66 - recoil, 0.0));
    let ring = cover(dm - 0.27, aa);
    out = pm_over(out, vec4<f32>(STEEL * 1.08 * ring, ring));
    let bore = cover(dm - 0.17, aa);
    out = pm_over(out, vec4<f32>(vec3<f32>(14.0, 13.0, 12.0) * bore, bore));
    return out;
}

// The building's own texels at `q` (radii): its tile, and a turret's head
// over its base (a mortar's tube over the turret's base).
fn building_body(k: u32, q: vec2<f32>, small: bool, turret: bool, ca: f32, sa: f32, recoil: f32, dry: bool, mortar: bool) -> vec4<f32> {
    // A pump (tile 8) turns whole, its hose into its crystal (11-14).
    var body = sprite(k, select(q, turret_frame(q, ca, sa, 0.0), k == 8u), small);
    if (turret) {
        var head = sprite(16u, turret_frame(q, ca, sa, recoil), small);
        if (mortar) { head = mortar_head(turret_frame(q, ca, sa, 0.0), recoil); }
        if (dry) {
            let g = dot(head.xyz, vec3<f32>(0.3, 0.59, 0.11));
            head = vec4<f32>(mix(head.xyz, vec3<f32>(g), 0.75) * 0.8, head.w);
        }
        body = pm_over(body, head);
    }
    return body;
}

const LAMP_TRACK: vec3<f32> = vec3<f32>(255.0, 190.0, 80.0);
const LAMP_DRY: vec3<f32> = vec3<f32>(240.0, 64.0, 52.0);
const LAMP_IDLE: vec3<f32> = vec3<f32>(70.0, 72.0, 60.0);
const CALL_RED: vec3<f32> = vec3<f32>(255.0, 90.0, 60.0);
const BITE_HOT: vec3<f32> = vec3<f32>(255.0, 120.0, 72.0);

// ---- The pump's rig (kind 6, `Marks::pump_rig`, 2026-11-15) ----
//
// The owner (2026-11-15): a pump "should probably just change the whole
// appearance of the crystal like you've built some sort of structure
// around it". Drawn round the crystal, not over it, so the crystal's own
// charge (the simulation, dimming as it is pumped) stays in sight: a
// steel hexagon just outside the crystal's pointy-top hexagon, a pylon at
// each corner with a lamp, and three struts to the pump head in the middle
// (a building sprite, kind 2, drawn over it). Shapes at screen resolution,
// so crisp at any zoom (owner, 11-14: play is zoomed in). What changes is
// the game's state: it is laid corner by corner clockwise from the top as
// it is built (the rest a faint plan), darkens as it is hurt, and the
// struts' charge lines and the lamps glow by its draw this frame
// (`pulse`, `Building::drew` of its full draw). Its level (1-3, `state`
// 0-2) adds an inner ring a level.
//
// Sizes are shares of the rig's radius R (twice the mark's half): the
// crystal is 1/1.3 of R (`pump_rig_r` in the window).
fn sd_hex_pointy(p: vec2<f32>, rc: f32) -> f32 {
    // iq's hexagon with x and y swapped, so a corner is at the top.
    let k = vec3<f32>(-0.8660254, 0.5, 0.57735027);
    let ri = rc * 0.8660254;
    var q = abs(vec2<f32>(p.y, p.x));
    q -= 2.0 * min(dot(k.xy, q), 0.0) * k.xy;
    q -= vec2<f32>(clamp(q.x, -k.z * ri, k.z * ri), ri);
    return length(q) * sign(q.y);
}

fn sd_seg(p: vec2<f32>, a: vec2<f32>, b: vec2<f32>) -> f32 {
    let pa = p - a; let ba = b - a;
    let h = clamp(dot(pa, ba) / dot(ba, ba), 0.0, 1.0);
    return length(pa - ba * h);
}

const STEEL: vec3<f32> = vec3<f32>(150.0, 160.0, 170.0);
const STEEL_DARK: vec3<f32> = vec3<f32>(70.0, 78.0, 88.0);

fn pump_rig(d: vec2<f32>, r: f32, built: f32, health: f32, working: bool, pulse: f32, level: u32) -> vec4<f32> {
    let big = 2.0 * r;
    let p = d / big;
    let aa = 1.0 / big;
    let l = normalize(LIGHT);
    // Clockwise from the top: 0 at the top corner, 1 all the way round.
    let turn = fract(atan2(p.x, -p.y) / 6.2831853 + 1.0);
    let laid = turn <= built;
    let f = 0.84;
    let t = max(0.04, 1.6 * aa);
    // The frame (and an inner ring a level), the pylons, the struts.
    var sd_frame = abs(sd_hex_pointy(p, f)) - t;
    for (var i = 1u; i <= min(level, 2u); i++) {
        sd_frame = min(sd_frame, abs(sd_hex_pointy(p, f - 0.07 * f32(i))) - 0.45 * t);
    }
    var sd_pylon = 1e9;
    var sd_lamp = 1e9;
    var sd_strut = 1e9;
    var sd_core = 1e9;
    let hub = 0.27;
    for (var i = 0u; i < 6u; i++) {
        let a = -1.5707963 + 1.0471976 * f32(i);
        let c = vec2<f32>(cos(a), sin(a)) * f;
        sd_pylon = min(sd_pylon, length(p - c) - max(0.085, 2.5 * aa));
        sd_lamp = min(sd_lamp, length(p - c) - max(0.035, 1.2 * aa));
        if (i % 2u == 0u) {
            let e = vec2<f32>(cos(a), sin(a)) * hub;
            sd_strut = min(sd_strut, sd_seg(p, c, e) - max(0.022, 1.2 * aa));
            sd_core = min(sd_core, sd_seg(p, c, e) - max(0.008, 0.5 * aa));
        }
    }
    let solid = min(min(sd_frame, sd_pylon), sd_strut);
    // The frame's shadow, cast away from the light.
    let sh = abs(sd_hex_pointy(p + vec2<f32>(l.x, l.y) * 0.035, f)) - t;
    var out = vec4<f32>(0.0, 0.0, 0.0, 0.3 * cover(sh, aa * 2.0) * select(0.0, 1.0, laid && built > 0.0));
    // A dark outline a pixel and a half round the steel, so it parts from
    // slime and ground alike (the buildings' rule, #227).
    let ink = cover(solid - OUTLINE * aa, aa);
    let body = cover(solid, aa);
    // Steel lit from the light's side: brighter where the surface faces it.
    let lit = 0.82 + 0.3 * clamp(dot(normalize(p + vec2<f32>(1e-5)), -vec2<f32>(l.x, l.y)), -1.0, 1.0);
    var steel = STEEL * lit;
    if (sd_strut < min(sd_frame, sd_pylon)) { steel = mix(STEEL_DARK, STEEL, 0.55) * lit; }
    let grey = dot(steel, vec3<f32>(0.3, 0.59, 0.11));
    let hurt = 1.0 - health;
    steel = mix(steel, vec3<f32>(grey), 0.6 * hurt) * (1.0 - 0.45 * hurt);
    if (laid) {
        out = pm_over(out, vec4<f32>(INK * ink, ink));
        out = pm_over(out, vec4<f32>(steel * body, body));
        if (built >= 1.0) {
            // Charge in the struts and the lamps, by the draw.
            let g = select(0.0, clamp(0.25 + 0.75 * pulse, 0.0, 1.0), working);
            let core = cover(sd_core, aa) * g;
            out = pm_over(out, vec4<f32>(CHARGE * core, core));
            let lamp = cover(sd_lamp, aa);
            let lc = mix(LAMP_IDLE, CHARGE_HOT, g);
            out = pm_over(out, vec4<f32>(lc * lamp, lamp));
            let halo = exp(-max(sd_lamp, 0.0) * max(sd_lamp, 0.0) / 0.0012) * 0.6 * g * (1.0 - lamp);
            out = vec4<f32>(out.xyz + CHARGE * halo, min(out.w + halo, 1.0));
        }
    } else {
        // The plan: a cream line round the steel to come, faint inside.
        let edge = clamp(ink - body, 0.0, 1.0) * 0.8;
        out = pm_over(out, vec4<f32>(RIM * edge, edge));
        out = pm_over(out, vec4<f32>(steel * body * 0.25, body * 0.25));
    }
    if (out.w <= 0.0) { return vec4<f32>(0.0); }
    return vec4<f32>(out.xyz / out.w, out.w);
}

fn building(d: vec2<f32>, r: f32, kind: u32, built: f32, health: f32, working: bool, pulse: f32, facing: f32, state: u32) -> vec4<f32> {
    if (kind == 6u) { return pump_rig(d, r, built, health, working, pulse, state); }
    let aa = 1.0 / max(r, 1.0);
    let l = normalize(LIGHT);
    let p = d / r;
    let small = r < 12.0;
    let mortar = kind == 7u;
    let k = select(4u + 2u * min(kind, 5u), 4u, mortar);
    let turret = kind == 0u || mortar;
    let ca = cos(facing); let sa = sin(facing);
    let recoil = select(0.0, pulse * 0.22, turret);
    let dry = turret && state == 2u && built >= 1.0;
    var body = building_body(k, p, small, turret, ca, sa, recoil, dry, mortar);
    // The silhouette's edge, OUTLINE pixels out (eight taps): a dark
    // outline round a standing building, as round a soldier, so it parts
    // from the slime (art review 2026-11-14), and the cream line of a
    // ghost's plan.
    let o = OUTLINE / r;
    var grown = 0.0;
    for (var i = 0u; i < 8u; i++) {
        let a = f32(i) * 0.785398;
        grown = max(grown, building_body(k, p + vec2<f32>(cos(a), sin(a)) * o, small, turret, ca, sa, recoil, false, mortar).w);
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
    if (built >= 1.0) {
        // The rim: inside the silhouette, where a pixel toward the light
        // is outside it (the soldier's, #227).
        let lp = vec2<f32>(l.x, l.y) / r * 1.2;
        let rim = clamp(body.w - building_body(k, p + lp, small, turret, ca, sa, recoil, false, mortar).w, 0.0, 1.0) * 0.6;
        out = vec4<f32>(mix(out.xyz, RIM * out.w, rim), out.w);
    }
    if (built < 1.0) {
        let seam = clamp(1.0 - abs(p.y - line) * r / 1.2, 0.0, 1.0) * max(body.w, edge);
        out = pm_over(out, vec4<f32>(RIM * seam, seam));
    }
    if (turret) {
        if (built >= 1.0) {
            let ph = turret_frame(p, ca, sa, recoil);
            // The lamp on the head's back.
            let lc = select(select(LAMP_IDLE, LAMP_TRACK, state == 1u), LAMP_DRY, state == 2u);
            let dl = length(ph - vec2<f32>(-0.46, 0.0));
            let la = cover(dl - 0.11, aa * 2.0);
            out = pm_over(out, vec4<f32>(lc * la, la));
            if (state != 0u) {
                let g = exp(-dl * dl / 0.02) * 0.5;
                out = vec4<f32>(out.xyz + lc * g * out.w, out.w);
            }
            if (pulse > 0.0 && !mortar) {
                out = pm_over(out, sprite(k + 1u, ph, small) * clamp(pulse * 1.5, 0.0, 1.0));
            }
            if (pulse > 0.0 && mortar) {
                // A shell leaving: the bore flares, then smokes.
                let dm = length(turret_frame(p, ca, sa, 0.0) - vec2<f32>(0.66 - recoil, 0.0));
                let g = pulse * exp(-dm * dm / 0.03);
                out = vec4<f32>(out.xyz + vec3<f32>(255.0, 214.0, 150.0) * g * out.w, out.w);
            }
        }
    } else {
        let ps = select(p, turret_frame(p, ca, sa, 0.0), k == 8u);
        if (working && built >= 1.0) { out = pm_over(out, sprite(k + 1u, ps, small)); }
        if (pulse > 0.0 && built >= 1.0) {
            let lamp = sprite(k + 1u, ps, small);
            // A store (the larder, an outpost) flares charge cyan as a load
            // is banked; a siren red as it calls (11-14: it went white-cyan
            // in the store's colours), its lamp less blown out.
            let store = kind == 3u || kind == 5u;
            let tint = select(CALL_RED, CHARGE, store);
            let g = pulse * select(0.8, 1.5, store);
            out = vec4<f32>(out.xyz + lamp.xyz * g, min(out.w + lamp.w * g, 1.0));
            // A halo round the lit cells.
            let lit = sprite(k + 1u, ps * 0.8, true).w;
            out = vec4<f32>(out.xyz + tint * lit * pulse * 0.5, min(out.w + lit * pulse * 0.5, 1.0));
            // The whole store washed in charge light, on the body only (no
            // new alpha, so it claims no neighbour's pixels): the lit cells
            // alone were a few pixels at zoom 12 and lost under the carriers
            // walking in (first film, 11-14).
            let wash = pulse * exp(-dot(p, p) / 0.6) * 0.45;
            out = vec4<f32>(out.xyz + tint * wash * out.w, out.w);
        }
    }
    if (out.w <= 0.0) { return vec4<f32>(0.0); }
    return vec4<f32>(out.xyz / out.w, out.w);
}

fn building_at(b: u32, d: vec2<f32>, r: f32) -> vec4<f32> {
    let extra = marks[b + 11u];
    return building(d, r, u32(bitcast<f32>(marks[b + 9u])), f32(extra & 255u) / 255.0, f32((extra >> 8u) & 255u) / 255.0, (extra & 65536u) != 0u, f32((extra >> 17u) & 255u) / 255.0, bitcast<f32>(marks[b + 8u]), (extra >> 25u) & 3u);
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
    if (shape == FLASH) {
        // A muzzle flash's jets reach about its length from the mouth.
        let rr = 1.35 * half(b).x + 4.0;
        let lo = max(s - vec2<f32>(rr), vec2<f32>(0.0));
        let hi = min(s + vec2<f32>(rr), vec2<f32>(f32(P.sw) - 1.0, f32(P.sh) - 1.0));
        if (hi.x < lo.x || hi.y < lo.y) { return; }
        let w = u32(hi.x - lo.x) + 1u;
        let total = w * (u32(hi.y - lo.y) + 1u);
        for (var j = t0; j < total; j += 64u * SPLIT) {
            let q = floor(lo) + vec2<f32>(f32(j % w), f32(j / w));
            let g = flash_at(b, q + vec2<f32>(0.5));
            if (g.x > 0.004 || g.y > 0.0) { claim(i32(q.x), i32(q.y), k, false); }
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
    // The core tapers from its full radius at the head to under half at
    // the tail, a comet rather than a rod (gun look, 2026-11-15; before,
    // the same width all along read as a flat dash at zoom 24).
    let rc = r * mix(0.45, 1.0, t);
    let core = clamp(rc - d + 0.5, 0.0, 1.0) * mix(0.35, 1.0, along);
    return vec2<f32>(select(halo, 0.0, d > 3.0 * r), core);
}

// One flame jet of a muzzle flash, in the flash's frame `q` (pixels, +x
// out of the barrel): from the barrel's mouth at angle `a`, `len` pixels
// long, `w` pixels across the radius at its root, tapering to a point.
// Returns (glow, core) as `streak_at`.
fn jet(q: vec2<f32>, a: f32, len: f32, w: f32) -> vec2<f32> {
    let dir = vec2<f32>(cos(a), sin(a));
    let t = dot(q, dir);
    if (t > len + 3.0 * w) { return vec2<f32>(0.0); }
    let tt = clamp(t / max(len, 1.0), 0.0, 1.0);
    var dd = abs(q.x * dir.y - q.y * dir.x);
    if (t < 0.0) { dd = length(q); }
    if (t > len) { dd = distance(q, dir * len); }
    let fall = 1.0 - tt;
    let r = w * pow(fall, 0.6) + 0.35;
    let halo = exp(-(dd * dd) / (2.0 * 4.8 * w * w)) * fall;
    let core = clamp(r - dd + 0.5, 0.0, 1.0) * mix(0.55, 1.0, fall);
    return vec2<f32>(halo, core);
}

// A muzzle flash (`Marks::muzzle_flash`, gun look 2026-11-15): a white-hot
// ball at the barrel's mouth and flame jets out of it, shaped by the gun
// (a rifle's long jet and two side tongues, a carbine's short fat burst,
// a long gun's long thin jet with its brake venting sideways, a
// scattergun's fan of five), each shot's lengths and angles varied by its
// seed. It shrinks and dims as `left` burns down. Light, as a streak.
// Before, a flat orange oval (the sprite's flash tile) sat at the barrel
// for the flash's 0.07 s, the same every shot.
fn flash_at(b: u32, p: vec2<f32>) -> vec2<f32> {
    let len = half(b).x;
    let ang = bitcast<f32>(marks[b + 9u]);
    let ex = marks[b + 11u];
    let kind = ex & 3u;
    let seed = f32((ex >> 2u) & 255u);
    let left = f32((ex >> 10u) & 255u) / 255.0;
    let d = p - anchor(b);
    if (dot(d, d) > 1.7 * len * len + 16.0) { return vec2<f32>(0.0); }
    let ca = cos(ang); let sa = sin(ang);
    let q = vec2<f32>(d.x * ca + d.y * sa, -d.x * sa + d.y * ca);
    let l = len * (0.45 + 0.55 * left);
    let w = max(l * 0.12, 0.8);
    let h1 = hash2(vec2<f32>(seed, 1.7)); let h2 = hash2(vec2<f32>(seed, 5.3));
    let h3 = hash2(vec2<f32>(seed, 9.1)); let h4 = hash2(vec2<f32>(seed, 13.9));
    // The ball at the barrel's mouth.
    let rb = w * 1.15;
    let db = length(q);
    var g = vec2<f32>(0.8 * exp(-(db * db) / (2.0 * 1.6 * rb * rb)), clamp(rb - db + 0.5, 0.0, 1.0));
    var j = vec2<f32>(0.0);
    if (kind == 3u) {
        // The scattergun's fan, one jet a pellet's worth of it.
        for (var i = 0u; i < 5u; i++) {
            let hi = hash2(vec2<f32>(seed, 20.0 + f32(i)));
            let a = (f32(i) - 2.0) * 0.24 + (hi - 0.5) * 0.14;
            let jj = jet(q, a, l * (0.55 + 0.3 * hi), w * 0.85);
            j = vec2<f32>(j.x + jj.x, max(j.y, jj.y));
        }
    } else {
        var fwd = vec2<f32>(1.0, 1.0);
        var side = vec3<f32>(0.95, 0.38, 0.7);
        if (kind == 1u) { fwd = vec2<f32>(0.72, 1.25); side = vec3<f32>(1.15, 0.46, 0.85); }
        if (kind == 2u) { fwd = vec2<f32>(1.0, 0.75); side = vec3<f32>(1.5, 0.3, 0.55); }
        let f0 = jet(q, (h1 - 0.5) * 0.12, l * fwd.x * (0.85 + 0.3 * h2), w * fwd.y);
        let fa = side.x + (h3 - 0.5) * 0.25;
        let s1 = jet(q, fa, l * side.y * (0.75 + 0.5 * h4), w * side.z);
        let s2 = jet(q, -fa, l * side.y * (0.75 + 0.5 * h1), w * side.z);
        j = vec2<f32>(f0.x + s1.x + s2.x, max(f0.y, max(s1.y, s2.y)));
    }
    g = vec2<f32>(min(g.x + 0.8 * j.x, 1.0), max(g.y, j.y));
    return vec2<f32>(g.x * sqrt(left), g.y * (0.35 + 0.65 * left));
}

fn soldier_at(b: u32, d: vec2<f32>, r: f32) -> vec4<f32> {
    let extra = marks[b + 11u];
    var f = soldier(d, r, bitcast<f32>(marks[b + 9u]), unpack(marks[b + 7u]), f32(extra & 255u) / 255.0, (extra & 256u) != 0u, f32((extra >> 9u) & 255u) / 255.0, f32((extra >> 17u) & 255u) / 255.0);
    // A bite (`Marks::soldier_bitten`, 11-14): the side the slime bit from
    // flushes hot, fading with the flinch, on the body only (no new alpha).
    let w = marks[b + 8u];
    let bite = f32(w & 255u) / 255.0;
    if (bite > 0.0 && f.w > 0.0) {
        let a = f32(w >> 8u) / 65535.0 * 6.2831853;
        let side = clamp(dot(d / r, vec2<f32>(cos(a), sin(a))) * 1.4 + 0.3, 0.0, 1.0);
        f = vec4<f32>(mix(f.xyz, BITE_HOT, bite * side * 0.75), f.w);
    }
    return f;
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
        if ((marks[b + 6u] == STREAK || marks[b + 6u] == FLASH) && !in_minimap(x, y)) {
            let pq = vec2<f32>(f32(x) + 0.5, f32(y) + 0.5);
            var g = vec2<f32>(0.0);
            if (marks[b + 6u] == STREAK) { g = streak_at(b, pq); } else { g = flash_at(b, pq); }
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
