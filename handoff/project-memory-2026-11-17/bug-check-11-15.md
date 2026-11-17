---
name: bug-check-11-15
description: 2026-11-15 deep bug check of the 11-14 merges (#276-#307): what was fixed (PR #311, #309), what is open, and the ground-edge skip rule
metadata:
  type: project
  modified: 2026-11-15T01:15:46.813Z
---

Owner asked 11-15 00:20Z for a deep bug check of the day's merges. Thread cmsg_01PcwPdssrGxiXYvyCG8WaUXEWcptqoxYJpWupjdAdCAU3. Report: Reports/bug-check-2026-11-15/README.md.

Fixed in PR #311 (merged 11-15 01:31Z): linked-squad soft-lock when the controlled squad dies (Cmd::Active); Thrift card taking a run gun over budget (order-dependent cost; put_in/move_to recheck); run shop trigger/barrels upgrades ignored (Run::armed); view.wgsl soil_detail skip must test all 4x4 spline cells, not 2x2 (seams off shores/bridges). llvmpipe +12-17% GPU tail; 1070 +0.2 ms GPU a frame (kept; report PR #313).

Fixed by the planets thread in #309: dune clay overwriting deep water/bridges (26/32 dune planets), bridge planks on dry land, stale walking maps starved, map_for tie, cache trim.

Rifle spread DONE (owner 08:10Z card: rifle 0.5, long gun 0.3, RIFLE_SPREAD = Rules::spread default; PR #335 merged 09:28Z). Fire-kill counting (Game::fire_killed, per-frame World::burned), plants in census tools (plants::planted, plants=0), N on planet:D:SEED -> SEED+1: PR #340 (merged 11-15 10:10Z, thread cmsg_01PcwPdssrGxiXYvyCG8WaUXErGDGpbmZQ7w5nhPjURKWs). Open (unowned): healing while working (ANSWERED owner 11-15 07:41Z: healing must be a specific action, not automatic; fixed in #330, Heal order key L); menu=0 run ignores Level::walkers; perf: Game::mods recomputes flight per call.

**How to apply:** any future spline/skip shortcut in view.wgsl must cover the full stencil the slow path reads, since later terms use shares as distances. See [[bug-sweep]], [[crisp-crystals]].
