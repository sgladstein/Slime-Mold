# Lab: sorting the walkers by place (2026-11-24)

The Lab lane's eleventh job, on the lab branch `claude/project-thread-2o4cic`
only (code `247b6730`, the ablation key `a99924b4`), not in the game.
Brief from Direction 2026-11-24 08:07Z, for Milestone 2's test 5 (60 fps
at 4K on the GTX 1070): Enemy is cutting `sense` (38% of the simulation
step on planet 8) with bit-for-bit changes; the bigger win they named is
sorting the walkers by place now and then, so neighbours read neighbouring
memory. Prototype it on the GPU engine, show it acts the same statistically
on paired seeds (walkers, kills, spread on planets 2 and 8), and have
Engine time it once on the 1070. Enemy left the sort to the Lab (08:12Z).

Page: "Walker Sort" (source in `page/`, listed in `Reports/pages.md`).

## What was built

`LAB_SORT=SECS`: every SECS game seconds (from `set_ceiling`, once a game
second), every slot's state is moved so the living walkers sit in the
order of their 8x8-cell tile, then their cell in it, and the empty slots
after them. The order is stable and decided on the CPU from a read-back of
`walkers` and `energy`, so a run stays deterministic. Moved together:
`walkers`, `energy`, `spec`, `sick`, `age`, and in `news` each slot's five
words (news, gas, mind, dense, set; the dense word carries a walker's lump,
striker and roam bits, so boss, heart and origin membership travel with
it). `intent`, `icell` and `claim` live within a step. A birth's slot
(`child_of`) is a random slot over the whole mask, so it does not depend on
the order. `LAB_SORT_LOG=1` logs the slots moved, the time, and the
distinct 8x8 tiles a warp of 32 slots spans before and after.
`LAB_SORT_PACK=1` packs the living first in their old order, with no place
order (the ablation).

A slot's random draws (`keyed(.., k)`) follow its number, so a sorted run
is a different run of the same rules, not the same run: hence the paired
seeds.

## Results

**GTX 1070, planet 8, run seed 1** (Engine, 08:42Z; `bench=40 hand=greedy
passes=30`, the two runs one after the other, the league paused; files on
Scott's PC in `pc-batch-out\lab-sort-20261124`):

| | Off | Sorted every second |
|---|---|---|
| GPU a step, every pass timed alone | 9.456 ms | 5.269 ms |
| `sense` | 3.334 | 0.842 |
| `egain` | 0.865 | 0.165 |
| `life` | 0.435 | 0.142 |
| `walk` | 0.289 | 0.149 |
| `trail`, `scent`, `plant` (a cell each) | 1.156, 0.824, 0.206 | 1.155, 0.823, 0.205 |
| Frame p50, every frame (1280x800) | 9.27 ms | 7.58 ms |
| Frame p95 | 9.93 ms | 8.48 ms |
| GPU tail on step frames, p50 | 8.23 ms | 6.49 ms |
| Longest frame after the first second | 13.30 ms | 103.99 ms |

Every per-walker pass got faster (`sense` by 4x); the per-cell passes did
not move. The cost: each sort took 86 to 91 ms on the CPU (read-backs and
writes of about 680,000 slots), a 100 ms hitch once a second. A warp of
32 slots spanned 3.3 to 5.2 tiles before each sort and 1.9 to 2.3 after.

**The same rules, statistically** (lavapipe, `hand=greedy`, bank 3,000;
off against sorted every second; `raw/paired.txt`):

| Planet, seed, time | Walkers | Killed | Soldiers | Cells held | Spread (radius) |
|---|---|---|---|---|---|
| 8, 1, 1:00 | 154,173 / 154,573 (+0.3%) | 8,784 / 7,887 (-10.2%) | 16 / 16 | +0.4% | 329.3 / 327.9 |
| 8, 2, 1:00 | 146,830 / 147,064 (+0.2%) | 531 / 470 (-11.5%) | 20 / 20 | +0.4% | 352.3 / 353.5 |
| 8, 3, 1:00 | 159,638 / 159,966 (+0.2%) | 22,066 / 21,924 (-0.6%) | 4 / 5 | +0.2% | 336.6 / 336.8 |
| 2, 1, 2:00 | 16,199 / 16,338 (+0.9%) | 2,624 / 2,532 (-3.5%) | 32 / 32 | -0.2% | 159.5 / 156.1 |
| 2, 2, 2:00 | 11,793 / 11,992 (+1.7%) | 7,037 / 6,943 (-1.3%) | 28 / 28 | +1.9% | 133.7 / 135.6 |
| 2, 3, 2:00 | 14,379 / 14,179 (-1.4%) | 9,369 / 9,393 (+0.3%) | 33 / 33 | -1.4% | 147.5 / 143.6 |
| 2, 4, 2:00 | 14,622 / 14,949 (+2.2%) | 8,315 / 8,142 (-2.1%) | 20 / 27 | -1.1% | 101.3 / 104.2 |

Walkers within 2.2% and the cells held within 2% on every pair, the
spread (the walkers' radius about their centre) within 3%. Walkers were
higher sorted in 6 of 7 pairs, kills lower in 6 of 7: sign-test p 0.125
each (two-sided), not significant, and neither survives a correction for
the five measures. The kills' lean is worth one more look with more seeds,
since a shot's kill can depend on which slot a GPU thread reaches first.
(The sorted runs also ran 9% to 15% faster on lavapipe, 7 of 7; lavapipe
is not for timing.)

## Recommendation

**Build the sort, on the GPU.** It cuts the simulation step's GPU time
by 44% on planet 8 on the 1070 and plays the same within what seven paired seeds
can show. Before it lands:

- Engine and Enemy: the order made on the GPU (a counting sort by tile:
  count, prefix sum, scatter, then a gather of each per-slot buffer), so
  there is no read-back and no hitch. Keep it stable (rank by old slot
  within a tile) so a run stays deterministic.
- How often: locality decays within a second (planet 2: 2.5 tiles a warp
  after a sort, 7 one second later, about 10 after five), mostly because
  births land in random slots. A GPU sort every few frames, or births into
  a free slot near the parent, would hold it.
- The ablation (`LAB_SORT_PACK=1`, asked of Engine): on planet 8 only
  about a quarter of the slots live, so part of the win may be packing the
  living into the first warps rather than the place order. If packing is
  most of it, a cheaper compaction does most of the job.
- More seeds on the kills' lean.

## Raw

`raw/runs.sh` (one run), `raw/queue.sh` (the fourteen), `raw/ms.py` (the
paired table from logs and dumps), `raw/paired.txt` (its output).
