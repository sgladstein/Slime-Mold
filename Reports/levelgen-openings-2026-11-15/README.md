# A different start on each biome (2026-11-15)

The play review ([report](../levelgen-review-2026-11-15/reports/fun.md), item
4, confirmed) found every planet opened the same way: one crystal 85-140 map
cells from the landing, toward the middle, 24-28 across, on 64 of 64 planets.
Its fix was to set the landing crystal's distance and size per biome (Age of
Empires II gives each map a start built for it).

**Outcome:** each biome now has its own opening (`Opening` in
`src/mould/levelgen.rs`). The meadow keeps the old start, so the first planet
plays as before. The rest:

| Biome | Opening | Distance (map cells) | Radius | Charge |
|---|---|---|---|---|
| Meadow | one, toward the middle (as before) | 85-140 | 24-28 | 192 or 256 |
| Barrens | one small, a walk off | 140-185 | 18-22 | 128 or 192 |
| Fen | two middling | 100-160 | 20-24 | 192 |
| River valley | one big, across the floor | 120-180 | 28-32 | 256 or 384 |
| Jungle | three small, close by | 60-110 | 15-18 | 128 |
| Highlands | one rich, a climb away | 150-195 | 28-31 | 384 |
| Caves | two small, in the landing's cavern | 50-95 | 18-22 | 192 |
| Core | one rich, toward the slime | 180-230 | 34-38 | 512 |

When the band toward the middle has no room, a crystal goes any way round out
to a farther limit (220-260), then onto a road, as the old start did. On the
core world the opening faces the nearest nest, not the middle.

- Before: main at bef8f1fa. After: this change.
- Page: [Planet Openings](https://claude.ai/artifact/WUT2XYmq9hBux7kqLw8uAw), copy in `page/`.
- 160 planets (seeds 1-20, planets 1-8):
  `cargo run --release --example planets -- seeds=1-20 depths=1-8 out=DIR`
  ([before](raw/survey-before.txt), [after](raw/survey-after.txt)).
- The opening by biome: [`raw/opening.py`](raw/opening.py) on those maps
  ([before](raw/opening-before.txt), [after](raw/opening-after.txt): medians
  over 20 seeds, then seed 2's crystals one by one).
- In-game frames: `larder-world` on llvmpipe, `hand=idle`, the landing at zoom
  3 at 1 s, seed 2 of each planet ([`raw/renders.sh`](raw/renders.sh));
  pictures by [`raw/make_pics.py`](raw/make_pics.py). Render times in
  [`raw/render-timing.txt`](raw/render-timing.txt).

## Measured (20 seeds a planet)

Distances in dish cells (half a map cell), straight from the landing.
"Energy" is the crystals' radius squared times charge, in map cells, summed
within 130 dish cells of the landing (thousands).

| Planet | Nearest crystal, median (range) | | Crystals within 130 | | Energy within 130 | |
|---|---|---|---|---|---|---|
| | before | after | before | after | before | after |
| 1 Meadow | 44 (37-64) | 44 (37-64) | 5.5 | 5.5 | 428 | 428 |
| 2 Barrens | 52 (40-66) | 74 (40-89) | 4 | 4 | 295 | 200 |
| 3 Fen | 52 (37-64) | 56 (38-68) | 3 | 4 | 232 | 253 |
| 4 River valley | 51 (37-79) | 69 (35-88) | 5 | 3 | 339 | 475 |
| 5 Jungle | 48 (35-72) | 40 (35-51) | 4 | 5 | 279 | 201 |
| 6 Highlands | 46 (35-95) | 81 (36-114) | 2.5 | 2.5 | 223 | 404 |
| 7 Caves | 44 (39-55) | 40 (38-47) | 3 | 3.5 | 215 | 189 |
| 8 Core | 60 (39-100) | 94 (44-115) | 2 | 1 | 180 | 701 |

The nearest crystal is sometimes one of the planet's own fields, not the
opening (barrens, valley and highlands seeds where a field lies close).

| | Before | After |
|---|---|---|
| First-try builds (160) | 143 | 141 |
| Fallbacks | 0 | 0 |
| First-try builds by planet | 18 20 18 18 19 15 19 16 | 18 19 18 18 19 13 19 17 |

The highlands lost two first tries: a rich crystal in a rocky room can narrow
a nest's way round, or sit across a room from the landing (a 276-cell walk
on seed 17, over the check's 200), and the planet is built again. A tuning
note; no planet fell back to the open plan.

New test `each_biome_opens_its_own_way`. Planets 2-8 and their dial pins are
re-pinned; the meadow and its pins are unchanged.

## Tried first

- **The highlands' crystal at 32-36 across:** the same 13 of 20 first tries
  as 28-31, so it went down to 28-31 (a rich charge, not a big disc, is the
  prize). The barrens at 150-200 and the highlands at 160-210 put the first
  crystal past the check's 200-cell walk on four planets (224, 205, 204, 217);
  both came in to 140-185 and 150-195.
- **Films with the greedy hand** (it pumps the nearest crystals on the Fork):
  on planets it harvested nothing before or after, and on the jungle the
  squad was wiped by 25 s both times, so the page shows stills.

## Not done here

- Whether each start plays as its name says is for the owner's play; the
  starting bank and squad are the run thread's.
- Nests off river crossings.
