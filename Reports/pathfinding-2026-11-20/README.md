# How squads and soldiers find their way (2026-11-20)

Squad and arsenal, the owner's ask (11-20 22:06Z): "review and improve
squad/individual solider pathfinding". Five faults found and fixed. The worst
was a squad steered past a rock. It stood still for the rest of the planet
with the way round in plain sight. Code: `Game::steer_point` and `Game::guide`
and `Game::slide` (`squads.rs`), `squad_way` and `want_round` (`steer.rs`),
`onto_map` (`paths.rs`).

## Method

- **`examples/pathprobe`** plays a run planet (`levelgen::level(seed, depth)`)
  on the CPU dish in squad mode with few walkers (8000), so the soldiers live
  to walk:
  - 60 soldiers in 4 squads.
  - Squad 0 is steered with eight-way keys at the furthest crystals, like
    WASD. It changes key only when the way is more than 30 degrees off, as a
    player would, and takes the next crystal when it gets within 12 cells.
  - The other squads get a move, an attack-move and a drain to a random
    crystal by turns, every 20 s.
  - At 60 s squad 1 splits. At 120 s both halves are sent to one point and
    merged once they overlap.
  - Every step goes to a CSV (`Game::path_rows`), with the ground as a PGM.
- **`tools/paths.py`** counts the faults from the CSV:
  - unplaced: standing over 2 cells from its goal for 3 s or more
  - strays: standing far from its squad
  - dither: walking back and forth
  - strung out
  - stalls: the anchor not moving for 3 s with a goal
  - trips: walked length against straight length, pace, unfinished
- **`tools/compare.py`** pairs two batches by map and gives a sign test.
  "Reached" counts the steered squad's arrivals, from the probe's log.
- **`tools/tracks.py`** draws each soldier's track over the ground, which is
  how each fault below was found and checked.
- **`tools/batch.sh`** runs planets 1 to 4 on run seeds 1 to 5 and
  1795038793, which is 24 maps, for 90 game seconds.
- **The real game.** `LARDER_PATHLOG=FILE` makes headless runs write the same
  rows, and `hand=steer@X,Y[,N]` buys N soldiers and steers them eight-way at
  X,Y (world units), as a player holds keys.
  - Films: `larder-world look=DIR run=1795038793:2 mode=squads hand=steer@172,208,15 centre=110,170,6 at=...`
  - Planet 3: `run=1795038793:3 hand=steer@338,270,15 centre=230,215,6`

## What was wrong, and the fixes

1. **Steered past a rock's tip, the squad stood for good.** On planet 2,
   seed 1795038793, 15 soldiers steered south-east from the landing stood at
   a bar's tip from 5 s to the end, 175 s.
   - The anchor heads for a point 40 cells ahead. That point was in rock, and
     a walking map grows from its goal over open cells only, so it reached
     nothing. The anchor slid along the grid alone into the corner.
   - Fix: `steer_point` takes the furthest standable point on the same line.
