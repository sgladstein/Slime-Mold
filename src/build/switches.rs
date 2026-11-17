//! **The switch list**: every feature that is built but is not what the
//! default game plays, declared once (the plan of 2026-11-17, rule 3:
//! "Everything lands on by default ... A switch that exists only as a
//! command-line key is not allowed: it must be declared in the switch
//! list", which the game's Experiments tab shows).
//!
//! Why a list in code: on 11-17 the owner said features left switched off
//! are a problem, because he cannot tell what is on or off or what needs
//! testing. Before this the list was a table in ROADMAP ("Switches that
//! are off") kept by hand, and the keys were scattered over `gpu::set`,
//! `Rules::set` and the window's arguments. Now:
//!
//! - [`SWITCHES`] is the one list. Each row has a name, its choices (each
//!   a set of the game's own keys), its default, the lane that owns it,
//!   the date it landed and one line on what the player will see.
//! - The window takes `try=NAME` or `try=NAME:CHOICE` and the menu's
//!   Experiments tab turns the same rows; the play log's header says which
//!   were off their default ([`state`], [`log_word`]).
//! - `no_switch_is_undeclared` (below) fails when the engine or the rules
//!   gain a key that the default game leaves off and no row declares. So a
//!   new off feature cannot land without a row here.
//! - `mouldgpu`'s all-switches test plays every row's first non-default
//!   choice together ([`all_on`]), so a new row is played without editing
//!   that test.
//!
//! The rule for a row (plan, rule 3): off is allowed only for a measured
//! harm or for one side of a real either-or, at most [`MOST_OFF`] at once
//! across all lanes, and each ends at the owner's next play: on, one
//! tier's trait, or deleted with its `Reports/dead-ends.md` line. The
//! rows of 11-17 are over that count: they are ROADMAP's table as it
//! stood, which the Enemy lane is settling; `MOST_OFF` is checked by
//! `the_list_is_sound` once they are down to it (`SETTLING`).

/// The lanes of the 11-17 plan, as a row's owner.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Lane {
    Direction,
    Enemy,
    Squad,
    Look,
    Engine,
    Lab,
}

impl Lane {
    pub fn name(self) -> &'static str {
        match self {
            Lane::Direction => "Direction",
            Lane::Enemy => "Enemy",
            Lane::Squad => "Squad and arsenal",
            Lane::Look => "Look and interface",
            Lane::Engine => "Engine and tools",
            Lane::Lab => "Lab",
        }
    }

    /// The lane a "what is new" line names (`news`): its first word,
    /// lower case.
    pub fn from_word(w: &str) -> Option<Lane> {
        Some(match w.trim().to_ascii_lowercase().as_str() {
            "direction" => Lane::Direction,
            "enemy" => Lane::Enemy,
            "squad" => Lane::Squad,
            "look" => Lane::Look,
            "engine" => Lane::Engine,
            "lab" => Lane::Lab,
            _ => return None,
        })
    }
}

/// The prefix of a rule's key among the engine keys (the window's
/// `Args::keys`): `set=unit_paths=1` on the command line is kept as
/// `("set:unit_paths", "1")`, set through `Rules::set` or `RoundRules::set`.
pub const RULE_KEY: &str = "set:";

/// The prefix of a key only the window reads (`win:setlook`: the
/// command line's `setlook=`). The window sets it (`larder-world`'s
/// `set_window_key`); the library's worlds ignore it.
pub const WINDOW_KEY: &str = "win:";

/// One value a switch can take: its name on the Experiments tab and after
/// `try=NAME:`, and the keys it sets. A plain key is an engine setting
/// (`gpu::set`), `set:NAME` a rule, `win:NAME` a window key. The default
/// choice sets nothing.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct Choice {
    pub name: &'static str,
    pub keys: &'static [(&'static str, &'static str)],
}

/// A declared switch.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct Switch {
    /// Its one name: `try=KEY` on the command line, in the play log and as
    /// the Experiments tab's option key (`exp:KEY`, `OPT_KEY`).
    pub key: &'static str,
    /// Two or three words for its row.
    pub label: &'static str,
    /// Its values; the first that is not `default` is what `try=KEY` and
    /// the all-switches test turn on.
    pub choices: &'static [Choice],
    /// The default game's choice, by name.
    pub default: &'static str,
    pub lane: Lane,
    /// The day it landed, YYYY-MM-DD.
    pub since: &'static str,
    /// One line: what the player will see with it on.
    pub sees: &'static str,
    /// Why it is not the default: the measured harm, or the either-or it
    /// is one side of.
    pub why_off: &'static str,
}

const OFF: Choice = Choice { name: "off", keys: &[] };

const fn on(keys: &'static [(&'static str, &'static str)]) -> Choice {
    Choice { name: "on", keys }
}

