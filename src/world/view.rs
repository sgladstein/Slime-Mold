//! **The view pass**: draws a world-sized RGBA frame (the GPU engine's
//! `frame`, or any buffer of `u32` RGBA a cell) onto the screen through a
//! `Camera`, with a minimap sampled from the same frame, the game's
//! `Marks`, and a screen-space overlay for HUD text. Three compute passes
//! (`view.wgsl`); the result is a screen-sized buffer the window copies to
//! its texture. It knows nothing of the game or the mould.
//!
//! **Why a pass of its own**, not the engine's `draw` drawing at the
//! camera: the engine draws each cell once a frame whatever the camera
//! does, and the view only resamples that picture, so a game can change
//! how the world is coloured (crystals, terrain, species) without touching
//! the camera, and the camera costs the same on any dish.
//!
//! The output's rows are `stride` pixels apart (the screen's width rounded
//! up to 64), because a buffer-to-texture copy needs rows a multiple of
//! 256 bytes.

use pixels::wgpu;

use super::camera::Camera;
use super::marks::{Marks, MARK_WORDS, MAX_MARKS};

pub const SHADER: &str = include_str!("view.wgsl");

/// The soldier sprites, rendered from `sprites/soldier.svg` by
/// `sprites/render.js` (the owner's pick, 2026-11-11 16:48Z).
pub const SPRITES: &[u8] = include_bytes!("sprites/sprites.png");

/// Bytes in the uniform (44 words, then the four lumps' vec4s since the
/// mini-bosses, 2026-11-16, and their fade, fill and held vec4s).
/// The slime's own look (Look B, 2026-11-19): the owner asked for several
/// looks, each for a slime, so you tell them apart on sight (00:08Z: "It
/// would also be interesting instead of having a single look to build
/// multiple looks could eventually apply to different slimes"). Each is
/// drawn from that slime's own walkers, never painted on.
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub enum SlimeLook {
    /// Every walker's film a soft disc round it: the look before 11-19.
    #[default]
    Plain = 0,
    /// The Sprawler's fast threads: each walker lays its film along the
    /// way it has just come (`draw_specks`' `lay_streak`), so a fast walker
    /// is a dash and walkers on one path join into a thread.
    Threads = 1,
    /// The Drifter's soft sheet: its own trail, which fades fast
    /// (`keep` 0.93) and spreads wide (sensors at 90 degrees, 18 cells
    /// out), shows as a thin gel round and behind its walkers
    /// (`SHEET_VEIL` in `view.wgsl`), so the body trails a fading wake
    /// as it drifts.
    Sheet = 2,
    /// The Trunk's cords (play-zoom pass, 11-20): its trail, kept 0.95,
    /// runs as long straight cords between its few big bodies, and off the
    /// body it stands up as a rounded, translucent tube (`CORD_VEIL`,
    /// `CORD_RISE` in `view.wgsl`), so the network you cut to starve it is
    /// seen. Plain, the cords were a few loose specks on bare ground.
    Cords = 3,
    /// The Swarmer's swell (11-20): it walks toward the alarm a strike
    /// lays, so where its alarm is, its body stands up, rounder and taller
    /// (`SWELL_GAIN`, `SWELL_RISE` in `view.wgsl`), and its loose walkers
    /// there are beads, not a flat glow: the swarm visibly gathers where
    /// it was hit. Plain, an alarmed Swarmer was a flat orange haze.
    Swell = 4,
    /// The Sleeper's crust (11-20): its dormant walkers (the engine's life
    /// state 2, `set_life`) are a dull, matte, darker crust, and the ones
    /// awake keep the gold, so a wave waking reads as gold spreading over
    /// the crust (`DORMANT_DIM`, `DORMANT_MATTE` in `view.wgsl`).
    Dormant = 5,
    /// The Grudge's scars (11-20): it remembers where it was hurt (the
    /// engine's scar, `set_scar`) and shuns that ground, so where the scar
    /// lies its body is knotted, darker scar tissue and bare ground there
    /// keeps a faint bruise (`SCAR_FULL`, `SCAR_BRUISE` in `view.wgsl`):
    /// you see where it will not come back to.
    Scarred = 6,
    /// The Sporer's spores (11-21): each spore in flight (the engine writes
    /// its flight into the walker's fourth word) is a small round bead with
    /// a short tail, a seed blown off the body, not a droplet's long
    /// tracer streak (`SPORE_HEAD`, `SPORE_TAIL` in `view.wgsl`).
    Spores = 7,
}

/// The game seconds of a walker's way that `SlimeLook::Threads` draws.
pub const STREAK_SECS: f32 = 0.1;

impl SlimeLook {
    /// The look for a slime of `tier` (`None`: the sandbox's plain mould).
    pub fn of(tier: Option<crate::mould::tiers::Tier>) -> SlimeLook {
        match tier {
            Some(crate::mould::tiers::Tier::Sprawler) => SlimeLook::Threads,
            Some(crate::mould::tiers::Tier::Drifter) => SlimeLook::Sheet,
            Some(crate::mould::tiers::Tier::Trunk) => SlimeLook::Cords,
            Some(crate::mould::tiers::Tier::Swarmer) => SlimeLook::Swell,
            Some(crate::mould::tiers::Tier::Sleeper) => SlimeLook::Dormant,
            Some(crate::mould::tiers::Tier::Grudge) => SlimeLook::Scarred,
            Some(crate::mould::tiers::Tier::Sporer) => SlimeLook::Spores,
            _ => SlimeLook::Plain,
        }
    }

    /// Every look, in the order the slime viewer lists them.
    pub const ALL: [SlimeLook; 8] = [SlimeLook::Plain, SlimeLook::Threads, SlimeLook::Sheet, SlimeLook::Cords, SlimeLook::Swell, SlimeLook::Dormant, SlimeLook::Scarred, SlimeLook::Spores];

    /// Its name, as the tweaks file writes it (`body TIER = NAME`).
    pub fn name(self) -> &'static str {
        match self {
            SlimeLook::Plain => "plain",
            SlimeLook::Threads => "threads",
            SlimeLook::Sheet => "sheet",
            SlimeLook::Cords => "cords",
            SlimeLook::Swell => "swell",
            SlimeLook::Dormant => "crust",
            SlimeLook::Scarred => "scars",
            SlimeLook::Spores => "seeds",
        }
    }

    /// By name (`name`).
    pub fn parse(s: &str) -> Option<SlimeLook> {
        SlimeLook::ALL.into_iter().find(|l| l.name() == s)
    }
}

const PARAMS_BYTES: usize = 320;
/// Cells round the shown ones that the look's samples reach: the slope
/// (1.5), the cast shadow (up to 1.8 toward the light) and the bilinear
/// blend (1) all fit in 4.
const MARGIN: u32 = 4;
/// The bindings after the first twenty, in `bind`'s order: 20 the lights,
/// then 21-24 the crystals' live charge, their heat (`drain`), the terrain
/// cells (`tcell`) and the crystal cells listed, and 25-26 the engine's two
/// germ fields, whose second block is the alarm (`set_alarm`), 27 each
/// walker's life word (`set_life`) and 28 the engine's damage memory, whose
/// second block is the scar (`set_scar`).
const BINDINGS_AFTER: [u32; 9] = [LIGHTS_BINDING, 21, 22, 23, 24, 25, 26, 27, 28];

/// The most point lights a frame (`View::set_lights`); `shaded` loops over
/// them a pixel, so the cap bounds that loop.
pub const MAX_LIGHTS: usize = 64;

/// The mini-bosses' bodies the view tells apart (`set_lumps`); the
/// engine's `BigParams::lumps` holds at most this many.
pub const MAX_LUMPS: usize = 4;

/// How a mini-boss's body is drawn (`set_lumps`, 2026-11-16). Each is a
/// material the lump's members carry over the ordinary slime's, by their
/// share of the mould round a cell: a body colour, a colour it lights
/// with from inside where its members pack, and how high and glossy it
/// stands (`LUMP_*` in `view.wgsl`). Not bronze: the owner took bronze off
/// the mini-bosses (2026-11-16 04:58Z), condensed slime keeps it.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u32)]
pub enum LumpLook {
    /// Drawn as the ordinary slime.
    Plain = 0,
    /// Crimson skin over an orange core.
    Ember = 1,
    /// Violet skin over a pale violet core (the hunter's).
    Violet = 2,
    /// Near-black skin over a pale lilac core.
    Ink = 3,
    /// Pale bone-grey skin over a faint warm core; hits show rose.
    Bone = 4,
    /// **A fruit** (2026-11-20, `larder_world::fruit`): umber over a warm
    /// cream core, darkening from rust as it ripens (its strength).
    Fruit = 5,
    /// **A mortar** (2026-11-20, the Enemy lane, `BossKind::Mortar`): deep
    /// moss skin over an acid yellow-green core, a hue no other kind or
    /// the slime uses, so the thing throwing at you is found at once.
    Mortar = 6,
    /// **A pinwheel** (2026-11-20, the Enemy lane, `BossKind::Pinwheel`):
    /// cobalt skin over an ice-blue core, the one blue on the slime, so
    /// its turning arms read apart from the gold round them.
    Pinwheel = 7,
    /// **The last-stand heart** (2026-11-20, the Enemy lane,
    /// `larder_world::stand`): a deep amber over a hot amber core, the
    /// slime's own gold run dark and hot, dimming as it is cut down (its
    /// strength).
    Heart = 8,
}

impl LumpLook {
    /// By name, as the look keys take it (`boss_look=`).
    pub fn parse(s: &str) -> Option<Self> {
        Some(match s {
            "plain" | "0" => Self::Plain,
            "ember" | "1" => Self::Ember,
            "violet" | "2" => Self::Violet,
            "ink" | "3" => Self::Ink,
            "bone" | "4" => Self::Bone,
            "fruit" | "5" => Self::Fruit,
            "mortar" | "6" => Self::Mortar,
            "pinwheel" | "7" => Self::Pinwheel,
            "heart" | "8" => Self::Heart,
            _ => return None,
        })
    }
}

/// One mini-boss's body as the frame draws it (`View::set_lumps`).
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct LumpBody {
    /// Its centre in world cells (where two lumps' members meet, a cell
    /// takes the nearer centre's look).
    pub at: (f32, f32),
    pub look: LumpLook,
    /// 0-1: a stronger one stands taller and its core burns brighter.
    pub strength: f32,
    /// How much of its look is left, 1 whole to 0 gone: a broken or spent
    /// lump's look drains out over its fade (the game's `lump_looks` bits
    /// 8-15) while its members are still marked, so it does not pop back
    /// to slime.
    pub remain: f32,
    /// 0-1, what it has drunk (a leech's take from the pump it sits on):
    /// its core light turns to the crystals' charge cyan and burns
    /// brighter by it.
    pub fill: f32,
    /// 0-1, how far its members' own density, not only their share of
    /// the walkers there, sets where its look shows: 1 once it is out on
    /// its errand (the game's `lump_looks` bit 3), so a held body reads
    /// whole wherever its members stand, even spread thin through the
    /// sheet (a leech ringing its pump: 123 members in a band 14-18 cells
    /// round it, about 0.36 a cell in the thick slime there); 0 while it
    /// gathers, when its colour seeps through the sheet by its share.
    pub held: f32,
}

impl Default for LumpBody {
    fn default() -> Self {
        LumpBody { at: (0.0, 0.0), look: LumpLook::Plain, strength: 0.0, remain: 1.0, fill: 0.0, held: 0.0 }
    }
}

/// The lights a frame uploads (`View::set_lights`): those whose reach
/// touches the world rectangle `seen` (x0, y0, x1, y1, cells), in the order
/// given, and past `MAX_LIGHTS` the brightest at their middles (`strength`,
/// so a light late in its fade goes first; strength x reach squared would
/// drop every muzzle light, 1 cell, before any burst). Until 2026-11-15 the first 64 pushed were kept, off
/// screen or not, so with 40 soldiers the muzzle lights, pushed after the
/// bursts and hits, were the ones dropped (the whole-game review, 11-15);
/// lights off screen also cost every pixel a loop turn for nothing.
pub fn pick_lights(lights: &[Light], seen: (f32, f32, f32, f32)) -> Vec<Light> {
    let (x0, y0, x1, y1) = seen;
    let mut on: Vec<Light> = lights.iter().copied().filter(|l| l.at.0 + l.reach >= x0 && l.at.0 - l.reach <= x1 && l.at.1 + l.reach >= y0 && l.at.1 - l.reach <= y1).collect();
    if on.len() > MAX_LIGHTS {
        // A bite zone first: it is the slime's attack, not a glow.
        let worth = |l: &Light| if l.is_bite() { f32::INFINITY } else { l.strength };
        let mut order: Vec<usize> = (0..on.len()).collect();
        order.sort_by(|&a, &b| worth(&on[b]).total_cmp(&worth(&on[a])).then(a.cmp(&b)));
        let mut keep = vec![false; on.len()];
        for &k in &order[..MAX_LIGHTS] {
            keep[k] = true;
        }
        let mut k = 0;
        on.retain(|_| {
            k += 1;
            keep[k - 1]
        });
    }
    on
}
/// The lights' binding: 15-19 are the graphics thread's (#212 and its
/// crystal work), so the lights sit after them.
const LIGHTS_BINDING: u32 = 20;

/// A point light (`View::set_lights`): something in the game that gives
/// off light, lighting the world's own relief. `at` and `reach` in world
/// cells, `lift` its height above the ground in cells, `colour` 0-255 and
/// `strength` (1 about a lamp's worth at its centre). A `lift` below 0 is
/// a scent light (a burning lure's, 11-14): `-lift` high, lighting each
/// spot only as far as the engine's scent there, so the glow is the scent
/// field itself.
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct Light {
    pub at: (f32, f32),
    pub reach: f32,
    pub lift: f32,
    pub colour: [f32; 3],
    pub strength: f32,
}

/// A bite zone's `lift` (`Light::bite`): above any real light's.
pub const BITE_LIFT: f32 = 1.0e9;

impl Light {
    /// The slime biting a soldier (the owner, 2026-11-18 18:08Z: a bite
    /// "just looks like more of the same slime"): the disc of `reach` world
    /// cells round `at` in which every walker bites it, sent with the
    /// lights. It lights nothing: the slime in it takes its own crimson
    /// (`view.wgsl`, `bite_share`) by `strength`, 0 to 1.
    pub fn bite(at: (f32, f32), reach: f32, strength: f32) -> Light {
        Light { at, reach, lift: BITE_LIFT, colour: [0.0; 3], strength }
    }

    pub fn is_bite(&self) -> bool {
        self.lift >= BITE_LIFT
    }
}

/// The minimap's longest side, in pixels, and its margin from the corner.
pub const MINIMAP_SIDE: u32 = 240;
pub const MINIMAP_MARGIN: u32 = 8;

/// Where the minimap sits: bottom right, the world's shape, at most
/// `MINIMAP_SIDE` across and a fifth of the screen's width. `None` when
/// the screen is too small for one.
pub fn minimap_rect(world: (usize, usize), screen: (u32, u32)) -> Option<(u32, u32, u32, u32)> {
    let side = MINIMAP_SIDE.min(screen.0 / 5).min(screen.1 / 3);
    if side < 32 {
        return None;
    }
    let (ww, wh) = (world.0 as f32, world.1 as f32);
    let (w, h) = if ww >= wh { (side, ((side as f32) * wh / ww).round().max(1.0) as u32) } else { (((side as f32) * ww / wh).round().max(1.0) as u32, side) };
    Some((screen.0 - w - MINIMAP_MARGIN, screen.1 - h - MINIMAP_MARGIN, w, h))
}

/// The minimap fitted inside a box (the UI's frame for it), centred, the
/// world's shape.
pub fn minimap_in(world: (usize, usize), bx: (u32, u32, u32, u32)) -> (u32, u32, u32, u32) {
    let (x, y, w, h) = bx;
    let (ww, wh) = (world.0 as f32, world.1 as f32);
    let k = (w as f32 / ww).min(h as f32 / wh);
    let (mw, mh) = ((ww * k).floor().max(1.0) as u32, (wh * k).floor().max(1.0) as u32);
    (x + (w - mw) / 2, y + (h - mh) / 2, mw, mh)
}

/// Which minimap pixel rectangle a screen point is on, as the world point
/// under it.
pub fn minimap_world(rect: (u32, u32, u32, u32), world: (usize, usize), at: (f32, f32)) -> Option<(f32, f32)> {
    let (x, y, w, h) = rect;
    let (u, v) = (at.0 - x as f32, at.1 - y as f32);
    (u >= 0.0 && v >= 0.0 && u < w as f32 && v < h as f32).then(|| (u / w as f32 * world.0 as f32, v / h as f32 * world.1 as f32))
}

pub struct View {
    device: wgpu::Device,
    queue: wgpu::Queue,
    pipes: [wgpu::ComputePipeline; 6],
    bgl: wgpu::BindGroupLayout,
    params: wgpu::Buffer,
    world_frame: wgpu::Buffer,
    overlay: wgpu::Buffer,
    marks: wgpu::Buffer,
    vis: wgpu::Buffer,
    /// The look (`set_look`): the engine's look field, the game's ground
    /// field, the walkers, and walkers a screen pixel.
    look: wgpu::Buffer,
    ground: wgpu::Buffer,
    walkers: wgpu::Buffer,
    /// Three screens of words: the walkers' specks (`draw_specks`), then the
    /// absorbed walkers' streaks (`draw_absorbed`, 11-14), a buffer half of
    /// their own so the specks' calm/lean/bolt fields keep their room, then
    /// the flung walkers' droplets (the bold spray, 11-15: a layer and a
    /// heat a pixel, by `atomicMax`; since 11-18 the heat in four bits and
    /// how squarely the bead faces the light in the other four).
    specks: wgpu::Buffer,
    /// Each walker's energy, when the world grows (`set_alive`): a slot
    /// below 0 is empty or dead and draws no speck.
    alive: wgpu::Buffer,
    /// This frame's point lights (`set_lights`), two vec4s each.
    lights: wgpu::Buffer,
    n_lights: u32,
    lives: bool,
    /// Each walker's life word, the engine's `spec` (`set_life`; Look B,
    /// 2026-11-20): bits 9-10 its state, 2 dormant. Read only with the
    /// Sleeper's look; a 16-byte stand-in otherwise.
    life: wgpu::Buffer,
    /// Whether `life` is the engine's.
    life_on: bool,
    /// The engine's damage memory (`set_scar`; its `crater`, whose second
    /// block is the Grudge's scar a cell), for the Grudge's look; a 16-byte
    /// stand-in otherwise.
    scar: wgpu::Buffer,
    /// Whether `scar` is the engine's.
    scar_on: bool,
    /// Food's news (`set_news`): the engine's news (the step each walker
    /// last heard of food) and state (the step now) buffers, and the
    /// engine's lag and lean in steps.
    news: wgpu::Buffer,
    state: wgpu::Buffer,
    /// The walkers a fine cell of `prep`'s rectangle (and, above bit 16,
    /// those cut off from food), counted each frame by `draw_specks`.
    fgrid: wgpu::Buffer,
    news_on: bool,
    /// Dense slime condenses (`set_dense`): where each walker's condensing
    /// word starts in the bound `news` (words; 0 off).
    dense_at: u32,
    /// The mini-bosses' bodies (`set_lumps`, 2026-11-16): each lump's
    /// centre in cells, its look (`LumpLook`, 0 drawn as ordinary slime)
    /// and its strength 0-1, up to `MAX_LUMPS`; then each one's look left
    /// (`LumpBody::remain`) and its fill.
    lumps: [[f32; 4]; MAX_LUMPS],
    lump_fade: [f32; MAX_LUMPS],
    lump_fill: [f32; MAX_LUMPS],
    lump_held: [f32; MAX_LUMPS],
    /// The plants' first cell block in `news` (words; 0 off,
    /// `set_plants`).
    plant_at: u32,
    /// The crust (`set_crust`, 2026-11-17): the word in `news` where the
    /// engine's crust block starts (`gpu::Engine::crust_at` / 4), 0 off.
    crust_at: u32,
    /// Set slime (`set_setting`, E2b): the set walkers a cell's first word
    /// in `news`, 0 off.
    setc_at: u32,
    /// The Mind statuses (`set_minds`, Look A5, 2026-11-18): the word in
    /// `news` where each walker's Mind word starts (0 off), and the engine
    /// steps over which a status fades as it runs out.
    mind_at: u32,
    mind_fade: u32,
    /// The Lure's steering field (`set_steer`, 2026-11-18): the word in
    /// `news` where its blocks start, 0 hidden.
    steer_at: u32,
    /// How far that field reaches, cells (`set_steer_reach`), 0 unknown.
    steer_reach: f32,
    /// The slime's own look (`set_slime_look`, Look B, 2026-11-19) and
    /// how far behind each walker its streak runs, cells.
    slime_look: SlimeLook,
    streak: f32,
    flora: u32,
    ripe_at: u32,
    lag: u32,
    lean: u32,
    /// The crystals drawn as hexagons (`set_crystals`): centres and radii,
    /// and each world cell's crystal (index + 1) near a footprint.
    gems: wgpu::Buffer,
    gem_cell: wgpu::Buffer,
    n_gems: u32,
    /// The crystals' live charge (`set_charge`): the engine's charge a cell
    /// (binding 21), each crystal's full charge a cell (written into
    /// `gems`' fourth word), the crystal cells listed (binding 24) and each
    /// one's last charge and how fast it is being taken (binding 22,
    /// `drain`). `live` 0 off, 1 on, 2 for a first frame that only reads.
    charge: wgpu::Buffer,
    heat: wgpu::Buffer,
    gem_list: wgpu::Buffer,
    /// The engine's germ fields, both sides (bindings 25-26; `set_alarm`),
    /// which is current (`set_alarm_side`), and the alarm that reads full.
    germ: [wgpu::Buffer; 2],
    alarm_side: u32,
    alarm_full: f32,
    n_list: u32,
    gem_words: Vec<f32>,
    live: std::cell::Cell<u32>,
    /// Game seconds (`set_time`) now and at the last `record`.
    game_t: f32,
    last_t: std::cell::Cell<f32>,
    /// Living walkers a cell for the look's body (0: the calibration's).
    body_density: f32,
    /// The soldier sprites (`SPRITES`), RGBA a texel.
    atlas: wgpu::Buffer,
    /// Each shown cell's height and material, worked out once a frame by
    /// the `prep` pass (`view.wgsl`), sized for the screen at zoom 1.
    hcell: wgpu::Buffer,
    mcell: wgpu::Buffer,
    /// Each shown cell's hard terrain, smoothed (`view.wgsl`, `tcell`).
    tcell: wgpu::Buffer,
    n_walk: u32,
    shade: bool,
    /// The screen-sized result, rows `stride` pixels apart.
    pub out: wgpu::Buffer,
    /// Which mark each screen pixel shows (cleared as it is read).
    top: wgpu::Buffer,
    group: wgpu::BindGroup,
    pub world: (usize, usize),
    pub screen: (u32, u32),
    pub stride: u32,
    /// Whether the minimap is drawn (M toggles it in the sandbox).
    pub minimap: bool,
    /// Where the UI wants it: a box it is fitted into (`minimap_in`);
    /// `None` the bottom-right corner.
    pub minimap_box: Option<(u32, u32, u32, u32)>,
    /// Hidden for now (a full-screen menu is up: the unframed minimap sat
    /// on the run shop's armory; UI review, 11-15).
    pub minimap_off: bool,
    /// The scent view (F2 in Larder World, 11-15): the world dimmed under
    /// the food scent (blue, with contour lines) and the trail (pink), the
    /// two fields a walker smells. Needs the look (`set_look`).
    pub scent: bool,
    has_overlay: bool,
    has_vis: bool,
    n_marks: u32,
}

