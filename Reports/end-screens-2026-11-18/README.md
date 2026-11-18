# The end of a planet says what happened to you (2026-11-18)

The Look and interface lane's brief, job 4: "Win and lose screens that
show the planet's numbers" (`handoff/look-and-interface.md`; `NOW.md`,
the lane's row). It also serves Milestone 1's exit test 4, "After playing
he can say in a sentence what each of the three slimes did to him". Page:
`page/index.html`, published at
https://claude.ai/artifact/YSjVHU9Es2ztXjeE52VLt1 (listed in
`Reports/pages.md`).

## Before

The end screen was a card 420 wide at 100% UI size, with a title, grey
lines and the buttons:

- **A run's planet:** "Planet 1 of 8, Dreskor: ***" or "Mission lost".
  Then the cause ("Your last soldier fell at 1:00 (best 0% of the
  goal)"), the run's line and a summary ("23 soldiers left, 5700 food
  harvested, 763 walkers killed with 621 sh..."), cut off at 100%. The
  card did not name the slime or say what it did.
- **A sandbox round:** "Time: you took more of the crystals than the
  mould", then the share and the slime's size, the summary and the score.

## Now

`ui::Ending` (`Model::end`) is what the card shows. `draw.rs` fills it
(`end_screen`, `mission_screen`, `end_tiles`, and `App::model` for a run's
planet), and `Hud::end_screen` draws it:

- **Where it was**, small, in the outcome's colour (green won, red lost,
  blue for a sandbox draw), which also runs along the card's top edge. A
  run's planet reads "Planet 3 of 8 · Drevix · the Trunk". A mission shows
  its name, and the sandbox shows its map.
- **The title is how it ended.** A run's planet won reads "Planet N
  freed" ("The home world is free" on the last planet). Otherwise the
  title is "Mission won", or by cause "The squad is lost", "The slime
  took the larder", "Out of time", "Out of food", or "The slime is gone,
  the goal missed". The sandbox titles are as before, with the clock
  moved to a tile. The title wraps rather than being cut off.
- **Stars**: three discs under the title, the earned ones filled, shown
  only for a win.
- **What the slime did**: on a run's planet, the tier's sentence
  (`run::does_to_you`). The landing alert gives the same sentence and
  until now it was not shown again.
- **Six tiles**: time played, walkers killed, shots with kills a shot,
  soldiers standing with how many were lost, food harvested, and the
  slime left against its peak (as the top bar counts walkers, "42.3k" from
  10,000). Numbers under 100,000 have their thousands set off.
- **Lines** under the tiles: how it ended (the mission's cause line, or
  the sandbox's share and score) and the run's points or its end. Then the
  same buttons.
- **Size**: 560 wide at 100% (less on a narrow screen). A card taller than
  the screen less 32 px is drawn smaller to fit, so at 200% on 1080 lines
  it fills most of the height.

## Pictures

They come from `film_ends`, an ignored test in
`src/bin/larder-world/film.rs`, using the window's own game, HUD and
drawing on the software renderer. Each case is a minute of play by the
scripted greedy hand, then the end the game would give: the slime wiped
(planet 1 of run seed 1), every soldier fallen (planet 3), or the clock
with 62% of the crystals taken (the Fork's sandbox). So the numbers are a
minute's, not a whole planet's.

| Case | Before | After |
|---|---|---|
| Planet 1 won | "Planet 1 of 8, Dreskor: ***"; the summary cut off | "Planet 1 freed", 3 stars, the Drifter's sentence, 6 tiles: 1:00, 763 killed, 621 shots (1.2 kills each), 23 standing, 5,700 food, 12.0k of 13.1k |
| Planet 3 lost | "Mission lost"; the summary cut off | "The squad is lost", the Trunk's sentence, 6 tiles: 1:00, 2,291 killed, 140 shots (16.4 each), 0 standing (9 lost), 0 food, 42.3k of 43.4k |
| The Fork's sandbox | a 50-character title and 3 lines | "Time: you took more of the crystals", 6 tiles, the share and the score |
| Planet 3 lost, 200% on 1920 x 1080 | | shrunk to fit |

`FILM=DIR cargo test --release --bin larder-world film_ends -- --ignored
--nocapture`, with `FILM_SIZE`, `FILM_UI` and `FILM_CASES` (`won`,
`lost`, `sandbox`). The page's pictures are cropped round the card.

## Test

`the_end_screen_shows_the_planets_numbers_and_fits` (`src/world/ui.rs`)
draws a run's planet lost with everything the card can hold and three
buttons. It checks the card at every UI size (75% to 300%) on 1280 x 800,
1920 x 1080 and 2560 x 1440: the Quit button stays on the screen, and the
numbers, the slime's name and its sentence are drawn.

## Cost

Only while the round is over: a card of some 30 text runs. Not timed.

## Not done here

- The guide's next step for screens: a landing, a win and a loss that
  happen in the picture, not on a card.
- The slime's peak is the game's (`Game::peak`). The run's other numbers,
  such as cards bought, are the shop's.
