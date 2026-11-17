---
name: run-progression
description: Run and enemy progression thread (11-15): stuck rule, mass goal, play log, 1% goal; D3 does_to_you test + D10 shop (PR #372)
metadata:
  type: project
  modified: 2026-11-15T23:13:00.727Z
---

Thread "Build run and enemy progression" (cmsg_01PcwPdssrGxiXYvyCG8WaUX12j6m23UpsCXcWgfDD3Uwx, branch claude/project-thread-uwavsa), from the whole-game review ([[whole-game-review]]) N1 row in ROADMAP.

First pass PR #351 merged 11-15 13:45Z (report Reports/run-progression-2026-11-15/, page https://claude.ai/artifact/3UZ3fiFNWsRYgpH67dp7x4):
- `count_stuck`: not stuck while a soldier walked, a pump stands, a building goes up, or shots are free (price 0) and a crystal has charge; goal line warns from `STUCK_WARN` 10 s.
- `World::mass` (thickened walker twice); GPU `count` adds enemy dense walkers to `wstat[8]`, WSTAT_BYTES 48. Goal::Shrink uses it.
- Play log `larder-world-log-MAP-SEED.txt`, `mould::playlog`, `examples/playlog.rs`. Ask the owner for logs after playtests.
- Baseline: squad kills 7-40/s vs slime swings 10x that; mass 11x planet 1->6, kill rate 2.5x.

Owner 11-15 18:02Z: win goal 1% of peak (`WIN_SHARE` 0.01, PR #359 merged).

Owner 21:41Z D3 "test it out": `run::does_to_you(tier)` (PR #372 merged 11-15 23:29Z, page https://claude.ai/artifact/GEMb4jx9chFLC7nxmXkUZu, report does-to-you.md): Drifter/Trunk pump_smell 20 lure_r 15; Sprawler smell 4 smell_r 4; Swarmer panic_margin 25 + surge_haste 3; Sporer spore_burst 0.2 spore_alarm 0.3; Sleeper wake_alarm 0.3. Pitfalls: wake_alarm/spore_alarm are thresholds (higher = less); louder alarm_weight pulls slime to wounds, not squad. D10: no outpost/lure/siren in run (`run_hidden`). D7: no rush brake until owner plays. Sizes await owner playtest; a "cut supply" rule would go to the slime thread ([[slime-enemy-review]]).
