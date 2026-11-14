//! The window's tests. Moved out of `larder-world.rs` unchanged (C9,
//! 2026-11-14).

#[test]
fn command_ids_are_one_registry() {
    // Unique and 1..=n, so a clash or a gap fails here, not in play.
    for (i, &(name, id)) in COMMANDS.iter().enumerate() {
        assert_eq!(id as usize, i + 1, "{name} is {id}; command ids run 1, 2, 3, ... in the table");
    }
    // Every building the build page places has exactly one command (the
    // wall is drawn with WALL, a two-click line, not a build_kind).
    use Kind as K;
    let placed = [K::Turret, K::Lure, K::Pump, K::Outpost, K::Siren];
    for kind in placed {
        let ids: Vec<&str> = COMMANDS.iter().filter(|&&(_, id)| build_kind(id) == Some(kind)).map(|&(n, _)| n).collect();
        assert_eq!(ids.len(), 1, "{} has commands {ids:?}", kind.name());
    }
    // ... and exactly one sprite, the sprites running 0, 1, 2, ... (#161).
    let mut sprites: Vec<u32> = placed.iter().map(|&k| sprite_kind(k).unwrap_or_else(|| panic!("{} has no sprite", k.name()))).collect();
    sprites.sort();
    assert_eq!(sprites, (0..placed.len() as u32).collect::<Vec<_>>(), "building sprite kinds clash or skip");
}

use super::*;

/// Every shape on the shapes page (`shape=`) is engine keys the race
/// world takes, and only A is today's 5,000 walkers.
#[test]
fn every_shape_is_keys_the_engine_takes() {
    for name in ["a", "b", "c", "d", "e"] {
        let (walkers, keys) = shape_keys(name).unwrap();
        let mut s = race_settings(1, 2, walkers);
        for (k, v) in keys {
            slime_mould::gpu::set(&mut s, k, v).unwrap_or_else(|e| panic!("shape {name}: {k}={v}: {e}"));
        }
        assert_eq!(walkers == 5_000, name == "a", "{name}");
    }
    assert!(shape_keys("f").is_none());
    assert_eq!((Rules::mass_for(5_000, 2), Rules::mass_for(1_250, 1), Rules::mass_for(25_000, 2)), (1.0, 1.0, 0.2));
}

/// **Shape C's peak against one walker a cell** (tech lead on #183,
/// 2026-11-13): `gpu::crowd_holds` checks only the start walkers, and
/// C now grows from 25,000 to its 50,000 slots within about 5 minutes
/// (shared food). Near one walker an open cell, the one-a-cell rule
/// would refuse births and block steps everywhere, not just inside the
/// body, without a word. So every map the window plays at scale 2 must
/// hold C's slots in at most half its open cells, the bound
/// `crowd_holds` sets on the start. At 2026-11-13 the worst is the
/// noise world, 34.8% (143,604 open cells); the designed maps 32.5-34.4%.
/// Inside the body the rule binds by design: that is C's sheet, and
/// what kept a fed mould's food from showing until it was shared
/// (`Reports/fed-mould-2026-11-13/`). No GPU needed.
#[test]
fn shape_c_at_its_peak_fills_at_most_half_of_any_map() {
    use slime_mould::gpu::agar_cells;
    use slime_mould::mould::big::BigDish;
    let dir = Path::new(env!("CARGO_MANIFEST_DIR")).join("Reports/larder-maps-2026-11-11/maps");
    let (walkers, keys) = shape_keys("c").unwrap();
    let mut s = race_settings(1, 2, walkers);
    for (k, v) in keys {
        slime_mould::gpu::set(&mut s, k, v).unwrap();
    }
    assert_eq!(s.capacity, 50_000, "C's slots");
    let mut worst = (0.0f32, String::new());
    for name in ["noise", "fork", "nest", "rim", "flats", "passes"] {
        let agar = if name == "noise" {
            agar_cells(&s, 2, BigDish::new)
        } else {
            let m = Map::load(&dir, name).unwrap();
            agar_cells(&s, 2, |q| BigDish::with_map(q, &m))
        };
        let full = s.capacity as f32 / agar as f32;
        eprintln!("{name}: {agar} open cells at scale 2, C's 50,000 slots fill {:.1}%", 100.0 * full);
        if full > worst.0 {
            worst = (full, name.to_string());
        }
    }
    assert!(worst.0 <= 0.5, "{}: C's slots fill {:.1}% of its open cells", worst.1, 100.0 * worst.0);
}

