use super::*;
use crate::gpu::larder_world::race_settings;
use crate::gpu::Settings;
use crate::mould::larder_world::{shape_keys, Rules};
use crate::mould::rounds::RoundRules;
use crate::mould::run::{does_to_you, enemy_keys, Run, PLANETS};
use crate::mould::tiers::Tier;

/// The keys a setter takes, read from its source: the first name of each
/// `"name" | "alias" => ...` arm (the aliases are the CPU's older names)
/// and each `name == "..."` in the function that starts on the line
/// holding `signature`.
///
/// Why from the source: the setters are hand-written matches with no list
/// of their keys, and turning 300 arms into a table would rewrite the two
/// files every lane's keys land in. `the_scan_finds_the_setters` fails if
/// a setter moves or changes shape, so this cannot silently find nothing.
fn keys_in(source: &str, signature: &str) -> Vec<String> {
    let mut lines = source.lines().skip_while(|l| !l.contains(signature));
    let first = lines.next().unwrap_or_else(|| panic!("no line holds `{signature}`"));
    let end = format!("{}}}", &first[..first.len() - first.trim_start().len()]);
    let mut keys: Vec<String> = Vec::new();
    for l in lines.take_while(|l| *l != end) {
        let t = l.trim_start();
        let found = if let Some(rest) = t.strip_prefix('"') {
            // An arm: a name, then `=>` after it and its aliases.
            let name = rest.split('"').next().unwrap_or("");
            let after = &rest[(name.len() + 1).min(rest.len())..];
            let arm = after.split("=>").next().is_some_and(|pat| after.contains("=>") && pat.chars().all(|c| c == '"' || c == '|' || c == ' ' || c == '_' || c.is_ascii_alphanumeric()));
            arm.then_some(name)
        } else {
            t.split("name == \"").nth(1).and_then(|r| r.split('"').next())
        };
        if let Some(k) = found.filter(|k| !k.is_empty() && !keys.iter().any(|x| x == k)) {
            keys.push(k.to_string());
        }
    }
    keys
}

fn engine_keys() -> Vec<String> {
    keys_in(include_str!("../../gpu/mod.rs"), "pub fn set(s: &mut Settings, key: &str, v: &str)")
}

fn rule_keys() -> Vec<String> {
    keys_in(include_str!("../../mould/larder_world/rules.rs"), "pub fn set(&mut self, name: &str, v: f32) -> bool")
}

fn round_keys() -> Vec<String> {
    keys_in(include_str!("../../mould/rounds.rs"), "pub fn set(&mut self, name: &str, v: f32) -> bool")
}

#[test]
fn the_scan_finds_the_setters() {
    let (e, r, rr) = (engine_keys(), rule_keys(), round_keys());
    // 11-17: 173 engine keys, 81 rules, 16 round rules.
    assert!(e.len() > 150 && r.len() > 60 && rr.len() > 10, "{} engine keys, {} rules, {} round rules: has a setter changed shape?", e.len(), r.len(), rr.len());
    for k in ["crowd", "far_smell", "crystal_regrow", "plants"] {
        assert!(e.iter().any(|x| x == k), "engine key {k} not found");
    }
    for k in ["last_stand", "unit_paths", "raid_every", "set_block"] {
        assert!(r.iter().any(|x| x == k), "rule {k} not found");
    }
    assert!(rr.iter().any(|x| x == "mind_r"));
    // Every key found is one the setter takes: an arm of some inner match
    // (a value, not a key) would show here.
    let mut s = race_settings(1, 2, 5000);
    for k in &e {
        let took = ["0", "1", "amber", "trail", "game", "spread", "drifter"].iter().any(|v| crate::gpu::set(&mut s, k, v).is_ok());
        assert!(took, "`{k}` was read as an engine key but gpu::set takes no value for it");
    }
    for k in &r {
        assert!(Rules::default().set(k, 1.0), "`{k}` was read as a rule but Rules::set does not take it");
    }
    for k in &rr {
        assert!(RoundRules::default().set(k, 1.0), "`{k}` was read as a round rule but RoundRules::set does not take it");
    }
}

