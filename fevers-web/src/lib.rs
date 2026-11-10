//! Two Fevers in a browser: `Fevers::paint`'s frame drawn into a buffer the
//! page copies to a canvas, through a plain C interface (`fv_*`) so the page
//! needs no generated glue. The opponent hand (`fever::Hand`) runs here, the
//! same code the census plays.

use std::cell::RefCell;

use slime_mould::mould::fever::{Fevers, Hand, Phase, Player, Rules, H, STEPS_PER_SECOND, TIDE, W};

/// Steps per sixtieth of a second: the dish's real speed.
const STEPS_PER_TICK: u32 = STEPS_PER_SECOND / 60;
const TICK_MS: f64 = 1000.0 / 60.0;
/// The most ticks one call catches up, so a stalled tab does not freeze on
/// return.
const MAX_TICKS: u32 = 4;

struct Game {
    g: Fevers,
    /// Tide's hand, `None` when a second person plays it.
    rival: Option<Player>,
    frame: Vec<u8>,
    acc: f64,
}

thread_local! {
    static GAME: RefCell<Option<Game>> = const { RefCell::new(None) };
}

fn with<R>(f: impl FnOnce(&mut Game) -> R, none: R) -> R {
    GAME.with(|g| g.borrow_mut().as_mut().map(f).unwrap_or(none))
}

/// A new match. `opponent`: 0 idle, 1 random, 2 plan, 3 a second person,
/// 4 plan that hushes.
#[no_mangle]
pub extern "C" fn fv_new(seed: u32, opponent: u32) {
    let rival = match opponent {
        0 => Some(Hand::Idle),
        1 => Some(Hand::Random),
        2 => Some(Hand::Plan),
        4 => Some(Hand::PlanHush),
        _ => None,
    }
    .map(|h| Player::new(h, TIDE, seed as u64));
    let g = Fevers::new(seed as u64, Rules::default());
    GAME.with(|cell| {
        *cell.borrow_mut() = Some(Game {
            g,
            rival,
            frame: vec![0; W * H * 4],
            acc: 0.0,
        });
    });
}

/// Advance by `ms` of wall time (times `speed`), then paint. Returns the
/// steps taken.
#[no_mangle]
pub extern "C" fn fv_tick(ms: f64, speed: u32) -> u32 {
    with(
        |gm| {
            gm.acc = (gm.acc + ms).min(TICK_MS * MAX_TICKS as f64);
            let mut steps = 0;
            while gm.acc >= TICK_MS {
                gm.acc -= TICK_MS;
                for _ in 0..STEPS_PER_TICK * speed.max(1) {
                    if gm.g.phase() == Phase::Over {
                        break;
                    }
                    if let Some(p) = gm.rival.as_mut() {
                        p.act(&mut gm.g);
                    }
                    gm.g.step();
                    steps += 1;
                }
            }
            gm.g.paint(&mut gm.frame);
            steps
        },
        0,
    )
}

#[no_mangle]
pub extern "C" fn fv_frame() -> *const u8 {
    with(|gm| gm.frame.as_ptr(), std::ptr::null())
}

#[no_mangle]
pub extern "C" fn fv_width() -> u32 {
    W as u32
}

#[no_mangle]
pub extern "C" fn fv_height() -> u32 {
    H as u32
}

/// Seed `strain` (0 Ember, 1 Tide) at dish cell (x, y); walkers caught.
#[no_mangle]
pub extern "C" fn fv_seed(strain: u32, x: f32, y: f32) -> u32 {
    with(|gm| gm.g.seed(strain as usize & 1, x, y), 0)
}

/// Walkers a seed of `strain` at (x, y) would catch now.
#[no_mangle]
pub extern "C" fn fv_would(strain: u32, x: f32, y: f32) -> u32 {
    with(|gm| gm.g.would_catch(strain as usize & 1, x, y), 0)
}

#[no_mangle]
pub extern "C" fn fv_seed_r() -> f32 {
    with(|gm| gm.g.rules.seed_r, 0.0)
}

#[no_mangle]
pub extern "C" fn fv_spores(strain: u32) -> u32 {
    with(|gm| gm.g.spores[strain as usize & 1], 0)
}

#[no_mangle]
pub extern "C" fn fv_spores_max() -> u32 {
    with(|gm| gm.g.rules.spores_max, 0)
}

/// How far the next spore has come, 0..1.
#[no_mangle]
pub extern "C" fn fv_spore_progress(strain: u32) -> f32 {
    with(|gm| gm.g.spore_progress(strain as usize & 1), 0.0)
}

/// The strain's share of the host now: owned (`Rules::owned`, the
/// default) or carrying.
#[no_mangle]
pub extern "C" fn fv_share(strain: u32) -> f32 {
    with(|gm| gm.g.share_now()[strain as usize & 1], 0.0)
}

/// The strain's score so far: its share of the host averaged over play.
#[no_mangle]
pub extern "C" fn fv_score(strain: u32) -> f64 {
    with(|gm| gm.g.score()[strain as usize & 1], 0.0)
}

/// Walkers immune to the strain now, as a share of the host (unused by the
/// page since owned ground; kept for a probe).
#[no_mangle]
pub extern "C" fn fv_immune(strain: u32) -> f32 {
    with(
        |gm| {
            let s = strain as usize & 1;
            gm.g.host.iter().filter(|h| h.immune[s] > 0).count() as f32 / gm.g.host.len().max(1) as f32
        },
        0.0,
    )
}

/// 0 warm-up, 1 play, 2 over.
#[no_mangle]
pub extern "C" fn fv_phase() -> u32 {
    with(
        |gm| match gm.g.phase() {
            Phase::Warm => 0,
            Phase::Play => 1,
            Phase::Over => 2,
        },
        0,
    )
}

#[no_mangle]
pub extern "C" fn fv_play_seconds() -> f32 {
    with(|gm| gm.g.play_seconds(), 0.0)
}

#[no_mangle]
pub extern "C" fn fv_match_seconds() -> f32 {
    with(|gm| gm.g.rules.match_seconds, 0.0)
}

/// Seconds of warm-up left.
#[no_mangle]
pub extern "C" fn fv_warm_left() -> f32 {
    with(
        |gm| gm.g.warm_steps().saturating_sub(gm.g.t) as f32 / STEPS_PER_SECOND as f32,
        0.0,
    )
}

/// Seeds planted by the strain.
#[no_mangle]
pub extern "C" fn fv_seeds(strain: u32) -> u32 {
    with(|gm| gm.g.seeds[strain as usize & 1], 0)
}

/// Walkers carrying the strain now, as a share of the host.
#[no_mangle]
pub extern "C" fn fv_carrying(strain: u32) -> f32 {
    with(
        |gm| {
            let c = gm.g.carriers()[strain as usize & 1];
            c as f32 / gm.g.host.len().max(1) as f32
        },
        0.0,
    )
}

/// Hush the strain's fever (`on` 1) or let it lay trail again (0); 1 if it
/// switched, 0 while its cooldown runs.
#[no_mangle]
pub extern "C" fn fv_set_hush(strain: u32, on: u32) -> u32 {
    with(|gm| gm.g.set_hush(strain as usize & 1, on != 0) as u32, 0)
}

#[no_mangle]
pub extern "C" fn fv_hush(strain: u32) -> u32 {
    with(|gm| gm.g.hush[strain as usize & 1] as u32, 0)
}

/// Seconds until the strain may switch hush again.
#[no_mangle]
pub extern "C" fn fv_hush_wait(strain: u32) -> f32 {
    with(|gm| gm.g.hush_wait(strain as usize & 1), 0.0)
}
