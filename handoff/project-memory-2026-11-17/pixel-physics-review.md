---
name: pixel-physics-review
description: Pixel_Physics reuse review (2026-11-10): what to take into Slime-Mold, frame split, review doc link
metadata:
  type: project
  modified: 2026-11-10T16:55:28.191Z
---

Pixel_Physics (github.com/sgladstein/Pixel_Physics) clones over plain https from the cloud container even though it is not a configured repo for this project.

Review done 2026-11-10, Claude Doc: https://claude.ai/artifact/Pe83NVdRMXhPvxeYoKASRu
- Measured at b8efc14: mouldprobe seeds=1 players=plan, scale 4 → mean 4.93 ms/frame = sim 1.10 + draw 3.83 (draw is 78%). Worst 14.94 ms.
- Recommended takes: parallel row render (src/render.rs ~5346), audio/ architecture (game is silent), rng::stream keyed draws, pheromone tile/row sleep; tools: determinism test + dish hash, frame_profile, licensecheck/notices, crispfont.py, docscheck.
- Misfits: capitals-only font, missing crispfont.py, no JetBrains Mono OFL notice, release profile tuned on sand engine (CI now billed), stale hud.rs/mod.rs comments.
- Owner had not decided which to act on at time of writing.

**How to apply:** before implementing any item, check the doc for owner comments; add a dish hash first so speed-ups can be proved bit-identical.

Update 2026-11-10 18:06Z (roadmap thread, ROADMAP.md in PR #15): hold the multi-core renderer until the owner decides HANDOFF item 6 (the GPU plan's phase 2 may replace it). Build keyed random draws only with the GPU thread, as one census reset. Step 1 (dish hash, PR #14) landed or is landing. Update the review's rows in ROADMAP.md as each step lands.
CI test job after #21 settled: 220 s on PR #23 (warm cache) vs 289 s with thin LTO, so dropping LTO did not slow CI.
Landed 2026-11-10: PR #14 (E1 dish hash), PR #21 (E4 cgu=1 kept, thin LTO dropped; E5 licensecheck/notices/notices.yml, OFL notice, crispfont.py). After #21 the first CI test job took 537 s against 289 s before, but the profile change also invalidated the build cache, so that is not a clean A/B. Left for the owner: E3 parallel draw (on hold, HANDOFF 6), E7 sound (HANDOFF 8). E6 keyed draws go with the GPU thread.
