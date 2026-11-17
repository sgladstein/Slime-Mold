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
