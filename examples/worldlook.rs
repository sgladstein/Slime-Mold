//! The world look (2026-11-11): the GPU mould on a 2048 x 1280 world, drawn
//! through the camera with the look on (the world shaded from the
//! simulation's own fields, every walker a speck, soldiers as shader
//! figures) and off (the frame as it was, soldiers as discs), with a fight:
//! a squad shooting into the mould, each hit cutting trail in the engine.
//!
//! `cargo run --release --example worldlook -- out=DIR [seconds=12]
//! [fight=3] [seed=1] [walkers=400000] [screen=1280x800]`
//!
//! Writes `DIR/{look,plain}-z{1,2,4}.png` (the same frame both ways),
//! `DIR/fight.gif` (the fight at zoom 3, look on), `DIR/soldiers-*.png`
//! (the soldier at each zoom), and prints the view pass's GPU time.
//!
//! The soldiers are stand-ins with no game rules (the Larder world owns
//! those): each fires every 0.3 s at a cell of mould within range that the
//! engine's look field says is thick, and the shot is a hit disc the
//! engine cuts (`Engine::set_hits`). The walls are cells taken off the
//! agar (`Engine::block`), so the mould really cannot cross them; the
//! crystals are the dish's piles. Neither drains here: wall health and
//! crystal charge live in the Larder world's CPU dish (PR #64), so the one
//! wall drawn from full to empty along its length is set by hand to show
//! the ramp, and says so on the page.

use pixels::wgpu;
use slime_mould::gpu::*;
use slime_mould::mould::STEPS_PER_SECOND;
use slime_mould::world::look::{dish_crystals, first_hit, ground, ground_buffer, Fields, Ground, Shot};
use slime_mould::world::marks::team_colour;
use slime_mould::world::{Camera, Marks, View};

struct Soldier {
    x: f32,
    y: f32,
    facing: f32,
    /// Seconds to the next shot.
    wait: f32,
    /// Seconds since the last shot.
    since: f32,
    team: u8,
    /// Where it faces when not firing (the arc it fires into).
    facing0: f32,
}

struct Tracer {
    from: (f32, f32),
    to: (f32, f32),
    age: f32,
}

const FLASH: f32 = 0.07;
const TRACER: f32 = 0.09;

fn lowbias(mut v: u32) -> u32 {
    v ^= v >> 16;
    v = v.wrapping_mul(0x7feb352d);
    v ^= v >> 15;
    v = v.wrapping_mul(0x846ca68b);
    v ^ (v >> 16)
}

