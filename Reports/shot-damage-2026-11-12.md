# Shot damage on the slime (2026-11-12)

**Why.** The owner playtested Larder World (2026-11-12 04:13Z) and wrote: "shooting the slime didn't feel satisfying. I know it was killing lots of individual walkers but visually it didn't impact much."

**What happens when a shot lands.** I ran a headless rush (`larder-world look=DIR hand=rush`, fork map, seed 1, 5000 walkers, scale 2) from 8 s to 16 s of game time.
- The mould: 4970 walkers alive at 8 s, 4360 at 16 s.
- The shots: 11 fired by 8 s, 42 by 16 s. They had killed 30 walkers by 8 s and 1456 by 16 s.
- So the shots killed 1426 walkers in those 8 s, but the mould shrank by only 610 (regrowth). A strike kills about 35 walkers in a disc 5 fine cells across.

The picture showed none of it, for three reasons:
1. **The trail saturates and refills.** The look's thickness was the trail's brightness. `bright` is 1 - exp(-v/knee), so it stays near 1 with fewer walkers. With the wound overlay switched off, a struck disc showed no hole at any frame sampled every 0.2 s.
2. **The hole is brief.** Once the thickness also counts living walkers (below), frames taken one game tick (1/15 s) apart show a dark hole for about 3 ticks, about 0.2 s, before walkers flow back in.
3. **The wound looked like a sticker.** The cut pass wrote the thickness it cut over the whole disc. The view then mixed that in after the lighting at up to 92%, which drew a flat, round, brown sticker of one size on an unchanged sheet.

**The candidate change** (look only, with no rule change):
- `view.wgsl` `body()`: the drawn thickness is the trail's brightness times 1 - exp(-n / `BODY_N`), where n is the walkers on the 5 x 5 cells around the cell (the look's byte 1) and `BODY_N` is 6.
  - Measured on the same run: over cells with trail of 0.5 or more, n had a median of 14, a 10th percentile of 4 and a 90th of 23.
  - A culled patch is therefore thin while it is empty, and the whole body thins as walkers die.
- `engine.wgsl` `strike`: each walker a strike kills adds `WOUND_DEATH` (112 of 255) to the wound of the cell it died on. The cut pass no longer writes the wound. Tried on the way:
  - 160: every struck cell saturated, so the mark was still a flat disc.
  - 64: too faint to see.
- `view.wgsl` `shaded`: the wound goes into the albedo before the lighting.
  - It chars the surface (ash, matte, up to 80%), so the relief of mould flowing back over a hit shows through.
  - A fresh, dense one adds an ember glow (wd³).

**Blind A/B.** The two videos run the same simulation, identical by `t` lines. The page is [`shot-damage-2026-11-12/`](shot-damage-2026-11-12/) and the key is off the page. The owner's pick will be recorded here.

**Not measured yet.**
- The 1070 frame cost of `body()`: 25 look reads per shown cell in the prep pass.
- Whether a hit should last longer than 0.2 s. That is a rule change (trail refill, cut pieces, food flow) and belongs to the Larder thread, which plans to make cut-off pieces starve.
