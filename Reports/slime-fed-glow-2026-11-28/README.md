# The slime lit from its food (2026-11-28)

Look B, "the slime up close" (the owner's pick on the lane's card, 11-28
07:12Z: of the four looks, the slime up close bothered him most; the 11-24
polish review's first finding: soft at zoom 6, flat inside a big body).

Page: `index.html`, published at
https://claude.ai/artifact/MLcuaeos9xFsk7JzxBjtqx (listed in
`Reports/pages.md`).

## What the game knows inside a big body

Filmed on planet 1 (run seed 1, idle hand, 60 s), lavapipe, 1536 x 864:

- Every cell of a packed body holds a walker, so the walker count, which
  sets the body's height, is flat; its only relief is the fine crinkle of
  the counts' noise.
- The trail saturates across the body (the scent view's pink is even).
- The food scent lies in a ring about two cells wide round each crystal.
- Each walker's energy does vary: it flows through the body from the
  crystals it covers (`energy_flow`, `egain`), so the mean energy is a hill
  over each crystal falling to near 0 at the rim (`energy.jpg`, a debug
  shader that drew each walker blue to yellow by energy / 2).

## What changed (`view.wgsl`)

- `draw_specks` adds each ordinary walker's energy (64ths, bits 8-31) and a
  count (bits 0-7) to the fine grid's third block, which only the Sleeper's
  crust used (`fed_on`: lives on, not the crust look).
- `prep` reads that block on its tile after the rows pass (the column sums
  moved above the edge test so every thread reaches the new barriers) and
  takes the mean under the body's Gaussian: `fed`, energy 0 to 2 as 0 to 1.
- In the body (wide count 0.3 to 0.6): the height rises `FED_RISE` 0.35 x
  fed, the material glows `FED_GLOW` 0.25 x its own colour x fed, gains
  `FED_GLOSS` 0.15 gloss x fed, and a hungry cell's colour falls to
  `FED_DIM` 0.8.
- The sheen's slope is sampled 6 screen pixels apart (was 9) with exponent
  22 + 46 x gloss (was 14 + 30).

Tried and not kept: a smoothstep from 0.3 to 1.8 with a rise of 0.5 and a
glow of 0.12: the top saturated into a flat plateau with a ledge round it,
and the glow barely showed at zoom 6.

## Cost

llvmpipe, planet 1, 1536 x 864, `viewpasses=20`, medians: prep 9.20 ms
before, 9.48 after; base 28.3 and 28.8 (a repeat of the before run gave
7.26 and 25.8, so the difference is inside the noise).

## Not fixed

Veins and streaming inside the body: the walkers there are packed and
cannot stream (`Reports/dead-ends.md`, the Trunk's veins, 11-19). That
would be a change to the slime's movement.
