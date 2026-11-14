# The loadout panel and a smaller HUD (2026-11-14)

The owner's playtest, 2026-11-14 01:49Z: "I also have no way to see what
the gun cards are in the interface." This is step 6 of the rounds' look plan
([`noita-spell-look-2026-11-14.md`](../noita-spell-look-2026-11-14.md)): a
panel in `larder-world`'s HUD that shows the squad's loadout as a spell deck
does. Checked on the cloud's software Vulkan (llvmpipe).

Page: https://claude.ai/artifact/61NM34LrFKEwgkrSrju3mX (source `page/`):
a film of the Lull round from 5 to 12 s, two tooltips and four more
loadouts, from `larder-world look=DIR hand=stand rounds=NAME bank=5000
at=9 screen=1920x1080` (and `hover=X,Y`, new here, for a tooltip). The
lit cast and the recharge read `Rounds::gun` (the gun rounds thread, #219);
the payload colours are its `card_look` glows. Card words come from the
window with the round's own numbers, not `Card::about`, so a changed
radius or delay shows in the tooltip.

## What it shows

- **Where:** a one-row strip, 50 pixels high at 1080 lines, on the
  selection panel's top edge at its right (on the command card's when the
  window is too narrow). The first cut, the same day, was a 140-pixel box
  over the command card with each card's name under its glyph; the art
  director's review (11-14 02:34Z) asked for a strip of card icons next to
  the selection with the details in tooltips, since the bottom HUD is
  already too tall.
- **Beside the cards:** the loadout's name (Lull, Trap, ...) and its state,
  and a button with its key, H. Hovering the name shows whose gun is shown
  ("Soldier 3"; with several selected the first of them, "Soldier 1 (of
  4)"; with none, the first alive, "(squad)") and how many guns are ready. The button
  sends the new command `NEXT_ROUND` (28, at the end of the `commands!`
  table), which does what H does (`App::next_round`).
- **Cards**, left to right in the gun's four slots (`rounds::SLOTS`), empty
  slots drawn empty. Each card has its glyph (ten new icons, `Icon::Blast`
  to `Icon::Split`), its name, and a stripe and frame by kind: a payload's
  own colour, modifiers blue, triggers amber, flight cards teal. A modifier
  or trigger's glyph takes the colour of the payload it goes with, so a
  cast reads as one colour; the payload colours match the bullets (blast
  orange, gas violet, germ green).
- **The next cast is lit:** the run of cards the shown soldier's next hit
  fires (its modifiers, trigger and payload; `rounds::casts`) glows with a
  notch above it. Cards after the last payload are dimmed: they only slow
  the recharge. Flight cards ride every bullet, so they are never lit as a
  cast and never dimmed.
- **Recharge:** a thin bar under the cards that fills back up after the
  last cast, and "Ready" or "Recharging 1.2 s" beside them.
- **Tooltips:** hovering a card shows its name, kind, price a cast and what it
  does with the round's own numbers (radius, delay, crowd size), and says
  when the world cannot cast it (germ on the GPU) or when it only slows
  the recharge.
- **Plain shots:** with no cards the strip still shows, four empty slots,
  "Plain shots" and the H button, so the loadouts can be found.

The HUD keeps no rules: the game fills `ui::Loadout` (`Model::loadout`)
from `Rounds` each frame (`App::loadout`, `gun_cards` in the window's
`draw.rs`), and the HUD hands back the Swap as a `HudEvent::Command`.

## Checks

- `world::ui` tests: the Swap button gives its command, a card takes the
  click (not the world) and draws its tooltip; the panel draws at
  1920x1080, 640x360, 200x120 and 3840x2160, with no cards, and with more
  cards than slots. `world::icons`: every new glyph draws.
- `larder-world` test `the_loadout_panel_lights_the_next_cast_and_dims_dead_cards`:
  the lit run, modifier colours, prices, dead cards after the last payload,
  flight cards, an uncastable payload, and every loadout H cycles.
- Frame cost: the model and HUD layout went from 0.058-0.087 ms to
  0.070-0.114 ms a frame on the cloud CPU (3 runs each, `look=` with
  `hand=rush rounds=trap at=15`, 1280x800; measured on the first cut, the
  strip draws less), about 0.01 ms.

## The HUD layout pass

The art director's second ask (11-14 02:34Z, and 02:41Z: check 1280x800
as well as 1080p, "that's where the bar was 20% of the screen"):

- The command card's buttons, gaps and padding went from 58, 6 and 10 UI
  units to 42, 5 and 8 (`CELL`, `CELL_GAP`, `CARD_PAD` in `world::ui`).
  The bottom panels (selection, command card, minimap, one height) now
  take 160 of 1080 pixels (14.8%; was 214, 19.8%) and 120 of 800 (15.0%;
  was 160, 20%). Glyphs and hotkeys shrank with them (icon 28 to 22,
  hotkey 11.5 to 10 units).
- With nothing selected and no detail card the selection panel collapses
  to a 40-unit strip along the bottom ("Nothing selected", click or drag
  to select), with the loadout strip beside it on the same row.
- `Camera::centre_on` centres a point in the band between the HUD's
  insets, not the screen's middle, and a new game's camera takes the
  insets from `Hud::insets` before the HUD has first drawn, so the larder
  opens in that band.
- `select=0` films the no-window pictures with nobody selected.

Checks: `world::ui` test `the_bottom_panels_cover_at_most_about_15_percent_and_collapse_when_empty`
(1920x1080, 1280x800, 2560x1440: at most 15.2%, `Hud::insets` matches the
drawn layout, the selection panel collapses); `world::camera`
`centre_on_puts_the_point_in_the_band_above_the_panels`; `larder-world`
`the_opening_camera_frames_the_larder_above_the_bottom_panels` (at
1280x800 and 1920x1080 the larder opens at y 261 and 353, inside the
band). Before and after pictures at both sizes, selected and not, are on
the page.

