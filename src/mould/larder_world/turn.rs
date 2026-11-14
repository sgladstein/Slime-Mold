//! One step of the game: the frame sent to the world, what comes back,
//! feeding, the core and the end. Moved out of `larder_world.rs` unchanged
//! (C9, 2026-11-14).

use super::*;

impl<W: World> Game<W> {
    /// One squad frame (`Rules::frame_ticks` ticks) with the squad's turn
    /// around it. Does nothing once the round has ended. It is
    /// `begin_step` then `finish_step` (2026-11-12): the window calls the
    /// two apart, so the GPU runs the frame while the window draws.
    pub fn step(&mut self) {
        self.begin_step();
        self.finish_step();
    }

    /// Whether `finish_step` would return without waiting on the world.
    pub fn step_ready(&self) -> bool {
        self.flight.as_ref().is_none_or(|f| f.out.is_some() || self.world.frame_ready())
    }

    /// The first half of `step`: the squad's turn (walking, banking, aims
    /// and shots) and the frame handed to the world, which may run it
    /// without waiting (`World::frame_start`). A step still running
    /// finishes first. Until `finish_step` the soldiers, bank and counts
    /// are the last step's, bar what this half changed.
    pub fn begin_step(&mut self) {
        self.finish_step();
        if self.end.is_some() {
            return;
        }
        self.solid = self.solid_now();
        let r = self.rules;
        let sc = self.s();
        let t = self.world.ticks();
        let dt = r.frame_ticks as f32 / r.tps as f32;
        // Steered and sent squads move their anchors first (`squads`).
        self.squad_turn(dt);
        let mut units = Vec::new();
        let mut who = Vec::new();
        let mut from = Vec::new();
        let mut shots = Vec::new();
        let mut shooters: Vec<Option<usize>> = Vec::new();
        // Elbow room first (`Rules::elbow`), so this frame's walk, touch
        // and shots all see the spread squad.
        self.elbow_room(dt);
        for k in 0..self.soldiers.len() {
            if !self.soldiers[k].alive {
                continue;
            }
            self.walk(k, dt);
            // A move that has arrived starts the next queued order (with
            // none queued it stays, as before: the window shows where).
            // (Not while its squad is still being sent somewhere: its move
            // there is the squad's, renewed every step.)
            let sent = self.rules.squads && self.squads.get(self.soldiers[k].squad as usize).is_some_and(|q| q.dest.is_some());
            if self.soldiers[k].queued > 0 && !sent {
                let arrived = match self.soldiers[k].order {
                    Order::Hold => true,
                    Order::Move(..) | Order::Fight(..) => self.goal(k).is_none_or(|g| (g.0 - self.soldiers[k].x).hypot(g.1 - self.soldiers[k].y) <= 1.5 * sc),
                    _ => false,
                };
                if arrived {
                    self.next(k);
                }
            }
            self.build_turn(k, dt);
            let s = self.soldiers[k];
            // Harvest: at its crystal and not full, a clearing soldier takes;
            // full, it walks home; home, it banks.
            let mut take = 0.0;
            let mut at_crystal = None;
            if let Order::Clear(x, y) = s.order {
                if s.homing {
                    let home = self.drop_off((s.x, s.y));
                    if (s.x - home.0).hypot(s.y - home.1) <= self.reach(home, 3.0) + 1.5 * (k as f32).sqrt() * sc {
                        self.bank += s.carry;
                        self.harvested += s.carry;
                        self.soldiers[k].carry = 0.0;
                        self.soldiers[k].homing = false;
                        if self.soldiers[k].dry {
                            self.soldiers[k].dry = false;
                            let skip = self.crystal_at((x, y)).map(|c| c.0);
                            self.after_dry(k, skip);
                        }
                    }
                } else if s.carry > 0.0 && s.carry >= r.carry - s.load {
                    // Never home empty-handed: with tools filling its carry
                    // (`Soldier::load`) a soldier would walk to the larder,
                    // bank nothing and turn round, for ever.
                    self.soldiers[k].homing = true;
                } else if let Some((i, c)) = self.crystal_at((x, y)) {
                    // From its rim (`Rules::crystal_solid`), with the slot's room.
                    let room = if self.rules.crystal_solid { RIM + 1.0 } else { 1.0 };
                    if (s.x - c.0).hypot(s.y - c.1) <= self.world.crystal_r(i) + room * sc {
                        take = (r.harvest * dt).min((r.carry - s.load - s.carry).max(0.0)) * r.charge;
                        at_crystal = Some(i);
                    }
                }
            }
            let q = s.squad as usize;
            let strike = self.strike_of(q);
            // A volleying squad fires every member at its point at once and
            // nothing else; the others fire on their own clocks.
            let volley = self.volley(k, t);
            let firing = self.squads[q].fire.is_some();
            if (volley.is_some() || (!firing && s.ready <= t)) && self.bank >= r.price {
                let aim = if firing { volley } else { self.target(k) };
                if let Some(at) = aim {
                    let (dx, dy) = (at.0 - s.x, at.1 - s.y);
                    let d = dx.hypot(dy).max(0.1);
                    let reach = self.gun_range() * sc;
                    if volley.is_some() || d <= reach + 4.0 {
                        let (mut ux, mut uy) = (dx / d, dy / d);
                        if r.spread > 0.0 {
                            let a = ((self.rng.f32() * 2.0 - 1.0) * r.spread * sc / d).atan();
                            let (sn, cs) = a.sin_cos();
                            (ux, uy) = (ux * cs - uy * sn, ux * sn + uy * cs);
                        }
                        shots.push(ShotIn { from: (s.x, s.y), to: (s.x + ux * reach, s.y + uy * reach), r: strike });
                        shooters.push(Some(k));
                        self.soldiers[k].ready = t + self.shot_gap(q);
                        self.bank -= r.price;
                        self.spent += r.price;
                    }
                }
            }
            let pull = self.core_rule.pull;
            let pulls = pull > 0.0 && (s.x - self.core.0).hypot(s.y - self.core.1) <= 2.0 * r.core_r * sc;
            units.push(UnitIn {
                x: s.x,
                y: s.y,
                touch: r.touch * sc,
                sight: self.gun_range() * sc,
                take_r: at_crystal.map_or(4.0 * sc, |i| self.world.crystal_r(i).min(4.0 * sc)),
                take,
                smell_r: if pulls { r.build.lure_r.max(r.smell_r) * sc } else { r.smell_r * sc },
                smell: self.world.food_deposit() * r.smell * s.smell * if pulls { pull.max(1.0) } else { 1.0 } * r.frame_ticks as f32,
            });
            who.push(Who::Soldier(k));
            from.push(at_crystal);
        }
        self.volley_done(t);
        // The player's own mould's gardeners (`own_mould`; nothing here
        // unless the world has one).
        let tended = self.world.tend(&self.soldiers, self.bank, r.frame_ticks);
        self.bank -= tended;
        self.spent += tended;
        // The larder: smells of the bank, and reads who touches it (none
        // without one, `Rules::larder`).
        if r.larder {
        units.push(UnitIn {
            x: self.larder.0,
            y: self.larder.1,
            touch: 3.0 * sc,
            sight: 0.0,
            take_r: 0.0,
            take: 0.0,
            smell_r: 3.0 * sc,
            smell: self.world.food_deposit() * r.hoard * self.bank.max(0.0) / 10_000.0 * r.frame_ticks as f32,
        });
        who.push(Who::Larder);
        from.push(None);
        }
        // Turrets, lures and pumps: units that do not move (`buildings`).
        // A turret's hits lay the crater too, through the same frame (the
        // buildings thread's call, 11-12 07:46Z: one rule for every shot).
        self.building_units(&mut units, &mut shots, &mut who, &mut from, dt);
        shooters.resize(shots.len(), None);
        let fired = shots.len() as u32;
        let (shots, by) = self.fly(shots, &shooters, dt);
        let crater = (r.crater * (r.tps * self.world.fine_steps()) as f32).round().clamp(0.0, u16::MAX as f32) as u16;
        let f = Frame { units, shots, slime: squad::slime_trail(6.0), crater, sink: r.sink };
        let out = self.world.frame_start(&f, r.frame_ticks);
        self.flight = Some(Flight { t, f, by, fired, who, from, out });
    }

