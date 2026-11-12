---
title: The squad toolkit
kind: tool
status: open
updated: 2026-11-12T10:31Z
sources: [Reports/squad-toolkit-2026-11-12.md, Reports/squad-toolkit-round2-2026-11-12.md, Reports/larder-world-roadmap-2026-11-11.md]
code: [Kit, KitRules, KitRules::carry_max, Kit::restock, Soldier::load, BigParams::germ_resist, Tool, BigParams::scorch_keep, BigParams::scorch_full, BigParams::sick_upkeep, BigParams::gas_death, BigDish::scorch_disc, BigDish::gas, BigDish::gassed_near, Soldier::smell, World::calm]
---

# The squad toolkit

## Now

Six tools for the Larder world's squad, each built on one term of the
mould's model (`mould::kit`, CPU world only): blast, mine, det cord, germ,
sleep gas, and mask or reek on a soldier's smell. The crater a blast or cord
leaves (`BigDish::scorch_disc`, trail fades fast and fades out) is the one
scorch in the game. With the crater a blast's hole holds 82 walkers at 10 s
against 188 without, of 370 (8 of 8 seeds). Since the Larder thread's PR 2
the crater is on the GPU too (binding 47) and the squad's shots lay it
(`Rules::crater`, 10 game seconds a hit), with shared defaults
`scorch_keep` 0.1 and `scorch_full` 30 (were 0.5 and 300; this thread's
probes found blasts and the cord no worse:
[report, *The hole under fire*](../../Reports/larder-world-roadmap-2026-11-11.md)). The germ kills about three
quarters of the mould once sickness costs energy, while steering-only
contagion does nothing. Sleep gas stops a crystal being eaten, and a blast
nearby wakes most sleepers. Mines are weak, and the det cord starves nothing
until food flows through the mould
([report](../../Reports/squad-toolkit-2026-11-12.md), *Tool by tool*). Four
plans built on them beat idle and random play on every tier (7-8 of 8), but
the loud plan (a runner carrying blasts to the mould's body) is best on all
four tiers, so the plans don't yet counter each other (*The four plans*).
Round 2 made tools take carry space, fetched at the larder (`KitRules`
default: a runner holds 5 at 50 carry each). Now the best plan changes with
the tier: loud on the Sleeper, Trunk, Drifter and Wary, plague (the germ) on
the Swarmer, Sporer and Sprawler, a tie on tier none and the Grudge (8 seeds,
by the mean; not yet significant). Gas and the det cord are never best
([round 2](../../Reports/squad-toolkit-round2-2026-11-12.md)). Pictures and
counts are on the [page](https://claude.ai/artifact/Vr6ysJedMVtEa6XXEA1pTC).

## Owner's word

- 2026-11-12 04:25Z: "yes and maybe explosions, mines, etc.  do we need scortch when we have walls?"
- 2026-11-12 04:31Z (card): "All six together".
- 2026-11-12 04:39Z: "A group of soldiers shooting the slime should start carving out a bigger and bigger hole that doesn't refill while being shot at."
- 2026-11-12 04:46Z (card): "Drop it, scorch is the crater".

## Tried and rejected

- Steering-only contagion as the germ: no effect at 30 s (2 higher, 6 lower against none), as in [`dead-ends.md`](../../Reports/dead-ends.md), contagion.
- A blast price of 120 and a reach of 15 dish cells to slow the loud plan: it still wiped the mould on 4 of 4 seeds ([report](../../Reports/squad-toolkit-2026-11-12.md), *The four plans*).

- Carry 2 tools at weight 100 or 50: loud fell to the farm hand's level and plague led every tier; weight made no difference, the trips home are the cost ([round 2](../../Reports/squad-toolkit-round2-2026-11-12.md), *Counter 1*).
- Amputation by steering (healthy walkers steer from germ, sick ones lay no trail): no effect on the germ ([round 2](../../Reports/squad-toolkit-round2-2026-11-12.md), *Answers to the germ*).

## Open

- Confirm round 2 on main with shot craters on (#113), 16 seeds, nine tiers, and two maps.
- A tier where gas or the det cord is the best plan.
- `germ_resist` as a tier key, if a campaign mould needs the germ answered.
- The tools as `squad::Frame` bursts on the CPU and GPU, then in the window (step 2).
- The det cord once food flows (the one-body thread's `energy_flow`).

## Sources

- [squad-toolkit-2026-11-12.md](../../Reports/squad-toolkit-2026-11-12.md), all sections.
- [squad-toolkit-round2-2026-11-12.md](../../Reports/squad-toolkit-round2-2026-11-12.md), all sections.
