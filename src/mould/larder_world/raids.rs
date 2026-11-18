//! **Raids** (2026-11-15, the defenses thread). The owner, 20:56Z: "I could
//! build a huge defense, but that just ends with the mold clustering around
//! its own crystals and not doing anything. even if i build lures, only a
//! thin trickle moves into my kill box". Picked 21:15Z: "Raids is the
//! number 1 option. Also consider that a slime that is losing might rush you
//! will everything it has."
//!
//! Why the turtle stalled: the surge (`BigParams::surge_every`) and the
//! panic charge reach only units within `panic_r` (the gun's range plus
//! `Rules::panic_margin`), so a defense built out of that reach was never
//! attacked, and a lure's scent (8 times a soldier's over 6 cells) reached
//! only the slime's edge. Outside games make static defense pay by making
//! the enemy come to it: Factorio's pollution reaches the nests and sets off
//! attacks on the factory, They Are Billions' noise draws the horde,
//! tower defense's enemy must pass the towers.
//!
//! A raid comes at random times (the gap drawn from a normal distribution
//! each raid, `RaidRules::every`; until 11-16 it came every 400 food the
//! slime ate, too regular for the owner), and the slime sends a share of
//! its body, drawn the same way, at the player's richest holding: the pump drawing the most, else any
//! standing building, else the thickest knot of soldiers. A lure that is on
//! within `RaidRules::lure_r` of that target takes the raid instead, so a
//! lure in front of the pumps with turrets round it is a kill box the raid
//! walks into. The raid's size is a share of the slime's mass (`size`), so
//! it grows as the slime does; the world sends the walkers nearest the
//! target (`BigParams::raid_at`, both engines): the game finds the reach
//! round the target that holds enough of them in the mould grid.
//!
//! **The last stand** (`RaidRules::last_stand`, off: the owner was "not
//! sure if this is a good idea or not"): once, when the slime's mass falls
//! under `last_stand_at` of its peak, every walker it has rushes the
//! thickest knot of soldiers.

use super::{Game, Kind, Rng, World};

/// The raids' numbers. Times in game seconds, distances in dish cells.
///
/// Owner 2026-11-16 01:48Z: "slime should send out raiders regardless if it
/// doesn't make a kill box a winning strategy yet. It shouldn't be so
/// consistent as every 400 food. Both the time between raids and the size
/// of the raid should be picked randomly (each raid) from a normal
/// distribution". Until then a raid came every 400 food the slime ate
/// (40-150 s apart): on the Fork turtle every 42 s like a clock.
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct RaidRules {
    /// The time till the next raid is drawn from a normal distribution
    /// with this mean and `every_sd` (0: raids off), never under `min`.
    /// 60 s: the food rule's gaps were 42 s on the Fork turtle and up to
    /// 150 s on a lean slime; a minute keeps about four in the first five
    /// minutes.
    pub every: f32,
    pub every_sd: f32,
    /// The first raid's time from the landing, drawn the same way.
    pub first: f32,
    pub first_sd: f32,
    pub min: f32,
    /// The raid's size, a share of the slime's mass: drawn from a normal
    /// distribution with this mean and `size_sd` each raid, kept within
    /// `size_min`..`size_max`.
    pub size: f32,
    pub size_sd: f32,
    pub size_min: f32,
    pub size_max: f32,
    /// The share of the walkers in the raid's reach that go: below 1 the
    /// body round the target thins rather than leaving it bare.
    pub share: f32,
    /// A lure that is on within this of the raid's target takes the raid.
    pub lure_r: f32,
    /// The last stand (off), and the share of its peak mass the slime
    /// falls under to make it.
    pub last_stand: bool,
    pub last_stand_at: f32,
}

impl Default for RaidRules {
    fn default() -> Self {
        RaidRules {
            every: 60.0,
            every_sd: 20.0,
            first: 75.0,
            first_sd: 15.0,
            min: 20.0,
            size: 0.15,
            size_sd: 0.06,
            size_min: 0.04,
            size_max: 0.4,
            share: 0.6,
            lure_r: 40.0,
            last_stand: false,
            last_stand_at: 0.25,
        }
    }
}

/// What a raid went for.
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum RaidAim {
    /// Building `k` (a pump, or any standing building).
    Building(usize),
    /// Lure `k`, which took the raid from building `from` (None: from the
    /// soldiers).
    Lure { lure: usize, from: Option<usize> },
    /// The thickest knot of soldiers, round soldier `k`.
    Soldiers(usize),
}

