# Rivers under reeds (2026-11-15)

The whole-game review (11-15, https://claude.ai/artifact/UMU3R7fPu166PQ96n6gGpU) found the rivers vanish under reeds within two minutes. The coordinator passed it on as an agreed item: "Keep a blue base on deep cells."

Page: https://claude.ai/artifact/EAvniuwXaf5D6Nh2AgGsd9 (`page/`; frames by `raw/films.sh`, pictures and numbers by `raw/make_page.py`, output in `raw/measure.txt`).

## What happened

Planet 4 seed 1 (river valley), with the squad idle.

- At 2 s, reeds stand on the banks.
- By 120 s they have spread over the shallows. The plant rule lets any kind grow on wet ground, and reeds grow there at full capacity.
- Zoomed in, the reeds' crowns closed over the shallow water.
- At zoom 4 and out, where the crowns fade to their flat mean colour, the shallows turned a green-teal wash. The river read as a dark green strip with a few dark pools.
- No plant grows on deep water itself (soil capacity 0). But the plants' cubic spline reaches a cell and a half, so it bled over the narrow deep channels.

## What changed (src/world/view.wgsl, `shaded`; look only)

- **Thinner over water.** Over water the plants are drawn with their density times `1 - WATER_THIN x wet`, where `WATER_THIN` is 0.65 and `wet` is a bilinear wet share from `water_share`, 4 ground reads. The crowns shrink, so blue water shows between the stems.
- **Nothing over deep water.** Plants are drawn at zero density where the deep share passes 0.6. They fade out from a deep share of 0.25.
- **The far view too.** Where the crowns fade to their flat cover, that cover is veiled by `1 - WATER_VEIL x wet x far`, with `WATER_VEIL` 0.6. `far` is the share of the cover not drawn as crowns. With the thinning alone, the shallows still read green at zoom 4.
- **The simulation is unchanged.** Reeds still grow over the shallows, wet the ground and burn.

## Measured

The deciding measure is the share of blue pixels (B > G + 4) in a 300 x 440 window round the river at 305,150. The window also takes in banks and some slime crossing, so it never reaches 1.

| | 2 s | 120 s before | 120 s after |
|---|---|---|---|
| zoom 12 | 0.558 | 0.355 | 0.457 |
| zoom 4 | 0.177 | 0.074 | 0.165 |

Some of what is left of the 120 s drop at zoom 12 is the slime crossing the river by then.

The llvmpipe view pass `base` (`bench=8 viewpasses=20`, river at zoom 12, 960 x 600) went from 74.10 to 74.65 ms. That is +0.5 ms, within noise.

Rejected: the thinning alone. At 120 s it got zoom 12 to 0.457, but zoom 4 only from 0.074 to 0.093, still green.
