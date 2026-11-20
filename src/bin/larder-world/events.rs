//! The window's event loop (`ApplicationHandler`). Moved out of
//! `larder-world.rs` unchanged (C9, 2026-11-14).

use super::*;

impl ApplicationHandler for App {
    fn resumed(&mut self, event_loop: &ActiveEventLoop) {
        if self.window.is_some() {
            return;
        }
        let room = event_loop.primary_monitor().map_or((1600, 900), |m| (m.size().width * 9 / 10, m.size().height * 9 / 10));
        let first = PhysicalSize::new(room.0.clamp(640, 1600), room.1.clamp(480, 1000));
        let attributes = Window::default_attributes().with_title("Larder World").with_inner_size(first);
        let window = match event_loop.create_window(attributes) {
            Ok(w) => Arc::new(w),
            Err(err) => return self.fail(event_loop, format!("window creation failed: {err}")),
        };
        // The engine binds 46 buffers in a pass: ask for the adapter's own
        // limits, not wgpu's defaults (8 storage buffers a stage).
        let instance = wgpu::Instance::new(wgpu::InstanceDescriptor::new_without_display_handle_from_env());
        let options = wgpu::RequestAdapterOptions { power_preference: wgpu::PowerPreference::HighPerformance, compatible_surface: None, force_fallback_adapter: false };
        let Ok(adapter) = block_on(instance.request_adapter(&options)) else { return self.fail(event_loop, "no GPU adapter".into()) };
        let size = window.inner_size();
        let (w, h) = (size.width.max(1), size.height.max(1));
        let desc = wgpu::DeviceDescriptor { label: Some("larder-world"), required_limits: adapter.limits(), ..Default::default() };
        let mut pixels = match PixelsBuilder::new(w, h, SurfaceTexture::new(size.width, size.height, Arc::clone(&window))).request_adapter_options(options).device_descriptor(desc).build() {
            Ok(p) => p,
            Err(err) => return self.fail(event_loop, format!("pixels init failed: {err}")),
        };
        pixels.set_scaling_mode(ScalingMode::PixelPerfect);
        // pixels 0.17 works out its scale only on a resize (mouldgpu).
        if let Err(err) = pixels.resize_surface(size.width, size.height) {
            return self.fail(event_loop, format!("surface resize failed: {err}"));
        }
        let info = pixels.adapter().get_info();
        println!("larder-world: {} ({:?})", info.name, info.backend);
        // A placeholder world frame until the round starts.
        let frame = pixels.device().create_buffer(&wgpu::BufferDescriptor { label: Some("no world"), size: 16, usage: wgpu::BufferUsages::STORAGE, mapped_at_creation: false });
        self.view = Some(View::new(pixels.device(), pixels.queue(), &frame, (1, 1), (w, h)));
        // The UI size: the saved or `ui=` size over the window's DPI scale.
        let mut canvas = Canvas::new((w, h));
        canvas.ui = UiSize { user: ui_user(), dpi: window.scale_factor() as f32 };
        self.canvas = Some(canvas);
        self.canvas_pass = Some(CanvasPass::new(pixels.device(), pixels.queue()));
        self.gpu = Some((pixels.device().clone(), pixels.queue().clone()));
        self.pixels = Some(pixels);
        self.window = Some(window);
        self.speaker = match sound::Output::open(self.args.seed) {
            // At the saved volume (`volume`), not the first-run level.
            Ok(mut o) => {
                o.send([master_now(self.sound_on)]);
                Some(o)
            }
            Err(e) => {
                eprintln!("larder-world: no sound ({e}); playing silent");
                None
            }
        };
        let opened = if self.in_menu() { self.open_menu() } else { self.start() };
        if let Err(e) = opened {
            self.fail(event_loop, e);
        }
    }

