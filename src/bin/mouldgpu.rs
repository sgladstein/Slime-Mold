//! **The GPU sandbox** (GPU plan phase 3, started 2026-11-11):
//! `cargo run --release --bin mouldgpu`
//!
//! The big dish's step (`slime_mould::mould::big`, the `nose` scaling rule)
//! on the GPU, drawn live, with every option the Jenson comparison tested
//! (Reports/jenson-compare-2026-11-10.md) as a control to play with. The
//! owner asked for it on 2026-11-11 00:06Z: "can we take all the different
//! things you tested for the jenson comparison as options that I can play
//! with in a sandbox". It is a toy for looking, not a game: no economy, no
//! census, no scoring.
//!
//! **Keys** (all on screen; H hides the panel):
//! - 1-8: the presets, each a dish from the comparison or the 4x Mould page
//!   (`PRESETS`). 9, 0 and Shift+1-5: the looks of Reports/jenson-looks-2026-11-11.md
//!   (trunk ring, contagion wave, sweeping ring, long curves, even polygons,
//!   endemic, sick ropes). Shift+6-0 and Ctrl+1-2: the outside rules'
//!   creatures. Ctrl+3 and 4: a fed and a starving mould (growth,
//!   Reports/growth-2026-11-11.md).
//! - Tab turns the panel's page: MAIN (the comparison's controls), LOOKS
//!   (start shape, the settings the trail sets, the deposit cap, the sensing
//!   floor, wander), CONTAGION (sickness, immunity, the germ field), OUTSIDE
//!   RULES and GROWTH (upkeep, births, starving, slots, the grass). With
//!   growth on the bottom line counts the living.
//! - Up / Down picks a control, Left / Right changes it (Shift: bigger steps).
//! - Space pauses, `.` steps one tick, R restarts, N restarts on a new seed.
//! - Left drag drops walkers at the mouse: each tick 0.2% of all walkers,
//!   taken from random places on the dish, jump into a disc under it, as
//!   respawned walkers jump (Jenson's 2020 performance spawned particles
//!   with a stylus). It reads as painting mould on; the walkers taken are
//!   spread too thin to see go. (Called "pulls" until the owner's playtest,
//!   2026-11-11 04:04Z, found it works as a deposit.)
//!   Right drag wipes the trail (and germ) under the mouse. Middle drag, or
//!   I once, makes the walkers under the mouse sick. F drops a food pile at
//!   the mouse. Esc quits.
//!
//! **Headless** (for checks and pictures): any `key=value` argument sets a
//! control, `preset=N` picks a preset first, and `look=DIR` runs without a
//! window for `seconds=` (default 10), then writes the last frame as a PNG,
//! the engine's measures (`BigMeasure`) and the frame cost. `gif=1` also
//! writes a GIF (a frame every `every=` frames, halved by `down=2`),
//! `frames=1` full-size PNG frames every `every=` frames for a video, and
//! `twice=1` runs it all again and says whether the pictures are the same
//! bit for bit; with contagion on it prints the share sick every half
//! second. Keys: `walkers= crowd=0|1 sensor_angle= turn= reach= step=
//! laid= keep= respawn= food=0|1 smell= colour=amber|grey field=trail|walkers
//! curve=game|soft knee= gamma= ticks= start=spread|blob|disc seed= scale=`,
//! the looks `disc= haze= trail_scale= reach_by=a:e angle_by=a:e turn_by=a:e
//! step_by=a:e cap= floor= wander=` and the contagion `sick= immune= catch=
//! germ= germ_keep= sick_speed= sick_trail= sick_reach= sick_turn= seed_at=
//! seed_r=` (the CPU's `BigParams` names work too).
//!
//! `MOULDGPU_SCREENSHOT_AFTER_FRAMES=N` saves the window's frame, panel and
//! all, as `mouldgpu-screenshot-pK.png` (K the preset) in the temp folder
//! after N frames, prints the frame cost and quits (the game has the same
//! hook).
//!
//! **The engine** (the shader, `Settings`, `Engine`) is the library's
//! `slime_mould::gpu` since 2026-11-11, so a game can use it; this file is
//! the sandbox's window, panel and presets, and the tests.
//!
//! **Tests on a GPU** (the twin test, phase 3's gate): one GPU tick from a
//! CPU dish's state against one `BigDish::tick`, the same seed twice, and
//! the crowd limit through jumps. They skip on a machine with no GPU unless
//! `MOULDGPU_TEST_GPU=1`, which CI sets with Mesa's software Vulkan.
//!
//! **One tick is the timing test's four compute passes** (`examples/gputime.rs`,
//! whose shader this one extends; that file stays as it was measured on the
//! GTX 1070): `sense` (a thread a walker: sensors, turn, aim, claim with
//! `atomicMin` of a keyed draw), `walk` (the winner moves and counts a
//! deposit), `trail` and `scent` (a thread a cell: 3x3 mean over agar,
//! kept). A frame is `ticks` ticks (2, the game's `STEPS_PER_FRAME`, by
//! default), then the drawing passes, on the GPU that `pixels` already
//! opens for the window, so no new dependency.
//!
//! **What the sandbox adds to the timing test's step:**
//! - **Jumps.** `respawn` moves that share of walkers a tick to a random spot
//!   with a random heading (`BigParams::respawn` on the CPU). The pull
//!   (left drag) moves 0.2% of walkers a tick to a disc at the mouse. As on
//!   the CPU, a jump comes after the walker's step (which lays its trail
//!   where it stepped), a jump off the agar is lost, and with the crowd
//!   limit a jump into a full cell is lost; two jumps into one empty cell go
//!   to the lower walker, the CPU's order. Two passes (`jump`, `land`), run
//!   only while respawn or the pull is on. (Until the independent review of
//!   2026-11-11 a jump replaced the step and joined its claim, so the jumper
//!   laid its trail where it landed: 1% respawn then spread trail over the
//!   dish and widened the tubes past every CPU seed, 8.33 against 7.70-8.01.)
//! - **The trail laid is set per 100,000 walkers** (`laid`): the deposit is
//!   `laid x 100,000 / walkers`, so changing the walker count keeps the trail
//!   over the dish the same, as the comparison's runs did.
//! - **Drawing** from the trail or from walkers a cell (`occ`), in the game's
//!   amber or grey on black, through the game's curve (1 - e^(-v/knee)) or
//!   a soft one ((v / p99.9)^gamma). The soft curve's p99.9 comes from a
//!   512-bin log histogram on the GPU each frame (8 bins an octave, so within
//!   9% of the true percentile), with no read back.
//! - **Fed scent on the GPU.** A pile lays `fed_scent` of its scent once tube
//!   lies on it, as on the CPU: the trail pass marks it (`reached`, one flag
//!   a pile) and the scent pass reads the flag, with no read back. (Until the
//!   review of 2026-11-11 the GPU had no fed scent, and every pile called at
//!   full strength for good: strays on the 32,000 dish were 3.6% at 10 s
//!   against 0.8-1.8% on the CPU's 16 seeds.)
//!
//! - **Jenson's looks** (`BigParams` of PR #45, Reports/jenson-looks-2026-11-11.md),
//!   all off by default, each the CPU's rule to the bit (the twin test
//!   `the_looks_match_the_cpu_for_one_tick`): a disc start with a haze
//!   around it; settings set by the trail a walker stands in (a setting is
//!   its base plus amp x s^power, s = trail x `trail_scale` capped at 1); the deposit
//!   cap (a cell's walkers lay deposit x sqrt(min(n, cap)) between them, so
//!   a crowd lays less each, after the move as on the CPU); the sensing
//!   floor (all three sensors below it: turn a random way) and wander (that
//!   share of walkers a tick take a random heading).
//! - **Contagion**: one counter a walker (`sick`, sick then immune) and a
//!   germ field that sick walkers lay where they stand, spread and kept like
//!   the trail. A healthy walker standing in more germ than `catch` falls
//!   sick; sick walkers can lay more trail, reach less, turn harder or move
//!   slower. Seeded at the dish's centre at `seed_at` seconds, or by hand.
//!   Germ is drawn in red over the trail, full at four times `catch` (the
//!   CPU's `style=sick`).
//! - **The outside rules** (`BigParams` of PR #55,
//!   Reports/outside-rules-2026-11-11.md), all off by default, each the
//!   CPU's rule (the twin test `the_outside_rules_match_the_cpu_for_one_tick`):
//!   species (up to four, each its own trail field and colour, smelling its
//!   own trail against the others', started in bands or mixed; cyclic
//!   species weigh the next and previous species apart); explorers (young
//!   walkers steer to the lowest reading and lay a share of the deposit,
//!   for a stage spread over the walkers; a jump starts it again); blocked
//!   walkers that lay trail where they stand and keep their heading; the
//!   chance turn, each walker's size, and area sensors. Blocked walkers'
//!   retry rounds (`blocked_retry`) are not ported: they need a pass a round.
//!
//! Every control is drawn straight from the simulation (the owner's rule);
//! the panel and the line at the bottom are the only things on screen that
//! are not.

use std::sync::Arc;
use std::time::Instant;

use pixels::{wgpu, Pixels, PixelsBuilder, ScalingMode, SurfaceTexture};
use slime_mould::gpu::*;
use slime_mould::hud;
use slime_mould::mould::STEPS_PER_SECOND;
use slime_mould::world::demo::Squad;
use slime_mould::world::ui::{theme, MenuItem};
use slime_mould::world::{look, Button, Camera, Canvas, CanvasPass, Controls, Hud, HudEvent, Marks, UiCommand, View};
use winit::application::ApplicationHandler;
use winit::dpi::PhysicalSize;
use winit::event::{ElementState, MouseButton, MouseScrollDelta, WindowEvent};
use winit::event_loop::{ActiveEventLoop, ControlFlow, EventLoop};
use winit::keyboard::{KeyCode, PhysicalKey};
use winit::window::{Window, WindowId};

/// The presets: keys 1-8 the dishes of the 4x Mould page and the Jenson
/// comparison; 9, 0 and Shift 1-5 the looks report's, Shift 6-0 the
/// outside rules report's, Ctrl 1-2 the creatures report's (2026-11-11).
const PRESETS: [&str; 24] = [
    "BIG DISH (PHASE 2)",
    "32,000 WALKERS",
    "NO LIMIT, OUR NUMBERS",
    "JENSON SETTINGS, 1M",
    "JENSON + RESPAWN 1%",
    "DRAWN BY WALKERS",
    "4M WALKERS",
    "4M, OUR NUMBERS",
    "TRUNK RING",
    "CONTAGION WAVE",
    "SWEEPING RING",
    "LONG CURVES",
    "EVEN POLYGONS",
    "ENDEMIC",
    "SICK ROPES",
    "TWO TERRITORIES",
    "CYCLIC RIBBONS",
    "EXPLORER FRONT",
    "DROPLETS",
    "BEADS ON STRINGS",
    "GRAZING DROPLETS",
    "GRAZING SPECIES",
    "FED MOULD",
    "STARVING MOULD",
];

/// The key a preset is on.
fn preset_key(k: usize) -> String {
    match k {
        0..=8 => (k + 1).to_string(),
        9 => "0".into(),
        10..=19 => format!("SHIFT+{}", (k - 9) % 10),
        _ => format!("CTRL+{}", (k - 19) % 10),
    }
}

fn preset(k: usize) -> Settings {
    let b = Settings::big_dish();
    // The comparison's dishes: no food, grey on black.
    let bare = Settings { food: false, colour: Colour::Grey, ..b.clone() };
    let jenson = Settings { walkers: 1_000_000, crowd: false, turn: 22.5, reach: 18.0, curve: Curve::Soft, ..bare.clone() };
    let respawn = Settings { respawn: 0.01, ..jenson.clone() };
    let trunk = Settings { start: Start::Disc, disc: 180.0, haze: 0.3, respawn: 0.001, trail_scale: 0.02, step_by: [1.0, 1.0], ..jenson.clone() };
    let sick = Settings { cap: 100, seed_at: 4.0, seed_r: 40.0, ..jenson.clone() };
    // The report's droplets: 30,000 walkers laying deposit 5 each (1.5 per
    // 100,000), SA 90, RA 45, SO 9, keep 0.93, a disc of 120 dish cells,
    // blocked walkers laying all of it; drawn by walkers a cell.
    let droplets = Settings {
        walkers: 30_000,
        crowd: true,
        laid: 1.5,
        sensor_angle: 90.0,
        turn: 45.0,
        reach: 9.0,
        keep: 0.93,
        start: Start::Disc,
        disc: 480.0,
        blocked_lay: 1.0,
        field: Field::Walkers,
        ..jenson.clone()
    };
    match k {
        0 => b,
        // The 4x Mould page's 32k dish laid today's deposit a walker.
        1 => Settings { walkers: 32_000, laid: b.laid * 0.32, ..b },
        // Ropes: 0 loops at 10 s against 151.5 with the limit.
        2 => Settings { crowd: false, ..bare },
        // Polygons: 43.5 loops at 10 s.
        3 => jenson,
        // A fine living mesh and a haze: 196 loops.
        4 => respawn,
        5 => Settings { field: Field::Walkers, ..respawn },
        6 => Settings { walkers: 4_000_000, ..jenson.clone() },
        7 => Settings { walkers: 4_000_000, crowd: false, curve: Curve::Soft, ..bare },
        // The looks report's dishes, its base the comparison's Jenson
        // settings at 1M. The trunk test: a dense disc (45 dish cells) with
        // 30% haze and slow respawn keeps a thick ring at 20 s (8 of 8 seeds).
        8 => trunk.clone(),
        // Contagion seeded in the middle at 4 s on the even polygons of the
        // square-root deposit: a red wave along the tubes in about 2 s.
        9 => Settings { sick: 120, immune: 480, catch_at: 0.5, sick_trail: 3.0, ..sick.clone() },
        // 36 Points: turning less in strong trail sweeps the first ring out
        // over the dish; narrower sensors there draw long curves.
        10 => Settings { step_by: [0.0, 1.0], turn_by: [-20.0, 1.0], ..trunk.clone() },
        11 => Settings { step_by: [0.0, 1.0], angle_by: [-30.0, 1.0], ..trunk.clone() },
        // The capped square-root deposit erases the start into even polygons.
        12 => Settings { trail_scale: 0.0, step_by: [0.0, 1.0], cap: 100, ..trunk },
        // Sick 0.5 s, immune 1 s: it never ends, 30-36% sick.
        13 => Settings { sick: 60, immune: 120, catch_at: 0.5, sick_trail: 3.0, ..sick.clone() },
        // Sick walkers sense half as far and turn twice as hard: the mesh
        // coarsens behind the wave (8 of 8 seeds at 9 s), then recovers.
        14 => Settings { sick: 240, immune: 6000, catch_at: 5.0, sick_reach: 0.5, sick_turn: 2.0, ..sick },
        // The outside rules report's dishes (`run.sh` and its commands),
        // on the comparison's Jenson settings at 1M unless said. Two
        // species started in halves hold two territories, avoiding each
        // other (border cells held 80%+ by one: 0.994-1.000, 8 of 8 seeds).
        15 => Settings { species: 2, zones: true, ..jenson.clone() },
        // Three cyclic species (own +1, next +0.6, previous -1) braid into
        // ribbons that keep flowing (97-125 loops at 10 s).
        16 => Settings { species: 3, cycle: true, sensor_angle: 20.0, turn: 20.0, reach: 32.0, step: 2.0, ..jenson.clone() },
        // Explorers from a small disc (20 dish cells), each for 0-2x 900
        // ticks: radial veins with a feathery front, best at about 2 s.
        17 => Settings { start: Start::Disc, disc: 80.0, explore: 900, explore_spread: 1.0, ..jenson },
        // Jones and Adamatzky's packed walkers, one a cell, laying trail
        // where they stand when blocked: about 180 round droplets (seed 1).
        18 => droplets,
        // The same with a longer reach: beads on strings.
        19 => Settings { reach: 27.0, ..droplets.clone() },
        // The creatures report's: grazing (weight 10) makes the droplets
        // crawl (centres moved 6.5-7.6 cells in 2 s against 0.67-0.77, 8 of
        // 8 seeds); two species of them, not avoiding each other, roam as
        // amber and teal cells (drawn by trail, in species colours).
        20 => Settings { graze: 10.0, ..droplets.clone() },
        21 => Settings { graze: 10.0, species: 2, avoid: 0.0, field: Field::Trail, ..droplets },
        // Growth (Larder World W0, `Reports/growth-2026-11-11.md`): the
        // census's dishes, 10,000 walkers in a blob in 100,000 slots,
        // upkeep 0.01, grazing 10; the grass regrows (fed) or does not
        // (starving). Fed fills the dish with about 96,000 by 30 s; starving
        // spreads as far, hollows out and is nearly gone by 30 s. The trail
        // laid is per 100,000 walkers at the start, so 0.5 here is the
        // census's deposit of 5 a walker (the default 5 would be 50 a
        // walker: thick ropes, not a mesh).
        22 => Settings { walkers: 10_000, capacity: 100_000, laid: 0.5, upkeep: 0.01, graze: 10.0, graze_regrow: 0.002, start: Start::Home, ..b.clone() },
        23 => Settings { walkers: 10_000, capacity: 100_000, laid: 0.5, upkeep: 0.01, graze: 10.0, graze_regrow: 0.0, start: Start::Home, ..b },
        _ => panic!("preset {k}: 0-23"),
    }
}

#[derive(Clone, Copy, PartialEq, Debug)]
enum Knob {
    Walkers,
    Crowd,
    SensorAngle,
    Turn,
    Reach,
    Step,
    Laid,
    Keep,
    Respawn,
    Food,
    Smell,
    Colour,
    Field,
    Curve,
    Knee,
    Gamma,
    Ticks,
    Start,
    // The looks page.
    Disc,
    Haze,
    TrailScale,
    ReachBy,
    AngleBy,
    TurnBy,
    StepBy,
    TrailPower,
    Cap,
    Floor,
    Wander,
    // The contagion page.
    Sick,
    Immune,
    Catch,
    Germ,
    GermKeep,
    SickSpeed,
    SickTrail,
    SickReach,
    SickTurn,
    SeedAt,
    SeedR,
    // The outside rules page.
    Species,
    Avoid,
    Zones,
    Cycle,
    Next,
    Prev,
    Explore,
    ExplorerTrail,
    ExploreSpread,
    BlockedLay,
    BlockedKeep,
    BlockedReset,
    Chance,
    SizeSpread,
    SensorR,
    Convert,
    Boost,
    Graze,
    GrazeEat,
    GrazeRegrow,
    // The growth page.
    Upkeep,
    StartEnergy,
    BirthEnergy,
    BirthCost,
    Starve,
    Slots,
}

const LOOKS: [Knob; 11] = [Knob::Disc, Knob::Haze, Knob::TrailScale, Knob::ReachBy, Knob::AngleBy, Knob::TurnBy, Knob::StepBy, Knob::TrailPower, Knob::Cap, Knob::Floor, Knob::Wander];
const SICK: [Knob; 11] = [Knob::Sick, Knob::Immune, Knob::Catch, Knob::Germ, Knob::GermKeep, Knob::SickSpeed, Knob::SickTrail, Knob::SickReach, Knob::SickTurn, Knob::SeedAt, Knob::SeedR];

const OUTSIDE: [Knob; 20] = [
    Knob::Species,
    Knob::Avoid,
    Knob::Zones,
    Knob::Cycle,
    Knob::Next,
    Knob::Prev,
    Knob::Explore,
    Knob::ExplorerTrail,
    Knob::ExploreSpread,
    Knob::BlockedLay,
    Knob::BlockedKeep,
    Knob::BlockedReset,
    Knob::Chance,
    Knob::SizeSpread,
    Knob::SensorR,
    Knob::Convert,
    Knob::Boost,
    Knob::Graze,
    Knob::GrazeEat,
    Knob::GrazeRegrow,
];

/// Growth and starving (Larder World W0): the walkers eat the grass, so
/// grazing's three controls are here too.
const GROWTH: [Knob; 9] = [Knob::Upkeep, Knob::StartEnergy, Knob::BirthEnergy, Knob::BirthCost, Knob::Starve, Knob::Slots, Knob::GrazeRegrow, Knob::GrazeEat, Knob::Graze];

/// The panel's pages (Tab turns them): the first holds the comparison's
/// controls, the others the looks report's, the outside rules report's
/// and growth's (2026-11-11).
const PAGES: [(&str, &[Knob]); 5] = [("MAIN", &KNOBS), ("LOOKS", &LOOKS), ("CONTAGION", &SICK), ("OUTSIDE RULES", &OUTSIDE), ("GROWTH", &GROWTH)];

const KNOBS: [Knob; 18] = [
    Knob::Walkers,
    Knob::Crowd,
    Knob::SensorAngle,
    Knob::Turn,
    Knob::Reach,
    Knob::Step,
    Knob::Laid,
    Knob::Keep,
    Knob::Respawn,
    Knob::Food,
    Knob::Smell,
    Knob::Colour,
    Knob::Field,
    Knob::Curve,
    Knob::Knee,
    Knob::Gamma,
    Knob::Ticks,
    Knob::Start,
];

const WALKER_STEPS: [usize; 10] = [10_000, 32_000, 100_000, 200_000, 500_000, 1_000_000, 2_000_000, 4_000_000, 6_000_000, 8_000_000];
const RESPAWN_STEPS: [f32; 9] = [0.0, 0.0001, 0.0003, 0.001, 0.003, 0.01, 0.03, 0.1, 0.3];
const SCALE_STEPS: [f32; 9] = [0.0, 0.005, 0.01, 0.02, 0.05, 0.1, 0.2, 0.5, 1.0];
const CAP_STEPS: [u16; 10] = [0, 2, 5, 10, 20, 50, 100, 200, 500, 1000];
const FLOOR_STEPS: [f32; 13] = [0.0, 0.1, 0.25, 0.5, 1.0, 2.0, 3.0, 3.5, 4.0, 5.0, 8.0, 12.0, 20.0];
const WANDER_STEPS: [f32; 7] = [0.0, 0.001, 0.003, 0.01, 0.03, 0.1, 0.3];
/// Ticks sick or immune (120 a second); the sum stays inside the CPU's u16.
const SICK_STEPS: [u32; 10] = [0, 30, 60, 120, 240, 480, 1200, 2400, 6000, 12000];
/// Ticks an explorer explores (the report's front: 900).
const EXPLORE_STEPS: [u16; 9] = [0, 30, 60, 120, 240, 480, 900, 1800, 3600];
const RESET_STEPS: [f32; 8] = [0.0, 0.001, 0.003, 0.01, 0.03, 0.1, 0.3, 1.0];
/// The chance turn's power (the report's test: 4).
const CHANCE_STEPS: [f32; 8] = [0.0, 0.5, 1.0, 1.5, 2.0, 4.0, 8.0, 16.0];
/// Conversion's factor (the creatures report tried 1.2-2) and grazing's
/// weight (10 crawls; 30 loses walkers; 100 a gas).
const CONVERT_STEPS: [f32; 8] = [0.0, 1.0, 1.1, 1.2, 1.5, 2.0, 3.0, 5.0];
/// Growth's upkeep a tick (the census: 0.01 against at most 0.05 eaten)
/// and the starving's chance of dying a tick.
const UPKEEP_STEPS: [f32; 9] = [0.0, 0.001, 0.003, 0.005, 0.01, 0.015, 0.02, 0.03, 0.05];
const STARVE_STEPS: [f32; 8] = [0.001, 0.003, 0.01, 0.03, 0.1, 0.3, 0.5, 1.0];
const GRAZE_STEPS: [f32; 9] = [0.0, 1.0, 3.0, 5.0, 10.0, 20.0, 30.0, 50.0, 100.0];

/// The entry of `list` after (or before) the last one at or below `v`.
fn next_in<T: PartialOrd + Copy>(list: &[T], v: T, dir: i32) -> T {
    // Below the list (a value from the command line): up is its first.
    let Some(i) = list.iter().rposition(|&x| x <= v) else {
        return if dir > 0 { list[0] } else { v };
    };
    let i = i as i32;
    let i = if dir > 0 || list[i as usize] == v { i + dir } else { i };
    list[i.clamp(0, list.len() as i32 - 1) as usize]
}

fn thousands(n: usize) -> String {
    let s = n.to_string();
    let mut out = String::new();
    for (i, c) in s.chars().enumerate() {
        if i > 0 && (s.len() - i).is_multiple_of(3) {
            out.push(',');
        }
        out.push(c);
    }
    out
}

/// Ticks as seconds, 120 a second.
fn secs(ticks: u32) -> String {
    if ticks == 0 {
        return "OFF".into();
    }
    format!("{} S", (ticks as f64 / STEPS_PER_SECOND as f64 * 100.0).round() / 100.0)
}

fn percent(v: f32) -> String {
    if v == 0.0 {
        return "OFF".into();
    }
    // To two places, so 0.3 reads 30%, not 30.000002%.
    format!("{}%", (v as f64 * 10_000.0).round() / 100.0)
}

