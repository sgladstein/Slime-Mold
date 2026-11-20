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
//! (`map=` rim, fork, flats, nest, passes, `planet:DEPTH[:SEED]` for a
//! generated planet (`mould::levelgen`, the roguelike run's levels), or
//! `noise` for the race world the censuses measured; `walkers=` and `scale=` as `larderworld`'s;
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
//! `scroll=N` with `menu=PAGE` scrolls the page N wheel notches a frame.
//! `hover=X,Y[,X2,Y2]` puts the cursor there (screen pixels; with four, it
//! sweeps from the one to the other over the frames), so the picture shows
//! that tooltip (a loadout card's, 2026-11-14); `select=0` selects nobody.
//! `drag=FROM>TO[/FROM>TO..]` with `menu=shop..` films bullet cards dragged
//! in the shop, one drag to an equal share of the frames: FROM a gun slot
//! (counted from 1, as the shop numbers them) or the name of a card out of
//! the gun, TO a slot or `bag` (the gun's cards, 2026-11-14).
//! `cut=T,X,Y,R[,HOLD[,WIDTH]]` takes a ring WIDTH world units wide (6), R
//! out round (X, Y), out of the ground at game second T and holds it (for
//! HOLD seconds, then puts it back), and prints the walkers inside each
//! second (does a part cut off from its crystals wither, or panic?);
//! `cutsquad=X,Y[,T[,N]]` after it sets N of the squad (all; bought first
//! if the squad is short) down there at game second T (the cut's) and
//! keeps them there. `boom=T,X,Y` sets the squad's round off at (X, Y)
//! at game second T, free, as a staged scene (the explosion films); repeat
//! it for more. `boss=T,X,Y` forms a mini-boss round (X, Y) at game second
//! T (`bosses.rs`; its rules from `set=boss_..`), and the round prints each
//! mini-boss once a game second.
//! `hand=volley` fires the squad into the thick body from 22 to 34 s,
//! then holds fire and steps back (`volley`); `hand=edge` stands in the
//! same place on free fire instead. `hand=hold` buys four more soldiers
//! at once and stands all eight at the larder on free fire, so they fire
//! into the mould as it comes and never walk in (`hand=hold@X,Y` stands
//! them at X, Y instead, world units as `centre=`); `hand=push` buys the
//! same eight and from 3 s walks them into the body on the core
//! (`Order::Attack`, the rush's order).
//! `-- bench=SECS [hand=..] [fps=60] [screen=..] [stamps=0] [grid=BIN]
//! [sound=PATH] [fast=1] [passes=N] [viewpasses=N]` times the window's frame without a
//! window (`bench`), a mission's too, and with `passes=` each GPU pass of
//! N more game steps (`viewpasses=` each of the view's passes over N more
//! frames); `hand=every` presses every command on the
//! card each game second (`press_every`), and `sound=` (empty: no file)
//! puts the round's sound on tape. The CI smoke tests play every mission
//! and `hand=every` through it for 60 game seconds. `crater=S` sets how
//! long a hit's crater lasts (`Rules::crater`, 2 game seconds; 0 is off);
//! `boss_look=NAME` how every
//! mini-boss's body is drawn whatever its kind (`LumpLook`: plain, ember,
//! violet, ink, bone; for the look's A/B) and `follow=boss` keeps the near view
//! on the first live mini-boss; `scent=1` opens with the
//! scent view on (F2 in play: the food scent and the trail the mould smells);
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
use slime_mould::mould::guns::{Gun, GunKind};

/// The gun `gun=` names: a kind's average at `depth` (`rifle`, `carbine`,
/// `long`, `scatter`), `KIND:SEED` one of that kind rolled, `roll` or
/// `roll:SEED` any kind rolled (the seed defaults to the game's).
fn gun_of(spec: &str, seed: u64, depth: u32) -> Result<Gun, String> {
    let (what, s) = match spec.split_once(':') {
        Some((w, s)) => (w, Some(s.parse::<u64>().map_err(|_| format!("gun={spec}: the seed is a number"))?)),
        None => (spec, None),
    };
    let seed = s.unwrap_or(seed);
    if what == "roll" {
        return Ok(Gun::roll(seed, depth));
    }
    let kind = GunKind::parse(what).ok_or_else(|| format!("gun={spec}: rifle, carbine, long, scatter or roll"))?;
    Ok(if s.is_some() { Gun::roll_kind(kind, seed, depth) } else { kind.average_at(depth) })
}
use slime_mould::mould::larder_world::{GUARD_RIM, PUMP_LEVELS, TURRET_MUZZLE, WALL_LEVELS, Building, Cmd, CoreKind, End, Game, Kind, Order, Rules, Stance, World};
use slime_mould::mould::campaign::{Mission, Tracker, BIN};
use slime_mould::mould::run::{Ground as RunGround, Run};
use slime_mould::mould::map::Map;
use slime_mould::mould::tiers::Tier;
use slime_mould::sound::synth::VoiceBank;
use slime_mould::sound::world::{Ear, WorldListener};
use slime_mould::sound;
use slime_mould::world::icons::Icon;
use slime_mould::world::look::{self, Fields, Ground};
use slime_mould::world::marks::{team_colour, SLASH, STANDING, TURRET_DRY, TURRET_IDLE, TURRET_TRACKING, WALKING};
use slime_mould::world::ui::{theme, Card, CardKind, Command, Ghost, GunCard, Info, Loadout, MenuItem, Model, Resource, Site, Target, Throw};
use slime_mould::world::view::{LumpBody, LumpLook};
use slime_mould::world::{Button, Camera, Canvas, CanvasPass, Controls, Hud, HudEvent, Marks, UiCommand, Unit, Light, View};
use winit::application::ApplicationHandler;
use winit::dpi::PhysicalSize;
use winit::event::{ElementState, MouseButton, MouseScrollDelta, WindowEvent};
use winit::event_loop::{ActiveEventLoop, ControlFlow, EventLoop};
use winit::keyboard::{KeyCode, PhysicalKey};
use winit::window::{Window, WindowId};

/// Each mini-boss kind's look (`View::set_lumps`, the `lump_looks` code's
/// bits 0-2; `boss_look=` overrides every kind for the look's A/B): told
/// apart by value as well as hue, so they read for colour-blind players and
/// on every planet. The hunter violet (the hue nothing else on screen
/// uses), the wanderer pale bone (a fat grub that grows as it grazes), the
/// sapper ember (an acid red at your walls), the leech ink (dark tar on the
/// pump rig, its core turning the charge's cyan as it drinks), the mortar
/// moss with an acid core and the pinwheel cobalt (2026-11-20).
fn kind_look(kind: u16) -> LumpLook {
    match kind {
        1 => LumpLook::Violet,
        2 => LumpLook::Bone,
        3 => LumpLook::Ember,
        4 => LumpLook::Ink,
        5 => LumpLook::Mortar,
        6 => LumpLook::Pinwheel,
        _ => LumpLook::Plain,
    }
}

/// A leech's take (bank food) at which its core is half the charge's cyan
/// (`LumpBody::fill`, `take / (take + LEECH_HALF)`): a leech took about
/// 280 in its first minute on planet 3 (mini-boss round 3), so a full one
/// reads as two-thirds cyan, never all of it (that is a crystal's).
const LEECH_HALF: f32 = 150.0;
mod args;
mod experiments;
mod input;
mod draw;
mod ending;
mod events;
mod headless;
mod bench;
mod hands;
mod runbot;
mod strat;
mod utility;
mod league;
mod menu;
mod squadmode;
mod uisize;
mod volume;
use uisize::*;
use volume::*;
use slime_mould::world::settings::{self, UiSize};
use squadmode::{squad_card_key, squad_help, SquadInput};
// The moved modules' functions, reached as before from here and from each
// other.
use args::*;
use bench::*;
use draw::*;
use hands::*;
use menu::*;
use slime_mould::world::start::StartMenu;
use headless::*;

/// The designed maps (`Reports/larder-maps-2026-11-11.md`, v2), N's order;
/// then the River, the first map with deep water (new ground kinds,
/// 2026-11-14, `Reports/ground-kinds-2026-11-14/`): two fords and a bridge.
const MAPS: [&str; 6] = ["fork", "nest", "rim", "flats", "passes", "river"];

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
    // Burn a line's id until the owner removed it (2026-11-15 19:51Z):
    // Guard took its place and its F (`Order::Guard`).
    GUARD = 4,
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
    // The build page's cancel (2026-11-14, the build-flow thread): click a
    // ghost to call it off for its whole price back (`Cmd::Unbuild`).
    CANCEL_BUILD = 29,
    // Card 2 (owner 2026-11-15 19:38Z): the one fire button that cycles the
    // stances (Z, X and C still give HOLD_FIRE, DEFEND and FREE), and the
    // squad's Heal and Lookouts off the squad panel.
    FIRE = 30,
    HEAL_ORDER = 31,
    LOOKOUTS = 32,
    // A building clicked with nothing else selected (owner 19:51Z, option
    // b): its card's actions. CANCEL_BUILD calls off a ghost there.
    DEMOLISH = 33,
    UPGRADE = 34,
    SWITCH = 35,
    // The build page's door (walls thread, owner 2026-11-16 14:24Z): a
    // two-click line, as WALL.
    DOOR = 36,
    // The build page's mortar (the turrets thread, owner 2026-11-17
    // 00:47Z: the first turret body from the run shop's tree).
    MORTAR = 37,
    // The tools page's Provoke and Still (the simulation-use plan's B1 and
    // B2, 2026-11-17): Mind statuses as test devices; runs leave them out.
    T_PROVOKE = 38,
    T_STILL = 39,
    // A turret's or mortar's card in a run (owner 2026-11-19: each turret
    // can be changed one by one): the next loadout it fires.
    TURRET_LOADOUT = 40,
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
        MORTAR => Some(Kind::Mortar),
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
        // as it calls.
        Kind::Siren => Some(4),
        // The turret's base under a mortar's tube, drawn in the view's
        // shader (5 is the larder's, 6 a pump's rig).
        Kind::Mortar => Some(7),
        // Drawn as a line, not a sprite.
        Kind::Wall { .. } => None,
    }
}

