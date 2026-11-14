//! **The start menu** (cloud plan 2026-11-14, C2): the window opens on it
//! when run with no arguments (or `menu=1`), over a live sandbox round,
//! the one Sandbox would start. It is `slime_mould::world::start`'s widget;
//! this file fills it from the game (the campaign's missions, today's
//! switches and their defaults, the key card) and turns a pick into the
//! round's `Args`, as the command line would have given them.
//!
//! Any other argument starts that round at once, as before, so every
//! command line keeps working; `menu=1` with them opens the menu with
//! those values preset (a preset that differs from the default shows in
//! amber), and `menu=PAGE` (`main`, `campaign`, `options`, `keys`) with
//! `look=DIR` films that page over the round (the pictures on the page).

use super::*;
use slime_mould::mould::run::{self as run, Pick};
use slime_mould::mould::run::{Face, Group};
use slime_mould::world::start::{Entry, Opt, Page, Shop, ShopArea, ShopRow, StartEvent, StartMenu};

/// The campaign's missions, as `mission=STEM` names them, in their order
/// (the number before the dash). Files that do not parse are skipped, with
/// a word on stderr. (The campaign thread's `Mission::list` will do this
/// in the library; switch to it when it lands.)
pub(super) fn missions() -> Vec<(String, Mission)> {
    let dir = Path::new(env!("CARGO_MANIFEST_DIR")).join("Reports/larder-campaign-2026-11-12/missions");
    let mut out: Vec<(u32, String, Mission)> = Vec::new();
    for e in std::fs::read_dir(&dir).into_iter().flatten().flatten() {
        let p = e.path();
        let Some(stem) = p.file_stem().and_then(|s| s.to_str()).filter(|_| p.extension().is_some_and(|x| x == "txt")) else { continue };
        let n = stem.split('-').next().and_then(|d| d.parse().ok()).unwrap_or(u32::MAX);
        match std::fs::read_to_string(&p).map_err(|e| e.to_string()).and_then(|t| Mission::parse(&t)) {
            Ok(m) => out.push((n, stem.to_string(), m)),
            Err(e) => eprintln!("larder-world: {}: {e}", p.display()),
        }
    }
    out.sort_by(|a, b| (a.0, &a.1).cmp(&(b.0, &b.1)));
    out.into_iter().map(|(_, s, m)| (s, m)).collect()
}

/// The engine keys the options page sets, with shape C's values as their
/// defaults (`shape_keys("c")`, the default mould).
const ENGINE_OPTS: [&str; 5] = ["panic", "wither", "energy_flow", "condense", "roam"];

/// The options that change the world itself: when one is turned, the dish
/// behind the menu starts again on the new world (owner, 11-14 05:04Z:
/// "when you switch the map in the menu it should switch in the
/// background"). An option that picks a new kind of world (a game mode)
/// goes here too; the rest wait for the round.
const DISH_OPTS: [&str; 2] = ["map", "own"];

/// The dish options' values, to tell when one moved.
fn dish_key(m: &StartMenu) -> Vec<String> {
    DISH_OPTS.iter().filter_map(|k| m.option(k).map(|o| o.value().to_string())).collect()
}

/// The value a key last took in `keys` (the last wins, as `gpu::set` plays them).
fn key_value<'a>(keys: &'a [(String, String)], k: &str) -> Option<&'a str> {
    keys.iter().rev().find(|(n, _)| n == k).map(|(_, v)| v.as_str())
}

