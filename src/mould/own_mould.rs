//! **The player's own mould: gardeners** (ROADMAP L-O; the owner's card,
//! 2026-11-12 04:45Z: "Gardeners: soldiers grow it"). The player owns a
//! second species in the Larder world. The soldiers grow it: they lay its
//! roads, feed it charge from the bank and cut it, and it fights the enemy
//! mould by conversion (`BigParams::species_convert`, weighed by presence
//! with `species_convert_per`). Proposal and probe:
//! `Reports/player-mould-2026-11-12.md`.
//!
//! **Shape** (agreed with the game integration thread, 04:40Z): everything
//! is in this file. `larder_world` has one hook, `World::tend`, which the
//! game calls once a frame with the soldiers and the bank. `Garden` wraps
//! the CPU big dish and is a `World` of its own, so `Game<Garden>` plays
//! the same rules and today's `Game<BigDish>` is unchanged (its `tend` is
//! the trait's no-op). To the game, "the mould" is the enemy alone:
//! `Garden::living`, the soldiers' touch, their aim, their shots and the
//! larder's theft all count the enemy's walkers only. The player's mould
//! is the gardeners' to handle through `Task`s.
//!
//! **Tasks**, one a soldier, done at the soldier's cell each frame (a disc
//! of `Tend::r` dish cells):
//! - `Road`: lays the player species' trail (and the shared trail), so the
//!   player's walkers follow where the soldier walks.
//! - `Feed`: spends `Tend::feed` food a game second from the bank and
//!   spreads it on the disc as crystal charge (`Rules::charge` a food), so
//!   the walkers there eat it, the enemy's too if they reach it.
//! - `Cut`: kills the player's walkers in the disc and clears their trail:
//!   a knife the soldier carries, to sever or shape the player's mould.
//!
//! **Not yet** (each on the wiki page's Open list): the GPU world (the
//! engine runs the species already; the squad frame's species test is the
//! GPU thread's to port, after the toolkit's rows).
//!
//! **Each side's walkers** (2026-11-12, the tech lead's review of #119):
//! each mould may hold at most half the slots (`BigParams::species_cap`),
//! so a big player mould cannot win a fight by filling the enemy's birth
//! slots (a sown mould reached 32,000 of 50,000 in the shared pool). A
//! taken enemy core sets the enemy's cap to 0: its births stop, the
//! player's go on.

use super::kit::KitRules;
use super::big::{BigDish, BigParams, SpeciesSet, LIFE_DEAD, LIFE_SPORE};
use super::larder_world::{race_params, race_world, Soldier, World};
use super::map::Map;
use super::squad::{self, cell_of, disc, path, r2, Frame, FrameOut, ShotIn, ShotOut, UnitIn, UnitOut, BIN, PATH_MAX, R_MAX, SHOTS_MAX, SHOT_ALARM, SHOT_ALARM_R, UNITS_MAX};

/// The player's species id: the last of the engine's four slots
/// (`gpu::MAX_SPECIES`), so new enemy tiers fill 1 and 2 without moving it
/// (proposed to the integration thread, 2026-11-12 05:55Z). The enemy's
/// main mould stays species 0.
pub const OWN: u8 = 3;
/// Species slots the garden world runs (0..=OWN).
pub const SPECIES: u8 = OWN + 1;
const NONE: u32 = u32::MAX;

/// What a soldier does to the player's mould.
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub enum Task {
    #[default]
    None,
    Road,
    Feed,
    Cut,
}

/// The gardeners' numbers.
#[derive(Clone, Copy, Debug)]
pub struct Tend {
    /// The disc a task covers, dish cells.
    pub r: f32,
    /// Trail a Road lays on each cell of its disc, a game second (the
    /// game's knee is 6; 30 a second is about five knees, a road the
    /// walkers read at once).
    pub road: f32,
    /// Food a Feed spends a game second, spread on its disc as charge.
    pub feed: f32,
    /// Charge a food is (`Rules::charge`).
    pub charge: f32,
    /// Game ticks a second and a frame (`Rules::tps`, `frame_ticks`), for
    /// the rates.
    pub tps: u32,
}

