---
title: Plants
kind: mechanism
status: open
updated: 2026-11-14
sources:
  - Reports/plants-2026-11-14/README.md
code:
  - BigDish::ignite
  - BigParams::fire_damp
  - BigParams::plant_grow
---

# Plants

## Now

A per-cell plant field in four kinds (meadow, jungle, reeds, scrub;
`mould::plants`), separate from the old grazing grass. It grows toward a
capacity set by soil, kind and wet and spreads from its edges, 15 plant
steps a game second, on the CPU world and the GPU engine (bit-exact twin
on llvmpipe). Fire (`BigDish::ignite`, lit by scorch rounds and blasts)
spreads through plants, eats them and leaves char; wet ground, reeds and
slime with trail >= `fire_damp` 3 don't burn, so fire takes the slime's
edge, not its body (probe: 501 killed in 80 s while it grew 5,405 to
7,669). Char comes from the plants that burnt (`char_gain` 1), so a
jungle leaves darker ground than a meadow. Drawn in view.wgsl to the
graphics lead's spec and review (18:20Z: plants and heat read through a
3 x 3 Gaussian so edges are soft and fire a halo; colour by heat^1.5,
capped at (250, 170, 60); char a warm grey-brown; heights jungle 1.2,
reeds 0.8, scrub 0.4, meadow 0.25); +3 ms a frame on
llvmpipe ([page](https://claude.ai/artifact/5pChXx8qdHxa3qwwP27HdY)).
Maps carry plants in the PNG's B channel (kind << 6 | density 1-63).

## Owner's word

- 2026-11-14 17:02Z: "No. This should be fully new from the old grass." (levelgen thread, asked whether plants reuse the engine grass); biomes roll their own patterns, some more likely than others; any map beatable by any strategy.
- 2026-11-14 17:29Z, on the brainstorm: "Slime doesn't eat plants at all." Cover: "slime senses by smells so that is a little convoluted. Lets think about it." Slow ground: "We can test this, but I am guessing it might be annoying." Fire: "I like this but need to think hard about how hard fire hurts slime. You don't want to make it too easy to just wipe out the whole slime with fire." Blocks shots, thickets, slime-eating plants, fertile battlefields: "we can try it. Need to playtest to see if it is fun." Harvest and sowing: "No." 

## Open

- Plants are on by default (menu Plants, `plants=0` off); maps without
  their own get `plants::stand_in` until levelgen writes them.
- Cover (plants muffling a soldier's scent) awaits the owner.
- Next: shots stopped by tall plants, thickets, slime-eating plants, fertile battlefields and slow ground (off), each a switch for playtest; fire on soldiers; the 1070's frame cost.

## Sources

- [plants report](../../Reports/plants-2026-11-14/README.md), *brainstorm*.
