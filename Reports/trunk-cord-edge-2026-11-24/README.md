# The Trunk's cords get an edge (2026-11-24)

Look lane, Look B, the Trunk half of polish review finding 1 ("planet 3's
Trunk trail is a flat translucent yellow sheet"). Page:
https://claude.ai/artifact/LPEJkAfmgJyrNtMRPuKwtB (source `index.html`
here, clips beside it).

**Before.** `SlimeLook::Cords` drew the trail off the body at
`CORD_VEIL` x trail and raised it `CORD_RISE` x trail². The Trunk keeps
its trail 0.95 a step, so the trail fanned off the body as wide blurred
plumes, and each cord wore a halo about 20 px wide at zoom 6 (40 px at
zoom 12).

**After** (`prep` in `src/world/view.wgsl`). The cord is the trail times
a smooth step from 0.15 to 0.3 (`CORD_EDGE`, the Drifter's `SHEET_EDGE`),
in both the colour and the rise, so its rim is the trail's contour, about
a cell wide. The tail past it shows on the fen's mud as the slick track
(`SLICK_*`) the trail already leaves there. Only the Trunk changes; the
cost is one smooth step a fine cell in `prep`.

**Not done:** the review's darker core; the rim and the rise read as a
tube already.

Filmed on llvmpipe: planet 3, seed 1, idle hand, centres 378,121 and
218,186 at zoom 6 and 378,121 at zoom 12, 20 frames from 1:00.
