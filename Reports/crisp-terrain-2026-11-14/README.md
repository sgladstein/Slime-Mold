# Crisp rock, wall and rubble edges (2026-11-14)

**Ask.** The owner's playtest (11-14 01:49Z: "The crystals are pixelated"),
then the art director's review ([art review](../art-review-2026-11-14/README.md),
issue 2: cell staircases on rock, walls and crystals): hard shapes need exact
edges at every zoom, 1 to 24, antialiased over one pixel, with a short soft
shadow ([style guide](../../wiki/looks/art-direction.md), rule 4). This
report is the rock, walls and rubble; the crystals' hexagons come from the
slime thread's PR, and this thread then makes them show their charge.

**Page:** https://claude.ai/artifact/DFSpBUyJ65uDB1nDAn2R5N (copy: `index.html`
here, with its GIFs).

## What was built

`view.wgsl`, *Hard terrain*:

- `prep` works out each shown cell's terrain word (`tcell`, binding 23):
  its kind, height (rock 3, a wall 0.6 + 2.6 x health, rubble low), value,
  and its occupancy smoothed 1 2 1 over the 3 x 3 cells round it.
- `terrain_over`, after the ground's shading in `base`, takes a cubic
  B-spline of those smoothed occupancies over the 4 x 4 cells round the
  pixel, and the edge is where it crosses 0.4; the coverage is the signed
  distance to the edge (the field over its gradient) in pixels, so every
  zoom gets a one-pixel antialiased edge. The top bevels down over 0.9 of a
  cell, lit by the world's light; the ground 1.2 cells away from the light
  is darkened by up to 45% by the terrain's height, soft over about a cell.
- Rock, walls and rubble no longer raise `hcell` or colour `mcell`: their
  cells shade as soil underneath. The old slope between a cell 3 tall and
  the ground was the stepped bevel and the dark band; the old cast shadow
  sampled heights three cells toward the light.
- Off the world is drawn as rock by the same code (it was a bevelled frame).
- `look::mark_far` sets ground bit 7 (`look::FAR`) on cells with no hard
  terrain within 4 cells, and `terrain_over` returns at once there. The soil
  kind keeps bits 4-6. Larder World and the sandbox mark their ground; a
  caller that doesn't still draws right, only slower.

### What did not work

- A quadratic B-spline (3 x 3) alone: a scalloped edge on diagonals.
- The cubic alone: a notch every two cells along Fork's ridges (the maps'
  rock is drawn on the coarse grid, so a diagonal is a staircase of 2-cell
  steps). Smoothing the field over a 1.5-cell box at the pixel (four
  splines) fixed the look but cost 7 ms a frame more on llvmpipe; the same
  smoothing done once a cell in `prep` looks the same.
- A bevel tilted by the terrain's height (0.6 + 0.4 h): with the light high
  (z 0.7), steep faces read as a dark rim on every side, the band again.
  Now 0.35 + 0.15 h.

## The test

Look: `larder-world look=DIR hand=rush at=2,2.25,...,8 screen=960x600
centre=175,145,Z` (Z = 6, 12, 24; `centre=256,160,1` for the whole map),
seed 1 on Fork, before (main at 7d09bb0) and after; GIFs on the page.

Cost: `larder-world bench=8 hand=idle screen=1280x800` on llvmpipe, GPU tail
p50 (p95), ms a frame:

| Build | p50 | p95 |
|---|---|---|
| main | 37.1-38.0 | 44.4-47.6 |
| every pixel its own field, smoothed at the pixel | 54.6 | 66.7 |
| without the pixel smoothing | 47.5 | 57.3 |
| shipped (smoothing in `prep`, `FAR` skip) | 47.0-49.0 | 58.2-60.1 |
| shipped with `terrain_over` returning at once | 39.1 | 49.2 |

So the terrain costs about 10 ms of llvmpipe's 47 at the opening zoom,
where ridges and the map's rim fill much of the screen; `prep`'s share is
about 1 ms. Not timed on the GTX 1070 (the PC is busy, 11-14 00:28Z).

Tests: `far_marks_only_cells_with_no_terrain_in_reach` (look.rs); the
whole suite and clippy pass on llvmpipe (`MOULDGPU_TEST_GPU=1`).

## Not done

- The open ground (dark, cool, textured only from real fields): next.
- The crystals: after the slime thread's PR lands its hexagons.
- 1070 timing.
