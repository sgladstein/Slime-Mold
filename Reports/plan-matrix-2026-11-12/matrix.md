# Plan-by-tier matrix

Made 2026-11-12 17:38Z from commit `02499342b` by `scripts/plan_matrix.py`; seeds ['201-216'] (16). Raw output in `raw/`.

Provenance: `larderworld the race world, scale 2 walkers 5000 cap 900 s, tps 15, seeds [201, 202, 203, 204, 205, 206, 207, 208, 209, 210, 211, 212, 213, 214, 215, 216]; on the CPU; tier none; flee 0 (default); rules Rules { tps: 15, frame_ticks: 1, speed: 10.0, range: 15.0, strike: 2.5, shot_every: 1.0, price: 0…`

The number after each tier is how many paired seeds it has; a stopped run leaves some short. Each cell: median `Game::score` (v2), then the ends (mould wiped / cap / squad wiped). A win is above 1; a stalemate at the cap is about −0.4 to +0.5. **Bold** is the best plan by median when it beats the runner-up after Holm over the whole matrix; otherwise the column is a tie.

| Tier | idle | random | farm | boom | claim | fort | strike | randkit | loud | quiet | plague | siege | Best |
| --- | --- | --- | --- | --- | --- | --- | --- | --- | --- | --- | --- | --- | --- |
| none (16) | -0.32 0/16/0 | -1.66 0/2/14 | +0.31 0/16/0 | +0.20 0/16/0 | +0.32 3/13/0 | +0.21 0/16/0 | +0.33 4/12/0 | -1.15 4/3/9 | +1.47 10/6/0 | +0.15 0/16/0 | +1.45 16/0/0 | +0.24 0/16/0 | loud over plague 6-10, median -0.12, Holm p 1.000 (tie) |
| sporer (16) | -0.41 0/15/1 | -1.73 0/2/14 | +0.21 0/16/0 | +0.12 0/16/0 | +0.14 0/16/0 | +0.11 0/16/0 | +0.10 0/16/0 | +0.95 8/2/6 | +0.15 6/10/0 | +0.09 0/16/0 | **+1.36 10/6/0** | +0.13 0/16/0 | plague over farm 16-0, median +1.14, Holm p 0.004 |
| sleeper (11) | -0.17 0/11/0 | -1.77 0/2/9 | +0.32 0/11/0 | +0.33 0/11/0 | +0.49 0/11/0 | +0.33 0/11/0 | +0.49 0/11/0 | -1.58 0/0/11 | +1.22 7/4/0 | +0.34 0/11/0 | +0.49 0/11/0 | +0.34 0/11/0 | loud over strike 10-1, median +0.72, Holm p 0.258 (tie) |
| drifter (16) | -0.15 0/16/0 | +0.29 7/4/5 | +0.42 0/16/0 | +0.38 0/16/0 | +1.84 14/2/0 | +0.38 0/16/0 | +2.30 13/3/0 | +0.92 8/4/4 | +1.80 14/2/0 | +0.39 0/16/0 | +0.48 6/10/0 | +0.42 0/16/0 | strike over claim 13-3, median +0.47, Holm p 0.447 (tie) |
| sprawler (16) | -0.76 0/8/8 | -1.77 0/1/15 | +0.46 7/9/0 | +0.10 4/12/0 | +0.21 5/11/0 | +0.10 5/11/0 | +1.97 9/7/0 | -0.36 6/2/8 | +1.39 11/5/0 | +0.09 0/16/0 | +1.50 16/0/0 | +0.24 2/14/0 | strike over plague 9-7, median +0.53, Holm p 1.000 (tie) |
| trunk (16) | -0.28 0/16/0 | -0.67 5/3/8 | +0.33 1/15/0 | +0.26 2/14/0 | +1.79 9/7/0 | +0.27 2/14/0 | +2.20 13/3/0 | +0.49 7/6/3 | +1.54 16/0/0 | +0.25 0/16/0 | +1.40 14/2/0 | +0.30 0/16/0 | strike over claim 13-3, median +0.37, Holm p 0.447 (tie) |
| grudge (14) | -0.33 0/14/0 | -1.75 0/2/12 | +0.15 0/14/0 | +0.11 0/14/0 | +0.13 0/14/0 | +0.11 0/14/0 | +0.16 1/13/0 | – | – | – | – | – | strike over farm 8-6, median +0.01, Holm p 1.000 (tie) |

