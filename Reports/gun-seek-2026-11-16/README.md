# Seeking bullets: step C of the gun-card plan

2026-11-16. Step C of the [gun-system review](../gun-system-review-2026-11-16/README.md)
("seekers that follow the simulation"; the owner's 02:34Z ask for homing),
after the [trigger cards](../gun-triggers-2026-11-16/README.md) and
[multicast](../gun-multicast-2026-11-16/README.md).

Page: [Seeking Bullets](https://claude.ai/artifact/JxxzYTtiug3oNrhfwX3b7J) (source in `page/`;
version 4 leads with homing in the shot trace, the owner's pick; versions
3, 2 and the fly-over prototype below).

Status: **Seek as homing in the shot trace (version 8), for merge.** A
seeking bullet that reaches the point its soldier aimed at with nothing on
its line strikes the slime or walker beside it there. On thin slime it hits
**89.1% of shots against 80.3%** with no card (higher in 13 of 13 paired
windows, sign test p < 0.001; misses 88 to 47) and kills 554 against 538
(9 windows more, 3 fewer; p = 0.15, not significant). On the Fork's thick
front it changes nothing (557 against 552 killed). The earlier grid
steering is off by default (`seek_turn` 0).

## Homing in the shot trace (the owner's pick, 19:38Z)

The owner chose "Build homing into the shot trace" ("Both engines bend a
seeking bullet cell by cell toward the thickest slime it passes, using live
slime") and asked to see hit % beside slime killed ("I would check % hits
alongside slime killed", 19:38Z).

### What was built

- **The engines** (`squad::trace`, the GPU `shots` pass; exact twins):
  each shot carries a seek word (`Frame::catch`: `squad::SEEKS`, and its
  catch width in fine cells, `squad::CATCH`; packed in the high 16 bits of
  the shader's `SqShot.ar2`). A seeking shot stops at a live walker on its
  line as well as at slime's look (sunk as any hit), and one whose line
  misses looks out its catch either side, across the line's longer axis,
  nearest first, a side ending at a wall, and strikes the first cell that
  stops a shot or holds a walker (`squad::caught`), not sunk. A line that
  hits keeps its own hit.
- **The game** (`Game::catches`): a bullet with a seek card gets `SEEKS` on
  every leg; its catch (`Rules::seek_catch` 2 dish cells a card) only on
  the legs that pass within the catch of its aim point and on its last.
  `Bullet::aim` is how far its soldier aimed: a soldier's bullet flies on
  to its full range, so its aim point is where its target was.
- `Rules::seek_turn` 0: version 3's grid steering (`Game::steer`) stays as a
  key (`set=seek_turn=0.45`) but is off; on top of the catch it added
  nothing (below).
- Tests: `squad::tests::a_seeking_shot_strikes_slime_beside_its_line`,
  `larder_world::tests::a_seekers_catch_rides_its_aim_and_last_leg`, and
  three seeking cases in the `mouldgpu` twin (`the_squad_matches_the_cpu_for_one_frame`:
  catch, craters and the sink, and trail too faint to stop any shot, where
  16 of 19 seekers struck walkers); GPU equals CPU in all.
- Cost: no seeker, no work (the catch list is empty and packs as before).
  A seeker's leg near its aim point checks up to 2 x catch cells beside each
  line cell, on the GPU's shots pass (one thread a shot); `Game::catches` is
  one pass over the bullets. 1070 timing: booked with the PR.

### Paired windows (the deciding measure)

`raw/aim-windows.sh`, `raw/aim-windows.txt`, `raw/trace_table.py`: one game
with no cards to T, then each variant for 10 s; 13 thin-slime windows (five
planet scenes at 10, 25 and 40 s with shots, the Fork's greedy game at 5
and 11 s) and the Fork's thick front at 10 and 30 s. Plain shots here match
version 3's run exactly (538 killed, 446 shots), so the build leaves plain
shots as they were.

| Thin slime, 13 windows | Hits of shots | Misses | Killed |
|---|---|---|---|
| No card | 358 of 446 (80.3%) | 88 | 538 |
| **Seek, catch 2 (as built)** | **385 of 432 (89.1%)** | **47** | **554** |
| Seek, catch 4 | 376 of 405 (92.8%) | 29 | 536 |
| Seek, catch 2 and the grid steering | 355 of 397 (89.4%) | 42 | 543 |

| Paired against no card (13 thin windows) | Hit rate higher / lower | Kills higher / lower |
|---|---|---|
| catch 2 | 13 / 0 (p < 0.001) | 9 / 3 (p = 0.15) |
| catch 4 | 13 / 0 (p < 0.001) | 7 / 4 (p = 0.55) |
| catch 2 and steering | 10 / 3 (p = 0.09) | 8 / 5 (p = 0.58) |

The hit-rate rise for catch 2 and catch 4 (p = 0.0002 each) survives a
Bonferroni correction over this run's six comparisons and over all 24 of
the full paired runs for versions 4-8 (0.05 / 24 = 0.002); no kill
difference does. The thick front (two windows): hits 98 of 127 (77.2%)
against 105 of 127 (82.7%), killed 557 against 552.

A rescued miss strikes a lone walker or the edge of a thin patch, so hits
rise far more than kills. Seekers also fired fewer shots (432 against 446):
the soldiers' aims follow the slime, and the fights part after T.

### Films

Version 3's scenes and scripts (`raw/trace-films.sh`, `raw/trace-counts.json`,
clips `page/trace-*.mp4`): one game with no cards to T, then 4 s of plain or
seek+plain, near view at zoom 6, llvmpipe; and the same pair over 10 s.
Shots still in flight at the end count as neither hit nor miss.

| From T | Shots | Hits | Missed | Killed |
|---|---|---|---|---|
| Guard squad, planet 1 seed 3, 40-44 s, plain | 16 | 12 | 4 | 21 |
| same, seek | 17 | 14 | 3 | 19 |
| same, 40-50 s, plain | 39 | 30 | 9 | 46 |
| same, 40-50 s, seek | 49 | 42 | 7 | 44 |
| The Fork's greedy squad, 12-16 s, plain | 35 | 26 | 5 | 35 |
| same, seek | 35 | 28 | 7 | 43 |
| same, 12-21 s, plain | 71 | 53 | 19 | 65 |
| same, 12-21 s, seek | 72 | 55 | 14 | 69 |

### On the way (versions 4-7, each a paired run)

| Version | Thin: hits, killed | Thick: killed | Why it was changed |
|---|---|---|---|
| No card | 80.3%, 538 | 557 | |
| 4: catch beside the line at every cell, before the line's own hit | 93.1%, 504 (catch 2) | 311 | Struck the thin fringe beside the line instead of sinking into the body (hold-30: 471 to 266) |
| 5a: catch only when a leg's line misses | (stopped at 26 of 110 runs) | 377 | A bullet flies 1.5 dish cells a frame, so every leg short of the slime "missed": the same fringe |
| 5b: catch on the bullet's last leg only | 80.0% (6 windows) | 557 | Kept every plain hit, rescued nothing |
| 6: last leg, walkers too | 80.5%, 548 | 560 | A soldier's bullet flies to its full range: its last leg is far past its target |
| 7: stop at live walkers on the line | identical to no card | 557 | Missed bullets pass beside their target, not through its walkers |
| 7 with the grid steering | 80.9%, 567 | 520 | Kills +5% (9 / 2, p = 0.065), hits unchanged |
| **8: catch at the aim point** | **89.1%, 554** | **552** | Built |

Raw: `raw/trace-windows.sh` and `.txt` (4), `raw/line-windows-every-leg.txt`
(5a), `raw/line-windows-last-leg.txt` (5b), `raw/line-windows.sh` and
`raw/line-windows-last-leg-walkers.txt` (6), `raw/walk-windows.sh` and
`.txt` (7). Versions 4-7 ran with the code at each step, which is not kept
in a commit; `squad::trace`'s and `Game::catches`'s comments record each.

## Version 3: homing by the mould grid (as reported before)

Version 3's report as it stood. Its last section, "Asked of the owner",
was answered at 19:38Z: homing in the shot trace (above).

### The owner's answer (14:24Z)

To the first try's question: seeking the slime that feeds on a crystal is
pointless ("the rest of the slime will just immediately surge back to the
crystal. You're not going to cut it off"); a mini-boss card is "such a
specific card"; homing on the densest slime risks bullets that "avoid all
the individual walkers in front"; "Homie needs to be a clear benefit. I
mean, how often do soldiers you're not controlling miss their target".

### How often squads on their own miss

A miss here is a soldier's bullet that ends its range, or hits a wall or
rock, having struck no slime (`Game::missed_range`, `Game::missed_wall`,
printed on the headless rounds line). 60 s each (the Fork's greedy game ends
at 21 s when its larder is overrun), llvmpipe, one game a scene, the squad
on its own (`hand=` the scene's), `load=0.5,plain` or `load=0.5,seek+plain`
(`raw/miss-census.txt`):

| Scene | No card: missed of shots | Seek | Killed, no card / seek | Soldiers |
|---|---|---|---|---|
| The Fork, holding at the larder (thick front) | 39 of 368 (11%) | 43 of 400 (11%) | 2,713 / 2,739 | 5 / 5 |
| The Fork, greedy, at the larder | 22 of 100 | 7 of 93 | 106 / 133 | 9 / 9 |
| Planet 1 seed 2, guarding | 19 of 89 | 8 of 76 | 77 / 79 | 8 / 8 |
| Planet 1 seed 3, guarding | 46 of 183 | 31 of 174 | 186 / 186 | 8 / 8 |
| Planet 3 seed 4, guarding | 14 of 42 | 5 of 36 | 33 / 41 | 8 / 8 |
| Planet 1 seed 1, squad at work | 6 of 92 | 3 of 95 | 222 / 208 | 8 / 8 |
| Planet 2 seed 1, squad at work | 27 of 179 | 21 of 182 | 233 / 250 | 8 / 8 |
| **Thin slime (all but the first row)** | **134 of 685 (19.6%)** | **75 of 656 (11.4%)** | **857 / 897** | |

**The answer to the owner's question** is the no-card column: squads on
their own strike nothing with about one shot in ten on a thick front and
one in five against thin slime (134 of 685). Four more scenes (planet 1
seed 5, planet 3 seeds 2 and 6, planet 2 seed 3 at work) had no shots in
60 s.

The seek column looked like a clear win (fewer misses in 6 of 6 thin-slime
scenes, sign test p = 0.03), but each row is two whole games that part at
0.5 s, so the two columns are different fights. The paired windows below
did not reproduce it.

### Paired windows (the deciding measure)

One game with no cards to T, then plain or seek+plain for 10 s, so both
start from the same fight (`raw/miss-windows.sh`, `raw/miss-windows.txt`,
`raw/windows_table.py`). Windows at 10, 25 and 40 s in five planet scenes
and at 5 and 11 s in the Fork's greedy game; four windows had no shots.

| 13 thin-slime windows | No card | Seek |
|---|---|---|
| Missed of shots | 88 of 446 (19.7%) | 81 of 438 (18.5%) |
| Windows with the lower miss rate | 7 | 6 (sign test p = 1.0) |
| Slime killed | 538 | 573 (+6.5%) |
| Windows with more kills | 2 | 9 (2 even; p = 0.065) |
| Soldiers lost | 0 | 0 |

The Fork's thick front (windows at 10 and 30 s): 29 of 127 missed against
31 of 132, 557 killed against 520. Seek gives a few percent more kills
against thin slime (not significant) and leaves misses where they were.

### Why homing cannot fix the misses

(Version 3's reasoning. The trace can: version 8 strikes where the bullet
reaches its target, from the live slime, and halves the misses.)

- The game steers by the mould grid: walkers counted per dish cell, a
  game second old on the GPU (`GRID_EVERY`). A bullet stops on trail at
  the slime's look (`squad::slime_trail`), not on walkers. Against thin
  slime the misses are bullets that cross walkers whose trail is too faint
  to stop them, or walkers that have moved on; a seeker steered by the
  grid chases the same walkers.
- The ceiling is low anyway: one shot in five against thin slime and one
  in ten on a front. A homing card that rescued every miss would add at
  most a fifth more hits where the slime is thin.
- Seekers also bent into walls (7 of a guard squad's 49 shots in a first
  paired run); a turn whose new heading meets a wall before its mark is
  now skipped (`Game::turn_to`), but some still end on walls after their
  mark turns out empty (7 of 71 in one window against 4).

### What was built

- **`Card::Seek`** (kind "Always", cost 2, tier 2; `Game::steer`, before
  each frame's legs): every bullet of the squad, once it has flown
  `Rules::seek_after` (3 dish cells; a split piece or a let-out cast at
  once), turns each frame by at most `seek_turn` (0.45 rad) a card toward a
  live mini-boss ahead in reach and sight, else the cheapest square of the
  mould grid (`World::mould_grid` at a dish cell; distance times one plus
  twice the turn) within `seek_r` (12 dish cells, and what it has left to
  fly) and `seek_cone` (1 rad) holding `seek_min` (1) walker, in sight, more
  than `seek_gap` (2 dish cells) from where a bullet struck last frame and
  from what earlier seekers took this frame; a turn whose new heading meets
  a wall before its mark is skipped. No engine change: bullets are game
  state and each frame's leg is straight.
- **Nearest first, weighted to straight ahead**, so a seeker never passes
  the walkers coming at the squad for denser slime behind them (the
  owner's worry). The gap is freshness: the grid is a game second old on
  the GPU, so it still holds the slime last frame's bullets killed; and two
  seekers do not chase one walker.
- `seek_min` 1, not the first try's 2: the misses are mostly at lone
  walkers, and with 2 the guarding squads turned seldom.
- Counts: `Game::seek_turns`, `Game::hits`, `Game::hit_cells`,
  `Game::missed_range`, `Game::missed_wall`; the headless rounds line
  prints them. Presets `seek`, `seek-buck`, `seek-split`; `set=seek_*`
  tunes the rules.
- Test: `larder_world::tests::a_seeker_turns_onto_the_nearest_live_slime_ahead`
  (turns toward slime by at most the cap; keeps on at the nearest slime;
  two seekers take squares apart; a square hit last frame is not taken;
  no card or too early flies straight).
- Cost: `steer` took 0.05-0.08 ms a frame while seekers flew (the cloud
  CPU, three scenes: 49, 84 and 58 µs a frame on average, 4-12 bullets),
  nothing without them.

### Films

Paired clips from one game (`raw/miss-films.sh`, `raw/miss-counts.json`):
no cards to T, then 4 s of plain or seek+plain at 30 frames a second,
the near view at zoom 6, llvmpipe; and the same pair counted over 10 s.

| From T | Shots | Missed (wall) | Hits | Killed | Seeker turns |
|---|---|---|---|---|---|
| Guard squad, planet 1 seed 3, 40-44 s, plain | 16 | 4 (0) | 12 | 21 | |
| same, seek | 16 | 1 (0) | 15 | 22 | 24 |
| same, 40-50 s, plain | 39 | 9 (0) | 30 | 46 | |
| same, 40-50 s, seek | 49 | 14 (7) | 35 | 52 | 70 |
| The Fork's greedy squad at its larder, 12-16 s, plain | 35 | 5 (1) | 26 | 35 | |
| same, seek | 35 | 8 (1) | 28 | 53 | 32 |
| same, 12-21 s, plain | 71 | 19 (3) | 53 | 65 | |
| same, 12-21 s, seek | 70 | 22 (2) | 43 | 71 | 86 |

The 40-50 s seek count ran before the wall check; the 4 s clips show the
bend; the counts swing both ways.

### Tried and dropped

- **Turning only a bullet with no walker on its line** (what the owner was
  told at 14:24Z: "bend only when nothing is left on its line"). The grid
  counts walkers, a game second old on the GPU, and a bullet stops on
  trail, not walkers (it flies through a lone walker's faint trail), so the
  line test called most misses hits: against thin slime 87 misses of 635
  shots (with the gap; kills 815) against 75 of 656 for homing on every
  seeker (kills 897); without the gap, 92 of 743 over five scenes against
  66 of 700. Every seeker homing
  onto the nearest slime gives the same promise (it never turns away from
  walkers in front) without the guess.
- **Turning twice as fast** (`seek_turn` 0.9, one game a scene): 115 misses
  of 1,024 against 118 of 1,056; no gain, kept at 0.45.
- **No hit gap** (every seeker to the cheapest square, hits or not): over
  the four thin-slime scenes it ran in, 39 misses of 319 shots, against 23
  of 300 with the gap (no card: 61 of 323).
- **The first try** (seek_min 2, on the thick front) and **the fly-over
  prototype**: below, as they were.

### Asked of the owner

Homing does not clear the owner's bar. Options: drop homing and go on to
step D, keeping the miss counters (recommended: the most any homing can
add is a fifth more hits on thin slime); keep Seek as a small card (bullets
visibly bend, about 6% more kills on thin slime); or build homing into the
shot trace, the engines bending a seeking bullet cell by cell toward the
thickest trail it passes (fresh and exact, both engines, a few hours, still
under that ceiling).

## The first try (as shown on the page's first version)

- **`Card::Seek`** (kind "Always", cost 2, tier 2): every bullet of the
  squad, once it has flown `Rules::seek_after` (3 dish cells; a split piece
  or a let-out cast at once), turns each frame by at most `seek_turn`
  (0.45 rad) a card (`Game::steer`, before the frame's legs). Its mark: a
  live mini-boss ahead in reach and sight first; else the cheapest square
  of the mould grid (`World::mould_grid` at a dish cell, the GPU's from the
  frame's readout) within `seek_r` (12) and `seek_cone` (1 rad) holding
  `seek_min` (2) walkers, in sight, more than `seek_gap` (2) dish cells from
  last frame's hits and from earlier seekers' picks this frame (the plan's
  "shrapnel seek": each pellet or piece its own patch). No engine change:
  bullets are game state and each frame's leg is straight.
- Counts: `Game::seek_turns`, `Game::hits`, `Game::hit_cells` (the dish
  cells hit, once a frame); the headless rounds line prints them.
- Presets `seek`, `seek-buck`, `seek-split`; `set=seek_*` tunes the rules.

### Its films

`hand=hold@150,265 bank=8000`, the Fork, squad mode, llvmpipe, near view
`centre=176,266,12`, 960 x 600, 30 frames a second (`raw/films.sh`,
`raw/make_clips.py`, `raw/counts.json`). One game to 29 s with plain
bullets, then each loadout; the mini-boss clips add `boss=29,185,250`.

| 3 s, 29-32 s | Bullets | Hits in cells | Seeker turns | Killed |
|---|---|---|---|---|
| buckshot | 84 | 81 in 58 | | 58 |
| seek + buckshot | 84 | 84 in 57 | 62 | 63 |
| split + blast | 21 | 49 in 49 | | 1,497 |
| seek + split + blast | 21 | 50 in 50 | 134 | 1,366 |
| mini-boss, plain | 21 | 21 in 21 | | 68 |
| mini-boss, seek + plain | 21 | 20 in 20 | 35 | 71 |

| 10 s, 29-39 s | Without seek | With seek |
|---|---|---|
| plain: killed | 472 | 434 |
| buckshot: killed | 228 | 214 |
| split + blast: killed | 1,744 | 1,756 |
| mini-boss, plain: its members left of 62 | 25 | 40 |
| mini-boss, buckshot: its members left of 62 | 60 | 62 |

These ran with `seek_min` 2 (add `set=seek_min=2` to reproduce). One game each: a few percent is noise. Nothing says seek helps; the
mini-boss rows lean the other way.

### Why it changed little

- A shot stops at the first slime on its line (owner 11-11). The front is
  solid, so a bent bullet still hits the slime in front, a cell or two
  from where it would have.
- Soldiers already aim at the densest slime in sight and their hits were
  already spread (the plain game to 39 s: 215 hits in 214 cells), so the
  gap rule has no crowding to undo.
- The plan predicted it for plain homing ("lands where the bullet was
  going anyway"); the gap and the mini-boss mark did not change that.
- Aim cards meet the same wall: squads on their own hold the defend stance
  and only shoot slime near them, so an aim card would change only which
  nearby slime they pick; it matters for squads on attack orders.

### Prototype: seekers fly over the slime to a mark

To try the recommended answer (commit af0f76a8 on the branch, `Rules::seek_over`, removed since): a
seeker past `seek_after` with a mark ahead within what it has left to fly
and in sight (a live mini-boss's centre, else the densest square on a
crystal holding `seek_fed` 20 walkers) is not traced (`Bullet::over`;
`Flight::legs` keeps every leg, the world gets the others) until this
frame's leg reaches the mark, so it strikes nothing on the way and lands
there. No engine change.

| Mini-boss, 10 s, 29-39 s | Members left of 62 | Killed | Soldiers left of 7 |
|---|---|---|---|
| plain | 25 | 259 | 6 |
| seek + plain, flying over | 30 | 508 | 7 |
| buckshot | 60 | 136 | 6 |
| seek + buckshot, flying over | 45 | 78 | 5 |

3 s clips from 34 s (the hunt's start): buckshot 91 hits in 75 cells, the
mini-boss 62 to 61, 7 soldiers; seek + buckshot flying over 82 hits in 40
cells, 62 to 52, 6 soldiers. The pellets reach the mini-boss instead of
the slime in front; the slime between is left alone. With plain bullets
the two came out about even on the mini-boss. In the hold scene with no
mini-boss nothing changed (434 and 214 killed, as without it): crystal
feeders seldom became marks, as a squad on its own fires only at slime
near it.

Rejected: flying over from the muzzle. That squad lost 4 of 7 soldiers in
8 s (from 29 s) against none, because it stopped shooting the slime on
itself; the first leg is now traced.

### Asked of the owner then

How homing should work: seekers that fly over slime to their mark (a
mini-boss, the slime feeding on a crystal, a vein), bending the
first-slime rule for that card only (recommended: the only version that
reaches what a plain shot cannot); keep seek small as built; or drop
homing and go on to step D.

## Files

- `page/index.html` and its clips: the Artifact page's source (version 8's
  clips `trace-*.mp4`).
- `raw/aim-windows.sh`, `raw/aim-windows.txt`, `raw/trace_table.py`: version 8's paired windows; versions 4-7's runs are listed in their section.
- `raw/trace-films.sh`, `raw/trace-counts.json`: version 8's paired clips and 10 s counts.
- `raw/miss-census.txt`, `raw/census_table.py`: the one-game-a-scene miss runs, every variant's last count lines.
- `raw/miss-windows.sh`, `raw/miss-windows.txt`, `raw/windows_table.py`: the paired windows.
- `raw/miss-films.sh`: version 3's paired clips and 10 s counts.
- `raw/films.sh`: the first try's clips and 10 s runs; `raw/film.sh`: one clip's frames.
- `raw/make_clips.py`: frames to H.264 MP4 and the counts as JSON.
- `raw/counts.json`: the first try's counts; `raw/miss-counts.json`: version 3's.
