# Menus and HUD review, three reviewers (2026-11-15)

Owner, 11-15 08:06Z: "After you finish your work, spin up a 3-agent review
of your work and the general current state of our menu system, in game,
squad, build, tools, HUD, shop."

Three reviewers read the code at d5af42ab (the UI-size branch, PR #334) and
looked at renders of every screen: a game designer and player
([designer.md](designer.md)), a technical lead for the UI code
([tech-lead.md](tech-lead.md)) and an interface-consistency reviewer
([consistency.md](consistency.md)). Their shared brief is
[brief.md](brief.md). The owner's copy is a Claude Doc,
https://claude.ai/artifact/CjbNqLt6ViKTaTt4L86WN5; this file is the record.

The picture paths in the three reports were in the session's scratchpad and
are not kept; each report gives the `larder-world look=` command for its
frames, so they can be made again.

## The verdict

- **The approach is right and cheap.** One UI unit scales everything and
  pages scroll instead of shrinking. The HUD's model and layout take 0.08 to
  0.20 ms of CPU a frame (9 `look=` logs).
- **The bottom bar trades text for empty space.** Each panel falls back on
  its own threshold (card labels under 340 units, gun strip under a 300-unit
  selection, the squad header's four steps). At 125% on 1920x1080 the
  command card goes to icons while the squad panel keeps about 450 px empty;
  at 200% the gun strip draws at 0.6, so the gun's name is 17 px at 200%
  against 22 px at 150%.
- **Key letters that do something else.** With the build page open the
  squad panel shows Split Y, Combine J, Guards I and Heal L; the card routes
  those letters first, to Siren, Lures on/off, Outpost and Lure.
- **The hover card can still come up mid-volley.** A click puts away only
  the card already up; when the aim slides onto the home crystal while the
  button is held, the key changes and a card appears 0.35 s later.

Two bugs came in when main's #330 (Heal is a squad order) merged into the
UI-size branch, and #334 landed with them at 09:41Z; the pull request that
adds this report fixes both, with tests: Heal was outside the header's fit and
drew over "1 squad" at 150% (`the_squad_header_keeps_clear_at_every_size`),
and a card dropped in a scrolled shop went to the wrong slot because the
slot list was filtered to the visible ones
(`a_drop_after_scrolling_lands_in_its_own_slot`). See
[the UI-size report](../ui-scale-2026-11-15/README.md#review-fixes).

## Questions for the owner

1. Restart during a run is a free retry of the planet: keep, cost, or
   remove? (Suggested: rename "Restart this planet" and confirm.)
2. Heal and Guards into the two empty squad-mode slots of the command card?
   The 11-14 ruling put Combine and Split on the squad panel; Heal and Guards
   are not covered.
3. The window opens at 90% of the monitor, capped at 1600x1000: 1600x972 on
   1080p, so "100%" draws at 0.9. Open maximized?
4. 125% as the default on 1080p, once the bar keeps its labels there?
5. A confirm step for Give up the run (12 units from Land, next in keyboard
   order) and Quit during a run.

## Recommended order

1. Small fixes: no hover card while a mouse button is held or WASD steers,
   flip instead of slide, squad wording, a timing test; key letters only
   where they work; an Options row click moves the focus (it resets the row
   today, so one click takes 200% back to 100%) and All to defaults keeps
   the UI size; drop the squad header's second line and fix the AUTO tile's
   state word; in a run the gun strip names the gun and has no H; a lock,
   not a red price, on locked buildings.
2. One allocator for the bottom row: each panel gives a minimum and a
   preferred width, spare width goes out in one stated order, panels shed
   parts before text shrinks. A layout test over every UI size, the default
   window sizes (1600x972, 1600x1000), squads and RTS, 1 to 9 squads: no
   overlap, no cut text, no text under its size, true keys.
3. A goal bar and star pips on the HUD for run planets and missions.
4. Shop and gun range: pinned slot row, compact gun cards with differences
   from the carried gun, a one-line header, the card's text by the card.
5. Start pages: one frame (compact header, fixed footer, scrollbar by the
   list), Options grouped with Display first, notes in a side panel.
6. Pause, F1 and end screen: one bindings table for both key cards and
   every key letter, F1 scrolls, the end screen wraps and shows stars, the
   pause menu takes arrows and Enter and reaches the UI size, a note when
   Ctrl + hits the cap.
7. A token pass: 20 text sizes to six roles (12, 14, 15, 18, 24, 40), one
   meaning per icon, alert icons by kind.

## Findings, ranked by play

D designer, T technical lead, C consistency.

| # | Finding | Found by |
|---|---|---|
| 1 | The bar trades text for empty space (125% to 200%) | D T C |
| 2 | Squad key letters that the build page takes | T C |
| 3 | Hover card mid-volley; slides under the cursor; RTS wording in squad mode | D T |
| 4 | Run goal reads "(0%)" for a minute; no star conditions shown | D |
| 5 | Shop and range: slots and cards not on screen together at 150%+ | D C |
| 6 | Give up the run, Restart and Quit act on one click | D |
| 7 | The window opens at 1600x972, so "100%" is 0.9 | T |
| 8 | A click on the UI size row resets it | T C |
| 9 | UI size is the last of 15 options | D C |
| 10 | Cut text: end screen, F1, Keys page, option notes, squad sub-line | T |
| 11 | The gun strip names a preset ("Plain"), not the gun; H only refuses in a run | D T C |
| 12 | UI steps over the cap change nothing (200-300% all 1.9 on 1080p) | T C |
| 13 | A locked building's price is red like "can't afford" | D C |
| 14 | The command card is a 4x3 grid poured into 6x2 | C |
| 15 | The end screen shows no stars or run progress | D |
| 16 | One icon, several meanings; every alert a warning sign | D C |
| 17 | The objective covers the resources at 1280x800 200% | T |
| 18 | Start-page scrolling polish (bar placement, touchpad, focus after Back) | T C |
| 19 | First launch: the campaign teaches RTS keys, squads is the default | D |
| 20 | Tests check hits, not what the owner sees | T |

The rows, causes and file:line references are in the three reports.

## Checked and guessed

Looked at 23 renders (every start page and the HUD at 100% and 200%,
1280x800 to 2560x1440) and 12 more frames the reviewers made. Read in code
only: the pause menu, F1 card, end screen, squad-mode build page and the
hover card (it runs on wall-clock time). The first renders came from a
build made before #330, so they had no Heal button; the technical lead
rebuilt HEAD and found the overlap that way. Guessed: whether the owner
maximizes the window, the designer's 14 px floor, the comparisons with
other games (from memory), and that quitting loses a run (no run save
found). The start menu's own frame cost was not measured: the `look=` log
times `hud.draw` on menu pages too.
