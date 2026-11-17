---
name: scent-view
description: F2 scent view in Larder World (View::scent, scent=1): food scent blue with contour lines, trail pink; PR #365 11-15
metadata:
  type: project
  modified: 2026-11-15T21:26:45.806Z
---

Owner 11-15 20:54Z asked to see the mould's scent; said yes to a toggle. PR #365 merged 11-15 21:36Z.
F2 toggles `View::scent` (view.wgsl `scent_view`, params word after alarm_look; `scent=1` arg for films). Shows look byte 3 (food scent, blue + contour lines 1-3 px at any zoom) and byte 0 (trail, pink): the two fields `sense_at` adds. Page https://claude.ai/artifact/RhNbheKFwttFH2XpvUfH51, report Reports/scent-view-2026-11-15/. 1070 timing not done.

**Why:** owner wanted to see what the slime follows. **How to apply:** new field views reuse this pattern; don't run `cargo fmt` on the whole tree (the repo isn't fmt-clean; it reformatted 100+ files in this PR's first commit). Related: [[world-look]], [[slime-look]].
