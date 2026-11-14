//! **The controls**: RTS-style mouse and keys over a scrolling world, as
//! Larder's window has them (`src/bin/larder.rs`, owner 03:14Z), turned
//! into plain `UiCommand`s in world cells. No game rules: the game says
//! where its units are (`Unit`) and decides what each command does.
//!
//! - Left click a unit selects it; a double click selects every local
//!   unit on screen; left drag a box selects the local team's units in it
//!   (Shift adds to the selection); a click on empty ground clears it.
//! - Right click gives the selection an order at that point (`Order`); the
//!   game decides whether it is a move, or clearing a pile or a crystal.
//! - Hold F and left drag: a burn line (`Burn`).
//! - Ctrl+1-9 makes the selection a group; 1-9 selects it again, and a
//!   second press within half a second centres the camera on it.
//! - The minimap: left click or drag moves the camera there; right click
//!   orders the selection there.
//! - The camera: the wheel zooms at the cursor, middle drag pans, the
//!   cursor at a screen edge scrolls, and (if `arrows_pan`) the arrow keys.
//! - Any other key comes out as `Key` with the selection and the cursor's
//!   world point, for the game to bind (Larder: A, B, T, G, Z, X, C, V).
//!
//! Only units of `team` (the local player's) can be selected, so a second
//! team or the player AI's units are seen but not commanded.

use std::time::{Duration, Instant};

use winit::keyboard::KeyCode;

use super::camera::Camera;
use super::marks::{team_colour, Marks, SELECTED};
use super::view::minimap_world;

/// A unit as the controls see it.
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct Unit {
    pub id: usize,
    pub team: u8,
    pub x: f32,
    pub y: f32,
    pub alive: bool,
}

#[derive(Clone, Debug, PartialEq)]
pub enum UiCommand {
    /// The selection changed (the game may ignore it).
    Select(Vec<usize>),
    /// Right click: the selection's order at a world point.
    Order { units: Vec<usize>, at: (f32, f32) },
    /// F + left drag: hold the line from `from` to `to` burning.
    Burn { units: Vec<usize>, from: (f32, f32), to: (f32, f32) },
    /// Any key the controls do not use, pressed: the selection and the
    /// cursor's world point (if on the world).
    Key { code: KeyCode, units: Vec<usize>, at: Option<(f32, f32)> },
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Button {
    Left,
    Right,
    Middle,
}

/// How far (screen pixels) a press must move to be a drag, and how near a
/// click must be to a unit to pick it.
pub const DRAG_PX: f32 = 4.0;
pub const PICK_PX: f32 = 10.0;
const DOUBLE: Duration = Duration::from_millis(500);

const DIGITS: [KeyCode; 9] = [KeyCode::Digit1, KeyCode::Digit2, KeyCode::Digit3, KeyCode::Digit4, KeyCode::Digit5, KeyCode::Digit6, KeyCode::Digit7, KeyCode::Digit8, KeyCode::Digit9];

#[derive(Clone, Copy, Debug, PartialEq)]
enum Drag {
    /// Left pressed on the world at this screen point.
    Select((f32, f32)),
    /// Left pressed on the minimap.
    Minimap,
    /// Middle pressed: the last screen point.
    Pan((f32, f32)),
}

#[derive(Debug)]
pub struct Controls {
    pub team: u8,
    pub selected: Vec<usize>,
    pub groups: [Vec<usize>; 9],
    /// The cursor, in screen pixels (`None` off the window).
    pub cursor: Option<(f32, f32)>,
    pub ctrl: bool,
    pub shift: bool,
    /// F is held: a left drag is a burn line.
    pub burn: bool,
    /// The arrow keys pan (off in the sandbox, whose arrows pick controls).
    pub arrows_pan: bool,
    /// Edge scrolling (off when the window is not focused).
    pub edge: bool,
    arrows: [bool; 4],
    drag: Option<Drag>,
    last_group: Option<(usize, Instant)>,
    /// The last click that picked a unit, for the double click.
    last_pick: Option<(usize, Instant)>,
}

impl Controls {
    pub fn new(team: u8) -> Self {
        Controls { team, selected: Vec::new(), groups: Default::default(), cursor: None, ctrl: false, shift: false, burn: false, arrows_pan: true, edge: true, arrows: [false; 4], drag: None, last_group: None, last_pick: None }
    }

