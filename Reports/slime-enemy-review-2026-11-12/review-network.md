# The lasting network: review (L2, B1, P1)

> Saved by the one-body thread from the reviewer's final message (subagents could not write files). Paths under `raw/` are this folder's `raw/`; pictures named in the text are in `img/` (see the report's picture list). Unedited otherwise.

I could not save `report.md` in my scratch folder: the tool refused report files from a subagent. The full report is below. Raw output and pictures are in `raw/network/`: the `q-*.txt`, `bd-*.txt` and `race-*.txt` files, plus folders of PNGs.

## 1. Can Jones reach it? Keep it and add to it. Don't replace it.

**Most of the "coarsening into strands" we measured is a test artefact.**
- `Engine::made` (src/gpu/mod.rs:1130) turns off the one-walker-a-cell limit whenever max(walkers, capacity) is more than half the agar cells. It does this silently.
- `race_params` sets capacity to 10 x walkers.
- So on the race world, every GPU run above about 6,500-8,000 walkers has no crowd limit. That includes `walkers=25000`, the 8.5 ms frame time at 20k walkers and the 100k scale goal.
- With no limit, Jones's rules fold the walkers into ropes. This has been known since 11-10: jenson-compare saw loops fall from 151.5 to 0.

Same game look, 25k walkers, spread start (folders under the path above):

| Run | Look at 30-60 s |
|---|---|
| Today's angles, capacity 250k (the brief's run, no limit) | a few thick strands by 30 s (`base/`) |
| SA 22.5 / SO 5, capacity 250k or 80k, no births | one ring (`so5nobirth/`; the 80k run gave the identical picture, so the limit was still off) |
| **SA 22.5 / SO 5, capacity 50k (limit on)** | **an even, continuous fine mesh over the whole map at 30-35 s** (`so5cap/`) |
| **Today's angles, capacity 50k, births on (grows to 48k)** | **a thicker mesh that still spans the map at 60 s** (`sa45cap/`) |

So Jones does hold a lasting, spread-out mesh in our game look. What it cannot do alone is give the mesh structure by age or by flow: mesh at the fronts, trunks behind (B1). Every part of the dish evolves at the same rate. That needs added layers.

- **Keep and extend:** keep Jones as the body. Every verb already writes into its fields: crater, alarm, cut, species. The additions cost roughly 0 to 0.1 ms a tick (estimated).
- **Replace with a Tero graph as a second `World`:** you get trunks but no body. Cuts, craters and the look would all need rebuilding, which is weeks of work, and the picture stops being walkers.
- **Hybrid (best):** a grid Tero-style flux field on the same engine. This is direction 5.

## 2. What our tests missed

- **The silent crowd drop.** The "15-20% density looks smooth" finding, the 25k coarsening test and the 8.5 ms at 20k were all measured without the limit. Re-run them with capacity at most half the agar.
- **A no-food spread start is not the game.** In the real game the mesh dies of mass:
  - Default race world (5k walkers): the mould falls to 1,750 walkers by 45 s and draws the food graph as beaded strands (`core/`).
  - Core start at 20k walkers with the limit on: a dense fan-shaped sheet at 10 s, then it starves to 7,084 by 30 s, leaving smooth food-to-food trunks and no mesh (`core20k/`).
  - A mesh needs about 15% local density (Jones Table 1, and our pictures). So the area a mould can hold as mesh is about walkers / 0.15.
- **Loops alone cannot tell mesh from trunks.** Add tube width, which BigMeasure already reports. Add a spatial measure too, such as loops in a band 30 dish cells inside the body's edge against loops inside it.

## 3. Jones 2010, Tero and the biology

**Jones 2010** (the PDF in the repo):
- **SA 45 / RA 45, the setting we ship, is his "minimising" regime.** Small loops close, the big holes grow, and the net tends to a hexagonal tiling (Fig. 5, 27 to 31,190 steps). In a bounded arena it condenses toward a sheet (Fig. 9).
- **SA 22.5 / RA 45 "never stabilizes completely".** New branches keep sprouting because the rotation angle is bigger than the sensor angle (Fig. 4, pp. 135-136).
- Sensor offset sets the scale; a larger offset evolves faster, and 3-7 gives fine nets (sec. 3.2).
- More walkers give thicker edges and fewer holes; faster decay gives coarser edges (Fig. 12).
- His parameter map (Fig. 13, 500 steps, 15% density) runs from reticular nets at low angles to labyrinths, then islands.
- He suggests slowing evolution over time to mimic the real mould sticking to its surface.

**Timescale:** at our 30 steps a second, Fig. 5's frames at 670, 2,364 and 13,160 steps fall at 22 s, 79 s and 7.3 min. Our CPU dish with the limit on matches that pace, so we do not run Jones too fast.

**Jones, Mayne & Adamatzky 2015** (arXiv 1511.05862, Table 1): their foraging runs grow from 10 particles using density windows. A walker that has just moved gives birth if its 9x9 window holds 0-15 walkers (checked every 3 steps). A walker dies if its 5x5 window holds more than 24. This keeps tubes thin.

