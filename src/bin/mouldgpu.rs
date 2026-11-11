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
//!   endemic, sick ropes).
//! - Tab turns the panel's page: MAIN (the comparison's controls), LOOKS
//!   (start shape, the settings the trail sets, the deposit cap, the sensing
//!   floor, wander) and CONTAGION (sickness, immunity, the germ field).
//! - Up / Down picks a control, Left / Right changes it (Shift: bigger steps).
//! - Space pauses, `.` steps one tick, R restarts, N restarts on a new seed.
//! - Left drag pulls walkers to the mouse (they jump there, as respawned
//!   walkers do; Jenson's 2020 performance spawned particles with a stylus).
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
//!
//! Every control is drawn straight from the simulation (the owner's rule);
//! the panel and the line at the bottom are the only things on screen that
//! are not.

use std::sync::atomic::{AtomicBool, Ordering};
use std::sync::Arc;
use std::time::Instant;

use pixels::{wgpu, Pixels, PixelsBuilder, ScalingMode, SurfaceTexture};
use slime_mould::hud;
use slime_mould::mould::big::{BigDish, BigParams};
use slime_mould::mould::STEPS_PER_SECOND;
use winit::application::ApplicationHandler;
use winit::dpi::PhysicalSize;
use winit::event::{ElementState, MouseButton, WindowEvent};
use winit::event_loop::{ActiveEventLoop, ControlFlow, EventLoop};
use winit::keyboard::{KeyCode, PhysicalKey};
use winit::window::{Window, WindowId};

const SHADER: &str = r#"
struct Params {
    w: u32, h: u32, n: u32, crowd: u32,
    s_turn: u32, s_blocked: u32, s_claim: u32, s_jump: u32,
    sa: f32, ra: f32, so: f32, sso: f32,
    st: f32, deposit: f32, keep: f32, scent_keep: f32,
    respawn: f32, pull: f32, bx: f32, by: f32,
    br: f32, knee: f32, gamma: f32, s_pull: u32,
    wipe: u32, colour: u32, field: u32, curve: u32,
    tube: f32, fed: f32, cap: u32, s_wander: u32,
    // Jenson's looks (`BigParams`, 2026-11-11): settings set by the trail a
    // walker stands in (`sense_scale`, each `*_mod` as amp and exponent),
    // the sensing floor and wander.
    ss: f32, om_a: f32, om_e: f32, am_a: f32,
    am_e: f32, tm_a: f32, tm_e: f32, sm_a: f32,
    sm_e: f32, sfloor: f32, wander: f32, catch_at: f32,
    // Contagion: off while `sick_steps` is 0. `infect` makes every walker
    // within `ir` of (`ix`, `iy`) sick at the tick's start.
    sick_steps: u32, immune: u32, infect: u32, _p0: u32,
    sick_dep: f32, sick_keep: f32, sick_speed: f32, sick_trail: f32,
    sick_reach: f32, sick_turn: f32, ix: f32, iy: f32,
    ir: f32, _p1: f32, _p2: f32, _p3: f32,
};
struct State { t: u32, top: f32, _a: u32, _b: u32 };

@group(0) @binding(0) var<uniform> P: Params;
@group(0) @binding(1) var<storage, read_write> S: State;
@group(0) @binding(2) var<storage, read_write> walkers: array<vec4<f32>>;
@group(0) @binding(3) var<storage, read_write> intent: array<vec4<f32>>;
@group(0) @binding(4) var<storage, read_write> icell: array<u32>;
@group(0) @binding(5) var<storage, read_write> occ: array<atomic<u32>>;
@group(0) @binding(6) var<storage, read_write> claim: array<atomic<u32>>;
@group(0) @binding(7) var<storage, read_write> dep: array<atomic<u32>>;
@group(0) @binding(8) var<storage, read> agar: array<u32>;
@group(0) @binding(9) var<storage, read> lay: array<f32>;
@group(0) @binding(10) var<storage, read_write> trail_in: array<f32>;
@group(0) @binding(11) var<storage, read_write> trail_out: array<f32>;
@group(0) @binding(12) var<storage, read_write> scent_in: array<f32>;
@group(0) @binding(13) var<storage, read_write> scent_out: array<f32>;
@group(0) @binding(14) var<storage, read_write> hist: array<atomic<u32>>;
@group(0) @binding(15) var<storage, read_write> frame: array<u32>;
@group(0) @binding(16) var<storage, read> hud: array<u32>;
// Which pile a cell's food belongs to (index + 1, 0 for none), and whether
// the tube has reached each pile (the CPU's `BigDish::reached`).
@group(0) @binding(17) var<storage, read> pile: array<u32>;
@group(0) @binding(18) var<storage, read_write> reached: array<atomic<u32>>;
// Contagion (the CPU's `BigDish::sick` and `germ`): each walker's counter
// (0 healthy, over `immune` sick, 1..=`immune` immune), sick walkers' trail
// a cell (`sdep`, as `dep` counts the rest), sick walkers a cell for the
// germ (`gdep`), and the germ field.
@group(0) @binding(19) var<storage, read_write> sick: array<u32>;
@group(0) @binding(20) var<storage, read_write> sdep: array<atomic<u32>>;
@group(0) @binding(21) var<storage, read_write> gdep: array<atomic<u32>>;
@group(0) @binding(22) var<storage, read_write> germ_in: array<f32>;
@group(0) @binding(23) var<storage, read_write> germ_out: array<f32>;

const NONE: u32 = 0xFFFFFFFFu;
const TAU: f32 = 6.2831855;

fn lowbias32(v: u32) -> u32 {
    var x = v;
    x ^= x >> 16u; x *= 0x7FEB352Du; x ^= x >> 15u; x *= 0x846CA68Bu;
    return x ^ (x >> 16u);
}
// The CPU's `keyed`, with the seed and use folded into `s` on the CPU.
fn keyed(s: u32, t: u32, walker: u32) -> u32 {
    return lowbias32(walker ^ lowbias32(lowbias32(s) ^ t));
}
fn unit(x: u32) -> f32 { return f32(x >> 8u) / 16777216.0; }

fn cell(x: f32, y: f32) -> u32 {
    if (x < 0.0 || y < 0.0) { return NONE; }
    let cx = u32(x); let cy = u32(y);
    if (cx >= P.w || cy >= P.h) { return NONE; }
    return cy * P.w + cx;
}
fn scent_at(x: f32, y: f32) -> f32 {
    let i = cell(x, y);
    if (i == NONE) { return 0.0; }
    return scent_in[i];
}
fn sense_at(x: f32, y: f32, a: f32, so: f32) -> f32 {
    let c = cos(a); let s = sin(a);
    let i = cell(x + c * so, y + s * so);
    if (i == NONE || agar[i] == 0u) { return -1.0e6; }
    return trail_in[i] + scent_at(x + c * P.sso, y + s * P.sso);
}

// v^e as the CPU's `powf` gives it at v = 0 (0, or 1 when e is 0); WGSL's
// `pow` leaves 0 undefined.
fn pow0(v: f32, e: f32) -> f32 {
    if (v <= 0.0) { return select(0.0, 1.0, e == 0.0); }
    return pow(v, e);
}

@compute @workgroup_size(256)
fn sense(@builtin(global_invocation_id) gid: vec3<u32>) {
    let k = gid.x;
    if (k >= P.n) { return; }
    let t = S.t;
    let wk = walkers[k];
    var h = wk.z;
    let here = cell(wk.x, wk.y);
    // The CPU's `move_walkers` order: trail-set settings from the trail
    // where the walker stands, then catching and the sick walker's changes.
    var so = P.so; var sa = P.sa; var ra = P.ra; var st = P.st;
    if (P.ss > 0.0) {
        let v = min(trail_in[here] * P.ss, 1.0);
        so = so + P.om_a * pow0(v, P.om_e);
        sa = sa + P.am_a * pow0(v, P.am_e);
        ra = ra + P.tm_a * pow0(v, P.tm_e);
        st = max(st + P.sm_a * pow0(v, P.sm_e), 0.0);
    }
    if (P.sick_steps > 0u) {
        var c = sick[k];
        if (P.infect != 0u) {
            let dx = wk.x - P.ix; let dy = wk.y - P.iy;
            if (dx * dx + dy * dy <= P.ir * P.ir) { c = P.sick_steps + P.immune; }
        }
        if (c == 0u && germ_in[here] > P.catch_at) { c = P.sick_steps + P.immune; }
        sick[k] = c;
        if (c > P.immune) { st *= P.sick_speed; so *= P.sick_reach; ra *= P.sick_turn; }
    }
    let f = sense_at(wk.x, wk.y, h, so);
    let fl = sense_at(wk.x, wk.y, h - sa, so);
    let fr = sense_at(wk.x, wk.y, h + sa, so);
    if (P.wander > 0.0 && unit(keyed(P.s_wander, t, k | 0x80000000u)) < P.wander) {
        h = unit(keyed(P.s_wander, t, k | 0x40000000u)) * TAU;
    } else if (P.sfloor > 0.0 && max(max(f, fl), fr) < P.sfloor) {
        if ((keyed(P.s_wander, t, k) & 1u) == 0u) { h += ra; } else { h -= ra; }
    } else if (f > fl && f > fr) {
    } else if (f < fl && f < fr) {
        if ((keyed(P.s_turn, t, k) & 1u) == 0u) { h += ra; } else { h -= ra; }
    } else if (fl < fr) {
        h += ra;
    } else if (fr < fl) {
        h -= ra;
    }
    let nx = wk.x + cos(h) * st;
    let ny = wk.y + sin(h) * st;
    let there = cell(nx, ny);
    var aim = NONE;
    if (there != NONE) {
        if (there == here) {
            aim = there;
        } else if (agar[there] != 0u && (P.crowd == 0u || atomicLoad(&occ[there]) < P.crowd)) {
            aim = there;
            if (P.crowd > 0u) { atomicMin(&claim[there], keyed(P.s_claim, t, k)); }
        }
    }
    intent[k] = vec4<f32>(nx, ny, h, 0.0);
    icell[k] = aim;
}