impl Default for Tend {
    fn default() -> Self {
        Tend { r: 3.0, road: 30.0, feed: 20.0, charge: 64.0, tps: 15 }
    }
}

/// The CPU race world with a second mould for the player, and the
/// gardeners' tasks. A `World`, so `larder_world::Game` plays on it.
#[derive(Clone)]
pub struct Garden {
    pub dish: BigDish,
    pub tasks: Vec<Task>,
    pub tend: Tend,
    /// Totals over the game: what the gardeners did and what the player's
    /// mould ate (crystal charge, its walkers' `took`).
    pub roads: u64,
    pub fed: f32,
    pub cut: u32,
    pub own_ate: f64,
    pub enemy_ate: f64,
    /// Walkers converted to the player's mould and to the enemy (alive
    /// before and after a tick, species changed), over the game.
    pub turned_own: u64,
    pub turned_enemy: u64,
}

/// The race world of `seed` with the enemy on its home crystal and the
/// player's mould on the crystal farthest from it (where the larder lands),
/// `walkers` each; both the race mould's settings unless `sets` names
/// them (enemy, player). Conversion over a square of 4 by presence.
pub fn garden_world(seed: u64, scale: usize, walkers: usize, sets: Option<(SpeciesSet, SpeciesSet)>) -> Garden {
    let mut m = Map::of_dish(&race_world(seed, scale, walkers));
    let home = m.items.crystals[0];
    let far = *m.items.crystals.iter().max_by(|a, b| (a.x - home.x).hypot(a.y - home.y).total_cmp(&(b.x - home.x).hypot(b.y - home.y))).expect("a crystal");
    m.items.moulds = vec![(home.x, home.y), (far.x, far.y)];
    m.items.landing = Some((far.x, far.y));
    let mut p: BigParams = race_params(seed, scale, 2 * walkers);
    p.species = SPECIES;
    p.species_avoid = 0.0;
    p.species_convert = 1.2;
    p.species_convert_r = 4;
    p.species_convert_per = true;
    // Half the slots a side; the unused species 1 and 2 hold none.
    let half = p.capacity.max(p.walkers) / 2;
    p.species_cap = (0..SPECIES).map(|s| if s == 0 || s == OWN { half } else { 0 }).collect();
    let base = SpeciesSet::from_params(&p);
    let (e, o) = sets.unwrap_or((base, base));
    p.species_sets = (0..SPECIES).map(|s| if s == OWN { o } else if s == 0 { e } else { base }).collect();
    let mut dish = BigDish::with_map(p, &m);
    // The map's starts share the walkers in order (`place_walkers`): the
    // first home's are the enemy's, the second's the player's.
    let n = dish.walkers.len();
    for k in 0..n {
        dish.spec[k] = if k < walkers { 0 } else { OWN };
    }
    Garden { dish, tasks: Vec::new(), tend: Tend::default(), roads: 0, fed: 0.0, cut: 0, own_ate: 0.0, enemy_ate: 0.0, turned_own: 0, turned_enemy: 0 }
}

impl Garden {
    /// The toolkit's det cord (`kit::Tool::Cord`) on the garden: blasts of
    /// `cord_r` every `cord_every` dish cells from `a` to `b` (fine-cell
    /// points), each killing every walker in it, the player's too, and
    /// leaving its crater, so one cut tool serves both moulds (the
    /// designer's review of #132: Cut's disc of radius 3 moved nothing
    /// against 23,000 cells). Returns (the player's, the enemy's) killed;
    /// the caller pays `cord_price` a dish cell from the bank.
    pub fn cord(&mut self, a: (f32, f32), b: (f32, f32), r: &KitRules, tps: u32) -> (usize, usize) {
        let d = &mut self.dish;
        let s = d.p.scale as f32;
        let before = |d: &BigDish| (0..d.walkers.len()).filter(|&k| d.lives(k)).fold((0usize, 0usize), |(o, e), k| if d.spec[k] == OWN { (o + 1, e) } else { (o, e + 1) });
        let (o0, e0) = before(d);
        d.enable_alarm(r.alarm_keep);
        let len = (a.0 - b.0).hypot(a.1 - b.1);
        let n = (len / (r.cord_every * s)).ceil().max(1.0) as usize;
        let steps = (r.crater_secs * tps as f32 * d.p.substeps as f32).round().min(u16::MAX as f32) as u16;
        for j in 0..=n {
            let f = j as f32 / n as f32;
            let p = (a.0 + (b.0 - a.0) * f, a.1 + (b.1 - a.1) * f);
            d.strike(p.0 / s, p.1 / s, r.cord_r, r.blast_alarm, 2.0 * r.cord_r);
            d.scorch_disc(p.0, p.1, r.cord_r * s, steps);
        }
        let (o1, e1) = before(d);
        (o0.saturating_sub(o1), e0.saturating_sub(e1))
    }

