//! **Mini-bosses** (2026-11-16, the mini-boss thread;
//! `Reports/mini-boss-2026-11-16/`). The owner, 03:41Z: "There should be
//! random chances (when slime get concentrated) that mini-boss appear. It
//! can be similar to the condensation event (maybe even replace it). The
//! slime forms a stronger, more aggressive, sub-unit that leaves to seek
//! out the player."
//!
//! The owner's answers to round 1, 04:58Z: every kind on the page; "Remove
//! the bronze, keep the specks (specks and mini-boss are separate but both
//! happen. Specks more often early game)"; a mix of hunters and stay-put
//! kinds; chance and a visible cause both, "Chance increases with
//! progression into the run. But they also shouldn't just come out in a
//! random spot. It should spawn where there is crowding or a big part gets
//! cut off"; squads target mini-bosses first.
//!
//! Where one forms, from the mould grid once a game second (walkers on each
//! dish cell, both engines):
//! - **Crowding**: when a chance comes (`every`, drawn as raids draw
//!   theirs), a knot whose square of `reach` round it is at least `crowd`
//!   full (a walker a fine cell is full), picked weighted by how full.
//! - **A big part cut off**: a piece of slime joined to no crystal with
//!   food, of at least `cut_least` walkers, becomes one with chance `cut`
//!   each second, instead of only withering and panicking.
//!
//! The first kind, the hunter: the walkers round the knot become a lump
//! (`BigParams::lumps`, both engines; any walker, not condensed bronze): for
//! `gather` seconds it clenches where it stands (its leash alone pulls it
//! into a tight knot, the tell), then it hunts the nearest soldier, steered
//! by a pull added to what it smells and held together by its leash, so it
//! travels as one body and not as the stream of specks a breakaway or a
//! raid makes. It breaks up when it has lost most of its members or hunted
//! `life` seconds; its members are then slime far from food, which withers.
//!
//! On in a run from planet 2 (`BossRules::for_depth`, rising with the
//! planet; none on planet 1 since 11-18, the owner's play notes); off in
//! `Rules::default()` so the censuses and replays keep their games; a film
//! turns it on with `set=boss_every=40` or stages one with `boss=T,X,Y`.

use super::{Game, Kind, Rng, World};
use crate::mould::big::{Lump, LUMPS};

/// The mini-bosses' numbers. Times in game seconds, distances in dish
/// cells.
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct BossRules {
    /// The time till the next chance, drawn from a normal distribution
    /// with this mean and `every_sd` (0: off), never under `min`; the
    /// first from the landing the same way.
    pub every: f32,
    pub every_sd: f32,
    pub first: f32,
    pub first_sd: f32,
    pub min: f32,
    /// Walkers within this of the knot join (a cut-off part: within its
    /// own size, up to twice this).
    pub reach: f32,
    /// A crowded knot: the square of side 2 `reach` + 1 dish cells round it
    /// at least this full (walkers over fine cells).
    pub crowd: f32,
    /// A part cut off from every crystal with food, of at least `cut_least`
    /// walkers, becomes one with this chance each game second (0 off),
    /// from `first` seconds after the landing (the opening's blob is often
    /// cut off), and never within `rest` seconds of one ending where it
    /// was (its own remnant, an unfed part, re-formed at once with its
    /// hits healed: review of the mini-boss PR).
    pub cut: f32,
    pub cut_least: u32,
    pub rest: f32,
    /// Fewest members it must hold a second after forming, else it falls
    /// apart at once (the chance came to nothing).
    pub least: u32,
    /// At most this many at once (up to `big::LUMPS`).
    pub most: usize,
    /// Seconds it clenches in place before it hunts, on `clench`'s leash.
    pub gather: f32,
    pub clench: f32,
    /// The hunt's steering (`big::Lump`): the pull to the soldier, the
    /// leash (x its radius) and its pace.
    pub pull: f32,
    pub leash: f32,
    pub haste: f32,
    /// It breaks up under this share of its first members, or after
    /// `life` seconds of hunting.
    pub quorum: f32,
    pub life: f32,
    /// **Strength** (owner 05:23Z: "They should be significantly stronger
    /// than normal slime too and vary quite a bit in strength from boss to
    /// boss"): each one's grade is drawn evenly from `grade_lo` to
    /// `grade_hi` (0 to 1) as it forms, and sets, from the low end to the
    /// high: the blows each member takes to kill (`armour_lo` to
    /// `armour_hi`; ordinary slime takes 1), the walkers it
    /// touches a soldier as beyond itself (`bite_lo` to `bite_hi`), its
    /// size (`reach` x 1 -/+ `size`/2) and its pace (`haste` x 1 to
    /// 1 + `faster`).
    pub grade_lo: f32,
    pub grade_hi: f32,
    pub armour_lo: f32,
    pub armour_hi: f32,
    pub bite_lo: f32,
    pub bite_hi: f32,
    pub size: f32,
    pub faster: f32,
    /// **The body** (the look thread's ask, 11-16 06:31Z: a sparse even
    /// cloud does not read as a predator; the look must come from the
    /// simulation, owner 05:28Z there). Its leash and clench tighten with
    /// the grade (x 1 - `tight` x grade), and a hunter **lunges**: every
    /// `lunge_every` seconds of the hunt (0 off) its pull and pace go up
    /// `lunge_pull` and `lunge_haste` times for `lunge` seconds, so the
    /// body bunches and surges for real (no painted pulse).
    pub tight: f32,
    pub lunge_every: f32,
    pub lunge: f32,
    pub lunge_pull: f32,
    pub lunge_haste: f32,
    /// Seconds a broken or spent one's members stay marked, unsteered and
    /// unarmoured, before they are let go (its lump not reused till
    /// then), so the view drains its colour as it scatters
    /// (`Game::lump_looks`); a fizzled one goes at once.
    pub fade: f32,
    /// **The kinds** (owner 04:58Z: "all of them"; a mix of hunters and
    /// kinds with an errand): each one's kind is drawn as it forms, by
    /// these weights (hunter, wanderer, sapper, leech), among the kinds it
    /// has a target for (a sapper needs something built, a leech a pump, a
    /// wanderer a fresh crystal far off); a cut-off part makes only the
    /// runners (hunter, wanderer). `kind` 1-4 forces one (0 draws).
    pub kinds: [f32; 4],
    pub kind: f32,
    /// The wanderer: a fresh crystal (at least `fresh` of its charge left)
    /// at least `far` dish cells off; it roots there `root` seconds, eats,
    /// takes in the slime there (its reach x `grow`), and sets off again.
    pub far: f32,
    pub fresh: f32,
    pub root: f32,
    pub grow: f32,
    /// The leech holds its members in a ring round the pump it sits on,
    /// `ring` x the crystal's radius from its middle (`big::Lump::ring`; 0
    /// sits on it as a disc). 1.5 is just outside the rig, which is drawn
    /// to 1.3 (`pump_rig_r`). Look thread, 11:56Z: sat on the pump, its
    /// members lay under the crystal, the rig and the box, and almost none
    /// of it showed.
    pub ring: f32,
}

