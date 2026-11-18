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
//! **The last stand** (`RaidRules::last_stand`): once, when the slime's
//! mass falls under `last_stand_at` of its peak, every walker it has rushes
//! the thickest knot of soldiers. Built 11-15 for every slime and left off
//! (the owner was "not sure if this is a good idea or not"); since
//! 2026-11-17 it is the Swarmer's trait in a run (`run::does_to_you`, the
//! Enemy lane: the charging slime's way to lose) and off for every other
//! slime.

use super::{Game, Kind, Rng, World};

/// The raids' numbers. Times in game seconds, distances in dish cells.
///
/// Owner 2026-11-16 01:48Z: "slime should send out raiders regardless if it
/// doesn't make a kill box a winning strategy yet. It shouldn't be so
/// consistent as every 400 food. Both the time between raids and the size
/// of the raid should be picked randomly (each raid) from a normal
/// distribution". Until then a raid came every 400 food the slime ate
/// (40-150 s apart): on the Fork turtle every 42 s like a clock.
///
/// **Rare early, often later** (2026-11-18, the Enemy lane; the owner's
/// play notes, 15:56Z: "The raids come too early and too often (or at
/// least too often in the early game). Occasionally a raid can happen early
/// in a session but not every time. and they can happen often later on in
/// a session, but not right at the begining. It is hard to build pumps,
/// walls, turrets because you get raided so quick"). Direction's targets:
/// the first raid usually between 3:00 and 5:00, about one game in five
/// before 2:00; the gap from about 150 s falling to about 45 s by 10:00 on
/// the planet. Until then the first came at 75 +/- 15 s from the world's
/// start and every 60 +/- 20 s after, the same all game (the owner met a
/// raid on his pumps about a minute in, every game). Times now count from
/// the landing (`Game::landed`), so a planet whose slime grows alone first
/// does not raid the squad as it lands.
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct RaidRules {
    /// The time till the next raid is drawn from a normal distribution
    /// with this mean and `every_sd` (0: raids off), never under `min`, at
    /// the landing; both fall to `late` of themselves by `ramp` seconds
    /// after it, in a straight line, and stay there.
    pub every: f32,
    pub every_sd: f32,
    pub late: f32,
    pub ramp: f32,
    /// The first raid's time from the landing, drawn the same way; with
    /// the chance `early` it is drawn from `early_at` and `early_sd`
    /// instead (an early raid now and then, not every game).
    pub first: f32,
    pub first_sd: f32,
    pub early: f32,
    pub early_at: f32,
    pub early_sd: f32,
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
    /// The last stand (off here; on for the Swarmer in a run), and the
    /// share of its peak mass the slime falls under to make it.
    pub last_stand: bool,
    pub last_stand_at: f32,
}

