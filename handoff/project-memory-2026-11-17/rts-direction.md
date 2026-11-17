---
name: rts-direction
description: RTS work on the mould game, 2026-11-10: design (Bloomfront, brainstorm, red team, field retest, verb census, comeback, layout sweep), where it lives, owner rulings
metadata:
  type: project
---

On 2026-11-10 the owner asked how the mould sim could become an RTS. Design lives in the Claude Doc https://claude.ai/artifact/EDygsdTpEg8vK7HZzTXhdp, `Reports/rts-design-2026-11-10.md` and its folders `brainstorm/`, `red-team/`, `field/` (PRs #10, #18, #22, #26); wiki `wiki/rts/directions.md`.

Findings that stand: food share mostly counts heads on piles; the food store sits at 0 (`Params::pieces`), so spending needs its own currency. The Wild One (7 piles) is decided by the opening and no verb moves much. On the 23-pile mirrored "field" (~6,600 walkers): the habit cycle hunt > maze > cord > hunt holds on the scatter layout only (1 of 10 layouts; usually cord > hunt > maze); Taut's pull wins share (a stat); the 60 s leader keeps the lead ~80% with no hands; a salt wall across the leader's road overturns the opening on scatter and the symmetric Wild One, not scatter2; the lamp and the cut never do.

Owner rulings: 17:52Z readability "can be solved", no readability probes yet. 18:15Z brainstorms start from the model, not the demo tools. 18:25Z card-draw luck is not ruled out. 19:24Z "only 7 food sources... limits the potential options" (led to the field). 20:33Z "lets focus on actually building playable demos". 20:42Z card "Both, in two threads": the skirmish here, Two Fevers in thread cmsg_01PcwPdssrGxiXYvyCG8WaUXVoWg8bedVyDjYZSi7o5psh.

**How to apply:** the RTS is now in the build phase; see [[rts-skirmish]] for the demo and the owner's playtest. Don't re-run design probes unless they are cheap and parallel. Related: [[web-build]], [[runner-threads]].

Outside RTS research (2026-11-10, PR #34): `Reports/rts-research-2026-11-10.md`, wiki `wiki/rts/rts-craft.md`, 22 cards in `research/sources/`. Verdict on both demos: dominant strategy plus unmanaged snowball; fix structure first (size cost, pull-back rule, points ending, telegraphed attacks, exploiter search). Feedback sent to both demo threads.
