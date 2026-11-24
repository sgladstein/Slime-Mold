# The Drifter's sheet gets an edge and a lit relief (2026-11-24)

Look lane, Milestone 2's Look B (the slime itself), the polish review's
first finding ("at zoom 6 the Drifter is a flat orange wash with a wide
glowing edge"). Page: https://claude.ai/artifact/DvALBJUXZsbaAYvndsmvra
(source `index.html` here, clips beside it).

**Before.** Planet 1's slime look (`SlimeLook` 2, the Drifter) drew its
trail as a thin gel at `SHEET_VEIL` x trail. The trail fades over three or
four cells, so the gel faded over about 20 px at zoom 6 and 40 px at zoom
12, and it had no height, so the key light never touched it: a halo round
the body.

**After** (`prep` in `src/world/view.wgsl`). The sheet is the trail where
it stands over `SHEET_EDGE` (a smooth step from 0.15 to 0.3, so its rim is
the trail's own contour, about a cell wide), and it rises `SHEET_RISE` 0.2
x its square, so the light shades its swells and rims it. Only the Drifter
changes; the cost is a smoothstep and two multiplies a cell in `prep`.

**Tried and dropped** (same run, same frames):
- a rise of 0.25 or 0.6 as a floor (`max`) over the body's height: one
  glossy slab that buried the body's ridges;
- a rise alone (0.1, 0.2 added) with no edge: lit, still a 40 px blur at
  zoom 12;
- a colour cut on the slime's thickness under 0.008: a speckled, ragged
  rim, since the thickness's tail is single walkers and the trail's is
  smooth.

**Ruling it bends:** "no thresholded tube edge". The edge is a smooth step
about a cell wide on the simulation's own trail, not a drawn line.

**Clips:** planet 1, run seed 1, `hand=idle`, crops 720 x 300 of
1280-wide frames, 20 frames at 120 ms, zoom 6 and 12; before and after
play the same game, only the view shader differs (`LARDER_VIEW_WGSL`).
