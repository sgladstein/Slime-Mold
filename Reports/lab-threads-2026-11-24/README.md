# Lab: the Sprawler's body as threads (2026-11-24)

The Lab lane's sixth prototype, on the lab branch `claude/project-thread-2o4cic`
only (code `283617eb`, on main's code of 11-24 before the origin, `406c2c9`),
not in the game. Brief from Direction 2026-11-24 03:14Z, after Enemy's three
dead ends of 11-22 (spacing, room, lanes; [dead ends](../dead-ends.md)): the
Sprawler's packed body as threads, strands between its crystals and its food
with holes in the body round them, the walkers' number within 5% of the off
arm, planet 2 seed 1 then seeds 2 and 3, frame cost on lavapipe, about four
hours. GPU only, no CPU twin.

Page: "Sprawler Threads" (source in `page/`, listed in `Reports/pages.md`).

## From the model

Planet 2's slime is the plain rules with the Sprawler's walk (trail kept 0.8
a step, sensors 18 fine cells out, step 1.5, explorers for 200 steps). Its
walkers lean up the trail and up the crystals' scent. Three things pack the
body:

- A fine cell holds one walker. Round a crystal the scent hill is the
  strongest reading, so every calm walker leans in and the body is a disc
  over its crystal; inside it a walker is blocked and only gets a new
  heading.
- A fed walker has a child in a free cell beside it, so any cell the body
  frees is filled (Enemy's lanes: 18,096 to 27,400 walkers).
- Nothing in the body moves one way more than another, so the trail is flat
  across it and there is no ridge to gather on.

Threads need room (cells that can hold more than one walker, while the
number stays the same) and a flow that lays ridges through the body.

## What was built

Keys from the environment, in the Mind row's two spare words (`lab_th`,
`lab_p`); none in the game.

1. **Lanes** (`LAB_LANES=N:R`, Enemy's rule of 11-22): a calm walker
   blocked by a full cell may still step in, up to N more walkers there,
   when that cell holds R times its own cell's trail.
2. **The shuttle** (`LAB_SHUTTLE=W:F`): a calm walker fed past F of its
   birth energy reads the crystals' scent x -W, so it walks out from its
   crystal along the trail, and back up the scent once hungry. Two-way
   traffic between the crystal and the body's edge, along the same ridges.
3. **The count held** (`LAB_CAPS=FILE`, `LAB_HOLD=1`): the ceiling is held
   each second to the off arm's walkers that second (from its dump), and
   the upkeep eases (x 0.85 a second, at most to x 0.1) while the slime is
   under 97% of them, so the look is judged at the same number. On seeds 2
   and 3 the upkeep never moved; on seed 1 it moved only after 1:50.
4. The Sprawler's walk changed with them: trail kept 0.92 (the Sprawler's
   0.8) and sensors 6 fine cells out (its 18), passed as engine keys.

Also tried, kept as keys: `LAB_GROW=G:r` (a child only where the box of
radius r round its cell holds G or fewer), `LAB_AHEAD=1` (a child ahead of
its parent), `LAB_DUMP=DIR` (each second's walkers a fine cell, for the
measure).

The pick: `LAB_LANES=3:1.0 LAB_SHUTTLE=3:0.9 keep=0.92 sensor_offset=6`.

## Results

Planet 2, run seed 1, lavapipe. The squad holds at 20,270, the map's far
corner, and buys 8 soldiers, so both arms play to 3:00; the camera is held on
the eastern body (390,165) at zoom 6, and at zoom 12 at (410,160).

**The measure.** In the camera's frame (fine cells x 284-496, y 98-232):
the body's outline is the cells where 5 or more of the 3x3 hold walkers,
closed over gaps up to about 40 cells wide by a disc of radius 20 and its
enclosed gaps filled. Of that outline: the share with a walker, and the
share that is hole (no walker in its 3x3). Over the whole slime: the share
of walkers in cells holding two or more.

| Seed, time | Arm | Walkers | Outline (cells) | With a walker | Holes | Walkers stacked |
|---|---|---|---|---|---|---|
| 1, 2:50 | off | 20,402 | 14,193 | 74.1% | 4.2% | 16.4% |
| 1, 2:50 | threads | 19,998 | 12,178 | 38.1% | 29.2% | 69.9% |
| 2, 1:29 | off | 13,932 | 9,877 | 69.3% | 8.2% | 1.0% |
| 2, 1:29 | threads | 13,923 | 7,298 | 46.8% | 24.7% | 77.4% |
| 3, 1:49 | off | 20,403 | 7,564 | 77.8% | 0.2% | 3.7% |
| 3, 1:49 | threads | 19,791 | 6,591 | 44.5% | 27.2% | 84.3% |

(Seeds 2 and 3: the squad held where it landed and was wiped at about 1:40
and 2:00 in both arms, so they are compared just before.)

**What it looks like.** On all three seeds the body round its crystal
becomes spokes: four to seven arms, 8 to 15 fine cells wide, running out
from the crystal to the body's edge and to the next body, with bare ground
between them. The off arm is a solid disc. At zoom 12 the arms are pale gold
ropes with lit rims (cells holding 2 to 4 walkers take the thick colour);
between them the trail's veil (`THREAD_VEIL`) tints the bare ground orange,
so the holes read as glowing ground, not dark. The clip (seed 1, 1:40 to
3:00, off left) shows the arms reach and swing as the traffic shifts; the
body never settles back to a disc.

**The walkers' number.** Seed 1: within 5% of the off arm in 158 of 179
seconds (median ratio 1.000). The misses are 2:21 to 2:40, after the off
arm's fruiting body burst at 2:21 and sowed 2,528 walkers at once (worst
-17.5% at 2:23); 3:00 ends at 20,012 against 21,643. Seeds 2 and 3: within
3% at the times above, with the upkeep never eased.

