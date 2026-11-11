//! **Larder: a squad against the mould**, the rough window (build plan
//! milestone M1b, `Reports/larder-build-plan-2026-11-11.md`).
//! `cargo run --release --bin larder [-- SEED]`
//!
//! The game is `slime_mould::mould::larder`; this file is winit, `pixels`
//! and the squad's marks. A rough window on purpose, so the owner sees play
//! before the full controls (M3): the design review (2026-11-11, finding 9)
//! asked for a look at play right after M1.
//!
//! **Controls** (RTS-style, owner 03:14Z):
//! - Left click a soldier to select it; left drag a box to select several;
//!   left click on empty ground clears the selection.
//! - Right click: on a pile, the selected soldiers clear it, hold it and
//!   carry its food to the larder; on ground, they walk there.
//! - Hold F and left drag: the selected soldiers hold that line burning.
//! - A: attack the mould's core (its home pile, on the left).
//! - Z hold fire, X defend (the default: only mould near them), C free fire.
//!   (M1b had these on 1/2/3; M3 gives the digits to groups.)
//! - Ctrl+1..9 makes the selection a group; 1..9 selects that group.
//! - V shows the scent field the mould follows (the dish's own `scent`).
//! - Space pauses; Tab runs four times faster; R restarts; Esc quits.
//!   R and Esc save the round's commands to `larder-replay-SEED.txt`;
//!   `cargo run --release --bin larder -- replay FILE` plays one back with
//!   no window and prints the dish hash it reaches (build plan M3).
//!   `-- shot REPLAY OUT.png [scent]` saves the window's frame at the
//!   replay's end; `-- bench` times a frame's work with 40 soldiers.
//! - **Watch the AI** (`larder_ai`): `-- SEED ai=STYLE` lets a player AI
//!   (`adapt`, `turtle`, `boom`, `rush`, `lure`, with knobs after a colon,
//!   as `examples/larderai.rs` takes them) play the round, through the
//!   same commands; your own commands still go in beside it, and R or Esc
//!   saves the round as a replay as usual.
//! - Spending (M2): B buys a soldier at the larder (400 food); T builds a
//!   turret under the cursor (600, within a soldier's range); G lays 300
//!   food of bait under the cursor (within a soldier's range).
//!
//! **What is drawn.** The dish exactly as the simulation has it (the game's
//! look, `View::bare`): a shot is the wound it leaves, nothing painted.
//! Soldiers are 3-cell marks, white, ringed in yellow when selected, with
//! a health bar under each that is its `hp`; a burn order's line is drawn
//! as a thin dotted guide, because it is an order you gave, not an effect.
//!
//! **Time.** One game second is `larder::GAME_SECOND` dish steps (15), so
//! the window runs a fractional number of steps a frame: an accumulator
//! steps the game whenever a step's worth of time has passed (engineering
//! review, finding 2: the herding window steps 2 a frame).

use std::sync::Arc;
use std::time::{Duration, Instant};

use pixels::{Pixels, ScalingMode, SurfaceTexture};
use slime_mould::hud;
use slime_mould::mould::larder::{field_game, Cmd, Game, Order, Rules, Stance, GAME_SECOND};
use slime_mould::mould::larder_ai::Hand;
use slime_mould::mould::look::Style;
use slime_mould::mould::render::{self, View};
use slime_mould::mould::{H, W};
use winit::application::ApplicationHandler;
use winit::dpi::LogicalSize;
use winit::event::{ElementState, MouseButton, WindowEvent};
use winit::event_loop::{ActiveEventLoop, ControlFlow, EventLoop};
use winit::keyboard::{KeyCode, PhysicalKey};
use winit::window::{Window, WindowId};

