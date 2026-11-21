//! **Boss cores that re-form, and beaten raids' cores** (2026-11-20, the
//! helper thread "Build boss cores for shop money", beside Starfall,
//! `starfall.rs`). The design (the thread "Brainstorm how battles earn
//! shop money", round 2, which the owner picked 19:06Z: "I like starfall
//! with boss cores"): "a broken mini-boss or a beaten raid drops a glowing
//! core for 15 s. Walkers stream back to it, and if they get there first
//! the boss re-forms."
//!
//! `starfall.rs` drops a core where a mini-boss broke, worth more the
//! stronger it was, and soldiers by it drain it into shop points. Here:
//!
//! - **It runs home.** A boss breaks where the guns reach, and a core left
//!   there was free money: on the first films (planet 2, run 1795106014,
//!   a wanderer broken 16 units from a held squad) 0 to 4 walkers lived to
//!   reach it in its 15 s. So it slides (`core_drift`, 8 dish cells a
//!   second) to the thickest slime within `core_drift_r` of where it
//!   dropped, and the squad must reach into the slime for it. The core is
//!   game state moving, drawn where it is, not a painted effect.
//! - **Walkers stream back to it.** A core smells as ore does and is a lure
//!   where it goes (`LureMark`, the GPU world's pull), so the slime round it
//!   leans in while the squad runs out to it.
//! - **If they get there first the boss re-forms.** Once walkers have eaten
//!   `StarRules::core_reform` of a mini-boss's core, and more than the
//!   squad took, the boss forms again on it (`BossCause::Core`), of the
//!   walkers there, its kind and grade as before, and the core is gone. It
//!   carries what the core had left: broken again, its core is worth that
//!   and no more, so a boss cannot be farmed by breaking it over and over.
//!   Too few came back and it fizzles, the core lost to the slime.
//! - **A beaten raid drops a core.** A march shot apart on its way, or one
//!   that met the squad and lost `raid_beat` of its members within
//!   `raid_window` seconds (the walkers the game killed, a whole-game
//!   count, so a fight elsewhere in that window counts too), drops a core
//!   where it was. It does not re-form (a raid is no body once it pours);
//!   walkers who eat half of it feed the slime's next surge.
//! - **The planet's pot.** Fresh cores hold at most `StarRules::core_pot`
//!   in all, so waiting for more bosses and raids pays nothing more
//!   (the owner's 13:49Z objection to anything that can be farmed).
//!
//! Re-forming takes a lump (`big::LUMPS`, shared by mini-bosses, marches,
//! fruits and shells); the broken boss's own, if it is still fading, is
//! taken back for it.

use super::*;
use crate::mould::big::Lump;

/// The cores' state (`Game::cores`).
#[derive(Debug, Default)]
pub struct Cores {
    /// Fresh core worth dropped this planet (`StarRules::core_pot`).
    pub pot: f32,
    /// Mini-bosses formed again from their cores, and those that then
    /// fizzled.
    pub reformed: u32,
    /// Each re-formed boss (its birth tick and lump) and what it carries.
    carry: Vec<(u64, usize, f32)>,
    /// Raids beaten and the cores they dropped.
    pub raids_beaten: u32,
    /// Marches that met the squad, waiting on `raid_window`; how many of
    /// `Raids::ends` were looked at.
    meeting: Vec<MarchEnd>,
    ends_seen: usize,
}

impl Cores {
    /// What a re-formed boss `b` carries, taken (None for a boss that
    /// never re-formed).
    pub(super) fn carried(&mut self, b: &Boss) -> Option<f32> {
        let i = self.carry.iter().position(|c| c.0 == b.born && c.1 == b.lump)?;
        Some(self.carry.swap_remove(i).2)
    }
}

