//! The player's input: orders, the HUD's buttons, the toolkit, keys. Moved
//! out of `larder-world.rs` unchanged (C9, 2026-11-14).

use super::*;

impl App {
    pub(super) fn units(&self) -> Vec<Unit> {
        self.game.as_ref().map(|g| g.soldiers.iter().enumerate().map(|(id, s)| Unit { id, team: 0, x: s.x, y: s.y, alive: s.alive }).collect()).unwrap_or_default()
    }

    /// The next loadout this world can cast (every card on the GPU world
    /// too since 11-14), kept for the next game too: H, or the loadout
    /// strip's button.
    pub(super) fn next_round(&mut self) {
        // The gun range builds its gun in the armory, which H opens.
        if self.args.range.is_some() {
            return self.open_armory();
        }
        // A run's gun is what its shop built.
        if self.args.run.is_some() {
            self.hud.alert("In a run the gun holds the cards you bought: change it in the shop between planets", None, theme::WARN);
            return;
        }
        let ok = |l: &str| self.game.as_ref().is_none_or(|g| rounds::parse(l).unwrap_or_default().iter().all(|c| match c {
            rounds::Card::Gas => g.world.has_gas(),
            rounds::Card::Germ => g.world.has_germ(),
            _ => true,
        }));
        let mut i = self.round;
        for _ in 0..LOADOUTS.len() {
            i = (i + 1) % LOADOUTS.len();
            if ok(LOADOUTS[i].1) {
                break;
            }
        }
        self.round = i;
        let (name, cards) = LOADOUTS[i];
        let round = rounds::parse(cards).unwrap_or_default();
        // The squad owns the gun and its cards (owner 11-14 16:46Z): in the
        // squad mode H loads the controlled squad's own (`Cmd::Load`); in
        // the RTS round, the army's round as before.
        let squad = self.game.as_ref().filter(|g| g.rules.squads).and_then(|g| g.active);
        let whose = match (squad, self.game.as_mut()) {
            (Some(q), Some(g)) => {
                g.apply(Cmd::Load(q as u8, Some(rounds::Deck::new(&round))));
                format!("Squad {} round", q + 1)
            }
            (_, g) => {
                self.rounds = Rounds::new(round.clone(), RoundRules::default());
                // Squad 0 may hold rolled bullets of its own (Ctrl+H):
                // back to the army's round.
                if let Some(g) = g {
                    g.apply(Cmd::Load(0, None));
                }
                "Round".to_string()
            }
        };
        let what = if cards == "none" { format!("{whose}: plain shots") } else { format!("{whose}: {name} ({}), a card a shot", cards.replace('+', ", ")) };
        self.hud.alert(&what, None, theme::ACCENT);
    }

    /// A test gun for the controlled squad (squad 0 in the RTS round):
    /// Shift+H rolls the next kind (rifle, carbine, long gun, scattergun
    /// in turn) for this planet's depth, a new roll each press, so a
    /// playtest sees the kinds and how far their rolls stray. Goes through
    /// `Cmd::Arm`, so the replay keeps it.
    pub(super) fn next_gun(&mut self) {
        if self.args.range.is_some() {
            return self.open_armory();
        }
        // A run's guns come from its shop (the roguelike thread).
        if self.args.run.is_some() {
            self.hud.alert("In a run the squad's gun is the one it carries: change it in the shop between planets", None, theme::WARN);
            return;
        }
        let Some(g) = self.game.as_mut() else { return };
        let q = g.active.unwrap_or(0);
        let n = self.gun_rolls;
        self.gun_rolls += 1;
        let kind = GunKind::ALL[n as usize % GunKind::ALL.len()];
        let gun = Gun::roll_kind(kind, self.args.seed.wrapping_mul(1_000_003) ^ n as u64, self.planet_depth);
        g.apply(Cmd::Arm(q as u8, Some(gun)));
        let what = format!("Gun: {}, {}", draw::capital(&gun.title()), gun.line());
        self.hud.alert(&what, None, theme::ACCENT);
    }

