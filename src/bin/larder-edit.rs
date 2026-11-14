//! **Larder World's map editor** (ROADMAP WM5, plan C10 of 2026-11-14):
//! paint the ground, place crystals, the mould's starts, the squad's
//! landing and walls, save to the WM1 map files, and play the map.
//!
//! `cargo run --release --bin larder-edit [-- map=fork name=my-fork]`
//!
//! - `map=` opens a designed map (fork, nest, rim, flats, passes), a saved
//!   one (`map=DIR/NAME`), `noise` (the race world from `seed=`, as a map to
//!   edit), or nothing for a blank map.
//! - `name=` is the saved map's name (default the opened map's, or `new`)
//!   and `dir=` its folder (default `maps/` in the repo). The designed maps
//!   in `Reports/larder-maps-2026-11-11/maps/` are never written unless
//!   `dir=` names that folder.
//! - Enter saves and plays: it starts `larder-world map=DIR/NAME` beside
//!   the editor (the binary next to this one, or `cargo run` if that one
//!   is not built), with any other `key=value` given here passed on, such
//!   as `seed=3` or `tier=sleeper`. The editor stays open, so a change is
//!   one Enter away from the next try.
//!
//! **Why its own binary.** `larder-world` is the game, and a start menu is
//! being built at its front door in parallel (plan C2). The editor needs
//! none of the game's state, only the map files, so keeping it apart means
//! the two never fight over one entry point; the menu can start this
//! binary for an "Edit maps" item. The editing itself is
//! `slime_mould::mould::map_edit`, tested without a window.
//!
//! **What is drawn is the file.** Each pixel is the map cell under it, in
//! the world view's own ground colours (`view.wgsl`), crystals as the
//! engine's hexagons, so the map looks here as it will at the start of a
//! round, before any mould is on it.
//!
//! Headless, for checks without a display: `look=FILE.png` saves the
//! window's picture of the opened map; `demo=DIR` draws a small mirrored
//! map from blank, as a hand would, saving a frame after each step
//! (`DIR/frame-NNN.png`) and the map itself (`DIR/map/NAME`).

use std::path::{Path, PathBuf};
use std::sync::Arc;

use pixels::{Pixels, PixelsBuilder, SurfaceTexture};
use slime_mould::mould::larder_world::race_world;
use slime_mould::mould::map::{Map, MAP_H, MAP_W};
use slime_mould::mould::map_edit::{self, Brush, EditView, Editor, Mirror, Overlay, Tool};
use slime_mould::world::settings::{self, UiSize};
use winit::application::ApplicationHandler;
use winit::dpi::PhysicalSize;
use winit::event::{ElementState, MouseButton, MouseScrollDelta, WindowEvent};
use winit::event_loop::{ActiveEventLoop, ControlFlow, EventLoop};
use winit::keyboard::{KeyCode, PhysicalKey};
use winit::window::{Window, WindowId};

const DESIGNED: &str = "Reports/larder-maps-2026-11-11/maps";

fn repo() -> PathBuf {
    PathBuf::from(env!("CARGO_MANIFEST_DIR"))
}

/// A `map=` value to a map: a designed map's name, `DIR/NAME`, or `noise`.
fn open(spec: &str, seed: u64) -> Result<Map, String> {
    if spec == "noise" {
        // The race world the censuses measured, at scale 2 (its own map
        // cells as `Map::of_dish` takes them).
        return Ok(Map::of_dish(&race_world(seed, 2, 5000)));
    }
    let (dir, name) = match spec.rsplit_once(['/', '\\']) {
        Some((d, n)) => (PathBuf::from(d), n.to_string()),
        None => (repo().join(DESIGNED), spec.to_string()),
    };
    Map::load(&dir, &name)
}

/// The view that fits the whole map beside the panel and above the status
/// line, at UI size `ui`.
fn fit(size: (u32, u32), ui: f32) -> EditView {
    let (panel, status) = map_edit::chrome(ui);
    let left = panel + 8.0;
    EditView::fit((left, 8.0, (size.0 as f32 - left - 8.0).max(64.0), (size.1 as f32 - status - 16.0).max(64.0)))
}

