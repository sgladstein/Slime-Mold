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
- 16:50Z, after round 1's page: "we need way more variability between biomes. There should be some with plants and water in many of them. They should look really different terrain. Some are open, some have lots of rock making different areas, etc. think about how this affects strategy. There should be different crystal distributions and sizes" ([brainstorm](biomes-brainstorm.md)).
- 17:02Z, answers to the brainstorm's questions: plants "fully new from the old grass"; deep water that stops soldiers, "I like that idea"; dials and named biomes, "your rec for now"; the crystal pattern "rolls its own, but some biomes are more likely to get certain patterns"; "any maps should be beatable by any strategy"; open early and mazes later, "That makes sense"; "New ground. Yes!" ([plan](biomes-plan.md)).
- 17:24Z: "Do we need any additional research on world/level gen or have we done enough?" Answered: enough to build; one targeted read (how StarCraft map generators score chokepoints and fairness) when the every-strategy check is written.
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

Recipes over regions, not raw noise. Since round 3 a biome is a named range
on a few **dials**, and each planet rolls its value on each (`Recipe::of`,
from its own random stream, so a failed attempt redraws the layout, not the
kind of planet): openness, water, soil and crystal pattern.

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
4. **Rock, by openness.** *Open*: few ridges, no mesas. *Outcrops*: ridges
   and mesas. *Pillars*: many small ragged pillars (round 2's Spires).
   *Ridges*: long ridges right across the map with a pass or two (round
   2's Canyon fronts). *Rooms*: nearly every border a whole, wide ridge,
   joined by passes. *Caves*: rock everywhere but a cavern round each site
   and a tunnel along each passable border. A pass is a slot through the
   ridge plus a disc as wide as the widest ridge, cleared of every ridge's
   rock; a short open border on the spanning tree is cut the same way.
5. **Water.** *Pools* (round 2's Marsh), *lakes* with a deep middle,
   *rivers* right across the route with shallow fords and sand banks, or
   narrow wandering *streams*. Deep water is written as shallow wet until
   the ground thread's deep water (soil kind 4, which soldiers cannot stand
   in) is on main; the fords and rims are shallow already.
6. **Soil.** *Sand* patches, *dunes* (sand with clay islands under the
   crystals), barren *bands* across the route, barren everywhere but the
   *oases*, *clay* patches, *cracked* clay (a jittered grid's Voronoi
   borders) or *mixed*. Plain ground round the landing (150 cells), clay
   round the nests where the soil or water holds the slime at home, both
   with ragged edges.
7. **Crystals, by pattern.** Always a rich home crystal under each nest
   and a medium one 85-140 cells from the landing, in the landing's own
   region. Then *scattered* (big ones midway, small anywhere), *fields* (a
   big crystal ringed by three to five small ones), *veins* (lines of small
   crystals across the route), *jackpot* (one giant, radius 56-70, a few
   small), *oases* (a cluster at each oasis) or *richer farther* (radius
   12 to 40 and charge 96 to 512 rising toward the slime). None on rock or
   water.
8. **Soft rock.** Gates across some passes; on Jungle and Core a ring
   round each nest with its gap facing the landing (the Nest's shape), and
   curved hedges, kept off the passes. Each is a chain of short pieces that
   wobbles and changes thickness (11-21 cells), so it reads as grown.
9. **The checks** (`levelgen::check`), on a 4-cell walking grid narrowed by
   one cell so a pass a squad can't get through counts as shut: every
   crystal and nest reachable from the landing (breaking soft rock allowed);
   the far nest at least 0.6 of the map's width away on foot and every nest
   0.45; a crystal within 200 cells' walk of the landing; half the charge
   reachable without breaking soft rock; rock under 30% (40% for ridges and
   rooms, 72% for caves); every nest placed. A failed attempt is rebuilt
   from the next attempt's stream (24 tries, then an open plan with no
   ridges, which no seed has needed).

## The eight biomes, in the set order (round 3)

From the owner's answers (17:02Z): dials plus named biomes, the crystal
pattern rolled with biome odds, open and readable early, mazes and caves
later ([plan](biomes-plan.md)). Round 2's seven shapes are dial values now.
The ground kinds are the ones the simulation already plays
([larder maps, *Which ground the mould feels*](../larder-maps-2026-11-11.md)):
wet and barren ground halve the slime, sand spreads it (about 60% bigger),
clay holds it in place. Weights in brackets.

| Planet | Biome | Openness | Water | Soil | Crystal pattern |
| --- | --- | --- | --- | --- | --- |
| 1 | Meadow | open (3), outcrops (2), pillars (1) | pools (2), dry, streams | plain (2), sand, clay | scattered (3), fields (3), richer (2), veins, jackpot |
| 2 | Barrens | open (2), outcrops (2), pillars (1) | dry | bands or oases | oases with oasis soil; else scattered (2), veins (2), richer (2), jackpot |
| 3 | Fen | open, outcrops | lakes, pools | clay (2), plain | veins (3), fields (2), scattered (2), richer |
| 4 | River valley | ridges (2), outcrops (1) | rivers (one, or two two times in five) | dunes (2), sand | fields (3), veins (2), richer (2), scattered |
| 5 | Jungle | outcrops (2), open (1) | streams (2), pools (1) | clay (2), plain | scattered (3), fields (2), jackpot, richer; nine hedges, rings, gates |
| 6 | Highlands | rooms (3), ridges (1) | dry (2), pools (1) | cracks (2), clay | fields (3), jackpot (2), richer (2), scattered |
| 7 | Caves | caves | pools (2), dry | plain, clay, sand | veins (3), jackpot (2), scattered (2), fields |
| 8 | Core | outcrops, pillars, ridges, rooms | any | mixed | any; three nests, rings and hedges |

Past planet 8 the order repeats (for longer runs). Passes narrow 3% a planet
to a floor of 70%.

## Numbers: 400 planets (round 2)

`cargo run --release --example planets -- seeds=1-50 depths=1-8`
([raw](raw/planets-seeds-1-50-look2.txt); seeds 1-4
alone: [raw](raw/planets-seeds-1-4-look2.txt); the first
layouts' numbers: [raw](raw/planets-seeds-1-50.txt)).
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
patterns and sizes): [brainstorm](biomes-brainstorm.md),
questions in the thread.

## Bigger planets: maps carry their own size

The owner's sizes (15:33Z) are `AREA`: 0.6, 0.8, 1, 1.4, 2, 2.8, 4 and 6
times today's 1024 x 640 for planets 1-8. A map now carries its own size:
`Map { w, h, .. }` (any multiple of 4 from 64 to 4096 map cells),
`BigParams::dish` (today's 256 x 160 dish cells unless a map says
otherwise), the GPU world, the camera and the editor all take it from the
map. The generator scales with the area: regions, small crystals and hedges
with the area, big crystals with its square root, so a bigger planet has
the same density and a longer route.

**Held at today's size above 1x** (`GROW_CAP` 1.0) until the 1070 has timed
the bigger worlds, as the technical lead asked (16:42Z): planets 1 and 2
are smaller now (800 x 496 and 912 x 576), planets 3-8 today's size.
`map=planet:D:S:AREA` asks for any area to 6 for that timing and for dev
play (`larder-world map=planet:8:1:6`).

**GPU memory, measured.** `larder-world -- bench` now prints the bytes of
every storage buffer the engine, the view, the canvas and the ground look
make (`gpu::buffer_bytes`, counted at creation; screen-sized textures not
in it). Seed 1, default settings (25,000 walkers, 50,000 slots, scale 2) on
llvmpipe ([raw](raw/sizes-bench-llvmpipe.txt)):

| Planet (area) | Map cells | Dish fine cells | GPU buffers | Biggest buffer |
| --- | --- | --- | --- | --- |
| 1 (0.6x) | 800 x 496 | 400 x 248 | 71.6 MB | 8.4 MB |
| 3 (1x, today) | 1024 x 640 | 512 x 320 | 84.1 MB | 8.4 MB |
| 5 at 2x | 1456 x 912 | 728 x 456 | 116.6 MB | 8.4 MB |
| 7 at 4x | 2048 x 1280 | 1024 x 640 | 179.1 MB | 11.1 MB |
| 8 at 6x | 2512 x 1568 | 1256 x 784 | 234.9 MB | 16.4 MB |

A straight line through the five rows: about 54 MB that does not grow
with the map, plus 184 bytes a fine cell. The core world at 6x needs 235 MB of the 1070's 8 GB, and its biggest
buffer is 16 MB, far under the 128 MiB a storage binding may be by default
(the game asks for the adapter's own limits, which are higher). Memory does
not stop 6x; the step's and the draw's time per frame are what the 1070
timing measures. The player's own mould adds per-cell blocks; it was not in
these runs.

**Generation time.** The region scan was the cost at 6x (1.1 s): a cell
now looks only at the sites that can be nearest inside its 16 x 16 tile
(each tile's warped box, padded), in the same order, so it finds the same
site as the full scan. Planets 3-8 at today's size kept their pinned
fingerprints exactly through the change. Planet 8 at 6x builds in about
0.5 s, once a level.

**Seen.** Planet 1 and planet 8 at 6x play headless with nobody playing
(30 s, no errors). On planet 1 the same 25,000 walkers cover about a third
of the smaller map by 30 s; on the 6x core world they are a corner of it.
Walkers per planet are the run thread's (the slime's tier), so they scale
there.

**Still to do before the default grows:** the 1070 timing (1x, 2x, 4x, 6x;
step and draw ms), and the technical lead's other point, that only the
grown region should cost (live bounds or tiles), if the timing shows the
big worlds' empty cells cost frames.

## Round 3: biomes that play differently

From the owner's answers to the [brainstorm](biomes-brainstorm.md) (17:02Z)
and the [plan](biomes-plan.md): the eight biomes above, each a named range on
the dials, in a new order (open early, mazes late). New in the generator:
rocky rooms, caves (caverns and tunnels), lakes, rivers with fords, streams,
oases, and six crystal patterns rolled with each biome's odds. Plants (the
plants thread's new field) and deep water and other new ground (the ground
thread's) are written into the maps when each is on main: plants in the
PNG's B channel (kind in the top two bits, density in the low six), deep
water as soil kind 4 with the wet bit, a bridge as kind 5.

**Numbers: 400 planets.** `cargo run --release --example planets --
seeds=1-50 depths=1-8` ([raw](raw/planets-seeds-1-50-look3.txt); the survey
now prints each planet's dials). Medians over 50 seeds, ranges in brackets.

| Biome | Crystals | Rock | Sand | Clay | Barren | Wet | Soft-rock pieces | Walk to nearest nest | Walk to nearest crystal | Charge open without breaking soft rock | First attempt passes |
| --- | --- | --- | --- | --- | --- | --- | --- | --- | --- | --- | --- |
| meadow | 11 (6-18) | 2% | 0% | 22% | 0% | 8% | 8 | 563 (488-632) | 57 | 100% | 45/50 |
| barrens | 14 (9-20) | 5% | 0% | 5% | 51% | 0% | 0 | 676 (548-798) | 75 | 100% | 44/50 |
| fen | 15 (6-22) | 3% | 0% | 35% | 0% | 26% | 0 | 781 (620-950) | 64 | 100% | 50/50 |
| river valley | 15 (10-22) | 10% | 37% | 29% | 0% | 5% | 0 | 595 (484-917) | 67 | 100% | 49/50 |
| jungle | 17 (11-17) | 4% | 0% | 33% | 0% | 6% | 130 | 587 (486-796) | 60 | 100% | 47/50 |
| highlands | 12 (11-17) | 16% | 0% | 57% | 0% | 0% | 4 | 654 (488-1052) | 64 | 100% | 42/50 |
| cave | 11 (6-16) | 50% | 0% | 18% | 0% | 4% | 12 | 597 (490-909) | 54 | 100% | 44/50 |
| core | 15 (10-21) | 9% | 12% | 34% | 4% | 5% | 82 | 536 (483-777) | 62 | 100% | 33/50 |

- Every one of the 400 passed its checks within four attempts, 354 on the
  first; none used the fallback. 75 ms a planet (median; at most 408 ms).
- The patterns came up as weighted: on the 50 Barrens, 27 oases worlds
  (barren 50-67% of the ground) and 23 with bands; on the 50 cave planets, 22 veins,
  16 jackpots, 11 scattered, 1 fields.
- Planet 1 now has 6-18 crystals: a jackpot meadow has its giant and few
  others, by design. The check's floor stays six.

**Tried and changed.**
- *Rocky rooms walled the landing in.* With gates at 0.2 (0.35 on the core
  world, with rings round the nests), 14 of 100 highlands and core worlds
  (seeds 1-50) took 4 to 19 attempts and one used the fallback (2.0 s).
  Three causes, each seen on a rendered failing attempt: a pass slot,
  square to the line between the two sites, ran along a ridge the warp had
  bent instead of through it; a neighbouring ridge reached into a slot at a
  corner; a short open border on the spanning tree (where three regions
  meet) was shut by its neighbours' ridges. Now every pass, and every open
  tree border, is cut through every ridge's rock with a disc as wide as the
  widest ridge; rooms gate a pass at 0.08 (core rooms 0.1, no rings);
  hedges keep 30 cells off passes; the landing's crystal is in the
  landing's own region. Then all 100 passed within four attempts, 75 first.
- *Veins started off the map.* A vein began at a random point and stopped
  at the map's edge, leaving 6-7 crystals on a planet; it now grows both
  ways from a point midway (9-22 crystals).
- *Caves too open.* Caverns 0.45-0.65 of the site spacing left 28-36%
  rock, more like open ground with holes; at 0.32-0.5, tunnels 0.6 of a
  pass, 50% rock (cap 72% for caves).

**Seen** ([page](https://claude.ai/artifact/8YTtSxUEGnXivr6VejCaNU), source
`page3/`): films of planets 1-8, seeds 1 and 2, two minutes with nobody
playing (`raw/films-look3.sh`, logs in `raw/films-look3/`), each with the
dials it rolled; and a contact sheet of seeds 1-3 of every planet in false
colour and grey, straight from the map files (`raw/biome_sheet.py` on
[`raw/planets-seeds-1-3-look3.txt`](raw/planets-seeds-1-3-look3.txt)).
Each biome reads apart at a glance on the sheet: meadow streams, barren
bands, fen lakes, the valley's river, the jungle's hedges and rings, the
highlands' rooms on cracked clay, the caverns, the core's mix.

**After the graphics lead's review (18:56Z).** Round 3 accepted; the
river and the streams too thin to read at zoom 1: rivers 10-14 cells of
the 256 x 160 grid wide (4 map cells each), streams at least 4. Rivers now at least 40
map cells across (half-width 22-28, edge noise ±10%; were 34 at the
least), streams at least 16 (half-width 9-12; were 8). Width across each
water's own course, medians on seeds 1-3 (`raw/medial.py`): river valley
37-47 cells before, 39-49 after; streams 13-17 before, 19-25 after. The
checks are unchanged (400 planets: the same 354 first, all within four;
water never blocks a walk). The films and sheets above are from before the
change. Water's value against the ground is the ground thread's.

## Plants, deep water and 2x planets

**Plants** (owner 19:10Z, "You have a meadow biome with no green"). The
plants thread's field (#290) reads the map's B byte: kind in the top two
bits, density in the low six. Levelgen now writes each biome's (`Sow`) on
one canopy noise about 140 cells across, so patches thicken from the
edge in (the graphics lead's density gradients):

| Biome | Plants |
| --- | --- |
| meadow | grass over three quarters of it, a few jungle copses |
| barrens | thin scrub where the ground isn't barren; lush grass and copses in the oases |
| fen | reed beds round every pool and lake (18 cells), grass between |
| river valley | reeds along the banks (14), scrub on the dunes |
| jungle | canopy over most of it, grass in the clearings |
| highlands | sparse grass in the rooms |
| cave | a little growth in the caverns, reeds at the pools |
| core | grass, canopy and scrub mixed |

Reeds reach a third as far into the water as onto land (a full fringe hid
the rivers' shallows). Nothing on deep water, bridges, barren ground, a
crystal, or a ragged pad round the landing. The fingerprints now include
the plants.

**Deep water and bridges** (the ground thread's #285, merged into this
branch): river and lake middles are deep (`G_DEEP`), and half the rivers
get a bridge 12-16 cells wide away from the fords. The walk checks use
`map::standable`. On the first try deep water cut the squad off on 29 of
400 planets (one core world, rocky rooms and rivers, took 23 attempts and
the fallback, 4.6 s). `cut_fords` now makes the shortest way to each
cut-off crystal or nest shallow, 32 cells wide, as passes are cut through
ridges. Then 358 of 400 first, all within five attempts, none fall back
([raw](raw/planets-seeds-1-50-plants.txt)).

**2x planets.** The 1070 timing the technical lead asked for
([raw](raw/sizes-1070/), PR #284's head, seed 1, rush hand, 25k walkers;
runs 1 and 8 at low clocks):

| Area | GPU step (timestamps) | Drawing | Frame p95 | Prepare max |
| --- | --- | --- | --- | --- |
| 1x | 0.63 ms | 0.08 ms | 3.1-3.3 ms | 4.6 ms |
| 2x | 0.97 ms | 0.10 ms | 2.7-3.7 ms | 8.4 ms |
| 4x | 1.50 ms | 0.17 ms | 3.3-4.5 ms | 23.6 ms |
| 6x | 2.11 ms | 0.24 ms | 3.7-5.2 ms | 32.9 ms |

The prepare spike is `refresh_ground`: once a game second it reads back
every cell's charge and rebuilds every cell's ground code on the CPU, a
dropped frame or two a second from 4x. `GROW_CAP` 1.0 -> 2.0: planets 1-5
at the owner's sizes (0.6, 0.8, 1.0, 1.4, 2.0), 6-8 at 2x until the
refresh touches only the cells that change (next). Build time with
everything: 172 ms median, 1.2 s at most.

## A sparse ground refresh (20:00-21:00Z)

The technical lead's fix for the 2x cap: the timed ground refresh now
reads and rebuilds only the cells whose code can change between walls
going up, the crystals' (charge) and the walls' (health). At world start
and whenever a wall goes up it still rebuilds every cell, as before, and
notes the crystal and wall cells (`live_cells`). Once a game second a
compute pass gathers those cells' charge and health into one small buffer,
one copy reads it back (`gpu::Gather`), `live_code` rebuilds those codes
as `ground_codes` would, and only the runs of codes that changed are
written. The look's own bits are kept (a crystal is never hard terrain,
and a wall stays hard as it wears to rubble, so no cell's `FAR` mark moves).

**Same picture.** 16 headless frames (planet 5 seed 1 rushing at 3, 9 and
20 s; planet 6 seed 3 pressing every command at 5 and 15 s; the walled Nest
map pressing every command at 6, 18 and 30 s; whole map and close up) from
the merged code with and without the change: 15 byte-identical. The one
other differs only in the top-left strip where the planet's toast fades by
the wall clock, and two runs of the old build differ there too; it matches
one of them exactly. A test (`the_sparse_ground_refresh_matches_a_whole_one`)
plays the Nest map 16 game seconds with a worn and a broken wall cell set
by hand, and every refresh's codes equal a whole rebuild from the same
charge and health (6 refreshes on software Vulkan; the broken cell draws as
rubble; crystals were eaten).

**Timing** (software Vulkan in the cloud, so a CPU playing GPU; planet 8
seed 1 at the area shown, `bench=20`, rush hand; the bench now prints the
refresh's own time and its read-back, and the frame, step and prepare maxima
after the first game second; [raw](raw/refresh-llvmpipe/README.md)):

| Area | Live cells | Prepare max, old | A copy a row | Gathered | Refresh max | Its read-back |
| --- | --- | --- | --- | --- | --- | --- |
| 4x | 26,371 | 25.0 ms | 14.2-14.5 ms | 3.40 ms | 2.64 ms | 1.44 ms |
| 6x | 39,969 | 31.9-32.2 ms | 21.1-22.2 ms | 3.75 ms | 2.84 ms | 1.31 ms |

**Rejected: a copy a row** (`copy_buffer_to_buffer` for each run of
neighbouring cells, 1,801 runs at 4x and 2,914 at 6x): its read-back alone
took up to 12.8 and 20.8 ms here, nearly all of the refresh. The gather is
one dispatch and one copy whatever the cells' shape.

**Still to do** (then): a wall going up still read and rebuilt every cell,
and planets stayed at 2x until the 1070 timed this build. Both are in
*Walking maps at 6x* below.

**The river look in grey.** The ground thread's river look (#292) and the
plants reached main, so the graphics lead's thumbnail test again, seed 1 of
planets 1-8 at 1 s, 266 x 164 each (`raw/make_grey3.py`):
`page3/grey/sheet-water.png` and `sheet-water-colour.png`.

## Walking maps at 6x (21:30-22:30Z)

The 1070 rerun of the sparse refresh (the PC runner, head 36ad7613)
cleared the frame's prepare (2.0-2.6 ms at every size) and showed what was
left: a game step's max after the first second, 22, 70, 94 and 210 ms at
1x, 2x, 4x and 6x, the same with the walkers' spread off. It was the
soldiers' walking maps. A map is the whole world, built the step a soldier
first needs it; it tested every crystal at every cell (`standable` walks
the crystal list) and ran a binary-heap Dijkstra, and every soldier's
formation slot is its own goal, so a squad's new order cost a map a
soldier in one step. The technical lead's plan, in parts, each timed at 6x
(planet 8 seed 1, `bench=10 spread=0`, software Vulkan in the cloud, so the
GPU's frame runs on the same four cores; game-step max over the frames
that ran a step):

| Part | One map at 6x (alone) | Rushing hand | Every-command hand |
| --- | --- | --- | --- |
| before | about 167 ms | 186 ms | 34 ms |
| an open-cell grid and a bucket queue | 28 ms | 44 ms | 54 ms |
| a slice a step, shared and stale maps | 4-5 ms a step | 12 ms | 18 ms |

- **The grid** (`standable_cells`): the world's open cells in one pass
  (`World::open_cells`; the dish answers from its own arrays), then each
  solid crystal's disc cleared over its own box by `solid_at`'s own test,
  so the same cells (`standable_cells_are_standable_at_each_middle`, deep
  water included): 104 ms per cell before, 7.9 ms, 1.9 ms with the dish's
  grid.
- **The bucket queue** (`MapBuild`): steps cost 5 or 7, so a ring of 8
  buckets holds every distance still to take and no heap is needed: the
  heap's own distances on random mazes, sliced or whole
  (`the_walking_map_ring_matches_the_heap`), in 26-28 ms against 62-64.
- **A slice a step** (`Game::pump_maps`, once at the start of a step):
  131,072 cells (`MAP_POPS`) across the maps being built, at most one
  started a step, asked goals first; a 1x map is one step or two, a 6x
  planet's six or seven. A soldier or squad asks (`Game::map_for`) and
  waits while its map is built. A built map whose goal is within 4 dish
  cells and a straight walk from the asker's goal serves it, so a squad's
  slots share one map (a steered squad's moving point takes one half a
  lead off). An ask not renewed by the next step is dropped.
- **Stale, not dropped**: a wall going up, and every `PATHS_EVERY` (30 s),
  makes every map stale; each is rebuilt in turn, walked last first, and
  walked meanwhile, where before all were dropped at once and every
  soldier built again in the same step. The cache keeps 48 maps or 16M
  cells (64 MB) and drops the one walked longest ago; it started again
  from empty at 48 before.

**Rejected: one whole map a step** (`MAPS_A_STEP` 1): the rushing hand's
max stayed at 44 ms, one 6x map's worth, over a frame. A grid kept between
maps was not built: the solid discs shrink with every bite of a crystal, so
it would rarely be the same twice; each build makes its own (1.9 ms at 6x).
The every-command hand's 119 ms frames are its commands, not a step: a wall
or building reads the dish back at once, which waits on the GPU's frame,
about 110 ms on software Vulkan and about 2 ms on the 1070.

**Walls without the whole ground.** A wall going up rebuilt every cell's
ground code (the 25-32 ms hitch at 6x). Now its cells join the gathered
ones and only the 32-cell tiles it touches are re-marked, each grown by
the `FAR` mark's reach (`look::mark_far_box`, prefix sums over the box,
the world's edge as hard). The sparse refresh's test now raises three
walls at 6 s (across a crystal, on open ground, along the world's edge)
and every refresh still equals a whole rebuild.

## Plants that tell the biomes apart (22:00-22:40Z)

The plants thread's thumbnails showed almost every planet green edge to
edge, Barrens, Cave and Highlands too (graphics lead, 21:56Z: plants should
help tell biomes apart, cover from about 5% to 80%). Two `Sow` dials:
`clumps` sows on the fine noise (clumps about 20 map cells across) instead
of the canopy's 140-cell patches, and `soils` names the soils anything but
reeds grows on. Share of all map cells (rock included) with plants, seeds
1-3, main at 8941373 against this branch (`examples/planets` maps; the test
`plants_tell_the_biomes_apart` holds the new spread):

| Biome | Before | After | What grows |
| --- | --- | --- | --- |
| meadow | 76.6% | 76.4% | grass over three quarters, a few jungle copses |
| barrens | 44.0% | 10.6% | scrub in sparse clumps; oases lush |
| fen | 64.0% | 63.7% | reed beds round every pool and lake, grass between |
| river valley | 77.8% | 77.6% | reeds along the banks, scrub on the dunes |
| jungle | 79.4% | 79.1% | canopy over most of it, grass in the clearings |
| highlands | 22.8% | 19.3% | grass in patches on the clay only, reeds at pools |
| cave | 8.7% | 4.1% | reeds at the pools and a clump or two |
| core | 68.2% | 67.8% | grass, canopy and scrub mixed |

**Edges that fade** (terrain look thread, 22:15Z: at zoom 24 the Fen and
Jungle beds still showed 1-cell steps every ~7 cells, from the map's own
data). The view draws a patch's edge where its density crosses 0.2
between cells; the generator stepped from 0.25-0.4 to nothing at a patch's
edge, so the line ran along the cells' own staircase. Every edge now fades
to nothing over a few cells: a patch over 0.06 of the canopy noise (about
6 map cells) or 0.15 of the fine one, the reeds over their last 4 map cells
(a fringe into the water over its outer half), the clearings round
crystals and the landing over 6. Where reeds meet grass or grass meets
jungle both fade, leaving a seam about a game cell wide. Drawn from the
map data at zoom 12 (`raw/plant_edges.py`), the reed fringe along the
Fen's ridges and the jungle beds' borders run smooth where they stepped.

**Moisture.** Each planet's map now carries its biome's `moisture BASE
SPREAD` line (#295, agreed with the plants thread): meadow 0.25 0.3,
barrens 0 0.2, fen 0.5 0.3, river valley 0.3 0.3, jungle 0.4 0.3,
highlands 0.1 0.2, cave 0.35 0.2, core 0.2 0.35 (`Biome::moisture`).

## Next

1. **Planets to the owner's 6x** once the 1070 has timed the walking maps;
   a three-agent adversarial review (owner, 19:12Z and 19:32Z); the core
   world's few large open arenas with hedges round the nests (graphics
   lead).
2. **Every strategy can win**: the check that each nest has an open
   approach and a choke, after a short read on how StarCraft map
   generators score chokepoints and fairness (owner's question, 17:24Z).
3. **Twists** shown before landing (Flooded, Rich crystals, ...).
4. **Rescue planets**, rare: base ruins and stranded soldiers as new map
   items, with a rescue goal from the run thread.
5. The owner's play.
