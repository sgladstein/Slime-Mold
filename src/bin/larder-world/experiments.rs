//! **The Experiments tab's data** (the Engine and tools lane, 2026-11-17):
//! the declared switches (`slime_mould::build::switches`) as options of
//! the start menu, and the menu's picks as a round's keys. The Look and
//! interface lane draws the tab (`menu.rs`, `start.rs`); this file only
//! says what is on it and what a pick sets.
//!
//! A pick lasts until the game closes, as every option but the UI size
//! does: each launch starts on the default game, so what is on is never a
//! leftover from last week (owner, 11-17: features left switched off are a
//! problem because he can't tell what is on or off). `try=NAME` on the
//! command line presets one (`play-new-slime.cmd` did, until the two rows
//! it turned on became every planet's on 11-17 and the script went).

use super::*;
use slime_mould::build::switches::{self, Switch, SWITCHES};
use slime_mould::world::start::{Opt, StartMenu};

/// What a round's keys make of switch `s`: the name of its choice, its
/// default's when nothing of it is set, or "custom" for a film's own
/// numbers.
fn now(keys: &[(String, String)], s: &Switch) -> &'static str {
    switches::state(keys).into_iter().find(|(o, _)| o.key == s.key).map_or(s.default, |(_, c)| c)
}

/// One option a switch, preset from the command line's keys.
pub(super) fn options(base: &Args) -> Vec<Opt> {
    SWITCHES.iter().map(|s| Opt::new(&s.opt_key(), s.label, &s.names(), s.default, now(&base.keys, s), &format!("{}. Not on yet: {}", s.sees, s.why_off)).in_tab("Experiments")).collect()
}

/// The menu's picks over the command line's keys in `a`. A switch the
/// menu moved loses the keys the command line gave it; then every switch
/// off its default sets its choice's keys (in the list's order, so where
/// two rows share a key the later row's value stands, as `try=` does).
pub(super) fn apply(a: &mut Args, base: &Args, menu: &StartMenu) {
    let pick = |s: &Switch| menu.option(&s.opt_key()).map(|o| o.value().to_string());
    let moved: Vec<&Switch> = SWITCHES.iter().filter(|s| pick(s).is_some_and(|v| v != now(&base.keys, s))).collect();
    a.keys.retain(|(k, _)| !moved.iter().any(|s| s.choices.iter().any(|c| c.keys.iter().any(|(n, _)| n == k))));
    for s in SWITCHES {
        if let Some(c) = pick(s).and_then(|v| s.choice(&v)) {
            // A switch the menu left alone keeps the command line's keys.
            if moved.iter().any(|m| m.key == s.key) || c.name != s.default {
                a.keys.extend(switches::keys_of(c));
            }
        }
    }
}

/// The window keys a switch may set (`the_switches_window_keys_are_read`
/// holds the list to these), as the command line's own keys of the same
/// names: `shake` and `hitstop`, the camera's shake and the freeze on a
/// hit (off at 0, the Squad lane's toggle). (`setlook`, the set slime's
/// look, was one to 2026-11-18: amber is what the game draws.)
pub(super) const WINDOW_KEYS: [&str; 2] = ["shake", "hitstop"];

