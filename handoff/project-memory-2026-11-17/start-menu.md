---
name: start-menu
description: Larder World start menu (PR #207, merged 2026-11-14): world::start widget, bin menu.rs, menu=0/1/PAGE flags, options list, Edit maps
metadata:
  type: project
  modified: 2026-11-14T01:55:36.169Z
---

Built 2026-11-14 (cloud plan C2 / decision D5), merged in #207 at 5597cc4.

- Widget: `src/world/start.rs` (`StartMenu`, rules-free like the HUD). Bin wiring: `src/bin/larder-world/menu.rs`.
- `larder-world` with no args or `menu=1` opens on the menu over a live sandbox round; any other args start the round at once; `menu=0` never; `menu=PAGE` + `look=DIR` films a page.
- Options: map, own, rounds (GPU-castable loadouts: no germ), bullet, panic, wither, energy_flow. Defaults come from shape C's keys and `Rules::default()`. An option at its default adds nothing, so missions keep their own mould.
- Missions are read from the mission files; switch to `Mission::list` when the campaign thread lands it.
- Edit maps launches `larder-edit` (#209) beside the binary.
- F1 or ? opens the key card in play. `MenuItem::MainMenu` is on the Esc menu and the end screen.
- Pictures page: https://claude.ai/artifact/C6wF9cqX9EVFPQLnUAyVhS

Left from D5: a key card trimmed to a mission's unlocked verbs.

**Why:** the owner couldn't playtest; the coordinator asked for this from the plan in #205.
**How to apply:** new switches go in `options()` in menu.rs with the value they default to; new options should add nothing to the round at their default. Related: [[larder-campaign]], [[world-ui]].
