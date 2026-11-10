# The Wild One: a second mould that steals and fights (2026-11-10)

**Status: built, its own dish (the fifth, THE WILD ONE); waits on the owner's play.**
Game: the mould probe (`cargo run --release --bin mould`).
Pictures: https://claude.ai/artifact/QDgynHJg8dPRKuhZf54sf9 (seeds 1 and 4, doing
nothing against the probe's `warden` hand, at 30, 75, 120 and 145 s, with the
counts beside them).

Step 3 of the order the owner agreed after the independent review
([mould-review-2026-11-10.md](mould-review-2026-11-10.md)): (1) the endless
dish and (2) water and sleep are in
[mould-keep-alive-2026-11-10.md](mould-keep-alive-2026-11-10.md). This is (3):
a wild second mould in another colour, under the same rules, nobody steering,
that steals food from the same piles and then fights where the two meet. The
owner's bar: the player's hand must change which mould ends bigger, paired
over seeds; quote the frame cost.

## What it is

- **The wild mould is a whole second `Dish`** (`Dish::rival`), with
  `wild = true`, no level, no storm, no tools and its own rng. Same `Params`,
  so the same walkers, trail, feeding, dividing and starving. `Dish::step` now
  wraps the old body (`step_own`): it copies the lamp, the ground and the
  water to the rival, applies this step's snips to it too, kills its walkers
  standing off agar (salt), marks each mould's cells as occupied for the
  other (`hold_foe_cells`) so they never share a cell, steps both, runs the
  fights and syncs the piles.
- **Stealing is the shared table.** A pile feeds at most `Params::table`
  walkers in total across both moulds, split by head count (`Pile::bite`,
  `Pile::foe_eaters`). The first version gave each mould its own table and
  every hand ended at about half the dish: no tool mattered, because there
  was nothing to contest. With the shared table, whoever has more walkers on
  a pile takes more of it.
- **Each mould's trail repels the other** (`Params::foe_repel`, 1.0 for
  yours, `WILD_REPEL` 0.6 for the wild one). Without fights and with repel
  0.3, the wild one took 90-99% of the dish for every hand, tools included.
- **Fights** (`Params::fight`, 0.02): each step a walker dies with chance
  `fight x (foe neighbours / 8)`, halved on fed tube and doubled off it, so a
  supplied front holds and a stray loses. Counted as `Kill::Fight` /
  `Counts::fought`. About 2,700 walkers a side die fighting in a run when
  nobody plays; 1,300-1,800 under the tool hand.
- **The dish**: seven piles, the wild one starts on the right one
  (`WILD_START` 600 walkers against your `INOCULUM` 400), oats 4 (one back
  per 15 s), salt 1, snips 3, 150 s at 22.5 degrees. Stars by your walkers
  over its walkers at the bell: 1.5x one, 3x two, none of it left three
  (`Win::Rival`). It is open from the start, like Keep It Alive.
- **Drawing**: `render::wild` lays a violet ramp over its tube with pale
  walker dots. A first pass; the looks thread was told so it can redo it.

## Numbers

Your share = your walkers / both moulds' walkers at the bell. Paired seeds,
sign tests, three comparisons in the main table so the bar is p < 0.017.
Scratch census: `mouldprobe level=5 players=idle,random,warden`.

| Hand (16 seeds) | Stars | Your share | Ended bigger |
|---|---|---|---|
| doing nothing | 0 on all 16 | 0.39-0.58 | 9 |
| random | 0 on 14, 1 on 2 | 0.41-0.64 | 12 |
| warden (lamp, salt, snip, oats) | 1 on 2, 2 on 10, 3 on 4 | 0.61-1.00 | 16 |

Warden beats doing nothing 16-0 and random 16-0 on share (p < 0.0001 each);
doing nothing against random 4-12, p 0.08, not significant.

One tool at a time (`WARDEN=lamp` etc.), 8 seeds, against doing nothing:

| Only | Stars | Share | vs doing nothing |
|---|---|---|---|
| lamp | 0-2 | 0.55-0.82 | 8-0, p 0.008 |
| salt wall | 1 on all 8 | 0.64-0.67 | 8-0, p 0.008 |
| oats | 0-1 | 0.50-0.67 | 6-2, p 0.29 |
| snip | 0 on all 8 | 0.44-0.59 | 5-3, p 0.73 |

The lamp and the wall each win alone; only all four reach 2 and 3 stars. The
snip as the warden uses it (every 12 s across the wild tube) does nothing on
its own because the cut heals: the weakest tool here.

**Frame cost** (`RIVALCOST=1 mouldprobe seeds=1,2,3`: the free dish with and
without a rival on alternating frames, 90 s, scale 4): mean 4.3-4.9 ms ->
5.7-6.3 ms, about +1.4 ms a frame; worst frames 15-22 ms either way. The
rival is a full second dish, so it costs about what yours does; it is only
on in this dish.

## Tried and changed

- Per-mould tables: share about 0.5 for every hand, tools irrelevant.
- Shared table, no fights, wild repel 1.0: the warden won 16/16 but doing
  nothing won 9/16 and random 11/16, too generous.
- Wild repel 0.3 or 0.0 without fights: the wild one took 90-99% whatever the
  hand (warden 1 win in 16).
- Fights 0.02 with wild repel 1.0 against 0.6: 0.6 keeps doing nothing at 0
  stars and the warden winning, so it shipped.
