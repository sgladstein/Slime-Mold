//! **The end plays out in the picture** (Look D, 2026-11-20): when a
//! planet is won or lost, the slime keeps living for a beat before the end
//! card comes up, and the camera draws back a little, so the moment is
//! seen on the planet and not only read on a card. On a loss the slime
//! flows on over the larder or the squad's last ground; on a wipe its
//! last film and trail fade as the world decays them. Nothing is painted:
//! it is the world's own steps.
//!
//! The game itself has ended (`Game::begin_step` steps no more once
//! `Game::end` is set, and past a mission's verdict the window steps it no
//! more), so the beat runs the world alone: frames with no soldiers and no
//! shots, at the game's pace. The game's counts (`Game::mould`, kills,
//! score) stay as they were at the end, and the clock is held at the end
//! (`Beat::at`), so the card and the top bar say what happened at the
//! end, not after it.
//!
//! Only in the window: a headless run, a census, the run bot and the
//! tests end at once as before (`App::beat` is None there).
//!
//! The beat's frames carry no soldiers: a frame's units are built by the
//! game's own step (`Game::begin_step`), which has ended. The slime
//! ignores a standing squad for the beat's two seconds, which does not
//! show at play zoom.

use super::*;
use slime_mould::mould::squad::{self, Frame};

/// Real seconds the picture plays on before the end card comes up.
/// Longer read as the game hanging; shorter, the slime barely moves.
pub(super) const END_BEAT: f32 = 2.4;
/// How far the camera draws back over the beat, as a share of its zoom.
const DRAW_BACK: f32 = 0.18;

/// The end's beat, from the game's start.
#[derive(Clone, Copy, Debug, Default)]
pub(super) struct Beat {
    /// Real seconds played since the end.
    pub played: f32,
    /// Game seconds owed to the world.
    owed: f32,
    /// The game clock at the end, held through the beat and after.
    pub at: Option<f32>,
    /// The camera's zoom at the end.
    zoom: f32,
}

impl App {
    /// The beat after the end: the world's own frames until `END_BEAT`.
    /// Called each frame the game is over and not paused.
    pub(super) fn play_on(&mut self, dt: f32) {
        let (Some(b), Some(g)) = (self.beat.as_mut(), self.game.as_mut()) else { return };
        if b.at.is_none() {
            // A mission's verdict comes after a step began (`follow_mission`
            // runs after the step loop): that step's results are in on the
            // GPU already, so they are taken (a fifteenth of a second)
            // before the world runs on its own.
            if g.in_flight() {
                g.finish_step();
            }
            b.at = Some(g.time());
            b.zoom = self.camera.zoom;
        }
        if b.played >= END_BEAT {
            return;
        }
        b.played += dt;
        let r = g.rules;
        let step = r.frame_ticks as f32 / r.tps as f32;
        b.owed += dt.min(0.25);
        // Its ticks in one share: the world's spread would leave a readout
        // out that the game knows nothing of.
        g.world.spread = 1;
        let mut n = 0;
        while b.owed >= step && n < 4 {
            let f = Frame { units: Vec::new(), shots: Vec::new(), slime: squad::slime_trail(6.0), crater: 0, sink: r.sink, spray: 0, spray_speed: 0.0, spray_share: r.spray_share, catch: Vec::new() };
            g.world.frame(&f, r.frame_ticks);
            b.owed -= step;
            n += 1;
        }
        if n >= 4 {
            b.owed = 0.0;
        }
        // The draw-back, eased out, about the screen's middle.
        let u = (b.played / END_BEAT).min(1.0);
        let z = b.zoom * (1.0 - DRAW_BACK * (1.0 - (1.0 - u) * (1.0 - u)));
        let mid = (self.camera.screen.0 / 2.0, self.camera.screen.1 / 2.0);
        self.camera.zoom_to(z, mid);
    }

    /// Whether the end card is up: at once without a beat, else once the
    /// beat has played.
    pub(super) fn end_shown(&self) -> bool {
        self.beat.is_none_or(|b| b.played >= END_BEAT)
    }

    /// The game clock, held at the end through the beat (`Beat::at`).
    pub(super) fn clock_now(&self, g: &Game<GpuWorld>) -> f32 {
        self.beat.and_then(|b| b.at).unwrap_or_else(|| g.time())
    }
}
