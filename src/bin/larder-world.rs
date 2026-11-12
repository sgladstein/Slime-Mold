//! **Larder World, the window** (ROADMAP W2c): the Larder game
//! (`mould::larder_world::Game`, W2b) on the GPU world
//! (`gpu::larder_world::GpuWorld`), on a designed map (WM1), drawn with the
//! world look (#69) and played with the world's camera, controls and UI
//! (#63, #80).
//!
//! `cargo run --release --bin larder-world [-- map=fork seed=1]`
//! (`map=` rim, fork, flats, nest, passes, or `noise` for the race world
//! the censuses measured; `walkers=` and `scale=` as `larderworld`'s;
//! `mission=1-landfall` plays a campaign mission: its map, rules and goal,
//! `mould::campaign`, design `Reports/larder-campaign-2026-11-12.md`; any
//! other key is an engine setting over the race world's (and the
//! mission's tier), `gpu::set`'s, such as `tier=sleeper`.)
//!
//! **Why a window of its own.** The owner played `mouldgpu`'s stand-in
//! squad (Q) on 2026-11-11 and found soldiers that did not touch the
//! slime and rivals that lost health by being near each other: that squad
//! is the camera and UI thread's test of the controls, with no game rules
//! (`world::demo`). This window runs the real rules, the ones the W2b
//! census measured: shots are rays that stop at the first slime and kill
//! the walkers they strike, soldiers harvest crystals and carry the charge
//! to the larder, walls cost by length and the mould wears them, and the
//! round ends at the cap, when either side is wiped, or sooner with the
//! core taken. The sandbox keeps its stand-in for trying the controls.
//!
//! **One game, one world.** Every command goes through `Game::apply` (so
//! the round can be replayed: Esc's menu or the end saves
//! `larder-world-replay-MAP-SEED.txt`), and the game steps at its own
//! 15 frames a game second through an accumulator, whatever the window's
//! frame rate. What is drawn is the engine's own fields: the look shades
//! the mould from its trail and walkers, the ground from the world's rock,
//! wet, walls (dimming as the GPU wears them) and crystals (dimming as
//! their charge goes), read back once a game second. A shot is drawn as
//! its ray for a tenth of a second, from the soldier to the slime it hit,
//! and leaves the wound the engine cut.
//!
//! **Controls.** Left click or drag selects; right click moves, or on a
//! crystal harvests it. The command card (bottom right) has the rest, each
//! with its key: A attack the core, S stop, G harvest, F burn a line,
//! Z/X/C hold fire, defend, free fire, B buy a soldier, V lay a wall.
//! P pauses, Tab runs four times faster, N starts the next map, M the
//! minimap, Esc is the menu. Wheel zooms, the middle button, the arrows or
//! the screen edge pan.
//!
//! **Sound** (`slime_mould::sound::world`, 2026-11-12, after the owner's
//! playtest: "shooting the slime didn't feel satisfying"): every shot
//! cracks and strikes with a weight of the walkers it killed, kills
//! crackle, a pulse quickens as the mould closes on the squad, soldiers
//! groan when hurt; all read off the game, so sound on and off play the
//! same round. On by default where the build has a device; O turns it off
//! and on. The mould has a voice of its own, one breathing drone for the
//! whole body (its size, hunger, eating, flinching at gunfire); K turns
//! just that off and on, `voice=0` starts without it.
//!
//! `-- look=DIR [hand=rush|farm|idle] [at=5,30,60] [screen=1280x800]
//! [sound=FILE.wav]` plays without a window and saves the window's picture
//! at those game seconds (`headless`), and with `sound=` what the round
//! sounded like to an ear on the squad, to the last of those seconds;
//! `holes=1` also times how long each hit's hole stays open.
//! `-- bench=SECS [hand=..] [fps=60] [screen=..] [stamps=0]` times the
//! window's frame without a window (`bench`).

use std::path::{Path, PathBuf};
use std::sync::Arc;
use std::time::Instant;

use pixels::{wgpu, Pixels, PixelsBuilder, ScalingMode, SurfaceTexture};
use slime_mould::gpu::larder_world::{race_settings, GpuWorld};
use slime_mould::gpu::{block_on, read_back};
use slime_mould::mould::larder_world::{Cmd, End, Game, Order, Rules, Stance, World};
use slime_mould::mould::campaign::{Mission, Tracker, BIN};
use slime_mould::mould::map::Map;
use slime_mould::mould::tiers::Tier;
use slime_mould::sound::synth::{VoiceBank, DEFAULT_MASTER};
use slime_mould::sound::world::{Ear, WorldListener};
use slime_mould::sound::{self, SoundCmd};
use slime_mould::world::icons::Icon;
use slime_mould::world::look::{self, Fields, Ground};
use slime_mould::world::marks::team_colour;
use slime_mould::world::ui::{theme, Card, Command, Ghost, Info, MenuItem, Model, Resource, Target};
use slime_mould::world::{Button, Camera, Canvas, CanvasPass, Controls, Hud, HudEvent, Marks, UiCommand, Unit, View};
use winit::application::ApplicationHandler;
use winit::dpi::PhysicalSize;
use winit::event::{ElementState, MouseButton, MouseScrollDelta, WindowEvent};
use winit::event_loop::{ActiveEventLoop, ControlFlow, EventLoop};
use winit::keyboard::{KeyCode, PhysicalKey};
use winit::window::{Window, WindowId};

/// The designed maps (`Reports/larder-maps-2026-11-11.md`, v2), N's order.
const MAPS: [&str; 5] = ["fork", "nest", "rim", "flats", "passes"];

// Command ids (the HUD echoes them back).
const ATTACK: u32 = 1;
const STOP: u32 = 2;
const HARVEST: u32 = 3;
const BURN: u32 = 4;
const HOLD_FIRE: u32 = 5;
const DEFEND: u32 = 6;
const FREE: u32 = 7;
const BUY: u32 = 8;
const WALL: u32 = 9;
const RALLY: u32 = 10;

/// How long a shot's ray and its muzzle flash show, real seconds (the world
/// look's fight: `examples/worldlook.rs`).
const TRACER: f32 = 0.1;
const FLASH: f32 = 0.07;
/// Game seconds between read-backs of the crystals' and walls' cells for
/// the ground the look shades.
const GROUND_EVERY: f32 = 1.0;

struct Args {
    /// None: the race world from noise.
    map: Option<String>,
    seed: u64,
    scale: usize,
    walkers: usize,
    /// Engine settings over the race world's (`gpu::set` keys, such as
    /// `tier=sleeper` or `alarm_keep=0.98`), in order.
    keys: Vec<(String, String)>,
    /// A campaign mission (`Reports/larder-campaign-2026-11-12/missions/NAME.txt`):
    /// its map, rules and tier, and its goal on the HUD (`mould::campaign`).
    mission: Option<String>,
    /// The mould's voice at the start (`voice=0` turns it off; K in the
    /// window).
    voice: bool,
}

/// A shot being drawn: its ray, whether it hit slime, and its age.
struct Tracer {
    from: (f32, f32),
    to: (f32, f32),
    hit: bool,
    age: f32,
}

/// What the window keeps of each soldier beside the game: the way it
/// faces, its muzzle flash, and whether it was alive last frame.
#[derive(Clone, Copy)]
struct Look {
    facing: f32,
    flash: f32,
    at: (f32, f32),
    alive: bool,
}

struct App {
    args: Args,
    window: Option<Arc<Window>>,
    pixels: Option<Pixels<'static>>,
    /// The window's GPU (pixels'), or the headless run's.
    gpu: Option<(wgpu::Device, wgpu::Queue)>,
    game: Option<Game<GpuWorld>>,
    /// The map's name on the title and the replay's file name.
    name: String,
    /// Each crystal's charge at the start (for the hover's share left).
    charge0: Vec<f32>,
    view: Option<View>,
    camera: Camera,
    controls: Controls,
    hud: Hud,
    canvas: Option<Canvas>,
    canvas_pass: Option<CanvasPass>,
    marks: Marks,
    /// The look's ground field (`look::ground`), refreshed once a game
    /// second and when a wall goes up.
    ground: Option<wgpu::Buffer>,
    ground_at: f32,
    ground_walls: u32,
    /// Each crystal cell's depth in its crystal (`look::crystal_depth`),
    /// worked out once a world: the crystals keep their cells, and the
    /// depth was most of each ground rebuild (3.9-6.1 ms on the cloud CPU).
    depth: Vec<u8>,
    looks: Vec<Look>,
    tracers: Vec<Tracer>,
    paused: bool,
    menu_paused: bool,
    fast: bool,
    /// Game seconds owed to the game (the accumulator).
    owed: f32,
    last_frame: Instant,
    shift: bool,
    ctrl: bool,
    /// The larder is selected (a click on it with no soldier picked): its
    /// card shows, and right click sets its rally point.
    larder_sel: bool,
    /// Alerts already given: crystals dry, the core taken, the larder
    /// robbed (game second), out of food.
    dry: Vec<bool>,
    core_told: bool,
    robbed_at: Option<f32>,
    stolen: f32,
    broke: bool,
    /// Bank samples (game second, bank) for the food rate.
    bank_log: std::collections::VecDeque<(f32, f32)>,
    /// Frame times (ms) since the last report, and when it was.
    frame_ms: Vec<f32>,
    steps: u32,
    report_at: Instant,
    saved: bool,
    /// The mission being played, and the last game second it followed.
    mission: Option<Tracker>,
    mission_sec: u64,
    /// The speaker (`slime_mould::sound`), or `None` where the build or the
    /// machine has none: the window then plays silent.
    speaker: Option<sound::Output>,
    listener: Option<WorldListener>,
    /// O turns sound off and on (on by default).
    sound_on: bool,
    /// The headless run's recording: its voice bank and samples.
    tape: Option<(VoiceBank, Vec<f32>)>,
    /// Listener time a frame, microseconds, since the last report.
    hear_us: Vec<f32>,
    result: Result<(), String>,
}

impl App {
    fn new(args: Args) -> Self {
        App {
            name: args.map.clone().unwrap_or_else(|| "noise".into()),
            args,
            window: None,
            pixels: None,
            gpu: None,
            game: None,
            charge0: Vec::new(),
            view: None,
            camera: Camera::new((512, 320), (1280, 800)),
            controls: Controls::new(0),
            hud: Hud::new(),
            canvas: None,
            canvas_pass: None,
            marks: Marks::default(),
            ground: None,
            ground_at: 0.0,
            ground_walls: 0,
            depth: Vec::new(),
            looks: Vec::new(),
            tracers: Vec::new(),
            paused: false,
            menu_paused: false,
            fast: false,
            owed: 0.0,
            last_frame: Instant::now(),
            shift: false,
            ctrl: false,
            larder_sel: false,
            dry: Vec::new(),
            core_told: false,
            robbed_at: None,
            stolen: 0.0,
            broke: false,
            bank_log: Default::default(),
            frame_ms: Vec::new(),
            steps: 0,
            report_at: Instant::now(),
            saved: false,
            mission: None,
            mission_sec: 0,
            speaker: None,
            listener: None,
            sound_on: true,
            tape: None,
            hear_us: Vec::new(),
            result: Ok(()),
        }
    }

