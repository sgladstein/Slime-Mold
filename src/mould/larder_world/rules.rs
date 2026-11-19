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
    /// What each `Range` gun card adds, as a fraction of `range`
    /// (`Game::gun_range`). 1 since 2026-11-14 15:40Z (the owner's notes:
    /// "2-4x distance or rate of fire"): one card 48 cells, two 72, three
    /// 96; 0.5 before (36, 48).
    pub range_card: f32,
    /// What each `Rate` gun card adds to how often a soldier fires: 1 is
    /// twice as often with one card, three times with two.
    pub rate_card: f32,
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
    /// A seek card's catch: dish cells a card either side of the line
    /// where the bullet reaches its aim point and on its last leg
    /// (`Game::catches`; the owner picked homing in the shot trace,
    /// 2026-11-16 19:38Z). It hit 89.1% of shots on thin slime against
    /// 80.3% with no card (13 of 13 paired windows). The seek turn that
    /// steered bullets onto the slime ahead (`seek_turn` and its
    /// `seek_after`, `seek_r`, `seek_cone`, `seek_min`, `seek_gap`) was off
    /// from 11-16 for no gain (hits 89.4%, kills 543 against 554,
    /// `Reports/gun-seek-2026-11-16`) and deleted 11-17 (the technical
    /// lead's off-switch review; `Reports/dead-ends.md`).
    pub seek_catch: f32,
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
    /// Health a soldier standing in fire loses a game second at full heat
    /// (x the heat on its cell, `UnitOut::heat`): the owner (11-14 19:18Z)
    /// "It should hurt the player too". 25: four seconds in the hottest
    /// fire kills, so a soldier walking out of a front lives.
    pub fire_hurt: f32,
    /// Walkers within this many dish cells touch a soldier.
    pub touch: f32,
    /// Elbow room: the least distance between two living soldiers'
    /// centres, in `touch` radii (0 is off). Closer pairs step apart each
    /// frame, at most a wading walk step, only onto standable ground
    /// (`Game::elbow_room`). Before it (main at 3d31dd3) a squad walked
    /// from the larder in one column at one speed and stood stacked: in
    /// the `volley` clip at 0:23, 7 of 9 soldiers sat within one drawn
    /// figure of each other and the window showed 2 (art review
    /// 2026-11-14, rule 6: every soldier countable at zoom 6). At 1.3 the
    /// window's figures (1.15 touch radii, shoulders 0.8 of that) still
    /// overlapped in a five-soldier knot at zoom 6; 1.6 shows each one.
    /// It goes with wider sunflower slots (`Game::slot`, 2.52 dish cells a
    /// ring at 1.6, 1.5 with elbow room off), so a squad at rest is not
    /// pushed off its slots. It spreads a squad's drain over more ground, so it is a
    /// census baseline break, not a balance call.
    pub elbow: f32,
    /// Walk round a wall's corner, not across it (`downhill`, 2026-11-14,
    /// on): the step across was refused in `Game::walk`, so a soldier at a
    /// wall's end stood still for good (the owner's "stuck on the wall").
    /// Off is the old path, kept for the engine twins in mouldgpu: a walk
    /// rule plays the same on both engines, but it moves where the squad
    /// goes, and seed 3's chaos then parted the crater twin's moulds past
    /// its bound (951 GPU against 1049 CPU at 30 s).
    pub corners: bool,
    /// Cells of walking map built a step (`paths::MAP_POPS`, 2026-11-14:
    /// a fixed slice a step, so a 6x planet's new map no longer stalls one
    /// step). `usize::MAX` is the old pace, kept for the engine twins in
    /// mouldgpu: each goal's own map, built whole the step it is asked for.
    /// A walk rule plays the same on both engines, but waiting a step or
    /// two for a map moves where the squad goes, and seed 3's chaos then
    /// parted the twins' moulds past their bound (crater twin 1585 GPU
    /// against 1316 CPU at 25 s; core twin 1630 against 1467, and with
    /// whole maps a step late 1573 kills against 1279).
    pub map_pops: usize,
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
    /// A Guard order's zone round a plain spot, dish cells (a crystal's or
    /// a building's is its own edge plus `GUARD_RIM`): slime in it is shot
    /// whatever the stance but hold fire (`Game::guard_zone`, owner
    /// 2026-11-15 19:51Z). About the defend reach, so a guard on open
    /// ground covers the ground it stands round.
    pub guard_r: f32,
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
    /// The most soldiers alive at once. 200 since 2026-11-16 (owner, after
    /// the soldier-cap thread's measures: "Flat 200, the realistic max",
    /// "I want to test the max"); 40 before, the M2 design number. The
    /// window's soldiers' turn costs about 0.01 ms a soldier a step on a
    /// planet-1 map and 0.03 ms on a 6x planet (cloud CPU,
    /// `examples/soldiercap.rs`; `Reports/soldier-cap-2026-11-16.md`).
    /// Held under `squad::UNITS_MAX` less the home and every building.
    pub max_squad: usize,
    /// Food a dish cell of wall costs, and its width in dish cells.
    pub wall_cost: f32,
    pub wall_width: f32,
    /// Set slime blocks soldiers (E2b, the owner's 01:13Z "Both"): a fine
    /// cell with this many set walkers or more in its 3 x 3 is no ground to
    /// soldiers' feet until the count falls under half of it
    /// (`Game::bulwark`). 0 off; 5 on the Trunk's planet since 2026-11-17
    /// (`run::does_to_you`), where slime sets. The trapping alternative
    /// (`set_hold`: bulwark cells stayed ground and held a soldier to a
    /// share of its speed) was the other side of this A/B and was deleted
    /// the same day (`Reports/dead-ends.md`).
    pub set_block: f32,
    /// **The fort** (2026-11-18, the Enemy lane; `bulwark.rs`'s
    /// `time_fort`): the slime walls each crystal it holds. The game marks
    /// a ring this many dish cells thick round the crystal, with gates,
    /// and slime on the ring sets with no shot needed (`World::set_fort`),
    /// so `set_block` makes it a wall. 0 off; 3 on the Trunk's planet
    /// (`run::does_to_you`); `set=fort=N`. GPU engine only.
    pub fort: f32,
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
    /// survived 46 of 48 games): a mould at the larder eats the bank dry
    /// and then takes the round. (It was paired with the rations, removed
    /// 2026-11-14, which also emptied an idle squad's bank.) Before PR
    /// 4 a campaign mission turned it on.
    pub overrun: f32,
    /// **Reserve recruit** (the same pick): while fewer than 2 soldiers live
    /// and the bank can't buy one, the larder makes a free soldier after
    /// each this many game seconds of it, unless the mould holds the larder.
    /// With it on, a wiped squad is no loss while a recruit can still come.
    /// 0, the default, is off.
    pub reserve: f32,
    /// **The last soldier's death ends the round**, however much food is
    /// in the bank and whatever the reserve would send (1 on, 0 off, the
    /// default). A run turns it on (`Run::rules`): the owner reached planet
    /// 2, lost every soldier and saw them come back (2026-11-18 15:56Z:
    /// "you should lose if all your soliders die"). Free play, missions and
    /// the censuses keep the old rule, where a bank that can buy a soldier,
    /// or a recruit that can come, keeps the round going.
    pub wipe_ends: f32,
    // Rations and hunger (`ration`, `hunger`; PR 4, the economy: food a
    // second each soldier ate from the bank, and hit points lost when it
    // couldn't pay) were removed on 2026-11-14 after being off by default
    // since 01:49Z (owner: "I don't know when it was decided that I have to
    // keep my solider fed or they die, but that is bad"; 04:53Z: "There
    // should be no maintenance costs in this game"). Food buys soldiers,
    // walls, buildings and heals; nothing costs food to keep.
    // `Reports/dead-ends.md` has the numbers.
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
    /// **The spray** (gun look, 2026-11-15, the owner 08:30Z: "The gun
    /// graphics need work. They should really be one of the draws of the
    /// game. They need to be really satisfying."): every walker a shot
    /// kills flies out of the hit as a droplet, back toward the shooter,
    /// for up to `spray_secs`, the furthest `spray_reach` cells, slowing to
    /// a stop, and dies where it lands (`squad::Frame::spray`). Noita's
    /// splash, the world's own matter in flight: each hit visibly blows
    /// slime out of the body, with nothing drawn on the slime (the owner's
    /// 11-12 ruling against stamps), and the kills are as without it. 0
    /// seconds: none (`spray=0` in the window). (A first version threw the
    /// living mould past the strike and let it live: 22% fewer kills by 30
    /// s, 1 to 29 walkers in the air, too few to see; `Reports/dead-ends.md`.)
    /// 0.3 s and 5 cells until the bold spray (11-15: the owner saw no
    /// difference between all of a hit's kills flying and a quarter); now
    /// 0.45 s and 7, drawn hot in their own layer (`view.wgsl`).
    pub spray_secs: f32,
    pub spray_reach: f32,
    /// The share of a shot's kills that spray (`squad::Frame::spray_share`;
    /// `spray_share=` in the window): 1 throws every kill. Asked of the
    /// owner as a blind A/B (the whole-game review, 11-15, wanted a few
    /// walkers a hit), so a choice, not a measured default.
    pub spray_share: f32,
    /// The controlled squad's edge (`squads`, owner 2026-11-14 04:08Z:
    /// "whoever you are actively controlling is stronger"; 04:18Z: faster
    /// fire and harder hits, seen in the bullets and the holes).
    pub lead: super::LeadRules,
    /// The squad mode (`squads`, owner 2026-11-14 04:49Z: "a separate
    /// mode for right now. Not erase the other game"): formations by
    /// squad, the controlled squad's edge and bought
    /// soldiers joining it. Off, the game plays as before (squad commands
    /// still apply, but nothing sends them).
    pub squads: bool,
    /// Squads funnel through places narrower than their formation (on;
    /// `funnel`, owner 2026-11-15). Off, the formation's places that fall
    /// in rock all go to the anchor, as before.
    pub funnel: bool,
    /// Soldiers in the squad mode walk with a velocity (on; `steer`,
    /// squad movement rework 2026-11-16, owner: "It is still clunky
    /// overall"): they speed up and slow down, walk in step with their
    /// squad, follow the squad's way past rock and keep their elbow room
    /// by steering (`Game::steer_all`). Off is the walk before it: full
    /// speed straight at the place each step, shoved apart after.
    pub steer: bool,
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
    /// With no larder, the share of `harvest` a soldier drains by hand
    /// (owner, 2026-11-15: "I just run to the biggest Crystal harvest the
    /// whole thing and I have a full squad of 40 within the first 30
    /// seconds"; pumps are the main income now, `BuildRules::pump_share`).
    /// 1 was the old rule: a starting squad of four draining 120 food a
    /// second. With the larder, carrying is its own brake and this is
    /// not read.
    pub hand: f32,
    /// Defenders while a squad works (`squads`, owner 2026-11-14 15:06Z):
    /// some of a harvesting or building squad stand guard against nearby
    /// slime while the rest work (`Game::cover_turn`).
    pub cover: super::CoverRules,
    /// Healing (owner 2026-11-14 15:06Z: "Units should be able to heal but
    /// it cost energy, and requires units to be idle for bit, maybe only
    /// near a captured crystal"): `Game::heal`.
    pub heal: super::HealRules,
    /// Aim spread, dish cells at the target: each shot's line turns by a
    /// uniform angle that moves its aim up to this far to either side of
    /// the target. 0 aims dead on and draws no random number. The default
    /// is the rifle's `guns::RIFLE_SPREAD` since 11-15 (owner 08:10Z: a
    /// small base spread, so bullet cards' spread shows on it); until
    /// then 0, so games stayed as they were. A look switch for the owner's
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
    /// **A scared piece's reach follows the guns** (2026-11-14, the game
    /// designer's review of #249). A cut-off walker charges a unit within
    /// `BigParams::panic_r` dish cells; the game keeps that at the gun's
    /// `range` plus this many dish cells, once a game second, so range
    /// cards move it too. Why: the rule was built with the reach 5 cells
    /// past a gun's 15 (20); at the range of 24 (#249) soldiers firing from
    /// 21-24 cells sat outside the trigger and shot a cut piece down with
    /// no risk, so the last stand stopped being a threat. 5 keeps the
    /// first rule's margin (29 at range 24). Negative leaves the world's
    /// own `panic_r` (a test or a probe that sets it).
    pub panic_margin: f32,
    /// **The slime surges when it has eaten** (2026-11-15, the slime-enemy
    /// thread with the game designer, after #343's fixed clock): food (in
    /// the bank's units, `charge` a food) the slime eats from its crystals
    /// to build one surge (`Game::surge_meter`). When the meter holds this
    /// much and `surge_min` game seconds have passed since the last surge
    /// (or the landing), the next surge starts and the meter empties; with
    /// nothing eaten one still comes after the world's `surge_every`, the
    /// longest wait. 0 keeps the world's fixed clock. Works only on a world
    /// whose slime surges (`World::surge_every` above 0).
    /// Why: the owner, 11-15 08:34Z, wants the slime to "keep getting
    /// stronger by holding onto its crystals and getting more of them". A
    /// fixed beat ignored that; this one comes sooner the more crystals the
    /// slime holds, and taking crystals or starving it pushes it back.
    /// On at 150 food (`SURGE_FOOD`) with a floor of 10 s; shape C's
    /// `surge` of 1800 steps (60 s) is the longest wait. On the Fork
    /// (squad mode, seed 1, no soldier near) C eats 2.3 food a second in
    /// its first 10 s, 6.9 at 50-60 s and 11 at 150-180 s as it spreads
    /// over its crystals, so its first surge comes at about 43 s, against
    /// the old beat's 30, and the gaps shrink to about 14 s by 3:00: nine
    /// surges in three minutes against six (Reports/surge-2026-11-15/README.md,
    /// *Food-driven*). 120 brought the gaps to 11 s by 2:00.
    pub surge_food: f32,
    pub surge_min: f32,
    /// **Raids** (2026-11-15, the defenses thread; `raids.rs`): the slime
    /// spends what it eats on waves at the player's richest holding.
    pub raid: super::RaidRules,
    /// **Charges round walls** (the simulation-use plan's S1, 2026-11-17;
    /// `raids.rs`'s `time_unit_paths`): the units' walking distance,
    /// rebuilt once a game second, which surging and panicking walkers go
    /// down instead of steering straight at a unit (`set=unit_paths=1`).
    /// Off until the owner's blind films.
    pub unit_paths: bool,
    /// **A lure's reach** (2026-11-18, the Enemy lane; `raids.rs`'s
    /// `time_lure_paths`): how far, in dish cells of walking, the lure's
    /// field goes from a Lure or a lure mark; calm slime inside it leans
    /// toward the lure (`gpu::Settings::lure_pull`). 60, the lab's rig
    /// (`Reports/lab-steering-2026-11-17`); `set=lure_reach=N`.
    pub lure_reach: f32,
    /// **Mini-bosses** (2026-11-16, the mini-boss thread; `bosses.rs`):
    /// slime at a crowded knot or a big part cut off from food clenches
    /// into a lump that hunts the nearest soldier. Off here (`boss_every`
    /// and `boss_cut` 0); a run sets `BossRules::for_depth`.
    pub boss: super::BossRules,
}

