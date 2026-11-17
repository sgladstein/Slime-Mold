# Lab: steering the slime into a kill zone

2026-11-17. The Lab lane's first prototype ([brief](../../handoff/lab.md)):
"a clip of 30 to 60 seconds on an Artifact page of a squad herding a slime
into a kill zone on one of planets 1 to 3, the counts beside it, and your
recommendation." The owner, 11-16 05:11Z: "I could image a gun that lets you
corral and push the slime onto areas. Into a kill zone".

Page: [Steering the Slime](https://claude.ai/artifact/FNGWU1K7kKNicmzFhR6zmC) (source in `page/`): the clips at zoom
6 and 12 with the counts beside each.

Status: **it works, and the recommendation is to build it into the game,
changed.** A lure that every walker leans toward pulls the planet 1 slime off
its home crystal into three turrets within 30 seconds: 4,158 walkers killed
by 1:30 against 1,623 without the lure, and three times the food rendered.
The same on planets 2 and 3. Everything here is one run seed a scene on the
software GPU, so the sizes are a first look and not a balance. The code is
on the branch `claude/project-thread-4tc75h` and is **not for main**: it
changes two shared shaders in place and borrows the far smell's slot.

## What walkers and fields respond to (the model, before any tool)

A walker turns toward whichever of three sensors (ahead, left, right) reads
highest. A sensor sums what the engine keeps a cell: the slime's own trail,
food scent weighted by the walker's hunger, the alarm (read as a repellent
or, in a panic, an attractant), the scar and crust. Three rules override the
sum: a panic or surge points the walker at a target, a raid walks it down a
distance field, a breakaway roams. Walls, rock and water are where a sensor
reads nothing; the crowd rule (one walker a cell) is what turns leaning into
flow.

So there are four ways to move a body that is not being shot:

| Lever | What exists today | What it does to a body |
|---|---|---|
| Scent | food, pumps (`pump_smell`), soldiers (`smell`), the Lure building | Draws only hungry walkers, and only within the scent's few cells. The Lure building at 285,100 drew 229 walkers within 15 cells at +10 s and 345 at +20 s, was eaten by +15 s and held 0 at +30 s. This is why D10 took it out of the run shop ("the lure drew nothing at 130 or 250 units"). |
| Alarm | shots, the Flare card, fire | Pushes, but as a side effect of a hit, briefly, and a panic reverses it. |
| A distance field | the raid's way to the units, the far smell (S2) to food | Reaches across the map and bends round rock and walls, because it is a walking distance. Only hungry walkers read the far smell; only raiders read the raid's. |
| Ground and walls | walls, doors, rock | Shape the flow but start none. |

The lever nothing uses for the player is the third: **a distance field every
walker reads**. That is the prototype.

## What was built

One more term in the three sensors: `weight x (field here - field at the
sensor)`, so every walker leans down the field (`engine.wgsl`, the LAB block
in `sense`; three extra field reads a walker a step, the same as the far
smell). The game builds the field once a game second on a worker thread
(`larder_world/lab.rs`, `steer_field`) from what stands on the map:

- a **lure**: the field is the walking distance to the nearest lure, out to
  a reach (60 dish cells here). Past the reach nothing changes.
- a **pusher**: a hill 25 dish cells of walking round it, so walkers lean
  away. It stands in for a building or a dart that does not exist yet.

The view draws the field itself: pink contour rings every 20 fine cells of
walking distance, brighter near the lure, a bright core on it
(`view.wgsl`, `steer_px`). Nothing is painted that the walkers do not read,
and where a wall bends the field the rings bend with it.

