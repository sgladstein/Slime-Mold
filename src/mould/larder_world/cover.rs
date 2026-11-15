//! **Defenders while a squad works, and healing** (the squad mode, owner
//! 2026-11-14 15:06Z).
//!
//! "When building or harvesting, there could be a percentage of soldiers
//! who defend. Harvesters/builders can still shoot but slower. More
//! defenders slows build harvest time, but only a bit, not linearly.
//! Defenders will automatically position themselves in the best position
//! based on the the slime is but we have to be careful it doesn't just make
//! them run into the thickest part of the slime if the player tries to
//! harvest/build right next to slime. If the there is no slime nearby no
//! one defend by default. In the menu you get also change the squad
//! behavior to never defenders or more defenders than default."
//!
//! - A squad works when some of its members drain a crystal or raise a
//!   building (`Squad::work`, set by `Cmd::SquadOrder`). Once a game second
//!   (`cover_turn`) it looks for slime round the work: with enough within
//!   `CoverRules::sense`, `Cover::share` of it (at least one, never all)
//!   stand guard on the side the slime is on, each on its own bearing of a
//!   small arc at the work's edge: the squad's front toward the slime,
//!   still one body with the workers. With no slime near, everyone works.
//! - Guards used to stand out from the work, up to 8 cells past its edge
//!   where the slime under the post was thin (`out` 8, `thin` 4 walkers
//!   within 2 cells; #276). Reverted 2026-11-15 after the owner's "It
//!   doesn't seem to work well": on shape C a thin film reaches far round
//!   the body, so the posts stood alone in it, out of the squad, and the
//!   guards were bitten and died while the workers were bitten as often as
//!   with no guards. Drain by hand, 8 soldiers, 60 s, seeds 1-4 (Fork): with
//!   guards out, health at 60 s fell below guards off on 4 of 4 seeds and 5
//!   soldiers died (none off); guards at the edge lost 1 and killed more
//!   slime than guards off on 4 of 4 (summed health at 60 s: off 2462,
//!   edge 2220, out 1781 of 3200; Reports/guards-2026-11-15).
//! - The workers' rate rises so the squad's whole rate falls only by
//!   `CoverRules::slow` times the defenders' share (`work_boost`): 3 of 8
//!   on guard cost 22.5%, not 37.5%. (0.2 first, 7.5%: "seems too small to
//!   even notice a difference", owner 15:49Z.)
//! - A worker fires `CoverRules::work_fire` as often as it would.
//!
//! "Units should be able to heal but it cost energy, and requires units to
//! be idle for bit, maybe only near a captured crystal" (`heal`). Healing is
//! an order (`Cmd::Heal`, owner 2026-11-15 07:41Z: "Healing should be a
//! specific action"; it was automatic in #276): the squad stops, and each
//! member that has stood still, unbitten and not firing for `HealRules::wait`
//! seconds heals, paying `HealRules::cost` food a point from the bank, the
//! energy that builds: a full heal in 10 s by a crystal the player holds
//! (`Game::guarded`, the no-larder game's), in a minute anywhere else
//! (owner 15:49Z: "anywhere at a slower rate ... 10 Seconds near a held
//! crystal (for a full heal); a full minute away").

use super::*;

/// How many of a working squad stand guard (`Cmd::Cover`; the owner's "never
/// defenders or more defenders than default").
#[derive(Clone, Copy, Debug, PartialEq, Eq, Default)]
pub enum Cover {
    Never,
    #[default]
    Some,
    More,
}

impl Cover {
    pub const ALL: [Cover; 3] = [Cover::Never, Cover::Some, Cover::More];

    pub fn name(self) -> &'static str {
        match self {
            Cover::Never => "none",
            Cover::Some => "some",
            Cover::More => "more",
        }
    }

    pub fn parse(s: &str) -> Option<Cover> {
        Cover::ALL.into_iter().find(|c| c.name() == s)
    }

    pub fn next(self) -> Cover {
        match self {
            Cover::Never => Cover::Some,
            Cover::Some => Cover::More,
            Cover::More => Cover::Never,
        }
    }

    /// The share of a working squad on guard while slime is near.
    pub fn share(self, r: &CoverRules) -> f32 {
        match self {
            Cover::Never => 0.0,
            Cover::Some => r.some,
            Cover::More => r.more,
        }
    }
}

