---
title: The squad toolkit
kind: tool
status: open
updated: 2026-11-12
sources: [Reports/squad-toolkit-2026-11-12.md]
code: [Kit, KitRules, Tool, BigParams::scorch_keep, BigParams::scorch_full, BigParams::sick_upkeep, BigParams::gas_death, BigDish::scorch_disc, BigDish::gas, BigDish::gassed_near, Soldier::smell, World::calm]
---

# The squad toolkit

## Now

Six tools for the Larder world's squad, each built on one term of the
mould's model (`mould::kit`, CPU world only): blast, mine, det cord, germ,
sleep gas, and mask or reek on a soldier's smell. The crater a blast or cord
leaves (`BigDish::scorch_disc`, trail fades fast and fades out) is the one
scorch in the game. With the crater a blast's hole holds 82 walkers at 10 s
against 188 without, of 370 (8 of 8 seeds). The germ kills about three
quarters of the mould once sickness costs energy, while steering-only
contagion does nothing. Sleep gas stops a crystal being eaten, and a blast
nearby wakes most sleepers. Mines are weak, and the det cord starves nothing
until food flows through the mould
([report](../../Reports/squad-toolkit-2026-11-12.md), *Tool by tool*). Four
plans built on them beat idle and random play on every tier (7-8 of 8), but
the loud plan (a runner carrying blasts to the mould's body) is best on all
four tiers, so the plans don't yet counter each other (*The four plans*).
The fix proposed is a tier that answers a blast. Pictures and counts are on the
[page](https://claude.ai/artifact/Vr6ysJedMVtEa6XXEA1pTC).

## Owner's word

- 2026-11-12 04:25Z: "yes and maybe explosions, mines, etc.  do we need scortch when we have walls?"
- 2026-11-12 04:31Z (card): "All six together".
- 2026-11-12 04:39Z: "A group of soldiers shooting the slime should start carving out a bigger and bigger hole that doesn't refill while being shot at."
- 2026-11-12 04:46Z (card): "Drop it, scorch is the crater".

## Tried and rejected

- Steering-only contagion as the germ: no effect at 30 s (2 higher, 6 lower against none), as in [`dead-ends.md`](../../Reports/dead-ends.md), contagion.
- A blast price of 120 and a reach of 15 dish cells to slow the loud plan: it still wiped the mould on 4 of 4 seeds ([report](../../Reports/squad-toolkit-2026-11-12.md), *The four plans*).

## Open

- A tier that answers a blast (scatters spores, or learns to avoid the alarm), then the plans census again on 16 seeds and two maps.
- The tools as `squad::Frame` bursts on the CPU and GPU, then in the window (step 2).
- The det cord once food flows (the one-body thread's `energy_flow`).

## Sources

- [squad-toolkit-2026-11-12.md](../../Reports/squad-toolkit-2026-11-12.md), all sections.
