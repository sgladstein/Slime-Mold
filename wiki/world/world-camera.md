---
title: The world camera and UI
kind: mechanism
status: current
updated: 2026-11-15
sources:
  - Reports/body-saturation-2026-11-15/README.md
  - Reports/building-looks-2026-11-14/README.md
  - Reports/crisp-terrain-2026-11-14/README.md
  - Reports/crystal-harvest-2026-11-14/README.md
  - Reports/gun-card-hud-2026-11-14/README.md
  - Reports/larder-world-roadmap-2026-11-11.md
  - Reports/levelgen-2026-11-14/README.md
  - Reports/panic-look-2026-11-14/README.md
  - Reports/sharp-slime-2026-11-14/README.md
  - Reports/shot-damage-2026-11-12.md
  - Reports/siren-alarm-2026-11-14/README.md
  - Reports/slime-material-2026-11-14/README.md
  - Reports/slime-over-crystals-2026-11-14/README.md
  - Reports/slime-sheen-2026-11-14/README.md
  - Reports/soldier-controls-2026-11-12.md
  - Reports/spent-crystals-2026-11-14/README.md
  - Reports/start-menu-2026-11-14/README.md
  - Reports/turret-look-2026-11-14/README.md
  - Reports/ui-review-2026-11-15/README.md
  - Reports/ui-scale-2026-11-15/README.md
  - Reports/ui-size-2026-11-14/README.md
  - Reports/units-read-2026-11-14/README.md
  - Reports/units-read-2026-11-14/followup.md
  - Reports/units-read-2026-11-14/harvest.md
  - Reports/view-cost-2026-11-15/README.md
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
  - Game::elbow_room
  - Game::last_takes
  - Game::overrun_share
  - Game::seek
  - Gather
  - Hud::aim
  - Hud::draw
  - HudEvent
  - Loadout
  - MAX_HITS
  - MAX_MARKS
  - MAX_ZOOM
  - Marks::building
  - Marks::building_facing
  - Marks::building_pulse
  - Marks::health
  - Marks::icon
  - Marks::label
  - Marks::selected
  - Marks::soldier
  - Marks::soldier_bitten
  - Marks::soldier_working
  - Marks::turret
  - NEXT_ROUND
  - Order::Fight
  - SPRITES
  - Squad::model
  - StartMenu
  - UiCommand
  - UiSize::px
  - View::fine
  - View::record
  - View::record_each
  - View::set_alarm
  - View::set_alive
  - View::set_body_density
  - View::set_crystals
  - View::set_look
  - View::set_news
  - first_hit
  - mark_far
  - ring_groups
---

# The world camera and UI

## Now

A world bigger than the screen is seen through `slime_mould::world`: a
camera that pans and zooms at the cursor (whole steps up to `MAX_ZOOM` 24, raised from 8 after the owner's playtest,
and below 1 down to the fit, each pixel the mean of its cells), a minimap
from the same frame, marks drawn on the GPU (up to `MAX_MARKS` 2,048), and
Larder's controls in world cells, given out as `UiCommand`s. On the GTX 1070
the view costs 0.32-0.42 ms a frame at 1920 x 1080, and 0.8-1.2 ms with the
look below. With today's ground, plants and fire the 1070's view is
1.8-2.9 ms at zoom 1-24 (2026-11-15; 5.2 ms at zoom 1 before); plants are the largest part, and
`bench=S viewpasses=N` with `scripts/view-cost.ps1` times each pass and each
part of the shading ([view cost](../../Reports/view-cost-2026-11-15/README.md)).

In `larder-world` the soldiers take RTS orders
([report](../../Reports/soldier-controls-2026-11-12.md)): Shift queues any
order (`Cmd::Queue`), A then a click attack-moves (`Order::Fight`), a
harvester moves on from a dry crystal to a safe one (`Rules::seek`, on;
the census gate holds with it), and
the larder is selectable with a rally point (`Cmd::Rally`); a double click
selects every soldier on screen. A placed building is a ghost the selected
soldiers walk to and raise (`Order::Build`), anywhere on the map: with
nobody selected the nearest free soldier walks out (`Cmd::Send`), and the
ghost stays faint with a bar and the seconds left until it stands
(owner, 2026-11-12 22:06Z; part 4 of the report).

