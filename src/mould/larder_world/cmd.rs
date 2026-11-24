//! The player's commands (`Cmd`) and how the game applies them. Moved out
//! of `larder_world.rs` unchanged (C9, 2026-11-14).

use super::*;
use crate::mould::kit::Tool;

/// Half a read block (`squad::BIN`, engine cells): about how far past a
/// radius the centre of a block holding a walker inside it can lie, so a
/// defending soldier's test allows it (`Game::sight`). Not the half
/// diagonal (2.8): at scale 1 that took defend's 7.5 cells to 10.3, and
/// `a_guard_shoots_slime_on_what_it_guards` caught slime 10 cells off.
const DEFEND_SLACK: f32 = squad::BIN as f32 * 0.5;
/// Frames a soldier reads its whole reach after a near read found no
/// slime, before it looks near again (`Game::read_aim`): 4 at 15 frames
/// a second, so a walker that comes near is seen within about 0.27 s,
/// and three frames in four aim at the far slime.
const FAR_FRAMES: u8 = 4;

/// A player's command; `Game::apply` keeps the log. As M3's, with walls and
/// buildings (`buildings`) in place of M3's turret and bait.
/// `Order::Clear` names a crystal by a point inside it.
#[derive(Clone, Copy, Debug, PartialEq)]
#[derive(serde::Serialize, serde::Deserialize)]
pub enum Cmd {
    Order(usize, Order),
    Stance(usize, Stance),
    Buy,
    /// A wall between two points, fine cells (`Cmd::Build` of a
    /// `Kind::Wall`, kept so older replays and hands still load).
    Wall((f32, f32), (f32, f32)),
    /// A building at a point, fine cells.
    Build(Kind, (f32, f32)),
    /// Switch building `k` (a lure) on or off.
    Toggle(usize),
    /// Call off ghost `b` (`Game::unbuild`): it goes, its price comes back.
    Unbuild(usize),
    /// Upgrade pump `b` a level (`Game::upgrade`), paid now.
    Upgrade(usize),
    /// Pull down standing building `b` (`Game::demolish`), part of its
    /// price back. Appended last in replay words, as every new command.
    Demolish(usize),
    /// Shift + an order: soldier `k` does it after the ones it has
    /// (`Soldier::queue`); a soldier holding starts it at once.
    Queue(usize, Order),
    /// The larder's rally point (`Game::rally`): what a bought soldier is
    /// first ordered to do; `Order::Hold` clears it.
    Rally(Order),
    /// Send a builder to ghost `b` (`Game::send`): the nearest idle
    /// soldier, else the nearest one not already building, which goes
    /// back to its order once the building stands. What a build order with
    /// no soldier selected does, so a ghost placed far away still rises.
    Send(usize),
    /// Squad commands (`squads`). Steer squad `q` (a direction, zero to
    /// stop; WASD).
    Steer(u8, (f32, f32)),
    /// Every member of squad `q` takes the order; a move or fight sends
    /// the squad there in formation.
    SquadOrder(u8, Order),
    /// Squad `q` volleys at the point while it is Some (left button held).
    Fire(u8, Option<(f32, f32)>),
    /// Squad `q` faces the point (the cursor).
    Face(u8, (f32, f32)),
    /// Squad `q` splits; the half nearer the point is a new squad.
    Split(u8, (f32, f32)),
    /// Soldier `k` joins squad `q` (a free id starts a new squad).
    Join(usize, u8),
    /// Squad `b` joins squad `a`.
    Merge(u8, u8),
    /// The player controls squad `q` (`u8::MAX`: none).
    Active(u8),
    /// Squad `q` takes a formation and a spread step (`squads::SPREADS`).
    Formation(u8, Formation, u8),
    /// Shift + a squad order: squad `q` does it after what it is doing
    /// (`Squad::queue`); an idle squad starts it at once.
    SquadQueue(u8, Order),
    /// Links squads `a` and `b` (and whatever each is linked to): they move
    /// and take orders together, each keeping its own guns and settings
    /// (owner 2026-11-14 15:06Z, "join").
    Link(u8, u8),
    /// Squad `q` leaves its link.
    Unlink(u8),
    /// The squads whose bits are set (bit `q` for squad `q`), and every
    /// squad linked with one, are picked together, as a drag over them
    /// picks them (`Squad::band`; owner 2026-11-21 06:53Z): commanded as
    /// one, as a link is, until the player commands a squad outside them or
    /// lets go. 0 drops the pick.
    Band(u16),
    /// Squad `q` sets tool `t` off from its device slot (`Game::device`,
    /// `Kit::device`; the squad mode's tools, owner 2026-11-21 06:55Z).
    Device(u8, Tool),
    /// The bodies whose bits are set (bit `t` for tool `t`, `kit::TOOLS`
    /// order) go off on their own (`Game::autocast`, gun step E4).
    Auto(u8),
    /// Squad `q` takes a gun (`Game::arm`; owner 11-14 16:46Z, "The squad
    /// owns the gun"); None gives back the standard rifle.
    Arm(u8, Option<Gun>),
    /// Squad `q` takes its own cards (`Game::load`); None the army's.
    Load(u8, Option<Deck>),
    /// Turret (or mortar) `b` takes its own cards from the run's loadout
    /// `l` (`Building::deck`, `Building::loadout`; owner 2026-11-19: each
    /// turret changed one by one); None, None: back on the turrets' deck.
    TurretLoad(u32, Option<u8>, Option<Deck>),
    /// Squad `q` stops and heals (`Squad::heal`).
    Heal(u8),
    /// A door between two points, fine cells (`Game::door`): a wall the
    /// soldiers walk through, snapped onto a standing wall it lies along.
    Door((f32, f32), (f32, f32)),
}

