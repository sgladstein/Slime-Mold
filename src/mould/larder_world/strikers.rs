//! **Condensed strikers** (2026-11-20, the thread "Make the slime a more
//! interesting enemy", the owner's pick on the balance thread's card that
//! day: "Condensed strikers + late lunge").
//!
//! The owner, 11-20 14:10Z: once the squad is big enough "the slime cannot
//! touch them, so it doesn't matter how much slime their is ... we could go
//! back to our early feature we had where the slime can condense and get
//! tougher. Although the visuals and mechanics of that would need to be
//! much improved from what we had before." And 14:16Z, of a hardening
//! idea: hardened slime "doesn't actually make this more challenging
//! because hardened slime doesn't attack you. It makes it easier."
//!
//! So late in a planet (under `below` of its peak), the slime's edge facing
//! a big squad condenses: once a game second each ordinary walker within
//! the guns' reach and `band` dish cells of a soldier tries with chance
//! `share` (none against `few` soldiers or fewer, all of it from `full`;
//! the run sets the share by planet, and `rate` caps how many form a
//! second), and in each block of fine cells the first that tries takes in
//! up to three of the rest (engine.wgsl's `condense`). A condensed walker
//! counts once, takes a blow for each walker it took in (and one more to
//! die), and bites once more for each. Mass and bite stay what they were:
//! no new slime. And it charges: every condensed walker runs for the
//! nearest soldier at `haste` of its pace, down the units' walking
//! distance, and never sits on a crystal. A shot's strike kills every
//! walker in its disc, so a quarter as many die a shot, and the same mass
//! crosses the guns' band four times as tough.
//!
//! The look is the simulation: a condensed walker is a bright bead in the
//! slime's gold, larger by what it holds, with a slow pulse, so where they
//! bunch they read as hot cores (view.wgsl's `draw_specks`). No bronze, no
//! outline.
//!
//! The old condensing (11-14 to 11-17, Reports/dead-ends.md) packed thick
//! slime anywhere: its lobes sat on their crystals and thick walkers got
//! stuck in the body. This one condenses only the edge facing the squad,
//! and every condensed walker charges.
//!
//! GPU only (the owner's 11-17 19:21Z card): a world that cannot condense
//! (`World::condense`, the CPU engine) never does.

use super::{Game, World};

/// The strikers' numbers. Off by default (the campaign and the sandbox are
/// frozen); a run turns it on from its planet (`run::ENEMY_RULES`, `run::FIRST_PLANET`).
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct StrikerRules {
    pub on: bool,
    /// The slime condenses while its mass is under this share of its peak.
    pub below: f32,
    /// How far past the guns' reach the condensing edge runs (dish cells).
    pub band: f32,
    /// The chance a walker in the band tries, once a game second.
    pub share: f32,
    /// A condensed walker's pace as it charges, times its own.
    pub haste: f32,
    /// The most walkers one takes in (1 to 15): it takes that many blows
    /// more and bites that many times more.
    pub most: u32,
    /// The squad sizes between which the edge condenses from none to the
    /// full `share` (in a straight line): a squad of `few` or fewer meets
    /// no strikers, one of `full` or more meets them all, so the strikers
    /// answer a big army, not a small squad.
    pub few: f32,
    pub full: f32,
    /// The most strikers formed a game second: `rate`, or `rate_share` of
    /// the living slime, whichever is fewer (0 `rate`: no cap). The share
    /// a walker tries with is cut to what would form about that many, from
    /// the walkers in the band the last pass counted (each striker takes
    /// in up to `most`). Without it (the retune of 11-20 evening) planet 1,
    /// run seed 1, 2:27, condensed 4,218 walkers into 1,607 strikers in one
    /// second against 43 soldiers, and all 43 died within 8 s (strikers
    /// off: won, 2 stars, 53 alive). Direction's numbers, 11-20 22:25Z:
    /// about 150 a second or 2% of the slime, so a strike builds over
    /// seconds you can see and answer.
    pub rate: f32,
    pub rate_share: f32,
    /// **The rate cap grows with an army past `full`** (2026-11-22, the
    /// thread "Make the run harder after full-game win": on the owner's
    /// 11-22 run of all 8 planets his army reached 90 to 100 and met the
    /// strikers a squad of 24 meets). The cap is times the living soldiers
    /// over `full`, at least 1 and at most `army_most` (1: the cap as
    /// before; a run sets it, `run::Run::rules`).
    pub army_most: f32,
    /// **A settled peak** (Lab, 2026-11-24, Direction's brief of 15:39Z):
    /// 0 reads the game's peak (`Game::peak`, from the landing) as before.
    /// Above 0, the strikers count their own peak only once the slime has
    /// settled: from the first second its mass is 1% over its lowest so
    /// far, or from `settle` game seconds at the latest, with no strikers
    /// before. Why: a lap-two slime (lab key `LAB_LAP2`) lands bigger than
    /// its crystals hold and sheds about half in its first 30 s (planet 12,
    /// 126k to 69k; 15, 234k to 124k), so under a peak taken at the
    /// landing it is "late" within seconds and strikers wiped a greedy
    /// squad inside a minute on 12 and 15 (strikers off: both lived to
    /// 2:00 with 24 soldiers). A lap-one slime grows from its landing, so
    /// it settles within seconds and its peak is the game's.
    pub settle: f32,
}

