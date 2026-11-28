//! **Waves: the slime reads its losses** (2026-11-20, the thread "Make the
//! slime a more interesting enemy", its second idea after fruiting). The
//! owner, 11-20 05:46Z: "once I get strong enough there is just lots of
//! shooting through masses of uniform orange. Think outside the box".
//!
//! A strong squad wins by standing still and shooting whatever walks into
//! its range: the slime feeds itself into the guns a few walkers at a
//! time. Now, when it is losing walkers fast (the squad's kills over the
//! last `window` seconds at least `loss` of its mass, and its mass down
//! over that window), it stops: the slime near the squad pulls back out of
//! the guns' reach to two or three points round the squad, the slime
//! behind joins it there, and after `gather` seconds all of it comes in at
//! once from every point, for `strike` seconds. Then it is calm again,
//! until it loses that fast again (`every` seconds at the least).
//!
//! The rule is a field, as the lure's: the walking distance to the wave's
//! points (the gathering points, then the soldiers), which a calm walker
//! leans down (engine.wgsl's `wave_dist`, `World::set_wave_field`). The
//! picture is the simulation: the masses are the slime's own walkers.
//!
//! What it does to the game: standing still and shooting no longer wins
//! alone; a pause, then slime from two or three sides at once. Noita's
//! rule (the owner, 11-18 22:33Z): the counter is the squad's own, to
//! break up a mass while it gathers out there, or to face outward.
//!
//! GPU only (the owner's 11-17 19:21Z card): a world that cannot
//! (`World::can_wave`, the CPU engine) never sends one.
//!
//! **Hunted waves** (2026-11-28, the Enemy lane; Direction's job after the
//! owner, 11-28 03:31Z: "I didn't build turrets because the game was too
//! easy to beat without them ... trying to make the slime enemy
//! stronger"). The hunt (`hunt.rs`) brought the slime along the squad's
//! trail, but it came in a single file down the trail, into the guns at
//! the back, and died a few walkers at a time. Now, once the squad has
//! walked (`moved` dish cells over the trail's warm seconds) and at least
//! `near` of the slime's mass has caught up to the ring past the guns'
//! reach, the trail goes cold and a wave starts: the slime that caught up
//! masses where it is, and the points it lacks are filled `flank` degrees
//! either side of the biggest mass, so the wave comes in wide even from
//! one trail. The massed point's field starts `lead` of the way to its
//! nearest flank, so the mass splits: the walkers nearer a flank go round
//! to it. `hunted_gather` seconds (shorter than the losing wave's, as the
//! squad is walking), then the strike as any wave's. No losses needed,
//! the `every` gap and the one-at-a-time rule with fruits as any wave.

use super::raids::Way;
use super::{Game, World};

/// The waves' numbers. Times in game seconds, distances in dish cells.
/// Off by default (the campaign and the sandbox are frozen); a run turns
/// it on from its planet (`run::ENEMY_RULES`, `run::FIRST_PLANET`).
#[derive(Clone, Copy, Debug, PartialEq)]
#[derive(serde::Serialize, serde::Deserialize)]
pub struct WaveRules {
    pub on: bool,
    /// None until this long after the landing.
    pub first: f32,
    /// Losing: the squad's kills over the last `window` seconds at least
    /// `loss` of the slime's mass now, and the mass down over the window.
    pub window: f32,
    pub loss: f32,
    /// Seconds from one wave's end to the next one's start.
    pub every: f32,
    /// At most this many points, at least `apart` degrees apart round the
    /// squad, `stand` dish cells past the guns' reach, each with at least
    /// `least` of the slime's mass within `disc` dish cells of it. Fewer
    /// than two such points and there is no wave.
    pub sides: u32,
    pub apart: f32,
    pub stand: f32,
    pub disc: f32,
    pub least: f32,
    /// Seconds it gathers, and the pull to the points; the field reaches
    /// `reach` dish cells of walking from them.
    pub gather: f32,
    pub gather_pull: f32,
    pub reach: f32,
    /// Seconds it strikes, and the pull to the soldiers; the field reaches
    /// the guns' reach, `stand` and `reach` dish cells from them.
    pub strike: f32,
    pub strike_pull: f32,
    /// **Hunted waves** (the module's note): on by planet
    /// (`run::FIRST_PLANET`), and with the hunt only. The squad's walk over
    /// the trail's warm seconds, the share of the slime's mass caught up
    /// on the ring, the angle a flank point is filled at, the share of the
    /// way to it a massed point's field starts at, and its gather seconds.
    #[serde(default)]
    pub hunted: bool,
    #[serde(default = "hunted_moved")]
    pub moved: f32,
    #[serde(default = "hunted_near")]
    pub near: f32,
    #[serde(default = "hunted_flank")]
    pub flank: f32,
    #[serde(default = "hunted_lead")]
    pub lead: f32,
    #[serde(default = "hunted_gather")]
    pub hunted_gather: f32,
}

