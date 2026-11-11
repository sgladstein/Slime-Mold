# The field skirmish: a playable RTS demo, and what wins it (2026-11-10)

**Status: version 1 played by the owner and set aside; version 2 (the
budget skirmish) playable at https://claude.ai/artifact/936ACBn8mJUkvFpb4iSnUS,
not yet played by the owner; it opens on desperate fights (round 5).** The owner, 2026-11-10
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

**Round 3 (the counterplay rules) did not change that.** With spores asleep
for 5 s where they land abroad, a spore price that rises half again for
each one bought in 30 s, a cheap 5 s rally for defence, and three new plans
built to use them, soft still lost no pairing in 720 games (124 of 144 games;
6 of its 8 wins survive Holm). Only its own copy with a rally added (keep,
8/16) and the plan that answers attacks with soft's own strike (trade, soft
9/16) hold it. Played on the same seeds and sides, the new rules moved soft's
record against idle from 15 to 12 of 16 (share lower in 8 of 16 games, p 1)
and against raid, centre and fort not at all. Pure defence with the rally
lost to soft 16/16. See *Round 3*. The game also favoured the left mould,
yours on the page: in mirror games it won 14 of 16 (fixed since, 2026-11-11:
the piles sat half a cell off a mirror; see *Which side you play matters*).

**Rounds 4-6 (changes to the model itself, 2026-11-11) did not change it
either.** The owner asked to explore model changes (23:29Z). Over 3,136
games on the fixed layout, against a control of the counterplay rules
alone: a holder's bonus at piles made the snowball worse (wipe-outs 271
against 235); supply lines (walkers far along their tube fight as unfed)
changed nothing; upkeep that grows with size eased it most (wipe-outs 100,
comebacks 79 against 48) but let doing nothing tie the best plans (idle
took 10/16 off soft and keep); and desperate fights (starving walkers fight
harder) eased it with the tools intact (wipe-outs 198, margins smaller,
paired p 4e-5), so **the page now opens on desperate fights**. Under every
rule set soft and its copy with a rally, keep, are the top two, and keep
lost no pairing under six of the seven (all but size upkeep). About half
the food is eaten by 1:00, so even with no snowball the side ahead then
would win about 70% of games that reach the bell: a comeback needs food
after 1:00, which is the economy sketch's next test. See *Rounds 4 to 6*.

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

### Round 3: the counterplay rules

The first three ideas from *What would make the strategy deeper*, built
(commit 8e3bbe7) and run as one rule set,
`RULES="spore_dormant=5 dormant_abroad_only=1 spore_ramp=0.5"`:

- **Rally** (`Verb::Rally`, 150, 5 s): a beacon at gain 10 (a beacon's is
  3), only on a pile the side is already eating at.
- **Rising spore price** (`Rules::spore_ramp` 0.5): a spore costs 1,000 x
  (1 + 0.5 x your spores in the last 30 s).
- **Sleeping spores** abroad (5 s), as in the last probe.
- Three plans: **trade** (25%; saves until a rival spore lands on, or a
  rival beacon or rally is lit on, one of its piles, then soft's strike for
  10 s), **keep** (soft, but first a rally on any of its piles a rival
  spore is landing on), **ward** (10%; rallies on piles under attack,
  nothing else). Random now draws rallies too.

Ten plans (idle, random, soft, raid, centre, fort, guard, trade, keep,
ward), seeds 1-8, both sides: 720 games on four runner threads (raw
`raw/rr2-runner-1..4.txt`; `analyse.py` output `raw/analysis-rr2.txt`). The
deciding measure, named before the run: does any plan beat soft in a
pairing, and is there a cycle among pairings that survive Holm over 45?

| Plan | Pairings won (survive Holm) | Lost (survive) | Mean share |
|---|---|---|---|
| soft | 8 (6) | 0 (0) | 0.676 |
| keep | 7 (6) | 1 (0) | 0.671 |
| idle | 6 (3) | 2 (0) | 0.561 |
| trade | 5 (4) | 4 (2) | 0.557 |
| random | 4 (0) | 5 (2) | 0.455 |
| centre | 4 (0) | 5 (3) | 0.431 |
| ward | 4 (1) | 5 (3) | 0.428 |
| raid | 1 (0) | 7 (3) | 0.414 |
| guard | 2 (0) | 7 (4) | 0.409 |
| fort | 2 (1) | 7 (4) | 0.399 |

- **No plan beats soft.** Soft beat idle 12/16 (p 0.077), trade 9/16 and
  keep 8/16; every other plan 15-16/16.
