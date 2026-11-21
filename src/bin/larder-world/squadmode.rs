//! **The squad mode's input** (`mode=squads`; `larder_world::squads`,
//! owner 2026-11-14 04:08-04:49Z, built as a separate mode so the RTS
//! round plays as before). The controlled squad is the one you play:
//!
//! - **WASD** steers it; the camera follows it while you do. Pan away
//!   freely (middle drag, arrows; WASD while paused); **Space** brings the
//!   camera back.
//! - **Hold the left button**: every soldier in it fires at the cursor on
//!   the same beat. It faces the cursor.
//! - **Right click** sends it there as a squad; on a crystal it stands by
//!   and guards it (a slow income while no slime is on it, for as long as
//!   it holds), **Ctrl + right click** drains it (fast, until it is gone
//!   for both sides). No larder in this mode (`Rules::larder`).
//!   On a half-built building it goes back to raising it (`resume`; no
//!   new placement, nothing paid again).
//! - **1-9** or **Ctrl + wheel** switch squads; the wheel zooms, about
//!   the squad while the camera follows it.
//! - **Y** splits it; the half nearer the cursor becomes a new squad and
//!   you control that one. **Shift + drag** a box pulls the soldiers in it
//!   out into a squad of their own. **J** joins the nearest squad within
//!   reach into it.
//! - **T** opens the card's Formation page: blob, spread, line, wedge,
//!   column, ring (a squad key that cycled them until 2026-11-21).
//! - **Esc** lets go of it (and the panel's Let go button): nothing is
//!   controlled, every squad keeps its last orders, fires at slime in reach
//!   and guards on its own ("auto" on its tile and over it), and WASD pans
//!   the camera. A number, a tile, a click on a squad or Space takes command
//!   again; a second Esc opens the menu (owner 2026-11-15 02:58Z: "a way to
//!   deselect so that you're not actually controlling any specific squad").
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
use slime_mould::mould::larder_world::{Formation, NO_LINK, SQUADS_MAX};
use slime_mould::world::ui::{SquadPanel, SquadRow};
use slime_mould::world::ui::key_name;

/// Squads overlap (and so can combine, `MERGE`) when their middles are no
/// further apart than their two spreads and this many dish cells (owner
/// 2026-11-14 15:06Z: combine "should only appear when squads overlap";
/// before, any squad within 24 cells of yours could be combined).
const OVERLAP_GAP: f32 = 4.0;

/// The squads overlapping squad `q` (`OVERLAP_GAP`), nearest first.
pub(super) fn overlapping(g: &Game<GpuWorld>, q: usize) -> Vec<usize> {
    let spread = |r: usize| {
        let m = g.middle(r)?;
        Some((m, g.members(r).iter().map(|&k| (g.soldiers[k].x - m.0).hypot(g.soldiers[k].y - m.1)).fold(0.0, f32::max)))
    };
    let Some((m, a)) = spread(q) else { return Vec::new() };
    let mut near: Vec<(usize, f32)> = g
        .live_squads()
        .into_iter()
        .filter(|&r| r != q)
        .filter_map(|r| spread(r).map(|(c, b)| (r, (c.0 - m.0).hypot(c.1 - m.1), b)))
        .filter(|&(_, d, b)| d <= a + b + OVERLAP_GAP * g.world.scale())
        .map(|(r, d, _)| (r, d))
        .collect();
    near.sort_by(|x, y| x.1.total_cmp(&y.1));
    near.into_iter().map(|x| x.0).collect()
}

