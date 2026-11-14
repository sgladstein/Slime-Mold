//! The game's rules: `Rules` and the mould's core (`Core`, `CoreKind`).
//! Moved out of `larder_world.rs` unchanged (C9, 2026-11-14).

use super::*;

/// What the mould's core is, and what it does while a squad holds it
/// (core defence, PR 3, 2026-11-12; the shape is the tiers thread's, so
/// each tier can say its own through `Tier::core`). A mould with no tier
/// of its own gets `Core::of(&rules)`: headless, with `Rules::core_pull`.
///
/// **No king by default** (owner, 2026-11-12 20:21Z: "remove the king rule.
/// As long as the slime has food/crystal energy it can make walkers.
/// Headless."). Before, a squad that held the core clear for 5 s stopped
/// births on the whole map for good; in the 64-seed race census that was
/// the only way any plan crippled the mould (rush took it 34 times, strike
/// 12; farm, wall and fort reached the cap 64 of 64), so the best plan was
/// always to walk at the core (slime-enemy review,
/// `Reports/slime-enemy-review-2026-11-12.md`). `CoreKind::Fixed` stays
/// for a tier that wants one fatal weak spot. The pull stays: the mould
/// still closes over soldiers at its home, which is what kept random from
/// walking into the hub (below).
///
/// Why a pull: with craters on (#113) the random hand wiped the plain
/// mould on 11 of 32 race seeds (201-232; 5 of 32 without craters) and no
/// plan beat it after Holm. In the window (seed 204) its soldiers, sent
/// one at a time on Attack, walk into the hub in the first minute and
/// fire into the densest walkers, killing about 1,200 in 30 s, while the
/// holes around them don't refill and the body never comes for them;
/// farm and strike fight at the network's edge. With soldiers at the
/// core smelling like a lure the hub closes over them. Over both seed
/// blocks (race 5-36 and 201-232, 15-minute games, against the same code
/// at pull 0): random took the core on 5 of 64 games (18 without) and
/// wiped the mould on 5 (18 without); every plan beats random 29-3 or
/// better on each block, all under Holm (201-232 had none without); farm,
/// wall, fort and idle play the same within chance; strike, the planned
/// core attack, still takes it on 12 (13 without) and scores higher on
/// 45 of 64 (17 lower); and a rush that fails to take it loses its squad
/// on 8 of the 10 failures on 201-232.
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct Core {
    pub kind: CoreKind,
    /// Game seconds a squad must hold the core with no walker touching it
    /// before it falls.
    pub secs: f32,
    /// A soldier within twice `Rules::core_r` of the core lays this many
    /// times its own scent over `BuildRules::lure_r` (a lure's reach): the
    /// body is drawn onto a squad at its core through the scent it already
    /// follows, nothing drawn. 0 is none. A headless core pulls too (since
    /// headless became the default); the Sprawler sets 0.
    pub pull: f32,
}

/// How a core falls (`Core`).
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum CoreKind {
    /// Held clear for `Core::secs`, it falls for good: births stop.
    Fixed,
    /// No core: nothing to hold, so only a wipe ends the mould; births go
    /// on wherever a walker is fed. The default since 2026-11-12 20:21Z
    /// (`Core::of`); first the tiers thread's Sprawler. The Drifter's
    /// migrating core is to come: it needs to know where the body is on
    /// the GPU, which the GPU world can't read back cheaply yet.
    ///
    /// A third kind, `Regrow { starve_secs }`, was built and merged in
    /// PR #163 (the tiers thread's Trunk: the core falls, and grows back
    /// unless the whole body eats nothing for `starve_secs`), then taken
    /// out: the owner's no-king ruling (2026-11-12 20:21Z, "remove the
    /// king rule ... Headless") left no tier to use it. Its code is in
    /// #163's merge if a core comes back (`Reports/dead-ends.md`).
    Headless,
}

impl Core {
    /// The core of a mould with no tier of its own: headless, so taking
    /// its home stops nothing; it still pulls (`Core::pull`).
    pub fn of(r: &Rules) -> Core {
        Core { kind: CoreKind::Headless, secs: r.core_secs, pull: r.core_pull }
    }
}

