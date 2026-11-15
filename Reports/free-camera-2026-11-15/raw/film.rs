#[test]
#[ignore]
fn film_free_camera() {
    let dir = std::env::var("FILM").unwrap();
    let Some((device, queue)) = gpu() else { panic!("no gpu") };
    let mut app = offscreen(squad_args(true), (960, 600), &device, &queue).unwrap();
    app.game.as_mut().unwrap().bank = 1e6;
    for _ in 0..8 {
        app.game.as_mut().unwrap().apply(Cmd::Buy);
    }
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
    let run = |app: &mut App, frames: usize, shoot: &mut dyn FnMut(&mut App)| {
        for i in 0..frames {
            app.controls.update(&mut app.camera, dt);
            app.squad_frame(dt);
            app.step(dt);
            if i % 2 == 0 {
                shoot(app);
            }
        }
    };
    app.squad_frame(dt);
    app.camera.zoom_to(5.0, app.camera.to_screen(app.camera.centre()));
    for _ in 0..10 { app.squad_frame(dt); app.step(dt); }
    // Three squads: split twice; send the two let go off on orders.
    let q0 = app.game.as_ref().unwrap().active.unwrap();
    let m = app.game.as_ref().unwrap().middle(q0).unwrap();
    {
        let g = app.game.as_mut().unwrap();
        let a = g.split(q0, (m.0 + 30.0, m.1)).unwrap();
        let b = g.split(a, (m.0, m.1 + 30.0)).unwrap();
        g.apply(Cmd::SquadOrder(q0 as u8, Order::Move(m.0 - 60.0, m.1 - 20.0)));
        g.apply(Cmd::SquadOrder(a as u8, Order::Fight(m.0 + 70.0, m.1 + 30.0)));
        g.apply(Cmd::Active(b as u8));
    }
    // Steer the controlled one right.
    let units = app.units();
    app.squad_key(KeyCode::KeyD, true, false, true);
    run(&mut app, 60, &mut shot);
    app.squad_key(KeyCode::KeyD, false, false, true);
    run(&mut app, 10, &mut shot);
    // Esc: let go. WASD now pans.
    app.squad_key(KeyCode::Escape, true, false, true);
    run(&mut app, 30, &mut shot);
    for (code, frames) in [(KeyCode::KeyA, 50), (KeyCode::KeyW, 25), (KeyCode::KeyD, 70), (KeyCode::KeyS, 40)] {
        app.controls.wasd_pan = app.squad_free();
        app.controls.key(code, true, false, &units, &mut app.camera);
        run(&mut app, frames, &mut shot);
        app.controls.key(code, false, false, &units, &mut app.camera);
    }
    run(&mut app, 20, &mut shot);
    // Space takes the last squad back; the camera comes to it.
    app.squad_key(KeyCode::Space, true, false, true);
    run(&mut app, 50, &mut shot);
}