#[derive(Clone, Copy, Debug, PartialEq)]
pub struct CoverRules {
    /// The share on guard for `Cover::Some` and `Cover::More`.
    pub some: f32,
    pub more: f32,
    /// Dish cells round the work looked at for slime.
    pub sense: f32,
    /// Walkers within `sense` that call defenders out (fewer: everyone works).
    pub near: u32,
    /// The squad's whole work rate falls by this times the defenders' share.
    pub slow: f32,
    /// A worker fires this often as it would (0.5: half as often).
    pub work_fire: f32,
}

impl Default for CoverRules {
    fn default() -> Self {
        CoverRules {
            some: 1.0 / 3.0,
            more: 0.5,
            sense: 30.0,
            near: 20,
            slow: 0.6,
            work_fire: 0.5,
        }
    }
}

#[derive(Clone, Copy, Debug, PartialEq)]
pub struct HealRules {
    /// Health a second while resting by a held crystal (a soldier has
    /// `Rules::hp`, 100: 10 a second is a full heal in 10 s); 0: off.
    pub rate: f32,
    /// Health a second while resting anywhere else (1.67: a full heal in a
    /// minute).
    pub away: f32,
    /// Game seconds still, unbitten and not firing before it starts (3 while
    /// it was automatic; 1 once it was an order).
    pub wait: f32,
    /// Food from the bank a point of health.
    pub cost: f32,
    /// Dish cells past a held crystal's rim that count as by it.
    pub near: f32,
}

impl Default for HealRules {
    fn default() -> Self {
        HealRules {
            rate: 10.0,
            away: 100.0 / 60.0,
            wait: 1.0,
            cost: 0.25,
            near: 10.0,
        }
    }
}

/// The mould's walkers on a coarse grid, read once (`World::mould_grid`),
/// summed over discs without going back to the world each time.
struct Walkers {
    grid: Vec<u32>,
    gw: usize,
    gh: usize,
    bin: f32,
}

impl Walkers {
    fn within(&self, at: (f32, f32), r: f32) -> u32 {
        let (cx, cy) = (at.0 / self.bin, at.1 / self.bin);
        let rr = r / self.bin;
        let (x0, x1) = (
            (cx - rr).floor().max(0.0) as usize,
            ((cx + rr).ceil() as usize).min(self.gw.saturating_sub(1)),
        );
        let (y0, y1) = (
            (cy - rr).floor().max(0.0) as usize,
            ((cy + rr).ceil() as usize).min(self.gh.saturating_sub(1)),
        );
        let mut n = 0;
        for y in y0..=y1 {
            for x in x0..=x1 {
                if (x as f32 + 0.5 - cx).hypot(y as f32 + 0.5 - cy) <= rr {
                    n += self.grid[y * self.gw + x];
                }
            }
        }
        n
    }
}

impl<W: World> Game<W> {
    fn walkers(&self) -> Option<Walkers> {
        let bin = (self.s() as usize).max(1);
        let grid = self.world.mould_grid(bin)?;
        let (w, h) = self.world.size();
        Some(Walkers {
            grid,
            gw: w.div_ceil(bin),
            gh: h.div_ceil(bin),
            bin: bin as f32,
        })
    }

    /// Where squad `q`'s work is: the crystal or building its workers are
    /// at, fine cells.
    fn work_at(&self, q: usize) -> Option<(f32, f32)> {
        match self.squads[q].work? {
            Order::Clear(x, y) => Some(self.crystal_at((x, y)).map_or((x, y), |c| c.1)),
            Order::Build(b) => self.buildings.get(b).map(|b| b.at),
            _ => None,
        }
    }

    /// Whether soldier `k` is working (draining or raising) for its squad.
    pub(super) fn working(&self, k: usize) -> bool {
        let s = &self.soldiers[k];
        !s.cover && matches!(s.order, Order::Clear(..) | Order::Build(_))
    }

