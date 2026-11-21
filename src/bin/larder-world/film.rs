//! **Films of squads moving** (squad movement rework, 2026-11-16): the
//! window's own game, camera and drawing, stepped at a real frame rate
//! and every frame saved, so what a player sees between the game's 15
//! steps a second shows too (the narrow-gap films of 11-15 saved one
//! frame a step, which hid it). Ignored; run by hand on software Vulkan:
//!
//! `FILM=DIR FILM_MAP=planet:1:1 FILM_N=40 FILM_DO=steer cargo test
//! --release --bin larder-world film_moves -- --ignored --nocapture`
//!
//! `FILM_DO`: `steer` (WASD east, north, west, let go, through the
//! window's own keys), `send:X,Y` (a right click there, dish cells) or
//! `far` (a right click on the crystal furthest within 120 dish cells).
//! `FILM_FPS` (60), `FILM_SECS` (12), `FILM_ZOOM` (8), `FILM_SIZE`
//! (960x540), `FILM_FOLLOW` (1: the camera follows the squad as in play;
//! 0: it stays on `FILM_CENTRE`, dish cells).

use super::*;

#[test]
#[ignore]
fn film_moves() {
    let env = |k: &str| std::env::var(k).ok();
    let num = |k: &str, d: f32| env(k).map_or(d, |v| v.parse().unwrap());
    let pair = |v: &str| {
        let (a, b) = v.split_once(',').unwrap();
        (a.parse::<f32>().unwrap(), b.parse::<f32>().unwrap())
    };
    let dir = env("FILM").expect("FILM=DIR");
    std::fs::create_dir_all(&dir).unwrap();
    let Some((device, queue)) = gpu() else { panic!("no gpu") };
    let size = env("FILM_SIZE").map_or((960, 540), |v| {
        let (a, b) = v.split_once('x').unwrap();
        (a.parse().unwrap(), b.parse().unwrap())
    });
    let args = Args { map: Some(env("FILM_MAP").unwrap_or("planet:1:1".into())), seed: num("FILM_SEED", 1.0) as u64, scale: 2, walkers: 5000, squads: true, ..Default::default() };
    let mut app = offscreen(args, size, &device, &queue).unwrap();
    let n = num("FILM_N", 40.0) as usize;
    {
        let g = app.game.as_mut().unwrap();
        g.rules.max_squad = g.rules.max_squad.max(n);
        // `FILM_STEER=0`: the walk before the rework (`Rules::steer`).
        if env("FILM_STEER").is_some_and(|v| v == "0") {
            g.rules.steer = false;
        }
        g.bank = 1e9;
        while g.alive() < n && g.apply(Cmd::Buy) {}
    }
    let fps = num("FILM_FPS", 60.0);
    let dt = 1.0 / fps;
    let zoom = num("FILM_ZOOM", 8.0);
    let follow = env("FILM_FOLLOW").is_none_or(|v| v != "0");
    // Two game seconds for the bought soldiers to gather.
    for _ in 0..(2.0 / dt) as usize {
        app.squad_frame(dt);
        app.step(dt);
    }
    let what = env("FILM_DO").unwrap_or("steer".into());
    let sc = app.game.as_ref().unwrap().world.scale();
    let centre = env("FILM_CENTRE").map(|v| pair(&v)).map(|c| (c.0 * sc, c.1 * sc)).unwrap_or_else(|| app.game.as_ref().unwrap().middle(0).unwrap());
    app.camera.centre_on(centre);
    app.camera.zoom_to(zoom, app.camera.to_screen(centre));
    app.camera.centre_on(centre);
    if let Some(sq) = app.sq.as_mut() {
        sq.follow = follow;
        sq.put = None;
    }
    let order = |app: &mut App, p: (f32, f32)| {
        app.game.as_mut().unwrap().apply(Cmd::SquadOrder(0, Order::Move(p.0, p.1)));
    };
    if let Some(p) = what.strip_prefix("send:") {
        let p = pair(p);
        order(&mut app, (p.0 * sc, p.1 * sc));
    } else if what == "far" {
        let g = app.game.as_ref().unwrap();
        let at = g.middle(0).unwrap();
        let d = |c: (f32, f32)| (c.0 - at.0).hypot(c.1 - at.1);
        let c = g.world.crystals().into_iter().map(|c| c.0).filter(|&c| d(c) <= 120.0 * sc).max_by(|a, b| d(*a).total_cmp(&d(*b))).unwrap();
        order(&mut app, c);
    }
    // `steer`: the window's own keys, W A S D.
    let keys = |t: f32| -> [bool; 4] {
        if t < 3.0 {
            [false, false, false, true]
        } else if t < 5.5 {
            [true, false, false, false]
        } else if t < 8.0 {
            [false, true, false, false]
        } else {
            [false; 4]
        }
    };
    // `FILM_RAW=1`: soldiers drawn where the game's last step put them
    // (as before the rework), not between steps.
    if env("FILM_RAW").is_some_and(|v| v == "1") {
        app.sub = Some(1.0);
    }
    // Each frame: where the first members are drawn, on the screen.
    let mut log = String::from("f,k,wx,wy,sx,sy\n");
    let secs = num("FILM_SECS", 12.0);
    for f in 0..(secs / dt) as usize {
        let t = f as f32 * dt;
        if what == "steer" {
            let k = keys(t);
            if let Some(sq) = app.sq.as_mut() {
                if sq.keys != k {
                    sq.keys = k;
                    app.send_steer();
                }
            }
        }
        app.squad_frame(dt);
        if !follow {
            app.camera.centre_on(centre);
        }
        app.step(dt);
        app.marks();
        app.prepare();
        let (view, cp) = (app.view.as_ref().unwrap(), app.canvas_pass.as_mut().unwrap());
        let mut enc = device.create_command_encoder(&wgpu::CommandEncoderDescriptor { label: Some("film") });
        view.record(&mut enc, &app.camera, None);
        cp.record(&mut enc, &view.out, view.stride);
        queue.submit([enc.finish()]);
        view.picture().save(format!("{dir}/f{f:04}.png")).unwrap();
        let a = app.between();
        let g = app.game.as_ref().unwrap();
        for (i, k) in g.members(0).into_iter().take(8).enumerate() {
            let s = g.soldiers[k];
            let w = (s.was.0 + (s.x - s.was.0) * a, s.was.1 + (s.y - s.was.1) * a);
            let p = app.camera.to_screen(w);
            log += &format!("{f},{i},{:.3},{:.3},{:.2},{:.2}\n", w.0, w.1, p.0, p.1);
        }
    }
    std::fs::write(format!("{dir}/drawn.csv"), log).unwrap();
}

