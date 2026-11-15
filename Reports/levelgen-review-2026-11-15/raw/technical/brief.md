# Adversarial review brief (three reviewers)

You are one of three independent reviewers of the procedural planet work in
the Slime-Mold repo: one on aesthetics, one on fun, one on technical checks.
The snapshot to review is the git worktree /home/user/sm-review (commit
4494ad4f); its history holds the planet PRs #277, #284, #289, #302, #306, #307
and the 6x size change (`git log --oneline -15`, `git show`). The owner
asked for an adversarial review: your job is to find what is wrong, missing
or weak, and to say how to make it better, not to confirm it.

## What the work is for

The game is Larder World: soldiers against a slime mould on a GPU dish. A
roguelike run takes a squad through 8 generated planets toward the slime's
core world. `levelgen::level(seed, depth) -> Map` makes each planet: its
ground (rock, wet, soil kinds, deep water, bridges), plants, crystals (the
energy both sides want), slime nests, the landing, moisture for fire, and
the map's size. The owner's rulings for planets (quoted in
wiki/decisions-game.md and Reports/levelgen-2026-11-14/README.md):

- Fun over fidelity: interesting simulations that make a game; real
  landscapes, ecology and geology are inspiration, not ignored and not the
  bar.
- Way more variability between biomes; biomes are dials rolled per planet.
- No biome forces a strategy: every map is beatable by any build (guns,
  turrets and walls, kill boxes, sneaking, sniping, artillery, jumping).
- Open early, mazes late. Sizes from 0.6x today's map on planet 1 to 6x
  area at the core.
- Deep water stops soldiers; slime crawls through it slowly.
- Everything on screen is the simulation. The owner plays zoomed in, so
  every edge must be crisp at close zoom.

Planets now grow to the owner's sizes: 0.6x, 0.8x, 1x, 1.4x, 2x, 2.8x, 4x
and 6x today's map area on planets 1-8 (`AREA`, `GROW_CAP`). The core world
(planet 8) has a few open arenas and a hedge round every nest. The work
also makes big planets run smoothly: a sparse ground refresh
(`gpu::Gather`, src/bin/larder-world/draw.rs `refresh_ground`, `live_cells`,
`live_code`; walls re-mark only their tiles, src/world/look.rs
`mark_far_box`) and soldiers' walking maps built a slice a step
(src/mould/larder_world/paths.rs `MapBuild`, `Game::map_for`,
`Game::pump_maps`; squads.rs `guide`). And it sows plants per biome with
fading edges (levelgen.rs `Sow`, `sow`).

Read first: CLAUDE.md, wiki/looks/art-direction.md (the style guide; every
reviewer should know the look rules), Reports/levelgen-2026-11-14/README.md
(the report),
Reports/levelgen-2026-11-14/biomes-plan.md, wiki/world/planets.md, then the
code above.

## Lead with the big picture

1. Is the approach right for the goal (a run of planets that play well and
   differ, and run smoothly at 6x)? What would you do differently?
2. What did the tests and measurements miss: controls, confounds, planets
   never looked at, claims with no evidence behind them?
3. Inspiration from outside: published procedural map generation (strategy
   games' map fairness and chokepoint scoring, roguelike level design,
   terrain synthesis, pathfinding at scale such as flow fields and
   hierarchical maps) and real landscapes, where they suggest something
   better.
4. Ranked next directions, each with a first experiment that would settle it.

Then correctness: concrete failures with file:line, the input that breaks
it, and what you ran to see it.

## Every feature of the world

Whatever your angle, examine each feature a planet can have, one by one,
and say how it does from your angle, with the planets and seeds you looked
at:

- Ground: open ground, outcrops, pillars, ridges and their passes, rocky
  rooms and gates, caves and tunnels, the core world's arenas.
- Water: pools, lakes, rivers with deep middles, fords and bridges,
  streams, the way `cut_fords` makes a crossing shallow.
- Soil: plain, sand, dunes, barren bands, oases, clay, cracked clay, mixed.
- Soft rock: hedges, rings round nests, grown soft rock.
- Plants: each biome's cover, reed beds at water, patch edges, moisture for
  fire.
- Crystals: scattered, fields, veins, one jackpot, at the oases, richer
  toward the slime; big and small crystals.
- Nests, the landing, the map's size and shape, and the fallback map.

## Your angle

**Technical.**

You review whether the planet work is correct, fast, deterministic and well
tested, fully focused on that. Leave the look and fun to the other two.

- **Determinism.** Same seed and depth give the same planet on every
  machine: no platform maths, and pinned fingerprints. Find anything that
  could differ on Windows or another GPU: float functions, hash-map order,
  thread timing.
- **Checks.** Are the fairness checks (reachable crystals and nests, a
  crystal near the landing, open charge) right? Find seeds that pass the
  checks but play badly, or that need many attempts or the fallback. Run
  the planets example over many seeds and depths.
- **Speed at 6x.** Check the claims in the report against runs: walking
  maps a slice a step (`MapBuild`, `map_for`, `pump_maps`), stale maps,
  the cache cap, the sparse ground refresh (`Gather`), and the gas
  read-back (`read_back_range`). Look for a hitch the benches miss:
  first-second costs, many squads, walls built in quick succession, maps
  larger than any timed. Check memory too.
- **Correctness.** Do sliced maps give the same distances as whole ones? Can
  a soldier wait forever for a map? What happens when goals change every
  step? Are the edge cases right (map edges, 4096-cell maps, deep water
  with no ford)?
- **Tests.** Which claims have no test? Which tests would pass even if the
  feature broke?
- **Output.** Concrete failures with file:line, the input that breaks it
  and what you ran, marked CONFIRMED or SUSPECTED, ranked, each with the
  smallest fix.

## Rules

- Do not edit files in /home/user/Slime-Mold or /home/user/sm-review, do not
  commit, push or comment on GitHub. Work in your own folder: /home/user/review-technical.
- Prebuilt binaries of the snapshot are in /home/user/sm-review-bin
  (larder-world, planets). Do not run cargo in /home/user/Slime-Mold or
  /home/user/sm-review. The machine has 4 cores shared by three reviewers
  and the author: if you must build changed code, copy the repo to your
  folder and build there with its own target folder (a first build takes
  10+ minutes), and say how long it took. Prefer the binaries and reading.
- Useful commands (from /home/user/review-technical):
  `/home/user/sm-review-bin/larder-world look=DIR map=planet:DEPTH:SEED seed=1 hand=rush at=5,20 screen=1280x800`
  renders frames (PNG) of a planet in play (`centre=X,Y,ZOOM` in world
  units for close views; software Vulkan, about 1 s a frame;  takes
  fractions, so  gives GIF frames, which PIL joins);
  `/home/user/sm-review-bin/larder-world bench=10 seed=1 map=planet:D:S:AREA hand=rush spread=0`
  times frames (the "frames with a step" line's step max is the game step);
  `/home/user/sm-review-bin/planets seeds=1-3 depths=1-8 out=DIR` writes
  planets as map files (PNG: G byte ground, B byte plants; TXT items) and
  prints their numbers. Test names are in the source's tests modules. The
  page images are in /home/user/sm-review/Reports/levelgen-2026-11-14/page3/ and page4/.
- Look at pictures before measuring. Quote numbers with what you ran.
- Save your report as /home/user/review-technical/report.md and return the same text. Keep it under 1,500 words. End with your findings ranked most serious
  first, each marked CONFIRMED (you reproduced it) or SUSPECTED, with the
  smallest fix you would make.