Over it sits a game UI with no game rules
([report](../../Reports/world-ui-2026-11-11.md)): a GPU canvas pass
(`CanvasPass`) and a HUD the game fills with a `Model` each frame, handing
back `HudEvent`s (top bar, alerts, groups, selection, a command card, a
placement ghost, hover cards, pause and end screens); about 0.1 ms of GPU
and of CPU a frame on the 1070. F1 or ? opens the key card from play (it
pauses; again, or Esc, goes back to play). A start menu (`StartMenu`,
`world::start`, 2026-11-14) draws on the same canvas down the left of the
screen, over a live dish: sandbox, the campaign's missions, an options
page with each switch's default beside it, and the key card; the
window opens on it when run with no arguments, and turning its map
switches the dish behind it
([report](../../Reports/start-menu-2026-11-14/README.md)). A one-row
loadout strip (`ui::Loadout`, 2026-11-14) on the selection panel's top
edge shows the squad's gun cards in their four slots, framed by kind, with
the cast the shown soldier's next shot fires lit, a tooltip
per card and a swap button (H, `NEXT_ROUND`); the same change cut the
bottom panels from 20% of the screen to 15% (160 of 1080 pixels, 120 of
800), collapses the selection panel when nothing is selected, and opens
the camera in the band above them ([report](../../Reports/gun-card-hud-2026-11-14/README.md)). The whole UI, the map editor's panel included, lays out in `UiSize::px` pixels a unit (2026-11-14): the height's share of 1080 lines as before, with the window's DPI scale as a floor and the player's size (75% to 300%, Ctrl + / Ctrl -, the Options page, `ui=`) over it, capped so the bottom bar stays under a third of the screen, and saved in `settings.txt`; the same day the owner asked for a wider, shorter bottom bar, so the gun cards are a panel at its far left, the command card is 6 x 2 and the squad one row filling the middle, 113 of 1080 pixels (10.5%), the minimap standing at its old height in the corner ([report](../../Reports/ui-size-2026-11-14/README.md)); since 11-14 20:19Z (owner: the bar "changes totally when you zoom in") the minimap shows at every zoom, in the row at the bar's height, and the bar's panels draw at 92% (104 pixels at 100%) so the layout never changes with zoom ([report](../../Reports/steady-hud-2026-11-14/README.md)). On 11-15 (owner: "the text seems too small ... not always making the best use of space") the bar went back to the full unit (113 pixels), the command buttons became 116 x 42 with their names beside the icons (square again when the row is too short, as at 150% on 1080 lines), squad tiles show the squad's state, and the menus, key list and run shop got 13 to 15 unit text, landscape bullet cards in the shop and no minimap over full-screen menus ([report](../../Reports/ui-review-2026-11-15/README.md)). Later that day (owner: "everything was supposed to be on one row") the gun strip stays in the bar's row at every UI size and draws smaller rather than moving above the bar, and the hover card over the world waits for the cursor to rest, shows briefly, fades and comes back only for something else, so it does not cover what is being shot ([report](../../Reports/ui-review-2026-11-15/README.md#follow-up-11-15-0800z-one-row-and-the-hover-card)). The start menu's pages, the run shop and the gun range's armory draw at the UI size too (they had been held to the window's height, so Ctrl + changed little there) and scroll with the wheel or the arrow keys when taller than the window (`StartMenu::wheel`, `Canvas::clip`); the in-game key card takes up to three columns ([report](../../Reports/ui-scale-2026-11-15/README.md)). The game itself is `cargo run --release
--bin larder-world` (W2c,
[report](../../Reports/larder-world-roadmap-2026-11-11.md)): the Larder
rules on the GPU world through this camera, UI and look. The GPU sandbox's
Q squad (`world::demo::Squad`) is a stand-in with no game rules.

## The look

On by default ([world look](../../Reports/world-look-2026-11-11.md)): the
view shades the world from the engine's look field (mould thickness,
walkers, wounds) and a ground field (rock, walls by health, crystals by
charge): height, light, gloss, a speck a walker. Its once-a-second refresh
reads only the crystal and wall cells (`Gather`, 2026-11-14; [levelgen
report](../../Reports/levelgen-2026-11-14/README.md), *A sparse ground refresh*),
and a wall going up rebuilds only the 32-cell tiles it touches, their `FAR`
marks redone over each tile and its reach (`look::mark_far_box`; *Walking
maps at 6x*). A command's read-backs copy only the words they read
(`gpu::read_back_range`): the gas press read the engine's whole `news`
buffer twice, 115 ms at 6x on software Vulkan, now 9 ms (*The 1070 at 6x,
and the gas press*). Shots (`Engine::set_hits`)
stop at the first slime or rock on their line (`first_hit`) and cut there,
leaving a fading burn. The view draws no wound (the owner, 04:38Z, on a
blind A/B: "Neither. Both of those look like stamps"): the drawn
thickness counts the living walkers near each cell (`BODY_N`), so a hit
shows as the hole it really makes, open about half a second
([shot damage](../../Reports/shot-damage-2026-11-12.md)). The count does not saturate in dense slime (11-15: under 0.3% of body pixels past near 0.95, even at 6:00 with 64k mass), because `n0` scales with the world's living density; what hides a plain hit is the body's step-to-step churn ([body saturation](../../Reports/body-saturation-2026-11-15/README.md)). With growth, `View::set_alive` takes the walkers' energy so
empty and dead slots draw no speck. With withering on, `View::set_news` reads each walker's news of food (2026-11-14, plan C8): a cut-off walker's speck is amber while it leans and red once it bolts, larger with the zoom, and the mould fades to ash by the share of nearby walkers cut off, so a starving piece greys before it thins ([starving mould](../../Reports/panic-look-2026-11-14/README.md)). Since 2026-11-14 (the owner's playtest: "the slime looks very blurry", "the crystals are pixelated") the body is worked out below the cell: walkers are counted where they are on fine cells (`View::fine`, two a cell from zoom 6), with a sharp Gaussian at the frontier and a wide one in the body, a B-spline height, and Gaussian trail and colour, so a straight edge at zoom 12 rises over about a cell (2.58 before); crystals are their exact hexagons (`View::set_crystals`), antialiased to a pixel; llvmpipe frame +18% ([sharp slime](../../Reports/sharp-slime-2026-11-14/README.md)). After the owner's playtest of it ("the middle still looks blurry", "single individual walkers should be brighter") every speck grows with the zoom and a calm walker's light falls with the square of the walkers a cell round it, so a lone explorer is a bright point and the body a calm grain; the body keeps 40% of the sharp count; the slope is capped at 0.7; the slime is vivid wet gold with tight glints over a gel film; grey, amber and red ramp with each walker's news age; llvmpipe frame +4% ([slime material](../../Reports/slime-material-2026-11-14/README.md)). After the art director's notes the frontier is flat (height comes in with the wide count, so a lone walker is a point of light with no flank or shadow), the highlight takes its normal from a height sampled 9 / zoom cells apart (a broad sheen along the veins, not glitter), the thickest body is a little less saturated, and a walker on a crater or scorch line, where the engine's colour holds no mould, is drawn in the mould's gold rather than black; the body is opaque from a third of full thickness, so ground (the fork's clay disc) shows only through the thin film ([slime sheen](../../Reports/slime-sheen-2026-11-14/README.md)). Soldiers are an SVG sprite (the owner's
pick of a blind A/B, 16:48Z) with facing, team helmet and muzzle flash;
crystals are hexagons. Rock, walls and rubble are drawn at the screen pixel from a smooth field of their cells (`look::mark_far` skips cells far from any), with a one-pixel antialiased edge and a short soft shadow at every zoom ([crisp terrain](../../Reports/crisp-terrain-2026-11-14/README.md)). Crystals read the engine's charge per cell every frame (`View::set_charge`, `View::set_time`): each stands and glows by its charge, greys and sinks from the side being taken, and cells losing charge glow white-cyan, held about 0.35 game seconds; a dry crystal is worn dark stone in three steps ([crystal harvest](../../Reports/crystal-harvest-2026-11-14/README.md), [spent crystals](../../Reports/spent-crystals-2026-11-14/README.md)). Mould on a crystal grows over it as a film whose cover follows its thickness, lit by the faces, the charge glowing through (owner 11-14 08:15Z; [slime over crystals](../../Reports/slime-over-crystals-2026-11-14/README.md)). Since 11-14 every soldier can be counted at zoom 6
(elbow room `Rules::elbow`, outlined figures, `Marks::selected` with one
ring round an overlapping group (`ring_groups`), hurt-only
`Marks::health`, a dead pose), and buildings and the larder show their
state in the sprite (`Marks::building`, `Game::overrun_share`; a ghost's
builder state is a sign, `Marks::icon`). Harvesting shows on the soldier:
a taker (`Game::last_takes`) faces the crystal with a pulse at its hands, its
pack fills cyan as it carries (`Marks::soldier_working`; no carry bar), and the
larder flashes as a load is banked (`Marks::building_pulse`). A turret's head
turns on its base to its aim and kicks with a muzzle flash as it fires, its
lamp the game's state, and hovering one shows its reach (`Marks::turret`;
[turret look](../../Reports/turret-look-2026-11-14/README.md)). The other
buildings show their work too, all soldier-scale: a burning lure lights the
engine's real scent field green (a scent light, `Light` with lift below 0), a
pump turns its hose into its crystal with a charge line as bright as its draw
(`Marks::building_facing`, `Building::drew`), an outpost is a mini larder that
flares when a load is banked there, and a siren's real call shows as the
engine's alarm field itself, a pale front spreading and fading
(`View::set_alarm`, bindings 25-26;
[siren alarm](../../Reports/siren-alarm-2026-11-14/README.md)). A soldier the slime bites flinches every 3 hp of damage, the
bitten side flushing hot (`Marks::soldier_bitten`;
[building looks](../../Reports/building-looks-2026-11-14/README.md))
([units read](../../Reports/units-read-2026-11-14/README.md), [follow-up](../../Reports/units-read-2026-11-14/followup.md), [harvest](../../Reports/units-read-2026-11-14/harvest.md); [world look, *Buildings*](../../Reports/world-look-2026-11-11.md)). Ground byte 3 holds elevation, left 0 until the
simulation uses height. One mark with a big box (a 480-pixel ring) cost the
view 0.7 ms, since one workgroup walks a mark's whole box: draw big outlines
as lines.

## Owner's word

- 2026-11-11 16:48Z: "Solider A looks better" (the SVG sprite); "Gun fire should hit the first slime in its path"; "The crystals should have hexagonal outline, not circle."
- 2026-11-11 17:37Z: "from looking at the gif/video it doesn't always look like that" (shots stopping at the first slime): the stand-in counted only thick tubes; now anything that looks like slime (26/255) stops a shot.
- 2026-11-11 14:06Z: "We should do a camera UI thread too as you suggested".
- 2026-11-11 20:16Z: "We need to know about solider health, numbers, ways to control them, eventually we will have different weapons, upgrades, build walls (and probably other things in the future), menus, enemy info"; "make it look modern and professional."
- 2026-11-14 01:49Z: "I also have no way to see what the gun cards are in the interface."
- 2026-11-14 08:07Z: "We need an option to scale the UI/text etc, because they look too small on my screen."
- 2026-11-14 08:33Z: "re-arrange the bottom menu to be one row for the squad and two rows for the other buttons so the whole thing is wider but shorter"; 08:46Z: "gun/card menu should go on the far left to fill the rest of the screen."
- 2026-11-11 14:06Z: "A longer persistent world. Maybe a campaign. Crystal should run out which forces you to expand."

## Open

- The owner's hands.
- From the UI review: command pages when weapons and buildings outgrow the
  4 x 3 card, selectable buildings, a production queue, Shift-queued moves,
  minimap unit blips, a settings menu in play (the UI size is on the start
  menu's Options and Ctrl + / Ctrl -).
- Fog of war: the view has a visibility hook; the owner has not decided.

## Sources

- [World camera and UI](../../Reports/world-camera-2026-11-11.md).
- [The game UI over the Larder world](../../Reports/world-ui-2026-11-11.md).
- [The loadout panel](../../Reports/gun-card-hud-2026-11-14/README.md).
- [UI and text size](../../Reports/ui-size-2026-11-14/README.md).
- [Menus, shop and HUD review](../../Reports/ui-review-2026-11-15/README.md).