/// The options page for the command line's `base` (its values preset).
pub(super) fn options(base: &Args) -> Vec<Opt> {
    let (_, c) = shape_keys(DEFAULT_SHAPE).expect("the default shape");
    let c_default = |k: &str| c.iter().find(|(n, _)| *n == k).map(|(_, v)| v.to_string()).unwrap_or_else(|| "0".into());
    let map_now = base.map.clone().unwrap_or_else(|| "noise".into());
    let maps: Vec<&str> = MAPS.iter().copied().chain(["noise"]).collect();
    // The GPU world casts no germ yet (H skips those loadouts too); gas
    // came with #211 (`GpuWorld::has_gas`).
    let loadouts: Vec<&str> = LOADOUTS.iter().filter(|l| !l.1.contains("germ")).map(|l| l.0).collect();
    let bullet = Rules::default().bullet_speed;
    let num = |v: f32| format!("{v}");
    let mut o = vec![
        Opt::new("map", "Sandbox map", &maps, "fork", &map_now, "The sandbox's map (noise: the censuses' race world)"),
        Opt::new("planet", "Planet", &["1", "2", "3", "4", "5", "6", "7", "8"], "1", &planet_now(base), "Which planet of a run Random planet lands on: its biome and nests (8 is the core world)"),
        Opt::new("own", "Your own mould", &["off", "on"], "off", if base.own { "on" } else { "off" }, "A green mould the squad sows and feeds; sandbox on the race world"),
        Opt::new("mode", "Control", &["rts", "squads"], "rts", if base.squads { "squads" } else { "rts" }, "Squads: steer one squad with WASD, fire at the cursor, split and merge; no larder (guard or drain crystals)"),
        Opt::new("rounds", "Rounds", &loadouts, LOADOUTS[0].0, LOADOUTS[base.round].0, "The squad's loadout at the start (H cycles it in play)"),
        Opt::new("bullet", "Bullet speed", &["0", "30", "45", "60", "90"], &num(bullet), &base.bullet.map_or(num(bullet), num), "Dish cells a game second a shot flies; 0 is the instant ray"),
    ];
    let notes = [
        ("panic", "Panic", &["0", "1", "2", "4"][..], "A cut-off part charges the squad or bolts for food; 0 is off"),
        ("wither", "Wither", &["0", "0.005", "0.01", "0.02"][..], "How fast a part cut off from food thins; 0 is off"),
        ("energy_flow", "Shared food", &["0", "0.025", "0.05", "0.1"][..], "How much food the body shares from its crystals; 0 is off"),
        // Dense slime condenses (11-14, `BigParams::condense`): off by
        // default for the owner's playtest; the steps a crowded or feeding
        // walker takes to condense (300 is 10 game seconds).
        ("condense", "Condense", &["0", "150", "300", "600"][..], "Dense or long-fed slime absorbs half of itself; the rest is twice as tough (bronze); 0 is off"),
        // Condensed slime breaks away (11-14, `BigParams::roam`): off by
        // default; how often (steps) a condensed piece may leave for an
        // untouched crystal. Needs Condense.
        ("roam", "Breakaway", &["0", "150", "300", "600"][..], "Condensed pieces leave together for an untouched crystal far away; needs Condense; 0 is off"),
        // Plants (11-14, `mould::plants`): on by default; a stand-in
        // field on maps without their own. Fire from blasts and scorch
        // rounds burns through them.
        ("plants", "Plants", &["0", "1"][..], "Plants grow on the map and burn: blasts and scorch rounds light them, the slime's body stops fire; 0 is off"),
    ];
    for (k, label, choices, note) in notes {
        // Plants are an engine setting (`Settings::plants`, on), not a
        // shape key, so the shape's fallback "0" would show them off.
        let d = if k == "plants" { "1".to_string() } else { c_default(k) };
        let now = key_value(&base.keys, k).map_or(d.clone(), str::to_string);
        o.push(Opt::new(k, label, choices, &d, &now, note));
    }
    // The run's dev mode (owner, 11-14 15:33Z): the shop's rows to jump
    // planets, add points and redraw cards. A run's setting, not a round's.
    o.push(Opt::new("dev", "Dev mode", &["off", "on"], "off", if base.dev { "on" } else { "off" }, "The run's shop gets rows to jump planets, add points and redraw the cards"));
    // The UI's size (owner, 11-14 08:07Z): turned here it takes effect at
    // once and is saved; not a round's setting, so `round_args` skips it.
    let sizes: Vec<String> = settings::STEPS.iter().map(|&v| settings::percent(v)).collect();
    let sizes: Vec<&str> = sizes.iter().map(String::as_str).collect();
    o.push(Opt::new("ui", "UI and text size", &sizes, "100%", &settings::percent(ui_user()), "Every panel, card, tooltip and this menu, over the screen's own scale; Ctrl + and Ctrl - anywhere"));
    o
}

