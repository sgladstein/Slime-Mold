# The build pages: what a build holds, in the menu and in play (2026-11-18)

The Look and interface lane's first job in its brief: the pages that tell
the owner what a build holds, drawn from Engine and tools' data
(`slime_mould::build`, #449). The owner's word (11-17): features left
switched off are a problem because he can't tell what is on or off or
what needs testing. Page: `page/index.html`, published at
https://claude.ai/artifact/2SwV4MTJMh7MFLQAyuqHPn (listed in `Reports/pages.md`).

## What it is

- **A "What is new" row on the main menu** (`StartMenu::main_page`,
  `Hit::Build`), between the ways to play and Options, with an accent chip
  of the lines this PC has not seen ("19 new") and a caption: how many
  changes since the last build played here, and how many things to try
  are not ticked.
- **A page named for the build** (`Page::News`, "Build 3127", or the
  commit where the build has no number), two tabs, Tab between them:
  - *What is new*: every line of `news/whats-new.txt`, the unseen first
    with a NEW chip, then by date; one line a row, cut to fit, and a pane
    at the right with the focused line whole, how to try it, its lane and
    its date. Pages by height, never scrolls.
  - *Things to try*: the five lines of `news/things-to-try.txt`, the most
    important first, each with a tick box. A tick is saved to the
    settings file at once (`Memory::set_tried`) and stays on that PC.
- **The build in the main page's corner** (`build 3127 (d08583d,
  2026-11-17)`), and, left of it in the HUD's warning yellow, any
  experiment on: "Experiment on: Shake and hit-stop", or "2 experiments
  on".
- **A mark in play** (`ui::Model::trying`, `top_bar`): with any switch off
  its default, a small yellow chip under the clock, "EXPERIMENT ON" or
  "2 EXPERIMENTS ON". Pointing at it names them: "Not the default game:
  Shake and hit-stop on, Crystals refill on. Options, the Experiments
  tab". The default game shows nothing there.

Seen lines are marked when a round starts (Engine's `Memory::played`, as
before), so the NEW chips are the lines added since this PC last played,
not since it last opened the page.

## Fitting one screen

Nine rows on the main page at 200% on 1080 lines did not fit at 40 units a
row: the rows now take what is left, from 40 down to 34 units, so the
page never scrolls. The fit tests hold the build page's two tabs, with
every row focused, at the menu's sizes (`the_menu_pages_fit_one_screen`,
`the_full_main_page_fits`, `every_page_follows_the_ui_size`, and the
window's `the_menus_real_lists_fit_one_screen` with the game's own
lists, both tabs and no scrolling). In the pictures the 22 lines take one
page at 100% and three at 200%; the five things to try take one.

## A fix in Engine and tools' `news.rs`

`Memory::set_tried` saved the whole memory. The window's copy keeps the
`news_seen` and `last_build` it loaded, while `played()` writes the
round's mark to the file, so the first tick after a round would have
taken that mark back off and every line would have shown as new again.
It now writes only the `tried` key. `a_pc_remembers_what_it_played` holds
it, and fails on the old code.

## What the pictures show

Planet 1, run seed 1, 1920 x 1080, on llvmpipe (`raw/film.sh`, pictures
by `raw/make_pics.py`). The settings file is a PC whose last round was on
main at ecacd8f (2026-11-17): the three lines the file held then are
seen, the nineteen since are new, and the camera shake line is ticked
(`raw/film_settings.py`). The build is a work-in-progress commit of the
branch (59821c0), and a cloud clone has no history to count, so it reads
"build 59821c0"; on the owner's PC it reads "build" and a number.

| Picture | What it holds |
|---|---|
| Main menu, 100% | the row with "19 new", its caption, the build in the corner |
| What is new, 100% | 19 NEW rows, 3 seen, one page, the pane on the first |
| Things to try, 100% | five lines, one ticked |
| Main menu, 200%, shake on | nine rows at 34 to 40 units, "Experiment on: Shake and hit-stop" |
| What is new, 200% | 9 rows a page, 1 of 3 |
| Things to try, 200% | five lines on one page |
| Play, shake on, 20 s | the mark under the clock |
| Play, two on, the cursor on the mark | the mark and its tip |

## Tried and not kept

- **The mark inside the clock's pill.** Reasoned from the bar's sizes, not
  built: at 200% with the clock paused the pill reaches the goal's pill.
- **An "Experiments:" line above the build in the menu's corner.** Built
  and replaced by the one line beside the build: on a narrow screen the
  line would meet the caption box of the last rows (reasoned, not seen).

## Tests

- `the_build_page_marks_what_is_new_and_ticks_what_was_tried`
  (`src/world/start.rs`): no row without a build page; the row's count,
  the build in the corner, one experiment named and two counted; the row
  opens the page with NEW on the unseen lines only; a click on a seen line
  tells it and starts nothing; Tab to the things to try, a click ticks
  (`StartEvent::Tried`) and Enter unticks; Tab back, and Esc returns to
  the row; with nothing new, no count and the page says so.
- `a_round_off_the_default_game_wears_a_mark` (`src/world/ui.rs`): no mark
  in the default game; one and two experiments at 100% and 200% with a
  paused clock and the long goal: the mark under the top bar at the
  right, touching no other word; on it, the tip names what is on and a
  click does not reach the world; two are counted.
- `the_menu_says_what_the_build_holds` (`src/bin/larder-world/tests.rs`):
  the game's own lists; a PC that never played sees every line as new,
  one that played an older list sees the three lines since first and its
  last build named (by commit when it has no number); a tick reaches the
  settings file, an untick takes it off, and neither starts a round.

## Frame cost

Not timed: the mark reads the round's keys against the two rows of the
switch list once a frame and draws one chip when an experiment is on. The
menu's pages draw only in the menu.

## Not done here

- The Experiments tab's own look is unchanged (Engine's first tab, one row
  a switch).
- The pause menu does not list the experiments; the mark in the top bar
  does.
- Seen and left: at 200% on 1080 lines the goal's pill covers the top
  bar's fourth number (so on main before this change).
