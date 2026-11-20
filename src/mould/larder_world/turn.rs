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

    /// **Asleep in your own gas** (2026-11-18, `Rules::doze`; the owner's
    /// Noita rule, "Any single spell that is incredibly powerful is also
    /// really dangerous"): a living soldier whose cell had a sleep cloud at
    /// `doze` times the slime's sleep level last frame does not fire this
    /// one; it still walks where it is sent (the owner, 11-19: a squad that
    /// could not walk through its own gas was "just annoying"). Gas cast
    /// into slime on top of the squad, or a cloud drifting back over it,
    /// leaves the squad unable to shoot while the slime sleeps beside it.
    pub fn dozing(&self, k: usize) -> bool {
        let r = self.rules.doze;
        r > 0.0 && self.soldiers.get(k).is_some_and(|s| s.alive && s.doze >= r)
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
        self.pump_maps();
        let r = self.rules;
        let sc = self.s();
        let t = self.world.ticks();
        let dt = r.frame_ticks as f32 / r.tps as f32;
        // Which squads on attack-move stand to fight (`fight_turn`), then
        // steered and sent squads move their anchors (`squads`).
        self.fight_turn();
        self.squad_turn(dt);
        let mut units = Vec::new();
        let mut who = Vec::new();
        let mut from = Vec::new();
        let mut shots = Vec::new();
        // Who fired each shot, and the cast it carries (`Bullet::cast`).
        let mut shooters: Vec<Firer> = Vec::new();
        // How far each soldier's shot was aimed (`Bullet::aim`).
        let mut aims: Vec<f32> = Vec::new();
        // The squad mode's walk (`Rules::steer`): every soldier at once,
        // velocity, elbow room and queuing together (`steer`).
        let steer = r.squads && r.steer;
        if steer {
            self.steer_all(dt);
        } else {
            for s in self.soldiers.iter_mut() {
                s.was = (s.x, s.y);
                (s.vx, s.vy) = (0.0, 0.0);
            }
            // Elbow room first (`Rules::elbow`), so this frame's walk, touch
            // and shots all see the spread squad.
            self.elbow_room(dt);
        }
        for k in 0..self.soldiers.len() {
            if !self.soldiers[k].alive {
                continue;
            }
            // Where its walk began (with `steer` everyone has walked already).
            let was = self.soldiers[k].was;
            // Groggy in its own gas (`Rules::doze`) it still walks: until
            // 11-19 a dozing soldier also stood still, and the owner, 17:10Z:
            // "It basically just creates a barrier that we cannot walk
            // through ... dangerous and stopping you from moving anywhere is
            // different and the latter is just annoying".
            let dozing = self.dozing(k);
            if !steer {
                self.walk(k, dt);
            }
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
                        take = (r.harvest * if r.larder { 1.0 } else { r.hand } * dt * self.work_boost(k)).min((r.carry - s.load - s.carry).max(0.0)) * r.charge;
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
            let mut shot = false;
            if !dozing && (volley.is_some() || (!firing && s.ready <= t)) && self.bank >= r.price {
                let aim = if firing { volley } else { self.target(k) };
                if let Some(at) = aim {
                    let (dx, dy) = (at.0 - s.x, at.1 - s.y);
                    let d = dx.hypot(dy).max(0.1);
                    let reach = self.gun_range(q) * sc;
                    // Your own dangers (`Rules::doze`,
                    // `RoundRules::blast_hurt`): fire on its own keeps them
                    // off the squad where the shot really stops
                    // (`RoundRules::blast_margin`, in `Rounds::fire`).
                    if volley.is_some() || d <= reach + 4.0 {
                        // The squad's gun (`Game::gun`): its spread, and its
                        // bullets a shot, each turned on its own (the
                        // scattergun's fan; the standard rifle fires one).
                        let gun = self.gun(q);
                        // Every shot fires the next card in turn, at the
                        // gun's rate, and never stops to recharge (owner
                        // 18:05Z); a scattergun's fan all carry it.
                        let m = self.mods(q);
                        let n = m.casts;
                        // The gun's own turn (`BulletMods::cycle`): a cast
                        // another carries is let out by its carrier.
                        let cast = if n > 0 {
                            let c = s.next % n;
                            self.soldiers[k].next = (c + 1) % n;
                            m.cycle[c as usize]
                        } else {
                            0
                        };
                        // The casts beside it (`Card::Multicast`,
                        // 2026-11-16): one shot fires them all, each its own
                        // bullets, fanned `rounds::VOLLEY_FAN` apart.
                        let mut volley = vec![cast];
                        while let Some(&w) = volley.last().filter(|&&c| c > 0).map(|&c| &m.with[c as usize - 1]).filter(|&&w| w > 0 && !volley.contains(&w)) {
                            volley.push(w);
                        }
                        self.multicast += volley.len() as u32 - 1;
                        let nv = volley.len();
                        for (v, &cast) in volley.iter().enumerate() {
                            // The bullet it fires (`bullets::Ammo`): its damage
                            // widens the strike (area as damage, up to the
                            // engines' `squad::R_MAX`), its range and spread
                            // times the gun's; its speed rides on the bullet.
                            let ammo = if cast > 0 { m.ammo[cast as usize - 1] } else { Default::default() };
                            // Its set's shape (`bullets::Shape`, 2026-11-16):
                            // buckshot fires a fan of small pellets.
                            let shape = ammo.set.shape();
                            let hit_r = (strike * ammo.reach()).min(squad::R_MAX) * shape.r;
                            let reach = reach * ammo.range();
                            let spread = gun.spread * ammo.spread();
                            let pellets = shape.pellets.max(1);
                            let side = if nv > 1 { (v as f32 / (nv - 1) as f32 * 2.0 - 1.0) * crate::mould::rounds::VOLLEY_FAN } else { 0.0 };
                            for _ in 0..gun.shots.max(1) {
                                let (mut ux, mut uy) = (dx / d, dy / d);
                                if spread > 0.0 {
                                    let a = ((self.rng.f32() * 2.0 - 1.0) * spread * sc / d).atan();
                                    let (sn, cs) = a.sin_cos();
                                    (ux, uy) = (ux * cs - uy * sn, ux * sn + uy * cs);
                                }
                                for p in 0..pellets {
                                    let a = side + if pellets > 1 { (p as f32 / (pellets - 1) as f32 * 2.0 - 1.0) * shape.fan } else { 0.0 };
                                    let (sn, cs) = a.sin_cos();
                                    let (px, py) = (ux * cs - uy * sn, ux * sn + uy * cs);
                                    shots.push(ShotIn { from: (s.x, s.y), to: (s.x + px * reach, s.y + py * reach), r: hit_r, over: false });
                                    shooters.push((Some(k), cast, None));
                                    aims.push(d);
                                }
                            }
                        }
                        // A worker of a squad shoots less often (`CoverRules::work_fire`).
                        let gap = self.shot_gap(q);
                        let gap = if r.squads && self.working(k) { ((gap as f32 / r.cover.work_fire.max(0.01)).round() as u64).max(1) } else { gap };
                        self.soldiers[k].ready = t + gap;
                        shot = true;
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
                sight: self.sight(k),
                take_r: at_crystal.map_or(4.0 * sc, |i| self.world.crystal_r(i).min(4.0 * sc)),
                take,
                smell_r: if pulls { r.build.lure_r.max(r.smell_r) * sc } else { r.smell_r * sc },
                smell: self.world.food_deposit() * r.smell * s.smell * if pulls { pull.max(1.0) } else { 1.0 } * r.frame_ticks as f32,
                over: false,
            });
            who.push(Who::Soldier(k));
            from.push(at_crystal);
            let s = self.soldiers[k];
            self.heal(k, (s.x - was.0).hypot(s.y - was.1), shot, dt);
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
            over: false,
        });
        who.push(Who::Larder);
        from.push(None);
        }
        // Turrets, lures and pumps: units that do not move (`buildings`).
        // A turret's hits lay the crater too, through the same frame (the
        // buildings thread's call, 11-12 07:46Z: one rule for every shot).
        shooters.resize(shots.len(), (None, 0, None));
        self.building_units(&mut units, &mut shots, &mut shooters, &mut who, &mut from);
        shooters.resize(shots.len(), (None, 0, None));
        // Starfall's ore and the cores (`starfall.rs`): they smell and
        // read the walkers on them.
        self.drop_units(&mut units, &mut who, &mut from);
        let fired = shots.len() as u32;
        let (shots, by) = self.fly(shots, &shooters, &aims, dt);
        let crater = (r.crater * (r.tps * self.world.fine_steps()) as f32).round().clamp(0.0, u16::MAX as f32) as u16;
        // The spray: its longest flight in fine steps, and the first speed
        // that takes the fastest droplet that flies longest `spray_reach`
        // cells as it slows (speed x (0.7 steps + 0.3) in all: the sum of
        // 0.4 + 0.6 x t / steps over its steps, `big::spray_pace`).
        let spray = (r.spray_secs * (r.tps * self.world.fine_steps()) as f32).round().clamp(0.0, u16::MAX as f32) as u16;
        let spray_speed = r.spray_reach * self.world.scale() / (0.7 * spray as f32 + 0.3);
        let catch = self.catches(&by, dt);
        // The units the slime charges, for their walking distance (S1,
        // `time_unit_paths`), as the world takes them; a masked soldier
        // (the toolkit's mask, smell 0) is left out, so the charge's paths
        // don't lead to it.
        if r.unit_paths {
            let size = self.world.size();
            // Ore and cores are no unit to charge: the slime eats them.
            let masked = |w: &Who| matches!(*w, Who::Soldier(k) if self.soldiers[k].smell <= 0.0) || matches!(*w, Who::Drop(_));
            self.raids.unit_cells = units.iter().zip(&who).take(squad::UNITS_MAX).filter(|(_, w)| !masked(w)).map(|(u, _)| super::raid_cell((u.x, u.y), size)).collect();
        }
        let f = Frame { units, shots, slime: squad::slime_trail(6.0), crater, sink: r.sink, spray, spray_speed, spray_share: r.spray_share, catch };
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
        self.last_shots = f.shots.iter().zip(&out.shots).zip(&by).map(|((i, o), &(by, fired, cast, turret))| Shot { from: i.from, to: i.to, hit: o.hit, killed: o.killed, by, turret, fired, cast, r: i.r }).collect();
        self.flown.clear();
        if r.bullet_speed > 0.0 {
            self.flown.extend_from_slice(&self.bullets);
        }
        self.landed(&f.shots, &out.shots);
        {
            // The soldiers' hits this frame and the dish cells they fell
            // in, each once (`hit_cells`).
            let bin = self.s().max(1.0);
            let mut cells: Vec<(i32, i32)> = out.shots.iter().zip(&by).filter(|(_, b)| b.0.is_some()).filter_map(|(o, _)| o.hit).map(|h| ((h.0 / bin) as i32, (h.1 / bin) as i32)).collect();
            self.hits += cells.len() as u32;
            cells.sort_unstable();
            cells.dedup();
            self.hit_cells += cells.len() as u32;
        }
        self.killed += out.shots.iter().map(|o| o.killed).sum::<u32>();
        self.turret_killed += out.shots.iter().zip(&by).filter(|(_, b)| b.0.is_none()).map(|(o, _)| o.killed).sum::<u32>();
        self.mortar_killed += out.shots.iter().zip(&by).filter(|(_, b)| b.3.and_then(|k| self.buildings.get(k)).is_some_and(|bd| bd.kind == Kind::Mortar)).map(|(o, _)| o.killed).sum::<u32>();
        let hits: Vec<(Option<(f32, f32)>, u32)> = out.shots.iter().map(|o| (o.hit, o.killed)).collect();
        self.render_pay(&hits);
        // The plants' fire's kills, every frame on both worlds (bug check
        // 11-15: they were counted only on a Scorch trail's frames, so a
        // CPU game without one never counted them, and the GPU's went to
        // the next blast). A Scorch trail's burns come after the step
        // (`Rounds::after_step`), there.
        let fire = self.world.burned();
        self.fire_killed += fire;
        self.killed += fire;
        // Gassed mould sleeps and does not bite (the squad toolkit's sleep
        // gas): its walkers are taken off each unit's touch.
        let calm = self.world.calm(&f.units.iter().map(|u| (u.x, u.y)).collect::<Vec<_>>(), r.touch * sc);
        // Your own sleep gas (2026-11-18, `Rules::doze`): the cloud each
        // soldier stood in this frame, from the same pass.
        let clouds = self.world.clouds();
        for (j, k) in who.iter().enumerate() {
            if let Who::Soldier(k) = *k {
                self.soldiers[k].doze = clouds.get(j).copied().unwrap_or(0.0);
            }
        }
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
                Who::Drop(i) => {
                    self.drop_out(i, u.touching, dt);
                    self.drift_core(i, dt);
                }
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
                    if u.heat > 0.0 {
                        s.hp -= r.fire_hurt * u.heat * dt;
                        if s.hp <= 0.0 {
                            s.alive = false;
                            s.hp = 0.0;
                            s.carry = 0.0;
                        }
                    }
                    if u.touching > 0 {
                        if self.first_touch.is_none() {
                            self.first_touch = Some((self.world.ticks() - self.landed) as f32 / r.tps as f32);
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
        self.time_surges(out.units.iter().map(|u| u.took as f64).sum());
        let now = self.world.ticks();
        if now / r.tps as u64 != t / r.tps as u64 {
            self.mould = self.world.living();
            self.mark_bulwark();
            self.guard_pay();
            self.cover_turn();
            self.count_stuck();
            self.check_broken();
            self.keep_panic_reach();
            self.time_raids();
            self.time_unit_paths();
            self.time_food_paths();
            self.time_lure_paths();
            self.time_fort();
            self.time_bosses();
            self.time_fruit();
            self.time_waves();
            self.time_heart();
            self.time_strikers();
            self.time_cores();
            self.time_starfall();
        }
        self.campaign_rules(now - t);
        self.check_end();
    }

    /// A scared piece's reach, past the gun's range (`Rules::panic_margin`):
    /// at the start and once a game second, so a range card moves it
    /// (`gun_range`, which the rounds' `Range` cards stretch).
    pub(super) fn keep_panic_reach(&mut self) {
        if self.rules.panic_margin >= 0.0 {
            // A wave's strike (`waves.rs`, 2026-11-20) reaches its masses,
            // which gathered `stand` past the guns' reach.
            let w = self.rules.wave;
            let strike = self.waves.live.as_ref().is_some_and(|v| v.phase == WavePhase::Strike);
            let extra = if strike { w.stand.max(0.0) + w.disc.max(0.0) } else { 0.0 };
            self.world.set_panic_r(self.gun_range_max() + self.rules.panic_margin + extra);
        }
    }

    /// **Food-driven surges** (`Rules::surge_food`), each frame: the meter
    /// fills with what the slime ate (the crystals' charge gone since the
    /// last frame, less the `took` the units harvested, as food), and a
    /// surge starts on the next step (`World::set_surge_at`) once the meter
    /// is full and `surge_min` has passed since the last, or once the
    /// world's `surge_every` has, eaten or not. The meter then empties. On
    /// a world with no surge, or with `surge_food` 0, the world keeps its
    /// own clock.
    pub(super) fn time_surges(&mut self, took: f64) {
        let r = self.rules;
        let now: f64 = self.world.crystals().iter().map(|c| c.1 as f64).sum();
        let ate = (self.charge_was - now - took).max(0.0);
        self.charge_was = now;
        self.slime_ate += (ate / r.charge.max(1e-6) as f64) as f32;
        let every = self.world.surge_every();
        if r.surge_food <= 0.0 || every == 0 {
            return;
        }
        let fine = self.world.fine_steps().max(1) as u64;
        let step = self.world.ticks() * fine;
        if !self.surge_timed {
            self.take_surges();
            return;
        }
        self.surge_meter += (ate / r.charge.max(1e-6) as f64) as f32;
        let since = step.saturating_sub(self.surge_last);
        let min = (r.surge_min.max(0.0) * (r.tps as u64 * fine) as f32).round() as u64;
        if (self.surge_meter >= r.surge_food && since >= min) || since >= every as u64 {
            self.world.set_surge_at((step + 1).min(SURGE_WAIT as u64 - 1) as u32);
            self.surge_last = step;
            self.surge_meter = 0.0;
            self.surges += 1;
        }
    }

    /// The game takes the surges from the world's clock (`time_surges`):
    /// calm until the meter fills, counted from now (the landing).
    pub(super) fn take_surges(&mut self) {
        if self.rules.surge_food > 0.0 && self.world.surge_every() > 0 {
            self.world.set_surge_at(SURGE_WAIT);
            self.surge_last = self.world.ticks() * self.world.fine_steps().max(1) as u64;
            self.surge_meter = 0.0;
            self.surge_timed = true;
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
        // Half a dish cell walked is a move (a soldier settling in its
        // formation shifts less than that).
        let step = 0.5 * self.s();
        let walked = self.soldiers.iter().zip(&self.stuck_at).any(|(s, &(x, y))| s.alive && (s.x - x).hypot(s.y - y) > step);
        self.stuck_at = self.soldiers.iter().map(|s| (s.x, s.y)).collect();
        // A standing pump still has a crystal to give, and a building going
        // up is the bank already spent on a plan.
        let working = self.buildings.iter().any(|b| b.alive && (b.built < 1.0 || b.kind == Kind::Pump));
        // With free shots and a crystal still charged, a living squad
        // always has a way on (walk to it, or fire), so standing still is
        // the player's choice, which the clock judges (the chores thread's
        // window test, 11-15: planet 6 in squad mode ended "Stuck" at 0:34
        // before the squad had met the slime).
        let way_on = self.rules.price <= 0.0 && self.world.crystals().iter().any(|c| c.1 > 0.0);
        if self.alive() > 0 && !walked && !working && !way_on && self.shots == self.stuck_mark.0 && taken <= self.stuck_mark.1 && self.bank < self.rules.soldier_cost {
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
            // No recruit for a squad that is gone where that ends the
            // round (`Rules::wipe_ends`): it could come on the step the
            // last soldier fell, before the end is checked.
            let gone = r.wipe_ends > 0.0 && self.alive() == 0;
            if self.alive() < 2 && self.bank < r.soldier_cost && !held && !gone {
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
        // In a run the last soldier's death ends it (`Rules::wipe_ends`).
        let wiped = self.rules.wipe_ends > 0.0 || (self.bank < self.rules.soldier_cost && !recruit);
        if self.alive() == 0 && wiped {
            self.end = Some(End::SquadWiped(t));
        } else if self.mould == 0 {
            self.end = Some(End::MouldWiped(t));
        } else if t >= self.rules.cap {
            self.cap_held = Some(self.share());
            self.end = Some(End::Cap(self.take_share()));
        }
    }
}