- Wild start 400 against 600 made no clear difference to who ended bigger
  before fights; 600 shipped so it starts as the bigger mould.

## Finer tubes: tried, kept at reach 9

The review suggested finer tubes by shortening each walker's sensor reach
(`Params::sensor_offset`, 9). Measured on all five dishes, 8 seeds, with
`mouldprobe level=all sensor_offset=N` and the best hands on dishes 4 and 5;
page https://claude.ai/artifact/5JT3D1sv7CwSRmYfkR6VTW.

| Planned hand, stars per seed | reach 9 | 8 | 7 | 6 |
|---|---|---|---|---|
| Behind the Wall | 33333333 | 33333333 | 13333323 | 20031231 |
| Beat the Ruler | 33333333 | 30330300 | 00000000 | 00000000 |
| Running Dry | 33333333 | 33333333 | 33333333 | 33333033 |

At reach 6, Keep It Alive's lamp+oats+water hand lived 3:04-12:02 (median
3:41) against 4:11-10:42 (median 6:21), shorter on 6 of 8 seeds; The Wild
One's all-tools share fell from 0.67-1.00 to 0.49-0.63. Doing nothing stayed
at 0 stars everywhere. The tubes are only a little thinner (they branch
more), and the mould senses food and oats from less far, so oat trails
that solve each dish stop reaching. Kept at 9; the thin look belongs in the
drawing. Entry in `Reports/dead-ends.md`.

## Review 2 fixes: scored by the food eaten (2026-11-10)

The second independent review
([mould-review-2-2026-11-10.md](mould-review-2-2026-11-10.md)) found the
walker count at the bell swinging 28-54% in the last ten seconds, as four of
the seven piles ran dry together, and ranked "score the share of food eaten"
first. Page: https://claude.ai/artifact/VBFB3yHpVxTJNePiDPQd63 (source in
[`mould-wild-one-2026-11-10/score-by-food/`](mould-wild-one-2026-11-10/score-by-food/index.html)).

- **Score**: `Dish::ate` and `Dish::wild_ate` count every bite each mould
  takes, all dish; stars are your share (`Dish::food_share`): 57%, 62%, and
  none of it left (`Win::Rival { bars: [0.57, 0.62] }`). The card reads "YOU
  ATE N% OF THE FOOD", the bar "You ate N%  wild M".
- **Small fixes from the review**: salt kills count as `Kill::Salt`
  (`Counts::salted`), not snips; a pile's last bite is credited to the wild
  mould only up to what this mould left; `wild_left` is set after the fights
  (and at `add_rival_of`); the totals stop at the bell, so the bar matches
  the card while the window keeps stepping. Guard:
  `the_wild_one_shares_the_food_but_never_a_cell` now checks that the food
  that left the piles equals what the two moulds were credited with; it
  fails with the wild count cut by 10%.
- **Probe**: the review's hands are now in `mouldprobe` (`HJIT=` aim error,
  `HLAG=` seconds between looks, `STOPAT=`, `NOWATER=1`, `INVAR=1`, and the
  openers `camp`, `nestsnip`, `nestsalt`, `jail`, `strike`); the print line
  carries `ate mine/wild share`.

Your share of the food, 8 seeds (`level=5`; runner, arms and raw output in
[`score-by-food/census/`](mould-wild-one-2026-11-10/score-by-food/census/),
whose `run.sh` paths point at a scratch folder):

| Hand | Share | Stars |
|---|---|---|
| doing nothing | 0.494-0.526 | 0 on 8 |
| random | 0.467-0.559 | 0 on 8 |
| `jail` (salt ring round the nest) | 0.489-0.517 | 0 on 8 |
| warden, `STOPAT=30` | 0.567-0.577 | 1 on 7, 0 on 1 |
| warden, `HJIT=6 HLAG=2` | 0.575-0.584 | 1 on 8 |
| warden, `HJIT=3` | 0.583-0.616 | 1 on 8 |
| warden, exact | 0.642-0.722 | 2 on 5, 3 on 3 |

Warden beats doing nothing and random 8-0 each (p 0.008); clumsy against
letting go at 30 s 7-1 (p 0.07). Each bar sits in a gap under one point
wide, so a wider sweep should check them. The snip: without it, the exact
warden 4-4 and the clumsy one 3-5 against with it, no difference.

**Tried and taken out**: three late piles on the seam (0:45, 1:20, 1:50,
the warden's oats chasing each) to give the middle of the game a race.
Doing nothing spread to 0.44-0.60 (a star on one seed), and the clumsy
hand still matched letting go at 30 s (5-3). Entry in `Reports/dead-ends.md`.

## Open next steps (handed over 2026-11-10, account move)

1. **Make seconds 30-150 matter at a person's aim.** Letting go at 0:30
   still scores about what clumsy play to the bell does; two stars need
   aim within about 2 cells. Late food made it luckier, not more played.
2. **The snip on this dish**: make a cut-off wild piece visibly starve, or
   take the snip off The Wild One (it still moves nothing).
3. **Check the star bars on 16+ seeds**: 57% and 62% each sit within a
   point of a hand on the other side.
4. **Keep It Alive (review 2, rank 3)**: trace per run what separates the
   short games from the long ones, then hand that choice to the player;
   its 5:30 star was earned once in 32.
