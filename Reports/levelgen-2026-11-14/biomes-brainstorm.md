# Biomes that play differently: brainstorm (2026-11-14)

The owner, 16:50Z: "we need way more variability between biomes. There
should be some with plants and water in many of them. They should look
really different terrain. Some are open, some have lots of rock making
different areas, etc. think about how this affects strategy. There should be
different crystal distributions and sizes. Brainstorm this. Maybe research
worldgen a bit. Then ask me questions to help guide the plan".

Starting from the model (CLAUDE.md), not from the old demo's tools.

## What the simulation can already put on the ground

| Ground | What it does to the slime today | Map format |
| --- | --- | --- |
| Plain agar | Baseline | default |
| Sand | Spreads it (about 60% bigger) | G high nibble 1 |
| Clay | Holds it in place | G high nibble 2 |
| Barren | Starves it (no births, three times the upkeep) | G high nibble 3 |
| Wet | Halves its step | G bit 0x01 |
| Rock | Nothing crosses | G bit 0x08 |
| Soft rock | Walls the slime and shots wear down | `wall` items |
| Crystals | Food, scent, the thing both sides fight over | `crystal` items (any radius, any charge) |
| **Grass** (the engine's grazing field, off in the game today) | A regrowing field walkers eat and lean toward; with growth on it feeds births | not in maps yet |

Grass is the engine's "plants". Two dead ends bound it: grass that feeds well
floods (all 50,000 slots by 15 s on the race world), and grass that feeds
very little looked like plain droplets (`Reports/dead-ends.md`, the Drifter
tier). A plant biome needs grass between those, and per-cell plant density in
the map (the PNG's R channel is parked height; B is free).

## What worldgen practice says

- **Fields first, biomes from them** (Red Blob Games, polygon map
  generation and mapgen4): elevation and moisture are computed once; rivers
  run downhill; each region's biome comes from its elevation and moisture
  (a Whittaker diagram). Features then agree with each other (a lake sits in
  a low basin, plants follow the water) instead of being independent noise
  layers. Today's generator draws each biome's ground from its own noise.
- **An openness dial separate from the biome** (RimWorld): every biome comes
  flat, small hills, large hills or mountainous; mountains give walled
  pockets you defend, flat maps leave you exposed. The same biome then plays
  differently from map to map.
- **Resources as patches whose richness grows with distance** (Factorio's
  map generator): a guaranteed start area, then clustered patches, bigger
  and richer farther out, which pushes the player outward.
- **Fair start, contested expansions, a few routes** (Age of Empires
  random map scripts; Liapis et al., map sketches for strategy games;
  competitive RTS map design threads): map archetypes (open, islands,
  choke-heavy) decide the playstyle, and generators score candidate maps on
  resource safety and path balance before keeping one.

## A planet as dials, biomes as named mixes

Each planet sets a few dials; a biome is a named range on each, so two
planets of one biome still differ and biomes differ a lot from each other.

1. **Openness**: open plain, scattered outcrops, ridges and passes,
   rocky rooms (plateaus joined by passes), caves (mostly rock, carved
   caverns and tunnels).
2. **Water**: dry, ponds, rivers (bands that split the map, with fords),
   lakes, archipelago (mostly water, islands).
3. **Plants** (grass): none, patches, meadows, jungle.
4. **Soil mix**: sand, clay, barren shares.
5. **Soft rock**: none, gates, hedges, thicket.
6. **Crystal pattern** (below).
7. **Size** (already planned: 60% to 6x area).

Elevation and moisture fields would drive 1-3 together: rock on the highs,
water in the lows, plants near water.

## Biome sketches and what they do to strategy

| Sketch | Ground | Strategy it pushes |
| --- | --- | --- |
| Open plains | Little rock, meadows of grass | The slime arrives as a wide front, fed by the grass; the squad has to move and burn lines; turrets get flanked |
| Jungle | Dense grass, rivers, soft-rock thickets | The slime grows anywhere, not only on crystals; you deny ground (burn) rather than guard crystals |
| Archipelago | Mostly water, islands joined by shallows | Water slows the slime, so islands are bases you can hold; the shallows are chokepoints |
| Highlands | Plateaus of open ground walled by rock, joined by passes | Room by room: hold a pass with walls and turrets, push one room at a time |
| Caves | Mostly rock, caverns and tunnels | Tight fights, few routes, the slime funnels into tubes you can cut |
| River valley | One or two wide rivers across the map, few fords | The fords are the whole battle |
| Wastes with oases | Barren everywhere, crystals and plants only in oases | The slime starves between oases; you hop from oasis to oasis |
| Crystal fields | Open ground, one huge field of crystals mid-map | Everyone fights over the middle |

## Crystal patterns

Today: a home crystal under each nest, one near the landing, 1-3 big ones
midway, a dozen small ones scattered; radius 18-42 cells.

- **Scattered**: many small (today's).
- **Fields**: a big crystal ringed by small ones, a few fields a planet.
  Guard play: one field held is worth a lot.
- **Veins**: lines of small crystals along ridges or rivers.
  Drain play along a line.
- **Jackpot**: one giant crystal mid-map (radius 60+), little else.
- **Oases**: crystals only where the ground is good.
- **Richer farther** (Factorio): small near the landing, big and rich near
  the slime, which pushes the squad out.
- Sizes from 10 to 70 cells, charges to match.

## Questions for the owner

In the thread, 2026-11-14 ~17:00Z.

## Sources

- Red Blob Games, polygon map generation: https://www.redblobgames.com/maps/polygon-map-generation/
- Red Blob Games, mapgen4: https://www.redblobgames.com/maps/mapgen4
- RimWorld wiki, world generation: https://rimworldwiki.com/wiki/World_generation ; biomes: https://rimworldwiki.com/wiki/Biomes
- Factorio wiki, map generator: https://wiki.factorio.com/Map_generator
- Age of Empires, generated map basics: https://support.ageofempires.com/hc/en-us/articles/4865310033684-Generated-Map-Basics
- Liapis, Yannakakis, Togelius, generating map sketches for strategy games: https://mail.antoniosliapis.com/papers/generating_map_sketches_for_strategy_games.pdf
- Wave Function Collapse for balanced strategy maps: https://amt-games.com/news/applying_wave_function_collapse__wfc__algorithms_for_balanced_strategy_maps.html
