# Mini-boss review (2026-11-20)

The owner, 2026-11-20 05:41Z: "I want another high effort thread to
analyze the minibosses, including looking at my playtests and improve
them". The owner's doc for this review: https://claude.ai/artifact/NB1CsrKYfiK9ips8UePUke

## His playtests

His notes on mini-bosses are in `wiki/decisions-game.md` (11-16 04:58Z and
05:23Z on what they are; 11-18 15:56Z: none on planet 1, done in #490). His
13 logged rounds on the `play-logs` branch carry no boss or raid lines (the
play log did not write them), so what a boss did to him could not be read.
The review therefore played his own maps: the run seeds of his logs
(1795038793, 1795106014, 1795119328), planets 2 to 4, with the strategy bot
(`hand=strat:econ,go=4,aim=6`, the league's economy plan) to 5:00, on the
software GPU (`tools/census.sh`; table by `tools/table.py`; the boss and
raid lines of every game in `census/`). The play log now writes them, so
his next rounds will show what each boss did.

## What they did (main at dd38b09, 17 bosses formed before 5:00)

The deciding measures, named first: how many reach the squad or their
errand, and how long they stand still on the way.

| | hunter | leech | sapper | wanderer | all |
| --- | --- | --- | --- | --- | --- |
| formed | 6 | 4 | 4 | 3 | 17 |
| reached squad (<20 units) | 2 | 1 | 0 | 0 | 3 |
| came within 30 units of its errand | 0 | 0 | 0 | 1 | 1 |
| got to its errand | 0 | 0 | 0 | 1 | 1 |
| took food | 0 | 0 | 0 | 0 | 0 |
| stalled 10 s or more | 2 | 2 | 2 | 3 | 9 |
| stood still 10 s+ (seconds) | 25 | 78 | 84 | 71 | 258 |
| swallowed by a march | 2 | 0 | 1 | 0 | 3 |
| ended Broken | 4 | 1 | 1 | 0 | 6 |
| ended Spent | 0 | 2 | 1 | 1 | 4 |
| ended Fizzled | 0 | 0 | 0 | 0 | 0 |
| still alive | 2 | 1 | 2 | 2 | 7 |

- No leech or sapper came within 30 world units of its errand (one
  wanderer rooted on its crystal); the leeches' pumps were 150 to 250
  units away.
- 9 of 17 stood 10 s or more within 3 units of where they were (258 s in
  all), pressed on rock, a fort's ring, a river's edge, or with the tail
  wedged in packed slime and the leash holding the head back.
- 3 vanished the second a raid's march set off beside them: forming the
  march's lump takes every walker in reach, members of a boss included (a
  hunter of 437 fell to 15 on run 1795119328 planet 3; a sapper of 421 to
  none on run 1795038793 planet 4).
- They formed 130 to 330 units from the squad, deep in the body.

`stalled` counts seconds in a walk or hunt with the centre within 3 units
of where it was 10 s before.

## What changed (on by default; no engine change)

All in `larder_world/bosses.rs`, two lines of `raids.rs`, and the play log.

- **A march grows its lump** (`World::grow_lump`) from walkers in no lump,
  so it never takes a boss's members.
- **Bosses walk round rock** (`Game::boss_steer`): straight while the line
  is clear on the slime's ground (rock, the slime's and the player's walls,
  deep water closed), else down a walking field built off-frame as a raid's
  (`Way::start` to the goal; `Way::start_units` to every live soldier for a
  hunter, rebuilt each second), aimed at the farthest point down it seen
  from where it stands (a string pulled round the corner; one point a look
  ahead left a wanderer 90 s on a ridge).
- **A stuck body gathers again on its front** (`Game::boss_stall`): 6 s
  within 2 dish cells of where it last got anywhere, no soldier within
  guns' reach, it re-forms round its leading part (a body's length down the
  way, or toward the goal on a straight walk), holding no more walkers than
  its own area (`reach_holding`; a first cut at 1.5 radii took a hunter of
  437 to 4,630). This is what frees a tail wedged in packed slime: on run
  1795119328 planet 3 a leech of 545 stood 54 s at 276,70 with a clear line
  to its pump (still `stills/p3-leech-wedged-125s.png`); now it walks to 148,26,
  18 units from it, by 2:36.
- **They form nearer the player**: a crowded knot weighs its fullness times
  1 / (1 + (d / `near`)^2), d the distance to the nearest soldier or
  building, `BossRules::near` 40 dish cells, and none forms inside a live
  soldier's guns (one did and was shot in 3 s). The owner's rule (forms
  only where the slime crowds or a big part is cut off) is kept: this picks
  among crowded knots.
- **An errand's walk has its own clock**: a leech's or sapper's walk counts
  against `BossRules::trip` (90 s), `life` only after.
- **A wanderer hops toward the player**: the nearest fresh crystal nearer
  the player's things than it is, else the one nearest them.
- **The play log** writes `raid` lines, a `boss` line as each forms and
  ends, and every 5 s its phase, size, place and nearest soldier.

## After (this branch merged with main fd548db, 18 bosses formed before 5:00)

