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
//! amber), and `menu=PAGE` (`main`, `campaign`, `options`, `keys`, `news`)
//! with `look=DIR` films that page over the round (the pictures on the
//! page).
//!
//! It also says what the build holds (`build_page`, from the Engine and
//! tools lane's `slime_mould::build`): the build in the main page's
//! corner, what is new with the lines new to this PC marked, and the
//! things to try with their ticks.

use super::*;
use slime_mould::mould::run::{self as run, Pick};
use slime_mould::mould::run::{Face, Group};
use slime_mould::mould::dps::Profile;
use slime_mould::mould::guns;
use slime_mould::mould::bullets::{Set, Shell};
use slime_mould::build::news::{self, Memory};
use slime_mould::world::start::{BuildPage, Entry, Glance, NewsLine, Opt, Page, Shop, ShopArea, ShopRow, ShotTag, StartEvent, StartMenu, TryLine, BAG};

/// The campaign's missions, as `mission=STEM` names them, in their order
/// (the number before the dash). Files that do not parse are skipped, with
/// a word on stderr. (The campaign thread's `Mission::list` will do this
/// in the library; switch to it when it lands.)
pub(super) fn missions() -> Vec<(String, Mission)> {
    let dir = slime_mould::build::root().join("Reports/larder-campaign-2026-11-12/missions");
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
const ENGINE_OPTS: [&str; 6] = ["panic", "wither", "energy_flow", "roam", "surge", "flinch"];

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
    // Every loadout: the GPU world has cast the germ since 11-14
    // (`GpuWorld::has_germ`), and gas since #211. Until 2026-11-16 the
    // germ's were left out ("the GPU world casts no germ yet").
    let loadouts: Vec<&str> = LOADOUTS.iter().map(|l| l.0).collect();
    let bullet = Rules::default().bullet_speed;
    let num = |v: f32| format!("{v}");
    let mut o = vec![
        Opt::new("map", "Sandbox map", &maps, "fork", &map_now, "The sandbox's map (noise: a made-up race world)"),
        Opt::new("planet", "Planet", &["1", "2", "3", "4", "5", "6", "7", "8"], "1", &planet_now(base), "Which planet of a run Random planet lands on: its biome and nests (8 is the core world)"),
        Opt::new("own", "Your own slime", &["off", "on"], "off", if base.own { "on" } else { "off" }, "A green slime the squad sows and feeds; sandbox on the race world"),
        Opt::new("mode", "Control", &["squads", "rts"], "squads", if base.squads { "squads" } else { "rts" }, "Squads: steer one squad with WASD, fire at the cursor, split and merge; guard or drain crystals"),
        Opt::new("rounds", "Rounds", &loadouts, LOADOUTS[0].0, LOADOUTS[base.round].0, "The squad's loadout at the start (H cycles it in play)"),
        Opt::new("bullet", "Bullet speed", &["0", "30", "45", "60", "90"], &num(bullet), &base.bullet.map_or(num(bullet), num), "Cells a game second a shot flies; 0 is the instant ray"),
    ];
    let notes = [
        ("panic", "Panic", &["0", "1", "2", "4"][..], "A cut-off part charges the squad or bolts for food; 0 is off"),
        ("wither", "Wither", &["0", "0.005", "0.01", "0.02"][..], "How fast a part cut off from food thins; 0 is off"),
        ("energy_flow", "Shared food", &["0", "0.025", "0.05", "0.1"][..], "How much food the body shares from its crystals; 0 is off"),
        // Condense (11-14) and Thicken (11-15) left the menu on 11-16: the
        // owner, 04:58Z, "Remove the bronze, keep the specks"; both rules
        // were deleted 11-17.
        // Slime breaks away (11-14, `BigParams::roam`; from ordinary slime
        // since 11-16): how often (steps) a ripe walker may leave for an
        // untouched crystal far off. Off in the sandbox; the run sets it
        // per planet (`run::enemy_keys`).
        ("roam", "Breakaway", &["0", "150", "300", "600"][..], "Slime crowded or feeding at a crystal sends specks off to a fresh crystal farther away; steps between chances (300 is 10 s); 0 is off"),
        // The body surges (11-15, `BigParams::surge_every`): steps between
        // surges (30 a second); on in shape C at 900 (30 s).
        ("surge", "Surge", &["0", "900", "1800", "3600"][..], "When the slime has eaten enough from its crystals, the slime near your soldiers and buildings creeps toward them, then lunges; the longest wait between surges in steps (1800 is 60 s); 0 is off"),
        // The body flinches from fire (11-15, `BigParams::flinch`): on in
        // shape C at 0.9, the alarm field's keep a fine step; a slime with
        // an alarm of its own (the Swarmer's) keeps it.
        ("flinch", "Flinch", &["0", "0.8", "0.9", "0.95"][..], "Slime a shot lands near backs off for a moment, so a ring opens round soldiers who keep firing; higher lasts longer; 0 is off"),
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
    // The dev skip (owner, 11-22 21:36Z): a new run starts in the shop
    // before this planet, each planet before it skipped for its average
    // pay (`run::skip_pay`). The shop's Dev tab skips one more at a time.
    let starts: Vec<String> = (1..=run::PLANETS).map(|d| d.to_string()).collect();
    let starts: Vec<&str> = starts.iter().map(String::as_str).collect();
    o.push(Opt::new("start", "Start on planet", &starts, "1", "1", "Dev: a new run skips the planets before this one, pays what an average win on each pays, and opens the shop; the shop's Dev tab skips one planet at a time"));
    // The UI's size (owner, 11-14 08:07Z): turned here it takes effect at
    // once and is saved; not a round's setting, so `round_args` skips it.
    let sizes: Vec<String> = settings::STEPS.iter().map(|&v| settings::percent(v)).collect();
    let sizes: Vec<&str> = sizes.iter().map(String::as_str).collect();
    // First on the page (the review, 11-15: it was the last of 15 rows,
    // below the fold, and the one the owner needed).
    // The declared switches (`build::switches`; the Engine and tools
    // lane, 11-17), one row each on their own tab: `experiments.rs` says
    // what is on it, this file and `start.rs` how it is drawn.
    o.extend(experiments::options(base));
    // The volume (sound pass two, 2026-11-20), second: taken at once and
    // saved, like the UI size; not a round's setting.
    let vols: Vec<String> = settings::VOLUME_STEPS.iter().map(|&v| settings::percent(v)).collect();
    let vols: Vec<&str> = vols.iter().map(String::as_str).collect();
    // The music under it (2026-11-22, `sound::score`): the planet's music
    // bed and its air, on at 100%; 0% leaves only the game's own sounds.
    o.insert(0, Opt::new("music", "Music and air", &vols, "100%", &settings::percent(music_user()), "The planet's music bed and its wind, water and insects, under every other sound; 0% turns them off"));
    o.insert(0, Opt::new("volume", "Volume", &vols, "100%", &settings::percent(volume_user()), "Every sound, over the first-run level; 0% is silent, and O mutes in play"));
    o.insert(0, Opt::new("ui", "UI and text size", &sizes, "100%", &settings::percent(ui_user()), "Every panel, card, tooltip and this menu, over the screen's own scale; Ctrl + and Ctrl - anywhere"));
    // The screen shake under the sound (the owner, 2026-11-22, looked for
    // it here and found none): taken at once and saved, not a round's.
    let shakes: Vec<String> = settings::SHAKE_STEPS.iter().map(|&v| settings::percent(v)).collect();
    let shakes: Vec<&str> = shakes.iter().map(String::as_str).collect();
    // The world's detail under it (performance thread, 2026-11-22): the
    // owner's 4K screen drew the world at 8 times the pixels the GTX 1070
    // was timed at. Taken at once and saved, not a round's. Steps from
    // 100% to 40% since 11-24 (the owner approved a quality slider).
    o.insert(3, Opt::new("detail", "World detail", &settings::DETAIL_STEPS, "Auto", detail_user(), "How many of the screen's pixels the world is drawn at, each way. Auto starts at the screen's own step (half on a 4K screen) and steps down toward 40% only while frames run slower than 60 a second; 100% draws every pixel, lower steps are softer and faster. Text, panels and soldiers stay sharp"));
    // First-sight hints (`hints.rs`, Direction 11-24): one line under each
    // of the slime's later threats the first time it shows on this PC.
    // Taken at once and saved, not a round's.
    o.push(Opt::new("hints", "Hints", &["on", "off"], "on", if super::hints::saved_on() { "on" } else { "off" }, "A line under each new threat the first time you meet it (strikers, waves, mini-bosses, fruiting and more), and how to collect a Starfall"));
    o.insert(3, Opt::new("shake", "Screen shake", &shakes, "100%", &settings::percent(shake_user()), "How hard the camera shakes when a blast lands; 0% keeps it still"));
    // The page's tabs (11-15, the owner's "Having to scroll is horrible"):
    // the game's own settings, the planet's, and the mould's rules, each
    // tab one screen; the key list is the last tab. Tabs come in the
    // order their first setting does.
    let mut o = o
        .into_iter()
        .map(|opt| {
            let tab = match opt.key.as_str() {
                // How the game shows on this screen (performance thread,
                // 11-22): the World detail row did not fit the Game tab
                // at 1280 x 540 and 200%, so the UI size, the world's
                // detail and the shake have a tab of their own, first.
                "ui" | "detail" | "shake" => "Screen",
                "map" | "planet" | "own" | "plants" => "Planet",
                // The slime's rules as raw numbers (Panic 2, Wither 0.01):
                // a tuning tab, last before Keys and named for what it is
                // (polish review finding 11, 11-24: "The mould" third read
                // as the developer's console among the player's settings).
                "panic" | "wither" | "energy_flow" | "roam" | "surge" | "flinch" => TUNING_TAB,
                k if k.starts_with(slime_mould::build::switches::OPT_KEY) => "Experiments",
                _ => "Game",
            };
            opt.in_tab(tab)
        })
        .collect::<Vec<_>>();
    // Tabs come in the order of their first row: the tuning rows last.
    o.sort_by_key(|opt| opt.tab == TUNING_TAB);
    o
}

/// The Options tab of the slime's tuning numbers.
pub(super) const TUNING_TAB: &str = "Slime tuning";

/// The key card's rows (the start menu's Keys page; in play F1 shows the
/// HUD's own, built from the command card). In the squad game, its keys
/// only, under headings (a row with no words, `squadmode::squad_help`);
/// in the RTS round its own rows, then the squad mode's.
pub(super) fn key_card(squads: bool) -> Vec<(String, String)> {
    let rts: Vec<(String, String)> = [
        ("Left click / drag", "Select soldiers"),
        ("Right click", "Move; on a crystal, harvest it"),
        ("Shift + order", "Queue it after the current orders"),
        ("Ctrl + 1-9, 1-9", "Make a group, select it (twice: look)"),
        ("Wheel, middle drag", "Zoom, pan; arrows, the screen edge and WASD while paused pan"),
        ("A, click", "Attack-move; on the core, attack it"),
        ("S", "Stop"),
        ("G", "Harvest"),
        ("F, click", "Guard a spot, building or crystal: keep slime off it"),
        ("Z / X / C", "Hold fire, defend, free fire (the fire button cycles them)"),
        ("B", "Buy a soldier (or select your base)"),
        ("U, click", "Place a pump on a crystal (again on one: upgrade it)"),
        ("R", "Your base's rally point"),
        ("Q", "Build page: turret, pump, wall (V, drag), lure, siren, outpost"),
        ("Click a building", "With no soldier selected: select it (cancel, demolish, upgrade, switch)"),
        ("E", "Tools page: blast, det cord, mine"),
        ("H", "Next round loadout"),
        ("Shift+H", "Roll a test gun for the squad (rifle, carbine, long gun, scattergun in turn)"),
        ("Ctrl+H", "Roll test bullets for the squad, as many as its gun holds"),
        ("H (gun range)", "The armory: every gun, card and bullet, free"),
        ("P", "Pause"),
        ("Tab", "Four times faster, or back"),
        ("N", "Next map"),
        ("M", "Minimap"),
        ("F2", "Scent view: what the slime smells"),
        ("F9", "Drop a note in the play log: marks this moment and place"),
        ("O / K", "Sound; the slime's voice"),
        ("Space", "Look at the newest alert"),
        ("Esc", "Cancel placing, or the menu"),
        ("F1 or ?", "The key card"),
        ("Ctrl + / Ctrl -", "Bigger or smaller UI and text (saved)"),
    ]
    .iter()
    .map(|&(k, v)| (k.to_string(), v.to_string()))
    .collect();
    let k = |id: u32, key: KeyCode| squad_card_key(id, key).and_then(slime_mould::world::ui::key_name).unwrap_or_default();
    if squads {
        // The squad game's keys only, in groups (polish review finding
        // 11, 11-24): the RTS round's rows, its test guns, the next map
        // and the scent view are gone from the card it plays by; they
        // stay on the RTS round's own card below.
        let mut squad = squad_help();
        let at = |rows: &[(String, String)], head: &str| rows.iter().position(|r| r.0 == head && r.1.is_empty()).map_or(rows.len(), |i| i + 1);
        let fight = at(&squad, "Fighting");
        squad.insert(fight + 2, (format!("{}, {}, {}", k(ATTACK, KeyCode::KeyA), k(HARVEST, KeyCode::KeyG), k(GUARD, KeyCode::KeyF)), "On the card: attack-move, drain a crystal, guard".into()));
        let more: Vec<(String, String)> = [
            ("Building and tools", ""),
            ("Q", "Build page: turret, pump, wall (V, drag), door, lure, siren"),
            ("E", "Tools page: blast, det cord, mine, germ, sleep gas"),
            ("H", "Swap to the next loadout"),
            ("The game", ""),
            ("P", "Pause"),
            ("Tab", "Four times faster, or back"),
            ("M", "Minimap"),
            ("O / K", "Sound; the slime's voice"),
            ("F9", "Drop a note in the play log: marks this moment and place"),
            ("Esc", "Cancel placing, or the menu"),
            ("F1 or ?", "The key card"),
            ("Ctrl + / Ctrl -", "Bigger or smaller UI and text (saved)"),
        ]
        .iter()
        .map(|&(a, b)| (a.to_string(), b.to_string()))
        .collect();
        let tools = at(&more, "Building and tools");
        let mut more = more;
        more.insert(tools + 2, (format!("{}, {} (tools page)", k(T_MINE, KeyCode::KeyD), k(T_GAS, KeyCode::KeyS)), "Mine, sleep gas (A, S and D steer)".into()));
        squad.extend(more);
        squad
    } else {
        let mut squad = vec![("Squad mode".to_string(), "Options: Control, squads (or mode=squads)".to_string())];
        squad.extend(squad_help().into_iter().filter(|r| !r.1.is_empty()));
        squad.push((format!("{}, {}, {}", k(ATTACK, KeyCode::KeyA), k(HARVEST, KeyCode::KeyG), k(GUARD, KeyCode::KeyF)), "The card for the squad: attack-move, drain, guard".into()));
        squad.push((format!("{}, {} (tools page)", k(T_MINE, KeyCode::KeyD), k(T_GAS, KeyCode::KeyS)), "Mine, sleep gas (A, S and D steer)".into()));
        rts.into_iter().chain(squad).collect()
    }
}

/// The menu for the command line's `base`.
pub(super) fn start_menu(base: &Args) -> StartMenu {
    let entries = missions()
        .into_iter()
        .map(|(stem, m)| {
            // Its map small in the note (second polish pass, 11-24).
            let dir = slime_mould::build::root().join("Reports/larder-maps-2026-11-11/maps");
            let glance = slime_mould::mould::map::Map::load(&dir, &m.map).ok().map(|mp| glance_from(&mp, 96));
            Entry { stem, detail: format!("{}{}", title_case(&m.map), m.tier.as_ref().map_or(String::new(), |t| format!(", {}", title_case(t)))), name: m.name, says: m.says, glance }
        })
        .collect();
    let mut menu = StartMenu::new(slime_mould::world::GAME_NAME, entries, options(base), key_card(base.squads));
    menu.edit_maps = true;
    menu.planets = true;
    menu.most_off = slime_mould::build::switches::MOST_OFF;
    // What the build holds, as a PC that has not played sees it (every
    // line new, nothing ticked): the window puts its own memory over it
    // when it opens the menu (`App::open_menu`). A film may show a PC
    // that has played by naming its settings file (`LARDER_SETTINGS`),
    // which is read here and never written: nothing in a film plays a
    // round or ticks a row.
    let seen = std::env::var_os("LARDER_SETTINGS").filter(|v| !v.is_empty()).map_or_else(Memory::unsaved, |p| Memory::load_from(std::path::Path::new(&p)));
    menu.build = Some(build_page(&seen));
    menu
}

/// **What the build says of itself**, for the menu (`start::BuildPage`),
/// as `memory` has it: which build this is, the last one played here, the
/// "what is new" list with the lines no round has been played on marked,
/// and the things to try with their ticks.
///
/// The new lines come first, and each part is the newest first by the
/// day a line was written. The file's own order is the order pull
/// requests merged in, which on 11-18 already had three lines of 11-17
/// under five of 11-18.
pub(super) fn build_page(memory: &Memory) -> BuildPage {
    let b = slime_mould::build::build();
    let mut lines: Vec<NewsLine> = news::news().iter().map(|n| NewsLine { date: n.date.into(), lane: n.lane.name().into(), sees: n.sees.into(), try_it: n.try_it.into(), new: memory.is_new(n) }).collect();
    lines.sort_by(|a, b| b.new.cmp(&a.new).then_with(|| b.date.cmp(&a.date)));
    let last = memory.last_build.as_ref().map(|(n, commit, date)| {
        let name = if *n > 0 { format!("build {n}") } else { format!("build {commit}") };
        if date.is_empty() { name } else { format!("{name}, {date}") }
    });
    BuildPage { name: b.short(), long: b.long(), last, news: lines, to_try: news::to_try().iter().map(|t| TryLine { text: t.text.into(), tried: memory.is_tried(t) }).collect() }
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
    // Shape C's walkers by the level (`run_walkers`); its keys go on in
    // `start`, under the tier's (`run_shape`).
    a.walkers = run_walkers(level.walkers);
    a.run = Some(r.clone());
    a
}

/// The shop page's contents for run `r` (the gun range's armory when
/// `r` is the range's).
pub(super) fn shop_of(r: &Run) -> Shop {
    let l = r.level();
    let p = r.planet();
    let mut lines = vec![format!("{}. Next: {}, a {} planet; its slime is {:.1} times the first planet's", r.line(), p.name, p.biome.name(), l.walkers)];
    // What this tier does to you (`run::does_to_you`).
    // What this tier does to you (`run::does_to_you`) is the pane's own
    // block above these lines since the planet's introduction (11-24).
    let does = run::does_to_you(l.tier).says;
    // The level's speed star and head start (`run::par`, `run::head_start`);
    // the clock counts up (owner, 2026-11-20), so there is no time limit to
    // tell, only the star's.
    let par = l.mission.rules(Rules::default()).par;
    let secs = par.round() as u32;
    let mut clock = format!("A star for clearing it under {}:{:02}", secs / 60, secs % 60);
    if l.grow > 0.0 {
        clock += &format!("; the slime spreads for {:.0} s before you land", l.grow);
    }
    // How often it surges here (`run::enemy_keys`; on the clock's line so the
    // heading keeps two lines, the run thread's ask).
    clock += &format!("; it surges as it feeds, at least every {:.0} s", run::surge_secs(l.depth));
    if r.depth == run::PLANETS {
        clock = format!("The slime's home world: beat it and the run is won. {clock}");
    }
    lines.push(clock);
    if r.range {
        lines = vec![
            "Everything is free. Put cards in the gun: the squad fires them left to right, over and over.".into(),
            format!("Guns and bullets are rolled as on planet {}; the buttons along the bottom roll them again, deeper or shallower", r.depth),
        ];
    }
    // The cards' words and colours as the HUD's loadout strip shows them,
    // with the run's own rules.
    let mut rules = Rules::default();
    r.rules(&mut rules);
    let rr = RoundRules { budget: r.budget(), ..RoundRules::default() };
    // The slots and budget of what the gun slots show: the squad's gun,
    // or the turrets' rack (`Run::rack`).
    let gun = r.armed_of(r.editing).unwrap_or_else(|| r.rack());
    let raw = r.shop();
    // The gun's cards in slot order, as the shop's gun row lists them (the
    // rows it marks in the gun), for the fused shots (owner, 11-24).
    let in_gun: Vec<rounds::Card> = raw.iter().filter(|row| row.on).filter_map(|row| if let Face::Card(c) = row.face { Some(c) } else { None }).collect();
    let (shots, plain_shot) = rounds::shots(&in_gun);
    let rows = raw
        .into_iter()
        .map(|row| {
            let after = dps_after(r, row.pick);
            let (group, area) = match row.group {
                // The range: its own rows on the left, the shelf by what a
                // card works on, then the bag.
                Group::Dev if r.range => (String::new(), ShopArea::Upgrades),
                // The gun slots' switch to the turrets' deck: a bottom-bar
                // button (`Pick::EditTurrets`).
                // The loadouts' buttons too (2026-11-19).
                _ if matches!(row.pick, Pick::EditTurrets | Pick::Edit(_) | Pick::NewLoadout | Pick::TurretsUse | Pick::DevicesUse | Pick::SquadUses | Pick::Reroll | Pick::Hold) => (String::new(), ShopArea::Upgrades),
                Group::Cards if r.range => (range_shelf(&row).to_string(), ShopArea::Cards),
                Group::Squad | Group::Cards if matches!(row.face, Face::Item(_)) => ("Squad".to_string(), ShopArea::Upgrades),
                // Each a tab of the shop's counter (Shop B, 11-15).
                Group::Build | Group::Tools | Group::Relics | Group::Dev => (row.group.name().to_string(), ShopArea::Upgrades),
                Group::Guns => ("Guns".to_string(), ShopArea::Guns),
                _ => (if row.price.is_empty() { BAG } else { "Cards" }.to_string(), ShopArea::Cards),
            };
            let mut s = ShopRow::new(&group, area, Icon::Upgrade, &row.name);
            (s.about, s.price, s.state, s.ready, s.on) = (row.about.clone(), row.price, row.state, row.ready, row.on);
            (s.dps, s.no_room) = (after.map(|a| a.0.dps), after.is_some_and(|a| a.1));
            s.stats = after.map_or(Vec::new(), |a| profile_bars(&a.0));
            match row.face {
                Face::Item(key) => {
                    s.icon = item_icon(key);
                    let max = run::ITEMS.iter().find(|i| i.key == key).map_or(1, |i| i.max);
                    s.level = (r.level_of(key), max);
                    s.bars = item_bars(key, s.level.0);
                    // A device body (gun step E3, 11-24): its kind as the
                    // name and its four rolls as bars, as a gun's are
                    // (second polish pass, Direction 08:41Z: the rolls
                    // were packed into a title the card cut off). An
                    // offer of a kind the run owns shows each roll as the
                    // owned one's and then its own.
                    let body = match row.pick {
                        Pick::Body(i) => r.body_offers.get(i).map(|b| (b, r.body_of(b.kind).and_then(|j| r.bodies.get(j)))),
                        Pick::CarryBody(i) => r.bodies.get(i).map(|b| (b, None)),
                        _ => None,
                    };
                    // A body's autocast switch reads as a switch, lit when
                    // on: as its tool's unlock (level 1 of 1) it wore the
                    // owned tick beside "autocast: off" (Look, 11-24).
                    // A relic's line opened "A relic, rare: For the rest of
                    // the run" (Look, 11-24): the tab says Relics, the card
                    // says what it does, rare last.
                    if row.group == Group::Relics {
                        s.about = format!("{} Rare: one relic a shop.", draw::capital(s.about.trim_start_matches("A relic, rare: ")));
                    }
                    if matches!(row.pick, Pick::BodyAuto(_)) {
                        s.level = (0, 0);
                        s.bars = Vec::new();
                    }
                    if let Some((b, mine)) = body {
                        s.name = b.kind.name().to_string();
                        s.level = (0, 0);
                        s.bars = body_bars(b, mine);
                        // ("laid along a drag; ..." followed the title.)
                        s.about = draw::capital(&s.about);
                    }
                }
                Face::Card(c) => {
                    let (icon, kind, words) = draw::card_face(c, &rr, &rules);
                    s.icon = icon;
                    s.tint = kind.colour(draw::payload_colour(c));
                    s.cost = c.cost();
                    // Its kind and words, the run's note after them (too
                    // big for the gun), its budget last (owner 11-15
                    // 02:57Z: the cards' text "should be simplified").
                    s.about = format!("{} Budget {}.", row.about.replacen(c.about(), &draw::card_words(c, kind, &words), 1), c.cost());
                    if let rounds::Card::Ammo(a) = c {
                        // A rolled bullet: its set as a badge, its rolls
                        // as bars (the owner, 2026-11-22: "more of visual
                        // representation of damage speed, range").
                        s.badge = Some(draw::ammo_icon(a.set));
                        s.bars = ammo_bars(&a);
                        // The range's shelf holds one of each shape and
                        // payload: a grid of them, so its tiles need no
                        // names (all three menu researchers, 11-15).
                        if r.range {
                            let set = Set::ALL.iter().position(|&x| x == a.set).unwrap_or(0) as u8;
                            let shell = Shell::ALL.iter().position(|&x| x == a.shell).unwrap_or(0) as u8;
                            let payload = if a.shell == Shell::Plain { "Bullet".to_string() } else { draw::capital(a.shell.key()) };
                            s.cell = Some(((set, draw::capital(a.set.key())), (shell, payload)));
                        }
                    }
                }
                Face::Gun(g) => {
                    s.icon = draw::gun_icon(g.kind);
                    // No gun is labelled lucky (owner 18:18Z).
                    s.tint = theme::TEXT;
                    s.bars = gun_bars(&g);
                    s.about = format!("{} {}", g.kind.about(), row.about);
                }
                Face::Dev => {
                    s.icon = match row.pick {
                        Pick::DevDepth(1) => Icon::Upgrade,
                        Pick::DevDepth(_) => Icon::Hold,
                        Pick::EmptyGun => Icon::Close,
                        Pick::EmptyBag => Icon::Stop,
                        _ => Icon::Fast,
                    }
                }
            }
            s
        })
        .collect();
    Shop {
        heading: if r.range { "The gun range".into() } else { format!("{} next: {}", l.mission.name, l.mission.says) },
        lines,
        purse: if r.range { "Free".into() } else { format!("{} points", r.points) },
        rows,
        slots: gun.slots,
        budget: (slime_mould::mould::rounds::cost(&r.gun()), gun.budget),
        range: r.range,
        turrets: r.editing_turrets(),
        bench: {
            let gun = r.gun_of(r.editing).map_or_else(|| "no gun: the turrets' rack, a squad fires the plain rifle".to_string(), |g| draw::capital(&g.title()));
            format!("{}  ·  {gun}", r.loadout_name(r.editing))
        },
        field: r.field,
        glance: (!r.range).then(|| glance_of(r.seed, r.depth)),
        dps: dps_of(r).map(|p| p.dps),
        stats: dps_of(r).map_or(Vec::new(), |p| profile_bars(&p)),
        run: (!r.range).then(|| run_view(r)),
        slime: (!r.range && !does.is_empty()).then(|| (title_case(l.tier.name()), does.to_string())),
        twist: r.twist().map(|t| (t.name().to_string(), t.card())),
        shots: shots
            .into_iter()
            .map(|t| match t {
                rounds::Shot::Fired(n) => ShotTag::Fired(n),
                rounds::Shot::Every => ShotTag::Every,
                rounds::Shot::Idle => ShotTag::Idle,
            })
            .collect(),
        plain_shot,
    }
}

/// The run as the menu's caption, the shop's strip and the end of a run
/// show it (`start::RunView`, Look D, 11-24): each planet's name and
/// slime, the stars won, where it stands.
pub(super) fn run_view(r: &Run) -> slime_mould::world::start::RunView {
    use slime_mould::mould::run;
    use slime_mould::world::start::{RunStop, RunView};
    let n = run::PLANETS.max(r.depth);
    let stops = (1..=n)
        .map(|d| {
            let mut at = r.clone();
            at.depth = d;
            let stars = r.stars.get(d as usize - 1).copied();
            RunStop { name: at.planet().name, slime: title_case(at.level().tier.name()), stars, here: d == r.depth && stars.is_none(), lost: r.over && !r.victory() && d == r.depth }
        })
        .collect();
    RunView { stops, depth: r.depth, points: r.points, played: (r.played > 0.0).then_some(r.played), over: r.over, won: r.victory(), mid: false }
}

/// The estimated damage a second of the gun the shop's slots show
/// (`mould::dps`): its gun as armed (the turrets' rack for a loadout with
/// none), its cards, the run's rules.
fn dps_of(r: &Run) -> Option<Profile> {
    profile_with(r, &r.gun())
}

/// The gun the shop's slots show, as armed, fired with `cards`
/// (`mould::dps::profile`).
fn profile_with(r: &Run, cards: &[rounds::Card]) -> Option<Profile> {
    let gun = r.armed_of(r.editing).unwrap_or_else(|| r.rack());
    let mut rules = Rules::default();
    r.rules(&mut rules);
    let rr = RoundRules { budget: r.budget(), ..RoundRules::default() };
    Some(slime_mould::mould::dps::profile(&gun, cards, &rules, &rr))
}

/// A gun's stats as the shop's bars (the owner, 2026-11-22 08:53Z: "make
/// visual for speed range, etc"): damage a shot, fire rate, range, bullet
/// speed and aim, each on a log scale from a quarter to four times the
/// starting rifle's, which sits in the middle (damage to sixteen times), so a card's bar and the
/// tick of the gun without it share one scale. Aim is the spread turned
/// over, so a longer bar is better on every row.
fn profile_bars(p: &Profile) -> Vec<(String, f32, String)> {
    let bar = |x: f32| (0.5 + x.max(1e-3).ln() / 16f32.ln()).clamp(0.04, 1.0);
    let rules = Rules::default();
    vec![
        // Damage on a longer scale, a quarter to sixteen times the rifle's
        // (the rifle a third of the way): a scattergun's shot is 58 and a
        // pierce on it 144, both past four times.
        ("Damage".to_string(), ((p.damage / 10.0 * 4.0).max(1e-3).ln() / 64f32.ln()).clamp(0.04, 1.0), format!("{:.0}", p.damage)),
        ("Fire rate".to_string(), bar(p.rate / (1.0 / rules.shot_every.max(1e-3))), format!("{:.1}/s", p.rate)),
        ("Range".to_string(), bar(p.range / rules.range.max(1e-3)), format!("{:.0}", p.range)),
        ("Speed".to_string(), bar(p.speed / rules.bullet_speed.max(1e-3)), format!("{:.0}", p.speed)),
        ("Aim".to_string(), bar(guns::RIFLE_SPREAD / p.spread.max(1e-3)), format!("{:.1}", p.spread)),
    ]
}

/// That estimate once `pick` is made, on a copy of the run with the points
/// to make it (so an offer too dear shows what it would do), and whether
/// it is only what it would be with room: a card that would go to the bag
/// (the gun's slots or budget full) is reckoned in the gun anyway, where
/// the game puts cards (`hypothetical`), so a full gun still shows what a
/// card is worth. None for a pick that changes no gun, or that the run
/// refuses.
fn dps_after(r: &Run, pick: Pick) -> Option<(Profile, bool)> {
    let card = match pick {
        Pick::Offer(i) => r.offers.get(i).copied(),
        Pick::Card(i) => r.cards.get(i).filter(|c| !c.1).map(|c| c.0),
        Pick::Gun(_) | Pick::Carry(_) => None,
        Pick::Item(i) if matches!(run::ITEMS.get(i).map(|x| x.key), Some("strike" | "rate")) => None,
        _ => return None,
    };
    let mut t = r.clone();
    t.points = t.points.max(1_000_000);
    let done = t.pick(pick);
    match card {
        // A card meant for the gun that did not go in.
        Some(c) if !done || t.gun().len() <= r.gun().len() => Some((hypothetical(r, c)?, true)),
        _ if !done => None,
        _ => Some((dps_of(&t)?, false)),
    }
}

/// The estimate with card `c` in the gun past its slots and budget, where
/// `Run::put_in` would put it: after the cards of its rank, before the
/// first of a later one (flight cards, then modifiers and triggers, then
/// bullets).
fn hypothetical(r: &Run, c: rounds::Card) -> Option<Profile> {
    use rounds::Kind as K;
    let rank = |c: rounds::Card| match c.kind() {
        K::Flight => 0,
        K::Modifier | K::Trigger => 1,
        K::Payload => 2,
    };
    let mut cards = r.gun();
    let at = cards.iter().position(|&o| rank(o) > rank(c)).unwrap_or(cards.len());
    cards.insert(at, c);
    profile_with(r, &cards)
}

/// A rolled bullet's numbers as bars against a standard bullet's: each on
/// a log scale from a quarter to four times (`bullets::MULT_MIN`, `MAX`),
/// a standard bullet's at the middle. Spread reads as Aim, so a longer bar
/// is better on every row.
fn ammo_bars(a: &slime_mould::mould::bullets::Ammo) -> Vec<(String, f32, String)> {
    use slime_mould::mould::bullets::DAMAGE;
    let bar = |x: f32| (0.5 + x.max(1e-3).ln() / 16f32.ln()).clamp(0.04, 1.0);
    let times = |x: f32| format!("x{x:.1}");
    vec![
        ("Damage".to_string(), bar(a.damage() / DAMAGE), format!("{:.0}", a.damage())),
        ("Speed".to_string(), bar(a.speed()), times(a.speed())),
        ("Range".to_string(), bar(a.range()), times(a.range())),
        ("Aim".to_string(), bar(1.0 / a.spread().max(1e-3)), times(1.0 / a.spread().max(1e-3))),
    ]
    .into_iter()
    // How long its crater, burn, lure or status holds (gun step F's
    // threshold roll, `Ammo::lasts`), for the payloads that last.
    .chain(a.shell.lasting().then(|| ("Lasts".to_string(), bar(a.lasts()), times(a.lasts()))))
    .collect()
}

/// The planet ahead drawn small for the shop's pane (`start::Glance`),
/// from the map the level generator makes for the run's seed and depth.
/// Kept for the last planet asked, since the shop is rebuilt after every
/// buy and a map takes a moment to make.
fn glance_of(seed: u64, depth: u32) -> Glance {
    thread_local! {
        static LAST: std::cell::RefCell<Option<((u64, u32), Glance)>> = const { std::cell::RefCell::new(None) };
    }
    if let Some(g) = LAST.with(|l| l.borrow().as_ref().filter(|(k, _)| *k == (seed, depth)).map(|(_, g)| g.clone())) {
        return g;
    }
    let m = slime_mould::mould::levelgen::level(seed, depth);
    // 96 across (64 until 11-24; the pane blends between cells too).
    let g = glance_from(&m, 96);
    LAST.with(|l| *l.borrow_mut() = Some(((seed, depth), g.clone())));
    g
}

/// `m` as a glance `across` cells wide: each cell the mean colour of the
/// map cells under it, rock, deep and shallow water and the soils in the
/// game's own dark values, so the shapes read as the planet will.
pub(super) fn glance_from(m: &Map, across: usize) -> Glance {
    use slime_mould::mould::{big, map};
    let w = across.min(m.w).max(1);
    let h = ((w * m.h) as f32 / m.w as f32).round().max(1.0) as usize;
    let colour = |g: u8| -> [f32; 3] {
        let soil = g >> 4;
        if g & map::G_ROCK != 0 {
            [92.0, 88.0, 82.0]
        } else if soil == big::SOIL_DEEP {
            [22.0, 40.0, 66.0]
        } else if soil == big::SOIL_BRIDGE {
            [104.0, 86.0, 60.0]
        } else if g & map::G_WET != 0 {
            [34.0, 58.0, 74.0]
        } else {
            match soil {
                big::SOIL_SAND => [74.0, 68.0, 52.0],
                big::SOIL_CLAY => [66.0, 54.0, 44.0],
                big::SOIL_BARREN => [70.0, 68.0, 62.0],
                _ => [44.0, 52.0, 40.0],
            }
        }
    };
    let mut cells = Vec::with_capacity(w * h);
    for j in 0..h {
        for i in 0..w {
            let (x0, x1) = (i * m.w / w, ((i + 1) * m.w / w).max(i * m.w / w + 1));
            let (y0, y1) = (j * m.h / h, ((j + 1) * m.h / h).max(j * m.h / h + 1));
            let mut sum = [0.0f32; 3];
            let mut n = 0.0;
            // Every second map cell each way: the mean is as good, at a
            // quarter of the reads.
            for y in (y0..y1.min(m.h)).step_by(2) {
                for x in (x0..x1.min(m.w)).step_by(2) {
                    let c = colour(m.ground[y * m.w + x]);
                    for k in 0..3 {
                        sum[k] += c[k];
                    }
                    n += 1.0;
                }
            }
            let n = f32::max(n, 1.0);
            // A little under the game's values: the pane is darker than
            // the world, and the marks on it must stand out.
            let k = 0.8 / n;
            cells.push([(sum[0] * k) as u8, (sum[1] * k) as u8, (sum[2] * k) as u8, 255]);
        }
    }
    let (mw, mh) = (m.w as f32, m.h as f32);
    Glance {
        w,
        h,
        cells,
        crystals: m.items.crystals.iter().map(|c| (c.x / mw, c.y / mh, c.r / mw)).collect(),
        moulds: m.items.moulds.iter().map(|&(x, y)| (x / mw, y / mh)).collect(),
        landing: m.items.landing.map(|(x, y)| (x / mw, y / mh)),
    }
}

/// The range's heading for a card row: the shelf's by what the card
/// works on, or the bag.
fn range_shelf(row: &run::Row) -> &'static str {
    match (row.pick, &row.face) {
        // The shove cards their own shelf (2026-11-16): with them on the
        // next bullet's, that tab took two pages at 200% on 1080 lines.
        // With the trails since 11-17 (below).
        (Pick::Offer(_), Face::Card(rounds::Card::Heave | rounds::Card::Pull)) => "Trails and shoves",
        // The dart (2026-11-24) moves the slime too: on the special
        // bullets' tab it took two pages at 200% on 1080 lines.
        (Pick::Offer(_), Face::Card(rounds::Card::Ammo(a))) if a.shell == rounds::Shell::Dart => "Trails and shoves",
        // The Mind bullets too (2026-11-16): three more rows of the
        // bullets' grid put that tab on three pages at 200% on 1080 lines.
        // The readers (Shatter, Detonate) share it: their own tab would
        // have been the armory's seventh for two rows; so do the flare and
        // the torch (2026-11-17), the route and element families' payloads.
        (Pick::Offer(_), Face::Card(rounds::Card::Ammo(a))) if a.shell.special() => "Special bullets",
        // The trails off every bullet's shelf (2026-11-17): with Tinder,
        // the fifth trail, it took two pages at 200% on 1080 lines. They
        // share the shove cards' tab (eight rows): a seventh tab ran the
        // tab row under the card's panel at 200% on 1920 x 1080.
        (Pick::Offer(_), Face::Card(c)) if rounds::trails(&[*c]) != [0; rounds::TRAILS] => "Trails and shoves",
        (Pick::Offer(_), Face::Card(c)) => match c.kind() {
            rounds::Kind::Payload => "Bullets",
            rounds::Kind::Flight => "Cards for every bullet",
            _ => "Cards for the next bullet",
        },
        _ => BAG,
    }
}

