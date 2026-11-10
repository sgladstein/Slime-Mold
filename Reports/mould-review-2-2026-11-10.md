# Independent review 2: Keep It Alive, water and sleep, The Wild One (2026-11-10)

**Status: review only, no build. Findings sent to the mould thread.**
Page: https://claude.ai/artifact/DnqCHgesBQYWDJBHjE7PHr (source in
[`mould-review-2-2026-11-10/page/`](mould-review-2-2026-11-10/page/index.html)).
Every result line: [`mould-review-2-2026-11-10/runs.txt`](mould-review-2-2026-11-10/runs.txt).

Asked for by the owner (2026-11-10 04:08Z, "After you finish all your work,
have the fable thread review it"). Reviewed at Pixel_Physics main `b3bd36ac1`,
the last commit before the game moved here. Follows
[mould-review-2026-11-10.md](mould-review-2026-11-10.md); reviews
[mould-keep-alive-2026-11-10.md](mould-keep-alive-2026-11-10.md) and
[mould-wild-one-2026-11-10.md](mould-wild-one-2026-11-10.md).

## Verdict

Both dishes work as built, and every number in their reports reproduces. Neither
is yet a game a person can get better at. Keep It Alive is mostly luck once
water is used at all. The Wild One is one opening move, then two minutes of
holding the lamp at an aim a person does not have.

## Method

Not played by hand. The thread's scripted hands (`sleeper`, `warden` in
`examples/mouldprobe.rs`) were re-run on 16 seeds, then made worse on purpose
on 8 seeds each, with a scratch copy of the probe
([`mould-review-2026-11-10/mouldrev.patch`](mould-review-2026-11-10/mouldrev.patch);
the committed patch lacks the later `INVARSHOW` line, which only prints the
check's values):

- `HJIT=n`: lamp, water, snip and oat placements land anywhere in a disc of
  n cells round the intended point.
- `HLAG=s`: the hand looks at the dish only every s seconds.
- `STOPAT=s`: the warden lets go after s seconds.
- `NOWATER=1`: the sleeper bakes but never waters.
- One-move openings on the wild nest: `camp` (lamp held on its middle all
  game), `nestsnip` (three snips through it at 0.5 s), `nestsalt` (the salt
  line through it), `jail` (a salt ring of radius 9.5 round it), `strike`
  (salt, snips and the camped lamp together).
- `INVAR=1`: every 15 steps, count cells holding both moulds, wild walkers
  off agar, and the difference between the two dishes' pile food.

About 270 runs. Paired by seed; p values are two-sided sign tests. About 14
comparisons were made, so the strict bar is 0.004 and an 8-0 (p 0.008) does
not pass it: these are consistent directions, not proofs.

## Keep It Alive (dish 4), 16 seeds re-run, then 8

| Hand | Lived (seeds 1-8) | Stars |
|---|---|---|
| doing nothing | 2:13 2:27 2:53 3:48 2:06 1:08 1:47 1:47 | 0 on 8 |
| oat-saver (`intended`) | 3:09 4:26 4:09 3:28 3:48 4:08 5:26 3:29 | 1 on 2 |
| sleeper | 8:01 9:21 4:11 8:03 4:28 4:41 10:42 4:41 | 3,3,0,3,1,1,3,1 |
| sleeper, `NOWATER=1` | 2:57 4:19 4:00 3:58 3:39 3:59 5:16 3:22 | 1 on 2 |
| sleeper, `HJIT=6 HLAG=2` | 5:31 4:24 4:06 9:25 4:27 4:08 8:45 4:09 | 2,1,0,3,1,0,3,0 |
| sleeper, `HJIT=10 HLAG=3` | 4:08 11:30 11:23 9:26 8:23 4:09 11:38 8:15 | 0,3,3,3,3,0,3,3 |

- All 16-seed figures in the dish's report reproduce exactly.
- **Water is the mechanism.** Bake-without-water against the oat-saver: 1-7
  (p 0.07), a few seconds shorter. The gain is the waking.
- **Not flattered by precision.** The clumsiest hand beats the oat-saver 8-0
  (p 0.008) and the precise sleeper 6-2 (p 0.29).
- **Two humps.** Over the 32 water-hand runs (16 precise, 8 and 8 clumsy):
  17 died by 5:05, 14 lived past 7:20, 1 fell between (5:31). The 5:30 star
  was earned once in 32.
- **Which hump is not skill, on this evidence.** Seed 3: 4:11 precise, 4:06
  and 11:23 clumsy. Seed 1: 8:01, 5:31, 4:08.
- The lamp is on for 3-22 s of a 3-10 minute game.
- Eaten piles stay on the dish as dull discs, about 20 by minute six.

Not measured: what separates the humps. The guess to test with a per-run
trace is whether sleepers lie within reach of where a far pile lands at about
the twelfth pile, when the gap passes 140 cells.

## The Wild One (dish 5), 16 seeds re-run, then 8

Stars per seed 1-8, and walkers you / wild at the bell:

| Hand | Stars | You / wild |
|---|---|---|
| doing nothing | 0 on 8 | 0.65-1.36 |
| random | 0 on 7, 1 on 1 | 0.70-1.76 |
| warden as written | 2,2,2,3,2,1,3,3 | 2.02 to cleared |
| warden, `STOPAT=30` | 1 on 8 | 1.66-1.95 |
| warden, `STOPAT=60` | 1 on 8 | 1.70-1.94 |
| warden, `STOPAT=90` | 1 on 8 | 1.52-1.98 |
| warden, `HJIT=2` | 1,2,2,2,2,2,1,1 | 1.67-4.13 |
| warden, `HJIT=3` | 1,2,1,1,1,1,1,1 | 1.91-3.54 |
| warden, `HJIT=6` | 1 on 8 | 1.78-2.83 |
| warden, `HLAG=2` | 2,2,1,1,3,1,2,1 | 1.90 to cleared |
| warden, `HJIT=6 HLAG=2` | 1 on 8 | 1.77-2.60 |
| warden, `HJIT=10 HLAG=3` | 1 on 8 | 1.73-2.31 |
| `jail` | 0,1,0,1,1,1,1,0 | 1.29-2.90 |
| `camp` | 0 on 8 | 0.75-1.36 |
| `nestsnip` | 0 on 8 | 0.77-1.22 |
| `nestsalt` | 0 on 7, 1 on 1 | 0.56-1.71 |
| `strike` | 0 on 6, 1 on 2 | 0.48-2.22 |

- All 16-seed figures in the dish's report reproduce.
- **The first star is the opening.** Stopping at 30 s keeps it on 8 of 8;
  the dish's own report has the salt wall alone at one star on 8 of 8.
- **The second and third stars are an aiming test.** Exact aim: 2-3 stars on
  7 of 8. Two cells off: 2 stars on 5, none cleared. Three cells off: 1 star
  on 7. Slow looking alone (2 s) costs less than aim does. `HJIT` moves the
  lamp, snips and oats together, so which of them needs the precision is not
  separated here.
- Playing on clumsily (`HJIT=6 HLAG=2`) beats stopping at 30 s on lead by
  7-0 with a tie (p 0.016): median 1.93x against 1.81x. No star moves.
- The warden holds the lamp 102-150 s of 150.
- **No cheap knockout**: `camp` 5-3 against doing nothing, `nestsnip` 7-1
  (p 0.07), `nestsalt` 6-2, `strike` 5-3. `jail` is 8-0 (p 0.008) and takes a
  star on 5 of 8: a second one-move opening worth a star.
- **The snip delivers nothing**: three cuts through a nest packed at about
  one walker a cell left 0 stars on 8 of 8.

**The bell rings mid-crash.** Four of the seven piles run dry between about
2:10 and 2:20. Warden, you / wild at 140 s and at the bell (150 s):

| Seed | 140 s | Bell | Lead |
|---|---|---|---|
| 1 | 1508 / 491 | 783 / 204 | 3.1x -> 3.8x |
| 2 | 1439 / 475 | 1031 / 304 | 3.0x -> 3.4x |
| 3 | 1759 / 230 | 804 / 226 | 7.6x -> 3.6x |
| 5 | 1613 / 364 | 885 / 150 | 4.4x -> 5.9x |
| 6 | 1280 / 734 | 776 / 385 | 1.7x -> 2.0x |

The player's mould loses 28-54% in the last ten seconds and the lead moves
by up to half. Doing nothing shows the same fall (seed 1: 874 / 1062 to
354 / 543).

## The two-mould step (`Dish::step`, `Pile::bite`, `fight_losses`, `wild_left`)

Read in full; no bug found. `INVAR` over four full games (warden and random,
seeds 1 and 2), every 15 steps: 0 shared cells, 0 wild walkers off agar, 0
pile-food difference. The check was seen to run (`INVARSHOW`, seed 3, ten
samples printed with live counts). Small things:

- Wild walkers killed for standing on salt are counted as `Kill::Snip`.
- Each mould eats from its own copy of a pile in a step, so both can take
  the last bite: a pile can give up to one step's bite more than it holds
  (the sync clamps at 0).
- `wild_left` is read before the wild mould steps: one step stale, harmless.
- The window steps the dish after `Phase::Over`, so the bar's live
  "You / wild" line drifts from the card.

Frame cost was not re-measured (the machine was running these batches).

## Recommendations, ranked

1. **The Wild One: score the share of food eaten**, not walkers at the bell
   (or ring the bell at about 2:15). Cumulative, on theme, and a crash
   cannot move it.
2. **The Wild One: make seconds 30-150 matter at a person's aim.** Until
   then the dish is an opening plus a steadiness test, and bars cannot fix
   that: 1.5x is the opening alone and 3x needs aim within about 2 cells.
3. **Keep It Alive: trace what decides the long games** (individual runs,
   not the pooled table), then hand that decision to the player, for
   example by showing the next pile's landing spot earlier and letting
   sleepers last longer so seed beds are a plan.
4. **The snip: make a cut-off piece of wild mould visibly starve, or take
   the snip off this dish.**
5. **Star bars follow the above.** Keep It Alive's 5:30 bar separates
   nothing today.

## Not reviewed

Finer tubes (sensor reach 6-8, PR #1062) were not on main at `b3bd36ac1`.
Mouse feel and sound cannot be judged from here.
