---
title: The mould tiers (difficulty ladder)
kind: design
status: open
updated: 2026-11-12
sources:
  - Reports/first-reach-2026-11-11.md
  - Reports/larder-world-roadmap-2026-11-11.md
  - Reports/life-gpu-2026-11-11.md
  - Reports/mould-tiers-2026-11-11.md
  - Reports/mould-tiers-campaign-2026-11-12.md
  - Reports/mould-tiers-review-2026-11-11.md
  - Reports/species-gpu-2026-11-11.md
  - Reports/species-sets-2026-11-11.md
code:
  - BigDish::scar
  - BigParams::alarm_weight
  - BigParams::explore_spread
  - BigParams::graze_weight
  - BigParams::scar_keep
  - BigParams::scar_weight
  - BigParams::sleep_rate
  - BigParams::species_convert
  - BigParams::species_sets
  - BigParams::spore_rate
  - Core
  - CoreKind
  - Tier
  - Tier::core
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
| 1 | Drifter | round droplet bodies on crystals; hungry walkers follow food and soldiers off a dry crystal (`Tier::Drifter`, hunger 16, reach 18) | any squad |
| 2 | Sprawler | a fast thin mesh over the map (`Tier::Sprawler`, explorers for 200 steps) | walls; chasing loses |
| 3 | Trunk | long straight cords round a few big bodies (`Tier::Trunk`) | starve it (needs shared food) |
| 4-7 | Hunter, Rival pair, Converter, Mixes | not built | |
| added | Sleeper, Swarmer, Sporer | the life cycle and the alarm ([the life cycle](../model/life-cycle.md)) | bait then strike; a kill zone; starve it |
| added | Grudge | a scar field that remembers where it was shot (`BigParams::scar_keep`), read toward, with the Swarmer's alarm | don't fire twice from one spot; random fire loses its squad |

The Drifter, Sprawler and Trunk (2026-11-12) are settings over rules both
engines run; the GPU keeps each within 2% of the CPU over 20 s. Grudge comes
back in force to a spot shot again and again (8 of 8 fresh seeds); the
avoiding twin, Wary, kept away from it but lost to random fire and was
removed. The scar is
CPU only until it shares the toolkit's damage buffer on the GPU
([report](../../Reports/mould-tiers-campaign-2026-11-12.md)). In the
Larder world's race the Swarmer and the Sleeper's bait hold on 8 of 8, the
Sporer's edge is not significant
([report](../../Reports/mould-tiers-2026-11-11.md), *In the Larder world*).

`Tier::core` (2026-11-12) gives the Sprawler no pull and the Grudge
twice the plain pull (16); every other tier keeps the game's core, which
the whole body defends (`Rules::core_pull` 8). Since the owner's ruling of
2026-11-12 20:21Z every core is headless: holding a mould's home stops
nothing, and only a wipe ends it.

## Open

- Each tier's core: all headless since 11-12 20:21Z (no king rule); the Sprawler pulls nothing and the Grudge pulls at 16 (twice the plain mould's 8; `tiers::GRUDGE_PULL`). The Trunk's regrowing core (`CoreKind::Regrow`, PR #163: a fallen core grew back unless the body starved) was taken out in PR 4; the tiers thread dropped it and the Drifter's Migrate under the no-king ruling (20:53Z): with no core to take, the Trunk keeps its tube network as its identity ([dead end](../../Reports/dead-ends.md)).

- The census of the new tiers, re-run after shot craters (#113). Before craters (held-out 201-216): Grudge is the hardest (every plan lower than on plain, 16 of 16; random loses its squad 15 of 16);
- Candidate tiers from review 2 ([the reviews](../../Reports/mould-tiers-review-2026-11-11.md)), most promising first: supply lines (traffic-made arteries that carry food; cut one to starve the far end; the one body thread's shared food), clans that fuse or fight along a scar, a mould that learns the rhythm of fire, a pacemaker heartbeat, day and night light, a nomad that avoids its own slime, two foods. Each has a first experiment there.
- The order of the added tiers. On the GPU engine, species and
  conversion came with the outside rules, and each species' own settings,
  conversion over a square and a second kind followed (11-11, [species on
  the GPU](../../Reports/species-gpu-2026-11-11.md)), so a rival pair can
  pair a droplet species with a network one there, and the life cycle
  (Sporer, Sleeper, Swarmer) followed (11-12).
- The tiers in the GPU world game: `Tier::settings` on the world's
  settings, the player's strike. (On the GTX 1070 each tier's life cycle
  adds at most 0.74 ms a frame at 100,000 walkers: [life cycle on the
  GPU](../../Reports/life-gpu-2026-11-11.md), *On the GTX 1070*.)
- Each tier's counter tested with the player AI ([player AI](player-ai.md)) once Larder runs in the world.

## Sources

- [larder-world-roadmap-2026-11-11.md](../../Reports/larder-world-roadmap-2026-11-11.md), *Mould progression*.
- [mould-tiers-2026-11-11.md](../../Reports/mould-tiers-2026-11-11.md): Sporer, Sleeper, Swarmer; *In the Larder world*.
- [mould-tiers-review-2026-11-11.md](../../Reports/mould-tiers-review-2026-11-11.md): two independent reviews and the response.
- [first-reach-2026-11-11.md](../../Reports/first-reach-2026-11-11.md): droplet moulds and mixes on food piles.
- [species-sets-2026-11-11.md](../../Reports/species-sets-2026-11-11.md): per-species settings, two species on one dish.
- [species-gpu-2026-11-11.md](../../Reports/species-gpu-2026-11-11.md): the species rules on the GPU engine.
- [life-gpu-2026-11-11.md](../../Reports/life-gpu-2026-11-11.md): the tiers' life cycle on the GPU engine.