const SCALE: u32 = 4;
/// One dish step of game time at normal speed.
const STEP: Duration = Duration::from_nanos(1_000_000_000 / GAME_SECOND);
const FAST: u32 = 4;
/// Food a G press lays as bait (the baiter plan's size, `larderm2`).
const BAIT: f32 = 300.0;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let args: Vec<String> = std::env::args().skip(1).collect();
    if args.first().map(String::as_str) == Some("replay") {
        let path = args.get(1).ok_or("replay FILE")?;
        let text = std::fs::read_to_string(path)?;
        let g = Game::replay(&text, Rules::default()).ok_or("not a replay file")?;
        println!(
            "replayed {} commands to step {} ({:.1} game s): dish hash {:016x}, {} soldiers alive, bank {:.0}",
            g.log.len(),
            g.dish.t,
            g.time(),
            g.dish.state_hash(),
            g.alive(),
            g.bank
        );
        return Ok(());
    }
    if args.first().map(String::as_str) == Some("bench") {
        // The M3 gate's frame with no window: 40 soldiers on free fire,
        // one game step and one full draw (dish, squad, HUD, and the scent
        // view in the second pass) a frame, as at Tab speed, for 60 game
        // seconds. Times the work the window does a frame, not the swap to
        // the screen.
        for scent in [false, true] {
            let rules = Rules { squad: 40, start_bank: 1e9, ..Rules::default() };
            let mut g = field_game(1, rules);
            for k in 0..40 {
                g.apply(Cmd::Stance(k, Stance::Free));
            }
            let (w, h) = render::frame_size(SCALE);
            let mut frame = vec![0u8; (w * h * 4) as usize];
            let view = View { bare: true, keys: false, style: Style::GAME, ..View::default() };
            let all: Vec<usize> = (0..40).collect();
            let mut times = Vec::new();
            let mut parts = [0.0f64; 4];
            for _ in 0..60 * GAME_SECOND {
                let t0 = Instant::now();
                g.step();
                let t1 = Instant::now();
                render::draw(&g.dish, &view, SCALE, &mut frame);
                let t2 = Instant::now();
                if scent {
                    draw_scent(&g, &mut frame);
                }
                let t3 = Instant::now();
                draw_squad(&g, &all, None, false, &mut frame);
                draw_hud(&g, false, true, &mut frame);
                let t4 = Instant::now();
                for (p, d) in parts.iter_mut().zip([t1 - t0, t2 - t1, t3 - t2, t4 - t3]) {
                    *p += d.as_secs_f64() * 1e3;
                }
                times.push(t0.elapsed().as_secs_f64() * 1e3);
            }
            let n = times.len() as f64;
            times.sort_by(|a, b| a.total_cmp(b));
            println!(
                "40 soldiers, scent view {}: frame mean {:.2} ms, p95 {:.2} ms, max {:.2} ms over {} frames (game step {:.2}, dish draw {:.2}, scent {:.2}, squad and HUD {:.2}); {} alive, {} walkers at the end",
                if scent { "on" } else { "off" },
                times.iter().sum::<f64>() / n,
                times[times.len() * 95 / 100],
                times[times.len() - 1],
                times.len(),
                parts[0] / n,
                parts[1] / n,
                parts[2] / n,
                parts[3] / n,
                g.alive(),
                g.dish.walkers.len()
            );
        }
        return Ok(());
    }
    if args.first().map(String::as_str) == Some("shot") {
        // The window's exact frame, with no window: replay a file and
        // draw what the window would show at its end (for pages, and to
        // look at the HUD and the scent view in a container with no
        // display). `shot REPLAY OUT.png [scent]`.
        let path = args.get(1).ok_or("shot REPLAY OUT.png [scent]")?;
        let out = args.get(2).ok_or("shot REPLAY OUT.png [scent]")?;
        let g = Game::replay(&std::fs::read_to_string(path)?, Rules::default()).ok_or("not a replay file")?;
        let (w, h) = render::frame_size(SCALE);
        let mut frame = vec![0u8; (w * h * 4) as usize];
        let view = View {
            bare: true,
            keys: false,
            style: Style::GAME,
            ..View::default()
        };
        render::draw(&g.dish, &view, SCALE, &mut frame);
        if args.get(3).map(String::as_str) == Some("scent") {
            draw_scent(&g, &mut frame);
        }
        let all: Vec<usize> = (0..g.soldiers.len()).filter(|&k| g.soldiers[k].alive).collect();
        draw_squad(&g, &all, None, false, &mut frame);
        draw_hud(&g, false, false, &mut frame);
        image::RgbaImage::from_raw(w, h, frame).ok_or("frame size")?.save(out)?;
        println!("{out}: step {}, {} soldiers, {} walkers", g.dish.t, g.alive(), g.dish.walkers.len());
        return Ok(());
    }
    let seed = args.first().and_then(|s| s.parse().ok()).unwrap_or(1);
    let ai = args.iter().find_map(|a| a.strip_prefix("ai=")).map(String::from);
    if let Some(name) = &ai {
        Hand::named(name, &field_game(seed, Rules::default()), seed).ok_or("ai=adapt|turtle|boom|rush|lure[:knob=v+...]")?;
    }
    let event_loop = EventLoop::new()?;
    event_loop.set_control_flow(ControlFlow::Poll);
    let mut app = App::new(seed, ai);
    event_loop.run_app(&mut app)?;
    Ok(())
}

