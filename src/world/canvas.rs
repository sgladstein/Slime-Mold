//! **The UI canvas**: a small 2D renderer for the game's panels, drawn on
//! the GPU over the view's finished frame (`canvas.wgsl`). Shapes are
//! rounded rectangles (flat or a vertical gradient, with a border), soft
//! shadows, bars, rings (a cooldown sweep) and atlas masks: text in Inter
//! and line icons (`icons`), rasterized once on the CPU into a coverage
//! atlas and drawn tinted.
//!
//! The owner asked for a UI that looks "modern and professional"
//! (2026-11-11 20:16Z). The first panel was the 5x7 pixel font in screen
//! marks; marks claim a pixel outright, so anti-aliased text over a
//! translucent panel would blend its edges with the world beneath, not the
//! panel. Hence a separate pass that blends every shape in order. The CPU
//! bins shapes into 16 x 16 tiles; the GPU walks only a tile's own list.
//!
//! The fonts are Inter Medium and SemiBold (SIL OFL 1.1,
//! `assets/fonts/OFL.txt`), embedded in the binary.

use std::collections::HashMap;

use ab_glyph::{Font, FontRef, PxScale, ScaleFont};
use pixels::wgpu;

use super::icons::Icon;

pub const SHADER: &str = include_str!("canvas.wgsl");

const PRIM_WORDS: usize = 16;
/// The most shapes a frame, and tile entries (a shape covers several
/// tiles). Past either, shapes are dropped (the last-drawn first).
/// A frame's shapes past this many are dropped (a runaway, not a busy
/// screen: the busiest page, the help over a full selection, is ~1,500).
pub const MAX_PRIMS: usize = 1 << 16;
/// The GPU buffers' first sizes; they grow when a frame needs more. (They
/// once stayed fixed and a frame over the item budget kept each tile's
/// first shapes, which are the panels, and dropped the text and menus on
/// top: the review of 2026-11-11 caught it.)
const FIRST_PRIMS: usize = 8192;
const FIRST_ITEMS: usize = 1 << 17;
pub const TILE: u32 = 16;
/// The atlas, in texels a side.
pub const ATLAS: u32 = 1024;

const RRECT: u32 = 0;
const SHADOW: u32 = 1;
const MASK: u32 = 2;
const BAR: u32 = 3;
const RING: u32 = 4;

pub type Rgba = [u8; 4];

/// The UI's two weights of Inter.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub enum Weight {
    Medium,
    SemiBold,
}

static MEDIUM: &[u8] = include_bytes!("../../assets/fonts/Inter-Medium.otf");
static SEMIBOLD: &[u8] = include_bytes!("../../assets/fonts/Inter-SemiBold.otf");

#[derive(Clone, Copy, Debug, PartialEq)]
struct Prim {
    kind: u32,
    lo: (f32, f32),
    hi: (f32, f32),
    radius: f32,
    colour: Rgba,
    colour2: Rgba,
    border: f32,
    soft: f32,
    uv: (u32, u32),
    border_colour: Rgba,
    fill: f32,
    /// A rectangle left open (left, top, right, bottom pixels); zero none.
    hole: (u32, u32),
}

impl Prim {
    fn new(kind: u32, lo: (f32, f32), hi: (f32, f32), colour: Rgba) -> Self {
        Prim { kind, lo, hi, radius: 0.0, colour, colour2: [0; 4], border: 0.0, soft: 0.0, uv: (0, 0), border_colour: [0; 4], fill: 1.0, hole: (0, 0) }
    }

    /// The pixels it can touch (a shadow spreads past its box).
    fn bounds(&self) -> (f32, f32, f32, f32) {
        let s = if self.kind == SHADOW { self.soft + 1.0 } else { 1.0 };
        (self.lo.0 - s, self.lo.1 - s, self.hi.0 + s, self.hi.1 + s)
    }

