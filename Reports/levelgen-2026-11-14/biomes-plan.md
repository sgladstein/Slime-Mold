# Biomes that play differently: the plan (2026-11-14)

Follows the [brainstorm](biomes-brainstorm.md). The owner's answers to its
seven questions, 2026-11-14 17:02Z (this thread), verbatim:

> 1) No. This should be fully new from the old grass. If you need an agent
> to just focus on plants, you can. 2) I like that idea. 3) Lets go with your
> rec for now. 4) rolls its own, but some biomes are more likely to get
> certain patterns. 5) No the strategy is probably set, by how the player
> upgrades their items (somewhat). Some biomes are more favorable to some
> strategies than others, but any maps should be beatable by any strategy:
> full gun killing or fully building turret/walls/kill boxes and luring
> slime into it or many other strategies, sneaking around, shooting from
> afar, using artillary, abilities to jump across the map, etc. Some players
> might choose to full specialize building, some fully into direct combat,
> some more balanced. 6) That makes sense. 7) New ground. Yes!

What each answer settles:

1. **Plants are a new simulation**, not the engine's grazing grass. A thread
   of its own designs and builds them (the owner offered one).
2. **Deep water stops soldiers; the slime crawls through it slowly.** Fords,
   shallows and bridges matter; shots fly over water.
3. **Dials plus named biomes.** A biome is a named range on each dial, so two
   planets of one biome differ and two biomes differ a lot.
4. **The crystal pattern rolls on its own**, from a table each biome weights.
5. **No biome forces a strategy.** Each favours some; every planet must be
   beatable by any build: all guns, all turrets and walls with lures and kill
   boxes, sneaking, shooting from afar, artillery, jumping across.
6. **Open and readable early, mazes and caves later.**
7. **New ground kinds are in.** A thread of its own builds what they do.

## Who builds what

| Piece | Thread | Notes |
| --- | --- | --- |
| Dials, biome table, crystal patterns, new set order, layouts (rivers, lakes, rooms, caves, oases), checks | this thread | Starts on today's ground; writes new kinds once the ground thread reads them |
| Plants: what they are and do (food, cover, fuel: the thread's own brainstorm with the owner), CPU and GPU, look | new *plants* thread | Reads its start density from the map (below) |
| Deep water and new ground kinds: their effect on the slime (CPU and GPU engine), on soldiers, buildings and paths (the game), editor brushes, look | new *ground* thread | Deep water first (answer 2) |
| Any new ground or plant look | graphics lead first | Before anyone touches `view.wgsl` (graphics lead, 11-14) |

## Map format (proposed; the ground and plants threads confirm)

Today: the PNG's G byte holds wet (0x01), rock (0x08) and the soil kind in
the high nibble (0 plain, 1 sand, 2 clay, 3 barren); R is parked for height
and B is free. The engine copies G into bits 8-15 of its ground word, where
bits 0x02 and 0x04 are built walls, so new map bits cannot go there.

- **Deep water: soil kind 4, written with the wet bit.** A build that does
  not know kind 4 still plays it as wet ground, never as plain.
- **New ground kinds: soil kinds 5 to 15**, numbered by the ground thread.
- **Plants: the B channel**, start density 0-255, read by the plants thread.

## The dials

Each planet rolls a value inside its biome's range on each dial.

| Dial | Values |
| --- | --- |
| Openness | open plain; scattered outcrops (today's pillars); ridges and passes (today's canyon); rocky rooms joined by passes; caves |
| Water | dry; pools (today's marsh); rivers with fords; lakes; archipelago |
| Plants | none; patches; meadows; jungle |
| Soil | shares of sand, clay, barren (today's bands, cracks, dune islands) |
| Soft rock | none; gates; hedges and rings (today's thicket) |
| Hazard ground | the ground thread's new kinds, per biome weight |
| Crystals | pattern (rolled, below) and size range |
| Size | 60% to 6x area by depth (owner, 15:33Z; the size PR) |

Today's seven signature shapes, which the graphics lead accepted, stay as
dial values, so no biome's look is lost.

## Crystal patterns

Rolled per planet from a table the biome weights (answer 4):

- **Scattered**: many small (today's).
- **Fields**: a big crystal ringed by small ones; a field held is worth a lot.
- **Veins**: lines of small crystals along ridges, shores or rivers.
- **Jackpot**: one giant crystal mid-map, little else.
- **Oases**: crystals only where the ground is good.
- **Richer farther**: small near the landing, big near the slime.

Sizes run from 10 to 70 map cells, the charge with the area.

## The set order (answer 6)

One biome a planet, open early and mazes late. A first proposal; the names
and order change freely.

| Planet | Biome | Dials | Favours (every strategy still wins) |
| --- | --- | --- | --- |
| 1 | Meadow | open; pools; meadows; scattered or fields | mobile guns, artillery; builders wall in the open |
| 2 | Barrens | open with outcrops; dry; plants only in oases; barren bands | the slime starves between oases: sniping, sneaking; builders hold an oasis |
| 3 | Fen | open; pools and lakes, deep and shallow; plants on the shores | wet slows the slime: kiting, shooting from afar; deep water bends paths |
| 4 | River valley | ridges; one or two deep rivers with fords; sand bars (today's dunes) | fords as kill boxes for builders; jumpers and artillery skip them |
| 5 | Jungle | scattered; streams; jungle plants; hedges and rings | the slime grows on plants anywhere: fire, denial; plants hide soldiers |
| 6 | Highlands | rocky rooms; dry; clay floors with cracks | room by room: builders hold passes, guns push through |
| 7 | Caves | caves; pools in the caverns; hazard ground | tunnels: kill boxes, short sight lines; artillery and jumpers go over |
| 8 | Core world | everything, the slime's own ground | all of it, at 6x area |

## Checks (answer 5)

The checks every planet passes today stay (every crystal and nest reachable
on foot, a crystal near the landing, the far nest across the map), with deep
water closed to feet. Two more, once there is something to check them with:

- **No planet needs one strategy**: every crystal reachable on foot (so no
  planet needs jumping), and an open approach to every nest as well as a
  choke (so building and direct play both have their ground).
- **The spread**: when the player AI has a builder hand and a gun hand, one
  small census of both over the eight biomes (a strategy question in
  CLAUDE.md's sense; named, timed and asked for first).

## Order of work

1. The size PR (maps carry their own size; 1070 timing at 1x, 2x, 4x, 6x).
2. Dials and the biome table on today's ground, crystal patterns, the new
   order; rivers, lakes, rooms, caves and oases as layouts; films and a grey
   page for the graphics lead.
3. Deep water written as kind 4 once the ground thread reads it; the checks
   close it to feet.
4. Plants density in B once the plants thread reads it; hazard ground as the
   ground thread lands each kind.
