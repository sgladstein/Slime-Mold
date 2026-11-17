---
name: ui-size
description: Larder World UI/text size setting (owner 2026-11-14 08:07Z): world::settings::UiSize, Ctrl +/-, Options row, ui= arg, settings.txt; covers HUD, menu, editor
metadata:
  type: project
  modified: 2026-11-14T08:22:37.906Z
---

Owner asked 2026-11-14 08:07Z: "We need an option to scale the UI/text etc, because they look too small on my screen." Built in thread cmsg_01PcwPdssrGxiXYvyCG8WaUX2GbpNZoGc7M8Ls1Pa7u8dZ (branch claude/project-thread-hsi86z).

- `src/world/settings.rs`: `UiSize { user, dpi }` on `Canvas::ui`; `UiSize::px(screen)` is the one scale for HUD, start menu, `Hud::insets(screen, ui)`. 100% on dpi 1 = old height-based scale exactly. DPI >1 is a floor; capped at min(h/540, w/1000). Editor uses `UiSize::fixed()`.
- Saved `ui=150%` in `%APPDATA%\Larder World\settings.txt` (or ~/.config/larder-world; `LARDER_SETTINGS` overrides). Pictures (look=/bench=/demo=) ignore the saved value.
- Controls: Ctrl +/- (game, menu, editor), start menu Options last row "UI and text size", `ui=150%` arg. Bin global in `src/bin/larder-world/uisize.rs`.
- Editor text now Inter via `hud::draw_text_inter`.
- Merged in PR #250 (2026-11-14 09:26Z), together with the bottom-bar layout: the owner asked at 08:33Z for one squad row and the commands on two rows (`CARD_COLS` 6 x `CARD_ROWS` 2, `card_slot`, bar 160 -> 113 units), and at 08:46Z for the gun cards as their own panel at the far left (`loadout_w`), with the selection filling the middle. The squad thread owns the squad tiles in that middle panel.
- Page: https://claude.ai/artifact/PdKeUiFDit4v4KkciRCwAx; report Reports/ui-size-2026-11-14/.

**How to apply:** new HUD/menu drawing must use the canvas's `c.ui.px(c.screen)` (via the `s` the draw passes), never its own height scale. Related: [[world-ui]], [[start-menu]], [[gun-card-hud]].
