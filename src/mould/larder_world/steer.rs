//! **How soldiers walk in the squad mode** (squad movement rework,
//! 2026-11-16; owner 05:17Z: "It is still clunky overall", start over).
//!
//! **What was wrong.** A soldier had no speed of its own: each step it
//! walked full speed straight at its place and stood dead once there, and
//! elbow room then shoved it sideways (`Game::elbow_room`). A squad's
//! anchor walks at three quarters of a soldier's speed, so every member
//! dashed to its place, stopped, and dashed again; at rock its place fell
//! back to the anchor, and members whose place could not be seen asked a
//! walking map for a place that moved every step and stood still while it
//! was built (`move_tests`: 40 soldiers at the 6-cell door spent 13% of
//! their steps standing, with 190 waits on a map, and turned 17 degrees a
//! step). Past rock they walked from cell to cell, so every way was a
//! zig-zag of 45-degree legs.
//!
//! **How the good RTS games do it** (the squad movement report's sources):
//! a group's plan is kept apart from each unit's walk. The group moves a
//! formation along one path at the pace of its slowest (Pottinger's
//! *Coordinated Unit Movement* for Age of Empires); every unit reads its
//! way from one field built per goal (flow fields, Emerson's *Crowd
//! Pathfinding and Steering Using Flow Field Tiles* for Supreme Commander
//! 2), smoothed to any angle; each has a velocity it changes by steering
//! (Reynolds' seek, arrive, separation and queuing), so it speeds up,
//! slows into its place and never walks into the back of the one ahead.
//!
//! **What a soldier does now** (`Rules::steer`, on), each step:
//!
//! 1. **Want.** Its goal as before (`Game::goal_ground`). Walking with a
//!    squad that moves as one, it takes the anchor's velocity and closes on
//!    its place from there (`CATCH`): in step, not dash-and-stop. A place
//!    in rock or out of the anchor's sight is a distance from the anchor
//!    instead, its place's own (`Game::stand_off`), so the squad packs
//!    round whatever ground is free instead of piling onto one point. With
//!    no straight walk it follows the squad's own walking map
//!    (`Squad::map`), never one of its own, and holds back if that puts it
//!    ahead of the anchor. Alone, it slows into its goal (`ARRIVE`).
//! 2. **Way.** Down a walking map it heads for the furthest of the next
//!    `LOOK` cells it can walk straight to (`Game::ahead_on_map`): any
//!    angle, not 45-degree legs.
//! 3. **Room.** Two soldiers closer than elbow room steer apart (`SPREAD`),
//!    and a soldier walking into a slower one ahead brakes to its pace
//!    (queuing), so a squad at a door queues instead of shoving. One coming
//!    at it, or another squad's soldier standing in its way, it turns aside
//!    round; one of its own standing in its way gives way while the squad
//!    walks.
//! 4. **Speed.** Its velocity turns toward what it wants at most a full
//!    speed in `RISE` seconds, never past its wading speed, and it slides
//!    along rock and round a crystal's rim.
//!
//! The window draws each soldier between where its walk began and where it
//! ended (`Soldier::was`, `Game::drawn_between`).

use super::*;

/// Game seconds a soldier takes to reach full speed from a stand, or to
/// stop from it.
pub const RISE: f32 = 0.2;
/// How hard a member walking with its squad closes on its place: a
/// second's walk a cell short, times this.
pub const CATCH: f32 = 2.5;
/// How a lone soldier slows into its goal: speed at most this times the
/// distance left, a game second.
pub const ARRIVE: f32 = 4.0;
/// How hard two soldiers inside elbow room steer apart: the overlap times
/// this, a game second.
pub const SPREAD: f32 = 5.0;
/// Cells down a walking map a soldier looks for the furthest it can walk
/// straight to.
pub const LOOK: usize = 10;
/// Game seconds of its own pace over which a sent squad's anchor slows
/// into its goal (`Game::squad_turn`).
pub const EASE: f32 = 0.5;
/// A standing soldier this share of elbow room from where it heads is
/// there: it stops pulling, so elbow room alone settles a packed squad.
pub const SETTLE: f32 = 0.35;

