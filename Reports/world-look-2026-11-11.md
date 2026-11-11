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

## The owner's picks and what changed (16:48Z)

The owner, 16:48Z: "1) Solider A looks better. 2) [...] Gun fire should
hit the first slime in its path. This doesn't look good. 3) The crystals
should have hexagonal outline, not circle."

1. **Soldiers are the SVG sprite.** `src/world/sprites/soldier.svg` is
   rendered by `sprites/render.js` (Playwright's Chromium) into
   `sprites.png`, a 512 x 160 atlas: body, helmet (white, tinted with the
   team's colour in the shader), muzzle flash and a dead pose, at 128 px and
   again at 32 px for soldiers under 12 px across the radius. The view
   samples it bilinearly, rotated to the soldier's facing; the shadow stays
   a shape cast by the world's light. The shape-drawn figure's code is in
   commit 105c358.
2. **Shots stop at the first slime.** `world::look::first_hit` marches the
   shot's line in half-cell steps from the muzzle and returns the first
   mould cell (cut there with `Engine::set_hits`) or the first rock cell
   (the shot stops); player walls let shots over, as the Larder thread asked
   (it owns the firing rule and calls this). In the stand-in fight each
   soldier fires every 0.3 s at a random angle within 0.8 rad of its front,
   to full range 16 dish cells, against thickness over 60/255 read back
   every 3 frames (too thick and too stale: see the 17:37Z section); the cut is 0.4 dish cells across the radius.
3. **Crystals are hexagons.** The Larder world's simulation now makes each
   crystal's footprint a pointy-top hexagon of circumradius `pile_r` (PR
   #64, 6456f6b); `dish_crystals` draws the same hexagon in the sandbox. A
   crystal's height is charge x its depth inside the footprint, so a
   hexagon stands as a six-faced pyramid; the slope rounding that faked
   faces on round crystals is gone.

| Measure (same set-up as above, 4 s of fire) | Result |
|---|---|
| Shots / stopped by mould | 120 / 63 |
| View pass, look on, zoom 1 / 2 / 4 (lavapipe, median of 21) | 33.4 / 32.7 / 33.0 ms |
| View pass, look off | 13.5 / 13.3 / 15.2 ms |

## On the GTX 1070 (17:10Z)

The same example on Scott-Desktop at 77f4788 (built on D:, since C: was full):
`worldlook -- seconds=12 fight=4 screen=1920x1080`.

| Measure | Result |
|---|---|
| Shots / stopped by mould | 120 / 55 |
| View pass, look on, zoom 1 / 2 / 4, 1920 x 1080 (median of 21) | 1.999 / 1.835 / 1.819 ms |
| View pass, look off | 0.309 / 0.302 / 0.303 ms |

The look costs about 1.5-1.7 ms a frame more at 1080p, 6x the plain view,
about a tenth of a 16.7 ms frame. Most of it is the per-pixel height samples
(each a bilinear of four cells reading two fields) and the speck pass over
400,000 walkers. Not yet tried: working each cell's height once a frame
into a world-sized field, so a pixel reads one field instead of
recomputing it.

## Shots that flew through slime (owner, 17:37Z)

The owner: "The report now says that the shots hit the first slime in the
path, but from looking at the gif/video it doesn't always look like that."
They were right. Two causes in the stand-in fight:

1. **Slime to a shot was thickness over 60/255.** The view lays slime
   colour over the ground by 1 - exp(-5 t), so a cell at 60 is already 70%
   slime and one at 26 is 40%. Shots flew through every tube thinner than 60,
   which on screen is most of the rim of each hole.
2. **Thickness was read back every third frame**, so a tube that grew in
   since could be missed.

The deciding measure, named before the runs: shots whose drawn line crosses
a cell that looks like slime this frame (thickness 26 or more, read back
fresh), short of the shot's last cell. Shots whose muzzle is already in
slime (soldiers standing in a tube) are counted apart: they hit at once, as
they should. Seed 1, 120 shots each, lavapipe:

| Slime from / read back | Stopped by mould | Crossed slime | From inside slime | Mean flight |
|---|---|---|---|---|
| 60 / every 3 frames (as shipped) | 60 | 98 | 22 | 90 cells |
| 60 / every frame | 57 | 100 | 19 | 90 cells |
| 26 / every 3 frames | 120 | 9 | 17 | 36 cells |
| 26 / every frame (now) | 120 | 0 | 18 | 35 cells |

The threshold is the cause; the stale read adds 9 crossings once it is
fixed. Now 26 and every frame (the `HIT` constant in
`examples/worldlook.rs`; `hit=` and `every=` set them). The fight video
and the stills on the page are re-rendered with it. Every shot now ends in
mould: the arc ahead of the squad has no clear line of 16 cells. Reading
the look field back every frame is a 5 MB copy a frame for this world; a
game would read back only the cells near soldiers. `first_hit` itself is
unchanged: the caller's `mould` test decides what counts, so the Larder
thread's caller should use the same visible threshold.

## Ground, rock and wet (owner, 19:29Z), and the prep pass

The owner: "In the pictures that I have seen they are still just black
background." True: the view had rock and wet colours, but the test world
had neither, and bare agar was drawn in the engine's own flat near-black.

- **Bare ground is soil**: a mid loam with the engine's tints (scent,
  species, germ, grass) laid over it at 0.6 strength.
- **Rock** is the dish's own terrain noise (`big::terrain_noise`, channel
  0 under 0.22, blobs 12 dish cells): 205,231 fine cells taken off the
  agar with `Engine::block`, so the mould really cannot enter. A flat
  stone plateau 3 cells high, lit, with a cast shadow.
