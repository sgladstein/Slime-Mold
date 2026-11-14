# Sharp slime and crisp crystals (2026-11-14)

The owner's playtest, 11-14 01:49Z: "Graphics need a big improvement. The
crystals are pixelated. The slime looks very blurry/out of focus." The art
director's brief (02:26Z): sharp is right, lumpy is wrong; a wet, glossy
sheet; the edge from the walker density, never a step; grain only at the
frontier. The crystals go on to the crystals thread from this PR.

**Page:** [Sharp Slime](https://claude.ai/artifact/5YWCariQ3diWQQKqG2PKNj)
(source in `page/`): before and now GIFs at zoom 1, 6, 12 and 24, same
simulation (every t line matches), and the grey state strip.

## Why it was blurry

The body's thickness was a cell value from the walkers on the 5 x 5 cells
round it (`crowd`, the look's byte 1), bilinear between cells, with its
slope sampled 1.5 cells apart: an edge spread over about five cells, 30
pixels at zoom 6 and 100 at zoom 24. Crystals stood in the cell field, so
their slanted edges were 6-pixel stairs at zoom 6.

## What the view does now (`src/world/view.wgsl`, `view.rs`)

- **Fine cells.** `prep` works on `View::fine(zoom)` cells a cell each way
  (1 below zoom 6, else 2). `draw_specks` counts each living walker in its
  fine cell (and, above bit 16, cut off from food, for #212's ash), into a
  buffer cleared each frame.
- **Two Gaussians.** Each fine cell sums the counts round it with a sharp
  Gaussian (0.6 cells; 0.8 at one fine cell a cell) and a wide one (1.1
  cells), as walkers a cell against `BODY_N`. Where the wide count says
  body, the wide value; towards the frontier the sharp one, blended
  smoothly (no contour). The trail and the engine's colour are Gaussians
  of the 4 x 4 cells round, so no cell squares show; the ground kinds are
  bilinear as before.
- **Height** is a cubic B-spline of the fine cells, its slope a forward
  difference 0.75 / fine cells; the shadow sample stays bilinear.
- **Crystals** are their exact hexagons (`View::set_crystals`: centre,
  circumradius, each cell's crystal): height and faces worked out, the edge
  antialiased to a pixel, charge still each cell's, mould over them in its
  own colour, no cyan rim (a crystal cell takes its mould colour from the
  nearest cell beside it).

## Tried and changed

- Same cells, a 0.8-cell Gaussian and a 0.75-cell slope: zoom 4-6 sharper,
  zoom 12-24 as soft as before. Kept as the one-fine-cell case.
- Four fine cells a cell, one 0.5-cell Gaussian on a 7 x 7 window: sharp but
  lumpy, and the truncated window made square blobs a cell wide. Two fine
  cells and a 9 x 9 window fixed the squares.
- Bilinear trail and height: square facets a cell wide in the shading at
  zoom 12-24 (the slope jumps at each border). Gaussian trail and colour,
  B-spline height.
- The B-spline's own slope (one evaluation): showed the walkers' one-a-cell
  lattice as a fine plaid in the dense body; the 0.75 / fine difference
  smooths it.

## Measured

- GPU test `the_slime_edge_is_as_sharp_as_its_walkers`: a straight body
  edge at zoom 12 rises 10%-90% over 0.96 cells, against 2.58 on main.
- GPU test `crystals_are_their_hexagons_to_the_pixel`: a third of a cell
  in from the hexagon's edge is crystal and a third out is not, at 40
  angles (fails on main's cell crystals).
- Frame cost, llvmpipe, `larder-world bench=20 hand=farm seed=1` (zoom 4),
  builds alternated: frame p50 31.9 and 31.7 ms on main, 37.5 and 37.5 ms
  now (+18%); GPU tail 23.9 against 28.7 and 29.6. The first fine version
  cost 44.2 ms; three B-spline samples instead of five and separable
  weights brought it down. The 1070 timing waits for the PC.
- Grey strip (fed 2:29; a held ring cut at 2:30, 27 of 232 walkers left
  at +30 s; the lean at +7.5 s with four soldiers): the starving piece
  reads as beads rather than a smear; the amber lean is still thin at zoom
  6 (the art director's rule 5 asks for a brighter, denser front: next).

Commands: clips `larder-world look=DIR hand=farm seed=1 map=fork
centre=420,232,ZOOM at=150,150.1,...,153` on llvmpipe; strip
`cut=150,365,240,32 centre=365,238,6 at=149,180` and with
`cutsquad=365,264,150,4 at=157.5`; `raw/mkgif.py` made the GIFs. Logs in
`raw/`.
