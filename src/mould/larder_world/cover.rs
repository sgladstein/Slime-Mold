//! **A squad at work, and healing** (the squad mode).
//!
//! A squad works when some of its members drain a crystal or raise a
//! building (`Squad::work`, set by `Cmd::SquadOrder`). Once a game second
//! (`cover_turn`) it is checked: once nobody is on the work any more it
//! starts what it has queued.
//!
//! **Lookouts are gone** (owner, 2026-11-21 05:04Z: "remove the feature
//! that some soldiers guard while harvesting"). From 2026-11-14 15:06Z
//! (`Cover`, the card's Lookouts: none, some, more) a share of a working
//! squad with slime within 30 cells stood guard on the slime's side of the
//! work, the workers worked faster to make up most of it (`work_boost`)
//! and fired at half rate (`work_fire`). Now every member works and fires
//! at its full rate; the squad's guns, the fire button and the Guard order
//! defend the work. Guards out past the work's edge were tried before and
//! reverted 2026-11-15 (they stood alone in the slime's film and died;
//! Reports/guards-2026-11-15).
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
        HealRules { rate: 10.0, away: 100.0 / 60.0, wait: 1.0, cost: 0.25, near: 10.0 }
    }
}

impl<W: World> Game<W> {
    /// Once a game second: each working squad that is done (nobody on its
    /// work any more: the crystal ran dry, the building stands, or the
    /// player sent them elsewhere) starts what it has queued. A squad most
    /// of whose members took up the same work on their own (auto-harvest, a
    /// builder taking a ghost) works it as if it had been ordered to.
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
            if matches!(o, Order::Clear(..) | Order::Build(_)) && 2 * m.iter().filter(|&&k| self.soldiers[k].order == o).count() > m.len() {
                self.squads[q].work = Some(o);
            }
        }
        for q in self.live_squads() {
            let Some(work) = self.squads[q].work else { continue };
            if !self.members(q).iter().any(|&k| self.soldiers[k].order == work) {
                self.squads[q].work = None;
                self.squad_next(q);
            }
        }
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
