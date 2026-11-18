//! LAB: the steering prototype (2026-11-17, the lab thread; this branch
//! only, never the main game).
//!
//! **The idea.** The slime's walkers turn toward whichever of three
//! sensors reads highest. Almost everything the player owns writes damage;
//! this writes a *direction*: one more term in the sensors, a field the
//! game builds from what the player has placed, which every walker leans
//! down. The field rides in the far smell's slot on the GPU (a negative
//! `far_smell` is its weight; `engine.wgsl`, the LAB block in `sense`), so
//! there is no new pass, no new buffer and no CPU twin.
//!
//! **What the player places** (all existing buildings, so the game's own
//! commands, costs and looks carry the prototype):
//!
//! - a **lure** (`Kind::Lure`, on and whole): the field is the walking
//!   distance to the nearest lure, out to `LAB_REACH` dish cells of
//!   walking. Walking distance goes round rock and walls, so a wall with a
//!   gap in it funnels what the lure draws.
//! - a **pusher** (`Kind::Siren`, on and whole, standing in for a new
//!   building): a hill `LAB_PUSH` dish cells of walking round it, so
//!   walkers lean away from it.
//!
//! Once a game second the field started a second ago is set and the next
//! is started on a worker thread, as the units' walking distance
//! (`Game::time_unit_paths`), so a replay does not depend on the thread.

use super::raids::Way;
use super::{Game, Kind, World};
use crate::mould::big::units_field_for;

/// A lab knob from the environment (`LAB_REACH=120`), read every time:
/// this is a film rig, not a rule.
fn knob(name: &str, default: f32) -> f32 {
    std::env::var(name).ok().and_then(|v| v.parse().ok()).unwrap_or(default)
}

/// A mark: a lure (or a pusher) at `at` (world units, fine cells, as
/// `centre=` and the squad's places) from game second `from` for `secs`.
#[derive(Clone, Copy, Debug)]
pub struct Mark {
    pub push: bool,
    pub from: f32,
    pub at: (f32, f32),
    pub secs: f32,
}

/// `LAB_MARKS=lure:T:X:Y:SECS;push:T:X:Y:SECS;...`, read every time.
pub fn marks() -> Vec<Mark> {
    let Ok(v) = std::env::var("LAB_MARKS") else { return Vec::new() };
    marks_of(&v)
}

fn marks_of(v: &str) -> Vec<Mark> {
    v.split(';')
        .filter_map(|m| {
            let w: Vec<&str> = m.split(':').collect();
            let [kind, t, x, y, secs] = w.as_slice() else { return None };
            Some(Mark { push: *kind == "push", from: t.parse().ok()?, at: (x.parse().ok()?, y.parse().ok()?), secs: secs.parse().ok()? })
        })
        .collect()
}

/// A mark a soldier's shot laid (`LAB_SHOT`, part two: a gun that lays the
/// pull along its shots): a fine cell (x | y << 16), the tick it goes out,
/// and whether it pushes (a hill) or draws (a lure).
#[derive(Clone, Copy, Debug)]
pub struct ShotMark {
    pub cell: u32,
    /// The tick it starts (a dart's landing; a shot's mark at once).
    pub from: u64,
    pub out: u64,
    pub push: bool,
}

/// Most shot marks alive at once; past it the oldest go.
const SHOT_MARKS_MAX: usize = 256;

/// A dart (part three, 2026-11-18: the gun carries the lure): fired by
/// squad 0 at game second `from` toward `aim` (world units), it lands
/// after its flight (`LAB_DART_SPEED` dish cells a second, default 60) no
/// farther than `LAB_DART_RANGE` dish cells (default 60) from the squad's
/// middle, lobbed over slime and walls, and stands there `secs` as a lure
/// or a pusher. `LAB_DARTS=lure|push:T:X:Y:SECS;...`, read every time.
pub fn darts() -> Vec<Mark> {
    let Ok(v) = std::env::var("LAB_DARTS") else { return Vec::new() };
    marks_of(&v)
}

/// A lone pusher's height at its rim (`steer_field`).
const PUSH_BASE: u32 = 60;