    fn fail(&mut self, event_loop: &ActiveEventLoop, msg: String) {
        eprintln!("larder-world: {msg}");
        self.result = Err(msg);
        event_loop.exit();
    }

    /// A new round on the map named in `args` (or the noise world).
    fn start(&mut self) -> Result<(), String> {
        self.save_replay();
        let Some((device, queue)) = &self.gpu else { return Ok(()) };
        let a = &self.args;
        let mut s = race_settings(a.seed, a.scale, a.walkers);
        let mission = match &a.mission {
            Some(name) => {
                let path = Path::new(env!("CARGO_MANIFEST_DIR")).join(format!("Reports/larder-campaign-2026-11-12/missions/{name}.txt"));
                let m = Mission::parse(&std::fs::read_to_string(&path).map_err(|e| format!("{}: {e}", path.display()))?)?;
                // A ladder name not built yet (drifter, sprawler, trunk) plays
                // the plain mould, and says so.
                if let Some(t) = m.tier.as_deref() {
                    match Tier::from_name(t) {
                        Some(tier) => tier.settings(&mut s)?,
                        None => eprintln!("larder-world: tier {t} is not built yet; the plain mould plays"),
                    }
                }
                Some(m)
            }
            None => None,
        };
        // Engine keys last, over the race world's and the mission's tier.
        for (k, v) in &a.keys {
            slime_mould::gpu::set(&mut s, k, v)?;
        }
        let map_name = mission.as_ref().map(|m| m.map.clone()).or_else(|| a.map.clone());
        let world = match &map_name {
            Some(name) => {
                let dir = Path::new(env!("CARGO_MANIFEST_DIR")).join("Reports/larder-maps-2026-11-11/maps");
                let (dir, name) = match name.rsplit_once('/') {
                    Some((d, n)) => (PathBuf::from(d), n.to_string()),
                    None => (dir, name.clone()),
                };
                let m = Map::load(&dir, &name)?;
                GpuWorld::with_map(device, queue, s, a.scale, &m)
            }
            None => GpuWorld::new(device, queue, s, a.scale),
        };
        let rules = mission.as_ref().map_or_else(Rules::default, |m| m.rules(Rules::default()));
        let game = Game::new(world, rules, a.seed);
        self.mission = mission.map(|m| Tracker::new(m, &game));
        self.mission_sec = 0;
        let e = &game.world.e;
        let size = (e.dish.w, e.dish.h);
        self.charge0 = game.world.crystals().iter().map(|c| c.1).collect();
        self.dry = vec![false; self.charge0.len()];
        self.depth = look::crystal_depth(e.dish.w, e.dish.w * e.dish.h, &e.dish.pile_of);
        let codes = ground_codes(&game, &self.depth, None, None);
        let ground = look::ground_buffer(device, queue, &codes);
        if let Some(v) = self.view.as_mut() {
            v.set_world(&e.frame, size);
            v.set_look(&e.look, &ground, &e.walkers, e.n);
            v.set_alive(e.grows.then_some(&e.energy));
            // The body's walkers a cell, from the start's living walkers
            // (the look's `body`; a growing world has ten slots a walker).
            v.set_body_density(e.alive() as f32 / (size.0 * size.1) as f32);
            // On the larder, at zoom 4 or the whole map if that is closer
            // (the minimap shows the rest).
            self.camera = Camera::new(size, v.screen);
            let z = self.camera.fit().max(4.0);
            self.camera.zoom_to(z, self.camera.to_screen(self.camera.centre()));
            self.camera.centre_on(game.larder);
        }
        self.ground = Some(ground);
        (self.ground_at, self.ground_walls) = (0.0, 0);
        self.looks = game.soldiers.iter().map(|s| Look { facing: -std::f32::consts::FRAC_PI_2, flash: 0.0, at: (s.x, s.y), alive: s.alive }).collect();
        self.tracers.clear();
        self.controls.selected.clear();
        self.controls.groups = Default::default();
        self.hud.cancel_targeting();
        (self.owed, self.core_told, self.robbed_at, self.stolen, self.broke, self.saved) = (0.0, false, None, 0.0, false, false);
        self.bank_log.clear();
        let voice = self.listener.as_ref().is_none_or(|l| l.voice);
        self.listener = Some(WorldListener::new(&game));
        if let Some(l) = self.listener.as_mut() {
            l.voice = voice && self.args.voice;
        }
        self.game = Some(game);
        if let Some(tr) = &self.mission {
            self.name = tr.mission.map.clone();
        }
        if let Some(w) = &self.window {
            w.set_title(&format!("Larder World: {} (seed {})", title_case(&self.name), self.args.seed));
        }
        match &self.mission {
            Some(tr) => self.hud.alert(&format!("{}: {}", tr.mission.name, tr.mission.says), None, theme::ACCENT),
            None => self.hud.alert(&format!("{}: take the mould's core, or hold more crystals than it at 15:00", title_case(&self.name)), None, theme::ACCENT),
        }
        Ok(())
    }

    /// The round's commands, to replay it (`Game::replay`).
    fn save_replay(&mut self) {
        let Some(g) = self.game.as_ref().filter(|g| !g.log.is_empty() && !self.saved) else { return };
        let path = format!("larder-world-replay-{}-{}.txt", self.name.replace('/', "_"), self.args.seed);
        match std::fs::write(&path, g.replay_text(self.args.seed)) {
            Ok(()) => println!("larder-world: saved {path}"),
            Err(e) => eprintln!("larder-world: {path}: {e}"),
        }
        self.saved = true;
    }

    fn units(&self) -> Vec<Unit> {
        self.game.as_ref().map(|g| g.soldiers.iter().enumerate().map(|(id, s)| Unit { id, team: 0, x: s.x, y: s.y, alive: s.alive }).collect()).unwrap_or_default()
    }

    fn living(&self, ids: &[usize]) -> Vec<usize> {
        let Some(g) = &self.game else { return Vec::new() };
        ids.iter().copied().filter(|&k| g.soldiers.get(k).is_some_and(|s| s.alive)).collect()
    }

    /// An order to each soldier; with Shift held, queued after what each
    /// is doing (`Cmd::Queue`) instead.
    fn order_all(&mut self, ids: &[usize], o: impl Fn(usize) -> Order) {
        let ids = self.living(ids);
        let queue = self.shift;
        if let Some(g) = self.game.as_mut() {
            for (n, k) in ids.into_iter().enumerate() {
                g.apply(if queue { Cmd::Queue(k, o(n)) } else { Cmd::Order(k, o(n)) });
            }
        }
    }

    /// Right click: harvest a crystal under it, else walk there.
    fn order_at(&mut self, ids: &[usize], at: (f32, f32)) {
        let o = self.order_for(at);
        self.order_all(ids, |_| o);
    }

    /// What a point means as an order: harvest a crystal under it, else
    /// walk there.
    fn order_for(&self, at: (f32, f32)) -> Order {
        match &self.game {
            Some(g) if g.crystal_at(at).is_some() => Order::Clear(at.0, at.1),
            _ => Order::Move(at.0, at.1),
        }
    }

    /// The larder's rally point at `at` (`Cmd::Rally`).
    fn rally(&mut self, at: (f32, f32)) {
        let o = self.order_for(at);
        if let Some(g) = self.game.as_mut() {
            g.apply(Cmd::Rally(o));
        }
    }

    /// A click on the world picked no soldier: the larder, if it is under
    /// the click.
    fn on_larder(&self, at: Option<(f32, f32)>) -> bool {
        match (&self.game, at) {
            (Some(g), Some(p)) => (p.0 - g.larder.0).hypot(p.1 - g.larder.1) <= 3.0 * g.world.scale(),
            _ => false,
        }
    }

    /// A burn line shared out: each soldier keeps the whole line burning
    /// (`Order::Burn`) from its own place along it.
    fn burn(&mut self, ids: &[usize], from: (f32, f32), to: (f32, f32)) {
        self.order_all(ids, |_| Order::Burn(from, to));
    }

    fn hud_events(&mut self, events: Vec<HudEvent>, event_loop: &ActiveEventLoop) {
        for ev in events {
            let sel = self.living(&self.controls.selected);
            match ev {
                HudEvent::SelectOnly(k) => {
                    self.controls.selected = vec![k];
                    self.larder_sel = false;
                }
                HudEvent::Deselect(k) => self.controls.selected.retain(|&q| q != k),
                HudEvent::Group(g) => self.controls.selected = self.living(&self.controls.groups[g].clone()),
                HudEvent::Idle => {
                    let after = self.controls.selected.first().copied();
                    if let Some(k) = self.next_idle(after).or_else(|| self.next_idle(None)) {
                        self.controls.selected = vec![k];
                        if let Some(s) = self.game.as_ref().map(|g| g.soldiers[k]) {
                            self.camera.centre_on((s.x, s.y));
                        }
                    }
                }
                HudEvent::Menu(MenuItem::Opened) => {
                    self.menu_paused = self.paused;
                    self.paused = true;
                }
                HudEvent::Menu(MenuItem::Resume) => self.paused = self.menu_paused,
                HudEvent::Menu(MenuItem::Restart) => {
                    if let Err(e) = self.start() {
                        return self.fail(event_loop, e);
                    }
                    self.paused = false;
                }
                HudEvent::Menu(MenuItem::Quit) => {
                    self.save_replay();
                    event_loop.exit();
                }
                HudEvent::Command { id } => match id {
                    STOP => self.order_all(&sel, |_| Order::Hold),
                    HOLD_FIRE | DEFEND | FREE => {
                        let st = match id {
                            HOLD_FIRE => Stance::HoldFire,
                            DEFEND => Stance::Defend,
                            _ => Stance::Free,
                        };
                        if let Some(g) = self.game.as_mut() {
                            for &k in &sel {
                                g.apply(Cmd::Stance(k, st));
                            }
                        }
                    }
                    BUY => {
                        if let Some(g) = self.game.as_mut() {
                            if g.apply(Cmd::Buy) {
                                let s = *g.soldiers.last().unwrap();
                                self.hud.alert("A soldier has joined at the larder", Some((s.x, s.y)), theme::GOOD);
                            }
                        }
                    }
                    _ => {}
                },
                HudEvent::Point { id: HARVEST, at } => self.order_all(&sel, |_| Order::Clear(at.0, at.1)),
                HudEvent::Point { id: RALLY, at } => self.rally(at),
                // A then click: on the core's ring, attack the core; else
                // attack-move there.
                HudEvent::Point { id: ATTACK, at } => {
                    let Some(g) = &self.game else { continue };
                    let on_core = (at.0 - g.core.0).hypot(at.1 - g.core.1) <= g.rules.core_r * g.world.scale() * 1.5;
                    self.order_all(&sel, |_| if on_core { Order::Attack } else { Order::Fight(at.0, at.1) });
                }
                HudEvent::Line { id: BURN, from, to } => self.burn(&sel, from, to),
                HudEvent::Line { id: WALL, from, to } => {
                    if let Some(g) = self.game.as_mut() {
                        let price = g.wall_price(from, to);
                        if !g.apply(Cmd::Wall(from, to)) {
                            let why = if g.bank < price { format!("A wall that long costs {price:.0} food") } else { "Both ends of a wall must be within a soldier's range".to_string() };
                            self.hud.alert(&why, None, theme::WARN);
                        }
                    }
                }
                _ => {}
            }
        }
    }

