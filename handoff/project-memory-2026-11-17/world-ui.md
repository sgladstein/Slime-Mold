---
name: world-ui
description: Larder world game UI (PR #80, 2026-11-11): rules-free HUD (Model in, HudEvents out) on a GPU canvas pass; open items
metadata:
  type: project
  modified: 2026-11-11T20:57:37.978Z
---

The game UI for the Larder world was built 2026-11-11 after the owner asked for soldier health, numbers, controls, weapons, upgrades, walls, menus and enemy info, "modern and professional". It is in PR #80: `world::ui` (Hud), `world::canvas` (GPU pass), `world::icons`, and `world::demo` (the stand-in squad).

- The game fills `ui::Model` each frame and handles `HudEvent`s.
- Input goes to `hud.button` and `hud.key` before the controls.
- Report: Reports/world-ui-2026-11-11.md. Page: https://claude.ai/artifact/3ramepC25K9cQJYC6BUS53

**Why:** an independent agent reviewed it, as the owner asked, and found input bugs, which were fixed. The 1070 timing found that a single big mark (a 480-pixel ring) costs the view 0.7 ms.

**How to apply:**
- Draw big outlines as line marks.
- Still open from the review: Shift-queued moves (promised to the owner, not built), command pages or submenus, selectable buildings, a production queue, minimap unit blips, and a settings menu.

Related: [[larder-world-merge-order]], [[independent-reviews]].
