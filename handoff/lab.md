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

- 2026-11-17 19:20Z: thread started on the steering prototype. Branch
  `claude/project-thread-4tc75h`. The prototype's code stays on that branch
  and is not merged; only this file, the report and the page come to main.
- Next step: describe what walkers and fields respond to, try each as a
  way to move a body, film the best into a kill zone on planet 1 to 3.
