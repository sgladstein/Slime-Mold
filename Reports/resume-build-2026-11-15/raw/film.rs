// Ignored film test: append to src/bin/larder-world/tests.rs and run
// FILM=DIR cargo test --release --bin larder-world film_resume_build -- --ignored --nocapture

#[test]
#[ignore]
fn film_resume_build() {
    let dir = std::env::var("FILM").unwrap();
    let Some((device, queue)) = gpu() else { panic!("no gpu") };
    let mut app = offscreen(squad_args(true), (960, 600), &device, &queue).unwrap();
    app.game.as_mut().unwrap().bank = 2000.0;
    let dt = 1.0 / 30.0;
    let mut n = 0;
    let mut shot = |app: &mut App| {
        app.marks();
        app.prepare();
        let (view, cp) = (app.view.as_ref().unwrap(), app.canvas_pass.as_mut().unwrap());
        let mut enc = device.create_command_encoder(&wgpu::CommandEncoderDescriptor { label: Some("film") });
        view.record(&mut enc, &app.camera, None);
        cp.record(&mut enc, &view.out, view.stride);
        queue.submit([enc.finish()]);
        view.picture().save(format!("{dir}/f{n:04}.png")).unwrap();
        n += 1;
    };
    app.squad_frame(dt);
    app.camera.zoom_to(6.0, app.camera.to_screen(app.camera.centre()));
    for _ in 0..10 { app.squad_frame(dt); app.step(dt); }
    let (q, m, sc) = { let g = app.game.as_ref().unwrap(); let q = g.active.unwrap(); (q, g.middle(q).unwrap(), g.world.scale()) };
    let at = (0..48).map(|k| { let a = k as f32 * 0.785; let r = 12.0 + (k / 8) as f32 * 3.0; (m.0 + a.cos() * r * sc, m.1 + a.sin() * r * sc) })
        .find(|&p| { let g = app.game.as_ref().unwrap(); g.build_blocked(Kind::Turret, p).is_none() && (-3..=3).all(|i| (-3..=3).all(|j| g.crystal_at((p.0 + i as f32 * sc, p.1 + j as f32 * sc)).is_none())) }).unwrap();
    eprintln!("FILM at {at:?} m {m:?}");
    let away = (m.0 - (at.0 - m.0) * 3.0, m.1 - (at.1 - m.1) * 3.0);
    let point = |app: &mut App, w: (f32, f32)| { let c = app.camera.to_screen(w); app.controls.cursor = Some(c); app.hud.moved(Some(c)); };
    point(&mut app, at);
    for _ in 0..10 { app.squad_frame(dt); app.step(dt); shot(&mut app); }
    app.hud_event(HudEvent::Point { id: TURRET, at }).unwrap();
    let b = app.game.as_ref().unwrap().buildings.len() - 1;
    let mut i = 0;
    while app.game.as_ref().unwrap().buildings[b].built < 0.4 && i < 600 { app.squad_frame(dt); app.step(dt); if i % 2 == 0 { shot(&mut app); } i += 1; }
    point(&mut app, away);
    app.game.as_mut().unwrap().apply(Cmd::SquadOrder(q as u8, Order::Move(away.0, away.1)));
    for i in 0..150 { app.squad_frame(dt); app.step(dt); if i % 2 == 0 { shot(&mut app); } }
    point(&mut app, at);
    for _ in 0..8 { app.squad_frame(dt); app.step(dt); shot(&mut app); }
    let bank = app.game.as_ref().unwrap().bank;
    assert!(app.squad_button(Button::Right, true));
    app.squad_button(Button::Right, false);
    let mut i = 0;
    while app.game.as_ref().unwrap().buildings[b].built < 1.0 && i < 2000 { app.squad_frame(dt); app.step(dt); if i % 2 == 0 { shot(&mut app); } i += 1; }
    for _ in 0..30 { app.squad_frame(dt); app.step(dt); shot(&mut app); }
    let g = app.game.as_ref().unwrap();
    eprintln!("FILM bank before click {bank:.1} after {:.1}; buildings {}", g.bank, g.buildings.len());
}
