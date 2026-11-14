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
  - Run::carried
  - Run::gun
  - Run::level
  - Run::rules
  - Run::shop
  - gun_price
  - reward
  - run_args
  - shop_of
  - shop_plan
  - shrink_goal
---

# The roguelike run

## Now

A separate mode beside the campaign (Run on the start menu, or
`run=SEED`): eight planets on the way to the slime's home world, each one
map with one slime to cut to `shrink_goal` of its peak (25% on planet 1 down
to 10%), the tier climbing `LADDER` and walkers a fifth more each planet. A
won planet pays points for a shop between planets that upgrades the squad,
unlocks and upgrades buildings and tools, and sells guns and gun cards
that get better deeper; a lost planet ends the run. The shop page has two
areas: upgrades as a grid of command-card tiles with Buildings, Tools and
Dev as folders, and an armory of gun cards (icon, stat bars) and bullet
cards (the HUD's icons, in the carried gun's slots, the bag, for sale). The
run starts with the average rifle and carries one gun (`Run::carried`,
the gun thread's `guns::Gun`); two guns rolled for the coming planet are on
sale each shop, priced by `gun_price`, and the carried gun's budget and
slots hold the cards. Walkers scale with depth and the planet's area. Locked buildings and tools show
locked on the command card. Dev mode adds rows to jump planets, add points
and redraw cards. Each planet is the level generator's `levelgen::level(seed, depth)` for the run's seed ([planets](planets.md)). Not yet played by the
owner; nothing is balanced ([report](../../Reports/roguelike-run-2026-11-14/README.md)).

## Owner's word

- 2026-11-14 15:21Z: "For now separate, but it might become the main game. I don't want to erase the campaign yet"; one level one map with a shop between: "Yes. Unless you have other suggestions. The maps should get bigger too as we progress."
- 2026-11-14 15:25Z: "the slime is taking over planets and each level is a new planet as we move towards the core slime world. This is just head cannon right now".
- 2026-11-14 15:26Z: level generation is its own thread; this one reviews it "but you don't do level generation".
- 2026-11-14 16:55Z: "Guns and bullets should have icons. This whole menu should not just be uniform buttons. There should be one area for unlocking/upgrading that should match the player menu from the screen (buildings and tools should be separate menus that you open up). The second area of the screen is for Guns/bullet cards."
- 2026-11-14 15:33Z: "There should be a dev mode to control what level planet i go to"; rare planets with base remains or soldiers are "more of a rescue the human planet".

## Open

- A mortar or artillery building; rescue planets; saving a run.
- Prices, rewards and the goal by depth wait for the owner's playtest.

## Sources

- [report](../../Reports/roguelike-run-2026-11-14/README.md), *What was built*.