fn storage(dev: &wgpu::Device, label: &str, bytes: u64) -> wgpu::Buffer {
    crate::gpu::note_bytes(bytes.max(16));
    dev.create_buffer(&wgpu::BufferDescriptor { label: Some(label), size: bytes.max(16), usage: wgpu::BufferUsages::STORAGE | wgpu::BufferUsages::COPY_DST | wgpu::BufferUsages::COPY_SRC, mapped_at_creation: false })
}

impl View {
    /// A view of `world_frame` (`world.0 x world.1` RGBA `u32`s) on a screen
    /// `screen` pixels.
    pub fn new(device: &wgpu::Device, queue: &wgpu::Queue, world_frame: &wgpu::Buffer, world: (usize, usize), screen: (u32, u32)) -> Self {
        // `LARDER_VIEW_WGSL=PATH` swaps in another copy of this shader, for
        // timing a change on the 1070 without a rebuild (performance
        // thread, 11-15: each part's cost measured by leaving it out).
        let source = std::env::var("LARDER_VIEW_WGSL").ok().and_then(|p| std::fs::read_to_string(p).ok()).unwrap_or_else(|| SHADER.to_string());
        let module = device.create_shader_module(wgpu::ShaderModuleDescriptor { label: Some("view"), source: wgpu::ShaderSource::Wgsl(source.into()) });
        let entry = |binding: u32, ty: wgpu::BufferBindingType| wgpu::BindGroupLayoutEntry {
            binding,
            visibility: wgpu::ShaderStages::COMPUTE,
            ty: wgpu::BindingType::Buffer { ty, has_dynamic_offset: false, min_binding_size: None },
            count: None,
        };
        let ro = wgpu::BufferBindingType::Storage { read_only: true };
        let rw = wgpu::BufferBindingType::Storage { read_only: false };
        let entries = [entry(0, wgpu::BufferBindingType::Uniform), entry(1, ro), entry(2, ro), entry(3, ro), entry(4, rw), entry(5, ro), entry(6, rw), entry(7, ro), entry(8, ro), entry(9, ro), entry(10, rw), entry(11, ro), entry(12, rw), entry(13, rw), entry(14, ro), entry(15, ro), entry(16, ro), entry(17, rw), entry(18, ro), entry(19, ro), entry(LIGHTS_BINDING, ro), entry(21, ro), entry(22, rw), entry(23, rw), entry(24, ro), entry(25, ro), entry(26, ro), entry(27, ro), entry(28, ro)];
        let bgl = device.create_bind_group_layout(&wgpu::BindGroupLayoutDescriptor { label: Some("view"), entries: &entries });
        let layout = device.create_pipeline_layout(&wgpu::PipelineLayoutDescriptor { label: Some("view"), bind_group_layouts: &[Some(&bgl)], immediate_size: 0 });
        let pipe = |entry_point: &str| {
            device.create_compute_pipeline(&wgpu::ComputePipelineDescriptor { label: Some(entry_point), layout: Some(&layout), module: &module, entry_point: Some(entry_point), compilation_options: Default::default(), cache: None })
        };
        let pipes = [pipe("base"), pipe("draw_marks"), pipe("compose"), pipe("draw_specks"), pipe("prep"), pipe("drain")];
        let params = device.create_buffer(&wgpu::BufferDescriptor { label: Some("view params"), size: PARAMS_BYTES as u64, usage: wgpu::BufferUsages::UNIFORM | wgpu::BufferUsages::COPY_DST, mapped_at_creation: false });
        let marks = storage(device, "marks", (MAX_MARKS * MARK_WORDS * 4) as u64);
        let vis = storage(device, "visibility", 16);
        let (stride, out, overlay, top) = Self::screen_buffers(device, screen);
        let (look, ground, walkers) = (storage(device, "look", 16), storage(device, "ground", 16), storage(device, "walkers", 16));
        let specks = storage(device, "specks", stride as u64 * screen.1 as u64 * 12);
        let atlas = Self::atlas(device, queue);
        let (hcell, mcell, tcell) = Self::cell_buffers(device, screen);
        let alive = storage(device, "alive", 16);
        let life = storage(device, "life", 16);
        let scar = storage(device, "scar", 16);
        let lights = storage(device, "lights", (MAX_LIGHTS * 32) as u64);
        let (news, state) = (storage(device, "news", 16), storage(device, "state", 16));
        let fgrid = Self::fine_buffer(device, screen);
        let (gems, gem_cell) = (storage(device, "crystals", 16), storage(device, "crystal cells", 16));
        let (charge, heat, gem_list) = (storage(device, "charge", 16), storage(device, "crystal heat", 16), storage(device, "crystal list", 16));
        let germ = [storage(device, "germ 0", 16), storage(device, "germ 1", 16)];
        let group = Self::bind(device, &bgl, [&params, world_frame, &overlay, &marks, &out, &vis, &top, &look, &ground, &walkers, &specks, &atlas, &hcell, &mcell, &alive, &news, &state, &fgrid, &gems, &gem_cell, &lights, &charge, &heat, &tcell, &gem_list, &germ[0], &germ[1], &life, &scar]);
        View {
            device: device.clone(),
            queue: queue.clone(),
            pipes,
            bgl,
            params,
            world_frame: world_frame.clone(),
            overlay,
            marks,
            vis,
            look,
            ground,
            walkers,
            specks,
            alive,
            lights,
            n_lights: 0,
            lives: false,
            life,
            life_on: false,
            scar,
            scar_on: false,
            news,
            state,
            fgrid,
            news_on: false,
            dense_at: 0,
            lumps: [[0.0; 4]; MAX_LUMPS],
            lump_fade: [1.0; MAX_LUMPS],
            lump_fill: [0.0; MAX_LUMPS],
            lump_held: [0.0; MAX_LUMPS],
            plant_at: 0,
            crust_at: 0,
            mind_at: 0,
            mind_fade: 1,
            setc_at: 0,
            steer_at: 0,
            steer_reach: 0.0,
            slime_look: SlimeLook::Plain,
            streak: 0.0,
            flora: 0,
            ripe_at: 0,
            lag: 0,
            lean: 0,
            gems,
            gem_cell,
            n_gems: 0,
            charge,
            heat,
            gem_list,
            germ,
            alarm_side: 0,
            alarm_full: 0.0,
            n_list: 0,
            gem_words: Vec::new(),
            live: std::cell::Cell::new(0),
            game_t: 0.0,
            last_t: std::cell::Cell::new(0.0),
            body_density: 0.0,
            atlas,
            hcell,
            mcell,
            tcell,
            n_walk: 0,
            shade: false,
            out,
            top,
            group,
            world,
            screen,
            stride,
            minimap: true,
            minimap_box: None,
            minimap_off: false,
            scent: false,
            has_overlay: false,
            has_vis: false,
            n_marks: 0,
        }
    }

    /// The soldier sprite atlas on the GPU: `SPRITES` decoded, a u32 RGBA
    /// a texel, 3200 x 160 (25 tiles).
    fn atlas(dev: &wgpu::Device, queue: &wgpu::Queue) -> wgpu::Buffer {
        let img = image::load_from_memory(SPRITES).expect("sprites.png").to_rgba8();
        assert_eq!(img.dimensions(), (3200, 160), "the atlas the view's shader expects");
        let b = storage(dev, "sprites", img.as_raw().len() as u64);
        queue.write_buffer(&b, 0, img.as_raw());
        b
    }

    fn screen_buffers(dev: &wgpu::Device, screen: (u32, u32)) -> (u32, wgpu::Buffer, wgpu::Buffer, wgpu::Buffer) {
        let stride = screen.0.div_ceil(64) * 64;
        let out = storage(dev, "view out", stride as u64 * screen.1 as u64 * 4);
        let overlay = storage(dev, "overlay", screen.0 as u64 * screen.1 as u64 * 4);
        let top = storage(dev, "mark claims", stride as u64 * screen.1 as u64 * 4);
        (stride, out, overlay, top)
    }

    /// The cells `prep` works out: at zoom 1 or more the screen shows at
    /// most its own size in cells, plus the margin (`MARGIN` each side).
    /// Cell heights hold three grids: the whole height, the mould's own
    /// (`film_th` in the shader) and the bare ground's colour (`ground_est`;
    /// 11-14).
    fn cell_buffers(dev: &wgpu::Device, screen: (u32, u32)) -> (wgpu::Buffer, wgpu::Buffer, wgpu::Buffer) {
        let n = (screen.0 + 2 * MARGIN + 2) as u64 * (screen.1 + 2 * MARGIN + 2) as u64;
        (storage(dev, "cell heights", n * 12), storage(dev, "cell materials", n * 16), storage(dev, "cell terrain", n * 4))
    }

    /// `prep`'s fine cells, at most as many as `cell_buffers`' cells: the
    /// rectangle is the screen over the zoom and a margin, `fine` (at most
    /// a third of the zoom) cells each way.
    /// Two grids of `fine_half` words: the walkers a fine cell (and the
    /// cut-off ones), then the condensing weights (`set_dense`: condensed
    /// and ripening in the low 16 bits, fresh absorptions above).
    fn fine_buffer(dev: &wgpu::Device, screen: (u32, u32)) -> wgpu::Buffer {
        // Three halves (a third since 11-20): the walkers, the dense and
        // lump counts, and the Sleeper's dormant and awake counts.
        storage(dev, "fine walkers", Self::fine_half(screen) as u64 * 12)
    }

    fn fine_half(screen: (u32, u32)) -> u32 {
        (screen.0 + 2 * MARGIN + 2) * (screen.1 + 2 * MARGIN + 2)
    }

    /// Fine cells a cell each way at zoom `zoom` (`prep`; 2026-11-14): one
    /// below zoom 6, else 2. Four (from zoom 12) made the 9 x 9 window too
    /// small for a Gaussian wide enough not to look lumpy: square blobs.
    pub fn fine(zoom: f32) -> u32 {
        ((zoom / 3.0).floor() as u32).clamp(1, 2)
    }

    /// The bind group: `bufs` in binding order, 0-19 then `BINDINGS_AFTER`.
    fn bind(dev: &wgpu::Device, bgl: &wgpu::BindGroupLayout, bufs: [&wgpu::Buffer; 29]) -> wgpu::BindGroup {
        let binding = |i: usize| if i < 20 { i as u32 } else { BINDINGS_AFTER[i - 20] };
        let entries: Vec<wgpu::BindGroupEntry> = bufs.iter().enumerate().map(|(i, b)| wgpu::BindGroupEntry { binding: binding(i), resource: b.as_entire_binding() }).collect();
        dev.create_bind_group(&wgpu::BindGroupDescriptor { label: Some("view"), layout: bgl, entries: &entries })
    }

    fn rebind(&mut self) {
        self.group = Self::bind(&self.device, &self.bgl, [&self.params, &self.world_frame, &self.overlay, &self.marks, &self.out, &self.vis, &self.top, &self.look, &self.ground, &self.walkers, &self.specks, &self.atlas, &self.hcell, &self.mcell, &self.alive, &self.news, &self.state, &self.fgrid, &self.gems, &self.gem_cell, &self.lights, &self.charge, &self.heat, &self.tcell, &self.gem_list, &self.germ[0], &self.germ[1], &self.life, &self.scar]);
    }

    /// A new world frame (a restart made a new engine).
    pub fn set_world(&mut self, world_frame: &wgpu::Buffer, world: (usize, usize)) {
        self.world_frame = world_frame.clone();
        self.world = world;
        if self.has_vis {
            self.set_visibility(None);
        }
        self.clear_look();
        self.clear_news();
        self.n_gems = 0;
        self.clear_charge();
        self.rebind();
    }

    /// Shades the world from the simulation (`view.wgsl`, *The look*): the
    /// engine's `look` field (a u32 a cell), the game's `ground` field (a
    /// u32 a cell: kind, value; see `look::ground`), and `n_walk` walkers
    /// (`vec4<f32>` x, y, heading, _ in cells) drawn a speck each. Bound
    /// buffers, so they follow the engine with no copy; set again after a
    /// restart (`set_world` drops them).
    pub fn set_look(&mut self, look: &wgpu::Buffer, ground: &wgpu::Buffer, walkers: &wgpu::Buffer, n_walk: u32) {
        (self.look, self.ground, self.walkers, self.n_walk, self.shade) = (look.clone(), ground.clone(), walkers.clone(), n_walk, true);
        self.rebind();
    }

    /// With growth on, the walkers' energy buffer (an `f32` a slot, below 0
    /// for an empty or dead slot), so the look draws specks for the living
    /// only. Without it every slot is a speck: a growing world's empty
    /// slots sit stacked at the dish's centre and its dead where they
    /// died, so the Larder world drew a bright dot in the middle and the
    /// ghosts of every walker its shots killed (W2c, 2026-11-12).
    /// Each walker's life word (the engine's `spec`, Look B 2026-11-20),
    /// for the Sleeper's look: its dormant crust drawn dull and matte
    /// (`SlimeLook::Dormant`). None: no life cycle on this world.
    pub fn set_life(&mut self, spec: Option<&wgpu::Buffer>) {
        (self.life, self.life_on) = match spec {
            Some(b) => (b.clone(), true),
            None => (storage(&self.device, "life", 16), false),
        };
        self.rebind();
    }

    /// The engine's damage memory (its `crater` buffer: the crater's
    /// countdown a cell, then the scar a cell), for the Grudge's scars
    /// (`SlimeLook::Scarred`, Look B 2026-11-20). None: not shown.
    pub fn set_scar(&mut self, crater: Option<&wgpu::Buffer>) {
        (self.scar, self.scar_on) = match crater {
            Some(b) => (b.clone(), true),
            None => (storage(&self.device, "scar", 16), false),
        };
        self.rebind();
    }

    pub fn set_alive(&mut self, energy: Option<&wgpu::Buffer>) {
        (self.alive, self.lives) = match energy {
            Some(b) => (b.clone(), true),
            None => (storage(&self.device, "alive", 16), false),
        };
        self.rebind();
    }

    /// Shows each walker's news of food (plan C8, 2026-11-14): the engine's
    /// `news` (binding 48: the step each walker last heard of food, its
    /// first `n_walk` words) and `state` (word 0 the step now) buffers, and
    /// its `wither_lag` and `panic_lean` in steps. A walker whose news is
    /// older than the lag is cut off: its speck turns amber while it leans
    /// and red once it bolts (the phases the engine's `panic_to` steers
    /// by), and the mould round a cell fades to ash by the share of its
    /// walkers cut off, so a piece cut from food greys as it starves and
    /// withers. `None` turns it off; a restart (`set_world`) drops it.
    pub fn set_news(&mut self, news: Option<(&wgpu::Buffer, &wgpu::Buffer)>, lag: u32, lean: u32) {
        match news {
            Some((n, s)) => {
                (self.news, self.state, self.news_on, self.lag, self.lean) = (n.clone(), s.clone(), true, lag, lean);
            }
            None => self.clear_news(),
        }
        self.rebind();
    }

    /// Draws the crystals as hexagons at screen resolution (2026-11-14):
    /// each crystal's centre and circumradius in cells (the pointy-top
    /// hexagon its footprint is cut from, `look::dish_crystals`), and
    /// `crystal`, each world cell's crystal (index + 1, 0 none; the dish's
    /// `pile_of`). The charge is still each cell's (the ground's). An empty
    /// list draws them from the cells, as before; a restart (`set_world`)
    /// drops them.
    pub fn set_crystals(&mut self, gems: &[((f32, f32), f32)], crystal: &[u16]) {
        let (w, h) = self.world;
        if gems.is_empty() || crystal.len() != w * h {
            self.n_gems = 0;
            return;
        }
        // Each cell within one of a footprint names its crystal, so a pixel
        // on a cell the hexagon's edge only cuts through still finds it.
        let mut near = vec![0u32; w * h];
        for y in 0..h {
            for x in 0..w {
                let k = crystal[y * w + x] as u32;
                if k == 0 || k as usize > gems.len() {
                    continue;
                }
                for yy in y.saturating_sub(1)..(y + 2).min(h) {
                    for xx in x.saturating_sub(1)..(x + 2).min(w) {
                        near[yy * w + xx] = k;
                    }
                }
            }
        }
        let g: Vec<f32> = gems.iter().flat_map(|&((x, y), r)| [x, y, r, 0.0]).collect();
        self.gems = storage(&self.device, "crystals", (g.len() * 4) as u64);
        self.queue.write_buffer(&self.gems, 0, &g.iter().flat_map(|v| v.to_le_bytes()).collect::<Vec<u8>>());
        self.gem_words = g;
        // The crystal cells listed, for `drain`.
        let list: Vec<u32> = (0..w * h).filter(|&i| crystal[i] as usize >= 1 && crystal[i] as usize <= gems.len()).map(|i| i as u32).collect();
        self.gem_list = storage(&self.device, "crystal list", (list.len() * 4) as u64);
        self.queue.write_buffer(&self.gem_list, 0, &crate::gpu::u32_bytes(&list));
        self.n_list = list.len() as u32;
        self.clear_charge();
        self.gem_cell = storage(&self.device, "crystal cells", (near.len() * 4) as u64);
        self.queue.write_buffer(&self.gem_cell, 0, &crate::gpu::u32_bytes(&near));
        self.n_gems = gems.len() as u32;
        self.rebind();
    }

    /// The crystals drawn from their live charge (after `set_crystals`):
    /// `charge` the engine's charge a cell (`Engine::charge`), `full` each
    /// crystal's full charge a cell (`BigDish::pile_fill`). The look then
    /// follows the charge every frame, not the ground's once a second: a
    /// crystal sinks, dims and greys where it is taken, and a cell being
    /// taken glows (`drain`, by the game seconds `set_time` gives). `None`
    /// goes back to the ground's charge.
    pub fn set_charge(&mut self, charge: Option<(&wgpu::Buffer, &[f32])>) {
        match charge {
            Some((buf, full)) if self.n_gems > 0 && full.len() * 4 == self.gem_words.len() => {
                for (k, &f) in full.iter().enumerate() {
                    self.gem_words[4 * k + 3] = f;
                }
                self.queue.write_buffer(&self.gems, 0, &self.gem_words.iter().flat_map(|v| v.to_le_bytes()).collect::<Vec<u8>>());
                self.charge = buf.clone();
                self.heat = storage(&self.device, "crystal heat", (self.world.0 * self.world.1 * 8) as u64);
                self.live.set(2);
                self.rebind();
            }
            _ => self.clear_charge(),
        }
    }

    /// Shows the engine's alarm field (2026-11-14, the art director: a
    /// siren's call drawn from the real field, not rings timed from the
    /// call): `germ` the engine's two germ fields (`gpu::Engine::germ`),
    /// whose second block of cells is the alarm, and `full` the alarm that
    /// reads at full strength (a siren's `siren_alarm`). Since 2026-11-17
    /// the view draws it on the slime alone, as the slime's own colour:
    /// slime under alarm flushes hot (`view.wgsl`, `flush_at`), spreading
    /// and fading as the engine spreads and decays the field, so the slime
    /// a shot makes flinch shows it, and bare ground shows nothing. (To
    /// then: a pale haze with a brighter front over ground and slime
    /// alike, which washed a firing squad's ground mint.) `None` hides it.
    /// Each frame `set_alarm_side` says which side is current.
    pub fn set_alarm(&mut self, germ: Option<[&wgpu::Buffer; 2]>, full: f32) {
        match germ {
            Some([a, b]) => {
                self.germ = [a.clone(), b.clone()];
                self.alarm_full = full.max(1e-3);
            }
            None => {
                self.germ = [storage(&self.device, "germ 0", 16), storage(&self.device, "germ 1", 16)];
                self.alarm_full = 0.0;
            }
        }
        self.rebind();
    }

    /// Which germ field is current (`Engine::done` % 2), each frame.
    pub fn set_alarm_side(&mut self, side: u32) {
        self.alarm_side = side & 1;
    }

    fn clear_charge(&mut self) {
        if self.live.get() != 0 {
            (self.charge, self.heat) = (storage(&self.device, "charge", 16), storage(&self.device, "crystal heat", 16));
            self.live.set(0);
        }
    }

