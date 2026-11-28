//! Doors and thick walls (owner, 2026-11-16 14:24Z and 14:25Z, the walls
//! thread's two picks).
//!
//! **A door** is a wall (`Kind::Wall`, `Building::door`): its cells are the
//! world's wall cells, so the slime can't step on them and bites them down
//! at the same pace as any wall's (`BigDish::bite_walls`, the real
//! simulation's damage), while soldiers stand on them as on open ground
//! (`Game::doors`, read by `Game::ground` and `standable`). Shots still stop
//! at it (`World::clear`): it is a wall to everything but soldiers' feet.
//! Drawn across a gap it is raised like a wall; drawn along a standing wall
//! it snaps onto that wall's line and is cut at once (`Game::door`), since
//! its cells are already there. Rejected for now: gates that open (they
//! would need the engines to lift wall cells, which neither can) and
//! climbing over (soldier path costs, owned by two other threads that day).
//!
//! **Harden** (`Game::upgrade` on a wall, since 2026-11-28): each level
//! gives every standing cell of the wall more health (`Game::wall_full`,
//! `Rules::wall_hp`, `wall_harden`), at the wall's first price, up to
//! `WALL_LEVELS`; worn cells keep their wear and rubble stays rubble. The
//! owner, 06:07Z: walls "should not actually get thicker when upgrading,
//! they get stronger (more health per cell). I just layer the walls in
//! play to thicken." From 11-16 to 11-28 a level instead added a band of
//! cells on the home side (Thicken: 2, 4, 6 then 8 cells), soldiers in
//! the new band stepping out to the home side first; a wall is now always
//! its one band.

use super::*;

/// A wall's top level: as placed, then three hardenings.
pub const WALL_LEVELS: u8 = 4;

/// A clicked door's length, in wall widths (`Game::door_from_click`): 6
/// cells at the default width and scale 1, the doors the tests drew.
pub const DOOR_WIDTHS: f32 = 3.0;

impl<W: World> Game<W> {
    /// A door's cell at `(x, y)` (fine cells): a standing door's wall cell,
    /// or one worn to rubble.
    pub fn door_at(&self, x: f32, y: f32) -> bool {
        if self.doors.is_empty() || x < 0.0 || y < 0.0 {
            return false;
        }
        let (w, h) = self.world.size();
        let (cx, cy) = (x as usize, y as usize);
        cx < w && cy < h && self.doors[cy * w + cx]
    }

    /// Ground a soldier's feet may be on: open (`World::open`) or a door.
    /// What the walk, the steer and the squad's anchor test in place of
    /// `World::open` (squad-movement thread, 2026-11-16: "the one place is
    /// `Game::standable` plus `standable_cells`", and squad_turn's two
    /// checks).
    /// Not a bulwark cell (`Game::bulwark`, E2b): set slime is no ground.
    pub fn ground(&self, x: f32, y: f32) -> bool {
        (self.world.open(x, y) || self.door_at(x, y)) && !self.bulwark_at(x, y)
    }

    /// Wall `b`'s line, fine cells: its one band (since 2026-11-28 a level
    /// hardens it rather than adding a band, `Game::harden`).
    pub fn wall_band(&self, b: usize) -> ((f32, f32), (f32, f32)) {
        let bd = self.buildings[b];
        let Kind::Wall { to } = bd.kind else { return (bd.at, bd.at) };
        (bd.at, to)
    }

    /// A wall cell's full health at `level` (1 as placed): `Rules::wall_hp`,
    /// and `wall_harden` of it more each level.
    pub fn wall_full(&self, level: u8) -> f32 {
        self.rules.wall_hp * (1.0 + self.rules.wall_harden * f32::from(level.max(1) - 1))
    }

    /// Write standing wall `b`'s cells into the world at its cell health
    /// (`Building::max_hp`), then mark the doors. Its cells already walls
    /// keep their worn health; rubble in its footprint is built again
    /// (`World::build_wall_hp` writes open cells only), so a door cut into
    /// a holed wall fills it.
    pub(super) fn wall_bands(&mut self, b: usize) {
        let s = self.s();
        let (a, z) = self.wall_band(b);
        let hp = self.buildings[b].max_hp;
        self.world.build_wall_hp((a.0 / s, a.1 / s), (z.0 / s, z.1 / s), self.rules.wall_width, hp);
        self.walls += 1;
        self.mark_doors();
        self.stale_maps();
    }

