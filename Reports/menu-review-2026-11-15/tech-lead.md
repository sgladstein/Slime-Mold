# Reviewer 2: technical lead review of the UI code

Branch `claude/project-thread-xbk94d` at d5af42ab. ui.rs, start.rs and canvas.rs are the same as at 77101557.

## Read this first: the review renders are not HEAD

`target/release/larder-world` was built at 08:50. That is before 8b4a4c6a and 77101557 merged #330 ("Healing is a squad order"). Every picture in `review/renders/`, and anything else rendered with that binary, has no Heal button. It also misses the bug that button brings with it (finding 1).

I built HEAD into a separate target folder in the scratchpad, so the repo's `target/` was not touched:
`/tmp/claude-0/-home-user/44a0ac59-f077-5adf-a8b1-37721f37a933/scratchpad/tgt/release/larder-world`. The four frames below come from that build:

- `/tmp/claude-0/-home-user/44a0ac59-f077-5adf-a8b1-37721f37a933/scratchpad/review/extra/tl-a-sq150/` (header-zoom.png, bar.png): squads, 1920x1080, 150%
- `/tmp/claude-0/-home-user/44a0ac59-f077-5adf-a8b1-37721f37a933/scratchpad/review/extra/tl-b-sq200/bar.png`: squads, 1920x1080, 200%
- `/tmp/claude-0/-home-user/44a0ac59-f077-5adf-a8b1-37721f37a933/scratchpad/review/extra/tl-c-rts1280/` (topbar-zoom.png): RTS, 1280x800, ui=200% (1.3 after the cap)
- `/tmp/claude-0/-home-user/44a0ac59-f077-5adf-a8b1-37721f37a933/scratchpad/review/extra/tl-d-sqbuild/bar.png`: squads, 1920x1080, 100%

## What I checked and what I guessed

**Checked:**
- Read the layout, hit-testing and input code: `src/world/ui.rs`, `src/world/start.rs`, `src/world/canvas.rs` and `canvas.wgsl`, `src/world/settings.rs`, `src/bin/larder-world/{events,squadmode,draw,menu,uisize,main,headless}.rs`.
- Looked at all 23 review renders and my 4 new frames.
- Ran `world::ui::tests` from the 09:10 test binary: 20 of 20 pass, including the squad tile test.
- Measured the cut-off text with PIL and the game's own Inter Medium and SemiBold fonts.
- Took the per-frame CPU cost from the `look=` logs.

**Guessed, and marked as such below:**
- Whether the owner maximizes the window.
- The start menu's CPU and GPU cost.
- How often the shop's scroll and drag case comes up in play.

## The big picture

**1. The way the UI is built is fine for this game, and cheap.**
- The HUD and menus are immediate-mode: rectangles placed by hand in UI units, with hits recorded as they are drawn.
- The HUD's model and layout cost 0.08 to 0.20 ms of CPU a frame (9 `look=` logs today).
- The new clip is small and correct:
  - Word 16 is a band of rows. `bounds()` trims binning to the band, and a shape wholly outside it is dropped in `push`.
  - Hits are cut to the visible band. The keyboard focus scrolls into view.
  - Two harmless details: the band edges are truncated to whole pixels, so they can be off by one row; and the clip covers rows only, not columns.

**2. The bottom bar is the weak part. It is one row of fixed height, and its width in UI units shrinks as the UI size grows.**
- At 1920 px wide, 200% leaves 1,010 units for a row designed around about 1,900.
- Each panel has its own fallback with its own threshold, and each decides alone, one after another:
  - The command card loses its labels below `LABEL_ROOM` (340 units).
  - The gun strip shrinks, down to 60%, when the selection would keep less than 300 units.
  - The selection panel vanishes below 120 units.
  - The squad header steps down four times, then drops buttons.
  - The squad header's second line ("commanding Squad 1, …") needs 70 units.