fn len(v: (f32, f32)) -> f32 {
    v.0.hypot(v.1)
}

fn cap(v: (f32, f32), m: f32) -> (f32, f32) {
    let l = len(v);
    if l > m && l > 0.0 {
        (v.0 * m / l, v.1 * m / l)
    } else {
        v
    }
}

impl<W: World> Game<W> {
    /// The point to head for down walking map `key` from `from`: the
    /// furthest of the next `LOOK` cells downhill that is a straight walk
    /// away (three looks first, then nearer ones), else the middle of its
    /// own cell. None off the map.
    pub(super) fn ahead_on_map(&self, key: usize, from: (f32, f32)) -> Option<(f32, f32)> {
        let (w, h) = self.world.size();
        let map = &self.paths.maps.get(&key)?.0;
        let mut at = onto_map(w, h, map, cell(w, h, from))?;
        let mut way = Vec::with_capacity(LOOK);
        for _ in 0..LOOK {
            let n = downhill(w, h, map, at, self.rules.corners);
            if n == at {
                break;
            }
            way.push(n);
            at = n;
        }
        let mid = |i: usize| ((i % w) as f32 + 0.5, (i / w) as f32 + 0.5);
        if way.is_empty() {
            // At the map's goal cell.
            return Some(mid(at));
        }
        for i in [way.len() - 1, way.len() * 2 / 3, way.len() / 3] {
            let c = mid(way[i]);
            if self.walk_line(from, c) {
                return Some(c);
            }
        }
        // None of those: the nearer cells in turn, then the middle of its
        // own cell. Heading for the next cell regardless walked soldiers on
        // a cell's edge into a crystal's rim, where they stood.
        for i in (0..way.len() / 3).rev() {
            let c = mid(way[i]);
            if self.walk_line(from, c) {
                return Some(c);
            }
        }
        let own = mid(cell(w, h, from));
        Some(if self.walk_line(from, own) { own } else { mid(way[0]) })
    }

    /// How far soldier `k` keeps from its squad's anchor while its place is
    /// in rock or out of the anchor's sight: its place's own distance from
    /// it, fine cells, so the squad packs round the free ground as its
    /// formation would.
    fn stand_off(&self, k: usize) -> f32 {
        let o = self.place(k);
        len(o) * self.s()
    }

    /// Where soldier `k` heads round its squad's anchor at `a`, fine cells:
    /// its place if it can stand there and the anchor can see it, else its
    /// place's distance from the anchor on its own side (`stand_off`).
    fn aim_round(&self, k: usize, a: (f32, f32)) -> (f32, f32) {
        let sc = self.s();
        let o = self.place(k);
        let slot = (a.0 + o.0 * sc, a.1 + o.1 * sc);
        if self.standable(slot.0, slot.1) && (self.in_column(k) || self.walk_line(a, slot)) {
            return slot;
        }
        let s = self.soldiers[k];
        let (dx, dy) = (s.x - a.0, s.y - a.1);
        let d = dx.hypot(dy);
        let off = self.stand_off(k).min(d);
        if d > 1e-4 {
            (a.0 + dx / d * off, a.1 + dy / d * off)
        } else {
            a
        }
    }

