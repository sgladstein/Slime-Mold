//! What the window draws from the game: marks, the HUD's model, the
//! ground, the end and mission screens. Moved out of `larder-world.rs`
//! unchanged (C9, 2026-11-14).

use super::*;

/// The look's ground field from the game's world: rock, wet and agar from
/// the engine's CPU mirror; crystals from its own cells (`pile_of`, so
/// each map crystal's size and shape is the one the mould eats); with
/// `charge`, each crystal cell's share of its full charge; with `health`,
/// each wall cell's share of its full health, and a wall the GPU has worn
/// through drawn as rubble (the mirror keeps it a wall); and the map's
/// soil, which the engine plays since 2026-11-12 (WM3: sand thins the
/// trail, clay keeps it, barren ground starves and takes no births), so
/// the look draws it (#84). `depth` is the crystals' depth
/// (`App::depth`), worked out once a world.
pub(super) fn ground_codes(g: &Game<GpuWorld>, depth: &[u8], charge: Option<&[f32]>, health: Option<&[f32]>) -> Vec<u32> {
    let d = &g.world.e.dish;
    let cells = d.w * d.h;
    let pile = |i: usize| d.pile_of.get(i).copied().unwrap_or(0) as usize;
    let charge_share: Vec<f32> = match charge {
        Some(c) => (0..cells).map(|i| if pile(i) > 0 { c[i] / d.pile_fill.get(pile(i) - 1).copied().unwrap_or(d.p.crystal_charge).max(1e-6) } else { 0.0 }).collect(),
        None => Vec::new(),
    };
    let broken = |i: usize| health.is_some_and(|h| d.wall.get(i).copied().unwrap_or(false) && h[i] <= 0.0);
    let wall_share: Vec<f32> = match health {
        Some(h) => (0..cells).map(|i| if d.wall.get(i).copied().unwrap_or(false) { let full = d.wall_health.get(i).copied().unwrap_or(f32::INFINITY); if full.is_finite() && full > 0.0 { h[i] / full } else { 1.0 } } else { 0.0 }).collect(),
        None => Vec::new(),
    };
    let wall: Vec<bool> = (0..cells).map(|i| d.wall.get(i).copied().unwrap_or(false) && !broken(i)).collect();
    let agar: Vec<bool> = (0..cells).map(|i| d.agar[i] || broken(i)).collect();
    let mut codes = look::ground(d.w, cells, &Fields { agar: &agar, rock: &d.rock, wet: &d.wet, wall: &wall, wall_share: &wall_share, crystal: &d.pile_of, charge_share: &charge_share, elev: &[], soil: &d.soil, depth });
    for (i, c) in codes.iter_mut().enumerate() {
        if broken(i) {
            *c = look::code(Ground::Rubble, 0.0);
        }
    }
    look::mark_far(d.w, &mut codes);
    codes
}

pub(super) fn end_screen(end: End, g: &Game<GpuWorld>) -> (String, Vec<String>) {
    let clock = |t: f32| format!("{}:{:02}", t as u32 / 60, t as u32 % 60);
    let title = match end {
        End::MouldWiped(t) => format!("The mould is wiped out at {}", clock(t)),
        End::SquadWiped(t) => format!("The squad is lost at {}", clock(t)),
        End::Overrun(t) => format!("The mould overran the larder at {}", clock(t)),
        End::Cap(s) if s > 0.5 => "Time: you took more of the crystals than the mould".to_string(),
        End::Cap(s) if s < 0.5 => "Time: the mould took more of the crystals".to_string(),
        End::Cap(_) => "Time: an even race".to_string(),
    };
    let mut lines = Vec::new();
    if let End::Cap(s) = end {
        let m = g.mould as f32 / g.start_mould.max(1) as f32;
        lines.push(format!("You took {:.0}% of the charge gone from the crystals; the mould is {} walkers, {m:.1} times its start", s * 100.0, thousands(g.mould)));
    }
    lines.push(summary(g));
    lines.push(format!("Score {:+.2} (a win is over 1, a loss under -1)", g.score().unwrap_or(0.0)));
    (title, lines)
}

/// A mission's end: stars and the cause in the title, then what happened.
pub(super) fn mission_screen(tr: &Tracker, g: &Game<GpuWorld>) -> (String, Vec<String>) {
    let v = tr.verdict.expect("a verdict");
    let stars = if v.stars == 0 { "Mission lost".to_string() } else { format!("{}: {}", tr.mission.name, "*".repeat(v.stars as usize)) };
    (stars, vec![tr.cause_line(), summary(g)])
}

pub(super) fn summary(g: &Game<GpuWorld>) -> String {
    format!("{} soldiers left, {:.0} food harvested, {} walkers killed with {} shots, {} walls", g.alive(), g.harvested, thousands(g.killed as usize), g.shots, g.walls)
}

/// A building's mark colour; None for walls, which the world draws.
pub(super) fn building_colour(k: Kind) -> Option<[u8; 4]> {
    match k {
        Kind::Turret => Some([150, 200, 255, 255]),
        Kind::Lure => Some([255, 196, 90, 255]),
        Kind::Pump => Some([90, 220, 230, 255]),
        Kind::Outpost => Some([214, 196, 150, 255]),
        Kind::Siren => Some([255, 120, 120, 255]),
        Kind::Wall { .. } => None,
    }
}

pub(super) fn crystal_name(i: usize) -> String {
    if i == 0 {
        "The mould's home crystal".into()
    } else {
        format!("Crystal {i}")
    }
}

pub(super) fn stance_name(s: Stance) -> &'static str {
    match s {
        Stance::HoldFire => "hold fire",
        Stance::Defend => "defend",
        Stance::Free => "free fire",
    }
}

pub(super) fn title_case(s: &str) -> String {
    let s = s.rsplit('/').next().unwrap_or(s);
    let mut c = s.chars();
    match c.next() {
        Some(f) => format!("The {}{}", f.to_uppercase(), c.as_str()),
        None => String::new(),
    }
}

/// "trap" as "Trap".
fn capital(s: &str) -> String {
    let mut c = s.chars();
    c.next().map_or(String::new(), |f| f.to_uppercase().chain(c).collect())
}

pub(super) fn thousands(n: usize) -> String {
    if n >= 10_000 {
        format!("{:.1}k", n as f32 / 1000.0)
    } else {
        format!("{n}")
    }
}

