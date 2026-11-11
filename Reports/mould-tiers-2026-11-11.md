# New mould tiers and how each answers attack (2026-11-11)

Research with small tests on the CPU big dish, no game change: every rule
is a `BigParams` field, off by default, and the default hash is unchanged
(869141a7bf398adf, `preset=dense seconds=2 blob=0 piles=1`). The brief came
from the creature-rules thread, approved by the owner: find the next rungs
of the Larder World ladder (a Sporer that jumps between crystals, a Sleeper
that lies dormant, a mould that splits or migrates) from what published
agent-Physarum work suggests, each with a body and a weakness the player
can use. The owner, 15:07Z, in this thread: "You should also consider how
the mold reacts to the player, how it reacts to being attacked, etc."
Page: https://claude.ai/artifact/EGJKqpbfE5Gh8nDToXHfMu (source and media in
[`mould-tiers-2026-11-11/`](mould-tiers-2026-11-11/)). Wiki:
[`wiki/model/life-cycle.md`](../wiki/model/life-cycle.md).

## Summary

- **A life cycle for every walker.** Each walker is ordinary, a spore in
  flight, dormant, or dead, with a timer (`BigDish::life`, `life_t`). A
  new field, the **alarm**, is laid by the player's strike
  (`BigDish::strike`: kill every walker in a disc, clear its trail, lay
  alarm out to twice its radius) and relayed by waking walkers; sensors
  add `alarm_weight` x alarm. Three tiers come out of it, each with a
  counter the player can learn, all four tests at **8 of 8 paired seeds,
  p 0.0078, surviving a Bonferroni correction over the five tests**:
  - **Sporer** (`spore_rate` 1e-4, fruiting only on food): holds 6-7 of 7
    piles at 20 s against 3-5. **Struck**, it sprays spores
    (`spore_burst` 0.05) and holds 7 of 7 on every seed against 3-5; the
    wound empties (66-736 walkers left at it against 1171-2361, not a named
    measure). Counter: starve it, don't hit it.
  - **Sleeper** (`sleep_rate` 0.01 off food): a dormant crust that a
    strike wakes in a wave and that rushes the wound, then is deaf to
    alarm for 5 s. Counter: **bait** it. A harmless shock at the far edge
    4 s before the strike leaves 1858-2216 walkers at the strike 2 s
    after it, against 2360-2971.
  - **Swarmer** (`alarm_weight` +3): feeds itself into the fire. 11
    strikes in 5 s kill 1144-4158 walkers against 614-3591. Counter: a
    kill zone.