/// The font has no lower-case-only marks such as `_` or `\`; a message
/// (often a path) says them as the nearest it has.
fn printable(s: &str) -> String {
    s.chars().map(|c| if slime_mould::hud::has_crisp_glyph(c) { c } else if c == '\\' { '/' } else { '-' }).collect()
}

struct App {
    ed: Editor,
    name: String,
    dir: PathBuf,
    /// `key=value`s passed on to `larder-world` when playing.
    play_keys: Vec<String>,
    view: EditView,
    window: Option<Arc<Window>>,
    pixels: Option<Pixels<'static>>,
    size: (u32, u32),
    cursor: Option<(f32, f32)>,
    /// The left button is down: the last map point of the stroke, or a
    /// wall's start.
    stroke: Option<(f32, f32)>,
    wall_from: Option<(f32, f32)>,
    /// The right or middle button is down: the last screen point.
    pan: Option<(f32, f32)>,
    ctrl: bool,
    message: String,
    /// N was pressed once on a changed map: a second N clears it.
    clear_armed: bool,
    /// The UI's size (the saved or `ui=` size, the window's DPI scale).
    ui: UiSize,
    result: Result<(), String>,
}

impl App {
    fn new(ed: Editor, name: String, dir: PathBuf, play_keys: Vec<String>, user: f32) -> Self {
        let size = (1280, 800);
        let ui = UiSize { user, dpi: 1.0 };
        App { ed, name, dir, play_keys, view: fit(size, ui.fixed()), window: None, pixels: None, size, cursor: None, stroke: None, wall_from: None, pan: None, ctrl: false, message: "paint, place, then ENTER to play".into(), clear_armed: false, ui, result: Ok(()) }
    }

    /// Fits the view again, as F does (after a resize or a new UI size).
    fn refit(&mut self) {
        self.view = fit(self.size, self.ui.fixed());
    }

    /// Ctrl + and Ctrl - step the UI's size, saved for both binaries.
    fn resize_ui(&mut self, by: i32) {
        self.ui.user = settings::step(self.ui.user, by);
        settings::save_ui(self.ui.user);
        self.refit();
        self.message = format!("UI size {}", settings::percent(self.ui.user));
    }

    fn map_point(&self, s: (f32, f32)) -> (f32, f32) {
        self.view.to_map(s)
    }

    fn on_panel(&self, s: (f32, f32)) -> bool {
        let (panel, status) = map_edit::chrome(self.ui.fixed());
        s.0 < panel || s.1 > self.size.1 as f32 - status
    }

    fn save(&mut self) -> bool {
        if let Err(e) = std::fs::create_dir_all(&self.dir) {
            self.message = printable(&format!("cannot make {}: {e}", self.dir.display()));
            return false;
        }
        match self.ed.map.save(&self.dir, &self.name) {
            Ok(()) => {
                self.ed.dirty = false;
                self.message = printable(&format!("saved {}", self.dir.join(&self.name).display()));
                true
            }
            Err(e) => {
                self.message = printable(&e);
                false
            }
        }
    }

    /// Saves and starts `larder-world` on the map.
    fn play(&mut self) {
        let problems = self.ed.problems();
        if !problems.is_empty() {
            self.message = format!("cannot play: {}", problems.join(", "));
            return;
        }
        if !self.save() {
            return;
        }
        let spec = format!("map={}/{}", self.dir.display().to_string().replace('\\', "/"), self.name);
        let mut args = vec![spec];
        args.extend(self.play_keys.iter().cloned());
        let beside = std::env::current_exe().ok().map(|e| e.with_file_name(format!("larder-world{}", std::env::consts::EXE_SUFFIX)));
        let started = match beside.filter(|p| p.exists()) {
            Some(exe) => std::process::Command::new(exe).args(&args).spawn(),
            // Not built yet: cargo builds and runs it.
            None => std::process::Command::new("cargo").current_dir(repo()).args(["run", "--release", "--bin", "larder-world", "--"]).args(&args).spawn(),
        };
        self.message = match started {
            Ok(_) => printable(&format!("playing {}", self.name)),
            Err(e) => printable(&format!("cannot start larder-world: {e}")),
        };
    }

