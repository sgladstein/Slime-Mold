//! **Starfall and boss cores** (2026-11-20, the thread "Brainstorm how
//! battles earn shop money"). The owner, 17:05Z, after a first round of
//! ideas that paid for what the player already does (stars, food banked,
//! crystals held): "Many of these ideas could easily be gamed or push you
//! to play the game in a less interesting way ... A new resource that falls
//! from the sky or appears somehow and only last for a bit so you have to
//! rush to get it, this adds extra challange". 19:06Z, picking from the
//! second round: "I like starfall with boss cores. The amount of star fall
//! should have a minimum but can sometimes happen a lot. Random selection
//! from distribution as normal".
//!
//! **Starfall.** Ore falls on the planet at least `StarRules::least` times,
//! and each further fall comes with chance `tail` (a geometric tail: most
//! planets see three or four, about one in ten seven or more, up to
//! `most`), at times drawn across `first` to `last` seconds after the
//! landing (on a run's planet, to half its par if that is sooner). Its shadow shows where for `warn` seconds; it lands as a blast
//! (a kill, a crater, and a hurt to any soldier under it). On the ground
//! it is rich food: it smells as a soldier does, `smell` times over, and on
//! the GPU world it is a lure (`LureMark`) for its life, so the slime
//! heads for it. Soldiers by it drain its `worth` into shop points; walkers
//! on it eat it, and once they have eaten half, the slime's next surge is
//! fed (`Game::surge_meter`). It cools to nothing `life` seconds after it
//! lands.
//!
//! **Boss cores.** A mini-boss broken (`BossEnd::Broken`) or a fruit shot
//! down (`FruitEnd::Broken`) drops its core where it was, worth more the
//! stronger the boss: it lasts `core_life` seconds, and walkers on it soak
//! it back. Planet 1 has no mini-bosses, so its cores are fruits' and
//! raids'. A beaten raid drops one too, walkers stream back to every core,
//! a boss whose core they take back re-forms, and a planet's cores hold no
//! more than `core_pot` in all (`cores.rs`).
//!
//! Why these cannot be farmed (the owner's 13:49Z objection to paying for
//! crystals held: "I just kind of wall it in and then I can easily take
//! over all of the crystals but one and then just hold them for as long as
//! I want to rack up money"): a planet's falls are drawn when it starts and
//! stop at `last`, and cores come only from bosses and fruits, which a
//! beaten slime does not make. Waiting never makes more.
//!
//! The run turns it on for every planet (`StarRules::for_depth`); the
//! campaign and the sandbox keep it off. It works on both engines; the
//! lure's pull is the GPU world's only.

use super::*;
use crate::mould::squad::{Burst, BurstRules};

/// Starfall's and the cores' numbers: times in game seconds, distances in
/// dish cells, worth in shop points. First numbers; Direction tunes them.
#[derive(Clone, Copy, Debug, PartialEq)]
#[derive(serde::Serialize, serde::Deserialize)]
pub struct StarRules {
    pub on: bool,
    /// Falls a planet: at least `least`, then each more with chance
    /// `tail`, at most `most`.
    pub least: u32,
    pub tail: f32,
    pub most: u32,
    /// The falls' times, drawn evenly between these (after the landing),
    /// at least `gap` apart.
    pub first: f32,
    pub last: f32,
    /// On a planet with a speed star (`Rules::par`), the window ends by
    /// `par_share` of the par if that is sooner than `last` (0: `last`
    /// alone). Direction, 2026-11-20: the bug check found the falls drawn
    /// across 0:50 to 10:00 while the owner's planet 1 took a median 4:05
    /// (balance review), so a short planet saw one fall of its three or
    /// four. Half the par is 4:00 on planet 1 and 5:00 on planets 2 and 3,
    /// so a planet played at his pace sees them all. Not a shorter `last`
    /// for every planet: deeper planets' par grows with their area.
    pub par_share: f32,
    /// **The teaching fall** (Direction, 2026-11-21, after the owner's
    /// 11-20 23:17Z "it isn't clear how to collect"): with `teach` over 0
    /// the first fall's shadow shows at `teach` seconds (it lands `warn`
    /// later: 0:50 and 1:00 on planet 1) and it lands on open ground
    /// within `teach_far` of the nearest soldier (and still `near` from
    /// every one), where the fewest walkers are within `slime_r`, so it
    /// shows how a pickup pays before the slime eats it. The other falls
    /// keep the window and the slime. On planet 1 only
    /// (`StarRules::for_depth`): six planet 1 games with the strategy bot
    /// chasing drops took 36 points of ore and cores and the slime ate 851
    /// (`Reports/boss-cores-2026-11-20/`), since a fall lands by slime.
    pub teach: f32,
    pub teach_far: f32,
    /// ... and no more than this many dish cells above or below the
    /// soldier nearest it (11-24, the polish review's finding 13: planet
    /// 1's teaching fall landed under the bottom bar, the screen's
    /// shortest side). Beside the squad, not below it: about 120 pixels
    /// at zoom 6 and 240 at zoom 12, clear of the bars with the camera on
    /// the squad, without the game reading the view. Where nothing fits,
    /// the band is dropped before the fall goes ordinary.
    pub teach_dy: f32,
    pub gap: f32,
    /// Seconds its shadow shows before it lands.
    pub warn: f32,
    /// The impact: walkers killed within `kill_r`, crater over `crater_r`
    /// for `crater_secs`, `hurt` health off a soldier within `kill_r`.
    pub kill_r: f32,
    pub crater_r: f32,
    pub crater_secs: f32,
    pub hurt: f32,
    /// Where it may land: at least `near` from every live soldier, and
    /// within `far` of the nearest; slime within `slime_r` preferred.
    pub near: f32,
    pub far: f32,
    pub slime_r: f32,
    /// The ore: its radius, seconds it lasts once down, and its worth.
    pub ore_r: f32,
    pub life: f32,
    pub worth: f32,
    /// Points a second each soldier within `reach` of its rim drains, at
    /// the planet's harvest (Quick hands drains faster).
    pub take: f32,
    pub reach: f32,
    /// Points a second each walker on it eats, times the slime's mass
    /// (`Rules::mass`, as the larder's theft).
    pub eat: f32,
    /// Its smell, times a soldier's, and its reach.
    pub smell: f32,
    pub smell_r: f32,
    /// A core: worth `core_worth` plus `core_grade` times a mini-boss's
    /// grade (0 to 1); `core_life` seconds; radius `core_r`.
    pub core_worth: f32,
    pub core_grade: f32,
    pub core_life: f32,
    pub core_r: f32,
    /// **The boss re-forms** (`cores.rs`): once walkers have eaten
    /// `core_reform` of a mini-boss's core, and more than the squad took,
    /// the boss forms again on it, as it was, carrying what the core had
    /// left. `core_pot`: the most the planet's cores may hold in all
    /// (fresh ones; a re-formed boss's core is its old one again).
    pub core_reform: f32,
    pub core_pot: f32,
    /// Points a second each walker on a core eats, times the slime's mass
    /// (`eat` is ore's): at ore's 1, a core landing in thick slime was half
    /// eaten in 2 s and a wanderer re-formed 3 s after it broke (planet 2,
    /// run 1795106014).
    pub core_eat: f32,
    /// **A core runs home** (`cores.rs`): it slides at `core_drift` dish
    /// cells a second to the thickest slime within `core_drift_r` of where
    /// it dropped, so the squad must reach into the slime for it.
    pub core_drift: f32,
    pub core_drift_r: f32,
    /// **A beaten raid's core** (`cores.rs`): a march shot apart on its
    /// way, or one that met the squad and lost `raid_beat` of its members
    /// to the guns within `raid_window` seconds, drops a core worth
    /// `core_worth` plus `core_grade` times its members over `raid_full`
    /// (to 1), where it was. Walkers that eat half of it feed the slime's
    /// next surge.
    pub raid_beat: f32,
    pub raid_window: f32,
    pub raid_full: f32,
}

