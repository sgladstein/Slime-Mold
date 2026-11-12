---
title: The world camera and UI
kind: mechanism
status: current
updated: 2026-11-12
sources:
  - Reports/larder-world-roadmap-2026-11-11.md
  - Reports/shot-damage-2026-11-12.md
  - Reports/soldier-controls-2026-11-12.md
  - Reports/world-camera-2026-11-11.md
  - Reports/world-look-2026-11-11.md
  - Reports/world-ui-2026-11-11.md
code:
  - Camera::set_inset
  - Camera::wheel
  - CanvasPass
  - Cmd::Queue
  - Cmd::Rally
  - Controls::button
  - Engine::set_hits
  - Game::seek
  - Hud::aim
  - Hud::draw
  - HudEvent
  - MAX_HITS
  - MAX_MARKS
  - MAX_ZOOM
  - Marks::building
  - Marks::label
  - Marks::soldier
  - Order::Fight
  - SPRITES
  - Squad::model
  - UiCommand
  - View::record
  - View::set_alive
  - View::set_body_density
  - View::set_look
  - first_hit
---

# The world camera and UI

## Now

A world bigger than the screen is seen through `slime_mould::world`: a
camera that pans and zooms at the cursor (whole steps up to `MAX_ZOOM` 24, raised from 8 after the owner's playtest,
and below 1 down to the fit, each pixel the mean of its cells), a minimap
from the same frame, marks drawn on the GPU (up to `MAX_MARKS` 2,048), and
Larder's controls in world cells, given out as `UiCommand`s. On the GTX 1070
the view costs 0.32-0.42 ms a frame at 1920 x 1080, and 0.8-1.2 ms with the
look below.

In `larder-world` the soldiers take RTS orders
([report](../../Reports/soldier-controls-2026-11-12.md)): Shift queues any
order (`Cmd::Queue`), A then a click attack-moves (`Order::Fight`), a
harvester moves on from a dry crystal to a safe one (`Rules::seek`, on;
the census gate holds with it), and
the larder is selectable with a rally point (`Cmd::Rally`); a double click
selects every soldier on screen. A placed building is a ghost the selected
soldiers walk to and raise (`Order::Build`).

Over it sits a game UI with no game rules
([report](../../Reports/world-ui-2026-11-11.md)): a GPU canvas pass
(`CanvasPass`) and a HUD the game fills with a `Model` each frame, handing
back `HudEvent`s (top bar, alerts, groups, selection, a command card, a
placement ghost, hover cards, pause and end screens); about 0.1 ms of GPU
and of CPU a frame on the 1070. The game itself is `cargo run --release
--bin larder-world` (W2c,
[report](../../Reports/larder-world-roadmap-2026-11-11.md)): the Larder
rules on the GPU world through this camera, UI and look. The GPU sandbox's
Q squad (`world::demo::Squad`) is a stand-in with no game rules.

## The look

On by default ([world look](../../Reports/world-look-2026-11-11.md)): the
view shades the world from the engine's look field (mould thickness,
walkers, wounds) and a ground field (rock, walls by health, crystals by
charge): height, light, gloss, a speck a walker. Shots (`Engine::set_hits`)
stop at the first slime or rock on their line (`first_hit`) and cut there,
leaving a fading burn. The view draws no wound (the owner, 04:38Z, on a
blind A/B: "Neither. Both of those look like stamps"): the drawn
thickness counts the living walkers near each cell (`BODY_N`), so a hit
shows as the hole it really makes, open about half a second
([shot damage](../../Reports/shot-damage-2026-11-12.md)). With growth, `View::set_alive` takes the walkers' energy so
empty and dead slots draw no speck. Soldiers are an SVG sprite (the owner's
pick of a blind A/B, 16:48Z) with facing, team helmet and muzzle flash;
crystals are hexagons. Buildings (turret, lure, pump, outpost) are sprites in the same style (`Marks::building`): they rise as they are built, darken as they are hurt and light their lamp only while working ([world look, *Buildings*](../../Reports/world-look-2026-11-11.md)). Ground byte 3 holds elevation, left 0 until the
simulation uses height. One mark with a big box (a 480-pixel ring) cost the
view 0.7 ms, since one workgroup walks a mark's whole box: draw big outlines
as lines.

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
