//! **The mould's pieces, heard** (cloud plan C7, 2026-11-14): which parts
//! of the mould are cut off from the body, followed from frame to frame,
//! so each can have its own voice, its cut can tear, its withering can fizz
//! and its lean toward a soldier (the panic's tell) can snarl.
//!
//! It reads only the mould grid (`World::mould_grid`: walkers on each
//! square of `bin` fine cells), which the GPU world packs into the frame's
//! readout it already makes, so it adds no read-back once the bin is
//! asked for. The pieces are the grid's 4-connected groups of occupied
//! squares; the biggest is the body. Four-connected, not eight: a held ring
//! cut is 6 fine cells wide, a square or more at `bin` 4, and a diagonal
//! step would join the two sides across it.
//!
//! Like the rest of `sound`, this only reads: it draws no random numbers
//! and changes nothing in the game.

/// A group of occupied squares this update.
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct Blob {
    pub walkers: u32,
    /// Walker-weighted centre, fine cells.
    pub at: (f32, f32),
    pub squares: u32,
}

/// The 4-connected groups of squares holding `floor` walkers or more,
/// biggest first. A floor of 2 at one dish cell a square: a stray walker
/// is not a bridge. The Fork's held ring left 55-75 walkers stranded on
/// its cells for ten seconds and more, mostly one to a square; at floor 1
/// they joined the cut part to the body until they died.
pub fn blobs(grid: &[u32], cols: usize, bin: usize, floor: u32) -> Vec<Blob> {
    let floor = floor.max(1);
    let rows = grid.len() / cols.max(1);
    let mut seen = vec![false; grid.len()];
    let mut out = Vec::new();
    let mut stack = Vec::new();
    let b = bin as f32;
    for start in 0..grid.len() {
        if seen[start] || grid[start] < floor {
            continue;
        }
        seen[start] = true;
        stack.push(start);
        let (mut n, mut sx, mut sy, mut sq) = (0u64, 0.0f64, 0.0f64, 0u32);
        while let Some(i) = stack.pop() {
            let w = grid[i] as u64;
            let (x, y) = (i % cols, i / cols);
            n += w;
            sx += w as f64 * ((x as f64 + 0.5) * b as f64);
            sy += w as f64 * ((y as f64 + 0.5) * b as f64);
            sq += 1;
            let mut push = |j: usize| {
                if !seen[j] && grid[j] >= floor {
                    seen[j] = true;
                    stack.push(j);
                }
            };
            if x > 0 {
                push(i - 1);
            }
            if x + 1 < cols {
                push(i + 1);
            }
            if y > 0 {
                push(i - cols);
            }
            if y + 1 < rows {
                push(i + cols);
            }
        }
        out.push(Blob { walkers: n.min(u32::MAX as u64) as u32, at: ((sx / n as f64) as f32, (sy / n as f64) as f32), squares: sq });
    }
    out.sort_by(|a, b| b.walkers.cmp(&a.walkers).then(a.at.0.total_cmp(&b.at.0)));
    out
}

/// A cut-off piece followed over time.
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct Piece {
    pub id: u32,
    pub walkers: u32,
    pub at: (f32, f32),
    pub squares: u32,
    /// Seconds it has been seen.
    pub age: f32,
    /// Its walkers' change a second as a share of it, smoothed (negative:
    /// withering, or shot).
    pub change: f32,
    /// Walkers lost a second, smoothed.
    pub losing: f32,
    /// Its centre's speed toward the nearest soldier, fine cells a
    /// second, smoothed (negative: away).
    pub approach: f32,
    /// The voice slot it holds (1 ..= 3), or 0.
    pub slot: u8,
    /// Seconds since its tell (the snarl) last sounded.
    pub since_tell: f32,
    /// Seconds it has been missing from the grid (0: seen this update).
    pub missing: f32,
    /// Its tear was heard (or it was there at the first look): only such a
    /// piece is heard to go, so a rejoin never sounds without its tear.
    pub announced: bool,
}

