# The roguelike run, first build (2026-11-14)

The owner's notes (2026-11-14 15:06Z) asked for "the whole game progress
could be a rogue like ... procedurally generate levels that change as you get
farther with harder enemies. You upgrade your base soldier, unlock or upgrade
turrets, walls, bombs, mortar/artillary as you progress ... you get money or
points to spend between levels. You also get cards for your gun or new guns
which get better at deeper levels." Answers (15:21Z): "For now separate, but
it might become the main game. I don't want to erase the campaign yet"; one
level is one map with one slime to beat, a shop between levels ("Yes. Unless
you have other suggestions. The maps should get bigger too"). Story (15:25Z):
"the slime is taking over planets and each level is a new planet as we move
towards the core slime world". Level generation is its own thread (15:22Z,
15:26Z: "you don't do level generation"); dev mode is this thread's (15:33Z:
"There should be a dev mode to control what level planet i go to").

## What was built

- `src/mould/run.rs`: `Run` (seed, depth, points, owned upgrades, cards,
  the shop's offer), `Run::level` (a `Level`: ground, tier, walker factor and
  a campaign `Mission` with `Goal::Shrink`), `Run::shop` rows and
  `Run::pick`, `Run::rules` and `Run::kit` (the upgrades applied), `Run::gun`.
- A run is **8 planets** (`PLANETS`); the 8th is the slime's home world,
  whose win wins the run. A lost planet ends the run.
- **The slime by depth** (`LADDER`): Drifter, Sprawler, Trunk, Swarmer,
  Sporer, Sleeper; past the ladder the last three take turns. The Grudge is
  left out: `Tier::settings` refuses it on the GPU engine (no scar keys yet),
  and a test keeps every rung playable. Walkers grow a fifth a planet
  (`WALKERS_PER_DEPTH`).
- **The goal**: cut the slime to `shrink_goal(depth)` of its peak, 25% on
  planet 1, 3 points less a planet, 10% from planet 6 (2% on every planet
  from 11-15, *Winning when the slime is gone* below). The mission tracker's
  stars, stuck and cap rules apply unchanged. Free play's bite (`drain` 2),
  not the missions' 1.
- **Ground**: planet `depth` of the run's seed from the level generator
  (`levelgen::level(seed, depth)`, #277, merged while this was built):
  biomes in the generator's set order, the core world at planet 8
  (`levelgen::CORE_DEPTH` = `PLANETS`, a test holds them equal). A level's
  mission names its map `planet:DEPTH:SEED`, which the window's `map=`
  reads, so the title and alert give the planet's name; the shop shows the
  next planet's name and biome from `levelgen::planet` without building it.
  The first build cycled the five hand-made maps (`Ground::Named`, kept).
- **Points**: a won planet pays `reward(depth, stars)` = 150 + 50 x depth +
  50 x (stars - 1): 200 for planet 1 at one star, one unlock.
- **The shop** (`ITEMS`): squad (armour +25% hp, heavier shots +20% strike,
  faster trigger -15% shot gap, longer barrels +3 range, bigger landing +2
  soldiers, supply drop +500 bank, quick hands +25% harvest), buildings
  (walls, owned at the start; turrets and turret power; lures, pumps,
  outposts, sirens), tools (blast, mine, det cord, sleep gas; cheaper tools
  -20%), gun (bigger gun, more cards on offer). Levels cost the first
  price times the level. Locked buildings and tools show on the command
  card dimmed, "Locked: unlock it in the shop between planets".
- **Gun cards** (the gun thread's budget, #275, merged while this was
  built): three on offer a shop (`OFFER`), gated by `Card::tier`: common
  from planet 1, uncommon from planet 2, rare (the bomb) from planet 4; 80,
  150, 250 points. The gun's card budget starts at 5 (`START_BUDGET`; free
  play's is 10) and the Bigger gun upgrade adds 2 a level up to 11, at most
  4 cards (`rounds::fit`). A bought card goes into the gun when it fits,
  else the bag; a pick moves a card in or out, free. H does not cycle the
  gun in a run.
- **Dev mode** (Options, Dev mode, or `dev=1`): shop rows for the next
  planet one deeper or shallower, 1,000 points, and a fresh draw of cards.
- **In the window**: Run on the start menu's main page (Back to the run
  while one is under way); the end screen's first button is "On to the
  shop", "End the run" or "The home world is free"; the shop page lists the
  rows in columns that fit every UI size; "Land on the next planet" and
  "Give up the run". `run=SEED` starts a run from the command line and
  `run=SEED menu=shop look=DIR` films its shop.

