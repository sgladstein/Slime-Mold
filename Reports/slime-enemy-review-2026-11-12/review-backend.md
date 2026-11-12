# Backend review: is Jones the right engine?

> Saved by the one-body thread from the reviewer's final message (subagents could not write files). Paths under `raw/` are this folder's `raw/`; pictures named in the text are in `img/` (see the report's picture list). Unedited otherwise.

The tool would not let me save `report.md` to the scratch folder, so the full report is this message. The pilot's scripts and pictures are saved there.

## 1. Answer: extend. Keep Jones, add a supply layer under it.

The Jones walkers are a good engine for **texture, exploring and steering**. They are a poor engine for **a body**. Tuning can fix the specks. It cannot give one supply, cut pieces that act as pieces, or two speeds. Those need a quantity at the scale of the whole body, and the Jones model has none. Its only coupling is a local trail field.

My recommendation is to **extend**, not replace. Add a coarse adaptive flow lattice under the walkers, after Tero et al. 2010: tubes that thicken with flux and wither without it. Walkers keep exploring, drawing, dying to shots and reading every field we have. The lattice carries the food, sets the trunks and defines the pieces.

Scored against every goal:

| Backend | Gets nearly free | Struggles with | 1070 cost | Keeps of today |
|---|---|---|---|---|
| Jones as run | L5, L6, B4 steering, all tools | L1 at 3-5% density; L2 (mesh coarsens to strands by ~30 s); L3/B3 (moving pieces stream, largest part 0.27-0.58); P1 (one speed); B2 (sharing is a cliff or invisible) | 1.37 ms per 2 ticks at 100k walkers (`wiki/model/scale.md`) | everything |
| Jones tuned | L1 (smooth at 15-20% density, measured); some L2 (respawn keeps a mesh but erases trunk memory, `jenson-looks`) | B2, B3, L3, P1: eight one-body rules failed (`dead-ends.md` lines 53-59) | about +0.5 ms at 100k | everything |
| Tero flow network | B1 trunks between food; B2; siege; B5 (thick trunks); P2; P3 (tube volume); L3 (pieces are graph components) | exploring front, texture, L5; trees rather than loops unless the sources fluctuate | coarse lattice, about 0.1-0.3 ms a frame (estimate) | nothing on its own |
| Fields (Gray-Scott, Keller-Segel mass, Creeper-style fluid, Hu-Cai continuum) | L1, P3; lumps (Keller-Segel); slow fronts (fluid) | foraging network (Gray-Scott, fluid); lasting mesh needs an implicit solver (Hu-Cai) | cheap stencils | little |
| Cellular Potts / CA | cohesion of a single blob | networks at 655k cells; sequential Metropolis updates are hostile to a GPU | medium | little |
| **Hybrid: flow lattice under the walkers** | the union of rows 2 and 3 | moving-hunk cohesion still unproven; frontier specks | about +0.2-0.4 ms a frame | nearly all |

**The coordinator's framing, "what layer on top of Jones fixes 1 and 2".**

- **Criterion 1 (one organism).** The supply lattice fixes it. A piece is a connected component of tubes, which gives the piece id that the one-body report and the align/pull dead ends say is missing. A walker eats from its node's volume, so a straggler that leaves the piece's tubes starves. A cut stops the flux, so the far tubes visibly thin.
- **Criterion 2 (intent shown ahead).** The pressure gradient shows it. A sink (hungry front, soldier scent) draws flux, and the trunk toward it thickens and streams seconds before the front arrives. Fire turns nodes into sources and the flow runs away from them. That is motion of the simulation's own quantity, not a drawn effect.

**What each option costs:**

- **Keep:** 1-3 days (walker mass factor, more walkers). Fixes L1 only.
- **Extend:** a one-day CPU prototype, then about a week for the GPU port. That is 3-4 passes, new Params rows and twin tests.
- **Replace** (mass field plus continuum network): weeks of work. It re-gates every rule and loses L5's walker deaths. Only worth it if the extension fails.

**How it fits the current code.** The lattice lives inside `BigDish` as off-by-default `BigParams` fields, so the existing `World` impl, `cutprobe` and the `larderworld` census run it unchanged. Every verb still has a field to write into:

- craters zero the lattice's tubes in the disc;
- alarm and scar stay walker fields, and can also bias node demand;
- species get one lattice each, or share one.

## 2. Is our approach right? What the tests missed

- **Density and size are confounded.** Every one-body arm changed the mould's size: H was 2.5x, and sharing was either a cliff or invisible. So the look tests compared moulds of different mass. There has been no control with the same mass and more walkers. That needs the agreed walker mass factor: each walker counts as a quarter, upkeep and births are scaled, and there are 4x the walkers.
- **There is one speed.** Walkers cross 15 dish cells a second, faster than soldiers (10). A 3x slower clock still refilled a hole in 0.73 s (`dead-ends.md` line 47). No test measured the front's advance separately from the trunk's refill time. P1 needs both numbers.
- **"Largest part" measures nearness, not body.** With a 2-cell bridge it counts walkers close together. B2 and B3 ask whether a walker is fed by the core. A supply-connected share would measure that directly.
- **Cuts change nothing the walkers read.** A cut removes trail, the network loops, and the gap refills in 2 s. So the test was sound but the model has no supply for a cut to stop.
- **The tests ran at the wrong scale.** The look tests ran 5,000 walkers on 512 x 320. The engine is built for 100k on 1024 x 640, and Jenson ran millions with no crowd limit. Coarsening was measured with `start=spread food=0`, which is not the game's condition.
- **Frame budget.** The 8.5 ms p95 at 20k walkers is the whole game frame on a busy PC. The engine itself is 1.37 ms at 100k. **The backend is not what limits the frame**, and replacing it would not buy frame time.

## 3. What biology and published models suggest

- **The plasmodium is one cell, so identity comes free.** It moves by shuttle streaming through elastic tubes. The flow period is about 100 s (131 ± 43 s), with about one wavelength across a 3-17 mm body (Alim et al. 2013, PNAS 110:13306, pp. 13306, 13308).
- **Tubes are the memory.** A food event is stored in tube diameters within about 15 min. An idle network prunes thin tubes first (Kramar & Alim 2021, PNAS, pp. 2-3, 5).
- **Tero 2010** (Science 327, pp. 440-441; PDF at pdodds.w3.uvm.edu):
  - D grows with f(|Q|) = |Q|^γ/(1+|Q|^γ) and decays without flow, starting from a fine random mesh.
  - Higher I0 or lower γ gives alternative routes and cross-links; low I0 or high γ gives a minimum spanning tree.
  - So "mesh versus strands" is a dial with a known direction, unlike our coarsening.
- **Growth plus adaptation has been built** (Almeida 2021, arXiv 2305.12244, pp. 59-64). Food pumps flux to boundary sinks, which spend it on new tubes.
  - At γ 2/3 it gives fan-shaped fronts and a trunk hierarchy, but no stable loops.
  - At γ below 1/2 the network stays a mesh with no hierarchy.
  - Direct food-to-food tubes appear when sources alternate. That is pulsing food: shuttle streaming, a visible simulated pulse.
- **The continuum version holds a mesh.** It selects a channel spacing that "survives without coarsening to larger scales", with λ ~ R^-1/4. But it was solved by implicit Newton with LU (Holden et al. 2026, arXiv 2605.16130, pp. 1-4, 6-7). That is a reason to keep the network on a coarse lattice and let the walkers carry the fine scale.
- **Jones's own model coarsens.** SA 45 / RA 45 minimises and settles (Jones 2010, pp. 135-137). Explore-then-network agent states, switched by starvation, are a published variant (Reginato et al. 2024, arXiv 2412.19790, pp. 7-8, 13).

## 4. Ranked directions

**1. Matched-mass density (keep; test first, half a day).** This is the tech lead's middle path.
- *Mechanism:* walker mass 0.25, with births and upkeep scaled so mass matches today within 10%.
- *Why it should work:* it fixes tube specks without making the mould stronger.
- *Risk:* frontier walkers are "alone by definition" (`one-body`, Lone step), so the front stays specky. The CPU census runs about 3-4x slower.
- *1070 cost:* about +0.5 ms a frame (0.82 ms at 32k to 1.37 ms at 100k).
- *Experiment:* race seeds 5-20 at 300 s. Name the measures first: a blind A/B of the game look at 60 s, plus the gate (Holm over 6). It removes the density confound from every later test.
- *What it will not fix:* B2, B3, P1 or coarsening.

**2. Supply lattice under the walkers (extend; the recommendation).**
- *Mechanism:*
  - The lattice is 128 x 80 on the race world (one node = 4 x 4 fine cells), or 256 x 160 on the world map.
  - Crystals credit their node's volume.
  - The elastic tubes are relaxed explicitly, 4 substeps a tick, with no global solve. Supply travels at a finite speed, so a cut's effect spreads over seconds.
  - Tubes adapt by Tero's rule.
  - A walker's energy comes from its node, replacing `energy_flow`.
  - A walker's step scales with its node's conductance along the flux, and walkers off a tube are slow. That gives two speeds.
- *Risk:* moving hunks may still stream. Tuning γ and I0 against coarsening.
- *1070 cost:* 41k nodes at about 40 B per substep is about 1.6 MB, so dispatches dominate. My estimate is +0.2-0.4 ms a frame. That is not measured on the 1070, and the GPU port is not built. For comparison, sharing plus hunger measured +0.23 ms.
- *Experiment:* see the day prototype below.

**3. Two speeds on Jones alone (keep; one look).**
- *Mechanism:* the existing `step_mod` (a GPU key) with `sense_scale`, so walkers are fast in strong trail and slow in weak.
- *Experiment:* measure front advance (95th-percentile radius, dish cells a second) and gap refill time. To pass, the front must be under 10 dish cells a second and the refill at most 2 s. If it passes, P1 needs no new layer.

**4. Mass field body (replace; fallback only).**
- *Mechanism:* a Darcy or Keller-Segel mass moving along a pressure field, with a Hu-Cai conductance field.
- *What it gives:* L1, L3 and P3 directly.
- *Cost:* loses walkers and re-gates every tool.
- *When:* only if direction 2 fails its measure (a) below.

**Rejected as the core:**
- Gray-Scott: a tier's look, no foraging.
- Creeper-style fluid: a "flood" tier at most. It is a fluid, not a creature.
- Cellular Potts: networks are too large, and the updates are hostile to a GPU.

## 5. The one-day prototype that settles it

Build direction 2 on the CPU as off-by-default `BigParams` fields, with keys in `cutprobe` and the census. Name the deciding measures before any run:

- **(a)** A held ring cut off food, on fork and rim, 6 seeds:
  - the piece's walkers fall to a tenth in 10-30 s;
  - its largest part at 20 s is at least 0.8 when moving (today 0.27-0.58);
  - its tubes visibly thin.
- **(b)** Front speed under 10 dish cells a second, and trunk refill of a gap within 2 s.
- **(c)** Mould size at 300 s within ±25% of today's 7,658 walkers.
- **(d)** The gate on race seeds 5-20: farm, wall and rush each beat idle and random, Holm over 6.

**Decision:**
- If (a), (b) and (d) pass: extend, and port to the GPU.
- If (a) fails: the body needs a mass field, which is direction 4.
- If only (c) or (d) fails: it is tuning.

**Pilot already run:** about 120 lines of numpy, one seed, guessed numbers, not a test. Files are in `raw/backend/`: the scripts `flow_pilot.py`, `flow_pilot2.py`, `flow_pilot3.py` and `flow_cut.py`, the picture `pilot-trunks-119s.png`, and the folders `run3/` and `cut/`.

- It grew a continuous sheet with a hierarchy of trunks from the food.
- Explicit elastic relaxation at 4 substeps was stable, at 1.0 ms a tick for 10k nodes in numpy on one core.
- After a held cut, the far piece's tubes fell from conductance 0.12 to 0 in 6-8 s while the fed side held at 7-8.
- The far nodes did not starve. Store and upkeep need tuning.
- Alone, the lattice grew as a slow compact disc: radius 13 nodes in 180 s. Its home crystal ran dry before it reached another. **The lattice does not explore**, which is the walkers' job. That supports the split.

## 6. Correctness notes

- Jones 2011 is marked "paywalled, needs upload" in `research/sources/jones-2011.md`. A search turned up a copy at `ics-websites.science.uu.nl` (not opened).
- The "largest part" share depends on the 2-cell bridge. Quote that bridge with every value, or switch to the supply-connected share.
- No repo files were changed and nothing was built.