impl Piece {
    /// Settled and seen: heard (its voice, its snarl, its withering).
    pub fn heard(&self) -> bool {
        self.age >= SETTLE && self.missing == 0.0
    }
}

/// What changed this update.
#[derive(Clone, Debug, Default, PartialEq)]
pub struct Events {
    /// A new piece, torn from the body (where).
    pub torn: Vec<(f32, f32)>,
    /// A piece gone: rejoined the body (true) or withered out (false).
    pub gone: Vec<((f32, f32), bool)>,
    /// A heard piece that broke up near a soldier (where): a scared part's
    /// charge. The piece's centre barely moves before it goes: in the
    /// Fork's staged last stand (a 184-walker part in a held ring with
    /// four soldiers, panic 2) it was seen settled once, at 156 walkers,
    /// and in the next readout its walkers were spread one to a square,
    /// under the link floor, running at the squad. So the charge is heard
    /// as the piece breaking up within `NEAR` of a soldier, not from its
    /// centre's approach (which never passed 1 dish cell a second).
    pub scattered: Vec<(f32, f32)>,
}

/// A soldier this near a piece that breaks up (fine cells per world
/// scale unit: the caller passes `near` already scaled) makes it a charge.
pub const NEAR_DISH: f32 = 20.0;

/// Follows the pieces from update to update.
#[derive(Clone, Debug)]
pub struct Pieces {
    pub pieces: Vec<Piece>,
    /// The body (the biggest group) as of the last update.
    pub body: Option<Blob>,
    next: u32,
    /// A square's side, fine cells.
    bin: f32,
    /// `NEAR_DISH` in fine cells.
    near: f32,
}

/// Smoothing, seconds: a piece's change and its approach.
const CHANGE_SMOOTH: f32 = 1.0;
const APPROACH_SMOOTH: f32 = 0.4;
/// Seconds a group must last before it counts as torn off, so a group that
/// flickers on the grid (a thin tube between two read-outs) is not a cut.
/// The first cut's 0.5 s heard 21 tears in the Fork's first three minutes
/// (farm hand, seed 1), 16 of them back in the body 1-3 s later: the GPU
/// world's grid is a game second old, so 2 s is two of its readouts.
pub const SETTLE: f32 = 2.0;
/// Seconds a piece may be missing from the grid and still be the same
/// piece when it comes back (a tube thinning under the link floor for a
/// readout): the first cut heard the Fork's held ring tear twice, 8 s
/// apart, as one piece dropped out for two readouts.
pub const GHOST: f32 = 2.5;

