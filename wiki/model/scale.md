---
title: The mould at scale (outside models and the GPU plan)
kind: mechanism
status: open
updated: 2026-11-11
sources: [Reports/larder-world-roadmap-2026-11-11.md, Reports/jenson-physarum-2026-11-10.md, Reports/jenson-compare-2026-11-10.md, Reports/gpu-sandbox-2026-11-11.md, Reports/gpu-plan-2026-11-10.md, research/sources/jenson-physarum-post.md, research/sources/jones-2010.md, research/sources/mcpm-2022.md, research/sources/jenson-36-points.md, research/sources/bleuje-interactive-physarum.md, research/sources/lague-slime-simulation.md, research/sources/polyphy.md, Reports/jenson-looks-2026-11-11.md, Reports/outside-rules-2026-11-11.md, research/sources/primordia.md, research/sources/reginato-2024.md, Reports/creatures-2026-11-11.md]
code: [W, H, STEPS_PER_FRAME, FED_EVERY, INOCULUM, WILD_START, Dish::step_cost, STEP_PASSES, Params::census_every, Params::census_lag, Params::all_at_once, Params::fixed_point, keyed, BigDish, BigParams, BigMeasure, BigParams::respawn, BigParams::blob_r, BigParams::haze, BigParams::sense_scale, BigParams::deposit_cap, BigParams::sense_floor, BigParams::wander, BigParams::sick_steps, BigDish::infect, BigParams::sensor_r, BigParams::species, BigParams::species_cycle, BigParams::chance_exp, BigParams::explore_steps, BigParams::explore_spread, BigParams::size_spread, BigParams::blocked_keep, BigParams::blocked_retry, BigParams::blocked_deposit, BigParams::graze_weight, BigParams::species_convert, BigParams::species_boost, BigParams::crystal_charge, BigParams::crystal_regrow, BigParams::terrain_rock, BigParams::rect, BigDish::build_wall]
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

**Phases 1 and 2 have run, the step runs on the owner's GPU, and phase 3's sandbox is built.** The plan, revised after an independent review on
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

**The scaling rule and the big dish** (`BigDish` in `src/mould/big.rs`: the
engine alone at any size, with the GPU's move and float fields, no economy;
`examples/bigdish.rs`). On 1024 x 640 with 32,000 walkers, 16 paired seeds:

- **Chosen rule:** today's numbers counted in fine cells, one fine step a
  tick, food smelled at today's reach (`BigParams::scent_offset` 36 fine
  cells) while trail is sensed 9 fine cells out. From a spread start it
  makes 64.5 loops at 20 s against today's 3, tubes 3.5 of today's cells
  wide, 2.2% strays; 11 ms a tick on one core.
- **The sensor offset sets the mesh size**; scent smelled far costs nothing
  and cuts strays on every seed.
- **Copying every number in today's units gives no network** (0 loops on
  16 of 16): today's loops come from its one-walker-a-cell grain.
- **More steps a frame is rejected** (3.7x the cost, fewer loops, twice the
  strays).
