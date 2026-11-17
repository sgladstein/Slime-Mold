---
name: runner-threads
description: Owner allows up to 4 Haiku runner threads per thread for long test runs; ask the coordinator with branch, exact commands, seeds
metadata:
  type: feedback
  modified: 2026-11-10T19:46:07.750Z
---

Scotttt, 2026-11-10 19:44Z: any thread running long tests may "start some haiku runner threads just to get additional compute ... they make no decisions and just run the code you tell them and send the output back to you. You can run up to 4 threads."

**Why:** cloud containers have 4 cores; a full paired census (every dish, 16 seeds, one arm) takes about 25-30 minutes, and several arms at once take hours.

**How to apply:** push the exact commit first, then message the coordinator with each runner's branch, commit, exact commands, seeds, and how to send output back (the GPU plan thread asked runners to commit only the raw output files to their own branch and reply with branch, sha, timing and a few hash= lines as a cross-machine bit check). Raw output still ends up committed in this repo (owner rule: research and raw census output live in the repo). See [[gpu-plan]].
