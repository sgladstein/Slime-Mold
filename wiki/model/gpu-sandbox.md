---
title: The GPU sandbox (phase 3)
kind: mechanism
status: current
updated: 2026-11-14
sources:
  - Reports/creatures-2026-11-11.md
  - Reports/gpu-readout-2026-11-12.md
  - Reports/gpu-sandbox-2026-11-11.md
  - Reports/jenson-looks-2026-11-11.md
  - Reports/life-gpu-2026-11-11.md
  - Reports/pc-batch-1070-2026-11-14.md
  - Reports/outside-rules-2026-11-11.md
  - Reports/species-gpu-2026-11-11.md
code:
  - BigParams::graze_weight
  - BigParams::sleep_death
  - BigParams::species
  - BigParams::species_boost
  - BigParams::spore_rate
  - Engine::record_share
  - Engine::squad_one
  - Engine::strike
  - GpuWorld::spread
---

# The GPU sandbox (phase 3)

## Now

`cargo run --release --bin mouldgpu` runs the big dish's step live on the
GPU, with every option the Jenson comparison tested as a control (owner,
2026-11-11 00:06Z), plus the looks, outside rules and creature rules as
controls on later panel pages and keys. It matches the CPU dish on paired
seeds (no measure differs after Bonferroni), and twin tests in CI check
one GPU tick against one CPU tick on software Vulkan; a PR that changes
the GPU code runs them on the GTX 1070 too (*Twins on the 1070*, below),
where 27 of 29 pass and the two game-length twins drift from the 1070's
own float maths, with no lean over 32 seeds. On the GTX 1070
a frame of two ticks and the drawing takes 1.45 ms at 100,000 walkers and
8-10 ms at a million. Rules compiled in but off cost 18-24% on the software
GPU, so they compile out when off. Detail below; tables in
[the report](../../Reports/gpu-sandbox-2026-11-11.md). The engine itself
lives in [the mould at scale](scale.md).

## Detail

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
On the GTX 1070 their presets take 1.0-1.8 ms of ticks a frame at 30,000
walkers and 9.6-17.9 at a million (ribbons the dearest, from sensing).
Growth and starving (GPU phase 4's first rule, Larder World W0) has [its
own page](growth.md); it is a fifth panel page and keys Ctrl+3 and 4.
The creature thread's species rules are on the GPU engine too (11-11,
Larder World W3a): species 0's share (`share`), conversion weighed over a
square (`convert_r`), a second kind of walker in one mould (`mix_*`) and
each species' own settings (`sN.FIELD`, so a droplet species and a network
species can share a dish), as `set` keys of the sandbox and the engine, not
yet on a panel page. A twin test matches them on 12 dishes, a growth dish
checks that a child keeps its parent's kind, and 28 planted bugs are
caught. On the GTX 1070 they cost nothing off; two species pay 0.05 ms a
frame more, the second kind 0.12-0.19 ms, and the square of radius 8 1.3 ms
of a 2.3 ms frame at 100,000 walkers
([species on the GPU](../../Reports/species-gpu-2026-11-11.md), *Cost*). The
tiers' life cycle followed (11-12): spores, dormancy and dying asleep, the
alarm field, the player's strike (`Engine::strike`) and the squad shots'
alarm, under the CPU's keys, off by default, with no new binding. One GPU
tick matches one CPU tick on 18 dishes (the three tiers among them) and
the squad's frame on four ([life cycle on the
GPU](../../Reports/life-gpu-2026-11-11.md)).

Three facts bind the design:

- The census feeds the economy every step (`FED_EVERY` 6), so its fed and
  piece labels must go back up to the GPU.
- A finer grid changes the mesh and the smell range in dish terms; the
  chosen scaling rule keeps the fine mesh and today's smell reach.
- Cloud sessions have no GPU. CI runs the GPU step on Mesa's software
  Vulkan; with the fields `f32` (fixed point changed play, below), the twin
  test is one tick from a CPU state, not the same bits over a run.

## Twins on the 1070

The integration thread's rule (2026-11-12 15:55Z, from the technical
lead's review): a PR that changes a shader (`src/gpu/*.wgsl`) or the GPU's
`Params` or bindings runs the twin tests on the GTX 1070 before it merges,
when the PC is free under its booking file
(`/mnt/project-files/scott-desktop/booking.md`), and its body quotes the
bench's machine line and the pass and fail counts. With the PC busy or off
it may merge saying "1070 twin pending". The twins also run before any
owner playtest build, and a twin that fails on the 1070 but passes on
lavapipe keeps its rule from becoming a default until the failure is
understood. In the PR's own worktree on the PC, in PowerShell:

```
$env:MOULDGPU_TEST_GPU="1"
cargo run --release --bin larder-world -- bench=20 hand=farm    # the machine line, first
cargo test --release --locked --bin mouldgpu
cargo test --release --locked --lib gpu::
```

On `main` after the speed work, 27 of `mouldgpu`'s 29 twins pass and the
library's 3. The two that fail, `the_life_cycle_matches_the_cpu_for_four_ticks`
(spores, trail 1.2e-4) and
`a_squad_firing_on_the_fleeing_mould_plays_on_the_engine_as_on_the_cpu`
(908 killed against 765), fail the same way on `main` at 9f5e4a4: float
drift, since the 1070's `sin`, `cos` and `sqrt` differ from the CPU's and
its driver fuses `a * b + c`. A third failure, or other numbers in those
two, is the PR's to explain. Lavapipe hides races between workgroups that
the 1070 shows, so a pass where two threads may meet on one cell gives
each cell to one thread or takes an atomic (`cut`'s crater takes
`atomicMax` on the f32's bits since 2026-11-12: [the readout
report](../../Reports/gpu-readout-2026-11-12.md), *Two GPU fixes*). To see the drift itself:
`cargo test --release --locked --bin mouldgpu the_adapters -- --ignored --nocapture`
(the adapter's maths beside the CPU's, then the game over seeds; set
`DRIFT_SEEDS=32` for 32). The numbers are in [the readout
report](../../Reports/gpu-readout-2026-11-12.md), *On the GTX 1070*.

The whole batch now runs from one script, `scripts/pc-batch.ps1` (twins at
main, then timing with `-Timing`). At 004b7cb (2026-11-14) four more twins
failed on the 1070 alone: card drift (birth order, one flung walker) and
chaos on a small late mould (the squad twins), none a rule. Their tolerances
carry the card's numbers. Every game frame was under 2.6 ms p95, the draw
0.1-0.6 ms ([the batch](../../Reports/pc-batch-1070-2026-11-14.md)).

## Sources

- [gpu-readout-2026-11-12.md](../../Reports/gpu-readout-2026-11-12.md): *On the GTX 1070*, the game's passes, the twins and the maths on the 1070.

- [gpu-sandbox-2026-11-11.md](../../Reports/gpu-sandbox-2026-11-11.md): the sandbox, its controls and presets, the twin tests and timings.
- [species-gpu-2026-11-11.md](../../Reports/species-gpu-2026-11-11.md): the species settings on the GPU, their twin test, planted bugs and cost.
- [life-gpu-2026-11-11.md](../../Reports/life-gpu-2026-11-11.md): the tiers' life cycle on the GPU, its twin tests, planted bugs and cost.
- [pc-batch-1070-2026-11-14.md](../../Reports/pc-batch-1070-2026-11-14.md): the 1070 batch at 004b7cb, twins triaged and every timing.
