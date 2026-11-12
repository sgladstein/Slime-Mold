//! **Larder: a squad against the mould** (owner, 2026-11-11 00:18Z: "a
//! demo where the slime mold is the enemy"). Milestone M1 of the build
//! plan (`Reports/larder-build-plan-2026-11-11.md`): soldiers, their
//! orders, firing, the mould attacking them, and the round's rules, on
//! today's CPU dish. No window here; `examples/larderprobe.rs` (`m1`)
//! drives it with scripted plans and the window comes in M1b.
//!
//! **Game time.** One game second is `GAME_SECOND` dish steps (15, an
//! eighth of the dish's 120), so the mould crosses the field in about 30
//! game seconds instead of 4 and a front can be seen coming and shot
//! (priced-ring test, 2026-11-11: first bite at 31.6 s). Every rule here
//! counts in game seconds and converts once, through `secs`. The engineering
//! review (2026-11-11, finding 2) found the earlier harness slowed only its
//! own clock: the dish's own timers (`Params::wound` among them) still ran
//! in dish seconds. Shots here set their own heal step, so the one dish
//! timer Larder leans on is in game time; the others (oats, wetting,
//! crusts, walls) are not used on this dish.
//!
//! **The owner's rulings** (`wiki/decisions.md`): RTS-style orders
//! (03:14Z); the squad starts at 4 and grows to 40, soldiers die and are
//! attacked by the mould, and the round ends when the soldiers or the mould
//! are all dead (03:38Z); food stops late and a 15-minute cap ends a
//! stalemate on share of the map held (04:04Z).
//!
//! **How the mould attacks.** Soldiers smell like food: each lays scent
//! like a small pile every step (`Rules::smell`), so the mould grows toward
//! them. Each walker within `Rules::touch` of a soldier drains its health.
//! A dead soldier becomes a food pile (`Rules::body`): losses feed the
//! enemy. Nothing here is drawn-only; a soldier is a game piece.
//!
//! **The mould's core.** Its home pile. A soldier standing within
//! `Rules::core_r` of it with no walker within `2 * core_r` for
//! `Rules::core_secs` game seconds takes the core, and the mould stops
//! dividing (`Params::divide` set to 0), so a wipe can be reached: without
//! it, any piece left on any of the 23 piles regrows (design review,
//! 2026-11-11, finding 3).

use super::{disc, Dish, Ground, Pile, Rng, H, W};

/// Dish steps per game second.
pub const GAME_SECOND: u64 = 15;

/// Dish steps in `s` game seconds.
pub fn secs(s: f32) -> u64 {
    (s * GAME_SECOND as f32).round() as u64
}

/// Walker-count bins, in cells: the densest-mould search reads these.
const BIN: usize = 4;
const BW: usize = W.div_ceil(BIN);
const BH: usize = H.div_ceil(BIN);

/// Every number in the game layer, starting values for the census.
#[derive(Clone, Copy, Debug)]
pub struct Rules {
    /// Cells a game second. The design review (finding 4) asked for 1.5-2x
    /// a front's speed so a soldier can retreat; fronts measured in the M1
    /// census (`Census::front_speed`).
    pub speed: f32,
    /// How far a soldier fires, cells. 15, not the plan's first 30: at 30,
    /// 40 soldiers' circles cover the field 2.7 times (design, finding 5).
    pub range: f32,
    /// Length of one shot's line, cells, and its half-width.
    pub line: f32,
    pub width: f32,
    /// Game seconds between one soldier's shots.
    pub shot_every: f32,
    /// Food a shot costs.
    pub price: f32,
    /// Game seconds a shot's wound stays open.
    pub wound: f32,
    pub hp: f32,
    /// Health a walker touching a soldier takes in a game second.
    pub drain: f32,
    /// Walkers within this many cells touch a soldier.
    pub touch: f32,
    /// A soldier's scent, as a fraction of a full pile's.
    pub smell: f32,
    /// Food a dead soldier leaves.
    pub body: f32,
    /// Food a soldier takes from a clear pile in a game second, and how much
    /// it can carry before walking it home.
    pub harvest: f32,
    pub carry: f32,
    pub core_r: f32,
    pub core_secs: f32,
    /// The cap, game seconds (owner, 04:04Z: 15 minutes).
    pub cap: f32,
    pub start_bank: f32,
    pub squad: usize,
    // ------------------------------------------------ M2: spending
    /// Food a new soldier costs, and the most there can be (owner, 03:38Z:
    /// "start with 4 and can grow to 40").
    pub soldier_cost: f32,
    pub max_squad: usize,
    /// A turret: a fixed gun on free fire, paid per shot from the bank like
    /// a soldier's; the mould wears it down like a soldier. No smell.
    pub turret_cost: f32,
    pub turret_hp: f32,
    pub turret_range: f32,
    // ------------------------------------------- M2: the mould's answers
    /// **Hoard smell**: the larder smells of the bank, `hoard` times a full
    /// pile per 10,000 food banked. 0 is off.
    pub hoard: f32,
    /// **Hunger**: a mould that has eaten little in the last
    /// `HUNGER_WINDOW` game seconds drains soldiers and smells them up to
    /// `1 + hunger` times as hard. A fed mould is slow to anger; a starved
    /// one hunts the squad. 0 is off.
    pub hunger: f32,
    /// **Drops**: a pile of `drop_food` lands every `drop_every` game
    /// seconds at a random open spot, smelling `drop_warn` seconds before it
    /// lands, and none after `drops_stop` (owner, 04:04Z: food stops late
    /// in the round). `drop_every` 0 is off.
    pub drop_every: f32,
    pub drop_food: f32,
    pub drop_warn: f32,
    pub drops_stop: f32,
}

/// Game seconds over which the mould's eating is counted for `hunger`.
pub const HUNGER_WINDOW: usize = 30;

/// Food a walker eats in a game second when the mould is fed (`Game::hunger`).
pub const FED_RATE: f32 = 0.04;