    /// Test bullets for the controlled squad (squad 0 in the RTS round):
    /// Ctrl+H rolls bullets for this planet's depth (`Ammo::roll`) and
    /// loads as many as the squad's gun takes in budget and slots
    /// (`rounds::fit_gun`), a new load each press, so a playtest sees the
    /// sets and how far their rolls stray. Goes through `Cmd::Load`, so
    /// the replay keeps it.
    pub(super) fn next_bullets(&mut self) {
        if self.args.range.is_some() {
            return self.open_armory();
        }
        if self.args.run.is_some() {
            self.hud.alert("In a run the gun holds the bullets you bought: change them in the shop between planets", None, theme::WARN);
            return;
        }
        let Some(g) = self.game.as_mut() else { return };
        let q = g.active.unwrap_or(0);
        let gun = g.gun(q);
        let n = self.ammo_rolls;
        self.ammo_rolls += 1;
        let cards = bullet_load(&gun, self.args.seed, n, self.planet_depth);
        g.apply(Cmd::Load(q as u8, Some(rounds::Deck::new(&cards))));
        let names: Vec<String> = cards.iter().map(|c| c.title()).collect();
        let what = format!("Bullets: {} (cost {} of {})", names.join(", "), rounds::cost(&cards), gun.budget);
        self.hud.alert(&what, None, theme::ACCENT);
    }

    pub(super) fn living(&self, ids: &[usize]) -> Vec<usize> {
        let Some(g) = &self.game else { return Vec::new() };
        ids.iter().copied().filter(|&k| g.soldiers.get(k).is_some_and(|s| s.alive)).collect()
    }

    /// An order to each soldier; with Shift held, queued after what each
    /// is doing (`Cmd::Queue`) instead.
    pub(super) fn order_all(&mut self, ids: &[usize], o: impl Fn(usize) -> Order) {
        let ids = self.living(ids);
        let queue = self.shift;
        if let Some(g) = self.game.as_mut() {
            for (n, k) in ids.into_iter().enumerate() {
                g.apply(if queue { Cmd::Queue(k, o(n)) } else { Cmd::Order(k, o(n)) });
            }
        }
    }

    /// In the squad mode, order `o` to the controlled squad as one; false
    /// in the RTS round (or with no squad controlled). Every order the
    /// card gives goes this way there (drain, attack-move, burn, build):
    /// given soldier by soldier, a squad on the move took them back at its
    /// next step (`Game::squad_turn`; bug sweep, 2026-11-14).
    pub(super) fn squad_order(&mut self, o: Order) -> bool {
        let q = self.game.as_ref().filter(|g| g.rules.squads && self.sq.is_some()).and_then(|g| g.active);
        match (q, self.game.as_mut()) {
            (Some(q), Some(g)) => g.apply(Cmd::SquadOrder(q as u8, o)),
            _ => false,
        }
    }

    /// Right click: harvest a crystal under it, else walk there.
    pub(super) fn order_at(&mut self, ids: &[usize], at: (f32, f32)) {
        let o = self.order_for(at);
        self.order_all(ids, |_| o);
    }

    /// The building just placed (the newest): the selected soldiers go to
    /// raise it (Shift: after their orders, so Shift + clicks lay a build
    /// queue they raise in turn). With none selected, the nearest free
    /// soldier walks there, however far (`Cmd::Send`; owner's playtest
    /// 2026-11-12: a build ordered anywhere sends a soldier), and with
    /// Shift the soldier sent for the last one queues it instead, so one
    /// builder works down the queue rather than each ghost taking another.
    /// In the squad mode the controlled squad goes as a squad
    /// (`Cmd::SquadOrder`, which also ends a move it was sent on; before,
    /// that move took the build order back the next step).
    pub(super) fn builders(&mut self, sel: &[usize]) {
        let Some(g) = &self.game else { return };
        let Some(b) = g.buildings.len().checked_sub(1) else { return };
        if g.buildings[b].built >= 1.0 {
            return;
        }
        let squad = g.active.filter(|_| self.sq.is_some() && !self.shift);
        let last = self.last_sent.filter(|&k| self.shift && g.soldiers.get(k).is_some_and(|s| s.alive && matches!(s.order, Order::Build(_))));
        let picked = !self.living(sel).is_empty();
        if picked && squad.is_none() {
            return self.order_all(sel, |_| Order::Build(b));
        }
        let Some(g) = self.game.as_mut() else { return };
        if let Some(q) = squad {
            g.apply(Cmd::SquadOrder(q as u8, Order::Build(b)));
        } else if let Some(k) = last {
            g.apply(Cmd::Queue(k, Order::Build(b)));
        } else if g.apply(Cmd::Send(b)) {
            // The one soldier `send` put on it.
            self.last_sent = (0..g.soldiers.len()).find(|&k| g.soldiers[k].alive && g.soldiers[k].order == Order::Build(b));
        }
    }