fn main() {
    let mut out = String::from("worldlook");
    let (mut seconds, mut fight, mut seed, mut walkers) = (12.0f32, 3.0f32, 1u64, 400_000usize);
    let mut screen = (1280u32, 800u32);
    for a in std::env::args().skip(1) {
        let (k, v) = a.split_once('=').expect("key=value");
        match k {
            "out" => out = v.into(),
            "seconds" => seconds = v.parse().unwrap(),
            "fight" => fight = v.parse().unwrap(),
            "seed" => seed = v.parse().unwrap(),
            "walkers" => walkers = v.parse().unwrap(),
            "screen" => {
                let (w, h) = v.split_once('x').unwrap();
                screen = (w.parse().unwrap(), h.parse().unwrap());
            }
            _ => panic!("unknown key {k}"),
        }
    }
    std::fs::create_dir_all(&out).unwrap();
    let instance = wgpu::Instance::new(wgpu::InstanceDescriptor::new_without_display_handle_from_env());
    let adapter = block_on(instance.request_adapter(&wgpu::RequestAdapterOptions { power_preference: wgpu::PowerPreference::HighPerformance, ..Default::default() })).expect("no GPU adapter");
    println!("adapter: {}", adapter.get_info().name);
    let timestamps = adapter.features().contains(wgpu::Features::TIMESTAMP_QUERY);
    let feats = if timestamps { wgpu::Features::TIMESTAMP_QUERY } else { wgpu::Features::empty() };
    let (device, queue) = block_on(adapter.request_device(&wgpu::DeviceDescriptor { label: None, required_features: feats, required_limits: adapter.limits(), ..Default::default() })).expect("no device");
    let shared = Shared::new(&device);
    let s = Settings { seed, walkers, ..Settings::big_dish() };
    let scale = 8usize;
    let mut e = Engine::new(&device, &queue, &shared, &s, scale);
    let (w, h) = (e.dish.w, e.dish.h);
    let sc = scale as f32;
    // The fight's place: the world's middle.
    let (fx, fy) = (w as f32 * 0.5, h as f32 * 0.5);
    // Walls: an L of sandbags in front of the squad, 2 cells thick, and a
    // short wall to its side whose health falls from full to none along
    // it (set by hand: the ramp, not a simulation).
    let mut wall_cells: Vec<(usize, f32)> = Vec::new();
    let mut line = |ax: f32, ay: f32, bx: f32, by: f32, th: f32, ramp: bool| {
        let n = ((bx - ax).hypot(by - ay) * 2.0) as usize;
        for k in 0..=n {
            let t = k as f32 / n as f32;
            let (x, y) = (ax + (bx - ax) * t, ay + (by - ay) * t);
            for dy in -(th as i32)..=th as i32 {
                for dx in -(th as i32)..=th as i32 {
                    let (cx, cy) = ((x + dx as f32 * 0.5) as usize, (y + dy as f32 * 0.5) as usize);
                    if cx < w && cy < h {
                        wall_cells.push((cy * w + cx, if ramp { 1.0 - t } else { 1.0 }));
                    }
                }
            }
        }
    };
    line(fx - 6.0 * sc, fy + 3.0 * sc, fx + 2.0 * sc, fy + 3.0 * sc, 6.0, false);
    line(fx + 2.0 * sc, fy + 3.0 * sc, fx + 2.0 * sc, fy + 7.0 * sc, 6.0, false);
    line(fx - 14.0 * sc, fy - 4.0 * sc, fx - 14.0 * sc, fy + 8.0 * sc, 6.0, true);
    let cells: Vec<usize> = wall_cells.iter().map(|c| c.0).collect();
    e.block(&cells);
    let mut is_wall = vec![false; w * h];
    let mut share = vec![1.0f32; w * h];
    for &(i, v) in &wall_cells {
        is_wall[i] = true;
        share[i] = v;
    }
    // Crystals: the dish's piles.
    let crystal = dish_crystals(&e.dish);
    let codes = ground(w, w * h, &Fields { agar: &e.dish.agar, wall: &is_wall, wall_share: &share, crystal: &crystal, ..Default::default() });
    assert_eq!(codes[cells[0]] & 255, Ground::Wall as u32);
    let ground_buf = ground_buffer(&device, &queue, &codes);

    let run = |e: &mut Engine, frames: u32| {
        for _ in 0..frames {
            e.write_params(&s, Hand::default(), s.ticks);
            let mut enc = device.create_command_encoder(&wgpu::CommandEncoderDescriptor { label: None });
            e.record(&shared, &mut enc, s.ticks, s.curve == Curve::Soft, None);
            queue.submit([enc.finish()]);
        }
        device.poll(wgpu::PollType::wait_indefinitely()).unwrap();
    };
    let fps = STEPS_PER_SECOND as f32 / s.ticks as f32;
    run(&mut e, (seconds * fps) as u32);
    println!("ran {seconds} s: {} walkers on {w} x {h}", e.n);

    // The squad: an arc behind the sandbags, facing up into the mould.
    let mut squad: Vec<Soldier> = (0..9)
        .map(|k| {
            let a = -2.4 + k as f32 * 0.6;
            Soldier { x: fx - 2.0 * sc + a * 2.4 * sc, y: fy + 5.5 * sc + (a * 0.5).powi(2) * sc, facing: -std::f32::consts::FRAC_PI_2, wait: (k as f32 * 0.037) % 0.3, since: 9.0, team: 0, facing0: -std::f32::consts::FRAC_PI_2 }
        })
        .collect();
    squad.push(Soldier { x: fx + 9.0 * sc, y: fy - 1.0 * sc, facing: 2.6, wait: 9e9, since: 9.0, team: 1, facing0: 2.6 });
    let range = 16.0 * sc;
    let mut tracers: Vec<Tracer> = Vec::new();
    let screen_gif = (640u32, 400u32);
    let mut gif_view = View::new(&device, &queue, &e.frame, (w, h), screen_gif);
    gif_view.minimap = false;
    gif_view.set_look(&e.look, &ground_buf, &e.walkers, e.n);
    let mut gif_cam = Camera::new((w, h), screen_gif);
    gif_cam.zoom_to(2.0, (0.0, 0.0));
    gif_cam.centre_on((fx - 2.0 * sc, fy - 2.0 * sc));
    let mut gif = Vec::new();
    let (mut fired, mut landed) = (0usize, 0usize);
    let dt = 1.0 / fps;
    let mut thick: Vec<u8> = Vec::new();
    let fight_frames = (fight * fps) as u32;
    for f in 0..fight_frames {
        if f % 3 == 0 {
            thick = read_back(&device, &queue, &e.look).chunks_exact(4).map(|c| c[0]).collect();
        }
        let mut hits = Vec::new();
        for (k, so) in squad.iter_mut().enumerate() {
            so.since += dt;
            so.wait -= dt;
            if so.wait > 0.0 {
                continue;
            }
            so.wait += 0.3;
            // Aim somewhere in the arc ahead; the shot flies along that line
            // and ends at the first mould on it (`first_hit`; the owner,
            // 16:48Z), or at rock, or at full range.
            let rnd = lowbias(f * 7919 + k as u32 * 104_729);
            let a = so.facing0 + ((rnd & 0xffff) as f32 / 65535.0 - 0.5) * 1.6;
            let muzzle = (so.x + a.cos() * 1.6 * 0.9 * sc, so.y + a.sin() * 1.6 * 0.9 * sc);
            let end = (so.x + a.cos() * range, so.y + a.sin() * range);
            let shot = first_hit(w, h, muzzle, end, |i| thick[i] > 60, |i| !e.dish.agar[i] && !is_wall[i]);
            fired += 1;
            so.facing = a;
            so.since = 0.0;
            let to = match shot {
                Shot::Mould(x, y) => {
                    landed += 1;
                    hits.push((x, y, 0.4 * sc));
                    (x, y)
                }
                Shot::Stopped(x, y) => (x, y),
                Shot::Clear => end,
            };
            tracers.push(Tracer { from: muzzle, to, age: 0.0 });
        }
        e.set_hits(&hits);
        run(&mut e, 1);
        e.set_hits(&[]);
        for t in &mut tracers {
            t.age += dt;
        }
        tracers.retain(|t| t.age < TRACER);
        if f % 2 == 0 {
            let marks = squad_marks(&squad, &tracers, sc, true);
            gif_view.set_marks(&marks);
            let mut enc = device.create_command_encoder(&wgpu::CommandEncoderDescriptor { label: None });
            gif_view.record(&mut enc, &gif_cam, None);
            queue.submit([enc.finish()]);
            gif.push(gif_view.picture());
        }
    }
    println!("fight {fight} s: {fired} shots, {landed} stopped by mould");
    let ms = (2000.0 / fps).round() as u32;
    write_gif(&format!("{out}/fight.gif"), gif, ms);

    // Stills: the same frame with the look on and off, at three zooms.
    let marks_on = squad_marks(&squad, &tracers, sc, true);
    let marks_off = squad_marks(&squad, &tracers, sc, false);
    let mut view = View::new(&device, &queue, &e.frame, (w, h), screen);
    for zoom in [1.0f32, 2.0, 4.0] {
        let mut cam = Camera::new((w, h), screen);
        cam.zoom_to(zoom, (0.0, 0.0));
        cam.centre_on((fx - 2.0 * sc, fy));
        for on in [true, false] {
            if on {
                view.set_look(&e.look, &ground_buf, &e.walkers, e.n);
                view.set_marks(&marks_on);
            } else {
                view.clear_look();
                view.set_marks(&marks_off);
            }
            let t = time_view(&device, &queue, &view, &cam, timestamps);
            let path = format!("{out}/{}-z{}.png", if on { "look" } else { "plain" }, zoom as u32);
            view.picture().save(&path).unwrap();
            println!("{path}: view {t}");
        }
    }
    // A crystal (the first pile) at zoom 3.
    {
        let (px, py) = (e.dish.piles[0].0 * sc, e.dish.piles[0].1 * sc);
        let mut cam = Camera::new((w, h), screen);
        cam.zoom_to(3.0, (0.0, 0.0));
        cam.centre_on((px, py));
        view.set_look(&e.look, &ground_buf, &e.walkers, e.n);
        view.set_marks(&Marks::default());
        let mut enc = device.create_command_encoder(&wgpu::CommandEncoderDescriptor { label: None });
        view.record(&mut enc, &cam, None);
        queue.submit([enc.finish()]);
        view.picture().save(format!("{out}/crystal-z3.png")).unwrap();
    }
    // The soldier alone at each zoom, firing and not, over the world.
    let solo = (360u32, 200u32);
    let mut sv = View::new(&device, &queue, &e.frame, (w, h), solo);
    sv.minimap = false;
    sv.set_look(&e.look, &ground_buf, &e.walkers, e.n);
    let at = (squad[4].x, squad[4].y);
    for zoom in [1.0f32, 2.0, 3.0, 4.0, 6.0] {
        let mut cam = Camera::new((w, h), solo);
        cam.zoom_to(zoom, (0.0, 0.0));
        cam.centre_on(at);
        let mut m = Marks::default();
        m.soldier(at, 0.9 * sc, -1.2, team_colour(0), 0.0, true);
        m.soldier((at.0 + 3.0 * sc, at.1), 0.9 * sc, -0.6, team_colour(0), 1.0, true);
        m.soldier((at.0 - 3.0 * sc, at.1), 0.9 * sc, 2.4, team_colour(1), 0.0, true);
        m.soldier((at.0 - 6.0 * sc, at.1 + 1.0 * sc), 0.9 * sc, 0.5, team_colour(0), 0.0, false);
        sv.set_marks(&m);
        let mut enc = device.create_command_encoder(&wgpu::CommandEncoderDescriptor { label: None });
        sv.record(&mut enc, &cam, None);
        queue.submit([enc.finish()]);
        sv.picture().save(format!("{out}/soldiers-z{}.png", zoom as u32)).unwrap();
        // The same ground with no soldiers, for comparing sprite art on it.
        sv.set_marks(&Marks::default());
        let mut enc = device.create_command_encoder(&wgpu::CommandEncoderDescriptor { label: None });
        sv.record(&mut enc, &cam, None);
        queue.submit([enc.finish()]);
        sv.picture().save(format!("{out}/bare-z{}.png", zoom as u32)).unwrap();
    }
    println!("wrote {out}");
}

