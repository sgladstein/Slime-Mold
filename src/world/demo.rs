//! **The stand-in squad**: units that walk, fight and spend by made-up
//! rules, so the camera, the controls and the UI can be tried on the GPU
//! sandbox (`mouldgpu`, Q) before the Larder world's rules exist (W2b).
//! The rules here are placeholders, not a design: the mould does not see
//! the squad, and the squad does not touch the mould. Two teams: yours
//! (white) and rivals (blue) that stand guard and trade fire with soldiers
//! who come close, so health bars, alerts and losses show.
//!
//! It is also the worked example of filling a `ui::Model` and answering
//! `HudEvent`s, for the Larder world's window (W2c).

use winit::keyboard::KeyCode;

use super::controls::{unit_marks, UiCommand, Unit};
use super::icons::Icon;
use super::marks::{team_colour, Marks};
use super::ui::{theme, Card, Command, HudEvent, Info, Model, Resource, Target, Ghost};

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Stance {
    HoldFire,
    Defend,
    Free,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Weapon {
    Rifle,
    Flamer,
}

impl Weapon {
    fn range(self) -> f32 {
        match self {
            Weapon::Rifle => 24.0,
            Weapon::Flamer => 10.0,
        }
    }
    fn damage(self) -> f32 {
        match self {
            Weapon::Rifle => 9.0,
            Weapon::Flamer => 22.0,
        }
    }
    fn name(self) -> &'static str {
        match self {
            Weapon::Rifle => "Rifle",
            Weapon::Flamer => "Flamer",
        }
    }
    fn icon(self) -> Icon {
        match self {
            Weapon::Rifle => Icon::Rifle,
            Weapon::Flamer => Icon::Flamer,
        }
    }
}

#[derive(Clone, Copy, Debug)]
pub struct StandIn {
    pub x: f32,
    pub y: f32,
    pub team: u8,
    pub hp: f32,
    pub to: Option<(f32, f32)>,
    pub burn: Option<((f32, f32), (f32, f32))>,
    pub stance: Stance,
    pub weapon: Weapon,
    /// Attack-moving: fights whatever it meets on the way.
    pub attack: bool,
    /// Hold position: does not chase.
    pub hold: bool,
    warned: bool,
}

/// What the stand-in squad tells the window to show as an alert.
#[derive(Clone, Debug, PartialEq)]
pub struct Note {
    pub text: String,
    pub at: Option<(f32, f32)>,
    pub bad: bool,
}

pub const MAX_HP: f32 = 100.0;
const RECRUIT: f32 = 400.0;
const TURRET: f32 = 600.0;
const WALL_A_CELL: f32 = 2.0;
const UPGRADE: f32 = 800.0;
const UPGRADE_SECONDS: f32 = 12.0;

// Command ids (the game's own; the UI echoes them back).
pub const ATTACK: u32 = 1;
pub const STOP: u32 = 2;
pub const HOLD: u32 = 3;
pub const BURN: u32 = 4;
pub const HOLD_FIRE: u32 = 5;
pub const DEFEND: u32 = 6;
pub const FREE: u32 = 7;
pub const SWAP: u32 = 8;
pub const RECRUIT_ID: u32 = 9;
pub const TURRET_ID: u32 = 10;
pub const WALL_ID: u32 = 11;
pub const UPGRADE_ID: u32 = 12;

pub struct Squad {
    pub units: Vec<StandIn>,
    /// World cells a second.
    pub speed: f32,
    /// Fine cells a dish cell, for sizes.
    pub scale: f32,
    pub energy: f32,
    pub time: f32,
    pub turrets: Vec<(f32, f32)>,
    pub walls: Vec<((f32, f32), (f32, f32))>,
    /// Armour level (each takes a fifth off the damage taken).
    pub armour: u32,
    /// Seconds left on the upgrade under way.
    pub research: Option<f32>,
    pub home: (f32, f32),
    pub world: (f32, f32),
    pub notes: Vec<Note>,
    pub lost: usize,
}