- **The result at 150% to 200%:**
  - The command card shows icons only.
  - The gun strip's text ends up about as big as at 100%: at 1080 lines it is drawn at 1.14 against the HUD's 1.9, and at 1280x800 at 0.78 against 1.3. That is the gun cards, the thing you read while shooting.
  - The squad header loses its word, its key letters, its second line and a button.
  - Meanwhile the selection panel keeps about half its width empty when you have one squad (renders `hud-1920x1080-200`, `tl-b-sq200`).
- **So the fallbacks trade text for empty space, the opposite of both of the owner's asks** ("the text seems too small", "not always making the best use of space").

**3. The tests check that hits stay on the screen, not what the owner sees.**
- Nothing checks that text stays off buttons, that text is not cut, that text is not smaller than the UI size, or that a key letter shown is the key that does it.
- No test covers the default mode (squads) with a gun at a UI size over 100%.
- Every finding below passes the current suite.

**4. The sizes we review are not the window the game opens.**
- `events.rs:11-12` opens the window at 90% of the monitor, capped at 1600x1000.
- On a 1080p monitor that is 1600x972, so the UI draws at 0.9 at "100%". On a 1440p monitor it is 1600x1000, so 0.95 (`UiSize::px` follows the window height).
- If the owner does not maximize the window (a guess), the text they see is 5 to 10% smaller than every review picture at "100%". That alone feeds "the text seems too small".

### What I would do next, in this order

1. **Today, small:**
   - Fix the squad header's fit (finding 1).
   - Hide header key letters the current card page takes (finding 2).
   - Stop the hover card while a button is held (finding 4).
2. **Gun strip:** never scale it below the UI size. Drop the name column (its contents are already in the tooltip) and then Swap before shrinking anything, and size the panel to its contents (finding 3).
3. **One row allocator for the bottom bar.**
   - Each panel reports a minimum width and a preferred width from its content at the current scale:
     - the selection: header buttons, or n squad tiles;
     - the command card: square or labelled;
     - the gun strip: cards only, then with its name.
   - The allocator gives out the minimums, then hands the spare width to upgrades in one stated priority order, for example card labels, then gun name, then selection room.
   - This replaces the five separate thresholds.
4. **A layout-lint test over the real cases.**
   - Screens: 1600x972, 1600x1000, 1280x800, 1920x1080, 2560x1440. Every UI step. Squads and RTS. 1, 4 and 9 squads. Guns with 3, 5 and 8 slots. The main, build and tools pages.
   - It asserts:
     - no text run overlaps a button or another run;
     - every run sits inside its panel;
     - no HUD text is drawn under about 11 px times the UI size;
     - every key letter shown is the key that does it.
   - Canvas could record text runs under `cfg(test)` for this.
5. **Window and size:**
   - Open the window maximized, or base the UI unit on the monitor rather than the window.
   - Make the UI size steps that change nothing say so (finding 6).
   - Make a row click stop resetting the UI size (finding 7).
6. **Cut text:** wrap instead of cutting on the end screen, both key cards and the option notes (finding 8).
7. **Start menu drag and scroll fixes** (findings 9 and 12).

## Findings, ranked by how much fixing them improves play

### 1. The squad header: Heal sits on top of "1 squad" at 150% and vanishes at 200%. This came in with today's merge.

**Where:** squad mode HUD. `src/world/ui.rs:1324-1349`.

**What the player sees:**
- At 1920x1080 and 150%, the header reads "1", with a greyed "Heal L" button covering the word "squad" (`extra/tl-a-sq150/header-zoom.png`).
- At 200%, Heal is not drawn at all (`extra/tl-b-sq200/bar.png`).
- At 100%, it fits, and the second line is cut to "commanding …" (`extra/tl-d-sqbuild/bar.png`).

**Cause:**
- e8b97d2c sized the header's fit with `row_w` over four buttons: Let go, Combine, Split, Guards.
- 4a9a28d6 added a fifth, Heal, to the drawing loop.
- The merge 8b4a4c6a joined the two without a conflict. Heal is outside the fit, and it is last in the loop.
- So the header word is kept on the strength of four buttons, and Heal is then drawn over it. Or Heal is the first button skipped, not Guards as the comment and the commit say.
- The squad test passes because its rig has no gun strip and only runs at 100%.

