---
title: Larder, the mould as the enemy (shooter brainstorm)
kind: design
status: open
updated: 2026-11-11
sources: [Reports/mould-shooter-2026-11-11.md, Reports/larder-m1-2026-11-11.md, Reports/larder-m2-2026-11-11.md, Reports/larder-m3-2026-11-11.md, Reports/larder-build-plan-2026-11-11.md, Reports/rts-research-2026-11-10.md, research/sources/creeper-world-2025.md]
code: [Dish::fire, Params::pieces, Params::wound, Params::light_weight, Dish::step_cost]
---

# Larder, the mould as the enemy (shooter brainstorm)

## Now

**Larder World (2026-11-11 14:06Z):** the owner moved Larder into a persistent world on the GPU big dish, with energy crystals that run out and several mould species; no more small-dish tuning. Roadmap: [`larder-world-roadmap-2026-11-11.md`](../../Reports/larder-world-roadmap-2026-11-11.md) (doc https://claude.ai/artifact/1N2NHKefAzKxUMZcr3ktjJ). Below is the small-dish history it builds on.

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

**Build, M2** ([report](../../Reports/larder-m2-2026-11-11.md)): with
spending (soldiers, bait, turrets) and three mould answers (hoard smell,
hunger, drops), the gate fails on held-out seeds: no plan crosses over
between a fed and a lean field, and one plan (bait the mould away, farm,
expand once it is small) is best on 12-13 of 16. Hoard smell makes saving
cost and drops keep a lean mould alive; hunger barely acts. Next, per the
plan: one more mould answer that makes waiting lose against a lean mould.

**Build, M3** ([report](../../Reports/larder-m3-2026-11-11.md)): two such
answers (spores, a dormant crust) failed and were cut. What loosened the
ladder was the soldiers: firing from range instead of walking onto an eaten
pile, spreading shots, and a loose formation. On held-out seeds no plan is
best on 13+ of 16 any more. The window has groups, a scent view and replays;
its frame on Scott-Desktop is 7.2 ms mean with 40 soldiers. Next: the
owner's playtest.

## Numbers

| Measure | Result | Seeds | Paired | Source |
|---|---|---|---|---|
| M3, best plan per seed, held-out: reader / striker / farmer, fed and lean | 8/5/3 and 10/4/2 (M2: reader 13 and 12) | 16 (17-32) | flips striker-farmer, striker-baiter, none survives Holm over 34 | [M3 report, *Census*](../../Reports/larder-m3-2026-11-11.md) |
| M3 window frame, 40 soldiers, Scott-Desktop (i5-6600K), scent view off / on | 7.2 / 8.9 ms mean, 9.6 / 11.6 ms p95 | 3 runs | - | [M3 report](../../Reports/larder-m3-2026-11-11.md) |
| M2 gate, held-out: reader against farmer (fed) / baiter (lean), larder field | 13 of 16 / 12 of 16 | 16 (17-32) | p 0.021 / 0.077, neither survives Holm over 24 | [M2 report, *The census*](../../Reports/larder-m2-2026-11-11.md) |
| M2, hoard smell off: fed reader median score | -0.300 to 0.167 | 16 (17-32) | reader over farmer 13 to 16 of 16 | [M2 report, *Each answer switched off*](../../Reports/larder-m2-2026-11-11.md) |
| M2, drops off: lean mould wiped, baiter and reader | 16 of 16 each (5 and 11 with drops) | 16 (17-32) | - | [M2 report, *Each answer switched off*](../../Reports/larder-m2-2026-11-11.md) |
| Larder theft, ring held from 0 s (fixed snip), against idle | 0 against 12,301 | 16 | 16-0, p 0.00003 | [report, *Correction*](../../Reports/mould-shooter-2026-11-11.md) |
| Priced ring, bank at the bell: ring then mow / ring all round / idle, shot price 40 | 23,420 / 7,540 / 10,600 | 16 | ring-then-mow best on 16/16; ring below idle 16/16 | [report, *The priced ring*](../../Reports/mould-shooter-2026-11-11.md) |
| Larder theft, ring raised when mould is near (fixed snip), against idle | 12,225 against 12,301 | 16 | 15-1, p 0.0005 | [report, *Correction*](../../Reports/mould-shooter-2026-11-11.md) |
| Larder theft, snips through the larder (fixed snip), against idle | 154 against 12,301 | 16 | 16-0, p 0.00003 | [report, *Correction*](../../Reports/mould-shooter-2026-11-11.md) |
| Trail-only gun, any hand against idle | 12,269-12,275 | 16 | 7-9 of 16, p >= 0.80 | [report, *Results*](../../Reports/mould-shooter-2026-11-11.md) |
| First bite of the larder, smell 3 against 0.3 | 4.1 s against 4.6 s | 16 | 16-0, p 0.00003 | [report, *Results*](../../Reports/mould-shooter-2026-11-11.md) |
| Loops at 45 s, piles full against a quarter | 10 against 1 | 16 | 16-0, p 0.00003 | [report, *Results*](../../Reports/mould-shooter-2026-11-11.md) |

## Owner's word

- 2026-11-11 14:09Z: also explore a race start, "both the player and the mold starting in one small area and then we're both trying to expand at the same time".
- 2026-11-11 14:07Z: a player AI thread for automated testing; later "two soldier teams against multiple molds so in the future the AI could also play soldiers".
- 2026-11-11 14:06Z: "A longer persistent world. Maybe a campaign. Crystal should run out which forces you to expand. Several species sounds interesting. Put everything into the world version."
- 2026-11-11 14:01Z: "You should also be moving your game into the big dish ... should become more of a world, and the food should probably become more something like energy crystals".
- 2026-11-11 04:04Z (card): "Both": food stops arriving late in the round, and a 15-game-minute cap ends any stalemate on share of the map held.
- 2026-11-11 03:38Z: "Start with 4 and can grow to 40. They should definitely die and be attacked by the slime. The round ends when the soldiers or slime are all dead?"
- 2026-11-11 03:14Z: "Rts-style. When you finish your current task, let's develop a full detailed plan of how to build the demo".
- 2026-11-11 03:03Z: "Yes." (to the priced-ring test).
- 2026-11-11 01:50Z: "Shooting should be able to directly cut through a branch after a free seconds and holding a ring of fire should stop all mold from getting to the food".
- 2026-11-11 00:30Z: "Squad, start just you against the mold, tests first" (and asked why a browser build, since the browser cannot run the GPU-scale game).
- 2026-11-11 00:18Z: "brainstorm a demo where the slime mold is the enemy. The player is a soldier or unit of soldiers who can shoot the mold to destroy parts of it. The player needs food to upgrade, build defenses, etc. and the slime mold wants food to expand/survive, etc,"

## Tried and rejected

- A ring of cuts raised round a pile once the mould is near (a ring held from the start works): [`dead-ends.md`](../../Reports/dead-ends.md), Larder entry.
- Spores and a dormant crust as the mould's fourth answer ([`dead-ends.md`](../../Reports/dead-ends.md), two Larder entries).
- The reader buying soldiers toward an army once the mould is small: the extra soldiers feed a starving mould ([`dead-ends.md`](../../Reports/dead-ends.md), larderm2 entry).

## Open

- The owner's playtest of the M3 window (`cargo run --release --bin larder -- 17`); then M4 upgrades and the M2 gate re-run. Hunger barely acts and needs a lower bar or should go.

- Make the mould answer the squad so plans can counter each other: smell growing with the bank, starving mould fighting harder at a ring, drops that pull fronts away, layouts where the best piles differ.
- The build plan, reviewed by three independent agents and revised ([larder-build-plan-2026-11-11.md](../../Reports/larder-build-plan-2026-11-11.md); Claude Doc "Larder: build plan for the RTS demo", https://claude.ai/artifact/5ricncRubPz7WTBGJZFnHs): six milestones, M1 soldiers in the harness, M2 the mould answers back (gate: a crossover under visible conditions plus a reader plan that beats every fixed plan), M3 first playable window, M4 economy, M5 polish, M6 GPU port after S4. **M1 built** ([report](../../Reports/larder-m1-2026-11-11.md)): fortress and farmer beat idle and random 16/16 held-out; four soldiers attacking (hunter) die in ~45 game s; no mould wipe; window `cargo run --release --bin larder`.
- Re-run smell on a dish the mould takes 30 s or more to cross.
- Versus (two squads, one mould) is set aside for now (owner, 00:30Z: solo first).

## Sources

- [mould-shooter-2026-11-11.md](../../Reports/mould-shooter-2026-11-11.md): the whole brainstorm.
- [rts-research-2026-11-10.md](../../Reports/rts-research-2026-11-10.md) and the [Creeper World card](../../research/sources/creeper-world-2025.md): the nearest published game with a simulated enemy.