2. **Steered along a slanting wall, the squad shuffled in place.** On planet
   3, seed 1795038793, steered south-east from 12 s to the end.
   - The anchor stepped straight where the next step was open, and back up
     the walking map where it was not. The two steps cancelled.
   - Fix: a steered squad that took its map last step keeps to it until the
     whole line ahead is open (`guide`'s `whole`).
   - Also, `slide` no longer counts a zero-length "slide" along an axis that
     doesn't move, and tries the step turned 45 degrees each way, so a squad
     steered into a slanting face slides down it.
3. **A sent squad's anchor stood next to a crystal while its members walked
   off.** On planet 3, run seed 1, at 80 s.
   - The middle's cell was on no walking map (beside a crystal), so the
     anchor slid straight instead of taking the map.
   - Fix: `onto_map` uses the best map cell within 8 cells.
4. **A member behind a crystal from its steered anchor stood still 77 s.**
   - It went down the map to the point ahead, which led round the other way,
     and that map read it as only 3.7 cells behind.
   - Fix: members of a steered squad take the map to the anchor itself
     (`squad_way`).
5. **A soldier in a standing squad stood 25 s, 17 cells off, behind a
   crystal.** The map's goal is any point within 12 cells of the squad's
   point, so it read 1.3 cells behind, under its place's 8.
   - Fix: a soldier counts itself no nearer than its straight distance,
     unless it is ahead of a walking anchor (`want_round`).

## Numbers

24 maps, 90 s each, the same maps before and after
([compare-final.txt](data/compare-final.txt)):

| Measure | Before | After | Maps better / worse | Sign p |
|---|---|---|---|---|
| Steered squad's arrivals at a crystal, per map | 1.9 | 4.8 | 19 / 0 | < 0.001 |
| Back-and-forth walking (share of soldier-steps) | 9.7% | 4.9% | 19 / 5 | 0.007 |
| Standing away from its place (share) | 6.4% | 4.4% | 17 / 7 | 0.064 |
| Anchor stalls, per map | 0.75 | 0 | 2 / 0 | 0.5 |
| Strays (share) | 0.06% | 0.00% | 3 / 0 | 0.25 |
| Walked / straight on a sent trip (median) | 0.98 | 0.99 | 7 / 16 | 0.09 |

Over the nine measures, arrivals survive a Bonferroni correction (p < 0.0056)
and back-and-forth walking just misses it. Sent squads' trips are the same
length and pace, and the same number end unfinished at 90 s (3.4 against
3.3). They were not where the faults were.

The real game agrees.
- On planet 2, before, the steered squad stood at 89,138 to the end of the
  film. After, it reaches the crystal at 160,193.
- On planet 3, before, it stood at 175,195. After, it reaches 322,256.

Films, before on the left: [planet 2 at zoom 6](films/p2-pair.mp4),
[planet 2 at zoom 12](films/p2z-pair.mp4),
[planet 3 at zoom 6](films/p3-pair.mp4). Stills:
[p2](films/p2-still.jpg), [p2 zoom 12](films/p2z-still.jpg),
[p3](films/p3-still.jpg).

Frame cost: `steer_point` checks at most 40 points a steered squad a step.
`onto_map` reads at most 288 cells, and only when a map is taken from off its
cells. The walking map is built no more often than before.

## Tried and not kept

- **A member of a standing squad whose place is free walks at full speed
  with no hold-back.** `a_squad_crosses_planet_8_and_settles` went from
  settling in under 10 s to 31 s, because members walked through each other
  at the end. Replaced by fix 5, which changes only how far off a soldier
  counts itself.
- **A steered squad slides along the grid instead of taking the map when the
  map's way is over twice the straight distance** ("DETOUR", meant to keep a
  steered squad from walking far back the way it came).
  - It left a squad at a slanting wall on planet 4, seed 1795038793, that
    goes round without it: unplaced 15.8% with it, 2.9% without, and one
    arrival without it against none. On planet 4 seed 3, unplaced 11.4%
    against 1.2%, and no arrivals against two.
  - Removed. With it, arrivals were 4.2 a map ([with DETOUR](data/compare-with-detour.txt)); without it, 4.8.

## Looked at and left

- **Soldiers whose place is in rock** stand at the place's distance from the
  anchor instead. This is by design (the 11-16 rework), not a fault.
- **Long ways round lakes and fords** on planet 3 seed 19 and planet 4 took
  the crossings a player would.
- **Bots merge squads at any distance**, while the window's J needs them to
  touch. The probe merges only once they overlap. Worth knowing when reading
  bot runs.
- **Walking maps do not avoid slime or fire.** Tried next, below, and not
  kept.
- **The maps are rebuilt every 30 s**, so a crystal eaten since then still
  blocks its cells. No squad was seen held by one in these runs.

## Routing round slime and fire: tried, not kept (2026-11-21)

The coordinator's follow-up: make squads route round slime and fire.

**What was built** ([patch](avoid-experiment.patch), `avoid.rs`,
`Rules::avoid`). A squad on a plain move whose way ahead crossed slime
replanned its way once a game second on 4-dish-cell squares. Each square cost
more for each walker on it (`World::mould_grid`) and for fire seen there in
the last 8 s: burn and light bursts, and soldiers that felt heat. The anchor
headed for the furthest point of that way it could walk to in a straight
line. Attack-moves, drains and steering were unchanged.

