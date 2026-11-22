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

use super::raids::{down_way, Way};
use super::{Game, Kind, Rng, World};
use crate::mould::big::{Lump, LUMPS, RAID_BLOCK};
use crate::mould::squad::{Burst, BurstRules};

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
    /// `life` seconds of hunting, sitting on its errand or (a wanderer)
    /// roaming. A leech's or a sapper's walk to its errand counts against
    /// `trip` instead, so the walk does not eat the time it has there
    /// (2026-11-20: on the owner's own maps two of four leeches were spent
    /// on the way and none took any food).
    pub quorum: f32,
    pub life: f32,
    pub trip: f32,
    /// **Where it forms** (2026-11-20): a crowded knot is picked weighted
    /// by how full it is over 1 + (d / `near`)^2, d its distance in dish
    /// cells from the nearest soldier or building (0: by fullness alone).
    /// By fullness alone they formed deep in the body, 130 to 330 world
    /// units from the squad on the owner's maps, and most broke up or were
    /// spent before they reached anything.
    pub near: f32,
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
    /// **Late in a planet** (the owner, 11-20: a late-game army crushes
    /// mini-bosses without trouble; the balance thread's brief): once the
    /// slime is under `late` of its peak mass, each one's grade is drawn
    /// from at least `late_grade`, so the last ones are the strongest.
    /// The strikers' 0.4 (`StrikerRules::below`) was tried first: on the
    /// owner's three runs, planets 2 to 4 to 7:00, no boss formed with
    /// the slime that low (its least was 0.44). At 0.6, 2 of 24 did, at
    /// 351 s and 393 s.
    pub late: f32,
    pub late_grade: f32,
    /// **Late against a big army** (11-20, Direction's pick of the
    /// review's options A and B, after the big-army census: a boss walking
    /// into 30 or more soldiers lived 1 to 3 s at any grade or size). Scaled
    /// by the squad as the strikers are (`StrikerRules::few`, `full`: none
    /// against `few` soldiers or fewer, all of it from `full`):
    /// - A: the Mortar's weight in the draw rises by `late_mortar` x that
    ///   scale once the slime is late, from planet 2 (`kinds[4]` is 0 there
    ///   until then), so the late threat lands as shells;
    /// - B: from the full scale, a late hunter or pinwheel goes round the
    ///   squad to a building or pump out of its guns' reach (else to the
    ///   squad's far side) instead of charging its front, choosing the one
    ///   nearest it within `round_far` dish cells so the trip fits.
    pub late_mortar: f32,
    pub round_far: f32,
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
    /// these weights (hunter, wanderer, sapper, leech, mortar, pinwheel),
    /// among the kinds it has a target for (a sapper needs something
    /// built, a leech a pump, a wanderer a fresh crystal far off); a
    /// cut-off part makes only the runners (hunter, wanderer, pinwheel).
    /// `kind` 1-6 forces one (0 draws).
    pub kinds: [f32; 6],
    pub kind: f32,
    /// **The mortar** (the owner's pick, 2026-11-20 13:21Z, "Mortar
    /// first"): every `lob_every` seconds it throws the slime in a ring
    /// `shell` dish cells thick just outside its body at the nearest
    /// soldier or building within the guns' reach and `lob` dish cells
    /// more of its edge (it forms just outside the guns), each
    /// walker flying the same way (a push, `squad::Burst::push`), so the
    /// ring comes down round the target and lives on there. It does not
    /// walk, and takes half its grade's blows (`armour_at`): strong from
    /// afar, weak up close (the owner's balance rule, 11-18).
    pub lob: f32,
    pub lob_every: f32,
    /// **The pinwheel** (the owner's doc's second proposal, built
    /// 2026-11-20 after the Mortar): a hunter whose body gathers into
    /// three arms that turn about its centre at `spin` turns a second
    /// (`Lump::spin`; the way by the lump it holds, so two turn opposite
    /// ways) and sweep across the squad; its lunge is a faster turn
    /// (x `lunge_pull`) instead of a harder pull. From planet 3.
    pub spin: f32,
    /// The pinwheel's pace on top of its grade's (`haste_at`): leaning into
    /// its arms, members spend their steps going round, and without it
    /// one walked at 2.4 units a second against a hunter's 6.3 (median 2
    /// against 4) on the owner's planet 3 and 4 maps, and reached the
    /// squad 3 times of 7. At 3 a lump walking to an aim 40 cells off on
    /// the GPU test world went as far turning as plain (11.5 cells in
    /// 4 s; 2: 7.5), but on the owner's planet 4 the body tore apart along
    /// a rock (spread 1 to 4): its arms now turn only within `spin_near`
    /// dish cells of its prey (45: 90 world units at the run's scale 2;
    /// the first cut compared 90 unscaled, the review lane's catch), and
    /// this is a small push there.
    pub spin_haste: f32,
    pub spin_near: f32,
    pub shell: f32,
    /// **What a throw does** (2026-11-20, after the owner's "a late-game
    /// army crushes minibosses"): the ring thrown stays the mortar's for
    /// `shell_life` seconds, in a lump of its own (`Shell`) aimed at the
    /// target, with the mortar's bite and its armour, and its look, so it
    /// lands and bites as the boss's own; then it is let go into the
    /// slime. Let go as it was thrown (before), the ring landed as
    /// ordinary slime that touches as one walker and the squad shot it.
    pub shell_life: f32,
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
            // The mortar from planet 4 (the proposal the owner picked).
            kinds: [1.0, 1.0, 1.0, 1.0, if depth >= 4 { 1.0 } else { 0.0 }, if depth >= 3 { 1.0 } else { 0.0 }],
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
            trip: 90.0,
            near: 40.0,
            grade_lo: 0.0,
            grade_hi: 1.0,
            late: 0.6,
            late_grade: 0.8,
            late_mortar: 3.0,
            round_far: 160.0,
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
            kinds: [1.0, 1.0, 1.0, 1.0, 0.0, 0.0],
            kind: 0.0,
            lob: 30.0,
            lob_every: 5.0,
            spin: 0.2,
            spin_haste: 1.3,
            spin_near: 45.0,
            shell: 1.5,
            shell_life: 8.0,
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
    /// Sits where it formed and lobs the slime round it at the nearest
    /// soldier or building in reach (`BossRules::lob`).
    Mortar = 5,
    /// A hunter whose body turns in arms (`BossRules::spin`).
    Pinwheel = 6,
}

impl BossKind {
    pub const ALL: [BossKind; 6] = [BossKind::Hunter, BossKind::Wanderer, BossKind::Sapper, BossKind::Leech, BossKind::Mortar, BossKind::Pinwheel];

