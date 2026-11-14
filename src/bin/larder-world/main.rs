//! **Larder World, the window** (ROADMAP W2c): the Larder game
//! (`mould::larder_world::Game`, W2b) on the GPU world
//! (`gpu::larder_world::GpuWorld`), on a designed map (WM1), drawn with the
//! world look (#69) and played with the world's camera, controls and UI
//! (#63, #80).
//!
//! `cargo run --release --bin larder-world` opens on the **start menu**
//! (`menu.rs`, 2026-11-14): sandbox, the campaign, options, keys, over a
//! live sandbox round; `menu=1` opens it with the command line's values
//! preset. Any other command line starts its round at once, as before:
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
//! `-- look=DIR [hand=rush|stand|volley|edge|hold|push|farm|far|idle] [at=5,30,60] [screen=1280x800]
//! [sound=FILE.wav]` plays without a window and saves the window's picture
//! at those game seconds (`headless`), and with `sound=` what the round
//! sounded like to an ear on the squad, to the last of those seconds;
//! `holes=1` also times how long each hit's hole stays open; `bank=N` starts
//! the round with N food banked (films of rounds); `centre=X,Y[,ZOOM]`
//! holds the near view on one place (world units, as the t lines' `squad
//! at`), so a run of frames plays as a clip from a still camera.
//! `hover=X,Y` puts the cursor there (screen pixels), so the picture shows
//! that tooltip (a loadout card's, 2026-11-14); `select=0` selects nobody.
//! `cut=T,X,Y,R[,HOLD[,WIDTH]]` takes a ring WIDTH world units wide (6), R
//! out round (X, Y), out of the ground at game second T and holds it (for
//! HOLD seconds, then puts it back), and prints the walkers inside each
//! second (does a part cut off from its crystals wither, or panic?);
//! `cutsquad=X,Y[,T[,N]]` after it sets N of the squad (all; bought first
//! if the squad is short) down there at game second T (the cut's) and
//! keeps them there. `boom=T,X,Y` sets the squad's round off at (X, Y)
//! at game second T, free, as a staged scene (the explosion films); repeat
//! it for more.
//! `hand=volley` fires the squad into the thick body from 22 to 34 s,
//! then holds fire and steps back (`volley`); `hand=edge` stands in the
//! same place on free fire instead. `hand=hold` buys four more soldiers
//! at once and stands all eight at the larder on free fire, so they fire
//! into the mould as it comes and never walk in (`hand=hold@X,Y` stands
//! them at X, Y instead, world units as `centre=`); `hand=push` buys the
//! same eight and from 3 s walks them into the body on the core
//! (`Order::Attack`, the rush's order).
//! `-- bench=SECS [hand=..] [fps=60] [screen=..] [stamps=0] [grid=BIN]
//! [sound=PATH] [fast=1] [passes=N]` times the window's frame without a
//! window (`bench`), a mission's too, and with `passes=` each GPU pass of
//! N more game steps; `hand=every` presses every command on the
//! card each game second (`press_every`), and `sound=` (empty: no file)
//! puts the round's sound on tape. The CI smoke tests play every mission
//! and `hand=every` through it for 60 game seconds. `crater=S` sets how
//! long a hit's crater lasts (`Rules::crater`, 2 game seconds; 0 is off);
//! `aim=STRIKE,EVERY,SPREAD` sets the shot's strike, fire rate and aim
//! spread (`Rules::spread`); `core_pull=P` how strongly soldiers near the
//! mould's core draw it (`Rules::core_pull`; 0 is off); `wade=W` how many
//! walkers touching a soldier halve its speed (`Rules::wade`; 0 is off).
//!
//! **Shapes to playtest** (owner, 2026-11-12 20:21Z: "I will playtest the
//! shapes"): `shape=a|b|c|d|e` plays one of the five films on the shapes
//! page (`Reports/slime-questions-2026-11-12/`): A today's 5,000 walkers;
//! B a mesh everywhere (25,000 scattered, no food scent); C a sheet with
//! trunks (25,000 grown from home); D as C with 22.5-degree sensors 5
//! cells out (sheet, then mesh, then trunks); E as C with a slow thin
//! front. Put it first: keys after it change it. **C is the default**
//! (owner, 2026-11-12 22:19Z: "C is better how it looks, the shape it
//! takes, how it networks the crystals"): a round with no `shape=`,
//! `walkers=`, `mission=` or `own=1` plays C, its keys under any given;
//! `shape=a` plays today's 5,000 walkers. A mission and the own mould's
//! garden keep their own mould. A walker's weight on buildings, the larder and
//! wading follows the walkers a cell (`Rules::mass`: 5,000 at scale 2
//! count 1 each); `mass=M` sets it. Its bite on a soldier
//! (`Rules::drain`, 2 a second) is whole at any count since 2026-11-12:
//! the owner walked 40 soldiers into C's thickest part and lost none.
//! **C shares its food** since 2026-11-13 (`energy_flow` 0.05, the one
//! body thread's rule): the owner could not tell it grew stronger by
//! holding crystals, and now its body swells round the crystals it holds
//! (on Fork about 2.4 times the mould on screen at 5:00); `energy_flow=0`
//! plays C as it was. **A cut-off part of C withers** since 2026-11-13
//! (`wither` 0.01): one that has heard no news of food for 5 s thins over
//! a minute or two (the owner's pick, "double it" from 0.02); `wither=0`
//! plays it without.

use std::path::{Path, PathBuf};
use std::sync::Arc;
use std::time::Instant;