    /// Sets soldier `k`'s task.
    pub fn set_task(&mut self, k: usize, t: Task) {
        if self.tasks.len() <= k {
            self.tasks.resize(k + 1, Task::None);
        }
        self.tasks[k] = t;
    }

    /// Whether walker `k` counts: alive, not a spore or dead.
    fn counts(&self, k: usize) -> bool {
        let d = &self.dish;
        d.lives(k) && (d.life.is_empty() || !matches!(d.life[k], LIFE_SPORE | LIFE_DEAD))
    }

    /// Living walkers of the player's mould and of the enemy.
    pub fn counts_by_side(&self) -> (usize, usize) {
        let (mut own, mut enemy) = (0, 0);
        for k in 0..self.dish.walkers.len() {
            if self.counts(k) {
                if self.dish.spec[k] == OWN {
                    own += 1;
                } else {
                    enemy += 1;
                }
            }
        }
        (own, enemy)
    }

    /// Walkers of the enemy on each fine cell.
    fn enemy_occ(&self) -> Vec<u16> {
        let d = &self.dish;
        let mut occ = vec![0u16; d.w * d.h];
        for k in 0..d.walkers.len() {
            if self.counts(k) && d.spec[k] != OWN {
                let (x, y, _) = d.walkers[k];
                occ[y as usize * d.w + x as usize] += 1;
            }
        }
        occ
    }

    /// The enemy's trail under cell `i`: the shared trail less the player's.
    fn enemy_trail(&self, i: usize) -> f32 {
        self.dish.trail[i] - self.dish.species_trail[OWN as usize][i]
    }