    /// The second half of `step`: the frame's results, waited for if the
    /// world has not given them (`World::frame_finish`), and what they do
    /// (kills, harvest, touches, theft, the core, the end). Nothing when no
    /// step is running.
    pub fn finish_step(&mut self) {
        let Some(Flight { t, f, by, fired, who, from, out }) = self.flight.take() else { return };
        let out = out.unwrap_or_else(|| self.world.frame_finish());
        let r = self.rules;
        let sc = self.s();
        let dt = r.frame_ticks as f32 / r.tps as f32;
        self.shots += fired;
        self.last_shots = f.shots.iter().zip(&out.shots).zip(&by).map(|((i, o), &(by, fired))| Shot { from: i.from, to: i.to, hit: o.hit, killed: o.killed, by, fired }).collect();
        self.landed(&f.shots, &out.shots);
        self.killed += out.shots.iter().map(|o| o.killed).sum::<u32>();
        // Gassed mould sleeps and does not bite (the squad toolkit's sleep
        // gas): its walkers are taken off each unit's touch.
        let calm = self.world.calm(&f.units.iter().map(|u| (u.x, u.y)).collect::<Vec<_>>(), r.touch * sc);
        // Whole walkers off whole walkers: what each is worth is decided below, per rule.
        let out = FrameOut { units: out.units.iter().zip(&calm).map(|(u, &c)| UnitOut { touching: u.touching.saturating_sub(c), ..*u }).collect(), ..out };
        let left: Vec<f32> = if r.home_dry { self.world.crystals().iter().map(|c| c.1).collect() } else { Vec::new() };
        let mut empty = Vec::new();
        self.last_takes.clear();
        for (((u, k), c), ask) in out.units.iter().zip(&who).zip(&from).zip(&f.units) {
            if let Some(c) = *c {
                self.taken[c] += u.took;
                if u.took > 0.0 {
                    let by = if let Who::Soldier(k) = *k { Some(k) } else { None };
                    self.last_takes.push(Take { by, crystal: c, at: (ask.x, ask.y), took: u.took });
                }
            }
            match *k {
                Who::Building(b) => self.building_out(b, u, dt),
                Who::Soldier(k) => {
                    let s = &mut self.soldiers[k];
                    s.touching = u.touching;
                    s.aim = u.target;
                    s.dense = u.dense;
                    s.carry += u.took / r.charge;
                    if !r.larder {
                        // Drained straight into the bank: nothing to carry.
                        self.bank += s.carry;
                        self.harvested += s.carry;
                        s.carry = 0.0;
                    }
                    // A crystal run dry sends its carrier home with what it
                    // has (`Rules::home_dry`, on): the frame gave under half
                    // its ask and the whole crystal holds less than one
                    // frame's ask (it never reaches 0 exactly: 0.8 charge
                    // can stay on its cells). Once it banks it holds at the
                    // larder (`Soldier::dry`) rather than walk back for
                    // crumbs. Without it (W2b, M3) a carrier went home only
                    // full: on the v1 Fork (WM1) a soldier emptied a crystal
                    // of 12,032 charge (188 food) and stood there for a 300
                    // load that never came. Four rules were tried under W2b's
                    // score and each cost the planned hands (the narrowest,
                    // this one: farm lower on 10 of 10), because that score
                    // counted a crystal with crumbs left as held while a
                    // soldier stood on it (`Reports/dead-ends.md`). Under
                    // score v2 (`Game::score`, charge taken) it is even: farm
                    // 5 higher and 8 lower of 16, wall 7 and 9. With the hold
                    // and a farm hand that skips dry crystals, the hand
                    // harvests twice the food (22,875 to 46,364 a game).
                    if let (true, Some(i)) = (r.home_dry, *c) {
                        if u.took < 0.5 * ask.take && left[i] < ask.take {
                            if s.carry > 0.0 {
                                s.homing = true;
                                s.dry = true;
                            } else if r.seek || s.queued > 0 || !r.larder {
                                // Empty-handed at a dry crystal (sent to
                                // one, or it ran dry on the first take):
                                // on at once, nothing to bank.
                                empty.push((k, i));
                            }
                        }
                    }
                    if u.touching > 0 {
                        if self.first_touch.is_none() {
                            self.first_touch = Some(self.world.ticks() as f32 / r.tps as f32);
                        }
                        // Whole walkers: each bites `drain` at any mass, so walking into a thick body is very deadly (owner 11-12 22:22Z, #174).
                        s.hp -= r.drain * u.touching as f32 * dt;
                        if s.hp <= 0.0 {
                            s.alive = false;
                            s.hp = 0.0;
                            s.carry = 0.0;
                        }
                    }
                }
                Who::Larder => {
                    // By mass: a body over the larder eats what the 5,000-walker mould's did, so C's 25,000 don't empty the bank five times as fast.
                    let take = (r.steal * r.mass * u.touching as f32 * dt).min(self.bank.max(0.0));
                    self.bank -= take;
                    self.stolen += take;
                    self.larder_touch = u.touching;
                }
            }
        }
        for (k, i) in empty {
            self.after_dry(k, Some(i));
        }
        self.check_core();
        let now = self.world.ticks();
        if now / r.tps as u64 != t / r.tps as u64 {
            self.mould = self.world.living();
            self.guard_pay();
            self.count_stuck();
            self.check_broken();
            self.keep_panic_reach();
        }
        self.feed(dt);
        self.campaign_rules(now - t);
        self.check_end();
    }