impl BossRules {
    /// A run's planet `depth` (1 to 8): none on planet 1, then the chance
    /// rises through the run (owner 04:58Z), from a crowding chance every
    /// 150 s and a cut-off part's 2% a second on planet 2 to every 45 s and
    /// 10% on planet 8; at most one at once on planets 2-3, two from 4.
    /// Specks (breakaways) stay more common early (the slime-enemy
    /// thread's).
    ///
    /// None on planet 1 since 2026-11-18 (the owner's play notes, 15:56Z:
    /// "Mini-bosses also shouldn't show up in world one. that should be
    /// part of progression"). Until then the curve started on planet 1;
    /// it moved up a planet unchanged, so planet 2 has what planet 1 had
    /// and planet 8 still has every 45 s and 10%.
    pub fn for_depth(depth: u32) -> BossRules {
        if depth <= 1 {
            return BossRules::default();
        }
        let t = (depth.min(8) - 2) as f32 / 6.0;
        BossRules {
            every: 150.0 + (45.0 - 150.0) * t,
            every_sd: 30.0 + (10.0 - 30.0) * t,
            first: 90.0 + (40.0 - 90.0) * t,
            cut: 0.02 + (0.1 - 0.02) * t,
            most: if depth >= 4 { 2 } else { 1 },
            // Weaker on the first planets, all of them stronger by the
            // last, and a wide spread on each.
            grade_lo: 0.4 * t,
            grade_hi: 0.6 + 0.4 * t,
            ..BossRules::default()
        }
    }

    /// The blows a member of a grade `g` mini-boss takes (`Lump::armour`).
    pub fn armour_at(&self, g: f32) -> u32 {
        (self.armour_lo + (self.armour_hi - self.armour_lo) * g).round().clamp(1.0, 15.0) as u32
    }

    /// The walkers a member touches as beyond itself (`Lump::bite`).
    pub fn bite_at(&self, g: f32) -> u32 {
        (self.bite_lo + (self.bite_hi - self.bite_lo) * g).round().max(0.0) as u32
    }

    /// Its reach (dish cells) as it forms.
    pub fn reach_at(&self, reach: f32, g: f32) -> f32 {
        reach * (1.0 + self.size * (g - 0.5))
    }

    /// Its leash (`leash` or `clench`) at grade `g`: tighter, stronger.
    pub fn leash_at(&self, leash: f32, g: f32) -> f32 {
        leash * (1.0 - self.tight * g).max(0.1)
    }

    /// Whether `t` ticks into its hunt, at `tps`, falls in a lunge.
    pub fn lunging(&self, t: u64, tps: f32) -> bool {
        let (every, long) = ((self.lunge_every * tps).round() as u64, (self.lunge * tps).round() as u64);
        every > 0 && long > 0 && t % every < long
    }

    /// Its members' pace (`Lump::haste`) as it hunts, never under the
    /// slime's own (1; a `Lump`'s 0 is off, its own pace too).
    pub fn haste_at(&self, g: f32) -> f32 {
        self.haste.max(1.0) * (1.0 + self.faster * g)
    }
}

impl Default for BossRules {
    fn default() -> Self {
        BossRules {
            every: 0.0,
            every_sd: 15.0,
            first: 45.0,
            first_sd: 10.0,
            min: 15.0,
            reach: 6.0,
            crowd: 0.8,
            cut: 0.0,
            cut_least: 400,
            rest: 30.0,
            least: 40,
            most: 2,
            gather: 6.0,
            clench: 1.0,
            // The body probe's held body (aim 30, leash 1.3: compact 1.2,
            // aspect 2, 6 of 6 seeds; `examples/bodyprobe.rs`).
            pull: 30.0,
            leash: 1.3,
            haste: 1.0,
            quorum: 0.3,
            life: 90.0,
            grade_lo: 0.0,
            grade_hi: 1.0,
            // 4 to 15 blows a member against ordinary slime's 1, and
            // touching as 2 to 5 walkers. Packed as it is, each shot that
            // lands blows about 30 members at once (planet 3, seed 1, 8
            // soldiers on free fire, every gun on the mini-boss first): at 5
            // blows a member one of 344 lasted 5 s under fire, about as long
            // as an unarmoured one of 418 had lasted before the squad aimed
            // at mini-bosses first.
            armour_lo: 4.0,
            armour_hi: 15.0,
            bite_lo: 1.0,
            bite_hi: 4.0,
            size: 0.6,
            faster: 0.4,
            tight: 0.25,
            lunge_every: 5.0,
            lunge: 1.0,
            lunge_pull: 2.0,
            lunge_haste: 2.0,
            fade: 1.5,
            kinds: [1.0, 1.0, 1.0, 1.0],
            kind: 0.0,
            far: 40.0,
            fresh: 0.5,
            root: 15.0,
            grow: 2.0,
            ring: 1.5,
        }
    }
}

