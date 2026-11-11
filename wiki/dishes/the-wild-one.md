---
title: The Wild One (a rival mould)
kind: dish
status: paused
updated: 2026-11-10
sources: [Reports/mould-wild-one-2026-11-10.md, Reports/mould-review-2-2026-11-10.md, Reports/mould-review-thread-handoff-2026-11-10.md]
code: [Win::Rival, Dish::rival, Dish::food_share, Dish::ate, Dish::wild_ate, Params::fight, Params::table, WILD_START, WILD_REPEL]
---

# The Wild One (a rival mould)

## Now

The fifth challenge dish: a wild violet mould, a whole second `Dish` under
the same rules with nobody steering, shares seven mirrored piles with yours
for 150 s at 22.5 degrees. It starts bigger (`WILD_START` 600 walkers against
your 400). Both eat from one table per pile, split by head count, and walkers
die fighting where the two touch. Tools: 4 oats (one back per 15 s), 1 salt,
3 snips. It costs about +1.4 ms a frame.

Since review 2 it scores **your share of all the food both moulds ate**
(`Dish::food_share`), not walkers at the bell: stars at 57%, 62% and none of
it left (`Win::Rival { bars: [0.57, 0.62] }`). Doing nothing and random get
0 stars; the exact scripted warden gets 2 or 3. But the first star is the
opening salt wall alone, and the second and third are an aiming test (within
about 2 cells), so **the middle of the game does not yet matter at a
person's aim**. The snip moves nothing here. Waits on the owner's play.


## Owner's word

- 2026-11-10 03:00Z: "Also wondering about having multiple molds that can
  fight or steal resources from each other" (review thread).
- 2026-11-10 03:06Z: "Agreed" to a wild second mould, another colour, same
  rules, nobody steering, that steals from the same piles and fights at the
  seam. The bar: the player's hand must change which mould ends bigger,
  paired over seeds; quote the frame cost.

## Tried and rejected

- Per-mould tables: every hand ended near half the dish; nothing to contest.
- Wild repel 0.3 or 0 without fights: the wild one took 90-99% whatever the hand.
- Shared table without fights, repel 1.0: doing nothing won 9 of 16, too generous.
- Three late piles on the seam (0:45, 1:20, 1:50): added luck, not play.
  [`dead-ends.md`](../../Reports/dead-ends.md), *The Wild One's late piles*;
  re-test when the player can see food coming early enough to prepare.

## Open

- **Make seconds 30-150 matter at a person's aim.** `HJIT` moved lamp,
  snips and oats together, so which needs the precision is not known yet;
  separate them before trying again.
- **The snip**: make a cut-off wild piece visibly starve, or take the snip
  off this dish. See [the knife](../tools/knife.md).
- Check the 57% and 62% bars on 16+ seeds.
- Wild walkers are drawn with a first-pass violet ramp (`render::wild`).

## Sources

- [mould-wild-one-2026-11-10.md](../../Reports/mould-wild-one-2026-11-10.md): the build, numbers, tuning, the score change, open steps.
- [mould-review-2-2026-11-10.md](../../Reports/mould-review-2-2026-11-10.md), *The Wild One*: openings, clumsy hands, the crash at the bell.
- [mould-review-thread-handoff-2026-11-10.md](../../Reports/mould-review-thread-handoff-2026-11-10.md): owner's words with times.
