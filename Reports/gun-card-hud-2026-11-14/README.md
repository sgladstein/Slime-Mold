# The loadout panel: gun cards in the HUD (2026-11-14)

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

- **Where:** on the command card's top edge, bottom right, the same width.
- **Header:** the loadout's name (Lull, Trap, ...), whose gun is shown
  ("Soldier 3"; with several selected the first of them, "Soldier 1 (of
  4)"; with none, the first alive, "(squad)"), and a **Swap** button with its key, H. The button
  sends the new command `NEXT_ROUND` (28, at the end of the `commands!`
  table), which does what H does (`App::next_round`).
- **Cards**, left to right in the gun's four slots (`rounds::SLOTS`), empty
  slots drawn empty. Each card has its glyph (ten new icons, `Icon::Blast`
  to `Icon::Split`), its name, and a stripe and frame by kind: a payload's
  own colour, modifiers blue, triggers amber, flight cards teal. A modifier
  or trigger's glyph takes the colour of the payload it goes with, so a
  cast reads as one colour; the payload colours match the bullets (blast
  orange, gas violet, germ green). A payload shows its price a cast.
- **The next cast is lit:** the run of cards the shown soldier's next hit
  fires (its modifiers, trigger and payload; `rounds::casts`) glows with a
  notch above it. Cards after the last payload are dimmed: they only slow
  the recharge. Flight cards ride every bullet, so they are never lit as a
  cast and never dimmed.
- **Recharge:** a bar that fills back up after the last cast, "Ready" or
  "Recharging 1.2 s", and with several soldiers "4 of 6 guns ready,
  recharge 5.0 s".
- **Tooltips:** hovering a card shows its name, kind, price and what it
  does with the round's own numbers (radius, delay, crowd size), and says
  when the world cannot cast it (germ on the GPU) or when it only slows
  the recharge.
- **Plain shots:** with no cards the panel still shows, four empty slots
  and "no cards: Swap for a loadout", so the loadouts can be found.

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
  `hand=rush rounds=trap at=15`, 1280x800), about 0.01 ms. The canvas pass
  draws about 40 more primitives.