/// One raid the game sent.
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct Raid {
    /// Tick it began.
    pub tick: u64,
    pub aim: RaidAim,
    /// Its target, fine cells, and reach, dish cells.
    pub to: (f32, f32),
    pub reach: f32,
    /// Its size as drawn (a share of the slime's mass; infinite for the
    /// last stand).
    pub size: f32,
    /// The walkers the reach held as it began (the mould grid's count),
    /// and the share of them sent.
    pub held: u32,
    pub share: f32,
    pub last_stand: bool,
}

/// The game's raid state.
#[derive(Debug)]
pub struct Raids {
    /// Every raid sent, oldest first.
    pub sent: Vec<Raid>,
    /// The tick the next raid is due (drawn at the first second, then at
    /// each raid).
    pub next: Option<u64>,
    /// The raids' own draws, apart from `Game::rng` so raids don't move
    /// any other draw in the game.
    rng: Rng,
    pub(super) last_stand_done: bool,
    /// A raid decided and waiting a game second for its way round walls,
    /// which a worker thread builds (`big::raid_field_for`: 7-8 ms on
    /// planet 8, too long for one step by the levelgen thread's hitch rule).
    pending: Option<(Raid, Way)>,
    /// The units the last frame sent (fine cells, x | y << 16), for their
    /// walking distance (`Rules::unit_paths`, S1).
    pub(super) unit_cells: Vec<u32>,
    /// The units' walking distance being built, set a game second after
    /// it was started (`time_unit_paths`).
    unit_way: Option<Way>,
    /// The far smell's field being built (S2, `time_food_paths`).
    pub(super) food_way: Option<Way>,
    /// LAB: the marks the squad's shots have laid (`lab.rs`, `LAB_SHOT`).
    pub(super) lab_shots: Vec<super::lab::ShotMark>,
}

impl Raids {
    pub fn new(seed: u64) -> Raids {
        Raids { sent: Vec::new(), next: None, rng: Rng::new(seed ^ 0x7a1d_5eed), last_stand_done: false, pending: None, unit_cells: Vec::new(), unit_way: None, food_way: None, lab_shots: Vec::new() }
    }

    /// A normal draw (Box-Muller) with mean `m` and spread `sd`.
    pub(super) fn normal(&mut self, m: f32, sd: f32) -> f32 {
        let u = self.rng.f32().max(1e-7);
        let v = self.rng.f32();
        m + sd * (-2.0 * u.ln()).sqrt() * (std::f32::consts::TAU * v).cos()
    }
}

impl Default for Raids {
    fn default() -> Self {
        Raids::new(0)
    }
}

/// A raid's way round walls being built.
/// How often (game seconds) the far smell's field is rebuilt
/// (`time_food_paths`): untouched crystals change slowly.
pub(super) const FOOD_PATHS_EVERY: u64 = 2;

/// A test's delay (ms) on every way's thread, to show that the game is
/// the same however slow the thread (the technical lead, 11-17 13:50Z).
#[cfg(test)]
pub(super) static WAY_DELAY_MS: std::sync::atomic::AtomicU64 = std::sync::atomic::AtomicU64::new(0);

#[cfg(not(target_arch = "wasm32"))]
fn way_delay() {
    #[cfg(test)]
    std::thread::sleep(std::time::Duration::from_millis(WAY_DELAY_MS.load(std::sync::atomic::Ordering::Relaxed)));
}

#[derive(Debug)]
pub(super) enum Way {
    Ready(Vec<u32>),
    #[cfg(not(target_arch = "wasm32"))]
    Building(std::thread::JoinHandle<Vec<u32>>),
}

impl Way {
    /// Built off the frame where threads exist (the native game), at once
    /// on the web build.
    fn start(ground: Option<(Vec<bool>, usize, usize)>, to: u32) -> Way {
        let Some((agar, w, h)) = ground else { return Way::Ready(Vec::new()) };
        #[cfg(not(target_arch = "wasm32"))]
        {
            Way::Building(std::thread::spawn(move || {
                way_delay();
                crate::mould::big::raid_field_for(&agar, w, h, to)
            }))
        }
        #[cfg(target_arch = "wasm32")]
        {
            Way::Ready(crate::mould::big::raid_field_for(&agar, w, h, to))
        }
    }

    /// The units' walking distance (`big::units_field_for`), off the
    /// frame as `start`'s.
    fn start_units(ground: Option<(Vec<bool>, usize, usize)>, units: Vec<u32>) -> Way {
        let Some((agar, w, h)) = ground else { return Way::Ready(Vec::new()) };
        #[cfg(not(target_arch = "wasm32"))]
        {
            Way::Building(std::thread::spawn(move || {
                way_delay();
                crate::mould::big::units_field_for(&agar, w, h, &units)
            }))
        }
        #[cfg(target_arch = "wasm32")]
        {
            Way::Ready(crate::mould::big::units_field_for(&agar, w, h, &units))
        }
    }