    /// A scared piece's reach, past the gun's range (`Rules::panic_margin`):
    /// at the start and once a game second, so a range card moves it.
    pub(super) fn keep_panic_reach(&mut self) {
        if self.rules.panic_margin >= 0.0 {
            self.world.set_panic_r(self.rules.range + self.rules.panic_margin);
        }
    }

    /// Once a game second, at the count: the broken mould (`Rules::broken`).
    pub(super) fn check_broken(&mut self) {
        let r = self.rules;
        self.peak = self.peak.max(self.mould);
        if r.broken <= 0.0 {
            return;
        }
        // A ratio of whole walkers to the peak's, so the same at any mass.
        let under = (self.mould as f32) < r.broken * self.peak as f32;
        match (under, self.broken) {
            (true, None) => {
                let up = self.world.upkeep();
                self.world.set_upkeep(up * r.broken_starve);
                self.broken = Some((self.world.ticks(), up));
            }
            (false, Some((_, up))) => {
                self.world.set_upkeep(up);
                self.broken = None;
            }
            _ => {}
        }
    }

    /// `Rules::ration` and `Rules::hunger`: every live soldier eats from
    /// the bank; for what it can't pay, the squad loses hit points.
    pub(super) fn feed(&mut self, dt: f32) {
        let r = self.rules;
        let alive = self.alive();
        self.hungry = false;
        if r.ration <= 0.0 || alive == 0 {
            return;
        }
        let need = r.ration * alive as f32 * dt;
        let paid = need.min(self.bank.max(0.0));
        self.bank -= paid;
        self.eaten += paid;
        let short = 1.0 - paid / need;
        if short <= 1e-6 {
            return;
        }
        self.hungry = true;
        for s in self.soldiers.iter_mut().filter(|s| s.alive) {
            s.hp -= r.hunger * short * dt;
            if s.hp <= 0.0 {
                s.alive = false;
                s.hp = 0.0;
                s.carry = 0.0;
            }
        }
    }