impl Default for StarRules {
    fn default() -> Self {
        StarRules {
            on: false,
            least: 3,
            tail: 0.55,
            most: 12,
            first: 50.0,
            last: 600.0,
            par_share: 0.5,
            teach: 0.0,
            teach_far: 30.0,
            teach_dy: 10.0,
            gap: 25.0,
            warn: 10.0,
            kill_r: 4.0,
            crater_r: 6.0,
            crater_secs: 20.0,
            hurt: 60.0,
            near: 14.0,
            far: 60.0,
            slime_r: 18.0,
            ore_r: 2.5,
            life: 60.0,
            worth: 40.0,
            take: 2.5,
            reach: 2.0,
            eat: 1.0,
            smell: 3.0,
            smell_r: 8.0,
            core_worth: 40.0,
            core_grade: 40.0,
            core_life: 15.0,
            core_r: 2.0,
            core_reform: 0.5,
            core_pot: 200.0,
            core_eat: 0.3,
            core_drift: 8.0,
            core_drift_r: 60.0,
            raid_beat: 0.5,
            raid_window: 20.0,
            raid_full: 400.0,
        }
    }
}

impl StarRules {
    /// A run's planet: on, the ore worth 30 points and 10 more a planet.
    pub fn for_depth(depth: u32) -> StarRules {
        let teach = if depth <= 1 { 50.0 } else { 0.0 };
        StarRules { on: true, worth: 30.0 + 10.0 * depth.max(1) as f32, core_pot: 120.0 + 40.0 * depth.max(1) as f32, teach, ..StarRules::default() }
    }
}

/// What lies on the ground.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[derive(serde::Serialize, serde::Deserialize)]
pub enum DropKind {
    /// Ore from the sky.
    Ore,
    /// A broken mini-boss's, fruit's or raid's core.
    Core,
}

/// What dropped a core (`cores.rs`).
#[derive(Clone, Copy, Debug, PartialEq)]
#[derive(serde::Serialize, serde::Deserialize)]
pub enum CoreFrom {
    /// Ore from the sky.
    Sky,
    /// A mini-boss broken: its kind and grade, which it re-forms with.
    Boss { kind: BossKind, grade: f32 },
    Fruit,
    /// A beaten raid.
    Raid,
}

/// One fall or core.
#[derive(Clone, Copy, Debug, PartialEq)]
#[derive(serde::Serialize, serde::Deserialize)]
pub struct Drop {
    pub kind: DropKind,
    pub from: CoreFrom,
    /// Where a core is sliding to (fine cells; `cores.rs`), once chosen.
    pub home: Option<(f32, f32)>,
    /// Fine cells.
    pub at: (f32, f32),
    /// The tick it lands (a core's: the tick it dropped) and the tick it is
    /// gone.
    pub lands: u64,
    pub ends: u64,
    /// The tick its shadow showed.
    pub warned: u64,
    pub down: bool,
    /// Radius, fine cells.
    pub r: f32,
    pub worth: f32,
    pub left: f32,
    /// What soldiers took and walkers ate from it.
    pub taken: f32,
    pub eaten: f32,
    /// Soldiers and walkers on it at the last frame.
    pub takers: u32,
    pub eaters: u32,
    fed: bool,
    /// Where its lure mark is (`Game::lure_marks`), once it has one: the
    /// mark goes when it does (`Game::drop_gone`).
    pub(super) lure: Option<(f32, f32)>,
}

impl Drop {
    /// Ore from the sky, warned of at `now`, landing at `lands` (the
    /// slime viewer drops one where it is clicked, `Game::force_ore`); its
    /// lure mark is set when it lands.
    pub fn ore(at: (f32, f32), worth: f32, now: u64, lands: u64, ends: u64, r: f32) -> Drop {
        Drop { kind: DropKind::Ore, from: CoreFrom::Sky, home: None, at, lands, ends, warned: now, down: false, r, worth, left: worth, taken: 0.0, eaten: 0.0, takers: 0, eaters: 0, fed: false, lure: None }
    }