struct App {
    seed: u64,
    game: Game,
    /// The player AI playing this round, if `ai=` was given, and its name.
    hand: Option<Hand>,
    ai: Option<String>,
    window: Option<Arc<Window>>,
    pixels: Option<Pixels<'static>>,
    selected: Vec<usize>,
    /// Dish cells.
    cursor: Option<(f32, f32)>,
    drag_from: Option<(f32, f32)>,
    f_held: bool,
    ctrl: bool,
    paused: bool,
    fast: bool,
    scent: bool,
    groups: [Vec<usize>; 9],
    last: Instant,
    owed: Duration,
    /// Frame times since the last report, for the M3 gate (frame under
    /// 10 ms with 40 soldiers): a line to stdout every 5 s.
    frames: Vec<Duration>,
    reported: Instant,
}

impl App {
    fn new(seed: u64, ai: Option<String>) -> App {
        let game = field_game(seed, Rules::default());
        App {
            seed,
            hand: ai.as_deref().and_then(|n| Hand::named(n, &game, seed)),
            ai,
            game,
            window: None,
            pixels: None,
            selected: Vec::new(),
            cursor: None,
            drag_from: None,
            f_held: false,
            ctrl: false,
            paused: false,
            fast: false,
            scent: false,
            groups: Default::default(),
            frames: Vec::new(),
            reported: Instant::now(),
            last: Instant::now(),
            owed: Duration::ZERO,
        }
    }

    fn soldier_at(&self, p: (f32, f32)) -> Option<usize> {
        self.game
            .soldiers
            .iter()
            .enumerate()
            .filter(|(_, s)| s.alive)
            .map(|(k, s)| (k, (s.x - p.0).hypot(s.y - p.1)))
            .filter(|&(_, d)| d <= 4.0)
            .min_by(|a, b| a.1.total_cmp(&b.1))
            .map(|(k, _)| k)
    }

    fn left(&mut self, pressed: bool) {
        let Some(c) = self.cursor else { return };
        if pressed {
            self.drag_from = Some(c);
            return;
        }
        let Some(from) = self.drag_from.take() else { return };
        let far = (c.0 - from.0).hypot(c.1 - from.1) > 2.0;
        if self.f_held && far {
            for &k in &self.selected {
                self.game.apply(Cmd::Order(k, Order::Burn(from, c)));
            }
        } else if far {
            let (x0, x1) = (from.0.min(c.0), from.0.max(c.0));
            let (y0, y1) = (from.1.min(c.1), from.1.max(c.1));
            self.selected = (0..self.game.soldiers.len())
                .filter(|&k| {
                    let s = self.game.soldiers[k];
                    s.alive && s.x >= x0 && s.x <= x1 && s.y >= y0 && s.y <= y1
                })
                .collect();
        } else {
            self.selected = self.soldier_at(c).into_iter().collect();
        }
    }

    fn right(&mut self) {
        let Some(c) = self.cursor else { return };
        let pile = self
            .game
            .dish
            .piles
            .iter()
            .find(|p| (p.x - c.0).hypot(p.y - c.1) <= p.r + 1.5)
            .map(|p| (p.x, p.y));
        for &k in &self.selected {
            let o = match pile {
                Some(p) => Order::Clear(p.0, p.1),
                None => Order::Move(c.0, c.1),
            };
            self.game.apply(Cmd::Order(k, o));
        }
    }