impl Default for StrikerRules {
    fn default() -> Self {
        // The retune of 2026-11-20 evening (was below 0.4, band 12, share
        // 0.5, haste 1.5, and every squad): the balance thread's 36 planet 2
        // games found the first numbers bit an 80-soldier army no more than
        // strikers off (up 1, down 3, tied 5). On lavapipe, planet 2 map 1,
        // one game each (soldiers at 6:30, bought, bank at the end): first
        // numbers 80, 79, 22,434; these 65, 109, 670. At 40% the slime fell
        // from 15k to 3k inside 30 s, before strikers could form; 60% is
        // what bites (haste 3 and 7 blows at 40%: 80, 80, 14,387), and 15
        // blows added little (62, 113, 328). Map 7, four soldiers: lost at
        // 2:33 with the first numbers, 1:14 with these and no squad scale,
        // won at 4:21 (as with no strikers on the GPU) with few 8, full 24.
        StrikerRules { on: false, below: 0.6, band: 30.0, share: 1.0, haste: 3.0, most: 3, few: 8.0, full: 24.0, rate: 150.0, rate_share: 0.02, army_most: 1.0, settle: 0.0 }
    }
}

/// What the strikers have done.
#[derive(Clone, Debug, Default)]
pub struct Strikers {
    /// The engine's rule is on (from the first second the slime is under
    /// `below` of its peak; it stays on, so condensed walkers keep charging
    /// if the slime grows back).
    pub on: bool,
    /// The condensed walkers living at the last condensing, and the
    /// walkers they still held.
    pub live: u32,
    pub held: u32,
    /// The walkers taken in, all told.
    pub took: u64,
    /// The walkers in the band at the last pass (`StrikerRules::rate`).
    pub band: u32,
    /// The settled peak (`StrikerRules::settle`): the slime's lowest mass
    /// so far until it settles, then its peak from there; and the game
    /// second it settled (0: not yet).
    pub low: usize,
    pub peak: usize,
    pub settled: f32,
}

impl<W: World> Game<W> {
    /// Once a game second: condense the edge facing the squad while the
    /// slime is under `below` of its peak.
    pub(super) fn time_strikers(&mut self) {
        let r = self.rules.strikers;
        if !r.on || r.haste <= 0.0 {
            if std::mem::take(&mut self.strikers.on) {
                self.world.set_strikers(None);
            }
            return;
        }
        let peak = if r.settle > 0.0 { self.settled_peak(r.settle) } else { self.peak };
        let low = peak > 0 && (self.mould as f32) < r.below * peak as f32;
        if !low && !self.strikers.on {
            return;
        }
        self.world.set_strikers(Some(r.haste));
        self.strikers.on = true;
        // Scaled by the squad (2026-11-20 retune, `few` and `full`): on
        // planet 2 map 7 the slime fell under its peak's share at 1:00
        // against four soldiers, and condensing there only crushed a squad
        // that was already losing (lavapipe, one game each: lost at 2:33
        // with strikers at 40%, at 1:14 with them at 60%, and still at 1:35
        // with the share scaled straight by the squad over 20 soldiers).
        let alive = self.soldiers.iter().filter(|s| s.alive).count() as f32;
        let army = ((alive - r.few) / (r.full - r.few).max(1.0)).clamp(0.0, 1.0);
        let mut share = if low { r.share * army } else { 0.0 };
        // The rate cap: about `cap` strikers this second, each the first of
        // up to `most + 1` tries in its block, from the band the last pass
        // counted (none counted yet: this pass only counts).
        if r.rate > 0.0 {
            let big = (alive / r.full.max(1.0)).clamp(1.0, r.army_most.max(1.0));
            let cap = r.rate.min(r.rate_share * self.mould as f32).max(0.0) * big;
            let tries = cap * (r.most + 1) as f32;
            share = if self.strikers.band == 0 { 0.0 } else { share.min(tries / self.strikers.band as f32) };
        }
        let reach = (self.gun_range_max() + r.band).max(0.0) * self.world.scale();
        if let Some((live, held, took, band)) = self.world.condense(reach, share, r.most) {
            (self.strikers.live, self.strikers.held, self.strikers.band) = (live, held, band);
            self.strikers.took += took as u64;
        }
    }

    /// The strikers' peak with `settle` on (`StrikerRules::settle`): 0
    /// until the slime settles, then its peak from there.
    fn settled_peak(&mut self, settle: f32) -> usize {
        let (t, mould) = (self.time(), self.mould);
        let st = &mut self.strikers;
        if st.settled > 0.0 {
            st.peak = st.peak.max(mould);
            return st.peak;
        }
        st.low = if st.low == 0 { mould } else { st.low.min(mould) };
        if (mould as f32) > 1.01 * st.low as f32 || t >= settle {
            (st.settled, st.peak) = (t.max(0.001), mould);
            return st.peak;
        }
        0
    }
}