impl Pieces {
    /// Take this update's groups. A group of `min` walkers or more and
    /// under `share` of all the walkers is a piece; a bigger one is part of
    /// the body (on the Fork the mould holds two or three parts of 8,000
    /// walkers each, apart at this grid's square, each on its own
    /// crystals: the first cut called the smaller ones pieces, and a
    /// piece's match jumping between them read as a 4,700-walker-a-second
    /// wither and a 60-cell-a-second charge). `dt` is game seconds since
    /// the last update; `soldiers` the living soldiers.
    pub fn update(&mut self, blobs: &[Blob], min: u32, share: f32, dt: f32, soldiers: &[(f32, f32)]) -> Events {
        let mut ev = Events::default();
        let dt = dt.max(1e-3);
        let Some((&body, rest)) = blobs.split_first() else {
            for p in self.pieces.drain(..) {
                if p.announced {
                    ev.gone.push((p.at, false));
                }
            }
            self.body = None;
            return ev;
        };
        let first = self.body.is_none();
        let total: u32 = blobs.iter().map(|b| b.walkers).sum();
        let max = (total as f32 * share) as u32;
        let candidates: Vec<Blob> = rest.iter().copied().filter(|b| b.walkers >= min && b.walkers < max).collect();
        let parts: Vec<Blob> = blobs.iter().copied().filter(|b| b.walkers >= max).collect();
        let mut taken = vec![false; candidates.len()];
        let mut kept = Vec::with_capacity(self.pieces.len());
        // Match each old piece to the nearest unclaimed group within its
        // reach (about its own size, at least a few squares) and of a
        // like size (a third to three times), so a match never jumps to
        // another group that happens to be near.
        let mut old = std::mem::take(&mut self.pieces);
        old.sort_by_key(|p| std::cmp::Reverse(p.walkers));
        for p in old {
            let reach = (p.squares as f32).sqrt().max(3.0) * self.bin * 1.5;
            let best = candidates.iter().enumerate().filter(|(i, c)| !taken[*i] && c.walkers * 3 >= p.walkers && c.walkers <= p.walkers.saturating_mul(3)).map(|(i, c)| (i, (c.at.0 - p.at.0).hypot(c.at.1 - p.at.1))).filter(|&(_, d)| d <= reach).min_by(|a, b| a.1.total_cmp(&b.1));
            match best {
                Some((i, _)) => {
                    taken[i] = true;
                    let c = candidates[i];
                    let k = 1.0 - (-dt / CHANGE_SMOOTH).exp();
                    let rate = (c.walkers as f32 - p.walkers as f32) / dt;
                    let change = p.change + (rate / p.walkers.max(1) as f32 - p.change) * k;
                    let losing = p.losing + ((-rate).max(0.0) - p.losing) * k;
                    let toward = nearest(soldiers, c.at).map_or(0.0, |s| {
                        let (d0, d1) = ((s.0 - p.at.0).hypot(s.1 - p.at.1), (s.0 - c.at.0).hypot(s.1 - c.at.1));
                        (d0 - d1) / dt
                    });
                    let ka = 1.0 - (-dt / APPROACH_SMOOTH).exp();
                    let approach = p.approach + (toward - p.approach) * ka;
                    kept.push(Piece { walkers: c.walkers, at: c.at, squares: c.squares, age: p.age + dt, change, losing, approach, since_tell: p.since_tell + dt, missing: 0.0, ..p });
                }
                None if p.missing + dt < GHOST => {
                    if p.missing == 0.0 && p.announced && p.age >= SETTLE && nearest(soldiers, p.at).is_some_and(|s| (s.0 - p.at.0).hypot(s.1 - p.at.1) <= self.near) {
                        ev.scattered.push(p.at);
                    }
                    kept.push(Piece { age: p.age + dt, since_tell: p.since_tell + dt, missing: p.missing + dt, ..p });
                }
                None => {
                    if p.announced {
                        // Gone next to the body and not dwindling: it
                        // rejoined; else it withered out.
                        let near = parts.iter().any(|b| (b.at.0 - p.at.0).hypot(b.at.1 - p.at.1) <= (b.squares as f32).sqrt() * self.bin * 1.2);
                        let dwindled = p.walkers < min.saturating_mul(3) / 4 || p.change < -0.05;
                        ev.gone.push((p.at, near && !dwindled));
                    }
                }
            }
        }
        // New groups: pieces from now on; a cut is heard once one settles.
        for (i, c) in candidates.iter().enumerate() {
            if !taken[i] {
                self.next += 1;
                // The pieces already there at the first look are not cuts.
                let age = if first { SETTLE } else { 0.0 };
                kept.push(Piece { id: self.next, walkers: c.walkers, at: c.at, squares: c.squares, age, announced: first, change: 0.0, losing: 0.0, approach: 0.0, slot: 0, since_tell: f32::MAX / 2.0, missing: 0.0 });
            }
        }
        for p in kept.iter_mut() {
            if !p.announced && p.heard() {
                p.announced = true;
                ev.torn.push(p.at);
            }
        }
        // Voice slots 1 ..= 3: kept by the pieces that hold them, the rest
        // go to the biggest settled pieces without one.
        kept.sort_by(|a, b| b.walkers.cmp(&a.walkers).then(a.id.cmp(&b.id)));
        let mut used = [false; 4];
        for p in kept.iter_mut() {
            if p.slot > 0 {
                used[p.slot as usize] = true;
            }
        }
        for p in kept.iter_mut().filter(|p| p.slot == 0 && p.heard()) {
            if let Some(s) = (1..4).find(|&s| !used[s]) {
                used[s] = true;
                p.slot = s as u8;
            }
        }
        self.pieces = kept;
        self.body = Some(body);
        ev
    }

