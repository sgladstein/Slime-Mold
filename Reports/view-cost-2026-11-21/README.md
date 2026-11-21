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
