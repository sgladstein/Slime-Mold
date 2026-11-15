# Command card rebuilt: Card 2, Guard, a building's own card (2026-11-15)

The owner's picks from the menus research
([menu-layout-research](../menu-layout-research-2026-11-15/README.md)):

- **19:38Z, "Card 2".** One fire button that cycles, 5 x 2, Pump on the
  main card, Heal and Guards off the squad panel onto it, the wall on the
  Build page, no empty tile.
- **19:51Z, "Remove burn line. Use options c for guard/defend. (b) Click a
  building to select it when no squad is selected."**
- **19:53Z, "Build option 1".** The tidied Build page: what you place along
  the top, the switch and Back below; Build and Tools stay separate pages.

Page: https://claude.ai/artifact/9XmKNPSBSncdjAVQcfdjaR (listed in `Reports/pages.md`), source in
`page/`.

## What changed

**The card** (`src/bin/larder-world/draw.rs`, `src/world/ui.rs`).
`CARD_COLS` is 5; a command's `slot` is its own (row, column) on the 5 x 2
grid (it was the old 4 x 3 card read in order onto 6 x 2).

| | squad mode (the default) | RTS round |
|---|---|---|
| row 1 | Attack-move R, Guard F, Drain G, Fire X, Heal L | Attack-move A, Stop S, Harvest G, Guard F, Fire X |
| row 2 | Buy B, Pump U, Build Q, Tools E, Lookouts I | Buy B, Pump U, Rally R, Build Q, Tools E |

- **The fire button** (`FIRE`) shows the stance ("Fire: defend") and a click
  cycles hold, defend, free. Z, X and C still pick one: a button's `also`
  keys (`Command::also`) are tried before every button's own key.
- **Heal and Lookouts** moved off the squad panel's header (which keeps Let
  go, Combine and Split, about 150 units freed) onto the card
  (`HEAL_ORDER`, `LOOKOUTS`; the keys L and I as before). The squad's
  "Guards: none/some/more" is now **Lookouts**, so the word stays the Guard
  order's (the coordinator's note). The cover code (`cover.rs`, #361) is
  untouched; only the labels and where the button lives moved.
- **Build page, option 1:** Turret T, Pump U, Wall V, Lure L, Siren Y; then
  the lures' switch J, Outpost I (RTS with a larder only), the rally point
  (RTS with the own mould), Back Q. Cancel a build left the page: click the
  building. **Tools page:** Blast, Det cord, Mine, Germ, Sleep gas; Road
  and (squad mode with the own mould) Sow below; Back.
- The card is 122 units narrower labelled, which the bar's allocator gives
  back: at 125% on 1920 x 1080 the gun strip now keeps its name ("Rifle,
  Plain shots"), which it dropped before (`page/card-before.webp`,
  `card-after.webp`).

**Burn a line is gone** from Larder World: the card, F + drag
(`Controls::burn_lines`, off in this window), the key card and `App::burn`.
`Order::Burn` stays in the engine: the dish game (`larder.rs`), its AI,
`plans.rs` and the window's scripted `volley` hand still use it.

**Guard** (`Order::Guard`, `Squad::guard`, `Game::guard_zone`, `Game::zone_at`).
F, then a click on a spot, a building or a crystal. A squad walks there as
one (as a move) and keeps the zone till it is steered or given another
order; in the RTS round each soldier gets the order. Its members fire on
slime in the zone as well as their own defence: `Rules::guard_r` (8 cells)
round a spot, `GUARD_RIM` (3 cells) past a crystal's rim or a building's
footprint. Not on hold fire. A ring shows the zone. Defend stays the
default fire mode.

**A building selected** (option b). With no squad commanded (Esc) a left
click on a building selects it (`App::building_sel`; in the RTS round a
click that picks no soldier and no larder). The selection panel shows its
health, state and, for a pump, its level and draw; the squads' tiles come
back with a click on open ground, a number or Space. Its card:

- a ghost: **Call it off** (Del), the whole price back (`Cmd::Unbuild`);
- standing: **Demolish** (Del), half the price back (`Cmd::Demolish`,
  `BuildRules::demolish` 0.5). Half was the
  menus thread's proposal; the owner ruled it at 21:37Z ("Demolishing a
  building giving half back is good. Canceling a partial build should
  give all of it back"), which `unbuild` already does (tested on a
  half-raised turret). A wall can't be pulled down (its cells are the
  GPU world's; there is no way to lift them yet);