/// Soldier `k`'s place in a loose formation round where it was sent: a
/// sunflower spiral 1.5 cells apart, so a squad sent to one point spreads
/// instead of stacking on one cell (M2b: 17 soldiers on one cell drew the
/// mould in on 17 smells at once).
fn slot(k: usize) -> (f32, f32) {
    if k == 0 {
        return (0.0, 0.0);
    }
    let r = 1.5 * (k as f32).sqrt();
    let a = k as f32 * 2.399_963;
    (r * a.cos(), r * a.sin())
}

/// A fixed gun the squad builds.
#[derive(Clone, Copy, Debug)]
pub struct Turret {
    pub x: f32,
    pub y: f32,
    pub hp: f32,
    ready: u64,
}

impl Default for Rules {
    fn default() -> Self {
        Rules {
            speed: 10.0,
            range: 15.0,
            line: 6.0,
            width: 2.5,
            shot_every: 1.0,
            price: 10.0,
            wound: 4.0,
            hp: 100.0,
            drain: 1.0,
            touch: 1.5,
            smell: 0.5,
            body: 400.0,
            harvest: 30.0,
            carry: 300.0,
            core_r: 5.0,
            core_secs: 5.0,
            cap: 900.0,
            start_bank: 2000.0,
            squad: 4,
            soldier_cost: 400.0,
            max_squad: 40,
            turret_cost: 600.0,
            turret_hp: 300.0,
            turret_range: 12.0,
            hoard: 1.0,
            hunger: 1.0,
            drop_every: 60.0,
            drop_food: 1500.0,
            drop_warn: 10.0,
            drops_stop: 600.0,
        }
    }
}

/// What a soldier fires at by itself.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Stance {
    HoldFire,
    /// Only mould near the soldier, or on the pile it is clearing: the
    /// default (design and method reviews, 2026-11-11: free fire by
    /// default spends the bank to nothing).
    Defend,
    /// The densest mould in range.
    Free,
}

#[derive(Clone, Copy, Debug, PartialEq)]
pub enum Order {
    Hold,
    Move(f32, f32),
    /// Clear the pile at this point, keep it clear, and carry its food to
    /// the larder.
    Clear(f32, f32),
    /// Keep this segment burning: stand within range of it and fire along
    /// it in turn.
    Burn((f32, f32), (f32, f32)),
    /// Walk on the mould's core, firing freely.
    Attack,
}

/// **A player's command**, as the window gives it. Every command goes
/// through `Game::apply`, which keeps the log; a log replayed on the same
/// seed and rules reaches the same dish hash (build plan M3: "replay to the
/// same hash").
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum Cmd {
    Order(usize, Order),
    Stance(usize, Stance),
    Buy,
    Turret((f32, f32)),
    Bait((f32, f32), f32),
}

impl Cmd {
    /// One line of a replay file, after the step: `order 3 move 120.5 40`.
    pub fn to_line(self) -> String {
        match self {
            Cmd::Order(k, o) => match o {
                Order::Hold => format!("order {k} hold"),
                Order::Move(x, y) => format!("order {k} move {x} {y}"),
                Order::Clear(x, y) => format!("order {k} clear {x} {y}"),
                Order::Burn(a, b) => format!("order {k} burn {} {} {} {}", a.0, a.1, b.0, b.1),
                Order::Attack => format!("order {k} attack"),
            },
            Cmd::Stance(k, st) => format!(
                "stance {k} {}",
                match st {
                    Stance::HoldFire => "hold",
                    Stance::Defend => "defend",
                    Stance::Free => "free",
                }
            ),
            Cmd::Buy => "buy".into(),
            Cmd::Turret(at) => format!("turret {} {}", at.0, at.1),
            Cmd::Bait(at, f) => format!("bait {} {} {f}", at.0, at.1),
        }
    }

    /// The inverse of `to_line`. Floats print in Rust's shortest exact
    /// form, so a line reads back to the same bits.
    pub fn parse(line: &str) -> Option<Cmd> {
        let w: Vec<&str> = line.split_whitespace().collect();
        let f = |i: usize| w.get(i).and_then(|v| v.parse::<f32>().ok());
        let k = || w.get(1).and_then(|v| v.parse::<usize>().ok());
        Some(match *w.first()? {
            "order" => Cmd::Order(
                k()?,
                match *w.get(2)? {
                    "hold" => Order::Hold,
                    "move" => Order::Move(f(3)?, f(4)?),
                    "clear" => Order::Clear(f(3)?, f(4)?),
                    "burn" => Order::Burn((f(3)?, f(4)?), (f(5)?, f(6)?)),
                    "attack" => Order::Attack,
                    _ => return None,
                },
            ),
            "stance" => Cmd::Stance(
                k()?,
                match *w.get(2)? {
                    "hold" => Stance::HoldFire,
                    "defend" => Stance::Defend,
                    "free" => Stance::Free,
                    _ => return None,
                },
            ),
            "buy" => Cmd::Buy,
            "turret" => Cmd::Turret((f(1)?, f(2)?)),
            "bait" => Cmd::Bait((f(1)?, f(2)?), f(3)?),
            _ => return None,
        })
    }
}

#[derive(Clone, Copy, Debug)]
pub struct Soldier {
    pub x: f32,
    pub y: f32,
    pub hp: f32,
    pub order: Order,
    pub stance: Stance,
    /// Food carried, and whether it is walking it home.
    pub carry: f32,
    pub homing: bool,
    pub alive: bool,
    /// Step of its next shot.
    ready: u64,
    /// Which piece of a burn line it fires next.
    beat: u32,
}

#[derive(Clone, Copy, Debug, PartialEq)]
pub enum End {
    /// Every soldier dead, at game second `t`.
    SquadWiped(f32),
    /// No walker left, at game second `t`.
    MouldWiped(f32),
    /// The cap, with the squad's share of the held piles.
    Cap(f32),
    /// The mould held the larder with the bank empty for
    /// `larder_world::Rules::overrun` seconds, at game second `t` (Larder
    /// World only, a campaign mission's switch; 2026-11-12).
    Overrun(f32),
}

