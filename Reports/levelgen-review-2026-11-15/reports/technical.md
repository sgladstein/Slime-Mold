# Technical review: procedural planets at 4494ad4f

Reviewer: technical. Everything was run in /home/user/review-technical. Planet sweeps used the prebuilt `planets` binary. Five review tests were built into a copy of the snapshot (`sm-copy/`, sources in `tests-src/`). Builds used my own target folder, seeded from a copy of the existing release deps. The first build took 1 min 27 s and each rebuild 1.5-2.7 min. Raw output is in `runs/`.

## Big picture

1. **Walking maps are the weak part at 6x.** Each goal gets its own Dijkstra map over the whole world. At 6x that is 985k fine cells, about 8 steps (0.5 game s) at `MAP_POPS`. It works while goals are few and fixed, as in every bench (the rush hand has one goal, the core). It breaks when goals move or are many: steering, formation slots, many squads. Recommendation: build walking maps on the 4-cell bins the levelgen check already walks. That is about 62k cells at 6x, one step per map. Do fine steering locally. HPA* (a cluster graph over 32-cell tiles) is the standard answer at this size.
2. **The checks walk a different grid from the game.** `check` walks map-cell bins, with the map's edge open. The game walks the dish's fine cells, and the dish has no agar within 3 dish cells of its edge. Run the checks on the game's own grid; `BigDish::with_map` runs on the CPU.
3. **What the measurements missed:**
   - Benches never steer, never have more than a few distinct goals, and raise walls only once a second.
   - The first game second is cut from the maxima (bench.rs:165). By the code's own note it held a 327 ms step at 6x on the 1070: an untracked hitch at every landing.
   - The pins have never run on Windows. CI runs only on ubuntu, and the Windows target is only `cargo check`ed.
4. **Generation determinism looks right.** levelgen uses only `+ - * /`, `sqrt`, `floor` and `round`. It has no hash maps and no threads, and Rust does not fuse multiply-adds. The gap is evidence, not code (see the test findings below).

## Ranked next directions, each with a first experiment

1. **Steered maps:** one-line fix, tested below. First experiment: add `review_steered_squad_round_rock` to the suite.
2. **Coarse-bin walking maps:** build maps on 4x4 bins for 50 planets. Compare path length with fine maps, and the step max at 6x on `hand=every` with 8 squads.
3. **Checks on the game grid:** add the dish rim to `walkable`. Re-run 1,600 planets and count the flips.
4. **A Windows pin job:** one CI run of `levelgen::tests` on windows-latest. Pin one seed per dial value, not just seed 1.
5. **A real fallback:** build the fallback for every planet and require it to pass `check`.

## Every feature, technically

These are from 1,600 planets (seeds 51-250, depths 1-8): none used the fallback, and the worst needed 5 attempts. The forced fallback was tested on 96 planets (seeds 1-12) and the rim on 320 (seeds 1-40).

- **Ground.** Passes are cut through every ridge. Rooms (Highlands) retry most: 49 of 200 needed more than one attempt. A ridge running to the map's edge can fool the check (finding 4, planet 5 seed 39). Caves are 54% rock even in the fallback.
- **Water.** Deep water is unwalkable in both the check (`map::standable`) and the game (`open = agar && !deep`). `cut_fords` left no seed cut off. The only crossing test is the 1x synthetic river.
- **Soft rock.**
  - Core worlds at 6x carry 272-374 wall items.
  - A broken wall cell triggers a whole-agar read-back (finding 6).
  - Half the charge behind soft rock still passes: Fen seed 207 had 53% of its charge open.
- **Crystals.** Solid discs in the check match the game. 25 of 200 planet 1s have 7 or fewer crystals.
- **Nests and landing.** A nest is only checked through its home crystal; `to_first == MAX` alone never fails.
- **Size.** The slowest 6x build was 3.4 s (planet 8 seed 157, five attempts). Build it while the shop is open. Every `planet:D:S:0.25` and `:0.4` at depths 4-8 falls back, because the nests cannot fit.
- **Fallback.** See finding 5.

## Findings, most serious first

**1. CONFIRMED: a steered squad pressed against rock stalls at 6x and runs at half speed at 1x.**
- Where: squads.rs:432 (`near = lead / 2.0`), with paths.rs:314-357 (`map_for`) and 365-424 (`pump_maps`).
- Cause: the steered point 40 cells ahead moves one step (`lead`) every time the anchor moves. That is farther than `lead / 2`, so every move asks for a new map. The anchor then waits for that map: 2 steps at 1x, about 8 at 6x.
- Run: `review_steered_squad_round_rock`. A rock bar between the landing and the steer direction, 4 soldiers, steered east, at the same coordinates on a 1x and a 6x world.
- Result, time for the middle to get past the bar:

  | World | Sliced maps (today) | Whole maps |
  | --- | --- | --- |
  | 1x | 26.3 s | 12.9 s |
  | 6x | never in 60 s (anchor still for 798 of 900 steps) | 12.9 s |