impl<W: World> Game<W> {
    /// The raids beaten since the last look: (where, worth). Once a game
    /// second, from `time_starfall`.
    pub(super) fn raid_cores(&mut self) -> Vec<((f32, f32), f32)> {
        let r = self.rules.star;
        let now = self.world.ticks();
        let window = (r.raid_window.max(0.0) * self.rules.tps as f32).round() as u64;
        let ends = &self.raids.ends;
        let new: Vec<MarchEnd> = ends[self.cores.ends_seen.min(ends.len())..].to_vec();
        self.cores.ends_seen = ends.len();
        let mut beaten = Vec::new();
        for e in new {
            match e.how {
                MarchHow::Broken => beaten.push(e),
                MarchHow::Met => self.cores.meeting.push(e),
            }
        }
        // A march that met the squad is beaten once the guns have killed
        // `raid_beat` of what it brought, inside the window.
        let killed = self.killed;
        self.cores.meeting.retain(|e| {
            let won = killed.saturating_sub(e.killed) as f32 >= r.raid_beat * e.members.max(1) as f32;
            if won {
                beaten.push(*e);
            }
            !won && now < e.tick + window
        });
        self.cores.raids_beaten += beaten.len() as u32;
        beaten.iter().map(|e| (e.at, r.core_worth + r.core_grade * (e.first.max(e.members) as f32 / r.raid_full.max(1.0)).min(1.0))).collect()
    }

    /// Once a game second, before `time_starfall` (which clears a core
    /// eaten to nothing): each mini-boss's core the walkers have taken back
    /// re-forms its boss.
    pub(super) fn time_cores(&mut self) {
        let r = self.rules.star;
        if !r.on {
            return;
        }
        let mut i = 0;
        while i < self.starfall.live.len() {
            let d = self.starfall.live[i];
            let CoreFrom::Boss { kind, grade } = d.from else {
                i += 1;
                continue;
            };
            if d.eaten < r.core_reform * d.worth || d.eaten <= d.taken || !self.reform_boss(d.at, kind, grade, d.worth - d.taken) {
                i += 1;
                continue;
            }
            let d = self.starfall.live.remove(i);
            self.drop_gone(&d);
        }
    }

    /// Where a core dropped at `at` (fine cells) runs to: the nearest
    /// square within `core_drift_r` whose 5 x 5 squares round it hold at
    /// least half as much slime as the fullest; `at` itself with no slime
    /// there or no grid.
    fn core_home(&self, at: (f32, f32)) -> (f32, f32) {
        let r = self.rules.star;
        let sc = self.world.scale();
        let (w, h) = self.world.size();
        let bin = (sc as usize).max(1);
        let Some(g) = self.world.mould_grid(bin) else { return at };
        let (gw, gh) = (w.div_ceil(bin), h.div_ceil(bin));
        let reach = r.core_drift_r * sc;
        let (cx, cy) = (at.0 / bin as f32, at.1 / bin as f32);
        let span = (reach / bin as f32).ceil() as i64;
        let mut squares = Vec::new();
        for y in (cy as i64 - span).max(0)..(cy as i64 + span + 1).min(gh as i64) {
            for x in (cx as i64 - span).max(0)..(cx as i64 + span + 1).min(gw as i64) {
                let p = ((x as f32 + 0.5) * bin as f32, (y as f32 + 0.5) * bin as f32);
                let d = (p.0 - at.0).hypot(p.1 - at.1);
                // Thick slime, not a stream: the walkers in the 5 x 5
                // squares round it.
                let mut n = 0;
                for yy in (y - 2).max(0)..(y + 3).min(gh as i64) {
                    for xx in (x - 2).max(0)..(x + 3).min(gw as i64) {
                        n += g.get((yy * gw as i64 + xx) as usize).copied().unwrap_or(0);
                    }
                }
                if d <= reach && n > 0 && self.world.open(p.0, p.1) {
                    squares.push((p, d, n));
                }
            }
        }
        let most = squares.iter().map(|s| s.2).max().unwrap_or(0);
        squares.into_iter().filter(|s| 2 * s.2 >= most).min_by(|a, b| a.1.total_cmp(&b.1)).map_or(at, |s| s.0)
    }

