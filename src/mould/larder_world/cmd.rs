//! The player's commands (`Cmd`) and how the game applies them. Moved out
//! of `larder_world.rs` unchanged (C9, 2026-11-14).

use super::*;

/// A player's command; `Game::apply` keeps the log. As M3's, with walls and
/// buildings (`buildings`) in place of M3's turret and bait.
/// `Order::Clear` names a crystal by a point inside it.
#[derive(Clone, Copy, Debug, PartialEq)]
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
    Formation(u8, Formation),
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
            Cmd::Formation(q, f) => format!("formation {q} {}", f.name()),
        }
    }

    pub fn parse(line: &str) -> Option<Cmd> {
        let w: Vec<&str> = line.split_whitespace().collect();
        let f = |i: usize| w.get(i).and_then(|v| v.parse::<f32>().ok());
        match *w.first()? {
            "wall" => return Some(Cmd::Wall((f(1)?, f(2)?), (f(3)?, f(4)?))),
            "toggle" => return Some(Cmd::Toggle(w.get(1)?.parse().ok()?)),
            "send" => return Some(Cmd::Send(w.get(1)?.parse().ok()?)),
            "steer" => return Some(Cmd::Steer(w.get(1)?.parse().ok()?, (f(2)?, f(3)?))),
            "fire" if w.get(2) == Some(&"none") => return Some(Cmd::Fire(w.get(1)?.parse().ok()?, None)),
            "fire" => return Some(Cmd::Fire(w.get(1)?.parse().ok()?, Some((f(2)?, f(3)?)))),
            "face" => return Some(Cmd::Face(w.get(1)?.parse().ok()?, (f(2)?, f(3)?))),
            "split" => return Some(Cmd::Split(w.get(1)?.parse().ok()?, (f(2)?, f(3)?))),
            "join" => return Some(Cmd::Join(w.get(1)?.parse().ok()?, w.get(2)?.parse().ok()?)),
            "merge" => return Some(Cmd::Merge(w.get(1)?.parse().ok()?, w.get(2)?.parse().ok()?)),
            "active" => return Some(Cmd::Active(w.get(1)?.parse().ok()?)),
            "formation" => return Some(Cmd::Formation(w.get(1)?.parse().ok()?, Formation::parse(w.get(2)?)?)),
            "sorder" => {
                let q: u8 = w.get(1)?.parse().ok()?;
                let rest = w[2..].join(" ");
                return match super::super::larder::Cmd::parse(&format!("order 0 {rest}"))? {
                    super::super::larder::Cmd::Order(_, o) => Some(Cmd::SquadOrder(q, o)),
                    _ => None,
                };
            }
            "build" => {
                let kind = match *w.get(1)? {
                    "turret" => Kind::Turret,
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
            Cmd::Build(kind, at) => self.build(kind, at),
            Cmd::Toggle(k) => self.toggle(k),
            Cmd::Send(b) => self.send(b).is_some(),
            Cmd::Rally(o) => {
                self.rally = o;
                true
            }
            Cmd::Steer(..) | Cmd::SquadOrder(..) | Cmd::Fire(..) | Cmd::Face(..) | Cmd::Split(..) | Cmd::Join(..) | Cmd::Merge(..) | Cmd::Active(..) | Cmd::Formation(..) => self.squad_cmd(c),
            _ => false,
        }
    }

    pub fn buy(&mut self) -> bool {
        if self.bank < self.rules.soldier_cost || self.alive() >= self.rules.max_squad {
            return false;
        }
        self.bank -= self.rules.soldier_cost;
        self.spent += self.rules.soldier_cost;
        let a = self.rng.f32() * std::f32::consts::TAU;
        // A bought soldier joins the controlled squad, at its side (owner,
        // 2026-11-14 04:18Z); with none controlled it starts at the larder
        // on the rally order, as before.
        let join = self.active.filter(|&q| self.rules.squads && !self.members(q).is_empty());
        let at = join.and_then(|q| self.middle(q));
        self.add_soldier_at(at.unwrap_or(self.larder), a);
        let gather = join.is_some_and(|q| self.squads[q].steer == (0.0, 0.0) && self.squads[q].dest.is_none());
        if let Some(s) = self.soldiers.last_mut() {
            match join {
                Some(q) => s.squad = q as u8,
                None => s.order = self.rally,
            }
        }
        if let (true, Some(q)) = (gather, join) {
            let o = self.soldiers[self.members(q)[0]].order;
            let k = self.soldiers.len() - 1;
            self.soldiers[k].order = o;
        }
        true
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

    /// Write a wall's cells into the world (once it is whole).
    pub(super) fn wall_cells(&mut self, a: (f32, f32), b: (f32, f32)) {
        self.walls += 1;
        let s = self.s();
        self.world.build_wall((a.0 / s, a.1 / s), (b.0 / s, b.1 / s), self.rules.wall_width);
        self.paths.maps.clear();
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
            if (p.0 - s.x).hypot(p.1 - s.y) <= self.gun_range() * sc + 4.0 {
                return Some(p);
            }
        }
        let aim = s.aim?;
        // Only at mould that shows as slime, and that the soldier can see
        // (no rock or wall on the line: a shot into rock is paid for and
        // hits nothing).
        if s.dense < self.rules.aim_min || !self.clear_line((s.x, s.y), aim) {
            return None;
        }
        let near = |p: (f32, f32), r: f32| (p.0 - aim.0).hypot(p.1 - aim.1) <= r;
        let stance = if self.rules.squads && Some(s.squad as usize) == self.active {
            // The controlled squad's guns are the player's (`squads`): on
            // its own it only defends itself.
            if s.stance == Stance::HoldFire { Stance::HoldFire } else { Stance::Defend }
        } else if matches!(s.order, Order::Attack | Order::Fight(..)) {
            Stance::Free
        } else {
            s.stance
        };
        match stance {
            Stance::HoldFire => None,
            Stance::Free => Some(aim),
            Stance::Defend => {
                // Presence, any walker at any mass: one on a soldier is one to shoot.
                let mut ok = s.touching > 0 || near((s.x, s.y), self.rules.defend * sc);
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
