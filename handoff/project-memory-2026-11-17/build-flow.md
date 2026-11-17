---
name: build-flow
description: Build flow thread (11-14): reach 6, near side, Shift queue (#255 merged); cancel/refund + reach ring (#268 merged)
metadata:
  type: project
  modified: 2026-11-14T08:48:36.165Z
---

Owner 2026-11-14 08:10Z asked for build queueing, builders that start once in range, and soldiers that stay on one side of a turret raised near the enemy. They also asked for a review of the rest of the build flow. Thread cmsg_01PcwPdssrGxiXYvyCG8WaUXXm53dh4nBUpSGTQnqJjs7z.

PR #255 (merged 11-14 11:53Z):
- `BuildRules::reach` 6: a builder walks straight at the site and stands once in reach (`Game::build_band`, `at_site`, the Order::Build arm of goal_raw). It no longer takes a sunflower place round the ghost.
- In the window, `builders()` queues on `App::last_sent` when Shift is held and nobody is selected. In squad mode it sends `SquadOrder(q, Build)`.
- Squads: a squad sent to a dest keeps its members' `queued`; steering or a new SquadOrder move clears it.

Measured: far side 2/8 -> 0/8. Build time unchanged. Clip hand `hand=buildq`.

Report Reports/build-flow-2026-11-14/. Page https://claude.ai/artifact/4muCEiRcRLnM8WGhc421cF

PR #268 (merged 11-14 12:18Z): `Cmd::Unbuild`/`Game::unbuild` gives a full refund for a ghost, the build page has CANCEL_BUILD 29 (C, slot 2,0), and a reach ring shows while placing. Queue numbers on ghosts were skipped because the order lines already chain through the ghosts.
Merge-queue lesson: Reports/README.md, pages.md and tests.rs conflict with every PR ahead in the queue. For tests.rs, take main's file and re-append your own block rather than merging both sides.

Related: [[soldier-controls]], [[larder-buildings]], [[squad-control]].
