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

/// Bytes in the uniform (20 words).
const PARAMS_BYTES: usize = 128;
/// Cells round the shown ones that the look's samples reach: the slope
/// (1.5), the cast shadow (up to 1.8 toward the light) and the bilinear
/// blend (1) all fit in 4.
const MARGIN: u32 = 4;
/// The bindings after the first twenty, in `bind`'s order: 20 the lights,
/// then 21-24 the crystals' live charge, their heat (`drain`), the terrain
/// cells (`tcell`) and the crystal cells listed.
const BINDINGS_AFTER: [u32; 5] = [LIGHTS_BINDING, 21, 22, 23, 24];

/// The most point lights a frame (`View::set_lights`); `shaded` loops over
/// them a pixel, so the cap bounds that loop.
pub const MAX_LIGHTS: usize = 64;
/// The lights' binding: 15-19 are the graphics thread's (#212 and its
/// crystal work), so the lights sit after them.
const LIGHTS_BINDING: u32 = 20;

/// A point light (`View::set_lights`): something in the game that gives
/// off light, lighting the world's own relief. `at` and `reach` in world
/// cells, `lift` its height above the ground in cells, `colour` 0-255 and
/// `strength` (1 about a lamp's worth at its centre).
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct Light {
    pub at: (f32, f32),
    pub reach: f32,
    pub lift: f32,
    pub colour: [f32; 3],
    pub strength: f32,
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
    specks: wgpu::Buffer,
    /// Each walker's energy, when the world grows (`set_alive`): a slot
    /// below 0 is empty or dead and draws no speck.
    alive: wgpu::Buffer,
    /// This frame's point lights (`set_lights`), two vec4s each.
    lights: wgpu::Buffer,
    n_lights: u32,
    lives: bool,
    /// Food's news (`set_news`): the engine's news (the step each walker
    /// last heard of food) and state (the step now) buffers, and the
    /// engine's lag and lean in steps.
    news: wgpu::Buffer,
    state: wgpu::Buffer,
    /// The walkers a fine cell of `prep`'s rectangle (and, above bit 16,
    /// those cut off from food), counted each frame by `draw_specks`.
    fgrid: wgpu::Buffer,
    news_on: bool,
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
    has_overlay: bool,
    has_vis: bool,
    n_marks: u32,
}

fn storage(dev: &wgpu::Device, label: &str, bytes: u64) -> wgpu::Buffer {
    dev.create_buffer(&wgpu::BufferDescriptor { label: Some(label), size: bytes.max(16), usage: wgpu::BufferUsages::STORAGE | wgpu::BufferUsages::COPY_DST | wgpu::BufferUsages::COPY_SRC, mapped_at_creation: false })
}