fn squad_marks(squad: &[Soldier], tracers: &[Tracer], sc: f32, look: bool) -> Marks {
    let mut m = Marks::default();
    for t in tracers {
        // The shot in flight: its line, fading over its life.
        let a = (255.0 * (1.0 - t.age / TRACER)).clamp(0.0, 255.0) as u8;
        m.line(t.from, t.to, 0.0, [255, 226, 150, a]);
    }
    for so in squad {
        let r = 0.9 * sc;
        if look {
            let flash = (1.0 - so.since / FLASH).max(0.0);
            m.soldier((so.x, so.y), r, so.facing, team_colour(so.team), flash, true);
        } else {
            m.disc((so.x, so.y), r, team_colour(so.team), true);
        }
    }
    m
}

fn time_view(device: &wgpu::Device, queue: &wgpu::Queue, view: &View, cam: &Camera, timestamps: bool) -> String {
    if !timestamps {
        let mut enc = device.create_command_encoder(&wgpu::CommandEncoderDescriptor { label: None });
        view.record(&mut enc, cam, None);
        queue.submit([enc.finish()]);
        return "no timestamps".into();
    }
    let qs = device.create_query_set(&wgpu::QuerySetDescriptor { label: None, ty: wgpu::QueryType::Timestamp, count: 2 });
    let resolve = device.create_buffer(&wgpu::BufferDescriptor { label: None, size: 16, usage: wgpu::BufferUsages::QUERY_RESOLVE | wgpu::BufferUsages::COPY_SRC, mapped_at_creation: false });
    let mut times = Vec::new();
    for _ in 0..21 {
        let mut enc = device.create_command_encoder(&wgpu::CommandEncoderDescriptor { label: None });
        view.record(&mut enc, cam, Some((&qs, Some(0), Some(1))));
        enc.resolve_query_set(&qs, 0..2, &resolve, 0);
        queue.submit([enc.finish()]);
        let t: Vec<u64> = read_back(device, queue, &resolve).chunks_exact(8).map(|c| u64::from_le_bytes(c.try_into().unwrap())).collect();
        times.push(t[1].saturating_sub(t[0]) as f64 * queue.get_timestamp_period() as f64 / 1e6);
    }
    times.sort_by(|a, b| a.partial_cmp(b).unwrap());
    format!("GPU {:.3} ms (median of 21)", times[10])
}

fn write_gif(path: &str, shots: Vec<image::RgbaImage>, ms: u32) {
    let delay = image::Delay::from_numer_denom_ms(ms, 1);
    let frames: Vec<image::Frame> = shots.into_iter().map(|img| image::Frame::from_parts(img, 0, 0, delay)).collect();
    let file = std::fs::File::create(path).expect("gif file");
    let mut enc = image::codecs::gif::GifEncoder::new_with_speed(file, 10);
    enc.set_repeat(image::codecs::gif::Repeat::Infinite).unwrap();
    enc.encode_frames(frames).expect("gif encode");
}
