//! The mould from one spot (owner, 2026-11-11 20:07Z: "If you start a slime
//! mold in one part of your map instead of everywhere, how well does it
//! navigate the rock/terrain and walls, etc.?"). The GPU mould starts packed
//! on its home pile in a 2048 x 1280 world with the Larder World's terrain
//! on the GPU (rock it cannot enter, wet ground that halves its step) and
//! a wall across the way to the nearest pile; the same seed on open ground
//! is the comparison.
//!
//! `cargo run --release --example worldspread -- out=DIR [seconds=90]
//! [seed=1] [walkers=400000] [rock=0.22] [wet=0.22] [wall=1] [every=1] [down=2]
//! [any GPU setting, such as upkeep=0.01 capacity=400000 crystal=2]`
//!
//! A second here is 120 engine ticks whatever `ticks` is (`ticks` only
//! batches them into frames). At this world's scale 8 that is one game
//! second of `larder_world` (8 engine ticks a game tick, 15 a second); a
//! `bigdish` second on the scale-2 race world is 8 game seconds. `every=S`
//! keeps one GIF picture every S seconds (food tubes, 2026-11-12: a
//! 720 s run at every=8 is 90 pictures, as a 90 s run at every=1).
//! `down=N` shrinks each picture N times (default 2: 1024 x 640; 4 gives
//! 512 x 320 and a GIF about a quarter the size).
//!
//! Prints, every 5 s, the piles reached and the share of open ground the
//! mould covers (look thickness 26/255 or more: where it reads as slime),
//! then each pile's first-reached time. Writes `DIR/spread.gif`: the whole
//! world at zoom 1 (look on), downscaled 2x, a frame a second.

use pixels::wgpu;
use slime_mould::gpu::*;
use slime_mould::mould::STEPS_PER_SECOND;
use slime_mould::world::look::{dish_crystals, ground, ground_buffer, Fields};
use slime_mould::world::{Camera, View};

/// Thickness at which a cell looks like slime (as `worldlook`'s shots).
const SEEN: u8 = 26;

