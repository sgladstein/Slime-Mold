//! **The player's settings that outlive a run**: the UI size (owner,
//! 2026-11-14 08:07Z: "We need an option to scale the UI/text etc, because
//! they look too small on my screen") and the sound's volume (sound pass
//! two, 2026-11-20; until then O was the only control, on or off).
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

/// A saved value from the settings file at `p` (what this PC has seen,
/// `build::news::Memory`, reads and writes a file it is given, so its test
/// never touches the player's).
pub fn get_at(p: &std::path::Path, key: &str) -> Option<String> {
    get_in(p, key)
}

/// Saves `key=value` in the settings file at `p`, as `set`.
pub fn set_at(p: &std::path::Path, key: &str, value: &str) -> Result<(), String> {
    set_in(p, key, value)
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

/// The volumes the Options page steps through, as a share of the first-run
/// level (`sound::synth::DEFAULT_MASTER`; `master_at`). The master squares
/// its value, so 50% is -12 dB and 200% +12 dB (the master at full).
pub const VOLUME_STEPS: [f32; 9] = [0.0, 0.1, 0.25, 0.5, 0.75, 1.0, 1.25, 1.5, 2.0];

/// A volume from the Options page or the file: `75%` or `0.75`.
pub fn parse_volume(v: &str) -> Option<f32> {
    let n = match v.strip_suffix('%') {
        Some(p) => p.trim().parse::<f32>().ok()? / 100.0,
        None => v.trim().parse::<f32>().ok()?,
    };
    (n.is_finite() && (0.0..=2.0).contains(&n)).then_some(n)
}

/// The saved volume (`volume=` in the file), 100% when there is none.
pub fn saved_volume() -> f32 {
    get("volume").as_deref().and_then(parse_volume).unwrap_or(1.0)
}

/// Saves the volume, with a word on stderr if it could not.
pub fn save_volume(v: f32) {
    if let Err(e) = set("volume", &percent(v)) {
        eprintln!("larder-world: the volume was not saved ({e})");
    }
}

/// The saved music level (`music=` in the file; `sound::score`), 100%
/// when there is none. The same steps as the volume.
pub fn saved_music() -> f32 {
    get("music").as_deref().and_then(parse_volume).unwrap_or(1.0)
}

/// Saves the music level, with a word on stderr if it could not.
pub fn save_music(v: f32) {
    if let Err(e) = set("music", &percent(v)) {
        eprintln!("larder-world: the music level was not saved ({e})");
    }
}

/// The camera shake's strengths the Options page steps through (the
/// owner, 2026-11-22: "I thought I could turn off the screen shake but I
/// dont see the option"): a share of the game's shake, 0% off. Until then
/// only the command line's `shake=` turned it.
pub const SHAKE_STEPS: [f32; 5] = [0.0, 0.25, 0.5, 0.75, 1.0];

/// The saved screen shake (`shake=` in the file), 100% when there is none.
pub fn saved_shake() -> f32 {
    get("shake").as_deref().and_then(parse_volume).map_or(1.0, |v| v.min(1.0))
}

/// Saves the screen shake, with a word on stderr if it could not.
pub fn save_shake(v: f32) {
    if let Err(e) = set("shake", &percent(v)) {
        eprintln!("larder-world: the screen shake was not saved ({e})");
    }
}

/// **World detail** (performance thread, 2026-11-22; quality steps
/// 2026-11-24, the owner's "approved" for a render scale with a quality
/// slider): how many of the screen's pixels the world is drawn at, each
/// way (`View::set_percent`). The HUD, text, soldiers and their marks are
/// always drawn at full size. "Auto" picks the most detail whose picture
/// stays within `AUTO_PIXELS` (`auto_percent`): half on the owner's 4K
/// screen, which held 60 frames a second in every second of his play logs
/// after 11-22 (planets 1 to 6, about 3,700 seconds), where full detail
/// cost the GTX 1070 about 38 ms a frame for the world alone. 40% is the
/// floor: on his 3840 x 2160 at 250% that is one world pixel to each of
/// the screen's own points. In the window and the bench Auto's step is
/// only its top: it steps down toward 40% while frames run long (the
/// window's `autodetail`, 2026-11-24; planet 8's peak on the 1070).
pub const DETAIL_STEPS: [&str; 8] = ["Auto", "100%", "85%", "75%", "67%", "60%", "50%", "40%"];

/// A world detail as `DETAIL_STEPS` spells it, from the Options page, the
/// file or `detail=`: a step itself, any case (`auto`, `75%`, `75`), or
/// the names before the steps (`Full` 100%, `Half` 50%), which a
/// settings file saved on 11-22 or 11-23 holds.
pub fn parse_detail(v: &str) -> Option<&'static str> {
    let v = v.trim();
    let v = match v.to_ascii_lowercase().as_str() {
        "full" => "100%",
        "half" => "50%",
        _ => v,
    };
    DETAIL_STEPS.iter().copied().find(|d| d.eq_ignore_ascii_case(v) || d.strip_suffix('%') == Some(v))
}

