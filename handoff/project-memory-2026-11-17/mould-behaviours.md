---
name: mould-behaviours
description: Mould tiers thread (2026-11-11): Sporer, Sleeper, Swarmer on the CPU big dish (PR #70), then into the game (PR #85: Tier API, shots lay alarm, world re-test, sleep_death, two reviews)
metadata:
  type: project
  modified: 2026-11-12T00:08:05.014Z
---

Thread "Find new mould tiers and bring them into the game". Default hash must stay 869141a7bf398adf (bigdish preset=dense seconds=2 blob=0 piles=1).

PR #70 (merged 11-11 17:40Z): per-walker life cycle (walker/spore/dormant/dead) and an alarm field laid by BigDish::strike. Sporer (spore_rate), Sleeper (sleep_rate, bait then strike), Swarmer (alarm_weight +3). Skittish (flee) was a dead end.

PR #85 (merged 11-12 00:17Z, a64c412), after the owner asked for two reviews (20:11Z) and "make sure you work is being integrated into the game" (20:12Z):
- `mould::tiers::Tier` with apply(&mut BigParams) and settings(&mut gpu::Settings) (refuses until the GPU port, W3a); `bigdish tier=`, `strike_pile=N`.
- squad::shots lays SHOT_ALARM when the alarm field is on (before, game shots laid none).
- World race re-test (charge 512, 5k in 50k, 8 seeds): Swarmer 8/8, Sleeper bait 8/8, Sporer n.s. (7/8, p 0.07). The Sleeper is a nest (1-4 crystals held).
- Free sleep froze dried-out worlds; sleep paying upkeep lost the bait (dead end); `sleep_death` 1e-4 kept.
- Reviews: Reports/mould-tiers-review-2026-11-11.md. Review 2's top candidates: supply lines, habituation, clans.

Next: the player-AI thread took the counter tests (11-12 00:18Z, after its world-AI PR): each Tier against its five styles plus idle and random. The GPU thread ports sleep_death and the shot alarm in W3a.

**Why:** owner wants creatures with a body, a reaction to attack and a learnable weakness, judged by play in the game's world, not on a test dish.
**How to apply:** test new tiers in the world race with growth on, not the fixed-count dish; counters against equal-cost random actions. Related: [[mould-shooter]], [[larder-world-merge-order]], [[independent-reviews]].