    /// What a point means as an order: harvest a crystal under it, else
    /// walk there.
    pub(super) fn order_for(&self, at: (f32, f32)) -> Order {
        match &self.game {
            Some(g) if g.crystal_at(at).is_some() => Order::Clear(at.0, at.1),
            _ => Order::Move(at.0, at.1),
        }
    }

    /// The larder's rally point at `at` (`Cmd::Rally`).
    pub(super) fn rally(&mut self, at: (f32, f32)) {
        let o = self.order_for(at);
        if let Some(g) = self.game.as_mut() {
            g.apply(Cmd::Rally(o));
        }
    }

    /// A click on the world picked no soldier: the larder, if it is under
    /// the click.
    pub(super) fn on_larder(&self, at: Option<(f32, f32)>) -> bool {
        match (&self.game, at) {
            (Some(g), Some(p)) => (p.0 - g.larder.0).hypot(p.1 - g.larder.1) <= 3.0 * g.world.scale(),
            _ => false,
        }
    }

    /// A burn line shared out: each soldier keeps the whole line burning
    /// (`Order::Burn`) from its own place along it.
    pub(super) fn burn(&mut self, ids: &[usize], from: (f32, f32), to: (f32, f32)) {
        if !self.squad_order(Order::Burn(from, to)) {
            self.order_all(ids, |_| Order::Burn(from, to));
        }
    }

    pub(super) fn hud_events(&mut self, events: Vec<HudEvent>, event_loop: &ActiveEventLoop) {
        for ev in events {
            match self.hud_event(ev) {
                Ok(true) => event_loop.exit(),
                Ok(false) => {}
                Err(e) => return self.fail(event_loop, e),
            }
        }
    }

