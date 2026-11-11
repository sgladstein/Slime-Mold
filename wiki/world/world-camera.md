---
title: The world camera and UI
kind: mechanism
status: current
updated: 2026-11-11
sources: [Reports/world-camera-2026-11-11.md, Reports/world-ui-2026-11-11.md]
code: [Camera::wheel, Camera::set_inset, View::record, Controls::button, UiCommand, Marks::label, MAX_ZOOM, MAX_MARKS, Hud::draw, Hud::aim, HudEvent, CanvasPass, Squad::model]
---

# The world camera and UI

## Now

A world bigger than the screen is seen through `slime_mould::world`: a
camera that pans (middle drag, the screen edge, the arrows) and zooms at the
cursor (whole steps from 1 to `MAX_ZOOM` 8, and below 1 down to the fit,
where each pixel is the mean of the cells under it). There is a minimap
drawn from the same frame, marks drawn on the GPU (up to `MAX_MARKS`
2,048), and Larder's controls in world cells, given out as `UiCommand`s.
On the GTX 1070 the view costs 0.32-0.42 ms a frame at 1920 x 1080.

Over it sits a game UI ([report](../../Reports/world-ui-2026-11-11.md)): a
GPU canvas pass (`CanvasPass`: glass panels, Inter text, line icons) and a
HUD with no game rules. The game fills a `Model` each frame and handles the
`HudEvent`s that `Hud::draw`'s buttons, hotkeys and placements give back:
top bar, alerts, groups, a selection panel with health, a command card with
costs and cooldowns, a placement ghost that refuses red spots, enemy hover
cards, a pause menu and an end screen. The camera's inset lets the world
scroll out from under the panels. In play it costs about 0.1 ms of GPU and
0.1 ms of CPU a frame on the 1070. A single mark with a big box (a 480-pixel
ring) cost the view 0.7 ms, since one workgroup walks a mark's whole box;
draw big outlines as lines. The GPU sandbox shows it all: W gives a 2048 x
1280 world, Q a stand-in squad (`world::demo::Squad`) that drives every
panel. Not yet played by the owner.

## Owner's word

- 2026-11-11 14:06Z: "We should do a camera UI thread too as you suggested".
- 2026-11-11 20:16Z: "We need to know about solider health, numbers, ways to control them, eventually we will have different weapons, upgrades, build walls (and probably other things in the future), menus, enemy info"; "make it look modern and professional."
- 2026-11-11 14:06Z: "A longer persistent world. Maybe a campaign. Crystal should run out which forces you to expand."

## Open

- The owner's hands.
- From the UI review: command pages when weapons and buildings outgrow the
  4 x 3 card, selectable buildings, a production queue, Shift-queued moves,
  minimap unit blips, a settings menu.
- Fog of war: the view has a visibility hook; the owner has not decided.

## Sources

- [World camera and UI](../../Reports/world-camera-2026-11-11.md).
- [The game UI over the Larder world](../../Reports/world-ui-2026-11-11.md).