/// The gun range's round: the sandbox's, as its pick would start it,
/// armed from the range's armory (`r`).
pub(super) fn range_args(base: &Args, chosen: bool, menu: &StartMenu, r: &Run) -> Args {
    let mut a = round_args(base, chosen, menu, &StartEvent::Sandbox);
    // The armory's gun and cards, not the command line's.
    (a.gun, a.ammo) = (None, None);
    a.range = Some(r.clone());
    a
}

/// An upgrade's icon: the command card's own for what it unlocks.
fn item_icon(key: &str) -> Icon {
    match key {
        "hp" => Icon::Heart,
        // Carrion, the first relic (gun step F): a lure where slime died.
        "carrion" => Icon::Lure,
        "fever" => Icon::Germ,
        "backdraft" => Icon::Flamer,
        "strike" => Icon::Attack,
        "rate" => Icon::Fast,
        "range" => Icon::Eye,
        "squad" => Icon::Recruit,
        "bank" => Icon::Crystal,
        "harvest" => Icon::Upgrade,
        "fireproof" => Icon::Shield,
        "walls" => Icon::Wall,
        "turret" => Icon::Turret,
        "turret+" => Icon::Upgrade,
        "tools+" => Icon::Tag,
        "mortar" => Icon::Bomb,
        "sprayer" => Icon::Gas,
        "hunter" => Icon::Target,
        "lure" => Icon::Lure,
        "pump+" => Icon::Crystal,
        "outpost" => Icon::Hold,
        "siren" => Icon::Alert,
        "blast" => Icon::Grenade,
        "mine" => Icon::Mine,
        "cord" => Icon::Burn,
        "gas" => Icon::Sleep,
        "draw" => Icon::Menu,
        _ => Icon::Upgrade,
    }
}

