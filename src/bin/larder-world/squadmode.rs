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
//! - **1-9** or the **wheel** switch squads (Ctrl + wheel zooms).
//! - **X** splits it; the half nearer the cursor becomes a new squad and
//!   you control that one. **Shift + drag** a box pulls the soldiers in it
//!   out into a squad of their own. **G** merges the nearest squad within
//!   reach into it.
//! - **T** cycles its formation: blob, line, wedge, ring.
//!
//! Every action is a `Cmd`, so a squad round replays like any other.

use super::*;
use slime_mould::mould::larder_world::{Formation, SQUADS_MAX};
use slime_mould::world::ui::{SquadPanel, SquadRow};

/// Squads whose middles are this close (dish cells) merge on G.
const MERGE_REACH: f32 = 24.0;

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
}

impl SquadInput {
    pub(super) fn new() -> Self {
        SquadInput { follow: true, ..SquadInput::default() }
    }
}

impl App {
    fn active(&self) -> Option<u8> {
        self.game.as_ref().and_then(|g| g.active).map(|q| q as u8)
    }

    fn cursor_world(&self) -> Option<(f32, f32)> {
        self.controls.cursor.and_then(|c| self.camera.world_at(c))
    }

    /// A key in the squad mode; true if it was the squad's.
    pub(super) fn squad_key(&mut self, code: KeyCode, pressed: bool, repeat: bool) -> bool {
        if self.sq.is_none() || self.game.is_none() {
            return false;
        }
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
        if !pressed || repeat {
            return matches!(code, KeyCode::KeyX | KeyCode::KeyG | KeyCode::KeyT | KeyCode::Space) || digit(code).is_some();
        }
        let at = self.cursor_world();
        let Some(q) = self.active() else { return digit(code).is_some() };
        let g = self.game.as_mut().unwrap();
        match code {
            KeyCode::KeyX => {
                let p = at.or_else(|| g.middle(q as usize)).unwrap_or_default();
                if g.apply(Cmd::Split(q, p)) {
                    self.steer_new_squad();
                }
            }
            KeyCode::KeyG => {
                let Some(m) = g.middle(q as usize) else { return true };
                let reach = MERGE_REACH * g.world.scale();
                let other = g.live_squads().into_iter().filter(|&o| o != q as usize).filter_map(|o| g.middle(o).map(|c| (o, (c.0 - m.0).hypot(c.1 - m.1)))).filter(|&(_, d)| d <= reach).min_by(|a, b| a.1.total_cmp(&b.1));
                if let Some((o, _)) = other {
                    g.apply(Cmd::Merge(q, o as u8));
                }
            }
            KeyCode::KeyT => {
                let f = g.squads[q as usize].formation.next();
                g.apply(Cmd::Formation(q, f));
                self.hud.alert(&format!("Formation: {}", f.name()), None, theme::TEXT);
            }
            KeyCode::Space => {
                let sq = self.sq.as_mut().unwrap();
                sq.follow = true;
                sq.put = None;
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

    /// The wheel in the squad mode: the next or last squad (Ctrl: zoom).
    pub(super) fn squad_wheel(&mut self, notches: i32) -> bool {
        if self.sq.is_none() || self.ctrl || notches == 0 {
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
        let sc = g.world.scale();
        let sq = self.sq.as_mut().unwrap();
        let moved = |a: Option<(f32, f32)>, b: Option<(f32, f32)>| match (a, b) {
            (Some(a), Some(b)) => (a.0 - b.0).hypot(a.1 - b.1) > 0.75 * sc,
            (a, b) => a.is_some() != b.is_some(),
        };
        if sq.firing && at.is_some() && moved(at, sq.fire_at) {
            sq.fire_at = at;
            g.apply(Cmd::Fire(q as u8, at));
        } else if !sq.firing && at.is_some() && moved(at, sq.face_at) {
            sq.face_at = at;
            g.apply(Cmd::Face(q as u8, at.unwrap()));
        }
        self.controls.selected = g.members(q);
        // Follow: eased toward the squad's middle; a pan by the player (the
        // camera not where it was put) lets go until Space or WASD.
        if let (true, Some(m)) = (sq.follow, g.middle(q)) {
            let c = self.camera.centre();
            if sq.put.is_some_and(|p| (p.0 - c.0).hypot(p.1 - c.1) > 0.5) {
                sq.follow = false;
                sq.put = None;
                return;
            }
            let k = if sq.put.is_none() { 1.0 } else { (dt * 6.0).min(1.0) };
            let to = (c.0 + (m.0 - c.0) * k, c.1 + (m.1 - c.1) * k);
            self.camera.centre_on(to);
            sq.put = Some(self.camera.centre());
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

/// The squad panel (`ui::SquadPanel`, owner 2026-11-14 08:22Z): a row a
/// live squad in hotkey order (row `r` is `live_squads()[r]`, key `r + 1`,
/// as the digit keys pick), and Split and Combine.
impl App {
    pub(super) fn squad_panel(&self) -> Option<SquadPanel> {
        self.sq.as_ref()?;
        let g = self.game.as_ref()?;
        let live = g.live_squads();
        let rows = live
            .iter()
            .enumerate()
            .map(|(i, &q)| {
                let sq = &g.squads[q];
                let mut note = Vec::new();
                if sq.formation != Formation::Blob {
                    note.push(sq.formation.name().to_string());
                }
                note.push(if sq.fire.is_some() { "firing" } else if sq.steer != (0.0, 0.0) || sq.dest.is_some_and(|d| g.middle(q).is_some_and(|m| (m.0 - d.0).hypot(m.1 - d.1) > 4.0 * g.world.scale())) { "moving" } else { "holding" }.to_string());
                SquadRow { key: if i < 9 { format!("{}", i + 1) } else { "-".into() }, name: format!("Squad {}", q + 1), soldiers: g.members(q).len(), note: Some(note.join(", ")), active: g.active == Some(q) }
            })
            .collect();
        let mine = g.active.map_or(0, |q| g.members(q).len());
        Some(SquadPanel {
            rows,
            split_key: Some("X".into()),
            combine_key: Some("G".into()),
            split_off: if mine < 2 {
                Some("A squad of one can't split".into())
            } else if g.free_squad().is_none() {
                Some(format!("At most {SQUADS_MAX} squads"))
            } else {
                None
            },
            combine_off: (live.len() < 2).then(|| "No other squad to combine with".into()),
        })
    }

    /// A squad panel event: command a squad, bring one in, split, combine.
    pub(super) fn squad_panel_event(&mut self, ev: HudEvent) {
        let Some(g) = self.game.as_mut() else { return };
        let live = g.live_squads();
        let Some(q) = g.active else { return };
        match ev {
            HudEvent::Squad(r) => {
                if let Some(&o) = live.get(r) {
                    self.switch_to(o as u8);
                }
            }
            HudEvent::SquadInto(r) => {
                if let Some(&o) = live.get(r).filter(|&&o| o != q) {
                    g.apply(Cmd::Merge(q as u8, o as u8));
                }
            }
            HudEvent::SquadSplit => {
                // The front half (the way it faces) becomes the new squad.
                let Some(m) = g.middle(q) else { return };
                let f = g.squads[q].face;
                let reach = 20.0 * g.world.scale();
                if g.apply(Cmd::Split(q as u8, (m.0 + f.0 * reach, m.1 + f.1 * reach))) {
                    self.steer_new_squad();
                }
            }
            HudEvent::SquadCombine => {
                // The nearest other squad, however far: it walks over.
                let Some(m) = g.middle(q) else { return };
                let other = live.iter().copied().filter(|&o| o != q).filter_map(|o| g.middle(o).map(|c| (o, (c.0 - m.0).hypot(c.1 - m.1)))).min_by(|a, b| a.1.total_cmp(&b.1));
                if let Some((o, _)) = other {
                    g.apply(Cmd::Merge(q as u8, o as u8));
                }
            }
            _ => {}
        }
    }
}
