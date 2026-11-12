---
title: The squad toolkit
kind: tool
status: open
updated: 2026-11-12T17:25Z
sources: [Reports/flashes-of-chaos-guns-2026-11-12.md, Reports/squad-toolkit-2026-11-12.md, Reports/squad-toolkit-round2-2026-11-12.md, Reports/larder-world-roadmap-2026-11-11.md]
code: [Kit, KitRules, KitRules::carry_max, Kit::restock, Soldier::load, BigParams::germ_resist, Tool, BigParams::scorch_keep, BigParams::scorch_full, BigParams::sick_upkeep, BigParams::gas_death, BigDish::scorch_disc, BigDish::gas, BigDish::gassed_near, Soldier::smell, World::calm]
---

# The squad toolkit

## Now

Six tools for the Larder world's squad, each built on one term of the
mould's model (`mould::kit`, CPU world only): blast, mine, det cord, germ,
sleep gas, and mask or reek on a soldier's smell. The crater a blast, cord
or shot leaves (`BigDish::scorch_disc`; GPU binding 47 since #113, shared
`scorch_keep` and `scorch_full`: 0 and 150 in the race world since PR 3b's
cone, were 0.1 and 30, which earlier `kitplans` tables used) is the one
scorch in the game. Each tool
shows in the mould; mines are weak and the det cord starves nothing until
food flows ([report](../../Reports/squad-toolkit-2026-11-12.md)). Tools take
carry space and come from the larder (`KitRules`: a runner holds 5 at 50
carry each). At 32 seeds on main, Holm over the table, the best plan changes
with the tier: loud (blasts carried to the body) over plague (the germ) on
the Drifter (30/2), plague over loud on the Sporer (30/2); by the mean
plague also leads on none, Swarmer, Sprawler and Grudge, loud on Sleeper
and Trunk. Random play beats or ties every plan on the Drifter and the Trunk
(gate 1 fails there, before the core pull of #126; re-run on main pending). Gas and the det cord are never best
([round 2](../../Reports/squad-toolkit-round2-2026-11-12.md), *Confirmed at 32
seeds*; [page](https://claude.ai/artifact/Vr6ysJedMVtEa6XXEA1pTC)).

Idea, not built (2026-11-12): **rounds**, after the ammo-card guns of
Flashes of Chaos and Noita's wands. A soldier's or turret's shot becomes up to
four cards in order (payloads such as crater, bait, alarm, germ, sleep, scar;
modifiers such as pierce, scatter, quiet; triggers that read the mould, such as
"when thick" or carried along the tubes), so the toolkit's tools ride on shots
and the best loadout can vary by tier
([research](../../Reports/flashes-of-chaos-guns-2026-11-12.md)); reviewers: compile rounds onto
this toolkit's burst list, not a new system.

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

- Random play wipes the Drifter and the Trunk at 945e080 (gate 1 fails): re-run on main with the core pull (#126) and the new Drifter (#133).
- Round 2 on a second map, and as `larderworld census` hands.
- A tier where gas or the det cord is the best plan.
- `germ_resist` as a tier key, if a campaign mould needs the germ answered.
- The tools as `squad::Frame` bursts on the CPU and GPU, then in the window (step 2).
- The det cord once food flows (the one-body thread's `energy_flow`).

## Sources

- [squad-toolkit-2026-11-12.md](../../Reports/squad-toolkit-2026-11-12.md), all sections.
- [squad-toolkit-round2-2026-11-12.md](../../Reports/squad-toolkit-round2-2026-11-12.md), all sections.
