//! **The player's settings that outlive a run**: today only the UI size
//! (owner, 2026-11-14 08:07Z: "We need an option to scale the UI/text
//! etc, because they look too small on my screen").
//!
//! The UI was already drawn in units that grow with the window's height
//! (1 at 1080 lines, `UiSize::px`), so on a tall screen it grew, but a
//! small high-DPI screen (a laptop at 150% or 200%) got the size a 1080-line
//! desktop would, which is small at arm's length. `UiSize` adds the
//! window's DPI scale as a floor and the player's own size on top; every
//! panel, the gun cards, tooltips, the start menu and the map editor read
//! the one number.
//!
//! Saved as `key=value` lines in `settings.txt` in the user's config folder
//! (`%APPDATA%\Larder World` on Windows, `$XDG_CONFIG_HOME/larder-world` or
//! `~/.config/larder-world` elsewhere; `LARDER_SETTINGS=PATH` overrides),
//! shared by `larder-world` and `larder-edit`. Lines it does not know are
//! kept.

use std::path::PathBuf;

/// The UI's size: the player's choice (`user`, 1 = 100%) over the window's
/// DPI scale (`dpi`, winit's `scale_factor`).
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct UiSize {
    pub user: f32,
    pub dpi: f32,
}

impl Default for UiSize {
    fn default() -> Self {
        UiSize { user: 1.0, dpi: 1.0 }
    }
}

/// The sizes the Options page and Ctrl +/- step through.
pub const STEPS: [f32; 10] = [0.75, 0.9, 1.0, 1.1, 1.25, 1.5, 1.75, 2.0, 2.5, 3.0];

impl UiSize {
    /// Pixels a UI unit on a `screen`-sized window.
    ///
    /// Before 11-14 it was the height's share of 1080 lines, 0.7 to 2.4, in
    /// steps of 0.05 so a window dragged through every size rasterizes a
    /// few sets of glyph sizes, not one a pixel. At 100% on a 100% screen it
    /// still is (every picture and test before this is unchanged). A DPI
    /// scale over 1 is a floor under it (a 1600-line laptop at 200% gets 2,
    /// not 1.5), and the player's size multiplies the lot, up to where the
    /// bottom panels would take a third of the window or outgrow its width
    /// (`most`; never below the height's own size).
    pub fn px(&self, screen: (u32, u32)) -> f32 {
        let (sw, sh) = (screen.0 as f32, screen.1 as f32);
        let fit = (sh / 1080.0).clamp(0.7, 2.4);
        let base = if self.dpi > 1.0 { fit.max(self.dpi) } else { fit };
        let most = (sh / 540.0).min(sw / 1000.0).max(fit);
        let s = (base * self.user).min(most).max(0.5);
        (s * 20.0).round() / 20.0
    }
}

impl UiSize {
    /// Screen pixels a UI pixel where the UI does not grow with the window
    /// (the map editor's panel, laid out in fixed pixels): the DPI scale
    /// (at least 1) times the player's size, in steps of 0.05.
    pub fn fixed(&self) -> f32 {
        ((self.dpi.max(1.0) * self.user).clamp(0.5, 4.0) * 20.0).round() / 20.0
    }
}

/// `user` as the Options page shows it ("125%").
pub fn percent(user: f32) -> String {
    format!("{}%", (user * 100.0).round() as i32)
}

/// A size from the command line or the Options page: `125%` or `1.25`.
pub fn parse_size(v: &str) -> Option<f32> {
    let n = match v.strip_suffix('%') {
        Some(p) => p.trim().parse::<f32>().ok()? / 100.0,
        None => v.trim().parse::<f32>().ok()?,
    };
    (n.is_finite() && (0.25..=4.0).contains(&n)).then_some(n)
}

/// The next step from `user` up (`by` > 0) or down, held at the ends.
pub fn step(user: f32, by: i32) -> f32 {
    let near = STEPS.iter().enumerate().min_by(|a, b| (a.1 - user).abs().total_cmp(&(b.1 - user).abs())).map_or(2, |(i, _)| i) as i32;
    STEPS[(near + by).clamp(0, STEPS.len() as i32 - 1) as usize]
}

/// Where the settings live (see the module's note); None with no home.
pub fn path() -> Option<PathBuf> {
    if let Some(p) = std::env::var_os("LARDER_SETTINGS") {
        return Some(PathBuf::from(p));
    }
    let env = |k: &str| std::env::var_os(k).filter(|v| !v.is_empty()).map(PathBuf::from);
    let dir = if cfg!(windows) {
        env("APPDATA")?.join("Larder World")
    } else {
        env("XDG_CONFIG_HOME").or_else(|| env("HOME").map(|h| h.join(".config")))?.join("larder-world")
    };
    Some(dir.join("settings.txt"))
}

