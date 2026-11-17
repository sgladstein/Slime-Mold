---
name: testing-practice
description: Owner rule 2026-11-12 17:26Z (card): build first, balance later; census not a gate for features/looks; merged in CLAUDE.md via PR #150
metadata:
  type: project
  modified: 2026-11-12T17:05:01.254Z
---

Owner (Scotttt, 2026-11-12 17:00Z): "for every feature we build we run lots of tests for balance to see if it beats random. Sometimes this is hours of testing... reviewers suggest holding off actual implementation work because something was unbalanced in another area... We were developing bad looking slimes because more runners killed the player too easily (instead of rebalancing)." 17:02Z: look at how this has been done across the whole project, not only his examples.

Review doc: https://claude.ai/artifact/VUQ1oXjzRVVzdzU7itnZmH (thread cmsg_01PcwPdssrGxiXYvyCG8WaUXB5uQMwuRYCuXDSiZB3DHRE). Findings: ~70 of 102 threads were census runners vs ~3 owner playtests; census rejected owner-asked feel fixes (flee hole), a dry-crystal QoL fix (bot score fell), held dense mould look; but it did find dominant strategies.

Rule ADOPTED by card 17:26Z ("Adopt as written"), merged PR #150 (fc6b926) into CLAUDE.md, wiki/decisions.md, wiki/method/census.md, ROADMAP, Reports/testing-practice-2026-11-12.md: census is not a gate for features/looks/QoL; balance never blocks looks or features (retune numbers instead); census only for strategy questions (player AI, dominance checks before milestones/playtests, mission difficulty, owner questions), name the question and runner time, under ~1 h without owner OK; cheap smoke checks per feature (idle doesn't win, no sub-minute game, no soft lock, frame cost).

**How to apply:** follow the CLAUDE.md wording; coordinator told all threads. Merged. See [[pr-merging]], [[design-review]], [[tech-lead-review]].