impl End {
    /// **One ordered number a round** (method review, 2026-11-11, finding
    /// 4): a win beats a stalemate beats a loss, and inside each band
    /// sooner and more soldiers left is better for the squad. Squad wins
    /// are in (1, 3], stalemates in [-0.5, 0.5], losses in [-2, -1).
    pub fn score(self, left: usize, cap: f32) -> f32 {
        match self {
            End::MouldWiped(t) => 1.0 + left as f32 / 40.0 + (cap - t) / cap,
            End::Cap(share) => share - 0.5,
            End::SquadWiped(t) | End::Overrun(t) => -(1.0 + (cap - t) / cap),
        }
    }

    pub fn band(self) -> &'static str {
        match self {
            End::MouldWiped(_) => "mould wiped",
            End::Cap(_) => "cap",
            End::SquadWiped(_) => "squad wiped",
            End::Overrun(_) => "larder overrun",
        }
    }
}

pub struct Game {
    pub dish: Dish,
    pub soldiers: Vec<Soldier>,
    pub rules: Rules,
    pub bank: f32,
    pub spent: f32,
    pub harvested: f32,
    pub shots: u32,
    pub killed: u32,
    pub larder: (f32, f32),
    /// The mould's home pile: its core.
    pub core: (f32, f32),
    pub core_down: Option<u64>,
    core_clear: u64,
    /// Game second a soldier was first touched, if one was.
    pub first_touch: Option<f32>,
    pub end: Option<End>,
    pub turrets: Vec<Turret>,
    pub bought: u32,
    pub baited: f32,
    /// Food the mould ate in each of the last game seconds, newest last.
    pub eaten: Vec<f32>,
    /// 0 fed to 1 starving (`Rules::hunger`).
    pub hunger: f32,
    /// The next drop: when (step) and where.
    pub next_drop: Option<(u64, (f32, f32))>,
    food_then: f32,
    /// Own RNG, apart from the dish's, so a soldier rule never shifts the
    /// mould's dice (engineering review, finding 11).
    pub rng: Rng,
    bins: Vec<u16>,
    /// The player's commands, each at the step it was given (`Cmd`).
    pub log: Vec<(u64, Cmd)>,
}

/// **The M1 dish**: the 23-pile field (`skirmish::FIELD`) with one mould
/// on the left home and the squad on the right home, the larder. No rival,
/// no level bell, no storm. The harness and the window both start here.
pub fn field_game(seed: u64, rules: Rules) -> Game {
    use super::skirmish::{FIELD, HOMES};
    let mut d = Dish::from_level_with(&FIELD, seed, FIELD.params_for());
    d.rival = None;
    d.level = None;
    d.storm_at = u64::MAX;
    Game::new(d, HOMES[1], HOMES[0], rules, seed)
}

impl Game {
    /// A squad at the larder, a mould on its home pile. `dish` comes ready
    /// (the harness makes it from the FIELD level with the rival removed).
    pub fn new(dish: Dish, larder: (f32, f32), core: (f32, f32), rules: Rules, seed: u64) -> Game {
        let soldiers = (0..rules.squad)
            .map(|k| {
                let a = k as f32 / rules.squad.max(1) as f32 * std::f32::consts::TAU;
                Soldier {
                    x: larder.0 + 3.0 * a.cos(),
                    y: larder.1 + 3.0 * a.sin(),
                    hp: rules.hp,
                    order: Order::Hold,
                    stance: Stance::Defend,
                    carry: 0.0,
                    homing: false,
                    alive: true,
                    ready: 0,
                    beat: 0,
                }
            })
            .collect();
        Game {
            dish,
            soldiers,
            rules,
            bank: rules.start_bank,
            spent: 0.0,
            harvested: 0.0,
            shots: 0,
            killed: 0,
            larder,
            core,
            core_down: None,
            core_clear: 0,
            first_touch: None,
            end: None,
            rng: Rng::new(seed ^ 0x001a_2de2_501d),
            bins: vec![0; BW * BH],
            turrets: Vec::new(),
            bought: 0,
            baited: 0.0,
            eaten: Vec::new(),
            hunger: 0.0,
            next_drop: None,
            food_then: 0.0,
            log: Vec::new(),
        }
    }

    /// Game seconds since the start.
    pub fn time(&self) -> f32 {
        self.dish.t as f32 / GAME_SECOND as f32
    }

    pub fn alive(&self) -> usize {
        self.soldiers.iter().filter(|s| s.alive).count()
    }

    /// Carry out a player's command and log it at this step (`Cmd`).
    /// True if it took effect (a buy, turret or bait can fail).
    pub fn apply(&mut self, c: Cmd) -> bool {
        self.log.push((self.dish.t, c));
        match c {
            Cmd::Order(k, o) => {
                self.order(k, o);
                true
            }
            Cmd::Stance(k, st) => {
                self.stance(k, st);
                true
            }
            Cmd::Buy => self.buy(),
            Cmd::Turret(at) => self.build_turret(at),
            Cmd::Bait(at, food) => self.bait(at, food),
        }
    }

    /// The log as a replay file: a header line with the seed, then one
    /// command a line after its step.
    pub fn replay_text(&self, seed: u64) -> String {
        let mut out = format!("larder replay seed {seed} end {}\n", self.dish.t);
        for (t, c) in &self.log {
            out += &format!("{t} {}\n", c.to_line());
        }
        out
    }

