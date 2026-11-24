//! **Auto's World detail that follows the frame rate** (4K thread,
//! 2026-11-24; Direction's pick "E" of the planet 8 options). Auto starts at
//! the screen's step (`settings::auto_percent`: 50% on the owner's 4K
//! screen) and steps down toward `FLOOR` only while frames run long, then
//! comes back up once they have run short for a while.
//!
//! Why: on the GTX 1070 at 3840 x 2004 and 50%, planets 1 to 5 hold 60
//! frames a second (planet 3 p95 12.5 ms, planet 5 at its 132k peak 15.5),
//! but planet 8 at its 221k peak ran 17.7 ms p50 and 19.5 p95: 12.4 ms of
//! GPU a simulation step (3.1 a frame) and a 12.2 ms view. 40% saved 2.5 ms
//! there. His own planet 8 (11-22) peaked at landing (209k mould) and was
//! half that by 3:00, so only its first minutes need the lower step.
//!
//! How: each second's frames give a 90th-percentile frame time, `P90`
//! (the interval between frames in the window, where vsync holds a quick
//! frame to 16.7 ms; the frame's own time in the bench, which has no
//! vsync). A second whose `P90` is past `SLOW_MS` steps down one step at
//! once. After `hold` seconds with every `P90` under `FAST_MS` it steps up
//! one; if
//! that step's first `PROBE_SECS` seconds hold a slow one, it steps back
//! down and `hold` doubles (to `HOLD_MAX`), so a step that does not fit is
//! tried less and less often. A new game starts again from the top. Not
//! the mean or the worst frame, so a few hitches a second (a boss's CPU
//! turn, a ground refresh: up to a tenth of its frames) do not cost the
//! picture its detail.
//!
//! The median first (#759), then the 90th percentile (11-24, timed on the
//! 1070 with Engine's walker sort, #789): planet 8 at 50% ran a median
//! frame of 16.1 ms, under `SLOW_MS`, while its frames with a game step,
//! one in four, ran 18.9 at their p95, so Auto held 50% for 126 s with
//! every fourth frame late (a judder in a vsync'd window), p95 20.6. At
//! 40% the same frames are p95 16.4. A median cannot see a slow quarter;
//! a 90th percentile can, and still ignores one hitch in ten.
//!
//! Rejected: GPU timestamps a frame to see the headroom before stepping
//! up (a query set and a read-back each frame in the window, for a probe
//! that costs at most a second or two of long frames, rarer each time).

use slime_mould::world::settings;

/// A second whose `P90` frame is past this is slow (ms): a frame over
/// 60 a second's 16.67 by more than vsync's jitter.
pub(super) const SLOW_MS: f32 = 17.5;
/// Every second's `P90` under this for `hold` seconds: try a step up.
pub(super) const FAST_MS: f32 = 17.0;
/// The lowest step Auto goes to (one world pixel to each of the owner's
/// screen points at 250%).
pub(super) const FLOOR: u32 = 40;
/// The share of a second's frames at or under the frame time it is judged
/// by: its 90th percentile.
pub(super) const P90: f32 = 0.9;
/// Seconds of fast frames before the first try at a step up, and the
/// most it doubles to.
pub(super) const HOLD_MIN: f32 = 15.0;
pub(super) const HOLD_MAX: f32 = 240.0;
/// A step up that runs slow within this many seconds goes back down.
pub(super) const PROBE_SECS: f32 = 3.0;

#[derive(Clone, Debug)]
pub(super) struct AutoDetail {
    /// The step now, percent each way; 0 until the first `percent`.
    pct: u32,
    /// The top: the screen's own Auto step (`settings::auto_percent`).
    top: u32,
    /// This second's frame times (ms) and the time they add up to.
    frames: Vec<f32>,
    sum: f32,
    /// Seconds in a row with a fast `P90` since the last change.
    fast_for: f32,
    /// Seconds since the last step up (a probe while under `PROBE_SECS`).
    since_up: Option<f32>,
    hold: f32,
}

impl Default for AutoDetail {
    fn default() -> Self {
        AutoDetail { pct: 0, top: 0, frames: Vec::with_capacity(128), sum: 0.0, fast_for: 0.0, since_up: None, hold: HOLD_MIN }
    }
}

impl AutoDetail {
    /// The step to draw at on a `screen`-sized window. A new screen size
    /// (or the first call) starts at its top.
    pub(super) fn percent(&mut self, screen: (u32, u32)) -> u32 {
        let top = settings::auto_percent(screen);
        if top != self.top {
            *self = AutoDetail { top, pct: top, ..AutoDetail::default() };
        }
        self.pct
    }

    /// Back to the top, with the hold reset: a new game.
    pub(super) fn restart(&mut self) {
        let top = self.top;
        *self = AutoDetail { top, pct: top, ..AutoDetail::default() };
    }

    /// One frame of `ms`. Returns true when the step changed.
    pub(super) fn frame(&mut self, ms: f32) -> bool {
        if self.top == 0 || !ms.is_finite() || ms <= 0.0 {
            return false;
        }
        // A frame over a quarter second is a load or a pause, not the
        // picture's cost: it neither counts nor ends a second.
        if ms > 250.0 {
            return false;
        }
        self.frames.push(ms);
        self.sum += ms;
        if self.sum < 1000.0 {
            return false;
        }
        let mut f = std::mem::take(&mut self.frames);
        f.sort_by(f32::total_cmp);
        let p90 = f[((f.len() as f32 * P90) as usize).min(f.len() - 1)];
        let secs = self.sum / 1000.0;
        (self.frames, self.sum) = (f, 0.0);
        self.frames.clear();
        self.second(p90, secs)
    }