- **Wet ground** (channel 1 under 0.22, 229,495 cells): glossy blue-green
  water, with mould in it keeping its colour. The GPU engine does not slow
  walkers on it yet, so in this test it is a preview, not the simulation.
  The Larder world's engine ground word (PR #74) carries a WET bit that
  will, and the view moves to that word when it lands.
- **Elevation** has a home (ground byte 3, which the Larder thread
  reserves in the engine's word too), and the view raises and lights the
  land by it. Maps leave it 0 until the simulation uses height (no
  drawn-only relief); the Larder thread proposes it as the first new
  ground kind.

**The prep pass.** A pixel of the look read 24 heights and 4 materials,
each worked out from the fields. Now `prep` works out each shown cell
once a frame (plus a 4-cell margin) and `base` reads them. Picture: at
most 1/255 different at zoom 1/2/4. lavapipe, look on, zoom 1/2/4:
33.4 / 32.7 / 33.0 -> 35.9 / 23.9 / 24.5 ms.

On the GTX 1070, 1920 x 1080, six runs alternating the control (2598126,
before the prep pass) with 6378ccf (prep pass and ground), view GPU ms,
median of 21 each, look on at zoom 1 / 2 / 4:

| Run | Commit | Look | Plain (z1) |
|---|---|---|---|
| 1 | control | 2.912 / 1.913 / 2.054 | 0.386 |
| 2 | prep | 1.184 / 0.817 / 0.883 | 0.287 |
| 3 | control | 1.814 / 1.633 / 1.726 | 0.298 |
| 4 | prep | 1.670 / 1.290 / 1.513 | 0.370 |
| 5 | control | 1.818 / 1.637 / 1.734 | 0.298 |
| 6 | prep | 1.187 / 0.821 / 0.890 | 0.288 |

The card has two speed states: runs 1 and 4 are slow (their plain view is
0.37-0.39 ms against 0.29-0.30). In the matched fast runs (3, 5 against
2, 6) the look falls from 1.81 / 1.63 / 1.73 to 1.19 / 0.82 / 0.89 ms:
its extra over the plain view goes from about 1.4 to about 0.6-0.9 ms a
frame. Prep is faster in all three pairs at every zoom; one seed and one
place, so it is a timing, not a census.