**Fix:**
- Build the list of (label, key, off, hit, about) once, and use that same list for `row_w` and for drawing.
- State the drop order explicitly.
- Never place a button left of `inner.x + hw + 12s`.
- Extend `squad_tiles_command_a_squad_bring_one_in_and_split_and_combine`:
  - give it a loadout;
  - run it at every UI step on 1920x1080;
  - assert that no header button intersects the header text's rectangle;
  - assert that each action is either drawn or named in the drop order.

### 2. Gun strip at 60%, and command card labels lost early

**Where:** HUD bottom row. `ui.rs:1040-1067` (the decisions), `ui.rs:1623-1624` (contents centred in a full-height panel).

**What the player sees:**
- At 200%, the gun cards, "Plain" and "Plain shots" are about the size they are at 100%.
- They float in the middle of a tall, mostly empty panel (`renders/hud-1920x1080-200`, `extra/tl-c-rts1280`).
- The command card loses its labels from 125% at 1080 lines.

**Cause:**
- The shrink is triggered by a fixed 300-unit minimum for the selection. That ignores what the selection holds: one squad tile is 136 units.
- The labelled-card decision (`ui.rs:1042-1044`) counts the gun strip at full width, even though the strip can shrink afterwards.
- Worked out from the code's constants at 125% on 1920x1080:
  - The labelled card needs 425 px of spare row; the full gun strip leaves 258.
  - The strip without its 124-unit name column and its Swap button leaves 483, so the labels would stay.

**Fix:**
- Give the selection a minimum width taken from its content.
- Shed the gun strip's parts, not its scale. Put the name and state into the tooltip that already shows them.
- Let the panel height follow the content.
- Then make the card's labelled or square choice after the gun strip has been sized, not before.

### 3. The default window draws the UI at 90 to 95% (guess about how the owner plays)

**Where:** every screen. `src/bin/larder-world/events.rs:11-12` and `src/world/settings.rs:49-56`.

**What the player sees:** on a 1080p monitor the window is 1600x972 and "100%" is 0.9. On a 1440p monitor it is 1600x1000 and 0.95. None of today's tests or renders use these sizes.

**Fix:**
- Open the window maximized (`with_maximized(true)`), or take `fit` from the monitor's height.
- Add 1600x972 and 1600x1000 to the screen lists in the tests and the renders.

### 4. The hover card still comes up in the middle of a volley, and sits on the cursor near the edges

**Where:** world hover. `ui.rs:615-634` (`HoverCard::alpha`), `744-746` (the click that puts it away), `1909-1910` (where it is placed).

**What the player sees:**
- A click only puts away the card that is already up.
- While the fire button is held, moving the aim onto a crystal changes the hover key. That resets `shut`, so after 0.35 s of rest the card appears mid-volley. This is the owner's case: slime sitting on the home crystal.
- In squad mode the camera follows the squad, so a screen cursor that is "resting" sweeps across the world and keeps finding new things to hover.
- Near the right edge or the bottom bar, `min()` slides the card left or up, under the cursor, instead of flipping it to the other side.

**Fix:**
- `alpha` should return 0 while any mouse button is held. Track that in `button`. Also return 0 while WASD steers.
- Flip the card's side instead of clamping it, or dock it in a fixed corner.
- Add a unit test: `alpha` and `moved` already take `now`, so REST, SHOW, FADE, shut and the key reset can be tested without a window. There is no test for them today.

### 5. The squad header shows the wrong keys while the build page is open

**Where:** squad mode with the build page (Q) open. `squadmode.rs:76-82` against `draw.rs:1005-1017`.

**What the player sees:**
- The squad's own keys are I (Guards), Y (Split), J (Combine), L (Heal) and T (formation). They go to the command card first.
- The build page uses the same letters: T Turret, L Lure, I Outpost, Y Siren, J Lures on/off.
- The header still says "Guards: some I", "Split Y", "Combine J", "Heal L". Pressing I starts placing an outpost.
- Checked in the code. Not rendered: `hand=buildq` does not open the page in squad mode.