**Measured in the real game** (GPU, run seed 1795038793, 20 soldiers sent
from crystal to crystal, `hand=movetour,20[,clear]`, 120 s, the same
binary with `set=avoid=0` as the pair). The three numbers in each cell are:
- the share of the way with walkers on a soldier
- health lost per soldier per trip
- the median trip time

| Run | As before | Round all slime | Round thick bodies only |
|---|---|---|---|
| Planet 3, far crystals | 10.3%, 8.0, 37 s | 10.7%, 6.6, 40 s | 9.9%, 7.8, 40 s |
| Planet 2, crystals clear of slime | 15.1%, 9.0, 19 s | 17.3%, 9.6, 20 s | 16.7%, 9.8, 19 s |
| Planet 3, crystals clear of slime | 4.3%, 2.3, 11 s | 5.1%, 3.0, 13 s | 7.4%, 5.2, 13 s |

None of the six pairs was clearly better, and the trips took longer. The
films show why:
- On planet 3 the far crystals sit inside the body, so there is no way
  round.
- On planet 2 loose walkers stream over the whole map and come to the squad
  wherever it walks.

A longer way only means more time among them. Fire was never seen in these
runs, so routing round it was not tested. Revisit only if the slime stops
coming to the squad (a slime that holds still, or one held behind walls),
or for fire alone with a hand that lights it.

Tools added for this:
- `hand=movetour,N[,clear]`
- `examples/pathprobe orders=` (m, f and c)
- [tools/slime.py](tools/slime.py), which measures what a sent squad walks
  through

The CPU probe's slime of 25,000 walkers touched sent squads on only 1 to 3%
of the way, against 10% in the real game. Measure this on the GPU.

## Walking round fire on a move: kept (2026-11-21)

Direction's call: a soldier standing to fight in fire stays (burning is the
price of the Needle Blast deck), but a squad given a move order should walk
round burning ground where there is a way round. It didn't: squads walked
straight through. Built and kept, on by default (`fireway.rs`,
`Rules::fireway`, `set=fireway=0` turns it off).

**How it works.** The GPU sends back no fire field, only each soldier's
heat, so the game keeps fire marks: a burn or light burst (`Rounds::flush`)
or a soldier feeling heat (`turn.rs`) marks its spot hot for 15 s. A squad
on a plain move whose next 40 cells cross a mark plans a way round once a
game second on 4-cell squares. A square is hot if its middle is within the
mark's reach plus 2 cells and a quarter of the squad's width, and costs
twice a cold one. The anchor heads for the furthest point of that way it can
walk to in a straight line crossing no more hot squares than the way does.
Within 10 cells of where it was sent it walks straight in. Attack-moves,
drains, steering and standing squads are unchanged.

**Measured.**
- CPU probe (`examples/pathprobe fire=6`, a 6-cell fire lit halfway along
  every move order, marked only where it caught), the 24 maps, 120 s
  ([fire-compare.txt](data/fire-compare.txt), [tools/fire.py](tools/fire.py)):

  | Measure | Off | On | Maps lower / higher | Sign p |
  |---|---|---|---|---|
  | Health lost to fire on the way (no walkers on the soldier) | 413 | 274 | 24 / 0 | < 0.001 |
  | Health lost to fire, all | 1400 | 1212 | 22 / 2 | < 0.001 |
  | Trip time, median s | 10.6 | 10.6 | 12 / 11 | 1.0 |
  | Move trips finished | 16.3 | 15.9 | 9 / 0 | 0.004 |

  Both burn lines and trips finished survive a Bonferroni correction over
  four. A way round is a little longer, so a few trips that go round don't
  finish before the probe sends the squad on (every 20 s).
- Real game (GPU, run seed 1795038793, 30 soldiers sent crystal to crystal
  with a burn lit halfway on ground, `hand=movetour,30,clear,fire`, 90 s;
  one run each, and the tours part after the first trip):
  - Planet 1: health lost to fire while walking 515 off, 337 on; standing
    180 against 53. Trips took longer (median 3.3 s against 6.9 s; 16
    against 13 trips): the squad tours a tight ring of crystals at the
    map's west edge and the fires add up there.
  - Planet 2: the burns barely caught (8 health off, none on); 10 trips
    each, median 5.1 s against 6.4 s.

