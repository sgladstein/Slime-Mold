//! **The squad mode's input** (`mode=squads`; `larder_world::squads`,
//! owner 2026-11-14 04:08-04:49Z, built as a separate mode so the RTS
//! round plays as before). The controlled squad is the one you play:
//!
//! - **WASD** steers it; the camera follows it while you do. Pan away
//!   freely (middle drag, arrows); **Space** brings the camera back.
//! - **Hold the left button**: every soldier in it fires at the cursor on
//!   the same beat. It faces the cursor.
//! - **Right click** sends it there as a squad; on a crystal it stands by
//!   and guards it (a slow income while no slime is on it, for as long as
//!   it holds), **Ctrl + right click** drains it (fast, until it is gone
//!   for both sides). No larder in this mode (`Rules::larder`).
//! - **1-9** or **Ctrl + wheel** switch squads; the wheel zooms, about
//!   the squad while the camera follows it.
//! - **Y** splits it; the half nearer the cursor becomes a new squad and
//!   you control that one. **Shift + drag** a box pulls the soldiers in it
//!   out into a squad of their own. **J** joins the nearest squad within
//!   reach into it.
//! - **T** cycles its formation: blob, line, wedge, ring.
//! - The command card works as in the RTS round, for the squad, with its
//!   letters moved off WASD (`squad_card_key`).
//!
//! **Keys, after the owner's playtest (2026-11-14 08:01Z).** WASD and
//! Space come before the card; every other key goes to the card first, so
//! its letters do what the card says on every page. Before, every squad key
//! came first: G merged instead of harvesting, X split instead of
//! defending, T on the build page cycled the formation instead of placing
//! a turret, and the wheel switched squads, so there was no zoom without
//! Ctrl ("the shortcuts for harvesting ... doesn't work", "I cannot zoom
//! in anymore"). The owner had not yet learned the squad's letters (08:19Z:
//! "how do you split the squad?"), so split and merge moved to Y and J and
//! the card's G and X stayed.
//!
//! Every action is a `Cmd`, so a squad round replays like any other.

use super::*;
use slime_mould::mould::larder_world::{Formation, SQUADS_MAX};
use slime_mould::world::ui::key_name;

/// Squads whose middles are this close (dish cells) merge on `MERGE`.
const MERGE_REACH: f32 = 24.0;

/// The squad's own keys after the card's (`App::squad_key`).
pub(super) const SPLIT: KeyCode = KeyCode::KeyY;
pub(super) const MERGE: KeyCode = KeyCode::KeyJ;
pub(super) const FORMATION: KeyCode = KeyCode::KeyT;

/// The bearing (radians) the cursor must swing round the controlled
/// squad's middle before a line or wedge turns to it again (`aim`).
const FACE_TURN: f32 = 0.17;

/// The card's letter in the squad mode, or None to leave the command off
/// the card: WASD steer, so what the RTS card has there moves (Attack-move
/// to R, which the larder's rally point leaves free, the toolkit's mine
/// and gas to R and F, the own mould's sowing to U), Stop goes (letting go
/// of WASD, or a tap of it, stops the squad), and so does the rally point
/// (no larder in this mode).
pub(super) fn squad_card_key(id: u32, key: KeyCode) -> Option<KeyCode> {
    match id {
        STOP | RALLY => None,
        ATTACK => Some(KeyCode::KeyR),
        T_MINE => Some(KeyCode::KeyR),
        T_GAS => Some(KeyCode::KeyF),
        SOW => Some(KeyCode::KeyU),
        _ => Some(key),
    }
}

/// The squad mode's rows on the key card (F1 in play, the start menu's
/// Keys page), first.
pub(super) fn squad_help() -> Vec<(String, String)> {
    let k = |c: KeyCode| key_name(c).unwrap_or_default();
    [
        ("WASD".to_string(), "Steer your squad; the camera follows it".to_string()),
        ("Hold left button".into(), "Every soldier in it fires at the cursor at once".into()),
        ("Right click".into(), "Send it; on a crystal it guards it (slow pay, lasts)".into()),
        ("Ctrl + right click".into(), "On a crystal: drain it (fast, then it is gone)".into()),
        ("1-9, Ctrl + wheel".into(), "Take command of another squad".into()),
        (k(SPLIT), "Split: the half nearer the cursor becomes a new squad".into()),
        ("Shift + drag".into(), "Pull the soldiers in the box out into a new squad".into()),
        (k(MERGE), "Join the nearest squad into yours".into()),
        (k(FORMATION), "Formation: blob, line, wedge, ring".into()),
        ("Wheel, middle drag".into(), "Zoom, pan; Space brings the camera back".into()),
    ]
    .into_iter()
    .collect()
}