    /// The door mask (`Game::doors`) from every standing door: the wall
    /// cells (`World::wall_cell`) in its bands' footprints, by
    /// `BigDish::build_wall`'s own test (a cell's middle within half the
    /// width of the line). Cleared when there is none, so a game without
    /// doors reads nothing. Runs only when a wall changes.
    pub(super) fn mark_doors(&mut self) {
        let doors: Vec<usize> = (0..self.buildings.len()).filter(|&b| self.buildings[b].up() && self.buildings[b].door).collect();
        if doors.is_empty() {
            self.doors.clear();
            return;
        }
        let (w, h) = self.world.size();
        let mut m = vec![false; w * h];
        let r = (self.rules.wall_width * self.s() / 2.0).max(0.5);
        for &b in &doors {
            {
                let (a, z) = self.wall_band(b);
                let (x0, x1) = (((a.0.min(z.0) - r).floor().max(0.0)) as usize, ((a.0.max(z.0) + r).ceil().max(0.0) as usize).min(w.saturating_sub(1)));
                let (y0, y1) = (((a.1.min(z.1) - r).floor().max(0.0)) as usize, ((a.1.max(z.1) + r).ceil().max(0.0) as usize).min(h.saturating_sub(1)));
                for y in y0..=y1 {
                    for x in x0..=x1 {
                        let p = (x as f32 + 0.5, y as f32 + 0.5);
                        let q = nearest_on(a, z, p);
                        let i = y * w + x;
                        if (q.0 - p.0).hypot(q.1 - p.1) <= r && self.world.wall_cell(i) {
                            m[i] = true;
                        }
                    }
                }
            }
        }
        self.doors = m;
    }

    /// The standing (or rising) plain wall a door from `a` to `b` lies
    /// along, if any: both ends within its footprint and a cell either
    /// side, nearest first. The door snaps onto its line.
    pub fn door_host(&self, a: (f32, f32), b: (f32, f32)) -> Option<usize> {
        let sc = self.s();
        let ww = self.rules.wall_width * sc;
        let mut best: Option<(usize, f32)> = None;
        for (k, bd) in self.buildings.iter().enumerate() {
            let Kind::Wall { to } = bd.kind else { continue };
            if !bd.alive || bd.door || bd.home == (0.0, 0.0) {
                continue;
            }
            let l = (to.0 - bd.at.0).hypot(to.1 - bd.at.1);
            if l < 1e-3 {
                continue;
            }
            let u = ((to.0 - bd.at.0) / l, (to.1 - bd.at.1) / l);
            // Across the line: half a width either side, a cell's slack.
            let (lo, hi) = (-ww / 2.0 - sc, ww / 2.0 + sc);
            let fits = |p: (f32, f32)| {
                let (dx, dy) = (p.0 - bd.at.0, p.1 - bd.at.1);
                let along = dx * u.0 + dy * u.1;
                let across = dx * bd.home.0 + dy * bd.home.1;
                (-sc..=l + sc).contains(&along) && (lo..=hi).contains(&across)
            };
            if fits(a) && fits(b) {
                let mid = ((a.0 + b.0) / 2.0, (a.1 + b.1) / 2.0);
                let q = nearest_on(bd.at, to, mid);
                let d = (q.0 - mid.0).hypot(q.1 - mid.1);
                if best.is_none_or(|(_, e)| d < e) {
                    best = Some((k, d));
                }
            }
        }
        best.map(|(k, _)| k)
    }

    /// Where a door drawn from `a` to `b` goes: onto its host wall's line
    /// (`door_host`), its ends brought square onto it; else where drawn.
    pub fn door_line(&self, a: (f32, f32), b: (f32, f32)) -> ((f32, f32), (f32, f32), Option<usize>) {
        match self.door_host(a, b) {
            Some(h) => {
                let bd = self.buildings[h];
                let Kind::Wall { to } = bd.kind else { unreachable!() };
                (nearest_on(bd.at, to, a), nearest_on(bd.at, to, b), Some(h))
            }
            None => (a, b, None),
        }
    }

