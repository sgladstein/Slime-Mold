//! Timing: the bench, the machine line and the off-screen window. Moved out
//! of `larder-world.rs` unchanged (C9, 2026-11-14).

use super::*;

/// **The window's frame time without a window** (`-- bench=SECS`,
/// 2026-11-12): plays `secs` game seconds with `hand` at `fps` frames a
/// real second, doing each frame what the window does (the game's steps
/// through the same accumulator, the ground, marks and HUD, the view and
/// canvas recorded and submitted), then waits for the GPU, as a present
/// with vsync off would. So it can run over Remote Control on the GTX 1070
/// with nobody at the screen. The W2c gate is a frame under 10 ms p95.
/// Prints each frame's time and its parts (the game's steps, which wait on
/// the GPU inside; preparing; recording; the GPU's tail), the step frames'
/// alone, the read-backs and waits a step, and with timestamps the GPU's
/// ms a step's ticks (`stamps=0` leaves them off). `grid=BIN` asks for the
/// mould grid in squares of BIN dish cells every game step, as the
/// campaign's goals do. `hand=` is `rush`, `farm`, a player AI style
/// (`larder_world_ai`: boom, claim, late, fort, strike), or `every`
/// (`press_every`). A mission's verdict ends the run as the game's end
/// does, and a few frames of the end screen follow, untimed. With
/// `passes=N` (and timestamps), N more game steps follow, each pass of each
/// stamped alone and waited for, printed as a table: the GPU ms a step of
/// each pass (all its runs in the step), median over the N steps. Returns
/// the last line of the timed run (the round at its end), for the smoke
/// test. `centre=X,Y[,ZOOM]` puts the camera there for the run (the draw
/// timed at a zoom; the window opens fitted).
pub(super) fn bench(args: Args, b: &Bench, device: &wgpu::Device, queue: &wgpu::Queue) -> Result<String, String> {
    let (secs, hand, fps, screen, grid) = (b.secs, b.hand.as_str(), b.fps, b.screen, b.grid);
    let mut app = offscreen(args, screen, device, queue)?;
    let stamped = b.stamps && device.features().contains(wgpu::Features::TIMESTAMP_QUERY);
    if stamped {
        app.game.as_mut().unwrap().world.stamps = Some(slime_mould::gpu::Stamps::new(device, queue));
    }
    if b.sound.is_some() {
        app.tape = Some((VoiceBank::new(48_000, app.args.seed), Vec::new()));
    }
    app.fast = b.fast;
    app.spread = b.spread;
    app.game.as_mut().unwrap().world.e.squad_one = b.squad1;
    if let Some((x, y, zoom)) = b.view {
        // The HUD sets the camera's insets as it draws (as `headless`).
        app.prepare();
        app.camera.zoom_to(zoom, app.camera.to_screen(app.camera.centre()));
        app.camera.centre_on((x, y));
        println!("  view: zoom {} at {x:.0},{y:.0}", app.camera.zoom);
    }
    let mut ai = match hand {
        "rush" | "farm" | "every" => None,
        _ => app.game.as_ref().and_then(|g| slime_mould::mould::larder_world_ai::Hand::named(hand, g)),
    };
    // The tape's ear: on the squad, at the window's opening zoom (as the
    // headless run's).
    let span = app.camera.span();
    let dt = 1.0 / fps;
    let mut last_sec = u64::MAX;
    // Each frame: total, step, prepare, record, GPU tail (ms); and whether
    // the game stepped in it.
    type Row = ([f64; 5], bool);
    let mut rows: Vec<Row> = Vec::new();
    let (reads0, steps0) = (slime_mould::gpu::reads(), app.steps);
    let ms = |t: Instant| t.elapsed().as_secs_f64() * 1e3;
    let over = |app: &App| app.game.as_ref().is_none_or(|g| g.end.is_some()) || app.mission.as_ref().is_some_and(|m| m.verdict.is_some());
    // One frame as the window draws it: the game's steps, the sound, the
    // HUD and marks, the view and canvas; the GPU waited for.
    let frame = |app: &mut App, t0: Instant, step: f64| -> Result<[f64; 5], String> {
        if let Some(g) = app.game.as_ref().filter(|_| app.tape.is_some()) {
            let alive: Vec<(f32, f32)> = g.soldiers.iter().filter(|s| s.alive).map(|s| (s.x, s.y)).collect();
            let n = alive.len().max(1) as f32;
            let centre = if alive.is_empty() { g.larder } else { (alive.iter().map(|p| p.0).sum::<f32>() / n, alive.iter().map(|p| p.1).sum::<f32>() / n) };
            app.hear(dt, Ear { centre, half: (span.0 / 2.0, span.1 / 2.0) });
        }
        let t1 = Instant::now();
        app.prepare();
        let prep = ms(t1);
        let t2 = Instant::now();
        let (view, cp) = (app.view.as_ref().unwrap(), app.canvas_pass.as_mut().unwrap());
        let mut enc = device.create_command_encoder(&wgpu::CommandEncoderDescriptor { label: Some("bench") });
        view.record(&mut enc, &app.camera, None);
        cp.record(&mut enc, &view.out, view.stride);
        let index = queue.submit([enc.finish()]);
        let rec = ms(t2);
        let t3 = Instant::now();
        device.poll(wgpu::PollType::Wait { submission_index: Some(index), timeout: None }).map_err(|e| format!("poll: {e}"))?;
        Ok([ms(t0), step, prep, rec, ms(t3)])
    };
    while app.game.as_ref().is_some_and(|g| g.time() < secs) && !over(&app) {
        let t0 = Instant::now();
        // Whole steps: the hand plans on the step that crossed the second
        // once it has finished, as it did before the split (a step left
        // running would hand it the last step's bank and crystals).
        let sec = app.game.as_ref().map_or(0, |g| g.settled_ticks() / g.rules.tps as u64);
        if sec != last_sec {
            last_sec = sec;
            if hand == "every" {
                press_every(&mut app, sec)?;
            } else {
                let g = app.game.as_mut().unwrap();
                match ai.as_mut() {
                    Some(h) => {
                        h.play(g);
                    }
                    None => plan(g, hand),
                }
            }
        }
        let before = app.steps;
        app.step(dt);
        if b.settle {
            app.settle();
        }
        if grid > 0 && app.steps > before {
            // As the campaign's goals ask (2026-11-12): once a game step.
            std::hint::black_box(app.game.as_ref().unwrap().world.mould_grid(grid));
        }
        let step = ms(t0);
        rows.push((frame(&mut app, t0, step)?, app.steps > before));
    }
    // The last step's results, so the counts below are the whole game's.
    app.settle();
    if over(&app) {
        for _ in 0..3 {
            app.step(dt);
            frame(&mut app, Instant::now(), 0.0)?;
        }
    }
    let g = app.game.as_ref().unwrap();
    let steps = (app.steps - steps0).max(1);
    let q = |pick: &dyn Fn(&Row) -> Option<f64>, at: f64| {
        let mut v: Vec<f64> = rows.iter().filter_map(pick).collect();
        v.sort_by(f64::total_cmp);
        if v.is_empty() {
            return 0.0;
        }
        v[((v.len() - 1) as f64 * at).round() as usize]
    };
    let line = |name: &str, i: usize, only_steps: bool| {
        let pick = move |r: &Row| (!only_steps || r.1).then_some(r.0[i]);
        format!("{name} p50 {:.2} p95 {:.2} max {:.2}", q(&pick, 0.5), q(&pick, 0.95), q(&pick, 1.0))
    };
    let e = &g.world.e;
    let keys: String = app.args.keys.iter().map(|(k, v)| format!(" {k}={v}")).collect();
    let keys = if grid > 0 { format!("{keys}; the mould grid in squares of {grid} each step") } else { keys };
    let keys = match (&app.args.mission, b.fast) {
        (Some(m), f) => format!("{keys}; mission {m}{}", if f { ", fast forward" } else { "" }),
        (None, true) => format!("{keys}; fast forward"),
        (None, false) => keys,
    };
    // The crowd limit as played (`Engine::crowd`, after the run's params),
    // so a bench says when the engine dropped it (`gpu::crowd_holds`).
    let crowd = match (e.crowd, e.crowd_ok) {
        (true, _) => "one walker a cell",
        (false, true) => "no crowd limit",
        (false, false) => "the crowd limit OFF (walkers over half the agar)",
    };
    println!("larder-world bench: {} seed {}, {} x {} cells, {} slots, {crowd}, scale {} walkers {}{keys}; hand {hand}, {:.0} game s at {fps} frames a second, screen {}x{}", app.name, app.args.seed, e.dish.w, e.dish.h, e.n, app.args.scale, app.args.walkers, g.time(), screen.0, screen.1);
    println!("  {} frames, {} game steps; {:.2} read-backs a step, {:.2} waits a step ({:.2} ms a step waiting); steps finished once in {}, when the next was due {}", rows.len(), steps, (slime_mould::gpu::reads() - reads0) as f64 / steps as f64, g.world.waits as f64 / steps as f64, g.world.wait_ms / steps as f64, app.finished.0, app.finished.1);
    for (only, tag) in [(false, "every frame"), (true, "frames with a step")] {
        println!("  {tag} ({} of them), ms: {}; step {}; prepare {}; record {}; GPU tail {}", rows.iter().filter(|r| !only || r.1).count(), line("frame", 0, only), line("", 1, only), line("", 2, only), line("", 3, only), line("", 4, only));
    }
    if let Some(st) = &g.world.stamps {
        let (sim, draw, _, n) = st.median();
        if n > 0 {
            println!("  GPU (timestamps, median of {n} steps): the step's ticks {sim:.2} ms, its drawing {draw:.2} ms");
        } else {
            println!("  GPU: no step timestamps, as each step ran in shares (spread=0 for them)");
        }
    } else {
        println!("  GPU: no timestamps on this adapter");
    }
    if let (Some(path), Some((bank, tape))) = (&b.sound, app.tape.as_ref()) {
        let rms = (tape.iter().map(|v| v * v).sum::<f32>() / tape.len().max(1) as f32).sqrt();
        println!("  sound: {:.1} s on tape, rms {:.4}, {} grains", tape.len() as f32 / 2.0 / bank.sample_rate() as f32, rms, bank.stats.grains);
        if !path.is_empty() {
            slime_mould::wav::write(Path::new(path), tape, 2, bank.sample_rate()).map_err(|e| format!("{path}: {e}"))?;
            println!("  {path}");
        }
    }
    let verdict = app.mission.as_ref().and_then(|m| m.verdict).map_or(String::new(), |v| format!(", mission: {} star{} ({:?})", v.stars, if v.stars == 1 { "" } else { "s" }, v.cause));
    let last = format!("  t {:.0} s: mould {}, soldiers {}, buildings {}, harvested {:.0}, shots {}, killed {}{}{verdict}", g.time(), g.mould, g.alive(), g.buildings.len(), g.harvested, g.shots, g.killed, g.end.map_or(String::new(), |e| format!(", ended: {}", e.band())));
    println!("{last}");
    if b.passes > 0 && stamped && !over(&app) {
        print_passes(&mut app, b.passes, dt);
    }
    Ok(last)
}