impl App {
    /// The look's ground from the world's cells: once a game second (the
    /// crystals drain, the walls wear), and at once when a wall goes up.
    pub(super) fn refresh_ground(&mut self) {
        let (Some(g), Some(buf)) = (&self.game, &self.ground) else { return };
        let t = g.time();
        // A step still running would make the read-back below wait for it
        // (on llvmpipe the split's prepare p95 was 14.9 ms against 1.3 ms
        // for whole steps, 2026-11-12), so the timed refresh waits for a
        // frame between steps, up to a second late; a new wall reads at once.
        let late = t - self.ground_at;
        if (late < GROUND_EVERY || (g.in_flight() && late < 2.0 * GROUND_EVERY)) && g.walls == self.ground_walls {
            return;
        }
        let e = &g.world.e;
        let floats = |b: Vec<u8>| -> Vec<f32> { b.chunks_exact(4).map(|c| f32::from_le_bytes(c.try_into().unwrap())).collect() };
        let charge = floats(read_back(&e.device, &e.queue, &e.charge));
        let health = e.dish.wall.iter().any(|&w| w).then(|| floats(read_back(&e.device, &e.queue, &e.health)));
        let codes = ground_codes(g, &self.depth, Some(&charge), health.as_deref());
        e.queue.write_buffer(buf, 0, &codes.iter().flat_map(|c| c.to_le_bytes()).collect::<Vec<u8>>());
        (self.ground_at, self.ground_walls) = (t, g.walls);
    }