fn hunted_moved() -> f32 {
    25.0
}
fn hunted_near() -> f32 {
    0.06
}
fn hunted_flank() -> f32 {
    70.0
}
fn hunted_lead() -> f32 {
    0.5
}
fn hunted_gather() -> f32 {
    8.0
}

impl Default for WaveRules {
    fn default() -> Self {
        WaveRules {
            on: false,
            first: 90.0,
            window: 20.0,
            loss: 0.12,
            every: 45.0,
            sides: 3,
            apart: 70.0,
            stand: 20.0,
            disc: 15.0,
            least: 0.03,
            gather: 12.0,
            gather_pull: 40.0,
            reach: 60.0,
            strike: 8.0,
            strike_pull: 40.0,
            hunted: false,
            moved: hunted_moved(),
            near: hunted_near(),
            flank: hunted_flank(),
            lead: hunted_lead(),
            hunted_gather: hunted_gather(),
        }
    }
}

/// A wave's squad middle and its points (fine cells).
type WaveSite = ((f32, f32), Vec<(f32, f32)>);

/// What a wave is doing.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[derive(serde::Serialize, serde::Deserialize)]
pub enum WavePhase {
    /// Pulling back to its points and massing there.
    Gather,
    /// Coming in from every point at once.
    Strike,
}

/// One wave.
#[derive(Clone, Debug)]
#[derive(serde::Serialize, serde::Deserialize)]
pub struct Wave {
    pub phase: WavePhase,
    /// The tick its phase began.
    pub since: u64,
    /// The squad's middle and the points it gathers at (fine cells).
    pub centre: (f32, f32),
    pub points: Vec<(f32, f32)>,
    /// Started by the hunt catching up (the module's note), not by losses.
    #[serde(default)]
    pub hunted: bool,
}

/// The game's waves.
#[derive(Debug, Default)]
#[derive(serde::Serialize, serde::Deserialize)]
pub struct Waves {
    pub live: Option<Wave>,
    /// Waves sent, and the tick the last one ended.
    pub sent: u32,
    pub last: Option<u64>,
    /// Of those sent, the hunt's (the module's note).
    #[serde(default)]
    pub hunted: u32,
    /// The field being built and the pull it goes with (set a game second
    /// after it was started, as the lure's), and whether the world holds
    /// one to clear.
    way: Option<(Way, f32)>,
    set: bool,
    /// The squad's kills and the slime's mass at each of the last game
    /// seconds, oldest first.
    kills: std::collections::VecDeque<(u32, usize)>,
}

impl<W: World> Game<W> {
    /// Once a game second: set the field started a second ago, move a
    /// wave on, or start one when the slime is losing fast.
    pub(super) fn time_waves(&mut self) {
        let r = self.rules.wave;
        self.waves.kills.push_back((self.killed, self.mould));
        let keep = r.window.max(1.0) as usize + 1;
        while self.waves.kills.len() > keep {
            self.waves.kills.pop_front();
        }
        if !(r.on || r.hunted) || !self.world.can_wave() {
            return;
        }
        if let Some((way, pull)) = self.waves.way.take() {
            let field = way.finish();
            self.world.set_wave_field(field, pull);
            self.waves.set = true;
        }
        let now = self.world.ticks();
        let secs = |s: f32| (s.max(0.0) * self.rules.tps as f32) as u64;
        if let Some(mut w) = self.waves.live.take() {
            let alive = self.soldiers.iter().any(|s| s.alive);
            let gather = if w.hunted { r.hunted_gather } else { r.gather };
            match w.phase {
                WavePhase::Gather if alive && now >= w.since + secs(gather) => {
                    w.phase = WavePhase::Strike;
                    w.since = now;
                    self.start_strike_way();
                    self.waves.live = Some(w);
                    // All at once: a surge from every mass (the panic reach
                    // stretched to them while it strikes, `keep_panic_reach`).
                    self.keep_panic_reach();
                    self.wave_surge();
                }
                WavePhase::Strike if alive && now < w.since + secs(r.strike) => {
                    // The soldiers move: the way to them is rebuilt every
                    // second.
                    self.start_strike_way();
                    self.waves.live = Some(w);
                }
                WavePhase::Gather if alive => self.waves.live = Some(w),
                _ => {
                    // Over (or no one is left): calm again.
                    self.waves.way = None;
                    self.world.set_wave_field(Vec::new(), 0.0);
                    self.waves.set = false;
                    self.waves.last = Some(now);
                    self.keep_panic_reach();
                }
            }
            return;
        }
        if now < self.landed + secs(r.first) {
            return;
        }
        if self.waves.last.is_some_and(|t| now < t + secs(r.every)) {
            return;
        }
        // One nameable thing at a time (the review lane, 11-20: fruiting
        // and waves fire on nearly the same signal, so nearly every fruit
        // came with a wave): none while a fruit ripens.
        if self.fruits.live.is_some() {
            return;
        }
        if r.hunted && self.hunt_caught_up() && self.start_hunted_wave() {
            return;
        }
        if !r.on {
            return;
        }
        let (Some(&(k0, m0)), Some(&(k1, m1))) = (self.waves.kills.front(), self.waves.kills.back()) else { return };
        if self.waves.kills.len() < keep || m1 >= m0 || (k1.saturating_sub(k0) as f32) < r.loss * m1 as f32 {
            return;
        }
        self.start_wave();
    }