    fn mine<'a>(&self, units: &'a [Unit]) -> impl Iterator<Item = &'a Unit> + 'a {
        let team = self.team;
        units.iter().filter(move |u| u.alive && u.team == team)
    }

    /// The local unit nearest a screen point, within `PICK_PX`.
    pub fn pick(&self, units: &[Unit], cam: &Camera, at: (f32, f32)) -> Option<usize> {
        self.mine(units)
            .map(|u| {
                let s = cam.to_screen((u.x, u.y));
                (u.id, (s.0 - at.0).hypot(s.1 - at.1))
            })
            .filter(|&(_, d)| d <= PICK_PX)
            .min_by(|a, b| a.1.total_cmp(&b.1))
            .map(|(k, _)| k)
    }

    /// Drops dead or lost units from the selection and the groups.
    pub fn prune(&mut self, units: &[Unit]) {
        let alive = |k: &usize| units.iter().any(|u| u.id == *k && u.alive);
        self.selected.retain(alive);
        for g in &mut self.groups {
            g.retain(alive);
        }
    }

    /// The cursor moved to screen point `at`.
    pub fn moved(&mut self, at: Option<(f32, f32)>, cam: &mut Camera, mm: Option<(u32, u32, u32, u32)>) {
        self.cursor = at;
        match (self.drag, at) {
            (Some(Drag::Pan(last)), Some(p)) => {
                cam.drag((p.0 - last.0, p.1 - last.1));
                self.drag = Some(Drag::Pan(p));
            }
            (Some(Drag::Minimap), Some(p)) => {
                if let Some(w) = mm.and_then(|r| minimap_world(r, (cam.world.0 as usize, cam.world.1 as usize), p)) {
                    cam.centre_on(w);
                }
            }
            _ => {}
        }
    }

    /// A mouse button. `mm` is the minimap's rectangle this frame.
    pub fn button(&mut self, b: Button, pressed: bool, units: &[Unit], cam: &mut Camera, mm: Option<(u32, u32, u32, u32)>) -> Vec<UiCommand> {
        let Some(c) = self.cursor else {
            if !pressed {
                self.drag = None;
            }
            return Vec::new();
        };
        let world = (cam.world.0 as usize, cam.world.1 as usize);
        let on_map = mm.and_then(|r| minimap_world(r, world, c));
        let mut out = Vec::new();
        match (b, pressed) {
            (Button::Middle, true) => self.drag = Some(Drag::Pan(c)),
            (Button::Middle, false) => self.drag = None,
            (Button::Left, true) => {
                if let Some(w) = on_map {
                    cam.centre_on(w);
                    self.drag = Some(Drag::Minimap);
                } else {
                    self.drag = Some(Drag::Select(c));
                }
            }
            (Button::Left, false) => {
                if let Some(Drag::Select(from)) = self.drag {
                    let far = (c.0 - from.0).hypot(c.1 - from.1) > DRAG_PX;
                    let (a, z) = (cam.to_world(from), cam.to_world(c));
                    if self.burn && far {
                        if !self.selected.is_empty() {
                            out.push(UiCommand::Burn { units: self.selected.clone(), from: a, to: z });
                        }
                    } else {
                        let picked: Vec<usize> = if far {
                            let (x0, x1, y0, y1) = (a.0.min(z.0), a.0.max(z.0), a.1.min(z.1), a.1.max(z.1));
                            self.mine(units).filter(|u| u.x >= x0 && u.x <= x1 && u.y >= y0 && u.y <= y1).map(|u| u.id).collect()
                        } else {
                            let k = self.pick(units, cam, c);
                            let double = k.is_some() && self.last_pick.is_some_and(|(q, t)| Some(q) == k && t.elapsed() < DOUBLE);
                            self.last_pick = if double { None } else { k.map(|k| (k, Instant::now())) };
                            if double {
                                // Every local unit on screen.
                                self.mine(units)
                                    .filter(|u| {
                                        let s = cam.to_screen((u.x, u.y));
                                        s.0 >= 0.0 && s.1 >= 0.0 && s.0 < cam.screen.0 && s.1 < cam.screen.1
                                    })
                                    .map(|u| u.id)
                                    .collect()
                            } else {
                                k.into_iter().collect()
                            }
                        };
                        if self.shift {
                            for k in picked {
                                if !self.selected.contains(&k) {
                                    self.selected.push(k);
                                }
                            }
                        } else {
                            self.selected = picked;
                        }
                        out.push(UiCommand::Select(self.selected.clone()));
                    }
                }
                self.drag = None;
            }
            (Button::Right, true) => {
                let at = on_map.or_else(|| cam.world_at(c));
                if let (Some(at), false) = (at, self.selected.is_empty()) {
                    out.push(UiCommand::Order { units: self.selected.clone(), at });
                }
            }
            (Button::Right, false) => {}
        }
        out
    }

