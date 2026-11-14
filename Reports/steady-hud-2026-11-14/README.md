# Steady bottom bar (2026-11-14)

The owner, 2026-11-14 20:19Z: "I don't like that the bottom hud menu changes
totally when you zoom in because of the minimap, shows up. The minimap should
always be visible and everything could be made slightly small to fit in the
same row with the same height." And 20:23Z: "changing the UI zoom, doesn't
change the main menu".

Page: see `Reports/pages.md` (source `page/`): the bar's bottom 30% before
and after, zoomed out and in, at 100%, 150% and 200%; the start menu at 100%
and 150% before and after. Frames from `larder-world look=DIR at=4
screen=1920x1080 mode=squads select=1 ui=N%` (the `-all` and `-near`
pictures) and `menu=main at=1`, on the cloud's llvmpipe.

## What changed

- **The minimap is always shown** (`main.rs` no longer hides it once the
  camera sees the whole world; `View::minimap_rect` shows a HUD-framed
  minimap at every zoom, an unframed one, mouldgpu's, still hides). M still
  toggles it.
- **One row, one height.** The minimap's frame is the bar's height (it was
  152 units, standing above the 105-unit bar) and at most 300 bar units
  wide. The gun cards, selection and squad tiles, and command card draw at
  `BAR` = 92% of the UI unit; the top bar, alerts, tooltips and menus keep
  the full unit. The bar is 104 of 1080 pixels at 100% (was 113), 156 at
  150%. `Hud::insets` follows.
- **The start menu follows the UI size.** Every page was held to the
  Options page's height (`330 + 62 x options`), which with a dozen options
  is about 1 on 1080 lines, so Ctrl + did nothing on the main page. The main
  page and the campaign list are now held by their own rows
  (`330 + 74 x rows`); Options, Keys and the shop as before.

Tests: `the_bottom_bar_and_minimap_stay_put_at_every_zoom` (bar top,
minimap box and minimap equal at the fit and at zoom 24),
`the_main_page_follows_the_ui_size` (150% is over 1.2x 100%, 75% smaller,
Quit on screen, four screens). Existing UI-size tests pass unchanged; the
collapse test's "full panel" height went from 100 to 90 units.

Frame cost: no new per-frame work (same panels, the minimap pass already
ran when zoomed in; zoomed out it now runs too, the view's minimap is one
sample a pixel of its ~170 x 80 box).