    fn next_idle(&self, after: Option<usize>) -> Option<usize> {
        let g = self.game.as_ref()?;
        let idle: Vec<usize> = (0..g.soldiers.len()).filter(|&k| g.soldiers[k].alive && g.soldiers[k].order == Order::Hold).collect();
        let start = after.and_then(|a| idle.iter().position(|&k| k > a)).unwrap_or(0);
        idle.get(start).copied()
    }

    /// Steps the game for `dt` real seconds and keeps the window's notes
    /// (facing, flashes, tracers, alerts).
    fn step(&mut self, dt: f32) {
        let Some(g) = self.game.as_mut() else { return };
        for t in &mut self.tracers {
            t.age += dt;
        }
        self.tracers.retain(|t| t.age < TRACER);
        for l in &mut self.looks {
            l.flash = (l.flash - dt).max(0.0);
        }
        // A bought soldier (by the card or a replay) joins the notes.
        for s in &g.soldiers[self.looks.len()..] {
            self.looks.push(Look { facing: -std::f32::consts::FRAC_PI_2, flash: 0.0, at: (s.x, s.y), alive: s.alive });
        }
        let mission_over = self.mission.as_ref().is_some_and(|m| m.verdict.is_some());
        if self.paused || g.end.is_some() || mission_over {
            self.owed = 0.0;
            return;
        }
        let step = g.rules.frame_ticks as f32 / g.rules.tps as f32;
        self.owed += dt.min(0.25) * if self.fast { 4.0 } else { 1.0 };
        let mut n = 0;
        while self.owed >= step && g.end.is_none() {
            g.step();
            self.owed -= step;
            n += 1;
            if let Some(l) = self.listener.as_mut() {
                l.shots(&g.last_shots);
            }
            for s in &g.last_shots {
                self.tracers.push(Tracer { from: s.from, to: s.hit.unwrap_or(s.to), hit: s.hit.is_some(), age: 0.0 });
                if let Some(k) = g.soldiers.iter().position(|q| q.alive && (q.x - s.from.0).abs() < 1e-3 && (q.y - s.from.1).abs() < 1e-3) {
                    self.looks[k].flash = FLASH;
                    self.looks[k].facing = (s.to.1 - s.from.1).atan2(s.to.0 - s.from.0);
                }
            }
            // Behind on frames (a slow one): drop the debt, not the pace.
            if n >= 8 {
                self.owed = 0.0;
            }
        }
        self.steps += n;
        // The mission follows the game once a game second. The GPU world has
        // no mould grid yet (an occupancy read-back), so a goal that needs
        // where the mould is shows no bar here; the CPU census has it.
        if let Some(tr) = self.mission.as_mut() {
            let sec = g.world.ticks() / g.rules.tps as u64;
            // A game that ended inside a game second (a wipe) still gives
            // the mission its verdict, so the end screen is the mission's.
            while (self.mission_sec < sec || g.end.is_some()) && tr.verdict.is_none() {
                self.mission_sec += 1;
                // None until the GPU world reads its mould grid back (the
                // GPU thread's PR #101); then the positional goals fill here.
                let grid = g.world.mould_grid(BIN);
                if let Some(v) = tr.second(g, grid.as_deref(), 1.0) {
                    let colour = if v.stars > 0 { theme::GOOD } else { theme::BAD };
                    self.hud.alert(&tr.cause_line(), None, colour);
                }
            }
        }
        let t = g.time();
        for (k, s) in g.soldiers.iter().enumerate() {
            let l = &mut self.looks[k];
            let (dx, dy) = (s.x - l.at.0, s.y - l.at.1);
            if l.flash <= 0.0 && dx.hypot(dy) > 0.05 {
                l.facing = dy.atan2(dx);
            }
            l.at = (s.x, s.y);
            if l.alive && !s.alive {
                self.hud.alert("A soldier has fallen", Some((s.x, s.y)), theme::BAD);
            }
            l.alive = s.alive;
        }
        for (i, (c, charge)) in g.world.crystals().into_iter().enumerate() {
            if !self.dry[i] && charge <= 0.0 && self.charge0[i] > 0.0 {
                self.dry[i] = true;
                self.hud.alert(&format!("{} has run dry", crystal_name(i)), Some(c), theme::WARN);
            }
        }
        if g.core_down.is_some() && !self.core_told {
            self.core_told = true;
            self.hud.alert("The core is taken: the mould can no longer grow", Some(g.core), theme::GOOD);
        }
        if g.stolen > self.stolen {
            if self.robbed_at.is_none_or(|r| t - r > 30.0) {
                self.hud.alert("The mould is at the larder, eating the bank", Some(g.larder), theme::BAD);
                self.robbed_at = Some(t);
            }
            self.stolen = g.stolen;
        }
        let broke = g.bank < g.rules.price;
        if broke && !self.broke {
            self.hud.alert("Out of food: soldiers cannot fire", None, theme::BAD);
        }
        self.broke = broke;
        self.bank_log.push_back((t, g.bank));
        while self.bank_log.front().is_some_and(|&(t0, _)| t - t0 > 10.0) {
            self.bank_log.pop_front();
        }
        if let Some(end) = g.end {
            println!("larder-world: {} on {} seed {}: {}; score {:+.3}", end.band(), self.name, self.args.seed, summary(g), g.score().unwrap_or(0.0));
            self.save_replay();
        }
    }

    /// Once a drawn frame, after the steps: what the round should sound
    /// like now, to an ear on `ear` (the camera's view). The listener only
    /// reads the game (`sound::world`), so this cannot change it.
    fn hear(&mut self, dt: f32, ear: Ear) {
        let (Some(g), Some(l)) = (self.game.as_ref(), self.listener.as_mut()) else { return };
        if self.speaker.is_none() && self.tape.is_none() {
            return;
        }
        let t0 = Instant::now();
        let cmds = l.hear(g, ear, dt.min(0.25), self.paused);
        self.hear_us.push(t0.elapsed().as_secs_f32() * 1e6);
        if let Some(sp) = self.speaker.as_mut().filter(|_| self.sound_on) {
            sp.send(cmds.iter().copied());
        }
        if let Some((bank, tape)) = self.tape.as_mut() {
            for c in cmds {
                bank.apply(c);
            }
            let mut buf = vec![0.0f32; (dt * bank.sample_rate() as f32).round() as usize * 2];
            bank.render(&mut buf, 2);
            tape.extend_from_slice(&buf);
        }
    }

    /// The camera's view as the listener's ear.
    fn ear(&self) -> Ear {
        let (w, h) = self.camera.span();
        Ear { centre: self.camera.centre(), half: (w / 2.0, h / 2.0) }
    }

    /// The look's ground from the world's cells: once a game second (the
    /// crystals drain, the walls wear), and at once when a wall goes up.
    fn refresh_ground(&mut self) {
        let (Some(g), Some(buf)) = (&self.game, &self.ground) else { return };
        let t = g.time();
        if t - self.ground_at < GROUND_EVERY && g.walls == self.ground_walls {
            return;
        }
        let e = &g.world.e;
        let floats = |b: Vec<u8>| -> Vec<f32> { b.chunks_exact(4).map(|c| f32::from_le_bytes(c.try_into().unwrap())).collect() };
        let charge = floats(read_back(&e.device, &e.queue, &e.charge));
        let health = e.dish.wall.iter().any(|&w| w).then(|| floats(read_back(&e.device, &e.queue, &e.health)));
        let codes = ground_codes(g, &self.depth, Some(&charge), health.as_deref());
        e.queue.write_buffer(buf, 0, &codes.iter().flat_map(|c| c.to_le_bytes()).collect::<Vec<u8>>());
        (self.ground_at, self.ground_walls) = (t, g.walls);
    }

    fn marks(&mut self) {
        self.marks.clear();
        let Some(g) = &self.game else { return };
        let sc = g.world.scale();
        let zoom = self.camera.zoom;
        // The larder (the squad's base: bought soldiers stand there and
        // carried food is banked there) and the core's ring (stand in it
        // with no mould near for 5 s to take it). Game objects, not the
        // simulation, so marks.
        self.marks.rect(g.larder, (1.4 * sc, 1.4 * sc), [214, 196, 150, 255], true);
        self.marks.ring(g.larder, 3.0 * sc, [214, 196, 150, 140]);
        if self.larder_sel {
            self.marks.ring(g.larder, 3.6 * sc, [255, 255, 255, 200]);
        }
        if let Order::Move(x, y) | Order::Clear(x, y) = g.rally {
            if self.larder_sel || !self.controls.selected.is_empty() {
                self.marks.line(g.larder, (x, y), 2.0, [214, 196, 150, 110]);
                self.marks.ring((x, y), 1.2 * sc, [214, 196, 150, 160]);
            }
        }
        let core = if g.core_down.is_some() { [94, 214, 132, 200] } else { [242, 98, 98, 150] };
        self.marks.ring(g.core, g.rules.core_r * sc, core);
        for (k, s) in g.soldiers.iter().enumerate() {
            if !s.alive {
                continue;
            }
            if let Order::Burn(a, b) = s.order {
                self.marks.line(a, b, 4.0, [255, 150, 60, 200]);
            }
            if self.controls.selected.contains(&k) {
                // The order, then the queue (Shift) as a chain of lines,
                // fainter down the queue; red for fighting, white else.
                let mut from = (s.x, s.y);
                let orders = std::iter::once(s.order).chain(s.queue[..s.queued as usize].iter().copied());
                for (n, o) in orders.enumerate() {
                    let to = match o {
                        Order::Move(x, y) | Order::Fight(x, y) => Some((x, y)),
                        Order::Clear(..) if n == 0 && s.homing => Some(g.larder),
                        Order::Clear(x, y) => Some((x, y)),
                        Order::Attack => Some(g.core),
                        Order::Burn(a, b) => Some(((a.0 + b.0) / 2.0, (a.1 + b.1) / 2.0)),
                        Order::Hold => None,
                    };
                    let Some(to) = to else { continue };
                    let a = if n == 0 { 120 } else { 80 };
                    let colour = if matches!(o, Order::Attack | Order::Fight(..)) { [255, 110, 110, a] } else { [255, 255, 255, a - 30] };
                    self.marks.line(from, to, 2.0, colour);
                    if n > 0 {
                        self.marks.ring(to, 1.0 * sc, colour);
                    }
                    from = to;
                }
            }
        }
        for t in &self.tracers {
            let a = (255.0 * (1.0 - t.age / TRACER)).clamp(0.0, 255.0) as u8;
            self.marks.line(t.from, t.to, 0.0, if t.hit { [255, 236, 170, a] } else { [200, 200, 200, a / 2] });
        }
        // A soldier is drawn its touch wide (walkers that close drain it),
        // and at least 8 pixels across the radius, so it reads at any zoom
        // (at 0.9 dish cells, the stand-in's size, it was a 4-pixel dot at
        // the whole map's zoom).
        let r = (g.rules.touch * sc).max(8.0 / zoom);
        for (k, s) in g.soldiers.iter().enumerate() {
            let l = self.looks.get(k).copied().unwrap_or(Look { facing: 0.0, flash: 0.0, at: (s.x, s.y), alive: s.alive });
            self.marks.soldier((s.x, s.y), r, l.facing, team_colour(0), l.flash / FLASH, s.alive);
            if s.alive {
                let below = (r * zoom).max(2.0) + 5.0;
                self.marks.bar((s.x, s.y), below, (7.0, 1.0), s.hp / g.rules.hp, [90, 220, 110, 255], [70, 30, 30, 255]);
                if s.carry > 0.0 {
                    self.marks.bar((s.x, s.y), below + 3.0, (7.0, 0.8), s.carry / g.rules.carry, theme::ENERGY, [20, 40, 50, 255]);
                }
            }
        }
        let units = self.units();
        self.controls.prune(&units);
        self.controls.marks(&units, &self.camera, r, &mut self.marks);
    }

