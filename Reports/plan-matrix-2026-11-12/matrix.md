# Plan-by-tier matrix

Made 2026-11-12 17:09Z from commit `02499342b` by `scripts/plan_matrix.py`; seeds ['201-216'] (16). Raw output in `raw/`.

Provenance: `larderworld the race world, scale 2 walkers 5000 cap 900 s, tps 15, seeds [201, 202, 203, 204, 205, 206, 207, 208, 209, 210, 211, 212, 213, 214, 215, 216]; on the CPU; tier drifter; flee 0 (default); rules Rules { tps: 15, frame_ticks: 1, speed: 10.0, range: 15.0, strike: 2.5, shot_every: 1.0, price…`

Each cell: median `Game::score` (v2), then the ends (mould wiped / cap / squad wiped). A win is above 1; a stalemate at the cap is about −0.4 to +0.5. **Bold** is the best plan by median when it beats the runner-up after Holm over the whole matrix; otherwise the column is a tie.

| Tier | idle | random | farm | boom | claim | fort | strike | randkit | loud | quiet | plague | siege | Best |
| --- | --- | --- | --- | --- | --- | --- | --- | --- | --- | --- | --- | --- | --- |
| drifter | -0.15 0/16/0 | +0.29 7/4/5 | +0.42 0/16/0 | +0.38 0/16/0 | +1.84 14/2/0 | +0.38 0/16/0 | +2.30 13/3/0 | +0.92 8/4/4 | +1.80 14/2/0 | +0.39 0/16/0 | +0.48 6/10/0 | +0.42 0/16/0 | strike over claim 13-3, median +0.47, Holm p 0.213 (tie) |
| sprawler | -0.76 0/8/8 | -1.77 0/1/15 | +0.46 7/9/0 | +0.10 4/12/0 | +0.21 5/11/0 | +0.10 5/11/0 | +1.97 9/7/0 | -0.36 6/2/8 | +1.39 11/5/0 | +0.09 0/16/0 | +1.50 16/0/0 | +0.24 2/14/0 | strike over plague 9-7, median +0.53, Holm p 1.000 (tie) |

## Each plan against idle and random (higher-lower, Holm p)

| Tier | farm | boom | claim | fort | strike | loud | quiet | plague | siege |
| --- | --- | --- | --- | --- | --- | --- | --- | --- | --- |
| drifter | idle 16-0 0.001, random 8-8 1.000 | idle 16-0 0.001, random 8-8 1.000 | idle 16-0 0.001, random 13-3 0.213 | idle 16-0 0.001, random 8-8 1.000 | idle 16-0 0.001, random 14-2 0.046 | idle 16-0 0.001, random 10-6 1.000 | idle 16-0 0.001, random 8-8 1.000 | idle 16-0 0.001, random 8-8 1.000 | idle 16-0 0.001, random 8-8 1.000 |
| sprawler | idle 16-0 0.001, random 16-0 0.001 | idle 16-0 0.001, random 16-0 0.001 | idle 16-0 0.001, random 16-0 0.001 | idle 16-0 0.001, random 16-0 0.001 | idle 16-0 0.001, random 16-0 0.001 | idle 16-0 0.001, random 16-0 0.001 | idle 16-0 0.001, random 16-0 0.001 | idle 16-0 0.001, random 16-0 0.001 | idle 16-0 0.001, random 16-0 0.001 |

## Random's mould wipes per tier

drifter 7, sprawler 0 (of 16)

38 tests in the Holm family.