/// The default mould is shape C, under the keys the command line gave.
#[test]
fn the_default_is_shape_c_under_given_keys() {
    let mut args = Args { map: Some("fork".into()), seed: 1, scale: 2, walkers: 5000, keys: vec![("upkeep".into(), "0.001".into())], voice: true, mission: None, crater: None, aim: None, core_pull: None, mass: None, own: false, wade: None, sink: None, ration: None, round: 0, bullet: None, squads: false };
    default_shape(&mut args);
    assert_eq!(args.walkers, 25_000);
    let mut s = race_settings(1, 2, args.walkers);
    for (k, v) in &args.keys {
        slime_mould::gpu::set(&mut s, k, v).unwrap();
    }
    assert_eq!(args.keys.first().map(|k| k.0.as_str()), Some("upkeep"));
    assert_eq!(args.keys.last(), Some(&("upkeep".to_string(), "0.001".to_string())));
}

/// The window plays the real game: soldiers sent at the core fire,
/// their shots kill walkers, the window draws each shot's ray, and the
/// engine leaves the look's wound where a shot cut (the squad's `cut`
/// pass; before W2c only the stand-in hits wrote it). Skips with no
/// GPU (CI); the cloud's software Vulkan runs it.
#[test]
fn the_window_plays_the_real_game() {
    let Some((device, queue)) = gpu() else { return };
    let mut app = offscreen(Args { map: Some("fork".into()), seed: 1, scale: 2, walkers: 5000, keys: Vec::new(), voice: true, mission: None, crater: None, aim: None, core_pull: None, mass: None, own: false, wade: None, sink: None, ration: None, round: 0, bullet: None, squads: false }, (640, 400), &device, &queue).unwrap();
    let step = 1.0 / 15.0;
    // Shots fly as bullets (11-13): a tracer for every leg flown, and
    // the legs that were firings are the shots.
    let (mut rays, mut legs, mut fired) = (0, 0, 0);
    while app.game.as_ref().unwrap().time() < 12.0 {
        plan(app.game.as_mut().unwrap(), "rush");
        app.step(step);
        rays += app.tracers.iter().filter(|t| t.age == 0.0).count();
        let g = app.game.as_ref().unwrap();
        legs += g.last_shots.len();
        fired += g.last_shots.iter().filter(|s| s.fired).count() as u32;
    }
    let g = app.game.as_ref().unwrap();
    assert!(g.shots > 0 && g.killed > 0, "shots {} killed {}", g.shots, g.killed);
    assert_eq!(rays, legs, "a tracer drawn for every leg");
    assert_eq!(fired, g.shots, "the firings are the shots");
    let look = read_back(&device, &queue, &g.world.e.look);
    let wounded = look.chunks_exact(4).filter(|c| c[2] > 0).count();
    assert!(wounded > 0, "no wound in the look after {} shots", g.shots);
    app.prepare();
    let m = app.model(None, None);
    // The soldiers' nine, the rally point, the build page's key (Q) and
    // the tools page's (E).
    assert_eq!(m.commands.len(), 12);
    assert!(m.end.is_none());
    // The build page: turret, lure, pump, outpost, the lures' switch, back.
    app.build_page = true;
    let m = app.model(None, None);
    assert_eq!(m.commands.iter().map(|c| c.name.as_str()).collect::<Vec<_>>(), ["Turret", "Lure", "Pump", "Outpost", "Siren", "Lures, sirens on/off", "Back"]);
}

/// **The tools page works in the window** (2026-11-12): soldiers take
/// tools at the larder between steps, the page lists the five, the germ
/// is ready on the GPU world, as is the sleep gas (both since 11-14), a
/// blast thrown near a soldier is paid, spends a held tool and leaves a
/// crater, and a det cord and a sleep gas cloud do too.
#[test]
fn the_window_uses_the_tools() {
    let Some((device, queue)) = gpu() else { return };
    let mut app = offscreen(Args { map: Some("fork".into()), seed: 1, scale: 1, walkers: 1250, keys: Vec::new(), voice: false, mission: None, crater: None, aim: None, core_pull: None, mass: None, own: false, wade: None, sink: None, ration: None, round: 0, bullet: None, squads: false }, (320, 200), &device, &queue).unwrap();
    for _ in 0..10 {
        app.step(1.0 / 15.0);
    }
    let held: u32 = (0..app.game.as_ref().unwrap().soldiers.len()).map(|k| app.kit.held(k)).sum();
    assert!(held > 0, "soldiers took tools at the larder");
    app.tools_page = true;
    let m = app.model(None, None);
    assert_eq!(m.commands.iter().map(|c| c.name.as_str()).collect::<Vec<_>>(), ["Blast", "Det cord", "Mine", "Germ", "Sleep gas", "Back"]);
    assert!(m.commands[0].ready && m.commands[3].ready && m.commands[4].ready, "blast, germ and gas ready on the GPU");
    let g = app.game.as_mut().unwrap();
    g.bank = 10_000.0;
    let s = g.soldiers[0];
    let k = g.world.scale();
    app.use_tool(Tool::Blast((s.x + 10.0 * k, s.y)));
    app.use_tool(Tool::Cord((s.x - 10.0 * k, s.y + 8.0 * k), (s.x + 10.0 * k, s.y + 8.0 * k)));
    app.use_tool(Tool::Gas((s.x, s.y - 10.0 * k)));
    let g = app.game.as_ref().unwrap();
    assert_eq!((app.kit.uses[0], app.kit.uses[2], app.kit.uses[4]), (1, 1, 1), "the blast, the cord and the gas were used");
    assert!(g.bank < 10_000.0, "and paid");
    let cratered = g.world.e.crater_of().iter().filter(|&&c| c > 0.0).count();
    assert!(cratered > 0, "the blast left a crater");
    assert_eq!((0..g.soldiers.len()).map(|k| app.kit.held(k)).sum::<u32>(), held - 3, "three held tools spent");
}