    /// Play a replay file's commands on a fresh game: the same seed and
    /// rules give the same dish (`same_commands_same_hash`). Returns the
    /// game at the recorded end step.
    pub fn replay(text: &str, rules: Rules) -> Option<Game> {
        let mut lines = text.lines();
        let head: Vec<&str> = lines.next()?.split_whitespace().collect();
        let seed: u64 = head.get(3)?.parse().ok()?;
        let end: u64 = head.get(5)?.parse().ok()?;
        let mut cmds = Vec::new();
        for l in lines {
            let (t, rest) = l.split_once(' ')?;
            cmds.push((t.parse::<u64>().ok()?, Cmd::parse(rest)?));
        }
        let mut g = field_game(seed, rules);
        let mut next = 0;
        while g.dish.t < end && g.end.is_none() {
            while next < cmds.len() && cmds[next].0 == g.dish.t {
                g.apply(cmds[next].1);
                next += 1;
            }
            g.step();
        }
        while next < cmds.len() && cmds[next].0 == g.dish.t {
            g.apply(cmds[next].1);
            next += 1;
        }
        Some(g)
    }

    pub fn order(&mut self, k: usize, order: Order) {
        self.soldiers[k].order = order;
        self.soldiers[k].homing = false;
    }

    pub fn stance(&mut self, k: usize, stance: Stance) {
        self.soldiers[k].stance = stance;
    }

    /// The pile at a point, if one is there (piles never move; bodies and
    /// drops are pushed at the end, so an index is not an identity:
    /// engineering review, finding 7).
    pub fn pile_at(&self, p: (f32, f32)) -> Option<usize> {
        self.dish
            .piles
            .iter()
            .position(|q| (q.x - p.0).abs() < 0.5 && (q.y - p.1).abs() < 0.5)
    }

    /// One shot by soldier `k` across the point `at`, if it is in range,
    /// loaded and paid for. True if fired.
    pub fn shoot(&mut self, k: usize, at: (f32, f32)) -> bool {
        let s = self.soldiers[k];
        let t = self.dish.t;
        if !s.alive || s.ready > t || self.bank < self.rules.price {
            return false;
        }
        let (dx, dy) = (at.0 - s.x, at.1 - s.y);
        let d = dx.hypot(dy);
        if d > self.rules.range {
            return false;
        }
        // Across the line of fire: a shot cuts what runs past the shooter.
        let (ux, uy) = if d > 0.1 { (-dy / d, dx / d) } else { (1.0, 0.0) };
        let h = self.rules.line / 2.0;
        self.fire_line((at.0 - ux * h, at.1 - uy * h), (at.0 + ux * h, at.1 + uy * h));
        self.soldiers[k].ready = t + secs(self.rules.shot_every);
        // The bins are counted once a step; empty the one just shot so the
        // next soldier aims elsewhere (M2b: stacked squads all fired on one
        // bin, already emptied, while the mould closed in round them).
        let (bx, by) = ((at.0.max(0.0) as usize / BIN).min(BW - 1), (at.1.max(0.0) as usize / BIN).min(BH - 1));
        self.bins[by * BW + bx] = 0;
        true
    }

    fn fire_line(&mut self, a: (f32, f32), b: (f32, f32)) {
        let heal = self.dish.t + secs(self.rules.wound);
        self.killed += self.dish.fire(a, b, self.rules.width, heal);
        self.bank -= self.rules.price;
        self.spent += self.rules.price;
        self.shots += 1;
    }

    fn bin_walkers(&mut self) {
        self.bins.iter_mut().for_each(|b| *b = 0);
        for w in &self.dish.walkers {
            let (bx, by) = ((w.x as usize / BIN).min(BW - 1), (w.y as usize / BIN).min(BH - 1));
            self.bins[by * BW + bx] = self.bins[by * BW + bx].saturating_add(1);
        }
    }

    /// The centre of the densest bin within `r` of `c`, and its count.
    fn densest(&self, c: (f32, f32), r: f32) -> Option<(f32, f32)> {
        let lo = |v: f32| ((v - r).max(0.0) as usize) / BIN;
        let (bx0, by0) = (lo(c.0), lo(c.1));
        let (bx1, by1) = (
            (((c.0 + r) as usize) / BIN).min(BW - 1),
            (((c.1 + r) as usize) / BIN).min(BH - 1),
        );
        let mut best: Option<(u16, (f32, f32))> = None;
        for by in by0..=by1 {
            for bx in bx0..=bx1 {
                let n = self.bins[by * BW + bx];
                if n == 0 {
                    continue;
                }
                let p = (
                    (bx * BIN) as f32 + BIN as f32 / 2.0,
                    (by * BIN) as f32 + BIN as f32 / 2.0,
                );
                if (p.0 - c.0).hypot(p.1 - c.1) > r + BIN as f32 * 0.71 {
                    continue;
                }
                if best.is_none_or(|b| n > b.0) {
                    best = Some((n, p));
                }
            }
        }
        best.map(|b| b.1)
    }

    fn walkers_near(&self, c: (f32, f32), r: f32) -> usize {
        let r2 = r * r;
        self.dish
            .walkers
            .iter()
            .filter(|w| (w.x - c.0).powi(2) + (w.y - c.1).powi(2) <= r2)
            .count()
    }

