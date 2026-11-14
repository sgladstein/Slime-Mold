//! Recording without a window: films, sound and the shot marks. Moved out
//! of `larder-world.rs` unchanged (C9, 2026-11-14).

use super::*;

/// **No window** (`look=DIR`): the round played by a simple hand, with the
/// window's whole picture (the look, marks and HUD) saved at the game
/// seconds in `at`, and the counts beside each printed: what the window
/// shows, checkable where there is no screen (the cloud's software
/// Vulkan). Hands: `rush` (all attack the core), `farm` (all harvest the
/// crystal nearest the larder, buying soldiers from 1,200 food), `idle`.
// Two more than clippy's seven: the look's options are plain values from
// the command line, and a struct for them would only rename them.
#[allow(clippy::too_many_arguments)]
pub(super) fn headless(args: Args, dir: &str, hand: &str, at: &[f32], screen: (u32, u32), wav: Option<&str>, time_holes: bool, fixed: Option<(f32, f32, f32)>, cut: Option<Cut>, bank: Option<f32>, booms: &[(f32, f32, f32)], menu_page: Option<slime_mould::world::start::Page>, hover: Option<(f32, f32)>, select: bool) -> Result<(), String> {
    std::fs::create_dir_all(dir).map_err(|e| format!("{dir}: {e}"))?;
    let (device, queue) = gpu().ok_or("no GPU adapter")?;
    let mut app = offscreen(args, screen, &device, &queue)?;
    if let (Some(b), Some(g)) = (bank, app.game.as_mut()) {
        g.bank = b;
    }
    // `menu=PAGE`: the start menu's page drawn over the round, as the
    // window shows it (the menu's pictures).
    if let Some(p) = menu_page {
        let mut m = start_menu(&app.args);
        m.show(p);
        app.menu = Some(m);
    }
    if wav.is_some() {
        app.tape = Some((VoiceBank::new(48_000, app.args.seed), Vec::new()));
    }
    // The recording's ear: on the squad, at the window's opening zoom.
    let span = app.camera.span();
    let mut voice_at = u64::MAX;
    let step = { let r = Rules::default(); r.frame_ticks as f32 / r.tps as f32 };
    let mut last_sec = u64::MAX;
    // `hand=` a player AI style (boom, claim, late, fort, strike) plays
    // through `larder_world_ai`, as the census does, so a campaign mission's
    // pictures show the hands its numbers were set on.
    let mut ai = match hand {
        "rush" | "farm" | "stand" | "volley" | "edge" | "squads" => None,
        _ => app.game.as_ref().and_then(|g| slime_mould::mould::larder_world_ai::Hand::named(hand, g)),
    };
    // Each hit: where, its tick, the body there the tick before, and the
    // ticks it took to refill.
    type Hole = ((f32, f32), u64, f32, Option<u64>);
    let mut holes: Vec<Hole> = Vec::new();
    let mut prev_lk: Vec<u8> = Vec::new();
    // `cut=T,X,Y,R` (2026-11-13, the slime-enemy thread: does a part cut
    // off from every crystal wither?): at game time T, a ring 6 world cells
    // wide (the fine grid, as `centre=` and the squad's places) from R out
    // round (X, Y) goes off the agar for the rest of the round (`Engine::block`, as a wall the game built but
    // with no health), so the cut is held. Each game second after, the
    // walkers inside the ring (the look's walker count, channel 1) are
    // printed against their count at the cut.
    let mut cut_inside: Option<(Vec<usize>, u32)> = None;
    let mut cut_sec = u64::MAX;
    // With HOLD (`cut=T,X,Y,R,HOLD`), the ring's agar cells go back on the
    // agar HOLD game seconds after the cut, so a piece that has gone stale
    // inside can leave (does a cut-off part panic?).
    let mut ring_back: Option<(f32, Vec<usize>)> = None;
    // The soldiers `cutsquad=` set down, kept there.
    let mut squad_set: Option<Vec<usize>> = None;
    let mut boomed: Vec<usize> = Vec::new();
    let inside = |lk: &[u8], cells: &[usize]| -> u32 { cells.iter().map(|&i| lk[4 * i + 1] as u32).sum() };
    for &t in at {
        // A mission's verdict stops the steps, as the game's end does.
        while app.game.as_ref().is_some_and(|g| g.time() < t && g.end.is_none()) && app.mission.as_ref().is_none_or(|m| m.verdict.is_none()) {
            let g = app.game.as_mut().unwrap();
            let sec = g.world.ticks() / g.rules.tps as u64;
            if sec != last_sec {
                last_sec = sec;
                match ai.as_mut() {
                    Some(h) => {
                        h.play(g);
                    }
                    None => plan(g, hand),
                }
            }
            if let (Some(Cut { t: ct, x: cx, y: cy, r: cr, width: cw, .. }), None) = (cut, cut_inside.as_ref()) {
                let g = app.game.as_mut().unwrap();
                if g.time() >= ct {
                    let (w, h) = (g.world.e.dish.w, g.world.e.dish.h);
                    let (mut ring, mut core) = (Vec::new(), Vec::new());
                    for i in 0..w * h {
                        let d = ((i % w) as f32 + 0.5 - cx).hypot((i / w) as f32 + 0.5 - cy);
                        if d > cr && d <= cr + cw {
                            ring.push(i);
                        } else if d <= cr {
                            core.push(i);
                        }
                    }
                    if let Some(c) = cut.filter(|c| c.hold.is_finite()) {
                        let agar: Vec<usize> = ring.iter().copied().filter(|&i| g.world.e.dish.agar[i]).collect();
                        ring_back = Some((g.time() + c.hold, agar));
                    }

                    g.world.e.block(&ring);
                    let n0 = inside(&read_back(&device, &queue, &g.world.e.look), &core);
                    println!("  cut t {:.1} s: ring of {} fine cells round ({cx}, {cy}) r {cr}; {n0} walkers inside", g.time(), ring.len());
                    cut_inside = Some((core, n0.max(1)));
                }
            }
            // `cutsquad=X,Y[,T[,N]]`: at T the first N living soldiers (a
            // squad short of N is made up first) are set down in a row there (a staged scene, as the ring is: a
            // squad walking there through C's body dies on the way); from
            // then every one of them not near it is sent back, after the
            // hand's orders, so it stays. The rest stay with the hand.
            if let Some((sx, sy, st, n)) = cut.and_then(|c| c.squad) {
                let g = app.game.as_mut().unwrap();
                if g.time() >= st && squad_set.is_none() {
                    // With N given, a squad short of N is made up first,
                    // bought at the larder with food the scene adds.
                    if n != usize::MAX {
                        while g.alive() < n.min(g.rules.max_squad) {
                            g.bank += g.rules.soldier_cost;
                            if !g.buy() {
                                break;
                            }
                        }
                    }
                    let set: Vec<usize> = (0..g.soldiers.len()).filter(|&k| g.soldiers[k].alive).take(n).collect();
                    let sc = g.world.scale();
                    for (j, &k) in set.iter().enumerate() {
                        (g.soldiers[k].x, g.soldiers[k].y) = (sx + (j as f32 - (set.len() as f32 - 1.0) / 2.0) * 1.5 * sc, sy);
                    }
                    squad_set = Some(set);
                }
                if let Some(set) = squad_set.as_ref() {
                    for &k in set {
                        let near = (g.soldiers[k].x - sx).hypot(g.soldiers[k].y - sy) < 2.0 * g.world.scale();
                        if g.soldiers[k].alive && !near && g.soldiers[k].order != Order::Move(sx, sy) {
                            g.apply(Cmd::Order(k, Order::Move(sx, sy)));
                        }
                    }
                }
            }
            if ring_back.as_ref().is_some_and(|(at, _)| app.game.as_ref().is_some_and(|g| g.time() >= *at)) {
                let (_, cells) = ring_back.take().unwrap();
                let g = app.game.as_mut().unwrap();
                g.world.e.unblock(&cells);
                println!("  cut t {:.1} s: the ring opens ({} cells back on the agar)", g.time(), cells.len());
            }
            // `boom=T,X,Y`: the round set off there once T has come.
            for (j, &(bt, bx, by)) in booms.iter().enumerate() {
                let g = app.game.as_mut().unwrap();
                if g.time() >= bt && !boomed.contains(&j) {
                    boomed.push(j);
                    let (k0, f0) = (app.rounds.killed, app.rounds.flung);
                    app.rounds.cast_at(g, (bx, by));
                    println!("  boom t {:.2} s at {bx},{by}: {} killed, {} flung", g.time(), app.rounds.killed - k0, app.rounds.flung - f0);
                }
            }
            app.step(step);
            // Whole steps here (the window leaves a step running while it
            // draws): the pictures, the holes and the tape read the game as
            // they did before the step split.
            app.settle();
            if let Some((core, n0)) = cut_inside.as_ref() {
                let g = app.game.as_ref().unwrap();
                // A step can run several ticks, so a second is counted when
                // it starts, not on a tick that is a multiple of `tps`.
                let sec = g.time().floor() as u64;
                if sec != cut_sec {
                    cut_sec = sec;
                    let n = inside(&read_back(&device, &queue, &g.world.e.look), core);
                    println!("  cut t {sec} s: {n} walkers inside ({:.2} of the cut's)", n as f32 / *n0 as f32);
                }
            }
            // `holes=1`: how long a hit's hole stays open (owner, 04:38Z:
            // "maybe part of the issue is the slime/walkers move to fast").
            // The drawn body (the view's `body`: trail x living walkers in
            // the 5 x 5 round each cell) is averaged over each hit's strike
            // disc, and a hole counts as refilled when that is back to 80% of
            // the tick before the hit. A frame's readback a tick: slow.
            if time_holes {
                let g = app.game.as_ref().unwrap();
                let (w, h) = (g.world.e.dish.w, g.world.e.dish.h);
                let lk = read_back(&device, &queue, &g.world.e.look);
                let body = |lk: &[u8], x: i32, y: i32| -> f32 {
                    let cl = |x: i32, y: i32| (y.clamp(0, h as i32 - 1) as usize) * w + x.clamp(0, w as i32 - 1) as usize;
                    let mut n = 0u32;
                    for dy in -2..=2 { for dx in -2..=2 { n += lk[4 * cl(x + dx, y + dy) + 1] as u32; } }
                    lk[4 * cl(x, y)] as f32 / 255.0 * (1.0 - (-(n as f32) / 6.0).exp())
                };
                let disc = |lk: &[u8], cx: f32, cy: f32, r: f32| -> f32 {
                    let (mut sum, mut k) = (0.0, 0);
                    let ri = r.ceil() as i32;
                    for dy in -ri..=ri { for dx in -ri..=ri { if ((dx * dx + dy * dy) as f32) <= r * r { sum += body(lk, cx as i32 + dx, cy as i32 + dy); k += 1; } } }
                    sum / k as f32
                };
                let tick = g.world.ticks();
                let r = g.rules.strike * g.world.scale();
                for t in app.tracers.iter().filter(|t| t.age == 0.0 && t.hit) {
                    let before = if prev_lk.is_empty() { f32::NAN } else { disc(&prev_lk, t.to.0, t.to.1, r) };
                    holes.push((t.to, tick, before, None::<u64>));
                }
                for hl in holes.iter_mut() {
                    if hl.2.is_nan() { continue; }
                    let v = disc(&lk, hl.0 .0, hl.0 .1, r);
                    if hl.3.is_none() && tick > hl.1 && v >= 0.8 * hl.2.max(0.05) { hl.3 = Some(tick - hl.1); }
                }
                prev_lk = lk;
            }
            if app.tape.is_some() {
                let g = app.game.as_ref().unwrap();
                let alive: Vec<(f32, f32)> = g.soldiers.iter().filter(|s| s.alive).map(|s| (s.x, s.y)).collect();
                let n = alive.len().max(1) as f32;
                let centre = if alive.is_empty() { g.larder } else { (alive.iter().map(|p| p.0).sum::<f32>() / n, alive.iter().map(|p| p.1).sum::<f32>() / n) };
                app.hear(step, Ear { centre, half: (span.0 / 2.0, span.1 / 2.0) });
                // The mould's voice's inputs every 5 game seconds, beside
                // the recording.
                let g = app.game.as_ref().unwrap();
                let sec = (g.time() + 1e-3) as u64;
                if sec.is_multiple_of(5) && sec != voice_at {
                    voice_at = sec;
                    let s = app.listener.as_ref().unwrap().stats;
                    println!("  voice t {sec:>3} s: mould {}, growth {:+.4}/s, hunger {:.2}, eating {:.5}/s, kills {:.0}/s, inhales {}", g.mould, s.growth, s.hunger, s.eating, s.kills_per_sec, s.inhales);
                }
            }
        }
        if time_holes {
            let done: Vec<u64> = holes.iter().filter_map(|h| h.3).collect();
            let open = holes.iter().filter(|h| h.3.is_none()).count();
            let mut d = done.clone(); d.sort();
            let tps = app.game.as_ref().unwrap().rules.tps as f32;
            if !d.is_empty() { println!("  holes: shots that hit {} refilled {} (still open {open}); ticks to 80%: median {} ({:.2} s), q25 {}, q75 {}, max {}", holes.len(), d.len(), d[d.len() / 2], d[d.len() / 2] as f32 / tps, d[d.len() / 4], d[3 * d.len() / 4], d[d.len() - 1]); }
        }
        let g = app.game.as_ref().unwrap();
        let rd = &app.rounds;
        if !rd.round.is_empty() {
            println!("  rounds t {:>4.0} s: {} casts (blast, gas, germ, bomb {:?}), {} walkers killed, {} flung, {:.0} food spent, {} skipped for food, {} not on this world, {} lodged", g.time(), rd.cast.iter().sum::<u32>(), rd.cast, rd.killed, rd.flung, rd.spent, rd.broke, rd.unsupported, rd.waiting());
        }
        // The view on the fight: the squad's middle, or the larder; or,
        // with `centre=X,Y`, one place for every frame, so a run of frames
        // plays as a clip from a still camera (the crater looks in motion,
        // 2026-11-12: a camera on the squad pans as it walks and jumps to
        // the larder when it dies).
        let alive: Vec<(f32, f32)> = g.soldiers.iter().filter(|s| s.alive).map(|s| (s.x, s.y)).collect();
        let squad = (!alive.is_empty()).then(|| (alive.iter().map(|p| p.0).sum::<f32>() / alive.len() as f32, alive.iter().map(|p| p.1).sum::<f32>() / alive.len() as f32));
        // The squad mode follows the controlled squad, as the window does.
        let squad = g.active.and_then(|q| g.middle(q)).filter(|_| g.rules.squads).or(squad);
        let centre = fixed.map(|f| (f.0, f.1)).or(squad).unwrap_or(g.larder);
        app.controls.selected = match g.active.filter(|_| g.rules.squads) {
            Some(q) => g.members(q),
            None if select => (0..g.soldiers.len()).collect(),
            None => Vec::new(),
        };
        app.hud.moved(hover);
        // The HUD sets the camera's insets as it draws, and `fit` reads
        // them: without this the run's first "all" picture was zoomed in
        // past the world's edges (fit with no insets).
        app.prepare();
        for tag in ["all", "near"] {
            let zoom = if tag == "all" { app.camera.fit() } else { fixed.map_or(6.0, |f| f.2) };
            app.camera.zoom_to(zoom, app.camera.to_screen(app.camera.centre()));
            app.camera.centre_on(if tag == "all" { (app.camera.world.0 / 2.0, app.camera.world.1 / 2.0) } else { centre });
            app.prepare();
            let (view, cp) = (app.view.as_ref().unwrap(), app.canvas_pass.as_mut().unwrap());
            let mut enc = device.create_command_encoder(&wgpu::CommandEncoderDescriptor { label: Some("shot") });
            view.record(&mut enc, &app.camera, None);
            cp.record(&mut enc, &view.out, view.stride);
            queue.submit([enc.finish()]);
            // Whole seconds keep their old names; a time between (a frame
            // sequence) is named in milliseconds.
            let path = if t.fract() == 0.0 { format!("{dir}/{}-{hand}-{:03}s-{tag}.png", app.name.replace('/', "_"), t as u32) } else { format!("{dir}/{}-{hand}-{:05}ms-{tag}.png", app.name.replace('/', "_"), (t * 1000.0) as u32) };
            view.picture().save(&path).map_err(|e| format!("{path}: {e}"))?;
            println!("{path}");
        }
        let g = app.game.as_ref().unwrap();
        let left: f32 = g.world.crystals().iter().map(|c| c.1).sum::<f32>() / app.charge0.iter().sum::<f32>().max(1.0);
        // The CPU work the window adds: the ground field (once a game
        // second) and the model and HUD layout (every frame).
        let cells = g.world.e.dish.w * g.world.e.dish.h;
        let t0 = Instant::now();
        let _ = ground_codes(g, &app.depth, Some(&vec![256.0; cells]), Some(&vec![100.0; cells]));
        let ground_ms = t0.elapsed().as_secs_f64() * 1e3;
        let t0 = Instant::now();
        for _ in 0..20 {
            let m = app.model(None, None);
            app.canvas.as_mut().unwrap().clear();
            app.hud.draw(&m, &app.controls, &app.camera, true, app.canvas.as_mut().unwrap(), &mut Marks::default());
        }
        println!("  CPU: ground field {ground_ms:.2} ms (once a game second), model and HUD {:.3} ms a frame", t0.elapsed().as_secs_f64() * 1e3 / 20.0);
        let g = app.game.as_ref().unwrap();
        if g.world.garden.is_some() {
            println!("  t {:.0} s: own mould {} (green), enemy {}", g.time(), g.world.counts_by_side().0, g.mould);
        }
        println!("  t {:.0} s: mould {}, soldiers {}, bank {:.0}, harvested {:.0}, shots {}, killed {}, took {:.0}%, crystals left {:.3}{}{}", g.time(), g.mould, g.alive(), g.bank, g.harvested, g.shots, g.killed, g.take_share() * 100.0, left, g.end.map_or(String::new(), |e| format!(", ended: {}", e.band())), squad.map_or(String::new(), |(x, y)| format!("; squad at {x:.1},{y:.1}")));
        // What holds the live soldiers (2026-11-13, for the wading and
        // arming clips): walkers touching them, and their health.
        let live: Vec<_> = g.soldiers.iter().filter(|s| s.alive).collect();
        if !live.is_empty() {
            let n = live.len() as f32;
            println!("  squad: touching {:.1} mean, {} most; health {:.0} mean", live.iter().map(|s| s.touching as f32).sum::<f32>() / n, live.iter().map(|s| s.touching).max().unwrap_or(0), live.iter().map(|s| s.hp).sum::<f32>() / n);
        }
        for b in &g.buildings {
            println!("    {} at ({:.0}, {:.0}): hp {:.0} / {:.0}, built {:.1}, {}{}", b.kind.name(), b.at.0, b.at.1, b.hp, b.max_hp, b.built, if b.alive { "standing" } else { "worn down" }, if b.working { ", working" } else { "" });
        }
        if !g.buildings.is_empty() {
            println!("    pumped {:.0} food, lures burnt {:.0}", g.pumped, g.lured);
        }
    }
    if let (Some(path), Some((bank, tape))) = (wav, app.tape.as_ref()) {
        slime_mould::wav::write(Path::new(path), tape, 2, bank.sample_rate()).map_err(|e| format!("{path}: {e}"))?;
        let peak = tape.iter().fold(0.0f32, |m, v| m.max(v.abs()));
        let rms = (tape.iter().map(|v| v * v).sum::<f32>() / tape.len().max(1) as f32).sqrt();
        println!("{path}: {:.1} s, peak {:.3}, rms {:.4} ({:.1} dBFS)", tape.len() as f32 / 2.0 / bank.sample_rate() as f32, peak, rms, 20.0 * rms.max(1e-9).log10());
        let s = app.listener.as_ref().map(|l| l.stats).unwrap_or_default();
        println!(
            "  heard: shots {} (voiced {}), impacts {} (light {} mid {} heavy {}), kills {}, beats {}, hurts {}, fallen {}, banked {}, bought {}, walls {}, dry {}, cores {}, endings {}",
            s.shots, s.voiced, s.impacts, s.light, s.mid, s.heavy, s.kills, s.beats, s.hurts, s.fallen, s.banked, s.bought, s.walls, s.dry, s.cores, s.endings
        );
        println!(
            "  heard (C7): zips {}, sirens {}, blasts {}, pieces torn {} rejoined {} withered out {}, tells {} (charges {}); at the end {} pieces losing {:.0} walkers/s",
            s.zips, s.sirens, s.blasts, s.torn, s.rejoined, s.withered, s.tells, s.charges, s.pieces, s.withering
        );
        let b = bank.stats;
        println!("  played: grains {} (touch {} stolen {} kills {} wither {}, verbs {}), bursts {}, alerts {}, steals {}", b.grains, b.grains_by_texture[0], b.grains_by_texture[1], b.grains_by_texture[2], b.grains_by_texture[3], b.verb_grains, b.bursts, b.alerts, b.steals);
        let us = &app.hear_us;
        println!("  listener: mean {:.1} us, max {:.1} us a frame over {} frames", us.iter().sum::<f32>() / us.len().max(1) as f32, us.iter().fold(0.0f32, |a, &b| a.max(b)), us.len());
    }
    Ok(())
}