/// What a mini-boss is doing.
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum BossPhase {
    /// Clenching where it formed.
    Gather,
    /// Going for soldier `k` (None: no soldier left, it holds).
    Hunt(Option<usize>),
    /// On its errand, going for a point (fine cells): a wanderer's
    /// crystal, a sapper's wall or building, a leech's pump.
    Go((f32, f32)),
    /// There: a wanderer rooted on its crystal (since the tick), a sapper
    /// pressing into the wall, a leech on the pump.
    Sit((f32, f32), u64),
}

/// **What a mini-boss is** (`BossRules::kinds`), the view's kind number
/// (`Game::lump_looks` bits 0-2).
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum BossKind {
    /// Comes for the squads: hunts the nearest soldier, lunging.
    Hunter = 1,
    /// Today's break-away as one body: goes for a far fresh crystal, roots
    /// there and eats, takes in the slime there, and goes on bigger.
    Wanderer = 2,
    /// Goes for what you build: presses into the wall nearest it (or, with
    /// none, the nearest building) and eats through it.
    Sapper = 3,
    /// Goes for a pump and sits on it: the pump's income is its own while
    /// it sits; breaking it pays the stolen food back.
    Leech = 4,
}

impl BossKind {
    pub const ALL: [BossKind; 4] = [BossKind::Hunter, BossKind::Wanderer, BossKind::Sapper, BossKind::Leech];

    pub fn name(self) -> &'static str {
        match self {
            BossKind::Hunter => "hunter",
            BossKind::Wanderer => "wanderer",
            BossKind::Sapper => "sapper",
            BossKind::Leech => "leech",
        }
    }
}

/// One mini-boss.
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct Boss {
    /// Its lump (`big::LUMPS`).
    pub lump: usize,
    /// The tick it formed, and where (fine cells).
    pub born: u64,
    pub at: (f32, f32),
    pub cause: BossCause,
    pub phase: BossPhase,
    /// Its members a second after forming (0 until then), and now; its
    /// centre now (fine cells).
    pub first: u32,
    pub members: u32,
    pub centre: (f32, f32),
    /// How strong it is, 0 to 1, drawn when it forms (`BossRules::grade`).
    pub grade: f32,
    pub kind: BossKind,
    /// A leech's take from the pump it sits on (bank food), paid back if
    /// it is broken; a wanderer's crystals rooted on (a bit each, up to 64).
    pub stolen: f32,
    pub visited: u64,
}

/// What made a mini-boss.
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum BossCause {
    /// A crowded knot.
    Crowd,
    /// A part cut off from food.
    Cut,
    /// Staged (a film's `boss=`, a test).
    Staged,
}

/// How a mini-boss ended.
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum BossEnd {
    /// Too few walkers at the knot (`BossRules::least`).
    Fizzled,
    /// Under `quorum` of its first members.
    Broken,
    /// Hunted `life` seconds.
    Spent,
}

/// The coarse squares `Game::slime_parts` joins, in dish cells a side.
pub const PART_BIN: usize = 2;

/// One piece of slime (`Game::slime_parts`).
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct Part {
    pub walkers: u32,
    pub centre: (f32, f32),
    pub r: f32,
    pub squares: usize,
    pub fed: bool,
}

/// The game's mini-boss state.
#[derive(Debug)]
pub struct Bosses {
    pub live: Vec<Boss>,
    /// Every one that ended, with how and the tick.
    pub ended: Vec<(Boss, BossEnd, u64)>,
    /// The broken and spent still fading (`BossRules::fade`): each one and
    /// the tick it is let go.
    pub fading: Vec<(Boss, u64)>,
    /// Food leeches took from the pumps, and what broken ones paid back.
    pub stolen: f32,
    pub returned: f32,
    /// The tick of the next chance (drawn at the first second, then at
    /// each chance).
    pub next: Option<u64>,
    /// The fullest square a crowding chance last saw (a share of full),
    /// for tuning `BossRules::crowd`.
    pub crowd_top: f32,
    /// Their own draws, apart from `Game::rng`.
    rng: Rng,
}

impl Bosses {
    pub fn new(seed: u64) -> Bosses {
        Bosses { live: Vec::new(), ended: Vec::new(), fading: Vec::new(), stolen: 0.0, returned: 0.0, next: None, crowd_top: 0.0, rng: Rng::new(seed ^ 0xb055_5eed) }
    }

    /// A normal draw (Box-Muller) with mean `m` and spread `sd`.
    fn normal(&mut self, m: f32, sd: f32) -> f32 {
        let u = self.rng.f32().max(1e-7);
        let v = self.rng.f32();
        m + sd * (-2.0 * u.ln()).sqrt() * (std::f32::consts::TAU * v).cos()
    }
}

impl Default for Bosses {
    fn default() -> Self {
        Bosses::new(0)
    }
}

