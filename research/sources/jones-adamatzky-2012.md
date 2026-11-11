---
id: jones-adamatzky-2012
title: Emergence of self-organized amoeboid movement in a multi-agent approximation of Physarum polycephalum
authors: Jeff Jones, Andrew Adamatzky
year: 2012
venue: Bioinspiration & Biomimetics (preprint arXiv 1212.0023)
url: https://arxiv.org/abs/1212.0023
read: pages 1-13 (all but the references) via alphaXiv, 2026-11-11; quoted passages via SciSpace, 2026-11-09
copy: none
licence: publisher's copyright (preprint on arXiv)
used-in: ../../Reports/mould-game-2026-11-08/depth-2026-11-09/sources-research.md, ../../Reports/mould-tiers-2026-11-11.md
---

# Jones & Adamatzky 2012: the amoeboid blob

## Key facts for us (page numbers of the arXiv preprint)

- Oscillation and travelling waves emerge from jammed walkers; lowering the
  chance a blocked walker re-orients strengthens them. Blobs migrate; food
  strengthens the oscillation, light weakens it.
- The oscillatory motor rule (pp. 4-5): a blocked particle keeps facing the
  same way until the cell in front frees, instead of taking a random
  heading; `pID` is the chance it resets, lower giving stronger inertia.
  Particles update one at a time in random order, and lay trail only when
  they move. Diffusion: 3x3 mean, damping 0.07.
- A 9,380-particle blob at SA 90, RA 45, SO 15 condenses into a disc; pID
  0.05 starts oscillation, pID 0.01 moves it (Fig. 3, pp. 6-7). Other
  SA/RA with a smaller SO fragment it (p. 7).
- A small blob (900 particles, SA 90, RA 45, SO 9, pID 0.001) keeps a dome
  shape and moves steadily forward (Fig. 4, pp. 7-8).
- Light: sensor readings in a lit region are multiplied by a factor under 1;
  the blob moves out of the light (Fig. 6, pp. 9-10). A band of light
  through a 5,000-particle blob cleaves it in two; two blobs fuse once their
  border particles sense each other's trail (Fig. 7, pp. 10-11).
- Food pulls a pseudopod that engulfs it, then the blob rounds up again
  (Fig. 5, p. 9).

**Used:** the light rule is the model for `BigParams::alarm_weight` under 0
(a mould that flees a strike), [mould tiers](../../Reports/mould-tiers-2026-11-11.md).