## Each plan against idle and random (higher-lower, Holm p)

| Tier | farm | boom | claim | fort | strike | loud | quiet | plague | siege |
| --- | --- | --- | --- | --- | --- | --- | --- | --- | --- |
| none | idle 16-0 0.004, random 16-0 0.004 | idle 16-0 0.004, random 16-0 0.004 | idle 16-0 0.004, random 16-0 0.004 | idle 16-0 0.004, random 16-0 0.004 | idle 16-0 0.004, random 16-0 0.004 | idle 16-0 0.004, random 16-0 0.004 | idle 16-0 0.004, random 16-0 0.004 | idle 16-0 0.004, random 16-0 0.004 | idle 16-0 0.004, random 16-0 0.004 |
| sporer | idle 16-0 0.004, random 16-0 0.004 | idle 16-0 0.004, random 16-0 0.004 | idle 16-0 0.004, random 16-0 0.004 | idle 16-0 0.004, random 16-0 0.004 | idle 16-0 0.004, random 16-0 0.004 | idle 16-0 0.004, random 16-0 0.004 | idle 16-0 0.004, random 16-0 0.004 | idle 16-0 0.004, random 16-0 0.004 | idle 16-0 0.004, random 16-0 0.004 |
| sleeper | idle 11-0 0.040, random 11-0 0.040 | idle 11-0 0.040, random 11-0 0.040 | idle 11-0 0.040, random 11-0 0.040 | idle 11-0 0.040, random 11-0 0.040 | idle 11-0 0.040, random 11-0 0.040 | idle 11-0 0.040, random 11-0 0.040 | idle 11-0 0.040, random 11-0 0.040 | idle 11-0 0.040, random 11-0 0.040 | idle 11-0 0.040, random 11-0 0.040 |
| drifter | idle 16-0 0.004, random 8-8 1.000 | idle 16-0 0.004, random 8-8 1.000 | idle 16-0 0.004, random 13-3 0.447 | idle 16-0 0.004, random 8-8 1.000 | idle 16-0 0.004, random 14-2 0.096 | idle 16-0 0.004, random 10-6 1.000 | idle 16-0 0.004, random 8-8 1.000 | idle 16-0 0.004, random 8-8 1.000 | idle 16-0 0.004, random 8-8 1.000 |
| sprawler | idle 16-0 0.004, random 16-0 0.004 | idle 16-0 0.004, random 16-0 0.004 | idle 16-0 0.004, random 16-0 0.004 | idle 16-0 0.004, random 16-0 0.004 | idle 16-0 0.004, random 16-0 0.004 | idle 16-0 0.004, random 16-0 0.004 | idle 16-0 0.004, random 16-0 0.004 | idle 16-0 0.004, random 16-0 0.004 | idle 16-0 0.004, random 16-0 0.004 |
| trunk | idle 16-0 0.004, random 10-6 1.000 | idle 16-0 0.004, random 12-4 1.000 | idle 16-0 0.004, random 13-3 0.447 | idle 16-0 0.004, random 12-4 1.000 | idle 16-0 0.004, random 16-0 0.004 | idle 16-0 0.004, random 13-3 0.447 | idle 16-0 0.004, random 10-6 1.000 | idle 16-0 0.004, random 12-4 1.000 | idle 16-0 0.004, random 10-6 1.000 |
| grudge | idle 14-0 0.006, random 14-0 0.006 | idle 14-0 0.006, random 14-0 0.006 | idle 14-0 0.006, random 14-0 0.006 | idle 14-0 0.006, random 14-0 0.006 | idle 14-0 0.006, random 14-0 0.006 | – | – | – | – |

## Random's mould wipes per tier

none 0/16, sporer 0/16, sleeper 0/11, drifter 7/16, sprawler 0/16, trunk 5/16, grudge 0/14

125 tests in the Holm family.