    /// Once a game second: each working squad's defenders, called out to
    /// face nearby slime or sent back to work. A squad most of whose
    /// members took up the same work on their own (auto-harvest, a builder
    /// taking a ghost) works it as if it had been ordered to.
    pub(super) fn cover_turn(&mut self) {
        if !self.rules.squads {
            return;
        }
        // A healing squad whose members are all whole is done.
        for q in self.live_squads() {
            if self.squads[q].heal && self.members(q).iter().all(|&k| self.soldiers[k].hp >= self.rules.hp) {
                self.squads[q].heal = false;
            }
        }
        for q in self.live_squads() {
            let sq = self.squads[q];
            if sq.work.is_some() || sq.dest.is_some() || sq.steer != (0.0, 0.0) {
                continue;
            }
            let m = self.members(q);
            let o = self.soldiers[m[0]].order;
            if matches!(o, Order::Clear(..) | Order::Build(_))
                && 2 * m.iter().filter(|&&k| self.soldiers[k].order == o).count() > m.len()
            {
                self.squads[q].work = Some(o);
            }
        }
        let busy: Vec<usize> = self
            .live_squads()
            .into_iter()
            .filter(|&q| self.squads[q].work.is_some())
            .collect();
        if busy.is_empty() {
            return;
        }
        let walkers = self.walkers();
        for q in busy {
            self.cover_one(q, &walkers);
        }
    }

    /// Squad `q`'s defenders now (a work order, a change of `Cover`).
    pub(super) fn cover_squad(&mut self, q: usize) {
        if self.rules.squads {
            let walkers = self.walkers();
            self.cover_one(q, &walkers);
        }
    }

    fn cover_one(&mut self, q: usize, walkers: &Option<Walkers>) {
        let r = self.rules.cover;
        let sc = self.s();
        let within = |g: &Self, at: (f32, f32), rad: f32| match walkers {
            Some(w) => w.within(at, rad),
            None => g.world.walkers_in(at, rad),
        };
        {
            let Some(work) = self.squads[q].work else { return };
            let m = self.members(q);
            // Done: nobody on the work any more (the crystal ran dry, the
            // building stands, or the player sent them elsewhere). The
            // defenders come in, and the squad starts what it has queued.
            if !m
                .iter()
                .any(|&k| self.soldiers[k].order == work && !self.soldiers[k].cover)
            {
                self.squads[q].work = None;
                let mid = self.middle(q);
                for &k in &m {
                    if self.soldiers[k].cover {
                        self.soldiers[k].cover = false;
                        self.soldiers[k].order = mid.map_or(Order::Hold, |p| Order::Move(p.0, p.1));
                    }
                }
                self.squad_next(q);
                return;
            }
            let Some(at) = self.work_at(q) else { return };
            let near = within(self, at, r.sense * sc);
            let share = self.squads[q].cover.share(&r);
            let n = m.len();
            // Not till the squad is there: guards called out at the start of
            // the walk went by attack-move, stopped at every slime they saw
            // on the way, and held up the workers with them (the film's
            // first cut, Fork seed 1: 5 of 8 never reached the crystal).
            let there = self
                .middle(q)
                .is_some_and(|c| (c.0 - at.0).hypot(c.1 - at.1) <= r.sense * sc);
            // Guards already out stay till the slime is half as near, so a
            // count about the threshold doesn't send them out and back each
            // second (seen in the film: 0, 1, 0 guards at 47-50 s).
            let out = m.iter().any(|&k| self.soldiers[k].cover);
            let call = if out { r.near / 2 } else { r.near };
            let want = if near < call || n < 2 || !there {
                0
            } else {
                ((n as f32 * share).round() as usize).clamp(usize::from(share > 0.0), n - 1)
            };
            // Which way the slime is: the walkers in twelve bearings at half
            // the sensing range, summed as a vector.
            let mut v = (0.0f32, 0.0f32);
            for i in 0..12 {
                let a = i as f32 * std::f32::consts::TAU / 12.0;
                let (dx, dy) = (a.cos(), a.sin());
                let p = (at.0 + dx * r.sense * 0.5 * sc, at.1 + dy * r.sense * 0.5 * sc);
                let w = within(self, p, r.sense * 0.25 * sc) as f32;
                v = (v.0 + dx * w, v.1 + dy * w);
            }
            let base = v.1.atan2(v.0);
            // Keep the defenders already out (no reshuffle), call the ones
            // nearest the slime's side first.
            let mut on: Vec<usize> = m.iter().copied().filter(|&k| self.soldiers[k].cover).collect();
            let mut off: Vec<usize> = m
                .iter()
                .copied()
                .filter(|&k| !self.soldiers[k].cover && self.soldiers[k].order == work)
                .collect();
            let toward = |g: &Self, k: usize| {
                let s = &g.soldiers[k];
                -((s.x - at.0) * base.cos() + (s.y - at.1) * base.sin())
            };
            off.sort_by(|&a, &b| toward(self, a).total_cmp(&toward(self, b)).then(a.cmp(&b)));
            while on.len() < want && !off.is_empty() {
                on.push(off.remove(0));
            }
            while on.len() > want {
                let k = on.pop().unwrap();
                self.soldiers[k].cover = false;
                self.soldiers[k].order = work;
            }
            on.sort_unstable();
            let g = on.len();
            let rim = match work {
                Order::Clear(..) => self.crystal_at(at).map_or(0.0, |c| self.world.crystal_r(c.0)) + RIM * sc,
                _ => 2.0 * sc,
            };
            for (i, &k) in on.iter().enumerate() {
                let a = base + (i as f32 - (g as f32 - 1.0) / 2.0) * 0.45;
                // At the work's edge on its bearing (see the module notes:
                // posts out in the slime's film cost soldiers for nothing).
                let p = (at.0 + a.cos() * rim, at.1 + a.sin() * rim);
                let p = if self.standable(p.0, p.1) && self.solid_at(p).is_none() {
                    p
                } else {
                    self.nearest_standable(p, 6.0 * sc)
                        .unwrap_or((self.soldiers[k].x, self.soldiers[k].y))
                };
                // Walks to its post, then stands and fights there (an
                // attack-move to it stopped at the first slime in reach).
                let s = &mut self.soldiers[k];
                let far = (s.x - p.0).hypot(s.y - p.1) > 2.0 * sc;
                s.cover = true;
                s.order = if far {
                    Order::Move(p.0, p.1)
                } else {
                    Order::Fight(p.0, p.1)
                };
                s.homing = false;
            }
        }
    }