@compute @workgroup_size(256)
fn walk(@builtin(global_invocation_id) gid: vec3<u32>) {
    let k = gid.x;
    if (k >= P.n) { return; }
    let t = S.t;
    let aim = icell[k];
    let wk = walkers[k];
    let here = cell(wk.x, wk.y);
    var wins = false;
    if (aim != NONE) {
        wins = aim == here || P.crowd == 0u || atomicLoad(&claim[aim]) == keyed(P.s_claim, t, k);
    }
    var at = here;
    if (wins) {
        if (aim != here) {
            atomicSub(&occ[here], 1u);
            atomicAdd(&occ[aim], 1u);
        }
        walkers[k] = intent[k];
        at = aim;
    } else {
        walkers[k] = vec4<f32>(wk.x, wk.y, unit(keyed(P.s_blocked, t, k)) * TAU, 0.0);
    }
    // Trail: a walker that moved lays it where it landed; with the capped
    // square-root deposit every walker counts where it stands after the
    // move (`trail` shares the cell's deposit among them). Sick walkers are
    // counted apart, for `sick_trail`.
    var ill = false;
    if (P.sick_steps > 0u) { ill = sick[k] > P.immune; }
    if (P.cap > 0u || wins) {
        if (ill) { atomicAdd(&sdep[at], 1u); } else { atomicAdd(&dep[at], 1u); }
    }
    // Sick walkers lay germ where they stand, then every counter falls by
    // one (before any jump, as on the CPU).
    if (P.sick_steps > 0u) {
        if (ill) { atomicAdd(&gdep[at], 1u); }
        let c = sick[k];
        if (c > 0u) { sick[k] = c - 1u; }
    }
}

// Where walker k jumps this tick, if it does (w = 1): the pull (to a disc
// at the mouse) first, then respawn (anywhere), each by its own keyed
// draws; the heading by respawn's (the CPU's `respawn_walkers`).
fn jump_of(k: u32, t: u32) -> vec4<f32> {
    var at = vec2<f32>(0.0, 0.0);
    if (P.pull > 0.0 && unit(keyed(P.s_pull, t, 4u * k)) < P.pull) {
        let a = unit(keyed(P.s_pull, t, 4u * k + 1u)) * TAU;
        let r = sqrt(unit(keyed(P.s_pull, t, 4u * k + 2u))) * P.br;
        at = vec2<f32>(P.bx + cos(a) * r, P.by + sin(a) * r);
    } else if (P.respawn > 0.0 && unit(keyed(P.s_jump, t, 4u * k)) < P.respawn) {
        at = vec2<f32>(unit(keyed(P.s_jump, t, 4u * k + 1u)) * f32(P.w), unit(keyed(P.s_jump, t, 4u * k + 2u)) * f32(P.h));
    } else {
        return vec4<f32>(0.0, 0.0, 0.0, 0.0);
    }
    return vec4<f32>(at, unit(keyed(P.s_jump, t, 4u * k + 3u)) * TAU, 1.0);
}

// Jumps come after the move, as on the CPU: a walker steps (and lays its
// trail where it stepped), then may jump. A jump off the agar is lost. With
// no crowd limit it lands here; with one, it claims its cell (lowest walker
// wins, the CPU's order) and `land` moves the winners. Only dispatched while
// respawn or the pull is on.
@compute @workgroup_size(256)
fn jump(@builtin(global_invocation_id) gid: vec3<u32>) {
    let k = gid.x;
    if (k >= P.n) { return; }
    icell[k] = NONE;
    let j = jump_of(k, S.t);
    if (j.w == 0.0) { return; }
    let to = cell(j.x, j.y);
    if (to == NONE || agar[to] == 0u) { return; }
    let wk = walkers[k];
    let here = cell(wk.x, wk.y);
    if (P.crowd == 0u) {
        if (to != here) {
            atomicSub(&occ[here], 1u);
            atomicAdd(&occ[to], 1u);
        }
        walkers[k] = vec4<f32>(j.xyz, 0.0);
        return;
    }
    // A cell claimed by a step this tick now holds its winner, so it is
    // full and never claimed here: the claim's stale keys cannot win.
    if (atomicLoad(&occ[to]) >= P.crowd) { return; }
    atomicMin(&claim[to], k);
    icell[k] = to;
}

@compute @workgroup_size(256)
fn land(@builtin(global_invocation_id) gid: vec3<u32>) {
    let k = gid.x;
    if (k >= P.n) { return; }
    let to = icell[k];
    if (to == NONE || atomicLoad(&claim[to]) != k) { return; }
    let wk = walkers[k];
    atomicSub(&occ[cell(wk.x, wk.y)], 1u);
    atomicAdd(&occ[to], 1u);
    walkers[k] = vec4<f32>(jump_of(k, S.t).xyz, 0.0);
}

// The trail a cell gets this tick: `deposit` a walker that moved (the
// sick lay `sick_trail` times as much), or with `cap` the CPU's capped
// square-root deposit, deposit x sqrt(min(n, cap)) shared by the cell's n
// walkers.
fn laid(j: u32) -> f32 {
    let a = f32(atomicLoad(&dep[j]));
    if (P.cap == 0u && P.sick_steps == 0u) { return P.deposit * a; }
    var b = 0.0;
    if (P.sick_steps > 0u) { b = f32(atomicLoad(&sdep[j])); }
    let w = a + P.sick_trail * b;
    if (P.cap == 0u) { return P.deposit * w; }
    let n = a + b;
    if (n == 0.0) { return 0.0; }
    return P.deposit * sqrt(min(n, f32(P.cap))) / n * w;
}

@compute @workgroup_size(256)
fn trail(@builtin(global_invocation_id) gid: vec3<u32>) {
    let i = gid.x;
    if (i >= P.w * P.h) { return; }
    // The germ spreads as the trail does, with its own keep.
    let germs = P.sick_steps > 0u;
    // A pile is reached once tube lies on it: the CPU marks it at the end
    // of a tick from that tick's trail, which is this tick's `trail_in`, so
    // the scent pass below lays this tick's food as the CPU's next tick does.
    let pi = pile[i];
    if (pi != 0u && trail_in[i] > P.tube) { atomicMax(&reached[pi - 1u], 1u); }
    let x = i % P.w; let y = i / P.w;
    var gone = agar[i] == 0u || x == 0u || y == 0u || x + 1u == P.w || y + 1u == P.h;
    // The right mouse wipes trail and germ, as the looks report's knife.
    if (!gone && P.wipe != 0u) {
        let dx = f32(x) + 0.5 - P.bx; let dy = f32(y) + 0.5 - P.by;
        gone = dx * dx + dy * dy <= P.br * P.br;
    }
    if (gone) {
        trail_out[i] = 0.0;
        if (germs) { germ_out[i] = 0.0; }
        return;
    }
    var sum = 0.0; var gsum = 0.0; var n = 0.0;
    for (var dy = 0u; dy < 3u; dy++) {
        for (var dx = 0u; dx < 3u; dx++) {
            let j = (y + dy - 1u) * P.w + x + dx - 1u;
            if (agar[j] != 0u) {
                sum += trail_in[j] + laid(j);
                if (germs) { gsum += germ_in[j] + P.sick_dep * f32(atomicLoad(&gdep[j])); }
                n += 1.0;
            }
        }
    }
    trail_out[i] = sum / n * P.keep;
    if (germs) { germ_out[i] = gsum / n * P.sick_keep; }
}

@compute @workgroup_size(256)
fn scent(@builtin(global_invocation_id) gid: vec3<u32>) {
    let i = gid.x;
    if (i >= P.w * P.h) { return; }
    atomicStore(&dep[i], 0u);
    atomicStore(&claim[i], NONE);
    if (P.sick_steps > 0u) {
        atomicStore(&sdep[i], 0u);
        atomicStore(&gdep[i], 0u);
    }
    if (i == 0u) { S.t = S.t + 1u; }
    let x = i % P.w; let y = i / P.w;
    if (agar[i] == 0u || x == 0u || y == 0u || x + 1u == P.w || y + 1u == P.h) { scent_out[i] = 0.0; return; }
    var sum = 0.0; var n = 0.0;
    for (var dy = 0u; dy < 3u; dy++) {
        for (var dx = 0u; dx < 3u; dx++) {
            let j = (y + dy - 1u) * P.w + x + dx - 1u;
            if (agar[j] != 0u) {
                // A reached pile lays `fed` of its scent (the CPU's `fed_scent`).
                var l = lay[j];
                if (l != 0.0 && pile[j] != 0u && atomicLoad(&reached[pile[j] - 1u]) != 0u) { l *= P.fed; }
                sum += scent_in[j] + l;
                n += 1.0;
            }
        }
    }
    scent_out[i] = sum / n * P.scent_keep;
}

// What a pixel is drawn from: the trail, or walkers a cell.
fn shown(i: u32) -> f32 {
    if (P.field == 1u) { return f32(atomicLoad(&occ[i])); }
    return trail_in[i];
}

// The soft curve's reference, p99.9 of the shown field over cells above 0:
// a 512-bin histogram of log2 (8 bins an octave from 2^-16 to 2^48), summed
// in workgroup memory first so the global bins see one add a workgroup. (It
// had 256 bins, to 2^16, until the review of 2026-11-11: a trail laid at
// 2,000 passed that and 0.66% of cells clipped to full brightness.)
const BINS: u32 = 512u;
var<workgroup> local: array<atomic<u32>, 512>;

