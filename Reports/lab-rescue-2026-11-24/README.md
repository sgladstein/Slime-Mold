# Lab: the rescue planet, a stranded squad to bring home (2026-11-24)

The Lab lane's nineteenth job, on the lab branch `claude/project-thread-2o4cic`
(code `a7c788e2`), not in the game. Brief from Direction 2026-11-24 17:24Z:
today every planet is the same job, kill the heart; prototype one planet
type with a different goal that comes from the model, for example a
stranded squad deep in the slime that the player must reach and bring
home, where the slime's response to the march is the danger. It should
take 4 to 8 minutes, be clearly winnable and lose in a way you can see;
smoke-test it with greedy and idle squads, film it at zoom 6, and say
whether it earns a place in a run.

Page: "Rescue Planet" (source in `page/`, listed in `Reports/pages.md`).

## The rule

`LAB_RESCUE=X,Y[,N,HIDE,OUT,ALARM,RING,WOUND,THICK]` strands N (3) of your
soldiers at (X, Y) from the first second. Every part is a rule the game
already has:

- **Hidden.** The slime cannot smell them (the toolkit's mask, smell 0)
  and they fire only at slime that comes near them (Defend). They take no
  orders until reached. The origin does not lunge at a soldier it cannot
  smell (new: without it the heart hunted them down from 50 cells in 2:20).
- **Dug in.** A ring of eight doors 7 cells round them, raised whole:
  soldiers walk through doors and the slime cannot, and it gnaws them as
  any wall. How long they hold is the walls' and the slime's own rule.
- **Found.** At HIDE (5:00), or the moment one of your soldiers comes
  within 8 cells, they reek (smell 3, the toolkit's reek) and fire
  freely. Reached, they join that soldier's squad and the slime's strikers
  turn on against the march whatever its size (ALARM).
- **Wounded.** Found, they walk at 0.3 of a soldier's pace, and a squad
  keeps to its slowest member (`Game::pace`, as it does for wading), so the
  march home is slow. A member who strays too far is left behind.
- **Won** when one of them stands within 14 cells of the larder (the lab
  scores it as a wipe); **lost** when all of them are dead. The heart's
  own win still stands.

The bot's hand `rescue@X,Y[,T]` plays greedy until T, then sends everyone
to them and home as squads. It costs one check a game second and one
lookup in each soldier's pace.

## Smoke tests (planet 1, run seed 1, lavapipe)

Stranded at 40,150, deep in the slime's south-west body, 37 cells from
the heart's crystal and about 300 from the landing; defaults otherwise.

| Hand | Reached | Ended | Outcome |
|---|---|---|---|
| rescue at once (5 soldiers) | 0:17, 3 of 3 | 1:19 | won, 3 of 3 home |
| greedy, then rescue from 1:30 | 1:40, 3 of 3 | 2:30 | won, 3 of 3 home |
| greedy, then rescue from 3:00 | 3:06, 3 of 3 | 3:20 | lost: all three left behind on the march and killed, 44 soldiers alive |
| greedy (never goes) | never | 4:40 | lost: the slime gnawed through the ring and killed them inside |
| idle | never | 6:13 | lost: found at 5:00, dead from 5:58 |

Clips (zoom 6, a frame a second, `raw/film.sh`, `raw/clips.py`): the win
from 1:30; greedy's loss as the ring gives way at 4:00 to 4:42; the late
march, two wounded left at the slime's edge as the army walks on.

Stranded by the heart's crystal instead (110,215, inside the rock ring)
they held 3:31 alone and every rescue won, from 1:11 to 2:58, losing no
one: too short and too safe. On planet 3 (the Trunk, 154,102) they held
5:00 and a rescue at 3:00 reached all three at 3:10, but lavapipe ran
planet 3 at under a game second a minute, so those games were stopped
unfinished.

## Tried and changed

- No hiding, free fire, no ring (v1): dead in 28 s alone; the rescue was
  home at 0:31.
- Hold fire inside a wall ring: slime walled in with them (dead by 0:03);
  cleared first, the origin hunted them (2:20); deep in the body, slime
  born or pushed inside by the walls' evict killed them in 10 to 40 s.
  Defend fixed all three.
- Plain walls: the rescuers reached them and they could not walk out (the
  last died in the ring at 3:06 with the squad home), so the ring is doors.
- Thickened toward the larder, the wall's bands filled the ring (dead by
  0:07); thickening is now outward, and off by default.
- Without the wound, the march home took 14 s.

## Card line (round one)

"Distress call: three of yours are dug in deep in the slime. Reach them and
walk them home at a wounded pace; the slime wakes when you find them, and
leave it too late and it breaks in."

## Recommendation (round one)

It earns a place as an occasional planet goal, one planet a run from
planet 3, rolled like a twist. It asks for different play from the heart:
a timed push into the body, then a slow escort the squad must keep
together, against a slime that turns on it. It wins and loses in ways you
can see, and every part of it is the model.

It does not meet 4 to 8 minutes on planet 1: an early rescue wins in 1:19
because the map is small and soldiers are fast. Length should come from a
deeper body and a longer walk on later planets, or two dug-outs to reach,
not from a clock. Before it goes in, time it on planets 3 to 5 on the GTX
1070, where lavapipe could not. Two rule changes go with it and touch other
lanes: the origin ignores soldiers it cannot smell (this also hides a
masked soldier from it), and wounded soldiers slow their squad. The heart
can still be killed while the dug-out holds, which makes the rescue an
either-or; a rescue planet could instead have no heart to kill.

## Round two: planets 3 to 5 (Direction 19:37Z)

Direction took it as an occasional planet goal from planet 3, with the
heart win dropped: the heart still stands and bites, but only the rescue
wins. Before it is built it must be a 4-to-8-minute planet on planets 3 to
5, with paired smoke checks timed on the GTX 1070. Code `233b0f8a` and
`a7b9204e`:

- **No heart win.** On a rescue planet the origin does not break.
- **More dug-outs.** `LAB_RESCUE_MORE=X,Y[;X,Y]` adds dug-outs on the same
  terms. The planet is won when one soldier from every dug-out has reached
  the larder, and lost when any one dug-out is wiped.
- **The hand.** `rescue[@T]` plays greedy until T. Then everyone goes to
  each dug-out in turn, then home. Once a dug-out is reached they move as
  squads, at the wounded's pace.

Three batches, 64 games on run seed 1, about 31 minutes of 1070 time.
There is one line a game in `raw/batches-1070.txt` (scripts
`raw/pc-rescue{2,3,4}.ps1`).

**Planet 3 (the Trunk) meets the brief with two dug-outs**: 154,102 and
42,139, the defaults (alarm on, ring one level, wound 0.3).

| Hand | Outcome | Ended |
|---|---|---|
| idle | lost | 5:06 |
| greedy | lost | 5:25 |
| rescue from 1:00 | won, 2 and 3 of 3 home | 3:57 |
| rescue from 2:30 | lost, never reached | 5:21 |
| rescue from 4:00 | won, 1 and 3 of 3 home | 6:23 |

The losses come when the slime gnaws through a ring at about 4:00 or the
hiding ends at 5:00 and it closes in. The 2:30 rescue never reached either
dug-out, the same in both batches: the bot's army stalls on the way. With
one dug-out a rescue wins in 2:20 to 5:28. A second dug-out at 250,70 lay
on the greedy hand's farm and was reached by chance at 0:13, so it moved.

**Planet 4 (the Swarmer) can't be judged with this bot.** The greedy
hand's own squad is wiped at 0:43 on run seed 1 with no rescue at all. In
every rescue game the hand's army was dead before it could go, so greedy
and rescue@60, @150 and @240 played the same. Rings one level thick fell
at about 2:00 (idle lost at 1:59 and 2:34). Three levels held them to 4:30.

**Planet 5 (the Sporer) is winnable only by an early rescue, and only
without the alarm.** With the alarm on, every reached dug-out died on the
march home through 87,000 to 140,000 walkers (12 of 12), mostly beside
help. With rings three levels, wound 0.5 and the strikers not forced, a
rescue from 1:00 won at 2:27 (and at 2:28 with reek 1). Rescues from 2:30
and 4:00 still lost on the march, at 3:59 to 5:06. Greedy lost at 5:26 to
7:41.

### The two rule changes, as patches

Against main (made at `0e01b479`, still applying at `0c3b742c`), each applies alone, and together they pass
`cargo check --all-targets`:

- `raw/origin-unsmelled.patch` (Enemy): the origin does not lunge at or
  count a soldier it cannot smell (smell 0). That covers the toolkit's
  mask too.
- `raw/wounded-pace.patch` (Squad): `Soldier::pace` (1 whole) multiplies
  `Game::wade`, so a wounded soldier walks slower, and a squad keeps to
  its slowest as it does for wading.

### Recommendation after round two

Build it for planet 3 first, as the Trunk's rescue planet: two dug-outs
deep in the body, alarm on, one-level rings. It plays 4 to 6.5 minutes,
is won by going early or late, and is lost when a ring falls. For planets
4 and 5 it needs more than the dug-outs' numbers:

- **Planet 4** needs a hand that survives the Swarmer before it can be
  timed. The owner's play or the strategy bot would show it.
- **Planet 5** needs the alarm off and a shorter way home, for example
  dug-outs nearer the body's edge. As it is, the march through the Sporer's
  body kills the wounded after 2:30 every time.

Card line for planet 3: "Distress call: two squads are dug in deep in the
Trunk. Reach both and walk them home at a wounded pace. The slime wakes
when you find them, and its rings won't hold past five minutes."
