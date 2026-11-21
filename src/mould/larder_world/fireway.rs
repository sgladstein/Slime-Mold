//! **A squad on a move walks round fire** (pathfinding thread, 2026-11-21;
//! `Rules::fireway`, Direction's call: a move order's way avoids burning
//! ground where there is a way round; a squad standing to fight in fire is
//! left as it is, the price of a fire deck).
//!
//! The game knows where fire is from what lit it and who felt it: a burst
//! that burns or lights (`Rounds::flush`) and a soldier with heat under it
//! (`turn.rs`) mark the spot hot for `HOT_FOR` seconds (`Game::mark_hot`).
//! The GPU reads no fire back but each soldier's heat, so marks, not the
//! fire field, serve both worlds. On a plain move (a right click, or a
//! guard's walk; not an attack-move, a drain or steering), a squad whose
//! way ahead crosses a hot spot plans its way again once a game second on
//! coarse squares (`FIRE_BIN` dish cells a side), hot ones costing
//! `FIRE_COST` times a cold one, and its anchor heads for the furthest
//! point of that way it can walk to in a straight line. Within `FIRE_NEAR`
//! of where it was sent, or where the way ahead is cold, it walks as
//! before.
//!
//! Before: with a fire lit halfway along each squad's move (the pathfinding
//! probe, `examples/pathprobe fire=6`), squads walked straight through it:
//! planet 3, run seed 1, 60 soldiers lost 1,120 health walking in fire in
//! 2 minutes. The walking maps know rock alone (`paths.rs`); they are
//! built for 30 s and shared by every squad going near the same goal, so a
//! fire that lasts seconds can't be in them.
//!
//! The same plan round the slime too (each walker on a square adding to its
//! cost) was tried and not kept: the slime comes to the squad whichever
//! way it walks (`Reports/dead-ends.md`, 2026-11-21).
use super::*;

/// Dish cells a side of the squares a way round fire is planned on.
pub(super) const FIRE_BIN: usize = 4;
/// Game seconds between a squad's plans.
pub(super) const FIRE_EVERY: f32 = 1.0;
/// Dish cells of its way ahead a squad checks for fire, and the furthest
/// it heads for at once.
pub(super) const FIRE_LOOK: f32 = 40.0;
/// Within this many dish cells of where it was sent a squad walks straight
/// in.
pub(super) const FIRE_NEAR: f32 = 10.0;
/// What a hot square costs to cross against a cold one: a way round may be
/// this many times longer than the way through (6, then 3, went further
/// round for little less fire; `plan_round_fire`).
pub(super) const FIRE_COST: f32 = 2.0;
/// How long a spot stays hot after fire was seen there, game seconds: a
/// fire on dry ground burns out in about this, and a soldier who feels it
/// still marks it again.
pub(super) const HOT_FOR: f32 = 15.0;
/// How far round a lit spot fire is taken to reach, dish cells beyond the
/// burst's own radius (fire spreads a few cells in seconds on dry ground).
pub(super) const HOT_SPREAD: f32 = 2.0;
/// Fire marks kept at once (the oldest go first).
const HOT_MAX: usize = 256;

impl<W: World> Game<W> {
    /// Fire at `p` (fine cells) within `r` (fine cells): a burn or a light
    /// went off there (`Rounds::flush`) or a soldier felt heat there
    /// (`turn.rs`). Squads on a move walk round it for `HOT_FOR` seconds.
    pub fn mark_hot(&mut self, p: (f32, f32), r: f32) {
        let now = self.world.ticks();
        let until = now + self.secs(HOT_FOR);
        let sc = self.s();
        self.paths.hot.retain(|h| h.2 > now);
        if let Some(h) = self.paths.hot.iter_mut().find(|h| (h.0 .0 - p.0).hypot(h.0 .1 - p.1) < sc && h.1 >= r) {
            h.2 = until;
            return;
        }
        if self.paths.hot.len() >= HOT_MAX {
            self.paths.hot.remove(0);
        }
        self.paths.hot.push((p, r, until));
    }