impl Default for RaidRules {
    fn default() -> Self {
        RaidRules {
            // 150 +/- 45 s at the landing, 45 +/- 13.5 s from 10:00.
            every: 150.0,
            every_sd: 45.0,
            late: 0.3,
            ramp: 600.0,
            // 240 +/- 40 s: 3:00 to 5:00 holds 87% of the draws, and
            // one game in five draws from 80 +/- 20 s instead.
            first: 240.0,
            first_sd: 40.0,
            early: 0.2,
            early_at: 80.0,
            early_sd: 20.0,
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

impl RaidRules {
    /// The gap's mean and spread `since` game seconds after the landing:
    /// `every` and `every_sd` falling to `late` of themselves by `ramp`.
    pub fn gap_at(&self, since: f32) -> (f32, f32) {
        let f = if self.ramp > 0.0 { (since / self.ramp).clamp(0.0, 1.0) } else { 1.0 };
        let k = 1.0 + (self.late - 1.0) * f;
        (self.every * k, self.every_sd * k)
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
    food_way: Option<Way>,
    /// The lure's field being built (`time_lure_paths`), and whether the
    /// world holds one to clear when the last lure has gone.
    lure_way: Option<Way>,
    lure_set: bool,
}

impl Raids {
    pub fn new(seed: u64) -> Raids {
        Raids { sent: Vec::new(), next: None, rng: Rng::new(seed ^ 0x7a1d_5eed), last_stand_done: false, pending: None, unit_cells: Vec::new(), unit_way: None, food_way: None, lure_way: None, lure_set: false }
    }

    /// A normal draw (Box-Muller) with mean `m` and spread `sd`.
    pub(super) fn normal(&mut self, m: f32, sd: f32) -> f32 {
        let u = self.rng.f32().max(1e-7);
        let v = self.rng.f32();
        m + sd * (-2.0 * u.ln()).sqrt() * (std::f32::consts::TAU * v).cos()
    }

    /// The first raid's time from the landing, game seconds: early with
    /// the chance `early`, else from `first`; never under `min`.
    pub fn first_raid(&mut self, r: &RaidRules) -> f32 {
        let early = r.early > 0.0 && self.rng.f32() < r.early;
        let (m, sd) = if early { (r.early_at, r.early_sd) } else { (r.first, r.first_sd) };
        self.normal(m, sd).max(r.min)
    }
}

impl Default for Raids {
    fn default() -> Self {
        Raids::new(0)
    }
}

/// **A lure with no building** (2026-11-18, the Enemy lane, for Squad and
/// arsenal: what a thrown canister or a lure dart leaves): the slime's
/// lure field (`Game::time_lure_paths`) reaches it exactly as it reaches a
/// standing Lure, until its tick. Push one to `Game::lure_marks`; the
/// field drops it when it has run out. It draws no scent and takes no
/// raid: it is the field's source and nothing else.
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct LureMark {
    /// Where: world units (fine cells), as a building's place.
    pub at: (f32, f32),
    /// The tick (`World::ticks`) it ends at.
    pub until: u64,
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

    /// The lure's field: the walking distance to the nearest of `lures`
    /// (`round_walk`, so a lure in a closed block starts from the open
    /// block nearest it), kept out to `reach` fine cells of walking and
    /// `u32::MAX` past it. Off the frame as `start`'s.
    fn start_lures(ground: Option<(Vec<bool>, usize, usize)>, lures: Vec<u32>, reach: u32) -> Way {
        let Some((agar, w, h)) = ground else { return Way::Ready(Vec::new()) };
        let build = move || round_walk(&agar, w, h, &lures, reach);
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

    fn finish(self) -> Vec<u32> {
        match self {
            Way::Ready(f) => f,
            #[cfg(not(target_arch = "wasm32"))]
            Way::Building(j) => j.join().unwrap_or_default(),
        }
    }
}

/// **The lure's walking distance, round on open ground** (2026-11-18, the
/// Enemy lane). The raid's search (`big::units_field_for`) steps 2 a block
/// and 3 a diagonal, so on open ground its distance is an octagon: the
/// Lure's rings drew as a target over the meadow (the Look and interface
/// lane's note, 11-18) and the pull's shape was the same octagon. This one
/// adds the knight's moves, at the chamfer weights 5, 7 and 11 a block
/// (within 2% of the straight line at every angle), and reads the same
/// units as the raid's search: fine cells of walking, rounded. A diagonal
/// passes only where both sides are open, and a knight's move only where
/// the two blocks it crosses are, so it goes round rock as the raid's
/// does. A lure in a closed block starts from the nearest open block
/// within `big::UNITS_REACH`, as there. Its own search and not the raid's,
/// since `big.rs` (where the raid's lives, under the charges and the far
/// smell too) is frozen until Milestone 2 (the owner's card, 11-17 19:21Z).
/// The search stops past `limit` fine cells of walking (`u32::MAX` past
/// it), so a lure's field costs its reach and not the map.
pub(super) fn round_walk(agar: &[bool], w: usize, h: usize, from: &[u32], limit: u32) -> Vec<u32> {
    use crate::mould::big::{RAID_BLOCK, UNITS_REACH};
    let b = RAID_BLOCK;
    let (bw, bh) = (w.div_ceil(b), h.div_ceil(b));
    // The open blocks: every fine cell agar.
    let mut open = vec![true; bw * bh];
    for (i, &a) in agar.iter().enumerate() {
        if !a {
            open[((i / w) / b) * bw + (i % w) / b] = false;
        }
    }
    let r = UNITS_REACH as i64;
    let mut starts: Vec<usize> = from
        .iter()
        .filter_map(|&u| {
            let (tx, ty) = ((((u & 0xFFFF) as usize).min(w - 1) / b) as i64, (((u >> 16) as usize).min(h - 1) / b) as i64);
            let mut best: Option<(i64, usize)> = None;
            for dy in -r..=r {
                for dx in -r..=r {
                    let (x, y) = (tx + dx, ty + dy);
                    if x < 0 || y < 0 || x >= bw as i64 || y >= bh as i64 {
                        continue;
                    }
                    let q = y as usize * bw + x as usize;
                    if open[q] && best.is_none_or(|p| dx * dx + dy * dy < p.0) {
                        best = Some((dx * dx + dy * dy, q));
                    }
                }
            }
            best.map(|p| p.1)
        })
        .collect();
    starts.sort_unstable();
    starts.dedup();
    // On a grid two closed blocks wider on every side, so no move needs a
    // bounds check.
    let pw = bw + 4;
    let mut pad = vec![false; pw * (bh + 4)];
    for y in 0..bh {
        pad[(y + 2) * pw + 2..(y + 2) * pw + 2 + bw].copy_from_slice(&open[y * bw..(y + 1) * bw]);
    }
    let mut pd = vec![u32::MAX; pad.len()];
    let p = pw as isize;
    // (offset, cost, the two blocks it crosses; a straight step crosses
    // none, so its own block twice).
    let mut moves: Vec<(isize, u32, isize, isize)> = Vec::with_capacity(16);
    for (dx, dy) in [(1isize, 0isize), (0, 1), (-1, 0), (0, -1)] {
        moves.push((dy * p + dx, 5, dy * p + dx, dy * p + dx));
    }
    for (dx, dy) in [(1isize, 1isize), (1, -1), (-1, 1), (-1, -1)] {
        moves.push((dy * p + dx, 7, dx, dy * p));
    }
    for (dx, dy) in [(2isize, 1isize), (2, -1), (-2, 1), (-2, -1), (1, 2), (-1, 2), (1, -2), (-1, -2)] {
        // A knight's move crosses the straight block and the diagonal one
        // on its way: (2, 1) crosses (1, 0) and (1, 1).
        let (sx, sy) = if dx.abs() == 2 { (dx / 2, 0) } else { (0, dy / 2) };
        moves.push((dy * p + dx, 11, sy * p + sx, dy.signum() * p + dx.signum()));
    }
    // Moves cost 5 to 11, so twelve buckets in a ring hold every distance
    // still to settle.
    let mut ring: [Vec<u32>; 12] = Default::default();
    let mut left = 0usize;
    for &q in &starts {
        let s = (q / bw + 2) * pw + q % bw + 2;
        pd[s] = 0;
        ring[0].push(s as u32);
        left += 1;
    }
    let mut at = 0u32;
    let last = limit.saturating_mul(5) / b as u32 + 5;
    while left > 0 && at <= last {
        let slot = (at % 12) as usize;
        let now = std::mem::take(&mut ring[slot]);
        left -= now.len();
        for &q in &now {
            if pd[q as usize] != at {
                continue;
            }
            let q = q as isize;
            for &(o, c, s1, s2) in &moves {
                let j = (q + o) as usize;
                if !pad[j] || !pad[(q + s1) as usize] || !pad[(q + s2) as usize] {
                    continue;
                }
                let nd = at + c;
                if nd < pd[j] {
                    pd[j] = nd;
                    ring[(nd % 12) as usize].push(j as u32);
                    left += 1;
                }
            }
        }
        ring[slot] = now;
        ring[slot].clear();
        at += 1;
    }
    // Fifths of a block to fine cells (a block is `b` = 2 fine cells),
    // rounded.
    let mut d = vec![u32::MAX; bw * bh];
    for y in 0..bh {
        for x in 0..bw {
            let v = pd[(y + 2) * pw + x + 2];
            let f = if v == u32::MAX { u32::MAX } else { (v * b as u32 + 2) / 5 };
            if f <= limit {
                d[y * bw + x] = f;
            }
        }
    }
    d
}

impl<W: World> Game<W> {
    /// The ground the slime's ways are built on (a raid's, the charge
    /// paths', the far smell's and the lure's): the world's
    /// (`World::raid_ground`) less the bulwark's cells (2026-11-18). A
    /// walker cannot step into a cell a set walker holds, so a way through
    /// the slime's own wall led a charge to press on it from behind; with
    /// the walls taken out the ways go through a fort's gates and round a
    /// wall the shots made.
    fn slime_ground(&self) -> Option<(Vec<bool>, usize, usize)> {
        let (mut ground, w, h) = self.world.raid_ground()?;
        if self.bulwark.len() == ground.len() {
            for (g, &wall) in ground.iter_mut().zip(&self.bulwark) {
                *g &= !wall;
            }
        }
        Some((ground, w, h))
    }

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
        self.raids.unit_way = Some(Way::start_units(self.slime_ground(), units));
    }

    /// **The far smell** (`BigParams::far_smell`, the simulation-use plan's
    /// S2, 2026-11-17): every `FOOD_PATHS_EVERY` game seconds, the field
    /// started last time is set (`World::set_food_field`) and a new one is
    /// started from the untouched crystals, off the frame and always one
    /// period late, joining the worker if it is slow, as the units' way.
    pub(super) fn time_food_paths(&mut self) {
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
        self.raids.food_way = Some(Way::start_units(self.slime_ground(), fresh));
    }

    /// **The lure's field** (`gpu::Settings::lure_pull`, 2026-11-18, the
    /// Enemy lane; the lab's steering prototype made a rule,
    /// `Reports/lab-steering-2026-11-17`). The owner, 11-15 20:56Z: "even
    /// if i build lures, only a thin trickle moves into my kill box"; a
    /// lure's scent reaches 6 cells. Once a game second the field started
    /// a second ago is set (`World::set_lure_field`) and a new one is
    /// started from every Lure that is whole and on (`Kind::Lure`) and
    /// every mark (`LureMark`): the walking distance to the nearest, out
    /// to `Rules::lure_reach` dish cells. Calm slime inside it leans down
    /// it (engine.wgsl, the lure block in `sense`). Walking distance goes
    /// round rock and walls, so a wall with a gap funnels what a lure
    /// draws. Always a second late, so the game is the same however fast
    /// the thread was, as the units' way; when the last lure goes, the
    /// field is cleared the second after.
    ///
    /// Only a world that reads it builds it (`World::lure_pull`: the GPU
    /// world with the key over 0, which a run's planets set). There is no
    /// CPU twin (the owner's card, 11-17 19:21Z). No lure, no thread and
    /// no write.
    pub(super) fn time_lure_paths(&mut self) {
        if !self.world.lure_pull() {
            return;
        }
        if let Some(way) = self.raids.lure_way.take() {
            self.world.set_lure_field(way.finish());
        } else if self.raids.lure_set {
            self.world.set_lure_field(Vec::new());
            self.raids.lure_set = false;
        }
        let now = self.world.ticks();
        self.lure_marks.retain(|m| m.until > now);
        let (w, h) = self.world.size();
        let cell = |at: (f32, f32)| (at.0.clamp(0.0, w as f32 - 1.0) as u32) | ((at.1.clamp(0.0, h as f32 - 1.0) as u32) << 16);
        let mut lures: Vec<u32> = self.buildings.iter().filter(|b| b.kind == Kind::Lure && b.up() && b.on).map(|b| cell(b.at)).collect();
        lures.extend(self.lure_marks.iter().map(|m| cell(m.at)));
        if lures.is_empty() {
            return;
        }
        let reach = (self.rules.lure_reach.max(0.0) * self.world.scale()) as u32;
        self.raids.lure_way = Some(Way::start_lures(self.slime_ground(), lures, reach));
        self.raids.lure_set = true;
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
                let t = self.landed + secs(self.raids.first_raid(&r));
                self.raids.next = Some(t);
                t
            }
        };
        if now < next {
            return;
        }
        let Some((aim, to)) = self.raid_aim() else { return };
        let size = self.raids.normal(r.size, r.size_sd).clamp(r.size_min, r.size_max);
        let since = now.saturating_sub(self.landed) as f32 / self.rules.tps as f32;
        let (m, sd) = r.gap_at(since);
        self.raids.next = Some(now + gap(&mut self.raids, m, sd));
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
        let way = Way::start(self.slime_ground(), super::raid_cell(to, self.world.size()));
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

#[cfg(test)]
mod tests {
    use super::{round_walk, RaidRules, Raids};
    use crate::mould::big::{units_field_for, RAID_BLOCK};

    /// **Raids are rare early and often later** (2026-11-18, the owner's
    /// play notes and Direction's targets): over 4,000 games the first
    /// raid comes before 2:00 in about one in five, and the rest mostly
    /// between 3:00 and 5:00; the gap's mean falls from 150 s at the
    /// landing to 45 s by 10:00 and stays there.
    #[test]
    fn raids_are_rare_early_and_often_later() {
        let r = RaidRules::default();
        let n = 4000;
        let firsts: Vec<f32> = (0..n).map(|seed| Raids::new(seed).first_raid(&r)).collect();
        let early = firsts.iter().filter(|&&t| t < 120.0).count() as f32 / n as f32;
        let late: Vec<f32> = firsts.iter().copied().filter(|&t| t >= 120.0).collect();
        let usual = late.iter().filter(|&&t| (180.0..=300.0).contains(&t)).count() as f32 / late.len() as f32;
        assert!((0.17..0.23).contains(&early), "one game in five raided before 2:00: {early}");
        assert!(usual > 0.8, "the rest mostly between 3:00 and 5:00: {usual}");
        assert!(firsts.iter().all(|&t| t >= r.min), "never under the floor");
        assert_eq!(r.gap_at(0.0), (150.0, 45.0));
        let (m, sd) = r.gap_at(600.0);
        assert!((m - 45.0).abs() < 1e-3 && (sd - 13.5).abs() < 1e-3, "{m} {sd} at 10:00");
        assert_eq!(r.gap_at(900.0), r.gap_at(600.0), "and stays there");
        assert!((1..60).all(|k| r.gap_at(k as f32 * 10.0).0 < r.gap_at((k - 1) as f32 * 10.0).0), "falls all the way");
        // No ramp: the gap stays `every`; no early chance: always `first`.
        let flat = RaidRules { late: 1.0, early: 0.0, ..r };
        assert_eq!(flat.gap_at(900.0), (150.0, 45.0));
        let mean = (0..2000).map(|seed| Raids::new(seed).first_raid(&flat)).sum::<f32>() / 2000.0;
        assert!((236.0..244.0).contains(&mean), "no early ones: mean {mean}");
    }

    /// A block 70 fine cells right of the test's lure, past a reach of 60.
    fn behind_cut(bw: usize) -> usize {
        (100 / RAID_BLOCK) * bw + 170 / RAID_BLOCK
    }

    /// **The lure's walking distance is round on open ground and walks
    /// round rock** (2026-11-18). From one lure on an open map, at 40
    /// blocks in sixteen directions, the distance is within 3% of the
    /// straight line, where the raid's search is up to 6% out (its
    /// octagon). Through a wall with one gap, the far side is reached only
    /// by the gap, never through the wall.
    #[test]
    fn the_lures_walk_is_round_and_goes_round_rock() {
        let (w, h) = (200usize, 200usize);
        let b = RAID_BLOCK;
        let bw = w.div_ceil(b);
        let at = (100u32, 100u32);
        let open = vec![true; w * h];
        let round = round_walk(&open, w, h, &[at.0 | at.1 << 16], u32::MAX);
        let raid = units_field_for(&open, w, h, &[at.0 | at.1 << 16]);
        let (cx, cy) = (at.0 as f32 / b as f32, at.1 as f32 / b as f32);
        let (mut worst_round, mut worst_raid) = (0.0f32, 0.0f32);
        for k in 0..16 {
            let a = k as f32 * std::f32::consts::TAU / 16.0;
            let (x, y) = ((cx + 40.0 * a.cos()).round() as usize, (cy + 40.0 * a.sin()).round() as usize);
            let line = ((x as f32 - cx).hypot(y as f32 - cy)) * b as f32;
            worst_round = worst_round.max((round[y * bw + x] as f32 / line - 1.0).abs());
            worst_raid = worst_raid.max((raid[y * bw + x] as f32 / line - 1.0).abs());
        }
        assert!(worst_round < 0.03, "round within 3% of the line: {worst_round}");
        assert!(worst_raid > worst_round, "rounder than the raid's octagon ({worst_raid} against {worst_round})");
        assert_eq!(round[(cy as usize) * bw + cx as usize], 0, "0 at the lure");
        // A wall across the map at x = 120 fine cells, 4 thick, with a gap
        // at y 20 to 31: the far side is reached round by the gap.
        let mut walled = open.clone();
        for y in 0..h {
            if (20..32).contains(&y) {
                continue;
            }
            for x in 120..124 {
                walled[y * w + x] = false;
            }
        }
        let d = round_walk(&walled, w, h, &[at.0 | at.1 << 16], u32::MAX);
        // Cut at a reach, the search goes no further.
        let near = round_walk(&walled, w, h, &[at.0 | at.1 << 16], 60);
        assert!(near.iter().all(|&v| v == u32::MAX || v <= 60) && near[behind_cut(bw)] == u32::MAX && near.iter().filter(|&&v| v != u32::MAX).count() > 100, "the search stops at its limit");
        let behind = (100 / b) * bw + 140 / b;
        // The shortest way: to the gap's near corner, through, and back.
        let by_gap = 20.0f32.hypot(69.0) + 4.0 + 16.0f32.hypot(69.0);
        let got = d[behind] as f32;
        assert!(got >= 0.97 * by_gap && got <= 1.1 * by_gap, "behind the wall by the gap: {got} against {by_gap}");
        assert!((120 / b..124 / b).all(|x| d[(100 / b) * bw + x] == u32::MAX), "no distance inside the wall");
    }
}