The same nine games. Main moved meanwhile (deep water a wall to the slime,
fruiting), so this is what lands, not a clean pair; the run on this
branch before the merge and before the stall fix (`after` in the commit
history) gave 6 reached the squad, 4 stalled 10 s+ (133 s), 0 swallowed.

| | hunter | leech | sapper | wanderer | all |
| --- | --- | --- | --- | --- | --- |
| formed | 7 | 4 | 2 | 5 | 18 |
| reached squad (<20 units) | 4 | 0 | 0 | 3 | 7 |
| came within 30 units of its errand | 0 | 1 | 0 | 2 | 3 |
| got to its errand | 0 | 0 | 0 | 2 | 2 |
| took food | 0 | 0 | 0 | 0 | 0 |
| stalled 10 s or more | 0 | 1 | 1 | 0 | 2 |
| stood still 10 s+ (seconds) | 6 | 13 | 19 | 2 | 40 |
| swallowed by a march | 0 | 0 | 0 | 0 | 0 |
| ended Broken | 4 | 0 | 1 | 1 | 6 |
| ended Spent | 0 | 2 | 1 | 1 | 4 |
| ended Fizzled | 0 | 0 | 0 | 0 | 0 |
| still alive | 3 | 2 | 0 | 3 | 8 |

- Standing still: 258 s to 40 s; bosses stalled 10 s or more: 9 to 2.
- Reached the squad: 3 to 7 (hunters 2 to 4, wanderers 0 to 3).
- Swallowed by a march: 3 to 0.
- Still no errand reached in 5:00: the planet 3 leech came within 18 units
  of its pump and turned when the pump went; the others walked 150 to 250
  units at 1 to 4 units a second. The trip (90 s) or the pick of a nearer
  pump is tuning; left for a review, as the owner asked.

Nine games, one bot; no sign test is claimed. Planet 2 games end by 2:30
(the bot loses its squad there), so planets 3 and 4 carry most of it.

Stills (planet 4, run 1795038793, from this branch before the deep-water
line): `stills/p4-hunter-at-river-105s.png`, a hunter of 398 pressed on the
river's edge for 70 s; `stills/p4-hunter-round-rock-140s.png`, the same hunter
strung round the rock to the squad by 2:17.

## Known limit

A lump has one aim for all its members, so on a rock bar about a third of a
body can stay pressed on the near side while the rest goes round (unit
test `a_mini_boss_walks_round_rock`, the rock bar map). The regroup
cuts that tail loose; a per-member aim would be an engine change.

## Proposals (in the owner's doc)

The Mortar first (sits at the slime's edge and lobs spores at the nearest
soldier or building past gun range; from planet 4, after Milestone 1), the
Pinwheel second (a hunter whose arms sweep round); a bounty for a kill; an
off-screen tell when one forms; a size that follows the squad.

## Follow-up: a leech or sapper forms by its errand (Direction's call, 11-20)

Direction's first rule, a leech or sapper drawn only when its pump or wall
is within reach of where it forms, drew none at all in the same nine games
at a reach of 100 or of 180 world units (2 units a second times the 90 s
trip): no pump or wall stood that near a crowded knot (planet 3, run
1795119328: the nearest pump 196 units off at 72 s). Dropped. Direction's
pick instead: the kind is drawn first, and a crowd's leech or sapper forms
at the crowded knot nearest its pump or wall (`Game::form_boss`,
`Game::crowded_knot` with the errand); the others keep the knot nearest
the player; all four kinds and the 90 s trip stay. The kind is now drawn
before the grade, so a seed's bosses differ from before.

The same nine games (`knot` lines in `census/`):

| | hunter | leech | sapper | wanderer | all |
| --- | --- | --- | --- | --- | --- |
| formed | 6 | 3 | 1 | 4 | 14 |
| reached squad (<20 units) | 3 | 2 | 0 | 1 | 6 |
| came within 30 units of its errand | 0 | 0 | 1 | 1 | 2 |
| got to its errand | 0 | 0 | 1 | 1 | 2 |
| took food | 0 | 0 | 0 | 0 | 0 |
| stalled 10 s or more | 0 | 1 | 1 | 0 | 2 |
| stood still 10 s+ (seconds) | 0 | 14 | 17 | 0 | 31 |
| swallowed by a march | 0 | 0 | 0 | 0 | 0 |
| ended Broken | 4 | 1 | 1 | 3 | 9 |
| ended Spent | 0 | 0 | 0 | 1 | 1 |
| ended Fizzled | 0 | 0 | 0 | 0 | 0 |
| still alive | 2 | 2 | 0 | 0 | 4 |

- Leeches and sappers: 6 formed (4 and 2) before, 4 (3 and 1) now; got to
  their errand 0 before, 1 now (a sapper on its wall at 4:53, run
  1795106014 planet 3).
- The leeches now form 50 to 130 units from a pump, but the bot keeps its
  pumps by its squad and builds new ones meanwhile, so by the time a leech
  has gathered its nearest pump has moved and the squad stands between:
  2 of 3 met the squad on the way (planet 3: a leech of 495 formed 56
  units from a pump and was shot to 475 with 389 hurt by the squad round
  it). A leech that reaches a pump has to come through the guns that
  guard it, which reads as intended.