    fn model(&self, hover: Option<(f32, f32)>, ghost: Option<Ghost>) -> Model {
        let Some(g) = &self.game else { return Model::default() };
        let r = g.rules;
        let sc = g.world.scale();
        let sel = self.living(&self.controls.selected);
        let t = g.time();
        let clock = |t: f32| format!("{}:{:02}", t as u32 / 60, t as u32 % 60);
        let rate = self.bank_log.front().filter(|f| t - f.0 >= 2.0).map(|&(t0, b0)| (g.bank - b0) / (t - t0));
        let lost = g.soldiers.iter().filter(|s| !s.alive).count();
        let took = g.take_share();
        let mut m = Model {
            resources: vec![
                Resource { icon: Icon::Crystal, colour: theme::ENERGY, label: "Food: crystal charge carried home; soldiers, shots and walls cost it".into(), value: format!("{:.0}", g.bank), rate: rate.map(|v| format!("{v:+.0}/s")) },
                Resource { icon: Icon::Soldier, colour: theme::TEXT, label: format!("Soldiers alive (at most {})", r.max_squad), value: format!("{}", g.alive()), rate: (lost > 0).then(|| format!("-{lost} lost")) },
                Resource { icon: Icon::Mould, colour: [196, 220, 120, 255], label: "The mould: walkers alive".into(), value: thousands(g.mould), rate: g.core_down.map(|_| "core taken".into()) },
                Resource { icon: Icon::Target, colour: theme::ACCENT, label: "Your share of the charge gone from the crystals (with the mould's size, the score at 15:00)".into(), value: format!("{:.0}%", took * 100.0), rate: None },
            ],
            clock: format!("{} / {}", clock(t), clock(r.cap)),
            speed: if self.paused { Some("Paused".into()) } else if self.fast { Some("x4".into()) } else { None },
            objective: Some(if let Some(tr) = &self.mission {
                match tr.bar {
                    Some(b) => format!("{} ({:.0}%)", tr.mission.says, 100.0 * b),
                    // Said, not hidden (design review, 05:38Z): the GPU world
                    // gives no mould grid yet, so a goal on where the mould
                    // is has no bar in the window.
                    None if tr.secs.iter().sum::<u32>() > 0 => format!("{} (progress not shown in this build yet)", tr.mission.says),
                    None => tr.mission.says.clone(),
                }
            } else if g.core_down.is_some() { "Core taken: wipe out the mould, or cut it down by 15:00".into() } else { "Take the mould's core, or out-harvest it and cut it down by 15:00".into() }),
            idle: g.soldiers.iter().filter(|s| s.alive && s.order == Order::Hold).count(),
            ..Model::default()
        };
        m.selection = sel.iter().map(|&k| {
            let s = g.soldiers[k];
            let badge = match s.order {
                Order::Hold => None,
                Order::Move(..) => Some(Icon::Move),
                Order::Fight(..) => Some(Icon::Attack),
                Order::Clear(..) => Some(Icon::Crystal),
                Order::Burn(..) => Some(Icon::Burn),
                Order::Attack => Some(Icon::Attack),
            };
            Card { id: k, icon: Icon::Soldier, team: 0, hp: s.hp / r.hp, badge }
        }).collect();
        if sel.len() == 1 {
            let k = sel[0];
            let s = g.soldiers[k];
            let doing = match s.order {
                Order::Hold => "holding",
                Order::Move(..) => "moving",
                Order::Fight(..) if s.dense > 0 => "fighting its way",
                Order::Fight(..) => "attack-moving",
                Order::Clear(..) if s.homing => "carrying charge home",
                Order::Clear(..) => "harvesting a crystal",
                Order::Burn(..) => "holding a burn line",
                Order::Attack => "attacking the core",
            };
            m.info = Some(Info {
                icon: Icon::Soldier,
                colour: team_colour(0),
                title: format!("Soldier {}", k + 1),
                subtitle: format!("{doing}, {}", stance_name(s.stance)),
                hp: Some((s.hp / r.hp, format!("{:.0} / {:.0}", s.hp, r.hp))),
                stats: vec![
                    ("Carrying".into(), format!("{:.0} / {:.0}", s.carry, r.carry)),
                    ("Stance".into(), stance_name(s.stance).into()),
                    ("Range".into(), format!("{:.0} cells", r.range)),
                    ("A shot".into(), format!("{:.0} food", r.price)),
                    ("Mould on it".into(), format!("{}", s.touching)),
                ],
                note: None,
            });
        }
        if self.larder_sel && sel.is_empty() {
            m.info = Some(Info {
                icon: Icon::Hold,
                colour: [214, 196, 150, 255],
                title: "The larder".into(),
                subtitle: match g.rally {
                    Order::Clear(..) => "new soldiers go to harvest".into(),
                    Order::Move(..) => "new soldiers go to the rally point".into(),
                    _ => "new soldiers wait here".into(),
                },
                hp: None,
                stats: vec![("Bank".into(), format!("{:.0}", g.bank)), ("Harvested".into(), format!("{:.0}", g.harvested)), ("Eaten by the mould".into(), format!("{:.0}", g.stolen)), ("Soldiers".into(), format!("{} / {}", g.alive(), r.max_squad))],
                note: Some("B buys a soldier. Right-click sets where new soldiers go; on a crystal they start harvesting it.".into()),
            });
        }
        // The hover card: a crystal or the larder under the cursor.
        if let Some(p) = hover {
            if (p.0 - g.larder.0).hypot(p.1 - g.larder.1) <= 3.0 * sc {
                m.hover = Some(Info {
                    icon: Icon::Hold,
                    colour: [214, 196, 150, 255],
                    title: "The larder".into(),
                    subtitle: "your base".into(),
                    hp: None,
                    stats: vec![("Bank".into(), format!("{:.0}", g.bank)), ("Harvested".into(), format!("{:.0}", g.harvested)), ("Eaten by the mould".into(), format!("{:.0}", g.stolen))],
                    note: Some("Harvesters bring charge here; bought soldiers stand here. Mould touching it eats the bank.".into()),
                });
            } else if let Some((i, c)) = g.crystal_at(p) {
                let charge = g.world.crystals()[i].1;
                let full = self.charge0.get(i).copied().unwrap_or(1.0).max(1.0);
                m.hover = Some(Info {
                    icon: Icon::Crystal,
                    colour: theme::ENERGY,
                    title: crystal_name(i),
                    subtitle: format!("{:.0}% of its charge left", 100.0 * charge / full),
                    hp: Some((charge / full, format!("{:.0} food", charge / r.charge))),
                    stats: vec![("Size".into(), format!("{:.0} cells across", 2.0 * g.world.crystal_r(i) / sc))],
                    note: Some(if (c.0 - g.core.0).hypot(c.1 - g.core.1) < 1.0 { "Where the mould grew from. Stand on it with no mould near for 5 s to take it.".into() } else { "Right-click with soldiers selected to harvest it and hold it.".into() }),
                });
            }
        }
        let any = !sel.is_empty();
        let stance = sel.first().map(|&k| g.soldiers[k].stance).filter(|&f| sel.iter().all(|&k| g.soldiers[k].stance == f));
        let need = |cost: f32| (g.bank < cost).then(|| format!("Need {:.0} more food", cost - g.bank));
        let no_sel = || Some("Select soldiers first".to_string());
        let full = g.alive() >= r.max_squad;
        let mut cmds = vec![
            Command { target: Target::Point { radius: 2.0 * sc }, ready: any, why_not: no_sel(), about: format!("Walk there, stopping to fire freely at any mould in reach on the way. Click the core to attack it: stand on it with no mould near for {:.0} s and it can grow no more. Shift queues it.", r.core_secs), ..Command::new(ATTACK, (0, 0), Icon::Attack, "Attack-move", KeyCode::KeyA) },
            Command { ready: any, why_not: no_sel(), about: "Drop the order and stand still; still fires by its stance.".into(), ..Command::new(STOP, (0, 1), Icon::Stop, "Stop", KeyCode::KeyS) },
            Command { target: Target::Point { radius: 2.0 * sc }, ready: any, why_not: no_sel(), about: format!("Go to a crystal, hold it, and carry its charge to the larder, {:.0} food a load. Right-click on a crystal does the same.", r.carry), ..Command::new(HARVEST, (0, 2), Icon::Crystal, "Harvest a crystal", KeyCode::KeyG) },
            Command { target: Target::Line { width: r.strike * sc }, ready: any, why_not: no_sel(), about: "Stand by a line and keep firing along it, so the mould cannot cross.".into(), ..Command::new(BURN, (0, 3), Icon::Burn, "Burn a line", KeyCode::KeyF) },
            Command { ready: any, on: stance == Some(Stance::HoldFire), why_not: no_sel(), about: "Do not shoot at all.".into(), ..Command::new(HOLD_FIRE, (1, 0), Icon::Close, "Hold fire", KeyCode::KeyZ) },
            Command { ready: any, on: stance == Some(Stance::Defend), why_not: no_sel(), about: format!("Shoot only mould touching the soldier, within {:.0} cells, or on the crystal it is harvesting. The default.", r.defend), ..Command::new(DEFEND, (1, 1), Icon::Eye, "Defend", KeyCode::KeyX) },
            Command { ready: any, on: stance == Some(Stance::Free), why_not: no_sel(), about: format!("Shoot the thickest mould in reach, {:.0} cells. Every shot costs {:.0} food.", r.range, r.price), ..Command::new(FREE, (1, 2), Icon::Target, "Free fire", KeyCode::KeyC) },
            Command {
                cost: Some(format!("{:.0}", r.soldier_cost)),
                ready: g.bank >= r.soldier_cost && !full,
                why_not: if full { Some(format!("The squad is at its most, {}", r.max_squad)) } else { need(r.soldier_cost) },
                about: "A new soldier at the larder.".into(),
                ..Command::new(BUY, (2, 0), Icon::Recruit, "Buy a soldier", KeyCode::KeyB)
            },
            Command {
                target: Target::Line { width: r.wall_width * sc },
                cost: Some(format!("{:.0}/cell", r.wall_cost)),
                ready: g.bank >= r.wall_cost,
                why_not: need(r.wall_cost),
                about: "A wall the mould has to wear through, slowly. Both ends within a soldier's range; costs by its length.".into(),
                ..Command::new(WALL, (2, 1), Icon::Wall, "Lay a wall", KeyCode::KeyV)
            },
            Command {
                target: Target::Point { radius: 2.0 * sc },
                ready: true,
                about: "Where bought soldiers go: on a crystal they start harvesting it, elsewhere they walk there. Also: select the larder and right-click.".into(),
                ..Command::new(RALLY, (2, 2), Icon::Move, "Rally point", KeyCode::KeyR)
            },
        ];
        for c in &mut cmds {
            if c.ready {
                c.why_not = None;
            }
        }
        m.commands = cmds;
        (m.ghost_ok, m.ghost_note) = match ghost {
            Some((HARVEST, at, _)) => match g.crystal_at(at) {
                Some((i, _)) => (true, Some(format!("harvest {}", crystal_name(i).to_lowercase()))),
                None => (false, Some("not on a crystal".into())),
            },
            Some((WALL, at, Some(from))) => {
                let price = g.wall_price(from, at);
                if g.bank < price {
                    (false, Some(format!("{price:.0} food, {:.0} short", price - g.bank)))
                } else if !g.reachable(from) || !g.reachable(at) {
                    (false, Some("both ends must be in a soldier's range".into()))
                } else {
                    (true, Some(format!("{price:.0} food, release to lay")))
                }
            }
            Some((WALL, at, None)) if !g.reachable(at) => (false, Some("out of every soldier's range".into())),
            _ => (true, None),
        };
        if let Some(tr) = self.mission.as_ref().filter(|t| t.verdict.is_some()) {
            m.end = Some(mission_screen(tr, g));
        } else if let Some(end) = g.end {
            m.end = Some(end_screen(end, g));
        }
        m.help = vec![
            ("Right click".into(), "Move there; on a crystal, harvest it".into()),
            ("Shift + order".into(), "Queue it after the soldiers' current orders".into()),
            ("A, click".into(), "Attack-move there; on the core, attack it".into()),
            ("Click the larder".into(), "Select it: B buys, right click sets the rally point".into()),
            ("P".into(), "Pause".into()),
            ("Tab".into(), "Four times faster, or back".into()),
            ("N".into(), "Start the next map".into()),
        ];
        m
    }

