---
title: Salt
kind: tool
status: paused
updated: 2026-11-10
sources:
  - Reports/mould-probe-2026-11-07.md
  - Reports/mould-review-2-2026-11-10.md
  - Reports/mould-review-2026-11-10.md
  - Reports/mould-wild-one-2026-11-10.md
code:
  - Ground::Salt
  - Kill::Salt
  - SALT_LEN
  - SALT_STOCK
  - Tool::Salt
---

# Salt

## Now

A drag draws a salt wall three cells wide and up to `SALT_LEN` 60 cells long, one per dish
(`SALT_STOCK` 1), that never comes back. Salt is ground a walker cannot step
on; walkers under a new line die (`Kill::Salt`), and **scent does not cross
it**, so the mould cannot smell food behind a wall. Behind the Wall's wall
is the level's own salt, not the player's.

Which dishes give it: the free dish and The Wild One (1 each); the lesson
has a salt step and hands one back if the stock is empty. The three puzzle
dishes and Keep It Alive give none. In [The Wild One](../dishes/the-wild-one.md)
the opening salt wall alone wins (8-0) and is the whole first star; on the
free dish, review 1 never needed it.


## Owner's word

- 2026-11-08, after his first play: the salt and the snip were mysteries
  ([probe, *What the owner's play found*](../../Reports/mould-probe-2026-11-07.md)).
  The briefing card and tool hints were added for it ("DRAG: A SALT WALL THE
  MOULD CAN'T CROSS").

## Tried and rejected

- None about salt itself.

## Open

- Review 1 ranked **marks that stay** second: more salt, rubbed out by a
  click, so the player shapes the dish. Not tried.
- The Wild One's first star is the opening wall alone, so the middle of the
  game does not matter yet ([The Wild One](../dishes/the-wild-one.md)).
- Wild walkers killed by salt were once counted as snip kills; fixed
  (`Counts::salted`) in [wild-one, *Review 2 fixes*](../../Reports/mould-wild-one-2026-11-10.md).

## Sources

- [mould-probe-2026-11-07.md](../../Reports/mould-probe-2026-11-07.md), *What was built*, *What the owner's play found*, *Challenge dishes*.
- [mould-wild-one-2026-11-10.md](../../Reports/mould-wild-one-2026-11-10.md), *Numbers*, *Review 2 fixes*.
- [mould-review-2-2026-11-10.md](../../Reports/mould-review-2-2026-11-10.md), *The Wild One*.
- [mould-review-2026-11-10.md](../../Reports/mould-review-2026-11-10.md), *What playing showed*, *Fun, ranked*.