    fn save_replay(&self) {
        if self.game.log.is_empty() {
            return;
        }
        let path = format!("larder-replay-{}.txt", self.seed);
        match std::fs::write(&path, self.game.replay_text(self.seed)) {
            Ok(()) => println!("saved {} commands to {path} (dish hash {:016x} at step {})", self.game.log.len(), self.game.dish.state_hash(), self.game.dish.t),
            Err(e) => eprintln!("could not save {path}: {e}"),
        }
    }

    fn key(&mut self, code: KeyCode, el: &ActiveEventLoop) {
        match code {
            KeyCode::Escape => {
                self.save_replay();
                el.exit();
            }
            KeyCode::KeyV => self.scent = !self.scent,
            KeyCode::Space => self.paused = !self.paused,
            KeyCode::Tab => self.fast = !self.fast,
            KeyCode::KeyR => {
                self.save_replay();
                self.game = field_game(self.seed, Rules::default());
                self.hand = self.ai.as_deref().and_then(|n| Hand::named(n, &self.game, self.seed));
                self.selected.clear();
            }
            KeyCode::KeyB => {
                self.game.apply(Cmd::Buy);
            }
            KeyCode::KeyT => {
                if let Some(c) = self.cursor {
                    self.game.apply(Cmd::Turret(c));
                }
            }
            KeyCode::KeyG => {
                if let Some(c) = self.cursor {
                    self.game.apply(Cmd::Bait(c, BAIT));
                }
            }
            KeyCode::KeyA => {
                for &k in &self.selected {
                    self.game.apply(Cmd::Order(k, Order::Attack));
                }
            }
            KeyCode::KeyZ | KeyCode::KeyX | KeyCode::KeyC => {
                let st = match code {
                    KeyCode::KeyZ => Stance::HoldFire,
                    KeyCode::KeyX => Stance::Defend,
                    _ => Stance::Free,
                };
                for &k in &self.selected {
                    self.game.apply(Cmd::Stance(k, st));
                }
            }
            KeyCode::Digit1
            | KeyCode::Digit2
            | KeyCode::Digit3
            | KeyCode::Digit4
            | KeyCode::Digit5
            | KeyCode::Digit6
            | KeyCode::Digit7
            | KeyCode::Digit8
            | KeyCode::Digit9 => {
                let g = [
                    KeyCode::Digit1,
                    KeyCode::Digit2,
                    KeyCode::Digit3,
                    KeyCode::Digit4,
                    KeyCode::Digit5,
                    KeyCode::Digit6,
                    KeyCode::Digit7,
                    KeyCode::Digit8,
                    KeyCode::Digit9,
                ]
                .iter()
                .position(|&d| d == code)
                .expect("a digit");
                if self.ctrl {
                    self.groups[g] = self.selected.clone();
                } else {
                    self.selected = self.groups[g].iter().copied().filter(|&k| self.game.soldiers[k].alive).collect();
                }
            }
            _ => {}
        }
    }

