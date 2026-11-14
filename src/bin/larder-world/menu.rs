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
use slime_mould::world::start::{Entry, Opt, Page, StartEvent, StartMenu};

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
const ENGINE_OPTS: [&str; 3] = ["panic", "wither", "energy_flow"];

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
        Opt::new("own", "Your own mould", &["off", "on"], "off", if base.own { "on" } else { "off" }, "A green mould the squad sows and feeds; sandbox on the race world"),
        Opt::new("rounds", "Rounds", &loadouts, LOADOUTS[0].0, LOADOUTS[base.round].0, "The squad's loadout at the start (H cycles it in play)"),
        Opt::new("bullet", "Bullet speed", &["0", "30", "45", "60", "90"], &num(bullet), &base.bullet.map_or(num(bullet), num), "Dish cells a game second a shot flies; 0 is the instant ray"),
    ];
    let notes = [
        ("panic", "Panic", &["0", "1", "2", "4"][..], "A cut-off part charges the squad or bolts for food; 0 is off"),
        ("wither", "Wither", &["0", "0.005", "0.01", "0.02"][..], "How fast a part cut off from food thins; 0 is off"),
        ("energy_flow", "Shared food", &["0", "0.025", "0.05", "0.1"][..], "How much food the body shares from its crystals; 0 is off"),
    ];
    for (k, label, choices, note) in notes {
        let d = c_default(k);
        let now = key_value(&base.keys, k).map_or(d.clone(), str::to_string);
        o.push(Opt::new(k, label, choices, &d, &now, note));
    }
    o
}

/// The key card's rows (the start menu's Keys page; in play F1 shows the
/// HUD's own, built from the command card).
pub(super) fn key_card() -> Vec<(String, String)> {
    [
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
        ("Q", "Build page: turret, lure, pump, outpost, siren"),
        ("E", "Tools page: blast, det cord, mine"),
        ("H", "Next round loadout"),
        ("P", "Pause"),
        ("Tab", "Four times faster, or back"),
        ("N", "Next map"),
        ("M", "Minimap"),
        ("O / K", "Sound; the mould's voice"),
        ("Space", "Look at the newest alert"),
        ("Esc", "Cancel placing, or the menu"),
        ("F1 or ?", "The key card"),
    ]
    .iter()
    .map(|&(k, v)| (k.to_string(), v.to_string()))
    .collect()
}

/// The menu for the command line's `base`.
pub(super) fn start_menu(base: &Args) -> StartMenu {
    let entries = missions()
        .into_iter()
        .map(|(stem, m)| Entry { stem, detail: format!("{}{}", title_case(&m.map), m.tier.as_ref().map_or(String::new(), |t| format!(", {}", title_case(t)))), name: m.name, says: m.says })
        .collect();
    let mut menu = StartMenu::new("Larder World", entries, options(base), key_card());
    menu.edit_maps = true;
    menu
}

/// A page by its `menu=` name.
pub(super) fn page_named(name: &str) -> Option<Page> {
    Some(match name {
        "1" | "main" => Page::Main,
        "campaign" | "missions" => Page::Missions,
        "options" => Page::Options,
        "keys" => Page::Keys,
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
    match pick {
        StartEvent::Mission(i) => {
            a.mission = menu.missions.get(*i).map(|e| e.stem.clone());
            a.own = false;
        }
        _ => {
            a.mission = None;
            a.map = val("map").filter(|m| m != "noise");
            a.own = val("own").as_deref() == Some("on");
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

    /// Back to the start menu from play (the Esc menu's or the end
    /// screen's Main menu).
    pub(super) fn back_to_menu(&mut self) -> Result<(), String> {
        self.save_replay();
        self.menu = self.parked.take();
        if let Some(m) = self.menu.as_mut() {
            m.show(Page::Main);
        }
        self.open_menu()
    }
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