impl<W: World> Game<W> {
    /// Once a game second: each mini-boss's count, phase and aim, and a
    /// new one when a chance comes.
    pub(super) fn time_bosses(&mut self) {
        let r = self.rules.boss;
        let now = self.world.ticks();
        let tps = self.rules.tps as f32;
        let secs = move |s: f32| (s.max(0.0) * tps).round() as u64;
        let lumps = self.world.lumps();
        // The faded are let go.
        let world = &mut self.world;
        self.bosses.fading.retain(|&(b, until)| {
            let keep = now < until;
            if !keep {
                world.form_lump(b.lump, None);
            }
            keep
        });
        let mut k = 0;
        while k < self.bosses.live.len() {
            let mut b = self.bosses.live[k];
            let (members, centre) = lumps[b.lump].map_or((0, b.centre), |(x, y, n)| (n, (x, y)));
            (b.members, b.centre) = (members, centre);
            // Counted at the first second after forming (the lump's sums
            // come with the next step's results on either engine).
            let first = b.first == 0 && now > b.born;
            if first {
                b.first = members;
            }
            let end = if first && members < r.least.max(1) {
                Some(BossEnd::Fizzled)
            } else if b.first > 0 && (members as f32) < r.quorum * b.first as f32 {
                Some(BossEnd::Broken)
            } else if now > b.born + secs(r.gather + r.life) {
                Some(BossEnd::Spent)
            } else {
                None
            };
            if let Some(e) = end {
                // Unsteered and unarmoured at once; a broken or spent one's
                // members stay marked while it fades. A broken leech's
                // take drains back to the bank.
                self.world.set_lump(b.lump, Lump::default());
                if e == BossEnd::Broken && b.stolen > 0.0 {
                    self.bank += b.stolen;
                    self.bosses.returned += b.stolen;
                }
                if e == BossEnd::Fizzled || r.fade <= 0.0 {
                    self.world.form_lump(b.lump, None);
                } else {
                    self.bosses.fading.push((b, now + secs(r.fade).max(1)));
                }
                self.bosses.ended.push((b, e, now));
                self.bosses.live.remove(k);
                continue;
            }
            let hunt = b.born + secs(r.gather);
            let errand = if now < hunt || b.kind == BossKind::Hunter { None } else { self.errand(&mut b, now) };
            let cell = |p: (f32, f32)| crate::mould::larder_world::raid_cell(p, self.world.size());
            let strong = Lump { armour: r.armour_at(b.grade), bite: r.bite_at(b.grade), ..Lump::default() };
            let leash = r.leash_at(r.leash, b.grade);
            let clench = r.leash_at(r.clench, b.grade);
            let go = |p: (f32, f32)| Lump { aim: cell(p), pull: r.pull, leash, haste: r.haste_at(b.grade), ..strong };
            let lump = if now < hunt {
                b.phase = BossPhase::Gather;
                Lump { aim: cell(centre), leash: clench, ..strong }
            } else if let Some(p) = errand {
                // On its errand: going there, or sitting on it (a wanderer
                // rooted and eating, a leech holding on, both clenched; a
                // sapper pressing into the wall).
                match (b.phase, b.kind) {
                    (BossPhase::Sit(..), BossKind::Sapper) => go(p),
                    (BossPhase::Sit(..), kind) => {
                        // A leech rings its pump (`ring`): the pull still
                        // presses it in, onto the ring's inner edge.
                        let ring = if kind == BossKind::Leech { self.leech_ring(p) } else { 0.0 };
                        Lump { aim: cell(p), pull: 0.5 * r.pull, leash: clench, ring, ..strong }
                    }
                    _ => go(p),
                }
            } else {
                let prey = self.nearest_soldier(centre);
                b.phase = BossPhase::Hunt(prey);
                match prey {
                    Some(s) => {
                        // The lunge: harder and faster for a moment.
                        let (pull, haste) = if r.lunging(now - hunt, tps) { (r.lunge_pull.max(0.0), r.lunge_haste.max(1.0)) } else { (1.0, 1.0) };
                        Lump { aim: cell((self.soldiers[s].x, self.soldiers[s].y)), pull: r.pull * pull, leash, haste: r.haste_at(b.grade) * haste, ..strong }
                    }
                    None => Lump { aim: cell(centre), leash, ..strong },
                }
            };
            self.world.set_lump(b.lump, lump);
            self.bosses.live[k] = b;
            k += 1;
        }
        if (r.every <= 0.0 && r.cut <= 0.0) || self.bosses.live.len() >= r.most.min(LUMPS) {
            return;
        }
        // A big part cut off from food: each second's chance, once the
        // landing's grace is over, away from the live ones and from where
        // one ended lately.
        if r.cut > 0.0 && now >= secs(r.first) {
            let parts = self.slime_parts();
            let near = r.reach * self.world.scale();
            let rest = secs(r.rest);
            for p in parts.iter().filter(|p| !p.fed && p.walkers >= r.cut_least) {
                let by = |c: (f32, f32)| (c.0 - p.centre.0).hypot(c.1 - p.centre.1) < p.r + near;
                let lately = self.bosses.ended.iter().rev().take_while(|e| now < e.2 + rest).any(|e| by(e.0.centre));
                if self.bosses.live.len() >= r.most.min(LUMPS) || lately || self.bosses.live.iter().any(|b| by(b.centre)) {
                    continue;
                }
                if self.bosses.rng.f32() < r.cut {
                    let reach = (p.r / self.world.scale()).clamp(1.0, 2.0 * r.reach);
                    self.form_boss(p.centre, reach, BossCause::Cut);
                }
            }
        }
        if r.every <= 0.0 {
            return;
        }
        let gap = |bs: &mut Bosses, m: f32, sd: f32| secs(bs.normal(m, sd).max(r.min));
        let next = match self.bosses.next {
            Some(t) => t,
            None => {
                let t = now + gap(&mut self.bosses, r.first, r.first_sd);
                self.bosses.next = Some(t);
                t
            }
        };
        // A chance waits for a crowded knot: tried each second once due.
        if now < next || self.bosses.live.len() >= r.most.min(LUMPS) {
            return;
        }
        if let Some(at) = self.crowded_knot() {
            self.bosses.next = Some(now + gap(&mut self.bosses, r.every, r.every_sd));
            self.form_boss(at, r.reach, BossCause::Crowd);
        }
    }

