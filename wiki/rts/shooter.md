---
title: Larder, the mould as the enemy (shooter brainstorm)
kind: design
status: open
updated: 2026-11-11
sources: [Reports/mould-shooter-2026-11-11.md, Reports/rts-research-2026-11-10.md, research/sources/creeper-world-2025.md]
code: [Params::pieces, Params::wound, Params::light_weight, Dish::step_cost]
---

# Larder, the mould as the enemy (shooter brainstorm)

## Now

Brainstorm: [`mould-shooter-2026-11-11.md`](../../Reports/mould-shooter-2026-11-11.md).
A squad shoots the mould; both want the same food; the player's larder is a
pile the mould steals from. The owner chose a squad, solo, tests first, and
ruled that a held ring of fire should stop all mould. **It does now:** a snip
fired again on an open wound used to do nothing, so a held ring healed after
4 s; fixed, a ring held from the start keeps every walker off the larder (0
stolen, 16/16). A ring raised once the mould is near saves 0.6%: the mould
bites at about 4 s, and theft is capped per pile (`Params::table`) once any
walker eats. Firing on the eaters saves 98.7%. Three independent reviews
(code, write-up, design) are in the report's folder; the design review finds
the ring now dominant and proposes pricing it per cell per second, slowing
fronts and giving the mould teeth from the model.

## Numbers

| Measure | Result | Seeds | Paired | Source |
|---|---|---|---|---|
| Larder theft, ring held from 0 s (fixed snip), against idle | 0 against 12,301 | 16 | 16-0, p 0.00003 | [report, *Correction*](../../Reports/mould-shooter-2026-11-11.md) |
| Larder theft, ring raised when mould is near (fixed snip), against idle | 12,225 against 12,301 | 16 | 15-1, p 0.0005 | [report, *Correction*](../../Reports/mould-shooter-2026-11-11.md) |
| Larder theft, snips through the larder (fixed snip), against idle | 154 against 12,301 | 16 | 16-0, p 0.00003 | [report, *Correction*](../../Reports/mould-shooter-2026-11-11.md) |
| Trail-only gun, any hand against idle | 12,269-12,275 | 16 | 7-9 of 16, p >= 0.80 | [report, *Results*](../../Reports/mould-shooter-2026-11-11.md) |
| First bite of the larder, smell 3 against 0.3 | 4.1 s against 4.6 s | 16 | 16-0, p 0.00003 | [report, *Results*](../../Reports/mould-shooter-2026-11-11.md) |
| Loops at 45 s, piles full against a quarter | 10 against 1 | 16 | 16-0, p 0.00003 | [report, *Results*](../../Reports/mould-shooter-2026-11-11.md) |

## Owner's word

- 2026-11-11 01:50Z: "Shooting should be able to directly cut through a branch after a free seconds and holding a ring of fire should stop all mold from getting to the food".
- 2026-11-11 00:30Z: "Squad, start just you against the mold, tests first" (and asked why a browser build, since the browser cannot run the GPU-scale game).
- 2026-11-11 00:18Z: "brainstorm a demo where the slime mold is the enemy. The player is a soldier or unit of soldiers who can shoot the mold to destroy parts of it. The player needs food to upgrade, build defenses, etc. and the slime mold wants food to expand/survive, etc,"

## Tried and rejected

- A ring of cuts raised round a pile once the mould is near (a ring held from the start works): [`dead-ends.md`](../../Reports/dead-ends.md), Larder entry.

## Open

- The design review's test: a ring priced per cell per second, a larder that runs dry and slowed fronts, scripted plans against random; pass if "ring forever" is not the best plan.
- Re-run smell on a dish the mould takes 30 s or more to cross.
- Versus (two squads, one mould) is set aside for now (owner, 00:30Z: solo first).

## Sources

- [mould-shooter-2026-11-11.md](../../Reports/mould-shooter-2026-11-11.md): the whole brainstorm.
- [rts-research-2026-11-10.md](../../Reports/rts-research-2026-11-10.md) and the [Creeper World card](../../research/sources/creeper-world-2025.md): the nearest published game with a simulated enemy.
