# Slime sheen: a flat frontier and a broad wet highlight (2026-11-14)

The graphics thread, from the art director's notes on #241 (11-14 08:15Z):
"(1) The thin frontier reads as pebbles or bubble wrap... Keep lone walkers
bright... but as points of light, not raised bumps. They should carry
little or no height and no shadow until there's real thickness. (2) The
dense body's specular is so high-frequency that it reads as crumpled gold
foil at zoom 6. Lower the frequency of the highlight, broad wet sheens
along the veins rather than glitter, and drop saturation slightly where
it's thickest." Also the crystals thread's ask (after #247's darker ground)
that a thin film stay above the ground in grey.

Page: https://claude.ai/artifact/2gtoXBwm5tGsg5pA5at7rW (`page/`).

## What changed (`src/world/view.wgsl`)

- **The frontier is flat.** The height takes its thickness through
  `smoothstep(0.3, 0.6, wide)` (`prep`'s `th_h`): a walker alone (wide
  about 0.4) has almost no height, so no shaded flank and no cast shadow;
  its speck is the point of light. Height comes in as walkers gather and
  is full in the body. The colour's thickness (`tm`) is unchanged, so the
  lone walker stays bright.
- **A broad sheen.** The specular takes its own normal from a bilinear
  height sampled `9 / zoom` cells each side (0.6 to 1.5 cells), not the
  shading's fine slope, with a lower exponent (14 + 30 gloss, was tighter).
  The highlight follows the veins' shape, not the walkers' grain. A fixed
  1.5-cell step was tried first and made zoom 12 soft.
- **Calmer thick colour.** The saturation push eases off by 30% where the
  body is thickest (`smoothstep(0.6, 1.0, th)`), so the soldiers and the
  bullet light keep the value order.
- **Gel film.** The thin film adds 6% of the hue's own light over the
  tinted soil, so it stays above #247's darker ground.

## Measures

Films: `larder-world look=DIR hand=farm seed=1 map=fork centre=420,232,Z`,
frames every 0.1 s from 2:30 (`raw/films.sh`), main (68870d04) against
this change; same simulation, so the walker counts match.

| Zoom 6 / 12 at 2:31, slime pixels | main | now |
|---|---|---|
| Mean pixel Laplacian (fine grain), zoom 6 | 8.7 | 6.7 |
| Mean pixel Laplacian, zoom 12 | 3.9 | 3.6 |
| Pixels in highlight (luminance > 215), zoom 6 | 2,815 | 3,393 |
| Pixels in highlight, zoom 12 | 5,137 | 10,801 |
| Median saturation, zoom 6 | 202 | 196 |

Less fine grain, more highlight area: broader sheens, not more glitter.

Test `a_lone_walker_is_bright_and_the_body_stays_calm` now also checks the
darkest pixel within 1.5 cells of a lone walker: 60.5 against bare ground
42.1 now; on main the dome's shaded flank came down to 44.6 against 40.3.

Cost (llvmpipe, `bench=20 hand=farm seed=1`, 1280 x 800, builds
alternated): frame p50 main 59.2 and 64.6 ms, now 58.4 and 60.6 ms, within
the machine's noise. 1070 timing waits for the PC.