@compute @workgroup_size(256)
fn histogram(@builtin(global_invocation_id) gid: vec3<u32>, @builtin(local_invocation_index) li: u32) {
    atomicStore(&local[li], 0u);
    atomicStore(&local[li + 256u], 0u);
    workgroupBarrier();
    let i = gid.x;
    if (i < P.w * P.h) {
        let v = shown(i);
        if (v > 0.0) {
            let b = u32(clamp((log2(v) + 16.0) * 8.0, 0.0, f32(BINS - 1u)));
            atomicAdd(&local[b], 1u);
        }
    }
    workgroupBarrier();
    for (var j = li; j < BINS; j += 256u) {
        let c = atomicLoad(&local[j]);
        if (c > 0u) { atomicAdd(&hist[j], c); }
    }
}

@compute @workgroup_size(1)
fn top() {
    var total = 0u;
    for (var j = 0u; j < BINS; j++) { total += atomicLoad(&hist[j]); }
    let want = max(total / 1000u, 1u);
    var seen = 0u;
    var b = BINS;
    loop {
        if (b == 0u) { break; }
        b -= 1u;
        seen += atomicLoad(&hist[b]);
        if (seen >= want) { break; }
    }
    // The bin's upper edge.
    S.top = exp2((f32(b) + 1.0) / 8.0 - 16.0);
    for (var j = 0u; j < BINS; j++) { atomicStore(&hist[j], 0u); }
}

@compute @workgroup_size(256)
fn draw(@builtin(global_invocation_id) gid: vec3<u32>) {
    let i = gid.x;
    if (i >= P.w * P.h) { return; }
    let v = shown(i);
    var g = 0.0;
    if (v > 0.0) {
        if (P.curve == 0u) {
            g = 1.0 - exp(-v / P.knee);
        } else {
            g = pow(clamp(v / S.top, 0.0, 1.0), P.gamma);
        }
    }
    var c = vec3<f32>(0.0, 0.0, 0.0);
    if (agar[i] == 0u) {
        if (P.colour == 0u) { c = vec3<f32>(8.0, 14.0, 18.0); }
    } else if (P.colour == 0u) {
        // The game's amber (`BigDish::image`): scent tints the ground.
        let sg = 1.0 - exp(-scent_in[i] / 60.0);
        let base = vec3<f32>(12.0 + 30.0 * sg, 22.0 + 26.0 * sg, 26.0 + 10.0 * sg);
        c = base + (vec3<f32>(255.0, 190.0, 90.0) - base) * g;
    } else {
        c = vec3<f32>(255.0 * g);
    }
    // With contagion on, the germ in red, full at four times the catch
    // level (the looks report's `style=sick`).
    if (P.sick_steps > 0u && agar[i] != 0u) {
        let gs = sqrt(clamp(germ_in[i] / (4.0 * max(P.catch_at, 1.0e-6)), 0.0, 1.0));
        c = vec3<f32>(max(c.x, 255.0 * gs), c.y * (1.0 - 0.8 * gs), c.z * (1.0 - 0.8 * gs));
    }
    // The panel, premultiplied, over the dish.
    let o = hud[i];
    let a = f32(o >> 24u) / 255.0;
    let hc = vec3<f32>(f32(o & 255u), f32((o >> 8u) & 255u), f32((o >> 16u) & 255u));
    c = hc + c * (1.0 - a);
    let q = vec3<u32>(clamp(c, vec3<f32>(0.0), vec3<f32>(255.0)));
    frame[i] = q.x | (q.y << 8u) | (q.z << 16u) | (255u << 24u);
}
"#;

/// The CPU's `keyed` stream key for a use (`mould::keyed`'s first line).
fn stream(seed: u64, use_: u32) -> u32 {
    (seed as u32) ^ ((seed >> 32) as u32) ^ use_.wrapping_mul(0x9E37_79B9)
}

// Draw uses, as in big.rs (106 is the CPU's respawn).
const DRAW_TURN: u32 = 101;
const DRAW_BLOCKED: u32 = 102;
const CLAIM_MOVE: u32 = 103;
const DRAW_JUMP: u32 = 106;
const DRAW_PULL: u32 = 107;
/// The sensing floor's and wander's draws (big.rs `DRAW_WANDER`).
const DRAW_WANDER: u32 = 108;

/// Bytes in `Params` (60 words).
const PARAMS_BYTES: usize = 240;

/// Piles the GPU tracks (the dish's seven and food dropped with F); F does
/// nothing once they are all used.
const MAX_PILES: usize = 64;

/// The share of walkers the pull moves a tick (0.2%: at 100,000 walkers, a
/// quarter of them in about a second of holding).
const PULL: f32 = 0.002;

#[derive(Clone, Copy, PartialEq, Debug)]
enum Colour {
    Amber,
    Grey,
}

#[derive(Clone, Copy, PartialEq, Debug)]
enum Field {
    Trail,
    Walkers,
}

#[derive(Clone, Copy, PartialEq, Debug)]
enum Curve {
    Game,
    Soft,
}

/// Where the walkers start: spread over the dish, packed at home (today's
/// inoculum), or a disc in the middle (`BigParams::blob_r`, the looks
/// report's trunk start).
#[derive(Clone, Copy, PartialEq, Debug)]
enum Start {
    Spread,
    Home,
    Disc,
}

/// Every control. Angles in degrees, distances in fine cells.
#[derive(Clone, PartialEq, Debug)]
struct Settings {
    walkers: usize,
    /// One walker a cell (crowd 1); dropped when there are more walkers than
    /// half the agar cells.
    crowd: bool,
    sensor_angle: f32,
    turn: f32,
    reach: f32,
    step: f32,
    /// Trail laid a step by 100,000 walkers: the deposit is
    /// `laid * 100,000 / walkers`.
    laid: f32,
    keep: f32,
    respawn: f32,
    /// The dish's seven piles (dropped food stays either way).
    food: bool,
    /// How far out food is smelled (`BigParams::scent_offset`).
    smell: f32,
    colour: Colour,
    field: Field,
    curve: Curve,
    knee: f32,
    gamma: f32,
    /// Ticks a frame.
    ticks: u32,
    start: Start,
    /// Jenson's looks (`Reports/jenson-looks-2026-11-11.md`, `BigParams`).
    /// The disc start's radius, and the share of walkers spread over the
    /// dish instead with a disc or home start (`haze`).
    disc: f32,
    haze: f32,
    /// Settings set by the trail a walker stands in (36 Points):
    /// s = min(trail x `trail_scale`, 1), 0 off; each setting is its base
    /// plus amp x s^power, `[amp, power]` (angles in degrees).
    trail_scale: f32,
    reach_by: [f32; 2],
    angle_by: [f32; 2],
    turn_by: [f32; 2],
    step_by: [f32; 2],
    /// The capped square-root deposit (`deposit_cap`), 0 off.
    cap: u16,
    /// The sensing floor and wander (chance a look), 0 off.
    floor: f32,
    wander: f32,
    /// Contagion, off when `sick` is 0: sick and immune for so many ticks,
    /// caught in germ over `catch_at`; a sick walker lays `germ` a tick
    /// (kept `germ_keep` a tick as it spreads) and moves, lays trail,
    /// senses and turns so many times as much.
    sick: u32,
    immune: u32,
    catch_at: f32,
    germ: f32,
    germ_keep: f32,
    sick_speed: f32,
    sick_trail: f32,
    sick_reach: f32,
    sick_turn: f32,
    /// The walkers within `seed_r` of the middle fall sick `seed_at`
    /// seconds in (0: at the start, `BigParams::sick_seed_r`).
    seed_at: f32,
    seed_r: f32,
    seed: u64,
}

impl Settings {
    /// The phase 2 big dish: the `nose` rule at 100,000 walkers, food on,
    /// the game's look (examples/bigdish.rs preset `dense`, spread start).
    fn big_dish() -> Self {
        let t = BigParams::today();
        Settings {
            walkers: 100_000,
            crowd: true,
            sensor_angle: t.sensor_angle.to_degrees(),
            turn: t.rotate_angle.to_degrees(),
            reach: t.sensor_offset,
            step: t.step,
            laid: t.deposit,
            keep: t.keep,
            respawn: 0.0,
            food: true,
            smell: t.sensor_offset * 4.0,
            colour: Colour::Amber,
            field: Field::Trail,
            curve: Curve::Game,
            knee: 6.0,
            gamma: 0.6,
            ticks: 2,
            start: Start::Spread,
            disc: 180.0,
            haze: 0.0,
            trail_scale: 0.0,
            reach_by: [0.0, 1.0],
            angle_by: [0.0, 1.0],
            turn_by: [0.0, 1.0],
            step_by: [0.0, 1.0],
            cap: 0,
            floor: 0.0,
            wander: 0.0,
            sick: 0,
            immune: 480,
            catch_at: 0.5,
            germ: 1.0,
            germ_keep: 0.9,
            sick_speed: 1.0,
            sick_trail: 1.0,
            sick_reach: 1.0,
            sick_turn: 1.0,
            seed_at: 4.0,
            seed_r: 40.0,
            seed: 1,
        }
    }

    /// Whether contagion is on.
    fn sick_on(&self) -> bool {
        self.sick > 0
    }
}

/// The presets: keys 1-8 the dishes of the 4x Mould page and the Jenson
/// comparison; 9, 0 and Shift 1-5 the looks report's (2026-11-11).
const PRESETS: [&str; 15] = [
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
];

