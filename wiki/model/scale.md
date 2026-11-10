---
title: The mould at scale (outside models and the GPU plan)
kind: mechanism
status: open
updated: 2026-11-10
sources: [Reports/jenson-physarum-2026-11-10.md, Reports/gpu-plan-2026-11-10.md, research/sources/jenson-physarum-post.md, research/sources/jones-2010.md, research/sources/mcpm-2022.md, research/sources/jenson-36-points.md]
code: [W, H, STEPS_PER_FRAME, FED_EVERY, INOCULUM, WILD_START, Dish::step_cost, STEP_PASSES, Params::census_every, Params::census_lag, Params::all_at_once, Params::fixed_point, keyed]
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

**Phase 1 has started; nothing on the GPU is built.** The plan, revised after an independent review on
2026-11-10, has six gated phases:

1. CPU foundations that test the port's riskiest assumptions at today's scale.
2. An offline 1024 x 640 dish with about 100k walkers, judged by paired seeds
   and pictures, plus a GPU timing test on the owner's machine.
3. A GPU sandbox.
4. Game rules on the GPU.
5. Porting the dishes that survive a dense network.
6. New play that only scale allows.

Today, diffusing trail and scent is 62% of the simulation's time and moving the walkers 24% (`Dish::step_cost`, printed by `mouldprobe`).

Phase 1's three arms are `Params` switches, all off by default, measured on
every scored dish over 16 paired seeds:

- **A late census is safe.** `census_every` 30 and `census_lag` 4 (labels 4
  times a second, two frames late) keep every ship bar within one seed.
- **Fixed point changes play.** `fixed_point` rounds trail and scent to
  1/65,536. In floats they never reach zero: after 200 steps half the dish
  holds trail or scent below that step. Walkers steer by it. Rounded away,
  walkers past the network sense nothing, go straight, and random play
  reaches the food behind the wall on 5 seeds of 16. So the GPU keeps trail
  and scent as `f32`.
- **Moving all at once cuts fights where two moulds meet.** `all_at_once`
  (sense before anyone deposits, enter only cells empty at the start,
  claims by `keyed` draws) is how a GPU moves walkers; on The Wild One the
  warden's share falls on 15 of 16 seeds. A second claim round is the knob
  if more fighting is wanted.

The owner (2026-11-10 20:55Z) ruled today's dishes unlikely to be the true
games, so from here GPU changes are judged on engine measures on a neutral
dish (network shape, spread speed, strays, seam fights), not ship bars.

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
| Today's step by pass, plan hand, free dish | diffuse 0.69-0.70 ms a frame (62%), move 0.27 (24%), census 0.05, economy 0.05, light 0.05; all 1.11-1.14 | seed 1, 2 runs | | [gpu plan, *Phase 1 progress*](../../Reports/gpu-plan-2026-11-10.md) |
| Late census (4 a second, two frames late) vs today | ship bar met Wall/Ruler/Dry 15/15/15 vs 15/16/15; no hand moves after Bonferroni (27 tests) | 16 | yes | [gpu plan, *three GPU arms*](../../Reports/gpu-plan-2026-11-10.md) |
| Fixed-point trail, scent, positions, headings vs floats | ship bar 11/12/14 vs 15/16/15; random reaches the walled food on 5 seeds (15 stars vs 1); after 200 steps 15,247 of 29,899 agar cells hold trail below 1/65,536 in floats | 16; trace seed 8 | yes | same |
| All walkers at once vs one at a time | The Wild One, warden's share 0.648 to 0.594 (medians), worse on 15 of 16, p 0.0005; seam fights down on 13-15 of 16 for every hand | 16 | yes | same |
| Frame cost of the arms, ms a frame | today 1.08-1.09; late 1.05-1.08; all at once 1.10-1.13; fixed point 1.73 (emulation) | seed 1, 2 runs | | same |
| Step cost, bare move and diffusion, one core | 256 x 160 / 2.2k 0.13 ms; 1024 x 640 / 100k 9.5-11.2 ms; 1024 x 640 / 1M 96-98 ms; 2048 x 1280 / 1M 111-112 ms | 2 runs | | [gpu plan, *What the independent review changed*](../../Reports/gpu-plan-2026-11-10.md), [bench](../../Reports/gpu-plan-2026-11-10/step-bench.rs) |

## Owner's word

- 2026-11-10 17:01Z: "our goal is not a realistic slime mold. It is interesting simulations that we can turn into a game." (this subject's thread)
- 2026-11-10 17:13Z: asked for "a comprehensive plan forward, including GPU development, not just the 5 cheapest tests".
- 2026-11-10 17:24Z: "I like this plan, but run an independent review agent first." The review ran and its findings are in the plan.
- 2026-11-10 17:50Z: "How many threads do we need to start your plan or do you want to just proceed with the implementations yourself?" Phase 1 started in the plan's thread; the Pixel_Physics review thread builds the dish hash.
- 2026-11-10 17:54Z: "my home machine has a geforce 1070", which is not the machine the owner usually works at, so GPU runs wait for them to be at it.
- 2026-11-10 20:55Z: "I just think The Wild One and other current demos are unlikely to be the true games. So I don't want us to be wasting effort or get too far away from your original task which was not really about the game part of this project. (we are now exploring RTS type games)"

## Tried and rejected

- The lamp as a steering term (`Params::light_weight`) cut the tube it was meant to steer, so a GPU port keeps the push. See the `light_weight` entry in [`dead-ends.md`](../../Reports/dead-ends.md).
- A shorter sensor reach (`Params::sensor_offset` 9 to 6-8) broke planned hands by smelling food from less far. A finer grid with no scaling rule would repeat that. Same file.
- 2048 x 1280 for the offline look in phase 2 was dropped as too slow to sweep: about 110 ms a step on one core ([plan](../../Reports/gpu-plan-2026-11-10.md)).
- Fixed-point trail and scent at 1/65,536 (`Params::fixed_point`), the plan's all-integer recommendation, changed play: see the `fixed_point` entry in [`dead-ends.md`](../../Reports/dead-ends.md).

## Open

- The owner's decisions are in [HANDOFF.md](../../HANDOFF.md): floats or integers (measured; floats recommended for trail and scent), uploading Jones 2011, and holding the Pixel_Physics renderer port.
- Whether the invisible far smell is good for play is open: the owner's rule on showing the simulation argues against an invisible mechanism, but today's dishes are tuned on it.
- What in the all-at-once move cuts the seam fights, measured on a neutral two-mould dish.
- The Bleuje, Lague and PolyPhy code is still unread (phase 3). Only PolyPhy's MIT code may be reused.

## Sources

- [jenson-physarum-2026-11-10.md](../../Reports/jenson-physarum-2026-11-10.md), with the owner's PDF and GIFs in [its folder](../../Reports/jenson-physarum-2026-11-10/).
- [gpu-plan-2026-11-10.md](../../Reports/gpu-plan-2026-11-10.md) and its [independent review](../../Reports/gpu-plan-2026-11-10/review.md).
- Source cards: [index](../../research/sources/README.md); [Jenson's post](../../research/sources/jenson-physarum-post.md), [36 Points](../../research/sources/jenson-36-points.md), [Jones 2010](../../research/sources/jones-2010.md), [MCPM](../../research/sources/mcpm-2022.md).