#[derive(Default)]
pub(super) struct SquadInput {
    /// W, A, S, D held.
    keys: [bool; 4],
    steer: (f32, f32),
    /// The left button held (firing), and the point last sent.
    firing: bool,
    fire_at: Option<(f32, f32)>,
    face_at: Option<(f32, f32)>,
    /// The camera follows the controlled squad.
    pub(super) follow: bool,
    /// Where the camera was put last frame, to tell a pan by the player.
    put: Option<(f32, f32)>,
    /// The cursor's world point this frame: the aim, sent between steps
    /// (`aim`), and where the controlled squad's guns point.
    pub(super) want: Option<(f32, f32)>,
}

impl SquadInput {
    pub(super) fn new() -> Self {
        SquadInput { follow: true, ..SquadInput::default() }
    }

    /// Between two steps (`App::step`, before `Game::begin_step`): the
    /// controlled squad's volley point follows the cursor while the button
    /// is held, and a line or wedge turns to it once it has swung
    /// `FACE_TURN` round the squad. Before the bug sweep (2026-11-14) both
    /// were sent from the frame whenever the cursor moved 0.75 cells, and
    /// each command finished the step running on the GPU early: the
    /// window then lost that step's shots (`App::catch_up`), and every
    /// frame waited for the GPU.
    pub(super) fn aim(&mut self, g: &mut Game<GpuWorld>) {
        let (Some(q), Some(at)) = (g.active, self.want) else { return };
        let sc = g.world.scale();
        if self.firing {
            if self.fire_at.is_none_or(|f| (f.0 - at.0).hypot(f.1 - at.1) > 0.75 * sc) {
                self.fire_at = Some(at);
                g.apply(Cmd::Fire(q as u8, Some(at)));
            }
            return;
        }
        if !matches!(g.squads[q].formation, Formation::Line | Formation::Wedge) {
            return;
        }
        let Some(m) = g.middle(q) else { return };
        let bearing = |p: (f32, f32)| (p.1 - m.1).atan2(p.0 - m.0);
        let swung = self.face_at.is_none_or(|f| {
            let d = (bearing(at) - bearing(f)).rem_euclid(std::f32::consts::TAU);
            d.min(std::f32::consts::TAU - d) > FACE_TURN
        });
        if swung && (at.0 - m.0).hypot(at.1 - m.1) > sc {
            self.face_at = Some(at);
            g.apply(Cmd::Face(q as u8, at));
        }
    }
}

impl App {
    fn active(&self) -> Option<u8> {
        self.game.as_ref().and_then(|g| g.active).map(|q| q as u8)
    }

    fn cursor_world(&self) -> Option<(f32, f32)> {
        self.controls.cursor.and_then(|c| self.camera.world_at(c))
    }

    /// A key in the squad mode; true if it was the squad's. `first`: the
    /// keys asked before the card's (WASD steering and Space); else the
    /// squad's own letters, asked after the card has passed on the key.
    pub(super) fn squad_key(&mut self, code: KeyCode, pressed: bool, repeat: bool, first: bool) -> bool {
        // Not once the round is over: the end screen takes every key (the
        // review after the bug sweep, 2026-11-14: WASD still steered and
        // logged commands after the end).
        if self.sq.is_none() || self.game.as_ref().is_none_or(|g| g.end.is_some()) {
            return false;
        }
        if first {
            let wasd = [KeyCode::KeyW, KeyCode::KeyA, KeyCode::KeyS, KeyCode::KeyD];
            if let Some(i) = wasd.iter().position(|&k| k == code) {
                let sq = self.sq.as_mut().unwrap();
                sq.keys[i] = pressed;
                if pressed {
                    sq.follow = true;
                }
                self.send_steer();
                return true;
            }
            if code == KeyCode::Space {
                if pressed && !repeat {
                    let sq = self.sq.as_mut().unwrap();
                    sq.follow = true;
                    sq.put = None;
                }
                return true;
            }
            return false;
        }
        let ours = matches!(code, SPLIT | MERGE | FORMATION) || digit(code).is_some();
        if !pressed || repeat || !ours {
            return ours;
        }
        let at = self.cursor_world();
        let Some(q) = self.active() else { return true };
        let g = self.game.as_mut().unwrap();
        match code {
            SPLIT => {
                let p = at.or_else(|| g.middle(q as usize)).unwrap_or_default();
                if g.apply(Cmd::Split(q, p)) {
                    self.steer_new_squad();
                }
            }
            MERGE => {
                let Some(m) = g.middle(q as usize) else { return true };
                let reach = MERGE_REACH * g.world.scale();
                let other = g.live_squads().into_iter().filter(|&o| o != q as usize).filter_map(|o| g.middle(o).map(|c| (o, (c.0 - m.0).hypot(c.1 - m.1)))).filter(|&(_, d)| d <= reach).min_by(|a, b| a.1.total_cmp(&b.1));
                if let Some((o, _)) = other {
                    g.apply(Cmd::Merge(q, o as u8));
                } else {
                    self.hud.alert(&format!("No squad within {MERGE_REACH:.0} cells to join"), None, theme::WARN);
                }
            }
            FORMATION => {
                let f = g.squads[q as usize].formation.next();
                g.apply(Cmd::Formation(q, f));
                self.hud.alert(&format!("Formation: {}", f.name()), None, theme::TEXT);
                if let Some(sq) = self.sq.as_mut() {
                    // A line or wedge turns to the cursor at once.
                    sq.face_at = None;
                }
            }
            _ => {
                let Some(n) = digit(code) else { return false };
                if let Some(&o) = g.live_squads().get(n) {
                    self.switch_to(o as u8);
                }
            }
        }
        true
    }

