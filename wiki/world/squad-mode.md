---
title: Squad mode
kind: design
status: open
updated: 2026-11-15
sources:
  - Reports/bug-sweep-2026-11-14.md
  - Reports/build-flow-2026-11-14/README.md
  - Reports/free-camera-2026-11-15/README.md
  - Reports/narrow-squads-2026-11-15/README.md
  - Reports/squad-mode-2026-11-14/README.md
code:
  - Game::heal
  - Game::rank_squad
  - Rules::funnel
  - Rules::guard
  - Rules::squads
---

# Squad mode

## Now

You steer one squad of soldiers with WASD; it volleys at the cursor while the
left button is held, and fires twice as often with a 1.4x strike while it is
the one you command. The squads you do not command keep their last order and
fire on their own. Squads split, join, link, queue orders and stand guard while
they drain or build; there is no larder. It is `mode=squads` (`Rules::squads`),
the start menu's *Control* option. It is the window's default (2026-11-15, #330; owner 07:57Z); missions and
headless `look=`/`bench` runs stay on the RTS round (`mode=rts` or Control: rts
brings it back), so the campaign keeps the old mode (owner 08:09Z). Detail: [report](../../Reports/squad-mode-2026-11-14/README.md).

## How it plays

- **Commanding.** The bottom bar's middle panel has a tile per squad (hotkey,
  soldiers, health, state). Click a tile or press its number to take command;
  Ctrl + wheel switches. **Esc** or *Let go* commands none: every squad shows
  AUTO, WASD pans, and a click on a squad takes it back
  ([free camera](../../Reports/free-camera-2026-11-15/README.md)).
- **Split, join, link.** Y splits the front half off; J or the tile's + joins
  only a squad that touches yours. Linked squads (Ctrl + number, only while
  touching) are commanded as one and keep their own guns and settings.
- **Orders.** Right click sends, Ctrl + right click drains or guards a crystal,
  Shift queues up to six. Formations: blob (default), line, wedge, ring.
- **Guards.** A squad draining or building with slime near posts a share of
  itself as guards (key I: none, some, more); 3 of 8 cost about a fifth of the
  work (`cover.rs`).
- **Healing is an order** (#330): tell a squad to heal (Heal button, key L; the
  whole link). It stops, each member heals after 1 s still, paying food from the
  bank, and stops when all are whole or on any other order
  (`Game::heal`, `Cmd::Heal`). Nothing heals on its own.
- **Narrow gaps.** Squads funnel into columns through bridges and doors
  narrower than their formation (`Rules::funnel`, on)
  ([report](../../Reports/narrow-squads-2026-11-15/README.md)).
- **Building** is shared with the RTS round: ghosts soldiers raise from reach 6,
  Shift queues, an unfinished one can be called off for the whole price
  ([report](../../Reports/build-flow-2026-11-14/README.md)).

Income and crystals: [economy](economy.md). Guns: [guns](guns.md).

## Owner's word

- 2026-11-14 04:08Z: "the unit control should be squad based from the ground up. All soliders in a squad stick together at least as well as they can."
- 2026-11-14 04:49Z: "This should be built as a seperate mode for right now. Not erase the other game."
- 2026-11-15 07:57Z: "squad mode is default mode".
- 2026-11-15 08:09Z: "No campaign can stay the old mode".
- 2026-11-15 07:41Z: "Healing should be a specific action".
- 2026-11-15 02:58Z: a way to deselect, move with WASD and see each squad run on its own (built as Esc and AUTO).

## Open

- A call-in delay for bought soldiers.

## Sources

- [squad mode](../../Reports/squad-mode-2026-11-14/README.md), [bug sweep](../../Reports/bug-sweep-2026-11-14.md), [free camera](../../Reports/free-camera-2026-11-15/README.md), [narrow squads](../../Reports/narrow-squads-2026-11-15/README.md), [build flow](../../Reports/build-flow-2026-11-14/README.md).