    /// Whether the hunt has caught up (the module's note): the slime
    /// follows a trail, the squad walked `moved` dish cells along it over
    /// its warm seconds, and `near` of the slime's mass is on the ring
    /// between the guns' reach and `stand` and `disc` past it, round the
    /// thickest knot of soldiers.
    fn hunt_caught_up(&self) -> bool {
        let r = self.rules.wave;
        if !self.world.hunt_pull() || self.mould == 0 {
            return false;
        }
        let sc = self.world.scale();
        let Some(walked) = self.hunt.walked() else { return false };
        if walked < r.moved * sc {
            return false;
        }
        let Some(k) = self.thickest_knot() else { return false };
        let (kx, ky) = (self.soldiers[k].x, self.soldiers[k].y);
        let bin = (sc as usize).max(1);
        let Some(g) = self.world.mould_grid(bin) else { return false };
        let gw = self.world.size().0.div_ceil(bin);
        let b = bin as f32;
        let range = self.gun_range_max() * sc;
        let out = range + (r.stand + r.disc) * sc;
        let held: u64 = g.iter().enumerate().filter(|&(i, &n)| {
            let d = (((i % gw) as f32 + 0.5) * b - kx).hypot(((i / gw) as f32 + 0.5) * b - ky);
            n > 0 && d >= range && d <= out
        }).map(|(_, &n)| n as u64).sum();
        held as f32 >= r.near * self.mould as f32
    }

    /// A hunted wave gathering now (the module's note): the massed points,
    /// and flank points filled `flank` degrees either side of the biggest;
    /// false with none massed.
    fn start_hunted_wave(&mut self) -> bool {
        let r = self.rules.wave;
        let now = self.world.ticks();
        let Some((centre, mut points)) = self.wave_points_from(1) else { return false };
        let massed = points.len();
        let sc = self.world.scale();
        let (w, h) = self.world.size();
        let range = self.gun_range_max() * sc;
        let soldiers: Vec<(f32, f32)> = self.soldiers.iter().filter(|s| s.alive).map(|s| (s.x, s.y)).collect();
        let best = points[0];
        let (dx, dy) = (best.0 - centre.0, best.1 - centre.1);
        let (ring, a0) = (dx.hypot(dy), dy.atan2(dx));
        let flank = r.flank.to_radians();
        let apart = r.apart.to_radians();
        for a in [a0 + flank, a0 - flank] {
            if points.len() >= r.sides.max(2) as usize {
                break;
            }
            let p = (centre.0 + ring * a.cos(), centre.1 + ring * a.sin());
            let inside = p.0 >= 0.0 && p.1 >= 0.0 && p.0 < w as f32 && p.1 < h as f32;
            let clear = soldiers.iter().all(|q| (q.0 - p.0).hypot(q.1 - p.1) >= range);
            let gap = points.iter().all(|q| {
                let d = ((q.1 - centre.1).atan2(q.0 - centre.0) - a).rem_euclid(std::f32::consts::TAU);
                d.min(std::f32::consts::TAU - d) >= apart * 0.9
            });
            if inside && clear && gap {
                points.push(p);
            }
        }
        if points.len() < 2 {
            return false;
        }
        // A massed point's field starts `lead` of the way to its nearest
        // flank point, so the walkers nearer a flank go round to it.
        let cell = |p: (f32, f32)| (p.0.clamp(0.0, w as f32 - 1.0) as u32) | ((p.1.clamp(0.0, h as f32 - 1.0) as u32) << 16);
        let flanks = &points[massed..];
        let from: Vec<(u32, u32)> = points.iter().enumerate().map(|(i, &p)| {
            let lead = if i < massed {
                flanks.iter().map(|q| (q.0 - p.0).hypot(q.1 - p.1)).fold(f32::INFINITY, f32::min)
            } else {
                0.0
            };
            (cell(p), if lead.is_finite() { (lead * r.lead.clamp(0.0, 1.0)) as u32 } else { 0 })
        }).collect();
        // The reach counts from each point's start, so the massed points'
        // lead is added to it, or a lead past the reach would draw nothing.
        let limit = (r.reach.max(0.0) * sc) as u32 + from.iter().map(|f| f.1).max().unwrap_or(0);
        let way = match self.slime_ground() {
            Some((agar, gw, gh)) => Way::start_with(move || super::raids::round_walk_from(&agar, gw, gh, &from, limit)),
            None => Way::Ready(Vec::new()),
        };
        self.waves.way = Some((way, r.gather_pull));
        self.waves.sent += 1;
        self.waves.hunted += 1;
        self.waves.live = Some(Wave { phase: WavePhase::Gather, since: now, centre, points, hunted: true });
        true
    }