/// `Rules::surge_food`'s default: food the slime eats to build a surge.
pub const SURGE_FOOD: f32 = 150.0;

impl Rules {
    /// **Set one rule by name** (the window's `set=NAME=V`, 2026-11-15, the
    /// whole-game review's W44: no reviewer could film a pump's scent on,
    /// as the window took no rule keys). False if there is no such rule.
    pub fn set(&mut self, name: &str, v: f32) -> bool {
        // Raids (`RaidRules`, 11-15): `raid_every=0` turns them off,
        // `last_stand=1` on.
        if name == "last_stand" {
            self.raid.last_stand = v > 0.0;
            return true;
        }
        // `raid_march=0` (Enemy lane, 2026-11-18): raids stream walker by
        // walker, as before the march.
        if name == "raid_march" {
            self.raid.march = v > 0.0;
            return true;
        }
        // `turret_over=0`: turrets stop at walls as soldiers' guns do.
        if name == "turret_over" {
            self.build.turret_over = v > 0.0;
            return true;
        }
        // Mini-bosses' whole numbers (`BossRules`).
        match name {
            "boss_least" => {
                self.boss.least = v.max(0.0) as u32;
                return true;
            }
            "boss_cut_least" => {
                self.boss.cut_least = v.max(0.0) as u32;
                return true;
            }
            "boss_most" => {
                self.boss.most = (v.max(0.0) as usize).min(crate::mould::big::LUMPS);
                return true;
            }
            _ => {}
        }
        // `max_squad=N` (soldier-cap thread, 11-16): the most soldiers
        // alive, so the 1070 bench can time big armies (`hand=greedy` buys
        // to it). Past `squad::UNITS_MAX` less the home and every building
        // a soldier would drop out of the frame, so it is held under that.
        if name == "max_squad" {
            let room = crate::mould::squad::UNITS_MAX - 1 - self.build.max;
            self.max_squad = (v.max(1.0) as usize).min(room);
            return true;
        }
        // The squad movement rework's walk (`steer`, 2026-11-16) and the
        // narrow places' columns (`funnel`, 11-15): `set=steer=0` films
        // the walk before it.
        if name == "steer" {
            self.steer = v > 0.0;
            return true;
        }
        if name == "funnel" {
            self.funnel = v > 0.0;
            return true;
        }
        if name == "unit_paths" {
            self.unit_paths = v > 0.0;
            return true;
        }
        let f = match name {
            "raid_first" => &mut self.raid.first,
            "raid_first_sd" => &mut self.raid.first_sd,
            "raid_min" => &mut self.raid.min,
            "raid_every" => &mut self.raid.every,
            "raid_every_sd" => &mut self.raid.every_sd,
            "raid_size" => &mut self.raid.size,
            "raid_size_sd" => &mut self.raid.size_sd,
            "raid_size_min" => &mut self.raid.size_min,
            "raid_size_max" => &mut self.raid.size_max,
            "raid_share" => &mut self.raid.share,
            "raid_lure_r" => &mut self.raid.lure_r,
            "raid_late" => &mut self.raid.late,
            "raid_ramp" => &mut self.raid.ramp,
            "raid_early" => &mut self.raid.early,
            "raid_early_at" => &mut self.raid.early_at,
            "raid_early_sd" => &mut self.raid.early_sd,
            "raid_march_pull" => &mut self.raid.march_pull,
            "raid_march_leash" => &mut self.raid.march_leash,
            "raid_march_haste" => &mut self.raid.march_haste,
            "raid_march_quorum" => &mut self.raid.march_quorum,
            "raid_march_time" => &mut self.raid.march_time,
            "raid_march_stall" => &mut self.raid.march_stall,
            "raid_march_size" => &mut self.raid.march_size,
            "lure_reach" => &mut self.lure_reach,
            "boss_every" => &mut self.boss.every,
            "boss_every_sd" => &mut self.boss.every_sd,
            "boss_first" => &mut self.boss.first,
            "boss_first_sd" => &mut self.boss.first_sd,
            "boss_min" => &mut self.boss.min,
            "boss_reach" => &mut self.boss.reach,
            "boss_crowd" => &mut self.boss.crowd,
            "boss_cut" => &mut self.boss.cut,
            "boss_gather" => &mut self.boss.gather,
            "boss_clench" => &mut self.boss.clench,
            "boss_pull" => &mut self.boss.pull,
            "boss_leash" => &mut self.boss.leash,
            "boss_haste" => &mut self.boss.haste,
            "boss_quorum" => &mut self.boss.quorum,
            "boss_life" => &mut self.boss.life,
            "boss_grade_lo" => &mut self.boss.grade_lo,
            "boss_grade_hi" => &mut self.boss.grade_hi,
            "boss_armour_lo" => &mut self.boss.armour_lo,
            "boss_armour_hi" => &mut self.boss.armour_hi,
            "boss_bite_lo" => &mut self.boss.bite_lo,
            "boss_bite_hi" => &mut self.boss.bite_hi,
            "boss_size" => &mut self.boss.size,
            "boss_faster" => &mut self.boss.faster,
            "boss_tight" => &mut self.boss.tight,
            "boss_lunge_every" => &mut self.boss.lunge_every,
            "boss_lunge" => &mut self.boss.lunge,
            "boss_lunge_pull" => &mut self.boss.lunge_pull,
            "boss_lunge_haste" => &mut self.boss.lunge_haste,
            "boss_fade" => &mut self.boss.fade,
            "boss_kind" => &mut self.boss.kind,
            "boss_hunter" => &mut self.boss.kinds[0],
            "boss_wanderer" => &mut self.boss.kinds[1],
            "boss_sapper" => &mut self.boss.kinds[2],
            "boss_leech" => &mut self.boss.kinds[3],
            "boss_far" => &mut self.boss.far,
            "boss_fresh" => &mut self.boss.fresh,
            "boss_root" => &mut self.boss.root,
            "boss_grow" => &mut self.boss.grow,
            "boss_ring" => &mut self.boss.ring,
            "last_stand_at" => &mut self.raid.last_stand_at,
            "seek_catch" => &mut self.seek_catch,
            "pump_smell" => &mut self.build.pump_smell,
            "pump_hp" => &mut self.build.pump_hp,
            "lure_smell" => &mut self.build.lure_smell,
            "lure_r" => &mut self.build.lure_r,
            "smell" => &mut self.smell,
            "smell_r" => &mut self.smell_r,
            "hoard" => &mut self.hoard,
            "panic_margin" => &mut self.panic_margin,
            "surge_food" => &mut self.surge_food,
            "surge_min" => &mut self.surge_min,
            "range" => &mut self.range,
            "drain" => &mut self.drain,
            "cap" => &mut self.cap,
            "start_bank" => &mut self.start_bank,
            "soldier_cost" => &mut self.soldier_cost,
            // Walls (the walls thread, 11-16): thickness films and tries.
            "wall_width" => &mut self.wall_width,
            "wall_cost" => &mut self.wall_cost,
            // Reserve recruit (11-17): its switch (`build::switches`,
            // "Free recruits") needs a key; until then only a campaign
            // mission's own rules could set it.
            "reserve" => &mut self.reserve,
            "wipe_ends" => &mut self.wipe_ends,
            "set_block" => &mut self.set_block,
            "fort" => &mut self.fort,
            _ => return false,
        };
        *f = v;
        true
    }

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
            // 24 since 2026-11-14 08:08Z (owner's playtest: "gun range
            // should be increased"); 15 before. Soldiers' sight and every
            // shot's reach, mouse-aimed volleys included.
            range: 24.0,
            range_card: 1.0,
            rate_card: 1.0,
            strike: 2.5,
            shot_every: 1.0,
            bullet_speed: 45.0,
            split_n: 3,
            split_fan: 0.5,
            split_range: 6.0,
            split_r: 0.7,
            pierce_gap: 2.0,
            seek_catch: 2.0,
            price: 0.0,
            hp: 100.0,
            drain: 2.0,
            fire_hurt: 25.0,
            touch: 1.5,
            elbow: 1.6,
            corners: true,
            map_pops: super::paths::MAP_POPS,
            wade: 6.0,
            smell: 0.5,
            smell_r: 2.0,
            harvest: 30.0,
            carry: 300.0,
            charge: 64.0,
            defend: 7.5,
            guard_r: 8.0,
            aim_min: 1,
            core_r: 5.0,
            core_secs: 5.0,
            core_pull: 8.0,
            cap: 900.0,
            start_bank: 2000.0,
            squad: 4,
            soldier_cost: 400.0,
            max_squad: 200,
            wall_cost: 20.0,
            wall_width: 2.0,
            set_block: 0.0,
            fort: 0.0,
            hoard: 1.0,
            steal: 1.0,
            home_dry: true,
            build: BuildRules::default(),
            overrun: 20.0,
            reserve: 0.0,
            wipe_ends: 0.0,
            seek: true,
            crater: 2.0,
            sink: 1.0,
            spray_secs: 0.45,
            spray_reach: 7.0,
            spray_share: 1.0,
            lead: super::LeadRules::default(),
            squads: false,
            funnel: true,
            steer: true,
            larder: true,
            guard: 120.0,
            guard_walkers: 8,
            hand: 0.2,
            cover: super::CoverRules::default(),
            heal: super::HealRules::default(),
            spread: crate::mould::guns::RIFLE_SPREAD,
            crystal_solid: true,
            mass: 1.0,
            broken: 0.03,
            broken_starve: 50.0,
            panic_margin: 5.0,
            surge_food: SURGE_FOOD,
            surge_min: 10.0,
            raid: super::RaidRules::default(),
            unit_paths: false,
            lure_reach: 60.0,
            boss: super::BossRules::default(),
        }
    }
}
