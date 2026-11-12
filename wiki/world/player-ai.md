---
title: The Larder player AI
kind: tool
status: current
updated: 2026-11-12T21:46Z
sources: [Reports/plan-matrix-2026-11-12.md, Reports/larder-world-tiers-2026-11-12.md, Reports/larder-world-ai-2026-11-12.md, Reports/larder-ai-2026-11-11.md, Reports/larder-m3-2026-11-11.md, Reports/larder-world-roadmap-2026-11-11.md]
code: [Brain, Style, Side, View, RandomHand, set_rule, Game::apply, mould_grid]
---

# The Larder player AI

## Now

The player AI plays Larder's soldier side for automated playtests. It gives only the player's commands (`Cmd` through `Game::apply`), so every AI round is a replay. It reads only what a player sees (a `View`), and it commands a `Side`, so a second soldier team later is a second brain.

**On the Larder world** (`src/mould/larder_world_ai.rs`, [report](../../Reports/larder-world-ai-2026-11-12.md), [page](https://claude.ai/artifact/BuPV981PmqpXkDhxTnadzs)) it has five styles on knobs: boom, claim, late, fort and strike. It sees the mould through `World::mould_grid` (the CPU world bins its occupancy; the GPU world sums it on the GPU since #101). It skips crystals under one frame's ask, which the dry-crystal rule sends a carrier home from. On held-out seeds 21-36 of the CPU race world, every style beats idle and random on 16 of 16. Boom, claim, late and fort beat the scripted farm hand on 15 or 16 of 16. Strike is the gamble: it wiped the mould once and lost to farm on five seeds. (W2b's score; under score v2 strike beats farm 14-2 on seeds 5-20, [roadmap report](../../Reports/larder-world-roadmap-2026-11-11.md), *Score v2*).

**Against the mould tiers, after core defence** ([report](../../Reports/plan-matrix-2026-11-12.md), [page](https://claude.ai/artifact/17cgbF9X4UdYMQANH91xVA)): on main after #126 (seeds 201-216, twelve plans including the toolkit's) the strike falls to +0.33 on the plain mould, level with claim and farm. It leads by median only on the Drifter, Sprawler and Trunk, never significantly; plague over farm on the Sporer (16-0) is the one cell that survives Holm. Random wipes the Drifter and Trunk on 7 and 5 of 16. No AI plan is stuck at all. `python3 scripts/plan_matrix.py run OUT seeds=… tiers=…` regenerates it; under the owner's testing rule (17:26Z) it runs only for a strategy question, with the runner time named first.

**Before craters** ([report](../../Reports/larder-world-tiers-2026-11-12.md), [page](https://claude.ai/artifact/T4S63gUxYWbJqJEQfviHDe)): on main before craters (seeds 53-60, eight tiers) the strike is the best plan on seven tiers, and it wipes the plain mould, the Swarmer, the Trunk and the Drifter on 5-8 of 8 seeds. The Sporer flips it: the strike takes the core but the spores have left, and claim is best. The Sporer, Grudge and Sprawler make every plan worse; the Sleeper, Drifter and Trunk are easier than plain. No AI plan is ever stuck more than 2 s a game. The matrix re-runs with `larderworld census … tier=NAME` and `Reports/larder-world-tiers-2026-11-12/matrix.py`.

The small-dish AI (`src/mould/larder_ai.rs`, [report](../../Reports/larder-ai-2026-11-11.md)) stays for the CPU field dish. Its results there are about the AI, not balance.

## Owner's word

- 2026-11-11 14:07Z: "maybe a player AI thread, for automated testing so you can keep developing I without me having to playtest everything. Also, it can be interesting to maybe have two soldier teams against multiple molds so in the future the AI could also play soldiers" ([decisions](../decisions.md)).

## Tried and rejected

- The tier census's predictions that a strike gains against the Swarmer and loses against the Sleeper (W2b's score, seeds 53-60): neither held ([report](../../Reports/larder-world-tiers-2026-11-12.md), *Run 1*).

- World strike with a mop-up after the core fell: lower on 6 seeds and higher on 1 of 21-36, with no more wipes ([report](../../Reports/larder-world-ai-2026-11-12.md), *Tried and rejected*).
- World strike sending every soldier: the bank ran dry with no farmers left (seeds 1-4); it now keeps 8 farming.
- Boom with two guards burning round home on the alarm: wiped 8 of 8 in both fields (seeds 1-8); a source comment on `Style::BOOM` records it.

## Open

- When a strategy question asks for it: the Swarmer column and the Grudge's toolkit rows, held-out seeds from 301 and the map Rim, and a buildings hand (turret ring); once the GPU census exists, once there (tech-lead review, 04:40Z).
- A counter to the core rush: the strike is best on seven of eight tiers (the tiers thread's gate: on at most two).
- New hands for the toolkit and buildings plans: turret ring first, then kill zone, turtle, plague, siege (design review, 04:35Z).
- A second team: a team on each soldier, a home per team, and `Game::apply_as(team, cmd)`.
- W1: how often the early leader wins with each opening and crystal rule, as a per-mould difficulty stat (owner, 14:14Z and 14:54Z).
- An exploiter search over `Style` knobs against a rule change.
- On a map (WM1) crystals differ in size and the larder has its own place: read `World::crystal_r(c)` and `World::landing()`, not one radius and the farthest crystal ([roadmap](../../Reports/larder-world-roadmap-2026-11-11.md), *WM1*).

## Sources

- [plan-matrix-2026-11-12.md](../../Reports/plan-matrix-2026-11-12.md).
- [larder-world-tiers-2026-11-12.md](../../Reports/larder-world-tiers-2026-11-12.md).
- [larder-world-ai-2026-11-12.md](../../Reports/larder-world-ai-2026-11-12.md).
- [larder-ai-2026-11-11.md](../../Reports/larder-ai-2026-11-11.md).
- [larder-m3-2026-11-11.md](../../Reports/larder-m3-2026-11-11.md), *Census* (the scripted plans it is compared with).
- [larder-world-roadmap-2026-11-11.md](../../Reports/larder-world-roadmap-2026-11-11.md).
