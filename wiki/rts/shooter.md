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
pile the mould steals from. The owner chose a squad, solo, tests first. **The
three cheap tests have run** (`examples/larderprobe.rs`, today's dish): a held
ring of fire round the larder saves 0.5%, firing on the mould eating it saves
90%, because a pile feeds the piece on it; cuts only starve mould with no food.
A trail-only gun does nothing. Smell and lean-against-fat are real (16/16) but
tiny here, because the mould crosses today's dish in about 4 s. So the shot is
a kill on eaters, and the rest of the design needs a map where fronts travel.

## Numbers

| Measure | Result | Seeds | Paired | Source |
|---|---|---|---|---|
| Larder theft, ring of snips on the thickest tube in, against idle | 12,213 against 12,273 | 16 | 16-0, p 0.00003 | [report, *Results*](../../Reports/mould-shooter-2026-11-11.md) |
| Larder theft, snips through the larder, against idle | 1,220 against 12,273 | 16 | 16-0, p 0.00003 | [report, *Results*](../../Reports/mould-shooter-2026-11-11.md) |
| Trail-only gun, any hand against idle | 12,269-12,275 | 16 | 7-9 of 16, p >= 0.80 | [report, *Results*](../../Reports/mould-shooter-2026-11-11.md) |
| First bite of the larder, smell 3 against 0.3 | 4.1 s against 4.6 s | 16 | 16-0, p 0.00003 | [report, *Results*](../../Reports/mould-shooter-2026-11-11.md) |
| Loops at 45 s, piles full against a quarter | 10 against 1 | 16 | 16-0, p 0.00003 | [report, *Results*](../../Reports/mould-shooter-2026-11-11.md) |

## Owner's word

- 2026-11-11 00:30Z: "Squad, start just you against the mold, tests first" (and asked why a browser build, since the browser cannot run the GPU-scale game).
- 2026-11-11 00:18Z: "brainstorm a demo where the slime mold is the enemy. The player is a soldier or unit of soldiers who can shoot the mold to destroy parts of it. The player needs food to upgrade, build defenses, etc. and the slime mold wants food to expand/survive, etc,"

## Tried and rejected

- A held ring of cuts round a pile to keep the mould off it: [`dead-ends.md`](../../Reports/dead-ends.md), Larder entry.

## Open

- Re-run tests 1 and 2 on a dish the mould takes 30 s or more to cross (slowed walkers on the CPU dish first; the GPU dish has no economy yet).
- Versus (two squads, one mould) is set aside for now (owner, 00:30Z: solo first).

## Sources

- [mould-shooter-2026-11-11.md](../../Reports/mould-shooter-2026-11-11.md): the whole brainstorm.
- [rts-research-2026-11-10.md](../../Reports/rts-research-2026-11-10.md) and the [Creeper World card](../../research/sources/creeper-world-2025.md): the nearest published game with a simulated enemy.
