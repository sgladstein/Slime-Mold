# Crisp ground edges at close zoom (2026-11-14)

**Ask.** Owner, 11-14 20:21Z: "when I zoom in on the world, the edges between
different grounds is pixalated. ... The assumption should be that I am
playing zoomed in and the graphics quality should match that." The
coordinator gave the ground boundaries to this thread (fire blur went to the
plants thread).

**Page:** https://claude.ai/artifact/HdYmS22KYVsHm1Ci1NMWg4 (copy: `index.html` here).

## What changed (`view.wgsl`, plus a bigger `hcell` in `view.rs`)

- **Soils crisp at the pixel.** Each soil's smooth share (the cubic
  B-spline over 4 x 4 cells) is sharpened by a softmax whose sharpness grows
  with the zoom, so a boundary is where two shares are equal, about a pixel
  wide at any zoom. Textures follow the crisp shares.
- **Plants crisp at the pixel** in the first cut (density spline, crisp
  cover at 0.2, kinds by softmax). Dropped at the merge with #304, whose
  per-pixel crowns draw plants and char over the ground.
- **Exact cancellation of the material's blend.** `soil_detail` adds
  `want - est`. `est` is the bare ground as `mat_at` sees it, which at
  `fine` 2 is a blend of blends. The `prep` pass now stores it a fine cell
  (third grid of `hcell`, r g b packed in an f32 under 2^24) and reads it
  bilinear. A plain bilinear over 4 cells left a cell-wide scallop on every
  waterline; working out the exact 3 x 3 weights a pixel cost +38 ms.
- **Plant height smoothed 3 x 3** (as their colour already was). Read cell by
  cell, a row of reeds along a bank stood as a string of beads with lit and
  shaded flanks: the dark stepped waterline at zoom 12 was this, not the
  colour (found by shading flat albedo: dropping land height or the wet dip
  changed nothing, dropping plant height removed it).
- **Film thickness from the mould's own height.** `shaded` read the slime
  film's thickness back from the whole cell height, so jungle (1.2 high) and
  high land counted as thick slime and switched the ground detail off (bare
  ~0.05 on every plant patch). The mould's own height is now the second grid
  of `hcell`; the crystal film reads it too. The highlight's normal also
  comes from it, so plant beds and land slopes no longer catch the water's
  gloss.
- **Ash grains round** (two value noises on turned grids; one on the cell
  axes thresholded into squares). **Clay plates one shade each** (hashed on
  the plate, not on squares a seventh of a plate).
- Soil textures fade to a quarter under full plant cover.

## Cost

llvmpipe, `map=river seed=1 bench=8 hand=idle screen=1280x800`, GPU tail
p50, alternating rounds against main 5168e103.

| Build | ms |
| --- | --- |
| Main | 95.8-99.6 |
| First cut (exact weights at the pixel, 9 `ground_cell`s) | 133.5-136.2 (+38) |
| `est` stored by `prep` | 112.5-117.6 |
| Without the plant spline (to measure it) | 96.0-104.6 |
| Final | 115.7-117.9 (median +18) |

Most of what is left is reading the plants' density over 16 cells a pixel.

**After merging main's plant crowns (#304) and the art lead's cost note
(22:15Z).** The plants thread now draws plants and char at the pixel, so
this change drops its own plant spline, crisp plant cover and kinds. Where
the four cells round a pixel are one ground, the 4 x 4 spline is skipped.
llvmpipe River, one round each against main 2b1a851c (which has #304):

| Zoom | Main | This |
| --- | --- | --- |
| 1 | 351.9 | 337.6 |
| 6 | 206.8 | 192.1 |
| 12 | 173.5 | 167.9 |

The art lead's 1070 run of main with #304 (not #305): about +1.9 ms at
every zoom over the morning's build, frame p95 4.3-6.6 ms; the +25 ms
llvmpipe cap is lifted.

## Left as it is

- Fen and Jungle's reed and jungle beds follow the map's own stair (1-cell
  steps every ~7 cells from the level generator): drawn now as a smooth
  outline with rounded corners, but the steps are in the data.
- A faint light rim along some reed beds over deep water is the engine's
  own colour (`tint`) on those cells, not a drawn effect.

## 2026-11-15: the spline skip's seams (bug check)

The skip tested the 2 x 2 cells round a pixel, but the spline reads 4 x 4,
and the bank, shallow margin, bridge shadow and depth read its shares as a
distance, so they stepped at cell middles near every shore and bridge. The
skip now needs all 16 cells to be one ground:
[bug check](../bug-check-2026-11-15/README.md#cost-of-fix-3).
