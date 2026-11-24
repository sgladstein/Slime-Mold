# Balance review, 2026-11-24: today's build with the origin

Scott's OK, 2026-11-24 03:15Z ("3/4) approved"): a fresh balance review
on today's build, whose fixes go to him before any tuning changes. **This is a
review only. No tuning was changed.** It revisits the
[11-20 review](../balance-review-2026-11-20/README.md) and answers
Direction's three questions (11-24 03:20Z): planet time with the origin on,
the food he can't spend, and what makes planet 5 long.

## Data

- **His play logs** since 11-22 (`play-logs` branch, 22 planets on builds
  3072 to 3198; 3198 is main at 406c2c9, before the origin).
  [`curves.py`](curves.py) prints the slime's peak, the time at half or more,
  the fall from half to 5%, the army, the losses, and the food at half and at
  the end ([output](curves-output.txt)).
- **An Arms batch on main with the origin** (PR #743, Arms run 45318989043,
  on #699's head, which is what landed as fd3fe28). There were three plans on
  the same maps, whole runs through planets 1 to 4, each planet capped at
  12 game minutes:
  - *as-is*, the run bot's plan (`bot`, then the rush with no cards);
  - *cards*, the same rush buying Wide, Rate and Needle Blast;
  - *econ*, the league's best planet 3 plan, which sends 4 to 8 soldiers to
    the heart with the same cards.

  Planet 5 played as is. Rows are in [`arms-rows.tsv`](arms-rows.tsv)
  (planet 1 is the same game in every arm, since all three play `bot` there).
  The 35-minute shard budget skipped three games: econ on seed 1, cards on
  1795119328, and planet 5 on seed 3.
- **The run bot's nightly run** on 406c2c9 (run 45233517666, seeds 1 to 3,
  planets 1 to 3).
- **The shop simulator** from 11-20 rerun on today's offers
  ([output](shopsim-output.txt)).
- **The strategy league's PC rows** on cb8d00f (11-22, before #736), for
  planets 2 and 3.

## Ranked findings

### 1. For him the fight is one-sided on planets 1 to 4

| Planet | His planets (builds 3072 to 3182) | Length | Soldiers lost | Peak army |
|---|---|---|---|---|
| 1 | 3 won | 3:51 to 6:23 | 0, 0, 3 | 14 to 52 |
| 2 | 3 won | 4:31 to 4:40 | 0, 0, 2 | 10 to 27 |
| 3 | 3 won | 4:56 to 8:19 | 28, 1, 0 | 27 to 35 |
| 4 | 2 won, 3 quit | 4:25, 6:38 | 1, 1 | 24 to 48 |

He wins with small squads too (10 to 16 soldiers on planets 1 and 2), so the
cause is not only the army snowball from the 11-20 review. **The slime
can't reach a squad he steers.** The snowball's shape is still there: the
slime holds at half its peak or more for most of a planet, then falls from
half to 5% in 0.5 to 2.2 minutes. He loses soldiers late, once the slime is
under about a fifth of its peak.

#736 ("sooner and tougher from planet 2": first raid at 2:00, armoured
marchers, a harder bite, mini-bosses that reach the squad) landed after his
last planets 1 to 4 (build 3182). He hasn't played planets 2 to 4 since. The
bot says #736 bites (finding 4).

**Proposal:** no change now. Read his next planets 2 to 4 on #736 plus the
origin first. The measure to read is soldiers lost and peak army a planet.
If he still loses 0 to 3 a planet, take the next step from #721's and
#736's levers on planets 2 to 4, not 1:

- `RAID_ARMOUR` one step up a planet;
- `STRIKER_ARMY` reaching its 2.5x at 40 soldiers, not 60.

Both make the slime that reaches the squad tougher, and neither grows the
slime with the army (his 11-20 ruling).

### 2. With the origin, a small squad that goes straight for the heart wins fastest

| Plan (seed 2) | Planet 2 | Planet 3 | Planet 4 |
|---|---|---|---|
| econ: 8 to 15 soldiers to the heart, Wide + Rate + Tinder, then Needle Blast | **won 1:10**, 8 soldiers, none lost | **won 2:32**, none lost | **won 2:17**, 15 soldiers, 14 lost |
| as-is rush | lost 4:14 (30 soldiers wiped) | — | — |

