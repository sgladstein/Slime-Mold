---
name: throw-aim
description: Throw aiming ghost for tools (owner 11-16 05:04Z ask): Model::throw / Throw, Kit::thrower; PR #379 merged 11-16; page YRsCKRrkXTesoCEyC782LC
metadata:
  type: project
  modified: 2026-11-16T05:42:18.629Z
---

Owner 2026-11-16 05:04Z: "when throwing an explosive, we should improve the visual for aiming and range." Thread "Improve explosive throw aiming visual" (cmsg_01PcwPdssrGxiXYvyCG8WaUX1qDfpawUKtqQc4qtDFLTx3), branch claude/project-thread-cpl5yh, PR #379 MERGED 11-16 05:51Z.

Built: `Model::throw: Option<Throw>` (src/world/ui.rs; drawn in `Hud::ghost` via `throw_aim`): a thrower ring, a dashed cream reach ring, a lobbed arc (`ARC_RISE` 0.28), and the area in the tool's colour with crater and alarm rings. Out of reach it all turns red, a cross marks the reach edge, the note says "N cells out of reach", and the click is refused. Filled by `App::throw_ghost` (larder-world draw.rs) from `Kit::thrower`. Headless: `hover=X,Y,X2,Y2` sweeps the cursor; `hand=throw` / `throwgas`. Report Reports/throw-aim-2026-11-16/, page https://claude.ai/artifact/YRsCKRrkXTesoCEyC782LC. Gun thread told: step E devices can fill a Throw.

Awaiting the owner's look. Not done: no grenade flight (instant), 1070 untimed. Related: [[squad-toolkit]], [[gun-system-review]].