    /// Forms a mini-boss round `at` (fine cells), of the walkers within
    /// `reach` dish cells (by its grade, `BossRules::reach_at`), on the
    /// first free lump, its grade drawn (`BossRules::grade_lo`); false with
    /// none free.
    pub fn form_boss(&mut self, at: (f32, f32), reach: f32, cause: BossCause) -> bool {
        // Never on a march (`RaidRules::march`): a lump forming takes every
        // walker in its reach from its lump, and a march is both a crowded
        // knot and a part away from food.
        let sc = self.world.scale();
        let on = |m: &super::March| (m.centre.0 - at.0).hypot(m.centre.1 - at.1) < self.rules.raid.march_leash.max(1.0) * (m.members as f32 / std::f32::consts::PI).sqrt() + 2.0 * reach * sc;
        if self.raids.marching.iter().any(on) {
            return false;
        }
        // Nor on a ripening fruit (`fruit.rs`, 2026-11-20), for the same
        // reason.
        if self.fruits.live.is_some_and(|f| (f.centre.0 - at.0).hypot(f.centre.1 - at.1) < (f.members as f32 / std::f32::consts::PI).sqrt() + 2.0 * reach * sc) {
            return false;
        }
        // A lump no boss, march or fruit holds.
        let Some(lump) = (0..LUMPS).find(|&l| !self.lump_taken(l)) else { return false };
        let r = self.rules.boss;
        let (lo, hi) = (r.grade_lo.clamp(0.0, 1.0), r.grade_hi.clamp(0.0, 1.0));
        let grade = lo + (hi - lo).max(0.0) * self.bosses.rng.f32();
        self.world.form_lump(lump, Some((at, r.reach_at(reach, grade))));
        let cell = crate::mould::larder_world::raid_cell(at, self.world.size());
        self.world.set_lump(lump, Lump { aim: cell, leash: r.leash_at(r.clench, grade), armour: r.armour_at(grade), bite: r.bite_at(grade), ..Lump::default() });
        let now = self.world.ticks();
        let mut b = Boss { lump, born: now, at, cause, phase: BossPhase::Gather, first: 0, members: 0, centre: at, grade, kind: BossKind::Hunter, stolen: 0.0, visited: 0 };
        b.kind = self.pick_kind(&b);
        self.bosses.live.push(b);
        true
    }

    /// Its kind (`BossRules::kinds`): forced by `kind`, else drawn by the
    /// weights among the kinds with a target from where it formed (a
    /// cut-off part: the runners only).
    fn pick_kind(&mut self, b: &Boss) -> BossKind {
        let r = self.rules.boss;
        let forced = r.kind.round() as usize;
        if (1..=4).contains(&forced) {
            return BossKind::ALL[forced - 1];
        }
        let w: Vec<f32> = BossKind::ALL
            .iter()
            .enumerate()
            .map(|(i, &kind)| {
                let runner = matches!(kind, BossKind::Hunter | BossKind::Wanderer);
                let able = kind == BossKind::Hunter || self.boss_goal(&Boss { kind, ..*b }).is_some();
                if (b.cause != BossCause::Cut || runner) && able { r.kinds[i].max(0.0) } else { 0.0 }
            })
            .collect();
        let sum: f32 = w.iter().sum();
        if sum <= 0.0 {
            return BossKind::Hunter;
        }
        let mut x = self.bosses.rng.f32() * sum;
        for (i, &wi) in w.iter().enumerate() {
            if x < wi {
                return BossKind::ALL[i];
            }
            x -= wi;
        }
        BossKind::Hunter
    }

    /// Where boss `b`'s errand takes it (fine cells): a wanderer's next
    /// fresh crystal far off and not rooted on before, a sapper's nearest
    /// point of a standing wall (else the nearest building), a leech's
    /// nearest working pump. None for a hunter, or with nothing to go for.
    pub fn boss_goal(&self, b: &Boss) -> Option<(f32, f32)> {
        let d = |p: (f32, f32)| (p.0 - b.centre.0).hypot(p.1 - b.centre.1);
        let near = |ps: Vec<(f32, f32)>| ps.into_iter().min_by(|a, c| d(*a).total_cmp(&d(*c)));
        match b.kind {
            BossKind::Hunter => None,
            BossKind::Wanderer => {
                let r = self.rules.boss;
                let far = r.far * self.world.scale();
                let cs = self.world.crystals();
                near(cs.iter().enumerate().filter(|&(j, _)| j >= 64 || b.visited & 1 << j == 0).filter(|&(j, c)| c.1 >= r.fresh * self.start_charge.get(j).copied().unwrap_or(0.0) && c.1 > 0.0 && d(c.0) >= far).map(|(_, c)| c.0).collect())
            }
            BossKind::Sapper => {
                let walls: Vec<(f32, f32)> = self.buildings.iter().filter(|w| w.alive).filter_map(|w| if let Kind::Wall { to } = w.kind { Some(nearest_on(w.at, to, b.centre)) } else { None }).collect();
                near(walls).or_else(|| near(self.buildings.iter().filter(|w| w.alive).map(|w| w.at).collect()))
            }
            BossKind::Leech => near(self.buildings.iter().filter(|w| w.alive && w.kind == Kind::Pump && w.up()).map(|w| w.at).collect()),
        }
    }

