# Shop B and menu A, the owner's picks (2026-11-15)

The owner, 11-15 17:31Z: "Having to scroll is horrible. That's not
professional." 17:41Z: "The bullet tiles could easily be half the size
without even losing any info." Three researchers drew blind pairs for the
main menu and the shop ([research](../menu-layout-research-2026-11-15/README.md));
at 19:37Z the owner picked: "Shop B, menu A,". Options, the keys and the
campaign had gone to one screen first
([one-screen menus](../one-screen-menus-2026-11-15/README.md)); with this
change no page of the start menu, the run shop or the gun range's armory
scrolls.

Page: https://claude.ai/artifact/Sv62LV9qkAgWrr1GUZSr9e, source in `page/`.

## Menu A (`src/world/start.rs`)

The main page is one column of names: the ways to play (Run, Sandbox,
Campaign, Random planet, Gun range), then Options and Edit maps, then Quit,
on 40-unit rows with a gap before Options and Quit. One caption box beside
the row under the cursor, else the keyboard's, carries that row's line.
The title is 36 units on this page only. There is no Keys row: the key list
is Options' last tab, and F1 or ? opens it from anywhere. A notice (the map
editor did not start) takes the hint's line.

At 200% on 1080 lines (1000 x 562 units) the page had 66-unit rows with
their lines under them, from 160 units down: about 780 units of 562, so it
scrolled. Now 8 rows end at 484.

## Shop B (`src/world/start/shop.rs`, `StartMenu::shop_page`)

One frame for the run's shop and the gun range's armory:

- **Top bar** (46 units): the planet's line (the armory: "The gun range")
  at the left, the points ("Free") at the right. The old title, subtitle
  and three-line briefing took about 220 of the 562 units.
- **The gun and the bag, pinned.** The carried gun's line (name, budget
  used of its budget, slots filled, "fired left to right"), its slots
  numbered in firing order (an empty one says so), and under it the bag.
  They never move: buying, emptying or a fuller bag leaves every slot,
  tab and button where it was (a test checks it; the owner's 02:58Z "everything shifts in the shop").
  A bag too long for its row pages with its own arrows.
- **Tabs over a counter.** The run: Cards, Guns, Squad, Buildings, Tools,
  each with how many it holds that can still be bought and a dot while
  something there is affordable. The armory: Guns, Bullets and the two
  shelves of modifiers (Cards for the next bullet, Cards for every bullet).
  Tab moves along them, 1-9 picks one. A counter too long for the screen
  pages (`Hit::Flip`); it never scrolls.
- **Half-size tiles.** Bullet and modifier cards are 50 units tall (64
  before) and 150 wide in the gun's slots (184 before), widened up to 210
  to fill a counter row; in the bullet grid a cell is about 100 x 43. The
  icon, name, budget dots and price stay on the tile, and the rest moves to
  the pane. Upgrades, buildings and tools keep the command card's tiles.
- **Guns as a table** against the carried gun: range, rate, aim, budget,
  slots and shots, each green when better than yours and red when worse,
  the carried gun first and marked "(yours)". The old page drew each gun
  as a large card of stat bars, two to a row.
- **Bullets as a grid** in the armory: shape down (Standard, Slug, Needle,
  Buckshot), payload across (Bullet, Blast, Bomb, Gas, Germ), headers in
  the payload's colour. Twenty bullets in one 4 x 5 grid.
- **One pane at the right.** It tells the picked tile in full: its icon and
  name, where it is (in slot 2, in the bag, on offer), its budget, level
  and price, what it does, and for a gun its stats beside yours. With
  nothing picked it shows the planet's briefing (the armory: how the range
  works). Its button does the tile's one thing: "Buy · 150 points", "Put it
  in the gun", "Take it out of the gun", "Carry this gun"; it is outlined
  red when the points are short.