The field rides in the far smell's slot on the GPU, and a negative
`far_smell=-W` is its weight, so the prototype needed no new pass, buffer or
CPU twin (the owner's card, 11-17 19:21Z). That is also why it cannot merge
as it is: steering on means the far smell off.

The rig (any scene can be re-run from the branch):

```
LAB_MARKS="lure:30:275:95:60" LAB_LOG=1 larder-world look=DIR run=1:1 \
  mode=squads bank=20000 hand=hold@325,80 far_smell=-30 \
  turret2=275,75,20 turret2=292,106,20 turret2=258,106,20 \
  watch=275,95,30 centre=255,85,6 at=60,90
```

`LAB_MARKS` is `lure|push:FROM:X:Y:SECS;...` in world units (fine cells, as
`centre=` and `watch=`). A Lure or Siren building that is on and whole is a
lure or a pusher too. `LAB_REACH` and `LAB_PUSH` are in dish cells.

## What happened

The deciding measure, named before the runs: **walkers killed by 1:30 with
the lure against without it**, same planet, same run seed, same turrets and
squad. Every scene: run seed 1, squad mode, eight soldiers holding on free
fire beside the zone, three rendering turrets (level 2) placed at 0:20, the
lure from 0:30, weight 30.

### A lure with no guns (planet 1)

Walkers within 20 fine cells of the lure at 0:40, 0:50, 1:00 and 1:14:

| Weight | 0:40 | 0:50 | 1:00 | 1:14 | What it looks like |
|---|---|---|---|---|---|
| 3 | 316 | 679 | 573 | 601 | A blob forms on the lure; the body stays on its crystal. |
| 30 | 591 | 823 | 960 | 911 | The whole home body slides off its crystal to the lure. |

### Into a kill zone (planets 1 to 3)

Killed / food rendered / slime alive. No soldier was lost in any arm.

| Planet | Arm | at 1:00 | at 1:30 |
|---|---|---|---|
| 1 Drifter | lure | 3,344 / 834 / 12,525 | 4,158 / 1,038 / 11,142 |
| 1 Drifter | no lure | 419 / 74 / 15,246 | 1,623 / 344 / 15,915 |
| 2 Sprawler | lure | 5,707 / 1,252 / 19,528 | 9,030 / 2,061 / 17,373 |
| 2 Sprawler | no lure | 1,763 / 164 / 23,832 | 6,065 / 1,130 / 23,357 |
| 3 Trunk | lure | 3,164 / 781 / 41,269 | 5,695 / 1,414 / 26,764 |
| 3 Trunk | no lure | 201 / 20 / 44,081 | 2,124 / 482 / 29,560 |

Kills from 0:29 to 1:30: 4,147 against 1,612 (2.6x), 8,408 against 5,443
(1.5x), 5,664 against 2,093 (2.7x). Three of three the same way; a sign test
on three pairs is p = 0.25, so this is a direction, not a proof. On planet 3
both arms lost about 14,000 walkers between 1:15 and 1:30 for a reason
outside this test; it was not chased.

Places (world units): planet 1 turrets 275,75 / 292,106 / 258,106, lure
275,95, squad 325,80; planet 2 turrets 120,115 / 137,147 / 103,147, lure
120,135, squad 75,105; planet 3 turrets 345,148 / 362,180 / 328,180, lure
345,168, squad 400,205.

### The levers, on planet 1's scene, by 1:30

| Scene | Killed | Rendered | Slime alive | What it looks like |
|---|---|---|---|---|
| No lure | 1,623 | 344 | 15,915 | The slime reaches the turrets in its own time. |
| Lure, weight 30 | 4,158 | 1,038 | 11,142 | The home body empties into the zone by about 1:15, then nothing is left to draw. |
| Lure, weight 10 | 6,024 | 1,503 | 13,929 | A steady stream; the body stays home, keeps breeding and keeps feeding the zone. |
| The Lure building, weight 30 | 4,300 | 1,058 | 11,756 | The same as the mark until the slime eats the lure at about 0:58. |
| A fence with one gap, weight 30 | 6,036 | 1,508 | 14,051 | The field bends through the gap and the slime comes out of it as a jet onto the turrets. |
| A pusher alone on the home crystal | 1,876 | 415 | 13,195 | The crystal is cleared and the slime piles at the hill's rim; it does not feed the zone by itself. |

The weight is a design knob with two characters. At 10 the lure milks a
colony: more kills and more food by 1:30 than at 30, because the body is
never pulled off the crystal that breeds it. At 30 it evicts: the home
crystal is bare in about 45 seconds. The fence shows what the walking
distance buys: the player shapes the flow with walls already in the game.

## Cost

Three field reads a walker a step when the weight is not zero, the same as
the far smell. The field is two block searches a second on a worker thread
(the far smell's one measured 8 to 10 ms on the core world, off the frame;
S2). The view adds a branch a pixel and four reads where the field reaches.
Nothing was timed on the 1070: the software GPU is for checking and
filming.

## Recommendation: build it, changed

Build the steering field into the game as the Lure's rule. It is the first
thing the player owns that writes a direction, it reads at a glance, it
turns walls and turrets already in the game into a plan, and it meets D10's
condition for the lure coming back ("stay out until they visibly draw the
slime").

What changes from the prototype, by lane (a proposal for Direction to place
in a milestone, not a job anyone has been given):

- **Enemy**: the field gets its own slot and its own weight, so the far
  smell and steering can both be on, and a per-tier weight: how hard a
  slime follows a lure is a trait (a Trunk that barely leans, a Swarmer
  that stampedes). Decide whether a panic or a surge ignores the lure (in
  the prototype the overrides still win; that was not looked at on its
  own).
- **Squad and arsenal**: the Lure building returns to the run shop with
  this rule; the pusher needs a carrier (a dart, a thrown canister or the
  Siren). The lure should be mortal and short: the slime ate the building
  in 28 seconds, which is a fair clock.
- **Look and interface**: the rings' colour. The prototype's pink sits in
  the hue the art direction keeps for the core planet's flora, and "a new
  colour needs a role nobody has". The rings at zoom 6 and 12 are on the
  page.
- **Direction**: the weight and the price. Weight 30 with three turrets
  removed a planet 1 home colony in 45 seconds for no losses, which is too
  strong for a 300-food building; weight 10 is the better first number.

Risks seen: at weight 30 the lure is a win button on planet 1; a lure next
to the squad would pull the whole body onto the soldiers (not tried, and
maybe that is the player's mistake to make); the pusher's rim is where the
slime piles up, so a pusher alone moves a problem a few cells.

## Not tried

Steering by bullets (a trail of marks laid along a shot, the "gun that
corrals" in the owner's words), ground that slows or speeds walkers,
leapfrogging lures to walk a body across the map, a lure against a
mini-boss, and any planet past 3. Each fits the same field.

## Files

- `page/`: the page's source, clips and poster frames (`index.html`; the
  clips are made by `raw/film.sh` on the lab branch's build, its header
  lists each clip's command).
- `raw/`: `run.sh` and `film.sh` (the rig), and `logs/`, each filmed run's
  log (a count line a frame; the page's counts are the lines at the whole
  second). The filmed runs repeat the tables' runs exactly.
- Code: branch `claude/project-thread-4tc75h`, commits "LAB: steering field
  prototype" and "LAB: the lure's core glow"; module
  `src/mould/larder_world/lab.rs`.