    /// `squad::shots` for the garden: a shot stops at the first cell whose
    /// enemy trail looks like slime, so it flies over the player's mould,
    /// and strikes the enemy's walkers and trail only.
    fn shots(&mut self, shots: &[ShotIn], slime: f32, crater: u16) -> Vec<ShotOut> {
        let (w, h) = (self.dish.w, self.dish.h);
        let shots = &shots[..shots.len().min(SHOTS_MAX)];
        let mut used = 0;
        // Enemy walkers a cell, for the walker-in-a-crater stop.
        let occ = if self.dish.scorch.is_empty() { Vec::new() } else { self.enemy_occ() };
        let hits: Vec<u32> = shots
            .iter()
            .map(|s| {
                let cells = path(w, h, s.from, s.to);
                let cells = &cells[..cells.len().min(PATH_MAX - used)];
                used += cells.len();
                for &i in cells {
                    if !self.dish.agar[i as usize] {
                        return NONE;
                    }
                    // As `squad::shots`: an enemy walker standing in a
                    // crater stops the shot though the crater holds no trail.
                    if self.enemy_trail(i as usize) >= slime || (!occ.is_empty() && self.dish.scorch[i as usize] > 0 && occ[i as usize] > 0) {
                        return i;
                    }
                }
                NONE
            })
            .collect();
        let r2s: Vec<u32> = shots.iter().map(|s| r2(s.r.min(R_MAX))).collect();
        let inside = |c: usize, q: usize| {
            let hit = hits[q];
            if hit == NONE {
                return false;
            }
            let (dx, dy) = ((c % w) as i64 - (hit as usize % w) as i64, (c / w) as i64 - (hit as usize / w) as i64);
            dx * dx + dy * dy <= r2s[q] as i64
        };
        let mut killed = vec![0u32; shots.len()];
        for k in 0..self.dish.walkers.len() {
            if !self.counts(k) || self.dish.spec[k] == OWN {
                continue;
            }
            let (x, y, _) = self.dish.walkers[k];
            let c = y as usize * w + x as usize;
            if let Some(q) = (0..shots.len()).find(|&q| inside(c, q)) {
                self.dish.kill(k);
                killed[q] += 1;
            }
        }
        let n = self.dish.species_trail.len();
        for q in 0..shots.len() {
            if hits[q] == NONE {
                continue;
            }
            let (hx, hy) = ((hits[q] as usize % w) as i64, (hits[q] as usize / w) as i64);
            for c in disc(w, h, hx, hy, r2s[q]) {
                let own = self.dish.species_trail[OWN as usize][c];
                for s in 0..n {
                    if s != OWN as usize {
                        self.dish.species_trail[s][c] = 0.0;
                    }
                }
                self.dish.trail[c] = own;
            }
            // The crater (`Frame::crater`), as `squad::shots` lays it. It
            // fades every species' trail there, the player's too: the
            // ground is burnt for both (shots stop at enemy slime, so it
            // lies on the enemy's side).
            if crater > 0 {
                if self.dish.scorch.is_empty() {
                    self.dish.scorch = vec![0; w * h];
                }
                let cone = self.dish.p.scorch_cone;
                for c in disc(w, h, hx, hy, r2s[q]) {
                    if self.dish.agar[c] {
                        let steps = if cone {
                            let (dx, dy) = ((c % w) as i64 - hx, (c / w) as i64 - hy);
                            let c2 = crater as u32;
                            (c2 - c2 * (dx * dx + dy * dy) as u32 / (r2s[q] + 1)) as u16
                        } else {
                            crater
                        };
                        self.dish.scorch[c] = self.dish.scorch[c].max(steps);
                    }
                }
            }
            if !self.dish.alarm.is_empty() {
                for c in disc(w, h, hx, hy, r2(SHOT_ALARM_R * shots[q].r.min(R_MAX))) {
                    if self.dish.agar[c] {
                        self.dish.alarm[c] += SHOT_ALARM;
                    }
                }
            }
        }
        hits.iter()
            .zip(killed)
            .map(|(&hit, killed)| ShotOut { hit: (hit != NONE).then(|| ((hit as usize % w) as f32 + 0.5, (hit as usize / w) as f32 + 0.5)), killed })
            .collect()
    }

    /// `squad::read` on the enemy's walkers only.
    fn read(&self, units: &[UnitIn]) -> Vec<(u32, Option<usize>, u32)> {
        let d = &self.dish;
        let (w, h) = (d.w, d.h);
        let occ = self.enemy_occ();
        let (nbw, nbh) = (w.div_ceil(BIN) as i64, h.div_ceil(BIN) as i64);
        units[..units.len().min(UNITS_MAX)]
            .iter()
            .map(|u| {
                let (cx, cy) = cell_of(w, h, u.x, u.y);
                let touching = disc(w, h, cx, cy, r2(u.touch)).map(|i| occ[i] as u32).sum();
                let s2 = r2(u.sight) as i64;
                let r = squad::isqrt(r2(u.sight)) + 2;
                let (by0, by1) = ((cy - r).max(0) / BIN as i64, ((cy + r) / BIN as i64).min(nbh - 1));
                let (bx0, bx1) = ((cx - r).max(0) / BIN as i64, ((cx + r) / BIN as i64).min(nbw - 1));
                let mut best = (0u32, None);
                for by in by0..=by1 {
                    for bx in bx0..=bx1 {
                        let (mx, my) = (bx * BIN as i64 + BIN as i64 / 2 - cx, by * BIN as i64 + BIN as i64 / 2 - cy);
                        if mx * mx + my * my > s2 {
                            continue;
                        }
                        let mut n = 0u32;
                        for y in (by as usize * BIN)..((by as usize + 1) * BIN).min(h) {
                            for x in (bx as usize * BIN)..((bx as usize + 1) * BIN).min(w) {
                                n += occ[y * w + x] as u32;
                            }
                        }
                        if n > best.0 {
                            best = (n, Some((by * nbw + bx) as usize));
                        }
                    }
                }
                (touching, best.1, best.0)
            })
            .collect()
    }