use pixels::{wgpu, Pixels, PixelsBuilder, ScalingMode, SurfaceTexture};
use slime_mould::gpu::larder_world::{garden_settings, race_settings, GpuWorld};
use slime_mould::mould::own_mould::Task;
use slime_mould::gpu::{block_on, read_back};
use slime_mould::mould::kit::{Kit, KitRules, Tool};
use slime_mould::mould::rounds::{self, RoundRules, Rounds};
use slime_mould::mould::larder_world::{Cmd, CoreKind, End, Game, Kind, Order, Rules, Stance, World};
use slime_mould::mould::campaign::{Mission, Tracker, BIN};
use slime_mould::mould::map::Map;
use slime_mould::mould::tiers::Tier;
use slime_mould::sound::synth::{VoiceBank, DEFAULT_MASTER};
use slime_mould::sound::world::{Ear, WorldListener};
use slime_mould::sound::{self, SoundCmd};
use slime_mould::world::icons::Icon;
use slime_mould::world::look::{self, Fields, Ground};
use slime_mould::world::marks::{team_colour, SLASH, STANDING, WALKING};
use slime_mould::world::ui::{theme, Card, CardKind, Command, Ghost, GunCard, Info, Loadout, MenuItem, Model, Resource, Site, Target};
use slime_mould::world::{Button, Camera, Canvas, CanvasPass, Controls, Hud, HudEvent, Marks, UiCommand, Unit, Light, View};
use winit::application::ApplicationHandler;
use winit::dpi::PhysicalSize;
use winit::event::{ElementState, MouseButton, MouseScrollDelta, WindowEvent};
use winit::event_loop::{ActiveEventLoop, ControlFlow, EventLoop};
use winit::keyboard::{KeyCode, PhysicalKey};
use winit::window::{Window, WindowId};
mod args;
mod input;
mod draw;
mod events;
mod headless;
mod bench;
mod hands;
mod menu;
mod squadmode;
mod uisize;
use uisize::*;
use slime_mould::world::settings::{self, UiSize};
use squadmode::SquadInput;
// The moved modules' functions, reached as before from here and from each
// other.
use args::*;
use bench::*;
use draw::*;
use hands::*;
use menu::*;
use slime_mould::world::start::StartMenu;
use headless::*;

/// The designed maps (`Reports/larder-maps-2026-11-11.md`, v2), N's order.
const MAPS: [&str; 5] = ["fork", "nest", "rim", "flats", "passes"];

// Command ids (the HUD echoes them back). One table holds every id, so two
// threads adding a button can't both take the same number without the
// `command_ids_are_one_registry` test failing (technical lead, 11-13: the
// siren landed on 27 because SOW and ROAD had taken 25 and 26 by hand).
// Append new ids at the end; the test wants them to run 1, 2, 3, ...
macro_rules! commands {
    ($($name:ident = $id:expr),* $(,)?) => {
        $(const $name: u32 = $id;)*
        #[cfg(test)]
        const COMMANDS: &[(&str, u32)] = &[$((stringify!($name), $name)),*];
    };
}
commands! {
    ATTACK = 1,
    STOP = 2,
    HARVEST = 3,
    BURN = 4,
    HOLD_FIRE = 5,
    DEFEND = 6,
    FREE = 7,
    BUY = 8,
    WALL = 9,
    RALLY = 10,
    // The build page (Q): turret, lure, pump, outpost, the lures' switch, back.
    BUILD_PAGE = 11,
    TURRET = 12,
    LURE = 13,
    PUMP = 14,
    OUTPOST = 15,
    LURES = 16,
    BACK = 17,
    // The tools page (E, 2026-11-12): the squad toolkit (`mould::kit`), each
    // tool a burst the GPU world applies (`World::burst`).
    TOOLS_PAGE = 18,
    T_BLAST = 19,
    T_CORD = 20,
    T_MINE = 21,
    T_GERM = 22,
    T_GAS = 23,
    T_BACK = 24,
    // The player's own mould (`own=1`): the selected soldiers sow it (Feed as
    // they walk) or lay its road.
    SOW = 25,
    ROAD = 26,
    // The build page's siren (#160).
    SIREN = 27,
    // The loadout panel's swap button (2026-11-14): the next loadout, as H.
    NEXT_ROUND = 28,
}

/// Between two steps: every living soldier at the larder takes one tool
/// (`KitRules::carry_each`), a selected one as many as it may hold
/// (`carry_max`; each would cost carry, `tool_weight`, but that is 0 since
/// 2026-11-14: loaded soldiers looked as if they refused harvest orders),
/// and mines with enough mould on them go off.
fn kit_between(kit: &mut Kit, g: &mut Game<GpuWorld>, selected: &[usize]) {
    for k in 0..g.soldiers.len() {
        let n = if selected.contains(&k) { kit.rules.carry_max } else { kit.rules.carry_each };
        if g.soldiers[k].alive && kit.held(k) < n {
            kit.restock(g, k, n);
        }
    }
    kit.frame(g);
}

/// The building a build-page command places.
fn build_kind(id: u32) -> Option<Kind> {
    match id {
        TURRET => Some(Kind::Turret),
        LURE => Some(Kind::Lure),
        PUMP => Some(Kind::Pump),
        OUTPOST => Some(Kind::Outpost),
        SIREN => Some(Kind::Siren),
        _ => None,
    }
}

/// The sprite a building draws with (`Marks::building`: atlas tiles 4 + 2k
/// for the body, 5 + 2k for the lamp). Another hand-numbered registry, so
/// `command_ids_are_one_registry` checks these too: distinct, 0, 1, 2, ...
/// The larder's sprite kind (`Marks::building`), after the buildings'.
const LARDER_SPRITE: u32 = 5;

fn sprite_kind(k: Kind) -> Option<u32> {
    match k {
        Kind::Turret => Some(0),
        Kind::Lure => Some(1),
        Kind::Pump => Some(2),
        Kind::Outpost => Some(3),
        // Its own sprite (#161): the horns, and the lens and sound arcs lit
        // while it is on.
        Kind::Siren => Some(4),
        // Drawn as a line, not a sprite.
        Kind::Wall { .. } => None,
    }
}

