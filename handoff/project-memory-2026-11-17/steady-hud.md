---
name: steady-hud
description: Larder World bottom bar (owner 11-14 20:19Z): minimap always in the row at bar height, bar panels at BAR=92%; start menu follows UI size (PR #300)
metadata:
  type: project
  modified: 2026-11-14T21:00:56.107Z
---

Owner 2026-11-14 20:19Z: the bottom bar must not change with zoom; minimap always visible, everything slightly smaller, one row, one height. 20:23Z: Ctrl +/- must also scale the start menu.

PR #300 (queued): `world::ui` const `BAR` = 0.92 scales the bar panels (gun cards, selection/squad tiles, command card, minimap frame); top bar/tooltips/menus keep the full unit. Minimap frame = bar height, max 300 bar units wide. `start.rs` caps Main/Missions pages by their own rows, not the options count. Page https://claude.ai/artifact/MLYLxpTW8GsXutxV5RDbJ9.

**How to apply:** new bottom-bar panels use the bar unit `b`, not `s`; don't reintroduce zoom-dependent HUD layout. Related: [[ui-size]], [[gun-card-hud]], [[world-ui]].