    /// A core dropped at tick `now` (`cores.rs`), gone at `ends`, `r` fine
    /// cells round.
    pub fn core(at: (f32, f32), worth: f32, from: CoreFrom, now: u64, ends: u64, r: f32) -> Drop {
        // A raid's core feeds the next surge once half is eaten; a boss's
        // re-forms it instead (`Game::time_cores`).
        Drop { kind: DropKind::Core, from, home: None, at, lands: now, ends, warned: now, down: true, r, worth, left: worth, taken: 0.0, eaten: 0.0, takers: 0, eaters: 0, fed: from != CoreFrom::Raid, lure: None }
    }

    /// Its warmth, 1 as it lands to 0 when gone (a shadow's, 0 to 1 as it
    /// comes).
    pub fn warmth(&self, now: u64) -> f32 {
        if !self.down {
            return (now.saturating_sub(self.warned) as f32 / self.lands.saturating_sub(self.warned).max(1) as f32).clamp(0.0, 1.0);
        }
        (self.ends.saturating_sub(now) as f32 / self.ends.saturating_sub(self.lands).max(1) as f32).clamp(0.0, 1.0)
    }
}

/// A drop the squad emptied (`Starfall::cashed`): the moment its points
/// came in, for the window's "+N" and the coin chime (Boss cores thread,
/// 2026-11-21: make the pickup feel good).
#[derive(Clone, Copy, Debug, PartialEq)]
#[derive(serde::Serialize, serde::Deserialize)]
pub struct Cashed {
    pub at: (f32, f32),
    pub kind: DropKind,
    /// What the squad took from it in all.
    pub taken: f32,
    pub tick: u64,
}

/// Cashed drops kept (`Starfall::cashed`): more than ever empty in a
/// second or two, which is as long as the window shows one.
pub const CASHED_KEPT: usize = 8;

/// Dish cells kept clear round a crystal's rim by a drop (`DropGround`),
/// on top of the drop's own radius: a gem beside a crystal, never on it.
pub const CRYSTAL_CLEAR: f32 = 1.5;

/// Dish cells of open ground kept past a drop's rim (`DropGround::fits`):
/// the first film (planet 1, run 1795106014) put a fall's disc against a
/// rock ridge, reachable but half its dial on the rock.
pub const ROCK_CLEAR: f32 = 1.0;

/// The game's starfall state.
#[derive(Debug)]
#[derive(serde::Serialize, serde::Deserialize)]
pub struct Starfall {
    /// The planet's falls not yet begun (ticks their shadows show), soonest
    /// last; None until drawn at the first second.
    pub plan: Option<Vec<u64>>,
    /// Falls drawn for the planet.
    pub falls: u32,
    /// What lies on the ground (and what is coming down).
    pub live: Vec<Drop>,
    /// Shop points the squad has taken this planet, and from what.
    pub points: f32,
    pub ore_taken: f32,
    pub core_taken: f32,
    /// Points the slime ate back, landed falls and cores dropped.
    pub eaten: f32,
    pub landed: u32,
    pub cores: u32,
    /// The latest drops the squad emptied, oldest first (at most
    /// `CASHED_KEPT`), and how many it has emptied this planet.
    pub cashed: Vec<Cashed>,
    pub cashed_n: u32,
    /// Falls begun (their shadows shown): the first is the teaching
    /// fall when `StarRules::teach` is set.
    begun: u32,
    rng: Rng,
    /// Mini-bosses ended already looked at, and the fruit last seen (its
    /// centre) with the last fruit end already looked at.
    bosses_seen: usize,
    fruit_at: Option<(f32, f32)>,
    fruit_seen: Option<u64>,
}

impl Starfall {
    pub fn new(seed: u64) -> Starfall {
        Starfall { plan: None, falls: 0, live: Vec::new(), points: 0.0, ore_taken: 0.0, core_taken: 0.0, eaten: 0.0, landed: 0, cores: 0, cashed: Vec::new(), cashed_n: 0, begun: 0, rng: Rng::new(seed ^ 0x57a2_fa11), bosses_seen: 0, fruit_at: None, fruit_seen: None }
    }

    /// The whole shop points taken this planet.
    pub fn earned(&self) -> u32 {
        self.points.floor().max(0.0) as u32
    }
}

/// How many fall: `least`, then one more with chance `tail` each time, to
/// `most` (`u` the draws in turn, each 0 to 1).
pub fn fall_count(r: &StarRules, mut u: impl FnMut() -> f32) -> u32 {
    let mut n = r.least;
    while n < r.most.max(r.least) && u() < r.tail {
        n += 1;
    }
    n
}

/// Where a drop may lie (2026-11-22, owner's play: Starfall and mini-boss
/// cores "can both fall or move ... on top of crystals or rock/walls and
/// become inaccessible"): the cells a soldier can stand on **and walk to
/// from where the squad is now**, with every crystal's disc and a rim
/// round it taken out. Built once per fall, core or core's home (a flood
/// over the walking grid, as a walking map is; never per frame, not yet timed on the GTX 1070).
///
/// Before, a fall checked only that its middle cell was `World::open`: a
/// crystal is open agar to the world (only solid to soldiers), and the
/// fall aimed for thick slime, which sits on the crystals, so ore came
/// down on them; its disc could hang over rock, and a pocket of open
/// ground walled in by rock passed. A core dropped at its boss's centre
/// (which can be over rock or a crystal the boss wrapped) and slid to the
/// thickest slime near it, again on a crystal as often as not.
pub(super) struct DropGround {
    w: usize,
    h: usize,
    /// Standable and reached from a live soldier (every standable cell when
    /// no soldier lives).
    ok: Vec<bool>,
    /// Every crystal's centre and the radius kept clear round it.
    crystals: Vec<((f32, f32), f32)>,
    /// Ground kept open past a drop's rim (fine cells, `ROCK_CLEAR`).
    pad: f32,
}

impl DropGround {
    fn cell(&self, p: (f32, f32)) -> Option<usize> {
        (p.0 >= 0.0 && p.1 >= 0.0 && (p.0 as usize) < self.w && (p.1 as usize) < self.h).then(|| p.1 as usize * self.w + p.0 as usize)
    }

    /// A soldier can walk to `p`.
    pub(super) fn walkable(&self, p: (f32, f32)) -> bool {
        self.cell(p).is_some_and(|i| self.ok[i])
    }

