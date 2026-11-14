# Art review of Larder World, 2026-11-14

The owner's playtest (11-14 01:49-01:50Z): "Graphics need a big improvement.
The crystals are pixelated. The slime looks very blurry/out of focus. The
flashes of chaos bullets are really bad. [...] I also have no way to see what
the gun cards are in the interface." and "The bullets shouldn't just be flashy
effects. Research how noita does their wand animations." The owner then asked
(02:24Z) for one thread to review all graphics, manage the graphics work and
start new work where it sees issues: "We need to polish this game up".

This report is that thread's first review. The standing art direction it
sets is the wiki page [art direction](../../wiki/looks/art-direction.md).

## How it was looked at

Main at 7df2182 (#213), the window's own picture made headless on llvmpipe
(`larder-world map=fork look=DIR`, 1280 x 800, seed 1, shape C): the whole
map at 1:10 under the rush hand, the near view at zoom 6 and 12 during the
`volley` hand with `rounds=shrapnel` (45 frames at 15 a second), the gas
loadout, and the start menu. Pictures and the film are in `page/`, beside the
page they appear on (`page/index.html`, https://claude.ai/artifact/6FHfyB88A8Mtk5cQkkuVDm). No 1070 timing (the PC is busy).

| file | what |
|---|---|
| `1-overview-z1.jpg` | whole map, 1:10, rush hand (0 soldiers left, mould 32,398) |
| `2-fight-z12.jpg` | near view zoom 12, 0:23, volley with shrapnel (9 soldiers selected) |
| `3-fight-z6.jpg` | near view zoom 6, 0:24 (8 selected, 2 figures visible) |
| `4-menu-z6.jpg` | the start menu over the sandbox at 0:03 |
| `5-walls-slime-z12-zoomed.jpg` | a crop of zoom 12 enlarged 2x without smoothing |
| `6-volley-z12.gif` | 3 s of the volley at zoom 12 |

## Issues, by impact on play

1. **The slime is out of focus at zoom 6 and closer.** It is the subject of
   the game and it reads as a blurred photo: thickness is a 5 x 5-cell box
   sum and the light's slope is sampled 1.5 cells apart (the slime thread's
   finding), so at zoom 12 one cell is 12 pixels of smear. Owner: "very
   blurry/out of focus". *Slime sharpness thread.*
2. **Cell staircases on every hard shape.** Rock ridges, walls and crystal
   faces are drawn from the cell grid, so their edges step every 6-12 pixels
   with a 1-cell black shadow band; crystals that touch rock are cut into
   odd shapes (Fork's two side crystals). Owner: "The crystals are
   pixelated"; the rock has the same fault. *Crystals thread, widened to all
   terrain edges.*
3. **The slime reads as sand or sponge, not as a creature.** Ochre on olive
   ground, close in value; a fine mottled texture everywhere; no veins, no
   wet film, no sense of which way it is flowing. At zoom 1 the network
   (Jones-style tubes) is legible only at its thin edges. *Slime thread.*
4. **Shots are thin lines and their hits are soft dark smudges.** The tracer
   is a 1-pixel dotted line; a hit's hole is a blurred dark spot that closes
   in under a second, so a volley reads as nothing happening. *Gun rounds
   thread (#219 brings bullets as lights on the slime and the impact light).*
5. **Your squad can't be counted.** Soldiers stack exactly on one another (9
   selected, 2 figures on screen at zoom 12; 8 and 2 at zoom 6); at zoom 6
   a soldier is a 20-pixel olive disc on ochre slime, mostly hidden by a
   heavy yellow selection ring; health bars are 7 x 1 pixels; the larder is
   a plain beige square; a dead soldier is a grey shape that reads as
   alive. *New thread: units and buildings.*
6. **The HUD hides the fight.** The bottom bar takes 160 of 800 pixels and
   stays full height with "Nothing selected"; the camera's opening view puts
   the larder and squad under it (frame 3). *Gun card HUD thread, after its
   panel lands.*
7. **The ground is a flat dark olive.** Nothing to read on open agar; the
   engine's tints (scent and others) show as flat tinted discs that look
   like UI (the blue disc top left and orange disc at the bottom crystal of
   frame 1). Soil kinds are flat colours. *Terrain, after item 2.*
8. **Explosions and gas**: under way in the explosion cards thread (blast that
   flings live slime) and the gun rounds thread (gas as a drifting cloud).
9. **Start menu**: clean and readable; its background inherits items 1-2.
10. **Map editor**: not on main yet (#209); reviewed once it merges.

## Who takes what (sent 11-14)

| thread | takes |
|---|---|
| Slime sharpness | 1, 3 |
| Crisp crystals | 2 (crystals, rock, walls, rubble), then 7 |
| Gun rounds (bullet look) | 4, gas cloud |
| Explosion cards | blasts |
| Gun card HUD | card panel, then 6 |
| new: units and buildings | 5 |
