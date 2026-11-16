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
    let mut args = Args { map: Some("fork".into()), seed: 1, scale: 2, walkers: 5000, keys: vec![("upkeep".into(), "0.001".into())], voice: true, ..Default::default() };
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
    let mut app = offscreen(Args { map: Some("fork".into()), seed: 1, scale: 2, walkers: 5000, voice: true, ..Default::default() }, (640, 400), &device, &queue).unwrap();
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
    // Card 2 (2026-11-15): attack-move, stop, harvest, guard, the fire
    // button; buy, pump, the rally point, Build (Q) and Tools (E).
    assert_eq!(m.commands.len(), 10);
    assert!(m.end.is_none());
    // The build page, option 1: what you place along the top, the wall
    // among them; the lures' switch, the outpost, the door (walls thread,
    // 11-16) and Back below. (Cancel a build left it: click the building.)
    app.build_page = true;
    let m = app.model(None, None);
    assert_eq!(m.commands.iter().map(|c| c.name.as_str()).collect::<Vec<_>>(), ["Turret", "Pump", "Wall", "Door", "Lure", "Siren", "Lures, sirens on/off", "Outpost", "Back"]);
    assert!(!m.commands[6].ready, "no lures to switch");
}

/// **The tools page works in the window** (2026-11-12): soldiers take
/// tools at the larder between steps, the page lists the five, the germ
/// is ready on the GPU world, as is the sleep gas (both since 11-14), a
/// blast thrown near a soldier is paid, spends a held tool and leaves a
/// crater, and a det cord and a sleep gas cloud do too.
#[test]
fn the_window_uses_the_tools() {
    let Some((device, queue)) = gpu() else { return };
    let mut app = offscreen(Args { map: Some("fork".into()), seed: 1, scale: 1, walkers: 1250, ..Default::default() }, (320, 200), &device, &queue).unwrap();
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

/// Aiming a tool (owner 2026-11-16: a better visual "for aiming and
/// range" when throwing an explosive): the ghost carries a throw from the
/// soldier that would throw, with the kit's reach and the blast's crater
/// and alarm rings; in reach it can go, past it the ghost is red and says
/// how far out, and a building's ghost carries no throw.
#[test]
fn a_tool_is_aimed_with_its_thrower_and_reach() {
    let Some((device, queue)) = gpu() else { return };
    let mut app = offscreen(Args { map: Some("fork".into()), seed: 1, scale: 1, walkers: 1250, ..Default::default() }, (320, 200), &device, &queue).unwrap();
    for _ in 0..10 {
        app.step(1.0 / 15.0);
    }
    let g = app.game.as_mut().unwrap();
    g.bank = 10_000.0;
    let sc = g.world.scale();
    let reach = app.kit.rules.reach * sc;
    let (k, _) = app.kit.thrower(g, g.larder).expect("a soldier holds a tool");
    let s = g.soldiers[k];
    // Open ground a few cells from the thrower, in reach.
    let near = (0..16).map(|i| i as f32 / 16.0 * std::f32::consts::TAU).map(|a| (s.x + 6.0 * sc * a.cos(), s.y + 6.0 * sc * a.sin())).find(|p| g.world.open(p.0, p.1)).expect("open ground by the squad");
    let m = app.model(None, Some((T_BLAST, near, None)));
    let w = m.throw.as_ref().expect("a blast's ghost is a throw");
    assert!(m.ghost_ok, "in reach it can go: {:?}", m.ghost_note);
    assert_eq!(w.reach, reach);
    assert!((w.from.0 - near.0).hypot(w.from.1 - near.1) <= reach, "thrown from a soldier in reach");
    assert_eq!(w.rings.len(), 2, "the crater and the alarm");
    let far = (s.x + 3.0 * reach, s.y);
    let m = app.model(None, Some((T_BLAST, far, None)));
    assert!(m.throw.is_some() && !m.ghost_ok, "past the reach the ghost is red");
    assert!(m.ghost_note.as_deref().is_some_and(|n| n.contains("out of reach")), "and says so: {:?}", m.ghost_note);
    // A cord lands at its middle.
    let m = app.model(None, Some((T_CORD, (s.x + 4.0 * sc, s.y), Some((s.x - 4.0 * sc, s.y)))));
    assert_eq!(m.throw.as_ref().and_then(|w| w.at), Some((s.x, s.y)));
    let m = app.model(None, Some((TURRET, near, None)));
    assert!(m.throw.is_none(), "a building is not thrown");
}

/// A campaign mission plays its own map and rules, shows its goal as the
/// objective, and ends on its own screen when the squad is lost (a rush
/// into the Flats' mould dies by 0:31 on seed 1).
#[test]
fn a_mission_shows_its_goal_and_its_end() {
    let Some((device, queue)) = gpu() else { return };
    let mut app = offscreen(Args { seed: 1, scale: 2, walkers: 5000, mission: Some("1-landfall".into()), ..Default::default() }, (640, 400), &device, &queue).unwrap();
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
    let args = Args { map: mission.is_none().then(|| "fork".into()), seed: 1, scale: 1, walkers: 1250, voice: true, mission: mission.map(String::from), ..Default::default() };
    let b = Bench { secs: 60.0, hand: hand.into(), fps: 24.0, screen: (320, 200), stamps: false, grid: 0, sound: Some(String::new()), fast, passes: 0, settle: false, squad1: false, spread: true, view: None, viewpasses: 0 };
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
    let args = Args { seed: 1, scale: 1, walkers: 625, own: true, ..Default::default() };
    let b = Bench { secs: 60.0, hand: "every".into(), fps: 24.0, screen: (320, 200), stamps: false, grid: 0, sound: Some(String::new()), fast: false, passes: 0, settle: false, squad1: false, spread: true, view: None, viewpasses: 2 };
    let last = bench(args, &b, &device, &queue).unwrap_or_else(|e| panic!("own=1: {e}"));
    assert!(last.contains("t "), "{last}");
    let app = offscreen(Args { seed: 1, scale: 1, walkers: 625, own: true, ..Default::default() }, (320, 200), &device, &queue).unwrap();
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
    let mut app = offscreen(Args { map: Some("fork".into()), seed: 1, scale: 1, walkers: 1250, ..Default::default() }, (320, 200), &device, &queue).unwrap();
    let g = app.game.as_mut().unwrap();
    let s = *g.soldiers.last().unwrap();
    g.last_shots = vec![slime_mould::mould::larder_world::Shot { from: (s.x, s.y), to: (s.x + 5.0, s.y), hit: None, killed: 0, by: Some(g.soldiers.len() - 1), turret: None, fired: true, cast: 0, r: 2.0 }];
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
    // A bare command line plays the squad mode (owner 2026-11-15 07:57Z: "squad mode is default mode"; `main`).
    let bare = Args { map: Some("fork".into()), seed: 1, scale: 2, walkers: 5000, voice: true, squads: true, ..Default::default() };
    let mut cli = bare.clone();
    default_shape(&mut cli);
    let mut menu = start_menu(&bare);
    assert!(menu.options.iter().all(|o| o.is_default()), "a bare command line presets nothing");
    let a = round_args(&bare, false, &menu, &StartEvent::Sandbox);
    assert_eq!((a.map.as_deref(), a.walkers, &a.keys, a.round, a.bullet, a.own), (cli.map.as_deref(), cli.walkers, &cli.keys, 0, None, false));
    assert!(a.squads, "the sandbox plays the squad mode by default");
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
    let cards = |s: &str, next| gun_cards(&rounds::parse(s).unwrap(), &r, &Rules::default(), next, |c| c != rounds::Card::Germ);
    let lit = |v: &[GunCard]| v.iter().map(|c| c.next).collect::<Vec<_>>();
    // Two casts: quiet+gas, then quiet+blast; the second is next.
    let lull = cards("quiet+gas+quiet+blast", Some(1));
    assert_eq!(lit(&lull), [false, false, true, true]);
    assert_eq!(lull.iter().map(|c| c.kind).collect::<Vec<_>>(), [CardKind::Modifier, CardKind::Payload, CardKind::Modifier, CardKind::Payload]);
    // A modifier takes its payload's colour; a payload has no price a hit
    // (owner 18:38Z: "The bank is just to build the gun"), only its
    // budget cost.
    assert_eq!(lull[0].colour, lull[1].colour);
    assert_ne!(lull[0].colour, lull[3].colour);
    assert!(lull[3].about.ends_with("Budget 3."), "{}", lull[3].about);
    // A card after the last payload casts nothing; flight cards ride every
    // bullet, never lit as a cast and never idle.
    let tail = cards("blast+quiet", Some(0));
    assert_eq!((tail[0].next, tail[1].idle, tail[1].next), (true, true, false));
    // A carried cast lights with its carrier: one shot fires both.
    assert_eq!(lit(&cards("trigger+plain+blast+gas", Some(0))), [true, true, true, false]);
    assert_eq!(lit(&cards("trigger+plain+blast+gas", Some(2))), [false, false, false, true]);
    let lance = cards("pierce+split+blast", Some(0));
    assert_eq!(lance.iter().map(|c| (c.next, c.idle)).collect::<Vec<_>>(), [(false, false), (false, false), (true, false)]);
    // A payload this world cannot cast says so.
    assert!(cards("germ", Some(0))[0].about.contains("Not on this world"));
    // Short words in plain language (owner 11-15 02:57Z: "The text for
    // all the gun cards is too much. They should be simplified and
    // clarified"): one sentence each, at most 60 characters, no kind
    // jargon; the run's rolled bullets a line of numbers more.
    for c in rounds::Card::ALL {
        let (_, kind, words) = card_face(c, &r, &Rules::default());
        let low = words.to_lowercase();
        assert!(words.len() <= 60 && words.matches(". ").count() == 0 && !["payload", "modifier", "trigger", "walker", "mould"].iter().any(|j| low.contains(j)), "{c:?}: {words}");
        assert!(["Bullet", "Next bullet", "Always"].contains(&kind.label()));
    }
    assert_eq!(draw::card_words(rounds::Card::Wide, CardKind::Modifier, "Its hit is 1.6x wider."), "Next bullet: its hit is 1.6x wider.");
    let needle = rounds::Card::Ammo(rounds::Ammo::roll_of(rounds::Shell::Bomb, rounds::Set::Needle, 7, 4));
    let (_, _, words) = card_face(needle, &r, &Rules::default());
    assert!(words.len() <= 120 && words.starts_with("Damage "), "{words}");
    // Every loadout H cycles through draws, each card with a glyph and words.
    for (name, l) in LOADOUTS {
        for c in cards(l, Some(0)) {
            assert!(c.icon.is_some() && !c.about.is_empty() && !c.name.is_empty(), "{name}");
        }
    }
}

/// Rolled bullets in the window (Ctrl+H, `ammo=`): a load fits its gun's
/// budget and slots, and each card shows its glyph and its numbers.
#[test]
fn rolled_bullets_load_within_the_gun_and_show_their_numbers() {
    let r = RoundRules::default();
    for kind in GunKind::ALL {
        let gun = kind.average();
        for n in 0..20 {
            let cards = input::bullet_load(&gun, 1, n, 3);
            assert!(!cards.is_empty() && rounds::fit_gun(cards.clone(), &gun).is_ok(), "{kind:?} load {n}: {}", rounds::name(&cards));
            for (c, face) in cards.iter().zip(gun_cards(&cards, &r, &Rules::default(), Some(0), |_| true)) {
                assert!(face.icon.is_some() && face.kind == CardKind::Payload, "{c:?}");
                if let rounds::Card::Ammo(a) = c {
                    assert_eq!(face.name, a.title());
                    assert!(face.about.contains(&format!("Damage {:.1}", a.damage())), "{}", face.about);
                }
            }
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
        let app = offscreen(Args { map: Some("fork".into()), seed: 1, scale: 1, walkers: 1250, ..Default::default() }, screen, &device, &queue).unwrap();
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
    let base = Args { map: Some("fork".into()), seed: 1, scale: 2, walkers: 5000, voice: true, ..Default::default() };
    let mut app = offscreen(base.clone(), (640, 400), &device, &queue).unwrap();
    app.base = Some((base.clone(), true));
    app.menu = Some(start_menu(&base));
    app.open_menu().unwrap();
    app.menu.as_mut().unwrap().show(Page::Options);
    // To the map row, the first on the Planet tab (Options has tabs since
    // 11-15; Right on the Game tab's first row would turn and save the UI
    // size).
    assert_eq!(app.menu.as_ref().unwrap().option("map").unwrap().tab, "Planet");
    app.menu_input(|m| m.key(KeyCode::Tab, true)).unwrap();
    // Down to Planet and back: no restart (the round's clock runs on).
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

/// The squad panel (owner, 2026-11-14 08:22Z) in a squad-mode window: Split
/// makes a second row and commands it, a row click commands that squad,
/// Combine brings the other in. Skips with no GPU (CI).
#[test]
fn the_squad_panel_splits_switches_and_combines() {
    let Some((device, queue)) = gpu() else { return };
    let args = Args { map: Some("fork".into()), seed: 1, scale: 2, walkers: 5000, squads: true, ..Default::default() };
    let mut app = offscreen(args, (1280, 720), &device, &queue).unwrap();
    let rows = |app: &App| app.model(None, None).squads.expect("a squad panel in squad mode").rows;
    let r = rows(&app);
    assert_eq!(r.len(), 1);
    assert!(r[0].active && r[0].soldiers >= 2, "{r:?}");
    let all = r[0].soldiers;
    app.hud_event(HudEvent::SquadSplit).unwrap();
    let r = rows(&app);
    assert_eq!((r.len(), r[0].soldiers + r[1].soldiers), (2, all), "{r:?}");
    assert!(r[1].active && !r[0].active, "the new half is commanded: {r:?}");
    app.hud_event(HudEvent::Squad(0)).unwrap();
    assert!(rows(&app)[0].active);
    app.hud_event(HudEvent::SquadCombine).unwrap();
    let r = rows(&app);
    assert_eq!((r.len(), r[0].soldiers), (1, all), "{r:?}");
    // Stable keys (bug sweep's review, 11-14): a squad keeps its digit
    // while it lives. Split twice (squads 1, 2, 3), fold squad 1 into 2:
    // 2 and 3 keep their keys, and 1 picks nothing.
    app.hud_event(HudEvent::SquadSplit).unwrap();
    app.hud_event(HudEvent::SquadSplit).unwrap();
    assert_eq!(app.game.as_ref().unwrap().live_squads(), vec![0, 1, 2]);
    app.game.as_mut().unwrap().apply(Cmd::Merge(1, 0));
    let keys: Vec<String> = rows(&app).iter().map(|r| r.key.clone()).collect();
    assert_eq!(keys, ["2", "3"]);
    let active = |app: &App| app.game.as_ref().unwrap().active;
    assert!(app.squad_key(KeyCode::Digit3, true, false, false));
    assert_eq!(active(&app), Some(2));
    assert!(app.squad_key(KeyCode::Digit2, true, false, false));
    assert_eq!(active(&app), Some(1));
    app.squad_key(KeyCode::Digit1, true, false, false);
    assert_eq!(active(&app), Some(1), "key 1's squad is gone");
    // The build page has no Outpost without the larder.
    app.build_page = true;
    let cmds = app.model(None, None).commands;
    assert!(cmds.iter().any(|c| c.name == "Turret") && !cmds.iter().any(|c| c.name == "Outpost"), "{:?}", cmds.iter().map(|c| &c.name).collect::<Vec<_>>());
    app.build_page = false;
    // The RTS round has no panel.
    let rts = Args { squads: false, ..app.args.clone() };
    let app = offscreen(rts, (640, 400), &device, &queue).unwrap();
    assert!(app.model(None, None).squads.is_none());
}

fn squad_args(squads: bool) -> Args {
    Args { map: Some("fork".into()), seed: 1, scale: 1, walkers: 1250, squads, ..Default::default() }
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
    // Card 2: one fire button; X still picks Defend.
    let fire = m.commands.iter().find(|c| c.id == FIRE).unwrap();
    assert!(fire.also.contains(&(KeyCode::KeyX, DEFEND)) && fire.also.contains(&(KeyCode::KeyZ, HOLD_FIRE)) && fire.also.contains(&(KeyCode::KeyC, FREE)));
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

/// **The card's Guard reaches a moving squad** (Burn a line's test until
/// the owner removed it, 2026-11-15 19:51Z; the review after the bug sweep,
/// 11-14: an order given soldier by soldier was taken back at the squad's
/// next step). The squad walks to the spot as one and keeps its zone.
#[test]
fn the_cards_guard_reaches_a_moving_squad() {
    let Some((device, queue)) = gpu() else { return };
    let mut app = offscreen(squad_args(true), (640, 400), &device, &queue).unwrap();
    let dt = 1.0 / 60.0;
    app.squad_key(KeyCode::KeyD, true, false, true);
    for _ in 0..30 {
        app.squad_frame(dt);
        app.step(dt);
    }
    app.squad_key(KeyCode::KeyD, false, false, true);
    let at = {
        let g = app.game.as_ref().unwrap();
        let m = g.active.and_then(|q| g.middle(q)).unwrap();
        let sc = g.world.scale();
        g.nearest_standable((m.0 + 6.0 * sc, m.1), 8.0 * sc).unwrap()
    };
    app.hud_event(HudEvent::Point { id: GUARD, at }).unwrap();
    for _ in 0..30 {
        app.squad_frame(dt);
        app.step(dt);
    }
    let g = app.game.as_ref().unwrap();
    let q = g.active.unwrap();
    assert_eq!(g.squads[q].guard, Some(at), "the squad guards the spot");
    for k in g.members(q) {
        assert_eq!(g.guard_zone(k), Some(g.zone_at(at)), "soldier {k} keeps the zone");
    }
}

/// **The minimap in the squad mode** (review after the bug sweep,
/// 2026-11-14): a right click on it sends the squad where it shows, as in
/// the RTS round, and a left click moves the camera, not a volley at the
/// ground hidden behind the minimap.
#[test]
fn the_minimap_sends_the_squad_where_it_shows() {
    let Some((device, queue)) = gpu() else { return };
    let mut app = offscreen(squad_args(true), (640, 400), &device, &queue).unwrap();
    app.squad_frame(1.0 / 60.0);
    app.prepare();
    let (x, y, w, h) = app.hud.minimap.expect("the HUD drew a minimap");
    let c = (x as f32 + w as f32 * 0.8, y as f32 + h as f32 * 0.3);
    app.controls.cursor = Some(c);
    app.hud.moved(Some(c));
    let shows = app.hud.on_minimap(c).unwrap();
    assert!(!app.squad_button(Button::Left, true), "a left click on the minimap is the controls' (the camera)");
    assert!(app.game.as_ref().unwrap().squads.iter().all(|q| q.fire.is_none()), "and fires nothing");
    app.squad_button(Button::Left, false);
    assert!(app.squad_button(Button::Right, true));
    let g = app.game.as_ref().unwrap();
    let d = g.squads[g.active.unwrap()].dest.expect("the squad was sent");
    let sc = g.world.scale();
    assert!((d.0 - shows.0).hypot(d.1 - shows.1) <= 8.0 * sc, "sent to {d:?}, the minimap shows {shows:?}");
}

/// **A steady bottom bar** (owner, 11-14 20:19Z: "the bottom hud menu
/// changes totally when you zoom in because of the minimap"): zoomed all
/// the way out or in close, the minimap is drawn and every bottom panel
/// is where it was.
#[test]
fn the_bottom_bar_and_minimap_stay_put_at_every_zoom() {
    let Some((device, queue)) = gpu() else { return };
    let mut app = offscreen(squad_args(true), (1280, 800), &device, &queue).unwrap();
    let mut bars = Vec::new();
    for zoom in [0.0f32, 24.0] {
        let mid = (640.0, 400.0);
        app.camera.zoom_to(zoom.max(app.camera.fit()), mid);
        assert_eq!(app.camera.sees_all(), zoom == 0.0);
        app.squad_frame(1.0 / 60.0);
        app.prepare();
        assert!(app.hud.minimap.is_some(), "zoom {zoom}: a minimap");
        bars.push((app.hud.bottom, app.hud.minimap_box, app.hud.minimap));
    }
    assert_eq!(bars[0], bars[1], "the bar zoomed out and zoomed in");
}

/// A turret's look follows the game (owner's playtest 11-14 08:15Z:
/// "turrets need to look better"): its shot turns the head down the shot's
/// line and kicks it; between shots it turns toward its aim at
/// `TURRET_TURN`; its lamp is red only when the bank can't pay for a shot.
#[test]
fn a_turret_faces_its_shot_and_its_lamp_is_the_game_state() {
    let Some((device, queue)) = gpu() else { return };
    let mut app = offscreen(Args { map: Some("fork".into()), seed: 1, scale: 1, walkers: 1250, ..Default::default() }, (320, 200), &device, &queue).unwrap();
    let g = app.game.as_mut().unwrap();
    let at = (g.larder.0 + 6.0, g.larder.1);
    g.bank += g.build_price(Kind::Turret, at);
    assert!(g.build(Kind::Turret, at));
    let k = g.buildings.len() - 1;
    (g.buildings[k].built, g.buildings[k].hp) = (1.0, g.buildings[k].max_hp);
    g.last_shots = vec![slime_mould::mould::larder_world::Shot { from: at, to: (at.0, at.1 + 5.0), hit: None, killed: 0, by: None, turret: None, fired: true, cast: 0, r: 2.0 }];
    app.blooks.clear();
    turret_fired(app.game.as_ref().unwrap(), &mut app.blooks);
    let t = app.blooks[k];
    assert!((t.facing - std::f32::consts::FRAC_PI_2).abs() < 1e-4 && t.shot == TURRET_SHOT, "down its shot, kicking: {} {}", t.facing, t.shot);
    // Aim to the right: a tenth of a second turns it TURRET_TURN / 10.
    let g = app.game.as_mut().unwrap();
    (g.buildings[k].aim, g.buildings[k].dense) = (Some((at.0 + 5.0, at.1)), g.rules.aim_min);
    turn_turrets(app.game.as_ref().unwrap(), &mut app.blooks, 0.1);
    assert!((app.blooks[k].facing - (std::f32::consts::FRAC_PI_2 - TURRET_TURN * 0.1)).abs() < 1e-4);
    let g = app.game.as_mut().unwrap();
    assert_eq!(turret_state(g, &g.buildings[k]), TURRET_TRACKING);
    g.rules.price = 1.0;
    g.bank = 0.5;
    assert_eq!(turret_state(g, &g.buildings[k]), TURRET_DRY);
    (g.bank, g.buildings[k].aim) = (10.0, None);
    assert_eq!(turret_state(g, &g.buildings[k]), TURRET_IDLE);
}

/// A step's notes for the building looks (11-14): a siren that called
/// shows its call, a load banked at an outpost flares the outpost and not
/// the larder (one banked at the larder flares the larder), and the slime's
/// bite makes a soldier flinch from the side it came from once
/// `FLINCH_HP` of damage is done.
#[test]
fn a_step_shows_a_siren_call_a_bank_at_an_outpost_and_a_bite() {
    let Some((device, queue)) = gpu() else { return };
    let mut app = offscreen(Args { map: Some("fork".into()), seed: 1, scale: 1, walkers: 1250, ..Default::default() }, (320, 200), &device, &queue).unwrap();
    let g = app.game.as_mut().unwrap();
    let home = g.larder;
    let mut stand = |kind: Kind, at: (f32, f32)| {
        g.bank += g.build_price(kind, at);
        assert!(g.build(kind, at), "{} at {at:?}: {:?}", kind.name(), g.build_blocked(kind, at));
        let k = g.buildings.len() - 1;
        (g.buildings[k].built, g.buildings[k].hp) = (1.0, g.buildings[k].max_hp);
        k
    };
    let siren = stand(Kind::Siren, (home.0 + 6.0, home.1));
    let post = stand(Kind::Outpost, (home.0 - 14.0, home.1));
    let at = g.buildings[post].at;
    (g.buildings[siren].working, g.buildings[siren].on) = (true, true);
    (g.soldiers[0].x, g.soldiers[0].y, g.soldiers[0].carry) = (at.0 + 1.0, at.1, 0.0);
    app.looks.clear();
    join_looks(app.game.as_ref().unwrap(), &mut app.looks);
    app.looks[0].carry = 5.0;
    let mut banked = 0.0;
    step_notes(app.game.as_ref().unwrap(), &mut app.blooks, &mut app.looks, &mut banked);
    assert_eq!(app.blooks[siren].call, SIREN_CALL, "the siren's call");
    assert!(app.blooks[post].bank == BANKED && banked == 0.0, "the outpost flares, not the larder: {} {banked}", app.blooks[post].bank);
    let g = app.game.as_mut().unwrap();
    (g.soldiers[0].x, g.soldiers[0].y) = g.larder;
    app.looks[0].carry = 5.0;
    step_notes(app.game.as_ref().unwrap(), &mut app.blooks, &mut app.looks, &mut banked);
    assert_eq!(banked, BANKED, "a load at the larder flares the larder");
    // Bitten from the right: under FLINCH_HP no flinch, past it one.
    let g = app.game.as_mut().unwrap();
    let s = &mut g.soldiers[0];
    (s.touching, s.aim, s.hp) = (3, Some((s.x + 5.0, s.y)), app.looks[0].hp - 0.5 * FLINCH_HP);
    step_notes(app.game.as_ref().unwrap(), &mut app.blooks, &mut app.looks, &mut banked);
    assert_eq!(app.looks[0].flinch, 0.0, "a nibble under FLINCH_HP");
    app.game.as_mut().unwrap().soldiers[0].hp -= 0.6 * FLINCH_HP;
    step_notes(app.game.as_ref().unwrap(), &mut app.blooks, &mut app.looks, &mut banked);
    assert!(app.looks[0].flinch == FLINCH && app.looks[0].bite.abs() < 1e-4, "flinch {} from {}", app.looks[0].flinch, app.looks[0].bite);
}

fn run_base() -> Args {
    Args { map: Some("fork".into()), seed: 1, scale: 1, walkers: 1250, ..Default::default() }
}

/// A run's level plays as a mission would: no sandbox map, the level's seed and walkers, and the run; the
/// sandbox after it is no run.
#[test]
fn the_menu_starts_a_runs_level_like_a_mission() {
    use slime_mould::world::start::StartEvent;
    let base = run_base();
    let menu = start_menu(&base);
    let mut r = Run::new(9);
    r.depth = 3;
    let a = run_args(&base, &menu, &r);
    let l = r.level();
    assert_eq!((a.map.as_deref(), a.mission.as_deref(), a.own, a.seed, a.keys.len()), (None, None, false, l.seed, 0));
    // Shape C's walkers by the level, whatever the base's (its keys go on
    // in `start`, under the tier's: `run_shape`).
    assert_eq!(a.walkers, run_walkers(l.walkers));
    assert_eq!(run_walkers(1.0), 25_000);
    assert_eq!(a.run.as_ref().map(|r| r.depth), Some(3));
    let mut cli = base.clone();
    cli.run = Some(r);
    assert!(round_args(&cli, false, &menu, &StartEvent::Sandbox).run.is_none(), "the sandbox is not a run");
    // The shop page shows what the run sells, by group.
    let shop = shop_of(&Run::new(1));
    assert!(shop.heading.starts_with("Planet 1 of 8"), "{}", shop.heading);
    assert!(shop.rows.iter().any(|row| row.group == "Buildings" && row.name == "Turrets"));
}

/// **A run in the window** (needs a GPU; the cloud's software Vulkan runs
/// it): planet 1 is Flats with the Drifter and its goal, locked buildings
/// show locked, a won level carries on to the shop with its points, a
/// buy and the next planet land on planet 2 with the upgrade in its rules,
/// and a lost level ends the run on the main menu.
#[test]
fn a_run_plays_a_planet_then_the_shop_then_the_next() {
    use slime_mould::mould::campaign::{Cause, Verdict};
    use slime_mould::mould::run::{reward, ITEMS};
    use slime_mould::world::start::{Page, StartEvent};
    let Some((device, queue)) = gpu() else { return };
    let base = run_base();
    let mut a = base.clone();
    a.run = Some(Run::new(1));
    let mut app = offscreen(a, (1280, 720), &device, &queue).unwrap();
    app.base = Some((base.clone(), true));
    // Planet 1 of the run's seed, the level generator's (#277).
    assert_eq!(app.name, "planet:1:1");
    app.prepare();
    let m = app.model(None, None);
    assert!(m.objective.as_deref().is_some_and(|o| o.starts_with("Wipe out the slime")), "{:?}", m.objective);
    // Turrets and walls are on the build page (Card 2); walls are owned
    // from the start.
    app.build_page = true;
    let m = app.model(None, None);
    let wall = m.commands.iter().find(|c| c.id == WALL).unwrap();
    assert!(wall.why_not.as_deref() != Some("Locked: unlock it in the shop between planets"));
    let turret = m.commands.iter().find(|c| c.id == TURRET).unwrap();
    assert!(!turret.ready && turret.why_not.as_deref().is_some_and(|w| w.starts_with("Locked")));
    app.build_page = false;
    // A won level: the end screen carries on to the shop.
    let win = Verdict { cause: Cause::Goal, at: 300.0, stars: 2, best: 1.0 };
    app.mission.as_mut().unwrap().verdict = Some(win);
    let m = app.model(None, None);
    assert_eq!(m.carry_on.as_deref(), Some("On to the shop"));
    app.carry_on().unwrap();
    assert!(app.in_menu());
    let menu = app.menu.as_ref().unwrap();
    assert_eq!(menu.page, Page::Shop);
    assert!(menu.run_live);
    assert_eq!(app.run.as_ref().map(|r| (r.depth, r.points)), Some((2, reward(1, 2))));
    // Buy armour, then land.
    let hp = app.run.as_ref().unwrap().shop().iter().position(|r| r.name == ITEMS[0].name).unwrap();
    app.menu_event(StartEvent::Buy(hp)).unwrap();
    assert_eq!(app.run.as_ref().unwrap().level_of("hp"), 1);
    // A card dragged to the front of the gun (owner 20:32Z): the gun fires
    // it first, and the shop is drawn again with it there.
    app.run.as_mut().unwrap().cards = vec![(rounds::Card::Wide, true), (rounds::Card::Blast, true)];
    let blast = app.run.as_ref().unwrap().shop().iter().position(|r| r.name == "Blast").unwrap();
    app.menu_event(StartEvent::Move(blast, 0)).unwrap();
    assert_eq!(app.run.as_ref().unwrap().gun_text(), "blast+wide");
    let shop = app.menu.as_ref().unwrap().shop.as_ref().unwrap();
    let on: Vec<&str> = shop.rows.iter().filter(|r| r.on && r.area == slime_mould::world::start::ShopArea::Cards).map(|r| r.name.as_str()).collect();
    assert_eq!(on, ["Blast", "Wide"]);
    assert!(app.in_menu(), "a move is no pick of a round");
    // The turrets' deck (the turrets thread, 2026-11-16): with turrets
    // owned the gun slots switch to the turrets', a bag card goes on, and
    // the landed game's turrets carry it while the squad keeps its gun.
    {
        use slime_mould::mould::run::Pick;
        let r = app.run.as_mut().unwrap();
        r.owned[ITEMS.iter().position(|i| i.key == "turret").unwrap()] = 1;
        r.cards.push((rounds::Card::Gas, false));
        assert!(r.pick(Pick::EditTurrets));
        let gas = r.cards.iter().position(|c| c.0 == rounds::Card::Gas).unwrap();
        assert!(r.pick(Pick::Card(gas)));
        assert_eq!(r.squad_cards(), [rounds::Card::Blast, rounds::Card::Wide]);
    }
    app.menu_event(StartEvent::NextLevel).unwrap();
    assert!(!app.in_menu());
    assert_eq!(app.game.as_ref().unwrap().turret_deck.map(|d| d.cards()), Some(vec![rounds::Card::Gas]));
    assert_eq!(app.rounds.round, [rounds::Card::Blast, rounds::Card::Wide]);
    assert_eq!(app.args.run.as_ref().map(|r| r.depth), Some(2));
    assert_eq!(app.game.as_ref().unwrap().rules.hp, 1.25 * Rules::default().hp);
    assert_eq!(app.name, "planet:2:1");
    // A lost level ends the run.
    app.mission.as_mut().unwrap().verdict = Some(Verdict { stars: 0, cause: Cause::SquadWiped, ..win });
    assert_eq!(app.model(None, None).carry_on.as_deref(), Some("End the run"));
    app.carry_on().unwrap();
    let menu = app.menu.as_ref().unwrap();
    assert!(app.run.is_none() && !menu.run_live && menu.page == Page::Main);
    assert!(menu.notice.as_deref().is_some_and(|n| n.starts_with("The run ended on planet 2")), "{:?}", menu.notice);
}

/// **The gun range in the window** (owner, 11-15 02:55Z: "I need a test
/// environment built where I can play with the guns and cards and
/// bullets"): the start menu's Gun range opens its armory with nothing to
/// pay; two cards taken free (one dragged in front) and Back to the range
/// start the sandbox's round armed with them, with no time limit and a
/// bank that does not run dry. H opens the armory over the paused round;
/// a rack gun swapped in and Back arm the squad with it in the same round,
/// and Restart keeps it. Leave the range goes to the main page, and the
/// gun is still built the next time.
#[test]
fn the_gun_range_builds_a_gun_and_plays_it() {
    use slime_mould::mould::guns::GunKind;
    use slime_mould::mould::run::Pick;
    use slime_mould::world::start::{Page, StartEvent};
    let Some((device, queue)) = gpu() else { return };
    let base = run_base();
    let mut app = offscreen(base.clone(), (1280, 720), &device, &queue).unwrap();
    app.base = Some((base.clone(), true));
    app.open_menu().unwrap();
    app.menu_event(StartEvent::Range).unwrap();
    let menu = app.menu.as_ref().unwrap();
    assert!(menu.page == Page::Shop && menu.shop.as_ref().is_some_and(|s| s.range && s.purse == "Free") && !app.armory);
    let row = |app: &App, name: &str, offer: bool| app.range.as_ref().unwrap().shop().iter().position(|r| r.name == name && matches!(r.pick, Pick::Offer(_) | Pick::Gun(_)) == offer).unwrap_or_else(|| panic!("no {name}"));
    app.menu_event(StartEvent::Buy(row(&app, "Standard blast", true))).unwrap();
    app.menu_event(StartEvent::Move(row(&app, "Wide", true), 0)).unwrap();
    let r = app.range.clone().unwrap();
    assert_eq!((r.gun_text().split('+').next(), r.gun().len(), r.points), (Some("wide"), 2, 0));
    // The shop shows them in the gun.
    let shop = app.menu.as_ref().unwrap().shop.as_ref().unwrap();
    assert_eq!(shop.rows.iter().filter(|x| x.on && x.area == slime_mould::world::start::ShopArea::Cards).count(), 2);
    app.menu_event(StartEvent::NextLevel).unwrap();
    assert!(!app.in_menu() && !app.armory && !app.paused);
    let g = app.game.as_ref().unwrap();
    assert!(g.rules.cap >= 3600.0 && g.bank >= 100_000.0, "cap {} bank {}", g.rules.cap, g.bank);
    assert_eq!(g.gun(0), r.carried());
    assert_eq!(app.rounds.cards_of(g, 0), r.gun());
    app.prepare();
    let m = app.model(None, None);
    assert!(!m.clock.contains('/') && m.objective.as_deref().is_some_and(|o| o.starts_with("Gun range")), "{} {:?}", m.clock, m.objective);
    for _ in 0..8 {
        app.step(1.0 / 15.0);
    }
    let t = app.game.as_ref().unwrap().time();
    let minimap = app.view.as_ref().map(|v| v.minimap);
    // H: the armory over the round, paused, the view's own minimap off
    // (the camera stays zoomed in under it).
    app.next_round();
    assert!(app.in_menu() && app.armory && app.paused && app.menu.as_ref().unwrap().page == Page::Shop);
    assert_eq!(app.view.as_ref().map(|v| v.minimap), minimap.map(|_| false));
    let long = app.range.as_ref().unwrap().shop().iter().position(|x| x.name == "Long gun" && matches!(x.pick, Pick::Gun(_))).unwrap();
    app.menu_event(StartEvent::Buy(long)).unwrap();
    assert_eq!(app.range.as_ref().unwrap().carried().kind, GunKind::Long);
    app.menu_event(StartEvent::NextLevel).unwrap();
    assert!(!app.in_menu() && !app.armory && !app.paused);
    assert_eq!(app.view.as_ref().map(|v| v.minimap), minimap, "the minimap as it was");
    let g = app.game.as_ref().unwrap();
    assert!(g.time() >= t, "the same round: {} then {t}", g.time());
    assert!((0..g.squads.len()).all(|q| g.gun(q).kind == GunKind::Long));
    assert_eq!(app.rounds.cards_of(g, 0), r.gun(), "the cards came along");
    // Restart keeps the range's gun.
    app.start().unwrap();
    assert_eq!(app.game.as_ref().unwrap().gun(0).kind, GunKind::Long);
    // Leave the range: the main page over the sandbox, no range round.
    app.next_gun();
    assert!(app.armory);
    app.menu_event(StartEvent::EndRun).unwrap();
    let menu = app.menu.as_ref().unwrap();
    assert!(!app.armory && menu.page == Page::Main && menu.shop.is_none() && app.args.range.is_none());
    // The gun is still built the next time.
    app.menu_event(StartEvent::Range).unwrap();
    let r = app.range.as_ref().unwrap();
    assert_eq!((r.carried().kind, r.gun().len()), (GunKind::Long, 2));
    // Leave from the menu's armory: the main page.
    app.menu_event(StartEvent::EndRun).unwrap();
    assert_eq!(app.menu.as_ref().unwrap().page, Page::Main);
}

/// **A run's level does not panic the window's loop** (the smoke test's
/// pattern): planet 3, every command pressed every game second, its
/// locked ones too, for 60 game seconds.
#[test]
fn the_window_plays_a_runs_planet() {
    let Some((device, queue)) = smoke_gpu() else { return };
    let mut args = run_base();
    let mut r = Run::new(4);
    r.depth = 3;
    args.run = Some(r);
    let b = Bench { secs: 60.0, hand: "every".into(), fps: 24.0, screen: (320, 200), stamps: false, grid: 0, sound: Some(String::new()), fast: false, passes: 0, settle: false, squad1: false, spread: true, view: None, viewpasses: 0 };
    let last = bench(args, &b, &device, &queue).unwrap_or_else(|e| panic!("run: {e}"));
    assert!(last.contains("t "), "{last}");
}

/// **A run in squad mode, deeper in** (whole-game review, 2026-11-15:
/// squad mode is the default since #330, yet the only run smoke test above
/// plays planet 3 in the RTS round, and nothing played a deeper planet's
/// bigger map and tougher slime through the window). Planets 3 and 6 in
/// squad mode, 60 game seconds each: once with every command pressed every
/// game second (`hand=every`), once with the squad film's hand (`squads`:
/// steer, fire, split, merge back). A round that ends early (planet 6 with
/// `every` ends Stuck at 34 s on seed 4) still passes: the test is that
/// the loop does not panic, not that the round lasts.
#[test]
fn the_window_plays_a_run_in_squad_mode_at_depths_3_and_6() {
    let Some((device, queue)) = smoke_gpu() else { return };
    for depth in [3, 6] {
        for hand in ["every", "squads"] {
            let mut args = run_base();
            args.squads = true;
            let mut r = Run::new(4);
            r.depth = depth;
            args.run = Some(r);
            // 12 frames a second, so frames hold one or two steps (the
            // fast-forward path) and the four rounds cost about half the
            // 170 s they took at 24 on software Vulkan (2026-11-15).
            let b = Bench { secs: 60.0, hand: hand.into(), fps: 12.0, screen: (320, 200), stamps: false, grid: 0, sound: Some(String::new()), fast: false, passes: 0, settle: false, squad1: false, spread: true, view: None, viewpasses: 0 };
            let t0 = Instant::now();
            let last = bench(args, &b, &device, &queue).unwrap_or_else(|e| panic!("planet {depth} {hand}: {e}"));
            eprintln!("squad run planet {depth} {hand}: {last} ({:.1} s)", t0.elapsed().as_secs_f32());
            assert!(last.contains("t "), "planet {depth} {hand}: {last}");
        }
    }
}

/// Every test loadout H cycles fits a default gun's budget (owner's card,
/// 11-14 15:34Z): the presets stay playable as the budget lands.
#[test]
fn every_test_loadout_fits_the_default_budget() {
    let b = RoundRules::default().budget;
    for (name, cards) in LOADOUTS {
        let r = rounds::parse(cards).unwrap_or_else(|| panic!("{name} parses"));
        assert!(rounds::cost(&r) <= b, "{name} costs {} of {b}", rounds::cost(&r));
    }
}

/// **The timed ground refresh reads only the crystal and wall cells and
/// keeps the codes a whole-field rebuild makes** (2026-11-14: the whole
/// read-back and rebuild took 32.9 ms at 6x a planet's area on the 1070;
/// the graphics lead's condition was a byte-identical ground). On the
/// nest map (walls and crystals), with the squad rushing and wall cells
/// worn and broken by hand and three walls raised at 6 s (across a crystal,
/// on open ground, along the world's edge), every refresh over 16 game
/// seconds matches `ground_codes` from the same charge and health.
#[test]
fn the_sparse_ground_refresh_matches_a_whole_one() {
    let Some((device, queue)) = gpu() else { return };
    let mut app = offscreen(Args { map: Some("nest".into()), seed: 1, scale: 1, walkers: 1250, ..Default::default() }, (320, 200), &device, &queue).unwrap();
    let floats = |b: Vec<u8>| -> Vec<f32> { b.chunks_exact(4).map(|c| f32::from_le_bytes(c.try_into().unwrap())).collect() };
    let walls: Vec<usize> = { let d = &app.game.as_ref().unwrap().world.e.dish; (0..d.w * d.h).filter(|&i| d.wall[i]).collect() };
    assert!(walls.len() > 10, "the nest map has walls");
    assert!(app.live.cells.len() > walls.len() && app.live.gather.is_some(), "crystal and wall cells live");
    let (mut checked, mut worn, mut raised, mut doored, mut at) = (0, false, false, false, app.ground_at);
    let wall_cells = |app: &App| app.codes.iter().filter(|&&c| c & 0xF == look::Ground::Wall as u32).count();
    let walls_before = wall_cells(&app);
    while app.game.as_ref().unwrap().time() < 16.0 {
        plan(app.game.as_mut().unwrap(), "rush");
        app.step(1.0 / 15.0);
        if !worn && app.game.as_ref().unwrap().time() > 3.0 {
            // A worn wall cell and a broken one: the refresh draws a
            // share and rubble from health it reads alone.
            let e = &app.game.as_ref().unwrap().world.e;
            let full = e.dish.wall_health.get(walls[0]).copied().filter(|h| h.is_finite()).unwrap_or(400.0);
            e.queue.write_buffer(&e.health, walls[0] as u64 * 4, &(full * 0.4).to_le_bytes());
            e.queue.write_buffer(&e.health, walls[1] as u64 * 4, &0.0f32.to_le_bytes());
            worn = true;
        }
        if !raised && app.game.as_ref().unwrap().time() > 6.0 {
            // Walls raised in play (technical lead, 11-14): across a
            // crystal, on open ground beside it, and along the world's
            // edge; only the tiles they touch are rebuilt.
            use slime_mould::mould::larder_world::World as _;
            app.settle();
            let g = app.game.as_mut().unwrap();
            let ((cx, cy), _) = g.world.crystals()[0];
            g.world.build_wall((cx - 12.0, cy), (cx + 12.0, cy), 3.0);
            g.world.build_wall((cx - 30.0, cy + 30.0), (cx - 30.0, cy + 60.0), 2.0);
            g.world.build_wall((1.0, 5.0), (1.0, 40.0), 2.0);
            g.walls += 3;
            raised = true;
        }
        if !doored && app.game.as_ref().unwrap().time() > 9.0 {
            // Doors cut into standing walls (walls thread, 11-16): no wall
            // cell changes, only `Game::doors`; the refresh that a wall
            // count's change runs draws them as doors.
            app.settle();
            let g = app.game.as_mut().unwrap();
            let n = g.world.e.dish.w * g.world.e.dish.h;
            g.doors = vec![false; n];
            for &i in walls.iter().skip(2).take(6) {
                g.doors[i] = true;
            }
            g.walls += 1;
            doored = true;
        }
        app.prepare();
        if app.ground_at != at {
            at = app.ground_at;
            let g = app.game.as_ref().unwrap();
            let e = &g.world.e;
            let charge = floats(read_back(&e.device, &e.queue, &e.charge));
            let health = floats(read_back(&e.device, &e.queue, &e.health));
            assert_eq!(app.codes, ground_codes(g, &app.depth, Some(&charge), Some(&health)), "t {:.1} s", g.time());
            checked += 1;
        }
    }
    // A refresh waits up to a second for a frame between steps, so 16
    // game seconds hold 6 to 16 of them (6 in 12 s on software Vulkan).
    assert!(checked >= 6, "{checked} refreshes checked");
    let g = app.game.as_ref().unwrap();
    assert_eq!(app.codes[walls[1]] & 0xF, look::Ground::Rubble as u32, "the broken wall cell drew as rubble");
    assert!(doored && walls.iter().skip(2).take(6).all(|&i| app.codes[i] & 0xF == look::Ground::Door as u32), "the doors drew as doors");
    assert!(raised && wall_cells(&app) > walls_before + 20, "the raised walls drew: {} wall cells, {walls_before} before", wall_cells(&app));
    let left: f32 = g.world.crystals().iter().map(|c| c.1).sum();
    assert!(left < app.charge0.iter().sum::<f32>(), "crystals were eaten, so charges changed: {left}");
}

/// **The squad's order lines** (owner's playtest, 2026-11-14 20:12Z and
/// 20:15Z: "there should be a line showing where the selected squad is
/// moving ... same with attack move and other commands ... if I set up
/// three moves, I should see the full path"). A move then two Shift-queued
/// orders draw one chain from the squad's middle through all three points;
/// before, a squad sent anywhere drew no line at all.
#[test]
fn a_squads_orders_and_queue_draw_as_one_path() {
    let Some((device, queue)) = gpu() else { return };
    let mut app = offscreen(squad_args(true), (640, 400), &device, &queue).unwrap();
    app.squad_frame(1.0 / 60.0);
    let g = app.game.as_mut().unwrap();
    let q = g.active.unwrap();
    let m = g.middle(q).unwrap();
    let pts = [(m.0 + 40.0, m.1), (m.0 + 40.0, m.1 + 40.0), (m.0, m.1 + 40.0)];
    assert!(g.apply(Cmd::SquadOrder(q as u8, Order::Move(pts[0].0, pts[0].1))));
    assert!(g.apply(Cmd::SquadQueue(q as u8, Order::Fight(pts[1].0, pts[1].1))));
    assert!(g.apply(Cmd::SquadQueue(q as u8, Order::Move(pts[2].0, pts[2].1))));
    app.marks();
    let lines: Vec<_> = app.marks.list.iter().filter(|k| k.shape == slime_mould::world::Shape::Line).map(|k| (k.at, k.half, k.colour)).collect();
    let near = |a: (f32, f32), b: (f32, f32)| (a.0 - b.0).hypot(a.1 - b.1) < 0.5;
    let from = [m, pts[0], pts[1]];
    for (i, (&a, &b)) in from.iter().zip(&pts).enumerate() {
        let l = lines.iter().find(|l| near(l.0, a) && near(l.1, b));
        assert!(l.is_some(), "leg {i} ({a:?} -> {b:?}) not drawn: {lines:?}");
        // The attack-move leg is red, the moves white.
        assert_eq!(l.unwrap().2[1] < 200, i == 1, "leg {i}'s colour {:?}", l.unwrap().2);
    }
    // One chain for the squad, not one per soldier.
    let n = app.game.as_ref().unwrap().members(q).len();
    assert!(n > 1 && lines.iter().filter(|l| near(l.1, pts[0])).count() == 1, "{n} soldiers drew {} lines to the first point", lines.iter().filter(|l| near(l.1, pts[0])).count());
}

/// **Letting go of the squad** (owner 2026-11-15 02:58Z: "a way to
/// deselect so that you're not actually controlling any specific squad in
/// there and then you can move around the map using wasd"): Esc lets go
/// (nothing selected, every tile AUTO, Let go greyed), WASD then pans the
/// camera unpaused and steers nobody, a second Esc is the HUD's (the menu),
/// and Space, a tile or a click on a squad takes command again.
#[test]
fn esc_lets_go_of_the_squad_and_wasd_pans_until_a_squad_is_taken_again() {
    let Some((device, queue)) = gpu() else { return };
    let mut app = offscreen(squad_args(true), (640, 400), &device, &queue).unwrap();
    app.game.as_mut().unwrap().bank = 1e6;
    for _ in 0..3 {
        app.game.as_mut().unwrap().apply(Cmd::Buy);
    }
    let dt = 1.0 / 60.0;
    app.squad_frame(dt);
    let q = app.game.as_ref().unwrap().active.expect("a squad controlled at the start");
    assert!(!app.controls.selected.is_empty());
    let panel = app.model(None, None).squads.unwrap();
    assert!(!panel.free && panel.rows.iter().all(|r| r.auto != r.active), "the commanded squad is not auto");
    // Esc lets go.
    assert!(app.squad_key(KeyCode::Escape, true, false, true));
    assert!(app.squad_free());
    assert_eq!(app.game.as_ref().unwrap().active, None);
    app.squad_frame(dt);
    assert_eq!(app.game.as_ref().unwrap().active, None, "the frame does not take a squad back");
    assert!(app.controls.selected.is_empty(), "nothing selected");
    let panel = app.model(None, None).squads.unwrap();
    assert!(panel.free && panel.rows.iter().all(|r| r.auto && !r.active), "every tile AUTO");
    // A second Esc is the HUD's: the squad key passes it on.
    assert!(!app.squad_key(KeyCode::Escape, true, false, true));
    // WASD pans the camera, unpaused, and steers no squad.
    assert!(!app.paused);
    app.camera.zoom_to(6.0, app.camera.to_screen(app.camera.centre()));
    let c0 = app.camera.focus();
    assert!(!app.squad_key(KeyCode::KeyD, true, false, true), "D is not the squad's");
    app.controls.wasd_pan = app.squad_free();
    let units = app.units();
    app.controls.key(KeyCode::KeyD, true, false, &units, &mut app.camera);
    for _ in 0..30 {
        app.controls.update(&mut app.camera, dt);
        app.squad_frame(dt);
        app.step(dt);
    }
    app.controls.key(KeyCode::KeyD, false, false, &units, &mut app.camera);
    let c1 = app.camera.focus();
    assert!(c1.0 > c0.0 + 1.0, "D panned the camera right: {c0:?} -> {c1:?}");
    assert!(app.game.as_ref().unwrap().squads.iter().all(|s| s.steer == (0.0, 0.0)), "no squad steered");
    // Space takes the last squad back, and the camera follows it.
    assert!(app.squad_key(KeyCode::Space, true, false, true));
    assert_eq!(app.game.as_ref().unwrap().active, Some(q));
    assert!(!app.squad_free() && app.sq.as_ref().unwrap().follow);
    // The panel's Let go, then a tile, do the same.
    app.squad_panel_event(HudEvent::SquadFree);
    assert!(app.squad_free());
    app.squad_panel_event(HudEvent::Squad(0));
    assert!(!app.squad_free());
    assert_eq!(app.game.as_ref().unwrap().active, Some(app.game.as_ref().unwrap().live_squads()[0]));
    // A click on a squad takes it.
    app.let_go();
    let m = { let g = app.game.as_ref().unwrap(); g.middle(q).unwrap() };
    app.camera.centre_on(m);
    app.squad_frame(dt);
    let at = app.camera.to_screen(m);
    app.controls.cursor = Some(at);
    assert!(app.squad_button(Button::Left, true));
    app.squad_button(Button::Left, false);
    assert_eq!(app.game.as_ref().unwrap().active, Some(q), "clicked squad taken");
}

/// **One click resumes a half-built building** (owner 2026-11-15: "If you
/// start building something but have to stop you should just need to click
/// again to get the squad to resume building"). The squad lays a turret,
/// raises part of it, is sent away; a right click on the ghost, or the
/// turret card's click on it again, sends it back to raise the same ghost:
/// no second building, nothing paid again.
#[test]
fn a_click_on_a_half_built_building_sends_the_squad_back_to_it() {
    let Some((device, queue)) = gpu() else { return };
    let mut app = offscreen(squad_args(true), (640, 400), &device, &queue).unwrap();
    app.game.as_mut().unwrap().bank = 1e6;
    let dt = 1.0 / 60.0;
    app.squad_frame(dt);
    let (q, m, sc) = {
        let g = app.game.as_ref().unwrap();
        let q = g.active.unwrap();
        (q, g.middle(q).unwrap(), g.world.scale())
    };
    let at = [(10.0, 0.0), (-10.0, 0.0), (0.0, 10.0), (0.0, -10.0), (7.0, 7.0), (-7.0, -7.0)]
        .iter()
        .map(|&(dx, dy)| (m.0 + dx * sc, m.1 + dy * sc))
        .find(|&p| app.game.as_ref().unwrap().build_blocked(Kind::Turret, p).is_none())
        .expect("room for a turret beside the squad");
    app.hud_event(HudEvent::Point { id: TURRET, at }).unwrap();
    let b = app.game.as_ref().unwrap().buildings.len() - 1;
    let built = |app: &App| app.game.as_ref().unwrap().buildings[b].built;
    for _ in 0..600 {
        if built(&app) > 0.2 {
            break;
        }
        app.squad_frame(dt);
        app.step(dt);
    }
    let part = built(&app);
    assert!(part > 0.0 && part < 1.0, "part built: {part}");
    // Sent away, it stops raising.
    let away = (m.0 - (at.0 - m.0) * 4.0, m.1 - (at.1 - m.1) * 4.0);
    app.game.as_mut().unwrap().apply(Cmd::SquadOrder(q as u8, Order::Move(away.0, away.1)));
    for _ in 0..240 {
        app.squad_frame(dt);
        app.step(dt);
    }
    let stopped = built(&app);
    assert!(stopped < 1.0 && app.game.as_ref().unwrap().raising(b).0 == 0, "nobody raises it once the squad has gone");
    let (bank, n) = { let g = app.game.as_ref().unwrap(); (g.bank, g.buildings.len()) };
    // One right click on the ghost.
    let c = app.camera.to_screen(at);
    app.controls.cursor = Some(c);
    app.hud.moved(Some(c));
    assert!(app.squad_button(Button::Right, true));
    app.squad_button(Button::Right, false);
    {
        let g = app.game.as_ref().unwrap();
        assert_eq!(g.squads[q].work, Some(Order::Build(b)), "the squad goes back to raise it");
        assert!(g.members(q).iter().all(|&k| g.soldiers[k].order == Order::Build(b)));
        assert_eq!((g.bank, g.buildings.len()), (bank, n), "nothing paid, no second ghost");
    }
    for _ in 0..3000 {
        if built(&app) >= 1.0 {
            break;
        }
        app.squad_frame(dt);
        app.step(dt);
    }
    assert!(built(&app) >= 1.0, "it stands: {}", built(&app));
    assert_eq!(app.game.as_ref().unwrap().buildings.len(), n);

    // The turret card's click again on a second, half-built one: the same.
    let at2 = (at.0 + (at.0 - m.0), at.1 + (at.1 - m.1));
    app.game.as_mut().unwrap().apply(Cmd::Build(Kind::Turret, at2));
    let b2 = app.game.as_ref().unwrap().buildings.len() - 1;
    app.game.as_mut().unwrap().apply(Cmd::SquadOrder(q as u8, Order::Move(away.0, away.1)));
    let (bank, n) = { let g = app.game.as_ref().unwrap(); (g.bank, g.buildings.len()) };
    app.hud_event(HudEvent::Point { id: TURRET, at: at2 }).unwrap();
    let g = app.game.as_ref().unwrap();
    assert_eq!(g.squads[q].work, Some(Order::Build(b2)), "the card's click on its own ghost resumes it");
    assert_eq!((g.bank, g.buildings.len()), (bank, n), "and places nothing");
}

/// N on a planet named with its seed goes on to the next seed (bug check
/// 11-15: the name's seed won over the fresh `seed=`, so N reloaded the
/// same planet); other maps and seedless planets are not renamed.
#[test]
fn n_on_a_seeded_planet_goes_on_to_the_next_seed() {
    use super::input::next_planet;
    assert_eq!(next_planet("planet:3:41").as_deref(), Some("planet:3:42"));
    assert_eq!(next_planet("planet:2:7:1.5").as_deref(), Some("planet:2:8:1.5"));
    assert_eq!(next_planet("planet:3"), None);
    assert_eq!(next_planet("planet"), None);
    assert_eq!(next_planet("fork"), None);
}

/// The menus' own lists on one screen (owner 11-15 17:31Z: "Having to
/// scroll is horrible. That's not professional."): every Options tab and
/// the campaign in one page and the key list in three at most, nothing
/// scrolling, at the worst sizes (16:9 and 21:9 at 200%, a small laptop at
/// 175%, the window as it opens at 125%), in both control modes. The
/// library's `the_menu_pages_fit_one_screen` checks every word's place.
#[test]
fn the_menus_real_lists_fit_one_screen() {
    use slime_mould::world::start::Page;
    for squads in [true, false] {
        let base = Args { squads, ..Default::default() };
        let tabs: Vec<String> = options(&base).iter().map(|o| o.tab.clone()).fold(Vec::new(), |mut t, n| {
            if !t.contains(&n) {
                t.push(n);
            }
            t
        });
        assert_eq!(tabs, ["Game", "Planet", "The mould"]);
        for (screen, user) in [((1920, 1080), 2.0), ((1280, 540), 2.0), ((1280, 720), 2.0), ((1280, 800), 1.75), ((1600, 972), 1.25), ((1920, 1080), 1.0)] {
            let at = format!("{screen:?} at {user}, squads {squads}");
            let mut m = start_menu(&base);
            let mut c = Canvas::new(screen);
            c.ui.user = user;
            let mut draw = |m: &mut StartMenu| {
                for _ in 0..2 {
                    c.clear();
                    m.draw(&mut c);
                }
            };
            m.show(Page::Options);
            for tab in &tabs {
                draw(&mut m);
                assert!(m.sheets() == 1 && !m.scrolls(), "{at}: the {tab} tab takes {} pages (scrolls: {})", m.sheets(), m.scrolls());
                m.key(KeyCode::Tab, true);
            }
            assert_eq!(m.page, Page::Keys, "{at}: Tab after the last options tab opens the keys");
            draw(&mut m);
            // 49 lines in two columns: one page at 100% on 1080 lines, two
            // in the window as it opens at 125% and at 200% on 1080, three
            // at 200% on 720 (11-15; its text stays 15 units, the owner's
            // "the text seems too small").
            assert!(m.sheets() <= 3 && !m.scrolls(), "{at}: the key list takes {} pages", m.sheets());
            m.key(KeyCode::Tab, true);
            assert_eq!(m.page, Page::Options, "{at}: Tab on the keys goes round to the first tab");
            m.show(Page::Missions);
            draw(&mut m);
            assert!(m.sheets() == 1 && !m.scrolls(), "{at}: the campaign takes {} pages", m.sheets());
            // The run's shop three planets in (as `menu=shop-mid` films it)
            // and the gun range's armory (Shop B, 11-15): every tab whole on
            // one screen, but the armory's eight guns, which may take two
            // pages at 200% on a 540-line screen.
            let mut r = Run::new(4);
            for _ in 0..3 {
                r.won(3);
            }
            r.points += 900;
            for _ in 0..3 {
                r.pick(slime_mould::mould::run::Pick::Offer(0));
            }
            for (name, shop) in [("shop", shop_of(&r)), ("armory", shop_of(&Run::range(4)))] {
                m.shop = Some(shop);
                m.show(Page::Shop);
                let n = m.shop.as_ref().map_or(0, |s| s.rows.len());
                // (Cards, Guns, Squad, Buildings, Tools; Guns, Bullets,
                // the two shelves of modifiers and the shove cards'.)
                for tab in 0..5 {
                    draw(&mut m);
                    let most = if name == "armory" && tab == 0 { 2 } else { 1 };
                    assert!(m.sheets() <= most && !m.scrolls(), "{at}: the {name}'s tab {tab} takes {} pages ({n} rows)", m.sheets());
                    m.key(KeyCode::Tab, true);
                }
            }
        }
    }
}

/// **A building clicked with no squad commanded** (owner 2026-11-15
/// 19:51Z, option b): its panel and card; the card's Demolish pulls it
/// down for half its price, and a pump's Upgrade raises its level.
#[test]
fn a_building_selected_shows_its_card_and_acts() {
    let Some((device, queue)) = gpu() else { return };
    let mut app = offscreen(squad_args(true), (1280, 720), &device, &queue).unwrap();
    app.squad_frame(1.0 / 60.0);
    let g = app.game.as_mut().unwrap();
    g.bank = 1e6;
    g.rules.build.raise = 0.0;
    let sc = g.world.scale();
    let m = g.active.and_then(|q| g.middle(q)).unwrap();
    let at = (6..40).filter_map(|d| g.nearest_standable((m.0 + d as f32 * sc, m.1), 4.0 * sc)).find(|&p| g.build_blocked(Kind::Turret, p).is_none()).unwrap();
    assert!(g.apply(Cmd::Build(Kind::Turret, at)));
    let b = g.buildings.len() - 1;
    // A click on it while a squad is commanded fires; let go, it selects.
    app.let_go();
    app.building_sel = app.game.as_ref().unwrap().building_at(at);
    assert_eq!(app.building_sel, Some(b));
    app.controls.selected.clear();
    let mdl = app.model(None, None);
    assert_eq!(mdl.info.as_ref().map(|i| i.title.as_str()), Some("Turret"));
    let ids: Vec<u32> = mdl.commands.iter().map(|c| c.id).collect();
    assert_eq!(ids, vec![DEMOLISH, UPGRADE], "a turret's card: demolish and Render (the kill box pays, 2026-11-15)");
    let bank = app.game.as_ref().unwrap().bank;
    app.hud_event(HudEvent::Command { id: UPGRADE }).unwrap();
    let g = app.game.as_ref().unwrap();
    assert_eq!(g.buildings[b].level, 2, "it renders");
    assert_eq!(g.bank, bank - g.rules.build.render_cost);
    let before = g.bank;
    app.hud_event(HudEvent::Command { id: DEMOLISH }).unwrap();
    let g = app.game.as_ref().unwrap();
    assert!(!g.buildings[b].alive);
    assert_eq!(g.bank, before + 0.5 * g.rules.build.turret_cost);
    assert_eq!(app.building_sel, None);
}

/// **Shake and hit-stop feel blasts only** (D6, 11-15): a bomb-sized blast
/// (6 kills, 80 thrown: size 0.24) adds 0.12 + 0.6 x 0.24 to the shake at
/// `shake=1` and holds the game 0.05 + 0.07 x 0.24 s at `hitstop=1`; a
/// second within 0.6 s shakes again but does not hold; blasts already felt
/// do nothing; both at 0 (the default) do nothing.
#[test]
fn blasts_shake_and_hold_only_when_asked() {
    use slime_mould::mould::rounds::{Card, Pop};
    let pop = |tick: u64| Pop { burst: Default::default(), throw: None, payload: Card::Bomb, tick, killed: 6, flung: 80 };
    let (mut felt, mut trauma, mut stop, mut cool) = (0u64, 0.0f32, 0.0f32, 0.0f32);
    feel(&[pop(10)], &mut felt, &mut trauma, &mut stop, &mut cool, (0.0, 0.0));
    assert_eq!((trauma, stop, felt), (0.0, 0.0, 10), "off by default, but felt");
    let (mut felt, mut trauma, mut stop, mut cool) = (0u64, 0.0f32, 0.0f32, 0.0f32);
    let size = 6.0 / 150.0 + 80.0 / 400.0;
    feel(&[pop(10)], &mut felt, &mut trauma, &mut stop, &mut cool, (1.0, 1.0));
    assert!((trauma - (0.12 + 0.6 * size)).abs() < 1e-6, "shake {trauma}");
    assert!((stop - (0.05 + 0.07 * size)).abs() < 1e-6, "hold {stop}");
    assert_eq!(cool, 0.6);
    // Felt already: nothing more.
    let t0 = trauma;
    feel(&[pop(10)], &mut felt, &mut trauma, &mut stop, &mut cool, (1.0, 1.0));
    assert_eq!(trauma, t0);
    // A new one inside the 0.6 s: more shake, no new hold.
    stop = 0.0;
    feel(&[pop(10), pop(12)], &mut felt, &mut trauma, &mut stop, &mut cool, (1.0, 1.0));
    assert!(trauma > t0 && stop == 0.0, "shake {trauma}, hold {stop}");
}
