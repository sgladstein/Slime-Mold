---
title: The mould tiers (difficulty ladder)
kind: design
status: open
updated: 2026-11-12T00:10Z
sources: [Reports/larder-world-roadmap-2026-11-11.md, Reports/mould-tiers-2026-11-11.md, Reports/mould-tiers-review-2026-11-11.md]
code: [BigParams::graze_weight, BigParams::explore_spread, BigParams::species_convert, BigParams::spore_rate, BigParams::sleep_rate, BigParams::alarm_weight, Tier]
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
| added | Sleeper | a nest: a dormant crust round its home crystal that wakes in a wave (`Tier::Sleeper`) | bait it, then strike (proposed) |
| added | Swarmer | floods where it is hit (`Tier::Swarmer`, `alarm_weight` +3); a modifier any tier can carry | a kill zone (proposed) |
| added | Sporer | fed walkers fly to new crystals (`Tier::Sporer`) | starve it, don't strike near untaken crystals (proposed) |

The added three are on the CPU big dish, named for the game by
`mould::tiers::Tier`, and the squad's shots lay the alarm they answer. In
the Larder world's race (growth, crystals that run dry) the Swarmer and the
Sleeper's bait hold on 8 of 8 seeds, but the Sporer's edge over the plain
mould is not significant, and the Sleeper barely leaves its home crystal,
so it suits an early tier or a map feature
([the life cycle](../model/life-cycle.md)). Their place in the order is not
set; none is on the GPU yet.

## Open

- Candidate tiers from review 2 ([the reviews](../../Reports/mould-tiers-review-2026-11-11.md)), most promising first: supply lines (traffic-made arteries that carry food; cut one to starve the far end), habituation (a mould that learns to ignore a repeated trick), clans that fuse or fight along a scar, a mould that learns the rhythm of fire, a pacemaker heartbeat, day and night light, a nomad that avoids its own slime, two foods. Each has a first experiment there.
- The order of the added tiers, and which tiers reach the GPU first (after W0).
- Each tier's counter tested with the player AI ([player AI](player-ai.md)) once Larder runs in the world.

## Sources

- [larder-world-roadmap-2026-11-11.md](../../Reports/larder-world-roadmap-2026-11-11.md), *Mould progression*.
- [mould-tiers-2026-11-11.md](../../Reports/mould-tiers-2026-11-11.md): Sporer, Sleeper, Swarmer; *In the Larder world*.
- [mould-tiers-review-2026-11-11.md](../../Reports/mould-tiers-review-2026-11-11.md): two independent reviews and the response.