    pub fn name(self) -> &'static str {
        match self {
            BossKind::Hunter => "hunter",
            BossKind::Wanderer => "wanderer",
            BossKind::Sapper => "sapper",
            BossKind::Leech => "leech",
            BossKind::Mortar => "mortar",
            BossKind::Pinwheel => "pinwheel",
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
    /// Seconds of its `life` and its `trip` used (`BossRules::life`).
    pub lived: f32,
    pub walked: f32,
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
    /// Formed again on its core, the walkers there having taken it back
    /// first (`cores.rs`, 2026-11-20).
    Core,
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
    /// Each live one's way to where it is going (`Game::boss_steer`).
    ways: Vec<BossWay>,
    /// How many times one stalled on its way and gathered again (`STALL`).
    pub regrouped: u32,
    /// The mortars' throws still in flight or biting (`BossRules::
    /// shell_life`).
    pub shells: Vec<Shell>,
    /// Walkers thrown, and how many throws kept their own lump.
    pub thrown: u64,
    pub shelled: u32,
    /// Their own draws, apart from `Game::rng`.
    rng: Rng,
}

/// **A mortar's throw** while it is still the mortar's (`BossRules::
/// shell_life`): its lump, where it was thrown, its mortar's look word
/// (`Game::lump_looks`) and the tick it is let go.
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct Shell {
    pub lump: usize,
    pub target: (f32, f32),
    pub look: u16,
    pub until: u64,
}

/// **A mini-boss's way** (2026-11-20): the walking distance to its goal
/// (`big::raid_field_for`, on the raids' ground, `Game::slime_ground`), a
/// word a `RAID_BLOCK` block, built off the frame and set a game second
/// after it was started, as a raid's (`raids::Way`), so the game is the
/// same however fast the thread. A lump's pull is straight at its aim, and
/// on the owner's own maps (2026-11-20, nine of his planets 2 to 4, the
/// strategy bot) 11 of 20 mini-bosses stood 10 s or more against rock or a
/// fort's ring, one 64 s; a march walks down its way (`raids::down_way`)
/// and the mini-bosses now do too.
#[derive(Debug)]
struct BossWay {
    lump: usize,
    /// The goal the field leads to (fine cells; None: the nearest live
    /// soldier, a hunter's), and the one being built.
    goal: Option<(f32, f32)>,
    field: Vec<u32>,
    building: Option<(Way, Option<(f32, f32)>)>,
    /// Where it stood when it last got anywhere (fine cells) and the
    /// seconds since (`STALL`).
    mark: (f32, f32),
    still: u32,
}

/// **A body wrapped round rock gathers again on its leading part**
/// (2026-11-20): a lump steers its members by one aim, so a body that has
/// flowed round both sides of a rock pillar is pulled into the rock from
/// one side whatever the aim, its leash holding it on the rock between
/// (on the owner's planet 2, run 1795106014, a wanderer of 249 stood 90 s
/// wrapped round a pillar with its goal in plain sight of one arm). Not
/// two dish cells on in `STALL` seconds of walking a way, and with no
/// soldier near, it forms again round the point a body's length down its
/// way (one and a half times its radius round it), counted as a new body: the arm left behind goes back to the
/// slime, and the re-formed members' hits are healed (`World::form_lump`),
/// which is why it never happens in a fight.
const STALL: u32 = 6;

/// A goal that has moved this far (dish cells) from its way's gets a new
/// way (a goal changed: a leech's pump worn down, a sapper through its
/// hole). A hunter's way leads to every live soldier at once
/// (`big::units_field_for`) and is built again each second: a way to the
/// one it hunts was never used, as the nearest soldier changed from one
/// second to the next within the squad (on the bar map it pressed up the
/// rock's near side, its way always a second stale).
const WAY_MOVED: f32 = 6.0;
/// A mortar's thrown walker's speed x `step` (as `RoundRules::fling_speed`,
/// the shove cards' push).
const MORTAR_FLING: f32 = 1.5;
/// The share of a mortar's walkers it keeps when it throws: the rest of
/// its body, and `BossRules::shell` past it, is the throw.
const SHELL_CORE: f32 = 0.6;

impl Bosses {
    pub fn new(seed: u64) -> Bosses {
        Bosses { live: Vec::new(), ended: Vec::new(), fading: Vec::new(), stolen: 0.0, returned: 0.0, next: None, crowd_top: 0.0, ways: Vec::new(), regrouped: 0, shells: Vec::new(), thrown: 0, shelled: 0, rng: Rng::new(seed ^ 0xb055_5eed) }
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
        // Throws past their time are let go into the slime.
        self.bosses.shells.retain(|sh| {
            let keep = now < sh.until;
            if !keep {
                world.set_lump(sh.lump, Lump::default());
                world.form_lump(sh.lump, None);
            }
            keep
        });
        // The ways of those gone are dropped (a way still being built
        // finishes on its own thread and is thrown away).
        let held: Vec<usize> = self.bosses.live.iter().map(|b| b.lump).collect();
        self.bosses.ways.retain(|w| held.contains(&w.lump));
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
            } else if b.lived > r.life || b.walked > r.trip {
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
            let round = now >= hunt && matches!(b.kind, BossKind::Hunter | BossKind::Pinwheel) && self.boss_late() && self.army_scale() >= 1.0;
            let errand = if now < hunt || b.kind == BossKind::Mortar {
                None
            } else if round {
                self.round_errand(&mut b, now)
            } else if matches!(b.kind, BossKind::Hunter | BossKind::Pinwheel) {
                None
            } else {
                self.errand(&mut b, now)
            };
            let size = self.world.size();
            let cell = move |p: (f32, f32)| crate::mould::larder_world::raid_cell(p, size);
            let strong = Lump { armour: r.armour_at(b.grade), bite: r.bite_at(b.grade), ..Lump::default() };
            let leash = r.leash_at(r.leash, b.grade);
            let clench = r.leash_at(r.clench, b.grade);
            let haste = r.haste_at(b.grade);
            let go = move |p: (f32, f32)| Lump { aim: cell(p), pull: r.pull, leash, haste, ..strong };
            // On the way (an errand's walk, a hunt): down its way round
            // rock and the slime's own walls, not straight at it.
            let body = (members as f32 / std::f32::consts::PI).sqrt();
            let regrouped = self.bosses.regrouped;
            let lump = if now < hunt {
                b.phase = BossPhase::Gather;
                Lump { aim: cell(centre), leash: clench, ..strong }
            } else if b.kind == BossKind::Mortar {
                // Held on where it sat down (its aim its own centre let the
                // body drift with the slime round it: 28 cells in 11 s on
                // the CPU dish).
                let hold = Lump { aim: cell(b.at), pull: 0.5 * r.pull, leash: clench, armour: r.armour_at(0.5 * b.grade), ..strong };
                if self.mortar(&mut b, centre, body, now, secs(r.lob_every)) {
                    hold
                } else {
                    // Nothing in reach along a clear line: it walks toward
                    // the player's nearest thing until something is, on its
                    // trip's clock. Sitting where it formed, one on the
                    // owner's planet 4 (run 1795119328) never threw in
                    // 90 s, its nearest thing past its reach.
                    let near = self.player_things().into_iter().min_by(|a, c| (a.0 - centre.0).hypot(a.1 - centre.1).total_cmp(&(c.0 - centre.0).hypot(c.1 - centre.1)));
                    match near {
                        Some(p) => {
                            b.phase = BossPhase::Go(p);
                            go(self.boss_steer(b.lump, p, false, centre, body))
                        }
                        None => hold,
                    }
                }
            } else if let Some(p) = errand {
                // On its errand: going there, or sitting on it (a wanderer
                // rooted and eating, a leech holding on, both clenched; a
                // sapper pressing into the wall).
                match (b.phase, b.kind) {
                    (BossPhase::Sit(..), BossKind::Sapper | BossKind::Hunter | BossKind::Pinwheel) => go(p),
                    // Round the squad: aimed past it, not through its guns.
                    (_, BossKind::Hunter | BossKind::Pinwheel) => go(self.boss_steer(b.lump, self.round_squad(centre, p, body), false, centre, body)),
                    (BossPhase::Sit(..), kind) => {
                        // A leech rings its pump (`ring`): the pull still
                        // presses it in, onto the ring's inner edge.
                        let ring = if kind == BossKind::Leech { self.leech_ring(p) } else { 0.0 };
                        Lump { aim: cell(p), pull: 0.5 * r.pull, leash: clench, ring, ..strong }
                    }
                    _ => go(self.boss_steer(b.lump, p, false, centre, body)),
                }
            } else {
                let prey = self.nearest_soldier(centre);
                b.phase = BossPhase::Hunt(prey);
                match prey {
                    Some(s) => {
                        // The lunge: harder and faster for a moment.
                        let (pull, haste) = if r.lunging(now - hunt, tps) { (r.lunge_pull.max(0.0), r.lunge_haste.max(1.0)) } else { (1.0, 1.0) };
                        let aim = self.boss_steer(b.lump, (self.soldiers[s].x, self.soldiers[s].y), true, centre, body);
                        // A pinwheel's arms turn at one rate: the engine
                        // sets them at `spin` x its step count, so a faster
                        // turn for the lunge (tried first) jumped them round.
                        // Its arms only near its prey (`spin_near`): turning
                        // the whole way, leaning into the arms halved its
                        // pace (2.4 units a second against a hunter's 6.3,
                        // the squad reached 3 times of 7 on the owner's
                        // planet 3 and 4 maps), and three times the haste
                        // to make up for it tore the body apart on rock.
                        // Far off it walks as a compact hunter; close, it
                        // unfurls, which is also its tell.
                        let near = (self.soldiers[s].x - centre.0).hypot(self.soldiers[s].y - centre.1) < r.spin_near * self.world.scale();
                        let spin = if b.kind == BossKind::Pinwheel && near {
                            let way = if b.lump.is_multiple_of(2) { 1.0 } else { -1.0 };
                            way * r.spin * std::f32::consts::TAU / (tps * self.world.fine_steps().max(1) as f32)
                        } else {
                            0.0
                        };
                        let haste = if spin != 0.0 { haste * r.spin_haste.max(1.0) } else { haste };
                        Lump { aim: cell(aim), pull: r.pull * pull, leash, haste: r.haste_at(b.grade) * haste, spin, ..strong }
                    }
                    None => Lump { aim: cell(centre), leash, ..strong },
                }
            };
            self.world.set_lump(b.lump, lump);
            // Gathered again (`STALL`): counted again as a new body next
            // second, so a remnant under `least` fizzles and the quorum is
            // the new body's.
            if self.bosses.regrouped != regrouped {
                b.first = 0;
            }
            // A second of its life, or of its walk: a leech's or a
            // sapper's way to its errand (not a sapper's way on through
            // its hole, `visited` 1), a mortar's to a shot and a late
            // runner's round a big squad are its trip.
            if now >= hunt {
                match (b.phase, b.kind, b.visited) {
                    (BossPhase::Go(_), BossKind::Leech | BossKind::Sapper, 0) | (BossPhase::Go(_), BossKind::Mortar | BossKind::Hunter | BossKind::Pinwheel, _) => b.walked += 1.0,
                    _ => b.lived += 1.0,
                }
            }
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
        if let Some(at) = self.crowded_knot(None) {
            self.bosses.next = Some(now + gap(&mut self.bosses, r.every, r.every_sd));
            self.form_boss(at, r.reach, BossCause::Crowd);
        }
    }