    /// What one HUD event does to the game and the window; true to quit.
    /// Apart from `hud_events` (2026-11-12) so the smoke test's hand can
    /// press every command on the card as a player does (`press_every`).
    pub(super) fn hud_event(&mut self, ev: HudEvent) -> Result<bool, String> {
        let sel = self.living(&self.controls.selected);
        // (The block keeps the arms where they stood inside the loop, so
        // the threads adding commands merge without re-indenting.)
        {
            match ev {
                HudEvent::SelectOnly(k) => {
                    self.controls.selected = vec![k];
                    self.larder_sel = false;
                }
                HudEvent::Deselect(k) => self.controls.selected.retain(|&q| q != k),
                ev @ (HudEvent::Squad(_) | HudEvent::SquadInto(_) | HudEvent::SquadLink(_) | HudEvent::SquadSplit | HudEvent::SquadCombine | HudEvent::SquadCover | HudEvent::SquadFree) => self.squad_panel_event(ev),
                HudEvent::Group(g) => self.controls.selected = self.living(&self.controls.groups[g].clone()),
                HudEvent::Idle => {
                    let after = self.controls.selected.first().copied();
                    if let Some(k) = self.next_idle(after).or_else(|| self.next_idle(None)) {
                        self.controls.selected = vec![k];
                        if let Some(s) = self.game.as_ref().map(|g| g.soldiers[k]) {
                            self.camera.centre_on((s.x, s.y));
                        }
                    }
                }
                HudEvent::Menu(MenuItem::Opened) => {
                    self.menu_paused = self.paused;
                    self.paused = true;
                }
                HudEvent::Menu(MenuItem::Resume) => self.paused = self.menu_paused,
                HudEvent::Menu(MenuItem::Restart) => {
                    self.start()?;
                    self.paused = false;
                }
                HudEvent::Menu(MenuItem::MainMenu) => self.back_to_menu()?,
                HudEvent::Menu(MenuItem::Continue) => self.carry_on()?,
                HudEvent::Menu(MenuItem::Quit) => {
                    self.save_replay();
                    return Ok(true);
                }
                HudEvent::Command { id } => match id {
                    NEXT_ROUND => self.next_round(),
                    STOP => self.order_all(&sel, |_| Order::Hold),
                    HOLD_FIRE | DEFEND | FREE => {
                        let st = match id {
                            HOLD_FIRE => Stance::HoldFire,
                            DEFEND => Stance::Defend,
                            _ => Stance::Free,
                        };
                        if let Some(g) = self.game.as_mut() {
                            for &k in &sel {
                                g.apply(Cmd::Stance(k, st));
                            }
                        }
                    }
                    SOW | ROAD => {
                        // A toggle on the selected soldiers: on for all unless
                        // all already do it, then off.
                        let t = if id == SOW { Task::Feed } else { Task::Road };
                        if let Some(gd) = self.game.as_mut().and_then(|g| g.world.garden.as_mut()) {
                            let on = !sel.iter().all(|&k| gd.tasks.get(k) == Some(&t));
                            for &k in &sel {
                                gd.set_task(k, if on { t } else { Task::None });
                            }
                        }
                    }
                    BUILD_PAGE => self.build_page = true,
                    BACK => self.build_page = false,
                    TOOLS_PAGE => self.tools_page = true,
                    T_BACK => self.tools_page = false,
                    LURES => {
                        if let Some(g) = self.game.as_mut() {
                            let lures: Vec<usize> = (0..g.buildings.len()).filter(|&k| g.buildings[k].alive && matches!(g.buildings[k].kind, Kind::Lure | Kind::Siren)).collect();
                            // All on if any is off, else all off.
                            let on = lures.iter().any(|&k| !g.buildings[k].on);
                            for k in lures {
                                if g.buildings[k].on != on {
                                    g.apply(Cmd::Toggle(k));
                                }
                            }
                        }
                    }
                    BUY => {
                        if let Some(g) = self.game.as_mut() {
                            if g.apply(Cmd::Buy) {
                                let s = *g.soldiers.last().unwrap();
                                let what = if g.rules.squads { "A soldier has joined your squad" } else { "A soldier has joined at the larder" };
                                self.hud.alert(what, Some((s.x, s.y)), theme::GOOD);
                            }
                        }
                    }
                    _ => {}
                },
                // In the squad mode the card's orders go to the controlled
                // squad as one (`Cmd::SquadOrder`), as its right click does;
                // given soldier by soldier, a squad on the move took them
                // back at its next step (bug sweep, 2026-11-14).
                HudEvent::Point { id: HARVEST, at } if self.squad_order(Order::Clear(at.0, at.1)) => {}
                HudEvent::Point { id: HARVEST, at } => self.order_all(&sel, |_| Order::Clear(at.0, at.1)),
                HudEvent::Point { id: RALLY, at } => self.rally(at),
                // A then click: on the core's ring, attack the core; else
                // attack-move there.
                HudEvent::Point { id: ATTACK, at } => {
                    let Some(g) = &self.game else { return Ok(false) };
                    let on_core = (at.0 - g.core.0).hypot(at.1 - g.core.1) <= g.rules.core_r * g.world.scale() * 1.5;
                    let o = if on_core { Order::Attack } else { Order::Fight(at.0, at.1) };
                    if !self.squad_order(o) {
                        self.order_all(&sel, |_| o);
                    }
                }
                HudEvent::Point { id: CANCEL_BUILD, at } => {
                    if let Some(g) = self.game.as_mut() {
                        match g.ghost_at(at) {
                            Some(b) => {
                                let what = match g.buildings[b].kind {
                                    Kind::Wall { .. } => "wall",
                                    k => k.name(),
                                };
                                if g.apply(Cmd::Unbuild(b)) {
                                    let back = g.build_price(g.buildings[b].kind, g.buildings[b].at);
                                    self.hud.alert(&format!("Called off the {what}: {back:.0} food back"), Some(g.buildings[b].at), theme::GOOD);
                                }
                            }
                            None => self.hud.alert("No unfinished building there", None, theme::WARN),
                        }
                    }
                }
                // The pump on a crystal that has one upgrades it.
                HudEvent::Point { id: PUMP, at } if self.game.as_ref().is_some_and(|g| g.pump_crystal(at).and_then(|c| g.pump_on(c)).is_some()) => {
                    if let Some(g) = self.game.as_mut() {
                        let b = g.pump_crystal(at).and_then(|c| g.pump_on(c)).unwrap();
                        let price = g.upgrade_price(b);
                        if g.apply(Cmd::Upgrade(b)) {
                            let rate = g.pump_draw(b) * g.pump_yield(b);
                            self.hud.alert(&format!("Pump upgraded to level {}: {rate:.0} food a second", g.buildings[b].level), Some(g.buildings[b].at), theme::GOOD);
                        } else {
                            let why = match price {
                                Some(p) => format!("Upgrading the pump costs {p:.0} food"),
                                None if !g.buildings[b].up() => "The pump is still being built".to_string(),
                                None => "The pump is at its top level".to_string(),
                            };
                            self.hud.alert(&why, None, theme::WARN);
                        }
                    }
                }
                HudEvent::Point { id, at } if build_kind(id).is_some() => {
                    if let Some(g) = self.game.as_mut() {
                        let kind = build_kind(id).unwrap();
                        if let Some(why) = g.build_blocked(kind, at) {
                            self.hud.alert(&format!("Can't build a {} there: {why}", kind.name()), None, theme::WARN);
                        } else if g.apply(Cmd::Build(kind, at)) {
                            self.builders(&sel);
                        }
                    }
                }
                HudEvent::Point { id: id @ (T_BLAST | T_MINE | T_GERM | T_GAS), at } => {
                    let tool = match id {
                        T_BLAST => Tool::Blast(at),
                        T_MINE => Tool::Mine(at),
                        T_GERM => Tool::Germ(at),
                        _ => Tool::Gas(at),
                    };
                    self.use_tool(tool);
                }
                HudEvent::Line { id: T_CORD, from, to } => self.use_tool(Tool::Cord(from, to)),
                HudEvent::Line { id: BURN, from, to } => self.burn(&sel, from, to),
                HudEvent::Line { id: WALL, from, to } => {
                    if let Some(g) = self.game.as_mut() {
                        let price = g.wall_price(from, to);
                        if !g.apply(Cmd::Wall(from, to)) {
                            let why = if g.bank < price { format!("A wall that long costs {price:.0} food") } else if let Some(why) = g.build_blocked(Kind::Wall { to }, from) { format!("Can't build a wall there: {why}") } else { "Can't build a wall there".to_string() };
                            self.hud.alert(&why, None, theme::WARN);
                        } else {
                            self.builders(&sel);
                        }
                    }
                }
                _ => {}
            }
        }
        Ok(false)
    }