    /// One dish step with the squad's turn before it. Does nothing once the
    /// round has ended.
    pub fn step(&mut self) {
        if self.end.is_some() {
            return;
        }
        self.bin_walkers();
        let per_step = 1.0 / GAME_SECOND as f32;
        for k in 0..self.soldiers.len() {
            if !self.soldiers[k].alive {
                continue;
            }
            self.soldier_turn(k);
            // The mould's bite.
            let s = self.soldiers[k];
            let n = self.walkers_near((s.x, s.y), self.rules.touch);
            if n > 0 {
                if self.first_touch.is_none() {
                    self.first_touch = Some(self.time());
                }
                let anger = 1.0 + self.rules.hunger * self.hunger;
                self.soldiers[k].hp -= self.rules.drain * anger * n as f32 * per_step;
                if self.soldiers[k].hp <= 0.0 {
                    self.die(k);
                }
            }
        }
        self.turrets_turn();
        self.smell();
        self.hoard_smell();
        self.drops();
        self.check_core();
        let food_before = self.pile_food();
        self.dish.step();
        // What the mould ate this step: pile food that went, less what the
        // squad took (counted where it takes it) and plus what landed.
        self.food_then += (food_before - self.pile_food()).max(0.0);
        if self.dish.t.is_multiple_of(GAME_SECOND) {
            self.eaten.push(self.food_then);
            self.food_then = 0.0;
            if self.eaten.len() > HUNGER_WINDOW {
                self.eaten.remove(0);
            }
            // Fed: eating `FED_RATE` food a walker a game second or more.
            // At 0.1 a fully fed mould on the full field read 0.37 hungry
            // (M2 first look, seeds 1-4, idle): it ate about 0.06.
            let ate: f32 = self.eaten.iter().sum::<f32>() / self.eaten.len() as f32;
            let need = FED_RATE * self.dish.walkers.len().max(1) as f32;
            self.hunger = (1.0 - ate / need).clamp(0.0, 1.0);
        }
        self.check_end();
    }

    fn die(&mut self, k: usize) {
        let s = &mut self.soldiers[k];
        s.alive = false;
        s.hp = 0.0;
        let (x, y) = (s.x, s.y);
        let food = self.rules.body + s.carry;
        s.carry = 0.0;
        self.dish.piles.push(Pile {
            x,
            y,
            r: 2.0,
            food,
            full: food,
            oats: false,
            expires: u64::MAX,
            eaters: 0,
            foe_eaters: 0,
            held: 0,
        });
    }

    fn smell(&mut self) {
        let strength = self.dish.params.food_deposit * self.rules.smell * (1.0 + self.rules.hunger * self.hunger);
        if strength <= 0.0 {
            return;
        }
        for s in self.soldiers.iter().filter(|s| s.alive) {
            for (ox, oy) in disc(2.0) {
                let (x, y) = (s.x + ox, s.y + oy);
                if x < 0.0 || y < 0.0 || x as usize >= W || y as usize >= H {
                    continue;
                }
                let i = y as usize * W + x as usize;
                if self.dish.ground[i] == Ground::Agar {
                    self.dish.scent[i] += strength;
                }
            }
        }
    }

    fn check_core(&mut self) {
        if self.core_down.is_some() {
            return;
        }
        let r = self.rules.core_r;
        let on = self
            .soldiers
            .iter()
            .any(|s| s.alive && (s.x - self.core.0).hypot(s.y - self.core.1) <= r);
        if on && self.walkers_near(self.core, 2.0 * r) == 0 {
            self.core_clear += 1;
            if self.core_clear >= secs(self.rules.core_secs) {
                self.core_down = Some(self.dish.t);
                self.dish.params.divide = 0.0;
            }
        } else {
            self.core_clear = 0;
        }
    }

    fn check_end(&mut self) {
        let t = self.time();
        if self.alive() == 0 {
            self.end = Some(End::SquadWiped(t));
        } else if self.dish.walkers.is_empty() {
            self.end = Some(End::MouldWiped(t));
        } else if t >= self.rules.cap {
            self.end = Some(End::Cap(self.share()));
        }
    }

    /// **Share of the map held**, for the cap (method review, finding 11:
    /// define it): of the piles with food that either side holds, the
    /// fraction the squad holds. The squad holds a pile with a soldier
    /// within range of it and no mould eating; the mould holds one it is
    /// eating. One half when neither holds any.
    pub fn share(&self) -> f32 {
        let (mut ours, mut theirs) = (0, 0);
        for p in self.dish.piles.iter().filter(|p| p.food > 0.0) {
            if p.eaters > 0 {
                theirs += 1;
            } else if self
                .soldiers
                .iter()
                .any(|s| s.alive && (s.x - p.x).hypot(s.y - p.y) <= self.rules.range)
            {
                ours += 1;
            }
        }
        if ours + theirs == 0 {
            0.5
        } else {
            ours as f32 / (ours + theirs) as f32
        }
    }

    pub fn score(&self) -> Option<f32> {
        self.end.map(|e| e.score(self.alive(), self.rules.cap))
    }

    /// Run to the end, with `plan` called once a game second before the step.
    pub fn play(&mut self, mut plan: impl FnMut(&mut Game)) -> End {
        while self.end.is_none() {
            if self.dish.t.is_multiple_of(GAME_SECOND) {
                plan(self);
            }
            self.step();
        }
        self.end.expect("ended")
    }

    fn pile_food(&self) -> f32 {
        self.dish.piles.iter().map(|p| p.food).sum()
    }

    /// Buy a soldier at the larder. True if bought.
    pub fn buy(&mut self) -> bool {
        if self.bank < self.rules.soldier_cost || self.alive() >= self.rules.max_squad {
            return false;
        }
        self.bank -= self.rules.soldier_cost;
        self.spent += self.rules.soldier_cost;
        self.bought += 1;
        let a = self.rng.f32() * std::f32::consts::TAU;
        self.soldiers.push(Soldier {
            x: self.larder.0 + 3.0 * a.cos(),
            y: self.larder.1 + 3.0 * a.sin(),
            hp: self.rules.hp,
            order: Order::Hold,
            stance: Stance::Defend,
            carry: 0.0,
            homing: false,
            alive: true,
            ready: 0,
            beat: 0,
        });
        true
    }

    /// Within range of a live soldier: where the squad can build or bait.
    pub fn reachable(&self, at: (f32, f32)) -> bool {
        self.soldiers
            .iter()
            .any(|s| s.alive && (s.x - at.0).hypot(s.y - at.1) <= self.rules.range)
    }

