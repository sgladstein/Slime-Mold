//! **The sound's volume** (sound pass two, 2026-11-20): the Options page's
//! Volume row, saved as it turns (`world::settings`), over the first-run
//! level (`sound::synth::master_at`). Until then O only turned the sound
//! on or off at one level. O still mutes and unmutes, at this volume.
//!
//! A process-wide value like the UI size (`uisize`), so the options page
//! is built with it without another argument; the tests and the pictures
//! never load it, so they play at 100%.

use std::sync::atomic::{AtomicU32, Ordering};

use slime_mould::sound::synth::master_at;
use slime_mould::sound::{GrainKind, SoundCmd};

use super::*;

static VOLUME: AtomicU32 = AtomicU32::new(0x3f80_0000); // 1.0

/// The player's volume (1 = 100%).
pub(super) fn volume_user() -> f32 {
    f32::from_bits(VOLUME.load(Ordering::Relaxed))
}

pub(super) fn set_volume_user(v: f32) {
    VOLUME.store(v.to_bits(), Ordering::Relaxed);
}

/// The music's level (`sound::score`, 2026-11-22), 1 = 100%, 0 = off:
/// the Options page's Music row, saved like the volume. Under the volume.
static MUSIC: AtomicU32 = AtomicU32::new(0x3f80_0000); // 1.0

pub(super) fn music_user() -> f32 {
    f32::from_bits(MUSIC.load(Ordering::Relaxed))
}

pub(super) fn set_music_user(v: f32) {
    MUSIC.store(v.to_bits(), Ordering::Relaxed);
}

/// The screen shake's strength (2026-11-22), 1 = 100%, 0 = off: the
/// Options page's Screen shake row, saved like the volume. It scales the
/// command line's `shake=` (1 in the game), so the films' `shake=0` stays
/// still and the pictures, which never load it, shake as before.
static SHAKE: AtomicU32 = AtomicU32::new(0x3f80_0000); // 1.0

pub(super) fn shake_user() -> f32 {
    f32::from_bits(SHAKE.load(Ordering::Relaxed))
}

pub(super) fn set_shake_user(v: f32) {
    SHAKE.store(v.to_bits(), Ordering::Relaxed);
}

/// The world detail (performance thread, 2026-11-22; `settings::DETAIL_STEPS`
/// by index, 0 "Auto"): the Options page's World detail row, saved like
/// the volume, and `detail=` on the command line.
static DETAIL: AtomicU32 = AtomicU32::new(0);

pub(super) fn detail_user() -> &'static str {
    settings::DETAIL_STEPS[(DETAIL.load(Ordering::Relaxed) as usize).min(settings::DETAIL_STEPS.len() - 1)]
}

pub(super) fn set_detail_user(v: &str) {
    if let Some(i) = settings::DETAIL_STEPS.iter().position(|&d| d.eq_ignore_ascii_case(v)) {
        DETAIL.store(i as u32, Ordering::Relaxed);
    }
}

/// The master to send for the sound on or off.
pub(super) fn master_now(on: bool) -> SoundCmd {
    SoundCmd::Master(if on { master_at(volume_user()) } else { 0.0 })
}

impl App {
    /// Sets the volume to `v`: the speaker, the menus' row, the saved file,
    /// and a tick at the new level so the player hears where it is.
    pub(super) fn set_volume(&mut self, v: f32) {
        set_volume_user(v);
        let on = self.sound_on;
        if let Some(sp) = self.speaker.as_mut() {
            sp.send([master_now(on)]);
        }
        let label = settings::percent(v);
        for m in [self.menu.as_mut(), self.parked.as_mut()].into_iter().flatten() {
            if let Some(o) = m.options.iter_mut().find(|o| o.key == "volume") {
                if let Some(at) = o.choices.iter().position(|c| *c == label) {
                    o.at = at;
                }
            }
        }
        settings::save_volume(v);
        self.ui_sound(&[SoundCmd::Grain { kind: GrainKind::Tick, pan: 0.0, gain: 0.45, delay: 0.0 }]);
    }

    /// Sets the music's level to `v`: the menus' row and the saved file.
    /// The listener sends it with the score each frame (`App::hear`).
    pub(super) fn set_music(&mut self, v: f32) {
        set_music_user(v);
        let label = settings::percent(v);
        for m in [self.menu.as_mut(), self.parked.as_mut()].into_iter().flatten() {
            if let Some(o) = m.options.iter_mut().find(|o| o.key == "music") {
                if let Some(at) = o.choices.iter().position(|c| *c == label) {
                    o.at = at;
                }
            }
        }
        settings::save_music(v);
    }

    /// Sets the screen shake to `v`: the menus' row and the saved file. The
    /// next blast shakes by it (`feel`).
    pub(super) fn set_shake(&mut self, v: f32) {
        set_shake_user(v);
        let label = settings::percent(v);
        for m in [self.menu.as_mut(), self.parked.as_mut()].into_iter().flatten() {
            if let Some(o) = m.options.iter_mut().find(|o| o.key == "shake") {
                if let Some(at) = o.choices.iter().position(|c| *c == label) {
                    o.at = at;
                }
            }
        }
        settings::save_shake(v);
    }

    /// Sets the world detail to `v`: the menus' row and the saved file.
    /// The next frame draws at it (`App::prepare_now`).
    pub(super) fn set_detail(&mut self, v: &str) {
        set_detail_user(v);
        let v = detail_user();
        for m in [self.menu.as_mut(), self.parked.as_mut()].into_iter().flatten() {
            if let Some(o) = m.options.iter_mut().find(|o| o.key == "detail") {
                if let Some(at) = o.choices.iter().position(|c| c == v) {
                    o.at = at;
                }
            }
        }
        settings::save_detail(v);
    }
}
