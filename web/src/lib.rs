//! The budget skirmish in a browser: the game's own frame (`render::draw`,
//! the game's look) drawn into a buffer the page copies to a canvas. A
//! plain C interface, so the page needs no generated glue: it calls these
//! `sk_*` functions and reads the frame out of the module's memory.
//!
//! You are side 0; the rival (side 1) plays one of the scripted plans
//! (`Plan`), the same code the round robins ran. The page asks which pile a
//! press landed on (`sk_pile_at`) and buys there (`sk_act`); the module
//! keeps no tool of its own. Version 1's page (lamp, salt, keys 1-4) is in
//! git history at 1b23018.

use std::cell::RefCell;

use slime_mould::mould::look::Style;
use slime_mould::mould::render::{self, View};
use slime_mould::mould::skirmish::{Habit, Plan, Skirmish, Verb, TITHES};
use slime_mould::mould::{Phase, STEPS_PER_FRAME, STEPS_PER_SECOND};

/// Frame pixels per dish cell, as the window's.
const SCALE: u32 = 4;
/// One tick is a sixtieth of a second: `STEPS_PER_FRAME` steps.
const TICK_MS: f64 = 1000.0 / 60.0;
const MAX_TICKS: u32 = 4;
const FAST: u32 = 4;
/// A press counts for a pile within this many cells of its centre.
const PICK: f32 = 9.0;

struct Game {
    sk: Skirmish,
    view: View,
    frame: Vec<u8>,
    acc: f64,
}

thread_local! {
    static GAME: RefCell<Option<Game>> = const { RefCell::new(None) };
}

fn with<R>(f: impl FnOnce(&mut Game) -> R, none: R) -> R {
    GAME.with(|g| g.borrow_mut().as_mut().map(f).unwrap_or(none))
}

fn side(s: u32) -> usize {
    (s as usize).min(1)
}

/// The page's verb numbers: 0 beacon, 1 spore, 2 rally (on pile `arg`),
/// 3 habit (`Habit::ALL[arg]`).
fn verb(v: u32, arg: u32) -> Option<Verb> {
    let i = arg as usize;
    match v {
        0 => Some(Verb::Beacon(i)),
        1 => Some(Verb::Spore(i)),
        2 => Some(Verb::Rally(i)),
        3 => Habit::ALL.get(i).map(|&h| Verb::Habit(h)),
        4 => Some(Verb::Fortify(i)),
        _ => None,
    }
}

impl Game {
    fn tick(&mut self, ms: f64, draw: bool) {
        self.acc += ms.min(250.0);
        let mut ticks = 0;
        while self.acc >= TICK_MS && ticks < MAX_TICKS {
            self.acc -= TICK_MS;
            ticks += 1;
            if self.view.paused || self.view.brief {
                continue;
            }
            let n = if self.view.fast {
                STEPS_PER_FRAME * FAST
            } else {
                STEPS_PER_FRAME
            };
            for _ in 0..n {
                self.sk.tick();
                self.sk.dish.step();
            }
        }
        if ticks == MAX_TICKS {
            self.acc = 0.0;
        }
        if draw {
            render::draw(&self.sk.dish, &self.view, SCALE, &mut self.frame);
        }
    }
}

/// The page's rule sets, by `sk_new`'s `rules`: 0 version 2 as first
/// built; 1 round 3's counterplay (spores sleep 5 s where the buyer has no
/// mould, and a spore costs half again for each bought in the last 30 s);
/// 2-4 that plus one of round 4's model changes: a holder's bonus at a
/// pile (`hold` 2), upkeep that grows with size (`upkeep_size` 0.2), or
/// desperate fights (starving walkers fight harder). 5: desperate fights
/// with the economy sketch's Strike, a spore that lands 5 s after it is
/// bought in view of both sides (no sleeping), against which Fortify and
/// the rally answer.
const RULE_SETS: [&[(&str, &str)]; 6] = [
    &[],
    &[("spore_dormant", "5"), ("dormant_abroad_only", "1"), ("spore_ramp", "0.5")],
    &[("spore_dormant", "5"), ("dormant_abroad_only", "1"), ("spore_ramp", "0.5"), ("hold", "2")],
    &[("spore_dormant", "5"), ("dormant_abroad_only", "1"), ("spore_ramp", "0.5"), ("upkeep_size", "0.2")],
    &[
        ("spore_dormant", "5"),
        ("dormant_abroad_only", "1"),
        ("spore_ramp", "0.5"),
        ("fight_fed", "2"),
        ("fight_unfed", "0.5"),
    ],
    &[("spore_ramp", "0.5"), ("fight_fed", "2"), ("fight_unfed", "0.5"), ("strike_delay", "5")],
];

