# Plants, drawn close (2026-11-14)

The owner, 19:26Z: plants "are mostly just green blobs right now. They need to look better." 20:21Z: "the fire is blurry. The assumption should be that I am playing zoomed in and the graphics quality should match that". The graphics lead's notes, in order: plants muted dark blue-green, under the slime's thin film in value, a raised darker canopy and a fine grass texture, no gold, drawn from the density field with no stamps. Then at 21:26Z, on fire v3: light the fire per crown, make the advancing edge the brightest line, and draw char as crisp near-black grey with scattered ember points.

Page: https://claude.ai/artifact/7ZFRbDuC4ahCxKVV6wDui5 (`page/`, built by `raw/films.sh` and `raw/make_page.py`; `raw/film.log` is the run's output).

## What changed (src/world/view.wgsl, src/gpu/engine.wgsl)

- **Per pixel.** `plant_px` takes one cubic B-spline over the 4 x 4 cells round each pixel and gets each kind's density, the fire's heat, its freshness and the char from it. Before, the plants were mixed into each cell's material through a 3 x 3 Gaussian and blended between cells, which made soft blobs with no texture.
- **Crowns sized by the field.** `crowns` puts each kind on a jittered grid. Each crown's radius is a share of the spacing times the square root of the kind's density at that pixel, so the ground a kind covers follows its density. A dome lights each crown, and five to eight lobes ruffle its outline. The kinds are drawn lowest first: meadow, then scrub, reeds and jungle.

  | Kind | Spacing (cells) | Radius (share of spacing) | Notes |
  |---|---|---|---|
  | Meadow | 0.45 | 0.8 | lobe 0.2, relief 0.35 |
  | Scrub | 1.1 | 0.5 | |
  | Reeds | 0.4 | 0.6 | 2.4 times longer along y |
  | Jungle | 1.7 | 0.85 | relief 0.9 |

  Rejected:
  - Round domes read as cobbles.
  - Seven deep lobes on every tuft read as stars.

  Below 1.6-3.2 pixels per feature, a kind fades to its mean cover and colour.
- **Colours.** Green at hue about 130-150, at the same values as the first cut: jungle (20,42,28), reeds (34,56,40), meadow (40,60,40), scrub (54,54,42). Before the per-pixel look, meadow was the olive (68,76,48). The first per-pixel cut was jungle (18,42,40), reeds (32,56,54), meadow (36,60,50) and scrub (50,54,44). The graphics lead (21:56Z) found that teal sat at hue ~175, next to the crystals' cyan, and took the crystals' pop on the thumbnails.
- **Under the slime.** The cell material buffer `mcell` grew from 2 to 4 words a fine cell. The third word is `vis`, how much ground shows through the slime (1 - 0.95 opaque, and water's own share over WET). That doubles the buffer, to 16 bytes a fine cell.
- **Fire per crown.** A burning crown's flame takes the bilinear heat and freshness at its centre, so each crown flares and dies on its own. Freshness is engine block 8 (`PB_FRESH`): it is 1 when a cell catches and is multiplied by 0.85 each fire step while it burns. A fresh crown burns 0.3 hotter, toward yellow. The heat itself flickers from step to step, so a rise in heat marks noise, not the front. The game does not read freshness, and the CPU keeps no copy.
- **Embers.** Where the plants are gone and heat remains, ember grains appear on a 0.6-cell grid, sized by the heat. They shrink away as it fades.
- **Spill.** The spill starts at heat 0.04 and is 0.4 strong (was 0 and 0.55), so dying embers no longer make a brown blur.
- **Zoomed out**, the smooth flame stands in.
- **Char.** Near-black grey (25,24,23) with a little ash grain, rising over char 0.06-0.3 per pixel, laid over what is left of the plants (stubble). Before, it was brown (40,35,30) and cell-blended.

## Measured

- **llvmpipe frame**, planet 2 seed 1, two bombs at 1 s, zoom 12, 960x600, `bench=8`: p50 66.1 ms, p95 85.2 ms. With the fire ramp alone (342f9ced) it was p50 56.6 ms and p95 73.2 ms.
- **Zoom 24** over the jungle edge: p50 60.0 ms. The GTX 1070 has not been timed.
- **Film numbers are unchanged by the look:** slime 25,000 → 28,655 at 45 s, 4 of 4 soldiers alive.
- **Tests:** the full suite with `MOULDGPU_TEST_GPU=1` passes (447 + 29 + 39).