    /// The wheel: `notches` up (in) or down (out).
    pub fn wheel(&mut self, notches: i32, cam: &mut Camera) {
        if let Some(c) = self.cursor {
            cam.wheel(notches, c);
        }
    }

    /// A key. Returns what it gave the game; keys the controls use (F,
    /// Ctrl, Shift, digits, arrows when they pan) give nothing.
    pub fn key(&mut self, code: KeyCode, pressed: bool, repeat: bool, units: &[Unit], cam: &mut Camera) -> Vec<UiCommand> {
        match code {
            KeyCode::KeyF => {
                self.burn = pressed;
                return Vec::new();
            }
            KeyCode::ControlLeft | KeyCode::ControlRight => {
                self.ctrl = pressed;
                return Vec::new();
            }
            KeyCode::ShiftLeft | KeyCode::ShiftRight => {
                self.shift = pressed;
                return Vec::new();
            }
            _ => {}
        }
        let arrow = [KeyCode::ArrowLeft, KeyCode::ArrowRight, KeyCode::ArrowUp, KeyCode::ArrowDown].iter().position(|&k| k == code);
        if let (Some(a), true) = (arrow, self.arrows_pan) {
            self.arrows[a] = pressed;
            return Vec::new();
        }
        if !pressed || repeat {
            return Vec::new();
        }
        if let Some(g) = DIGITS.iter().position(|&d| d == code) {
            if self.ctrl {
                self.groups[g] = self.selected.clone();
                return Vec::new();
            }
            self.selected = self.groups[g].iter().copied().filter(|k| self.mine(units).any(|u| u.id == *k)).collect();
            let again = self.last_group.is_some_and(|(h, t)| h == g && t.elapsed() < DOUBLE);
            if again {
                let at: Vec<(f32, f32)> = self.selected.iter().filter_map(|k| units.iter().find(|u| u.id == *k)).map(|u| (u.x, u.y)).collect();
                if !at.is_empty() {
                    let n = at.len() as f32;
                    cam.centre_on((at.iter().map(|p| p.0).sum::<f32>() / n, at.iter().map(|p| p.1).sum::<f32>() / n));
                }
            }
            self.last_group = Some((g, Instant::now()));
            return vec![UiCommand::Select(self.selected.clone())];
        }
        let at = self.cursor.and_then(|c| cam.world_at(c));
        vec![UiCommand::Key { code, units: self.selected.clone(), at }]
    }

    /// The window lost focus: nothing stays held.
    pub fn unfocus(&mut self) {
        (self.ctrl, self.shift, self.burn, self.arrows, self.drag) = (false, false, false, [false; 4], None);
    }

    /// Once a frame: the edge scroll and the arrow keys over `dt` seconds.
    pub fn update(&mut self, cam: &mut Camera, dt: f32) {
        let dt = dt.min(0.1);
        if let (Some(c), true, None) = (self.cursor, self.edge, self.drag) {
            cam.edge_scroll(c, dt);
        }
        let step = super::camera::PAN_SPEED * dt;
        let d = ((self.arrows[1] as i32 - self.arrows[0] as i32) as f32 * step, (self.arrows[3] as i32 - self.arrows[2] as i32) as f32 * step);
        if d != (0.0, 0.0) {
            cam.pan(d);
        }
    }

    /// The marks the controls own: a ring round each selected unit (`r`
    /// world cells, the units' size, so the ring clears the disc at any
    /// zoom) and the drag box or burn line under way. The game draws the
    /// units; push these after them, so they draw on top.
    pub fn marks(&self, units: &[Unit], cam: &Camera, r: f32, out: &mut Marks) {
        // Just clear of the figure's body (0.8 of its radius in sprite A),
        // thin, so the soldier inside stays the thing you see.
        let ring = (r * cam.zoom * 0.95).max(2.0) + 2.0;
        for u in units.iter().filter(|u| u.alive && self.selected.contains(&u.id)) {
            out.selected((u.x, u.y), ring);
        }
        if let (Some(Drag::Select(from)), Some(c)) = (self.drag, self.cursor) {
            if (c.0 - from.0).hypot(c.1 - from.1) > DRAG_PX {
                let (a, z) = (cam.to_world(from), cam.to_world(c));
                if self.burn {
                    out.line(a, z, 4.0, [255, 120, 40, 255]);
                } else {
                    out.frame(a, z, SELECTED);
                }
            }
        }
    }
}