/// Sets window key `name` (a switch's `win:NAME`) on a round's args
/// (`App::start`, before the round is built).
pub(super) fn set_window_key(a: &mut Args, name: &str, v: &str) -> Result<(), String> {
    let bad = |e: &dyn std::fmt::Display| format!("{}{name}={v}: {e}", switches::WINDOW_KEY);
    match name {
        "shake" => a.shake = v.parse().map_err(|e| bad(&e))?,
        "hitstop" => a.hitstop = v.parse().map_err(|e| bad(&e))?,
        _ => return Err(format!("{}{name}: the window reads no such key (it reads {})", switches::WINDOW_KEY, WINDOW_KEYS.join(", "))),
    }
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;
    use slime_mould::build::switches::WINDOW_KEY;
    use slime_mould::world::start::StartEvent;

    #[test]
    fn the_switches_window_keys_are_read() {
        for s in SWITCHES {
            for (k, v) in s.choices.iter().flat_map(|c| c.keys.iter()) {
                if let Some(name) = k.strip_prefix(WINDOW_KEY) {
                    assert!(WINDOW_KEYS.contains(&name), "switch {}: the window reads no key {k}", s.key);
                    set_window_key(&mut Args::default(), name, v).unwrap_or_else(|e| panic!("switch {}: {e}", s.key));
                }
            }
        }
        for k in WINDOW_KEYS {
            assert!(set_window_key(&mut Args::default(), k, "1").is_ok(), "{k}");
        }
        assert!(set_window_key(&mut Args::default(), "nope", "1").is_err());
    }

    /// The window's own command-line keys on 2026-11-17 that are not
    /// switches: what sets a round up (map, seed, run, mode, gun), the
    /// films' and the bench's tools (look, at, hand, boom, turret, bench),
    /// and numbers a film forces for a picture (crater, wade, spray, aim).
    const TOOLS: &str = "map seed scale walkers mission run range dev grow mode crater scent follow boss_look core_pull bullet own mass shape wade real sink spray aim gun ammo look ui menu hand rounds sound voice at bench fps stamps grid fast passes viewpasses settle squad1 spread holes select drag scroll card hover bank set centre cut turret turret2 mortar lure pump outpost siren wall boss load try crustlook watch tload boom light lure_mark trail cross fencepick necks cutsquad screen";

    /// **No switch hides as a window key** (the plan's rule 3: "A switch
    /// that exists only as a command-line key is not allowed"). The
    /// engine's and the rules' keys are held by the library's
    /// `no_switch_is_undeclared`; this holds the window's own: every key
    /// `main` reads is a tool on the list above or a declared switch's
    /// window key (`WINDOW_KEYS`), so a new one is a written choice.
    #[test]
    fn no_window_key_is_an_undeclared_switch() {
        let src = include_str!("main.rs");
        let from = src.find("    for a in std::env::args().skip(1) {").expect("main's argument loop");
        let to = src[from..].find("// Any other key is an engine setting").expect("the loop's last arm") + from;
        let mut keys: Vec<&str> = Vec::new();
        for l in src[from..to].lines() {
            // The loop's own arms sit at twelve spaces: `"a" | "b" => ...`.
            let Some(arm) = l.strip_prefix("            \"").filter(|_| l.contains("=>")) else { continue };
            keys.extend(arm.split("=>").next().unwrap_or("").split('"').step_by(2).filter(|k| !k.is_empty()));
        }
        assert!(keys.len() >= 60 && keys.contains(&"map") && keys.contains(&"try"), "{} keys read from main: has its loop changed shape?", keys.len());
        // The Engine and tools lane's own keys (`version`: print the build
        // and quit, for CI), kept apart from TOOLS so that lanes editing
        // that line and this lane do not conflict.
        const ENGINE_TOOLS: &str = "version";
        let tools: Vec<&str> = TOOLS.split(' ').chain(ENGINE_TOOLS.split(' ')).collect();
        let declared: Vec<&str> = SWITCHES.iter().flat_map(|s| s.choices.iter()).flat_map(|c| c.keys.iter()).filter_map(|(k, _)| k.strip_prefix(WINDOW_KEY)).collect();
        for k in keys {
            if WINDOW_KEYS.contains(&k) {
                assert!(declared.contains(&k), "window key {k} is a switch's (WINDOW_KEYS) but no row of SWITCHES sets win:{k}: declare its row in src/build/switches.rs, or move it to TOOLS if it is a film's tool");
            } else {
                assert!(tools.contains(&k), "\n\nThe window reads a new key, `{k}`. If it turns on something the default game does not play, it is a switch: add it to WINDOW_KEYS and set_window_key here, and declare its row in src/build/switches.rs with the key `win:{k}`, so the Experiments tab lists it. If it only sets a round up or serves a film or the bench, add it to TOOLS in this test.\n");
            }
        }
    }

    /// The menu's Experiments tab: one row a switch at its default, a pick
    /// sets its keys on the round (a run's too, which is `round_args`
    /// under `run_args`), a pick moved back takes the command line's keys
    /// off again, and `try=` presets a row.
    #[test]
    fn the_experiments_tab_sets_a_rounds_keys() {
        let bare = Args { map: Some("fork".into()), seed: 1, scale: 2, walkers: 5000, squads: true, ..Default::default() };
        let mut menu = start_menu(&bare);
        let turn = |menu: &mut StartMenu, key: &str, to: &str| {
            let o = menu.options.iter_mut().find(|o| o.key == format!("{}{key}", switches::OPT_KEY)).unwrap_or_else(|| panic!("no row for {key}"));
            o.at = o.choices.iter().position(|c| c == to).unwrap_or_else(|| panic!("{key} has no choice {to}"));
        };
        for s in SWITCHES {
            let o = menu.option(&s.opt_key()).unwrap_or_else(|| panic!("no row for {}", s.key));
            assert!(o.is_default() && o.tab == "Experiments" && o.value() == s.default, "{}: {o:?}", s.key);
        }
        let plain = round_args(&bare, false, &menu, &StartEvent::Sandbox);
        assert_eq!(switches::log_word(&plain.keys), "none");
        // The rows this plays are the two left since the Enemy lane
        // settled the 11-17 list (the far smell, the trap and the crust,
        // which this test turned until then, are no longer rows) and the
        // free recruits left it (11-18).
        turn(&mut menu, "crystal_regrow", "on");
        turn(&mut menu, "feel", "on");
        let a = round_args(&bare, false, &menu, &StartEvent::Sandbox);
        assert_eq!(switches::log_word(&a.keys), "feel:on,crystal_regrow:on");
        assert!(a.keys.contains(&("crystal_regrow".into(), "0.001".into())) && a.keys.contains(&("win:hitstop".into(), "1".into())));
        let run = run_args(&bare, &menu, &Run::new(1));
        assert_eq!(switches::log_word(&run.keys), "feel:on,crystal_regrow:on", "a run's planets play the picks");
        // The command line's `try=feel try=crystal_regrow` presets the
        // rows; turning one off in the menu takes its keys off the round.
        let mut cli = bare.clone();
        for t in ["feel", "crystal_regrow"] {
            cli.keys.extend(switches::keys_of(switches::parse_try(t).unwrap().1));
        }
        let mut menu = start_menu(&cli);
        assert_eq!(menu.option("exp:feel").map(|o| o.value()), Some("on"));
        assert_eq!(switches::log_word(&round_args(&cli, false, &menu, &StartEvent::Sandbox).keys), "feel:on,crystal_regrow:on");
        turn(&mut menu, "feel", "off");
        let a = round_args(&cli, false, &menu, &StartEvent::Sandbox);
        assert_eq!(switches::log_word(&a.keys), "crystal_regrow:on");
        assert!(!a.keys.iter().any(|(k, _)| k == "win:shake" || k == "win:hitstop"));
        // A film's own number shows as custom and is left alone.
        let mut film = bare.clone();
        film.keys.push(("crystal_regrow".into(), "0.01".into()));
        let menu = start_menu(&film);
        assert_eq!(menu.option("exp:crystal_regrow").map(|o| o.value()), Some("custom"));
        assert!(round_args(&film, false, &menu, &StartEvent::Sandbox).keys.contains(&("crystal_regrow".into(), "0.01".into())));
    }

    /// The play log's header names the build and the switches.
    #[test]
    fn the_logs_header_names_the_build_and_the_switches() {
        let mut keys = vec![("panic".to_string(), "1".to_string())];
        keys.extend(switches::keys_of(switches::parse_try("feel:on").unwrap().1));
        let extra = log_keys(&keys);
        let head = slime_mould::mould::playlog::Playlog::head("planet:1:7", 7, &[("build", slime_mould::build::build().word())].into_iter().chain(extra).collect::<Vec<_>>());
        let r = slime_mould::mould::playlog::read(&head);
        let get = |k: &str| r.head.iter().find(|(n, _)| n == k).map(|(_, v)| v.as_str());
        assert_eq!(get("build").and_then(slime_mould::build::parse).map(|b| b.1), Some(slime_mould::build::build().commit.to_string()));
        assert_eq!(get("switches"), Some("feel:on"));
        assert_eq!(get("keys"), Some("panic=1,win:shake=1,win:hitstop=1"));
    }
}