    pub(super) fn marks(&mut self) {
        self.marks.clear();
        let Some(g) = &self.game else { return };
        let sc = g.world.scale();
        let zoom = self.camera.zoom;
        // The larder (the squad's base: bought soldiers stand there and
        // carried food is banked there) and the core's ring (stand in it
        // with no mould near for 5 s to take it). Game objects, not the
        // simulation, so marks.
        if g.rules.larder {
        // The larder is a sprite in soldier A's style (art review
        // 2026-11-14: it was a plain beige square), drawn from the game's
        // state: its charge cells lit while the bank holds food, darkening
        // as the mould holds it with the bank dry (the overrun count). The
        // faint ring is the ground a walker must reach to hold it.
        self.marks.building_pulse(g.larder, 1.6 * sc, LARDER_SPRITE, 1.0, 1.0 - g.overrun_share(), g.bank >= 1.0, self.banked / BANKED);
        self.marks.ring(g.larder, 3.0 * sc, [214, 196, 150, 90]);
        } else {
            // No larder (the squad mode): each guarded crystal's ring, the
            // game's count of who holds it (`Game::guarded`).
            for &i in &g.guarded {
                let c = g.world.crystals()[i].0;
                self.marks.ring(c, g.world.crystal_r(i) + 2.0 * sc, [94, 214, 132, 170]);
            }
        }
        if g.rules.squads {
            // Each squad's number over its middle (the key that controls
            // it), the controlled one bright.
            for (n, q) in g.live_squads().into_iter().enumerate() {
                if let Some(m) = g.middle(q) {
                    let lit = g.active == Some(q);
                    let colour = if lit { [255, 255, 255, 255] } else { [200, 210, 220, 170] };
                    let label = format!("{}{}", n + 1, if lit { " *" } else { "" });
                    self.marks.label(m, (-4.0, -(4.0 * sc * zoom).clamp(14.0, 60.0)), &label, if lit { 2.0 } else { 1.5 }, colour);
                }
            }
        }
        if self.larder_sel {
            self.marks.selected(g.larder, 3.6 * sc * zoom);
        }
        if let Order::Move(x, y) | Order::Clear(x, y) = g.rally {
            if self.larder_sel || !self.controls.selected.is_empty() {
                self.marks.line(g.larder, (x, y), 2.0, [214, 196, 150, 110]);
                self.marks.ring((x, y), 1.2 * sc, [214, 196, 150, 160]);
            }
        }
        // The core ring marks a home that can be taken, so a headless mould
        // (the default since 11-12 20:21Z: no king) shows none.
        if g.core_rule.kind != CoreKind::Headless {
            let core = if g.core_down.is_some() { [94, 214, 132, 200] } else { [242, 98, 98, 150] };
            self.marks.ring(g.core, g.rules.core_r * sc, core);
        }
        // Buildings: sprites in soldier A's style (`Marks::building`,
        // src/world/sprites/buildings.svg), drawn from the game's state:
        // rising as they are built, darkening as they are hurt, the lamp
        // lit while working. Their effects (a lure's scent, a pump's
        // dimming crystal, a turret's hits) are the simulation's own fields;
        // walls are ground. The bars stay as UI.
        for (k, b) in g.buildings.iter().enumerate() {
            if !b.alive {
                continue;
            }
            // A ghost's progress (owner's playtest 2026-11-12: "a faint
            // building with a progress bar that fills in as it builds so I
            // can tell how long until it is complete"): the bar is
            // `built`, and beside it the seconds left at the rate its
            // builders raise it now (`Game::raise_left`), "on the way"
            // while they walk, "no builder" when nobody is sent.
            // The bar spans the building on screen (10-40 pixels each side).
            let wide = (g.rules.build.touch * sc * zoom).clamp(10.0, 40.0);
            let progress = |marks: &mut Marks, at: (f32, f32), below: f32| {
                marks.bar(at, below, (wide, 2.0), b.built, [235, 235, 235, 255], [40, 40, 40, 220]);
                // The seconds left as text; "on the way" and "no builder"
                // as a figure striding, or standing struck through (art
                // director, 11-14: two ghosts side by side ran their words
                // into each other).
                let (x, y) = (wide + 4.0, below - 5.0);
                let ink = [235, 235, 235, 230];
                match (g.raise_left(k), g.raising(k)) {
                    (Some(t), _) => marks.label(at, (x, y), &format!("{}s", t.ceil() as u32), 1.5, ink),
                    (None, (_, n)) if n > 0 => marks.icon(at, (x, y), WALKING, 1.5, ink),
                    _ => {
                        marks.icon(at, (x, y), STANDING, 1.5, [200, 200, 200, 200]);
                        marks.icon(at, (x, y), SLASH, 1.5, [235, 80, 70, 255]);
                    }
                }
            };
            if let Kind::Wall { to } = b.kind {
                // A wall is ground once whole; until then a faint dashed
                // line where it will stand, with its bar at the middle.
                if !b.up() {
                    self.marks.line(b.at, to, 3.0, [235, 220, 180, 170]);
                    progress(&mut self.marks, ((b.at.0 + to.0) / 2.0, (b.at.1 + to.1) / 2.0), 6.0);
                }
                continue;
            }
            if building_colour(b.kind).is_none() {
                continue;
            }
            let Some(kind) = sprite_kind(b.kind) else { continue };
            let half = g.rules.build.touch * sc * 0.7;
            // A lure switched off is not working.
            let working = b.working && (!matches!(b.kind, Kind::Lure | Kind::Siren) || b.on);
            self.marks.building(b.at, half, kind, if b.up() { 1.0 } else { b.built }, b.hp / b.max_hp.max(1.0), working);
            let below = (half * zoom).max(2.0) + 5.0;
            // Health once it stands (a ghost's lower hp is its progress,
            // shown by the bar below and the sprite's seam).
            if b.up() {
                self.marks.health(b.at, below, b.hp / b.max_hp.max(1.0));
            }
            if !b.up() {
                progress(&mut self.marks, b.at, below + 4.0);
            }
        }
        for (k, s) in g.soldiers.iter().enumerate() {
            if !s.alive {
                continue;
            }
            if let Order::Burn(a, b) = s.order {
                self.marks.line(a, b, 4.0, [255, 150, 60, 200]);
            }
            if self.controls.selected.contains(&k) {
                // The order, then the queue (Shift) as a chain of lines,
                // fainter down the queue; red for fighting, white else.
                let mut from = (s.x, s.y);
                let orders = std::iter::once(s.order).chain(s.queue[..s.queued as usize].iter().copied());
                for (n, o) in orders.enumerate() {
                    let to = match o {
                        Order::Move(x, y) | Order::Fight(x, y) => Some((x, y)),
                        Order::Clear(..) if n == 0 && s.homing => Some(g.larder),
                        Order::Clear(x, y) => Some((x, y)),
                        Order::Attack => Some(g.core),
                        Order::Burn(a, b) => Some(((a.0 + b.0) / 2.0, (a.1 + b.1) / 2.0)),
                        Order::Build(b) => g.buildings.get(b).map(|b| b.at),
                        Order::Hold => None,
                    };
                    let Some(to) = to else { continue };
                    let a = if n == 0 { 120 } else { 80 };
                    let colour = if matches!(o, Order::Attack | Order::Fight(..)) { [255, 110, 110, a] } else { [255, 255, 255, a - 30] };
                    self.marks.line(from, to, 2.0, colour);
                    if n > 0 {
                        self.marks.ring(to, 1.0 * sc, colour);
                    }
                    from = to;
                }
            }
        }
        // A shot's line, only while shots are rays: with bullets flying
        // (`bullet_speed` > 0) the bullet's own streak shows its path, and
        // the line beside it was a painted effect.
        for t in self.tracers.iter().filter(|_| g.rules.bullet_speed <= 0.0) {
            let a = (255.0 * (1.0 - t.age / TRACER)).clamp(0.0, 255.0) as u8;
            self.marks.line(t.from, t.to, 0.0, if t.hit { [255, 236, 170, a] } else { [200, 200, 200, a / 2] });
        }
        // Each bullet in flight (`Game::bullets`) as Noita draws a spell
        // (Reports/noita-spell-look-2026-11-14.md): a streak along this
        // frame's travel (`draw_as_long`), in the colour of the payload its
        // soldier's next hit casts (`Rounds::next_cast`; a plain shot, a
        // turret's, or one fired while the gun recharges is pale gold),
        // and a light that lights the real slime as it flies over it.
        // Its shape is its cards: wide is bigger, quiet dimmer, pierce a
        // long thin lance, split a fatter head, scatter three heads.
        // (Before 11-14: a 3-pixel disc and a line, the same for every
        // card; the owner's playtest: "really bad".)
        self.lights.clear();
        let t = g.world.ticks();
        let tps = g.rules.tps as f32;
        let travel = g.rules.bullet_speed * sc * 0.06;
        for b in &g.bullets {
            // A payload this world can't cast yet flies plain, as it lands.
            let castable = |c: &rounds::Cast| match c.payload {
                rounds::Card::Gas => g.world.has_gas(),
                rounds::Card::Germ => g.world.has_germ(),
                _ => true,
            };
            let cast = b.by.and_then(|k| self.rounds.next_cast(k, t)).filter(|&(c, charging)| !charging && castable(&c)).map(|(c, _)| c);
            let look = card_look(cast.map(|c| c.payload));
            let wide = rounds::RoundRules::default().wide.powi(cast.map_or(0, |c| c.wide) as i32);
            let quiet = cast.is_some_and(|c| c.quiet);
            let core = (0.22 * sc * zoom).clamp(1.4, 5.0) * wide * if b.split > 0 { 1.3 } else { 1.0 } * if b.pierce > 0 { 0.8 } else { 1.0 };
            let len = travel * if b.pierce > 0 { 2.4 } else { 1.0 };
            let tail = (b.at.0 - b.dir.0 * len, b.at.1 - b.dir.1 * len);
            let glow = [look.glow[0], look.glow[1], look.glow[2], if quiet { 90 } else { 210 }];
            self.marks.streak(b.at, tail, core, glow, look.core);
            if cast.is_some_and(|c| c.scatter) {
                let side = (-b.dir.1 * 0.9 * sc, b.dir.0 * 0.9 * sc);
                for f in [-1.0, 1.0] {
                    let h = (b.at.0 + side.0 * f - b.dir.0 * 0.6 * sc, b.at.1 + side.1 * f - b.dir.1 * 0.6 * sc);
                    self.marks.streak(h, (h.0 - b.dir.0 * len * 0.6, h.1 - b.dir.1 * len * 0.6), core * 0.6, glow, look.core);
                }
            }
            self.lights.push(Light { at: b.at, reach: 7.0 * sc * wide, lift: 3.0, colour: look.light, strength: if quiet { 0.35 } else { 0.9 } });
        }
        // A burst lights the world for its first tenth of a second (Noita's
        // explosion light lasts 0.08 s at 8 radii), brighter the more it
        // killed; after that only what it did stays: the hole, the crater,
        // the sleepers, the sick colour.
        for p in self.rounds.pops.iter().rev() {
            let age = (t.saturating_sub(p.tick)) as f32 / tps;
            if age > BURST_LIGHT {
                break;
            }
            let f = 1.0 - age / BURST_LIGHT;
            let look = card_look(Some(p.payload));
            let at = (p.burst.at.0 * sc, p.burst.at.1 * sc);
            // A bomb's light reaches its throw ring and grows with the walkers
            // thrown, so the flash and the spray read as one (#225).
            let thrown = (p.flung as f32 / 400.0).min(1.0);
            self.lights.push(Light { at, reach: 3.0 * p.burst.fling.max(p.burst.r) * sc, lift: 4.0, colour: look.light, strength: f * f * (1.6 + (p.killed as f32 / 120.0).min(2.0) + thrown) });
        }
        // Plain hits (a shot with no cast) light their spot as the tracer
        // fades, and a muzzle lights round the soldier as it fires.
        for tr in self.tracers.iter().filter(|tr| tr.hit) {
            self.lights.push(Light { at: tr.to, reach: 4.0 * sc, lift: 3.0, colour: [255.0, 214.0, 150.0], strength: 0.8 * (1.0 - tr.age / TRACER) });
        }
        for (k, s) in g.soldiers.iter().enumerate() {
            let fl = self.looks.get(k).map_or(0.0, |l| l.flash / FLASH);
            if s.alive && fl > 0.0 {
                self.lights.push(Light { at: (s.x, s.y), reach: 5.0 * sc, lift: 3.0, colour: [255.0, 220.0, 170.0], strength: 0.7 * fl });
            }
        }
        // A lodged cast (delay, thick) is an ember where it waits, its light
        // swelling as its fuse burns down (delay) or steady (thick).
        for (c, at, due) in self.rounds.lodged() {
            let look = card_look(Some(c.payload));
            let left = due.saturating_sub(t) as f32 / tps;
            let f = if c.trigger == Some(rounds::Card::Delay) { 1.0 - (left / self.rounds.rules.delay_secs).clamp(0.0, 1.0) } else { 0.4 };
            self.marks.streak(at, at, (0.18 * sc * zoom).clamp(1.2, 4.0), [look.glow[0], look.glow[1], look.glow[2], (90.0 + 150.0 * f) as u8], look.core);
            self.lights.push(Light { at, reach: 3.0 * sc, lift: 2.0, colour: look.light, strength: 0.3 + 0.7 * f });
        }
        // A soldier is drawn its touch wide (walkers that close drain it),
        // and at least 8 pixels across the radius, so it reads at any zoom
        // (at 0.9 dish cells, the stand-in's size, it was a 4-pixel dot at
        // the whole map's zoom).
        // Drawn 1.15 times that and at least 10 pixels since the art review
        // (2026-11-14: a 20-pixel olive disc at zoom 6); elbow room
        // (`Rules::elbow`, 1.6 touch radii) keeps the bodies apart. The
        // dead first, so the living are drawn over them.
        let r = (g.rules.touch * sc * 1.15).max(10.0 / zoom);
        let order = g.soldiers.iter().enumerate().filter(|(_, s)| !s.alive).chain(g.soldiers.iter().enumerate().filter(|(_, s)| s.alive));
        for (k, s) in order {
            let l = self.looks.get(k).copied().unwrap_or(Look { facing: 0.0, flash: 0.0, take: 0.0, at: (s.x, s.y), alive: s.alive });
            // The load as charge in its pack, the take as light at its
            // hands (harvest look, 11-14; the carry bar is gone).
            let carry = if g.rules.carry > 0.0 { s.carry / g.rules.carry } else { 0.0 };
            self.marks.soldier_working((s.x, s.y), r, l.facing, team_colour(0), l.flash / FLASH, s.alive, carry, l.take / TAKE);
        }
        // The bars after every figure, so a neighbour never hides one.
        for s in &g.soldiers {
            if s.alive {
                // Health only when hurt (art direction rule 6).
                let below = (r * zoom * 0.9).max(2.0) + 5.0;
                self.marks.health((s.x, s.y), below, s.hp / g.rules.hp);

            }
        }
        let units = self.units();
        self.controls.prune(&units);
        self.controls.marks(&units, &self.camera, r, &mut self.marks);
    }