## The shop, second build (2026-11-14 18:00Z)

The owner, 16:55Z: "Guns and bullets should have icons. This whole menu
should not just be uniform buttons. There should be one area for
unlocking/upgrading that should match the player menu from the screen
(buildings and tools should be separate menus that you open up). The second
area of the screen is for Guns/bullet cards." Built on the gun thread's
guns (#283: `guns::Gun`, rolled by kind around averages, deeper rolls
bigger; the owner, 16:46Z, "The squad owns the gun").

- **Upgrades** (left): a grid of tiles six wide, drawn as the command card
  draws its buttons (the icon in the middle, the name, levels as pips, the
  price bottom right in the energy colour or red when short, a tick when an
  unlock is owned), with the command card's own icons. Buildings, Tools and
  (in dev mode) Dev are folder tiles with the build and tools pages' icons;
  a click or Enter opens one, Back or Esc closes it. A dot on a folder
  says something in it is affordable now. Under the grid a box says what
  the tile under the cursor (or the keyboard's) is, does and costs.
- **Armory** (right): gun cards (the kind's icon, gold for a lucky roll;
  the name; carried, armory or the price; bars for range, rate, aim,
  bullets a shot, budget and slots), then the carried gun's slots with its
  cards ("In the gun, budget 7 of 22, 3 of 3 slots", empty slots drawn
  empty), then the cards for sale and in the bag as bullet cards: a band in
  the kind's colour, the HUD's card icon, the name, the budget cost as
  pips, the price. The HUD's own card words fill the detail box.
- **Guns in the run** (`Run::guns`, `carry`, `gun_offers`): the run starts
  with the average rifle (budget 10, 4 slots, free play's gun) and each
  shop sells two guns rolled for the planet ahead (`Gun::roll`). A bought
  gun is carried at once; an owned one is carried again for free. The
  carried gun's budget and slots hold the cards (`rounds::fit_gun`); when a
  smaller gun is carried, the last cards bought go back to the bag. Price:
  `(80 + 40 x depth) x grade^2`, in tens (`gun_price`). Every soldier lands
  in squad 0 armed with it (`Cmd::Arm`); a split's new half gets the
  standard rifle (the gun thread's rule). "Bigger gun" (+2 budget a level,
  from 5) is gone: a bigger gun is bought.
- **Walkers by area**: a planet's walkers are its depth factor times its
  area over today's map, so the level generator's smaller planets 1 and 2
  (#284: 60% and 80%) are not flooded and bigger ones are not empty.
- `menu=shop-mid` films a run three planets in (cards in the gun and the
  bag, a second gun, points left); `menu=shop-build` and `shop-tools` with
  that folder open.

Checks: `world::start::tests::the_shop_fits_and_its_rows_pick` (every tile,
on the first page and in each folder, with 6 to 24 cards, on screen and
apart at 1280x720 to 1920x1080 and every UI size; folders open and close by
click, Enter and Esc); `mould::run::tests` (7: cards fit the carried gun's
budget and slots and leave it when a smaller gun is carried; deeper shops
sell guns with bigger budgets, 40 runs a depth; the first gun is carried).

## Rolled bullets in the shop (2026-11-14 19:40Z)

Built on the gun thread's rolled bullets (#291, `Ammo::roll`: a shell, a
set, damage, speed, range and spread at 0.25-4x, a budget cost that follows
the roll; the owner, 18:05Z, made the payloads bullets).

- Each shop offers `AMMO_OFFER` (2) rolled bullets beside `OFFER` (3)
  modifier, trigger and flight cards (plus `draw`'s). The fixed payload
  cards (blast, gas, bomb) are no longer sold: a bullet is rolled instead.
  Shells: plain, blast and gas from planet 1, the bomb from planet 4
  (`shells_at`); no germ (the GPU world casts none yet).
- Price in points (`ammo_price`): `(40 + 20 x the shell's base cost) x
  grade^1.5 x (1 + 0.1 x (depth - 1))`, in tens. Its budget cost in the gun
  is the roll's own (`Card::cost`).
- A bullet card shows the shell's icon, the set's glyph in the corner (as
  the HUD's loadout strip badges it), its name ("Needle bullet", "Slug
  bomb") and its budget cost as pips; the detail box gives its numbers.
- **Walkers per open cell** (the technical lead, 19:23Z): at level start a
  run's walkers, set from the planet's area, are scaled to its open cells
  (`open_scale`: cells not rock over the area times `REF_OPEN` 0.93, the
  mean open share of planets 1-8 of seeds 1-6, 0.84 to 0.98). Counts from
  a base of 5,000: planet 1 at 0.6 area about 3,000; 2x area planets 6-8
  20,000 to 24,000; a 6x core world would be about 72,000.

Checks: `mould::run::tests::every_shop_rolls_bullets_and_prices_them_by_the_roll`
(2 bullets a shop at depths 1-8 over 20 seeds, no germ, no bomb before 4,
no fixed payload card; a bought bullet goes into the gun and its text reads
back through `rounds::parse`).

## Run pacing: the clock and the head start (2026-11-15 02:30Z)

From the planet review's findings 6 and 5 (three reviewers, 11-15,
`Reports/levelgen-review-2026-11-15/`), passed on by the coordinator.

**The clock grows with the planet** (`run::clock`, finding 6). It was 900 s
on every planet while the walk to the far nest grew from 14 s to 60 s, so
the owner's "15/20 minutes to 1 hour+" a level (11-14 15:33Z) was not wired
up. Now 900 + 450 x (area - 0.6) s in half minutes, from the planet's area
as a share of today's map (`run::area_of`): 15:00, 16:30, 18:00, 21:00,
25:30, 31:30, 40:30 and 55:30 on planets 1-8. Linear in area, not in its
square root (the walk), because the slime and the crystals scale with area.
The level carries it as its mission's `rule cap`, so the HUD's clock, the
end and the AI read it unchanged. Two stars now need the goal before four
fifths of the clock (`campaign::two_stars`), as 12:00 was of 15:00; the
campaign's missions keep 15:00 and 12:00. The shop's heading says the next
planet's clock.

**The slime grows alone before landing** (`run::head_start`,
`larder_world::head_start`, finding 5): 60 x (sqrt(area) - 1) s on planets
bigger than today's map, so 11, 25, 40, 60 and 87 s on planets 4-8 and none
on 1-3. Frames with no unit and no shot, then `Game::new` notes the world's
ticks (`Game::landed`) and `Game::time` counts from there, so the clock,
the goal's 60 s wait and the end screen start at 0:00 on landing.
`grow=SECS` sets it by hand (`grow=0` lands on a fresh slime) and
`run=SEED:DEPTH` films a deep planet's level.

What it does, core world seed 1, idle hand, llvmpipe (pictures on the page):

| | Walkers at landing | At 1:00 |
|---|---|---|
| Fresh (`grow=0`) | 65,654 (three discs) | 7,335 (a tube network between the nests) |
| With the 87 s head start | 7,292 (the network) | |

**[Corrected 11-15 08:30Z: the shedding below is the old bare slime a run played by mistake, not shape C; see *The run's slime is shape C* below.]** **What it does not do: spread the slime over the map.** The slime sheds
most of its walkers in its first 30 s on every planet, whatever it starts
with: planet 1 3,234 -> 1,652 at 30 s, planet 5 18,027 -> 4,053, the core
world 65,654 -> 7,335 at 60 s (fresh, idle hand, seed 1). So the walkers
the run scales by area (`Level::walkers`) are mostly gone in half a
minute, and the core world settles at about 4.4 times planet 1's slime on
10 times its area. The review's 3% cover is this, not the front's speed. A
longer head start only shows the network longer; the size the slime keeps
is the slime's own economy (its food and upkeep), which is the slime
thread's to look at, so it is passed on and not tuned here.

Rejected: a clock by the square root of area (the walk): 15:00 to 47:30,
under the owner's "1 hour+" on the core world.

Cost: the head start runs before the first frame, once a level. On
llvmpipe 87 s at 6x took 101-195 s; on the 1070 the 6x step is 2.11 ms, so
about 1,300 frames is an estimated 3 s pause at landing (not yet timed).

The breather planet the review suggests around planet 6 (smaller and
richer, as Noita's Holy Mountain) is map work: the levelgen thread put it
to the owner, who chose (02:17Z) to judge the pace in play first. The run
follows whatever sizes `levelgen::AREA` gives: a smaller planet gets a
shorter clock, fewer walkers and no head start, and the walker test now
asks for more walkers a cell deeper, not in all.

Checks: `run::tests::bigger_planets_get_a_longer_clock_and_a_grown_slime`
(the eight clocks and head starts; every level's rules carry its clock),
`larder_world::tests::a_head_start_grows_the_slime_and_the_clock_starts_on_landing`
(CPU world: ticks run on, `time` is 0 on landing, a 2 s cap ends at 2 s).

## Winning when the slime is gone (2026-11-15 03:30Z)

Owner, 03:02Z: "I know I originally said getting the last few slime
stragglers is annoying but right now how it's set up I feel like I keep
winning before I've actually killed everything. So maybe we need a
different threshold or something".

The goal was the slime cut to 25% of its peak on planet 1, three points
less a planet, 10% from planet 6, so a planet was won with a tenth to a
quarter of the slime still on the map. Now 2% on every planet
(`run::WIN_SHARE`; the goal reads "Wipe out the slime: cut it to 2% of its
peak"). The stragglers stay easy: below 3% of its peak the slime is
already broken (`Rules::broken`, the owner's 11-12 ruling), its upkeep
50 times, so pieces off food starve in a second or two and the HUD says
"The mould is broken: its stragglers starve unless they reach a crystal".
2% sits under that line, since the game's peak counts from the landing
and the goal's from 60 s, so what is left to shoot when the goal is near
is slime on its crystals. Only the run changes: the campaign's missions
keep their own goals in their files, and free play is won by wiping the
mould or taking its core, as before.

Smoke check, planet 1 seed 1, llvmpipe, three AI hands to 15:00:

| Hand | End | Slime left | Walkers killed |
|---|---|---|---|
| strike | goal met at 6:37, 3 stars | 19 walkers (peak about 1,800) | 8,409 |
| claim | the clock, no goal | 1,533 | 11,756 |
| fort | the clock, no goal (never fires) | 1,822 | 0 |

The strike hand's slime was at 1,783 walkers at 6:00 and 19 at 6:37; under
the old 25% it would have been won with about 450 left. Claim and fort did
not reach 25% either, so the change costs them nothing; nothing soft-locks
(the clock ends the round). Not a census: one seed, a smoke check.

Rejected: winning only when the slime is wiped (`MouldWiped`), since a
straggler on a crystal can eat and breed back over the broken line, the
chase the owner found annoying on 11-12.

Check: `run::tests::levels_get_harder_and_the_same_seed_gives_the_same_run`
(goal no higher deeper); the window's run test reads the new goal text.

## The run's slime is shape C (2026-11-15 08:45Z)

The slime-enemy thread (08:23Z) traced the shedding above: a run never
played shape C, the slime the owner chose as the default (11-12). The first
build's `run_args` started a level "as a mission does", with no shape
keys over the race world, so every run (the menu's Run and `run=`) played
the bare race keys at 5,000 walkers times the level's factor, plus the
tier's keys: upkeep 0.001, no shared food, no withering, panic or charge.
That slime sheds about nine tenths of itself in its first minute.

Now a run's level plays C under its tier: `run_shape` sets C's keys
before the tier's in `start` (the tier still wins where it sets a key),
and its start walkers are C's 25,000 times the level's factor
(`run_walkers`), so planet 1 lands about 15,000 (C's sandbox density on
0.6 of the map) and the core world about 340,000. C's slots (50,000 for
25,000 walkers) grow by the same factor, and both by the planet's open
cells (`open_scale`). The slime thread's note: C's growth stops at its
slots, and it is building strength that does not need more slots ("a fed
walker that can't divide thickens"), so the slots here are sized for cover
and frame cost, not as the slime's growth.

**A planet's slime varies** (owner, 08:34Z, answering the slime thread:
"It should vary and be more later on"): each level's walkers are rolled
0.8 to 1.25 times what depth and area give (`run::size_roll`, even on a
log scale), on top of a fifth more a planet and the area.

Idle hand, seed 1, llvmpipe, no head start unless said (C now):

| Planet | Landing | 0:30 | 1:00 | 2:00 | Later |
|---|---|---|---|---|---|
| 1 (0.6) | 15,442 | 16,079 | 17,773 | 17,862 | 30,877 at 7:30, 30,883 at 10:00 |
| 5 (2.0), 25 s head start | | | 87,786 | | 94,740 at 3:00 |
| 5 (2.0), fresh | 90,168 | 91,700 | 95,189 | 66,429 | |
| 8 core (6.0), fresh | 337,210 | 338,134 | 322,935 | 133,237 | |

On planet 1 C grows and holds, so idle cannot win. On the big planets the
start discs far from crystals wither (C's `wither`, a part with no news of
food thins) before the body settles; the head start does that before
landing. The goal's peak counts from 60 s after landing, so a withering
start is not counted as the player's work on planets 4-8 where the head
start runs; the core world's fresh start fell from 322,935 to 133,237
between 1:00 and 2:00, which a fresh start would have counted as 60% of
the goal (the head start covers it, 87 s before landing, but the core
world's settling should be checked in play).

Frame cost: not yet timed on the 1070. The core world's 340,000 walkers
in 700,000 slots is about 14 times C's sandbox; a GPU step scales with
walkers. The PC runner times it next; if it is too slow, the slots and the
walkers come down together (the walker factor's area term), not the shape.

Checks: `the_menu_starts_a_runs_level_like_a_mission` (C's walkers by the
level, no keys in the args: C goes on in `start`),
`run::tests::a_planets_slime_size_varies_by_seed`, the walker test now
divides out the roll.

## Pictures

[Roguelike Run](https://claude.ai/artifact/4BSDgRDDTbt2qmUAkmoFcs), archived in `page/`: the core world fresh at landing and 1:00 and with its head start, the shop's clock line; rolled bullets in the shop (three planets in; after planet 1), the second shop (after planet 1; three planets in; the Buildings and Tools folders; 1280x720 with dev mode), then the first build's start menu, planet 1 at 1:30 (the generated barrens planet; farm hand: 19 soldiers, 4,823 food, the slime 1,322 walkers at 1:30), the shop after planet 1 at two stars (250 points), and dev mode.

## Checks

- Library: `mould::run::tests` (6): harder deeper and repeatable; a level is
  a mission the campaign's parser takes; the shop sells what the points buy;
  cards fill the gun to its budget and get better deeper; dev mode and the
  home world; every ladder rung sets on the GPU engine.
- Menu: `world::start::tests::the_shop_fits_and_its_rows_pick` (every row and
  button on screen and apart at 1280x720 to 1920x1080, every UI size, 5 to
  40 rows); the main page's keyboard walk with Run second.
- Window (llvmpipe): `a_run_plays_a_planet_then_the_shop_then_the_next` (a
  won planet to the shop, a buy, planet 2 with the upgrade in its rules, a
  lost planet ends the run), `the_window_plays_a_runs_planet` (planet 3,
  every command pressed every game second for 60 s, no panic).
- No census: build first (owner, 11-12 17:26Z). Whether planet 1 can be won
  at 25%, and whether the prices are right, is the owner's playtest.

## Next

- Bigger planets deeper (the generator's next PR: 60% to 6x by depth).
- Mortar or artillery: no such building exists yet; the shop sells it once
  one is built.
- Rescue planets (base ruins, stranded soldiers; owner 15:33Z): a rescue
  goal here, the map items in the level generator.
- Saving a run between sessions.
