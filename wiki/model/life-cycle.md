---
title: The life cycle and the mould's answer to attack (Sporer, Sleeper, Swarmer)
kind: mechanism
status: open
updated: 2026-11-11
sources: [Reports/mould-tiers-2026-11-11.md, research/sources/jones-adamatzky-2012.md, research/sources/jones-2015-survey.md]
code: [BigParams::spore_rate, BigParams::spore_scent, BigParams::spore_burst, BigParams::spore_alarm, BigParams::spore_steps, BigParams::spore_speed, BigParams::spore_reach, BigParams::spore_deposit, BigParams::spore_land, BigParams::spore_rest, BigParams::sleep_rate, BigParams::sleep_deposit, BigParams::wake_scent, BigParams::wake_alarm, BigParams::wake_steps, BigParams::sleep_rest, BigParams::alarm_keep, BigParams::alarm_weight, BigParams::alarm_relay, BigDish::strike, BigDish::life, BigDish::life_t, BigDish::alarm, BigDish::life_counts, LIFE_SPORE, LIFE_DORMANT, LIFE_DEAD]
---

# The life cycle and the mould's answer to attack (Sporer, Sleeper, Swarmer)

## Now

On the CPU big dish (`src/mould/big.rs`) each walker can be ordinary, a
spore, dormant or dead, and the player's strike (`BigDish::strike`) lays an
alarm field. All off by default. Three Larder ladder tiers, each beating its
control on 8 of 8 paired seeds (20,000 walkers, 7 piles), each with a
counter:

- **Sporer** (`spore_rate`): fed walkers fly to new piles; struck, it
  scatters (`spore_burst`). Starve it. At 1e-4 it takes all 7 piles by 10 s
  on 6 of 8 seeds, too fast for the owner's rule; 1e-5 is gentler.
- **Sleeper** (`sleep_rate`): a dormant crust that wakes in a wave and
  rushes a wound, then is deaf for 5 s (`sleep_rest`). Bait it, then strike.
- **Swarmer** (`alarm_weight` +3): walks into fire. Use a kill zone.

Fleeing (`alarm_weight` below 0) did not split the body; a splitting or
migrating tier is open. The alarm field costs +31% a step on one core.
Tables: [the report](../../Reports/mould-tiers-2026-11-11.md), *Summary*
and *Robustness*.

## Owner's word

- 2026-11-11 15:07Z (this work's thread): "You should also consider how the mold reacts to the player, how it reacts to being attacked, etc."
- 2026-11-11 14:59Z (cited in this work's brief): "The mold shouldn't be able to take over a whole map in 10 seconds."
- 2026-11-11 (cited in the brief): the ladder "can also should expand as the mold research session finds more behaviors and new types of mold."

## Tried and rejected

- A strike's alarm as a push the mould flees, to split it: no measurable split ([`dead-ends.md`](../../Reports/dead-ends.md), `alarm_weight` below 0).
- Spores homing from launch land back on their own pile; replaced by the two-half flight ([`dead-ends.md`](../../Reports/dead-ends.md), `spore_reach`).

## Open

- The GPU port: every rule's doc comment in `BigParams` is the spec; the GPU thread owns births and upkeep.
- A tier that splits or migrates as a body.
- The Sporer's rate for play, and whether spores should cost food (today they are free).
- Mould against economy (crystals, upkeep) belongs to the creature thread.

## Sources

- [mould-tiers report](../../Reports/mould-tiers-2026-11-11.md), all sections.
- [Jones and Adamatzky 2012](../../research/sources/jones-adamatzky-2012.md): blocked walkers keep heading, light as a push.
- [Jones 2015 survey](../../research/sources/jones-2015-survey.md): the sclerotium and fruiting.
- Page: listed in [`Reports/pages.md`](../../Reports/pages.md).
