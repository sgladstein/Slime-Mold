//! Shots and bullets in flight. Moved out of `larder_world.rs` unchanged
//! (C9, 2026-11-14).

use super::*;

impl<W: World> Game<W> {
    /// Whether a step has begun and not finished.
    pub fn in_flight(&self) -> bool {
        self.flight.is_some()
    }

    /// The world's ticks as of the last finished step: while a step runs,
    /// the tick it began on. A reader keyed on game seconds (the
    /// campaign's tracker in the window) keys on this, so it reads whole
    /// steps, as the census does.
    pub fn settled_ticks(&self) -> u64 {
        self.flight.as_ref().map_or_else(|| self.world.ticks(), |f| f.t)
    }

    /// This frame's shots as the engines trace them. With
    /// `Rules::bullet_speed` 0 they are the fired rays as they were. Else
    /// the fired shots join `bullets` (past `squad::SHOTS_MAX` in flight the
    /// newest are dropped) and each bullet's leg is one frame's flight,
    /// ended early at the first wall or rock (`World::open`, checked a fine
    /// cell at a time, so a bullet never jumps a thin wall). Returns the
    /// legs and, for each, who fired it, whether it was fired now and the
    /// cast it carries (`by` gives who and the cast of each fired shot).
    pub(super) fn fly(&mut self, fired: Vec<ShotIn>, by: &[(Option<usize>, u8)], dt: f32) -> (Vec<ShotIn>, Vec<LegBy>) {
        let r = self.rules;
        if r.bullet_speed <= 0.0 {
            return (fired, by.iter().map(|&(b, c)| (b, true, c)).collect());
        }
        for (s, &(b, cast)) in fired.iter().zip(by) {
            if self.bullets.len() >= squad::SHOTS_MAX {
                self.bullets_dropped += 1;
                continue;
            }
            let (dx, dy) = (s.to.0 - s.from.0, s.to.1 - s.from.1);
            let d = dx.hypot(dy).max(1e-3);
            let m = b.map_or(BulletMods::default(), |k| self.mods(self.soldiers[k].squad as usize));
            let ammo = if cast > 0 { m.ammo.get(cast as usize - 1).copied() } else { None };
            let speed = ammo.map_or(1.0, |a| a.speed());
            // A needle flies on through its set's hits on top of the
            // Pierce cards' (`bullets::Shape::pierce`).
            let pierce = m.pierce.saturating_add(ammo.map_or(0, |a| a.set.shape().pierce));
            // The cast it carries (`Card::Trigger`), if its cast is a carrier.
            let (carry, expire) = if cast > 0 { (m.child[cast as usize - 1], m.expire[cast as usize - 1]) } else { (0, false) };
            self.bullets.push(Bullet { at: s.from, dir: (dx / d, dy / d), left: d, r: s.r, by: b, fresh: true, pierce, split: m.split, cast, speed, carry, expire, seek: m.seek, flown: 0.0 });
        }
        if self.bullets.iter().any(|b| b.seek > 0) {
            self.steer();
        }
        let leg = r.bullet_speed * self.s() * dt;
        let mut legs = Vec::with_capacity(self.bullets.len());
        let mut who = Vec::with_capacity(self.bullets.len());
        for b in &mut self.bullets {
            let len = (leg * b.speed).min(b.left);
            let mut to = (b.at.0 + b.dir.0 * len, b.at.1 + b.dir.1 * len);
            let n = len.ceil() as u32;
            for j in 1..=n {
                let t = (j as f32).min(len);
                let p = (b.at.0 + b.dir.0 * t, b.at.1 + b.dir.1 * t);
                if !self.world.clear(p.0, p.1) {
                    to = p;
                    // Nothing left past the wall: the leg ends on it and
                    // the bullet with it (`landed`).
                    b.left = t;
                    break;
                }
            }
            legs.push(ShotIn { from: b.at, to, r: b.r });
            who.push((b.by, b.fresh, b.cast));
        }
        (legs, who)
    }