/// The key a preset is on.
fn preset_key(k: usize) -> String {
    match k {
        0..=8 => (k + 1).to_string(),
        9 => "0".into(),
        _ => format!("SHIFT+{}", k - 9),
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
        _ => panic!("preset {k}: 0-14"),
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
}

const LOOKS: [Knob; 11] = [Knob::Disc, Knob::Haze, Knob::TrailScale, Knob::ReachBy, Knob::AngleBy, Knob::TurnBy, Knob::StepBy, Knob::TrailPower, Knob::Cap, Knob::Floor, Knob::Wander];
const SICK: [Knob; 11] = [Knob::Sick, Knob::Immune, Knob::Catch, Knob::Germ, Knob::GermKeep, Knob::SickSpeed, Knob::SickTrail, Knob::SickReach, Knob::SickTurn, Knob::SeedAt, Knob::SeedR];

/// The panel's pages (Tab turns them): the first holds the comparison's
/// controls, the others the looks report's (2026-11-11).
const PAGES: [(&str, &[Knob]); 3] = [("MAIN", &KNOBS), ("LOOKS", &LOOKS), ("CONTAGION", &SICK)];

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
                s.disc = (s.disc + 10.0 * k * d).clamp(10.0, 320.0);
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
        }
        false
    }
}

/// A switch on the command line: 1/on/true/yes or 0/off/false/no (until the
/// review of 2026-11-11 anything but 0 or off meant on, so `crowd=false`
/// turned the limit on).
fn on_off(key: &str, v: &str) -> Result<bool, String> {
    match v {
        "1" | "on" | "true" | "yes" => Ok(true),
        "0" | "off" | "false" | "no" => Ok(false),
        _ => Err(format!("{key}={v}: 1 or 0 (on, off)")),
    }
}

/// Sets a control from a `key=value` argument.
fn set(s: &mut Settings, key: &str, v: &str) -> Result<(), String> {
    let f = || v.parse::<f32>().map_err(|e| format!("{key}={v}: {e}"));
    let int = || v.parse::<u32>().map_err(|e| format!("{key}={v}: {e}"));
    // `amp` or `amp:power`, as examples/bigdish.rs takes the `*_mod`s.
    let pair = || -> Result<[f32; 2], String> {
        let (a, e) = v.split_once(':').unwrap_or((v, "1"));
        Ok([a.parse().map_err(|e| format!("{key}={v}: {e}"))?, e.parse().map_err(|e| format!("{key}={v}: {e}"))?])
    };
    let b = || on_off(key, v);
    let pick = |names: &[&str]| names.iter().position(|&n| n == v).ok_or_else(|| format!("{key}={v}: one of {}", names.join(", ")));
    match key {
        "walkers" => s.walkers = v.parse().map_err(|e| format!("{key}={v}: {e}"))?,
        "crowd" => s.crowd = b()?,
        "sensor_angle" => s.sensor_angle = f()?,
        "turn" | "rotate_angle" => s.turn = f()?,
        "reach" | "sensor_offset" => s.reach = f()?,
        "step" => s.step = f()?,
        "laid" => s.laid = f()?,
        "keep" => s.keep = f()?,
        "respawn" => s.respawn = f()?,
        "food" => s.food = b()?,
        "smell" | "scent_offset" => s.smell = f()?,
        "colour" | "color" => s.colour = [Colour::Amber, Colour::Grey, Colour::Grey][pick(&["amber", "grey", "gray"])?],
        "field" => s.field = [Field::Trail, Field::Walkers, Field::Walkers][pick(&["trail", "walkers", "count"])?],
        "curve" => s.curve = [Curve::Game, Curve::Soft][pick(&["game", "soft"])?],
        "knee" => s.knee = f()?,
        "gamma" => s.gamma = f()?,
        "ticks" => s.ticks = v.parse::<u32>().map_err(|e| format!("{key}={v}: {e}"))?.clamp(1, 32),
        "start" => s.start = [Start::Spread, Start::Home, Start::Home, Start::Disc][pick(&["spread", "blob", "home", "disc"])?],
        "disc" => s.disc = f()?,
        "haze" => s.haze = f()?,
        "trail_scale" | "sense_scale" => s.trail_scale = f()?,
        "reach_by" | "offset_mod" => s.reach_by = pair()?,
        "angle_by" | "angle_mod" => s.angle_by = pair()?,
        "turn_by" | "turn_mod" => s.turn_by = pair()?,
        "step_by" | "step_mod" => s.step_by = pair()?,
        "cap" | "deposit_cap" => s.cap = int()?.min(u16::MAX as u32) as u16,
        "floor" | "sense_floor" => s.floor = f()?,
        "wander" => s.wander = f()?,
        "sick" | "sick_steps" => s.sick = int()?.min(30_000),
        "immune" | "immune_steps" => s.immune = int()?.min(30_000),
        "catch" | "sick_catch" => s.catch_at = f()?,
        "germ" | "sick_deposit" => s.germ = f()?,
        "germ_keep" | "sick_keep" => s.germ_keep = f()?,
        "sick_speed" => s.sick_speed = f()?,
        "sick_trail" => s.sick_trail = f()?,
        "sick_reach" => s.sick_reach = f()?,
        "sick_turn" => s.sick_turn = f()?,
        "seed_at" => s.seed_at = f()?,
        "seed_r" => s.seed_r = f()?,
        "seed" => s.seed = v.parse().map_err(|e| format!("{key}={v}: {e}"))?,
        _ => return Err(format!("unknown control {key}")),
    }
    Ok(())
}

/// The CPU dish a sandbox dish starts from (walkers, agar, piles).
fn big_params(s: &Settings, scale: usize, crowd: bool) -> BigParams {
    let t = BigParams::today();
    BigParams {
        scale,
        walkers: s.walkers,
        sensor_angle: s.sensor_angle.to_radians(),
        rotate_angle: s.turn.to_radians(),
        sensor_offset: s.reach,
        scent_offset: s.smell,
        step: s.step,
        deposit: s.laid * 100_000.0 / s.walkers.max(1) as f32,
        keep: s.keep,
        crowd: crowd as u8,
        scent_coarse: false,
        blob: s.start != Start::Spread,
        respawn: s.respawn,
        blob_r: if s.start == Start::Disc { s.disc / scale as f32 } else { 0.0 },
        haze: if s.start == Start::Spread { 0.0 } else { s.haze },
        sense_scale: s.trail_scale,
        offset_mod: s.reach_by,
        angle_mod: [s.angle_by[0].to_radians(), s.angle_by[1]],
        turn_mod: [s.turn_by[0].to_radians(), s.turn_by[1]],
        step_mod: s.step_by,
        deposit_cap: s.cap,
        sense_floor: s.floor,
        wander: s.wander,
        sick_steps: s.sick as u16,
        immune_steps: s.immune as u16,
        sick_catch: s.catch_at,
        sick_deposit: s.germ,
        sick_keep: s.germ_keep,
        sick_speed: s.sick_speed,
        sick_trail: s.sick_trail,
        sick_reach: s.sick_reach,
        sick_turn: s.sick_turn,
        sick_seed_r: if s.seed_at <= 0.0 { s.seed_r / scale as f32 } else { 0.0 },
        seed: s.seed,
        ..t
    }
}

fn f32_bytes(v: &[f32]) -> Vec<u8> {
    v.iter().flat_map(|x| x.to_le_bytes()).collect()
}

fn u32_bytes(v: &[u32]) -> Vec<u8> {
    v.iter().flat_map(|x| x.to_le_bytes()).collect()
}

fn floats(bytes: &[u8]) -> Vec<f32> {
    bytes.chunks_exact(4).map(|c| f32::from_le_bytes([c[0], c[1], c[2], c[3]])).collect()
}

/// What the mouse is doing this frame, in fine cells.
#[derive(Clone, Copy, Default)]
struct Hand {
    at: Option<(f32, f32)>,
    pull: bool,
    wipe: bool,
    /// Make the walkers under the mouse sick (middle drag, or I once).
    infect: bool,
}

/// The shader, its layout and its nine pipelines: made once, kept across
/// restarts.
struct Shared {
    bgl: wgpu::BindGroupLayout,
    /// sense, walk, trail, scent, histogram, top, draw, jump, land.
    pipes: Vec<wgpu::ComputePipeline>,
}

const BINDINGS: u32 = 24;
const SENSE: usize = 0;
const WALK: usize = 1;
const TRAIL: usize = 2;
const SCENT: usize = 3;
const HISTOGRAM: usize = 4;
const TOP: usize = 5;
const DRAW: usize = 6;
const JUMP: usize = 7;
const LAND: usize = 8;

impl Shared {
    fn new(dev: &wgpu::Device) -> Self {
        let module = dev.create_shader_module(wgpu::ShaderModuleDescriptor { label: Some("sandbox"), source: wgpu::ShaderSource::Wgsl(SHADER.into()) });
        let entry = |binding: u32, ty: wgpu::BufferBindingType| wgpu::BindGroupLayoutEntry {
            binding,
            visibility: wgpu::ShaderStages::COMPUTE,
            ty: wgpu::BindingType::Buffer { ty, has_dynamic_offset: false, min_binding_size: None },
            count: None,
        };
        let rw = wgpu::BufferBindingType::Storage { read_only: false };
        let ro = wgpu::BufferBindingType::Storage { read_only: true };
        let mut entries = vec![entry(0, wgpu::BufferBindingType::Uniform)];
        for b in 1..BINDINGS {
            entries.push(entry(b, if matches!(b, 8 | 9 | 16 | 17) { ro } else { rw }));
        }
        let bgl = dev.create_bind_group_layout(&wgpu::BindGroupLayoutDescriptor { label: Some("sandbox"), entries: &entries });
        let layout = dev.create_pipeline_layout(&wgpu::PipelineLayoutDescriptor { label: Some("sandbox"), bind_group_layouts: &[Some(&bgl)], immediate_size: 0 });
        let pipes = ["sense", "walk", "trail", "scent", "histogram", "top", "draw", "jump", "land"]
            .iter()
            .map(|entry_point| {
                dev.create_compute_pipeline(&wgpu::ComputePipelineDescriptor {
                    label: Some(entry_point),
                    layout: Some(&layout),
                    module: &module,
                    entry_point: Some(entry_point),
                    compilation_options: Default::default(),
                    cache: None,
                })
            })
            .collect();
        Shared { bgl, pipes }
    }
}

