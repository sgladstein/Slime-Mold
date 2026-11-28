---
title: Scent and trail fields
kind: mechanism
status: open
updated: 2026-11-28
sources:
  - Reports/lab-hunt-2026-11-26/README.md
  - Reports/lab-walker-sort-2026-11-24/README.md
  - Reports/scent-review-2026-11-28/README.md
code:
  - BigParams::clay_keep
  - BigParams::fed_scent
  - BigParams::food_deposit
  - BigParams::scent_keep
  - BigParams::scent_offset
  - HUNT_PULL
  - LURE_PULL
---

# Scent and trail fields

## Now

The slime steers by GPU fields that blur over 3x3 cells and fade by a keep
each engine step (trail 0.8 to 0.95 by tier, food smell `scent_keep`
0.97, alarm 0.98 on alarm tiers), plus CPU walking-distance maps (far
smell, lure, hunt, wave, raid, units) that walkers lean down. The food
smell's half-life is 0.76 s and it spreads 3.3 fine cells, but walkers
read it 36 cells ahead and still follow it at 1e-12, because they compare
readings rather than measure them. The squad is the loudest smell (a peak
of 4,136 on planet 2 at 1:00 against 85 to 96 at a touched crystal), and
only the trail and the lure rings are on screen. One bug: on clay
(`clay_keep` 1.05) a trail keep at or above 0.952 grows without limit
([review](../../Reports/scent-review-2026-11-28/README.md), *Findings*).

The far smell is already in the food smell's exponent: −ln(smell) ranks
blocks like the CPU far smell's walking distance (Spearman 0.87 to 0.93
on planet 1). The trail and scent passes are 37% of the GPU step on planet
8 on the 1070 ([walker sort](../../Reports/lab-walker-sort-2026-11-24/README.md),
*Results*). The hunt's map is a disc round the squad, and shortest paths
bring the body in on one front ([hunt](../../Reports/lab-hunt-2026-11-26/README.md)).

## Owner's word

- 2026-11-28 07:19Z: "I want a thread to deeply examine the
  physics/behavior/simulation of our scents and trails. Are they ideally
  designed? How could they be better utilized or improved in the game. Feel
  free to think outside the box."

## Tried and rejected

- A stronger soldier smell to help the slime catch kiting squads: did
  nothing ([`dead-ends.md`](../../Reports/dead-ends.md), *Existing keys as
  a chase*, 11-26; the smell fades in under a second).

## Open

- Ten ranked ideas in the [review](../../Reports/scent-review-2026-11-28/README.md),
  *Ideas, ranked*: cap clay's keep; fold-then-blur for speed; flanking
  paths (recommended first prototype); drawing what the slime follows;
  smell channels and palates; a free far smell from log smell; smell on
  the wind; lost crystals that call the slime back; walking-distance maps
  on the GPU; ideas from the model. Which to prototype first is Scott's.

## Sources

- [scent-review-2026-11-28](../../Reports/scent-review-2026-11-28/README.md), *The fields*, *Findings*, *Ideas, ranked*.
- [lab-hunt-2026-11-26](../../Reports/lab-hunt-2026-11-26/README.md).
- [lab-walker-sort-2026-11-24](../../Reports/lab-walker-sort-2026-11-24/README.md), *Results*.
- Page archive: [`Reports/scent-review-2026-11-28/`](../../Reports/scent-review-2026-11-28/index.html).