    fn window_event(&mut self, event_loop: &ActiveEventLoop, _id: WindowId, event: WindowEvent) {
        match event {
            WindowEvent::CloseRequested => {
                self.save_replay();
                event_loop.exit();
            }
            WindowEvent::Resized(size) => {
                if size.width > 0 && size.height > 0 {
                    if let Some(p) = self.pixels.as_mut() {
                        let res = p.resize_buffer(size.width, size.height).map_err(|e| e.to_string()).and_then(|_| p.resize_surface(size.width, size.height).map_err(|e| e.to_string()));
                        if let Err(err) = res {
                            return self.fail(event_loop, format!("resize failed: {err}"));
                        }
                    }
                    if let Some(v) = self.view.as_mut() {
                        v.resize((size.width, size.height));
                    }
                    self.camera.resize((size.width, size.height));
                    if let Some(c) = self.canvas.as_mut() {
                        c.screen = (size.width, size.height);
                    }
                    if self.in_menu() && !self.armory {
                        self.menu_camera();
                    }
                }
            }
            WindowEvent::ScaleFactorChanged { scale_factor, .. } => {
                // Dragged to a screen with another DPI scale.
                if let Some(c) = self.canvas.as_mut() {
                    c.ui.dpi = scale_factor as f32;
                }
            }
            WindowEvent::ModifiersChanged(m) => {
                self.shift = m.state().shift_key();
                self.ctrl = m.state().control_key();
                (self.controls.shift, self.controls.ctrl) = (self.shift, self.ctrl);
            }
            WindowEvent::KeyboardInput { event, .. } => {
                let PhysicalKey::Code(code) = event.physical_key else { return };
                let pressed = event.state == ElementState::Pressed;
                // Ctrl + and Ctrl - size the UI, in play and on the menu.
                if pressed && self.ctrl && self.ui_key(code) {
                    return;
                }
                // F9 drops a note in the play log (`App::drop_note`), in
                // play only and whatever else is open, so it is never the
                // wrong moment to mark one.
                if pressed && !event.repeat && code == KeyCode::F9 && !self.in_menu() {
                    self.drop_note();
                    return;
                }
                // The start menu takes every key while it is up.
                if self.in_menu() {
                    let repeat = event.repeat && !matches!(code, KeyCode::ArrowUp | KeyCode::ArrowDown);
                    match self.menu_input(|m| if repeat { Vec::new() } else { m.key(code, pressed) }) {
                        Ok(picks) => self.menu_events(picks, event_loop),
                        Err(e) => self.fail(event_loop, e),
                    }
                    return;
                }
                // Paused, WASD pans the camera as the arrows do (owner's
                // playtest, 2026-11-14 20:12Z); unpaused it steers the squad
                // and is the card's letters, as before. A key let go while
                // paused still lets go of a steer held from before the pause.
                // With no squad controlled (Esc, `SquadInput::free`) it pans
                // unpaused too.
                self.controls.wasd_pan = (self.paused || self.squad_free()) && self.game.as_ref().is_some_and(|g| g.end.is_none());
                if self.controls.wasd_pan && !self.hud.menu_open() && slime_mould::world::controls::wasd_index(code).is_some() {
                    if !pressed {
                        self.squad_key(code, false, event.repeat, true);
                    }
                    let units = self.units();
                    self.controls.key(code, pressed, event.repeat, &units, &mut self.camera);
                    return;
                }
                // The squad mode's steering (WASD) and Space come before the
                // card's letters, and its own letters after them
                // (`squad_key`; bug sweep, 2026-11-14: they all came first,
                // so G merged instead of harvesting). The menus take
                // every key while they are up.
                if (!pressed || !self.hud.menu_open()) && self.squad_key(code, pressed, event.repeat, true) {
                    return;
                }
                // The game UI first: its command keys, Esc for its menu,
                // Space for the newest alert.
                if !event.repeat {
                    let (took, events) = self.hud.key(code, pressed, self.ctrl, &mut self.camera);
                    self.hud_events(events, event_loop);
                    if took {
                        return;
                    }
                }
                if self.squad_key(code, pressed, event.repeat, false) {
                    return;
                }
                let units = self.units();
                // The controls: arrows pan, digits are groups, F+drag a burn.
                for c in self.controls.key(code, pressed, event.repeat, &units, &mut self.camera) {
                    if let UiCommand::Key { code, .. } = c {
                        self.key(code, event_loop);
                    }
                }
            }
            WindowEvent::CursorMoved { position, .. } => {
                let at = (position.x as f32, position.y as f32);
                if let Some(m) = self.menu.as_mut() {
                    m.moved(Some(at));
                    return;
                }
                let mm = self.view.as_ref().and_then(|v| v.minimap_rect(&self.camera));
                self.hud.moved(Some(at));
                self.controls.moved(Some(at), &mut self.camera, mm);
            }
            WindowEvent::CursorLeft { .. } => {
                if let Some(m) = self.menu.as_mut() {
                    m.moved(None);
                }
                self.hud.moved(None);
                self.controls.moved(None, &mut self.camera, None);
            }
            WindowEvent::Focused(focused) => {
                self.controls.edge = focused;
                if !focused {
                    self.controls.unfocus();
                    self.squad_unfocus();
                }
            }
            WindowEvent::MouseWheel { delta, .. } => {
                let notches = match delta {
                    MouseScrollDelta::LineDelta(_, y) => y.signum() as i32,
                    MouseScrollDelta::PixelDelta(p) => (p.y / 40.0).round() as i32,
                };
                if self.in_menu() {
                    // A page taller than the window scrolls.
                    if let Some(m) = self.menu.as_mut() {
                        m.wheel(notches);
                    }
                    return;
                }
                if self.squad_wheel(notches) || self.squad_zoom(notches) {
                    return;
                }
                self.controls.wheel(notches, &mut self.camera);
            }
            WindowEvent::MouseInput { state, button, .. } => {
                let down = state == ElementState::Pressed;
                let b = match button {
                    MouseButton::Left => Button::Left,
                    MouseButton::Right => Button::Right,
                    MouseButton::Middle => Button::Middle,
                    _ => return,
                };
                if self.in_menu() {
                    match self.menu_input(|m| m.button(b, down)) {
                        Ok(picks) => {
                            self.click_sound(&picks);
                            self.menu_events(picks, event_loop)
                        }
                        Err(e) => self.fail(event_loop, e),
                    }
                    return;
                }
                let (took, events) = self.hud.button(b, down, &self.camera, self.shift);
                if !events.is_empty() {
                    self.ui_sound(&[slime_mould::sound::SoundCmd::Grain { kind: slime_mould::sound::GrainKind::Tick, pan: 0.0, gain: 0.45, delay: 0.0 }]);
                }
                self.hud_events(events, event_loop);
                if took {
                    return;
                }
                // Over the HUD the card took it; on the world the squad mode
                // fires, sends and switches (Shift + drag still boxes).
                if self.squad_button(b, down) {
                    return;
                }
                let units = self.units();
                let mm = self.view.as_ref().and_then(|v| v.minimap_rect(&self.camera));
                let point = self.controls.cursor.and_then(|c| self.camera.world_at(c));
                // The larder selected and no soldier: right click sets its
                // rally point (the controls give no order without a unit).
                if b == Button::Right && down && self.larder_sel && self.living(&self.controls.selected).is_empty() {
                    if let Some(at) = point {
                        self.rally(at);
                    }
                }
                for c in self.controls.button(b, down, &units, &mut self.camera, mm) {
                    match c {
                        UiCommand::Order { units, at } => self.order_at(&units, at),
                        UiCommand::Select(picked) if self.sq.is_some() => self.squad_picked(&picked),
                        UiCommand::Select(picked) => {
                            self.larder_sel = picked.is_empty() && self.on_larder(point);
                            // Nobody picked and no larder: a building under
                            // the click is selected (option b, 11-15).
                            self.building_sel = if picked.is_empty() && !self.larder_sel { point.and_then(|p| self.game.as_ref()?.building_at(p)) } else { None };
                        }
                        _ => {}
                    }
                }
            }
            WindowEvent::RedrawRequested => self.frame(event_loop),
            _ => {}
        }
    }

