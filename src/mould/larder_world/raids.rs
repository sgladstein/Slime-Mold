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
//! A raid is the slime spending what it ate: the game keeps a meter of the
//! food the slime eats from its crystals (`Game::time_surges`'s count),
//! and when it holds `RaidRules::food` the slime sends a share of its body
//! at the player's richest holding: the pump drawing the most, else any
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

use super::{Game, Kind, World};

/// The raids' numbers. Food in the bank's units, times in game seconds,
/// distances in dish cells.
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct RaidRules {
    /// Food the slime eats from its crystals to build one raid; 0 is off.
    /// 400: on the Fork, shape C eats about 2-11 food a second over its
    /// first three minutes (Reports/surge-2026-11-15/README.md), so a
    /// raid every half minute to two minutes, sooner the more it holds.
    pub food: f32,
    /// No raid before this many seconds from the landing...
    pub first: f32,
    /// ...nor within this many of the last one.
    pub min: f32,
    /// A raid comes after this many seconds even if the meter is not full
    /// (0: only the meter).
    pub every: f32,
    /// The raid's size, a share of the slime's mass.
    pub size: f32,
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
        RaidRules { food: 400.0, first: 60.0, min: 40.0, every: 150.0, size: 0.15, share: 0.6, lure_r: 40.0, last_stand: false, last_stand_at: 0.25 }
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
    /// The walkers the reach held as it began (the mould grid's count),
    /// and the share of them sent.
    pub held: u32,
    pub share: f32,
    pub last_stand: bool,
}

/// The game's raid state.
#[derive(Debug, Default)]
pub struct Raids {
    /// Food eaten since the last raid.
    pub meter: f32,
    /// Every raid sent, oldest first.
    pub sent: Vec<Raid>,
    pub(super) last_stand_done: bool,
    /// A raid decided and waiting a game second for its way round walls,
    /// which a worker thread builds (`big::raid_field_for`: 7-8 ms on
    /// planet 8, too long for one step by the levelgen thread's hitch rule).
    pending: Option<(Raid, Way)>,
}

/// A raid's way round walls being built.
#[derive(Debug)]
enum Way {
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
            Way::Building(std::thread::spawn(move || crate::mould::big::raid_field_for(&agar, w, h, to)))
        }
        #[cfg(target_arch = "wasm32")]
        {
            Way::Ready(crate::mould::big::raid_field_for(&agar, w, h, to))
        }
    }

    fn finish(self) -> Vec<u32> {
        match self {
            Way::Ready(f) => f,
            #[cfg(not(target_arch = "wasm32"))]
            Way::Building(j) => j.join().unwrap_or_default(),
        }
    }
}

impl<W: World> Game<W> {
    /// Once a game second: start a raid when the meter is full (or the
    /// longest wait has passed), or the last stand.
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
        if r.food <= 0.0 || now < secs(r.first) {
            return;
        }
        let last = self.raids.sent.last().map(|d| d.tick).unwrap_or(0);
        let since = now - last.min(now);
        if since < secs(r.min) {
            return;
        }
        let full = self.raids.meter >= r.food;
        let late = r.every > 0.0 && since >= secs(r.every);
        if !(full || late) {
            return;
        }
        let Some((aim, to)) = self.raid_aim() else { return };
        self.send_raid(aim, to, r.size, r.share, false);
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
        self.raids.meter = 0.0;
        self.raids.pending = Some((Raid { tick: self.world.ticks(), aim, to, reach, held, share, last_stand }, way));
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