    /// The ground a mini-boss walks and throws over: the slime's ground
    /// (`slime_ground`: rock, the slime's and the player's walls closed)
    /// with deep water closed too.
    fn boss_ground(&self) -> Option<(Vec<bool>, usize, usize)> {
        self.slime_ground().map(|(mut g, w, h)| {
            let open = self.world.open_cells();
            if open.len() == g.len() {
                for (a, o) in g.iter_mut().zip(open) {
                    *a &= o;
                }
            }
            (g, w, h)
        })
    }

    /// **A mortar's second** (`BossRules::lob`): its target, the nearest
    /// soldier or building past its body and within `lob` dish cells of its
    /// edge, kept in its phase (`Sit`, with the tick it last threw), and,
    /// `every` ticks after its last throw, the throw: the ring `shell`
    /// dish cells thick round its body pushed at the target as one (a
    /// fling with a heading, `squad::Burst::push`, every walker the same
    /// way for the same steps), so the ring lands round the target. Its
    /// walkers thrown are counted in `visited`.
    /// False with nothing in reach along a clear line: it walks then.
    fn mortar(&mut self, b: &mut Boss, centre: (f32, f32), body: f32, now: u64, every: u64) -> bool {
        let r = self.rules.boss;
        let sc = self.world.scale();
        let reach = body + (self.gun_range_max() + self.rules.panic_margin.max(0.0) + r.lob.max(0.0)) * sc;
        let d = |p: (f32, f32)| (p.0 - centre.0).hypot(p.1 - centre.1);
        // Only along a clear line: a thrown walker that meets rock, deep
        // water or a wall turns aside and lands there, and on the owner's
        // planet 4 (run 1795038793) the first throws filmed splashed on a
        // rock ridge between it and the squad. Your walls block it.
        let ground = self.boss_ground();
        let short = 4.0 * sc;
        let seen = |p: (f32, f32)| ground.as_ref().is_none_or(|(g, w, h)| line_clear(g, *w, *h, centre, p, short));
        let target = self.player_things().into_iter().filter(|&p| d(p) > body && d(p) <= reach && seen(p)).min_by(|a, c| d(*a).total_cmp(&d(*c)));
        let Some(t) = target else { return false };
        let last = match b.phase {
            BossPhase::Sit(_, t) => t,
            // Arrived from a walk (or first sat): held here from now on.
            _ => {
                b.at = centre;
                0
            }
        };
        b.phase = BossPhase::Sit(t, last);
        if last > 0 && now < last + every.max(1) {
            return true;
        }
        // `walker_step` is in fine cells, as the target's distance.
        let per = (self.world.walker_step() * MORTAR_FLING).max(1e-3);
        let steps = (d(t) / per).round().clamp(1.0, u16::MAX as f32) as u16;
        // The throw is its body's outer part, past the disc holding
        // `SHELL_CORE` of its walkers out to `shell` dish cells past its
        // edge, let go of first
        // so no member flies (a member thrown stays one, lands by the
        // target and runs home past its leash, dragging the centre after
        // it: a staged one of 47 on the CPU dish drifted 20 cells toward
        // the squad). From one and a half radii, its first version, a
        // throw took only the slime round it: after the first, the throws
        // on the owner's planet 4 (run 1795038793) took 1 walker each.
        // Measured on the mould grid, not from its walker count: a
        // mortar clenched on its spot packs its walkers two to a cell (a
        // spread of 0.55 of a packed disc's on the owner's planet 4), so
        // a ring from 0.7 to 1 of the packed radius held 7 to 12 of 244.
        let n = b.members.max(1) as f32;
        let inner = self.reach_holding(centre, (body / sc).max(1.0), SHELL_CORE * n);
        let outer = self.reach_holding(centre, (2.0 * body / sc).max(inner + 1.0), n).max(inner) + r.shell.max(0.1);
        self.world.form_lump(b.lump, Some((centre, inner)));
        // The ring a lump of its own (`Shell`), when one is free: it grows
        // from the walkers in no lump in the ring, flies as one (a flung
        // walker keeps its lump) and lands as the mortar's own, aimed at
        // the target, with its bite and its look.
        // Only while another lump stays free after it (the review lane,
        // 11-20): the world has `LUMPS` (4) for bosses, marches, fruits
        // and shells, and a mortar throwing every 5 s with shells living
        // 8 s held three, so a raid found none and streamed loose, a fruit
        // could not start and no other boss could form. Without a lump the
        // ring flies as ordinary slime, as before shells.
        let free: Vec<usize> = (0..crate::mould::big::LUMPS).filter(|&l| !self.lump_taken(l)).collect();
        let free = if free.len() >= 2 { free.first().copied() } else { None };
        if let Some(sl) = free {
            self.world.grow_lump(sl, (centre, outer));
            let cell = crate::mould::larder_world::raid_cell(t, self.world.size());
            // No leash: a ring walker the throw missed stays by the
            // mortar as a member, and with one the landed shell ran back
            // to its members' centre, halfway home, as a stream (filmed
            // on the owner's planet 4, run 1795038793 at 1:34). Its pull
            // brings every member to the target instead.
            self.world.set_lump(sl, Lump { aim: cell, pull: r.pull, armour: r.armour_at(0.5 * b.grade), bite: r.bite_at(b.grade), ..Lump::default() });
            let look = self.lump_looks()[b.lump];
            self.bosses.shells.push(Shell { lump: sl, target: t, look, until: now + ((r.shell_life.max(0.0) * self.rules.tps as f32).round() as u64).max(1) });
            self.bosses.shelled += 1;
        }
        let throw = Burst { at: (centre.0 / sc, centre.1 / sc), r: inner, fling: outer, fling_steps: steps, push: Some((t.1 - centre.1).atan2(t.0 - centre.0)), ..Burst::default() };
        let out = self.world.burst(&throw, &BurstRules { fling_speed: MORTAR_FLING, ..BurstRules::default() });
        b.visited += out.flung as u64;
        self.bosses.thrown += out.flung as u64;
        // Fed again from the slime round it while the shell holds the
        // thrown (none of them ordinary to take back): as far round as
        // holds its old body, which, as a regroup does, heals its
        // members' blows (`World::form_lump`): a mortar's blows count
        // between its throws only, and it takes half its grade's. With
        // no lump free for a shell, the thrown are ordinary walkers still
        // beside it, and stay let go.
        if free.is_some() {
            let keep = self.reach_holding(b.at, (1.5 * body / sc).max(1.0), std::f32::consts::PI * body * body);
            self.world.form_lump(b.lump, Some((b.at, keep)));
        }
        b.phase = BossPhase::Sit(t, now);
        true
    }

