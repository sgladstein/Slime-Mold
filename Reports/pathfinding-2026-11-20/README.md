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
- **Walking maps do not avoid slime or fire.** A sent squad walks the
  shortest way through a body of walkers. Steering round slime would be a
  rule change (attack-move already stands to fight), so it is left for the
  Squad and arsenal lane's list, not this pass.
- **The maps are rebuilt every 30 s**, so a crystal eaten since then still
  blocks its cells. No squad was seen held by one in these runs.