impl Cmd {
    pub fn to_line(self) -> String {
        match self {
            Cmd::Order(k, o) => super::super::larder::Cmd::Order(k, o).to_line(),
            Cmd::Stance(k, s) => super::super::larder::Cmd::Stance(k, s).to_line(),
            Cmd::Buy => "buy".into(),
            Cmd::Wall(a, b) => format!("wall {} {} {} {}", a.0, a.1, b.0, b.1),
            Cmd::Build(Kind::Wall { to }, a) => format!("build wall {} {} {} {}", a.0, a.1, to.0, to.1),
            Cmd::Build(k, a) => format!("build {} {} {}", k.name(), a.0, a.1),
            Cmd::Toggle(k) => format!("toggle {k}"),
            Cmd::Unbuild(b) => format!("unbuild {b}"),
            Cmd::Upgrade(b) => format!("upgrade {b}"),
            Cmd::Demolish(b) => format!("demolish {b}"),
            Cmd::Queue(k, o) => format!("queue{}", &super::super::larder::Cmd::Order(k, o).to_line()["order".len()..]),
            Cmd::Rally(o) => format!("rally{}", &super::super::larder::Cmd::Order(0, o).to_line()["order 0".len()..]),
            Cmd::Send(b) => format!("send {b}"),
            Cmd::Steer(q, d) => format!("steer {q} {} {}", d.0, d.1),
            Cmd::SquadOrder(q, o) => format!("sorder {q}{}", &super::super::larder::Cmd::Order(0, o).to_line()["order 0".len()..]),
            Cmd::Fire(q, Some(p)) => format!("fire {q} {} {}", p.0, p.1),
            Cmd::Fire(q, None) => format!("fire {q} none"),
            Cmd::Face(q, p) => format!("face {q} {} {}", p.0, p.1),
            Cmd::Split(q, p) => format!("split {q} {} {}", p.0, p.1),
            Cmd::Join(k, q) => format!("join {k} {q}"),
            Cmd::Merge(a, b) => format!("merge {a} {b}"),
            Cmd::Active(q) => format!("active {q}"),
            Cmd::Formation(q, f, s) => format!("formation {q} {} {s}", f.name()),
            Cmd::SquadQueue(q, o) => format!("squeue {q}{}", &super::super::larder::Cmd::Order(0, o).to_line()["order 0".len()..]),
            Cmd::Link(a, b) => format!("link {a} {b}"),
            Cmd::Unlink(q) => format!("unlink {q}"),
            Cmd::Band(m) => format!("band {m}"),
            Cmd::Device(q, t) => {
                let name = crate::mould::kit::TOOLS[t.index()];
                match t {
                    Tool::Cord(a, b) => format!("device {q} {name} {} {} {} {}", a.0, a.1, b.0, b.1),
                    Tool::Smell(k, m) => format!("device {q} {name} {k} {m}"),
                    _ => {
                        let p = crate::mould::kit::Kit::aim(t);
                        format!("device {q} {name} {} {}", p.0, p.1)
                    }
                }
            }
            Cmd::Auto(m) => format!("auto {m}"),
            Cmd::Heal(q) => format!("heal {q}"),
            Cmd::Arm(q, None) => format!("arm {q} none"),
            Cmd::Arm(q, Some(g)) => format!("arm {q} {}", g.to_words()),
            Cmd::Load(q, None) => format!("load {q} army"),
            Cmd::Load(q, Some(d)) => format!("load {q} {}", super::super::rounds::name(&d.cards())),
            Cmd::TurretLoad(b, l, d) => format!("tload {b} {} {}", l.map_or("-".to_string(), |l| l.to_string()), d.map_or("army".to_string(), |d| super::super::rounds::name(&d.cards()))),
            Cmd::Door(a, b) => format!("door {} {} {} {}", a.0, a.1, b.0, b.1),
        }
    }