    /// **Where the mini-boss on `lump` steers for, going to `to`** (fine
    /// cells; its centre `centre`, its body's radius `body`, fine cells): a
    /// body's length and ten dish cells down its way (`BossWay`), as a
    /// march steers; straight at `to` until its way is built, where the
    /// way does not reach, or with no ground to walk (a test's world).
    /// Starts a new way when `to` has moved `WAY_MOVED` dish cells from
    /// the last one's goal.
    /// A hunter (`soldiers`) goes down a way to every live soldier, `to`
    /// the one it hunts. Within a look of the goal by its way it aims
    /// straight at `to`: a goal on a closed cell (a wall's line, a pump's
    /// rig) starts its way from the nearest open block, which may lie on
    /// the wall's far side, and a sapper steered for it slid along its
    /// wall instead of pressing into it (its test's hole came at 99 s
    /// against about 60).
    fn boss_steer(&mut self, lump: usize, to: (f32, f32), soldiers: bool, centre: (f32, f32), body: f32) -> (f32, f32) {
        let sc = self.world.scale();
        let goal = (!soldiers).then_some(to);
        let moved = |a: Option<(f32, f32)>| match (a, goal) {
            (Some(a), Some(to)) => (a.0 - to.0).hypot(a.1 - to.1) > WAY_MOVED * sc,
            (a, g) => a.is_some() != g.is_some(),
        };
        // Straight while nothing stands in the way (rock, the slime's own
        // walls, the player's walls and buildings) short of the goal's own
        // footprint: a way to a goal on a closed cell starts from the open
        // block nearest it, which may be on its far side, and a sapper
        // led by it round its wall's end ate at the end and holed it at
        // 99 s against 30 s straight.
        // Deep water closed too: the slime crawls there slowly, and on the
        // owner's planet 4 (run 1795038793) a hunter of 398 stood 70 s at a
        // river's edge on a raid's ground, which counts the river open.
        let ground = self.boss_ground();
        let i = match self.bosses.ways.iter().position(|w| w.lump == lump) {
            Some(i) => i,
            None => {
                self.bosses.ways.push(BossWay { lump, goal, field: Vec::new(), building: None, mark: centre, still: 0 });
                self.bosses.ways.len() - 1
            }
        };
        if ground.as_ref().is_none_or(|(g, w, h)| line_clear(g, *w, *h, centre, to, body + 3.0 * sc)) {
            // Its leading part is a body's length toward the goal. A clear
            // line is no promise: on the owner's planet 3 (run 1795119328)
            // a leech of 545 stood 54 s on open ground with its line to
            // the pump clear, its tail wedged in the packed slime behind
            // and its leash holding the head back to its centre.
            let d = (to.0 - centre.0).hypot(to.1 - centre.1).max(1e-3);
            let k = body.min(d) / d;
            self.boss_stall(i, centre, body, (centre.0 + (to.0 - centre.0) * k, centre.1 + (to.1 - centre.1) * k));
            return to;
        }
        // The way started a second ago is set.
        if let Some((way, g)) = self.bosses.ways[i].building.take() {
            let w = &mut self.bosses.ways[i];
            (w.field, w.goal) = (way.finish(), g);
        }
        let w = &self.bosses.ways[i];
        if soldiers || w.field.is_empty() || moved(w.goal) {
            let size = self.world.size();
            let way = if soldiers {
                let cells = self.soldiers.iter().filter(|s| s.alive).map(|s| crate::mould::larder_world::raid_cell((s.x, s.y), size)).collect();
                Way::start_units(ground.clone(), cells)
            } else {
                Way::start(ground.clone(), crate::mould::larder_world::raid_cell(to, size))
            };
            self.bosses.ways[i].building = Some((way, goal));
        }
        let w = &self.bosses.ways[i];
        if w.field.is_empty() || moved(w.goal) {
            return to;
        }
        let bw = self.world.size().0.div_ceil(RAID_BLOCK);
        let look = body + 10.0 * sc;
        // From the reached block nearest its centre within its body: a body
        // pressed on rock has its centre on the rock (on the bar map, 6
        // fine cells in), past `down_way`'s own two blocks.
        let Some((from, left)) = nearest_reached(&w.field, bw, centre, body) else { return to };
        if left as f32 <= look {
            return to;
        }
        // The farthest point down its way, within four looks of walking,
        // that it sees straight from where it stands (a string pulled
        // round the corner). A point a look down the way could still lie
        // behind the rock from most of its members, who are pulled straight
        // at it: on the owner's planet 2 (run 1795106014) a wanderer stood
        // 90 s on a rock ridge with its way round it.
        let seen = |p: (f32, f32)| ground.as_ref().is_none_or(|(g, gw, gh)| line_clear(g, *gw, *gh, from, p, 0.0));
        let ahead = way_points(&w.field, bw, from, 4.0 * look);
        let aim = ahead.iter().rev().find(|&&p| seen(p)).copied().or_else(|| down_way(&w.field, bw, from, look)).unwrap_or(to);
        // Stalled (`STALL`): it gathers again on its leading part.
        let lead = ahead.iter().take_while(|&&p| (p.0 - from.0).hypot(p.1 - from.1) <= body).last().copied().unwrap_or(from);
        self.boss_stall(i, centre, body, lead);
        aim
    }