/// How long a shot's ray and its muzzle flash show, real seconds (the world
/// look's fight: `examples/worldlook.rs`).
const TRACER: f32 = 0.1;
const FLASH: f32 = 0.07;
/// How long a soldier's gun kicks back after a shot, seconds: it jumps
/// back at once and eases home over this (gun look, 2026-11-15).
const KICK: f32 = 0.14;
/// How far a soldier walks (dish cells) before its figure turns to the way
/// it is going (`App::step`).
const TURN: f32 = 0.3;
/// How fast a soldier's figure turns, radians a second: half a turn in a
/// third of a second. It snapped at once before (squad movement rework,
/// 2026-11-16), so a squad's figures flicked round 15 times a second as
/// they jostled or the cursor moved.
const TURN_RATE: f32 = 3.0 * std::f32::consts::PI;
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
const LOADOUTS: [(&str, &str); 68] = [
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
    // Bait (11-14 09:02Z, the owner's card) replaced the alarm fence:
    // a scent line every mould follows, here into a burning line.
    ("bait", "bait"),
    ("killzone", "bait+scorch"),
    // The owner's notes (11-14 15:06Z): rate of fire, cost cut, scent.
    ("rapid", "rate+rate+blast"),
    ("thrift", "thrift+bomb+thrift+bomb"),
    ("smother", "smother+smother"),
    ("mist", "mist"),
    ("firewall", "scorch+blast"),
    // Range (owner, 11-14 08:39Z: "there should be a range modifer card
    // too"): every soldier sees and shoots half as far again a card.
    ("long", "range+blast"),
    ("sniper", "range+range+pierce"),
    // Bullet sets' shapes (2026-11-16, the gun-system review's step A):
    // standard numbers, so only the shape differs from `plain`. A slug
    // shoves, a needle lances through three hits, buckshot fans four
    // pellets; and a payload takes its set's shape.
    ("slug", "ammo/plain/slug/1/100/100/100/100/1"),
    ("needle", "ammo/plain/needle/1/100/100/100/100/1"),
    ("buckshot", "ammo/plain/buckshot/1/100/100/100/100/1"),
    ("needle-blast", "ammo/blast/needle/1/100/100/100/100/3"),
    ("buck-gas", "ammo/gas/buckshot/1/100/100/100/100/3"),
    // Carriers (2026-11-16, step B): a bullet that carries the next cast
    // and lets it out where it hits (trigger) or where it stops (expire).
    ("plain-blast", "plain+blast"),
    ("trigger-blast", "trigger+plain+blast"),
    ("buck-trigger", "trigger+ammo/plain/buckshot/1/100/100/100/100/1+blast"),
    ("needle-deep", "expire+ammo/plain/needle/1/100/100/100/100/1+blast"),
    ("relay", "expire+plain+expire+plain+gas"),
    ("chain", "trigger+ammo/plain/buckshot/1/100/100/100/100/1+trigger+plain+blast"),
    // The plan's step-B film: a bomb that carries a gas against the two
    // fired in turn.
    ("bomb-gas", "bomb+gas"),
    ("trigger-bomb-gas", "trigger+bomb+gas"),
    // The shove cards (owner 2026-11-16 05:11Z: "corral and push the slime
    // onto areas. Into a kill zone"): a heaved shove toward the nearest
    // turret, or straight on with none in reach (Heave and Corral were two
    // cards until 2026-11-17), and one drawn back to the gun.
    ("heave", "heave+plain"),
    ("pull", "pull+heave+plain"),
    ("heave-slug", "heave+ammo/plain/slug/1/100/100/100/100/1"),
    // Multicast (2026-11-16, step B): the next cast fired beside in the
    // same shot; two copies fire three. The films' five-card combos
    // (multicast+multicast+plain+blast+gas 13, multicast+trigger+plain+
    // blast+gas 12) are over a default gun's budget of 10, so they are not
    // presets: `load=` takes them as card lists.
    ("multi-blast", "multicast+plain+blast"),
    ("multi-bomb-gas", "multicast+bomb+gas"),
    // Seek (2026-11-16, step C): a bullet that would pass beside slime
    // near its aim point strikes it; with buckshot each pellet, with split
    // each piece.
    ("seek", "seek+plain"),
    ("seek-buck", "seek+ammo/plain/buckshot/1/100/100/100/100/1"),
    ("seek-split", "seek+split+blast"),
    // The Mind bullets (2026-11-16, step D): a status where it hits, no
    // kill; with a plain bullet beside it, one that kills too.
    ("blind", "blind"),
    ("sour", "sour"),
    ("spin", "spin"),
    ("multi-blind", "multicast+plain+blind"),
    // Provoke and Still (2026-11-17): the devices' statuses as cards; a
    // provoked part charges the squad into its own plain fire.
    ("provoke", "provoke"),
    ("still", "still"),
    ("provoke-plain", "provoke+plain"),
    // The readers (2026-11-16, step D): each kills only what another card
    // set up, so alone it kills nothing; in turn with its setter, the
    // setter's sleepers or sick die at the next hit.
    ("shatter", "shatter"),
    ("detonate", "detonate"),
    ("gas-shatter", "gas+shatter"),
    ("germ-detonate", "germ+detonate"),
    // The route family (2026-11-17, step D): the flare moves the slime and
    // kills none; with a blast, every other bullet kills where it went.
    ("flare", "flare"),
    ("flare-blast", "flare+blast"),
    // The element family (2026-11-17, step D): the torch lights the plants
    // and the fire kills; with a blast, every other bullet kills at once.
    ("torch", "torch"),
    ("torch-blast", "torch+blast"),
    // Tinder (2026-11-17): every bullet lays dry brush along its flight;
    // with torches the line burns where they hit, with scorch as it is
    // laid.
    ("tinder-torch", "tinder+torch"),
    ("tinder-scorch", "tinder+scorch"),
];

/// The prefix of a rule key among `Args::keys` (`set=NAME=V`).
use slime_mould::build::switches::{RULE_KEY, WINDOW_KEY};

/// `Default` so a test or a tool names only the fields it sets (`..Default::default()`);
/// before, each of 18 literals spelled out all 23 fields and every new
/// field touched every one of them (whole-game review, 2026-11-15).
#[derive(Clone, Default)]
struct Args {
    /// `mode=squads`: the squad mode (`squadmode`, `Rules::squads`).
    squads: bool,
    /// `load=T,NAME` (look= only, 2026-11-16, the bullet-shape films):
    /// at game second T every squad loads the loadout NAME (`Cmd::Load`),
    /// so films of several loadouts play one game up to T and differ only
    /// after it. NAME may be a card list instead (`multicast+plain+blast`,
    /// 2026-11-16), for combos over a preset's budget.
    load_at: Option<(f32, String)>,
    /// `watch=X,Y,R` (look= only, 2026-11-16, the Mind films): each game
    /// second, the walkers within R world cells of (X, Y) and in the ring
    /// R to 2R round it (the look's walker count, as `cut=`'s), so a
    /// film's page can say how a hit's status scattered or held the
    /// slime there without the picture.
    watch: Option<(f32, f32, f32)>,
    /// `crustlook=T` (look= only, 2026-11-17, the simulation-use plan's
    /// E1a, before any engine rule): crust worked out offline from the
    /// look's read-back once a game second. A cell with a walker on it and
    /// trail thickness of T or more (the look's byte, 0-255) gains 1/60
    /// up to 1, else keeps 0.9942 (half in 2 minutes); no wipe by blasts
    /// or fire. Each picture gets a `-crust.png` beside it and a count
    /// line, to see whether crust shows roads or one plate under the body
    /// (the reviews' question).
    crust_look: Option<u8>,
    /// `light=T,X,Y,R` (2026-11-17, the crust fire films): a plain fire
    /// lit over R world units at (X, Y) at game second T (a burn burst,
    /// no kill); repeat for more.
    lights: Vec<(f32, f32, f32, f32)>,
    /// `lure_mark=T,X,Y,SECS` (look= only, 2026-11-18, the Enemy lane): a
    /// lure with no building (`LureMark`, the lure's field) at (X, Y),
    /// world units, from game second T for SECS; repeat for more.
    /// `lure=X,Y[,T]` stands a Lure building instead.
    lure_marks: Vec<(f32, f32, f32, f32)>,
    /// `trail=T,X1,Y1,X2,Y2,CARDS[,lit]` (2026-11-17, the element family's
    /// fence test): at game second T, the trails of CARDS (scorch, bait,
    /// mist, smother, tinder) laid along the line from (X1, Y1) to (X2, Y2),
    /// world units, as one bullet's leg (`rounds::trail_bursts`, Tinder's
    /// brush the whole line, as no soldier stands there: no `keep_clear`); `lit`
    /// also lights every disc (`squad::Burst::light` 1), so a tinder line
    /// burns along its length at once. Repeat for more.
    trails: Vec<(f32, [f32; 4], Vec<rounds::Card>, bool)>,
    /// `cross=X1,Y1,X2,Y2,D` (look= only, 2026-11-17, the game designer's
    /// crossing test): each game second, the living walkers beyond the
    /// line (left of X1,Y1 to X2,Y2 on the screen, within its span and D
    /// world units past it) and how many walkers have crossed to there
    /// from this side since the first second counted (by slot; a slot
    /// that died counts again only from where it next lives).
    cross: Option<[f32; 5]>,
    /// `fencepick=T` (look= only, 2026-11-17): at game second T, each
    /// crystal's centre (world units), its nearest walker and the walkers
    /// within 20 of it, to pick a crystal the slime is heading for.
    fence_pick: Option<f32>,
    /// `necks=T[,T...]` (look= only, 2026-11-17, the gun-card plan's cut
    /// count): at each game second T, the walkers in parts of the slime
    /// that only thin necks (2 or 4 dish cells wide) tie to a crystal with
    /// charge, so one cut there would leave them without food
    /// (`mould::necks`).
    necks: Vec<f32>,
    /// `tload=T,NAME` (look= only, the turrets thread, 2026-11-16): at game
    /// second T every turret loads NAME (`Cmd::Load(TURRET_DECK, ..)`,
    /// `Game::turret_deck`), a preset or a card list as `load=`.
    tload_at: Option<(f32, String)>,
    /// `card=A/B/..` (look= only, the command card's films, 2026-11-15):
    /// what the player does before frame N's picture, one step a frame
    /// (`headless::card_step`): main, build, tools, free, pick, fire, and
    /// the building card's upgrade, demolish, switch.
    card: Vec<String>,
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
    /// `boss_look=NAME`: every mini-boss's look (`LumpLook`) whatever its
    /// kind, for the look's A/B (else `kind_look`).
    boss_look: Option<LumpLook>,
    /// `follow=boss`: the near view follows the first live mini-boss
    /// (its centre; the squad or `centre=` when there is none), for films
    /// of the look.
    follow_boss: bool,
    /// `scent=1`: open with the scent view on (F2 toggles it), for films.
    scent: bool,
    /// `aim=STRIKE,EVERY,SPREAD`: a shot's strike radius, game seconds
    /// between one soldier's shots, and aim spread, dish cells
    /// (`Rules::strike`, `shot_every`, `spread`): the look A/B of a
    /// scattered smaller strike against the round one.
    aim: Option<[f32; 3]>,
    /// `gun=KIND`, `gun=KIND:SEED`, `gun=roll` or `gun=roll:SEED`: the gun
    /// the first squad starts with (`mould::guns`): a kind's average
    /// (rifle, carbine, long, scatter), or one rolled for the planet's
    /// depth (`map=planet:DEPTH`, else 1). None: the standard rifle.
    gun: Option<String>,
    /// `ammo=N`: the first squad starts with the Nth load of test bullets
    /// (`input::bullet_load`, as Ctrl+H rolls them) for its gun. Or
    /// `ammo=ROUND`: those cards, in `rounds::parse`'s text (rolled bullets
    /// as `Ammo::word`), so a film can set each bullet's numbers.
    ammo: Option<String>,
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
    /// `spray=S[,SHARE]`: how long the slime a shot kills flies out of the
    /// hit, in seconds (`Rules::spray_secs`; 0 none), and the share of the
    /// kills that fly (`Rules::spray_share`, 1 all), for films and A/Bs.
    spray: Option<(f32, f32)>,
    /// `shake=K`: camera shake on blasts (the whole-game review's D6; the
    /// owner 11-15 21:41Z: "Let me see the video"), K times the normal
    /// strength; 0, the default, none.
    shake: f32,
    /// `hitstop=K`: the game holds still for a moment when a big blast
    /// lands (D6), K times the normal hold; 0, the default, none.
    hitstop: f32,
    /// `real=1`: a headless frame sequence steps the window's way, a
    /// frame's real seconds at a time from the first frame on, so a
    /// hit-stop shows as held frames (the frames' times are real time, not
    /// game time; `headless`).
    real: bool,
    /// `mass=M`: one walker's fight weight (`Rules::mass`); None follows the
    /// start walkers a cell (`Rules::mass_for`).
    mass: Option<f32>,
    /// `rounds=NAME` (or its cards, such as `thick+wide+blast`): the
    /// squad's loadout at the start, an index into `LOADOUTS`.
    round: usize,
    /// `bullet=SPEED`: dish cells a game second a shot flies
    /// (`Rules::bullet_speed`; 0 is the instant ray, to compare).
    bullet: Option<f32>,
    /// A roguelike run's level (`mould::run`): this round plays the run's
    /// current planet, with its upgrades. `run=SEED` starts a run on its
    /// first planet; the start menu's Run does too. `run=SEED:DEPTH`
    /// lands on planet DEPTH.
    run: Option<Run>,
    /// `dev=1`: the run's shop has the dev rows (jump planets, points,
    /// redraw cards); also the options page's Dev mode.
    dev: bool,
    /// `grow=SECS`: a run level's head start (`run::head_start`) set by
    /// hand, to compare (`grow=0` lands on a fresh slime).
    grow: Option<f32>,
    /// The gun range (`Run::range`; owner, 11-15 02:55Z: "a test
    /// environment ... where I can play with the guns and cards and
    /// bullets"): this round is the sandbox's, the squad armed from the
    /// range's armory (H opens it over the fight), with no time limit and a
    /// bank that does not run dry. `range=1` (or `range=SEED`) starts one.
    range: Option<Run>,
}

/// The gun range's clock (game seconds): ten hours, so it never ends on
/// time.
const RANGE_CAP: f32 = 36_000.0;
/// The gun range's starting bank: soldiers to buy without a thought.
const RANGE_BANK: f32 = 100_000.0;

