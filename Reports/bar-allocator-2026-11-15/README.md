# One allocator for the bottom bar (2026-11-15)

Step 2 of the [three-reviewer review](../menu-review-2026-11-15/README.md)
of the menus and HUD, and its main finding: the bar traded text for empty
space. The owner's ask behind it (11-15 02:56Z): "The text seems too small.
I feel like we're not always making the best use of space."
Page (before/after at five sizes, with counts):
https://claude.ai/artifact/2CtpbKUNspanYXBNJsDg1b, source in `page/`.

## What was wrong

Each panel gave way on its own threshold, one after another, and each
decided alone:

- The command card went to square icons when 340 units would not be left
  beside the full gun strip. That strip included its 124-unit name column
  and Swap.
- The gun strip then drew smaller, down to 60% with its words, when the
  selection kept under 300 units.
- The squad header stepped down on its own, and then dropped buttons.

So at 125% on 1920x1080 the card went to icons while the squad panel kept
about 450 px empty. At 200%, the gun's name was drawn about as small as at
100%.

## What it does now

`Hud::bar_fit` (`src/world/ui.rs`) starts the row bare:

- square command buttons;
- the gun's cards, its icon, and stubs for its empty slots;
- the selection at the least that shows what it holds (`Hud::sel_needs`).
  For squads that is every header button (Let go, Combine, Split, Guards,
  Heal) without keys, and three tiles. For soldiers it is their line and
  six cards.

Then it gives out the rest of the row in one order, each part only where it
still fits:

1. The command labels.
2. The squad buttons' key letters.
3. The gun's empty slots at full size.
4. Swap (H).
5. The gun's name column. Until it fits, the gun's icon stands in, and its
   tooltip carries the gun's and the loadout's names.

The selection takes what is left: wider tiles, then the "2 squads" word,
then 13.5-unit button text (the steps in `HEADER_STEPS`).

The order follows the designer's report: shed the header's sub-line, then
the strip's name column, then the empty slots, then the key letters, and
only then the labels. Squad buttons rank above all of these, because they
are orders: the owner made Heal an explicit order at 07:41Z, and the
review found it was the first button dropped.

The strip still draws smaller in one case. When even the bare row leaves
the squad panel short of every button (or 300 units of soldiers' cards),
the bare strip draws down to 60%, and by then it has no words. One list of
header labels (`header_labels`, `header_w`) is measured by the allocator
and drawn by `squad_tiles`.

## Measured

There are 900 layout cases:

- 5 screens: 1600x972 and 1600x1000 (the window the game opens on a
  1080p or 1440p monitor), 1280x800, 1920x1080 and 2560x1440;
- the 10 UI steps;
- 1, 4 or 9 squads, or 6 soldiers in RTS mode;
- guns of 2, 4 and 6 slots (`mould::guns`: 2 to 6, the rifle 4);
- the main page and the build page.

The rule before the allocator is worked out on the same rows. The table is
in `raw/row-cases.txt`, and the ignored test `bottom_row_printout` writes
it.

| | before | after |
|---|---|---|
| Card keeps its labels | 300 | 384 |
| Gun strip's words drawn under the UI size | 378 (210 at 60%) | 0 |
| Bare strip drawn smaller (no words) | n/a | 224: 4 at 150% (a 6-slot gun at 0.98), the rest at 175% and up |

Changes at particular sizes:

- **125% on 1920x1080** (one squad, a 4-slot rifle): the labels now stay,
  and so do all five squad buttons. The keys, the header word and the gun's
  name move off the bar.
- **125% on 2560x1440**: the same.
- **200% on 1920x1080**: the bare strip draws at 0.83 instead of the full
  strip at 0.6, and Guards, which was left off, is back.
- **150% on 1920x1080**: unchanged. The labelled card is 444 units wider
  than the square one, and only about 235 are spare.
- **1600x972 at 125%**: unchanged, because the labels miss by about 100
  units. This is the window the game opens at, which the review's
  question 3 is about. Question 2, moving Heal and Guards to the card's two
  empty squad-mode slots, frees about 150 units of the header, which is
  enough there.

Frame cost: `bar_fit` takes 12 µs a call (the mean of 10,000 calls at 125%
with four squads), against 85 µs for a whole HUD draw. The `look=` logs
read "model and HUD" at 0.09 to 0.20 ms a frame in both builds (one frame
each, too noisy to separate).

## Checks

Two new tests:

- `the_bottom_row_keeps_its_text_at_every_size` runs the 900 cases. It
  asserts that:
  - panels stay on screen and apart;
  - every text run in the bar sits inside a panel;
  - no run's glyph pixels touch another run's (`Canvas::runs`, test
    builds only; the ink boxes of each run);
  - no text is under 10.5 units (the square card's costs);
  - only the bare strip ever draws smaller, and only at 1.3 pixels a unit
    or more;
  - all five squad buttons are drawn unless the strip has hit 60%.

  It caught Guards dropped at 200% when the squad panel got its need to
  the last float, so the needs now carry a unit spare.
- `at_125_percent_on_1080p_the_card_keeps_its_labels`: labels at 125% with
  a whole-size strip, and everything at 100%.

Frames (`larder-world look=DIR`, llvmpipe), before = main at 2f6ac3f4,
after = this change, all with `mode=squads select=1 hand=squads
rounds=lance at=30`:

- `screen=1920x1080` with `ui=100%`, `125%`, `150%` and `200%`;
- `screen=1600x972` with `ui=100%` and `125%`;
- `screen=2560x1440 ui=125%`;
- the run case, `screen=1920x1080 ui=125% run=1 mode=squads select=1
  at=20`.

`raw/make_pics.py` makes the crops.

## Not changed

- The panel heights stay one row. The designer suggested a panel height
  that follows its content, but the owner ruled on 11-14 20:19Z that the
  bar is the same row at the same height.
- The command card's grid.
- The minimap's width.
- Steps 3 to 7 of the review.
- The owner's five questions.