/// One dish on the GPU.
struct Engine {
    device: wgpu::Device,
    queue: wgpu::Queue,
    /// Bind groups for even and odd ticks (trail and scent swap).
    groups: [wgpu::BindGroup; 2],
    params: wgpu::Buffer,
    walkers: wgpu::Buffer,
    /// Kept for the twin tests' `inject` and checks (the sandbox itself
    /// only binds them).
    #[cfg_attr(not(test), allow(dead_code))]
    state: wgpu::Buffer,
    #[cfg_attr(not(test), allow(dead_code))]
    occ: wgpu::Buffer,
    trail: [wgpu::Buffer; 2],
    #[cfg_attr(not(test), allow(dead_code))]
    scent: [wgpu::Buffer; 2],
    lay: wgpu::Buffer,
    pile: wgpu::Buffer,
    reached: wgpu::Buffer,
    frame: wgpu::Buffer,
    hud: wgpu::Buffer,
    /// Contagion: each walker's counter, and the germ field.
    sick: wgpu::Buffer,
    #[cfg_attr(not(test), allow(dead_code))]
    germ: [wgpu::Buffer; 2],
    /// Whether the timed seeding (`Settings::seed_at`) has happened.
    seeded: bool,
    /// The CPU dish it started from: agar, piles, and the measures.
    dish: BigDish,
    /// Food dropped with F, in fine cells.
    dropped: Vec<(f32, f32)>,
    n: u32,
    cells: u32,
    /// Ticks run.
    done: u64,
    /// Whether the crowd limit can hold (walkers at most half the agar).
    crowd_ok: bool,
    /// From the last `write_params`: whether the jump passes run (respawn
    /// or the pull on), and with the claim (`land`) too.
    jumps: bool,
    crowd: bool,
}

impl Engine {
    fn new(device: &wgpu::Device, queue: &wgpu::Queue, shared: &Shared, s: &Settings, scale: usize) -> Self {
        let agar_cells = BigDish::new(BigParams { walkers: 1, ..big_params(s, scale, false) }).agar.iter().filter(|&&a| a).count();
        let crowd_ok = s.walkers <= agar_cells / 2;
        let d = BigDish::new(big_params(s, scale, s.crowd && crowd_ok));
        let dev = device;
        let cells = (d.w * d.h) as u32;
        let n = d.walkers.len() as u32;
        let storage = wgpu::BufferUsages::STORAGE | wgpu::BufferUsages::COPY_DST | wgpu::BufferUsages::COPY_SRC;
        let buf = |label: &str, bytes: &[u8], usage: wgpu::BufferUsages| {
            let b = dev.create_buffer(&wgpu::BufferDescriptor { label: Some(label), size: bytes.len().max(16) as u64, usage, mapped_at_creation: false });
            queue.write_buffer(&b, 0, bytes);
            b
        };
        let zeros = |label: &str, bytes: usize| dev.create_buffer(&wgpu::BufferDescriptor { label: Some(label), size: bytes.max(16) as u64, usage: storage, mapped_at_creation: false });
        let params = buf("params", &[0u8; PARAMS_BYTES], wgpu::BufferUsages::UNIFORM | wgpu::BufferUsages::COPY_DST);
        let state = buf("state", &u32_bytes(&[0, 1f32.to_bits(), 0, 0]), storage);
        let flat: Vec<f32> = d.walkers.iter().flat_map(|&(x, y, h)| [x, y, h, 0.0]).collect();
        let walkers = buf("walkers", &f32_bytes(&flat), storage);
        let intent = zeros("intent", n as usize * 16);
        let icell = zeros("icell", n as usize * 4);
        let mut occ = vec![0u32; cells as usize];
        for &(x, y, _) in &d.walkers {
            occ[y as usize * d.w + x as usize] += 1;
        }
        let occ = buf("occ", &u32_bytes(&occ), storage);
        let claim = buf("claim", &u32_bytes(&vec![u32::MAX; cells as usize]), storage);
        let dep = zeros("dep", cells as usize * 4);
        let agar: Vec<u32> = d.agar.iter().map(|&a| a as u32).collect();
        let agar = buf("agar", &u32_bytes(&agar), storage);
        let lay = zeros("lay", cells as usize * 4);
        let pile = zeros("pile", cells as usize * 4);
        let mut reached = vec![0u32; MAX_PILES];
        for (r, &hit) in reached.iter_mut().zip(&d.reached) {
            *r = hit as u32;
        }
        let reached = buf("reached", &u32_bytes(&reached), storage);
        let trail = [zeros("trail0", cells as usize * 4), zeros("trail1", cells as usize * 4)];
        let scent = [zeros("scent0", cells as usize * 4), zeros("scent1", cells as usize * 4)];
        let hist = zeros("hist", 512 * 4);
        let frame = zeros("frame", cells as usize * 4);
        let hud = zeros("hud", cells as usize * 4);
        // Contagion's buffers are always there (a u32 a walker and three
        // fields), so it can be switched on as the dish runs.
        let counters: Vec<u32> = if d.sick.is_empty() { vec![0; n as usize] } else { d.sick.iter().map(|&c| c as u32).collect() };
        let sick = buf("sick", &u32_bytes(&counters), storage);
        let sdep = zeros("sdep", cells as usize * 4);
        let gdep = zeros("gdep", cells as usize * 4);
        let germ = [zeros("germ0", cells as usize * 4), zeros("germ1", cells as usize * 4)];
        let group = |even: bool| {
            let (a, b) = if even { (0, 1) } else { (1, 0) };
            let bufs: [&wgpu::Buffer; BINDINGS as usize] = [
                &params, &state, &walkers, &intent, &icell, &occ, &claim, &dep, &agar, &lay, &trail[a], &trail[b], &scent[a], &scent[b], &hist, &frame, &hud, &pile, &reached, &sick, &sdep, &gdep, &germ[a], &germ[b],
            ];
            let entries: Vec<wgpu::BindGroupEntry> = bufs.iter().enumerate().map(|(i, b)| wgpu::BindGroupEntry { binding: i as u32, resource: b.as_entire_binding() }).collect();
            dev.create_bind_group(&wgpu::BindGroupDescriptor { label: Some("sandbox"), layout: &shared.bgl, entries: &entries })
        };
        let groups = [group(true), group(false)];
        let mut e = Engine {
            device: device.clone(),
            queue: queue.clone(),
            groups,
            params,
            walkers,
            state,
            occ,
            trail,
            scent,
            lay,
            pile,
            reached,
            frame,
            hud,
            sick,
            germ,
            seeded: false,
            dish: d,
            dropped: Vec::new(),
            n,
            cells,
            done: 0,
            crowd_ok,
            jumps: false,
            crowd: false,
        };
        e.lay_food(s.food);
        e
    }

    /// Writes the scent the piles lay at full strength, and which pile each
    /// cell's food is: the dish's own if `food`, plus any dropped with F.
    /// The scent pass lays `fed_scent` of it once a pile is reached (the
    /// CPU's `lay_scent`); dish piles are marked even with `food` off, for
    /// the measures. Piles sit 28 dish cells apart, so their discs never
    /// overlap; dropped food that overlaps a pile takes its cells.
    fn lay_food(&mut self, food: bool) {
        let d = &self.dish;
        let mut lay = vec![0.0f32; d.w * d.h];
        let mut pile = vec![0u32; d.w * d.h];
        let s = d.p.scale as f32;
        let r = d.p.pile_r * s;
        let n = r.ceil() as i32;
        let dish = d.piles.iter().map(|&(x, y)| (x * s, y * s, food));
        let dropped = self.dropped.iter().map(|&(x, y)| (x, y, true));
        for (pi, (cx, cy, lays)) in dish.chain(dropped).enumerate().take(MAX_PILES) {
            for dy in -n..=n {
                for dx in -n..=n {
                    let (x, y) = (cx + dx as f32, cy + dy as f32);
                    if ((dx * dx + dy * dy) as f32) > r * r || x < 0.0 || y < 0.0 || x >= d.w as f32 || y >= d.h as f32 {
                        continue;
                    }
                    let i = y as usize * d.w + x as usize;
                    if d.agar[i] {
                        if lays {
                            lay[i] += d.p.food_deposit;
                        }
                        pile[i] = pi as u32 + 1;
                    }
                }
            }
        }
        self.queue.write_buffer(&self.lay, 0, &f32_bytes(&lay));
        self.queue.write_buffer(&self.pile, 0, &u32_bytes(&pile));
    }

    /// Whether F can drop another pile.
    fn can_drop(&self) -> bool {
        self.dish.piles.len() + self.dropped.len() < MAX_PILES
    }