/// A shot being drawn: its ray, whether it hit slime, and its age.
#[derive(Clone, Copy)]
struct Tracer {
    from: (f32, f32),
    to: (f32, f32),
    hit: bool,
    age: f32,
    /// The strike's radius, fine cells (`Shot::r`): a hit lights only
    /// what it kills (owner 20:22Z).
    r: f32,
    /// Seconds into the step's leg the drawn bullet reaches the hit, at
    /// its speed (`App::between`): its light waits for it.
    delay: f32,
}

/// What the window keeps of each soldier beside the game: the way it
/// faces, its muzzle flash, and whether it was alive last frame.
#[derive(Clone, Copy)]
struct Look {
    facing: f32,
    /// The way its figure turns toward, a little each frame (`TURN_RATE`;
    /// a shot or a take still turns it at once).
    turn_to: f32,
    flash: f32,
    /// Seconds left of the pulse at its hands from its last take from a
    /// crystal (`Game::last_takes`, `TAKE` long).
    take: f32,
    at: (f32, f32),
    alive: bool,
    /// Its health and load at the last step seen (`step_notes`), the
    /// damage the slime has done since its last flinch, the seconds left
    /// of that flinch (`FLINCH` long) and the way the bite came from
    /// (building looks, 11-14: "a flinch and a brief light where slime
    /// bites, from the real damage").
    hp: f32,
    carry: f32,
    hurt: f32,
    flinch: f32,
    bite: f32,
    /// Seconds left of its gun's kick from its last shot (`KICK` long),
    /// the cast that shot carried (`Shot::cast`: its flash takes the
    /// payload's colour) and the shots it has fired, which vary each
    /// muzzle flash (gun look, 2026-11-15).
    kick: f32,
    cast: u8,
    shots: u32,
    /// Seconds since it fell (counting while it lies dead), which spreads
    /// its pool of blood over `POOL_GROW` (`Marks::fallen`).
    fallen: f32,
}

impl Look {
    fn new(s: &slime_mould::mould::larder_world::Soldier) -> Look {
        Look { facing: -std::f32::consts::FRAC_PI_2, turn_to: -std::f32::consts::FRAC_PI_2, flash: 0.0, take: 0.0, at: (s.x, s.y), alive: s.alive, hp: s.hp, carry: s.carry, hurt: 0.0, flinch: 0.0, bite: 0.0, kick: 0.0, cast: 0, shots: 0, fallen: if s.alive { 0.0 } else { POOL_GROW } }
    }
}

/// What the window keeps of each building beside the game
/// (`Game::buildings` by index): where a turret's head or a pump's hose
/// points, the seconds left of a turret's last shot's kick and flash, of
/// a siren's call (`SIREN_CALL`) and of an outpost's flare as a load is
/// banked there (`BANKED`).
#[derive(Clone, Copy, Default)]
struct BuildingLook {
    facing: f32,
    shot: f32,
    call: f32,
    bank: f32,
}

/// How long a siren's call shows: its sound ring runs out to the alarm's
/// reach (`BuildRules::siren_r`) in this long and fades.
const SIREN_CALL: f32 = 0.7;
/// The slime's damage to a soldier, hp, that makes one flinch: a soldier
/// at the edge of a body flinches now and then, one wading in many times a
/// second (each walker touching bites `Rules::drain` a second).
const FLINCH_HP: f32 = 3.0;
/// How long a flinch lasts, seconds.
const FLINCH: f32 = 0.2;
/// How long a fallen soldier's pool of blood takes to spread, seconds
/// (`Marks::fallen`; the owner's playtest 2026-11-18 16:12Z).
const POOL_GROW: f32 = 2.5;

/// The camera shake's wobble at full trauma, screen pixels, and how fast
/// the trauma falls, a second (`App::shake`; D6, 11-15).
const SHAKE_PX: f32 = 12.0;
const SHAKE_FALL: f32 = 2.2;

/// **Feel a blast** (the whole-game review's D6, 11-15: camera shake and
/// hit-stop, both off unless `shake=` or `hitstop=`): each blast since
/// `felt` adds to the shake by its size (its kills over 150 and its throw
/// over 400, up to 1), 0.12 to 0.72 of a full shake at `shake=1` (a bomb
/// of 6 kills and 80 thrown, size 0.24, adds 0.26); the biggest, if at
/// least a fifth of full size and none in the last 0.6 s, holds the game
/// still for 0.05 to 0.12 s at `hitstop=1` (that bomb: 0.067 s, two frames
/// at 30). Plain hits do neither: in a fight every volley would.
fn feel(pops: &[slime_mould::mould::rounds::Pop], felt: &mut u64, trauma: &mut f32, stop: &mut f32, stop_cool: &mut f32, (shake, hold): (f32, f32)) {
    let mut big = 0.0f32;
    for p in pops.iter().rev().take_while(|p| p.tick > *felt) {
        let size = (p.killed as f32 / 150.0 + p.flung as f32 / 400.0).min(1.0);
        *trauma = (*trauma + shake * (0.12 + 0.6 * size)).min(1.0);
        big = big.max(size);
    }
    if let Some(p) = pops.last() {
        *felt = (*felt).max(p.tick);
    }
    if hold > 0.0 && big >= 0.2 && *stop_cool <= 0.0 {
        *stop = hold * (0.05 + 0.07 * big);
        *stop_cool = 0.6;
    }
}

/// How long a turret's kick and muzzle flash last, seconds.
const TURRET_SHOT: f32 = 0.16;
/// How fast a turret's head turns toward its aim between shots, radians a
/// second (it snaps to the shot's line when it fires, so the shot always
/// leaves the barrels).
const TURRET_TURN: f32 = 5.0;

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
    /// The ground buffer's codes as last written, and the cells a timed
    /// refresh reads (`refresh_ground`).
    codes: Vec<u32>,
    live: Live,
    /// The dish's wall cells as the ground last drew them: a wall going
    /// up is the cells that differ.
    wall_mask: Vec<bool>,
    /// The timed and wall refreshes this game: how many, their summed
    /// and longest ms, and the longest read-back (the bench prints them).
    ground_cost: (u32, f64, f64, f64),
    /// Each crystal cell's depth in its crystal (`look::crystal_depth`),
    /// worked out once a world: the crystals keep their cells, and the
    /// depth was most of each ground rebuild (3.9-6.1 ms on the cloud CPU).
    depth: Vec<u8>,
    looks: Vec<Look>,
    blooks: Vec<BuildingLook>,
    tracers: Vec<Tracer>,
    /// This frame's point lights (bullets, bursts, muzzles, lodged casts),
    /// filled with the marks (`draw.rs`).
    lights: Vec<Light>,
    paused: bool,
    menu_paused: bool,
    /// The loadout panel (`open_loadouts`): paused before it, and the
    /// loadout its squad used, so closing changes only what was picked.
    field_paused: bool,
    field_was: usize,
    fast: bool,
    /// Each step's GPU ticks spread over the frames until the next step is
    /// due (`GpuWorld::spread`, 2026-11-12); `spread=0` records them all on
    /// the step's frame, as before.
    spread: bool,
    /// Game seconds owed to the game (the accumulator).
    owed: f32,
    /// The end's beat (`ending::Beat`), in the window; None in a headless
    /// run, whose end card is at once.
    beat: Option<ending::Beat>,
    /// The camera's shake, 0-1 (`shake=`; D6): each blast adds to it by
    /// its size, it falls by `SHAKE_FALL` a second, and the camera wobbles
    /// by its square (`shake`).
    trauma: f32,
    /// The window's real seconds, for the shake's wobble.
    clock: f32,
    /// A hit-stop (`hitstop=`; D6): real seconds the game holds still, and
    /// until another may start.
    stop: f32,
    stop_cool: f32,
    /// The newest blast's tick already felt (`feel`).
    felt: u64,
    /// How far into the next step a headless picture is taken, 0-1 (it
    /// draws whole steps; `between`), or None in the window.
    sub: Option<f32>,
    last_frame: Instant,
    shift: bool,
    /// The soldier the last build with nobody selected sent (`builders`):
    /// Shift + another build queues on it.
    last_sent: Option<usize>,
    ctrl: bool,
    /// The larder is selected (a click on it with no soldier picked): its
    /// card shows, and right click sets its rally point.
    larder_sel: bool,
    /// The building selected (a click on it with no soldier or squad;
    /// owner 2026-11-15 19:51Z, option b): the panel shows its state and
    /// the card its actions (`App::building_card`).
    building_sel: Option<usize>,
    /// Alerts already given: crystals dry, the core taken, the larder
    /// robbed (game second), out of food.
    dry: Vec<bool>,
    core_told: bool,
    /// The broken mould (`Rules::broken`) told.
    broken_told: bool,
    /// Raids the HUD has told of (`Game::raids`).
    raids_told: usize,
    robbed_at: Option<f32>,
    stolen: f32,
    broke: bool,
    /// Bank samples (game second, bank) for the food rate.
    bank_log: std::collections::VecDeque<(f32, f32)>,
    /// Seconds left of the larder's flare (`BANKED`), set by a carrier
    /// banking there (`step_notes`).
    banked: f32,
    /// Frame times (ms) since the last report, and when it was.
    frame_ms: Vec<f32>,
    steps: u32,
    /// Steps finished on a frame once their results were in, and those
    /// finished only when the next step was due (the bench prints both).
    finished: (u32, u32),
    /// The game tick of the last step whose notes the window took (the
    /// kit, the rounds' casts, the shots' marks; `App::catch_up`).
    seen: u64,
    report_at: Instant,
    saved: bool,
    /// The play log (`mould::playlog`, whole-game review W13, 11-15): a
    /// file a round in `play-logs/` beside the game, a line a game second,
    /// in the window only (not the headless films) and not for the dish
    /// behind the start menu. The path, the writer, the last second
    /// written, whether the end is written, and the notes dropped (F9).
    log_notes: u32,
    log_path: Option<String>,
    playlog: slime_mould::mould::playlog::Playlog,
    log_sec: u64,
    log_ended: bool,
    /// The gun's cards as the log last named them (`App::log_loadout`): a
    /// change mid-round (H in free play, the range's armory) writes a
    /// `gun t=..` line.
    log_gun: String,
    /// The command card shows the build page (Q).
    build_page: bool,
    /// The squad toolkit (`mould::kit`) and whether the card shows its page
    /// (E). Tools are not in the game's replay (the kit is outside `Game`).
    kit: Kit,
    /// The squad's round (`mould::rounds`, after Flashes of Chaos's ammo
    /// cards; 2026-11-12): `rounds=` at the start, H cycles `LOADOUTS`.
    rounds: Rounds,
    round: usize,
    /// The planet's depth this game (`map=planet:DEPTH`, else 1), and the
    /// test guns (Shift+H) and bullet loads (Ctrl+H) rolled so far.
    planet_depth: u32,
    gun_rolls: u32,
    ammo_rolls: u32,
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
    /// Experiments picked on the pause menu that wait for the next round
    /// (`experiments::pick_experiment`): switch key, choice.
    exp_next: Vec<(&'static str, &'static str)>,
    /// The roguelike run under way, between its levels too (`args.run` is
    /// the level being played; this is the run itself, in the shop).
    run: Option<Run>,
    /// The gun range's armory, between rounds too (`args.range` is the
    /// one the round was armed from).
    range: Option<Run>,
    /// The armory is up over a live range round (the start menu's shop
    /// page, the round paused under it), not the menu over its dish
    /// (`App::set_armory`).
    armory: bool,
    /// The minimap's switch (M) while the armory hides it.
    armory_minimap: bool,
    result: Result<(), String>,
    /// What this PC has seen of the game's "what is new" list, and the
    /// last build played here (`build::news`): read from the settings
    /// file when a window opens (`main`), never saved by the no-window
    /// films and tests, which must not mark the owner's news as read.
    memory: slime_mould::build::news::Memory,
}

/// The play log header's words for a round's keys: the switches off their
/// default, and every key given over the default game (the shape's own
/// are the default game; the rest are the command line's and the menu's).
fn log_keys(keys: &[(String, String)]) -> Vec<(&'static str, String)> {
    let (_, shape) = shape_keys(DEFAULT_SHAPE).expect("the default shape");
    let given: Vec<String> = keys.iter().filter(|(k, v)| !shape.iter().any(|(n, x)| n == k && x == v)).map(|(k, v)| format!("{k}={v}")).collect();
    vec![("switches", slime_mould::build::switches::log_word(keys)), ("keys", if given.is_empty() { "none".into() } else { given.join(",") })]
}