    /// Where squad `q`, its anchor at `at`, sent to `dest` on a plain
    /// move, heads this step instead of `dest`: a point on its way round
    /// fire that it can walk to in a straight line, or None to walk as
    /// before. Planned once every `FIRE_EVERY` (or when it is nearly at the
    /// last point), kept between.
    pub(super) fn fire_via(&mut self, q: usize, at: (f32, f32), dest: (f32, f32)) -> Option<(f32, f32)> {
        if !self.rules.fireway {
            return None;
        }
        let now = self.world.ticks();
        let sq = self.squads[q];
        let near = |v: (f32, f32)| (v.0 - at.0).hypot(v.1 - at.1) < FIRE_BIN as f32 * self.s();
        if sq.via_at.is_some_and(|t| now < t + self.secs(FIRE_EVERY)) && sq.via_to == Some(dest) && !sq.via.is_some_and(near) {
            return sq.via.filter(|&v| self.walk_line(at, v));
        }
        let via = self.plan_round_fire(q, at, dest);
        let s = &mut self.squads[q];
        s.via = via;
        s.via_at = Some(now);
        s.via_to = Some(dest);
        via
    }

    /// Forget squad `q`'s way round fire (it was steered, attack-moved, or
    /// stopped).
    pub(super) fn drop_via(&mut self, q: usize) {
        let s = &mut self.squads[q];
        s.via = None;
        s.via_at = None;
        s.via_to = None;
    }

    /// Which squares a soldier can mostly stand on (half their cells or
    /// more), kept until the walking maps go stale (`stale_maps`).
    fn coarse_open(&mut self, cw: usize, ch: usize, side: usize) -> Vec<bool> {
        if let Some(c) = self.paths.coarse.as_ref().filter(|c| c.len() == cw * ch) {
            return c.clone();
        }
        let (w, _) = self.world.size();
        let mut n = vec![0u32; cw * ch];
        for (i, ok) in self.standable_cells().into_iter().enumerate() {
            if ok {
                n[(i / w / side) * cw + (i % w) / side] += 1;
            }
        }
        let open: Vec<bool> = n.iter().map(|&k| 2 * k as usize >= side * side).collect();
        self.paths.coarse = Some(open.clone());
        open
    }

