# Look and density review (L1, L5, L6, P3, frame cost)

> Saved by the one-body thread from the reviewer's final message (subagents could not write files). Paths under `raw/` are this folder's `raw/`; pictures named in the text are in `img/` (see the report's picture list). Unedited otherwise.

**Verdict:** the Jones backend can give a continuous body (L1) only where walkers are packed tightly. At Jones's own density (%p 15, which is the owner's 100k walkers on 1024x640), that is most of the body, and the 1070 can afford it. At the fronts, thin trunks and cut pieces, adding walkers does not help. Those are the parts play happens in. Fixing them needs a change in the simulation, or a body field of its own.

**How I measured.** I made no repo edits and no builds. I ran `bigdish` as a CPU stand-in for the race world: same settings, but no map and no squad. I saved the raw trail and walker fields at 60 and 120 game seconds on seeds 1-3 and measured them offline. "Speckle" below means how uneven the drawn brightness is inside the body; 0 is perfectly smooth. Everything is in `raw/look/` (numbers in `raw/look/results.txt`). The tool would not let me save a `report.md` there, so the report is only this message.

## 1. Can Jones reach a continuous body?

The body splits into two parts:

- **The packed part** holds 80-98% of the walkers: crystal lumps and fat trunks. It is smooth at any population.
- **The sparse part** holds about 0.11 walkers a cell, and that number does not change with population:
  - fixed populations of 5k, 10k and 25k on 512x320: 0.12, 0.11, 0.09;
  - growth worlds at today's 5k and at the 25k film's settings: 0.11 both.

The specks are in the sparse part, and its speckle does not fall with more walkers: trail speckle 0.31, 0.32 and 0.33 at 5k, 10k and 25k, lower on 0 of 6 paired runs.

What more walkers do is shrink the sparse part's share of the body's area:

| World | Sparse share of the body's area |
|---|---|
| Today (5k, growth) | 60% |
| 25k, growth | 22% |
| 25k, fixed | 16% |

The extra walkers go into lumps, and the body's area grows 3.8x (7.4k to 28k fine cells).

**Why it beads.** Jones's own trail settings (keep 0.9, 3x3 mean) smooth over about 1.8 cells. Walkers in the sparse part are about 3 cells apart, so their trail breaks into beads.

**The drawing can't fix this cheaply.** A smooth picture from 0.11 walkers a cell needs a smoothing window about 6 cells wide. I tested a long-memory field laid by the same walkers (keep 0.98, offline): it is continuous, but only by blurring tubes into soft glows.

| Drawing of the sparse part | Speckle | Gaps |
|---|---|---|
| Today's game look | 0.55 | 43% |
| Trail alone | 0.29 | 3% |
| Long-memory field (keep 0.98, with spread) | 0.17 (3 of 3 seeds) | 2% |
| Long-memory field, little spread | 0.30-0.34 | — |

## 2. What our tests missed

- **The 25k film hides its fronts.** The view scales `BODY_N` by the starting density, so n0 goes from 6 to 30. The sparse part has 2.7 walkers per 5x5 window in both worlds, but it is drawn at 8% of its trail at 25k against 33% today. The film looks smooth partly because the thin parts are nearly invisible.
- **The view's body term multiplies the specks.** In the sparse part, trail × walkers-in-5x5 leaves gaps in 43% of the tube core, against 3% for the trail alone, and it nearly doubles the speckle (0.55 against 0.29). By eye in the films, the height shading also turns each bump into a separate glossy droplet. I did not measure that part.
- **The "trail alone" film can't be reproduced.** It came from an uncommitted shader edit, and it was still shaded and shown at zoom 6.
- **"3-5% of cells holding a walker" is the wrong measure.** The right ones are walkers a cell in the sparse part, or the sparse share of the body's area.
- **Pixels per cell differ.** Jones's figures are about 0.5 px a cell and Jenson's about 1. Our near view is 6 px a cell, so any Jones run would look grainy there.
- **The one-walker-a-cell crowd rule is not what clumps them.** Bunching beyond chance is modest: 1.29 against a local shuffle. With no crowd rule it rises to 1.39, and gaps in the trail go from 4% to 34%, because walkers stacked on one cell move as one.
- **30 steps per game second is not a cause.** Each step is the same as Jones's, and a still picture doesn't depend on steps per second. keep 0.95 or a second diffusion pass smooth the sparse trail by 22-29% (6 of 6 runs), but they change behaviour: reach drops to 63-76 on 2 of 3 seeds, and loops go to 0.
- **My own measure changed after I looked.** My first named measure (counting small pieces) mostly counted stray walkers and disagreed with the pictures. I switched to speckle inside the sparse part. Treat these results as directions from 3 seeds, not proofs.

## 3. What the literature and biology suggest

