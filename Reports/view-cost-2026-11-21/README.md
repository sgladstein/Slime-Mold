# What the view's base pass costs, part by part (2026-11-21, open)

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

Timings go in the next GPU batch (Engine and tools); results and what is
cut follow here.
