# Procedural planets for the roguelike run (2026-11-14)

The roguelike run (a separate mode beside the campaign) plays a chain of
levels. This thread builds those levels: seeded maps in the Larder World map
format (WM1: a ground picture and an item list), so the game builds them with
`GpuWorld::with_map` like the five designed maps and the editor opens them.
Code: `src/mould/levelgen.rs`; harness: `examples/planets.rs`.

## The owner's word

- 15:22Z: "We should have a full separate thread for Procedurally generated levels. That is a huge task".
- 15:25Z: "My current story behind the roguelike/levels/world. Is that the slime is taking over planets and each level is a new planet as we move towards the core slime world. This is just head cannon right now, but it could guide our level design".
- 15:33Z, answers to this thread's questions: level length "15/20 minutes to 1 hour+" with "a lot of variability"; size "first planet about 60% of today's map, today's size by planet 3 or 4, up to 6 the area for the core world"; biomes in a "Set order"; the designed maps "will not come at all. They can be played in sandbox"; twists before landing "Yes"; a Random planet button "Yes" with "a dev mode to control what level planet i go to"; unfair starts "Maybe, not sure yet. slime, size for sure"; and "some planets that have remains of bases or even start with some soliders, but those should be rare ... more of a rescue the human planet".
- 16:50Z, after round 1's page: "we need way more variability between biomes. There should be some with plants and water in many of them. They should look really different terrain. Some are open, some have lots of rock making different areas, etc. think about how this affects strategy. There should be different crystal distributions and sizes" ([brainstorm](levelgen-2026-11-14/biomes-brainstorm.md)).
- Earlier (project chat, 15:21Z): the roguelike stays separate from the campaign for now; maps get bigger as the run goes deeper.

## The interface (agreed with the run thread, 15:25-15:40Z)

- `levelgen::level(seed, depth) -> Map`: pure, depth from 1. The run thread
  owns the rest of a level (the mould tier, walkers, the goal, rules,
  upgrades); this module owns the ground, crystals, nests (mould starts),
  landing, soft rock and, next, the map's size.
- `levelgen::planet(seed, depth) -> Planet { seed, depth, name, biome, size }`:
  the card the shop shows before landing, with no map built.
- A run is eight planets (`run::PLANETS`), the core world the eighth
  (`levelgen::CORE_DEPTH`).
- The same seed and depth give the same planet on every machine. The
  generator uses only `+ - * /`, `sqrt` and `floor`, which IEEE fixes; no
  `sin`, `cos`, `exp`, `powf` or `hypot`, which come from the platform's maths
  library and differ between Windows and Linux (HANDOFF, decision 6). Turning
  a vector by 15 degrees uses written constants. `planets_are_pinned` pins
  eight planets' fingerprints, so the owner's PC fails that test if its build
  ever draws a different planet from CI's.

## How a planet is made

Recipes over regions, not raw noise:

1. **Sites.** The landing near an edge (a short edge four times in five, so
   the slime is a long way off), the first nest across the map, more nests
   in the far half 280 cells apart (one nest to planet 3, two from 4, three
   from 8), then field sites by dart throwing.
2. **Regions.** A Voronoi diagram of the sites with its borders warped by
   noise (60 cells), so regions are organic shapes, not polygons.
3. **Borders.** A random spanning tree of the regions is always passable.
   Borders across the route (landing to nest) are mostly ridges with a pass:
   the lines a squad has to get through. Borders along the route are mostly
   open. Off the tree, the recipe's odds decide ridge, pass, or a pass shut
   with soft rock (wall items, which the slime and shots wear down).
4. **Rock.** Ridges along shut borders, their width varied by noise; a pass
   is a slot through the ridge. On the Canyon, *fronts*: long ridges right
   across the map, across the route, at even steps between the landing and
   the far nest, each with one or two passes. Mesas in the fields, never
   within 120 cells of a site; on Spires, many small ragged pillars.
