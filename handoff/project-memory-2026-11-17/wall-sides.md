---
name: wall-sides
description: Walls leave the squad on its own side (11-16, PR #374): Building::home, builders/guards on the home side, 4 s footprint wait
metadata:
  type: project
  modified: 2026-11-16T02:58:05.550Z
---

Owner playtest 2026-11-16 01:45Z: "some soliders always end up on the wrong side, trapped with the slime." Thread cmsg_01PcwPdssrGxiXYvyCG8WaUXUMdC6hNuxM1MN85rH3N8jz, branch claude/project-thread-rd2vor, PR #374 (merged 11-16 03:06Z).

Causes: guards posted 2 cells past a wall's first end toward slime; builders stood on whichever side they reached; shut-in soldiers walked out either side.
Fix: `Building::home` from `Game::wall_home` (less slime by 5+ walkers, else landing's side, else soldier majority); goal_raw Build arm sends a wall's builders to the home side (levelgen thread OK'd, paths.rs only that arm); a wall's guards keep Order::Build with cover=true (don't raise, full fire rate); last 0.5% waits up to 4 s (`WALL_WAIT_SECS`, timed in `Building::ready`) while someone is in the footprint.
Measured seeds 1-8: past the wall 9->0 (wall in front), 35->1 (through squad). Film hand `wallq` (mode=squads). Report Reports/wall-sides-2026-11-16/, page https://claude.ai/artifact/JsMqRiqL4vEqS1tSZerNnE.

Lesson: a Move/Fight goal is post + formation place (`place_at`), so a post behind a line can still land past it.

Related: [[build-flow]], [[larder-buildings]], [[squad-control]].