    /// The controlled squad changed by a split: carry the steering, the
    /// held fire and the camera over to it.
    fn steer_new_squad(&mut self) {
        if let Some(sq) = self.sq.as_mut() {
            sq.steer = (0.0, 0.0);
            sq.fire_at = None;
            sq.face_at = None;
        }
        self.send_steer();
    }

    fn switch_to(&mut self, q: u8) {
        let Some(g) = self.game.as_mut() else { return };
        if g.active == Some(q as usize) {
            // Again: the camera comes to it.
            if let Some(sq) = self.sq.as_mut() {
                sq.follow = true;
                sq.put = None;
            }
            return;
        }
        if g.apply(Cmd::Active(q)) {
            if let Some(sq) = self.sq.as_mut() {
                sq.follow = true;
                sq.put = None;
            }
            self.steer_new_squad();
        }
    }

    /// The steering from the keys held, sent when it changes.
    fn send_steer(&mut self) {
        let (Some(sq), Some(q)) = (self.sq.as_ref(), self.active()) else { return };
        let k = sq.keys;
        let d = ((k[3] as i32 - k[1] as i32) as f32, (k[2] as i32 - k[0] as i32) as f32);
        if d != sq.steer || (d != (0.0, 0.0) && self.game.as_ref().is_some_and(|g| g.squads[q as usize].steer == (0.0, 0.0))) {
            self.sq.as_mut().unwrap().steer = d;
            if let Some(g) = self.game.as_mut() {
                g.apply(Cmd::Steer(q, d));
            }
        }
    }

    /// A mouse button in the squad mode; true if it was the squad's.
    pub(super) fn squad_button(&mut self, b: Button, down: bool) -> bool {
        if self.sq.is_none() || self.game.is_none() {
            return false;
        }
        match b {
            // Shift + left drag is the box (the controls take it).
            Button::Left if self.shift && down => false,
            Button::Left => {
                let at = self.cursor_world();
                let Some(q) = self.active() else { return true };
                let sq = self.sq.as_mut().unwrap();
                sq.firing = down && at.is_some();
                sq.fire_at = if sq.firing { at } else { None };
                let f = sq.fire_at;
                if let Some(g) = self.game.as_mut() {
                    g.apply(Cmd::Fire(q, f));
                }
                // A release the controls began as a box still ends it.
                !(!down && self.controls_dragging())
            }
            Button::Right => {
                if down {
                    if let (Some(at), Some(q)) = (self.cursor_world(), self.active()) {
                        // On a crystal: stand by it and guard it; Ctrl drains it.
                        let o = match self.order_for(at) {
                            Order::Clear(x, y) if !self.ctrl => Order::Move(x, y),
                            o => o,
                        };
                        if let Some(g) = self.game.as_mut() {
                            g.apply(Cmd::SquadOrder(q, o));
                        }
                    }
                }
                true
            }
            Button::Middle => false,
        }
    }

    fn controls_dragging(&self) -> bool {
        self.controls.dragging()
    }

    /// The box picked soldiers (Shift + drag): they leave for a squad of
    /// their own, which you then control.
    pub(super) fn squad_picked(&mut self, picked: &[usize]) {
        let picked = self.living(picked);
        let Some(g) = self.game.as_mut() else { return };
        if picked.is_empty() {
            return;
        }
        // All of one squad picked: just control it.
        let first = g.soldiers[picked[0]].squad;
        if picked.iter().all(|&k| g.soldiers[k].squad == first) && g.members(first as usize).len() == picked.len() {
            self.switch_to(first);
            return;
        }
        let Some(n) = g.free_squad() else {
            self.hud.alert(&format!("At most {SQUADS_MAX} squads"), None, theme::BAD);
            return;
        };
        for &k in &picked {
            g.apply(Cmd::Join(k, n as u8));
        }
        if let Some(m) = g.middle(n) {
            g.apply(Cmd::SquadOrder(n as u8, Order::Move(m.0, m.1)));
        }
        self.switch_to(n as u8);
    }

