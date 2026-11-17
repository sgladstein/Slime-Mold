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
    v.split(';')
        .filter_map(|m| {
            let w: Vec<&str> = m.split(':').collect();
            let [kind, t, x, y, secs] = w.as_slice() else { return None };
            Some(Mark { push: *kind == "push", from: t.parse().ok()?, at: (x.parse().ok()?, y.parse().ok()?), secs: secs.parse().ok()? })
        })
        .collect()
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
    /// Once a game second (`time_food_paths`): set the field started a
    /// second ago, start the next from the lures and pushers standing now.
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
                    println!("  lab t {:.0} s: field reaches {reached} of {} blocks; at crystals {}", self.time(), field.len(), cs.join(" "));
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
        let reach = (knob("LAB_REACH", 60.0) * sc) as u32;
        let push = (knob("LAB_PUSH", 25.0) * sc) as u32;
        let slope = knob("LAB_PUSH_SLOPE", 1.0);
        let Some((agar, w, h)) = self.world.raid_ground() else { return };
        self.raids.food_way = Some(Way::start_with(move || steer_field(&agar, w, h, &lures, &pushers, reach, push, slope)));
    }
}
