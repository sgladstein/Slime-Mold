---
title: The mould at scale (outside models and the GPU plan)
kind: mechanism
status: open
updated: 2026-11-10
sources: [Reports/jenson-physarum-2026-11-10.md, Reports/gpu-plan-2026-11-10.md, research/sources/jenson-physarum-post.md, research/sources/jones-2010.md, research/sources/mcpm-2022.md, research/sources/jenson-36-points.md]
code: [W, H, STEPS_PER_FRAME, FED_EVERY, INOCULUM, WILD_START]
---

# The mould at scale (outside models and the GPU plan)

## Now

Today's dish is 256 x 160 cells (`W`, `H`) with 400-2,200 walkers, at most
one per cell, stepped `STEPS_PER_FRAME` 2 times a frame on one CPU core.
Tubes are about 5% of the dish wide, so it holds a few dozen loops at most.
Sage Jenson runs the same Jones 2010 rules with 5-10 million GPU particles
and no one-per-cell limit, and their network_2 clip grows from 3-5 loops to
about 180-340 in nine seconds. Scale, not a better rule, is most of the gap,
and the owner wants it for interesting play, not realism.

**Nothing is built.** The plan, revised after an independent review on
2026-11-10, has six gated phases:

1. CPU foundations that test the port's riskiest assumptions at today's scale.
2. An offline 1024 x 640 dish with about 100k walkers, judged by paired seeds
   and pictures, plus a GPU timing test on the owner's machine.
3. A GPU sandbox.
4. Game rules on the GPU.
5. Porting the dishes that survive a dense network.
6. New play that only scale allows.

Three facts bind the design:

- The census feeds the economy every step (`FED_EVERY` 6), so its fed and
  piece labels must go back up to the GPU.
- A finer grid alone slows the mould and shortens its smell in dish terms,
  so phase 1 picks a scaling rule.
- Cloud sessions have no GPU, so an all-integer simulation is recommended,
  with the same bits on CPU, GPU and CI.

**Contagion** (Jenson) is a mark that spreads from particle to particle
through the trail. It is planned as an off-by-default rule in phase 1 and a
tool in phase 6. Every outside source has a card in
[`research/sources/`](../../research/sources/README.md).

## Numbers

| Measure | Result | Seeds | Paired | Source |
|---|---|---|---|---|
| Jenson network_2, enclosed loops, first to last frame | 3-5 to 180-340 (two brightness cut-offs, 110 and 150) | one clip | | [jenson, *Pictures*](../../Reports/jenson-physarum-2026-11-10.md) |
| Particles, Jenson vs ours | 5-10 million vs 400-2,200 walkers | | | [jenson, *How our mould compares*](../../Reports/jenson-physarum-2026-11-10.md) |
| Frame today, mean | 4.93 ms: simulation 1.10, drawing 3.83; worst 14.94 | plan hand, seed 1 | | [gpu plan, *Where we start*](../../Reports/gpu-plan-2026-11-10.md) |
| Step cost, bare move and diffusion, one core | 256 x 160 / 2.2k 0.13 ms; 1024 x 640 / 100k 9.5-11.2 ms; 1024 x 640 / 1M 96-98 ms; 2048 x 1280 / 1M 111-112 ms | 2 runs | | [gpu plan, *What the independent review changed*](../../Reports/gpu-plan-2026-11-10.md), [bench](../../Reports/gpu-plan-2026-11-10/step-bench.rs) |

## Owner's word

- 2026-11-10 17:01Z: "our goal is not a realistic slime mold. It is interesting simulations that we can turn into a game." (this subject's thread)
- 2026-11-10 17:13Z: asked for "a comprehensive plan forward, including GPU development, not just the 5 cheapest tests".
- 2026-11-10 17:24Z: "I like this plan, but run an independent review agent first." The review ran and its findings are in the plan; no phase is approved to start yet.

## Tried and rejected

- The lamp as a steering term (`Params::light_weight`) cut the tube it was meant to steer, so a GPU port keeps the push. See the `light_weight` entry in [`dead-ends.md`](../../Reports/dead-ends.md).
- A shorter sensor reach (`Params::sensor_offset` 9 to 6-8) broke planned hands by smelling food from less far. A finer grid with no scaling rule would repeat that. Same file.
- 2048 x 1280 for the offline look in phase 2 was dropped as too slow to sweep: about 110 ms a step on one core ([plan](../../Reports/gpu-plan-2026-11-10.md)).

## Open

- The owner's decisions are in [HANDOFF.md](../../HANDOFF.md): whether to start phases 1 and 2, which machine and GPU, whether to go all-integer, uploading Jones 2011, and holding the Pixel_Physics renderer port.
- The Bleuje, Lague and PolyPhy code is still unread (phase 3). Only PolyPhy's MIT code may be reused.

## Sources

- [jenson-physarum-2026-11-10.md](../../Reports/jenson-physarum-2026-11-10.md), with the owner's PDF and GIFs in [its folder](../../Reports/jenson-physarum-2026-11-10/).
- [gpu-plan-2026-11-10.md](../../Reports/gpu-plan-2026-11-10.md) and its [independent review](../../Reports/gpu-plan-2026-11-10/review.md).
- Source cards: [index](../../research/sources/README.md); [Jenson's post](../../research/sources/jenson-physarum-post.md), [36 Points](../../research/sources/jenson-36-points.md), [Jones 2010](../../research/sources/jones-2010.md), [MCPM](../../research/sources/mcpm-2022.md).