    /// After a frame with bullets: a bullet that hit is spent, and the rest
    /// fly on from the end of their leg until their range runs out. A
    /// piercing bullet flies on from `pierce_gap` strike radii past its
    /// hit; a splitting one, once spent on slime, leaves `split_n` pieces
    /// fanned round its heading. Pieces join the end of the list in the
    /// order their parents stood, and past `squad::SHOTS_MAX` the rest are
    /// dropped (counted), so the cap and the order are the same on every
    /// world and every run.
    ///
    /// A carrier (`Bullet::carry`) lets its cast out at its first slime hit,
    /// or with `expire` where it stops; the released bullets join after the
    /// pieces (`Game::release`).
    pub(super) fn landed(&mut self, legs: &[ShotIn], out: &[squad::ShotOut]) {
        let r = self.rules;
        if r.bullet_speed <= 0.0 {
            return;
        }
        let sc = self.s();
        let mut pieces = Vec::new();
        // Where, which way, by whom and what each carrier lets out, and
        // whether it stopped at a wall.
        let mut out_casts: Vec<Release> = Vec::new();
        let mut k = 0;
        self.bullets.retain_mut(|b| {
            let q = k;
            k += 1;
            let (Some(l), Some(o)) = (legs.get(q), out.get(q)) else { return false };
            b.fresh = false;
            if let Some(h) = o.hit {
                // A trigger lets its cast out at the first hit; the bullet
                // flies on empty if it pierces.
                if b.carry > 0 && (!b.expire || b.pierce == 0) {
                    out_casts.push(Release { at: h, dir: b.dir, by: b.by, cast: b.carry, wall: false });
                    b.carry = 0;
                }
                // On a wall a leg ends with no hit; only slime gets here.
                if b.pierce > 0 {
                    b.pierce -= 1;
                    let gap = r.pierce_gap * b.r + 1.0;
                    let at = (h.0 + b.dir.0 * gap, h.1 + b.dir.1 * gap);
                    let went = (at.0 - l.from.0).hypot(at.1 - l.from.1);
                    b.left -= went;
                    b.flown += went;
                    b.at = at;
                    return b.left > 1e-3;
                }
                if b.split > 0 {
                    let n = r.split_n.max(1) as i32;
                    let a0 = b.dir.1.atan2(b.dir.0);
                    let rr = b.r * r.split_r;
                    for j in 0..n {
                        let a = a0 + if n > 1 { (j as f32 / (n - 1) as f32 * 2.0 - 1.0) * r.split_fan } else { 0.0 };
                        let dir = (a.cos(), a.sin());
                        let at = (h.0 + dir.0 * (rr + 1.0), h.1 + dir.1 * (rr + 1.0));
                        pieces.push(Bullet { at, dir, left: r.split_range * sc, r: rr, by: b.by, fresh: false, pierce: 0, split: b.split - 1, cast: 0, speed: b.speed, carry: 0, expire: false, seek: b.seek, flown: r.seek_after * sc });
                    }
                }
                return false;
            }
            let len = (l.to.0 - l.from.0).hypot(l.to.1 - l.from.1);
            b.at = l.to;
            b.left -= len;
            b.flown += len;
            let on = b.left > 1e-3;
            if !on && b.carry > 0 && b.expire {
                out_casts.push(Release { at: l.to, dir: b.dir, by: b.by, cast: b.carry, wall: !self.world.clear(l.to.0, l.to.1) });
            }
            on
        });
        for p in pieces {
            if self.bullets.len() >= squad::SHOTS_MAX {
                self.bullets_dropped += 1;
            } else {
                self.bullets.push(p);
            }
        }
        for o in out_casts {
            self.release(o);
        }
    }

