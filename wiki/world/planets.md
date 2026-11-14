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
  - GROW_CAP
  - Openness
  - Pattern
  - Recipe::of
  - Soil::Oases
  - StartEvent::RandomPlanet
  - Water::Rivers
  - gpu::buffer_bytes
  - levelgen::check
  - levelgen::level
  - levelgen::planet
  - levelgen::size_at
  - levelgen::spec
---

# Planets (the roguelike run's levels)

## Now

Each level of the roguelike run is a generated planet (`mould::levelgen`): `level(seed, depth)` is a map in the designed maps' format, so the game and the editor open it like any map, and `planet(seed, depth)` is its card (name, biome, size). Eight biomes come in a set order, open early and mazes late: Meadow, Barrens, Fen, River valley, Jungle, Highlands, Caves, and the core world eighth (`CORE_DEPTH`). Each biome is a named range on four dials, rolled per planet with the biome's odds: openness (`Openness`: open, outcrops, pillars, ridges, rocky rooms, caves), water (`Water`: dry, pools, lakes, rivers with fords, streams), soil (`Soil`: plain, sand, dunes, barren bands, oases in barren, clay, cracked clay, mixed), and the crystal pattern (`Pattern`: scattered, fields, veins, one jackpot, at the oases, richer toward the slime). Everything is ground the simulation already plays (barren and wet starve and slow the slime, sand spreads it, clay holds it, rock and soft rock wall it). A planet is regions round its landing, nests and fields, with ridges and passes across the route; one nest to planet 3, two from 4, three on the core world. Every planet is checked before it is played: every crystal and nest reachable, a crystal near the landing, the far nest across the map on foot; all of 400 planets passed within four attempts (354 first), in 75 ms each (median), once a level, rock from 2% (meadow) to 50% (caves) of the ground; rivers at least 40 map cells across and streams 16, so they read at zoom 1 (graphics lead, 18:56Z) ([report, *Round 3*](../../Reports/levelgen-2026-11-14/README.md)). The same seed and depth give the same planet on every machine (no platform maths). Play one with `larder-world map=planet:3:42`, or Random planet in the start menu (the Planet option picks the depth; N rolls another). A map carries its own size: planets 1 and 2 are 60% and 80% of today's area, planets 3-8 today's until the 1070 has timed the bigger ones (`GROW_CAP`); `map=planet:D:S:AREA` plays any area to 6x, whose GPU buffers measure 235 MB ([report, *Bigger planets*](../../Reports/levelgen-2026-11-14/README.md)). Deep water (soldiers can't wade, the slime crawls through) is written as plain water until the ground thread's soil kind 4 is on main; plants go into the maps' B channel when the plants thread's field is. Pages: [round 1 films](https://claude.ai/artifact/RYfaQD8SGMSMSuaKZtaD5P), [round 2](https://claude.ai/artifact/SJRBVoTF3yyEgfJvh6Da7t), [round 3](https://claude.ai/artifact/8YTtSxUEGnXivr6VejCaNU).

## Owner's word

- 2026-11-14 15:25Z: "the slime is taking over planets and each level is a new planet as we move towards the core slime world. This is just head cannon right now, but it could guide our level design" (this thread).
- 2026-11-14 17:02Z, answers to the brainstorm: plants "fully new from the old grass"; deep water that stops soldiers, "I like that idea"; dials and named biomes; the crystal pattern "rolls its own, but some biomes are more likely to get certain patterns"; "any maps should be beatable by any strategy"; open early, mazes later; "New ground. Yes!" ([plan](../../Reports/levelgen-2026-11-14/biomes-plan.md)).
- 2026-11-14 16:50Z: "we need way more variability between biomes. There should be some with plants and water in many of them ... Some are open, some have lots of rock making different areas ... different crystal distributions and sizes" ([brainstorm](../../Reports/levelgen-2026-11-14/biomes-brainstorm.md)).
- 2026-11-14 15:33Z: "first planet about 60% of today's map, today's size by planet 3 or 4, up to 6 the area for the core world"; biomes in a "Set order"; the designed maps "will not come at all" in a run; rare planets with "remains of bases or even start with some soliders" ([report](../../Reports/levelgen-2026-11-14/README.md), *The owner's word*).

## Tried and rejected

- Round 1's layouts (noise patches of barren, small pools, short ridge stubs and mesas on the canyon, round dune islands, straight gate bars): in grey four biomes were the same picture (graphics lead, 16:19Z); replaced by one signature shape a biome ([report, *Round 2*](../../Reports/levelgen-2026-11-14/README.md)).
- Round 2's one signature shape a biome (barren bands, big pools, canyon fronts, dune islands, cracked clay, hedges, pillars): each biome looked the same every time and every biome played open ground with walls; replaced by the dials, rolled with biome odds (owner, 16:50Z and 17:02Z; [report, *Round 3*](../../Reports/levelgen-2026-11-14/README.md)).
- Rocky rooms with gates at 0.2, rings round the core world's nests, and pass slots cut only through their own ridge: 14 of 100 highlands and core planets needed 4-19 attempts, one the fallback; every pass and open tree border is now cut through all rock ([report, *Round 3*](../../Reports/levelgen-2026-11-14/README.md)).
- A pass as a disc on the ridge's border: wherever the ridge ran wider than the disc, rock closed it (canyon seeds 2 and 3 needed 13 and 6 attempts); a pass is now a slot through the ridge's widest ([report](../../Reports/levelgen-2026-11-14/README.md), *Numbers*).

## Open

- Deep water, plants and other new ground written into the maps as their threads land them ([plan](../../Reports/levelgen-2026-11-14/biomes-plan.md)); the graphics lead's review of round 3.
- Every strategy can win (owner, 17:02Z): a check that each nest has an open approach and a choke.
- Planets bigger than today's: the 1070 timing at 1x, 2x, 4x and 6x, then `GROW_CAP` up.
- Twists shown before landing; rare rescue planets (base ruins, stranded soldiers) with the run thread's rescue goal.
- The owner's play.

## Sources

- [Procedural planets](../../Reports/levelgen-2026-11-14/README.md).
- [Larder maps](../../Reports/larder-maps-2026-11-11.md), *Which ground the mould feels* (what each ground does).
