---
title: Larder, the mould as the enemy (Larder World)
kind: design
status: open
updated: 2026-11-11T20:15Z
sources: [Reports/larder-world-roadmap-2026-11-11.md, Reports/larder-maps-2026-11-11.md, Reports/mould-shooter-2026-11-11.md, Reports/larder-m1-2026-11-11.md, Reports/larder-m2-2026-11-11.md, Reports/larder-m3-2026-11-11.md, Reports/larder-build-plan-2026-11-11.md, Reports/rts-research-2026-11-10.md, research/sources/creeper-world-2025.md]
code: [Dish::fire, Params::pieces, Params::wound, Params::light_weight, Dish::step_cost, Engine::set_squad, squad::frame]
---

# Larder, the mould as the enemy (Larder World)

## Now

**Larder World is the line all Larder work now goes to** (owner, 2026-11-11
14:06Z: "Put everything into the world version"): a squad in a persistent
world on the GPU big dish, energy crystals that run out, several mould
species. Six gated phases, W0-W5
([roadmap](../../Reports/larder-world-roadmap-2026-11-11.md); live doc
https://claude.ai/artifact/1N2NHKefAzKxUMZcr3ktjJ), replacing M4-M6 below.
**Done:** W1a, the world's fields on the CPU big dish (PR #64); W1b, the
same fields on the GPU engine; W2a, the squad's hooks (PR #74).
**Next:** W2b, M3's rules on the world, with the map loader (WM1); then the
window. Designed maps are now part of the plan (owner, 19:29Z; ROADMAP
WM1-WM5). The small-dish M3 window still waits on the owner's playtest.

## Larder World so far

**W1a** (PR #64): crystals that run dry or refill (`crystal_charge`,
`crystal_eat`, `crystal_regrow`), rock and wet terrain, a rectangular world,
and player walls (`build_wall`) that mould pressing on wears down to slow
rubble (owner, 15:24Z: walls must not seal a crystal for good). All on the
big dish, CPU, off by default. Wall health defaults to 400: a whole caged mould breaks out
in 20-30 s, while a passing tube never breaks a wall. Crystal charge flows
inside a crystal (`crystal_flow` 0.01), so the mould eats whole crystals.
With growth (scratch, no player), charge sets how long the mould lasts, not
how big it gets: charge 512 gives an idle mould about 10-12 minutes of
food on the 512 × 320 world
([report](../../Reports/larder-world-roadmap-2026-11-11.md), *Crystal
sizing with growth*).
Walls leak 0.9 of their scent, so a walled crystal still calls and a growing
mould gets through a ring in about 90 s; sealed, it walked past. Owner,
16:48Z: crystals are hexagons, and gunfire hits the first mould in its path.

**W1b**: the crystals, terrain and walls run on the GPU engine
(`Engine::build_wall`), and one GPU tick matches one CPU tick on nine world dishes, with all 17
planted bugs caught ([report, *W1b*](../../Reports/larder-world-roadmap-2026-11-11.md)).
On the GTX 1070 the world adds 0.25 ms to the frame at 1024 × 640 (1.73 to
2.11 ms). **W2**, Larder in the world, is planned in three steps: engine
hooks, M3's rules as a game layer, then the window ([report, *W2
plan*](../../Reports/larder-world-roadmap-2026-11-11.md)). **W2a** is
built (`mould::squad`, `Engine::set_squad`): shots stop at the first
cell that looks like slime (trail for 26/255) or at a wall, and strike a
disc; soldiers lay smell; each unit reads its touch and its densest block
in sight, and harvests crystal charge. One GPU frame equals the CPU's
exactly. On the 1070, 8 to 40 units add 1.0-1.5 ms a frame, mostly the
units pass running one thread a unit; W2b cuts it.

## Small-dish milestones (M1-M3)

The evidence the world builds on: what the squad's tools do and why the
plans formed a ladder.

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


## Owner's word

- 2026-11-11 14:54Z (14:55: "just my initial thought", for playtests to settle): test crystals that run out against ones that don't. Run-out crystals go with the race (an idle player should be overrun), refilling ones probably with the landing. Walls first, the Sporer and Sleeper tiers and a player-owned species are agreed. A CPU world for small tests, and the 1070 for GPU work.
- 2026-11-11 14:25Z: the mould tier ladder is agreed, and it grows as research finds new behaviours. Walls or weapons can come first.
- 2026-11-11 14:14Z: the early leader's win rate is a per-mould difficulty, not a gate. There should be mould tiers from easy to hard, and player walls, upgrades and weapons, so strategy depends on the mix.
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

- **Maps** (owner, 19:29Z: "We will want maps though. That should be part of the plan"): five paper maps, each drawn for a different winning style, and ground kinds ranked by the model term each changes ([plan](../../Reports/larder-maps-2026-11-11.md), [page](https://claude.ai/artifact/XPwkBkTAzsWq3Wg37pff7K)). The map files are written in the loader's format (PNG ground layers and item lists, agreed with the Larder thread). Height is parked (owner, 20:06Z: "I am skeptical how much fun/complexity/strategy this adds"); sand, clay and barren ground are tested first, and the maps are rebuilt on the graphics thread's terrain. No runs yet; maps wait on the map loader and the W2b harness.
- Larder World, phase by phase, per the roadmap; the open questions there (world size, how a world ends, whether two soldier teams are allies or rivals, fog of war, what carries between campaign worlds) are the owner's.
- The M3 window stays playable (`cargo run --release --bin larder -- 17`). Hunger barely acts on the small dish and needs a lower bar or should go.

- Make the mould answer the squad so plans can counter each other: smell growing with the bank, starving mould fighting harder at a ring, drops that pull fronts away, layouts where the best piles differ.
- The build plan, reviewed by three independent agents and revised ([larder-build-plan-2026-11-11.md](../../Reports/larder-build-plan-2026-11-11.md); Claude Doc "Larder: build plan for the RTS demo", https://claude.ai/artifact/5ricncRubPz7WTBGJZFnHs): six milestones, M1 soldiers in the harness, M2 the mould answers back (gate: a crossover under visible conditions plus a reader plan that beats every fixed plan), M3 first playable window, M4 economy, M5 polish, M6 GPU port after S4 (M4-M6 replaced by the world phases, 11-11 14:06Z). **M1 built** ([report](../../Reports/larder-m1-2026-11-11.md)): fortress and farmer beat idle and random 16/16 held-out; four soldiers attacking (hunter) die in ~45 game s; no mould wipe; window `cargo run --release --bin larder`.
- Re-run smell on a dish the mould takes 30 s or more to cross.
- Versus (two squads, one mould) is set aside for now (owner, 00:30Z: solo first).

## Sources

- [larder-world-roadmap-2026-11-11.md](../../Reports/larder-world-roadmap-2026-11-11.md): the world plan, phases W0-W5 and their gates.
- [larder-maps-2026-11-11.md](../../Reports/larder-maps-2026-11-11.md): maps and ground kinds (WM2, WM3), a plan.
- [mould-shooter-2026-11-11.md](../../Reports/mould-shooter-2026-11-11.md): the whole brainstorm.
- [rts-research-2026-11-10.md](../../Reports/rts-research-2026-11-10.md) and the [Creeper World card](../../research/sources/creeper-world-2025.md): the nearest published game with a simulated enemy.