    /// Where a door clicked at `at` goes (owner, 2026-11-21: "doors should
    /// be created with just a click, not dragging"): `DOOR_WIDTHS` wall
    /// widths along the plain wall under the click (standing or rising,
    /// `door_host`'s footprint), centred on the click and kept inside the
    /// wall's ends (a wall shorter than that is a door whole). None off
    /// every wall. Dragging a door across a gap went with the drag.
    pub fn door_from_click(&self, at: (f32, f32)) -> Option<((f32, f32), (f32, f32))> {
        let h = self.door_host(at, at)?;
        let bd = self.buildings[h];
        let Kind::Wall { to } = bd.kind else { return None };
        let l = (to.0 - bd.at.0).hypot(to.1 - bd.at.1);
        let u = ((to.0 - bd.at.0) / l, (to.1 - bd.at.1) / l);
        let half = (0.5 * DOOR_WIDTHS * self.rules.wall_width * self.s()).min(l / 2.0);
        let along = ((at.0 - bd.at.0) * u.0 + (at.1 - bd.at.1) * u.1).clamp(half, l - half);
        let c = (bd.at.0 + u.0 * along, bd.at.1 + u.1 * along);
        Some(((c.0 - u.0 * half, c.1 - u.1 * half), (c.0 + u.0 * half, c.1 + u.1 * half)))
    }

    /// A door from `a` to `b` (`Cmd::Door`), paid as a wall that long.
    /// Along a standing wall it takes the wall's thickness and home side
    /// and is cut at once; along a wall still rising it waits for it and
    /// rises after; across a gap it is a ghost builders raise. True if
    /// placed.
    pub fn door(&mut self, a: (f32, f32), b: (f32, f32)) -> bool {
        let (a, b, host) = self.door_line(a, b);
        if (a.0 - b.0).hypot(a.1 - b.1) < 0.5 * self.s() {
            return false;
        }
        if !self.build(Kind::Wall { to: b }, a) {
            return false;
        }
        let k = self.buildings.len() - 1;
        self.buildings[k].door = true;
        if let Some(h) = host {
            let hb = self.buildings[h];
            self.buildings[k].home = hb.home;
            self.buildings[k].level = hb.level;
            (self.buildings[k].max_hp, self.buildings[k].hp) = (hb.max_hp, hb.max_hp);
        }
        // Whole already (`BuildRules::raise` 0 makes every building whole
        // when placed, before it was a door): its bands and the mask again.
        if self.buildings[k].up() {
            self.wall_bands(k);
        } else if host.is_some_and(|h| self.buildings[h].up()) {
            self.raise(k, 1.0);
        }
        true
    }

    /// The wall building `b`'s thickness goes with: a door cut into a wall
    /// is that wall's, anything else its own.
    pub fn wall_of(&self, b: usize) -> usize {
        let bd = self.buildings[b];
        match bd.kind {
            Kind::Wall { to } if bd.door => self.door_host(bd.at, to).unwrap_or(b),
            _ => b,
        }
    }

    /// The doors cut into wall `h` (`door_host` of their own line).
    fn doors_in(&self, h: usize) -> Vec<usize> {
        (0..self.buildings.len())
            .filter(|&d| {
                let bd = self.buildings[d];
                let Kind::Wall { to } = bd.kind else { return false };
                d != h && bd.alive && bd.door && self.door_host(bd.at, to) == Some(h)
            })
            .collect()
    }

    /// Harden standing wall `b` a level (`Game::upgrade` pays): every
    /// standing cell gains the difference to `wall_full` of the new level,
    /// so a worn wall keeps its wear; doors cut into it harden with it.
    pub(super) fn harden(&mut self, b: usize) {
        let level = self.buildings[b].level + 1;
        self.buildings[b].level = level;
        for d in self.doors_in(b) {
            self.buildings[d].level = level;
        }
        let full = self.wall_full(level);
        self.harden_to(b, full);
    }

