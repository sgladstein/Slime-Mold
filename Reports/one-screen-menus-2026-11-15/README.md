# Options, keys and campaign on one screen (2026-11-15)

The owner, 11-15 17:31Z: "Having to scroll is horrible. That's not
professional." Three researchers then agreed on a set of changes
([research](../menu-layout-research-2026-11-15/README.md)). This change
builds the agreed parts that sit on the start menu's inner pages, while the
owner picks the main menu, the shop and the command card.

Page: https://claude.ai/artifact/Qqc92vTR4f9VigMbEtdjDv, source in `page/`.

## What changed (`src/world/start.rs`)

- **One-line header on the inner pages.** Options, its key list and the
  campaign have no "Larder World" title and subtitle any more. Each has a
  56-unit band with the page's name, the Options tabs and Back (Esc) at the
  right. The title and its line had taken 160 of the 562 units that a
  1080-line screen has at 200%.
- **Options in tabs.** The tabs are:
  - Game: UI size, Control, Rounds, Bullet speed, Dev mode.
  - Planet: Sandbox map, Planet, Your own mould, Plants.
  - The mould: Panic, Wither, Shared food, Condense, Breakaway, Thicken,
    Surge, Flinch.
  - Keys.

  `Opt::in_tab` sets an option's tab, and `menu.rs` assigns them. Tab
  moves along the tabs.
- **Rows and the note pane.**
  - Each setting is one 46-unit line. They were 62-unit rows of two lines,
    with the note cut to fit.
  - A dot after the name marks a setting moved off its default.
  - The pane on the right shows the setting under the mouse, else the one
    the keyboard is on. It gives the whole note, the value against the
    default, a "Back to X" button, and All to defaults.
  - The pane grows to fit its note.
- **The key list as Options' last tab.**
  - F1 or ? still opens it from anywhere, and Back returns to where it was
    opened. The tabs show only when Options or the main menu opened it.
  - It has two columns down to 788 units across.
  - The key column is 0.3 of a column, up to 180 units. At 0.36 and 210 it
    took three pages at 200% on 1080 lines; it takes two now.
  - Long lines wrap rather than being cut.
- **The campaign as one-line rows.** The map and mould sit at the right of
  each row. A pane tells the pointed-at mission in full.
- **Pages, not scrolling.** A list longer than the screen is split into
  pages. The pager sits at the right of the foot.
  - Left/Right turns the key list's pages; PageUp/PageDown turns any
    list's.
  - The keyboard's row picks its page, so Up/Down walks the whole list.
  - `reveal` (tests and the window's smoke hand) turns to the page, or the
    tab, that holds a row.
- `menu=options-planet` and `menu=options-mould` (with `look=`) draw those
  tabs.

The main page and the shop still scroll at large UI sizes until the owner
picks their layouts.

## Measured

These are the game's own pictures, from `larder-world look=DIR menu=PAGE
screen=WxH ui=N%` in the RTS control mode, whose key list is 48 lines. The
squad mode's is 49.

| Page | Before, 200% on 1080 lines | After, 200% on 1080 lines | After, 1600x972 at 125% (the opening window) |
|---|---|---|---|
| Options | 5 of 17 settings whole; two notes cut; Back and All to defaults below the fold | three tabs of 5, 4 and 8, each whole | each tab whole (before: 10 of 17) |
| Keys | 13 of 48 lines, one column | two columns, 2 pages, nothing cut | 1 page, 48 of 48, nothing cut; 2 pages in the squad mode (before: 48 of 48 in two columns, three lines cut short, a little scrolling) |
| Campaign | 5 of 8 missions whole; Back below the fold | 8 of 8 | 8 of 8 |

The key list's pages with the game's own list, both control modes:

- 200% on 1080, 540 or 800 lines: 2 pages.
- 200% on 720 lines: 3 pages.
- 125% in the opening window: 2 pages in squad mode, 1 in RTS mode.
- 100% on 1080 lines: 1 page.

The menu draws only while it is open, so a round's frame cost is
unchanged.

## Tests

- `world::start::tests::the_inner_pages_fit_one_screen`: a menu with the
  game's list sizes, at eight sizes:
  - 1920x1080 at 200%, 1280x540 at 200% (21:9), 1280x720 at 200% and
    1920x1080 at 100%;
  - 1280x800 at 175%, 1600x972 at 125%, 640x400 at 100% and 2560x1440 at
    300%.

  Every Options tab with each row focused, every page of the key list and
  each focused mission must:
  - not scroll;
  - keep every button and every glyph on the screen;
  - keep any two text runs' glyph boxes apart;
  - draw no text under 12.5 units;
  - fit each Options tab and the campaign on one page.
- `world::start::tests::a_long_list_pages_instead_of_scrolling`: 30
  missions and 30 options on one tab at 200%. Every row is drawn when
  focused, nothing scrolls, and the pager's arrows go round and back.
- `tests::the_menus_real_lists_fit_one_screen` (the window): the game's
  own options, missions and key card in both control modes. Every tab and
  the campaign take one page, and the keys three at most, at the six sizes
  that matter. The tabs are Game, Planet and The mould, and Tab goes round
  through the keys.
- These were updated:
  - `every_ui_size_reaches_every_option`: a long tab pages and never
    scrolls.
  - `every_page_follows_the_ui_size`: the header's Back is fixed above the
    body.
  - `turning_the_map_on_the_menu_switches_the_dish_behind`: the map is the
    Planet tab's first row.

`cargo test --release --locked --lib world::` (223 passed) and `--bin
larder-world` (38 passed) pass, and so does clippy with `-D warnings`. The
whole suite runs longer than this machine's 10-minute limit, so CI runs it
in the queue.

## Not in this change

- The main menu (caption box or subtitled cards), the shop and armory (one
  board or tabs), and the command card (Card 1 or 2, Guard, building
  actions) wait for the owner's picks on the
  [options page](https://claude.ai/artifact/Su1g5MdjbDU2P6ic8YhoLT).
- The mould's rules stay in Options on their own tab. A setup page for
  Sandbox and Random planet is still the owner's question.
