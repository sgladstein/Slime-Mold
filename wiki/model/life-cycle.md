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

The CPU big dish (`src/mould/big.rs`) gives each walker a life state: an
ordinary walker, a spore in flight, dormant, or dead. All of it is off by
default and the default hash is unchanged. Three new rungs for the Larder
World ladder come out of it, each with a body, an answer to the player's
strike (`BigDish::strike`: kill a disc, clear its trail, lay an alarm), and
a counter the player can learn. On a 20,000-walker body with 7 piles, 8
paired seeds each:

- **Sporer**: fed walkers fruit (`spore_rate`); a spore flies straight,
  then homes on scent and lands on a new pile. It holds 6-7 of 7 piles at
  20 s against 3-5 (8 of 8). Struck, it sprays spores (`spore_burst`) and
  holds 7 of 7 on every seed against 3-5. **Counter: starve it, don't hit
  it.** At `spore_rate` 1e-4 it is too fast for the owner's pace rule (7
  of 7 piles by 10 s on 6 of 8 seeds); at 1e-5 it holds 4-6 piles at 10 s
  and 6-7 by 60 s (seeds 1-3), so the rate is the dial.
- **Sleeper**: walkers away from food fall dormant into a crust
  (`sleep_rate`). A strike's alarm wakes the crust in a wave
  (`alarm_relay`), and the woken mould rushes the wound
  (`alarm_weight` +3). After a wake the crust is deaf to alarm for 5 s
  (`sleep_rest`). **Counter: bait.** A harmless shock at the far edge
  4 s before the real strike leaves 1858-2216 walkers at the strike
  against 2360-2971 (fewer on 8 of 8).
- **Swarmer**: `alarm_weight` +3 alone. It feeds itself into the fire: 11
  strikes in 5 s kill 1144-4158 walkers against 614-3591 (more on 8 of 8).
  **Counter: a kill zone.**

Over two minutes the Sporer keeps its piles and the Sleeper stays a
crust, and no tier rides the dish wall (seeds 1-3). A mould that flees attack (**Skittish**, `alarm_weight` below 0, after
Jones and Adamatzky's light) did not break up or move off measurably; it is
an option, not a tier (dead end). The splitting and migrating tier is still
open. Cost: the alarm field is one more full-grid pass, +31% a step on one
core at 100,000 walkers; spores +6-16%.

## Numbers

| Measure | Result | Seeds | Paired | Source |
|---|---|---|---|---|
| Sporer: piles held at 20 s (20+ grounded walkers by a pile), `spore_rate` 1e-4 vs none | 6-7 (median 7) vs 3-5 (median 4) | 8 | higher 8-0, p 0.0078, survives Bonferroni x5 | [mould-tiers, *Sporer*](../../Reports/mould-tiers-2026-11-11.md) |
| Struck Sporer: piles held at 20 s after a strike at 10 s, `spore_burst` 0.05 vs none | 7 on all 8 vs 3-5 | 8 | higher 8-0, p 0.0078, survives | same, *Hitting a Sporer* |
| Sleeper bait: walkers near the strike 2 s after it, bait 4 s before vs none | 1858-2216 (median 2030) vs 2360-2971 (median 2736) | 8 | fewer 8-0, p 0.0078, survives | same, *Sleeper* |
| Swarmer: walkers killed by 11 strikes, `alarm_weight` +3 vs 0 | 1144-4158 (median 1393) vs 614-3591 (median 816) | 8 | more 8-0, p 0.0078, survives | same, *Swarmer* |
| Skittish: tube bodies at 20 s under the same fire, `alarm_weight` -10 vs 0 | 1-5 (median 2) vs 1-3 (median 1.5) | 8 | 4-1, 3 ties, p 0.375, fails | same, *Skittish* |
| Sporer pace: piles held at 10 / 60 s, `spore_rate` 1e-5 vs none | 4-6 / 6-7 vs 3-5 / 3-5; at 3e-6, 3-6 throughout | 3 | no | same, *Robustness* |
| Two minutes with food: Sporer piles held at 120 s; Sleeper dormant at 120 s | 7 of 7 on 3 of 3 vs 4-5; 15,498-18,782 of 20,000 | 3 | no | same |
| Wall rail: share of walkers within 10 dish cells of the wall, 20 and 60 s | 0-1.8% on every tier, plain 0-0.8% (spread evenly: 18.8%) | 3 | no | same |
| Frame cost, 100k walkers spread, one core, ms a step | off 25.2-25.9; alarm 33.0-34.6; spores 27.7-29.2 | 1, 2 runs | no | same, *Cost* |

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
