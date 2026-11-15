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

## Pictures

- `<biome>-z12-before.png` and `<biome>-z12-after.png`: the landing site of each planet, seed 1, at zoom 12 and 1 s. Before is the same build with `PLAIN_AMP` at 0.
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