    /// How far squad `q`'s members' middle stands from its anchor at `at`
    /// once each is where it heads (`aim_round`), fine cells: what the
    /// anchor measures its lead from (`squad_turn`), so it waits for its
    /// squad only while members are short of where they head. Measured
    /// from the formation's own places (`shape_off`), members kept off
    /// rock at their distance stood behind those places for good, and the
    /// anchor waited for them for good (planet 1, 40 soldiers, from 17 s).
    pub(super) fn steer_rest(&self, q: usize, at: (f32, f32)) -> (f32, f32) {
        let m = self.members(q);
        let Some(mid) = self.middle(q) else { return (0.0, 0.0) };
        let stray = super::squads::STRAY * self.s();
        let near: Vec<usize> = m.iter().copied().filter(|&k| (self.soldiers[k].x - mid.0).hypot(self.soldiers[k].y - mid.1) <= stray).collect();
        let near = if near.is_empty() { m } else { near };
        let (x, y) = near.iter().map(|&k| self.aim_round(k, at)).fold((0.0, 0.0), |s, p| (s.0 + p.0 - at.0, s.1 + p.1 - at.1));
        let n = near.len().max(1) as f32;
        (x / n, y / n)
    }

    /// How far squad `q`'s members are, on average, from where they head
    /// round an anchor at `at` (`aim_round`), fine cells: a squad can stand
    /// with its middle in place and its members out of theirs (a line of
    /// 40 formed from a blob as it walked and arrived with members 5 cells
    /// off their places, 4 s to stand).
    pub(super) fn steer_short(&self, q: usize, at: (f32, f32)) -> f32 {
        let m = self.members(q);
        let Some(mid) = self.middle(q) else { return 0.0 };
        let stray = super::squads::STRAY * self.s();
        let (mut d, mut n) = (0.0, 0.0);
        for &k in &m {
            let s = self.soldiers[k];
            if (s.x - mid.0).hypot(s.y - mid.1) <= stray {
                let a = self.aim_round(k, at);
                d += (s.x - a.0).hypot(s.y - a.1);
                n += 1.0;
            }
        }
        if n > 0.0 { d / n } else { 0.0 }
    }

    /// The walking map to where squad `q` goes as one when its anchor took
    /// none this step (`Game::guide`): a sent squad's goal, or a steered
    /// one's anchor (any map within `STEER_NEAR` of either). None while it
    /// is built.
    fn squad_way(&mut self, q: usize) -> Option<usize> {
        let sq = self.squads[q];
        let sc = self.s();
        let a = sq.anchor?;
        // Steered, its members' way is to the anchor itself: asked only
        // when the anchor walks straight or slides along rock (on a map of
        // its own it lends that one, `Squad::map`), where the map to where
        // it heads can lead another way round. Down that one, a member
        // behind a crystal from its anchor, held at the wall, counted
        // itself nearer than the anchor and stood 60 s 28 cells off
        // (pathfinding thread, 2026-11-20: planet 3, run seed 1).
        let (t, near) = if sq.steer != (0.0, 0.0) { (a, STEER_NEAR * sc) } else { (sq.dest?, MAP_NEAR * sc) };
        let (w, h) = self.world.size();
        self.map_for(cell(w, h, t), t, near)
    }

    /// What soldier `k` wants to do this step: a velocity, fine cells a
    /// game second, or None to walk the old way (caught on a crystal or
    /// shut in rock: straight out, over anything, as `walk` does).
    fn want(&mut self, k: usize) -> Option<(f32, f32)> {
        let s = self.soldiers[k];
        let sc = self.s();
        if self.solid_at((s.x, s.y)).is_some() || !self.ground(s.x, s.y) {
            return None;
        }
        let max = self.rules.speed * sc * self.wade(k);
        if self.engaged(k) {
            return Some((0.0, 0.0));
        }
        let Some((g, moved)) = self.goal_ground(k) else { return Some((0.0, 0.0)) };
        let p = (s.x, s.y);
        // A move or a fight in the squad mode stands round its point in
        // formation (`Game::place_at`), whether its squad walks there as
        // one (the point is the anchor, moving) or stands there.
        let centre = match s.order {
            Order::Move(x, y) | Order::Fight(x, y) if self.rules.squads && !s.cover => Some((x, y)),
            _ => None,
        };
        if let Some(a) = centre {
            return Some(self.want_round(k, a, g, max));
        }
        let d = len((g.0 - p.0, g.1 - p.1));
        // A goal moved off rock is one cell for every place in the rock:
        // near it is there (as `walk`).
        let there = if moved { (self.rules.elbow * self.rules.touch * sc).max(0.5) } else { 0.5 };
        if d < there {
            return Some((0.0, 0.0));
        }
        let t = if self.walk_line(p, g) {
            g
        } else {
            let (w, h) = self.world.size();
            let Some(key) = self.map_for(cell(w, h, g), g, MAP_NEAR * sc) else {
                // Its map is being built (`pump_maps`): wait for it.
                self.map_waits += 1;
                return Some((0.0, 0.0));
            };
            match self.ahead_on_map(key, p) {
                Some(t) if t != g && len((t.0 - p.0, t.1 - p.1)) > 1e-3 => t,
                // At the map's goal, or off it: straight on.
                _ => g,
            }
        };
        let e = (t.0 - p.0, t.1 - p.1);
        let l = len(e).max(1e-6);
        // Slows into the goal itself, not into a way point.
        let sp = if t == g { (ARRIVE * (d - 0.5 * there)).clamp(0.0, max) } else { max };
        Some((e.0 / l * sp, e.1 / l * sp))
    }

