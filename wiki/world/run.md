---
title: The roguelike run
kind: design
status: open
updated: 2026-11-15
sources:
  - Reports/gun-order-2026-11-14.md
  - Reports/roguelike-run-2026-11-14/README.md
code:
  - LADDER
  - PLANETS
  - REF_OPEN
  - Run::buy_into
  - Run::carried
  - Run::gun
  - Run::level
  - Run::put_in
  - Run::rules
  - Run::shop
  - ammo_price
  - area_of
  - clock
  - gun_price
  - head_start
  - reward
  - run_args
  - shells_at
  - shop_of
  - shop_plan
  - shrink_goal
  - two_stars
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
slots hold the cards. The gun fires its slots left to right; the player
drags a card onto a slot to move it, buys a card for sale straight into a
slot, drags one onto the bag to take it out, or presses Q and E; a card
put in without a slot lands before the first bullet (`Run::put_in`)
([report](../../Reports/gun-order-2026-11-14.md)). Bullets are rolled (`Ammo::roll`, two a shop, priced by `ammo_price`); the fixed payload cards are no longer sold. Walkers scale with depth and the planet's area, then its open cells at level start (`open_scale`). The clock grows with the planet's area (`clock`: 15:00 on planet 1 to 55:30 on the core world; two stars before four fifths of it, `two_stars`), and on planets bigger than today's map the slime grows alone first (`head_start`: 11 to 87 s on planets 4-8), the clock starting on landing. The slime sheds most of its walkers in its first half minute on every planet, so a big planet's slime does not cover it ([report](../../Reports/roguelike-run-2026-11-14/README.md), *Run pacing*). Locked buildings and tools show
locked on the command card. Dev mode adds rows to jump planets, add points
and redraw cards. Each planet is the level generator's `levelgen::level(seed, depth)` for the run's seed ([planets](planets.md)). Not yet played by the
owner; nothing is balanced ([report](../../Reports/roguelike-run-2026-11-14/README.md)).

## Owner's word

- 2026-11-14 15:21Z: "For now separate, but it might become the main game. I don't want to erase the campaign yet"; one level one map with a shop between: "Yes. Unless you have other suggestions. The maps should get bigger too as we progress."
- 2026-11-14 15:25Z: "the slime is taking over planets and each level is a new planet as we move towards the core slime world. This is just head cannon right now".
- 2026-11-14 15:26Z: level generation is its own thread; this one reviews it "but you don't do level generation".
- 2026-11-14 16:55Z: "Guns and bullets should have icons. This whole menu should not just be uniform buttons. There should be one area for unlocking/upgrading that should match the player menu from the screen (buildings and tools should be separate menus that you open up). The second area of the screen is for Guns/bullet cards."
- 2026-11-14 20:32Z: "How do I actually build and adjust my guns. When I buy a card it goes into the gun but I don't see how to move cards around".
- 2026-11-14 15:33Z: "There should be a dev mode to control what level planet i go to"; rare planets with base remains or soldiers are "more of a rescue the human planet".

## Open

- A mortar or artillery building; rescue planets; saving a run.
- Prices, rewards and the goal by depth wait for the owner's playtest.
- A breather planet around planet 6 (the planet review, 11-15): the owner judges the pace in play first (02:17Z, via the levelgen thread).
- The slime's size on big planets: it settles at about 7,300 walkers on the core world from 65,600, so the area scaling does not reach play; the slime's economy, not the run's.

## Sources

- [report](../../Reports/roguelike-run-2026-11-14/README.md), *What was built*.
- [arranging the gun](../../Reports/gun-order-2026-11-14.md), *What was built*.
