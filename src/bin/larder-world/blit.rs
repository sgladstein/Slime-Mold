//! **The view's picture straight to the window** (performance thread,
//! 2026-11-22).
//!
//! The view draws the screen into a storage buffer (`View::out`, rows
//! `stride` pixels apart). Until 11-22 the window showed it through the
//! `pixels` crate: `Pixels::render_with` first uploaded the crate's own CPU
//! pixel buffer, the window's size in RGBA, from the CPU to its texture
//! every frame (a 2560 x 1440 window is 14.7 MB a frame, 880 MB a second at
//! 60 frames), then the view copied its buffer over that same texture, and
//! the crate's scaling pass drew the texture to the window. The upload was
//! never seen, and the bench (`bench.rs`, an off-screen view) never paid it,
//! so the GTX 1070 batches of 11-14 and 11-21 could not show it.
//!
//! Now the crate's buffer is one pixel (`events.rs`), and this pass reads
//! the view's buffer in a fragment shader and writes the window's image
//! with it: no upload, no buffer-to-texture copy, the same picture. On an
//! sRGB window (the crate picks one) the shader decodes the bytes first,
//! because the hardware encodes them again on the write: the old path's
//! sRGB texture did the same decode in its sampler.

use pixels::wgpu;

const SHADER: &str = r#"
struct P { stride: u32, srgb: u32, w: u32, h: u32 }
@group(0) @binding(0) var<storage, read> px: array<u32>;
@group(0) @binding(1) var<uniform> p: P;

@vertex
fn vs(@builtin(vertex_index) i: u32) -> @builtin(position) vec4<f32> {
    let x = f32(i32(i & 1u) * 4 - 1);
    let y = f32(i32(i >> 1u) * 4 - 1);
    return vec4<f32>(x, y, 0.0, 1.0);
}

fn decode(c: f32) -> f32 {
    if c <= 0.04045 {
        return c / 12.92;
    }
    return pow((c + 0.055) / 1.055, 2.4);
}

@fragment
fn fs(@builtin(position) pos: vec4<f32>) -> @location(0) vec4<f32> {
    let x = min(u32(pos.x), p.w - 1u);
    let y = min(u32(pos.y), p.h - 1u);
    let c = unpack4x8unorm(px[y * p.stride + x]);
    if p.srgb == 1u {
        return vec4<f32>(decode(c.r), decode(c.g), decode(c.b), c.a);
    }
    return c;
}
"#;

pub struct Blit {
    layout: wgpu::BindGroupLayout,
    pipeline: wgpu::RenderPipeline,
    params: wgpu::Buffer,
    srgb: bool,
}

impl Blit {
    /// A pass that writes images of `format` (the window's).
    pub fn new(device: &wgpu::Device, format: wgpu::TextureFormat) -> Self {
        let module = device.create_shader_module(wgpu::ShaderModuleDescriptor { label: Some("blit"), source: wgpu::ShaderSource::Wgsl(SHADER.into()) });
        let entry = |binding, ty| wgpu::BindGroupLayoutEntry { binding, visibility: wgpu::ShaderStages::FRAGMENT, ty, count: None };
        let layout = device.create_bind_group_layout(&wgpu::BindGroupLayoutDescriptor {
            label: Some("blit"),
            entries: &[
                entry(0, wgpu::BindingType::Buffer { ty: wgpu::BufferBindingType::Storage { read_only: true }, has_dynamic_offset: false, min_binding_size: None }),
                entry(1, wgpu::BindingType::Buffer { ty: wgpu::BufferBindingType::Uniform, has_dynamic_offset: false, min_binding_size: None }),
            ],
        });
        let pl = device.create_pipeline_layout(&wgpu::PipelineLayoutDescriptor { label: Some("blit"), bind_group_layouts: &[Some(&layout)], immediate_size: 0 });
        let pipeline = device.create_render_pipeline(&wgpu::RenderPipelineDescriptor {
            label: Some("blit"),
            layout: Some(&pl),
            vertex: wgpu::VertexState { module: &module, entry_point: Some("vs"), buffers: &[], compilation_options: Default::default() },
            primitive: wgpu::PrimitiveState::default(),
            depth_stencil: None,
            multisample: wgpu::MultisampleState::default(),
            fragment: Some(wgpu::FragmentState {
                module: &module,
                entry_point: Some("fs"),
                compilation_options: Default::default(),
                targets: &[Some(wgpu::ColorTargetState { format, blend: Some(wgpu::BlendState::REPLACE), write_mask: wgpu::ColorWrites::ALL })],
            }),
            multiview_mask: None,
            cache: None,
        });
        let params = device.create_buffer(&wgpu::BufferDescriptor { label: Some("blit params"), size: 16, usage: wgpu::BufferUsages::UNIFORM | wgpu::BufferUsages::COPY_DST, mapped_at_creation: false });
        Self { layout, pipeline, params, srgb: format.is_srgb() }
    }