    /// `want` for soldier `k` standing in formation round point `a` (its
    /// squad's anchor while the squad walks as one, else where it was
    /// sent), heading for `g` (`goal_ground`), at most `max`.
    fn want_round(&mut self, k: usize, a: (f32, f32), g: (f32, f32), max: f32) -> (f32, f32) {
        let s = self.soldiers[k];
        let sc = self.s();
        let p = (s.x, s.y);
        let q = s.squad as usize;
        let sq = self.squads[q];
        let moving = sq.anchor == Some(a);
        let vel = if moving { sq.vel } else { (0.0, 0.0) };
        let o = self.place(k);
        let slot = (a.0 + o.0 * sc, a.1 + o.1 * sc);
        let free = g == slot && self.standable(slot.0, slot.1);
        if free && self.walk_line(p, slot) {
            // Standing: there once within a third of elbow room, or a
            // neighbour's elbow room and its own pull took turns pushing
            // it back and forth for good (planet 8, 200 soldiers).
            if !moving && len((slot.0 - p.0, slot.1 - p.1)) < SETTLE * self.rules.elbow * self.rules.touch * sc {
                return (0.0, 0.0);
            }
            // In step with the anchor, closing on its place.
            return cap((vel.0 + CATCH * (slot.0 - p.0), vel.1 + CATCH * (slot.1 - p.1)), max);
        }
        let off = self.stand_off(k);
        let to_a = (a.0 - p.0, a.1 - p.1);
        let d = len(to_a);
        // Its place is open and the anchor sees it, but it doesn't: toward
        // the anchor until it does, with no stand-off (`aim_round` counts
        // it as on its way to its place). Held at its stand-off instead,
        // the squad's tail stood in a 6-cell door once its column ended
        // while the anchor, measuring them short of their places, waited
        // for them (24 soldiers, door 30 cells long).
        if !free && self.walk_line(p, a) && self.standable(slot.0, slot.1) && self.walk_line(a, slot) {
            return cap((vel.0 + CATCH * to_a.0, vel.1 + CATCH * to_a.1), max);
        }
        // Its place is in rock or out of the anchor's sight: keep its
        // place's distance from the point, on whatever side is free, so
        // the squad packs round the free ground as its formation would
        // instead of piling onto the point.
        if !free && self.walk_line(p, a) {
            let gap = d - off;
            if gap <= if moving { 0.0 } else { SETTLE * self.rules.elbow * self.rules.touch * sc } {
                return cap(vel, max);
            }
            return cap((vel.0 + CATCH * to_a.0 / d * gap, vel.1 + CATCH * to_a.1 / d * gap), max);
        }
        // Round rock: down a walking map, held back once it is no further
        // from the point than its distance. While the squad walks as one,
        // the squad's own map (or the map to where it goes, asked once for
        // the whole squad): a map of its own to the moving anchor was a new
        // map every few steps and a wait for each (planet 1, 40 soldiers:
        // 60 waits). Standing, the map to the point, one for the squad (a
        // map to each place was a wait each: 130 more as 200 soldiers stood
        // round a crystal on planet 8), its place's distance short of it;
        // once its place is in sight it walks straight there, above.
        let (w, h) = self.world.size();
        let key = if moving {
            match sq.map {
                Some(key) => Some(key),
                None => {
                    let key = self.squad_way(q);
                    self.squads[q].map = key;
                    key
                }
            }
        } else {
            self.map_for(cell(w, h, a), a, MAP_NEAR * sc)
        };
        let Some(key) = key else {
            // The map is being built: toward the point along the rock.
            self.map_waits += 1;
            return cap((to_a.0 / d.max(1e-6) * max, to_a.1 / d.max(1e-6) * max), max);
        };
        let Some(t) = self.ahead_on_map(key, p) else {
            return cap((to_a.0 / d.max(1e-6) * max, to_a.1 / d.max(1e-6) * max), max);
        };
        let map = &self.paths.maps[&key].0;
        let (mine, theirs) = (map[cell(w, h, p)], map[cell(w, h, a)]);
        // Map distances are 5 a side step.
        let behind = if theirs == u32::MAX { f32::MAX } else { (mine as f32 - theirs as f32) / 5.0 };
        // No nearer the point than its straight distance, unless it is
        // ahead of a walking squad's anchor. The map's own goal may be any
        // point within `MAP_NEAR` of the point (standing) or far on along
        // the way (walking), and the map's difference alone, which is no
        // more than its walk to the point, can read a soldier far off as
        // near: one 17 cells from its standing squad's point behind a
        // crystal read itself 1.3 cells behind, under its place's 8, and
        // stood 25 s; one 31 cells behind its steered squad's anchor, round
        // a crystal from it, read 3.7 and stood 77 s (pathfinding thread,
        // 2026-11-20: planet 3, run seed 1). Ahead of a walking anchor
        // (along its velocity) the map's reading stands, so a member beside
        // the way doesn't walk on ahead of its squad.
        let ahead = (p.0 - a.0) * vel.0 + (p.1 - a.1) * vel.1 > 0.0;
        let behind = if ahead { behind } else { behind.max(d) };
        let sp = (len(vel) + CATCH * (behind - off)).clamp(0.0, max);
        let e = (t.0 - p.0, t.1 - p.1);
        let l = len(e).max(1e-6);
        (e.0 / l * sp, e.1 / l * sp)
    }