- **Paired against round 2** (same seed, same side, the plans whose code
  did not change): soft against idle won 15 then 12 of 16, its share lower
  in 8 of 16 games (p 1); against raid 16 and 16 (lower in 11/16, p 0.21);
  centre and fort 16 and 16. The rules did not move soft.
- **The rally does not defend.** Ward, which only rallies (17 a game), lost
  to soft 16/16 and to keep 16/16; fort lost 16/16 as before. Keep bought
  4 rallies a game and tied soft 8/16, so the rally added nothing
  measurable. A rally pulls walkers to a pile the attacker has already
  thinned; it does not change who is fed there.
- **Trade is soft when attacked and dead weight when not.** Against soft it
  is attacked all game and strikes back all game (9/16 to soft). Against
  idle and ward, who never attack, it tithes 25% and never spends: idle beat
  it 16/16, ward 16/16.
- **One cycle survives Holm:** fort > ward (15/16) > trade (16/16) > fort
  (16/16). It is made of the tithe's cost (ward 10% and trade 25%, both idle
  against a quiet rival) and of fort's strike, not of a strategic choice; all
  three lose to soft 16/16.
- **Snowball unchanged:** 353 of 720 games ended in a wipe-out (round 2:
  300 of 576); the side ahead at 1:00 won 636 of 698 (round 2: 416 of 473).
  Against the four rivals both rounds share, soft bought 10.2 spores a game
  under the rising price, against 12.3 before.

**Why.** The rules gave the defender time and a tool, but the attack's
strength is not speed: a spore lands on the pile where the defender is
thinnest, and from then on the head-count share and the fight rule
(`Params::fight`: a walker on fed tube is half as likely to die, on unfed
tube twice) keep feeding whoever is ahead there. A rally brings walkers,
but they arrive unfed into a crowd that is fed.

### Which side you play matters (found and fixed)

The field is mirrored, but the two moulds are not stepped alike: yours is
the dish (`Dish`), the rival's is `Dish::rival`. In mirror games under the
round 3 rules (`raw/mirror-rr2.txt`, seeds 1-8) the left mould won idle v
idle 7 of 8 (mean share 0.524) and soft v soft 7 of 8 (0.612): 14 of 16
together, p 0.004. Over all 720 round 3 games the left side won 459 (round
2: 343 of 576). The round robins play every pairing from both sides, so
they are fair; a person on the page played left and had this edge.

**Tracing it** (idle v idle, seeds 1-32, raw `raw/side-swap-turns.txt`
with its job list, and `raw/side-half-cell.txt`):

| Setup | Left won | Left's mean share |
|---|---|---|
| As built (your mould left, moves first every step) | 24/32 (p 0.007) | 0.523 |
| Taking turns to move first (`Dish::take_turns`) | 24/32 | 0.534 |
| Your mould started on the right instead | 19/32 | 0.521 |
| The same, taking turns | 20/32 | 0.511 |
| Every side pile half a cell toward the centre line | 16/32 | 0.505 |

The edge followed the left **start**, not your mould, and moving first did
not matter. The cause was half a cell: a pile's own cells (where `fed`
starts and spores land) are the disc of cells round `floor(x)`, so a pile
at x = 40.0 owned the cells round cell 40 while its mirror at 216.0 owned
those round cell 216, one column off the true mirror (cell 215). Every
pile on the left sat one column nearer the middle than its twin.

**Fix:** `PILES` and `HOMES` now sit half a cell in (40.5 and 215.5, and
so on), which mirrors exactly in cells as well as in space. On the fixed
layout (`raw/mirror-fixed.txt`) the left won idle v idle 14 of 32 (mean
0.504) and soft v soft 6 of 16 (0.488): even. Taking turns stays on in the
skirmish (`Rules::take_turns`): it costs nothing and measured no harm. The
three centre-line piles stay on 128.0, one column right of centre. Rounds
1-4 ran on the old layout, paired by side, so their comparisons stand.

### Rounds 4 to 6: changing the model itself

After round 3 the report said the fix "probably has to be in the model
itself"; the owner (23:29Z): "then lets explore this". Five changes, each a
`Params` field set on both moulds by `Skirmish::set` (the harness's
`RULES`), all off by default with hashes unchanged (commits b360da0 and
114d4d3):

- **Holder's bonus** (`hold` 2, `hold_flip` 2): a pile is held by the
  mould that ate there alone (`Pile::held`); the holder's eaters count
  double in the head-count share until the other has twice as many there.
- **Size upkeep** (`upkeep_size` 0.2): each walker's upkeep is the flat
  upkeep x (1 + 0.2 x walkers / 1,000), so a mould of 5,000 pays double for
  every walker.