    /// **Bait**: spend `food` from the bank to lay a pile of it at `at`,
    /// within range of a soldier, on open agar. The mould smells it like
    /// any pile (the design review's verb for the Baiter plan).
    pub fn bait(&mut self, at: (f32, f32), food: f32) -> bool {
        if food <= 0.0 || self.bank < food || !self.reachable(at) || !self.open(at) {
            return false;
        }
        self.bank -= food;
        self.spent += food;
        self.baited += food;
        self.dish.piles.push(Pile {
            x: at.0,
            y: at.1,
            r: 2.5,
            food,
            full: food,
            oats: false,
            expires: u64::MAX,
            eaters: 0,
            foe_eaters: 0,
            held: 0,
        });
        true
    }

    fn open(&self, at: (f32, f32)) -> bool {
        at.0 >= 1.0
            && at.1 >= 1.0
            && (at.0 as usize) < W - 1
            && (at.1 as usize) < H - 1
            && self.dish.ground[at.1 as usize * W + at.0 as usize] == Ground::Agar
            && self.dish.piles.iter().all(|p| (p.x - at.0).hypot(p.y - at.1) > p.r + 3.0)
    }

    /// **A turret** at `at`, within range of a soldier. True if built.
    pub fn build_turret(&mut self, at: (f32, f32)) -> bool {
        if self.bank < self.rules.turret_cost || !self.reachable(at) || !self.open(at) {
            return false;
        }
        self.bank -= self.rules.turret_cost;
        self.spent += self.rules.turret_cost;
        self.turrets.push(Turret {
            x: at.0,
            y: at.1,
            hp: self.rules.turret_hp,
            ready: 0,
        });
        true
    }

    fn turrets_turn(&mut self) {
        let t = self.dish.t;
        let per_step = 1.0 / GAME_SECOND as f32;
        let mut k = 0;
        while k < self.turrets.len() {
            let tu = self.turrets[k];
            if tu.ready <= t && self.bank >= self.rules.price {
                if let Some(at) = self.densest((tu.x, tu.y), self.rules.turret_range) {
                    let (dx, dy) = (at.0 - tu.x, at.1 - tu.y);
                    let d = dx.hypot(dy).max(0.1);
                    let (ux, uy) = (-dy / d, dx / d);
                    let h = self.rules.line / 2.0;
                    self.fire_line((at.0 - ux * h, at.1 - uy * h), (at.0 + ux * h, at.1 + uy * h));
                    self.turrets[k].ready = t + secs(self.rules.shot_every);
                }
            }
            let n = self.walkers_near((tu.x, tu.y), self.rules.touch + 1.0);
            self.turrets[k].hp -= self.rules.drain * n as f32 * per_step;
            if self.turrets[k].hp <= 0.0 {
                self.turrets.swap_remove(k);
            } else {
                k += 1;
            }
        }
    }

    /// The larder smells of the bank (`Rules::hoard`).
    fn hoard_smell(&mut self) {
        if self.rules.hoard <= 0.0 || self.bank <= 0.0 {
            return;
        }
        let strength = self.dish.params.food_deposit * self.rules.hoard * self.bank / 10_000.0;
        let r = 4.0;
        for (ox, oy) in disc(r) {
            let (x, y) = (self.larder.0 + ox, self.larder.1 + oy);
            if x < 0.0 || y < 0.0 || x as usize >= W || y as usize >= H {
                continue;
            }
            let i = y as usize * W + x as usize;
            if self.dish.ground[i] == Ground::Agar {
                self.dish.scent[i] += strength;
            }
        }
    }

    /// Timed drops (`Rules::drop_every`): the next one is chosen a drop's
    /// interval ahead, smells from `drop_warn` before, and lands.
    fn drops(&mut self) {
        let r = self.rules;
        if r.drop_every <= 0.0 {
            return;
        }
        let t = self.dish.t;
        if self.next_drop.is_none() {
            let when = t + secs(r.drop_every);
            if when > secs(r.drops_stop) {
                return;
            }
            let at = loop {
                let at = (self.rng.f32() * W as f32, self.rng.f32() * H as f32);
                if self.open(at) {
                    break at;
                }
            };
            self.next_drop = Some((when, at));
        }
        let (when, at) = self.next_drop.expect("set");
        if t + secs(r.drop_warn) >= when {
            let strength = self.dish.params.food_deposit;
            for (ox, oy) in disc(2.5) {
                let (x, y) = (at.0 + ox, at.1 + oy);
                if x >= 0.0 && y >= 0.0 && (x as usize) < W && (y as usize) < H {
                    let i = y as usize * W + x as usize;
                    if self.dish.ground[i] == Ground::Agar {
                        self.dish.scent[i] += strength;
                    }
                }
            }
        }
        if t >= when {
            self.dish.piles.push(Pile {
                x: at.0,
                y: at.1,
                r: 2.5,
                food: r.drop_food,
                full: r.drop_food,
                oats: false,
                expires: u64::MAX,
                eaters: 0,
                foe_eaters: 0,
                held: 0,
            });
            self.next_drop = None;
        }
    }

    // ------------------------------------------------------- the soldier

    fn walk(&mut self, k: usize, to: (f32, f32), stop: f32) -> bool {
        let s = &mut self.soldiers[k];
        let (dx, dy) = (to.0 - s.x, to.1 - s.y);
        let d = dx.hypot(dy);
        if d <= stop {
            return true;
        }
        let v = self.rules.speed / GAME_SECOND as f32;
        let go = v.min(d - stop);
        let (nx, ny) = (s.x + dx / d * go, s.y + dy / d * go);
        if nx >= 0.0 && ny >= 0.0 && (nx as usize) < W && (ny as usize) < H {
            s.x = nx;
            s.y = ny;
        }
        false
    }

