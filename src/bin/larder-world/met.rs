//! **The Slimes met page** (Direction, 2026-11-24 18:41Z: "A "Slimes met"
//! page beside Past runs. It shows each slime and threat this PC has met:
//! its name, its card line, a small still from the game, how many times it
//! was beaten and how many times it won, and which hint it shows. Grey out
//! the ones not yet met. This gives a long run something to fill in and
//! tells him what's still ahead").
//!
//! What it reads, all already kept on this PC:
//! - a slime: met once a run has landed on its planet, beaten for each
//!   planet of it won and won for each run lost there, from the ended runs
//!   (`runs.txt`) and the run under way;
//! - a threat or mini-boss: met once its first-sight hint showed
//!   (`hints_seen`) or a planet with it ended, its counts from the planets
//!   won and lost while it showed (`hints::tally`, kept from this build on).

use slime_mould::mould::larder_world::BossKind;
use slime_mould::mould::run::{self, Run, LADDER};
use slime_mould::world::start::{Met, PastRun};

use super::draw::title_case;
use super::hints;

/// The stills' sheet (`assets/met-stills.png`, `Canvas::still`), a cell a
/// key in this order: the slimes by planet, the threats, the mini-bosses.
/// `assets/met-stills.py` films and cuts it in the same order.
pub(super) const STILLS: [&str; 22] = [
    "slime-drifter",
    "slime-sprawler",
    "slime-trunk",
    "slime-swarmer",
    "slime-sporer",
    "slime-sleeper",
    "slime-grudge",
    "slime-dozer",
    "strikers",
    "fruit",
    "core",
    "wave",
    "lunge",
    "stand",
    "wake",
    "scar",
    "boss-hunter",
    "boss-wanderer",
    "boss-sapper",
    "boss-leech",
    "boss-mortar",
    "boss-pinwheel",
];

/// A threat's name on the page and what it is, by its hint key.
fn threat(key: &str) -> Option<(&'static str, String)> {
    let from = |rule: &str| format!("A threat from planet {}", run::first_planet(rule));
    Some(match key {
        "strikers" => ("Strikers", from("strikers")),
        "fruit" => ("Fruiting", from("fruit")),
        "core" => ("Cores", "Shop points to pick up".to_string()),
        "wave" => ("Waves", from("wave")),
        "lunge" => ("The cornered slime", from("late_at")),
        "stand" => ("The last stand", from("stand")),
        "wake" => ("Waking", "Where the slime sleeps".to_string()),
        "scar" => ("Scar", "Where the slime holds a grudge".to_string()),
        _ => return None,
    })
}

/// Every slime and threat for the page, met or not, in the sheet's order.
pub(super) fn entries(runs: &[PastRun], live: Option<&Run>) -> Vec<Met> {
    let seen = hints::seen();
    let tally = hints::tally();
    let cell = |key: &str| STILLS.iter().position(|k| *k == key).map(|i| i as u32);
    let mut out = Vec::new();
    for (d, tier) in LADDER.iter().enumerate() {
        let name = title_case(tier.name());
        let planet = d + 1;
        let mut beaten = 0;
        let mut won = 0;
        let mut met = false;
        for r in runs {
            for (k, p) in r.planets.iter().enumerate() {
                if p.slime != name {
                    continue;
                }
                met |= p.stars.is_some() || p.secs.is_some() || k + 1 == r.depth as usize;
                beaten += u32::from(p.stars.is_some_and(|s| s > 0));
                won += u32::from(r.end == "lost" && k + 1 == r.depth as usize);
            }
        }
        // The run under way: the planets won so far, and the one it is on.
        if let Some(r) = live.filter(|r| !r.range) {
            met |= r.depth as usize >= planet;
            beaten += u32::from(r.stars.get(d).is_some_and(|&s| s > 0));
        }
        out.push(Met {
            name,
            kind: format!("The slime of planet {planet}"),
            group: 0,
            words: run::does_to_you(*tier).says.to_string(),
            said: "On its landing card".into(),
            still: cell(&format!("slime-{}", tier.name())),
            met,
            beaten,
            won,
        });
    }
    let keys = ["strikers", "fruit", "core", "wave", "lunge", "stand", "wake", "scar"].map(String::from).into_iter().chain(BossKind::ALL.iter().map(|b| format!("boss-{}", b.name())));
    for key in keys {
        let (name, kind, group) = match threat(&key) {
            Some((n, k)) => (n.to_string(), k, 1),
            None => {
                let b = key.strip_prefix("boss-").unwrap_or(&key);
                (title_case(b).trim_start_matches("The ").to_string(), "A mini-boss: break it for a core".to_string(), 2)
            }
        };
        let (beaten, won) = tally.iter().find(|e| e.0 == key).map_or((0, 0), |e| (e.1, e.2));
        out.push(Met {
            name,
            kind,
            group,
            words: hints::words(&key).unwrap_or_default().to_string(),
            said: "Its first-sight hint".into(),
            still: cell(&key),
            met: seen.contains(&key) || beaten + won > 0,
            beaten,
            won,
        });
    }
    out
}

#[cfg(test)]
mod tests {
    use super::*;

    /// Every entry has its still and its words, and the sheet has a cell
    /// for each, in the page's order.
    #[test]
    fn each_slime_and_threat_has_a_still_and_words() {
        let all = entries(&[], None);
        assert_eq!(all.len(), STILLS.len());
        for (i, m) in all.iter().enumerate() {
            assert_eq!(m.still, Some(i as u32), "{} is not cell {i}", m.name);
            assert!(!m.words.is_empty() && !m.name.is_empty(), "{m:?}");
            for w in ["walker", "mould", "larder", "dish", "origin", "tier"] {
                assert!(!m.words.to_lowercase().contains(w) && !m.kind.to_lowercase().contains(w), "{m:?}: {w}");
            }
        }
    }
}
