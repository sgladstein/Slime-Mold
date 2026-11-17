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
//! command line presets one, as `play-new-slime.cmd` does.

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

/// The value of a window key (`win:NAME`, `switches::WINDOW_KEY`) among a
/// round's keys: the last given.
pub(super) fn window_key<'a>(keys: &'a [(String, String)], name: &str) -> Option<&'a str> {
    debug_assert!(WINDOW_KEYS.contains(&name), "{name} is not a window key a switch may set");
    keys.iter().rev().find(|(k, _)| k.strip_prefix(switches::WINDOW_KEY) == Some(name)).map(|(_, v)| v.as_str())
}

/// The window keys a switch may set (`the_switches_window_keys_are_read`
/// holds the list to these): `setlook`, set slime's look
/// (`View::set_setting`: 1 amber, 2 glaze, 0 hidden).
pub(super) const WINDOW_KEYS: [&str; 1] = ["setlook"];

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
                    assert!(v.parse::<u32>().is_ok(), "switch {}: {k}={v}", s.key);
                }
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
        turn(&mut menu, "far_smell", "on");
        turn(&mut menu, "set_slime", "a trap");
        let a = round_args(&bare, false, &menu, &StartEvent::Sandbox);
        assert_eq!(switches::log_word(&a.keys), "set_slime:a_trap,far_smell:on");
        assert!(a.keys.contains(&("far_smell".into(), "1".into())) && a.keys.contains(&("set:set_hold".into(), "0.5".into())));
        let run = run_args(&bare, &menu, &Run::new(1));
        assert_eq!(switches::log_word(&run.keys), "set_slime:a_trap,far_smell:on", "a run's planets play the picks");
        // The command line's `try=crust try=unit_paths` presets the rows;
        // turning one off in the menu takes its keys off the round.
        let mut cli = bare.clone();
        for t in ["crust", "unit_paths"] {
            cli.keys.extend(switches::keys_of(switches::parse_try(t).unwrap().1));
        }
        let mut menu = start_menu(&cli);
        assert_eq!(menu.option("exp:crust").map(|o| o.value()), Some("on"));
        assert_eq!(switches::log_word(&round_args(&cli, false, &menu, &StartEvent::Sandbox).keys), "crust:on,unit_paths:on");
        turn(&mut menu, "crust", "off");
        let a = round_args(&cli, false, &menu, &StartEvent::Sandbox);
        assert_eq!(switches::log_word(&a.keys), "unit_paths:on");
        assert!(!a.keys.iter().any(|(k, _)| k == "crust"));
        // A film's own number shows as custom and is left alone.
        let mut film = bare.clone();
        film.keys.push(("crust_weight".into(), "2".into()));
        let menu = start_menu(&film);
        assert_eq!(menu.option("exp:crust_ground").map(|o| o.value()), Some("custom"));
        assert!(round_args(&film, false, &menu, &StartEvent::Sandbox).keys.contains(&("crust_weight".into(), "2".into())));
    }

    /// The play log's header names the build and the switches.
    #[test]
    fn the_logs_header_names_the_build_and_the_switches() {
        let mut keys = vec![("panic".to_string(), "1".to_string())];
        keys.extend(switches::keys_of(switches::parse_try("set_slime:a_wall").unwrap().1));
        let extra = log_keys(&keys);
        let head = slime_mould::mould::playlog::Playlog::head("planet:1:7", 7, &[("build", slime_mould::build::build().word())].into_iter().chain(extra).collect::<Vec<_>>());
        let r = slime_mould::mould::playlog::read(&head);
        let get = |k: &str| r.head.iter().find(|(n, _)| n == k).map(|(_, v)| v.as_str());
        assert_eq!(get("build").and_then(slime_mould::build::parse).map(|b| b.1), Some(slime_mould::build::build().commit.to_string()));
        assert_eq!(get("switches"), Some("set_slime:a_wall"));
        assert_eq!(get("keys"), Some("panic=1,setting=1,set:set_block=3"));
    }
}