/// The bench's `passes=N`: `n` more game steps with every GPU pass timed
/// alone, then a line a pass in the step's order: its runs a step, and the
/// median over the steps of its GPU ms a step (all its runs) and a run.
pub(super) fn print_passes(app: &mut App, n: u32, dt: f32) {
    app.game.as_mut().unwrap().world.profile = Some(Vec::new());
    let goal = app.steps + n;
    while app.steps < goal && app.game.as_ref().is_some_and(|g| g.end.is_none()) {
        app.step(dt);
    }
    app.settle();
    let steps = app.game.as_mut().unwrap().world.profile.take().unwrap_or_default();
    let mut names: Vec<&str> = Vec::new();
    for s in &steps {
        for &(p, _) in s {
            if !names.contains(&p) {
                names.push(p);
            }
        }
    }
    let median = |mut v: Vec<f64>| {
        v.sort_by(f64::total_cmp);
        v.get(v.len() / 2).copied().unwrap_or(0.0)
    };
    let total = median(steps.iter().map(|s| s.iter().map(|p| p.1).sum()).collect());
    println!("  GPU a step, every pass timed alone (median of {} steps): {total:.3} ms in all", steps.len());
    for name in names {
        // A pass that runs only on some steps (shots) shows how many; its
        // ms a step is still the median over all of them.
        let counts: Vec<usize> = steps.iter().map(|s| s.iter().filter(|p| p.0 == name).count()).collect();
        let runs = counts.iter().copied().max().unwrap_or(0);
        let some = counts.iter().filter(|&&c| c > 0).count();
        let sum = median(steps.iter().map(|s| s.iter().filter(|p| p.0 == name).map(|p| p.1).sum::<f64>().max(0.0)).collect());
        let one = median(steps.iter().flat_map(|s| s.iter().filter(|p| p.0 == name).map(|p| p.1)).collect());
        let on = if some < steps.len() { format!(", on {some} of {} steps", steps.len()) } else { String::new() };
        println!("    {name:<10} {runs:>3} a step  {sum:>7.3} ms a step  {one:>7.3} ms a run  {:>5.1}%{on}", 100.0 * sum / total.max(1e-9));
    }
}