/// The most switches that may be off their built form at once, across all
/// lanes (plan, rule 3).
pub const MOST_OFF: usize = 3;

/// True while the 11-17 rows (ROADMAP's table, carried over) are being
/// settled by the Enemy lane and the list is over `MOST_OFF`. Set it false
/// in the change that brings the list down to `MOST_OFF`, and the count
/// becomes a test failure from then on.
pub const SETTLING: bool = true;

/// **The list.** The rows of 2026-11-17 are ROADMAP's "Switches that are
/// off" table as it stood that day (14 rows; the two crust rows and the
/// two set-slime rows are each one either-or here). Their reports are
/// linked from that table and from `wiki/world/squad-toolkit.md`.
///
/// To add one: a row here, in the same change as the key. To settle one:
/// make its choice the engine's or the rules' own default (or a tier's
/// trait in `tiers.rs` / `run::does_to_you`) and delete the row, or delete
/// the feature and the row and add the `Reports/dead-ends.md` line.
pub const SWITCHES: &[Switch] = &[
    Switch {
        key: "crust",
        label: "Crust",
        choices: &[OFF, on(&[("crust", "1")])],
        default: "off",
        lane: Lane::Enemy,
        since: "2026-11-17",
        sees: "Ground the slime has lived on for a minute turns to a pale crust that stays after it leaves, and fire runs across crust but stops at the fed body",
        why_off: "New on 11-17; nothing in the game reads the crust on its own yet. Decided with the crust and set-slime picks",
    },
    Switch {
        key: "crust_ground",
        label: "Crust underfoot",
        choices: &[OFF, Choice { name: "pulls back", keys: &[("crust", "1"), ("crust_weight", "8"), ("crust_fast", "0.5")] }, Choice { name: "pushes on", keys: &[("crust", "1"), ("crust_weight", "-8"), ("crust_fast", "0.5")] }],
        default: "off",
        lane: Lane::Enemy,
        since: "2026-11-17",
        sees: "Slime either keeps to its old crusted ground and moves faster on it (pulls back), or is pushed off it onto fresh ground (pushes on)",
        why_off: "A real either-or: no sign is chosen (the pull stays 0 until the owner picks one)",
    },
    Switch {
        key: "setting",
        label: "Slime sets",
        choices: &[OFF, on(&[("setting", "1")])],
        default: "off",
        lane: Lane::Enemy,
        since: "2026-11-17",
        sees: "Slime on crust that is shot at sets hard: it stops moving and takes three hits to kill",
        why_off: "New on 11-17; a standing rifle squad killed 265 in the first minute against 366 without, so it makes the slime tougher. The owner's film pick",
    },
    Switch {
        key: "set_slime",
        label: "Set slime is",
        choices: &[
            Choice { name: "soft", keys: &[] },
            Choice { name: "a wall", keys: &[("setting", "1"), ("set:set_block", "3")] },
            Choice { name: "a trap", keys: &[("setting", "1"), ("set:set_hold", "0.5")] },
        ],
        default: "soft",
        lane: Lane::Enemy,
        since: "2026-11-17",
        sees: "Set slime either blocks soldiers like a wall, or lets them in and slows them like a trap",
        why_off: "A real either-or: block against trap (the trap replaces the block when both are set)",
    },
    Switch {
        key: "set_look",
        label: "Set slime's look",
        choices: &[Choice { name: "amber", keys: &[] }, Choice { name: "glaze", keys: &[("win:setlook", "2")] }, Choice { name: "hidden", keys: &[("win:setlook", "0")] }],
        default: "amber",
        lane: Lane::Look,
        since: "2026-11-17",
        sees: "How set slime is drawn when Slime sets is on: amber, a glaze, or not marked at all",
        why_off: "A look pick that only shows with Slime sets on; decided with it",
    },
    Switch {
        key: "last_stand",
        label: "Last stand",
        choices: &[OFF, on(&[("set:last_stand", "1")])],
        default: "off",
        lane: Lane::Enemy,
        since: "2026-11-15",
        sees: "Once, when the slime is down to a quarter of its biggest, all of it rushes your soldiers at once, with an alert",
        why_off: "The owner was not sure it is a good idea; a play with it on decides",
    },
    Switch {
        key: "reserve",
        label: "Free recruits",
        choices: &[OFF, on(&[("set:reserve", "5")])],
        default: "off",
        lane: Lane::Squad,
        since: "2026-11-12",
        sees: "With fewer than 2 soldiers alive and no money for another, a free soldier arrives after 5 seconds (the larder game; squad mode has no larder to send one)",
        why_off: "The owner's pick was free recruits off. Delete if nobody asks for it by the next play",
    },
    Switch {
        key: "crystal_regrow",
        label: "Crystals refill",
        choices: &[OFF, on(&[("crystal_regrow", "0.001")])],
        default: "off",
        lane: Lane::Enemy,
        since: "2026-11-11",
        sees: "A crystal that is not being eaten slowly fills again",
        why_off: "Crystals that run out suit the race at the start; to be settled by play on a world that is mostly slime",
    },
    Switch {
        key: "flow_loss",
        label: "Lossy food flow",
        choices: &[OFF, on(&[("flow_loss", "0.1")])],
        default: "off",
        lane: Lane::Enemy,
        since: "2026-11-12",
        sees: "Food fades as the body shares it, so a part cut off from every crystal starves in about half a minute instead of staying whole for minutes",
        why_off: "The one-body work stopped at shared food 0.05 without it. Delete unless a cut-the-network play wants it",
    },
    Switch {
        key: "lone_step",
        label: "Lone slime slows",
        choices: &[OFF, on(&[("lone_step", "3")])],
        default: "off",
        lane: Lane::Enemy,
        since: "2026-11-12",
        sees: "Slime with few others round it moves slower, so hunks hold together instead of scattering",
        why_off: "It held hunks together on the designed maps but was never made a default. Decided with Lossy food flow",
    },
    Switch {
        key: "unit_paths",
        label: "Charges go round walls",
        choices: &[OFF, on(&[("set:unit_paths", "1")])],
        default: "off",
        lane: Lane::Enemy,
        since: "2026-11-17",
        sees: "Surging and panicking slime walks round walls to reach your soldiers instead of piling up against them",
        why_off: "New on 11-17; behind a cup of walls half as many piled at the wall and as many reached the soldier, so it changes the charge's shape. On unless the owner objects in play",
    },
    Switch {
        key: "far_smell",
        label: "A far smell",
        choices: &[OFF, on(&[("far_smell", "1")])],
        default: "off",
        lane: Lane::Enemy,
        since: "2026-11-17",
        sees: "Hungry slime heads for untouched crystals far away, so it spreads over the planet sooner",
        why_off: "New on 11-17; on planet 3 with no squad the slime reached 59 crystals against 47 in 2:30 and ended up to 45% bigger, so the planet is harder. On unless the owner objects in play",
    },
    Switch {
        key: "fire_fed",
        label: "Fed slime fireproof",
        choices: &[OFF, on(&[("fire_fed", "1")])],
        default: "off",
        lane: Lane::Enemy,
        since: "2026-11-17",
        sees: "Fire stops at slime that is fed from a crystal, however thin its trail; only cut-off slime burns",
        why_off: "Without it fire through a fed disc's thin trail took 24 to 65% of it in 20 s. The owner's pick on the blind page",
    },
];

