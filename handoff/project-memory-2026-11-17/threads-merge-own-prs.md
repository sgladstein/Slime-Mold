---
name: threads-merge-own-prs
description: Owner rule (2026-11-10 17:47Z) that each thread merges its own PRs once green
metadata:
  type: feedback
  modified: 2026-11-10T17:49:15.938Z
---

Owner (Scotttt, 2026-11-10 17:47Z, project chat): "all threads handle their own PR mergers".

**Why:** the owner doesn't want to be the merge step for every thread's PR.
**How to apply:** merge your own PR yourself once both CI jobs and the wiki lint ("what changed") are green on the exact head, using a merge commit (HANDOFF.md, *How the work has been done*). Before merging, bring main in and run `python3 wiki/lint.py`: any new top-level `Reports/*.md` must be cited by a wiki page, or listed in NOT_YET_INGESTED. Related: [[llm-wiki-plan]].
