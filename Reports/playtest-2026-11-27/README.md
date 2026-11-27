# The owner's run of 2026-11-27: still too easy, and why planet 2's slime starved

The owner, 2026-11-27 05:29Z: "Better but still too easy. Planet two was
legit hard (although for selecting the hardest planet option it still
should have been harder, at first I thought I was crushing it and then the
slime destroyed me and had me on the ropes it had full control of the map
and all crystals but one and then it just died starved itself. not sure
what happened."

Sources: his play logs on Scott-Desktop,
`play-logs/sent/2026-11-27T04-26-21Z-planet-1-*` to `...T05-13-52Z-planet-6-*`
(run 1795753581, build 3627 `b16e089`, the star map's first six steps), and
headless runs of step 2's Hard planet (`run=1795753581:2h`) on the software
GPU in the cloud.

## The run, step by step

| Step | Planet | Pip (curve place) | Time | Stars | Soldiers lost | Most soldiers | Energy left |
|---|---|---|---|---|---|---|---|
| 1 | Solar, Drifter | landing (1.0) | 6:07 | 2 | 11 | 76 | 8,019 |
| 2 | Yarra, Sprawler | Hard (3.5) | 11:44 | 1 | 63 | 39 | 152 |
| 3 | Quarivor, Trunk | Safe (1.6) | 4:07 | 3 | 0 | 8 | 509 |
| 4 | Thamar, Trunk | Safe (2.1) | 3:22 | 2 | 6 | 27 | 4,034 |
| 5 | Ulme, Swarmer | Fair (3.6) | 3:55 | 3 | 0 | 51 | 5,420 |
| 6 | Athonith, Swarmer | Hard (5.7) | 8:04 | 3 | 0 | 63 | 7,143 |

"Soldiers lost" is the sum of each second's drop in live soldiers. The pip
is read from the map's depth in the log header (`map=planet:D:...`) against
`run::curve_depth`.

Against his 11-26 run (planets 1 to 4 each under 3:30 with one soldier lost
between them; planet 5 the only fight), the start is harder: step 1 took
6:07 and 11 soldiers, and step 2 was a real fight. Steps 3 to 6 were not:
6 soldiers lost in all, and step 6, a Hard pick, cost none against a slime
of 131,000 that peaked at 169,000. He built no turret all run, and left
5,000 to 8,000 energy unspent on steps 1, 5 and 6.

## Step 2: what happened

From the log's seconds (slime walkers, soldiers, pumps):

- 1:30 to 4:50: the Sprawler (landed at 19,108) sat at its ceiling,
  about 28,700 (its home crystal, crystal 0, 579,200 charge, held alone),
  while the squad grew to 39 soldiers and 12 pumps.
- 5:01 to 5:49: the squad cut it from 25,500 to 3,500 (21,700 killed).
- 5:51: a mortar mini-boss formed in the crowd (752 members). By 6:07 the
  squad was down from 24 soldiers to 7; the slime took the pumps (12 to 0
  by 6:37).
- 6:00 to 7:31: with 5 to 7 soldiers left the slime grew from 3,900 to
  4,935 and stopped there, flat for about 15 s.
- 7:33 to 7:50: his last soldiers drained crystal 6 (20,787 of its
  28,320). The slime fell from 4,935 to 2,027 by 9:11 while he killed only
  about 1,060, then he shot the rest by 11:30.

The flat top at 4,935 is the slime's birth ceiling (`larder_world/ceiling.rs`):
the charge in the crystals it **holds** times its landing density (0.0495
walkers a unit here). 4,935 walkers is about 100,000 charge, two or three of
the small crystals. It did not hold its home crystal again after 5:49, though
it was the only slime on the map: a crystal was held only once walkers
covered a tenth of its cells (91 of crystal 0's 905), kept down to a
twentieth (45). A Sprawler is threads, and keeps only a few tenths of a
percent of its walkers on a crystal at once (an unshot 25,000 on this
planet: 76 to 105 of crystal 0's cells covered between 2:20 and 2:40), so a
slime of 4,000 could never cover 91. Its ceiling stayed at its small
crystals; when he drained one, the ceiling fell under its size, its upkeep
rose (`ceiling_upkeep`) and it starved down to what was left.

### The same thing on the software GPU

Step 2's Hard planet (`run=1795753581:2h`, idle squad kept alive,
`wither=0` so the cut-off threads' own withering does not hide the
ceiling), the slime's home crystal and everything within 160 fine cells of
it killed at 1:40 (19,331 walkers; 5,234 left). Same seed, before and after
the change below:

| At | Before: slime, ceiling, crystal 0 held? | After: slime, ceiling, crystal 0 held? |
|---|---|---|
| 2:10 | 4,201, 3,991, no (32 cells covered) | 4,424, 25,421, yes (44 cells) |
| 2:20 | 2,185, 2,076, no | 2,776, 25,027, yes |
| 2:40 | 1,321, 1,255, no | 2,172, 23,837, yes |
| 3:20 | (not run) | 4,082, 27,800, yes, and 4 crystals held |

Before, it let crystal 0 go at 2:10 with 32 cells covered, its ceiling fell
under it, and it starved at 1.5 times its upkeep on to 1,321. After,
it keeps the crystal with 44 cells, its ceiling stays at about 24,000, and
from 2:40 it grows back. With withering on (the game's setting) the same cut
kept crystal 0 either way (72 cells at 2:40) and the slime still shrank,
6,120 to 4,762 by 2:40: cut-off threads withering, as `wither` is meant to.

## The change: an unguarded crystal needs only a share of the slime

`CEILING_HELD_SLIME` (0.5% of the slime's walkers to take a crystal) and
`CEILING_LET_GO_SLIME` (0.25% to keep it), at least `CEILING_LEAST_CELLS`
(8), cap the cover a crystal needs when the squad does not guard it: no
live soldier within its gun range of the crystal's rim, and no building of
the squad's on it. A slime of 25,000 needs at most 125 cells, more than the
91 its home here needed anyway, so a whole slime plays as before; one shot
down to 4,000 takes an unguarded crystal with 20 cells and keeps it with
10. A crystal the squad stands by or has a pump on still needs the tenth.
The landing's density is computed as before. Rejected: the floor on every
crystal, guarded or not (a squad clearing a crystal would have had to clear
its fringe down to a quarter of a percent of the slime).

## Hard planets harder

`Pip::offset` Hard +1.5 to +2.5: step 2's Hard planet now reads curve 4.5
(it was 3.5), step 6's 6.7 (was 5.7). A Hard planet reaches the curve's top (8,
the home world's numbers) from step 9, where it did from step 10. The star-map report's first retune was +2
([star-map-2026-11-26](../star-map-2026-11-26/README.md)); his step 6 Hard
cost no soldier, so it goes further.

## Not changed, for the next decision

- **Big armies.** He peaked at 76 soldiers on step 1 (the soldier price's
  climb, `SOLDIER_STEP`, is off on planet 1) and 51 to 63 on steps 5 and 6.
  The one threat that hurt was the mortar landing in a crowd of 24.
- **Unspent energy** on steps 1, 5 and 6 (5,000 to 8,000), and no turrets.
- **The star map and planet text** (his ask in the same message) go to
  Look and interface.

## Round 2, the same night: harder from planet 2

The owner, 06:54Z, after #870: "How much harder did you make everything?
Probably not enough. Make it harder!" Three of Direction's numbers, from
planet 2 on (planet 1 unchanged):

- `run::BLOWS` (an orange walker's blows): 1.25, 1.5, 1.75, 1.75, 2, 2.5,
  3 on planets 2 to 8 to 1.75, 2.25, 2.75, 2.75, 3.25, 3.75, 4.5. With
  the levels' +0.5 a level on top. Step 6's Hard planet (curve 6.7 since
  #870) reads about 3.6 where his step 6 read 2.3.
- `run::RAID_SIZE`: 0.15 to 0.3 by planet to 0.2 to 0.4.
- `BossRules::for_curve`: a mini-boss every 110 s on planet 2 to 35 s on
  planet 8 (was 150 to 45), the first at 60 to 30 s (was 90 to 40), and
  the mortar from planet 3 (was 4): it was the one threat that cost him
  soldiers.

Not changed: the soldier's price climb (an army of 76 on step 1). The
heavier slime and the mortar meet a big army first; the price is the next
lever if armies still win.