    fn words(&self) -> [u32; PRIM_WORDS] {
        let c = u32::from_le_bytes;
        [
            self.kind,
            self.lo.0.to_bits(),
            self.lo.1.to_bits(),
            self.hi.0.to_bits(),
            self.hi.1.to_bits(),
            self.radius.to_bits(),
            c(self.colour),
            c(self.colour2),
            self.border.to_bits(),
            self.soft.to_bits(),
            self.uv.0,
            self.uv.1,
            c(self.border_colour),
            self.fill.to_bits(),
            self.hole.0,
            self.hole.1,
        ]
    }
}

/// A rectangle on screen: left, top, width, height in pixels.
#[derive(Clone, Copy, Debug, PartialEq, Default)]
pub struct Rect {
    pub x: f32,
    pub y: f32,
    pub w: f32,
    pub h: f32,
}

impl Rect {
    pub const fn new(x: f32, y: f32, w: f32, h: f32) -> Self {
        Rect { x, y, w, h }
    }

    pub fn contains(&self, p: (f32, f32)) -> bool {
        p.0 >= self.x && p.1 >= self.y && p.0 < self.x + self.w && p.1 < self.y + self.h
    }

    pub fn right(&self) -> f32 {
        self.x + self.w
    }

    pub fn bottom(&self) -> f32 {
        self.y + self.h
    }

    /// Shrunk by `d` on every side.
    pub fn inset(&self, d: f32) -> Rect {
        Rect::new(self.x + d, self.y + d, (self.w - 2.0 * d).max(0.0), (self.h - 2.0 * d).max(0.0))
    }
}

/// Where a glyph or an icon sits in the atlas.
#[derive(Clone, Copy, Debug, PartialEq)]
struct Slot {
    u: u32,
    v: u32,
    w: u32,
    h: u32,
    /// Offset from the pen (glyphs) or the box's corner (icons).
    dx: i32,
    dy: i32,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
enum Key {
    Glyph(Weight, char, u16),
    Icon(Icon, u16),
}

/// The coverage atlas: glyphs and icons rasterized on first use, packed in
/// shelves. Only re-uploaded when something new went in.
pub struct Atlas {
    pub texels: Vec<u8>,
    slots: HashMap<Key, Option<Slot>>,
    shelf: (u32, u32, u32),
    pub dirty: bool,
    /// Something did not fit: the canvas clears the atlas before the next
    /// frame, which rasterizes what it uses again. (Without this a window
    /// resized through many sizes filled it, and new text never showed.)
    full: bool,
    fonts: [FontRef<'static>; 2],
}

impl Default for Atlas {
    fn default() -> Self {
        Self::new()
    }
}

impl Atlas {
    pub fn new() -> Self {
        let fonts = [FontRef::try_from_slice(MEDIUM).expect("Inter Medium"), FontRef::try_from_slice(SEMIBOLD).expect("Inter SemiBold")];
        Atlas { texels: vec![0; (ATLAS * ATLAS) as usize], slots: HashMap::new(), shelf: (1, 1, 0), dirty: true, full: false, fonts }
    }

    /// Empties the atlas (everything is rasterized again on next use).
    pub fn reset(&mut self) {
        self.texels.fill(0);
        self.slots.clear();
        self.shelf = (1, 1, 0);
        self.full = false;
        self.dirty = true;
    }

    pub fn is_full(&self) -> bool {
        self.full
    }

    fn font(&self, w: Weight) -> &FontRef<'static> {
        &self.fonts[w as usize]
    }

    /// Room for a w x h box (with a texel of gap); `None` when full.
    fn alloc(&mut self, w: u32, h: u32) -> Option<(u32, u32)> {
        let (x, y, row) = self.shelf;
        let (x, y, row) = if x + w + 1 > ATLAS { (1, y + row + 1, 0) } else { (x, y, row) };
        if y + h + 1 > ATLAS {
            self.full = true;
            return None;
        }
        self.shelf = (x + w + 1, y, row.max(h));
        Some((x, y))
    }