- **A packed home blob stays a bag** on the fine grid (3 piles at 20 s
  against today's 4); the economy's small start may avoid it.
- At 100,000 walkers the net has 164 loops at 10 s and coarsens to 92 by 30 s.

**The GPU timing test is built** (`examples/gputime.rs`): the same step as
four wgpu compute passes a tick, with the CPU's keyed draws and claims by
`atomicMin`. On a software GPU in the cloud its dish matches the CPU's
measures (100k walkers at 10 s: 165 loops against 163, width 4.83 against
4.96; one seed, and the CPU's own count moves about as much from one tick
to the next). **On the owner's GTX 1070** (2026-11-11), a 60 Hz frame of two ticks,
submitted one frame at a time, takes 0.82 ms at 32,000 walkers, 1.37 ms at
100,000 and 3.52 ms at 1,000,000 (no crowd limit), against the phase 3 bar
of 8 ms; the worst 1M frame was about 12 ms. The GPU dish matches the CPU's
at 20 s (100k: 115 loops against 114) and runs 37 times as fast. Drawing,
the census and the economy are not in those times.

**The GPU sandbox** (phase 3, 2026-11-11; `cargo run --release --bin
mouldgpu`) runs that step live in a window, drawn on the GPU, with every
option the Jenson comparison tested as a control (walkers to 8 million,
the one-a-cell limit, angles, reach, step, trail laid and kept, respawn,
food, colour, trail or walker counts, the game's or a soft curve) and the
comparison's dishes on keys 1-8 (the owner's ask, 00:06Z). The left mouse
drops walkers under it (0.2% of all walkers a tick, taken from random
places), the right wipes trail, F drops food. On a software
GPU and on the GTX 1070 its dishes match the CPU's spread at 10 s on 8
seeds (16 for the food dish): no measure differs after Bonferroni over 20
tests a GPU, once an independent review (11-11) had found and fixed two
gaps, fed scent and jumps after the step. A seed is a different dish on
each GPU, since their `f32` sums round differently, but gives the same frame
twice on one. A twin test in CI checks one GPU tick from a CPU dish's state
against one CPU tick on software Vulkan: to the bit on keys 1, 3, 4 and 5.
On the GTX 1070 a frame of two ticks and the drawing takes 1.45 ms of GPU
time at 100,000 walkers, 8.1-10 ms at 1,000,000 and 12.8 ms at 4,000,000
with our numbers (30 ms with Jenson's, whose far sensors double the sense
pass); the drawing is 0.07-0.29 ms of it. In the vsynced window at light
load the GPU steps its clocks down, so the window reads slower than
headless ([report, *Independent review*](../../Reports/gpu-sandbox-2026-11-11.md),
[page](https://claude.ai/artifact/1rdrxadVcNdxTmbxp2KJGy)).
The looks of the [jenson looks](../../Reports/jenson-looks-2026-11-11.md)
report (disc and haze start, trail-set settings, square-root deposit cap,
sensing floor, wander, contagion) are sandbox controls too (11-11), on
two panel pages and keys 9, 0 and Shift+1-5; a twin test matches them to
the bit on six dishes. Off, they cost 0.01 ms a frame on the GTX 1070;
contagion costs 0.3 ms at 100,000 walkers, and the contagion presets
take 9.5-9.8 ms of GPU ticks a frame at a million ([report, *The looks as
controls*](../../Reports/gpu-sandbox-2026-11-11.md)). (Corrected 11-11: the
looks' trail matched to the bit on four of the six dishes; on the two with
the deposit cap it is within 1.1e-7, from the order the shares are summed.)
The rules of the [outside rules](../../Reports/outside-rules-2026-11-11.md)
and [creatures](../../Reports/creatures-2026-11-11.md) reports are sandbox
controls too (11-11): species with bands, cyclic species and conversion,
explorers, blocked walkers that lay and keep their heading, the chance
turn, size spread, area sensors, grazing and species 0's boost, on a fourth panel page and
keys Shift+6-0 and Ctrl+1-2 (territories, ribbons, explorer front,
droplets, beads, grazing droplets, grazing species). A third twin test
matches them on fifteen dishes (every walker, age and species; fields
within 2.4e-8). `blocked_retry` is not ported. Compiled in but off they
cost 18-24% on the software GPU, so the sandbox compiles them out when off
([report, *The outside rules as controls*](../../Reports/gpu-sandbox-2026-11-11.md)).

Three facts bind the design:

- The census feeds the economy every step (`FED_EVERY` 6), so its fed and
  piece labels must go back up to the GPU.
- A finer grid changes the mesh and the smell range in dish terms; the
  chosen scaling rule keeps the fine mesh and today's smell reach.
- Cloud sessions have no GPU. CI runs the GPU step on Mesa's software
  Vulkan; with the fields `f32` (fixed point changed play, below), the twin
  test is one tick from a CPU state, not the same bits over a run.

**Why Jenson's clips look different** (compared on the big dish,
2026-11-10, no food, spread start). Jenson runs the same step; the look
comes from how it is run and drawn. Dropping the one-walker-a-cell limit
with our numbers folds the walkers into a few ropes (0 loops at 10 s against
151.5, 8 of 8 seeds), so Jenson's rule needs other settings; sensor 45,
turn 22.5, sensors 18 out gave a polygon mesh. Half the look is the
brightness curve: the game's saturates at a trail of about 15, and the same
dish drawn with a soft curve or by walkers a cell shows veins as thin as
Jenson's. Respawning 1% of walkers a step (`BigParams::respawn`, off by
default; Jenson's later work respawns every 1,000 steps) held a fine mesh
and gave Jenson's haze. Four times the walkers changed little. Every GPU
port read (Bleuje, Lague, PolyPhy, gpu-io, Barradeau) drops the limit.
A follow-up ([jenson looks](../../Reports/jenson-looks-2026-11-11.md))
added off-by-default rules for the three looks still missing. **Trunks**
come from the start: a dense centred disc (`BigParams::blob_r`, `haze`)
keeps its edge as a thick ring inside a mesh at 20 s, while respawn stays
slow (0.001). **Contagion** is built on the big dish (`sick_steps` and the
other `sick_*` fields, `BigDish::infect`, the same rule as Two Fevers): it
runs along the tubes in 2-4 s, coarsens the mesh while it passes when sick
walkers steer differently, leaves no mark by 20 s, and a knife cut does not
stop it. **Feathery fronts** are still not reproduced: with Jenson's rule any
haze organises on its own. 36 Points' trail-dependent settings
(`sense_scale`) gave long curves, a ring that expands across the dish, or a
lasting round trunk; Bleuje's square-root deposit (`deposit_cap`) gave even
polygons at once. Every outside source has a card in
[`research/sources/`](../../research/sources/README.md).

**World fields for the Larder World (W1a, 2026-11-11, [roadmap](../../Reports/larder-world-roadmap-2026-11-11.md)).** These are on the CPU big dish and off by default:
- `crystal_charge`, `crystal_eat` and `crystal_regrow`: finite charge in each pile cell. The walkers on a cell take equal shares. Pile scent is multiplied by the share of charge left, and the picture's glow is the charge.
- `terrain_rock` and `terrain_wet`: keyed value noise, the same map at every scale. Rock is not agar, and wet ground halves the step.
- `rect`: a rectangular world.
- `BigDish::build_wall`: a player wall that leaves the agar, so walkers, trail and scent all stay out.

At 512 × 320 the cost is within noise. A first wall test showed no effect while the walker count is fixed (road wall against none, 5 of 8 seeds, p 0.73). It waits on growth (W0).

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
| Scaling rule, 1024 x 640 / 32k walkers vs today, spread start, 20 s (medians) | loops: today 3, naive 52.5, steps 15.5, smell 11, reach 12, turn 0, nose 64.5; strays: today 0.7%, nose 2.2%, reach 39% | 16 | yes | [gpu plan, *the scaling rule*](../../Reports/gpu-plan-2026-11-10.md) |
| Scaling rule, piles reached at 20 s from a packed blob (medians) | today 4; naive 2.5; steps 2.5; smell 3; reach 4; turn 3; nose 3 (lower than today on 15 of 16) | 16 | yes | same |
| Big dish, nose rule, seed 1 | 32k walkers: 94 loops at 10 s, 59 at 30 s; 100k: 164 and 92; today 3 | 1 | | same, [page](https://claude.ai/artifact/WMfUwTj9Dfdwp9MCfy8MHs) |
| GPU step vs CPU step, same seed, 10 s, software Vulkan (llvmpipe) | 32k: loops 90 / 92, width 3.40 / 3.41, strays 0.9% / 1.0%; 100k: loops 165 / 163, width 4.83 / 4.96, strays 1.8% / 1.5% | seed 1 | | [gpu plan, *GPU timing test*](../../Reports/gpu-plan-2026-11-10.md) |
| GPU step on the GTX 1070, nose rule, seed 1: ms a tick (median), 120 ticks a submit / 2 a submit | 32k 0.374 / 0.410; 100k 0.631 / 0.685; 1M, crowd 0, 1.537 / 1.760 (worst 6.09); frame of two ticks 0.82, 1.37, 3.52 ms | seed 1 | | [gpu plan, *GPU timing on the GTX 1070*](../../Reports/gpu-plan-2026-11-10.md) |
| GPU dish vs CPU dish on the GTX 1070, 20 s, seed 1 | 32k: loops 66 / 68, width 3.48 / 3.48, strays 2.3% / 1.8%; 100k: loops 115 / 114, width 5.08 / 5.20, strays 2.0% / 1.9%; CPU 9.98 and 23.63 ms a tick on four threads | seed 1 | | same |
| GPU sandbox vs CPU dish, software Vulkan, 10 s, seed 1, no food, before the review's fixes: loops sandbox / CPU | one a cell 148 / 151; no limit 0 / 0; 1M Jenson settings 43 / 42; with respawn 1% 200 / 200 (width 8.33 / 7.99) | seed 1 | | [gpu sandbox](../../Reports/gpu-sandbox-2026-11-11.md) |
| GPU sandbox on the GTX 1070, a frame of two ticks and the drawing (median), seed 1, before the review's fixes (window figure a running average; "busy 1.4" is wall time, GPU 1.30) | 32k 0.88 ms; 100k 1.42 (4.6 in the vsynced window); 100k no limit 0.99; 1M Jenson settings 8.00; with respawn 9.84; 4M 30.07 (37 frames a second); drawing 0.06-0.18 | seed 1 | | [gpu sandbox](../../Reports/gpu-sandbox-2026-11-11.md) |
| GPU sandbox vs CPU dish after the review's fixes, 10 s, spread start, software Vulkan and GTX 1070 | no measure (loops, width, tube, strays on five dishes) differs after Bonferroni over 20 a GPU; smallest p 0.013 (1070, 32k with food, more loops on 12 of 16); 1M respawn width CPU 7.70-8.01, software 7.83-8.00, 1070 7.82-8.12; 32k strays 0.8-1.8%, 0.5-1.4%, 0.8-1.7% | 8 (16 for 32k) | by seed | [gpu sandbox, *Independent review*](../../Reports/gpu-sandbox-2026-11-11.md) |
| GPU sandbox on the GTX 1070 after the fixes, GPU ms a frame of two ticks + drawing, headless, seed 1 | 32k 0.82 + 0.07; 100k 1.39 + 0.07; 100k no limit 0.91 + 0.06; 1M Jenson settings 7.85 + 0.29; with respawn 9.68 + 0.27; 4M 29.79 + 0.29; 4M ours 12.57 + 0.28; sense pass a tick 2.51 ms at 1M (Jenson) against 3.72 at 4M (ours) | seed 1 | | same |
| GPU sandbox looks on the GTX 1070, GPU ms a frame of two ticks + drawing, headless, seed 1 | looks off: key 1 1.385-1.394 against main 1.378-1.382 (3 rounds), key 4 no change; key 1 + contagion 1.69 + 0.08 (trail pass 0.13 -> 0.22 a tick); key 1 + trail-set 1.45; + floor and wander 1.40; at 1M: trunk ring 5.81, contagion wave 9.79, sweeping ring 5.99, long curves 3.86, even polygons 7.58, endemic 9.69, sick ropes 9.45, drawing 0.26-0.29 | seed 1 | | [gpu sandbox, *The looks as controls*](../../Reports/gpu-sandbox-2026-11-11.md) |
| GPU sandbox outside rules, one GPU tick against one CPU tick, software Vulkan, 15 dishes | every walker, age and species agree; grass exact; fields exact on 11, else trail L1 1.4e-8 to 2.4e-8, species L1 1.3e-13 to 1.8e-8; 16 planted bugs all fail it | one tick | | [gpu sandbox, *The outside rules as controls*](../../Reports/gpu-sandbox-2026-11-11.md) |
| GPU sandbox outside rules off, software Vulkan (not the 1070), GPU ms of ticks a frame, 3 rounds | compiled in: key 1 32.4-32.6 against main 26.1-26.8, key 4 88.2-90.3 against 74.4-76.3; compiled out when off: key 1 25.7-26.5 against 25.2-25.7, key 4 72.9-76.5 against 74.1-76.2 | seed 1 | | [gpu sandbox, *The outside rules as controls*](../../Reports/gpu-sandbox-2026-11-11.md) |
| GPU clocks in the vsynced window, 100k, GTX 1070 | window median 1.80 + 0.08 ms with the clocks stepping 1594 to 797 MHz (memory 810 MHz) against headless 1.39 + 0.07 at 1936 MHz; 8 ticks a frame or 1M walkers hold 1936 MHz | seed 1 | | same |
| Big dish cost, ms a tick, one thread / four | today 0.54 / 0.53; nose 32k 11.3 / 6.7; nose 100k 21.6 / 20.2; steps 43.1 / 25.8; turn 71.1 / 31.7 | seed 1 | | same |
| No one-a-cell limit vs the limit, 100k walkers, no food, spread start, loops at 10 s | 0 (0-2) vs 151.5 (144-160): fewer on 8 of 8, p 0.008, survives Bonferroni over 4 | 8 | yes | [jenson compare, *The tests*](../../Reports/jenson-compare-2026-11-10.md) |
| Respawn 1% a step vs none, 1M walkers, no limit, sensor 45 / turn 22.5 / reach 18, loops at 10 s | 196 (187-208) vs 43.5 (38-52): more on 8 of 8, p 0.008, survives Bonferroni over 4; 166 vs 160 ms a tick on one core | 8 | yes | same |
| Tube width and haze on the picture, Jenson network_2 last frame vs the 1M dish drawn three ways | Jenson 0.3-0.4% of the disc, haze 41%; game curve 1.4-1.6%, haze 8%; soft curve 0.4-0.6%, 15% (44% with respawn); walkers a cell 0.1% | one clip; seed 1 | | same, [page](https://claude.ai/artifact/HL7jZ4qGhr6f1HpUDmNEj2) |
| Trunk: dense disc + 30% haze vs even start, 1M walkers, respawn 0.001, step x (1 + trail/50); share of the brightest 1% of trail in the first ring's band at 20 s | 0.70 (0.20-0.86) vs 0.15 (0.09-0.31): more on 8 of 8, p 0.008, survives Bonferroni over 3 | 8 | yes | [jenson looks, *The tests*](../../Reports/jenson-looks-2026-11-11.md) |
| Contagion with rope steering (sick 2 s) vs none, loops at 9 s / 20 s | 41-51 vs 48-58, fewer on 8 of 8, p 0.008 / 15-25 vs 17-25, p 0.45 | 8 | yes | same |
| Knife cut across the dish as the sickness starts vs none, far side ever sick at 6 s | 0.155-0.254 vs 0.157-0.260; 4 of 8, p 1 | 8 | yes | same |
| Chance-weighted turn (MCPM, e 4) vs Jones's turn, 1M walkers, loops at 10 s | 66.5 (60-73) vs 44 (38-52): more on 8 of 8, p 0.008, survives Bonferroni over 5; about +15% cost | 8 | yes | [outside rules, *The tests*](../../Reports/outside-rules-2026-11-11.md) |
| Area sensors 3 x 3 (Lague) vs one cell, loops at 10 s | 38 (32-46) vs 43.5 (38-52): fewer on 8 of 8, p 0.008; 290 vs 190 ms a tick (+50%) | 8 | yes | same |
| Two species in their own halves, avoidance 1 vs 0, species 0's trail in its half at 20 s / tube cells 80%+ one species | 0.89-1.00 vs 0.88-1.00, 5 of 8, p 0.73 / 0.994-1.000 vs 0.966-0.983, cleaner on 8 of 8, p 0.008 | 8 | yes | same |
| Cyclic vs plain three species, tube cells changing lead in 2 s at 10 s / loops | 0.65-0.70 vs 0.75-0.85, lower on 8 of 8 (the guess was higher) / 97-125 vs 22-63 | 8 | yes | same |
| Packed walkers laying trail where they stand (30k, one a cell, SO 9), bodies and drift | about 180 droplets of about 200 cells, drift 0.6-0.7 cells a second | seed 1 | | same |
| Grazing weight 10 vs 0 on those droplets, body drift over the last 2 s of 10 s | 6.5-7.6 vs 0.67-0.77 cells: more on 8 of 8, p 0.008; bodies 168-190 vs 179-193; 12-15 ms a tick either way | 8 | yes | [creatures](../../Reports/creatures-2026-11-11.md) |
| Fight edge: species 0 lays 1.5x vs even, conversion 1.2, two halves of the 1M network, species 0's walker share at 16 s | 0.474 (0.448-0.518) vs 0.500 (0.450-0.580): lower on 6 of 8, p 0.29; higher at 8 s on 6 of 8 | 8 | yes | [creatures, *An edge*](../../Reports/creatures-2026-11-11.md) |
| Cost of the new rules, 1M walkers, one core, ms a tick | base 132.7; trail-dependent step 176.5; square-root deposit 150.9; floor 139.6; wander 142.5; contagion 142.9 | seed 1 | | same |
| Step cost, bare move and diffusion, one core | 256 x 160 / 2.2k 0.13 ms; 1024 x 640 / 100k 9.5-11.2 ms; 1024 x 640 / 1M 96-98 ms; 2048 x 1280 / 1M 111-112 ms | 2 runs | | [gpu plan, *What the independent review changed*](../../Reports/gpu-plan-2026-11-10.md), [bench](../../Reports/gpu-plan-2026-11-10/step-bench.rs) |

## Owner's word

- 2026-11-10 17:01Z: "our goal is not a realistic slime mold. It is interesting simulations that we can turn into a game." (this subject's thread)
- 2026-11-10 17:13Z: asked for "a comprehensive plan forward, including GPU development, not just the 5 cheapest tests".
- 2026-11-10 17:24Z: "I like this plan, but run an independent review agent first." The review ran and its findings are in the plan.
- 2026-11-10 17:50Z: "How many threads do we need to start your plan or do you want to just proceed with the implementations yourself?" Phase 1 started in the plan's thread; the Pixel_Physics review thread builds the dish hash.
- 2026-11-10 17:54Z: "my home machine has a geforce 1070", which is not the machine the owner usually works at, so GPU runs wait for them to be at it.
- 2026-11-10 22:32Z, on a card after the 4x Mould page: the GPU mould aims for "100,000 walkers" (over 32,000, or stopping the GPU work).
- 2026-11-10 20:55Z: "I just think The Wild One and other current demos are unlikely to be the true games. So I don't want us to be wasting effort or get too far away from your original task which was not really about the game part of this project. (we are now exploring RTS type games)"
- 2026-11-11 00:06Z: "you can use my local PC for as many tests as you want", and "can we take all the different things you tested for the jenson comparison as options that I can play with in a sandbox".

## Tried and rejected

- The lamp as a steering term (`Params::light_weight`) cut the tube it was meant to steer, so a GPU port keeps the push. See the `light_weight` entry in [`dead-ends.md`](../../Reports/dead-ends.md).
- A shorter sensor reach (`Params::sensor_offset` 9 to 6-8) broke planned hands by smelling food from less far. A finer grid with no scaling rule would repeat that. Same file.
- 2048 x 1280 for the offline look in phase 2 was dropped as too slow to sweep: about 110 ms a step on one core ([plan](../../Reports/gpu-plan-2026-11-10.md)).
- Fixed-point trail and scent at 1/65,536 (`Params::fixed_point`), the plan's all-integer recommendation, changed play: see the `fixed_point` entry in [`dead-ends.md`](../../Reports/dead-ends.md).
- More fine steps a tick (`BigParams::substeps`) and copying every number in today's units (preset `turn`) as the scaling rule: both in [`dead-ends.md`](../../Reports/dead-ends.md).

## Open

- The owner's decisions are in [HANDOFF.md](../../HANDOFF.md): floats or integers (measured; floats recommended for trail and scent), uploading Jones 2011, and holding the Pixel_Physics renderer port.
- Whether the invisible far smell is good for play is open: the owner's rule on showing the simulation argues against an invisible mechanism, but today's dishes are tuned on it.
- What in the all-at-once move cuts the seam fights, measured on a neutral two-mould dish.
- The bag: whether a packed start on the fine grid still stalls once the economy grows the mould from a small inoculum.
- Census readback and the economy on the GPU are not yet timed; the drawing is (0.07-0.29 ms a frame on the GTX 1070, sandbox).
- Jenson's feathery fronts: seven mechanisms failed ([jenson looks](../../Reports/jenson-looks-2026-11-11.md)); explorers that turn into networkers one by one (`explore_spread`) grow radial feathery veins from a disc, one seed so far ([outside rules](../../Reports/outside-rules-2026-11-11.md)).
- A creature that moves as a body: still droplets crawl once they graze a regrowing field ([creatures](../../Reports/creatures-2026-11-11.md)). What lets one body beat another is open: conversion alone moved no border on even dishes.
- Whether contagion can leave a lasting mark for play (see the contagion entry in [`dead-ends.md`](../../Reports/dead-ends.md)). The outside code was read on 2026-11-10; only PolyPhy, gpu-io (MIT) and Barradeau (public domain) may be reused.

## Sources

- [jenson-physarum-2026-11-10.md](../../Reports/jenson-physarum-2026-11-10.md), with the owner's PDF and GIFs in [its folder](../../Reports/jenson-physarum-2026-11-10/).
- [jenson-compare-2026-11-10.md](../../Reports/jenson-compare-2026-11-10.md): the clips against the big dish, and how the outside versions build the step.
- [gpu-sandbox-2026-11-11.md](../../Reports/gpu-sandbox-2026-11-11.md): the GPU sandbox, its controls and presets, and its check against the CPU dish.
- [jenson-looks-2026-11-11.md](../../Reports/jenson-looks-2026-11-11.md): trunks, contagion and fronts on the big dish; [page](https://claude.ai/artifact/DqJxgjQGVgnwQNwbbzoGrg).
- [creatures-2026-11-11.md](../../Reports/creatures-2026-11-11.md): grazing droplets that crawl, two species of cells, conversion.
- [outside-rules-2026-11-11.md](../../Reports/outside-rules-2026-11-11.md): outside rules judged as creatures (species, explorers, droplets) and a survey of 14 more open-source simulations.
- [gpu-plan-2026-11-10.md](../../Reports/gpu-plan-2026-11-10.md) and its [independent review](../../Reports/gpu-plan-2026-11-10/review.md).
- Source cards: [index](../../research/sources/README.md); [Jenson's post](../../research/sources/jenson-physarum-post.md), [36 Points](../../research/sources/jenson-36-points.md), [Jones 2010](../../research/sources/jones-2010.md), [MCPM](../../research/sources/mcpm-2022.md), [Bleuje](../../research/sources/bleuje-interactive-physarum.md), [Lague](../../research/sources/lague-slime-simulation.md), [PolyPhy](../../research/sources/polyphy.md), [Primordia](../../research/sources/primordia.md), [Reginato 2024](../../research/sources/reginato-2024.md).
