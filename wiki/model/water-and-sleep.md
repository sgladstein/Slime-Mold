---
title: Water and sleep
kind: mechanism
status: paused
updated: 2026-11-10
sources:
  - Reports/mould-keep-alive-2026-11-10.md
  - Reports/mould-looks-2026-11-10.md
  - Reports/mould-review-2-2026-11-10.md
  - Reports/mould-review-thread-handoff-2026-11-10.md
code:
  - Dish::wet
  - Params::crust_life
  - Params::water_r
  - Params::wet
  - Tool::Water
  - WATER_REGROW_SECONDS
  - WATER_STOCK
---

# Water and sleep

## Now

Every cell has a wetness (`Dish::wet`, resting at 0.5). The lamp dries what
it shines on. On ground drier than 0.25 a walker falls asleep: it does not
move, eat, pay upkeep or get pushed by the lamp, and dies at random with a
mean life of 20 s (`Params::crust_life`). Ground wetter than 0.7 wakes it,
and for 5 s after waking it counts as fed. The **Water** tool drops a wet
disc of radius 12 (`Params::water_r`); you hold 2 drops and one comes back
every 40 s (`WATER_STOCK`, `WATER_REGROW_SECONDS`).

It is on only where a dish gives water (`Params::wet`): today only
[Keep It Alive](../dishes/keep-it-alive.md). There it is the mechanism that
makes the tools work on each other: bake mould asleep through a famine with
the [lamp](../tools/lamp.md), wake it with water by the next pile. A wet
dish nobody lights behaves step for step like the dry one.


## Owner's word

- 2026-11-10 02:58Z: "We also need to add complexity to the game. The tools
  are quite one dimensionally right now" (review thread).
- 2026-11-10 03:06Z: "Agreed" to **not more tools, but tools that work on
  each other through damp ground**: a water tool, the lamp dries, dry mould
  sleeps, water wakes it.

## Tried and rejected

- 3 drops, regrow 20 s, sleepers living 40 s: random play coasted to the
  ten-minute cap. Both levers were cut
  ([keep-alive, *Tuning that was tried*](../../Reports/mould-keep-alive-2026-11-10.md)).

## Open

- Sleepers lasting longer than 20 s, so seed beds become a plan, is one of
  review 2's ideas for Keep It Alive's luck. Not tried.
- None about the drawing: the looks pass draws sleeping mould as a matte
  crust, wet ground darker blue and dry ground paler, and gives water its
  tile and a preview that counts the sleepers a drop would wake
  ([looks, *Water and sleeping mould drawn*](../../Reports/mould-looks-2026-11-10.md)).
  The keep-alive report's "stand-in icon" line predates it.

## Sources

- [mould-keep-alive-2026-11-10.md](../../Reports/mould-keep-alive-2026-11-10.md), *Water and sleep*, *Numbers*, *Tuning*.
- [mould-review-2-2026-11-10.md](../../Reports/mould-review-2-2026-11-10.md), *Keep It Alive*.
- [mould-review-thread-handoff-2026-11-10.md](../../Reports/mould-review-thread-handoff-2026-11-10.md): owner's words.