    /// Ctrl + wheel in the squad mode: the next or last squad. (It was the
    /// plain wheel, with Ctrl + wheel to zoom, until the owner's playtest,
    /// 2026-11-14 08:01Z: "I cannot zoom in anymore".)
    pub(super) fn squad_wheel(&mut self, notches: i32) -> bool {
        if self.sq.is_none() || !self.ctrl || notches == 0 {
            return false;
        }
        let Some(g) = self.game.as_ref() else { return false };
        let live = g.live_squads();
        if live.is_empty() {
            return true;
        }
        let i = g.active.and_then(|a| live.iter().position(|&q| q == a)).unwrap_or(0) as i32;
        let n = live.len() as i32;
        let j = (i - notches.signum()).rem_euclid(n) as usize;
        self.switch_to(live[j] as u8);
        true
    }

    /// The wheel in the squad mode while the camera follows the squad: zoom
    /// about the middle of the screen, where the squad is, and keep
    /// following (zoomed at the cursor, the camera moved, which read as a
    /// pan by the player and let go of the squad). True if it zoomed.
    pub(super) fn squad_zoom(&mut self, notches: i32) -> bool {
        let Some(sq) = self.sq.as_mut().filter(|sq| sq.follow) else { return false };
        if self.game.is_none() || notches == 0 {
            return false;
        }
        let c = self.camera.to_screen(self.camera.focus());
        self.camera.wheel(notches, c);
        if sq.put.is_some() {
            sq.put = Some(self.camera.focus());
        }
        true
    }

    /// Each frame in the squad mode: keep a squad controlled, aim and fire
    /// where the cursor is, show the squad as the selection, and follow it.
    pub(super) fn squad_frame(&mut self, dt: f32) {
        if self.sq.is_none() || self.in_menu() {
            return;
        }
        let Some(g) = self.game.as_mut() else { return };
        // The screen edge does not pan: the cursor is the aim.
        self.controls.edge = false;
        if g.active.is_none_or(|q| g.members(q).is_empty()) {
            if let Some(&q) = g.live_squads().first() {
                g.apply(Cmd::Active(q as u8));
            } else {
                return;
            }
        }
        let q = g.active.unwrap();
        let at = self.controls.cursor.and_then(|c| self.camera.world_at(c));
        let sq = self.sq.as_mut().unwrap();
        // Sent between steps (`SquadInput::aim`), not from here.
        if at.is_some() {
            sq.want = at;
        }
        self.controls.selected = g.members(q);
        // Follow: eased toward the squad's middle; a pan by the player (the
        // camera not where it was put) lets go until Space or WASD.
        // (`Camera::focus`, the inverse of `centre_on`: eased from the
        // screen's middle, the camera settled about 100 cells below the
        // squad at zoom 4, off the screen once zoomed in; bug sweep,
        // 2026-11-14.)
        if let (true, Some(m)) = (sq.follow, g.middle(q)) {
            let c = self.camera.focus();
            if sq.put.is_some_and(|p| (p.0 - c.0).hypot(p.1 - c.1) > 0.5) {
                sq.follow = false;
                sq.put = None;
                return;
            }
            let k = if sq.put.is_none() { 1.0 } else { (dt * 6.0).min(1.0) };
            let to = (c.0 + (m.0 - c.0) * k, c.1 + (m.1 - c.1) * k);
            self.camera.centre_on(to);
            sq.put = Some(self.camera.focus());
        }
    }

    /// Focus lost: let go of the keys and the trigger.
    pub(super) fn squad_unfocus(&mut self) {
        if let Some(sq) = self.sq.as_mut() {
            sq.keys = [false; 4];
            let was = sq.firing;
            sq.firing = false;
            sq.fire_at = None;
            if let (true, Some(q), Some(g)) = (was, self.game.as_ref().and_then(|g| g.active), self.game.as_mut()) {
                g.apply(Cmd::Fire(q as u8, None));
            }
        }
        self.send_steer();
    }
}

fn digit(code: KeyCode) -> Option<usize> {
    [KeyCode::Digit1, KeyCode::Digit2, KeyCode::Digit3, KeyCode::Digit4, KeyCode::Digit5, KeyCode::Digit6, KeyCode::Digit7, KeyCode::Digit8, KeyCode::Digit9].iter().position(|&k| k == code)
}