/// **The end screens** (Look and interface, 2026-11-18): a run's planet
/// won, a run's planet lost and a sandbox round at its clock, each played
/// `FILM_SECS` (60) game seconds by the scripted hand `FILM_HAND`
/// (greedy), then ended as the game would end it, and the window's whole
/// picture saved as `DIR/end-<case>.png`. Ignored; run by hand on
/// software Vulkan:
///
/// `FILM=DIR cargo test --release --bin larder-world film_ends --
/// --ignored --nocapture`
///
/// `FILM_SIZE` (1280x800), `FILM_UI` (the UI size, 1 for 100%),
/// `FILM_CASES` (`won,lost,sandbox`), `FILM_KEYS` (keys over the run's),
/// `FILM_DRAIN` (a second to empty the slime's most covered crystal).
#[test]
#[ignore]
fn film_ends() {
    use slime_mould::mould::larder::End;
    let env = |k: &str| std::env::var(k).ok();
    let dir = env("FILM").expect("FILM=DIR");
    std::fs::create_dir_all(&dir).unwrap();
    let Some((device, queue)) = gpu() else { panic!("no gpu") };
    let size = env("FILM_SIZE").map_or((1280, 800), |v| {
        let (a, b) = v.split_once('x').unwrap();
        (a.parse().unwrap(), b.parse().unwrap())
    });
    let secs: f32 = env("FILM_SECS").map_or(60.0, |v| v.parse().unwrap());
    let hand = env("FILM_HAND").unwrap_or("greedy".into());
    let cases = env("FILM_CASES").unwrap_or("won,lost,sandbox".into());
    for case in cases.split(',') {
        // The command line's own defaults (`main`'s `args`).
        let base = Args { map: Some("fork".into()), seed: 1, scale: 2, walkers: 5000, ..Default::default() };
        let args = match case {
            // Planet 1 of run seed 1 (the Drifter), won; planet 3 (the
            // Trunk), lost.
            "won" | "lost" => {
                let mut r = Run::new(1);
                r.depth = if case == "won" { 1 } else { 3 };
                run_args(&base, &start_menu(&base), &r)
            }
            _ => Args { map: Some("fork".into()), seed: 1, scale: 2, walkers: 5000, squads: true, ..Default::default() },
        };
        // `FILM_KEYS` (2026-11-21, a before-and-after of the slime's
        // graph): engine keys and `set:RULE` keys over the run's, as
        // `k=v,k=v` (`crystal_keep=0,set:ceiling=0`).
        let mut args = args;
        for kv in env("FILM_KEYS").iter().flat_map(|v| v.split(',').map(str::to_string).collect::<Vec<_>>()) {
            let (k, v) = kv.split_once('=').expect("FILM_KEYS=k=v,k=v");
            args.keys.push((k.to_string(), v.to_string()));
        }
        let mut app = offscreen(args, size, &device, &queue).unwrap();
        if let Some(u) = env("FILM_UI") {
            app.canvas.as_mut().unwrap().ui.user = u.parse().unwrap();
        }
        app.prepare();
        let step = 1.0 / 15.0;
        let mut last = u64::MAX;
        // `FILM_DRAIN=T` (2026-11-21): at game second T the crystal the
        // slime covers most is emptied, as headless `LARDER_DRAIN`.
        let mut drain = env("FILM_DRAIN").map(|v| v.parse::<f32>().unwrap());
        while app.game.as_ref().is_some_and(|g| g.time() < secs && g.end.is_none()) {
            let g = app.game.as_mut().unwrap();
            let sec = g.world.ticks() / g.rules.tps as u64;
            if sec != last {
                last = sec;
                hands::plan(g, &hand);
            }
            if drain.is_some_and(|t| g.time() >= t) {
                drain = None;
                let cov = g.world.crystal_covered_each();
                if let Some(c) = (0..cov.len()).max_by_key(|&c| cov[c]) {
                    g.world.empty_crystal(c);
                    println!("drain t {:.1}: crystal {c}, {} of {} covered cells", g.time(), cov[c], cov.iter().sum::<usize>());
                }
            }
            app.step(step);
        }
        let g = app.game.as_mut().unwrap();
        let t = g.time();
        if case == "lost" {
            for s in g.soldiers.iter_mut() {
                s.alive = false;
            }
        }
        if g.end.is_none() {
            g.end = Some(match case {
                "won" => End::MouldWiped(t),
                "lost" => End::SquadWiped(t),
                _ => End::Cap(0.62),
            });
        }
        app.step(step);
        let fit = app.camera.fit();
        app.camera.zoom_to(fit, app.camera.to_screen(app.camera.centre()));
        app.camera.centre_on((app.camera.world.0 / 2.0, app.camera.world.1 / 2.0));
        app.prepare();
        let (view, cp) = (app.view.as_ref().unwrap(), app.canvas_pass.as_mut().unwrap());
        let mut enc = device.create_command_encoder(&wgpu::CommandEncoderDescriptor { label: Some("film") });
        view.record(&mut enc, &app.camera, None);
        cp.record(&mut enc, &view.out, view.stride);
        queue.submit([enc.finish()]);
        let path = format!("{dir}/end-{case}.png");
        view.picture().save(&path).unwrap();
        let m = app.model(None, None);
        println!("{path}: {:?}", m.end);
    }
}

