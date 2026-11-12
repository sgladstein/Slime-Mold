# Soldier controls for the Larder world, part 1 (2026-11-12)

The owner asked (2026-11-12 04:22Z) for better soldier controls and
commands in `larder-world`: "How I order them to harvest. How I order them
to build. Etc". The scheme was proposed in the thread at 04:30Z; the owner
picked "soldiers build over time" on the card at 04:41Z (not built yet: part
2, after the buildings thread's shared `Building` type). Then (04:42Z): "I
want to be able to zoom in more too. That was limited in my play test".

## What part 1 adds

Rules (`mould::larder_world`):
- **An order queue.** `Cmd::Queue(k, order)` puts an order after soldier
  `k`'s current one (`Soldier::queue`, up to `QUEUE` 8); a holding soldier
  starts it at once; a plain `Cmd::Order` drops the queue. A move or an
  attack-move that has arrived starts the next; a harvest order goes on
  once its crystal runs dry and the load is banked. With nothing queued a
  move keeps its order as before (the player AI reads it).
- **Attack-move**, `Order::Fight(x, y)` (in the shared `larder::Order`;
  M3 plays it as a move): walk there on free fire, standing to fight while
  mould it can hit is in sight (`Game::engaged`).
- **Moving on from a dry crystal**, `Rules::seek`: after banking (or at
  once, empty-handed) the soldier takes the nearest crystal with a frame's
  harvest left (`Game::seek`), one the mould has not eaten from first, never
  the mould's home.
- **The larder's rally point**, `Cmd::Rally(order)` / `Game::rally`: a
  bought soldier starts with it.
- Replays: `queue k ...`, `rally ...` and `order k fight x y` lines; the
  round trip is tested.

Window (`larder-world`) and controls (`world::controls`):
- Shift + any order (right click, G, A, F) queues it; the selection's queue
  is drawn as a chain of lines, red for fighting.
- A is now a targeted command: click a point to attack-move, click the core
  to attack it as before.
- Click the larder to select it: its card shows the bank and where new
  soldiers go; right click (or R, a new command) sets the rally point, on a
  crystal they start harvesting it.
- Double click a soldier: every local soldier on screen.
- `MAX_ZOOM` 8 → 24 pixels a cell (steps 12, 16, 24 added).

Cost: nothing new runs on the GPU; the rules add a queue check and, on a
dry crystal, one pass over the crystals. Not timed.

## The census: moving on, on and off

The Larder thread asked that `Rules::seek` either be a player's choice or
pass the gate (`Rules::home_dry` was measured under score v2). The deciding
measure, named before the run: the gate (farm and wall beat idle and random,
sign test, Holm), then paired farm and wall scores, seek on against off.

`larderworld census seeds=5-20 cap=900 hands=idle,random,farm,rush,wall`
with `seek=1` and `seek=0` (raw: `soldier-controls-2026-11-12/`):

| hand | mean score, on | mean score, off | on vs off, paired |
| --- | --- | --- | --- |
| farm | +0.141 | +0.048 | 8 higher, 7 lower, p 1.0 |
| wall | +0.120 | +0.152 | 2 higher, 14 lower, p 0.004 |

The gate holds both ways (farm and wall beat idle 14-15/16 and random 16/16,
all hold under Holm). Wall's loss survives a Bonferroni correction over the
two comparisons; the mould ends bigger on most of its seeds with seek on
(inferred: its farmers walk on past its wall instead of holding behind it).
That first version was not kept. Two reviews (the game designer and the
technical lead, 05:25Z) objected to the split it led to (on in the window,
off for the hands): the census would no longer measure the game played.

### The safe version, one default

`Game::seek` now picks only a safe crystal: a frame's harvest left, not the
mould's home, none the mould has eaten from, none nearer the core than the
larder; of those the nearest the larder that no other soldier works, else
the nearest the larder; none safe sends the soldier home to hold. Same
census with it on (`census-seek-safe.txt`), against off:

| hand | mean score, safe on | mean score, off | on vs off, paired |
| --- | --- | --- | --- |
| farm | +0.217 | +0.048 | 10 higher, 5 lower, p 0.30 |
| wall | +0.052 | +0.152 | 4 higher, 11 lower, p 0.12 |

The gate holds (farm 16/16 over idle and random, wall 15/16, all hold
under Holm). Neither difference is significant, so under "defaults on" it
is **on** for every hand and the window alike. Wall still leans lower (its
mean drops 0.10; it lost the squad on 2 seeds, against 1): re-test it when
soldiers raise the wall (part 2) or the hands plan with moving on.

## Next

Part 2: soldiers raise buildings over time (`Order::Build`, the buildings
thread's `Building`/`Game::raise`), walls included; harvesters carry to the
nearest depot once outposts exist. Not built from the proposal: right click
on mould as attack-move (needs a mould read under the cursor on the GPU
world) and a line formation for group moves.

## Part 2: soldiers raise buildings (2026-11-12)

The owner's card (04:41Z): "Soldiers build over time". On the buildings
thread's shared `Building` (#112: a ghost at `built` 0 holds a tenth of its
health and gains the rest as it rises; it acts only when whole; a wall's
cells go into the world only then):

- `Order::Build(b)`: the soldier walks beside the ghost (for a wall, to the
  nearest point of its line) and, within 3 dish cells plus its sunflower
  slot, raises it `dt / Game::raise_secs` a frame; builders add up. Once it
  stands or is gone the soldier takes its next queued order, else holds.
- `BuildRules::raise` 0 → **8** seconds for a lone builder; a wall takes 8
  per 10 dish cells of length (`WALL_RAISE_LEN`), at least 8.
- An idle soldier (holding, nothing queued) within its range takes up the
  nearest ghost by itself (`Game::build_turn`), so a ghost placed by an
  idle squad rises without another order.
- The window sends the selected soldiers to raise what it places (Shift:
  after their orders). Harvesters carry home to the nearest depot, the
  larder or a standing outpost (`Game::drop_offs`).
- The census hands send their nearest soldier to raise what they place
  (`send_builder`), and the farm hand leaves a builder be. Without that the
  first run (raise 8) broke the hands built for instant buildings: wall
  fell to -0.309 and no longer beat idle (5-3), its hand placing a fresh
  wall every second while the first was a ghost.

Census, seeds 5-12, cap 900, main's defaults (free shots), raise 8 against
0 (raw: `census-raise-8.txt`, `census-raise-0.txt`):

| hand | mean, raise 0 | mean, raise 8 | 8 vs 0, paired |
| --- | --- | --- | --- |
| farm | +0.201 | +0.201 | 0-0 |
| wall | +0.196 | +0.189 | 4-4 |
| ring | +0.201 | +0.184 | 4-4 |
| turtle | +0.325 | +0.236 | 0-7, p 0.016 |
| killzone | +0.209 | +0.200 | 3-5 |
| guard | +0.338 | +0.247 | 1-7, p 0.07 |
| pumps | +0.240 | +0.200 | 1-7, p 0.07 |
| army | +0.336 | +0.336 | 0-0 |

Every plan beats idle and random 8-0 either way (all hold under Holm).
The cost falls on the turret plans (turtle, guard, pumps), which the
buildings census found best on every tier; army, the same food on
soldiers, is now the top mean. Raising is labour and risk the player
spends, as the owner chose; on by default. Turtle's loss is significant
before correction only (8 comparisons: Bonferroni 0.006).

### Part 2 at the full bar (seeds 5-36, 201-232 and two maps)

Census at main a8669bd, cap 900, all ten hands (raw:
`census-122-race-5-36.txt`, `census-122-race-201-232.txt`,
`census-122-fork.txt`, `census-122-nest.txt`). On the race world every plan
beats idle 32-0 on both seed blocks, and random 30-2 (5-36) and 24-8 or
better (201-232), all under Holm. On the Nest map (seeds 5-20) wall, ring,
turtle, guard and pumps beat idle 16-0 or 15-1; farm (11-5) and army (12-4)
beat it but not significantly, and **killzone loses to idle 0-16**: a finding
for the buildings thread, not a controls bug (the hand sits its turrets
where the Nest's mould is not). The Fork run was stopped at seed 17 under
the owner's new testing rule (17:26Z, census is not a gate): every plan
beat idle and random 13-0 on the seeds it finished.

## Part 3: soldiers can't stand on a crystal (2026-11-12)

The owner, 14:43Z: soldiers "also shouldn't be able to stand on top of the
crystal". The Larder thread takes the other half of that message (heavy
slowdown in thick slime).

- `Rules::crystal_solid` (on): a crystal with charge left is ground a
  soldier can't stand on (`Game::standable`). Walking, the straight-line
  check (`walk_line`) and the distance maps use it; the line of fire
  (`clear_line`) does not, so shots still fly over crystals.
- A goal on a crystal (a harvest order, a move click, a larder or ghost
  placed on one) moves to just past its rim on the soldier's side
  (`Game::rim`, `RIM` 1 dish cell). Harvest, banking and building reach
  count from that rim (`Game::reach`). Spawns step outward from the larder
  until they are off its crystal. A soldier caught on one walks straight out.
- The solid disc holds the charge left: radius × √(charge / start charge).
  The first version used the full radius, and a harvester ate the rim (its
  take disc reaches 4 dish cells in) and then could not reach the middle:
  the dry-crystal test banked nothing. Now the soldiers eat in from the edge
  and step in after it, which is what the picture shows (bites out of the
  hexagons, soldiers at the edge: `crystal-rim.gif`, farm hand, seed 5,
  0-45 s, a frame a second).

Tests: `soldiers_stand_round_a_crystal_not_on_it` (spawns off the crystal,
no soldier on a solid disc over 40 s, a mover stops at the rim, the
harvester banks, and with the rule off the mover reaches the middle). The
cost per step: one `crystals()` copy, and a disc check per soldier move and
per distance-map cell when a map is built (every 30 s).

Smoke check (not a gate, under the owner's 17:26Z rule), race seeds 5-15,
on against off (raw `census-solid-on.txt`, `census-solid-off.txt`, stopped
part way): no game ends in the first minute; idle loses every seed (mean
-0.34 either way); random is wiped on most seeds either way. Harvests are
about even (farm 49.5k against 49.9k). Scores run a little lower with the
rule on for most plans (ring 1-9, pumps 0-10, farm 3-7, wall 5-5, army
3-7): walking round crystals costs time. Balance later, by the owner's
rule; on by default since the owner asked for it.

**The rule on both engines** (tech lead's ask after #153): to get CI green,
#153 turned the rule off in three long GPU twins (flee, craters, core).
There, seed 3's moulds parted by 9.7-12% once the squad moved. A short twin
now checks the rule itself. `solid_crystals_hold_the_squad_alike_on_both_engines`
runs race seed 3 with the squad holding fire and ordered onto the larder's
crystal. Every soldier is at the same place on both engines, to the bit,
for all 45 ticks. None stands on a solid crystal, and all four reach the
rim. The planted copy (`the_crystal_twin_catches_an_engine_without_the_rule`)
plays the GPU game without the rule, and the twin catches it at tick 1. So
the rule acts the same on both engines. The long twins' drift is the
mould's float drift amplified by a squad that stands elsewhere.
