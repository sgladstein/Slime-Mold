---
title: The world camera and UI
kind: mechanism
status: current
updated: 2026-11-11
sources: [Reports/world-camera-2026-11-11.md, Reports/world-look-2026-11-11.md]
code: [Camera::wheel, View::record, Controls::button, UiCommand, Marks::label, MAX_ZOOM, MAX_MARKS, View::set_look, Engine::set_hits, Marks::soldier, MAX_HITS, first_hit, SPRITES]
---

# The world camera and UI

## Now

A world bigger than the screen is seen through `slime_mould::world`: a
camera that pans (middle drag, the screen edge, the arrows) and zooms at the
cursor (whole steps from 1 to `MAX_ZOOM` 8, and below 1 down to the fit,
where each pixel is the mean of the cells under it). There is a minimap
drawn from the same frame, marks drawn on the GPU (up to `MAX_MARKS`
2,048), and Larder's controls in world cells, given out as `UiCommand`s.
It has no game rules. The GPU sandbox uses it: W gives a 2048 x 1280 world,
Q a stand-in squad, M the minimap. Today's dish at zoom 1 comes out
pixel-identical to the engine's own frame. On the GTX 1070 the view costs
0.32-0.42 ms a frame at 1920 x 1080. Not yet played by the owner.

**The look** (on by default, L in the sandbox; [world look](../../Reports/world-look-2026-11-11.md)):
the view shades the world from the engine's look field (mould thickness,
walkers, wounds) and a ground field (rock, walls by health, crystals by
charge): height, light, gloss, a speck a walker. Shots (`Engine::set_hits`)
stop at the first slime or rock on their line (`first_hit`) and cut there,
leaving a fading burn. Soldiers are an SVG sprite (the owner's pick of a
blind A/B, 16:48Z) with facing, team helmet and muzzle flash; crystals are
hexagons, as in the simulation. About 2.2x the plain view's time on the
software GPU; the 1070 cost is not yet measured.

## Numbers

| Measure | Result | Seeds | Paired | Source |
|---|---|---|---|---|
| Through the camera at zoom 1 against the engine's frame, 1024 x 640 dish | 0 of 573,440 pixels differ | 1 | - | [report, *Checked*](../../Reports/world-camera-2026-11-11.md) |
| View pass, 1920 x 1080, 49 marks, GTX 1070, zoom 1 / 4 / fit 0.84 | 0.333 / 0.315 / 0.415 ms | 1 | - | [report, *On the GTX 1070*](../../Reports/world-camera-2026-11-11.md) |

## Owner's word

- 2026-11-11 16:48Z: "Solider A looks better" (the SVG sprite); "Gun fire should hit the first slime in its path"; "The crystals should have hexagonal outline, not circle."

- 2026-11-11 14:06Z: "We should do a camera UI thread too as you suggested".
- 2026-11-11 14:06Z: "A longer persistent world. Maybe a campaign. Crystal should run out which forces you to expand."

## Open

- The owner's hands.
- Fog of war: the view has a visibility hook; the owner has not decided.

## Sources

- [World camera and UI](../../Reports/world-camera-2026-11-11.md).