/// The default game's engine settings: the sandbox (shape C on the race
/// world) and every tier on every planet of a run with the planet's keys
/// and what the tier does to you, as the window builds them.
fn default_engines() -> Vec<Settings> {
    let (_, c) = shape_keys("c").expect("shape C");
    let mut base = race_settings(1, 2, 5000);
    for (k, v) in c {
        crate::gpu::set(&mut base, k, v).unwrap();
    }
    let mut out = vec![base.clone()];
    for depth in 1..=PLANETS {
        for tier in Tier::ALL {
            let mut s = base.clone();
            for (k, v) in enemy_keys(depth) {
                crate::gpu::set(&mut s, k, &v).unwrap();
            }
            tier.settings(&mut s).unwrap();
            for &(k, v) in does_to_you(tier).keys {
                crate::gpu::set(&mut s, k, v).unwrap();
            }
            out.push(s);
        }
    }
    out
}

/// The default game's rules: the sandbox's and each planet's of a run.
fn default_rules() -> Vec<Rules> {
    let mut out = vec![Rules::default()];
    for seed in 1..=4 {
        for depth in 1..=PLANETS {
            let mut run = Run::new(seed);
            run.depth = depth;
            let mut r = Rules::default();
            run.rules(&mut r);
            out.push(r);
        }
    }
    out
}

/// The keys the default game leaves off: setting one to 0 changes nothing
/// in any default game. (A key that takes no 0, such as a colour's name,
/// is not a number or a switch and is passed over.)
fn off_keys() -> (Vec<String>, Vec<String>) {
    let engines = default_engines();
    let engine = engine_keys().into_iter().filter(|k| {
        let mut took = false;
        let same = engines.iter().all(|s| {
            let mut t = s.clone();
            match crate::gpu::set(&mut t, k, "0") {
                Ok(()) => {
                    took = true;
                    t == *s
                }
                Err(_) => true,
            }
        });
        took && same
    });
    let rules = default_rules();
    let rounds = RoundRules::default();
    let rule = rule_keys().into_iter().chain(round_keys()).filter(|k| {
        rules.iter().all(|r| {
            let mut t = *r;
            t.set(k, 0.0);
            format!("{t:?}") == format!("{r:?}")
        }) && {
            let mut t = rounds;
            t.set(k, 0.0);
            format!("{t:?}") == format!("{rounds:?}")
        }
    });
    (engine.collect(), rule.collect())
}

/// Engine keys a declared switch turns on through another key, as (key,
/// the switch it is part of). None today: on 11-17 `crust=1` set the rise,
/// the fuel and the barren char and `setting=1` the scar a walker sets at
/// and wakes under, and both rows are now the Trunk's trait, so the
/// default game sets their parts on planet 3.
const PARTS: &[(&str, &str)] = &[];

/// **Keys the default game leaves at 0 that are not switches**, as of
/// 2026-11-17, each with why. Three kinds:
///
/// - `DISH`: a rule of the model from the dish games (Jones's and
///   Jenson's looks, species, grazing, the germ), which the 11-17 plan
///   froze. The squad game's slime (shape C) never sets them; they are not
///   features waiting to be turned on.
/// - `CAST`: set by the game itself when a round or a tool is used (the
///   first gas, a Mind burst), not by a key.
/// - `FILM`: a number a film forces for a picture; 0 means "as the game
///   draws it".
///
/// A new key does not belong here unless it is one of these: a feature
/// that is built and off is a switch, and goes in `SWITCHES`.
const DISH: &str = "a rule of the paused dish games' model, which shape C never sets";
const CAST: &str = "set by the game when a round or tool is cast, not by a key";
const FILM: &str = "a film's override; 0 leaves the game's own draw";
const CAMPAIGN: &str = "a rule the frozen campaign's missions set themselves; the run never uses it";
const NOT_SWITCHES: &[(&str, &str)] = &[
    ("graze", DISH),
    ("respawn", DISH),
    ("haze", DISH),
    ("trail_scale", DISH),
    ("reach_by", DISH),
    ("angle_by", DISH),
    ("turn_by", DISH),
    ("step_by", DISH),
    ("cap", DISH),
    ("floor", DISH),
    ("wander", DISH),
    ("sick", DISH),
    ("immune", DISH),
    ("species", DISH),
    ("zones", DISH),
    ("cycle", DISH),
    ("blocked_keep", DISH),
    ("chance", DISH),
    ("size_spread", DISH),
    ("sensor_r", DISH),
    ("convert", DISH),
    ("share", DISH),
    ("convert_r", DISH),
    ("mix_share", DISH),
    ("gas_death", CAST),
    ("gas_keep", CAST),
    ("spin_curl", CAST),
    // The crater's cut for the look A/B of a shot's hole (11-15).
    ("scorch_keep", FILM),
    // `set=boss_kind=N` forces a mini-boss's kind for its film; 0 draws
    // one by the kinds' weights, which is the game.
    ("set:boss_kind", FILM),
    // The free recruit (`Rules::reserve`): a row until 2026-11-18. The
    // campaign's missions set it (`campaign::RESERVE`, the owner's
    // no-soft-lock pick of 11-12); squad mode has no larder to send one
    // from, and his pick for the game was free recruits off.
    ("set:reserve", CAMPAIGN),
];