    pub(super) fn model(&self, hover: Option<(f32, f32)>, ghost: Option<Ghost>) -> Model {
        let Some(g) = &self.game else { return Model::default() };
        let r = g.rules;
        let sc = g.world.scale();
        let sel = self.living(&self.controls.selected);
        let t = g.time();
        let clock = |t: f32| format!("{}:{:02}", t as u32 / 60, t as u32 % 60);
        let rate = self.bank_log.front().filter(|f| t - f.0 >= 2.0).map(|&(t0, b0)| (g.bank - b0) / (t - t0));
        let lost = g.soldiers.iter().filter(|s| !s.alive).count();
        let took = g.take_share();
        let mut m = Model {
            resources: vec![
                // The rate is the bank's net change, so it counts the squad's rations
                // (PR 4); when it is falling and the bank would be dry within a
                // minute it says when (the design review, 11-12 21:51Z: show the
                // drain before soldiers start losing health).
                Resource {
                    icon: Icon::Crystal,
                    colour: theme::ENERGY,
                    // Rations off by default since 2026-11-14 (owner): then the
                    // label leaves out the squad's keep.
                    label: if r.ration > 0.0 {
                        format!("Food: crystal charge carried home. The squad eats {:.0} a second; soldiers, shots and walls cost it", r.ration * g.alive() as f32)
                    } else {
                        "Food: crystal charge carried home. Soldiers, walls and buildings cost it".to_string()
                    },
                    value: format!("{:.0}", g.bank),
                    rate: rate.map(|v| match g.bank / -v {
                        dry if v < 0.0 && dry < 60.0 => format!("{v:+.0}/s, dry in {dry:.0} s"),
                        _ => format!("{v:+.0}/s"),
                    }),
                },
                Resource { icon: Icon::Soldier, colour: theme::TEXT, label: format!("Soldiers alive (at most {})", r.max_squad), value: format!("{}", g.alive()), rate: (lost > 0).then(|| format!("-{lost} lost")) },
                Resource { icon: Icon::Mould, colour: [196, 220, 120, 255], label: if g.world.garden.is_some() { "The enemy mould: walkers alive".into() } else { "The mould: walkers alive".into() }, value: thousands(g.mould), rate: g.core_down.map(|_| "core taken".into()) },
                Resource { icon: Icon::Target, colour: theme::ACCENT, label: "Your share of the charge gone from the crystals (with the mould's size, the score at 15:00)".into(), value: format!("{:.0}%", took * 100.0), rate: None },
            ],
            clock: format!("{} / {}", clock(t), clock(r.cap)),
            speed: if self.paused { Some("Paused".into()) } else if self.fast { Some("x4".into()) } else { None },
            objective: Some(if let Some(tr) = &self.mission {
                tr.progress_line()
            } else if g.core_rule.kind == CoreKind::Headless {
                "Wipe out the mould, or out-harvest it and cut it down by 15:00".into()
            } else if g.core_down.is_some() {
                "Core taken: wipe out the mould, or cut it down by 15:00".into()
            } else {
                "Take the mould's core, or out-harvest it and cut it down by 15:00".into()
            }),
            idle: g.soldiers.iter().filter(|s| s.alive && s.order == Order::Hold).count(),
            ..Model::default()
        };
        m.selection = sel.iter().map(|&k| {
            let s = g.soldiers[k];
            let badge = match s.order {
                Order::Hold => None,
                Order::Move(..) => Some(Icon::Move),
                Order::Fight(..) => Some(Icon::Attack),
                Order::Build(..) => Some(Icon::Wall),
                Order::Clear(..) => Some(Icon::Crystal),
                Order::Burn(..) => Some(Icon::Burn),
                Order::Attack => Some(Icon::Attack),
            };
            Card { id: k, icon: Icon::Soldier, team: 0, hp: s.hp / r.hp, badge }
        }).collect();
        if sel.len() == 1 {
            let k = sel[0];
            let s = g.soldiers[k];
            let doing = match s.order {
                Order::Hold => "holding",
                Order::Move(..) => "moving",
                Order::Fight(..) if s.dense > 0 => "fighting its way",
                Order::Fight(..) => "attack-moving",
                Order::Build(b) => match g.buildings.get(b) {
                    Some(bd) => match bd.kind {
                        Kind::Turret => "building a turret",
                        Kind::Lure => "building a lure",
                        Kind::Pump => "building a pump",
                        Kind::Outpost => "building an outpost",
                        Kind::Siren => "building a siren",
                        Kind::Wall { .. } => "building a wall",
                    },
                    None => "building",
                },
                Order::Clear(..) if s.homing => "carrying charge home",
                Order::Clear(..) => "harvesting a crystal",
                Order::Burn(..) => "holding a burn line",
                Order::Attack => "attacking the core",
            };
            m.info = Some(Info {
                icon: Icon::Soldier,
                colour: team_colour(0),
                title: format!("Soldier {}", k + 1),
                subtitle: format!("{doing}, {}", stance_name(s.stance)),
                hp: Some((s.hp / r.hp, format!("{:.0} / {:.0}", s.hp, r.hp))),
                stats: vec![
                    ("Carrying".into(), format!("{:.0} / {:.0}", s.carry, r.carry)),
                    ("Stance".into(), stance_name(s.stance).into()),
                    ("Range".into(), format!("{:.0} cells", g.gun_range())),
                    ("A shot".into(), if r.price > 0.0 { format!("{:.0} food", r.price) } else { format!("free, one every {:.1} s", r.shot_every) }),
                    ("Mould on it".into(), format!("{}", s.touching)),
                    ("Tools held".into(), format!("{} / {}", self.kit.held(k), self.kit.rules.carry_max)),
                ],
                note: None,
            });
        }
        if self.larder_sel && sel.is_empty() {
            m.info = Some(Info {
                icon: Icon::Hold,
                colour: [214, 196, 150, 255],
                title: "The larder".into(),
                subtitle: match g.rally {
                    Order::Clear(..) => "new soldiers go to harvest".into(),
                    Order::Move(..) => "new soldiers go to the rally point".into(),
                    _ => "new soldiers wait here".into(),
                },
                hp: None,
                stats: vec![("Bank".into(), format!("{:.0}", g.bank)), ("Harvested".into(), format!("{:.0}", g.harvested)), ("Eaten by the mould".into(), format!("{:.0}", g.stolen)), ("Soldiers".into(), format!("{} / {}", g.alive(), r.max_squad))],
                note: Some("B buys a soldier. Right-click sets where new soldiers go; on a crystal they start harvesting it.".into()),
            });
        }
        // The hover card: a crystal or the larder under the cursor.
        if let Some(p) = hover {
            if (p.0 - g.larder.0).hypot(p.1 - g.larder.1) <= 3.0 * sc {
                m.hover = Some(Info {
                    icon: Icon::Hold,
                    colour: [214, 196, 150, 255],
                    title: "The larder".into(),
                    subtitle: "your base".into(),
                    hp: None,
                    stats: vec![("Bank".into(), format!("{:.0}", g.bank)), ("Harvested".into(), format!("{:.0}", g.harvested)), ("Eaten by the mould".into(), format!("{:.0}", g.stolen))],
                    note: Some("Harvesters bring charge here; bought soldiers stand here. Mould touching it eats the bank.".into()),
                });
            } else if let Some((i, c)) = g.crystal_at(p) {
                let charge = g.world.crystals()[i].1;
                let full = self.charge0.get(i).copied().unwrap_or(1.0).max(1.0);
                m.hover = Some(Info {
                    icon: Icon::Crystal,
                    colour: theme::ENERGY,
                    title: crystal_name(i),
                    subtitle: format!("{:.0}% of its charge left", 100.0 * charge / full),
                    hp: Some((charge / full, format!("{:.0} food", charge / r.charge))),
                    stats: vec![("Size".into(), format!("{:.0} cells across", 2.0 * g.world.crystal_r(i) / sc))],
                    note: Some(if (c.0 - g.core.0).hypot(c.1 - g.core.1) < 1.0 { "Where the mould grew from. Stand on it with no mould near for 5 s to take it.".into() } else { "Right-click with soldiers selected to harvest it and hold it.".into() }),
                });
            }
        }
        let any = !sel.is_empty();
        let stance = sel.first().map(|&k| g.soldiers[k].stance).filter(|&f| sel.iter().all(|&k| g.soldiers[k].stance == f));
        let need = |cost: f32| (g.bank < cost).then(|| format!("Need {:.0} more food", cost - g.bank));
        let no_sel = || Some("Select soldiers first".to_string());
        let full = g.alive() >= r.max_squad;
        let mut cmds = vec![
            Command { target: Target::Point { radius: 2.0 * sc }, ready: any, why_not: no_sel(), about: format!("Walk there, stopping to fire freely at any mould in reach on the way. Click the core to attack it: stand on it with no mould near for {:.0} s and it can grow no more. Shift queues it.", r.core_secs), ..Command::new(ATTACK, (0, 0), Icon::Attack, "Attack-move", KeyCode::KeyA) },
            Command { ready: any, why_not: no_sel(), about: "Drop the order and stand still; still fires by its stance.".into(), ..Command::new(STOP, (0, 1), Icon::Stop, "Stop", KeyCode::KeyS) },
            Command { target: Target::Point { radius: 2.0 * sc }, ready: any, why_not: no_sel(), about: format!("Go to a crystal, hold it, and carry its charge to the larder, {:.0} food a load. Right-click on a crystal does the same.", r.carry), ..Command::new(HARVEST, (0, 2), Icon::Crystal, "Harvest a crystal", KeyCode::KeyG) },
            Command { target: Target::Line { width: r.strike * sc }, ready: any, why_not: no_sel(), about: "Stand by a line and keep firing along it, so the mould cannot cross.".into(), ..Command::new(BURN, (0, 3), Icon::Burn, "Burn a line", KeyCode::KeyF) },
            Command { ready: any, on: stance == Some(Stance::HoldFire), why_not: no_sel(), about: "Do not shoot at all.".into(), ..Command::new(HOLD_FIRE, (1, 0), Icon::Close, "Hold fire", KeyCode::KeyZ) },
            Command { ready: any, on: stance == Some(Stance::Defend), why_not: no_sel(), about: format!("Shoot only mould touching the soldier, within {:.0} cells, or on the crystal it is harvesting. The default.", r.defend), ..Command::new(DEFEND, (1, 1), Icon::Eye, "Defend", KeyCode::KeyX) },
            Command { ready: any, on: stance == Some(Stance::Free), why_not: no_sel(), about: format!("Shoot the thickest mould in reach, {:.0} cells.{}", g.gun_range(), if r.price > 0.0 { format!(" Every shot costs {:.0} food.", r.price) } else { String::new() }), ..Command::new(FREE, (1, 2), Icon::Target, "Free fire", KeyCode::KeyC) },
            Command {
                cost: Some(format!("{:.0}", r.soldier_cost)),
                ready: g.bank >= r.soldier_cost && !full,
                why_not: if full { Some(format!("The squad is at its most, {}", r.max_squad)) } else { need(r.soldier_cost) },
                about: "A new soldier at the larder.".into(),
                ..Command::new(BUY, (2, 0), Icon::Recruit, "Buy a soldier", KeyCode::KeyB)
            },
            Command {
                target: Target::Line { width: r.wall_width * sc },
                cost: Some(format!("{:.0}/cell", r.wall_cost)),
                ready: g.bank >= r.wall_cost,
                why_not: need(r.wall_cost),
                about: "A wall the mould has to wear through, slowly. Both ends within a soldier's range; costs by its length.".into(),
                ..Command::new(WALL, (2, 1), Icon::Wall, "Lay a wall", KeyCode::KeyV)
            },
            Command {
                target: Target::Point { radius: 2.0 * sc },
                ready: true,
                about: "Where bought soldiers go: on a crystal they start harvesting it, elsewhere they walk there. Also: select the larder and right-click.".into(),
                ..Command::new(RALLY, (2, 2), Icon::Move, "Rally point", KeyCode::KeyR)
            },
        ];
        // The player's own mould (own=1): Sow on the main page (the free slot),
        // Road on the tools page beside the det cord (main's slots are full).
        let mut road: Option<Command> = None;
        if let Some(gd) = &g.world.garden {
            let all = |t: Task| any && sel.iter().all(|&k| gd.tasks.get(k) == Some(&t));
            cmds.push(Command { ready: any, on: all(Task::Feed), why_not: no_sel(), about: format!("Sow your own mould (green): while on, the soldier spends {:.0} food a second from the bank, spread as charge where it walks, which your mould eats and grows on. Walk it where you want the mould to go. Press again to stop.", gd.tend.feed), ..Command::new(SOW, (1, 3), Icon::Crystal, "Sow your mould", KeyCode::KeyW) });
            road = Some(Command { ready: any, on: all(Task::Road), why_not: no_sel(), about: "Lay a road of your mould's trail where the soldier walks, free: your mould follows it. Lead it to food or into the enemy, where it converts the enemy's slime it outweighs. Press again to stop.".into(), ..Command::new(ROAD, (1, 2), Icon::Move, "Lay a road", KeyCode::KeyY) });
        }
        cmds.push(Command {
            about: "Turrets, lures, pumps, outposts and sirens.".into(),
            ..Command::new(BUILD_PAGE, (2, 2), Icon::Turret, "Build", KeyCode::KeyQ)
        });
        cmds.push(Command {
            about: "The squad's tools: blast, det cord, mine, germ, sleep gas. Every soldier at the larder picks one up; a selected soldier there takes up to five, and each one held costs carry.".into(),
            ..Command::new(TOOLS_PAGE, (2, 3), Icon::Grenade, "Tools", KeyCode::KeyE)
        });
        if self.tools_page {
            let k = &self.kit;
            let kr = k.rules;
            let held: u32 = (0..g.soldiers.len()).filter(|&i| g.soldiers[i].alive).map(|i| k.held(i)).sum();
            let tool = |id: u32, slot: (u8, u8), icon: Icon, name: &str, key: KeyCode, target: Target, price: f32, works: bool, about: String| Command {
                target,
                cost: Some(format!("{price:.0}")),
                ready: works && held > 0 && g.bank >= price,
                why_not: if !works { Some("Not on the GPU world yet".into()) } else if held == 0 { Some("No soldier holds a tool; they take them at the larder".into()) } else { need(price) },
                about,
                ..Command::new(id, slot, icon, name, key)
            };
            cmds = vec![
                tool(T_BLAST, (0, 0), Icon::Grenade, "Blast", KeyCode::KeyB, Target::Point { radius: kr.blast_r * sc }, kr.blast_price, true, format!("Kills the mould within {:.0} cells and leaves a crater it can't hold trail in for {:.0} s. The blast's alarm wakes and stirs the mould nearby. Thrown by a soldier holding a tool within {:.0} cells.", kr.blast_r, kr.crater_secs, kr.reach)),
                tool(T_CORD, (0, 1), Icon::Burn, "Det cord", KeyCode::KeyC, Target::Line { width: 2.0 * kr.cord_r * sc }, kr.cord_price, true, format!("A line of small blasts, each with its crater: a cut the mould can't knit for {:.0} s. {:.0} food a cell, at most {:.0} cells.", kr.crater_secs, kr.cord_price, kr.cord_max)),
                tool(T_MINE, (0, 2), Icon::Target, "Mine", KeyCode::KeyD, Target::Point { radius: kr.mine_r * sc }, kr.mine_price, true, format!("Goes off as a blast when {} walkers stand within {:.0} cells of it.", k.mine_trigger(g), kr.mine_r)),
                tool(T_GERM, (1, 0), Icon::Spore, "Germ", KeyCode::KeyG, Target::Point { radius: kr.germ_r * sc }, kr.germ_price, g.world.has_germ(), format!("Makes the mould within {:.0} cells sick; sickness travels along its tubes and starves what it passes.", kr.germ_r)),
                tool(T_GAS, (1, 1), Icon::Pause, "Sleep gas", KeyCode::KeyS, Target::Point { radius: kr.gas_r * sc }, kr.gas_price, g.world.has_gas(), format!("Puts the mould within {:.0} cells to sleep for {:.0} s: it doesn't eat or bite. A blast or gunfire nearby wakes it.", kr.gas_r, kr.gas_secs)),
                Command { about: "Back to the soldiers' commands.".into(), ..Command::new(T_BACK, (2, 3), Icon::Close, "Back", KeyCode::KeyE) },
            ];
            cmds.extend(road);
        }
        if self.build_page {
            let b = r.build;
            let lures = g.buildings.iter().filter(|q| q.alive && matches!(q.kind, Kind::Lure | Kind::Siren)).count();
            let room = g.standing() < b.max;
            let place = |id: u32, slot: (u8, u8), icon: Icon, name: &str, key: KeyCode, cost: f32, about: String| Command {
                target: Target::Point { radius: b.touch * sc },
                cost: Some(format!("{cost:.0}")),
                ready: g.bank >= cost && room,
                why_not: if room { need(cost) } else { Some(format!("At most {} buildings", b.max)) },
                about,
                ..Command::new(id, slot, icon, name, key)
            };
            cmds = vec![
                place(TURRET, (0, 0), Icon::Turret, "Turret", KeyCode::KeyT, b.turret_cost, format!("A gun that does not move: fires at the thickest mould within {:.0} cells every {:.1} s. Its hits call a Swarmer. Mould touching it wears it down.", b.turret_range, b.turret_every)),
                place(LURE, (0, 1), Icon::Bait, "Lure", KeyCode::KeyL, b.lure_cost, if b.lure_burn > 0.0 { format!("Gives off food scent, which the mould follows: pull it into your guns or off a crystal. Burns {:.0} food a second while on.", b.lure_burn) } else { "Gives off food scent, which the mould follows: pull it into your guns or off a crystal. Free to keep on.".to_string() }),
                place(PUMP, (0, 2), Icon::Crystal, "Pump", KeyCode::KeyU, b.pump_cost, format!("On a crystal: draws {:.0} food a second into the bank with no carriers. The mould smells the crystal.", b.pump_rate)),
                place(OUTPOST, (0, 3), Icon::Hold, "Outpost", KeyCode::KeyI, b.outpost_cost, "A second drop-off for carried charge. It smells of your bank, and mould touching it eats from the bank.".into()),
                place(SIREN, (1, 1), Icon::Alert, "Siren", KeyCode::KeyY, b.siren_cost, format!("Calls every {:.0} s with the alarm the mould's kinds answer: a Sleeper wakes, a Sporer bursts, a Swarmer floods in. The plain mould ignores it.", b.siren_every)),
                Command { ready: lures > 0, why_not: Some("No lures or sirens".into()), about: "Switch every lure and siren on, or every one off if all are on.".into(), ..Command::new(LURES, (1, 0), Icon::Bait, "Lures, sirens on/off", KeyCode::KeyJ) },
                Command { about: "Back to the soldiers' commands.".into(), ..Command::new(BACK, (2, 3), Icon::Close, "Back", KeyCode::KeyQ) },
            ];
        }
        for c in &mut cmds {
            if c.ready {
                c.why_not = None;
            }
        }
        if g.world.garden.is_some() {
            // The player's own mould beside the enemy's, in its colour.
            let own = g.world.counts_by_side().0;
            m.resources.insert(3, Resource { icon: Icon::Mould, colour: [120, 230, 90, 255], label: "Your own mould (green): walkers alive. Sow it (W); lay its roads from the tools page (E, then Y); it converts enemy slime it outweighs".into(), value: thousands(own), rate: None });
        }
        m.commands = cmds;
        m.buildings = g
            .buildings
            .iter()
            .filter(|b| b.alive)
            .map(|b| Site {
                kind: b.kind.name(),
                at: b.at,
                to: if let Kind::Wall { to } = b.kind { Some(to) } else { None },
                r: r.build.touch * sc,
                hp: b.hp,
                max_hp: b.max_hp,
                built: b.built,
                working: b.working,
            })
            .collect();
        (m.ghost_ok, m.ghost_note) = match ghost {
            Some((HARVEST, at, _)) => match g.crystal_at(at) {
                Some((i, _)) => (true, Some(format!("harvest {}", crystal_name(i).to_lowercase()))),
                None => (false, Some("not on a crystal".into())),
            },
            Some((WALL, at, Some(from))) => {
                let price = g.wall_price(from, at);
                if g.bank < price {
                    (false, Some(format!("{price:.0} food, {:.0} short", price - g.bank)))
                } else if let Some(why) = g.build_blocked(Kind::Wall { to: at }, from) {
                    (false, Some(why.into()))
                } else {
                    (true, Some(format!("{price:.0} food, release to lay")))
                }
            }
            Some((WALL, at, None)) if !g.rules.build.anywhere && !g.reachable(at) => (false, Some("out of every soldier's range".into())),
            Some((id, at, _)) if build_kind(id).is_some() => {
                let kind = build_kind(id).unwrap();
                match g.build_blocked(kind, at) {
                    Some(why) => (false, Some(why.into())),
                    None if g.reachable(at) => (true, Some(format!("{:.0} food, click to build", g.build_price(kind, at)))),
                    None => (true, Some(format!("{:.0} food, click: a soldier walks there", g.build_price(kind, at)))),
                }
            }
            _ => (true, None),
        };
        if let Some(tr) = self.mission.as_ref().filter(|t| t.verdict.is_some()) {
            m.end = Some(mission_screen(tr, g));
        } else if let Some(end) = g.end {
            m.end = Some(end_screen(end, g));
        }
        m.help = vec![
            ("Right click".into(), "Move there; on a crystal, harvest it".into()),
            ("Shift + order".into(), "Queue it after the soldiers' current orders".into()),
            ("Place a building".into(), "The selected soldiers walk there and raise it; idle soldiers nearby help".into()),
            ("A, click".into(), "Attack-move there; on the core, attack it".into()),
            ("Click the larder".into(), "Select it: B buys, right click sets the rally point".into()),
            ("P".into(), "Pause".into()),
            ("Tab".into(), "Four times faster, or back".into()),
            ("N".into(), "Start the next map".into()),
            ("Q".into(), "Build page: turret, lure, pump, outpost; Q again goes back".into()),
            ("E".into(), "Tools page: blast, det cord, mine, germ, sleep gas; E again goes back".into()),
        ];
        m.main_menu = self.base.is_some();
        m.loadout = Some(self.loadout(g, &sel));
        m
    }