    /// Uses a toolkit tool now, between steps (a running step finishes
    /// first, as a command does), and says why not if it can't.
    pub(super) fn use_tool(&mut self, t: Tool) {
        if self.game.as_ref().is_some_and(|g| g.in_flight()) {
            if let Some(g) = self.game.as_mut() {
                g.finish_step();
            }
            self.catch_up();
        }
        let Some(g) = self.game.as_mut() else { return };
        let k = &self.kit;
        let at = match t {
            Tool::Blast(p) | Tool::Mine(p) | Tool::Germ(p) | Tool::Gas(p) => p,
            Tool::Cord(a, b) => ((a.0 + b.0) / 2.0, (a.1 + b.1) / 2.0),
            Tool::Smell(..) => return,
        };
        let held = g.soldiers.iter().enumerate().any(|(i, s)| s.alive && k.held(i) > 0);
        let near = k.in_reach(g, at);
        let r = k.rules;
        if !self.kit.use_tool(g, t) {
            let why = if !held {
                "No soldier holds a tool: soldiers take them at the larder".to_string()
            } else if !near {
                format!("Too far: a soldier holding a tool must be within {:.0} cells", r.reach)
            } else if matches!(t, Tool::Cord(..)) {
                format!("A cord is at most {:.0} cells long, and needs the food", r.cord_max)
            } else {
                "Not enough food, or not on open ground".to_string()
            };
            self.hud.alert(&why, None, theme::WARN);
        }
    }