    fn frame(&mut self, event_loop: &ActiveEventLoop) {
        let t0 = Instant::now();
        let dt = self.last_frame.elapsed().as_secs_f32();
        self.last_frame = Instant::now();
        self.controls.update(&mut self.camera, dt);
        self.step(dt);
        let ear = self.ear();
        self.hear(dt, ear);
        self.prepare();
        let (Some(p), Some(view)) = (&self.pixels, self.view.as_ref()) else { return };
        let camera = self.camera;
        let mut canvas_pass = self.canvas_pass.as_mut();
        let res = p.render_with(|enc, target, ctx| {
            view.record(enc, &camera, None);
            if let Some(cp) = canvas_pass.as_mut() {
                cp.record(enc, &view.out, view.stride);
            }
            view.copy_to(enc, &ctx.texture);
            ctx.scaling_renderer.render(enc, target);
            Ok(())
        });
        if let Err(err) = res {
            return self.fail(event_loop, format!("render failed: {err}"));
        }
        self.frame_ms.push(t0.elapsed().as_secs_f32() * 1e3);
        let since = self.report_at.elapsed().as_secs_f32();
        if since >= 10.0 && !self.frame_ms.is_empty() {
            // The frame's CPU time (the game's steps, their read-backs, the
            // HUD and recording the view): the W2c gate is under 10 ms p95
            // on the GTX 1070.
            let mut ms = std::mem::take(&mut self.frame_ms);
            ms.sort_by(f32::total_cmp);
            let at = |q: f32| ms[((ms.len() - 1) as f32 * q).round() as usize];
            println!("larder-world: {:.0} frames a second, frame p50 {:.2} ms p95 {:.2} ms max {:.2} ms; {:.1} game steps a second", ms.len() as f32 / since, at(0.5), at(0.95), at(1.0), self.steps as f32 / since);
            if !self.hear_us.is_empty() {
                let us = std::mem::take(&mut self.hear_us);
                println!("larder-world: sound {}: listener mean {:.1} us max {:.1} us a frame", if self.sound_on { "on" } else { "off" }, us.iter().sum::<f32>() / us.len() as f32, us.iter().fold(0.0f32, |a, &b| a.max(b)));
            }
            self.steps = 0;
            self.report_at = Instant::now();
        }
    }

    /// The frame's ground, marks, HUD and canvas, ready to record.
    fn prepare(&mut self) {
        self.refresh_ground();
        self.marks();
        let minimap = self.view.as_ref().is_some_and(|v| v.minimap) && !self.camera.sees_all();
        let hover = self.controls.cursor.filter(|&p| !self.hud.over(p)).and_then(|p| self.camera.world_at(p));
        let aim = self.controls.cursor.and_then(|p| self.hud.aim(p, &self.camera));
        let ghost = self.hud.targeting().and_then(|(id, _, from)| aim.map(|h| (id, h, from)));
        let model = self.model(hover, ghost);
        if let Some(c) = self.canvas.as_mut() {
            c.clear();
            self.hud.draw(&model, &self.controls, &self.camera, minimap, c, &mut self.marks);
            self.camera.set_inset(self.hud.top, c.screen.1 as f32 - self.hud.bottom);
            if let Some(cp) = self.canvas_pass.as_mut() {
                cp.upload(c);
            }
        }
        if let Some(v) = self.view.as_mut() {
            v.minimap_box = self.hud.minimap_box;
            v.set_marks(&self.marks);
        }
    }

    fn key(&mut self, code: KeyCode, event_loop: &ActiveEventLoop) {
        match code {
            KeyCode::KeyP | KeyCode::Pause => self.paused = !self.paused,
            KeyCode::Tab => self.fast = !self.fast,
            KeyCode::KeyN => {
                let next = self.args.map.as_deref().and_then(|m| MAPS.iter().position(|&n| n == m)).map_or(0, |i| (i + 1) % MAPS.len());
                self.args.map = Some(MAPS[next].to_string());
                self.name = MAPS[next].to_string();
                if let Err(e) = self.start() {
                    self.fail(event_loop, e);
                }
            }
            KeyCode::KeyM => {
                if let Some(v) = self.view.as_mut() {
                    v.minimap = !v.minimap;
                }
            }
            KeyCode::KeyK => {
                if let Some(l) = self.listener.as_mut() {
                    l.voice = !l.voice;
                    let what = if l.voice { "The mould's voice on" } else { "The mould's voice off" };
                    self.hud.alert(what, None, theme::ACCENT);
                }
            }
            KeyCode::KeyO => {
                self.sound_on = !self.sound_on;
                if let Some(sp) = self.speaker.as_mut() {
                    sp.send([SoundCmd::Master(if self.sound_on { DEFAULT_MASTER } else { 0.0 })]);
                }
                let what = match (self.speaker.is_some(), self.sound_on) {
                    (false, _) => "No sound on this machine",
                    (true, true) => "Sound on",
                    (true, false) => "Sound off",
                };
                self.hud.alert(what, None, theme::ACCENT);
            }
            _ => {}
        }
    }
}

/// The look's ground field from the game's world: rock, wet and agar from
/// the engine's CPU mirror; crystals from its own cells (`pile_of`, so
/// each map crystal's size and shape is the one the mould eats); with
/// `charge`, each crystal cell's share of its full charge; with `health`,
/// each wall cell's share of its full health, and a wall the GPU has worn
/// through drawn as rubble (the mirror keeps it a wall); and the map's
/// soil, which the engine plays since 2026-11-12 (WM3: sand thins the
/// trail, clay keeps it, barren ground starves and takes no births), so
/// the look draws it (#84). `depth` is the crystals' depth
/// (`App::depth`), worked out once a world.
fn ground_codes(g: &Game<GpuWorld>, depth: &[u8], charge: Option<&[f32]>, health: Option<&[f32]>) -> Vec<u32> {
    let d = &g.world.e.dish;
    let cells = d.w * d.h;
    let pile = |i: usize| d.pile_of.get(i).copied().unwrap_or(0) as usize;
    let charge_share: Vec<f32> = match charge {
        Some(c) => (0..cells).map(|i| if pile(i) > 0 { c[i] / d.pile_fill.get(pile(i) - 1).copied().unwrap_or(d.p.crystal_charge).max(1e-6) } else { 0.0 }).collect(),
        None => Vec::new(),
    };
    let broken = |i: usize| health.is_some_and(|h| d.wall.get(i).copied().unwrap_or(false) && h[i] <= 0.0);
    let wall_share: Vec<f32> = match health {
        Some(h) => (0..cells).map(|i| if d.wall.get(i).copied().unwrap_or(false) { let full = d.wall_health.get(i).copied().unwrap_or(f32::INFINITY); if full.is_finite() && full > 0.0 { h[i] / full } else { 1.0 } } else { 0.0 }).collect(),
        None => Vec::new(),
    };
    let wall: Vec<bool> = (0..cells).map(|i| d.wall.get(i).copied().unwrap_or(false) && !broken(i)).collect();
    let agar: Vec<bool> = (0..cells).map(|i| d.agar[i] || broken(i)).collect();
    let mut codes = look::ground(d.w, cells, &Fields { agar: &agar, rock: &d.rock, wet: &d.wet, wall: &wall, wall_share: &wall_share, crystal: &d.pile_of, charge_share: &charge_share, elev: &[], soil: &d.soil, depth });
    for (i, c) in codes.iter_mut().enumerate() {
        if broken(i) {
            *c = look::code(Ground::Rubble, 0.0);
        }
    }
    codes
}

fn end_screen(end: End, g: &Game<GpuWorld>) -> (String, Vec<String>) {
    let clock = |t: f32| format!("{}:{:02}", t as u32 / 60, t as u32 % 60);
    let title = match end {
        End::MouldWiped(t) => format!("The mould is wiped out at {}", clock(t)),
        End::SquadWiped(t) => format!("The squad is lost at {}", clock(t)),
        End::Cap(s) if s > 0.5 => "Time: you took more of the crystals than the mould".to_string(),
        End::Cap(s) if s < 0.5 => "Time: the mould took more of the crystals".to_string(),
        End::Cap(_) => "Time: an even race".to_string(),
    };
    let mut lines = Vec::new();
    if let End::Cap(s) = end {
        let m = g.mould as f32 / g.start_mould.max(1) as f32;
        lines.push(format!("You took {:.0}% of the charge gone from the crystals; the mould is {} walkers, {m:.1} times its start", s * 100.0, thousands(g.mould)));
    }
    lines.push(summary(g));
    lines.push(format!("Score {:+.2} (a win is over 1, a loss under -1)", g.score().unwrap_or(0.0)));
    (title, lines)
}

/// A mission's end: stars and the cause in the title, then what happened.
fn mission_screen(tr: &Tracker, g: &Game<GpuWorld>) -> (String, Vec<String>) {
    let v = tr.verdict.expect("a verdict");
    let stars = if v.stars == 0 { "Mission lost".to_string() } else { format!("{}: {}", tr.mission.name, "*".repeat(v.stars as usize)) };
    (stars, vec![tr.cause_line(), summary(g)])
}