    /// A drop of radius `r` at `p` lies on open, reachable ground: its
    /// middle and twelve points a `pad` past its rim reachable, and clear
    /// of every crystal.
    pub(super) fn fits(&self, p: (f32, f32), r: f32) -> bool {
        if !self.walkable(p) || self.crystals.iter().any(|&(c, cr)| (c.0 - p.0).hypot(c.1 - p.1) < cr + r) {
            return false;
        }
        let rim = r + self.pad;
        (0..12).all(|k| {
            let a = k as f32 * std::f32::consts::TAU / 12.0;
            self.walkable((p.0 + rim * a.cos(), p.1 + rim * a.sin()))
        })
    }

    /// The straight slide from `a` to `b` stays on fitting ground (sampled
    /// every `step` fine cells).
    pub(super) fn clear_line(&self, a: (f32, f32), b: (f32, f32), r: f32, step: f32) -> bool {
        let n = ((b.0 - a.0).hypot(b.1 - a.1) / step.max(0.5)).ceil() as usize;
        (1..=n).all(|k| {
            let t = k as f32 / n as f32;
            self.fits((a.0 + (b.0 - a.0) * t, a.1 + (b.1 - a.1) * t), r)
        })
    }

    /// The nearest point to `p` where a drop of radius `r` fits, searched
    /// in rings `step` apart out to `reach`; failing that the nearest
    /// reachable cell; `None` with neither.
    pub(super) fn snap(&self, p: (f32, f32), r: f32, step: f32, reach: f32) -> Option<(f32, f32)> {
        if self.fits(p, r) {
            return Some(p);
        }
        let step = step.max(0.5);
        let mut loose = None;
        let rings = (reach / step).ceil() as usize;
        for k in 1..=rings {
            let d = k as f32 * step;
            let n = ((std::f32::consts::TAU * d / step).ceil() as usize).max(8);
            for j in 0..n {
                let a = std::f32::consts::TAU * j as f32 / n as f32;
                let q = (p.0 + d * a.cos(), p.1 + d * a.sin());
                if self.fits(q, r) {
                    return Some(q);
                }
                if loose.is_none() && self.walkable(q) && !self.crystals.iter().any(|&(c, cr)| (c.0 - q.0).hypot(c.1 - q.1) < cr) {
                    loose = Some(q);
                }
            }
        }
        loose
    }
}

impl<W: World> Game<W> {
    /// Once a game second: draw the planet's falls, bring down the next,
    /// land those due, drop cores for bosses and fruits broken, and clear
    /// what has gone.
    pub(super) fn time_starfall(&mut self) {
        let r = self.rules.star;
        if !r.on {
            return;
        }
        let tps = self.rules.tps as f32;
        let now = self.world.ticks();
        let secs = |s: f32| (s.max(0.0) * tps).round() as u64;
        if self.starfall.plan.is_none() {
            let rng = &mut self.starfall.rng;
            let n = fall_count(&r, || rng.f32());
            let last = if self.rules.par > 0.0 && r.par_share > 0.0 { r.last.min(r.par_share * self.rules.par) } else { r.last };
            let mut at: Vec<f32> = (0..n).map(|_| r.first + (last - r.first).max(0.0) * rng.f32()).collect();
            at.sort_by(f32::total_cmp);
            // The teaching fall is the first, at its own time; the rest
            // keep their draws (pushed back by `gap` only if it crowds them).
            if r.teach > 0.0 {
                if let Some(t) = at.first_mut() {
                    *t = r.teach;
                }
            }
            for i in 1..at.len() {
                at[i] = at[i].max(at[i - 1] + r.gap);
            }
            self.starfall.falls = n;
            self.starfall.plan = Some(at.iter().rev().map(|&t| self.landed + secs(t)).collect());
        }
        let sc = self.world.scale();
        while self.starfall.plan.as_ref().and_then(|p| p.last()).is_some_and(|&t| t <= now) {
            self.starfall.plan.as_mut().map(|p| p.pop());
            let teach = r.teach > 0.0 && self.starfall.begun == 0;
            self.starfall.begun += 1;
            let ground = self.drop_ground();
            if let Some(at) = self.fall_spot(&ground, teach) {
                let lands = now + secs(r.warn);
                self.starfall.live.push(Drop::ore(at, r.worth, now, lands, lands + secs(r.life), r.ore_r * sc));
            }
        }
        // Landing: the blast under it, a hurt to soldiers there, and its
        // lure for the rest of its life.
        for i in 0..self.starfall.live.len() {
            let d = self.starfall.live[i];
            if d.down || now < d.lands {
                continue;
            }
            self.starfall.live[i].down = true;
            self.starfall.landed += 1;
            let fine = tps * self.world.fine_steps().max(1) as f32;
            let crater = (r.crater_secs * fine).round().clamp(0.0, u16::MAX as f32) as u16;
            let b = Burst { at: (d.at.0 / sc, d.at.1 / sc), r: r.kill_r, kill: true, crater, crater_r: r.crater_r, ..Burst::default() };
            self.killed += self.world.burst(&b, &BurstRules::default()).killed;
            let hit = r.kill_r * sc;
            for s in self.soldiers.iter_mut().filter(|s| s.alive && (s.x - d.at.0).hypot(s.y - d.at.1) <= hit) {
                s.hp -= r.hurt;
                if s.hp <= 0.0 {
                    self.fell_starfall += 1;
                    s.alive = false;
                    s.hp = 0.0;
                    s.carry = 0.0;
                }
            }
            self.lure_marks.push(LureMark { at: d.at, until: d.ends });
            self.starfall.live[i].lure = Some(d.at);
        }
        // Cores: each mini-boss broken since the last look (a re-formed
        // one's worth what its core had left, `cores.rs`), the fruit that
        // broke (its centre the last second it lived), and each raid
        // beaten (`Game::raid_cores`).
        let ended: Vec<Boss> = self.bosses.ended[self.starfall.bosses_seen.min(self.bosses.ended.len())..].iter().filter(|e| e.1 == BossEnd::Broken).map(|e| e.0).collect();
        self.starfall.bosses_seen = self.bosses.ended.len();
        // (where, worth, from, fresh: counted against `core_pot`)
        let mut cores: Vec<((f32, f32), f32, CoreFrom, bool)> = Vec::new();
        for b in ended {
            let from = CoreFrom::Boss { kind: b.kind, grade: b.grade };
            match self.cores.carried(&b) {
                Some(carry) => cores.push((b.centre, carry, from, false)),
                None => cores.push((b.centre, r.core_worth + r.core_grade * b.grade.clamp(0.0, 1.0), from, true)),
            }
        }
        if let Some((t, end, _)) = self.fruits.last {
            if self.starfall.fruit_seen != Some(t) {
                self.starfall.fruit_seen = Some(t);
                if end == FruitEnd::Broken {
                    if let Some(at) = self.starfall.fruit_at {
                        cores.push((at, r.core_worth, CoreFrom::Fruit, true));
                    }
                }
            }
        }
        self.starfall.fruit_at = self.fruits.live.map(|f| f.centre);
        cores.extend(self.raid_cores().into_iter().map(|(at, worth)| (at, worth, CoreFrom::Raid, true)));
        // On open ground the squad can walk to (`DropGround`): a boss's
        // centre can be over rock, or over the crystal it wrapped.
        let ground = (!cores.is_empty()).then(|| self.drop_ground());
        for (at, worth, from, fresh) in cores {
            let at = ground.as_ref().and_then(|gr| gr.snap(at, r.core_r * sc, 0.5 * sc, 40.0 * sc)).unwrap_or(at);
            // The planet's pot: fresh cores hold no more than `core_pot` in
            // all, so waiting for more bosses and raids pays nothing more.
            let worth = if fresh { worth.min(r.core_pot - self.cores.pot) } else { worth };
            if worth < 1.0 {
                continue;
            }
            if fresh {
                self.cores.pot += worth;
            }
            self.starfall.cores += 1;
            let ends = now + secs(r.core_life);
            // It runs home to the slime, and walkers stream back to it: it
            // smells (`drop_units`) and is a lure where it goes
            // (`Game::time_cores`).
            self.starfall.live.push(Drop::core(at, worth, from, now, ends, r.core_r * sc));
        }
        let gone: Vec<Drop> = self.starfall.live.iter().filter(|d| d.down && (now >= d.ends || d.left <= 0.01)).copied().collect();
        for d in &gone {
            self.drop_gone(d);
        }
        self.starfall.live.retain(|d| !(d.down && (now >= d.ends || d.left <= 0.01)));
    }

