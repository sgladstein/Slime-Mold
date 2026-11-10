//! The field skirmish in a browser: the game's own frame (`render::draw`,
//! the game's look) drawn into a buffer the page copies to a canvas. A
//! plain C interface, so the page needs no generated glue: it calls these
//! `sk_*` functions and reads the frame out of the module's memory.
//!
//! This mirrors the window's handling in `src/bin/mould.rs` for the parts
//! the skirmish uses: a press on the dish holds the lamp or starts a salt
//! stroke, a press on a tool tile picks the tool, a release lets go, and
//! the briefing card is put away by the first press.

use std::cell::RefCell;

use slime_mould::mould::look::Style;
use slime_mould::mould::render::{self, Press, View};
use slime_mould::mould::skirmish::{Habit, Plan, Skirmish, Verb, TITHES};
use slime_mould::mould::{Phase, Tool, STEPS_PER_FRAME};

/// Frame pixels per dish cell, as the window's.
const SCALE: u32 = 4;
/// One tick is a sixtieth of a second: `STEPS_PER_FRAME` steps.
const TICK_MS: f64 = 1000.0 / 60.0;
const MAX_TICKS: u32 = 4;
const FAST: u32 = 4;

struct Game {
    sk: Skirmish,
    view: View,
    frame: Vec<u8>,
    pointer: Option<(f32, f32)>,
    cursor: Option<(f32, f32)>,
    pressed: bool,
    acc: f64,
}

thread_local! {
    static GAME: RefCell<Option<Game>> = const { RefCell::new(None) };
}

fn with<R>(f: impl FnOnce(&mut Game) -> R, none: R) -> R {
    GAME.with(|g| g.borrow_mut().as_mut().map(f).unwrap_or(none))
}

impl Game {
    fn hold(&mut self) {
        let lamp = match (self.pressed, self.view.tool, self.cursor) {
            (true, Tool::Lamp, Some(c)) => Some(c),
            _ => None,
        };
        self.sk.dish.set_lamp(lamp);
    }

    fn let_go(&mut self) {
        self.pressed = false;
        self.sk.dish.end_stroke();
        self.hold();
    }

    fn button(&mut self, down: bool) {
        if self.view.brief {
            if down {
                self.view.brief = false;
            }
            return self.let_go();
        }
        if !down {
            return self.let_go();
        }
        let Some((px, py)) = self.pointer else { return };
        let (x, y) = match render::press_at(&self.sk.dish, &self.view, SCALE, px, py) {
            Press::Dish(x, y) => (x, y),
            Press::Tile(t) => {
                if self.sk.dish.bar_tools().contains(&t) {
                    self.view.tool = t;
                }
                return self.let_go();
            }
            Press::Off => return,
        };
        self.pressed = true;
        if self.view.tool == Tool::Salt {
            self.sk.dish.begin_stroke(Tool::Salt, x, y);
        }
        self.hold();
    }

    fn moved(&mut self, px: f32, py: f32, inside: bool) {
        if !inside {
            self.pointer = None;
            self.cursor = None;
            return self.hold();
        }
        self.pointer = Some((px, py));
        let (x, y) = (px / SCALE as f32, py / SCALE as f32);
        self.cursor = (y < slime_mould::mould::H as f32).then_some((x, y));
        if let Some((x, y)) = self.cursor {
            if self.pressed && self.sk.dish.stroke_active() {
                self.sk.dish.drag_stroke(x, y);
            }
        }
        self.hold();
    }

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
            self.view.cursor = self.cursor;
            render::draw(&self.sk.dish, &self.view, SCALE, &mut self.frame);
        }
    }
}