/// An upgrade's effect as the shop's bars, now and at its next level
/// (the owner, 2026-11-22: "more of visual representation"); empty for an
/// upgrade with none drawn yet.
fn item_bars(key: &str, level: u8) -> Vec<(String, f32, String)> {
    match key {
        "fireproof" => {
            let bar = |l: u8| {
                let f = run::fireproof_share(l);
                (f, if f <= 0.0 { "immune".to_string() } else { format!("{:.0}%", 100.0 * f) })
            };
            let (now, now_t) = bar(level);
            let mut b = vec![("Fire harm now".to_string(), now, now_t)];
            if level < 3 {
                let (next, next_t) = bar(level + 1);
                b.push((format!("At level {}", level + 1), next, next_t));
            }
            b
        }
        _ => Vec::new(),
    }
}

/// A gun's numbers as the shop's bars, each against about the best a run
/// finds.
/// A device body's rolls as bars (second polish pass, 11-24): reach,
/// cooldown (shorter fills more), card slots and budget, each filled across
/// its roll's range (`bodies::REACH` and the rest); with `mine`, the body
/// of that kind the run owns, each value reads as its then this one's.
/// A body's tool's own cooldown, seconds (`KitRules`), before its roll.
fn tool_cd(kind: slime_mould::mould::bodies::BodyKind) -> f32 {
    use slime_mould::mould::bodies::BodyKind;
    let k = slime_mould::mould::kit::KitRules::default();
    match kind {
        BodyKind::Grenade => k.blast_cd,
        BodyKind::Mine => k.mine_cd,
        BodyKind::Line => k.cord_cd,
        BodyKind::Canister => k.gas_cd,
    }
}

