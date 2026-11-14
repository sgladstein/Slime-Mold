---
title: Plants
kind: mechanism
status: open
updated: 2026-11-14
sources:
  - Reports/plants-2026-11-14/README.md
code:
  - BigParams::graze_weight
---

# Plants

## Now

Plants are a new per-cell field for Larder World, separate from the old
grazing grass (`BigParams::graze_*`). Nothing is built yet: the
[brainstorm](../../Reports/plants-2026-11-14/brainstorm.md) lists ten
couplings and recommends food that regrows, cover for soldiers and fuel
for fire first. Maps would give each cell a start density in the level
PNG's B channel, written by [levelgen](planets.md).

## Owner's word

- 2026-11-14 17:02Z: "No. This should be fully new from the old grass." (levelgen thread, asked whether plants reuse the engine grass); biomes roll their own patterns, some more likely than others; any map beatable by any strategy.

## Open

- Which couplings to build first (asked 2026-11-14).

## Sources

- [plants report](../../Reports/plants-2026-11-14/README.md), *brainstorm*.
