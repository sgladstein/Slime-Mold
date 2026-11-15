# What the view costs, and making it cheaper (2026-11-15)

Performance thread, after the owner asked for one (11-15 00:21Z: "And a
performance improvement thread"). The 1070 A/B of 11-14 (main 2b1a851c
against 004b7cb, Reports/pc-batch-1070-2026-11-14.md) showed the day's
ground work adding about 2 ms of GPU a frame at every zoom. Looks were not
to be traded: every change here keeps the pixels.

## The tool

- `larder-world ... bench=S viewpasses=N`: after the timed run, N more
  frames with each of the view's passes stamped alone
  (`View::record_each`), a line a pass (base, prep, compose, marks,
  specks, drain).
- `LARDER_VIEW_WGSL=PATH` loads another copy of `view.wgsl`, so a part of
  the shading can be left out without a rebuild.
- `scripts/view-cost.ps1` (the 1070): one build, then every variant in
  `knockouts.json` (an exact text swap each) at five views, interleaved
  rounds; `-Compare SHA` times another commit's own exe beside it and
  `-Only` picks variants.

## Where the time went (the 1070, main's shader)

`view-cost.ps1` at 113d37e8, 2 rounds, 120 frames each; raw:
`runner-out/view-cost-113d37e8-20261114-183656/` on branch
`claude/project-thread-26drse` @ 6b1821c2. The screen pass (`base`) and
the per-cell pass (`prep`), ms; a part's cost is base with it minus base
without it:

| View | base | prep | plants | plant reads alone | crowns alone | slope | soil | terrain | sheen |
| --- | --- | --- | --- | --- | --- | --- | --- | --- | --- |
| fork, zoom 1 | 2.41 | 2.61 | 0.75 | 0.77 | 0.25 | 0.23 | 0.08 | 0.74 | 0.08 |
| fork, zoom 6 | 2.37 | 0.54 | 1.27 | 0.90 | 0.77 | 0.27 | 0.12 | 0.22 | 0.11 |
| fork, zoom 24 | 1.89 | 0.07 | 1.13 | 0.83 | 0.68 | 0.25 | 0.02 | -0.01 | 0.10 |
| planet 8, zoom 6 | 2.53 | 0.54 | 1.20 | 0.80 | 0.69 | 0.27 | 0.32 | 0.33 | 0.11 |
| planet 8, zoom 24 | 2.11 | 0.07 | 0.92 | 0.59 | 0.46 | 0.23 | 0.17 | 0.33 | 0.09 |

Every other part (crystals, the cast shadow, lights, alarm, specks) is
under 0.15 ms. Leaving out all of `shaded` takes base to 0.13-0.77 ms. The
fork is covered in meadow, so its plants run everywhere. llvmpipe's split
differs (crowns were 40% of base there, the reads 15%), which is why the
1070 decided.

## What changed

1. **Rock-covered pixels skip the ground.** `base` works out the hard
   terrain first (`terrain_at`); where it covers the pixel whole, the
   pixel is the terrain's top (`terrain_top`) and `shaded` never runs. The
   result was always the top alone there (`mix` by a cover of 1). The dark
   round a map smaller than the screen is rock, so at zoom 1 this is most
   of the screen.
2. **`prep` skips the map's far outside.** Workgroups wholly beyond
   `OFF_MAP` (8 cells) of the map write only their terrain cells, which the
   rock's top reads. At zoom 1 on the fork three quarters of `prep` was off
   the map.
3. **`prep`'s Gaussians run separably**: each tile row summed across once
   in workgroup memory (`rows5`), then nine rows down; 81 reads a fine cell
   were 33 of 59 ms of `prep` on llvmpipe at zoom 1.
4. **Crown outlines without `atan2`.** `lobe_wave` raises the unit vector
   to the lobes' power as a complex number instead of `sin(n atan2)`.

Same pixels: headless looks (`look=`) at zoom 1, 6 and 24 on the fork and
planet 8, and planet 3 at zoom 1, match main exactly for 1-3, and to 1/255
on at most 0.03% of pixels with 4 (llvmpipe), apart from one 25-pixel mark
near the larder that differs between two runs of main itself.

## The 1070 A/B

18d6e1b8 (1-4 plus the packing below) against 113d37e8 (main's shader),
all view passes, 3 rounds; raw: `runner-out/view-cost-18d6e1b8-vs-113d37e8-20261114-185735/`
@ 426cc4b5. Medians of the three rounds, ms:

| View | main | 18d6e1b8 | with 4 |
| --- | --- | --- | --- |
| fork, zoom 1 | 5.12 | 1.90 | 1.91 |
| fork, zoom 6 | 3.01 | 2.92 | 2.89 |
| fork, zoom 24 | 2.04 | 2.23 | 2.20 |
| planet 8, zoom 6 | 3.22 | 3.01 | 2.98 |
| planet 8, zoom 24 | 2.86 | 2.86 | 2.84 |

The final build's A/B (2011eb65: 1-4, packing reverted) is below once the
1070 has run it.

## Tried and dropped

- **Plant fields packed by `prep`** (eight adjacent words a cell for the
  pixels' B-spline, instead of five words a cell from the engine's blocks):
  the knockouts put the plant reads at 0.6-0.9 ms, but packed, base was
  0.1-0.2 ms *slower* at zoom 6 and 24. Neighbouring pixels share the
  engine's words in cache; the packed words were more of them.
- **Pruning crowns before their hashes** (a crown whose jitter box is out
  of reach skipped): no gain on llvmpipe; a warp's lanes run in step, so a
  skip pays only when every lane skips.