    /// Drop `d` is gone (drained, eaten, cooled or re-formed): its lure
    /// goes with it. Until the bug check of 2026-11-20 the mark stayed to
    /// the end of the drop's life: ore a squad drained in a second drew
    /// the slime within `Rules::lure_reach` to where the squad stood for
    /// the rest of its minute.
    pub(super) fn drop_gone(&mut self, d: &Drop) {
        if let Some(at) = d.lure {
            self.lure_marks.retain(|m| !(m.at == at && m.until == d.ends));
        }
    }

    /// The ground drops may lie on now (`DropGround`).
    pub(super) fn drop_ground(&self) -> DropGround {
        let (w, h) = self.world.size();
        let sc = self.s();
        let stand = self.standable_cells();
        // Each live soldier's cell, or the nearest standable one within two
        // dish cells (a soldier crossing a door's edge).
        let reach = (2.0 * sc).ceil() as i64;
        let mut stack: Vec<usize> = Vec::new();
        for s in self.soldiers.iter().filter(|s| s.alive) {
            let (cx, cy) = (s.x as i64, s.y as i64);
            let near = (-reach..=reach)
                .flat_map(|dy| (-reach..=reach).map(move |dx| (dx, dy)))
                .filter(|&(dx, dy)| {
                    let (x, y) = (cx + dx, cy + dy);
                    x >= 0 && y >= 0 && (x as usize) < w && (y as usize) < h && stand[y as usize * w + x as usize]
                })
                .min_by_key(|&(dx, dy)| dx * dx + dy * dy);
            if let Some((dx, dy)) = near {
                stack.push((cy + dy) as usize * w + (cx + dx) as usize);
            }
        }
        let ok = if stack.is_empty() {
            stand
        } else {
            let mut ok = vec![false; w * h];
            for &i in &stack {
                ok[i] = true;
            }
            while let Some(i) = stack.pop() {
                let (x, y) = (i % w, i / w);
                let mut go = |j: usize| {
                    if stand[j] && !ok[j] {
                        ok[j] = true;
                        stack.push(j);
                    }
                };
                if x > 0 {
                    go(i - 1);
                }
                if x + 1 < w {
                    go(i + 1);
                }
                if y > 0 {
                    go(i - w);
                }
                if y + 1 < h {
                    go(i + w);
                }
            }
            ok
        };
        let crystals = self.world.crystals().iter().enumerate().map(|(i, c)| (c.0, self.world.crystal_r(i) + CRYSTAL_CLEAR * sc)).collect();
        DropGround { w, h, ok, crystals, pad: ROCK_CLEAR * sc }
    }