impl Knob {
    fn label(self) -> &'static str {
        match self {
            Knob::Walkers => "WALKERS",
            Knob::Crowd => "ONE WALKER A CELL",
            Knob::SensorAngle => "SENSOR ANGLE",
            Knob::Turn => "TURN",
            Knob::Reach => "SENSOR REACH",
            Knob::Step => "STEP",
            Knob::Laid => "TRAIL LAID /100K",
            Knob::Keep => "TRAIL KEPT A TICK",
            Knob::Respawn => "RESPAWN A TICK",
            Knob::Food => "DISH FOOD",
            Knob::Smell => "FOOD SMELLED AT",
            Knob::Colour => "COLOUR",
            Knob::Field => "DRAWN FROM",
            Knob::Curve => "BRIGHTNESS CURVE",
            Knob::Knee => "GAME CURVE KNEE",
            Knob::Gamma => "SOFT CURVE POWER",
            Knob::Ticks => "TICKS A FRAME",
            Knob::Start => "START",
            Knob::Disc => "DISC RADIUS",
            Knob::Haze => "HAZE AT START",
            Knob::TrailScale => "TRAIL SCALE",
            Knob::ReachBy => "REACH BY TRAIL",
            Knob::AngleBy => "ANGLE BY TRAIL",
            Knob::TurnBy => "TURN BY TRAIL",
            Knob::StepBy => "STEP BY TRAIL",
            Knob::TrailPower => "TRAIL POWER",
            Knob::Cap => "SQRT DEPOSIT CAP",
            Knob::Floor => "SENSING FLOOR",
            Knob::Wander => "WANDER A TICK",
            Knob::Sick => "SICK FOR",
            Knob::Immune => "IMMUNE FOR",
            Knob::Catch => "CATCHES AT GERM",
            Knob::Germ => "GERM LAID",
            Knob::GermKeep => "GERM KEPT A TICK",
            Knob::SickSpeed => "SICK SPEED",
            Knob::SickTrail => "SICK TRAIL",
            Knob::SickReach => "SICK REACH",
            Knob::SickTurn => "SICK TURN",
            Knob::SeedAt => "SEEDED AT",
            Knob::SeedR => "SEED RADIUS",
            Knob::Species => "SPECIES",
            Knob::Avoid => "AVOID OTHERS",
            Knob::Zones => "START IN BANDS",
            Knob::Cycle => "CYCLIC SPECIES",
            Knob::Next => "NEXT SPECIES",
            Knob::Prev => "PREVIOUS SPECIES",
            Knob::Explore => "EXPLORE FOR",
            Knob::ExplorerTrail => "EXPLORER TRAIL",
            Knob::ExploreSpread => "EXPLORE SPREAD",
            Knob::BlockedLay => "BLOCKED LAY",
            Knob::BlockedKeep => "BLOCKED KEEP AIM",
            Knob::BlockedReset => "BLOCKED RESET",
            Knob::Chance => "CHANCE TURN POWER",
            Knob::SizeSpread => "SIZE SPREAD",
            Knob::SensorR => "AREA SENSOR RADIUS",
            Knob::Convert => "CONVERT AT",
            Knob::Boost => "SPECIES 0 LAYS",
            Knob::Graze => "GRAZING",
            Knob::GrazeEat => "GRASS EATEN A TICK",
            Knob::GrazeRegrow => "GRASS REGROWS",
            Knob::Upkeep => "UPKEEP A TICK",
            Knob::StartEnergy => "START ENERGY",
            Knob::BirthEnergy => "DIVIDES AT",
            Knob::BirthCost => "BIRTH COST",
            Knob::Starve => "STARVING DIES",
            Knob::Slots => "SLOTS",
        }
    }

    fn value(self, s: &Settings, crowd_ok: bool) -> String {
        let on = |b: bool| if b { "ON" } else { "OFF" }.to_string();
        match self {
            Knob::Walkers => thousands(s.walkers),
            Knob::Crowd if s.crowd && !crowd_ok => "OFF (TOO MANY)".into(),
            Knob::Crowd => on(s.crowd),
            Knob::SensorAngle => format!("{:.1}", s.sensor_angle),
            Knob::Turn => format!("{:.1}", s.turn),
            Knob::Reach => format!("{:.0}", s.reach),
            Knob::Step => format!("{:.2}", s.step),
            Knob::Laid => format!("{:.2}", s.laid),
            Knob::Keep => format!("{:.3}", s.keep),
            Knob::Respawn => percent(s.respawn),
            Knob::Food => on(s.food),
            Knob::Smell => format!("{:.0}", s.smell),
            Knob::Colour => if s.colour == Colour::Amber { "AMBER" } else { "GREY" }.into(),
            Knob::Field => if s.field == Field::Trail { "TRAIL" } else { "WALKERS A CELL" }.into(),
            Knob::Curve => if s.curve == Curve::Game { "GAME" } else { "SOFT" }.into(),
            Knob::Knee => format!("{:.1}", s.knee),
            Knob::Gamma => format!("{:.2}", s.gamma),
            Knob::Ticks => s.ticks.to_string(),
            Knob::Start => match s.start {
                Start::Spread => "SPREAD",
                Start::Home => "BLOB AT HOME",
                Start::Disc => "DISC IN THE MIDDLE",
            }
            .into(),
            Knob::Disc => format!("{:.0}", s.disc),
            Knob::Haze => percent(s.haze),
            Knob::TrailScale if s.trail_scale == 0.0 => "OFF".into(),
            Knob::TrailScale => format!("{} (FULL AT {})", s.trail_scale, (1.0 / s.trail_scale).round()),
            Knob::ReachBy => format!("{:+.0}", s.reach_by[0]),
            Knob::AngleBy => format!("{:+.1}", s.angle_by[0]),
            Knob::TurnBy => format!("{:+.1}", s.turn_by[0]),
            Knob::StepBy => format!("{:+.2}", s.step_by[0]),
            Knob::TrailPower => {
                let e = [s.reach_by[1], s.angle_by[1], s.turn_by[1], s.step_by[1]];
                if e.iter().all(|&x| x == e[0]) {
                    format!("{:.2}", e[0])
                } else {
                    "MIXED".into()
                }
            }
            Knob::Cap if s.cap == 0 => "OFF".into(),
            Knob::Cap => s.cap.to_string(),
            Knob::Floor if s.floor == 0.0 => "OFF".into(),
            Knob::Floor => format!("{}", s.floor),
            Knob::Wander => percent(s.wander),
            Knob::Sick => secs(s.sick),
            Knob::Immune if s.immune == 0 => "0 S".into(),
            Knob::Immune => secs(s.immune),
            Knob::Catch => format!("{:.2}", s.catch_at),
            Knob::Germ => format!("{:.2}", s.germ),
            Knob::GermKeep => format!("{:.3}", s.germ_keep),
            Knob::SickSpeed => format!("{:.1}X", s.sick_speed),
            Knob::SickTrail => format!("{:.1}X", s.sick_trail),
            Knob::SickReach => format!("{:.1}X", s.sick_reach),
            Knob::SickTurn => format!("{:.1}X", s.sick_turn),
            Knob::SeedAt if s.seed_at <= 0.0 => "THE START".into(),
            Knob::SeedAt => format!("{:.0} S", s.seed_at),
            Knob::SeedR => format!("{:.0}", s.seed_r),
            Knob::Species if s.fields() == 0 => "OFF".into(),
            Knob::Species => s.fields().to_string(),
            Knob::Avoid => format!("{:.2}", s.avoid),
            Knob::Zones => on(s.zones),
            Knob::Cycle if s.cycle && s.fields() < 3 => "ON (NEEDS 3)".into(),
            Knob::Cycle => on(s.cycle),
            Knob::Next => format!("{:+.1}", s.next),
            Knob::Prev => format!("{:+.1}", s.prev),
            Knob::Explore => secs(s.explore as u32),
            Knob::ExplorerTrail => format!("{:.2}X", s.explorer_trail),
            Knob::ExploreSpread if s.explore_spread == 0.0 => "OFF".into(),
            Knob::ExploreSpread => format!("0-{:.1}X", 1.0 + s.explore_spread),
            Knob::BlockedLay if s.blocked_lay == 0.0 => "OFF".into(),
            Knob::BlockedLay if s.cap > 0 => "OFF (WITH THE CAP)".into(),
            Knob::BlockedLay => format!("{:.2}X", s.blocked_lay),
            Knob::BlockedKeep => on(s.blocked_keep),
            Knob::BlockedReset => percent(s.blocked_reset),
            Knob::Chance if s.chance == 0.0 => "OFF".into(),
            Knob::Chance => format!("{}", s.chance),
            Knob::SizeSpread if s.size_spread == 0.0 => "OFF".into(),
            Knob::SizeSpread => format!("{:.1}", s.size_spread),
            Knob::SensorR if s.sensor_r == 0 => "ONE CELL".into(),
            Knob::SensorR => format!("{} ({} CELLS)", s.sensor_r, (2 * s.sensor_r as u32 + 1).pow(2)),
            Knob::Convert if s.convert == 0.0 => "OFF".into(),
            Knob::Convert => format!("{:.1}X OWN", s.convert),
            Knob::Boost => format!("{:.2}X", s.boost),
            Knob::Graze if s.graze == 0.0 => "OFF".into(),
            Knob::Graze => format!("{}", s.graze),
            Knob::GrazeEat => format!("{:.3}", s.graze_eat),
            Knob::GrazeRegrow if s.graze_regrow == 0.0 => "OFF".into(),
            Knob::GrazeRegrow => format!("{:.4}", s.graze_regrow),
            Knob::Upkeep if s.upkeep == 0.0 => "OFF (FIXED COUNT)".into(),
            Knob::Upkeep => format!("{:.3}", s.upkeep),
            Knob::StartEnergy => format!("{:.2}", s.start_energy),
            Knob::BirthEnergy => format!("{:.2}", s.birth_energy),
            Knob::BirthCost => format!("{:.2}", s.birth_cost),
            Knob::Starve => format!("{} A TICK", percent(s.starve)),
            Knob::Slots if s.capacity <= s.walkers => "THE WALKERS".into(),
            Knob::Slots => thousands(s.capacity),
        }
    }

    /// Changes `s` one step in `dir` (`big`: a bigger step). Returns true if
    /// the dish must restart for it to take effect.
    fn nudge(self, s: &mut Settings, dir: i32, big: bool, max_walkers: usize) -> bool {
        let d = dir as f32;
        let k = if big { 4.0 } else { 1.0 };
        let mul = |v: f32, f: f32| if dir > 0 { v * f } else { v / f };
        match self {
            Knob::Walkers => {
                let before = s.walkers;
                s.walkers = next_in(&WALKER_STEPS, s.walkers, dir).min(max_walkers);
                return s.walkers != before;
            }
            // A restart: switched on mid-run, walkers stacked by a dish with
            // no limit could never leave a full cell (the review, 2026-11-11,
            // found 3,906 cells still over one 720 ticks later).
            Knob::Crowd => {
                s.crowd = !s.crowd;
                return true;
            }
            Knob::SensorAngle => s.sensor_angle = (s.sensor_angle + 2.5 * k * d).clamp(0.0, 180.0),
            Knob::Turn => s.turn = (s.turn + 2.5 * k * d).clamp(0.0, 180.0),
            Knob::Reach => s.reach = (s.reach + k * d).clamp(1.0, 120.0),
            Knob::Step => s.step = (s.step + 0.25 * k * d).clamp(0.25, 8.0),
            Knob::Laid => s.laid = mul(s.laid, if big { 2.0 } else { 1.25 }).clamp(0.01, 10_000.0),
            Knob::Keep => s.keep = (s.keep + 0.01 * k * d).clamp(0.0, 0.999),
            Knob::Respawn => s.respawn = next_in(&RESPAWN_STEPS, s.respawn, dir),
            Knob::Food => s.food = !s.food,
            Knob::Smell => s.smell = (s.smell + 2.0 * k * d).clamp(1.0, 200.0),
            Knob::Colour => s.colour = if s.colour == Colour::Amber { Colour::Grey } else { Colour::Amber },
            Knob::Field => s.field = if s.field == Field::Trail { Field::Walkers } else { Field::Trail },
            Knob::Curve => s.curve = if s.curve == Curve::Game { Curve::Soft } else { Curve::Game },
            Knob::Knee => s.knee = mul(s.knee, if big { 2.0 } else { 1.25 }).clamp(0.05, 10_000.0),
            Knob::Gamma => s.gamma = (s.gamma + 0.05 * k * d).clamp(0.05, 3.0),
            Knob::Ticks => s.ticks = (s.ticks as i32 + dir * if big { 4 } else { 1 }).clamp(1, 32) as u32,
            Knob::Start => {
                let all = [Start::Spread, Start::Home, Start::Disc];
                let i = all.iter().position(|&x| x == s.start).unwrap() as i32;
                s.start = all[(i + dir).rem_euclid(3) as usize];
                return true;
            }
            // The start's own settings restart the dish only when used.
            Knob::Disc => {
                s.disc = (s.disc + 10.0 * k * d).clamp(10.0, 480.0);
                return s.start == Start::Disc;
            }
            Knob::Haze => {
                s.haze = (s.haze + 0.05 * k * d).clamp(0.0, 1.0);
                return s.start != Start::Spread;
            }
            Knob::TrailScale => s.trail_scale = next_in(&SCALE_STEPS, s.trail_scale, dir),
            Knob::ReachBy => s.reach_by[0] = (s.reach_by[0] + k * d).clamp(-60.0, 60.0),
            Knob::AngleBy => s.angle_by[0] = (s.angle_by[0] + 2.5 * k * d).clamp(-90.0, 90.0),
            Knob::TurnBy => s.turn_by[0] = (s.turn_by[0] + 2.5 * k * d).clamp(-90.0, 90.0),
            Knob::StepBy => s.step_by[0] = (s.step_by[0] + 0.25 * k * d).clamp(-4.0, 8.0),
            // One power for all four (the command line sets each).
            Knob::TrailPower => {
                let e = (s.reach_by[1] + 0.25 * k * d).clamp(0.0, 4.0);
                for m in [&mut s.reach_by, &mut s.angle_by, &mut s.turn_by, &mut s.step_by] {
                    m[1] = e;
                }
            }
            Knob::Cap => s.cap = next_in(&CAP_STEPS, s.cap, dir),
            Knob::Floor => s.floor = next_in(&FLOOR_STEPS, s.floor, dir),
            Knob::Wander => s.wander = next_in(&WANDER_STEPS, s.wander, dir),
            Knob::Sick => s.sick = next_in(&SICK_STEPS, s.sick, dir),
            Knob::Immune => s.immune = next_in(&SICK_STEPS, s.immune, dir),
            Knob::Catch => s.catch_at = mul(s.catch_at, if big { 2.0 } else { 1.25 }).clamp(0.01, 1000.0),
            Knob::Germ => s.germ = mul(s.germ, if big { 2.0 } else { 1.25 }).clamp(0.01, 100.0),
            Knob::GermKeep => s.germ_keep = (s.germ_keep + 0.01 * k * d).clamp(0.0, 0.999),
            Knob::SickSpeed => s.sick_speed = (s.sick_speed + 0.1 * k * d).clamp(0.0, 5.0),
            Knob::SickTrail => s.sick_trail = (s.sick_trail + 0.1 * k * d).clamp(0.0, 10.0),
            Knob::SickReach => s.sick_reach = (s.sick_reach + 0.1 * k * d).clamp(0.0, 5.0),
            Knob::SickTurn => s.sick_turn = (s.sick_turn + 0.1 * k * d).clamp(0.0, 5.0),
            // Seeding at the start takes a restart; a later time does not.
            Knob::SeedAt => {
                s.seed_at = (s.seed_at + k * d).clamp(0.0, 60.0);
                return s.seed_at == 0.0 && s.sick_on();
            }
            Knob::SeedR => {
                s.seed_r = (s.seed_r + 4.0 * k * d).clamp(4.0, 320.0);
                return s.seed_at == 0.0 && s.sick_on();
            }
            // Species and their bands take a restart (a field each, and
            // where they start).
            Knob::Species => {
                let before = s.fields();
                s.species = (s.fields() as i32 + dir).clamp(0, MAX_SPECIES as i32) as u8;
                if s.species == 1 {
                    s.species = if dir > 0 { 2 } else { 0 };
                }
                return s.fields() != before;
            }
            Knob::Avoid => s.avoid = (s.avoid + 0.25 * k * d).clamp(-2.0, 4.0),
            Knob::Zones => {
                s.zones = !s.zones;
                return s.fields() > 0 && s.start == Start::Spread;
            }
            Knob::Cycle => s.cycle = !s.cycle,
            Knob::Next => s.next = (s.next + 0.1 * k * d).clamp(-3.0, 3.0),
            Knob::Prev => s.prev = (s.prev + 0.1 * k * d).clamp(-3.0, 3.0),
            Knob::Explore => s.explore = next_in(&EXPLORE_STEPS, s.explore, dir),
            Knob::ExplorerTrail => s.explorer_trail = (s.explorer_trail + 0.05 * k * d).clamp(0.0, 2.0),
            Knob::ExploreSpread => s.explore_spread = (s.explore_spread + 0.1 * k * d).clamp(0.0, 1.0),
            Knob::BlockedLay => s.blocked_lay = (s.blocked_lay + 0.25 * k * d).clamp(0.0, 4.0),
            Knob::BlockedKeep => s.blocked_keep = !s.blocked_keep,
            Knob::BlockedReset => s.blocked_reset = next_in(&RESET_STEPS, s.blocked_reset, dir),
            Knob::Chance => s.chance = next_in(&CHANCE_STEPS, s.chance, dir),
            Knob::SizeSpread => s.size_spread = (s.size_spread + 0.1 * k * d).clamp(0.0, 2.0),
            Knob::SensorR => s.sensor_r = (s.sensor_r as i32 + dir).clamp(0, 3) as u8,
            Knob::Convert => s.convert = next_in(&CONVERT_STEPS, s.convert, dir),
            Knob::Boost => s.boost = (s.boost + 0.1 * k * d).clamp(0.1, 4.0),
            Knob::Graze => s.graze = next_in(&GRAZE_STEPS, s.graze, dir),
            Knob::GrazeEat => s.graze_eat = mul(s.graze_eat, if big { 2.0 } else { 1.25 }).clamp(0.001, 1.0),
            // Down past 0.0001 is off: the grass never comes back.
            Knob::GrazeRegrow if s.graze_regrow == 0.0 => s.graze_regrow = if dir > 0 { 0.0001 } else { 0.0 },
            Knob::GrazeRegrow if dir < 0 && s.graze_regrow <= 0.0001 => s.graze_regrow = 0.0,
            Knob::GrazeRegrow => s.graze_regrow = mul(s.graze_regrow, if big { 2.0 } else { 1.25 }).clamp(0.0001, 1.0),
            // Turning growth on or off restarts the dish (the slots and
            // their energy are made at the start); changing it does not.
            Knob::Upkeep => {
                let was = s.upkeep > 0.0;
                s.upkeep = next_in(&UPKEEP_STEPS, s.upkeep, dir);
                return (s.upkeep > 0.0) != was;
            }
            Knob::StartEnergy => {
                s.start_energy = (s.start_energy + 0.05 * k * d).clamp(0.05, 2.0);
                return s.upkeep > 0.0;
            }
            Knob::BirthEnergy => s.birth_energy = (s.birth_energy + 0.05 * k * d).clamp(0.1, 4.0),
            Knob::BirthCost => s.birth_cost = (s.birth_cost + 0.05 * k * d).clamp(0.0, 1.0),
            Knob::Starve => s.starve = next_in(&STARVE_STEPS, s.starve, dir),
            Knob::Slots => {
                let before = s.capacity.max(s.walkers);
                s.capacity = next_in(&WALKER_STEPS, before, dir).min(max_walkers).max(s.walkers);
                return s.upkeep > 0.0 && s.capacity != before;
            }
        }
        false
    }
}

/// The panel: premultiplied RGBA over the dish, dish-sized.
fn panel(w: usize, h: usize, lines: &[(String, [u8; 4])], bottom: &[(String, [u8; 4])]) -> Vec<u8> {
    let mut img = vec![0u8; w * h * 4];
    let row = 9; // logical; the crisp font draws at twice that
    let shade = |img: &mut [u8], x0: usize, y0: usize, x1: usize, y1: usize| {
        for y in y0..y1.min(h) {
            for x in x0..x1.min(w) {
                img[(y * w + x) * 4 + 3] = 170;
            }
        }
    };
    let width = |list: &[(String, [u8; 4])]| list.iter().map(|(t, _)| hud::text_width(t)).max().unwrap_or(0) as usize * 2 + 16;
    if !lines.is_empty() {
        shade(&mut img, 0, 0, width(lines), lines.len() * row * 2 + 12);
        for (i, (text, colour)) in lines.iter().enumerate() {
            hud::draw_text_crisp(&mut img, (w as u32, h as u32), (4, 3 + (i * row) as i32), text, *colour);
        }
    }
    if !bottom.is_empty() {
        let top = h.saturating_sub(bottom.len() * row * 2 + 10);
        shade(&mut img, 0, top, width(bottom), h);
        for (i, (text, colour)) in bottom.iter().enumerate() {
            hud::draw_text_crisp(&mut img, (w as u32, h as u32), (4, (top / 2) as i32 + 3 + (i * row) as i32), text, *colour);
        }
    }
    img
}

/// Lines of panel text, each with its colour.
type Lines = Vec<(String, [u8; 4])>;

const WHITE: [u8; 4] = [225, 225, 225, 255];
const PICKED: [u8; 4] = [255, 196, 96, 255];
const DIM: [u8; 4] = [150, 155, 160, 255];

/// The open window.
struct App {
    s: Settings,
    /// The preset last picked, to mark the panel when a control has moved.
    preset: usize,
    /// The panel page shown, and the control picked on each.
    page: usize,
    cursor: [usize; PAGES.len()],
    /// I was pressed: infect under the mouse on the next frame.
    infect_once: bool,
    scale: usize,
    window: Option<Arc<Window>>,
    pixels: Option<Pixels<'static>>,
    shared: Option<Shared>,
    engine: Option<Engine>,
    stamps: Option<Stamps>,
    adapter: String,
    max_walkers: usize,
    paused: bool,
    step_once: bool,
    show_panel: bool,
    panel_dirty: bool,
    shift: bool,
    ctrl: bool,
    hand: Hand,
    frames: u64,
    /// Frames drawn since the window opened.
    drawn: u64,
    /// `MOULDGPU_SCREENSHOT_AFTER_FRAMES`: save the frame, panel and all, then quit.
    shot_after: Option<u64>,
    fps_clock: Instant,
    fps: f64,
    /// Living walkers, read twice a second while growth is on.
    alive: Option<usize>,
    result: Result<(), String>,
    /// The camera over the dish (a world bigger than the window scrolls),
    /// the view pass that draws through it, and the RTS controls.
    camera: Camera,
    view: Option<View>,
    controls: Controls,
    /// Q: the stand-in squad, for trying the controls (no rules).
    squad: Option<Squad>,
    marks: Marks,
    last_frame: Instant,
    /// The game UI over the squad (`world::ui`), drawn by the canvas pass.
    hud: Hud,
    canvas: Option<Canvas>,
    canvas_pass: Option<CanvasPass>,
    /// Paused before the menu opened (the menu pauses; closing restores).
    menu_paused: bool,
    /// L: the look (the world shaded from the simulation's fields, every
    /// walker a speck, soldiers as sprites), on by default; off draws the
    /// engine's frame as it is. Its ground field (agar and piles).
    look: bool,
    ground: Option<wgpu::Buffer>,
}

impl App {
    fn new(s: Settings, preset: usize, scale: usize) -> Self {
        App {
            s,
            preset,
            page: 0,
            cursor: [0; PAGES.len()],
            infect_once: false,
            scale,
            window: None,
            pixels: None,
            shared: None,
            engine: None,
            stamps: None,
            adapter: String::new(),
            max_walkers: *WALKER_STEPS.last().unwrap(),
            paused: false,
            step_once: false,
            show_panel: true,
            panel_dirty: true,
            shift: false,
            ctrl: false,
            hand: Hand::default(),
            frames: 0,
            drawn: 0,
            shot_after: std::env::var("MOULDGPU_SCREENSHOT_AFTER_FRAMES").ok().and_then(|v| v.parse().ok()),
            fps_clock: Instant::now(),
            fps: 0.0,
            alive: None,
            result: Ok(()),
            camera: Camera::new((1024, 640), (1024, 640)),
            view: None,
            controls: {
                let mut c = Controls::new(0);
                c.arrows_pan = false;
                c
            },
            squad: None,
            marks: Marks::default(),
            look: true,
            ground: None,
            last_frame: Instant::now(),
            hud: Hud::new(),
            canvas: None,
            canvas_pass: None,
            menu_paused: false,
        }
    }

    fn world(&self) -> (usize, usize) {
        self.engine.as_ref().map_or((1, 1), |e| (e.dish.w, e.dish.h))
    }

    /// The minimap's rectangle on screen this frame, if drawn.
    fn minimap(&self) -> Option<(u32, u32, u32, u32)> {
        self.view.as_ref().and_then(|v| v.minimap_rect(&self.camera))
    }

    /// What the game UI gave: selection and menu events are the window's,
    /// the rest the squad's commands.
    fn hud_events(&mut self, events: Vec<HudEvent>, event_loop: &ActiveEventLoop) {
        for ev in events {
            let units = self.squad.as_ref().map(|q| q.units()).unwrap_or_default();
            match ev {
                HudEvent::SelectOnly(k) => self.controls.selected = vec![k],
                HudEvent::Deselect(k) => self.controls.selected.retain(|&q| q != k),
                HudEvent::Group(g) => self.controls.selected = self.controls.groups[g].iter().copied().filter(|k| units.iter().any(|u| u.id == *k && u.alive)).collect(),
                HudEvent::Idle => {
                    let after = self.controls.selected.first().copied();
                    if let Some(k) = self.squad.as_ref().and_then(|q| q.next_idle(after).or_else(|| q.next_idle(None))) {
                        self.controls.selected = vec![k];
                        if let Some(u) = units.iter().find(|u| u.id == k) {
                            self.camera.centre_on((u.x, u.y));
                        }
                    }
                }
                HudEvent::Menu(MenuItem::Opened) => {
                    self.menu_paused = self.paused;
                    self.paused = true;
                }
                HudEvent::Menu(MenuItem::Resume) => self.paused = self.menu_paused,
                HudEvent::Menu(MenuItem::Restart) => {
                    self.squad = self.engine.as_ref().map(|e| Squad::new((e.dish.w, e.dish.h), self.scale));
                    self.controls.selected.clear();
                    self.paused = self.menu_paused;
                }
                HudEvent::Menu(MenuItem::Quit) => event_loop.exit(),
                other => {
                    if let Some(sq) = self.squad.as_mut() {
                        sq.hud(&other, &self.controls.selected);
                    }
                }
            }
        }
    }

    fn fail(&mut self, event_loop: &ActiveEventLoop, msg: String) {
        eprintln!("mouldgpu: {msg}");
        self.result = Err(msg);
        event_loop.exit();
    }

    fn restart(&mut self) {
        let (Some(p), Some(shared)) = (&self.pixels, &self.shared) else { return };
        self.s.walkers = self.s.walkers.min(self.max_walkers);
        let dropped = self.engine.take().map(|e| e.dropped).unwrap_or_default();
        let mut e = Engine::new(p.device(), p.queue(), shared, &self.s, self.scale);
        e.dropped = dropped;
        e.lay_food(self.s.food);
        let world = (e.dish.w, e.dish.h);
        if let Some(v) = self.view.as_mut() {
            if v.world != world {
                self.camera = Camera::new(world, v.screen);
                self.squad = self.squad.take().map(|_| Squad::new(world, self.scale));
            }
            v.set_world(&e.frame, world);
        }
        self.engine = Some(e);
        self.ground = None;
        self.apply_look();
        self.panel_dirty = true;
        if let Some(w) = &self.window {
            w.set_title(&format!("Mould sandbox (GPU): {}", PRESETS[self.preset].to_lowercase()));
        }
    }

    /// Binds the look to the view (or takes it off), from the engine's
    /// fields and a ground field made from its dish.
    fn apply_look(&mut self) {
        let (Some(p), Some(e), Some(v)) = (&self.pixels, &self.engine, self.view.as_mut()) else { return };
        if !self.look {
            v.clear_look();
            return;
        }
        let g = self.ground.get_or_insert_with(|| {
            let crystal = look::dish_crystals(&e.dish);
            let codes = look::ground(e.dish.w, e.dish.w * e.dish.h, &look::Fields { agar: &e.dish.agar, crystal: &crystal, ..Default::default() });
            look::ground_buffer(p.device(), p.queue(), &codes)
        });
        v.set_look(&e.look, g, &e.walkers, e.n);
    }

    fn panel_lines(&self) -> (Lines, Lines) {
        if !self.show_panel {
            return (Vec::new(), Vec::new());
        }
        let crowd_ok = self.engine.as_ref().is_none_or(|e| e.crowd_ok);
        // The seed does not count as a change.
        let moved = Settings { seed: 0, ..self.s.clone() } != Settings { seed: 0, ..preset(self.preset) };
        let (name, knobs) = PAGES[self.page];
        let mut lines = vec![
            (format!("MOULD SANDBOX ON {}   H HIDES", self.adapter.to_uppercase().chars().filter(|&c| hud::has_crisp_glyph(c)).collect::<String>()), WHITE),
            (format!("{} {}{}", preset_key(self.preset), PRESETS[self.preset], if moved { " (CHANGED)" } else { "" }), PICKED),
            (format!("PAGE {}/{} {name}   TAB: NEXT PAGE", self.page + 1, PAGES.len()), DIM),
        ];
        for (i, k) in knobs.iter().enumerate() {
            let at = i == self.cursor[self.page];
            let mark = if at { ">" } else { " " };
            let colour = if at { PICKED } else { WHITE };
            lines.push((format!("{mark} {:<18} {}", k.label(), k.value(&self.s, crowd_ok)), colour));
        }
        let (done, ms) = self.engine.as_ref().map(|e| e.done).map_or((0, String::new()), |d| {
            let ms = self.stamps.as_ref().map_or("GPU TIME N/A".to_string(), |st| format!("GPU {:.2} MS A FRAME (TICKS {:.2} + DRAWING {:.2} + VIEW {:.2})", st.sim_ms + st.draw_ms + st.view_ms, st.sim_ms, st.draw_ms, st.view_ms));
            (d, ms)
        });
        // Each line fits the 1024-cell dish at the crisp font's 12 px a
        // character (`the_bottom_lines_fit_the_dish`).
        let mouse = if self.squad.is_some() {
            "SQUAD: LEFT SELECTS  RIGHT ORDERS  F+DRAG BURNS  CTRL+1-9 GROUPS  Q ENDS"
        } else if self.s.sick_on() {
            "LEFT DROPS WALKERS  RIGHT WIPES  I INFECTS  F DROPS FOOD  ESC QUITS"
        } else {
            "LEFT DRAG DROPS WALKERS  RIGHT DRAG WIPES  F DROPS FOOD  ESC QUITS"
        };
        let world = self.engine.as_ref().map_or((0, 0), |e| (e.dish.w, e.dish.h));
        let camera = format!("WHEEL ZOOMS ({:.2}X)  MIDDLE DRAG OR EDGE PANS  M MAP  L LOOK  W WORLD {}X{}  Q SQUAD", self.camera.zoom, world.0, world.1);
        let bottom = vec![
            ("1 BIG DISH  2 32K  3 NO LIMIT  4 JENSON  5 +RESPAWN  6 BY WALKERS  7 4M  8 4M OURS".to_string(), WHITE),
            ("9 TRUNK RING  0 CONTAGION  SHIFT+1 SWEEP  2 CURVES  3 POLYGONS  4 ENDEMIC  5 ROPES".to_string(), WHITE),
            ("SHIFT+6 TERRITORIES  7 RIBBONS  8 EXPLORERS  9 DROPLETS  0 BEADS".to_string(), WHITE),
            ("CTRL+1 GRAZING DROPLETS  2 GRAZING SPECIES  3 FED MOULD  4 STARVING MOULD".to_string(), WHITE),
            ("ARROWS PICK AND CHANGE (SHIFT: MORE)  SPACE PAUSE  . STEP  R RESTART  N NEW SEED".to_string(), DIM),
            (mouse.to_string(), DIM),
            (camera, DIM),
            (format!("{ms}  {:.0} FPS  {:.1} S  SEED {}{}{}", self.fps, done as f64 / STEPS_PER_SECOND as f64, self.s.seed, self.alive.map_or(String::new(), |a| format!("  ALIVE {}", thousands(a))), if self.paused { "  PAUSED" } else { "" }), WHITE),
        ];
        (lines, bottom)
    }

    fn key(&mut self, code: KeyCode, event_loop: &ActiveEventLoop) {
        // With the squad out, the digits are its groups and the rest of
        // the keys the controls do not use stay the sandbox's.
        if let Some(sq) = self.squad.as_mut() {
            let units = sq.units();
            let digit = matches!(code, KeyCode::Digit1 | KeyCode::Digit2 | KeyCode::Digit3 | KeyCode::Digit4 | KeyCode::Digit5 | KeyCode::Digit6 | KeyCode::Digit7 | KeyCode::Digit8 | KeyCode::Digit9);
            if digit {
                self.controls.key(code, true, false, &units, &mut self.camera);
                return;
            }
        }
        const DIGITS: [KeyCode; 10] = [KeyCode::Digit1, KeyCode::Digit2, KeyCode::Digit3, KeyCode::Digit4, KeyCode::Digit5, KeyCode::Digit6, KeyCode::Digit7, KeyCode::Digit8, KeyCode::Digit9, KeyCode::Digit0];
        // 1-9 and 0 are presets 1-10; Shift with 1-9 and 0 the ten after,
        // Ctrl with them the ten after that.
        let digit = DIGITS.iter().position(|&k| k == code).map(|k| k + if self.ctrl { 20 } else if self.shift { 10 } else { 0 }).filter(|&k| k < PRESETS.len());
        if let Some(k) = digit {
            let seed = self.s.seed;
            self.preset = k;
            self.s = Settings { seed, ..preset(k) };
            self.restart();
            return;
        }
        match code {
            KeyCode::Escape => event_loop.exit(),
            KeyCode::Tab => self.page = (self.page + 1) % PAGES.len(),
            KeyCode::ArrowUp | KeyCode::ArrowDown => {
                let n = PAGES[self.page].1.len();
                let c = &mut self.cursor[self.page];
                *c = if code == KeyCode::ArrowUp { (*c + n - 1) % n } else { (*c + 1) % n };
            }
            KeyCode::KeyI => self.infect_once = true,
            KeyCode::ArrowLeft | KeyCode::ArrowRight => {
                let dir = if code == KeyCode::ArrowRight { 1 } else { -1 };
                let knob = PAGES[self.page].1[self.cursor[self.page]];
                if knob.nudge(&mut self.s, dir, self.shift, self.max_walkers) {
                    self.restart();
                } else if knob == Knob::Food {
                    let food = self.s.food;
                    if let Some(e) = self.engine.as_mut() {
                        e.lay_food(food);
                    }
                }
            }
            KeyCode::Space | KeyCode::KeyP | KeyCode::Pause => self.paused = !self.paused,
            KeyCode::Period => self.step_once = true,
            KeyCode::KeyR => self.restart(),
            KeyCode::KeyN => {
                self.s.seed += 1;
                self.restart();
            }
            KeyCode::KeyH => self.show_panel = !self.show_panel,
            KeyCode::KeyL => {
                self.look = !self.look;
                self.apply_look();
            }
            KeyCode::KeyM => {
                if let Some(v) = self.view.as_mut() {
                    v.minimap = !v.minimap;
                }
            }
            KeyCode::KeyQ => {
                self.squad = match self.squad {
                    Some(_) => None,
                    None => self.engine.as_ref().map(|e| Squad::new((e.dish.w, e.dish.h), self.scale)),
                };
                self.controls.selected.clear();
                self.hud.cancel_targeting();
                self.camera.set_inset(0.0, 0.0);
                if self.squad.is_some() {
                    self.paused = false;
                }
            }
            KeyCode::KeyW => {
                // The world: today's 1024 x 640 dish, or twice as wide and
                // high (2048 x 1280) with four times the walkers, bigger
                // than a screen, to scroll.
                let (scale, walkers) = if self.scale == 4 { (8, self.s.walkers * 4) } else { (4, (self.s.walkers / 4).max(10_000)) };
                self.scale = scale;
                self.s.walkers = walkers.min(self.max_walkers);
                if let Some(e) = self.engine.as_mut() {
                    e.dropped.clear();
                }
                self.restart();
            }
            KeyCode::KeyF if self.squad.is_some() => {}
            KeyCode::KeyF => {
                let food = self.s.food;
                if let (Some(e), Some(at)) = (self.engine.as_mut(), self.hand.at) {
                    if e.can_drop() {
                        e.dropped.push(at);
                        e.lay_food(food);
                    }
                }
            }
            _ => return,
        }
        self.panel_dirty = true;
    }

    fn frame(&mut self, event_loop: &ActiveEventLoop) {
        self.frames += 1;
        let since = self.fps_clock.elapsed().as_secs_f64();
        if since >= 0.5 {
            self.fps = self.frames as f64 / since;
            self.frames = 0;
            self.fps_clock = Instant::now();
            self.panel_dirty = true;
            // A blocking read of the energy (0.4 MB at 100,000 slots), only
            // while growth is on and the panel shows it.
            self.alive = self.engine.as_ref().filter(|e| e.grows && self.show_panel).map(|e| e.alive());
        }
        let dt = self.last_frame.elapsed().as_secs_f32();
        self.last_frame = Instant::now();
        let before = (self.camera.x, self.camera.y, self.camera.zoom);
        self.controls.update(&mut self.camera, dt);
        if (self.camera.x, self.camera.y, self.camera.zoom) != before {
            self.hand.at = self.controls.cursor.and_then(|c| self.camera.world_at(c));
        }
        if self.panel_dirty {
            // The panel is screen-space now (the view's overlay); the
            // engine's own dish-sized `hud` stays empty.
            // With the squad out the game UI replaces the panel.
            let (lines, bottom) = self.panel_lines();
            let squad = self.squad.is_some();
            if let Some(v) = self.view.as_mut() {
                if squad {
                    v.set_overlay(None);
                } else {
                    let img = panel(v.screen.0 as usize, v.screen.1 as usize, &lines, &bottom);
                    v.set_overlay(Some(&img));
                }
            }
            self.panel_dirty = false;
        }
        self.marks.clear();
        if let Some(sq) = self.squad.as_mut() {
            sq.step(if self.paused { 0.0 } else { dt });
            let units = sq.units();
            self.controls.prune(&units);
            sq.marks_with(self.camera.zoom, self.look, &mut self.marks);
            self.controls.marks(&units, &self.camera, sq.radius(), &mut self.marks);
            for n in sq.notes.drain(..) {
                self.hud.alert(&n.text, n.at, if n.bad { theme::BAD } else { theme::GOOD });
            }
        }
        // The game UI: drawn into the canvas (and its ghost and pings into
        // the marks), and the camera told how much of the screen it covers.
        let minimap = self.view.as_ref().is_some_and(|v| v.minimap) && !self.camera.sees_all();
        if let Some(c) = self.canvas.as_mut() {
            c.clear();
            if let Some(sq) = self.squad.as_ref() {
                // What is under the cursor in the world (not behind a panel
                // or the minimap), and where a placement would land
                // (through the minimap when on it).
                let hover = self.controls.cursor.filter(|&p| !self.hud.over(p)).and_then(|p| self.camera.world_at(p));
                let aim = self.controls.cursor.and_then(|p| self.hud.aim(p, &self.camera));
                let ghost = self.hud.targeting().and_then(|(id, _, from)| aim.map(|h| (id, h, from)));
                let model = sq.model(&self.controls.selected, hover, ghost, self.paused);
                self.hud.draw(&model, &self.controls, &self.camera, minimap, c, &mut self.marks);
                self.camera.set_inset(self.hud.top, c.screen.1 as f32 - self.hud.bottom);
            }
            let bx = if self.squad.is_some() { self.hud.minimap_box } else { None };
            if let Some(v) = self.view.as_mut() {
                v.minimap_box = bx;
            }
            if let Some(cp) = self.canvas_pass.as_mut() {
                cp.upload(c);
            }
        }
        if let Some(v) = self.view.as_mut() {
            v.set_marks(&self.marks);
        }
        let ticks = if self.paused {
            std::mem::take(&mut self.step_once) as u32
        } else {
            self.s.ticks
        };
        let (Some(p), Some(shared), Some(e), Some(view)) = (&self.pixels, &self.shared, self.engine.as_mut(), self.view.as_ref()) else { return };
        let mut canvas_pass = self.canvas_pass.as_mut();
        let camera = self.camera;
        let mut hand = self.hand;
        hand.infect |= std::mem::take(&mut self.infect_once);
        e.write_params(&self.s, hand, ticks);
        let soft = self.s.curve == Curve::Soft;
        let stamps = self.stamps.as_ref().and_then(|st| st.take().map(|_| st));
        // pixels skips the closure when the window is hidden, so the
        // stamps count as written only if it ran.
        let mut wrote = false;
        let res = p.render_with(|enc, target, ctx| {
            let timed = stamps.filter(|_| ticks > 0);
            e.record(shared, enc, ticks, soft, timed.map(|st| &st.qs));
            view.record(enc, &camera, timed.map(|st| (&st.qs, None, Some(3))));
            if let Some(cp) = canvas_pass.as_mut() {
                cp.record(enc, &view.out, view.stride);
            }
            if let Some(st) = timed {
                st.resolve(enc);
                wrote = true;
            }
            view.copy_to(enc, &ctx.texture);
            ctx.scaling_renderer.render(enc, target);
            Ok(())
        });
        if let Err(err) = res {
            return self.fail(event_loop, format!("render failed: {err}"));
        }
        if let Some(st) = self.stamps.as_mut() {
            if wrote {
                st.map();
            }
            let _ = p.device().poll(wgpu::PollType::Poll);
            st.poll();
        }
        self.drawn += 1;
        if self.shot_after == Some(self.drawn) {
            let path = std::env::temp_dir().join(format!("mouldgpu-screenshot-p{}.png", self.preset + 1));
            view.picture().save(&path).unwrap();
            println!("mouldgpu: wrote {}", path.display());
            let st = self.stamps.as_ref().map_or("no timestamps".to_string(), |st| {
                let (sim, draw, view, n) = st.median();
                format!("GPU {:.3} ms a frame (ticks {sim:.3} + drawing {draw:.3} + view {view:.3}), median of {n} frames timed", sim + draw + view)
            });
            println!("mouldgpu: after {} frames, {} ticks: {st}; {:.1} frames a second", self.drawn, e.done, self.fps);
            event_loop.exit();
        }
    }
}

