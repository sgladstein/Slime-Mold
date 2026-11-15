# Each planet's own ground (2026-11-15)

This is item 3 of the art lead's planet review (11-15, https://claude.ai/artifact/Kcy1AoquWJjJ3EciyA3KxP): each biome gets its own ground micro-look at zoom 12. Plain soil, which is most of most maps, was one flat colour up close. Now it is drawn per biome:

| Biome | Ground |
| --- | --- |
| Meadow, core | Loam: soft crumbs at two scales. |
| Barrens | Grit: pale and dark specks, with faint wind streaks. |
| Fen | Mud: darker wet blotches a cell or two across, with wet glints. |
| Valley | River silt: soft bent bands, with pale pebbles. |
| Jungle | Humus: dense leaf-litter mottle. |
| Highlands | Gravel: stones about two thirds of a cell across, each its own shade, with dark gaps. |
| Caves | Stone dust: a faint grain over broad drifts. |

The biome comes from `P.flora`. That is the plants thread's per-map value (#315, `levelgen::Biome as u8`), so the view keeps one uniform for both. Maps without a flora line get flora 0, the meadow loam.

## How it works

The look lives in `plain_look` in `src/world/view.wgsl`, called from `soil_detail`.

- **Strength.** It changes the soil's colour by at most about a quarter either way (`PLAIN_AMP` 2.0). The first cut, at 1.0, read only as a faint grain over ground at about 40 grey.
- **Zoom.** It comes in from zoom 3 to 8, and stays out below that, where it would alias.
- **Under the slime.** It lies under the slime by `bare`.
- **No grid squares.** Every grid is turned off the cells' axes, so nothing shows squares at zoom 24.
- **The bug that hid it.** On main, a pixel of plain soil alone skipped `soil_detail` entirely, so plain soil could never take a texture. The early-out now applies the biome's look first.

## Measured

Measured with the reviewer's `measure.py` on planets 1-7, seeds 1-2. Plain bare ground, p10 / p50 / p90 grey, with the look off → on:

| Planet, seed | Plain ground, look off | Plain ground, look on |
| --- | --- | --- |
| Meadow 1 | 33/46/61 | 32/46/56 |
| Meadow 2 | 42/43/51 | unchanged |
| Barrens 1-2 | 13/42/56 | unchanged |
| Fen 1 | 42/48/51 | 29/34/42 |
| Fen 2 | 42/48/51 | 29/36/48 |
| Valley 1 | 38/43/53 | 40/44/55 |
| Valley 2 | 42/43/51 | 42/43/55 |
| Jungle 1 | 24/40/55 | 19/42/51 |
| Jungle 2 | 25/38/53 | 18/39/52 |
| Highlands 1-2 | 42/42/42 | 42/42/42 |
| Caves 1 | 42/42/53 | 42/42/55 |
| Caves 2 | 42/42/53 | 42/42/56 |

Some rows look unchanged because the measure registers a few cells by their middle pixel, which smooths the texture out. The stills below show it.

**The value band holds.** Every p50 is 34-46. The fen's mud is darker, as mud should be, and inside the 20-45 band.

**The slime film** is measured against #317. That comparison also includes the plants thread's #315, merged in between:

- Slime-coloured film p10 is 80 or above on 13 of 14 planet-seeds; highlands 1 is 75.
- The review's film p10 fell on fen 2 (90 → 73), where the thin film lies over the darker mud.

## Second round: a hue per soil, and the film's absolute floor (art lead, 11-15 04:49Z)

The art lead found barrens grit and cave dust nearly the same picture, and plain soil too alike in hue across the set. The changes:

- **One colour source.** `plain_soil(e)` now gives plain soil its biome's hue everywhere it is drawn: the cell material, the ground bake and the pixel redraw. That keeps soil edges and the far view consistent.
- **The hues** (`biome_tint`). Each keeps the value at the middle elevation inside the 20-45 band:
  - barrens: a sun-baked tan, about 30 degrees;
  - caves: a cold blue-grey, about 220 degrees;
  - meadow: a touch of brown;
  - jungle: a dark red-brown humus;
  - fen: olive mud;
  - valley: a warm silt;
  - highlands: neutral stone.
