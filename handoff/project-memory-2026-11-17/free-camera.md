---
name: free-camera
description: Squad mode "let go" (Esc / panel Let go): no squad controlled, WASD pans unpaused, AUTO tiles (PR #321, 11-15)
metadata:
  type: project
  modified: 2026-11-15T03:22:24.913Z
---

Owner 2026-11-15 02:58Z asked for a deselect so no squad is controlled and WASD moves the camera. Thread cmsg_01PcwPdssrGxiXYvyCG8WaUXWdZHt4xRnjqLx7F35qJFo9, branch claude/project-thread-uj9fim, PR #321 (merged 11-15 04:20Z).

Built: Esc (after placement cancel, before menu) or panel "Let go" -> Cmd::Active(u8::MAX); `SquadInput::free` in src/bin/larder-world/squadmode.rs; WASD pans via Controls::wasd_pan when `App::squad_free()`; Space/tile/number/click on squad retakes. Tiles show AUTO/YOU (86 px wide), map label "N auto" above topmost soldier. Bought soldier with none controlled joins lowest live squad. Page https://claude.ai/artifact/W49VVdkXFWNjMpuCv6f6w8; report Reports/free-camera-2026-11-15/. Awaits owner playtest. Related: [[squad-control]], [[order-lines]].
