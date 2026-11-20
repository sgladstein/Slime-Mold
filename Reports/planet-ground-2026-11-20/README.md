# Barrens and fen ground (Look C, planets 2 and 3, 2026-11-20)

Planet 1's meadow got its own growth on 11-20 (#535: patches, thickest by
water, spring-green tips). This is the same pass for planet 2 (the barrens,
flora 1) and planet 3 (the fen, flora 2), in `src/world/view.wgsl` only: no
rule or field changes, every pixel still from the plants', soils' and
moisture fields.

Page: https://claude.ai/artifact/Vk2jnBLdLoVFrCGmYHEAZu (`page/`, clips from
`make_clips.py`). Films: run seed 1, `mode=squads hand=greedy`, 0:40 to 0:44,
one build with main's shader and this one by `LARDER_VIEW_WGSL`, so both
sides are the same game.

## What was wrong (stills at 0:30 and 1:00, zoom 6 and 12)

- Barrens: the thorn scrub (58,56,40) sat on hardpan of about its own value;
  at zoom 6 the bushes were faint dark specks. The hardpan was one flat brown
  with crack lines. The barren bands (soil 3, no births) were the grey ash
  of every planet, mottled, and read as concrete beside the rock.
- Fen: the reeds round every pool were the default small green ovals
  (`flora_form` had no fen reed), dots on the shore. The moss hummocks
  (relief 0.95, lobe 0.08) were smooth dark balls, frogspawn at zoom 12.

## What changed

- **Plant shadows** (`Form::fall`, `Crown::sh`, `PlantLayer::dark`,
  `PLANT_SHADOW` 0.45): a crown's outline moved away from the light by
  `fall` of its radius darkens the ground and lower kinds. On barrens scrub
  (0.55), oasis grass (0.5) and palms (0.6); fen moss (0.45), trees (0.6)
  and reeds (0.6). Planet 1 and planets 4 to 8 are unchanged (`fall` 0).
- **Barrens scrub**: dark dry olive (46,44,30), straw tips (112,98,62),
  relief 0.8, crowns `r0` 0.5 to 0.68. Oasis grass: clump 0.4, bleached tips.
- **Mud plates** (`plain_look` flora 1; `HARDPAN_PLATE` 0.05,
  `HARDPAN_RIM` 0.45 cells, `HARDPAN_CURL` 0.18): each plate its own shade,
  its rim curled up so the side away from the light is lit. The first sign
  lit the rim toward the light: pillows that read as paving.
- **Salt pan** (`SALT_RGB` (60,56,47) by `barren_rgb()` on flora 1, at the
  ash's value; `SALT_NET` 2.8 cells, `SALT_RIDGE`, `SALT_CREST`,
  `SALT_FLANK`): a net of crust ridges bent by a slow noise and fading along
  their length. An even net (first cut, 2.2 cells) read as a tiled floor.
- **Fen reeds**: blades (shape 1) 0.42 cells apart in tussocks (clump
  0.55), straw green (46,58,34) with pale tops (104,106,64).
- **Fen moss**: lobe 0.16, relief 0.75 (cushions, not balls).

## Cost

GPU tail a frame, llvmpipe, 4 s bench at zoom 6, median, runs taken in
turn: planet 2 (300,300) main 116, 126, 121 ms against 120, 119, 116;
planet 3 (300,100) main 145, 148, 143, 154, 155, 156 against 169, 158, 150,
153, 155, 159. Inside the bench's 10-20% run-to-run noise on both.

## Round 2 (2026-11-20, later): water and rock

What still read flat at zoom 6 after round 1 (stills at 0:30 and 1:00 over
the fen's lakes and both planets' rock):

- The fen's lakes were the navy of every planet's water, the largest flat
  thing on planet 3, and with the walkers' specks in them read as night sky.
  Now bog water, green-black (`FEN_WET_RGB` (20,33,27), `FEN_DEEP_RGB`
  (4,9,6), `FEN_SHALLOW` for the margin; `wet_rgb()`, `deep_rgb()`), at the
  navy's values, so water keeps its step under the ground.
- The fen's reeds at zoom 6 were half blended into their far view (spacing
  0.42 x 6 = 2.5 px against the 1.6-3.2 fade). Spacing 0.55, `r0` 0.9: whole
  blades from zoom 6.
- Rock was the same slate on every planet. Planet 2: a rose-tan sandstone
  (`rock_tint()` (1.24,0.96,0.84) in `ter_cell`, so every zoom) in beds
  `STRATA` 2.6 cells thick, each its own shade, bent by noise. Beds 1.4
  cells apart with a strong swell read as wood grain at zoom 12 (first cut).
  Planet 3: wet stone (tint (0.94,1.02,0.97)) with moss in patches over its
  top (`ROCK_MOSS` (42,56,30), `MOSS_COVER` 0.75), away from its edge.

Stills on the page (`fen-water6.jpg`, `sandstone12.jpg`, `fen-rock12.jpg`).
Cost, llvmpipe GPU tail p50, 4 s at zoom 6, round 1's shader against
round 2's, runs 2 and 3 (run 1 overlapped with filming and is dropped):
planet 2 (300,330) 129, 127 ms against 134, 126; planet 3 (360,90) 166,
156 against 169, 168. Inside the noise.