fn summary(g: &Game<GpuWorld>) -> String {
    format!("{} soldiers left, {:.0} food harvested, {} walkers killed with {} shots, {} walls", g.alive(), g.harvested, thousands(g.killed as usize), g.shots, g.walls)
}

fn crystal_name(i: usize) -> String {
    if i == 0 {
        "The mould's home crystal".into()
    } else {
        format!("Crystal {i}")
    }
}

fn stance_name(s: Stance) -> &'static str {
    match s {
        Stance::HoldFire => "hold fire",
        Stance::Defend => "defend",
        Stance::Free => "free fire",
    }
}

fn title_case(s: &str) -> String {
    let s = s.rsplit('/').next().unwrap_or(s);
    let mut c = s.chars();
    match c.next() {
        Some(f) => format!("The {}{}", f.to_uppercase(), c.as_str()),
        None => String::new(),
    }
}

fn thousands(n: usize) -> String {
    if n >= 10_000 {
        format!("{:.1}k", n as f32 / 1000.0)
    } else {
        format!("{n}")
    }
}

impl ApplicationHandler for App {
    fn resumed(&mut self, event_loop: &ActiveEventLoop) {
        if self.window.is_some() {
            return;
        }
        let room = event_loop.primary_monitor().map_or((1600, 900), |m| (m.size().width * 9 / 10, m.size().height * 9 / 10));
        let first = PhysicalSize::new(room.0.clamp(640, 1600), room.1.clamp(480, 1000));
        let attributes = Window::default_attributes().with_title("Larder World").with_inner_size(first);
        let window = match event_loop.create_window(attributes) {
            Ok(w) => Arc::new(w),
            Err(err) => return self.fail(event_loop, format!("window creation failed: {err}")),
        };
        // The engine binds 46 buffers in a pass: ask for the adapter's own
        // limits, not wgpu's defaults (8 storage buffers a stage).
        let instance = wgpu::Instance::new(wgpu::InstanceDescriptor::new_without_display_handle_from_env());
        let options = wgpu::RequestAdapterOptions { power_preference: wgpu::PowerPreference::HighPerformance, compatible_surface: None, force_fallback_adapter: false };
        let Ok(adapter) = block_on(instance.request_adapter(&options)) else { return self.fail(event_loop, "no GPU adapter".into()) };
        let size = window.inner_size();
        let (w, h) = (size.width.max(1), size.height.max(1));
        let desc = wgpu::DeviceDescriptor { label: Some("larder-world"), required_limits: adapter.limits(), ..Default::default() };
        let mut pixels = match PixelsBuilder::new(w, h, SurfaceTexture::new(size.width, size.height, Arc::clone(&window))).request_adapter_options(options).device_descriptor(desc).build() {
            Ok(p) => p,
            Err(err) => return self.fail(event_loop, format!("pixels init failed: {err}")),
        };
        pixels.set_scaling_mode(ScalingMode::PixelPerfect);
        // pixels 0.17 works out its scale only on a resize (mouldgpu).
        if let Err(err) = pixels.resize_surface(size.width, size.height) {
            return self.fail(event_loop, format!("surface resize failed: {err}"));
        }
        let info = pixels.adapter().get_info();
        println!("larder-world: {} ({:?})", info.name, info.backend);
        // A placeholder world frame until the round starts.
        let frame = pixels.device().create_buffer(&wgpu::BufferDescriptor { label: Some("no world"), size: 16, usage: wgpu::BufferUsages::STORAGE, mapped_at_creation: false });
        self.view = Some(View::new(pixels.device(), pixels.queue(), &frame, (1, 1), (w, h)));
        self.canvas = Some(Canvas::new((w, h)));
        self.canvas_pass = Some(CanvasPass::new(pixels.device(), pixels.queue()));
        self.gpu = Some((pixels.device().clone(), pixels.queue().clone()));
        self.pixels = Some(pixels);
        self.window = Some(window);
        self.speaker = match sound::Output::open(self.args.seed) {
            Ok(o) => Some(o),
            Err(e) => {
                eprintln!("larder-world: no sound ({e}); playing silent");
                None
            }
        };
        if let Err(e) = self.start() {
            self.fail(event_loop, e);
        }
    }

    fn window_event(&mut self, event_loop: &ActiveEventLoop, _id: WindowId, event: WindowEvent) {
        match event {
            WindowEvent::CloseRequested => {
                self.save_replay();
                event_loop.exit();
            }
            WindowEvent::Resized(size) => {
                if size.width > 0 && size.height > 0 {
                    if let Some(p) = self.pixels.as_mut() {
                        let res = p.resize_buffer(size.width, size.height).map_err(|e| e.to_string()).and_then(|_| p.resize_surface(size.width, size.height).map_err(|e| e.to_string()));
                        if let Err(err) = res {
                            return self.fail(event_loop, format!("resize failed: {err}"));
                        }
                    }
                    if let Some(v) = self.view.as_mut() {
                        v.resize((size.width, size.height));
                    }
                    self.camera.resize((size.width, size.height));
                    if let Some(c) = self.canvas.as_mut() {
                        c.screen = (size.width, size.height);
                    }
                }
            }
            WindowEvent::ModifiersChanged(m) => {
                self.shift = m.state().shift_key();
                self.ctrl = m.state().control_key();
                (self.controls.shift, self.controls.ctrl) = (self.shift, self.ctrl);
            }
            WindowEvent::KeyboardInput { event, .. } => {
                let PhysicalKey::Code(code) = event.physical_key else { return };
                let pressed = event.state == ElementState::Pressed;
                // The game UI first: its command keys, Esc for its menu,
                // Space for the newest alert.
                if !event.repeat {
                    let (took, events) = self.hud.key(code, pressed, self.ctrl, &mut self.camera);
                    self.hud_events(events, event_loop);
                    if took {
                        return;
                    }
                }
                let units = self.units();
                // The controls: arrows pan, digits are groups, F+drag a burn.
                for c in self.controls.key(code, pressed, event.repeat, &units, &mut self.camera) {
                    if let UiCommand::Key { code, .. } = c {
                        self.key(code, event_loop);
                    }
                }
            }
            WindowEvent::CursorMoved { position, .. } => {
                let at = (position.x as f32, position.y as f32);
                let mm = self.view.as_ref().and_then(|v| v.minimap_rect(&self.camera));
                self.hud.moved(Some(at));
                self.controls.moved(Some(at), &mut self.camera, mm);
            }
            WindowEvent::CursorLeft { .. } => {
                self.hud.moved(None);
                self.controls.moved(None, &mut self.camera, None);
            }
            WindowEvent::Focused(focused) => {
                self.controls.edge = focused;
                if !focused {
                    self.controls.unfocus();
                }
            }
            WindowEvent::MouseWheel { delta, .. } => {
                let notches = match delta {
                    MouseScrollDelta::LineDelta(_, y) => y.signum() as i32,
                    MouseScrollDelta::PixelDelta(p) => (p.y / 40.0).round() as i32,
                };
                self.controls.wheel(notches, &mut self.camera);
            }
            WindowEvent::MouseInput { state, button, .. } => {
                let down = state == ElementState::Pressed;
                let b = match button {
                    MouseButton::Left => Button::Left,
                    MouseButton::Right => Button::Right,
                    MouseButton::Middle => Button::Middle,
                    _ => return,
                };
                let (took, events) = self.hud.button(b, down, &self.camera, self.shift);
                self.hud_events(events, event_loop);
                if took {
                    return;
                }
                let units = self.units();
                let mm = self.view.as_ref().and_then(|v| v.minimap_rect(&self.camera));
                let point = self.controls.cursor.and_then(|c| self.camera.world_at(c));
                // The larder selected and no soldier: right click sets its
                // rally point (the controls give no order without a unit).
                if b == Button::Right && down && self.larder_sel && self.living(&self.controls.selected).is_empty() {
                    if let Some(at) = point {
                        self.rally(at);
                    }
                }
                for c in self.controls.button(b, down, &units, &mut self.camera, mm) {
                    match c {
                        UiCommand::Order { units, at } => self.order_at(&units, at),
                        UiCommand::Burn { units, from, to } => self.burn(&units, from, to),
                        UiCommand::Select(picked) => self.larder_sel = picked.is_empty() && self.on_larder(point),
                        _ => {}
                    }
                }
            }
            WindowEvent::RedrawRequested => self.frame(event_loop),
            _ => {}
        }
    }

    fn about_to_wait(&mut self, _event_loop: &ActiveEventLoop) {
        if let Some(window) = &self.window {
            window.request_redraw();
        }
    }
}