    /// Boss `b`'s errand this second: its goal, and its phase on the way
    /// (`Go`) or there (`Sit`); a wanderer that has sat out its `root`
    /// takes in the slime round it and sets off for the next. None: it has
    /// nothing to go for and hunts instead.
    fn errand(&mut self, b: &mut Boss, now: u64) -> Option<(f32, f32)> {
        let r = self.rules.boss;
        let sc = self.world.scale();
        let body = (b.members as f32 / std::f32::consts::PI).sqrt();
        if let (BossPhase::Sit(p, since), BossKind::Wanderer) = (b.phase, b.kind) {
            if now < since + (r.root.max(0.0) * self.rules.tps as f32).round() as u64 {
                return Some(p);
            }
            // Rooted long enough: it takes in the slime there and goes on.
            if let Some(j) = self.world.crystals().iter().position(|c| c.0 == p) {
                if j < 64 {
                    b.visited |= 1 << j;
                }
            }
            let reach = (body * r.grow / sc).clamp(1.0, 2.0 * r.reach);
            self.world.grow_lump(b.lump, (b.centre, reach));
        }
        // A sapper that has worn a hole through the wall where it sat goes
        // through it to the far side (`visited` 1), then hunts (2). The
        // first film's sapper sat on at its hole for 25 s after the slime
        // poured through; one that left when the wall's middle cell at its
        // spot broke slid along the wall, 45 s before the hole; one that
        // hunted from the hole slid along the wall too (the aim pulls
        // straight, the lump does not find paths).
        if b.kind == BossKind::Sapper {
            match (b.visited, b.phase) {
                (1, BossPhase::Go(p)) => {
                    if (p.0 - b.centre.0).hypot(p.1 - b.centre.1) > body.max(2.0 * sc) {
                        return Some(p);
                    }
                    b.visited = 2;
                    return None;
                }
                (1.., _) => return None,
                (_, BossPhase::Sit(q, _)) => {
                    if let Some(p) = self.hole_through(q, b.centre, body) {
                        b.visited = 1;
                        b.phase = BossPhase::Go(p);
                        return Some(p);
                    }
                }
                _ => {}
            }
        }
        // A goal it has holds while it stands (a wanderer's far crystal
        // would fall under `far` as it closes in; a sapper's nearest point
        // would slide along the wall to its end and round it, as the first
        // test run showed).
        let p = match b.phase {
            BossPhase::Go(q) | BossPhase::Sit(q, _) if self.goal_holds(b, q) => q,
            _ => self.boss_goal(b)?,
        };
        let reach = match b.kind {
            BossKind::Wanderer => self.world.crystals().iter().position(|c| c.0 == p).map_or(0.0, |j| self.world.crystal_r(j)),
            _ => 2.0 * sc,
        };
        let off = (p.0 - b.centre.0).hypot(p.1 - b.centre.1);
        // Once sitting it stays sat unless pushed well off (twice as far).
        b.phase = match b.phase {
            BossPhase::Sit(q, since) if q == p && off <= 2.0 * (body + reach) => BossPhase::Sit(q, since),
            _ if off <= body + reach => BossPhase::Sit(p, now),
            _ => BossPhase::Go(p),
        };
        Some(p)
    }

    /// Where a body at `from` of radius `body` goes through a hole in the
    /// standing wall through `q`: that far past the wall's far side, along
    /// its normal. None while the wall there is not holed: open ground all
    /// the way across it, from a half cell before it to a half cell past.
    fn hole_through(&self, q: (f32, f32), from: (f32, f32), body: f32) -> Option<(f32, f32)> {
        let near = |w: &super::Building| match w.kind {
            Kind::Wall { to } => {
                let n = nearest_on(w.at, to, q);
                ((n.0 - q.0).hypot(n.1 - q.1) < 0.5).then_some((w.at, to))
            }
            _ => None,
        };
        let (a, to) = self.buildings.iter().filter(|w| w.alive).find_map(near)?;
        let l = (to.0 - a.0).hypot(to.1 - a.1);
        if l <= 0.0 {
            return None;
        }
        // The normal, pointing away from the body.
        let n = (-(to.1 - a.1) / l, (to.0 - a.0) / l);
        let n = if (q.0 - from.0) * n.0 + (q.1 - from.1) * n.1 < 0.0 { (-n.0, -n.1) } else { n };
        let s = self.world.scale();
        let half = (0.5 * self.rules.wall_width + 0.5) * s;
        let steps = (4.0 * half / s).ceil().max(1.0) as i32;
        let open = (0..=steps).all(|i| {
            let o = -half + 2.0 * half * i as f32 / steps as f32;
            self.world.open(q.0 + n.0 * o, q.1 + n.1 * o)
        });
        open.then_some((q.0 + n.0 * (half + body), q.1 + n.1 * (half + body)))
    }

    /// Whether `q` is on a standing wall's line.
    fn on_wall(&self, q: (f32, f32)) -> bool {
        self.buildings.iter().filter(|w| w.alive).any(|w| match w.kind {
            Kind::Wall { to } => {
                let n = nearest_on(w.at, to, q);
                (n.0 - q.0).hypot(n.1 - q.1) < 0.5
            }
            _ => false,
        })
    }

    /// Whether goal `q` still stands for boss `b`: a wanderer's fresh
    /// crystal not yet rooted on, a point on a standing wall or a standing
    /// building for a sapper, a working pump for a leech.
    fn goal_holds(&self, b: &Boss, q: (f32, f32)) -> bool {
        let r = self.rules.boss;
        match b.kind {
            BossKind::Hunter => false,
            BossKind::Wanderer => self.world.crystals().iter().enumerate().any(|(j, c)| c.0 == q && (j >= 64 || b.visited & 1 << j == 0) && c.1 > 0.0 && c.1 >= r.fresh * self.start_charge.get(j).copied().unwrap_or(0.0)),
            BossKind::Sapper => self.on_wall(q) || self.buildings.iter().any(|w| w.alive && !matches!(w.kind, Kind::Wall { .. }) && w.at == q),
            BossKind::Leech => self.buildings.iter().any(|w| w.alive && w.kind == Kind::Pump && w.up() && w.at == q),
        }
    }

    /// The live leech sitting on building `k` (a pump), if any.
    /// The ring (fine cells) a leech sat at `p` holds round its pump: the
    /// pump's crystal's radius x `BossRules::ring`; 0 with none.
    fn leech_ring(&self, p: (f32, f32)) -> f32 {
        let r = self.rules.boss.ring;
        let pump = self.buildings.iter().find(|w| w.alive && w.kind == Kind::Pump && (w.at.0 - p.0).hypot(w.at.1 - p.1) < 1.0);
        match pump.and_then(|w| w.crystal) {
            Some(c) if r > 0.0 => self.world.crystal_r(c) * r,
            _ => 0.0,
        }
    }

    pub fn leech_on(&self, k: usize) -> Option<usize> {
        let at = self.buildings.get(k)?.at;
        self.bosses.live.iter().position(|b| b.kind == BossKind::Leech && matches!(b.phase, BossPhase::Sit(p, _) if (p.0 - at.0).hypot(p.1 - at.1) < 1.0))
    }

