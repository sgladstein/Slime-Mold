# Enemy lane

State first, kept current after every merged batch; the brief the lane was
given on 2026-11-17 is below it, unchanged.

## State (2026-11-17, after the switch settlement)

**Open branch:** `claude/project-thread-s5k9cw`, one pull request (the
switch settlement, job 1). Nothing else is open.

**Job 1, the switches: done.** What became of each of the fourteen is in
ROADMAP, *Switches that are off*. In short:

- On on every run planet: the far smell (`far_smell`, in `run::enemy_keys`)
  and charges round walls (`unit_paths`, in `run::ENEMY_RULES`).
  `play-new-slime.cmd` is deleted.
- The Trunk's trait (planet 3; `TRUNK_KEYS`, `TRUNK_RULES` in `run.rs`):
  crust, its pull at +8 with fast ground, setting, the wall (`set_block` 5)
  and `fire_fed`. Its card: "Where you shoot it, it sets into amber walls".
  Planet 3 was picked over a deeper tier because it is inside Milestone 1
  and its old card was the Drifter's.
- The Swarmer's trait (planet 4; `SWARMER_RULES`): the last stand.
- Deleted, with dead-end lines: the trap (`set_hold`), and the GPU game's
  half of lossy flow and the lone step (Params words 844, 880, 884, 888 are
  free). `src/mould/big.rs` still has `flow_loss` and `lone_step`: it is
  frozen until Milestone 2 (the owner's 19:21Z answer).
- Not deleted, against the brief: the reserve recruit. `Rules::reserve` has
  no `set=` name and the run never turns it on; it is the frozen campaign's
  mission rule (a free soldier 20 s after the last one dies, the owner's
  11-12 no-soft-lock pick). It is not a switch, so it stays with the
  campaign. Direction was told.
- Still off: crystals that refill (`crystal_regrow`), which is D7's. It is
  the only off switch in this lane's files.
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

**Next step: job 2**, planets 1 to 3 each do one nameable thing by minute
2, one still or clip a planet. Where it stands: planet 3 now has its thing
(the walls). Planet 1 does nothing to a squad that drains by hand and
builds no pump (the Drifter only smells pumps), and planet 2's hunt is the
Sprawler's own. Start with planet 1. No rush brake (D7). Then job 3, the
origin, as a design page to Direction before any code.

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