/// The key card's rows (the start menu's Keys page; in play F1 shows the
/// HUD's own, built from the command card): the RTS round's, then the squad
/// mode's (`squadmode::squad_help`), whose card letters move off WASD; the
/// squad mode's first when the command line chose it.
pub(super) fn key_card(squads: bool) -> Vec<(String, String)> {
    let rts: Vec<(String, String)> = [
        ("Left click / drag", "Select soldiers"),
        ("Right click", "Move; on a crystal, harvest it"),
        ("Shift + order", "Queue it after the current orders"),
        ("Ctrl + 1-9, 1-9", "Make a group, select it (twice: look)"),
        ("Wheel, middle drag", "Zoom, pan; arrows and the screen edge pan"),
        ("A, click", "Attack-move; on the core, attack it"),
        ("S", "Stop"),
        ("G", "Harvest"),
        ("F, drag", "Burn a line"),
        ("Z / X / C", "Hold fire, defend, free fire"),
        ("B", "Buy a soldier (or select the larder)"),
        ("V, drag", "Lay a wall"),
        ("R", "The larder's rally point"),
        ("Q", "Build page: turret, lure, pump, outpost, siren"),
        ("E", "Tools page: blast, det cord, mine"),
        ("H", "Next round loadout"),
        ("Shift+H", "Roll a test gun for the squad (rifle, carbine, long gun, scattergun in turn)"),
        ("Ctrl+H", "Roll test bullets for the squad, as many as its gun holds"),
        ("P", "Pause"),
        ("Tab", "Four times faster, or back"),
        ("N", "Next map"),
        ("M", "Minimap"),
        ("O / K", "Sound; the mould's voice"),
        ("Space", "Look at the newest alert"),
        ("Esc", "Cancel placing, or the menu"),
        ("F1 or ?", "The key card"),
        ("Ctrl + / Ctrl -", "Bigger or smaller UI and text (saved)"),
    ]
    .iter()
    .map(|&(k, v)| (k.to_string(), v.to_string()))
    .collect();
    let mut squad = vec![("Squad mode".to_string(), "Options: Control, squads (or mode=squads)".to_string())];
    squad.extend(squad_help());
    // The card's letters as `squad_card_key` moves them.
    let k = |id: u32, key: KeyCode| squad_card_key(id, key).and_then(slime_mould::world::ui::key_name).unwrap_or_default();
    squad.push((format!("{}, {}, {}", k(ATTACK, KeyCode::KeyA), k(HARVEST, KeyCode::KeyG), k(BURN, KeyCode::KeyF)), "The card for the squad: attack-move, drain, burn".into()));
    squad.push((format!("{}, {} (tools page)", k(T_MINE, KeyCode::KeyD), k(T_GAS, KeyCode::KeyS)), "Mine, sleep gas (A, S and D steer)".into()));
    if squads {
        squad.push(("RTS round".into(), "Options: Control, rts".into()));
        squad.into_iter().chain(rts).collect()
    } else {
        rts.into_iter().chain(squad).collect()
    }
}

/// The menu for the command line's `base`.
pub(super) fn start_menu(base: &Args) -> StartMenu {
    let entries = missions()
        .into_iter()
        .map(|(stem, m)| Entry { stem, detail: format!("{}{}", title_case(&m.map), m.tier.as_ref().map_or(String::new(), |t| format!(", {}", title_case(t)))), name: m.name, says: m.says })
        .collect();
    let mut menu = StartMenu::new("Larder World", entries, options(base), key_card(base.squads));
    menu.edit_maps = true;
    menu.planets = true;
    menu
}

/// The round a run's level plays: the options over the command line, as a
/// mission takes them (the tier's own mould, not shape C; the engine keys
/// only where moved off their default), the control mode the options
/// pick, and the level's seed and walkers.
pub(super) fn run_args(base: &Args, menu: &StartMenu, r: &Run) -> Args {
    let mut a = round_args(base, true, menu, &StartEvent::Sandbox);
    let level = r.level();
    a.mission = None;
    a.map = None;
    a.own = false;
    a.seed = level.seed;
    a.walkers = ((base.walkers as f32) * level.walkers).round() as usize;
    a.run = Some(r.clone());
    a
}

