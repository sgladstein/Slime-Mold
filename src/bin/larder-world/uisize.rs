//! **The UI's size** (owner, 2026-11-14 08:07Z: "We need an option to
//! scale the UI/text etc, because they look too small on my screen"): the
//! player's size, from `ui=` on the command line, else the saved one
//! (`world::settings`), turned on the Options page or with Ctrl + and
//! Ctrl - anywhere, and saved as it changes. The canvas multiplies it with
//! the window's DPI scale (`UiSize::px`).
//!
//! A process-wide value rather than an `App` field so the no-window
//! pictures (`look=`, `bench=`) take `ui=` without another argument through
//! `headless`; the tests never set it, so they draw at 100%.

use std::sync::atomic::{AtomicU32, Ordering};

use super::*;

static USER: AtomicU32 = AtomicU32::new(0x3f80_0000); // 1.0

/// The player's UI size (1 = 100%).
pub(super) fn ui_user() -> f32 {
    f32::from_bits(USER.load(Ordering::Relaxed))
}

pub(super) fn set_ui_user(v: f32) {
    USER.store(v.to_bits(), Ordering::Relaxed);
}

impl App {
    /// Sizes the UI to `v`: the canvas, the menu's row, the saved file.
    pub(super) fn set_ui(&mut self, v: f32) {
        set_ui_user(v);
        if let Some(c) = self.canvas.as_mut() {
            c.ui.user = v;
        }
        let label = settings::percent(v);
        for m in [self.menu.as_mut(), self.parked.as_mut()].into_iter().flatten() {
            if let Some(o) = m.options.iter_mut().find(|o| o.key == "ui") {
                if let Some(at) = o.choices.iter().position(|c| *c == label) {
                    o.at = at;
                }
            }
        }
        settings::save_ui(v);
    }

    /// Ctrl + (or =, or the keypad's) and Ctrl - step the UI's size; true
    /// when `code` was one.
    pub(super) fn ui_key(&mut self, code: KeyCode) -> bool {
        let by = match code {
            KeyCode::Equal | KeyCode::NumpadAdd => 1,
            KeyCode::Minus | KeyCode::NumpadSubtract => -1,
            _ => return false,
        };
        self.set_ui(settings::step(ui_user(), by));
        true
    }
}
