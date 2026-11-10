//! **The listener**: reads the dish, says what should be heard.
//!
//! Takes `&Dish` only, so it cannot change the game, and draws no random
//! numbers at all (the voice bank has its own). Called once per drawn frame
//! with the wall-clock time since the last call; it compares the dish's
//! tallies (`Dish::counts`) with the last frame's and turns the differences
//! into one-shots, and turns the current counts (births per second,
//! walkers on food, deaths per second) into textures.
//!
//! **Rates are per wall second**, so fast-forward (four times the steps a
//! frame) sounds four times busier, as it looks, up to the texture ceiling.

use crate::mould::{Counts, Dish, Phase, W};

use super::synth::TEXTURE_RATE_CAP;
use super::{AlertKind, GrainKind, SoundCmd, TextureKind};

/// The exponent of the loudness law: amplitude follows `count^LOUDNESS_LAW`.
/// From Pixel_Physics (its design §3b): a hundredfold crowd is about 16 dB
/// louder rather than 40, so a big mould does not drown its own tools.
pub const LOUDNESS_LAW: f32 = 0.4;
/// Seconds the per-second rates are smoothed over. Births arrive a few per
/// frame, so an unsmoothed rate flickers between 0 and a few hundred.
const RATE_SMOOTH: f32 = 0.35;
/// Seconds of fighting at the seam gathered into one burst.
const FIGHT_GATHER: f32 = 0.25;

/// Map `count` sources into a texture's (grain rate, per-grain level).
/// Rate follows the count up to `TEXTURE_RATE_CAP`; amplitude follows
/// `count^LOUDNESS_LAW`, divided by the square root of the rate actually
/// played so overlapping grains do not add up to more.
pub fn density(count: f32, per_source: f32, reference: f32, level_at_reference: f32) -> (f32, f32) {
    if count <= 0.0 {
        return (0.0, 0.0);
    }
    let rate = (count * per_source).min(TEXTURE_RATE_CAP);
    let amplitude = level_at_reference * (count / reference).powf(LOUDNESS_LAW);
    let level = (amplitude / (rate / 50.0).max(0.02).sqrt()).min(1.0);
    (rate, level)
}

/// -1 at the dish's left edge, 1 at its right.
fn pan_of(x: f32) -> f32 {
    (x / W as f32 * 2.0 - 1.0).clamp(-1.0, 1.0)
}

/// Counters for "did it fire": printed beside every recording.
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct ListenStats {
    pub hears: u64,
    pub rebaselines: u64,
    pub oats: u32,
    pub salt: u32,
    pub snips: u32,
    pub drops: u32,
    pub burned: u64,
    pub joins: u32,
    pub arrivals: u32,
    pub fights: u64,
    pub storms: u32,
    pub warnings: u32,
    pub bells: u32,
    pub deaths: u32,
    /// The textures' last per-second inputs: births/s, walkers on food,
    /// deaths/s.
    pub births_per_sec: f32,
    pub eaters: u32,
    pub starving_per_sec: f32,
}

#[derive(Clone, Copy, Debug, PartialEq)]
enum Ending {
    Playing,
    Ended,
}

pub struct Listener {
    seed: u64,
    t: u64,
    counts: Counts,
    phase: Phase,
    warned: bool,
    alive: bool,
    ending: Ending,
    births_rate: f32,
    starve_rate: f32,
    burn_rate: f32,
    fight_acc: f32,
    fight_wait: f32,
    pub stats: ListenStats,
}

impl Listener {
    pub fn new(d: &Dish) -> Listener {
        let mut l = Listener {
            seed: 0,
            t: 0,
            counts: Counts::default(),
            phase: Phase::Growing,
            warned: false,
            alive: true,
            ending: Ending::Playing,
            births_rate: 0.0,
            starve_rate: 0.0,
            burn_rate: 0.0,
            fight_acc: 0.0,
            fight_wait: 0.0,
            stats: ListenStats::default(),
        };
        l.rebaseline(d);
        l.stats.rebaselines = 0;
        l
    }

