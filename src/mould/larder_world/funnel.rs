//! **Funnelling through narrow places** (owner, 2026-11-15: "it gets
//! awkward moving with large squads through narrow areas"). A squad moving
//! as one keeps its formation round its anchor (`squads`). In a door or on
//! a bridge narrower than the formation, every place that fell in rock or
//! deep water used to fall back to the anchor itself (`Game::place_at`),
//! so half the squad walked to one point and the rest shoved: a 24-soldier
//! blob crossed an 8-cell bridge as one knot with up to 10 pairs of
//! soldiers less than half a gap apart (`narrow_tests`).
//!
//! Now a squad that sees a narrow place ahead *funnels*: its anchor goes
//! to the squad's front and the members take places in rows along the way
//! the anchor came (`Column::trail`), each row as wide as the ground there
//! allows (at most the blob's own width, about the square root of the
//! squad's size). So the squad narrows into a column at the mouth, queues
//! through two or three abreast, and widens again behind as the ground
//! opens. Once every member has a straight walk to the anchor and the
//! ground round it is wide again, it goes back to its own formation round
//! the anchor, which waits for the squad's middle to catch up
//! (`squads::LEAD`): the squad re-forms just past the gap.
//!
//! Everything is game state (a replay plays the same) and runs only while a
//! squad moves as one. Cost: a few hundred `standable` checks per moving
//! squad per step for the rows' widths.

use super::*;

/// One squad's column.
#[derive(Clone, Debug, Default)]
pub struct Column {
    /// Funnelling now.
    pub on: bool,
    /// Where the anchor has been, oldest first, fine cells, a point about
    /// every `TRAIL_STEP` dish cells (kept while the squad moves as one).
    pub trail: Vec<(f32, f32)>,
    /// Each rank's place while funnelling, dish cells from the anchor.
    pub places: Vec<(f32, f32)>,
}

/// Dish cells between trail points.
const TRAIL_STEP: f32 = 0.5;
/// Dish cells between probes across the way when measuring its width.
const PROBE: f32 = 0.5;

impl<W: World> Game<W> {
    /// Free ground across the way at `p` heading `dir` (a unit vector), to
    /// the left and right of `p`, each capped at `cap`, fine cells. Zero
    /// both ways where `p` itself can't be stood on.
    pub(super) fn across(&self, p: (f32, f32), dir: (f32, f32), cap: f32) -> (f32, f32) {
        if !self.standable(p.0, p.1) {
            return (0.0, 0.0);
        }
        let n = (-dir.1, dir.0);
        let step = PROBE * self.s();
        let side = |sgn: f32| {
            let mut d = 0.0;
            while d < cap {
                let e = (d + step).min(cap);
                if !self.standable(p.0 + n.0 * e * sgn, p.1 + n.1 * e * sgn) {
                    break;
                }
                d = e;
            }
            d
        };
        (side(1.0), side(-1.0))
    }

    /// How wide squad `q`'s own formation stands, dish cells: twice its
    /// furthest place from the anchor, plus a gap.
    pub(super) fn formation_width(&self, q: usize) -> f32 {
        let n = self.members(q).len();
        let sq = self.squads[q];
        // A column is as wide as a pair across its way, however long: it
        // files through a door as it is, with no funnel.
        let far = (0..n)
            .map(|r| sq.formation.offset(r, n, self.gap()))
            .map(|o| if sq.formation == Formation::Column { o.1.abs() } else { o.0.hypot(o.1) })
            .fold(0.0, f32::max);
        2.0 * far + self.gap()
    }

    /// Whether the way at `p`, heading `dir`, is too narrow for a
    /// formation `need` fine cells wide.
    fn narrow(&self, p: (f32, f32), dir: (f32, f32), need: f32) -> bool {
        let (l, r) = self.across(p, dir, need);
        l + r < need
    }

