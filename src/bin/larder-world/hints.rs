//! **First-sight hints** (Direction, 2026-11-24 17:05Z: "Planet 1 has
//! coaching, but later threats arrive with no word"). The first time each
//! of the slime's later threats shows in a run on this PC (strikers, a
//! wave, the cornered slime's lunge, the last-stand heart, each kind of
//! mini-boss, fruiting, a dropped core, sleeping slime waking, scar), one
//! short line hangs under it for a few seconds, with no pause.
//!
//! Nothing is painted on: the line hangs from the thing itself (its
//! centre, read from the game each frame, so it follows it), and the ring
//! round it is the simulation's own number for it: a body's radius from
//! its members (`sqrt(members / pi)` fine cells, as `bosses.rs` and
//! `fruit.rs` size one), the band strikers condense in (guns' reach plus
//! `StrikerRules::band`), the reach the cornered slime lunges from (guns'
//! reach plus `Rules::late_margin`), a wave's gather points, a core's
//! pickup reach.
//!
//! What has been seen is kept in the settings file (`hints_seen`, a comma
//! list), as the coaching's `coached`; Options' "Hints" row turns them off
//! (`hints=off`). Films and tests show each on its first sight and never
//! write.

use slime_mould::gpu::larder_world::GpuWorld;
use slime_mould::mould::larder_world::World;
use slime_mould::mould::larder_world::{BossKind, CoreFrom, DropKind, Game};
use slime_mould::world::settings;

/// Game seconds a hint stays (whole, then fading out over `FADE`).
const SHOW_SECS: f32 = 7.0;
const FADE: f32 = 1.2;

/// One hint on screen: what it is (its `hints_seen` key), its words, the
/// thing's centre and ring (fine cells; 0 for none), and when it began.
#[derive(Clone, Debug)]
struct Live {
    key: String,
    text: String,
    at: (f32, f32),
    ring: f32,
    since: f32,
}

#[derive(Default)]
pub(super) struct Hints {
    /// Read from the settings file (in the window, once).
    loaded: bool,
    pub(super) on: bool,
    seen: Vec<String>,
    live: Option<Live>,
    /// The planet being watched (game time at its last frame: a new planet
    /// starts its clock again), the sleeping count a game second ago, and
    /// where the squad's shots last landed.
    last_t: f32,
    asleep: Option<(f32, usize)>,
    hit: Option<(f32, f32)>,
    /// What the window shows this frame: words, centre, ring, opacity.
    pub(super) show: Option<(String, (f32, f32), f32, f32)>,
    /// Every hint's thing that showed on this planet, hinted or not: the
    /// Slimes met page counts the planets each was on (`tally`).
    planet: Vec<String>,
}

/// The settings key of the Slimes met page's counts: `KEY:BEATEN:WON`,
/// comma separated, a planet won or lost while the thing showed.
const TALLY: &str = "met_tally";

/// Each hint's planets beaten and lost with it on (`TALLY`), as saved.
pub(super) fn tally() -> Vec<(String, u32, u32)> {
    let text = settings::get(TALLY).unwrap_or_default();
    text.split(',')
        .filter_map(|e| {
            let mut p = e.split(':');
            let (k, b, w) = (p.next()?, p.next()?.parse().ok()?, p.next()?.parse().ok()?);
            (!k.is_empty()).then(|| (k.to_string(), b, w))
        })
        .collect()
}

/// What this PC has seen (`hints_seen`), even with hints off.
pub(super) fn seen() -> Vec<String> {
    settings::get("hints_seen").map(|s| s.split(',').filter(|k| !k.is_empty()).map(String::from).collect()).unwrap_or_default()
}

/// Whether hints are on in the settings file (on unless `hints=off`).
pub(super) fn saved_on() -> bool {
    settings::get("hints").as_deref() != Some("off")
}