    /// Where the next fall lands (fine cells): open ground at least `near`
    /// from every live soldier and within `far` of one, away from what is
    /// already down, by slime where it can be; None with no open ground.
    ///
    /// `teach` (the teaching fall, `StarRules::teach`): within `teach_far`
    /// of the nearest soldier, where the fewest walkers are, instead of
    /// where the most are; today's rule if no such spot is found.
    ///
    /// Only where a soldier can walk to and stand, clear of crystals
    /// (`DropGround::fits`, 2026-11-22): before, only its middle cell had
    /// to be open, and ore came down on crystals and over rock.
    fn fall_spot(&mut self, ground: &DropGround, teach: bool) -> Option<(f32, f32)> {
        let r = self.rules.star;
        let sc = self.world.scale();
        let size = r.ore_r * sc;
        let (w, h) = self.world.size();
        let soldiers: Vec<(f32, f32)> = self.soldiers.iter().filter(|s| s.alive).map(|s| (s.x, s.y)).collect();
        let others: Vec<(f32, f32)> = self.starfall.live.iter().map(|d| d.at).collect();
        let bin = (sc as usize).max(1);
        let grid = self.world.mould_grid(bin);
        let gw = w.div_ceil(bin);
        let slime_near = |p: (f32, f32)| -> u32 {
            let Some(g) = grid.as_ref() else { return 0 };
            let reach = (r.slime_r * sc / bin as f32).ceil() as i64;
            let (cx, cy) = ((p.0 / bin as f32) as i64, (p.1 / bin as f32) as i64);
            let gh = (g.len() / gw.max(1)) as i64;
            let mut n = 0;
            for y in (cy - reach).max(0)..(cy + reach + 1).min(gh) {
                for x in (cx - reach).max(0)..(cx + reach + 1).min(gw as i64) {
                    if (x - cx) * (x - cx) + (y - cy) * (y - cy) <= reach * reach {
                        n += g[(y * gw as i64 + x) as usize];
                    }
                }
            }
            n
        };
        let mut open = None;
        let mut fair = None;
        let mut best: Option<((f32, f32), u32)> = None;
        let mut calm: Option<((f32, f32), u32)> = None;
        // The teaching fall's spot outside `teach_dy`: used only when none
        // inside fits.
        let mut tall: Option<((f32, f32), u32)> = None;
        let far = if teach { r.teach_far.min(r.far) } else { r.far };
        // Near the squad a random point rarely falls within `teach_far`:
        // the teaching fall also tries points round each soldier.
        // (Doubled with the stricter ground, 11-22: a planet's crystals and
        // the ground cut off from the squad turn more tries away.)
        let tries = if teach { 480 } else { 120 };
        for k in 0..tries {
            let p = if teach && k % 2 == 1 && !soldiers.is_empty() {
                let q = soldiers[(self.starfall.rng.f32() * soldiers.len() as f32) as usize % soldiers.len()];
                let (a, d) = (self.starfall.rng.f32() * std::f32::consts::TAU, (r.near + (far - r.near).max(0.0) * self.starfall.rng.f32()) * sc);
                (q.0 + d * a.cos(), q.1 + d * a.sin())
            } else {
                (self.starfall.rng.f32() * w as f32, self.starfall.rng.f32() * h as f32)
            };
            if p.0 < 0.0 || p.1 < 0.0 || p.0 >= w as f32 || p.1 >= h as f32 {
                continue;
            }
            if !ground.fits(p, size) {
                continue;
            }
            open.get_or_insert(p);
            let nearest = soldiers.iter().map(|q| (q.0 - p.0).hypot(q.1 - p.1)).fold(f32::INFINITY, f32::min);
            if nearest < r.near * sc || (nearest > far * sc && !soldiers.is_empty()) || others.iter().any(|q| (q.0 - p.0).hypot(q.1 - p.1) < 2.0 * r.near * sc) {
                continue;
            }
            fair.get_or_insert(p);
            let n = slime_near(p);
            if n > 0 && best.is_none_or(|b| n > b.1) {
                best = Some((p, n));
            }
            let beside = !teach || soldiers.iter().min_by(|a, b| (a.0 - p.0).hypot(a.1 - p.1).total_cmp(&(b.0 - p.0).hypot(b.1 - p.1))).is_none_or(|q| (q.1 - p.1).abs() <= r.teach_dy * sc);
            let slot = if beside { &mut calm } else { &mut tall };
            if slot.is_none_or(|c| n < c.1) {
                *slot = Some((p, n));
            }
        }
        if teach {
            if let Some(c) = calm.or(tall) {
                return Some(c.0);
            }
            // Nowhere near the squad: an ordinary fall.
            return self.fall_spot(ground, false);
        }
        // Nothing found by chance (a squad boxed in a small cave): the
        // nearest fitting ground `near` out from a soldier.
        let last = || soldiers.first().and_then(|q| ground.snap((q.0 + r.near * sc, q.1), size, 0.5 * sc, far * sc));
        best.map(|b| b.0).or(fair).or(open).or_else(last)
    }

    /// The units the frame sends for what is down: each smells (`smell`)
    /// and reads the walkers on it.
    pub(super) fn drop_units(&self, units: &mut Vec<UnitIn>, who: &mut Vec<Who>, from: &mut Vec<Option<usize>>) {
        let r = self.rules.star;
        let sc = self.world.scale();
        for (i, d) in self.starfall.live.iter().enumerate() {
            if !d.down || units.len() >= squad::UNITS_MAX {
                continue;
            }
            // A core smells as ore does, so the slime streams back to it
            // (`cores.rs`).
            units.push(UnitIn {
                x: d.at.0,
                y: d.at.1,
                touch: d.r,
                smell_r: r.smell_r * sc,
                smell: self.world.food_deposit() * self.rules.smell * r.smell * self.rules.frame_ticks as f32,
                ..UnitIn::default()
            });
            who.push(Who::Drop(i));
            from.push(None);
        }
    }