    fn frame(&mut self, el: &ActiveEventLoop) {
        let now = Instant::now();
        let started = now;
        self.owed += now - self.last;
        self.last = now;
        let step = if self.fast { STEP / FAST } else { STEP };
        let mut n = 0;
        while self.owed >= step && n < 64 {
            self.owed -= step;
            n += 1;
            if !self.paused {
                if let Some(h) = self.hand.as_mut() {
                    if self.game.dish.t.is_multiple_of(GAME_SECOND) {
                        h.play(&mut self.game);
                    }
                }
                self.game.step();
            }
        }
        if n == 64 {
            self.owed = Duration::ZERO;
        }
        self.selected.retain(|&k| self.game.soldiers[k].alive);
        if let Some(w) = &self.window {
            w.set_title(&self.title());
        }
        let Some(pixels) = self.pixels.as_mut() else { return };
        let view = View {
            bare: true,
            keys: false,
            style: Style::GAME,
            ..View::default()
        };
        let frame = pixels.frame_mut();
        render::draw(&self.game.dish, &view, SCALE, frame);
        if self.scent {
            draw_scent(&self.game, frame);
        }
        draw_squad(
            &self.game,
            &self.selected,
            self.drag_from.zip(self.cursor),
            self.f_held,
            frame,
        );
        draw_hud(&self.game, self.paused, self.fast, frame);
        if let Err(e) = pixels.render() {
            eprintln!("render failed: {e}");
            el.exit();
        }
        // Simulation steps plus drawing: what the M3 gate measures.
        self.frames.push(started.elapsed());
        if self.reported.elapsed() >= Duration::from_secs(5) && !self.frames.is_empty() {
            let mut f: Vec<f64> = self.frames.iter().map(|d| d.as_secs_f64() * 1e3).collect();
            f.sort_by(|a, b| a.total_cmp(b));
            let mean = f.iter().sum::<f64>() / f.len() as f64;
            println!(
                "{:.0} game s: {} soldiers, {} walkers; frame mean {mean:.2} ms, p95 {:.2} ms, max {:.2} ms over {} frames{}",
                self.game.time(),
                self.game.alive(),
                self.game.dish.walkers.len(),
                f[f.len() * 95 / 100],
                f[f.len() - 1],
                f.len(),
                if self.fast { " (fast)" } else { "" }
            );
            self.frames.clear();
            self.reported = Instant::now();
        }
    }

    fn title(&self) -> String {
        format!(
            "LARDER (seed {}{}) - {}",
            self.seed,
            self.ai.as_deref().map(|n| format!(", AI {n}")).unwrap_or_default(),
            if self.paused { "paused" } else { "running" }
        )
    }
}

fn put(frame: &mut [u8], x: i32, y: i32, c: [u8; 4]) {
    let (w, h) = render::frame_size(SCALE);
    if x >= 0 && y >= 0 && (x as u32) < w && (y as u32) < h {
        let i = ((y as u32 * w + x as u32) * 4) as usize;
        frame[i..i + 4].copy_from_slice(&c);
    }
}

fn draw_squad(g: &Game, selected: &[usize], drag: Option<((f32, f32), (f32, f32))>, burn: bool, frame: &mut [u8]) {
    let s_ = SCALE as f32;
    // Burn orders: a dotted guide on the line each soldier was told to hold.
    for s in g.soldiers.iter().filter(|s| s.alive) {
        if let Order::Burn(a, b) = s.order {
            dotted(frame, a, b, [255, 150, 60, 255]);
        }
    }
    // Turrets: a grey square, its hp as a bar under it.
    for t in &g.turrets {
        let (cx, cy) = ((t.x * s_) as i32, (t.y * s_) as i32);
        for dy in -4..=4 {
            for dx in -4..=4 {
                put(frame, cx + dx, cy + dy, [150, 160, 175, 255]);
            }
        }
        let w = (t.hp / g.rules.turret_hp * 12.0).round() as i32;
        for dx in -6..6 {
            let c = if dx + 6 < w { [90, 220, 110, 255] } else { [70, 30, 30, 255] };
            put(frame, cx + dx, cy + 7, c);
        }
    }
    for (k, s) in g.soldiers.iter().enumerate() {
        let (cx, cy) = ((s.x * s_) as i32, (s.y * s_) as i32);
        let col = if s.alive {
            [245, 245, 240, 255]
        } else {
            [120, 30, 30, 255]
        };
        for dy in -5..=5 {
            for dx in -5..=5 {
                if dx * dx + dy * dy <= 25 {
                    put(frame, cx + dx, cy + dy, col);
                }
            }
        }
        if !s.alive {
            continue;
        }
        if selected.contains(&k) {
            for a in 0..48 {
                let t = a as f32 / 48.0 * std::f32::consts::TAU;
                put(
                    frame,
                    cx + (8.0 * t.cos()) as i32,
                    cy + (8.0 * t.sin()) as i32,
                    [255, 230, 80, 255],
                );
            }
        }
        // Health: its hp, as a bar under the mark.
        let w = (s.hp / g.rules.hp * 12.0).round() as i32;
        for dx in -6..6 {
            let c = if dx + 6 < w {
                [90, 220, 110, 255]
            } else {
                [70, 30, 30, 255]
            };
            put(frame, cx + dx, cy + 8, c);
            put(frame, cx + dx, cy + 9, c);
        }
    }
    if let Some((a, b)) = drag {
        if burn {
            dotted(frame, a, b, [255, 120, 40, 255]);
        } else {
            let (x0, x1) = ((a.0.min(b.0) * s_) as i32, (a.0.max(b.0) * s_) as i32);
            let (y0, y1) = ((a.1.min(b.1) * s_) as i32, (a.1.max(b.1) * s_) as i32);
            for x in x0..=x1 {
                put(frame, x, y0, [255, 230, 80, 255]);
                put(frame, x, y1, [255, 230, 80, 255]);
            }
            for y in y0..=y1 {
                put(frame, x0, y, [255, 230, 80, 255]);
                put(frame, x1, y, [255, 230, 80, 255]);
            }
        }
    }
}