    fn glyph(&mut self, weight: Weight, ch: char, px: u16) -> Option<Slot> {
        let key = Key::Glyph(weight, ch, px);
        if let Some(s) = self.slots.get(&key) {
            return *s;
        }
        let font = self.font(weight).clone();
        let g = font.glyph_id(ch).with_scale_and_position(PxScale::from(px as f32), ab_glyph::point(0.0, 0.0));
        let slot = font.outline_glyph(g).and_then(|o| {
            let b = o.px_bounds();
            let (w, h) = (b.width().ceil() as u32, b.height().ceil() as u32);
            let (u, v) = self.alloc(w.max(1), h.max(1))?;
            o.draw(|x, y, c| {
                let i = ((v + y) * ATLAS + u + x) as usize;
                if i < self.texels.len() {
                    self.texels[i] = (c.clamp(0.0, 1.0) * 255.0).round() as u8;
                }
            });
            Some(Slot { u, v, w, h, dx: b.min.x.floor() as i32, dy: b.min.y.floor() as i32 })
        });
        self.slots.insert(key, slot);
        self.dirty = true;
        slot
    }

    fn icon(&mut self, icon: Icon, px: u16) -> Option<Slot> {
        let key = Key::Icon(icon, px);
        if let Some(s) = self.slots.get(&key) {
            return *s;
        }
        let side = px as u32;
        let slot = self.alloc(side, side).map(|(u, v)| {
            let cover = super::icons::rasterize(icon, side);
            for y in 0..side {
                for x in 0..side {
                    self.texels[((v + y) * ATLAS + u + x) as usize] = cover[(y * side + x) as usize];
                }
            }
            Slot { u, v, w: side, h: side, dx: 0, dy: 0 }
        });
        self.slots.insert(key, slot);
        self.dirty = true;
        slot
    }

    /// The pen advance of `text` at `px` (kerning included), in pixels.
    pub fn measure(&self, weight: Weight, px: f32, text: &str) -> f32 {
        // The size text is drawn at (whole pixels, so glyphs cache).
        let f = self.font(weight).as_scaled(PxScale::from(px.round().max(4.0)));
        let mut w = 0.0;
        let mut last = None;
        for ch in text.chars() {
            let id = f.glyph_id(ch);
            if let Some(l) = last {
                w += f.kern(l, id);
            }
            w += f.h_advance(id);
            last = Some(id);
        }
        w
    }

    /// The font's ascent and descent at `px` (descent is negative).
    pub fn metrics(&self, weight: Weight, px: f32) -> (f32, f32) {
        let f = self.font(weight).as_scaled(PxScale::from(px));
        (f.ascent(), f.descent())
    }

    /// The atlas as words, four texels a word, for the GPU.
    pub fn words(&self) -> Vec<u32> {
        self.texels.chunks_exact(4).map(|c| u32::from_le_bytes([c[0], c[1], c[2], c[3]])).collect()
    }
}

/// How text sits against its anchor.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Align {
    Left,
    Centre,
    Right,
}

/// A frame's UI shapes, in draw order (later on top).
pub struct Canvas {
    prims: Vec<Prim>,
    pub atlas: Atlas,
    pub screen: (u32, u32),
}

impl Canvas {
    pub fn new(screen: (u32, u32)) -> Self {
        Canvas { prims: Vec::new(), atlas: Atlas::new(), screen }
    }

    /// Starts a frame (and empties the atlas if the last frame filled it).
    pub fn clear(&mut self) {
        self.prims.clear();
        if self.atlas.full {
            self.atlas.reset();
        }
    }

    pub fn len(&self) -> usize {
        self.prims.len()
    }

    pub fn is_empty(&self) -> bool {
        self.prims.is_empty()
    }