    /// Lets a carried cast out (`Bullet::carry`) as its own bullets: as its
    /// soldier's gun would fire that cast (its set's pellets and fan, its
    /// strike, range and speed, the squad's flight cards), from `at` along
    /// the carrier's heading, with no aim spread. Off a wall it heads back
    /// out, turned as a ball off the wall's face (the side its last step
    /// back is open on). Past `squad::SHOTS_MAX` the rest are dropped.
    ///
    /// A carried cast with casts beside it (`Card::Multicast`) lets them
    /// all out, fanned as the gun fires them.
    pub(super) fn release(&mut self, o: Release) {
        let Some(k) = o.by else { return };
        let q = self.soldiers[k].squad as usize;
        let m = self.mods(q);
        let (mut dir, mut at) = (o.dir, o.at);
        if o.wall {
            // One cell back is where it flew from, open; which of the two
            // steps back into the wall is shut says which face it hit.
            let back = (at.0 - dir.0, at.1 - dir.1);
            let x = !self.world.clear(back.0 + dir.0, back.1);
            let y = !self.world.clear(back.0, back.1 + dir.1);
            dir = match (x, y) {
                (true, false) => (-dir.0, dir.1),
                (false, true) => (dir.0, -dir.1),
                _ => (-dir.0, -dir.1),
            };
            at = back;
        }
        self.releases += 1;
        let mut volley = vec![o.cast];
        while let Some(&w) = volley.last().map(|&c| &m.with[c as usize - 1]).filter(|&&w| w > 0 && !volley.contains(&w)) {
            volley.push(w);
        }
        self.multicast += volley.len() as u32 - 1;
        let nv = volley.len();
        for (v, &cast) in volley.iter().enumerate() {
            let i = cast as usize - 1;
            let ammo = m.ammo[i];
            let shape = ammo.set.shape();
            let r = (self.strike_of(q) * ammo.reach()).min(squad::R_MAX) * shape.r;
            let reach = self.gun_range(q) * self.s() * ammo.range();
            let side = if nv > 1 { (v as f32 / (nv - 1) as f32 * 2.0 - 1.0) * crate::mould::rounds::VOLLEY_FAN } else { 0.0 };
            let pellets = shape.pellets.max(1);
            for p in 0..pellets {
                if self.bullets.len() >= squad::SHOTS_MAX {
                    self.bullets_dropped += 1;
                    continue;
                }
                let a = side + if pellets > 1 { (p as f32 / (pellets - 1) as f32 * 2.0 - 1.0) * shape.fan } else { 0.0 };
                let (sn, cs) = a.sin_cos();
                let d = (dir.0 * cs - dir.1 * sn, dir.0 * sn + dir.1 * cs);
                let pierce = m.pierce.saturating_add(shape.pierce);
                self.bullets.push(Bullet { at, dir: d, left: reach, r, by: o.by, fresh: false, pierce, split: m.split, cast, speed: ammo.speed(), carry: m.child[i], expire: m.expire[i], seek: m.seek, flown: self.rules.seek_after * self.s() });
            }
        }
    }
}