**The squad.** Seed 1, kills by 3:00: 8,659 threads against 7,999 off;
both squads standing (8 soldiers).

**Each part alone** (seed 1, `page/parts.jpg`):

- Lanes with the count held, no shuttle (keep 0.92): the body condenses
  into a dense lump and one cord to the next body, no holes in it.
- The shuttle without lanes: no change; the body stays packed, as spacing
  did, since a packed walker cannot step.
- Both, the Sprawler's own trail (0.8, sensors 18): two rough arms, a cross.
- Both, trail kept 0.95: the cleanest spokes (six), but the planet's
  fruiting lump gathered most of them by 2:50.
- Both, fed past half its birth energy (F 0.5): a rim cord round the body's
  edge with a thin lace of single walkers inside, the most thread-like
  picture, but the slime fell to about 9,500 against 14,000 at 2:00 even
  with the upkeep at x 0.1 (fed walkers leave the crystal before they breed).

**Frame cost** (lavapipe, bench 40 s, planet 2 seed 1, farm hand, every pass
timed alone, median of 30 steps; lavapipe is for checking, not timing):
`sense` 3.07 ms a step off, 4.03 ms threads (+0.96 ms; the two arms' walkers
are in different places, and the sensors read 6 cells out, not 18); `walk`
0.88 against 1.01; every pass 25.3 against 29.2 ms; the frame's p50 121
against 125 ms. The rule itself is one trail read for a blocked step and one
compare for the scent's sign.

## Recommendation

**Build it for the Sprawler, changed**: lanes 3 at 1.0 and the shuttle at 3
past 0.9 of the birth energy, the Sprawler only, with its trail at 0.92 and
its sensors at 6 (Enemy's numbers to retune by play). Before it lands:

- The view: the trail's veil fills the holes with orange; Look should judge
  whether the veil stays inside the body. The arms' thick colour is pale
  gold, the data (2 to 4 walkers a cell).
- The ceiling: a held count needs no help on seeds 2 and 3, but Enemy should
  check the ceiling's crystal cover, since fed walkers leave the crystal's
  middle bare at times.
- What the player feels: spokes leave lanes of bare ground into the body,
  so a squad can walk between the arms toward the crystal. Untested with a
  playing hand.

## Not tried

A shuttle by a slow clock instead of energy (half the walkers out, half in);
lanes only on fed walkers; a shuttle toward food the slime has not reached
(the far smell) for strands between crystals rather than spokes round one.

## Raw

`raw/run.sh` (the run: env keys, seed, zoom, centre), `raw/measure3.py` (the
table), `raw/measure.py` and `raw/occimg.py` (the dump reader and a picture of
the walkers a cell). Every run is deterministic on llvmpipe.