/// The saved world detail (`detail=` in the file), "Auto" when none.
pub fn saved_detail() -> String {
    get("detail").as_deref().and_then(parse_detail).unwrap_or("Auto").into()
}

/// Saves the world detail, with a word on stderr if it could not.
pub fn save_detail(v: &str) {
    if let Err(e) = set("detail", v) {
        eprintln!("larder-world: the world detail was not saved ({e})");
    }
}

/// The render scale in percent each way (`View::set_percent`) for world
/// detail `detail` on a `screen`-sized window: the step's own, or for
/// "Auto" (and anything else) `auto_percent`.
pub fn detail_percent(detail: &str, screen: (u32, u32)) -> u32 {
    match parse_detail(detail).and_then(|d| d.strip_suffix('%')).and_then(|p| p.parse().ok()) {
        Some(p) => p,
        None => auto_percent(screen),
    }
}

/// "Auto"'s step: the largest of `DETAIL_STEPS` whose picture
/// (`View::reduced`) has at most `AUTO_PIXELS` pixels, else the last.
pub fn auto_percent(screen: (u32, u32)) -> u32 {
    let steps = DETAIL_STEPS.iter().filter_map(|d| d.strip_suffix('%')?.parse::<u32>().ok());
    let px = |p: u32| (screen.0 * p).div_ceil(100) as u64 * (screen.1 * p).div_ceil(100) as u64;
    let last = steps.clone().next_back().unwrap_or(100);
    steps.into_iter().find(|&p| px(p) <= AUTO_PIXELS).unwrap_or(last)
}

/// "Auto"'s budget, pixels the world is drawn at: 2.4 million, so 1920 x
/// 1200 and smaller draw at full, 2560 x 1440 at 75% (1920 x 1080) and a
/// 4K window at 50% (1920 x 1080; 60% would be 2.8 million). Before the
/// quality steps (11-22), Auto was half above 2560 x 1600 and full below;
/// half at 4K is unchanged.
pub const AUTO_PIXELS: u64 = 2_400_000;

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
    fn volumes_parse_and_print() {
        assert_eq!(parse_volume("75%"), Some(0.75));
        assert_eq!(parse_volume("0%"), Some(0.0));
        assert_eq!(parse_volume("300%"), None);
        assert_eq!(parse_volume("loud"), None);
        for v in VOLUME_STEPS {
            assert_eq!(parse_volume(&percent(v)), Some(v));
        }
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

    #[test]
    fn auto_detail_keeps_half_at_4k_and_full_on_small_screens() {
        assert_eq!(detail_percent("Auto", (3840, 2160)), 50);
        assert_eq!(detail_percent("Auto", (3840, 2004)), 50, "the owner's window, maximised");
        assert_eq!(detail_percent("Auto", (2560, 1440)), 75);
        assert_eq!(detail_percent("Auto", (1920, 1200)), 100);
        assert_eq!(detail_percent("Auto", (1280, 800)), 100);
        assert_eq!(detail_percent("Auto", (7680, 4320)), 40, "the floor");
        assert_eq!(detail_percent("100%", (3840, 2160)), 100);
        assert_eq!(detail_percent("67%", (3840, 2160)), 67);
        assert_eq!(detail_percent("Half", (1280, 800)), 50);
    }

    #[test]
    fn a_world_detail_reads_in_every_spelling_and_the_old_names() {
        for d in DETAIL_STEPS {
            assert_eq!(parse_detail(d), Some(d));
            assert_eq!(parse_detail(&d.to_lowercase()), Some(d));
        }
        assert_eq!(parse_detail("Full"), Some("100%"));
        assert_eq!(parse_detail("half"), Some("50%"));
        assert_eq!(parse_detail("75"), Some("75%"));
        assert_eq!(parse_detail("70%"), None);
        assert_eq!(parse_detail(""), None);
    }
}