    fn redraw(&mut self) {
        let Some(p) = self.pixels.as_mut() else { return };
        let o = Overlay { cursor: self.cursor.map(|c| self.view.to_map(c)), wall_from: self.wall_from, name: &self.name, message: &self.message, ui: self.ui.fixed() };
        map_edit::draw(&self.ed, &self.view, p.frame_mut(), self.size, &o);
        if let Err(e) = p.render() {
            self.result = Err(format!("render failed: {e}"));
        }
    }

    fn key(&mut self, code: KeyCode) {
        let tool = match code {
            KeyCode::Digit1 => Some(Tool::Paint(Brush::Plain)),
            KeyCode::Digit2 => Some(Tool::Paint(Brush::Sand)),
            KeyCode::Digit3 => Some(Tool::Paint(Brush::Clay)),
            KeyCode::Digit4 => Some(Tool::Paint(Brush::Barren)),
            KeyCode::Digit5 => Some(Tool::Paint(Brush::Rock)),
            KeyCode::Digit6 => Some(Tool::Paint(Brush::Wet)),
            KeyCode::KeyC => Some(Tool::Crystal),
            KeyCode::KeyM => Some(Tool::Mould),
            KeyCode::KeyL => Some(Tool::Landing),
            KeyCode::KeyW => Some(Tool::Wall),
            KeyCode::KeyE => Some(Tool::Erase),
            _ => None,
        };
        if code != KeyCode::KeyN {
            self.clear_armed = false;
        }
        if let Some(t) = tool.filter(|_| !self.ctrl) {
            self.ed.tool = t;
            return;
        }
        match code {
            KeyCode::KeyS if self.ctrl => {
                self.save();
            }
            KeyCode::KeyZ if self.ctrl => {
                self.message = if self.ed.undo() { "undone".into() } else { "nothing to undo".into() };
            }
            KeyCode::KeyY if self.ctrl => {
                self.message = if self.ed.redo() { "redone".into() } else { "nothing to redo".into() };
            }
            KeyCode::Equal | KeyCode::NumpadAdd if self.ctrl => self.resize_ui(1),
            KeyCode::Minus | KeyCode::NumpadSubtract if self.ctrl => self.resize_ui(-1),
            KeyCode::BracketLeft | KeyCode::Minus => self.ed.size(-1),
            KeyCode::BracketRight | KeyCode::Equal => self.ed.size(1),
            KeyCode::KeyX => self.ed.mirror = self.ed.mirror.next(),
            KeyCode::KeyF => self.refit(),
            KeyCode::Enter | KeyCode::NumpadEnter | KeyCode::F5 => self.play(),
            KeyCode::KeyN => {
                if self.ed.dirty && !self.clear_armed {
                    self.clear_armed = true;
                    self.message = "unsaved changes: N again to clear the map".into();
                } else {
                    self.ed.begin();
                    self.ed.map = Editor::blank();
                    self.clear_armed = false;
                    self.message = "blank map (CTRL+Z brings it back)".into();
                }
            }
            _ => {}
        }
    }

    fn press(&mut self, s: (f32, f32)) {
        if self.on_panel(s) {
            return;
        }
        let p = self.map_point(s);
        match self.ed.tool {
            Tool::Paint(b) => {
                self.ed.begin();
                self.ed.stamp(b, p);
                self.stroke = Some(p);
            }
            Tool::Wall => self.wall_from = Some(p),
            _ => {
                self.ed.begin();
                self.ed.click(p);
            }
        }
    }