    /// The work rate of soldier `k` against one alone: a working squad's
    /// workers make up most of what its defenders would have done.
    pub(super) fn work_boost(&self, k: usize) -> f32 {
        if !self.rules.squads {
            return 1.0;
        }
        let q = self.soldiers[k].squad as usize;
        let m = self.members(q);
        let guards = m.iter().filter(|&&j| self.soldiers[j].cover).count();
        if guards == 0 {
            return 1.0;
        }
        let n = m.len() as f32;
        let f = guards as f32 / n;
        n * (1.0 - self.rules.cover.slow * f) / (n - guards as f32).max(1.0)
    }

    /// Healing, every step (`HealRules`): still, unbitten and not firing
    /// for `wait` seconds by a crystal the player holds.
    pub(super) fn heal(&mut self, k: usize, moved: f32, fired: bool, dt: f32) {
        let h = self.rules.heal;
        let s = self.soldiers[k];
        // Only a squad told to heal (`Cmd::Heal`; owner 2026-11-15 07:41Z:
        // "Healing should be a specific action"). From #276 to then any
        // soldier still, unbitten and not firing for `wait` seconds healed
        // on its own, harvesters and builders at work too, spending food
        // the player never chose to spend.
        if !self.rules.squads || !self.squads[s.squad as usize].heal {
            self.soldiers[k].rest = 0.0;
            return;
        }
        if moved > 0.05 * self.s() || s.touching > 0 || fired {
            self.soldiers[k].rest = 0.0;
            return;
        }
        self.soldiers[k].rest = s.rest + dt;
        if s.rest + dt < h.wait || s.hp >= self.rules.hp || self.bank <= 0.0 {
            return;
        }
        let sc = self.s();
        let by = self.guarded.iter().any(|&i| {
            let (c, _) = self.world.crystals()[i];
            (s.x - c.0).hypot(s.y - c.1) <= self.world.crystal_r(i) + h.near * sc
        });
        let rate = if by { h.rate } else { h.away };
        if rate <= 0.0 {
            return;
        }
        let gain = (rate * dt).min(self.rules.hp - s.hp).min(self.bank / h.cost.max(1e-6));
        self.soldiers[k].hp += gain;
        self.bank -= gain * h.cost;
        self.spent += gain * h.cost;
        self.healed += gain;
    }
}
