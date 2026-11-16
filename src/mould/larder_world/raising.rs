//! Builders raising buildings and walls over time. Moved out of
//! `larder_world.rs` unchanged (C9, 2026-11-14).

use super::*;

/// A wall's raise time counts once per this many dish cells of length.
pub const WALL_RAISE_LEN: f32 = 10.0;
/// Walkers more on one side of a wall than the other that make the other
/// its home side (`Game::wall_home`).
pub const WALL_SLIME: u32 = 5;
/// How far each side of a wall `wall_home` looks for slime and soldiers, dish
/// cells at least (half the wall's length if longer).
pub const WALL_LOOK: f32 = 8.0;
/// How far a wall rises while a soldier stands in its footprint
/// (`build_turn`): its last sliver waits for them to step out.
pub const WALL_WAIT: f32 = 0.995;
/// Game seconds a wall's last sliver waits for its footprint to clear
/// (`build_turn`; the wall's `Building::ready` holds when it began).
pub const WALL_WAIT_SECS: f32 = 4.0;

impl<W: World> Game<W> {
    /// Where soldier `k` stands to raise building `b` (fine cells): beside
    /// it, or for a wall the nearest point of its line; None once it stands
    /// or is gone.
    pub(super) fn site(&self, k: usize, b: usize) -> Option<(f32, f32)> {
        let bd = self.buildings.get(b).filter(|bd| bd.alive && bd.built < 1.0)?;
        let s = self.soldiers[k];
        Some(match bd.kind {
            Kind::Wall { to } => nearest_on(bd.at, to, (s.x, s.y)),
            _ => bd.at,
        })
    }

    /// Seconds a lone builder takes to raise building `b`
    /// (`BuildRules::raise`; a wall a raise per `WALL_RAISE_LEN` dish cells
    /// of its length, at least one).
    pub fn raise_secs(&self, b: usize) -> f32 {
        let bd = self.buildings[b];
        let r = self.rules.build.raise;
        match bd.kind {
            Kind::Wall { to } => r * ((bd.at.0 - to.0).hypot(bd.at.1 - to.1) / self.s() / WALL_RAISE_LEN).max(1.0),
            _ => r,
        }
    }

    /// A builder's turn (`Order::Build`): within reach of its site it
    /// raises the building `dt / raise_secs` (builders add up); once the
    /// building stands or is gone it takes its next order, else holds. And
    /// an idle soldier (holding with nothing queued) takes the nearest
    /// ghost within its range, so a ghost placed by an idle squad rises
    /// without another order (the census hands lay buildings with
    /// `Cmd::Build` alone).
    pub(super) fn build_turn(&mut self, k: usize, dt: f32) {
        let s = self.soldiers[k];
        let sc = self.s();
        if s.order == Order::Hold && s.queued == 0 {
            let reach = self.rules.range * sc;
            let near = (0..self.buildings.len())
                .filter_map(|b| self.site(k, b).map(|p| (b, (p.0 - s.x).hypot(p.1 - s.y))))
                .filter(|&(_, d)| d <= reach)
                .min_by(|a, b| a.1.total_cmp(&b.1));
            if let Some((b, _)) = near {
                self.soldiers[k].order = Order::Build(b);
            }
            return;
        }
        let Order::Build(b) = s.order else { return };
        match self.site(k, b) {
            None => {
                if !self.next(k) {
                    self.soldiers[k].order = Order::Hold;
                }
            }
            Some(_) => {
                // A wall's defenders stand with its builders but don't
                // raise it (`cover_one`).
                if self.at_site(k, b) && !s.cover {
                    let secs = self.raise_secs(b);
                    // A working squad's builders make up for its defenders
                    // (`Game::work_boost`).
                    let boost = self.work_boost(k);
                    let mut amount = if secs > 0.0 { dt / secs * boost } else { 1.0 };
                    // A wall doesn't close on a soldier still in its
                    // footprint: it waits at its last sliver while they
                    // step out to its home side (`goal_raw`), so nobody is
                    // shut in it and popped out on the slime's side.
                    // At most `WALL_WAIT_SECS`, though: a soldier that
                    // can't step out (held by another order) is not worth a
                    // wall that never stands; it walks out of the risen
                    // wall as before (`walk`). Its first cut waited for
                    // good, and on the Fork a guard whose post was past the
                    // line held the wall at its last sliver.
                    let bd = self.buildings[b];
                    if bd.built + amount >= 1.0 && self.wall_occupied(b) {
                        let now = self.world.ticks();
                        if bd.ready == 0 {
                            self.buildings[b].ready = now.max(1);
                        }
                        let waited = now.saturating_sub(self.buildings[b].ready) as f32 / self.rules.tps as f32;
                        if waited < WALL_WAIT_SECS {
                            amount = (WALL_WAIT - bd.built).max(0.0);
                        }
                    }
                    self.raise(b, amount);
                }
            }
        }
    }

    /// How near site `p` of building `b` a builder stands, fine cells:
    /// (no nearer than this, raising from up to this). The far end is
    /// `BuildRules::reach` (past a crystal's rim for a ghost on one); the
    /// near end is a cell clear of the footprint (a wall: of half its
    /// width), so the ghost stays in sight and a wall can't shut its
    /// builder in.
    pub(super) fn build_band(&self, b: usize, p: (f32, f32)) -> (f32, f32) {
        let sc = self.s();
        let far = self.reach(p, self.rules.build.reach);
        let near = match self.buildings[b].kind {
            Kind::Wall { .. } => (self.rules.wall_width / 2.0 + 1.0) * sc,
            _ => (self.rules.build.touch + 1.0) * sc,
        };
        (near.min(0.5 * far), far)
    }