    fn push(&mut self, p: Prim) {
        if self.prims.len() < MAX_PRIMS {
            self.prims.push(p);
        }
    }

    /// A rounded box, flat.
    pub fn rrect(&mut self, r: Rect, radius: f32, colour: Rgba) {
        self.push(Prim { radius, ..Prim::new(RRECT, (r.x, r.y), (r.right(), r.bottom()), colour) });
    }

    /// A rounded box with a vertical gradient and an inner border.
    pub fn panel(&mut self, r: Rect, radius: f32, top: Rgba, bottom: Rgba, border: f32, border_colour: Rgba) {
        self.push(Prim { radius, colour2: bottom, border, border_colour, ..Prim::new(RRECT, (r.x, r.y), (r.right(), r.bottom()), top) });
    }

    /// A panel with a rectangular hole left open (a frame round something
    /// drawn before the canvas, such as the minimap).
    #[allow(clippy::too_many_arguments)]
    pub fn panel_with_hole(&mut self, r: Rect, radius: f32, top: Rgba, bottom: Rgba, border: f32, border_colour: Rgba, hole: Rect) {
        let p = |a: f32, b: f32| (a.max(0.0) as u32 & 0xffff) | ((b.max(0.0) as u32 & 0xffff) << 16);
        let hole = (p(hole.x, hole.y), p(hole.right(), hole.bottom()).max(1));
        self.push(Prim { radius, colour2: bottom, border, border_colour, hole, ..Prim::new(RRECT, (r.x, r.y), (r.right(), r.bottom()), top) });
    }

    /// A rounded outline `width` wide inside `r`.
    pub fn outline(&mut self, r: Rect, radius: f32, width: f32, colour: Rgba) {
        self.push(Prim { radius, border: width, border_colour: colour, ..Prim::new(RRECT, (r.x, r.y), (r.right(), r.bottom()), [0; 4]) });
    }

    /// A soft shadow under a box: `soft` pixels of blur, offset `dy` down.
    pub fn shadow(&mut self, r: Rect, radius: f32, soft: f32, dy: f32, colour: Rgba) {
        self.push(Prim { radius, soft, ..Prim::new(SHADOW, (r.x, r.y + dy), (r.right(), r.bottom() + dy), colour) });
    }

    /// A bar `fill` (0-1) full, rounded.
    pub fn bar(&mut self, r: Rect, fill: f32, colour: Rgba, back: Rgba) {
        self.push(Prim { radius: r.h / 2.0, colour2: back, fill: fill.clamp(0.0, 1.0), ..Prim::new(BAR, (r.x, r.y), (r.right(), r.bottom()), colour) });
    }

    /// A ring `width` wide on `r`'s rounded edge, swept clockwise from the
    /// top to `fill` (1 a full ring).
    pub fn ring(&mut self, r: Rect, radius: f32, width: f32, fill: f32, colour: Rgba) {
        self.push(Prim { radius, border: width, fill: fill.clamp(0.0, 1.0), ..Prim::new(RING, (r.x, r.y), (r.right(), r.bottom()), colour) });
    }

    /// An icon `px` pixels square with its top-left at `at`.
    pub fn icon(&mut self, icon: Icon, at: (f32, f32), px: f32, colour: Rgba) {
        let side = px.round().max(4.0) as u16;
        if let Some(s) = self.atlas.icon(icon, side) {
            let (x, y) = (at.0.round(), at.1.round());
            self.push(Prim { uv: (s.u, s.v), ..Prim::new(MASK, (x, y), (x + s.w as f32, y + s.h as f32), colour) });
        }
    }