/// A campaign mission plays its own map and rules, shows its goal as the
/// objective, and ends on its own screen when the squad is lost (a rush
/// into the Flats' mould dies by 0:31 on seed 1).
#[test]
fn a_mission_shows_its_goal_and_its_end() {
    let Some((device, queue)) = gpu() else { return };
    let mut app = offscreen(Args { map: None, seed: 1, scale: 2, walkers: 5000, keys: Vec::new(), voice: false, mission: Some("1-landfall".into()), crater: None, aim: None, core_pull: None, mass: None, own: false, wade: None, sink: None, ration: None, round: 0, bullet: None, squads: false }, (640, 400), &device, &queue).unwrap();
    assert_eq!(app.name, "flats");
    assert_eq!(app.game.as_ref().unwrap().bank, 0.0, "the mission's start_bank");
    app.prepare();
    assert!(app.model(None, None).objective.is_some_and(|o| o.starts_with("Grow your squad to")));
    // An idle squad meets nothing by a 1:00 cap. (A rush on the core was
    // lost by 2:00 until the mission's reserve recruit, Larder PR #106:
    // its squad now keeps coming back, and an idle one fires on the
    // mould that reaches it, so neither ends early.)
    app.game.as_mut().unwrap().rules.cap = 60.0;
    let step = 1.0 / 15.0;
    while app.mission.as_ref().unwrap().verdict.is_none() && app.game.as_ref().unwrap().time() < 120.0 {
        app.step(step);
    }
    // Since the Drifter is a real tier (the campaign tiers PR, #115),
    // Landfall's mould is slower and fewer and never reaches the idle
    // squad, which then has no food, no shot and an empty bank: Stuck
    // ends it before the cap. Either is the lost mission this tests.
    let v = app.mission.as_ref().unwrap().verdict.expect("lost by the 1:00 cap");
    use slime_mould::mould::campaign::Cause;
    assert!(matches!(v.cause, Cause::Cap | Cause::Stuck), "{v:?}");
    assert_eq!(v.stars, 0);
    app.prepare();
    assert_eq!(app.model(None, None).end.map(|e| e.0).as_deref(), Some("Mission lost"));
}

/// A GPU for the smoke tests, which skip without one, except in CI:
/// there `MOULDGPU_TEST_GPU=1` (Mesa's software Vulkan), and a missing
/// adapter fails them rather than letting them pass unrun.
fn smoke_gpu() -> Option<(wgpu::Device, wgpu::Queue)> {
    let g = gpu();
    assert!(g.is_some() || std::env::var("MOULDGPU_TEST_GPU").map_or(true, |v| v != "1"), "MOULDGPU_TEST_GPU=1 but no GPU adapter");
    g
}

/// One smoke round: 60 game seconds through the bench, which does each
/// frame what the window does (the steps through its accumulator, left
/// running while it draws; the sound on tape; the HUD, marks, view and
/// canvas drawn), at 24 frames a second so some steps finish when ready
/// and the rest when the next is due, then the end screen. Scale 1 (a
/// 256 x 160 dish, 1,250 walkers): 5-9 s a round on software Vulkan,
/// against 20-24 s at scale 2 (2026-11-12).
fn smoke(device: &wgpu::Device, queue: &wgpu::Queue, mission: Option<&str>, hand: &str, fast: bool) -> String {
    let args = Args { map: mission.is_none().then(|| "fork".into()), seed: 1, scale: 1, walkers: 1250, keys: Vec::new(), voice: true, mission: mission.map(String::from), crater: None, aim: None, core_pull: None, mass: None, own: false, wade: None, sink: None, ration: None, round: 0, bullet: None, squads: false };
    let b = Bench { secs: 60.0, hand: hand.into(), fps: 24.0, screen: (320, 200), stamps: false, grid: 0, sound: Some(String::new()), fast, passes: 0, settle: false, squad1: false, spread: true, view: None };
    let t0 = Instant::now();
    let last = bench(args, &b, device, queue).unwrap_or_else(|e| panic!("{mission:?} {hand}: {e}"));
    eprintln!("smoke {mission:?} {hand}{}: {last} ({:.1} s)", if fast { " fast" } else { "" }, t0.elapsed().as_secs_f32());
    last
}

