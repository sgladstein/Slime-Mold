---
name: gpu-plan
description: The 2026-11-10 plan to take the mould simulation to GPU scale: phases, gates, review findings, where it lives, open owner decisions
metadata:
  type: project
  modified: 2026-11-11T19:25:00.000Z
---

Plan: `Reports/gpu-plan-2026-11-10.md` (doc https://claude.ai/artifact/NhwbErNYntvbu53kh39szf), built by the plan's thread (branch claude/project-thread-df7snz). Six gated phases: (1) CPU foundations; (2) offline 1024x640 big dish + GPU timing test; (3) `--bin mouldgpu` sandbox; (4) game rules on GPU; (5) port surviving dishes; (6) new play from scale.

**Why:** owner judges by interesting play, not realism; scale (Jenson's 180-340 loop clips) is the target. Owner 2026-11-10 20:55Z: current demos are not the true games; judge engine work by engine measures on a neutral dish.

**Owner's GPU:** GTX 1070 on "Scott-Desktop" ([[scott-desktop]]); runs go by Remote Control from this thread.

**Done:** phase 1 (fields stay f32: fixed point changed play; late census safe). Scaling rule "nose" + `mould::big` (PR #30). Owner picked 100k walkers (22:32Z). `examples/gputime.rs` on the 1070: 100k 1.37 ms/frame of two ticks (raw recovered later into gpu-plan-2026-11-10/raw/). Sandbox `--bin mouldgpu` (PRs #39/#40, page https://claude.ai/artifact/1rdrxadVcNdxTmbxp2KJGy), twin test in CI on Mesa lavapipe (PR #43: one GPU tick from CPU state vs one BigDish::tick).

**Independent review 2026-11-11 (PR #46):** fed scent and jumps-after-step added on the GPU; then nothing differs from CPU after Bonferroni (8-16 seeds, llvmpipe and 1070). Quote headless timings (the 1070 downclocks at light vsynced load). Lessons: quote seed spreads, keep raw for every number; each GPU is a new seed (f32 rounding), so compare as spreads.

**Looks (PR #48) and outside rules (PR #59) are sandbox controls**, twin-tested; the shader compiles outside rules out when off (`override OUTSIDE`, they cost 18-24% compiled in). Warm converting dishes up unboosted.

**Engine as a library:** PR #63 (camera thread) moved Engine to `slime_mould::gpu`, merged 11-11 19:19Z.

**Larder World W0 (owner ruling 11-11 14:06Z; roadmap Reports/larder-world-roadmap-2026-11-11.md):** this thread owns W0, births/upkeep/starving per walker, CPU in big.rs first, then GPU + twin test after the Engine library move. Agreed with Larder (W1): their eating is equal shares per cell (min(crystal_eat, charge/n), n = occ after the move), per-walker take in BigDish `took`; W0's life() runs right after their eating block; dead walkers leave occ; births check agar (rock is not agar).

**W0 merged 2026-11-11 19:22Z (PR #73, 16256ad):** growth on the CPU and GPU (passes life/breed/born, binding 33 `took`); `energy_flow` CPU-only scratch, off. With the life cycle, `BigDish::active(k)` = lives(k) && LIFE_WALKER: only ordinary walkers pay upkeep, starve, divide, graze; spores/dormant keep energy; strike frees the slot (alive=false). Open: does sleep cost upkeep; cut-tube test for energy_flow. Agreed next (creature thread): port species_share, species_convert_r, mix_*, species_sets to the GPU, each with a twin dish (uniform array of up to 4 sets OK); the life-cycle rules too (tiers thread: BigParams doc comments are the spec).

**Next (older):** when the RC session wakes (only when the owner writes in this thread): 1070 timing of the outside presets, and the camera thread's world-camera timing (command in Reports/world-camera-2026-11-11.md on its branch; results to that thread, coordinator ask 14:42Z); species_share when the looks thread relays it (on top of the library move); the owner's hands; phase 3's tools as field writes.

**Still open (HANDOFF):** upload Jones 2011; hold Pixel_Physics renderer port. See [[pixel-physics-review]], [[rts-direction]], [[judging-outside-sims]], [[software-gpu]].