    /// Loads a CPU dish's state (walkers, trail, scent, the scent its piles
    /// lay, and its step for the keyed draws) in place of the GPU's, for the
    /// twin test: one GPU tick from it should match one `BigDish::tick`.
    /// The dish must have this engine's settings.
    #[cfg(test)]
    fn inject(&mut self, d: &BigDish) {
        assert_eq!((d.w, d.h, d.walkers.len() as u32, d.sw, d.sh), (self.dish.w, self.dish.h, self.n, d.w, d.h), "a dish of this engine's size, on the fine scent grid");
        let flat: Vec<f32> = d.walkers.iter().flat_map(|&(x, y, h)| [x, y, h, 0.0]).collect();
        self.queue.write_buffer(&self.walkers, 0, &f32_bytes(&flat));
        let mut occ = vec![0u32; self.cells as usize];
        for &(x, y, _) in &d.walkers {
            occ[y as usize * d.w + x as usize] += 1;
        }
        self.queue.write_buffer(&self.occ, 0, &u32_bytes(&occ));
        // The tick's bind group reads side `done % 2`.
        let side = (d.steps % 2) as usize;
        self.queue.write_buffer(&self.trail[side], 0, &f32_bytes(&d.trail));
        self.queue.write_buffer(&self.scent[side], 0, &f32_bytes(&d.scent));
        self.queue.write_buffer(&self.state, 0, &u32_bytes(&[d.steps as u32, 1f32.to_bits(), 0, 0]));
        let mut reached = vec![0u32; MAX_PILES];
        for (r, &hit) in reached.iter_mut().zip(&d.reached) {
            *r = hit as u32;
        }
        self.queue.write_buffer(&self.reached, 0, &u32_bytes(&reached));
        let counters: Vec<u32> = if d.sick.is_empty() { vec![0; self.n as usize] } else { d.sick.iter().map(|&c| c as u32).collect() };
        self.queue.write_buffer(&self.sick, 0, &u32_bytes(&counters));
        let germ = if d.germ.is_empty() { vec![0.0; self.cells as usize] } else { d.germ.clone() };
        self.queue.write_buffer(&self.germ[side], 0, &f32_bytes(&germ));
        self.done = d.steps;
        self.dish = d.clone();
        self.dropped.clear();
        self.lay_food(true);
    }

    /// Writes the controls for the next frame of `ticks` ticks.
    fn write_params(&mut self, s: &Settings, hand: Hand, ticks: u32) {
        let seed = s.seed;
        let (w, h) = (self.dish.w as u32, self.dish.h as u32);
        let crowd = (s.crowd && self.crowd_ok) as u32;
        let deposit = s.laid * 100_000.0 / self.n.max(1) as f32;
        let (bx, by) = hand.at.unwrap_or((-1.0e6, -1.0e6));
        let br = 6.0 * self.dish.p.scale as f32;
        let pull = if hand.pull && hand.at.is_some() { PULL } else { 0.0 };
        let wipe = (hand.wipe && hand.at.is_some()) as u32;
        let t = &self.dish.p;
        let mut b = u32_bytes(&[w, h, self.n, crowd, stream(seed, DRAW_TURN), stream(seed, DRAW_BLOCKED), stream(seed, CLAIM_MOVE), stream(seed, DRAW_JUMP)]);
        // Food smelled at the reach when `smell` is 0, as `BigParams::scent_offset`.
        let smell = if s.smell > 0.0 { s.smell } else { s.reach };
        b.extend(f32_bytes(&[s.sensor_angle.to_radians(), s.turn.to_radians(), s.reach, smell, s.step, deposit, s.keep, t.scent_keep]));
        b.extend(f32_bytes(&[s.respawn, pull, bx, by, br, s.knee, s.gamma]));
        b.extend(u32_bytes(&[stream(seed, DRAW_PULL), wipe, (s.colour == Colour::Grey) as u32, (s.field == Field::Walkers) as u32, (s.curve == Curve::Soft) as u32]));
        b.extend(f32_bytes(&[t.tube, t.fed_scent]));
        b.extend(u32_bytes(&[s.cap as u32, stream(seed, DRAW_WANDER)]));
        // The looks: angles in radians, as `BigParams`.
        let (am, tm) = ([s.angle_by[0].to_radians(), s.angle_by[1]], [s.turn_by[0].to_radians(), s.turn_by[1]]);
        b.extend(f32_bytes(&[s.trail_scale, s.reach_by[0], s.reach_by[1], am[0], am[1], tm[0], tm[1], s.step_by[0], s.step_by[1], s.floor, s.wander, s.catch_at]));
        // Infect: the mouse's disc, or the middle once `seed_at` has come.
        let (mut infect, mut ix, mut iy, mut ir) = (0u32, 0.0f32, 0.0f32, 0.0f32);
        if let (true, Some((x, y))) = (hand.infect, hand.at) {
            (infect, ix, iy, ir) = (1, x, y, br);
        } else if s.sick_on() && s.seed_at > 0.0 && !self.seeded && ticks > 0 && self.done as f32 >= s.seed_at * STEPS_PER_SECOND as f32 {
            (infect, ix, iy, ir) = (1, w as f32 / 2.0, h as f32 / 2.0, s.seed_r);
            self.seeded = true;
        }
        b.extend(u32_bytes(&[s.sick, s.immune, infect, 0]));
        b.extend(f32_bytes(&[s.germ, s.germ_keep, s.sick_speed, s.sick_trail, s.sick_reach, s.sick_turn, ix, iy, ir, 0.0, 0.0, 0.0]));
        debug_assert_eq!(b.len(), PARAMS_BYTES);
        self.queue.write_buffer(&self.params, 0, &b);
        self.jumps = s.respawn > 0.0 || pull > 0.0;
        self.crowd = crowd > 0;
    }

    /// Records a frame: `ticks` ticks, then the drawing. With `stamps`
    /// (and at least one tick), GPU timestamps at the first tick's start
    /// (0), the last tick's end (1) and the drawing's end (2).
    fn record(&mut self, shared: &Shared, enc: &mut wgpu::CommandEncoder, ticks: u32, soft: bool, stamps: Option<&wgpu::QuerySet>) {
        let stamps = if ticks > 0 { stamps } else { None };
        let writes = |begin: Option<u32>, end: Option<u32>| {
            stamps.and_then(|qs| (begin.is_some() || end.is_some()).then_some(wgpu::ComputePassTimestampWrites { query_set: qs, beginning_of_pass_write_index: begin, end_of_pass_write_index: end }))
        };
        let tick = self.tick_passes();
        for k in 0..ticks {
            let group = &self.groups[(self.done % 2) as usize];
            for (q, &(p, size)) in tick.iter().enumerate() {
                let begin = (k == 0 && q == 0).then_some(0);
                let end = (k + 1 == ticks && q + 1 == tick.len()).then_some(1);
                let mut pass = enc.begin_compute_pass(&wgpu::ComputePassDescriptor { label: None, timestamp_writes: writes(begin, end) });
                pass.set_pipeline(&shared.pipes[p]);
                pass.set_bind_group(0, group, &[]);
                pass.dispatch_workgroups(size, 1, 1);
            }
            self.done += 1;
        }
        let group = &self.groups[(self.done % 2) as usize];
        for &(p, groups) in &self.draw_passes(soft) {
            let end = (p == DRAW).then_some(2);
            let mut pass = enc.begin_compute_pass(&wgpu::ComputePassDescriptor { label: None, timestamp_writes: writes(None, end) });
            pass.set_pipeline(&shared.pipes[p]);
            pass.set_bind_group(0, group, &[]);
            pass.dispatch_workgroups(groups, 1, 1);
        }
    }

    /// A tick's passes and their workgroups: the jump passes only while
    /// respawn or the pull is on, `land` only with the crowd limit.
    fn tick_passes(&self) -> Vec<(usize, u32)> {
        let (n, cells) = (self.n.div_ceil(256), self.cells.div_ceil(256));
        let mut tick = vec![(SENSE, n), (WALK, n)];
        if self.jumps {
            tick.push((JUMP, n));
            if self.crowd {
                tick.push((LAND, n));
            }
        }
        tick.extend([(TRAIL, cells), (SCENT, cells)]);
        tick
    }

    /// The drawing's passes: the soft curve's histogram and reference first.
    fn draw_passes(&self, soft: bool) -> Vec<(usize, u32)> {
        let cells = self.cells.div_ceil(256);
        let mut draw = Vec::new();
        if soft {
            draw.extend([(HISTOGRAM, cells), (TOP, 1)]);
        }
        draw.push((DRAW, cells));
        draw
    }

    /// One tick and the drawing with every pass timed on its own (each
    /// pass's start and end stamped), submitted and waited for: GPU ms a
    /// pass, by name. It runs the tick, so the dish moves on by one.
    fn profile(&mut self, shared: &Shared, soft: bool) -> Vec<(&'static str, f64)> {
        const NAMES: [&str; 9] = ["sense", "walk", "trail", "scent", "histogram", "top", "draw", "jump", "land"];
        let tick = self.tick_passes();
        let passes: Vec<(usize, u32, usize)> = tick.iter().map(|&(p, g)| (p, g, (self.done % 2) as usize)).chain(self.draw_passes(soft).into_iter().map(|(p, g)| (p, g, ((self.done + 1) % 2) as usize))).collect();
        let count = 2 * passes.len() as u32;
        let qs = self.device.create_query_set(&wgpu::QuerySetDescriptor { label: Some("profile"), ty: wgpu::QueryType::Timestamp, count });
        let resolve = self.device.create_buffer(&wgpu::BufferDescriptor { label: Some("profile"), size: count as u64 * 8, usage: wgpu::BufferUsages::QUERY_RESOLVE | wgpu::BufferUsages::COPY_SRC, mapped_at_creation: false });
        let mut enc = self.device.create_command_encoder(&wgpu::CommandEncoderDescriptor { label: None });
        for (q, &(p, groups, side)) in passes.iter().enumerate() {
            let writes = wgpu::ComputePassTimestampWrites { query_set: &qs, beginning_of_pass_write_index: Some(2 * q as u32), end_of_pass_write_index: Some(2 * q as u32 + 1) };
            let mut pass = enc.begin_compute_pass(&wgpu::ComputePassDescriptor { label: None, timestamp_writes: Some(writes) });
            pass.set_pipeline(&shared.pipes[p]);
            pass.set_bind_group(0, &self.groups[side], &[]);
            pass.dispatch_workgroups(groups, 1, 1);
        }
        self.done += 1;
        enc.resolve_query_set(&qs, 0..count, &resolve, 0);
        self.queue.submit([enc.finish()]);
        let t: Vec<u64> = read_back(&self.device, &self.queue, &resolve).chunks_exact(8).map(|c| u64::from_le_bytes(c.try_into().unwrap())).collect();
        let period = self.queue.get_timestamp_period() as f64;
        passes.iter().enumerate().map(|(q, &(p, ..))| (NAMES[p], t[2 * q + 1].saturating_sub(t[2 * q]) as f64 * period / 1e6)).collect()
    }