    pub fn parse(line: &str) -> Option<Cmd> {
        let w: Vec<&str> = line.split_whitespace().collect();
        let f = |i: usize| w.get(i).and_then(|v| v.parse::<f32>().ok());
        match *w.first()? {
            "wall" => return Some(Cmd::Wall((f(1)?, f(2)?), (f(3)?, f(4)?))),
            "door" => return Some(Cmd::Door((f(1)?, f(2)?), (f(3)?, f(4)?))),
            "toggle" => return Some(Cmd::Toggle(w.get(1)?.parse().ok()?)),
            "unbuild" => return Some(Cmd::Unbuild(w.get(1)?.parse().ok()?)),
            "upgrade" => return Some(Cmd::Upgrade(w.get(1)?.parse().ok()?)),
            "demolish" => return Some(Cmd::Demolish(w.get(1)?.parse().ok()?)),
            "send" => return Some(Cmd::Send(w.get(1)?.parse().ok()?)),
            "steer" => return Some(Cmd::Steer(w.get(1)?.parse().ok()?, (f(2)?, f(3)?))),
            "fire" if w.get(2) == Some(&"none") => return Some(Cmd::Fire(w.get(1)?.parse().ok()?, None)),
            "fire" => return Some(Cmd::Fire(w.get(1)?.parse().ok()?, Some((f(2)?, f(3)?)))),
            "face" => return Some(Cmd::Face(w.get(1)?.parse().ok()?, (f(2)?, f(3)?))),
            "split" => return Some(Cmd::Split(w.get(1)?.parse().ok()?, (f(2)?, f(3)?))),
            "join" => return Some(Cmd::Join(w.get(1)?.parse().ok()?, w.get(2)?.parse().ok()?)),
            "merge" => return Some(Cmd::Merge(w.get(1)?.parse().ok()?, w.get(2)?.parse().ok()?)),
            "active" => return Some(Cmd::Active(w.get(1)?.parse().ok()?)),
            "formation" => {
                // Two words before 2026-11-23 (no spread); a name of the
                // 11-21 set maps to its blob (`Formation::parse`).
                let (f, was) = Formation::parse(w.get(2)?)?;
                let s = w.get(3).and_then(|s| s.parse().ok()).or(was).unwrap_or(0);
                return Some(Cmd::Formation(w.get(1)?.parse().ok()?, f, s));
            }
            "link" => return Some(Cmd::Link(w.get(1)?.parse().ok()?, w.get(2)?.parse().ok()?)),
            "unlink" => return Some(Cmd::Unlink(w.get(1)?.parse().ok()?)),
            "band" => return Some(Cmd::Band(w.get(1)?.parse().ok()?)),
            "device" => {
                let q = w.get(1)?.parse().ok()?;
                let p = (f(3)?, f(4)?);
                let t = match *w.get(2)? {
                    "blast" => Tool::Blast(p),
                    "mine" => Tool::Mine(p),
                    "cord" => Tool::Cord(p, (f(5)?, f(6)?)),
                    "germ" => Tool::Germ(p),
                    "gas" => Tool::Gas(p),
                    "provoke" => Tool::Provoke(p),
                    "still" => Tool::Still(p),
                    _ => return None,
                };
                return Some(Cmd::Device(q, t));
            }
            "auto" => return Some(Cmd::Auto(w.get(1)?.parse().ok()?)),
            "heal" => return Some(Cmd::Heal(w.get(1)?.parse().ok()?)),
            "arm" if w.get(2) == Some(&"none") => return Some(Cmd::Arm(w.get(1)?.parse().ok()?, None)),
            "arm" => return Some(Cmd::Arm(w.get(1)?.parse().ok()?, Some(Gun::from_words(&w[2..])?))),
            "load" if w.get(2) == Some(&"army") => return Some(Cmd::Load(w.get(1)?.parse().ok()?, None)),
            "load" => return Some(Cmd::Load(w.get(1)?.parse().ok()?, Some(Deck::new(&super::super::rounds::parse(w.get(2)?)?)))),
            "tload" => {
                let l = match *w.get(2)? {
                    "-" => None,
                    l => Some(l.parse().ok()?),
                };
                let d = match *w.get(3)? {
                    "army" => None,
                    d => Some(Deck::new(&super::super::rounds::parse(d)?)),
                };
                return Some(Cmd::TurretLoad(w.get(1)?.parse().ok()?, l, d));
            }
            v @ ("sorder" | "squeue") => {
                let q: u8 = w.get(1)?.parse().ok()?;
                let rest = w[2..].join(" ");
                return match super::super::larder::Cmd::parse(&format!("order 0 {rest}"))? {
                    super::super::larder::Cmd::Order(_, o) if v == "sorder" => Some(Cmd::SquadOrder(q, o)),
                    super::super::larder::Cmd::Order(_, o) => Some(Cmd::SquadQueue(q, o)),
                    _ => None,
                };
            }
            "build" => {
                let kind = match *w.get(1)? {
                    "turret" => Kind::Turret,
                    "mortar" => Kind::Mortar,
                    "sprayer" => Kind::Sprayer,
                    "hunter" => Kind::Hunter,
                    "lure" => Kind::Lure,
                    "pump" => Kind::Pump,
                    "outpost" => Kind::Outpost,
                    "siren" => Kind::Siren,
                    "wall" => Kind::Wall { to: (f(4)?, f(5)?) },
                    _ => return None,
                };
                return Some(Cmd::Build(kind, (f(2)?, f(3)?)));
            }
            _ => {}
        }
        if let Some(rest) = line.trim_start().strip_prefix("rally ") {
            return match super::super::larder::Cmd::parse(&format!("order 0 {rest}"))? {
                super::super::larder::Cmd::Order(_, o) => Some(Cmd::Rally(o)),
                _ => None,
            };
        }
        if let Some(rest) = line.trim_start().strip_prefix("queue ") {
            return match super::super::larder::Cmd::parse(&format!("order {rest}"))? {
                super::super::larder::Cmd::Order(k, o) => Some(Cmd::Queue(k, o)),
                _ => None,
            };
        }
        match super::super::larder::Cmd::parse(line)? {
            super::super::larder::Cmd::Order(k, o) => Some(Cmd::Order(k, o)),
            super::super::larder::Cmd::Stance(k, s) => Some(Cmd::Stance(k, s)),
            super::super::larder::Cmd::Buy => Some(Cmd::Buy),
            _ => None,
        }
    }
}

