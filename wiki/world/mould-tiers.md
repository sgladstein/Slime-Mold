---
title: The mould tiers (difficulty ladder)
kind: design
status: open
updated: 2026-11-11T22:00Z
sources: [Reports/larder-world-roadmap-2026-11-11.md, Reports/mould-tiers-2026-11-11.md, Reports/first-reach-2026-11-11.md, Reports/species-sets-2026-11-11.md, Reports/species-gpu-2026-11-11.md]
code: [BigParams::species_sets, BigParams::graze_weight, BigParams::explore_spread, BigParams::species_convert, BigParams::spore_rate, BigParams::sleep_rate]
---

# The mould tiers (difficulty ladder)

## Now

Larder World's difficulty comes from which mould you face, not from tuning
one mould (owner, 2026-11-11 14:14Z; ladder agreed 14:25Z). The ladder grows
as the research threads find new behaviours, and each tier must have a
counter the player can see work. The early leader's win rate is a per-mould
difficulty stat, not a gate (14:14Z).

| Tier | Mould | Rule | What beats it |
|---|---|---|---|
| 1 | Drifter | grazing bodies that crawl (`graze_weight`) | any squad |
| 2 | Sprawler | fast feathery fronts, thin tubes (`explore_spread`) | let it overreach, cut thin tubes, walls |
| 3 | Trunk | thick cords that heal round a cut (dense start, W0 growth) | starve it, heavy weapons |
| 4 | Hunter | hungry mould that drains soldiers | turrets, walls, bait |
| 5 | Rival pair | two species fighting over borders | lure one into the other |
| 6 | Converter | converts rivals where it outnumbers them (`species_convert`) | kill early, keep rivals apart |
| 7 | Mixes | several of the above | adapt the build |
| added | Sporer, Sleeper, Swarmer | spores, a dormant crust, walking into fire (`spore_rate`, `sleep_rate`, `alarm_weight`) | starve, bait then strike, a kill zone |

The added three are built on the CPU big dish and each beat its control on
8 of 8 seeds ([the life cycle](../model/life-cycle.md)); where they sit in
the order is not set. A splitting or migrating tier is open.

The Drifter's grazing is what makes it forage: on food piles it sends
small droplets to far food, while the edge of its body moves about 10
dish cells a minute. Adding 20% scouts to it reached more food on 10 of 16 seeds and
less on none, a candidate step up the ladder
([first reach](../../Reports/first-reach-2026-11-11.md)). Each species can
now have its own settings on one dish (`species_sets`;
[coexistence](../../Reports/species-sets-2026-11-11.md)).

## Open

- The order of the added tiers. On the GPU engine, species and
  conversion came with the outside rules, and each species' own settings,
  conversion over a square and a second kind followed (11-11, [species on
  the GPU](../../Reports/species-gpu-2026-11-11.md)), so a rival pair can
  pair a droplet species with a network one there. The life cycle (Sporer,
  Sleeper, Swarmer) is next, after the tiers' re-test at the world's
  operating point.
- Each tier's counter tested with the player AI ([player AI](player-ai.md)) once Larder runs in the world.

## Sources

- [larder-world-roadmap-2026-11-11.md](../../Reports/larder-world-roadmap-2026-11-11.md), *Mould progression*.
- [mould-tiers-2026-11-11.md](../../Reports/mould-tiers-2026-11-11.md): Sporer, Sleeper, Swarmer.
- [first-reach-2026-11-11.md](../../Reports/first-reach-2026-11-11.md): droplet moulds and mixes on food piles.
- [species-sets-2026-11-11.md](../../Reports/species-sets-2026-11-11.md): per-species settings, two species on one dish.
- [species-gpu-2026-11-11.md](../../Reports/species-gpu-2026-11-11.md): the species rules on the GPU engine.