**Fix:** in `squad_tiles`, drop a key letter that a command on the current card page uses (`self.commands` is there to check against). Or give the squad's actions letters that no card page uses.

### 6. Text cut short at every size

These all cut at every size, because the room grows with the text.

**a. End screen** (`ui.rs:2037-2039`):
- The modal is 420 units wide, which leaves 372 for text.
- `summary(g)` measures 540 px and the time-up line 659 px at 1x. Both are cut: "4 soldiers left, 1234 food harvested, 12,…".
- Fix: wrap the lines and let the modal grow.

**b. In-game key card, F1** (`ui.rs:2021-2022`). The description column is 390 units.

  | Row | Needs |
  |---|---|
  | WASD | 638 |
  | Esc | 621 |
  | Ctrl + 1-9 | 623 |
  | E (Tools page) | 495 |
  | The key "Right click a half-built building" | 219, in a 174-unit key column |

  - Two 600-unit columns use 1,200 of the 1,880 px a 1920-wide window offers, so there is room to wrap or widen.

**c. The start menu's Keys page:**
- The description has 566 units in one column, 482 in two.
- The WASD row (638) is cut.

**d. Option notes** (`start.rs:863`):
- The Control note is cut at 100% and at 200% (renders), and the player has no way to read the rest.
- Fix: wrap to a second line, or show the focused row's full note under the panel.

**e. The squad header's second line** (`ui.rs:1367`):
- Its 70-unit floor lets "commandin…" (2560x1440 at 200%) and "commanding …" (100%) through.
- Fix: show it only when it fits whole.

### 7. One stray click on the "UI and text size" row puts the UI back to 100%

**Where:** start menu Options. `start.rs:558-563`, applied at once by `menu.rs:463`.

**What the player sees:**
- A click anywhere on an option row resets it to its default.
- On the UI size row, that snaps a 200% player back to 100%.
- "All to defaults" resets the UI size too.

**Fix:**
- A row click should only move the focus. Reset with Backspace or a dedicated button.
- Keep `ui` out of "All to defaults".

### 8. UI size steps that change nothing

**Where:** `settings.rs:53`.

**What the player sees:**
- The cap is `(sh/540).min(sw/1000)`.
- At 1920x1080, 200%, 250% and 300% all draw at 1.9.
- At 1600x972, everything from 175% up draws at 1.6.
- The Options row and Ctrl + still step to "300%", with no visible change.

**Fix:** show the size actually used ("300%, 190% fits here"), or stop stepping at the cap for the current window.

### 9. Shop and gun range: a dragged card can land in the wrong slot once the page has scrolled

**Where:** run shop and gun range armory. `start.rs:713` and `start.rs:417-423`.

**Cause and what the player sees:**
- After layout, `slot_rects` is filtered down to the visible slots.
- `drop_at` then treats an index in that filtered list as the slot number.
- During the drag, the ring is drawn from the unfiltered list (`start.rs:1046`), so it shows the right slot.
- So with the first row of slots scrolled off, a card dropped on slot 5 goes to slot 1, while the ring showed slot 5.
- This is reachable at 200% on 1920x1080 with a 5-slot gun:
  - the stacked layout fits 4 slots a row;
  - each row is 122 px;
  - the visible band is about 578 px.
- How often it happens in play is a guess.

**A second case:** the bag's drop area (`start.rs:422`, `1043`) is not limited to the visible band. In the stacked layout a gun card dropped over the fixed "Land on the next planet" and "Give up the run" buttons comes out of the gun.

**Fix:**
- Keep `slot_rects` whole and require `body.contains(p)` in `drop_at`.
- Cut the bag to the band.
- Consider scrolling the page when a drag nears its edge.
- Add a drag test with the page scrolled.

### 10. The top bar's objective covers the resources

