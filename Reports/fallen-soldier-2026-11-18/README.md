# The fallen soldier (2026-11-18)

The owner's playtest note, 2026-11-18 16:12Z: "When soliders die, the body
on the floor looks really bad. You could create a better looking body or
just a blood splotch." Both: a better body over a pool of blood. Page:
`page/index.html`, published at
https://claude.ai/artifact/A6fJKZbrQDx9PCwuHWFBTj (listed in `Reports/pages.md`).

## Before

The 11-14 corpse (art review: "the dead look dead") was the living figure's
tile redrawn in cold greys, flat on its back with its limbs flung wide and
the helmet rolled off, as big as a living soldier. At zoom 6 it read as a
grey spider or tick, not a person: four stick limbs round a round grey
body.

## Now

- **A soldier lying full length** (`src/world/sprites/soldier.svg`, the
  `dead` layer; the atlas's tile 3 re-rendered with `render.js`, the other
  16 tiles byte-identical): on its back, head and helmet behind it and boots
  toward what it faced (it fell back from the slime it was shooting), one
  arm flung past its head and the other out at its side, the rifle dropped
  beside it. The uniform is the living olive, darker and duller, so it is
  plainly one of ours; no outline, rim light or shadow, as before (it lies
  on the ground).
- **A pool of blood under it** (`blood_pool` in `view.wgsl`), drawn by the
  shader, not the tile: lumpy, centred under the chest, dark at its rim and
  red inside with a soft glint on the side toward the world's light, and
  four drops flung past its edge. It spreads from a quarter of its size at
  the fall to all of it over `POOL_GROW` 2.5 s, fast then slowing
  (`Look::fallen` in the window counts the seconds since the fall;
  `Marks::fallen` carries the share and a seed, so no two pools match). Its
  radius is 0.95 of the figure's (1.2 times that along the body), inside
  the soldier mark's box.
- The fallen keep the facing they died with (before, a figure caught
  mid-turn kept turning for a moment after it fell).

## Films

Built from main at 60502ed with this change; the before stills from main
itself, the same game.

| Scene | Zoom | Soldiers standing |
|---|---|---|
| The Fork held, plain rounds, 25 to 28 s (not squad mode) | 12 and 6 | 8 at 25 s, 7 from 26 s: one falls in the clip |
| Planet 2, run seed 1, greedy hand, 86 to 92 s | 6 and 12 | 11 at 86 s, 8 at 92 s (the hand buys more as they fall) |

## Seen

- **At zoom 6 a corpse reads as a soldier in its blood**, olive on red on
  the gold slime, where the grey shape read as a creature.
- **The pool spreads after the fall** for about two seconds, so a death in
  the fight is a beat you can catch, then it lies still.
- **Where many fall in one place they pile up** (planet 2's crystal edge:
  six bodies and their pools in one heap). That is where they died; the
  game keeps every body for the planet.

## Cost

A fallen soldier's pixels run the pool (an `atan2`, four sines and four
drops) in the marks pass, inside its mark's box, 4 figure radii across: at
zoom 12 about 7,000 pixels a body. Not timed on the GTX 1070 (the cloud's
software renderer is not for timing); twenty bodies on screen add about
140,000 pixel evaluations of tens of operations, well under a tenth of a
millisecond on that card by estimate.

## Files

- `page/`: the page, four clips (H.264, 30 fps, the middle 960 x 600 at
  its own pixels) with posters, and two before and after stills.
- `raw/film.sh OUT [OLD_GAME]` films them; `raw/make_clips.py OUT PAGE`
  makes the clips and stills and prints the soldiers standing.