- a pump: **Upgrade** (U); a lure or siren: **Switch on/off** (J).

## Checks

- Tests: lib 516 (two new: `a_guard_shoots_slime_on_what_it_guards`,
  demolish in `a_ghost_called_off_...`), the window's 39 (new
  `a_building_selected_shows_its_card_and_acts`;
  `the_cards_guard_reaches_a_moving_squad` replaces the burn one), the
  window smoke tests, clippy. The 900-case
  `the_bottom_row_keeps_its_text_at_every_size` now runs on the real Card 2
  and Build page labels and passes (squad header: all three buttons drawn
  wherever the row's floor holds).
- **Guard against a plain move to the same crystal** (the squad mode's
  right click, `GUARD_MOVE=1`), `hand=guard`, eight soldiers, the
  slimed crystal nearest the squad, 30 s, seeds 1-6 on the Fork (raw in
  the thread's run; command below):

  | seed | walkers in the zone at 29 s, move / guard | killed, move / guard |
  |---|---|---|
  | 1 | 54 / 7 | 2422 / 2685 |
  | 2 | 573 / 585 | 670 / 677 |
  | 3 | 126 / 80 | 2317 / 3189 |
  | 4 | 76 / 88 | 2603 / 2984 |
  | 5 | 1180 / 1090 | 6168 / 6523 |
  | 6 | 23 / 73 | 2197 / 1739 |

  Guard killed more on 5 of 6 (sign test p = 0.22) and left fewer walkers
  in the zone on 3 of 6 (p = 1): it shoots more, but does not yet clearly
  keep a crystal clearer. Seeds 2 and 5 picked crystals 49 and 92 cells
  away, deep in the slime. A soldier still aims at the thickest slime it
  sees (`squad::sees`), so when that is outside the zone it holds, even
  with thinner slime inside; aiming into the zone first would need the
  GPU's aim to take a point. Build first, balance later: a tuning note.
- **Frame cost:** the HUD's model and layout 0.14 ms a frame with a guard
  ring on screen against 0.10 ms without (llvmpipe, 39 frames each, seed
  3); `guard_zone` is a crystal and building scan per soldier with a zone.
- Smoke: the card's every command is pressed by `hand=every` in the
  smoke tests; nothing soft-locks (the building card always leaves Esc,
  a number or a click on open ground).

## Not done

- The commanded squad's own stance is still forced to Defend (its guns are
  the player's, `cmd.rs`), so Fire: free shows on its button but only acts
  once the squad runs on its own. Unchanged from before.
- No drag-a-line Guard (the coordinator's note: not wanted).
- Sticky build (the research's take: not now).

## Commands

Frames (llvmpipe), `target/release/larder-world` at this change:

```
larder-world mode=squads select=1 hand=squads look=R/main screen=1920x1080 ui=125% at=12
larder-world mode=squads hand=guard look=R/fire screen=1920x1080 ui=125% card=main/fire/fire/fire/build/tools hover=1475,975 at=2,2.5,3,3.5,4,4.5
larder-world mode=squads hand=guard look=R/bsel screen=1920x1080 ui=125% pump=280,120 bank=3000 card=main/free/pick/upgrade/upgrade/demolish at=2,2.5,3,3.5,4,4.5
larder-world mode=squads hand=guard seed=3 look=R/s3guard screen=1280x720 at=1,1.5,...,20   (and GUARD_MOVE=1 into R/s3move)
larder-world mode=squads hand=guard seed=S look=R/guardS screen=640x360 at=30            (S 1-6, and GUARD_MOVE=1)
```

Before: main at c88adacd, `mode=squads hand=squads look=R/before
screen=1920x1080 ui=125% at=2`. `raw/make_gifs.py R` makes the clips and
crops. `card=` is new: a step a frame before its picture
(`headless::card_step`).
