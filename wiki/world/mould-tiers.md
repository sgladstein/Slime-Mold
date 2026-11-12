---
title: The mould tiers (difficulty ladder)
kind: design
status: open
updated: 2026-11-12T16:44Z
sources: [Reports/mould-tiers-campaign-2026-11-12.md, Reports/larder-world-roadmap-2026-11-11.md, Reports/mould-tiers-2026-11-11.md, Reports/mould-tiers-review-2026-11-11.md, Reports/first-reach-2026-11-11.md, Reports/species-sets-2026-11-11.md, Reports/species-gpu-2026-11-11.md, Reports/life-gpu-2026-11-11.md]
code: [BigParams::scar_keep, BigParams::scar_weight, BigDish::scar, BigParams::species_sets, BigParams::graze_weight, BigParams::explore_spread, BigParams::species_convert, BigParams::spore_rate, BigParams::sleep_rate, BigParams::alarm_weight, Tier, Tier::core, Core]
---

# The mould tiers (difficulty ladder)

## Now

Larder World's difficulty comes from which mould you face, not from tuning
one mould (owner, 2026-11-11 14:14Z; ladder agreed 14:25Z). Each tier must
have a counter the player can see work; the early leader's win rate is a
per-mould difficulty stat, not a gate (14:14Z). A tier is a set of keys,
`mould::tiers::Tier`, which the game, the campaign's missions and the
censuses ask for by name (`tier=drifter`).

| Tier | Mould | Rule | What beats it |
|---|---|---|---|
| 1 | Drifter | round droplet bodies on crystals, crawling between them (`Tier::Drifter`) | any squad |
| 2 | Sprawler | a fast thin mesh over the map (`Tier::Sprawler`, explorers for 200 steps) | walls; chasing loses |
| 3 | Trunk | long straight cords round a few big bodies (`Tier::Trunk`) | starve it (needs shared food) |
| 4-7 | Hunter, Rival pair, Converter, Mixes | not built | |
| added | Sleeper, Swarmer, Sporer | the life cycle and the alarm ([the life cycle](../model/life-cycle.md)) | bait then strike; a kill zone; starve it |
| added | Wary, Grudge | a scar field that remembers where it was shot (`BigParams::scar_keep`), read away or toward | Wary: burn a road once; Grudge: don't fire twice from one spot |

The Drifter, Sprawler and Trunk (2026-11-12) are settings over rules both
engines run; the GPU keeps each within 2% of the CPU over 20 s. Wary keeps
away from a spot shot again and again (fewer walkers there 20 s after the
fire on 8 of 8 seeds); Grudge comes back to it in force (8 of 8 fresh seeds). The scar is
CPU only until it shares the toolkit's damage buffer on the GPU
([report](../../Reports/mould-tiers-campaign-2026-11-12.md)). In the
Larder world's race the Swarmer and the Sleeper's bait hold on 8 of 8, the
Sporer's edge is not significant
([report](../../Reports/mould-tiers-2026-11-11.md), *In the Larder world*).

`Tier::core` (2026-11-12) is None for every tier: the game's core, which
the whole body defends (`Rules::core_pull` 8). Headless is built.

## Open

- Each tier's core: Sprawler Headless, Trunk Regrow, Drifter Migrate,
  Grudge a stronger pull (the tiers thread, after the core PR).

- The census of the new tiers, re-run after shot craters (#113). Before craters (held-out 201-216): Grudge is the hardest (every plan lower than on plain, 16 of 16; random loses its squad 15 of 16); strike is the best plan on every tier, so the best plan does not change between tiers yet; the Sprawler now explores 200 steps (at 400 everything wiped it); the toolkit's blast plan wins every tier ([report](../../Reports/mould-tiers-campaign-2026-11-12.md), *The census*). Still to come: the Drifter, the rest of Wary, turrets (turtle, guard) against Grudge and Wary.
- Candidate tiers from review 2 ([the reviews](../../Reports/mould-tiers-review-2026-11-11.md)), most promising first: supply lines (traffic-made arteries that carry food; cut one to starve the far end; the one body thread's shared food), clans that fuse or fight along a scar, a mould that learns the rhythm of fire, a pacemaker heartbeat, day and night light, a nomad that avoids its own slime, two foods. Each has a first experiment there.
- The order of the added tiers. On the GPU engine, species and
  conversion came with the outside rules, and each species' own settings,
  conversion over a square and a second kind followed (11-11, [species on
  the GPU](../../Reports/species-gpu-2026-11-11.md)), so a rival pair can
  pair a droplet species with a network one there, and the life cycle
  (Sporer, Sleeper, Swarmer) followed (11-12).
- The tiers in the GPU world game: `Tier::settings` on the world's
  settings, the player's strike, the 1070's cost.
- Each tier's counter tested with the player AI ([player AI](player-ai.md)) once Larder runs in the world.

## Sources

- [larder-world-roadmap-2026-11-11.md](../../Reports/larder-world-roadmap-2026-11-11.md), *Mould progression*.
- [mould-tiers-2026-11-11.md](../../Reports/mould-tiers-2026-11-11.md): Sporer, Sleeper, Swarmer; *In the Larder world*.
- [mould-tiers-review-2026-11-11.md](../../Reports/mould-tiers-review-2026-11-11.md): two independent reviews and the response.
- [first-reach-2026-11-11.md](../../Reports/first-reach-2026-11-11.md): droplet moulds and mixes on food piles.
- [species-sets-2026-11-11.md](../../Reports/species-sets-2026-11-11.md): per-species settings, two species on one dish.
- [species-gpu-2026-11-11.md](../../Reports/species-gpu-2026-11-11.md): the species rules on the GPU engine.
- [life-gpu-2026-11-11.md](../../Reports/life-gpu-2026-11-11.md): the tiers' life cycle on the GPU engine.