/// The steering field over the blocks of `agar` (`w` x `h` fine cells):
/// lower is where walkers lean. `lures` and `pushers` are fine cells
/// (x | y << 16); `reach` and `push` are walking distances in fine cells;
/// `slope` is the hill's height a fine cell. `u32::MAX` where nothing
/// reaches.
pub fn steer_field(agar: &[bool], w: usize, h: usize, lures: &[u32], pushers: &[u32], reach: u32, push: u32, slope: f32) -> Vec<u32> {
    if lures.is_empty() && pushers.is_empty() {
        return Vec::new();
    }
    let to_lure = if lures.is_empty() { Vec::new() } else { units_field_for(agar, w, h, lures) };
    let to_push = if pushers.is_empty() { Vec::new() } else { units_field_for(agar, w, h, pushers) };
    let n = to_lure.len().max(to_push.len());
    (0..n)
        .map(|q| {
            let dl = to_lure.get(q).copied().filter(|&d| d <= reach);
            let dp = to_push.get(q).copied().filter(|&d| d <= push);
            let hill = dp.map(|d| (slope * (push - d) as f32) as u32);
            match (dl, hill) {
                (None, None) => u32::MAX,
                // Lean to the lure, and off the pusher's hill where it
                // stands in the way.
                (Some(d), hill) => d + hill.unwrap_or(0),
                // A pusher alone: its hill, down to `PUSH_BASE` at its
                // rim (not 0: the view lights the field's lowest ground
                // as a lure's core, and a hill's rim is no lure).
                (None, Some(hl)) => hl + PUSH_BASE,
            }
        })
        .collect()
}

impl<W: World> Game<W> {
    /// Every step: the squad's shots lay their marks (`LAB_SHOT`), and
    /// the field is rebuilt every `LAB_EVERY` game seconds (default 1; a
    /// gun wants it quicker, and each field is still set one period late
    /// so a replay does not depend on the worker thread).
    ///
    /// `LAB_SHOT=push`: each soldier's bullet that hits slime lays a pusher
    /// on the hit, so the body leans away from where it is being shot.
    /// `LAB_SHOT=lure`: it lays a lure `LAB_SHOT_AHEAD` dish cells (default
    /// 20) past the hit along the bullet's line, so the body leans the way
    /// the squad fires. `LAB_SHOT=drive`: both. A mark lasts
    /// `LAB_SHOT_SECS` (default 3). Turrets' shots lay none.
    pub(super) fn lab_turn(&mut self, before: u64, now: u64) {
        if !self.world.lab_steer() {
            return;
        }
        let tps = self.rules.tps as f32;
        if let Ok(mode) = std::env::var("LAB_SHOT") {
            let (push, lure) = (mode != "lure", mode != "push");
            let out = now + (knob("LAB_SHOT_SECS", 3.0) * tps).round() as u64;
            let ahead = knob("LAB_SHOT_AHEAD", 20.0) * self.world.scale();
            let cell = |x: f32, y: f32| (x.max(0.0) as u32) | ((y.max(0.0) as u32) << 16);
            let mut laid = Vec::new();
            for s in self.last_shots.iter().filter(|s| s.by.is_some()) {
                let Some(h) = s.hit else { continue };
                if push {
                    laid.push(ShotMark { cell: cell(h.0, h.1), from: now, out, push: true });
                }
                if lure {
                    let (dx, dy) = (s.to.0 - s.from.0, s.to.1 - s.from.1);
                    let d = dx.hypot(dy).max(1e-3);
                    laid.push(ShotMark { cell: cell(h.0 + dx / d * ahead, h.1 + dy / d * ahead), from: now, out, push: false });
                }
            }
            let m = &mut self.raids.lab_shots;
            m.retain(|k| k.out > now);
            m.extend(laid);
            if m.len() > SHOT_MARKS_MAX {
                m.drain(..m.len() - SHOT_MARKS_MAX);
            }
        }
        // Darts fired this step (`darts`): from squad 0's middle, clamped
        // to the range, landing after the flight.
        let sc = self.world.scale();
        for d in darts() {
            let t = (d.from * tps).round() as u64;
            if !(before < t && t <= now) && !(before == 0 && t == 0) {
                continue;
            }
            let live: Vec<(f32, f32)> = self.soldiers.iter().filter(|s| s.alive && s.squad == 0).map(|s| (s.x, s.y)).collect();
            if live.is_empty() {
                continue;
            }
            let n = live.len() as f32;
            let from = (live.iter().map(|p| p.0).sum::<f32>() / n, live.iter().map(|p| p.1).sum::<f32>() / n);
            let (dx, dy) = (d.at.0 - from.0, d.at.1 - from.1);
            let dist = dx.hypot(dy).max(1e-3);
            let range = knob("LAB_DART_RANGE", 60.0) * sc;
            let go = dist.min(range);
            let at = (from.0 + dx / dist * go, from.1 + dy / dist * go);
            let land = now + (go / (knob("LAB_DART_SPEED", 60.0) * sc) * tps).round() as u64;
            let out = land + (d.secs * tps).round() as u64;
            println!("  lab dart t {:.2} s: {} from {:.0},{:.0} to {:.0},{:.0} (aimed {:.0},{:.0}), lands at {:.2} s", now as f32 / tps, if d.push { "pusher" } else { "lure" }, from.0, from.1, at.0, at.1, d.at.0, d.at.1, land as f32 / tps);
            let c = (at.0.max(0.0) as u32) | ((at.1.max(0.0) as u32) << 16);
            self.raids.lab_shots.push(ShotMark { cell: c, from: land, out, push: d.push });
        }
        let every = ((knob("LAB_EVERY", 1.0) * tps).round() as u64).max(1);
        if now / every != before / every {
            self.steer_turn();
        }
    }