/// **The end's beat** (Look and interface, 2026-11-20, `ending`): a run's
/// planet played by `FILM_HAND` (greedy) to its verdict or `FILM_SECS`
/// game seconds, ended as `film_ends` ends it if it has not ended, then
/// the beat filmed as the window plays it, `FILM_FPS` (15) frames a
/// second at zoom `FILM_ZOOM` (6) on the squad, until a second after the
/// card is up: `DIR/<case>/f0000.png` on. `FILM_CASES` (`won,lost`): the
/// Drifter's planet 1 and the Trunk's planet 3 of run seed 1, as
/// `film_ends`.
///
/// `FILM=DIR cargo test --release --bin larder-world film_end_beat --
/// --ignored --nocapture`
#[test]
#[ignore]
fn film_end_beat() {
    use slime_mould::mould::larder::End;
    let env = |k: &str| std::env::var(k).ok();
    let num = |k: &str, d: f32| env(k).map_or(d, |v| v.parse().unwrap());
    let dir = env("FILM").expect("FILM=DIR");
    let Some((device, queue)) = gpu() else { panic!("no gpu") };
    let size = env("FILM_SIZE").map_or((960, 540), |v| {
        let (a, b) = v.split_once('x').unwrap();
        (a.parse().unwrap(), b.parse().unwrap())
    });
    let secs = num("FILM_SECS", 90.0);
    let hand = env("FILM_HAND").unwrap_or("greedy".into());
    let zoom = num("FILM_ZOOM", 6.0);
    let dt = 1.0 / num("FILM_FPS", 15.0);
    for case in env("FILM_CASES").unwrap_or("won,lost".into()).split(',') {
        let out = format!("{dir}/{case}");
        std::fs::create_dir_all(&out).unwrap();
        let base = Args { map: Some("fork".into()), seed: 1, scale: 2, walkers: 5000, ..Default::default() };
        let mut r = Run::new(1);
        r.depth = if case == "won" { 1 } else { 3 };
        let mut app = offscreen(run_args(&base, &start_menu(&base), &r), size, &device, &queue).unwrap();
        app.prepare();
        let mut last = u64::MAX;
        let over = |app: &App| app.game.as_ref().is_none_or(|g| g.end.is_some() || g.time() >= secs) || app.mission.as_ref().is_some_and(|m| m.verdict.is_some());
        while !over(&app) {
            let g = app.game.as_mut().unwrap();
            let sec = g.world.ticks() / g.rules.tps as u64;
            if sec != last {
                last = sec;
                hands::plan(g, &hand);
            }
            app.step(1.0 / 15.0);
        }
        let verdict = app.mission.as_ref().and_then(|m| m.verdict);
        let g = app.game.as_mut().unwrap();
        if g.end.is_none() && verdict.is_none() {
            if case == "lost" {
                for s in g.soldiers.iter_mut() {
                    s.alive = false;
                }
            }
            let t = g.time();
            g.end = Some(if case == "won" { End::MouldWiped(t) } else { End::SquadWiped(t) });
        }
        println!("{case}: t {:.1}, end {:?}, verdict {:?}, mould {}", g.time(), g.end, verdict.map(|v| (v.cause, v.stars)), g.mould);
        let n = g.soldiers.len().max(1) as f32;
        let at = g.soldiers.iter().fold((0.0, 0.0), |a, s| (a.0 + s.x / n, a.1 + s.y / n));
        app.beat = Some(Default::default());
        app.camera.centre_on(at);
        app.camera.zoom_to(zoom, app.camera.to_screen(at));
        app.camera.centre_on(at);
        for f in 0..((ending::END_BEAT + 1.0) / dt) as usize {
            app.step(dt);
            app.marks();
            app.prepare();
            let (view, cp) = (app.view.as_ref().unwrap(), app.canvas_pass.as_mut().unwrap());
            let mut enc = device.create_command_encoder(&wgpu::CommandEncoderDescriptor { label: Some("film") });
            view.record(&mut enc, &app.camera, None);
            cp.record(&mut enc, &view.out, view.stride);
            queue.submit([enc.finish()]);
            view.picture().save(format!("{out}/f{f:04}.png")).unwrap();
        }
        let g = app.game.as_ref().unwrap();
        println!("{case}: after the beat, world living {}, card {}", g.world.living(), app.model(None, None).end.is_some());
    }
}