    /// The loadout panel (`ui::Loadout`): the squad's round, with the gun
    /// of the one selected soldier (or the first selected, or the first
    /// alive) lit at the cast its next hit fires.
    fn loadout(&self, g: &Game<GpuWorld>, sel: &[usize]) -> Loadout {
        let rd = &self.rounds;
        let rech = rounds::recharge(&rd.round, &rd.rules);
        let scope: Vec<usize> = if sel.is_empty() { (0..g.soldiers.len()).filter(|&k| g.soldiers[k].alive).collect() } else { sel.to_vec() };
        let lead = scope.first().copied();
        let tick = g.world.ticks();
        let ticks = (rech * g.rules.tps as f32).max(1.0);
        let left = |k: usize| rd.gun(k).map_or(0.0, |(_, ready)| ready.saturating_sub(tick) as f32 / ticks);
        let next = lead.and_then(|k| rd.gun(k)).map(|(cast, _)| cast);
        let cards = gun_cards(&rd.round, &rd.rules, &g.rules, next, |c| match c {
            rounds::Card::Gas => g.world.has_gas(),
            rounds::Card::Germ => g.world.has_germ(),
            _ => true,
        });
        let (lname, lcards) = LOADOUTS[self.round];
        let name = if rounds::parse(lcards).unwrap_or_default() == rd.round { lname.to_string() } else { rounds::name(&rd.round) };
        let whose = match (sel.len(), lead) {
            (_, None) => "no soldiers".to_string(),
            (1, Some(k)) => format!("Soldier {}", k + 1),
            (0, Some(k)) => format!("Soldier {} (squad)", k + 1),
            (n, Some(k)) => format!("Soldier {} (of {n})", k + 1),
        };
        let casting = !rounds::casts(&rd.round, &rd.rules).is_empty();
        let note = (casting && scope.len() > 1).then(|| {
            let ready = scope.iter().filter(|&&k| left(k) <= 0.0).count();
            format!("{ready} of {} guns ready, recharge {rech:.1} s", scope.len())
        });
        Loadout {
            name: if rd.round.is_empty() { "Plain".into() } else { capital(&name) },
            whose,
            cards,
            slots: rounds::SLOTS,
            recharge: rech,
            charging: lead.map_or(0.0, left).clamp(0.0, 1.0),
            note: if rd.round.is_empty() { Some("No cards: H or the button for a loadout".into()) } else { note },
            cycle: Some((NEXT_ROUND, KeyCode::KeyH)),
        }
    }
}