- **Skittish** (`alarm_weight` -10, Jones and Adamatzky's light as a push)
  did not break the body up: 1-5 tube bodies against 1-3, 4 higher, 1
  lower, 3 ties, p 0.375. Dead end; the splitting or migrating tier is
  still open.
- **Pace.** At `spore_rate` 1e-4 the Sporer is too fast for the owner's
  rule (14:59Z: "The mold shouldn't be able to take over a whole map in
  10 seconds"): it holds 7 of 7 piles by 10 s on 6 of 8 seeds (by 5 s
  on 5), against 3-5 without spores.
  `spore_rate` is the dial (`pace.sh`, seeds 1-3): at 1e-5 it holds 4-6
  piles at 10 s (7 on none of 3) and 6-7 by 60 s; at 3e-6, 3-6 from 10 s
  to 60 s, barely above the 3-5 of the plain body.
- **Cost**, one core, 100,000 walkers: the alarm field is one more
  full-grid pass, 33.0-34.6 ms a step against 25.2-25.9 (+31%); spores
  27.7-29.2 (+6-16%); a dish that sleeps is cheaper, since dormant walkers
  don't move.
- **Robustness** (`robust.sh`, `wall.sh`, seeds 1-3, descriptive): over
  2 minutes the Sporer keeps all 7 piles and the Sleeper stays a crust
  (15,498-18,782 of 20,000 dormant at 120 s); at 5,000 walkers a Sporer
  whose body reaches no pile never feeds, so it never fruits; at 80,000 the
  plain body already holds 5-7 piles. No tier rides the wall: at most
  1.8% of walkers within 10 dish cells of it, where walkers spread evenly
  put 18.8%.

## Method

The body: 20,000 walkers in a centre disc of radius 40 dish cells on the
1024 x 640 fine grid (256 x 160 dish cells, scale 4), preset dense (one
walker a cell), 7 food piles, one core, 20 s (2,400 steps). It grows a
meshy body with arms to the nearest piles, holding 3-5 of 7 at 20 s on its
own. The strike's defaults: radius 8 dish cells, alarm 50 out to 16.
`Reports/mould-tiers-2026-11-11/run.sh` holds every arm and names each
deciding measure before the run; `pairs.py` reads the raw output and gives
the paired counts and sign tests. One deciding measure a test, seeds 1-8,
Bonferroni over 5 (a test survives at p 0.01; the best 8 seeds can give is
0.0078). Looks came first: seed 1 of every arm rendered in `style=life`
(amber ordinary walkers, violet dormant, white spores, red alarm, grey
trail; all data, no outline), on the page.

Published work the rules come from (cards in `research/sources/`):
Jones and Adamatzky 2012 (the amoeboid model: blocked walkers keep their
heading; light read as sensed values times less than 1 pushes and splits
a blob; fragments fuse when they sense each other) and the Jones 2015
survey (the sclerotium: drying mould shrinks to a disc and revives).
Physarum fruits under starvation and light; here a spore fruits where it
feeds, so a Sporer spreads from its crystals, because a mould that spores
only when starving spores only when it is already losing.

## Sporer

`spore_rate` 1e-4, `spore_scent` 1 (fruit only where scent is over 1, on
food), `spore_steps` 240, `spore_speed` 3, `spore_reach` 36, `spore_land` 1,
`spore_deposit` 0.2. A fruiting walker leaves its cell and flies; the first
half of its flight is straight and it cannot land, so it leaves its own
crystal; in the second half it steers by scent with three sensors 36 fine
cells out and lands on the first free cell with scent over 1; if its timer
runs out it drifts straight to a free cell. A landed spore is an ordinary
walker that cannot fruit again for `spore_rest` 1200 steps (10 s). Spores
are a faint white streak on the picture (they lay 0.2 x deposit).

**Deciding measure: piles held at 20 s** (piles with 20 or more grounded
walkers within `pile_r` + 4 dish cells). Sporer 6-7 (median 7) against
3-5 (median 4): **higher on 8 of 8, p 0.0078, survives.** At any moment
only 6-40 walkers are in the air (seed 1); the outposts grow from the
crystal they land on.

**Hitting a Sporer.** `spore_burst` 0.05 with `spore_alarm` 1 adds a 5%
chance a step to fruit where the alarm is over 1, on or off food. One
strike at the body's left edge at 10 s (`alarm_keep` 0.98). **Deciding
measure: piles held at 20 s.** 7 on all 8 seeds against 3-5: **higher on
8 of 8, p 0.0078, survives.** 1963-3804 spores are in the air a second
after the strike; the wound empties (66-736 grounded walkers within 16
dish cells of it at 20 s against 1171-2361, fewer on 8 of 8, a measure
picked after the run). The strike killed 678-2170.

## Sleeper

`sleep_rate` 0.01, `wake_scent` 1, `wake_alarm` 1, `sleep_deposit` 0.3,
`alarm_keep` 0.98, `alarm_relay` 20, `alarm_weight` 3, `wake_steps` 240,
`sleep_rest` 600. Walkers off food fall dormant at 1% a step: by 10 s
99% of the body is a violet crust (19,813 of 20,000 on seed 1) holding
only the piles it sits on (2 of 7). A dormant walker lays 0.3 x deposit,
so the crust keeps its own trail; scent or alarm over 1 wakes it, and a
waking walker adds 20 to the alarm on its cell, once: the wake runs
through the crust as a wave (an excitable medium), across the whole body in about 2 s (seed 1). The
woken mould swarms to the alarm (`alarm_weight` 3). After 2 s awake it can
sleep again, and a walker that falls asleep is deaf to alarm for 5 s
(scent still wakes it).

The window is the weakness. **Bait**: a shock with no kill (radius 0,
alarm out to 20 dish cells) at the left edge at 10 s; the real strike at
the right edge at 14 s. On seed 1 the bait wakes 19,442 walkers by 12 s,
they fall back asleep by 14 s, and the strike lands on a deaf crust.
**Deciding measure: grounded walkers within 16 dish cells of the strike
at 16 s.** With the bait 1858-2216 (median 2030) against 2360-2971
(median 2736): **fewer on 8 of 8, p 0.0078, survives.**

## Swarmer

`alarm_weight` 3 on an ordinary body, `alarm_keep` 0.98, 11 strikes every
0.5 s from 10 to 15 s at the body's left edge. **Deciding measure: walkers
killed.** 1144-4158 (median 1393) against 614-3591 (median 816): **more on
8 of 8, p 0.0078, survives.** On seed 1 the first strike kills 1113 either
way; after it, the Swarmer keeps walking into the fire (1896 dead at 20 s
against 1199).

## Skittish

`alarm_weight` -10 under the same fire, after Jones and Adamatzky's light,
which splits their blob. **Deciding measure: tube bodies of 4+ dish cells
at 20 s.** 1-5 (median 2) against 1-3 (median 1.5): 4 higher, 1 lower, 3
ties, **p 0.375, fails.** On seed 1 the body's centre moved about 0.2
dish cells and the dead were the same (1201 against 1199). The alarm
spreads a few fine cells a second, so the push reaches only the walkers by
the wound; the fed arms hold the rest to their piles. A
[`dead-ends.md`](dead-ends.md) entry records when to try again.

## Cost

`cost.sh`: preset dense (100,000 walkers spread over the dish, one a
cell), one core, 10 s, two interleaved rounds, ms a step averaged over
the run (raw `raw/cost.txt`):

| Arm | ms a step |
|---|---|
| off | 25.2-25.9 |
| life states allocated, nothing on | 25.6-26.7 |
| alarm field (+3 weight, one strike) | 33.0-34.6 |
| spores | 27.7-29.2 |
| sleep (with the alarm field) | 16.5-16.8 |
| all of it | 16.2-17.4 |

The alarm's cost is its 3x3 mean over every cell each step, the same as
one more trail pass; it could skip while the field is empty. Sleep is
cheaper because most walkers end up dormant and dormant walkers neither
sense nor move.

## Robustness

Seeds 1-3, descriptive, measures named in the scripts before the runs.

**Two minutes with food** (`robust.sh`, R1 and R2). The Sporer holds 7 of
7 piles at 60 and 120 s on all three seeds, against 4-5 for the plain body
at 120 s; 104-164 spores are in the air at 120 s (more walkers sit on
food by then) and its outposts make 2-7 tube bodies against 1-3. Nothing
dies (the model has no births or deaths yet). The Sleeper stays a crust:
15,465-18,771 walkers dormant at 60 s and 15,498-18,782 at 120 s, after
the strike at 14 s has woken it and it has gone back down; it holds 2-3
piles against 3-5 for the plain body under the same strike. Neither
drifts between 60 and 120 s.

**Other sizes** (R3, 20 s, discs about a quarter full). At 5,000 walkers
(radius 20) the Sporer holds 0-6 piles against 0-2; on seed 3 the body
reaches no pile, never feeds and never fruits, so a small Sporer must
reach a crystal first. At 80,000 (radius 70) it holds 7 of 7 at 10 s on
all three against 5-7: the plain body nearly covers the piles already.

**Pace** (`pace.sh`). Piles held, `spore_rate` 1e-5 / 3e-6 / none:

| Seed | 10 s | 20 s | 60 s |
|---|---|---|---|
| 1 | 4 / 3 / 3 | 4 / 3 / 3 | 6 / 3 / 3 |
| 2 | 6 / 6 / 4 | 7 / 6 / 4 | 7 / 6 / 4 |
| 3 | 6 / 5 / 5 | 6 / 5 / 5 | 7 / 5 / 5 |

The outposts are landed spores, not roads: with no streak
(`spore_deposit` 0, `spore_rate` 1e-4) the Sporer still holds 7 of 7 at
20 s on seeds 1-3 (raw `x-nostreak-*`).

**The wall** (`wall.sh`, R4, 60 s). The creature review warned that the
dish wall can be a rail a rule leans on. The wall reads as a wall to
sensors 9 dish cells out, so walkers spread to the edge leave a 2-cell
band in 0.2 s (1.8% of them at 0.1 s, 0.7% at 0.2 s); `rim` counts the
share within 2 and within 10 dish cells. Spread evenly, 18.8% sit within
10. At 20 and 60 s every arm reads 0.000-0.001 within 2 cells and, within
10: plain 0.000-0.008, Sporer 0.001-0.018, struck Sporer 0.000-0.011,
Sleeper 0.000, Swarmer 0.000-0.007, Skittish 0.000-0.008. The struck
Sporer's spores reach the wall (on the page) but land on food, away from
it. No tier rides the wall.

## Tried and changed on the way

- Spores that home on scent from launch landed back on their own pile, and
  spores circled a full pile for their whole flight: replaced by the
  two-half flight and drifting straight at the end
  ([`dead-ends.md`](dead-ends.md)).
- Fruiting was first gated on the body's own trail; it is now gated on
  food scent (`spore_scent`), so a mould fruits only where it feeds.
- A wake relay of 5 with alarm keep 0.95 died out in the crust; 20 with
  0.98 runs through the whole body in about 2 s.
- The alarm laid only where the strike kills barely spread; it now reaches
  twice the kill radius.
- A 100,000-walker body of radius 20 jammed (only 20,375 placed, no
  spores); the tests use 20,000 at radius 40.

## For the other threads

- **GPU** (owns births and upkeep): every field's doc comment in
  `BigParams` is written to port to WGSL: `spore_*`, `sleep_*`, `wake_*`,
  `alarm_*`, plus `BigDish::strike`. The keyed draws are `DRAW_FRUIT`,
  `DRAW_SLEEP`, `DRAW_SPORE_H` and `CLAIM_LAND`.
- **Larder** (the ladder): Sporer, Sleeper and Swarmer, each with its
  counter; Skittish is weak; spores cost nothing today and should
  probably cost food in the world.

## Files

`mould-tiers-2026-11-11/`: `run.sh` (the five tests), `pairs.py`,
`cost.sh`, `robust.sh`, `pace.sh`, `media.sh` (seed-1 videos and stills),
`page.html` and `pics/`, `raw/` (every run's output).