impl<W: World> Game<W> {
    /// Carry out a command and log it at this tick. True if it took effect.
    /// A step still running finishes first (`finish_step`), so a command
    /// lands between whole steps, where a replay puts it.
    pub fn apply(&mut self, c: Cmd) -> bool {
        self.finish_step();
        self.log.push((self.world.ticks(), c));
        match c {
            Cmd::Order(k, o) if k < self.soldiers.len() => {
                self.soldiers[k].order = o;
                self.soldiers[k].homing = false;
                // A new order forgets the last crystal running dry: kept,
                // the next trip home to bank left the new crystal as if it
                // were the dry one (2026-11-14).
                self.soldiers[k].dry = false;
                self.soldiers[k].queued = 0;
                true
            }
            Cmd::Queue(k, o) if k < self.soldiers.len() => {
                let s = &mut self.soldiers[k];
                if s.order == Order::Hold && s.queued == 0 {
                    s.order = o;
                    s.homing = false;
                    true
                } else if (s.queued as usize) < QUEUE {
                    s.queue[s.queued as usize] = o;
                    s.queued += 1;
                    true
                } else {
                    false
                }
            }
            Cmd::Stance(k, st) if k < self.soldiers.len() => {
                self.soldiers[k].stance = st;
                true
            }
            Cmd::Buy => self.buy(),
            Cmd::Wall(a, b) => self.wall(a, b),
            Cmd::Door(a, b) => self.door(a, b),
            Cmd::Build(kind, at) => self.build(kind, at),
            Cmd::Toggle(k) => self.toggle(k),
            Cmd::Unbuild(b) => self.unbuild(b).is_some(),
            Cmd::Upgrade(b) => self.upgrade(b),
            Cmd::Demolish(b) => self.demolish(b).is_some(),
            Cmd::Send(b) => self.send(b).is_some(),
            Cmd::Rally(o) => {
                self.rally = o;
                true
            }
            Cmd::Arm(q, g) => self.arm(q as usize, g),
            Cmd::Load(q, d) => self.load(q as usize, d),
            Cmd::TurretLoad(b, l, d) => match self.buildings.get_mut(b as usize).filter(|b| b.alive && matches!(b.kind, Kind::Turret | Kind::Mortar | Kind::Sprayer | Kind::Hunter)) {
                Some(bd) => {
                    bd.deck = d;
                    bd.loadout = l;
                    bd.next = 0;
                    true
                }
                None => false,
            },
            Cmd::Steer(..) | Cmd::SquadOrder(..) | Cmd::Fire(..) | Cmd::Face(..) | Cmd::Split(..) | Cmd::Join(..) | Cmd::Merge(..) | Cmd::Active(..) | Cmd::Formation(..) | Cmd::SquadQueue(..) | Cmd::Link(..) | Cmd::Unlink(..) | Cmd::Band(..) | Cmd::Heal(..) => self.squad_cmd(c),
            Cmd::Device(q, t) => self.device(q as usize, t),
            Cmd::Auto(m) => {
                self.autocast = m;
                true
            }
            _ => false,
        }
    }