/// A unit's usual marks: a disc in its team's colour (`r` world cells, on
/// the minimap too) and a health bar `hp` full just under it at `zoom`.
pub fn unit_marks(u: &Unit, r: f32, hp: f32, zoom: f32, out: &mut Marks) {
    if !u.alive {
        out.disc((u.x, u.y), r, [120, 30, 30, 255], false);
        return;
    }
    out.disc((u.x, u.y), r, team_colour(u.team), true);
    out.bar((u.x, u.y), (r * zoom).max(2.0) + 5.0, (7.0, 1.0), hp, [90, 220, 110, 255], [70, 30, 30, 255]);
}

#[cfg(test)]
mod tests {
    use super::*;

    fn squad() -> Vec<Unit> {
        let mut v: Vec<Unit> = (0..5).map(|k| Unit { id: k, team: 0, x: 100.0 + 20.0 * k as f32, y: 100.0, alive: true }).collect();
        v.push(Unit { id: 5, team: 1, x: 120.0, y: 110.0, alive: true });
        v.push(Unit { id: 6, team: 0, x: 140.0, y: 105.0, alive: false });
        v
    }

    fn cam() -> Camera {
        let mut c = Camera::new((2048, 1280), (1280, 800));
        c.zoom_to(2.0, (0.0, 0.0));
        c.x = 50.0;
        c.y = 50.0;
        c
    }

    fn click(k: &mut Controls, b: Button, at: (f32, f32), units: &[Unit], c: &mut Camera) -> Vec<UiCommand> {
        k.moved(Some(at), c, None);
        let mut out = k.button(b, true, units, c, None);
        out.extend(k.button(b, false, units, c, None));
        out
    }

    fn drag(k: &mut Controls, from: (f32, f32), to: (f32, f32), units: &[Unit], c: &mut Camera) -> Vec<UiCommand> {
        k.moved(Some(from), c, None);
        let mut out = k.button(Button::Left, true, units, c, None);
        k.moved(Some(to), c, None);
        out.extend(k.button(Button::Left, false, units, c, None));
        out
    }

    #[test]
    fn a_box_in_world_cells_selects_only_the_local_teams_living_units() {
        let (u, mut c, mut k) = (squad(), cam(), Controls::new(0));
        // World (90..150, 90..115) at zoom 2 from (50, 50): screen (80..200, 80..130).
        let out = drag(&mut k, (80.0, 80.0), (200.0, 130.0), &u, &mut c);
        assert_eq!(out, vec![UiCommand::Select(vec![0, 1, 2])]);
        // Shift adds, a click on empty ground clears.
        k.shift = true;
        let s = c.to_screen((180.0, 100.0));
        click(&mut k, Button::Left, s, &u, &mut c);
        assert_eq!(k.selected, vec![0, 1, 2, 4]);
        k.shift = false;
        click(&mut k, Button::Left, (900.0, 700.0), &u, &mut c);
        assert!(k.selected.is_empty());
    }

    #[test]
    fn a_click_picks_within_pixels_at_any_zoom() {
        let (u, mut c, mut k) = (squad(), cam(), Controls::new(0));
        for zoom in [0.75, 1.0, 4.0] {
            c.zoom_to(zoom, (0.0, 0.0));
            c.centre_on((140.0, 100.0));
            let s = c.to_screen((140.0, 100.0));
            k.last_pick = None;
            click(&mut k, Button::Left, (s.0 + 6.0, s.1 - 5.0), &u, &mut c);
            assert_eq!(k.selected, vec![2], "zoom {zoom}");
        }
        // A second click on the same unit soon after: all of ours on screen.
        let s = c.to_screen((140.0, 100.0));
        click(&mut k, Button::Left, s, &u, &mut c);
        let mut on: Vec<usize> = u.iter().filter(|q| q.alive && q.team == 0).filter(|q| {
            let p = c.to_screen((q.x, q.y));
            p.0 >= 0.0 && p.1 >= 0.0 && p.0 < c.screen.0 && p.1 < c.screen.1
        }).map(|q| q.id).collect();
        on.sort();
        let mut got = k.selected.clone();
        got.sort();
        assert!(got.len() > 1 && got == on, "{got:?} against {on:?}");
        k.last_pick = None;
        // The other team's unit cannot be picked.
        let s = c.to_screen((120.0, 110.0));
        click(&mut k, Button::Left, s, &u, &mut c);
        assert!(k.selected.is_empty());
    }