/// A new skirmish on dish `seed` against `Plan::ALL[rival]`, under
/// `RULE_SETS[rules]`. You start on a 10% tithe. Time stands still until
/// `sk_unbrief`.
#[no_mangle]
pub extern "C" fn sk_new(seed: u32, rival: u32, rules: u32) {
    let plan = Plan::ALL.get(rival as usize).copied().unwrap_or(Plan::Soft);
    let mut sk = Skirmish::new(seed as u64, [None, Some(plan)]);
    for (k, v) in RULE_SETS.get(rules as usize).copied().unwrap_or(&[]) {
        sk.set(k, v);
    }
    sk.set_tithe(0, TITHES[1]);
    let (w, h) = render::frame_size(SCALE);
    let mut g = Game {
        sk,
        view: View {
            // Time stands still until the page's own start card is put away;
            // `bare` leaves the frame's card, corners and tool bar (the
            // herding game's stars and lamp) to the page.
            brief: true,
            bare: true,
            keys: false,
            style: Style::GAME,
            ..View::default()
        },
        frame: vec![0; (w * h * 4) as usize],
        acc: 0.0,
    };
    g.tick(0.0, true);
    GAME.with(|cell| *cell.borrow_mut() = Some(g));
}

/// The number of plans the rival can play, and each one's name as bytes
/// in the module's memory (`sk_plan_name_ptr`, `sk_plan_name_len`).
#[no_mangle]
pub extern "C" fn sk_plans() -> u32 {
    Plan::ALL.len() as u32
}

#[no_mangle]
pub extern "C" fn sk_plan_name_ptr(i: u32) -> *const u8 {
    Plan::ALL.get(i as usize).map_or(std::ptr::null(), |p| p.name().as_ptr())
}

#[no_mangle]
pub extern "C" fn sk_plan_name_len(i: u32) -> u32 {
    Plan::ALL.get(i as usize).map_or(0, |p| p.name().len() as u32)
}

#[no_mangle]
pub extern "C" fn sk_frame_w() -> u32 {
    render::frame_size(SCALE).0
}

#[no_mangle]
pub extern "C" fn sk_frame_h() -> u32 {
    render::frame_size(SCALE).1
}

/// The rows of the frame the dish takes; with `bare` the rest is blank.
#[no_mangle]
pub extern "C" fn sk_dish_h() -> u32 {
    slime_mould::mould::H as u32 * SCALE
}

/// Frame pixels per dish cell.
#[no_mangle]
pub extern "C" fn sk_scale() -> u32 {
    SCALE
}

/// The RGBA frame, `sk_frame_w() * sk_frame_h() * 4` bytes; valid until
/// the next `sk_new`.
#[no_mangle]
pub extern "C" fn sk_frame_ptr() -> *const u8 {
    with(|g| g.frame.as_ptr(), std::ptr::null())
}

/// Advance by `ms` of wall time, and draw the frame unless `draw` is 0.
/// Drawing is about two thirds of a frame's cost in the browser (9.3 of
/// 14.3 ms in headless Chromium on a loaded 4-core box, 30 s into a dish),
/// so a slow machine can skip every other draw and keep the dish at speed.
#[no_mangle]
pub extern "C" fn sk_tick(ms: f64, draw: u32) {
    with(|g| g.tick(ms, draw != 0), ())
}

/// The live pile nearest frame pixel (`px`, `py`) within `PICK` cells, or
/// -1.
#[no_mangle]
pub extern "C" fn sk_pile_at(px: f32, py: f32) -> i32 {
    with(
        |g| {
            let (x, y) = (px / SCALE as f32, py / SCALE as f32);
            g.sk
                .dish
                .piles
                .iter()
                .enumerate()
                .filter(|(_, p)| p.food > 0.0 && !p.oats)
                .map(|(i, p)| (i, (p.x - x).hypot(p.y - y)))
                .filter(|&(_, d)| d <= PICK)
                .min_by(|a, b| a.1.total_cmp(&b.1))
                .map_or(-1, |(i, _)| i as i32)
        },
        -1,
    )
}

/// Piles on the dish now (late food adds to them).
#[no_mangle]
pub extern "C" fn sk_piles() -> u32 {
    with(|g| g.sk.dish.piles.len() as u32, 0)
}