    /// The mould grid at a dish cell a square (`World::mould_grid`) with
    /// its size, if the world gives one.
    fn boss_grid(&self) -> Option<(Vec<u32>, usize, usize, usize)> {
        let bin = (self.world.scale() as usize).max(1);
        let g = self.world.mould_grid(bin)?;
        let (w, h) = self.world.size();
        let (gw, gh) = (w.div_ceil(bin), h.div_ceil(bin));
        (g.len() >= gw * gh && gw * gh > 0).then_some((g, gw, gh, bin))
    }

    /// A crowded knot (fine cells): of the squares whose square of side
    /// 2 `reach` + 1 round them is at least `crowd` full, one picked
    /// weighted by how full, away from the live mini-bosses.
    fn crowded_knot(&mut self) -> Option<(f32, f32)> {
        let r = self.rules.boss;
        let sc = self.world.scale();
        let (g, gw, gh, bin) = self.boss_grid()?;
        // Box sums by a summed-area table, a square of side 2 x reach + 1.
        let mut sat = vec![0u64; (gw + 1) * (gh + 1)];
        for y in 0..gh {
            for x in 0..gw {
                sat[(y + 1) * (gw + 1) + x + 1] = g[y * gw + x] as u64 + sat[y * (gw + 1) + x + 1] + sat[(y + 1) * (gw + 1) + x] - sat[y * (gw + 1) + x];
            }
        }
        let q = r.reach.max(1.0).round() as usize;
        let boxed = |x: usize, y: usize| {
            let (x0, y0, x1, y1) = (x.saturating_sub(q), y.saturating_sub(q), (x + q + 1).min(gw), (y + q + 1).min(gh));
            sat[y1 * (gw + 1) + x1] + sat[y0 * (gw + 1) + x0] - sat[y0 * (gw + 1) + x1] - sat[y1 * (gw + 1) + x0]
        };
        let away = 2.0 * r.reach * sc;
        // A full square: a walker on each of its fine cells.
        let full = ((2 * q + 1) * (2 * q + 1) * bin * bin) as f32;
        let floor = (r.crowd.clamp(0.0, 1.0) * full).ceil() as u64;
        let mut sums: Vec<(usize, u64)> = Vec::new();
        for y in q..gh.saturating_sub(q) {
            for x in q..gw.saturating_sub(q) {
                if g[y * gw + x] == 0 {
                    continue;
                }
                let b = boxed(x, y);
                if b < floor.max(1) {
                    continue;
                }
                let p = ((x as f32 + 0.5) * bin as f32, (y as f32 + 0.5) * bin as f32);
                if self.bosses.live.iter().any(|b| (b.centre.0 - p.0).hypot(b.centre.1 - p.1) < away) {
                    continue;
                }
                sums.push((y * gw + x, b));
            }
        }
        self.bosses.crowd_top = sums.iter().map(|s| s.1 as f32 / full).fold(0.0, f32::max);
        let total: u64 = sums.iter().map(|s| s.1).sum();
        let mut pick = (self.bosses.rng.f32() as f64 * total as f64) as u64;
        let i = sums.iter().find(|s| {
            if pick < s.1 {
                return true;
            }
            pick -= s.1;
            false
        })?;
        Some((((i.0 % gw) as f32 + 0.5) * bin as f32, ((i.0 / gw) as f32 + 0.5) * bin as f32))
    }

    /// The slime's parts (`parts`) on the mould grid, a crystal with food
    /// left feeding the parts it touches.
    pub fn slime_parts(&self) -> Vec<Part> {
        let Some((g, gw, gh, bin)) = self.boss_grid() else { return Vec::new() };
        let food: Vec<((f32, f32), f32)> = self.world.crystals().iter().enumerate().filter(|(_, c)| c.1 > 0.0).map(|(i, c)| (c.0, self.world.crystal_r(i))).collect();
        parts(&g, gw, gh, bin, &food)
    }

    /// **The mini-boss soldier `k` shoots first** (owner 04:58Z, round 1's
    /// question 6, "yes"): of the live mini-bosses whose body (a packed
    /// disc of its members, a walker a fine cell) comes within its next
    /// shot's reach (`Game::shot_reach`) and that it can see, the nearest;
    /// the point it aims at (the centre, or the body's near edge's line at
    /// that reach) and the body's radius (fine cells). A soldier with slime
    /// on it (`touching`) shoots that instead unless the mini-boss's body
    /// is within its defend reach (`Rules::defend`), so being eaten is
    /// never ignored for a body further off (review of the mini-boss PR).
    /// One function on top of the default aim so the gun thread's aim
    /// cards can put their own rule over it.
    pub fn boss_aim(&self, k: usize) -> Option<((f32, f32), f32)> {
        let s = self.soldiers[k];
        let sc = self.world.scale();
        let reach = self.shot_reach(k) * sc;
        let mine = if s.touching > 0 { self.rules.defend * sc } else { f32::INFINITY };
        self.bosses
            .live
            .iter()
            .filter(|b| b.members > 0)
            .map(|b| {
                let r = (b.members as f32 / std::f32::consts::PI).sqrt();
                let (dx, dy) = (b.centre.0 - s.x, b.centre.1 - s.y);
                (b.centre, r, dx.hypot(dy), (dx, dy))
            })
            .filter(|&(c, r, d, _)| d - r <= reach.min(mine) && self.clear_line((s.x, s.y), c))
            .min_by(|a, b| a.2.total_cmp(&b.2))
            .map(|(c, r, d, (dx, dy))| {
                let t = (reach / d.max(0.1)).min(1.0);
                (if t < 1.0 { (s.x + dx * t, s.y + dy * t) } else { c }, r)
            })
    }