/// A saved value, if the file has it.
pub fn get(key: &str) -> Option<String> {
    get_in(&path()?, key)
}

fn get_in(p: &std::path::Path, key: &str) -> Option<String> {
    let text = std::fs::read_to_string(p).ok()?;
    text.lines().rev().find_map(|l| l.split_once('=').filter(|(k, _)| k.trim() == key).map(|(_, v)| v.trim().to_string()))
}

/// Saves `key=value`, keeping the file's other lines. A failure is
/// returned for a word on stderr; the game goes on either way.
pub fn set(key: &str, value: &str) -> Result<(), String> {
    set_in(&path().ok_or("no config folder (set LARDER_SETTINGS=PATH)")?, key, value)
}

fn set_in(p: &std::path::Path, key: &str, value: &str) -> Result<(), String> {
    let old = std::fs::read_to_string(p).unwrap_or_default();
    let mut lines: Vec<String> = old.lines().filter(|l| l.split_once('=').is_none_or(|(k, _)| k.trim() != key)).map(String::from).collect();
    lines.push(format!("{key}={value}"));
    if let Some(d) = p.parent() {
        std::fs::create_dir_all(d).map_err(|e| format!("{}: {e}", d.display()))?;
    }
    std::fs::write(p, lines.join("\n") + "\n").map_err(|e| format!("{}: {e}", p.display()))
}

/// The saved UI size (`ui=` in the file), 100% when there is none.
pub fn saved_ui() -> f32 {
    get("ui").as_deref().and_then(parse_size).unwrap_or(1.0)
}

/// Saves the UI size, with a word on stderr if it could not.
pub fn save_ui(user: f32) {
    if let Err(e) = set("ui", &percent(user)) {
        eprintln!("larder-world: the UI size was not saved ({e})");
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn at_100_percent_on_a_plain_screen_the_size_is_the_old_one() {
        let old = |sh: f32| ((sh / 1080.0).clamp(0.7, 2.4) * 20.0).round() / 20.0;
        for screen in [(640, 400), (1280, 800), (1600, 900), (1920, 1080), (2560, 1440), (3840, 2160), (1920, 1123)] {
            assert_eq!(UiSize::default().px(screen), old(screen.1 as f32), "{screen:?}");
        }
    }

    #[test]
    fn dpi_is_a_floor_and_the_players_size_multiplies() {
        // A 2560x1600 laptop at 200%: 2, not the height's 1.5.
        assert_eq!(UiSize { user: 1.0, dpi: 2.0 }.px((2560, 1600)), 2.0);
        // A 4K screen at 150%: the height's 2 already beats the DPI.
        assert_eq!(UiSize { user: 1.0, dpi: 1.5 }.px((3840, 2160)), 2.0);
        assert_eq!(UiSize { user: 1.5, dpi: 1.0 }.px((1920, 1080)), 1.5);
        assert_eq!(UiSize { user: 0.75, dpi: 1.0 }.px((1920, 1080)), 0.75);
        // Never past a third of the height for the bottom bar, nor the width.
        assert_eq!(UiSize { user: 3.0, dpi: 1.0 }.px((1920, 1080)), 1.9);
        assert_eq!(UiSize { user: 3.0, dpi: 1.0 }.px((1280, 800)), 1.3);
    }

    #[test]
    fn sizes_parse_print_and_step() {
        assert_eq!(parse_size("125%"), Some(1.25));
        assert_eq!(parse_size("1.5"), Some(1.5));
        assert_eq!(parse_size("big"), None);
        assert_eq!(parse_size("900%"), None);
        assert_eq!(percent(1.25), "125%");
        assert_eq!(step(1.0, 1), 1.1);
        assert_eq!(step(1.0, -1), 0.9);
        assert_eq!(step(3.0, 1), 3.0);
        assert_eq!(step(1.3, 1), 1.5);
    }

    #[test]
    fn the_file_keeps_other_lines() {
        let p = std::env::temp_dir().join(format!("larder-settings-{}/settings.txt", std::process::id()));
        set_in(&p, "other", "7").unwrap();
        set_in(&p, "ui", "90%").unwrap();
        set_in(&p, "ui", "150%").unwrap();
        assert_eq!(std::fs::read_to_string(&p).unwrap(), "other=7\nui=150%\n");
        assert_eq!(get_in(&p, "ui").as_deref().and_then(parse_size), Some(1.5));
        assert_eq!(get_in(&p, "missing"), None);
        let _ = std::fs::remove_dir_all(p.parent().unwrap());
    }
}
