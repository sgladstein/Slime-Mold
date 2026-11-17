---
name: view-cost
description: Performance thread 11-15: how to time each view pass and each shading part on the 1070; what the view costs and what was tried
metadata:
  type: project
  modified: 2026-11-15T01:10:34.434Z
---

Performance thread (11-15, owner asked 00:21Z), branch claude/project-thread-p0w3km.

- `larder-world ... bench=S viewpasses=N` prints each view pass's GPU ms alone (base, prep, compose, marks, specks, drain).
- `LARDER_VIEW_WGSL=PATH` loads another view.wgsl without a rebuild; `scripts/view-cost.ps1` (PC runner) times every swap in `Reports/view-cost-2026-11-15/knockouts.json` at fork/planet zoom 1/6/24, `-Compare SHA` for A/B, `-Only` to pick variants. Worktree `C:\Users\Scott\Code\sm-perf`.
- 1070, main 11-14: screen pass 1.9-2.5 ms at zoom 6-24, plants 0.9-1.3 ms of it (plant reads ~0.8, crowns ~0.7); prep 2.6 ms at zoom 1 (mostly off-map). After the fixes zoom 1 view 5.12 -> 1.90 ms.
- llvmpipe's split differs from the 1070's (crowns looked dominant there); decide with the 1070.
- Packing plant fields for the pixels was SLOWER on the 1070 (dead-ends). Next idea: a workgroup tile of plant fields in `base`.

Report: Reports/view-cost-2026-11-15/README.md. See [[software-gpu]], [[scott-desktop]].