    /// Counts the seconds the way's lump `i` has stood within two dish
    /// cells of where it last got anywhere, and at `STALL` with no soldier
    /// near gathers it again round `lead`, its leading part.
    fn boss_stall(&mut self, i: usize, centre: (f32, f32), body: f32, lead: (f32, f32)) {
        let sc = self.world.scale();
        let w = &mut self.bosses.ways[i];
        if (centre.0 - w.mark.0).hypot(centre.1 - w.mark.1) > 2.0 * sc {
            (w.mark, w.still) = (centre, 0);
        } else {
            w.still += 1;
        }
        let fight = (self.gun_range_max() + self.rules.panic_margin.max(0.0)) * sc + body;
        let near = self.soldiers.iter().any(|s| s.alive && (s.x - centre.0).hypot(s.y - centre.1) < fight);
        if self.bosses.ways[i].still >= STALL && !near {
            // As far round as holds no more walkers than it has: forming
            // takes every walker in reach, and in a thick body a first cut
            // at one and a half radii took in the slime round it (a hunter
            // of 437 on the owner's planet 3 came out at 4,630). Up to three
            // radii now (11-22): a hunter of 240 wrapped round a rock ridge
            // on planet 2 (run 1795106014) spread to three times its radius,
            // and each regroup capped at one and a half kept only what lay
            // in that, 240 to 71 to 16, until it fizzled without a fight.
            // It stops short of its size (`reach_under`), since from three
            // radii the bin that crossed it grew it each time.
            let reach = self.reach_under(lead, (3.0 * body / sc).max(1.0), std::f32::consts::PI * body * body);
            let lump = self.bosses.ways[i].lump;
            self.world.form_lump(lump, Some((lead, reach)));
            (self.bosses.ways[i].mark, self.bosses.ways[i].still) = (lead, 0);
            self.bosses.regrouped += 1;
        }
    }

    /// The largest reach round `at` (dish cells), up to `most`, whose
    /// walkers on the mould grid number no more than `want` (at least one
    /// dish cell); `most` with no grid. It reaches to the grid bin that
    /// crosses `want`, so it takes that bin's walkers too.
    fn reach_holding(&self, at: (f32, f32), most: f32, want: f32) -> f32 {
        self.ring_holding(at, most, want).1
    }

    /// As `reach_holding`, but stopping at the last bin short of `want`, so
    /// what it reaches never holds more: the regroup's (11-22), which
    /// reaching past it grew a hunter of 240 to 539 and one of 321 to 873.
    fn reach_under(&self, at: (f32, f32), most: f32, want: f32) -> f32 {
        self.ring_holding(at, most, want).0
    }

    /// The reach short of `want` walkers round `at` and the reach that
    /// crosses it (dish cells, each at least one and at most `most`).
    fn ring_holding(&self, at: (f32, f32), most: f32, want: f32) -> (f32, f32) {
        let Some((g, gw, gh, bin)) = self.boss_grid() else { return (most, most) };
        let sc = self.world.scale();
        let b = bin as f32;
        let mut near: Vec<(f32, u32)> = Vec::new();
        let r = most * sc;
        let (x0, x1) = (((at.0 - r) / b).floor().max(0.0) as usize, (((at.0 + r) / b) as usize).min(gw - 1));
        let (y0, y1) = (((at.1 - r) / b).floor().max(0.0) as usize, (((at.1 + r) / b) as usize).min(gh - 1));
        for y in y0..=y1 {
            for x in x0..=x1 {
                let d = ((x as f32 + 0.5) * b - at.0).hypot((y as f32 + 0.5) * b - at.1);
                if d <= r && g[y * gw + x] > 0 {
                    near.push((d, g[y * gw + x]));
                }
            }
        }
        near.sort_by(|a, c| a.0.total_cmp(&c.0));
        // Short of `want` is the last ring nearer than the crossing bin's:
        // bins come in rings of four or eight at one distance, and stopping
        // at the bin before it on the same ring reaches as far.
        let mut held = 0.0;
        let (mut short, mut ring) = (1.0f32, 0.0f32);
        for &(d, n) in &near {
            if d > ring {
                (short, ring) = ((ring / sc).clamp(1.0, most), d);
            }
            held += n as f32;
            if held > want {
                return (short.min(most), (d / sc).clamp(1.0, most));
            }
        }
        (most, most)
    }