impl Squad {
    pub fn new(world: (usize, usize), scale: usize) -> Self {
        let (w, h) = (world.0 as f32, world.1 as f32);
        let s = scale as f32;
        let mut units = Vec::new();
        let unit = |x, y, team| StandIn { x, y, team, hp: MAX_HP, to: None, burn: None, stance: Stance::Defend, weapon: Weapon::Rifle, attack: false, hold: false, warned: false };
        for k in 0..12 {
            let (i, j) = ((k % 4) as f32, (k / 4) as f32);
            let mut u = unit(w * 0.3 + i * 4.0 * s, h * 0.5 + j * 4.0 * s, 0);
            if k % 4 == 3 {
                u.weapon = Weapon::Flamer;
            }
            units.push(u);
        }
        for k in 0..6 {
            units.push(unit(w * 0.7 + (k % 3) as f32 * 4.0 * s, h * 0.4 + (k / 3) as f32 * 4.0 * s, 1));
        }
        // Larder's soldier speed, about 12 dish cells a second.
        Squad { units, speed: 12.0 * s, scale: s, energy: 1500.0, time: 0.0, turrets: Vec::new(), walls: Vec::new(), armour: 0, research: None, home: (w * 0.28, h * 0.5), world: (w, h), notes: Vec::new(), lost: 0 }
    }

    /// A unit's radius in world cells: about Larder's soldier, a dish cell.
    pub fn radius(&self) -> f32 {
        0.9 * self.scale
    }

    pub fn units(&self) -> Vec<Unit> {
        self.units.iter().enumerate().map(|(id, u)| Unit { id, team: u.team, x: u.x, y: u.y, alive: u.hp > 0.0 }).collect()
    }

    /// Spreads `ids` round `at` in a square, `gap` cells apart, so they do
    /// not stack on one cell.
    pub fn spread(ids: &[usize], at: (f32, f32), gap: f32) -> Vec<(usize, (f32, f32))> {
        let side = (ids.len() as f32).sqrt().ceil().max(1.0) as usize;
        ids.iter()
            .enumerate()
            .map(|(n, &k)| {
                let (i, j) = ((n % side) as f32, (n / side) as f32);
                let off = (side as f32 - 1.0) / 2.0;
                (k, (at.0 + (i - off) * gap, at.1 + (j - off) * gap))
            })
            .collect()
    }

    fn living(&self, ids: &[usize]) -> Vec<usize> {
        ids.iter().copied().filter(|&k| self.units.get(k).is_some_and(|u| u.hp > 0.0 && u.team == 0)).collect()
    }

    pub fn command(&mut self, c: UiCommand) {
        let gap = 4.0 * self.scale;
        match c {
            UiCommand::Order { units, at } => {
                for (k, p) in Self::spread(&self.living(&units), at, gap) {
                    let u = &mut self.units[k];
                    (u.to, u.burn, u.attack, u.hold) = (Some(p), None, false, false);
                }
            }
            UiCommand::Burn { units, from, to } => self.burn(&units, from, to),
            _ => {}
        }
    }

    fn burn(&mut self, units: &[usize], from: (f32, f32), to: (f32, f32)) {
        // Each takes a place along the line.
        let units = self.living(units);
        let n = units.len().max(1) as f32;
        for (i, &k) in units.iter().enumerate() {
            let f = (i as f32 + 0.5) / n;
            let u = &mut self.units[k];
            u.to = Some((from.0 + (to.0 - from.0) * f, from.1 + (to.1 - from.1) * f));
            u.burn = Some((from, to));
        }
    }

    fn note(&mut self, text: String, at: Option<(f32, f32)>, bad: bool) {
        self.notes.push(Note { text, at, bad });
    }

