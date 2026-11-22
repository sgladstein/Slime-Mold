# What the view's base pass costs, part by part (2026-11-21)

Look and interface. The 1070 batch for Milestone 1
([pc-batch-1070-m1-2026-11-21](../pc-batch-1070-m1-2026-11-21.md)) timed the
view's `base` pass at 3.2 to 3.5 ms of a 5 ms frame (planet 3, the larder,
1280 x 800, zoom 6 and 12), up from 0.1 to 0.6 ms for the whole draw on
11-14. It did not say which part of `base` costs that.

`variants.py` writes copies of `src/world/view.wgsl`, each with one part of
`base` taken out: soil grain, plants, set slime's crust, water, crystals,
rock and walls over the ground, point lights, and all of `shaded`'s lighting
(`flat`); `full` is the shader as it is. Each is swapped in with
`LARDER_VIEW_WGSL` (no rebuild) and timed with the game's view timer, so a
variant's drop in `base` against `full` is roughly that part's cost. The
pictures are wrong on purpose; they are for timing only. Every variant
compiles (checked on llvmpipe, 11-21).

The run, for each variant (the batch's own view command):

    python3 Reports/view-cost-2026-11-21/variants.py vc
    LARDER_VIEW_WGSL=vc/NAME.wgsl larder-world run=1:3 mode=squads bench=60 \
        hand=strat:rush,aim=6 centre=larder,6 viewpasses=30

## The timings (GTX 1070, 11-21)

Engine and tools ran the nine variants on Scott's GTX 1070 at main
c241fd7e (planet 3, `centre=larder,6`, `bench=60`, `viewpasses=30`), each
twice and interleaved; raw output on branch `runner-out-1070-followup`,
`runner-out/20261120-221333/view-*.txt`. `base` in ms, run 1 / run 2, and
what taking the part out saves against `full` (means):

| variant | base (ms) | saves |
|---|---|---|
| full | 3.391 / 3.449 | |
| plants | 2.368 / 2.389 | 1.04 |
| soil | 2.910 / 2.954 | 0.49 |
| crust | 3.143 / 3.173 | 0.26 |
| gems | 3.171 / 3.181 | 0.24 |
| water | 3.198 / 3.201 | 0.22 |
| terrain | 3.203 / 3.203 | 0.22 |
| lights | 3.417 / 3.419 | 0.00 (no lights in the shot) |
| flat | 0.772 / 0.772 | 2.65 (all of `shaded`'s lighting) |

The view as a whole took 3.93 / 4.00 ms, the frame 4.97 / 5.08 (p50).
`full-1` began on an idle card (569 MHz) and still matched `full-2` within
2%. The parts sum to about 2.5 ms; nearly all of `base` is `shaded`, and the
plants are its dearest single part.

## What was cut

**The plants, under the slime** (11-21). `shaded` worked out every kind's
crowns (3 x 3 neighbours each, with a slow noise and the moisture field
for the clumped kinds) at every pixel, and then weighted them by how much
bare ground shows (`gv`), which is 0 under the body. Now the crowns and the
water's thinning are worked out only where `gv` is over 0.001 or the plants
burn (their flames light the slime over them). The picture is the same:
filmed on planet 3 at 60 s, zoom 6, before and after, the moss and reeds
on bare ground unchanged. On the 1070 (Engine, 11-21, main 92a0e43e,
the same shot, two runs) `base` is 3.431 and 3.457 ms against 3.391 and
3.449 before: no saving in this shot, where at 60 s by the larder little
of the slime lies over plants. The plants variant's 1.04 ms was all plant
work, nearly all of it on bare ground, which the cut keeps. A cheaper
plant layer on bare ground is where that millisecond is.

Next if more is needed: the slope's two extra B-spline samples and the
highlight's four `film_h` reads in `shaded`, which every pixel pays.

## The second pass (11-21, Direction: the 1070 needs 60 frames a second on the core world)

Two cuts that leave every pixel as it was (planets 1 to 3 at 3 s and 40 s,
zoom 12 and the whole map, run 1, `hand=strat:bot`: the world's pixels
identical to main's; only the alert text's fade differs by its timing):

- **The slope's samples share one read.** `shaded`'s height and its forward
  slope were three B-splines, 48 reads of `hcell`; `cells_height3` reads the
  5 x 5 fine cells they share once, 25 reads, and sums them in the same order.
- **Far crowns skip their clump.** A clumped plant kind (the meadow's
  grass, the fen's moss, the barrens' scrub) worked out its slow noise and
  read the moisture field for all nine neighbouring crowns before it knew
  whether one reached the pixel. A crown that can neither touch the pixel
  nor shade it is now passed over first (bounds in `crowns`).

On llvmpipe (`centre=larder,6`, planet 3, `bench=10`, three runs a side)
`base` was 134.5 / 132.1 / 143.7 ms on main and 130.4 / 132.8 / 142.2 with
both cuts: inside the software GPU's 10% noise. Nor did the plants' own
parts separate there (`onekind` 153, `nolobe` 143, `noclump` 128 against
`full` about 135; `crowns`, the crown layer stubbed, 76; `plants` 74;
`flat` 43): on llvmpipe the crowns cost about 57 of 135 ms, but none of
their parts alone shows past the noise. The 1070 has to say.

Tried and not kept: the highlight from three height samples instead of
five (forward differences off the film's height at the pixel, `spec3`):
it moves each highlight half a sample (up to 4 px at zoom 12), seen on a
wanderer at zoom 12, for 8 of the pixel's roughly 100 buffer reads.

New variants for the next 1070 batch (`variants.py`): `crowns`, `onekind`,
`nolobe`, `noclump` (the plants split), `nocull` and `slope3` (this pass's
two cuts undone, so each is timed against `full`), and `spec3`.

## The third batch (GTX 1070, 11-22)

Engine and tools ran the new variants on Scott's 1070 at main 911a119e,
each twice (`run=1:3 mode=squads bench=60 hand=strat:rush,aim=6
centre=larder,6 viewpasses=30`; every run ended in the same state, t 60
and 34,924 walkers; twins within 0.05 ms). This base is planet 3 at zoom 6
with 35k slime, so it is not comparable to the 3.4 ms above. Raw files on
his PC, `C:\Users\Scott\Code\pc-batch-out\20261122-030143\`.

| variant | base (ms) | saves | frame p50 |
|---|---|---|---|
| full | 5.638 / 5.687 | | 7.09 / 7.18 |
| nocull | 5.586 / 5.598 | 0.07 | |
| slope3 | 4.392 / 4.399 | 1.27 | 5.88 |
| crowns | 3.696 / 3.697 | 1.97 | 5.24 |
| onekind | 4.657 / 4.656 | 1.01 | 6.14 |
| nolobe | 5.586 / 5.591 | 0.07 | |
| noclump | 5.415 / 5.413 | 0.25 | |
| spec3 | 5.601 / 5.599 | 0.06 | |
| plants | 2.906 / 2.908 | 2.76 | 4.47 |
| flat | 0.990 / 0.990 | 4.67 | 2.59 |

**Reverted: the slope's shared read.** `slope3`, the 11-21 pass's
`cells_height3` undone (three plain B-splines again), was 1.27 ms *faster*:
the shared read indexed its 25-word array by a run-time offset, which most
likely put the array in local memory instead of registers. It is reverted
(the same pixels on planets 1 to 3 at zoom 12 on llvmpipe, `hand=idle`).
The far crowns' cull (`nocull` 0.07) and the three-sample highlight
(`spec3` 0.06) are inside the twins' spread: the cull stays, `spec3` stays
out.

**Where the rest is.** The crown layer alone is about 2 ms (`crowns`), and
three of its four kinds about 1 ms (`onekind`): the 3 x 3 neighbours per
kind per pixel, not their lobes (0.07) or clump noise (0.25).