    /// How fast squad `q`'s members walk on average, each over its own top
    /// speed in the slime it stands in (`Game::wade`): near nothing once
    /// they stand, or once they are jammed. Members fighting where they
    /// stand (`Game::engaged`) are left out, and a squad all fighting
    /// counts as walking: over full speed with them in, a squad wading or
    /// fighting read as jammed, so its anchor walked on and "arrived" and
    /// its next order started with the fight behind it (review,
    /// 2026-11-16).
    pub(super) fn squad_speed(&self, q: usize) -> f32 {
        let full = self.rules.speed * self.s();
        let (mut v, mut n) = (0.0, 0.0);
        for k in 0..self.soldiers.len() {
            let s = self.soldiers[k];
            if !s.alive || s.squad as usize != q || self.engaged(k) {
                continue;
            }
            v += s.vx.hypot(s.vy) / (full * self.wade(k)).max(1e-6);
            n += 1.0;
        }
        if n > 0.0 { v / n } else { 1.0 }
    }

    /// Every living soldier's walk this step in the squad mode
    /// (`Rules::steer`): what it wants, elbow room and queuing, then its
    /// velocity eased toward that and walked on standable ground. In place
    /// of `elbow_room` and `walk`. Pairs are all pairs (200 soldiers are
    /// 19,900, a few microseconds); a grid would pay past a thousand.
    pub(super) fn steer_all(&mut self, dt: f32) {
        let r = self.rules;
        let sc = self.s();
        let n = self.soldiers.len();
        let full = r.speed * sc;
        let min = r.elbow * r.touch * sc;
        // Queuing behind a soldier that stands reaches a quarter of elbow
        // room past it, so one walking up stops short instead of walking
        // in and being pushed out, step after step, for good (planet 8:
        // two of 40 soldiers did so for 20 s after arriving). Behind one
        // that walks it starts at elbow room: the longer reach there too
        // braked squads walking in step (steer 200 jerk 0.09 -> 0.11, a
        // 6-cell door's 40 walked back 0.18 -> 0.71 cells on arriving).
        let reach = 1.25 * min;
        let stood = 0.1 * full;
        let mut want: Vec<Option<(f32, f32)>> = vec![Some((0.0, 0.0)); n];
        // Where each soldier of a squad standing in formation walks to:
        // its place, for the queuing below.
        let mut place: Vec<Option<(f32, f32)>> = vec![None; n];
        for (k, w) in want.iter_mut().enumerate() {
            self.soldiers[k].was = (self.soldiers[k].x, self.soldiers[k].y);
            if self.soldiers[k].alive {
                *w = self.want(k);
                let s = self.soldiers[k];
                if let Order::Move(x, y) | Order::Fight(x, y) = s.order {
                    let still = self.squads.get(s.squad as usize).is_some_and(|sq| sq.anchor != Some((x, y)));
                    if r.squads && !s.cover && still {
                        let o = self.place(k);
                        place[k] = Some((x + o.0 * sc, y + o.1 * sc));
                    }
                }
            }
        }
        // Elbow room and queuing, from where everyone stands before the
        // pass, so the order soldiers are visited in changes nothing.
        let mut push = vec![(0.0f32, 0.0f32); n];
        let mut brake = vec![(0.0f32, 0.0f32); n];
        if min > 0.0 {
            for i in 0..n {
                let a = self.soldiers[i];
                if !a.alive {
                    continue;
                }
                for j in i + 1..n {
                    let b = self.soldiers[j];
                    if !b.alive {
                        continue;
                    }
                    let (dx, dy) = (a.x - b.x, a.y - b.y);
                    if dx.abs() >= reach || dy.abs() >= reach {
                        continue;
                    }
                    let d = dx.hypot(dy);
                    if d >= reach {
                        continue;
                    }
                    let u = if d > 1e-4 {
                        (dx / d, dy / d)
                    } else {
                        let t = (i * 31 + j) as f32 * 2.399_963;
                        (t.cos(), t.sin())
                    };
                    // Overlaps under a twentieth of elbow room don't push:
                    // a packed squad at rest stands still instead of
                    // trembling.
                    let f = SPREAD * (min - d - 0.05 * min).max(0.0);
                    push[i].0 += u.0 * f;
                    push[i].1 += u.1 * f;
                    push[j].0 -= u.0 * f;
                    push[j].1 -= u.1 * f;
                    // Queuing: what each wants toward the other, past the
                    // other's own pace that way, is taken off: behind one
                    // standing, in full at elbow room and none at `reach`;
                    // behind one walking, none at elbow room and in full a
                    // third of it closer. Never more than it wants that
                    // way, so nobody walks backwards from one coming at it.
                    // What is taken off turns aside instead, round the side
                    // it wanted to pass on (head on: its right), when the
                    // other walks at it, or stands and is another squad's:
                    // braking alone, two squads meeting on the 8-cell bridge
                    // stood face to face for 14 s (review, 2026-11-16).
                    // Behind its own squad's standing soldiers it waits: at
                    // rest a packed squad turning aside milled round
                    // itself, and walking, at a 6-cell door, it spread along
                    // the wall instead of queuing. While the squad walks as
                    // one the standing one gives way, walking on the way the
                    // one behind wants: members held at their distance from
                    // the anchor (`stand_off`) in a 6-cell door plugged it
                    // once its column ended, and those behind them waited
                    // for good (24 soldiers, door 30 cells long). So it
                    // does for one on a job (building, guarding: not
                    // walking to a formation place): a builder stood behind
                    // a squad-mate on the wall's home side, in the wall's
                    // footprint, until the wall shut it in (wall sides,
                    // seed 3).
                    // Walking to its place in a squad that stands, it
                    // doesn't queue behind one its straight walk there
                    // passes clear of: a line's places stand a gap apart,
                    // inside a standing neighbour's queuing reach, and its
                    // members crept the last cell into them, stopped by
                    // their neighbours, for 3 s (a line of 40 arriving).
                    let clear = |me: usize, o: (f32, f32)| {
                        place[me].is_some_and(|t| {
                            let a = self.soldiers[me];
                            let (vx, vy) = (t.0 - a.x, t.1 - a.y);
                            let l2 = (vx * vx + vy * vy).max(1e-6);
                            let f = (((o.0 - a.x) * vx + (o.1 - a.y) * vy) / l2).clamp(0.0, 1.0);
                            (a.x + vx * f - o.0).hypot(a.y + vy * f - o.1) >= 0.95 * min
                        })
                    };
                    for (me, other, toward) in [(i, j, (-u.0, -u.1)), (j, i, u)] {
                        let Some(wv) = want[me] else { continue };
                        let o = self.soldiers[other];
                        let still = o.vx.hypot(o.vy) < stood;
                        if still && clear(me, (o.x, o.y)) {
                            continue;
                        }
                        let deep = (if still { (reach - d) / (reach - min) } else { (min - d) / (0.3 * min) }).clamp(0.0, 1.0);
                        let mine = wv.0 * toward.0 + wv.1 * toward.1;
                        let theirs = o.vx * toward.0 + o.vy * toward.1;
                        let rel = (mine - theirs).min(mine);
                        if rel > 0.0 {
                            brake[me].0 -= toward.0 * rel * deep;
                            brake[me].1 -= toward.1 * rel * deep;
                            let same = o.squad == self.soldiers[me].squad;
                            if still && same && (place[me].is_none() || self.squads.get(o.squad as usize).is_some_and(|sq| sq.anchor.is_some())) {
                                push[other].0 += toward.0 * rel * deep;
                                push[other].1 += toward.1 * rel * deep;
                            }
                            if theirs < -stood || still && !same {
                                let past = wv.0 * toward.1 - wv.1 * toward.0;
                                let side = if past > 1e-3 * full { -1.0 } else { 1.0 };
                                brake[me].0 -= side * toward.1 * rel * deep;
                                brake[me].1 += side * toward.0 * rel * deep;
                            }
                        }
                    }
                }
            }
        }
        for k in 0..n {
            if !self.soldiers[k].alive {
                continue;
            }
            let Some(wv) = want[k] else {
                // Caught on a crystal or shut in rock: out the old way.
                self.walk(k, dt);
                self.soldiers[k].vx = 0.0;
                self.soldiers[k].vy = 0.0;
                continue;
            };
            let s = self.soldiers[k];
            let wade = self.wade(k);
            let target = (wv.0 + brake[k].0 + push[k].0, wv.1 + brake[k].1 + push[k].1);
            // Elbow room may move a soldier at half speed even in thick
            // slime, so a knot there still opens (as `elbow_room`).
            let top = full * wade.max(if len(push[k]) > 0.0 { 0.5 } else { 0.0 });
            let target = cap(target, top);
            let dv = cap((target.0 - s.vx, target.1 - s.vy), full / RISE * dt);
            let mut v = cap((s.vx + dv.0, s.vy + dv.1), top);
            // No creeping: wanting to stand and nearly stood, it stands.
            if len(target) < 0.05 * full && len(v) < 0.15 * full {
                v = (0.0, 0.0);
            }
            let (nx, ny) = (s.x + v.0 * dt, s.y + v.1 * dt);
            // Along rock: the way that is open, and no speed into the rock.
            let (x, y) = if self.standable(nx, ny) {
                (nx, ny)
            } else if self.standable(nx, s.y) {
                v.1 = 0.0;
                (nx, s.y)
            } else if self.standable(s.x, ny) {
                v.0 = 0.0;
                (s.x, ny)
            } else if let Some(t) = self.solid_at((nx, ny)).map(|(c, _)| (s.y - c.1, c.0 - s.x)).filter(|_| len(v) > 0.0) {
                // Into a crystal's disc both ways along the grid: round
                // its rim instead, the way it was going. Along the grid
                // alone it stopped dead at the rim, and members of a squad
                // of 8 stood there for good (`a_squad_sent_past_a_dead_end
                // _goes_round_it`, seed 4).
                let side = if v.0 * t.0 + v.1 * t.1 >= 0.0 { 1.0 } else { -1.0 };
                let u = (t.0 * side / len(t).max(1e-6), t.1 * side / len(t).max(1e-6));
                let sp = len(v);
                let (tx, ty) = (s.x + u.0 * sp * dt, s.y + u.1 * sp * dt);
                if self.standable(tx, ty) {
                    v = (u.0 * sp, u.1 * sp);
                    (tx, ty)
                } else {
                    v = (0.0, 0.0);
                    (s.x, s.y)
                }
            } else {
                v = (0.0, 0.0);
                (s.x, s.y)
            };
            let sk = &mut self.soldiers[k];
            (sk.x, sk.y, sk.vx, sk.vy) = (x, y, v.0, v.1);
        }
    }

