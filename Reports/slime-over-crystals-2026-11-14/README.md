# Slime grows over crystals (2026-11-14)

**Ask.** Owner, 11-14 08:15Z, via the art director: "slime shouldn't go
under crystals, they should grow over it but it needs to still clearly look
like a crystal." The exact hexagons (#230) were drawn over the slime, with
the mould mixed into the crystal's albedo at most 70% and lit by the faces
only, so the mould read as sliding underneath.

**Page:** https://claude.ai/artifact/WNkC1j2uVaZ5TMe6A9FH1c (copy: `index.html` here).

## What changed (`view.wgsl`, the crystal block of `shaded`)

- The slime on a crystal is a film over its faces. Its cover is
  `0.75 (1 - exp(-25 th))`, from the mould's real thickness there (the
  inverse of the cells' height curve). A thin film shows the faces through,
  and a feeding mat covers up to 75%.
- The film is the slime as shaded off the crystal: its own bumps, gloss and
  glint. It is tilted by the face it lies on (`0.3 + 0.7 x face light`),
  so the six faces still read.
- The charge's light comes up through the film, brightest deep in a full
  crystal. A mould draining the crystal (`hot`, the same charge loss as the
  harvest glow) takes a cyan cast.
- A dark line a pixel and a half wide inside the hexagon's edge keeps the
  silhouette crisp under the film.
- `cell_colour` on crystal cells: the engine draws a crystal cell as agar
  lerped to cyan by its share, then the mould over it. The view now gets the
  mould's own amount back from the red channel. Before, it took the colour
  of the nearest non-crystal cell within 6 cells, which never reached the
  middle of a crystal 30 cells wide, so the mould there kept the cyan and
  read pale.

## Pictures

Fork map, idle hand, the left crystal from 12 to 74 s as the mould reaches
it and covers it: before (main) and after at zoom 6, and after at zoom 12.

## Cost

A few ALU ops a crystal pixel and none elsewhere; not measurable on
llvmpipe.
