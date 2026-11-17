---
name: no-upkeep
description: Upkeep removed from code 11-14 (PR #298): Rules::ration/hunger and lure_burn gone; what still lowers the bank
metadata:
  type: project
  modified: 2026-11-14T20:51:53.246Z
---

Owner 11-14 20:10Z still saw "upkeep" after the rulings (01:49Z no feeding, 04:53Z no maintenance). On main both were already 0; PR #298 (branch claude/project-thread-3duzqj, merged 11-14 21:13Z) deletes Rules::ration, Rules::hunger, Game::feed/eaten/hungry/lured, BuildRules::lure_burn, the ration= key, campaign `rule ration`, census ration=/hunger= and `eaten` column. Test `soldiers_cost_nothing_to_keep`.

Kept (not upkeep): one-time prices, squad-mode healing 0.25 food/pt, opt-in Sow, mould eating the bank at larder/outposts, overrun 20 s. Asked owner what exactly they saw; unanswered at PR time.

**Why:** owner rule: nothing costs food to keep. **How to apply:** never add a per-second food cost; bind food via prices or the mould. See [[larder-playtest-fixes]].