    /// Soldiers' positions drawn `a` (0-1) of the way from where this
    /// step's walk began to where it ended (`Soldier::was`), put in place
    /// of their own for the window's drawing; returns their own, for
    /// `put_back`. Only the living: the dead lie where they fell.
    pub fn drawn_between(&mut self, a: f32) -> Vec<(f32, f32)> {
        let a = a.clamp(0.0, 1.0);
        let own: Vec<(f32, f32)> = self.soldiers.iter().map(|s| (s.x, s.y)).collect();
        for s in self.soldiers.iter_mut().filter(|s| s.alive) {
            s.x = s.was.0 + (s.x - s.was.0) * a;
            s.y = s.was.1 + (s.y - s.was.1) * a;
        }
        own
    }

    /// Squad `q`'s middle as drawn `a` of the way through the step
    /// (`drawn_between`), for a camera that follows it smoothly.
    pub fn drawn_middle(&self, q: usize, a: f32) -> Option<(f32, f32)> {
        let a = a.clamp(0.0, 1.0);
        let at = |s: &Soldier| (s.was.0 + (s.x - s.was.0) * a, s.was.1 + (s.y - s.was.1) * a);
        let m: Vec<(f32, f32)> = self.soldiers.iter().filter(|s| s.alive && s.squad as usize == q).map(at).collect();
        if m.is_empty() {
            return None;
        }
        let mean = |ps: &[(f32, f32)]| {
            let (x, y) = ps.iter().fold((0.0, 0.0), |s, p| (s.0 + p.0, s.1 + p.1));
            (x / ps.len() as f32, y / ps.len() as f32)
        };
        let all = mean(&m);
        let stray = super::squads::STRAY * self.s();
        let near: Vec<(f32, f32)> = m.iter().copied().filter(|p| (p.0 - all.0).hypot(p.1 - all.1) <= stray).collect();
        Some(if near.is_empty() { all } else { mean(&near) })
    }

    /// Puts back what `drawn_between` moved.
    pub fn put_back(&mut self, own: &[(f32, f32)]) {
        for (s, &(x, y)) in self.soldiers.iter_mut().zip(own) {
            s.x = x;
            s.y = y;
        }
    }
}