    fn soldier_turn(&mut self, k: usize) {
        let s = self.soldiers[k];
        let here = (s.x, s.y);
        // Food goes home first.
        if s.homing {
            if self.walk(k, self.larder, 2.0) {
                self.bank += s.carry;
                self.harvested += s.carry;
                self.soldiers[k].carry = 0.0;
                self.soldiers[k].homing = false;
            }
            self.defend(k, here);
            return;
        }
        match s.order {
            Order::Hold => self.defend(k, here),
            Order::Move(x, y) => {
                let o = slot(k);
                self.walk(k, (x + o.0, y + o.1), 0.5);
                self.defend(k, here);
            }
            Order::Clear(x, y) => {
                let Some(pi) = self.pile_at((x, y)) else {
                    self.soldiers[k].order = Order::Hold;
                    return;
                };
                // While the mould is eating the pile, fire from range and
                // walk in only once it is clear. Walking straight on (M1,
                // M2) put the squad in the mould: in M2b's first look a
                // striker's 18 soldiers died in 47 s on a 446-walker mould,
                // each body a 400-food pile for it.
                let stop = if self.dish.piles[pi].eaters > 0 { self.rules.range - 2.0 } else { 2.0 };
                let o = slot(k);
                self.walk(k, (x + o.0, y + o.1), stop);
                // At the pile: on or beside it, wherever its formation slot
                // put it (a slot 2.6 cells out never came within the old
                // 2-cell stop of the centre, and the fortress's farmer
                // harvested nothing).
                let me = self.soldiers[k];
                let at = stop <= 2.0 && (me.x - x).hypot(me.y - y) <= self.dish.piles[pi].r + 2.0 + o.0.hypot(o.1);
                let p = &self.dish.piles[pi];
                let (eaters, food, r) = (p.eaters, p.food, p.r);
                if eaters > 0 || s.stance == Stance::Free {
                    if let Some(t) = self.densest((x, y), r + 4.0) {
                        self.shoot(k, t);
                    }
                }
                self.defend(k, here);
                if at && eaters == 0 && food > 0.0 {
                    let take = (self.rules.harvest / GAME_SECOND as f32).min(food);
                    self.dish.piles[pi].food -= take;
                    self.soldiers[k].carry += take;
                    if self.soldiers[k].carry >= self.rules.carry {
                        self.soldiers[k].homing = true;
                    }
                }
            }
            Order::Burn(a, b) => {
                let mid = ((a.0 + b.0) / 2.0, (a.1 + b.1) / 2.0);
                let half = (b.0 - a.0).hypot(b.1 - a.1) / 2.0;
                self.walk(k, mid, (self.rules.range - half - 1.0).max(1.0));
                if self.soldiers[k].ready <= self.dish.t {
                    // Pieces of `line` along the segment, fired in turn.
                    let n = ((2.0 * half / self.rules.line).ceil() as u32).max(1);
                    let j = self.soldiers[k].beat % n;
                    let f = (j as f32 + 0.5) / n as f32;
                    let at = (a.0 + (b.0 - a.0) * f, a.1 + (b.1 - a.1) * f);
                    let (ux, uy) = ((b.0 - a.0) / (2.0 * half).max(0.1), (b.1 - a.1) / (2.0 * half).max(0.1));
                    let h = self.rules.line / 2.0;
                    let s = self.soldiers[k];
                    if (at.0 - s.x).hypot(at.1 - s.y) <= self.rules.range && self.bank >= self.rules.price {
                        self.fire_line((at.0 - ux * h, at.1 - uy * h), (at.0 + ux * h, at.1 + uy * h));
                        self.soldiers[k].ready = self.dish.t + secs(self.rules.shot_every);
                        self.soldiers[k].beat += 1;
                    }
                }
                self.defend(k, here);
            }
            Order::Attack => {
                self.walk(k, self.core, 1.0);
                let s = self.soldiers[k];
                if let Some(t) = self.densest((s.x, s.y), self.rules.range) {
                    self.shoot(k, t);
                }
            }
        }
    }