    /// Set standing wall `b`'s cell health to `full` (only upward): its
    /// cells and its doors' gain the difference. The rescue's dug-out rings
    /// use it to stand at their own health (`RescueRules::thick`).
    pub(super) fn harden_to(&mut self, b: usize, full: f32) {
        let add = full - self.buildings[b].max_hp;
        if add <= 0.0 {
            return;
        }
        let s = self.s();
        let (a, z) = self.wall_band(b);
        self.world.harden_wall((a.0 / s, a.1 / s), (z.0 / s, z.1 / s), self.rules.wall_width, add);
        for k in std::iter::once(b).chain(self.doors_in(b)) {
            let bd = &mut self.buildings[k];
            bd.max_hp += add;
            bd.hp += add;
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn small(seed: u64) -> Game<BigDish> {
        Game::new(race_world(seed, 1, 1_000), Rules { build: BuildRules { raise: 0.0, ..BuildRules::default() }, ..Rules::default() }, seed)
    }

    /// A game with a straight wall across the middle, standing, and its
    /// squad of one on the home side: (game, wall, its ends).
    fn walled() -> (Game<BigDish>, usize, (f32, f32), (f32, f32)) {
        let mut g = small(1);
        g.rules.build.anywhere = true;
        g.bank = 1e6;
        let s = g.soldiers[0];
        let sc = g.s();
        // Wall to wall, so the only way past is through it.
        let (_, h) = g.world.size();
        let (a, b) = ((s.x + 6.0 * sc, 0.0), (s.x + 6.0 * sc, h as f32));
        assert!(g.apply(Cmd::Wall(a, b)));
        let w = g.buildings.len() - 1;
        g.raise(w, 1.0);
        assert!(g.buildings[w].up());
        (g, w, a, b)
    }

    /// A door cut into a standing wall stands at once, its cells are still
    /// the world's walls (the slime's walkers can't enter them, shots stop
    /// at them) and soldiers may stand on them; the rest of the wall is
    /// shut to them.
    #[test]
    fn a_door_is_wall_to_the_slime_and_ground_to_soldiers() {
        let (mut g, w, a, _) = walled();
        let sc = g.s();
        let mid = (a.0, g.soldiers[0].y);
        assert!(!g.standable(mid.0, mid.1), "the wall is shut before the door");
        let (da, db) = ((mid.0 + 0.3 * sc, mid.1 - 2.0 * sc), (mid.0 + 0.3 * sc, mid.1 + 2.0 * sc));
        assert!(g.apply(Cmd::Door(da, db)));
        let d = g.buildings.len() - 1;
        assert!(g.buildings[d].door && g.buildings[d].up(), "cut into a standing wall at once");
        let Kind::Wall { to } = g.buildings[d].kind else { panic!() };
        assert!((g.buildings[d].at.0 - a.0).abs() < 1e-3 && (to.0 - a.0).abs() < 1e-3, "snapped onto the wall's line");
        assert!(g.standable(mid.0, mid.1), "the door is ground to soldiers");
        assert!(!g.world.open(mid.0, mid.1) && !g.world.clear(mid.0, mid.1), "and wall to the slime and to shots");
        let end = (a.0, mid.1 + 6.0 * sc);
        assert!(!g.standable(end.0, end.1), "the rest of the wall stays shut");
        assert_eq!(g.buildings[w].level, 1);
        assert!(g.standable_cells()[cell_of(&g, mid)]);
    }

    fn cell_of(g: &Game<BigDish>, p: (f32, f32)) -> usize {
        let (w, _) = g.world.size();
        p.1 as usize * w + p.0 as usize
    }

    /// A soldier walks through the door to the far side of the wall, and
    /// without the door it can't.
    #[test]
    fn a_soldier_walks_through_a_door() {
        for door in [false, true] {
            let (mut g, _, a, _) = walled();
            let sc = g.s();
            let mid = (a.0, g.soldiers[0].y);
            if door {
                assert!(g.apply(Cmd::Door((mid.0, mid.1 - 3.0 * sc), (mid.0, mid.1 + 3.0 * sc))));
            }
            let s = g.soldiers[0];
            let far = (a.0 + 6.0 * sc, s.y);
            assert!(g.apply(Cmd::SquadOrder(0, Order::Move(far.0, far.1))));
            for _ in 0..600 {
                g.step();
                if g.soldiers[0].x > a.0 + 2.0 * sc {
                    break;
                }
            }
            let x = g.soldiers[0].x;
            if door {
                assert!(x > a.0 + 2.0 * sc, "through the door: x {x}, wall at {}", a.0);
            } else {
                assert!(x < a.0, "no door, no way through: x {x}, wall at {}", a.0);
            }
        }
    }

    /// Harden (2026-11-28): each level raises every standing cell of the
    /// wall to `wall_full` of the level, up to `WALL_LEVELS`, at the wall's
    /// first price; no band grows on either side, a worn cell keeps its
    /// wear, and a door cut into it hardens with it and still lets soldiers
    /// through.
    #[test]
    fn harden_strengthens_the_wall_in_place() {
        let (mut g, w, a, b) = walled();
        let sc = g.s();
        let mid = (a.0, g.soldiers[0].y);
        assert!(g.apply(Cmd::Door((mid.0, mid.1 - 3.0 * sc), (mid.0, mid.1 + 3.0 * sc))));
        let d = g.buildings.len() - 1;
        let n = g.buildings[w].home;
        let price = g.wall_price(a, b);
        assert_eq!(g.upgrade_price(w), Some(price));
        let ww = g.rules.wall_width * sc;
        let (fw, _) = g.world.size();
        let cell = |p: (f32, f32)| p.1 as usize * fw + p.0 as usize;
        let on = (a.0, mid.1 + 6.0 * sc);
        let worn = (a.0, mid.1 + 8.0 * sc);
        let door = (mid.0, mid.1);
        assert!(!g.world.open(on.0, on.1) && !g.world.open(worn.0, worn.1));
        assert_eq!(g.world.wall_health[cell(on)], g.wall_full(1), "placed at level 1's health");
        g.world.wall_health[cell(worn)] -= 30.0;
        let side = (a.0 + n.0 * ww, on.1 + n.1 * ww);
        for l in 2..=WALL_LEVELS {
            let bank = g.bank;
            assert!(g.apply(Cmd::Upgrade(w)));
            assert_eq!(g.buildings[w].level, l);
            assert!((bank - g.bank - price).abs() < 1e-3, "each level costs the first price");
            assert!((g.world.wall_health[cell(on)] - g.wall_full(l)).abs() < 1e-3, "level {l}: its cells at full");
            assert!((g.world.wall_health[cell(worn)] - (g.wall_full(l) - 30.0)).abs() < 1e-3, "level {l}: the worn cell keeps its wear");
            assert!((g.world.wall_health[cell(door)] - g.wall_full(l)).abs() < 1e-3, "level {l}: the door's cells harden too");
            assert!((g.buildings[w].max_hp - g.wall_full(l)).abs() < 1e-3);
            assert!(g.world.open(side.0, side.1), "level {l}: no band on the home side");
            assert_eq!(g.buildings[d].level, l, "the door hardens with it");
            assert!(g.standable(door.0, door.1), "the door still a door");
        }
        assert!(g.wall_full(WALL_LEVELS) > 3.9 * g.wall_full(1), "the top level holds four times the first");
        assert_eq!(g.upgrade_price(w), None, "at the top level");
        assert!(!g.apply(Cmd::Upgrade(w)));
    }

    /// The slime bites a door down as any wall: the same health, so the
    /// same pace (it is the world's own wall cells).
    #[test]
    fn a_door_has_a_walls_health() {
        let (mut g, _, a, _) = walled();
        let sc = g.s();
        let mid = (a.0, g.soldiers[0].y);
        assert!(g.apply(Cmd::Door((mid.0, mid.1 - 3.0 * sc), (mid.0, mid.1 + 3.0 * sc))));
        let wall = g.world.cell_at(a.0, mid.1 + 6.0 * sc).unwrap();
        let door = g.world.cell_at(mid.0, mid.1).unwrap();
        assert!(g.world.wall[door] && !g.world.agar[door]);
        assert_eq!(g.world.wall_health[door], g.world.wall_health[wall]);
    }

    /// **A click makes a door** (owner, 2026-11-21: "doors should be
    /// created with just a click, not dragging"): on a standing wall, a
    /// door `DOOR_WIDTHS` widths long along it, centred on the click, cut
    /// at once; near a wall's end it stays inside the wall; off every wall,
    /// nothing.
    #[test]
    fn a_click_on_a_wall_cuts_a_door() {
        let (mut g, w, a, b) = walled();
        let sc = g.s();
        let len = DOOR_WIDTHS * g.rules.wall_width * sc;
        let mid = (a.0, g.soldiers[0].y);
        // A little off the line, inside its footprint.
        let (da, db) = g.door_from_click((mid.0 + 0.4 * sc, mid.1)).expect("a door on the wall");
        assert!(((da.0 - db.0).hypot(da.1 - db.1) - len).abs() < 1e-3, "{len} long: {da:?} to {db:?}");
        assert!((da.0 - a.0).abs() < 1e-3 && (db.0 - a.0).abs() < 1e-3, "on the wall's line");
        assert!((((da.1 + db.1) / 2.0) - mid.1).abs() < 1e-3, "centred on the click");
        assert!(g.apply(Cmd::Door(da, db)));
        let d = g.buildings.len() - 1;
        assert!(g.buildings[d].door && g.buildings[d].up(), "cut at once");
        assert_eq!(g.wall_of(d), w);
        assert!(g.standable(mid.0, mid.1), "soldiers stand in it");
        // At the wall's end: kept inside it.
        let lo = a.1.min(b.1);
        let (ea, eb) = g.door_from_click((a.0, lo + 0.5 * sc)).unwrap();
        assert!(ea.1.min(eb.1) >= lo - 1e-3 && ((ea.1 - eb.1).abs() - len).abs() < 1e-3, "inside the wall: {ea:?} to {eb:?}");
        // Off every wall.
        assert!(g.door_from_click((a.0 + 10.0 * sc, mid.1)).is_none());
    }
}
