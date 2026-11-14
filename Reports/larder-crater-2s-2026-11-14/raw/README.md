# Raw files: two-second craters (2026-11-14)

One binary for every file: `larder-world` and `examples/larderworld.rs`,
built from the PR's first commit (ed0bf08, `Rules::crater` 2.0), run on
the cloud's llvmpipe Vulkan (look files) or the CPU (smoke files). The
4.5 s arm passes `crater=4.5`; the 2 s arm is the default.

- `holes-HAND-4.5.txt`, `holes-HAND-2.txt`: `larder-world look=DIR
  map=fork hand=HAND holes=1 [crater=4.5] at=...` on shape C (the
  window's default), seed 1. The `holes:` line is the refill timer
  (ticks to 80% of the body before the hit, 15 ticks a second). HAND is
  `volley` and `edge` (at=21.9,33.9,40; both squads die by 25 s, so the
  last frames are never reached), `stand` and `rush` (at=30,60).
- `squad-volley-4.5.txt`: `hand=volley crater=4.5
  at=15,19,21,22,22.5,23,23.5`, the squad's touching and health a frame,
  showing the burn order firing nothing after 22 s.
- `smoke-c-crater4.5.txt`, `smoke-c-crater2.txt`: `larderworld census
  seeds=5-8 walkers=25000 upkeep=0.0002
  keys=capacity:50000+energy_flow:0.05
  hands=idle,random,farm,strike,rush cap=600 threads=4 crater=4.5|2`, the
  smoke games, with `time`'s lines at the end.
