# Units and buildings that read, 2026-11-14

Art review 2026-11-14, issue 5 (the art director's thread; style guide rule
6, "Units read at zoom 6"): soldiers stacked exactly on each other (9
selected, 2 figures at zoom 12), a soldier at zoom 6 was a 20-pixel olive
disc on ochre slime under a heavy yellow ring, health bars were 7 x 1
pixels, the larder was a beige square, the dead read as alive.

Page: https://claude.ai/artifact/FQ1y7vhrqpgGZGn9LFw64F (source `page/`,
before = main 3d31dd3, after = this branch). Films made headless on
llvmpipe, Fork, seed 1, shape C, 1280 x 800 cropped:
`larder-world map=fork look=DIR hand=volley rounds=shrapnel centre=268,215,Z at=21..24` (15 frames a second, every other frame kept),
`hand=far centre=250,172,Z at=7..11`, `hand=build centre=224,120,12 at=20`.

## What changed

- **Elbow room** (`Rules::elbow` 1.6 touch radii, 0 off; `Game::elbow_room`
  in `paths.rs`, run before each frame's walk): two living soldiers closer
  than that each step half the overlap apart, at most a walk step at
  max(wade, 0.5) of full speed, onto standable ground only, pushes taken
  from positions before the pass (order-free). The sunflower slots widen
  to 2.52 dish cells a ring (was 1.5; 1.5 again with elbow room off) so a squad at rest sits at the
  separation; builders get the separation added to their reach (without it
  two builders raised a turret in 56 ticks against one's 61). Agreed with
  the integration thread, which reviews. At 1.3 a five-soldier knot still
  overlapped at zoom 6; 1.6 shows each one. **Census baseline break**: a
  squad's drain spreads over more ground.
- **Soldier figure** (`view.wgsl` `soldier`): drawn 1.15 touch radii, at least
  10 pixels; body olive lightened; a dark outline 1.5 screen pixels round
  body and helmet (eight taps of the atlas); a cream rim light on edges
  facing the light. Selection: `Marks::selected`, a 1-pixel cream ring over a
  1-pixel dark one, 0.95 of the figure radius + 2 pixels (was a 1.5-pixel
  yellow ring 4 pixels outside). Health: `Marks::health`, only under 95%,
  18 x 3 pixels in a dark frame, green / amber under 60% / red under 30%,
  drawn after every figure.
- **Dead**: a new pose in `soldier.svg` (flat, limbs flung, cold greys, helmet
  rolled off, rifle dropped), no team tint, outline, rim or shadow; the dead
  are drawn before the living.
- **Buildings** (`view.wgsl` `building`): a ghost is a cream plan (the
  outline in cream, body at 22%) filling from its foot with a cream seam
  at the rising line; standing buildings get the soldier's dark outline;
  hurt ones scorch (a share of texels charred, about half the health lost)
  as well as darkening. A ghost's low hp is not drawn as hurt, and its
  health bar shows only once it stands.
- **Larder**: sprite tiles 14-15 in `buildings.svg` (atlas now 2048 x 160):
  hipped roof, sacks, four charge cells lit while the bank holds 1 food or
  more, scorching by `Game::overrun_share` (the overrun count, 0-1).

## Bug found on the way

The first outline and rim used colours in 0-1 while the atlas is 0-255, so
both came out black; a ghost's cream line only showed after the fix.

## Cost

llvmpipe, `bench=30 hand=volley`, 1280 x 800, squad on screen: GPU tail
p50 37.6 -> 40.2 ms, frame p50 47.0 -> 49.9 ms. No 1070 timing (the PC is
busy). Elbow room is O(n²): 780 pairs at 40 soldiers, 0.012 ms a call
on the CPU (40 soldiers packed in a 3.5 x 3 cell knot, 2,000 calls,
release build), once a game frame.

## Engine twins

Two GPU-against-CPU twins in `mouldgpu` (seed 3) left their bound with the
new walk: the crater twin 896 against 1014 walkers at 30 s (with elbow
room), the core-pull twin 1647 against 1516 at 25 s (from the wider slots
alone; with 1.5 slots it passed). Both turn elbow room off, as they turn
off solid crystals: a game-layer rule the same on both engines that moves
where soldiers stand, so seed 3's chaos parts the moulds. Slots are 1.5
with elbow room off, so those twins play as before.
