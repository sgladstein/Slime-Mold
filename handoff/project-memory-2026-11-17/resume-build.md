---
name: resume-build
description: One click resumes a half-built building (owner 11-15 07:50Z); PR #332 merged 11-15 09:00Z; Game::unfinished_at + App::resume
metadata:
  type: project
  modified: 2026-11-15T08:14:20.122Z
---

Owner 2026-11-15 07:50Z: "If you start building something but have to stop you should just need to click again to get the squad to resume building". Thread cmsg_01PcwPdssrGxiXYvyCG8WaUXRViQPtZqHHVBWpY5t2hM6Z, branch claude/project-thread-fzevje, PR #332 (merged 11-15 09:00Z).

Right click on an unfinished building (or the same build card's click on it again; pump card on a rig) -> `App::resume` (src/bin/larder-world/input.rs) gives the controlled squad Order::Build (Shift queues); RTS: selected soldiers else Cmd::Send. `Game::unfinished_at` (buildings.rs) = ghost_at or the unfinished pump rig on the crystal under the click. No new placement, nothing paid again; Ctrl+right click still drains. No new command id.
Page https://claude.ai/artifact/RtqbiucG1oQVpTm3MgFzZm; report Reports/resume-build-2026-11-15/ (film helper raw/film.rs).

Related: [[build-flow]], [[squad-control]].
