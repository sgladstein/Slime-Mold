---
name: cloud-plan
description: 2026-11-14 plan for working without owner playtests or the 1070 (PR #205): items C1-C10, picks D1-D6, two new threads asked
metadata:
  type: project
  modified: 2026-11-14T00:33:38.916Z
---

On 2026-11-14 at 00:28Z the owner (Scotttt) couldn't playtest and the GPU was busy, so he asked for a review of every thread and a plan. The plan is `Reports/cloud-plan-2026-11-14.md`, merged in #205 (323832e).

What it found: about 80 PRs had merged since the last combined playtest page (11-12 11:10Z). Every feature sits behind a command-line flag. The 1070 twins for #181 and #198 never ran. Missions 4-7 are unblocked.

Cloud items and owners:
- C1 playtest pack (integration)
- C2 start menu and key card (new thread)
- C3 missions 4-7 (campaign)
- C4 small plan-by-tier matrix on shape C (player AI), after the designer's counter table D3
- C5 germ and gas on the GPU (toolkit and rounds)
- C6 one PC batch script (integration; timing half held until the owner says go)
- C7 sounds (sound thread)
- C8 visible panic and withering (graphics)
- C9 split larder_world.rs and larder-world.rs, and delete src/bin/mouldgpu.rs.orig (integration, in a freeze window)
- C10 map editor (new thread)

Picks for the reviewers and integration:
- D1 census baseline moves to C
- D2 flow_loss parked
- D3 counter table
- D4 mission 8's rival pair
- D5 front-door scope
- D6 freeze window

The coordinator was asked at 00:3xZ to start the C2 and C10 threads and to message the rest.

**How to apply:** check the report before starting no-playtest work, and update items there as they land. See [[game-integration]], [[design-review]], [[tech-lead-review]].