/// The shop page's contents for run `r`.
pub(super) fn shop_of(r: &Run) -> Shop {
    let l = r.level();
    let p = r.planet();
    let mut lines = vec![format!("{}. Next: {}, a {} planet; the slime is {}, {:.1} times the first planet's walkers", r.line(), p.name, p.biome.name(), title_case(l.tier.name()), l.walkers)];
    if r.depth == run::PLANETS {
        lines.push("The slime's home world: beat it and the run is won".into());
    }
    // The cards' words and colours as the HUD's loadout strip shows them,
    // with the run's own rules.
    let mut rules = Rules::default();
    r.rules(&mut rules);
    let rr = RoundRules { budget: r.budget(), ..RoundRules::default() };
    let gun = r.carried();
    let rows = r
        .shop()
        .into_iter()
        .map(|row| {
            let (group, area) = match row.group {
                Group::Squad | Group::Cards if matches!(row.face, Face::Item(_)) => ("Squad".to_string(), ShopArea::Upgrades),
                Group::Build | Group::Tools | Group::Dev => (row.group.name().to_string(), ShopArea::Folder),
                Group::Guns => ("Guns".to_string(), ShopArea::Guns),
                _ => (if row.price.is_empty() { "In the bag" } else { "For sale" }.to_string(), ShopArea::Cards),
            };
            let mut s = ShopRow::new(&group, area, Icon::Upgrade, &row.name);
            (s.about, s.price, s.state, s.ready, s.on) = (row.about.clone(), row.price, row.state, row.ready, row.on);
            match row.face {
                Face::Item(key) => {
                    s.icon = item_icon(key);
                    let max = run::ITEMS.iter().find(|i| i.key == key).map_or(1, |i| i.max);
                    s.level = (r.level_of(key), max);
                }
                Face::Card(c) => {
                    let (icon, kind, words) = draw::card_face(c, &rr, &rules);
                    s.icon = icon;
                    s.tint = kind.colour(draw::payload_colour(c));
                    s.cost = c.cost();
                    s.about = format!("{}, budget {}. {}", kind.label(), c.cost(), row.about.replacen(c.about(), &words, 1));
                    if let rounds::Card::Ammo(a) = c {
                        // A rolled bullet: its set as a badge, its numbers.
                        s.badge = Some(draw::ammo_icon(a.set));
                        s.about = format!("{}: {}. {}", a.title(), a.line(), row.about.replacen(c.about(), &words, 1));
                    }
                }
                Face::Gun(g) => {
                    s.icon = draw::gun_icon(g.kind);
                    // No gun is labelled lucky (owner 18:18Z).
                    s.tint = theme::TEXT;
                    s.bars = gun_bars(&g);
                    s.about = format!("{}. {}", g.kind.about(), row.about);
                }
                Face::Dev => s.icon = Icon::Fast,
            }
            s
        })
        .collect();
    Shop {
        heading: format!("{} next: {}", l.mission.name, l.mission.says),
        lines,
        purse: format!("{} points", r.points),
        rows,
        slots: gun.slots,
        budget: (slime_mould::mould::rounds::cost(&r.gun()), gun.budget),
        folder_icons: vec![(Group::Build.name().into(), Icon::Turret), (Group::Tools.name().into(), Icon::Grenade), (Group::Dev.name().into(), Icon::Fast)],
    }
}

/// An upgrade's icon: the command card's own for what it unlocks.
fn item_icon(key: &str) -> Icon {
    match key {
        "hp" => Icon::Heart,
        "strike" => Icon::Attack,
        "rate" => Icon::Fast,
        "range" => Icon::Eye,
        "squad" => Icon::Recruit,
        "bank" => Icon::Crystal,
        "harvest" => Icon::Upgrade,
        "walls" => Icon::Wall,
        "turret" => Icon::Turret,
        "lure" => Icon::Bait,
        "pump" => Icon::Crystal,
        "outpost" => Icon::Hold,
        "siren" => Icon::Alert,
        "blast" => Icon::Grenade,
        "mine" => Icon::Target,
        "cord" => Icon::Burn,
        "gas" => Icon::Pause,
        "draw" => Icon::Menu,
        _ => Icon::Upgrade,
    }
}