/// `hand=every`, the smoke test's hand (2026-11-12): once a game second,
/// every command the HUD's card shows ready, pressed through the window's
/// own `hud_event` as a player's click would send it, starting one further
/// along the card each second (so each second ends on another order), with
/// the whole squad selected and the larder every other second, aimed by
/// turns at the squad's middle, a crystal, the core and the larder (a line
/// runs across the aim). A command a thread adds to the card is pressed
/// with the rest, by its target.
pub(super) fn press_every(app: &mut App, sec: u64) -> Result<(), String> {
    let Some(g) = app.game.as_ref() else { return Ok(()) };
    let sc = g.world.scale();
    let alive: Vec<usize> = (0..g.soldiers.len()).filter(|&k| g.soldiers[k].alive).collect();
    let n = alive.len().max(1) as f32;
    let mid = if alive.is_empty() { g.larder } else { (alive.iter().map(|&k| g.soldiers[k].x).sum::<f32>() / n, alive.iter().map(|&k| g.soldiers[k].y).sum::<f32>() / n) };
    let cs = g.world.crystals();
    let crystal = cs.get(sec as usize % cs.len().max(1)).map_or(g.core, |c| c.0);
    let aims = [mid, crystal, g.core, g.larder];
    let (w, h) = app.camera.world;
    let inside = |p: (f32, f32)| (p.0.clamp(0.0, w - 1.0), p.1.clamp(0.0, h - 1.0));
    app.larder_sel = sec % 2 == 1;
    app.hud_event(HudEvent::Idle)?;
    app.controls.selected = alive;
    let cmds = app.model(None, None).commands;
    for i in 0..cmds.len() {
        let k = (i + sec as usize) % cmds.len();
        let c = &cmds[k];
        let at = aims[k % aims.len()];
        let ev = match c.target {
            _ if !c.ready => HudEvent::NotReady { id: c.id },
            Target::Instant => HudEvent::Command { id: c.id },
            Target::Point { .. } | Target::Area { .. } => HudEvent::Point { id: c.id, at },
            Target::Line { .. } => HudEvent::Line { id: c.id, from: inside((at.0 - 4.0 * sc, at.1 - 2.0 * sc)), to: inside((at.0 + 4.0 * sc, at.1 - 2.0 * sc)) },
        };
        if app.hud_event(ev)? {
            break;
        }
    }
    Ok(())
}

