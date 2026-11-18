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

## Part two: a gun that lays the pull (2026-11-18)

Scott's card, 11-18 03:03Z: "Lure now, in the three planets" (the lure
goes into the game, built by Enemy, Squad and arsenal, and Look). Direction
left the lab steering by bullets, the item closest to his words: "a gun
that lets you corral and push the slime onto areas".

**Built** (lab branch, commit "LAB: shots lay marks"): each soldier's
bullet that hits slime writes into the same field for `LAB_SHOT_SECS` (3):
`LAB_SHOT=push` a pusher on the hit, `lure` a lure `LAB_SHOT_AHEAD` dish
cells (30 here) past the hit along the bullet's line, `drive` both.
Turrets lay none. The field is now rebuilt every `LAB_EVERY` game seconds
(0.25 here) in `Game::lab_turn`, still one period late; at most 256 marks.

**Scene** (planet 1, run seed 1): the squad holds at 325,80 and walks at
0:55 to 258,47, east of the colony on the north crystal (210,47); three
rendering turrets stand on its west side at 172,30 / 165,55 / 180,78.
Nothing is placed by hand except in the dart rows. By 2:00:

| Arm | Killed | Food rendered | Slime alive |
|---|---|---|---|
| Plain fire | 10,533 | 1,282 | 11,460 |
| Push on the hit, pull 30 | 8,543 | 1,434 | 13,058 |
| Push on the hit, pull 10 | 8,947 | 1,478 | 14,102 |
| Push, 12 dish cells, pull 30 | 9,625 | 1,356 | 13,230 |
| Lure 30 ahead, pull 30 | 1,565 | 254 | 10,386 |
| Lure 30 ahead, reach 25, pull 10 | 4,618 | 702 | 10,551 |
| Both, pull 30 | 2,020 | 408 | 9,930 |
| Both, reach 25, push 12, pull 10 | 10,005 | 1,520 | 10,867 |
| One lure dart into the turrets (172,55) at 0:55 for 10 s, pull 10 | 10,403 | 1,436 | 12,715 |
| The same, pull 30 | 10,306 | 1,425 | 11,987 |
| Three darts, 0:55, 1:15, 1:35, 8 s each, pull 10 | 10,661 | 1,533 | 11,736 |
| The same, pull 30 | 10,411 | 1,566 | 11,212 |

The three clips on the page (`page/shots-*.mp4`, 800 x 500, about 1.7 MB
each) are separate filmed runs of plain fire, "both, pull 10" and "lure
ahead, pull 30"; their counts are on the page from their own logs
(`raw/logs/shots-*.log.gz`) and differ from the table's runs by about 1%
at the same time (frames every 1/7.5 s, where the table's runs took six
stills).

**Result: no bullet steering card.** No arm beat plain fire on kills.
A bullet lands where the slime is nearest the squad, so the marks gather
round the squad: pushes make the body back off the squad's fire (fewer
kills, more slime alive), lures along the line pull it about in front of
the squad. Placement is what herds, as in part one. Here the squad already
stood in range of the body, so even aimed darts added only 12 to 20% to
the turrets' food and about nothing to the kills.

**A lead: evicting starves.** With lures 30 ahead at pull 30 the colony
left its crystal before the squad arrived and chased its own marks across
the field: 1,565 killed by 2:00 against 10,533, yet 10,386 walkers alive
against 11,460. Off its crystal it stops growing. A strong lure that drags
a colony off its food is another way to shrink it, and fits the Lure now
going in.

**Recommendation.** Do not build a bullet that lays the pull. If the
player is to steer with the gun, the gun carries the lure: a dart or a
thrown canister placed by aim, the same lure as the building. Next in the
lab, if Direction agrees: the eviction lead (a lure that pulls a colony
off its crystal, and how fast it then shrinks), or the pusher's carrier.

## Part three: the dart (2026-11-18)

Direction's pick after part two: the carrier, "a dart from the gun that
sets a lure or a pusher where the player aims", shown at the game's pull of
10 (the number Enemy is porting).

**Built** (lab branch, commit "LAB: the dart"): `LAB_DARTS=lure|push:T:X:Y:SECS;...`.
At game second T squad 0 fires from its soldiers' middle toward X,Y; the
dart flies at `LAB_DART_SPEED` (60 dish cells a second), over slime and
walls, no farther than `LAB_DART_RANGE` (60 dish cells), and stands SECS as
a lure or a pusher in the same field. Each dart is logged
(`lab dart t ...: pusher from 291,49 to 232,47, lands at 30.47 s`).