/// **The pause menu's pictures** (Look and interface, 2026-11-19): a
/// sandbox round on the Fork a few seconds in, Esc pressed, and the
/// menu filmed; with `FILM_PAGES=main,experiments` each page in turn
/// (the Experiments page is clicked open from the menu's own button). A
/// switch may be preset as the command line's `try=` would
/// (`FILM_TRY=feel,crystal_regrow`), and `FILM_PICK=crystal_regrow`
/// turns one on from the page itself, so the picture shows a pick
/// waiting for Restart. `FILM_UI=2` for 200%.
///
/// `FILM=DIR cargo test --release --bin larder-world film_pause --
/// --ignored --nocapture`
#[test]
#[ignore]
fn film_pause() {
    let env = |k: &str| std::env::var(k).ok();
    let dir = env("FILM").expect("FILM=DIR");
    std::fs::create_dir_all(&dir).unwrap();
    let Some((device, queue)) = gpu() else { panic!("no gpu") };
    let size = env("FILM_SIZE").map_or((1920, 1080), |v| {
        let (a, b) = v.split_once('x').unwrap();
        (a.parse().unwrap(), b.parse().unwrap())
    });
    let mut args = Args { map: Some("fork".into()), seed: 1, scale: 2, walkers: 5000, squads: true, ..Default::default() };
    for t in env("FILM_TRY").unwrap_or_default().split(',').filter(|t| !t.is_empty()) {
        args.keys.extend(slime_mould::build::switches::keys_of(slime_mould::build::switches::parse_try(t).unwrap().1));
    }
    let mut app = offscreen(args, size, &device, &queue).unwrap();
    if let Some(u) = env("FILM_UI") {
        app.canvas.as_mut().unwrap().ui.user = u.parse().unwrap();
    }
    app.prepare();
    for _ in 0..(4.0 * 15.0) as usize {
        app.step(1.0 / 15.0);
    }
    let (_, ev) = app.hud.key(KeyCode::Escape, true, false, &mut app.camera);
    assert_eq!(ev, vec![HudEvent::Menu(MenuItem::Opened)]);
    let shoot = |app: &mut App, name: &str| {
        app.prepare();
        let (view, cp) = (app.view.as_ref().unwrap(), app.canvas_pass.as_mut().unwrap());
        let mut enc = device.create_command_encoder(&wgpu::CommandEncoderDescriptor { label: Some("film") });
        view.record(&mut enc, &app.camera, None);
        cp.record(&mut enc, &view.out, view.stride);
        queue.submit([enc.finish()]);
        let path = format!("{dir}/pause-{name}.png");
        view.picture().save(&path).unwrap();
        println!("{path}");
    };
    for page in env("FILM_PAGES").unwrap_or("main".into()).split(',') {
        film_pause_page(&mut app, page, env("FILM_PICK").as_deref());
        shoot(&mut app, page);
    }
}

/// Opens pause menu page `page` (`main`: as Esc left it; `experiments`,
/// with `pick`'s switch turned to its first choice off the default from
/// the page, as a click on it would).
fn film_pause_page(app: &mut App, page: &str, pick: Option<&str>) {
    match page {
        "main" => {}
        "experiments" => {
            app.hud.show_experiments();
            if let Some(key) = pick {
                let row = slime_mould::build::switches::SWITCHES.iter().position(|s| s.key == key).expect("FILM_PICK: no such switch");
                let s = &slime_mould::build::switches::SWITCHES[row];
                let choice = s.names().iter().position(|n| *n == s.tried().name).unwrap();
                app.pick_experiment(row, choice).unwrap();
            }
        }
        _ => panic!("no pause page {page}"),
    }
}
