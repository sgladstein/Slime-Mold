---
title: The roguelike run
kind: design
status: open
updated: 2026-11-15
sources:
  - Reports/flinch-2026-11-15/README.md
  - Reports/gun-order-2026-11-14.md
  - Reports/gun-range-2026-11-15.md
  - Reports/menu-fixes-2026-11-15/README.md
  - Reports/pump-economy-2026-11-15/README.md
  - Reports/roguelike-run-2026-11-14/README.md
  - Reports/run-progression-2026-11-15/README.md
  - Reports/run-progression-2026-11-15/does-to-you.md
  - Reports/shop-b-2026-11-15/README.md
  - Reports/surge-2026-11-15/README.md
  - Reports/whole-game-review-2026-11-15/README.md
code:
  - LADDER
  - PLANETS
  - Playlog
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
  - STUCK_WARN
  - WIN_SHARE
  - ammo_price
  - area_of
  - clock
  - does_to_you
  - enemy_keys
  - gun_price
  - head_start
  - range_args
  - reward
  - run_args
  - run_hidden
  - run_shape
  - run_walkers
  - shells_at
  - shop_of
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
map with one slime to cut to `shrink_goal` of its peak (`WIN_SHARE`, 1% on
every planet since 11-15 18:02Z; 2% from 03:02Z, and before that 25% on
planet 1 down to 10%; each time the owner kept winning with slime left; under 3% the slime is broken and its
stragglers off food starve, `Rules::broken`), the tier climbing `LADDER` and walkers a fifth more each planet. The goal on the HUD counts it out: "counting from 1:00" and the walkers now before the peak starts counting, then the slime now against the count to reach, both as mass (a thickened walker twice, as the bar counts it; "12,630 walkers now → 126" at 1%), [report](../../Reports/menu-fixes-2026-11-15/README.md)). A
won planet pays points for a shop between planets that upgrades the squad,
unlocks and upgrades buildings and tools, and sells guns and gun cards
that get better deeper; a lost planet ends the run. The shop page is one
screen at every UI size (shop B, the owner's pick of 2026-11-15 19:37Z):
the planet's line and the points along the top, the carried gun's slots
and the bag pinned under it, a row of tabs (Cards, Guns, Squad, Buildings,
Tools) over a counter of half-size tiles that pages rather than scrolls,
and a pane at the right that tells the picked tile, else the planet's
briefing, with its Buy button; a click picks, Enter or the button buys
([report](../../Reports/shop-b-2026-11-15/README.md)). Until then it had
upgrades as a grid of command-card tiles with Buildings, Tools and Dev as
folders, and an armory of gun cards and bullet cards below, which scrolled
at large UI sizes (cards in the HUD's icons, in the carried gun's slots,
the bag, for sale). The
run starts with the average rifle and carries one gun (`Run::carried`,
the gun thread's `guns::Gun`); two guns rolled for the coming planet are on
sale each shop, priced by `gun_price`, and the carried gun's budget and
slots hold the cards. The gun fires its slots left to right; the player
drags a card onto a slot to move it, buys a card for sale straight into a
slot, drags one onto the bag to take it out, or presses Q and E; a card
put in without a slot lands before the first bullet (`Run::put_in`)
([report](../../Reports/gun-order-2026-11-14.md)). Bullets are rolled (`Ammo::roll`, two a shop, priced by `ammo_price`); the fixed payload cards are no longer sold. Walkers scale with depth and the planet's area, then its open cells at level start (`open_scale`). The clock grows with the planet's area (`clock`: 15:00 on planet 1 to 55:30 on the core world; two stars before four fifths of it, `two_stars`), and on planets bigger than today's map the slime grows alone first (`head_start`: 11 to 87 s on planets 4-8), the clock starting on landing. The slime is shape C under the tier's keys (`run_shape`), from C's 25,000 walkers times the level's factor (`run_walkers`) and a per-planet roll of 0.8 to 1.25 (`size_roll`; owner 11-15 08:34Z: "It should vary and be more later on"), C's slots grown to match, and then the planet's own engine keys (`enemy_keys`, 11-15, the slime-enemy thread; the run thread agreed the curve): the slime lunges 1 s to 1.8 s (`surge_dash`) and thickens from 2 down to 1.3 births' energy (`thicken`), so deeper slime presses harder and toughens faster on what it holds. On planets 1-3, whose tiers read no alarm of their own, C's flinch is on (`BigParams::flinch` 0.9): the slime backs off ground a volley keeps hitting and closes in when it stops, at no frame cost on the GTX 1070; planets 4-8 keep their tier's alarm ([flinch](../../Reports/flinch-2026-11-15/README.md)). Its surges come when it has eaten enough from its crystals (`Rules::surge_food`, set from `surge_food`: a third more than the sandbox's 150 food on planets 1-2, as much on 3-4, down to half on the core world, times the level's walkers, since a bigger slime eats more), at least 10 s apart and at most 80 s on planets 1-2, 60 s on 3-4, 50 s on 5, 40 s on 6-7 and 30 s on the core world (`surge_every`, the longest wait); the shop's clock line names the longest wait (`surge_secs`). Before food-driven surges (later on 11-15) they came on a fixed beat of half those waits; neither costs a frame on the GTX 1070 ([surge](../../Reports/surge-2026-11-15/README.md)); until 11-15 a run played the bare race slime by mistake, which shed nine tenths of itself in a minute ([report](../../Reports/roguelike-run-2026-11-14/README.md), *The run's slime is shape C*). Locked buildings and tools show
locked on the command card, except pumps, the main income since 2026-11-15: they are never locked, never drain their crystal (owner, 2026-11-15 17:14Z), and the shop sells Pump power (`pump+`, a quarter more draw a level) in place of their unlock ([report](../../Reports/pump-economy-2026-11-15/README.md)). Dev mode adds rows to jump planets, add points
and redraw cards. Each planet is the level generator's `levelgen::level(seed, depth)` for the run's seed ([planets](planets.md)). Not yet played by the
owner; nothing is balanced ([report](../../Reports/roguelike-run-2026-11-14/README.md)).

Ending early, the goal's mass, the play log and the planets' pressure: below.
The gun range plays with its shop as a free armory (below).

## Ending early, the play log, pressure by depth

Since 11-15 (the whole-game review's W17, W30 and W13;
[report](../../Reports/run-progression-2026-11-15/README.md)), a second
counts as stuck only when, besides no shot, no food taken and the bank under
a soldier's price, no soldier walked, no pump stands, nothing is being built
and shots cost food or every crystal is dry (in squad mode shots are free,
so a planet with charge left never ends stuck); from `STUCK_WARN` (10 s) the goal line warns
that the planet ends at `STUCK_END` (30 s); before, a run ended at 1:37 with
four healthy soldiers and both pumps standing. The shrink goal counts the
slime's mass (`World::mass`: a thickened walker twice; the GPU's `count`
pass adds the enemy's condensed walkers to the readout), so a body that
thickens no longer reads as shrinking. The window writes a play log
(`Playlog`) beside the replay, `larder-world-log-MAP-SEED.txt`, a line a
game second (walkers, mass, soldiers, squads, bank, income, shots, kills,
buildings, frame p50/p95) with each command; `cargo run --release --example
playlog -- FILE` prints time to 10-40 soldiers, splits, pumps, kills a
minute against the slime's mass, and frame times.

Run seed 1, planets 1-6 ([report](../../Reports/run-progression-2026-11-15/README.md), *Measured*): idle is clearly losing by 5:00 on planets 1-3, and a four-soldier squad with the depth's average rifle kills 7-40 walkers a second while the slime's own swing is ten times that, so its kills don't show in the mass at 2:00; mass at landing grows 11x from planet 1 to 6 and the kill rate 2.5x. What rises by planet: see below.

## What each planet does to you

Since 11-15 (the owner's D3, 21:41Z: "Agreed. We can test it out";
[report](../../Reports/run-progression-2026-11-15/does-to-you.md)), each
rung of `LADDER` does one thing to you (`does_to_you`), named on the planet
card and in the landing alert: the Drifter and the Trunk smell your pumps
(`pump_smell` 20 over `lure_r` 15 cells) and come to eat their crystals; the
Sprawler smells your soldiers from far off (`smell` 4 over 4 cells); the
Swarmer's surges reach 25 cells past your range (`panic_margin`) and lunge
3x (`surge_haste`); the Sporer bursts into spores at an alarm of 0.3; the
Sleeper wakes at an alarm of 0.3. Run seed 1 at 2:00, each ends or wears the
four-soldier test squad (or the pumps) sooner than before: three planets that
squad survived now end it. A test: the owner's play sets the size. The run
shop no longer sells the outpost, lure or siren (D10; `run_hidden` hides
them on the command card), and the 40-soldier rush is left as it is (D7).

## The gun range

Gun range on the start menu, or `range=SEED`, uses a run
as a free armory (`Run::range`): every gun kind owned at its average with a
rolled one of each on the rack to swap in, a bullet of every payload and set
(the standard set exact) and every other card, never sold out, and tiles to
roll them again, deeper or shallower (from `RANGE_DEPTH`), and to empty the
gun or the bag. The armory is the shop's frame with the range's tabs: Guns
as a table whose stats are green or red against the carried gun, Bullets as
a grid of shape by payload, and the modifiers' two shelves; the roll and
empty tools sit on the bottom bar ([report](../../Reports/shop-b-2026-11-15/README.md)). Back to the range plays the sandbox's round armed from it
(`range_args`), with no time limit and a 100,000 bank; H there opens the
armory over the paused round and Back re-arms every squad
([report](../../Reports/gun-range-2026-11-15.md)).

## Owner's word

- 2026-11-14 15:21Z: "For now separate, but it might become the main game. I don't want to erase the campaign yet"; one level one map with a shop between: "Yes. Unless you have other suggestions. The maps should get bigger too as we progress."
- 2026-11-14 15:25Z: "the slime is taking over planets and each level is a new planet as we move towards the core slime world. This is just head cannon right now".
- 2026-11-14 15:26Z: level generation is its own thread; this one reviews it "but you don't do level generation".
- 2026-11-14 16:55Z: "Guns and bullets should have icons. This whole menu should not just be uniform buttons. There should be one area for unlocking/upgrading that should match the player menu from the screen (buildings and tools should be separate menus that you open up). The second area of the screen is for Guns/bullet cards."
- 2026-11-14 20:32Z: "How do I actually build and adjust my guns. When I buy a card it goes into the gun but I don't see how to move cards around".
- 2026-11-15 02:58Z: "everything shifts in the shop. Once you add a bag that shouldn't happen" (fixed: the page reserves two card groups; in shop B the gun and bag are pinned and hold still, a test checks it).
- 2026-11-15 17:31Z: "Having to scroll is horrible. That's not professional." 17:41Z: "The bullet tiles could easily be half the size without even losing any info." 19:37Z: "Shop B, menu A," (the tabbed counter with a pane).
- 2026-11-15 08:34Z (to the slime thread, on a planet's slime size): "It should vary and be more later on."
- 2026-11-15 21:41Z (the review's decisions): "1. Agreed. We can test it out. 2. No. 3. Let me see the video. 4. Play first. No changes yet. 5-7) agreed" (D3 does-to-you as a test, D5 no payload card, D7 no rush brake yet, D10 no outpost, lure or siren in the run shop).
- 2026-11-15 18:02Z: "We are still winning the game with too many slime left. Drop it to 1% of peak" (`WIN_SHARE` 1%).
- 2026-11-15 03:02Z: "I know I originally said getting the last few slime stragglers is annoying but right now how it's set up I feel like I keep winning before I've actually killed everything. So maybe we need a different threshold or something" (`WIN_SHARE` 2%).
- 2026-11-14 15:33Z: "There should be a dev mode to control what level planet i go to"; rare planets with base remains or soldiers are "more of a rescue the human planet".
- 2026-11-15 02:55Z: "I need a test environment built where I can play with the guns and cards and bullets."
- 2026-11-15 08:30Z (play notes for the whole-game review): "I mostly run to the biggest crystal, harvest and create new soliders until I have 40 soliders and then I attack the slime. That is not what I want to play though"; "Some enemy slimes just hang at their home crystal ... They don't attach my pumping crystals so no need to defend"; "There needs to be more interesting progression in the enemy"; the real game is squad mode in the run; "The gun graphics need work. They should really be one of the draws of the game."

## Open

- The whole-game review (11-15, [report](../../Reports/whole-game-review-2026-11-15/README.md)): the slime sitting at home was the bare race slime, fixed by shape C (#336); re-filmed on main, planet 3's slime wears two of four pumps to 0 by 3:00. Still open: the drain-and-buy rush reaches 40 soldiers by 2:00 on planet 1 against the live slime; what the enemy does to you planet by planet is now a test (above), awaiting play.
- A mortar or artillery building; rescue planets; saving a run.
- Prices, rewards and the goal by depth wait for the owner's playtest.
- A breather planet around planet 6 (the planet review, 11-15): the owner judges the pace in play first (02:17Z, via the levelgen thread).
- The core world with C: 364,924 walkers, frame p95 12-14 ms on the 1070 (planet 1 4.6 ms) and a 12 s head-start pause before landing; kept until play says otherwise.

## Sources

- [report](../../Reports/roguelike-run-2026-11-14/README.md), *What was built*.
- [arranging the gun](../../Reports/gun-order-2026-11-14.md), *What was built*.
- [the gun range](../../Reports/gun-range-2026-11-15.md), *What was built*.
- [the whole-game review](../../Reports/whole-game-review-2026-11-15/README.md), *Re-check on main after the review*.