**Tero 2010, and Almeida 2021** (arXiv 2305.12244):
- Tubes grow with flux^γ and shrink when unused.
- γ above about 0.5 gives trees with no loops; below 0.5 gives a redundant mesh with no hierarchy.
- Growth from one food source gives fan-shaped fronts with thick inner veins.
- Loops survive only when the food sources take turns pumping (Tero's random choice of source; in the real mould, shuttle streaming).

**Biology:** large plasmodia are fan-shaped, with "a sheet-like leading edge" and veins to the rear (Kuroda et al. 2015, JEB). Streaming inside reaches about 1 mm/s and reverses every 1-2 min, while the front creeps far slower. P1 is the biology: the front's speed is set by growth, not by how fast particles move.

## 4. Ranked directions

Measures are from the CPU dish (`bigdish`, 512x320, 25k walkers, no food, limit on, 2 seeds). Loops, then tube width in dish cells, from 5 s to 60 s.

**1. Fix the crowd drop and re-measure.** Cost 0.
- Base capacity on the agar cells and print a warning whenever the limit is dropped.
- First test: the 25k spread and core-start runs with capacity 50k, 8 paired seeds. Name loops and width at 30 and 60 s as the deciding measures.

**2. Leave the minimising regime: SA 22.5, SO 5.** Cost 0 ms.

| Setting | Loops | Width |
|---|---|---|
| Today (45/45, SO 9) | 37 → 12-13 | 10.3 → 17.6 |
| SA 22.5 | 50 → 20-23 | about 8.5, flat |
| SA 22.5 + SO 5 | 112-116 → 51 (flat from 30 s) | 7.0, flat |

- Respawn at 0.001 a step held 61-69 loops, but it teleports walkers across the map. A local version (re-randomising a walker's heading in place) would keep one body; it is untested.
- The 36 Points angle switch (22.5 in thin trail, 45 in thick) did no better than plain SA 22.5: 19-22 loops.
- Risk: it changes how mobile the mould is, so the census and the tier ladder would need re-checking.

**3. A front that grows instead of running (P1, L1).** Cost about 0.02 ms a tick.
- Walkers step slowly where the trail is thin and at full speed inside tubes.
- Tried with existing keys `step=0.15 trail_scale=0.02 step_by=0.85:2`: the front at 15 s was a solid fan-shaped sheet with few outriders (`slowfront/`).
- But the mould starved to 988 walkers by 45 s, against 1,750 today. It must be paired with direction 4.
- Why it gives B1: mesh age grows with distance behind a slow front. At about 1 dish cell a second and a 30 s coarsening time, mesh lasts about 30 cells behind the front, and trunks form further back.
- First test: the same rule with start energy that covers the first 60 s, 16 seeds. Measures: front speed (reach per second), largest part, crystals reached at 120 s, and random against plan.

**4. Pay for the mesh: births and deaths by local density (B1, P2).** Cost about 0.05 ms a tick for two box sums.
- Energy still pays for a birth, but a walker is born only where its 9x9 window holds 15 or fewer walkers. Walkers crowded past 24 in a 5x5 window shed.
- Births then go to the thin front, and fat trunks thin out.
- First test: the CPU race world at 300 s. Measures: loops in the outer band, width and walker count.

**5. Trunks from flux, not from food scent (B1, B2, B3).** Cost about 0.1 ms a tick for the field (a guess); a pressure solve once a second would add about 0.05 ms a tick averaged.
- A slow "tube" field that walker flow builds up as |flow|^γ, with γ below 1 so loops survive. It keeps about 0.999 a step.
- Walkers sense trail plus a weight times tube. Tube sets the drawn thickness and walker speed, so trunks are fast. Energy moves along tube instead of spreading evenly as `energy_flow` does today.
- Crystals take turns pumping on a 60-120 s cycle, so loops survive.
- Craters and cuts zero the field, so the existing verbs still work.
- First test: cut a trunk (B3) and measure how the far side thins over 10-30 s against today.

**6. A slower clock (15 steps a second).** It halves the simulation cost. Walkers drop to 7.5 dish cells/s, under the soldiers' 10, and coarsening takes twice as long. It is a pace knob, not a cure: the shot-damage report found a hole barely lasted longer at a 3x slower clock.

## Correctness

- The silent crowd drop above is the main one. The doc for `Settings::crowd` says the limit is dropped by walker count, but the code checks capacity, which is 10x walkers on the race world.
- Larder-world look runs end when the squad is wiped. In the no-limit run, a mesh over the whole map killed the squad by 35 s, so late frames in such runs show the end screen, not the mould.

Sources: Jones 2010, the PDF in the repo; [Jones, Mayne & Adamatzky 2015](https://arxiv.org/pdf/1511.05862); [Almeida 2021](https://arxiv.org/pdf/2305.12244); [Kuroda et al. 2015, JEB](https://cob.silverchair.com/jeb/article/218/23/3729/14430/Allometry-in-Physarum-plasmodium-during-free); [Wikipedia, Physarum polycephalum](https://en.wikipedia.org/wiki/Physarum_polycephalum).