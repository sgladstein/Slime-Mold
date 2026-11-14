# Dark, cool open ground (2026-11-14)

**Ask.** The art director's phase 2 ([style guide](../../wiki/looks/art-direction.md),
rules 2 and 3): open ground is the darkest thing on screen, cool and low in
saturation, so the gold slime and the cyan crystals are the only colour.
The old olive-brown ground's middle read as sand beside the slime, and the
sand soil was brighter than a thin film of slime.

**Page:** https://claude.ai/artifact/XhF3786aox9FaRUbSYAiT1 (copy: `index.html` here, with its pictures).

## What changed

Only the bare-ground colours in `view.wgsl` (the mould's lines are the
slime thread's, agreed):

| Ground | Before (RGB) | Luma | After (RGB) | Luma |
|---|---|---|---|---|
| Plain, flat land | 57, 54, 43 | 53 | 35, 38, 42 | 38 |
| Plain, low to high | 30,34,28 to 84,74,58 | 32 to 75 | 18,22,27 to 52,55,58 | 21 to 55 |
| Sand | 150, 134, 92 | 133 | 70, 68, 60 | 67 |
| Clay | 104, 54, 38 | 67 | 56, 38, 34 | 43 |
| Barren | 78, 76, 72 | 76 | 50, 50, 50 | 50 |
| Wet | 22, 46, 62 | 41 | 14, 30, 42 | 27 |

Rock (70-118 grey) and walls are now brighter than every ground, which is
the style guide's value order: ground, then rock and walls, then thin slime.
The three soils still read apart by value and a slight cast (passes map:
clay, plain and sand bands). The engine's tints (scent, species, grass)
still lie over the soil, so the ground's texture comes only from the fields.

## Pictures

All five maps at the opening of a rush round before and after, and zoom 6
on the passes map from 20 to 32 s. `passes-after-grey.jpg` is the grey
check: the ground is the darkest band, the slime and crystals the lightest.

## Cost

None: constants only.