    /// Soldier `k` is close enough to ghost `b` to raise it: in reach
    /// (`build_band`), plus elbow room (`Rules::elbow`), so a builder a
    /// neighbour pushed a little back still raises (without it the one
    /// pushed back stood idle: two builders raised a turret in 56 ticks
    /// against one's 61).
    pub(super) fn at_site(&self, k: usize, b: usize) -> bool {
        let Some(p) = self.site(k, b) else { return false };
        let s = self.soldiers[k];
        let (_, far) = self.build_band(b, p);
        let elbow = self.rules.elbow.max(0.0) * self.rules.touch * self.s();
        (p.0 - s.x).hypot(p.1 - s.y) <= far + elbow
    }

    /// A side for wall `at`-`to` placed now (`Building::home`): a unit
    /// vector square to its line. The side with less slime by it, when the
    /// two differ by `WALL_SLIME` walkers or more (a wall goes up against
    /// the slime: the owner's "trapped with the slime"); else the larder's
    /// side; else the side most soldiers near it stand on.
    pub(super) fn wall_home(&self, at: (f32, f32), to: (f32, f32)) -> (f32, f32) {
        let sc = self.s();
        let (dx, dy) = (to.0 - at.0, to.1 - at.1);
        let l = dx.hypot(dy);
        if l < 1e-3 {
            return (0.0, 0.0);
        }
        let n = (-dy / l, dx / l);
        let mid = ((at.0 + to.0) / 2.0, (at.1 + to.1) / 2.0);
        let r = (l / 2.0).max(WALL_LOOK * sc);
        let probe = |sg: f32| self.world.walkers_in((mid.0 + sg * n.0 * r, mid.1 + sg * n.1 * r), r);
        let (plus, minus) = (probe(1.0), probe(-1.0));
        if plus.abs_diff(minus) >= WALL_SLIME {
            return if plus < minus { n } else { (-n.0, -n.1) };
        }
        // No slime to tell: the larder's side (where the squad came from),
        // or with the larder on the line itself, the side most soldiers
        // near it stand on. A majority vote first split ties badly: a squad
        // straddling the line by the mould went with its larger half.
        let side = |p: (f32, f32)| (p.0 - mid.0) * n.0 + (p.1 - mid.1) * n.1;
        let mut sg = side(self.larder);
        if sg.abs() < r {
            let near = r + self.rules.range * sc;
            let votes: i32 = self
                .soldiers
                .iter()
                .filter(|s| s.alive && (s.x - mid.0).hypot(s.y - mid.1) <= near)
                .map(|s| side((s.x, s.y)))
                .filter(|d| d.abs() > 1e-3)
                .map(|d| if d > 0.0 { 1 } else { -1 })
                .sum();
            if votes != 0 {
                sg = votes as f32;
            }
        }
        if sg < 0.0 {
            (-n.0, -n.1)
        } else {
            n
        }
    }

    /// How far `p` stands on wall `b`'s home side of its line, fine cells
    /// (negative past it); None for anything but a wall with a home side.
    pub(super) fn wall_side(&self, b: usize, p: (f32, f32)) -> Option<f32> {
        let bd = self.buildings.get(b)?;
        let Kind::Wall { .. } = bd.kind else { return None };
        if bd.home == (0.0, 0.0) {
            return None;
        }
        Some((p.0 - bd.at.0) * bd.home.0 + (p.1 - bd.at.1) * bd.home.1)
    }

    /// Whether a live soldier stands in wall ghost `b`'s footprint (its
    /// width and half a dish cell), so its cells would shut them in.
    pub(super) fn wall_occupied(&self, b: usize) -> bool {
        let bd = self.buildings[b];
        let Kind::Wall { to } = bd.kind else { return false };
        // A door's cells are ground to soldiers: one in it is not shut in.
        if bd.door {
            return false;
        }
        let r = (self.rules.wall_width / 2.0 + 0.5) * self.s();
        self.soldiers.iter().any(|s| {
            let q = nearest_on(bd.at, to, (s.x, s.y));
            s.alive && (q.0 - s.x).hypot(q.1 - s.y) < r
        })
    }

    /// Ghost `b`'s builders: (raising it now, ordered to it in all). The
    /// window's progress label reads it.
    pub fn raising(&self, b: usize) -> (usize, usize) {
        let mut n = (0, 0);
        for (k, s) in self.soldiers.iter().enumerate() {
            if s.alive && s.order == Order::Build(b) {
                n.1 += 1;
                n.0 += self.at_site(k, b) as usize;
            }
        }
        n
    }

    /// Seconds until ghost `b` stands at the rate it rises now (builders
    /// add up); None while nobody is raising it, or once it stands.
    pub fn raise_left(&self, b: usize) -> Option<f32> {
        let bd = self.buildings.get(b).filter(|bd| bd.alive && bd.built < 1.0)?;
        let (now, _) = self.raising(b);
        (now > 0).then(|| (1.0 - bd.built) * self.raise_secs(b) / now as f32)
    }
}