/// A gun's numbers as the shop's bars, each against about the best a run
/// finds.
fn gun_bars(g: &slime_mould::mould::guns::Gun) -> Vec<(String, f32, String)> {
    let mut b = vec![
        ("Range".to_string(), g.range / 48.0, format!("{:.0}", g.range)),
        ("Rate".to_string(), g.rate / 3.0, format!("{:.1}/s", g.rate)),
        ("Aim".to_string(), 1.0 - g.spread / 8.0, format!("{:.1}", g.spread)),
    ];
    if g.shots > 1 {
        b.push(("Shots".to_string(), g.shots as f32 / 6.0, format!("{}", g.shots)));
    }
    b.push(("Budget".to_string(), g.budget as f32 / 30.0, format!("{}", g.budget)));
    b.push(("Slots".to_string(), g.slots as f32 / 6.0, format!("{}", g.slots)));
    b
}

/// A page by its `menu=` name.
pub(super) fn page_named(name: &str) -> Option<Page> {
    Some(match name {
        "1" | "main" => Page::Main,
        "campaign" | "missions" => Page::Missions,
        "options" => Page::Options,
        "keys" => Page::Keys,
        "shop" | "shop-mid" | "shop-build" | "shop-tools" => Page::Shop,
        _ => return None,
    })
}

/// The round the menu's pick plays: the command line's `base` with the
/// options over it, and shape C unless the command line chose a mould
/// (`chosen`) or the round is a mission or the own mould's garden (which
/// keep their own, as `main` does).
pub(super) fn round_args(base: &Args, chosen: bool, menu: &StartMenu, pick: &StartEvent) -> Args {
    let mut a = base.clone();
    let val = |k: &str| menu.option(k).map(|o| o.value().to_string());
    // The engine keys the page owns: its value replaces the command line's.
    a.keys.retain(|(k, _)| !ENGINE_OPTS.contains(&k.as_str()));
    // Not a run's level unless `run_args` makes it one.
    a.run = None;
    match pick {
        StartEvent::Mission(i) => {
            a.mission = menu.missions.get(*i).map(|e| e.stem.clone());
            a.own = false;
            // The missions are written for the larder.
            a.squads = false;
        }
        // A generated planet from `seed=` (`menu_event` gives the round a
        // fresh seed), at the Planet option's depth.
        StartEvent::RandomPlanet => {
            a.mission = None;
            a.map = Some(format!("planet:{}", val("planet").unwrap_or_else(|| "1".into())));
            a.own = false;
            a.squads = val("mode").as_deref() == Some("squads");
        }
        _ => {
            a.mission = None;
            a.map = val("map").filter(|m| m != "noise");
            a.own = val("own").as_deref() == Some("on");
            a.squads = val("mode").as_deref() == Some("squads");
            if a.own {
                a.map = None; // the garden plays on the race world
            }
        }
    }
    if let Some(r) = val("rounds").and_then(|n| LOADOUTS.iter().position(|l| l.0 == n)) {
        a.round = r;
    }
    a.bullet = match menu.option("bullet") {
        Some(o) if !o.is_default() => o.value().parse().ok(),
        _ => base.bullet,
    };
    if !chosen && a.mission.is_none() && !a.own {
        default_shape(&mut a);
    }
    // On a mission or the garden (their own mould) only a value moved off
    // its default is set; on shape C the default is C's own anyway.
    for k in ENGINE_OPTS {
        if let Some(v) = menu.changed(k) {
            a.keys.push((k.to_string(), v.to_string()));
        }
    }
    a
}

impl App {
    /// The start menu is up over a live round.
    pub(super) fn in_menu(&self) -> bool {
        self.menu.is_some()
    }

    /// Opens the start menu over a fresh sandbox round (the one its
    /// Sandbox would start with today's options), the camera on the whole
    /// map.
    pub(super) fn open_menu(&mut self) -> Result<(), String> {
        let Some((base, chosen)) = self.base.clone() else { return Ok(()) };
        let menu = self.menu.take().unwrap_or_else(|| start_menu(&base));
        self.args = round_args(&base, chosen, &menu, &StartEvent::Sandbox);
        self.name = self.args.map.clone().unwrap_or_else(|| "noise".into());
        self.menu = Some(menu);
        self.paused = false;
        self.start()?;
        self.menu_camera();
        Ok(())
    }