    /// A wave gathering now round the squad; false with no points to
    /// gather at. Called by `time_waves` on the slime's losses, and by the
    /// slime viewer whenever it is asked (`Game::force_wave`).
    pub(super) fn start_wave(&mut self) -> bool {
        let r = self.rules.wave;
        let now = self.world.ticks();
        let Some((centre, points)) = self.wave_points() else { return false };
        let sc = self.world.scale();
        let (w, h) = self.world.size();
        let cell = |p: (f32, f32)| (p.0.clamp(0.0, w as f32 - 1.0) as u32) | ((p.1.clamp(0.0, h as f32 - 1.0) as u32) << 16);
        let cells = points.iter().map(|&p| cell(p)).collect();
        self.waves.way = Some((Way::start_lures(self.slime_ground(), cells, (r.reach.max(0.0) * sc) as u32), r.gather_pull));
        self.waves.sent += 1;
        self.waves.live = Some(Wave { phase: WavePhase::Gather, since: now, centre, points, hunted: false });
        true
    }

    /// A surge on the next step, counted as the meter's are
    /// (`time_surges`), on a world that surges.
    fn wave_surge(&mut self) {
        if self.world.surge_every() == 0 {
            return;
        }
        let step = self.world.ticks() * self.world.fine_steps().max(1) as u64;
        self.world.set_surge_at((step + 1).min(super::SURGE_WAIT as u64 - 1) as u32);
        self.surge_last = step;
        self.surge_meter = 0.0;
        self.surges += 1;
    }

    /// The way to every soldier standing, out to the guns' reach, `stand`
    /// and `reach` dish cells.
    fn start_strike_way(&mut self) {
        let r = self.rules.wave;
        let sc = self.world.scale();
        let (w, h) = self.world.size();
        let cell = |x: f32, y: f32| (x.clamp(0.0, w as f32 - 1.0) as u32) | ((y.clamp(0.0, h as f32 - 1.0) as u32) << 16);
        let cells = self.soldiers.iter().filter(|s| s.alive).map(|s| cell(s.x, s.y)).collect();
        let reach = ((self.gun_range_max() + r.stand + r.reach).max(0.0) * sc) as u32;
        self.waves.way = Some((Way::start_lures(self.slime_ground(), cells, reach), r.strike_pull));
    }

    /// The walkers within `disc` dish cells of each of the live wave's
    /// points, and within the guns' reach and `stand` of its squad's
    /// middle, from the mould grid (for the log and tests).
    pub fn wave_counts(&self) -> Option<(Vec<u32>, u32)> {
        let w = self.waves.live.as_ref()?;
        let sc = self.world.scale();
        let bin = (sc as usize).max(1);
        let g = self.world.mould_grid(bin)?;
        let gw = self.world.size().0.div_ceil(bin);
        let b = bin as f32;
        let within = |p: (f32, f32), r: f32| -> u32 {
            g.iter().enumerate().filter(|&(i, &n)| n > 0 && (((i % gw) as f32 + 0.5) * b - p.0).hypot(((i / gw) as f32 + 0.5) * b - p.1) <= r).map(|(_, &n)| n).sum()
        };
        let at = w.points.iter().map(|&p| within(p, self.rules.wave.disc * sc)).collect();
        let front = within(w.centre, (self.gun_range_max() + self.rules.wave.stand) * sc);
        Some((at, front))
    }

