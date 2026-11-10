---
title: The knife (snip)
kind: tool
status: open
updated: 2026-11-10
sources: [Reports/mould-probe-2026-11-07.md, Reports/mould-wild-one-2026-11-10.md, Reports/mould-review-2-2026-11-10.md, Reports/mould-review-thread-handoff-2026-11-10.md]
code: [Tool::Snip, Tools::snips, Params::wound, Params::pieces, Ground::Wound]
---

# The knife (snip)

## Now

A snip cuts a line through the mould. The cut cells become wound ground
that the mould cannot cross for 4 s (`Params::wound`), then heal back to
agar. Because each piece eats only the food it is joined to
(`Params::pieces`, on), a piece cut off from its food goes hungry, farthest
from food first. Of the challenge dishes, only
[The Wild One](../dishes/the-wild-one.md) gives snips (3); the lesson has a
snip step and hands one back if the stock is empty.

**It is the weakest tool today.** In The Wild One the snip alone does
nothing, three cuts through the wild nest change nothing, and taking it away
from the scripted warden changes nothing. A cut heals, and a trunk between
two fed piles leaves both sides fed.

## Numbers

| Measure | Result | Seeds | Paired | Source |
|---|---|---|---|---|
| Old snip (no wound): grey tube in a whole scripted game | 0 cells at 30, 60, 82 s; healed in about 0.5 s | | | [probe, *Making a cut matter*](../../Reports/mould-probe-2026-11-07.md) |
| Wound snip across the trunk at 40 s, most grey tube in 12 s | 14, 139, 308, 40, 12 cells vs 26, 80, 74, 31, 21 for the old snip | 5 | not paired (different cut places) | same |
| The Wild One, snip only vs doing nothing (walker count) | share 0.44-0.59, 0 stars | 8 | 5-3, p 0.73 | [wild-one, *Numbers*](../../Reports/mould-wild-one-2026-11-10.md) |
| Three snips through the wild nest at 0.5 s (`nestsnip`) | 0 stars on 8 | 8 | 7-1 vs doing nothing, p 0.07 | [review 2](../../Reports/mould-review-2-2026-11-10.md) |
| Warden with vs without the snip (food share) | exact 4-4, clumsy 3-5 | 8 | no difference | [wild-one, *Review 2 fixes*](../../Reports/mould-wild-one-2026-11-10.md) |

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