    pub fn buy(&mut self) -> bool {
        // Not once the squad is gone where that ends the round
        // (`Rules::wipe_ends`): its end screen must not show a soldier.
        let gone = self.rules.wipe_ends > 0.0 && self.alive() == 0;
        if self.bank < self.rules.soldier_cost || self.alive() >= self.rules.max_squad || gone {
            return false;
        }
        self.bank -= self.rules.soldier_cost;
        self.spent += self.rules.soldier_cost;
        let a = self.rng.f32() * std::f32::consts::TAU;
        // A bought soldier joins the controlled squad, at its side (owner,
        // 2026-11-14 04:18Z); with none controlled it starts at the larder
        // on the rally order, as before. In the squad mode with none
        // controlled (deselected, owner 2026-11-15 02:58Z) it joins the
        // lowest live squad: there is no larder to start at.
        let join = self.active.filter(|&q| !self.members(q).is_empty()).or_else(|| self.live_squads().first().copied()).filter(|_| self.rules.squads);
        let at = join.and_then(|q| self.middle(q));
        self.add_soldier_at(at.unwrap_or(self.larder), a);
        let gather = join.is_some_and(|q| self.squads[q].steer == (0.0, 0.0) && self.squads[q].dest.is_none());
        // It fires as the squad it joins does (its first member's stance):
        // in the squad mode the fire mode and Hold fire are the squad's, and
        // a newcomer on Defend in a squad told to hold fire shot anyway and
        // showed the card's fire button as "mixed" (controls review,
        // 2026-11-21).
        let stance = join.and_then(|q| self.members(q).first().map(|&k| self.soldiers[k].stance));
        if let Some(s) = self.soldiers.last_mut() {
            match join {
                Some(q) => s.squad = q as u8,
                None => s.order = self.rally,
            }
            if let Some(st) = stance {
                s.stance = st;
            }
        }
        if let (true, Some(q)) = (gather, join) {
            let o = self.soldiers[self.members(q)[0]].order;
            let k = self.soldiers.len() - 1;
            self.soldiers[k].order = o;
        }
        // The next one's price, with this one alive (`Rules::soldier_step`).
        self.reprice();
        true
    }

    /// **The soldier's price now** (`Rules::soldier_step`, 2026-11-22):
    /// the base price, and `soldier_step` share more for each soldier alive
    /// over `soldier_from`. Kept in `Rules::soldier_cost`, so the shop
    /// button, the hands and the bots all read the price now. Called after
    /// a buy and once a step (soldiers fall). The base is read from
    /// `soldier_cost` on the first call and kept in `Game::price_base`, so
    /// anything that changes the price mid-game (a discount card, a rule
    /// set mid-planet, a test) must edit `price_base`, not `soldier_cost`,
    /// which the next call overwrites (Squad's review of #721).
    pub fn reprice(&mut self) {
        if self.price_base <= 0.0 {
            self.price_base = self.rules.soldier_cost;
        }
        let r = &self.rules;
        let over = (self.alive() as f32 - r.soldier_from).max(0.0);
        let k = if r.soldier_step > 0.0 { 1.0 + over / r.soldier_step } else { 1.0 };
        self.rules.soldier_cost = (self.price_base * k).round();
    }

    /// Within range of a live soldier.
    pub fn reachable(&self, at: (f32, f32)) -> bool {
        let r = self.rules.range * self.s();
        self.soldiers.iter().any(|s| s.alive && (s.x - at.0).hypot(s.y - at.1) <= r)
    }

    /// The price of a wall from `a` to `b` (fine cells).
    pub fn wall_price(&self, a: (f32, f32), b: (f32, f32)) -> f32 {
        ((a.0 - b.0).hypot(a.1 - b.1) / self.s()).max(1.0) * self.rules.wall_cost
    }

    /// A wall, if both ends are within a soldier's range and it is paid for:
    /// a building of `Kind::Wall` (`Game::build`).
    pub fn wall(&mut self, a: (f32, f32), b: (f32, f32)) -> bool {
        self.build(Kind::Wall { to: b }, a)
    }

    /// Write a wall's cells into the world, one band, no building: tests
    /// that need a bare wall. A building's cells go in by `wall_bands`
    /// (`doors`), every band of its level, since thick walls (11-16).
    #[cfg(test)]
    pub(super) fn wall_cells(&mut self, a: (f32, f32), b: (f32, f32)) {
        self.walls += 1;
        let s = self.s();
        self.world.build_wall((a.0 / s, a.1 / s), (b.0 / s, b.1 / s), self.rules.wall_width);
        self.stale_maps();
    }