    /// Text with its baseline at `at.1`, aligned on `at.0`. Returns its
    /// width.
    pub fn text(&mut self, weight: Weight, px: f32, at: (f32, f32), align: Align, text: &str, colour: Rgba) -> f32 {
        let w = self.atlas.measure(weight, px, text);
        let x0 = match align {
            Align::Left => at.0,
            Align::Centre => at.0 - w / 2.0,
            Align::Right => at.0 - w,
        };
        let size = px.round().max(4.0) as u16;
        let font = self.atlas.font(weight).clone();
        let f = font.as_scaled(PxScale::from(size as f32));
        let mut pen = x0;
        let mut last = None;
        let base = at.1.round();
        for ch in text.chars() {
            let id = f.glyph_id(ch);
            if let Some(l) = last {
                pen += f.kern(l, id);
            }
            if let Some(s) = self.atlas.glyph(weight, ch, size) {
                let x = (pen.round() as i32 + s.dx) as f32;
                let y = (base as i32 + s.dy) as f32;
                self.push(Prim { uv: (s.u, s.v), ..Prim::new(MASK, (x, y), (x + s.w as f32, y + s.h as f32), colour) });
            }
            pen += f.h_advance(id);
            last = Some(id);
        }
        w
    }

    /// Text cut to fit `max` pixels, with an ellipsis.
    #[allow(clippy::too_many_arguments)]
    pub fn text_fit(&mut self, weight: Weight, px: f32, at: (f32, f32), align: Align, text: &str, max: f32, colour: Rgba) -> f32 {
        if self.atlas.measure(weight, px, text) <= max {
            return self.text(weight, px, at, align, text, colour);
        }
        let mut s: String = text.into();
        while !s.is_empty() && self.atlas.measure(weight, px, &format!("{s}…")) > max {
            s.pop();
        }
        self.text(weight, px, at, align, &format!("{s}…"), colour)
    }

    /// Text wrapped to `max` pixels a line, `lead` pixels between
    /// baselines, from a first baseline at `at`. Returns the lines used.
    #[allow(clippy::too_many_arguments)]
    pub fn paragraph(&mut self, weight: Weight, px: f32, at: (f32, f32), max: f32, lead: f32, text: &str, colour: Rgba) -> usize {
        let lines = self.wrap(weight, px, max, text);
        for (k, l) in lines.iter().enumerate() {
            self.text(weight, px, (at.0, at.1 + k as f32 * lead), Align::Left, l, colour);
        }
        lines.len()
    }

    /// `text` broken into lines no wider than `max`.
    pub fn wrap(&self, weight: Weight, px: f32, max: f32, text: &str) -> Vec<String> {
        let mut out = Vec::new();
        for para in text.split('\n') {
            let mut line = String::new();
            for word in para.split(' ') {
                let trial = if line.is_empty() { word.to_string() } else { format!("{line} {word}") };
                if self.atlas.measure(weight, px, &trial) > max && !line.is_empty() {
                    out.push(std::mem::take(&mut line));
                    line = word.to_string();
                } else {
                    line = trial;
                }
            }
            out.push(line);
        }
        out
    }

    /// The tile lists the GPU walks: `offsets` (tiles + 1) and `items`.
    pub fn bin(&self) -> (Vec<u32>, Vec<u32>, (u32, u32)) {
        let (tx, ty) = (self.screen.0.div_ceil(TILE), self.screen.1.div_ceil(TILE));
        let n = (tx * ty) as usize;
        let mut count = vec![0u32; n + 1];
        let span = |p: &Prim| {
            let (x0, y0, x1, y1) = p.bounds();
            let cl = |v: f32, m: u32| (v.max(0.0) as u32 / TILE).min(m - 1);
            if x1 < 0.0 || y1 < 0.0 || x0 >= self.screen.0 as f32 || y0 >= self.screen.1 as f32 {
                return None;
            }
            Some((cl(x0, tx), cl(y0, ty), cl(x1, tx), cl(y1, ty)))
        };
        let spans: Vec<_> = self.prims.iter().map(span).collect();
        let mut total = 0usize;
        for s in spans.iter().flatten() {
            for j in s.1..=s.3 {
                for i in s.0..=s.2 {
                    count[(j * tx + i) as usize] += 1;
                    total += 1;
                }
            }
        }
        let mut offsets = vec![0u32; n + 1];
        for t in 0..n {
            offsets[t + 1] = offsets[t] + count[t];
        }
        let mut fill = offsets.clone();
        let mut items = vec![0u32; total];
        for (k, s) in spans.iter().enumerate() {
            if let Some(s) = s {
                for j in s.1..=s.3 {
                    for i in s.0..=s.2 {
                        let t = (j * tx + i) as usize;
                        items[fill[t] as usize] = k as u32;
                        fill[t] += 1;
                    }
                }
            }
        }
        (offsets, items, (tx, ty))
    }

