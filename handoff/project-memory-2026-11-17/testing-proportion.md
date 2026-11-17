---
name: testing-proportion
description: Owner 2026-11-12 17:00Z: too much balance-census gating; don't hold features for balance elsewhere; rebalance numbers instead of uglifying
metadata:
  type: feedback
  modified: 2026-11-12T17:11:14.844Z
---

Owner (Scotttt, 2026-11-12 17:00Z): "for every feature we build we run lots of tests for balance to see if it beats random. Sometimes this is hours of testing... right now we are testing our gun code which seem unnecessary... reviewers suggest holding off actual implementation work because something was unbalanced in another area... We were developing bad looking slimes because more runners killed the player too easily (instead of rebalancing)." A testing-review thread exists for it (cmsg_01PcwPdssrGxiXYvyCG8WaUXB5uQMwuRYCuXDSiZB3DHRE).

**Why:** census gates on every feature slowed building playable features, and balance holds blocked unrelated work.

**How to apply:** keep fast correctness tests (unit, twins, replay hash, CI window loop). A full census gate is only for shipped defaults every thread measures against, and for AI-strategy work. Feature code: build it, unit-test it, show the owner; at most a quick, non-blocking sanity run. Imbalance means retune the numbers, never hold or degrade a feature. Never hold one area's implementation for balance in another. See [[tech-lead-review]], [[independent-reviews]].

Adopted 2026-11-12 17:26Z (PR #150, review https://claude.ai/artifact/VUQ1oXjzRVVzdzU7itnZmH): build first, balance later; no census gates on features, looks or controls; balance never blocks work; censuses only for strategy questions, with runner time named first; over about an hour needs the owner's OK.
