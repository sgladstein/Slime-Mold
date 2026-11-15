# Every screen follows the UI size (2026-11-15)

Owner, 11-15 07:56Z: "Changing the UI /text percentage increase does do
what it should the menus are unaffected. The gun range is unaffected (I
cannot read any of the text). I am guessing the shop doesn't change."
Same morning, 07:51Z (gun cards above the bar) and 07:54Z (hover card over
the crystal): see the follow-up in
[ui-review-2026-11-15](../ui-review-2026-11-15/README.md#follow-up-11-15-0800z-one-row-and-the-hover-card).
Page: [page/index.html](page/index.html), https://claude.ai/artifact/4m4axMzedWsxoX3yAFxhH4.

## What was wrong

`StartMenu::draw` held each page to what fitted the window's height:
`s = ui.px(screen).min(sh / held)`, with `held` 330 + 62 a row for the
options and key pages. At 1080 lines with 15 options that is 1080 / 1260 =
0.85, at any UI size. The main page was held too (about 1.17 at most), and
the shop's layout started at 1.3x and shrank by 0.92 a step until it fitted,
which for the gun range's armory (every gun and card) was about 0.6. So
Ctrl + changed nothing on the options and key pages, and little anywhere.
The in-game key card (F1) had one column of 25-unit rows and ran off the
screen at big sizes (some 40 rows in squad mode).

## What changed

- Every start-menu page draws at `UiSize::px`. A page taller than the
  window scrolls under its title and over its hint (and the shop's two
  buttons, now fixed): the mouse wheel (`StartMenu::wheel`), the arrow keys
  (the focused row is scrolled into view the next frame), a thin bar at the
  right edge. What scrolls out cannot be clicked; partly in, only there.
- The canvas gained a clip: `Canvas::clip`, a band of rows each shape keeps
  to (word 16 of a shape, `PRIM_WORDS` 16 -> 17; the shader skips pixels
  outside it; binning only touches tiles inside it).
- The shop still starts at 1.3x where it fits, but never goes under the UI
  size for height (only the window's width shrinks it further); past that it
  scrolls. Where the armory beside the upgrades would hold one gun card a
  row, it goes under them at the full width (200% on 1920x1080).
- The key page uses two columns only when each can be 480 units wide; else
  one, and it scrolls.
- The in-game key card uses up to three 600-unit columns and shrinks only
  past that.
- `scroll=N` (no-window pictures): with `menu=PAGE`, N wheel notches a frame.

## Checked

Tests: `every_page_follows_the_ui_size` (main, campaign, options, keys and
the gun range's armory at 1280x800, 1920x1080, 2560x1440: 200% draws at
least 0.9x the size's growth bigger, the shop never under the UI size, and
every row is reached and in view by the keyboard alone),
`every_ui_size_reaches_every_option` (22 options, every size),
`the_key_card_fits_at_every_size` (in play, 43 rows), and the shop, main
page and click tests now scroll to what they click. `cargo test --release
--locked` and clippy pass.

Stills (`larder-world look=DIR screen=1920x1080|2560x1440 ui=100%|200% at=1
menu=main|options|keys`, `run=1 menu=shop-mid`, `range=1 menu=shop`) and
clips (the same with `scroll=1 at=1,1.25,..,4.75`); before = main at
17f2e5b8. Pictures by `raw/make_pics.py`.

Frame cost: the clip is one more word a shape and a compare a pixel per
shape; the menu draws once a frame as before. Not timed on the 1070.

## Not changed

- The UI size is still capped where the bottom bar would take a third of
  the window or outgrow its width (`UiSize::px`): 200% on 1920x1080 is 1.9x.
- The map editor's panel (`UiSize::fixed`) was not touched.