    fn release(&mut self) {
        self.stroke = None;
        if let (Some(a), Some(c)) = (self.wall_from.take(), self.cursor) {
            let b = self.map_point(c);
            self.ed.begin();
            self.ed.wall(a, b);
        }
    }

    fn moved(&mut self, s: (f32, f32)) {
        if let Some(last) = self.pan {
            self.view.pan((s.0 - last.0, s.1 - last.1));
            self.pan = Some(s);
        }
        let p = self.map_point(s);
        if let (Some(last), Tool::Paint(b)) = (self.stroke, self.ed.tool) {
            self.ed.stroke(b, last, p);
            self.stroke = Some(p);
        }
        self.cursor = Some(s);
    }
}

impl ApplicationHandler for App {
    fn resumed(&mut self, event_loop: &ActiveEventLoop) {
        if self.window.is_some() {
            return;
        }
        let first = PhysicalSize::new(self.size.0, self.size.1);
        let attributes = Window::default_attributes().with_title(format!("Larder World map editor: {}", self.name)).with_inner_size(first);
        let window = match event_loop.create_window(attributes) {
            Ok(w) => Arc::new(w),
            Err(e) => {
                self.result = Err(format!("window creation failed: {e}"));
                return event_loop.exit();
            }
        };
        let size = window.inner_size();
        self.size = (size.width.max(1), size.height.max(1));
        match PixelsBuilder::new(self.size.0, self.size.1, SurfaceTexture::new(self.size.0, self.size.1, Arc::clone(&window))).build() {
            Ok(p) => {
                let info = p.adapter().get_info();
                println!("larder-edit: {} ({:?})", info.name, info.backend);
                self.pixels = Some(p);
            }
            Err(e) => {
                self.result = Err(format!("pixels init failed: {e}"));
                return event_loop.exit();
            }
        }
        self.ui.dpi = window.scale_factor() as f32;
        self.refit();
        self.window = Some(window);
    }

    fn window_event(&mut self, event_loop: &ActiveEventLoop, _id: WindowId, event: WindowEvent) {
        match event {
            WindowEvent::CloseRequested => {
                // Nothing drawn is lost: a changed map is saved on the way out.
                if self.ed.dirty {
                    self.save();
                    println!("larder-edit: {}", self.message);
                }
                return event_loop.exit();
            }
            WindowEvent::RedrawRequested => {
                self.redraw();
                if self.result.is_err() {
                    event_loop.exit();
                }
                return;
            }
            WindowEvent::Resized(size) if size.width > 0 && size.height > 0 => {
                if let Some(p) = self.pixels.as_mut() {
                    if let Err(e) = p.resize_buffer(size.width, size.height).and_then(|_| p.resize_surface(size.width, size.height)) {
                        self.result = Err(format!("resize failed: {e}"));
                        return event_loop.exit();
                    }
                }
                self.size = (size.width, size.height);
                self.refit();
            }
            WindowEvent::ScaleFactorChanged { scale_factor, .. } => {
                self.ui.dpi = scale_factor as f32;
                self.refit();
            }
            WindowEvent::ModifiersChanged(m) => self.ctrl = m.state().control_key() || m.state().super_key(),
            WindowEvent::KeyboardInput { event, .. } => {
                if event.state == ElementState::Pressed && !event.repeat {
                    if let PhysicalKey::Code(code) = event.physical_key {
                        self.key(code);
                    }
                }
            }
            WindowEvent::CursorMoved { position, .. } => self.moved((position.x as f32, position.y as f32)),
            WindowEvent::CursorLeft { .. } => self.cursor = None,
            WindowEvent::MouseWheel { delta, .. } => {
                let notches = match delta {
                    MouseScrollDelta::LineDelta(_, y) => y,
                    MouseScrollDelta::PixelDelta(p) => p.y as f32 / 40.0,
                };
                let at = self.cursor.unwrap_or((self.size.0 as f32 / 2.0, self.size.1 as f32 / 2.0));
                self.view.zoom_at(1.2f32.powf(notches), at);
            }
            WindowEvent::MouseInput { state, button, .. } => {
                let down = state == ElementState::Pressed;
                match button {
                    MouseButton::Left if down => {
                        if let Some(c) = self.cursor {
                            self.press(c);
                        }
                    }
                    MouseButton::Left => self.release(),
                    MouseButton::Right | MouseButton::Middle => self.pan = if down { self.cursor } else { None },
                    _ => {}
                }
            }
            _ => return,
        }
        if let Some(w) = &self.window {
            w.set_title(&format!("Larder World map editor: {}{}", self.name, if self.ed.dirty { " *" } else { "" }));
            w.request_redraw();
        }
    }
}