    /// A UI event, for the selected `units`.
    pub fn hud(&mut self, e: &HudEvent, units: &[usize]) {
        let sel = self.living(units);
        let gap = 4.0 * self.scale;
        match *e {
            HudEvent::Command { id } => match id {
                STOP => {
                    for &k in &sel {
                        let u = &mut self.units[k];
                        (u.to, u.burn, u.attack, u.hold) = (None, None, false, false);
                    }
                }
                HOLD => {
                    for &k in &sel {
                        let u = &mut self.units[k];
                        (u.to, u.attack, u.hold) = (None, false, true);
                    }
                }
                HOLD_FIRE | DEFEND | FREE => {
                    let st = match id {
                        HOLD_FIRE => Stance::HoldFire,
                        DEFEND => Stance::Defend,
                        _ => Stance::Free,
                    };
                    for &k in &sel {
                        self.units[k].stance = st;
                    }
                }
                SWAP => {
                    // All to the other weapon of the first one's.
                    let to = match sel.first().map(|&k| self.units[k].weapon) {
                        Some(Weapon::Rifle) => Weapon::Flamer,
                        _ => Weapon::Rifle,
                    };
                    for &k in &sel {
                        self.units[k].weapon = to;
                    }
                }
                RECRUIT_ID if self.energy >= RECRUIT => {
                    self.energy -= RECRUIT;
                    let n = self.units.iter().filter(|u| u.team == 0).count() as f32;
                    let at = (self.home.0 + (n % 4.0) * gap, self.home.1 - 3.0 * gap);
                    self.units.push(StandIn { x: at.0, y: at.1, team: 0, hp: MAX_HP, to: None, burn: None, stance: Stance::Defend, weapon: Weapon::Rifle, attack: false, hold: false, warned: false });
                    self.note("A soldier has joined at the larder".into(), Some(at), false);
                }
                UPGRADE_ID if self.energy >= UPGRADE && self.research.is_none() && self.armour < 3 => {
                    self.energy -= UPGRADE;
                    self.research = Some(UPGRADE_SECONDS);
                }
                _ => {}
            },
            HudEvent::Point { id, at } => match id {
                ATTACK => {
                    for (k, p) in Self::spread(&sel, at, gap) {
                        let u = &mut self.units[k];
                        (u.to, u.burn, u.attack, u.hold) = (Some(p), None, true, false);
                    }
                }
                TURRET_ID if self.energy >= TURRET && self.can_build(at) => {
                    self.energy -= TURRET;
                    self.turrets.push(at);
                }
                _ => {}
            },
            HudEvent::Line { id, from, to } => match id {
                BURN => self.burn(&sel, from, to),
                WALL_ID => {
                    let cost = self.wall_cost(from, to);
                    if self.energy >= cost {
                        self.energy -= cost;
                        self.walls.push((from, to));
                    }
                }
                _ => {}
            },
            _ => {}
        }
    }

    pub fn wall_cost(&self, a: (f32, f32), b: (f32, f32)) -> f32 {
        ((a.0 - b.0).hypot(a.1 - b.1) / self.scale * WALL_A_CELL).ceil()
    }

    /// A turret can go where no unit or turret already stands.
    pub fn can_build(&self, at: (f32, f32)) -> bool {
        let r = 4.0 * self.scale;
        let (w, h) = self.world;
        at.0 > r && at.1 > r && at.0 < w - r && at.1 < h - r && !self.units.iter().any(|u| u.hp > 0.0 && (u.x - at.0).hypot(u.y - at.1) < r) && !self.turrets.iter().any(|t| (t.0 - at.0).hypot(t.1 - at.1) < 2.0 * r)
    }

    pub fn step(&mut self, dt: f32) {
        let dt = dt.min(0.1);
        if dt <= 0.0 {
            return;
        }
        self.time += dt;
        self.energy += 6.0 * dt;
        if let Some(r) = self.research.as_mut() {
            *r -= dt;
            if *r <= 0.0 {
                self.research = None;
                self.armour += 1;
                let a = self.armour;
                self.note(format!("Armour {a} is done: soldiers take {}% less damage", a * 20), None, false);
            }
        }
        let step = self.speed * dt;
        for u in &mut self.units {
            if u.hp <= 0.0 {
                continue;
            }
            if let Some((tx, ty)) = u.to {
                let (dx, dy) = (tx - u.x, ty - u.y);
                let d = dx.hypot(dy);
                if d <= step {
                    (u.x, u.y) = (tx, ty);
                    u.to = None;
                    u.attack = false;
                } else {
                    u.x += dx / d * step;
                    u.y += dy / d * step;
                }
            }
        }
        // Fire: each living unit hits its nearest enemy in range, unless
        // holding fire; rivals always fire. Turrets fire as rifles.
        let s = self.scale;
        let n = self.units.len();
        let mut hurt = vec![0.0f32; n];
        for i in 0..n {
            let u = self.units[i];
            if u.hp <= 0.0 || (u.team == 0 && u.stance == Stance::HoldFire) {
                continue;
            }
            let reach = u.weapon.range() * s * if u.team == 0 && u.stance == Stance::Free { 1.3 } else { 1.0 };
            let near = (0..n).filter(|&j| self.units[j].team != u.team && self.units[j].hp > 0.0).map(|j| (j, (self.units[j].x - u.x).hypot(self.units[j].y - u.y))).filter(|&(_, d)| d <= reach).min_by(|a, b| a.1.total_cmp(&b.1));
            if let Some((j, _)) = near {
                let dmg = u.weapon.damage() * if u.team == 1 { 0.6 } else { 1.0 };
                hurt[j] += dmg * dt;
            }
        }
        for t in self.turrets.clone() {
            let reach = 30.0 * s;
            if let Some(j) = (0..n).filter(|&j| self.units[j].team == 1 && self.units[j].hp > 0.0 && (self.units[j].x - t.0).hypot(self.units[j].y - t.1) <= reach).min_by(|&a, &b| (self.units[a].x - t.0).hypot(self.units[a].y - t.1).total_cmp(&(self.units[b].x - t.0).hypot(self.units[b].y - t.1))) {
                hurt[j] += 8.0 * dt;
            }
        }
        for (j, &h) in hurt.iter().enumerate() {
            if h <= 0.0 {
                continue;
            }
            let armour = if self.units[j].team == 0 { 1.0 - 0.2 * self.armour as f32 } else { 1.0 };
            let before = self.units[j].hp;
            let after = (before - h * armour).max(0.0);
            self.units[j].hp = after;
            let at = Some((self.units[j].x, self.units[j].y));
            if self.units[j].team == 0 {
                if after < 50.0 && !self.units[j].warned {
                    self.units[j].warned = true;
                    self.note(format!("Soldier {} is under fire", j + 1), at, true);
                }
                if after <= 0.0 && before > 0.0 {
                    self.lost += 1;
                    self.note(format!("Soldier {} was lost", j + 1), at, true);
                }
            } else if after <= 0.0 && before > 0.0 {
                self.note("A rival soldier is down".into(), at, false);
            }
        }
    }

