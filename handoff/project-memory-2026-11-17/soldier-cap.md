---
name: soldier-cap
description: Soldier cap thread 11-16: owner picked flat 200 soldiers (was 40) to test the max; costs measured; GPU unit slots raised 96->256
metadata:
  type: project
  modified: 2026-11-16T05:07:54.689Z
---

Soldier cap thread (cmsg_01PcwPdssrGxiXYvyCG8WaUXX7osAVKq1BA7ze1hGQ85oF), branch claude/project-thread-2d6a07, 2026-11-16.

- Owner 05:06Z picked "Flat 200, the realistic max" ("Maybe scale later in when releasing but I want to test the max"). Scaling by planet size is a possible later release change, not now.
- Limits found: `Rules::max_squad` (40), `squad::UNITS_MAX` (96 GPU frame slots shared by soldiers + home + up to 40 buildings; soldiers past ~55 silently dropped), SHOTS_MAX 256 bullets in flight (200 riflemen peaked 129), MAX_MARKS 2048 (dead drawn first, never removed), SQUADS_MAX 16.
- Cost (cloud CPU, `examples/soldiercap.rs`): soldiers' turn ~0.01 ms/soldier/step on planet 1, ~0.03 on a 6x planet; 6x planets already pay ~6.7 ms/step at 40 (walking maps, levelgen thread's code).
- Owner quote 11-15 (hands.rs greedy): "a full squad of 40 within the first 30 seconds" — the cap binds early.

See [[squad-control]], [[levelgen]], [[scott-desktop]].

**11-16 05:45Z:** PR #377 (cap 200, UNITS_MAX 256, SHOTS_MAX 1024, MAX_MARKS 4096; branch sha ceaf153) labelled `queue`, merged 05:42Z. 1070 timing asked via coordinator/PC runner: worktrees `sm-cap` (ceaf153), `sm-cap-main` (e305994), output `sm-cap-out`; `bench=60 hand=greedy mode=squads set=start_bank=1000000 set=max_squad=N passes=30`, planets 1:1 and 8:1. `hand=rush` buys nothing; use greedy. Squad movement rework thread (cmsg_01PcwPdssrGxiXYvyCG8WaUX4uE2KRVhL3PF2znGuNZZ1c) takes members()/rank_squad/elbow_room scaling.
**1070 (11-16 05:44Z):** buffers free; GPU step +0.4 ms at 200; a step's frame p95 at 200: planet 1 9.1 ms, planet 8 14.9 ms (CPU step p50 1.3 -> 6.3); one 58 ms frame unexplained. perf: per-soldier walk is what grows (movement thread has it). Greedy no-pump smoke stalls at 29-45. Follow-up PR #380 queued. Open: pumping-hand snowball smoke, zoom-6 readability, owner playtest.
**06:10Z:** 58 ms spike did not repeat in 6 runs (one-off on PC); worst CPU step at 200 on planet 8 ~14 ms (8 at 40) repeats. bench now prints its 3 longest frames with game time (313ce4f). Walking maps -> levelgen thread, per-soldier walk -> movement thread (both told).
**06:16Z:** levelgen thread: the walking-map builder cost is fixed (MAP_POPS/step, ~5 ms busy either way); at 200, 47% of map lookups went unserved because MAP_NEAR 4 < formation spread; their PR raises MAP_NEAR to 12 (unserved 47 -> 19%, 40-soldier turn 6.4 -> 3.8 ms). Next lever: coarser 6x maps. #380 merged 06:22Z. Thread done; open: owner playtest at 200, zoom-6 readability, pumping-hand snowball smoke, brake waits on D7.
**06:34Z:** MAP_NEAR 4 -> 12 is PR #382 (queued; report Reports/map-sharing-2026-11-16/). Movement rework (one map per squad) cut map waits on planet 8 at 200 from ~5,600 to ~60 per 40 s run.
