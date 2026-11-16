# Walking maps shared across a formation (2026-11-16)

The soldier-cap thread raised the cap from 40 to 200 (PR #377). On a 6x
planet the walking maps (`MapBuild::advance`) cost 4.9 ms a step at 40
soldiers and 6.0 at 200. The technical lead asked how many distinct maps
are built a step at 200, and whether sharing was biting.

**Outcome:** the map builder's cost doesn't grow with the number of maps.
It works `MAP_POPS` cells a step whatever is queued, and it was busy on
nearly every step at 40 and at 200. Sharing wasn't biting, though. At 200,
8 squads asked for 45 distinct goal cells a step, and 47% of map lookups
went unserved, so those soldiers stood and waited. The cause: a map served
only goals within `MAP_NEAR` 4 dish cells, but a squad of 25's formation
reaches 12.5 dish cells from its middle. `MAP_NEAR` is now 12. Unserved
lookups at 200 went from 47% to 19%, mostly in the few seconds after all 8
squads were sent at once.

- Before: main at f1be524c. After: this change.
- Runs: `examples/soldiercap depth=D n=N secs=30` (8 squads, half fight and
  half clear, new picks every 15 game s), with the map lookups logged by a
  temporary patch ([`raw/instrument.diff`](raw/instrument.diff): `MAPDBG=1`
  logs every `map_for` and `pump_maps`, `MAPNEAR=R` sets the radius). Runs
  one at a time on the cloud CPU ([outputs](raw/), logs in
  [`raw/lookups.tar.gz`](raw/lookups.tar.gz)), counted by
  [`raw/mapstats.py`](raw/mapstats.py) ([counts](raw/mapstats.txt)).
- A sweep of 8, 12 and 16 at 200 soldiers, three runs at once, so its times
  are not comparable: [`raw/sweep-*`](raw/). Unserved lookups every 2 game
  seconds: [`raw/unserved-by-time.txt`](raw/unserved-by-time.txt).

## Measured

Core world (planet 8, seed 1, 1256 x 784 fine cells, 6x), 450 steps.

| | 40, radius 4 | 40, radius 12 | 200, radius 4 | 200, radius 12 |
|---|---|---|---|---|
| Steps the builder worked | 415 | | 449 | |
| Maps started | 68 | 36 | 75 | 68 |
| Distinct goal cells asked a step, mean (max) | 11.1 (18) | 9.7 (16) | 45.0 (69) | 30.6 (60) |
| Map lookups unserved | 16.1% | 12.6% | 47.4% | 19.0% |
| Soldiers' turn, ms a step, mean | 6.40 | 3.77 | 10.00 | 10.42 |
| Soldiers' turn, ms a step, p95 | 10.24 | 8.54 | 14.39 | 15.27 |

- The builder's own cost was 4.9 ms a step at 40 and 5.4 ms at 200 (mean of
  `pump_maps`): a 6x map takes 6-7 steps, and at most one is started a step.
- At 200 the soldiers' turn rises 0.4 ms at radius 12: soldiers that waited
  now walk.
- Over time at 200 (unserved, every 2 s): at radius 4 it climbed from 14% to
  47% through the first pick as squads closed on their places and spread
  into formation, and stayed at 88% for 4 s after the second. At 12 it held
  at 5-13% and peaked at 53% after the second pick (at every radius, 90% in the first 2 s, while the first 8 maps are built). At 16 the sweep was
  about the same as at 12 (17.4% against 19.0%).
- Planet 1 (400 x 248): a map is built within its step; unserved 0.8-2.7%
  either way.

New test `squads_of_25_share_walking_maps_at_6x`: 200 soldiers' formation
places at 8 places across a 6x map, each asking for its map every step for
600 steps. At 4, only 89 of the 200 places got a map and 9,002 goal-steps
were lost after a first map (the cache, 34 maps at 6x, thrashing). At 8,
13,405 were lost. At 12 and 16, none.

## What changed

`MAP_NEAR` 4 -> 12 dish cells (`src/mould/larder_world/paths.rs`). A map
still serves a goal only when the map's goal has a straight walk to it. The
callers are `Game::walk` and a sent squad's `guide`, which the squad
movement thread is reworking; it keeps the new test passing.

## Not done here

- **The burst after a mass order.** Sending all 8 squads at once queues 8
  new maps, about 50 steps at 6x, so the last squad waits about 3 s. Coarser
  walking maps at 6x would cut that: the raid thread's slime field went from
  62 to 6 ms on 2x2 blocks.
