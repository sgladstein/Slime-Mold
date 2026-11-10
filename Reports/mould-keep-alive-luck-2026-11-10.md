# Keep It Alive: what puts a game in the short hump or the long one (2026-11-10)

**Status: measurement, no change to the game.** Review 2 found Keep It Alive's
water-hand lives fall in two humps (dead by about 5:00, or past 7:20) and that
the same seed lands in either depending on the hand. This traces single games
to find what decides it. Started while the owner's card "trace Keep It Alive's
luck now, or wait until you've played?" (2026-11-10 ~18:05Z) was unanswered,
on the option recommended there; it measures only. **The owner then picked
that option** (card, 2026-11-10 18:56Z: "Trace it now").

## Answer

**The game is decided at the first pile that lands while most of the mould is
asleep.** If a sleeper lies within about 45 cells of where that pile lands,
the hand waters it awake, it reaches the food, and the game goes long. If not,
the mould dies asleep within about one or two piles.

| First pile landing on mostly-asleep mould | Long game (lived 5:30+) | Short game |
|---|---|---|
| a sleeper within 45 cells of it | 13 | 2 |
| no sleeper within 45 cells | 7 | 26 |

48 games: 16 seeds x three water hands. Fisher's exact test, two-sided,
p = 0.00003. This is **one exploratory comparison whose cut was chosen after
looking at the traces** (see *Method*), so read it as a strong direction, not
a confirmed rule; it needs a fresh set of seeds to confirm.

This is review 2's guess (whether sleepers lie within reach where a far pile
lands), now with a count behind it. The "twelfth pile" in that guess is the
median: the deciding pile was the 8th to the 27th, mostly the 9th to the 14th.

**Why it is luck today.** Where piles land is fixed per seed
(`endless_food`, a random angle from the last pile at a growing gap), and the
game shows a landing spot only 5 s before the pile lands (the ring in
`look.rs` `piles`), while the mould is put to sleep earlier, when its food
runs low. So nobody, script or person, can choose to sleep the mould near the
next landing. Four seeds (6, 10, 11, 14) are short for all three hands, and
on each the deciding pile landed 51-123 cells from every sleeper.

## Same seed, different hands

Where hands split a seed between the humps, the measure picks the long one
on 6 of 10 seeds (1, 2, 3, 9, 13, 16) and misses on 4 (5, 8, 12, 15):

| Seed | precise / clumsy / clumsier (hump, first mostly-asleep pile) |
|---|---|
| 3 | 4:11 short far, 4:06 short far, **11:23 long NEAR** |
| 13 | 4:41 short far, 3:45 short far, **11:02 long NEAR** |
| 9 | **10:41 long NEAR**, **6:42 long NEAR**, 3:28 short far |
| 12 | 3:28 short NEAR, **6:07 long far**, 3:30 short far |

On seed 3 all three hands meet pile 12 at the same spot, 68-71 cells from the
nearest sleeper; the clumsier hand lives because it still had an awake walker
12 cells from it (the others' nearest awake walkers were 52 and 70 away), and
its first mostly-asleep pile came later, at the 14th, with a sleeper 44 cells
off. **The seven far-but-long games are not explained.** Two still had an
awake walker within 35 cells of that pile (5 and 35 cells); one lived 5:31,
a second past the cut; the other four (awake walkers 49-122 cells off, or
none) recovered by some route this trace does not show. The two near-but-short
games (precise, seeds 8 and 12) are likewise open.

## Method

- Instrument: `PILETRACE=1` in `examples/mouldprobe.rs` (added here; off
  costs nothing, it is one env check a frame). One line per pile landing:
  time, spot, gap from the pile before, walkers, how many asleep, the nearest
  awake and nearest sleeping walker to the spot, the store.
- Hands: `sleeper` (lamp, oats, water) precise, `HJIT=6 HLAG=2` and
  `HJIT=10 HLAG=3`, seeds 1-16, `level=4`. Commands in
  [`mould-keep-alive-luck-2026-11-10/jobs.txt`](mould-keep-alive-luck-2026-11-10/jobs.txt),
  raw output in `runs/`, the tables from `humps.py runs`.
- **Reproduction:** the precise hand's 16 lives, 2:51-10:42, median 4:41,
  are exactly the Keep It Alive report's; seed 3 lives 4:11 as review 2 found
  on `e242d2d`.
- **Order of looking, said plainly:** distance from the nearest *awake*
  walker to each new pile was tried first and separated little (a pile with
  an awake walker within 15 cells was survived for 60 s 84% of the time, one
  with none within 90 cells 46%). Looking at the traces showed the deaths
  follow piles that land on mostly-asleep mould, so the measure above was
  named after that look. The 45-cell cut is not tuned: it is the water hand's
  own rule for when it waters (`Sleeper` in `mouldprobe.rs`). Two cuts were
  examined in all; the result would pass a Bonferroni bar for both.
- Hump cut at 5:30 (330 s), the second star. 28 short, 20 long; review 2's
  32 runs had a different seed mix for the clumsy hands.
- Not measured: a person's hand; frame cost (the tracer is off in play).

## What it suggests for the player (not built)

The choice that decides the game is *where the mould sleeps relative to the
next landing*, and the player cannot see the landing in time to make it. The
simplest test of handing that choice over:

1. **Show the next pile's landing spot as soon as the one before lands**
   (20 s ahead instead of 5), keeping the ring and countdown as they are.
2. A scripted hand that uses it: bake (put to sleep) the mould on the side
   facing the shown spot, or lead it there with an oat first.
3. Deciding measure, named now: lives of that hand against today's `sleeper`
   on paired fresh seeds (17-32), sign test, and whether random still gets 0
   stars. A good result is the hand gaining on most seeds while random does
   not, which would make the humps skill rather than luck.

Review 2's other idea, sleepers that last longer than 20 s, would widen the
window instead of giving a choice; it is the fallback if a preview alone does
not move the long hump.