    /// The stance's own fire, after the order's.
    fn defend(&mut self, k: usize, _was: (f32, f32)) {
        let s = self.soldiers[k];
        let r = match s.stance {
            Stance::HoldFire => return,
            Stance::Defend => 4.0,
            Stance::Free => self.rules.range,
        };
        if let Some(t) = self.densest((s.x, s.y), r) {
            self.shoot(k, t);
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn game(seed: u64) -> Game {
        field_game(seed, Rules::default())
    }

    #[test]
    fn a_shot_wounds_for_game_seconds() {
        let mut g = game(1);
        // Open agar, away from every pile (a shot never wounds a pile).
        let at = (0..H * W)
            .map(|i| ((i % W) as f32 + 0.5, (i / W) as f32 + 0.5))
            .find(|&(x, y)| {
                x > 10.0
                    && g.dish.ground[y as usize * W + x as usize] == Ground::Agar
                    && g.dish.piles.iter().all(|p| (p.x - x).hypot(p.y - y) > p.r + 4.0)
            })
            .unwrap();
        g.soldiers[0].x = at.0 - 5.0;
        g.soldiers[0].y = at.1;
        assert!(g.shoot(0, at));
        let i = at.1 as usize * W + at.0 as usize;
        assert_eq!(g.dish.ground[i], Ground::Wound);
        assert!(!g.shoot(0, at), "reloading");
        for _ in 0..secs(g.rules.wound) - 1 {
            g.dish.step();
        }
        assert_eq!(g.dish.ground[i], Ground::Wound, "open until 4 game seconds");
        g.dish.step();
        g.dish.step();
        assert_eq!(g.dish.ground[i], Ground::Agar, "healed after them");
        assert_eq!(g.bank, g.rules.start_bank - g.rules.price);
    }

    #[test]
    fn out_of_range_or_broke_does_not_fire() {
        let mut g = game(1);
        let s = g.soldiers[0];
        assert!(!g.shoot(0, (s.x + 40.0, s.y)));
        g.bank = 0.0;
        assert!(!g.shoot(0, (s.x + 2.0, s.y)));
        assert_eq!(g.shots, 0);
    }

    #[test]
    fn a_soldier_in_the_mould_dies_and_becomes_food() {
        let mut g = game(2);
        g.soldiers.truncate(1);
        g.soldiers[0].stance = Stance::HoldFire;
        let w = g.dish.walkers[0];
        let piles = g.dish.piles.len();
        for _ in 0..secs(600.0) {
            // Pinned on the mould's oldest cell.
            g.soldiers[0].x = w.x;
            g.soldiers[0].y = w.y;
            g.step();
            if !g.soldiers[0].alive {
                break;
            }
        }
        assert!(!g.soldiers[0].alive);
        assert_eq!(g.dish.piles.len(), piles + 1, "its body is a pile");
        assert!(matches!(g.end, Some(End::SquadWiped(_))));
        assert!(g.score().unwrap() < -1.0);
    }

    #[test]
    fn scores_are_ordered_by_band() {
        let cap = 900.0;
        let win_late = End::MouldWiped(899.0).score(0, cap);
        let stalemate = End::Cap(1.0).score(4, cap);
        let lose_late = End::SquadWiped(899.0).score(0, cap);
        assert!(win_late > stalemate && stalemate > lose_late);
        assert!(End::MouldWiped(100.0).score(4, cap) > End::MouldWiped(200.0).score(4, cap));
        assert!(End::SquadWiped(200.0).score(0, cap) > End::SquadWiped(100.0).score(0, cap));
    }

    /// Open agar within `r` of soldier 0, away from piles.
    fn open_near(g: &Game, r: f32) -> (f32, f32) {
        let s = g.soldiers[0];
        (0..64)
            .flat_map(|k| {
                let a = k as f32 / 64.0 * std::f32::consts::TAU;
                (1..=r as usize).map(move |d| (s.x + d as f32 * a.cos(), s.y + d as f32 * a.sin()))
            })
            .find(|&at| g.open(at))
            .expect("open agar near the squad")
    }

    #[test]
    fn buying_costs_food_and_stops_at_the_cap() {
        let mut g = game(1);
        g.bank = 1e9;
        let start = g.alive();
        assert!(g.buy());
        assert_eq!(g.alive(), start + 1);
        assert_eq!(g.bank, 1e9 - g.rules.soldier_cost);
        while g.buy() {}
        assert_eq!(g.alive(), g.rules.max_squad);
        g.bank = g.rules.soldier_cost - 1.0;
        g.rules.max_squad += 1;
        assert!(!g.buy(), "broke");
    }

    #[test]
    fn bait_is_a_pile_laid_in_range() {
        let mut g = game(1);
        let piles = g.dish.piles.len();
        let s = g.soldiers[0];
        assert!(!g.bait((s.x + 60.0, s.y), 300.0), "out of every soldier's range");
        let at = open_near(&g, g.rules.range);
        assert!(g.bait(at, 300.0));
        assert_eq!(g.dish.piles.len(), piles + 1);
        assert_eq!(g.dish.piles[piles].food, 300.0);
        assert_eq!(g.bank, g.rules.start_bank - 300.0);
        g.bank = 100.0;
        let at2 = open_near(&g, g.rules.range);
        assert!(!g.bait(at2, 300.0), "broke");
    }

    #[test]
    fn a_turret_fires_from_the_bank_and_the_mould_eats_it() {
        let mut g = game(2);
        g.soldiers.iter_mut().for_each(|s| s.stance = Stance::HoldFire);
        let w = g.dish.walkers[0];
        // Stand soldier 0 on the mould's first cell; build next to it.
        g.soldiers[0].x = w.x + 4.0;
        g.soldiers[0].y = w.y;
        g.bank = 1e6;
        let at = open_near(&g, 4.0);
        assert!(g.build_turret(at));
        assert_eq!(g.turrets.len(), 1);
        let mut gone = false;
        for _ in 0..secs(600.0) {
            g.soldiers[0].hp = g.rules.hp;
            g.step();
            if g.turrets.is_empty() {
                gone = true;
                break;
            }
        }
        assert!(g.bank < 1e6 - g.rules.turret_cost, "its shots were paid for");
        assert!(gone, "a turret in the mould is eaten");
    }

    #[test]
    fn same_commands_same_hash() {
        let mut g = game(4);
        g.bank = 5000.0;
        let s = g.soldiers[0];
        for step in 0..secs(90.0) {
            match step {
                5 => {
                    g.apply(Cmd::Order(0, Order::Move(s.x - 30.3, s.y + 7.7)));
                }
                40 => {
                    g.apply(Cmd::Stance(1, Stance::Free));
                }
                60 => {
                    g.apply(Cmd::Buy);
                }
                90 => {
                    g.apply(Cmd::Order(2, Order::Burn((s.x - 9.1, s.y - 5.0), (s.x - 9.1, s.y + 5.0))));
                }
                200 => {
                    g.apply(Cmd::Order(3, Order::Attack));
                }
                _ => {}
            }
            g.step();
        }
        let text = g.replay_text(4);
        let mut again = Game::replay(&text, Rules { start_bank: 5000.0, ..Rules::default() }).expect("parses");
        // The bank was set by hand above, before any command: the replay
        // starts with it in the rules instead.
        assert_eq!(again.dish.t, g.dish.t);
        assert_eq!(again.dish.state_hash(), g.dish.state_hash());
        assert_eq!(again.bought, 1);
        for c in [Cmd::Order(2, Order::Burn((1.5, 2.25), (3.0, 4.0))), Cmd::Bait((9.75, 3.0), 300.0), Cmd::Stance(0, Stance::HoldFire), Cmd::Turret((7.0, 8.0))] {
            assert_eq!(Cmd::parse(&c.to_line()), Some(c));
        }
        again.step();
    }

    #[test]
    fn same_seed_same_round() {
        let run = |seed| {
            let mut g = game(seed);
            for _ in 0..secs(60.0) {
                if g.dish.t.is_multiple_of(GAME_SECOND) {
                    g.order(0, Order::Attack);
                }
                g.step();
            }
            (g.dish.state_hash(), g.shots, g.killed, g.soldiers[0].hp.to_bits())
        };
        assert_eq!(run(3), run(3));
    }
}