    /// LAB: any field built by `build` off the frame, as `start`'s.
    pub(super) fn start_with(build: impl FnOnce() -> Vec<u32> + Send + 'static) -> Way {
        #[cfg(not(target_arch = "wasm32"))]
        {
            Way::Building(std::thread::spawn(move || {
                way_delay();
                build()
            }))
        }
        #[cfg(target_arch = "wasm32")]
        {
            Way::Ready(build())
        }
    }

    pub(super) fn finish(self) -> Vec<u32> {
        match self {
            Way::Ready(f) => f,
            #[cfg(not(target_arch = "wasm32"))]
            Way::Building(j) => j.join().unwrap_or_default(),
        }
    }
}

impl<W: World> Game<W> {
    /// **The units' walking distance** (`Rules::unit_paths`, the
    /// simulation-use plan's S1, 2026-11-17): once a game second, the
    /// distance started a second ago is set (`World::set_units_field`) and
    /// a new one is started from where the units stood in the last frame.
    /// Always a second late, so the game is the same however fast the
    /// thread was (as a raid's way, `time_raids`). Charging walkers go
    /// down it, so walls bend a surge or a panic charge.
    pub(super) fn time_unit_paths(&mut self) {
        if !self.rules.unit_paths {
            return;
        }
        if let Some(way) = self.raids.unit_way.take() {
            self.world.set_units_field(way.finish());
        }
        let units = std::mem::take(&mut self.raids.unit_cells);
        self.raids.unit_way = Some(Way::start_units(self.world.raid_ground(), units));
    }

    /// **The far smell** (`BigParams::far_smell`, the simulation-use plan's
    /// S2, 2026-11-17): every `FOOD_PATHS_EVERY` game seconds, the field
    /// started last time is set (`World::set_food_field`) and a new one is
    /// started from the untouched crystals, off the frame and always one
    /// period late, joining the worker if it is slow, as the units' way.
    pub(super) fn time_food_paths(&mut self) {
        // LAB: the steering field takes the far smell's slot.
        if self.world.lab_steer() {
            return; // `Game::lab_turn` builds it, at its own pace.
        }
        if !self.world.far_smell() {
            return;
        }
        let secs = (self.world.ticks() as f32 / self.rules.tps as f32).round() as u64;
        if !secs.is_multiple_of(FOOD_PATHS_EVERY) {
            return;
        }
        if let Some(way) = self.raids.food_way.take() {
            self.world.set_food_field(way.finish());
        }
        let fresh = self.world.fresh_crystals();
        self.raids.food_way = Some(Way::start_units(self.world.raid_ground(), fresh));
    }

    /// Once a game second: start a raid when its drawn time comes, or the
    /// last stand.
    pub(super) fn time_raids(&mut self) {
        // A raid decided a game second ago goes now, with its way round
        // walls: always a second later, so the game is the same however
        // fast the thread was.
        if let Some((mut d, way)) = self.raids.pending.take() {
            let field = way.finish();
            let fine = self.world.fine_steps().max(1) as u64;
            d.tick = self.world.ticks();
            self.world.set_raid((d.tick * fine + 1).min(u32::MAX as u64 - 1) as u32, d.to, d.reach, d.share, field);
            self.raids.sent.push(d);
            return;
        }
        let r = self.rules.raid;
        let now = self.world.ticks();
        let secs = |s: f32| (s.max(0.0) * self.rules.tps as f32).round() as u64;
        if r.last_stand && !self.raids.last_stand_done && self.peak > 0 && (self.mould as f32) < r.last_stand_at * self.peak as f32 && self.mould > 0 {
            if let Some(k) = self.thickest_knot() {
                self.raids.last_stand_done = true;
                let to = (self.soldiers[k].x, self.soldiers[k].y);
                self.send_raid(RaidAim::Soldiers(k), to, f32::INFINITY, 1.0, true);
                return;
            }
        }
        if r.every <= 0.0 {
            return;
        }
        let gap = |raids: &mut Raids, m: f32, sd: f32| secs(raids.normal(m, sd).max(r.min));
        let next = match self.raids.next {
            Some(t) => t,
            None => {
                let t = gap(&mut self.raids, r.first, r.first_sd);
                self.raids.next = Some(t);
                t
            }
        };
        if now < next {
            return;
        }
        let Some((aim, to)) = self.raid_aim() else { return };
        let size = self.raids.normal(r.size, r.size_sd).clamp(r.size_min, r.size_max);
        self.raids.next = Some(now + gap(&mut self.raids, r.every, r.every_sd));
        self.send_raid(aim, to, size, r.share, false);
    }