    pub fn words(&self) -> Vec<u32> {
        self.prims.iter().flat_map(|p| p.words()).collect()
    }
}

fn storage(dev: &wgpu::Device, label: &str, bytes: u64) -> wgpu::Buffer {
    dev.create_buffer(&wgpu::BufferDescriptor { label: Some(label), size: bytes.max(16), usage: wgpu::BufferUsages::STORAGE | wgpu::BufferUsages::COPY_DST, mapped_at_creation: false })
}

/// The GPU side: draws a `Canvas` onto the view's `out` buffer.
pub struct CanvasPass {
    device: wgpu::Device,
    queue: wgpu::Queue,
    pipe: wgpu::ComputePipeline,
    bgl: wgpu::BindGroupLayout,
    params: wgpu::Buffer,
    prims: wgpu::Buffer,
    offsets: wgpu::Buffer,
    items: wgpu::Buffer,
    atlas: wgpu::Buffer,
    group: Option<(wgpu::BindGroup, wgpu::Buffer)>,
    tiles: (u32, u32),
    screen: (u32, u32),
    stride: u32,
    n: u32,
}

impl CanvasPass {
    pub fn new(device: &wgpu::Device, queue: &wgpu::Queue) -> Self {
        let module = device.create_shader_module(wgpu::ShaderModuleDescriptor { label: Some("canvas"), source: wgpu::ShaderSource::Wgsl(SHADER.into()) });
        let entry = |binding: u32, ty: wgpu::BufferBindingType| wgpu::BindGroupLayoutEntry {
            binding,
            visibility: wgpu::ShaderStages::COMPUTE,
            ty: wgpu::BindingType::Buffer { ty, has_dynamic_offset: false, min_binding_size: None },
            count: None,
        };
        let ro = wgpu::BufferBindingType::Storage { read_only: true };
        let entries = [entry(0, wgpu::BufferBindingType::Uniform), entry(1, ro), entry(2, ro), entry(3, ro), entry(4, ro), entry(5, wgpu::BufferBindingType::Storage { read_only: false })];
        let bgl = device.create_bind_group_layout(&wgpu::BindGroupLayoutDescriptor { label: Some("canvas"), entries: &entries });
        let layout = device.create_pipeline_layout(&wgpu::PipelineLayoutDescriptor { label: Some("canvas"), bind_group_layouts: &[Some(&bgl)], immediate_size: 0 });
        let pipe = device.create_compute_pipeline(&wgpu::ComputePipelineDescriptor { label: Some("canvas"), layout: Some(&layout), module: &module, entry_point: Some("draw"), compilation_options: Default::default(), cache: None });
        let params = device.create_buffer(&wgpu::BufferDescriptor { label: Some("canvas params"), size: 32, usage: wgpu::BufferUsages::UNIFORM | wgpu::BufferUsages::COPY_DST, mapped_at_creation: false });
        CanvasPass {
            device: device.clone(),
            queue: queue.clone(),
            pipe,
            bgl,
            params,
            prims: storage(device, "canvas prims", (FIRST_PRIMS * PRIM_WORDS * 4) as u64),
            offsets: storage(device, "canvas offsets", 16),
            items: storage(device, "canvas items", (FIRST_ITEMS * 4) as u64),
            atlas: storage(device, "canvas atlas", (ATLAS * ATLAS) as u64),
            group: None,
            tiles: (0, 0),
            screen: (0, 0),
            stride: 0,
            n: 0,
        }
    }

