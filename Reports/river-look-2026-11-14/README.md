# The river look: deep water and plank bridges (2026-11-14)

**Ask.** The owner, 11-14 18:01Z, on the first deep water (#285): "This
needs a lot of work visually. It is just a blue curved line on the map. It
doesn't look professional." The coordinator gave the materials to this
thread. The ground kinds thread redrew the River map's shape (river v2,
04b0ec9c): a wandering deep channel, a shallow margin, sand and clay banks,
fords, a pond and a creek. The art lead added two notes at 18:56Z on the
round 3 planets (#289):
- Water had the ground's value in grey. It should be a clear step
  darker, with deep water darker again.
- Soft rock at full health was as bright as thin slime. It should be a
  warm mid-grey, just above rock.

**Page:** https://claude.ai/artifact/EUtwNmz986oPhghQLioRhv (copy: `index.html` here).

## What changed (`view.wgsl` only)

Deep water is soil 4 with the wet bit and a bridge is soil 5, as #285
encodes them. Without #285 neither code appears, so nothing here changes
until it lands.

- **Deep water** is drawn at the pixel. It darkens from the
  water's colour at its edge to `DEEP_RGB` (1,6,14) about five cells in.
  Depth is the share of deep cells on two rings round the pixel (4 points
  at 2 cells, 8 at 5) times its cubic share.
- **A slow value-noise swell** keeps open water from looking flat.
- **The water's edge** stays the crisp, antialiased curve from #281. Inside
  it there is a shallow margin about a cell wide, where the bed shows
  through. Outside it there is a thin light waterline, then damp, darker
  ground about a cell wide.
- **Water is darker overall**: `WET_RGB` is (5,14,22), was (10,24,34). In
  grey it is now a clear step under every ground, so Fen's lakes, Meadow's
  streams and the Valley's river show in the 266x164 thumbnails.
- **The cells' own deep colour** isn't used. A darker deep colour in
  `cell_mat_c` showed the cells' bilinear blend as a stair inside the
  water, so the darkening happens only at the pixel.
- **A bridge** is a plank deck, its edge crisp like the water's.
  - The planks are 0.8 cells wide, laid across the way over. The way over
    is the axis along which the deck has no deep water beside it, found
    by 12 point reads.
  - A dark seam runs between planks, and each plank has its own shade and
    a faint grain.
  - A rail half a cell wide runs along each edge, with a lighter post
    every 3 cells.
  - The rail facing the key light is lit and the far one shaded, and the
    deck casts a half-cell shadow on the water on the side away from the
    light.
- **Soft rock (walls)** at full health is (128,118,104), was (168,156,132):
  just above rock and below thin slime. Damaged it goes to (58,50,42), was
  (70,58,48).
- **Early-outs (art lead, after #286)**:
  - No soil detail where the slime's cover leaves under 3% of the ground
    showing (it was 1%).
  - Clay's Voronoi is skipped at zoom 1 and below, and barren's grains at
    zoom 2 and below.

## Tried and dropped

- **Glitter on the water.** Streaks across the key light read as rain at
  zoom 6 and 12. Thresholded value-noise flecks read as angular chips, and
  a thresholded sheen as square blobs. Only the unthresholded swell is
  kept.
- **A deck height in `cell_height`.** The shading reads height as mould
  thickness, so the soil texture faded off the deck and the wood read
  grey. The bevel is now drawn in the material instead.

## Pictures

- **River (#285's river v2, seed 1, rush hand, 2-8 s):**
  - `river-old-z6.gif` and `river-new-z6.gif`: zoom 6, centre 236,150.
  - `river-old-z12.gif` and `river-new-z12.gif`: zoom 12, centre 250,155.
  - Built from the #285 branch at 04b0ec9c, with this `view.wgsl` and with
    its own.
- **Fen (#289, planet 3, seed 1, rush hand):** `fen-old-z6.gif` and
  `fen-new-z6.gif`, zoom 6, centre 300,180.
- **Round 3 planets in grey, seed 1, 5 s, 266x164:** `thumbs-old.png` and
  `thumbs-new.png`. Top row: Meadow, Barrens, Fen, Valley. Bottom row:
  Jungle, Highlands, Caves, Core. Fen's lakes, the Valley's river and
  Meadow's streams now read as dark hollows, and Jungle's hedges sit under
  the slime's value.

## Cost

llvmpipe, `map=river seed=1 bench=8 hand=idle screen=1280x800`, GPU tail
p50, alternating rounds. "Before" is #285's own `view.wgsl`, which has
#281's soils but not #286's.

| Build | ms |
| --- | --- |
| Before | 80.2-83.3 |
| First cut | 96.1-104.2 (five rounds, median 98.6) |
| With the early-outs | 93.2-102.1 (median 99.5): no change, since the bench sits at zoom 6 and most ground is bare |
| No depth rings (to measure them) | 94.7-95.8 |
| All soil detail off (to measure it) | 77.8-77.9 |
| Final: constant ring offsets, 12 points, no bank work inside the water | 92.4-95.2 (median about 94.5), **+12.5 ms** |

Where the cost goes: on this map the soil detail as a whole is about 16 ms
(its 16-cell read plus the textures on a mostly water, sand and clay
screen), and the depth rings about 4.

The GTX 1070 timing is booked with the other pending looks. That whole
frame was under 2.6 ms p95 this morning.