    /// Every `LAB_EVERY` game seconds (`lab_turn`): set the field started
    /// a period ago, start the next from the lures, pushers and marks
    /// standing now.
    pub(super) fn steer_turn(&mut self) {
        if let Some(way) = self.raids.food_way.take() {
            let field = way.finish();
            // `LAB_LOG=1`: what the field reaches, and its value at each
            // crystal's centre (dish cells).
            if std::env::var("LAB_LOG").is_ok() && !field.is_empty() {
                if let Some((_, w, _)) = self.world.raid_ground() {
                    let bw = w.div_ceil(crate::mould::big::RAID_BLOCK);
                    let at = |c: (f32, f32)| field.get((c.1 as usize / crate::mould::big::RAID_BLOCK) * bw + c.0 as usize / crate::mould::big::RAID_BLOCK).copied().unwrap_or(u32::MAX);
                    let reached = field.iter().filter(|&&d| d != u32::MAX).count();
                    let cs: Vec<String> = self.world.crystals().iter().map(|c| format!("({:.0},{:.0}):{}", c.0.0, c.0.1, at(c.0) as i32)).collect();
                    println!("  lab t {:.2} s: field reaches {reached} of {} blocks; {} shot marks; at crystals {}", self.time(), field.len(), self.raids.lab_shots.len(), cs.join(" "));
                }
            }
            self.world.set_food_field(field);
        }
        let cell = |at: (f32, f32)| (at.0.max(0.0) as u32) | ((at.1.max(0.0) as u32) << 16);
        let of = |kind: Kind| self.buildings.iter().filter(|b| b.kind == kind && b.up() && b.on).map(|b| cell(b.at)).collect::<Vec<u32>>();
        let (mut lures, mut pushers) = (of(Kind::Lure), of(Kind::Siren));
        let sc = self.world.scale();
        // Marks (`LAB_MARKS`): a lure or a pusher with no building, at a
        // place for a time, as a thrown canister or a dart would leave.
        let now = self.world.ticks() as f32 / self.rules.tps as f32;
        for m in marks() {
            if now >= m.from && now < m.from + m.secs {
                let c = cell(m.at);
                if m.push { pushers.push(c) } else { lures.push(c) }
            }
        }
        let tick = self.world.ticks();
        for k in self.raids.lab_shots.iter().filter(|k| k.from <= tick && k.out > tick) {
            if k.push { pushers.push(k.cell) } else { lures.push(k.cell) }
        }
        let reach = (knob("LAB_REACH", 60.0) * sc) as u32;
        let push = (knob("LAB_PUSH", 25.0) * sc) as u32;
        let slope = knob("LAB_PUSH_SLOPE", 1.0);
        let Some((agar, w, h)) = self.world.raid_ground() else { return };
        self.raids.food_way = Some(Way::start_with(move || steer_field(&agar, w, h, &lures, &pushers, reach, push, slope)));
    }
}