    /// How far soldier `k`'s next shot flies, dish cells: its gun's reach
    /// times the range of the bullet it fires next (`Soldier::next` of its
    /// squad's casts, as `begin_step` fires them). A buckshot's 0.6 fell
    /// short of a mini-boss its gun's reach took in, so every shot hit
    /// empty ground (review of the mini-boss PR).
    pub fn shot_reach(&self, k: usize) -> f32 {
        let s = self.soldiers[k];
        let q = s.squad as usize;
        let m = self.mods(q);
        let ammo = if m.casts > 0 { m.ammo[(s.next % m.casts) as usize] } else { Default::default() };
        self.gun_range(q) * ammo.range()
    }

    /// **What the view reads of each lump** (for the look thread,
    /// 2026-11-16): 0 for a lump with no mini-boss; else bits 0-2 its kind
    /// (`BossKind`: 1 hunter, 2 wanderer, 3 sapper, 4 leech), bit 3 set once
    /// it leaves (hunting or on its errand; clear while it gathers),
    /// bits 4-7 its strength grade 0-15 (`Boss::grade`), bit 8 set while it
    /// fades after breaking up (`BossRules::fade`) and bits 9-15 how far
    /// through the fade, 0 to 127 (from the game's ticks, so it moves a
    /// frame at a time).
    pub fn lump_looks(&self) -> [u16; LUMPS] {
        let mut out = [0u16; LUMPS];
        let word = |b: &Boss| b.kind as u16 | (!matches!(b.phase, BossPhase::Gather) as u16) << 3 | ((b.grade.clamp(0.0, 1.0) * 15.0).round() as u16) << 4;
        for b in &self.bosses.live {
            out[b.lump] = word(b);
        }
        let (now, fade) = (self.world.ticks(), (self.rules.boss.fade.max(0.0) * self.rules.tps as f32).max(1.0));
        for (b, until) in &self.bosses.fading {
            let gone = (fade - until.saturating_sub(now) as f32).clamp(0.0, fade) / fade;
            out[b.lump] = word(b) | 1 << 8 | ((gone * 127.0).round() as u16) << 9;
        }
        out
    }

    /// The live soldier nearest `at` (fine cells).
    fn nearest_soldier(&self, at: (f32, f32)) -> Option<usize> {
        (0..self.soldiers.len()).filter(|&k| self.soldiers[k].alive).min_by(|&a, &b| {
            let d = |k: usize| (self.soldiers[k].x - at.0).hypot(self.soldiers[k].y - at.1);
            d(a).total_cmp(&d(b))
        })
    }
}

/// **The slime's parts**: the squares of a mould grid (`gw` x `gh`, `bin`
/// fine cells a side) holding walkers, joined side to side or corner to
/// corner in squares of `PART_BIN` grid squares (so a part's ends a square
/// apart are one part); each part's walkers, centre (fine cells), radius
/// (fine cells, of a packed disc of its walkers) and whether it touches one
/// of the `food` discs (centre and radius, fine cells: the crystals with
/// food left).
pub fn parts(g: &[u32], gw: usize, gh: usize, bin: usize, food: &[((f32, f32), f32)]) -> Vec<Part> {
    let (cw, ch) = (gw.div_ceil(PART_BIN), gh.div_ceil(PART_BIN));
    if cw * ch == 0 || g.len() < gw * gh {
        return Vec::new();
    }
    let mut cg = vec![0u32; cw * ch];
    for y in 0..gh {
        for x in 0..gw {
            cg[(y / PART_BIN) * cw + x / PART_BIN] += g[y * gw + x];
        }
    }
    // The coarse squares each crystal's disc and a square more covers.
    let side = (bin * PART_BIN) as f32;
    let mut fed = vec![false; cw * ch];
    for &((x, y), r) in food {
        let r = r + side;
        let (x0, x1) = (((x - r) / side).floor().max(0.0) as usize, (((x + r) / side).max(0.0) as usize).min(cw - 1));
        let (y0, y1) = (((y - r) / side).floor().max(0.0) as usize, (((y + r) / side).max(0.0) as usize).min(ch - 1));
        for cy in y0..=y1 {
            for cx in x0..=x1 {
                fed[cy * cw + cx] = true;
            }
        }
    }
    let mut seen = vec![false; cw * ch];
    let mut out = Vec::new();
    let mut stack = Vec::new();
    for start in 0..cw * ch {
        if seen[start] || cg[start] == 0 {
            continue;
        }
        seen[start] = true;
        stack.push(start);
        let (mut n, mut sx, mut sy, mut squares, mut on_food) = (0u64, 0.0f64, 0.0f64, 0usize, false);
        while let Some(i) = stack.pop() {
            let (x, y) = (i % cw, i / cw);
            let c = cg[i] as u64;
            n += c;
            sx += (x as f64 + 0.5) * c as f64;
            sy += (y as f64 + 0.5) * c as f64;
            squares += 1;
            on_food |= fed[i];
            for (dx, dy) in [(-1i64, -1i64), (0, -1), (1, -1), (-1, 0), (1, 0), (-1, 1), (0, 1), (1, 1)] {
                let (nx, ny) = (x as i64 + dx, y as i64 + dy);
                if nx < 0 || ny < 0 || nx >= cw as i64 || ny >= ch as i64 {
                    continue;
                }
                let j = ny as usize * cw + nx as usize;
                if !seen[j] && cg[j] > 0 {
                    seen[j] = true;
                    stack.push(j);
                }
            }
        }
        let centre = ((sx / n as f64) as f32 * side, (sy / n as f64) as f32 * side);
        out.push(Part { walkers: n as u32, centre, r: (n as f32 / std::f32::consts::PI).sqrt(), squares, fed: on_food });
    }
    out
}

/// The point of segment `a`-`b` nearest `p` (all fine cells).
fn nearest_on(a: (f32, f32), b: (f32, f32), p: (f32, f32)) -> (f32, f32) {
    let (dx, dy) = (b.0 - a.0, b.1 - a.1);
    let l2 = dx * dx + dy * dy;
    let t = if l2 > 0.0 { (((p.0 - a.0) * dx + (p.1 - a.1) * dy) / l2).clamp(0.0, 1.0) } else { 0.0 };
    (a.0 + dx * t, a.1 + dy * t)
}