**Scene** (planet 1, run seed 1): the squad holds on the small crystal at
290,47; the colony sits on the north crystal at 210,47; three rendering
turrets stand on its far (west) side at 172,30 / 165,55 / 180,78. A dart
every 10 s from 0:30, each standing 10 s: pushers aimed at 232,47 (the
colony's near edge), lures at 172,55 (between the turrets). By 2:00
(walkers within 30 of the colony's crystal, killed, rendered, slime alive):

| Arm | On its crystal | Killed | Rendered | Slime alive |
|---|---|---|---|---|
| No darts | 2,438 | 8,612 | 1,444 | 13,468 |
| Pusher darts, pull 10 | 1 | 3,515 | 800 | 10,547 |
| Pusher darts, pull 30 | | 2,745 | 616 | 10,443 |
| Lure darts, pull 10 | 0 | 3,429 | 787 | 10,269 |
| Lure darts, pull 30 | 0 | 3,148 | 720 | 10,286 |
| Both, pull 10 | | 2,988 | 683 | 10,261 |

On its crystal through the run, pull 10 (0:30, 0:40, 0:50, 1:00, 1:15):
no darts 610, 1,019, 1,312, 1,563, 1,768; pusher darts 610, 670, 516,
291, 0; lure darts 610, 596, 151, 0, 0. Blank cells were not counted in
that run (its watch was on the turrets).

Clips on the page (800 x 500): no darts, pusher darts and lure darts at
zoom 6 from 0:26 to 1:26, and the first pusher darts at zoom 12 in real
time (0:29 to 0:44); their counts come from their own logs
(`raw/logs/dart-*.log.gz`).

**Result.** At pull 10 a dart every 10 s takes a crystal off a colony in
30 to 45 s and leaves the planet's slime 22% smaller by 2:00. It costs
kills and food: only part of the body crosses the turrets, the rest
scatters and withers off its crystal, and the squad has less in front of
it to shoot. Pull 30 adds nothing here (the same slime, less food), and
both darts together do no better than either.

**Recommendation.** Build the dart as the Lure's second form, with a
pusher setting: the lure dart when the turrets are in its line, the
pusher dart from the near side. Range 60, life 10 s and one dart every
10 s are first numbers, not tuned; the price and a cooldown are
Direction's. The pusher draws no core glow (by design, `PUSH_BASE`), only
faint rings at its rim, so Look may want to give it a mark of its own.

## Part four: after the colony leaves (2026-11-18)

Direction's question: once the pull takes a colony off its crystal, does
it starve, resettle on another crystal, or come for the squad, and does the
planet end sooner or later? Part three's scene at pull 10, followed to 5:00
over the whole planet (`watch=210,47,30`, the north crystal):

| Arm | Soldiers 2:00 / 5:00 | Slime alive 2:00 / 5:00 | On its crystal 2:00 / 5:00 | Killed 2:00 / 5:00 |
|---|---|---|---|---|
| Turrets, no darts | 8 / 0 | 13,468 / 22,405 | 2,438 / 3,534 | 8,612 / 20,918 |
| Turrets, four lure darts 0:30 to 1:00 | 8 / 8 | 10,374 / 16,513 | 23 / 2,955 | 3,597 / 16,731 |
| Turrets, a lure dart every 10 s to 4:50 | 8 / 8 | 10,269 / 14,859 | 0 / 8 | 3,429 / 5,214 |
| No turrets, no darts | 7 / 0 | 16,106 / 26,195 | 3,238 / 3,883 | 3,627 / 8,583 |
| No turrets, four lure darts | 7 / 0 | 16,106 / 26,134 | 3,254 / 3,470 | 3,211 / 9,058 |
| No turrets, eight lure darts | 7 / 0 | 16,220 / 24,649 | 3,163 / 3,485 | 2,706 / 9,631 |

The three turret arms are filmed on the page (`page/evict-*.mp4`, the whole
map, 5:00 in 10 s); the filmed runs repeat these counts exactly.

**Result.** The colony neither starves nor comes for the squad: most of it
dies in the turrets, and the crystal stays empty only while darts keep
coming. When they stop, an arm of the big south-west colony settles it
again within one to two minutes (23 walkers at 2:00, 1,185 at 3:00, 2,955
at 5:00), past the turrets, which feed on it. With darts all along the
crystal stays empty and the slime settles a crystal in the south-east
instead. The planet's slime grows back in every arm, more slowly with
darts; none ends the planet in 5:00. What changes is the squad: with no
darts the north colony overruns it between 3:00 and 5:00; with darts all
eight soldiers live (health 54 and 94 at 5:00).

**Without turrets the lure darts did nothing**: the colony raided the squad
and the raid overrides the lure (as every override does in the prototype),
so the squad was wiped in all three arms. A lure only herds where guns
stand where it leads, and where no raid is under way; whether a lure should
outrank a raid is a question for Enemy's port.

**Verdict.** A crystal taken by the lure is held only while the player
keeps luring, and only with guns where the lure leads: the dart is a tool
for holding ground, not for clearing a planet. Steering ends here as a lab
subject (Direction, 11-18 03:31Z).