    #[test]
    fn right_click_orders_at_the_world_point_and_f_drag_burns() {
        let (u, mut c, mut k) = (squad(), cam(), Controls::new(0));
        k.selected = vec![1, 3];
        let out = click(&mut k, Button::Right, (300.0, 200.0), &u, &mut c);
        assert_eq!(out, vec![UiCommand::Order { units: vec![1, 3], at: (200.0, 150.0) }]);
        k.key(KeyCode::KeyF, true, false, &u, &mut c);
        let out = drag(&mut k, (100.0, 100.0), (300.0, 100.0), &u, &mut c);
        assert_eq!(out, vec![UiCommand::Burn { units: vec![1, 3], from: (100.0, 100.0), to: (200.0, 100.0) }]);
    }

    #[test]
    fn groups_keep_their_units_and_other_keys_go_to_the_game() {
        // Far from the world's corner, so centring the camera needs no clamp.
        let u: Vec<Unit> = squad().into_iter().map(|u| Unit { x: u.x + 500.0, y: u.y + 500.0, ..u }).collect();
        let (mut c, mut k) = (cam(), Controls::new(0));
        k.selected = vec![0, 4];
        k.key(KeyCode::ControlLeft, true, false, &u, &mut c);
        k.key(KeyCode::Digit3, true, false, &u, &mut c);
        k.key(KeyCode::ControlLeft, false, false, &u, &mut c);
        k.selected.clear();
        assert_eq!(k.key(KeyCode::Digit3, true, false, &u, &mut c), vec![UiCommand::Select(vec![0, 4])]);
        // The second press centres the camera on the group.
        k.key(KeyCode::Digit3, true, false, &u, &mut c);
        let m = c.centre();
        assert!((m.0 - 640.0).abs() < 1e-3 && (m.1 - 600.0).abs() < 1e-3, "{m:?}");
        k.moved(Some((0.0, 0.0)), &mut c, None);
        let out = k.key(KeyCode::KeyA, true, false, &u, &mut c);
        assert!(matches!(&out[..], [UiCommand::Key { code: KeyCode::KeyA, units, at: Some(_) }] if units == &vec![0, 4]));
        // A held key's repeats give nothing.
        assert!(k.key(KeyCode::KeyA, true, true, &u, &mut c).is_empty());
    }

    #[test]
    fn the_minimap_moves_the_camera_and_takes_orders() {
        let (u, mut c, mut k) = (squad(), cam(), Controls::new(0));
        let mm = Some((1000, 600, 256, 160));
        k.moved(Some((1128.0, 680.0)), &mut c, mm);
        k.button(Button::Left, true, &u, &mut c, mm);
        let m = c.centre();
        assert!((m.0 - 1024.0).abs() < 1.0 && (m.1 - 640.0).abs() < 1.0, "{m:?}");
        k.button(Button::Left, false, &u, &mut c, mm);
        assert!(k.selected.is_empty(), "a minimap click selects nothing");
        k.selected = vec![2];
        let out = k.button(Button::Right, true, &u, &mut c, mm);
        assert_eq!(out, vec![UiCommand::Order { units: vec![2], at: (1024.0, 640.0) }]);
    }

    #[test]
    fn middle_drag_pans_and_dead_units_leave_the_groups() {
        let (mut u, mut c, mut k) = (squad(), cam(), Controls::new(0));
        let before = (c.x, c.y);
        k.moved(Some((500.0, 500.0)), &mut c, None);
        k.button(Button::Middle, true, &u, &mut c, None);
        k.moved(Some((480.0, 490.0)), &mut c, None);
        k.button(Button::Middle, false, &u, &mut c, None);
        assert!((c.x - before.0 - 10.0).abs() < 1e-3 && (c.y - before.1 - 5.0).abs() < 1e-3);
        k.selected = vec![0, 1];
        k.groups[0] = vec![0, 1];
        u[1].alive = false;
        k.prune(&u);
        assert_eq!((k.selected.clone(), k.groups[0].clone()), (vec![0], vec![0]));
    }
}
