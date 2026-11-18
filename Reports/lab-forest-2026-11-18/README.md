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

## What was built

Lab branch `claude/project-thread-4tc75h` only, GPU only, code commit
`a9b9e61` (never merges), switched on by `LAB_FLOOR=1`. It rides on the
plants' passes, which already run every plant step on a run planet.

- **Water.** Plant block 9 is the standing water. Each plant step every
  open cell trades water with its four neighbours toward the lower surface
  (height 1 minus the map's moisture, plus the water), the map's rivers and
  lakes carry a fifth of theirs away, and every cell dries a little. A
  shower falls for the last `LAB_RAIN_SECS` (20) of every `LAB_RAIN_EVERY`
  (60) game seconds, 0.0006 a plant step.
- **Live moisture** (block 10) is the map's own plus twice the water. The
  map's own (block 7) stays as it was, because the view shapes plant
  patches by it.
- **What reads it.** The slime leans toward standing water (sensor weight
  `LAB_SEEK` 400), pays 1.6 times the upkeep on dry ground down to 0.6
  times on damp, walks at 0.4 of its step in a pool (water over 0.15) and
  its trail washes out there; fire reads the live moisture; a soldier in a
  pool walks at half speed (the water read back once a game second); the
  view darkens the ground and makes it shine as water pools.
- **Litter that rots into food:** part two, below.

Frame cost: the water adds four neighbour reads a cell to the plant pass,
on the 99,200 cells of planet 1, every plant step. The once-a-second
read-back of the water for the soldiers copies the engine's whole `news`
buffer; a game version would read the water block alone, or move the mud
onto the GPU. Not timed (software GPU).

## Results (planet 1, run seed 1, greedy hand)

Clips `page/base.mp4` and `page/floor.mp4`: 0:35 to 1:35 at 4 frames a
second, played at 16 fps (four times speed). Mould walkers:

| | 0:40 | 0:50 | 1:00 | 1:10 | 1:20 | 1:35 | 2:00 |
|---|---|---|---|---|---|---|---|
| No floor | 12,552 | 12,253 | 12,158 | 11,904 | 11,735 | 10,900 | 10,523 |
| Floor | 12,751 | 12,456 | 12,351 | 11,517 | 10,603 | 9,841 | 9,622 |

The water through one shower (0:40 to 1:00), cells of 99,200: pooled
(over 0.15) 582 at 0:50, 6,397 at 1:00, 2,212 at 1:10, 185 at 1:30;
soldiers in a pool at most 5 of 19.

- **The ground visibly wets and dries.** The shower darkens the floor in
  a mottled pattern (the map's noise sets the hollows), the rivers take
  the water away, and the floor is dry again 30 to 40 s after the rain.
- **The slime answers it.** During the shower the slime's edge breaks
  into the wet ground in scattered beads, east and north of its body; as
  the ground dries they are stranded on dry ground that costs them more,
  and the slime ends 1:35 at 9,841 against 10,900 without the floor (one
  seed). A stronger cost of dry ground (3x, 0.3x on damp) shrank it to
  7,758 by 2:00, mostly before any rain.
- **The squad barely feels it** on planet 1: few pools form on the high
  ground round the north crystal, so at most 5 of 19 soldiers stood in mud.
- Fire through a dry spell was not filmed (the greedy hand lights none).

## Part two: litter that rots into food (11-18)

Direction, 14:14Z: "yes to the rot, a day at most ... logs and litter that
rot into food the slime can take, read off the same moisture." Code on the
lab branch (commit `47150aa`, same `LAB_FLOOR=1`, never merges):

- **Plant block 12 is the rot**, food in units of the slime's upkeep.
  Litter (the plants on a cell, up to 1) rots where rain has soaked it: the
  live moisture over the map's own, less 0.05, times 2 a plant step, up to
  200. Not in rivers and lakes. The walkers standing on a cell eat it (each
  up to 3 upkeeps a tick, shared), and rot does not dry out: it waits to
  be eaten.
- **The slime finds it** because rot lays scent like a crystal (0.3 a unit
  of rot, a crystal cell lays 20), and its sensors read it too.
- **It is drawn** as dark umber soft ground with pale flecks where richest.
- Tried and changed: rot wherever the live moisture passed 0.5 rotted the
  map's damp river banks before any rain, so the rot ran along every
  river and drew the slime down the banks; the rot now needs rain. Without
  its scent the slime ignored the rot more than a few cells from its edge
  (smell weights 2 to 40 changed nothing).

Results, planet 1, run seed 1, greedy hand, the same shower (0:40 to 1:00).
Clip `page/rot.mp4`, 0:35 to 1:35 as the floor clip:

| | 1:00 | 1:15 | 1:30 | 1:35 | 2:00 |
|---|---|---|---|---|---|
| Floor, no rot: slime | 12,351 | 10,965 | 9,953 | 9,841 | 9,622 |
| Floor, no rot: killed | 604 | 1,081 | 1,260 | 1,306 | 1,476 |
| Floor and rot: slime | 12,312 | 10,940 | 10,081 | 9,931 | 9,401 |
| Floor and rot: killed | 655 | 1,257 | 1,464 | 1,531 | 2,020 |

(2:00 from separate runs with the same settings: `r1-floor`, `r3-sc0.3`.)

- **Rot forms only after rain**: none before 0:45, 0.55 million upkeeps by
  1:00, 1.1 million by 1:15, and little more until the next shower.
- **The slime goes out for it** in scattered foragers across the wet
  meadow, and the squad shoots them: a sixth more killed by 1:35, a third
  more by 2:00. The slime ends no bigger: what it eats is about what it
  loses to the guns.
- **Most of the rot is never eaten**: the rot on the ground still grew
  from 1.07 to 1.10 million upkeeps between 1:15 and 1:35, after the
  rain; the slime takes what is near its edge.

So the rot works as bait the weather lays: after a shower the slime comes
out of its body to forage, where a squad can catch it. It is a stronger
play hook than the water alone, and the same recommendation stands: change
(a wet planets' trait, Look's pass on the water and the rot), then build.

## Recommendation

**Change, then build.** The ground can be simulation on the GPU engine at
little cost: water falls, runs to the rivers, pools and dries, and the
slime, soldiers, fire and the picture all read the same field. On planet 1
it is a mild tide, not a game changer: the slime is about a tenth smaller
after a shower and the squad rarely stands in mud. Before it goes in:

1. **Put it on wet planets** (Fen, Jungle) as a planet trait, where the
   map's hollows are deep enough to hold pools the squad must cross.
2. **Give the player a verb on it**: fire in a dry spell (it reads the
   moisture already), or a pump that drains a pool.
3. **Make the water read clearly**: at the full-planet view the wet floor
   is a darker, mottled green, easy to miss; puddle edges and the shine
   need Look's pass (only the full view was filmed).

The rot is part two, above. The lab stops here and waits for a brief
(Direction, 14:14Z).