    pub fn marks(&self, zoom: f32, out: &mut Marks) {
        let s = self.scale;
        for &(a, b) in &self.walls {
            // A wall: a thick line of discs, as it would sit in the ground.
            let n = ((a.0 - b.0).hypot(a.1 - b.1) / s).ceil().max(1.0) as usize;
            for i in 0..=n {
                let f = i as f32 / n as f32;
                out.disc((a.0 + (b.0 - a.0) * f, a.1 + (b.1 - a.1) * f), 0.9 * s, [150, 140, 125, 255], i % 6 == 0);
            }
        }
        for &t in &self.turrets {
            out.rect(t, (1.6 * s, 1.6 * s), [200, 205, 210, 255], true);
            out.ring(t, 30.0 * s, [255, 255, 255, 40]);
        }
        for (id, u) in self.units.iter().enumerate() {
            if let Some((a, b)) = u.burn {
                out.line(a, b, 4.0, [255, 150, 60, 255]);
            }
            if let (Some(to), true) = (u.to, u.hp > 0.0) {
                out.line((u.x, u.y), to, 2.0, if u.attack { [255, 110, 110, 120] } else { [255, 255, 255, 90] });
            }
            unit_marks(&Unit { id, team: u.team, x: u.x, y: u.y, alive: u.hp > 0.0 }, self.radius(), u.hp / MAX_HP, zoom, out);
        }
    }

    fn stance_of(&self, sel: &[usize]) -> Option<Stance> {
        let first = self.units[*sel.first()?].stance;
        sel.iter().all(|&k| self.units[k].stance == first).then_some(first)
    }

