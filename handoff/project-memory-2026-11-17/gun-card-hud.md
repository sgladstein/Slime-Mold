---
name: gun-card-hud
description: Larder World gun-card HUD strip and HUD layout pass merged in PR #233 (2026-11-14 06:37Z); where it lives and the numbers
metadata:
  type: project
  modified: 2026-11-14T06:38:03.355Z
---

PR #233 merged 2026-11-14 06:37Z (asked for by the owner at 01:49Z: "no way to see what the gun cards are in the interface").

- `world::ui::Loadout` / `GunCard` / `CardKind` are in the Model. The strip sits on the selection panel's top edge: the cards in firing order, the next cast lit, a recharge bar, and tooltips. H or its button sends `NEXT_ROUND` (command id 28 in the `commands!` table).
- Layout pass (art director, 02:34Z): the bottom panels take about 15% of the screen (120/800 px, 160/1080 px). The selection panel collapses when nothing is selected. `Hud::insets` feeds the camera so the opening view centres above the bar.
- Page: artifact 61NM34LrFKEwgkrSrju3mX; its source is in Reports/gun-card-hud-2026-11-14/.
- Still open: the owner's eye on the look. Related: [[gun-rounds]], [[world-ui]].