- **Bottom bar** (62 units): Land on the next planet and Give up the run
  (the armory: Back to the range and Leave the range, then the range's
  tools New rolls, Deeper rolls, Shallower rolls, Empty the gun, Empty the
  bag), and the hint at the right, whole or in a short form, never cut. A
  buy the run refuses (too few points, over the gun's budget) now says why
  there in amber; before, the shop said nothing.

Picking and buying: a click picks a tile and the pane tells it; Enter, or
the pane's button, buys. The first arrow key or Enter only picks, so the
keyboard cannot buy by accident. Dragging a card onto a slot of the gun
buys it and puts it there, as before; Q and E move a picked card in the
gun. Buildings and Tools were folders that opened in place; they are tabs
now.

## Measured

The game's own pictures, from `larder-world look=DIR run=1 menu=shop-mid`
(a run three planets in: 670 points, three cards in the long gun, one in
the bag, five on offer, two guns for sale) and `range=1 menu=shop`, at the
size named. `menu=shop-mid@TAB@NAME` opens a tab and picks a row.

| Page | Before | After |
|---|---|---|
| Run shop, 200% on 1080 lines (1000 x 562 units) | 10 tiles on screen (8 squad upgrades, the Buildings and Tools folders); the 4 guns, the gun's 3 cards, the bag and the 5 cards on offer below the fold | The gun's 3 slots and the bag always on screen; every tab whole on one page: Cards 5 of 5, Guns 4 of 4, Squad 8 of 8, Buildings 7 of 7, Tools whole |
| Run shop, the window as it opens (1600 x 972 at 125%) | Upgrades, 4 guns and the gun's 3 cards; the bag and the 5 cards on offer below the fold | Everything as at 200%, smaller |
| Armory, 200% on 1080 lines | The 5 range tools and one describing box; the 8 guns, 20 bullets, 14 modifiers, the gun and the bag below the fold | Guns 8 of 8 on one page; Bullets 20 of 20 in one grid; modifiers 6 and 8, each shelf whole; tools on the bottom bar |
| Armory, 125% | The tools and 5 of 8 guns; bullets and modifiers below the fold | Everything on one screen |
| Armory, 21:9 at 200% (1280 x 540 units) | (not measured) | Guns 6 of 8 on page 1 of 2; every other tab whole |
| Main menu, 200% on 1080 lines | 4 of 9 rows whole (about 780 of 562 units): Gun range cut, Options, Keys, Edit maps and Quit below the fold | 8 of 8 rows, ending at 484 of 562 units |

Tests:

- `the_menu_pages_fit_one_screen` (lib) draws the shop (a small run, a full
  one with six slots and 30 rows) and the armory (with 3 and 14 cards in
  the bag) at eight sizes, from 1000 x 562 units to 1920 x 1080, on every
  tab with each of its rows picked, and on the first tab with every row
  picked, the bench and the buttons included: no scroll, every glyph on
  screen, none touching another, none under 12.5 units.
- `the_shop_holds_still_when_the_bag_fills`: the gun's slots, the bag,
  the tabs and the bottom bar's buttons stay put when cards go into the bag,
  at three screens and every UI size.
- `the_gun_ranges_armory_goes_back_to_the_range`,
  `the_shops_rows_pick_and_buy`, `a_drop_lands_in_its_own_slot` (a
  six-slot gun at 200% on 1280 x 720) and
  `cards_drag_to_slots_and_q_and_e_move_them` cover the armory's tabs and
  tools, picking then buying, and drags.
- `the_menus_real_lists_fit_one_screen` (bin) adds the game's own run shop
  (Run 4, three planets won, 900 more points, three cards bought) and the
  real gun range: every shop tab one page and the armory's Guns at most
  two, at 200% on 1080, 540 and 720 lines, 175% on 1280 x 800, 125% in the
  opening window and 100% on 1080, in both control modes.

All 38 of the window's tests, the 19 start menu tests and clippy pass. The
shop draws only while it is open, so it costs no frame in play.

## Left for later

- The bag shows the outlines of its free places (as many as fit its row)
  so the bench keeps its height; with an empty bag they read as blank
  slots.
- At 21:9 and 200% the armory's guns take two pages; a shorter table row
  would fit 8, at 22 units a row already its least.
- Card 2 (the command card) and the 19:51Z rulings (Burn a line removed,
  Guard as an order, a click selects a building) are built in their own
  thread.
