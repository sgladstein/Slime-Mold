# Plan-by-tier matrix

Made 2026-11-14 01:00Z from commit `323832eab` by `scripts/plan_matrix.py`; seeds ['301-308'] (8). Raw output in `raw/`.

Provenance: `larderworld the race world, scale 2 walkers 25000 cap 900 s, tps 15, seeds [301, 302, 303, 304, 305, 306, 307, 308]; on the CPU; tier none; flee 0 (default); rules Rules { tps: 15, frame_ticks: 1, speed: 10.0, range: 15.0, strike: 2.5, shot_every: 1.0, bullet_speed: 45.0, split_n: 3, split_fan: 0.5,…`

The number after each tier is how many paired seeds it has; a stopped run leaves some short. Each cell: median `Game::score` (v2), then the ends (mould wiped / cap / squad wiped). A win is above 1; a stalemate at the cap is about −0.4 to +0.5. **Bold** is the best plan by median when it beats the runner-up after Holm over the whole matrix; otherwise the column is a tie.

| Tier | idle | random | boom | claim | fort | strike | cut | Best |
| --- | --- | --- | --- | --- | --- | --- | --- | --- |
| none (8) | -1.33 0/0/8 | -1.83 0/0/8 | +0.33 0/8/0 | +0.34 0/8/0 | +0.33 0/8/0 | -1.22 0/1/7 | +0.34 0/7/1 | claim over cut 6-2, median +0.01, Holm p 0.578 (tie) |

## Each plan against idle and random (higher-lower, Holm p)

| Tier | boom | claim | fort | strike | cut |
| --- | --- | --- | --- | --- | --- |
| none | idle 8-0 0.086, random 8-0 0.086 | idle 8-0 0.086, random 8-0 0.086 | idle 8-0 0.086, random 8-0 0.086 | idle 5-3 0.727, random 8-0 0.086 | idle 8-0 0.086, random 8-0 0.086 |

## Random's mould wipes per tier

none 0/8

11 tests in the Holm family.