/// The scent field over the dish (V): the dish's own `scent`, the smell
/// the mould climbs (piles, soldiers, the bank, drops about to land), as a
/// violet wash, log-scaled to the field's maximum. A view of the data,
/// not an effect.
fn draw_scent(g: &Game, frame: &mut [u8]) {
    let sc = &g.dish.scent;
    let max = sc.iter().cloned().fold(0.0f32, f32::max);
    if max <= 0.0 {
        return;
    }
    let lmax = (1.0 + max).ln();
    let (fw, _) = render::frame_size(SCALE);
    for y in 0..H {
        for x in 0..W {
            let v = sc[y * W + x];
            if v <= 0.0 {
                continue;
            }
            let a = ((1.0 + v).ln() / lmax).clamp(0.0, 1.0) * 0.8;
            for dy in 0..SCALE {
                for dx in 0..SCALE {
                    let i = (((y as u32 * SCALE + dy) * fw + x as u32 * SCALE + dx) * 4) as usize;
                    for (c, t) in frame[i..i + 3].iter_mut().zip([190.0f32, 120.0, 255.0]) {
                        *c = (*c as f32 * (1.0 - a) + t * a) as u8;
                    }
                }
            }
        }
    }
}

fn dotted(frame: &mut [u8], a: (f32, f32), b: (f32, f32), c: [u8; 4]) {
    let s_ = SCALE as f32;
    let n = ((b.0 - a.0).hypot(b.1 - a.1) * s_) as i32;
    for i in (0..=n).step_by(4) {
        let f = i as f32 / n.max(1) as f32;
        put(
            frame,
            ((a.0 + (b.0 - a.0) * f) * s_) as i32,
            ((a.1 + (b.1 - a.1) * f) * s_) as i32,
            c,
        );
    }
}

fn draw_hud(g: &Game, paused: bool, fast: bool, frame: &mut [u8]) {
    let (w, h) = render::frame_size(SCALE);
    // The rows under the dish are left blank by a bare view: paint them.
    let top = (H as u32 * SCALE * w * 4) as usize;
    for px in frame[top..].chunks_exact_mut(4) {
        px.copy_from_slice(&[14, 16, 12, 255]);
    }
    // `draw_text_scaled` takes logical pixels (half the frame's at scale
    // 2). M1b passed frame pixels, which put every line below the frame.
    let y = (H as u32 * SCALE / 2) as i32 + 4;
    let t = g.time();
    let clock = format!("{}:{:02}", t as u32 / 60, t as u32 % 60);
    let line1 = format!(
        "FOOD {:.0}   SOLDIERS {}/{}   MOULD {}   {}{}{}",
        g.bank,
        g.alive(),
        g.soldiers.len(),
        g.dish.walkers.len(),
        clock,
        if paused { "   PAUSED" } else { "" },
        if fast { "   FAST" } else { "" },
    );
    let s = 2;
    hud::draw_text_scaled(frame, (w, h), s, (4, y), &line1, [235, 235, 220, 255]);
    let line2 = match g.end {
        Some(e) => format!(
            "ROUND OVER: {} - SCORE {:.2} - R TO RESTART",
            e.band().to_uppercase(),
            g.score().unwrap_or(0.0)
        ),
        None => "R-CLICK: CLEAR/MOVE  F+DRAG: BURN  A: CORE  Z/X/C: HOLD/DEFEND/FREE  CTRL+1-9: GROUP".into(),
    };
    hud::draw_text_scaled(frame, (w, h), s, (4, y + 10), &line2, [170, 175, 160, 255]);
    let line3 = if g.core_down.is_some() {
        "THE MOULD'S CORE IS TAKEN: IT CAN NO LONGER DIVIDE"
    } else {
        "B: SOLDIER 400  T: TURRET 600  G: BAIT 300  V: SCENT  SPACE: PAUSE  TAB: FAST"
    };
    hud::draw_text_scaled(frame, (w, h), s, (4, y + 20), line3, [170, 175, 160, 255]);
}

