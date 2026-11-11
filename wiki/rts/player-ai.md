---
title: The Larder player AI
kind: tool
status: current
updated: 2026-11-11
sources: [Reports/larder-ai-2026-11-11.md, Reports/larder-m3-2026-11-11.md, Reports/larder-world-roadmap-2026-11-11.md]
code: [Brain, Style, Side, RandomHand, set_rule, Game::apply]
---

# The Larder player AI

## Now

`src/mould/larder_ai.rs` is an AI that plays Larder's soldier side for automated playtests ([report](../../Reports/larder-ai-2026-11-11.md)). It gives only the player's commands (`Cmd` through `Game::apply`), so every AI round is a replay. It reads only what a player sees (a `View`), and it commands a `Side`, so a second soldier team later is a second brain. There are five preset styles (turtle, boom, rush, lure, adapt) turned by named knobs, plus idle and random controls. On held-out seeds 17-32 of today's CPU field, boom, lure and adapt beat idle and random on 16 of 16 in both fields. Turtle and rush lose to idle on 16 of 16, as the scripted fortress and hunter did in M3. This is a harness waiting to be ported to the world version, not a balance result: the owner ruled no more small-dish tuning.

## Numbers

| Measure | Result | Seeds | Paired | Source |
|---|---|---|---|---|
| boom / lure / adapt against idle and random, fed and lean field | each wins 16 of 16 | 16 (17-32) | p 0.00003, survives Holm over 20 | [report, *Census*](../../Reports/larder-ai-2026-11-11.md) |
| Median score fed / lean: boom, lure, adapt | -0.389 / 2.035, -0.500 / 1.934, -0.400 / 0.500 | 16 (17-32) | lean: boom > adapt 15/16, lure > adapt 16/16 survive Holm | [report, *Census*](../../Reports/larder-ai-2026-11-11.md) |
| Best style per seed, fed / lean (boom/lure/adapt) | 5/6/5 and 12/4/0 | 16 (17-32) | - | [report, *Census*](../../Reports/larder-ai-2026-11-11.md) |
| Buy only what the bank can arm (30 s of fire), lure, fed | wiped 8 of 8 before; capped 6 of 8 after | 8 (1-8) | - | [report, *Two AI mistakes*](../../Reports/larder-ai-2026-11-11.md) |
| AI think cost | 15-39 µs once a game second (1-3 µs a step) | 16 | - | [report, *Census*](../../Reports/larder-ai-2026-11-11.md) |

## Owner's word

- 2026-11-11 14:07Z: "maybe a player AI thread, for automated testing so you can keep developing I without me having to playtest everything. Also, it can be interesting to maybe have two soldier teams against multiple molds so in the future the AI could also play soldiers" ([decisions](../decisions.md)).

## Tried and rejected

- Boom with two guards burning round home on the alarm: wiped 8 of 8 in both fields (seeds 1-8); a source comment on `Style::BOOM` records it.

## Open

- Port `View` and `Side` to the Larder world when crystals and a second side land; add a team id to `Cmd` there ([roadmap](../../Reports/larder-world-roadmap-2026-11-11.md)).
- W1: how often the early leader wins with each opening and crystal rule, as a per-mould difficulty stat on the mould ladder (owner, 14:14Z and 14:54Z), not a gate.
- An exploiter search over `Style` knobs against a rule change.

## Sources

- [larder-ai-2026-11-11.md](../../Reports/larder-ai-2026-11-11.md).
- [larder-m3-2026-11-11.md](../../Reports/larder-m3-2026-11-11.md), *Census* (the scripted plans it is compared with).
- [larder-world-roadmap-2026-11-11.md](../../Reports/larder-world-roadmap-2026-11-11.md).