    /// Once a game second, with no larder (`Rules::larder`): every crystal
    /// with charge left, a live soldier standing by it (not draining it)
    /// and no more than `Rules::guard_walkers` walkers on it pays a `Rules::guard`-th of the charge it has
    /// left: a crystal half drained guards for half, one drained dry (it
    /// keeps crumbs) for nothing.
    pub(super) fn guard_pay(&mut self) {
        let r = self.rules;
        self.guarded.clear();
        if r.larder || r.guard <= 0.0 {
            return;
        }
        let sc = self.s();
        let cs = self.world.crystals();
        for (i, &(c, charge)) in cs.iter().enumerate() {
            if charge <= 0.0 {
                continue;
            }
            let cr = self.world.crystal_r(i);
            let by = |s: &Soldier| s.alive && !matches!(s.order, Order::Clear(..)) && (s.x - c.0).hypot(s.y - c.1) <= cr + (RIM + 4.0) * sc;
            if !self.soldiers.iter().any(by) || self.world.walkers_in(c, cr + sc) > r.guard_walkers {
                continue;
            }
            let pay = charge / r.charge / r.guard;
            self.bank += pay;
            self.guard_paid += pay;
            self.guarded.push(i);
        }
    }

    /// Once a game second: `Game::stuck`.
    pub(super) fn count_stuck(&mut self) {
        let taken = self.harvested + self.soldiers.iter().map(|s| s.carry).sum::<f32>();
        if self.alive() > 0 && self.shots == self.stuck_mark.0 && taken <= self.stuck_mark.1 && self.bank < self.rules.soldier_cost {
            self.stuck += 1;
            self.stuck_run += 1;
        } else {
            self.stuck_run = 0;
        }
        self.stuck_mark = (self.shots, taken);
    }