impl ApplicationHandler for App {
    fn resumed(&mut self, el: &ActiveEventLoop) {
        if self.window.is_some() {
            return;
        }
        let (w, h) = render::frame_size(SCALE);
        let attrs = Window::default_attributes()
            .with_title(self.title())
            .with_inner_size(LogicalSize::new(w, h))
            .with_min_inner_size(LogicalSize::new(W as u32 * 2, H as u32 * 2));
        let window = match el.create_window(attrs) {
            Ok(w) => Arc::new(w),
            Err(e) => {
                eprintln!("window: {e}");
                return el.exit();
            }
        };
        let size = window.inner_size();
        let surface = SurfaceTexture::new(size.width, size.height, Arc::clone(&window));
        match Pixels::new(w, h, surface) {
            Ok(mut p) => {
                p.set_scaling_mode(scaling_for(size.width, size.height));
                self.pixels = Some(p);
            }
            Err(e) => {
                eprintln!("pixels: {e}");
                return el.exit();
            }
        }
        self.window = Some(window);
        self.last = Instant::now();
    }

    fn window_event(&mut self, el: &ActiveEventLoop, _id: WindowId, event: WindowEvent) {
        match event {
            WindowEvent::CloseRequested => el.exit(),
            WindowEvent::Resized(size) if size.width > 0 && size.height > 0 => {
                if let Some(p) = self.pixels.as_mut() {
                    p.set_scaling_mode(scaling_for(size.width, size.height));
                    let _ = p.resize_surface(size.width, size.height);
                }
            }
            WindowEvent::KeyboardInput { event, .. } => {
                if let PhysicalKey::Code(code) = event.physical_key {
                    if code == KeyCode::KeyF {
                        self.f_held = event.state == ElementState::Pressed;
                    } else if matches!(code, KeyCode::ControlLeft | KeyCode::ControlRight) {
                        self.ctrl = event.state == ElementState::Pressed;
                    } else if event.state == ElementState::Pressed && !event.repeat {
                        self.key(code, el);
                    }
                }
            }
            WindowEvent::CursorMoved { position, .. } => {
                self.cursor = self
                    .pixels
                    .as_ref()
                    .and_then(|p| p.window_pos_to_pixel((position.x as f32, position.y as f32)).ok())
                    .map(|(x, y)| (x as f32 / SCALE as f32, y as f32 / SCALE as f32))
                    .filter(|&(_, y)| y < H as f32);
            }
            WindowEvent::MouseInput { state, button, .. } => match button {
                MouseButton::Left => self.left(state == ElementState::Pressed),
                MouseButton::Right if state == ElementState::Pressed => self.right(),
                _ => {}
            },
            WindowEvent::RedrawRequested => self.frame(el),
            _ => {}
        }
    }

    fn about_to_wait(&mut self, _el: &ActiveEventLoop) {
        if let Some(w) = &self.window {
            w.request_redraw();
        }
    }
}

fn scaling_for(width: u32, height: u32) -> ScalingMode {
    if render::frame_fits(width, height, SCALE) {
        ScalingMode::PixelPerfect
    } else {
        ScalingMode::Fill
    }
}