    /// The crystal a point is inside (or nearest, within its radius plus 2
    /// dish cells): its index and centre.
    pub fn crystal_at(&self, p: (f32, f32)) -> Option<(usize, (f32, f32))> {
        let sc = self.s();
        self.world.crystals().iter().enumerate().map(|(i, c)| (i, c.0)).filter(|&(i, c)| (c.0 - p.0).hypot(c.1 - p.1) <= self.world.crystal_r(i) + 2.0 * sc).min_by(|a, b| {
            let d = |c: (f32, f32)| (c.0 - p.0).hypot(c.1 - p.1);
            d(a.1).total_cmp(&d(b.1))
        })
    }

    /// Send a builder to ghost `b` (`Cmd::Send`): the nearest idle soldier
    /// (holding, nothing queued), else the nearest live one not already
    /// building, whose order goes to the front of its queue so it returns
    /// to it (a full queue drops its last). The soldier sent, or None if
    /// `b` is not a ghost or nobody can go.
    ///
    /// Why both: the owner's playtest (2026-11-12 22:06Z) asked that a
    /// build ordered anywhere send a soldier there by itself. Idle
    /// soldiers first so a harvest isn't broken when one is free; a busy
    /// one second so the order is never silently dropped.
    pub fn send(&mut self, b: usize) -> Option<usize> {
        self.buildings.get(b).filter(|bd| bd.alive && bd.built < 1.0)?;
        let at = self.buildings[b].at;
        let d = |s: &Soldier| (s.x - at.0).hypot(s.y - at.1);
        let pick = |idle: bool| {
            (0..self.soldiers.len())
                .filter(|&k| {
                    let s = &self.soldiers[k];
                    s.alive && !matches!(s.order, Order::Build(_)) && (!idle || (s.order == Order::Hold && s.queued == 0))
                })
                .min_by(|&x, &y| d(&self.soldiers[x]).total_cmp(&d(&self.soldiers[y])))
        };
        let k = pick(true).or_else(|| pick(false))?;
        let s = &mut self.soldiers[k];
        if s.order != Order::Hold {
            let n = (s.queued as usize).min(QUEUE - 1);
            s.queue.copy_within(0..n, 1);
            s.queue[0] = s.order;
            s.queued = n as u8 + 1;
        }
        s.order = Order::Build(b);
        s.homing = false;
        Some(k)
    }

    /// Soldier `k` takes the next order in its queue, if it has one.
    pub(super) fn next(&mut self, k: usize) -> bool {
        let s = &mut self.soldiers[k];
        if s.queued == 0 {
            return false;
        }
        s.order = s.queue[0];
        s.queue.copy_within(1.., 0);
        s.queued -= 1;
        s.homing = false;
        true
    }

    /// The crystal a harvester from `skip` goes on to (`Rules::seek`), its
    /// centre, fine cells; None sends it home to hold. Only a safe one: at
    /// least a frame's harvest left, never the mould's home (crystal 0),
    /// none the mould has eaten from (as `share` counts it), and none
    /// nearer the core than the larder (the squad's side of the map). Of
    /// those, the nearest the larder that no other soldier is harvesting,
    /// else the nearest the larder. (The first version took the nearest to
    /// the soldier and only preferred uneaten crystals: the wall hand
    /// scored lower with it on 14 of 16 seeds, its farmers walking out past
    /// its wall.)
    pub fn seek(&self, k: usize, skip: Option<usize>) -> Option<(f32, f32)> {
        let r = self.rules;
        let ask = r.harvest * r.frame_ticks as f32 / r.tps as f32 * r.charge;
        let (l, core) = (self.larder, self.core);
        let d = |a: (f32, f32), b: (f32, f32)| (a.0 - b.0).hypot(a.1 - b.1);
        let busy: Vec<usize> = self
            .soldiers
            .iter()
            .enumerate()
            .filter(|&(q, s)| q != k && s.alive)
            .filter_map(|(_, s)| if let Order::Clear(x, y) = s.order { self.crystal_at((x, y)).map(|c| c.0) } else { None })
            .collect();
        self.world
            .crystals()
            .into_iter()
            .enumerate()
            .filter(|&(i, (c, charge))| i != 0 && Some(i) != skip && charge >= ask && d(c, l) < d(c, core))
            .filter(|&(i, (_, charge))| self.start_charge[i] - charge - self.taken[i] <= 0.01 * self.start_charge[i])
            .min_by(|a, b| busy.contains(&a.0).cmp(&busy.contains(&b.0)).then(d(a.1 .0, l).total_cmp(&d(b.1 .0, l))))
            .map(|(_, (c, _))| c)
    }

    /// Soldier `k` is done with a crystal that ran dry (`skip`): its next
    /// queued order, else (`Rules::seek`) the next crystal, else hold.
    pub(super) fn after_dry(&mut self, k: usize, skip: Option<usize>) {
        if self.next(k) {
            return;
        }
        let o = match self.rules.seek.then(|| self.seek(k, skip)).flatten() {
            Some(c) => Order::Clear(c.0, c.1),
            None => Order::Hold,
        };
        self.soldiers[k].order = o;
    }