/// How long a shot's ray and its muzzle flash show, real seconds (the world
/// look's fight: `examples/worldlook.rs`).
const TRACER: f32 = 0.1;
const FLASH: f32 = 0.07;
/// How long a take's pulse lasts at a harvester's hands, seconds: a step
/// is 1/15 s, so a soldier taking every step glows steadily and one that
/// stops fades in about a third of a second.
const TAKE: f32 = 0.3;
/// How long the larder's cells flare when a load is banked, seconds.
const BANKED: f32 = 0.6;
/// Game seconds between read-backs of the crystals' and walls' cells for
/// the ground the look shades.
const GROUND_EVERY: f32 = 1.0;

/// The loadouts H cycles (Y was taken by the road and siren commands, 11-13) through (`mould::rounds`): a name and its cards,
/// left to right. The census's hand-written rounds
/// (`Reports/rounds-2026-11-12.md`) plus one card at a time.
const LOADOUTS: [(&str, &str); 24] = [
    ("plain", "none"),
    // The flight cards (11-13): plain bullets that fly on, or break up.
    ("pierce", "pierce+pierce"),
    ("split", "split"),
    ("lance", "pierce+split+blast"),
    ("shrapnel", "split+split+blast"),
    ("blast", "blast"),
    ("wide", "wide+blast"),
    ("scatter", "scatter+blast"),
    ("quiet", "quiet+blast"),
    ("delay", "delay+blast"),
    ("trap", "thick+wide+blast"),
    ("shock", "scatter+blast+wide+blast"),
    ("gas", "gas"),
    ("lull", "quiet+gas+quiet+blast"),
    ("germ", "germ"),
    ("plague", "germ+delay+blast"),
    ("ambush", "quiet+delay+wide+blast"),
    // Explosions (11-14, the owner's "Do we have explosions yet"): a bomb
    // kills a small core and throws the live mould round it outward, where
    // it lands and lives on (`rounds::Card::Bomb`).
    ("bomb", "bomb"),
    ("cluster", "scatter+bomb"),
    ("blowout", "wide+bomb"),
    // Trails (11-14, Noita's burning and gunpowder trails; step 5 of the
    // rounds' look): every bullet lays a real field along its flight.
    ("scorch", "scorch+scorch"),
    ("fence", "alarm"),
    ("mist", "mist"),
    ("firewall", "scorch+blast"),
];

#[derive(Clone)]
struct Args {
    /// `mode=squads`: the squad mode (`squadmode`, `Rules::squads`).
    squads: bool,
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
    /// `crater=S`: game seconds of crater a hit lays (`Rules::crater`; 0
    /// turns the hole under fire off, to compare).
    crater: Option<f32>,
    /// `aim=STRIKE,EVERY,SPREAD`: a shot's strike radius, game seconds
    /// between one soldier's shots, and aim spread, dish cells
    /// (`Rules::strike`, `shot_every`, `spread`): the look A/B of a
    /// scattered smaller strike against the round one.
    aim: Option<[f32; 3]>,
    /// `core_pull=P`: how many times its own scent a soldier near the core
    /// lays (`Rules::core_pull`, the core defence; 0 turns it off, to
    /// compare). A tier's own core (`Tier::core`) wins.
    core_pull: Option<f32>,
    /// `own=1`: the player's own mould (`own_mould`, the gardeners, owner's
    /// card 2026-11-12 04:45Z) on the race world: a second mould, green, on
    /// the larder's crystal, which the soldiers sow and lay roads for
    /// (Sow E, Road Y) and which fights the enemy's by conversion. Shots fly
    /// over it. The GPU world's `garden`; its walkers share one birth pool
    /// with the enemy's until each side's cap is on the GPU.
    own: bool,
    /// `wade=W`: walkers touching a soldier that halve its speed
    /// (`Rules::wade`; 0 turns the slowdown in thick slime off).
    wade: Option<f32>,
    /// `sink=B`: how far into the slime a shot's blast is centred, in
    /// strike radii (`Rules::sink`; 0 centres it on the first slime cell).
    sink: Option<f32>,
    /// `ration=R`: food a game second each soldier eats (`Rules::ration`;
    /// 0, the default since 2026-11-14, is off; 1 plays the PR 4 economy).
    ration: Option<f32>,
    /// `mass=M`: one walker's fight weight (`Rules::mass`); None follows the
    /// start walkers a cell (`Rules::mass_for`).
    mass: Option<f32>,
    /// `rounds=NAME` (or its cards, such as `thick+wide+blast`): the
    /// squad's loadout at the start, an index into `LOADOUTS`.
    round: usize,
    /// `bullet=SPEED`: dish cells a game second a shot flies
    /// (`Rules::bullet_speed`; 0 is the instant ray, to compare).
    bullet: Option<f32>,
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
    /// Seconds left of the pulse at its hands from its last take from a
    /// crystal (`Game::last_takes`, `TAKE` long).
    take: f32,
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
    /// The squad mode's input (`mode=squads`), None in the RTS round.
    sq: Option<SquadInput>,
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
    /// This frame's point lights (bullets, bursts, muzzles, lodged casts),
    /// filled with the marks (`draw.rs`).
    lights: Vec<Light>,
    paused: bool,
    menu_paused: bool,
    fast: bool,
    /// Each step's GPU ticks spread over the frames until the next step is
    /// due (`GpuWorld::spread`, 2026-11-12); `spread=0` records them all on
    /// the step's frame, as before.
    spread: bool,
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
    /// The broken mould (`Rules::broken`) told.
    broken_told: bool,
    robbed_at: Option<f32>,
    stolen: f32,
    broke: bool,
    /// `Game::hungry` last frame, to tell the player once when the squad starts starving.
    hungry: bool,
    /// Bank samples (game second, bank) for the food rate.
    bank_log: std::collections::VecDeque<(f32, f32)>,
    /// Seconds left of the larder's flare (`BANKED`), and the harvest
    /// banked when it was last checked.
    banked: f32,
    harvested_seen: f32,
    /// Frame times (ms) since the last report, and when it was.
    frame_ms: Vec<f32>,
    steps: u32,
    /// Steps finished on a frame once their results were in, and those
    /// finished only when the next step was due (the bench prints both).
    finished: (u32, u32),
    report_at: Instant,
    saved: bool,
    /// The command card shows the build page (Q).
    build_page: bool,
    /// The squad toolkit (`mould::kit`) and whether the card shows its page
    /// (E). Tools are not in the game's replay (the kit is outside `Game`).
    kit: Kit,
    /// The squad's round (`mould::rounds`, after Flashes of Chaos's ammo
    /// cards; 2026-11-12): `rounds=` at the start, H cycles `LOADOUTS`.
    rounds: Rounds,
    round: usize,
    tools_page: bool,
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
    /// The start menu, while it is up (`menu.rs`), over a live sandbox
    /// round; and, while a round it started plays, the menu kept for Main
    /// menu (on the options as they were left).
    menu: Option<StartMenu>,
    parked: Option<StartMenu>,
    /// The command line's round before the default shape, and whether it
    /// chose the mould (`shape=`, `walkers=`): what the menu's picks build
    /// on. None in the headless runs, which have no menu to go back to.
    base: Option<(Args, bool)>,
    result: Result<(), String>,
}

