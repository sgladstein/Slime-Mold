
#[test]
#[ignore]
fn film_order_lines() {
    let dir = std::env::var("FILM").unwrap();
    let Some((device, queue)) = gpu() else { panic!("no gpu") };
    let mut app = offscreen(squad_args(true), (960, 600), &device, &queue).unwrap();
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
    app.camera.zoom_to(5.0, app.camera.to_screen(app.camera.centre()));
    for _ in 0..10 { app.squad_frame(dt); app.step(dt); }
    let (q, m) = { let g = app.game.as_ref().unwrap(); let q = g.active.unwrap(); (q, g.middle(q).unwrap()) };
    let pts = [(m.0 + 50.0, m.1 - 10.0), (m.0 + 60.0, m.1 + 35.0), (m.0 + 10.0, m.1 + 45.0)];
    for _ in 0..8 { app.squad_frame(dt); app.step(dt); shot(&mut app); }
    {
        let g = app.game.as_mut().unwrap();
        g.apply(Cmd::SquadOrder(q as u8, Order::Move(pts[0].0, pts[0].1)));
    }
    for _ in 0..8 { app.squad_frame(dt); app.step(dt); shot(&mut app); }
    app.game.as_mut().unwrap().apply(Cmd::SquadQueue(q as u8, Order::Fight(pts[1].0, pts[1].1)));
    for _ in 0..8 { app.squad_frame(dt); app.step(dt); shot(&mut app); }
    app.game.as_mut().unwrap().apply(Cmd::SquadQueue(q as u8, Order::Move(pts[2].0, pts[2].1)));
    for i in 0..300 { app.squad_frame(dt); app.step(dt); if i % 2 == 0 { shot(&mut app); } }
    // Pause, then WASD pans.
    app.paused = true;
    let units = app.units();
    for (code, frames) in [(KeyCode::KeyD, 30), (KeyCode::KeyS, 20), (KeyCode::KeyA, 30), (KeyCode::KeyW, 20)] {
        app.controls.wasd_pan = true;
        app.controls.key(code, true, false, &units, &mut app.camera);
        for _ in 0..frames { app.controls.update(&mut app.camera, dt); app.squad_frame(dt); app.step(dt); shot(&mut app); }
        app.controls.key(code, false, false, &units, &mut app.camera);
    }
}
