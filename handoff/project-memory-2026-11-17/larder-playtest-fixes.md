---
name: larder-playtest-fixes
description: 11-14 playtest fixes PR #237: tool_weight 0 (harvest refusals), wall builders beside the line + unstuck rules, lure_burn 0 (owner: no upkeep)
metadata:
  type: project
  modified: 2026-11-14T06:04:31.539Z
---

Owner playtest 2026-11-14 04:44-04:53Z. Thread "Fix soldier harvest and wall bugs" (cmsg_01PcwPdssrGxiXYvyCG8WaUXAkgia92uAPUifxEWKsZZoL), PR #237 queued.

- Harvest refusals: the window kit gave selected soldiers at the larder 5 tools x 50 carry, so they were full at 50 food and turned home. KitRules::tool_weight now 0; soldiers never home empty; Cmd::Order clears Soldier::dry.
- Stuck on walls: builders stood on the wall line (shut in when it rose); downhill cut wall corners. Builders now stand beside it; soldiers in rock/walls walk out; a goal in rock goes to Game::nearest_standable.
- Owner ruling 04:53Z: "There should be no maintenance costs in this game." BuildRules::lure_burn now 0 (the only per-second cost left; rations already 0). Per-use prices (tools, gun casts) stay.

**How to apply:** don't add any per-second food drain for the player. Report Reports/larder-playtest-fixes-2026-11-14.md. Related: [[soldier-controls]], [[larder-buildings]].