    /// Forms a mini-boss round `at` (fine cells), of the walkers within
    /// `reach` dish cells (by its grade, `BossRules::reach_at`), on the
    /// first free lump, its grade drawn (`BossRules::grade_lo`); false with
    /// none free.
    pub fn form_boss(&mut self, at: (f32, f32), reach: f32, cause: BossCause) -> bool {
        // Its kind first. A crowd's leech or sapper forms at the crowded
        // knot nearest its pump or wall (Direction's call, 2026-11-20): from
        // the knot nearest the player their errands were 150 to 250 world
        // units off, walked at 1 to 4 units a second, and none of six on
        // the owner's nine maps got there inside its `trip`. Drawing them
        // only with a target within reach (Direction's first rule) drew
        // none at all: no pump or wall stood within 180 units of a knot.
        let now = self.world.ticks();
        let r = self.rules.boss;
        let mut b = Boss { lump: 0, born: now, at, cause, phase: BossPhase::Gather, first: 0, members: 0, centre: at, grade: 0.0, kind: BossKind::Hunter, stolen: 0.0, visited: 0, lived: 0.0, walked: 0.0 };
        b.kind = self.pick_kind(&b);
        let mut at = at;
        // A mortar likewise takes the crowded knot nearest the player's
        // things, which (never inside the guns, `crowded_knot`) is at the
        // edge of their reach: from the knot weighted by nearness the first
        // one filmed sat 260 units off with nothing to throw at.
        if cause == BossCause::Crowd && matches!(b.kind, BossKind::Leech | BossKind::Sapper | BossKind::Mortar) {
            if let Some(p) = self.crowded_knot(Some(&b)) {
                at = p;
            }
        }
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
        let (mut lo, mut hi) = (r.grade_lo.clamp(0.0, 1.0), r.grade_hi.clamp(0.0, 1.0));
        if self.boss_late() {
            lo = lo.max(r.late_grade.clamp(0.0, 1.0));
            hi = hi.max(lo);
        }
        let grade = lo + (hi - lo).max(0.0) * self.bosses.rng.f32();
        self.world.form_lump(lump, Some((at, r.reach_at(reach, grade))));
        let cell = crate::mould::larder_world::raid_cell(at, self.world.size());
        self.world.set_lump(lump, Lump { aim: cell, leash: r.leash_at(r.clench, grade), armour: r.armour_at(grade), bite: r.bite_at(grade), ..Lump::default() });
        (b.lump, b.at, b.centre, b.grade) = (lump, at, at, grade);
        self.bosses.live.push(b);
        true
    }