/// A payload's colour in the panel: its bullets' glow (`card_look`), so a
/// card and its shot in the world match.
fn payload_colour(c: rounds::Card) -> [u8; 4] {
    let g = card_look(Some(c)).glow;
    [g[0], g[1], g[2], 255]
}

/// A card's glyph, frame and words for the loadout panel, with the round's
/// own numbers.
fn card_face(c: rounds::Card, r: &RoundRules, w: &Rules) -> (Icon, CardKind, String) {
    let split_n = w.split_n;
    use rounds::Card as C;
    let k = &r.kit;
    match c {
        C::Blast => (Icon::Blast, CardKind::Payload, format!("Where the shot hits: kills the walkers within {:.0} cells, leaves a crater the mould shuns, and lays alarm.", k.blast_r)),
        C::Gas => (Icon::Gas, CardKind::Payload, format!("Where the shot hits: the walkers within {:.0} cells fall asleep where they are. Spares the squad.", k.gas_r)),
        C::Germ => (Icon::Germ, CardKind::Payload, format!("Where the shot hits: infects the walkers within {:.0} cells; the sickness spreads through the body.", k.germ_r)),
        C::Bomb => (Icon::Bomb, CardKind::Payload, format!("Where the shot hits: kills the walkers within {:.0} cells and throws the living mould out to {:.0} cells straight outward; it lands and lives on.", r.bomb_r, r.bomb_r * r.bomb_ring)),
        C::Wide => (Icon::Wide, CardKind::Modifier, format!("The next payload is {:.1} times as wide.", r.wide)),
        C::Scatter => (Icon::Scatter, CardKind::Modifier, format!("The next payload goes off as {} smaller bursts round the hit.", r.scatter_n)),
        C::Quiet => (Icon::Quiet, CardKind::Modifier, "The next payload lays no alarm: a sleeper is not woken, a skittish mould is not warned.".into()),
        C::Delay => (Icon::Delay, CardKind::Trigger, format!("The next payload lodges and goes off {:.0} s later, once the mould has flowed back over the hole.", r.delay_secs)),
        C::Thick => (Icon::Thick, CardKind::Trigger, format!("The next payload lodges and waits until {} walkers crowd within {:.0} cells (at most {:.0} s): the mould sets off its own trap.", r.thick_n, r.thick_r, r.thick_secs)),
        C::Pierce => (Icon::Pierce, CardKind::Flight, "Every bullet flies on through one more slime hit, striking each.".into()),
        C::Split => (Icon::Split, CardKind::Flight, format!("Every bullet breaks into {split_n} pieces where it lands, fanned on past the hit.")),
        // Trails (2026-11-14): each reuses the glyph of the field it lays.
        C::Scorch => (Icon::Burn, CardKind::Flight, format!("Every bullet leaves a {:.0}-cell line of crater along its flight for {:.0} s: a fire line the mould shuns.", 2.0 * r.trail_r, r.trail_crater_secs)),
        C::Alarm => (Icon::Alert, CardKind::Flight, format!("Every bullet leaves a {:.0}-cell line of alarm along its flight: a fence the mould flees.", 2.0 * r.trail_r)),
        C::Mist => (Icon::Gas, CardKind::Flight, format!("Every bullet leaves a thin gas cloud along its flight ({:.1} a disc): the mould that drifts into it sleeps.", r.trail_cloud)),
        // Range (owner, 11-14 08:39Z): reuses free fire's target glyph.
        C::Range => (Icon::Target, CardKind::Flight, format!("Every soldier sees and shoots {:.0}% further ({:.0} cells a card), mouse-aimed volleys too.", 100.0 * w.range_card, w.range * w.range_card)),
    }
}