/// A switch by its key.
pub fn find(key: &str) -> Option<&'static Switch> {
    SWITCHES.iter().find(|s| s.key == key)
}

impl Switch {
    pub fn choice(&self, name: &str) -> Option<&'static Choice> {
        self.choices.iter().find(|c| c.name == name)
    }

    /// The names of its choices, for a menu's row.
    pub fn names(&self) -> Vec<&'static str> {
        self.choices.iter().map(|c| c.name).collect()
    }

    /// The choice `try=KEY` turns on: the first that is not the default.
    pub fn tried(&self) -> &'static Choice {
        self.choices.iter().find(|c| c.name != self.default).expect("a switch has a choice that is not its default (the_list_is_sound)")
    }

    /// Its key as an option on the menu's Experiments tab (`exp:crust`),
    /// clear of the engine keys the other tabs use.
    pub fn opt_key(&self) -> String {
        format!("{OPT_KEY}{}", self.key)
    }
}

/// The prefix of a switch's option key on the menu's Experiments tab.
pub const OPT_KEY: &str = "exp:";

/// `try=` read: `NAME` (its first choice off the default) or
/// `NAME:CHOICE`. The error names what there is.
pub fn parse_try(v: &str) -> Result<(&'static Switch, &'static Choice), String> {
    let (key, choice) = v.split_once(':').map_or((v, None), |(k, c)| (k, Some(c)));
    let s = find(key).ok_or_else(|| format!("try={v}: no such switch (one of {})", SWITCHES.iter().map(|s| s.key).collect::<Vec<_>>().join(", ")))?;
    let c = match choice {
        None => s.tried(),
        // The tab's names have spaces; the command line may write them
        // with `_` or `-`.
        Some(n) => s.choices.iter().find(|c| c.name == n || c.name.replace(' ', "_") == n || c.name.replace(' ', "-") == n).ok_or_else(|| format!("try={v}: {} is one of {}", s.key, s.names().join(", ")))?,
    };
    Ok((s, c))
}