- **Jones 2010**, Table 1 (p. 134): %p (walkers as a percentage of cells) 3-15, one walker a cell. Most of its figures are at %p 15 on lattices of 200-500 cells a side. At %p 2-5 (Figs. 18-20) the network contracts into a small area, so its tubes stay packed. The paper itself notes "diminished local activation at small %p".
- **Jenson 2019:** 5-10 million particles on a GTX 1070, with no crowd limit.
- **Bleuje** (port of Jenson's later work): 5.8 million particles on 1280x736, about 6 a pixel.
- **Lague:** 0.12 particles a pixel, smooth only because its trail lasts hundreds of steps.
- **Jones & Adamatzky 2012:** a coherent moving blob needs a packed body, 900-9,380 particles at one a cell.
- **Tero 2010:** a continuous tube model with no particles at all.
- **Real Physarum:** the advancing front is the thickest part, a fan fed by shuttle streaming.

**What this means for the 100k goal.** 100k walkers on 1024x640 is %p 15. Measured, that gives a body about 80% packed. Packed parts hold about 0.9 walkers a cell, so 100k smooths about 110k cells, roughly 17% of the map. The fronts stay specks.

## 4. Ranked directions

Frame costs are for the GTX 1070. Measured figures are marked; the rest are my estimates.

**1. Stop the view amplifying the specks** (no change to the simulation)
- **Change:** fix n0 per walker mass instead of scaling it by starting density, and read the body from a 9x9 window (or take the trail where walkers are sparse).
- **Why:** it removes the roughly 12x gap amplification in the sparse part.
- **Risk:** L5 holes may read less clearly.
- **Cost:** +0.1-0.2 ms at zoom 1, 1080p (scaled from the 5x5 tile's +0.14).
- **First test:** draw both versions offline from saved fields, then a blind A/B. Deciding measures: gaps in the sparse part (today 43%) and the `holes=1` refill time.

**2. Keep the front packed in the simulation** (the real L1 fix)
- **Change:** a birth lands on the hungriest nearby trail cell instead of beside its parent, or a tether at the margin only, with energy sharing and hunger on.
- **Why:** it is also P1's slow front.
- **Risk:** less spread (`lone_upkeep` stopped it; `lone_step` cost 25%).
- **Cost:** +0.01-0.03 ms a tick.
- **First test:** `cutprobe` with `tether=2 energy_flow=0.05 hunger_scent=8`, 8 seeds, 300 s. Deciding measures: sparse share of the body (today 0.60), crystals at 300 s (today 6.3), largest part.

**3. Route A at %p 15 with `walker_mass`** (25k on 512x320, or 100k on 1024x640)
- **Why:** sparse share falls to about 20%, and P3 keeps damage, drain and food counted in mass.
- **Risk:** about 4x the body area for 5x the walkers, the economy must be rebalanced, and the fronts don't change.
- **Cost:** engine at 100k is 1.37 ms per 2 ticks (measured, no growth); with the world layers and growth I estimate about 2.3 ms. At 25k on today's grid, about +0.3-0.5 ms. The CPU census is 3.7x slower per tick (3.4 to 12.5 ms, measured).
- **First test:** census with `walkers=25000 upkeep=0.0002 walker_mass=0.2`, 16 seeds. Deciding measures: plans still beat random and idle, and sparse share at or below 25%.

**4. A body mass field** (hybrid, the tech lead's "second `World`")
- **Change:** a field laid by living walkers that shots remove, soldiers are drained by and food flows through; every verb writes into it.
- **Why:** it is continuous at any walker density.
- **Risk:** about 6 cells of blur, and parts fade over about 1.7 s. That may read as a drawn fade unless the rules use the field.
- **Cost:** +0.02 ms a tick at 512x320, about +0.07 at 1024x640.
- **First test:** run the GPU world laying the field. Same measures as direction 2, plus an A/B for the owner.

**5. Route B (`scale=1`), but only as one tier's silhouette (L6)**
- **Why:** it halves speckle in world units (0.13 against 0.28) and runs about 3.6x faster on the CPU.
- **Cost:** it loses the mesh (0-1 loops, tubes 13 dish cells wide, 3.5x the body area) and looks blocky at 12 px a cell.

## 5. Correctness

- `set_body_density` is set once from the starting count (`src/bin/larder-world.rs:393`), while the living count swings 2-3x during a game.
- My offline field used walker positions at tick ends only. A control field with the trail's own keep scored about 13% worse than the real trail, so my figures for the long-memory field are conservative.

Pictures (in the scratch folder):
- `raw/look/cmp_g120.png`: rows today, 25k, coarse grid; columns trail, game body, blurred trail, walker density.
- `raw/look/crop1.png`: a close-up of today's and 25k's thin tubes.
- `raw/look/body_full_s1.png`: the long-memory body field next to the trail, game look and walkers.
- `raw/look/films_near.png`: one frame each of the existing 25k and trail-alone films.