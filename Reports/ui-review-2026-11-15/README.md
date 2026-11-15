# Menus, shop and HUD review (2026-11-15)

The owner, 2026-11-15 02:56Z: "I want someone to review all of our menus,
shop, hud. The text seems too small. I feel like we're not always making the
best use of space."

Page: see `Reports/pages.md` (source `page/`): every start-menu page, the
run's shop and the bottom bar before and after, clips of the bar in play and
of a card dragged in the shop, the bar at 150% and 200%, and a 1280x800
window.

## Findings (big picture first)

- **Type.** The UI unit is a pixel at 1080 lines and most words were 9 to
  12.5 units; the bar drew at 92% of that (`BAR`), so its key letters and
  costs were 8.7 px. Ctrl + grows boxes and words together, so the ratio
  (small words in big boxes) stayed. The fix is the ratio: body text 13 to
  15 units, nothing on the bar under 11.5.
- **Space.** The bar's middle panel was ~1,100 px of empty glass holding one
  66 px squad tile, while the command card held 12 unlabelled icons in
  290 px. The shop had width to spare and ran out of height; it shrinks to
  fit (`shop_page`'s loop), so taller cards made every word smaller.
- **Hierarchy.** Section labels and hint lines were the faintest grey (the
  disabled colour); option notes were cut at half the row; the key list's
  lines ended in "…".
- **Overlap.** The corner minimap drew over the shop's armory and the menus.

## What changed

- Command card: buttons 116 x 42 units with the name beside the icon
  (`CELL_W`, `short_label`, `split_label`); square buttons come back when the
  selection would keep under `LABEL_ROOM` (340) units, so 150% and 200% on
  1080 lines look as before with bigger keys and costs. `BAR` 0.92 -> 1.0
  (bar 104 -> 113 px at 100%; it stays one row at every zoom).
- Squad tiles up to 136 wide with the squad's state; loadout slots 34 -> 46;
  headers, chips, tooltips, alerts, the paused menu and key card 1 to 1.5
  units bigger.
- Start menu: buttons 18/14 (were 17/12.5) and 500 wide; options panel 1.4x
  wide with notes to the value column; keys 2.5x wide at 15; hints MUTED.
- Shop: tiles 60 -> 78 with names at 12.5 or on two lines; bullet cards
  78 x 98 portrait -> 184 x 64 landscape (`CARD_W`, `CARD_H`); gun-card
  numbers 10 -> 12.5; detail 12 -> 14; section labels MUTED; the layout
  starts at 1.3x the UI size (was 1.2x) and shrinks to fit as before. Keeps
  the run thread's steady-bag rule (#316).
- The view hides its unframed minimap while a full-screen menu is up
  (`View::minimap_off`).

Frame cost (CPU, model and HUD, run planet 1 at 4 s, squad mode, llvmpipe
box): 0.130 ms before, 0.155 ms after.

Tests: `cargo test --release` all pass (the squad tiles narrow back to 72 on
a 640x360 window so the existing tile test holds); clippy clean.

## Commands

Stills: `larder-world look=DIR screen=1920x1080 at=1 menu=main|campaign|options|keys`,
`run=1 menu=shop-mid|shop-build at=1`, `run=1 mode=squads select=1 at=4 [ui=150%|200%]`,
`at=4` (RTS), `select=0 at=4`, and `screen=1280x800`. Clips: `at=2,2.25,..,8.75`
with `run=1 mode=squads select=1`, and `run=1 menu=shop-mid "drag=Needle bullet>1"
at=1,1.25,..,6.75`. Pictures and clips by `raw/make_pics.py`.

## Next (not in this PR)

- Below 1080 lines the UI scales down with a 0.7 floor (`UiSize::px`); at
  1280x800 words are ~10 px. A higher floor or default needs the owner's pick.
- The squad panel could show the active squad's orders and gun.
- Card wording is the gun thread's (owner 02:57Z: too much text).
- The map editor's panel (`UiSize::fixed`) was not changed.