- Fix: steered `near` = 10 dish cells. In my copy that gives 13.6 s at 1x and 16.4 s at 6x.

**2. CONFIRMED (mechanism), SUSPECTED (frequency): the map cache thrashes at 6x above 17 live goals.**
- Where: paths.rs:413. The cache holds `MAP_CELLS_MAX / cells` maps, which is 17 at 6x.
- Run: `review_walking_map_cache_thrash`. N goals each asked every step for 1,500 steps.
- Results at 6x:

  | Live goals | Goal-steps unserved | Mean `pump_maps` per step |
  | --- | --- | --- |
  | 17 | 0% | 0.5 ms |
  | 18 | 5.1% | 6.8 ms |
  | 24 | 26% | 9.2 ms |
  | 32 | 41% | 6.0 ms |

  For 18 or more goals the mean stays high because the maps are rebuilt forever. At 1x nothing is lost (48 maps).
- Queueing: even without thrash, 32 goals all get their first map only after 235 steps (15.7 game s) at 6x, against 38 steps at 1x.
- Fix: cap the cache by map count, not cells (48 × 3.9 MB = 188 MB), or store `u16` distances.

**3. CONFIRMED: walls finishing every 0.4 s at 6x starve the map being built.**
- Where: paths.rs:429-437. `stale_maps` throws away the map in progress and restarts it.
- Run: `review_walls_in_succession_restart_the_build`.
- Result at 6x: with a wall every 2, 4 or 6 steps, the waiting goal got no map in 600 steps. With one every 8 steps it got one at step 15. At 1x it is always served by step 3.
- Fix: let the build finish, then mark that map stale.

**4. CONFIRMED: the checks pass a planet whose landing is walled in, in the game.**
- Where: levelgen.rs:2174. `walkable` never narrows the edge bins and treats the edge as open, but the dish has no agar within 12 map cells of its edge (big.rs:1520, 1654).
- Run: `review_checks_with_the_dish_rim` (whose rim-as-rock pass also narrows the bin next to the rim, so it over-flags), then `review_planets_through_the_rim` on the game's own `standable_cells`.
- Result on planet 5 seed 39:
  - levelgen says 100% of the charge is open without breaking soft rock.
  - The game's grid reaches 3 of 20 crystals without breaking soft rock.
  - With the rim opened, the game reaches 20 of 20.
- Over seeds 1-40 and depths 1-8, this is 1 of 320 planets. (The pass also flagged two other planets; the game's grid reaches every crystal on both, so they are artefacts of that extra narrowing.)
- Fix: shut the bins within 12 map cells of the edge in `walkable`.

**5. CONFIRMED: the fallback is neither open nor checked.**
- Where: levelgen.rs:251-252. The fallback keeps fronts, pillars, caves, hedges, rings, rivers and front-border ridges.
- Run: `review_fallback_maps`.
- Result: 6 of 96 forced fallbacks fail their own checks. Planet 8 seed 11 had 46% of its charge open; planet 6 seed 1 had its nearest crystal 329 cells' walk away. Cave fallbacks are 54% rock.
- Fix: in the fallback, set `fronts`, `pillars`, `front`, `hedges` and `ring` to 0, and test that every fallback passes `check`.

**6. SUSPECTED: a hitch whenever a wall cell breaks at 6x.**
- Where: gpu/larder_world.rs:463-469. Each step in which the broken count moves reads back the whole agar buffer (3.9 MB at 6x) and loops over every cell. The pattern #307 removed for gas.
- Estimate: about 3 ms on the 1070, scaled from the report's own 47 MB news-buffer timing. The core world rings every nest, so this will happen often.
- Not measured: my 6x idle bench did not finish under the machine's load, so I stopped it.
- Fix: read back only the wall cells' agar, through the existing `Gather`.

**7. CONFIRMED: test gaps.**
- `a_big_planet_is_pinned` duplicates `PINNED[7]`: both are `0x6a8d5a9e7fc09b42` (levelgen.rs:2425-2432), so it tests nothing new.
- No test drives `map_for` or `pump_maps` scheduling (sharing, stale maps, eviction). Findings 1-3 all pass the suite. `the_walking_map_ring_matches_the_heap` only covers `MapBuild` on mazes up to 64 × 48.
- The pins cover seed 1 only, so lakes, oases, pillars, fronts and bridges are unpinned.
- Fix: add my four walking-map tests, and pin one seed per dial value.

Not found: no hash-map order, thread timing or platform maths in levelgen. Sliced maps equal whole ones (same pop order). `read_back_range` and `Gather` offsets are correct. `mark_far_box` reads the full reach.
