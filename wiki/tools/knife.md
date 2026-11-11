---
title: The knife (snip)
kind: tool
status: paused
updated: 2026-11-11
sources: [Reports/mould-shooter-2026-11-11.md, Reports/mould-probe-2026-11-07.md, Reports/mould-wild-one-2026-11-10.md, Reports/mould-review-2-2026-11-10.md, Reports/mould-review-thread-handoff-2026-11-10.md]
code: [Tool::Snip, Tools::snips, Params::wound, Params::pieces, Ground::Wound]
---

# The knife (snip)

## Now

A snip cuts a line through the mould. The cut cells become wound ground
that the mould cannot cross for 4 s (`Params::wound`), then heal back to
agar; a snip over an open wound keeps it open from then (fixed 2026-11-11: before, only agar was wounded, so a held line of snips healed all at once, [Larder correction](../../Reports/mould-shooter-2026-11-11.md)). Because each piece eats only the food it is joined to
(`Params::pieces`, on), a piece cut off from its food goes hungry, farthest
from food first. Of the challenge dishes, only
[The Wild One](../dishes/the-wild-one.md) gives snips (3); the lesson has a
snip step and hands one back if the stock is empty.

**It is the weakest tool today.** In The Wild One the snip alone does
nothing, three cuts through the wild nest change nothing, and taking it away
from the scripted warden changes nothing. A cut heals, and a trunk between
two fed piles leaves both sides fed.


## Owner's word

- 2026-11-08, after his first play: the salt and the snip were mysteries
  (the report's paraphrase, not a quote;
  [probe, *What the owner's play found*](../../Reports/mould-probe-2026-11-07.md)).
- 2026-11-08, card: "Yes, each piece eats its own food" (quoted in the
  `Params::pieces` doc comment). That made a cut able to starve a piece.

## Tried and rejected

- The storm (a cut the dish makes, not the player) cutting three or two
  tubes: the toughness star was never earned. One cut shipped.
  [`dead-ends.md`](../../Reports/dead-ends.md), `STORM_CUTS`.

## Open

- **Make a cut-off piece visibly starve, or take the snip off The Wild
  One** (third in the [hand-off's next steps](../../HANDOFF.md)).
- The probe report's note: "a trunk between two fed piles is the wrong test
  of starving"; what a per-piece store changes is a piece with more mould
  than its food can feed. No test of that case yet.
- Wild walkers killed by salt used to be counted as snip kills; fixed in
  review 2's small fixes (`Kill::Salt`).

## Sources

- [mould-probe-2026-11-07.md](../../Reports/mould-probe-2026-11-07.md), *What the owner's play found*, *Making a cut matter*.
- [mould-wild-one-2026-11-10.md](../../Reports/mould-wild-one-2026-11-10.md), *Numbers*, *Review 2 fixes*.
- [mould-review-2-2026-11-10.md](../../Reports/mould-review-2-2026-11-10.md), *The Wild One*.
- [mould-review-thread-handoff-2026-11-10.md](../../Reports/mould-review-thread-handoff-2026-11-10.md), next steps.