    pub(super) fn next_idle(&self, after: Option<usize>) -> Option<usize> {
        let g = self.game.as_ref()?;
        let idle: Vec<usize> = (0..g.soldiers.len()).filter(|&k| g.soldiers[k].alive && g.soldiers[k].order == Order::Hold).collect();
        let start = after.and_then(|a| idle.iter().position(|&k| k > a)).unwrap_or(0);
        idle.get(start).copied()
    }

    pub(super) fn key(&mut self, code: KeyCode, event_loop: &ActiveEventLoop) {
        match code {
            KeyCode::KeyP | KeyCode::Pause => self.paused = !self.paused,
            KeyCode::Tab => self.fast = !self.fast,
            KeyCode::KeyN if self.args.run.is_some() => self.hud.alert("In a run the next map is the next planet: beat this one", None, theme::WARN),
            KeyCode::KeyN => {
                // On a generated planet, N rolls a new one at the same depth.
                if self.args.map.as_deref().is_some_and(|m| slime_mould::mould::levelgen::spec_planet(m, 0).is_some()) {
                    self.args.seed = fresh_seed();
                } else {
                    let next = self.args.map.as_deref().and_then(|m| MAPS.iter().position(|&n| n == m)).map_or(0, |i| (i + 1) % MAPS.len());
                    self.args.map = Some(MAPS[next].to_string());
                    self.name = MAPS[next].to_string();
                }
                if let Err(e) = self.start() {
                    self.fail(event_loop, e);
                }
            }
            KeyCode::KeyH if self.shift => self.next_gun(),
            KeyCode::KeyH if self.ctrl => self.next_bullets(),
            KeyCode::KeyH => self.next_round(),
            KeyCode::KeyM => {
                if let Some(v) = self.view.as_mut() {
                    v.minimap = !v.minimap;
                }
            }
            KeyCode::KeyK => {
                if let Some(l) = self.listener.as_mut() {
                    l.voice = !l.voice;
                    let what = if l.voice { "The mould's voice on" } else { "The mould's voice off" };
                    self.hud.alert(what, None, theme::ACCENT);
                }
            }
            KeyCode::KeyO => {
                self.sound_on = !self.sound_on;
                if let Some(sp) = self.speaker.as_mut() {
                    sp.send([SoundCmd::Master(if self.sound_on { DEFAULT_MASTER } else { 0.0 })]);
                }
                let what = match (self.speaker.is_some(), self.sound_on) {
                    (false, _) => "No sound on this machine",
                    (true, true) => "Sound on",
                    (true, false) => "Sound off",
                };
                self.hud.alert(what, None, theme::ACCENT);
            }
            _ => {}
        }
    }
}

/// Load `n` of test bullets for `gun` (Ctrl+H, `ammo=N`): bullets rolled
/// for `depth` (`Ammo::roll`), each kept while the gun's budget and slots
/// take it (`rounds::fit_gun`); a plain bullet if none fits.
pub(super) fn bullet_load(gun: &Gun, seed: u64, n: u32, depth: u32) -> Vec<rounds::Card> {
    let base = seed.wrapping_mul(1_000_033) ^ ((n as u64) << 8);
    let mut cards: Vec<rounds::Card> = Vec::new();
    for j in 0..(4 * gun.slots as u64) {
        let mut more = cards.clone();
        more.push(rounds::Card::Ammo(rounds::Ammo::roll(base + j, depth)));
        if rounds::fit_gun(more.clone(), gun).is_ok() {
            cards = more;
        }
        if cards.len() >= gun.slots as usize {
            break;
        }
    }
    if cards.is_empty() {
        cards.push(rounds::Card::Plain);
    }
    cards
}