/// **No window** (`look=DIR`): the round played by a simple hand, with the
/// window's whole picture (the look, marks and HUD) saved at the game
/// seconds in `at`, and the counts beside each printed: what the window
/// shows, checkable where there is no screen (the cloud's software
/// Vulkan). Hands: `rush` (all attack the core), `farm` (all harvest the
/// crystal nearest the larder, buying soldiers from 1,200 food), `idle`.
fn headless(args: Args, dir: &str, hand: &str, at: &[f32], screen: (u32, u32), wav: Option<&str>, time_holes: bool) -> Result<(), String> {
    std::fs::create_dir_all(dir).map_err(|e| format!("{dir}: {e}"))?;
    let (device, queue) = gpu().ok_or("no GPU adapter")?;
    let mut app = offscreen(args, screen, &device, &queue)?;
    if wav.is_some() {
        app.tape = Some((VoiceBank::new(48_000, app.args.seed), Vec::new()));
    }
    // The recording's ear: on the squad, at the window's opening zoom.
    let span = app.camera.span();
    let mut voice_at = u64::MAX;
    let step = { let r = Rules::default(); r.frame_ticks as f32 / r.tps as f32 };
    let mut last_sec = u64::MAX;
    // Each hit: where, its tick, the body there the tick before, and the
    // ticks it took to refill.
    type Hole = ((f32, f32), u64, f32, Option<u64>);
    let mut holes: Vec<Hole> = Vec::new();
    let mut prev_lk: Vec<u8> = Vec::new();
    for &t in at {
        // A mission's verdict stops the steps, as the game's end does.
        while app.game.as_ref().is_some_and(|g| g.time() < t && g.end.is_none()) && app.mission.as_ref().is_none_or(|m| m.verdict.is_none()) {
            let g = app.game.as_mut().unwrap();
            let sec = g.world.ticks() / g.rules.tps as u64;
            if sec != last_sec {
                last_sec = sec;
                plan(g, hand);
            }
            app.step(step);
            // `holes=1`: how long a hit's hole stays open (owner, 04:38Z:
            // "maybe part of the issue is the slime/walkers move to fast").
            // The drawn body (the view's `body`: trail x living walkers in
            // the 5 x 5 round each cell) is averaged over each hit's strike
            // disc, and a hole counts as refilled when that is back to 80% of
            // the tick before the hit. A frame's readback a tick: slow.
            if time_holes {
                let g = app.game.as_ref().unwrap();
                let (w, h) = (g.world.e.dish.w, g.world.e.dish.h);
                let lk = read_back(&device, &queue, &g.world.e.look);
                let body = |lk: &[u8], x: i32, y: i32| -> f32 {
                    let cl = |x: i32, y: i32| (y.clamp(0, h as i32 - 1) as usize) * w + x.clamp(0, w as i32 - 1) as usize;
                    let mut n = 0u32;
                    for dy in -2..=2 { for dx in -2..=2 { n += lk[4 * cl(x + dx, y + dy) + 1] as u32; } }
                    lk[4 * cl(x, y)] as f32 / 255.0 * (1.0 - (-(n as f32) / 6.0).exp())
                };
                let disc = |lk: &[u8], cx: f32, cy: f32, r: f32| -> f32 {
                    let (mut sum, mut k) = (0.0, 0);
                    let ri = r.ceil() as i32;
                    for dy in -ri..=ri { for dx in -ri..=ri { if ((dx * dx + dy * dy) as f32) <= r * r { sum += body(lk, cx as i32 + dx, cy as i32 + dy); k += 1; } } }
                    sum / k as f32
                };
                let tick = g.world.ticks();
                let r = g.rules.strike * g.world.scale();
                for t in app.tracers.iter().filter(|t| t.age == 0.0 && t.hit) {
                    let before = if prev_lk.is_empty() { f32::NAN } else { disc(&prev_lk, t.to.0, t.to.1, r) };
                    holes.push((t.to, tick, before, None::<u64>));
                }
                for hl in holes.iter_mut() {
                    if hl.2.is_nan() { continue; }
                    let v = disc(&lk, hl.0 .0, hl.0 .1, r);
                    if hl.3.is_none() && tick > hl.1 && v >= 0.8 * hl.2.max(0.05) { hl.3 = Some(tick - hl.1); }
                }
                prev_lk = lk;
            }
            if app.tape.is_some() {
                let g = app.game.as_ref().unwrap();
                let alive: Vec<(f32, f32)> = g.soldiers.iter().filter(|s| s.alive).map(|s| (s.x, s.y)).collect();
                let n = alive.len().max(1) as f32;
                let centre = if alive.is_empty() { g.larder } else { (alive.iter().map(|p| p.0).sum::<f32>() / n, alive.iter().map(|p| p.1).sum::<f32>() / n) };
                app.hear(step, Ear { centre, half: (span.0 / 2.0, span.1 / 2.0) });
                // The mould's voice's inputs every 5 game seconds, beside
                // the recording.
                let g = app.game.as_ref().unwrap();
                let sec = (g.time() + 1e-3) as u64;
                if sec.is_multiple_of(5) && sec != voice_at {
                    voice_at = sec;
                    let s = app.listener.as_ref().unwrap().stats;
                    println!("  voice t {sec:>3} s: mould {}, growth {:+.4}/s, hunger {:.2}, eating {:.5}/s, kills {:.0}/s, inhales {}", g.mould, s.growth, s.hunger, s.eating, s.kills_per_sec, s.inhales);
                }
            }
        }
        if time_holes {
            let done: Vec<u64> = holes.iter().filter_map(|h| h.3).collect();
            let open = holes.iter().filter(|h| h.3.is_none()).count();
            let mut d = done.clone(); d.sort();
            let tps = app.game.as_ref().unwrap().rules.tps as f32;
            if !d.is_empty() { println!("  holes: shots that hit {} refilled {} (still open {open}); ticks to 80%: median {} ({:.2} s), q25 {}, q75 {}, max {}", holes.len(), d.len(), d[d.len() / 2], d[d.len() / 2] as f32 / tps, d[d.len() / 4], d[3 * d.len() / 4], d[d.len() - 1]); }
        }
        let g = app.game.as_ref().unwrap();
        // The view on the fight: the squad's middle, or the larder.
        let alive: Vec<(f32, f32)> = g.soldiers.iter().filter(|s| s.alive).map(|s| (s.x, s.y)).collect();
        let centre = if alive.is_empty() { g.larder } else { (alive.iter().map(|p| p.0).sum::<f32>() / alive.len() as f32, alive.iter().map(|p| p.1).sum::<f32>() / alive.len() as f32) };
        app.controls.selected = (0..g.soldiers.len()).collect();
        // The HUD sets the camera's insets as it draws, and `fit` reads
        // them: without this the run's first "all" picture was zoomed in
        // past the world's edges (fit with no insets).
        app.prepare();
        for tag in ["all", "near"] {
            let zoom = if tag == "all" { app.camera.fit() } else { 6.0 };
            app.camera.zoom_to(zoom, app.camera.to_screen(app.camera.centre()));
            app.camera.centre_on(if tag == "all" { (app.camera.world.0 / 2.0, app.camera.world.1 / 2.0) } else { centre });
            app.prepare();
            let (view, cp) = (app.view.as_ref().unwrap(), app.canvas_pass.as_mut().unwrap());
            let mut enc = device.create_command_encoder(&wgpu::CommandEncoderDescriptor { label: Some("shot") });
            view.record(&mut enc, &app.camera, None);
            cp.record(&mut enc, &view.out, view.stride);
            queue.submit([enc.finish()]);
            // Whole seconds keep their old names; a time between (a frame
            // sequence) is named in milliseconds.
            let path = if t.fract() == 0.0 { format!("{dir}/{}-{hand}-{:03}s-{tag}.png", app.name.replace('/', "_"), t as u32) } else { format!("{dir}/{}-{hand}-{:05}ms-{tag}.png", app.name.replace('/', "_"), (t * 1000.0) as u32) };
            view.picture().save(&path).map_err(|e| format!("{path}: {e}"))?;
            println!("{path}");
        }
        let g = app.game.as_ref().unwrap();
        let left: f32 = g.world.crystals().iter().map(|c| c.1).sum::<f32>() / app.charge0.iter().sum::<f32>().max(1.0);
        // The CPU work the window adds: the ground field (once a game
        // second) and the model and HUD layout (every frame).
        let cells = g.world.e.dish.w * g.world.e.dish.h;
        let t0 = Instant::now();
        let _ = ground_codes(g, &app.depth, Some(&vec![256.0; cells]), Some(&vec![100.0; cells]));
        let ground_ms = t0.elapsed().as_secs_f64() * 1e3;
        let t0 = Instant::now();
        for _ in 0..20 {
            let m = app.model(None, None);
            app.canvas.as_mut().unwrap().clear();
            app.hud.draw(&m, &app.controls, &app.camera, true, app.canvas.as_mut().unwrap(), &mut Marks::default());
        }
        println!("  CPU: ground field {ground_ms:.2} ms (once a game second), model and HUD {:.3} ms a frame", t0.elapsed().as_secs_f64() * 1e3 / 20.0);
        let g = app.game.as_ref().unwrap();
        println!("  t {:.0} s: mould {}, soldiers {}, bank {:.0}, harvested {:.0}, shots {}, killed {}, took {:.0}%, crystals left {:.3}{}", g.time(), g.mould, g.alive(), g.bank, g.harvested, g.shots, g.killed, g.take_share() * 100.0, left, g.end.map_or(String::new(), |e| format!(", ended: {}", e.band())));
    }
    if let (Some(path), Some((bank, tape))) = (wav, app.tape.as_ref()) {
        slime_mould::wav::write(Path::new(path), tape, 2, bank.sample_rate()).map_err(|e| format!("{path}: {e}"))?;
        let peak = tape.iter().fold(0.0f32, |m, v| m.max(v.abs()));
        let rms = (tape.iter().map(|v| v * v).sum::<f32>() / tape.len().max(1) as f32).sqrt();
        println!("{path}: {:.1} s, peak {:.3}, rms {:.4} ({:.1} dBFS)", tape.len() as f32 / 2.0 / bank.sample_rate() as f32, peak, rms, 20.0 * rms.max(1e-9).log10());
        let s = app.listener.as_ref().map(|l| l.stats).unwrap_or_default();
        println!(
            "  heard: shots {} (voiced {}), impacts {} (light {} mid {} heavy {}), kills {}, beats {}, hurts {}, fallen {}, banked {}, bought {}, walls {}, dry {}, cores {}, endings {}",
            s.shots, s.voiced, s.impacts, s.light, s.mid, s.heavy, s.kills, s.beats, s.hurts, s.fallen, s.banked, s.bought, s.walls, s.dry, s.cores, s.endings
        );
        let b = bank.stats;
        println!("  played: grains {} (touch {} stolen {} kills {}, verbs {}), bursts {}, alerts {}, steals {}", b.grains, b.grains_by_texture[0], b.grains_by_texture[1], b.grains_by_texture[2], b.verb_grains, b.bursts, b.alerts, b.steals);
        let us = &app.hear_us;
        println!("  listener: mean {:.1} us, max {:.1} us a frame over {} frames", us.iter().sum::<f32>() / us.len().max(1) as f32, us.iter().fold(0.0f32, |a, &b| a.max(b)), us.len());
    }
    Ok(())
}