    /// One task at fine cell (`x`, `y`) for one frame of `ticks` ticks;
    /// returns the food it cost.
    fn do_task(&mut self, t: Task, x: f32, y: f32, ticks: u32, bank: f32) -> f32 {
        let d = &mut self.dish;
        let (w, h) = (d.w, d.h);
        let s = d.p.scale as f32;
        let secs = ticks as f32 / self.tend.tps as f32;
        let (cx, cy) = cell_of(w, h, x, y);
        let cells: Vec<usize> = disc(w, h, cx, cy, r2(self.tend.r * s)).filter(|&i| d.agar[i]).collect();
        match t {
            Task::None => 0.0,
            Task::Road => {
                let a = self.tend.road * secs;
                for &i in &cells {
                    d.species_trail[OWN as usize][i] += a;
                    d.trail[i] += a;
                }
                self.roads += 1;
                0.0
            }
            Task::Feed => {
                // Feed spreads charge on the ground of its disc, as a
                // crystal holds it: the walkers there eat it at the
                // crystals' rate, whichever mould they are. (A first cut put
                // energy straight into the player's walkers in the disc: on
                // seed 1, 5,855 food went into a few dozen crowded walkers
                // that could not divide fast enough, and the mould shrank
                // from 2,500 to 1,025 by 5:00.)
                let cost = (self.tend.feed * secs).min(bank.max(0.0));
                if cost <= 0.0 || d.charge.is_empty() || cells.is_empty() {
                    return 0.0;
                }
                let each = cost * self.tend.charge / cells.len() as f32;
                for &i in &cells {
                    d.charge[i] += each;
                }
                self.fed += cost;
                cost
            }
            Task::Cut => {
                let inside: std::collections::HashSet<usize> = cells.iter().copied().collect();
                for k in 0..d.walkers.len() {
                    let on = d.lives(k) && d.spec[k] == OWN && (d.life.is_empty() || !matches!(d.life[k], LIFE_SPORE | LIFE_DEAD));
                    if on && inside.contains(&(d.walkers[k].1 as usize * w + d.walkers[k].0 as usize)) {
                        d.kill(k);
                        self.cut += 1;
                    }
                }
                for &i in &cells {
                    let own = d.species_trail[OWN as usize][i];
                    d.species_trail[OWN as usize][i] = 0.0;
                    d.trail[i] = (d.trail[i] - own).max(0.0);
                }
                0.0
            }
        }
    }
}

impl World for Garden {
    fn size(&self) -> (usize, usize) {
        (self.dish.w, self.dish.h)
    }

    fn scale(&self) -> f32 {
        self.dish.p.scale as f32
    }

    fn ticks(&self) -> u64 {
        self.dish.ticks
    }

    /// For rules timed in fine steps (`Rules::crater`).
    fn fine_steps(&self) -> u32 {
        self.dish.p.substeps as u32
    }

    /// `squad::frame` with the species test: shots and reads see the
    /// enemy only; smell, ticks and harvest as the squad's. Counts what
    /// each side ate.
    fn frame(&mut self, f: &Frame, ticks: u32) -> FrameOut {
        let shots = self.shots(&f.shots, f.slime, f.crater);
        squad::smell(&mut self.dish, &f.units);
        let mut was = Vec::new();
        for _ in 0..ticks {
            if self.dish.p.species_convert > 0.0 {
                was.clear();
                was.extend((0..self.dish.walkers.len()).map(|k| if self.dish.lives(k) { self.dish.spec[k] } else { u8::MAX }));
            }
            self.dish.tick();
            for (k, &s) in was.iter().enumerate() {
                let now = self.dish.spec[k];
                if s != u8::MAX && now != s && self.dish.lives(k) {
                    if now == OWN {
                        self.turned_own += 1;
                    } else if s == OWN {
                        self.turned_enemy += 1;
                    }
                }
            }
            if !self.dish.took.is_empty() {
                for k in 0..self.dish.walkers.len() {
                    let t = self.dish.took[k] as f64;
                    if t > 0.0 && self.dish.lives(k) {
                        if self.dish.spec[k] == OWN {
                            self.own_ate += t;
                        } else {
                            self.enemy_ate += t;
                        }
                    }
                }
            }
        }
        let reads = self.read(&f.units);
        let took = squad::harvest(&mut self.dish, &f.units);
        let w = self.dish.w;
        FrameOut {
            units: reads.into_iter().zip(took).map(|((touching, b, dense), took)| UnitOut { touching, target: b.map(|b| squad::block_centre(w, b)), took, dense }).collect(),
            shots,
        }
    }

