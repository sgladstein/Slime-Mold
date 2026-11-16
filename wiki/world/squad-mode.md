---
title: Squad mode
kind: design
status: open
updated: 2026-11-16
sources:
  - Reports/bug-sweep-2026-11-14.md
  - Reports/build-flow-2026-11-14/README.md
  - Reports/command-card-2026-11-15/README.md
  - Reports/free-camera-2026-11-15/README.md
  - Reports/guards-2026-11-15/README.md
  - Reports/narrow-squads-2026-11-15/README.md
  - Reports/soldier-cap-2026-11-16/README.md
  - Reports/squad-mode-2026-11-14/README.md
code:
  - Game::demolish
  - Game::guard_zone
  - Game::heal
  - Game::rank_squad
  - Rules::funnel
  - Rules::guard
  - Rules::max_squad
  - Rules::squads
  - squad::UNITS_MAX
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
Up to 200 soldiers since 2026-11-16 (`Rules::max_squad`; 40 before; owner
05:06Z picked "Flat 200, the realistic max": "I want to test the max"); a
soldier costs the CPU about 0.01 ms a step on planet 1 and 0.03 ms on a 6x
planet ([report](../../Reports/soldier-cap-2026-11-16/README.md)); the 1070
timing and balance at 200 are still to come.

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
- **Lookouts** (called Guards until 2026-11-15 19:51Z, so the word stays the
  Guard order's). A squad draining or building with slime near posts a share of
  itself as lookouts (the card's Lookouts, key I: none, some, more) at the work's edge on the slime's
  side; 3 of 8 cost about a fifth of the work (`cover.rs`). Posted out in the
  slime's film (#276) they died for nothing; at the edge they killed a third
  more slime than none and the workers were bitten less, 4 of 4 seeds
  ([report](../../Reports/guards-2026-11-15/README.md)).
- **Guard** (owner 2026-11-15 19:51Z, in Burn a line's place, which is gone):
  F, then click a spot, a building or a crystal. The squad walks there as one
  and shoots slime in the zone on top of its own defence: `Rules::guard_r` (8
  cells) round a spot, `GUARD_RIM` (3) past a crystal's rim or a building's
  footprint (`Game::guard_zone`, `Squad::guard`, `Order::Guard`); a ring shows
  the zone. Steering or another order ends it. Defend stays the default fire
  mode ([report](../../Reports/command-card-2026-11-15/README.md)).
- **The command card** is Card 2 (owner 19:38Z): 5 x 2, Attack-move, Guard,
  Drain, one fire button that cycles hold, defend and free (Z, X, C pick one),
  Heal; Buy, Pump, Build, Tools, Lookouts. The Build page (option 1, 19:53Z):
  turret, pump, wall, lure, siren, then the lures' switch and Back. With no
  squad commanded a click on a building selects it: its health, level and
  state, and its card (call off, demolish for half its price back, upgrade a
  pump, switch a lure).
- **Healing is an order** (#330): tell a squad to heal (Heal on the card, key L; the
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
- 2026-11-15 19:38Z: "Card 2"; 19:51Z: "Remove burn line. Use options c for guard/defend. (b) Click a building to select it when no squad is selected."; 19:53Z: "Build option 1"; 21:37Z: "Demolishing a building giving half back is good. Canceling a partial build should give all of it back".
- 2026-11-15 18:00Z, on guards while working: "It doesn't seem to work well. Either improve it or remove it." (Improved: posts at the work's edge.)
- 2026-11-15 02:58Z: a way to deselect, move with WASD and see each squad run on its own (built as Esc and AUTO).

## Open

- A call-in delay for bought soldiers.

## Sources

- [squad mode](../../Reports/squad-mode-2026-11-14/README.md), [bug sweep](../../Reports/bug-sweep-2026-11-14.md), [free camera](../../Reports/free-camera-2026-11-15/README.md), [narrow squads](../../Reports/narrow-squads-2026-11-15/README.md), [build flow](../../Reports/build-flow-2026-11-14/README.md).