    fn second(&mut self, p90: f32, secs: f32) -> bool {
        if let Some(t) = self.since_up.as_mut() {
            *t += secs;
        }
        if p90 > SLOW_MS {
            self.fast_for = 0.0;
            // A step up that did not fit: the next try waits twice as long.
            if self.since_up.is_some_and(|t| t <= PROBE_SECS) {
                self.hold = (self.hold * 2.0).min(HOLD_MAX);
            }
            self.since_up = None;
            return self.step(false);
        }
        if p90 < FAST_MS {
            self.fast_for += secs;
            if self.fast_for >= self.hold && self.pct < self.top {
                self.fast_for = 0.0;
                self.since_up = Some(0.0);
                return self.step(true);
            }
        } else {
            self.fast_for = 0.0;
        }
        false
    }

    /// One step of `settings::DETAIL_STEPS` up or down, within `FLOOR`
    /// and the top.
    fn step(&mut self, up: bool) -> bool {
        let steps: Vec<u32> = settings::DETAIL_STEPS.iter().filter_map(|d| d.strip_suffix('%')?.parse().ok()).filter(|&p| p >= FLOOR.min(self.top) && p <= self.top).collect();
        // Largest first: 100, 85, ... 40.
        let at = steps.iter().position(|&p| p <= self.pct).unwrap_or(steps.len() - 1);
        let to = if up { at.checked_sub(1).map_or(steps[0], |i| steps[i]) } else { steps[(at + 1).min(steps.len() - 1)] };
        let changed = to != self.pct;
        self.pct = to;
        changed
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    const UHD: (u32, u32) = (3840, 2004);

    /// `n` seconds of frames of `ms`.
    fn run(a: &mut AutoDetail, ms: f32, secs: f32) -> Vec<u32> {
        let mut seen = Vec::new();
        let n = (secs * 1000.0 / ms).round() as usize;
        for _ in 0..n {
            if a.frame(ms) {
                seen.push(a.pct);
            }
        }
        seen
    }

    #[test]
    fn quick_frames_keep_the_screens_own_step() {
        let mut a = AutoDetail::default();
        assert_eq!(a.percent(UHD), 50);
        assert!(run(&mut a, 16.67, 120.0).is_empty());
        assert_eq!(a.percent(UHD), 50);
    }

    #[test]
    fn slow_seconds_step_down_one_a_second_to_the_floor_and_no_further() {
        let mut a = AutoDetail::default();
        a.percent(UHD);
        assert_eq!(run(&mut a, 19.5, 1.05), vec![40], "one slow second: 50 to 40");
        assert!(run(&mut a, 19.5, 10.0).is_empty(), "the floor holds");
        assert_eq!(a.percent(UHD), FLOOR);
        // A screen whose top is 100: one step a slow second.
        let mut b = AutoDetail::default();
        assert_eq!(b.percent((1280, 800)), 100);
        assert_eq!(run(&mut b, 33.3, 3.1), vec![85, 75, 67]);
    }

    #[test]
    fn one_hitch_a_second_does_not_step_down() {
        let mut a = AutoDetail::default();
        a.percent(UHD);
        for _ in 0..30 {
            for _ in 0..58 {
                assert!(!a.frame(16.67));
            }
            assert!(!a.frame(70.0), "a 70 ms boss turn among quick frames");
        }
        assert_eq!(a.percent(UHD), 50);
    }

    #[test]
    fn it_comes_back_up_after_the_hold_and_backs_off_a_step_that_does_not_fit() {
        let mut a = AutoDetail::default();
        a.percent(UHD);
        run(&mut a, 19.0, 1.05);
        assert_eq!(a.pct, 40);
        // Quick at 40%: up after HOLD_MIN seconds.
        assert_eq!(run(&mut a, 15.0, HOLD_MIN + 0.5), vec![50]);
        // 50% is still slow: back down at its first whole slow second (the
        // first may hold the quick frames before it), and the hold doubles.
        assert_eq!(run(&mut a, 19.0, 2.1), vec![40]);
        assert_eq!(a.hold, 2.0 * HOLD_MIN);
        assert!(run(&mut a, 15.0, HOLD_MIN + 0.5).is_empty(), "not after the old hold");
        assert_eq!(run(&mut a, 15.0, HOLD_MIN + 0.5), vec![50], "after the doubled one");
        // This time it fits: it stays, and never goes past the top.
        assert!(run(&mut a, 16.67, HOLD_MAX + 10.0).is_empty());
        assert_eq!(a.pct, 50);
        // A new game starts from the top with the hold reset.
        run(&mut a, 19.0, 1.05);
        a.restart();
        assert_eq!((a.percent(UHD), a.hold), (50, HOLD_MIN));
    }

    #[test]
    fn a_slow_quarter_of_the_frames_steps_down() {
        // Planet 8 at 50% with the walker sort (1070, 11-24): one frame in
        // four takes a game step and runs late, the median stays quick.
        let mut a = AutoDetail::default();
        a.percent(UHD);
        let mut seen = Vec::new();
        for i in 0..200 {
            if a.frame(if i % 4 == 0 { 19.0 } else { 15.5 }) {
                seen.push(a.pct);
            }
        }
        assert_eq!(seen, vec![40]);
        // Five hitches in sixty quick frames is under a tenth.
        let mut b = AutoDetail::default();
        b.percent(UHD);
        for _ in 0..20 {
            for i in 0..60 {
                assert!(!b.frame(if i % 12 == 0 { 40.0 } else { 15.0 }));
            }
        }
        assert_eq!(b.pct, 50);
    }

    #[test]
    fn loads_and_pauses_do_not_count() {
        let mut a = AutoDetail::default();
        a.percent(UHD);
        for _ in 0..20 {
            assert!(!a.frame(900.0));
        }
        assert_eq!(a.pct, 50);
    }
}
