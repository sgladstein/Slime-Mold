---
title: The world camera and UI
kind: mechanism
status: current
updated: 2026-11-12T00:30Z
sources: [Reports/world-camera-2026-11-11.md, Reports/world-ui-2026-11-11.md, Reports/world-look-2026-11-11.md, Reports/larder-world-roadmap-2026-11-11.md]
code: [Camera::wheel, Camera::set_inset, View::record, Controls::button, UiCommand, Marks::label, MAX_ZOOM, MAX_MARKS, Hud::draw, Hud::aim, HudEvent, CanvasPass, Squad::model, View::set_look, Engine::set_hits, Marks::soldier, MAX_HITS, first_hit, SPRITES, View::set_alive]
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
1280 world, Q a stand-in squad (`world::demo::Squad`, no game rules) that
drives every panel. The game itself is `cargo run --release --bin
larder-world` (W2c, [report](../../Reports/larder-world-roadmap-2026-11-11.md)):
the Larder rules on the GPU world, through the same camera, UI and look.

**The look** (on by default, L in the sandbox; [world look](../../Reports/world-look-2026-11-11.md)):
the view shades the world from the engine's look field (mould thickness,
walkers, wounds) and a ground field (rock, walls by health, crystals by
charge): height, light, gloss, a speck a walker. Shots (`Engine::set_hits`)
stop at the first slime or rock on their line (`first_hit`) and cut there,
leaving a fading burn; the game's own shots (the squad's `cut` pass) leave
the same wound. With growth, `View::set_alive` takes the walkers' energy so
empty and dead slots draw no speck. Soldiers are an SVG sprite (the owner's pick of a
blind A/B, 16:48Z) with facing, team helmet and muzzle flash; crystals are
hexagons, as in the simulation. Bare ground is soil with the engine's
tints, rock and wet ground have their own look, and ground byte 3 holds
elevation (left 0 until the simulation uses height). A `prep` pass works
each shown cell's height once a frame: on the 1070 at 1080p the view takes
0.8-1.2 ms with the look against 0.3 ms without.

## Owner's word

- 2026-11-11 16:48Z: "Solider A looks better" (the SVG sprite); "Gun fire should hit the first slime in its path"; "The crystals should have hexagonal outline, not circle."
- 2026-11-11 17:37Z: "from looking at the gif/video it doesn't always look like that" (shots stopping at the first slime): the stand-in counted only thick tubes; now anything that looks like slime (26/255) stops a shot.
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
