---
title: Growth and starving on the big dish
kind: mechanism
status: open
updated: 2026-11-11T17:10Z
sources: [Reports/growth-2026-11-11.md, Reports/mould-shape-2026-11-11.md, Reports/larder-world-roadmap-2026-11-11.md]
code: [BigParams::upkeep, BigParams::start_energy, BigParams::birth_energy, BigParams::birth_cost, BigParams::starve, BigParams::capacity, BigDish::alive, BigDish::energy, DRAW_STARVE, DRAW_BIRTH, CLAIM_BIRTH]
---

# Growth and starving on the big dish

## Now

The big dish's walker count can follow its food (Larder World W0). With
`BigParams::upkeep` above 0 (default 0, off) each walker holds energy, eats
its equal share of the grass its cell loses, pays upkeep, starves at zero
(dies with chance `starve` a step) and divides at `birth_energy` into a
free agar cell beside it. Walkers live in `capacity` fixed slots, and a
child takes its parent's slot xor one keyed mask a step, so no order
decides who is born and a GPU thread a slot can run it. A fed mould grows
and a starved one dies on 8 of 8 seeds, and the size it settles at is
nearly linear in the food: doubling the grass regrowth gives 1.9 times the
walkers. It turns over fast: a walker lives about 3 s. Growth off leaves
every dish exactly as it was. On the GPU it is three passes that match the
CPU to the bit in a one-tick twin test in CI, and on the GTX 1070 it adds
0.16 ms to a 1.82 ms frame at 100,000 walkers. In the GPU sandbox it is
the GROWTH panel page, with a fed and a starving mould on Ctrl+3 and 4.

With Larder's crystals on (main since PR #64), a walker's crystal take
(`took`) is food like its grass share, empty slots take nothing, and there
is no grass unless `graze_weight` asks for it (`growth_eats_crystals`).
On those crystals (one seed, first run on a scratch merge): with
crystals that refill, a net of tubes linking them holds at about 2,000
walkers; with crystals that run dry, the mould starves by 60 s with 61% of
the charge left, because it eats only the cells its tubes cross. With
Larder's `crystal_flow` (charge evens out inside a crystal) it eats 99% of
the charge, peaks at 2,400 against 1,454, and starves at about 20 s
because the food is gone. On crystals the size stops near 20,000 whatever
the charge (the Larder thread's 300 s runs): one walker a cell stands on a
crystal and bites 0.005, so the crystals' area sets the intake and the
intake sets the size, while walkers packed on the crystals hoard (mean
energy 26-30). A scratch `energy_flow` (energy evens out between
neighbouring walkers) turns the hoard into 15-24% more walkers, not more
with charge ([mould shape](../../Reports/mould-shape-2026-11-11.md)). Grass
under the crystals swamps them: the mould fills its slots by 5 s and eats
every crystal by 10 s.

## Numbers

| Measure | Result | Seeds | Paired | Source |
|---|---|---|---|---|
| Living walkers at 30 s, nose dish 1024 x 640, 10,000 to start in 100,000 slots, upkeep 0.01, grass regrowing 0.002 / none | 96,358 (92,641-96,909) / 428 (0-9,420) | 8 | fed over starved 8 of 8, p 0.0078, survives Holm over 3 | [growth, *Fed against starved*](../../Reports/growth-2026-11-11.md) |
| Living walkers at 60 s, 512 x 320, 2,500 to start, regrowth 0.0005 / 0.001 / 0.002 | 7,113 / 13,522 / 25,029 (ratios 1.90 and 1.87) | 8 | each step up 8 of 8, p 0.0078, both survive Holm | same |
| Turnover at that steady state (0.002, seed 1, 15-60 s) | about 8,700 born and 8,700 starved a second against 25,000 alive | seed 1 | | same |
| Growth off against main before growth, three dishes, 3 s | every measure line and the hash the same | seed 1 | | same |
| CPU cost, 100,000 walkers, one thread, growth on with nothing born or starving / off | 26.80-28.35 / 27.30-27.67 ms a tick (3 rounds; within noise) | seed 1 | | [growth, *Cost*](../../Reports/growth-2026-11-11.md) |
| GPU against CPU, one tick, software Vulkan, seven dishes (one a cell, no limit, Jenson's 1M slots, species and explorers and contagion, the cap, births beside deaths) | to the bit on five; energy L1 6.1e-6 and 1.7e-5, trail L1 1.0e-8 on the two that sum a cell's walkers in another order; 20 planted bugs all fail it | one tick | | [growth, *On the GPU*](../../Reports/growth-2026-11-11.md) |
| GTX 1070, 100,000 walkers with grazing, GPU ms of ticks a frame of two, growth off / on with nothing born or starving / the fed dish growing | 1.823-1.828 / 1.987-1.993 / 1.787; passes 0.066 ms a tick | seed 1, 3 rounds | | same |

## Owner's word

- 2026-11-11 14:06Z: "Crystal should run out which forces you to expand."
  (the Larder World ruling that needs growth; [decisions](../decisions.md)).
- 2026-11-11 14:54Z (Larder thread), on crystals that run out: "If you do nothing, the
  slime should get much stronger than you and kill you."

## Tried and rejected

- Pairing slots by cycling single bits: a small mould bred about one step
  in five ([`dead-ends.md`](../../Reports/dead-ends.md)).

## Open

- Run-dry crystals: `crystal_flow` makes the whole disc reachable (one
  seed); for a mould left alone to keep growing, a crystal's yield has to
  rise with its charge (the Larder thread's rule).
- `energy_flow` (scratch, this thread's branch): keep it for the
  cut-a-tube lever? A cut has not been tested.

## Sources

- [growth](../../Reports/growth-2026-11-11.md), all sections.
- [mould shape](../../Reports/mould-shape-2026-11-11.md), grass and crystals, size on crystals.
- [Larder World roadmap](../../Reports/larder-world-roadmap-2026-11-11.md), W0.