    /// `bin` the grid's square and `scale` fine cells a dish cell.
    pub fn new(bin: usize, scale: f32) -> Self {
        Pieces { pieces: Vec::new(), body: None, next: 0, bin: bin as f32, near: NEAR_DISH * scale }
    }
}

fn nearest(points: &[(f32, f32)], at: (f32, f32)) -> Option<(f32, f32)> {
    points.iter().copied().min_by(|a, b| (a.0 - at.0).hypot(a.1 - at.1).total_cmp(&(b.0 - at.0).hypot(b.1 - at.1)))
}

#[cfg(test)]
mod tests {
    use super::*;

    /// A 12 x 6 grid: the body on the left, a piece on the right, a gap of
    /// one empty column between, and one diagonal touch that must not join.
    fn grid() -> (Vec<u32>, usize) {
        let cols = 12;
        let mut g = vec![0u32; cols * 6];
        for y in 0..6 {
            for x in 0..6 {
                g[y * cols + x] = 10;
            }
            for x in 7..10 {
                g[y * cols + x] = 5;
            }
        }
        (g, cols)
    }

    #[test]
    fn groups_are_four_connected_and_biggest_first() {
        let (mut g, cols) = grid();
        let b = blobs(&g, cols, 4, 1);
        assert_eq!(b.len(), 2);
        assert_eq!((b[0].walkers, b[1].walkers), (360, 90));
        assert!((b[1].at.0 - 34.0).abs() < 1e-3, "the piece's centre, fine cells: {:?}", b[1].at);
        // Grow the body into the gap's top square, which touches the
        // piece's square (7, 1) only at a corner: still two groups.
        for y in [0, 2, 3, 4, 5] {
            g[y * cols + 7] = 0;
        }
        g[6] = 1;
        assert_eq!(blobs(&g, cols, 4, 1).len(), 2, "a diagonal does not join");
        g[7] = 1;
        assert_eq!(blobs(&g, cols, 4, 1).len(), 1, "a side does");
    }

    #[test]
    fn a_piece_tears_once_settled_withers_out_and_rejoins() {
        let (g, cols) = grid();
        let mut p = Pieces::new(4, 1.0);
        let dt = 1.0 / 15.0;
        let body_only: Vec<u32> = g.iter().enumerate().map(|(i, &v)| if i % cols < 6 { v } else { 0 }).collect();
        // A piece there at the first look is not a cut: start whole.
        assert!(p.update(&blobs(&body_only, cols, 4, 1), 50, 0.5, dt, &[]).torn.is_empty());
        let mut torn = 0;
        for _ in 0..60 {
            torn += p.update(&blobs(&g, cols, 4, 1), 50, 0.5, dt, &[]).torn.len();
        }
        assert_eq!(torn, 1, "one cut, heard once it settled");
        assert_eq!(p.pieces.len(), 1);
        assert_eq!(p.pieces[0].slot, 1);
        // It withers: down to a few walkers, then gone.
        let mut w = g.clone();
        for step in 0..20 {
            for y in 0..6 {
                for x in 7..10 {
                    w[y * cols + x] = (5 - step / 4).max(1) as u32;
                }
            }
            p.update(&blobs(&w, cols, 4, 1), 10, 0.5, dt, &[]);
        }
        assert!(p.pieces[0].change < 0.0 && p.pieces[0].losing > 0.0, "{:?}", p.pieces[0]);
        // Missing for under `GHOST` it is still the same piece, unheard.
        let ev = p.update(&blobs(&body_only, cols, 4, 1), 10, 0.5, dt, &[]);
        assert!(ev.gone.is_empty() && !p.pieces[0].heard());
        let ev = gone_after(&mut p, &body_only, cols, 10, dt);
        assert_eq!(ev.gone.len(), 1);
        assert!(!ev.gone[0].1, "a dwindling piece withered out");
        // A healthy piece that vanishes next to the body rejoined it.
        let mut q = Pieces::new(4, 1.0);
        q.update(&blobs(&body_only, cols, 4, 1), 50, 0.5, dt, &[]);
        for _ in 0..60 {
            q.update(&blobs(&g, cols, 4, 1), 50, 0.5, dt, &[]);
        }
        let ev = gone_after(&mut q, &body_only, cols, 50, dt);
        assert_eq!(ev.gone, vec![(q_at(&g, cols), true)]);
        // A piece missing for a readout or two and back is the same piece:
        // no second tear.
        let mut b = Pieces::new(4, 1.0);
        b.update(&blobs(&body_only, cols, 4, 1), 50, 0.5, dt, &[]);
        let mut torn = 0;
        for i in 0..90 {
            let gr = if (40..55).contains(&i) { &body_only } else { &g };
            torn += b.update(&blobs(gr, cols, 4, 1), 50, 0.5, dt, &[]).torn.len();
        }
        assert_eq!((torn, b.pieces.len()), (1, 1));
        // A group of half the walkers or more is part of the body, never a
        // piece (the Fork's mould in two or three big parts).
        let mut r = Pieces::new(4, 1.0);
        r.update(&blobs(&body_only, cols, 4, 1), 50, 0.15, dt, &[]);
        for _ in 0..60 {
            assert!(r.update(&blobs(&g, cols, 4, 1), 50, 0.15, dt, &[]).torn.is_empty());
        }
        assert!(r.pieces.is_empty());
    }

