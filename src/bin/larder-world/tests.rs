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
    let mut args = Args { map: Some("fork".into()), seed: 1, scale: 2, walkers: 5000, keys: vec![("upkeep".into(), "0.001".into())], voice: true, mission: None, crater: None, aim: None, core_pull: None, mass: None, own: false, wade: None, sink: None, round: 0, bullet: None };
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
    let mut app = offscreen(Args { map: Some("fork".into()), seed: 1, scale: 2, walkers: 5000, keys: Vec::new(), voice: true, mission: None, crater: None, aim: None, core_pull: None, mass: None, own: false, wade: None, sink: None, round: 0, bullet: None }, (640, 400), &device, &queue).unwrap();
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
    let mut app = offscreen(Args { map: Some("fork".into()), seed: 1, scale: 1, walkers: 1250, keys: Vec::new(), voice: false, mission: None, crater: None, aim: None, core_pull: None, mass: None, own: false, wade: None, sink: None, round: 0, bullet: None }, (320, 200), &device, &queue).unwrap();
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
    let mut app = offscreen(Args { map: None, seed: 1, scale: 2, walkers: 5000, keys: Vec::new(), voice: false, mission: Some("1-landfall".into()), crater: None, aim: None, core_pull: None, mass: None, own: false, wade: None, sink: None, round: 0, bullet: None }, (640, 400), &device, &queue).unwrap();
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
    let args = Args { map: mission.is_none().then(|| "fork".into()), seed: 1, scale: 1, walkers: 1250, keys: Vec::new(), voice: true, mission: mission.map(String::from), crater: None, aim: None, core_pull: None, mass: None, own: false, wade: None, sink: None, round: 0, bullet: None };
    let b = Bench { secs: 60.0, hand: hand.into(), fps: 24.0, screen: (320, 200), stamps: false, grid: 0, sound: Some(String::new()), fast, passes: 0, settle: false, squad1: false, spread: true };
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
    let args = Args { map: None, seed: 1, scale: 1, walkers: 625, keys: Vec::new(), voice: false, mission: None, crater: None, aim: None, core_pull: None, mass: None, own: true, wade: None, sink: None, round: 0, bullet: None };
    let b = Bench { secs: 60.0, hand: "every".into(), fps: 24.0, screen: (320, 200), stamps: false, grid: 0, sound: Some(String::new()), fast: false, passes: 0, settle: false, squad1: false, spread: true };
    let last = bench(args, &b, &device, &queue).unwrap_or_else(|e| panic!("own=1: {e}"));
    assert!(last.contains("t "), "{last}");
    let app = offscreen(Args { map: None, seed: 1, scale: 1, walkers: 625, keys: Vec::new(), voice: false, mission: None, crater: None, aim: None, core_pull: None, mass: None, own: true, wade: None, sink: None, round: 0, bullet: None }, (320, 200), &device, &queue).unwrap();
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
    let mut app = offscreen(Args { map: Some("fork".into()), seed: 1, scale: 1, walkers: 1250, keys: Vec::new(), voice: false, mission: None, crater: None, aim: None, core_pull: None, mass: None, own: false, wade: None, sink: None, round: 0, bullet: None }, (320, 200), &device, &queue).unwrap();
    let g = app.game.as_mut().unwrap();
    let s = *g.soldiers.last().unwrap();
    g.last_shots = vec![slime_mould::mould::larder_world::Shot { from: (s.x, s.y), to: (s.x + 5.0, s.y), hit: None, killed: 0, by: Some(g.soldiers.len() - 1), fired: true }];
    app.looks.clear();
    shot_marks(app.game.as_ref().unwrap(), &mut app.tracers, &mut app.looks, None);
    assert_eq!(app.looks.len(), app.game.as_ref().unwrap().soldiers.len());
    assert!(app.looks.last().unwrap().flash > 0.0, "the shot's flash on its soldier");
}