/// **The test the plan asks for** (job 1: "A test fails on a switch that
/// is not declared"). A key that the default game leaves off, on every
/// planet and tier, is a feature nobody plays unless they type it. Each
/// one must be a declared switch's key, a part of one, or on the short
/// list above with its reason.
#[test]
fn no_switch_is_undeclared() {
    let (engine, rules) = off_keys();
    let declared: Vec<&str> = SWITCHES.iter().flat_map(|s| s.choices.iter()).flat_map(|c| c.keys.iter().map(|(k, _)| *k)).collect();
    let known = |k: &str| declared.contains(&k) || PARTS.iter().any(|(p, _)| *p == k) || NOT_SWITCHES.iter().any(|(n, _)| *n == k);
    let missing: Vec<String> = engine.iter().cloned().chain(rules.iter().map(|k| format!("{RULE_KEY}{k}"))).filter(|k| !known(k)).collect();
    assert!(
        missing.is_empty(),
        "\n\nThe default game leaves these keys off (0 on every planet and tier), and no switch declares them:\n\n    {}\n\n\
         A feature that is built but off must be declared: add a row to SWITCHES in src/build/switches.rs \
         (its name, choices, lane, date, what the player will see and why it is off), so the game's Experiments tab lists it.\n\
         Better: turn it on. Put its value in the shape's keys, the planet's (run::enemy_keys) or a tier's, and this test passes with no row.\n\
         Only if it is not a feature at all (see NOT_SWITCHES in src/build/switches/tests.rs) does it go on that list, with its reason.\n",
        missing.join(", ")
    );
    // A part names a switch that is there.
    for (k, s) in PARTS {
        assert!(find(s).is_some(), "PARTS: {k} is part of {s}, which is not a switch (delete the line with the switch)");
    }
}

