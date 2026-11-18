# Enemy lane

State first, kept current after every merged batch; the brief the lane was
given on 2026-11-17 is below it, unchanged.

## State (2026-11-18, after the planet 2 look)

**Open branch:** `claude/project-thread-s5k9cw`, with the planet 2 look
(docs only). Merged on 11-18: #451 (the switch settlement, job 1), #457
(the lure's field), #473 (the Trunk's fort), #478 (the lure's field round
on open ground), #480 (job 2's page and job 3's design).

**The order of work now** (11-18): the origin is designed and waits on
Direction ([design](../Reports/origin-design-2026-11-18/README.md): a
sitting lump on the home crystal, regrowing from its charge, calling the
body home when struck, its death the win; three questions for the owner,
which Direction holds until after his next play). It is Milestone 2's, so
it is not built until Direction says go. Direction counts this lane's
Milestone 1 jobs done but planet 1's minute-2 gap for a rusher (D7, the
owner's call).

**Planet 2, the bot or the planet: mostly the planet** (Direction's
question, 11-18; [report](../Reports/planet2-sprawler-2026-11-18/README.md)).
On run seeds 1, 3 and 4 the squad lands by crystals of 20,000 to 120,000
charge while every crystal over 130,000 sits in a slime of 23,000 to
25,000 walkers; the bot has 3 to 9 soldiers at 2:30. A bot that backs off
farther, or the Sprawler's smell off, gains a little on 2 of 3 seeds; the
slime at 0.6 of its size gives 18 to 24 soldiers on all three. Seed 2
lands by a free 365,000 crystal and builds 30. The size is Direction's
(the tuning numbers); the lever is `Level::walkers` (`run.rs`'s
`WALKERS_PER_DEPTH`, the area and `size_roll`), which has no per-tier
factor today. The two local patches the films used are in the report.

**Job 2, what planets 1 to 3 do by minute 2: page delivered**
([report](../Reports/planet-things-2026-11-18/README.md), one clip a
planet). Planet 2 (hunts the squad) and planet 3 (walls its crystals,
hardens where shot) do their thing to the run bot by 2:00. Planet 1 eats
pumps by 1:15 but leaves a squad that builds none alone (the bot: 50
soldiers at health 93 at 2:00). That gap is D7's; Direction holds the
candidate (below).

**The lure's field is round** (#478; Look and interface's note, 11-18):
the lure has its own walking search, `raids::round_walk` (chamfer 5, 7,
11 with the knight's moves, within 3% of the line, stopping at the
reach). The charges, far smell and raids keep `big::units_field_for`'s
octagon (`big.rs` is frozen). Look's `set_steer_reach` reads the same
units.

**The Trunk's fort: built** (the owner in this lane's thread, 11-18 03:11Z,
on the walls that form where the Trunk is shot: "This is interesting and I
want to play test it, but when it was original suggested, I thought the
walls would form protecting the core of the slime or the slime crystals
giving them a base of sorts";
[report](../Reports/trunk-fort-2026-11-18/README.md)). On planet 3 the slime
walls each crystal it holds: an amber ring with four gates, round its home
crystal by 0:30 and round each crystal it takes after. Hardening where it
is shot stays on, so he can play both.

- The rule is `Rules::fort` (the ring's thickness in dish cells; 0 off, 3 in
  `run::TRUNK_RULES`). `Game::time_fort` (`bulwark.rs`) looks every 2 game
  seconds at the walkers on each crystal: held at a quarter covered, lost
  under a twentieth or dry. On a change it writes the rings
  (`bulwark::fort_mask`) through `World::set_fort` into the fourth quarter
  of the cells' region in `news` (`fort_at` in engine.wgsl), and Params
  word 888 is the flag that a mask is written (`Engine::forted`). A walker
  on a masked block with crust under it sets with no scar and stays set;
  the bulwark rule makes it a wall. GPU only.
- All four quarters of the cells' region are now taken (charge paths, far
  smell, lure, fort). Params words 880, 884 and 892 are free.
- The slime's own ways (charges, far smell, lure, raids) go through the
  gates: `Game::slime_ground` in `raids.rs` is the raid ground less the
  wall cells.
- The first gate of the home ring faces the middle of the map and every
  other ring's faces home; `open_gate` turns a ring so that at least two
  gates open on ground.
- A fire burns the stretch of wall between two gates and stops at them (the
  fed slime in a gate does not burn). The burnt stretch had not grown back
  70 s later; whether a wall chewed by shots refills was not checked.
