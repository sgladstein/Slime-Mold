# What shapes the mould: food layout, smell, walker count (2026-11-11)

Answers to the owner's questions of 16:01-16:05Z ("why does the mold with
the crystals look so different than food?", "Do we want grass and crystals
on the same dish for the real game. How does it change behavior? ... the
number of walkers seems to drastically change the overall slime
behavior"), plus a finding from the Larder thread's crystal sizing runs.
Page: [What Shapes the Mould](https://claude.ai/artifact/7YCmZjZyB4f4LMDhCKmzpt)
(source `mould-shape-2026-11-11/page.html`).

In one line: the amount of food sets how big the mould gets, and what the
walkers follow (fresh grass, a smell, the crowd limit) sets its shape.

## Grass and crystals in one world (one seed, a look)

Growth (W0) on the Larder thread's crystals branch (f292952, crystals that
run dry with `crystal_flow` 0.01) plus one scratch commit (65d914f: a
walker's crystal take into its energy, the dead take nothing, no grass with
crystals unless `graze_weight` is set). The Larder world: 512 x 320, 23
crystals at charge 1, `crystal_eat` 0.005, 5,000 walkers in 50,000 slots,
upkeep 0.001, seed 1, 60 s. Grass, where on, starts full on every cell with
`graze_weight` 10. Script `mould-shape-2026-11-11/grass-crystals.sh`, raw
`raw/gc-*.txt`, stills `stills/`.

| Food | Alive at 5 / 10 / 20 / 60 s | Loops at the same times | Left at 60 s |
|---|---|---|---|
| Crystals only | 2,297 / 2,400 / 515 / 20 | 3 / 1 / 0 / 0 | crystals 0.8% |
| Crystals on grass that never regrows | 49,548 / 48,069 / 33 / 0 | 42 / 30 / 0 / 0 | grass 0.1%, crystals 0 |
| Crystals on grass regrowing 0.0001 a step | 49,576 / 49,575 / 14,077 / 14,312 | 46 / 74 / 95 / 104 | grass 2.5%, crystals 0 |
| Grass regrowing 0.0001, piles that only smell (charge 0) | 49,449 / 48,705 / 9,021 / 12,444 | 35 / 24 / 6 / 14 | grass 9% |

- Crystals alone give roads between the crystals; the mould eats 99% of
  the charge and is nearly gone by 30 s.
- Grass that never regrows is a boom and bust: the 50,000 slots fill by
  5 s, everything is eaten by 10 s, and the mould is dead by 40 s, sooner
  than on crystals alone.
- Grass that regrows holds a steady size (about 14,000) in an even net of
  small cells over the whole dish; the crystals are gone in 10 s.
- Smell bends the net into roads: with charge 0, `crystal_left` returns 1,
  so the piles call at full strength and feed nothing, and the net comes
  out coarse with thick tubes between the piles. Eaten crystals stop
  calling, so in the row above only the grass steers.
- Full grass on every cell swamps 23 crystals at charge 1. If the game has
  both, crystals must be far richer than grass or grass far thinner.
- Three of four runs hit the 50,000-slot cap by 5 s, so the slots cap their
  boom. One seed: looks, not a census.

## Walker count and the crowd limit (8 seeds)