    /// Its kind (`BossRules::kinds`): forced by `kind`, else drawn by the
    /// weights among the kinds with a target from where it formed (a
    /// cut-off part: the runners only).
    pub(super) fn pick_kind(&mut self, b: &Boss) -> BossKind {
        let r = self.rules.boss;
        let forced = r.kind.round() as usize;
        if (1..=BossKind::ALL.len()).contains(&forced) {
            return BossKind::ALL[forced - 1];
        }
        let army = if self.boss_late() { self.army_scale() } else { 0.0 };
        let w: Vec<f32> = BossKind::ALL
            .iter()
            .enumerate()
            .map(|(i, &kind)| {
                let runner = matches!(kind, BossKind::Hunter | BossKind::Wanderer | BossKind::Pinwheel);
                let able = matches!(kind, BossKind::Hunter | BossKind::Mortar | BossKind::Pinwheel) || self.boss_goal(&Boss { kind, ..*b }).is_some();
                let late = if kind == BossKind::Mortar { r.late_mortar.max(0.0) * army } else { 0.0 };
                if (b.cause != BossCause::Cut || runner) && able { r.kinds[i].max(0.0) + late } else { 0.0 }
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

    /// The squad's size on the strikers' scale (`StrikerRules::few`,
    /// `full`): 0 at `few` live soldiers or fewer, 1 from `full`.
    pub fn army_scale(&self) -> f32 {
        let r = self.rules.strikers;
        let alive = self.soldiers.iter().filter(|s| s.alive).count() as f32;
        ((alive - r.few) / (r.full - r.few).max(1.0)).clamp(0.0, 1.0)
    }

    /// The live soldiers' centre and the farthest of them from it (fine
    /// cells), and the guns' reach round them; None with no soldier.
    fn squad_ring(&self) -> Option<((f32, f32), f32)> {
        let live: Vec<(f32, f32)> = self.soldiers.iter().filter(|s| s.alive).map(|s| (s.x, s.y)).collect();
        if live.is_empty() {
            return None;
        }
        let n = live.len() as f32;
        let c = (live.iter().map(|p| p.0).sum::<f32>() / n, live.iter().map(|p| p.1).sum::<f32>() / n);
        let spread = live.iter().map(|p| (p.0 - c.0).hypot(p.1 - c.1)).fold(0.0, f32::max);
        let fight = (self.gun_range_max() + self.rules.panic_margin.max(0.0)) * self.world.scale();
        Some((c, spread + fight))
    }

    /// Where a body at `from` of radius `body` aims on its way to `to`
    /// round the squad (`squad_ring`): `to` while the straight line keeps
    /// clear of the ring, else the point beside the ring on `from`'s side.
    fn round_squad(&self, from: (f32, f32), to: (f32, f32), body: f32) -> (f32, f32) {
        let Some((c, ring)) = self.squad_ring() else { return to };
        let ring = ring + body;
        let n = nearest_on(from, to, c);
        if (n.0 - c.0).hypot(n.1 - c.1) >= ring || (from.0 - c.0).hypot(from.1 - c.1) < ring {
            return to;
        }
        // Off the line through the centre, on the side `from` is on.
        let (dx, dy) = (to.0 - from.0, to.1 - from.1);
        let l = dx.hypot(dy).max(1e-3);
        let mut side = (-dy / l, dx / l);
        if (from.0 - c.0) * side.0 + (from.1 - c.1) * side.1 < 0.0 {
            side = (-side.0, -side.1);
        }
        (c.0 + side.0 * 1.2 * ring, c.1 + side.1 * 1.2 * ring)
    }

    /// A late boss's errand round a big squad (`BossRules::round_far`):
    /// the standing building or pump nearest it that no soldier's guns
    /// reach, within `round_far`, held while it stands and stays out of
    /// reach; else the squad's far side, from which it hunts. None: it
    /// hunts as before.
    pub(super) fn round_errand(&mut self, b: &mut Boss, now: u64) -> Option<(f32, f32)> {
        let r = self.rules.boss;
        let sc = self.world.scale();
        let (c, ring) = self.squad_ring()?;
        let fight = (self.gun_range_max() + self.rules.panic_margin.max(0.0)) * sc;
        let body = (b.members as f32 / std::f32::consts::PI).sqrt();
        let soldiers: Vec<(f32, f32)> = self.soldiers.iter().filter(|s| s.alive).map(|s| (s.x, s.y)).collect();
        let clear = |p: (f32, f32)| soldiers.iter().all(|q| (q.0 - p.0).hypot(q.1 - p.1) > fight);
        let d = |p: (f32, f32)| (p.0 - b.centre.0).hypot(p.1 - b.centre.1);
        let standing = |q: (f32, f32)| self.buildings.iter().any(|w| w.alive && !matches!(w.kind, Kind::Wall { .. }) && w.at == q);
        let held = match b.phase {
            BossPhase::Go(q) | BossPhase::Sit(q, _) if standing(q) && clear(q) => Some(q),
            _ => None,
        };
        let p = held.or_else(|| {
            self.buildings
                .iter()
                .filter(|w| w.alive && !matches!(w.kind, Kind::Wall { .. }) && clear(w.at) && d(w.at) <= r.round_far * sc)
                .map(|w| w.at)
                .min_by(|a, q| d(*a).total_cmp(&d(*q)))
        });
        let p = match p {
            Some(p) => p,
            None => {
                // The far side, once: there it hunts (`visited` 1).
                if b.visited != 0 {
                    return None;
                }
                let (dx, dy) = (c.0 - b.centre.0, c.1 - b.centre.1);
                let l = dx.hypot(dy).max(1e-3);
                let far = (c.0 + dx / l * (ring + body), c.1 + dy / l * (ring + body));
                if !self.world.open(far.0, far.1) || d(far) > r.round_far * sc {
                    return None;
                }
                if (far.0 - b.centre.0).hypot(far.1 - b.centre.1) <= body + 2.0 * sc {
                    b.visited = 1;
                    return None;
                }
                b.phase = BossPhase::Go(far);
                return Some(far);
            }
        };
        let off = d(p);
        b.phase = match b.phase {
            BossPhase::Sit(q, since) if q == p && off <= 2.0 * (body + 2.0 * sc) => BossPhase::Sit(q, since),
            _ if off <= body + 2.0 * sc => BossPhase::Sit(p, now),
            _ => BossPhase::Go(p),
        };
        Some(p)
    }

    /// Where boss `b`'s errand takes it (fine cells): a wanderer's next
    /// fresh crystal far off and not rooted on before, a sapper's nearest
    /// point of a standing wall (else the nearest building), a leech's
    /// nearest working pump. None for a hunter, or with nothing to go for.
    pub fn boss_goal(&self, b: &Boss) -> Option<(f32, f32)> {
        let d = |p: (f32, f32)| (p.0 - b.centre.0).hypot(p.1 - b.centre.1);
        let near = |ps: Vec<(f32, f32)>| ps.into_iter().min_by(|a, c| d(*a).total_cmp(&d(*c)));
        match b.kind {
            BossKind::Hunter | BossKind::Mortar | BossKind::Pinwheel => None,
            BossKind::Wanderer => {
                // A hop toward the player (2026-11-20): the nearest fresh
                // crystal to it of those nearer the player's things than it
                // is, else the one nearest them, so it comes crystal by
                // crystal, growing at each. Until then the nearest to
                // itself, and on the owner's own maps every wanderer grazed
                // the far side of the slime and never came within 55 world
                // units of a soldier. With nothing of the player's, the
                // nearest to itself.
                let r = self.rules.boss;
                let far = r.far * self.world.scale();
                let cs = self.world.crystals();
                let ok: Vec<(f32, f32)> = cs.iter().enumerate().filter(|&(j, _)| j >= 64 || b.visited & 1 << j == 0).filter(|&(j, c)| c.1 >= r.fresh * self.start_charge.get(j).copied().unwrap_or(0.0) && c.1 > 0.0 && d(c.0) >= far).map(|(_, c)| c.0).collect();
                let mine = self.player_things();
                let to_mine = |p: (f32, f32)| mine.iter().map(|q| (q.0 - p.0).hypot(q.1 - p.1)).fold(f32::INFINITY, f32::min);
                let toward: Vec<(f32, f32)> = ok.iter().copied().filter(|&c| to_mine(c) < to_mine(b.centre)).collect();
                if mine.is_empty() {
                    near(ok)
                } else if toward.is_empty() {
                    ok.into_iter().min_by(|a, c| to_mine(*a).total_cmp(&to_mine(*c)))
                } else {
                    near(toward)
                }
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
            BossKind::Hunter | BossKind::Mortar | BossKind::Pinwheel => false,
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
    /// weighted by how full, away from the live mini-bosses. With `errand`
    /// (a leech or sapper drawn there), the one nearest its pump or wall
    /// instead (`Game::form_boss`).
    fn crowded_knot(&mut self, errand: Option<&Boss>) -> Option<(f32, f32)> {
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
        // Nearer the player's things more likely (`BossRules::near`).
        // Never within the squad's guns (and the panic margin past them):
        // it clenches first, its tell, and one that formed on top of the
        // owner's 78 soldiers on his planet 4 (run 1795038793) was shot to
        // nothing in 3 s, still clenching.
        let mine = self.player_things();
        let fight = (self.gun_range_max() + self.rules.panic_margin.max(0.0)) * sc;
        let at = |i: usize| (((i % gw) as f32 + 0.5) * bin as f32, ((i / gw) as f32 + 0.5) * bin as f32);
        let shot = |p: (f32, f32)| self.soldiers.iter().any(|s| s.alive && (s.x - p.0).hypot(s.y - p.1) < fight);
        let weights: Vec<f64> = sums
            .iter()
            .map(|&(i, b)| {
                if shot(at(i)) {
                    return 0.0;
                }
                let d = mine.iter().map(|q| (q.0 - at(i).0).hypot(q.1 - at(i).1)).fold(f32::INFINITY, f32::min) / sc;
                let w = if r.near > 0.0 && d.is_finite() { 1.0 / (1.0 + (d / r.near).powi(2)) } else { 1.0 };
                b as f64 * w as f64
            })
            .collect();
        if let Some(e) = errand {
            // A mortar's goal is the player's nearest thing.
            // (one it can throw at: a clear line, `Game::mortar`).
            let ground = if e.kind == BossKind::Mortar { self.boss_ground() } else { None };
            let short = 4.0 * sc;
            let to_goal = |p: (f32, f32)| match e.kind {
                BossKind::Mortar => mine.iter().filter(|q| ground.as_ref().is_none_or(|(g, w, h)| line_clear(g, *w, *h, p, **q, short))).map(|q| (q.0 - p.0).hypot(q.1 - p.1)).fold(f32::INFINITY, f32::min),
                _ => self.boss_goal(&Boss { centre: p, ..*e }).map_or(f32::INFINITY, |g| (g.0 - p.0).hypot(g.1 - p.1)),
            };
            let i = sums.iter().zip(&weights).filter(|(_, &w)| w > 0.0).map(|(s, _)| (s.0, to_goal(at(s.0)))).filter(|s| s.1.is_finite()).min_by(|a, c| a.1.total_cmp(&c.1))?.0;
            return Some(at(i));
        }
        let total: f64 = weights.iter().sum();
        let mut pick = self.bosses.rng.f32() as f64 * total;
        let i = sums.iter().zip(&weights).find(|(_, &w)| {
            if pick < w {
                return true;
            }
            pick -= w;
            false
        })?.0;
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
    ///
    /// The origin too (`origin.rs`, 11-22, Engine's siege table on #699):
    /// soldiers beside it on Fight shot the densest slime in reach, the
    /// body round it, not its members, so it seldom lost the 2% a second
    /// that stops its regrowth (run seed 2's planet 1: in range from 2:50
    /// to 12:00, it held 2,188 of 2,314). Only for a soldier within its
    /// reach (`OriginRules::near` past its crystal's rim, where it lunges),
    /// so it never draws fire from across the map.
    pub fn boss_aim(&self, k: usize) -> Option<((f32, f32), f32)> {
        let s = self.soldiers[k];
        let sc = self.world.scale();
        let reach = self.shot_reach(k) * sc;
        let mine = if s.touching > 0 { self.rules.defend * sc } else { f32::INFINITY };
        let origin = self.stand.live.filter(|h| h.origin && h.members > 0 && self.origin_reaches(k)).map(|h| (h.centre, h.members));
        self.bosses
            .live
            .iter()
            .filter(|b| b.members > 0)
            .map(|b| (b.centre, b.members))
            .chain(origin)
            .map(|(centre, members)| {
                let r = (members as f32 / std::f32::consts::PI).sqrt();
                let (dx, dy) = (centre.0 - s.x, centre.1 - s.y);
                (centre, r, dx.hypot(dy), (dx, dy))
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
        for sh in &self.bosses.shells {
            out[sh.lump] = sh.look;
        }
        let (now, fade) = (self.world.ticks(), (self.rules.boss.fade.max(0.0) * self.rules.tps as f32).max(1.0));
        for (b, until) in &self.bosses.fading {
            let gone = (fade - until.saturating_sub(now) as f32).clamp(0.0, fade) / fade;
            out[b.lump] = word(b) | 1 << 8 | ((gone * 127.0).round() as u16) << 9;
        }
        out
    }

    /// Late in the planet (`BossRules::late`): the slime under that share
    /// of its peak mass.
    pub fn boss_late(&self) -> bool {
        self.peak > 0 && (self.mould as f32) < self.rules.boss.late * self.peak as f32
    }

    /// Where the player's things stand (fine cells): live soldiers and
    /// standing buildings.
    fn player_things(&self) -> Vec<(f32, f32)> {
        self.soldiers.iter().filter(|s| s.alive).map(|s| (s.x, s.y)).chain(self.buildings.iter().filter(|w| w.alive).map(|w| w.at)).collect()
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

/// The middles of the blocks (fine cells) down `field` (a way, `bw`
/// blocks a row) from the block at `from`, each step to the neighbour
/// nearest the goal, for `look` fine cells of walking or to the goal (as
/// `raids::down_way` walks it).
fn way_points(field: &[u32], bw: usize, from: (f32, f32), look: f32) -> Vec<(f32, f32)> {
    let bh = field.len() / bw.max(1);
    let b = RAID_BLOCK as f32;
    let at = |x: i64, y: i64| (x >= 0 && y >= 0 && (x as usize) < bw && (y as usize) < bh).then(|| field[y as usize * bw + x as usize]).filter(|&d| d != u32::MAX);
    let (mut x, mut y) = ((from.0 / b).floor() as i64, (from.1 / b).floor() as i64);
    let mut out = Vec::new();
    for _ in 0..(look / b).ceil().max(0.0) as usize {
        let Some(here) = at(x, y) else { break };
        let Some((d, nx, ny)) = (-1..=1i64).flat_map(|dy| (-1..=1i64).map(move |dx| (dx, dy))).filter(|&o| o != (0, 0)).filter_map(|(dx, dy)| at(x + dx, y + dy).map(|d| (d, x + dx, y + dy))).min_by_key(|&(d, _, _)| d) else { break };
        if d >= here {
            break;
        }
        (x, y) = (nx, ny);
        out.push(((x as f32 + 0.5) * b, (y as f32 + 0.5) * b));
    }
    out
}

/// Whether the straight line from `from` to `to` (fine cells) crosses only
/// open cells of `ground` (`w` x `h`), up to `short` fine cells before `to`.
fn line_clear(ground: &[bool], w: usize, h: usize, from: (f32, f32), to: (f32, f32), short: f32) -> bool {
    let l = (to.0 - from.0).hypot(to.1 - from.1);
    let n = (l - short).floor();
    if n <= 0.0 {
        return true;
    }
    let (ux, uy) = ((to.0 - from.0) / l, (to.1 - from.1) / l);
    (0..=n as usize).all(|i| {
        let (x, y) = (from.0 + ux * i as f32, from.1 + uy * i as f32);
        let (x, y) = (x.max(0.0) as usize, y.max(0.0) as usize);
        x >= w || y >= h || ground[y * w + x]
    })
}

/// The middle of the block of `field` (a way, `RAID_BLOCK` blocks, `bw` a
/// row) nearest `at` (fine cells) that the way reaches, within `r` fine
/// cells and two blocks more, and its distance there.
fn nearest_reached(field: &[u32], bw: usize, at: (f32, f32), r: f32) -> Option<((f32, f32), u32)> {
    let bh = field.len() / bw.max(1);
    let b = RAID_BLOCK as f32;
    let (fx, fy) = ((at.0 / b).floor() as i64, (at.1 / b).floor() as i64);
    let q = (r / b).ceil() as i64 + 2;
    (-q..=q)
        .flat_map(|dy| (-q..=q).map(move |dx| (dx, dy)))
        .filter_map(|(dx, dy)| {
            let (x, y) = (fx + dx, fy + dy);
            let d = (x >= 0 && y >= 0 && (x as usize) < bw && (y as usize) < bh).then(|| field[y as usize * bw + x as usize]).filter(|&d| d != u32::MAX)?;
            Some((dx * dx + dy * dy, d, x, y))
        })
        .min_by_key(|&(o, d, _, _)| (o, d))
        .map(|(_, d, x, y)| (((x as f32 + 0.5) * b, (y as f32 + 0.5) * b), d))
}

/// The point of segment `a`-`b` nearest `p` (all fine cells).
fn nearest_on(a: (f32, f32), b: (f32, f32), p: (f32, f32)) -> (f32, f32) {
    let (dx, dy) = (b.0 - a.0, b.1 - a.1);
    let l2 = dx * dx + dy * dy;
    let t = if l2 > 0.0 { (((p.0 - a.0) * dx + (p.1 - a.1) * dy) / l2).clamp(0.0, 1.0) } else { 0.0 };
    (a.0 + dx * t, a.1 + dy * t)
}
