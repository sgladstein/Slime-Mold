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
            self.bullets.push(Bullet { at: s.from, dir: (dx / d, dy / d), left: d, r: s.r, by: b, fresh: true, pierce, split: m.split, cast, speed, carry, expire });
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
                    b.left -= (at.0 - l.from.0).hypot(at.1 - l.from.1);
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
                        pieces.push(Bullet { at, dir, left: r.split_range * sc, r: rr, by: b.by, fresh: false, pierce: 0, split: b.split - 1, cast: 0, speed: b.speed, carry: 0, expire: false });
                    }
                }
                return false;
            }
            let len = (l.to.0 - l.from.0).hypot(l.to.1 - l.from.1);
            b.at = l.to;
            b.left -= len;
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
                self.bullets.push(Bullet { at, dir: d, left: reach, r, by: o.by, fresh: false, pierce, split: m.split, cast, speed: ammo.speed(), carry: m.child[i], expire: m.expire[i] });
            }
        }
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