The GPU sandbox's Jenson preset (`mouldgpu preset=3`, commit b1f0383),
1024 x 640, no food, changing only `walkers` and `crowd`. Deciding measure,
named before the runs: closed loops at 30 s. The GTX 1070 ran the limit off
at 32,000, 100,000, 300,000 and 1,000,000 walkers (10 s and 30 s; 1M seed 8
cut off) before its C: drive filled up (`raw/gtx1070-sweep.txt`, copied by
hand by the PC's session). The limit on, the limit off at 200,000 and every
picture ran in the cloud on llvmpipe with the same code
(`walker-sweep-cloud.sh`, `raw/cloud-sweep.txt`). With `crowd=1` the
sandbox forces the limit off above half the agar: 300,000 prints
`(crowd 0)`, so the third limit-on arm is 200,000.

| Walkers | Loops at 30 s, limit off (8 seeds) | Loops at 30 s, limit on (8 seeds) | Tube cells at 30 s, off / on |
|---|---|---|---|
| 32,000 | 0 (1070), 0 (cloud) | 43-54 | 1,138-1,926 / 6,948-7,612 |
| 100,000 | 0 (1070), 0-1 (cloud) | 57-70 | 1,113-2,514 / 10,123-10,493 |
| 200,000 | 0-1 (cloud) | 120-144 | 1,318-2,421 / 15,126-15,373 |
| 300,000 | 0 (1070) | forced off | 842-1,623 |
| 1,000,000 | 0 (1070, 7 seeds) | forced off | 1,063-1,810 |

- Limit on against off: more loops on 8 of 8 seeds at 32k, 100k and 200k
  (sign test p 0.0078 each, cloud runs, same device). More walkers with the
  limit on: 100k over 32k and 200k over 100k on 8 of 8 (p 0.0078 each).
  All five survive Holm (smallest adjusted p 0.039).
- Without the limit, count changes nothing: 0-1 loops at every count from
  32,000 to 1,000,000; the 1070 and the cloud agree. With it, count sets the
  shape: a lacy net at 32,000, a denser one at 100,000, a sheet with holes
  at 200,000.
- Why (inferred from the rule, not tested apart): with one walker a cell,
  walkers cannot all stand on the strongest tube; a blocked walker turns
  and lays trail beside it, so tubes widen, branch and close, and the mould
  must cover at least as many cells as it has walkers.
- With growth on, the count follows the food, so with the limit on a
  growing mould thickens from net to sheet and a starving one thins back.
  The sandbox's big presets run above the forced-off line, which is why
  they come out as ropes.
- Cost on the 1070, limit off, GPU ms for a frame of two ticks (seed 1,
  30 s): 0.69 / 0.89 / 1.34 / 3.01 at 32k / 100k / 300k / 1M.

## On crystals, size follows intake, not charge (one seed each)

The Larder thread's sizing runs (PR #64,
`Reports/larder-world-roadmap-2026-11-11/growth-sizing/runs-after-fix.txt`,
its command with `crystal_charge` 64, 512, 4096, 300 s) found the mould
levelling off at 18-24k walkers whatever the charge while mean energy
climbed to 26-30: walkers packed on the crystals can divide only into a
free cell beside them, so they hoard. A scratch rule, `energy_flow` (cry-wt
commit 116c0ec, on this thread's branch, not in the W0 PR), evens each
cell's mean walker energy with its occupied eight neighbours after eating,
a pair giving what the other takes. Raw: `growth-2026-11-11/raw/energy-flow.txt`.

| Charge a cell | Alive at 300 s, as built / flow 0.05 | Mean energy at 300 s, as built / flow 0.05 |
|---|---|---|
| 64 | 141 / 2,309 | 1.7 / 0.89 |
| 512 | 21,015 / 24,206 (flow 0.12: 20,782) | 26.4 / 0.66 |
| 4,096 | 18,200 / 22,515 | 30.0 / 0.69 |

Flow turns the hoard into 15-24% more walkers, but size still does not rise
with charge. One walker a cell stands on a crystal and bites 0.005 a step,
so the crystals' area sets the intake, and intake over upkeep (0.001, plus
birth costs) sets the size; the charge sets only how long it lasts. Higher
flow (0.12) evens energy below `birth_energy` and fewer walkers divide. For
"if you do nothing, the slime should get much stronger than you" (owner,
14:54Z), a crystal's yield has to grow with its charge (a bigger bite from a
richer cell, or richer crystals covering more cells); that is the Larder
thread's rule. `energy_flow` stays a scratch option: it would also make a
cut tube starve the far side, which has not been tested.

## Open

- A cut tube with `energy_flow` on: does the far side starve, and how fast?
- The crowd limit's shape with growth on, over a growing and a starving
  mould on the big dish (net to sheet and back), seen in a video.
- A crystal yield tied to charge (Larder thread).
