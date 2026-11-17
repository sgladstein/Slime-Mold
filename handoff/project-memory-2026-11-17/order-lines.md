---
name: order-lines
description: Squad order lines through the Shift queue + WASD pans while paused (PR #301 merged 11-14 21:13Z)
metadata:
  type: project
---

Owner playtest 11-14 20:12Z/20:15Z: line from selected squad to its order, every command incl. attack-move, whole queue visible; WASD pans camera while paused.
PR #301 (branch claude/project-thread-q9qd2m, thread cmsg_01PcwPdssrGxiXYvyCG8WaUXGeGb43TexfJr5M1vvbtG8h): `order_chain` in src/bin/larder-world/draw.rs (squad mode: active + linked squads, one chain from middle; classic keeps per-soldier lines); `Controls::wasd_pan` set from App::paused (events.rs + frame). Unpaused WASD unchanged (squad steer, classic card A/S/W/D).
Page https://claude.ai/artifact/4bbZ1NXACyVKnbXfxgkuXR; report Reports/order-lines-2026-11-14/. Film helper raw/film.rs (ignored test to append to tests.rs). Related: [[squad-control]].