- **Barrens texture.** Barrens now has a hardpan of large faint polygons about four cells across, with the wind streaks and fewer specks.
- **Caves texture.** Caves are smoother, with sparse pale grit and dark damp patches.
- **The art lead's test.** Barrens, caves and highlands are each nameable in a 420x240 crop at zoom 12 with no plants (the stills).
- **Meadow tint.** The meadow tint is milder than first set, (1.08, 1.0, 0.9) rather than (1.14, 0.99, 0.85). At 1.14 red, the bare ground in `the_slime_edge_is_as_sharp_as_its_walkers` rose from 39 to 44.5, and its 10-90% width read exactly 1.50 cells, the limit.
- **The film floor.** The thin film is now lifted with the slime's own light to at least 95 grey before shading (80 after the 0.85 shade floor) over any ground. It is gated as the gel lift is, so the edge test passes.

Measured with the reviewer's `measure.py`. Each figure is round one → round two, before the jungle tint was raised to keep its value:

- **Plain bare p50:** 33-47 on all 14 planet-seeds.
- **Barrens plain ground:** 42 → 45.
- **Jungle plain ground:** 42 → 39 and 39 → 37. Its tint was then brightened to (1.33, 0.88, 0.7) to keep the value.
- **Slime-coloured film p10:** 13 of 14 planet-seeds at 78 or above. Jungle went 85 → 79 and 82 → 78. Highlands 1 is 75.
- **The review's film p10 on fen:** stays at 73. The film's floor does not reach it, because those pixels are the faint fringe past the last walkers and the veins' cast shadow on the mud. That fringe is exactly what the edge-sharpness test keeps at ground value. Lifting it is the change that widened the edge to 1.74 cells (dead-ends 11-15). So a p10 at 80 everywhere and an edge as sharp as the walkers pull against each other; this cut keeps the edge.

## The film measure now counts film only (art lead, 11-15 05:12Z)

The art lead ruled to keep the edge: the owner's sharpness complaint outranks a p10 number, and the fringe past the last walkers is meant to be ground. The reviewer's `Reports/levelgen-review-2026-11-15/raw/aesthetics/measure.py` changes accordingly:

- **What counts as film.** `film_p10` now counts only new slime pixels that show the slime's own colour at 60 s (R-B > 45, the same test as its `slime1`). Those are the pixels with walkers under them.
- **What it leaves out.** The faint fringe past the last walkers and the veins' cast shadow on the ground, which by design keep the ground's colour.
- **The old count** is kept as `film_p10_all`.
- **Running it.** The run folder can now be set with `VO_R=DIR`, and the script finds its helper (`worldrect.py`) beside itself.

Do not chase `film_p10_all`: its low end is ground, not film.

Re-measured with the changed script. The before run is main 66dd92fe, before #317; the after run is this branch, which also carries #315, #317 and other merges.

| Planet, seed | film p10 (walked) before → after |
| --- | --- |
| Meadow 1 | 81 → 86 |
| Meadow 2 | 81 → 85 |
| Barrens 1 | 96 → 96 |
| Barrens 2 | 96 → 103 |
| Fen 1 | 73 → 82 |
| Fen 2 | 75 → 96 |
| Valley 1 | 74 → 85 |
| Valley 2 | 89 → 88 |
| Jungle 1 | 62 → 75 |
| Jungle 2 | 65 → 77 |
| Highlands 1 | 69 → 73 |
| Highlands 2 | 72 → 78 |
| Caves 1 | 68 → 87 |
| Caves 2 | 67 → 83 |

**Result.** 10 of 14 planet-seeds are at 80 or above, up from 4. Jungle (75, 77) and highlands (73, 78) are still under. Both are above their rock (41-56) and soft rock (44-66).

## Pictures

- `<biome>-z12-before.png` and `<biome>-z12-after.png`: the landing site of each planet, seed 1, at zoom 12 and 1 s. Before is round one's build with `PLAIN_AMP` at 0 (main's flat slate soil); after is round two.
- `highlands-, barrens-, fen-z12-*.gif`: 2-8 s at the same spots.
- `index.html` is the page.

## Cost

The cost is one to three value noises a pixel, applied only to plain ground at zoom 3 and above. The highlands' gravel also runs a 3 x 3 cell search, so highlands is the most expensive case.

Timed on llvmpipe with `map=planet:6:1 seed=1 bench=4 hand=idle screen=1280x800 centre=400,250,12` (highlands at zoom 12). Each figure is the GPU tail p50 in ms, three rounds run alternately:

| Build | Round 1 | Round 2 | Round 3 |
| --- | --- | --- | --- |
| Without the look | 114.3 | 110.6 | 108.3 |
| With the look | 112.3 | 109.5 | 109.6 |

The difference is within the noise. It has not been timed on the 1070.