    fn build_wall(&mut self, a: (f32, f32), b: (f32, f32), width: f32) {
        self.dish.build_wall(a.0, a.1, b.0, b.1, width);
    }

    /// The enemy's walkers: to the game, the mould.
    fn living(&self) -> usize {
        self.counts_by_side().1
    }

    fn crystals(&self) -> Vec<((f32, f32), f32)> {
        World::crystals(&self.dish)
    }

    fn crystal_r(&self, c: usize) -> f32 {
        World::crystal_r(&self.dish, c)
    }

    fn landing(&self) -> Option<(f32, f32)> {
        World::landing(&self.dish)
    }

    fn open(&self, x: f32, y: f32) -> bool {
        World::open(&self.dish, x, y)
    }

    /// The enemy's births stop (its cap goes to 0); the player's mould
    /// keeps growing.
    fn stop_births(&mut self) {
        if let Some(c) = self.dish.p.species_cap.get_mut(0) {
            *c = 0;
        }
    }

    fn food_deposit(&self) -> f32 {
        self.dish.p.food_deposit
    }

    fn calm(&self, at: &[(f32, f32)], r: f32) -> Vec<u32> {
        World::calm(&self.dish, at, r)
    }

    fn hash(&self) -> u64 {
        self.dish.hash()
    }