fn main() {
    let mut out = String::from("worldspread");
    let (mut seconds, mut seed, mut walkers) = (90.0f32, 1u64, 400_000usize);
    let (mut rock, mut wet, mut wall) = (0.22f32, 0.22f32, true);
    // Seconds between GIF pictures (whole seconds of the loop below).
    let mut every = 1u32;
    // Times smaller than the world, each GIF picture.
    let mut down = 2u32;
    let mut extra: Vec<(String, String)> = Vec::new();
    for a in std::env::args().skip(1) {
        let (k, v) = a.split_once('=').expect("key=value");
        match k {
            "out" => out = v.into(),
            "seconds" => seconds = v.parse().unwrap(),
            "seed" => seed = v.parse().unwrap(),
            "walkers" => walkers = v.parse().unwrap(),
            "rock" => rock = v.parse().unwrap(),
            "wet" => wet = v.parse().unwrap(),
            "wall" => wall = v != "0",
            "every" => every = v.parse::<u32>().unwrap().max(1),
            "down" => down = v.parse::<u32>().unwrap().max(1),
            // Anything else is a GPU setting (`gpu::set`: growth, crystals).
            _ => extra.push((k.to_string(), v.to_string())),
        }
    }
    std::fs::create_dir_all(&out).unwrap();
    let instance = wgpu::Instance::new(wgpu::InstanceDescriptor::new_without_display_handle_from_env());
    let adapter = block_on(instance.request_adapter(&wgpu::RequestAdapterOptions { power_preference: wgpu::PowerPreference::HighPerformance, ..Default::default() })).expect("no GPU adapter");
    println!("adapter: {}", adapter.get_info().name);
    let (device, queue) = block_on(adapter.request_device(&wgpu::DeviceDescriptor { label: None, required_limits: adapter.limits(), ..Default::default() })).expect("no device");
    let shared = Shared::new(&device);
    let mut s = Settings { seed, walkers, start: Start::Home, rock, wet, ..Settings::big_dish() };
    for (k, v) in &extra {
        set(&mut s, k, v).expect("a GPU setting");
    }
    let scale = 8usize;
    let mut e = Engine::new(&device, &queue, &shared, &s, scale);
    let (w, h) = (e.dish.w, e.dish.h);
    let piles = e.dish.piles.clone();
    // Where the mould starts: its home pile, or the dish's middle for a
    // disc start (`start=disc disc=R`, R in fine cells).
    let home = if s.start == Start::Disc { (w as f32 / scale as f32 / 2.0, h as f32 / scale as f32 / 2.0) } else { piles[0] };
    let start_r = if s.start == Start::Disc { s.disc / scale as f32 } else { 0.0 };
    // A wall across the way to the nearest pile: 24 dish cells long, 2
    // thick, square to the line, 70% of the way there (past the start blob).
    if wall {
        let (k, &(px, py)) = piles.iter().enumerate().skip(1).filter(|(_, p)| (p.0 - home.0).hypot(p.1 - home.1) > start_r + 14.0).min_by(|a, b| (a.1 .0 - home.0).hypot(a.1 .1 - home.1).total_cmp(&(b.1 .0 - home.0).hypot(b.1 .1 - home.1))).unwrap();
        let (dx, dy) = (px - home.0, py - home.1);
        let d = dx.hypot(dy);
        let (ux, uy) = (dx / d, dy / d);
        // Past the start disc, short of the pile.
        let at = (0.7 * d).max(start_r + 6.0).min(d - 6.0) / d;
        let (cx, cy) = (home.0 + dx * at, home.1 + dy * at);
        e.build_wall(&shared, &s, cx - uy * 12.0, cy + ux * 12.0, cx + uy * 12.0, cy - ux * 12.0, 2.0);
        println!("wall across the way to pile {k}, {d:.0} dish cells from home");
    }
    let crystal = dish_crystals(&e.dish);
    let hp = s.wall_hp;
    let look_ground = |e: &Engine| {
        let g: Vec<u32> = read_back(&device, &queue, &e.ground).chunks_exact(4).map(|c| u32::from_le_bytes(c.try_into().unwrap())).collect();
        let hl: Vec<f32> = read_back(&device, &queue, &e.health).chunks_exact(4).map(|c| f32::from_le_bytes(c.try_into().unwrap())).collect();
        let is_rock: Vec<bool> = g.iter().map(|&v| v & GROUND_ROCK != 0).collect();
        let is_wet: Vec<bool> = g.iter().map(|&v| v & GROUND_WET != 0).collect();
        let is_wall: Vec<bool> = g.iter().map(|&v| v & GROUND_WALL != 0).collect();
        let share: Vec<f32> = hl.iter().map(|&x| if hp > 0.0 && x.is_finite() { x / hp } else { 1.0 }).collect();
        ground(w, w * h, &Fields { rock: &is_rock, wet: &is_wet, wall: &is_wall, wall_share: &share, crystal: &crystal, ..Default::default() })
    };
    let codes = look_ground(&e);
    let open = codes.iter().filter(|&&c| c & 255 == 0 || c & 255 == 1).count();
    println!("open ground: {open} cells ({} rock, {} wet)", codes.iter().filter(|&&c| c & 255 == 2).count(), codes.iter().filter(|&&c| c & 255 == 1).count());
    let mut ground_buf = ground_buffer(&device, &queue, &codes);

    let fps = STEPS_PER_SECOND as f32 / s.ticks as f32;
    let screen = (w as u32, h as u32);
    let mut view = View::new(&device, &queue, &e.frame, (w, h), screen);
    view.minimap = false;
    view.set_look(&e.look, &ground_buf, &e.walkers, e.n);
    let mut cam = Camera::new((w, h), screen);
    cam.zoom_to(1.0, (0.0, 0.0));
    cam.centre_on((w as f32 / 2.0, h as f32 / 2.0));
    let mut gif = Vec::new();
    let mut first = vec![f32::NAN; piles.len()];
    let frames = (seconds * fps) as u32;
    for f in 0..=frames {
        if f > 0 {
            e.write_params(&s, Hand::default(), s.ticks);
            let mut enc = device.create_command_encoder(&wgpu::CommandEncoderDescriptor { label: None });
            e.record(&shared, &mut enc, s.ticks, s.curve == Curve::Soft, None);
            queue.submit([enc.finish()]);
        }
        let t = f as f32 / fps;
        if f % fps.round() as u32 != 0 {
            continue;
        }
        let r: Vec<u32> = read_back(&device, &queue, &e.reached).chunks_exact(4).map(|c| u32::from_le_bytes(c.try_into().unwrap())).collect();
        for (k, fr) in first.iter_mut().enumerate() {
            if fr.is_nan() && r[k] != 0 {
                *fr = t;
            }
        }
        // Walls wear: redraw the ground from the engine every 5 s.
        if f % (5.0 * fps).round() as u32 == 0 {
            ground_buf = ground_buffer(&device, &queue, &look_ground(&e));
            view.set_look(&e.look, &ground_buf, &e.walkers, e.n);
            let th: Vec<u8> = read_back(&device, &queue, &e.look).chunks_exact(4).map(|c| c[0]).collect();
            let covered = (0..w * h).filter(|&i| (codes[i] & 255 <= 1) && th[i] >= SEEN).count();
            let (_, bites, broken) = e.world_counts();
            println!("t {t:>4.0} s: piles reached {} of {}, open ground covered {:.1}%, wall bites {bites}, wall cells broken {broken}", r.iter().take(piles.len()).filter(|&&v| v != 0).count(), piles.len(), 100.0 * covered as f64 / open as f64);
        }
        if !(f / fps.round() as u32).is_multiple_of(every) {
            continue;
        }
        let mut enc = device.create_command_encoder(&wgpu::CommandEncoderDescriptor { label: None });
        view.record(&mut enc, &cam, None);
        queue.submit([enc.finish()]);
        let pic = view.picture();
        gif.push(image::imageops::resize(&pic, w as u32 / down, h as u32 / down, image::imageops::FilterType::Triangle));
    }
    for (k, (fr, p)) in first.iter().zip(&piles).enumerate() {
        println!("pile {k} at ({:.0}, {:.0}), {:.0} dish cells from home: reached at {}", p.0, p.1, (p.0 - home.0).hypot(p.1 - home.1), if fr.is_nan() { "never".to_string() } else { format!("{fr:.0} s") });
    }
    let delay = image::Delay::from_numer_denom_ms(100, 1);
    let frames: Vec<image::Frame> = gif.into_iter().map(|img| image::Frame::from_parts(img, 0, 0, delay)).collect();
    let file = std::fs::File::create(format!("{out}/spread.gif")).expect("gif file");
    let mut enc = image::codecs::gif::GifEncoder::new_with_speed(file, 10);
    enc.set_repeat(image::codecs::gif::Repeat::Infinite).unwrap();
    enc.encode_frames(frames).expect("gif encode");
    println!("wrote {out}");
}
