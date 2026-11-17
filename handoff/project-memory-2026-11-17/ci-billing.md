---
name: ci-billing
description: CI stopped 2026-11-12 01:08Z on the Actions spending limit; owner made the repo PUBLIC 01:40Z to fix it, so everything pushed is public
metadata:
  type: project
---

From about 2026-11-12 01:08Z every CI job on sgladstein/Slime-Mold failed in 2-3 s with no runner and no log. The job annotation (`repos/sgladstein/Slime-Mold/check-runs/<id>/annotations`) said the job was not started because of failed payments or the spending limit. The owner fixed it at 01:40:54Z: "I made the repo public which should fix the github actions issue". CI ran again from 01:41Z, and #86 merged green at 01:46Z.

**Why:** the repo is public now. Commits, reports, raw output and PR text can be read by anyone.
**How to apply:** keep private details out of anything pushed: the owner's email and other personal data, and machine paths beyond the documented ones. If CI fails instantly again (runner id 0, log 404), read the annotation before you spend a re-run or a fix. Merge only on green, per [[pr-merging]].