fn body_bars(b: &slime_mould::mould::bodies::Body, mine: Option<&slime_mould::mould::bodies::Body>) -> Vec<(String, f32, String)> {
    use slime_mould::mould::bodies::{Body, BUDGET, COOL, REACH, SLOTS};
    let span = |v: f32, lo: f32, hi: f32| (0.15 + 0.85 * (v - lo) / (hi - lo)).clamp(0.0, 1.0);
    let rows = |b: &Body| {
        [
            ("Reach", span(b.reach, REACH.0, REACH.1), format!("{:.0}", b.reach)),
            // Seconds, the tool's own cooldown times the roll (the
            // percentage read as nothing in the shop, Direction 12:18Z),
            // before the run's Cheaper tools.
            ("Cooldown", span(COOL.1 - b.cool, 0.0, COOL.1 - COOL.0), format!("{:.0} s", tool_cd(b.kind) * b.cool)),
            ("Card slots", span(b.slots as f32, SLOTS.0 as f32, SLOTS.1 as f32), b.slots.to_string()),
            ("Budget", span(b.budget as f32, BUDGET.0 as f32, BUDGET.1 as f32), b.budget.to_string()),
        ]
    };
    let was = mine.map(rows);
    rows(b)
        .into_iter()
        .enumerate()
        .map(|(k, (l, f, v))| {
            let v = match &was {
                Some(w) if w[k].2 != v => format!("{} \u{2192} {v}", w[k].2),
                _ => v,
            };
            (l.to_string(), f, v)
        })
        .collect()
}

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
    // (`shop-mid@TAB@NAME`: a tab and a pick, `headless`.)
    let name = name.split('@').next().unwrap_or(name);
    Some(match name {
        // (`main-run`, `main-won`, `main-lost`: the Run row with a run
        // three planets in, a won run, a run lost on planet 4, `headless`.)
        "1" | "main" | "main-run" | "main-won" | "main-lost" | "main-runs" => Page::Main,
        // (The Past runs page over three made-up runs, `headless`.)
        "runs" => Page::Runs,
        "campaign" | "missions" => Page::Missions,
        "options" | "options-planet" | "options-mould" | "options-experiments" => Page::Options,
        "keys" => Page::Keys,
        // (`news-try`: its second tab, the things to try.)
        "news" | "news-try" => Page::News,
        "shop" | "shop-mid" | "shop-build" | "shop-tools" | "shop-turrets" | "shop-landing" => Page::Shop,
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
    // Not a run's level unless `run_args` makes it one, nor the gun range
    // unless `range_args` does.
    a.run = None;
    a.range = None;
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
    // The Experiments tab's picks, last so they stand over the rest.
    experiments::apply(&mut a, base, menu);
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
        let mut menu = self.menu.take().unwrap_or_else(|| start_menu(&base));
        // What this PC has seen and ticked, over the menu's own (a PC
        // that has not played). The no-window films keep the menu's: their
        // memory is never this PC's.
        if self.window.is_some() {
            menu.build = Some(build_page(&self.memory));
        }
        self.args = round_args(&base, chosen, &menu, &StartEvent::Sandbox);
        // A run's planet behind the menu (Look D, 11-20): one of planets 1
        // to 4 of a run on the clock's seed, its slime grown by the level's
        // head start, as the game is now played. Until 11-20 the dish was
        // the sandbox's map (the Fork, the old herding puzzle's walls). A map
        // picked on the Planet tab, or given on the command line, still
        // shows here, so that tab keeps its preview.
        if !chosen && !self.args.own && menu.option("map").is_none_or(|o| o.is_default()) {
            let secs = std::time::SystemTime::now().duration_since(std::time::UNIX_EPOCH).map_or(1, |d| d.as_secs());
            let mut r = Run::new(secs);
            r.depth = 1 + (secs % 4) as u32;
            self.args = run_args(&base, &menu, &r);
        }
        self.name = self.args.map.clone().unwrap_or_else(|| "noise".into());
        self.menu = Some(menu);
        self.paused = false;
        self.start()?;
        self.menu_camera();
        Ok(())
    }

    /// The map filling the window, under no HUD (Look D, 11-20): zoomed to
    /// cover the screen, its edges cropped, not fitted whole inside black
    /// bands as a board on a table (before 11-20).
    pub(super) fn menu_camera(&mut self) {
        self.camera.set_inset(0.0, 0.0);
        let (screen, world) = (self.camera.screen, self.camera.world);
        let z = (screen.0 / world.0).max(screen.1 / world.1).max(self.camera.fit());
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
        if let Some(v) = self.menu.as_ref().and_then(|m| m.option("volume")).and_then(|o| settings::parse_volume(o.value())).filter(|&v| v != volume_user()) {
            self.set_volume(v);
        }
        if let Some(v) = self.menu.as_ref().and_then(|m| m.option("music")).and_then(|o| settings::parse_volume(o.value())).filter(|&v| v != music_user()) {
            self.set_music(v);
        }
        if let Some(v) = self.menu.as_ref().and_then(|m| m.option("detail")).map(|o| o.value().to_string()).filter(|v| v != detail_user()) {
            self.set_detail(&v);
        }
        if let Some(v) = self.menu.as_ref().and_then(|m| m.option("shake")).and_then(|o| settings::parse_volume(o.value())).filter(|&v| v != shake_user()) {
            self.set_shake(v);
        }
        if let Some(on) = self.menu.as_ref().and_then(|m| m.option("hints")).map(|o| o.value() == "on").filter(|&on| on != super::hints::saved_on()) {
            let _ = settings::set("hints", if on { "on" } else { "off" });
            self.hints.on = on;
        }
        let Some(m) = self.menu.as_mut() else { return Ok(picks) };
        if picks.is_empty() && dish_key(m) != before {
            self.open_menu()?;
        }
        Ok(picks)
    }

    pub(super) fn menu_events(&mut self, picks: Vec<StartEvent>, event_loop: &ActiveEventLoop) {
        // Landing: the menu waits for it (`landing_frame`).
        if self.landing_due.is_some() {
            return;
        }
        for p in picks {
            // A run's next planet lands a frame later, under its landing
            // card (Direction 11-24, the time between planets: the planet
            // and its head start were built with the shop frozen under the
            // pressed button, up to an estimated 3 s on the 1070 for
            // planet 8's 87 s head start).
            if p == StartEvent::NextLevel && self.run.is_some() && !self.fielding() && !self.ranging() {
                self.show_landing();
                self.landing_due = Some(2);
                return;
            }
            match self.menu_event(p) {
                Ok(true) => {
                    self.save_replay();
                    self.save_run();
                    event_loop.exit();
                }
                Ok(false) => {}
                Err(e) => return self.fail(event_loop, e),
            }
        }
    }

    /// The shop's landing card for the run's next planet.
    pub(super) fn show_landing(&mut self) {
        let Some(r) = self.run.as_ref() else { return };
        let level = r.level();
        let slime = draw::title_case(level.tier.name());
        let line = if level.grow > 0.0 { format!("{slime} has had {:.0} s to spread", level.grow) } else { slime };
        let card = (format!("Landing on planet {} of {}", r.depth, slime_mould::mould::run::PLANETS), r.planet().name, line);
        if let Some(m) = self.menu.as_mut() {
            m.landing = Some(card);
        }
    }

    /// Counts down a landing (`App::landing_due`) and lands at 0: the
    /// landing card has been on screen a frame by then.
    pub(super) fn landing_frame(&mut self, event_loop: &ActiveEventLoop) {
        match self.landing_due {
            None => {}
            Some(n) if n > 0 => self.landing_due = Some(n - 1),
            Some(_) => {
                self.landing_due = None;
                if let Some(m) = self.menu.as_mut() {
                    m.landing = None;
                }
                match self.menu_event(StartEvent::NextLevel) {
                    Ok(true) => event_loop.exit(),
                    Ok(false) => {}
                    Err(e) => self.fail(event_loop, e),
                }
            }
        }
    }

    /// What a pick on the menu does; true to quit.
    pub(super) fn menu_event(&mut self, ev: StartEvent) -> Result<bool, String> {
        match ev {
            StartEvent::Quit => return Ok(true),
            StartEvent::Range => self.show_armory(),
            // The slime viewer (`viewer.rs`): a run's planet 1, its panel
            // open; Main menu leaves it.
            StartEvent::Viewer => {
                let Some((base, _)) = self.base.clone() else { return Ok(false) };
                let Some(menu) = self.menu.take() else { return Ok(false) };
                let mut r = Run::new(fresh_seed());
                r.dev = true;
                r.restock();
                self.args = run_args(&base, &menu, &r);
                (self.args.viewer, self.args.squads, self.args.dev) = (true, true, true);
                self.round = self.args.round;
                self.parked = Some(menu);
                self.tweaks = slime_mould::world::tweaks::Tweaks::load();
                self.viewer = Some(viewer::Viewer::new(&self.tweaks));
                self.apply_look_tweaks();
                self.start()?;
                self.paused = false;
            }
            StartEvent::NextLevel if self.fielding() => self.close_loadouts(),
            StartEvent::EndRun if self.fielding() => self.close_loadouts(),
            StartEvent::NextLevel if self.ranging() => {
                if self.armory {
                    self.close_armory();
                } else {
                    // Into the range, armed from the armory.
                    let (Some((base, chosen)), Some(r), Some(menu)) = (self.base.clone(), self.range.clone(), self.menu.take()) else { return Ok(false) };
                    self.args = range_args(&base, chosen, &menu, &r);
                    self.round = self.args.round;
                    self.name = self.args.map.clone().unwrap_or_else(|| "noise".into());
                    self.parked = Some(menu);
                    self.start()?;
                    self.paused = false;
                    self.hud.alert("The gun range: I opens the armory, B buys soldiers, N a new map", None, theme::ACCENT);
                }
            }
            StartEvent::EndRun if self.ranging() => {
                // Leaves the range for the main page (the gun stays built
                // for next time).
                if self.armory {
                    self.set_armory(false);
                    self.parked = self.menu.take();
                    return self.back_to_menu().map(|()| false);
                }
                if let Some(m) = self.menu.as_mut() {
                    m.shop = None;
                    m.show(Page::Main);
                }
            }
            // A planet left mid-game carries on (save and resume A,
            // `resume.rs`); else the shop, then the next planet.
            StartEvent::Run if self.run.is_some() => {
                if !self.resume_planet()? {
                    self.show_shop();
                }
            }
            StartEvent::Run | StartEvent::NextLevel => {
                let Some((base, _)) = self.base.clone() else { return Ok(false) };
                let fresh = self.run.is_none();
                let mut r = self.run.take().unwrap_or_else(|| {
                    // A new run on a new seed each time (the clock), so two
                    // runs differ; `run=SEED` on the command line repeats one.
                    let seed = std::time::SystemTime::now().duration_since(std::time::UNIX_EPOCH).map_or(1, |d| d.as_secs());
                    Run::new(seed)
                });
                let Some(menu) = self.menu.take() else { return Ok(false) };
                r.dev = menu.option("dev").is_some_and(|o| o.value() == "on");
                // Options' "Start on planet": a new run skips to it and
                // opens the shop there (`Run::skip`), rather than landing.
                let start = menu.option("start").and_then(|o| o.value().parse::<u32>().ok()).unwrap_or(1);
                if fresh && start > 1 {
                    r.restock();
                    while r.depth < start && r.skip() {}
                    self.run = Some(r);
                    self.menu = Some(menu);
                    self.show_shop();
                    self.save_run();
                    return Ok(false);
                }
                self.args = run_args(&base, &menu, &r);
                self.run = Some(r);
                self.round = self.args.round;
                self.parked = Some(menu);
                // A planet landing fresh: any planet saved before is old.
                self.drop_planet();
                self.start()?;
                self.paused = false;
                // As it lands: closed mid-planet, it lands here again.
                self.save_run();
            }
            StartEvent::Buy(i) => {
                if let Some(r) = self.shop_run() {
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
                if let Some(r) = self.shop_run() {
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
                let gone = self.run.take();
                if let Some(r) = gone.as_ref().filter(|r| !r.range) {
                    let why = format!("Given up in the shop before planet {}, with {}.", r.depth, r.line());
                    self.keep_run(r, "given up", why);
                }
                let line = gone.map(|r| format!("Run given up on planet {}: {}", r.depth, r.line()));
                if let Some(m) = self.menu.as_mut() {
                    m.run_live = false;
                    m.shop = None;
                    m.notice = line;
                    m.show(Page::Main);
                }
                self.save_run();
            }
            // A thing to try ticked or unticked: this PC remembers.
            StartEvent::Tried(i, tried) => {
                if let Some(t) = news::to_try().get(i) {
                    self.memory.set_tried(t, tried);
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

    /// The armory up over the range's round, or not. The view draws a
    /// minimap of its own whenever the camera doesn't see the whole map,
    /// and the armory leaves the round's camera as it was (zoomed in), so
    /// the minimap is switched off under it and back as it was after.
    pub(super) fn set_armory(&mut self, on: bool) {
        if on == self.armory {
            return;
        }
        self.armory = on;
        if let Some(v) = self.view.as_mut() {
            if on {
                self.armory_minimap = std::mem::replace(&mut v.minimap, false);
            } else {
                v.minimap = self.armory_minimap;
            }
        }
    }

    /// The shop page on show is the gun range's armory.
    fn ranging(&self) -> bool {
        self.menu.as_ref().and_then(|m| m.shop.as_ref()).is_some_and(|s| s.range)
    }

    /// The run the shop page on show buys for: the range's in its
    /// armory, else the roguelike run's.
    fn shop_run(&mut self) -> Option<&mut Run> {
        if self.ranging() {
            self.range.as_mut()
        } else {
            self.run.as_mut()
        }
    }

    /// The gun range's armory as the menu's shop page (a new range the
    /// first time), over the menu's dish or the range round.
    pub(super) fn show_armory(&mut self) {
        let r = self.range.get_or_insert_with(|| Run::range(fresh_seed()));
        if let Some(m) = self.menu.as_mut() {
            m.shop = Some(shop_of(r));
            if m.page != Page::Shop {
                m.show(Page::Shop);
            }
        }
    }

    /// I in the range round: the armory over it, the round paused under
    /// it (the parked menu brought back on its shop page).
    pub(super) fn open_armory(&mut self) {
        if self.args.range.is_none() || self.in_menu() {
            return;
        }
        let Some((base, _)) = self.base.clone() else {
            self.hud.alert("The armory is on the start menu's Gun range", None, theme::WARN);
            return;
        };
        let mut menu = self.parked.take().unwrap_or_else(|| start_menu(&base));
        if self.window.is_some() {
            menu.build = Some(build_page(&self.memory));
        }
        self.menu = Some(menu);
        self.set_armory(true);
        self.paused = true;
        self.show_armory();
    }

    /// Back from the armory to the range round: its gun on every squad
    /// and the army's round its cards (each squad back on it), the menu
    /// parked again, play on. Restart and N keep it (`args.range`).
    pub(super) fn close_armory(&mut self) {
        self.set_armory(false);
        if let Some(mut m) = self.menu.take() {
            m.shop = None;
            m.show(Page::Main);
            self.parked = Some(m);
        }
        self.paused = false;
        let Some(r) = self.range.clone() else { return };
        if let Some(g) = self.game.as_mut() {
            let mut rr = RoundRules { budget: r.budget(), ..RoundRules::default() };
            r.round_rules(&mut rr);
            g.rounds = Rounds::new(r.squad_cards(), rr);
            let gun = r.carried();
            for q in 0..g.squads.len() {
                g.apply(Cmd::Arm(q as u8, Some(gun)));
                g.apply(Cmd::Load(q as u8, None));
            }
        }
        let cards: Vec<String> = r.squad_cards().iter().map(|c| c.title()).collect();
        let what = if cards.is_empty() { format!("{}, no cards: plain shots", draw::capital(&r.carried().title())) } else { format!("{}: {}", draw::capital(&r.carried().title()), cards.join(", ")) };
        self.hud.alert(&what, None, theme::ACCENT);
        self.args.range = Some(r);
    }

    /// The loadout panel is up over a run's planet (`Run::field`).
    pub(super) fn fielding(&self) -> bool {
        self.armory && self.run.as_ref().is_some_and(|r| r.field)
    }

    /// H on a run's planet (owner, 2026-11-19 19:22Z: "A key (or a HUD
    /// button) opens the loadout panel and pauses the game"): the shop's
    /// page with only the guns and cards owned, nothing for sale, showing
    /// the loadout the commanded squad uses, the planet paused under it.
    pub(super) fn open_loadouts(&mut self) {
        if self.args.run.is_none() || self.in_menu() {
            return;
        }
        let Some((base, _)) = self.base.clone() else { return };
        let Some(g) = self.game.as_ref() else { return };
        let q = g.active.or_else(|| g.live_squads().first().copied()).unwrap_or(0);
        let l = g.squads.get(q).map_or(0, |s| s.loadout as usize);
        // The run as the planet has it (`args.run`), the menu's copy too.
        let Some(mut r) = self.args.run.clone() else { return };
        r.field = true;
        r.field_squad = Some((q as u8, l));
        self.field_was = l;
        self.field_paused = self.paused;
        r.edit(l.min(r.loadouts.len() - 1));
        self.run = Some(r);
        let mut menu = self.parked.take().unwrap_or_else(|| start_menu(&base));
        if self.window.is_some() {
            menu.build = Some(build_page(&self.memory));
        }
        self.menu = Some(menu);
        self.set_armory(true);
        self.paused = true;
        self.show_shop();
    }

    /// Back to the planet from the loadout panel: every squad armed from
    /// the loadout it uses (the panel's squad from the one picked for it),
    /// the turrets from theirs, at once (no reload, owner 19:22Z), the run
    /// kept for the next shop, play on.
    pub(super) fn close_loadouts(&mut self) {
        self.set_armory(false);
        if let Some(mut m) = self.menu.take() {
            m.shop = None;
            m.show(Page::Main);
            self.parked = Some(m);
        }
        // Paused as before it (P then H then Esc stays paused).
        self.paused = self.field_paused;
        let Some(r) = self.run.as_mut() else { return };
        r.field = false;
        let chosen = r.field_squad.take();
        let r = r.clone();
        self.args.run = Some(r.clone());
        let Some(g) = self.game.as_mut() else { return };
        // The army's round, keeping what is live (`Rounds::set_round`).
        g.rounds.set_round(r.squad_cards(), r.budget());
        // Only a squad given another loadout (`Pick::SquadUses`), and its
        // links, change: opening and closing set every linked squad to the
        // commanded one's (review, 2026-11-20).
        if let Some((q, l)) = chosen.filter(|c| c.1 != self.field_was) {
            // The squad and every squad linked with it.
            for o in g.linked(q as usize) {
                if let Some(s) = g.squads.get_mut(o) {
                    s.loadout = l as u8;
                }
            }
        }
        arm_squads(g, &r);
        let names: Vec<String> = r.cards_of(chosen.map_or(0, |c| c.1)).iter().map(|c| c.title()).collect();
        let l = chosen.map_or(0, |c| c.1);
        let what = format!("{}: {}", r.loadout_name(l), if names.is_empty() { "no cards, plain shots".to_string() } else { names.join(", ") });
        self.hud.alert(&what, None, theme::ACCENT);
        self.save_run();
    }

    /// The shop page, filled from the run (the menu up, its dish behind).
    pub(super) fn show_shop(&mut self) {
        if self.ranging() {
            return self.show_armory();
        }
        let Some(r) = self.run.as_mut() else { return };
        if let Some(m) = self.menu.as_mut() {
            r.dev = m.option("dev").is_some_and(|o| o.value() == "on");
            m.shop = Some(shop_of(r));
            m.run_live = true;
            if m.page != Page::Shop {
                m.show(Page::Shop);
            }
        }
        // Each buy, move and pick in the shop comes back through here.
        self.save_run();
    }

    /// The run saved when the game last closed, as the run under way (the
    /// main menu's "Back to the run"). A save that does not read is moved
    /// aside to `run-unreadable.txt` with a word on the menu, never
    /// overwritten unseen.
    pub(super) fn load_run(&mut self) {
        // The runs that ended, for the Past runs page.
        self.load_runs();
        let Some(p) = self.run_file.clone() else { return };
        let Ok(text) = std::fs::read_to_string(&p) else { return };
        let note = match Run::from_save(&text) {
            Ok(r) if !r.over => {
                let (depth, line) = (r.depth, r.line());
                self.run_saved = text;
                self.run = Some(r);
                if self.planet_saved().is_some() {
                    format!("Your run is saved mid-planet {depth} ({line}): Back to the run carries on where you left")
                } else {
                    format!("Your run is saved on planet {depth} ({line}): Back to the run carries on")
                }
            }
            Ok(_) => {
                let _ = std::fs::remove_file(&p);
                return;
            }
            Err(e) => {
                let aside = p.with_file_name("run-unreadable.txt");
                let _ = std::fs::rename(&p, &aside);
                eprintln!("larder-world: the saved run did not load ({e}); moved to {}", aside.display());
                format!("The saved run could not be read ({e}); it was moved to {}", aside.display())
            }
        };
        if let Some(m) = self.menu.as_mut() {
            m.run_live = self.run.is_some();
            m.notice = Some(note);
        }
    }

    /// **Saves the run** to its file (`run_file`) if it changed since the
    /// last write, or deletes the file once there is no run under way (given
    /// up, lost or won). Called wherever the run changes (landing, each buy
    /// in the shop, the loadout panel closing, a planet's end) and on
    /// quitting, and every few seconds as a net (`autosave`). Written to a
    /// side file and renamed over the save, so a crash mid-write leaves the
    /// last good save.
    pub(super) fn save_run(&mut self) {
        let Some(p) = self.run_file.clone() else { return };
        // The run in the shop, or as it landed on the planet being played;
        // the loadout panel's copy is the window's until it closes.
        let live = self.run.as_ref().filter(|r| !r.over && !r.range && !r.field).or(self.args.run.as_ref().filter(|r| !r.over && self.run.is_some()));
        let Some(r) = live else {
            self.drop_planet();
            if !self.run_saved.is_empty() || p.exists() {
                if let Err(e) = std::fs::remove_file(&p).or_else(|e| if e.kind() == std::io::ErrorKind::NotFound { Ok(()) } else { Err(e) }) {
                    eprintln!("larder-world: the finished run's save was not removed ({}: {e})", p.display());
                }
                self.run_saved.clear();
            }
            return;
        };
        let text = r.save_text();
        if text == self.run_saved {
            return;
        }
        match write_save(&p, &text) {
            Ok(()) => self.run_saved = text,
            Err(e) => eprintln!("larder-world: the run was not saved ({e})"),
        }
    }

    /// The save as a net, every few seconds of the window's time.
    pub(super) fn autosave(&mut self) {
        if self.run_file.is_none() || self.run_checked.is_some_and(|t| t.elapsed().as_secs_f32() < AUTOSAVE_SECS) {
            return;
        }
        self.run_checked = Some(Instant::now());
        self.save_run();
    }

    /// The end screen's carry-on in a run: the level's verdict goes to the
    /// run (a win's points and the next planet; a loss ends it), then the
    /// shop, or the main menu with the run's line when it is over. With
    /// no menu (`menu=0`) the next planet starts at once.
    pub(super) fn carry_on(&mut self) -> Result<(), String> {
        let Some(v) = self.mission.as_ref().and_then(|t| t.verdict) else { return Ok(()) };
        let Some(mut r) = self.args.run.clone() else { return Ok(()) };
        // What the squad took from starfall and boss cores (`Game::starfall`).
        r.found(self.game.as_ref().map_or(0, |g| g.starfall.earned()));
        // The slime that ended a lost planet, for the Past runs page.
        let slime = title_case(r.level().tier.name());
        if v.stars > 0 {
            r.won(v.stars);
        } else {
            r.lost();
        }
        if r.over {
            let why = match (r.victory(), self.game.as_ref()) {
                (true, _) => format!("All {} planets freed, {} of {} stars.", r.stars.len(), r.stars.iter().map(|&s| u32::from(s)).sum::<u32>(), 3 * r.stars.len()),
                (false, Some(g)) => runs::why_lost(v, g, &slime),
                (false, None) => format!("{slime} won planet {}.", r.depth),
            };
            self.keep_run(&r, if r.victory() { "won" } else { "lost" }, why);
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
        let end = r.over.then(|| run_view(&r));
        self.run = if r.over { None } else { Some(r) };
        self.back_to_menu()?;
        if let Some(m) = self.menu.as_mut() {
            m.run_live = self.run.is_some();
            // The run's end is the Run row's summary (`StartMenu::run_end`,
            // Look D, 11-24); the notice line keeps the words for a film
            // or a screen too small for the summary.
            m.run_end = end;
            m.notice = over;
        }
        if self.run.is_some() {
            self.show_shop();
        }
        // Won on to the next planet, or over: the save follows.
        self.save_run();
        Ok(())
    }

    /// Back to the start menu from play (the Esc menu's or the end
    /// screen's Main menu).
    pub(super) fn back_to_menu(&mut self) -> Result<(), String> {
        self.save_replay();
        self.set_armory(false);
        // Out of the slime viewer: the look as saved, not as left.
        if self.viewer.take().is_some() {
            self.args.viewer = false;
            self.tweaks = slime_mould::world::tweaks::Tweaks::load();
            self.reset_look();
        }
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
    std::process::Command::new(&exe).current_dir(slime_mould::build::root()).spawn().map(|_| ()).map_err(|e| format!("The map editor did not start: {e}"))
}

/// Every live squad armed from the run's loadout it uses (`Squad::loadout`,
/// `Cmd::Arm` and `Cmd::Load`, so a replay keeps it), and the turrets from
/// theirs (`Run::turret_cards`), and the devices from theirs
/// (`Run::body_cards`). A loadout past the run's (an old replay)
/// is the first.
pub(super) fn arm_squads(g: &mut Game<GpuWorld>, r: &Run) {
    for q in g.live_squads() {
        let l = (g.squads[q].loadout as usize).min(r.loadouts.len() - 1);
        g.apply(Cmd::Arm(q as u8, Some(r.squad_gun(l))));
        g.apply(Cmd::Load(q as u8, Some(rounds::Deck::new(&r.cards_of(l)))));
    }
    let t = r.turret_cards();
    g.apply(Cmd::Load(slime_mould::mould::larder_world::TURRET_DECK, (!t.is_empty()).then(|| rounds::Deck::new(&t))));
    // Each body's cards (gun step E, `Run::body_cards`) as they now
    // stand.
    for kind in slime_mould::mould::bodies::BodyKind::ALL {
        let d = r.body_cards(kind);
        g.apply(Cmd::Load(slime_mould::mould::larder_world::DEVICE_DECKS + kind.tool() as u8, (!d.is_empty()).then(|| rounds::Deck::new(&d))));
    }
    // Which bodies go off on their own (gun step E4).
    g.apply(Cmd::Auto(r.autocast()));
    // Turrets on a loadout of their own (`Building::loadout`) take its
    // cards as they now stand.
    for b in 0..g.buildings.len() {
        if let Some(l) = g.buildings[b].loadout.filter(|_| g.buildings[b].alive) {
            let l = (l as usize).min(r.loadouts.len() - 1);
            g.apply(Cmd::TurretLoad(b as u32, Some(l as u8), Some(rounds::Deck::new(&r.turret_deck_of(l)))));
        }
    }
}

/// Seconds between the window's checks that the run's save is current
/// (`App::autosave`): each check costs a few hundred bytes of text, and a
/// write only when the run changed.
pub(super) const AUTOSAVE_SECS: f32 = 5.0;

/// Where the run is saved: `run.txt` beside the settings file
/// (`settings::path`: `%APPDATA%\Larder World` on Windows), or
/// `LARDER_RUN=PATH`.
pub(super) fn run_save_path() -> Option<PathBuf> {
    if let Some(p) = std::env::var_os("LARDER_RUN").filter(|v| !v.is_empty()) {
        return Some(PathBuf::from(p));
    }
    Some(settings::path()?.with_file_name("run.txt"))
}

/// Writes `text` to a side file and renames it over `p`.
pub(super) fn write_save(p: &Path, text: &str) -> Result<(), String> {
    if let Some(d) = p.parent().filter(|d| !d.as_os_str().is_empty()) {
        std::fs::create_dir_all(d).map_err(|e| format!("{}: {e}", d.display()))?;
    }
    let tmp = p.with_extension("txt.new");
    std::fs::write(&tmp, text).map_err(|e| format!("{}: {e}", tmp.display()))?;
    std::fs::rename(&tmp, p).map_err(|e| format!("{}: {e}", p.display()))
}

#[cfg(test)]
mod glance_tests {
    /// The shop's planet glance (Look D, 11-21) holds the whole planet:
    /// every cell coloured, its crystals, a nest and the landing, all on
    /// the map; and a second ask for the same planet is the same glance.
    #[test]
    fn the_glance_holds_the_planet() {
        for depth in 1..=4 {
            let g = super::glance_of(7, depth);
            assert_eq!(g.w, 96, "planet {depth}");
            assert_eq!(g.cells.len(), g.w * g.h, "planet {depth}");
            assert!(g.crystals.len() >= 6 && !g.moulds.is_empty() && g.landing.is_some(), "planet {depth}: {} crystals, {} nests", g.crystals.len(), g.moulds.len());
            let on = |x: f32, y: f32| (0.0..=1.0).contains(&x) && (0.0..=1.0).contains(&y);
            assert!(g.crystals.iter().all(|&(x, y, _)| on(x, y)) && g.moulds.iter().all(|&(x, y)| on(x, y)), "planet {depth}: a mark off the map");
            assert_eq!(super::glance_of(7, depth), g, "planet {depth}: the kept glance differs");
        }
    }
}


#[cfg(test)]
mod body_bar_tests {
    use super::body_bars;
    use slime_mould::mould::bodies::{Body, BodyKind};

    /// A device body's rolls read as four bars, an offer of an owned kind
    /// as the owned one's value then its own (second polish pass, 11-24).
    #[test]
    fn a_body_reads_as_four_bars() {
        let plain = Body::plain(BodyKind::Grenade);
        let b = body_bars(&plain, None);
        assert_eq!(b.iter().map(|x| x.0.as_str()).collect::<Vec<_>>(), ["Reach", "Cooldown", "Card slots", "Budget"]);
        assert_eq!(b[0].2, "24");
        assert!(b.iter().all(|x| (0.0..=1.0).contains(&x.1)));
        let better = Body { reach: 31.0, cool: 0.9, ..plain };
        let b = body_bars(&better, Some(&plain));
        assert_eq!(b[0].2, "24 \u{2192} 31");
        assert_eq!(b[1].2, "20 s \u{2192} 18 s", "the Grenade's 20 s blast cooldown times the roll");
        assert_eq!(b[2].2, "2");
        assert!(b[1].1 > body_bars(&plain, None)[1].1, "a shorter cooldown fills more");
    }
}
