---
title: Planets (the roguelike run's levels)
kind: design
status: open
updated: 2026-11-15
sources:
  - Reports/larder-maps-2026-11-11.md
  - Reports/levelgen-2026-11-14/README.md
  - Reports/levelgen-mapfix-2026-11-15/README.md
  - Reports/levelgen-review-2026-11-15/README.md
code:
  - Biome::ORDER
  - Biome::moisture
  - CORE_DEPTH
  - GROW_CAP
  - Openness
  - Pattern
  - Recipe::of
  - Soil::Oases
  - Sow::of
  - StartEvent::RandomPlanet
  - Water::Rivers
  - cut_fords
  - gpu::buffer_bytes
  - levelgen::check
  - levelgen::fallback
  - levelgen::level
  - levelgen::planet
  - levelgen::size_at
  - levelgen::spec
---

# Planets (the roguelike run's levels)

## Now

Each level of the roguelike run is a generated planet (`mould::levelgen`): `level(seed, depth)` is a map in the designed maps' format, so the game and the editor open it like any map, and `planet(seed, depth)` is its card (name, biome, size). Eight biomes come in a set order, open early and mazes late: Meadow, Barrens, Fen, River valley, Jungle, Highlands, Caves, and the core world eighth (`CORE_DEPTH`). Each biome is a named range on four dials, rolled per planet with the biome's odds: openness (`Openness`: open, outcrops, pillars, ridges, rocky rooms, caves), water (`Water`: dry, pools, lakes, rivers with fords, streams), soil (`Soil`: plain, sand, dunes, barren bands, oases in barren, clay, cracked clay, mixed), and the crystal pattern (`Pattern`: scattered, fields, veins, one jackpot, at the oases, richer toward the slime). Everything is ground the simulation already plays (barren and wet starve and slow the slime, sand spreads it, clay holds it, rock and soft rock wall it). A planet is regions round its landing, nests and fields, with ridges and passes across the route; one nest to planet 3, two from 4, three on the core world. Every planet is checked before it is played: every crystal and nest reachable, a crystal near the landing, the far nest across the map on foot; all of 400 planets passed within four attempts (354 first), in 75 ms each (median), once a level, rock from 2% (meadow) to 50% (caves) of the ground; rivers at least 40 map cells across and streams 16, so they read at zoom 1 (graphics lead, 18:56Z) ([report, *Round 3*](../../Reports/levelgen-2026-11-14/README.md)). The same seed and depth give the same planet on every machine (no platform maths). Play one with `larder-world map=planet:3:42`, or Random planet in the start menu (the Planet option picks the depth; N rolls another). A map carries its own size: planets 1-8 are 60%, 80%, 100%, 140%, 200%, 280%, 400% and 600% of today's area (`AREA`, capped by `GROW_CAP` 6 since 2026-11-15). Three fixes made 6x play smoothly on the 1070. The window's ground refresh reads back only crystal and wall cells (32.9 to 2.6 ms). Soldiers' walking maps are built a slice a step (a game step's 210 ms max to 1.7 ms). The gas press reads back only its words (37.8 to 7.6 ms) ([report, *Walking maps at 6x*, *The 1070 at 6x, and the gas press*, *Planets at full size*](../../Reports/levelgen-2026-11-14/README.md)). A core world takes 0.74 s to build at 6x on average, 4.4 s at worst; `map=planet:D:S:AREA` plays any area to 6x, whose GPU buffers measure 235 MB ([report, *Bigger planets*](../../Reports/levelgen-2026-11-14/README.md)). Each biome writes its own plants into the map's B byte (meadow grass, jungle canopy, reed beds at water, scrub on sand, sparse scrub clumps on the barrens, lush oases, grass on the highlands' clay only, next to none in caves: cover from about 4% to 79% of a planet; density fading in from each patch's edge over a few cells, so the drawn edges run smooth at close zoom; [report, *Plants that tell the biomes apart*](../../Reports/levelgen-2026-11-14/README.md)) and its moisture line for fire (`Biome::moisture`), and lakes and rivers have deep middles (`G_DEEP`: soldiers can't wade, the slime crawls through) with fords and, on half the rivers, a bridge; where deep water would cut the squad off a crystal or nest, `cut_fords` makes the shortest way shallow ([report, *Plants, deep water and 2x planets*](../../Reports/levelgen-2026-11-14/README.md)). The core world has a few open arenas (`Recipe::arenas`, three on today's map and more at the same spacing on a bigger one): field regions cleared of rock, pillars and hedges, opened on every side but a nest's, and outside rocky rooms a hedge rings every nest. Over 50 core worlds, rock fell from 11.1% to 7.6% of the map ([report, *The core world's arenas*](../../Reports/levelgen-2026-11-14/README.md)). Three reviews (looks, play, code; 2026-11-15) ranked what to change next: the far view below zoom 1 is unshaded, rock and soft rock outshine the thin slime, half the biomes can't be told apart at zoom 12, and 4 of 112 nests have both a narrow direct way and a wide way round; the checks now keep off the dish's 12-cell rim, and the fallback is an open plan tried until it passes (`fallback`) ([report](../../Reports/levelgen-review-2026-11-15/README.md)). Their first map fixes, looks first: nests at least 150 map cells off every edge (0.16 of the width, 0.2 of the height) and 0.3 of the map's width apart, giving way only where the far part has no room; streams keep the 1x count on a bigger map and rivers stop at two; a nest's ring is 2-3 arcs at their own radii; no hedge piece on water; bridge decks are straight bands past the banks and the deep middle tapers to a ford. Over 160 planets, nests by an edge went from 141 of 280 to none and hedge pieces on water from 2,518 to 378 (cave gates on pools remain) ([report](../../Reports/levelgen-mapfix-2026-11-15/README.md)). Pages: [round 1 films](https://claude.ai/artifact/RYfaQD8SGMSMSuaKZtaD5P), [round 2](https://claude.ai/artifact/SJRBVoTF3yyEgfJvh6Da7t), [round 3](https://claude.ai/artifact/8YTtSxUEGnXivr6VejCaNU), [core world arenas](https://claude.ai/artifact/YXPyHxhBmaL1K92Mq8L1Bv), [the three reviews](https://claude.ai/artifact/Kcy1AoquWJjJ3EciyA3KxP), [map fixes](https://claude.ai/artifact/HPbTQJWDTe3qUTktVtuFqA).

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
- Nests 280 map cells apart and 100-110 off the edges at every size, one round ring with one gap, streams and rivers grown with the map, bridge decks in the river's outline and square fords: core-world nests bunched (worst pair 0.12 of the width), 141 of 280 nests by an edge, rings read as bullseyes, a 6x core world webbed with streams; replaced 2026-11-15 ([report](../../Reports/levelgen-mapfix-2026-11-15/README.md)).

## Open

- The three reviews' map items still to do: a front door and a back road for each nest ("any maps should be beatable by any strategy", owner 17:02Z), nests off river crossings, cave gates off pools, fens weighted to lakes, cave tunnels of varying width ([report](../../Reports/levelgen-review-2026-11-15/README.md), *Next*; [map fixes](../../Reports/levelgen-mapfix-2026-11-15/README.md), *Not fixed here*).
- Twists shown before landing; rare rescue planets (base ruins, stranded soldiers) with the run thread's rescue goal.
- The owner's play.

## Sources

- [Procedural planets](../../Reports/levelgen-2026-11-14/README.md).
- [Three reviews of the procedural planets](../../Reports/levelgen-review-2026-11-15/README.md).
- [Map fixes from the planet reviews](../../Reports/levelgen-mapfix-2026-11-15/README.md).
- [Larder maps](../../Reports/larder-maps-2026-11-11.md), *Which ground the mould feels* (what each ground does).