    /// The soldiers' tasks, once a frame before the frame's ticks; returns
    /// the food spent.
    fn tend(&mut self, soldiers: &[Soldier], bank: f32, ticks: u32) -> f32 {
        let mut spent = 0.0;
        for (k, s) in soldiers.iter().enumerate() {
            let t = self.tasks.get(k).copied().unwrap_or_default();
            if s.alive && t != Task::None {
                spent += self.do_task(t, s.x, s.y, ticks, bank - spent);
            }
        }
        spent
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::mould::larder_world::{Game, Order, Rules};

    fn garden() -> Garden {
        garden_world(5, 1, 600, None)
    }

    /// Each side starts on its own crystal with its share, and the game's
    /// mould is the enemy alone.
    #[test]
    fn the_garden_starts_two_moulds() {
        let g = garden();
        let (own, enemy) = g.counts_by_side();
        assert!(own > 500 && enemy > 500, "own {own} enemy {enemy}");
        let game = Game::new(g, Rules::default(), 5);
        assert_eq!(game.mould, enemy);
        // The larder is on the player's home crystal.
        let d = &game.world.dish;
        let k = (0..d.walkers.len()).filter(|&k| d.lives(k) && d.spec[k] == OWN).map(|k| d.walkers[k]).map(|w| (w.0 - game.larder.0).hypot(w.1 - game.larder.1)).fold(0.0f32, f32::max);
        assert!(k < 40.0, "the player's mould starts by the larder: farthest {k}");
    }

    /// Each side holds half the slots, and a taken core stops only the
    /// enemy's births.
    #[test]
    fn taken_core_stops_only_the_enemys_births() {
        let mut g = garden();
        let half = g.dish.walkers.len() / 2;
        assert_eq!(g.dish.p.species_cap[0], half);
        assert_eq!(g.dish.p.species_cap[OWN as usize], half);
        g.stop_births();
        assert_eq!(g.dish.p.species_cap[0], 0);
        assert_eq!(g.dish.p.species_cap[OWN as usize], half);
    }

    /// A road lays the player's trail where the soldier stands; a cut kills
    /// the player's walkers there and leaves the enemy's; a feed spends the
    /// bank into the player's walkers.
    #[test]
    fn tasks_act_on_the_players_mould_only() {
        let mut g = garden();
        let own_at = g.dish.walkers.iter().zip(&g.dish.spec).find(|(_, &s)| s == OWN).map(|(w, _)| (w.0, w.1)).unwrap();
        let soldier = |x: f32, y: f32| {
            let mut s = Game::new(garden(), Rules::default(), 5).soldiers[0];
            s.x = x;
            s.y = y;
            s
        };
        let s = [soldier(own_at.0, own_at.1)];
        let i = own_at.1 as usize * g.dish.w + own_at.0 as usize;
        let before = g.dish.species_trail[OWN as usize][i];
        g.set_task(0, Task::Road);
        assert_eq!(g.tend(&s, 0.0, 15), 0.0);
        assert!(g.dish.species_trail[OWN as usize][i] >= before + 29.0);
        let c0: f64 = g.dish.charge.iter().map(|&c| c as f64).sum();
        g.set_task(0, Task::Feed);
        let cost = g.tend(&s, 100.0, 15);
        let c1: f64 = g.dish.charge.iter().map(|&c| c as f64).sum();
        assert!((cost - 20.0).abs() < 1e-3 && (c1 - c0 - 20.0 * 64.0).abs() < 1.0, "cost {cost}, charge {c0} -> {c1}");
        let (own0, enemy0) = g.counts_by_side();
        g.set_task(0, Task::Cut);
        g.tend(&s, 0.0, 15);
        let (own1, enemy1) = g.counts_by_side();
        assert!(own1 < own0 && enemy1 == enemy0, "own {own0} -> {own1}, enemy {enemy0} -> {enemy1}");
        assert_eq!(g.dish.species_trail[OWN as usize][i], 0.0);
    }

    /// Soldiers' shots and touch see only the enemy: a soldier standing in
    /// the player's mould is not drained and does not fire on it.
    #[test]
    fn soldiers_leave_the_players_mould_alone() {
        let mut game = Game::new(garden(), Rules::default(), 5);
        let start = game.world.counts_by_side().0;
        for k in 0..game.soldiers.len() {
            game.apply(crate::mould::larder_world::Cmd::Order(k, Order::Hold));
        }
        for _ in 0..20 {
            game.step();
        }
        assert!(game.soldiers.iter().all(|s| s.alive && s.hp == Rules::default().hp), "soldiers drained in their own mould");
        assert_eq!(game.killed, 0, "shots on the player's mould");
        assert!(game.world.counts_by_side().0 > start / 2);
    }

    /// A shot at the enemy lays the crater on its strike, as the one-mould
    /// game's do, and a shot with crater 0 lays none.
    #[test]
    fn shots_lay_the_crater_on_the_enemy() {
        let mut g = garden();
        for _ in 0..30 {
            g.dish.tick();
        }
        let w = g.dish.w;
        let slime = squad::slime_trail(6.0);
        let at = (0..w * g.dish.h).find(|&i| g.enemy_trail(i) >= slime && g.dish.species_trail[OWN as usize][i] < 0.1).expect("enemy slime");
        let to = ((at % w) as f32 + 0.5, (at / w) as f32 + 0.5);
        let shot = [ShotIn { from: to, to, r: 2.0 }];
        let mut plain = g.clone();
        let out = plain.shots(&shot, slime, 0);
        assert!(out[0].hit.is_some(), "{out:?}");
        assert!(plain.dish.scorch.is_empty(), "crater 0 lays none");
        g.shots(&shot, slime, 40);
        assert_eq!(g.dish.scorch[at], 40, "the crater on the strike");
    }

    /// A det cord across both moulds kills walkers of each and craters
    /// its line.
    #[test]
    fn a_cord_cuts_both_moulds() {
        let mut g = garden();
        for _ in 0..30 {
            g.dish.tick();
        }
        let pick = |g: &Garden, own: bool| (0..g.dish.walkers.len()).find(|&k| g.dish.lives(k) && (g.dish.spec[k] == OWN) == own).map(|k| (g.dish.walkers[k].0, g.dish.walkers[k].1)).unwrap();
        let (a, b) = (pick(&g, true), pick(&g, false));
        let (own, enemy) = g.cord(a, b, &KitRules::default(), 15);
        assert!(own > 0 && enemy > 0, "own {own} enemy {enemy}");
        let i = a.1 as usize * g.dish.w + a.0 as usize;
        assert!(g.dish.scorch[i] > 0, "the line is cratered");
    }
}
