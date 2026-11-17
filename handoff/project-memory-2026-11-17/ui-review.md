---
name: ui-review
description: 11-15 menus/shop/HUD thread: #324-#363; owner NO SCROLLING; Shop B + Menu A built (#363); Card 2, Burn removed, Guard, building select in their own thread
metadata:
  type: project
  modified: 2026-11-15T19:53:12.269Z
---

Thread cmsg_01PcwPdssrGxiXYvyCG8WaUXDJ58rTuLEuNfxu3qt3aNLi owns menus, shop, HUD layout, text size, UI scale, hover overlays.

- #324 (Reports/ui-review-2026-11-15): named command buttons 116x42 (`CELL_W`), squad tiles show state.
- #334: bar is ONE row at every UI size; hover card must not block shooting; every screen follows the UI size.
- Review (Reports/menu-review-2026-11-15): open owner questions: Restart in a run, open window maximized, 125% default on 1080p, confirm Give up/Quit. #341, #348 (hover card shut while firing; `Command::locked`).
- #353 (Reports/bar-allocator-2026-11-15): `Hud::bar_fit` starts bare, then gives room in order: labels, squad keys, gun empty slots, Swap, gun name. 900-case test `the_bottom_row_keeps_its_text_at_every_size` checks glyph pixels via test-only `Canvas::runs`.
- Owner 11-15 17:31Z: scrolling menus/shop unacceptable; every page fits one screen at every UI size (overrides #334's "long pages scroll"). 17:41Z bullet tiles half size.
- Research (Reports/menu-layout-research-2026-11-15, #360, page https://claude.ai/artifact/Su1g5MdjbDU2P6ic8YhoLT). Fit sizes 1000x562 / 1280x540 / 1280x720 / 1920x1080 units.
- #362 (Reports/one-screen-menus-2026-11-15): Options tabs (Opt::in_tab; menu.rs assigns Game/Planet/The mould), Keys last tab, campaign rows + pane, paging (Hit::Flip) not scrolling. Tests: lib `the_inner_pages_fit_one_screen`, bin `the_menus_real_lists_fit_one_screen`.
- Owner picks 11-15: 19:37Z Shop B (tabbed counter) + Menu A (names + caption box); 19:38Z Card 2 (one cycling fire button, 5x2, Pump on card, Heal/Guards on card, walls on Build). 19:51Z "Remove burn line. Use options c for guard/defend. (b) Click a building to select it when no squad is selected." = Burn a line removed; Guard order on F (click spot/building/crystal, squad keeps slime off it; Defend stays default fire mode; squad "Guards" count renamed "Lookouts"); with no squad selected a click selects a building (panel: health/level/state; card: cancel, demolish, upgrade pump, switch lure). Demolish refund unanswered (recommended half price).
- Split 19:55Z: this thread does Menu A (local c461fec6) + Shop B + armory; asked coordinator for a parallel thread for Card 2 + Burn removal + Guard + building select (HUD/squad code). Guards/I key: check thread cmsg_01PcwPdssrGxiXYvyCG8WaUXAjpdUFZKCE99TBg7S7Mwpo (#361).
- #363 merged 11-15 20:42Z (Reports/shop-b-2026-11-15, page https://claude.ai/artifact/Sv62LV9qkAgWrr1GUZSr9e): Menu A + Shop B. Shop code in src/world/start/shop.rs (`shop_page`, tabs `shop_tabs`, pane, `Hit::Do/ShopTab/BagFlip`, `ShopRow::cell` for the bullet grid). Films: `menu=shop-mid@TAB@NAME` (names with spaces can't be passed). Left: empty bag outlines read as blank slots; 21:9 armory guns 2 pages.
- Still unanswered: Build+Tools on one page; mould rules stay in Options?; step 3 goal bar + stars.

**How to apply:** a new bar part enters `bar_fit`'s order and `sel_needs`; run the 900-case test. HUD text >= 10.5 units (target 12+), MUTED not FAINT. A test that turns the UI size sets LARDER_SETTINGS. Review renders come from a binary built at HEAD. Related: [[steady-hud]], [[gun-card-hud]], [[squad-control]].
