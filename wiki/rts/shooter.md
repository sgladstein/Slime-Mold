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

A brainstorm, no runs and no code: [`mould-shooter-2026-11-11.md`](../../Reports/mould-shooter-2026-11-11.md).
A squad shoots the mould; both sides need the same food. The lead shape,
Larder, makes the player's stored food smell like any pile, so hoarding draws
the mould (a size cost from the scent field), scores food shipped out over a
fixed round, and makes a shot matter by holding a tube clear (an empty
stretch is a cut while it stays empty) rather than by killing walkers, which
the knife shows is weak. Four strategies (Fortress, Hunter, Farmer, Baiter)
are meant to counter each other through the state each pushes the mould into
(lean and tree-like, or fat and looped); that is a hypothesis. Three cheap
tests on today's dish come first; v0 would be a browser build on today's dish.

## Numbers

| Measure | Result | Seeds | Paired | Source |
|---|---|---|---|---|
| None yet: a brainstorm. The facts it leans on are on [knife](../tools/knife.md) and in [dead-ends](../../Reports/dead-ends.md) (`Params::light_weight`) | | | | [report](../../Reports/mould-shooter-2026-11-11.md) |

## Owner's word

- 2026-11-11 00:18Z: "brainstorm a demo where the slime mold is the enemy. The player is a soldier or unit of soldiers who can shoot the mold to destroy parts of it. The player needs food to upgrade, build defenses, etc. and the slime mold wants food to expand/survive, etc,"

## Open

- One soldier or a squad; solo or Versus (two squads, one mould); tests first or v0 first. Owner's call, in `HANDOFF.md`.
- Test 1 (does held fire cut supply where a snip does not) decides whether shooting clears or must scorch.

## Sources

- [mould-shooter-2026-11-11.md](../../Reports/mould-shooter-2026-11-11.md): the whole brainstorm.
- [rts-research-2026-11-10.md](../../Reports/rts-research-2026-11-10.md) and the [Creeper World card](../../research/sources/creeper-world-2025.md): the nearest published game with a simulated enemy.
