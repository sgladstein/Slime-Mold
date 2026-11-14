---
title: The roguelike run
kind: design
status: open
updated: 2026-11-14
sources:
  - Reports/roguelike-run-2026-11-14/README.md
code:
  - LADDER
  - PLANETS
  - Run::gun
  - Run::level
  - Run::rules
  - Run::shop
  - reward
  - run_args
  - shop_of
  - shrink_goal
---

# The roguelike run

## Now

A separate mode beside the campaign (Run on the start menu, or
`run=SEED`): eight planets on the way to the slime's home world, each one
map with one slime to cut to `shrink_goal` of its peak (25% on planet 1 down
to 10%), the tier climbing `LADDER` and walkers a fifth more each planet. A
won planet pays points for a shop between planets that upgrades the squad,
unlocks and upgrades buildings and tools, and sells gun cards that get
better deeper; a lost planet ends the run. Locked buildings and tools show
locked on the command card. Dev mode adds rows to jump planets, add points
and redraw cards. Planets play the hand-made maps until the level
generator's `levelgen::level(seed, depth)` lands. Not yet played by the
owner; nothing is balanced ([report](../../Reports/roguelike-run-2026-11-14/README.md)).

## Owner's word

- 2026-11-14 15:21Z: "For now separate, but it might become the main game. I don't want to erase the campaign yet"; one level one map with a shop between: "Yes. Unless you have other suggestions. The maps should get bigger too as we progress."
- 2026-11-14 15:25Z: "the slime is taking over planets and each level is a new planet as we move towards the core slime world. This is just head cannon right now".
- 2026-11-14 15:26Z: level generation is its own thread; this one reviews it "but you don't do level generation".
- 2026-11-14 15:33Z: "There should be a dev mode to control what level planet i go to"; rare planets with base remains or soldiers are "more of a rescue the human planet".

## Open

- The gun thread's card budget (PR #275) in place of slots and power.
- A mortar or artillery building; rescue planets; saving a run.
- Prices, rewards and the goal by depth wait for the owner's playtest.

## Sources

- [report](../../Reports/roguelike-run-2026-11-14/README.md), *What was built*.