    /// The whole map, under no HUD.
    pub(super) fn menu_camera(&mut self) {
        self.camera.set_inset(0.0, 0.0);
        let z = self.camera.fit();
        self.camera.zoom_to(z, self.camera.to_screen(self.camera.centre()));
        self.camera.centre_on((self.camera.world.0 / 2.0, self.camera.world.1 / 2.0));
    }

    /// A key or a click on the menu (`input`): what it picked, after the
    /// dish behind is started again if it turned a world option (`DISH_OPTS`).
    pub(super) fn menu_input(&mut self, input: impl FnOnce(&mut StartMenu) -> Vec<StartEvent>) -> Result<Vec<StartEvent>, String> {
        let Some(m) = self.menu.as_mut() else { return Ok(Vec::new()) };
        let before = dish_key(m);
        let picks = input(m);
        // The UI size option, applied the moment it turns.
        if let Some(v) = m.option("ui").and_then(|o| settings::parse_size(o.value())).filter(|&v| v != ui_user()) {
            self.set_ui(v);
        }
        let Some(m) = self.menu.as_mut() else { return Ok(picks) };
        if picks.is_empty() && dish_key(m) != before {
            self.open_menu()?;
        }
        Ok(picks)
    }

    pub(super) fn menu_events(&mut self, picks: Vec<StartEvent>, event_loop: &ActiveEventLoop) {
        for p in picks {
            match self.menu_event(p) {
                Ok(true) => {
                    self.save_replay();
                    event_loop.exit();
                }
                Ok(false) => {}
                Err(e) => return self.fail(event_loop, e),
            }
        }
    }

    /// What a pick on the menu does; true to quit.
    pub(super) fn menu_event(&mut self, ev: StartEvent) -> Result<bool, String> {
        match ev {
            StartEvent::Quit => return Ok(true),
            StartEvent::Run if self.run.is_some() => self.show_shop(),
            StartEvent::Run | StartEvent::NextLevel => {
                let Some((base, _)) = self.base.clone() else { return Ok(false) };
                let mut r = self.run.take().unwrap_or_else(|| {
                    // A new run on a new seed each time (the clock), so two
                    // runs differ; `run=SEED` on the command line repeats one.
                    let seed = std::time::SystemTime::now().duration_since(std::time::UNIX_EPOCH).map_or(1, |d| d.as_secs());
                    Run::new(seed)
                });
                let Some(menu) = self.menu.take() else { return Ok(false) };
                r.dev = menu.option("dev").is_some_and(|o| o.value() == "on");
                self.args = run_args(&base, &menu, &r);
                self.run = Some(r);
                self.round = self.args.round;
                self.parked = Some(menu);
                self.start()?;
                self.paused = false;
            }
            StartEvent::Buy(i) => {
                if let Some(r) = self.run.as_mut() {
                    if let Some(row) = r.shop().get(i) {
                        let p: Pick = row.pick;
                        if !r.pick(p) {
                            if let Some(m) = self.menu.as_mut() {
                                m.notice = Some(format!("{}: {}", row.name, if row.ready { "can't be done" } else { "not enough points, or locked" }));
                            }
                        }
                    }
                }
                self.show_shop();
            }
            // A bullet card dragged (or Q/E) to another slot of the gun;
            // one for sale is bought on the way.
            StartEvent::Move(i, slot) => {
                if let Some(r) = self.run.as_mut() {
                    if let Some(row) = r.shop().get(i) {
                        let done = match row.pick {
                            Pick::Card(ci) => r.pick(Pick::Move(ci, slot)),
                            Pick::Offer(k) => r.buy_into(k, slot),
                            _ => true,
                        };
                        if !done {
                            if let Some(m) = self.menu.as_mut() {
                                m.notice = Some(format!("{}: over the gun's budget or slots, or not enough points", row.name));
                            }
                        }
                    }
                }
                self.show_shop();
            }
            StartEvent::EndRun => {
                let line = self.run.take().map(|r| format!("Run given up on planet {}: {}", r.depth, r.line()));
                if let Some(m) = self.menu.as_mut() {
                    m.run_live = false;
                    m.shop = None;
                    m.notice = line;
                    m.show(Page::Main);
                }
            }
            StartEvent::EditMaps => {
                let note = match launch_editor() {
                    Ok(()) => "The map editor opened in its own window".to_string(),
                    Err(e) => e,
                };
                if let Some(m) = self.menu.as_mut() {
                    m.notice = Some(note);
                }
            }
            pick => {
                let (Some((base, chosen)), Some(menu)) = (self.base.clone(), self.menu.take()) else { return Ok(false) };
                self.args = round_args(&base, chosen, &menu, &pick);
                if pick == StartEvent::RandomPlanet {
                    self.args.seed = fresh_seed();
                }
                self.round = self.args.round;
                self.name = self.args.map.clone().unwrap_or_else(|| "noise".into());
                // Kept for Main menu, on the page it was left on.
                self.parked = Some(menu);
                self.start()?;
                self.paused = false;
            }
        }
        Ok(false)
    }

