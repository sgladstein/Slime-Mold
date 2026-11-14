# Soil materials for the planets (2026-11-14)

**Ask.** The art lead, 11-14 16:19Z (review issue 7): the roguelike's
procedural planets (#277) build biomes from barren, wet, sand, clay, rock
and soft rock, but drawn as flat colours the biomes can't be told apart.
Give each soil its own material from the soil field: sand fine wind
ripples with its value just above the ground; clay dark with crack lines;
barren ash grey with a dry speckle; wet deep and dark with a soft
key-light gloss and a lighter edge where it meets land; rock raised, lit
from top left, with a lit rim and a shade side. Keep the value order and
rule 4's crisp edges. Test: every biome recognisable from a grey 266x164
thumbnail, and zoom 6 has something to read.

**Page:** https://claude.ai/artifact/7Cyz76JZc5TVJnBDhhHw8y (copy: `index.html` here).

## What changed (`view.wgsl` only)

- New `soil_share` / `soil_detail`, applied to the ground's albedo in
  `shaded` where it is bare (fades out under mould, `exp(-30 th)`).
  The soil shares are a cubic B-spline over the 4 x 4 cells round the
  pixel, as `terrain()` does for rock, so no patch edge shows the cell
  stair.
- **Sand** (50,48,42), was (70,68,60): just above plain ground. Wind
  ripples 1.6 cells apart, wavy, +-14%, faded in from zoom ~2; long dune
  swells 14 cells apart (+-16%) that a whole-map thumbnail still shows.
- **Clay** (38,28,26), was (56,38,34): darker. Voronoi plates 3 cells
  across with dark cracks about a pixel wide (60% darker), and a slight
  plate-to-plate variation from zoom 3 up.
- **Barren** (64,64,62), was (50,50,50): the lightest soil, ash grey, with a
  speckle of light and dark grains half a cell across from zoom 3 up.
- **Wet** (10,24,34), was (14,30,42): darker. Its edge is where the smooth
  wet share crosses one half, antialiased over one pixel (it was the cell
  stair of the material blend); the land just outside it has a lighter
  shore about half a cell wide. The existing gloss 0.75 gives the soft
  key-light sheen.
- **Rock** (`ter_cell` 92-138 grey, was darker) with a steeper bevel
  (tilt 0.8 + 0.2 h, was 0.35 + 0.15 h) and a lit rim on the edge facing
  the key light, two pixels wide; the far side is in the bevel's shade.
- Value order kept: water < clay < plain ground ~ sand < barren < rock <
  slime film.

## Pictures

`thumbs-old.png` / `thumbs-new.png`: the eight planets (Barrens, Marsh,
Canyon, Dunes / Claypan, Thicket, Spires, Core), seed 1, at 5 s, whole map
in grey at 266x164. GIFs at zoom 6, rush hand, 2-8 s, centre 300,180:
`p1-new.gif` Barrens, `p2-old.gif` / `p2-new.gif` Marsh, `p3-new.gif`
Canyon, `p4-new.gif` Dunes, `p5-new.gif` Claypan. Frames from
`larder-world map=planet:D:1 seed=1 look=DIR hand=rush at=... screen=960x600
centre=300,180,6` on llvmpipe, built on #277's tree with this view.wgsl.

Read of the grey thumbnails: Barrens (pale ash patches), Marsh (darkest,
smooth), Canyon (bright rock bands), Dunes (diagonal swells) and Core now
read apart; Claypan, Thicket and Spires are still close at thumbnail size
(clay cracks only show from about zoom 3), so the tell between those is
their shapes, not their soil.

## Cost

llvmpipe, planet 4 (Dunes), seed 1, `bench=8 hand=idle screen=1280x800`,
three alternating pairs, GPU tail p50: before 75.5 / 75.3 / 74.3 ms, after
81.6 / 81.2 / 80.3 ms, about +6 ms (8%). Most of it is the 16-cell soil
read per ground pixel. Not timed on the GTX 1070 yet (its whole frame was
under 2.6 ms p95 this morning).

## Not done

- Soft rock is the wall kind; it gets the same bevel and lit rim as rock but keeps the wall's colour, with no material of its own.
- A thumbnail tell for Claypan against Thicket/Spires would need a
  coarser texture than cracks (for example clay-darkened ground at map
  scale); not tried.

## Follow-up (art lead, 11-14 16:54Z)

The art lead accepted #281 and asked for three things.

- **Sand ripples read as a hatch at zoom 6 and a moire at zoom 1.** The
  ripples' phase is now bent by two octaves of value noise (one at 24
  cells, which also bends the dune swells, and one at 4 cells), which
  also squeezes and stretches their spacing. They fade to flat sand
  between zoom 4 and 2.5. Sums of sines cost the same but looked like
  wood grain.
- **Barren ash was close to rock in grey, and its speckle read as TV
  static.** It is a step darker, (55,55,53) where it was (64,64,62). Its
  grains are soft value noise about a cell across, in clumps a few cells
  wide with bare ash between, and fade in from zoom 2-4.
- **Bake the soil instead of reading 16 cells a pixel.** Built and
  measured, with no saving, so it was reverted (see `Reports/dead-ends.md`).
  Five alternating rounds, Dunes, GPU tail p50:

  | Build | Median (ms) |
  | --- | --- |
  | main before #281 | 73.8 |
  | #281 | 82.7 |
  | This follow-up | 83.4 |

  An earlier five-round set gave 74.9, 81.0 and 83.3 for the follow-up's
  first form with the bake, and 77.6 for the bake with no texture. The
  cost is the arithmetic per pixel. A real bake would have to run only
  when the ground changes, which needs the game to say when that is.

Pictures: `thumbs-v2.png` (the eight planets in grey, as above),
`p1-v2.gif` (Barrens) and `p4-v2.gif` (Dunes) at zoom 6.