    /// Adds the anchor's new point `a` to squad `q`'s trail, as points
    /// `TRAIL_STEP` apart along the straight line from the last one, and
    /// drops the oldest beyond `keep` fine cells of trail.
    fn lay_trail(&mut self, q: usize, a: (f32, f32), keep: f32) {
        let step = TRAIL_STEP * self.s();
        let t = &mut self.columns[q].trail;
        match t.last().copied() {
            None => t.push(a),
            Some(l) => {
                let d = (a.0 - l.0).hypot(a.1 - l.1);
                if d >= step {
                    let k = (d / step).floor() as usize;
                    for i in 1..=k {
                        let f = (i as f32 * step) / d;
                        t.push((l.0 + (a.0 - l.0) * f, l.1 + (a.1 - l.1) * f));
                    }
                }
            }
        }
        // Keep `keep` of trail behind the newest point.
        let mut len = 0.0;
        let mut cut = 0;
        for i in (1..t.len()).rev() {
            len += (t[i].0 - t[i - 1].0).hypot(t[i].1 - t[i - 1].1);
            if len > keep {
                cut = i - 1;
                break;
            }
        }
        if cut > 0 {
            t.drain(..cut);
        }
    }

    /// The column's places for squad `q` with its head at `a`, dish cells
    /// from `a`: rows `gap` apart back along the trail, each as wide as the
    /// ground there allows (at most `cols` abreast) and centred on it.
    fn column_places(&self, q: usize, a: (f32, f32), n: usize) -> Vec<(f32, f32)> {
        let sc = self.s();
        let gap = self.gap() * sc;
        // A column (`Formation::Column`) files two abreast however wide the
        // ground; any other formation as wide as its own blob.
        let cols = if self.squads[q].formation == Formation::Column { 2.min(n) } else { ((n as f32).sqrt().ceil() as usize).max(1) };
        // The way back: the head, then the trail newest first.
        let mut way: Vec<(f32, f32)> = vec![a];
        way.extend(self.columns[q].trail.iter().rev().copied().filter(|p| (p.0 - a.0).hypot(p.1 - a.1) > 1e-3));
        let mut out = Vec::with_capacity(n);
        let (mut seg, mut along) = (0usize, 0.0f32);
        // Direction of travel at the head, for rows past the trail's end.
        let mut dir = (1.0, 0.0);
        if way.len() >= 2 {
            let (p, b) = (way[0], way[way.len().min(4) - 1]);
            if let Some(u) = unit2((p.0 - b.0, p.1 - b.1)) {
                dir = u;
            }
        } else if let Some(u) = unit2(self.squads[q].face) {
            dir = u;
        }
        let mut row = 0usize;
        let mut last = cols;
        while out.len() < n {
            // The point `row * gap` back along the way.
            let want = row as f32 * gap;
            let (p, d) = loop {
                if seg + 1 >= way.len() {
                    // Past the trail's end: straight on back.
                    let last = *way.last().unwrap();
                    let back = want - along;
                    break ((last.0 - dir.0 * back, last.1 - dir.1 * back), dir);
                }
                let (s0, s1) = (way[seg], way[seg + 1]);
                let l = (s1.0 - s0.0).hypot(s1.1 - s0.1);
                if along + l >= want {
                    let f = if l > 1e-6 { (want - along) / l } else { 0.0 };
                    let d = unit2((s0.0 - s1.0, s0.1 - s1.1)).unwrap_or(dir);
                    break ((s0.0 + (s1.0 - s0.0) * f, s0.1 + (s1.1 - s0.1) * f), d);
                }
                along += l;
                seg += 1;
                if let Some(u) = unit2((s0.0 - s1.0, s0.1 - s1.1)) {
                    dir = u;
                }
            };
            let nrm = (-d.1, d.0);
            let cap = cols as f32 * gap;
            let (l, r) = self.across(p, d, cap);
            // A row at most one wider than the row ahead of it, so the
            // squad tapers into the mouth instead of crowding it square.
            let fit = (((l + r) / gap).floor() as usize).clamp(1, cols).min(last + 1).min(n - out.len());
            last = fit;
            // Centred on the free ground across the way.
            let mid = (r - l) / 2.0;
            let c = (p.0 + nrm.0 * mid, p.1 + nrm.1 * mid);
            for j in 0..fit {
                let o = (j as f32 - (fit as f32 - 1.0) / 2.0) * gap;
                let at = (c.0 + nrm.0 * o, c.1 + nrm.1 * o);
                // A place only on the trail's side of any wall: the row's
                // middle moves by (r - l) / 2 along `nrm`, toward the side
                // with less room, so beside a wall its places could fall
                // past it, on open ground there. 60 soldiers sent out of a
                // walled base by a door 4 cells wide: the rows lay back
                // along the inside of its walls, 53 stood in their places
                // there for good and the head waited outside for them
                // (`a_squad_leaves_a_walled_base_by_its_door`; 2026-11-21).
                // Moving the middle the other way, (l - r) / 2, was tried:
                // out in 38 s against 26 with this, and a steered squad of 8
                // spread past `a_steered_squad_moves_together_and_settles_
                // when_let_go`'s bound.
                let at = if self.standable(at.0, at.1) && self.walk_line(p, at) { at } else { p };
                out.push(((at.0 - a.0) / sc, (at.1 - a.1) / sc));
            }
            row += 1;
        }
        out
    }

