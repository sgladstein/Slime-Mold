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
//! list began on 11-17 as ROADMAP's table as it stood, 14 rows. The Enemy
//! lane settled ten of them that day; four are left, one over the count,
//! and `MOST_OFF` is held by the compiler once one more is settled
//! (`SETTLING`).

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

/// True while the list is over `MOST_OFF`. It stands at four since the
/// Enemy lane settled its rows of ROADMAP's table (11-17): the set
/// slime's look (Look and interface), shake and hit-stop and the free
/// recruits (Squad and arsenal), crystals that refill (held with the
/// owner's answer on the rush, D7). Set it false in the change that brings
/// the list down to `MOST_OFF`, and a row over the count stops the build
/// from then on.
pub const SETTLING: bool = true;

// The count, held by the compiler: with `SETTLING` false, a fourth row
// does not build.
const _: () = assert!(SETTLING || SWITCHES.len() <= MOST_OFF, "more switches are declared than the plan allows (MOST_OFF): settle one (on, a tier's trait, or deleted) before adding another");

/// **The list.** It began as ROADMAP's "Switches that are off" table of
/// 2026-11-17 (14 rows). The Enemy lane settled ten that day: the far
/// smell and the charge paths are on on every run planet
/// (`run::enemy_keys`, `run::ENEMY_RULES`); the crust, its pull, setting,
/// the wall and fireproof fed slime are the Trunk's trait and the last
/// stand the Swarmer's (`run::does_to_you`); lossy flow, the lone step and
/// the trap (`set_hold`) were deleted (`Reports/dead-ends.md`). What
/// became of each is in ROADMAP, with the reports.
///
/// To add one: a row here, in the same change as the key. To settle one:
/// make its choice the engine's or the rules' own default (or a tier's
/// trait in `tiers.rs` / `run::does_to_you`) and delete the row, or delete
/// the feature and the row and add the `Reports/dead-ends.md` line.
pub const SWITCHES: &[Switch] = &[
    Switch {
        key: "set_look",
        label: "Set slime's look",
        choices: &[Choice { name: "amber", keys: &[] }, Choice { name: "glaze", keys: &[("win:setlook", "2")] }, Choice { name: "hidden", keys: &[("win:setlook", "0")] }],
        default: "amber",
        lane: Lane::Look,
        since: "2026-11-17",
        sees: "How the slime that sets on planet 3 (the Trunk) is drawn: amber, a glaze, or not marked at all",
        why_off: "A look pick: slime sets on planet 3 since 11-17 and is drawn amber. The Look and interface lane's to settle",
    },
    Switch {
        key: "feel",
        label: "Shake and hit-stop",
        choices: &[OFF, on(&[("win:shake", "1"), ("win:hitstop", "1")])],
        default: "off",
        lane: Lane::Squad,
        since: "2026-11-15",
        sees: "The camera shakes on blasts, and the game holds still for a moment when a big blast lands",
        why_off: "The owner asked to see it before it goes on (11-15, D6); his play with it on decides",
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
/// numbers, such as `crystal_regrow=0.01`).
///
/// It reads only the keys given on top of the default game, so a tier's
/// own trait (the Sprawler's `fire_alarm`) is not a switch that is on.
pub fn state(keys: &[(String, String)]) -> Vec<(&'static Switch, &'static str)> {
    let mut out = Vec::new();
    for s in SWITCHES {
        // The choice with the most keys that all hold wins: on 11-17 "a
        // wall" (setting and set_block) was not to be read as plain
        // setting. No two rows share a key since the Enemy lane settled
        // those; the rule stays for the next either-or that does.
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

/// The play log's word for `state`: `set_look:glaze,reserve:on`, or
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

/// The same with every switch at its last choice: the other side of each
/// either-or (the set slime's look hidden instead of a glaze), which the
/// all-switches test plays as its second round.
pub fn all_last() -> Vec<(String, String)> {
    SWITCHES.iter().flat_map(|s| keys_of(s.choices.iter().rev().find(|c| c.name != s.default).expect("a choice off the default"))).collect()
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
