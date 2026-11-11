---
title: The lesson and the three puzzle dishes
kind: dish
status: paused
updated: 2026-11-10
sources: [Reports/mould-probe-2026-11-07.md, Reports/mould-review-2026-11-10.md, Reports/mould-wild-one-2026-11-10.md]
code: [Lesson, Lesson::dish_with, Dish::best_join, Dish::storm_at, Params::stray_reach, Params::stray_grace, Params::oats_grace, STRAY_REACH, Route]
---

# The lesson and the three puzzle dishes

## Now

The window opens on **the lesson** (`mould::lesson`, seed 0): six steps,
each judged from the dish's own counters and up at least 4 s: hold the lamp
near the mould, oats eaten, join all the food, a snip that cut mould, draw
salt, then the storm, held back until that step (`Dish::storm_at`). The
dish's piles stay full until the storm step, and the snip and salt steps hand
one back if the stock is empty. A scripted hand passes all six at 51 s.

Then three hand-made dishes (`level.rs`), each one question, oats and lamp
only, no storm, 45 degrees:

- **Behind the Wall** (90 s): food behind a level salt wall with one gap;
  scent does not cross salt, so the player leads the mould to the gap. Stars
  for joining at all, by 1:00, by 0:45.
- **Beat the Ruler** (100 s): three foods in a triangle, scored on the
  shortest join through the tube (`Dish::best_join`); stars at all joined,
  0.98x and 0.95x of the ruler, best possible 0.87x. Review 1 called it the
  best of the three: its answer is a join in empty space.
- **Running Dry** (120 s, oats back every 10 s): small piles, a new one
  every 20 s further on; stars at 100, 200 and 300 mould alive at the bell.

All three pass their bar at 45 degrees (doing nothing 0, random at most 1,
the intended hand `play::Route` 3 on all 8 seeds), with one known gap:
Running Dry's random hand takes 3 stars on seed 7. On a hand-made dish a
stray only starts eating within 24 cells of fed mould (`STRAY_REACH`), which
is what stops doing nothing joining by drift. Keep It Alive grew out of
Running Dry ([Keep It Alive](keep-it-alive.md)).

## Numbers

| Measure | Result | Seeds | Paired | Source |
|---|---|---|---|---|
| Ship bar: doing nothing / random / intended, stars | Wall 0 / one 1 / 3s; Ruler 0 / 0 / 3s; Dry 0 / seed 7 only 3 / 3s | 8 | | [probe, *Review 2 fixes*](../../Reports/mould-probe-2026-11-07.md) |
| Intended hand's result | Wall joined 0:10-0:11; Ruler 0.87-0.92x; Dry 354-411 alive | 8 | | [probe, *Review 1 fixes*](../../Reports/mould-probe-2026-11-07.md) |
| Running Dry, doing nothing died at, 45 vs 22.5 degrees | 1:02-1:24 vs 1:44-1:50 or the bell | 8 | later at 22.5 on 8 of 8, p 0.008 | [review 1, *What playing showed*](../../Reports/mould-review-2026-11-10.md) |
| Running Dry, random alive at the bell, 45 vs 22.5 | 1 of 8 vs 6 of 8 | 8 | 5-0, 3 level, p 0.06 | same |
| Planned hand at sensor reach 9 / 8 / 7 / 6 | Wall 33333333 / 33333333 / 13333323 / 20031231; Ruler 33333333 / 30330300 / 00000000 / 00000000 | 8 | | [wild-one, *Finer tubes*](../../Reports/mould-wild-one-2026-11-10.md) |
| Lesson, scripted hand | all six steps, ends at 51 s, 2 stars | 1 | | [probe, *What the owner's play found*](../../Reports/mould-probe-2026-11-07.md) |
| A slow lesson before the refill | 0 of 400 walkers at 200 s left alone | 1 | test `a_slow_lesson_does_not_starve` | same |

## Owner's word

- 2026-11-08, after his first play: "I didn't really know what I was doing",
  and there "needs to be a challenge" ([probe, *What the owner's play
  found*](../../Reports/mould-probe-2026-11-07.md)). The lesson answers the
  first, the three dishes the second.

## Tried and rejected

- Behind the Wall's brainstorm layout, 207 cells round: an oat relay died
  after carrying the front 98 cells; now 157 ([probe, *Challenge
  dishes*](../../Reports/mould-probe-2026-11-07.md)).
- Beat the Ruler scored on tube area: area follows food joined, not shape
  (doing nothing read 1.40x). Scored on the shortest join instead.
- Running Dry's brainstorm bars (300/600/900 alive): out of reach, the bell
  always finds the mould between piles.
- The 2 s `stray_grace` applied at all times let the intended hand join
  round the side of Beat the Ruler (1.10x, seed 7); it applies only while
  nothing on the dish is fed ([probe, *Review 1 fixes*](../../Reports/mould-probe-2026-11-07.md)).
- Shorter sensor reach for finer tubes: Beat the Ruler loses half its seeds
  at 8. [`dead-ends.md`](../../Reports/dead-ends.md), `Params::sensor_offset`.

## Open

- Review 1: keep the three as a side pack at 45 degrees; they fail their bar
  at 22.5 ([review 1, *Fun, ranked*](../../Reports/mould-review-2026-11-10.md)).
- Running Dry's random seed 7 (3 stars, 723 alive): the 5-seed bar is not an
  order statistic yet; not fixed.

## Sources

- [mould-probe-2026-11-07.md](../../Reports/mould-probe-2026-11-07.md), *What the owner's play found*, *Challenge dishes*, *Review 1 fixes*, *Review 2 fixes*.
- [mould-review-2026-11-10.md](../../Reports/mould-review-2026-11-10.md), *What playing showed*, *Fun, ranked*.
- [mould-wild-one-2026-11-10.md](../../Reports/mould-wild-one-2026-11-10.md), *Finer tubes*.
