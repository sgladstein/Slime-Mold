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

**Blind A/B.** The two videos run the same simulation, identical by `t` lines. The page is [`shot-damage-2026-11-12/`](shot-damage-2026-11-12/). The key: X was the candidate and Y was main.

**The owner's answer** (04:38Z): "Neither. Both of those look like stamps. Placed on the slime that fade. Maybe part of the issue is the slime/walkers move to fast." Then (04:39Z): "A group of soldiers shooting the slime should start carving out a bigger and bigger hole that doesn't refill while being shot at."

**What changed after that.**
- No wound is drawn at all. The engine is back to main: the `cut` pass still writes the wound byte, and nothing draws it.
- `body()` stays, so a hit shows as the real hole in the living body. `hole.mp4` is the same round with this change.

**How long a hole stays open, by walker clock.** Run with `larder-world look=DIR hand=rush at=16 holes=1` (fork map, seed 1, 5000 walkers, 0-16 s).
- Each hit's drawn body (trail × living walkers) is averaged over its strike disc, 5 fine cells across.
- A hole counts as refilled once that average is back to 80% of the tick before the hit.
- The clock was slowed with a scratch patch that ran fewer engine ticks a game tick. That patch is not committed. The Larder thread asked for a slower clock rather than a shorter stride, since the stride sets the network's shape.

| Fine steps a game second | Holes refilled | Still open at 16 s | Median to 80% | q25 | q75 |
|---|---|---|---|---|---|
| 30 (now) | 38 of 42 | 4 | 0.53 s | 0.13 s | 0.73 s |
| 15 | 34 of 42 | 8 | 0.47 s | 0.40 s | 0.93 s |
| 10 | 18 of 32 | 14 | 0.73 s | 0.53 s | 0.80 s |

Slowing the clock 3× barely lengthens a hole. Holes fill from the rim, by diffusion and by walkers stepping in. What the owner asked for (a hole that grows under sustained fire and doesn't refill) needs a rule: ground the mould avoids or can't enter after a hit. That rule is the Larder thread's, and the look will draw it from that rule's own field.

**On the GTX 1070** (1920 x 1080, `worldlook seconds=4`, view GPU ms, median of 21; six runs alternating main 19a8765 with 32d8cd8, the 25-read `body()`):

| Run | Build | Look z1 / z2 / z4 | Plain z1 |
|---|---|---|---|
| 1 | main | 1.336 / 0.914 / 0.764 | 0.311 |
| 2 | 32d8cd8 | 1.686 / 0.883 / 0.689 | 0.287 |
| 3 | main | 1.192 / 0.769 / 0.672 | 0.288 |
| 4 | 32d8cd8 | 1.683 / 0.883 / 0.689 | 0.287 |
| 5 | main | 1.303 / 0.883 / 0.764 | 0.307 |
| 6 | 32d8cd8 | 1.684 / 0.886 / 0.690 | 0.287 |

Matched fast runs (3 against 2, 4, 6): +0.49 ms at zoom 1, +0.11 at zoom 2, +0.02 at zoom 4. Answer: the prep pass loads walker counts once into a 20 x 20 workgroup tile (the 16 x 16 cells and a 2-cell rim), so each cell sums from the tile; the picture is unchanged (the same frame differs only in the HUD's alert text). On the 1070, nine runs cycling main 19a8765, the tile 9c33c5c and 32d8cd8 (same settings):

| Run | Build | Look z1 / z2 / z4 | Plain z1 |
|---|---|---|---|
| 1 | main | 1.190 / 0.767 / 0.669 | 0.287 |
| 2 | tile | 1.451 / 0.823 / 0.672 | 0.287 |
| 3 | 32d8cd8 | 2.425 / 0.993 / 0.783 | 0.301 (slow state) |
| 4 | main | 1.190 / 0.767 / 0.669 | 0.288 |
| 5 | tile | 1.452 / 0.823 / 0.672 | 0.287 |
| 6 | 32d8cd8 | 1.681 / 0.882 / 0.687 | 0.287 |
| 7 | main | 1.586 / 1.126 / 0.763 | 0.352 (slow state) |
| 8 | tile | 1.455 / 0.829 / 0.677 | 0.287 |
| 9 | 32d8cd8 | 1.683 / 0.883 / 0.689 | 0.287 |

In the steady runs the tile adds +0.26 ms at zoom 1 (1.19 -> 1.45), +0.06 at zoom 2 and nothing at zoom 4. The 25-read version added +0.49 ms at zoom 1. The tile ships.

*Possibly inflated (owner, 2026-11-12 15:28Z):* the PC was low on memory during today's runs. Every run above was a paired, alternating run on the same machine, so the differences between builds should hold better than the absolute ms. Re-timed on the 1070 at 15:49Z, with memory free. These were nine runs cycling base 19a8765, the tile 9c33c5c and main 9f5e4a4 (`worldlook seconds=4`, view GPU ms, median of 21). Two things differ from the runs above:
- They used the example's default screen, not 1920 x 1080, so the absolute times are about half of those above.
- Other GPU programs were running (a campaign and census binaries, 80-85% GPU use), so compare these runs only with each other.

| Build | Look z1 / z2 / z4 (runs 1, 2, 3) | Plain z1 |
|---|---|---|
| base | 0.579 / 0.401 / 0.383; 0.579 / 0.402 / 0.385; 0.579 / 0.402 / 0.385 | 0.163-0.164 |
| tile | 0.715 / 0.433 / 0.388; 0.715 / 0.432 / 0.387; 0.784 / 0.473 / 0.399 | 0.163-0.169 |
| main | 0.715 / 0.432 / 0.386; 0.717 / 0.433 / 0.388; 0.715 / 0.432 / 0.387 | 0.163-0.164 |

The tile costs +0.14 ms at zoom 1 (0.579 -> 0.715, 3 of 3 runs), +0.03 at zoom 2 and nothing at zoom 4. Main, which has the building sprites, matches the tile. In proportion this is the earlier result again: +23% at zoom 1 here, against +22% at 1080p above. So the low memory did not change the conclusion.

**Density** (tech lead review, 04:48Z): `BODY_N` was calibrated at 5000 walkers on 512 x 320. It now scales with the world's living walkers a cell, which the game sets once from the start (`View::set_body_density`; 0 keeps the calibration). The slot count would not do: a growing world has ten slots a walker. Hole times at higher counts (`holes=1`, rush, 0-16 s): 50,000 walkers at scale 2, 18 hits, median 0.20 s; 100,000 at scale 4, 25 hits, 3 refilled and 22 still open at 16 s (the strike killed only 20 walkers there: a scale-4 strike is a smaller share of a cell, worth a look by the Larder thread).

**Not measured yet.**
- The 1070 frame cost of `body()`: 25 look reads per shown cell in the prep pass.
- Whether a hit should last longer than 0.2 s. That is a rule change (trail refill, cut pieces, food flow) and belongs to the Larder thread, which plans to make cut-off pieces starve.
