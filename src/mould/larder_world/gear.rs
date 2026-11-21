//! The squad's gear in the game: its tools (`mould::kit`) and the army's
//! gun cards (`mould::rounds`), each run after every finished step.
//!
//! Until 2026-11-20 the window's `App` held a `Kit` and a `Rounds` beside
//! the game and ran them after each step it saw (`kit_between`, then
//! `Rounds::after_step`), so only a game in the window cast its cards: the
//! library's tests and the census fired the gun's bullets and never cast
//! what they carried. The lane brief's job 6 (Engine and tools) moved both
//! here, so tests, replays and the bots play the game the owner plays.
//!
//! One order changed with the move: a step a command finished
//! (`Game::apply`) ran its kit and rounds after the command, when the
//! window next caught up; now they run as the step finishes, before the
//! command, the same as every other step.

use super::*;
use crate::mould::kit::{KitRules, Tool};
use crate::mould::rounds::RoundRules;
use crate::mould::run::Run;

impl<W: World> Game<W> {
    /// After each finished step: the kit restocks every living soldier at
    /// the larder (`carry_full` to `carry_max`, the rest to `carry_each`)
    /// and sets off its mines, then the gun cards cast on the step's hits
    /// (`Rounds::after_step`).
    pub(super) fn gear_turn(&mut self) {
        if !self.gear {
            return;
        }
        let full = std::mem::take(&mut self.carry_full);
        self.with_kit(|kit, g| {
            for k in 0..g.soldiers.len() {
                let n = if full.contains(&k) { kit.rules.carry_max } else { kit.rules.carry_each };
                if g.soldiers[k].alive && kit.held(k) < n {
                    kit.restock(g, k, n);
                }
            }
            kit.frame(g);
        });
        self.carry_full = full;
        self.with_rounds(|rd, g| rd.after_step(g));
    }

    /// Runs `f` on the game's kit and the game together (`Kit`'s calls take
    /// both). While `f` runs, `self.kit` is a blank stand-in, so `f` reads
    /// the kit only through its first argument.
    pub fn with_kit<R>(&mut self, f: impl FnOnce(&mut Kit, &mut Self) -> R) -> R {
        let mut kit = std::mem::replace(&mut self.kit, Kit::new(KitRules::default()));
        let out = f(&mut kit, self);
        self.kit = kit;
        out
    }

    /// Runs `f` on the game's gun cards and the game together; while it
    /// runs `self.rounds` is a blank stand-in (as `with_kit`).
    pub fn with_rounds<R>(&mut self, f: impl FnOnce(&mut Rounds, &mut Self) -> R) -> R {
        let mut rd = std::mem::take(&mut self.rounds);
        let out = f(&mut rd, self);
        self.rounds = rd;
        out
    }

    /// Uses a tool (`Kit::use_tool`): false when it cannot be used here or
    /// the bank is short.
    pub fn use_tool(&mut self, t: Tool) -> bool {
        self.with_kit(|kit, g| kit.use_tool(g, t))
    }

    /// **The device slot** (`Cmd::Device`; owner 2026-11-21 06:55Z: "One
    /// device slot per squad, on a cooldown", his 11-16 gun-system ruling):
    /// squad `q`, or a squad commanded with it (its link, a drag's pick)
    /// whose slot is ready, sets tool `t` off from a soldier within reach,
    /// free, and its slot waits the tool's cooldown (`KitRules::cooldown`).
    /// `q` goes first, then the others in id order, so a pick of three
    /// squads throws three in a row. Why: the squad mode draws no larder,
    /// so a soldier took a tool only near the landing spot and threw one a
    /// planet (controls review, 2026-11-21).
    pub fn device(&mut self, q: usize, t: Tool) -> bool {
        if q >= self.squads.len() || matches!(t, Tool::Smell(..)) {
            return false;
        }
        let now = self.world.ticks();
        let mut order = self.linked(q);
        order.sort_by_key(|&r| r != q);
        for r in order {
            if self.squads[r].device_at > now || self.members(r).is_empty() {
                continue;
            }
            if self.with_kit(|kit, g| kit.device(g, r, t)) {
                let secs = self.kit.rules.cooldown(t);
                self.squads[r].device_at = now + ((secs * self.rules.tps as f32).round() as u64).max(1);
                self.squads[r].device_secs = secs;
                return true;
            }
        }
        false
    }

    /// Seconds until squad `q`'s device slot is ready (0: ready).
    pub fn device_wait(&self, q: usize) -> f32 {
        let at = self.squads.get(q).map_or(0, |s| s.device_at);
        at.saturating_sub(self.world.ticks()) as f32 / self.rules.tps as f32
    }

    /// The device slot readiest among squad `q` and the squads commanded
    /// with it: (seconds left, the cooldown it was set to); (0, _) when one
    /// is ready.
    pub fn device_ready_in(&self, q: usize) -> (f32, f32) {
        self.linked(q).into_iter().map(|r| (self.device_wait(r), self.squads[r].device_secs)).min_by(|a, b| a.0.total_cmp(&b.0)).unwrap_or((0.0, 0.0))
    }

    /// Kits a planet out from its run, or the range's armory: the run's
    /// tool prices (`Run::kit`), the army's round of the cards it bought
    /// with its budget (`Run::squad_cards`, `Run::budget`; the rest of
    /// `rr` as given, for `set=` keys) and its turret deck
    /// (`Run::turret_cards`). The gun itself is the caller's to arm
    /// (`Cmd::Arm`): a run lands with `Run::armed`, the range with
    /// `Run::carried`, and a `gun=` key overrides both.
    pub fn kit_out(&mut self, r: &Run, rr: RoundRules) {
        let mut kit = KitRules::default();
        r.kit(&mut kit);
        self.kit = Kit::new(kit);
        self.rounds = Rounds::new(r.squad_cards(), RoundRules { budget: r.budget(), ..rr });
        let t = r.turret_cards();
        if !t.is_empty() {
            self.apply(Cmd::Load(TURRET_DECK, Some(Deck::new(&t))));
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::mould::rounds::parse;

    /// A game with no window around it restocks its soldiers' tools and
    /// casts its gun's cards on its hits (job 6, 11-20: before, only the
    /// window's `App` ran either, so this game cast nothing).
    #[test]
    fn a_game_without_the_window_casts_its_cards_and_restocks_its_tools() {
        let rules = Rules { cap: 30.0, ..Rules::default() };
        let mut g = Game::new(race_world(3, 1, 2_000), rules, 3);
        g.bank = 1.0e9;
        g.rounds = Rounds::new(parse("blast").unwrap(), RoundRules::default());
        for k in 0..g.soldiers.len() {
            assert!(g.apply(Cmd::Order(k, Order::Attack)));
        }
        let mut stocked = false;
        while g.end.is_none() && g.time() < 30.0 && g.rounds.cast.iter().sum::<u32>() == 0 {
            g.step();
            stocked |= (0..g.soldiers.len()).any(|k| g.kit.held(k) > 0);
        }
        assert!(stocked, "a soldier took a tool at the larder");
        assert!(g.rounds.cast.iter().sum::<u32>() > 0, "no blast cast in {:.0} s ({} shots, {} hits)", g.time(), g.shots, g.hits);
    }
}