impl View {
    /// A view of `world_frame` (`world.0 x world.1` RGBA `u32`s) on a screen
    /// `screen` pixels.
    pub fn new(device: &wgpu::Device, queue: &wgpu::Queue, world_frame: &wgpu::Buffer, world: (usize, usize), screen: (u32, u32)) -> Self {
        let module = device.create_shader_module(wgpu::ShaderModuleDescriptor { label: Some("view"), source: wgpu::ShaderSource::Wgsl(SHADER.into()) });
        let entry = |binding: u32, ty: wgpu::BufferBindingType| wgpu::BindGroupLayoutEntry {
            binding,
            visibility: wgpu::ShaderStages::COMPUTE,
            ty: wgpu::BindingType::Buffer { ty, has_dynamic_offset: false, min_binding_size: None },
            count: None,
        };
        let ro = wgpu::BufferBindingType::Storage { read_only: true };
        let rw = wgpu::BufferBindingType::Storage { read_only: false };
        let entries = [entry(0, wgpu::BufferBindingType::Uniform), entry(1, ro), entry(2, ro), entry(3, ro), entry(4, rw), entry(5, ro), entry(6, rw), entry(7, ro), entry(8, ro), entry(9, ro), entry(10, rw), entry(11, ro), entry(12, rw), entry(13, rw), entry(14, ro), entry(15, ro), entry(16, ro), entry(17, rw), entry(18, ro), entry(19, ro), entry(LIGHTS_BINDING, ro), entry(21, ro), entry(22, rw), entry(23, rw), entry(24, ro)];
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
        let specks = storage(device, "specks", stride as u64 * screen.1 as u64 * 4);
        let atlas = Self::atlas(device, queue);
        let (hcell, mcell, tcell) = Self::cell_buffers(device, screen);
        let alive = storage(device, "alive", 16);
        let lights = storage(device, "lights", (MAX_LIGHTS * 32) as u64);
        let (news, state) = (storage(device, "news", 16), storage(device, "state", 16));
        let fgrid = Self::fine_buffer(device, screen);
        let (gems, gem_cell) = (storage(device, "crystals", 16), storage(device, "crystal cells", 16));
        let (charge, heat, gem_list) = (storage(device, "charge", 16), storage(device, "crystal heat", 16), storage(device, "crystal list", 16));
        let group = Self::bind(device, &bgl, [&params, world_frame, &overlay, &marks, &out, &vis, &top, &look, &ground, &walkers, &specks, &atlas, &hcell, &mcell, &alive, &news, &state, &fgrid, &gems, &gem_cell, &lights, &charge, &heat, &tcell, &gem_list]);
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
            news,
            state,
            fgrid,
            news_on: false,
            lag: 0,
            lean: 0,
            gems,
            gem_cell,
            n_gems: 0,
            charge,
            heat,
            gem_list,
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
            has_overlay: false,
            has_vis: false,
            n_marks: 0,
        }
    }

    /// The soldier sprite atlas on the GPU: `SPRITES` decoded, a u32 RGBA
    /// a texel, 2176 x 160 (17 tiles).
    fn atlas(dev: &wgpu::Device, queue: &wgpu::Queue) -> wgpu::Buffer {
        let img = image::load_from_memory(SPRITES).expect("sprites.png").to_rgba8();
        assert_eq!(img.dimensions(), (2176, 160), "the atlas the view's shader expects");
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
    fn cell_buffers(dev: &wgpu::Device, screen: (u32, u32)) -> (wgpu::Buffer, wgpu::Buffer, wgpu::Buffer) {
        let n = (screen.0 + 2 * MARGIN + 2) as u64 * (screen.1 + 2 * MARGIN + 2) as u64;
        (storage(dev, "cell heights", n * 4), storage(dev, "cell materials", n * 8), storage(dev, "cell terrain", n * 4))
    }

    /// `prep`'s fine cells, at most as many as `cell_buffers`' cells: the
    /// rectangle is the screen over the zoom and a margin, `fine` (at most
    /// a third of the zoom) cells each way.
    fn fine_buffer(dev: &wgpu::Device, screen: (u32, u32)) -> wgpu::Buffer {
        let n = (screen.0 + 2 * MARGIN + 2) as u64 * (screen.1 + 2 * MARGIN + 2) as u64;
        storage(dev, "fine walkers", n * 4)
    }

    /// Fine cells a cell each way at zoom `zoom` (`prep`; 2026-11-14): one
    /// below zoom 6, else 2. Four (from zoom 12) made the 9 x 9 window too
    /// small for a Gaussian wide enough not to look lumpy: square blobs.
    pub fn fine(zoom: f32) -> u32 {
        ((zoom / 3.0).floor() as u32).clamp(1, 2)
    }

    /// The bind group: `bufs` in binding order, 0-19 then `BINDINGS_AFTER`.
    fn bind(dev: &wgpu::Device, bgl: &wgpu::BindGroupLayout, bufs: [&wgpu::Buffer; 25]) -> wgpu::BindGroup {
        let binding = |i: usize| if i < 20 { i as u32 } else { BINDINGS_AFTER[i - 20] };
        let entries: Vec<wgpu::BindGroupEntry> = bufs.iter().enumerate().map(|(i, b)| wgpu::BindGroupEntry { binding: binding(i), resource: b.as_entire_binding() }).collect();
        dev.create_bind_group(&wgpu::BindGroupDescriptor { label: Some("view"), layout: bgl, entries: &entries })
    }

    fn rebind(&mut self) {
        self.group = Self::bind(&self.device, &self.bgl, [&self.params, &self.world_frame, &self.overlay, &self.marks, &self.out, &self.vis, &self.top, &self.look, &self.ground, &self.walkers, &self.specks, &self.atlas, &self.hcell, &self.mcell, &self.alive, &self.news, &self.state, &self.fgrid, &self.gems, &self.gem_cell, &self.lights, &self.charge, &self.heat, &self.tcell, &self.gem_list]);
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

    fn clear_charge(&mut self) {
        if self.live.get() != 0 {
            (self.charge, self.heat) = (storage(&self.device, "charge", 16), storage(&self.device, "crystal heat", 16));
            self.live.set(0);
        }
    }

    /// The game's time in seconds, each frame (the crystals' `drain`
    /// measures how fast charge goes in game time, so a paused or fast game
    /// reads the same).
    pub fn set_time(&mut self, t: f32) {
        self.game_t = t;
    }

    fn clear_news(&mut self) {
        if self.news_on {
            (self.news, self.state) = (storage(&self.device, "news", 16), storage(&self.device, "state", 16));
            self.news_on = false;
        }
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
        self.specks = storage(&self.device, "specks", stride as u64 * screen.1 as u64 * 4);
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

    /// This frame's point lights (past `MAX_LIGHTS` the rest are dropped):
    /// they light the shaded world (`view.wgsl`, `shaded`), so only with
    /// the look on.
    pub fn set_lights(&mut self, lights: &[Light]) {
        let n = lights.len().min(MAX_LIGHTS);
        if n > 0 {
            let words: Vec<f32> = lights[..n].iter().flat_map(|l| [l.at.0, l.at.1, l.reach, l.lift, l.colour[0], l.colour[1], l.colour[2], l.strength]).collect();
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
        if !self.minimap || cam.sees_all() {
            return None;
        }
        match self.minimap_box {
            Some(b) => Some(minimap_in(self.world, b)),
            None => minimap_rect(self.world, self.screen),
        }
    }

    /// Records the passes for camera `cam` (after the engine's draw).
    /// With `stamp`, timestamps at the first pass's start and the last
    /// pass's end, at the indices given.
    pub fn record(&self, enc: &mut wgpu::CommandEncoder, cam: &Camera, stamp: Option<(&wgpu::QuerySet, Option<u32>, Option<u32>)>) {
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
            let writes = stamp
                .map(|(qs, begin, end)| wgpu::ComputePassTimestampWrites { query_set: qs, beginning_of_pass_write_index: begin.filter(|_| q == 0), end_of_pass_write_index: end.filter(|_| q == last) })
                .filter(|w| w.beginning_of_pass_write_index.is_some() || w.end_of_pass_write_index.is_some());
            let mut pass = enc.begin_compute_pass(&wgpu::ComputePassDescriptor { label: Some("view"), timestamp_writes: writes });
            pass.set_pipeline(&self.pipes[p]);
            pass.set_bind_group(0, &self.group, &[]);
            pass.dispatch_workgroups(x, y, 1);
        }
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
}