/// Every number in the game layer: M3's (`larder::Rules`) where the rule
/// carried over, so a census can be read against M3's.
#[derive(Clone, Copy, Debug)]
pub struct Rules {
    /// Ticks a game second (M3: 15, an eighth of the dish's 120).
    pub tps: u32,
    /// Ticks a squad frame: the squad reads and acts this often.
    pub frame_ticks: u32,
    /// Dish cells a game second.
    pub speed: f32,
    /// How far a soldier sees and fires, dish cells.
    pub range: f32,
    /// Radius of a shot's strike, dish cells (at most `squad::R_MAX` fine).
    pub strike: f32,
    /// Game seconds between one soldier's shots.
    pub shot_every: f32,
    /// How fast a shot flies, dish cells a game second; 0 is the instant
    /// ray every shot was until 2026-11-13. Why (owner, 11-13 02:37Z):
    /// "Would it look better if the round was a moving bullet instead of a
    /// ray? I suspect it would", and yes to building it. A bullet is game
    /// state (`Game::bullets`), not a drawing: each frame it flies one leg,
    /// which the engines trace exactly as they traced a whole ray (stop at
    /// the first slime, strike there), so neither engine changed and the
    /// mould can grow into a bullet's path or leave it before it lands.
    pub bullet_speed: f32,
    /// What a split bullet makes (`Bullet::split`, the rounds' `split`
    /// card): `split_n` bullets fanned `split_fan` radians either side of
    /// the parent's heading, each `split_range` dish cells long with
    /// `split_r` of its strike radius. Pierce (`Bullet::pierce`) carries a
    /// bullet `pierce_gap` strike radii past its hit, so the next leg does
    /// not land in the same hole.
    pub split_n: u8,
    pub split_fan: f32,
    pub split_range: f32,
    pub split_r: f32,
    pub pierce_gap: f32,
    /// Food a shot costs. 0 since 2026-11-12: the owner's playtest (04:13Z)
    /// ran the bank dry with soldiers left, who could then neither fight
    /// nor harvest past the mould ("energy required for shooting isn't
    /// fun"); a soldier's fire rate (`shot_every`) is the limit. Was 10
    /// (M3's), against 30 food a second a harvester takes.
    pub price: f32,
    pub hp: f32,
    /// Health a walker touching a soldier takes a game second: its bite,
    /// whatever the mould's walker count (`Rules::mass` does not shrink
    /// it). 2 since 2026-11-12 (the owner, 22:18Z, playing shape C: "I can
    /// walk my group of soliders into the thickest part of the slime and
    /// kill most of the slime and harvest the whole crystal without losing
    /// a single soldier"; 22:22Z: "walking directly into the enemy should
    /// be very deadly"). Was 1 x `mass`, 0.2 a walker at C's 25,000: 40
    /// soldiers walked into C's thickest part at 240 s lost none in 60 s
    /// on race seeds 1-4 (`examples/thickprobe.rs`); at 2 a walker they
    /// lose 26 to 35 of 40 (the report's *Thick slime*). A walker touches
    /// a soldier on about a third of its frames there (most 1-4 walkers,
    /// 20 or more on 1%), so a soldier on the edge lives and one in the
    /// middle dies in seconds. Buildings keep their own
    /// (`BuildRules::drain`). Today's 5,000-walker mould (shape A, mass 1)
    /// bites twice what it did.
    pub drain: f32,
    /// Walkers within this many dish cells touch a soldier.
    pub touch: f32,
    /// Wading (the owner, 2026-11-12 14:42Z: "soldiers should be heavily
    /// slowed down if they try to run into the thick slime"): a soldier
    /// with this many walkers touching it walks at half speed, and slower
    /// as more press on it, speed / (1 + (touching / wade)²). A walker or
    /// two at a tendril barely slow it (2 touching: 90%); the middle of a
    /// body holds it to a fifth (12 touching, as a rush meets on race seeds
    /// 1-4, where touching runs 0 to 26). The read is the frame's touch
    /// count, the same on both engines, so the GPU needs nothing new. 0 is
    /// off. Every walker counts whole, at any `Rules::mass`, since
    /// 2026-11-13 (the owner, 15:55Z, of shape C: "why are they not way
    /// slowing down. I thought we implemented that"): counted at C's mass
    /// of 0.2 a walker, half speed took 30 touching, and a soldier with 26
    /// on it in C's body still walked at 57% (now 5%). The video the owner
    /// remembered working was the 5,000-walker mould, mass 1.
    pub wade: f32,
    /// A soldier's scent, as a share of a full pile's, laid on the agar
    /// within `smell_r` dish cells of it.
    pub smell: f32,
    pub smell_r: f32,
    /// Food a soldier takes from a crystal a game second, how much it can
    /// carry, and the charge one food is.
    pub harvest: f32,
    pub carry: f32,
    pub charge: f32,
    /// Defend fires on mould this near, dish cells.
    pub defend: f32,
    /// Walkers the aimed 4 x 4 block must hold before a soldier fires at
    /// it. 1 (any): on rush seed 2 over 200 s, 1 wasted 1 shot of 200 and
    /// killed 1,600 walkers, 2 killed 1,486 and 4 killed 1,452, once the
    /// sight check stopped shots into rock (before it, 104 of 200 shots
    /// hit nothing at 1 and 2 of 199 at 4). Whole walkers, not by mass:
    /// at 1 it is presence, the same at any mass.
    pub aim_min: u32,
    pub core_r: f32,
    pub core_secs: f32,
    /// The core's pull (`Core::pull` for a mould with no tier of its own):
    /// a soldier within twice `core_r` of the core smells like a lure,
    /// this many times its own scent over `BuildRules::lure_r`, so the
    /// body comes for a squad at its core. 0 is off. 8, a lure's own
    /// strength (`BuildRules::lure_smell`), since 2026-11-12: on seeds
    /// 201-208 pull 4 still let random take the core on 2 and pull 16 cut
    /// strike's takes to 2 of 8, where 8 left random none and strike its
    /// 3. See `Core` for the census.
    pub core_pull: f32,
    /// The cap, game seconds (owner, 04:04Z: 15 minutes).
    pub cap: f32,
    pub start_bank: f32,
    pub squad: usize,
    pub soldier_cost: f32,
    pub max_squad: usize,
    /// Food a dish cell of wall costs, and its width in dish cells.
    pub wall_cost: f32,
    pub wall_width: f32,
    /// The larder smells of the bank, `hoard` times a full pile per 10,000
    /// food (M2), and walkers touching it take `steal` food a game second.
    pub hoard: f32,
    pub steal: f32,
    /// A carrier whose crystal has run dry (the frame gave under half its
    /// ask and the whole crystal holds less than one frame's ask) walks
    /// home with what it has, banks it and holds there (`Soldier::dry`). On:
    /// under score v2 (`Game::score`) farm and wall are even with it off
    /// (2026-11-12). Under W2b's score it cost farm 10 of 10 (WM1), because
    /// that score counted a crystal with crumbs left as held.
    pub home_dry: bool,
    /// Turrets, lures, pumps and walls (`buildings`).
    pub build: BuildRules,
    /// **Larder overrun** (campaign thread, the owner's pick 2026-11-12
    /// 04:44Z): the round is lost (`End::Overrun`) once the mould has held
    /// the larder (a walker within its 3 dish cells) with the bank under 1
    /// food for this many game seconds straight. 0 is off. **20 s by
    /// default since PR 4** (the economy; integration, 11-12 17:10Z: the
    /// owner ruled on 11-11 that an idle player should lose, and idle
    /// survived 46 of 48 games): with `Rules::ration` an idle squad's bank
    /// runs dry, and a mould at the larder then takes the round. Before PR
    /// 4 a campaign mission turned it on.
    pub overrun: f32,
    /// **Reserve recruit** (the same pick): while fewer than 2 soldiers live
    /// and the bank can't buy one, the larder makes a free soldier after
    /// each this many game seconds of it, unless the mould holds the larder.
    /// With it on, a wiped squad is no loss while a recruit can still come.
    /// 0, the default, is off.
    pub reserve: f32,
    /// **Rations** (PR 4, the economy): food a game second each live
    /// soldier eats from the bank (`Game::eaten`). Why: food didn't bind.
    /// Farm, wall and fort ended 15-minute games with 83-85% of their
    /// harvest unspent, and strike won by banking behind a 30-soldier army
    /// it never paid to keep (integration, 11-12 17:10Z, from the one-body
    /// thread's slime-enemy review). An army now costs its keep: farm's 8
    /// soldiers eat 8 a second of the about 55 they bank, and strike's 30
    /// eat 30. Paid each step; what the bank can't pay, the squad goes
    /// hungry for (`Rules::hunger`). 0 is off.
    ///
    /// 1, from smoke runs on race seeds 5-8 (11-12 20:20Z): at 1 idle's
    /// squad starves by 570-600 s on all 4, farm and fort hold to the cap,
    /// and strike wins 2, loses 1 and stalemates 1 (it had stalemated all
    /// 4). At 2 idle starves by 340-350 s, but strike starves its 30
    /// soldiers on 3 of 4 and fort on 1: an army costing 4.5 times its
    /// price over a game was too steep for a first default.
    ///
    /// **0, off, since 2026-11-14** (owner, after a playtest, 01:49Z: "I
    /// don't know when it was decided that I have to keep my solider fed or
    /// they die, but that is bad. The whole game is just trying to harvest
    /// crystals then I am have no time to attack the slime."). Food buys
    /// soldiers, walls and buildings; it no longer keeps soldiers alive.
    /// `ration=1` (window and census) plays the PR 4 economy.
    pub ration: f32,
    /// Hit points a game second every soldier loses while the bank can't
    /// pay the rations (a share of it for a part-paid step; `Game::hungry`).
    /// Hunger costs only hit points: a starving soldier walks, fires and
    /// harvests at full speed, so a farm that runs dry recovers as soon as
    /// a carrier banks, and the need falls as the squad shrinks. Slowing
    /// hungry soldiers would spiral (slower carriers bank less, so starve
    /// more; integration's check, 19:59Z). At 1 a full-health soldier
    /// lasts 100 s of an empty bank.
    pub hunger: f32,
    /// A carrier back from a dry crystal with nothing queued walks on to
    /// the nearest crystal with charge left (`Game::seek`), the mould's
    /// home excepted, rather than holding at the larder (owner, 11-12
    /// 04:22Z: better orders for harvesting). Only to a safe crystal
    /// (`Game::seek`). On, for the window and the census alike (one game):
    /// on seeds 5-20 the gate holds, farm 10 higher and 5 lower than with
    /// it off (p 0.30), wall 4 and 11 (p 0.12). The first version (nearest
    /// to the soldier, any crystal) cost wall 14 of 16 (p 0.004) and was
    /// replaced (`Reports/dead-ends.md`).
    pub seek: bool,
    /// **The hole under fire**: game seconds of crater (`squad::Frame::
    /// crater`, the squad toolkit's scorch field) each hit lays on its
    /// strike's cells, so sustained fire carves a hole the mould can't
    /// re-knit until the fire stops (owner, 11-12 04:39Z). 0 is off. A
    /// walker standing in a crater still stops a shot (`squad::shots`).
    /// 10: on the race world (seeds 1-8), four soldiers firing for 20 s
    /// left a median hole of 60 dish cells against 29 with none (8 of 8
    /// larger), hits landed 5.3 dish cells out instead of 3.8, kills rose
    /// from 163 to 741, and 79% of the hole was left 5 s after the last
    /// shot; 1 and 3 s gave +2 and +5, 30 s less (kills halve once the
    /// near rim is empty). A strike (the toolkit's `scorch_keep` 0.1,
    /// `scorch_full` 30) bites at full strength until the last second.
    /// Gate (race 5-36, the Fork and the Rim 5-20): farm, wall, fort and
    /// strike beat idle 32-0 and random 24-8 or better, all under Holm;
    /// strike no longer beats farm (20-12, p 0.22; 28-4 without craters).
    /// Before the walker stop, shots flew over walkers in the holes and
    /// the rush hand's squad was wiped on 22 of 32 seeds (9 with it, 7
    /// without craters). Random wiped the mould on 7 of 32 race seeds
    /// against 2 without craters (5-0 by seed, p 0.06). Why a crater and
    /// not a flee: the mould turning from gunfire made a bigger hole but
    /// let random hold the core (`Reports/dead-ends.md`); the crater
    /// changes only the struck cells.
    ///
    /// **4.5 s since 2026-11-12, the owner's pick** (17:29Z: "Can we do
    /// 4.5 s? I am thinking between the 3 and 6"), after clips of the
    /// cone crater (PR 3b) with the squad firing into the thick ring on
    /// the core crystal (`larder-world hand=volley`): at 10 s the hole
    /// was still pocked 4 s after the fire stopped (15:18Z: "3 fills in
    /// too slow still", said of a flat-crater clip), at 3 s it closed in
    /// about 2 s, at 1 s it looked like none. Set on the clips and the
    /// tests, with no census (the owner's 17:26Z testing rule: a census
    /// is not a gate for a look). Any time works, not just whole seconds:
    /// the countdown is in fine steps (`World::fine_steps` a tick).
    ///
    /// **2 s since 2026-11-14, the owner's pick** (card, 01:05Z: "Shorter
    /// craters, 2 s"), after playing shape C (11-12 22:18Z: "The craters
    /// last too long"). The card's other choices were a crater that keeps
    /// half the trail under it (`scorch_keep` 0.5, so thick mould closes
    /// it soonest) and keeping 4.5 s. Two seconds answers "too long"
    /// directly, and since aims see their slime (#198) a squad by a wall
    /// fires about four times as often, so a hole under steady fire is
    /// renewed before it closes. On shape C (the Fork, seed 1, three
    /// hands) a hit's hole is back to 80% of its body in a median 0.5 to
    /// 0.9 s, against 1.3 to 3.9 s at 4.5 s, and a shot kills 2-6% fewer
    /// (`Reports/larder-crater-2s-2026-11-14.md`). Set from the card
    /// with smoke checks, no census (the 17:26Z testing rule).
    pub crater: f32,
    /// How far into the slime a shot's blast is centred, in strike radii
    /// (`squad::Frame::sink`): on along the shot's line from the first
    /// slime it meets, to the last slime within this many radii. 1 since
    /// 2026-11-12: the owner, playing shape D (22:06Z), found a squad
    /// standing off a large mass "barely seems to do damage, but if I move
    /// them into the mold, they destroy it easily", and said "Yes" (23:14Z)
    /// to centring the blast inside the slime. A blast centred on the
    /// body's edge spent about half its disc on open ground. 0 is the edge
    /// hit as before (`sink=0` on the window and the example). On shape C
    /// a squad standing still on free fire went from 12.6 kills a shot to
    /// 20.3, walking in from 42.0 to 47.9
    /// (`Reports/larder-shot-sink-2026-11-12.md`). Named sink, not bite
    /// (its first name): the mould's bite is its damage to soldiers
    /// (`drain`).
    pub sink: f32,
    /// The controlled squad's edge (`squads`, owner 2026-11-14 04:08Z:
    /// "whoever you are actively controlling is stronger"; 04:18Z: faster
    /// fire and harder hits, seen in the bullets and the holes).
    pub lead: super::LeadRules,
    /// The squad mode (`squads`, owner 2026-11-14 04:49Z: "a separate
    /// mode for right now. Not erase the other game"): formations by
    /// squad, the controlled squad's self-defence stance and bought
    /// soldiers joining it. Off, the game plays as before (squad commands
    /// still apply, but nothing sends them).
    pub squads: bool,
    /// The larder (on). Off (the squad mode, owner 2026-11-14 04:18Z: "get
    /// rid of the whole larder system"), a crystal is *drained* or
    /// *guarded* (owner's card 04:27Z): a soldier harvesting it banks the
    /// charge as it takes it, nothing carried, until it runs dry for both
    /// sides; and a crystal with soldiers standing by it and no walker on
    /// it is guarded, paying `guard` into the bank without being used up
    /// (owner 04:28Z: "guarding has to be more valuable in the long run").
    /// No larder to rob or smell; bought soldiers start by the squad.
    pub larder: bool,
    /// Game seconds a guarded crystal takes to pay what draining it whole
    /// would, and it goes on paying: past that, guarding is worth more.
    pub guard: f32,
    /// Walkers on a crystal (whole) that stop its guard paying: shape C
    /// lays a thin film nearly everywhere, so with 0 no crystal paid on
    /// the Fork.
    pub guard_walkers: u32,
    /// Aim spread, dish cells at the target: each shot's line turns by a
    /// uniform angle that moves its aim up to this far to either side of
    /// the target. 0 (the default) aims dead on and draws no random
    /// number, so games stay as they were. A look switch for the owner's
    /// A/B (game designer, 11-12): a fixed-radius strike repeated on one
    /// target reads as a round stamp; a smaller strike scattered over a
    /// wider patch at the same kills a second may read as gunfire.
    pub spread: f32,
    /// Soldiers walk round a crystal with charge left, not over it (owner,
    /// 11-12 14:43Z: "shouldn't be able to stand on top of the crystal"):
    /// they harvest from its rim, bank at the rim of a larder that sits on
    /// one, and a goal inside one moves to its rim (`Game::standable`,
    /// `Game::rim`). Shots still fly over crystals.
    pub crystal_solid: bool,
    /// How much one walker counts in a fight: a walker touching a building
    /// or the larder does `mass` times `BuildRules::drain` (or `steal`).
    /// Not a soldier's: a walker's bite (`drain`) is whole at any count
    /// since 2026-11-12, when shape C's 0.2 a walker let 40 soldiers stand
    /// in its thickest part and lose none, and its hold on a soldier's
    /// legs (`wade`) since 2026-11-13.
    /// 1 (the default) is today's 5,000-walker mould. A denser mould is
    /// the same body cut into more, smaller walkers, so it plays at
    /// 5,000 / its walkers at scale 2 (`Rules::mass_for`); without that the 25,000-walker
    /// sheet, mesh and trunk shapes wiped an idle squad on the Fork in
    /// 0:47 to 1:56 (`Reports/slime-questions-2026-11-12/`), which is the
    /// walker count, not the shape. Shots are unchanged: a crater kills
    /// the walkers in its cells, so a denser body already loses more to
    /// each one. Agreed with the Larder thread (11-12, reference density
    /// over density); first set by the owner's shape playtest (20:21Z).
    pub mass: f32,
    /// The broken mould (owner, 2026-11-12 22:06-22:07Z, playing shape D:
    /// "after killing almost all the mold, having the kill the few random
    /// stragglers spread across the map to win is annoying"; "There are
    /// still clearly lone walkers moving around"): once the mould falls
    /// under `broken` times its peak walkers, its upkeep is `broken_starve`
    /// times (`World::set_upkeep`), so its stragglers off food starve in a
    /// second or two, while any on a crystal still eat and can breed it
    /// back over the line, which gives the upkeep back. 0 is off. Why not a
    /// lone walker's own upkeep (`BigParams::lone_upkeep`): at 25,000
    /// walkers only about 1% are alone at any time, 3 and 10 times did not
    /// lower that, and 10 cost the spread (15.5 crystals at 180 s against
    /// 19.5, 4 of 4 seeds; `examples/loneprobe.rs`).
    pub broken: f32,
    pub broken_starve: f32,
}

