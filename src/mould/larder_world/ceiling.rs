//! **The slime's size follows the crystal it holds** (`Rules::ceiling`,
//! 2026-11-21, the thread "Make the slime a more interesting enemy").
//!
//! The owner, 11-21 06:30Z, on the rule that "crystals never run out from
//! eating or pumping, and only your squad can empty one by draining it":
//! "this 100%"; at 06:34Z he picked option 1 of three: the slime's size
//! follows the food it holds. With `crystal_keep` a crystal the slime sits
//! on feeds it for ever, so without a ceiling it refills its slots (twice
//! its landing walkers) on any crystal, and the squad taking crystals off
//! it changes nothing you can see.
//!
//! Once a game second the slime's ceiling is set from the crystals it
//! **holds**: those with walkers on at least `CEILING_HELD` of their cells
//! (smoothed over `CEILING_SMOOTH` seconds, crystal by crystal; let go
//! under `CEILING_LET_GO`), each
//! counted by the charge it has left. Times its walkers a unit of held
//! charge at the landing, at least `Rules::ceiling` a cell's worth. So a
//! planet starts with its slime at its ceiling (its start plays as
//! before), the ceiling rises as it takes more crystals and falls as the
//! squad drains or takes one, never over `Rules::ceiling_roof` times its
//! landing walkers or its slots.
//!
//! - Over the ceiling no walker is born (the engine's `CEIL`, species 0's
//!   birth cap).
//! - Over the ceiling its upkeep rises, to `Rules::ceiling_upkeep` times
//!   at `CEILING_OVER` over, so the walkers that lost their food starve
//!   within a minute or so and the slime shrinks to what it still holds;
//!   at or under it the upkeep is the shape's.
//! - A ceiling that falls under the slime is set at most `CEILING_FALL`
//!   under its size, a second at a time, so it shrinks over tens of
//!   seconds and not in one starving crash.
//!
//! Rejected:
//! - The first cut (#659, 11-21): the ceiling from the crystal *cells* it
//!   covered (with a walker on them) times its walkers a covered cell at
//!   landing, at most 25. Planet 3's Trunk broke it (the Enemy lane's exit
//!   test 4 recheck, 11-22, run seeds 1-3): it landed at 32 a cell, so its
//!   ceiling was under its size from the first second, or its walkers
//!   spread off the crystal (1,856 covered cells to 1,242 by 0:24, seed
//!   1); either way upkeep x3 took it from about 40,000 walkers to 12,000
//!   to 15,000 by 0:45, which set off its strikers (under 60% of its peak)
//!   with no cut by the player, and seed 2's squad died at 2:57. How many
//!   walkers stand on a crystal is the slime's shape, not its food.
//!
//! And before it (measured 11-21 morning, planet 1, run seeds 1-3, greedy):
//! - The upkeep raised everywhere, all the time: x2 cut the slime at 1:00
//!   by 29% (7,959 against 11,261 on seed 1), x3 by 52%. At landing most of
//!   the slime is off its crystal on its start energy, so it starved.
//! - The floor at the landing walkers (the plan as first written): a slime
//!   that held any crystal at all regrew to its landing size, so draining
//!   its crystals did not shrink it. A floor on the density instead keeps
//!   the start and lets the size follow the crystal.
//! - A ceiling from the food it ate a second (its income) over the income
//!   at 1:00, enforced by upkeep alone (a prototype): income swings with
//!   surges and fights, so the ceiling swung with it, and upkeep never
//!   stopped births.

use super::{Game, World};

/// Seconds each crystal's covered cells are smoothed over: a surge or a
/// shot that empties a crystal's rim for a moment does not drop it.
pub const CEILING_SMOOTH: f64 = 5.0;

/// The share of a crystal's cells the slime must cover (smoothed) to take
/// it: a few walkers that wander onto a crystal do not.
pub const CEILING_HELD: f64 = 0.1;

/// The share under which it lets a crystal it held go: a crystal the squad
/// has cleared down to a fringe is no longer the slime's. Under
/// `CEILING_HELD`, so a crystal on the edge does not flicker in and out
/// (planet 3, seed 2, 11-22: its ceiling swung between 41,000 and 79,000
/// a few seconds apart with one share for both).
pub const CEILING_LET_GO: f64 = 0.05;

/// How far under the slime a falling ceiling may be set: when it loses a
/// crystal its ceiling follows it down, at most this share under its size,
/// so its upkeep stays mild while births are stopped, and it shrinks over
/// tens of seconds instead of all at once. With the whole fall at once
/// (11-22, planet 3, seed 1, the squad taking a crystal that held 45% of
/// its charge at 0:30), upkeep x3 for 20 s starved its whole off-crystal
/// body together: 36,683 walkers at 0:40, 16,224 at 0:50, half its new
/// ceiling.
pub const CEILING_FALL: f64 = 0.05;

/// How far over its ceiling the slime pays the whole of
/// `Rules::ceiling_upkeep` (a share; under it, in proportion).
pub const CEILING_OVER: f64 = 0.2;

#[derive(Clone, Debug, Default)]
pub struct Ceiling {
    /// The walkers the slime landed with, once read (0 before).
    pub start: usize,
    /// Its walkers a unit of held charge at the landing, at least
    /// `Rules::ceiling` a cell's worth.
    pub density: f64,
    /// Each crystal's covered cells, smoothed.
    pub cover: Vec<f64>,
    /// Each crystal's charge at the landing (its whole worth).
    pub full: Vec<f64>,
    /// Whether it holds each crystal now.
    pub holding: Vec<bool>,
    /// The charge the slime holds now.
    pub held: f64,
    /// The crystals it holds now.
    pub holds: usize,
    /// The ceiling now (0 before the first second or with the rule off).
    pub now: u32,
    /// The shape's upkeep, before any raise.
    pub upkeep: f32,
    /// The upkeep's factor now (1 at or under the ceiling).
    pub factor: f32,
}

