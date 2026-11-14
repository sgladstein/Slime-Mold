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
   is a slot through the ridge. Mesas and, on Spires, small pillars in the
   fields, never within 120 cells of a site.
5. **Soil and water.** Noise patches per biome, independent of the regions;
   plain ground round the landing (150 cells), clay round the nests on the
   biomes that hold the slime at home, as the designed maps do.
6. **Crystals.** A rich home crystal under each nest (the slime starts on
   it), a medium one 85-140 cells from the landing toward the middle, one to
   three big contested ones midway, small ones scattered; none on rock or
   wet ground.
7. **Soft rock.** Gates across some passes; on Thicket and Core a ring
   round each nest with its gap facing the landing (the Nest's shape), and
   curved hedges.
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
| 1 | Barrens | Bands of barren ground between you and the slime | Starves it: the easy first planet |
| 2 | Marsh | Wet pools on clay banks | Slows it |
| 3 | Canyon | Wide rock ridges with narrow passes | Chokepoints |
| 4 | Dunes | Sand everywhere, clay islands under the crystals (the Flats' idea) | The slime races; first planet with two nests |
| 5 | Claypan | Mostly clay | The slime digs in and is slow to leave |
| 6 | Thicket | Soft-rock hedges and walled nests | Walls to chew through, rings at home |
| 7 | Spires | Rock pillars and mesas on sand and clay | Cover the slime grows behind (the tiers page's "grows round cover" idea), so the squad has to move |
| 8 | Core | Every ground, three nests, rings and gates | The core world |

Past planet 8 the order repeats (for longer runs). Passes narrow 3% a planet
to a floor of 70%.

## Numbers: 400 planets

`cargo run --release --example planets -- seeds=1-50 depths=1-8`
([raw](levelgen-2026-11-14/raw/planets-seeds-1-50.txt); seeds 1-4 alone:
[raw](levelgen-2026-11-14/raw/planets-seeds-1-4.txt)). Medians over 50 seeds
each, ranges in brackets; walks in map cells (the map is 1024 wide).

| Biome | Crystals | Rock | Sand | Clay | Barren | Wet | Soft-rock walls | Walk to nearest nest | Walk to nearest crystal | Charge open without breaking soft rock | First attempt passes |
| --- | --- | --- | --- | --- | --- | --- | --- | --- | --- | --- | --- |
| barrens | 16 | 6% | 0% | 0% | 38% | 0% | 1 | 794 (643-942) | 62 (51-140) | 100% (72-100%) | 45/50 |
| marsh | 16 | 4% | 0% | 30% | 0% | 19% | 1 | 794 (640-956) | 60 (47-91) | 100% (97-100%) | 50/50 |
| canyon | 16 | 17% | 26% | 0% | 0% | 0% | 2 | 882 (661-1144) | 68 (48-193) | 100% (70-100%) | 43/50 |
| dunes | 22 | 3% | 49% | 30% | 0% | 0% | 0 | 578 (484-770) | 62 (48-106) | 100% (100-100%) | 48/50 |
| claypan | 17 | 7% | 0% | 70% | 1% | 0% | 1 | 610 (493-860) | 68 (50-109) | 100% (73-100%) | 50/50 |
| thicket | 17 | 5% | 0% | 46% | 0% | 0% | 80 | 587 (488-900) | 66 (49-120) | 100% (56-100%) | 45/50 |
| spires | 17 | 7% | 28% | 21% | 0% | 0% | 0 | 545 (485-906) | 60 (48-108) | 100% (87-100%) | 49/50 |
| core | 21 | 8% | 14% | 32% | 5% | 7% | 61 | 542 (478-652) | 56 (44-88) | 96% (54-100%) | 36/50 |

- Every one of the 400 planets passed its checks within five attempts; 366
  on the first. None used the fallback.
- Building a planet takes 55 ms (median; at most 295 ms with retries) on
  one cloud core while four films ran beside it. It runs once a level,
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

## Next

1. **Planets grow** (the owner's 60% to 6x): `Map` carries its own size,
   `BigDish` and the GPU world take it from the map, the generator scales its
   regions, nests and crystals with area. 6x the area is 6x the cells for the
   engine and the soldiers' walking maps (each map 655 KB and about 15 ms
   to build at today's size), so the biggest size is timed on the 1070 first.
2. **Twists** shown before landing (Flooded, Rich crystals, ...).
3. **Rescue planets**, rare: base ruins and stranded soldiers as new map
   items, with a rescue goal from the run thread.
4. The graphics lead's review of the look (each biome readable in a grey
   thumbnail) and the owner's play.