    fn about_to_wait(&mut self, _event_loop: &ActiveEventLoop) {
        if let Some(window) = &self.window {
            window.request_redraw();
        }
    }
}

/// **The menus and the HUD answer a click** (sound pass two, 2026-11-20;
/// until then they were silent). Straight to the speaker: a click is the
/// player's, not the game's, so it never goes through the listener.
impl App {
    pub(super) fn ui_sound(&mut self, cmds: &[slime_mould::sound::SoundCmd]) {
        if let Some(sp) = self.speaker.as_mut().filter(|_| self.sound_on) {
            sp.send(cmds.iter().copied());
        }
    }

    /// A pick's sound: buying chimes, a card dropped in a slot knocks,
    /// landing on a planet whooshes, anything else ticks.
    fn click_sound(&mut self, picks: &[slime_mould::world::start::StartEvent]) {
        use slime_mould::sound::{AlertKind, GrainKind, SoundCmd};
        use slime_mould::world::start::StartEvent;
        let Some(p) = picks.first() else { return };
        let cmd = match p {
            StartEvent::Buy(_) => SoundCmd::Alert { kind: AlertKind::Joined, weight: 0.8, pan: 0.0 },
            StartEvent::Move(..) => SoundCmd::Grain { kind: GrainKind::Knock, pan: 0.0, gain: 0.3, delay: 0.0 },
            StartEvent::NextLevel | StartEvent::Run | StartEvent::Sandbox | StartEvent::Mission(_) | StartEvent::RandomPlanet => SoundCmd::Grain { kind: GrainKind::Whoosh, pan: 0.0, gain: 0.55, delay: 0.0 },
            _ => SoundCmd::Grain { kind: GrainKind::Tick, pan: 0.0, gain: 0.45, delay: 0.0 },
        };
        self.ui_sound(&[cmd]);
    }
}