    /// `Rules::overrun` and `Rules::reserve`, each off at 0.
    pub(super) fn campaign_rules(&mut self, ticks: u64) {
        let r = self.rules;
        // Presence: any walker on the larder holds it, at any mass.
        let held = self.larder_touch > 0;
        if r.overrun > 0.0 {
            self.held_ticks = if held && self.bank < 1.0 { self.held_ticks + ticks } else { 0 };
            if self.held_ticks >= self.secs(r.overrun) && self.end.is_none() {
                self.end = Some(End::Overrun(self.time()));
            }
        }
        if r.reserve > 0.0 {
            if self.alive() < 2 && self.bank < r.soldier_cost && !held {
                self.reserve_ticks += ticks;
                if self.reserve_ticks >= self.secs(r.reserve) {
                    self.reserve_ticks = 0;
                    let a = self.soldiers.len() as f32 * 2.399_963;
                    self.add_soldier(a);
                }
            } else {
                self.reserve_ticks = 0;
            }
        }
    }

    pub(super) fn check_core(&mut self) {
        if self.core_down.is_some() || self.core_rule.kind == CoreKind::Headless {
            return;
        }
        let sc = self.s();
        let r = self.rules.core_r * sc;
        let on = self.soldiers.iter().any(|s| s.alive && (s.x - self.core.0).hypot(s.y - self.core.1) <= r);
        // No walker near: the touch of the soldiers on it reads 0 (presence, so mass doesn't apply).
        let clear = on && self.soldiers.iter().filter(|s| s.alive && (s.x - self.core.0).hypot(s.y - self.core.1) <= r).all(|s| s.touching == 0 && s.aim.is_none_or(|a| (a.0 - self.core.0).hypot(a.1 - self.core.1) > 2.0 * r));
        if clear {
            self.core_clear += self.rules.frame_ticks as u64;
            if self.core_clear >= self.secs(self.core_rule.secs) {
                self.core_down = Some(self.world.ticks());
                self.world.stop_births();
            }
        } else {
            self.core_clear = 0;
        }
    }

    pub(super) fn check_end(&mut self) {
        if self.end.is_some() {
            return;
        }
        let t = self.time();
        // With the reserve on, a wiped squad waits for its recruit unless
        // the mould holds the larder.
        let recruit = self.rules.reserve > 0.0 && self.larder_touch == 0;
        if self.alive() == 0 && self.bank < self.rules.soldier_cost && !recruit {
            self.end = Some(End::SquadWiped(t));
        } else if self.mould == 0 {
            self.end = Some(End::MouldWiped(t));
        } else if t >= self.rules.cap {
            self.cap_held = Some(self.share());
            self.end = Some(End::Cap(self.take_share()));
        }
    }
}