impl<W: World> Game<W> {
    /// Once a game second, after `check_broken`: the slime's birth ceiling
    /// from the crystals it holds, and its upkeep over it
    /// (`Rules::ceiling`).
    pub(super) fn time_ceiling(&mut self) {
        let r = self.rules;
        if r.ceiling <= 0.0 {
            return;
        }
        let covered = self.world.crystal_covered_each();
        let cells = self.world.crystal_cells_each();
        let charge: Vec<f64> = self.world.crystals().iter().map(|c| c.1 as f64).collect();
        let (slots, mould) = (self.world.slots(), self.mould);
        let first = self.ceiling.start == 0;
        let c = &mut self.ceiling;
        if first {
            c.cover = covered.iter().map(|&v| v as f64).collect();
            c.full = charge.clone();
        }
        c.cover.resize(covered.len(), 0.0);
        c.holding.resize(covered.len(), false);
        for (s, &v) in c.cover.iter_mut().zip(&covered) {
            *s += (v as f64 - *s) / CEILING_SMOOTH;
        }
        // The charge it holds, and a cell's worth of it (for the density's
        // floor).
        let (mut held, mut held_cells) = (0.0, 0.0);
        c.holds = 0;
        for (j, (&cov, hold)) in c.cover.iter().zip(c.holding.iter_mut()).enumerate() {
            let n = cells.get(j).copied().unwrap_or(0) as f64;
            *hold = n > 0.0 && cov >= n * if *hold { CEILING_LET_GO } else { CEILING_HELD };
            if *hold {
                let (now, full) = (charge.get(j).copied().unwrap_or(0.0), c.full.get(j).copied().unwrap_or(0.0));
                held += now.min(full).max(0.0);
                held_cells += n * if full > 0.0 { (now / full).clamp(0.0, 1.0) } else { 0.0 };
                c.holds += 1;
            }
        }
        c.held = held;
        if first {
            if held <= 0.0 {
                // Landed holding nothing (a world that does not count its
                // crystals): no ceiling yet.
                c.cover.clear();
                c.holding.clear();
                return;
            }
            c.start = mould.max(1);
            let per_cell = held / held_cells.max(1.0);
            c.density = (mould as f64 / held).max(r.ceiling as f64 / per_cell.max(1e-9));
            // `ceiling_grow` (11-22): room over the landing to regrow
            // into, on the density so the ceiling still follows the
            // charge it holds.
            c.density *= r.ceiling_grow.max(0.1) as f64;
            c.upkeep = self.world.upkeep();
        }
        let mut ceil = (held * c.density).min(c.start as f64 * r.ceiling_roof.max(1.0) as f64);
        if slots > 0 {
            ceil = ceil.min(slots as f64);
        }
        if !first && ceil < mould as f64 * (1.0 - CEILING_FALL) {
            ceil = (mould as f64 * (1.0 - CEILING_FALL)).min(c.now as f64);
        }
        c.now = ceil.round().max(1.0) as u32;
        self.world.set_ceiling(c.now);
        // A broken slime starves on its own rule (`check_broken`), which
        // saves and gives back the upkeep it found.
        if self.broken.is_some() {
            return;
        }
        let over = ((mould as f64 / ceil.max(1.0) - 1.0) / CEILING_OVER).clamp(0.0, 1.0);
        c.factor = 1.0 + (r.ceiling_upkeep - 1.0).max(0.0) * over as f32;
        // LAB: threads (2026-11-24; never merges). `LAB_HOLD=1` with
        // `LAB_CAPS`: the upkeep eases while the slime is under the off
        // arm's walkers that second (x 0.85 a second, at most x 0.1) and
        // comes back while it is at or over them, so a rule that costs the
        // slime walkers is judged by its look at the same count.
        let mut lab = 1.0f32;
        if std::env::var("LAB_HOLD").is_ok_and(|v| v != "0") {
            let sec = LAB_HOLD_SEC.fetch_add(1, std::sync::atomic::Ordering::Relaxed) as usize;
            let want = std::env::var("LAB_CAPS").ok().and_then(|f| std::fs::read_to_string(f).ok()).and_then(|t| t.lines().nth(sec + 1).and_then(|l| l.trim().parse::<f64>().ok()));
            let mut k = f32::from_bits(LAB_HOLD_K.load(std::sync::atomic::Ordering::Relaxed));
            if let Some(w) = want {
                k = if (mould as f64) < 0.97 * w { k * 0.85 } else if (mould as f64) >= w { k / 0.85 } else { k };
            }
            k = k.clamp(0.1, 1.0);
            LAB_HOLD_K.store(k.to_bits(), std::sync::atomic::Ordering::Relaxed);
            lab = k;
            eprintln!("lab hold t {sec}: slime {mould}, want {}, upkeep x{k:.3}", want.unwrap_or(0.0));
        }
        self.world.set_upkeep(c.upkeep * c.factor * lab);
    }
}

/// LAB: the seconds `LAB_HOLD` has seen and its upkeep factor (f32 bits).
static LAB_HOLD_SEC: std::sync::atomic::AtomicU32 = std::sync::atomic::AtomicU32::new(0);
static LAB_HOLD_K: std::sync::atomic::AtomicU32 = std::sync::atomic::AtomicU32::new(0x3F80_0000);