impl Rules {
    /// The fight weight of one walker in a mould that starts with
    /// `walkers` on a world of `scale` (`Rules::mass`): today's density,
    /// 5,000 at scale 2 (1,250 at scale 1, 20,000 at 4), counts 1 each.
    pub fn mass_for(walkers: usize, scale: usize) -> f32 {
        (1_250 * scale * scale) as f32 / walkers.max(1) as f32
    }
}

impl Default for Rules {
    fn default() -> Self {
        Rules {
            tps: 15,
            frame_ticks: 1,
            speed: 10.0,
            range: 15.0,
            strike: 2.5,
            shot_every: 1.0,
            bullet_speed: 45.0,
            split_n: 3,
            split_fan: 0.5,
            split_range: 6.0,
            split_r: 0.7,
            pierce_gap: 2.0,
            price: 0.0,
            hp: 100.0,
            drain: 2.0,
            touch: 1.5,
            wade: 6.0,
            smell: 0.5,
            smell_r: 2.0,
            harvest: 30.0,
            carry: 300.0,
            charge: 64.0,
            defend: 7.5,
            aim_min: 1,
            core_r: 5.0,
            core_secs: 5.0,
            core_pull: 8.0,
            cap: 900.0,
            start_bank: 2000.0,
            squad: 4,
            soldier_cost: 400.0,
            max_squad: 40,
            wall_cost: 20.0,
            wall_width: 2.0,
            hoard: 1.0,
            steal: 1.0,
            home_dry: true,
            build: BuildRules::default(),
            overrun: 20.0,
            reserve: 0.0,
            ration: 0.0,
            hunger: 1.0,
            seek: true,
            crater: 2.0,
            sink: 1.0,
            lead: super::LeadRules::default(),
            squads: false,
            larder: true,
            guard: 120.0,
            guard_walkers: 8,
            spread: 0.0,
            crystal_solid: true,
            mass: 1.0,
            broken: 0.03,
            broken_starve: 50.0,
        }
    }
}
