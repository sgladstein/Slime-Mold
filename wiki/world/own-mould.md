---
title: The player's own mould (gardeners)
kind: design
status: open
updated: 2026-11-12
sources: [Reports/player-mould-2026-11-12.md, Reports/species-gpu-2026-11-11.md, Reports/species-sets-2026-11-11.md]
code: [BigParams::species_cap, own_mould::Garden, World::tend, BigParams::species_convert_per, BigParams::species_convert, BigParams::species_convert_r, SpeciesSet]
---

# The player's own mould (gardeners)

## Now

The owner chose **gardeners** (2026-11-12 04:45Z). The player owns a second
species in the Larder world. The soldiers grow it: they feed it charge, lay
its roads and cut it. It fights the enemy mould by conversion. A CPU probe (`examples/ownmould.rs`, no soldiers)
put body forms against each other on a mirrored race world. The fast network
out-eats crawling hunks (14 of 16 seeds; no other eating pairing is
significant). Where bodies touch, though, the dense forms win every contact:
heavy and hunks each beat the network on 7 of 7 seeds. Forms are a trade
between eating and fighting, not a counter cycle
([report](../../Reports/player-mould-2026-11-12.md),
[page](https://claude.ai/artifact/H4Pd9hmsLHGgjqEciuNBmE)). For the probe,
conversion weighs each species' trail over its own deposit
(`species_convert_per`, off by default, CPU only).

**First CPU build** (`src/mould/own_mould.rs`, `examples/garden.rs`): a
`Garden` world with the enemy as species 0 and the player's mould as
species 3 (`OWN`) on the larder crystal; shots, smell and bites hit only
the enemy; soldiers take a Road, Feed or Cut task worked in a disc of
radius 3. Its only hook is `World::tend`, default 0, so the one-mould game
is unchanged. On 32 seeds, sowing (Feed while walking to an unclaimed
crystal and back) grows the player's mould 13 times farm's (32 of 32);
feeding in one spot does nothing; farming alone starves it. The enemy is not
smaller: the two blind species meet along one front and hold it
([page](https://claude.ai/artifact/7Z4b4NB1JG1H7QEf9TrUer)). Each side may hold at most half the slots (`species_cap`), and a taken enemy core stops only the enemy's births; on the same seeds that changed nothing yet (the sides never neared it).

## Owner's word

- 2026-11-12 04:45Z (card): "Gardeners: soldiers grow it". The other options were a mould-only mode and a bought ally.
- 2026-11-11 14:54Z: a player-owned species is agreed ([decisions-game](../decisions-game.md)).

## Tried and rejected

- Conversion on raw trail between forms that lay different amounts: deposit won every fight ([`dead-ends.md`](../../Reports/dead-ends.md), species conversion on raw trail).

## Open

- Contact: make the enemy seek the player's mould (`species_avoid` below 0 for the enemy only), so a big player mould is a threat it must answer.
- The front: how far each action moves the line between the two moulds in 20 s.
- Cut was never used by a planned hand.
- The fight needs contact: with the species blind to each other, it is rare. Attraction (`species_avoid` below 0) brings it on and hands the dense form the economy too (4 seeds).
- Crust and spore-flier forms need per-species life-cycle rates.
- Rerun the forms with the one body thread's cohesion rules once they land.

## Sources

- [player-mould-2026-11-12.md](../../Reports/player-mould-2026-11-12.md): the proposal copy, the form census and the contact counts.
- [species-gpu-2026-11-11.md](../../Reports/species-gpu-2026-11-11.md): the species rules on the GPU and their cost.
- [species-sets-2026-11-11.md](../../Reports/species-sets-2026-11-11.md): per-species settings.