    /// Uploads this frame's shapes (and the atlas if it grew).
    pub fn upload(&mut self, c: &mut Canvas) {
        if c.atlas.dirty {
            let bytes: Vec<u8> = c.atlas.texels.clone();
            self.queue.write_buffer(&self.atlas, 0, &bytes);
            c.atlas.dirty = false;
        }
        let (offsets, items, tiles) = c.bin();
        if self.offsets.size() < (offsets.len() * 4) as u64 {
            self.offsets = storage(&self.device, "canvas offsets", (offsets.len() * 4) as u64);
            self.group = None;
        }
        self.queue.write_buffer(&self.offsets, 0, &u32s(&offsets));
        if self.items.size() < (items.len() * 4) as u64 {
            self.items = storage(&self.device, "canvas items", (items.len() * 4).next_power_of_two() as u64);
            self.group = None;
        }
        if !items.is_empty() {
            self.queue.write_buffer(&self.items, 0, &u32s(&items));
        }
        let words = c.words();
        if self.prims.size() < (words.len() * 4) as u64 {
            self.prims = storage(&self.device, "canvas prims", (words.len() * 4).next_power_of_two() as u64);
            self.group = None;
        }
        if !words.is_empty() {
            self.queue.write_buffer(&self.prims, 0, &u32s(&words));
        }
        self.tiles = tiles;
        self.screen = c.screen;
        self.n = c.len() as u32;
    }

    /// Draws the uploaded shapes onto `out` (rows `stride` pixels apart).
    pub fn record(&mut self, enc: &mut wgpu::CommandEncoder, out: &wgpu::Buffer, stride: u32) {
        self.record_timed(enc, out, stride, None);
    }

    /// As `record`, writing GPU timestamps at the pass's start and end.
    pub fn record_timed(&mut self, enc: &mut wgpu::CommandEncoder, out: &wgpu::Buffer, stride: u32, stamp: Option<(&wgpu::QuerySet, Option<u32>, Option<u32>)>) {
        let tw = stamp.map(|(q, a, b)| wgpu::ComputePassTimestampWrites { query_set: q, beginning_of_pass_write_index: a, end_of_pass_write_index: b });
        if self.n == 0 {
            // Nothing to draw; an empty pass still writes the timestamps.
            if tw.is_some() {
                drop(enc.begin_compute_pass(&wgpu::ComputePassDescriptor { label: Some("canvas"), timestamp_writes: tw }));
            }
            return;
        }
        let stale = self.group.as_ref().is_none_or(|(_, o)| o != out);
        if stale || self.stride != stride {
            let bufs = [&self.params, &self.prims, &self.offsets, &self.items, &self.atlas, out];
            let entries: Vec<wgpu::BindGroupEntry> = bufs.iter().enumerate().map(|(i, b)| wgpu::BindGroupEntry { binding: i as u32, resource: b.as_entire_binding() }).collect();
            let g = self.device.create_bind_group(&wgpu::BindGroupDescriptor { label: Some("canvas"), layout: &self.bgl, entries: &entries });
            self.group = Some((g, out.clone()));
        }
        self.stride = stride;
        let p = [self.screen.0, self.screen.1, stride, self.tiles.0, self.tiles.1, self.n, ATLAS, 0];
        self.queue.write_buffer(&self.params, 0, &u32s(&p));
        let mut pass = enc.begin_compute_pass(&wgpu::ComputePassDescriptor { label: Some("canvas"), timestamp_writes: tw });
        pass.set_pipeline(&self.pipe);
        pass.set_bind_group(0, &self.group.as_ref().unwrap().0, &[]);
        pass.dispatch_workgroups(self.tiles.0, self.tiles.1, 1);
    }
}