#[test]
fn the_list_is_sound() {
    let base = default_engines().remove(0);
    for (i, s) in SWITCHES.iter().enumerate() {
        let what = s.key;
        assert!(SWITCHES[..i].iter().all(|o| o.key != s.key && o.label != s.label), "{what}: declared twice");
        assert!(!s.key.is_empty() && s.key.chars().all(|c| c == '_' || c.is_ascii_lowercase() || c.is_ascii_digit()), "{what}: a key is lower case with _");
        assert!(!s.label.is_empty() && s.label.len() <= 28, "{what}: the label is two or three words");
        assert!(s.sees.len() > 20 && !s.sees.ends_with('.'), "{what}: say what the player will see, one line, no full stop");
        assert!(s.why_off.len() > 20, "{what}: say why it is not the default (a measured harm, or the either-or)");
        let d: Vec<&str> = s.since.split('-').collect();
        assert!(d.len() == 3 && s.since.len() == 10 && d.iter().all(|x| x.parse::<u32>().is_ok()), "{what}: since is YYYY-MM-DD");
        let default = s.choice(s.default).unwrap_or_else(|| panic!("{what}: its default `{}` is not one of its choices", s.default));
        assert!(default.keys.is_empty(), "{what}: the default choice sets nothing (the default game is the engine's and the rules' own values)");
        assert!(s.choices.len() >= 2, "{what}: a switch has a choice that is not its default");
        for (j, c) in s.choices.iter().enumerate() {
            assert!(s.choices[..j].iter().all(|o| o.name != c.name) && c.name != "custom" && !c.name.contains([':', ',', '_', '-']), "{what}: choice `{}`", c.name);
            if c.name == s.default {
                continue;
            }
            // Every key is one its setter takes, and the choice changes
            // the default game (window keys are the window's to check).
            let (mut e, mut r, mut rr) = (base.clone(), Rules::default(), RoundRules::default());
            for (k, v) in c.keys {
                set_key(&mut e, &mut r, &mut rr, k, v).unwrap_or_else(|err| panic!("{what}, {}: {err}", c.name));
            }
            let moved = e != base || format!("{r:?}") != format!("{:?}", Rules::default()) || format!("{rr:?}") != format!("{:?}", RoundRules::default());
            assert!(moved || c.keys.iter().all(|(k, _)| k.starts_with(WINDOW_KEY)), "{what}, {}: its keys change nothing in the default game, so it is on already: delete the row", c.name);
            // The play log reads it back by name.
            let on = state(&keys_of(c));
            assert!(on.iter().any(|(o, n)| o.key == s.key && *n == c.name), "{what}, {}: read back as {:?}", c.name, on.iter().map(|(o, n)| (o.key, *n)).collect::<Vec<_>>());
            assert_eq!(parse_try(&format!("{}:{}", s.key, c.name.replace(' ', "_"))).map(|(o, x)| (o.key, x.name)), Ok((s.key, c.name)));
        }
        assert_eq!(parse_try(s.key).map(|(_, c)| c.name), Ok(s.tried().name));
    }
    assert!(parse_try("no_such").unwrap_err().contains("crystal_regrow"));
    assert!(parse_try("crystal_regrow:sideways").unwrap_err().contains("off, on"));
}

#[test]
fn the_keys_say_which_switches_are_on() {
    let keys = |v: &[(&str, &str)]| v.iter().map(|(k, v)| (k.to_string(), v.to_string())).collect::<Vec<_>>();
    assert_eq!(log_word(&[]), "none");
    // The default game's own keys are no switch: a planet's, a tier's
    // trait, and since 11-17 the far smell and the charge paths, which
    // were rows until the Enemy lane turned them on on every planet.
    assert_eq!(log_word(&keys(&[("panic", "1"), ("surge", "900"), ("set:pump_smell", "2"), ("far_smell", "1"), ("set:unit_paths", "1"), ("setting", "1"), ("set:set_block", "5")])), "none");
    // In the list's order, whatever order the keys came in.
    assert_eq!(log_word(&keys(&[("crystal_regrow", "0.001"), ("win:shake", "1"), ("win:hitstop", "1")])), "feel:on,crystal_regrow:on");
    // 1.0 is 1; a later 0 turns it off again.
    assert_eq!(log_word(&keys(&[("win:shake", "1.0"), ("win:hitstop", "1.0")])), "feel:on");
    assert_eq!(log_word(&keys(&[("crystal_regrow", "0.001"), ("crystal_regrow", "0")])), "none");
    // A choice is named when all its keys hold: shake alone is not the
    // row's "on".
    assert_eq!(log_word(&keys(&[("win:shake", "1"), ("win:hitstop", "1")])), "feel:on");
    assert_eq!(log_word(&keys(&[("win:shake", "1")])), "feel:custom");
    // A film's own number is custom, not lost.
    assert_eq!(log_word(&keys(&[("crystal_regrow", "0.01")])), "crystal_regrow:custom");
    // Everything at once reads back as every switch (the all-switches
    // test's keys). No row's choice covers another's today (11-17: "a
    // wall" covered plain setting, and the crust's pull the crust).
    let all = state(&all_on());
    for s in SWITCHES {
        assert!(all.iter().any(|(o, _)| o.key == s.key), "{}", s.key);
    }
}
