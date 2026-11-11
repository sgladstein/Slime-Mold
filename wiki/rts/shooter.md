---
title: Larder, the mould as the enemy (shooter brainstorm)
kind: design
status: open
updated: 2026-11-11
sources: [Reports/mould-shooter-2026-11-11.md, Reports/larder-build-plan-2026-11-11.md, Reports/rts-research-2026-11-10.md, research/sources/creeper-world-2025.md]
code: [Params::pieces, Params::wound, Params::light_weight, Dish::step_cost]
---

# Larder, the mould as the enemy (shooter brainstorm)

## Now

Brainstorm: [`mould-shooter-2026-11-11.md`](../../Reports/mould-shooter-2026-11-11.md).
A squad shoots the mould; both want the same food; the player's larder is a
pile the mould steals from. The owner chose a squad, solo, tests first, and
ruled that a held ring of fire should stop all mould: after a snip fix it
does (0 stolen, 16/16). **Priced** (every shot costs food, the larder runs
dry, fronts slowed to arrive at about 32 s), ringing forever is never the
best plan: once the larder is dry the ring guards nothing. But one plan,
ring the larder while it has food then mow the eaters off the other piles,
tops all 16 seeds at shot prices 10 and 40, and at 120 no shooting pays. It
is a ladder with a timing choice, not counter-strategies, because the mould
does not adapt to the squad. Three independent reviews are in the report's
folder.

## Numbers

| Measure | Result | Seeds | Paired | Source |
|---|---|---|---|---|
| Larder theft, ring held from 0 s (fixed snip), against idle | 0 against 12,301 | 16 | 16-0, p 0.00003 | [report, *Correction*](../../Reports/mould-shooter-2026-11-11.md) |
| Priced ring, bank at the bell: ring then mow / ring all round / idle, shot price 40 | 23,420 / 7,540 / 10,600 | 16 | ring-then-mow best on 16/16; ring below idle 16/16 | [report, *The priced ring*](../../Reports/mould-shooter-2026-11-11.md) |
| Larder theft, ring raised when mould is near (fixed snip), against idle | 12,225 against 12,301 | 16 | 15-1, p 0.0005 | [report, *Correction*](../../Reports/mould-shooter-2026-11-11.md) |
| Larder theft, snips through the larder (fixed snip), against idle | 154 against 12,301 | 16 | 16-0, p 0.00003 | [report, *Correction*](../../Reports/mould-shooter-2026-11-11.md) |
| Trail-only gun, any hand against idle | 12,269-12,275 | 16 | 7-9 of 16, p >= 0.80 | [report, *Results*](../../Reports/mould-shooter-2026-11-11.md) |
| First bite of the larder, smell 3 against 0.3 | 4.1 s against 4.6 s | 16 | 16-0, p 0.00003 | [report, *Results*](../../Reports/mould-shooter-2026-11-11.md) |
| Loops at 45 s, piles full against a quarter | 10 against 1 | 16 | 16-0, p 0.00003 | [report, *Results*](../../Reports/mould-shooter-2026-11-11.md) |

## Owner's word

- 2026-11-11 04:04Z (card): "Both": food stops arriving late in the round, and a 15-game-minute cap ends any stalemate on share of the map held.
- 2026-11-11 03:38Z: "Start with 4 and can grow to 40. They should definitely die and be attacked by the slime. The round ends when the soldiers or slime are all dead?"
- 2026-11-11 03:14Z: "Rts-style. When you finish your current task, let's develop a full detailed plan of how to build the demo".
- 2026-11-11 03:03Z: "Yes." (to the priced-ring test).
- 2026-11-11 01:50Z: "Shooting should be able to directly cut through a branch after a free seconds and holding a ring of fire should stop all mold from getting to the food".
- 2026-11-11 00:30Z: "Squad, start just you against the mold, tests first" (and asked why a browser build, since the browser cannot run the GPU-scale game).
- 2026-11-11 00:18Z: "brainstorm a demo where the slime mold is the enemy. The player is a soldier or unit of soldiers who can shoot the mold to destroy parts of it. The player needs food to upgrade, build defenses, etc. and the slime mold wants food to expand/survive, etc,"

## Tried and rejected

- A ring of cuts raised round a pile once the mould is near (a ring held from the start works): [`dead-ends.md`](../../Reports/dead-ends.md), Larder entry.

## Open

- Make the mould answer the squad so plans can counter each other: smell growing with the bank, starving mould fighting harder at a ring, drops that pull fronts away, layouts where the best piles differ.
- The build plan, reviewed by three independent agents and revised ([larder-build-plan-2026-11-11.md](../../Reports/larder-build-plan-2026-11-11.md); Claude Doc "Larder: build plan for the RTS demo", https://claude.ai/artifact/5ricncRubPz7WTBGJZFnHs): six milestones, M1 soldiers in the harness, M2 the mould answers back (gate: a crossover under visible conditions plus a reader plan that beats every fixed plan), M3 first playable window, M4 economy, M5 polish, M6 GPU port after S4. Not built.
- Re-run smell on a dish the mould takes 30 s or more to cross.
- Versus (two squads, one mould) is set aside for now (owner, 00:30Z: solo first).

## Sources

- [mould-shooter-2026-11-11.md](../../Reports/mould-shooter-2026-11-11.md): the whole brainstorm.
- [rts-research-2026-11-10.md](../../Reports/rts-research-2026-11-10.md) and the [Creeper World card](../../research/sources/creeper-world-2025.md): the nearest published game with a simulated enemy.