/// **The window's loop does not panic on any mission** (CI smoke test,
/// 2026-11-12: the integration thread and the technical lead asked for
/// it after #114's window panic, a reserve recruit with no note, reached
/// main, as nothing in CI ran the window's own loop). Every mission file
/// plays 60 game seconds with a player AI style, a different one each
/// (`larder_world_ai`), every other one in fast forward, so frames hold
/// two or three steps (where the recruit's panic lived).
#[test]
fn the_window_plays_every_mission() {
    let Some((device, queue)) = smoke_gpu() else { return };
    let dir = Path::new(env!("CARGO_MANIFEST_DIR")).join("Reports/larder-campaign-2026-11-12/missions");
    let mut names: Vec<String> = std::fs::read_dir(&dir).unwrap().filter_map(|e| e.ok()?.file_name().to_str()?.strip_suffix(".txt").map(String::from)).collect();
    names.sort();
    assert!(names.len() >= 3, "the missions in {}: {names:?}", dir.display());
    let styles = ["boom", "claim", "strike", "fort", "late"];
    for (i, name) in names.iter().enumerate() {
        let last = smoke(&device, &queue, Some(name), styles[i % styles.len()], i % 2 == 1);
        assert!(last.contains("t "), "{name}: {last}");
    }
}

/// **Every command on the card, pressed every game second, does not
/// panic the window** (the same smoke test's other half): `hand=every`
/// presses each ready command through the window's own `hud_event`, and
/// the unready ones as the HUD sends them, with the squad selected and
/// aimed in turn at the squad, a crystal, the core and the larder. A
/// command a thread adds to the card is pressed with the rest. The
/// squad fires (free fire is on the card), and the round's sound is on
/// tape.
#[test]
fn the_window_takes_every_command() {
    let Some((device, queue)) = smoke_gpu() else { return };
    let last = smoke(&device, &queue, None, "every", false);
    assert!(!last.contains("shots 0,"), "the pressed squad fired: {last}");
    // The card's Build page is pressed open and shut in turn, so its
    // buildings are pressed on alternate seconds (#112's buildings).
    assert!(!last.contains("buildings 0,"), "the pressed squad built: {last}");
}

/// **The player's own mould in the window** (`own=1`, 2026-11-12):
/// every command on the card, Sow and Road with the rest, pressed every
/// game second for 60 game seconds, does not panic it, and the garden's
/// t line reports both moulds.
#[test]
fn the_window_plays_the_players_own_mould() {
    let Some((device, queue)) = smoke_gpu() else { return };
    let args = Args { map: None, seed: 1, scale: 1, walkers: 625, keys: Vec::new(), voice: false, mission: None, crater: None, aim: None, core_pull: None, mass: None, own: true, wade: None, sink: None, ration: None, round: 0, bullet: None, squads: false };
    let b = Bench { secs: 60.0, hand: "every".into(), fps: 24.0, screen: (320, 200), stamps: false, grid: 0, sound: Some(String::new()), fast: false, passes: 0, settle: false, squad1: false, spread: true, view: None };
    let last = bench(args, &b, &device, &queue).unwrap_or_else(|e| panic!("own=1: {e}"));
    assert!(last.contains("t "), "{last}");
    let app = offscreen(Args { map: None, seed: 1, scale: 1, walkers: 625, keys: Vec::new(), voice: false, mission: None, crater: None, aim: None, core_pull: None, mass: None, own: true, wade: None, sink: None, ration: None, round: 0, bullet: None, squads: false }, (320, 200), &device, &queue).unwrap();
    let g = app.game.as_ref().unwrap();
    assert_eq!(g.world.counts_by_side(), (625, 625));
    assert_eq!(g.mould, 625, "the game's mould is the enemy's");
}