    /// What the next raid goes for, and where (fine cells): the pump
    /// drawing most, else the standing building with the most health, else
    /// the thickest knot of soldiers; a lure that is on within
    /// `RaidRules::lure_r` of it takes it (the nearest such lure).
    pub fn raid_aim(&self) -> Option<(RaidAim, (f32, f32))> {
        let sc = self.world.scale();
        let up = |k: &usize| self.buildings[*k].up() && !matches!(self.buildings[*k].kind, Kind::Wall { .. });
        let pump = (0..self.buildings.len()).filter(up).filter(|&k| self.buildings[k].kind == Kind::Pump && self.pump_draw(k) > 0.0).max_by(|&a, &b| self.pump_draw(a).total_cmp(&self.pump_draw(b)));
        let any = || (0..self.buildings.len()).filter(up).filter(|&k| !self.is_lure(k)).max_by(|&a, &b| self.buildings[a].max_hp.total_cmp(&self.buildings[b].max_hp));
        let from = pump.or_else(any);
        let to = match from {
            Some(k) => self.buildings[k].at,
            None => {
                let k = self.thickest_knot()?;
                (self.soldiers[k].x, self.soldiers[k].y)
            }
        };
        let reach = self.rules.raid.lure_r * sc;
        let lure = (0..self.buildings.len())
            .filter(up)
            .filter(|&k| self.is_lure(k) && self.buildings[k].on)
            .map(|k| (k, (self.buildings[k].at.0 - to.0).hypot(self.buildings[k].at.1 - to.1)))
            .filter(|&(_, d)| d <= reach)
            .min_by(|a, b| a.1.total_cmp(&b.1));
        Some(match (lure, from) {
            (Some((l, _)), _) => (RaidAim::Lure { lure: l, from }, self.buildings[l].at),
            (None, Some(k)) => (RaidAim::Building(k), to),
            (None, None) => (RaidAim::Soldiers(self.thickest_knot()?), to),
        })
    }

    fn is_lure(&self, k: usize) -> bool {
        self.buildings[k].kind == Kind::Lure
    }

    /// The live soldier with the most live soldiers within 15 dish cells
    /// (ties to the first).
    fn thickest_knot(&self) -> Option<usize> {
        let r = 15.0 * self.world.scale();
        let live: Vec<usize> = (0..self.soldiers.len()).filter(|&k| self.soldiers[k].alive).collect();
        live.iter()
            .map(|&k| {
                let s = &self.soldiers[k];
                (k, live.iter().filter(|&&j| (self.soldiers[j].x - s.x).hypot(self.soldiers[j].y - s.y) <= r).count())
            })
            .fold(None, |best: Option<(usize, usize)>, (k, n)| if best.is_none_or(|b| n > b.1) { Some((k, n)) } else { best })
            .map(|b| b.0)
    }

    /// Decides a raid at `to` (fine cells): `size` of the slime's mass (an
    /// infinite size sends the whole slime), `share` of the walkers in the
    /// reach. It goes at the next game second (`time_raids`).
    fn send_raid(&mut self, aim: RaidAim, to: (f32, f32), size: f32, share: f32, last_stand: bool) {
        let (reach, held) = if size.is_finite() { self.raid_reach(to, size * self.world.mass() as f32 / share.max(1e-3)) } else { (1.0e4, self.world.mass() as u32) };
        let way = Way::start(self.world.raid_ground(), super::raid_cell(to, self.world.size()));
        self.raids.pending = Some((Raid { tick: self.world.ticks(), aim, to, reach, size, held, share, last_stand }, way));
    }

    /// The smallest reach round `to` (dish cells) whose squares of the
    /// mould grid hold `want` walkers, and what it holds; the whole map
    /// where the world gives no grid.
    fn raid_reach(&self, to: (f32, f32), want: f32) -> (f32, u32) {
        let sc = self.world.scale();
        let bin = (sc as usize).max(1);
        let Some(g) = self.world.mould_grid(bin) else { return (1.0e4, 0) };
        let (w, _) = self.world.size();
        let gw = w.div_ceil(bin);
        let b = bin as f32;
        let mut near: Vec<(f32, u32)> = g
            .iter()
            .enumerate()
            .filter(|&(_, &n)| n > 0)
            .map(|(i, &n)| ((((i % gw) as f32 + 0.5) * b - to.0).hypot(((i / gw) as f32 + 0.5) * b - to.1), n))
            .collect();
        near.sort_by(|a, c| a.0.total_cmp(&c.0));
        let mut sum = 0u32;
        for &(d, n) in &near {
            sum += n;
            if sum as f32 >= want {
                return (d / sc + 1.0, sum);
            }
        }
        (near.last().map(|l| l.0 / sc + 1.0).unwrap_or(0.0), sum)
    }
}
