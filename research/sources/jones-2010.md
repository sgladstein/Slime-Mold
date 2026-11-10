---
id: jones-2010
title: Characteristics of Pattern Formation and Evolution in Approximations of Physarum Transport Networks
authors: Jeff Jones
year: 2010
venue: Artificial Life 16(2):127-153
url: http://eprints.uwe.ac.uk/15260/1/artl.2010.16.2.pdf
read: full, 2026-11-09 (text and Table 1, Figs. 10-13 viewed)
copy: ../../Reports/mould-game-2026-11-08/depth-2026-11-09/papers/jones-2010-artl-physarum-transport-networks.pdf (+ .txt)
licence: publisher's copyright; kept in this private repo for reading only (owner, 2026-11-10)
used-in: ../../Reports/mould-game-2026-11-08/depth-2026-11-09/sources-research.md, ../../Reports/jenson-physarum-2026-11-10.md, ../../Reports/gpu-plan-2026-11-10.md
---

# Jones 2010: the walker-and-trail model

The model this game runs. Walkers with three forward sensors deposit onto a
trail that is blurred (3x3 mean) and decayed; walkers turn toward the
strongest sensor.

## Key facts for us

- Table 1 (p. 134): lattice 200x200, %p 3-15, SA 22.5 or 45, RA 45, SO 9,
  deposit 5, decay 0.1, one walker per cell, periodic edges. Our `Params`
  match every agent value except we run SA 45.
- SA 22.5 / RA 45 "never stabilizes completely" and visits the whole lattice;
  45/45 minimises and settles (pp. 135-137).
- Sensor offset sets the scale; bigger offset evolves faster (p. 138).
- A blocked walker stands for the gel (p. 132): what one-per-cell buys.
- Food weight sets Steiner tree (0.01) versus spanning tree (0.5)
  (pp. 145-147). Slow random removal shrinks a sheet onto food, the same
  network every run (pp. 147-148).

Full notes with page refs: `../../Reports/mould-game-2026-11-08/depth-2026-11-09/sources-research.md` section 3a.
