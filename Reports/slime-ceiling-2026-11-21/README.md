# The slime follows its crystal (2026-11-21)

The thread "Make the slime a more interesting enemy". The owner's ruling at
06:30Z ("crystals never run out from eating or pumping, and only your squad
can empty one by draining it ... this 100%") and his pick of option 1 at
06:34Z (the slime's size follows the food it holds). Page:
https://claude.ai/artifact/N9YydWDBDBVSFwr7MvrvPQ (`index.html` here, with
its four end cards).

What was built (GPU only), and the measurements, are in
[`wiki/model/growth.md`](../../wiki/model/growth.md), *The run's ceiling*.

**End cards** (`film_ends`, `FILM_CASES=won`, planet 1 of run seed 1,
greedy; before = `FILM_KEYS=crystal_keep=0,set:ceiling=0`):

- 7:00 played: peak 14.1k before, 13.9k after; the squad keeps the slime
  under its ceiling, so the graphs match.
- The slime's only covered crystal (971 cells) emptied at 3:00
  (`FILM_DRAIN=180`): before 10.2k, 9.4k, 5.2k at 3:00, 3:30, 4:00 and freed
  at 6:00 with 14 soldiers lost; after 10.2k, 6.1k, 2.7k and freed at 5:21
  with 2 lost.

**Starts** (headless, `hand=greedy`, run seeds 1-3, at 1:00/2:00/3:00):
before 11,261/8,663/9,078, 15,054/13,641/13,273, 11,282/9,448/9,360; after
11,593/10,308/9,477, 15,497/13,133/12,586, 11,336/9,439/9,465. 8 of 9 within
5%.

**Rejected:** upkeep x2 all the time (1:00 slime -29%), x3 (-52%); a
ceiling floored at the landing walkers.
