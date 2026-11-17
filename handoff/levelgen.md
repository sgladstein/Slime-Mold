# Handoff: procedural planets (levelgen), 2026-11-17

This area is the roguelike run's generated levels, from the thread "Generate
roguelike levels procedurally", which started 2026-11-14. The wiki page is
[`wiki/world/planets.md`](../wiki/world/planets.md) and holds the full
history with links to every report.

## What it owns

- **`src/mould/levelgen.rs`.**
  - `level(seed, depth) -> Map` and `planet(seed, depth)` (name, biome, size).
  - Eight biomes in a set order (Meadow ... Caves, then the core world, which
    is 8th), each rolled on four dials (openness, water, soil, crystal pattern).
  - Every planet is checked before play (`check`). A failed check retries, and
    an open-plan `fallback` is the last resort.
  - Results are pure, with no platform maths, and pinned by the tests
    `planets_are_pinned` and `every_dial_is_pinned`. Re-pin only when a change
    means to move a planet, and say so in the commit.
- **The walking-map cache** in `src/mould/larder_world/paths.rs`:
  `distance_map`, `MapBuild` (built a slice a step, `MAP_POPS`), `map_for`,
  `pump_maps`, `trim_maps` and `stale_maps`. `MAP_NEAR` is 12 dish cells
  (#382). How soldiers move over these maps is not this area. That code is
  `steer.rs`, `walk`, `guide` and `squads.rs`, from the squad movement rework,
  #392.

## State: everything merged, no open PRs or branches

- **Planets.**
  - Map fixes #323, front doors and back roads #327, fen lakes #329, caves #337.
  - Per-biome openings #345, rivers off the nests #352, cave gates off pools #355.
  - Map sharing for 200 soldiers #382.
  - All the map items from the three planet reviews are done
    ([review](../Reports/levelgen-review-2026-11-15/README.md)).
- **Tools.**
  - `cargo run --release --example planets -- seeds=1-20 depths=D out=DIR`
    writes a survey plus maps (PNG and TXT) for each planet.
  - `larder-world map=planet:D:S` plays a planet. `look=DIR hand=idle` renders
    frames (llvmpipe works in the cloud).
  - Counting scripts live in the reports' `raw/` folders: `gatewater.py`,
    `nestwater.py` and `fc.py` (false-colour layouts).
  - `examples/soldiercap` and `move_tests.rs` (`MOVE=planet8`) measure the
    map cache at 6x.

## Waiting on the owner

- **Fen moat or rock rims**: a blind A/B, with the key in
  `Reports/levelgen-fen-moats-2026-11-15/raw/ab-key.txt`. The moat
  (`Recipe::moat`) is built but off. The owner couldn't see the difference
  (11-15), so it stays off unless one side stands out in play.
- **Breather planet**: the owner chose "Wait for your playtest" (11-15). There
  is no smaller rich planet yet.
- **Looks to judge in play**:
  - the dark clay bank under a cave gate on a sill (cave seed 16, zoom 6);
  - whether a river valley with one river plays as well as one with two.

## Next steps, none started

- Twists shown before landing, and rare rescue planets (base ruins, stranded
  soldiers). Do these with the run's code (`wiki/world/run.md`).
- Coarser walking maps at 6x, for example on 2x2 blocks. When all squads are
  ordered at once, the 8 new maps queue for about 3 s
  ([report](../Reports/map-sharing-2026-11-16/README.md)).
- The pins have never been run on Windows. A planet built on Windows should
  match one built in the cloud.
- `wiki/world/planets.md` is over the wiki's 12 KB cap and should be split
  (the lint warns).

## Notes for whoever picks this up

- Set slime ("bulwark", #413, off by default) and doors (#395) both change
  `standable_cells`. The bulwark calls `stale_maps` at most every 5 game
  seconds, and only after 16 or more cells change. Making every map stale
  more often than that never lets the queue drain at 6x: one map takes 6-7
  steps, and up to 34 are cached.
- When a planet change moves the queue, the usual clash is two appended
  lines in `Reports/README.md`, `Reports/pages.md` or `Reports/dead-ends.md`.
  Merge main, keep both lines, then label the PR `queue` again.