    /// The UI's model this frame: `sel` the selection, `hover` the world
    /// point under the cursor, `ghost` the targeting command and point.
    pub fn model(&self, sel: &[usize], hover: Option<(f32, f32)>, ghost: Option<Ghost>, paused: bool) -> Model {
        let sel = self.living(sel);
        let mine = self.units.iter().filter(|u| u.team == 0 && u.hp > 0.0).count();
        let t = self.time as u32;
        let mut m = Model {
            resources: vec![
                Resource { icon: Icon::Crystal, colour: theme::ENERGY, label: "Energy: crystals harvested, spent on soldiers and building".into(), value: format!("{:.0}", self.energy), rate: Some("+6/s".into()) },
                Resource { icon: Icon::Soldier, colour: theme::TEXT, label: "Soldiers alive".into(), value: format!("{mine}"), rate: (self.lost > 0).then(|| format!("-{} lost", self.lost)) },
                Resource { icon: Icon::Shield, colour: theme::ACCENT, label: "Armour level (upgrades)".into(), value: format!("{}", self.armour), rate: self.research.map(|r| format!("{r:.0}s")) },
            ],
            clock: format!("{}:{:02}", t / 60, t % 60),
            speed: paused.then(|| "Paused".to_string()),
            objective: Some("Stand-in squad: try the controls (no game rules yet)".into()),
            idle: self.units.iter().filter(|u| u.team == 0 && u.hp > 0.0 && u.to.is_none() && u.burn.is_none() && !u.hold).count(),
            ..Model::default()
        };
        m.selection = sel.iter().map(|&k| {
            let u = &self.units[k];
            Card { id: k, icon: Icon::Soldier, team: 0, hp: u.hp / MAX_HP, badge: Some(u.weapon.icon()) }
        }).collect();
        // Info: a hovered rival, else one selected soldier.
        let pick = 2.5 * self.scale;
        let hovered = hover.and_then(|p| self.units.iter().enumerate().find(|(_, u)| u.hp > 0.0 && (u.x - p.0).hypot(u.y - p.1) < pick));
        let info_of = |k: usize, u: &StandIn| {
            let (title, colour) = if u.team == 0 { (format!("Soldier {}", k + 1), team_colour(0)) } else { (format!("Rival soldier {}", k + 1), team_colour(1)) };
            let doing = if u.hold {
                "holding position"
            } else if u.burn.is_some() {
                "holding a burn line"
            } else if u.attack {
                "attack-moving"
            } else if u.to.is_some() {
                "moving"
            } else {
                "idle"
            };
            let stance = match u.stance {
                Stance::HoldFire => "Hold fire",
                Stance::Defend => "Defend",
                Stance::Free => "Free fire",
            };
            Info {
                icon: Icon::Soldier,
                colour,
                title,
                subtitle: format!("{}, {doing}", u.weapon.name()),
                hp: Some((u.hp / MAX_HP, format!("{:.0} / {:.0}", u.hp, MAX_HP))),
                stats: vec![
                    ("Weapon".into(), u.weapon.name().into()),
                    ("Range".into(), format!("{:.0} cells", u.weapon.range())),
                    ("Damage".into(), format!("{:.0}/s", u.weapon.damage())),
                    ("Stance".into(), stance.into()),
                    ("Armour".into(), if u.team == 0 { format!("{}", self.armour) } else { "0".into() }),
                ],
                note: (u.team == 1).then(|| "An enemy: right-click it with soldiers selected to attack".to_string()),
            }
        };
        if let Some((k, u)) = hovered.filter(|(_, u)| u.team == 1) {
            m.info = Some(info_of(k, u));
        } else if sel.len() == 1 {
            m.info = Some(info_of(sel[0], &self.units[sel[0]]));
        }
        let any = !sel.is_empty();
        let stance = self.stance_of(&sel);
        let weapon = sel.first().map(|&k| self.units[k].weapon);
        let need = |cost: f32| (self.energy < cost).then(|| format!("Need {:.0} more energy", cost - self.energy));
        let no_sel = || Some("Select soldiers first".to_string());
        let mut cmds = vec![
            Command { target: Target::Point { radius: 3.0 * self.scale }, ready: any, why_not: no_sel(), about: "Move there, fighting whatever is met on the way.".into(), ..Command::new(ATTACK, (0, 0), Icon::Attack, "Attack-move", KeyCode::KeyA) },
            Command { ready: any, why_not: no_sel(), about: "Drop every order and stand still.".into(), ..Command::new(STOP, (0, 1), Icon::Stop, "Stop", KeyCode::KeyS) },
            Command { ready: any, why_not: no_sel(), about: "Stay put and do not chase; still fires by its stance.".into(), ..Command::new(HOLD, (0, 2), Icon::Hold, "Hold position", KeyCode::KeyH) },
            Command { target: Target::Line { width: 2.0 * self.scale }, ready: any, why_not: no_sel(), about: "Spread out along a line and keep it burning, so the mould cannot cross.".into(), ..Command::new(BURN, (0, 3), Icon::Burn, "Burn a line", KeyCode::KeyF) },
            Command { ready: any, on: stance == Some(Stance::HoldFire), why_not: no_sel(), about: "Do not shoot, even when fired on. Saves the mould's attention.".into(), ..Command::new(HOLD_FIRE, (1, 0), Icon::Close, "Hold fire", KeyCode::KeyZ) },
            Command { ready: any, on: stance == Some(Stance::Defend), why_not: no_sel(), about: "Shoot only what comes close. The default.".into(), ..Command::new(DEFEND, (1, 1), Icon::Shield, "Defend", KeyCode::KeyX) },
            Command { ready: any, on: stance == Some(Stance::Free), why_not: no_sel(), about: "Shoot anything in reach, a third further than defending.".into(), ..Command::new(FREE, (1, 2), Icon::Target, "Free fire", KeyCode::KeyC) },
            Command {
                ready: any,
                why_not: no_sel(),
                about: "Change weapons. The rifle reaches 24 cells; the flamer 10, but burns twice as hard.".into(),
                ..Command::new(SWAP, (1, 3), if weapon == Some(Weapon::Rifle) { Icon::Flamer } else { Icon::Rifle }, if weapon == Some(Weapon::Rifle) { "Take the flamer" } else { "Take the rifle" }, KeyCode::KeyE)
            },
            Command { cost: Some(format!("{RECRUIT:.0}")), ready: self.energy >= RECRUIT, why_not: need(RECRUIT), about: "A new soldier at the larder.".into(), ..Command::new(RECRUIT_ID, (2, 0), Icon::Recruit, "Recruit a soldier", KeyCode::KeyB) },
            Command { target: Target::Area { half: (1.6 * self.scale, 1.6 * self.scale) }, cost: Some(format!("{TURRET:.0}")), ready: self.energy >= TURRET, why_not: need(TURRET), about: "A turret that fires at enemies within 30 cells.".into(), ..Command::new(TURRET_ID, (2, 1), Icon::Turret, "Build a turret", KeyCode::KeyT) },
            Command { target: Target::Line { width: 1.8 * self.scale }, cost: Some(format!("{WALL_A_CELL:.0}/cell")), ready: self.energy >= 20.0, why_not: need(20.0), about: "A wall the mould has to wear through. Costs by its length.".into(), ..Command::new(WALL_ID, (2, 2), Icon::Wall, "Lay a wall", KeyCode::KeyV) },
            Command {
                cost: Some(format!("{UPGRADE:.0}")),
                ready: self.energy >= UPGRADE && self.research.is_none() && self.armour < 3,
                cooldown: self.research.map_or(0.0, |r| r / UPGRADE_SECONDS),
                why_not: if self.armour >= 3 { Some("Armour is at its best".into()) } else if self.research.is_some() { Some("Already under way".into()) } else { need(UPGRADE) },
                about: format!("Armour {}: every soldier takes a fifth less damage. Takes {UPGRADE_SECONDS:.0} s.", self.armour + 1),
                ..Command::new(UPGRADE_ID, (2, 3), Icon::Upgrade, "Upgrade armour", KeyCode::KeyU)
            },
        ];
        for c in &mut cmds {
            if c.ready {
                c.why_not = None;
            }
        }
        m.commands = cmds;
        m.ghost_ok = match ghost {
            Some((TURRET_ID, at, _)) => self.can_build(at),
            Some((WALL_ID, at, Some(from))) => self.energy >= self.wall_cost(from, at),
            Some(_) => true,
            None => true,
        };
        m.help = vec![("Q".into(), "Put the squad away".into())];
        m
    }

