---
name: squad-movement
description: Squad movement rework 11-16 (steer.rs, Rules::steer on): velocity steering, one map per squad, drawn between steps; thread, numbers, open items
metadata:
  type: project
  modified: 2026-11-16T11:35:34.771Z
---

Owner 2026-11-16 05:17Z: squad movement "still clunky overall", start over. Thread "Make squad movement feel smooth" (cmsg_01PcwPdssrGxiXYvyCG8WaUX4uE2KRVhL3PF2znGuNZZ1c), branch claude/project-thread-k03fc6. PR #392 merged 11-16 12:32Z. Page https://claude.ai/artifact/MWtDtnMANViS1NytZLwaNB. Owner 05:18Z allowed 1-3 independent review subagents; brief kept at Reports/squad-movement-2026-11-16/review-brief.md.

What it is: `src/mould/larder_world/steer.rs` replaces walk + elbow_room in squad mode (`Rules::steer`, default on; `set=steer=0` / `MOVE_STEER=0` for the old walk). Soldier vx/vy and `was`; window draws via `Game::drawn_between`. Anchor eases into goal (EASE), soft governor on lag (incl. `steer_short`), arrives when members stand or have all but stopped. rank_squad uncrosses places (UNCROSS) in steer mode.

Harness: `move_tests.rs` (`measure_moves`, MOVE/MOVE_NS/MOVE_SEEDS/MOVE_STEER; trace_move, trace_settle); `raw/measure.sh`, `raw/summary.py`, films `raw/films.sh` (film.rs, llvmpipe, needs a main worktree with film.rs patched).

Result (final, 11-16): planet jerk 23/24 better, turn 24/24, waits 24/24, cost 21/24; 200 open ground ~5 -> 0.9 ms. Worse: line settle 1-4 s, planet1 200 settle 22.8 vs 14.8 s, planet8 200 p95 7.66 -> 8.40 ms.

Open: places fitted to the ground (review's first pick), big squads in caves, ring/wedge of 200 are 160 cells across, wall through a standing squad seed 8 leaves 3 builders past it (home-side spot in rock; walls thread's code).

Related: [[squad-control]], [[levelgen]] (map cache owner), [[wall-sides]], [[walls]].

Doors (walls thread, PR #395, 11-16): soldier-feet checks use `Game::ground(x, y)` (world.open OR a door cell), not world.open; standable, paths.rs shut check, steer.rs want(), squad_turn dest/anchor checks already switched. New foot checks must use ground().