/// The window's first size: the dish if it fits in nine tenths of the
/// screen, else that (a world bigger than the screen scrolls).
fn first_size(world: (u32, u32), monitor: Option<PhysicalSize<u32>>) -> PhysicalSize<u32> {
    let room = monitor.map_or((1600, 900), |m| (m.width * 9 / 10, m.height * 9 / 10));
    PhysicalSize::new(world.0.min(room.0).max(320), world.1.min(room.1).max(240))
}

impl ApplicationHandler for App {
    fn resumed(&mut self, event_loop: &ActiveEventLoop) {
        if self.window.is_some() {
            return;
        }
        let p = big_params(&self.s, self.scale, false);
        let world = (p.w() as u32, p.h() as u32);
        let first = first_size(world, event_loop.primary_monitor().map(|m| m.size()));
        let attributes = Window::default_attributes().with_title("Mould sandbox (GPU)").with_inner_size(first);
        let window = match event_loop.create_window(attributes) {
            Ok(w) => Arc::new(w),
            Err(err) => return self.fail(event_loop, format!("window creation failed: {err}")),
        };
        // Look at the GPU first, to ask for timestamps if it has them.
        let instance = wgpu::Instance::new(wgpu::InstanceDescriptor::new_without_display_handle_from_env());
        let probe = block_on(instance.request_adapter(&wgpu::RequestAdapterOptions { power_preference: wgpu::PowerPreference::HighPerformance, ..Default::default() })).ok();
        // The pixel buffer is the window's size, one pixel a screen pixel:
        // the camera's view pass draws the dish into it at any zoom.
        let size = window.inner_size();
        let (w, h) = (size.width.max(1), size.height.max(1));
        let options = wgpu::RequestAdapterOptions { power_preference: wgpu::PowerPreference::HighPerformance, compatible_surface: None, force_fallback_adapter: false };
        let mut built = None;
        if let Some(a) = probe.as_ref().filter(|a| a.features().contains(wgpu::Features::TIMESTAMP_QUERY)) {
            let desc = wgpu::DeviceDescriptor { label: Some("mouldgpu"), required_features: wgpu::Features::TIMESTAMP_QUERY, required_limits: a.limits(), ..Default::default() };
            built = PixelsBuilder::new(w, h, SurfaceTexture::new(size.width, size.height, Arc::clone(&window))).request_adapter_options(options.clone()).device_descriptor(desc).build().ok();
        }
        let timestamps = built.is_some();
        if built.is_none() {
            built = match PixelsBuilder::new(w, h, SurfaceTexture::new(size.width, size.height, Arc::clone(&window))).request_adapter_options(options).build() {
                Ok(p) => Some(p),
                Err(err) => return self.fail(event_loop, format!("pixels init failed: {err}")),
            };
        }
        let mut pixels = built.unwrap();
        pixels.set_scaling_mode(ScalingMode::PixelPerfect);
        // pixels 0.17 recomputes the drawing's scale (and the mouse's
        // inverse) only on a resize, so a window opened smaller than the
        // dish would draw it cropped until one came (independent review,
        // 2026-11-11).
        if let Err(err) = pixels.resize_surface(size.width, size.height) {
            return self.fail(event_loop, format!("surface resize failed: {err}"));
        }
        let info = pixels.adapter().get_info();
        self.adapter = info.name.clone();
        println!("mouldgpu: {} ({:?}), timestamps {}", info.name, info.backend, if timestamps { "yes" } else { "no" });
        let limit = pixels.device().limits().max_storage_buffer_binding_size as usize;
        self.max_walkers = WALKER_STEPS.iter().copied().filter(|&n| n * 16 <= limit).max().unwrap_or(100_000);
        self.shared = Some(Shared::new(pixels.device()));
        if timestamps {
            self.stamps = Some(Stamps::with_view(pixels.device(), pixels.queue(), true));
        }
        self.pixels = Some(pixels);
        self.window = Some(window);
        self.restart();
        if let (Some(p), Some(e)) = (&self.pixels, &self.engine) {
            let world = (e.dish.w, e.dish.h);
            self.view = Some(View::new(p.device(), p.queue(), &e.frame, world, (w, h)));
            self.camera = Camera::new(world, (w, h));
            self.canvas = Some(Canvas::new((w, h)));
            self.canvas_pass = Some(CanvasPass::new(p.device(), p.queue()));
        }
        self.apply_look();
    }

