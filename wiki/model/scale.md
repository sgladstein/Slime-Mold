---
title: The mould at scale (the GPU plan)
kind: mechanism
status: open
updated: 2026-11-12T17:26Z
sources: [Reports/gpu-plan-2026-11-10.md, Reports/larder-world-roadmap-2026-11-11.md, Reports/slime-enemy-review-2026-11-12.md]
code: [W, H, STEPS_PER_FRAME, FED_EVERY, INOCULUM, WILD_START, Dish::step_cost, STEP_PASSES, Params::census_every, Params::census_lag, Params::all_at_once, Params::fixed_point, keyed, BigDish, BigParams, BigMeasure, BigParams::respawn, BigParams::blob_r, BigParams::haze, BigParams::sense_scale, BigParams::deposit_cap, BigParams::sense_floor, BigParams::wander, BigParams::sick_steps, BigDish::infect, BigParams::sensor_r, BigParams::species, BigParams::species_cycle, BigParams::chance_exp, BigParams::explore_steps, BigParams::explore_spread, BigParams::size_spread, BigParams::blocked_keep, BigParams::blocked_retry, BigParams::blocked_deposit, BigParams::graze_weight, BigParams::species_convert, BigParams::species_boost, BigParams::crystal_charge, BigParams::crystal_regrow, BigParams::terrain_rock, BigParams::rect, BigDish::build_wall, BigParams::wall_hp, BigParams::wall_scent, BigParams::crystal_flow, BigDish::in_pile, BigParams::species_share, BigParams::species_convert_r, BigParams::mix_share]
---

# The mould at scale (the GPU plan)

## Now

The game's dish is 256 x 160 cells with 400-2,200 walkers on one CPU core,
a few dozen loops at most; Jenson runs the same Jones rules with millions of
GPU particles and hundreds of loops, and scale, not a better rule, is most
of the gap. **The GPU plan's phases 1-3 are done.** The fields stay `f32`
(fixed point changed play), walkers move all at once as a GPU does, and the
chosen scaling rule (today's numbers in fine cells, food smelled at today's
reach) gives about 65 loops at 32,000 walkers on the 1024 x 640 big dish
(`BigDish`). On the owner's GTX 1070 a frame of two ticks takes 1.37 ms at
100,000 walkers and 3.52 ms at a million, against an 8 ms bar. Phase 3, the
live sandbox, has [its own page](gpu-sandbox.md); what the outside looks and
creature rules do is on [outside looks and creatures](outside-looks.md).
**Phase 4 has begun with Larder World's W0**, [growth and
starving](growth.md) per walker, built on the CPU and the GPU and matched
in CI, and the world's fields are on the [Larder](../world/larder.md) page. Tables: [the GPU plan
report](../../Reports/gpu-plan-2026-11-10.md).

**Density needs one walker a cell.** The GPU engine drops that rule when its
slots pass half the agar, and the race world gives 10 slots a walker, so
every race-world run above about 7,000 walkers ran without it and its
walkers stacked into ropes; with `capacity` held to half the agar, 25,000
walkers make a lasting mesh or one continuous sheet (11-12, one seed; [slime
enemy review](../../Reports/slime-enemy-review-2026-11-12.md)). The GPU
thread is making the check loud.

## The plan and phase 1

**Phases 1 and 2 have run, the step runs on the owner's GPU, and phase 3's sandbox is built.** The plan, revised after an independent review on
2026-11-10, has six gated phases:

1. CPU foundations that test the port's riskiest assumptions at today's scale.
2. An offline 1024 x 640 dish with about 100k walkers, judged by paired seeds
   and pictures, plus a GPU timing test on the owner's machine.
3. A GPU sandbox.
4. Game rules on the GPU.
5. Porting the dishes that survive a dense network.
6. New play that only scale allows.

Phase 4 (game rules on the GPU) has begun with Larder World's W0: births,
upkeep and starving per walker, built on the CPU and the GPU and matched
to the bit in CI ([growth and starving](growth.md)). W1b adds the world's
crystals, terrain and walls to the engine, twin-tested on nine dishes;
on the GTX 1070 they take the 1024 × 640 frame from 1.73 to 2.11 ms. W2a
adds the squad's hooks (shots, strike, smell, touch, aim, harvest): 40
units a frame cost 1.5 ms there, mostly one thread a unit. W2b runs
Larder's rules on the engine: the CPU race world takes `scale` fine steps a
tick, so the engine takes `scale` ticks a game tick (report, *W1b*, *W2a*
and *W2b*;
[Larder World roadmap](../../Reports/larder-world-roadmap-2026-11-11.md),
[Larder](../world/larder.md)).

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

## The scaling rule and the big dish

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

## GPU timing

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

## Sources

- [gpu-plan-2026-11-10.md](../../Reports/gpu-plan-2026-11-10.md) and its [independent review](../../Reports/gpu-plan-2026-11-10/review.md).
- [larder-world-roadmap-2026-11-11.md](../../Reports/larder-world-roadmap-2026-11-11.md): phase 4 as Larder World W0.