- Standing still 40 s to 31 s; no boss swallowed by a march.

## The Mortar, and fruit no longer swallowing a mini-boss (11-20)

The owner picked the Mortar first (11-20 13:21Z card, over the Pinwheel).
`BossKind::Mortar` (5) is drawn from planet 4 on (`BossRules::for_depth`,
`kinds[4]`), formed at the crowded knot nearest a player thing it can see
along a clear line. It does not walk while it has a shot: every
`lob_every` (5 s) it throws the ring at its body's edge (`shell` 1.5 dish
cells thick, from 1.5 radii) at the nearest player thing within
`lob` (30 dish cells) past the longest gun's reach and the panic margin,
as one `squad::Burst` pushed one way (`Game::mortar`), so the ring flies
as a crescent and lands round the target as ordinary slime. Thrown members
are let go and the rest formed again on its spot (which heals it, so its
blows count between throws; its armour is half its grade's). It throws
its own body away: a mortar of 249 on run 1795038793 planet 4 was down to
nothing after about seven throws. Rock, deep water or a wall in the line
blocks a throw (the first throws filmed splashed on a rock ridge), so walls
are the answer. With nothing in reach along a clear line it walks toward
the player's nearest thing on its trip's clock (90 s) until it has one:
without that, the one mortar of the census below formed out of reach and
sat 90 s without a throw. Look: moss skin with an acid core
(`LumpLook::Mortar`, `view.wgsl` case 6), which its core light brings up
to a saturated green, apart from the plants' muted greens.

Fruit: `start_fruit` formed its lump, which takes every ordinary walker in
reach, a mini-boss's members too; in the knot census 3 of 14 mini-bosses
were broken that way (a hunter of 400 on run 1795038793 planet 4). It now
grows its lump from walkers in no lump (`World::grow_lump`), as a march does.

The same nine games with both (`mortar` lines in `census/`; the Mortar's
walk added after, so its one mortar here never threw):

| | hunter | leech | sapper | wanderer | mortar | all |
| --- | --- | --- | --- | --- | --- | --- |
| formed | 7 | 2 | 3 | 3 | 1 | 16 |
| reached squad (<20 units) | 5 | 1 | 1 | 1 | 0 | 8 |
| came within 30 units of its errand | 0 | 0 | 2 | 0 | 0 | 2 |
| got to its errand | 0 | 0 | 2 | 0 | 0 | 2 |
| throws (mortar) | 0 | 0 | 0 | 0 | 0 | 0 |
| took food | 0 | 0 | 0 | 0 | 0 | 0 |
| stalled 10 s or more | 1 | 1 | 1 | 0 | 0 | 3 |
| stood still 10 s+ (seconds) | 12 | 14 | 19 | 0 | 0 | 45 |
| swallowed by a march | 0 | 0 | 0 | 0 | 0 | 0 |
| ended Broken | 3 | 0 | 2 | 2 | 0 | 7 |
| ended Spent | 0 | 0 | 0 | 1 | 1 | 2 |
| ended Fizzled | 0 | 0 | 0 | 0 | 0 | 0 |
| still alive | 4 | 2 | 1 | 0 | 0 | 7 |

- Reached the squad 6 of 14 (knot census) to 8 of 16; sappers at their wall
  1 to 2; no boss swallowed by a fruit or a march.

Scott's three planet 4 maps with every mini-boss a mortar (`set=boss_kind=5`,
`forced` lines), the walk in:

| | hunter | leech | sapper | wanderer | mortar | all |
| --- | --- | --- | --- | --- | --- | --- |
| formed | 0 | 0 | 0 | 0 | 7 | 7 |
| reached squad (<20 units) | 0 | 0 | 0 | 0 | 1 | 1 |
| throws (mortar) | 0 | 0 | 0 | 0 | 48 | 48 |
| stalled 10 s or more | 0 | 0 | 0 | 0 | 0 | 0 |
| ended Broken | 0 | 0 | 0 | 0 | 4 | 4 |
| ended Spent | 0 | 0 | 0 | 0 | 1 | 1 |
| ended Fizzled | 0 | 0 | 0 | 0 | 1 | 1 |
| still alive | 0 | 0 | 0 | 0 | 1 | 1 |

- 48 throws from 7 mortars; 3 walked first (44 to 51 s), one of them never
  found a shot and fizzled when it regathered on its front with no walkers
  there. Four ended Broken, mostly by throwing themselves away.
- What the throws cost the player does not show in three games: soldiers
  at the last row (290 s, or the end) were 61, 65 and 33 against 38, 75
  and 19 in the knot census,
  whose bosses differ too. Tuning (`lob_every`, `shell`) waits on play.

Clips (`mortar/`, zoom 3.5 and 8, run 1795038793 planet 4, `set=boss_kind=5`):
`mortar-wide.mp4` (92-112 s, 2x) and `mortar-close.mp4` (97-107 s, 2x). Page: https://claude.ai/artifact/RPtr4iteMNYdKhbRbp12xH (copy `mortar/page.html`).
