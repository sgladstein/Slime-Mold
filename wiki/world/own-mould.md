---
title: The player's own mould (gardeners)
kind: design
status: open
updated: 2026-11-14
sources:
  - Reports/larder-campaign-2026-11-12.md
  - Reports/player-mould-2026-11-12.md
  - Reports/species-gpu-2026-11-11.md
  - Reports/species-sets-2026-11-11.md
code:
  - BigParams::species_avoid_of
  - BigParams::species_cap
  - BigParams::species_convert
  - BigParams::species_convert_per
  - BigParams::species_convert_r
  - GpuWorld::garden
  - GpuWorld::garden_on_map
  - SpeciesSet
  - WORLD_GARDEN
  - World::own_living
  - World::tend
  - garden_settings
  - own_mould::Garden
  - own_mould::Garden::cord
  - own_mould::garden_on_map
---

# The player's own mould (gardeners)

## Now

The owner chose **gardeners** (2026-11-12 04:45Z): the player owns a second
species in the Larder world, which the soldiers grow (feed, roads, cut), and
it fights the enemy mould by conversion. It is built on the CPU
(`own_mould.rs`) and on the GPU, where it plays in the window with
`larder-world own=1` (off by default; Sow on the command card, Road and the
det cord on the tools page, `hand=sow`). Sowing grows it, a road pushes it forward, fire with craters
burns the enemy back; the enemy is not yet smaller for long. Each side
holds at most half the slots on both engines (`species_cap`, on the GPU
since 11-14), so a mission on `own=1` plays the same on either. It also goes on a
designed map (`own_mould::garden_on_map` and `GpuWorld::garden_on_map`,
11-14): the map's mould starts are the enemy's, the player's mould starts on
the crystal by the landing, each walker's side set where it was placed;
campaign mission 8 (Two moulds, `own` in a mission file) plays on it
([campaign report, *Mission 8 built*](../../Reports/larder-campaign-2026-11-12.md)).
Next: an attacker's edge in conversion. Detail below
([report](../../Reports/player-mould-2026-11-12.md), window
[page](https://claude.ai/artifact/SLmH8k3C797UCo3949RcMn)).

## Detail

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
([page](https://claude.ai/artifact/7Z4b4NB1JG1H7QEf9TrUer)). Each side may hold at most half the slots (`species_cap`), and a taken enemy core stops only the enemy's births; on the same seeds that changed nothing yet (the sides never neared it). Shots lay craters as in the one-mould game. **The front** (32 seeds, two soldiers for 20 s where the moulds meet): a road past the front pushes the player's ground forward (+886 cells, 27/4), sowing thickens it (+609, 25/6), fire with craters burns the enemy's back (-396, 24/7); all three hold under Holm. Without craters fire took nothing. One det cord (the toolkit's, `Garden::cord`, 16 cells across the line) takes about what 20 s of fire does at first (-289 enemy cells) but does not hold (10/21, p 0.07). **An enemy that hunts the player's mould feeds it** (`species_avoid_of`, CPU, off): about 7,000 walkers change sides each way in a sown game already, and with the pull net turning flips to the player under every hand (27/3, 30/2, 29/3); so the fight is not short of contact but symmetric, and hunting needs an attacker's edge to be a threat. A **front hand** (sow2 plus a front team of two that reads the line every 20 s and picks fire, sow or road) leaves the enemy smaller than sow2 on 6 of 8 seeds ([page](https://claude.ai/artifact/Rv4ZqMJ1hQDnJKiVcAXihx), GIF). All of this is 2,500 walkers a side on the CPU dish; only the direction should carry to the GPU.

**On the GPU and in the window** (`larder-world own=1`, off by default; the plain game stays hash-identical): the player's mould is species 3 on the engine (world bit `GARDEN` 32). Shots stop only at the enemy's trail, the squad's strike and cut spare species 3, the thick-slime slowdown counts only enemy walkers, and eat runs over every cell. Sow (W) is on the command card and Road (Y) on the tools page (E), beside the det cord (C), which takes both moulds alike on both engines: the player's cut. `hand=sow` plays it. Each side's birth cap runs on the GPU too (11-14: `capcount` before the births; twin test: capped at 1,000 the enemy reaches 1,027 against 1,116 free, the engines equal each frame). 1070 timing is booked.

## Owner's word

- 2026-11-12 04:45Z (card): "Gardeners: soldiers grow it". The other options were a mould-only mode and a bought ally.
- 2026-11-11 14:54Z: a player-owned species is agreed ([decisions-game](../decisions-game.md)).

## Tried and rejected

- An enemy drawn to the player's trail as its threat: it converts on arrival and feeds the player ([`dead-ends.md`](../../Reports/dead-ends.md), the enemy hunting the player's mould).
- Conversion on raw trail between forms that lay different amounts: deposit won every fight ([`dead-ends.md`](../../Reports/dead-ends.md), species conversion on raw trail).

## Open

- An attacker's edge in conversion (the fed or massed walker wins), so a hunting enemy is a threat.
- The front hand's thresholds are first guesses; tune by play.
- Cut's disc (radius 3) moves nothing visible.
- Cut was never used by a planned hand.
- Crust and spore-flier forms need per-species life-cycle rates.
- Rerun the forms with the one body thread's cohesion rules once they land.

## Sources

- [player-mould-2026-11-12.md](../../Reports/player-mould-2026-11-12.md): the proposal copy, the form census and the contact counts.
- [species-gpu-2026-11-11.md](../../Reports/species-gpu-2026-11-11.md): the species rules on the GPU and their cost.
- [species-sets-2026-11-11.md](../../Reports/species-sets-2026-11-11.md): per-species settings.