    /// The shop page, filled from the run (the menu up, its dish behind).
    pub(super) fn show_shop(&mut self) {
        let Some(r) = self.run.as_mut() else { return };
        if let Some(m) = self.menu.as_mut() {
            r.dev = m.option("dev").is_some_and(|o| o.value() == "on");
            m.shop = Some(shop_of(r));
            m.run_live = true;
            if m.page != Page::Shop {
                m.show(Page::Shop);
            }
        }
    }

    /// The end screen's carry-on in a run: the level's verdict goes to the
    /// run (a win's points and the next planet; a loss ends it), then the
    /// shop, or the main menu with the run's line when it is over. With
    /// no menu (`menu=0`) the next planet starts at once.
    pub(super) fn carry_on(&mut self) -> Result<(), String> {
        let Some(v) = self.mission.as_ref().and_then(|t| t.verdict) else { return Ok(()) };
        let Some(mut r) = self.args.run.clone() else { return Ok(()) };
        if v.stars > 0 {
            r.won(v.stars);
        } else {
            r.lost();
        }
        if self.base.is_none() {
            if r.over {
                println!("larder-world: the run is over: {}", r.line());
                self.args.run = None;
                self.run = None;
                return Ok(());
            }
            self.args.seed = r.level().seed;
            self.args.run = Some(r.clone());
            self.run = Some(r);
            return self.start();
        }
        let over = r.over.then(|| if r.victory() { format!("The home world is free: the run is won. {}", r.line()) } else { format!("The run ended on planet {}: {}", r.depth, r.line()) });
        self.run = if r.over { None } else { Some(r) };
        self.back_to_menu()?;
        if let Some(m) = self.menu.as_mut() {
            m.run_live = self.run.is_some();
            m.notice = over;
        }
        if self.run.is_some() {
            self.show_shop();
        }
        Ok(())
    }

    /// Back to the start menu from play (the Esc menu's or the end
    /// screen's Main menu).
    pub(super) fn back_to_menu(&mut self) -> Result<(), String> {
        self.save_replay();
        self.menu = self.parked.take();
        if let Some(m) = self.menu.as_mut() {
            m.show(Page::Main);
            m.run_live = self.run.is_some();
            m.shop = None;
        }
        self.open_menu()
    }
}

/// The Planet option's value for the command line's map: its depth when
/// the map is a planet, else the first.
fn planet_now(base: &Args) -> String {
    base.map.as_deref().and_then(|m| slime_mould::mould::levelgen::spec_planet(m, base.seed)).and_then(Result::ok).map_or("1".into(), |p| p.depth.min(8).to_string())
}

/// A seed for a new planet: the clock's, kept to six digits so the title's
/// "seed 482913" can be typed back as `seed=` to play the planet again.
pub(super) fn fresh_seed() -> u64 {
    let t = std::time::SystemTime::now().duration_since(std::time::UNIX_EPOCH).map_or(1, |d| d.as_nanos() as u64);
    t % 1_000_000
}

/// Starts the map editor (`larder-edit`, #209) beside this binary.
fn launch_editor() -> Result<(), String> {
    let me = std::env::current_exe().map_err(|e| e.to_string())?;
    let exe = me.with_file_name(format!("larder-edit{}", std::env::consts::EXE_SUFFIX));
    if !exe.exists() {
        return Err("No map editor built: cargo build --release --bin larder-edit".into());
    }
    std::process::Command::new(&exe).current_dir(env!("CARGO_MANIFEST_DIR")).spawn().map(|_| ()).map_err(|e| format!("The map editor did not start: {e}"))
}