    /// Squad `q`'s funnel for this step (`squad_turn`), with its anchor at
    /// `at` heading for `a` (its next point) and `ahead` a point further
    /// along its way. Returns the anchor to use: on starting to funnel, the
    /// squad's front.
    pub(super) fn funnel(&mut self, q: usize, at: (f32, f32), a: (f32, f32), ahead: (f32, f32), front: (f32, f32)) -> (f32, f32) {
        let sc = self.s();
        let m = self.members(q);
        let n = m.len();
        let need = self.formation_width(q) * sc;
        let dir = unit2((ahead.0 - at.0, ahead.1 - at.1)).or_else(|| unit2((a.0 - at.0, a.1 - at.1)));
        let Some(dir) = dir else {
            self.columns[q].places.clear();
            return a;
        };
        // A column always walks as a funnel's column, two abreast along the
        // way its head went, so it snakes round rock and through doors in
        // file instead of holding a straight line that cuts the corners
        // (2026-11-21; at rest it stands as its own formation).
        let column = self.squads[q].formation == Formation::Column;
        let tight = n > 1 && (column || self.narrow(a, dir, need) || self.narrow(ahead, dir, need));
        let was = self.columns[q].on;
        let a = if tight && !was {
            // Starting: the anchor goes to the squad's front, so nobody
            // walks back to a place behind it.
            self.columns[q].on = true;
            front
        } else if !tight && was {
            // Done once every member has a straight walk to the anchor,
            // and (`Rules::steer`) is within the formation's width of it:
            // out of a straight door every member saw the anchor as soon as
            // it left, and the tail, still in the door, stood at its
            // distance from the anchor (`stand_off`) with the rest queued
            // behind (squad movement rework, 2026-11-16; 24 soldiers, a
            // 6-cell door 30 long).
            let near = |k: usize| !self.rules.steer || (self.soldiers[k].x - a.0).hypot(self.soldiers[k].y - a.1) <= need;
            if m.iter().all(|&k| near(k) && self.walk_line((self.soldiers[k].x, self.soldiers[k].y), a)) {
                self.columns[q].on = false;
            }
            a
        } else {
            a
        };
        let keep = (n as f32 * self.gap() + need / sc + 10.0) * sc;
        self.lay_trail(q, a, keep);
        self.columns[q].places = if self.columns[q].on { self.column_places(q, a, n) } else { Vec::new() };
        a
    }

    /// Squad `q` stops moving as one: no column, no trail.
    pub(super) fn funnel_stop(&mut self, q: usize) {
        let c = &mut self.columns[q];
        c.on = false;
        c.trail.clear();
        c.places.clear();
    }

    /// Soldier `k`'s column place, dish cells from its anchor, while its
    /// squad funnels.
    pub(super) fn column_place(&self, k: usize, r: usize, n: usize) -> Option<(f32, f32)> {
        let c = self.columns.get(self.soldiers[k].squad as usize)?;
        (c.on && c.places.len() == n).then(|| c.places.get(r).copied()).flatten()
    }

    /// Whether soldier `k`'s squad funnels now.
    pub(super) fn in_column(&self, k: usize) -> bool {
        self.columns.get(self.soldiers[k].squad as usize).is_some_and(|c| c.on && !c.places.is_empty())
    }
}

fn unit2(v: (f32, f32)) -> Option<(f32, f32)> {
    let d = v.0.hypot(v.1);
    (d > 1e-4).then(|| (v.0 / d, v.1 / d))
}