impl<W: World> Game<W> {
    /// **Seeking bullets turn** (`rounds::Card::Seek`, step C of the
    /// gun-system review, 2026-11-16), before this frame's legs. Each
    /// seeker that has flown `Rules::seek_after` looks over the mould grid
    /// (`World::mould_grid` at a dish cell a square, the GPU's from the
    /// frame's readout) within `Rules::seek_r` and `Rules::seek_cone` of
    /// its heading for squares holding `Rules::seek_min` walkers, and takes
    /// the cheapest (distance, times one plus twice the turn) that is more
    /// than `Rules::seek_gap` from last frame's hits and from what the
    /// seekers before it in the list took this frame, and that it can see.
    /// It turns toward it by at most `Rules::seek_turn` a seek card. A
    /// mini-boss ahead in reach and in sight comes first, for every seeker.
    /// The bullets' order is the same on every world and run, so the picks
    /// are.
    ///
    /// Why the gap: soldiers already aim at the densest slime, so a homing
    /// bullet with no gap went where it was going anyway (the plan's
    /// argument); with it, the pellets of one shot and a split's pieces
    /// spread over different slime, and a bullet that clears a wall's end
    /// turns into the slime behind it.
    pub(super) fn steer(&mut self) {
        let r = self.rules;
        let sc = self.s();
        let bin = (sc as usize).max(1);
        let Some(grid) = self.world.mould_grid(bin) else { return };
        let (w, h) = self.world.size();
        let (gw, gh) = (w.div_ceil(bin), h.div_ceil(bin));
        if grid.len() < gw * gh {
            return;
        }
        let b = bin as f32;
        let reach = r.seek_r * sc;
        let gap = r.seek_gap * sc;
        let after = r.seek_after * sc;
        let cos_cone = r.seek_cone.cos();
        // Last frame's hits, then each pick as it is made.
        let mut taken: Vec<(f32, f32)> = self.last_shots.iter().filter_map(|s| s.hit).collect();
        let mut turned = 0u32;
        for i in 0..self.bullets.len() {
            let bl = self.bullets[i];
            if bl.seek == 0 || bl.flown + 1e-3 < after {
                continue;
            }
            // A mini-boss ahead in reach and in sight first (`Game::boss_aim`'s
            // rule for soldiers): every seeker may take it, so no gap.
            let boss = self
                .bosses
                .live
                .iter()
                .filter(|o| o.members > 0)
                .map(|o| {
                    let (dx, dy) = (o.centre.0 - bl.at.0, o.centre.1 - bl.at.1);
                    let d = dx.hypot(dy).max(1e-3);
                    (o.centre, d, (dx * bl.dir.0 + dy * bl.dir.1) / d, (o.members as f32 / std::f32::consts::PI).sqrt())
                })
                .filter(|&(c, d, cos, r)| d - r <= reach && cos >= cos_cone && self.clear_line(bl.at, c))
                .min_by(|a, b| a.1.total_cmp(&b.1))
                .map(|(c, ..)| c);
            if let Some(p) = boss {
                self.turn_to(i, p);
                turned += 1;
                continue;
            }
            let (cx, cy) = ((bl.at.0 / b) as i64, (bl.at.1 / b) as i64);
            let n = (reach / b).ceil() as i64 + 1;
            let mut best: Vec<(f32, (f32, f32))> = Vec::new();
            for gy in (cy - n).max(0)..=(cy + n).min(gh as i64 - 1) {
                for gx in (cx - n).max(0)..=(cx + n).min(gw as i64 - 1) {
                    if grid[gy as usize * gw + gx as usize] < r.seek_min {
                        continue;
                    }
                    let p = ((gx as f32 + 0.5) * b, (gy as f32 + 0.5) * b);
                    let (dx, dy) = (p.0 - bl.at.0, p.1 - bl.at.1);
                    let d = dx.hypot(dy);
                    if d > reach || d < 0.5 {
                        continue;
                    }
                    let cos = (dx * bl.dir.0 + dy * bl.dir.1) / d;
                    if cos < cos_cone {
                        continue;
                    }
                    if taken.iter().any(|t| (t.0 - p.0).hypot(t.1 - p.1) <= gap) {
                        continue;
                    }
                    best.push((d * (1.0 + 2.0 * cos.clamp(-1.0, 1.0).acos()), p));
                }
            }
            best.sort_by(|a, b| a.0.total_cmp(&b.0));
            // In sight: the first few cheapest, so a wall ahead costs a few
            // line tests, not one a square.
            let Some(&(_, p)) = best.iter().take(4).find(|(_, p)| self.clear_line(bl.at, *p)) else { continue };
            taken.push(p);
            self.turn_to(i, p);
            turned += 1;
        }
        self.seek_turns += turned;
    }

    /// Bullet `i` turns toward `p`, at most `Rules::seek_turn` a seek card.
    fn turn_to(&mut self, i: usize, p: (f32, f32)) {
        let bl = self.bullets[i];
        let want = (p.1 - bl.at.1).atan2(p.0 - bl.at.0);
        let now = bl.dir.1.atan2(bl.dir.0);
        let mut da = want - now;
        while da > std::f32::consts::PI {
            da -= std::f32::consts::TAU;
        }
        while da < -std::f32::consts::PI {
            da += std::f32::consts::TAU;
        }
        let max = self.rules.seek_turn * bl.seek as f32;
        let a = now + da.clamp(-max, max);
        self.bullets[i].dir = (a.cos(), a.sin());
    }
}

/// A carried cast to let out (`Game::release`).
pub(super) struct Release {
    at: (f32, f32),
    dir: (f32, f32),
    by: Option<usize>,
    cast: u8,
    /// It stopped at a wall, not at the end of its range.
    wall: bool,
}