    /// What soldier `k` guards, if anything: its own `Order::Guard`, or in
    /// the squad mode its squad's `Squad::guard`. The zone is a crystal
    /// there with its rim, a building there with its footprint, each and
    /// `GUARD_RIM` more, else `Rules::guard_r` round the spot: middle and
    /// radius, fine cells.
    pub fn guard_zone(&self, k: usize) -> Option<((f32, f32), f32)> {
        let s = self.soldiers[k];
        let p = match s.order {
            Order::Guard(x, y) => (x, y),
            _ if self.rules.squads => self.squads.get(s.squad as usize)?.guard?,
            _ => return None,
        };
        Some(self.zone_at(p))
    }

    /// The zone a Guard order at `p` keeps (`guard_zone`).
    pub fn zone_at(&self, p: (f32, f32)) -> ((f32, f32), f32) {
        let sc = self.s();
        if let Some((i, c)) = self.crystal_at(p) {
            return (c, self.world.crystal_r(i) + GUARD_RIM * sc);
        }
        if let Some(b) = self.building_at(p).filter(|&b| !matches!(self.buildings[b].kind, Kind::Wall { .. })) {
            return (self.buildings[b].at, (self.rules.build.touch + GUARD_RIM) * sc);
        }
        (p, self.rules.guard_r * sc)
    }

    /// The stance soldier `k` fires by on its own clock (not volleying).
    /// An attack-move fires freely; otherwise its own stance.
    ///
    /// The controlled squad keeps the fire mode the player gave it
    /// (2026-11-18). Before, its stance was read as defend whatever the
    /// player chose, since its guns were "the player's to aim", and its
    /// attack-move stood to fight (`Game::engaged`) at slime it would not
    /// shoot. The owner, 11-18 17:01Z: against loose walkers he lets go of
    /// the fire button and lets them shoot, "but the soldiers seem to
    /// shoot much slower or not all of them are shooting". His log that
    /// morning (planet 1): 23.9 shots and 143 kills a second with fire
    /// held, 4.5 and 11 on their own; his "free" at 71 s and 346 s did
    /// nothing.
    pub(super) fn stance_of(&self, k: usize) -> Stance {
        let s = self.soldiers[k];
        let commanded = self.rules.squads && Some(s.squad as usize) == self.active;
        if commanded && s.stance == Stance::HoldFire {
            Stance::HoldFire
        } else if matches!(s.order, Order::Attack | Order::Fight(..)) {
            Stance::Free
        } else {
            s.stance
        }
    }

    /// How far soldier `k`'s read of the slime looks for its aim
    /// (`squad::UnitIn::sight`, fine cells): its gun's reach, but only
    /// as far as it defends (`Rules::defend`) when that is all it shoots
    /// at (`Stance::Defend`, nothing to guard, no crystal to clear, not
    /// on the core).
    ///
    /// The engine's aim is the block with the most walkers in sight, and
    /// a defending soldier fires only when that block is near. With the
    /// whole reach read, a thicker clump 20 cells off hid the loose
    /// walkers on top of the squad: the soldiers held fire as they were
    /// eaten, or, touched, fired past them at the clump (2026-11-18). Read
    /// short, each soldier aims at the thickest slime near itself, so a
    /// squad among loose walkers spreads its fire over them.
    ///
    /// The rest (free fire, guarding, clearing a crystal) read near
    /// themselves first (`Rules::near_first`) and the whole reach only
    /// after a near read found no slime, for `FAR_FRAMES` frames
    /// (`Game::read_aim`). Read whole every frame, a free squad fired at
    /// the thickest clump in its 24-cell reach, often off the screen at
    /// zoom 12, past walkers standing beside it (filmed 11-22).
    pub(super) fn sight(&self, k: usize) -> f32 {
        self.sight_read(k).0
    }

    /// `sight`, and whether it is the near read that `read_aim` checks.
    fn sight_read(&self, k: usize) -> (f32, bool) {
        let s = self.soldiers[k];
        let sc = self.s();
        let full = self.gun_range(s.squad as usize) * sc;
        // On the core the read decides whether it is clear (`check_core`,
        // walkers within twice its radius), so it looks the whole way.
        let on_core = (s.x - self.core.0).hypot(s.y - self.core.1) <= 3.0 * self.rules.core_r * sc;
        if on_core {
            return (full, false);
        }
        if self.stance_of(k) != Stance::Defend || self.guard_zone(k).is_some() || matches!(s.order, Order::Clear(..)) {
            let near = self.rules.near_first * sc;
            return if s.far == 0 && near > 0.0 && near < full { (near, true) } else { (full, false) };
        }
        ((self.rules.defend * sc).min(full), false)
    }

