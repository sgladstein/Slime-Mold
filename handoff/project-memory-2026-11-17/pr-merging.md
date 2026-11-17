---
name: pr-merging
description: Threads merge their own PRs by adding the `queue` label (own Actions merge queue, live 2026-11-14); no more re-merging main by hand
metadata:
  type: feedback
  modified: 2026-11-14T04:22:00.000Z
---

The owner ruled on 2026-11-10 at 17:47Z that "all threads handle their own PR mergers". Since 2026-11-14 ~04:20Z they do it through the merge queue: add the label `queue` (`gh api repos/sgladstein/Slime-Mold/issues/N/labels -f 'labels[]=queue'`; MCP issue_write `labels` replaces other labels; GraphQL and so `gh pr merge --auto` are blocked from sessions). `.github/workflows/merge-queue.yml` (PR #229) merges up to 5 queued PRs onto main, runs ci.yml once on the candidate, fast-forwards main when green. Conflicts/red PRs lose the label and get a comment.

**Why:** GitHub's own merge queue is org-only and the repo is user-owned; owner card 2026-11-14 03:55Z picked "Build our own queue in Actions" over moving the repo to an org. Before it, PRs re-ran ~11 min CI after every merge (one went green 4 times).

**How to apply:** don't merge main in to re-run CI; label and wait for the PR to show merged. First run #231 (docs) landed in ~1.5 min. To disable: delete merge-queue.yml. Pushes to main by the queue use GITHUB_TOKEN, so no push-CI runs on main for queued merges (the exact commit was tested).

**Billing stop (2026-11-12, 01:08-01:40Z):** a red `what changed` job with no steps means Actions billing, not the PR's code: tell the owner, re-run once fixed.

Merge queue (live 2026-11-14 04:19Z, #229): label a PR `queue`; Actions merges main + up to 5 queued PRs, runs ci.yml on the candidate, fast-forwards main when green. No need to re-merge main yourself. Keep ci.yml's workflow_call inputs (ref, base) and `ref: ${{ inputs.ref }}` on checkouts. Queued merges land via GITHUB_TOKEN, so push-triggered CI on main does not run for them.

No assignee, no review request (owner, 11-14 19:35Z): never add sgladstein as assignee or request their review on a PR; GitHub emails them about every PR. Keep the attribution lines in the PR body.