- **Desperate fights** (`fight_fed` 2, `fight_unfed` 0.5): the fight's
  multipliers reversed, so walkers on tube no food reaches fight harder.
  **Even fights** (1 and 1) was screened only.
- **Supply lines** (`supply` 10, 20 or 40 cells): a walker fights as fed
  only within that many cells of tube from eaten food, as the research
  thread's [economy sketch](rts-research-2026-11-10/skirmish-economy-sketch.md)
  proposes (its test 2, with its target: the side ahead at 1:00 wins under
  70%).

Every set ran on top of round 3's counterplay rules. **Frame cost:** none
adds a pass over the dish. Supply and the fight multipliers change the
comparison and the factor the fight already uses per walker at a front;
size upkeep adds one multiply where upkeep is charged; the holder's bite a
branch per pile bite and a pass over the 23 piles before the rival copies
them.

**Round 4, the screen** (old layout, seven plans, seeds 1-4 from both
sides: 8 games a pairing, 168 a rule set; raw `raw/rr3-runner-1..4.txt`,
`screen.py` output `raw/analysis-rr3.txt`). With the counterplay rules
alone no plan took a majority off soft. Each change let one do so: hold
(trade 6/8), even (keep 6/8, trade 5/8), desperate (trade 5/8), size
(keep 5/8), and size with all three (keep 5/8). Under hold a majority cycle
appeared at the top: soft > idle > trade > soft.

**Rounds 5 and 6, full round robins** on the fixed layout: eight plans
(idle, random, soft, raid, fort, trade, keep, ward), seeds 1-8, both sides,
448 games a rule set, 3,136 in all. Raw: `raw/rr5-counter.txt` (the
control, the counterplay rules alone, run on this box), `raw/rr4-runner-1..4.txt`
(hold, size, desperate) and `raw/rr6-runner-1..4.txt` (supply), on runner
threads. `screen.py` gives the records, `analyse.py` (one rule set at a
time) the Holm tests over 28 pairings, and `compare.py` the game-by-game
comparison with the control. The deciding measures, named before the run:
does a plan beat soft, does a cycle survive Holm among plans that beat
random, and how often does the side ahead at 1:00 win.