    /// Core `i` slides `dt` seconds toward its home, stopping at rock.
    pub(super) fn drift_core(&mut self, i: usize, dt: f32) {
        let r = self.rules.star;
        let sc = self.world.scale();
        let Some(d) = self.starfall.live.get(i).copied() else { return };
        if d.kind != DropKind::Core {
            return;
        }
        // Its home, chosen on its first frame, and the slime's lure there.
        let home = match d.home {
            Some(h) => h,
            None => {
                let h = self.core_home(d.at);
                self.starfall.live[i].home = Some(h);
                self.lure_marks.push(LureMark { at: h, until: d.ends });
                self.starfall.live[i].lure = Some(h);
                h
            }
        };
        let (dx, dy) = (home.0 - d.at.0, home.1 - d.at.1);
        let gap = dx.hypot(dy);
        if gap < 1e-3 {
            return;
        }
        let step = (r.core_drift * sc * dt).min(gap);
        let next = (d.at.0 + dx / gap * step, d.at.1 + dy / gap * step);
        if self.world.open(next.0, next.1) {
            self.starfall.live[i].at = next;
        } else {
            self.starfall.live[i].home = Some(d.at);
        }
    }

    /// Forms boss `kind` of `grade` again round `at` (fine cells), of the
    /// walkers there, carrying `carry` points; false with no lump to be
    /// had.
    fn reform_boss(&mut self, at: (f32, f32), kind: BossKind, grade: f32, carry: f32) -> bool {
        // Its own lump, if it is still fading there, is taken back.
        let sc = self.world.scale();
        let rb = self.rules.boss;
        let reach = rb.reach;
        if let Some(k) = self.bosses.fading.iter().position(|f| (f.0.centre.0 - at.0).hypot(f.0.centre.1 - at.1) < 2.0 * reach * sc) {
            let (b, _) = self.bosses.fading.swap_remove(k);
            self.world.form_lump(b.lump, None);
        }
        if !self.form_boss(at, reach, BossCause::Core) {
            return false;
        }
        let Some(b) = self.bosses.live.last_mut() else { return false };
        (b.kind, b.grade) = (kind, grade);
        let (lump, born) = (b.lump, b.born);
        let size = self.world.size();
        self.world.set_lump(lump, Lump { aim: raid_cell(at, size), leash: rb.leash_at(rb.clench, grade), armour: rb.armour_at(grade), bite: rb.bite_at(grade), ..Lump::default() });
        self.cores.carry.push((born, lump, carry));
        self.cores.reformed += 1;
        true
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn cored(seed: u64) -> Game<BigDish> {
        let star = StarRules { core_pot: 100.0, ..StarRules::for_depth(2) };
        let boss = BossRules { fade: 3.0, ..BossRules::default() };
        Game::new(race_world(seed, 1, 1_000), Rules { star, boss, ..Rules::default() }, seed)
    }

    fn run_to(g: &mut Game<BigDish>, t: f32) {
        while g.time() < t && g.end.is_none() {
            g.step();
        }
    }

    /// The middle of the slime (fine cells).
    fn slime_centre(g: &Game<BigDish>) -> (f32, f32) {
        let w = &g.world;
        let living: Vec<usize> = (0..w.walkers.len()).filter(|&k| w.lives(k)).collect();
        let (x, y) = living.iter().fold((0.0, 0.0), |a, &k| (a.0 + w.walkers[k].0, a.1 + w.walkers[k].1));
        (x / living.len() as f32, y / living.len() as f32)
    }

    /// The design's rule: a broken boss drops a core; walkers that eat it
    /// first form the same boss again, which carries what was left and,
    /// broken again, drops no more than that.
    #[test]
    fn a_core_the_slime_takes_back_reforms_its_boss() {
        let mut g = cored(3);
        run_to(&mut g, 3.0);
        let at = slime_centre(&g);
        assert!(g.form_boss(at, 6.0, BossCause::Staged));
        g.bosses.live[0].kind = BossKind::Wanderer;
        g.bosses.live[0].grade = 0.5;
        run_to(&mut g, 5.0);
        // Broken: its members under the quorum (their count set by hand, as
        // the guns would leave it).
        let b = g.bosses.live[0];
        assert!(b.first > 0, "counted");
        g.bosses.ended.push((b, BossEnd::Broken, g.world.ticks()));
        g.bosses.live.clear();
        g.bosses.fading.push((b, g.world.ticks() + 1_000));
        let t = g.time();
        run_to(&mut g, t + 1.5);
        let core = *g.starfall.live.iter().find(|d| d.kind == DropKind::Core).expect("a core dropped");
        assert_eq!(core.from, CoreFrom::Boss { kind: BossKind::Wanderer, grade: 0.5 });
        let worth = g.rules.star.core_worth + 0.5 * g.rules.star.core_grade;
        assert!((core.worth - worth).abs() < 1e-3, "worth {}", core.worth);
        assert!(core.home.is_some_and(|h| g.lure_marks.iter().any(|m| m.at == h)), "it runs home, and the slime is drawn there");
        // The squad took a little, the slime most of it.
        let k = g.starfall.live.iter().position(|d| d.kind == DropKind::Core).unwrap();
        (g.starfall.live[k].taken, g.starfall.live[k].eaten, g.starfall.live[k].left) = (5.0, 30.0, worth - 35.0);
        let t = g.time();
        run_to(&mut g, t + 1.5);
        assert!(g.starfall.live.iter().all(|d| d.kind != DropKind::Core), "the core is gone into the boss");
        assert_eq!(g.cores.reformed, 1);
        let nb = *g.bosses.live.last().expect("the boss formed again");
        assert_eq!((nb.cause, nb.kind, nb.grade), (BossCause::Core, BossKind::Wanderer, 0.5));
        assert!(g.bosses.fading.is_empty(), "its fading lump taken back");
        // Broken again: its core holds what it carried, the pot untouched.
        let pot = g.cores.pot;
        g.bosses.ended.push((nb, BossEnd::Broken, g.world.ticks()));
        g.bosses.live.clear();
        let t = g.time();
        run_to(&mut g, t + 1.5);
        let again = g.starfall.live.iter().find(|d| d.kind == DropKind::Core).expect("its core again");
        assert!((again.worth - (worth - 5.0)).abs() < 1e-3, "carried {} of {}", again.worth, worth - 5.0);
        assert_eq!(g.cores.pot, pot, "a re-formed boss's core is no new money");
    }

    /// The squad that drains most of a core keeps its boss broken.
    #[test]
    fn a_core_the_squad_takes_stays_taken() {
        let mut g = cored(5);
        run_to(&mut g, 2.0);
        let at = slime_centre(&g);
        let mut d = Drop::core(at, 60.0, CoreFrom::Boss { kind: BossKind::Hunter, grade: 0.2 }, 0, u64::MAX, 2.0);
        (d.taken, d.eaten, d.left) = (31.0, 29.0, 0.0);
        g.starfall.live.push(d);
        g.time_cores();
        assert_eq!(g.cores.reformed, 0);
        assert!(g.bosses.live.is_empty());
    }

    /// A march shot apart, and one that met the squad and lost half to the
    /// guns, each drop a core; the planet's pot caps them all.
    #[test]
    fn beaten_raids_drop_cores_inside_the_pot() {
        let mut g = cored(7);
        run_to(&mut g, 2.0);
        let now = g.world.ticks();
        let at = slime_centre(&g);
        let killed = g.killed;
        g.raids.ends.push(MarchEnd { how: MarchHow::Broken, at, first: 200, members: 40, tick: now, killed });
        g.raids.ends.push(MarchEnd { how: MarchHow::Met, at: (at.0 + 30.0, at.1), first: 3000, members: 3000, tick: now, killed });
        let t = g.time();
        run_to(&mut g, t + 1.5);
        let raid = |g: &Game<BigDish>| g.starfall.live.iter().filter(|d| d.from == CoreFrom::Raid).count();
        assert_eq!(raid(&g), 1, "the broken march's core");
        let r = g.rules.star;
        assert!((g.starfall.live.iter().find(|d| d.from == CoreFrom::Raid).unwrap().worth - (r.core_worth + r.core_grade * 0.5)).abs() < 1e-3);
        // The squad's guns kill what the meeting march brought.
        g.killed += 1500;
        let t = g.time();
        run_to(&mut g, t + 1.5);
        assert_eq!(g.cores.raids_beaten, 2);
        // 60 then 80, capped at the pot of 100: the second is 40.
        assert!((g.cores.pot - 100.0).abs() < 1e-3, "pot {}", g.cores.pot);
        // Past the pot nothing more drops.
        g.raids.ends.push(MarchEnd { how: MarchHow::Broken, at, first: 200, members: 40, tick: g.world.ticks(), killed: g.killed });
        let n = g.starfall.cores;
        let t = g.time();
        run_to(&mut g, t + 1.5);
        assert_eq!(g.starfall.cores, n, "the pot is spent");
    }
}