The same plan lost planet 2 on seed 5 (4:53, 10 soldiers) and held to the
12:00 cap on seed 4 (3 soldiers left), so the snipe is a gamble. The Enemy
lane's own figure is that 44 soldiers win planet 1 in 1:34 with none lost.
The origin shortens a planet for anyone who goes for it, and that runs
against Direction's two-hour run.

**This is D7, the rush brake, which is Scott's.** NOW.md says "add no rush
brake unasked", so this is a question for him, not a fix. Options:

- **(a)** Leave it: sniping the heart is a skill play, and it fails often.
- **(b)** A tougher heart deeper in the run. `run::ORIGIN_SIZE` is already
  per planet (700, 500, 1,000, then 2,400 to 4,800); its armour could rise
  with depth too (Enemy's file).
- **(c)** Shield the heart while its crystal is full, so draining comes
  first (his 11-21 crystal ruling makes draining the squad's job).

**Recommended: (b).** It keeps the snipe as a play and makes it cost
something.

### 3. Fire burns whole armies on planet 3 when no fire card was bought

In 3 of 7 bot games on planet 3, fire killed most of the squad:

| Game | Gun | Soldiers burned | Result |
|---|---|---|---|
| as-is, seed 5 | rifle, no cards | 54 of 56, from 9:00 to 10:00 | lost |
| cards, seed 4 | Wide + Rate + Needle Blast | 31 of 31, from 2:30 to 3:00 | lost |
| nightly run bot, seed 2 | rifle, no cards | 34 | won |

The engine hurts a soldier standing in any fire on plants (`unit_heat` in
`engine.wgsl` reads every burning cell). Craters also light plants
(`fire_light_blast`): Needle Blast, thrown blasts and starfall impacts. The
Trunk's ground carries fire well (`fire_fed`). `run::ITEMS` (Fireproof's
comment) and `Rules::fire_hurt` describe fire as hurting "only where the
squad's own fire cards lit it". The engine doesn't do that.

**Proposal (a fix, not tuning):** replay as-is seed 5, planet 3, and find what
lit the fire. Then decide one of two things:

- fire the squad didn't light doesn't hurt (what the comments say);
- the Trunk's fire is a planet 3 hazard on purpose, and then it needs a
  look and a warning, by his rule that every threat is something you can
  see.

Engine and Enemy own the two files.

### 4. Since #736, planet 2 is where the bot's rush dies

The as-is rush lost planet 2 on 2 of 5 maps: seed 2 at 4:14 and seed 4 at
6:16, with 30 and 33 soldiers, every one bitten. On the 11-22 build (before
#736) the same plan won 97% of 7,549 league games on planet 2, in a median
3:55. The cards plan lost 1 of 4 (seed 2, where Tinder burned 9).

**Proposal:** no change before he plays it. #736 was his ask ("still too
easy"). If his next planet 2 is lost or close, the first levers are #736's own
planet 2 values: `RAID_ARMOUR[1]` 2 to 1 and `DRAIN[1]` 2.5 to 2.

### 5. The food he can't spend is earned after the fight is won

This is Direction's second question. He ended planet 5 with 16.5k to 46k food,
planet 6 with 110k and planet 8 with 34k. Most of it came in after the slime
fell under half its peak:

| Planet | Food at the slime's half | Food at the end | Income after half |
|---|---|---|---|
| 6 (11-22 08:05) | 2.0k | 109.8k | 120k of 163k |
| 5 (11-23 03:15) | 2.3k | 41.3k | 52k of 85k |
| 5 (11-22 23:20) | 2.7k | 42.5k | 43k of 91k |
| 8 (11-22 08:27) | 14.7k | 34.2k | 40k of 71k |

The bank is low while the fight is open, so this is neither a missing sink
nor too much income in the contested part of a planet. It is pump income
during the mop-up, with nothing left to buy for, and it's lost at the win.
The exception is planet 5 on two of his four plays, where he had 27k to 32k
banked at half. On the biggest planets income outruns a soldier's climbing
price (950 food at 85 alive).

**Proposal:** no change. The origin ends a planet soon after the fight
turns, which should remove most of the mop-up. Read his next planets 5 to 8.
Paying leftover food into shop points is against his 11-20 ruling (13:49Z:
"I just wall it in ... rack up money").

### 6. Planet 5 is long because of its size, and the replays were not losses

This is Direction's third question. He played planet 5 of run 1795380658
five times:

- one quit at 3:24 on build 3182, with 19 soldiers;
- three wins on builds 3192, 3194 and 3198, in 12:52, 11:27 and 10:34.

The wins were the saved run replayed on new builds, not losses. His other
planet 5 took 14:00. The one thing that sets planet 5 apart is the slime's
size: it peaks at 133k to 164k against 44k to 62k on planet 4 (2.2 to 3.0
times). His army is 60 to 123 there. With the origin on, the bot won planet 5
in 4:50 and 11:28 (seeds 2 and 1), with 5 and 9 lost.

**Proposal:** don't shrink the Sporer. For a two-hour run, planet 5's 10 to
14 minutes is nearer the target than planets 1 to 4's 4 to 6. Smoothing the
step from planet 4 belongs with Direction's stretch dial, not a cut to
planet 5.

### 7. Planet times with the origin on

This is Direction's first question. The bot's wins with the origin:

| Planet | as-is rush | cards rush | econ heart snipe | His, before the origin |
|---|---|---|---|---|
| 1 | 4:56 to 6:54 (5 maps) | same game | same game | 3:51 to 6:23 |
| 2 | 5:38 to 6:23 | 3:14 to 5:13 | 1:10 | 4:31 to 4:40 |
| 3 | 4:13 to 9:41 | 3:21 to 8:02 | 2:32 | 4:56 to 8:19 |
| 4 | 9:00 | 4:40 to 7:12 | 2:17 | 4:25 to 6:38 |
| 5 | 4:50 to 11:28 | — | — | 10:34 to 14:00 |

On 3 of 8 planet 1 maps the `bot` plan held to the 12:00 cap. It shoots
96k to 122k walkers off a 12k to 17k slime that regrows on its crystals, and
never goes for the heart. That's the bot (Engine's siege work, #707), not the
planet. But it also shows that a player who never strikes the heart can sit on
a planet forever.

### 8. Cards help, in combination, and nothing is dead alone

On the 5 planets where both rushes played the same map and the two results
differed, the cards rush did better on all 5, quicker or winning where the
plain rush lost, and with fewer soldiers lost:

- planet 2, seed 1: 5:13 against 5:38, with 0 lost against 5;
- planet 3, seed 1: 3:21 against 4:13, with 9 lost against 23;
- planet 4, seed 1: 7:12 against 9:00, with 31 lost against 38;
- planet 2, seed 4: won against lost;
- planet 3, seed 5: won against lost.

Both lost planet 2 on seed 2. The sign test on 5 of 5 gives p = 0.06
(two-sided), so this is not significant on 5 games.

- The dead-alone cards from the 11-20 review were fixed in Squad's batch 7:
  - leftover modifiers get a plain cast;
  - carriers are held back until a bullet card is owned;
  - owned cards are offered again.

  The simulator's "needs a bullet after it" line (50.8%) still counts the old
  list, so read it as the share of shops that offer one of those cards, not
  dead ones.
- The league's best deck (Wide + Rate + Needle Blast) is offered by planet 3
  in 0.62% of 4,000 runs (0.47% on 11-20).
- His Noita rule holds: the deck's own blasts killed 18 and 19 of the squad
  on planets 3 and 4 (`blasted_end`).

**Proposal:** no change.

### 9. Resolved since 11-20

- **The time-out win.** The clock counts up (his 11-20 14:10Z ruling). A
  planet ends only at the far stalemate limit, and "held on" is counted
  apart. No bot planet reached the limit.
- **Planet 2 unmeasured.** It is now measured, finding 4.
- **Food lost at the win.** Starfall and boss cores pay the shop (#573);
  leftover food is finding 5.

## What would change a finding

- Finding 1 depends on his next planets 2 to 4 on #736 and the origin.
- Finding 2 depends on his D7 answer.
- Finding 3's fix depends on what lit the fire.
- Findings 4 to 6 are bot or log counts on few maps. The batch had 8 run
  maps, with 2 to 5 reaching each planet past 1. Read them as directions,
  not rates.
