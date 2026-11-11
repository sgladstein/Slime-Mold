---
title: The GPU sandbox (phase 3)
kind: mechanism
status: current
updated: 2026-11-11T19:15Z
sources: [Reports/gpu-sandbox-2026-11-11.md, Reports/jenson-looks-2026-11-11.md, Reports/outside-rules-2026-11-11.md, Reports/creatures-2026-11-11.md]
code: [BigParams::species, BigParams::graze_weight, BigParams::species_boost]
---

# The GPU sandbox (phase 3)

## Now

`cargo run --release --bin mouldgpu` runs the big dish's step live on the
GPU, with every option the Jenson comparison tested as a control (owner,
2026-11-11 00:06Z), plus the looks, outside rules and creature rules as
controls on later panel pages and keys. It matches the CPU dish on paired
seeds (no measure differs after Bonferroni), and three twin tests in CI
check one GPU tick against one CPU tick on software Vulkan. On the GTX 1070
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

Three facts bind the design:

- The census feeds the economy every step (`FED_EVERY` 6), so its fed and
  piece labels must go back up to the GPU.
- A finer grid changes the mesh and the smell range in dish terms; the
  chosen scaling rule keeps the fine mesh and today's smell reach.
- Cloud sessions have no GPU. CI runs the GPU step on Mesa's software
  Vulkan; with the fields `f32` (fixed point changed play, below), the twin
  test is one tick from a CPU state, not the same bits over a run.

## Sources

- [gpu-sandbox-2026-11-11.md](../../Reports/gpu-sandbox-2026-11-11.md): the sandbox, its controls and presets, the twin tests and timings.
