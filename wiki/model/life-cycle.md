---
title: The life cycle and the mould's answer to attack (Sporer, Sleeper, Swarmer)
kind: mechanism
status: open
updated: 2026-11-12T06:26Z
sources: [Reports/larder-world-roadmap-2026-11-11.md, Reports/life-gpu-2026-11-11.md, Reports/mould-tiers-2026-11-11.md, Reports/mould-tiers-review-2026-11-11.md, research/sources/jones-adamatzky-2012.md, research/sources/jones-2015-survey.md]
code: [BigParams::spore_rate, BigParams::spore_scent, BigParams::spore_burst, BigParams::spore_alarm, BigParams::spore_steps, BigParams::spore_speed, BigParams::spore_reach, BigParams::spore_deposit, BigParams::spore_land, BigParams::spore_rest, BigParams::sleep_rate, BigParams::sleep_deposit, BigParams::wake_scent, BigParams::wake_alarm, BigParams::wake_steps, BigParams::sleep_rest, BigParams::sleep_death, BigParams::alarm_keep, BigParams::alarm_weight, BigParams::alarm_relay, BigDish::strike, BigDish::life, BigDish::life_t, BigDish::alarm, BigDish::life_counts, LIFE_SPORE, LIFE_DORMANT, LIFE_DEAD, Tier, SHOT_ALARM, Engine::strike, Engine::life_of, Engine::alarm_of, shout]
---

# The life cycle and the mould's answer to attack (Sporer, Sleeper, Swarmer)

## Now

On the CPU big dish (`src/mould/big.rs`) each walker can be ordinary, a
spore, dormant or dead. The player's strike (`BigDish::strike`) and, with
the alarm field on, the squad's shots (`squad::shots`, `SHOT_ALARM`) lay
an alarm. All off by default. `mould::tiers::Tier` names three tiers' keys
for the game. The GPU engine runs every rule under the same keys
(`Tier::settings`), the shots' alarm and `sleep_death` too, and matches
the CPU exactly: every walker, state and timer on 18 dishes over four
ticks, the three tiers among them, with every field within 1e-6
([life cycle on the GPU](../../Reports/life-gpu-2026-11-11.md), *The twin
test*).

- **Swarmer** (`alarm_weight` +3): floods where it is hit. Holds in the
  Larder world (8 of 8). A modifier any mould could carry.
- **Sleeper** (`sleep_rate`): a dormant crust round its home crystal that
  wakes in a wave and is then deaf for 5 s. A bait before a strike holds in
  the world (8 of 8). It barely spreads: a nest, not a map threat. A dormant
  walker dies at `sleep_death`, so a crust never fed fades (free sleep froze
  a world whose crystals ran dry).
- **Sporer** (`spore_rate` 1e-5): fed walkers fly to new crystals. It won
  8 of 8 on the test dish, but in the world its edge is not significant.

The counters (bait, kill zone, starve) are proposals, not yet tested
against random play. Fleeing the alarm did not split the body. It holds a
hole open under a squad's fire, but as the Larder race world's default it
let random hold the undefended core and win (2026-11-12,
[Larder](../world/larder.md)), so it is an opt-in. The alarm
costs +31% a step on one core. Tables: [the report](../../Reports/mould-tiers-2026-11-11.md),
*Summary* and *In the Larder world*.

## Owner's word

- 2026-11-11 20:12Z (this work's thread): "Also make sure you work is being integrated into the game".
- 2026-11-11 15:07Z (this work's thread): "You should also consider how the mold reacts to the player, how it reacts to being attacked, etc."
- 2026-11-11 14:59Z (cited in this work's brief): "The mold shouldn't be able to take over a whole map in 10 seconds."
- 2026-11-11 (cited in the brief): the ladder "can also should expand as the mold research session finds more behaviors and new types of mold."

## Tried and rejected

- Sleep paying a share of the upkeep, to stop the frozen world: the crust starved in seconds and the bait stopped working ([`dead-ends.md`](../../Reports/dead-ends.md), `sleep_upkeep`).
- A strike's alarm as a push the mould flees, to split it: no measurable split ([`dead-ends.md`](../../Reports/dead-ends.md), `alarm_weight` below 0).
- Spores homing from launch land back on their own pile; replaced by the two-half flight ([`dead-ends.md`](../../Reports/dead-ends.md), `spore_reach`).

## Open

- The counters as plans against random and idle hands, with shot-sized and sustained fire, in the world (W2b, with the player AI); [the reviews](../../Reports/mould-tiers-review-2026-11-11.md).
- Life-cycle fields per species, for mixes; species conversion still takes dormant walkers and spores.
- The GPU port's cost on the GTX 1070 (the software GPU's is in [the GPU report](../../Reports/life-gpu-2026-11-11.md), *Cost*), and the tiers on the panel.
- With [growth](growth.md) on, spores and the dormant pay no upkeep (the dormant die at `sleep_death` instead) and the struck free their slots.
- A tier that splits or migrates as a body.
- A Sporer that matters in the world: fruiting set off by a crystal running low, with a visible delay, and spores that carry energy (review 1).
- Mould against economy (crystals, upkeep) belongs to the creature thread.

## Sources

- [mould-tiers report](../../Reports/mould-tiers-2026-11-11.md), all sections.
- [life cycle on the GPU](../../Reports/life-gpu-2026-11-11.md): the port to `gpu::Engine`, its twin tests, planted bugs and cost.
- [The two reviews](../../Reports/mould-tiers-review-2026-11-11.md) (2026-11-11 evening).
- [Jones and Adamatzky 2012](../../research/sources/jones-adamatzky-2012.md): blocked walkers keep heading, light as a push.
- [Jones 2015 survey](../../research/sources/jones-2015-survey.md): the sclerotium and fruiting.
- Page: listed in [`Reports/pages.md`](../../Reports/pages.md).
