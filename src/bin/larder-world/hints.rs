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
            self.on = !window || saved_on();
            if window {
                self.seen = settings::get("hints_seen").map(|s| s.split(',').filter(|k| !k.is_empty()).map(String::from).collect()).unwrap_or_default();
            }
        }
        self.show = None;
        let t = g.time();
        if t + 0.5 < self.last_t {
            // A new planet: its clock started again.
            (self.live, self.asleep, self.hit) = (None, None, None);
        }
        self.last_t = t;
        let hits: Vec<(f32, f32)> = g.last_shots.iter().filter_map(|s| s.hit).collect();
        if !hits.is_empty() {
            let n = hits.len() as f32;
            self.hit = Some((hits.iter().map(|h| h.0).sum::<f32>() / n, hits.iter().map(|h| h.1).sum::<f32>() / n));
        }
        let woke = self.woke(g, t);
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
        for key in KEYS.iter().map(|k| k.to_string()).chain(BossKind::ALL.iter().map(|b| format!("boss-{}", b.name()))) {
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

/// Hint `key` if its thing is on the planet now: its words, centre and
/// ring (fine cells).
fn sight(g: &Game<GpuWorld>, key: &str, hit: Option<(f32, f32)>, woke: bool) -> Option<(String, (f32, f32), f32)> {
    let sc = g.world.scale();
    let reach = g.gun_range_max() * sc;
    let (text, at, ring) = match key {
        "strikers" if g.strikers.live > 0 => ("Strikers: near your squad the slime packs into hard knots that charge it", squad_at(g)?, reach + g.rules.strikers.band * sc),
        "wave" => {
            let w = g.waves.live.as_ref()?;
            let n = w.points.len().max(1) as f32;
            let r = w.points.iter().map(|p| (p.0 - w.centre.0).hypot(p.1 - w.centre.1)).sum::<f32>() / n;
            ("A wave: the slime pulls back all round your squad, then strikes from every side", w.centre, r)
        }
        "lunge" if g.late => ("Cornered, what is left of the slime charges farther and faster: keep your distance", squad_at(g)?, reach + g.rules.late_margin.max(0.0) * sc),
        "stand" => {
            let h = g.stand.live.as_ref().filter(|h| (!h.origin || h.gather) && h.members > 0)?;
            ("Last stand: the slime packs into its heart, which lunges at soldiers in reach", h.centre, body(h.members))
        }
        "fruit" => {
            let f = g.fruits.live.as_ref().filter(|f| f.members > 0)?;
            ("Fruiting: kill this knot before it ripens and bursts into new slime", f.centre, body(f.members))
        }
        "core" => {
            let d = g.starfall.live.iter().find(|d| d.down && d.left > 0.01 && d.kind != DropKind::Ore)?;
            let text = match d.from {
                CoreFrom::Boss { .. } => "Mini-boss core: stand a squad on it to collect shop points",
                _ => "Core: stand a squad on it to collect shop points",
            };
            (text, d.at, d.r + g.rules.star.reach * sc)
        }
        "wake" if woke => ("Sleeping slime wakes where your shots land, and charges", hit.or_else(|| squad_at(g))?, 0.0),
        "scar" if g.world.scarred() >= 50.max(g.mould / 50) => ("Slime you keep hitting in one place grows back tough there: fight from fresh ground", hit.or_else(|| squad_at(g))?, 0.0),
        k => {
            let kind = BossKind::ALL.into_iter().find(|b| k.strip_prefix("boss-") == Some(b.name()))?;
            let b = g.bosses.live.iter().find(|b| b.kind == kind && b.members > 0)?;
            let text = match kind {
                BossKind::Hunter => "Mini-boss, a hunter: it chases your nearest soldier. Break it for a core",
                BossKind::Wanderer => "Mini-boss, a wanderer: it roots on a far crystal and grows there",
                BossKind::Sapper => "Mini-boss, a sapper: it eats through your nearest wall or building",
                BossKind::Leech => "Mini-boss, a leech: it sits on a pump and takes its food until broken",
                BossKind::Mortar => "Mini-boss, a mortar: it sits still and lobs slime at what is in reach",
                BossKind::Pinwheel => "Mini-boss, a pinwheel: a hunter whose arms sweep through your squad",
            };
            (text, b.centre, body(b.members))
        }
    };
    Some((text.to_string(), at, ring))
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