/// Notes for the soldiers the game made since the last call: one bought
/// (by the card or a replay) or the reserve's free recruit
/// (`Rules::reserve`), which `finish_step` makes.
pub(super) fn join_looks(g: &Game<GpuWorld>, looks: &mut Vec<Look>) {
    for s in &g.soldiers[looks.len()..] {
        looks.push(Look { facing: -std::f32::consts::FRAC_PI_2, flash: 0.0, take: 0.0, at: (s.x, s.y), alive: s.alive });
    }
}

/// The last step's shots as tracers, and their soldiers' flashes and
/// facing (before the next step walks them on). A recruit made in one step
/// of a frame can fire in the next step of the same frame (fast forward, or
/// a slow frame), before the frame's own `join_looks`, so its note joins
/// here (2026-11-12, after #114's panic on a recruit with no note).
pub(super) fn shot_marks(g: &Game<GpuWorld>, tracers: &mut Vec<Tracer>, looks: &mut Vec<Look>, listener: Option<&mut WorldListener>) {
    if let Some(l) = listener {
        l.shots(&g.last_shots);
    }
    join_looks(g, looks);
    for s in &g.last_shots {
        // Each leg is where the bullet flew this frame (game state, not an
        // effect), so a bullet draws as a short streak moving out.
        tracers.push(Tracer { from: s.from, to: s.hit.unwrap_or(s.to), hit: s.hit.is_some(), age: 0.0 });
        if !s.fired {
            continue;
        }
        if let Some(k) = s.by.filter(|&k| k < looks.len() && g.soldiers.get(k).is_some_and(|q| q.alive)) {
            looks[k].flash = FLASH;
            looks[k].facing = (s.to.1 - s.from.1).atan2(s.to.0 - s.from.0);
        }
    }
    // A harvester taking from a crystal turns to face it and its hands
    // light (harvest look, 11-14); a shot this step still sets its facing
    // above, as firing wins.
    let cs = g.world.crystals();
    for t in &g.last_takes {
        if let (Some(k), Some(c)) = (t.by.filter(|&k| k < looks.len()), cs.get(t.crystal)) {
            looks[k].take = TAKE;
            if looks[k].flash <= 0.0 {
                looks[k].facing = (c.0 .1 - t.at.1).atan2(c.0 .0 - t.at.0);
            }
        }
    }
}