    /// The point `fire_via` heads for: None while no hot spot lies on the
    /// straight way ahead, within `FIRE_NEAR` of `dest`, or when no point of
    /// the planned way is in a straight walk.
    fn plan_round_fire(&mut self, q: usize, at: (f32, f32), dest: (f32, f32)) -> Option<(f32, f32)> {
        let sc = self.s();
        let d = (dest.0 - at.0).hypot(dest.1 - at.1);
        let now = self.world.ticks();
        if d < FIRE_NEAR * sc || self.paths.hot.iter().all(|h| h.2 <= now) {
            return None;
        }
        let (w, h) = self.world.size();
        let side = FIRE_BIN * (sc as usize).max(1);
        let sf = side as f32;
        let (cw, ch) = (w.div_ceil(side), h.div_ceil(side));
        // Hot squares: their middles within a mark's reach plus a quarter
        // of the squad's width, so its flank keeps clear of most of the
        // fire as its middle goes by. Half its width plus half a square,
        // with a hot square costing 3, made a 12-cell fire a 34-cell
        // detour for 30 soldiers: on the 24 maps fire on the way was 248
        // against 274 now, but trips took 11.4 s against 10.6 (as off),
        // and in the real game's planet 1 tour 12 s against 7. Half its
        // width with no half square came to 282, no better than a quarter.
        // A quarter lets the flank of a squad of 15 brush a fire's edge
        // (its nearest member 4.5 cells from a 6-cell fire's middle).
        let half = 0.25 * self.formation_width(q) * sc;
        let mut hot = vec![false; cw * ch];
        for &(p, r, until) in &self.paths.hot {
            if until <= now {
                continue;
            }
            let r = r + HOT_SPREAD * sc + half;
            let (x0, x1) = (((p.0 - r).max(0.0) / sf) as usize, (((p.0 + r) / sf) as usize).min(cw - 1));
            let (y0, y1) = (((p.1 - r).max(0.0) / sf) as usize, (((p.1 + r) / sf) as usize).min(ch - 1));
            for y in y0..=y1 {
                for x in x0..=x1 {
                    let c = ((x as f32 + 0.5) * sf, (y as f32 + 0.5) * sf);
                    if (c.0 - p.0).hypot(c.1 - p.1) <= r {
                        hot[y * cw + x] = true;
                    }
                }
            }
        }
        let sq_of = |p: (f32, f32)| ((p.1.max(0.0) / sf) as usize).min(ch - 1) * cw + ((p.0.max(0.0) / sf) as usize).min(cw - 1);
        let centre = |c: usize| (((c % cw) as f32 + 0.5) * sf, ((c / cw) as f32 + 0.5) * sf);
        // How many hot squares a straight line from `a` to `b` crosses.
        let crosses = |a: (f32, f32), b: (f32, f32)| {
            let l = (b.0 - a.0).hypot(b.1 - a.1);
            let n = (l / (0.5 * sf)).ceil().max(1.0) as usize;
            let mut seen = usize::MAX;
            let mut k = 0;
            for i in 0..=n {
                let t = i as f32 / n as f32;
                let c = sq_of((a.0 + (b.0 - a.0) * t, a.1 + (b.1 - a.1) * t));
                if c != seen && hot[c] {
                    k += 1;
                }
                seen = c;
            }
            k
        };
        let look = (FIRE_LOOK * sc).min(d);
        let ahead = (at.0 + (dest.0 - at.0) / d * look, at.1 + (dest.1 - at.1) / d * look);
        if crosses(at, ahead) == 0 {
            return None;
        }
        let open = self.coarse_open(cw, ch, side);
        let (s0, g0) = (sq_of(at), sq_of(dest));
        let ok = |c: usize| open[c] || c == s0 || c == g0;
        let cost = |c: usize| if hot[c] { FIRE_COST } else { 1.0 };
        let mut dist = vec![u32::MAX; cw * ch];
        let mut from = vec![usize::MAX; cw * ch];
        let mut heap = std::collections::BinaryHeap::new();
        dist[s0] = 0;
        heap.push(std::cmp::Reverse((0u32, s0)));
        while let Some(std::cmp::Reverse((dc, c))) = heap.pop() {
            if c == g0 {
                break;
            }
            if dc > dist[c] {
                continue;
            }
            let (x, y) = ((c % cw) as i64, (c / cw) as i64);
            for (dx, dy) in [(-1i64, 0i64), (1, 0), (0, -1), (0, 1), (-1, -1), (1, -1), (-1, 1), (1, 1)] {
                let (nx, ny) = (x + dx, y + dy);
                if nx < 0 || ny < 0 || nx >= cw as i64 || ny >= ch as i64 {
                    continue;
                }
                let j = ny as usize * cw + nx as usize;
                let diag = dx != 0 && dy != 0;
                if !ok(j) || (diag && (!ok(y as usize * cw + nx as usize) || !ok(ny as usize * cw + x as usize))) {
                    continue;
                }
                let nd = dc + ((if diag { 14.0 } else { 10.0 }) * cost(j)).round() as u32;
                if nd < dist[j] {
                    dist[j] = nd;
                    from[j] = c;
                    heap.push(std::cmp::Reverse((nd, j)));
                }
            }
        }
        if dist[g0] == u32::MAX {
            return None;
        }
        let mut route = vec![g0];
        while let Some(&c) = route.last().filter(|&&c| c != s0) {
            route.push(from[c]);
        }
        route.reverse();
        // The furthest point of the way within `FIRE_LOOK` that is a
        // straight walk and crosses no more hot squares than the way to it
        // (a straight cut across the corner of the fire it goes round would
        // walk into it). Where the way must cross fire (it lies round where
        // the squad was sent), the cut is held to that too: with "any line
        // once the way is hot" (the first version), squads sent between
        // close crystals with fire between them took the straight line and
        // burned on planet 1 of the real game (2026-11-21).
        //
        // The next square's point is taken even when the line to it clips
        // a hot square's corner: from where the anchor stands in its square
        // that is all but the way, and with none taken the squad walked
        // straight at where it was sent, into the fire (planet 1 film,
        // 2026-11-21: it stopped going round for 2 s at the fire's edge).
        let mut via = None;
        let mut walked = 0.0;
        let mut prev = at;
        let mut way_hot = hot[s0] as usize;
        for (i, &c) in route[1..].iter().enumerate() {
            let p = if c == g0 { dest } else { self.nearest_standable(centre(c), 0.5 * sf).unwrap_or(centre(c)) };
            walked += (p.0 - prev.0).hypot(p.1 - prev.1);
            prev = p;
            if walked > FIRE_LOOK * sc {
                break;
            }
            way_hot += hot[c] as usize;
            if !self.walk_line(at, p) || (i > 0 && crosses(at, p) > way_hot) {
                break;
            }
            via = Some(p);
        }
        via
    }
}
