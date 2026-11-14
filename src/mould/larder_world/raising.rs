//! Builders raising buildings and walls over time. Moved out of
//! `larder_world.rs` unchanged (C9, 2026-11-14).

use super::*;

/// A wall's raise time counts once per this many dish cells of length.
pub const WALL_RAISE_LEN: f32 = 10.0;

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
                if self.at_site(k, b) {
                    let secs = self.raise_secs(b);
                    self.raise(b, if secs > 0.0 { dt / secs } else { 1.0 });
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