**Where:** `ui.rs:1166-1172`.

**What the player sees:** the objective pill is centred with no regard for the resources pill or the clock. At 1280x800 and 200% (1.3), the RTS objective hides the end of the resources pill (`extra/tl-c-rts1280/topbar-zoom.png`). Longer mission objectives will cover more.

**Fix:** place the left and right pills first, then fit the objective between them with `text_fit`, or move it under the bar.

### 11. Small layout gaps

- **The placement chip by the cursor is not kept on screen** (`ui.rs:1898`; `tip_small` has no clamp). At large sizes it runs off the right edge when placing near it.
- **Alerts are not width-capped** (`ui.rs:1832`).

### 12. Start menu scrolling polish

- **The scrollbar is in the wrong place and does nothing.**
  - It sits against the screen's right edge (`start.rs:716`), about 900 px from the column at 1080 lines.
  - It has no hit, so it cannot be dragged.
- **Some wheels don't scroll.**
  - `PixelDelta` is rounded per event (`events.rs:184`). Small touchpad deltas round to 0, so they never scroll.
  - A `LineDelta` of 3 counts as 1.
  - Fix: accumulate the deltas.
- **The focus can sit on a row you can't see.**
  - "Back" by mouse returns to the main page at scroll 0, with the focus on the row that led away (`start.rs:462-472`).
  - At 200% that row can be off screen, and nothing scrolls to it, because `follow` is only set by keys.
  - Enter then acts on a row you cannot see.
- **Hover highlights through the footer.** Hover uses each row's whole rectangle, while a click uses only its visible part. Pointing at the footer can light up a row hidden under it, or change the shop's detail box, without the row being clickable there.
- **The title block is large.** It takes 160 units on every page: 304 px at 200% on 1080 lines, 28% of the height.

### 13. Frame cost, and what is not measured

**Measured:** the HUD's model and layout take 0.08 to 0.20 ms of CPU a frame.

**Not measured:**
- **The start menu's CPU cost.** The `look=` log's CPU line times `hud.draw` even on menu pages (`headless.rs:430-436`), so the menu pictures report the HUD's cost, not the menu's.
- **The shop's layout passes.** `shop_page` runs `shop_plan` up to about 10 times a frame in its shrink loop, then once more.
- **`text_fit`** is O(n²) on a cut line: it pops one character and measures again.
- **The 40-step shade on the start menu.** It is 40 full-height shapes over up to 720 units of width. My estimate is about 400k tile items a frame at 2560x1440 and 200%. One gradient shape would do.
- **The clip word's GPU cost on the 1070.** It is one load and one compare per shape per pixel.

**Fix:** time `menu.draw` and `Canvas::bin` in the `look=` log, and time the canvas pass on the 1070.

### 14. Code hygiene that makes the layout fragile

- **`BAR` is 1.0 and does nothing now.**
- **Three units are in play:**
  - `s` for the top bar, tooltips and menus;
  - `b` for the bar;
  - `gb` for the gun strip.
- **These are passed into the same helpers**, for example `glass(p, 12*s)` with the shrunken `s`.
- Once the row allocator exists, fold them back into one unit and delete `BAR`.

## Tests that would not catch what the owner would see

- **`every_ui_size_keeps_the_hud_on_screen`:**
  - RTS selection only, no squad panel.
  - It asserts hits on screen and the bar's height.
  - It does not check overlap, text inside panels, or a floor on text size.
  - It passes with the gun strip at 60%.
- **`squad_tiles_…`:** 100% only, no loadout. It passes with Heal drawn over the header at 150%.
- **No test** for:
  - the hover card's timing;
  - key letters shown against the keys the current card page binds;
  - a drag in a scrolled shop;
  - a row click on the UI size row;
  - the default window sizes (1600x972, 1600x1000).
- **The start menu tests** check that rows can be reached and clicked. They do not check what a drop does after a scroll.
- **Renders:** the `look=` log should print the commit it was built from. Today's review pictures came from a build that predates the merge it was reviewing.