- Not checked: a squad fighting through a ring, planets other than 3 (the
  rule is the Trunk's alone), the GTX 1070's time. Look and interface
  reviews the clips: the set slime's flat slab is now a standing feature.
- It belongs in the origin's design page (job 3): a walled home crystal is
  the base to break into.

**The lure's field: merged** (#457; the owner's card, 11-18 03:03Z: "Lure
now, in the three planets";
[report](../Reports/lure-field-2026-11-18/README.md)). Calm slime within
`Rules::lure_reach` (60 cells of walking) of a Lure that is on, or of a
`LureMark` in `Game::lure_marks`, leans down the walking distance to it.
The weight is the engine key `lure_pull`, 0 by default and
`run::LURE_PULL` 10 on every planet (Direction's to tune). GPU only.

- The field is the third quarter of the cells' region in `news`
  (`lure_dist` in engine.wgsl). The pull is Params word 844, sent only
  while a field is written (`Engine::lured`).
- Charging, surging, raiding and breakaway walkers and mini-boss members
  ignore it (this lane's call); Blind and Sour apply to it.
- Squad and arsenal brought the Lure back to the run's build card (300
  food, 150 health; free from planet 1 with no unlock since #464) and wrote
  its news line. Look and interface filmed it pulling on planets 2 and 3
  (412 and 503 walkers within 30 cells at 0:40, 11-18). Films use
  `lure=X,Y[,T]` (the building); `lure_mark=T,X,Y,SECS` lays a mark.
- The rings on the ground are Look and interface's (`View::set_steer`,
  #458). This lane's call is in the window's `prepare_now` (`main.rs`):
  each frame, the field's word (`dense_at / 4 + DENSE_HEAD + n + 2 * nb`)
  while `Engine::lured`, else 0.
- Not checked: a lure behind a wall with a gap, the GTX 1070's time.

**Job 1, the switches: done.** What became of each of the fourteen is in
`Reports/enemy-switches-2026-11-17/README.md` (the roadmap that held the
table is history since 11-18). In short:

- On on every run planet: the far smell (`far_smell`, in `run::enemy_keys`)
  and charges round walls (`unit_paths`, in `run::ENEMY_RULES`).
  `play-new-slime.cmd` is deleted.
- The Trunk's trait (planet 3; `TRUNK_KEYS`, `TRUNK_RULES` in `run.rs`):
  crust, its pull at +8 with fast ground, setting, the wall (`set_block` 5)
  and `fire_fed`, and since 11-18 the fort. Its card: "It walls the
  crystals it holds in amber, and sets where you shoot it".
  Planet 3 was picked over a deeper tier because it is inside Milestone 1
  and its old card was the Drifter's.
- The Swarmer's trait (planet 4; `SWARMER_RULES`): the last stand.
- Deleted, with dead-end lines: the trap (`set_hold`), and the GPU game's
  half of lossy flow and the lone step (which freed Params words 844, 880,
  884 and 888; the lure took 844). `src/mould/big.rs` still has `flow_loss` and `lone_step`: it is
  frozen until Milestone 2 (the owner's 19:21Z answer).
- Not deleted, against the brief: the reserve recruit. The run never turns
  `Rules::reserve` on; it is the frozen campaign's mission rule (a free
  soldier 20 s after the last one dies, the owner's 11-12 no-soft-lock
  pick), so deleting it would change the campaign. The switch list gave it
  the row "Free recruits", which is Squad and arsenal's. Direction was told.
- Still off: crystals that refill (`crystal_regrow`), which is D7's. It is
  the only off switch in this lane's files.
- The switch list in code (`src/build/switches.rs`, Engine and tools') lost
  the ten rows this settled and has four left: `crystal_regrow` (this
  lane's), `set_look` (Look and interface), `feel` and `reserve` (Squad and
  arsenal). That is one over the plan's three, so its `SETTLING` is still
  true; the lane that settles the next row sets it false.
- Handed over: `setlook` and the haze to Look and interface (amber is what
  the game draws). The flinch stays at 0.9 (`shape_keys`), and the 11-15
  blind page for it is closed: it is this lane's number, changed if the
  owner says the slime backs off too much or too little.
- The crust "name the ground" quiz is dropped (Direction's sort, 11-17).

**Numbers, all first numbers from one film each on the software GPU**
(run seed 1, `mode=squads hand=greedy`, at 2:00; before, after):
planet 1, 42 soldiers and 42 (health 97, 90); planet 2, 18 and 7; planet 3,
1 and 0, with 9,831 of the Trunk's 18,984 walkers set and the squad's kills
by 1:00 down from 7,992 to 3,205. Nothing here was timed on the GTX 1070;
the frame costs quoted in the pull request are the simulation-use
reports' llvmpipe ones, and the 1070 run goes in Engine and tools'
Milestone 1 batch.

**Not done, and known:**

- The last stand has no film on planet 4 (a scripted hand does not cut a
  Swarmer to a quarter in a short film). It is covered by
  `all_off_switches_on_together_play_on_both_engines` and the raid tests.
- The bulwark's map wait and the nest-door check from
  `Reports/sim-use-2026-11-17/e2b-bulwark.md` were never measured.
- `tiers::set_key` (the CPU setter) does not know the Trunk's composite
  keys (`crust`, `setting`, `fire_fed`); the run sets them on the GPU
  engine only, and `every_ladder_tier_does_something_to_you` lets the CPU
  refuse them for the Trunk alone.

**Job 2's gap**: planet 1 does nothing to a squad that drains by hand and
builds no pump (the Drifter only smells pumps). No rush brake (D7):
Direction (11-17 22:28Z) holds "the Drifter also smells crystals drained
by hand" as the first candidate for when the owner rules on D7, and it is
not to be built before, not even behind a switch.

**Planet 1's two hands, for the run bot's paired seeds** (Direction's ask;
the gap between them is the fact D7 needs). Run seed 1 on the software
GPU, this branch, one film each:

- The builder is raided:
  `larder-world look=DIR run=1:1 mode=squads hand=pumps at=30,60,90,120,180`.
  The pump at (290,50) is at 187 of 200 hp at 0:30 and 73 at 1:00; raid 1
  goes at 0:59 for the pump at (374,25); both pumps are worn down by 1:30.
  The squad of 4 is at health 67 at 1:30 and 12 at 2:00 (3 soldiers), and 1
  soldier is left at 3:00.
- The rush is not touched:
  `larder-world look=DIR run=1:1 mode=squads hand=greedy at=20,60,120`.
  42 soldiers at health 90 at 2:00, 3,283 killed, 38% of the crystals
  left; two raids of about 1,670 walkers went at the squad (0:59, 1:35)
  and the slime was down to 9,603 walkers in its bowl.

The same builder hand on the other two planets (`run=1:2`, `run=1:3` with
`crustlook=1`, `at=20,40,60,90,120`): on planet 2 the squad of 4 is dead
by 1:30 (raid 1 at 0:59, pumps at 183 and 140 hp at 1:00 and down by
1:30); on planet 3 raid 1 goes at 1:12, the pumps are down by 2:00, 578
walkers are set at 1:30 and 3,019 (2,932 wall cells) at 2:00, and the
squad of 4 is at health 40.

**Filming notes** beyond `Reports/whole-game-review-2026-11-15/kit.md`:
`light=T,X,Y,R` takes dish cells, which are half the world units the log's
`squad at` prints on a run planet (scale 2); `centre=`, `hold@` and `boom=`
take world units. `crustlook=1` adds the set-walker and wall-cell counts to
the log.

---

# Lane brief: Enemy

Model and effort: Fable, extra high. Read `00-all-lanes.md` in this folder
first.

## Your job

The slime is the game's enemy and its best idea. You own its rules
(`src/mould/big.rs`, `src/gpu/engine.wgsl`), `src/mould/tiers.rs`,
`larder_world/{bosses,raids,shapes,bulwark}.rs`, and the ladder in
`src/mould/run.rs` (`LADDER`, `does_to_you`, `enemy_keys`). For Milestone
1 your target is exit test 4: after playing planets 1 to 3, Scott can say
in a sentence what each slime did to him.

Start from `handoff/slime-enemy.md`, `handoff/simulation-use.md`,
`handoff/mini-boss.md` and `handoff/run-progression.md` as history.

## First jobs, in order

1. **Settle the 14 built-but-off switches** (ROADMAP, "Switches that are
   off") in your first two days. Scott cannot test what he cannot see.
   - Far smell (`far_smell`) and charges round walls (`unit_paths`): on by
     default on the run's planets. Delete `play-new-slime.cmd`.
   - Setting, block or trap (`set_block`, `set_hold`), crust and its pull,
     fed slime fireproof (`fire_fed`): pick one configuration yourself and
     give it to one deeper tier as that tier's trait, as fire shyness
     became the Sprawler's. The 11-17 films exist if you need them.
   - Reserve recruit, lossy flow, lone step: delete, each with its
     `Reports/dead-ends.md` line.
   - Last stand: one deep tier's trait, or delete.
   - Crystals that refill: stays off; it belongs to Scott's D7 answer.
   - The set slime's look (`setlook`) and the haze strength go to Look and
     interface. You keep the flinch strength.
   After this at most three switches are off in the whole game, each
   declared in the switch list Engine and tools is building.
2. **Planets 1 to 3 each do one nameable thing to the player by minute
   2.** Today's film of run seed 1 (one seed, scripted hand): on planet 1
   a drain-and-buy squad reaches 42 soldiers at 2:00 with nothing touching
   it and the slime stays in its bowl; on planet 3 the same hand is down
   to 1 soldier at 2:00. Make the three steps of that curve deliberate.
   Deliver one still or clip per planet showing its thing. Do not add a
   rush brake (prices, soldier caps): that is D7 and Scott is holding it.
3. **Then the origin** (Scott's 11-16 idea, Milestone 2's win rule): the
   body at the slime's start crystal as the thing you kill to win a
   planet. Write the design in a page for Direction before building.

## Rules particular to this lane

- New slime rules are written on the GPU engine and tested on the
  software GPU. Write no new CPU twin until Scott answers the 19:10Z card.
  Prefer levers that write fields both engines already have (scent,
  alarm, `squad::Burst`).
- Quote the frame cost of anything that runs every step. The GTX 1070
  runs go in one batch before the milestone check; ask Engine and tools.
- New mini-boss kinds are parked until Scott has said how the four built
  ones play.
