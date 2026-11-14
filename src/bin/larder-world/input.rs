//! The player's input: orders, the HUD's buttons, the toolkit, keys. Moved
//! out of `larder-world.rs` unchanged (C9, 2026-11-14).

use super::*;

impl App {
    pub(super) fn units(&self) -> Vec<Unit> {
        self.game.as_ref().map(|g| g.soldiers.iter().enumerate().map(|(id, s)| Unit { id, team: 0, x: s.x, y: s.y, alive: s.alive }).collect()).unwrap_or_default()
    }

    /// The next loadout this world can cast (the GPU world has no germ
    /// yet), kept for the next game too: H, or the loadout panel's Swap.
    pub(super) fn next_round(&mut self) {
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
        self.rounds = Rounds::new(rounds::parse(cards).unwrap_or_default(), RoundRules::default());
        let what = if cards == "none" { "Round: plain shots".to_string() } else { format!("Round: {name} ({}), recharge {:.1} s", cards.replace('+', ", "), rounds::recharge(&self.rounds.round, &self.rounds.rules)) };
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

    /// Right click: harvest a crystal under it, else walk there.
    pub(super) fn order_at(&mut self, ids: &[usize], at: (f32, f32)) {
        let o = self.order_for(at);
        self.order_all(ids, |_| o);
    }

    /// The building just placed (the newest): the selected soldiers go to
    /// raise it (Shift: after their orders). With none selected, the
    /// nearest free soldier walks there, however far (`Cmd::Send`; owner's
    /// playtest 2026-11-12: a build ordered anywhere sends a soldier).
    pub(super) fn builders(&mut self, sel: &[usize]) {
        let Some(g) = &self.game else { return };
        let Some(b) = g.buildings.len().checked_sub(1) else { return };
        if g.buildings[b].built < 1.0 {
            if self.living(sel).is_empty() {
                if let Some(g) = self.game.as_mut() {
                    g.apply(Cmd::Send(b));
                }
            } else {
                self.order_all(sel, |_| Order::Build(b));
            }
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
        self.order_all(ids, |_| Order::Burn(from, to));
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
                                self.hud.alert("A soldier has joined at the larder", Some((s.x, s.y)), theme::GOOD);
                            }
                        }
                    }
                    _ => {}
                },
                HudEvent::Point { id: HARVEST, at } => self.order_all(&sel, |_| Order::Clear(at.0, at.1)),
                HudEvent::Point { id: RALLY, at } => self.rally(at),
                // A then click: on the core's ring, attack the core; else
                // attack-move there.
                HudEvent::Point { id: ATTACK, at } => {
                    let Some(g) = &self.game else { return Ok(false) };
                    let on_core = (at.0 - g.core.0).hypot(at.1 - g.core.1) <= g.rules.core_r * g.world.scale() * 1.5;
                    self.order_all(&sel, |_| if on_core { Order::Attack } else { Order::Fight(at.0, at.1) });
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
        let Some(g) = self.game.as_mut() else { return };
        if g.in_flight() {
            g.finish_step();
            kit_between(&mut self.kit, g, &self.controls.selected);
            self.rounds.after_step(g);
        }
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
            KeyCode::KeyN => {
                let next = self.args.map.as_deref().and_then(|m| MAPS.iter().position(|&n| n == m)).map_or(0, |i| (i + 1) % MAPS.len());
                self.args.map = Some(MAPS[next].to_string());
                self.name = MAPS[next].to_string();
                if let Err(e) = self.start() {
                    self.fail(event_loop, e);
                }
            }
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
