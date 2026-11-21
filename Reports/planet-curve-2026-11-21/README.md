# The bot's curve over planets 2 to 5, after the threats were paced (2026-11-21)

The coordinator asked on 11-21 00:49Z, after the owner's pacing ruling of 11-20 23:32Z (`run::FIRST_PLANET`, #606, #612), whether any of planets 2 to 5 jumps in difficulty compared with its neighbours, and what timing moves would smooth it. This is a measurement for Milestone 2's "curve over all eight planets". Direction (11-21 04:53Z) put planets 4 and 5 outside Milestone 1 and asked for the numbers and proposals to be kept here. Nothing in the game changed.

## The run

- **PC job `00-b-curve`**: the strategy bot's PC worker on the owner's PC, src `4180f5ed06c3` (main through #612, plus two bot-only fixes on the bot's branch: `broke=20` and dry crystals under 10 points). 108 games: `league=DEPTH:SEED:1200` cold landings, with the shops of the planets before, on depths 2 to 5 and maps 1 to 8 plus 1795038793. Three plans: `rush,aim=6,cards=0`; `econ,go=4,aim=6,hold=8,grow=1.3,regroup=3,spare=1,fire=0.5`; `lure,go=4,aim=6,hold=8,grow=1.3,regroup=3,spare=1`.
- Rows: [`00-b-curve.jsonl`](00-b-curve.jsonl). Summary: `python3 curve.py 00-b-curve.jsonl`.
- **The deciding measure**: wins of 27 per planet. A planet jumps if it loses a third or more of its neighbour's wins.

## Results

| Planet | Slime | Wins of 27 | Rush | Econ (switches) | Lure (switches) | Median start / peak slime | Losses with a squad of 24+ | ... cut under 60% of peak | ... under 20% |
|---|---|---|---|---|---|---|---|---|---|
| 2 | Sprawler | 20 | 9 (4.5 min) | 5 (5.1) | 6 (6.1) | 13.8k / 16.5k | 3 of 7 | 2 | 0 |
| 3 | Trunk | 21 | 8 (5.5) | 6 (10.2) | 7 (9.8) | 37.0k / 41.1k | 5 of 6 | 5 | 2 |
| 4 | Swarmer | 12 | 6 (6.2) | 1 (19.5) | 5 (10.3) | 61.1k / 67.3k | 11 of 15 | 8 | 3 |
| 5 | Sporer | 2 | 2 (11.5) | 0 | 0 | 86.7k / 146.9k | 19 of 25 | 12 | 5 |

- **Planets 2 and 3 are level** (20 and 21 of 27). On planet 2, half the dead burned in the bot's own fire (157 of 312).
- **Planet 4 drops** to 12, and the two build-first plans drop most (econ 6 to 1). Bites killed 2,116 soldiers and fire only 68. Eight of the eleven big-squad losses came after the squad had cut the slime under 60% of its peak, where the strikers condense; three came under 20%, where the last-stand heart forms (planet 4 is its first planet since #606).
- **Planet 5 is a wall** at 2 of 27. Its slime grows from 87k to a median peak of 147k, more than twice planet 4's. Rush plans built 80 to 109 soldiers and still lost at 10 to 12 minutes, with 6% to 46% of the slime left, to bursts of 20 to 30 bites in 30 s. Twelve of the nineteen big-squad losses came under 60% of the peak.
- **The wall at 5 is older than the late threats.** The bot's whole runs on the game before strikers (`14-runs`, 66 runs; `Reports/strategy-bots-2026-11-19/pc/14-runs.jsonl`) won planet 5 on 0 of the 6 runs that reached it, against 6 of 15 on planet 4, 15 of 27 on planet 3, 27 of 38 on planet 2 and 38 of 66 on planet 1.
- **Caveat**: these games ran before the heart's bouts were capped at a quarter of the squad (#633, 11-21). That cap targets exactly the bite bursts above.

## Proposals (none made; Direction's call)

1. **Move the last-stand heart from planet 4 to planet 5** (`FIRST_PLANET`'s `stand` 4 to 5), so planet 4 adds only the slime's mortar. This is Direction's first pick if the cliff at planet 4 outlasts the heart cap.
2. **Ramp the strikers' share** over planets 2 to 5 (`STRIKER_SHARE` 0.6, 0.75, 0.9, 1.0) instead of full from planet 3.
3. **Planet 5's size**: `tier_size` for the Sporer is 1.0 today. Direction decides it at Milestone 2.

**Next** (Direction, 11-21 04:53Z): after the bot's job 00-c, replay planets 4 and 5 on current main (with #633) with rush and lure on the same 9 maps (`00-d-curve-p45`, 36 games, queued on the bot branch with main through #636; both plans carry `siege=0` so the bot plays as in 00-b), paired against these rows. If planet 4 still drops, do proposal 1. The arms job that would test each lever (`set=stand=0`, `set=strikers=0` on planets 4 and 5) is held.