/// The loadout panel's cards: each card in order, the run of cards that
/// makes the cast `next` lit (a cast is the modifiers and trigger before a
/// payload, and the payload), cards after the last payload idle. Flight
/// cards ride every bullet, so they are never idle.
pub(super) fn gun_cards(round: &[rounds::Card], r: &RoundRules, w: &Rules, next: Option<usize>, castable: impl Fn(rounds::Card) -> bool) -> Vec<GunCard> {
    use rounds::Card as C;
    let payloads = round.iter().filter(|c| c.kind() == rounds::Kind::Payload).count();
    let mut cast = 0;
    let mut out = Vec::new();
    // Each card's cast: the payload it goes with.
    let mut runs = Vec::new();
    for &c in round {
        runs.push(cast);
        if c.kind() == rounds::Kind::Payload {
            cast += 1;
        }
    }
    let payload_of = |j: usize| round.iter().filter(|c| c.kind() == rounds::Kind::Payload).nth(j).copied();
    for (i, &c) in round.iter().enumerate() {
        let (icon, kind, mut about) = card_face(c, r, w);
        let flight = kind == CardKind::Flight;
        let idle = !flight && runs[i] >= payloads;
        let goes_with = if flight { None } else { payload_of(runs[i]) };
        let colour = match (kind, goes_with) {
            (CardKind::Flight, _) => [96, 214, 214, 255],
            (_, Some(p)) => payload_colour(p),
            (_, None) => [146, 156, 168, 255],
        };
        let price = match c {
            C::Blast => Some(r.kit.blast_price),
            C::Gas => Some(r.kit.gas_price),
            C::Germ => Some(r.kit.germ_price),
            C::Bomb => Some(r.bomb_price),
            _ => None,
        };
        if kind == CardKind::Payload && !castable(c) {
            about.push_str(" Not on this world yet: it casts nothing here.");
        }
        out.push(GunCard {
            icon: Some(icon),
            name: capital(c.name()),
            kind,
            colour,
            about,
            cost: price.map(|p| format!("{p:.0}")),
            next: !flight && !idle && next == Some(runs[i]),
            idle,
        });
    }
    out
}


