---
title: Keep It Alive (the endless dish)
kind: dish
status: paused
updated: 2026-11-10
sources: [Reports/mould-keep-alive-2026-11-10.md, Reports/mould-keep-alive-luck-2026-11-10.md, Reports/mould-review-2-2026-11-10.md, Reports/mould-review-thread-handoff-2026-11-10.md, Reports/mould-wild-one-2026-11-10.md]
code: [Win::Endless, ENDLESS_MIN, GAP_MAX, Dish::awake_at, Tools::water]
---

# Keep It Alive (the endless dish)

## Now

The fourth challenge dish: Running Dry at 22.5 degrees with no bell. A new
pile lands every 20 s, each further away (gap 70 + 6 per pile, at most
`GAP_MAX` 170 cells); you have 6 oats that never come back, the lamp and
[water](../model/water-and-sleep.md). It ends when fewer than `ENDLESS_MIN`
(20) walkers are left, and scores how long it lived, timed to the last moment
20 were awake (`Dish::awake_at`). Stars at 4:15, 5:30 and 6:40
(`Win::Endless { bars: [255, 330, 400] }`).

It works as built (doing nothing and random get 0 stars on 16 of 16 seeds;
the water hand beats both on 15 of 16), but it is **not yet a game you get
better at**: water-hand lives fall in two humps, dead by about 5:00 or past
7:20, and on this evidence which hump is luck, not skill. The 5:30 star was
earned once in 32 water-hand runs. **What decides the hump** (traced
2026-11-10, exploratory): the first pile that lands while most of the mould
is asleep. With a sleeper within 45 cells of it the game went long 13 times
and short 2; without, long 7 and short 26 (48 games, p 0.00003). It is luck
because the landing spot shows only 5 s ahead, after the mould has been put
to sleep. Waits on the owner's play.

## Numbers

| Measure | Result | Seeds | Paired | Source |
|---|---|---|---|---|
| Lived, doing nothing | 1:08-3:48 (median 2:10), 0 stars | 16 | | [keep-alive, *Numbers*](../../Reports/mould-keep-alive-2026-11-10.md) |
| Lived, random | 1:04-3:29 (2:03), 0 stars | 16 | | same |
| Lived, oat-saver (no water) | 2:47-5:26 (3:34), 1 star on 3 | 16 | beats doing nothing 14-1, p 0.001 | same |
| Lived, waterer (`sleeper`) | 2:51-10:42 (4:41) | 16 | beats oat-saver, random, doing nothing 15-1 each, p 0.0005 | same |
| Bake without water vs oat-saver | a few seconds shorter | 8 | 1-7, p 0.07 | [review 2, *Keep It Alive*](../../Reports/mould-review-2-2026-11-10.md) |
| Clumsiest water hand (`HJIT=10 HLAG=3`) vs oat-saver | longer | 8 | 8-0, p 0.008 | same |
| Two humps, all water-hand runs | 17 dead by 5:05, 14 past 7:20, 1 between | 32 | | same |
| First mostly-asleep pile has a sleeper within 45 cells: long / short | 13 / 2 (else 7 / 26) | 16 x 3 hands | Fisher p 0.00003, cut named after looking | [keep-alive luck, *Answer*](../../Reports/mould-keep-alive-luck-2026-11-10.md) |

Review 2 made about 14 comparisons, so its strict bar is p < 0.004; its 8-0
results are consistent directions, not proofs. The water hand is a script,
not a person; the lamp is on for 3-22 s of a 3-10 minute game.

## Owner's word

- 2026-11-10 02:57Z, card: **the keep-it-alive endless dish first** (review thread).
- 2026-11-10 03:06Z: "Agreed" to the order endless dish, then water and
  sleep, then a wild mould, with the bar that a good game uses three or more
  tools and a random hand still loses to a plan.

## Tried and rejected

- Water 3 drops regrowing every 20 s with sleepers living 40 s: random
  coasted to the ten-minute cap on 4 of 16. Shipped 2 drops / 40 s / 20 s
  ([keep-alive, *Tuning that was tried*](../../Reports/mould-keep-alive-2026-11-10.md)).
- Scoring the time a sleeping remnant lasted: two sleepers ran a dish to the
  cap. Hence `ENDLESS_MIN` and `awake_at`.
- Sensor reach 6 for finer tubes: the water hand lived shorter on 6 of 8
  (median 6:21 to 3:41); see [`dead-ends.md`](../../Reports/dead-ends.md),
  `sensor_offset`.

## Open

- **Give the player the deciding choice.** Traced (above): the hump turns on
  whether the mould sleeps near the next landing, which nobody can see in
  time. Next test, named in the [luck report](../../Reports/mould-keep-alive-luck-2026-11-10.md):
  show the next landing as the one before lands (20 s ahead, not 5), a hand
  that sleeps the mould toward it, lives against today's `sleeper` on fresh
  seeds 17-32. Fallback: sleepers that last longer than 20 s.
- Confirm the trace's rule on fresh seeds; 7 long games and 2 short ones it
  does not explain.
- Star bars: re-check on 16+ seeds once a game has changed. The 5:30 bar
  separates nothing today.
- Eaten piles stay as dull discs, about 20 by minute six.

## Sources

- [mould-keep-alive-2026-11-10.md](../../Reports/mould-keep-alive-2026-11-10.md): the dish, numbers, tuning.
- [mould-review-2-2026-11-10.md](../../Reports/mould-review-2-2026-11-10.md), *Keep It Alive*: reproduction, clumsy hands, the two humps.
- [mould-review-thread-handoff-2026-11-10.md](../../Reports/mould-review-thread-handoff-2026-11-10.md): owner's words with times.
- [mould-keep-alive-luck-2026-11-10.md](../../Reports/mould-keep-alive-luck-2026-11-10.md): 48 traced games, what decides the hump.
- [mould-wild-one-2026-11-10.md](../../Reports/mould-wild-one-2026-11-10.md), *Finer tubes*.