impl App {
    fn new(args: Args) -> Self {
        App {
            memory: slime_mould::build::news::Memory::unsaved(),
            name: args.map.clone().unwrap_or_else(|| "noise".into()),
            round: args.round,
            planet_depth: 1,
            gun_rolls: 0,
            ammo_rolls: 0,
            run: args.run.clone(),
            range: args.range.clone(),
            armory: false,
            armory_minimap: false,
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
            codes: Vec::new(),
            live: Live::default(),
            wall_mask: Vec::new(),
            ground_cost: (0, 0.0, 0.0, 0.0),
            depth: Vec::new(),
            looks: Vec::new(),
            blooks: Vec::new(),
            tracers: Vec::new(),
            lights: Vec::new(),
            paused: false,
            menu_paused: false,
            field_paused: false,
            field_was: 0,
            fast: false,
            spread: true,
            owed: 0.0,
            beat: None,
            trauma: 0.0,
            clock: 0.0,
            stop: 0.0,
            stop_cool: 0.0,
            felt: 0,
            sub: None,
            last_frame: Instant::now(),
            shift: false,
            last_sent: None,
            ctrl: false,
            larder_sel: false,
            building_sel: None,
            dry: Vec::new(),
            core_told: false,
            broken_told: false,
            raids_told: 0,
            robbed_at: None,
            stolen: 0.0,
            broke: false,
            bank_log: Default::default(),
            banked: 0.0,
            frame_ms: Vec::new(),
            steps: 0,
            finished: (0, 0),
            seen: 0,
            report_at: Instant::now(),
            saved: false,
            log_path: None,
            playlog: Default::default(),
            log_sec: 0,
            log_ended: false,
            log_notes: 0,
            log_gun: String::new(),
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
            exp_next: Vec::new(),
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
        // The pause menu's picks that wait for a new round (2026-11-19).
        self.take_experiments();
        // A switch's window keys (`win:NAME`, the Experiments tab's picks
        // and `try=`) over the command line's own, each round: the round's
        // args are built afresh from the command line (`round_args`), so a
        // pick turned back leaves nothing behind.
        for (k, v) in self.args.keys.clone() {
            if let Some(name) = k.strip_prefix(WINDOW_KEY) {
                experiments::set_window_key(&mut self.args, name, &v)?;
            }
        }
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
        // A run's level is a mission the run writes (`run::Level`), its
        // ground a hand-made map by name or, from the level generator, a
        // made one.
        let level = a.run.as_ref().map(Run::level);
        let mission = match (&level, &a.mission) {
            (Some(l), _) => Some(l.mission.clone()),
            (None, Some(name)) => {
                let path = slime_mould::build::root().join(format!("Reports/larder-campaign-2026-11-12/missions/{name}.txt"));
                Some(Mission::parse(&std::fs::read_to_string(&path).map_err(|e| format!("{}: {e}", path.display()))?)?)
            }
            (None, None) => None,
        };
        // A run's slime is shape C (the owner's default slime, 11-12: its
        // shared food, withering, panic and charge) with the tier's keys over
        // it, and C's slots grown with the level's walkers. Until 11-15 a run
        // played the bare race keys (shape A's), as a mission keeps its own
        // mould; that slime shed nine tenths of its walkers in its first
        // minute on every planet (the slime-enemy thread found why).
        if let Some(l) = &level {
            run_shape(&mut s, l.walkers, l.depth)?;
        }
        // The food the planet's slime eats to build a surge (`run::surge_food`).
        let surge_food = level.as_ref().map(|l| slime_mould::mould::run::surge_food(l.depth, l.walkers));
        // A ladder name not built yet (drifter, sprawler, trunk) plays
        // the plain mould, and says so.
        if let Some(t) = mission.as_ref().and_then(|m| m.tier.as_deref()) {
            match Tier::from_name(t) {
                Some(tier) => {
                    tier.settings(&mut s)?;
                    // In a run, what the tier does to you over its own keys
                    // (`run::does_to_you`).
                    if level.is_some() {
                        for &(k, v) in slime_mould::mould::run::does_to_you(tier).keys {
                            slime_mould::gpu::set(&mut s, k, v)?;
                        }
                    }
                    core_tier = Some(tier);
                }
                None => eprintln!("larder-world: tier {t} is not built yet; the plain mould plays"),
            }
        }
        let made = match level.map(|l| l.ground) {
            Some(RunGround::Made(m)) => Some(m),
            _ => None,
        };
        // Engine keys last, over the race world's and the mission's tier
        // (`set:` keys are the game's rules, set below).
        for (k, v) in a.keys.iter().filter(|(k, _)| !k.starts_with(RULE_KEY) && !k.starts_with(WINDOW_KEY)) {
            slime_mould::gpu::set(&mut s, k, v)?;
            if k == "tier" {
                core_tier = Tier::from_name(v);
            }
        }
        let mut walkers = s.walkers;
        let own = a.own || mission.as_ref().is_some_and(|m| m.own);
        let map_name = mission.as_ref().map(|m| m.map.clone()).or_else(|| a.map.clone());
        // The planet's depth (`map=planet:DEPTH`), what a rolled gun
        // (`gun=roll`, Shift+H) is rolled for.
        let mut depth = 1;
        let world = match &map_name {
            _ if made.is_some() => GpuWorld::with_map(device, queue, s, a.scale, made.as_ref().expect("a made map")),
            None if own => GpuWorld::garden(device, queue, s, a.seed, a.scale, a.walkers)?,
            Some(name) => {
                let dir = slime_mould::build::root().join("Reports/larder-maps-2026-11-11/maps");
                let (dir, name) = match name.rsplit_once('/') {
                    Some((d, n)) => (PathBuf::from(d), n.to_string()),
                    None => (dir, name.clone()),
                };
                // `planet`, `planet:DEPTH` or `planet:DEPTH:SEED`: a
                // generated planet (`mould::levelgen`), from `seed=`.
                let mut m = match slime_mould::mould::levelgen::spec(&name, a.seed) {
                    Some(made) => {
                        let (p, m) = made?;
                        depth = p.depth;
                        m
                    }
                    None => Map::load(&dir, &name)?,
                };
                // A run's walkers go on the planet's open cells
                // (`run::open_scale`), not its whole area.
                let mut s = s;
                if a.run.is_some() {
                    let open = slime_mould::mould::run::open_scale(&m);
                    s.walkers = ((s.walkers as f32) * open).round().max(1.0) as _;
                    s.capacity = ((s.capacity as f32) * open).round().max(s.walkers as f32) as _;
                    walkers = s.walkers;
                }
                // Plants (11-14): the stand-in on a map without its own,
                // until levelgen writes each biome's (`plants=0` off).
                if s.plants {
                    slime_mould::mould::plants::stand_in(&mut m, a.seed as u32, 1.0);
                }
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
        // The run's upgrades over the level's rules.
        if let Some(r) = &a.run {
            r.rules(&mut rules);
        }
        if let Some(f) = surge_food {
            rules.surge_food = f;
        }
        // The gun range: no clock to beat and a bank that never runs dry,
        // so soldiers can always be bought (B) and the round never ends on
        // time or a wiped squad.
        if a.range.is_some() {
            rules.cap = RANGE_CAP;
            rules.start_bank = rules.start_bank.max(RANGE_BANK);
        }
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
        if let Some((s, share)) = a.spray {
            (rules.spray_secs, rules.spray_share) = (s, share);
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
        // A run's big planets: the slime grows alone first (`run::head_start`,
        // the planet review's finding 5, 11-15), and the clock starts on
        // landing (`Game::landed`).
        let mut world = world;
        if let Some(r) = &a.run {
            let grow = a.grow.unwrap_or(r.level().grow);
            if grow > 0.0 {
                let t0 = std::time::Instant::now();
                slime_mould::mould::larder_world::head_start(&mut world, grow, &rules);
                eprintln!("head start: the slime grew {grow:.0} s alone in {:.1} s", t0.elapsed().as_secs_f32());
            }
        }
        // Rule keys (`set=NAME=V`, 11-15, W44) last, over everything above.
        for (k, v) in a.keys.iter().filter_map(|(k, v)| k.strip_prefix(RULE_KEY).map(|k| (k, v))) {
            let v: f32 = v.parse().map_err(|e| format!("set={k}={v}: {e}"))?;
            if !rules.set(k, v) && !RoundRules::default().set(k, v) {
                return Err(format!("set={k}: no such rule"));
            }
        }
        let mut game = Game::new(world, rules, a.seed).with_tier(core_tier);
        self.sq = a.squads.then(SquadInput::new);
        if a.squads {
            game.apply(Cmd::Active(0));
        }
        self.planet_depth = depth;
        if let Some(spec) = &a.gun {
            // Every soldier starts in squad 0 (both halves of a split
            // copy its gun, `Squad::offshoot`).
            game.apply(Cmd::Arm(0, Some(gun_of(spec, a.seed, depth)?)));
        } else if let Some(r) = &a.run {
            // A run lands with the gun it carries, upgraded (`Run::armed`).
            game.apply(Cmd::Arm(0, Some(r.armed())));
        } else if let Some(r) = &a.range {
            // The range: the armory's gun as it is.
            game.apply(Cmd::Arm(0, Some(r.carried())));
        }
        let mut kit = KitRules::default();
        if let Some(r) = &a.run {
            r.kit(&mut kit);
        }
        self.kit = Kit::new(kit);
        // A run's gun is the cards it bought (`Run::gun`), the range's the
        // ones put in at its armory; H does not cycle either.
        let armed = a.run.as_ref().or(a.range.as_ref());
        let gun = match armed {
            Some(r) => r.squad_cards(),
            None => rounds::parse(LOADOUTS[self.round].1).unwrap_or_default(),
        };
        let budget = armed.map_or(RoundRules::default().budget, Run::budget);
        let mut rr = RoundRules { budget, ..RoundRules::default() };
        // The round rules' `set=` keys (the Mind bullets', 2026-11-16).
        for (k, v) in a.keys.iter().filter_map(|(k, v)| k.strip_prefix(RULE_KEY).map(|k| (k, v))) {
            rr.set(k, v.parse().map_err(|e| format!("set={k}={v}: {e}"))?);
        }
        self.rounds = Rounds::new(gun, rr);
        // A run's turrets fire its turret deck (`Run::turret_cards`, the
        // turrets thread): none without turrets or cards for them.
        if let Some(r) = armed {
            let t = r.turret_cards();
            if !t.is_empty() {
                game.apply(Cmd::Load(slime_mould::mould::larder_world::TURRET_DECK, Some(rounds::Deck::new(&t))));
            }
        }
        if let Some(v) = &a.ammo {
            let cards = match v.parse::<u32>() {
                Ok(n) => {
                    self.ammo_rolls = n + 1;
                    input::bullet_load(&game.gun(0), a.seed, n, depth)
                }
                Err(_) => rounds::parse(v).unwrap_or_default(),
            };
            game.apply(Cmd::Load(0, Some(rounds::Deck::new(&cards))));
            // Films and reports quote the load, so name it in the log.
            for c in &cards {
                let line = c.ammo().map_or(String::new(), |b| b.line());
                eprintln!("ammo: {}: {line}", c.title());
            }
        }
        self.mission = mission.map(|m| Tracker::new(m, &game));
        self.mission_sec = 0;
        let e = &game.world.e;
        let size = (e.dish.w, e.dish.h);
        self.charge0 = game.world.crystals().iter().map(|c| c.1).collect();
        self.dry = vec![false; self.charge0.len()];
        self.depth = look::crystal_depth(e.dish.w, e.dish.w * e.dish.h, &e.dish.pile_of);
        let codes = ground_codes(&game, &self.depth, None, None);
        let ground = look::ground_buffer(device, queue, &codes);
        self.live = live_cells(&game, &self.live);
        self.wall_mask = (0..codes.len()).map(|i| game.world.e.dish.wall.get(i).copied().unwrap_or(false)).collect();
        self.codes = codes;
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
            // The dense words, always where the world grows: a lump's
            // members are marked in them (bits 25-27, the mini-bosses,
            // 2026-11-16). The ripening's warmth (`ripe_at`) is 0 since
            // condensing was deleted (11-17); the view's bronze reads bit
            // 16, which nothing sets now.
            let dense = e.grows;
            v.set_dense(dense.then_some((&e.news, &e.state)), (e.dense_at / 4) as u32 + slime_mould::gpu::DENSE_HEAD as u32, 0);
            // Plants, their fire and the char (11-14), from the same buffer.
            v.set_plants(e.plants.then_some((&e.news, &e.state)), (e.plant_at / 4) as u32 + slime_mould::gpu::PLANT_HEAD as u32, e.dish.flora_of.unwrap_or(0) as u32);
            // The crust (2026-11-17, the simulation-use plan's E1a), from
            // the same buffer, when the world has one (`crust=1`).
            v.set_crust((e.crust_at > 0).then_some((&e.news, &e.state)), (e.crust_at / 4) as u32);
            // Set slime (E2b), from the block after the crust's set words.
            v.set_setting((e.crust_at / 4) as u32 + e.cells + e.n);
            // The Mind statuses on the slime they hold (Look A5,
            // 2026-11-18): each slot's Mind word, after the gas's
            // (`Engine::mind_words`), fading over a status's last 1.5 s.
            let mind_at = (e.gas_at / 4) as u32 + slime_mould::gpu::GAS_SLOTS as u32 + e.n;
            let fine_tps = game.rules.tps as f32 * game.world.fine_steps() as f32;
            v.set_minds(Some((&e.news, &e.state)), mind_at, (1.5 * fine_tps).round() as u32);
            // The slime's own look (Look B, 2026-11-19), by its tier, and a
            // walker's pace in dish cells over the look's streak.
            let pace = game.world.s.step / game.world.scale() * fine_tps;
            v.set_slime_look(slime_mould::world::view::SlimeLook::of(core_tier), pace * slime_mould::world::view::STREAK_SECS);
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
            // The engine's alarm field, so a siren's call (and a shot's
            // alarm) is the field itself spreading and fading (the art
            // director, 11-14: not rings timed from the call); full at a
            // siren's call. Drawn on the slime it reaches, as the slime's
            // own colour (the look lane, 11-17; the `alarm_look=` key that
            // scaled the old haze went with the haze).
            v.set_alarm(Some([&e.germ[0], &e.germ[1]]), game.rules.build.siren_alarm);
            if self.args.scent {
                v.scent = true;
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
        (self.ground_at, self.ground_walls, self.ground_cost) = (0.0, 0, (0, 0.0, 0.0, 0.0));
        self.looks = game.soldiers.iter().map(Look::new).collect();
        self.blooks.clear();
        self.tracers.clear();
        self.controls.selected.clear();
        self.controls.burn_lines = false;
        self.building_sel = None;
        self.controls.groups = Default::default();
        self.hud.cancel_targeting();
        (self.owed, self.core_told, self.robbed_at, self.stolen, self.broke, self.saved) = (0.0, false, None, 0.0, false, false);
        self.broken_told = false;
        self.raids_told = 0;
        self.bank_log.clear();
        self.banked = 0.0;
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
        self.seen = 0;
        // The end's beat (`ending`), in the window only.
        self.beat = self.window.is_some().then(ending::Beat::default);
        if let Some(tr) = &self.mission {
            self.name = tr.mission.map.clone();
        }
        if let Some(w) = &self.window {
            // The build at the end (11-17: "the build tells Scott what
            // it holds"), so a screenshot or a note names the code played.
            w.set_title(&format!("Larder World: {} (seed {}) - {}", map_title(&self.name, self.args.seed), self.args.seed, slime_mould::build::build().short()));
        }
        self.open_log();
        match &self.mission {
            Some(tr) => {
                self.hud.alert(&format!("{}: {}", tr.mission.name, tr.mission.says), None, theme::ACCENT);
                // What the run's tier does to you (`run::does_to_you`).
                if let Some(r) = self.args.run.as_ref().filter(|r| !r.range) {
                    let does = slime_mould::mould::run::does_to_you(r.level().tier).says;
                    if !does.is_empty() {
                        self.hud.alert(&format!("{}: {does}", title_case(r.level().tier.name())), None, theme::WARN);
                    }
                }
            }
            None => self.hud.alert(&format!("{}: take the mould's core, or hold more crystals than it at 15:00", map_title(&self.name, self.args.seed)), None, theme::ACCENT),
        }
        Ok(())
    }

    /// A new round's play log (`log_path`), its header written: the map,
    /// seed, mode, and the mission's name and tier when there is one;
    /// since 11-17 the build (`build=NUMBER:COMMIT:DATE`), the switches
    /// off their default (`switches=crust:on,...` or `none`; the keys
    /// behind them and any other given are in `keys=`), and on a planet
    /// its number and, in a run, its slime.
    ///
    /// A round played out of the menu also marks this build as played on
    /// this PC (`build::news::Memory::played`): the menu's own dish behind
    /// it is not play.
    fn open_log(&mut self) {
        self.log_path = None;
        self.playlog = Default::default();
        self.log_sec = 0;
        self.log_ended = false;
        self.log_notes = 0;
        // The dish behind the start menu is not play: no log of it (until
        // 11-17 it wrote one, which the round picked from the menu then
        // wrote over; with a file a round it would fill the folder).
        if self.window.is_none() || self.menu.is_some() {
            return;
        }
        use slime_mould::mould::playlog;
        let when = playlog::stamp(std::time::SystemTime::now().duration_since(std::time::UNIX_EPOCH).map_or(0, |d| d.as_secs()));
        let path = format!("{}/{}", playlog::DIR, playlog::file_name(&when, &self.name, self.args.seed));
        self.memory.played();
        let mut extra = vec![("when", when), ("build", slime_mould::build::build().word()), ("mode", if self.args.squads { "squads" } else { "rts" }.to_string())];
        if let Some(tr) = &self.mission {
            extra.push(("mission", tr.mission.name.clone()));
            extra.push(("tier", tr.mission.tier.clone().unwrap_or_default()));
        }
        if let Some(r) = self.args.run.as_ref().filter(|r| !r.range) {
            extra.push(("planet", r.depth.to_string()));
            extra.push(("run", r.seed.to_string()));
            if self.mission.is_none() {
                extra.push(("tier", r.level().tier.name().to_string()));
            }
        } else if self.name.starts_with("planet") {
            extra.push(("planet", self.planet_depth.to_string()));
        }
        extra.extend(self.log_loadout());
        self.log_gun = rounds::name(&self.rounds.round);
        extra.extend(log_keys(&self.args.keys));
        let head = playlog::Playlog::head(&self.name, self.args.seed, &extra);
        match std::fs::create_dir_all(playlog::DIR).and_then(|()| std::fs::write(&path, head)) {
            Ok(()) => self.log_path = Some(path),
            Err(e) => eprintln!("larder-world: {path}: {e}"),
        }
    }

    /// **The loadout the round lands with**, for the play log's head
    /// (coordinator, 2026-11-18, reviewing Scott's playtest: "play logs
    /// don't record the army round, so we can't see which cards his
    /// gun/turrets carried"): `gun=` the cards the gun fires in turn, and
    /// in a run or at the range `turrets=` the turrets' deck and `bought=`
    /// the shop's upgrades owned, as key and level. The gun itself (kind,
    /// range, rate) is already the log's `arm` line, and the turrets' deck
    /// its `load 255` line, but only once a turret is built.
    fn log_loadout(&self) -> Vec<(&'static str, String)> {
        let mut v = vec![("gun", rounds::name(&self.rounds.round))];
        if let Some(r) = self.args.run.as_ref() {
            v.push(("turrets", rounds::name(&r.turret_cards())));
            let owned: Vec<String> = slime_mould::mould::run::ITEMS.iter().zip(r.owned).filter(|(_, n)| *n > 0).map(|(i, n)| format!("{}{n}", i.key)).collect();
            v.push(("bought", if owned.is_empty() { "none".to_string() } else { owned.join("+") }));
        }
        v
    }

    /// **Drops a note in the play log** (F9; the plan's play record,
    /// 11-17): the game second and where the camera is looking, numbered
    /// from 1 each round, with a word on the HUD so the player knows which
    /// number to name afterwards.
    fn drop_note(&mut self) {
        let Some(g) = self.game.as_ref().filter(|_| self.log_path.is_some()) else { return };
        self.log_notes += 1;
        let t = g.time();
        let line = slime_mould::mould::playlog::Playlog::note(self.log_notes, t, self.camera.centre(), self.camera.zoom);
        Self::log_write(&self.log_path, &line);
        self.hud.alert(&format!("Note {} dropped at {}:{:02} (it is in the play log)", self.log_notes, t as u32 / 60, t as u32 % 60), None, theme::ACCENT);
    }

    /// Appends `text` to the play log (nothing without one).
    fn log_write(path: &Option<String>, text: &str) {
        use std::io::Write;
        let Some(path) = path else { return };
        if let Ok(mut f) = std::fs::OpenOptions::new().append(true).open(path) {
            let _ = f.write_all(text.as_bytes());
        }
    }

    /// The round's commands, to replay it (`Game::replay`).
    fn save_replay(&mut self) {
        let Some(g) = self.game.as_ref().filter(|g| !g.log.is_empty() && !self.saved) else { return };
        let path = format!("larder-world-replay-{}-{}.txt", file_part(&self.name), self.args.seed);
        match std::fs::write(&path, g.replay_text(self.args.seed)) {
            Ok(()) => println!("larder-world: saved {path}"),
            Err(e) => eprintln!("larder-world: {path}: {e}"),
        }
        self.saved = true;
    }

    /// How far the frame is from the last finished step to the next, 0-1:
    /// the game seconds owed over a step. Bullets glide along the last
    /// step's legs by it (gun look, 2026-11-15): at 15 steps a second and
    /// 60 frames, each bullet sat still for four frames and then jumped
    /// its 0.067 s of flight (6 fine cells), so a shot read as a dash that
    /// blinked across the ground rather than flew.
    fn between(&self) -> f32 {
        if let Some(a) = self.sub {
            return a;
        }
        let Some(g) = self.game.as_ref() else { return 0.0 };
        let step = g.rules.frame_ticks as f32 / g.rules.tps as f32;
        if self.paused || g.end.is_some() { 1.0 } else { (self.owed / step).clamp(0.0, 1.0) }
    }

    /// The camera's shake now, world cells (`shake=`; D6): the square of
    /// the trauma times `SHAKE_PX` screen pixels, wobbling on two sums of
    /// sines at unrelated rates so it never settles into a pattern.
    fn shake(&self) -> (f32, f32) {
        let a = self.trauma * self.trauma * SHAKE_PX / self.camera.zoom.max(1e-3);
        let c = self.clock;
        (a * (0.6 * (c * 41.0).sin() + 0.4 * (c * 67.0 + 1.7).sin()), a * (0.6 * (c * 53.0 + 0.9).sin() + 0.4 * (c * 29.0 + 2.3).sin()))
    }

    /// Ages the window's notes of the game by `dt` seconds: tracers,
    /// muzzle flashes and kicks, takes and flinches.
    fn age_notes(&mut self, dt: f32) {
        for t in &mut self.tracers {
            t.age += dt;
        }
        self.tracers.retain(|t| t.age < TRACER + t.delay);
        for l in &mut self.looks {
            // Turning toward where it walks or aims, at `TURN_RATE`; held
            // while its muzzle flashes, which points along the shot, and
            // for good once it has fallen (its body lies as it fell).
            if !l.alive {
                l.fallen += dt;
            } else if l.flash <= 0.0 {
                let d = (l.turn_to - l.facing + std::f32::consts::PI).rem_euclid(std::f32::consts::TAU) - std::f32::consts::PI;
                l.facing += d.clamp(-TURN_RATE * dt, TURN_RATE * dt);
            }
            l.flash = (l.flash - dt).max(0.0);
            l.kick = (l.kick - dt).max(0.0);
            l.take = (l.take - dt).max(0.0);
            l.flinch = (l.flinch - dt).max(0.0);
        }
        for b in &mut self.blooks {
            b.call = (b.call - dt).max(0.0);
            b.bank = (b.bank - dt).max(0.0);
        }
    }

    /// Steps the game for `dt` real seconds and keeps the window's notes
    /// (facing, flashes, tracers, alerts).
    /// **The mission follows the game** once a game second, with the GPU
    /// world's mould grid (an occupancy read-back, PR #101), and at once
    /// when the game has ended, so the end screen is the mission's.
    fn follow_mission(&mut self) {
        let (Some(g), Some(tr)) = (self.game.as_ref(), self.mission.as_mut()) else { return };
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

    fn step(&mut self, dt: f32) {
        self.clock += dt;
        self.trauma = (self.trauma - SHAKE_FALL * dt).max(0.0);
        self.stop_cool = (self.stop_cool - dt).max(0.0);
        // A hit-stop holds the game and its notes still (the flash, the
        // tracers, the burst's light stay where they are); the step in
        // flight on the GPU waits for the next frame.
        if self.stop > 0.0 {
            self.stop = (self.stop - dt).max(0.0);
            return;
        }
        self.age_notes(dt);
        // A step a command finished since the last frame (`catch_up`).
        self.catch_up();
        let Some(g) = self.game.as_mut() else { return };
        turn_turrets(g, &mut self.blooks, dt);
        self.banked = (self.banked - dt).max(0.0);
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
            feel(&self.rounds.pops, &mut self.felt, &mut self.trauma, &mut self.stop, &mut self.stop_cool, (self.args.shake, self.args.hitstop));
            self.finished.0 += 1;
            shot_marks(g, &mut self.tracers, &mut self.looks, self.listener.as_mut());
            turret_fired(g, &mut self.blooks);
        step_notes(g, &mut self.blooks, &mut self.looks, &mut self.banked);
            self.seen = g.world.ticks();
        } else if g.in_flight() {
            // Its next share of ticks (`GpuWorld::spread`).
            g.world.frame_more();
        }
        if self.paused || g.end.is_some() || mission_over {
            self.owed = 0.0;
            // A game that ended in the step finished above still gets the
            // mission's verdict, so its end screen is the mission's (a run's
            // "On to the shop"). Missing here, a planet won by its last
            // walker dying (WIN_SHARE 0, #487) ended the Game before the
            // mission saw it, and a run showed only "Play again" (the
            // owner's play, 2026-11-18 17:20Z).
            let (paused, ended) = (self.paused, g.end.is_some() || mission_over);
            self.follow_mission();
            // The end plays out in the picture before its card (`ending`).
            if ended && !paused {
                self.play_on(dt);
            }
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
                feel(&self.rounds.pops, &mut self.felt, &mut self.trauma, &mut self.stop, &mut self.stop_cool, (self.args.shake, self.args.hitstop));
                self.finished.1 += 1;
                shot_marks(g, &mut self.tracers, &mut self.looks, self.listener.as_mut());
                turret_fired(g, &mut self.blooks);
                step_notes(g, &mut self.blooks, &mut self.looks, &mut self.banked);
                self.seen = g.world.ticks();
                if g.end.is_some() {
                    break;
                }
            }
            // The squad mode's aim, between whole steps (`SquadInput::aim`).
            if let Some(sq) = self.sq.as_mut() {
                sq.aim(g);
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
        self.follow_mission();
        let Some(g) = self.game.as_mut() else { return };
        let t = g.time();
        // The play log's line for each whole game second passed.
        if self.log_path.is_some() {
            let sec = (g.settled_ticks().saturating_sub(g.landed)) / g.rules.tps as u64;
            if sec > self.log_sec {
                self.log_sec = sec;
                let mut line = self.playlog.second(g);
                let gun = rounds::name(&self.rounds.round);
                if gun != self.log_gun {
                    line += &format!("gun t={t:.1} {gun}\n");
                    self.log_gun = gun;
                }
                Self::log_write(&self.log_path, &line);
            }
            let verdict = self.mission.as_ref().and_then(|m| m.verdict.map(|_| m.cause_line()));
            if !self.log_ended && (g.end.is_some() || verdict.is_some()) {
                self.log_ended = true;
                let why = verdict.unwrap_or_else(|| g.end.map(|e| e.band().to_string()).unwrap_or_default());
                let text = format!("{}end t={t:.0} {why}\n", self.playlog.commands(g));
                Self::log_write(&self.log_path, &text);
            }
        }
        // A soldier the game made inside these steps (`Rules::reserve`'s
        // free recruit) joins the notes too.
        join_looks(g, &mut self.looks);
        // Where each soldier's gun points between shots (bug sweep,
        // 2026-11-14, after "the gun animation gets messed up"): in the
        // squad mode the controlled squad's at the cursor, its aim, and a
        // volleying squad's at its volley point; else the way it walks,
        // once it has gone `TURN` since it last turned. Before, any move
        // over 0.05 fine cells turned it, so the shoves of elbow room and a
        // formation settling spun the guns between shots.
        let cursor = self.sq.as_ref().and_then(|sq| sq.want);
        let turn = TURN * g.world.scale();
        for (k, s) in g.soldiers.iter().enumerate() {
            let l = &mut self.looks[k];
            let sq = g.squads.get(s.squad as usize).filter(|_| g.rules.squads);
            let aim = sq.and_then(|sq| sq.fire).or_else(|| cursor.filter(|_| g.active == Some(s.squad as usize) && g.rules.squads));
            let (dx, dy) = (s.x - l.at.0, s.y - l.at.1);
            // With `Rules::steer` a soldier's velocity says where it walks:
            // it turns once it walks at a third of its speed in the slime it
            // wades (`Game::wade`; of full speed, one wading never turned).
            let walking = g.rules.squads && g.rules.steer && s.vx.hypot(s.vy) > g.rules.speed * g.world.scale() * g.wade(k) / 3.0;
            if let Some(p) = aim.filter(|_| s.alive) {
                if (p.0 - s.x).hypot(p.1 - s.y) > 1e-3 {
                    l.turn_to = (p.1 - s.y).atan2(p.0 - s.x);
                }
                l.at = (s.x, s.y);
            } else if walking {
                l.turn_to = s.vy.atan2(s.vx);
                l.at = (s.x, s.y);
            } else if dx.hypot(dy) > turn && !(g.rules.squads && g.rules.steer) {
                l.turn_to = dy.atan2(dx);
                l.at = (s.x, s.y);
            }
            if l.alive && !s.alive {
                self.hud.alert("A soldier has fallen", Some((s.x, s.y)), theme::BAD);
                l.fallen = 0.0;
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
        // A raid the slime sends (`Rules::raid`, 11-15): told once, where
        // it goes, so the player can look before it lands.
        if g.raids.sent.len() > self.raids_told {
            self.raids_told = g.raids.sent.len();
            if let Some(d) = g.raids.sent.last() {
                self.hud.alert(&raid_words(g, d), Some(d.to), theme::BAD);
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
        self.bank_log.push_back((t, g.income()));
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
        self.catch_up();
        if let Some(g) = self.game.as_mut().filter(|g| g.in_flight()) {
            g.finish_step();
        }
        // The step's notes, as `step` takes them: rounds read its hits (the
        // headless looks settle every step), and the kit restocks and sets
        // off its mines. (Before the review after the bug sweep,
        // 2026-11-14, this was a copy without `kit_between`, so no
        // headless run ever ran the kit.)
        self.catch_up();
    }

    /// The window's notes of a step a command finished: `Game::apply`
    /// finishes a running step first, and before the bug sweep
    /// (2026-11-14) the window never saw that step. Its bullets' legs,
    /// muzzle flashes and facings were lost, its hits cast no round and
    /// laid no trail, and the kit skipped it. In the squad mode the cursor
    /// sent a command almost every frame (`Cmd::Face`, `Cmd::Fire`), so
    /// most steps went unseen: the owner's playtest, 08:01Z, "the gun
    /// animation gets messed up sometimes". Runs before the next step
    /// begins, so the game is the same as when every step was seen.
    fn catch_up(&mut self) {
        let Some(g) = self.game.as_mut() else { return };
        if g.in_flight() || g.world.ticks() <= self.seen {
            return;
        }
        self.seen = g.world.ticks();
        kit_between(&mut self.kit, g, &self.controls.selected);
        self.rounds.after_step(g);
        feel(&self.rounds.pops, &mut self.felt, &mut self.trauma, &mut self.stop, &mut self.stop_cool, (self.args.shake, self.args.hitstop));
        shot_marks(g, &mut self.tracers, &mut self.looks, self.listener.as_mut());
        turret_fired(g, &mut self.blooks);
        step_notes(g, &mut self.blooks, &mut self.looks, &mut self.banked);
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
        // The gun cards' payloads (sound pass two, 2026-11-20).
        l.pops(&self.rounds.pops);
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
            // WASD pans only while paused (`Controls::wasd_pan`): set here
            // too, so a key held through P stops panning as play resumes.
            // (Also with no squad controlled, `SquadInput::free`.)
            self.controls.wasd_pan &= (self.paused || self.squad_free()) && self.game.as_ref().is_some_and(|g| g.end.is_none());
            self.controls.update(&mut self.camera, dt);
            self.squad_frame(dt);
        }
        self.step(dt);
        // The dish under the menu plays on: a round it ends starts again.
        // (Not the gun range's paused under its armory.)
        // A run planet's dish ends at its mission's verdict (`open_menu`).
        let over = self.game.as_ref().is_some_and(|g| g.end.is_some()) || self.mission.as_ref().is_some_and(|m| m.verdict.is_some());
        if self.in_menu() && !self.armory && over {
            if let Err(e) = self.open_menu() {
                return self.fail(event_loop, e);
            }
        }
        let ear = self.ear();
        self.hear(dt, ear);
        // The shake moves the camera for this frame's picture only: the
        // HUD is drawn on its own layer, and the cursor's aim reads the
        // still camera.
        let still = self.camera;
        let (sx, sy) = self.shake();
        (self.camera.x, self.camera.y) = (still.x + sx, still.y + sy);
        self.prepare();
        let camera = self.camera;
        (self.camera.x, self.camera.y) = (still.x, still.y);
        let (Some(p), Some(view)) = (&self.pixels, self.view.as_ref()) else { return };
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
        self.playlog.frame(t0.elapsed().as_secs_f32() * 1e3);
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
    /// Everything drawn this frame, with each soldier drawn between where
    /// its last step's walk began and ended, by how far the frame is
    /// through to the next step (`between`, `Game::drawn_between`): the
    /// game steps 15 times a second, and drawn at its steps a squad jumped
    /// every fourth frame at 60 while the camera glided, so every soldier
    /// shook on the screen (squad movement rework, 2026-11-16).
    fn prepare(&mut self) {
        let a = self.between();
        let own = self.game.as_mut().map(|g| g.drawn_between(a));
        self.prepare_now();
        if let (Some(g), Some(own)) = (self.game.as_mut(), own) {
            g.put_back(&own);
        }
    }

    fn prepare_now(&mut self) {
        self.refresh_ground();
        if let (Some(g), Some(v)) = (self.game.as_ref(), self.view.as_mut()) {
            v.set_time(g.time());
            v.set_alarm_side((g.world.e.done % 2) as u32);
            // The lure's field as rings on the ground (Look and
            // interface's `set_steer`, 2026-11-18), while the engine holds
            // one: the third quarter of the cells' region of `news`
            // (`Engine::write_lure_field`). Each frame, since a lure
            // stands and falls in play; 0 hides the rings. The world's
            // setup bound `news` (`set_dense`), which the view needs first.
            let e = &g.world.e;
            let b = slime_mould::mould::big::RAID_BLOCK;
            let nb = (e.dish.w.div_ceil(b) * e.dish.h.div_ceil(b)) as u32;
            let field = (e.dense_at / 4) as u32 + slime_mould::gpu::DENSE_HEAD as u32 + e.n + 2 * nb;
            v.set_steer(if e.lured { field } else { 0 });
            // How far it reaches, as `time_lure_paths` builds it, so no
            // ring is drawn on its rim (Look and interface, 2026-11-18).
            v.set_steer_reach(((g.rules.lure_reach.max(0.0) * g.world.scale()) as u32) as f32);
            // The mini-bosses' bodies (2026-11-16): each live or fading
            // one's lump, centre, look by its kind, strength by its grade,
            // look left by its fade and a leech's fill by its take.
            let looks = g.lump_looks();
            let mut lumps = [LumpBody::default(); slime_mould::world::view::MAX_LUMPS];
            for b in g.bosses.live.iter().chain(g.bosses.fading.iter().map(|f| &f.0)) {
                let code = looks.get(b.lump).copied().unwrap_or(0);
                let look = match (code & 7, self.args.boss_look) {
                    (0, _) => LumpLook::Plain,
                    (_, Some(l)) => l,
                    (k, None) => kind_look(k),
                };
                // Bit 8 fading, bits 9-15 how far through (0-127).
                let remain = if code & 1 << 8 != 0 { 1.0 - ((code >> 9) & 127) as f32 / 127.0 } else { 1.0 };
                let take = if b.kind == slime_mould::mould::larder_world::BossKind::Leech { b.stolen.max(0.0) } else { 0.0 };
                if let Some(l) = lumps.get_mut(b.lump) {
                    // The grade is bits 4-7 (the word is a u16 since the
                    // fade went in above it, bits 8-15).
                    // Bit 3: out on its errand (not gathering), so its
                    // members' own density carries its look.
                    let held = if code & 8 != 0 { 1.0 } else { 0.0 };
                    *l = LumpBody { at: b.centre, look, strength: ((code >> 4) & 15) as f32 / 15.0, remain, fill: take / (take + LEECH_HALF), held };
                }
            }
            // The fruit (`fruit.rs`, 2026-11-20): its own look, its
            // strength its ripeness, its members' density carrying it.
            if let Some((l, at, ripe)) = g.fruit_looks() {
                if let Some(b) = lumps.get_mut(l) {
                    *b = LumpBody { at, look: LumpLook::Fruit, strength: ripe, remain: 1.0, fill: 0.0, held: 1.0 };
                }
            }
            v.set_lumps(&lumps);
        }
        self.marks();
        // Always, zoomed out too, so the bottom bar never rearranges with the
        // zoom (owner, 11-14 20:19Z); M still hides it.
        let minimap = self.view.as_ref().is_some_and(|v| v.minimap);
        let hover = self.controls.cursor.filter(|&p| !self.hud.over(p)).and_then(|p| self.camera.world_at(p));
        let aim = self.controls.cursor.and_then(|p| self.hud.aim(p, &self.camera));
        let ghost = self.hud.targeting().and_then(|(id, _, from)| aim.map(|h| (id, h, from)));
        // A pump snaps to the crystal under the cursor (owner, 2026-11-15).
        let ghost = ghost.map(|(id, at, from)| match (id, self.game.as_ref()) {
            (PUMP, Some(g)) => (id, g.pump_crystal(at).map_or(at, |c| g.world.crystals()[c].0), from),
            _ => (id, at, from),
        });
        // Placing a building: a faint ring where its builders will stand and
        // raise it from (`BuildRules::reach`; 2026-11-14, the build-flow
        // thread), so a turret by the slime can be placed with that ring
        // on the near side of the ground the squad holds.
        if let (Some((id, at, _)), Some(g)) = (ghost, self.game.as_ref()) {
            if build_kind(id).is_some() {
                self.marks.ring(at, g.rules.build.reach * g.world.scale(), [235, 220, 180, 110]);
            }
        }
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
                v.minimap_off = true;
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
            v.minimap_off = false;
            v.set_marks(&self.marks);
            let cam = &self.camera;
            v.set_lights(&self.lights, (cam.x, cam.y, cam.x + cam.screen.0 / cam.zoom, cam.y + cam.screen.1 / cam.zoom));
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
    /// `viewpasses=N`: after the timed run, N more frames with each of the
    /// view's passes timed alone (`View::record_each`), as a table a pass.
    viewpasses: u32,
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let mut args = Args { map: Some("fork".into()), seed: 1, scale: 2, walkers: 5000, voice: true, ..Default::default() };
    let (mut look, mut hand, mut at, mut screen, mut wav) = (None, "rush".to_string(), vec![5.0, 30.0, 60.0], (1280u32, 800u32), None);
    // `runbot=SEED[:TO]`: the run bot plays run SEED from planet 1 to
    // planet TO (3) or a loss, with no window (`runbot`).
    let mut runbot: Option<(u64, u32)> = None;
    // `league=...` and `plan=PLAN`: the strategy league (`league.rs`).
    let (mut league_job, mut league_plan) = (None::<league::Job>, String::new());
    // `gunlab=DEPTH:SEED[:SECS]` with `deck=CARDS` (and `gun=KIND`): one
    // deck on one planet with the bot hand (`runbot::gun_lab`).
    let mut gunlab: Option<(u32, u64, f32)> = None;
    let mut deck = String::new();
    // `arena=N[:focus]`: the gun lab's arena, N soldiers and no economy
    // (`runbot::arena_hand`); `tail=T`: seconds more with fire held.
    let mut arena = 0usize;
    let mut focus = false;
    let mut guard = false;
    let mut tail = 0.0f32;
    let (mut bench_secs, mut fps, mut stamps, mut grid, mut fast, mut passes) = (None::<f32>, 60.0f32, true, 0usize, false, 0u32);
    let (mut settle, mut squad1, mut spread) = (false, false, true);
    let mut viewpasses = 0u32;
    let mut holes = false;
    let mut bank = None;
    let mut centre = None;
    let mut hover = None;
    let mut hover_to = None;
    let mut scroll = 0i32;
    let mut drags: Vec<(String, String)> = Vec::new();
    let mut select = true;
    let mut cut = None;
    let mut booms: Vec<(f32, f32, f32, Option<Vec<rounds::Card>>)> = Vec::new();
    let mut bosses: Vec<(f32, f32, f32)> = Vec::new();
    let mut turrets: Vec<(Kind, (f32, f32), f32)> = Vec::new();
    let mut renders: Vec<usize> = Vec::new();
    // Whether the command line picks the mould (`shape=`, `walkers=`); if
    // not, and no mission does, it is shape C (`DEFAULT_SHAPE`).
    let mut chosen = false;
    // `menu=1` opens the start menu with the command line's values preset,
    // `menu=0` never; with no arguments at all the window opens on it.
    // With `look=`, `menu=PAGE` films that page over the round.
    let mut menu_flag: Option<String> = None;
    // Whether the command line says `mode=`: if not, a window plays the
    // squad mode (owner 2026-11-15 07:57Z: "squad mode is default mode");
    // the no-window films and censuses (`look=`) and the bench keep the RTS round they
    // were written for unless they say `mode=squads`.
    let mut mode_given = false;
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
            "run" => {
                // `run=SEED` or `run=SEED:DEPTH` (a run landing on planet
                // DEPTH, to film a deep planet's level).
                let (seed, depth) = v.split_once(':').unwrap_or((v, "1"));
                let mut r = Run::new(seed.parse()?);
                r.depth = depth.parse::<u32>()?.clamp(1, slime_mould::mould::run::PLANETS);
                args.run = Some(r);
            }
            "range" => args.range = Some(Run::range(v.parse()?)),
            "dev" => args.dev = v != "0",
            "grow" => args.grow = Some(v.parse()?),
            "mode" => {
                mode_given = true;
                args.squads = match v {
                    "squads" => true,
                    "rts" => false,
                    _ => return Err("mode=squads|rts".into()),
                }
            }
            "crater" => args.crater = Some(v.parse()?),
            "scent" => args.scent = v != "0",
            "follow" => args.follow_boss = v == "boss",
            "boss_look" => args.boss_look = Some(LumpLook::parse(v).ok_or_else(|| format!("boss_look={v}: plain, ember, violet, ink or bone"))?),
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
            "shake" => args.shake = v.parse()?,
            "hitstop" => args.hitstop = v.parse()?,
            "real" => args.real = v != "0",
            "sink" => args.sink = Some(v.parse()?),
            // `spray=SECS[,SHARE]`: the spray's seconds (0 off) and the
            // share of each shot's kills that spray (`Rules::spray_share`).
            "spray" => {
                let w = v.split(',').map(|s| s.parse::<f32>()).collect::<Result<Vec<_>, _>>()?;
                args.spray = match w.as_slice() {
                    [s] => Some((*s, 1.0)),
                    [s, share] => Some((*s, *share)),
                    _ => return Err("spray=SECS or spray=SECS,SHARE".into()),
                };
            }
            "aim" => {
                let v: Vec<f32> = v.split(',').map(|s| s.parse::<f32>()).collect::<Result<_, _>>()?;
                args.aim = Some(v.try_into().map_err(|_| "aim=STRIKE,EVERY,SPREAD")?);
            }
            "gun" => {
                gun_of(v, 1, 1)?;
                args.gun = Some(v.to_string());
            }
            "ammo" => {
                if v.parse::<u32>().is_err() && rounds::parse(v).is_none() {
                    return Err(format!("ammo=: a number, or a round such as ammo/plain/slug/0/160/60/90/60/6+blast (got {v})").into());
                }
                args.ammo = Some(v.to_string());
            }
            // `version=1`: says which build this is and stops (CI checks
            // its Windows build starts with it; no window, no GPU).
            "version" => {
                println!("larder-world {}", slime_mould::build::build().long());
                return Ok(());
            }
            "look" => look = Some(v.to_string()),
            "league" => league_job = Some(league::parse(v)?),
            "plan" => league_plan = v.to_string(),
            "gunlab" => {
                let w: Vec<&str> = v.split(':').collect();
                let depth = w.first().ok_or("gunlab=DEPTH:SEED[:SECS]")?.parse::<u32>()?.clamp(1, slime_mould::mould::run::PLANETS);
                let seed = w.get(1).map_or(Ok(1), |s| s.parse::<u64>())?;
                let secs = w.get(2).map_or(Ok(0.0), |s| s.parse::<f32>())?;
                gunlab = Some((depth, seed, secs));
            }
            "deck" => deck = v.to_string(),
            "arena" => {
                let (n, how) = v.split_once(':').unwrap_or((v, ""));
                arena = n.parse()?;
                focus = how == "focus";
                guard = how == "guard";
            }
            "tail" => tail = v.parse()?,
            "runbot" => {
                let (seed, to) = v.split_once(':').unwrap_or((v, "3"));
                runbot = Some((seed.parse()?, to.parse::<u32>()?.clamp(1, slime_mould::mould::run::PLANETS)));
            }
            // `ui=150%` or `ui=1.5`: the UI's size this run (`uisize`).
            "ui" => ui = Some(settings::parse_size(v).ok_or("ui=PERCENT% (25% to 400%) or a factor such as 1.5")?),
            "menu" => {
                if v != "0" && page_named(v).is_none() {
                    return Err("menu=0|1|main|campaign|options|keys|news|shop (news-try: the things to try; options-planet, options-mould, options-experiments: that tab; shop-mid, shop-build, shop-tools: a run three planets in, a tab open; shop-turrets: its gun slots on the turrets' deck; shop@TAB@NAME: that tab, that row picked)".into());
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
            "viewpasses" => viewpasses = v.parse()?,
            "settle" => settle = v != "0",
            "squad1" => squad1 = v != "0",
            "spread" => spread = v != "0",
            "holes" => holes = v != "0",
            // `select=0`: the no-window pictures with nothing selected (the
            // HUD's collapsed selection panel).
            "select" => select = v != "0",
            "drag" => {
                for d in v.split('/') {
                    // `go`: the gun range's armory closes, and the film
                    // goes on in the round from the first frame whose
                    // `at=` time moves on.
                    if d == "go" {
                        drags.push(("go".into(), "go".into()));
                        continue;
                    }
                    let (a, b) = d.split_once('>').ok_or("drag=FROM>TO[/FROM>TO..][/go]: a slot (from 1) or a card's name, to a slot or bag; go leaves the range's armory")?;
                    drags.push((a.to_string(), b.to_string()));
                }
            }
            // `scroll=N`: with `menu=PAGE`, the page scrolls N wheel notches
            // down each frame after the first (films of a page taller than
            // the window, 11-15).
            "scroll" => scroll = v.parse::<i32>()?,
            "card" => args.card = v.split('/').map(String::from).collect(),
            "hover" => {
                // `hover=X,Y`: the no-window pictures' cursor, screen
                // pixels (a tooltip in the picture: the loadout's cards).
                // `hover=X,Y,X2,Y2` sweeps it over the frames (a throw's
                // aim leaving its reach, `hand=throw`).
                let n: Vec<f32> = v.split(',').map(|x| x.parse::<f32>()).collect::<Result<_, _>>()?;
                match n[..] {
                    [x, y] => hover = Some((x, y)),
                    [x, y, x2, y2] => (hover, hover_to) = (Some((x, y)), Some((x2, y2))),
                    _ => return Err("hover=X,Y[,X2,Y2]".into()),
                }
            }
            // `bank=N`: the no-window round starts with N food in the bank
            // (films of rounds, whose casts are paid from it).
            "bank" => bank = Some(v.parse::<f32>()?),
            // `set=NAME=V`: one of the game's rules by name
            // (`Rules::set`, such as `set=pump_smell=2`), kept
            // with the engine keys under `set:` so a menu round keeps it.
            "set" => {
                let (name, val) = v.split_once('=').ok_or("set=NAME=VALUE (a rule, such as set=pump_smell=2)")?;
                let x: f32 = val.parse().map_err(|e| format!("set={name}={val}: {e}"))?;
                if !Rules::default().set(name, x) && !RoundRules::default().set(name, x) {
                    return Err(format!("set={name}: no such rule").into());
                }
                args.keys.push((format!("{RULE_KEY}{name}"), val.to_string()));
            }
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
            // `turret2=X,Y` (11-15, the kill box films): a turret that
            // renders (level 2) from the start.
            // `mortar=X,Y[,T]` (2026-11-17, the mortar films): a mortar so.
            "turret" | "turret2" | "mortar" | "lure" | "pump" | "outpost" | "siren" => {
                // `turret=X,Y` (11-14, the turret films; the other kinds for
                // the building looks): one standing whole at (X, Y) from the
                // start, world units as `centre=` (paid for, not raised);
                // repeat for more. `turret=X,Y,T` (2026-11-16, the shove
                // card films): standing from game second T instead, so the
                // game before it is the same as without.
                let w = v.split(',').map(|s| s.parse::<f32>()).collect::<Result<Vec<_>, _>>()?;
                let (x, y, t) = match w.as_slice() {
                    [x, y] => (*x, *y, 0.0),
                    [x, y, t] => (*x, *y, *t),
                    _ => return Err(format!("{k}=X,Y[,T]").into()),
                };
                let kind = match k { "turret" | "turret2" => Kind::Turret, "mortar" => Kind::Mortar, "lure" => Kind::Lure, "pump" => Kind::Pump, "outpost" => Kind::Outpost, _ => Kind::Siren };
                turrets.push((kind, (x, y), t));
                if k == "turret2" {
                    renders.push(turrets.len() - 1);
                }
            }
            "wall" => {
                // `wall=X,Y,X2,Y2[,T]` (2026-11-16, the sapper films): a
                // wall standing whole from (X, Y) to (X2, Y2), world units
                // as `centre=`, from the start or from game second T.
                let w = v.split(',').map(|s| s.parse::<f32>()).collect::<Result<Vec<_>, _>>()?;
                let (x, y, to, t) = match w.as_slice() {
                    [x, y, x2, y2] => (*x, *y, (*x2, *y2), 0.0),
                    [x, y, x2, y2, t] => (*x, *y, (*x2, *y2), *t),
                    _ => return Err("wall=X,Y,X2,Y2[,T]".into()),
                };
                turrets.push((Kind::Wall { to }, (x, y), t));
            }
            "boss" => {
                // `boss=T,X,Y` (2026-11-16, the mini-boss films): at game
                // second T a mini-boss forms round (X, Y), world units as
                // `centre=` (`Game::form_boss`), whatever its rules' chances.
                let w = v.split(',').map(|s| s.parse::<f32>()).collect::<Result<Vec<_>, _>>()?;
                let [t, x, y] = w.as_slice() else { return Err("boss=T,X,Y".into()) };
                bosses.push((*t, *x, *y));
            }
            "load" => {
                let (t, name) = v.split_once(',').ok_or("load=T,NAME")?;
                let cards = match LOADOUTS.iter().find(|l| l.0 == name) {
                    Some(l) => l.1,
                    None if rounds::parse(name).is_some() => name,
                    None => return Err(format!("load=: a card list or one of {}", LOADOUTS.map(|l| l.0).join(", ")).into()),
                };
                args.load_at = Some((t.parse::<f32>()?, cards.to_string()));
            }
            // `try=NAME` or `try=NAME:CHOICE`: a declared switch by its
            // name (`build::switches`, the Experiments tab's rows), its
            // keys set as if typed.
            "try" => {
                let (_, choice) = slime_mould::build::switches::parse_try(v)?;
                args.keys.extend(slime_mould::build::switches::keys_of(choice));
            }
            "crustlook" => args.crust_look = Some(v.parse()?),
            "watch" => {
                let w = v.split(',').map(|s| s.parse::<f32>()).collect::<Result<Vec<_>, _>>()?;
                let [x, y, r] = w.as_slice() else { return Err("watch=X,Y,R".into()) };
                args.watch = Some((*x, *y, *r));
            }
            "tload" => {
                let (t, name) = v.split_once(',').ok_or("tload=T,NAME")?;
                let cards = match LOADOUTS.iter().find(|l| l.0 == name) {
                    Some(l) => l.1,
                    None if rounds::parse(name).is_some() => name,
                    None => return Err(format!("tload=: a card list or one of {}", LOADOUTS.map(|l| l.0).join(", ")).into()),
                };
                args.tload_at = Some((t.parse::<f32>()?, cards.to_string()));
            }
            "boom" => {
                // `boom=T,X,Y` (11-14, the explosion films): at game second
                // T the squad's round goes off at (X, Y), world units as
                // `centre=`, free (`Rounds::cast_at`); repeat for more.
                // `boom=T,X,Y,CARDS` (2026-11-16, the readers' films) casts
                // those cards instead (`Rounds::cast_cards_at`), so one
                // film can gas at 20 s and shatter at 23.
                let (w, cards) = match v.splitn(4, ',').collect::<Vec<_>>().as_slice() {
                    [t, x, y] => ([*t, *x, *y], None),
                    [t, x, y, c] => ([*t, *x, *y], Some(rounds::parse(c).ok_or("boom=T,X,Y,CARDS: a card list")?)),
                    _ => return Err("boom=T,X,Y[,CARDS]".into()),
                };
                let [t, x, y] = w.map(|s| s.parse::<f32>());
                booms.push((t?, x?, y?, cards));
            }
            "light" => {
                let w = v.split(',').map(|s| s.parse::<f32>()).collect::<Result<Vec<_>, _>>()?;
                let [t, x, y, r] = w.as_slice() else { return Err("light=T,X,Y,R".into()) };
                args.lights.push((*t, *x, *y, *r));
            }
            "lure_mark" => {
                let w = v.split(',').map(|s| s.parse::<f32>()).collect::<Result<Vec<_>, _>>()?;
                let [t, x, y, secs] = w.as_slice() else { return Err("lure_mark=T,X,Y,SECS".into()) };
                args.lure_marks.push((*t, *x, *y, *secs));
            }
            "trail" => {
                let p: Vec<&str> = v.split(',').collect();
                let (nums, rest) = p.split_at(5.min(p.len()));
                let n = nums.iter().map(|s| s.parse::<f32>()).collect::<Result<Vec<_>, _>>()?;
                let ([t, x1, y1, x2, y2], [cards, more @ ..]) = (n.as_slice(), rest) else { return Err("trail=T,X1,Y1,X2,Y2,CARDS[,lit]".into()) };
                let cards = rounds::parse(cards).ok_or("trail=: a card list")?;
                args.trails.push((*t, [*x1, *y1, *x2, *y2], cards, more.first() == Some(&"lit")));
            }
            "cross" => {
                let w = v.split(',').map(|s| s.parse::<f32>()).collect::<Result<Vec<_>, _>>()?;
                let [x1, y1, x2, y2, d] = w.as_slice() else { return Err("cross=X1,Y1,X2,Y2,D".into()) };
                args.cross = Some([*x1, *y1, *x2, *y2, *d]);
            }
            "fencepick" => args.fence_pick = Some(v.parse()?),
            "necks" => args.necks = v.split(',').map(|s| s.parse::<f32>()).collect::<Result<_, _>>()?,
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
                Err(e) => return Err(format!("{e} (or one of map, seed, scale, walkers, shape, voice, mission, crater, aim, core_pull, mass, wade, sink; look, hand, at, screen, sound, holes, ui, centre, cut, hover, select, turret, lure, pump, outpost, siren; bench, fps, stamps, grid, fast, passes, viewpasses, settle, squad1, spread)").into()),
            },
        }
    }
    // The own mould's garden (`own=1`, #165) keeps its own walkers, as a
    // mission keeps its mould: its two species start at twice `walkers`.
    // The pictures and the bench draw at `ui=` or 100%, never at a size
    // saved on this machine; the window opens at the saved size.
    set_ui_user(ui.unwrap_or_else(|| if look.is_some() || bench_secs.is_some() { 1.0 } else { settings::saved_ui() }));
    // The saved volume (`volume`); the pictures' recordings stay at 100%.
    if look.is_none() && bench_secs.is_none() {
        set_volume_user(settings::saved_volume());
    }
    if !mode_given && look.is_none() && bench_secs.is_none() {
        args.squads = true;
    }
    let base = args.clone();
    // A run keeps its tier's mould, as a mission does.
    if let Some(r) = args.run.as_mut() {
        r.dev = args.dev;
        r.restock();
        // The level's seed and walkers, as the menu's Run gives them
        // (`run_args`), so `run=SEED:DEPTH` films the level a run plays.
        let l = r.level();
        args.seed = l.seed;
        args.walkers = run_walkers(l.walkers);
    }
    if !chosen && args.mission.is_none() && !args.own && args.run.is_none() {
        default_shape(&mut args);
    }
    if let Some(secs) = bench_secs {
        println!("larder-world machine at the start: {}", machine());
        let (device, queue) = gpu().ok_or("no GPU adapter")?;
        bench(args, &Bench { secs, hand, fps, screen, stamps, grid, sound: wav, fast, passes, settle, squad1, spread, view: centre, viewpasses }, &device, &queue)?;
        println!("  machine at the end: {}", machine());
        return Ok(());
    }
    if let Some(job) = league_job {
        let (device, queue) = gpu().ok_or("no GPU adapter")?;
        match job {
            league::Job::Planet { depth, seed, secs } => league::planet(base, depth, seed, secs, &league_plan, &device, &queue)?,
            league::Job::Run { seed, to, secs } => league::whole_run(base, seed, to, secs, &league_plan, &device, &queue)?,
        }
        return Ok(());
    }
    if let Some((depth, seed, secs)) = gunlab {
        let (device, queue) = gpu().ok_or("no GPU adapter")?;
        let kind = base.gun.clone().unwrap_or_else(|| "rifle".into());
        runbot::gun_lab(base, (depth, seed, secs, arena), (&kind, &deck, focus, tail, guard), &device, &queue)?;
        return Ok(());
    }
    if let Some((seed, to)) = runbot {
        let (device, queue) = gpu().ok_or("no GPU adapter")?;
        runbot::run_bot(base, seed, to, &device, &queue)?;
        return Ok(());
    }
    if let Some(dir) = look {
        let page = menu_flag.as_deref().and_then(|v| Some((page_named(v)?, v)));
        return Ok(headless(args, &dir, &hand, &at, screen, wav.as_deref(), holes, centre, cut, bank, &booms, &bosses, &turrets, &renders, page, hover, hover_to, select, &drags, scroll)?);
    }
    let event_loop = EventLoop::new()?;
    event_loop.set_control_flow(ControlFlow::Poll);
    let mut app = App::new(args);
    app.memory = slime_mould::build::news::Memory::load();
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
#[cfg(test)]
mod film;

/// The alert for raid `d`: what the slime goes for.
fn raid_words<W: slime_mould::mould::larder_world::World>(g: &Game<W>, d: &slime_mould::mould::larder_world::Raid) -> String {
    use slime_mould::mould::larder_world::RaidAim;
    let name = |k: usize| g.buildings.get(k).map_or("building", |b| b.kind.name());
    match d.aim {
        _ if d.last_stand => "Last stand: the whole mould rushes your soldiers".into(),
        RaidAim::Building(k) => format!("The mould is raiding your {}", name(k)),
        RaidAim::Lure { from: Some(k), .. } => format!("A raid on your {} turns for the lure", name(k)),
        RaidAim::Lure { from: None, .. } => "A raid on your soldiers turns for the lure".into(),
        RaidAim::Soldiers(_) => "The mould is raiding your soldiers".into(),
    }
}
