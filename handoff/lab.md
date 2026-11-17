# Lane brief: Lab

Model and effort: Fable, high. Read `00-all-lanes.md` in this folder first.

## Your job

One prototype at a time, outside the milestone, in your own example
program or branch. Nothing of yours goes into the main game or behind a
switch there. You show Scott a playable clip and a recommendation: build
it into the game, change it, or drop it. He chose "Slice first, plus one
lab thread" on 11-17; you are that thread.

## First prototype: steering the slime

The idea: the slime follows fields, and today the player's tools write
almost only damage into them. Let the player herd it instead: lures that
visibly draw it, walls and ground that funnel it, bait and false trails,
so a good player moves the slime into a kill zone. No other game's enemy
allows this, so it could be the game's signature verb.

Scott's words that point here, 11-16 05:11Z: "There should be ways to
modify the pushback. I could image a gun that lets you corral and push the
slime onto areas. Into a kill zone". He did not answer the question of
whether steering should be your first prototype, so it goes ahead on the
plan's recommendation; if he names another, switch.

How to work:
- Start from the model, not from the old demo tools (his ruling, 11-10):
  what can walkers and fields respond to? Scent, food, alarm, trail,
  ground speed, walls. Describe the simulation neutrally before listing
  tools.
- Grep `Reports/dead-ends.md` for lure, bait, siren and scent before
  building: the siren, the lure and the outpost were taken out of the run
  shop on 11-15, and the reasons matter.
- Build on the GPU engine and test on the software GPU.
- Deliver within about three days: a clip of 30 to 60 seconds on an
  Artifact page of a squad herding a slime into a kill zone on one of
  planets 1 to 3, the counts beside it, and your recommendation.

## Later briefs, for after this one

A second slime species that fights the first; the player's own mould
(parked, D12); the forest floor.

## State (kept current by the lab thread)

- 2026-11-17 22:30Z: **the steering prototype is delivered.** Report
  [`Reports/lab-steering-2026-11-17/`](../Reports/lab-steering-2026-11-17/README.md),
  page "Steering the Slime" (in `Reports/pages.md`). Recommendation: build
  it into the game, changed (its own field slot and a per-tier weight, the
  Lure back in the run shop, pull 10 not 30). Whether and when is
  Direction's to place; nothing of it is in the main game.
- Open branch: `claude/project-thread-4tc75h` holds the prototype's code
  (commits "LAB: steering field prototype" and "LAB: the lure's core
  glow"; module `src/mould/larder_world/lab.rs`). It changes
  `engine.wgsl` and `view.wgsl` in place and borrows the far smell's slot
  (`far_smell=-W` is the pull), so it must not merge. The rig and every
  scene's places are in the report.
- The lab session can push only that branch, so the lab's docs (this file,
  reports, pages) reach main through Direction: the files go in the
  project's shared folder under `lab-docs/` and Direction merges them
  docs-only.
- Next step: wait for the ruling on the recommendation. If steering is
  taken up, the lab's follow-ups are in the report's *Not tried* (a gun
  that lays the pull along its shots is the closest to the owner's words).
  Otherwise start the next brief below.