Film, the second trip on planet 1 (10 to 23 s), the burn lit halfway along
it, off on the left: [zoom 6](films/fire-pair.mp4),
[zoom 12](films/fire-z12-pair.mp4), stills [zoom 6](films/fire-still.jpg)
and [zoom 12](films/fire-z12-still.jpg). Off, the squad walks through the
burning ground between two crystals and loses 139 health to fire on that
trip; on, it goes south of the crystal and loses 20.

**Tried on the way, not kept.**
- *Wider marks and dearer fire.* Half the squad's width plus half a square,
  a hot square costing 3: fire on the way 248 on the 24 maps (against 274
  now), but trips took 11.4 s against 10.6, and on planet 1 of the real
  game 12.2 s against 6.9. A cost of 6 and 3 cells of spread went further
  still (13.4 trips finished a map against 16.3 off). Half the squad's
  width with no half square: 282, no better than a quarter's 274. The lean
  marks keep five sixths of the gain at no cost in trip time; the flank of
  a squad of 15 may brush a fire's edge (its nearest member 4.5 cells from
  a 6-cell fire's middle, `a_squad_sent_past_fire_walks_round_it`).
- *Any straight cut once the way must cross fire* (the first version): on
  planet 1 of the real game, with fire between close crystals covering
  where the squad was sent, walking burn was 471 against 515 off. Now a cut
  crosses no more hot squares than the way it stands for.
- *No point when the line to the next square clips a hot corner*: the
  squad then walked straight at where it was sent, into the fire, for
  2 s at its edge (seen in the film). It takes the next square's point.

The test hand first lit some of its fires inside crystals, where nothing
burns but the mark still turned the squad; it now lights them on the
nearest ground.

Frame cost: a squad that sees fire on its way plans on squares a sixteenth
the dish's cells (about 4,000 on planet 1) once a game second; squads with
no fire ahead check 40 cells of line.

## Own spots round a crystal: kept (2026-11-21)

Look's exit test 2 pass found soldiers stacked round crystals at zoom 6 and
12. Each soldier's goal inside a solid crystal was projected to the rim on
its own side (`Game::rim`), so a squad pressed into one arc. Now
`Game::rim_spots` gives each its own spot round the rim, `SPOT` 3.5 dish
cells apart, filling ring 0 all the way round before a second ring.
Planet 1, run seed 1, `hand=greedy`, `LARDER_PATHLOG` with
[tools/spacing.py](tools/spacing.py): nearest neighbour of standing
soldiers 2.12 -> 3.54 cells median, under 2.4 cells 92% -> 0%, under
3.45 cells 93% -> 20%; harvest at 60 s 4,081 -> 4,199, soldiers 18 -> 19.
Tried first and changed: rings filled nearest-first put a second ring out
before the first was full and halved the harvest at 60 s (2,040), since
only ring 0 is in reach. Still: [crystal-spots/zoom12-before-after.png](crystal-spots/zoom12-before-after.png).

Smoke check on CI's run bot (seeds 1 to 3, planets 1 to 3; main just
before #674, 474dde43, against just after, 618f6964, the only change):
all 9 planets won both times, stars 23 -> 24, soldiers lost 59 -> 31,
harvest 172,205 -> 164,540, but planets 2,540 -> 2,957 s in all, 6 of 9
longer (seed 3's planet 3 400 -> 647 s, 2 stars -> 1). The harvest was not
it (equal income to 160 s in a local replay of seed 2's planet 1): the
bot's rush is an attack-move onto the slime's home crystal, and spots
spread it round the whole rim, half of it through the slime. Follow-up:
spots only for move, guard and drain orders. Local replays of planet 1
(main with #674 against the follow-up): seed 2 wiped at 268 -> 247 s;
seed 3 at 300 s slime left 11,959 -> 1, harvest 13,373 -> 18,422. The
greedy hand's spacing round a crystal stays (3.6 cells median, harvest
at 60 s 4,199).
