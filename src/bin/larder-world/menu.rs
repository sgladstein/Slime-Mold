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
use slime_mould::mould::bullets::{Set, Shell};
use slime_mould::world::start::{Entry, Opt, Page, Shop, ShopArea, ShopRow, StartEvent, StartMenu, BAG};

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
        Opt::new("map", "Sandbox map", &maps, "fork", &map_now, "The sandbox's map (noise: the censuses' race world)"),
        Opt::new("planet", "Planet", &["1", "2", "3", "4", "5", "6", "7", "8"], "1", &planet_now(base), "Which planet of a run Random planet lands on: its biome and nests (8 is the core world)"),
        Opt::new("own", "Your own mould", &["off", "on"], "off", if base.own { "on" } else { "off" }, "A green mould the squad sows and feeds; sandbox on the race world"),
        Opt::new("mode", "Control", &["squads", "rts"], "squads", if base.squads { "squads" } else { "rts" }, "Squads: steer one squad with WASD, fire at the cursor, split and merge; no larder (guard or drain crystals)"),
        Opt::new("rounds", "Rounds", &loadouts, LOADOUTS[0].0, LOADOUTS[base.round].0, "The squad's loadout at the start (H cycles it in play)"),
        Opt::new("bullet", "Bullet speed", &["0", "30", "45", "60", "90"], &num(bullet), &base.bullet.map_or(num(bullet), num), "Dish cells a game second a shot flies; 0 is the instant ray"),
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
    // The UI's size (owner, 11-14 08:07Z): turned here it takes effect at
    // once and is saved; not a round's setting, so `round_args` skips it.
    let sizes: Vec<String> = settings::STEPS.iter().map(|&v| settings::percent(v)).collect();
    let sizes: Vec<&str> = sizes.iter().map(String::as_str).collect();
    // First on the page (the review, 11-15: it was the last of 15 rows,
    // below the fold, and the one the owner needed).
    o.insert(0, Opt::new("ui", "UI and text size", &sizes, "100%", &settings::percent(ui_user()), "Every panel, card, tooltip and this menu, over the screen's own scale; Ctrl + and Ctrl - anywhere"));
    // The page's tabs (11-15, the owner's "Having to scroll is horrible"):
    // the game's own settings, the planet's, and the mould's rules, each
    // tab one screen; the key list is the last tab. Tabs come in the
    // order their first setting does.
    o.into_iter()
        .map(|opt| {
            let tab = match opt.key.as_str() {
                "map" | "planet" | "own" | "plants" => "Planet",
                "panic" | "wither" | "energy_flow" | "roam" | "surge" | "flinch" => "The mould",
                _ => "Game",
            };
            opt.in_tab(tab)
        })
        .collect()
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
        ("Wheel, middle drag", "Zoom, pan; arrows, the screen edge and WASD while paused pan"),
        ("A, click", "Attack-move; on the core, attack it"),
        ("S", "Stop"),
        ("G", "Harvest"),
        ("F, click", "Guard a spot, building or crystal: keep slime off it"),
        ("Z / X / C", "Hold fire, defend, free fire (the fire button cycles them)"),
        ("B", "Buy a soldier (or select the larder)"),
        ("U, click", "Place a pump on a crystal (again on one: upgrade it)"),
        ("R", "The larder's rally point"),
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
        ("F2", "Scent view: what the mould smells"),
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
    squad.push((format!("{}, {}, {}", k(ATTACK, KeyCode::KeyA), k(HARVEST, KeyCode::KeyG), k(GUARD, KeyCode::KeyF)), "The card for the squad: attack-move, drain, guard".into()));
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
    let mut lines = vec![format!("{}. Next: {}, a {} planet; the slime is {}, {:.1} times the first planet's walkers", r.line(), p.name, p.biome.name(), title_case(l.tier.name()), l.walkers)];
    // What this tier does to you (`run::does_to_you`).
    let does = run::does_to_you(l.tier).says;
    if !does.is_empty() {
        lines[0] += &format!(". {does}");
    }
    // The level's clock and head start (`run::clock`, `run::head_start`).
    let cap = l.mission.rules(Rules::default()).cap;
    let secs = cap.round() as u32;
    let mut clock = format!("{}:{:02} on the clock", secs / 60, secs % 60);
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
    let gun = if r.editing_turrets { r.rack() } else { r.armed() };
    let rows = r
        .shop()
        .into_iter()
        .map(|row| {
            let (group, area) = match row.group {
                // The range: its own rows on the left, the shelf by what a
                // card works on, then the bag.
                Group::Dev if r.range => (String::new(), ShopArea::Upgrades),
                // The gun slots' switch to the turrets' deck: a bottom-bar
                // button (`Pick::EditTurrets`).
                _ if row.pick == Pick::EditTurrets => (String::new(), ShopArea::Upgrades),
                Group::Cards if r.range => (range_shelf(&row).to_string(), ShopArea::Cards),
                Group::Squad | Group::Cards if matches!(row.face, Face::Item(_)) => ("Squad".to_string(), ShopArea::Upgrades),
                // Each a tab of the shop's counter (Shop B, 11-15).
                Group::Build | Group::Tools | Group::Dev => (row.group.name().to_string(), ShopArea::Upgrades),
                Group::Guns => ("Guns".to_string(), ShopArea::Guns),
                _ => (if row.price.is_empty() { BAG } else { "Cards" }.to_string(), ShopArea::Cards),
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
                    // Its kind and words, the run's note after them (too
                    // big for the gun), its budget last (owner 11-15
                    // 02:57Z: the cards' text "should be simplified").
                    s.about = format!("{} Budget {}.", row.about.replacen(c.about(), &draw::card_words(c, kind, &words), 1), c.cost());
                    if let rounds::Card::Ammo(a) = c {
                        // A rolled bullet: its set as a badge.
                        s.badge = Some(draw::ammo_icon(a.set));
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
        turrets: r.editing_turrets,
    }
}

/// The range's heading for a card row: the shelf's by what the card
/// works on, or the bag.
fn range_shelf(row: &run::Row) -> &'static str {
    match (row.pick, &row.face) {
        // The shove cards their own shelf (2026-11-16): with them on the
        // next bullet's, that tab took two pages at 200% on 1080 lines.
        (Pick::Offer(_), Face::Card(rounds::Card::Heave | rounds::Card::Corral | rounds::Card::Pull)) => "Cards that shove",
        // The Mind bullets too (2026-11-16): three more rows of the
        // bullets' grid put that tab on three pages at 200% on 1080 lines.
        // The readers (Shatter, Detonate) share it: their own tab would
        // have been the armory's seventh for two rows; so do the flare and
        // the torch (2026-11-17), the route and element families' payloads.
        (Pick::Offer(_), Face::Card(rounds::Card::Ammo(a))) if a.shell.special() => "Special bullets",
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
        "strike" => Icon::Attack,
        "rate" => Icon::Fast,
        "range" => Icon::Eye,
        "squad" => Icon::Recruit,
        "bank" => Icon::Crystal,
        "harvest" => Icon::Upgrade,
        "walls" => Icon::Wall,
        "turret" => Icon::Turret,
        "mortar" => Icon::Bomb,
        "lure" => Icon::Bait,
        "pump+" => Icon::Crystal,
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
    // (`shop-mid@TAB@NAME`: a tab and a pick, `headless`.)
    let name = name.split('@').next().unwrap_or(name);
    Some(match name {
        "1" | "main" => Page::Main,
        "campaign" | "missions" => Page::Missions,
        "options" | "options-planet" | "options-mould" => Page::Options,
        "keys" => Page::Keys,
        "shop" | "shop-mid" | "shop-build" | "shop-tools" | "shop-turrets" => Page::Shop,
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
            StartEvent::Range => self.show_armory(),
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
        self.menu = Some(self.parked.take().unwrap_or_else(|| start_menu(&base)));
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
        self.rounds = Rounds::new(r.squad_cards(), RoundRules { budget: r.budget(), ..RoundRules::default() });
        if let Some(g) = self.game.as_mut() {
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
        self.set_armory(false);
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
