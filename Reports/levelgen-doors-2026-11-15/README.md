# A front door and a back road for every nest (2026-11-15)

The play review ([report](../levelgen-review-2026-11-15/README.md), item 4)
found almost no nest with both a narrow direct way in and a wide way round:
4 of 112 on seeds 1-8. The open planets had no cheap choke to hold, and on the
river valleys, highlands and caves every way in was narrow. The owner's plan
for this thread (01:56Z, "Resume") put this right after the map fixes
([report](../levelgen-mapfix-2026-11-15/README.md)).

- Before: the map fixes (PR #323). After: this change.
- Page: [Front Doors and Back Roads](https://claude.ai/artifact/KTjWVt9GmBfDwgPKqzbJoF), copy in `page/`.
- Deciding measure, named before the first run: the play reviewer's own
  script, `raw/localchoke.py` (one fix, below), on seeds 1-8 of every planet
  (112 nests): nests whose direct way is at most 10 dish cells wide and whose
  way round is at least 16.
- 160 planets surveyed (seeds 1-20, depths 1-8):
  `cargo run --release --example planets -- seeds=1-20 depths=1-8 out=DIR`,
  then `python3 raw/survey.py BEFORE AFTER` ([table](raw/survey-table.txt)).
- In-game frames: `larder-world` on llvmpipe, `hand=idle`
  ([`raw/renders.sh`](raw/renders.sh), [`raw/renders2.sh`](raw/renders2.sh),
  centred by [`raw/points.py`](raw/points.py)); false-colour layouts by
  [`raw/fc.py`](raw/fc.py); the page's pictures by
  [`raw/make_pics.py`](raw/make_pics.py) (render times in
  [`raw/render-timing.txt`](raw/render-timing.txt)).

## What a nest looks like now (`src/mould/levelgen.rs`)

- **A rock bowl.** Each nest sits in a bowl 104-136 map cells across inside,
  with a rim 26-36 thick. The rim is lobed (its radius wanders 15% with a
  90-cell noise) and knobbly (its thickness 0.3-1.7 of the mean with a 26-cell
  noise). With the first wobble, 4% and 0.75-1.25, it read as a drawn circle
  in the game's frames.
- **A front door.** A straight channel 7-9 dish cells wide faces the landing,
  then a lane as narrow runs on until it meets open ground.
- **A breach.** A gap 28-35 dish cells wide opens 105-150 degrees round to one
  side, as far round as the road fits.
- **A back road.** A road 20-23 wide runs from the landing to in front of the
  door, then round the outside of the rim to the breach. It follows the
  ground (`route`): the cheapest way over 8-cell bins, rock costing 6 times
  plain ground and deep water 3 times, with noise so it wanders. It cuts rock
  only where it must and is made shallow across deep water.
- **Kept clear.** Crystals, pillars, hedges, gates and the nest rings keep off
  the road, the door and the breach (`lanes`). The landing's crystal does
  too: first toward the middle, then anywhere within 220 in the landing's
  region or in plain sight of it, and only last on a road.
- **Caves.** A cavern the rims cut off is filled in, so no crystal is left
  where it can't be reached.
- **Checked.** `check` holds every nest to both: no direct way (within 15%,
  at most 250 cells, of the shortest walk to ground in sight of the nest)
  wider than `DOOR_MAX` 10, and a way round at least `ROAD_MIN` 16
  (`doors`, `door_problems`). Shots stop at rock, so only ground in sight of
  the nest counts as reaching it.
- **Nests 470 apart at least.** Nests stood 0.3 of the map's width apart
  (the map fixes); now also at least 470 map cells, so two bowls and a road
  round one fit between them (river valley seed 2's second nest moved).
- **Small maps.** Below today's area, a nest that can't stand 400 cells from
  the others is left out, and the check asks for one nest there. A cave asked
  for at 0.6 of today's area put two nests 175 apart, their bowls made one
  cavern, and every attempt failed.

New test: `every_nest_has_a_front_door_and_a_back_road` (seeds 1-6 of every
planet: direct 6-10, round 16 or more). All 14 pins re-pinned.

## Measured

**The play reviewer's measure** ([table](raw/choke-table.txt)), on seeds 1-8,
112 nests:

| Build | Nests with both |
|---|---|
| Reviewed (4494ad4f) | 4 / 112 |
| Map fixes (before) | 4 / 112 |
| Doors (after) | **111 / 112** |

Every biome but the core is 100%: 8/8 meadow, barrens and fen; 16/16
valley, jungle, highlands and caves; 23/24 core. On that measure the direct
ways are 4-10 wide and the ways round 17-34. Its widths are 2 x the Euclidean
clearance on unnarrowed walking bins, with goals in a disc round the nest,
sight or not.

The one miss is the far nest on core seed 2 (12 / 22). Its "direct" way of 12
is the road round to the breach: that walk is 1.149 times the walk through
the door, just inside the measure's 15%. So that nest has two near-equal ways
in, not one wide one. `doors` puts it at 8 / 22, as its limit is the shorter
of 15% and 250 cells.

**`doors` over 160 planets** (280 nests; [survey](raw/survey-after.txt)):

- Direct: 6 wide on 194 nests, 8 on 81, 10 on 5.
- Round: 16-32 wide. The median is 20-26 by planet, and 3 nests sit at
  exactly 16.

**Cost and checks** ([table](raw/survey-table.txt)):

| Planet | First-try passes | Median build | Median rock share |
|---|---|---|---|
| 1 | 19 -> 18 of 20 | 50 -> 54 ms | 2% -> 7% |
| 2 | 19 -> 20 of 20 | 54 -> 64 ms | 4% -> 7% |
| 3 | 19 -> 18 of 20 | 86 -> 96 ms | 3% -> 6% |
| 4 | 20 -> 18 of 20 | 157 -> 168 ms | 10% -> 12% |
| 5 | 19 -> 19 of 20 | 168 -> 191 ms | 5% -> 6% |
| 6 | 16 -> 15 of 20 | 253 -> 288 ms | 19% -> 17% |
| 7 | 18 -> 19 of 20 | 296 -> 360 ms | 56% -> 51% |
| 8 | 19 -> 16 of 20 | 561 -> 718 ms | 9% -> 8% |
| All | 149 -> 143 of 160 | | |

- No fallback planets.
- Build time is up 8-28% by planet. The core world takes 0.72 s once a level.
- The rock share rises on the open planets (the rims) and falls in the caves
  (the roads).
- The walk to the far nest is up to 13% shorter (highlands 1,618 -> 1,408
  map cells), as the roads run straight through.
- Soft-rock pieces are down on the jungle and core (hedges and rings kept off
  the roads).
- Fewer crystals on planets 1-4 and the caves: medians 11 -> 10, 12.5 ->
  10.5, 13.5 -> 13, 16 -> 15 and 35 -> 31 (caves fewer on 16 of 20 seeds).
  The fields lose spots to the lanes and the filled caverns. Planets 5, 6 and
  8 are level. A tuning note: the field counts can be raised.

## Tried on the way

These were measured on seeds 1-8 (64 planets, first-try passes); the
`Reports/dead-ends.md` entries hold the conditions.

1. **A rim, a door, a breach and a straight road.** Planets 1-3 all passed;
   42 of 64 overall.
2. **The landing's crystal sat on the roads' first stretch.** On highlands
   seed 3 it narrowed the way round to 10. Moving it off the roads: 43 of 64.
3. **Nest rings across the breach.** They shut the way round on every
   jungle. Moving them off the lanes: 53 of 64.
4. **The landing's crystal had no room off the roads** where the landing
   sits in a small room (highlands seed 4). Letting it sit farther, anywhere
   in plain sight of the landing: 54 of 64.
5. **The road's arc started 60 degrees round from the door.** The leg from
   the landing to there, then a short arc, was as quick a way to the breach as
   to the door on the caves and big planets. In the caves, the door's lane
   could end in rock. Starting the arc at the door: 58 of 64.
6. **Cut-off caverns.** Filling them: 60 of 64.
7. **Straight roads.** Cut straight from the landing, the roads ran as wide
   boulevards across the caves and through the highlands' ridges. Routed over
   the ground: still 60 of 64, and the caves keep their tunnels.
8. **Dropping a nest under 400 apart on every map.** That failed 7 of 20 river
   valleys' first tries, which pass `check` with the nests closer. Now it
   applies only below today's area.
9. **Lobed rims.** On 160 planets: 145 -> 143 first-try passes.

**The measure's fix.** `raw/localchoke.py` is the play review's script with
one fix: its search took `inf <= inf` as a pass. A width that reached no nest
then counted as "within 15%", so its direct widths were too wide on funnel
planets. The review's 4 of 112 holds with the fix too.

## Not done here

- **The rims are rock on every biome.** A moat on the fens, a thorn hedge
  (soft rock the squad can break) in the jungle, or a ring of boulders on the
  meadows would tell the biomes apart. That is for the looks review's
  item 3 (half the biomes can't be told apart at zoom 12).
- **The door is narrowest at 6 dish cells on our measure, and 4 on the
  reviewer's.** The door channel is 7-9 wide; the rim's knobs and the
  walking narrowing take the rest. A squad files through either way.
- **Fens weighted to lakes, cave tunnels 5-15 wide, a per-biome landing
  crystal, and nests off river crossings** are still to do.
- **Crystal counts** on planets 1-4 and the caves are down 4-11% (above).
- **The Windows run of the pins** (PC runner) is still to ask for.