/// A recruit's first shot in the same frame it joined (fast forward,
/// or a slow frame holding several steps): its tracer and flash need a
/// note, which `shot_marks` now makes itself.
#[test]
fn a_shot_from_a_soldier_with_no_note_yet_draws() {
    let Some((device, queue)) = gpu() else { return };
    let mut app = offscreen(Args { map: Some("fork".into()), seed: 1, scale: 1, walkers: 1250, keys: Vec::new(), voice: false, mission: None, crater: None, aim: None, core_pull: None, mass: None, own: false, wade: None, sink: None, ration: None, round: 0, bullet: None, squads: false }, (320, 200), &device, &queue).unwrap();
    let g = app.game.as_mut().unwrap();
    let s = *g.soldiers.last().unwrap();
    g.last_shots = vec![slime_mould::mould::larder_world::Shot { from: (s.x, s.y), to: (s.x + 5.0, s.y), hit: None, killed: 0, by: Some(g.soldiers.len() - 1), fired: true }];
    app.looks.clear();
    shot_marks(app.game.as_ref().unwrap(), &mut app.tracers, &mut app.looks, None);
    assert_eq!(app.looks.len(), app.game.as_ref().unwrap().soldiers.len());
    assert!(app.looks.last().unwrap().flash > 0.0, "the shot's flash on its soldier");
}

/// The start menu's Sandbox, untouched, plays the round the bare command
/// line plays (shape C on Fork), and its options only add what was moved.
#[test]
fn the_menu_plays_the_command_lines_rounds() {
    use slime_mould::world::start::StartEvent;
    let bare = Args { map: Some("fork".into()), seed: 1, scale: 2, walkers: 5000, keys: Vec::new(), voice: true, mission: None, crater: None, aim: None, core_pull: None, mass: None, own: false, wade: None, sink: None, ration: None, round: 0, bullet: None, squads: false };
    let mut cli = bare.clone();
    default_shape(&mut cli);
    let mut menu = start_menu(&bare);
    assert!(menu.options.iter().all(|o| o.is_default()), "a bare command line presets nothing");
    let a = round_args(&bare, false, &menu, &StartEvent::Sandbox);
    assert_eq!((a.map.as_deref(), a.walkers, &a.keys, a.round, a.bullet, a.own), (cli.map.as_deref(), cli.walkers, &cli.keys, 0, None, false));
    // A mission keeps its own mould: no shape C keys, no map.
    assert!(menu.missions.len() >= 3, "the campaign's missions are listed");
    assert_eq!(menu.missions[0].stem, "1-landfall");
    let m = round_args(&bare, false, &menu, &StartEvent::Mission(1));
    assert_eq!((m.mission.as_deref(), m.walkers, m.keys.len()), (Some(menu.missions[1].stem.as_str()), 5000, 0));
    // Options moved: they land as the command line's keys would.
    for o in &mut menu.options {
        o.at = o.choices.iter().position(|c| c == match o.key.as_str() { "panic" => "0", "bullet" => "0", "map" => "rim", "rounds" => "split", _ => o.value() }).unwrap();
    }
    let a = round_args(&bare, false, &menu, &StartEvent::Sandbox);
    assert_eq!((a.map.as_deref(), a.bullet, LOADOUTS[a.round].0), (Some("rim"), Some(0.0), "split"));
    assert_eq!(a.keys.last(), Some(&("panic".to_string(), "0".to_string())));
    let m = round_args(&bare, false, &menu, &StartEvent::Mission(0));
    assert_eq!(m.keys, vec![("panic".to_string(), "0".to_string())]);
    // A command line's own value is the preset, and the page's value wins.
    let mut given = bare.clone();
    given.keys.push(("wither".into(), "0".into()));
    let mut menu = start_menu(&given);
    assert_eq!(menu.changed("wither"), Some("0"));
    let w = menu.options.iter_mut().find(|o| o.key == "wither").unwrap();
    w.at = w.default;
    let a = round_args(&given, false, &menu, &StartEvent::Sandbox);
    assert_eq!(a.keys.iter().filter(|k| k.0 == "wither").map(|k| k.1.as_str()).collect::<Vec<_>>(), vec!["0.01"], "reset to C's own");
}