/// The machine a bench runs on, as it starts and ends: free memory, the
/// GPU's memory in use and how busy it is, and the other programs on it
/// (from `nvidia-smi`, where there is one), so a slow run shows why in its
/// own output. Asked for by the technical lead on 2026-11-12, after a
/// round of 1070 timings overlapped a campaign run on the owner's PC with
/// 0.5 GB of 8 free: every frame's median went from 0.6 ms to 2-3 ms and
/// one run died for want of memory, with nothing in its output to say so.
pub(super) fn machine() -> String {
    let run = |cmd: &str, args: &[&str]| {
        let out = std::process::Command::new(cmd).args(args).output().ok().filter(|o| o.status.success())?;
        Some(String::from_utf8_lossy(&out.stdout).trim().to_string())
    };
    let gb = |kb: f64| kb / 1024.0 / 1024.0;
    let ram = if cfg!(windows) {
        run("powershell", &["-NoProfile", "-Command", "$o = Get-CimInstance Win32_OperatingSystem; \"$($o.FreePhysicalMemory) $($o.TotalVisibleMemorySize)\""])
    } else {
        std::fs::read_to_string("/proc/meminfo").ok().map(|m| {
            let kb = |key: &str| m.lines().find(|l| l.starts_with(key)).and_then(|l| l.split_whitespace().nth(1)).unwrap_or("0").to_string();
            format!("{} {}", kb("MemAvailable:"), kb("MemTotal:"))
        })
    };
    let ram = ram
        .and_then(|r| {
            let mut v = r.split_whitespace().map(|x| x.parse::<f64>().ok());
            Some(format!("RAM {:.1} GB free of {:.1}", gb(v.next()??), gb(v.next()??)))
        })
        .unwrap_or_else(|| "RAM unknown".into());
    let gpu = run("nvidia-smi", &["--query-gpu=memory.used,memory.total,utilization.gpu", "--format=csv,noheader,nounits"])
        .and_then(|l| {
            let v: Vec<&str> = l.lines().next()?.split(',').map(str::trim).collect();
            Some(format!("GPU memory {} MB in use of {}, {}% busy", v.first()?, v.get(1)?, v.get(2)?))
        })
        .unwrap_or_else(|| "GPU memory and use unknown (no nvidia-smi)".into());
    // The process table: `| GPU GI CI PID Type Name... Memory |`, names
    // cut to the file name, this bench left out.
    let me = std::env::current_exe().ok().and_then(|p| p.file_name().map(|n| n.to_string_lossy().to_string())).unwrap_or_default();
    let mut others: Vec<String> = Vec::new();
    let table = run("nvidia-smi", &[]);
    if let Some(t) = &table {
        for l in t.lines().skip_while(|l| !l.contains("Processes:")) {
            let w: Vec<&str> = l.trim_matches(|c: char| c == '|' || c.is_whitespace()).split_whitespace().collect();
            if w.len() >= 7 && w[3].parse::<u32>().is_ok() {
                let name = w[5..w.len() - 1].join(" ");
                let name = name.rsplit(['\\', '/']).next().unwrap_or(&name).to_string();
                if name != me && !others.contains(&name) {
                    others.push(name);
                }
            }
        }
    }
    let others = match (&table, others.is_empty()) {
        (None, _) => "no list of the GPU's programs".to_string(),
        (Some(_), true) => "no other programs on the GPU".to_string(),
        (Some(_), false) => format!("on the GPU too: {}", others.join(", ")),
    };
    format!("{ram}; {gpu}; {others}")
}

