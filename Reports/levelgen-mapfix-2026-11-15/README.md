# Map fixes from the planet reviews (2026-11-15)

The three planet reviews ([report](../levelgen-review-2026-11-15/README.md))
listed map items for this thread. This change does the ones about looks and
spacing first, since looks are the owner's first concern (00:09Z): items 5, 9
and 10 of the ranked list, and the core world half of item 5. The front door
and back road for every nest (item 4) come next.

- Before: main at 4494ad4f, the build the reviews looked at. After: this
  change, on top of the reviews' technical fixes (PR #312).
- Page: [Planet Map Fixes](https://claude.ai/artifact/HPbTQJWDTe3qUTktVtuFqA), copy in `page/`.
- 160 planets measured (seeds 1-20, depths 1-8) from the planets example's
  map files: `cargo run --release --example planets -- seeds=1-20 depths=1-8 out=DIR`
  in each tree, then `python3 raw/measure.py BEFORE AFTER`
  ([table](raw/measures-table.txt)) and `python3 raw/survey.py` on the two
  surveys ([table](raw/survey-table.txt)).
- In-game frames: `larder-world` on llvmpipe, `hand=idle`
  ([`raw/renders.sh`](raw/renders.sh); close views centred by
  [`raw/points.py`](raw/points.py)); false-colour layouts by
  [`raw/fc.py`](raw/fc.py); the page's pictures and films by
  [`raw/make_pics.py`](raw/make_pics.py); the numbers beside each picture by
  [`raw/pics.py`](raw/pics.py) ([output](raw/pictured-planets.txt)).

## What changed (`src/mould/levelgen.rs`)

1. **Nests off the edges.** Nests keep at least 0.16 of the map's width (and
   0.2 of its height) from the edges, between 150 and 240 map cells. The
   slime's start is a blob of radius 130-190 map cells, so at 110 and 100 the
   frame cut it.
2. **Nests spaced by the map's width.** Two nests stand 0.3 of the map's
   width apart, not 280 cells at every size. Where the far part has no room,
   the spacing gives way by a tenth every 50 tries, and after 400 tries the
   farthest candidate is kept, so no planet loses a nest.
3. **Streams and rivers capped.** Streams keep the 1x count on a bigger map
   (they grew with the square root of the area: a 6x core world had a web of
   them), and rivers stop at two (core seed 5 had four parallel ones).
4. **Nest rings in arcs.** Two or three arcs, each at its own radius
   (0, +16% and -10% of the ring's) with a gap after each, starting at a
   random piece, not one round ring with one gap: rings read as bullseyes.
5. **No hedges on water.** A ring or free hedge piece is left out where
   either end or its middle is wet.
6. **Straight bridge decks.** The deck is a band that runs 6 cells past the
   widest the banks reach, not the river's ragged outline.
7. **Tapered fords.** The deep middle narrows to a ford over 9 cells either
   side, so the ford's edge runs smooth instead of stopping square.

All 8 + 6 pins re-pinned (`planets_are_pinned`, `every_dial_is_pinned`);
new test `nests_keep_off_the_edges_and_apart` (seeds 1-6 of every planet:
every nest 150 or more from every edge, every pair 0.2 of the width apart or
more).

## Measured (160 planets, before -> after)

| Planet | Nests within 150 of an edge | Nearest pair / width (min, median) | Hedge pieces on water | Water share |
|---|---|---|---|---|
| 1 Meadow | 17/20 -> 0/20 | one nest | 60/166 -> 2/114 | 0.120 -> 0.134 |
| 2 Barrens | 14/20 -> 0/20 | one nest | 0/48 -> 0/32 | 0 |
| 3 Fen | 17/20 -> 0/20 | one nest | 23/68 -> 12/48 | 0.243 -> 0.234 |
| 4 River valley | 21/40 -> 0/40 | 0.23, 0.31 -> 0.28, 0.32 | 4/68 -> 10/76 | 0.049 -> 0.050 |
| 5 Jungle | 18/40 -> 0/40 | 0.22, 0.27 -> 0.28, 0.33 | 1,143/4,518 -> 53/4,130 | 0.136 -> 0.110 |
| 6 Highlands | 20/40 -> 0/40 | 0.18, 0.25 -> 0.27, 0.33 | 20/208 -> 15/196 | 0.071 -> 0.063 |
| 7 Caves | 18/40 -> 0/40 | 0.18, 0.35 -> 0.30, 0.33 | 164/936 -> 217/1,052 | 0.074 -> 0.074 |
| 8 Core | 16/60 -> 0/60 | 0.12, 0.18 -> 0.25, 0.33 | 1,104/6,628 -> 69/6,114 | 0.105 -> 0.083 |
| All | 141/280 -> 0/280 | | 2,518/12,640 -> 378/11,762 | |

Cost and checks, same 160 planets ([table](raw/survey-table.txt)): first-try
passes 143 -> 149 of 160; median build time within 10% on every planet
(planet 8: 577 -> 561 ms); the walk to the far nest 1-8% shorter (nests no
longer in the corners; planet 7: 2,174 -> 2,004 cells, still above the
checks' 0.6 of the width); the median planet still opens 99-100% of its
charge without breaking soft rock.

## Not fixed here

- **Hedges on water in the caves rose** (164 -> 217 pieces): they are gates
  across tunnels with a pool in them. A gate is the pass's lock, so it stays;
  a gate could move to the dry end of its tunnel next.
- **The same seed lays out differently.** Nest placement draws more random
  numbers, so everything after it moves; compare the kinds of change, not the
  positions.
- **River valley seed 2's near nest now sits by the bridge**, so the slime is
  at the bridge from the start. Keeping nests off a river's crossings belongs
  with the front door and back road work.
- **The far view below zoom 1 is unshaded** (review item 1, drawing). The
  whole-map pictures of the 6x core world use it.

## Next

A front door (a direct way at most 10 wide) and a back road (at least 16 wide)
for every nest, checked in `check`; first experiment, 50 seeds a biome with
the fun reviewer's `localchoke.py`. Then fens weighted to lakes, cave tunnels
5-15 wide, a per-biome landing crystal.