#[test]
fn the_loadout_panel_lights_the_next_cast_and_dims_dead_cards() {
    let r = RoundRules::default();
    let cards = |s: &str, next| gun_cards(&rounds::parse(s).unwrap(), &r, 3, next, |c| c != rounds::Card::Germ);
    let lit = |v: &[GunCard]| v.iter().map(|c| c.next).collect::<Vec<_>>();
    // Two casts: quiet+gas, then quiet+blast; the second is next.
    let lull = cards("quiet+gas+quiet+blast", Some(1));
    assert_eq!(lit(&lull), [false, false, true, true]);
    assert_eq!(lull.iter().map(|c| c.kind).collect::<Vec<_>>(), [CardKind::Modifier, CardKind::Payload, CardKind::Modifier, CardKind::Payload]);
    // A modifier takes its payload's colour; the payload shows its price.
    assert_eq!(lull[0].colour, lull[1].colour);
    assert_ne!(lull[0].colour, lull[3].colour);
    assert_eq!(lull[3].cost.as_deref(), Some(format!("{:.0}", r.kit.blast_price).as_str()));
    // A card after the last payload casts nothing; flight cards ride every
    // bullet, never lit as a cast and never idle.
    let tail = cards("blast+quiet", Some(0));
    assert_eq!((tail[0].next, tail[1].idle, tail[1].next), (true, true, false));
    let lance = cards("pierce+split+blast", Some(0));
    assert_eq!(lance.iter().map(|c| (c.next, c.idle)).collect::<Vec<_>>(), [(false, false), (false, false), (true, false)]);
    // A payload this world cannot cast says so.
    assert!(cards("germ", Some(0))[0].about.contains("Not on this world"));
    // Every loadout H cycles through draws, each card with a glyph and words.
    for (name, l) in LOADOUTS {
        for c in cards(l, Some(0)) {
            assert!(c.icon.is_some() && !c.about.is_empty() && !c.name.is_empty(), "{name}");
        }
    }
}

/// The opening view (the art director, 11-14: "the camera opens with the
/// squad under it"): the larder is in the middle of the band between the
/// top bar and the bottom panels, at the window's sizes.
#[test]
fn the_opening_camera_frames_the_larder_above_the_bottom_panels() {
    let Some((device, queue)) = gpu() else { return };
    for screen in [(1280, 800), (1920, 1080)] {
        let app = offscreen(Args { map: Some("fork".into()), seed: 1, scale: 1, walkers: 1250, keys: Vec::new(), voice: false, mission: None, crater: None, aim: None, core_pull: None, mass: None, own: false, wade: None, sink: None, ration: None, round: 0, bullet: None, squads: false }, screen, &device, &queue).unwrap();
        let g = app.game.as_ref().unwrap();
        let (top, bottom) = Hud::insets(screen, app.canvas.as_ref().unwrap().ui);
        let y = app.camera.to_screen(g.larder).1;
        assert!(y > top + 40.0 && y < screen.1 as f32 - bottom - 40.0, "{screen:?}: the larder at y {y:.0}, the band {top:.0}..{:.0}", screen.1 as f32 - bottom);
    }
}

/// Turning the sandbox map (or the own mould) on the start menu starts the
/// dish behind it again on that map (owner, 11-14 05:04Z); turning another
/// option, or moving between rows, leaves the dish playing. Skips with no
/// GPU (CI); the cloud's software Vulkan runs it.
#[test]
fn turning_the_map_on_the_menu_switches_the_dish_behind() {
    use slime_mould::world::start::Page;
    let Some((device, queue)) = gpu() else { return };
    let base = Args { map: Some("fork".into()), seed: 1, scale: 2, walkers: 5000, keys: Vec::new(), voice: true, mission: None, crater: None, aim: None, core_pull: None, mass: None, own: false, wade: None, sink: None, ration: None, round: 0, bullet: None, squads: false };
    let mut app = offscreen(base.clone(), (640, 400), &device, &queue).unwrap();
    app.base = Some((base.clone(), true));
    app.menu = Some(start_menu(&base));
    app.open_menu().unwrap();
    app.menu.as_mut().unwrap().show(Page::Options);
    // Down to "Your own mould" and back: no restart (the round's clock runs on).
    for _ in 0..3 {
        app.game.as_mut().unwrap().step();
    }
    let t = app.game.as_ref().unwrap().time();
    app.menu_input(|m| m.key(KeyCode::ArrowDown, true)).unwrap();
    app.menu_input(|m| m.key(KeyCode::ArrowUp, true)).unwrap();
    assert_eq!(app.game.as_ref().unwrap().time(), t, "moving between rows kept the dish");
    // The map row: Right turns Fork to the next map, and the dish is on it.
    let picks = app.menu_input(|m| m.key(KeyCode::ArrowRight, true)).unwrap();
    assert!(picks.is_empty());
    let next = app.menu.as_ref().unwrap().option("map").unwrap().value().to_string();
    assert_ne!(next, "fork");
    assert_eq!((app.args.map.as_deref(), app.name.as_str()), (Some(next.as_str()), next.as_str()));
    assert_eq!(app.game.as_ref().unwrap().time(), 0.0, "a fresh round");
    assert!(app.in_menu(), "still on the menu");
}

fn squad_args(squads: bool) -> Args {
    Args { map: Some("fork".into()), seed: 1, scale: 1, walkers: 1250, keys: Vec::new(), voice: false, mission: None, crater: None, aim: None, core_pull: None, mass: None, own: false, wade: None, sink: None, ration: None, round: 0, bullet: None, squads }
}