/// The window's picture of `ed`, without a window.
fn picture(ed: &Editor, name: &str, message: &str, size: (u32, u32), cursor: Option<(f32, f32)>, wall_from: Option<(f32, f32)>, ui: f32) -> image::RgbaImage {
    let mut frame = vec![0u8; size.0 as usize * size.1 as usize * 4];
    let view = fit(size, ui);
    map_edit::draw(ed, &view, &mut frame, size, &Overlay { cursor, wall_from, name, message, ui });
    image::RgbaImage::from_raw(size.0, size.1, frame).expect("frame size")
}

/// Draws a small mirrored map from blank, a frame a step, as a hand would
/// (the cloud check's film: the editor's own drawing, step by step).
fn demo(dir: &Path, name: &str, size: (u32, u32), ui: f32) -> Result<(), String> {
    std::fs::create_dir_all(dir).map_err(|e| e.to_string())?;
    let mut ed = Editor::new(Editor::blank());
    ed.mirror = Mirror::Turn;
    let mut n = 0;
    let mut shot = |ed: &Editor, msg: &str, cursor: Option<(f32, f32)>| -> Result<(), String> {
        let view = fit(size, ui);
        let c = cursor.map(|p| view.to_screen(p));
        let img = picture(ed, name, msg, size, c.map(|s| view.to_map(s)), None, ui);
        img.save(dir.join(format!("frame-{n:03}.png"))).map_err(|e| e.to_string())?;
        n += 1;
        Ok(())
    };
    shot(&ed, "a blank map, mirror half turn", None)?;
    // A sand belt across the middle, the strokes in four drags.
    ed.tool = Tool::Paint(Brush::Sand);
    ed.radius = 40.0;
    let belt = [(120.0, 320.0), (300.0, 260.0), (512.0, 320.0)];
    for w in belt.windows(2) {
        ed.begin();
        ed.stroke(Brush::Sand, w[0], w[1]);
        shot(&ed, "2 sand: a belt the mould spreads over", Some(w[1]))?;
    }
    // Clay pockets by each start, rock ridges, a wet hollow.
    ed.tool = Tool::Paint(Brush::Clay);
    ed.radius = 50.0;
    ed.begin();
    ed.stamp(Brush::Clay, (160.0, 110.0));
    shot(&ed, "3 clay: holds the mould near its start", Some((160.0, 110.0)))?;
    ed.tool = Tool::Paint(Brush::Rock);
    ed.radius = 14.0;
    ed.begin();
    ed.stroke(Brush::Rock, (330.0, 60.0), (420.0, 200.0));
    shot(&ed, "5 rock: a ridge", Some((420.0, 200.0)))?;
    ed.tool = Tool::Paint(Brush::Wet);
    ed.radius = 30.0;
    ed.begin();
    ed.stroke(Brush::Wet, (250.0, 470.0), (330.0, 520.0));
    shot(&ed, "6 wet: a hollow", Some((330.0, 520.0)))?;
    ed.tool = Tool::Paint(Brush::Barren);
    ed.radius = 36.0;
    ed.begin();
    ed.stamp(Brush::Barren, (512.0, 120.0));
    shot(&ed, "4 barren: no births", Some((512.0, 120.0)))?;
    // Crystals, starts, the landing, a wall.
    ed.tool = Tool::Crystal;
    for (p, r, charge) in [((160.0, 110.0), 30.0, 384.0), ((420.0, 300.0), 24.0, 256.0), ((80.0, 560.0), 36.0, 512.0)] {
        (ed.crystal_r, ed.charge) = (r, charge);
        ed.begin();
        ed.click(p);
        shot(&ed, "C crystal (mirrored)", Some(p))?;
    }
    ed.tool = Tool::Mould;
    ed.begin();
    ed.click((175.0, 140.0));
    shot(&ed, "M mould start: two, mirrored", Some((175.0, 140.0)))?;
    ed.tool = Tool::Landing;
    ed.begin();
    ed.click((512.0, 320.0));
    shot(&ed, "L the squad lands in the middle", Some((512.0, 320.0)))?;
    ed.tool = Tool::Wall;
    ed.begin();
    ed.wall((440.0, 380.0), (440.0, 470.0));
    shot(&ed, "W a wall", Some((440.0, 470.0)))?;
    let out = dir.join("map");
    std::fs::create_dir_all(&out).map_err(|e| e.to_string())?;
    ed.map.save(&out, name)?;
    ed.dirty = false;
    shot(&ed, &printable(&format!("saved {name}: ready to play")), None)?;
    println!("larder-edit: {n} frames in {}; map {}/{name} (problems: {:?})", dir.display(), out.display(), ed.problems());
    Ok(())
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let (mut spec, mut name, mut dir, mut seed) = (None::<String>, None::<String>, None::<PathBuf>, 1u64);
    let (mut look, mut demo_dir, mut play_keys) = (None::<String>, None::<String>, Vec::new());
    let mut ui = None;
    for a in std::env::args().skip(1) {
        let (k, v) = a.split_once('=').ok_or_else(|| format!("expected key=value, got {a}"))?;
        match k {
            "map" => spec = Some(v.to_string()),
            "name" => name = Some(v.to_string()),
            "dir" => dir = Some(PathBuf::from(v)),
            "look" => look = Some(v.to_string()),
            "demo" => demo_dir = Some(v.to_string()),
            // `ui=150%`: the UI's size this run (passed on to the game too).
            "ui" => {
                ui = Some(settings::parse_size(v).ok_or("ui=PERCENT% (25% to 400%) or a factor such as 1.5")?);
                play_keys.push(a.clone());
            }
            "seed" => {
                seed = v.parse()?;
                play_keys.push(a.clone());
            }
            // Anything else is the game's, for Enter's round.
            _ => play_keys.push(a.clone()),
        }
    }
    let size = (1280, 800);
    if let Some(d) = demo_dir {
        // The pictures at `ui=` or 100%, never a size saved on this machine.
        demo(Path::new(&d), name.as_deref().unwrap_or("demo"), size, ui.unwrap_or(1.0))?;
        return Ok(());
    }
    let map = match &spec {
        Some(s) => open(s, seed)?,
        None => Editor::blank(),
    };
    let name = name.unwrap_or_else(|| spec.as_deref().map(|s| s.rsplit(['/', '\\']).next().unwrap_or(s).to_string()).unwrap_or_else(|| "new".into()));
    let dir = dir.unwrap_or_else(|| repo().join("maps"));
    assert_eq!(map.ground.len(), MAP_W * MAP_H);
    let ed = Editor::new(map);
    if let Some(path) = look {
        picture(&ed, &name, "", size, None, None, ui.unwrap_or(1.0)).save(&path)?;
        println!("larder-edit: {path}");
        return Ok(());
    }
    let event_loop = EventLoop::new()?;
    event_loop.set_control_flow(ControlFlow::Wait);
    let mut app = App::new(ed, name, dir, play_keys, ui.unwrap_or_else(settings::saved_ui));
    event_loop.run_app(&mut app)?;
    app.result.map_err(Into::into)
}