impl Hints {
    /// Once a frame. `window`: read and write the settings file (films and
    /// tests show every hint once a game and keep nothing). `busy`: another
    /// pinned hint (the Starfall one) has the slot this frame.
    pub(super) fn update(&mut self, g: &Game<GpuWorld>, window: bool, busy: bool) {
        if !self.loaded {
            self.loaded = true;
            // `LARDER_HINTS=off`: a film with none (the Slimes met page's
            // stills, which show the thing without its words).
            self.on = (!window || saved_on()) && std::env::var("LARDER_HINTS").as_deref() != Ok("off");
            if window {
                self.seen = seen();
            }
        }
        self.show = None;
        let t = g.time();
        if t + 0.5 < self.last_t {
            // A new planet: its clock started again.
            (self.live, self.asleep, self.hit) = (None, None, None);
            self.planet.clear();
        }
        self.last_t = t;
        let hits: Vec<(f32, f32)> = g.last_shots.iter().filter_map(|s| s.hit).collect();
        if !hits.is_empty() {
            let n = hits.len() as f32;
            self.hit = Some((hits.iter().map(|h| h.0).sum::<f32>() / n, hits.iter().map(|h| h.1).sum::<f32>() / n));
        }
        let woke = self.woke(g, t);
        // What is on this planet, for the Slimes met page's counts, with
        // hints on or off.
        for key in all_keys() {
            if !self.planet.contains(&key) && sight(g, &key, self.hit, woke).is_some() {
                self.planet.push(key);
            }
        }
        if !self.on {
            self.live = None;
            return;
        }
        if let Some(l) = &mut self.live {
            let age = t - l.since;
            if age >= SHOW_SECS {
                self.live = None;
            } else {
                // Follow the thing while it is there; else stay where it was.
                if let Some((_, at, ring)) = sight(g, &l.key, self.hit, woke) {
                    (l.at, l.ring) = (at, ring);
                }
                let a = (age / 0.5).min(1.0) * ((SHOW_SECS - age) / FADE).clamp(0.0, 1.0);
                self.show = Some((l.text.clone(), l.at, l.ring, a));
                return;
            }
        }
        if busy {
            return;
        }
        for key in all_keys() {
            if self.seen.contains(&key) {
                continue;
            }
            let Some((text, at, ring)) = sight(g, &key, self.hit, woke) else { continue };
            self.seen.push(key.clone());
            if window {
                let _ = settings::set("hints_seen", &self.seen.join(","));
            }
            if std::env::var_os("HINTS_LOG").is_some() {
                eprintln!("hint at {t:.1} s: {key} at {at:?}, ring {ring:.1}");
            }
            self.live = Some(Live { key, text, at, ring, since: t });
            self.show = Some((self.live.as_ref().unwrap().text.clone(), at, ring, 0.0));
            return;
        }
    }

    /// The planet ended, `beaten` or lost: each thing that showed on it
    /// counts it (`TALLY`, in the window only; films and tests keep
    /// nothing), and the next planet starts a new list.
    pub(super) fn ended(&mut self, beaten: bool, window: bool) {
        if window && !self.planet.is_empty() {
            let mut all = tally();
            for key in &self.planet {
                let i = all.iter().position(|e| &e.0 == key).unwrap_or_else(|| {
                    all.push((key.clone(), 0, 0));
                    all.len() - 1
                });
                if beaten {
                    all[i].1 += 1;
                } else {
                    all[i].2 += 1;
                }
            }
            let text: Vec<String> = all.iter().map(|(k, b, w)| format!("{k}:{b}:{w}")).collect();
            let _ = settings::set(TALLY, &text.join(","));
        }
        self.planet.clear();
    }

    /// Whether the sleeping slime woke this game second: a fifth or more of
    /// it (at least 200, and a tenth of the slime) gone from asleep in one
    /// game second (`GpuWorld::asleep`, the engine's per-frame count).
    fn woke(&mut self, g: &Game<GpuWorld>, t: f32) -> bool {
        let n = g.world.asleep();
        match self.asleep {
            Some((at, _)) if t - at < 1.0 => false,
            Some((_, was)) => {
                self.asleep = Some((t, n));
                was >= 200.max(g.mould / 10) && (n as f32) < 0.8 * was as f32
            }
            None => {
                self.asleep = Some((t, n));
                false
            }
        }
    }
}

/// The hints that are not a mini-boss, in the order they are checked when
/// two come at once.
const KEYS: [&str; 8] = ["stand", "wave", "strikers", "lunge", "fruit", "core", "wake", "scar"];

/// Where the squad the player has (or the first live one) stands.
fn squad_at(g: &Game<GpuWorld>) -> Option<(f32, f32)> {
    g.active.and_then(|q| g.middle(q)).or_else(|| g.live_squads().into_iter().find_map(|q| g.middle(q)))
}

/// A body's radius in fine cells from its members, as the game sizes one.
fn body(members: u32) -> f32 {
    (members as f32 / std::f32::consts::PI).sqrt()
}