    /// Take the dish as it is now as the baseline, so a new dish (restart,
    /// next level) does not sound as one enormous burst of every tally.
    fn rebaseline(&mut self, d: &Dish) {
        self.seed = d.seed;
        self.t = d.t;
        self.counts = d.counts.clone();
        self.phase = d.phase;
        self.warned = d.storm_line.is_some();
        self.alive = !d.walkers.is_empty();
        self.ending = if d.phase == Phase::Over { Ending::Ended } else { Ending::Playing };
        self.births_rate = 0.0;
        self.starve_rate = 0.0;
        self.burn_rate = 0.0;
        self.fight_acc = 0.0;
        self.fight_wait = 0.0;
        self.stats.rebaselines += 1;
    }

    /// What should sound after this frame. `dt` is wall-clock seconds since
    /// the last call; `hushed` is the window's paused / card-up state.
    pub fn hear(&mut self, d: &Dish, dt: f32, hushed: bool) -> Vec<SoundCmd> {
        let mut out = Vec::with_capacity(12);
        self.stats.hears += 1;
        let c = &d.counts;
        // A different dish, or this one restarted: start over, silently.
        // (Not the event lists' lengths: `snaps` is pruned as its flashes fade.)
        let went_back = d.t < self.t || c.births < self.counts.births || c.oats_placed < self.counts.oats_placed || c.snaps < self.counts.snaps;
        if d.seed != self.seed || went_back {
            self.rebaseline(d);
        }
        out.push(SoundCmd::Hush(hushed));
        let dt = dt.max(1e-3);
        let k = 1.0 - (-dt / RATE_SMOOTH).exp();
        let p = self.counts.clone();

        // --- textures --------------------------------------------------
        let births = (c.births - p.births) as f32 / dt;
        let starving = ((c.starved + c.hungry + c.died_asleep) - (p.starved + p.hungry + p.died_asleep)) as f32 / dt;
        self.births_rate += (births - self.births_rate) * k;
        self.starve_rate += (starving - self.starve_rate) * k;
        // Where the mould is: the walkers' mean x and spread.
        let n = d.walkers.len().max(1) as f32;
        let (sx, sxx) = d.walkers.iter().fold((0.0f32, 0.0f32), |(a, b), w| (a + w.x, b + w.x * w.x));
        let mx = sx / n;
        let spread = ((sxx / n - mx * mx).max(0.0).sqrt() / W as f32 * 2.0).min(1.0);
        let pan = pan_of(mx);
        let (creep_rate, creep_level) = density(self.births_rate, 1.0, 60.0, 0.22);
        out.push(SoundCmd::Texture { kind: TextureKind::Creep, rate: creep_rate, level: creep_level, pan, spread });
        // Feeding, panned to the eaten piles.
        let (mut eaters, mut ex) = (0u32, 0.0f32);
        for pile in &d.piles {
            let e = pile.eaters + pile.foe_eaters;
            eaters += e;
            ex += pile.x * e as f32;
        }
        let feed_pan = if eaters > 0 { pan_of(ex / eaters as f32) } else { pan };
        let (feed_rate, feed_level) = density(eaters as f32, 0.12, 100.0, 0.14);
        out.push(SoundCmd::Texture { kind: TextureKind::Feed, rate: feed_rate, level: feed_level, pan: feed_pan, spread: 0.3 });
        let (starve_rate, starve_level) = density(self.starve_rate, 1.0, 30.0, 0.35);
        out.push(SoundCmd::Texture { kind: TextureKind::Starve, rate: starve_rate, level: starve_level, pan, spread });
        self.stats.births_per_sec = self.births_rate;
        self.stats.eaters = eaters;
        self.stats.starving_per_sec = self.starve_rate;

        // --- the lamp ---------------------------------------------------
        let lamp_pan = d.lamp.map(|(x, _)| pan_of(x)).unwrap_or(0.0);
        out.push(SoundCmd::Lamp { on: d.lamp.is_some(), pan: lamp_pan });
        let burned = c.burned - p.burned;
        if burned > 0 {
            self.stats.burned += burned;
            self.burn_rate += burned as f32;
        }
        // Burning comes a few walkers a frame; gather it into a burst every
        // so often rather than one burst per frame.
        if self.burn_rate >= 3.0 || (self.burn_rate > 0.0 && d.lamp.is_none()) {
            let (rate, _) = density(self.burn_rate / dt.max(0.1), 1.0, 30.0, 0.3);
            out.push(SoundCmd::Burst { kind: GrainKind::Sizzle, rate: rate.max(20.0), secs: 0.12, pan: lamp_pan, spread: 0.1, gain: (0.2 * (self.burn_rate / 3.0).powf(LOUDNESS_LAW)).min(0.6) });
            self.burn_rate = 0.0;
        }

        // --- the tools: each one a tally that moved --------------------
        if c.oats_placed > p.oats_placed {
            let at = d.piles.iter().rev().find(|q| q.oats).map(|q| pan_of(q.x)).unwrap_or(0.0);
            self.stats.oats += c.oats_placed - p.oats_placed;
            out.push(SoundCmd::Grain { kind: GrainKind::Thunk, pan: at, gain: 0.7, delay: 0.0 });
        }
        if c.salt_cells > p.salt_cells {
            let cells = (c.salt_cells - p.salt_cells) as f32;
            self.stats.salt += c.salt_cells - p.salt_cells;
            out.push(SoundCmd::Burst { kind: GrainKind::Grit, rate: (cells * 8.0).clamp(30.0, 160.0), secs: 0.08, pan, spread: 0.3, gain: 0.35 });
        }
        for &(x, _, _) in d.snip_marks.iter().filter(|e| e.2 >= self.t) {
            self.stats.snips += 1;
            out.push(SoundCmd::Grain { kind: GrainKind::Knock, pan: pan_of(x), gain: 0.35, delay: 0.0 });
        }
        let snipped = (c.snipped + c.salted) - (p.snipped + p.salted);
        if snipped > 0 {
            out.push(SoundCmd::Burst { kind: GrainKind::Crackle, rate: (snipped as f32 * 4.0).clamp(20.0, 120.0), secs: 0.1, pan, spread: 0.2, gain: 0.25 });
        }
        if c.water_drops > p.water_drops {
            self.stats.drops += c.water_drops - p.water_drops;
            for i in 0..(c.water_drops - p.water_drops).min(3) {
                out.push(SoundCmd::Grain { kind: GrainKind::Drip, pan, gain: 0.5, delay: i as f32 * 0.07 });
            }
        }
        // The seam with the wild mould fights a few walkers most frames
        // (1,520 in a played Wild One): gathered into one burst a quarter
        // second, or it was 1,251 bursts thrashing six slots.
        let fought = c.fought - p.fought;
        self.stats.fights += fought;
        self.fight_acc += fought as f32;
        self.fight_wait -= dt;
        if self.fight_acc > 0.0 && self.fight_wait <= 0.0 {
            let (rate, _) = density(self.fight_acc / FIGHT_GATHER, 1.0, 20.0, 0.2);
            let gain = (0.12 * (self.fight_acc / 5.0).powf(LOUDNESS_LAW)).min(0.3);
            out.push(SoundCmd::Burst { kind: GrainKind::Grit, rate: rate.max(10.0), secs: 0.12, pan: feed_pan, spread: 0.2, gain });
            self.fight_acc = 0.0;
            self.fight_wait = FIGHT_GATHER;
        }

        // --- the dish's own events --------------------------------------
        for &(x, _, _) in d.arrived.iter().filter(|e| e.2 >= self.t) {
            self.stats.arrivals += 1;
            out.push(SoundCmd::Grain { kind: GrainKind::Knock, pan: pan_of(x), gain: 0.25, delay: 0.0 });
        }
        for &(x, _, _) in d.snaps.iter().filter(|e| e.2 >= self.t) {
            self.stats.joins += 1;
            let weight = (c.snaps as f32 / 5.0).min(1.0);
            out.push(SoundCmd::Alert { kind: AlertKind::Joined, weight, pan: pan_of(x) });
        }
        if d.storm_line.is_some() && !self.warned {
            self.warned = true;
            self.stats.warnings += 1;
            let at = d.storm_line.map(|((x0, _), (x1, _))| pan_of((x0 + x1) * 0.5)).unwrap_or(0.0);
            out.push(SoundCmd::Grain { kind: GrainKind::Whoosh, pan: at, gain: 0.5, delay: 0.0 });
        }
        if let Phase::Storm(_) = d.phase {
            if !matches!(self.phase, Phase::Storm(_)) {
                self.stats.storms += 1;
                out.push(SoundCmd::Grain { kind: GrainKind::Thud, pan: 0.0, gain: 0.9, delay: 0.0 });
            }
        }
        if d.storm_line.is_none() {
            self.warned = false;
        }
        let alive = !d.walkers.is_empty();
        if self.alive && !alive {
            self.stats.deaths += 1;
            out.push(SoundCmd::Alert { kind: AlertKind::Died, weight: 1.0, pan });
        }
        if d.phase == Phase::Over && self.ending == Ending::Playing {
            self.ending = Ending::Ended;
            if alive {
                self.stats.bells += 1;
                let weight = d.score.as_ref().map(|s| s.after as f32 / s.piles.max(1) as f32).unwrap_or(0.0);
                out.push(SoundCmd::Alert { kind: AlertKind::Bell, weight, pan: 0.0 });
            }
        }

        self.alive = alive;
        self.phase = d.phase;
        self.t = d.t;
        self.counts = c.clone();
        out
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::mould::STEPS_PER_FRAME;

    #[test]
    fn density_rate_follows_count_to_the_ceiling_and_level_grows_slowly() {
        let (r1, _) = density(10.0, 1.0, 60.0, 0.3);
        let (r2, _) = density(1000.0, 1.0, 60.0, 0.3);
        assert_eq!(r1, 10.0);
        assert_eq!(r2, TEXTURE_RATE_CAP);
        assert_eq!(density(0.0, 1.0, 60.0, 0.3), (0.0, 0.0));
    }

    /// The point of the read-only listener: listening to every frame of a
    /// played dish leaves the dish exactly where an unheard one ends up.
    #[test]
    fn listening_never_changes_the_game() {
        let play = |listen: bool| {
            let mut d = Dish::new(4);
            let mut l = Listener::new(&d);
            for f in 0..1800 {
                if f == 300 {
                    let (x, y) = (d.piles[0].x + 30.0, d.piles[0].y);
                    d.place_oats(x, y);
                }
                for _ in 0..STEPS_PER_FRAME {
                    d.step();
                }
                if listen {
                    l.hear(&d, 1.0 / 60.0, false);
                }
            }
            (d.state_hash(), l.stats)
        };
        let (heard, stats) = play(true);
        let (unheard, _) = play(false);
        assert_eq!(heard, unheard);
        // The positive control: the listener did hear things, so the equal
        // hashes are not the listener doing nothing.
        assert_eq!(stats.oats, 1, "{stats:?}");
        assert!(stats.births_per_sec > 0.0 || stats.eaters > 0, "{stats:?}");
    }

    #[test]
    fn a_new_dish_is_a_rebaseline_not_a_burst() {
        let mut d = Dish::new(1);
        for _ in 0..1200 {
            d.step();
        }
        let mut l = Listener::new(&d);
        let fresh = Dish::new(2);
        let cmds = l.hear(&fresh, 1.0 / 60.0, false);
        assert_eq!(l.stats.rebaselines, 1);
        assert!(!cmds.iter().any(|c| matches!(c, SoundCmd::Grain { .. } | SoundCmd::Burst { .. } | SoundCmd::Alert { .. })), "{cmds:?}");
    }
}