/// A GPU with the adapter's own limits (the engine binds 48 buffers), and
/// timestamps where it has them (`bench`).
pub(super) fn gpu() -> Option<(wgpu::Device, wgpu::Queue)> {
    let instance = wgpu::Instance::new(wgpu::InstanceDescriptor::new_without_display_handle_from_env());
    let adapter = block_on(instance.request_adapter(&wgpu::RequestAdapterOptions { power_preference: wgpu::PowerPreference::HighPerformance, ..Default::default() })).ok()?;
    let info = adapter.get_info();
    println!("larder-world: {} ({:?})", info.name, info.backend);
    let required_features = adapter.features() & wgpu::Features::TIMESTAMP_QUERY;
    block_on(adapter.request_device(&wgpu::DeviceDescriptor { label: Some("larder-world"), required_features, required_limits: adapter.limits(), ..Default::default() })).ok()
}

/// The window's game, view and UI on a `screen`-sized picture with no
/// window, the round started.
pub(super) fn offscreen(args: Args, screen: (u32, u32), device: &wgpu::Device, queue: &wgpu::Queue) -> Result<App, String> {
    let mut app = App::new(args);
    let frame = device.create_buffer(&wgpu::BufferDescriptor { label: Some("no world"), size: 16, usage: wgpu::BufferUsages::STORAGE, mapped_at_creation: false });
    app.view = Some(View::new(device, queue, &frame, (1, 1), screen));
    app.canvas = Some(Canvas::new(screen));
    app.canvas_pass = Some(CanvasPass::new(device, queue));
    app.gpu = Some((device.clone(), queue.clone()));
    app.start()?;
    Ok(app)
}
