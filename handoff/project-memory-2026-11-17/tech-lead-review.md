---
name: tech-lead-review
description: Ongoing technical-lead review thread (2026-11-12): findings doc, owner wants re-review as other threads finish tasks
metadata:
  type: project
  modified: 2026-11-12T04:34:34.796Z
---

Started 2026-11-12 04:29Z on the owner's ask (two review threads: technical lead and game designer). Owner 04:30Z: "These threads should continue to review as the other threads finish each task"; the coordinator will notify on merges/milestones.

First report (doc https://claude.ai/artifact/PutmsG8iWZpDv6DBuBW5tT), against main 19a8765:
- larder-world window defaults to scale 2, 5,000 walkers (census size), not the GPU's 100k; game frame time never measured on the 1070.
- gpu::read_back allocates a staging buffer and waits indefinitely; ~4 blocking read-backs a game step.
- Census runs only on the CPU; twin tests prove 1-4 ticks, not equal outcome spreads.
- Collisions: scorch/crater (graphics, toolkit, buildings, Larder), scent emitters, alarm sources, walls (controls vs buildings), larder/outpost buildings.
- Caps: UNITS_MAX 64, SHOTS_MAX 256, MAX_PILES 64, MAX_HITS 256, MAX_SPECIES 4. PARAMS_BYTES is 848 since #92.
- Proposed: a new Game integration thread (params registry, merge queue, shared APIs, caps, rule tests); reuse the idle GPU thread for 1070 perf + GPU census.

**How to apply:** owner 04:43Z (standing, both reviewers): "Always send your advice directly to the threads. You don't need my approval" -- send with message_thread, no card first. See [[larder-world-merge-order]], [[independent-reviews]].

Owner 04:40Z tapped "Send all, start integration": per-thread notes sent to Larder, one body, graphics, toolkit, buildings, controls, sound, player AI; overlaps to the integration thread (cmsg_01PcwPdssrGxiXYvyCG8WaUXVvB9idhrH7XKQHoSEmCnSB); ROADMAP refresh asked of the coordinator. Next: re-review each thread as the coordinator reports merges/milestones.

Owner overnight rule (04:48Z, going to bed): "When there is a question have the threads work together to come up with the best answer and move forward with it." Tech lead calls technical questions first, with the designer review (cmsg_01PcwPdssrGxiXYvyCG8WaUX2dsdkBBhuYywD19AnT6kXG) and the integration thread; the thread then moves ahead. Reviewed 04:47Z: #93 (BODY_N=6 tuned at 5k walkers), #94 (good), #96 (FULL_KILL=80 tuned at 5k) -> asked for a walker-density rule.

Overnight to 11:22Z: #101 (one packed read-back, gpu=both census: no CPU/GPU bias on 32 seeds), #110 (step split, frame-rate replay test), #116 (CI runs the window loop) merged. Flee-from-gunfire dropped (random wiped 12/16). Craters (#113) merged ON despite random beating plans on seeds 201-232 (control shows craters cause it); agreed with the integration thread and the designer: core defence next, judged by plans over random after Holm on both seed blocks (5-36, 201-232). Seed blocks differ (random wipes 5/16 vs 2/32, p 0.03): the gate uses two blocks. One-body food sharing is all-or-nothing; advised local, capped sharing.

17:31Z: after PR #150, withdrew all census/two-block merge gates in notes to Larder, controls, tiers, one body, player mould, toolkit and buildings (correctness tests stay; player AI keeps small censuses). #152 merged: told buildings to build the siren now on the existing alarm field write (toolkit gives one entry point later), and to check by eye whether soldiers feed the nest map or the score fails to charge idle.

Thread ids: GPU species/tiers + engine perf thread is cmsg_01PcwPdssrGxiXYvyCG8WaUX1DK3MmPz7kn9bEAqueWoQt (PRs #148, #157); graphics is KJA1Ex. #157 (crowd_holds fix, crater atomicMax) advised merge on green; 1070 trail timing after, split countdown buffer if >5% of trail.

2026-11-14 cloud plan (#205): settled with designer+integration: D1 census moves to shape C rules at census size (4-seed direction check, baseline break); D6 integration prepares the big-file split on a branch, then a <45 min freeze; menu/map-editor threads stay off larder-world.rs until it lands; D4 mission 8 = own mould vs Grudge, rival pair to chapter 2 after the GPU per-species birth cap (player-mould thread PR). C6 PC batch script: integration sends it to me for review before the runner.

Thread ids (11-14/15): levelgen cmsg_01PcwPdssrGxiXYvyCG8WaUXHpvAsm955EuZKKdhjnHSNE; "Improve the slime enemy" (surges, thickening) cmsg_01PcwPdssrGxiXYvyCG8WaUXVjzMVR9jPQsKucgP9GF9Hs (not QsjsM, which is the older design thread); graphics polish cmsg_01PcwPdssrGxiXYvyCG8WaUX9zcGHdA2XYJgcvA8tL8cUU. Relays carry session ids only: check list_threads before replying.
