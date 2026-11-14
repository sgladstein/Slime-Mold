# UI and text size (2026-11-14)

The owner, 2026-11-14 08:07Z: "We need an option to scale the UI/text etc,
because they look too small on my screen."

Page: https://claude.ai/artifact/PdKeUiFDit4v4KkciRCwAx (source `page/`):
the HUD at 100%, 150% and 200% (1920x1080, `larder-world look=DIR at=4
screen=1920x1080 ui=N%`), the options page at 150% on 1600x900, and the
map editor at 100% and 150%. Rendered on the cloud's llvmpipe.

## What changed

- **One number for the whole UI.** `world::settings::UiSize { user, dpi }`
  sits on the canvas (`Canvas::ui`); the HUD (top bar, alerts, card strip,
  selection panel, command card, minimap frame, tooltips, the Esc menu and
  key card), the start menu and `Hud::insets` all lay out in
  `UiSize::px(screen)` pixels a unit. Before, that was the window height's
  share of 1080 lines (0.7 to 2.4), and it still is at 100% on a 100%
  screen: every picture and test before this draws the same
  (`at_100_percent_on_a_plain_screen_the_size_is_the_old_one`).
- **DPI.** A Windows display scale over 100% (`scale_factor`) is a floor
  under the height's size: a 2560x1600 laptop at 200% gets 2, not 1.48.
  The window follows a move to another screen (`ScaleFactorChanged`).
- **The player's size** multiplies it: 75% to 300% in ten steps
  (`settings::STEPS`), from Ctrl + / Ctrl - (also the keypad's) in play, on
  the menu and in the editor, the Options page's last row (live; not a round
  setting), or `ui=150%` / `ui=1.5` on either binary's command line.
- **Caps.** The HUD's unit never passes `min(height/540, width/1000)` (the
  bottom bar stays under a third of the screen and its row inside the
  width; 200% on 1920x1080 is 1.9). The start menu also holds its size where
  the options page, its tallest, would run off the bottom: at 150% on
  1600x900 it ran off before this cap, hiding the size row itself. Tests:
  `every_ui_size_keeps_the_hud_on_screen`,
  `every_ui_size_keeps_the_options_page_on_screen`.
- **Saved** as `ui=150%` in `settings.txt` under `%APPDATA%\Larder World`
  (Windows) or `~/.config/larder-world`; `LARDER_SETTINGS=PATH` overrides.
  Both binaries read it. Pictures (`look=`, `bench=`, `demo=`) draw at `ui=`
  or 100%, never at a saved size, so they stay reproducible.
- **The map editor** panel (`map_edit::chrome`, `Overlay::ui`) scales by the
  DPI floor times the player's size (`UiSize::fixed`; it does not grow with
  the window). Its text moved from the crisp pixel font, which could only be
  drawn at exactly 2x, to Inter (`hud::draw_text_inter`, the game UI's
  face, glyphs cached a size), laid out on the same rows.

## Wider, shorter bottom bar (08:33Z)

The owner, 08:33Z: "re-arrange the bottom menu to be one row for the squad
and two rows for the other buttons so the whole thing is wider but
shorter". The command card is 6 x 2 (the 4 x 3 slots in reading order,
`card_slot`), the selection panel one row of cards and up to 900 units wide
(600 before), and the bar 113 units at 1080 lines instead of 160 (10.5% of
the screen, 15% before). The minimap keeps the old 152-unit height in the
corner. Then (08:46Z: "gun/card menu should go on the far left to fill
the rest of the screen") the gun-card strip became its own panel at the
bar's far left, and the selection panel fills the row between it and the
command card (no 900 cap); where that would leave the selection under 240
units the strip goes back over the panels. Pictures: the page's *Wider,
shorter bottom bar*.

## Cost

The HUD's draw is unchanged in shape count; a bigger size only rasterizes
glyphs at other sizes into the atlas once. The editor's CPU text caches each
glyph a size, so a redraw blends cached coverage.

## Not done

- No size row on the in-play Esc menu (the keys work there).
- At large sizes in a short window the editor panel's last rows run off the
  bottom.