    /// Reads the dish back into the CPU dish and measures it.
    fn measure(&mut self) -> slime_mould::mould::big::BigMeasure {
        let side = (self.done % 2) as usize;
        self.dish.trail = floats(&read_back(&self.device, &self.queue, &self.trail[side]));
        self.dish.walkers = floats(&read_back(&self.device, &self.queue, &self.walkers)).chunks_exact(4).map(|c| (c[0], c[1], c[2])).collect();
        self.dish.ticks = self.done;
        // The GPU marks a pile from the trail a tick late (see `trail`), so
        // the CPU marks any the last tick's trail reached, as it would.
        let reached = read_back(&self.device, &self.queue, &self.reached);
        for (r, c) in self.dish.reached.iter_mut().zip(reached.chunks_exact(4)) {
            *r = u32::from_le_bytes(c.try_into().unwrap()) != 0;
        }
        self.dish.update_reached();
        self.dish.measure()
    }

    /// The share of walkers sick now (counter over `immune`).
    fn sick_share(&self, s: &Settings) -> f64 {
        let c = read_back(&self.device, &self.queue, &self.sick);
        let sick = c.chunks_exact(4).take(self.n as usize).filter(|b| u32::from_le_bytes((*b).try_into().unwrap()) > s.immune).count();
        sick as f64 / self.n.max(1) as f64
    }

    fn picture(&self) -> image::RgbaImage {
        let bytes = read_back(&self.device, &self.queue, &self.frame);
        image::RgbaImage::from_raw(self.dish.w as u32, self.dish.h as u32, bytes).expect("frame size")
    }
}

fn read_back(device: &wgpu::Device, queue: &wgpu::Queue, src: &wgpu::Buffer) -> Vec<u8> {
    let size = src.size();
    let staging = device.create_buffer(&wgpu::BufferDescriptor { label: Some("read back"), size, usage: wgpu::BufferUsages::MAP_READ | wgpu::BufferUsages::COPY_DST, mapped_at_creation: false });
    let mut enc = device.create_command_encoder(&wgpu::CommandEncoderDescriptor { label: None });
    enc.copy_buffer_to_buffer(src, 0, &staging, 0, size);
    queue.submit([enc.finish()]);
    staging.map_async(wgpu::MapMode::Read, .., |r| r.expect("map"));
    device.poll(wgpu::PollType::wait_indefinitely()).expect("poll");
    let bytes = staging.get_mapped_range(..).to_vec();
    staging.unmap();
    bytes
}

/// GPU timestamps read back without stalling: resolved into `read` on a
/// frame when it is free, mapped, and picked up on a later frame.
struct Stamps {
    qs: wgpu::QuerySet,
    resolve: wgpu::Buffer,
    read: wgpu::Buffer,
    ready: Arc<AtomicBool>,
    busy: bool,
    /// Nanoseconds a tick of the GPU's timestamp counter.
    period: f64,
    /// Smoothed readings, ms: the ticks and the drawing.
    sim_ms: f64,
    draw_ms: f64,
    /// Every reading, for the screenshot hook's median.
    all: Vec<(f64, f64)>,
}

impl Stamps {
    fn new(device: &wgpu::Device, queue: &wgpu::Queue) -> Self {
        let qs = device.create_query_set(&wgpu::QuerySetDescriptor { label: Some("frame"), ty: wgpu::QueryType::Timestamp, count: 3 });
        let resolve = device.create_buffer(&wgpu::BufferDescriptor { label: Some("resolve"), size: 24, usage: wgpu::BufferUsages::QUERY_RESOLVE | wgpu::BufferUsages::COPY_SRC, mapped_at_creation: false });
        let read = device.create_buffer(&wgpu::BufferDescriptor { label: Some("stamps"), size: 24, usage: wgpu::BufferUsages::MAP_READ | wgpu::BufferUsages::COPY_DST, mapped_at_creation: false });
        Stamps { qs, resolve, read, ready: Arc::new(AtomicBool::new(false)), busy: false, period: queue.get_timestamp_period() as f64, sim_ms: 0.0, draw_ms: 0.0, all: Vec::new() }
    }

    /// The query set to write this frame, if the last reading is in.
    fn take(&self) -> Option<&wgpu::QuerySet> {
        (!self.busy).then_some(&self.qs)
    }

    fn resolve(&self, enc: &mut wgpu::CommandEncoder) {
        enc.resolve_query_set(&self.qs, 0..3, &self.resolve, 0);
        enc.copy_buffer_to_buffer(&self.resolve, 0, &self.read, 0, 24);
    }

    /// After the submit: ask for the reading.
    fn map(&mut self) {
        let ready = Arc::clone(&self.ready);
        self.read.map_async(wgpu::MapMode::Read, .., move |r| {
            if r.is_ok() {
                ready.store(true, Ordering::Release);
            }
        });
        self.busy = true;
    }

    /// Picks up a reading if one has arrived.
    fn poll(&mut self) {
        if !self.busy || !self.ready.load(Ordering::Acquire) {
            return;
        }
        let t: Vec<u64> = self.read.get_mapped_range(..).chunks_exact(8).map(|c| u64::from_le_bytes(c.try_into().unwrap())).collect();
        self.read.unmap();
        self.ready.store(false, Ordering::Release);
        self.busy = false;
        let ms = |a: u64, b: u64| b.saturating_sub(a) as f64 * self.period / 1e6;
        let (sim, draw) = (ms(t[0], t[1]), ms(t[1], t[2]));
        // Smoothed, so the panel's number can be read; seeded with the
        // first reading (it started at 0 until the review of 2026-11-11, so
        // early screenshot figures read low).
        let a = if self.all.is_empty() { 1.0 } else { 0.1 };
        self.sim_ms += a * (sim - self.sim_ms);
        self.draw_ms += a * (draw - self.draw_ms);
        if self.all.len() < 100_000 {
            self.all.push((sim, draw));
        }
    }