/// Pile `i`'s centre in frame pixels, its food left as a fraction of full
/// (0 when eaten or not yet landed), and side `s`'s walkers eating there.
#[no_mangle]
pub extern "C" fn sk_pile_x(i: u32) -> f32 {
    with(|g| g.sk.dish.piles.get(i as usize).map_or(0.0, |p| p.x * SCALE as f32), 0.0)
}

#[no_mangle]
pub extern "C" fn sk_pile_y(i: u32) -> f32 {
    with(|g| g.sk.dish.piles.get(i as usize).map_or(0.0, |p| p.y * SCALE as f32), 0.0)
}

#[no_mangle]
pub extern "C" fn sk_pile_food(i: u32) -> f32 {
    with(
        |g| {
            g.sk.dish
                .piles
                .get(i as usize)
                .map_or(0.0, |p| if p.full > 0.0 { (p.food / p.full).max(0.0) } else { 0.0 })
        },
        0.0,
    )
}

#[no_mangle]
pub extern "C" fn sk_eaters(s: u32, i: u32) -> u32 {
    with(|g| if (i as usize) < g.sk.dish.piles.len() { g.sk.eaters(side(s), i as usize) } else { 0 }, 0)
}

/// Seconds left on side `s`'s beacon (`kind` 0), rally (1) or Fortify
/// (2) on pile `i`, 0 if none is lit.
#[no_mangle]
pub extern "C" fn sk_lit(s: u32, kind: u32, i: u32) -> f32 {
    with(
        |g| {
            let sd = &g.sk.sides[side(s)];
            let list = match kind {
                0 => &sd.beacons,
                1 => &sd.rallies,
                _ => &sd.fortified,
            };
            list.iter()
                .find(|b| b.0 == i as usize)
                .map_or(0.0, |b| b.1.saturating_sub(g.sk.dish.t) as f32 / STEPS_PER_SECOND as f32)
        },
        0.0,
    )
}

/// Seconds until side `s`'s sleeping spore on pile `i` wakes, 0 if none.
#[no_mangle]
pub extern "C" fn sk_landing(s: u32, i: u32) -> f32 {
    with(
        |g| {
            g.sk.spores
                .iter()
                .filter(|p| p.side == side(s) && p.pile == i as usize)
                .map(|p| p.wakes.saturating_sub(g.sk.dish.t) as f32 / STEPS_PER_SECOND as f32)
                .fold(0.0, f32::max)
        },
        0.0,
    )
}

/// Side `s`'s bank.
#[no_mangle]
pub extern "C" fn sk_bank(s: u32) -> f32 {
    with(|g| g.sk.sides[side(s)].bank, 0.0)
}

/// Your tithe, as an index into `TITHES` (0, 10, 25, 40%); side 1's too.
#[no_mangle]
pub extern "C" fn sk_tithe(s: u32) -> u32 {
    with(
        |g| {
            let t = g.sk.sides[side(s)].tithe;
            TITHES.iter().position(|&x| (x - t).abs() < 1e-4).unwrap_or(0) as u32
        },
        0,
    )
}

#[no_mangle]
pub extern "C" fn sk_set_tithe(k: u32) {
    with(
        |g| {
            if let Some(&t) = TITHES.get(k as usize) {
                g.sk.set_tithe(0, t);
            }
        },
        (),
    )
}

/// What verb `v` (see `verb`) costs you now; a spore's price can rise.
#[no_mangle]
pub extern "C" fn sk_price(v: u32) -> f32 {
    with(|g| verb(v, 0).map_or(0.0, |v| g.sk.price(0, v)), 0.0)
}

/// Whether you could buy verb `v` on `arg` now.
#[no_mangle]
pub extern "C" fn sk_can(v: u32, arg: u32) -> u32 {
    with(|g| verb(v, arg).is_some_and(|v| g.sk.can(0, v)) as u32, 0)
}

/// Buy verb `v` on `arg` (a pile, or a habit for verb 3): 1 if it took.
#[no_mangle]
pub extern "C" fn sk_act(v: u32, arg: u32) -> u32 {
    with(|g| verb(v, arg).is_some_and(|v| g.sk.act(0, v)) as u32, 0)
}

/// Side `s`'s purchases so far: `k` 0 beacons, 1 spores, 2 rallies, 3
/// habit switches, 4 fortifies.
#[no_mangle]
pub extern "C" fn sk_bought(s: u32, k: u32) -> u32 {
    with(
        |g| {
            let t = &g.sk.sides[side(s)].tally;
            match k {
                0 => t.beacons,
                1 => t.spores,
                2 => t.rallies,
                3 => t.switches,
                _ => t.fortifies,
            }
        },
        0,
    )
}