    /// Drop `i` this frame: the soldiers by it drain it into points, the
    /// `touching` walkers on it eat it, and the slime's surge is fed once
    /// it has eaten half an ore.
    pub(super) fn drop_out(&mut self, i: usize, touching: u32, dt: f32) {
        let r = self.rules.star;
        let sc = self.world.scale();
        let Some(d) = self.starfall.live.get(i).copied() else { return };
        let reach = d.r + r.reach * sc;
        let takers = self.soldiers.iter().filter(|s| s.alive && (s.x - d.at.0).hypot(s.y - d.at.1) <= reach).count() as u32;
        let hands = self.rules.harvest / Rules::default().harvest.max(1e-6);
        let took = (takers as f32 * r.take * hands * dt).min(d.left);
        // A core is eaten slower than ore (`core_eat`), so a boss broken by
        // thick slime gives the squad a few seconds before it re-forms.
        let eat = if d.kind == DropKind::Core { r.core_eat } else { r.eat };
        let ate = (touching as f32 * eat * self.rules.mass * dt).min(d.left - took);
        let d = &mut self.starfall.live[i];
        d.left -= took + ate;
        d.taken += took;
        d.eaten += ate;
        (d.takers, d.eaters) = (takers, touching);
        let feed = !d.fed && d.eaten >= 0.5 * d.worth;
        if feed {
            d.fed = true;
        }
        let kind = d.kind;
        // Emptied this frame with soldiers on it: its points are in. The
        // drop itself stays until the next second's tidy (`time_starfall`),
        // so the window hides it from here (`left`).
        let cashed = took > 0.0 && d.left <= 0.01 && d.left + took + ate > 0.01;
        if cashed {
            let c = Cashed { at: d.at, kind, taken: d.taken, tick: self.world.ticks() };
            let fs = &mut self.starfall;
            fs.cashed.push(c);
            if fs.cashed.len() > CASHED_KEPT {
                fs.cashed.remove(0);
            }
            fs.cashed_n += 1;
        }
        self.starfall.points += took;
        self.starfall.eaten += ate;
        match kind {
            DropKind::Ore => self.starfall.ore_taken += took,
            DropKind::Core => self.starfall.core_taken += took,
        }
        if feed {
            self.surge_meter += self.rules.surge_food;
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn starry(seed: u64, star: StarRules) -> Game<BigDish> {
        Game::new(race_world(seed, 1, 1_000), Rules { star, ..Rules::default() }, seed)
    }

    /// The owner (11-22): Starfall "can ... fall ... on top of crystals or
    /// rock/walls and become inaccessible". Every spot a fall picks is
    /// ground a soldier can walk to, clear of every crystal's disc.
    #[test]
    fn falls_land_on_reachable_ground_clear_of_crystals() {
        for seed in 1..5 {
            let mut g = starry(seed, StarRules::for_depth(2));
            while g.time() < 20.0 && g.end.is_none() {
                g.step();
            }
            let ground = g.drop_ground();
            let sc = g.world.scale();
            let size = g.rules.star.ore_r * sc;
            let crystals: Vec<((f32, f32), f32)> = g.world.crystals().iter().enumerate().map(|(i, c)| (c.0, g.world.crystal_r(i))).collect();
            for _ in 0..30 {
                let Some(p) = g.fall_spot(&ground, false) else { continue };
                assert!(ground.fits(p, size), "seed {seed}: {p:?} is not reachable open ground");
                for &(c, r) in &crystals {
                    assert!((c.0 - p.0).hypot(c.1 - p.1) >= r + size, "seed {seed}: {p:?} lies on the crystal at {c:?}");
                }
            }
        }
    }

    /// A core dropped over a crystal (a boss that wrapped one) is put
    /// beside it, and the home it slides to is reachable ground too.
    #[test]
    fn a_core_over_a_crystal_is_moved_off_it_and_slides_to_open_ground() {
        let mut g = starry(3, StarRules::for_depth(2));
        while g.time() < 30.0 && g.end.is_none() {
            g.step();
        }
        let ground = g.drop_ground();
        let sc = g.world.scale();
        let size = g.rules.star.core_r * sc;
        let mut n = 0;
        for (i, (c, _)) in g.world.crystals().into_iter().enumerate() {
            let Some(p) = ground.snap(c, size, 0.5 * sc, 40.0 * sc) else { continue };
            assert!((c.0 - p.0).hypot(c.1 - p.1) >= g.world.crystal_r(i), "crystal {i}: the core stayed on it at {p:?}");
            assert!(ground.walkable(p), "crystal {i}: {p:?} cannot be walked to");
            let home = g.core_home(p, size);
            assert!(home == p || ground.fits(home, size), "crystal {i}: home {home:?} is not reachable open ground");
            n += 1;
        }
        assert!(n > 0, "no crystal had open ground near it");
    }

    /// The owner's 19:06Z card: "a minimum but can sometimes happen a lot".
    #[test]
    fn falls_have_a_floor_and_a_long_tail() {
        let r = StarRules::for_depth(1);
        let mut rng = Rng::new(7);
        let n: Vec<u32> = (0..20_000).map(|_| fall_count(&r, || rng.f32())).collect();
        let mean = n.iter().sum::<u32>() as f32 / n.len() as f32;
        let want = r.least as f32 + r.tail / (1.0 - r.tail);
        assert!(n.iter().all(|&k| (r.least..=r.most).contains(&k)), "between {} and {}", r.least, r.most);
        assert!((mean - want).abs() < 0.05, "mean {mean} against {want}");
        let many = n.iter().filter(|&&k| k >= 7).count() as f32 / n.len() as f32;
        assert!((0.05..0.15).contains(&many), "seven or more on {many} of planets");
        assert_eq!(n.iter().min(), Some(&r.least), "the floor is met");
    }

    /// Ore falls, lands as a blast, and pays the soldiers standing by it;
    /// off, nothing falls.
    #[test]
    fn ore_falls_lands_and_pays_who_reaches_it() {
        let star = StarRules { first: 1.0, last: 3.0, gap: 1.0, warn: 2.0, near: 0.0, teach: 0.0, ..StarRules::for_depth(1) };
        let mut g = starry(3, star);
        let mut drained = None;
        let mut ores: Vec<Drop> = Vec::new();
        while g.time() < 40.0 && g.end.is_none() {
            g.step();
            if let Some(d) = g.starfall.live.iter().find(|d| d.down && d.kind == DropKind::Ore).copied() {
                if !ores.iter().any(|o| o.at == d.at) {
                    ores.push(d);
                }
                // Every soldier on it, held there, until it is empty.
                for s in g.soldiers.iter_mut().filter(|s| s.alive) {
                    (s.x, s.y) = d.at;
                }
                drained = drained.or(Some(g.time()));
            }
            if drained.is_some_and(|t| g.time() > t + 5.0) {
                break;
            }
        }
        assert!(g.starfall.falls >= star.least, "{} falls drawn", g.starfall.falls);
        assert!(g.starfall.landed >= 1, "one landed by {}", g.time());
        assert!(g.starfall.points > 0.0 && g.starfall.ore_taken > 0.0, "the squad took {}", g.starfall.points);
        assert!(g.starfall.points <= star.worth * g.starfall.falls as f32 + 1e-3, "never more than the falls hold");
        // Drained long before it would have cooled, its lure went with it
        // (the bug check, 2026-11-20): the slime is drawn to what is down.
        let gone: Vec<&Drop> = ores.iter().filter(|o| !g.starfall.live.iter().any(|d| d.at == o.at)).collect();
        assert!(gone.iter().any(|o| o.ends > g.world.ticks()), "an ore was drained before its time");
        for o in gone {
            assert!(!g.lure_marks.iter().any(|m| m.at == o.at), "a drained ore's lure stays at {:?}", o.at);
        }
        let mut off = starry(3, StarRules { on: false, ..star });
        while off.time() < 20.0 && off.end.is_none() {
            off.step();
        }
        assert!(off.starfall.plan.is_none() && off.starfall.live.is_empty() && off.starfall.points == 0.0, "off, nothing falls");
    }

    /// Planet 1's first fall comes at `teach` and lands near the squad,
    /// on less slime than an ordinary fall picks (Direction, 2026-11-21).
    #[test]
    fn the_teaching_fall_lands_near_the_squad_on_calm_ground() {
        assert_eq!(StarRules::for_depth(1).teach, 50.0);
        assert_eq!(StarRules::for_depth(2).teach, 0.0, "planet 1 only");
        let first = |teach: f32| {
            let mut g = starry(5, StarRules { teach, ..StarRules::for_depth(1) });
            while g.starfall.live.is_empty() && g.time() < 300.0 && g.end.is_none() {
                g.step();
            }
            let d = g.starfall.live[0];
            let sc = g.world.scale();
            let nearest = g.soldiers.iter().filter(|s| s.alive).map(|s| (s.x - d.at.0).hypot(s.y - d.at.1)).fold(f32::INFINITY, f32::min) / sc;
            (g.time(), nearest)
        };
        let beside = |teach: f32| {
            let mut g = starry(5, StarRules { teach, ..StarRules::for_depth(1) });
            while g.starfall.live.is_empty() && g.time() < 300.0 && g.end.is_none() {
                g.step();
            }
            let d = g.starfall.live[0];
            let sc = g.world.scale();
            let q = g.soldiers.iter().filter(|s| s.alive).min_by(|a, b| (a.x - d.at.0).hypot(a.y - d.at.1).total_cmp(&(b.x - d.at.0).hypot(b.y - d.at.1))).unwrap();
            (g.time(), (q.y - d.at.1).abs() / sc)
        };
        let (t, near) = first(50.0);
        let r = StarRules::for_depth(1);
        assert!((t - 50.0).abs() < 0.5, "its shadow shows at 0:50, to land at 1:00, not {t}");
        assert!(near >= r.near - 0.5 && near <= r.teach_far + 0.5, "{near} dish cells from the squad");
        let (_, dy) = beside(50.0);
        assert!(dy <= r.teach_dy + 0.5, "beside the squad, not under the bottom bar: {dy} dish cells up or down");
        let (t0, near0) = first(0.0);
        assert!(t0 >= r.first - 0.5, "off, the window as before: {t0}");
        assert!(near0 >= r.near - 0.5, "off, still clear of the squad: {near0}");
    }

    /// A star that lands on soldiers kills the ones it hurts past their
    /// health, and the game counts them apart from the slime's kills
    /// (`Game::fell_starfall`, the bot's `starfall_end`).
    #[test]
    fn a_star_landing_on_soldiers_is_counted_as_their_death() {
        let star = StarRules { first: 1.0, last: 3.0, gap: 1.0, warn: 2.0, near: 0.0, teach: 0.0, ..StarRules::for_depth(1) };
        let mut g = starry(3, star);
        while g.time() < 40.0 && g.end.is_none() && g.starfall.landed == 0 {
            if let Some(d) = g.starfall.live.iter().find(|d| !d.down).copied() {
                for s in g.soldiers.iter_mut().filter(|s| s.alive) {
                    (s.x, s.y) = d.at;
                    s.hp = s.hp.min(1.0);
                }
            }
            g.step();
        }
        assert!(g.starfall.landed >= 1, "one landed by {}", g.time());
        let dead = (g.soldiers.len() - g.alive()) as u32;
        assert!(dead > 0 && g.fell_starfall == dead && g.fell_bitten + g.fell_burned == 0, "{dead} dead, {} to the star", g.fell_starfall);
    }

    /// Waiting makes no more: every fall is drawn at the start, none after
    /// `last`.
    #[test]
    fn the_planets_falls_are_drawn_once() {
        let star = StarRules { first: 1.0, last: 6.0, gap: 1.0, warn: 1.0, life: 2.0, ..StarRules::for_depth(2) };
        let mut g = starry(5, star);
        while g.time() < 12.0 && g.end.is_none() {
            g.step();
        }
        let falls = g.starfall.falls;
        while g.time() < 40.0 && g.end.is_none() {
            g.step();
        }
        assert_eq!(g.starfall.falls, falls, "no more drawn");
        assert!(g.starfall.plan.as_ref().is_some_and(|p| p.is_empty()), "the plan is spent");
        assert!(g.starfall.live.iter().all(|d| d.kind == DropKind::Core), "every fall has cooled");
    }

    /// Direction, 2026-11-20: a planet with a par draws its falls by half
    /// of it (a 4-minute planet saw one fall when they ran to 10:00).
    #[test]
    fn the_falls_come_by_half_the_par() {
        let star = StarRules { gap: 1.0, ..StarRules::for_depth(1) };
        for (par, by) in [(0.0, star.last), (160.0, 80.0)] {
            let mut g = Game::new(race_world(5, 1, 1_000), Rules { star, par, ..Rules::default() }, 5);
            g.step();
            while g.starfall.plan.is_none() && g.end.is_none() {
                g.step();
            }
            let tps = g.rules.tps as f32;
            let late = g.starfall.plan.as_ref().and_then(|p| p.first()).map(|&t| (t - g.landed) as f32 / tps).unwrap();
            assert!(late <= by + g.starfall.falls as f32 * star.gap, "par {par}: last fall at {late} s, window to {by}");
            assert!(late > star.first, "par {par}: last fall at {late} s");
        }
    }
}
