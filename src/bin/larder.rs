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
//! - 1 hold fire, 2 defend (the default: only mould near them), 3 free fire.
//! - Space pauses; Tab runs four times faster; R restarts; Esc quits.
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
use slime_mould::mould::larder::{field_game, Game, Order, Rules, Stance, GAME_SECOND};
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
    let seed = std::env::args().nth(1).and_then(|s| s.parse().ok()).unwrap_or(1);
    let event_loop = EventLoop::new()?;
    event_loop.set_control_flow(ControlFlow::Poll);
    let mut app = App::new(seed);
    event_loop.run_app(&mut app)?;
    Ok(())
}

struct App {
    seed: u64,
    game: Game,
    window: Option<Arc<Window>>,
    pixels: Option<Pixels<'static>>,
    selected: Vec<usize>,
    /// Dish cells.
    cursor: Option<(f32, f32)>,
    drag_from: Option<(f32, f32)>,
    f_held: bool,
    paused: bool,
    fast: bool,
    last: Instant,
    owed: Duration,
}

impl App {
    fn new(seed: u64) -> App {
        App {
            seed,
            game: field_game(seed, Rules::default()),
            window: None,
            pixels: None,
            selected: Vec::new(),
            cursor: None,
            drag_from: None,
            f_held: false,
            paused: false,
            fast: false,
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
                self.game.order(k, Order::Burn(from, c));
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
            self.game.order(k, o);
        }
    }

    fn key(&mut self, code: KeyCode, el: &ActiveEventLoop) {
        match code {
            KeyCode::Escape => el.exit(),
            KeyCode::Space => self.paused = !self.paused,
            KeyCode::Tab => self.fast = !self.fast,
            KeyCode::KeyR => {
                self.game = field_game(self.seed, Rules::default());
                self.selected.clear();
            }
            KeyCode::KeyB => {
                self.game.buy();
            }
            KeyCode::KeyT => {
                if let Some(c) = self.cursor {
                    self.game.build_turret(c);
                }
            }
            KeyCode::KeyG => {
                if let Some(c) = self.cursor {
                    self.game.bait(c, BAIT);
                }
            }
            KeyCode::KeyA => {
                for &k in &self.selected {
                    self.game.order(k, Order::Attack);
                }
            }
            KeyCode::Digit1 | KeyCode::Digit2 | KeyCode::Digit3 => {
                let st = match code {
                    KeyCode::Digit1 => Stance::HoldFire,
                    KeyCode::Digit2 => Stance::Defend,
                    _ => Stance::Free,
                };
                for &k in &self.selected {
                    self.game.stance(k, st);
                }
            }
            _ => {}
        }
    }

    fn frame(&mut self, el: &ActiveEventLoop) {
        let now = Instant::now();
        self.owed += now - self.last;
        self.last = now;
        let step = if self.fast { STEP / FAST } else { STEP };
        let mut n = 0;
        while self.owed >= step && n < 64 {
            self.owed -= step;
            n += 1;
            if !self.paused {
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
    }

    fn title(&self) -> String {
        format!(
            "LARDER (seed {}) - {}",
            self.seed,
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
        None => "R-CLICK PILE: CLEAR  R-CLICK: MOVE  F+DRAG: BURN  A: CORE  1/2/3: HOLD/DEFEND/FREE".into(),
    };
    hud::draw_text_scaled(frame, (w, h), s, (4, y + 10), &line2, [170, 175, 160, 255]);
    let line3 = if g.core_down.is_some() {
        "THE MOULD'S CORE IS TAKEN: IT CAN NO LONGER DIVIDE"
    } else {
        "B: SOLDIER 400  T: TURRET 600  G: BAIT 300  SPACE: PAUSE  TAB: FAST  R: RESTART"
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
