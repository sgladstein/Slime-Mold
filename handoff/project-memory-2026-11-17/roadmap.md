---
name: roadmap
description: ROADMAP.md (2026-11-10, PR #15) sequences game fixes, GPU phases, Bloomfront RTS and Pixel_Physics ports; how to keep it current
metadata:
  type: project
  modified: 2026-11-10T18:06:23.214Z
---

`ROADMAP.md` at the repo root (PR #15, 2026-11-10) puts the four active plans in one order: G (game fixes, triage thread), E (Pixel_Physics ports), S (GPU phases, Jenson/GPU thread), R (Bloomfront, RTS thread). Four bands: now / waits on owner / scale fork (GPU phase 2 gate) / last. Linked from HANDOFF.md and README.md. HANDOFF gained open decisions 7 (Bloomfront: wait-or-replace, fog of war, 2 vs 4 keepers) and 8 (sound).

Cross-plan rules it sets (recommendations, not owner rulings): keyed random draws (E6) are built inside GPU phase 1 and land with any integer switch as ONE census reset, between reports; multi-core renderer held until the scale gate; profiler built once by the GPU thread; The Wild One's middle (G2) and Bloomfront Stage 0 done as one piece of work in one thread.

**How to apply:** a thread that passes a gate, merges a step's PR, or changes owner updates its row in ROADMAP.md in the same PR. Related: [[gpu-plan]], [[rts-direction]], [[pixel-physics-review]], [[handoff-triage-2026-11-10]].