/// A new skirmish on dish `seed`. The briefing card is up until the first
/// press.
///
/// Interim, until the budget skirmish has its own page: this is still the
/// first page's interface over the new rules (`skirmish.rs`). The rival
/// plays the counter plan (habits only), `react_s` is no longer used, and
/// you tithe 10% so a habit switch can be paid for.
#[no_mangle]
pub extern "C" fn sk_new(seed: u32, _react_s: u32) {
    let mut sk = Skirmish::new(seed as u64, [None, Some(Plan::Counter)]);
    sk.set_tithe(0, TITHES[1]);
    let (w, h) = render::frame_size(SCALE);
    let mut g = Game {
        sk,
        view: View {
            brief: true,
            // The window's key list names the herding game's keys (M:
            // dishes, H: help); the page lists this one's.
            keys: false,
            style: Style::GAME,
            tool: Tool::Lamp,
            ..View::default()
        },
        frame: vec![0; (w * h * 4) as usize],
        pointer: None,
        cursor: None,
        pressed: false,
        acc: 0.0,
    };
    g.tick(0.0, true);
    GAME.with(|cell| *cell.borrow_mut() = Some(g));
}

#[no_mangle]
pub extern "C" fn sk_frame_w() -> u32 {
    render::frame_size(SCALE).0
}

#[no_mangle]
pub extern "C" fn sk_frame_h() -> u32 {
    render::frame_size(SCALE).1
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

/// The pointer, in frame pixels; `inside` 0 when it left the canvas.
#[no_mangle]
pub extern "C" fn sk_pointer(px: f32, py: f32, inside: u32) {
    with(|g| g.moved(px, py, inside != 0), ())
}

#[no_mangle]
pub extern "C" fn sk_button(down: u32) {
    with(|g| g.button(down != 0), ())
}

/// Switch your habit (0 spread, 1 hunt, 2 maze, 3 cord): 1 if it took.
#[no_mangle]
pub extern "C" fn sk_switch(h: u32) -> u32 {
    with(
        |g| Habit::ALL.get(h as usize).is_some_and(|&h| g.sk.act(0, Verb::Habit(h))) as u32,
        0,
    )
}

/// Pick a tool: 0 lamp, 2 salt (`Tool::EVERY`'s order).
#[no_mangle]
pub extern "C" fn sk_tool(t: u32) {
    with(
        |g| {
            if let Some(&t) = Tool::EVERY.get(t as usize) {
                if g.sk.dish.bar_tools().contains(&t) {
                    g.view.tool = t;
                }
            }
            g.let_go();
        },
        (),
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

/// Put the briefing card away (a key does it as well as a press).
#[no_mangle]
pub extern "C" fn sk_unbrief() {
    with(|g| g.view.brief = false, ())
}

/// Your habit (side 0) or the rival's (side 1), as `sk_switch` numbers.
#[no_mangle]
pub extern "C" fn sk_habit(side: u32) -> u32 {
    with(
        |g| {
            let h = g.sk.sides[(side as usize).min(1)].habit;
            Habit::ALL.iter().position(|&x| x == h).unwrap_or(0) as u32
        },
        0,
    )
}

#[no_mangle]
pub extern "C" fn sk_cooldown() -> f32 {
    with(|g| g.sk.cooldown_s(0), 0.0)
}

#[no_mangle]
pub extern "C" fn sk_share() -> f32 {
    with(|g| g.sk.share(), 0.5)
}

#[no_mangle]
pub extern "C" fn sk_seconds() -> f32 {
    with(
        |g| g.sk.dish.t as f32 / slime_mould::mould::STEPS_PER_SECOND as f32,
        0.0,
    )
}

/// 1 once the dish is over (the bell has rung and the result is up).
#[no_mangle]
pub extern "C" fn sk_over() -> u32 {
    with(|g| (g.sk.dish.phase == Phase::Over) as u32, 0)
}

#[no_mangle]
pub extern "C" fn sk_salt() -> u32 {
    with(|g| g.sk.dish.salt_left, 0)
}

#[no_mangle]
pub extern "C" fn sk_tool_now() -> u32 {
    with(
        |g| Tool::EVERY.iter().position(|&t| t == g.view.tool).unwrap_or(0) as u32,
        0,
    )
}
