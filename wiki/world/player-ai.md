---
title: The Larder player AI
kind: tool
status: current
updated: 2026-11-12T00:30Z
sources: [Reports/larder-ai-2026-11-11.md, Reports/larder-m3-2026-11-11.md, Reports/larder-world-roadmap-2026-11-11.md]
code: [Brain, Style, Side, RandomHand, set_rule, Game::apply]
---

# The Larder player AI

## Now

`src/mould/larder_ai.rs` is an AI that plays Larder's soldier side for automated playtests ([report](../../Reports/larder-ai-2026-11-11.md)). It gives only the player's commands (`Cmd` through `Game::apply`), so every AI round is a replay. It reads only what a player sees (a `View`), and it commands a `Side`, so a second soldier team later is a second brain. There are five preset styles (turtle, boom, rush, lure, adapt) turned by named knobs, plus idle and random controls. On held-out seeds 17-32 of today's CPU field, boom, lure and adapt beat idle and random on 16 of 16 in both fields. Turtle and rush lose to idle on 16 of 16, as the scripted fortress and hunter did in M3. This is a harness waiting to be ported to the world version, not a balance result: the owner ruled no more small-dish tuning.

## Owner's word

- 2026-11-11 14:07Z: "maybe a player AI thread, for automated testing so you can keep developing I without me having to playtest everything. Also, it can be interesting to maybe have two soldier teams against multiple molds so in the future the AI could also play soldiers" ([decisions](../decisions.md)).

## Tried and rejected

- Boom with two guards burning round home on the alarm: wiped 8 of 8 in both fields (seeds 1-8); a source comment on `Style::BOOM` records it.

## Open

- Port `View` and `Side` to the Larder world's game layer (`larder_world::Game`, W2b: one squad, `Cmd` with `Wall`), then a second team: a team on each soldier, a home per team, and `Game::apply_as(team, cmd)` logging the team on the replay line ([roadmap](../../Reports/larder-world-roadmap-2026-11-11.md), *W2b*).
- W1: how often the early leader wins with each opening and crystal rule, as a per-mould difficulty stat on the mould ladder (owner, 14:14Z and 14:54Z), not a gate.
- An exploiter search over `Style` knobs against a rule change.
- On a map (WM1) crystals differ in size and the larder has its own place: read `World::crystal_r(c)` and `World::landing()`, not one radius and the farthest crystal ([roadmap](../../Reports/larder-world-roadmap-2026-11-11.md), *WM1*).

## Sources

- [larder-ai-2026-11-11.md](../../Reports/larder-ai-2026-11-11.md).
- [larder-m3-2026-11-11.md](../../Reports/larder-m3-2026-11-11.md), *Census* (the scripted plans it is compared with).
- [larder-world-roadmap-2026-11-11.md](../../Reports/larder-world-roadmap-2026-11-11.md).