/// The squad's own keys after the card's (`App::squad_key`).
pub(super) const SPLIT: KeyCode = KeyCode::KeyY;
pub(super) const MERGE: KeyCode = KeyCode::KeyJ;
/// The card's Formation button and the Formation page's Back (2026-11-21;
/// until then T cycled blob, line, wedge, ring as a squad key).
pub(super) const FORMATION: KeyCode = KeyCode::KeyT;
/// One key to go through the squads in turn, Shift for back (owner,
/// 2026-11-19 16:55Z: "there should be a single button to cycle through
/// squads"). The key left of 1, by the squads' own number keys: Tab was
/// taken by four times faster, and every letter near WASD by the card.
pub(super) const CYCLE: KeyCode = KeyCode::Backquote;
/// The card's Heal (Card 2, 2026-11-15 19:38Z; it was the squad panel's).
/// Heal: owner 2026-11-15 07:41Z, "Healing should be a specific action".
/// Since 2026-11-21 it has Pump's slot on the main card (Pump is on the
/// Build page), and L heals from the other pages too (`App::squad_key`),
/// bar the build page, where L is the lure's, as
/// every card key is that page's there. (The card's Lookouts, I, went
/// 2026-11-21 with the lookouts.)
pub(super) const HEAL_KEY: KeyCode = KeyCode::KeyL;

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
        ("WASD".to_string(), "Steer your squad; the camera follows it (paused, or no squad controlled: pan the camera)".to_string()),
        ("Esc".into(), "Let go of the squad: every squad runs on its own and WASD pans; Esc again: the menu".into()),
        ("Click a squad, Space".into(), "With none controlled: take command of it, or of the last one".into()),
        ("Hold left button".into(), "Every soldier in it fires at the cursor at once".into()),
        ("Right click".into(), "Send it; on a crystal it guards it (slow pay, lasts)".into()),
        ("Shift + right click".into(), "Queue it: the squad goes there after what it is doing".into()),
        ("Ctrl + right click".into(), "On a crystal: drain it (fast, then it is gone)".into()),
        ("Right click a half-built building".into(), "Back to building it (nothing paid again)".into()),
        ("1-9, Ctrl + wheel".into(), "Take command of another squad".into()),
        (format!("{0}, Shift + {0}", k(CYCLE)), "The next squad, or the one before".into()),
        (k(SPLIT), "Split: the half nearer the cursor becomes a new squad".into()),
        ("Shift + drag".into(), "Pull the soldiers in the box out into a new squad".into()),
        (k(MERGE), "Combine: a squad overlapping yours joins it".into()),
        ("Ctrl + 1-9".into(), "Link that squad with yours while they touch (again: unlink); a link is commanded as one".into()),
        (k(HEAL_KEY), "Heal: the squad stops and heals for food, fast by a crystal you hold (on the card)".into()),
        ("Z".into(), "Hold fire: on and off; the fire button (X) picks defend or free fire".into()),
        ("Click a building".into(), "With no squad commanded: select it (its health and state; cancel, demolish, upgrade, switch)".into()),
        (k(FORMATION), "Formation page: blob, spread, line, wedge, column, ring (on the card)".into()),
        ("Wheel, middle drag".into(), "Zoom, pan; Space brings the camera back".into()),
    ]
    .into_iter()
    .collect()
}

#[derive(Default)]
pub(super) struct SquadInput {
    /// W, A, S, D held.
    pub(super) keys: [bool; 4],
    steer: (f32, f32),
    /// The left button held (firing), and the point last sent.
    firing: bool,
    fire_at: Option<(f32, f32)>,
    face_at: Option<(f32, f32)>,
    /// The camera follows the controlled squad.
    pub(super) follow: bool,
    /// Where the camera was put last frame, to tell a pan by the player.
    pub(super) put: Option<(f32, f32)>,
    /// The cursor's world point this frame: the aim, sent between steps
    /// (`aim`), and where the controlled squad's guns point.
    pub(super) want: Option<(f32, f32)>,
    /// No squad controlled (Esc, the panel's Let go): every squad runs on
    /// its own and WASD pans the camera. Kept apart from `Game::active`
    /// being None, which also happens when the controlled squad dies, when
    /// the next one is taken at once.
    pub(super) free: bool,
    /// The squad controlled before letting go, for Space.
    last: Option<u8>,
}

impl SquadInput {
    pub(super) fn new() -> Self {
        SquadInput { follow: true, ..SquadInput::default() }
    }

