---
title: The world camera and UI
kind: mechanism
status: current
updated: 2026-11-11
sources: [Reports/world-camera-2026-11-11.md]
code: [Camera::wheel, View::record, Controls::button, UiCommand, Marks::label, MAX_ZOOM, MAX_MARKS]
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
pixel-identical to the engine's own frame. Not yet timed on the GTX 1070 and
not yet played by the owner.

## Numbers

| Measure | Result | Seeds | Paired | Source |
|---|---|---|---|---|
| Through the camera at zoom 1 against the engine's frame, 1024 x 640 dish | 0 of 573,440 pixels differ | 1 | - | [report, *Checked*](../../Reports/world-camera-2026-11-11.md) |
| View pass, 1280 x 800, 49 marks, lavapipe (a CPU, not a frame cost) | 15.5-26.9 ms | 1 | - | [report, *Checked*](../../Reports/world-camera-2026-11-11.md) |

## Owner's word

- 2026-11-11 14:06Z: "We should do a camera UI thread too as you suggested".
- 2026-11-11 14:06Z: "A longer persistent world. Maybe a campaign. Crystal should run out which forces you to expand."

## Open

- The view's cost on the GTX 1070, and the owner's hands.
- Fog of war: the view has a visibility hook; the owner has not decided.

## Sources

- [World camera and UI](../../Reports/world-camera-2026-11-11.md).