    /// A heard piece that breaks up next to a soldier is a charge; one
    /// that breaks up far from every soldier is not.
    #[test]
    fn a_piece_that_breaks_up_by_a_soldier_charged() {
        let (g, cols) = grid();
        let body_only: Vec<u32> = g.iter().enumerate().map(|(i, &v)| if i % cols < 6 { v } else { 0 }).collect();
        for (soldier, charges) in [((40.0, 12.0), 1), ((400.0, 12.0), 0)] {
            let mut p = Pieces::new(4, 1.0);
            p.update(&blobs(&body_only, cols, 4, 1), 50, 0.5, 0.25, &[soldier]);
            for _ in 0..12 {
                p.update(&blobs(&g, cols, 4, 1), 50, 0.5, 0.25, &[soldier]);
            }
            let ev = p.update(&blobs(&body_only, cols, 4, 1), 50, 0.5, 0.25, &[soldier]);
            assert_eq!(ev.scattered.len(), charges, "soldier at {soldier:?}");
        }
    }

    /// Updates with `grid` until a piece is gone (`GHOST` and a little).
    fn gone_after(p: &mut Pieces, grid: &[u32], cols: usize, min: u32, dt: f32) -> Events {
        for _ in 0..200 {
            let ev = p.update(&blobs(grid, cols, 4, 1), min, 0.5, dt, &[]);
            if !ev.gone.is_empty() {
                return ev;
            }
        }
        Events::default()
    }

    fn q_at(g: &[u32], cols: usize) -> (f32, f32) {
        blobs(g, cols, 4, 1)[1].at
    }

    #[test]
    fn a_piece_moving_at_a_soldier_approaches() {
        let cols = 40;
        let mut p = Pieces::new(4, 1.0);
        let soldier = [(150.0, 10.0)];
        for step in 0..20 {
            let mut g = vec![0u32; cols * 4];
            for y in 0..4 {
                for x in 0..4 {
                    g[y * cols + x] = 20;
                }
                g[y * cols + 10 + step] = 30;
                g[y * cols + 11 + step] = 30;
            }
            p.update(&blobs(&g, cols, 4, 1), 50, 0.5, 0.25, &soldier);
        }
        // One square (4 fine cells) a quarter second: 16 fine cells a second.
        assert!((p.pieces[0].approach - 16.0).abs() < 2.0, "{:?}", p.pieces[0]);
    }
}