/// **The window's frame time without a window** (`-- bench=SECS`,
/// 2026-11-12): plays `secs` game seconds with `hand` at `fps` frames a
/// real second, doing each frame what the window does (the game's steps
/// through the same accumulator, the ground, marks and HUD, the view and
/// canvas recorded and submitted), then waits for the GPU, as a present
/// with vsync off would. So it can run over Remote Control on the GTX 1070
/// with nobody at the screen. The W2c gate is a frame under 10 ms p95.
/// Prints each frame's time and its parts (the game's steps, which wait on
/// the GPU inside; preparing; recording; the GPU's tail), the step frames'
/// alone, the read-backs and waits a step, and with timestamps the GPU's
/// ms a step's ticks (`stamps=0` leaves them off).
fn bench(args: Args, secs: f32, hand: &str, fps: f32, screen: (u32, u32), stamps: bool) -> Result<(), String> {
    let (device, queue) = gpu().ok_or("no GPU adapter")?;
    let mut app = offscreen(args, screen, &device, &queue)?;
    let stamped = stamps && device.features().contains(wgpu::Features::TIMESTAMP_QUERY);
    if stamped {
        app.game.as_mut().unwrap().world.stamps = Some(slime_mould::gpu::Stamps::new(&device, &queue));
    }
    let dt = 1.0 / fps;
    let mut last_sec = u64::MAX;
    // Each frame: total, step, prepare, record, GPU tail (ms); and whether
    // the game stepped in it.
    type Row = ([f64; 5], bool);
    let mut rows: Vec<Row> = Vec::new();
    let (reads0, steps0) = (slime_mould::gpu::reads(), app.steps);
    let ms = |t: Instant| t.elapsed().as_secs_f64() * 1e3;
    while app.game.as_ref().is_some_and(|g| g.time() < secs && g.end.is_none()) {
        let t0 = Instant::now();
        let g = app.game.as_mut().unwrap();
        let sec = g.world.ticks() / g.rules.tps as u64;
        if sec != last_sec {
            last_sec = sec;
            plan(g, hand);
        }
        let before = app.steps;
        app.step(dt);
        let step = ms(t0);
        let t1 = Instant::now();
        app.prepare();
        let prep = ms(t1);
        let t2 = Instant::now();
        let (view, cp) = (app.view.as_ref().unwrap(), app.canvas_pass.as_mut().unwrap());
        let mut enc = device.create_command_encoder(&wgpu::CommandEncoderDescriptor { label: Some("bench") });
        view.record(&mut enc, &app.camera, None);
        cp.record(&mut enc, &view.out, view.stride);
        let index = queue.submit([enc.finish()]);
        let rec = ms(t2);
        let t3 = Instant::now();
        device.poll(wgpu::PollType::Wait { submission_index: Some(index), timeout: None }).map_err(|e| format!("poll: {e}"))?;
        rows.push(([ms(t0), step, prep, rec, ms(t3)], app.steps > before));
    }
    let g = app.game.as_ref().unwrap();
    let steps = (app.steps - steps0).max(1);
    let q = |pick: &dyn Fn(&Row) -> Option<f64>, at: f64| {
        let mut v: Vec<f64> = rows.iter().filter_map(pick).collect();
        v.sort_by(f64::total_cmp);
        if v.is_empty() {
            return 0.0;
        }
        v[((v.len() - 1) as f64 * at).round() as usize]
    };
    let line = |name: &str, i: usize, only_steps: bool| {
        let pick = move |r: &Row| (!only_steps || r.1).then_some(r.0[i]);
        format!("{name} p50 {:.2} p95 {:.2} max {:.2}", q(&pick, 0.5), q(&pick, 0.95), q(&pick, 1.0))
    };
    let e = &g.world.e;
    let keys: String = app.args.keys.iter().map(|(k, v)| format!(" {k}={v}")).collect();
    println!("larder-world bench: {} seed {}, {} x {} cells, {} slots, scale {} walkers {}{keys}; hand {hand}, {:.0} game s at {fps} frames a second, screen {}x{}", app.name, app.args.seed, e.dish.w, e.dish.h, e.n, app.args.scale, app.args.walkers, g.time(), screen.0, screen.1);
    println!("  {} frames, {} game steps; {:.2} read-backs a step, {:.2} waits a step ({:.2} ms a step waiting)", rows.len(), steps, (slime_mould::gpu::reads() - reads0) as f64 / steps as f64, g.world.waits as f64 / steps as f64, g.world.wait_ms / steps as f64);
    for (only, tag) in [(false, "every frame"), (true, "frames with a step")] {
        println!("  {tag} ({} of them), ms: {}; step {}; prepare {}; record {}; GPU tail {}", rows.iter().filter(|r| !only || r.1).count(), line("frame", 0, only), line("", 1, only), line("", 2, only), line("", 3, only), line("", 4, only));
    }
    if let Some(st) = &g.world.stamps {
        let (sim, draw, _, n) = st.median();
        println!("  GPU (timestamps, median of {n} steps): the step's ticks {sim:.2} ms, its drawing {draw:.2} ms");
    } else {
        println!("  GPU: no timestamps on this adapter");
    }
    println!("  t {:.0} s: mould {}, soldiers {}, harvested {:.0}, shots {}, killed {}{}", g.time(), g.mould, g.alive(), g.harvested, g.shots, g.killed, g.end.map_or(String::new(), |e| format!(", ended: {}", e.band())));
    Ok(())
}

/// A GPU with the adapter's own limits (the engine binds 46 buffers), and
/// timestamps where it has them (`bench`).
fn gpu() -> Option<(wgpu::Device, wgpu::Queue)> {
    let instance = wgpu::Instance::new(wgpu::InstanceDescriptor::new_without_display_handle_from_env());
    let adapter = block_on(instance.request_adapter(&wgpu::RequestAdapterOptions { power_preference: wgpu::PowerPreference::HighPerformance, ..Default::default() })).ok()?;
    println!("larder-world: {}", adapter.get_info().name);
    let required_features = adapter.features() & wgpu::Features::TIMESTAMP_QUERY;
    block_on(adapter.request_device(&wgpu::DeviceDescriptor { label: Some("larder-world"), required_features, required_limits: adapter.limits(), ..Default::default() })).ok()
}

/// The window's game, view and UI on a `screen`-sized picture with no
/// window, the round started.
fn offscreen(args: Args, screen: (u32, u32), device: &wgpu::Device, queue: &wgpu::Queue) -> Result<App, String> {
    let mut app = App::new(args);
    let frame = device.create_buffer(&wgpu::BufferDescriptor { label: Some("no world"), size: 16, usage: wgpu::BufferUsages::STORAGE, mapped_at_creation: false });
    app.view = Some(View::new(device, queue, &frame, (1, 1), screen));
    app.canvas = Some(Canvas::new(screen));
    app.canvas_pass = Some(CanvasPass::new(device, queue));
    app.gpu = Some((device.clone(), queue.clone()));
    app.start()?;
    Ok(app)
}

/// The headless run's hand, once a game second.
fn plan(g: &mut Game<GpuWorld>, hand: &str) {
    if g.time() > 0.5 && hand == "farm" && g.bank >= 1200.0 {
        g.apply(Cmd::Buy);
    }
    let cs = g.world.crystals();
    let target = cs.iter().enumerate().skip(1).filter(|c| c.1 .1 > 0.0).map(|(_, c)| c.0).min_by(|a, b| (a.0 - g.larder.0).hypot(a.1 - g.larder.1).total_cmp(&(b.0 - g.larder.0).hypot(b.1 - g.larder.1)));
    for k in 0..g.soldiers.len() {
        // New soldiers, and harvesters of a crystal gone dry, take a new one.
        let dry = match g.soldiers[k].order {
            Order::Clear(x, y) => g.crystal_at((x, y)).is_none_or(|(i, _)| cs[i].1 <= 0.0),
            _ => false,
        };
        if !g.soldiers[k].alive || (g.soldiers[k].order != Order::Hold && !dry) {
            continue;
        }
        match (hand, target) {
            ("rush", _) => {
                g.apply(Cmd::Order(k, Order::Attack));
            }
            ("farm", Some(c)) => {
                g.apply(Cmd::Order(k, Order::Clear(c.0, c.1)));
            }
            _ => {}
        }
    }
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let mut args = Args { map: Some("fork".into()), seed: 1, scale: 2, walkers: 5000, keys: Vec::new(), voice: true, mission: None };
    let (mut look, mut hand, mut at, mut screen, mut wav) = (None, "rush".to_string(), vec![5.0, 30.0, 60.0], (1280u32, 800u32), None);
    let (mut bench_secs, mut fps, mut stamps) = (None::<f32>, 60.0f32, true);
    let mut holes = false;
    for a in std::env::args().skip(1) {
        let (k, v) = a.split_once('=').ok_or_else(|| format!("expected key=value, got {a}"))?;
        match k {
            "map" => args.map = (v != "noise").then(|| v.to_string()),
            "seed" => args.seed = v.parse()?,
            "scale" => args.scale = v.parse()?,
            "walkers" => args.walkers = v.parse()?,
            "mission" => args.mission = Some(v.to_string()),
            "look" => look = Some(v.to_string()),
            "hand" => hand = v.to_string(),
            "sound" => wav = Some(v.to_string()),
            "voice" => args.voice = v != "0",
            "at" => at = v.split(',').map(|s| s.parse::<f32>()).collect::<Result<_, _>>()?,
            "bench" => bench_secs = Some(v.parse()?),
            "fps" => fps = v.parse()?,
            "stamps" => stamps = v != "0",
            "holes" => holes = v != "0",
            "screen" => {
                let (w, h) = v.split_once('x').ok_or("screen=WxH")?;
                screen = (w.parse()?, h.parse()?);
            }
            // Any other key is an engine setting (`gpu::set`), checked here.
            _ => match slime_mould::gpu::set(&mut race_settings(1, 2, 5000), k, v) {
                Ok(()) => args.keys.push((k.to_string(), v.to_string())),
                Err(e) => return Err(format!("{e} (or one of map, seed, scale, walkers, voice, mission; look, hand, at, screen, sound, holes; bench, fps, stamps)").into()),
            },
        }
    }
    if let Some(secs) = bench_secs {
        return Ok(bench(args, secs, &hand, fps, screen, stamps)?);
    }
    if let Some(dir) = look {
        return Ok(headless(args, &dir, &hand, &at, screen, wav.as_deref(), holes)?);
    }
    let event_loop = EventLoop::new()?;
    event_loop.set_control_flow(ControlFlow::Poll);
    let mut app = App::new(args);
    event_loop.run_app(&mut app)?;
    app.result.map_err(|e| e.into())
}

#[cfg(test)]
mod tests {
    use super::*;

    /// The window plays the real game: soldiers sent at the core fire,
    /// their shots kill walkers, the window draws each shot's ray, and the
    /// engine leaves the look's wound where a shot cut (the squad's `cut`
    /// pass; before W2c only the stand-in hits wrote it). Skips with no
    /// GPU (CI); the cloud's software Vulkan runs it.
    #[test]
    fn the_window_plays_the_real_game() {
        let Some((device, queue)) = gpu() else { return };
        let mut app = offscreen(Args { map: Some("fork".into()), seed: 1, scale: 2, walkers: 5000, keys: Vec::new(), voice: true, mission: None }, (640, 400), &device, &queue).unwrap();
        let step = 1.0 / 15.0;
        let mut rays = 0;
        while app.game.as_ref().unwrap().time() < 12.0 {
            plan(app.game.as_mut().unwrap(), "rush");
            app.step(step);
            rays += app.tracers.iter().filter(|t| t.age == 0.0).count();
        }
        let g = app.game.as_ref().unwrap();
        assert!(g.shots > 0 && g.killed > 0, "shots {} killed {}", g.shots, g.killed);
        assert_eq!(rays as u32, g.shots, "a ray drawn for every shot");
        let look = read_back(&device, &queue, &g.world.e.look);
        let wounded = look.chunks_exact(4).filter(|c| c[2] > 0).count();
        assert!(wounded > 0, "no wound in the look after {} shots", g.shots);
        app.prepare();
        let m = app.model(None, None);
        assert_eq!(m.commands.len(), 10, "the rally point is the tenth");
        assert!(m.end.is_none());
    }

    /// A campaign mission plays its own map and rules, shows its goal as the
    /// objective, and ends on its own screen when the squad is lost (a rush
    /// into the Flats' mould dies by 0:31 on seed 1).
    #[test]
    fn a_mission_shows_its_goal_and_its_end() {
        let Some((device, queue)) = gpu() else { return };
        let mut app = offscreen(Args { map: None, seed: 1, scale: 2, walkers: 5000, keys: Vec::new(), voice: false, mission: Some("1-landfall".into()) }, (640, 400), &device, &queue).unwrap();
        assert_eq!(app.name, "flats");
        assert_eq!(app.game.as_ref().unwrap().bank, 0.0, "the mission's start_bank");
        app.prepare();
        assert!(app.model(None, None).objective.is_some_and(|o| o.starts_with("Grow your squad to")));
        let step = 1.0 / 15.0;
        while app.mission.as_ref().unwrap().verdict.is_none() && app.game.as_ref().unwrap().time() < 120.0 {
            plan(app.game.as_mut().unwrap(), "rush");
            app.step(step);
        }
        let v = app.mission.as_ref().unwrap().verdict.expect("the rush is lost by 2:00");
        assert_eq!(v.stars, 0);
        app.prepare();
        assert_eq!(app.model(None, None).end.map(|e| e.0).as_deref(), Some("Mission lost"));
    }
}
