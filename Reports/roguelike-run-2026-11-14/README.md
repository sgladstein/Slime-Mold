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
  planet 1, 3 points less a planet, 10% from planet 6. The mission tracker's
  stars, stuck and cap rules apply unchanged. Free play's bite (`drain` 2),
  not the missions' 1.
- **Ground**: until the level generator lands
  (`levelgen::level(seed, depth) -> Map`, agreed with its thread 15:25Z), a
  planet plays a hand-made map, Flats first, then the other four in an order
  the run's seed keeps (`Ground::Named`). `Ground::Made(Map)` is wired in the
  window already (`GpuWorld::with_map`).
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

## Pictures

[Roguelike Run](https://claude.ai/artifact/4BSDgRDDTbt2qmUAkmoFcs), archived in `page/`: the start menu, planet 1 at 1:30 (farm hand: 14 soldiers, 3,771 food, the slime 5,017 walkers at the start and 2,135 at 1:30), the shop after planet 1 at two stars (250 points), and dev mode.

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

- Planet names and biomes from `levelgen::planet(seed, depth)` on the shop
  and the start alert; the generated maps (sizes 60% to 6x by depth).
- Mortar or artillery: no such building exists yet; the shop sells it once
  one is built.
- Rescue planets (base ruins, stranded soldiers; owner 15:33Z): a rescue
  goal here, the map items in the level generator.
- Saving a run between sessions.