    /// Draws `out` (`screen` pixels, rows `stride` apart) onto `target`.
    /// The bind group is made each frame: the view's buffer is remade on a
    /// resize, and a bind group costs microseconds.
    pub fn draw(&self, (device, queue): (&wgpu::Device, &wgpu::Queue), enc: &mut wgpu::CommandEncoder, target: &wgpu::TextureView, out: &wgpu::Buffer, stride: u32, screen: (u32, u32)) {
        let words = [stride, u32::from(self.srgb), screen.0.max(1), screen.1.max(1)];
        let bytes: Vec<u8> = words.iter().flat_map(|w| w.to_le_bytes()).collect();
        queue.write_buffer(&self.params, 0, &bytes);
        let group = device.create_bind_group(&wgpu::BindGroupDescriptor {
            label: Some("blit"),
            layout: &self.layout,
            entries: &[wgpu::BindGroupEntry { binding: 0, resource: out.as_entire_binding() }, wgpu::BindGroupEntry { binding: 1, resource: self.params.as_entire_binding() }],
        });
        let mut pass = enc.begin_render_pass(&wgpu::RenderPassDescriptor {
            label: Some("blit"),
            color_attachments: &[Some(wgpu::RenderPassColorAttachment {
                view: target,
                resolve_target: None,
                ops: wgpu::Operations { load: wgpu::LoadOp::Clear(wgpu::Color::BLACK), store: wgpu::StoreOp::Store },
                depth_slice: None,
            })],
            depth_stencil_attachment: None,
            timestamp_writes: None,
            occlusion_query_set: None,
            multiview_mask: None,
        });
        pass.set_pipeline(&self.pipeline);
        pass.set_bind_group(0, &group, &[]);
        pass.draw(0..3, 0..1);
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use slime_mould::gpu::read_back;

    /// The blit gives the window the view's bytes: every byte value, on an
    /// sRGB window in both channel orders and on a plain one, back within
    /// one step of rounding (the old path's sRGB texture decoded and the
    /// window encoded the same way).
    #[test]
    fn the_blit_draws_the_views_bytes() {
        let Some((device, queue)) = super::super::bench::gpu() else { return };
        let (w, h, stride) = (70u32, 8u32, 128u32);
        let mut bytes = vec![0u8; (stride * h * 4) as usize];
        let mut seed = 12345u32;
        for (i, b) in bytes.iter_mut().enumerate() {
            seed = seed.wrapping_mul(1_664_525).wrapping_add(1_013_904_223);
            *b = if i < 256 { i as u8 } else { (seed >> 24) as u8 };
        }
        let out = device.create_buffer(&wgpu::BufferDescriptor { label: None, size: bytes.len() as u64, usage: wgpu::BufferUsages::STORAGE | wgpu::BufferUsages::COPY_DST, mapped_at_creation: false });
        queue.write_buffer(&out, 0, &bytes);
        for format in [wgpu::TextureFormat::Rgba8UnormSrgb, wgpu::TextureFormat::Bgra8UnormSrgb, wgpu::TextureFormat::Rgba8Unorm] {
            let blit = Blit::new(&device, format);
            let tex = device.create_texture(&wgpu::TextureDescriptor { label: None, size: wgpu::Extent3d { width: w, height: h, depth_or_array_layers: 1 }, mip_level_count: 1, sample_count: 1, dimension: wgpu::TextureDimension::D2, format, usage: wgpu::TextureUsages::RENDER_ATTACHMENT | wgpu::TextureUsages::COPY_SRC, view_formats: &[] });
            let row = 512u32;
            let back = device.create_buffer(&wgpu::BufferDescriptor { label: None, size: (row * h) as u64, usage: wgpu::BufferUsages::COPY_SRC | wgpu::BufferUsages::COPY_DST, mapped_at_creation: false });
            let mut enc = device.create_command_encoder(&Default::default());
            blit.draw((&device, &queue), &mut enc, &tex.create_view(&Default::default()), &out, stride, (w, h));
            enc.copy_texture_to_buffer(
                wgpu::TexelCopyTextureInfo { texture: &tex, mip_level: 0, origin: wgpu::Origin3d::ZERO, aspect: wgpu::TextureAspect::All },
                wgpu::TexelCopyBufferInfo { buffer: &back, layout: wgpu::TexelCopyBufferLayout { offset: 0, bytes_per_row: Some(row), rows_per_image: Some(h) } },
                wgpu::Extent3d { width: w, height: h, depth_or_array_layers: 1 },
            );
            queue.submit(Some(enc.finish()));
            let got = read_back(&device, &queue, &back);
            let mut worst = 0i32;
            for y in 0..h as usize {
                for x in 0..w as usize {
                    let want = &bytes[(y * stride as usize + x) * 4..][..4];
                    let have = &got[y * row as usize + x * 4..][..4];
                    let have = if format == wgpu::TextureFormat::Bgra8UnormSrgb { [have[2], have[1], have[0], have[3]] } else { [have[0], have[1], have[2], have[3]] };
                    for c in 0..4 {
                        worst = worst.max((i32::from(want[c]) - i32::from(have[c])).abs());
                    }
                }
            }
            assert!(worst <= 1, "{format:?}: a byte off by {worst}");
        }
    }
}