| Rules (each on top of round 3's) | Ahead at 1:00 won | Comebacks | Wipe-outs | Close | Top two (pairings won-lost, mean share) | Soft and keep v idle | Cycles surviving Holm |
|---|---|---|---|---|---|---|---|
| Counterplay alone (control) | 389/437 (89%) | 48 | 235 | 145 | keep 7-0 (0.673), soft 5-2 (0.659) | 15/16, 15/16 | none |
| Holder's bonus | 394/435 (91%) | 41 | 271 | 117 | keep 7-0 (0.689), soft 5-2 (0.682) | 16/16, 16/16 | idle > trade > raid; fort > ward > trade |
| Size upkeep | 363/442 (82%) | 79 | 100 | 274 | soft 4-1 (0.594), keep 5-1 (0.588) | 6/16, 6/16 | none |
| Desperate fights | 363/430 (84%) | 67 | 198 | 145 | keep 7-0 (0.661), soft 5-2 (0.640) | 16/16, 16/16 | none |
| Supply 10 | 378/433 (87%) | 55 | 239 | 138 | keep 6-0 (0.677), soft 6-1 (0.672) | 16/16, 16/16 | fort > ward > trade |
| Supply 20 | 378/432 (88%) | 54 | 228 | 147 | keep 6-0 (0.680), soft 6-0 (0.671) | 15/16, 15/16 | fort > ward > trade |
| Supply 40 | 392/438 (89%) | 46 | 241 | 143 | keep 7-0 (0.665), soft 5-2 (0.659) | 14/16, 14/16 | fort > ward > trade |

"Close" is a game the winner took with under 60% of the food. "Comebacks"
are games the side behind at 1:00 won.

**Paired against the control**, game by game (the same seed, plans and
sides; `raw/compare-rr5-rr6.txt`). Each cell: games where only this rule
set had it, against games where only the control did, and the sign test p.
Holm over the 24 tests: size's four, and the wipe-outs and margins of hold
and desperate fights, survive; hold's close games (p 0.004) and desperate
fights' comebacks (p 0.05) do not.

| Rules | Wipe-outs | Comebacks | Close | Margin smaller / larger |
|---|---|---|---|---|
| Holder's bonus | 65 / 29 (p 0.0003) | 28 / 35 (p 0.45) | 31 / 59 (p 0.004) | 185 / 262 (p 0.0003) |
| Size upkeep | 2 / 137 (p 3e-38) | 67 / 36 (p 0.003) | 139 / 10 (p 3e-30) | 339 / 106 (p 2e-29) |
| Desperate fights | 44 / 81 (p 0.001) | 52 / 33 (p 0.05) | 51 / 51 (p 1) | 268 / 180 (p 4e-5) |
| Supply 10 | 36 / 32 (p 0.72) | 38 / 31 (p 0.47) | 36 / 43 (p 0.5) | 212 / 230 (p 0.42) |
| Supply 20 | 33 / 40 (p 0.48) | 29 / 23 (p 0.49) | 35 / 33 (p 0.9) | 214 / 229 (p 0.51) |
| Supply 40 | 42 / 36 (p 0.57) | 22 / 24 (p 0.88) | 32 / 34 (p 0.9) | 230 / 205 (p 0.25) |

- **No change to the model dethrones the strike.** Under every rule set
  the top two are keep and soft: beacons, then spores, on the rival's
  thinnest pile, keep adding a rally where a spore lands on it. Keep lost no
  pairing under the control, the holder's bonus, desperate fights or any
  supply limit. The screen's upsets were noise at 8 games: trade against
  soft went from 6/8 under the holder's bonus to 9/16 (p 0.8), and from
  5/8 under desperate fights to 9/16. Keep and trade take a bare majority
  off soft even under the control (10/16 and 9/16), so soft and keep are one
  family, and no plan outside it beats either.
- **Size upkeep eases the snowball most, and makes the tools matter
  less.** It ended 100 games in a wipe-out against the control's 235 and
  had the most close games (274 against 145) and comebacks (79 against 48).
  But doing nothing caught up with the best plans: idle took 10 of 16 off
  both soft and keep (p 0.45), where soft and keep had beaten idle 15/16
  under the control, and they beat random 13/16 rather than 15/16. A mould
  that buys walkers pays for every one of them every second, so a strike
  earns little more than it costs. By the rule that every tool must deliver
  something you can see, that is a measured harm, so size upkeep is not the
  default.
- **Desperate fights ease it and leave the tools intact.** Fewer
  wipe-outs (198; paired 44 against 81, p 0.001), smaller margins (p 4e-5)
  and more comebacks (67 against 48; paired p 0.05, which does not survive
  Holm), while soft and keep still beat idle 16/16 and random 14/16. **The
  page now opens on desperate fights.**
- **The holder's bonus makes it worse:** more wipe-outs (271; paired 65
  against 29, p 0.0003) and wider margins. Holding a pile is already what
  the side ahead does, so the bonus is paid to the leader.
- **Supply lines change nothing** at 10, 20 or 40 cells: every paired
  measure has p of 0.25 or more, and the side ahead at 1:00 won 87-89%.
  This fails the economy sketch's test 2 (target: under 70%). A spore's
  walkers land on a pile and eat there, so they start their own fed tube;
  a short supply weakens only long tubes, which are not where games are
  decided (inferred, not traced). The first look (soft against trade, seed
  3), where supply 20 turned soft's 0.585 at 1:00 into a tie, was not
  typical.
