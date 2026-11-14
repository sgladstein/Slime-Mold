---
title: Planets (the roguelike run's levels)
kind: design
status: open
updated: 2026-11-14
sources:
  - Reports/larder-maps-2026-11-11.md
  - Reports/levelgen-2026-11-14/README.md
code:
  - Biome::ORDER
  - CORE_DEPTH
  - Recipe::of
  - StartEvent::RandomPlanet
  - levelgen::check
  - levelgen::level
  - levelgen::planet
  - levelgen::spec
---

# Planets (the roguelike run's levels)

## Now

Each level of the roguelike run is a generated planet (`mould::levelgen`): `level(seed, depth)` is a map in the designed maps' format, so the game and the editor open it like any map, and `planet(seed, depth)` is its card (name, biome, size). Eight biomes come in a set order, one a planet: Barrens, Marsh, Canyon, Dunes, Claypan, Thicket, Spires, and the core world eighth (`CORE_DEPTH`); each is made of ground the simulation already plays (barren and wet starve and slow the slime, sand spreads it, clay holds it, rock and soft rock wall it). A planet is regions round its landing, nests and fields, with ridges and passes across the route; one nest to planet 3, two from 4, three on the core world. Since the graphics lead's review each biome has one signature shape at zoom 1: barren bands across the route, big pools, canyon ridges right across the map, ragged dune islands, cracked clay, hedges and rings that wobble, small pillars ([round 2 page](https://claude.ai/artifact/SJRBVoTF3yyEgfJvh6Da7t)). Every planet is checked before it is played: every crystal and nest reachable, a crystal near the landing, the far nest across the map on foot; all of 400 planets passed within four attempts, in 65 ms each (median), once a level ([report, *Numbers: 400 planets*](../../Reports/levelgen-2026-11-14/README.md)). The same seed and depth give the same planet on every machine (no platform maths). Play one with `larder-world map=planet:3:42`, or Random planet in the start menu (the Planet option picks the depth; N rolls another); every planet is still today's map size. Films of planets 1-8, two seeds each, two minutes with nobody playing: [page](https://claude.ai/artifact/RYfaQD8SGMSMSuaKZtaD5P).

## Owner's word

- 2026-11-14 15:25Z: "the slime is taking over planets and each level is a new planet as we move towards the core slime world. This is just head cannon right now, but it could guide our level design" (this thread).
- 2026-11-14 16:50Z: "we need way more variability between biomes. There should be some with plants and water in many of them ... Some are open, some have lots of rock making different areas ... different crystal distributions and sizes" ([brainstorm](../../Reports/levelgen-2026-11-14/biomes-brainstorm.md); questions open).
- 2026-11-14 15:33Z: "first planet about 60% of today's map, today's size by planet 3 or 4, up to 6 the area for the core world"; biomes in a "Set order"; the designed maps "will not come at all" in a run; rare planets with "remains of bases or even start with some soliders" ([report](../../Reports/levelgen-2026-11-14/README.md), *The owner's word*).

## Tried and rejected

- Round 1's layouts (noise patches of barren, small pools, short ridge stubs and mesas on the canyon, round dune islands, straight gate bars): in grey four biomes were the same picture (graphics lead, 16:19Z); replaced by one signature shape a biome ([report, *Round 2*](../../Reports/levelgen-2026-11-14/README.md)).
- A pass as a disc on the ridge's border: wherever the ridge ran wider than the disc, rock closed it (canyon seeds 2 and 3 needed 13 and 6 attempts); a pass is now a slot through the ridge's widest ([report](../../Reports/levelgen-2026-11-14/README.md), *Numbers*).

## Open

- Far more variety between biomes (plants from the engine's grass, water, open against rocky, crystal patterns and sizes): the owner's answers to the [brainstorm](../../Reports/levelgen-2026-11-14/biomes-brainstorm.md).
- Planets that grow, 60% of today's map to 6x the area on the core world: maps carry their own size, then a timing run on the 1070 at the biggest.
- Twists shown before landing; rare rescue planets (base ruins, stranded soldiers) with the run thread's rescue goal.
- The owner's play.

## Sources

- [Procedural planets](../../Reports/levelgen-2026-11-14/README.md).
- [Larder maps](../../Reports/larder-maps-2026-11-11.md), *Which ground the mould feels* (what each ground does).