/// **A step a command finishes keeps its shots** (bug sweep, 2026-11-14,
/// the owner's "the gun animation gets messed up sometimes"): `Game::apply`
/// finishes the step running on the GPU first, and the window never saw
/// that step, so its bullets' legs, flashes and casts were lost. In the
/// squad mode the cursor sent such a command almost every frame.
#[test]
fn a_step_a_command_finishes_keeps_its_shots() {
    let Some((device, queue)) = gpu() else { return };
    let mut app = offscreen(squad_args(false), (320, 200), &device, &queue).unwrap();
    let (mut checked, mut legs) = (0, 0);
    while app.game.as_ref().unwrap().time() < 30.0 && checked < 20 {
        plan(app.game.as_mut().unwrap(), "rush");
        app.step(1.0 / 15.0);
        let g = app.game.as_mut().unwrap();
        if !g.in_flight() {
            g.begin_step();
        }
        // A command lands while the step runs, as a click does.
        g.apply(Cmd::Stance(0, Stance::Free));
        let n = g.last_shots.len();
        let before = app.tracers.len();
        app.step(0.0);
        let drawn = app.tracers.len() - before;
        assert_eq!(drawn, n, "a tracer for every leg of the step the command finished");
        if n > 0 {
            checked += 1;
            legs += n;
        }
    }
    assert!(checked >= 5, "only {checked} steps with shots ({legs} legs)");
}

/// **The squad mode's keys** (bug sweep, 2026-11-14, the owner's playtest:
/// "The shortcuts for harvesting and maybe other things doesn't work", "I
/// cannot zoom in anymore"): the card has no letter on WASD and its G
/// drains a crystal as the squad; Y splits and J joins (they were X and G,
/// over the card's Defend and Harvest); the plain wheel zooms and keeps
/// following the squad, Ctrl + wheel switches squads.
#[test]
fn the_squad_modes_keys_do_what_the_card_says() {
    let Some((device, queue)) = gpu() else { return };
    let mut app = offscreen(squad_args(true), (640, 400), &device, &queue).unwrap();
    app.game.as_mut().unwrap().bank = 1e6;
    for _ in 0..4 {
        app.game.as_mut().unwrap().apply(Cmd::Buy);
    }
    app.squad_frame(1.0 / 60.0);
    app.prepare();
    let m = app.model(None, None);
    let wasd = [KeyCode::KeyW, KeyCode::KeyA, KeyCode::KeyS, KeyCode::KeyD];
    for page in [(false, false), (true, false), (false, true)] {
        (app.build_page, app.tools_page) = page;
        let m = app.model(None, None);
        for c in &m.commands {
            assert!(!wasd.contains(&c.key), "{} on {:?} steers instead", c.name, c.key);
        }
    }
    (app.build_page, app.tools_page) = (false, false);
    let key_of = |name: &str| m.commands.iter().find(|c| c.name == name).map(|c| c.key);
    assert_eq!(key_of("Drain a crystal"), Some(KeyCode::KeyG));
    assert_eq!(key_of("Attack-move"), Some(KeyCode::KeyR));
    assert_eq!(key_of("Defend"), Some(KeyCode::KeyX));
    assert!(m.help_base.as_ref().is_some_and(|h| h.iter().any(|r| r.1.starts_with("Split"))), "F1 lists the squad's keys");
    // G is the card's: it aims a drain, and merges nothing.
    assert!(!app.squad_key(KeyCode::KeyG, true, false, false));
    let (took, _) = app.hud.key(KeyCode::KeyG, true, false, &mut app.camera);
    assert!(took && app.hud.targeting().is_some_and(|t| t.0 == HARVEST), "G aims a drain");
    app.hud.cancel_targeting();
    // Y splits the controlled squad.
    let n = app.game.as_ref().unwrap().live_squads().len();
    assert!(app.squad_key(squadmode::SPLIT, true, false, false));
    assert_eq!(app.game.as_ref().unwrap().live_squads().len(), n + 1, "Y split the squad");
    // The wheel zooms (about the squad, still following it); Ctrl + wheel
    // switches squads.
    let (z, active) = (app.camera.zoom, app.game.as_ref().unwrap().active);
    app.ctrl = false;
    assert!(!app.squad_wheel(1));
    assert!(app.squad_zoom(1));
    assert!(app.camera.zoom > z, "the wheel zoomed in: {z} -> {}", app.camera.zoom);
    assert!(app.sq.as_ref().unwrap().follow, "and still follows the squad");
    assert_eq!(app.game.as_ref().unwrap().active, active);
    app.ctrl = true;
    assert!(app.squad_wheel(1));
    assert_ne!(app.game.as_ref().unwrap().active, active, "Ctrl + wheel switched squads");
}