- **The 70% target is out of reach for any change to the fight.** About
  half the food is eaten by 1:00 (46-47% in three traced games on seed 3:
  soft v trade, keep v soft, idle v idle; `raw/trace-s3-*.txt`). Shuffling what happens after
  1:00 between games, which removes any snowball, the side ahead at 1:00
  would still win 67-70% of the games that reach the bell, under every rule
  set; the real figures among those games run from 77% (size upkeep) and
  83% (desperate fights) to 89% (the control; `compare.py`'s last table).
  The snowball adds 10-20 points to a lead that is mostly banked already. A comeback needs food after 1:00: the sketch's longer
  match with regrowing piles, or a score that is not food eaten.

## What would make the strategy deeper

Tested and failed: prices and a defensive rally (round 3), and four changes
to the model (rounds 4-6): the holder's bonus makes the snowball worse,
supply lines do nothing, size upkeep eases it but lets doing nothing tie the
best plans, and desperate fights ease it a little (now the page's default).
Under all of them the strike on the thinnest pile wins. What is left:

1. **Food after 1:00.** About half the food is eaten by 1:00 and the bell
   is at 2:30, so a lead at 1:00 is mostly banked; even with no snowball the
   side ahead then would win about 70% of the games that reach the bell.
   The economy sketch's longer match (4:00) with piles that regrow (faster
   where nobody has eaten for 20 s), or a score from ground held over time,
   is the next test: its test 3, with wipe-outs and comebacks as the
   measures and this report's shuffled 70% as the floor to compare with.
2. **An answer tied to the strike.** The sketch's Fortify against a strike
   with a countdown both sides see (its test 1). At a pile both sides'
   walkers already fight as fed, so Fortify must do more than make walkers
   count as fed there: a lower death rate inside the ring, for example.
3. ~~Fix the side edge~~: done, see *Which side you play matters*.
4. **A person.** The plans are one-line rules. A player who reads the
   picture (where the rival is thin, where its spores sleep) may find
   answers the scripts did not; the round robins say only that no simple
   rule beats soft and keep.

## Version 2's page

https://claude.ai/artifact/936ACBn8mJUkvFpb4iSnUS (source `web/page/`,
`web/src/lib.rs`). You are the gold mould on the left. Below the dish: your
bank and the four tithes; beacon, spore and rally, each with its price now
(a spore's rises under the counterplay rules); the four habits with their
price and cooldown. Pick one and click a pile: a press lands on the live
pile within 9 cells. Beside it: the food share, both sides' walkers, bank,
tithe and habit, and a feed of what each side bought. You pick the rival's
plan (any of the scripted ones, soft by default, with a line saying what it
does) and the rules: desperate fights (the default since round 5), size
upkeep, the holder's bonus, the counterplay rules alone, or as first built.
Each model change runs on top of the counterplay rules. Pause
stops the clock and buying still works, so a player can plan a turn.

Rings drawn over the dish show what is bought (gold yours, violet the
rival's; thin a beacon, thick a rally, the arc the time left), "zz" and a
count where a spore still sleeps, and a dashed ring with the price where a
press would land. They are the interface, not the simulation. The frame
itself is `render::draw` with `View::bare`, a new flag that leaves out the
herding game's corner chips, tool bar and cards (which named its stars and
the lamp); the page draws its own start card instead.

**First look, in a test browser:** soft attacks within the first 10 s, and
a hand that tithed 40% and bought a rally, a beacon and one spore by 0:23
held 578 walkers to the rival's 4,582 (31% of the food). That matches the
scripts: hoard, which tithes 40% and buys nothing, was gone by 0:44 against
soft on seed 7 under the counterplay rules. The page shows the same game the
round robins measured.

**Frame cost:** 8.9-9.4 ms a frame on average over the first 30 s, headless
Chromium on the 4-core box, which drew every other frame; the rings add 23
piles' worth of canvas arcs a drawn frame.

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
- The model's levers for rounds 4-6, all `Params` and off by default with
  hashes unchanged: `fight_fed` and `fight_unfed` (the fight's multipliers,
  0.5 and 2), `upkeep_size`, `hold` and `hold_flip` (`Pile::held`,
  `Pile::bite_held`), and `supply`. `Skirmish::set` sets a rule or one of
  these on both moulds; three tests (the holder's bite, size upkeep, and a
  short supply only adding losses at the same front).
- Analysis scripts in `rts-skirmish-2026-11-10/`: `analyse.py` (one round
  robin: sign tests, Holm, cycles), `screen.py` (several rule sets, tagged
  lines: records, majority cycles, the snowball) and `compare.py` (rule sets
  game by game against a control, and the shuffled null).
- The harness's `duel SEED LEFT RIGHT` mode
  (`rts-design-2026-11-10/red-team/harness`), with `TRACE=1` (both sides
  every 10 s), `LOOK=DIR` (a still every 15 s) and `RULES="name=value ..."`.
  Its v1 `skirmish` mode is in git history at 6c28e01.
- `web/`: the browser build. The library compiles for
  `wasm32-unknown-unknown` unchanged; `web/src/lib.rs` is a cdylib with
  plain `extern "C"` functions (no generated glue) that steps the dish and
  draws it with the game's own `render::draw` at scale 4 (`View::bare`).
  The page in `web/page/` is version 2's; version 1's is copied in
  `rts-skirmish-2026-11-10/page-v1/`.
- **Frame cost** (version 1, headless Chromium on the 4-core cloud box with
  every core busy, 30 s in): drawing 9.3 ms, stepping and drawing 14.3 ms a
  60th of a second. A beacon adds one comparison per pile per step while
  lit, and the tithe one multiply per bite.

## Pages

- Version 1, playable: https://claude.ai/artifact/NDJJZJuabHpjMjLNBr45Lw
  (copy in `rts-skirmish-2026-11-10/page-v1/`).
- Version 2, playable: https://claude.ai/artifact/936ACBn8mJUkvFpb4iSnUS
  (source `web/page/`).
- Version 2's strategies, with the matrix and stills:
  [`Reports/pages.md`](pages.md), *Budget Skirmish Strategies*.