    fn window_event(&mut self, event_loop: &ActiveEventLoop, _id: WindowId, event: WindowEvent) {
        match event {
            WindowEvent::CloseRequested => event_loop.exit(),
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
                    self.panel_dirty = true;
                }
            }
            WindowEvent::ModifiersChanged(m) => {
                self.shift = m.state().shift_key();
                self.ctrl = m.state().control_key();
                (self.controls.shift, self.controls.ctrl) = (self.shift, self.ctrl);
            }
            WindowEvent::KeyboardInput { event, .. } => {
                if let PhysicalKey::Code(code) = event.physical_key {
                    // With the squad out the game UI has the keys first: its
                    // command card's hotkeys, Esc for its menu, Space for
                    // the newest alert. (F is its burn-line button, so the
                    // controls' F+drag is not needed here.)
                    if self.squad.is_some() && !event.repeat {
                        let (took, events) = self.hud.key(code, event.state == ElementState::Pressed, self.ctrl, &mut self.camera);
                        self.hud_events(events, event_loop);
                        if took {
                            return;
                        }
                    }
                    let repeat_ok = matches!(code, KeyCode::ArrowLeft | KeyCode::ArrowRight | KeyCode::ArrowUp | KeyCode::ArrowDown | KeyCode::Period);
                    if event.state == ElementState::Pressed && (!event.repeat || repeat_ok) {
                        self.key(code, event_loop);
                    }
                }
            }
            WindowEvent::CursorMoved { position, .. } => {
                // Screen pixels (the buffer is the window's size), then the
                // world cell under them through the camera.
                let at = (position.x as f32, position.y as f32);
                let mm = self.minimap();
                self.hud.moved(Some(at));
                self.controls.moved(Some(at), &mut self.camera, mm);
                self.hand.at = self.camera.world_at(at).filter(|_| mm.is_none_or(|r| slime_mould::world::view::minimap_world(r, self.world(), at).is_none()));
            }
            WindowEvent::CursorLeft { .. } => {
                self.hand.at = None;
                self.hud.moved(None);
                self.controls.moved(None, &mut self.camera, None);
            }
            WindowEvent::Focused(focused) => {
                self.controls.edge = focused;
                if !focused {
                    self.hand.pull = false;
                    self.hand.wipe = false;
                    self.hand.infect = false;
                    self.controls.unfocus();
                }
            }
            WindowEvent::MouseWheel { delta, .. } => {
                let notches = match delta {
                    MouseScrollDelta::LineDelta(_, y) => y.signum() as i32,
                    MouseScrollDelta::PixelDelta(p) => (p.y / 40.0).round() as i32,
                };
                self.controls.wheel(notches, &mut self.camera);
                self.panel_dirty = true;
            }
            WindowEvent::MouseInput { state, button, .. } => {
                let down = state == ElementState::Pressed;
                let mm = self.minimap();
                let on_map = self.controls.cursor.zip(mm).is_some_and(|(c, r)| slime_mould::world::view::minimap_world(r, self.world(), c).is_some());
                let b = match button {
                    MouseButton::Left => Some(Button::Left),
                    MouseButton::Right => Some(Button::Right),
                    MouseButton::Middle => Some(Button::Middle),
                    _ => None,
                };
                // The middle button pans, the minimap moves the camera, and
                // with the squad out every button is the controls'; else the
                // left and right are the sandbox's tools as before.
                // The game UI first: a click on a panel, or while placing.
                if let (Some(b), true) = (b, self.squad.is_some()) {
                    let (took, events) = self.hud.button(b, down, &self.camera, self.shift);
                    self.hud_events(events, event_loop);
                    if took {
                        return;
                    }
                }
                let to_controls = self.squad.is_some() || button == MouseButton::Middle || (on_map && down) || !down;
                if let (Some(b), true) = (b, to_controls) {
                    let units = self.squad.as_ref().map(|q| q.units()).unwrap_or_default();
                    let cmds = self.controls.button(b, down, &units, &mut self.camera, mm);
                    if let Some(sq) = self.squad.as_mut() {
                        for c in cmds {
                            sq.command(c);
                        }
                    }
                }
                if self.squad.is_none() && !(on_map && down) {
                    match button {
                        MouseButton::Left => self.hand.pull = down,
                        MouseButton::Right => self.hand.wipe = down,
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

fn write_gif(path: &str, shots: Vec<image::RgbaImage>, ms: u32) {
    let delay = image::Delay::from_numer_denom_ms(ms, 1);
    let frames: Vec<image::Frame> = shots.into_iter().map(|img| image::Frame::from_parts(img, 0, 0, delay)).collect();
    let file = std::fs::File::create(path).expect("gif file");
    let mut enc = image::codecs::gif::GifEncoder::new_with_speed(file, 10);
    enc.set_repeat(image::codecs::gif::Repeat::Infinite).unwrap();
    enc.encode_frames(frames).expect("gif encode");
}

/// The brightest of each `down` x `down` block, channel by channel, so thin
/// veins survive the shrink.
fn shrink(img: &image::RgbaImage, down: u32) -> image::RgbaImage {
    if down <= 1 {
        return img.clone();
    }
    let (w, h) = (img.width() / down, img.height() / down);
    image::RgbaImage::from_fn(w, h, |x, y| {
        let mut out = [0u8, 0, 0, 255];
        for dy in 0..down {
            for dx in 0..down {
                let p = img.get_pixel(x * down + dx, y * down + dy).0;
                for k in 0..3 {
                    out[k] = out[k].max(p[k]);
                }
            }
        }
        image::Rgba(out)
    })
}

struct Look {
    dir: String,
    seconds: f32,
    gif: bool,
    /// Full-size PNG frames every `every` frames in `dir/tag/`, for a video encoder.
    frames: bool,
    every: u64,
    down: u32,
    twice: bool,
    tag: String,
    /// `view=WxH`: also draw the last frame through the camera onto a
    /// screen that size (`cam=X,Y,ZOOM`, the world point at its middle;
    /// default the dish's middle at zoom 1), with the stand-in squad if
    /// `squad=1`, and time the view pass.
    view: Option<(u32, u32)>,
    cam: Option<(f32, f32, f32)>,
    squad: bool,
    /// `ui=STATE` with `squad=1`: the game UI over the squad, in one of
    /// the states the page shows (group, one, enemy, wall, turret, menu,
    /// help, empty).
    ui: String,
    /// `walls=AX,AY,BX,BY,WIDTH@SECONDS;...`: Larder World walls, in dish
    /// cells, each built (`Engine::build_wall`) before the first frame at or
    /// after its time.
    walls: Vec<([f32; 5], f32)>,
    /// `fire=N`: the Larder world's squad hooks every frame (`Engine::
    /// set_squad`, W2a): N units on the crystals in turn, each taking a
    /// little charge and smelling, and (with growth on) N shots, each from a
    /// unit at the block it aimed at last frame, read back every frame as
    /// the game will, for the frame cost.
    fire: usize,
}

/// The headless run: frames as the window would submit them (one frame of
/// `ticks` ticks and the drawing a submit), timed, then measured and drawn.
fn headless(s: &Settings, scale: usize, look: &Look) {
    let instance = wgpu::Instance::new(wgpu::InstanceDescriptor::new_without_display_handle_from_env());
    let adapter = block_on(instance.request_adapter(&wgpu::RequestAdapterOptions { power_preference: wgpu::PowerPreference::HighPerformance, ..Default::default() })).expect("no GPU adapter");
    let info = adapter.get_info();
    println!("adapter: {} ({:?}, {:?}, driver {} {})", info.name, info.backend, info.device_type, info.driver, info.driver_info);
    let timestamps = adapter.features().contains(wgpu::Features::TIMESTAMP_QUERY);
    let required_features = if timestamps { wgpu::Features::TIMESTAMP_QUERY } else { wgpu::Features::empty() };
    let (device, queue) = block_on(adapter.request_device(&wgpu::DeviceDescriptor { label: Some("mouldgpu"), required_features, required_limits: adapter.limits(), ..Default::default() })).expect("no device");
    let shared = Shared::new(&device);
    std::fs::create_dir_all(&look.dir).unwrap();
    let mut pictures = Vec::new();
    for round in 0..if look.twice { 2 } else { 1 } {
        let mut e = Engine::new(&device, &queue, &shared, s, scale);
        if round == 0 {
            println!(
                "== {} walkers on {} x {} (crowd {}), sensor {} turn {} reach {} laid {} keep {} respawn {} food {}, drawn {:?} {:?} {:?}, seed {}",
                e.n,
                e.dish.w,
                e.dish.h,
                (s.crowd && e.crowd_ok) as u8,
                s.sensor_angle,
                s.turn,
                s.reach,
                s.laid,
                s.keep,
                s.respawn,
                s.food as u8,
                s.colour,
                s.field,
                s.curve,
                s.seed
            );
            let looks = Settings { seed: s.seed, ..Settings::big_dish() };
            if (s.start, s.trail_scale, s.cap, s.floor, s.wander) != (looks.start, looks.trail_scale, looks.cap, looks.floor, looks.wander) {
                println!(
                    "   looks: start {:?} disc {} haze {}; trail scale {} reach {:?} angle {:?} turn {:?} step {:?}; cap {} floor {} wander {}",
                    s.start, s.disc, s.haze, s.trail_scale, s.reach_by, s.angle_by, s.turn_by, s.step_by, s.cap, s.floor, s.wander
                );
            }
            if s.upkeep > 0.0 {
                println!(
                    "   growth: upkeep {} start energy {} divides at {} birth cost {} starving dies {} a tick, {} slots",
                    s.upkeep, s.start_energy, s.birth_energy, s.birth_cost, s.starve, s.capacity.max(s.walkers)
                );
            }
            if s.crystal > 0.0 || s.rock > 0.0 || s.wet > 0.0 || !look.walls.is_empty() {
                println!(
                    "   world: {} piles{}, crystal {} eat {} flow {} regrow {}; rock {} wet {} (step {}); walls {} (health {} bite {} scent {})",
                    s.piles,
                    if s.rect { " in a rectangle" } else { "" },
                    s.crystal,
                    s.crystal_eat,
                    s.crystal_flow,
                    s.crystal_regrow,
                    s.rock,
                    s.wet,
                    s.wet_step,
                    look.walls.len(),
                    s.wall_hp,
                    s.wall_bite,
                    s.wall_scent
                );
            }
            if s.sick_on() {
                println!(
                    "   contagion: sick {} immune {} ticks, catch {}, germ {} kept {}, sick speed {} trail {} reach {} turn {}, seeded at {} s r {}",
                    s.sick, s.immune, s.catch_at, s.germ, s.germ_keep, s.sick_speed, s.sick_trail, s.sick_reach, s.sick_turn, s.seed_at, s.seed_r
                );
            }
            if (s.fields(), s.explore, s.blocked_lay, s.blocked_keep, s.chance, s.size_spread, s.sensor_r, s.graze) != (0, 0, 0.0, false, 0.0, 0.0, 0, 0.0) {
                println!(
                    "   outside: species {} avoid {} zones {} cycle {} next {} prev {} convert {} boost {}; explore {} ticks, trail {} spread {}; blocked lay {} keep {} reset {}; chance {} size spread {} sensor r {}; graze {} eat {} regrow {}",
                    s.fields(),
                    s.avoid,
                    s.zones as u8,
                    s.cycle as u8,
                    s.next,
                    s.prev,
                    s.convert,
                    s.boost,
                    s.explore,
                    s.explorer_trail,
                    s.explore_spread,
                    s.blocked_lay,
                    s.blocked_keep as u8,
                    s.blocked_reset,
                    s.chance,
                    s.size_spread,
                    s.sensor_r,
                    s.graze,
                    s.graze_eat,
                    s.graze_regrow
                );
            }
            // The species settings (2026-11-11): the share, conversion's
            // square, the second kind, and each species' own settings.
            if s.species_share > 0.0 || s.convert_r > 0 || s.mix_share > 0.0 || s.per_species() {
                println!(
                    "   species: share {} convert r {}; mix share {} reach {} angle {} turn {} step {} deposit {}; own settings {:?}",
                    s.species_share,
                    s.convert_r,
                    s.mix_share,
                    s.mix_reach,
                    s.mix_angle,
                    s.mix_turn,
                    s.mix_step,
                    s.mix_deposit,
                    &s.sets[..s.fields() as usize]
                );
            }
            // The life cycle (2026-11-11).
            if s.spore_rate > 0.0 || s.spore_burst > 0.0 || s.sleep_rate > 0.0 || s.alarm_keep > 0.0 {
                println!(
                    "   life: spores rate {} over scent {} burst {} over alarm {}, {} steps at {}x, reach {} lay {} land over {} rest {}; dormant rate {} lay {} wake over scent {} or alarm {}, awake {} rest {} die {}; alarm keep {} weight {} relay {}",
                    s.spore_rate,
                    s.spore_scent,
                    s.spore_burst,
                    s.spore_alarm,
                    s.spore_steps,
                    s.spore_speed,
                    s.spore_reach,
                    s.spore_deposit,
                    s.spore_land,
                    s.spore_rest,
                    s.sleep_rate,
                    s.sleep_deposit,
                    s.wake_scent,
                    s.wake_alarm,
                    s.wake_steps,
                    s.sleep_rest,
                    s.sleep_death,
                    s.alarm_keep,
                    s.alarm_weight,
                    s.alarm_relay
                );
            }
        }
        e.write_params(s, Hand::default(), s.ticks);
        let mut st = timestamps.then(|| Stamps::new(&device, &queue));
        let frames = (look.seconds * STEPS_PER_SECOND as f32 / s.ticks as f32).round() as u64;
        let (mut wall, mut gpu) = (Vec::new(), Vec::new());
        let mut shots = Vec::new();
        // With contagion on, the share sick every half second, for the
        // wave's course (a read back, so it only runs headless).
        let half = (STEPS_PER_SECOND as u64 / 2 / s.ticks as u64).max(1);
        let mut course = Vec::new();
        let mut built = vec![false; look.walls.len()];
        let mut aims: Vec<Option<(f32, f32)>> = vec![None; look.fire];
        let (mut fired, mut kills, mut took) = (0u64, 0u64, 0f64);
        for f in 0..frames {
            let now = (f * s.ticks as u64) as f32 / STEPS_PER_SECOND as f32;
            for (b, &(w, at)) in built.iter_mut().zip(&look.walls) {
                if !*b && now >= at {
                    e.build_wall(&shared, s, w[0], w[1], w[2], w[3], w[4]);
                    *b = true;
                }
            }
            if look.fire > 0 {
                use slime_mould::mould::squad::{slime_trail, Frame, ShotIn, UnitIn};
                let k = e.dish.p.scale as f32;
                let piles = &e.dish.piles;
                let units: Vec<UnitIn> = (0..look.fire)
                    .map(|u| {
                        let (px, py) = piles[u % piles.len()];
                        UnitIn { x: px * k + (u / piles.len()) as f32 * 3.0, y: py * k, touch: 3.0, sight: 30.0, take_r: 4.0, take: 0.5, smell_r: 2.0, smell: 0.5 }
                    })
                    .collect();
                let shots: Vec<ShotIn> = if e.grows { units.iter().zip(&aims).filter_map(|(u, a)| a.map(|to| ShotIn { from: (u.x, u.y), to, r: 3.0 })).collect() } else { Vec::new() };
                fired += shots.len() as u64;
                e.set_squad(&Frame { units, shots, slime: slime_trail(s.knee), crater: 0 });
            }
            // GPU time on every tenth frame (the read back waits).
            let stamp = st.as_ref().filter(|_| f % 10 == 5);
            // Again each frame, for the timed seeding.
            e.write_params(s, Hand::default(), s.ticks);
            let mut enc = device.create_command_encoder(&wgpu::CommandEncoderDescriptor { label: None });
            e.record(&shared, &mut enc, s.ticks, s.curve == Curve::Soft, stamp.map(|x| &x.qs));
            if let Some(x) = stamp {
                x.resolve(&mut enc);
            }
            let t0 = Instant::now();
            queue.submit([enc.finish()]);
            device.poll(wgpu::PollType::wait_indefinitely()).expect("poll");
            if look.fire > 0 {
                // The game reads the frame back before it plans the next.
                let out = e.squad_out();
                aims = out.units.iter().map(|u| u.target).collect();
                kills += out.shots.iter().map(|q| q.killed as u64).sum::<u64>();
                took += out.units.iter().map(|u| u.took as f64).sum::<f64>();
            }
            wall.push(t0.elapsed().as_secs_f64() * 1e3);
            if stamp.is_some() {
                let x = st.as_mut().unwrap();
                let t: Vec<u64> = read_back(&device, &queue, &x.resolve).chunks_exact(8).map(|c| u64::from_le_bytes(c.try_into().unwrap())).collect();
                let ms = |a: u64, b: u64| b.saturating_sub(a) as f64 * x.period / 1e6;
                gpu.push((ms(t[0], t[1]), ms(t[1], t[2])));
            }
            if round == 0 && s.sick_on() && (f + 1) % half == 0 {
                course.push(format!("{:.3}", e.sick_share(s)));
            }
            if round == 0 && f % look.every == 0 {
                if look.gif {
                    shots.push(shrink(&e.picture(), look.down));
                }
                if look.frames {
                    let dir = format!("{}/{}", look.dir, look.tag);
                    std::fs::create_dir_all(&dir).unwrap();
                    e.picture().save(format!("{dir}/{:05}.png", f / look.every)).unwrap();
                }
            }
        }
        let pic = e.picture();
        // The same frame through the camera, before the profile below moves
        // the dish on.
        if let (0, Some(screen)) = (round, look.view) {
            view_shot(&device, &queue, &e, screen, look, timestamps);
        }
        if round == 0 {
            let median = |v: &mut Vec<f64>| {
                v.sort_by(|a, b| a.partial_cmp(b).unwrap());
                v.get(v.len() / 2).copied().unwrap_or(0.0)
            };
            let timed = &mut wall[1.min(wall.len())..].to_vec();
            let worst = timed.iter().cloned().fold(0.0, f64::max);
            let m = median(timed);
            print!("wall ms a frame of {} ticks and the drawing, {} frames: median {m:.3}, worst {worst:.3}", s.ticks, frames);
            if !gpu.is_empty() {
                let (mut a, mut b): (Vec<f64>, Vec<f64>) = gpu.iter().cloned().unzip();
                print!("; GPU ms ticks {:.3}, drawing {:.3}", median(&mut a), median(&mut b));
            }
            println!();
            if look.fire > 0 {
                println!("squad: {} units a frame, {fired} shots, {kills} walkers killed, {took:.1} charge taken (the wall ms above include reading it back every frame)", look.fire);
            }
            let mm = e.measure();
            let mut sick = if s.sick_on() { format!("  sick {:.3}", e.sick_share(s)) } else { String::new() };
            if e.grows {
                sick += &format!("  alive {}", mm.alive);
            }
            println!("dish at {:.1}s: tube {:.1}  loops {}  width {:.2}  reach {:.1}  piles {}/{}  strays {:.3}{sick}", mm.seconds, mm.tube_area, mm.loops, mm.width, mm.reach, mm.reached, e.dish.piles.len(), mm.strays);
            if !course.is_empty() {
                println!("sick every 0.5 s: {}", course.join(" "));
            }
            if e.cycles {
                // `measure` read the states back into the dish.
                println!("life: walkers, spores, dormant, dead {:?}", e.dish.life_counts());
            }
            if e.npiles > 0 || e.walled {
                let (_, bites, broken) = e.world_counts();
                println!("world: crystals left {:.3}, wall bites {bites}, wall cells broken {broken}", e.dish.crystals_left());
            }
            let path = format!("{}/{}-{:03}s.png", look.dir, look.tag, look.seconds.round() as u32);
            pic.save(&path).unwrap();
            println!("wrote {path}");
            if look.gif {
                let path = format!("{}/{}.gif", look.dir, look.tag);
                let ms = (1000.0 * look.every as f32 * s.ticks as f32 / STEPS_PER_SECOND as f32).round() as u32;
                write_gif(&path, shots, ms.max(20));
                println!("wrote {path}");
            }
            // Where a tick's time goes: every pass stamped on its own, the
            // median of 11 ticks (after the measures, which they move on).
            if timestamps {
                let runs: Vec<Vec<(&str, f64)>> = (0..11).map(|_| e.profile(&shared, s.curve == Curve::Soft)).collect();
                let parts: Vec<String> = (0..runs[0].len())
                    .map(|q| {
                        let mut v: Vec<f64> = runs.iter().map(|r| r[q].1).collect();
                        v.sort_by(|a, b| a.partial_cmp(b).unwrap());
                        format!("{} {:.3}", runs[0][q].0, v[v.len() / 2])
                    })
                    .collect();
                println!("GPU ms a pass, median of 11 ticks each submitted alone: {}", parts.join(", "));
            }
        }
        pictures.push(pic);
    }
    if look.twice {
        println!("same picture twice: {}", if pictures[0] == pictures[1] { "yes" } else { "NO" });
    }
}

/// The headless run's last frame through the camera (`view=WxH`), saved as
/// `TAG-view.png`, with the view pass's GPU time (median of 21).
fn view_shot(device: &wgpu::Device, queue: &wgpu::Queue, e: &Engine, screen: (u32, u32), look: &Look, timestamps: bool) {
    let world = (e.dish.w, e.dish.h);
    let mut view = View::new(device, queue, &e.frame, world, screen);
    let mut cam = Camera::new(world, screen);
    let (cx, cy, zoom) = look.cam.unwrap_or((world.0 as f32 / 2.0, world.1 as f32 / 2.0, cam.zoom));
    cam.zoom_to(zoom, (0.0, 0.0));
    cam.centre_on((cx, cy));
    let mut marks = Marks::default();
    let mut canvas = Canvas::new(screen);
    let mut hud = Hud::new();
    if look.squad {
        // The squad, three selected, one ordered off and two on a burn line,
        // and a drag box under way: every mark the controls draw.
        let mut sq = Squad::new(world, e.dish.p.scale);
        let mut k = Controls::new(0);
        sq.command(UiCommand::Order { units: vec![0], at: (sq.units[0].x + 60.0, sq.units[0].y - 40.0) });
        let (a, b) = ((sq.units[4].x - 20.0, sq.units[4].y + 30.0), (sq.units[7].x + 30.0, sq.units[7].y + 30.0));
        sq.command(UiCommand::Burn { units: vec![4, 5], from: a, to: b });
        sq.step(0.5);
        k.selected = vec![0, 4, 5];
        if !look.ui.is_empty() {
            ui_state(&look.ui, &mut sq, &mut k, &mut cam, &mut view, &mut canvas, &mut hud, &mut marks);
            view.minimap_box = hud.minimap_box;
        }
        let units = sq.units();
        let mut squad_marks = Marks::default();
        sq.marks(cam.zoom, &mut squad_marks);
        k.marks(&units, &cam, sq.radius(), &mut squad_marks);
        // The squad under the UI's ghost and pings.
        squad_marks.list.append(&mut marks.list);
        marks = squad_marks;
    }
    view.set_marks(&marks);
    if look.ui.is_empty() {
        let (lines, bottom) = (vec![("CAMERA VIEW".to_string(), WHITE)], vec![(format!("ZOOM {:.2}  WORLD {} X {}", cam.zoom, world.0, world.1), DIM)]);
        view.set_overlay(Some(&panel(screen.0 as usize, screen.1 as usize, &lines, &bottom)));
    }
    let mut cp = CanvasPass::new(device, queue);
    cp.upload(&mut canvas);
    // The view and the UI timed apart (stamps 0-1 and 2-3), and the UI's
    // CPU side (binning and upload, every frame in the game) on the clock.
    let (mut times, mut ui_times, mut cpu_times) = (Vec::new(), Vec::new(), Vec::new());
    let qs = timestamps.then(|| device.create_query_set(&wgpu::QuerySetDescriptor { label: Some("view"), ty: wgpu::QueryType::Timestamp, count: 4 }));
    let resolve = device.create_buffer(&wgpu::BufferDescriptor { label: Some("view stamps"), size: 32, usage: wgpu::BufferUsages::QUERY_RESOLVE | wgpu::BufferUsages::COPY_SRC, mapped_at_creation: false });
    for _ in 0..21 {
        let t0 = Instant::now();
        cp.upload(&mut canvas);
        cpu_times.push(t0.elapsed().as_secs_f64() * 1e3);
        let mut enc = device.create_command_encoder(&wgpu::CommandEncoderDescriptor { label: None });
        view.record(&mut enc, &cam, qs.as_ref().map(|q| (q, Some(0), Some(1))));
        cp.record_timed(&mut enc, &view.out, view.stride, qs.as_ref().map(|q| (q, Some(2), Some(3))));
        if let Some(q) = &qs {
            enc.resolve_query_set(q, 0..4, &resolve, 0);
        }
        queue.submit([enc.finish()]);
        if qs.is_some() {
            let t: Vec<u64> = read_back(device, queue, &resolve).chunks_exact(8).map(|c| u64::from_le_bytes(c.try_into().unwrap())).collect();
            let period = queue.get_timestamp_period() as f64 / 1e6;
            times.push(t[1].saturating_sub(t[0]) as f64 * period);
            ui_times.push(t[3].saturating_sub(t[2]) as f64 * period);
        }
    }
    let median = |v: &mut Vec<f64>| {
        v.sort_by(|a, b| a.partial_cmp(b).unwrap());
        v.get(v.len() / 2).copied()
    };
    let path = format!("{}/{}-view.png", look.dir, look.tag);
    view.picture().save(&path).unwrap();
    let ms = match (median(&mut times), median(&mut ui_times)) {
        (Some(v), Some(u)) => format!("GPU {v:.3} ms the view, {u:.3} ms the UI"),
        _ => "no timestamps".to_string(),
    };
    let cpu = median(&mut cpu_times).unwrap_or(0.0);
    println!("view {}x{} at zoom {:.2} on ({cx:.0}, {cy:.0}), {} marks, {} UI shapes: {ms}, CPU {cpu:.3} ms binning and uploading the UI (medians of 21); wrote {path}", screen.0, screen.1, cam.zoom, marks.len(), canvas.len());
}

/// The squad and its UI in one of the page's states (`ui=STATE`). The UI
/// is drawn twice: once to lay it out (so a button's place is known for a
/// hover), then for the picture.
#[allow(clippy::too_many_arguments)]
fn ui_state(state: &str, sq: &mut Squad, k: &mut Controls, cam: &mut Camera, view: &mut View, canvas: &mut Canvas, hud: &mut Hud, marks: &mut Marks) {
    use slime_mould::world::demo::WALL_ID;
    // A fight under way: two soldiers forward and hurt, one lost, a wall
    // and a turret built, an upgrade researching.
    k.selected.clear();
    for (i, hp) in [(1usize, 38.0), (2, 71.0), (6, 22.0), (9, 0.0)] {
        sq.units[i].hp = hp;
    }
    sq.lost = 1;
    sq.energy = 1240.0;
    sq.time = 254.0;
    sq.armour = 1;
    sq.research = Some(7.0);
    let base = (sq.units[0].x, sq.units[0].y);
    let s = sq.scale;
    sq.turrets.push((base.0 - 10.0 * s, base.1 + 18.0 * s));
    sq.walls.push(((base.0 + 30.0 * s, base.1 - 10.0 * s), (base.0 + 34.0 * s, base.1 + 22.0 * s)));
    k.groups[0] = vec![0, 1, 2, 3, 4, 5];
    k.groups[1] = vec![6, 7, 8];
    k.groups[3] = vec![10, 11];
    hud.alert("Soldier 7 is under fire", Some((sq.units[6].x, sq.units[6].y)), theme::BAD);
    hud.alert("Soldier 10 was lost", Some((sq.units[9].x, sq.units[9].y)), theme::BAD);
    let minimap = view.minimap && !cam.sees_all();
    let draw = |sq: &Squad, k: &Controls, cam: &mut Camera, canvas: &mut Canvas, hud: &mut Hud, marks: &mut Marks, paused: bool| {
        canvas.clear();
        marks.clear();
        let hover = k.cursor.filter(|&p| !hud.over(p)).and_then(|p| cam.world_at(p));
        let aim = k.cursor.and_then(|p| hud.aim(p, cam));
        let ghost = hud.targeting().and_then(|(id, _, from)| aim.map(|h| (id, h, from)));
        let m = sq.model(&k.selected, hover, ghost, paused);
        hud.draw(&m, k, cam, minimap, canvas, marks);
        cam.set_inset(hud.top, canvas.screen.1 as f32 - hud.bottom);
    };
    let to_screen = |cam: &Camera, w: (f32, f32)| cam.to_screen(w);
    match state {
        "group" | "turret" | "wall" => k.selected = vec![0, 1, 2, 3, 4, 5, 6, 7],
        "one" => k.selected = vec![1],
        "enemy" | "menu" | "help" => k.selected = vec![0, 1, 2, 3],
        _ => {}
    }
    draw(sq, k, cam, canvas, hud, marks, false);
    match state {
        "group" => {
            // Hover the wall button: its tooltip.
            if let Some(r) = hud.command_rect(WALL_ID) {
                hud.moved(Some((r.x + r.w / 2.0, r.y + r.h / 2.0)));
            }
        }
        "enemy" => {
            let rival = sq.units.iter().position(|u| u.team == 1).unwrap();
            let p = to_screen(cam, (sq.units[rival].x, sq.units[rival].y));
            hud.moved(Some(p));
            k.cursor = Some(p);
        }
        "wall" => {
            hud.key(KeyCode::KeyV, true, false, cam);
            let a = to_screen(cam, (base.0 + 50.0 * s, base.1 - 30.0 * s));
            hud.moved(Some(a));
            hud.button(Button::Left, true, cam, false);
            let b = to_screen(cam, (base.0 + 70.0 * s, base.1 + 5.0 * s));
            hud.moved(Some(b));
            k.cursor = Some(b);
        }
        "turret" => {
            hud.key(KeyCode::KeyT, true, false, cam);
            // Over a soldier: red, it cannot go there.
            let p = to_screen(cam, (sq.units[3].x, sq.units[3].y));
            hud.moved(Some(p));
            k.cursor = Some(p);
        }
        "attack" => {
            hud.key(KeyCode::KeyA, true, false, cam);
        }
        "menu" => {
            hud.key(KeyCode::Escape, true, false, cam);
        }
        "help" => hud.show_help(),
        _ => {}
    }
    let paused = state == "menu" || state == "help";
    draw(sq, k, cam, canvas, hud, marks, paused);
    // The CPU cost of a frame's model and HUD layout, which the game pays
    // every frame (median of 21; the last draw is the one shown).
    let mut cpu = Vec::new();
    for _ in 0..21 {
        let t0 = Instant::now();
        draw(sq, k, cam, canvas, hud, marks, paused);
        cpu.push(t0.elapsed().as_secs_f64() * 1e3);
    }
    cpu.sort_by(|a, b| a.partial_cmp(b).unwrap());
    println!("ui {state}: CPU {:.3} ms the model and the HUD's layout (median of 21)", cpu[10]);
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let mut s = Settings::big_dish();
    let mut picked = 0usize;
    let mut scale = 4usize;
    let mut look = Look { dir: String::new(), seconds: 10.0, gif: false, frames: false, every: 6, down: 2, twice: false, tag: String::new(), view: None, cam: None, squad: false, ui: String::new(), walls: Vec::new(), fire: 0 };
    let args: Vec<String> = std::env::args().skip(1).collect();
    // The preset first, so the other keys change it.
    for arg in &args {
        if let Some(v) = arg.strip_prefix("preset=") {
            picked = v.parse::<usize>().map_err(|e| format!("preset={v}: {e}"))?.clamp(1, PRESETS.len()) - 1;
            s = preset(picked);
        }
    }
    for arg in &args {
        let (k, v) = arg.split_once('=').ok_or_else(|| format!("expected key=value, got {arg}"))?;
        match k {
            "preset" => {}
            "scale" => scale = v.parse()?,
            "look" => look.dir = v.into(),
            "seconds" => look.seconds = v.parse()?,
            "gif" => look.gif = on_off(k, v)?,
            "frames" => look.frames = on_off(k, v)?,
            "every" => look.every = v.parse::<u64>()?.max(1),
            "down" => look.down = v.parse::<u32>()?.max(1),
            "twice" => look.twice = on_off(k, v)?,
            "tag" => look.tag = v.into(),
            "view" => {
                let (w, h) = v.split_once('x').ok_or("view=WxH")?;
                look.view = Some((w.parse()?, h.parse()?));
            }
            "cam" => {
                let p: Vec<f32> = v.split(',').map(|x| x.parse()).collect::<Result<_, _>>()?;
                look.cam = Some((*p.first().ok_or("cam=X,Y,ZOOM")?, *p.get(1).ok_or("cam=X,Y,ZOOM")?, p.get(2).copied().unwrap_or(1.0)));
            }
            "squad" => look.squad = on_off(k, v)?,
            "ui" => look.ui = v.into(),
            "fire" => look.fire = v.parse::<usize>()?.min(slime_mould::mould::squad::UNITS_MAX),
            "walls" => {
                for w in v.split(';').filter(|w| !w.is_empty()) {
                    let (seg, at) = w.split_once('@').unwrap_or((w, "0"));
                    let p: Vec<f32> = seg.split(',').map(|x| x.parse()).collect::<Result<_, _>>()?;
                    let p: [f32; 5] = p.try_into().map_err(|_| "walls=AX,AY,BX,BY,WIDTH@SECONDS;...")?;
                    look.walls.push((p, at.parse()?));
                }
            }
            _ => set(&mut s, k, v)?,
        }
    }
    if !look.dir.is_empty() {
        if look.tag.is_empty() {
            look.tag = format!("p{}-s{}", picked + 1, s.seed);
        }
        headless(&s, scale, &look);
        return Ok(());
    }
    let event_loop = EventLoop::new()?;
    event_loop.set_control_flow(ControlFlow::Poll);
    let mut app = App::new(s, picked, scale);
    event_loop.run_app(&mut app)?;
    app.result.map_err(|e| e.into())
}

#[cfg(test)]
mod tests {
    use super::*;
    use slime_mould::mould::big::{BigDish, BigParams, LIFE_DEAD, LIFE_DORMANT, LIFE_SPORE, SOIL_BARREN, SOIL_CLAY, SOIL_SAND};
    use slime_mould::mould::tiers::Tier;

    #[test]
    fn every_panel_character_can_be_drawn() {
        let app = App::new(Settings::big_dish(), 0, 4);
        let mut texts: Vec<String> = PRESETS.iter().map(|s| s.to_string()).collect();
        for k in 0..PRESETS.len() {
            let s = preset(k);
            texts.push(preset_key(k));
            for knob in PAGES.iter().flat_map(|p| p.1.iter()) {
                texts.push(knob.label().into());
                assert!(knob.label().len() <= 18, "{} is wider than the panel's column", knob.label());
                texts.push(knob.value(&s, true));
                texts.push(knob.value(&s, false));
            }
        }
        let (lines, bottom) = app.panel_lines();
        texts.extend(lines.into_iter().chain(bottom).map(|(t, _)| t));
        for t in texts {
            for c in t.chars() {
                assert!(hud::has_crisp_glyph(c), "{c:?} in {t:?} has no glyph");
            }
        }
    }

    /// The looks build's preset line (87 characters) and the mouse line
    /// with contagion on (84) ran past the right edge of the 1024-cell dish.
    #[test]
    fn the_bottom_lines_fit_the_dish() {
        for k in [0, 9] {
            let app = App::new(preset(k), k, 4);
            let (_, bottom) = app.panel_lines();
            for (t, _) in bottom {
                // Drawn from x = 4 at twice the font's logical size.
                assert!(8 + 2 * hud::text_width(&t) <= 1024, "{t:?} is wider than the dish");
            }
        }
    }

    #[test]
    fn presets_are_the_comparisons_dishes() {
        let today = BigParams::today();
        let b = preset(0);
        assert_eq!((b.walkers, b.crowd, b.sensor_angle, b.turn, b.reach), (100_000, true, 45.0, 45.0, today.sensor_offset));
        // The nose rule smells food at today's reach in fine cells.
        assert_eq!(b.smell, 36.0);
        // The comparison scaled the deposit by 100,000 / walkers: 0.5 at 1M.
        let j = big_params(&preset(3), 4, false);
        assert!((j.deposit - 0.5).abs() < 1e-6, "{}", j.deposit);
        assert_eq!((j.rotate_angle.to_degrees().round(), j.sensor_offset), (23.0, 18.0));
        assert_eq!(preset(4).respawn, 0.01);
        // The 32k dish laid today's deposit a walker.
        assert!((big_params(&preset(1), 4, true).deposit - today.deposit).abs() < 1e-4);
        // The looks report's trunk test (`trunk.sh`, scale 4): a disc of 45
        // dish cells, 30% haze, respawn 0.001, sense_scale 0.02, step_mod 1:1.
        let k = big_params(&preset(8), 4, false);
        assert_eq!((k.blob, k.blob_r, k.haze, k.respawn, k.sense_scale, k.step_mod), (true, 45.0, 0.3, 0.001, 0.02, [1.0, 1.0]));
        assert!((k.deposit - 0.5).abs() < 1e-6 && k.crowd == 0);
        // Its 36 Points looks (raw kb, kc) and square-root deposit (ka).
        assert!((big_params(&preset(10), 4, false).turn_mod[0] - (-20f32).to_radians()).abs() < 1e-6);
        assert!((big_params(&preset(11), 4, false).angle_mod[0] - (-30f32).to_radians()).abs() < 1e-6);
        assert_eq!(big_params(&preset(12), 4, false).deposit_cap, 100);
        // Contagion (raw c1, c2, Rsick-s1): seeded at 4 s, 10 dish cells.
        let c = big_params(&preset(9), 4, false);
        assert_eq!((c.sick_steps, c.immune_steps, c.sick_catch, c.sick_trail, c.deposit_cap, c.sick_seed_r), (120, 480, 0.5, 3.0, 100, 0.0));
        assert_eq!((preset(9).seed_at, preset(9).seed_r / 4.0), (4.0, 10.0));
        assert_eq!((preset(13).sick, preset(13).immune), (60, 120));
        let r = big_params(&preset(14), 4, false);
        assert_eq!((r.sick_steps, r.immune_steps, r.sick_catch, r.sick_reach, r.sick_turn, r.sick_trail), (240, 6000, 5.0, 0.5, 2.0, 1.0));
        // The outside rules report's (`run.sh` and its commands, scale 4).
        let t = big_params(&preset(15), 4, false);
        assert_eq!((t.species, t.species_zones, t.species_avoid, t.blob, t.walkers), (2, true, 1.0, false, 1_000_000));
        assert!((t.deposit - 0.5).abs() < 1e-6 && t.sensor_offset == 18.0);
        let c = big_params(&preset(16), 4, false);
        assert_eq!((c.species, c.species_cycle, c.species_next, c.species_prev, c.sensor_offset, c.step), (3, true, 0.6, -1.0, 32.0, 2.0));
        assert!((c.sensor_angle.to_degrees() - 20.0).abs() < 1e-4 && (c.rotate_angle.to_degrees() - 20.0).abs() < 1e-4);
        let x = big_params(&preset(17), 4, false);
        assert_eq!((x.blob, x.blob_r, x.haze, x.explore_steps, x.explore_spread, x.explore_deposit), (true, 20.0, 0.0, 900, 1.0, 0.05));
        let d = big_params(&preset(18), 4, true);
        assert_eq!((d.walkers, d.crowd, d.sensor_offset, d.keep, d.blob_r, d.blocked_deposit, d.blocked_keep), (30_000, 1, 9.0, 0.93, 120.0, 1.0, false));
        assert!((d.deposit - 5.0).abs() < 1e-4 && (d.sensor_angle.to_degrees() - 90.0).abs() < 1e-4 && (d.rotate_angle.to_degrees() - 45.0).abs() < 1e-4);
        assert_eq!(big_params(&preset(19), 4, true).sensor_offset, 27.0);
        // The creatures report's: grazing weight 10 on the droplets; two
        // species with no avoidance, drawn by trail in species colours.
        let g = big_params(&preset(20), 4, true);
        assert_eq!((g.graze_weight, g.graze_eat, g.graze_regrow, g.species, g.blocked_deposit, g.walkers), (10.0, 0.05, 0.002, 0, 1.0, 30_000));
        let g2 = big_params(&preset(21), 4, true);
        assert_eq!((g2.graze_weight, g2.species, g2.species_avoid, g2.species_convert), (10.0, 2, 0.0, 0.0));
        assert_eq!(preset(21).field, Field::Trail);
        // Every preset has a key: 1-9 and 0, then Shift with 1-9 and 0.
        assert_eq!((preset_key(8), preset_key(9), preset_key(14)), ("9".to_string(), "0".to_string(), "SHIFT+5".to_string()));
        assert_eq!((preset_key(15), preset_key(19), preset_key(20), preset_key(21)), ("SHIFT+6".to_string(), "SHIFT+0".to_string(), "CTRL+1".to_string(), "CTRL+2".to_string()));
        // Growth's two: the census's fed and starved dishes.
        let (fed, starving) = (big_params(&preset(22), 4, true), big_params(&preset(23), 4, true));
        assert_eq!((preset_key(22), preset_key(23)), ("CTRL+3".to_string(), "CTRL+4".to_string()));
        assert_eq!((fed.walkers, fed.capacity, fed.upkeep, fed.graze_weight, fed.graze_regrow, starving.graze_regrow), (10_000, 100_000, 0.01, 10.0, 0.002, 0.0));
    }

    #[test]
    fn knobs_stay_in_range() {
        let mut s = Settings::big_dish();
        for knob in PAGES.iter().flat_map(|p| p.1.iter().copied()) {
            for _ in 0..40 {
                knob.nudge(&mut s, 1, true, 8_000_000);
            }
            for _ in 0..80 {
                knob.nudge(&mut s, -1, false, 8_000_000);
            }
        }
        assert!(s.walkers >= 10_000 && s.ticks >= 1 && s.keep >= 0.0 && s.reach >= 1.0 && s.step >= 0.25 && s.knee > 0.0);
        // The CPU adds sick and immune ticks in a u16; powers stay at 0 or
        // above, where `powf` and the GPU's `pow0` agree.
        assert!(s.sick + s.immune <= u16::MAX as u32 && s.catch_at > 0.0 && s.germ_keep < 1.0 && s.disc >= 10.0);
        assert!([s.reach_by[1], s.angle_by[1], s.turn_by[1], s.step_by[1]].iter().all(|&e| e >= 0.0));
        // The outside rules: at most four species (a colour each), never one.
        assert!(s.species <= MAX_SPECIES && s.species != 1 && s.sensor_r <= 3 && s.explorer_trail >= 0.0 && s.explore_spread <= 1.0 && s.chance >= 0.0 && s.boost >= 0.1);
        let mut t = Settings::big_dish();
        assert!(Knob::Species.nudge(&mut t, 1, false, 8_000_000) && t.fields() == 2, "species start at two, with a restart");
        assert!(Knob::Species.nudge(&mut t, -1, false, 8_000_000) && t.fields() == 0);
        assert!(!Knob::Zones.nudge(&mut t, 1, false, 8_000_000), "bands without species change nothing");
        assert_eq!(next_in(&WALKER_STEPS, 100_000, 1), 200_000);
        assert_eq!(next_in(&WALKER_STEPS, 100_000, -1), 32_000);
        assert_eq!(next_in(&WALKER_STEPS, 150_000, -1), 100_000);
        assert_eq!(next_in(&RESPAWN_STEPS, 0.0, -1), 0.0);
        // A value below the list (from the command line) goes up to its first.
        assert_eq!(next_in(&WALKER_STEPS, 5_000, 1), 10_000);
        assert_eq!(next_in(&WALKER_STEPS, 5_000, -1), 5_000);
        assert_eq!((percent(0.3), percent(0.0001), percent(0.0)), ("30%".to_string(), "0.01%".to_string(), "OFF".to_string()));
        // The limit restarts the dish (switched on mid-run it would jam).
        assert!(Knob::Crowd.nudge(&mut s, 1, false, 8_000_000));
        // Growth: on or off restarts, a change while on does not; the
        // grass's regrowth goes down to off and back.
        let mut g = Settings::big_dish();
        assert!(Knob::Upkeep.nudge(&mut g, 1, false, 8_000_000) && g.upkeep > 0.0);
        assert!(!Knob::Upkeep.nudge(&mut g, 1, false, 8_000_000));
        assert!(Knob::Slots.nudge(&mut g, 1, false, 8_000_000) && g.capacity == 200_000);
        g.graze_regrow = 0.0001;
        Knob::GrazeRegrow.nudge(&mut g, -1, false, 8_000_000);
        assert_eq!((g.graze_regrow, Knob::GrazeRegrow.value(&g, true)), (0.0, "OFF".to_string()));
        Knob::GrazeRegrow.nudge(&mut g, 1, false, 8_000_000);
        assert_eq!(g.graze_regrow, 0.0001);
        assert_eq!(thousands(1_000_000), "1,000,000");
        assert_eq!(thousands(32_000), "32,000");
    }

    /// A GPU for the twin tests, or `None` (the test then passes, saying it
    /// was skipped) on a machine with none. CI installs Mesa's software
    /// Vulkan (lavapipe) and sets `MOULDGPU_TEST_GPU=1`, so there a missing
    /// GPU fails the test instead.
    fn test_gpu() -> Option<(wgpu::Device, wgpu::Queue)> {
        let required = std::env::var("MOULDGPU_TEST_GPU").is_ok_and(|v| v == "1");
        let instance = wgpu::Instance::new(wgpu::InstanceDescriptor::new_without_display_handle_from_env());
        let adapter = match block_on(instance.request_adapter(&wgpu::RequestAdapterOptions::default())) {
            Ok(a) => a,
            Err(e) => {
                assert!(!required, "MOULDGPU_TEST_GPU=1 but no GPU adapter: {e}");
                eprintln!("no GPU adapter ({e}): skipped");
                return None;
            }
        };
        let info = adapter.get_info();
        eprintln!("adapter: {} ({:?})", info.name, info.backend);
        let desc = wgpu::DeviceDescriptor { label: Some("twin"), required_limits: adapter.limits(), ..Default::default() };
        Some(block_on(adapter.request_device(&desc)).expect("device"))
    }

    /// Small dishes for the GPU tests: scale 2 (512 x 320 fine cells), a
    /// quarter of the walkers, so software Vulkan runs them in seconds.
    const SCALE: usize = 2;

    fn small(k: usize) -> Settings {
        let s = preset(k);
        Settings { walkers: s.walkers / 4, ..s }
    }

    fn run(e: &mut Engine, shared: &Shared, s: &Settings, ticks: u32, hand: Hand) {
        e.write_params(s, hand, ticks);
        let mut enc = e.device.create_command_encoder(&wgpu::CommandEncoderDescriptor { label: None });
        e.record(shared, &mut enc, ticks, s.curve == Curve::Soft, None);
        e.queue.submit([enc.finish()]);
    }

    fn walkers_of(e: &Engine) -> Vec<(f32, f32, f32)> {
        floats(&read_back(&e.device, &e.queue, &e.walkers)).chunks_exact(4).map(|c| (c[0], c[1], c[2])).collect()
    }

    /// The shader's `keyed` (the CPU's, with the seed and use folded into
    /// `s` by `stream`).
    fn keyed_as_the_gpu(s: u32, t: u32, walker: u32) -> u32 {
        let low = |v: u32| {
            let mut x = v;
            x ^= x >> 16;
            x = x.wrapping_mul(0x7FEB_352D);
            x ^= x >> 15;
            x = x.wrapping_mul(0x846C_A68B);
            x ^ (x >> 16)
        };
        low(walker ^ low(low(s) ^ t))
    }

    /// The sum of |a - b| over the sum of |b|.
    fn l1(a: &[f32], b: &[f32]) -> f64 {
        let d: f64 = a.iter().zip(b).map(|(x, y)| (x - y).abs() as f64).sum();
        d / b.iter().map(|y| y.abs() as f64).sum::<f64>().max(1e-30)
    }

    /// Runs `ticks` GPU ticks from the CPU dish `cpu`'s state and the same
    /// on the CPU, and returns how far apart they end: (walkers that moved
    /// or turned on the CPU, walkers off by more than 0.001 in x, y or
    /// heading, the trail's L1 difference, the scent's).
    fn twin(e: &mut Engine, shared: &Shared, s: &Settings, cpu: &mut BigDish, ticks: u32) -> (usize, usize, f64, f64) {
        let before = cpu.walkers.clone();
        e.inject(cpu);
        run(e, shared, s, ticks, Hand::default());
        for _ in 0..ticks {
            cpu.tick();
        }
        let gpu = walkers_of(e);
        let side = (e.done % 2) as usize;
        let trail = floats(&read_back(&e.device, &e.queue, &e.trail[side]));
        let scent = floats(&read_back(&e.device, &e.queue, &e.scent[side]));
        let off = gpu.iter().zip(&cpu.walkers).filter(|(g, c)| (g.0 - c.0).abs() > 1e-3 || (g.1 - c.1).abs() > 1e-3 || (g.2 - c.2).abs() > 1e-3).count();
        let moved = before.iter().zip(&cpu.walkers).filter(|(a, b)| a != b).count();
        (moved, off, l1(&trail, &cpu.trail), l1(&scent, &cpu.scent))
    }

    /// **The twin test** (GPU plan phase 3's gate). The fields are `f32`, so
    /// a GPU and the CPU drift apart over a run (different `cos`, `sin` and
    /// summing order), and the plan's fallback applies: one GPU tick from
    /// the CPU dish's own state against one `BigDish::tick`. Each dish is
    /// warmed on the CPU first, so the trail, scent and crowding are a
    /// real dish's.
    #[test]
    fn one_gpu_tick_matches_one_cpu_tick() {
        let Some((device, queue)) = test_gpu() else { return };
        let shared = Shared::new(&device);
        // Presets 1 (one a cell, food), 3 (no limit), 4 (Jenson's angles
        // and reach, a million walkers in the sandbox) and 5 (4 with
        // respawn), and 1 with respawn (jumps that claim their cell).
        let dishes = [("1", small(0)), ("3", small(2)), ("4", small(3)), ("5", small(4)), ("1 + respawn", Settings { respawn: 0.01, ..small(0) })];
        for (name, s) in dishes {
            let mut e = Engine::new(&device, &queue, &shared, &s, SCALE);
            let mut cpu = e.dish.clone();
            for _ in 0..40 {
                cpu.tick();
            }
            let n = cpu.walkers.len();
            let (moved, off, lt, ls) = twin(&mut e, &shared, &s, &mut cpu, 1);
            eprintln!("preset {name}: {n} walkers, {moved} moved or turned, {off} off by more than 0.001; trail L1 {lt:.1e}, scent L1 {ls:.1e}");
            // On software Vulkan (2026-11-11) every walker agreed within
            // 0.001 (0.1% of them not to the bit: `cos` and `sin` round
            // differently), and the trail and scent agreed to the bit. The
            // bounds leave room for a real GPU's rounding to flip a near-tie
            // or two. Checked against two planted bugs: the tick's step
            // off by one put 143-14,492 walkers off (0.6-58%) and the trail
            // 5e-4-8e-3 off; the blocked draw on the wrong stream put 52%
            // off on preset 1.
            assert_eq!(moved, n, "preset {name}: every walker steps, turns or is blocked");
            assert!(off * 1000 <= n, "preset {name}: {off} of {n} walkers off");
            assert!(lt <= 1e-4 && ls <= 1e-6, "preset {name}: trail L1 {lt:.2e}, scent L1 {ls:.2e}");
        }
    }

    /// The twin test for the looks report's rules (2026-11-11): one GPU tick
    /// from a warmed CPU dish against one `BigDish::tick`, with the
    /// trail-set settings, the floor and wander, the square-root deposit,
    /// and contagion (sick walkers mid-dish, with and without the limit).
    /// Also the germ field and every walker's counter.
    #[test]
    fn the_looks_match_the_cpu_for_one_tick() {
        let Some((device, queue)) = test_gpu() else { return };
        let shared = Shared::new(&device);
        let j = small(3);
        // A scale that leaves most walkers short of full (s < 1), and a
        // different power for each setting, so a crossed one shows.
        let trail_set = Settings { trail_scale: 0.01, reach_by: [4.0, 0.5], angle_by: [-10.0, 2.0], turn_by: [-5.0, 1.5], step_by: [0.5, 0.75], ..j.clone() };
        let floor = Settings { floor: 20.0, wander: 0.01, ..j.clone() };
        // The disc start with haze, seeded sick in the middle at the start.
        let sick = Settings { start: Start::Disc, disc: 60.0, haze: 0.3, sick: 60, immune: 60, catch_at: 0.2, sick_trail: 3.0, sick_speed: 1.5, sick_reach: 0.5, sick_turn: 2.0, seed_at: 0.0, seed_r: 30.0, ..j.clone() };
        let dishes = [
            ("trail-set", trail_set),
            ("floor and wander", floor),
            ("square-root deposit", Settings { cap: 50, ..j.clone() }),
            ("contagion", sick.clone()),
            ("contagion, square-root deposit, respawn", Settings { cap: 100, respawn: 0.01, ..sick.clone() }),
            ("contagion, one a cell", Settings { start: Start::Spread, seed_r: 60.0, ..Settings { sick: 60, immune: 60, catch_at: 0.2, sick_trail: 3.0, sick_speed: 1.5, sick_reach: 0.5, sick_turn: 2.0, seed_at: 0.0, ..small(0) } }),
        ];
        for (name, s) in dishes {
            let mut e = Engine::new(&device, &queue, &shared, &s, SCALE);
            let mut cpu = e.dish.clone();
            for _ in 0..40 {
                cpu.tick();
            }
            let n = cpu.walkers.len();
            let germ_before = cpu.germ.clone();
            let (moved, off, lt, ls) = twin(&mut e, &shared, &s, &mut cpu, 1);
            let side = (e.done % 2) as usize;
            let (mut sick_off, mut lg, mut ill) = (0, 0.0, 0);
            if s.sick_on() {
                let gpu: Vec<u32> = read_back(&device, &queue, &e.sick).chunks_exact(4).take(n).map(|c| u32::from_le_bytes(c.try_into().unwrap())).collect();
                sick_off = gpu.iter().zip(&cpu.sick).filter(|(g, c)| **g != **c as u32).count();
                ill = cpu.sick.iter().filter(|&&c| c as u32 > s.immune).count();
                lg = l1(&floats(&read_back(&device, &queue, &e.germ[side])), &cpu.germ);
                assert!(germ_before.iter().any(|&g| g > 0.0), "{name}: germ laid in the warm-up");
            }
            eprintln!("{name}: {n} walkers, {moved} moved or turned, {off} off; trail L1 {lt:.1e}, scent L1 {ls:.1e}; {ill} sick, {sick_off} counters off, germ L1 {lg:.1e}");
            assert_eq!(moved, n, "{name}: every walker steps, turns or is blocked");
            assert!(off * 1000 <= n, "{name}: {off} of {n} walkers off");
            assert!(lt <= 1e-4 && ls <= 1e-6, "{name}: trail L1 {lt:.2e}, scent L1 {ls:.2e}");
            if s.sick_on() {
                assert!(ill > 0, "{name}: someone is sick");
                assert!(sick_off * 1000 <= n && lg <= 1e-4, "{name}: {sick_off} counters off, germ L1 {lg:.2e}");
            }
        }
    }

    /// The twin test for the outside rules (PRs #55, #56, #58, 2026-11-11): one
    /// GPU tick from a warmed CPU dish against one `BigDish::tick` with each
    /// rule on (species, cyclic species with a fourth weighed as an other,
    /// explorers with their spread and respawn, blocked walkers laying and
    /// keeping their heading, the chance turn, size spread, area sensors,
    /// grazing, conversion, species 0's boost, and mixes with the cap and
    /// contagion). Each
    /// species' field, every walker's age and species, and the grass are
    /// compared too. On software Vulkan (2026-11-11) every walker, age and
    /// species agreed on all fifteen dishes, the grass to the bit, and the
    /// fields to 2.4e-8 or better (exact on eleven). Sixteen planted bugs
    /// each failed it. PR #58's: the boost on species 1 (trail L1 4.0e-2),
    /// no boost on a walker's own reading in the convert pass (339 species
    /// off; 11, under the bound, with the boost of 1.1 a boosted warm-up
    /// allowed). PR #56's: conversion reading the field before the
    /// tick's deposit (1,434 of 250,000 species off), cyclic species
    /// joining the next not the previous (31,458), grass regrowing before
    /// it is eaten (grass L1 0.12), grass eaten where the walker stood
    /// before its move (4.1e-3). PR #55's: next and
    /// previous species swapped (128,091 of 250,000 walkers off), the
    /// explorers' readings not negated (44,727), the kept heading's draw on
    /// the blocked stream (3,755 of 25,000), a blocked explorer laying the
    /// full share (trail L1 3.3e-2), no age reset on a jump (1,774 ages
    /// off), the chance turn's sides swapped (111,308), the area square a
    /// row short (2,424), size on the angle not the reach (16,464), the
    /// avoidance's sign (3,946), the stage's spread on the size draw (96,782).
    #[test]
    fn the_outside_rules_match_the_cpu_for_one_tick() {
        let Some((device, queue)) = test_gpu() else { return };
        let shared = Shared::new(&device);
        let j = small(3);
        let one = small(0);
        let sick = |s: Settings| Settings { sick: 60, immune: 60, catch_at: 0.2, sick_trail: 3.0, seed_at: 0.0, seed_r: 60.0, ..s };
        let dishes = [
            // PR #58's boost: species 0 lays 1.5 times as much.
            ("two species in bands, species 0 boosted", Settings { species: 2, zones: true, avoid: 0.75, boost: 1.5, ..j.clone() }),
            ("three cyclic species", Settings { species: 3, cycle: true, avoid: 0.5, sensor_angle: 20.0, turn: 20.0, reach: 32.0, step: 2.0, ..j.clone() }),
            ("four cyclic species", Settings { species: 4, cycle: true, next: 0.4, prev: -0.8, ..j.clone() }),
            ("explorers, spread, respawn", Settings { explore: 60, explore_spread: 1.0, explorer_trail: 0.1, respawn: 0.01, ..j.clone() }),
            ("blocked lay and keep, one a cell", Settings { blocked_lay: 1.0, blocked_keep: true, blocked_reset: 0.1, ..one.clone() }),
            ("droplets", small(18)),
            ("chance, size spread, area sensors", Settings { chance: 2.0, size_spread: 1.0, sensor_r: 1, ..j.clone() }),
            ("species, explorers, cap, contagion", sick(Settings { species: 2, explore: 60, explore_spread: 0.5, cap: 100, ..j.clone() })),
            ("explorers, blocked lay, contagion, one a cell", sick(Settings { explore: 60, explore_spread: 1.0, blocked_lay: 0.5, ..one.clone() })),
            ("species, area sensors, one a cell", Settings { species: 3, sensor_r: 2, blocked_keep: true, ..one }),
            // PR #56: grazing and conversion.
            ("grazing droplets", small(20)),
            ("two species of grazing droplets", small(21)),
            ("two species converting, species 0 boosted", Settings { species: 2, convert: 1.2, boost: 1.5, ..j.clone() }),
            ("three cyclic species converting", Settings { species: 3, cycle: true, convert: 1.0, ..j.clone() }),
            ("converting, grazing, cap, explorers, respawn", Settings { species: 2, convert: 1.5, graze: 10.0, cap: 100, explore: 60, explore_spread: 1.0, respawn: 0.01, ..j.clone() }),
        ];
        for (name, s) in dishes {
            let mut e = Engine::new(&device, &queue, &shared, &s, SCALE);
            let mut cpu = e.dish.clone();
            // A converting dish warms up unboosted: boosted, species 0 takes
            // every walker within the 40 ticks (boost 1.5 at convert 1.2, or
            // 2 at 3, left none or one to convert), and a missing boost in
            // the convert pass slipped through at a boost of 1.1 (11 species
            // off, under the bound). The GPU has the boost all along; the
            // twin tick is the CPU's first with it.
            let boost = cpu.p.species_boost;
            if s.convert > 0.0 {
                cpu.p.species_boost = 1.0;
            }
            for _ in 0..40 {
                cpu.tick();
            }
            cpu.p.species_boost = boost;
            let n = cpu.walkers.len();
            let before_age = cpu.age.clone();
            let before_spec = cpu.spec.clone();
            let (moved, off, lt, ls) = twin(&mut e, &shared, &s, &mut cpu, 1);
            let side = (e.done % 2) as usize;
            let mut lsp = 0.0;
            if e.nsp > 0 {
                let fields: Vec<f32> = cpu.species_trail.iter().flatten().copied().collect();
                lsp = l1(&floats(&read_back(&device, &queue, &e.species[side])), &fields);
            }
            let ages: Vec<u32> = read_back(&device, &queue, &e.age).chunks_exact(4).take(n).map(|c| u32::from_le_bytes(c.try_into().unwrap())).collect();
            let ages_off = if cpu.age.is_empty() { 0 } else { ages.iter().zip(&cpu.age).filter(|(g, c)| **g != **c as u32).count() };
            let specs: Vec<u32> = read_back(&device, &queue, &e.spec).chunks_exact(4).take(n).map(|c| u32::from_le_bytes(c.try_into().unwrap())).collect();
            let (specs_off, joined) = if cpu.spec.is_empty() {
                (0, 0)
            } else {
                (specs.iter().zip(&cpu.spec).filter(|(g, c)| **g != **c as u32).count(), before_spec.iter().zip(&cpu.spec).filter(|(a, b)| a != b).count())
            };
            let lg = if cpu.grass.is_empty() { 0.0 } else { l1(&floats(&read_back(&device, &queue, &e.grass)), &cpu.grass) };
            // Explorers at the tick's start, by the CPU's rule (the keyed
            // draw rebuilt from `stream`, as the shader has it).
            let low = |v: u32| {
                let mut x = v;
                x ^= x >> 16;
                x = x.wrapping_mul(0x7FEB_352D);
                x ^= x >> 15;
                x = x.wrapping_mul(0x846C_A68B);
                x ^ (x >> 16)
            };
            let stage = |k: usize| {
                let u = (low(k as u32 | 0x8000_0000 ^ low(low(stream(s.seed, DRAW_SIZE)))) >> 8) as f32 / 16_777_216.0;
                s.explore as f32 * if s.explore_spread > 0.0 { 1.0 + s.explore_spread * (2.0 * u - 1.0) } else { 1.0 }
            };
            let young = if s.explore > 0 { before_age.iter().enumerate().filter(|&(k, &a)| (a as f32) < stage(k)).count() } else { 0 };
            eprintln!("{name}: {n} walkers, {moved} moved or turned, {off} off; trail L1 {lt:.1e}, scent L1 {ls:.1e}, species L1 {lsp:.1e}; {ages_off} ages off, {young} explorers; {joined} joined another species, {specs_off} species off; grass L1 {lg:.1e}");
            assert_eq!(cpu.species_trail.len(), e.nsp as usize, "{name}: the species fields");
            // A blocked walker that keeps its heading stays as it was.
            assert!(moved == n || (s.blocked_keep && moved > 0), "{name}: every walker steps, turns or is blocked");
            assert!(off * 1000 <= n, "{name}: {off} of {n} walkers off");
            assert!(lt <= 1e-4 && ls <= 1e-6 && lsp <= 1e-4, "{name}: trail L1 {lt:.2e}, scent L1 {ls:.2e}, species L1 {lsp:.2e}");
            assert!(ages_off * 1000 <= n, "{name}: {ages_off} ages off");
            assert!(specs_off * 1000 <= n && lg <= 1e-4, "{name}: {specs_off} species off, grass L1 {lg:.2e}");
            if s.convert > 0.0 {
                assert!(joined > 0, "{name}: someone joins another species in the tick");
            }
            if s.graze != 0.0 {
                assert!(cpu.grass.iter().any(|&g| g > 0.0 && g < 0.9), "{name}: grass eaten");
            }
            if s.explore > 0 {
                assert!(young > 0 && young < n, "{name}: explorers and networkers both ({young} of {n})");
            }
        }
    }

    /// The twin test for the creature thread's species settings on the GPU
    /// (2026-11-11): species 0's share of the walkers (`species_share`),
    /// conversion over a square (`species_convert_r`), the second kind of
    /// walker (`mix_*`), and each species' own settings (`species_sets`,
    /// `s0.reach=40` and the like): a droplet species beside a network
    /// species, own reach, step, deposit and keep, crowd limits that differ
    /// by species through respawn. One GPU tick from a warmed CPU dish
    /// against one `BigDish::tick`, as `the_outside_rules_match_the_cpu_for_one_tick`.
    #[test]
    fn the_species_settings_match_the_cpu_for_one_tick() {
        let Some((device, queue)) = test_gpu() else { return };
        let shared = Shared::new(&device);
        let j = small(3);
        let one = Settings { species: 2, zones: true, ..small(0) };
        let sick = |s: Settings| Settings { sick: 60, immune: 60, catch_at: 0.2, sick_trail: 3.0, seed_at: 0.0, seed_r: 60.0, ..s };
        let mix = |s: Settings| Settings { mix_share: 0.3, mix_reach: 3.0, mix_angle: 0.5, mix_turn: 2.0, mix_step: 1.5, mix_deposit: 0.5, ..s };
        let with = |mut s: Settings, keys: &[(&str, &str)]| {
            for (k, v) in keys {
                set(&mut s, k, v).unwrap();
            }
            s
        };
        // The creature thread's droplet and network species (big.rs
        // `species_sets_differ_by_species`, in the sandbox's units).
        let (drop, net) = (
            [("s0.crowd", "1"), ("s0.sensor_angle", "86"), ("s0.turn", "46"), ("s0.laid", "2"), ("s0.keep", "0.93"), ("s0.blocked_lay", "1"), ("s0.graze", "10")],
            [("s1.crowd", "0"), ("s1.keep", "0.6"), ("s1.graze", "0"), ("s1.explore", "60")],
        );
        let dishes = [
            ("species 0 at 70%, converting", Settings { species: 2, species_share: 0.7, convert: 1.2, ..j.clone() }),
            ("converting over a square of 3, bands", Settings { species: 2, zones: true, convert: 1.2, convert_r: 3, ..j.clone() }),
            ("hunting droplets converting over a square of 8 (the dead-end's dish)", Settings { avoid: -0.5, convert: 1.2, convert_r: 8, species_share: 0.55, ..one.clone() }),
            ("three cyclic species converting over a square of 2, cap", Settings { species: 3, cycle: true, convert: 1.0, convert_r: 2, cap: 100, ..j.clone() }),
            ("mixed walkers", mix(j.clone())),
            ("mixed walkers, one a cell, blocked lay and keep", mix(Settings { species: 0, blocked_lay: 1.0, blocked_keep: true, ..one.clone() })),
            ("mixed walkers, two species, cap, explorers, contagion", sick(mix(Settings { species: 2, explore: 60, explore_spread: 0.5, cap: 100, ..j.clone() }))),
            // The network species explores first, with a spread so some
            // still explore at the twin tick (with a stage of 20 every
            // walker had finished it, and the stage taken from the global
            // switch for both species went unseen).
            ("a droplet species and a network species", with(Settings { explore_spread: 1.0, ..one.clone() }, &[&drop[..], &net[..]].concat())),
            ("own reach, step, deposit and keep, scent at the own reach, respawn", with(Settings { species: 2, respawn: 0.01, smell: 0.0, ..j.clone() }, &[("s0.reach", "40"), ("s0.step", "2"), ("s1.laid", "3"), ("s1.keep", "0.8"), ("s1.sensor_angle", "30")])),
            ("crowd limits by species, respawn", with(Settings { respawn: 0.01, ..one.clone() }, &[("s1.crowd", "0"), ("s1.step", "1.5")])),
            // A third of the walkers jump a tick, so limited and unlimited
            // jumpers often aim at one cell.
            ("crowd limits by species, heavy respawn", with(Settings { respawn: 0.3, ..one.clone() }, &[("s1.crowd", "0")])),
            ("own settings, mixed, converting over a square of 4", mix(with(Settings { convert: 1.2, convert_r: 4, ..one.clone() }, &drop))),
        ];
        for (name, s) in dishes {
            let mut e = Engine::new(&device, &queue, &shared, &s, SCALE);
            assert_eq!(e.dish.p.species_sets.is_empty(), !s.per_species(), "{name}: the sets reach the CPU dish");
            assert_eq!(e.kinds, if s.mix_share > 0.0 { 2 } else { 1 }, "{name}: the kinds");
            // The GPU starts with the CPU dish's species and kinds (the
            // twin's inject writes them again, so this is the only check
            // of where they start).
            let start: Vec<u32> = read_back(&device, &queue, &e.spec).chunks_exact(4).take(e.n as usize).map(|c| u32::from_le_bytes(c.try_into().unwrap())).collect();
            assert!(start == spec_words(&e.dish), "{name}: the species and kinds the GPU starts with");
            let mut cpu = e.dish.clone();
            // The warm-up does not convert: with it one species took every
            // walker within the 40 ticks (species 0 at 70%). The twin tick
            // is the CPU's first with conversion, on a mixed dish.
            let convert = cpu.p.species_convert;
            cpu.p.species_convert = 0.0;
            for _ in 0..40 {
                cpu.tick();
            }
            cpu.p.species_convert = convert;
            let n = cpu.walkers.len();
            let before_spec = cpu.spec.clone();
            // Where each walker stands after the move and before any jump
            // (the tick without respawn, which comes last), for the jumps'
            // check below.
            let mut still = cpu.clone();
            still.p.respawn = 0.0;
            still.tick();
            let t = cpu.steps as u32;
            let (moved, off, lt, ls) = twin(&mut e, &shared, &s, &mut cpu, 1);
            let side = (e.done % 2) as usize;
            let mut lsp = 0.0;
            if e.nsp > 0 {
                let fields: Vec<f32> = cpu.species_trail.iter().flatten().copied().collect();
                lsp = l1(&floats(&read_back(&device, &queue, &e.species[side])), &fields);
            }
            let u32s = |b: &wgpu::Buffer| -> Vec<u32> { read_back(&device, &queue, b).chunks_exact(4).take(n).map(|c| u32::from_le_bytes(c.try_into().unwrap())).collect() };
            let ages_off = if cpu.age.is_empty() { 0 } else { u32s(&e.age).iter().zip(&cpu.age).filter(|(g, c)| **g != **c as u32).count() };
            let words = u32s(&e.spec);
            let specs_off = if cpu.spec.is_empty() { 0 } else { words.iter().zip(&cpu.spec).filter(|(g, c)| *g & 0xFF != **c as u32).count() };
            let joined = before_spec.iter().zip(&cpu.spec).filter(|(a, b)| a != b).count();
            // A walker's kind never changes, so every one must agree.
            let kinds_off = if cpu.kind.is_empty() { words.iter().filter(|&&w| w >> 8 != 0).count() } else { words.iter().zip(&cpu.kind).filter(|(g, c)| *g >> 8 != **c as u32).count() };
            let second = cpu.kind.iter().filter(|&&k| k == 1).count();
            let share0 = cpu.spec.iter().filter(|&&c| c == 0).count();
            let lg = if cpu.grass.is_empty() { 0.0 } else { l1(&floats(&read_back(&device, &queue, &e.grass)), &cpu.grass) };
            eprintln!(
                "{name}: {n} walkers, {moved} moved or turned, {off} off; trail L1 {lt:.1e}, scent L1 {ls:.1e}, species L1 {lsp:.1e}; {ages_off} ages off; {share0} of species 0, {joined} joined another species, {specs_off} species off; {second} of the second kind, {kinds_off} kinds off; grass L1 {lg:.1e}"
            );
            assert!(moved == n || (s.blocked_keep && moved > 0), "{name}: every walker steps, turns or is blocked");
            if s.respawn < 0.1 {
                assert!(off * 1000 <= n, "{name}: {off} of {n} walkers off");
            } else {
                // Heavy respawn. The GPU reads every jump's cell as the
                // jump pass began, where the CPU, jumping walkers in order,
                // sees a cell an earlier jumper left as free: 44 of 25,000
                // walkers off here (2026-11-11), as with one crowd limit for
                // all. So the jumps are checked where that cannot happen:
                // every walker whose jump lands in a cell no jumper left
                // must agree, and among them a walker of the unlimited
                // species and a limited one must aim at one cell, the
                // unlimited one first in the CPU's order, for the claim
                // rule to be seen.
                let key = |k: u32, q: u32| (keyed_as_the_gpu(stream(s.seed, DRAW_JUMP), t, 4 * k + q) >> 8) as f32 / 16_777_216.0;
                // The CPU's `cell` (private to big.rs).
                let (w, h) = (still.w, still.h);
                let cell = |x: f32, y: f32| (x >= 0.0 && y >= 0.0 && (x as usize) < w && (y as usize) < h).then(|| y as usize * w + x as usize);
                let at = |k: usize| cell(key(k as u32, 1) * w as f32, key(k as u32, 2) * h as f32).filter(|&j| still.agar[j]);
                let jumpers: Vec<usize> = (0..n).filter(|&k| key(k as u32, 0) < s.respawn).collect();
                let left: std::collections::HashSet<usize> = jumpers.iter().map(|&k| cell(still.walkers[k].0, still.walkers[k].1).unwrap()).collect();
                let clear: Vec<usize> = jumpers.iter().copied().filter(|&k| at(k).is_some_and(|j| !left.contains(&j))).collect();
                let gpu = walkers_of(&e);
                let clear_off = clear.iter().filter(|&&k| {
                    let (g, c) = (gpu[k], cpu.walkers[k]);
                    (g.0 - c.0).abs() > 1e-3 || (g.1 - c.1).abs() > 1e-3 || (g.2 - c.2).abs() > 1e-3
                }).count();
                let mut first: std::collections::HashMap<usize, usize> = std::collections::HashMap::new();
                for &k in &clear {
                    first.entry(at(k).unwrap()).or_insert(k);
                }
                let crowd_of = |k: usize| cpu.p.species_sets[still.spec[k] as usize].crowd;
                let contested = clear.iter().filter(|&&k| {
                    let f = first[&at(k).unwrap()];
                    f != k && crowd_of(f) == 0 && crowd_of(k) > 0
                }).count();
                eprintln!("   {} jumped, {} into cells no jumper left: {clear_off} off; {contested} limited walkers aimed at a cell an unlimited one took first", jumpers.len(), clear.len());
                assert!(clear_off == 0 && contested > 0, "{name}: {clear_off} jumpers off, {contested} contested");
                assert!(off * 100 <= n, "{name}: {off} of {n} walkers off");
            }
            assert!(lt <= 1e-4 && ls <= 1e-6 && lsp <= 1e-4 && lg <= 1e-4, "{name}: trail L1 {lt:.2e}, scent L1 {ls:.2e}, species L1 {lsp:.2e}, grass L1 {lg:.2e}");
            assert!(ages_off * 1000 <= n && specs_off * 1000 <= n, "{name}: {ages_off} ages, {specs_off} species off");
            assert_eq!(kinds_off, 0, "{name}: kinds off");
            if s.convert > 0.0 {
                assert!(joined > 0, "{name}: someone joins another species in the tick");
            }
            if s.mix_share > 0.0 {
                assert!(second * 5 > n && second * 2 < n, "{name}: {second} of {n} of the second kind");
            }
            if s.species_share > 0.0 {
                let want = s.species_share * n as f32;
                assert!((share0 as f32 - want).abs() < 0.2 * want, "{name}: {share0} of species 0, against {want}");
            }
        }
    }

    /// Conversion's square, its rows (the `rows` pass, a workgroup a
    /// 256-cell piece of a row with its margins): each species' trail summed
    /// over the 2r+1 cells of the row round every cell, against the CPU's
    /// sums in f64. The twin test sees a piece's edge only where a walker
    /// converts near one, so a tile loaded without its right margin passed
    /// it (the species report's planted bugs, round 3).
    #[test]
    fn the_squares_rows_sum_every_cell_of_the_row() {
        let Some((device, queue)) = test_gpu() else { return };
        let shared = Shared::new(&device);
        let r = 8;
        let s = Settings { species: 2, convert: 1.2, convert_r: r as u32, ..small(3) };
        let mut e = Engine::new(&device, &queue, &shared, &s, SCALE);
        let mut cpu = e.dish.clone();
        for _ in 0..40 {
            cpu.tick();
        }
        // A fresh engine: no deposit counted, so the rows hold the trail
        // alone.
        e.inject(&cpu);
        e.write_params(&s, Hand::default(), 1);
        let (w, h) = (cpu.w, cpu.h);
        assert!(w > 256 + r, "a row in more than one piece");
        e.run(&shared, &[(ROWS, (w as u32).div_ceil(256) * h as u32)]);
        let side = (e.done % 2) as usize;
        let got = floats(&read_back(&device, &queue, &e.species[1 - side]));
        let (mut off, mut worst, mut edge) = (0, 0f64, 0f64);
        for (sp, f) in cpu.species_trail.iter().enumerate() {
            for y in 0..h {
                for x in 0..w {
                    let want: f64 = (x.saturating_sub(r)..(x + r + 1).min(w)).map(|xx| f[y * w + xx] as f64).sum();
                    let d = (got[sp * w * h + y * w + x] as f64 - want).abs() / want.abs().max(1.0);
                    worst = worst.max(d);
                    if x % 256 + r >= 256 {
                        edge = edge.max(want);
                    }
                    off += usize::from(d > 1e-5);
                }
            }
        }
        eprintln!("rows of {w} cells, radius {r}: {off} sums off by more than 1e-5, worst {worst:.1e}; trail summed by the cells beside a piece's edge up to {edge:.1}");
        assert!(edge > 1.0, "trail at the pieces' edges");
        assert!(off == 0, "{off} row sums off, worst {worst:.1e}");
    }

    /// **The life cycle's twin test** (2026-11-11, the tiers' Sporer,
    /// Sleeper and Swarmer on the GPU): each dish warmed on the CPU, struck
    /// on the CPU part way where the walkers are thickest (dead walkers, a
    /// wound, alarm), then struck on both from the same state (GPU
    /// `Engine::strike` against `BigDish::strike`, compared before any
    /// tick) and four ticks on both (the later ones meet what the earlier
    /// left in the counts and claims). Compared: every walker, the trail, scent, species and alarm fields,
    /// every slot's life state, timer, energy, age and sick counter, each
    /// cell's count, the grass, the germ and the crystals' charge.
    /// Strengthened after the first round of planted bugs (13 of 60
    /// missed): four ticks, not two; the grass, the germ and a tight charge
    /// check; the strike's counts, lives and species fields checked before
    /// the tick re-injects them; a wall beside the strike on the crystal
    /// dish; a burst that lands all at once. Then (2026-11-12, after the
    /// tiers thread's PR #85) the three tiers as `Tier::settings` sets them,
    /// dormant walkers that die asleep (`sleep_death`), and every field
    /// within 1e-6 (the trail at 1e-4 missed a planted bug in round 2).
    #[test]
    fn the_life_cycle_matches_the_cpu_for_four_ticks() {
        const TICKS: u32 = 4;
        let Some((device, queue)) = test_gpu() else { return };
        let shared = Shared::new(&device);
        let with = |mut s: Settings, keys: &[(&str, &str)]| {
            for (k, v) in keys {
                set(&mut s, k, v).unwrap();
            }
            s
        };
        // Rates for a test: spores that fly 20 ticks and rest 10, walkers
        // that doze at 5% a tick (big.rs `life_cycle_threads_change_nothing`).
        let spores: &[(&str, &str)] = &[("spore_rate", "0.01"), ("spore_scent", "-1"), ("spore_steps", "20"), ("spore_reach", "8"), ("spore_land", "1"), ("spore_deposit", "0.2"), ("spore_rest", "10")];
        let sleep: &[(&str, &str)] = &[("sleep_rate", "0.05"), ("wake_scent", "50"), ("sleep_deposit", "0.3"), ("alarm_keep", "0.95"), ("alarm_relay", "20"), ("wake_alarm", "1"), ("alarm_weight", "2"), ("sleep_rest", "10"), ("wake_steps", "10")];
        let burst: &[(&str, &str)] = &[("spore_burst", "0.5"), ("spore_alarm", "1"), ("spore_steps", "20"), ("spore_reach", "8"), ("alarm_keep", "0.98")];
        let both = [spores, sleep].concat();
        let hungry = |s: Settings| Settings { upkeep: 0.02, starve: 0.2, birth_energy: 0.6, capacity: 3 * s.walkers / 2, ..s };
        let mix = |s: Settings| Settings { mix_share: 0.3, mix_reach: 3.0, mix_angle: 0.5, mix_turn: 2.0, mix_step: 1.5, mix_deposit: 0.5, ..s };
        let tier = |t: Tier, mut s: Settings| {
            t.settings(&mut s).unwrap();
            s
        };
        let crystals = Settings { piles: 23, rect: true, crystal: 512.0, ..small(0) };
        // Dormant walkers that die asleep, at 2% a tick so four ticks see it.
        let dying = [sleep, &[("sleep_death", "0.02")]].concat();
        let dishes = [
            ("spores, one a cell", with(small(0), spores)),
            ("spores, no limit", with(small(3), spores)),
            ("a burst of spores after a strike", with(small(0), burst)),
            // The t = 30 strike's burst lands at once in the first compared
            // tick (fruit at 30, ten ticks of flight, no landing on scent),
            // two species with their own crowd limits, so landings meet in
            // a cell and species 1's ignore the limit.
            ("a burst landing at once, two species with their own crowd limits", with(Settings { species: 2, ..small(0) }, &[("spore_burst", "1"), ("spore_alarm", "1"), ("spore_steps", "10"), ("spore_reach", "8"), ("spore_land", "0"), ("alarm_keep", "0.98"), ("s1.crowd", "0")])),
            ("dormancy, alarm and relays", with(small(0), sleep)),
            ("skittish: alarm only, the states from the strike", with(small(0), &[("alarm_keep", "0.98"), ("alarm_weight", "-10")])),
            ("both, two species with their own crowd limits, mixed", mix(with(Settings { species: 2, ..small(0) }, &[&both[..], &[("s1.crowd", "0"), ("s1.step", "1.5")]].concat()))),
            ("both, growth on crystals", with(hungry(crystals.clone()), &both)),
            ("both, growth on grass", with(hungry(Settings { graze: 10.0, ..small(0) }), &both)),
            ("both, cap, explorers, contagion", with(Settings { cap: 100, explore: 60, explore_spread: 1.0, sick: 60, immune: 60, catch_at: 0.2, seed_at: 0.0, seed_r: 60.0, ..small(3) }, &both)),
            ("both, respawn", with(Settings { respawn: 0.01, ..small(0) }, &both)),
            ("dormancy, dying asleep", with(small(0), &dying)),
            // The dying's counters (explorers age, the sick count down), and
            // with no crowd limit the capped share of a cell they leave to
            // ordinary walkers (round 3 missed the dying counted dormant on
            // the one-a-cell dish, where no walker shares their cell).
            ("dormancy, dying asleep, no limit, cap, explorers, contagion", with(Settings { cap: 100, explore: 60, explore_spread: 1.0, sick: 60, immune: 60, catch_at: 0.2, seed_at: 0.0, seed_r: 60.0, ..small(3) }, &dying)),
            ("both, dying asleep, growth on grass", with(hungry(Settings { graze: 10.0, ..small(0) }), &[&both[..], &[("sleep_death", "0.02")]].concat())),
            ("the Sporer (Tier::settings)", tier(Tier::Sporer, small(0))),
            ("the Sleeper", tier(Tier::Sleeper, small(0))),
            ("the Sleeper, growth on crystals", tier(Tier::Sleeper, hungry(crystals.clone()))),
            ("the Swarmer", tier(Tier::Swarmer, small(0))),
        ];
        let u32s = |b: &wgpu::Buffer, n: usize| -> Vec<u32> { read_back(&device, &queue, b).chunks_exact(4).take(n).map(|c| u32::from_le_bytes(c.try_into().unwrap())).collect() };
        for (name, s) in dishes {
            let mut e = Engine::new(&device, &queue, &shared, &s, SCALE);
            let mut cpu = e.dish.clone();
            let k = cpu.p.scale as f32;
            // The strike's place: the middle of the living walkers.
            let middle = |d: &BigDish| {
                let (mut x, mut y, mut m) = (0.0, 0.0, 0.0);
                for (j, &(wx, wy, _)) in d.walkers.iter().enumerate() {
                    if d.lives(j) {
                        (x, y, m) = (x + wx, y + wy, m + 1.0);
                    }
                }
                (x / m / k, y / m / k)
            };
            for t in 0..40 {
                if t == 30 {
                    let (x, y) = middle(&cpu);
                    cpu.strike(x, y, 6.0, 50.0, 20.0);
                }
                cpu.tick();
            }
            assert!(!cpu.life.is_empty(), "{name}: life states on");
            // The strike on both, from the same state, on an ordinary
            // walker (so it kills one at least).
            let j = (0..cpu.walkers.len()).find(|&j| cpu.lives(j) && cpu.life[j] == 0).expect("an ordinary walker");
            let (x, y) = (cpu.walkers[j].0 / k, cpu.walkers[j].1 / k);
            // A wall beside it on the world dish: the strike's alarm must
            // stay off the wall.
            if s.crystal > 0.0 {
                cpu.build_wall(x - 6.0, y + 3.0, x + 6.0, y + 3.0, 1.0);
            }
            e.inject(&cpu);
            let killed = cpu.strike(x, y, 4.0, 30.0, 12.0);
            e.strike(&shared, &s, x, y, 4.0, 30.0, 12.0);
            let side = (e.done % 2) as usize;
            let struck_trail = l1(&floats(&read_back(&device, &queue, &e.trail[side])), &cpu.trail);
            let struck_alarm = if cpu.alarm.is_empty() { 0.0 } else { l1(&e.alarm_of(), &cpu.alarm) };
            let struck_life = e.life_of().0.iter().zip(&cpu.life).filter(|(a, b)| a != b).count();
            let gpu_killed = e.struck();
            // What the next tick's inject would hide: the strike's counts,
            // lives and each species' trail.
            let struck_occ = u32s(&e.occ, cpu.agar.len()).iter().zip(cpu.occupancy()).filter(|(a, b)| **a != **b as u32).count();
            let struck_lives = if cpu.energy.is_empty() { 0 } else { floats(&read_back(&device, &queue, &e.energy)).iter().take(cpu.walkers.len()).enumerate().filter(|&(j, v)| (*v >= 0.0) != cpu.lives(j)).count() };
            let struck_species = if e.nsp > 0 { l1(&floats(&read_back(&device, &queue, &e.species[side])), &cpu.species_trail.iter().flatten().copied().collect::<Vec<f32>>()) } else { 0.0 };
            assert!(struck_occ == 0 && struck_lives == 0 && struck_species <= 1e-6, "{name}: after the strike {struck_occ} counts and {struck_lives} lives off, species L1 {struck_species:.1e}");
            assert!(killed > 0 && gpu_killed as usize == killed && struck_life == 0, "{name}: the strike killed {gpu_killed} on the GPU, {killed} on the CPU; {struck_life} states off");
            assert!(struck_trail <= 1e-6 && struck_alarm <= 1e-6, "{name}: after the strike trail L1 {struck_trail:.1e}, alarm L1 {struck_alarm:.1e}");
            // Two ticks on both (`twin` injects the CPU's state again, the
            // strike's included).
            let n = cpu.walkers.len();
            let before = cpu.life_counts();
            let (moved, off, lt, ls) = twin(&mut e, &shared, &s, &mut cpu, TICKS);
            let side = (e.done % 2) as usize;
            let (life, timer) = e.life_of();
            let life_off = life.iter().zip(&cpu.life).filter(|(a, b)| a != b).count();
            let timer_off = timer.iter().zip(&cpu.life_t).enumerate().filter(|&(j, (a, b))| cpu.lives(j) && a != b).count();
            let la = if cpu.alarm.is_empty() { 0.0 } else { l1(&e.alarm_of(), &cpu.alarm) };
            let lsp = if e.nsp > 0 { l1(&floats(&read_back(&device, &queue, &e.species[side])), &cpu.species_trail.iter().flatten().copied().collect::<Vec<f32>>()) } else { 0.0 };
            let occ = u32s(&e.occ, cpu.agar.len());
            let occ_off = occ.iter().zip(cpu.occupancy()).filter(|(a, b)| **a != **b as u32).count();
            // Energy in living slots only: a struck walker's slot keeps its
            // energy on the CPU (unread until a birth halves its parent's
            // into it) and holds -1 on the GPU.
            let living = |v: &[f32]| -> Vec<f32> { (0..n).filter(|&j| cpu.lives(j)).map(|j| v[j]).collect() };
            let le = if cpu.energy.is_empty() { 0.0 } else { l1(&living(&floats(&read_back(&device, &queue, &e.energy))), &living(&cpu.energy)) };
            let alarm_sum: f64 = cpu.alarm.iter().map(|&v| v as f64).sum();
            let lives_off = if cpu.energy.is_empty() { 0 } else { floats(&read_back(&device, &queue, &e.energy)).iter().take(n).enumerate().filter(|&(j, v)| (*v >= 0.0) != cpu.lives(j)).count() };
            let lc = if cpu.charge.is_empty() { 0.0 } else { l1(&floats(&read_back(&device, &queue, &e.charge)), &cpu.charge) };
            let lg = if cpu.grass.is_empty() { 0.0 } else { l1(&floats(&read_back(&device, &queue, &e.grass)), &cpu.grass) };
            let lgerm = if cpu.germ.is_empty() { 0.0 } else { l1(&floats(&read_back(&device, &queue, &e.germ[side]))[..cpu.germ.len()], &cpu.germ) };
            let ages_off = if cpu.age.is_empty() { 0 } else { u32s(&e.age, n).iter().zip(&cpu.age).enumerate().filter(|&(j, (g, c))| cpu.lives(j) && *g != *c as u32).count() };
            let sick_off = if cpu.sick.is_empty() { 0 } else { u32s(&e.sick, n).iter().zip(&cpu.sick).enumerate().filter(|&(j, (g, c))| cpu.lives(j) && *g != *c as u32).count() };
            let after = cpu.life_counts();
            eprintln!(
                "{name}: {n} slots, states before {before:?} (walkers, spores, dormant, dead), after {after:?}; struck {killed}; {moved} moved, {off} off; trail L1 {lt:.1e}, scent L1 {ls:.1e}, species L1 {lsp:.1e}, alarm L1 {la:.1e} (alarm in all {alarm_sum:.0}); {life_off} states and {timer_off} timers off; {occ_off} counts off; energy L1 {le:.1e}, {lives_off} lives off, charge L1 {lc:.1e}, grass L1 {lg:.1e}, germ L1 {lgerm:.1e}; {ages_off} ages and {sick_off} sick counters off"
            );
            assert!(off * 1000 <= n, "{name}: {off} of {n} walkers off");
            assert!(lt <= 1e-6 && ls <= 1e-6 && lsp <= 1e-6 && la <= 1e-6, "{name}: trail {lt:.1e}, scent {ls:.1e}, species {lsp:.1e}, alarm {la:.1e}");
            assert!(life_off * 1000 <= n && timer_off * 1000 <= n && occ_off * 1000 <= n, "{name}: {life_off} states, {timer_off} timers, {occ_off} counts off");
            assert!(le <= 1e-3 && lives_off * 1000 <= n && lc <= 1e-6, "{name}: energy {le:.1e}, {lives_off} lives, charge {lc:.1e}");
            assert!(lg <= 1e-6 && lgerm <= 1e-6, "{name}: grass {lg:.1e}, germ {lgerm:.1e}");
            assert!(ages_off * 1000 <= n && sick_off * 1000 <= n, "{name}: {ages_off} ages, {sick_off} sick counters off");
            if s.sleep_death >= 0.01 && s.upkeep == 0.0 {
                assert!(after[3] > before[3], "{name}: dormant walkers die asleep: dead {} then {}", before[3], after[3]);
            }
        }
    }

    /// The plain pipelines (the outside rules compiled out) give the dish the
    /// full ones give with every rule off, bit for bit. The full ones are
    /// forced by keeping blocked headings with a reset every tick, which
    /// keeps none.
    #[test]
    fn compiling_the_outside_rules_out_changes_nothing() {
        let Some((device, queue)) = test_gpu() else { return };
        let shared = Shared::new(&device);
        for base in [small(0), small(4)] {
            let forced = Settings { blocked_keep: true, blocked_reset: 1.0, ..base.clone() };
            let mut seen = Vec::new();
            for s in [&base, &forced] {
                let mut e = Engine::new(&device, &queue, &shared, s, SCALE);
                for _ in 0..10 {
                    run(&mut e, &shared, s, s.ticks, Hand::default());
                }
                assert_eq!(e.outside, s.blocked_keep, "the pipelines picked");
                let w: Vec<u32> = walkers_of(&e).iter().flat_map(|&(x, y, h)| [x.to_bits(), y.to_bits(), h.to_bits()]).collect();
                seen.push((e.picture(), w));
            }
            assert!(seen[0] == seen[1], "the plain and full pipelines differ");
        }
    }

    /// Fed scent: a pile the tube reaches lays `fed_scent` of its scent from
    /// the next tick, on the GPU as on the CPU. Two ticks from the state
    /// just before a pile is first reached on the CPU.
    #[test]
    fn a_reached_pile_quietens_on_the_gpu_as_on_the_cpu() {
        let Some((device, queue)) = test_gpu() else { return };
        let shared = Shared::new(&device);
        let s = small(0);
        let mut e = Engine::new(&device, &queue, &shared, &s, SCALE);
        let mut cpu = e.dish.clone();
        let mut ticks = 0;
        loop {
            let mut next = cpu.clone();
            next.tick();
            if next.reached != cpu.reached {
                break;
            }
            cpu = next;
            ticks += 1;
            assert!(ticks < 3000, "no pile reached in 100 s");
        }
        let was = cpu.reached.clone();
        let (moved, off, lt, ls) = twin(&mut e, &shared, &s, &mut cpu, 2);
        let gpu: Vec<bool> = read_back(&device, &queue, &e.reached).chunks_exact(4).take(cpu.reached.len()).map(|c| u32::from_le_bytes(c.try_into().unwrap()) != 0).collect();
        eprintln!("a pile reached after {} ticks: {was:?} to {:?}; GPU {gpu:?}; {moved} moved, {off} off, trail L1 {lt:.1e}, scent L1 {ls:.1e}", ticks + 1, cpu.reached);
        assert_eq!(gpu, cpu.reached, "the GPU marked the pile the CPU did");
        assert!(off * 1000 <= cpu.walkers.len() && lt <= 1e-4 && ls <= 1e-6, "{off} off, trail L1 {lt:.2e}, scent L1 {ls:.2e}");
    }

    /// The same seed gives the same dish and picture twice, bit for bit,
    /// with every random thing on: respawn, the pull, the wipe, the soft
    /// curve's histogram.
    #[test]
    fn the_same_seed_gives_the_same_picture_twice() {
        let Some((device, queue)) = test_gpu() else { return };
        let shared = Shared::new(&device);
        // Key 5, and the looks with contagion seeded by the mouse and by
        // the clock (`seed_at` 0.1 s).
        let looks = Settings { trail_scale: 0.02, turn_by: [-20.0, 1.0], floor: 0.25, wander: 0.01, cap: 100, sick: 60, immune: 120, seed_at: 0.1, ..small(9) };
        // The outside rules: cyclic species with explorers (their stage
        // started again by respawn and the pull), the chance turn, size
        // spread and area sensors; and two species of grazing droplets
        // keeping their heading and converting.
        let outside = Settings { respawn: 0.01, explore: 60, explore_spread: 1.0, chance: 2.0, size_spread: 0.5, sensor_r: 1, ..small(16) };
        let droplets = Settings { blocked_keep: true, blocked_reset: 0.03, convert: 1.2, ..small(21) };
        for s in [small(4), looks, outside, droplets] {
            let hand = |f: u32| Hand { at: Some((200.0 + 4.0 * f as f32, 150.0)), pull: f < 10, wipe: f >= 10, infect: f == 3 };
            let mut seen = Vec::new();
            for _ in 0..2 {
                let mut e = Engine::new(&device, &queue, &shared, &s, SCALE);
                for f in 0..20 {
                    run(&mut e, &shared, &s, s.ticks, hand(f));
                }
                let mut counters: Vec<u8> = read_back(&device, &queue, &e.sick);
                counters.extend(read_back(&device, &queue, &e.age));
                counters.extend(read_back(&device, &queue, &e.spec));
                counters.extend(read_back(&device, &queue, &e.grass));
                seen.push((e.picture(), walkers_of(&e), counters));
            }
            let bits = |w: &[(f32, f32, f32)]| w.iter().flat_map(|&(x, y, h)| [x.to_bits(), y.to_bits(), h.to_bits()]).collect::<Vec<u32>>();
            assert!(seen[0].0 == seen[1].0, "the pictures differ");
            assert_eq!(bits(&seen[0].1), bits(&seen[1].1), "the walkers differ");
            assert!(seen[0].2 == seen[1].2, "the sick, ages, species or grass differ");
        }
    }

    /// One walker a cell holds on the GPU through its claims, jumps and
    /// pulls: the count a cell kept on the GPU is the walkers standing
    /// there, none above one, and every walker is on agar.
    #[test]
    fn the_crowd_limit_holds_through_jumps() {
        let Some((device, queue)) = test_gpu() else { return };
        let shared = Shared::new(&device);
        let s = Settings { respawn: 0.01, ..small(0) };
        let mut e = Engine::new(&device, &queue, &shared, &s, SCALE);
        assert!(e.crowd_ok);
        for f in 0..30 {
            let hand = Hand { at: Some((256.0, 160.0)), pull: f % 2 == 0, wipe: false, infect: false };
            run(&mut e, &shared, &s, s.ticks, hand);
        }
        let walkers = walkers_of(&e);
        assert_eq!(walkers.len(), e.n as usize);
        let occ: Vec<u32> = read_back(&device, &queue, &e.occ).chunks_exact(4).map(|c| u32::from_le_bytes(c.try_into().unwrap())).collect();
        let mut counted = vec![0u32; e.cells as usize];
        for &(x, y, _) in &walkers {
            assert!(x >= 0.0 && y >= 0.0 && (x as usize) < e.dish.w && (y as usize) < e.dish.h, "a walker outside the dish at {x}, {y}");
            let i = y as usize * e.dish.w + x as usize;
            assert!(e.dish.agar[i], "a walker off the agar at {x}, {y}");
            counted[i] += 1;
        }
        assert_eq!(occ, counted, "the GPU's count a cell is the walkers there");
        assert!(counted.iter().all(|&c| c <= 1), "two walkers in a cell");
    }

    /// The soft curve's reference is the shown field's p99.9 to within a
    /// bin (2^(1/8)), on a dish whose trail runs far above 2^16.
    #[test]
    fn the_soft_curves_reference_is_the_p999() {
        let Some((device, queue)) = test_gpu() else { return };
        let shared = Shared::new(&device);
        let s = Settings { laid: 2000.0, ..small(2) };
        let s = Settings { curve: Curve::Soft, ..s };
        let mut e = Engine::new(&device, &queue, &shared, &s, SCALE);
        for _ in 0..30 {
            run(&mut e, &shared, &s, s.ticks, Hand::default());
        }
        let top = f32::from_bits(u32::from_le_bytes(read_back(&device, &queue, &e.state)[4..8].try_into().unwrap()));
        let mut lit: Vec<f32> = floats(&read_back(&device, &queue, &e.trail[(e.done % 2) as usize])).into_iter().filter(|&v| v > 0.0).collect();
        lit.sort_by(|a, b| a.partial_cmp(b).unwrap());
        let p999 = lit[lit.len() - (lit.len() / 1000).max(1)];
        eprintln!("top {top}, p99.9 {p999}, ratio {:.3}", top / p999);
        assert!(p999 > 65_536.0, "the dish should pass the old histogram's top: {p999}");
        assert!(top >= p999 * 0.999 && top <= p999 * 1.1, "top {top} against p99.9 {p999}");
    }

    #[test]
    fn controls_parse() {
        let mut s = preset(3);
        for (k, v) in [("walkers", "2000000"), ("respawn", "0.03"), ("curve", "game"), ("colour", "amber"), ("field", "walkers"), ("start", "blob"), ("crowd", "1")] {
            set(&mut s, k, v).unwrap();
        }
        assert_eq!((s.walkers, s.respawn, s.curve, s.colour, s.field, s.start, s.crowd), (2_000_000, 0.03, Curve::Game, Colour::Amber, Field::Walkers, Start::Home, true));
        // The looks' keys, with the CPU's names too (examples/bigdish.rs).
        for (k, v) in [("start", "disc"), ("haze", "0.3"), ("sense_scale", "0.02"), ("turn_mod", "-20:1"), ("step_by", "1"), ("deposit_cap", "100"), ("sick_steps", "120"), ("catch", "5")] {
            set(&mut s, k, v).unwrap();
        }
        assert_eq!((s.start, s.haze, s.trail_scale, s.turn_by, s.step_by, s.cap, s.sick, s.catch_at), (Start::Disc, 0.3, 0.02, [-20.0, 1.0], [1.0, 1.0], 100, 120, 5.0));
        assert!(set(&mut s, "turn_mod", "-20:x").is_err());
        // The outside rules' keys, by the CPU's names (examples/bigdish.rs).
        for (k, v) in [("species", "3"), ("species_cycle", "1"), ("species_avoid", "0.5"), ("explore_steps", "900"), ("explore_spread", "1"), ("blocked_deposit", "1"), ("blocked_keep", "on"), ("chance_exp", "4"), ("size_spread", "0.6"), ("sensor_r", "1")] {
            set(&mut s, k, v).unwrap();
        }
        assert_eq!((s.species, s.cycle, s.avoid, s.explore, s.explore_spread, s.blocked_lay, s.blocked_keep, s.chance, s.size_spread, s.sensor_r), (3, true, 0.5, 900, 1.0, 1.0, true, 4.0, 0.6, 1));
        set(&mut s, "species", "9").unwrap();
        assert_eq!(s.fields(), 4);
        for (k, v) in [("species_convert", "1.5"), ("graze_weight", "10"), ("graze_eat", "0.1"), ("graze_regrow", "0.004"), ("species_boost", "1.5")] {
            set(&mut s, k, v).unwrap();
        }
        assert_eq!((s.convert, s.graze, s.graze_eat, s.graze_regrow, s.boost), (1.5, 10.0, 0.1, 0.004, 1.5));
        assert!(set(&mut s, "nonsense", "1").is_err());
        // Switches and names are strict: `crowd=false` is off, a misspelt
        // name is an error, and no frame has 0 ticks.
        set(&mut s, "crowd", "false").unwrap();
        assert!(!s.crowd);
        for (k, v) in [("crowd", "maybe"), ("curve", "Soft"), ("colour", "red"), ("start", "packed")] {
            assert!(set(&mut s, k, v).is_err(), "{k}={v}");
        }
        set(&mut s, "ticks", "0").unwrap();
        assert_eq!(s.ticks, 1);
    }
    /// The twin test for growth (Larder World W0, 2026-11-11): one GPU tick
    /// from a warmed CPU dish against one `BigDish::tick`, with births and
    /// starving in the tick. Every slot's position, energy and life are
    /// compared, and its age, species and sickness where those are on.
    #[test]
    fn growth_matches_the_cpu_for_one_tick() {
        let Some((device, queue)) = test_gpu() else { return };
        let shared = Shared::new(&device);
        // A fifth of the preset's walkers to start, so the mould is still
        // growing at the twin tick. One a cell gets slots for three times
        // that (it needs them at most half the agar: 512 x 320 holds about
        // 128,000 cells); with no limit nothing slows a birth but the slots,
        // so it gets twenty times (it doubles about every 10 ticks of the
        // 40-tick warm-up; three times was full by then).
        let g = |s: Settings| Settings { upkeep: 0.01, walkers: s.walkers / 5, capacity: if s.crowd { 3 } else { 20 } * s.walkers / 5, graze: 10.0, ..s };
        // Hungry: grass eaten faster than it pays (0.03 upkeep against at
        // most 0.05 eaten), so some starve while others breed.
        let hungry = |s: Settings| Settings { upkeep: 0.03, starve: 0.2, birth_energy: 0.6, graze_regrow: 0.0, ..g(s) };
        let dishes = [
            ("growing, one a cell", g(small(0))),
            ("growing, no limit", g(small(2))),
            ("hungry, one a cell", hungry(small(0))),
            ("hungry, no limit, Jenson's", hungry(small(3))),
            ("hungry, species converting, explorers, contagion, respawn", Settings { species: 2, convert: 1.2, explore: 60, explore_spread: 1.0, sick: 60, immune: 60, catch_at: 0.2, seed_at: 0.0, seed_r: 60.0, respawn: 0.01, ..hungry(small(0)) }),
            ("hungry, cap, grazing off", Settings { cap: 100, graze: 0.0, ..hungry(small(3)) }),
            // Every other walker set starving and the rest set to breed
            // (below), with room in the slots: births land in cells freed
            // by deaths in the same tick, some of them claimed by the
            // walker's own move just before it died.
            // With species, explorers and contagion, so a child landing in
            // the slot of a walker that died keeps no age, sickness or
            // species of the dead one's.
            ("hungry, dying beside breeding", Settings { starve: 1.0, capacity: 60_000, species: 2, explore: 60, explore_spread: 1.0, sick: 60, immune: 60, catch_at: 0.2, seed_at: 0.0, seed_r: 60.0, ..hungry(small(0)) }),
            // The species settings (2026-11-11): children keep their
            // parent's kind, and a parent breeds by its species' crowd
            // limit (species 1 has none); species 1 does not graze, which
            // growth on grass overrides (every walker eats).
            ("hungry, mixed walkers, crowd limits by species", {
                let mut s = Settings { species: 2, zones: true, mix_share: 0.3, mix_reach: 2.0, mix_step: 1.5, mix_deposit: 0.5, ..hungry(small(0)) };
                for (k, v) in [("s1.crowd", "0"), ("s1.graze", "0"), ("s0.keep", "0.93")] {
                    set(&mut s, k, v).unwrap();
                }
                s
            }),
        ];
        for (name, s) in dishes {
            let mut e = Engine::new(&device, &queue, &shared, &s, SCALE);
            assert!(e.grows, "{name}: growth on");
            let mut cpu = e.dish.clone();
            // The GPU runs a few ticks of its own first, so a per-cell count
            // the tick fails to clear reaches the twin tick (the inject
            // writes the walkers and fields, not the counts). The grazing
            // counts were cleared only with `graze_w` until 2026-11-11, and
            // with grazing's weight 0 (the last dish but one) the grass L1 was
            // 0.12 against the bound of 1e-4; cleared, it is 0.
            run(&mut e, &shared, &s, 3, Hand::default());
            for _ in 0..40 {
                cpu.tick();
            }
            // With no limit every walker eats alike and they breed in
            // waves (5,000 became exactly 20,000), so warm up on to the
            // first tick that has births (and starving, when hungry).
            let hungry = name.starts_with("hungry");
            for _ in 0..100 {
                let mut next = cpu.clone();
                next.tick();
                if next.births > cpu.births && (!hungry || next.starved > cpu.starved) {
                    break;
                }
                cpu = next;
            }
            if name.ends_with("dying beside breeding") {
                for k in 0..cpu.walkers.len() {
                    if cpu.alive[k] {
                        cpu.energy[k] = if k % 2 == 0 { 0.0 } else { 1.0 };
                        // The starving set sick too, so a slot freed this
                        // tick holds a sickness a child must not keep (the
                        // sickness seeded at the start had gone by now).
                        if k % 2 == 0 {
                            cpu.sick[k] = 100;
                        }
                    }
                }
            }
            let n = cpu.walkers.len();
            let (born, starved) = (cpu.births, cpu.starved);
            let (moved, off, lt, ls) = twin(&mut e, &shared, &s, &mut cpu, 1);
            let (born, starved) = (cpu.births - born, cpu.starved - starved);
            let energy = floats(&read_back(&device, &queue, &e.energy));
            let want = energies(&cpu);
            let lives_off = energy.iter().zip(&want).filter(|(g, c)| (**g >= 0.0) != (**c >= 0.0)).count();
            let le = l1(&energy.iter().map(|&v| v.max(0.0)).collect::<Vec<_>>(), &want.iter().map(|&v| v.max(0.0)).collect::<Vec<_>>());
            let u32s = |b: &wgpu::Buffer| -> Vec<u32> { read_back(&device, &queue, b).chunks_exact(4).take(n).map(|c| u32::from_le_bytes(c.try_into().unwrap())).collect() };
            let ages_off = if cpu.age.is_empty() { 0 } else { u32s(&e.age).iter().zip(&cpu.age).filter(|(g, c)| **g != **c as u32).count() };
            let specs_off = if cpu.spec.is_empty() { 0 } else { u32s(&e.spec).iter().zip(&cpu.spec).filter(|(g, c)| *g & 0xFF != **c as u32).count() };
            // A child is its parent's kind (`mix_share`, bit 8).
            let kinds_off = if cpu.kind.is_empty() { 0 } else { u32s(&e.spec).iter().zip(&cpu.kind).filter(|(g, c)| *g >> 8 != **c as u32).count() };
            let sick_off = if cpu.sick.is_empty() { 0 } else { u32s(&e.sick).iter().zip(&cpu.sick).filter(|(g, c)| **g != **c as u32).count() };
            let occ: Vec<u32> = read_back(&device, &queue, &e.occ).chunks_exact(4).map(|c| u32::from_le_bytes(c.try_into().unwrap())).collect();
            let occ_off = occ.iter().zip(occupancy(&cpu)).filter(|(g, c)| **g != *c).count();
            let alive = cpu.alive.iter().filter(|&&a| a).count();
            let lg = l1(&floats(&read_back(&device, &queue, &e.grass)), &cpu.grass);
            eprintln!(
                "{name}: {n} slots, {alive} alive, {born} born and {starved} starved in the tick, {moved} moved or turned, {off} off; trail L1 {lt:.1e}, scent L1 {ls:.1e}, energy L1 {le:.1e}, grass L1 {lg:.1e}; {lives_off} lives off, {occ_off} cells' counts off, {ages_off} ages, {specs_off} species and {sick_off} sick counters off"
            );
            assert!(born > 0, "{name}: someone is born in the tick");
            if hungry {
                assert!(starved > 0, "{name}: someone starves in the tick");
            }
            assert!(off * 1000 <= n && lives_off * 1000 <= n && occ_off * 1000 <= n, "{name}: {off} walkers, {lives_off} lives, {occ_off} counts off of {n}");
            assert!(lt <= 1e-4 && ls <= 1e-6 && le <= 1e-4 && lg <= 1e-4, "{name}: trail L1 {lt:.2e}, scent L1 {ls:.2e}, energy L1 {le:.2e}, grass L1 {lg:.2e}");
            assert!((ages_off + specs_off + sick_off) * 1000 <= n, "{name}: {ages_off} ages, {specs_off} species, {sick_off} sick off");
            assert_eq!(kinds_off, 0, "{name}: kinds off");
        }
    }

    /// What the GPU holds of the Larder World after a tick, against the CPU
    /// dish: (crystal charge L1, or infinite if any charge is below 0;
    /// walkers whose crystal take is off by more than 1e-6, wall health L1,
    /// cells whose agar differs, cells whose ground bits differ).
    fn world_off(e: &Engine, cpu: &BigDish) -> (f64, usize, f64, usize, usize) {
        let (device, queue) = (&e.device, &e.queue);
        let wd = world_data(cpu);
        let u32s = |b: &wgpu::Buffer| -> Vec<u32> { read_back(device, queue, b).chunks_exact(4).map(|c| u32::from_le_bytes(c.try_into().unwrap())).collect() };
        let charge = floats(&read_back(device, queue, &e.charge));
        // Eaten charge is floored at 0, as on the CPU; a cell below it is a
        // bug however small (a planted one left -1e-4 of a cell's charge).
        let lc = if charge.iter().any(|&c| c < 0.0) { f64::INFINITY } else { l1(&charge, &wd.charge) };
        let share = floats(&read_back(device, queue, &e.share));
        let took_off = if cpu.took.is_empty() {
            0
        } else {
            (0..cpu.walkers.len()).filter(|&k| cpu.lives(k)).filter(|&k| {
                let (x, y, _) = cpu.walkers[k];
                (share[y as usize * cpu.w + x as usize] - cpu.took[k]).abs() > 1e-6
            }).count()
        };
        // Infinite health (a wall that never breaks) on both sides reads as 0.
        let finite = |v: Vec<f32>| -> Vec<f32> { v.into_iter().map(|h| if h.is_finite() { h } else { 0.0 }).collect() };
        let lh = l1(&finite(floats(&read_back(device, queue, &e.health))), &finite(wd.health));
        let agar_off = u32s(&e.agar).iter().zip(&cpu.agar).filter(|(g, c)| (**g != 0) != **c).count();
        let ground_off = u32s(&e.ground).iter().zip(&wd.ground).filter(|(g, c)| g != c).count();
        (lc, took_off, lh, agar_off, ground_off)
    }

    /// **The Larder World's twin test** (W1b, 2026-11-11): one GPU tick from
    /// a warmed CPU dish against one `BigDish::tick`, with crystals (equal
    /// shares, the flow inside a crystal, regrowing, hexagonal footprints
    /// and their scent by the share left), rock and wet ground, walls the
    /// mould bites and breaks into rubble (leaky and sealed), growth fed by
    /// the crystals, and soil (WM3, 2026-11-12: sand, clay and barren bands,
    /// with one species and two). The charge, each walker's take, the
    /// walls' health, the agar and every cell's ground bits are compared too.
    #[test]
    fn the_world_matches_the_cpu_for_one_tick() {
        let Some((device, queue)) = test_gpu() else { return };
        let shared = Shared::new(&device);
        // The Larder world's dish (23 piles, a rectangle), crystals at 512.
        let w = |s: Settings| Settings { piles: 23, rect: true, crystal: 512.0, crystal_eat: 0.05, ..s };
        // Walls across home's way out, and one that cuts through a crystal
        // (pile 0 sits at 56, 80 in dish cells), built a third of the way
        // through the warm-up; and one across home's crystal and the crowd
        // on it, built just before the tick, so the walkers it moves out
        // crowd its edge; and a cage round a packed start, so the crowd
        // presses on it and several walkers bite one cell at once.
        type Build = fn(&mut BigDish, u32);
        let walls: Build = |cpu, t| {
            if t == 20 {
                cpu.build_wall(70.0, 40.0, 70.0, 120.0, 2.0);
                cpu.build_wall(40.0, 70.0, 60.0, 90.0, 1.5);
            }
        };
        let late: Build = |cpu, t| {
            if t == 57 {
                cpu.build_wall(45.0, 60.0, 70.0, 100.0, 4.0);
            }
        };
        let cage: Build = |cpu, t| {
            if t == 0 {
                let (cx, cy, r) = (56.0f32, 80.0f32, 24.0f32);
                for q in 0..16 {
                    let (a, b) = (q as f32 * std::f32::consts::TAU / 16.0, (q + 1) as f32 * std::f32::consts::TAU / 16.0);
                    cpu.build_wall(cx + r * a.cos(), cy + r * a.sin(), cx + r * b.cos(), cy + r * b.sin(), 2.0);
                }
            }
        };
        // Soil in four bands across the dish, sand, clay, barren and plain,
        // laid before the warm-up (the inject carries it to the GPU).
        let soil: Build = |cpu, t| {
            if t == 0 {
                let w = cpu.w;
                cpu.soil = (0..w * cpu.h).map(|i| [SOIL_SAND, SOIL_CLAY, SOIL_BARREN, 0][(i % w) * 4 / w]).collect();
            }
        };
        let none: Build = |_, _| {};
        let grow = |s: Settings| Settings { upkeep: 0.01, walkers: 5_000, capacity: 15_000, crystal_eat: 0.02, ..s };
        let dishes: [(&str, Settings, Build); 12] = [
            ("crystals", w(small(0)), none),
            // Many walkers to a cell share it, and charge 1 is eaten to the
            // floor, so the share left goes to 0 and the regrowing shows.
            ("crystals at 1, no limit, regrowing", Settings { crystal: 1.0, crystal_regrow: 0.002, ..w(small(2)) }, none),
            // Charge 1 and no flow: cells are eaten to the floor and stay.
            ("crystals at 1, no flow, no limit", Settings { crystal: 1.0, crystal_flow: 0.0, ..w(small(2)) }, none),
            ("rock and wet ground", Settings { rock: 0.35, wet: 0.35, ..w(small(0)) }, none),
            // Health 3 (bites 1): walls break in the warm-up and the tick.
            ("walls that break, leaking scent", Settings { wall_hp: 3.0, ..w(small(0)) }, walls),
            ("walls that never break, sealed", Settings { wall_hp: 0.0, wall_scent: 0.0, ..w(small(2)) }, walls),
            // No limit, so several walkers on a cell bite one wall cell in
            // the same tick, and health 1,000 so the bites add up unbroken.
            ("walls under a crowd, no limit", Settings { wall_hp: 1000.0, ..w(small(2)) }, late),
            ("a cage round a packed start", Settings { wall_hp: 1000.0, walkers: 5_000, start: Start::Home, ..w(small(0)) }, cage),
            // Growth eating the crystals (the GPU thread's W0): the life
            // pass adds each walker's take.
            ("growth on crystals", Settings { wall_hp: 3.0, ..grow(w(small(0))) }, walls),
            // Soil: the trail's keep on sand and clay, barren upkeep and no
            // births there; then with two species (each field x the keep).
            ("soil bands, growth on crystals", grow(w(small(0))), soil),
            // One a cell: with no limit a child lands on a crystal cell
            // others share, and the take check (a cell's share against each
            // living walker's take) counts the newborn, which took nothing.
            ("soil bands, two species", Settings { species: 2, ..grow(w(small(0))) }, soil),
            // One body (2026-11-12): energy evens out between neighbouring
            // cells (`esum`, `egain`) and hungry walkers weigh the scent
            // harder, at the one-body thread's lead (0.05, 8), over walls
            // so some walkers are blocked and crowded.
            ("one body: food shared, hunger", Settings { energy_flow: 0.05, hunger_scent: 8.0, hunger_at: 1.0, wall_hp: 3.0, ..grow(w(small(0))) }, walls),
        ];
        for (name, s, build) in dishes {
            let mut e = Engine::new(&device, &queue, &shared, &s, SCALE);
            let mut cpu = e.dish.clone();
            for t in 0..60 {
                build(&mut cpu, t);
                cpu.tick();
            }
            // Soil: every walker fed to breed (birth energy 1), so children
            // aim at barren cells, where none may land, as at the rest. (Not
            // fed, no child was born in the tick, and a planted bug that let
            // them land on barren ground passed.)
            if !cpu.soil.is_empty() {
                for k in 0..cpu.walkers.len() {
                    if cpu.lives(k) {
                        cpu.energy[k] = 2.0;
                    }
                }
            }
            let n = cpu.walkers.len();
            let (broken, bitten, eaten, births) = (cpu.broken, cpu.wall_hits, cpu.eaten, cpu.births);
            let (moved, off, lt, ls) = twin(&mut e, &shared, &s, &mut cpu, 1);
            let (broken, bitten, eaten, births) = (cpu.broken - broken, cpu.wall_hits - bitten, cpu.eaten - eaten, cpu.births - births);
            let (lc, took_off, lh, agar_off, ground_off) = world_off(&e, &cpu);
            let (_, hits, gpu_broken) = e.world_counts();
            // Growth: what the walkers ate turned into energy; and no grass
            // where the CPU has none (crystals on, grazing off).
            let le = if e.grows { l1(&floats(&read_back(&device, &queue, &e.energy)).iter().map(|&v| v.max(0.0)).collect::<Vec<_>>(), &energies(&cpu).iter().map(|&v| v.max(0.0)).collect::<Vec<_>>()) } else { 0.0 };
            let grass = floats(&read_back(&device, &queue, &e.grass));
            let lg = if cpu.grass.is_empty() { grass.iter().map(|&g| g.abs() as f64).sum() } else { l1(&grass, &cpu.grass) };
            eprintln!(
                "{name}: {n} walkers, {moved} moved or turned, {off} off; trail L1 {lt:.1e}, scent L1 {ls:.1e}, charge L1 {lc:.1e}, health L1 {lh:.1e}, energy L1 {le:.1e}, grass {lg:.1e}; {eaten:.1} eaten; {bitten} bites and {broken} cells broken on the CPU, {hits} and {gpu_broken} on the GPU; {took_off} takes, {agar_off} agar and {ground_off} ground cells off"
            );
            assert!(le <= 1e-5 && lg <= 1e-6, "{name}: energy L1 {le:.2e}, grass {lg:.2e}");
            assert!(eaten > 0.0, "{name}: the mould eats in the tick");
            // One body's sums are cleared for the next tick (a 1-tick twin
            // cannot see a stale sum otherwise).
            if s.energy_flow > 0.0 {
                assert!(e.energy_flows, "{name}: one body's passes run");
                let left = read_back(&device, &queue, &e.body).iter().filter(|&&b| b != 0).count();
                assert_eq!(left, 0, "{name}: one body's sums left after the tick");
            }
            // Walls that never break take no bites on the CPU (it keeps no
            // health for them); the GPU counts the bites on their infinite
            // health, which changes nothing.
            if !cpu.wall.is_empty() && s.wall_hp > 0.0 {
                assert!(bitten > 0, "{name}: walkers bite the walls in the tick");
                // Health 3 breaks in the warm-up; 1,000 holds.
                assert!(s.wall_hp > 10.0 || cpu.broken > 0, "{name}: a wall broke in the warm-up");
                assert_eq!((hits as u64, gpu_broken as usize), (bitten, broken), "{name}: bites and cells broken");
            }
            if s.wet > 0.0 {
                assert!(!cpu.wet.is_empty() && cpu.walkers.iter().any(|&(x, y, _)| cpu.wet[y as usize * cpu.w + x as usize]), "{name}: walkers on wet ground");
            }
            if !cpu.soil.is_empty() {
                let on = |k: u8| (0..n).filter(|&q| cpu.lives(q) && cpu.soil[cpu.walkers[q].1 as usize * cpu.w + cpu.walkers[q].0 as usize] == k).count();
                let kinds = [on(SOIL_SAND), on(SOIL_CLAY), on(SOIL_BARREN), on(0)];
                eprintln!("{name}: living walkers on sand, clay, barren, plain {kinds:?}; {births} born in the tick");
                assert!(kinds.iter().all(|&c| c > 0) && births > 0, "{name}: walkers on every kind of soil {kinds:?}, {births} born");
                assert!(e.ground_world & WORLD_SOIL != 0, "{name}: soil on");
            }
            assert!(off * 1000 <= n, "{name}: {off} of {n} walkers off");
            assert!(lt <= 1e-4 && ls <= 1e-6, "{name}: trail L1 {lt:.2e}, scent L1 {ls:.2e}");
            assert!(lc <= 1e-6 && lh <= 1e-6, "{name}: charge L1 {lc:.2e}, health L1 {lh:.2e}");
            assert!(took_off * 1000 <= n && agar_off == 0 && ground_off == 0, "{name}: {took_off} takes, {agar_off} agar, {ground_off} ground off");
        }
    }

    /// **The squad's twin test** (Larder World W2a, 2026-11-11): one frame
    /// of one tick with units and shots (`slime_mould::mould::squad`) from a
    /// warmed CPU world with growth, crystals and walls, on the GPU against
    /// `squad::frame` on the CPU. Every output must agree exactly (each
    /// shot's hit and kills; each unit's walkers touching, the block it aims
    /// at, and the charge it took), and the world after the frame as in the
    /// world twin test: walkers, which slots live, trail, scent and charge.
    /// The frame is chosen so that every hook acts: shots that hit slime and
    /// kill, one that a wall stops, units in the mould and on crystals (two
    /// on one crystal, so the harvest's order shows), and smell.
    /// **The squad frame at its cap** (2026-11-12, for the buildings:
    /// 40 soldiers, the larder and up to 40 buildings are units). Before,
    /// the frame held 64 units and the GPU's units pass ran one workgroup
    /// of 64, so a unit past 64 was dropped silently. With `UNITS_MAX` units
    /// spread over the mould and the crystals, every unit's outputs match
    /// the CPU's exactly, and the last ones see, touch and harvest.
    #[test]
    fn the_squad_frame_holds_its_cap_on_the_gpu() {
        use slime_mould::mould::squad::{self, Frame, UnitIn};
        let Some((device, queue)) = test_gpu() else { return };
        let shared = Shared::new(&device);
        let s = Settings { piles: 23, rect: true, crystal: 512.0, crystal_eat: 0.02, upkeep: 0.01, walkers: 5_000, capacity: 15_000, ..small(0) };
        let mut e = Engine::new(&device, &queue, &shared, &s, SCALE);
        let mut cpu = e.dish.clone();
        for _ in 0..60 {
            cpu.tick();
        }
        let (w, h) = (cpu.w, cpu.h);
        let k = SCALE as f32;
        let thick = (0..w * h).max_by(|&a, &b| cpu.trail[a].total_cmp(&cpu.trail[b])).unwrap();
        let t = ((thick % w) as f32 + 0.5, (thick / w) as f32 + 0.5);
        let n = squad::UNITS_MAX;
        // Every third unit on a crystal (harvesting), the rest round the
        // thickest mould; the last on the mould itself.
        let units: Vec<UnitIn> = (0..n)
            .map(|i| {
                let at = if i == n - 1 {
                    t
                } else if i % 3 == 0 {
                    let p = cpu.piles[(i / 3) % cpu.piles.len()];
                    (p.0 * k, p.1 * k)
                } else {
                    let a = i as f32 * 2.4;
                    (t.0 + (6.0 + i as f32 * 0.3) * a.cos(), t.1 + (6.0 + i as f32 * 0.3) * a.sin())
                };
                UnitIn { x: at.0.clamp(0.0, w as f32 - 1.0), y: at.1.clamp(0.0, h as f32 - 1.0), touch: 3.0, sight: 30.0, take_r: 3.0, take: if i % 3 == 0 { 50.0 } else { 0.0 }, smell_r: 2.0, smell: 0.5 }
            })
            .collect();
        let f = Frame { units, shots: Vec::new(), slime: squad::slime_trail(s.knee), crater: 0 };
        e.inject(&cpu);
        e.set_squad(&f);
        run(&mut e, &shared, &s, 1, Hand::default());
        let cpu_out = squad::frame(&mut cpu, &f, 1);
        let gpu_out = e.squad_out();
        assert_eq!(cpu_out.units.len(), n, "the CPU reads every unit");
        let past: Vec<_> = cpu_out.units[64..].iter().collect();
        eprintln!("squad cap: {n} units; past 64, {} touch, {} aim, {} took", past.iter().filter(|u| u.touching > 0).count(), past.iter().filter(|u| u.target.is_some()).count(), past.iter().filter(|u| u.took > 0.0).count());
        assert_eq!(gpu_out, cpu_out, "every unit's outputs, the last 32 too");
        assert!(cpu_out.units[n - 1].touching > 0 && past.iter().any(|u| u.took > 0.0) && past.iter().all(|u| u.target.is_some()), "units past 64 touch, aim and harvest");
    }

    #[test]
    fn the_squad_matches_the_cpu_for_one_frame() {
        use slime_mould::mould::squad::{self, Frame, ShotIn, UnitIn};
        let Some((device, queue)) = test_gpu() else { return };
        let shared = Shared::new(&device);
        let base = Settings { piles: 23, rect: true, crystal: 512.0, crystal_eat: 0.02, upkeep: 0.01, walkers: 5_000, capacity: 15_000, wall_hp: 1000.0, ..small(0) };
        // And with the life cycle (2026-11-11): shots miss spores in flight
        // and kill the dormant, and a struck walker is dead.
        let life = with_life(base.clone());
        // And the tiers (2026-11-12, PR #85's shot alarm): with the alarm
        // field on, a hit lays `squad::SHOT_ALARM` within twice its radius.
        // The Swarmer has the field and no states (no spores, no dormancy,
        // no strike), the dish an alarm that needed the states missed.
        let tier = |t: Tier, mut s: Settings| {
            t.settings(&mut s).unwrap();
            s
        };
        let (sleeper, swarmer) = (tier(Tier::Sleeper, base.clone()), tier(Tier::Swarmer, base.clone()));
        // And shots that lay the crater (`Frame::crater`, 2026-11-12): 20
        // fine steps, under `scorch_full` (30), so the cut fades with the
        // steps left; the frame's tick then cuts the trail under it.
        // And the cone (`BigParams::scorch_cone`, the look A/B): fewer steps
        // toward the strike's rim, the same whole numbers on both.
        let cone = Settings { scorch_cone: true, ..base.clone() };
        let cases = [("plain", base.clone(), 0), ("spores and dormancy", life, 0), ("the Sleeper", sleeper, 0), ("the Swarmer, the alarm and no states", swarmer, 0), ("plain, shots that lay craters", base, 20), ("plain, cone craters", cone, 20)];
        for (name, s, crater) in cases {
            let mut e = Engine::new(&device, &queue, &shared, &s, SCALE);
            let mut cpu = e.dish.clone();
            for t in 0..60 {
                if t == 20 {
                    cpu.build_wall(70.0, 40.0, 70.0, 120.0, 2.0);
                }
                cpu.tick();
            }
            let (w, h) = (cpu.w, cpu.h);
            let k = SCALE as f32;
            let slime = squad::slime_trail(s.knee);
            let thick = (0..w * h).max_by(|&a, &b| cpu.trail[a].total_cmp(&cpu.trail[b])).unwrap();
            let t = ((thick % w) as f32 + 0.5, (thick / w) as f32 + 0.5);
            let pile = |p: usize| (cpu.piles[p].0 * k, cpu.piles[p].1 * k);
            let unit = |at: (f32, f32), take: f32| UnitIn { x: at.0, y: at.1, touch: 3.0, sight: 30.0, take_r: 4.0, take, smell_r: 2.0, smell: 0.5 };
            // Two units on one small patch of crystal 3, asking more than it
            // holds, so the second gets what the first left.
            let patch = |take: f32| UnitIn { take_r: 1.5, ..unit(pile(3), take) };
            let mut units = vec![unit(t, 0.0), patch(3000.0), patch(3000.0)];
            units.extend((5..10).map(|p| unit(pile(p), 100.0)));
            // Units that see only a few blocks, so which block's centre is in
            // sight decides where they aim.
            units.extend((0..6).map(|q| UnitIn { sight: 2.5 + q as f32, ..unit((t.0 + 3.0 * q as f32 - 7.0, t.1 + 2.0), 0.0) }));
            // Shots from each crystal toward the thickest mould, and two across
            // the wall at x = 140 (fine cells), one of them from beside it.
            let mut shots: Vec<ShotIn> = (0..12).map(|p| ShotIn { from: pile(p), to: t, r: 3.0 }).collect();
            shots.push(ShotIn { from: (120.0, 160.0), to: (190.0, 160.0), r: 3.0 });
            // A shot that meets the wall before any slime: from a clear cell
            // beside it, across it.
            let clear = (60..250)
                .flat_map(|y| (120..138).map(move |x| (x as f32 + 0.5, y as f32 + 0.5)))
                .find(|&(x, y)| {
                    let p = squad::path(w, h, (x, y), (x + 30.0, y));
                    let stop = p.iter().position(|&i| !cpu.agar[i as usize]);
                    stop.is_some_and(|n| p[..n].iter().all(|&i| cpu.trail[i as usize] < slime))
                })
                .expect("a clear way to the wall");
            shots.push(ShotIn { from: clear, to: (clear.0 + 30.0, clear.1), r: 2.0 });
            // A unit on the first wall cell that shot meets, so its smell disc
            // covers wall and agar (it must land only on the agar). One beside
            // the clear cell missed a planted bug: the wall was out of reach.
            let p = squad::path(w, h, clear, (clear.0 + 30.0, clear.1));
            let wall = p.into_iter().find(|&i| !cpu.agar[i as usize]).unwrap() as usize;
            units.push(UnitIn { smell_r: 4.0, ..unit(((wall % w) as f32 + 0.5, (wall / w) as f32 + 0.5), 0.0) });
            // Wide discs, as the soldiers' at scale 8 (2026-11-12, when the
            // unit passes took a workgroup each): boxes of over 256 cells,
            // so each thread takes several cells and blocks, and the harvest
            // several pieces, stopping in one or taking all it reaches.
            let wide = units.len();
            units.extend([(13, 60.0), (14, 1.0e6)].map(|(p, take)| UnitIn { touch: 12.0, sight: 60.0, take_r: 12.0, smell_r: 12.0, ..unit(pile(p), take) }));
            // The same shot twice: a walker in both strikes dies once, counted
            // for the first.
            let again = shots.len();
            shots.push(shots[0]);
            // A shot from the cell of a walker that starved in the mould, so its
            // strike covers an empty slot (which must stay out of the count).
            let starved = (0..cpu.walkers.len())
                .map(|k| (k, cpu.walkers[k]))
                .find(|&(k, (x, y, _))| !cpu.lives(k) && cpu.trail[y as usize * w + x as usize] >= slime)
                .expect("a slot that starved in the mould");
            let (sx, sy, _) = starved.1;
            shots.push(ShotIn { from: (sx, sy), to: t, r: 3.0 });
            // A radius that is not a whole number: its alarm's squared radius
            // (r2 of 5, 25) is not four times its strike's (r2 of 2.5, 6).
            shots.push(ShotIn { from: pile(12), to: t, r: 2.5 });
            // With the life cycle, shots from slime beside spores in flight,
            // whose strikes cover them (they must be missed).
            if !cpu.life.is_empty() {
                let slimy = |x: i64, y: i64| x >= 0 && y >= 0 && (x as usize) < w && (y as usize) < h && cpu.agar[y as usize * w + x as usize] && cpu.trail[y as usize * w + x as usize] >= slime;
                let beside: Vec<(f32, f32)> = (0..cpu.walkers.len())
                    .filter(|&k| cpu.lives(k) && cpu.life[k] == LIFE_SPORE)
                    .filter_map(|k| {
                        let (x, y) = (cpu.walkers[k].0 as i64, cpu.walkers[k].1 as i64);
                        (-2..=2).flat_map(|dy| (-2..=2).map(move |dx| (x + dx, y + dy))).find(|&(x, y)| slimy(x, y)).map(|(x, y)| (x as f32 + 0.5, y as f32 + 0.5))
                    })
                    .take(8)
                    .collect();
                shots.extend(beside.into_iter().map(|from| ShotIn { from, to: t, r: 4.0 }));
            }
            let f = Frame { units, shots, slime, crater };
            e.inject(&cpu);
            e.set_squad(&f);
            run(&mut e, &shared, &s, 1, Hand::default());
            let (before, at) = (cpu.life.clone(), cpu.walkers.clone());
            let cpu_out = squad::frame(&mut cpu, &f, 1);
            let gpu_out = e.squad_out();
            let kills: u32 = cpu_out.shots.iter().map(|q| q.killed).sum();
            assert_eq!(cpu_out.shots[again].hit, cpu_out.shots[0].hit, "the repeated shot hits where the first did");
            assert_eq!(cpu_out.shots[again].killed, 0, "and kills nothing the first did not");
            let walled = f.shots.iter().zip(&cpu_out.shots).filter(|(q, o)| o.hit.is_none() && squad::path(w, h, q.from, q.to).iter().any(|&i| !cpu.agar[i as usize])).count();
            let took: Vec<f32> = cpu_out.units.iter().map(|u| u.took).collect();
            eprintln!(
                "squad, {name}: {} shots, {} hit, {kills} killed, {walled} stopped by a wall; touching {:?}; took {took:?}",
                f.shots.len(),
                cpu_out.shots.iter().filter(|q| q.hit.is_some()).count(),
                cpu_out.units.iter().map(|u| u.touching).collect::<Vec<_>>()
            );
            assert_eq!(gpu_out, cpu_out, "the squad's outputs");
            assert!(kills > 0 && walled > 0, "shots kill, and a wall stops one");
            assert!(cpu_out.units[0].touching > 0 && cpu_out.units[0].target.is_some() && cpu_out.units[0].dense > 0, "the unit in the mould is touched and aims at walkers it counts");
            assert!(took[1] == 3000.0 && took[2] > 0.0 && took[2] < 3000.0 && took[3..].iter().any(|&t| t > 0.0), "units harvest; the second on a patch gets what the first left");
            assert!(took[wide] == 60.0 && took[wide + 1] > 0.0 && took[wide + 1] < 1.0e6 && cpu_out.units[wide].target.is_some(), "the wide discs harvest, stopping when full or taking all they reach");
            // The world after the frame.
            let gpu = walkers_of(&e);
            let energy = floats(&read_back(&device, &queue, &e.energy));
            let lives_off = (0..cpu.walkers.len()).filter(|&k| (energy[k] >= 0.0) != cpu.lives(k)).count();
            let off = (0..cpu.walkers.len()).filter(|&k| cpu.lives(k)).filter(|&k| {
                let (g, c) = (gpu[k], cpu.walkers[k]);
                (g.0 - c.0).abs() > 1e-3 || (g.1 - c.1).abs() > 1e-3 || (g.2 - c.2).abs() > 1e-3
            }).count();
            let side = (e.done % 2) as usize;
            let lt = l1(&floats(&read_back(&device, &queue, &e.trail[side])), &cpu.trail);
            let ls = l1(&floats(&read_back(&device, &queue, &e.scent[side])), &cpu.scent);
            let (lc, _, _, agar_off, ground_off) = world_off(&e, &cpu);
            // The alarm the shots laid, spread one tick (0 without the field).
            let la = if cpu.alarm.is_empty() { 0.0 } else { l1(&e.alarm_of(), &cpu.alarm) };
            let alarm_sum: f64 = cpu.alarm.iter().map(|&v| v as f64).sum();
            eprintln!("squad world, {name}: {lives_off} slots differ in life, {off} walkers off; trail L1 {lt:.1e}, scent L1 {ls:.1e}, charge L1 {lc:.1e}, alarm L1 {la:.1e} (in all {alarm_sum:.0}); {agar_off} agar and {ground_off} ground off");
            assert_eq!(lives_off, 0, "the same walkers die");
            assert!(off * 1000 <= cpu.walkers.len(), "{off} walkers off");
            assert!(lt <= 1e-6 && ls <= 1e-6 && lc <= 1e-6 && la <= 1e-6, "trail L1 {lt:.2e}, scent L1 {ls:.2e}, charge L1 {lc:.2e}, alarm L1 {la:.2e}");
            assert!(cpu.alarm.is_empty() || alarm_sum > 100.0 * squad::SHOT_ALARM as f64, "the shots lay the alarm: {alarm_sum:.0} in all");
            assert!(agar_off == 0 && ground_off == 0);
            // The craters the hits laid, counted down through the tick, to
            // the step (none without them).
            let (gc, cc) = (e.crater_of(), cpu.scorch.clone());
            if crater == 0 {
                assert!(cc.is_empty() && gc.iter().all(|&v| v == 0.0), "{name}: no crater unless shots lay one");
            } else {
                let off = (0..w * h).filter(|&i| gc[i] != cc[i] as f32).count();
                let laid = cc.iter().filter(|&&v| v > 0).count();
                eprintln!("squad craters, {name}: {laid} cells under a crater, {off} off");
                assert!(laid > 50 && off == 0, "{name}: {laid} crater cells, {off} off");
                // A flat crater leaves one count on every cell; a cone, many.
                let counts: std::collections::BTreeSet<u16> = cc.iter().copied().filter(|&v| v > 0).collect();
                assert_eq!(counts.len() > 1, s.scorch_cone, "{name}: counts {counts:?}");
            }
            if !before.is_empty() {
                // Spores in a struck disc are missed, the dormant in one die.
                let struck = |k: usize| {
                    let (x, y, _) = at[k];
                    f.shots.iter().zip(&cpu_out.shots).any(|(q, o)| o.hit.is_some_and(|(hx, hy)| (x.floor() + 0.5 - hx).hypot(y.floor() + 0.5 - hy) <= q.r))
                };
                let missed = (0..before.len()).filter(|&k| cpu.lives(k) && before[k] == LIFE_SPORE && struck(k)).count();
                let slept = (0..before.len()).filter(|&k| before[k] == LIFE_DORMANT && cpu.life[k] == LIFE_DEAD).count();
                let (st, tm) = e.life_of();
                // Every slot's, the emptied ones' too (a shot one is dead).
                let states_off = (0..before.len()).filter(|&k| st[k] != cpu.life[k] || tm[k] != cpu.life_t[k]).count();
                let count = |l: u8| (0..before.len()).filter(|&k| cpu.lives(k) && before[k] == l).count();
                eprintln!("squad life: {} spores, {} dormant at the frame; {missed} spores in a struck disc missed, {slept} dormant killed; {states_off} states or timers off", count(LIFE_SPORE), count(LIFE_DORMANT));
                assert!((missed > 0) == (s.spore_rate > 0.0) && slept > 0, "shots meet spores (if any fly) and the dormant");
                assert_eq!(states_off, 0, "life states and timers");
            }
        }
    }

    /// The alarm field alone (the Swarmer's, 2026-11-12): no spores, no
    /// dormancy and no strike, so no life states, and nothing else that
    /// needs the outside pipelines. The field must still spread and the
    /// walkers sense it, as on the CPU, which makes the field with the dish;
    /// a squad shot's alarm (`squad::SHOT_ALARM`) lands on such a dish.
    /// Alarm laid straight into the CPU's field round the thickest trail,
    /// then four ticks on both.
    #[test]
    fn an_alarm_without_the_states_matches_the_cpu() {
        use slime_mould::mould::squad;
        let Some((device, queue)) = test_gpu() else { return };
        let shared = Shared::new(&device);
        // The Swarmer's pull (+3), and the turn from gunfire
        // (`larder_world::FLEE_WEIGHT`, -1, an opt-in since 2026-11-12).
        for weight in ["3", "-1"] {
        let mut s = small(0);
        Tier::Swarmer.settings(&mut s).unwrap();
        set(&mut s, "alarm_weight", weight).unwrap();
        let mut e = Engine::new(&device, &queue, &shared, &s, SCALE);
        let mut cpu = e.dish.clone();
        for _ in 0..30 {
            cpu.tick();
        }
        let (w, h) = (cpu.w, cpu.h);
        let thick = (0..w * h).max_by(|&a, &b| cpu.trail[a].total_cmp(&cpu.trail[b])).unwrap();
        for c in squad::disc(w, h, (thick % w) as i64, (thick / w) as i64, 400) {
            if cpu.agar[c] {
                cpu.alarm[c] += squad::SHOT_ALARM;
            }
        }
        let laid: f64 = cpu.alarm.iter().map(|&v| v as f64).sum();
        let (moved, off, lt, ls) = twin(&mut e, &shared, &s, &mut cpu, 4);
        let la = l1(&e.alarm_of(), &cpu.alarm);
        let left: f64 = cpu.alarm.iter().map(|&v| v as f64).sum();
        eprintln!("the alarm alone, weight {weight}: {} walkers, {moved} moved, {off} off; alarm laid {laid:.0}, {left:.0} after four ticks; trail L1 {lt:.1e}, scent L1 {ls:.1e}, alarm L1 {la:.1e}; states {}, outside pipelines {}", cpu.walkers.len(), e.cycles, e.outside);
        assert!(cpu.life.is_empty() && !e.cycles && e.outside, "the field without the states, on the outside pipelines");
        assert!(off == 0 && lt <= 1e-6 && ls <= 1e-6 && la <= 1e-6, "weight {weight}: {off} walkers off; trail {lt:.1e}, scent {ls:.1e}, alarm {la:.1e}");
        }
    }

    /// `s` with spores and dormancy on, both frequent: a spore every hundred
    /// ticks a walker in the mould, flights of 20 ticks, a doze in twenty
    /// where the scent is under 50.
    fn with_life(mut s: Settings) -> Settings {
        for (k, v) in [("spore_rate", "0.01"), ("spore_scent", "-1"), ("spore_steps", "20"), ("spore_reach", "8"), ("spore_land", "1"), ("spore_rest", "10"), ("sleep_rate", "0.05"), ("wake_scent", "50"), ("sleep_rest", "10"), ("wake_steps", "10")] {
            set(&mut s, k, v).unwrap();
        }
        s
    }

    /// A wall built on the GPU (`Engine::build_wall`) against the same wall
    /// on the CPU, from the same warmed dish: the same cells walled, their
    /// trail, scent and charge gone, a crystal it cuts losing those cells,
    /// and every walker it covered moved to a free cell (the CPU moves them
    /// one by one, so not always to the same cells), with the crowd limit
    /// holding and no walker lost.
    #[test]
    fn a_wall_builds_on_the_gpu_as_on_the_cpu() {
        let Some((device, queue)) = test_gpu() else { return };
        let shared = Shared::new(&device);
        let w = Settings { piles: 23, rect: true, crystal: 512.0, ..small(0) };
        // Growth on, hungry and without crystals, so walkers starve all over
        // and empty slots keep stale places under the wall (before
        // 2026-11-11 the GPU moved those too).
        let hungry = Settings { crystal: 0.0, upkeep: 0.03, starve: 0.2, birth_energy: 0.6, graze: 10.0, graze_regrow: 0.0, capacity: 3 * w.walkers / 2, ..w.clone() };
        // With the life cycle: spores in flight and the struck dead stay
        // where they are under a new wall (the CPU skips them), the dormant
        // are moved out.
        let life = with_life(w.clone());
        for (name, s) in [("one a cell", w.clone()), ("no limit", Settings { crowd: false, ..w.clone() }), ("sealed", Settings { wall_scent: 0.0, ..w }), ("growth, empty slots under the wall", hungry), ("spores, dormant and the dead", life)] {
            let mut e = Engine::new(&device, &queue, &shared, &s, SCALE);
            let mut cpu = e.dish.clone();
            let cycles = s.spore_rate > 0.0;
            // Long enough with growth for walkers to starve (a walker's
            // energy lasts about 30 ticks at this upkeep).
            for t in 0..if s.upkeep > 0.0 { 150 } else { 40 } {
                if cycles && t == 30 {
                    // The dead under the wall to come.
                    cpu.strike(57.5, 80.0, 5.0, 0.0, 0.0);
                }
                cpu.tick();
            }
            e.inject(&cpu);
            // Across home's crystal and the mould on it (pile 0 at 56, 80).
            let seg = (45.0, 60.0, 70.0, 100.0, 4.0);
            let cells = cpu.build_wall(seg.0, seg.1, seg.2, seg.3, seg.4);
            e.build_wall(&shared, &s, seg.0, seg.1, seg.2, seg.3, seg.4);
            let (_, _, _, agar_off, ground_off) = world_off(&e, &cpu);
            let u32s = |b: &wgpu::Buffer| -> Vec<u32> { read_back(&device, &queue, b).chunks_exact(4).map(|c| u32::from_le_bytes(c.try_into().unwrap())).collect() };
            let agar: Vec<bool> = u32s(&e.agar).iter().map(|&a| a != 0).collect();
            let side = (e.done % 2) as usize;
            let trail = floats(&read_back(&device, &queue, &e.trail[side]));
            let scent = floats(&read_back(&device, &queue, &e.scent[side]));
            let charge = floats(&read_back(&device, &queue, &e.charge));
            let walled: Vec<usize> = (0..cpu.agar.len()).filter(|&i| cpu.wall[i]).collect();
            let left_on = walled.iter().filter(|&&i| trail[i] != 0.0 || scent[i] != 0.0 || charge[i] != 0.0).count();
            let gpu = walkers_of(&e);
            // Slots that hold a cell only: the empty ones (growth), spores and
            // the dead keep their places.
            let living: Vec<bool> = (0..gpu.len()).map(|k| cpu.lives(k) && (cpu.life.is_empty() || !matches!(cpu.life[k], LIFE_SPORE | LIFE_DEAD))).collect();
            let kinds = |l: u8| (0..gpu.len()).filter(|&k| cpu.lives(k) && cpu.life.get(k) == Some(&l) && !cpu.agar[cpu.walkers[k].1 as usize * cpu.w + cpu.walkers[k].0 as usize]).count();
            let (spores_under, dead_under) = (kinds(LIFE_SPORE), kinds(LIFE_DEAD));
            let dormant_out = (0..gpu.len()).filter(|&k| cpu.lives(k) && cpu.life.get(k) == Some(&LIFE_DORMANT) && cpu.walkers[k] != e.dish.walkers[k]).count();
            let empty_under = (0..gpu.len()).filter(|&k| !living[k] && !cpu.agar[cpu.walkers[k].1 as usize * cpu.w + cpu.walkers[k].0 as usize]).count();
            let inside = |ws: &[(f32, f32, f32)], ag: &[bool]| ws.iter().zip(&living).filter(|&(&(x, y, _), &l)| l && !ag[y as usize * cpu.w + x as usize]).count();
            let (gpu_inside, cpu_inside) = (inside(&gpu, &agar), inside(&cpu.walkers, &cpu.agar));
            let moved = |ws: &[(f32, f32, f32)]| ws.iter().zip(&e.dish.walkers).zip(&living).filter(|&((a, b), &l)| l && a != b).count();
            let (moved_gpu, moved_cpu) = (moved(&gpu), moved(&cpu.walkers));
            let stale_moved = gpu.iter().zip(&cpu.walkers).zip(&living).filter(|&((a, b), &l)| !l && a != b).count();
            let occ = u32s(&e.occ);
            let mut want = vec![0u32; occ.len()];
            for (&(x, y, _), &l) in gpu.iter().zip(&living) {
                if l {
                    want[y as usize * cpu.w + x as usize] += 1;
                }
            }
            let occ_off = occ.iter().zip(&want).filter(|(a, b)| a != b).count();
            let crowded = if s.crowd { occ.iter().filter(|&&c| c > 1).count() } else { 0 };
            eprintln!("{name}: {cells} cells walled, {} walkers; {moved_cpu} moved out on the CPU, {moved_gpu} on the GPU; {cpu_inside} and {gpu_inside} left inside; {agar_off} agar and {ground_off} ground cells off, {left_on} walled cells with trail, scent or charge, {occ_off} counts off, {crowded} cells over the limit; {empty_under} empty slots under the wall, {stale_moved} of them moved", gpu.len());
            if s.upkeep > 0.0 {
                assert!(empty_under > 0, "{name}: empty slots under the wall");
            }
            if cycles {
                eprintln!("{name}: under the wall {spores_under} spores and {dead_under} dead stay, {dormant_out} dormant moved out");
                assert!(dead_under > 0 && dormant_out > 0, "{name}: the dead stay under the wall and the dormant move");
            }
            assert_eq!(stale_moved, 0, "{name}: an empty slot moved");
            assert!(cells > 0 && moved_cpu > 0, "{name}: the wall covers walkers");
            assert!(agar_off == 0 && ground_off == 0 && left_on == 0, "{name}: {agar_off} agar, {ground_off} ground, {left_on} cells left on");
            assert_eq!(gpu.len(), cpu.walkers.len(), "{name}: no walker lost");
            assert!(gpu_inside <= cpu_inside, "{name}: {gpu_inside} walkers left inside the wall, {cpu_inside} on the CPU");
            assert!(moved_gpu == moved_cpu, "{name}: {moved_gpu} walkers moved, {moved_cpu} on the CPU");
            assert!(occ_off == 0 && crowded == 0, "{name}: {occ_off} counts off, {crowded} crowded");
        }
    }

    /// **The Larder game on the engine** (W2b, `gpu::larder_world`): the
    /// same game on the GPU world and the CPU race world, from the same
    /// seed. Both start alike (the race settings give the CPU's params); a
    /// soldier clearing a crystal far from the mould banks the same food on
    /// both, since no walker reaches it; and every 5 game seconds the mould
    /// is within 8% of the CPU's size, and its mould grid (squares of 4
    /// dish cells, summed on the GPU once a game second) holds at least 70%
    /// of it in the CPU's squares, after the same grid at the start. The
    /// starting walkers starve at about 17 s (start energy 0.5 at upkeep
    /// 0.001 a fine step) on both, so at
    /// 20 s the count has just fallen by a third (5,234 to 3,120 on seed
    /// 3, the GPU 3,220); at one engine tick a game tick, half the CPU's
    /// fine steps, the fall would come at 33 s and miss the bound by far.
    #[test]
    fn the_larder_game_runs_on_the_engine_as_on_the_cpu() {
        use slime_mould::gpu::larder_world::{race_settings, GpuWorld};
        use slime_mould::mould::larder_world::{race_world, Cmd, Game, Order, Rules, World};
        let Some((device, queue)) = test_gpu() else { return };
        let (seed, scale, walkers) = (3, 2, 5_000);
        let rules = Rules { cap: 20.0, ..Rules::default() };
        let mut cpu = Game::new(race_world(seed, scale, walkers), rules, seed);
        let mut gpu = Game::new(GpuWorld::new(&device, &queue, race_settings(seed, scale, walkers), scale), rules, seed);
        assert_eq!(gpu.world.size(), cpu.world.size());
        assert_eq!(gpu.world.crystals(), cpu.world.crystals(), "the same crystals and charge at the start");
        assert_eq!(gpu.world.living(), cpu.world.living());
        assert_eq!((gpu.larder, gpu.core), (cpu.larder, cpu.core));
        // The mould grid (the campaign's goals, the player AI): the same
        // squares from the same walkers, then the GPU's once-a-second sum
        // against the CPU's own (`GRID_EVERY`, 2026-11-12).
        const BIN: usize = 4;
        assert_eq!(gpu.world.mould_grid(BIN), cpu.world.mould_grid(BIN), "the same mould grid at the start");
        // Soldier 0 clears the crystal nearest the larder (the larder's own).
        let c = cpu.world.crystals().iter().map(|c| c.0).min_by(|a, b| (a.0 - cpu.larder.0).hypot(a.1 - cpu.larder.1).total_cmp(&(b.0 - cpu.larder.0).hypot(b.1 - cpu.larder.1))).unwrap();
        for g in [&mut cpu.log, &mut gpu.log] {
            g.clear();
        }
        assert!(cpu.apply(Cmd::Order(0, Order::Clear(c.0, c.1))) && gpu.apply(Cmd::Order(0, Order::Clear(c.0, c.1))));
        let (mut sizes, mut overlaps) = (Vec::new(), Vec::new());
        while cpu.end.is_none() {
            cpu.step();
            gpu.step();
            let (a, b) = (cpu.world.mould_grid(BIN).unwrap(), gpu.world.mould_grid(BIN).unwrap());
            if cpu.world.ticks() % 75 == 0 {
                sizes.push((cpu.world.ticks() / 15, cpu.world.living(), gpu.world.living()));
                // The share of the mould in the same squares on both.
                let same: u32 = a.iter().zip(&b).map(|(x, y)| *x.min(y)).sum();
                overlaps.push(same as f64 / a.iter().sum::<u32>().max(b.iter().sum::<u32>()).max(1) as f64);
            }
        }
        eprintln!("game s, living CPU, GPU: {sizes:?}; banked {} / {}; the grids' overlap {overlaps:.3?}", cpu.harvested, gpu.harvested);
        assert_eq!(gpu.world.ticks(), cpu.world.ticks(), "the same game ticks");
        assert!(cpu.harvested > 0.0 && gpu.harvested == cpu.harvested, "banked {} on the GPU, {} on the CPU", gpu.harvested, cpu.harvested);
        for &(t, a, b) in &sizes {
            let (a, b) = (a as f64, b as f64);
            assert!((a - b).abs() <= 0.08 * a, "mould at {t} s: {b} walkers on the GPU, {a} on the CPU");
        }
        assert!(sizes.iter().any(|&(_, a, _)| a > walkers) && sizes.last().unwrap().1 < walkers, "the mould grew, then its first walkers starved: {sizes:?}");
        // 0.91, 0.90, 0.88 and 0.78 (2026-11-12): the two engines' floats
        // part, and most at 20 s, just after the first walkers starve.
        assert!(overlaps.iter().all(|&o| o >= 0.7), "the mould grids overlap: {overlaps:.3?}");
    }

    /// **The campaign's tiers on the engine** (`mould::tiers`, 2026-11-12):
    /// the Drifter, Sprawler and Trunk are settings over rules both engines
    /// run, so each plays the race world's game on the GPU as on the CPU,
    /// within the race world's own 8% every 5 game seconds over 20 s.
    #[test]
    fn the_campaign_tiers_run_on_the_engine_as_on_the_cpu() {
        use slime_mould::gpu::larder_world::{race_settings, GpuWorld};
        use slime_mould::mould::big::BigDish;
        use slime_mould::mould::larder_world::{race_params, Game, Rules, World};
        use slime_mould::mould::tiers::Tier;
        let Some((device, queue)) = test_gpu() else { return };
        let (seed, scale, walkers) = (3, 2, 5_000);
        let rules = Rules { cap: 20.0, ..Rules::default() };
        for t in [Tier::Drifter, Tier::Sprawler, Tier::Trunk] {
            let mut p = race_params(seed, scale, walkers);
            t.apply(&mut p);
            let mut s = race_settings(seed, scale, walkers);
            t.settings(&mut s).unwrap();
            let mut cpu = Game::new(BigDish::new(p), rules, seed);
            let mut gpu = Game::new(GpuWorld::new(&device, &queue, s, scale), rules, seed);
            assert_eq!(gpu.world.living(), cpu.world.living(), "{}", t.name());
            let mut sizes = Vec::new();
            while cpu.end.is_none() {
                cpu.step();
                gpu.step();
                if cpu.world.ticks() % 75 == 0 {
                    sizes.push((cpu.world.ticks() / 15, cpu.world.living(), gpu.world.living()));
                }
            }
            eprintln!("{}: game s, living CPU, GPU: {sizes:?}", t.name());
            for &(at, a, b) in &sizes {
                let (a, b) = (a as f64, b as f64);
                assert!((a - b).abs() <= 0.08 * a, "{}: mould at {at} s: {b} walkers on the GPU, {a} on the CPU", t.name());
            }
        }
    }

    /// A squad firing into the race world's mould on the engine as on the
    /// CPU (2026-11-12) with the mould turning from fresh gunfire (the
    /// opt-in `larder_world::FLEE_KEEP` and `FLEE_WEIGHT`): every hit's
    /// alarm steers walkers on both sides. The squad attacks the core from
    /// the start for 30 s; the moulds stay within the race twin's 8% and the
    /// kills within 15%. Without the alarm (the race world's default) the
    /// CPU's mould differs from its flee game, so the bound tests the rule
    /// and not only the growth.
    #[test]
    fn a_squad_firing_on_the_fleeing_mould_plays_on_the_engine_as_on_the_cpu() {
        use slime_mould::gpu::larder_world::{race_settings, GpuWorld};
        use slime_mould::mould::larder_world::{race_params, Cmd, Game, Order, Rules, World, FLEE_KEEP, FLEE_WEIGHT};
        let Some((device, queue)) = test_gpu() else { return };
        let (seed, scale, walkers) = (3, 2, 5_000);
        // The flee alone: since 2026-11-12 shots lay craters by default,
        // which have their own twin (`a_squad_laying_craters_...`); with
        // both, seed 3's moulds part by 13% at 25 s.
        let rules = Rules { cap: 30.0, crater: 0.0, ..Rules::default() };
        let flee = || {
            let mut p = race_params(seed, scale, walkers);
            (p.alarm_keep, p.alarm_weight) = (FLEE_KEEP, FLEE_WEIGHT);
            p
        };
        let mut cpu = Game::new(slime_mould::mould::big::BigDish::new(flee()), rules, seed);
        let mut s = race_settings(seed, scale, walkers);
        (s.alarm_keep, s.alarm_weight) = (FLEE_KEEP, FLEE_WEIGHT);
        let mut gpu = Game::new(GpuWorld::new(&device, &queue, s, scale), rules, seed);
        let mut off = Game::new(slime_mould::mould::big::BigDish::new(race_params(seed, scale, walkers)), rules, seed);
        assert!(cpu.world.p.alarm_weight < 0.0 && gpu.world.s.alarm_weight < 0.0, "the race world flees gunfire on both");
        for g in [&mut cpu.log, &mut gpu.log, &mut off.log] {
            g.clear();
        }
        for k in 0..cpu.soldiers.len() {
            assert!(cpu.apply(Cmd::Order(k, Order::Attack)) && gpu.apply(Cmd::Order(k, Order::Attack)) && off.apply(Cmd::Order(k, Order::Attack)));
        }
        let mut sizes = Vec::new();
        while cpu.end.is_none() {
            cpu.step();
            gpu.step();
            off.step();
            if cpu.world.ticks() % 75 == 0 {
                sizes.push((cpu.world.ticks() / 15, cpu.world.living(), gpu.world.living(), off.world.living()));
            }
        }
        eprintln!("firing on the fleeing mould: game s, living CPU, GPU, CPU without the alarm: {sizes:?}; shots {} / {}, killed {} / {} (without the alarm {})", cpu.shots, gpu.shots, cpu.killed, gpu.killed, off.killed);
        assert!(cpu.shots > 20 && gpu.shots > 20, "the squad fired: {} / {}", cpu.shots, gpu.shots);
        let (a, b) = (cpu.killed as f64, gpu.killed as f64);
        assert!(a > 0.0 && (a - b).abs() <= 0.15 * a, "killed {b} on the GPU, {a} on the CPU");
        for &(t, a, b, _) in &sizes {
            let (a, b) = (a as f64, b as f64);
            assert!((a - b).abs() <= 0.08 * a, "mould at {t} s: {b} walkers on the GPU, {a} on the CPU");
        }
        assert!(off.killed != cpu.killed || sizes.iter().any(|&(_, a, _, c)| a != c), "the alarm changed the CPU's game");
    }

    /// **Shots that lay the crater** (`Rules::crater`, 2026-11-12, the
    /// owner's hole under fire) on the engine as on the CPU: the race world,
    /// seed 3, the squad sent to attack from the start for 30 s with 10 s
    /// craters. The moulds stay within the race twin's 8% and the kills
    /// within 15%, as the flee's twin; and the crater changes the CPU's game
    /// (a frame twin holds the field itself to the step:
    /// `the_squad_matches_the_cpu_for_one_frame`).
    #[test]
    fn a_squad_laying_craters_plays_on_the_engine_as_on_the_cpu() {
        use slime_mould::gpu::larder_world::{race_settings, GpuWorld};
        use slime_mould::mould::larder_world::{race_params, Cmd, Game, Order, Rules, World};
        let Some((device, queue)) = test_gpu() else { return };
        let (seed, scale, walkers) = (3, 2, 5_000);
        // The crater alone: the core's pull (on by default since the core
        // defence, `Rules::core_pull`) draws this attacking squad's mould
        // down to 730 walkers by 30 s, under the 8% bound's reach; the
        // pull's own twin plays both together, the shipped defaults
        // (`a_squad_at_the_core_draws_the_mould_on_the_engine_as_on_the_cpu`),
        // and `twin_gap_over_seeds_is_chaos` shows the gap at 30 s is the
        // CPU's own spread on either rule.
        let rules = Rules { cap: 30.0, crater: 10.0, core_pull: 0.0, ..Rules::default() };
        let mut cpu = Game::new(slime_mould::mould::big::BigDish::new(race_params(seed, scale, walkers)), rules, seed);
        let mut gpu = Game::new(GpuWorld::new(&device, &queue, race_settings(seed, scale, walkers), scale), rules, seed);
        let mut off = Game::new(slime_mould::mould::big::BigDish::new(race_params(seed, scale, walkers)), Rules { crater: 0.0, ..rules }, seed);
        for g in [&mut cpu.log, &mut gpu.log, &mut off.log] {
            g.clear();
        }
        for k in 0..cpu.soldiers.len() {
            assert!(cpu.apply(Cmd::Order(k, Order::Attack)) && gpu.apply(Cmd::Order(k, Order::Attack)) && off.apply(Cmd::Order(k, Order::Attack)));
        }
        let mut sizes = Vec::new();
        while cpu.end.is_none() {
            cpu.step();
            gpu.step();
            off.step();
            if cpu.world.ticks() % 75 == 0 {
                sizes.push((cpu.world.ticks() / 15, cpu.world.living(), gpu.world.living(), off.world.living()));
            }
        }
        let (laid_cpu, laid_gpu) = (cpu.world.scorch.iter().filter(|&&v| v > 0).count(), gpu.world.e.crater_of().iter().filter(|&&v| v > 0.0).count());
        eprintln!("firing with craters: game s, living CPU, GPU, CPU without craters: {sizes:?}; shots {} / {}, killed {} / {} (without craters {}); cells under a crater at the end {laid_cpu} / {laid_gpu}", cpu.shots, gpu.shots, cpu.killed, gpu.killed, off.killed);
        assert!(cpu.shots > 20 && gpu.shots > 20, "the squad fired: {} / {}", cpu.shots, gpu.shots);
        assert!(laid_cpu > 0 && laid_gpu > 0, "craters on both");
        let (a, b) = (cpu.killed as f64, gpu.killed as f64);
        assert!(a > 0.0 && (a - b).abs() <= 0.15 * a, "killed {b} on the GPU, {a} on the CPU");
        for &(t, a, b, _) in &sizes {
            let (a, b) = (a as f64, b as f64);
            assert!((a - b).abs() <= 0.08 * a, "mould at {t} s: {b} walkers on the GPU, {a} on the CPU");
        }
        assert!(off.killed != cpu.killed || sizes.iter().any(|&(_, a, _, c)| a != c), "the craters changed the CPU's game");
    }

    /// **Is the twins' gap chaos?** (the technical lead's question on the
    /// core defence, 2026-11-12). The race twins compare one seed at 30 s
    /// of attack, after the squad has shot the mould from 5,000 walkers to
    /// under 1,000, where any small difference is amplified. This plays
    /// seeds 3-10 at the shipped defaults (crater 10 s, pull 8) and with the
    /// pull off, on the CPU, the GPU, and the CPU again with one walker
    /// more (5,001, same map and crystals): the noise floor. Measured
    /// 2026-11-12 (software Vulkan), the mould at 30 s, mean gap from the
    /// CPU over 8 seeds:
    /// - pull 8: GPU 176 walkers, one more walker 213; the GPU above the
    ///   CPU on 4, below on 4; kills mean 1,469 CPU, 1,514 GPU, 1,554 one
    ///   more walker.
    /// - pull 0: GPU 107, one more walker 164; 4 above, 4 below (seed 7:
    ///   1,044 CPU, 1,408 GPU, 1,258 one more walker, 35% apart, so the 8%
    ///   bound holds on seed 3 at pull 0 by luck).
    /// - at 15 s, before the fire, all within 70 walkers.
    ///
    /// So the gap grows with the fire and is no bigger than the CPU's own
    /// spread; the twins' floor of 100 walkers rests on this. Ignored
    /// (about 200 s): `cargo test --release --bin mouldgpu
    /// twin_gap_over_seeds_is_chaos -- --ignored --nocapture`.
    #[test]
    #[ignore]
    fn twin_gap_over_seeds_is_chaos() {
        use slime_mould::gpu::larder_world::{race_settings, GpuWorld};
        use slime_mould::mould::larder_world::{race_params, Cmd, Game, Order, Rules, World};
        let Some((device, queue)) = test_gpu() else { return };
        let (scale, walkers) = (2, 5_000);
        for pull in [8.0f32, 0.0] {
            let (mut gap, mut floor, mut above) = (0.0f64, 0.0f64, 0u32);
            let seeds = 3..=10u64;
            let n = seeds.clone().count() as f64;
            for seed in seeds {
                let rules = Rules { cap: 30.0, crater: 10.0, core_pull: pull, ..Rules::default() };
                let mut cpu = Game::new(slime_mould::mould::big::BigDish::new(race_params(seed, scale, walkers)), rules, seed);
                let mut gpu = Game::new(GpuWorld::new(&device, &queue, race_settings(seed, scale, walkers), scale), rules, seed);
                let mut alt = Game::new(slime_mould::mould::big::BigDish::new(race_params(seed, scale, walkers + 1)), rules, seed);
                for g in [&mut cpu.log, &mut gpu.log, &mut alt.log] {
                    g.clear();
                }
                for k in 0..cpu.soldiers.len() {
                    assert!(cpu.apply(Cmd::Order(k, Order::Attack)) && gpu.apply(Cmd::Order(k, Order::Attack)) && alt.apply(Cmd::Order(k, Order::Attack)));
                }
                let mut row = String::new();
                while cpu.end.is_none() {
                    cpu.step();
                    gpu.step();
                    alt.step();
                    if cpu.world.ticks() % 75 == 0 {
                        row += &format!(" {}s:{}/{}/{}", cpu.world.ticks() / 15, cpu.world.living(), gpu.world.living(), alt.world.living());
                    }
                }
                let (c, g, a) = (cpu.world.living() as f64, gpu.world.living() as f64, alt.world.living() as f64);
                gap += (g - c).abs() / n;
                floor += (a - c).abs() / n;
                above += (g > c) as u32;
                eprintln!("pull {pull} seed {seed}: killed CPU {} GPU {} one more walker {}; living CPU/GPU/one more:{row}", cpu.killed, gpu.killed, alt.killed);
            }
            eprintln!("pull {pull}: mould at 30 s, mean gap from the CPU: GPU {gap:.0}, one more walker {floor:.0}; GPU above on {above} of {n}");
            assert!(gap <= 1.25 * floor, "pull {pull}: the GPU's gap {gap:.0} is past the CPU's own spread {floor:.0}");
            assert!((2..=6).contains(&above), "pull {pull}: the GPU leans one way, above on {above} of {n}");
        }
    }

    /// **The core's pull** (`Core::pull`, core defence, 2026-11-12) on the
    /// engine as on the CPU: the race world, seed 3, the squad sent to
    /// attack the core from the start for 30 s with the pull at 8. The
    /// pull is the soldiers' own scent, laid by the squad frame on both
    /// engines, so no new GPU row: the moulds stay within the race twin's
    /// 8% (or 100 walkers, once few are left) and the kills within 15%,
    /// and the pull changes the CPU's game.
    #[test]
    fn a_squad_at_the_core_draws_the_mould_on_the_engine_as_on_the_cpu() {
        use slime_mould::gpu::larder_world::{race_settings, GpuWorld};
        use slime_mould::mould::larder_world::{race_params, Cmd, Game, Order, Rules, World};
        let Some((device, queue)) = test_gpu() else { return };
        let (seed, scale, walkers) = (3, 2, 5_000);
        let rules = Rules { cap: 30.0, crater: 10.0, core_pull: 8.0, ..Rules::default() };
        let mut cpu = Game::new(slime_mould::mould::big::BigDish::new(race_params(seed, scale, walkers)), rules, seed);
        let mut gpu = Game::new(GpuWorld::new(&device, &queue, race_settings(seed, scale, walkers), scale), rules, seed);
        let mut off = Game::new(slime_mould::mould::big::BigDish::new(race_params(seed, scale, walkers)), Rules { core_pull: 0.0, ..rules }, seed);
        for g in [&mut cpu.log, &mut gpu.log, &mut off.log] {
            g.clear();
        }
        for k in 0..cpu.soldiers.len() {
            assert!(cpu.apply(Cmd::Order(k, Order::Attack)) && gpu.apply(Cmd::Order(k, Order::Attack)) && off.apply(Cmd::Order(k, Order::Attack)));
        }
        let (mut sizes, mut pulled) = (Vec::new(), 0u32);
        while cpu.end.is_none() {
            cpu.step();
            gpu.step();
            off.step();
            let sc = cpu.world.scale();
            pulled += cpu.soldiers.iter().any(|s| s.alive && (s.x - cpu.core.0).hypot(s.y - cpu.core.1) <= 2.0 * cpu.rules.core_r * sc) as u32;
            if cpu.world.ticks() % 75 == 0 {
                sizes.push((cpu.world.ticks() / 15, cpu.world.living(), gpu.world.living(), off.world.living()));
            }
        }
        eprintln!("attacking the core with the pull: game s, living CPU, GPU, CPU without the pull: {sizes:?}; ticks a soldier pulled {pulled}; shots {} / {}, killed {} / {} (without the pull {}), soldiers left {} / {} ({})", cpu.shots, gpu.shots, cpu.killed, gpu.killed, off.killed, cpu.alive(), gpu.alive(), off.alive());
        assert!(pulled > 0, "a soldier reached the core");
        let (a, b) = (cpu.killed as f64, gpu.killed as f64);
        assert!(a > 0.0 && (a - b).abs() <= 0.15 * a, "killed {b} on the GPU, {a} on the CPU");
        // 8%, or 100 walkers once the pulled mould is under 1,250: by 30 s
        // the drawn hub has fed the squad's fire down to 730 walkers on the
        // CPU and 815 on the GPU (seed 3), 11% apart on kills 1.2% apart.
        // The floor is chaos, not a rule the engines differ on: over seeds
        // 3-10 the CPU with one walker more lands further from itself (213
        // walkers on average at 30 s) than the GPU does (176), and the GPU
        // is above on 4 seeds, below on 4 (`twin_gap_over_seeds_is_chaos`).
        for &(t, a, b, _) in &sizes {
            let (a, b) = (a as f64, b as f64);
            assert!((a - b).abs() <= (0.08 * a).max(100.0), "mould at {t} s: {b} walkers on the GPU, {a} on the CPU");
        }
        assert!(off.killed != cpu.killed || sizes.iter().any(|&(_, a, _, c)| a != c), "the pull changed the CPU's game");
    }

    /// **Buildings on the engine as on the CPU** (turret, lure, pump: units
    /// that do not move in the squad frame, `larder_world::buildings`). A
    /// pump on the larder's crystal banks the same food on both and a lure
    /// burns the same, since no walker reaches them in 20 s; a turret by
    /// the mould's core fires and kills on both, and the moulds stay within
    /// the race world's 8%.
    #[test]
    fn buildings_play_on_the_engine_as_on_the_cpu() {
        use slime_mould::gpu::larder_world::{race_settings, GpuWorld};
        use slime_mould::mould::larder_world::{race_world, Cmd, Game, Kind, Rules, Stance, World};
        let Some((device, queue)) = test_gpu() else { return };
        let (seed, scale, walkers) = (3, 2, 5_000);
        let rules = Rules { cap: 20.0, start_bank: 1e6, ..Rules::default() };
        let mut cpu = Game::new(race_world(seed, scale, walkers), rules, seed);
        let mut gpu = Game::new(GpuWorld::new(&device, &queue, race_settings(seed, scale, walkers), scale), rules, seed);
        let near = |g: &Game<_>, p: (f32, f32)| -> (f32, f32) {
            let w: &dyn World = &g.world;
            for r in 0..80 {
                for k in 0..16 {
                    let a = k as f32 / 16.0 * std::f32::consts::TAU;
                    let q = (p.0 + r as f32 * a.cos(), p.1 + r as f32 * a.sin());
                    if w.open(q.0, q.1) {
                        return q;
                    }
                }
            }
            panic!("no open cell near {p:?}")
        };
        let (_, crystal) = cpu.crystal_at(cpu.larder).expect("the larder's crystal");
        // Just outside the blob (a turret inside it is worn away in its
        // first frame) and within its range of it.
        let occ = cpu.world.occupancy();
        let w = cpu.world.w;
        let empty = |p: (f32, f32)| (-6i64..=6).all(|dy| (-6i64..=6).all(|dx| occ.get((p.1 as i64 + dy) as usize * w + (p.0 as i64 + dx) as usize).is_none_or(|&n| n == 0)));
        let turret = (1..60).map(|r| near(&cpu, (cpu.core.0 + r as f32 * 2.0, cpu.core.1))).find(|&p| empty(p)).expect("open ground past the blob");
        eprintln!("turret {:.0} fine cells from the core", (turret.0 - cpu.core.0).hypot(turret.1 - cpu.core.1));
        let lure = near(&cpu, (cpu.larder.0 + 16.0, cpu.larder.1 + 16.0));
        cpu.place(turret, crystal, lure);
        gpu.place(turret, crystal, lure);
        trait Place {
            fn place(&mut self, turret: (f32, f32), crystal: (f32, f32), lure: (f32, f32));
        }
        impl<W: World> Place for Game<W> {
            fn place(&mut self, turret: (f32, f32), crystal: (f32, f32), lure: (f32, f32)) {
                self.soldiers[0].x = turret.0;
                self.soldiers[0].y = turret.1;
                self.soldiers[0].stance = Stance::HoldFire;
                assert!(self.apply(Cmd::Build(Kind::Turret, turret)), "turret");
                assert!(self.apply(Cmd::Build(Kind::Pump, crystal)), "pump");
                assert!(self.apply(Cmd::Build(Kind::Lure, lure)), "lure");
            }
        }
        let mut sizes = Vec::new();
        while cpu.end.is_none() {
            cpu.step();
            gpu.step();
            if cpu.world.ticks() % 75 == 0 {
                sizes.push((cpu.world.ticks() / 15, cpu.world.living(), gpu.world.living()));
            }
        }
        eprintln!("buildings: living CPU, GPU {sizes:?}; pumped {} / {}, lured {} / {}, shots {} / {}, killed {} / {}", cpu.pumped, gpu.pumped, cpu.lured, gpu.lured, cpu.shots, gpu.shots, cpu.killed, gpu.killed);
        assert!(cpu.pumped > 0.0 && (gpu.pumped - cpu.pumped).abs() <= 1e-3 * cpu.pumped, "pumped {} on the GPU, {} on the CPU", gpu.pumped, cpu.pumped);
        assert!(cpu.lured > 0.0 && gpu.lured == cpu.lured, "lured {} on the GPU, {} on the CPU", gpu.lured, cpu.lured);
        assert!(cpu.killed > 0 && gpu.killed > 0, "the turret kills on both: {} / {}", cpu.killed, gpu.killed);
        for &(t, a, b) in &sizes {
            let (a, b) = (a as f64, b as f64);
            assert!((a - b).abs() <= 0.08 * a, "mould at {t} s: {b} walkers on the GPU, {a} on the CPU");
        }
    }

    /// A designed map (ROADMAP WM1) on the engine as on the CPU: the Fork
    /// (rock, wet, crystals of five sizes and four charges) and the Nest
    /// (34 walls with health). The same crystals, start and landing; a
    /// soldier banks the same food; the moulds stay within 8% (the race
    /// world's own twin bound). With the maps' soil (WM3, 2026-11-12: the
    /// Fork's and the Nest's clay, the Passes' and the Rim's sand and clay;
    /// the engine read none before).
    #[test]
    fn a_designed_map_plays_on_the_engine_as_on_the_cpu() {
        use slime_mould::gpu::larder_world::{race_settings, GpuWorld};
        use slime_mould::mould::big::BigDish;
        use slime_mould::mould::larder_world::{race_params, Cmd, Game, Order, Rules, World};
        use slime_mould::mould::map::Map;
        let Some((device, queue)) = test_gpu() else { return };
        let dir = std::path::Path::new(env!("CARGO_MANIFEST_DIR")).join("Reports/larder-maps-2026-11-11/maps");
        let (seed, scale, walkers) = (3, 2, 5_000);
        let rules = Rules { cap: 20.0, ..Rules::default() };
        for name in ["fork", "nest", "passes", "rim"] {
            let m = Map::load(&dir, name).unwrap();
            let mut cpu = Game::new(BigDish::with_map(race_params(seed, scale, walkers), &m), rules, seed);
            let mut gpu = Game::new(GpuWorld::with_map(&device, &queue, race_settings(seed, scale, walkers), scale, &m), rules, seed);
            assert!(cpu.world.soil.contains(&SOIL_CLAY) && gpu.world.e.ground_world & WORLD_SOIL != 0, "{name}: the map's clay on both");
            assert!(name == "fork" || name == "nest" || cpu.world.soil.contains(&SOIL_SAND), "{name}: the map's sand");
            assert_eq!(gpu.world.crystals(), cpu.world.crystals(), "{name}: the same crystals and charge at the start");
            let n = cpu.world.crystals().len();
            assert!((0..n).all(|c| gpu.world.crystal_r(c) == cpu.world.crystal_r(c)), "{name}: radii");
            assert!((0..n).any(|c| cpu.world.crystal_r(c) != cpu.world.crystal_r(0)), "{name}: crystals of more than one size");
            assert_eq!(gpu.world.living(), cpu.world.living(), "{name}");
            assert_eq!((gpu.larder, gpu.core), (cpu.larder, cpu.core), "{name}");
            let k = scale as f32 / 4.0;
            assert_eq!(cpu.larder, (m.items.landing.unwrap().0 * k, m.items.landing.unwrap().1 * k), "{name}: the squad lands on the map's landing");
            let c = cpu.world.crystals().iter().map(|c| c.0).min_by(|a, b| (a.0 - cpu.larder.0).hypot(a.1 - cpu.larder.1).total_cmp(&(b.0 - cpu.larder.0).hypot(b.1 - cpu.larder.1))).unwrap();
            assert!(cpu.apply(Cmd::Order(0, Order::Clear(c.0, c.1))) && gpu.apply(Cmd::Order(0, Order::Clear(c.0, c.1))));
            let mut sizes = Vec::new();
            while cpu.end.is_none() {
                cpu.step();
                gpu.step();
                if cpu.world.ticks() % 75 == 0 {
                    sizes.push((cpu.world.ticks() / 15, cpu.world.living(), gpu.world.living()));
                }
            }
            eprintln!("{name}: game s, living CPU, GPU: {sizes:?}; banked {} / {}", cpu.harvested, gpu.harvested);
            // Equal to f32 rounding: a partial load (a crystal running dry)
            // is a sum of takes the GPU adds in another order.
            assert!(cpu.harvested > 0.0 && (gpu.harvested - cpu.harvested).abs() <= 1e-3 * cpu.harvested, "{name}: banked {} on the GPU, {} on the CPU", gpu.harvested, cpu.harvested);
            for &(t, a, b) in &sizes {
                let (a, b) = (a as f64, b as f64);
                assert!((a - b).abs() <= 0.08 * a, "{name}: mould at {t} s: {b} walkers on the GPU, {a} on the CPU");
            }
            let left = |w: &dyn World| w.crystals().iter().map(|c| c.1 as f64).sum::<f64>();
            let (lc, lg) = (left(&cpu.world), left(&gpu.world));
            assert!((lc - lg).abs() <= 0.02 * lc, "{name}: crystal charge left {lg} on the GPU, {lc} on the CPU");
        }
    }

    /// A diagnostic, not a gate (`--ignored --nocapture`): the adapter's
    /// float maths beside the CPU's, on the operations the walk and the
    /// fields use. On 2026-11-12 the GTX 1070 left 19 of 24,822 walkers a
    /// cell off the CPU's after four ticks of the life-cycle twin (lavapipe:
    /// 0), and the one-seed game twins drifted 18% apart; this says which
    /// operation differs and how often a walker's step lands in another
    /// cell. `a * b + c` is compared with the CPU's unfused and fused
    /// (`mul_add`) results, as a driver may contract it into one FMA.
    #[test]
    #[ignore]
    fn the_adapters_maths_beside_the_cpus() {
        let Some((device, queue)) = test_gpu() else { return };
        const SRC: &str = "
@group(0) @binding(0) var<storage, read> inp: array<f32>;
@group(0) @binding(1) var<storage, read_write> outp: array<f32>;
@compute @workgroup_size(256)
fn main(@builtin(global_invocation_id) g: vec3<u32>) {
    let i = g.x;
    if (8u * i >= arrayLength(&inp)) { return; }
    let x = inp[8u * i]; let y = inp[8u * i + 1u]; let h = inp[8u * i + 2u]; let st = inp[8u * i + 3u];
    let v = inp[8u * i + 4u]; let a = inp[8u * i + 5u]; let b = inp[8u * i + 6u]; let c = inp[8u * i + 7u];
    outp[8u * i] = cos(h);
    outp[8u * i + 1u] = sin(h);
    outp[8u * i + 2u] = exp(-v);
    outp[8u * i + 3u] = pow(a, 0.7);
    outp[8u * i + 4u] = sqrt(v);
    outp[8u * i + 5u] = x + cos(h) * st;
    outp[8u * i + 6u] = y + sin(h) * st;
    outp[8u * i + 7u] = a * b + c;
}";
        let n = 1usize << 20;
        let mut seed = 0x2545_F491_4F6C_DD1Du64;
        let mut unit = || {
            seed ^= seed << 13;
            seed ^= seed >> 7;
            seed ^= seed << 17;
            (seed >> 40) as f32 / (1u64 << 24) as f32
        };
        let mut inp = Vec::with_capacity(8 * n);
        for _ in 0..n {
            // A walker's place on a 512 x 320 dish, a heading as the walk
            // keeps it (turns add to it unwrapped), a step, a field value.
            inp.extend([unit() * 512.0, unit() * 320.0, (unit() - 0.5) * 40.0, 0.5 + unit(), unit() * 20.0, unit(), unit() * 4.0, unit() * 64.0]);
        }
        use wgpu::util::DeviceExt;
        let ib = device.create_buffer_init(&wgpu::util::BufferInitDescriptor { label: Some("maths in"), contents: &f32_bytes(&inp), usage: wgpu::BufferUsages::STORAGE });
        let ob = device.create_buffer(&wgpu::BufferDescriptor { label: Some("maths out"), size: (inp.len() * 4) as u64, usage: wgpu::BufferUsages::STORAGE | wgpu::BufferUsages::COPY_SRC, mapped_at_creation: false });
        let module = device.create_shader_module(wgpu::ShaderModuleDescriptor { label: Some("maths"), source: wgpu::ShaderSource::Wgsl(SRC.into()) });
        let pipe = device.create_compute_pipeline(&wgpu::ComputePipelineDescriptor { label: Some("maths"), layout: None, module: &module, entry_point: Some("main"), compilation_options: Default::default(), cache: None });
        let bg = device.create_bind_group(&wgpu::BindGroupDescriptor {
            label: Some("maths"),
            layout: &pipe.get_bind_group_layout(0),
            entries: &[wgpu::BindGroupEntry { binding: 0, resource: ib.as_entire_binding() }, wgpu::BindGroupEntry { binding: 1, resource: ob.as_entire_binding() }],
        });
        let mut enc = device.create_command_encoder(&Default::default());
        {
            let mut pass = enc.begin_compute_pass(&Default::default());
            pass.set_pipeline(&pipe);
            pass.set_bind_group(0, &bg, &[]);
            pass.dispatch_workgroups((n as u32).div_ceil(256), 1, 1);
        }
        queue.submit([enc.finish()]);
        let out = floats(&read_back(&device, &queue, &ob));
        let names = ["cos", "sin", "exp", "pow", "sqrt", "x + cos*st", "y + sin*st", "a*b + c"];
        let mut diff = [0usize; 8];
        let mut worst = [0f32; 8];
        let (mut cells_off, mut fused_same, mut unfused_same) = (0usize, 0usize, 0usize);
        for k in 0..n {
            let q = &inp[8 * k..8 * k + 8];
            let (x, y, h, st, v, a, b, c) = (q[0], q[1], q[2], q[3], q[4], q[5], q[6], q[7]);
            let cpu = [h.cos(), h.sin(), (-v).exp(), a.powf(0.7), v.sqrt(), x + h.cos() * st, y + h.sin() * st, a * b + c];
            let gpu = &out[8 * k..8 * k + 8];
            for j in 0..8 {
                if gpu[j].to_bits() != cpu[j].to_bits() {
                    diff[j] += 1;
                    worst[j] = worst[j].max((gpu[j] - cpu[j]).abs());
                }
            }
            if gpu[5].floor() != cpu[5].floor() || gpu[6].floor() != cpu[6].floor() {
                cells_off += 1;
            }
            fused_same += (gpu[7].to_bits() == a.mul_add(b, c).to_bits()) as usize;
            unfused_same += (gpu[7].to_bits() == (a * b + c).to_bits()) as usize;
        }
        for j in 0..8 {
            eprintln!("{:>11}: {} of {n} differ from the CPU's bits, worst by {:.2e}", names[j], diff[j], worst[j]);
        }
        eprintln!("a step lands in another cell for {cells_off} of {n}; a*b + c matches the CPU's unfused result {unfused_same} times and the fused {fused_same}");
    }

    /// A diagnostic, not a gate (`--ignored --nocapture`): whether the
    /// adapter's game leans one way against the CPU's. The technical lead,
    /// 2026-11-12: the campaign's 32-seed The gaps run had the 1070 end
    /// unwalled games sooner on 24 of 32 seeds (p 0.007), and the fleeing
    /// twin killed 908 on the GPU against 765 on the CPU; rounding noise
    /// should change sign from seed to seed, while the 1070's always-fused
    /// `a * b + c` (`the_adapters_maths_beside_the_cpus`) rounds every
    /// deposit and decay the same way. Eight seeds of the race world with an
    /// idle squad on both engines; every 5 game s, the trail's total and the
    /// living walkers, and how many seeds have the GPU above the CPU (with a
    /// two-sided sign test). The 1070 at 8 seeds: no lean that survives
    /// correction (trail below the CPU's on 7 of 8 at 30 s, p 0.07; lavapipe
    /// 5 of 8), so `DRIFT_SEEDS=32` for the answer.
    #[test]
    #[ignore]
    fn the_adapters_game_beside_the_cpus_over_seeds() {
        use slime_mould::gpu::larder_world::{race_settings, GpuWorld};
        use slime_mould::mould::larder_world::{race_params, Game, Rules, World};
        let Some((device, queue)) = test_gpu() else { return };
        let sign_p = |a: usize, b: usize| -> f64 {
            let n = a + b;
            let k = a.min(b);
            let (mut c, mut sum) = (1f64, 0f64);
            for i in 0..=k {
                if i > 0 {
                    c *= (n - i + 1) as f64 / i as f64;
                }
                sum += c;
            }
            (2.0 * sum / 2f64.powi(n as i32)).min(1.0)
        };
        let (scale, walkers) = (2, 5_000);
        let rules = Rules { cap: 30.0, ..Rules::default() };
        // A mark: game s, CPU trail, GPU trail, CPU living, GPU living.
        type Mark = (u64, f64, f64, usize, usize);
        let mut rows: Vec<Vec<Mark>> = Vec::new();
        // `DRIFT_SEEDS=N` for more (the 1070 plays 8 in about 30 s).
        let seeds = std::env::var("DRIFT_SEEDS").ok().and_then(|v| v.parse::<u64>().ok()).unwrap_or(8);
        for seed in 1..=seeds {
            let mut cpu = Game::new(slime_mould::mould::big::BigDish::new(race_params(seed, scale, walkers)), rules, seed);
            let mut gpu = Game::new(GpuWorld::new(&device, &queue, race_settings(seed, scale, walkers), scale), rules, seed);
            let mut r = Vec::new();
            while cpu.end.is_none() && gpu.end.is_none() {
                cpu.step();
                gpu.step();
                if cpu.world.ticks() % 75 == 0 {
                    let cells = cpu.world.trail.len();
                    let side = (gpu.world.e.done % 2) as usize;
                    let gt: f64 = floats(&read_back(&device, &queue, &gpu.world.e.trail[side]))[..cells].iter().map(|&v| v as f64).sum();
                    let ct: f64 = cpu.world.trail.iter().map(|&v| v as f64).sum();
                    r.push((cpu.world.ticks() / 15, ct, gt, cpu.world.living(), gpu.world.living()));
                }
            }
            eprintln!("seed {seed}: game s, trail CPU, GPU, living CPU, GPU: {:?}", r.iter().map(|&(t, a, b, c, d)| (t, a.round(), b.round(), c, d)).collect::<Vec<_>>());
            rows.push(r);
        }
        let marks = rows.iter().map(Vec::len).min().unwrap_or(0);
        for m in 0..marks {
            let t = rows[0][m].0;
            let (tu, td) = (rows.iter().filter(|r| r[m].2 > r[m].1).count(), rows.iter().filter(|r| r[m].2 < r[m].1).count());
            let (lu, ld) = (rows.iter().filter(|r| r[m].4 > r[m].3).count(), rows.iter().filter(|r| r[m].4 < r[m].3).count());
            let ratio: Vec<f64> = rows.iter().map(|r| r[m].2 / r[m].1.max(1e-9)).collect();
            let mut sorted = ratio.clone();
            sorted.sort_by(f64::total_cmp);
            eprintln!("at {t} s: GPU trail above the CPU's on {tu} seeds, below on {td} (sign p {:.3}; median ratio {:.4}); GPU mould larger on {lu}, smaller on {ld} (p {:.3})", sign_p(tu, td), sorted[sorted.len() / 2], sign_p(lu, ld));
        }
    }
}