/// The value a key last took in `keys` (the last wins, as the game sets
/// them in order).
fn last<'a>(keys: &'a [(String, String)], k: &str) -> Option<&'a str> {
    keys.iter().rev().find(|(n, _)| n == k).map(|(_, v)| v.as_str())
}

/// Two values of a key are the same setting (`1` and `1.0`, `on` and `1`).
fn same(a: &str, b: &str) -> bool {
    let num = |v: &str| match v {
        "on" | "true" | "yes" => Some(1.0),
        "off" | "false" | "no" => Some(0.0),
        _ => v.parse::<f32>().ok(),
    };
    a == b || matches!((num(a), num(b)), (Some(x), Some(y)) if x == y)
}

/// **What a round's keys turned on**: every switch that is off its
/// default under `keys` (the window's `Args::keys`: the command line's
/// and the menu's, rules as `set:NAME`, window keys as `win:NAME`), with
/// the name of the choice whose keys are all set, or "custom" when some of
/// a switch's keys are set but to no choice's values (a film's own
/// numbers, such as `crust_weight=2`).
///
/// It reads only the keys given on top of the default game, so a tier's
/// own trait (the Sprawler's `fire_alarm`) is not a switch that is on.
pub fn state(keys: &[(String, String)]) -> Vec<(&'static Switch, &'static str)> {
    let mut out = Vec::new();
    for s in SWITCHES {
        // The choice with the most keys that all hold wins, so "a wall"
        // (setting and set_block) is not read as plain setting.
        let held = s.choices.iter().filter(|c| !c.keys.is_empty() && c.keys.iter().all(|(k, v)| last(keys, k).is_some_and(|got| same(got, v)))).max_by_key(|c| c.keys.len());
        // Keys that are this switch's alone (no other switch's choice sets
        // them), given a value that is not "off".
        let own = |k: &str| !SWITCHES.iter().any(|o| o.key != s.key && o.choices.iter().any(|c| c.keys.iter().any(|(n, _)| *n == k)));
        let stray = s.choices.iter().flat_map(|c| c.keys.iter()).any(|(k, _)| own(k) && last(keys, k).is_some_and(|got| !same(got, "0")));
        match held {
            Some(c) if c.name != s.default => {
                // A switch whose every key belongs to a bigger choice of
                // another switch that also holds is that other switch's
                // doing (`setting=1` under "a wall").
                let covered = SWITCHES.iter().any(|o| o.key != s.key && o.choices.iter().any(|oc| oc.keys.len() > c.keys.len() && c.keys.iter().all(|k| oc.keys.contains(k)) && oc.keys.iter().all(|(k, v)| last(keys, k).is_some_and(|got| same(got, v)))));
                if !covered {
                    out.push((s, c.name));
                }
            }
            None if stray => out.push((s, "custom")),
            _ => {}
        }
    }
    out
}

/// The play log's word for `state`: `crust:on,set_slime:a_wall`, or
/// `none`.
pub fn log_word(keys: &[(String, String)]) -> String {
    let on = state(keys);
    if on.is_empty() {
        return "none".into();
    }
    on.iter().map(|(s, c)| format!("{}:{}", s.key, c.replace(' ', "_"))).collect::<Vec<_>>().join(",")
}

/// The keys that set `choice` of `s`, to push onto a round's keys.
pub fn keys_of(choice: &Choice) -> Vec<(String, String)> {
    choice.keys.iter().map(|(k, v)| (k.to_string(), v.to_string())).collect()
}

/// Every switch at its first choice off the default, together: the keys
/// the all-switches test plays (`mouldgpu`'s
/// `all_off_switches_on_together_play_on_the_engine`). Where two rows set
/// one key, the later row's value stands, as in a round.
pub fn all_on() -> Vec<(String, String)> {
    SWITCHES.iter().flat_map(|s| keys_of(s.tried())).collect()
}

/// **Sets one key** of a choice on the library's own parts: an engine
/// setting on `s`, a rule on `rules` or `rounds`. A window key is left to
/// the window (Ok). The error is the setter's.
pub fn set_key(s: &mut crate::gpu::Settings, rules: &mut crate::mould::larder_world::Rules, rounds: &mut crate::mould::rounds::RoundRules, k: &str, v: &str) -> Result<(), String> {
    if k.starts_with(WINDOW_KEY) {
        return Ok(());
    }
    match k.strip_prefix(RULE_KEY) {
        Some(name) => {
            let x: f32 = v.parse().map_err(|e| format!("set={name}={v}: {e}"))?;
            // Both take it: a rule either has is set (`sick_upkeep` is
            // nobody's twice).
            let (a, b) = (rules.set(name, x), rounds.set(name, x));
            if a || b {
                Ok(())
            } else {
                Err(format!("set={name}: no such rule"))
            }
        }
        None => crate::gpu::set(s, k, v),
    }
}

#[cfg(test)]
mod tests;