#[no_mangle]
pub extern "C" fn sk_pause(on: u32) {
    with(|g| g.view.paused = on != 0, ())
}

#[no_mangle]
pub extern "C" fn sk_fast(on: u32) {
    with(|g| g.view.fast = on != 0, ())
}

/// Put the briefing card away.
#[no_mangle]
pub extern "C" fn sk_unbrief() {
    with(|g| g.view.brief = false, ())
}

#[no_mangle]
pub extern "C" fn sk_briefed() -> u32 {
    with(|g| !g.view.brief as u32, 0)
}

/// Your habit (side 0) or the rival's (side 1), as `Habit::ALL` indices.
#[no_mangle]
pub extern "C" fn sk_habit(s: u32) -> u32 {
    with(
        |g| {
            let h = g.sk.sides[side(s)].habit;
            Habit::ALL.iter().position(|&x| x == h).unwrap_or(0) as u32
        },
        0,
    )
}

#[no_mangle]
pub extern "C" fn sk_cooldown() -> f32 {
    with(|g| g.sk.cooldown_s(0), 0.0)
}

/// Who wins as things stand (`Skirmish::winner`): 0 you, 1 the rival, 2 a
/// tie. A wipe-out is the survivor's whatever the score.
#[no_mangle]
pub extern "C" fn sk_winner() -> u32 {
    with(|g| g.sk.winner().map_or(2, |s| s as u32), 2)
}

/// Your share of the score in force: food eaten, or points for ground
/// held (`Rules::score_held`).
#[no_mangle]
pub extern "C" fn sk_share() -> f32 {
    with(|g| g.sk.share(), 0.5)
}

/// Your share of the food eaten and of the points for ground held, whichever
/// score is in force (the page shows both).
#[no_mangle]
pub extern "C" fn sk_food_share() -> f32 {
    with(|g| g.sk.food_share(), 0.5)
}

#[no_mangle]
pub extern "C" fn sk_points_share() -> f32 {
    with(|g| g.sk.points_share(), 0.5)
}

/// 1 when the score is ground held, 0 when it is food eaten.
#[no_mangle]
pub extern "C" fn sk_score_held() -> u32 {
    with(|g| g.sk.rules.score_held as u32, 0)
}

/// The bell, in seconds (150, or the rule set's).
#[no_mangle]
pub extern "C" fn sk_bell() -> u32 {
    with(|g| g.sk.rules.bell as u32, 150)
}

/// Seconds a spore counts down before it lands (0: at once).
#[no_mangle]
pub extern "C" fn sk_strike_delay() -> u32 {
    with(|g| g.sk.rules.strike_delay as u32, 0)
}

/// The piles' regrowth, a fraction of full a second (0: none).
#[no_mangle]
pub extern "C" fn sk_regrow() -> f32 {
    with(|g| g.sk.rules.regrow, 0.0)
}

/// Seconds until side `s`'s strike lands on pile `i` (`Rules::strike_delay`),
/// 0 if none is counting down there.
#[no_mangle]
pub extern "C" fn sk_strike(s: u32, i: u32) -> f32 {
    with(
        |g| {
            g.sk.strikes
                .iter()
                .filter(|k| k.side == side(s) && k.pile == i as usize)
                .map(|k| k.lands.saturating_sub(g.sk.dish.t) as f32 / STEPS_PER_SECOND as f32)
                .fold(0.0, f32::max)
        },
        0.0,
    )
}

/// 1 if side `s` holds pile `i` for the points (its eaters there at least
/// twice the other side's).
#[no_mangle]
pub extern "C" fn sk_holds(s: u32, i: u32) -> u32 {
    with(|g| ((i as usize) < g.sk.dish.piles.len() && g.sk.holds(side(s), i as usize)) as u32, 0)
}

/// Walkers in side `s`'s mould.
#[no_mangle]
pub extern "C" fn sk_walkers(s: u32) -> u32 {
    with(|g| g.sk.walkers(side(s)) as u32, 0)
}

#[no_mangle]
pub extern "C" fn sk_seconds() -> f32 {
    with(|g| g.sk.dish.t as f32 / STEPS_PER_SECOND as f32, 0.0)
}

/// 1 once the dish is over (the bell has rung, or one mould is gone).
#[no_mangle]
pub extern "C" fn sk_over() -> u32 {
    with(|g| (g.sk.dish.phase == Phase::Over) as u32, 0)
}
