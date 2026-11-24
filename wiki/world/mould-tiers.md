---
title: The mould tiers (difficulty ladder)
kind: design
status: open
updated: 2026-11-24
sources:
  - Reports/first-reach-2026-11-11.md
  - Reports/gun-route-2026-11-17/README.md
  - Reports/lab-pairing-2026-11-24/README.md
  - Reports/lab-scar-tissue-2026-11-24/README.md
  - Reports/lab-sleeper-2026-11-24/README.md
  - Reports/lab-threads-2026-11-24/README.md
  - Reports/lab-veins-2026-11-19/README.md
  - Reports/larder-world-roadmap-2026-11-11.md
  - Reports/life-gpu-2026-11-11.md
  - Reports/mould-tiers-2026-11-11.md
  - Reports/mould-tiers-campaign-2026-11-12.md
  - Reports/mould-tiers-review-2026-11-11.md
  - Reports/sim-use-2026-11-17/step0-scar-on-gpu.md
  - Reports/slime-review-2026-11-15/README.md
  - Reports/species-gpu-2026-11-11.md
  - Reports/species-sets-2026-11-11.md
code:
  - BigDish::scar
  - BigParams::alarm_weight
  - BigParams::explore_spread
  - BigParams::germ_resist
  - BigParams::germ_upkeep
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

In a run each tier also does one thing to you: [the run](run.md).

The Drifter, Sprawler and Trunk (2026-11-12) are settings over rules both
engines run; the GPU keeps each within 2% of the CPU over 20 s. Grudge comes
back in force to a spot shot again and again (8 of 8 fresh seeds); the
avoiding twin, Wary, kept away from it but lost to random fire and was
removed ([report](../../Reports/mould-tiers-campaign-2026-11-12.md)). The
scar runs on the GPU since 2026-11-17, in the toolkit's damage buffer's
second block, twin-tested to 1e-6 a cell, and the Grudge is the run's
planet 7 (`run::LADDER`; [report](../../Reports/sim-use-2026-11-17/step0-scar-on-gpu.md); one film there, a standing squad gone 8 s sooner with it on). In the
Larder world's race the Swarmer and the Sleeper's bait hold on 8 of 8, the
Sporer's edge is not significant
([report](../../Reports/mould-tiers-2026-11-11.md), *In the Larder world*).

Each slime meets the germ round with its own resistance (`germ_resist`,
which multiplies the level a walker catches at) and frailty
(`germ_upkeep`, which multiplies what a sick walker pays a step), both
2026-11-17 on the game designer's retune: shape C resists 10, kept by the
Sprawler and Trunk; the Swarmer and Grudge resist 15; the Sporer, Drifter
and Sleeper resist 1 and are frail 6. The Sleeper's sleeping crust neither
catches the germ nor pays for it, so the germ is useless on it until a
Flare wakes it ([route](../../Reports/gun-route-2026-11-17/README.md)).

`Tier::core` (2026-11-12) gives the Sprawler no pull and the Grudge
twice the plain pull (16); every other tier keeps the game's core, which
the whole body defends (`Rules::core_pull` 8). Since the owner's ruling of
2026-11-12 20:21Z every core is headless: holding a mould's home stops
nothing, and only a wipe ends it.

## Open

- The slime enemy review (11-15, [report](../../Reports/slime-review-2026-11-15/README.md)): every tier changes how the slime moves more than what it goes after, and only soldier scent, the home pull, the panic charge and tier specials point it at the player. The run's ladder (`run::LADDER`) had neither the plain C mould nor the Grudge (the Grudge joined at planet 7 on 11-17), and its planets played the old race keys rather than C's until the run thread's fix. Questions to the owner on the slime's main threat are open.
- Each tier's core: all headless since 11-12 20:21Z (no king rule); the Sprawler pulls nothing and the Grudge pulls at 16 (twice the plain mould's 8; `tiers::GRUDGE_PULL`). The Trunk's regrowing core (`CoreKind::Regrow`, PR #163: a fallen core grew back unless the body starved) was taken out in PR 4; the tiers thread dropped it and the Drifter's Migrate under the no-king ruling (20:53Z): with no core to take, the Trunk keeps its tube network as its identity ([dead end](../../Reports/dead-ends.md)).