/// Hint `key`'s words (`core-boss`: a mini-boss's core), as it shows in
/// play and on the Slimes met page; None for no such hint.
pub(super) fn words(key: &str) -> Option<&'static str> {
    Some(match key {
        "strikers" => "Strikers: near your squad the slime packs into hard knots that charge it",
        "wave" => "A wave: the slime pulls back all round your squad, then strikes from every side",
        "lunge" => "Cornered, what is left of the slime charges farther and faster: keep your distance",
        "stand" => "Last stand: the slime packs into its heart, which lunges at soldiers in reach",
        "fruit" => "Fruiting: kill this knot before it ripens and bursts into new slime",
        "core" => "Core: stand a squad on it to collect shop points",
        "core-boss" => "Mini-boss core: stand a squad on it to collect shop points",
        "wake" => "Sleeping slime wakes where your shots land, and charges",
        "scar" => "Slime you keep hitting in one place grows back tough there: fight from fresh ground",
        "boss-hunter" => "Mini-boss, a hunter: it chases your nearest soldier. Break it for a core",
        "boss-wanderer" => "Mini-boss, a wanderer: it roots on a far crystal and grows there",
        "boss-sapper" => "Mini-boss, a sapper: it eats through your nearest wall or building",
        "boss-leech" => "Mini-boss, a leech: it sits on a pump and takes its food until broken",
        "boss-mortar" => "Mini-boss, a mortar: it sits still and lobs slime at what is in reach",
        "boss-pinwheel" => "Mini-boss, a pinwheel: a hunter whose arms sweep through your squad",
        _ => return None,
    })
}

/// Every hint's key: the threats, then each mini-boss kind.
pub(super) fn all_keys() -> Vec<String> {
    KEYS.iter().map(|k| k.to_string()).chain(BossKind::ALL.iter().map(|b| format!("boss-{}", b.name()))).collect()
}

/// Hint `key` if its thing is on the planet now: its words, centre and
/// ring (fine cells).
fn sight(g: &Game<GpuWorld>, key: &str, hit: Option<(f32, f32)>, woke: bool) -> Option<(String, (f32, f32), f32)> {
    let sc = g.world.scale();
    let reach = g.gun_range_max() * sc;
    let (at, ring) = match key {
        "strikers" if g.strikers.live > 0 => (squad_at(g)?, reach + g.rules.strikers.band * sc),
        "wave" => {
            let w = g.waves.live.as_ref()?;
            let n = w.points.len().max(1) as f32;
            let r = w.points.iter().map(|p| (p.0 - w.centre.0).hypot(p.1 - w.centre.1)).sum::<f32>() / n;
            (w.centre, r)
        }
        "lunge" if g.late => (squad_at(g)?, reach + g.rules.late_margin.max(0.0) * sc),
        "stand" => {
            let h = g.stand.live.as_ref().filter(|h| (!h.origin || h.gather) && h.members > 0)?;
            (h.centre, body(h.members))
        }
        "fruit" => {
            let f = g.fruits.live.as_ref().filter(|f| f.members > 0)?;
            (f.centre, body(f.members))
        }
        "core" => {
            let d = g.starfall.live.iter().find(|d| d.down && d.left > 0.01 && d.kind != DropKind::Ore)?;
            let key = if matches!(d.from, CoreFrom::Boss { .. }) { "core-boss" } else { "core" };
            return Some((words(key)?.to_string(), d.at, d.r + g.rules.star.reach * sc));
        }
        "wake" if woke => (hit.or_else(|| squad_at(g))?, 0.0),
        "scar" if g.world.scarred() >= 50.max(g.mould / 50) => (hit.or_else(|| squad_at(g))?, 0.0),
        k => {
            let kind = BossKind::ALL.into_iter().find(|b| k.strip_prefix("boss-") == Some(b.name()))?;
            let b = g.bosses.live.iter().find(|b| b.kind == kind && b.members > 0)?;
            (b.centre, body(b.members))
        }
    };
    Some((words(key)?.to_string(), at, ring))
}

#[cfg(test)]
mod tests {
    /// Every hint is one short line in the game's words (`news/words.txt`).
    #[test]
    fn hints_are_short_and_in_the_games_words() {
        let all = [
            "Strikers: near your squad the slime packs into hard knots that charge it",
            "A wave: the slime pulls back all round your squad, then strikes from every side",
            "Cornered, what is left of the slime charges farther and faster: keep your distance",
            "Last stand: the slime packs into its heart, which lunges at soldiers in reach",
            "Fruiting: kill this knot before it ripens and bursts into new slime",
            "Sleeping slime wakes where your shots land, and charges",
            "Slime you keep hitting in one place grows back tough there: fight from fresh ground",
        ];
        let src = include_str!("hints.rs");
        for t in all {
            assert!(src.matches(t).count() >= 2, "{t} is not a hint's text any more");
            assert!(t.len() <= 90, "{t}: one short line");
            for w in ["walker", "mould", "larder", "dish", "origin", "tier"] {
                assert!(!t.to_lowercase().contains(w), "{t}: {w}");
            }
        }
    }
}
