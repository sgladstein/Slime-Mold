---
title: The player's own mould (gardeners)
kind: design
status: open
updated: 2026-11-12
sources: [Reports/player-mould-2026-11-12.md, Reports/species-gpu-2026-11-11.md, Reports/species-sets-2026-11-11.md]
code: [BigParams::species_convert_per, BigParams::species_convert, BigParams::species_convert_r, SpeciesSet]
---

# The player's own mould (gardeners)

## Now

The owner chose **gardeners** (2026-11-12 04:45Z). The player owns a second
species in the Larder world. The soldiers grow it: they feed it charge, lay
its roads and cut it. It fights the enemy mould by conversion. Nothing is
built into the game yet. A CPU probe (`examples/ownmould.rs`, no soldiers)
put body forms against each other on a mirrored race world. The fast network
out-eats crawling hunks (14 of 16 seeds; no other eating pairing is
significant). Where bodies touch, though, the dense forms win every contact:
heavy and hunks each beat the network on 7 of 7 seeds. Forms are a trade
between eating and fighting, not a counter cycle
([report](../../Reports/player-mould-2026-11-12.md),
[page](https://claude.ai/artifact/H4Pd9hmsLHGgjqEciuNBmE)). For the probe,
conversion weighs each species' trail over its own deposit
(`species_convert_per`, off by default, CPU only).

## Owner's word

- 2026-11-12 04:45Z (card): "Gardeners: soldiers grow it". The other options were a mould-only mode and a bought ally.
- 2026-11-11 14:54Z: a player-owned species is agreed ([decisions-game](../decisions-game.md)).

## Tried and rejected

- Conversion on raw trail between forms that lay different amounts: deposit won every fight ([`dead-ends.md`](../../Reports/dead-ends.md), species conversion on raw trail).

## Open

- The first gardeners build on the CPU world, behind a switch: the player's mould on the larder crystal, shots and bite that tell species apart, Road, Feed and Cut. It goes in its own file with a thin hook in `larder_world` (agreed with the game integration thread).
- The fight needs contact: with the species blind to each other, it is rare. Attraction (`species_avoid` below 0) brings it on and hands the dense form the economy too (4 seeds).
- Crust and spore-flier forms need per-species life-cycle rates.
- Rerun the forms with the one body thread's cohesion rules once they land.

## Sources

- [player-mould-2026-11-12.md](../../Reports/player-mould-2026-11-12.md): the proposal copy, the form census and the contact counts.
- [species-gpu-2026-11-11.md](../../Reports/species-gpu-2026-11-11.md): the species rules on the GPU and their cost.
- [species-sets-2026-11-11.md](../../Reports/species-sets-2026-11-11.md): per-species settings.
