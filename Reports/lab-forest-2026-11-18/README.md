# Lab: the forest floor, ground that is simulation

2026-11-18. The Lab lane's brief from Direction (13:34Z): "can a planet's
ground itself be simulation (wet and dry litter, logs that rot into food,
water that pools and dries) so the slime's spread and the squad's footing
change over a round, on the GPU engine?" Scott chose "the forest floor from
above" on 11-10 (`Reports/mould-real-world-2026-11-09.md`, §5); it was
frozen as a dish game, so this is the lab's, on its own branch. Start from
the model; `Reports/dead-ends.md` has nothing on soil water, decay or
moisture as mechanics (only soil looks and map water).

## The model, first

What a run planet's ground already carries on the GPU engine:

- **The ground word**, fixed for the map: rock and walls; a soil kind (sand
  and clay keep less trail; barren ground has no births and costs three
  times the upkeep; deep water is a slow step with no births); a wet bit
  (plants grow faster there and fire cannot burn it).
- **Plants**, four kinds that grow toward a capacity set by soil, kind and
  the wet bit, and spread from their edges. They are fire's fuel; fire
  leaves char, and char of 0.3 or more is barren.
- **Moisture**, 0.05 to 1 a cell, fixed for the map: the biome's base, two
  noises, wetter near water (`plants::moisture`). Only fire reads it (how
  fast it eats, how hot, the chance it goes out).
- **Crystals** (the food: whoever stands on one eats it) and **crust**
  (what the slime leaves behind).

So the ground changes during a round only by what burns, what the slime
eats and what it leaves. Nothing about it changes by itself, and the squad
reads none of it but walls, rock and water.

What a forest floor would add:

1. **Water that moves.** Rain adds water to every open cell; it runs down
   to low ground and pools there, and dries. The map already says where
   low ground is: its fixed moisture is highest by rivers and in wet
   biomes, so a height of 1 minus that moisture sends rain to the rivers
   and the fen, not to random hollows. Moisture becomes live: the map's
   own plus the standing water.
2. **Litter that rots into food.** Wood and leaf litter rots where it is
   wet and stops rotting where it is dry. The rot is food the slime eats
   by standing on it and smells, like grazing (the engine's grass field
   does that already).
3. **Everyone reads it.**
   - The slime: damp ground is cheap to live on and dry ground costly
     (slime moulds live on damp), and standing water is a slow step that
     washes trail. After a shower it surges; in a dry spell it shrinks.
   - The squad: standing water is mud, a slower step.
   - Fire: reads the live moisture, so nothing burns after rain and
     everything burns late in a dry spell.
   - Plants: grow faster where it is wet, as the wet bit does now.

A round then has weather, and each shower and dry spell moves the slime,
the squad and the fire differently: a surge after rain, mud in the low
ground, a fire window when it dries.

The prototype builds 1 and 3 first (water, live moisture, what the slime,
the squad and fire read), on the GPU engine only, and 2 if the first shows.
