---
title: Oats
kind: tool
status: paused
updated: 2026-11-10
sources:
  - Reports/mould-keep-alive-2026-11-10.md
  - Reports/mould-probe-2026-11-07.md
  - Reports/mould-review-2026-11-10.md
  - Reports/mould-wild-one-2026-11-10.md
code:
  - Dish::oats_would_start
  - OATS_FOOD
  - OATS_R
  - OATS_REGROW_SECONDS
  - OATS_SECONDS
  - OATS_STOCK
  - Params::oats_grace
  - Params::stray_reach
  - STRAY_REACH
  - Tool::Oats
---

# Oats

## Now

A click lays a small food pile (`OATS_R` 3 cells, `OATS_FOOD` 400) that
smells like any pile, so the mould grows toward it and eats it. Oats go off
after `OATS_SECONDS` 35 s. **Oats are the only tool that leads the mould
somewhere new**; the lamp steers and shapes, salt and snips take away. A
visible lean (15+ walkers within 15 cells) comes 0.45-0.85 s after laying.

Stock and regrowth are set per dish: the free dish and the three puzzle
dishes hold 5, one back every 15 s (10 s on Running Dry); The Wild One 4,
one back every 15 s; [Keep It Alive](../dishes/keep-it-alive.md) 6 that
never come back. On a hand-made dish an oat is only started by mould within
24 cells of fed mould (`STRAY_REACH`), or on ground that was fed in the last
8 s (`Params::oats_grace`); the preview says so ("Only starving strays
here"). At 45 degrees play with oats is dot-to-dot: each move is the next oat
about 20 cells further along a line (review 1).


## Owner's word

- None recorded about oats alone. The 2026-11-08 playtest (testers) found a
  fresh player had spent all five oats by about 20 s and had nothing to
  decide for a minute; that drove `OATS_REGROW_SECONDS`.

## Tried and rejected

- **Oats exempt from the leash outright** (review 2's option a): a random
  hand took 3 stars on Behind the Wall (seeds 1, 6, 8) and Running Dry
  (seeds 2, 4, 7). An 8 s grace over the whole 27-cell leash let strays
  start the intended hand's oats on Beat the Ruler (1.16x, 1 star on seed 7).
  Both in [probe, *Review 2 fixes*](../../Reports/mould-probe-2026-11-07.md).
- A stray rule with no reach limit let doing nothing join all of Beat the
  Ruler and a random hand take 3 stars ([probe, *Challenge
  dishes*](../../Reports/mould-probe-2026-11-07.md)); hence `stray_reach`.

## Open

- Review 1 ranked "oats as a nudge, not a ruler" third (fewer, earned, or
  weaker), only together with marks that stay. Not tried.
- In The Wild One, which of oats, lamp and snips needs the 2-cell aim is not
  separated ([The Wild One](../dishes/the-wild-one.md), *Open*).

## Sources

- [mould-probe-2026-11-07.md](../../Reports/mould-probe-2026-11-07.md), *Measurements*, *Playtest fixes*, *Challenge dishes*, *Review 2 fixes*.
- [mould-review-2026-11-10.md](../../Reports/mould-review-2026-11-10.md), *What playing showed*, *Fun, ranked*.
- [mould-wild-one-2026-11-10.md](../../Reports/mould-wild-one-2026-11-10.md), *Numbers*.
- [mould-keep-alive-2026-11-10.md](../../Reports/mould-keep-alive-2026-11-10.md), *Numbers*.