    /// Takes back soldier `k`'s read of the slime this frame (the
    /// engine's `UnitOut`): walkers on it, its aim and the walkers there.
    /// A near read (`sight_read`) that found no slime to fire at sends it
    /// to read the whole reach for the next `FAR_FRAMES` frames; then it
    /// looks near again, so slime that comes close is its aim within
    /// about a quarter second. The empty frame aims at nothing, which
    /// holds a shot at far slime back by at most one frame.
    pub(super) fn read_aim(&mut self, k: usize, touching: u32, aim: Option<(f32, f32)>, dense: u32) {
        let (_, probe) = self.sight_read(k);
        let none = aim.is_none() || dense < self.rules.aim_min;
        let s = &mut self.soldiers[k];
        s.touching = touching;
        s.aim = aim;
        s.dense = dense;
        s.far = if probe && none { FAR_FRAMES } else { s.far.saturating_sub(1) };
    }

    /// Where soldier `k` fires this frame, if anywhere (fine cells).
    pub(super) fn target(&mut self, k: usize) -> Option<(f32, f32)> {
        let s = self.soldiers[k];
        let sc = self.s();
        if let Order::Burn(a, b) = s.order {
            // Along the line in turn, five points.
            let t = (s.beat % 5) as f32 / 4.0;
            self.soldiers[k].beat += 1;
            let p = (a.0 + (b.0 - a.0) * t, a.1 + (b.1 - a.1) * t);
            // A point in reach (the shot's own test in `begin_step`) is
            // fired at. One out of reach, while the soldier still walks to
            // the line, falls through to its own aim and stance, so slime
            // that reaches it on the way is shot (2026-11-14). Before, a
            // burn point out of reach fired nothing: on shape C the
            // `volley` squad, wading 50-60 cells short of its line with
            // up to 17 walkers on each soldier, fired no shot after its
            // burn order and died in 2 s (`Reports/larder-crater-2s-
            // 2026-11-14.md`). The owner, 11-13 16:15Z: a soldier "standing
            // still and gets rushed by walkers" must be able to kill them.
            if (p.0 - s.x).hypot(p.1 - s.y) <= self.gun_range(s.squad as usize) * sc + 4.0 {
                return Some(p);
            }
        }
        // A mini-boss in reach and in sight first (`Game::boss_aim`,
        // 2026-11-16); its body's radius widens the stance's tests below.
        //
        // Beyond `Rules::near_first`, only when the soldier's own aim holds
        // no slime that near (Direction, 11-22, after #708): a boss up to
        // the gun's whole reach, 24 cells, took every shot past walkers
        // beside the squad, off the screen at zoom 12, the fault #708 fixed
        // for the soldier's own read. A near boss still comes first, and a
        // touched soldier still only takes one within its defend reach
        // (`boss_aim`). The origin is to come through here too (#699).
        let near = self.rules.near_first * sc;
        let slime_near = near > 0.0
            && s.dense >= self.rules.aim_min
            && s.aim.is_some_and(|a| (a.0 - s.x).hypot(a.1 - s.y) <= near + DEFEND_SLACK && self.clear_line((s.x, s.y), a));
        let boss = if slime_near { self.boss_aim_within(k, near) } else { self.boss_aim(k) };
        let (aim, body) = match boss {
            Some(b) => b,
            None => {
                let aim = s.aim?;
                // Only at mould that shows as slime, and that the soldier
                // can see (no rock or wall on the line: a shot into rock is
                // paid for and hits nothing).
                if s.dense < self.rules.aim_min || !self.clear_line((s.x, s.y), aim) {
                    return None;
                }
                (aim, 0.0)
            }
        };
        let near = |p: (f32, f32), r: f32| (p.0 - aim.0).hypot(p.1 - aim.1) <= r + body;
        match self.stance_of(k) {
            Stance::HoldFire => None,
            Stance::Free => Some(aim),
            Stance::Defend => {
                // Presence, any walker at any mass: one on a soldier is one
                // to shoot. The aim is a block's centre, so a walker just
                // inside the radius can read up to about half a block past
                // it (`DEFEND_SLACK`).
                let mut ok = s.touching > 0 || near((s.x, s.y), self.rules.defend * sc + DEFEND_SLACK);
                // Guarding (`Order::Guard`, or its squad's `Squad::guard`):
                // slime on what it guards too.
                if let Some((p, r)) = self.guard_zone(k) {
                    ok |= near(p, r);
                }
                if let Order::Clear(x, y) = s.order {
                    if let Some((i, c)) = self.crystal_at((x, y)) {
                        ok |= near(c, self.world.crystal_r(i) + 2.0 * sc);
                    }
                }
                ok.then_some(aim)
            }
        }
    }
}