fn u32s(v: &[u32]) -> Vec<u8> {
    v.iter().flat_map(|x| x.to_le_bytes()).collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn text_measures_and_rasterizes_into_the_atlas() {
        let mut c = Canvas::new((640, 360));
        let w = c.text(Weight::Medium, 16.0, (10.0, 30.0), Align::Left, "Soldiers 12", [255; 4]);
        assert!(w > 60.0 && w < 140.0, "width {w}");
        assert_eq!(c.len(), 10, "a shape a visible glyph (the space has none)");
        assert!(c.atlas.texels.iter().filter(|&&t| t > 128).count() > 100);
        // A second draw reuses the atlas.
        c.atlas.dirty = false;
        c.text(Weight::Medium, 16.0, (10.0, 60.0), Align::Left, "Soldiers", [255; 4]);
        assert!(!c.atlas.dirty);
    }

    #[test]
    fn shapes_bin_into_the_tiles_they_touch_in_order() {
        let mut c = Canvas::new((64, 32));
        c.rrect(Rect::new(0.0, 0.0, 20.0, 10.0), 2.0, [0, 0, 0, 255]);
        c.rrect(Rect::new(40.0, 20.0, 10.0, 10.0), 2.0, [0, 0, 0, 255]);
        let (off, items, tiles) = c.bin();
        assert_eq!(tiles, (4, 2));
        // With a pixel of slack the first box (x 0-20) reaches tiles 0 and 1
        // of the top row; the second (x 39-51, y 19-31) tiles 6 and 7.
        let tile = |t: usize| items[off[t] as usize..off[t + 1] as usize].to_vec();
        assert_eq!(tile(0), vec![0]);
        assert_eq!(tile(1), vec![0]);
        assert_eq!(tile(2), Vec::<u32>::new());
        assert_eq!(tile(6), vec![1]);
        assert_eq!(tile(7), vec![1]);
        assert_eq!(tile(5), Vec::<u32>::new());
    }

    #[test]
    fn a_full_atlas_is_emptied_for_the_next_frame() {
        let mut c = Canvas::new((1920, 1080));
        let text = "The quick brown fox jumps over the lazy dog 0123456789";
        let mut px = 8.0;
        while !c.atlas.is_full() && px < 400.0 {
            c.text(Weight::Medium, px, (0.0, 100.0), Align::Left, text, [255; 4]);
            px += 1.0;
        }
        assert!(c.atlas.is_full(), "never filled");
        c.clear();
        assert!(!c.atlas.is_full());
        // Text draws again (its glyphs go back in).
        let before = c.len();
        c.text(Weight::Medium, 20.0, (0.0, 100.0), Align::Left, "Again", [255; 4]);
        assert!(c.len() > before);
    }

    #[test]
    fn every_shape_is_binned_however_many() {
        // A full-screen modal under many shapes: no tile drops its top ones.
        let mut c = Canvas::new((256, 256));
        for _ in 0..3000 {
            c.rrect(Rect::new(0.0, 0.0, 256.0, 256.0), 0.0, [0, 0, 0, 10]);
        }
        let (offsets, items, _) = c.bin();
        assert_eq!(items.len(), 3000 * 256);
        assert_eq!(*offsets.last().unwrap() as usize, items.len());
        // Each tile's list ends with the last shape drawn.
        assert_eq!(items[offsets[1] as usize - 1], 2999);
    }

    #[test]
    fn wrap_breaks_on_words() {
        let c = Canvas::new((100, 100));
        let lines = c.wrap(Weight::Medium, 14.0, 80.0, "Lay a wall the mould must wear through");
        assert!(lines.len() >= 3, "{lines:?}");
        assert!(lines.iter().all(|l| c.atlas.measure(Weight::Medium, 14.0, l) <= 80.0 || !l.contains(' ')));
    }
}