- The census of the new tiers, re-run after shot craters (#113). Before craters (held-out 201-216): Grudge is the hardest (every plan lower than on plain, 16 of 16; random loses its squad 15 of 16);
- Candidate tiers from review 2 ([the reviews](../../Reports/mould-tiers-review-2026-11-11.md)), most promising first: supply lines (traffic-made arteries that carry food; cut one to starve the far end; the one body thread's shared food), clans that fuse or fight along a scar, a mould that learns the rhythm of fire, a pacemaker heartbeat, day and night light, a nomad that avoids its own slime, two foods. Each has a first experiment there. Added 11-13 (design review of #198, where shots need a line of sight): a mould that grows round cover, behind walls, rock and crystals, where shots can't reach, so the player has to move rather than sit.
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
- The Sprawler's body as threads (lab, 11-24, its branch only): two rules together turn planet 2's packed disc round a crystal into four to seven spokes with bare ground between, at the same walker count: lanes (a calm walker may step into a full cell holding as much trail as its own, up to 3 more there) and a shuttle (a walker near its birth energy walks down the crystals' scent, back up it when hungry), with the trail kept 0.92 and sensors at 6. Holes 25-29% of the body's outline against 0-8% off on seeds 1 to 3. Recommended for the Sprawler, changed; Direction places it ([Sprawler threads](../../Reports/lab-threads-2026-11-24/README.md)).
- The Grudge's scar tissue (lab, 11-24, its branch only): a hit on a walker standing on its scar shrugs off with chance up to 0.5 to 0.75, full at a scar of 2,000, so where it was hurt it grows back tough; on planet 7 seed 1 a holding squad killed 2,600 against 8,837 by 0:58 and the run bot kept 21 or 4 soldiers at 2:00 against 30. Recommended as the Grudge's thing; Direction places it ([scar tissue](../../Reports/lab-scar-tissue-2026-11-24/README.md)).
- The Sleeper grows in its sleep (lab, 11-24, its branch only): a sleeping walker buds a sleeping child with chance P a step, off the crystals' scent and under the ceiling, so farming before it wakes is no longer free; on planet 6 seed 1 the sleeping slime at 2:00 was 162,813 walkers at P 0.003 against 115,022 off, and 16 soldiers woken into it then were wiped within a minute while off's stood. Recommended as the Sleeper's thing; Direction places it ([it grows in its sleep](../../Reports/lab-sleeper-2026-11-24/README.md)).
- A pairing for planet 8 (lab, 11-24, engine keys only): the Swarmer with the Sleeper's life cycle sleeps until shot, charges, and sleeps it off in waves; a pair is both tiers' settings, and the trait keys that fight are listed in the report (sleepers don't move in surges or the last stand; one alarm field, one life-cycle timer, one scar, one walk). Recommended as planet 8's thing; Direction places it ([the Swarmer that naps](../../Reports/lab-pairing-2026-11-24/README.md)).
- The Trunk's veins (lab, 11-19): its body is packed a walker a cell, so rules on walker motion (deposit by flow, aligning to it) drew no veins inside it and only made the cords bolder; the flow that moves inside a packed body is the shared food, not yet drawn ([Trunk veins](../../Reports/lab-veins-2026-11-19/README.md)).

## Sources

- [larder-world-roadmap-2026-11-11.md](../../Reports/larder-world-roadmap-2026-11-11.md), *Mould progression*.
- [mould-tiers-2026-11-11.md](../../Reports/mould-tiers-2026-11-11.md): Sporer, Sleeper, Swarmer; *In the Larder world*.
- [mould-tiers-review-2026-11-11.md](../../Reports/mould-tiers-review-2026-11-11.md): two independent reviews and the response.
- [first-reach-2026-11-11.md](../../Reports/first-reach-2026-11-11.md): droplet moulds and mixes on food piles.
- [species-sets-2026-11-11.md](../../Reports/species-sets-2026-11-11.md): per-species settings, two species on one dish.
- [species-gpu-2026-11-11.md](../../Reports/species-gpu-2026-11-11.md): the species rules on the GPU engine.
- [life-gpu-2026-11-11.md](../../Reports/life-gpu-2026-11-11.md): the tiers' life cycle on the GPU engine.
- [Trunk veins](../../Reports/lab-veins-2026-11-19/README.md): walker-motion rules for veins inside the Trunk, and the food flow.
- [Grudge scar tissue](../../Reports/lab-scar-tissue-2026-11-24/README.md): scarred slime that shrugs off hits, planet 7.
- [Sleeper dreams](../../Reports/lab-sleeper-2026-11-24/README.md): a sleeping slime that grows while you farm, planet 6.
- [Napping Swarmer](../../Reports/lab-pairing-2026-11-24/README.md): a lap-two pairing for planet 8 and the trait keys that fight.
- [Sprawler threads](../../Reports/lab-threads-2026-11-24/README.md): lanes and a shuttle open the Sprawler's packed body into spokes, at the same walker count.