5. **Soil and water.** Each biome's signature shape, independent of the
   regions: barren bands across the route, broad pools, sand patches, clay
   cracked into polygons (a jittered grid's Voronoi borders). Plain ground
   round the landing (150 cells) and clay round the nests on the biomes that
   hold the slime at home, as the designed maps do, both with ragged edges.
6. **Crystals.** A rich home crystal under each nest (the slime starts on
   it), a medium one 85-140 cells from the landing toward the middle, one to
   three big contested ones midway, small ones scattered; none on rock or
   wet ground.
7. **Soft rock.** Gates across some passes; on Thicket and Core a ring
   round each nest with its gap facing the landing (the Nest's shape), and
   curved hedges. Each is a chain of short pieces that wobbles and changes
   thickness (11-21 cells), so it reads as grown.
8. **The checks** (`levelgen::check`), on a 4-cell walking grid narrowed by
   one cell so a pass a squad can't get through counts as shut: every
   crystal and nest reachable from the landing (breaking soft rock allowed);
   the far nest at least 0.6 of the map's width away on foot and every nest
   0.45; a crystal within 200 cells' walk of the landing; half the charge
   reachable without breaking soft rock; rock under 30%; every nest placed.
   A failed attempt is rebuilt from the next attempt's stream (24 tries,
   then an open plan with no ridges, which no seed has needed).

## The eight biomes, in the set order

The ground kinds are the ones the simulation already plays and the soil test
measured ([larder maps, *Which ground the mould feels*](larder-maps-2026-11-11.md)):
wet and barren ground halve the slime, sand spreads it (about 60% bigger),
clay holds it in place.

| Planet | Biome | What it is | Why there |
| --- | --- | --- | --- |
| 1 | Barrens | Three wide barren bands right across the route | Starves it: the easy first planet |
| 2 | Marsh | Big wet pools that run together, on clay banks | Slows it |
| 3 | Canyon | Two long ridges right across the map (three past planet 8), each with one or two passes | Chokepoints |
| 4 | Dunes | Sand everywhere, big ragged clay islands under the crystals (the Flats' idea) | The slime races; first planet with two nests |
| 5 | Claypan | Clay nearly everywhere, cracked into polygons of plain ground | The slime digs in and is slow to leave |
| 6 | Thicket | Grown soft-rock hedges and walled nests | Walls to chew through, rings at home |
| 7 | Spires | Many small rock pillars and a few mesas on sand and clay | Cover the slime grows behind (the tiers page's "grows round cover" idea), so the squad has to move |
| 8 | Core | Every ground, three nests, rings and gates | The core world |

Past planet 8 the order repeats (for longer runs). Passes narrow 3% a planet
to a floor of 70%.

## Numbers: 400 planets

`cargo run --release --example planets -- seeds=1-50 depths=1-8`
([raw](levelgen-2026-11-14/raw/planets-seeds-1-50-look2.txt); seeds 1-4
alone: [raw](levelgen-2026-11-14/raw/planets-seeds-1-4-look2.txt); the first
layouts' numbers: [raw](levelgen-2026-11-14/raw/planets-seeds-1-50.txt)).
Medians over 50 seeds each, ranges in brackets; walks in map cells (the map
is 1024 wide). Soft-rock pieces count each wall item.

| Biome | Crystals | Rock | Sand | Clay | Barren | Wet | Soft-rock pieces | Walk to nearest nest | Walk to nearest crystal | Charge open without breaking soft rock | First attempt passes |
| --- | --- | --- | --- | --- | --- | --- | --- | --- | --- | --- | --- |
| barrens | 16 | 6% | 0% | 0% | 27% | 0% | 4 | 794 (643-942) | 62 (51-140) | 100% (72-100%) | 45/50 |
| marsh | 16 | 4% | 0% | 28% | 0% | 28% | 4 | 790 (654-956) | 56 (47-87) | 100% (100-100%) | 49/50 |
| canyon | 16 | 14% | 27% | 0% | 0% | 0% | 2 | 888 (630-1339) | 68 (50-105) | 100% (77-100%) | 47/50 |
| dunes | 22 | 3% | 41% | 42% | 0% | 0% | 0 | 578 (484-770) | 62 (48-106) | 100% (100-100%) | 48/50 |
| claypan | 17 | 7% | 0% | 75% | 0% | 0% | 4 | 610 (498-856) | 62 (47-181) | 100% (57-100%) | 50/50 |
| thicket | 17 | 6% | 0% | 46% | 0% | 0% | 130 | 587 (494-900) | 66 (48-108) | 98% (53-100%) | 45/50 |
| spires | 17 | 9% | 28% | 20% | 0% | 0% | 0 | 552 (492-885) | 65 (48-108) | 100% (97-100%) | 48/50 |
| core | 21 | 8% | 14% | 32% | 5% | 7% | 90 | 542 (478-652) | 56 (44-144) | 88% (55-100%) | 36/50 |

- Every one of the 400 planets passed its checks within four attempts, 368 on the first. None used the fallback. A planet takes 65 ms to build (median; at most 324 ms)
  on one cloud core while two films ran beside it. It runs once a level,
  never a frame: no frame cost.
- Walks to the nearest nest drop from about 800 cells to about 550 at
  planet 4, where the second nest arrives; the far nest stays across the
  map (`to_first` in the raw files).

**Tried and changed.** A pass was first a disc on the border. Where the
ridge ran wider than the disc, rock was left across it: on the canyon, seeds
2 and 3 took 13 and 6 attempts (their first attempts had a nest and 10-11
crystals shut off). A pass is now a slot: its radius along the border,
the ridge's widest across it. Seeds 1-4's canyons then passed on attempts
0, 0, 2, 0. Barren bands first covered 43-64% of planet 1 (seeds 1-3); the
band's push was cut from 0.18 to 0.12, giving 30-59% (median 38% over 50).

## In the game

`larder-world map=planet:DEPTH[:SEED]` plays a planet (seed from `seed=`
when left out), and `larder-edit map=planet:DEPTH:SEED` opens it in the
editor (saved as `planet-DEPTH-SEED`, since Windows refuses colons in file
names; the game's replay and film names drop them the same way). The
window's title and opening alert name the planet: "Drevix, a canyon planet
(planet 3)". The start menu has **Random planet** after Campaign: a fresh
seed at the depth set by the new **Planet** option (1-8); in that round **N**
rolls a new planet at the same depth. The run thread's dev mode (pick the
planet, points, redraw) sets the depth the same way.

## Films

`larder-world map=planet:D:S seed=S look=DIR hand=idle at=1,10,...,120
screen=1280x800` on llvmpipe, planets 1-8, seeds 1 and 2: the whole map
(zoom 1) and the view near the squad, every 10 game seconds for two minutes
with nobody playing, so the ground's effect on the slime shows, and two
close clips near the squad (planets 3 and 6, 40-46 s). Page:
https://claude.ai/artifact/RYfaQD8SGMSMSuaKZtaD5P (copy in
`levelgen-2026-11-14/page/`, listed in [`pages.md`](pages.md); scripts and
logs in `raw/`).

Mould walkers at 120 s, seed 1 / seed 2 (thousands; the start blob is 25):
barrens 17 / 28, marsh 35 / 28, canyon 26 / 37, dunes 37 / 41, claypan
35 / 30, thicket 34 / 29, spires 43 / 33, core 42 / 43. Barren ground
visibly thins the slime into tubes; sand lets it spread. The squad at
the landing shot slime in 10 of the 16 films inside two minutes with
nobody playing: the shape C slime's explorers reach a long way, which is
the slime's tuning, not the map's.

## Round 2: layouts that read (the graphics lead's review)

The graphics lead reviewed round 1's films (16:19Z): "no, most biomes
don't read in a grey thumbnail; and yes, Barrens looks like plain ground.
In grey, Barrens, Canyon, Claypan and Spires are the same picture: dark
ground, mid-grey blobs and short grey sticks." Their fixes on this side, by
impact: one signature shape a biome at zoom 1; dune islands larger and
noise-edged; soft rock that wobbles and varies in thickness (straight gate
bars read as UI, perfect arcs as selection rings); keep the nests and
crystals as they are. The ground's materials (sand ripples, clay cracks,
barren ash, water gloss) are the terrain thread's shader work.

What changed, generator only:

- **Barrens:** three wide barren bands right across the route (at 0.28,
  0.5 and 0.72 of the way to the far nest, each about 0.12 of it wide),
  warped and ragged. Barren median 38% to 27%.
- **Marsh:** pools from one broad noise (cell 210 map cells), so they run
  together, on clay banks. Wet median 19% to 28%.
- **Canyon:** *fronts*, ridges right across the map at even steps between
  the landing and the far nest, about 50-80 cells wide, each with one or
  two passes (the second gated a quarter of the time); fewer, bigger
  regions (9) with steadier ridges (`ridge_vary` 0.5); no mesas. Rock 17%
  to 14%.
- **Dunes:** clay islands 2.4 crystal radii across (were 1.8), their
  edges pushed by a fine noise.
- **Claypan:** clay nearly everywhere, cracked into polygons of plain
  ground: the borders of a jittered grid's Voronoi cells, 110 map cells
  across, broken where a noise says.
- **Spires:** up to 210 small ragged pillars (radius 7-15), placed after
  the crystals. Placed before them, they left room for a median of 14
  crystals instead of 17.
- **Soft rock** (gates, rings, hedges, everywhere): chains of short pieces
  that wobble and vary in thickness (11-21 cells). Rings' radii wander by
  up to about 15%.

Same seeds filmed again (`raw/films-look2.sh`, `raw/clips-look2.sh`, logs
in `raw/films-look2/`). Mould walkers at 120 s with nobody playing, seed 1 /
seed 2 (thousands): barrens 21 / 34, marsh 32 / 25, canyon 28 / 32, dunes
34 / 38, claypan 28 / 38, thicket 34 / 31, spires 42 / 43, core 44 / 39.
Page: https://claude.ai/artifact/SJRBVoTF3yyEgfJvh6Da7t (copy in
`levelgen-2026-11-14/page2/`, made by `raw/make_gifs.py`,
`raw/make_grey.py`, `raw/make_page2.py` and `raw/layout_sheet.py`). In the
game's grey frames the ground kinds still differ little in value, so the
layout shows mostly through rock and soft rock; the page adds the layouts
drawn straight from the map files in false colour.

The owner, after seeing round 1 (16:50Z), asked for much more variety
between biomes (plants and water in many, open against rocky, crystal
patterns and sizes): [brainstorm](levelgen-2026-11-14/biomes-brainstorm.md),
questions in the thread.

## Next

1. **Biomes with far more variety**, from the owner's answers to the
   [brainstorm](levelgen-2026-11-14/biomes-brainstorm.md)'s questions.
2. **Planets grow** (the owner's 60% to 6x): `Map` carries its own size,
   `BigDish` and the GPU world take it from the map, the generator scales its
   regions, nests and crystals with area. 6x the area is 6x the cells for the
   engine and the soldiers' walking maps (each map 655 KB and about 15 ms
   to build at today's size), so the biggest size is timed on the 1070 first.
3. **Twists** shown before landing (Flooded, Rich crystals, ...).
4. **Rescue planets**, rare: base ruins and stranded soldiers as new map
   items, with a rescue goal from the run thread.
5. The owner's play.