    /// Shows which walkers have condensed (`BigParams::condense`, 11-14):
    /// the engine's `news` and `state` (as `set_news` takes them; bind them
    /// here too when the world does not wither) and the word each walker's
    /// condensing word starts at (`gpu::Engine::dense_at` / 4 +
    /// `gpu::DENSE_HEAD`). A condensed walker counts twice in the body (it
    /// is two walkers' worth) and the mould round it takes the condensed
    /// colour by the share of its walkers condensed; a ripening walker warms
    /// toward it by its ripeness over `ripe_at` (the `condense` steps), and
    /// a fresh one's speck flares. Call after `set_news`. **Since 11-17
    /// nothing condenses** (`condense` and `thicken` were deleted): the
    /// game passes `ripe_at` 0 and no word sets bit 16 (`DENSE_BIT`) or 17
    /// (`ABSORB_BIT`), so the bronze and the absorbed streaks never draw;
    /// the word is kept for the lumps. Their shader code is the slime
    /// look's to remove; don't reuse bits 16-17 before it goes.
    pub fn set_dense(&mut self, news: Option<(&wgpu::Buffer, &wgpu::Buffer)>, at: u32, ripe_at: u32) {
        match news {
            Some((n, s)) => {
                if !self.news_on {
                    (self.news, self.state) = (n.clone(), s.clone());
                }
                (self.dense_at, self.ripe_at) = (at, ripe_at);
            }
            None => self.dense_at = 0,
        }
        self.rebind();
    }

    /// The mini-bosses' bodies this frame (`mould::big` lumps; 2026-11-16):
    /// each lump's `LumpBody` (centre, look, strength, look left, fill), in
    /// lump order (lump `k` is the walkers whose dense word holds `k + 1`
    /// in bits 25-27). A lump's members are counted on their own a fine
    /// cell (`draw_specks`), and where they are the share of the mould
    /// round a cell, the body takes the nearest lump's look: it stands up,
    /// glosses and lights from inside where its members pack (`prep`), so
    /// the look is the members themselves. Needs `set_dense` (the walkers'
    /// dense words). An empty slice draws every lump as ordinary slime.
    pub fn set_lumps(&mut self, lumps: &[LumpBody]) {
        self.lumps = [[0.0; 4]; MAX_LUMPS];
        (self.lump_fade, self.lump_fill, self.lump_held) = ([1.0; MAX_LUMPS], [0.0; MAX_LUMPS], [0.0; MAX_LUMPS]);
        for (k, l) in lumps.iter().take(MAX_LUMPS).enumerate() {
            self.lumps[k] = [l.at.0, l.at.1, l.look as u32 as f32, l.strength.clamp(0.0, 1.0)];
            self.lump_fade[k] = l.remain.clamp(0.0, 1.0);
            self.lump_fill[k] = l.fill.clamp(0.0, 1.0);
            self.lump_held[k] = l.held.clamp(0.0, 1.0);
        }
    }

    /// The game's time in seconds, each frame (the crystals' `drain`
    /// measures how fast charge goes in game time, so a paused or fast game
    /// reads the same).
    pub fn set_time(&mut self, t: f32) {
        self.game_t = t;
    }

    fn clear_news(&mut self) {
        if self.news_on || self.dense_at > 0 || self.plant_at > 0 || self.crust_at > 0 {
            (self.news, self.state) = (storage(&self.device, "news", 16), storage(&self.device, "state", 16));
            self.news_on = false;
            self.dense_at = 0;
            self.plant_at = 0;
            self.crust_at = 0;
            self.setc_at = 0;
            self.steer_at = 0;
        }
    }

    /// Draws the plants (`mould::plants`, 11-14) from the engine's fields:
    /// `news` the engine's (`set_news` takes it too; `state` with it) and
    /// `at` the word where its plant blocks start (`gpu::Engine::plant_at`
    /// / 4 + `gpu::PLANT_HEAD`): the plants as a ground layer between the
    /// soil and the slime, fire's heat as glowing flame, the char as burnt
    /// ground. `None` hides them.
    /// `flora` is the map's (`map::Items::flora`): which forms its plants
    /// take up close.
    pub fn set_plants(&mut self, news: Option<(&wgpu::Buffer, &wgpu::Buffer)>, at: u32, flora: u32) {
        self.flora = flora;
        match news {
            Some((n, s)) => {
                if !self.news_on && self.dense_at == 0 {
                    (self.news, self.state) = (n.clone(), s.clone());
                }
                self.plant_at = at;
            }
            None => self.plant_at = 0,
        }
        self.rebind();
    }

    /// Draws the crust (`BigDish::crust`, the simulation-use plan's E1a,
    /// 2026-11-17) from the engine's `news` (`state` with it) from word `at`
    /// (`gpu::Engine::crust_at` / 4): satin patches on bare ground whose
    /// share of the ground is the crust, crisp at any zoom, under the
    /// slime as the soil's detail is. `None` hides it.
    pub fn set_crust(&mut self, news: Option<(&wgpu::Buffer, &wgpu::Buffer)>, at: u32) {
        match news {
            Some((n, s)) if at > 0 => {
                if !self.news_on && self.dense_at == 0 && self.plant_at == 0 {
                    (self.news, self.state) = (n.clone(), s.clone());
                }
                self.crust_at = at;
            }
            _ => self.crust_at = 0,
        }
        self.rebind();
    }

    /// Draws each walker under a Mind status (blind, sour, spin, provoke,
    /// still: the gun cards' Mind family) in the family's rose, from the
    /// engine's `news` word `at` (`gpu::Engine::gas_at` / 4 + `GAS_SLOTS` +
    /// walkers: a slot's kind in the top three bits, the step it ends in
    /// the low 29), fading over its last `fade` engine steps; `None` or
    /// `at` 0 hides it.
    pub fn set_minds(&mut self, news: Option<(&wgpu::Buffer, &wgpu::Buffer)>, at: u32, fade: u32) {
        match news {
            Some((n, s)) if at > 0 => {
                if !self.news_on && self.dense_at == 0 && self.plant_at == 0 && self.crust_at == 0 {
                    (self.news, self.state) = (n.clone(), s.clone());
                }
                (self.mind_at, self.mind_fade) = (at, fade.max(1));
            }
            _ => self.mind_at = 0,
        }
        self.rebind();
    }

    /// Gives the slime its own look (Look B, 2026-11-19; the owner, 00:08Z:
    /// several looks, each for a slime). `streak` is how far a walker goes
    /// in `STREAK_SECS`, cells: its pace, which `SlimeLook::Threads` draws
    /// behind it.
    pub fn set_slime_look(&mut self, look: SlimeLook, streak: f32) {
        self.slime_look = look;
        self.streak = streak.max(0.0);
    }

    /// The slime's look as last set (`set_slime_look`).
    pub fn slime_look(&self) -> SlimeLook {
        self.slime_look
    }

    /// **The view's shader again, from `source`** (the slime viewer,
    /// 2026-11-20: `tweaks::apply`'s copy of `SHADER` with the owner's
    /// look numbers in it). The bindings are the same, so only the
    /// pipelines are made again; the old ones stay if the source does not
    /// compile, and the error says why.
    pub fn reshade(&mut self, source: &str) -> Result<(), String> {
        let scope = self.device.push_error_scope(wgpu::ErrorFilter::Validation);
        let module = self.device.create_shader_module(wgpu::ShaderModuleDescriptor { label: Some("view"), source: wgpu::ShaderSource::Wgsl(source.into()) });
        let layout = self.device.create_pipeline_layout(&wgpu::PipelineLayoutDescriptor { label: Some("view"), bind_group_layouts: &[Some(&self.bgl)], immediate_size: 0 });
        let pipe = |entry_point: &str| {
            self.device.create_compute_pipeline(&wgpu::ComputePipelineDescriptor { label: Some(entry_point), layout: Some(&layout), module: &module, entry_point: Some(entry_point), compilation_options: Default::default(), cache: None })
        };
        let pipes = [pipe("base"), pipe("draw_marks"), pipe("compose"), pipe("draw_specks"), pipe("prep"), pipe("drain")];
        if let Some(e) = crate::gpu::block_on(scope.pop()) {
            return Err(e.to_string());
        }
        self.pipes = pipes;
        Ok(())
    }

    /// Draws set slime (the simulation-use plan's E2b, `BigDish::set_cells`)
    /// from the engine's `news` word `at` (`gpu::Engine::crust_at` / 4 +
    /// cells + walkers), bound with the crust (`set_crust` first), as amber
    /// resin; `at` 0 hides it. Off while the crust is. (To 2026-11-18 it
    /// took a look, 1 amber, 2 a pale glaze, 0 hidden, for a blind A/B;
    /// amber is the game's and the other two went: `view.wgsl`, `set_mat`.)
    pub fn set_setting(&mut self, at: u32) {
        self.setc_at = if self.crust_at > 0 { at } else { 0 };
    }

    /// Draws the Lure's steering field (2026-11-18) from the engine's
    /// `news` from word `at`: a word a 2 x 2 block of cells, row by row
    /// (`ceil(w / 2)` blocks a row), the walking distance to the nearest
    /// lure in cells + 1, and 0 where the field does not reach. It shows as
    /// cream rings of equal distance, every 20 cells, on the ground the
    /// field covers (`view.wgsl`, `steer_px`), and while it shows the
    /// Lure's scent light is not lit. `at` 0 hides it (no lure standing).
    /// Needs the engine's `news` bound (`set_news`, `set_dense`,
    /// `set_plants` or `set_crust` first, each frame's order in the game);
    /// without it nothing is drawn.
    pub fn set_steer(&mut self, at: u32) {
        self.steer_at = if self.news_on || self.dense_at > 0 || self.plant_at > 0 || self.crust_at > 0 { at } else { 0 };
    }

    /// How far the Lure's field reaches, in its own cells (the game's
    /// `Rules::lure_reach` times the world's scale; 2026-11-18). No ring
    /// is drawn within half a ring of it: the field stops there on its
    /// blocks' stepped edge, and a ring on it drew as a dotted line round
    /// the pull's rim (the game's reach, 120 cells, is a multiple of the
    /// rings' 20). 0, the default, draws every ring.
    pub fn set_steer_reach(&mut self, cells: f32) {
        self.steer_reach = cells.max(0.0);
    }

    /// The world's living walkers a cell (living walkers over its cells),
    /// which sets how many walkers near a cell make the look's body thick
    /// (`view.wgsl`, `body`). Leave it at 0 for the calibration's 5000 on
    /// 512 x 320. The slot count would not do: a growing world has ten
    /// slots a walker.
    pub fn set_body_density(&mut self, walkers_per_cell: f32) {
        self.body_density = walkers_per_cell.max(0.0);
    }

    /// Back to drawing the world frame as it is (the shading off).
    pub fn clear_look(&mut self) {
        if self.shade {
            (self.look, self.ground, self.walkers) = (storage(&self.device, "look", 16), storage(&self.device, "ground", 16), storage(&self.device, "walkers", 16));
            (self.n_walk, self.shade) = (0, false);
            self.rebind();
        }
    }

    pub fn shading(&self) -> bool {
        self.shade
    }

    /// A new screen size: the overlay is cleared.
    pub fn resize(&mut self, screen: (u32, u32)) {
        let (stride, out, overlay, top) = Self::screen_buffers(&self.device, screen);
        (self.stride, self.out, self.overlay, self.top, self.screen) = (stride, out, overlay, top, screen);
        self.specks = storage(&self.device, "specks", stride as u64 * screen.1 as u64 * 12);
        (self.hcell, self.mcell, self.tcell) = Self::cell_buffers(&self.device, screen);
        self.fgrid = Self::fine_buffer(&self.device, screen);
        self.has_overlay = false;
        self.rebind();
    }

    /// The overlay: premultiplied RGBA, screen-sized (`screen.0 x screen.1 x 4`
    /// bytes), or `None` for none. Write it only when it changes.
    pub fn set_overlay(&mut self, rgba: Option<&[u8]>) {
        match rgba {
            Some(b) => {
                assert_eq!(b.len(), self.screen.0 as usize * self.screen.1 as usize * 4, "a screen-sized overlay");
                self.queue.write_buffer(&self.overlay, 0, b);
                self.has_overlay = true;
            }
            None => self.has_overlay = false,
        }
    }

    /// The fog-of-war hook: a world-sized 0-1 visibility field the view
    /// darkens by (0 shows the world at 30%), or `None`. Not used yet: the
    /// owner has not decided on fog of war.
    pub fn set_visibility(&mut self, vis: Option<&[f32]>) {
        match vis {
            Some(v) => {
                assert_eq!(v.len(), self.world.0 * self.world.1, "a world-sized field");
                let bytes: Vec<u8> = v.iter().flat_map(|x| x.to_le_bytes()).collect();
                if self.vis.size() != bytes.len().max(16) as u64 {
                    self.vis = storage(&self.device, "visibility", bytes.len() as u64);
                    self.rebind();
                }
                self.queue.write_buffer(&self.vis, 0, &bytes);
                self.has_vis = true;
            }
            None => {
                if self.vis.size() != 16 {
                    self.vis = storage(&self.device, "visibility", 16);
                    self.rebind();
                }
                self.has_vis = false;
            }
        }
    }

    /// This frame's point lights, `pick_lights` of them for the world
    /// rectangle `seen` (x0, y0, x1, y1, cells): they light the shaded world
    /// (`view.wgsl`, `shaded`), so only with the look on.
    pub fn set_lights(&mut self, lights: &[Light], seen: (f32, f32, f32, f32)) {
        let lights = pick_lights(lights, seen);
        let n = lights.len();
        if n > 0 {
            let words: Vec<f32> = lights.iter().flat_map(|l| [l.at.0, l.at.1, l.reach, l.lift, l.colour[0], l.colour[1], l.colour[2], l.strength]).collect();
            let bytes: Vec<u8> = words.iter().flat_map(|x| x.to_le_bytes()).collect();
            self.queue.write_buffer(&self.lights, 0, &bytes);
        }
        self.n_lights = n as u32;
    }

    /// This frame's marks.
    pub fn set_marks(&mut self, marks: &Marks) {
        let words = marks.words();
        if !words.is_empty() {
            let bytes: Vec<u8> = words.iter().flat_map(|x| x.to_le_bytes()).collect();
            self.queue.write_buffer(&self.marks, 0, &bytes);
        }
        self.n_marks = marks.len() as u32;
    }

    /// The minimap's rectangle this frame, if drawn.
    pub fn minimap_rect(&self, cam: &Camera) -> Option<(u32, u32, u32, u32)> {
        if !self.minimap || self.minimap_off {
            return None;
        }
        // A HUD that frames the minimap shows it at every zoom (Larder
        // World's bar, 11-14 20:19Z); unframed (mouldgpu) it hides once the
        // whole world is on screen, as before.
        match self.minimap_box {
            Some(b) => Some(minimap_in(self.world, b)),
            None if cam.sees_all() => None,
            None => minimap_rect(self.world, self.screen),
        }
    }

    /// Records the passes for camera `cam` (after the engine's draw).
    /// With `stamp`, timestamps at the first pass's start and the last
    /// pass's end, at the indices given.
    pub fn record(&self, enc: &mut wgpu::CommandEncoder, cam: &Camera, stamp: Option<(&wgpu::QuerySet, Option<u32>, Option<u32>)>) {
        self.record_with(enc, cam, stamp, None);
    }

