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
const PARAMS_BYTES: usize = 96;
/// Cells round the shown ones that the look's samples reach: the slope
/// (1.5), the cast shadow (up to 1.8 toward the light) and the bilinear
/// blend (1) all fit in 4.
const MARGIN: u32 = 4;

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
    pipes: [wgpu::ComputePipeline; 5],
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
    /// The soldier sprites (`SPRITES`), RGBA a texel.
    atlas: wgpu::Buffer,
    /// Each shown cell's height and material, worked out once a frame by
    /// the `prep` pass (`view.wgsl`), sized for the screen at zoom 1.
    hcell: wgpu::Buffer,
    mcell: wgpu::Buffer,
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
        let entries = [entry(0, wgpu::BufferBindingType::Uniform), entry(1, ro), entry(2, ro), entry(3, ro), entry(4, rw), entry(5, ro), entry(6, rw), entry(7, ro), entry(8, ro), entry(9, ro), entry(10, rw), entry(11, ro), entry(12, rw), entry(13, rw)];
        let bgl = device.create_bind_group_layout(&wgpu::BindGroupLayoutDescriptor { label: Some("view"), entries: &entries });
        let layout = device.create_pipeline_layout(&wgpu::PipelineLayoutDescriptor { label: Some("view"), bind_group_layouts: &[Some(&bgl)], immediate_size: 0 });
        let pipe = |entry_point: &str| {
            device.create_compute_pipeline(&wgpu::ComputePipelineDescriptor { label: Some(entry_point), layout: Some(&layout), module: &module, entry_point: Some(entry_point), compilation_options: Default::default(), cache: None })
        };
        let pipes = [pipe("base"), pipe("draw_marks"), pipe("compose"), pipe("draw_specks"), pipe("prep")];
        let params = device.create_buffer(&wgpu::BufferDescriptor { label: Some("view params"), size: PARAMS_BYTES as u64, usage: wgpu::BufferUsages::UNIFORM | wgpu::BufferUsages::COPY_DST, mapped_at_creation: false });
        let marks = storage(device, "marks", (MAX_MARKS * MARK_WORDS * 4) as u64);
        let vis = storage(device, "visibility", 16);
        let (stride, out, overlay, top) = Self::screen_buffers(device, screen);
        let (look, ground, walkers) = (storage(device, "look", 16), storage(device, "ground", 16), storage(device, "walkers", 16));
        let specks = storage(device, "specks", stride as u64 * screen.1 as u64 * 4);
        let atlas = Self::atlas(device, queue);
        let (hcell, mcell) = Self::cell_buffers(device, screen);
        let group = Self::bind(device, &bgl, [&params, world_frame, &overlay, &marks, &out, &vis, &top, &look, &ground, &walkers, &specks, &atlas, &hcell, &mcell]);
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
            atlas,
            hcell,
            mcell,
            n_walk: 0,
            shade: false,
            out,
            top,
            group,
            world,
            screen,
            stride,
            minimap: true,
            has_overlay: false,
            has_vis: false,
            n_marks: 0,
        }
    }

    /// The soldier sprite atlas on the GPU: `SPRITES` decoded, a u32 RGBA
    /// a texel, 512 x 160.
    fn atlas(dev: &wgpu::Device, queue: &wgpu::Queue) -> wgpu::Buffer {
        let img = image::load_from_memory(SPRITES).expect("sprites.png").to_rgba8();
        assert_eq!(img.dimensions(), (512, 160), "the atlas the view's shader expects");
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
    fn cell_buffers(dev: &wgpu::Device, screen: (u32, u32)) -> (wgpu::Buffer, wgpu::Buffer) {
        let n = (screen.0 + 2 * MARGIN + 2) as u64 * (screen.1 + 2 * MARGIN + 2) as u64;
        (storage(dev, "cell heights", n * 4), storage(dev, "cell materials", n * 8))
    }

    fn bind(dev: &wgpu::Device, bgl: &wgpu::BindGroupLayout, bufs: [&wgpu::Buffer; 14]) -> wgpu::BindGroup {
        let entries: Vec<wgpu::BindGroupEntry> = bufs.iter().enumerate().map(|(i, b)| wgpu::BindGroupEntry { binding: i as u32, resource: b.as_entire_binding() }).collect();
        dev.create_bind_group(&wgpu::BindGroupDescriptor { label: Some("view"), layout: bgl, entries: &entries })
    }

    fn rebind(&mut self) {
        self.group = Self::bind(&self.device, &self.bgl, [&self.params, &self.world_frame, &self.overlay, &self.marks, &self.out, &self.vis, &self.top, &self.look, &self.ground, &self.walkers, &self.specks, &self.atlas, &self.hcell, &self.mcell]);
    }

    /// A new world frame (a restart made a new engine).
    pub fn set_world(&mut self, world_frame: &wgpu::Buffer, world: (usize, usize)) {
        self.world_frame = world_frame.clone();
        self.world = world;
        if self.has_vis {
            self.set_visibility(None);
        }
        self.clear_look();
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
        (self.hcell, self.mcell) = Self::cell_buffers(&self.device, screen);
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
        minimap_rect(self.world, self.screen)
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
        for v in [cam.x, cam.y, cam.zoom, 0.0] {
            b.extend(v.to_le_bytes());
        }
        for v in [mm.0, mm.1, mm.2, mm.3, self.shade as u32, self.n_walk, 0, 0] {
            b.extend(v.to_le_bytes());
        }
        // The shown cells and a margin: what `prep` works out.
        let (ox, oy) = (cam.x.floor() as i32 - MARGIN as i32, cam.y.floor() as i32 - MARGIN as i32);
        let zoom = cam.zoom.max(1.0);
        let (rw, rh) = ((self.screen.0 as f32 / zoom).ceil() as u32 + 2 * MARGIN + 2, (self.screen.1 as f32 / zoom).ceil() as u32 + 2 * MARGIN + 2);
        b.extend(ox.to_le_bytes());
        b.extend(oy.to_le_bytes());
        b.extend(rw.to_le_bytes());
        b.extend(rh.to_le_bytes());
        self.queue.write_buffer(&self.params, 0, &b);
        let (gx, gy) = (self.screen.0.div_ceil(16), self.screen.1.div_ceil(16));
        let mut passes: Vec<(usize, (u32, u32))> = Vec::new();
        if self.shade && self.n_walk > 0 && cam.zoom >= 1.0 {
            passes.push((3, (self.n_walk.div_ceil(256), 1)));
        }
        if self.shade && cam.zoom >= 1.0 {
            passes.push((4, (rw.div_ceil(16), rh.div_ceil(16))));
        }
        passes.push((0, (gx, gy)));
        if self.n_marks > 0 {
            passes.push((1, (self.n_marks, 1)));
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
}
