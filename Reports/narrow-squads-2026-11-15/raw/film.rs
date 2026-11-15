
/// Film of a big squad crossing a narrow place (narrow squads report,
/// 2026-11-15): `FILM=DIR FILM_MAP=PATH/NAME FILM_N=24 FILM_FUNNEL=0|1
/// FILM_GOAL=X,Y FILM_CENTRE=X,Y`. Appended to src/bin/larder-world/tests.rs
/// only while filming.
#[test]
#[ignore]
fn film_narrow() {
    let env = |k: &str| std::env::var(k).unwrap();
    let pair = |k: &str| { let v = env(k); let (a, b) = v.split_once(',').unwrap(); (a.parse::<f32>().unwrap(), b.parse::<f32>().unwrap()) };
    let dir = env("FILM");
    let Some((device, queue)) = gpu() else { panic!("no gpu") };
    let mut args = squad_args(true);
    args.map = Some(env("FILM_MAP"));
    let mut app = offscreen(args, (960, 540), &device, &queue).unwrap();
    let n: usize = env("FILM_N").parse().unwrap();
    {
        let g = app.game.as_mut().unwrap();
        g.rules.funnel = env("FILM_FUNNEL") != "0";
        g.rules.max_squad = 64;
        g.bank = 1e6;
        while g.alive() < n {
            g.apply(Cmd::Buy);
        }
    }
    let dt = 1.0 / 30.0;
    for _ in 0..60 { app.squad_frame(dt); app.step(dt); }
    let goal = pair("FILM_GOAL");
    let centre = pair("FILM_CENTRE");
    app.game.as_mut().unwrap().apply(Cmd::SquadOrder(0, Order::Move(goal.0, goal.1)));
    let mut f = 0;
    for i in 0..(16.0 / dt) as usize {
        app.squad_frame(dt);
        if let Some(sq) = app.sq.as_mut() { sq.follow = false; }
        app.camera.centre_on(centre);
        app.camera.zoom_to(6.0, app.camera.to_screen(centre));
        app.camera.centre_on(centre);
        app.step(dt);
        if i % 2 == 0 {
            app.marks();
            app.prepare();
            let (view, cp) = (app.view.as_ref().unwrap(), app.canvas_pass.as_mut().unwrap());
            let mut enc = device.create_command_encoder(&wgpu::CommandEncoderDescriptor { label: Some("film") });
            view.record(&mut enc, &app.camera, None);
            cp.record(&mut enc, &view.out, view.stride);
            queue.submit([enc.finish()]);
            view.picture().save(format!("{dir}/f{f:04}.png")).unwrap();
            f += 1;
        }
    }
}