#[test]
fn the_squad_cameras_follow_keeps_the_squad_in_the_middle() {
    // The bug sweep (2026-11-14): the follow eased from the screen's middle
    // but `centre_on` puts a point in the middle of the band between the
    // UI's insets, so each frame it fell short and the camera came to rest
    // about 100 cells off the squad (clamped at the map's edge), and every
    // zoom in lost it. Steered right for 5 s, zoomed in twice on the way,
    // the squad's middle must stay within 3 cells of the band's middle.
    let Some((device, queue)) = gpu() else { return };
    let mut app = offscreen(squad_args(true), (640, 400), &device, &queue).unwrap();
    let dt = 1.0 / 60.0;
    let mut worst: f32 = 0.0;
    app.squad_key(KeyCode::KeyD, true, false, true);
    for i in 0..300 {
        if i == 100 || i == 200 {
            assert!(app.squad_zoom(1));
        }
        app.squad_frame(dt);
        app.step(dt);
        let g = app.game.as_ref().unwrap();
        let m = g.active.and_then(|q| g.middle(q)).unwrap();
        let f = app.camera.focus();
        if i >= 10 {
            worst = worst.max((m.0 - f.0).hypot(m.1 - f.1));
        }
        assert!(app.sq.as_ref().unwrap().follow, "frame {i}: the camera stopped following");
    }
    app.squad_key(KeyCode::KeyD, false, false, true);
    let (sx, sy) = app.camera.to_screen(app.game.as_ref().map(|g| g.active.and_then(|q| g.middle(q)).unwrap()).unwrap());
    eprintln!("follow: worst {worst:.2} cells off; squad on screen at ({sx:.0}, {sy:.0}), zoom {}", app.camera.zoom);
    assert!(worst < 3.0, "the camera fell {worst:.1} cells behind the squad");
    assert!((0.0..640.0).contains(&sx) && (0.0..400.0).contains(&sy), "the squad left the screen");
}

/// **No two commands on a page share a slot or a key** (bug sweep,
/// 2026-11-14: the Rally point sat at (2, 2) under the Build button, so
/// only its key reached it). Every page, in the RTS round and the squad
/// mode, with and without the player's own mould.
#[test]
fn every_command_card_page_has_its_own_slots_and_keys() {
    let Some((device, queue)) = gpu() else { return };
    for (squads, own) in [(false, false), (false, true), (true, false), (true, true)] {
        let mut args = squad_args(squads);
        if own {
            (args.own, args.map) = (true, None);
        }
        let mut app = offscreen(args, (640, 400), &device, &queue).unwrap();
        for (build, tools) in [(false, false), (true, false), (false, true)] {
            (app.build_page, app.tools_page) = (build, tools);
            let cmds = app.model(None, None).commands;
            let page = format!("squads {squads}, own {own}, build {build}, tools {tools}");
            for (i, a) in cmds.iter().enumerate() {
                for b in &cmds[i + 1..] {
                    assert_ne!(a.slot, b.slot, "{page}: {} and {} share a slot", a.name, b.name);
                    assert_ne!(a.key, b.key, "{page}: {} and {} share a key", a.name, b.name);
                }
            }
        }
    }
}

/// **The card's burn reaches a moving squad** (review after the bug sweep,
/// 2026-11-14): given soldier by soldier, a squad steered with WASD took
/// its own order back at its next step, so F + drag did nothing.
#[test]
fn the_cards_burn_reaches_a_moving_squad() {
    let Some((device, queue)) = gpu() else { return };
    let mut app = offscreen(squad_args(true), (640, 400), &device, &queue).unwrap();
    let dt = 1.0 / 60.0;
    app.squad_key(KeyCode::KeyD, true, false, true);
    for _ in 0..30 {
        app.squad_frame(dt);
        app.step(dt);
    }
    let (from, to) = {
        let g = app.game.as_ref().unwrap();
        let m = g.active.and_then(|q| g.middle(q)).unwrap();
        let sc = g.world.scale();
        ((m.0 + 4.0 * sc, m.1 - 6.0 * sc), (m.0 + 4.0 * sc, m.1 + 6.0 * sc))
    };
    app.hud_event(HudEvent::Line { id: BURN, from, to }).unwrap();
    for _ in 0..30 {
        app.squad_frame(dt);
        app.step(dt);
    }
    let g = app.game.as_ref().unwrap();
    let q = g.active.unwrap();
    let burning = g.members(q).iter().filter(|&&k| matches!(g.soldiers[k].order, Order::Burn(..))).count();
    assert_eq!(burning, g.members(q).len(), "every member of the squad burns the line half a second later");
}
