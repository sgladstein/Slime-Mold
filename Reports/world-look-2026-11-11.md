# The world look: the Larder world drawn from the simulation (2026-11-11)

**Built on the camera branch (PR #63), pictures from the software GPU; not yet
timed on the GTX 1070 (its C: drive was full) and not yet played.** Page:
https://claude.ai/artifact/J7K7W5hSvxutqZmhN91yVd (copy in
[`world-look-2026-11-11/`](world-look-2026-11-11/), open `page.html`).

## The ask

The owner, 2026-11-11 15:50Z: shader art "makes sense for most of this, but
it probably wont work well for creating a solider, right? If I am wrong we
can test it. If that doesn't work, then SVG sprites, Free CC0 packs, or an
image generator are also good options to explore. As much as possible
(especially with the slime) the visuals should follow the simulation, not
just pretty graphics on top." 15:52Z: "This should include not just the
soldier, but the shooting animation, the visual of the damage to the slime,
the environment, the walls, etc." 15:59Z: "food/energy crystals will also
need a good visual".

## What was built

1. **Engine** (`src/gpu/engine.wgsl`, `slime_mould::gpu`): `Engine::set_hits`
   takes up to `MAX_HITS` (256) shot discs a frame, which cut trail and germ
   as the right mouse's wipe does. Each cell's wound records the thickness a
   hit took (the trail's brightness there), fading by 1/128 a tick (at least
   1 of 255). The draw pass writes a `look` u32 a cell: thickness (the
   trail's brightness, the number the colour ramp already uses), walkers on
   the cell, the wound and the scent. `Engine::block` takes cells off the
   agar (walls the mould cannot cross).
2. **Ground field** (`slime_mould::world::look`): a u32 a cell, kind (agar,
   wet, rock, wall, rubble, crystal) and value (wall health share, crystal
   charge share), plus a crystal's depth inside it. `ground` builds it from
   the world's own fields (the Larder world's `agar`, `rock`, `wet`, `wall`,
   `wall_health`, `charge` and `pile_of`, PR #64); `dish_crystals` makes
   crystals from the sandbox dish's piles.
3. **View** (`src/world/view.wgsl`, `View::set_look`): with the look on and
   zoom at least 1, each screen pixel is shaded from the fields, smooth
   between cells: height is 4 x thickness^0.65 for mould, 3 for rock,
   0.6 + 2.6 x health for a wall, (0.6 + 9 x depth) x charge for a crystal.
   It is lit from the top left, with gloss growing with thickness and a short
   cast shadow. Mould takes the engine colour's hue at full strength by
   coverage 1 - exp(-5 x thickness), so species, germ and scent tints still
   come from the engine. A crystal's slope is turned to the nearest sixth,
   so it reads as a six-faced cone, and it glows by charge. A wound glows
   ember orange and chars as it fades. Every walker is a speck at its own
   position (`draw_specks`, one thread a walker). The plain frame path is
   unchanged with the look off, so the camera's zoom-1 identity test holds.
4. **Soldier** (`Shape::Soldier`, `Marks::soldier`): drawn in the shader from
   shapes (shadow, pack, shoulders, arms, rifle, helmet, muzzle flash), lit
   by the world's light, facing any angle, the helmet in the team's colour,
   fatigues olive leaning a quarter toward it, dimmed and unarmed when dead.
   Tracers are `Line` marks that fade over the shot's 0.09 s; the flash lasts
   0.07 s.
5. **Sandbox**: L toggles the look (on by default); the stand-in squad draws
   as soldiers facing the way they last walked.
6. **`examples/worldlook`**: the stills, the fight and the timings below.

## The test

`cargo run --release --example worldlook -- out=DIR seconds=10 fight=2`
(lavapipe, 2048 x 1280, 400,000 walkers, seed 1, screen 1280 x 800). Walls
are an L in front of the squad and one wall whose health is set by hand from
1 at the top to 0 at the bottom, to show the ramp; the GPU world has no wall
health yet. The squad of nine fires every 0.3 s at the thickest of 48
sampled cells in range (16 dish cells) whose thickness is over 60/255, read
back every 6 frames; a hit is a disc 0.55 dish cells across the radius.

| Measure | Result |
|---|---|
| Shots in 2 s / at thick mould | 60 / 42 |
| View pass, look on, zoom 1 / 2 / 4 (lavapipe, median of 21) | 31.5 / 33.3 / 31.5 ms |
| View pass, look off, same frames | 13.5 / 14.8 / 15.5 ms |
| Ratio | about 2.2x |

On the 1070 the plain view was 0.33 ms at 1920 x 1080
([world camera](world-camera-2026-11-11.md)); the look's cost there is the
next measure.

## The soldier: shader against an SVG sprite

The owner doubted a shader soldier would read. The page shows a blind A/B at
zoom 1, 2 and 4 over the same ground. **Key (kept off the page): A is an SVG
sprite (`world-look-2026-11-11/soldier-a.svg`, rendered in Chromium and
rotated), B is the shader.** Both use the same design so the comparison is of
the method: the SVG has outlines and hands; the shader shares the world's
light. At zoom 1 a soldier is 14 px across and neither reads as more than a
dot with a gun; from zoom 2 both read as a helmeted rifleman. CC0 packs
(Kenney) and image generators were not tried: the cloud's proxy refuses
kenney.nl and opengameart.org. Either can be fetched on the owner's PC if the
A/B says sprites are worth it.

## Not done

- Wall health, rubble and draining crystals come from the Larder world's CPU
  dish (PR #64); the GPU world shows full walls and full crystals until that
  reaches it. Walkers that die (W0, growth) will show as fewer specks.
- Walkers that were inside a cell when it became a wall stay there as specks.
- Soldiers' shadows can cover a neighbour's body where they overlap (each
  pixel shows one mark).