impl App {
    fn new(args: Args) -> Self {
        App {
            name: args.map.clone().unwrap_or_else(|| "noise".into()),
            round: args.round,
            args,
            window: None,
            pixels: None,
            gpu: None,
            game: None,
            charge0: Vec::new(),
            view: None,
            camera: Camera::new((512, 320), (1280, 800)),
            controls: Controls::new(0),
            sq: None,
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
            lights: Vec::new(),
            paused: false,
            menu_paused: false,
            fast: false,
            spread: true,
            owed: 0.0,
            last_frame: Instant::now(),
            shift: false,
            ctrl: false,
            larder_sel: false,
            dry: Vec::new(),
            core_told: false,
            broken_told: false,
            robbed_at: None,
            stolen: 0.0,
            broke: false,
            hungry: false,
            bank_log: Default::default(),
            banked: 0.0,
            harvested_seen: 0.0,
            frame_ms: Vec::new(),
            steps: 0,
            finished: (0, 0),
            report_at: Instant::now(),
            saved: false,
            build_page: false,
            kit: Kit::new(KitRules::default()),
            rounds: Rounds::new(Vec::new(), RoundRules::default()),
            tools_page: false,
            mission: None,
            mission_sec: 0,
            speaker: None,
            listener: None,
            sound_on: true,
            tape: None,
            hear_us: Vec::new(),
            menu: None,
            parked: None,
            base: None,
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
        // own=1 on the noise world is the gardeners' race world; on a map,
        // or a mission with `own` (mission 8), the garden goes on the map
        // (`GpuWorld::garden_on_map`), which sets its own species keys over
        // the race settings and the tier's.
        let on_map = a.mission.is_some() || a.map.is_some();
        let mut s = if a.own && !on_map { garden_settings(a.seed, a.scale, a.walkers) } else { race_settings(a.seed, a.scale, a.walkers) };
        // The tier whose core the game plays (`Tier::core`, read by
        // `Game::with_tier`): the mission's, or a `tier=` key's (the last).
        let mut core_tier = None;
        let mission = match &a.mission {
            Some(name) => {
                let path = Path::new(env!("CARGO_MANIFEST_DIR")).join(format!("Reports/larder-campaign-2026-11-12/missions/{name}.txt"));
                let m = Mission::parse(&std::fs::read_to_string(&path).map_err(|e| format!("{}: {e}", path.display()))?)?;
                // A ladder name not built yet (drifter, sprawler, trunk) plays
                // the plain mould, and says so.
                if let Some(t) = m.tier.as_deref() {
                    match Tier::from_name(t) {
                        Some(tier) => {
                            tier.settings(&mut s)?;
                            core_tier = Some(tier);
                        }
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
            if k == "tier" {
                core_tier = Tier::from_name(v);
            }
        }
        let mut walkers = s.walkers;
        let own = a.own || mission.as_ref().is_some_and(|m| m.own);
        let map_name = mission.as_ref().map(|m| m.map.clone()).or_else(|| a.map.clone());
        let world = match &map_name {
            None if own => GpuWorld::garden(device, queue, s, a.seed, a.scale, a.walkers)?,
            Some(name) => {
                let dir = Path::new(env!("CARGO_MANIFEST_DIR")).join("Reports/larder-maps-2026-11-11/maps");
                let (dir, name) = match name.rsplit_once('/') {
                    Some((d, n)) => (PathBuf::from(d), n.to_string()),
                    None => (dir, name.clone()),
                };
                let m = Map::load(&dir, &name)?;
                if own {
                    // `walkers=` a side, as on the noise world.
                    walkers = 2 * slime_mould::mould::own_mould::garden_map_on(&m, a.walkers)?.1;
                    GpuWorld::garden_on_map(device, queue, s, a.scale, &m, a.walkers)?
                } else {
                    GpuWorld::with_map(device, queue, s, a.scale, &m)
                }
            }
            None => GpuWorld::new(device, queue, s, a.scale),
        };
        let mut rules = mission.as_ref().map_or_else(Rules::default, |m| m.rules(Rules::default()));
        if let Some(c) = a.crater {
            rules.crater = c;
        }
        if let Some(p) = a.core_pull {
            rules.core_pull = p;
        }
        if let Some(b) = a.bullet {
            rules.bullet_speed = b;
        }
        rules.mass = a.mass.unwrap_or(Rules::mass_for(walkers, a.scale));
        if let Some(w) = a.wade {
            rules.wade = w;
        }
        if let Some(b) = a.ration {
            rules.ration = b;
        }
        if let Some(b) = a.sink {
            rules.sink = b;
        }
        if let Some([strike, every, spread]) = a.aim {
            (rules.strike, rules.shot_every, rules.spread) = (strike, every, spread);
        }
        rules.squads = a.squads;
        if a.squads {
            // No larder: drain or guard (`Rules::larder`); a harvesting
            // squad doesn't scatter to the next crystal by itself, and
            // there is no larder to overrun.
            rules.larder = false;
            rules.seek = false;
            rules.overrun = 0.0;
        }
        let mut game = Game::new(world, rules, a.seed).with_tier(core_tier);
        self.sq = a.squads.then(SquadInput::new);
        if a.squads {
            game.apply(Cmd::Active(0));
        }
        self.kit = Kit::new(KitRules::default());
        self.rounds = Rounds::new(rounds::parse(LOADOUTS[self.round].1).unwrap_or_default(), RoundRules::default());
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
            // Each walker's news of food (plan C8, 2026-11-14): a cut-off
            // walker's speck amber while it leans, red once it bolts; the
            // mould ashen by the share cut off. Only when the world withers
            // (news is kept then).
            let ws = &game.world.s;
            v.set_news(e.withers.then_some((&e.news, &e.state)), ws.wither_lag, ws.panic_lean);
            // The crystals as hexagons at screen resolution (the owner's
            // playtest, 2026-11-14: "The crystals are pixelated").
            let gems: Vec<((f32, f32), f32)> = game.world.crystals().iter().enumerate().map(|(i, c)| (c.0, game.world.crystal_r(i))).collect();
            v.set_crystals(&gems, &e.dish.pile_of);
            // Their charge live from the engine, so a crystal sinks, dims
            // and greys where it is taken, and glows where it is being
            // taken (the owner's playtest, 04:53Z: "harvesting crystals
            // could look way better").
            if !e.dish.pile_fill.is_empty() {
                v.set_charge(Some((&e.charge, &e.dish.pile_fill)));
            }
            // The body's walkers a cell, from the start's living walkers
            // (the look's `body`; a growing world has ten slots a walker).
            v.set_body_density(e.alive() as f32 / (size.0 * size.1) as f32);
            // On the larder, at zoom 4 or the whole map if that is closer
            // (the minimap shows the rest).
            self.camera = Camera::new(size, v.screen);
            // The HUD's insets before it has drawn, so the larder opens in
            // the middle of the band above the bottom panels, not under
            // them (the art director, 11-14).
            let (top, bottom) = Hud::insets(v.screen, self.canvas.as_ref().map_or_else(UiSize::default, |c| c.ui));
            self.camera.set_inset(top, bottom);
            let z = self.camera.fit().max(4.0);
            self.camera.zoom_to(z, self.camera.to_screen(self.camera.centre()));
            self.camera.centre_on(game.larder);
        }
        self.ground = Some(ground);
        (self.ground_at, self.ground_walls) = (0.0, 0);
        self.looks = game.soldiers.iter().map(|s| Look { facing: -std::f32::consts::FRAC_PI_2, flash: 0.0, take: 0.0, at: (s.x, s.y), alive: s.alive }).collect();
        self.tracers.clear();
        self.controls.selected.clear();
        self.controls.groups = Default::default();
        self.hud.cancel_targeting();
        (self.owed, self.core_told, self.robbed_at, self.stolen, self.broke, self.saved) = (0.0, false, None, 0.0, false, false);
        self.broken_told = false;
        self.bank_log.clear();
        (self.banked, self.harvested_seen) = (0.0, 0.0);
        let voice = self.listener.as_ref().is_none_or(|l| l.voice);
        self.listener = Some(WorldListener::new(&game));
        if let Some(l) = self.listener.as_mut() {
            l.voice = voice && self.args.voice;
            // A mission's tracker reads the mould grid at `BIN`: the
            // pieces read the same square, so the GPU world packs one grid
            // into its readout and never reads one back (cloud plan C7).
            if self.mission.is_some() {
                l.piece_bin = BIN;
            }
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
            l.take = (l.take - dt).max(0.0);
        }
        self.banked = (self.banked - dt).max(0.0);
        // A load banked since the last frame flares the larder's cells.
        if g.harvested > self.harvested_seen + 0.5 {
            self.banked = BANKED;
        }
        self.harvested_seen = g.harvested;
        // A bought soldier (by the card or a replay) joins the notes.
        join_looks(g, &mut self.looks);
        let mission_over = self.mission.as_ref().is_some_and(|m| m.verdict.is_some());
        // A step runs on the GPU while the window draws (2026-11-12): it
        // begins below and finishes on a later frame, as soon as its results
        // are in (paused too), or at once when the next step is due. The
        // game's order is `step`'s, so a round replays the same (commands
        // finish a running step first, `Game::apply`). Past a mission's
        // verdict the census steps no more, so neither does this.
        if g.in_flight() && g.step_ready() && !mission_over {
            g.finish_step();
            kit_between(&mut self.kit, g, &self.controls.selected);
            self.rounds.after_step(g);
            self.finished.0 += 1;
            shot_marks(g, &mut self.tracers, &mut self.looks, self.listener.as_mut());
        } else if g.in_flight() {
            // Its next share of ticks (`GpuWorld::spread`).
            g.world.frame_more();
        }
        if self.paused || g.end.is_some() || mission_over {
            self.owed = 0.0;
            return;
        }
        let step = g.rules.frame_ticks as f32 / g.rules.tps as f32;
        let pace = dt.min(0.25) * if self.fast { 4.0 } else { 1.0 };
        self.owed += pace;
        // The frames a step lasts at this pace, which its ticks spread over
        // (at 60 frames a second and 15 steps, 4: at scale 8, 2 engine
        // ticks a frame).
        g.world.spread = if self.spread { ((step / pace.max(1e-4)).floor() as u32).max(1) } else { 1 };
        let mut n = 0;
        while self.owed >= step && g.end.is_none() {
            if g.in_flight() {
                g.finish_step();
                kit_between(&mut self.kit, g, &self.controls.selected);
                self.rounds.after_step(g);
                self.finished.1 += 1;
                shot_marks(g, &mut self.tracers, &mut self.looks, self.listener.as_mut());
                if g.end.is_some() {
                    break;
                }
            }
            g.begin_step();
            self.owed -= step;
            n += 1;
            // Behind on frames (a slow one): drop the debt, not the pace.
            if n >= 8 {
                self.owed = 0.0;
            }
        }
        self.steps += n;
        // The mission follows the game once a game second, with the GPU
        // world's mould grid (an occupancy read-back, PR #101).
        if let Some(tr) = self.mission.as_mut() {
            // Whole steps only (a step may still be running on the GPU).
            let sec = g.settled_ticks() / g.rules.tps as u64;
            // A game that ended inside a game second (a wipe) still gives
            // the mission its verdict, so the end screen is the mission's.
            while (self.mission_sec < sec || g.end.is_some()) && tr.verdict.is_none() {
                self.mission_sec += 1;
                let grid = g.world.mould_grid(BIN);
                if let Some(v) = tr.second(g, grid.as_deref(), 1.0) {
                    let colour = if v.stars > 0 { theme::GOOD } else { theme::BAD };
                    self.hud.alert(&tr.cause_line(), None, colour);
                }
            }
        }
        let t = g.time();
        // A soldier the game made inside these steps (`Rules::reserve`'s
        // free recruit) joins the notes too.
        join_looks(g, &mut self.looks);
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
        if g.broken.is_some() != self.broken_told {
            self.broken_told = g.broken.is_some();
            if self.broken_told {
                self.hud.alert("The mould is broken: its stragglers starve unless they reach a crystal", None, theme::GOOD);
            } else {
                self.hud.alert("The mould is growing back from a crystal", None, theme::BAD);
            }
        }
        if g.stolen > self.stolen {
            if self.robbed_at.is_none_or(|r| t - r > 30.0) {
                self.hud.alert("The mould is at the larder, eating the bank", Some(g.larder), theme::BAD);
                self.robbed_at = Some(t);
            }
            self.stolen = g.stolen;
        }
        // Only while shots cost food (`Rules::price`, 0 since 2026-11-12).
        let broke = g.rules.price > 0.0 && g.bank < g.rules.price;
        if broke && !self.broke {
            self.hud.alert("Out of food: soldiers cannot fire", None, theme::BAD);
        }
        self.broke = broke;
        // Rations (`Rules::ration`): told once each time the bank runs dry.
        if g.hungry && !self.hungry {
            self.hud.alert(&format!("Out of food: the squad is starving ({:.0} food a second to feed it)", g.rules.ration * g.alive() as f32), Some(g.larder), theme::BAD);
        }
        self.hungry = g.hungry;
        self.bank_log.push_back((t, g.bank));
        while self.bank_log.front().is_some_and(|&(t0, _)| t - t0 > 10.0) {
            self.bank_log.pop_front();
        }
        if let Some(end) = g.end {
            println!("larder-world: {} on {} seed {}: {}; score {:+.3}", end.band(), self.name, self.args.seed, summary(g), g.score().unwrap_or(0.0));
            self.save_replay();
        }
    }

    /// Finishes a step still running on the GPU and keeps its shots'
    /// marks (the headless pictures read whole steps).
    fn settle(&mut self) {
        let Some(g) = self.game.as_mut() else { return };
        if g.in_flight() {
            g.finish_step();
            // Rounds read the step's hits, so they cast here too (the
            // headless looks settle every step). A step a command finishes
            // (`Game::apply`) is not seen: its hits cast nothing.
            self.rounds.after_step(g);
            shot_marks(g, &mut self.tracers, &mut self.looks, self.listener.as_mut());
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
        // The kit's blasts (mines, thrown charges), heard where they went off.
        l.bangs(&self.kit.bangs);
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

    fn frame(&mut self, event_loop: &ActiveEventLoop) {
        let t0 = Instant::now();
        let dt = self.last_frame.elapsed().as_secs_f32();
        self.last_frame = Instant::now();
        if !self.in_menu() {
            self.controls.update(&mut self.camera, dt);
            self.squad_frame(dt);
        }
        self.step(dt);
        // The dish under the menu plays on: a round it ends starts again.
        if self.in_menu() && self.game.as_ref().is_some_and(|g| g.end.is_some()) {
            if let Err(e) = self.open_menu() {
                return self.fail(event_loop, e);
            }
        }
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
        if let (Some(g), Some(v)) = (self.game.as_ref(), self.view.as_mut()) {
            v.set_time(g.time());
        }
        self.marks();
        let minimap = self.view.as_ref().is_some_and(|v| v.minimap) && !self.camera.sees_all();
        let hover = self.controls.cursor.filter(|&p| !self.hud.over(p)).and_then(|p| self.camera.world_at(p));
        let aim = self.controls.cursor.and_then(|p| self.hud.aim(p, &self.camera));
        let ghost = self.hud.targeting().and_then(|(id, _, from)| aim.map(|h| (id, h, from)));
        let model = self.model(hover, ghost);
        if let (Some(c), Some(menu)) = (self.canvas.as_mut(), self.menu.as_mut()) {
            // The start menu instead of the HUD, over the whole map.
            c.clear();
            menu.draw(c);
            if let Some(cp) = self.canvas_pass.as_mut() {
                cp.upload(c);
            }
            if let Some(v) = self.view.as_mut() {
                v.minimap_box = None;
                v.set_marks(&self.marks);
            }
            return;
        }
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
            v.set_lights(&self.lights);
        }
    }

}

/// `cut=T,X,Y,R[,HOLD[,WIDTH]]` and `cutsquad=X,Y[,T[,N]]` (the slime-enemy
/// thread, 2026-11-13): a ring WIDTH world units wide (6) from R out round
/// (X, Y) taken off the ground at game second T for HOLD seconds (for
/// good), and where N of the squad (all) are set down at game second T (the
/// cut's) and stay, whatever the hand orders (does a cut-off part charge
/// it?).
#[derive(Clone, Copy, Debug)]
struct Cut {
    t: f32,
    x: f32,
    y: f32,
    r: f32,
    hold: f32,
    width: f32,
    squad: Option<(f32, f32, f32, usize)>,
}

/// The bench's settings (`-- bench=SECS` and the keys after it).
struct Bench {
    secs: f32,
    hand: String,
    fps: f32,
    screen: (u32, u32),
    stamps: bool,
    grid: usize,
    /// `sound=PATH`: the round's sound on tape (`App::tape`), heard each
    /// frame as the window hears it, and written to PATH unless it is
    /// empty.
    sound: Option<String>,
    /// `fast=1`: the window's fast forward, four game steps a step's time.
    fast: bool,
    /// `passes=N`: after the timed run, N more game steps with every GPU
    /// pass timed on its own (`GpuWorld::profile`), as a table a pass.
    passes: u32,
    /// `settle=1`: each step finished on the frame it began, as the census
    /// plays (`Game::step`), so the hand's plans never wait on the GPU.
    settle: bool,
    /// `squad1=1`: the squad's unit passes on one thread, as before
    /// 2026-11-12 (`Engine::squad_one`).
    squad1: bool,
    /// `spread=0`: each step's ticks all on the step's frame (`App::spread`).
    spread: bool,
    /// `centre=X,Y[,ZOOM]`: the camera's place and zoom for the timed run
    /// (11-14, the technical lead): the view's cost is a screen pixel's
    /// shading plus `prep`'s fine cells, which grow with the zoom (two a
    /// cell each way from zoom 6), so the draw is timed at the opening zoom
    /// and at `MAX_ZOOM` (24) over the body, not only where the window opens.
    view: Option<(f32, f32, f32)>,
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let mut args = Args { map: Some("fork".into()), seed: 1, scale: 2, walkers: 5000, keys: Vec::new(), voice: true, mission: None, crater: None, aim: None, core_pull: None, mass: None, own: false, wade: None, sink: None, ration: None, round: 0, bullet: None, squads: false };
    let (mut look, mut hand, mut at, mut screen, mut wav) = (None, "rush".to_string(), vec![5.0, 30.0, 60.0], (1280u32, 800u32), None);
    let (mut bench_secs, mut fps, mut stamps, mut grid, mut fast, mut passes) = (None::<f32>, 60.0f32, true, 0usize, false, 0u32);
    let (mut settle, mut squad1, mut spread) = (false, false, true);
    let mut holes = false;
    let mut bank = None;
    let mut centre = None;
    let mut hover = None;
    let mut select = true;
    let mut cut = None;
    let mut booms: Vec<(f32, f32, f32)> = Vec::new();
    // Whether the command line picks the mould (`shape=`, `walkers=`); if
    // not, and no mission does, it is shape C (`DEFAULT_SHAPE`).
    let mut chosen = false;
    // `menu=1` opens the start menu with the command line's values preset,
    // `menu=0` never; with no arguments at all the window opens on it.
    // With `look=`, `menu=PAGE` films that page over the round.
    let mut menu_flag: Option<String> = None;
    // (`ui=` alone still opens on the menu: it only sizes the UI.)
    let bare = std::env::args().skip(1).all(|a| a.starts_with("ui="));
    let mut ui = None;
    for a in std::env::args().skip(1) {
        let (k, v) = a.split_once('=').ok_or_else(|| format!("expected key=value, got {a}"))?;
        match k {
            "map" => args.map = (v != "noise").then(|| v.to_string()),
            "seed" => args.seed = v.parse()?,
            "scale" => args.scale = v.parse()?,
            "walkers" => {
                args.walkers = v.parse()?;
                chosen = true;
            }
            "mission" => args.mission = Some(v.to_string()),
            "mode" => {
                args.squads = match v {
                    "squads" => true,
                    "rts" => false,
                    _ => return Err("mode=squads|rts".into()),
                }
            }
            "crater" => args.crater = Some(v.parse()?),
            "core_pull" => args.core_pull = Some(v.parse()?),
            "bullet" => args.bullet = Some(v.parse()?),
            "own" => {
                args.own = v != "0";
                if args.own && args.map.as_deref() == Some("fork") {
                    args.map = None; // the default map gives way to the garden's race world
                }
            }
            "mass" => args.mass = Some(v.parse()?),
            "shape" => {
                chosen = true;
                let (walkers, keys) = shape_keys(v).ok_or("shape=a|b|c|d|e (the shapes page's films)")?;
                args.walkers = walkers;
                args.keys.extend(keys.iter().map(|(k, v)| (k.to_string(), v.to_string())));
            }
            "wade" => args.wade = Some(v.parse()?),
            "sink" => args.sink = Some(v.parse()?),
            "ration" => args.ration = Some(v.parse()?),
            "aim" => {
                let v: Vec<f32> = v.split(',').map(|s| s.parse::<f32>()).collect::<Result<_, _>>()?;
                args.aim = Some(v.try_into().map_err(|_| "aim=STRIKE,EVERY,SPREAD")?);
            }
            "look" => look = Some(v.to_string()),
            // `ui=150%` or `ui=1.5`: the UI's size this run (`uisize`).
            "ui" => ui = Some(settings::parse_size(v).ok_or("ui=PERCENT% (25% to 400%) or a factor such as 1.5")?),
            "menu" => {
                if v != "0" && page_named(v).is_none() {
                    return Err("menu=0|1|main|campaign|options|keys".into());
                }
                menu_flag = Some(v.to_string());
            }
            "hand" => hand = v.to_string(),
            "rounds" => args.round = LOADOUTS.iter().position(|l| l.0 == v || l.1 == v).ok_or_else(|| format!("rounds=: one of {}", LOADOUTS.map(|l| l.0).join(", ")))?,
            "sound" => wav = Some(v.to_string()),
            "voice" => args.voice = v != "0",
            "at" => at = v.split(',').map(|s| s.parse::<f32>()).collect::<Result<_, _>>()?,
            "bench" => bench_secs = Some(v.parse()?),
            "fps" => fps = v.parse()?,
            "stamps" => stamps = v != "0",
            "grid" => grid = v.parse()?,
            "fast" => fast = v != "0",
            "passes" => passes = v.parse()?,
            "settle" => settle = v != "0",
            "squad1" => squad1 = v != "0",
            "spread" => spread = v != "0",
            "holes" => holes = v != "0",
            // `select=0`: the no-window pictures with nothing selected (the
            // HUD's collapsed selection panel).
            "select" => select = v != "0",
            "hover" => {
                // `hover=X,Y`: the no-window pictures' cursor, screen
                // pixels (a tooltip in the picture: the loadout's cards).
                let (x, y) = v.split_once(',').ok_or("hover=X,Y")?;
                hover = Some((x.parse()?, y.parse()?));
            }
            // `bank=N`: the no-window round starts with N food in the bank
            // (films of rounds, whose casts are paid from it).
            "bank" => bank = Some(v.parse::<f32>()?),
            "centre" => {
                // `centre=X,Y[,ZOOM]`: the near view's place and zoom (6).
                let w = v.split(',').map(|s| s.parse::<f32>()).collect::<Result<Vec<_>, _>>()?;
                let [x, y, zoom @ ..] = w.as_slice() else { return Err("centre=X,Y[,ZOOM]".into()) };
                centre = Some((*x, *y, zoom.first().copied().unwrap_or(6.0)));
            }
            "cut" => {
                // `cut=T,X,Y,R[,HOLD]`: a held ring cut in the no-window
                // round, held HOLD game seconds (for good without it).
                let w = v.split(',').map(|s| s.parse::<f32>()).collect::<Result<Vec<_>, _>>()?;
                let [t, x, y, r, more @ ..] = w.as_slice() else { return Err("cut=T,X,Y,R[,HOLD[,WIDTH]]".into()) };
                let squad = cut.and_then(|c: Cut| c.squad);
                cut = Some(Cut { t: *t, x: *x, y: *y, r: *r, hold: more.first().copied().unwrap_or(f32::INFINITY), width: more.get(1).copied().unwrap_or(6.0), squad });
            }
            "boom" => {
                // `boom=T,X,Y` (11-14, the explosion films): at game second
                // T the squad's round goes off at (X, Y), world units as
                // `centre=`, free (`Rounds::cast_at`); repeat for more.
                let w = v.split(',').map(|s| s.parse::<f32>()).collect::<Result<Vec<_>, _>>()?;
                let [t, x, y] = w.as_slice() else { return Err("boom=T,X,Y".into()) };
                booms.push((*t, *x, *y));
            }
            "cutsquad" => {
                // `cutsquad=X,Y[,T[,N]]` (after `cut=`): N soldiers (all)
                // are set down there at T (the cut's time) and stay.
                let w = v.split(',').map(|s| s.parse::<f32>()).collect::<Result<Vec<_>, _>>()?;
                let [x, y, more @ ..] = w.as_slice() else { return Err("cutsquad=X,Y[,T[,N]]".into()) };
                let c = cut.as_mut().ok_or("cutsquad= needs cut= first")?;
                c.squad = Some((*x, *y, more.first().copied().unwrap_or(c.t), more.get(1).map_or(usize::MAX, |&n| n as usize)));
            }
            "screen" => {
                let (w, h) = v.split_once('x').ok_or("screen=WxH")?;
                screen = (w.parse()?, h.parse()?);
            }
            // Any other key is an engine setting (`gpu::set`), checked here.
            _ => match slime_mould::gpu::set(&mut race_settings(1, 2, 5000), k, v) {
                Ok(()) => args.keys.push((k.to_string(), v.to_string())),
                Err(e) => return Err(format!("{e} (or one of map, seed, scale, walkers, shape, voice, mission, crater, aim, core_pull, mass, wade, sink, ration; look, hand, at, screen, sound, holes, ui, centre, cut, hover, select; bench, fps, stamps, grid, fast, passes, settle, squad1, spread)").into()),
            },
        }
    }
    // The own mould's garden (`own=1`, #165) keeps its own walkers, as a
    // mission keeps its mould: its two species start at twice `walkers`.
    // The pictures and the bench draw at `ui=` or 100%, never at a size
    // saved on this machine; the window opens at the saved size.
    set_ui_user(ui.unwrap_or_else(|| if look.is_some() || bench_secs.is_some() { 1.0 } else { settings::saved_ui() }));
    let base = args.clone();
    if !chosen && args.mission.is_none() && !args.own {
        default_shape(&mut args);
    }
    if let Some(secs) = bench_secs {
        println!("larder-world machine at the start: {}", machine());
        let (device, queue) = gpu().ok_or("no GPU adapter")?;
        bench(args, &Bench { secs, hand, fps, screen, stamps, grid, sound: wav, fast, passes, settle, squad1, spread, view: centre }, &device, &queue)?;
        println!("  machine at the end: {}", machine());
        return Ok(());
    }
    if let Some(dir) = look {
        let page = menu_flag.as_deref().and_then(page_named);
        return Ok(headless(args, &dir, &hand, &at, screen, wav.as_deref(), holes, centre, cut, bank, &booms, page, hover, select)?);
    }
    let event_loop = EventLoop::new()?;
    event_loop.set_control_flow(ControlFlow::Poll);
    let mut app = App::new(args);
    app.spread = spread;
    if menu_flag.as_deref() != Some("0") {
        if bare || menu_flag.is_some() {
            app.menu = Some(start_menu(&base));
        }
        app.base = Some((base, chosen));
    }
    event_loop.run_app(&mut app)?;
    app.result.map_err(|e| e.into())
}

#[cfg(test)]
mod tests;
