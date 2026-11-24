# Lab: planet twists, a cold snap, night and wind (2026-11-24)

The Lab lane's eighteenth job, on the lab branch `claude/project-thread-2o4cic`
(code `4f4f3856`), not in the game. Brief from Direction 2026-11-24 17:00Z,
for Milestone 2's test 2 ("a run he wants to replay"): today two runs
differ only in maps and shop rolls, so prototype planet twists, one
condition a run can roll on a planet, starting from the model. Build two or
three, each changing what the slime does in a way you can see within a
minute, each with a smoke test and a clip at zoom 6, and name the one or
two for the main game with a card line each.

Page: "Planet Twists" (source in `page/`, listed in `Reports/pages.md`).

## What a twist is

`LAB_TWIST=NAME:START:LEN:F[:A]` changes one of the slime's own numbers
from game second START for LEN seconds (defaults 20 and 60), then gives it
back. It is driven once a game second from the GPU world's `set_ceiling`.
Each one is a rule of the walkers' model, so the picture is the change:

- **Cold snap** (`cold`): the slime's step times F (0.3). It crawls.
- **Night** (`night`): its sensors reach F (2.5) times as far, at 0.6 of
  their angle, so it finds food and soldiers from farther off.
- **Wind** (`wind`): every step a walker takes is also pushed F (0.25; 0.35
  in the films) dish cells towards angle A (0 is east). The body leans and
  creeps downwind. This is new code in `engine.wgsl`'s `sense`, through the
  lab words (0x4 in `lab_th`'s top nibble).

## What each does (planet 1, run seed 1, lavapipe)

Films: an idle squad of 4, the twist from 0:10 to 1:10, a frame a second
from 0:05 to 1:10 at zoom 6 on the squad (`raw/film.sh`, `raw/clips.py`;
clips in `page/`). Each changes the body within 30 s:

- **Cold**: the streams break into beads and stall short of the squad; at
  0:20 the body reaches 105 cells from its centre against 137 as it is.
- **Night**: the slime lays one thick road from its west lobe to the north
  crystal and on towards the squad, and leaves the far crystal.
- **Wind** (east): the body spills east past the squad and swallows the
  north-east crystal, which the slime as it is reaches only with a thin
  arm.

Smoke tests: the twist from 0:20 to 1:20, 3:00, greedy and idle hands
(`raw/smoke.sh`). Walkers the squad killed:

| Hand | Twist | 1:00 | 1:30 | 3:00 | Slime at 3:00 | End |
|---|---|---|---|---|---|---|
| greedy | none | 1,155 | 1,571 | 2,956 | 10,808 | 45 soldiers, no wipe |
| greedy | cold | 844 | 1,056 | 2,153 | 11,008 | 45 soldiers |
| greedy | night | 1,383 | 2,204 | 4,459 | 11,320 | 45 soldiers |
| greedy | wind | 1,286 | 1,739 | 3,247 | 11,005 | 45 soldiers |
| idle | none | 843 | 1,703 | 4,751 | 19,767 | 4 soldiers, no end |
| idle | cold | 340 | 867 | 3,751 | 19,984 | 4 soldiers |
| idle | night | 794 | 1,930 | 4,938 | 21,555 | 4 soldiers |
| idle | wind | 1,076 | 2,724 | 6,111 | 19,804 | 4 soldiers |

Smoke checks: each twist begins and ends on its second, with the slime's
numbers back after it; idle wins nothing; no game ends under a minute;
nothing soft-locks. On planet 1 none changes the outcome, so whether a twist
makes a planet harder or easier is for Direction to tune on later planets.
Cold brings the slime to the guns less (about 30% fewer kills while it
lasts), and night and wind bring it more (wind east, towards this squad,
kills 60% more by 1:30 idle). The night slime grows most, 9% more than
without the twist by 3:00 with an idle squad, because it finds more ground.

## Recommendation

Put **wind** and the **cold snap** in the main game, first as a run's roll
on one planet in three or so. Each asks the player for a choice:

- **Wind** is the clearest new picture: the whole body leans one way. It
  gives a planet a direction, so where you land, stand and pump matters.
  Card line: *"Wind from the west: the slime leans and creeps east all
  planet. Stand upwind, or let it blow onto your guns."*
- **Cold snap** is a timed window: for a minute the slime breaks into
  beads and crawls, and that is the minute to push. Card line: *"Cold snap
  at 0:20: for a minute the slime crawls in beads. Push now."*
- **Night** reads well, as thick roads to food, but it only makes the slime
  better at what it already does, with no new choice for the player. Keep it
  for a harder planet, or pair it with a look that shows the dark.

What the game needs to build them: wind as a planet roll with an angle
(the pass is a few lines in `sense`, now in the lab words; a real Params
row for it is Enemy's), a mark that shows the wind's way (in the
simulation's terms: the trail itself leans, so a still frame shows it), and
the twist's line on the planet's card and in the HUD. Cold and night need
only a timed change to existing numbers (`step`, `reach`, `sensor_angle`).

## Raw

`raw/film.sh` (the films), `raw/smoke.sh` (the eight smoke games),
`raw/clips.py` (the clips), `raw/occ.py` (body measures from a `LAB_DUMP`
folder).
