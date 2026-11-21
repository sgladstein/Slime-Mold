# Why the slime regrew after its crystals were drained (2026-11-21)

The owner, 2026-11-21 05:13Z, after planet 1 of run 1795236572 (lost at
16:33): "I harvested every crystal on the map except for one and the slime
was easily growing to its peak size and I had really beated it down a bunch
of times. I would be curious to see slime population overtime after each
match". 05:16Z: "if everything is working as expected and the slime was just
surviving off the one crystal left. I would then like a review of the
distribution of how much food both the player can pump from different sized
crystals and how much different sized crystals feel the slime. How far apart
is a big crystal versus average vs small".

Page: https://claude.ai/artifact/Vt6C8A69rvXuoBsCfLhKqz (source `page/index.html`, data inline).
No tuning was changed; balance questions went to Direction.

## What fed it

- **Its ceiling is its slots, not its food.** Planet 1's slime starts with
  14,320 walkers in 28,640 slots (`run_shape`: shape C's 25,000 in 50,000,
  times the level's factor). The log's slime touched the ceiling at 3:44,
  11:04, 14:23 and 16:27 (most 28,638). From 11,367 at 8:13 it was back at
  the ceiling at 11:04.
- **Its only food is crystal charge.** `life` gives a walker its cell's
  crystal share and, off crystal worlds, grass; starfall ore and cores eaten
  by the slime only fill the surge meter (`Game::drop_out`); births come
  from that energy (`breed`). A drained crystal falls to 0-680 charge in the
  replay below.
- **One big crystal pays almost the whole slime's upkeep.** A fully covered
  crystal yields about 0.151 charge a crystal cell a second (the home crystal
  146/s at the start, crystals 2 and 3 145 and 157/s from 6:40 to 8:20,
  crystal 1 71/s against 70 estimated). Upkeep is 0.0002 a fine step, 30
  fine steps a second at scale 2: 0.006 a walker a second. A crystal of radius
  20 (1,018 cells) pays for about 25,600 walkers; the ceiling is 28,640.
- **In this game more than one crystal still fed it.** The log ends with
  401,000 charge on the map. From 14:16 to 15:46, with no income, the map
  lost 564 charge a second, three to four big crystals' worth.
- **Replay.** `LARDER_REPLAY=FILE larder-world run=1795236572:1
  seed=14523872617039525855 mode=squads` replays the window's saved
  commands headless with a per-crystal line a second. It follows the real
  game closely (slime within a few % to 8:00) and then drifts: the replay's
  squad died at 8:30 where the real one kept 7. At 8:30, 7 of 11 crystals
  still held food (crystals 0, 1, 2, 3, 5, 8, 9).

From now on the play log writes `crystals t=T charge/taken ...` every 10 s,
so the next review can name which crystals fed the slime.

## Big, median and small crystals

`examples/crystalstats.rs` lists every crystal the level generator makes for
planets 1-8 over run seeds 1-40 (8,108 crystals). Totals are calibrated to
the GPU world's start charge (0.81 of the disc's fine cells are crystal).

| Planet | Crystals a planet | Pump at its 5/s floor | Pump food/s p10 · median · p90 · max | Walkers fed p10 · median · p90 · max |
|---|---|---|---|---|
| 1 | 9.6 | 53% | 5 · 5 · 40.7 · 139 | 3,602 · 7,061 · 21,918 · 65,578 |
| 2 | 11.1 | 69% | 5 · 5 · 36.7 · 208 | 3,138 · 5,780 · 19,613 · 78,451 |
| 3 | 14.2 | 53% | 5 · 5 · 38.7 · 71 | 3,602 · 7,061 · 20,749 · 28,242 |
| 4 | 15.7 | 54% | 5 · 5 · 42.8 · 71 | 3,138 · 7,061 · 21,918 · 28,242 |
| 5 | 27.9 | 62% | 5 · 5 · 36.9 · 179 | 4,099 · 7,061 · 20,749 · 67,644 |
| 6 | 30.4 | 59% | 5 · 5 · 36.9 · 174 | 4,099 · 7,061 · 20,749 · 67,644 |
| 7 | 30.8 | 79% | 5 · 5 · 29.4 · 174 | 3,138 · 5,780 · 17,435 · 76,225 |
| 8 | 63.0 | 68% | 5 · 5 · 39.2 · 202 | 3,602 · 6,404 · 20,749 · 76,225 |

A pump pays by the crystal's food (`Game::pump_draw`: 1/150 of it a second,
at least 5), and charge a cell rises with size (96 to 640), so a top-tenth
crystal pays a pump 6-9x the median and the biggest 14-42x. The slime eats by
area, so a top-tenth crystal feeds it 3-3.4x the median and the biggest
4-14x. Fully covered, the slime alone takes 11-21 min to empty a small or
median crystal and 42-57 min a top-tenth one.

## End screen graph

`Game::curve` keeps the walkers and soldiers at each game second's count;
the end card draws them under its numbers (`end_curve` in `ui.rs`): amber
columns for the slime (the most in each column's seconds), cyan dots for the
soldiers on their own scale, the peak and the clock. Stills:
`end-screen-won.png`, `end-screen-lost.png` (`film_ends`, run seed 1).