    /// The names of the view's passes, as `record_each` stamps them.
    pub const PASS_NAMES: [&'static str; 6] = ["base", "marks", "compose", "specks", "prep", "drain"];

    /// As `record`, with each pass stamped alone in `qs` (begin at 2 q, end
    /// at 2 q + 1, q the pass's place in this frame); returns each pass's
    /// index in `PASS_NAMES`, in order. (The bench's `viewpasses=`.)
    pub fn record_each(&self, enc: &mut wgpu::CommandEncoder, cam: &Camera, qs: &wgpu::QuerySet) -> Vec<usize> {
        self.record_with(enc, cam, None, Some(qs))
    }

    fn record_with(&self, enc: &mut wgpu::CommandEncoder, cam: &Camera, stamp: Option<(&wgpu::QuerySet, Option<u32>, Option<u32>)>, each: Option<&wgpu::QuerySet>) -> Vec<usize> {
        let mm = self.minimap_rect(cam).unwrap_or((0, 0, 0, 0));
        let mut b: Vec<u8> = Vec::with_capacity(PARAMS_BYTES);
        for v in [self.screen.0, self.screen.1, self.stride, self.world.0 as u32, self.world.1 as u32, self.n_marks, self.has_vis as u32, self.has_overlay as u32] {
            b.extend(v.to_le_bytes());
        }
        for v in [cam.x, cam.y, cam.zoom, f32::from_bits(self.n_lights)] {
            b.extend(v.to_le_bytes());
        }
        for v in [mm.0, mm.1, mm.2, mm.3, self.shade as u32, self.n_walk, self.lives as u32, self.body_density.to_bits()] {
            b.extend(v.to_le_bytes());
        }
        // The shown cells and a margin: what `prep` works out.
        let (ox, oy) = (cam.x.floor() as i32 - MARGIN as i32, cam.y.floor() as i32 - MARGIN as i32);
        let zoom = cam.zoom.max(1.0);
        let k = Self::fine(zoom);
        let (rw, rh) = (((self.screen.0 as f32 / zoom).ceil() as u32 + 2 * MARGIN + 2) * k, ((self.screen.1 as f32 / zoom).ceil() as u32 + 2 * MARGIN + 2) * k);
        b.extend(ox.to_le_bytes());
        b.extend(oy.to_le_bytes());
        b.extend(rw.to_le_bytes());
        b.extend(rh.to_le_bytes());
        // The game seconds since the last frame, for `drain` (0 on a frame
        // with no game time: the heat holds).
        let live = self.live.get();
        let dt = if live == 1 { (self.game_t - self.last_t.get()).max(0.0) } else { 0.0 };
        self.last_t.set(self.game_t);
        for v in [self.news_on as u32, self.lag, self.lean, self.n_gems, k, self.n_list * (live > 0) as u32, dt.to_bits(), live] {
            b.extend(v.to_le_bytes());
        }
        for v in [self.dense_at, self.ripe_at, Self::fine_half(self.screen), self.stride * self.screen.1] {
            b.extend(v.to_le_bytes());
        }
        for v in [self.alarm_side, self.alarm_full.to_bits(), self.plant_at, self.flora] {
            b.extend(v.to_le_bytes());
        }
        let n_lumps = self.lumps.iter().filter(|l| l[2] > 0.0).count() as u32;
        // The Mind words (the alarm haze's strengths to 2026-11-17).
        for v in [self.mind_at, self.mind_fade, self.scent as u32, n_lumps] {
            b.extend(v.to_le_bytes());
        }
        for l in self.lumps {
            for v in l {
                b.extend(v.to_le_bytes());
            }
        }
        for v in self.lump_fade.into_iter().chain(self.lump_fill).chain(self.lump_held) {
            b.extend(v.to_le_bytes());
        }
        // The third word was set slime's look to 2026-11-18; since, the
        // Lure's reach.
        for v in [self.crust_at, self.setc_at, self.steer_reach.to_bits(), self.steer_at] {
            b.extend(v.to_le_bytes());
        }
        let streak = if self.slime_look == SlimeLook::Threads { self.streak } else { 0.0 };
        for v in [self.slime_look as u32, streak.to_bits(), self.life_on as u32, self.scar_on as u32] {
            b.extend(v.to_le_bytes());
        }
        debug_assert_eq!(b.len(), PARAMS_BYTES);
        if live == 2 {
            self.live.set(1);
        }
        self.queue.write_buffer(&self.params, 0, &b);
        let (gx, gy) = (self.screen.0.div_ceil(16), self.screen.1.div_ceil(16));
        let mut passes: Vec<(usize, (u32, u32))> = Vec::new();
        if self.shade && cam.zoom >= 1.0 {
            enc.clear_buffer(&self.fgrid, 0, None);
        }
        if self.shade && self.n_walk > 0 && cam.zoom >= 1.0 {
            passes.push((3, (self.n_walk.div_ceil(256), 1)));
        }
        if self.shade && cam.zoom >= 1.0 {
            passes.push((4, (rw.div_ceil(16), rh.div_ceil(16))));
        }
        if live > 0 && self.n_list > 0 {
            passes.push((5, (self.n_list.div_ceil(256), 1)));
        }
        passes.push((0, (gx, gy)));
        if self.n_marks > 0 {
            // `SPLIT` workgroups a mark (view.wgsl's draw_marks).
            passes.push((1, (self.n_marks, 16)));
        }
        if self.has_overlay || self.n_marks > 0 {
            passes.push((2, (gx, gy)));
        }
        let last = passes.len() - 1;
        for (q, &(p, (x, y))) in passes.iter().enumerate() {
            let writes = match each {
                Some(qs) => Some(wgpu::ComputePassTimestampWrites { query_set: qs, beginning_of_pass_write_index: Some(2 * q as u32), end_of_pass_write_index: Some(2 * q as u32 + 1) }),
                None => stamp
                    .map(|(qs, begin, end)| wgpu::ComputePassTimestampWrites { query_set: qs, beginning_of_pass_write_index: begin.filter(|_| q == 0), end_of_pass_write_index: end.filter(|_| q == last) })
                    .filter(|w| w.beginning_of_pass_write_index.is_some() || w.end_of_pass_write_index.is_some()),
            };
            let mut pass = enc.begin_compute_pass(&wgpu::ComputePassDescriptor { label: Some("view"), timestamp_writes: writes });
            pass.set_pipeline(&self.pipes[p]);
            pass.set_bind_group(0, &self.group, &[]);
            pass.dispatch_workgroups(x, y, 1);
        }
        passes.into_iter().map(|p| p.0).collect()
    }

    /// Copies the result into a texture the screen's size (the window's).
    pub fn copy_to(&self, enc: &mut wgpu::CommandEncoder, texture: &wgpu::Texture) {
        enc.copy_buffer_to_texture(
            wgpu::TexelCopyBufferInfo { buffer: &self.out, layout: wgpu::TexelCopyBufferLayout { offset: 0, bytes_per_row: Some(self.stride * 4), rows_per_image: Some(self.screen.1) } },
            wgpu::TexelCopyTextureInfo { texture, mip_level: 0, origin: wgpu::Origin3d::ZERO, aspect: wgpu::TextureAspect::All },
            wgpu::Extent3d { width: self.screen.0, height: self.screen.1, depth_or_array_layers: 1 },
        );
    }

    /// The result as an image (for screenshots and tests; waits for the GPU).
    pub fn picture(&self) -> image::RgbaImage {
        let bytes = crate::gpu::read_back(&self.device, &self.queue, &self.out);
        let (w, h, stride) = (self.screen.0 as usize, self.screen.1 as usize, self.stride as usize);
        let mut img = Vec::with_capacity(w * h * 4);
        for y in 0..h {
            img.extend_from_slice(&bytes[y * stride * 4..(y * stride + w) * 4]);
        }
        image::RgbaImage::from_raw(w as u32, h as u32, img).expect("screen size")
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    /// Off-screen lights are dropped, and past the cap the brightest are
    /// kept in their order: a muzzle light pushed last still lands when
    /// 80 dim hit lights came first.
    #[test]
    fn the_lights_kept_are_on_screen_and_the_brightest() {
        let at = |x: f32, s: f32| Light { at: (x, 10.0), reach: 2.0, lift: 3.0, colour: [255.0; 3], strength: s };
        let seen = (0.0, 0.0, 100.0, 50.0);
        // Off the right edge by more than its reach, and just touching it.
        assert!(pick_lights(&[at(103.0, 1.0)], seen).is_empty());
        assert_eq!(pick_lights(&[at(101.5, 1.0)], seen).len(), 1);
        let mut many: Vec<Light> = (0..80).map(|k| at(k as f32, 0.1)).collect();
        many.push(at(50.0, 0.6));
        let kept = pick_lights(&many, seen);
        // The bright one pushed last, and the first 63 of the equal dim
        // ones, in the order given.
        let mut want = many[..MAX_LIGHTS - 1].to_vec();
        want.push(at(50.0, 0.6));
        assert_eq!(kept, want);
        // Under the cap, everything on screen, as given.
        let few: Vec<Light> = (0..10).map(|k| at(k as f32, 0.1)).collect();
        assert_eq!(pick_lights(&few, seen), few);
    }
    use crate::world::marks::{Mark, Shape};

    /// A GPU for the view's tests, as the sandbox's: skipped with none,
    /// unless `MOULDGPU_TEST_GPU=1` (CI, on software Vulkan).
    fn test_gpu() -> Option<(wgpu::Device, wgpu::Queue)> {
        let required = std::env::var("MOULDGPU_TEST_GPU").is_ok_and(|v| v == "1");
        let instance = wgpu::Instance::new(wgpu::InstanceDescriptor::new_without_display_handle_from_env());
        let adapter = match crate::gpu::block_on(instance.request_adapter(&wgpu::RequestAdapterOptions::default())) {
            Ok(a) => a,
            Err(e) => {
                assert!(!required, "MOULDGPU_TEST_GPU=1 but no GPU adapter: {e}");
                eprintln!("no GPU adapter ({e}): skipped");
                return None;
            }
        };
        let desc = wgpu::DeviceDescriptor { label: Some("view"), required_limits: adapter.limits(), ..Default::default() };
        Some(crate::gpu::block_on(adapter.request_device(&desc)).expect("device"))
    }

    /// A world whose every cell's colour says where it is: red x, green y.
    fn coded(dev: &wgpu::Device, q: &wgpu::Queue, w: usize, h: usize) -> wgpu::Buffer {
        let px: Vec<u8> = (0..w * h).flat_map(|i| [(i % w) as u8, (i / w) as u8, 7, 255]).collect();
        let b = storage(dev, "world", px.len() as u64);
        q.write_buffer(&b, 0, &px);
        b
    }

    #[test]
    fn each_screen_pixel_shows_the_cell_the_camera_puts_there() {
        let Some((dev, q)) = test_gpu() else { return };
        let (w, h) = (200, 120);
        let world = coded(&dev, &q, w, h);
        // 100 wide: not a multiple of 64, so the stride is in play.
        let screen = (100, 60);
        let view = View::new(&dev, &q, &world, (w, h), screen);
        for zoom in [1.0, 3.0] {
            let mut cam = Camera::new((w, h), screen);
            cam.zoom_to(zoom, (0.0, 0.0));
            cam.centre_on((130.0, 70.0));
            let mut enc = dev.create_command_encoder(&Default::default());
            view.record(&mut enc, &cam, None);
            q.submit([enc.finish()]);
            let pic = view.picture();
            for (sx, sy) in [(0, 0), (37, 21), (99, 59), (50, 30)] {
                let (wx, wy) = cam.to_world((sx as f32 + 0.5, sy as f32 + 0.5));
                let p = pic.get_pixel(sx, sy).0;
                assert_eq!((p[0], p[1]), (wx as u8, wy as u8), "zoom {zoom} screen ({sx}, {sy})");
            }
        }
    }

    #[test]
    fn zoomed_out_the_minimap_and_marks_show_and_the_overlay_covers_them() {
        let Some((dev, q)) = test_gpu() else { return };
        let (w, h) = (400, 250);
        let world = coded(&dev, &q, w, h);
        let screen = (320, 200);
        let mut view = View::new(&dev, &q, &world, (w, h), screen);
        let cam = Camera::new((w, h), screen);
        assert!(!cam.sees_all());
        // A white disc at world (cam middle), and a 2-pixel overlay square at (0, 0).
        let mut marks = Marks::default();
        marks.disc(cam.centre(), 3.0, [255, 255, 255, 255], true);
        marks.push(Mark { flags: 0, ..Mark::new(Shape::Rect, cam.to_world((300.0, 10.0)), (2.0, 2.0), [0, 0, 255, 255]) });
        view.set_marks(&marks);
        let mut ov = vec![0u8; 320 * 200 * 4];
        ov[..4].copy_from_slice(&[9, 9, 9, 255]);
        view.set_overlay(Some(&ov));
        let mut enc = dev.create_command_encoder(&Default::default());
        view.record(&mut enc, &cam, None);
        q.submit([enc.finish()]);
        let pic = view.picture();
        assert_eq!(pic.get_pixel(160, 100).0, [255, 255, 255, 255], "the disc at the middle");
        assert_eq!(pic.get_pixel(300, 10).0, [0, 0, 255, 255], "the rect");
        assert_eq!(pic.get_pixel(0, 0).0, [9, 9, 9, 255], "the overlay on top");
        let (mx, my, mw, mh) = view.minimap_rect(&cam).expect("a minimap");
        assert_eq!((mw as f32 / mh as f32 * 10.0).round(), 16.0, "the world's shape");
        // A minimap pixel near its top-left shows the world's top-left.
        let p = pic.get_pixel(mx + 1, my + 1).0;
        assert!(p[0] < 12 && p[1] < 12, "{p:?}");
        // The disc's dot is on the minimap where its world point is.
        let s = mw as f32 / w as f32;
        let c = cam.centre();
        let dot = pic.get_pixel(mx + (c.0 * s) as u32, my + (c.1 * s) as u32).0;
        assert_eq!(dot, [255, 255, 255, 255]);
    }

    /// Plan C8 (2026-11-14): a walker's news of food is on screen. A calm
    /// piece keeps its colour; a piece whose walkers are cut off (news older
    /// than the lag) greys, and its specks show the phase, amber while one
    /// leans and red once it bolts.
    #[test]
    fn cut_off_walkers_show_their_phase_and_their_piece_greys() {
        let Some((dev, q)) = test_gpu() else { return };
        let (w, h) = (64usize, 40usize);
        let px: Vec<u8> = (0..w * h).flat_map(|_| [200u8, 160, 60, 255]).collect();
        let world = storage(&dev, "world", px.len() as u64);
        q.write_buffer(&world, 0, &px);
        // Two pieces of 25 walkers, a walker a cell: A (16, 20) calm, B
        // (48, 20) cut off long ago; one leaning walker alone at (32, 10).
        let (now, lag, lean) = (1000u32, 150u32, 45u32);
        let (mut look, mut walk, mut news) = (vec![0u32; w * h], Vec::new(), Vec::new());
        for (cx, heard) in [(16usize, now), (48, now + 1 - 300)] {
            for dy in 0..5 {
                for dx in 0..5 {
                    let (x, y) = (cx + dx - 2, 20 + dy - 2);
                    look[y * w + x] = 255 | (1 << 8);
                    walk.extend([x as f32 + 0.5, y as f32 + 0.5, 0.0, 0.0]);
                    news.push(heard);
                }
            }
        }
        walk.extend([32.5f32, 10.5, 0.0, 0.0]);
        news.push(now + 1 - 170);
        let buf = |label: &str, bytes: &[u8]| {
            let b = storage(&dev, label, bytes.len() as u64);
            q.write_buffer(&b, 0, bytes);
            b
        };
        let look_b = buf("look", &look.iter().flat_map(|v| v.to_le_bytes()).collect::<Vec<u8>>());
        let ground = buf("ground", &vec![0u8; w * h * 4]);
        let walk_b = buf("walkers", &walk.iter().flat_map(|v| v.to_le_bytes()).collect::<Vec<u8>>());
        let news_b = buf("news", &news.iter().flat_map(|v| v.to_le_bytes()).collect::<Vec<u8>>());
        let state = buf("state", &[now, 0, 0, 0].iter().flat_map(|v| v.to_le_bytes()).collect::<Vec<u8>>());
        let screen = (384, 240);
        let mut view = View::new(&dev, &q, &world, (w, h), screen);
        view.set_look(&look_b, &ground, &walk_b, news.len() as u32);
        let mut cam = Camera::new((w, h), screen);
        cam.zoom_to(6.0, (0.0, 0.0));
        cam.centre_on((32.0, 20.0));
        let shot = |view: &View| {
            let mut enc = dev.create_command_encoder(&Default::default());
            view.record(&mut enc, &cam, None);
            q.submit([enc.finish()]);
            view.picture()
        };
        let at = |pic: &image::RgbaImage, wx: f32, wy: f32| {
            let (sx, sy) = cam.to_screen((wx, wy));
            pic.get_pixel(sx as u32, sy as u32).0
        };
        // Between specks: a cell's corner.
        let sat = |p: [u8; 4]| p[0].max(p[1]).max(p[2]) as i32 - p[0].min(p[1]).min(p[2]) as i32;
        let off = shot(&view);
        view.set_news(Some((&news_b, &state)), lag, lean);
        let on = shot(&view);
        let (a_off, a_on) = (at(&off, 16.1, 20.1), at(&on, 16.1, 20.1));
        let (b_off, b_on) = (at(&off, 48.1, 20.1), at(&on, 48.1, 20.1));
        assert!((sat(a_on) - sat(a_off)).abs() <= 2, "the calm piece keeps its colour: {a_off:?} {a_on:?}");
        assert!(sat(b_on) + 30 < sat(b_off), "the cut-off piece greys: {b_off:?} {b_on:?}");
        let bolt = at(&on, 48.5, 20.5);
        assert!(bolt[0] as i32 > bolt[1] as i32 + 100, "a bolting walker is red: {bolt:?}");
        let leaning = at(&on, 32.5, 10.5);
        assert!(leaning[0] > 200 && leaning[1] > 120 && (leaning[2] as i32) < leaning[1] as i32 - 60, "a leaning walker is amber: {leaning:?}");
        let calm = at(&on, 16.5, 20.5);
        assert!((calm[0] as i32) < calm[1] as i32 + 100, "a calm walker is not red: {calm:?}");
    }

    /// Crystals at screen resolution (2026-11-14, the owner's playtest:
    /// "The crystals are pixelated"): a pointy-top hexagon's edge, the
    /// slanted ones too, falls where the hexagon is, to under a third of a
    /// cell, not on the cells' staircase.
    #[test]
    fn the_scent_view_washes_scent_and_trail_and_its_lines_are_a_pixel_or_two_at_any_zoom() {
        // The scent view (F2, 11-15): the look's byte 3 (scent) a ramp along
        // x, byte 0 (trail) full in the top rows. On, the scent reads blue,
        // the trail pink, and every contour line across the ramp is one to
        // three pixels wide, close up as well as far (owner: crisp zoomed in).
        let Some((dev, q)) = test_gpu() else { return };
        let (w, h) = (64usize, 40usize);
        let px: Vec<u8> = (0..w * h).flat_map(|_| [12u8, 22, 26, 255]).collect();
        let world = storage(&dev, "world", px.len() as u64);
        q.write_buffer(&world, 0, &px);
        let mut look = vec![0u32; w * h];
        for y in 0..h {
            for x in 0..w {
                let scent = (x * 255 / (w - 1)) as u32;
                let trail = if y < 8 { 255 } else { 0 };
                look[y * w + x] = trail | scent << 24;
            }
        }
        let buf = |label: &str, bytes: &[u8]| {
            let b = storage(&dev, label, bytes.len() as u64);
            q.write_buffer(&b, 0, bytes);
            b
        };
        let look_b = buf("look", &crate::gpu::u32_bytes(&look));
        let ground_b = buf("ground", &vec![0u8; w * h * 4]);
        let walk_b = buf("walkers", &[0u8; 16]);
        let screen = (384, 240);
        let mut view = View::new(&dev, &q, &world, (w, h), screen);
        view.set_look(&look_b, &ground_b, &walk_b, 0);
        for zoom in [6.0f32, 24.0] {
            let mut cam = Camera::new((w, h), screen);
            cam.zoom_to(zoom, (0.0, 0.0));
            let shot = |view: &mut View, cam: &Camera, on: bool| {
                view.scent = on;
                let mut enc = dev.create_command_encoder(&Default::default());
                view.record(&mut enc, cam, None);
                q.submit([enc.finish()]);
                view.picture()
            };
            // Strong scent, no trail: bluer with the view on.
            cam.centre_on((52.0, 30.0));
            let (off, on) = (shot(&mut view, &cam, false), shot(&mut view, &cam, true));
            let (sx, sy) = cam.to_screen((52.3, 30.0));
            let (a, b) = (
                off.get_pixel(sx as u32, sy as u32).0,
                on.get_pixel(sx as u32, sy as u32).0,
            );
            assert!(
                b[2] as i32 > b[0] as i32 + 60 && b[2] > a[2],
                "zoom {zoom}: scent blue {a:?} -> {b:?}"
            );
            // Trail, no scent: pink.
            cam.centre_on((4.0, 3.0));
            let on = shot(&mut view, &cam, true);
            let (sx, sy) = cam.to_screen((4.0, 3.0));
            let p = on.get_pixel(sx as u32, sy as u32).0;
            assert!(p[0] as i32 > p[1] as i32 + 60, "zoom {zoom}: trail pink {p:?}");
            // The lines across the ramp, along a row: each run of near-white
            // pixels is 1 to 3 wide, and there are some.
            cam.centre_on((32.0, 24.0));
            let on = shot(&mut view, &cam, true);
            let (mut runs, mut run) = (Vec::new(), 0);
            for x in 0..screen.0 {
                let p = on.get_pixel(x, 120).0;
                if p[0] > 150 && p[1] > 170 && p[2] > 200 {
                    run += 1
                } else if run > 0 {
                    runs.push(run);
                    run = 0
                }
            }
            assert!(!runs.is_empty(), "zoom {zoom}: contour lines drawn");
            assert!(
                runs.iter().all(|&r| (1..=3).contains(&r)),
                "zoom {zoom}: line widths {runs:?}"
            );
        }
    }

    #[test]
    fn crystals_are_their_hexagons_to_the_pixel() {
        let Some((dev, q)) = test_gpu() else { return };
        let (w, h) = (64usize, 40usize);
        let px: Vec<u8> = (0..w * h).flat_map(|_| [12u8, 22, 26, 255]).collect();
        let world = storage(&dev, "world", px.len() as u64);
        q.write_buffer(&world, 0, &px);
        let (c, r) = ((32.0f32, 20.0f32), 7.0f32);
        let a = r * 3f32.sqrt() / 2.0;
        let inside = |x: f32, y: f32| {
            let (dx, dy) = ((x - c.0).abs(), (y - c.1).abs());
            a - dx.max(0.5 * dx + 0.8660254 * dy)
        };
        let mut pile = vec![0u16; w * h];
        let mut ground = vec![0u32; w * h];
        for y in 0..h {
            for x in 0..w {
                if inside(x as f32 + 0.5, y as f32 + 0.5) >= 0.0 {
                    pile[y * w + x] = 1;
                    ground[y * w + x] = 5 | 255 << 8 | 128 << 16;
                }
            }
        }
        let buf = |label: &str, bytes: &[u8]| {
            let b = storage(&dev, label, bytes.len() as u64);
            q.write_buffer(&b, 0, bytes);
            b
        };
        let look_b = buf("look", &vec![0u8; w * h * 4]);
        let ground_b = buf("ground", &crate::gpu::u32_bytes(&ground));
        let walk_b = buf("walkers", &[0u8; 16]);
        let screen = (384, 240);
        let mut view = View::new(&dev, &q, &world, (w, h), screen);
        view.set_look(&look_b, &ground_b, &walk_b, 0);
        view.set_crystals(&[(c, r)], &pile);
        let mut cam = Camera::new((w, h), screen);
        cam.zoom_to(6.0, (0.0, 0.0));
        cam.centre_on(c);
        let mut enc = dev.create_command_encoder(&Default::default());
        view.record(&mut enc, &cam, None);
        q.submit([enc.finish()]);
        let pic = view.picture();
        let cyan = |x: f32, y: f32| {
            let (sx, sy) = cam.to_screen((x, y));
            let p = pic.get_pixel(sx as u32, sy as u32).0;
            p[2] as i32 > p[0] as i32 + 10
        };
        // Across a slanted edge at many heights, and the vertical one: a
        // third of a cell in is crystal, a third out is not.
        let mut checked = 0;
        for i in 0..40 {
            let t = i as f32 / 40.0 * std::f32::consts::TAU;
            let (ux, uy) = (t.cos(), t.sin());
            // The edge along this ray from the centre: where `inside` is 0.
            let (mut lo, mut hi) = (0.0f32, r * 1.2);
            for _ in 0..40 {
                let m = 0.5 * (lo + hi);
                if inside(c.0 + ux * m, c.1 + uy * m) > 0.0 { lo = m } else { hi = m }
            }
            let at = |d: f32| (c.0 + ux * (lo + d), c.1 + uy * (lo + d));
            let (xi, yi) = at(-0.33);
            let (xo, yo) = at(0.33);
            assert!(cyan(xi, yi), "inside the edge at angle {t:.2}: ({xi:.2}, {yi:.2})");
            assert!(!cyan(xo, yo), "outside the edge at angle {t:.2}: ({xo:.2}, {yo:.2})");
            checked += 1;
        }
        assert_eq!(checked, 40);
    }

    /// The flinch is the slime's own colour (Look A, 2026-11-17): slime
    /// under the engine's alarm (`set_alarm`, the current germ side) flushes
    /// hot, its green and blue falling and its red kept; slime with no
    /// alarm on it keeps its gold, and bare ground under the same alarm
    /// shows nothing. At zoom 6 (two fine cells a cell) and at zoom 2 (one).
    /// The other side, or no alarm bound, shows nothing.
    /// Before, the alarm drew as a pale haze with a bright front on bare
    /// ground and slime alike. Planted wrong it fails both ways (llvmpipe,
    /// zoom 6): with `flush_mat` off the slime stays (255, 197, 78), and
    /// with the flush not kept to the slime the bare ground goes from
    /// (40, 41, 39) to (61, 36, 26).
    #[test]
    fn slime_under_alarm_flushes_hot_and_bare_ground_shows_nothing() {
        let Some((dev, q)) = test_gpu() else { return };
        let (w, h) = (64usize, 40usize);
        // Two pieces of 49 walkers, a walker a cell, gold as the engine
        // draws mould on agar: A round (16, 20), B round (48, 20). A disc
        // of alarm, 10 cells out round B, covers B and the bare ground by it.
        let full = 150.0f32;
        let mut px: Vec<u8> = (0..w * h).flat_map(|_| [12u8, 22, 26, 255]).collect();
        let (mut look, mut walk) = (vec![0u32; w * h], Vec::new());
        for cx in [16usize, 48] {
            for dy in 0..7 {
                for dx in 0..7 {
                    let (x, y) = (cx + dx - 3, 20 + dy - 3);
                    look[y * w + x] = 255 | (1 << 8);
                    px[4 * (y * w + x)..4 * (y * w + x) + 3].copy_from_slice(&[255, 190, 90]);
                    walk.extend([x as f32 + 0.5, y as f32 + 0.5, 0.0, 0.0]);
                }
            }
        }
        let mut germ = vec![0.0f32; 3 * w * h];
        for y in 0..h {
            for x in 0..w {
                if (x as f32 + 0.5 - 48.5).hypot(y as f32 + 0.5 - 20.5) <= 10.0 {
                    germ[w * h + y * w + x] = full;
                }
            }
        }
        let buf = |label: &str, bytes: &[u8]| {
            let b = storage(&dev, label, bytes.len() as u64);
            q.write_buffer(&b, 0, bytes);
            b
        };
        let f32s = |v: &[f32]| v.iter().flat_map(|x| x.to_le_bytes()).collect::<Vec<u8>>();
        let world = buf("world", &px);
        let look_b = buf("look", &crate::gpu::u32_bytes(&look));
        let ground_b = buf("ground", &vec![0u8; w * h * 4]);
        let walk_b = buf("walkers", &f32s(&walk));
        let (live, empty) = (buf("germ live", &f32s(&germ)), buf("germ empty", &f32s(&vec![0.0; 3 * w * h])));
        let screen = (384, 240);
        let mut view = View::new(&dev, &q, &world, (w, h), screen);
        view.set_look(&look_b, &ground_b, &walk_b, walk.len() as u32 / 4);
        for zoom in [6.0f32, 2.0] {
            let mut cam = Camera::new((w, h), screen);
            cam.zoom_to(zoom, (0.0, 0.0));
            cam.centre_on((32.0, 20.0));
            let frame = |view: &View| {
                let mut enc = dev.create_command_encoder(&Default::default());
                view.record(&mut enc, &cam, None);
                q.submit([enc.finish()]);
                view.picture()
            };
            // The mean colour of the 5 x 5 pixels round a world point (so a
            // walker's speck does not decide it).
            let at = |pic: &image::RgbaImage, x: f32, y: f32| {
                let (sx, sy) = cam.to_screen((x, y));
                let mut c = [0i32; 3];
                for dy in -2..=2 {
                    for dx in -2..=2 {
                        let p = pic.get_pixel((sx as i32 + dx) as u32, (sy as i32 + dy) as u32).0;
                        for k in 0..3 {
                            c[k] += p[k] as i32;
                        }
                    }
                }
                c.map(|v| v / 25)
            };
            // A's middle, B's middle, and bare ground in the alarm 7.5 cells
            // under B's middle, 4 cells clear of its last walkers (at 2.5
            // the film's faint tail still flushed, 3 of 255, at zoom 2).
            let spots = |pic: &image::RgbaImage| [at(pic, 16.5, 20.5), at(pic, 48.5, 20.5), at(pic, 48.5, 28.0)];
            view.set_alarm(None, 0.0);
            let [a0, b0, g0] = spots(&frame(&view));
            assert!(b0[1] > b0[2] + 60 && b0[0] > 150, "zoom {zoom}: the slime is gold: {b0:?}");
            view.set_alarm(Some([&empty, &live]), full);
            view.set_alarm_side(1);
            let [a, b, g] = spots(&frame(&view));
            let far = |p: [i32; 3], q: [i32; 3]| (0..3).map(|k| (p[k] - q[k]).abs()).max().unwrap();
            assert!(b[1] + 20 < b0[1] && b[2] < b0[2] && b[0] + 6 >= b0[0], "zoom {zoom}: slime under alarm flushes hot: {b0:?} -> {b:?}");
            assert!(far(a, a0) <= 2, "zoom {zoom}: slime with no alarm keeps its colour: {a0:?} -> {a:?}");
            assert!(far(g, g0) <= 2, "zoom {zoom}: bare ground shows no alarm: {g0:?} -> {g:?}");
            view.set_alarm_side(0);
            let [_, other, _] = spots(&frame(&view));
            assert!(far(other, b0) <= 2, "zoom {zoom}: the other side holds no alarm: {b0:?} -> {other:?}");
        }
    }

    /// The Lure's steering field draws as cream rings of equal walking
    /// distance (2026-11-18): a field round a lure at cell (21, 25), a word
    /// a 2 x 2 block, reaching 30 cells. The ring 20 cells out is lit east
    /// and north of the lure; half way to it, at the lure's own block and
    /// where the next ring would fall past the field's reach nothing shows;
    /// on slime the ring lifts the picture by less than half what it does
    /// on bare ground; `set_steer(0)` draws the picture with no field. The
    /// Lure's scent light is lit without the field and not lit with it. At
    /// zoom 6 and zoom 2 (a ring is a hairline at both).
    #[test]
    fn the_lures_field_draws_as_cream_rings_and_its_scent_light_goes() {
        let Some((dev, q)) = test_gpu() else { return };
        let (w, h) = (64usize, 48usize);
        let px: Vec<u8> = (0..w * h).flat_map(|_| [12u8, 22, 26, 255]).collect();
        // A gold piece of 49 walkers across the ring south of the lure,
        // round (21, 45), and scent on the ground round (31, 25).
        let (mut look, mut walk) = (vec![0u32; w * h], Vec::new());
        for dy in 0..6 {
            for dx in 0..7 {
                let (x, y) = (18 + dx, 42 + dy);
                look[y * w + x] = 255 | (1 << 8);
                walk.extend([x as f32 + 0.5, y as f32 + 0.5, 0.0, 0.0]);
            }
        }
        for y in 19..31 {
            for x in 25..37 {
                look[y * w + x] |= 255 << 24;
            }
        }
        // The field as the engine writes it, from word 4 of `news`: the
        // walking distance (octile over the blocks) to the lure's block
        // (10, 12), in cells + 1, and 0 past 30 cells.
        let (bw, bh) = (w.div_ceil(2), h.div_ceil(2));
        let mut nw = vec![0u32; 4];
        for by in 0..bh {
            for bx in 0..bw {
                let (dx, dy) = ((bx as f32 - 10.0).abs(), (by as f32 - 12.0).abs());
                let d = 2.0 * (dx.max(dy) + (2f32.sqrt() - 1.0) * dx.min(dy));
                nw.push(if d <= 30.0 { d.round() as u32 + 1 } else { 0 });
            }
        }
        let buf = |label: &str, bytes: &[u8]| {
            let b = storage(&dev, label, bytes.len() as u64);
            q.write_buffer(&b, 0, bytes);
            b
        };
        let world = buf("world", &px);
        let look_b = buf("look", &crate::gpu::u32_bytes(&look));
        let ground_b = buf("ground", &vec![0u8; w * h * 4]);
        let walk_b = buf("walkers", &walk.iter().flat_map(|v| v.to_le_bytes()).collect::<Vec<u8>>());
        let news = buf("news", &crate::gpu::u32_bytes(&nw));
        let state = buf("state", &crate::gpu::u32_bytes(&[100u32, 0, 0, 0]));
        let screen = (384, 288);
        let mut view = View::new(&dev, &q, &world, (w, h), screen);
        view.set_look(&look_b, &ground_b, &walk_b, walk.len() as u32 / 4);
        view.set_steer(4);
        assert_eq!(view.steer_at, 0, "no field is drawn before the engine's news is bound");
        view.set_dense(Some((&news, &state)), 4 + (bw * bh) as u32, 0);
        let scent = Light { at: (31.0, 25.0), reach: 8.0, lift: -2.0, colour: [150.0, 235.0, 80.0], strength: 1.3 };
        for zoom in [6.0f32, 2.0] {
            let mut cam = Camera::new((w, h), screen);
            cam.zoom_to(zoom, (0.0, 0.0));
            cam.centre_on((32.0, 24.0));
            let mut frame = |steer: u32, lights: &[Light]| {
                view.set_steer(steer);
                view.set_lights(lights, (0.0, 0.0, w as f32, h as f32));
                let mut enc = dev.create_command_encoder(&Default::default());
                view.record(&mut enc, &cam, None);
                q.submit([enc.finish()]);
                view.picture()
            };
            let at = |pic: &image::RgbaImage, x: f32, y: f32| {
                let (sx, sy) = cam.to_screen((x, y));
                let p = pic.get_pixel(sx as u32, sy as u32).0;
                [p[0] as i32, p[1] as i32, p[2] as i32]
            };
            let far = |p: [i32; 3], q: [i32; 3]| (0..3).map(|k| (p[k] - q[k]).abs()).max().unwrap();
            let lift = |p: [i32; 3], q: [i32; 3]| (0..3).map(|k| p[k] - q[k]).sum::<i32>();
            let none = frame(0, &[]);
            let rings = frame(4, &[]);
            let (east, north) = ((41.0, 25.0), (21.0, 5.0));
            for (name, (x, y)) in [("east", east), ("north", north)] {
                let (a, b) = (at(&none, x, y), at(&rings, x, y));
                assert!((0..3).all(|k| b[k] > a[k] + 25) && b[0] >= b[2], "zoom {zoom}: the ring 20 cells {name} is lit cream: {a:?} -> {b:?}");
            }
            for (name, (x, y)) in [("half way to the ring", (31.0, 25.0)), ("the lure's own block", (21.0, 25.0)), ("past the reach", (61.0, 25.0))] {
                let (a, b) = (at(&none, x, y), at(&rings, x, y));
                assert!(far(a, b) <= 2, "zoom {zoom}: {name} shows nothing: {a:?} -> {b:?}");
            }
            let ground = lift(at(&rings, east.0, east.1), at(&none, east.0, east.1));
            let slime = lift(at(&rings, 21.0, 45.0), at(&none, 21.0, 45.0));
            assert!(at(&none, 21.0, 45.0)[0] > 150 && 2 * slime < ground, "zoom {zoom}: on slime the ring is under half as strong: {slime} against {ground}");
            // The scent light: lit on the scented ground without the field,
            // and with the field the picture is the rings' alone.
            let lit = frame(0, &[scent]);
            let (a, b) = (at(&none, 31.0, 25.0), at(&lit, 31.0, 25.0));
            assert!(b[1] > a[1] + 10, "zoom {zoom}: the Lure's scent light is lit with no field: {a:?} -> {b:?}");
            let both = frame(4, &[scent]);
            assert!(both.as_raw() == rings.as_raw(), "zoom {zoom}: the scent light is lit under the rings");
            assert!(frame(0, &[]).as_raw() == none.as_raw(), "zoom {zoom}: set_steer(0) left something drawn");
        }
    }

    /// No ring on the pull's rim (2026-11-18): on planets 2 and 3 the
    /// game's last ring, 120 cells out, fell on the field's edge, where the
    /// field stops on its blocks' steps, and drew as a dotted line. A field
    /// reaching 40 cells round the lure at (21, 25): with no reach given
    /// the ring 40 cells east is drawn, with the reach (`set_steer_reach`)
    /// it is not, and the ring 20 cells east is drawn either way.
    #[test]
    fn no_ring_is_drawn_on_the_lures_rim() {
        let Some((dev, q)) = test_gpu() else { return };
        let (w, h) = (64usize, 48usize);
        let px: Vec<u8> = (0..w * h).flat_map(|_| [12u8, 22, 26, 255]).collect();
        let (bw, bh) = (w.div_ceil(2), h.div_ceil(2));
        let mut nw = vec![0u32; 4];
        for by in 0..bh {
            for bx in 0..bw {
                let (dx, dy) = ((bx as f32 - 10.0).abs(), (by as f32 - 12.0).abs());
                let d = 2.0 * (dx.max(dy) + (2f32.sqrt() - 1.0) * dx.min(dy));
                nw.push(if d <= 40.0 { d.round() as u32 + 1 } else { 0 });
            }
        }
        let buf = |label: &str, bytes: &[u8]| {
            let b = storage(&dev, label, bytes.len() as u64);
            q.write_buffer(&b, 0, bytes);
            b
        };
        let world = buf("world", &px);
        let look_b = buf("look", &crate::gpu::u32_bytes(&vec![0u32; w * h]));
        let ground_b = buf("ground", &vec![0u8; w * h * 4]);
        let walk_b = buf("walkers", &[0u8; 16]);
        let news = buf("news", &crate::gpu::u32_bytes(&nw));
        let state = buf("state", &crate::gpu::u32_bytes(&[100u32, 0, 0, 0]));
        let screen = (384, 288);
        let mut view = View::new(&dev, &q, &world, (w, h), screen);
        view.set_look(&look_b, &ground_b, &walk_b, 0);
        view.set_dense(Some((&news, &state)), 4 + (bw * bh) as u32, 0);
        let mut cam = Camera::new((w, h), screen);
        cam.zoom_to(6.0, (0.0, 0.0));
        cam.centre_on((32.0, 24.0));
        let mut frame = |steer: u32, reach: f32| {
            view.set_steer(steer);
            view.set_steer_reach(reach);
            let mut enc = dev.create_command_encoder(&Default::default());
            view.record(&mut enc, &cam, None);
            q.submit([enc.finish()]);
            view.picture()
        };
        let at = |pic: &image::RgbaImage, x: f32, y: f32| {
            let (sx, sy) = cam.to_screen((x, y));
            let p = pic.get_pixel(sx as u32, sy as u32).0;
            [p[0] as i32, p[1] as i32, p[2] as i32]
        };
        let lit = |a: [i32; 3], b: [i32; 3]| (0..3).all(|k| b[k] > a[k] + 20);
        let none = frame(0, 0.0);
        let every = frame(4, 0.0);
        let kept = frame(4, 40.0);
        let (inner, rim) = ((41.0, 25.0), (61.0, 25.0));
        assert!(lit(at(&none, rim.0, rim.1), at(&every, rim.0, rim.1)), "with no reach given the rim's ring is drawn: {:?} -> {:?}", at(&none, rim.0, rim.1), at(&every, rim.0, rim.1));
        assert_eq!(at(&kept, rim.0, rim.1), at(&none, rim.0, rim.1), "a ring on the rim");
        for pic in [&every, &kept] {
            assert!(lit(at(&none, inner.0, inner.1), at(pic, inner.0, inner.1)), "the ring 20 cells out is drawn");
        }
    }

    /// A crystal shows its live charge (the owner's playtest, 2026-11-14
    /// 04:53Z: "harvesting crystals could look way better"): with the
    /// engine's charge bound, the full half is cyan and the drained half
    /// grey stone; charge taken between two frames lights the cells it
    /// went from, and the light fades once the taking stops.
    #[test]
    fn a_crystal_shows_its_live_charge_and_where_it_is_taken() {
        let Some((dev, q)) = test_gpu() else { return };
        let (w, h) = (64usize, 40usize);
        let px: Vec<u8> = (0..w * h).flat_map(|_| [12u8, 22, 26, 255]).collect();
        let world = storage(&dev, "world", px.len() as u64);
        q.write_buffer(&world, 0, &px);
        let (c, r) = ((32.0f32, 20.0f32), 9.0f32);
        let a = r * 3f32.sqrt() / 2.0;
        let mut pile = vec![0u16; w * h];
        let mut ground = vec![0u32; w * h];
        let full = 100.0f32;
        let mut charge = vec![0.0f32; w * h];
        for y in 0..h {
            for x in 0..w {
                let (dx, dy) = ((x as f32 + 0.5 - c.0).abs(), (y as f32 + 0.5 - c.1).abs());
                if a - dx.max(0.5 * dx + 0.8660254 * dy) >= 0.0 {
                    pile[y * w + x] = 1;
                    ground[y * w + x] = 5 | 255 << 8;
                    // The left half drained dry, the right half full.
                    charge[y * w + x] = if (x as f32) < c.0 { 0.0 } else { full };
                }
            }
        }
        let buf = |label: &str, bytes: &[u8]| {
            let b = storage(&dev, label, bytes.len() as u64);
            q.write_buffer(&b, 0, bytes);
            b
        };
        let f32s = |v: &[f32]| v.iter().flat_map(|x| x.to_le_bytes()).collect::<Vec<u8>>();
        let look_b = buf("look", &vec![0u8; w * h * 4]);
        let ground_b = buf("ground", &crate::gpu::u32_bytes(&ground));
        let walk_b = buf("walkers", &[0u8; 16]);
        let charge_b = buf("charge", &f32s(&charge));
        let screen = (384, 240);
        let mut view = View::new(&dev, &q, &world, (w, h), screen);
        view.set_look(&look_b, &ground_b, &walk_b, 0);
        view.set_crystals(&[(c, r)], &pile);
        view.set_charge(Some((&charge_b, &[full])));
        let mut cam = Camera::new((w, h), screen);
        cam.zoom_to(6.0, (0.0, 0.0));
        cam.centre_on(c);
        let frame = |view: &mut View, t: f32| {
            view.set_time(t);
            let mut enc = dev.create_command_encoder(&Default::default());
            view.record(&mut enc, &cam, None);
            q.submit([enc.finish()]);
            view.picture()
        };
        let at = |pic: &image::RgbaImage, x: f32, y: f32| {
            let (sx, sy) = cam.to_screen((x, y));
            let p = pic.get_pixel(sx as u32, sy as u32).0;
            [p[0] as i32, p[1] as i32, p[2] as i32]
        };
        let pic = frame(&mut view, 0.0);
        let (dry, wet) = (at(&pic, c.0 - 0.5 * a, c.1), at(&pic, c.0 + 0.5 * a, c.1));
        assert!(wet[2] > wet[0] + 40, "the full half is cyan: {wet:?}");
        assert!((dry[2] - dry[0]).abs() < 30, "the drained half is grey: {dry:?}");
        // A quarter of the full half's charge taken in 0.1 game seconds.
        let taken: Vec<f32> = charge.iter().enumerate().map(|(i, &v)| if (i % w) as f32 >= c.0 { v * 0.75 } else { v }).collect();
        q.write_buffer(&charge_b, 0, &f32s(&taken));
        let lit = frame(&mut view, 0.1);
        let p = at(&lit, c.0 + 0.5 * a, c.1);
        let before = at(&pic, c.0 + 0.5 * a, c.1);
        assert!(p.iter().sum::<i32>() > before.iter().sum::<i32>() + 60, "taking lights it: {before:?} -> {p:?}");
        // No more taking: two game seconds on, the light is gone.
        let calm = frame(&mut view, 2.1);
        let p2 = at(&calm, c.0 + 0.5 * a, c.1);
        assert!(p2.iter().sum::<i32>() < p.iter().sum::<i32>() - 60, "the light fades: {p:?} -> {p2:?}");
    }

    /// The slime's edge is as sharp as its walkers (2026-11-14, the owner's
    /// playtest: "The slime looks very blurry/out of focus"): a body that
    /// ends at a straight line rises from bare ground to body (10% to 90%)
    /// within a cell and a half at zoom 12: 0.96 cells now, 2.58 with the
    /// 5 x 5-cell window before.
    #[test]
    fn the_slime_edge_is_as_sharp_as_its_walkers() {
        let Some((dev, q)) = test_gpu() else { return };
        let (w, h) = (64usize, 40usize);
        let px: Vec<u8> = (0..w * h).flat_map(|i| if i % w < 32 { [200u8, 160, 60, 255] } else { [12, 22, 26, 255] }).collect();
        let world = storage(&dev, "world", px.len() as u64);
        q.write_buffer(&world, 0, &px);
        let (mut look, mut walk) = (vec![0u32; w * h], Vec::new());
        for y in 0..h {
            for x in 0..32 {
                look[y * w + x] = 255 | (1 << 8);
                walk.extend([x as f32 + 0.5, y as f32 + 0.5, 0.0, 0.0]);
            }
        }
        let buf = |label: &str, bytes: &[u8]| {
            let b = storage(&dev, label, bytes.len() as u64);
            q.write_buffer(&b, 0, bytes);
            b
        };
        let look_b = buf("look", &crate::gpu::u32_bytes(&look));
        let ground = buf("ground", &vec![0u8; w * h * 4]);
        let walk_b = buf("walkers", &walk.iter().flat_map(|v| v.to_le_bytes()).collect::<Vec<u8>>());
        let screen = (384, 240);
        let mut view = View::new(&dev, &q, &world, (w, h), screen);
        view.set_look(&look_b, &ground, &walk_b, (walk.len() / 4) as u32);
        view.set_body_density((walk.len() / 4) as f32 / (w * h) as f32);
        let mut cam = Camera::new((w, h), screen);
        cam.zoom_to(12.0, (0.0, 0.0));
        cam.centre_on((32.0, 20.0));
        let mut enc = dev.create_command_encoder(&Default::default());
        view.record(&mut enc, &cam, None);
        q.submit([enc.finish()]);
        let pic = view.picture();
        // The mean of a column's red over rows clear of specks, across the edge.
        let col = |x: f32| {
            let mut sum = 0.0;
            for k in 0..10 {
                let (sx, sy) = cam.to_screen((x, 15.2 + k as f32));
                sum += pic.get_pixel(sx as u32, sy as u32).0[0] as f32;
            }
            sum / 10.0
        };
        let (body, bare) = (col(26.2), col(38.2));
        assert!(body > bare + 40.0, "body {body} against bare {bare}");
        let at = |share: f32| (0..200).map(|i| 26.0 + i as f32 * 0.06).find(|&x| col(x) < bare + share * (body - bare)).unwrap_or(38.0);
        let width = at(0.1) - at(0.9);
        assert!(width < 1.5, "the edge rises over {width:.2} cells (body {body}, bare {bare})");
    }

    /// A lone walker on bare ground reads plainly (owner's playtest, 11-14
    /// 04:50Z: "single individual walkers should be brighter"), while a
    /// walker inside a thick body adds only a calm grain: the speck's light
    /// falls with the walkers round it.
    #[test]
    fn a_lone_walker_is_bright_and_the_body_stays_calm() {
        let Some((dev, q)) = test_gpu() else { return };
        let (w, h) = (64usize, 40usize);
        // Agar, with the engine's mould colour round the lone walker (as the
        // engine tints a cell with mould on it).
        let px: Vec<u8> = (0..w * h).flat_map(|i| if (i % w).abs_diff(44) <= 2 && (i / w).abs_diff(20) <= 2 { [200u8, 160, 60, 255] } else { [12, 22, 26, 255] }).collect();
        let world = storage(&dev, "world", px.len() as u64);
        q.write_buffer(&world, 0, &px);
        // A body of 4 walkers a cell over x 4..20, and one walker alone at
        // (44.5, 20.5) with a faint trail under it.
        let (mut look, mut walk) = (vec![0u32; w * h], Vec::new());
        for y in 4..36 {
            for x in 4..20 {
                look[y * w + x] = 255 | (4 << 8);
                for k in 0..4 {
                    walk.extend([x as f32 + 0.2 + 0.2 * k as f32, y as f32 + 0.3 + 0.13 * k as f32, 0.0, 0.0]);
                }
            }
        }
        look[20 * w + 44] = 200 | (1 << 8);
        walk.extend([44.5f32, 20.5, 0.0, 0.0]);
        let buf = |label: &str, bytes: &[u8]| {
            let b = storage(&dev, label, bytes.len() as u64);
            q.write_buffer(&b, 0, bytes);
            b
        };
        let look_b = buf("look", &crate::gpu::u32_bytes(&look));
        let ground = buf("ground", &vec![0u8; w * h * 4]);
        let walk_b = buf("walkers", &walk.iter().flat_map(|v| v.to_le_bytes()).collect::<Vec<u8>>());
        let screen = (384, 240);
        let mut view = View::new(&dev, &q, &world, (w, h), screen);
        view.set_look(&look_b, &ground, &walk_b, (walk.len() / 4) as u32);
        let mut cam = Camera::new((w, h), screen);
        cam.zoom_to(6.0, (0.0, 0.0));
        cam.centre_on((32.0, 20.0));
        let mut enc = dev.create_command_encoder(&Default::default());
        view.record(&mut enc, &cam, None);
        q.submit([enc.finish()]);
        let pic = view.picture();
        let lum = |x: f32, y: f32| {
            let (sx, sy) = cam.to_screen((x, y));
            let p = pic.get_pixel(sx as u32, sy as u32).0;
            0.299 * p[0] as f32 + 0.587 * p[1] as f32 + 0.114 * p[2] as f32
        };
        let (lone, bare) = (lum(44.5, 20.5), lum(52.5, 20.5));
        assert!(lone > bare + 90.0, "a lone walker {lone} on bare ground {bare}");
        // And it is a point of light, not a dome (art director, 11-14
        // 08:15Z: the frontier read as bubble wrap): no shaded flank or cast
        // shadow on the side away from the light: the darkest pixel within
        // 1.5 cells of it stays well above bare ground.
        let mut lee = f32::MAX;
        for j in 0..18 {
            for i in 0..18 {
                lee = lee.min(lum(43.0 + i as f32 / 6.0, 19.0 + j as f32 / 6.0));
            }
        }
        // Flat, its film is lit all round it: 60.5 against bare 42.1, where
        // the dome's shaded flank came down to 44.6 against 40.3.
        assert!(lee > bare + 10.0, "the darkest pixel round a lone walker {lee} against bare ground {bare}");
        // In the body: the brightest and dimmest of a cell's pixels differ
        // by less than the lone walker stands above bare ground.
        let mut vals = Vec::new();
        for j in 0..6 {
            for i in 0..6 {
                vals.push(lum(12.0 + i as f32 / 6.0, 20.0 + j as f32 / 6.0));
            }
        }
        let spread = vals.iter().cloned().fold(f32::MIN, f32::max) - vals.iter().cloned().fold(f32::MAX, f32::min);
        assert!(spread < 0.6 * (lone - bare), "the body's grain spread {spread} against the lone walker's {}", lone - bare);
    }

    /// A mini-boss's members (2026-11-16) carry its look: a body whose
    /// walkers are a lump's members takes the lump's skin (Violet: blue
    /// well above green) where the same body of ordinary walkers stays
    /// gold; and a lump drawn Plain, or no lumps at all, leaves the
    /// picture exactly as it was.
    #[test]
    fn a_mini_boss_body_takes_its_look_and_plain_changes_nothing() {
        let Some((dev, q)) = test_gpu() else { return };
        let (w, h) = (64usize, 32usize);
        // Two gold bodies, x 4..24 (the lump's) and x 40..60 (ordinary),
        // 4 walkers a cell.
        let body = |x: usize, y: usize| ((4..24).contains(&x) || (40..60).contains(&x)) && (6..26).contains(&y);
        let px: Vec<u8> = (0..w * h).flat_map(|i| if body(i % w, i / w) { [200u8, 150, 70, 255] } else { [12, 22, 26, 255] }).collect();
        let world = storage(&dev, "world", px.len() as u64);
        q.write_buffer(&world, 0, &px);
        let (mut look, mut walk, mut words) = (vec![0u32; w * h], Vec::new(), Vec::new());
        for y in 0..h {
            for x in 0..w {
                if !body(x, y) {
                    continue;
                }
                look[y * w + x] = 255 | (4 << 8);
                for k in 0..4 {
                    walk.extend([x as f32 + 0.2 + 0.2 * k as f32, y as f32 + 0.3 + 0.13 * k as f32, 0.0, 0.0]);
                    // Lump 0's members on the left (its index + 1 in bits 25-27).
                    words.push(if x < 32 { 1u32 << 25 } else { 0 });
                }
            }
        }
        let buf = |label: &str, bytes: &[u8]| {
            let b = storage(&dev, label, bytes.len() as u64);
            q.write_buffer(&b, 0, bytes);
            b
        };
        let look_b = buf("look", &crate::gpu::u32_bytes(&look));
        let ground = buf("ground", &vec![0u8; w * h * 4]);
        let walk_b = buf("walkers", &walk.iter().flat_map(|v| v.to_le_bytes()).collect::<Vec<u8>>());
        // The dense words from word 4 of `news`, as the engine's buffer has
        // a head before them; the step now in `state`.
        let mut nw = vec![0u32; 4];
        nw.extend(&words);
        let news = buf("news", &crate::gpu::u32_bytes(&nw));
        let state = buf("state", &crate::gpu::u32_bytes(&[100u32, 0, 0, 0]));
        let screen = (384, 192);
        let mut view = View::new(&dev, &q, &world, (w, h), screen);
        view.set_look(&look_b, &ground, &walk_b, (walk.len() / 4) as u32);
        view.set_dense(Some((&news, &state)), 4, 0);
        let mut cam = Camera::new((w, h), screen);
        cam.zoom_to(6.0, (0.0, 0.0));
        cam.centre_on((32.0, 16.0));
        let mut shot = |lumps: &[LumpBody]| {
            view.set_lumps(lumps);
            let mut enc = dev.create_command_encoder(&Default::default());
            view.record(&mut enc, &cam, None);
            q.submit([enc.finish()]);
            view.picture()
        };
        let lump = |look: LumpLook, remain: f32, fill: f32| LumpBody { at: (14.0, 16.0), look, strength: 1.0, remain, fill, held: 0.0 };
        let none = shot(&[]);
        let plain = shot(&[lump(LumpLook::Plain, 1.0, 0.0)]);
        assert!(none.as_raw() == plain.as_raw(), "a Plain lump changed the picture");
        let violet = shot(&[lump(LumpLook::Violet, 1.0, 0.0)]);
        let half = shot(&[lump(LumpLook::Violet, 0.5, 0.0)]);
        let gone = shot(&[lump(LumpLook::Violet, 0.0, 0.0)]);
        let (ink, drunk, bone) = (shot(&[lump(LumpLook::Ink, 1.0, 0.0)]), shot(&[lump(LumpLook::Ink, 1.0, 1.0)]), shot(&[lump(LumpLook::Bone, 1.0, 0.0)]));
        let mean = |pic: &image::RgbaImage, cx: f32, cy: f32| {
            let (x0, y0) = cam.to_screen((cx - 3.0, cy - 3.0));
            let (x1, y1) = cam.to_screen((cx + 3.0, cy + 3.0));
            let mut s = [0.0f32; 3];
            let mut n = 0.0;
            for y in y0 as u32..y1 as u32 {
                for x in x0 as u32..x1 as u32 {
                    let p = pic.get_pixel(x, y).0;
                    for c in 0..3 {
                        s[c] += p[c] as f32;
                    }
                    n += 1.0;
                }
            }
            s.map(|v| v / n)
        };
        let (boss, gold, before) = (mean(&violet, 14.0, 16.0), mean(&violet, 50.0, 16.0), mean(&none, 14.0, 16.0));
        assert!(boss[2] > boss[1] + 40.0 && boss[0] > boss[1], "the lump's body is violet: {boss:?}");
        assert!(gold[0] > gold[2] + 30.0 && gold[1] > gold[2] + 15.0, "the ordinary body stays gold: {gold:?}");
        assert!(before[1] > before[2] + 15.0, "without a look the lump's body is gold too: {before:?}");
        // Only the lump's side moved.
        assert_eq!(mean(&none, 50.0, 16.0), gold, "the ordinary body changed with the lump's look");
        // A fading lump's look drains out: half way it lies between, gone
        // it is the slime's own gold again (its members still marked).
        let (h, g) = (mean(&half, 14.0, 16.0), mean(&gone, 14.0, 16.0));
        let violetness = |c: [f32; 3]| c[2] - c[1];
        assert!(violetness(h) < violetness(boss) - 10.0 && violetness(h) > violetness(before) + 10.0, "half faded is between: {h:?} {boss:?} {before:?}");
        assert!((0..3).all(|c| (g[c] - before[c]).abs() < 8.0), "a faded lump is the slime's gold: {g:?} {before:?}");
        // At 4 members a cell the core light is full and lights the whole
        // body (in play a held body is about 0.5 a cell): ink's core is
        // lilac, a leech's drink turns it to the charge's cyan, and bone's
        // is the faint one, so bone stays an unsaturated pale grey.
        let (i, d, b) = (mean(&ink, 14.0, 16.0), mean(&drunk, 14.0, 16.0), mean(&bone, 14.0, 16.0));
        assert!(i[2] > i[1] + 30.0 && i[0] > i[1], "ink lights lilac: {i:?}");
        assert!(d[1] > d[0] + 20.0 && d[1] > i[1] + 20.0, "a full leech glows cyan: {d:?} against {i:?}");
        let spread = |c: [f32; 3]| c.iter().cloned().fold(f32::MIN, f32::max) - c.iter().cloned().fold(f32::MAX, f32::min);
        assert!(spread(b) < 25.0 && b[2] > before[2] + 60.0, "bone is a pale grey, not gold: {b:?}");
    }

    /// A held lump (out on its errand, `LumpBody::held`) shows wherever
    /// its own members stand, even spread thin through a thick sheet: one
    /// walker in four is a member (a member a cell, a quarter of the
    /// walkers). Gathering (held 0), by its share, it only tints the sheet;
    /// held, it is its own colour. A lone member a few cells off adds
    /// nothing either way (mini-boss #396: a leech ringing its pump showed
    /// only as flecks in a gold collar).
    #[test]
    fn a_held_lump_shows_where_its_members_stand_thin_in_the_sheet() {
        let Some((dev, q)) = test_gpu() else { return };
        let (w, h) = (64usize, 32usize);
        let body = |x: usize, y: usize| (4..60).contains(&x) && (6..26).contains(&y);
        let px: Vec<u8> = (0..w * h).flat_map(|i| if body(i % w, i / w) { [200u8, 150, 70, 255] } else { [12, 22, 26, 255] }).collect();
        let world = storage(&dev, "world", px.len() as u64);
        q.write_buffer(&world, 0, &px);
        let (mut look, mut walk, mut words) = (vec![0u32; w * h], Vec::new(), Vec::new());
        for y in 0..h {
            for x in 0..w {
                if !body(x, y) {
                    continue;
                }
                look[y * w + x] = 255 | (4 << 8);
                for k in 0..4 {
                    walk.extend([x as f32 + 0.2 + 0.2 * k as f32, y as f32 + 0.3 + 0.13 * k as f32, 0.0, 0.0]);
                    // A member a cell over x 4..24, and one stray member
                    // in the far body at (48, 16).
                    let member = (k == 0 && x < 24) || (k == 0 && x == 48 && y == 16);
                    words.push(if member { 1u32 << 25 } else { 0 });
                }
            }
        }
        let buf = |label: &str, bytes: &[u8]| {
            let b = storage(&dev, label, bytes.len() as u64);
            q.write_buffer(&b, 0, bytes);
            b
        };
        let look_b = buf("look", &crate::gpu::u32_bytes(&look));
        let ground = buf("ground", &vec![0u8; w * h * 4]);
        let walk_b = buf("walkers", &walk.iter().flat_map(|v| v.to_le_bytes()).collect::<Vec<u8>>());
        let mut nw = vec![0u32; 4];
        nw.extend(&words);
        let news = buf("news", &crate::gpu::u32_bytes(&nw));
        let state = buf("state", &crate::gpu::u32_bytes(&[100u32, 0, 0, 0]));
        let screen = (384, 192);
        let mut view = View::new(&dev, &q, &world, (w, h), screen);
        view.set_look(&look_b, &ground, &walk_b, (walk.len() / 4) as u32);
        view.set_dense(Some((&news, &state)), 4, 0);
        let mut cam = Camera::new((w, h), screen);
        cam.zoom_to(6.0, (0.0, 0.0));
        cam.centre_on((32.0, 16.0));
        let mut shot = |held: f32| {
            view.set_lumps(&[LumpBody { at: (14.0, 16.0), look: LumpLook::Violet, strength: 1.0, held, ..LumpBody::default() }]);
            let mut enc = dev.create_command_encoder(&Default::default());
            view.record(&mut enc, &cam, None);
            q.submit([enc.finish()]);
            view.picture()
        };
        let (gathering, held) = (shot(0.0), shot(1.0));
        let mean = |pic: &image::RgbaImage, cx: f32, cy: f32, r: f32| {
            let (x0, y0) = cam.to_screen((cx - r, cy - r));
            let (x1, y1) = cam.to_screen((cx + r, cy + r));
            let mut s = [0.0f32; 3];
            let mut n = 0.0;
            for y in y0 as u32..y1 as u32 {
                for x in x0 as u32..x1 as u32 {
                    let p = pic.get_pixel(x, y).0;
                    for c in 0..3 {
                        s[c] += p[c] as f32;
                    }
                    n += 1.0;
                }
            }
            s.map(|v| v / n)
        };
        let violetness = |c: [f32; 3]| c[2] - c[1];
        let (g, h) = (mean(&gathering, 14.0, 16.0, 3.0), mean(&held, 14.0, 16.0, 3.0));
        assert!(violetness(g) < 0.0, "gathering, a quarter of the walkers only tints the gold: {g:?}");
        assert!(violetness(h) > 40.0 && h[0] > h[1], "held, the same members are violet: {h:?}");
        // The stray member: the far body the same gold either way.
        let (sg, sh) = (mean(&gathering, 48.0, 16.0, 1.5), mean(&held, 48.0, 16.0, 1.5));
        assert!((0..3).all(|c| (sg[c] - sh[c]).abs() < 6.0) && violetness(sh) < -40.0, "a stray member shows nothing: {sg:?} {sh:?}");
    }

    /// Two lumps side by side each take their own look, condensed members
    /// stay their lump's colour (not bronze), and members cut off from food
    /// draw no panic: at zoom 2, where every walker is its speck, the
    /// ordinary body's stale walkers bolt red and the members' do not (the
    /// engine spares members withering and panic; review, 2026-11-16).
    #[test]
    fn two_lumps_keep_their_looks_and_members_show_no_panic_or_bronze() {
        let Some((dev, q)) = test_gpu() else { return };
        let (w, h) = (72usize, 32usize);
        // Lump 0 over x 4..24, lump 1 over x 24..44 (touching), ordinary
        // walkers over x 52..68; 4 walkers a cell, every one condensed.
        let side = |x: usize| if x < 24 { 1u32 } else if x < 44 { 2 } else { 0 };
        let body = |x: usize, y: usize| ((4..44).contains(&x) || (52..68).contains(&x)) && (6..26).contains(&y);
        let px: Vec<u8> = (0..w * h).flat_map(|i| if body(i % w, i / w) { [200u8, 150, 70, 255] } else { [12, 22, 26, 255] }).collect();
        let world = storage(&dev, "world", px.len() as u64);
        q.write_buffer(&world, 0, &px);
        let (mut look, mut walk, mut words) = (vec![0u32; w * h], Vec::new(), Vec::new());
        for y in 0..h {
            for x in 0..w {
                if !body(x, y) {
                    continue;
                }
                look[y * w + x] = 255 | (4 << 8);
                for k in 0..4 {
                    walk.extend([x as f32 + 0.2 + 0.2 * k as f32, y as f32 + 0.3 + 0.13 * k as f32, 0.0, 0.0]);
                    words.push((side(x) << 25) | 0x10000);
                }
            }
        }
        let n = words.len();
        let buf = |label: &str, bytes: &[u8]| {
            let b = storage(&dev, label, bytes.len() as u64);
            q.write_buffer(&b, 0, bytes);
            b
        };
        let look_b = buf("look", &crate::gpu::u32_bytes(&look));
        let ground = buf("ground", &vec![0u8; w * h * 4]);
        let walk_b = buf("walkers", &walk.iter().flat_map(|v| v.to_le_bytes()).collect::<Vec<u8>>());
        // Every walker last heard of food at step 0; it is step 1000, the
        // lag 10 and the lean 5: all long cut off. The dense words follow
        // the news words.
        let mut nw = vec![0u32; n];
        nw.extend(&words);
        let news = buf("news", &crate::gpu::u32_bytes(&nw));
        let state = buf("state", &crate::gpu::u32_bytes(&[1000u32, 0, 0, 0]));
        let screen = (432, 192);
        let mut view = View::new(&dev, &q, &world, (w, h), screen);
        view.set_look(&look_b, &ground, &walk_b, n as u32);
        view.set_news(Some((&news, &state)), 10, 5);
        view.set_dense(Some((&news, &state)), n as u32, 0);
        view.set_lumps(&[LumpBody { at: (14.0, 16.0), look: LumpLook::Violet, strength: 0.5, ..LumpBody::default() }, LumpBody { at: (34.0, 16.0), look: LumpLook::Ember, strength: 0.5, ..LumpBody::default() }]);
        let mut cam = Camera::new((w, h), screen);
        let shot = |cam: &Camera| {
            let mut enc = dev.create_command_encoder(&Default::default());
            view.record(&mut enc, cam, None);
            q.submit([enc.finish()]);
            view.picture()
        };
        cam.zoom_to(6.0, (0.0, 0.0));
        cam.centre_on((36.0, 16.0));
        let near = shot(&cam);
        let mean = |pic: &image::RgbaImage, cam: &Camera, cx: f32, cy: f32| {
            let (x0, y0) = cam.to_screen((cx - 3.0, cy - 3.0));
            let (x1, y1) = cam.to_screen((cx + 3.0, cy + 3.0));
            let mut s = [0.0f32; 3];
            let mut c = 0.0;
            for y in y0 as u32..y1 as u32 {
                for x in x0 as u32..x1 as u32 {
                    let p = pic.get_pixel(x, y).0;
                    for i in 0..3 {
                        s[i] += p[i] as f32;
                    }
                    c += 1.0;
                }
            }
            s.map(|v| v / c)
        };
        let (a, b) = (mean(&near, &cam, 12.0, 16.0), mean(&near, &cam, 36.0, 16.0));
        assert!(a[2] > a[1] + 40.0 && a[0] > a[1], "lump 0 is violet, condensed and all: {a:?}");
        assert!(b[0] > b[1] + 40.0 && b[0] > b[2], "lump 1 is crimson: {b:?}");
        // Zoomed out to 2: count the bolting red specks a body.
        cam.zoom_to(2.0, (0.0, 0.0));
        cam.centre_on((36.0, 16.0));
        let far = shot(&cam);
        let red = |x0: f32, x1: f32| {
            let (sx0, sy0) = cam.to_screen((x0, 6.0));
            let (sx1, sy1) = cam.to_screen((x1, 26.0));
            let mut k = 0;
            for y in sy0 as u32..sy1 as u32 {
                for x in sx0 as u32..sx1 as u32 {
                    let p = far.get_pixel(x, y).0;
                    if p[0] > 180 && (p[1] as u32) < 90 && (p[2] as u32) < 90 {
                        k += 1;
                    }
                }
            }
            k
        };
        let (members, ordinary) = (red(4.0, 44.0), red(52.0, 68.0));
        assert!(ordinary > 50, "the control: the ordinary body's stale walkers bolt red ({ordinary} px)");
        assert_eq!(members, 0, "members drew {members} red panic pixels");
    }

    /// Walkers on a crater (trail wiped, the engine's colour bare agar)
    /// inside a gold body are gold, not black (owner, 2026-11-14 08:30Z:
    /// "Are these blackish spots in the middle of the mold intentional?").
    #[test]
    fn walkers_in_a_crater_stay_mould_coloured() {
        let Some((dev, q)) = test_gpu() else { return };
        let (w, h) = (48usize, 32usize);
        // A gold body over x 4..40, its trail wiped in a crater over
        // x 18..26, y 12..20 (the engine draws agar there), 4 walkers a
        // cell throughout.
        let crater = |x: usize, y: usize| (18..26).contains(&x) && (12..20).contains(&y);
        let mut px = Vec::new();
        let (mut look, mut walk) = (vec![0u32; w * h], Vec::new());
        for y in 0..h {
            for x in 0..w {
                let body = (4..40).contains(&x) && (4..28).contains(&y);
                let gold = body && !crater(x, y);
                px.extend(if gold { [200u8, 150, 70, 255] } else { [12, 22, 26, 255] });
                if body {
                    look[y * w + x] = if gold { 255 } else { 0 } | (4 << 8);
                    for k in 0..4 {
                        walk.extend([x as f32 + 0.2 + 0.2 * k as f32, y as f32 + 0.3 + 0.13 * k as f32, 0.0, 0.0]);
                    }
                }
            }
        }
        let world = storage(&dev, "world", px.len() as u64);
        q.write_buffer(&world, 0, &px);
        let buf = |label: &str, bytes: &[u8]| {
            let b = storage(&dev, label, bytes.len() as u64);
            q.write_buffer(&b, 0, bytes);
            b
        };
        let look_b = buf("look", &crate::gpu::u32_bytes(&look));
        let ground = buf("ground", &vec![0u8; w * h * 4]);
        let walk_b = buf("walkers", &walk.iter().flat_map(|v| v.to_le_bytes()).collect::<Vec<u8>>());
        let screen = (288, 192);
        let mut view = View::new(&dev, &q, &world, (w, h), screen);
        view.set_look(&look_b, &ground, &walk_b, (walk.len() / 4) as u32);
        let mut cam = Camera::new((w, h), screen);
        cam.zoom_to(6.0, (0.0, 0.0));
        cam.centre_on((24.0, 16.0));
        let mut enc = dev.create_command_encoder(&Default::default());
        view.record(&mut enc, &cam, None);
        q.submit([enc.finish()]);
        let pic = view.picture();
        // The mean colour over a 2 x 2 cell patch, every screen pixel.
        let mean = |cx: f32, cy: f32| {
            let (x0, y0) = cam.to_screen((cx - 1.0, cy - 1.0));
            let (x1, y1) = cam.to_screen((cx + 1.0, cy + 1.0));
            let mut s = [0.0f32; 3];
            let mut n = 0.0;
            for y in y0 as u32..y1 as u32 {
                for x in x0 as u32..x1 as u32 {
                    let p = pic.get_pixel(x, y).0;
                    for c in 0..3 {
                        s[c] += p[c] as f32;
                    }
                    n += 1.0;
                }
            }
            s.map(|v| v / n)
        };
        let lum = |c: [f32; 3]| 0.299 * c[0] + 0.587 * c[1] + 0.114 * c[2];
        let (hole, bare) = (mean(22.0, 16.0), mean(44.0, 16.0));
        assert!(lum(hole) > lum(bare) + 25.0, "the crater's walkers {hole:?} against bare ground {bare:?}");
        assert!(hole[0] > hole[2] + 30.0 && hole[1] > hole[2], "the crater's walkers are mould-coloured: {hole:?}");
    }

    /// Rubble is ground, so slime on it is drawn (the walls thread,
    /// 2026-11-16): a wall cell eaten to rubble is open ground in the
    /// simulation, and the slime walks over it. Drawn as a low wall over the
    /// slime, a ring three-quarters eaten still looked whole and the slime
    /// seemed to climb over it. A gold body over x 4..40 with a band of
    /// rubble under it at x 18..24: the band is mould-coloured, as bright
    /// as the body beside it, and bare rubble past the body is ground dark.
    #[test]
    fn slime_on_rubble_is_drawn_and_rubble_is_ground() {
        let Some((dev, q)) = test_gpu() else { return };
        let (w, h) = (48usize, 32usize);
        let rubble = |x: usize| (18..24).contains(&x);
        let mut px = Vec::new();
        let (mut look, mut walk, mut ground) = (vec![0u32; w * h], Vec::new(), vec![0u32; w * h]);
        for y in 0..h {
            for x in 0..w {
                let body = (4..40).contains(&x) && (4..20).contains(&y);
                px.extend(if body { [200u8, 150, 70, 255] } else { [12, 22, 26, 255] });
                if rubble(x) {
                    ground[y * w + x] = crate::world::look::code(crate::world::look::Ground::Rubble, 0.0);
                }
                if body {
                    look[y * w + x] = 255 | (4 << 8);
                    for k in 0..4 {
                        walk.extend([x as f32 + 0.2 + 0.2 * k as f32, y as f32 + 0.3 + 0.13 * k as f32, 0.0, 0.0]);
                    }
                }
            }
        }
        let world = storage(&dev, "world", px.len() as u64);
        q.write_buffer(&world, 0, &px);
        let buf = |label: &str, bytes: &[u8]| {
            let b = storage(&dev, label, bytes.len() as u64);
            q.write_buffer(&b, 0, bytes);
            b
        };
        let look_b = buf("look", &crate::gpu::u32_bytes(&look));
        let ground_b = buf("ground", &crate::gpu::u32_bytes(&ground));
        let walk_b = buf("walkers", &walk.iter().flat_map(|v| v.to_le_bytes()).collect::<Vec<u8>>());
        let screen = (288, 192);
        let mut view = View::new(&dev, &q, &world, (w, h), screen);
        view.set_look(&look_b, &ground_b, &walk_b, (walk.len() / 4) as u32);
        let mut cam = Camera::new((w, h), screen);
        cam.zoom_to(6.0, (0.0, 0.0));
        cam.centre_on((24.0, 16.0));
        let mut enc = dev.create_command_encoder(&Default::default());
        view.record(&mut enc, &cam, None);
        q.submit([enc.finish()]);
        let pic = view.picture();
        let mean = |cx: f32, cy: f32| {
            let (x0, y0) = cam.to_screen((cx - 1.0, cy - 1.0));
            let (x1, y1) = cam.to_screen((cx + 1.0, cy + 1.0));
            let mut s = [0.0f32; 3];
            let mut n = 0.0;
            for y in y0 as u32..y1 as u32 {
                for x in x0 as u32..x1 as u32 {
                    let p = pic.get_pixel(x, y).0;
                    for c in 0..3 {
                        s[c] += p[c] as f32;
                    }
                    n += 1.0;
                }
            }
            s.map(|v| v / n)
        };
        let lum = |c: [f32; 3]| 0.299 * c[0] + 0.587 * c[1] + 0.114 * c[2];
        let (on, body, bare_rubble, bare) = (mean(21.0, 12.0), mean(11.0, 12.0), mean(21.0, 27.0), mean(11.0, 27.0));
        assert!(on[0] > on[2] + 30.0 && on[1] > on[2], "slime on rubble is mould-coloured: {on:?}");
        assert!(lum(on) > lum(body) - 20.0, "as bright as the body beside it: {on:?} against {body:?}");
        assert!((lum(bare_rubble) - lum(bare)).abs() < 25.0, "bare rubble is ground dark: {bare_rubble:?} against {bare:?}");
    }

    /// A door is drawn as a gate, not as wall or water (the walls thread,
    /// 2026-11-16): hard terrain in timber brown with a grate of bars a
    /// cell apart. A door band at x 10..16 and a wall band at x 30..36,
    /// both whole, on bare ground: the door is warmer than the wall, and
    /// its pixels swing between bar and gap where the wall's hedge grain
    /// does less.
    #[test]
    fn a_door_draws_as_a_timber_grate() {
        let Some((dev, q)) = test_gpu() else { return };
        let (w, h) = (48usize, 32usize);
        let px: Vec<u8> = (0..w * h).flat_map(|_| [12u8, 22, 26, 255]).collect();
        let mut ground = vec![0u32; w * h];
        for y in 0..h {
            for x in 0..w {
                use crate::world::look::{code, Ground};
                if (10..16).contains(&x) {
                    ground[y * w + x] = code(Ground::Door, 1.0);
                } else if (30..36).contains(&x) {
                    ground[y * w + x] = code(Ground::Wall, 1.0);
                }
            }
        }
        let world = storage(&dev, "world", px.len() as u64);
        q.write_buffer(&world, 0, &px);
        let buf = |label: &str, bytes: &[u8]| {
            let b = storage(&dev, label, bytes.len() as u64);
            q.write_buffer(&b, 0, bytes);
            b
        };
        let look_b = buf("look", &crate::gpu::u32_bytes(&vec![0u32; w * h]));
        let ground_b = buf("ground", &crate::gpu::u32_bytes(&ground));
        let walk_b = buf("walkers", &[0u8; 16]);
        let screen = (384, 256);
        let mut view = View::new(&dev, &q, &world, (w, h), screen);
        view.set_look(&look_b, &ground_b, &walk_b, 0);
        let mut cam = Camera::new((w, h), screen);
        cam.zoom_to(12.0, (0.0, 0.0));
        cam.centre_on((23.0, 16.0));
        let mut enc = dev.create_command_encoder(&Default::default());
        view.record(&mut enc, &cam, None);
        q.submit([enc.finish()]);
        let pic = view.picture();
        // Over cells (x0..x1, 12..20), well inside the band: the mean and
        // the spread of luminance.
        let lum = |p: [u8; 4]| 0.299 * p[0] as f32 + 0.587 * p[1] as f32 + 0.114 * p[2] as f32;
        let band = |x0: f32, x1: f32| {
            let (a, b) = (cam.to_screen((x0, 12.0)), cam.to_screen((x1, 20.0)));
            let mut s = [0.0f32; 3];
            let (mut n, mut lo, mut hi) = (0.0, f32::MAX, 0.0f32);
            for y in a.1 as u32..b.1 as u32 {
                for x in a.0 as u32..b.0 as u32 {
                    let p = pic.get_pixel(x, y).0;
                    for c in 0..3 {
                        s[c] += p[c] as f32;
                    }
                    (lo, hi) = (lo.min(lum(p)), hi.max(lum(p)));
                    n += 1.0;
                }
            }
            (s.map(|v| v / n), hi - lo)
        };
        let ((door, door_swing), (wall, _)) = (band(11.5, 14.5), band(31.5, 34.5));
        assert!(door[0] > door[2] + 25.0, "the door is timber, not slate or water: {door:?}");
        assert!(door[0] - door[2] > wall[0] - wall[2] + 10.0, "warmer than the wall: {door:?} against {wall:?}");
        assert!(door_swing > 30.0, "its grate swings bar to gap: {door_swing}");
    }

    /// A squad of 40 you can count (Look A, 2026-11-18): every living
    /// soldier's head is drawn again above every body (`Marks::soldier_head`),
    /// so a soldier whose neighbour's shoulders cover its middle still shows
    /// its white helmet, and a soldier standing alone looks exactly as it
    /// did. Against the 11-18 shader (the head mark drawn as a whole figure)
    /// the neighbour's shoulders cover it again.
    #[test]
    fn a_packed_soldier_shows_its_head_and_a_lone_one_is_unchanged() {
        let Some((dev, q)) = test_gpu() else { return };
        let (w, h) = (48usize, 32usize);
        let px: Vec<u8> = (0..w * h).flat_map(|_| [40u8, 44, 40, 255]).collect();
        let world = storage(&dev, "world", px.len() as u64);
        q.write_buffer(&world, 0, &px);
        let screen = (384, 256);
        let mut view = View::new(&dev, &q, &world, (w, h), screen);
        let mut cam = Camera::new((w, h), screen);
        cam.zoom_to(12.0, (0.0, 0.0));
        cam.centre_on((24.0, 16.0));
        let (r, team) = (2.0, crate::world::marks::team_colour(0));
        let mut shot = |m: &Marks| {
            view.set_marks(m);
            let mut enc = dev.create_command_encoder(&Default::default());
            view.record(&mut enc, &cam, None);
            q.submit([enc.finish()]);
            view.picture()
        };
        // Both facing east, B 0.7 radii below A, drawn after it: B's
        // shoulders (the body is 0.8 radii across the facing) cover A's
        // middle.
        let (a, b) = ((24.0, 16.0), (24.0, 16.0 + 0.7 * r));
        let mut m = Marks::default();
        for p in [a, b] {
            m.soldier_bitten(p, r, 0.0, team, 0.0, true, 0.0, 0.0, 0.0, 0.0);
        }
        for p in [a, b] {
            m.soldier_head(p, r, 0.0, team, 0.0, 0.0, 0.0, 0.0, 0.0);
        }
        let packed = shot(&m);
        let (sx, sy) = cam.to_screen((a.0 + 0.04 * r, a.1));
        let head = packed.get_pixel(sx as u32, sy as u32).0;
        let (lo, hi) = (head[..3].iter().min().copied().unwrap(), head[..3].iter().max().copied().unwrap());
        assert!(lo > 150 && hi - lo < 40, "A's helmet shows over B's shoulders: {head:?}");
        // Alone, the head changes nothing.
        let mut body = Marks::default();
        body.soldier_bitten(a, r, 0.7, team, 0.0, true, 0.0, 0.0, 0.0, 0.0);
        let mut both = body.clone();
        both.soldier_head(a, r, 0.7, team, 0.0, 0.0, 0.0, 0.0, 0.0);
        let (one, two) = (shot(&body), shot(&both));
        let most = one.pixels().zip(two.pixels()).map(|(p, q)| (0..3).map(|c| (p.0[c] as i32 - q.0[c] as i32).abs()).max().unwrap()).max().unwrap();
        assert!(most <= 2, "a lone soldier's head is its own figure's: {most}");
    }

    /// Starfall's and the cores' looks (`Marks::star`, the visual pass,
    /// 2026-11-21): a hot ore is dark rock with molten cracks brighter than
    /// any of its rock, a core is a crimson gem (red over green and blue
    /// in its middle), the dial round them is a ring with the ground
    /// inside it untouched between ring and drop, and nothing reaches past
    /// a mark's box. Against the shader before the shape every pixel is
    /// the ground.
    #[test]
    fn ore_is_rock_with_molten_cracks_and_a_core_is_a_crimson_gem() {
        use crate::world::marks::{STAR_CORE, STAR_DIAL, STAR_ORE};
        let Some((dev, q)) = test_gpu() else { return };
        let (w, h) = (48usize, 32usize);
        let px: Vec<u8> = (0..w * h).flat_map(|_| [40u8, 44, 40, 255]).collect();
        let world = storage(&dev, "world", px.len() as u64);
        q.write_buffer(&world, 0, &px);
        let screen = (576, 384);
        let mut view = View::new(&dev, &q, &world, (w, h), screen);
        let mut cam = Camera::new((w, h), screen);
        cam.zoom_to(12.0, (0.0, 0.0));
        cam.centre_on((24.0, 16.0));
        let (ore, core, r) = ((14.0, 16.0), (34.0, 16.0), 2.5);
        let mut m = Marks::default();
        m.star(ore, r * 2.0, STAR_ORE, 1.0, 7, 0.3, 0.0, [255, 170, 60, 220], [255, 236, 180, 255], false);
        m.star(ore, r + 2.0, STAR_DIAL, 0.25, 7, 0.0, 0.0, [255, 236, 180, 255], [255, 170, 60, 220], false);
        m.star(core, r * 2.2, STAR_CORE, 1.0, 9, 0.5, 1.0, [235, 60, 85, 220], [255, 210, 220, 255], false);
        let mut shot = |m: &Marks| {
            view.set_marks(m);
            let mut enc = dev.create_command_encoder(&Default::default());
            view.record(&mut enc, &cam, None);
            q.submit([enc.finish()]);
            view.picture()
        };
        let bare = shot(&Marks::default());
        let pic = shot(&m);
        let at = |p: (f32, f32)| {
            let (x, y) = cam.to_screen(p);
            pic.get_pixel(x as u32, y as u32).0
        };
        let luma = |c: [u8; 4]| 0.3 * c[0] as f32 + 0.59 * c[1] as f32 + 0.11 * c[2] as f32;
        // The ore: its brightest pixel inside (a crack) well over its
        // median (the rock).
        let (cx, cy) = cam.to_screen(ore);
        let rp = r * 12.0 * 0.8;
        let mut inside: Vec<f32> = Vec::new();
        for y in (cy - rp) as u32..(cy + rp) as u32 {
            for x in (cx - rp) as u32..(cx + rp) as u32 {
                if ((x as f32 - cx).powi(2) + (y as f32 - cy).powi(2)).sqrt() < rp {
                    inside.push(luma(pic.get_pixel(x, y).0));
                }
            }
        }
        inside.sort_by(f32::total_cmp);
        let (mid, top) = (inside[inside.len() / 2], inside[inside.len() - 1]);
        assert!(top > mid + 80.0 && top > 200.0, "molten cracks in the rock: median {mid}, brightest {top}");
        // The core's middle is crimson.
        let c = at(core);
        assert!(c[0] as i32 > c[1] as i32 + 60 && c[0] as i32 > c[2] as i32 + 40, "a crimson gem: {c:?}");
        // The dial: its ring at its radius differs from the ground, and
        // the ground well outside every box is untouched.
        // (Half a dash clockwise from the top: the top is a gap.)
        let (rr, a) = (r + 2.0 - 0.18, std::f32::consts::TAU / 48.0);
        let p = (ore.0 + rr * a.sin(), ore.1 - rr * a.cos());
        let (px, py) = cam.to_screen(p);
        let (ring, under) = (at(p), bare.get_pixel(px as u32, py as u32).0);
        assert!(ring[..3] != under[..3], "the dial's ring is drawn: {ring:?} over {under:?}");
        let (gx, gy) = cam.to_screen((24.0, 4.0));
        assert_eq!(pic.get_pixel(gx as u32, gy as u32).0, bare.get_pixel(gx as u32, gy as u32).0, "nothing past the boxes");
    }

    /// The Armour looks (`Marks::dress_since`, owner 2026-11-19 14:41Z):
    /// three soldiers facing east in olive (A, no Armour), slate (B) and
    /// dark camo (C). B's shoulder is bluer than its red where A's is not;
    /// C's is darker than A's; the helmets stay the same white, so the squad
    /// can still be counted. Against the shader before the Armour looks
    /// every skin draws as sprite A and it fails.
    #[test]
    fn each_armour_look_draws_its_own_body_under_the_same_white_helmet() {
        let Some((dev, q)) = test_gpu() else { return };
        let (w, h) = (48usize, 32usize);
        let px: Vec<u8> = (0..w * h).flat_map(|_| [40u8, 44, 40, 255]).collect();
        let world = storage(&dev, "world", px.len() as u64);
        q.write_buffer(&world, 0, &px);
        let screen = (384, 256);
        let mut view = View::new(&dev, &q, &world, (w, h), screen);
        let mut cam = Camera::new((w, h), screen);
        cam.zoom_to(12.0, (0.0, 0.0));
        cam.centre_on((24.0, 16.0));
        let (r, team) = (2.0, crate::world::marks::team_colour(0));
        let (a, b, c) = ((14.0, 16.0), (24.0, 16.0), (34.0, 16.0));
        let mut m = Marks::default();
        for (p, skin) in [(a, 0), (b, 1), (c, 2)] {
            let n = m.len();
            m.soldier_bitten(p, r, 0.0, team, 0.0, true, 0.0, 0.0, 0.0, 0.0);
            m.soldier_head(p, r, 0.0, team, 0.0, 0.0, 0.0, 0.0, 0.0);
            m.dress_since(n, skin);
        }
        view.set_marks(&m);
        let mut enc = dev.create_command_encoder(&Default::default());
        view.record(&mut enc, &cam, None);
        q.submit([enc.finish()]);
        let pic = view.picture();
        let px_at = |p: (f32, f32)| {
            let (sx, sy) = cam.to_screen(p);
            pic.get_pixel(sx as u32, sy as u32).0.map(i32::from)
        };
        // Half a radius out across the facing: the shoulder.
        let [sa, sb, sc] = [a, b, c].map(|p| px_at((p.0, p.1 + 0.5 * r)));
        assert!(sb[2] > sb[0] && sa[2] + 15 < sa[0], "B slate, A olive: A {sa:?}, B {sb:?}");
        assert!(sc[..3].iter().sum::<i32>() + 40 < sa[..3].iter().sum::<i32>(), "C's dark camo darker than A's olive: A {sa:?}, C {sc:?}");
        let [ha, hb, hc] = [a, b, c].map(|p| px_at((p.0 + 0.04 * r, p.1)));
        let white = |p: [i32; 4]| p[..3].iter().min().copied().unwrap() > 150;
        assert!(white(ha) && (0..3).all(|k| (ha[k] - hb[k]).abs() <= 8 && (ha[k] - hc[k]).abs() <= 8), "the helmets the same white: {ha:?}, {hb:?}, {hc:?}");
    }

    /// Asleep in its own gas (`Marks::doze_since`, 2026-11-19): two
    /// soldiers facing east, B in a deep cloud and out. B's shoulder goes
    /// toward the gas's lavender (bluer than its green, where A's is not);
    /// C, as tinted but not out, is brighter than B (the out figure dims);
    /// the helmets stay the same white, so the squad can still be counted.
    /// With the 11-18 shader B draws as A and it fails.
    #[test]
    fn a_soldier_asleep_in_its_gas_takes_its_lavender_and_keeps_a_white_helmet() {
        let Some((dev, q)) = test_gpu() else { return };
        let (w, h) = (48usize, 32usize);
        let px: Vec<u8> = (0..w * h).flat_map(|_| [40u8, 44, 40, 255]).collect();
        let world = storage(&dev, "world", px.len() as u64);
        q.write_buffer(&world, 0, &px);
        let screen = (384, 256);
        let mut view = View::new(&dev, &q, &world, (w, h), screen);
        let mut cam = Camera::new((w, h), screen);
        cam.zoom_to(12.0, (0.0, 0.0));
        cam.centre_on((24.0, 16.0));
        let (r, team) = (2.0, crate::world::marks::team_colour(0));
        let (a, b, c) = ((14.0, 16.0), (24.0, 16.0), (34.0, 16.0));
        let mut m = Marks::default();
        for (p, tint, out) in [(a, 0.0, false), (b, 1.0, true), (c, 1.0, false)] {
            let n = m.len();
            m.soldier_bitten(p, r, 0.0, team, 0.0, true, 0.0, 0.0, 0.0, 0.0);
            m.soldier_head(p, r, 0.0, team, 0.0, 0.0, 0.0, 0.0, 0.0);
            m.doze_since(n, tint, out);
        }
        view.set_marks(&m);
        let mut enc = dev.create_command_encoder(&Default::default());
        view.record(&mut enc, &cam, None);
        q.submit([enc.finish()]);
        let pic = view.picture();
        let px_at = |p: (f32, f32)| {
            let (sx, sy) = cam.to_screen(p);
            pic.get_pixel(sx as u32, sy as u32).0.map(i32::from)
        };
        // Half a radius out across the facing: the shoulder.
        let [sa, sb, sc] = [a, b, c].map(|p| px_at((p.0, p.1 + 0.5 * r)));
        assert!(sb[2] - sb[1] > sa[2] - sa[1] + 25, "B's shoulder goes lavender: A {sa:?}, B {sb:?}");
        assert!(sc[..3].iter().sum::<i32>() > sb[..3].iter().sum::<i32>() + 30, "out, B dims: B {sb:?}, C {sc:?}");
        let [ha, hb, hc] = [a, b, c].map(|p| px_at((p.0 + 0.04 * r, p.1)));
        assert!((0..3).all(|k| (ha[k] - hb[k]).abs() <= 2 && (ha[k] - hc[k]).abs() <= 2), "the helmets the same white: {ha:?}, {hb:?}, {hc:?}");
    }

    /// The Sprawler's threads (Look B, 2026-11-19): two lone walkers on
    /// bare ground, A heading east and B west. In the plain look each is a
    /// round speck and film (lit on one flank by the key light); as threads
    /// each is a dash along the way it came, so a cell and a half behind it
    /// is lit and as far ahead is not. At zoom 12 and 6. With the plain
    /// look's 11-18 shader the two looks draw the same and it fails.
    #[test]
    fn the_sprawlers_walkers_draw_as_dashes_along_the_way_they_came() {
        let Some((dev, q)) = test_gpu() else { return };
        let (w, h) = (64usize, 40usize);
        let px: Vec<u8> = (0..w * h).flat_map(|_| [12u8, 22, 26, 255]).collect();
        let walk = [16.5f32, 20.5, 0.0, 0.0, 48.5, 20.5, std::f32::consts::PI, 0.0];
        let buf = |label: &str, bytes: &[u8]| {
            let b = storage(&dev, label, bytes.len() as u64);
            q.write_buffer(&b, 0, bytes);
            b
        };
        let world = buf("world", &px);
        let look_b = buf("look", &crate::gpu::u32_bytes(&vec![0u32; w * h]));
        let ground_b = buf("ground", &vec![0u8; w * h * 4]);
        let walk_b = buf("walkers", &walk.iter().flat_map(|x| x.to_le_bytes()).collect::<Vec<u8>>());
        let screen = (384, 240);
        let mut view = View::new(&dev, &q, &world, (w, h), screen);
        view.set_look(&look_b, &ground_b, &walk_b, 2);
        for zoom in [12.0f32, 6.0] {
            let mut cam = Camera::new((w, h), screen);
            cam.zoom_to(zoom, (0.0, 0.0));
            // The summed channels of the 3 x 3 pixels round each point.
            let mut shot = |look: SlimeLook, centre: (f32, f32), ps: [(f32, f32); 2]| {
                view.set_slime_look(look, 2.25);
                cam.centre_on(centre);
                let mut enc = dev.create_command_encoder(&Default::default());
                view.record(&mut enc, &cam, None);
                q.submit([enc.finish()]);
                let pic = view.picture();
                ps.map(|p| {
                    let (sx, sy) = cam.to_screen(p);
                    let mut c = 0i32;
                    for dy in -1..=1 {
                        for dx in -1..=1 {
                            let v = pic.get_pixel((sx as i32 + dx) as u32, (sy as i32 + dy) as u32).0;
                            c += v[0] as i32 + v[1] as i32 + v[2] as i32;
                        }
                    }
                    c / 9
                })
            };
            // Behind, then ahead, a cell and a half from each walker.
            let a = [(15.0, 20.5), (18.0, 20.5)];
            let b = [(50.0, 20.5), (47.0, 20.5)];
            for (name, c, ps) in [("A", (16.5, 20.5), a), ("B", (48.5, 20.5), b)] {
                let [pb, pa] = shot(SlimeLook::Plain, c, ps);
                let [tb, ta] = shot(SlimeLook::Threads, c, ps);
                // Plain, the key light makes one flank brighter (about 70
                // apart, either way); as a thread it is about 300 to 370.
                assert!(tb > ta + 200, "zoom {zoom}: {name} as a thread is lit behind it: behind {tb}, ahead {ta}");
                assert!(tb > pb + 150, "zoom {zoom}: {name}'s dash is brighter behind it than its plain speck: {pb} -> {tb}");
                assert!(ta < pa, "zoom {zoom}: {name}'s film ahead of it thins, as the dash holds one walker: {pa} -> {ta}");
            }
        }
    }

    /// The Grudge's scars (Look B, 2026-11-20): bare ground, its west half
    /// scarred (the engine's damage memory's second block) and its east
    /// half not. With the Grudge's look the scarred ground is darker than
    /// the clean; plain, or with no damage memory bound, the two halves
    /// draw alike. At zoom 6.
    #[test]
    fn the_grudges_scarred_ground_stays_marked() {
        let Some((dev, q)) = test_gpu() else { return };
        let (w, h) = (64usize, 40usize);
        let px: Vec<u8> = (0..w * h).flat_map(|_| [12u8, 22, 26, 255]).collect();
        let buf = |label: &str, bytes: &[u8]| {
            let b = storage(&dev, label, bytes.len() as u64);
            q.write_buffer(&b, 0, bytes);
            b
        };
        let world = buf("world", &px);
        let look_b = buf("look", &crate::gpu::u32_bytes(&vec![0u32; w * h]));
        let ground_b = buf("ground", &vec![0u8; w * h * 4]);
        let walk_b = buf("walkers", &[0u8; 16]);
        let mut crater = vec![0f32; 2 * w * h];
        for y in 0..h {
            for x in 0..w / 2 {
                crater[w * h + y * w + x] = 5.0;
            }
        }
        let crater_b = buf("crater", &crater.iter().flat_map(|x| x.to_le_bytes()).collect::<Vec<u8>>());
        let screen = (384, 240);
        let mut view = View::new(&dev, &q, &world, (w, h), screen);
        view.set_look(&look_b, &ground_b, &walk_b, 0);
        let mut cam = Camera::new((w, h), screen);
        cam.zoom_to(6.0, (0.0, 0.0));
        cam.centre_on((32.0, 20.0));
        // The summed channels of the 5 x 5 pixels round each point.
        let shot = |view: &mut View, look: SlimeLook| {
            view.set_slime_look(look, 0.0);
            let mut enc = dev.create_command_encoder(&Default::default());
            view.record(&mut enc, &cam, None);
            q.submit([enc.finish()]);
            let pic = view.picture();
            [(20.0f32, 20.0f32), (44.0, 20.0)].map(|p| {
                let (sx, sy) = cam.to_screen(p);
                let mut c = 0i32;
                for dy in -2..=2 {
                    for dx in -2..=2 {
                        let v = pic.get_pixel((sx as i32 + dx) as u32, (sy as i32 + dy) as u32).0;
                        c += v[0] as i32 + v[1] as i32 + v[2] as i32;
                    }
                }
                c / 25
            })
        };
        let [ps, pc] = shot(&mut view, SlimeLook::Scarred);
        // The ground's own grain parts the two spots by a few levels.
        assert!((ps - pc).abs() <= 6, "no damage memory bound: the halves draw alike ({ps}, {pc})");
        view.set_scar(Some(&crater_b));
        let [gs, gc] = shot(&mut view, SlimeLook::Scarred);
        assert!(gc - gs > pc - ps + 15, "the Grudge's scarred ground is darker: scarred {gs}, clean {gc} (unbound {ps}, {pc})");
        let [ls, lc] = shot(&mut view, SlimeLook::Plain);
        assert!((ls - ps).abs() <= 1 && (lc - pc).abs() <= 1, "plain, the scar draws nothing ({ls}, {lc})");
    }

    /// A Mind status on the slime it holds (Look A5, 2026-11-18): two
    /// pieces of gold slime, A's walkers with no status and B's under one
    /// (a Mind word: kind 3, spin, ending 200 steps on); B's middle turns
    /// toward the family's rose and A's keeps its gold. The same word run
    /// out (ending at the step now) draws nothing, and with 30 steps left
    /// of a 60-step fade B is tinted less. At zoom 12 and 6. Against the
    /// 11-18 shader (the two words unread) B stays gold.
    #[test]
    fn slime_under_a_mind_status_turns_rose_and_fades_as_it_runs_out() {
        let Some((dev, q)) = test_gpu() else { return };
        let (w, h) = (64usize, 40usize);
        let mut px: Vec<u8> = (0..w * h).flat_map(|_| [12u8, 22, 26, 255]).collect();
        let (mut look, mut walk) = (vec![0u32; w * h], Vec::new());
        for cx in [16usize, 48] {
            for dy in 0..7 {
                for dx in 0..7 {
                    let (x, y) = (cx + dx - 3, 20 + dy - 3);
                    look[y * w + x] = 255 | (1 << 8);
                    px[4 * (y * w + x)..4 * (y * w + x) + 3].copy_from_slice(&[255, 190, 90]);
                    walk.extend([x as f32 + 0.5, y as f32 + 0.5, 0.0, 0.0]);
                }
            }
        }
        let n = walk.len() / 4;
        let buf = |label: &str, bytes: &[u8]| {
            let b = storage(&dev, label, bytes.len() as u64);
            q.write_buffer(&b, 0, bytes);
            b
        };
        let world = buf("world", &px);
        let look_b = buf("look", &crate::gpu::u32_bytes(&look));
        let ground_b = buf("ground", &vec![0u8; w * h * 4]);
        let walk_b = buf("walkers", &walk.iter().flat_map(|x| x.to_le_bytes()).collect::<Vec<u8>>());
        // The Mind words after 16 other words; the step now is 1000.
        let (at, now) = (16usize, 1000u32);
        let words = |end: u32| {
            let mut v = vec![0u32; at + n];
            for k in n / 2..n {
                v[at + k] = (3 << 29) | end;
            }
            buf("news", &crate::gpu::u32_bytes(&v))
        };
        let state = buf("state", &crate::gpu::u32_bytes(&[now, 0, 0, 0]));
        let screen = (384, 240);
        let mut view = View::new(&dev, &q, &world, (w, h), screen);
        view.set_look(&look_b, &ground_b, &walk_b, n as u32);
        let (a, b) = ((16.5f32, 20.5f32), (48.5f32, 20.5f32));
        for zoom in [12.0f32, 6.0] {
            let mut cam = Camera::new((w, h), screen);
            cam.zoom_to(zoom, (0.0, 0.0));
            let mut shot = |centre: (f32, f32), news: Option<&wgpu::Buffer>, p: (f32, f32)| {
                view.set_minds(news.map(|n| (n, &state)), at as u32, 60);
                cam.centre_on(centre);
                let mut enc = dev.create_command_encoder(&Default::default());
                view.record(&mut enc, &cam, None);
                q.submit([enc.finish()]);
                let pic = view.picture();
                // The mean of the 5 x 5 pixels round the point.
                let (sx, sy) = cam.to_screen(p);
                let mut c = [0i32; 3];
                for dy in -2..=2 {
                    for dx in -2..=2 {
                        let v = pic.get_pixel((sx as i32 + dx) as u32, (sy as i32 + dy) as u32).0;
                        for k in 0..3 {
                            c[k] += v[k] as i32;
                        }
                    }
                }
                c.map(|v| v / 25)
            };
            let far = |p: [i32; 3], q: [i32; 3]| (0..3).map(|k| (p[k] - q[k]).abs()).max().unwrap();
            let (on, out, half) = (words(now + 200), words(now), words(now + 30));
            let (b0, b1) = (shot(b, None, b), shot(b, Some(&on), b));
            assert!(b0[2] + 60 < b0[1], "zoom {zoom}: the slime is gold: {b0:?}");
            assert!(b1[2] > b0[2] + 50 && b1[1] < b0[1], "zoom {zoom}: B, under a status, turns rose: {b0:?} -> {b1:?}");
            let (a0, a1) = (shot(a, None, a), shot(a, Some(&on), a));
            assert!(far(a0, a1) <= 2, "zoom {zoom}: A, with no status, keeps its gold: {a0:?} -> {a1:?}");
            let b2 = shot(b, Some(&out), b);
            assert!(far(b0, b2) <= 2, "zoom {zoom}: a status run out draws nothing: {b0:?} -> {b2:?}");
            let b3 = shot(b, Some(&half), b);
            assert!(b3[2] > b0[2] + 10 && b3[2] + 10 < b1[2], "zoom {zoom}: half way through its fade, less rose: {b0:?}, {b3:?}, {b1:?}");
        }
    }

    /// The slime biting a soldier (the owner, 2026-11-18 18:08Z: a bite
    /// "just looks like more of the same slime"): two pieces of gold slime,
    /// A and B, a soldier standing in each, and a bite zone (`Light::bite`,
    /// the soldier's touch) on B alone and on bare ground between them. B's
    /// slime turns crimson and shows over its soldier's shoulder; A's slime
    /// and soldier, and the bare ground in its zone, are unchanged. At zoom
    /// 12 and 6. Against the 11-18 shader (no bite zone) B stays gold and
    /// under its figure.
    #[test]
    fn slime_biting_a_soldier_turns_crimson_over_it_and_nothing_else_changes() {
        let Some((dev, q)) = test_gpu() else { return };
        let (w, h) = (64usize, 40usize);
        let mut px: Vec<u8> = (0..w * h).flat_map(|_| [12u8, 22, 26, 255]).collect();
        let (mut look, mut walk) = (vec![0u32; w * h], Vec::new());
        for cx in [16usize, 48] {
            for dy in 0..7 {
                for dx in 0..7 {
                    let (x, y) = (cx + dx - 3, 20 + dy - 3);
                    look[y * w + x] = 255 | (1 << 8);
                    px[4 * (y * w + x)..4 * (y * w + x) + 3].copy_from_slice(&[255, 190, 90]);
                    walk.extend([x as f32 + 0.5, y as f32 + 0.5, 0.0, 0.0]);
                }
            }
        }
        let buf = |label: &str, bytes: &[u8]| {
            let b = storage(&dev, label, bytes.len() as u64);
            q.write_buffer(&b, 0, bytes);
            b
        };
        let world = buf("world", &px);
        let look_b = buf("look", &crate::gpu::u32_bytes(&look));
        let ground_b = buf("ground", &vec![0u8; w * h * 4]);
        let walk_b = buf("walkers", &walk.iter().flat_map(|x| x.to_le_bytes()).collect::<Vec<u8>>());
        let screen = (384, 240);
        let mut view = View::new(&dev, &q, &world, (w, h), screen);
        view.set_look(&look_b, &ground_b, &walk_b, walk.len() as u32 / 4);
        let (r, team) = (2.0f32, crate::world::marks::team_colour(0));
        let (a, b, bare) = ((16.5f32, 20.5f32), (48.5f32, 20.5f32), (32.5f32, 20.5f32));
        let mut soldiers = Marks::default();
        for p in [a, b] {
            soldiers.soldier_bitten(p, r, 0.0, team, 0.0, true, 0.0, 0.0, 0.0, 0.0);
        }
        let zones = [Light::bite(b, r / 1.15, 1.0), Light::bite(bare, r / 1.15, 1.0)];
        for zoom in [12.0f32, 6.0] {
            let mut cam = Camera::new((w, h), screen);
            cam.zoom_to(zoom, (0.0, 0.0));
            let mut shot = |centre: (f32, f32), marks: &Marks, lights: &[Light]| {
                cam.centre_on(centre);
                view.set_marks(marks);
                view.set_lights(lights, (0.0, 0.0, w as f32, h as f32));
                let mut enc = dev.create_command_encoder(&Default::default());
                view.record(&mut enc, &cam, None);
                q.submit([enc.finish()]);
                (view.picture(), cam)
            };
            // The mean colour of the 3 x 3 pixels round a world point.
            let at = |(pic, cam): &(image::RgbaImage, Camera), p: (f32, f32)| {
                let (sx, sy) = cam.to_screen(p);
                let mut c = [0i32; 3];
                for dy in -1..=1 {
                    for dx in -1..=1 {
                        let v = pic.get_pixel((sx as i32 + dx) as u32, (sy as i32 + dy) as u32).0;
                        for k in 0..3 {
                            c[k] += v[k] as i32;
                        }
                    }
                }
                c.map(|v| v / 9)
            };
            let far = |p: [i32; 3], q: [i32; 3]| (0..3).map(|k| (p[k] - q[k]).abs()).max().unwrap();
            // Half a radius out across the facing: the soldier's shoulder,
            // inside its touch.
            let shoulder = |p: (f32, f32)| (p.0, p.1 + 0.5 * r);
            let none = Marks::default();
            // The slime alone, then with the soldiers.
            for (marks, what) in [(&none, "slime"), (&soldiers, "shoulder")] {
                let spot = |p: (f32, f32)| if what == "slime" { p } else { shoulder(p) };
                let (b0, b1) = (at(&shot(b, marks, &[]), spot(b)), at(&shot(b, marks, &zones), spot(b)));
                assert!(b1[0] > b1[1] + 80 && b1[0] > b1[2] + 40, "zoom {zoom}: B's biting {what} is crimson: {b0:?} -> {b1:?}");
                let (a0, a1) = (at(&shot(a, marks, &[]), spot(a)), at(&shot(a, marks, &zones), spot(a)));
                assert!(far(a0, a1) <= 2, "zoom {zoom}: A's {what}, not bitten, is unchanged: {a0:?} -> {a1:?}");
            }
            let (g0, g1) = (at(&shot(bare, &none, &[]), bare), at(&shot(bare, &none, &zones), bare));
            assert!(far(g0, g1) <= 2, "zoom {zoom}: bare ground in a bite zone is unchanged: {g0:?} -> {g1:?}");
        }
    }

    /// A fallen soldier (the owner's playtest, 2026-11-18 16:12Z: "When
    /// soliders die, the body on the floor looks really bad") lies in our
    /// olive over a pool of blood that spreads after the fall: beside its
    /// chest the bare ground the moment it falls and blood once the pool
    /// has spread, and the chest olive. Against the 11-14 corpse (a cold
    /// grey figure, no pool) it fails on the blood. In the slate Armour
    /// look (`Marks::dress_since`, 2026-11-19) the chest is slate.
    #[test]
    fn a_fallen_soldier_lies_olive_in_a_pool_of_blood_that_spreads() {
        let Some((dev, q)) = test_gpu() else { return };
        let (w, h) = (48usize, 32usize);
        let px: Vec<u8> = (0..w * h).flat_map(|_| [90u8, 90, 90, 255]).collect();
        let world = storage(&dev, "world", px.len() as u64);
        q.write_buffer(&world, 0, &px);
        let screen = (384, 256);
        let mut view = View::new(&dev, &q, &world, (w, h), screen);
        let mut cam = Camera::new((w, h), screen);
        cam.zoom_to(12.0, (0.0, 0.0));
        cam.centre_on((24.0, 16.0));
        // Facing east (0 radians): the body's frame is the screen's.
        let r = 2.0;
        let mut shot = |pool: f32, skin: u32| {
            let mut m = Marks::default();
            m.fallen((24.0, 16.0), r, 0.0, crate::world::marks::team_colour(0), pool, 7);
            m.dress_since(0, skin);
            view.set_marks(&m);
            let mut enc = dev.create_command_encoder(&Default::default());
            view.record(&mut enc, &cam, None);
            q.submit([enc.finish()]);
            view.picture()
        };
        let (fresh, spread, slate) = (shot(0.0, 0), shot(1.0, 0), shot(1.0, 1));
        // A point in radii of the body's frame (+x the way it faced).
        let at = |pic: &image::RgbaImage, bx: f32, by: f32| {
            let (sx, sy) = cam.to_screen((24.0 + bx * r, 16.0 + by * r));
            pic.get_pixel(sx as u32, sy as u32).0
        };
        let ground = at(&fresh, -5.0, 0.0);
        let then = at(&fresh, -0.3, -0.6);
        assert!((0..3).all(|c| (then[c] as i32 - ground[c] as i32).abs() <= 4), "the ground beside its chest as it falls: {then:?}, the ground {ground:?}");
        let beside = at(&spread, -0.3, -0.6);
        assert!(beside[0] as i32 > beside[1] as i32 + 60 && beside[0] as i32 > beside[2] as i32 + 60, "blood there once spread: {beside:?}");
        let chest = at(&spread, -0.3, 0.0);
        assert!(chest[1] as i32 > chest[2] as i32 + 20 && chest[1] as i32 + 10 >= chest[0] as i32, "the chest our olive: {chest:?}");
        let cloth = at(&slate, -0.3, 0.0);
        assert!(cloth[2] as i32 > cloth[1] as i32 && cloth[2] as i32 > cloth[0] as i32 + 10, "the chest slate in slate Armour: {cloth:?}");
    }

    /// A shot's flung droplet (Look A3, 2026-11-18) is a bead in the
    /// slime's material over the bright body: lit on the side toward the
    /// key light and darker on the far side, in the slime's gold rather
    /// than a pale chip, with no ink rim on its lit side, and a shadow on
    /// the body beside its far side. Against the 11-15 shader (a flat chip
    /// in a dark rim) it fails on the first: its two sides 241 and 243.
    #[test]
    fn a_flung_droplet_is_a_lit_bead_with_a_shadow_and_no_rim() {
        let Some((dev, q)) = test_gpu() else { return };
        let (w, h) = (48usize, 40usize);
        let px: Vec<u8> = (0..w * h).flat_map(|_| [200u8, 160, 60, 255]).collect();
        let world = storage(&dev, "world", px.len() as u64);
        q.write_buffer(&world, 0, &px);
        // A body of 4 walkers a cell everywhere, and over it one droplet a
        // shot's spray throws (the fourth word below 0: a killed walker's
        // flight a tick, 0.8 cells, fast) flying toward +x.
        let (mut look, mut walk) = (vec![0u32; w * h], Vec::new());
        for y in 0..h {
            for x in 0..w {
                look[y * w + x] = 255 | (4 << 8);
                for k in 0..4 {
                    walk.extend([x as f32 + 0.2 + 0.2 * k as f32, y as f32 + 0.3 + 0.13 * k as f32, 0.0, 0.0]);
                }
            }
        }
        walk.extend([24.5f32, 20.5, 0.0, -0.8]);
        let buf = |label: &str, bytes: &[u8]| {
            let b = storage(&dev, label, bytes.len() as u64);
            q.write_buffer(&b, 0, bytes);
            b
        };
        let look_b = buf("look", &crate::gpu::u32_bytes(&look));
        let ground = buf("ground", &vec![0u8; w * h * 4]);
        let walk_b = buf("walkers", &walk.iter().flat_map(|v| v.to_le_bytes()).collect::<Vec<u8>>());
        let screen = (384, 240);
        let mut view = View::new(&dev, &q, &world, (w, h), screen);
        view.set_look(&look_b, &ground, &walk_b, (walk.len() / 4) as u32);
        let mut cam = Camera::new((w, h), screen);
        let zoom = 12.0;
        cam.zoom_to(zoom, (0.0, 0.0));
        cam.centre_on((24.0, 20.0));
        let mut enc = dev.create_command_encoder(&Default::default());
        view.record(&mut enc, &cam, None);
        q.submit([enc.finish()]);
        let pic = view.picture();
        // The pixel (dx, dy) screen pixels from the droplet's head, and the
        // one `d` pixels from it along (dx, dy).
        let (hx, hy) = cam.to_screen((24.5, 20.5));
        let off = |dx: f32, dy: f32| pic.get_pixel((hx + dx) as u32, (hy + dy) as u32).0.map(|v| v as f32);
        let at = |dx: f32, dy: f32, d: f32| {
            let n = dx.hypot(dy).max(1e-6);
            off(dx / n * d, dy / n * d)
        };
        let lum = |p: [f32; 4]| 0.299 * p[0] + 0.587 * p[1] + 0.114 * p[2];
        // The key light (`LIGHT`) comes from the upper left.
        let (lx, ly) = (-0.42f32, -0.58f32);
        let rh = zoom / 3.2;
        let (lit, far) = (lum(at(lx, ly, 0.6 * rh)), lum(at(-lx, -ly, 0.6 * rh)));
        assert!(lit > far + 15.0, "a bead lit toward the light: {lit} against its far side {far}");
        let core = at(0.0, 0.0, 0.0);
        assert!(core[2] < 0.6 * core[0], "the slime's gold, not a pale chip: {core:?}");
        // The body itself, well clear of the droplet and its shadow: the
        // median of a row of pixels above it (the body has its own sheen).
        let mut row: Vec<f32> = (0..24).map(|k| lum(off(k as f32 - 12.0, -3.0 * rh))).collect();
        row.sort_by(f32::total_cmp);
        let body = row[row.len() / 2];
        // Just past the head straight up (the tail lies along -x): the 11-15
        // rim drew there, a pixel wide.
        let above = lum(at(0.0, -1.0, rh + 0.6));
        assert!(above > 0.75 * body, "no ink rim on the lit side: {above} against the body {body}");
        let shade = lum(at(-lx, -ly, rh + 1.2));
        assert!(shade < 0.8 * body, "a shadow on the body beside its far side: {shade} against the body {body}");
    }
}
