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
            drain: 2.0,
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
            End::SquadWiped(t) => -(1.0 + (cap - t) / cap),
        }
    }

    pub fn band(self) -> &'static str {
        match self {
            End::MouldWiped(_) => "mould wiped",
            End::Cap(_) => "cap",
            End::SquadWiped(_) => "squad wiped",
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
    /// Own RNG, apart from the dish's, so a soldier rule never shifts the
    /// mould's dice (engineering review, finding 11).
    pub rng: Rng,
    bins: Vec<u16>,
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
        }
    }

    /// Game seconds since the start.
    pub fn time(&self) -> f32 {
        self.dish.t as f32 / GAME_SECOND as f32
    }

    pub fn alive(&self) -> usize {
        self.soldiers.iter().filter(|s| s.alive).count()
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
                self.soldiers[k].hp -= self.rules.drain * n as f32 * per_step;
                if self.soldiers[k].hp <= 0.0 {
                    self.die(k);
                }
            }
        }
        self.smell();
        self.check_core();
        self.dish.step();
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
        let strength = self.dish.params.food_deposit * self.rules.smell;
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
                self.walk(k, (x, y), 0.5);
                self.defend(k, here);
            }
            Order::Clear(x, y) => {
                let at = self.walk(k, (x, y), 2.0);
                let Some(pi) = self.pile_at((x, y)) else {
                    self.soldiers[k].order = Order::Hold;
                    return;
                };
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