/// How long a burst lights the world, game seconds.
const BURST_LIGHT: f32 = 0.12;

/// A payload's colours: the streak's glow and hot core, and its light.
pub(super) struct CardLook {
    pub glow: [u8; 3],
    pub core: [u8; 4],
    pub light: [f32; 3],
}

/// Each payload in its own colour, as each Noita spell is (blast hot
/// orange, gas violet, germ acid green; none, a plain or recharging shot,
/// pale gold).
pub(super) fn card_look(payload: Option<rounds::Card>) -> CardLook {
    let (glow, core, light): ([u8; 3], [u8; 3], [f32; 3]) = match payload {
        Some(rounds::Card::Blast) => ([255, 120, 40], [255, 236, 190], [255.0, 160.0, 80.0]),
        Some(rounds::Card::Gas) => ([165, 110, 255], [236, 222, 255], [180.0, 140.0, 255.0]),
        Some(rounds::Card::Germ) => ([110, 255, 70], [226, 255, 196], [140.0, 255.0, 110.0]),
        // A bomb burns redder and hotter than a blast (#225).
        Some(rounds::Card::Bomb) => ([255, 90, 30], [255, 228, 170], [255.0, 130.0, 60.0]),
        _ => ([255, 200, 110], [255, 248, 222], [255.0, 214.0, 150.0]),
    };
    CardLook { glow, core: [core[0], core[1], core[2], 255], light }
}