    /// The median reading so far: the ticks and the drawing, ms, and how
    /// many readings.
    fn median(&self) -> (f64, f64, usize) {
        let mid = |mut v: Vec<f64>| {
            v.sort_by(|a, b| a.partial_cmp(b).unwrap());
            v.get(v.len() / 2).copied().unwrap_or(0.0)
        };
        (mid(self.all.iter().map(|r| r.0).collect()), mid(self.all.iter().map(|r| r.1).collect()), self.all.len())
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
    hand: Hand,
    frames: u64,
    /// Frames drawn since the window opened.
    drawn: u64,
    /// `MOULDGPU_SCREENSHOT_AFTER_FRAMES`: save the frame, panel and all, then quit.
    shot_after: Option<u64>,
    fps_clock: Instant,
    fps: f64,
    result: Result<(), String>,
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
            hand: Hand::default(),
            frames: 0,
            drawn: 0,
            shot_after: std::env::var("MOULDGPU_SCREENSHOT_AFTER_FRAMES").ok().and_then(|v| v.parse().ok()),
            fps_clock: Instant::now(),
            fps: 0.0,
            result: Ok(()),
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
        self.engine = Some(e);
        self.panel_dirty = true;
        if let Some(w) = &self.window {
            w.set_title(&format!("Mould sandbox (GPU): {}", PRESETS[self.preset].to_lowercase()));
        }
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
            let ms = self.stamps.as_ref().map_or("GPU TIME N/A".to_string(), |st| format!("GPU {:.2} MS A FRAME (TICKS {:.2} + DRAWING {:.2})", st.sim_ms + st.draw_ms, st.sim_ms, st.draw_ms));
            (d, ms)
        });
        let sick = if self.s.sick_on() { "  MIDDLE DRAG OR I INFECTS" } else { "" };
        let bottom = vec![
            ("1 BIG DISH  2 32K  3 NO LIMIT  4 JENSON  5 +RESPAWN  6 BY WALKERS  7 4M  8 4M OURS".to_string(), WHITE),
            ("9 TRUNK RING  0 CONTAGION  SHIFT+1 SWEEP  2 CURVES  3 POLYGONS  4 ENDEMIC  5 SICK ROPES".to_string(), WHITE),
            ("ARROWS PICK AND CHANGE (SHIFT: MORE)  SPACE PAUSE  . STEP  R RESTART  N NEW SEED".to_string(), DIM),
            (format!("LEFT DRAG PULLS  RIGHT DRAG WIPES  F DROPS FOOD{sick}  ESC QUITS"), DIM),
            (format!("{ms}  {:.0} FPS  {:.1} S  SEED {}{}", self.fps, done as f64 / STEPS_PER_SECOND as f64, self.s.seed, if self.paused { "  PAUSED" } else { "" }), WHITE),
        ];
        (lines, bottom)
    }

    fn key(&mut self, code: KeyCode, event_loop: &ActiveEventLoop) {
        const DIGITS: [KeyCode; 10] = [KeyCode::Digit1, KeyCode::Digit2, KeyCode::Digit3, KeyCode::Digit4, KeyCode::Digit5, KeyCode::Digit6, KeyCode::Digit7, KeyCode::Digit8, KeyCode::Digit9, KeyCode::Digit0];
        // 1-9 and 0 are presets 1-10; Shift with 1-5 the five after.
        let digit = DIGITS.iter().position(|&k| k == code).map(|k| if self.shift { k + 10 } else { k }).filter(|&k| k < PRESETS.len());
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
            KeyCode::Space => self.paused = !self.paused,
            KeyCode::Period => self.step_once = true,
            KeyCode::KeyR => self.restart(),
            KeyCode::KeyN => {
                self.s.seed += 1;
                self.restart();
            }
            KeyCode::KeyH => self.show_panel = !self.show_panel,
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
        }
        if self.panel_dirty {
            let (lines, bottom) = self.panel_lines();
            if let Some(e) = &self.engine {
                let img = panel(e.dish.w, e.dish.h, &lines, &bottom);
                e.queue.write_buffer(&e.hud, 0, &img);
            }
            self.panel_dirty = false;
        }
        let ticks = if self.paused {
            std::mem::take(&mut self.step_once) as u32
        } else {
            self.s.ticks
        };
        let (Some(p), Some(shared), Some(e)) = (&self.pixels, &self.shared, self.engine.as_mut()) else { return };
        let mut hand = self.hand;
        hand.infect |= std::mem::take(&mut self.infect_once);
        e.write_params(&self.s, hand, ticks);
        let soft = self.s.curve == Curve::Soft;
        let stamps = self.stamps.as_ref().and_then(|st| st.take().map(|_| st));
        // pixels skips the closure when the window is hidden, so the
        // stamps count as written only if it ran.
        let mut wrote = false;
        let res = p.render_with(|enc, target, ctx| {
            e.record(shared, enc, ticks, soft, stamps.map(|st| &st.qs));
            if let Some(st) = stamps.filter(|_| ticks > 0) {
                st.resolve(enc);
                wrote = true;
            }
            enc.copy_buffer_to_texture(
                wgpu::TexelCopyBufferInfo { buffer: &e.frame, layout: wgpu::TexelCopyBufferLayout { offset: 0, bytes_per_row: Some(e.dish.w as u32 * 4), rows_per_image: Some(e.dish.h as u32) } },
                wgpu::TexelCopyTextureInfo { texture: &ctx.texture, mip_level: 0, origin: wgpu::Origin3d::ZERO, aspect: wgpu::TextureAspect::All },
                ctx.texture_extent,
            );
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
            e.picture().save(&path).unwrap();
            println!("mouldgpu: wrote {}", path.display());
            let st = self.stamps.as_ref().map_or("no timestamps".to_string(), |st| {
                let (sim, draw, n) = st.median();
                format!("GPU {:.3} ms a frame (ticks {sim:.3} + drawing {draw:.3}), median of {n} frames timed", sim + draw)
            });
            println!("mouldgpu: after {} frames, {} ticks: {st}; {:.1} frames a second", self.drawn, e.done, self.fps);
            event_loop.exit();
        }
    }
}

fn scaling_for(window: PhysicalSize<u32>, dish: (usize, usize)) -> ScalingMode {
    if window.width as usize >= dish.0 && window.height as usize >= dish.1 {
        ScalingMode::PixelPerfect
    } else {
        ScalingMode::Fill
    }
}

impl ApplicationHandler for App {
    fn resumed(&mut self, event_loop: &ActiveEventLoop) {
        if self.window.is_some() {
            return;
        }
        let p = big_params(&self.s, self.scale, false);
        let (w, h) = (p.w() as u32, p.h() as u32);
        let attributes = Window::default_attributes().with_title("Mould sandbox (GPU)").with_inner_size(PhysicalSize::new(w, h));
        let window = match event_loop.create_window(attributes) {
            Ok(w) => Arc::new(w),
            Err(err) => return self.fail(event_loop, format!("window creation failed: {err}")),
        };
        // Look at the GPU first, to ask for timestamps if it has them.
        let instance = wgpu::Instance::new(wgpu::InstanceDescriptor::new_without_display_handle_from_env());
        let probe = block_on(instance.request_adapter(&wgpu::RequestAdapterOptions { power_preference: wgpu::PowerPreference::HighPerformance, ..Default::default() })).ok();
        let size = window.inner_size();
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
        pixels.set_scaling_mode(scaling_for(size, (w as usize, h as usize)));
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
            self.stamps = Some(Stamps::new(pixels.device(), pixels.queue()));
        }
        self.pixels = Some(pixels);
        self.window = Some(window);
        self.restart();
    }

    fn window_event(&mut self, event_loop: &ActiveEventLoop, _id: WindowId, event: WindowEvent) {
        match event {
            WindowEvent::CloseRequested => event_loop.exit(),
            WindowEvent::Resized(size) => {
                if size.width > 0 && size.height > 0 {
                    if let Some(p) = self.pixels.as_mut() {
                        let dish = self.engine.as_ref().map_or((1, 1), |e| (e.dish.w, e.dish.h));
                        p.set_scaling_mode(scaling_for(size, dish));
                        if let Err(err) = p.resize_surface(size.width, size.height) {
                            self.fail(event_loop, format!("surface resize failed: {err}"));
                        }
                    }
                }
            }
            WindowEvent::ModifiersChanged(m) => self.shift = m.state().shift_key(),
            WindowEvent::KeyboardInput { event, .. } => {
                if let PhysicalKey::Code(code) = event.physical_key {
                    let repeat_ok = matches!(code, KeyCode::ArrowLeft | KeyCode::ArrowRight | KeyCode::ArrowUp | KeyCode::ArrowDown | KeyCode::Period);
                    if event.state == ElementState::Pressed && (!event.repeat || repeat_ok) {
                        self.key(code, event_loop);
                    }
                }
            }
            WindowEvent::CursorMoved { position, .. } => {
                self.hand.at = self.pixels.as_ref().and_then(|p| p.window_pos_to_pixel((position.x as f32, position.y as f32)).ok()).map(|(x, y)| (x as f32 + 0.5, y as f32 + 0.5));
            }
            WindowEvent::CursorLeft { .. } => self.hand.at = None,
            WindowEvent::Focused(false) => {
                self.hand.pull = false;
                self.hand.wipe = false;
                self.hand.infect = false;
            }
            WindowEvent::MouseInput { state, button, .. } => {
                let down = state == ElementState::Pressed;
                match button {
                    MouseButton::Left => self.hand.pull = down,
                    MouseButton::Right => self.hand.wipe = down,
                    MouseButton::Middle => self.hand.infect = down,
                    _ => {}
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

fn block_on<F: std::future::Future>(f: F) -> F::Output {
    use std::task::{Context, Poll, Wake, Waker};
    struct Unpark(std::thread::Thread);
    impl Wake for Unpark {
        fn wake(self: Arc<Self>) {
            self.0.unpark();
        }
    }
    let waker = Waker::from(Arc::new(Unpark(std::thread::current())));
    let mut cx = Context::from_waker(&waker);
    let mut f = std::pin::pin!(f);
    loop {
        if let Poll::Ready(v) = f.as_mut().poll(&mut cx) {
            return v;
        }
        std::thread::park();
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
            if s.sick_on() {
                println!(
                    "   contagion: sick {} immune {} ticks, catch {}, germ {} kept {}, sick speed {} trail {} reach {} turn {}, seeded at {} s r {}",
                    s.sick, s.immune, s.catch_at, s.germ, s.germ_keep, s.sick_speed, s.sick_trail, s.sick_reach, s.sick_turn, s.seed_at, s.seed_r
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
        for f in 0..frames {
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
            let mm = e.measure();
            let sick = if s.sick_on() { format!("  sick {:.3}", e.sick_share(s)) } else { String::new() };
            println!("dish at {:.1}s: tube {:.1}  loops {}  width {:.2}  reach {:.1}  piles {}/{}  strays {:.3}{sick}", mm.seconds, mm.tube_area, mm.loops, mm.width, mm.reach, mm.reached, e.dish.piles.len(), mm.strays);
            if !course.is_empty() {
                println!("sick every 0.5 s: {}", course.join(" "));
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

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let mut s = Settings::big_dish();
    let mut picked = 0usize;
    let mut scale = 4usize;
    let mut look = Look { dir: String::new(), seconds: 10.0, gif: false, frames: false, every: 6, down: 2, twice: false, tag: String::new() };
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
        // Every preset has a key, and Shift+1-5 are the last five.
        assert_eq!((preset_key(8), preset_key(9), preset_key(14)), ("9".to_string(), "0".to_string(), "SHIFT+5".to_string()));
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
        for s in [small(4), looks] {
            let hand = |f: u32| Hand { at: Some((200.0 + 4.0 * f as f32, 150.0)), pull: f < 10, wipe: f >= 10, infect: f == 3 };
            let mut seen = Vec::new();
            for _ in 0..2 {
                let mut e = Engine::new(&device, &queue, &shared, &s, SCALE);
                for f in 0..20 {
                    run(&mut e, &shared, &s, s.ticks, hand(f));
                }
                let sick: Vec<u8> = read_back(&device, &queue, &e.sick);
                seen.push((e.picture(), walkers_of(&e), sick));
            }
            let bits = |w: &[(f32, f32, f32)]| w.iter().flat_map(|&(x, y, h)| [x.to_bits(), y.to_bits(), h.to_bits()]).collect::<Vec<u32>>();
            assert!(seen[0].0 == seen[1].0, "the pictures differ");
            assert_eq!(bits(&seen[0].1), bits(&seen[1].1), "the walkers differ");
            assert!(seen[0].2 == seen[1].2, "the sick differ");
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
}
