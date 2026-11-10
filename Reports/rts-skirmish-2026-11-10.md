# The field skirmish: a playable RTS demo, and what wins it (2026-11-10)

**Status: version 1 played by the owner and set aside; version 2 (the
budget skirmish) built and played only by scripts.** The owner, 2026-11-10
20:33Z: "You can run these tests, especially if they are cheap and/or can be
paralellized. But then lets focus on actually building playable demos". On
the card at 20:42Z the owner chose "Both, in two threads": this one, and Two
Fevers in a thread of its own.

**The answer to the owner's 21:17Z question** ("What different strategies
would different players use to win. How complex or interesting are the
strategies? Does one strategy beat all the others?"): in version 2 as built,
**one strategy beats all the others**. Tithe a quarter of your food and drop
beacons, then spores, on whichever of the rival's piles has the fewest of
its walkers ("soft"). It won 127 of 128 games against the eight other
plans, every pairing surviving Holm. The other eight form a straight ladder
with no cycle among 36 pairings, so the game is shallow today: one line of
rule play wins it. Three changes meant to give the defender an answer did
not stop it. See *Version 2* below.

## Version 1: habits, salt and the lamp

You (gold) and a rival mould (violet) start equal, 400 walkers each, at the
two homes of the field: 23 small food piles mirrored left to right
(`skirmish::FIELD`, the scatter layout of the
[field retest](rts-design-2026-11-10/field/README.md)). Whoever has eaten
more of the food at the bell (2:30) wins. Each rule rested on a measurement:

| Rule | Why | Source |
|---|---|---|
| The field, not The Wild One | On 7 big piles the opening decided everything and no verb moved much; 23 small piles grow about 6,600 walkers (Jones's density) | [field retest](rts-design-2026-11-10/field/README.md) |
| **Habits**, keys 1-4: spread (the shipped 22.5 degrees), hunt (drawn to the rival's trail), maze (67.5), cord (45); 10 s between switches | On this layout hunt beat maze 32/32, maze beat cord 30/32, cord beat hunt 25/32, and each beat spread 28-31 of 32; a switch to hunt turned 10 of 12 lost games | field retest, *The pattern grid* and *Comeback test* |
| **Salt walls**, 2 in stock, one back every 30 s | A wall across the leader's road turned 6 of 12 lost games once and 9 of 12 when repeated, and beat a random wall 16 of 16 | field retest, *Comeback test* |
| **The lamp**, held | Kept from the game because it is how you touch the dish; it never overturned a lead | same |
| Left out: Taut, oats, the knife | Taut is always best at full (a stat, not a choice); oats lost share; the cut never turned a game | field retest, *The verb census* |

The rival answered your habit with the one that beats it after a delay you
picked (12, 8 or 4 s), and from 0:30, once every 30 s while behind, walled
your road to your most forward pile.

**The rival against scripted hands** (8 s delay, 16 seeds each, share of the
food you ate at the bell; raw: [`raw/hands-v1.txt`](rts-skirmish-2026-11-10/raw/hands-v1.txt)):

| Your hand | Games won | Share, range (median) |
|---|---|---|
| idle (spread, nothing else) | 0/16 | 0.347-0.460 (0.399) |
| random (a random habit every 10-20 s, random walls) | 3/16 | 0.347-0.555 (0.468) |
| counter (switches to what beats the rival's habit at once; walls when behind) | 14/16 | 0.427-0.559 (0.526) |
| late (the same, 10 s slower) | 7/16 | 0.458-0.550 (0.495) |

Paired by seed, counter beat idle 16/16 and random 14/16 (sign test p 0.004);
late beat idle 16/16; counter beat late 11/16 (p 0.21). So the plan beats
random, as a good dish needs, but the plan is a lookup (press the counter),
and only 2 of 16 counter games reached the provisional 55% star.

**The owner played it, 21:14Z:** "This is still using the old tools, but
not sure what the strategy is with these tools. The lamp moves the slime
mold, and it used to work well for pushing the slime mold to food it wasn't
finding on its own. At 22.5 the mold moves and explores well on its own, so
I don't know how to use the tool to cause any affect. The salt should block
a path but it doesn't have as strong of a reaction as I was expecting.
Overall, user input seems limited and doesn't lead to any complex stratgy".
Version 1 is set aside; its page stays up (below) as the record.

## Version 2: the budget skirmish

The proposal that followed (21:15Z), which the owner called "an
interesting direction" (21:17Z): drop the herding tools and give the player
the decisions the mould makes alone, each with a cost. Food fills a bank;
**beacons** make your mould crave one pile; **spores** plant a colony on a
pile; habits stay but cost.

### The strategy map, from the model

What each verb buys follows from four facts about the mould, read off the
duel traces (`TRACE=1`, seed 1, both sides doing nothing):

1. **The field is full by 0:10.** Each mould stands on every pile of its own
   half within ten seconds (12 and 11 of 23 piles held). There is nothing
   to expand into unless new food lands.
2. **A pile feeds 40 eaters, shared by head count** (`Params::table`, `Pile::bite`).
   Extra walkers on a pile you hold alone buy nothing; extra walkers on a
   pile you share take a bigger part of it. Strength counts only where the
   moulds meet.
3. **Fights favour the fed four to one** (`Params::fight`: half the rate on
   fed tube, double on unfed). A mould cut off from its food starves and is
   eaten into, so losing piles loses fights, which loses more piles: the
   game snowballs. In the round robin below, 300 of 576 games ended before
   the bell with one mould gone, and the side ahead at 1:00 won 416 of 473.
4. **The food runs out at about 1:50** (375,000 food; the two moulds eat
   about 3,400 a second between them), so the late game barely moves the
   score.

So, in this model:

| Verb | What it buys |
|---|---|
| Tithe | Money now against body now: at 25% each pile feeds a quarter fewer of your walkers. Cheap where you are alone on your piles (40 eaters still stand there), dear at the seam, where head count decides |
| Spore | 150 walkers put down on a pile at once, already on food and so already fed, without walking through the rival's trail, which repels yours |
| Beacon | Your whole network leans toward one pile for 20 s |
| Habit | The network's shape, and whether it is drawn to the rival's trail (hunt) |

Facts 2 and 3 predict that a concentrated attack is strong (a spore wins
the head count at one pile and fights there at the fed rate, while the
defender's walkers are spread over every pile), and that the first lead
snowballs, so an early attacker should beat a saver. Fact 1 says expansion
pays only when food lands. The open question was whether defence can answer
attack, which would give the triangle RTS games are built on: attack beats
greed, defence beats attack, greed beats defence.

### The plans

Nine scripted strategies (`skirmish::Plan`), each thinking once a second and
seeing only what a player sees: the piles, who eats at each, late food once
it lands, its own bank and the rival's habit.

| Plan | Tithe | What it buys |
|---|---|---|
| idle | 0 | nothing: the thickest mould (control) |
| random | random | something random about one second in four (control) |
| boom | 25% | races for each new pile, nearest home first |
| raid | 40% | hunt, then beacons and spores on the rival's piles nearest its home |
| soft | 25% | beacons, then spores, on the rival's pile with the fewest of its walkers |
| centre | 25% | the piles within 40 cells of the centre line it does not hold, then expands |
| fort | 10%; 25% under attack | reinforces its own piles the rival eats at |
| hoard | 40% to 1:20, then 10% | saves, then raids with everything |
| counter | 10% | habits only: answers the rival's habit with the one that beats it |

### Round 1: money for nothing

The first build banked a tenth of all food eaten without taking it from
the mould (spore 800 food for 150 walkers, beacon 300 for 20 s at three
times a full pile's scent, at most three lit, habit 200; no late food).
Eight plans (soft in place of boom, which never found a free pile and so
played exactly as idle), 2 seeds, both sides, 112 games
([`raw/rr0-free-money.txt`](rts-skirmish-2026-11-10/raw/rr0-free-money.txt)):

- Soft won all 7 of its pairings (27 of 28 games); idle lost all 7.
- Any spending beat none: random beat idle 4/4.
- Too few seeds for any pairing to survive Holm (4/4 is p 0.125); the
  pattern is what led to round 2.

Two flaws followed from the model: free money makes every purchase pure
gain, so "save or spend" is no decision; and with the field full by 0:10,
expansion had nothing to buy. This build's code was not kept: round 2
replaced the free bank with the tithe the same hour.

### Round 2: money is your own food, and new food lands

Two changes (commit a8bd5b2, `Rules::default()`):

- **The tithe** (`Dish::tithe`, `skirmish::TITHES` = 0, 10, 25 or 40%): that
  fraction of every bite goes to the bank instead of the mould. The score
  (food eaten) is unchanged; the mould is thinner. Spore 1,000 for 150
  walkers (the colony itself is worth 712: 1 food a birth plus the 3.75 each
  carries).
- **Late food** (`skirmish::late_food`): from 0:20, every 15 s, a pile on the
  centre line or a mirrored pair, somewhere new each seed; seven arrivals,
  10 piles of 12,000.

Nine plans, 8 seeds, both sides, 576 games, run by four runner threads
(job lists [`jobs/`](rts-skirmish-2026-11-10/jobs/), raw
`raw/rr1-runner-1..4.txt`, analysis
[`raw/analysis-rr1.txt`](rts-skirmish-2026-11-10/raw/analysis-rr1.txt) by
[`analyse.py`](rts-skirmish-2026-11-10/analyse.py)). One game was rerun here
and matched its runner's line exactly. Each pairing is 16 games, a game won
when the plan ate more than half; sign test, Holm over 36 pairings.

| Plan | Pairings won (surviving Holm) | Lost (surviving) | Mean share | Bought a game: beacons, spores, switches |
|---|---|---|---|---|
| **soft** | 8 (8) | 0 | 0.737 | 48.9, 12.9, 0 |
| raid | 7 (3) | 1 (1) | 0.615 | 38.9, 34.1, 1.0 |
| idle | 4 (3) | 4 (2) | 0.548 | none |
| centre | 6 (1) | 2 (1) | 0.540 | 40.7, 21.8, 0 |
| counter | 5 (3) | 3 (1) | 0.510 | 0, 0, 1.3 |
| random | 3 (0) | 5 (1) | 0.465 | 7.1, 6.8, 2.5 |
| fort | 1 (1) | 6 (3) | 0.431 | 35.2, 22.0, 0 |
| boom | 1 (1) | 6 (4) | 0.389 | 13.6, 26.6, 0 |
| hoard | 0 | 8 (7) | 0.264 | 11.6, 12.2, 0 |

- **Soft beat every other plan**: 16/16 against raid, centre, counter, fort,
  boom, hoard and random, 15/16 against idle (p 0.0005), all surviving Holm.
- **No cycle.** Among the 20 pairings that survive Holm there is no
  three-way cycle, and none even on the bare majority of all 36: the plans
  stand on a ladder.
- **Doing nothing is third.** Idle, with no tithe and so the thickest mould,
  beat boom, fort and hoard 16/16 each: a tithe spent badly is worse than
  none.
- **Saving loses to everything** (hoard lost all 8, 7 surviving): by 1:20
  the lead has snowballed and the food is nearly gone.
- **Expansion does not pay:** boom (racing for late food) lost to idle 16/16.
- **Defence does not answer attack:** fort lost to soft 16/16 and to raid
  12/16 (p 0.08).

**Why soft wins.** It hits where the rival is thinnest, so its spore wins
the head count at once (fact 2), fights there at the fed rate (fact 3) and
cuts the rival's tube beyond, which starves. The defender's walkers are
spread over every pile, and any reinforcement it buys lands after the
attack. Raid, which hits the nearest pile however well held, is second.

### Looking for an answer to soft

Four probes, 2 seeds both sides, built on round 2's rules
(`RULES=...` in the harness; raw `raw/probe-*.txt`):

| Change | Result | Raw |
|---|---|---|
| **Guard**: beacons on its own thinnest border piles before any attack, spores on any pile attacked (10%) | lost to soft 4/4 and to idle 4/4 | `probe-guard-lean.txt` |
| **Lean**: soft's target, beacons only, 10% tithe | lost to soft 4/4; won 1 of 4 against idle (beacons alone do about nothing) | same |
| **Spore reach 25**: a spore may land only within 25 cells of your own fed tube | soft still beat boom, centre and fort 4/4, idle 3/4 (the field is so meshed that everything is in reach) | `probe-reach25.txt` |
| **Dormant spores, 5 s**: a spore lands as sleeping walkers (`Walker::slept`), which hold their cells but fight as unfed until they wake | soft still beat boom, fort and guard 4/4; idle drew level with soft, 2/4 | `probe-dormant5.txt` |
| Dormant only abroad (spores at home wake at once, so a defender's reinforcement is quicker than the attack) | the same: soft 4/4 against boom, fort and guard, idle 2/4 | `probe-dormant5-abroad.txt` |

Only one thing dented soft: a dormant spore among a thick, untithed mould.
Being thick everywhere is the natural answer to an attack on the thinnest
spot, once attacks are slow enough to answer; but no scripted defender
that spends found it.

## What would make the strategy deeper

Ideas from the model, each a change to test with the same round robin
(about 20 minutes on four runners):

1. **A cheap, quick defensive verb.** Beacons alone do nothing (lean tied
   idle), but a short strong rally (every walker within reach of one of your
   piles drawn to it for 5 s) would let a defender meet a spore where it
   lands. With dormant spores, attack would be dear and slow and defence
   cheap and quick: the shape that makes greed beat defence and attack beat
   greed.
2. **A price that rises.** Each spore within 30 s costs more than the last,
   so spending everywhere at once is impossible and the target matters.
3. **Exposure.** A side that tithes is thin at home; a plan that answers an
   attack by hitting the attacker's thinnest pile ("trade") may beat soft by
   racing it. Untested.
4. **A person.** The plans are one-line rules. A player who reads the
   picture (where the rival is thin, where its spores sleep) may find
   answers the scripts did not; the round robin says only that no simple
   rule beats soft.

## How it is built

- `src/mould/mod.rs`: `Dish::beacons` (a pile smells `gain` times a full
  pile to one mould only; `fed_scent` does not apply), `Dish::tithe` and
  `Dish::tithed` (read by the shared store and by `Params::pieces`), and
  `Dish::plant` / `Dish::plant_rival` (a spore: walkers carrying food, round
  the other mould, other walkers' food untouched). Hashes are unchanged for
  a dish that uses none of them.
- `src/mould/skirmish.rs`: the field, `Habit`, `Verb`, `Plan`, `Rules` (every
  price and reach, so the harness can try others), `Side` (habit, tithe,
  bank, lit beacons, tally), `Skirmish` (one dish, two sides, late food,
  dormant spores) and `front`. Four tests: the field is mirrored and even;
  the habit cycle closes; the tithe fills the bank and buying spends it; a
  beacon pulls only its own mould.
- The harness's `duel SEED LEFT RIGHT` mode
  (`rts-design-2026-11-10/red-team/harness`), with `TRACE=1` (both sides
  every 10 s), `LOOK=DIR` (a still every 15 s) and `RULES="name=value ..."`.
  Its v1 `skirmish` mode is in git history at 6c28e01.
- `web/`: the browser build. The library compiles for
  `wasm32-unknown-unknown` unchanged; `web/src/lib.rs` is a cdylib with
  plain `extern "C"` functions (no generated glue) that steps the dish and
  draws it with the game's own `render::draw` at scale 4. The page in
  `web/page/` is still version 1's; until version 2 has its own page,
  `lib.rs` runs the new rules under the old interface (the rival plays
  counter; you tithe 10%).
- **Frame cost** (version 1, headless Chromium on the 4-core cloud box with
  every core busy, 30 s in): drawing 9.3 ms, stepping and drawing 14.3 ms a
  60th of a second. A beacon adds one comparison per pile per step while
  lit, and the tithe one multiply per bite.

## Pages

- Version 1, playable: https://claude.ai/artifact/NDJJZJuabHpjMjLNBr45Lw
  (source `web/page/`).
- Version 2's strategies, with the matrix and stills:
  [`Reports/pages.md`](pages.md), *Budget Skirmish Strategies*.