    /// The idle soldier after `after` (the idle button cycles).
    pub fn next_idle(&self, after: Option<usize>) -> Option<usize> {
        let idle: Vec<usize> = (0..self.units.len()).filter(|&k| {
            let u = &self.units[k];
            u.team == 0 && u.hp > 0.0 && u.to.is_none() && u.burn.is_none() && !u.hold
        }).collect();
        let start = after.and_then(|a| idle.iter().position(|&k| k > a)).unwrap_or(0);
        idle.get(start).copied()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn recruiting_spends_and_the_model_greys_out_what_you_cannot_afford() {
        let mut sq = Squad::new((2048, 1280), 8);
        sq.energy = 450.0;
        sq.hud(&HudEvent::Command { id: RECRUIT_ID }, &[]);
        assert_eq!(sq.units.iter().filter(|u| u.team == 0).count(), 13);
        assert!(sq.energy < 60.0);
        let m = sq.model(&[0], None, None, false);
        let r = m.commands.iter().find(|c| c.id == RECRUIT_ID).unwrap();
        assert!(!r.ready && r.why_not.as_deref().unwrap().starts_with("Need"));
        assert!(m.info.is_some(), "one selected: its detail");
        assert_eq!(m.selection.len(), 1);
    }

    #[test]
    fn rivals_in_range_hurt_and_the_squad_says_so() {
        let mut sq = Squad::new((2048, 1280), 8);
        let rival = sq.units.iter().position(|u| u.team == 1).unwrap();
        let (x, y) = (sq.units[rival].x, sq.units[rival].y);
        sq.units[0].x = x - 30.0;
        sq.units[0].y = y;
        sq.units[0].stance = Stance::HoldFire;
        for _ in 0..200 {
            sq.step(0.1);
        }
        assert!(sq.units[0].hp < 50.0);
        assert!(sq.notes.iter().any(|n| n.bad && n.at.is_some()));
    }
}