    /// Where a wave gathers: round the thickest knot of soldiers (its
    /// soldiers within 30 dish cells), on the ring `stand` dish cells past
    /// the guns' reach from its rim, the points round it with the most
    /// slime within `disc`, at least `apart` degrees apart, none within the
    /// guns' reach of any soldier. The squad's middle and the points (fine
    /// cells); None with fewer than two.
    pub(super) fn wave_points(&self) -> Option<WaveSite> {
        self.wave_points_from(2)
    }

    /// `wave_points` with at least `fewest` points (a hunted wave fills
    /// its flanks from one).
    fn wave_points_from(&self, fewest: usize) -> Option<WaveSite> {
        let r = self.rules.wave;
        let sc = self.world.scale();
        let bin = (sc as usize).max(1);
        let g = self.world.mould_grid(bin)?;
        let (w, h) = self.world.size();
        let (gw, gh) = (w.div_ceil(bin), h.div_ceil(bin));
        if g.len() < gw * gh {
            return None;
        }
        let k = self.thickest_knot()?;
        let (kx, ky) = (self.soldiers[k].x, self.soldiers[k].y);
        let knot: Vec<(f32, f32)> = self.soldiers.iter().filter(|s| s.alive && (s.x - kx).hypot(s.y - ky) <= 30.0 * sc).map(|s| (s.x, s.y)).collect();
        let n = knot.len().max(1) as f32;
        let c = (knot.iter().map(|p| p.0).sum::<f32>() / n, knot.iter().map(|p| p.1).sum::<f32>() / n);
        let rim = knot.iter().map(|p| (p.0 - c.0).hypot(p.1 - c.1)).fold(0.0f32, f32::max);
        let range = self.gun_range_max() * sc;
        let ring = rim + range + r.stand * sc;
        let soldiers: Vec<(f32, f32)> = self.soldiers.iter().filter(|s| s.alive).map(|s| (s.x, s.y)).collect();
        let b = bin as f32;
        let disc = r.disc * sc;
        let db = (disc / b).ceil() as i64;
        let mass = self.mould.max(1) as f32;
        const ANGLES: usize = 36;
        let mut scored: Vec<(f32, f32, (f32, f32))> = Vec::new();
        for i in 0..ANGLES {
            let a = std::f32::consts::TAU * i as f32 / ANGLES as f32;
            let p = (c.0 + ring * a.cos(), c.1 + ring * a.sin());
            if p.0 < 0.0 || p.1 < 0.0 || p.0 >= w as f32 || p.1 >= h as f32 {
                continue;
            }
            if soldiers.iter().any(|q| (q.0 - p.0).hypot(q.1 - p.1) < range) {
                continue;
            }
            let (px, py) = ((p.0 / b) as i64, (p.1 / b) as i64);
            let mut held = 0u64;
            for y in (py - db).max(0)..(py + db + 1).min(gh as i64) {
                for x in (px - db).max(0)..(px + db + 1).min(gw as i64) {
                    let (mx, my) = ((x as f32 + 0.5) * b, (y as f32 + 0.5) * b);
                    if (mx - p.0).hypot(my - p.1) <= disc {
                        held += g[y as usize * gw + x as usize] as u64;
                    }
                }
            }
            if held as f32 >= r.least * mass {
                scored.push((held as f32, a, p));
            }
        }
        scored.sort_by(|x, y| y.0.total_cmp(&x.0));
        let apart = r.apart.to_radians();
        let mut picked: Vec<(f32, (f32, f32))> = Vec::new();
        for (_, a, p) in scored {
            if picked.len() >= r.sides as usize {
                break;
            }
            let gap = |b: f32| {
                let d = (a - b).rem_euclid(std::f32::consts::TAU);
                d.min(std::f32::consts::TAU - d)
            };
            if picked.iter().all(|&(b, _)| gap(b) >= apart) {
                picked.push((a, p));
            }
        }
        (picked.len() >= fewest.max(1)).then(|| (c, picked.into_iter().map(|(_, p)| p).collect()))
    }
}
