---
title: The roguelike run
kind: design
status: open
updated: 2026-11-15
sources:
  - Reports/gun-order-2026-11-14.md
  - Reports/gun-range-2026-11-15.md
  - Reports/pump-economy-2026-11-15/README.md
  - Reports/roguelike-run-2026-11-14/README.md
  - Reports/surge-2026-11-15/README.md
code:
  - LADDER
  - PLANETS
  - RANGE_DEPTH
  - REF_OPEN
  - Run::buy_into
  - Run::carried
  - Run::gun
  - Run::level
  - Run::put_in
  - Run::range
  - Run::rules
  - Run::shop
  - WIN_SHARE
  - ammo_price
  - area_of
  - clock
  - enemy_keys
  - gun_price
  - head_start
  - range_args
  - reward
  - run_args
  - run_shape
  - run_walkers
  - shells_at
  - shop_of
  - shop_plan
  - shrink_goal
  - size_roll
  - surge_food
  - surge_secs
  - two_stars
---

# The roguelike run

## Now

A separate mode beside the campaign (Run on the start menu, or
`run=SEED`): eight planets on the way to the slime's home world, each one
map with one slime to cut to `shrink_goal` of its peak (`WIN_SHARE`, 2% on
every planet since 11-15; it was 25% on planet 1 down to 10%, and the owner
kept winning with slime left; under 3% the slime is broken and its
stragglers off food starve, `Rules::broken`), the tier climbing `LADDER` and walkers a fifth more each planet. A
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
([report](../../Reports/gun-order-2026-11-14.md)). Bullets are rolled (`Ammo::roll`, two a shop, priced by `ammo_price`); the fixed payload cards are no longer sold. Walkers scale with depth and the planet's area, then its open cells at level start (`open_scale`). The clock grows with the planet's area (`clock`: 15:00 on planet 1 to 55:30 on the core world; two stars before four fifths of it, `two_stars`), and on planets bigger than today's map the slime grows alone first (`head_start`: 11 to 87 s on planets 4-8), the clock starting on landing. The slime is shape C under the tier's keys (`run_shape`), from C's 25,000 walkers times the level's factor (`run_walkers`) and a per-planet roll of 0.8 to 1.25 (`size_roll`; owner 11-15 08:34Z: "It should vary and be more later on"), C's slots grown to match, and then the planet's own engine keys (`enemy_keys`, 11-15, the slime-enemy thread; the run thread agreed the curve): the slime lunges 1 s to 1.8 s (`surge_dash`) and thickens from 2 down to 1.3 births' energy (`thicken`), so deeper slime presses harder and toughens faster on what it holds. Its surges come when it has eaten enough from its crystals (`Rules::surge_food`, set from `surge_food`: a third more than the sandbox's 150 food on planets 1-2, as much on 3-4, down to half on the core world, times the level's walkers, since a bigger slime eats more), at least 10 s apart and at most 80 s on planets 1-2, 60 s on 3-4, 50 s on 5, 40 s on 6-7 and 30 s on the core world (`surge_every`, the longest wait); the shop's clock line names the longest wait (`surge_secs`). Before food-driven surges (later on 11-15) they came on a fixed beat of half those waits ([surge](../../Reports/surge-2026-11-15/README.md)); until 11-15 a run played the bare race slime by mistake, which shed nine tenths of itself in a minute ([report](../../Reports/roguelike-run-2026-11-14/README.md), *The run's slime is shape C*). Locked buildings and tools show
locked on the command card, except pumps, the main income since 2026-11-15: they are never locked, and the shop sells Pump power (`pump+`, a quarter more draw a level) in place of their unlock ([report](../../Reports/pump-economy-2026-11-15/README.md)). Dev mode adds rows to jump planets, add points
and redraw cards. Each planet is the level generator's `levelgen::level(seed, depth)` for the run's seed ([planets](planets.md)). Not yet played by the
owner; nothing is balanced ([report](../../Reports/roguelike-run-2026-11-14/README.md)).

The gun range plays with its shop as a free armory (below).

## The gun range

Gun range on the start menu, or `range=SEED`, uses a run
as a free armory (`Run::range`): every gun kind owned at its average with a
rolled one of each on the rack to swap in, a bullet of every payload and set
(the standard set exact) and every other card, never sold out, and tiles to
roll them again, deeper or shallower (from `RANGE_DEPTH`), and to empty the
gun or the bag. Back to the range plays the sandbox's round armed from it
(`range_args`), with no time limit and a 100,000 bank; H there opens the
armory over the paused round and Back re-arms every squad
([report](../../Reports/gun-range-2026-11-15.md)).

## Owner's word

- 2026-11-14 15:21Z: "For now separate, but it might become the main game. I don't want to erase the campaign yet"; one level one map with a shop between: "Yes. Unless you have other suggestions. The maps should get bigger too as we progress."
- 2026-11-14 15:25Z: "the slime is taking over planets and each level is a new planet as we move towards the core slime world. This is just head cannon right now".
- 2026-11-14 15:26Z: level generation is its own thread; this one reviews it "but you don't do level generation".
- 2026-11-14 16:55Z: "Guns and bullets should have icons. This whole menu should not just be uniform buttons. There should be one area for unlocking/upgrading that should match the player menu from the screen (buildings and tools should be separate menus that you open up). The second area of the screen is for Guns/bullet cards."
- 2026-11-14 20:32Z: "How do I actually build and adjust my guns. When I buy a card it goes into the gun but I don't see how to move cards around".
- 2026-11-15 02:58Z: "everything shifts in the shop. Once you add a bag that shouldn't happen" (fixed: the page reserves two card groups).
- 2026-11-15 08:34Z (to the slime thread, on a planet's slime size): "It should vary and be more later on."
- 2026-11-15 03:02Z: "I know I originally said getting the last few slime stragglers is annoying but right now how it's set up I feel like I keep winning before I've actually killed everything. So maybe we need a different threshold or something" (`WIN_SHARE` 2%).
- 2026-11-14 15:33Z: "There should be a dev mode to control what level planet i go to"; rare planets with base remains or soldiers are "more of a rescue the human planet".
- 2026-11-15 02:55Z: "I need a test environment built where I can play with the guns and cards and bullets."

## Open

- A mortar or artillery building; rescue planets; saving a run.
- Prices, rewards and the goal by depth wait for the owner's playtest.
- A breather planet around planet 6 (the planet review, 11-15): the owner judges the pace in play first (02:17Z, via the levelgen thread).
- The core world with C: 364,924 walkers, frame p95 12-14 ms on the 1070 (planet 1 4.6 ms) and a 12 s head-start pause before landing; kept until play says otherwise.

## Sources

- [report](../../Reports/roguelike-run-2026-11-14/README.md), *What was built*.
- [arranging the gun](../../Reports/gun-order-2026-11-14.md), *What was built*.
- [the gun range](../../Reports/gun-range-2026-11-15.md), *What was built*.
