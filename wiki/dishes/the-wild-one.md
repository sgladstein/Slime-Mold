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

## Numbers

Scored by share of food eaten, 8 seeds, `mouldprobe level=5`:

| Hand | Share | Stars | Source |
|---|---|---|---|
| doing nothing | 0.494-0.526 | 0 on 8 | [wild-one, *Review 2 fixes*](../../Reports/mould-wild-one-2026-11-10.md) |
| random | 0.467-0.559 | 0 on 8 | same |
| `jail` (salt ring round the nest) | 0.489-0.517 | 0 on 8 | same |
| warden, lets go at 30 s | 0.567-0.577 | 1 on 7 | same |
| warden, clumsy (`HJIT=6 HLAG=2`) | 0.575-0.584 | 1 on 8 | same |
| warden, 3 cells off (`HJIT=3`) | 0.583-0.616 | 1 on 8 | same |
| warden, exact | 0.642-0.722 | 2 on 5, 3 on 3 | same |

- Warden beats doing nothing and random 8-0 each (p 0.008). Clumsy against
  letting go at 30 s: 7-1 (p 0.07), no star moves.
- Warden with against without the snip: exact 4-4, clumsy 3-5; the snip
  moves nothing ([knife](../tools/knife.md)).
- Each bar sits in a gap under one point wide between hands.
- Earlier, scored by walkers at the bell (16 seeds): warden beat doing
  nothing and random 16-0 (p < 0.0001); one tool alone, 8 seeds: lamp 8-0
  and salt wall 8-0 (p 0.008 each), oats 6-2 (p 0.29), snip 5-3 (p 0.73)
  ([wild-one, *Numbers*](../../Reports/mould-wild-one-2026-11-10.md)).
- Why the score changed: at the bell four of seven piles run dry together
  and the player's mould lost 28-54% in the last ten seconds, so the lead
  moved by up to half (seed 3: 7.6x at 140 s, 3.6x at the bell). Share of
  food eaten only adds up, so a crash cannot move it
  ([review 2, *The Wild One*](../../Reports/mould-review-2-2026-11-10.md)).
- The two-mould step was read in full and checked every 15 steps over four
  games: 0 shared cells, 0 wild walkers off agar, 0 pile-food difference.

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