    /// A WASD key held: the squad is being steered.
    pub(super) fn steering(&self) -> bool {
        self.keys.iter().any(|&k| k)
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

    /// The squad mode with no squad controlled (`SquadInput::free`).
    pub(super) fn squad_free(&self) -> bool {
        self.sq.as_ref().is_some_and(|sq| sq.free) && self.game.as_ref().is_some_and(|g| g.end.is_none())
    }

    /// Lets go of the controlled squad (Esc, the panel's Let go): it and
    /// its link stop steering and volleying and keep their other orders.
    pub(super) fn let_go(&mut self) {
        let Some(q) = self.active() else { return };
        self.squad_unfocus();
        if let Some(g) = self.game.as_mut() {
            g.apply(Cmd::Active(u8::MAX));
        }
        if let Some(sq) = self.sq.as_mut() {
            sq.free = true;
            sq.last = Some(q);
            sq.follow = false;
            sq.put = None;
            sq.steer = (0.0, 0.0);
            sq.face_at = None;
        }
        self.controls.selected.clear();
        self.hud.alert("No squad controlled: each runs on its own orders. WASD pans; click a squad, its number or Space to take command", None, theme::TEXT);
    }

    /// With none controlled: the live squad nearest the world point, if a
    /// soldier of it is within a few cells (a click on a squad takes it).
    fn squad_at(&self, at: (f32, f32)) -> Option<u8> {
        let g = self.game.as_ref()?;
        let near = (3.0 * g.world.scale()).max(14.0 / self.camera.zoom);
        g.soldiers.iter().filter(|s| s.alive).map(|s| (s.squad, (s.x - at.0).hypot(s.y - at.1))).filter(|&(_, d)| d <= near).min_by(|a, b| a.1.total_cmp(&b.1)).map(|x| x.0)
    }

    /// The world point under the cursor, or None over the HUD or the
    /// minimap: the world behind them is hidden (review after the bug
    /// sweep, 2026-11-14: a volley held over the bottom bar swung to the
    /// ground behind it, and a click on the minimap fired at or sent the
    /// squad to the ground behind the minimap).
    fn cursor_world(&self) -> Option<(f32, f32)> {
        self.controls.cursor.filter(|&c| !self.hud.over(c)).and_then(|c| self.camera.world_at(c))
    }

    /// The world point the minimap shows under the cursor, if it is on it.
    fn cursor_minimap(&self) -> Option<(f32, f32)> {
        self.controls.cursor.and_then(|c| self.hud.on_minimap(c))
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
            if code == KeyCode::Escape {
                // Esc lets go of the squad before it opens the menu; a
                // building being placed is called off first (the HUD's).
                if pressed && !repeat && !self.squad_free() && self.active().is_some() && self.hud.targeting().is_none() && !self.hud.menu_open() {
                    self.let_go();
                    return true;
                }
                return false;
            }
            if let Some(i) = wasd.iter().position(|&k| k == code) {
                if self.squad_free() {
                    // The controls pan (`Controls::wasd_pan`).
                    return false;
                }
                let sq = self.sq.as_mut().unwrap();
                sq.keys[i] = pressed;
                if pressed {
                    sq.follow = true;
                }
                self.send_steer();
                return true;
            }
            if code == KeyCode::Space {
                if pressed && !repeat && self.squad_free() {
                    // Take command again: the last squad, else the first.
                    let g = self.game.as_ref().unwrap();
                    let last = self.sq.as_ref().and_then(|sq| sq.last).filter(|&q| !g.members(q as usize).is_empty());
                    if let Some(q) = last.or_else(|| g.live_squads().first().map(|&q| q as u8)) {
                        self.switch_to(q);
                    }
                    return true;
                }
                if pressed && !repeat {
                    let sq = self.sq.as_mut().unwrap();
                    sq.follow = true;
                    sq.put = None;
                }
                return true;
            }
            return false;
        }
        let ours = matches!(code, SPLIT | MERGE | HEAL_KEY | CYCLE) || digit(code).is_some();
        if !pressed || repeat || !ours {
            return ours;
        }
        if code == CYCLE {
            self.cycle_squad(if self.shift { -1 } else { 1 });
            return true;
        }
        // Heal from wherever the card is (it sits on the Formation page).
        if code == HEAL_KEY {
            self.squad_heal();
            return true;
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
                if let Some(&o) = overlapping(g, q as usize).first() {
                    g.apply(Cmd::Merge(q, o as u8));
                } else {
                    self.hud.alert("No squad overlaps yours: bring them together to combine, or link them (Ctrl + its number)", None, theme::WARN);
                }
            }
            _ => {
                // Stable keys: digit n is squad id n - 1 for as long as it
                // lives, as an RTS control group (they once followed the
                // live squads' order, so a squad's key changed whenever a
                // lower one died, merged or split; bug sweep's review, 11-14).
                let Some(n) = digit(code) else { return false };
                if g.members(n).is_empty() {
                    return true;
                }
                if self.ctrl {
                    // Ctrl + its number: link it with yours, or unlink it.
                    if n != q as usize {
                        let on = g.linked(q as usize).contains(&n);
                        if on {
                            g.apply(Cmd::Unlink(n as u8));
                            self.hud.alert(&format!("Squad {} unlinked", n + 1), None, theme::TEXT);
                        } else if overlapping(g, q as usize).contains(&n) {
                            g.apply(Cmd::Link(q, n as u8));
                            self.hud.alert(&format!("Squad {} linked with yours: you command them as one", n + 1), None, theme::TEXT);
                        } else {
                            self.hud.alert(&format!("Squad {} isn't touching yours: bring them together to link", n + 1), None, theme::WARN);
                        }
                    }
                } else {
                    self.switch_to(n as u8);
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
        self.building_sel = None;
        let Some(g) = self.game.as_mut() else { return };
        if self.sq.as_ref().is_some_and(|sq| sq.free) {
            if g.members(q as usize).is_empty() || !g.apply(Cmd::Active(q)) {
                return;
            }
            let sq = self.sq.as_mut().unwrap();
            sq.free = false;
            sq.follow = true;
            sq.put = None;
            self.steer_new_squad();
            return;
        }
        if g.active.is_some_and(|a| g.linked(a).contains(&(q as usize))) {
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
    pub(super) fn send_steer(&mut self) {
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
            // On the minimap: the controls move the camera there.
            Button::Left if down && self.cursor_minimap().is_some() => false,
            Button::Left if self.squad_free() => {
                // A click on a squad takes command of it; else on a
                // building, selects it (owner 2026-11-15 19:51Z, option b:
                // only with no squad commanded, where the left button
                // doesn't fire), else lets go of one selected.
                if down {
                    let at = self.cursor_world();
                    if let Some(q) = at.and_then(|at| self.squad_at(at)) {
                        self.switch_to(q);
                    } else {
                        self.building_sel = at.and_then(|at| self.game.as_ref()?.building_at(at));
                    }
                }
                !(!down && self.controls_dragging())
            }
            Button::Right if self.squad_free() => {
                if down {
                    self.hud.alert("No squad controlled: click a squad, its tile or its number to give it orders", None, theme::WARN);
                }
                true
            }
            Button::Left => {
                let at = self.cursor_world();
                let Some(q) = self.active() else { return true };
                let sq = self.sq.as_mut().unwrap();
                let was = sq.firing;
                sq.firing = down && at.is_some();
                sq.fire_at = if sq.firing { at } else { None };
                let f = sq.fire_at;
                // (A release with nothing held sends nothing: a command
                // finishes the step running on the GPU.)
                if let Some(g) = self.game.as_mut().filter(|_| down || was) {
                    g.apply(Cmd::Fire(q, f));
                }
                // A release the controls began as a box still ends it.
                !(!down && self.controls_dragging())
            }
            Button::Right => {
                if down {
                    // On the minimap: where it shows, as in the RTS round.
                    if let (Some(at), Some(q)) = (self.cursor_minimap().or_else(|| self.cursor_world()), self.active()) {
                        // On an unfinished building: back to raising it, no
                        // new placement or price (owner 2026-11-15). Ctrl
                        // still drains the crystal under a pump rig.
                        let b = self.game.as_ref().filter(|_| !self.ctrl).and_then(|g| g.unfinished_at(at));
                        if b.is_some_and(|b| self.resume(&[], b)) {
                            return true;
                        }
                        // On a crystal: stand by it and guard it; Ctrl drains it.
                        let o = match self.order_for(at) {
                            Order::Clear(x, y) if !self.ctrl => Order::Move(x, y),
                            o => o,
                        };
                        // Shift queues it (`Cmd::SquadQueue`).
                        let shift = self.shift;
                        if let Some(g) = self.game.as_mut() {
                            g.apply(if shift { Cmd::SquadQueue(q, o) } else { Cmd::SquadOrder(q, o) });
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
        self.cycle_squad(-notches.signum());
        true
    }

    /// Command the squad `step` (1 or -1) on from the one commanded, by
    /// key, round to the first after the last (`CYCLE`, Ctrl + wheel).
    /// With none commanded, from the one last commanded.
    pub(super) fn cycle_squad(&mut self, step: i32) {
        let Some(g) = self.game.as_ref() else { return };
        let live = g.live_squads();
        if live.is_empty() {
            return;
        }
        let from = g.active.or_else(|| self.sq.as_ref().and_then(|sq| sq.last).map(usize::from));
        let n = live.len() as i32;
        // Past every squad linked with the one commanded: linked, it is
        // commanded already, and `switch_to` only brings the camera to it,
        // so the cycle stuck there (review, 2026-11-20).
        let linked = g.active.map(|a| g.linked(a)).unwrap_or_default();
        let j = match from.and_then(|a| live.iter().position(|&q| q == a)) {
            Some(i) => (1..=n).map(|k| (i as i32 + step * k).rem_euclid(n) as usize).find(|&j| !linked.contains(&live[j])),
            // None commanded (or it died): the one after where it was.
            None => Some(from.map_or(0, |a| live.iter().position(|&q| q > a).unwrap_or(0))),
        };
        let Some(j) = j else { return };
        self.switch_to(live[j] as u8);
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
        let at = self.cursor_world();
        let between = self.between();
        let Some(g) = self.game.as_mut() else { return };
        // The screen edge does not pan: the cursor is the aim.
        self.controls.edge = false;
        let sq = self.sq.as_mut().unwrap();
        if sq.free {
            // None controlled: the camera is the player's, nobody is
            // selected, and the cursor still says where a squad is.
            if at.is_some() {
                sq.want = at;
            }
            sq.follow = false;
            self.controls.selected.clear();
            return;
        }
        if g.active.is_none_or(|q| g.members(q).is_empty()) {
            if let Some(&q) = g.live_squads().first() {
                g.apply(Cmd::Active(q as u8));
            } else {
                return;
            }
        }
        let q = g.active.unwrap();
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
        // The middle as drawn (`Game::drawn_middle`), between steps: the
        // middle at the steps moved 15 times a second, and the camera eased
        // after it in jerks.
        if let (true, Some(m)) = (sq.follow, g.drawn_middle(q, between)) {
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

/// Why the card's Heal would do nothing for squad `a` now (everyone whole,
/// or healing already), or None.
pub(super) fn heal_off(g: &Game<GpuWorld>, a: usize) -> Option<String> {
    let ms: Vec<usize> = g.linked(a).into_iter().flat_map(|r| g.members(r)).collect();
    if ms.iter().all(|&k| g.soldiers[k].hp >= g.rules.hp) {
        Some("Everyone in the squad is at full health".to_string())
    } else if g.squads[a].heal {
        Some("Healing: give it another order to stop".to_string())
    } else {
        None
    }
}

fn digit(code: KeyCode) -> Option<usize> {
    [KeyCode::Digit1, KeyCode::Digit2, KeyCode::Digit3, KeyCode::Digit4, KeyCode::Digit5, KeyCode::Digit6, KeyCode::Digit7, KeyCode::Digit8, KeyCode::Digit9].iter().position(|&k| k == code)
}

/// The squad tiles (`ui::SquadPanel`, owner 2026-11-14 08:22Z and 08:48Z,
/// in the bottom middle panel's place): a tile a
/// live squad in id order (row `r` is `live_squads()[r]`; its key is its
/// id + 1, kept while it lives, as the digit keys pick), and Split and Combine.
impl App {
    pub(super) fn squad_panel(&self) -> Option<SquadPanel> {
        self.sq.as_ref()?;
        let g = self.game.as_ref()?;
        let live = g.live_squads();
        let overlap = g.active.map(|a| overlapping(g, a)).unwrap_or_default();
        let rows = live
            .iter()
            .map(|&q| {
                let sq = &g.squads[q];
                let mut note = Vec::new();
                if sq.formation != Formation::Blob {
                    note.push(sq.formation.name().to_string());
                }
                if g.active != Some(q) && !g.leads(q) {
                    note.push("on its own".into());
                }
                let state = if sq.fire.is_some() {
                    "firing".to_string()
                } else if sq.heal {
                    "healing".to_string()
                } else if sq.work.is_some() {
                    "working".to_string()
                } else if sq.steer != (0.0, 0.0) || sq.dest.is_some_and(|d| g.middle(q).is_some_and(|m| (m.0 - d.0).hypot(m.1 - d.1) > 4.0 * g.world.scale())) {
                    "moving".to_string()
                } else {
                    "holding".to_string()
                };
                note.push(state.clone());
                if sq.queued > 0 {
                    note.push(format!("{} queued", sq.queued));
                }
                if sq.link != NO_LINK {
                    note.push("linked".into());
                }
                let linked = g.active.is_some_and(|a| a != q && g.linked(a).contains(&q));
                let near = g.active.is_some_and(|a| a != q && overlap.contains(&q));
                SquadRow { key: if q < 9 { format!("{}", q + 1) } else { "-".into() }, name: format!("Squad {}", q + 1), soldiers: g.members(q).len(), hp: { let ms = g.members(q); ms.iter().map(|&k| g.soldiers[k].hp / g.rules.hp).sum::<f32>() / ms.len().max(1) as f32 }, note: Some(note.join(", ")), active: g.active == Some(q), near, linked, auto: !g.leads(q), state }
            })
            .collect();
        let mine = g.active.map_or(0, |q| g.members(q).len());
        Some(SquadPanel {
            rows,
            split_key: key_name(SPLIT),
            combine_key: key_name(MERGE),
            split_off: if mine < 2 {
                Some("A squad of one can't split".into())
            } else if g.free_squad().is_none() {
                Some(format!("At most {SQUADS_MAX} squads"))
            } else {
                None
            },
            combine_off: if live.len() < 2 {
                Some("No other squad to combine with".into())
            } else if overlap.is_empty() {
                Some("No squad overlaps yours: bring one over to combine, or link it (its tile's corner)".into())
            } else {
                None
            },
            free: g.active.is_none(),
            free_key: Some("Esc".into()),
        })
    }

    /// The card's Heal: the squad you command (and its link, commanded as
    /// one) stops and heals.
    pub(super) fn squad_heal(&mut self) {
        let (Some(q), Some(g)) = (self.active(), self.game.as_mut()) else { return };
        if g.apply(Cmd::Heal(q)) {
            self.hud.alert("Healing: stand still; fast by a crystal you hold, slow elsewhere, food from the bank", None, theme::TEXT);
        }
    }

    /// The Formation page's pick: the controlled squad and its link take
    /// formation `f`.
    pub(super) fn squad_formation(&mut self, f: Formation) {
        let (Some(q), Some(g)) = (self.active(), self.game.as_mut()) else { return };
        for r in g.linked(q as usize) {
            g.apply(Cmd::Formation(r as u8, f));
        }
        self.hud.alert(&format!("Formation: {}", f.name()), None, theme::TEXT);
        if let Some(sq) = self.sq.as_mut() {
            // A line or wedge turns to the cursor at once.
            sq.face_at = None;
        }
    }

    /// The fire button in the squad mode: defend and free in turn, by
    /// the first soldier's mode. A soldier on hold fire keeps holding; the
    /// mode is the one it fires once let go (`App::unheld`).
    pub(super) fn squad_fire_mode(&mut self, sel: &[usize]) {
        let Some(g) = self.game.as_mut() else { return };
        let mode = |k: usize| match g.soldiers[k].stance {
            Stance::HoldFire => self.unheld.get(&k).copied().unwrap_or(Stance::Defend),
            st => st,
        };
        let next = if sel.first().map(|&k| mode(k)) == Some(Stance::Free) { Stance::Defend } else { Stance::Free };
        let mut holding = false;
        for &k in sel {
            if g.soldiers[k].stance == Stance::HoldFire {
                self.unheld.insert(k, next);
                holding = true;
            } else {
                g.apply(Cmd::Stance(k, next));
            }
        }
        let note = if holding { " (holding fire till you let go)" } else { "" };
        self.hud.alert(&format!("Fire: {}{note}", stance_name(next)), None, theme::TEXT);
    }

    /// Hold fire in the squad mode, a toggle: on unless every soldier
    /// already holds, then off, back to the mode each had.
    pub(super) fn squad_hold_fire(&mut self, sel: &[usize]) {
        let Some(g) = self.game.as_mut() else { return };
        if sel.is_empty() {
            return;
        }
        let hold = !sel.iter().all(|&k| g.soldiers[k].stance == Stance::HoldFire);
        for &k in sel {
            let st = g.soldiers[k].stance;
            if hold && st != Stance::HoldFire {
                self.unheld.insert(k, st);
                g.apply(Cmd::Stance(k, Stance::HoldFire));
            } else if !hold {
                let back = self.unheld.remove(&k).unwrap_or(Stance::Defend);
                g.apply(Cmd::Stance(k, back));
            }
        }
        self.hud.alert(if hold { "Hold fire: on (the left button still fires a volley)" } else { "Hold fire: off" }, None, theme::TEXT);
    }

    /// A squad panel event: command a squad, bring one in, split, combine.
    pub(super) fn squad_panel_event(&mut self, ev: HudEvent) {
        let Some(g) = self.game.as_mut() else { return };
        let live = g.live_squads();
        match ev {
            HudEvent::Squad(r) => {
                if let Some(&o) = live.get(r) {
                    self.switch_to(o as u8);
                }
                return;
            }
            HudEvent::SquadFree => {
                self.let_go();
                return;
            }
            _ => {}
        }
        let Some(g) = self.game.as_mut() else { return };
        let Some(q) = g.active else { return };
        match ev {
            HudEvent::SquadInto(r) => {
                if let Some(&o) = live.get(r).filter(|&&o| o != q && overlapping(g, q).contains(&o)) {
                    g.apply(Cmd::Merge(q as u8, o as u8));
                }
            }
            HudEvent::SquadLink(r) => {
                if let Some(&o) = live.get(r).filter(|&&o| o != q) {
                    if g.linked(q).contains(&o) {
                        g.apply(Cmd::Unlink(o as u8));
                    } else if overlapping(g, q).contains(&o) {
                        g.apply(Cmd::Link(q as u8, o as u8));
                    }
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
                // The nearest squad overlapping yours.
                if let Some(&o) = overlapping(g, q).first() {
                    g.apply(Cmd::Merge(q as u8, o as u8));
                }
            }
            _ => {}
        }
    }
}
