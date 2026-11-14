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
    /// legs and, for each, who fired it and whether it was fired now.
    pub(super) fn fly(&mut self, fired: Vec<ShotIn>, by: &[Option<usize>], dt: f32) -> (Vec<ShotIn>, Vec<(Option<usize>, bool)>) {
        let r = self.rules;
        if r.bullet_speed <= 0.0 {
            return (fired, by.iter().map(|&b| (b, true)).collect());
        }
        for (s, &b) in fired.iter().zip(by) {
            if self.bullets.len() >= squad::SHOTS_MAX {
                self.bullets_dropped += 1;
                continue;
            }
            let (dx, dy) = (s.to.0 - s.from.0, s.to.1 - s.from.1);
            let d = dx.hypot(dy).max(1e-3);
            let m = if b.is_some() { self.bullet_mods } else { BulletMods::default() };
            self.bullets.push(Bullet { at: s.from, dir: (dx / d, dy / d), left: d, r: s.r, by: b, fresh: true, pierce: m.pierce, split: m.split });
        }
        let leg = r.bullet_speed * self.s() * dt;
        let mut legs = Vec::with_capacity(self.bullets.len());
        let mut who = Vec::with_capacity(self.bullets.len());
        for b in &mut self.bullets {
            let len = leg.min(b.left);
            let mut to = (b.at.0 + b.dir.0 * len, b.at.1 + b.dir.1 * len);
            let n = len.ceil() as u32;
            for j in 1..=n {
                let t = (j as f32).min(len);
                let p = (b.at.0 + b.dir.0 * t, b.at.1 + b.dir.1 * t);
                if !self.world.open(p.0, p.1) {
                    to = p;
                    // Nothing left past the wall: the leg ends on it and
                    // the bullet with it (`landed`).
                    b.left = t;
                    break;
                }
            }
            legs.push(ShotIn { from: b.at, to, r: b.r });
            who.push((b.by, b.fresh));
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
    pub(super) fn landed(&mut self, legs: &[ShotIn], out: &[squad::ShotOut]) {
        let r = self.rules;
        if r.bullet_speed <= 0.0 {
            return;
        }
        let sc = self.s();
        let mut pieces = Vec::new();
        let mut k = 0;
        self.bullets.retain_mut(|b| {
            let q = k;
            k += 1;
            let (Some(l), Some(o)) = (legs.get(q), out.get(q)) else { return false };
            b.fresh = false;
            if let Some(h) = o.hit {
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
                        pieces.push(Bullet { at, dir, left: r.split_range * sc, r: rr, by: b.by, fresh: false, pierce: 0, split: b.split - 1 });
                    }
                }
                return false;
            }
            let len = (l.to.0 - l.from.0).hypot(l.to.1 - l.from.1);
            b.at = l.to;
            b.left -= len;
            b.left > 1e-3
        });
        for p in pieces {
            if self.bullets.len() >= squad::SHOTS_MAX {
                self.bullets_dropped += 1;
            } else {
                self.bullets.push(p);
            }
        }
    }
}
