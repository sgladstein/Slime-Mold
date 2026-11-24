# Lab: it grows in its sleep, a choice for the Sleeper (2026-11-24)

The Lab lane's eighth prototype, on the lab branch `claude/project-thread-2o4cic`
only (code `3508d9d4`, on main of 11-24 after the origin), not in the game.
Brief from Direction 2026-11-24 05:16Z: take Enemy's runner-up of planets 4
to 8, the Sleeper (planet 6), which is free to farm before it wakes, and
prototype one change, from the model, that makes the time before it wakes a
choice rather than free food; about two hours, the same kind of page and
counts as scar tissue.

Page: "Sleeper Dreams" (source in `page/`, listed in `Reports/pages.md`).

## From the model

The Sleeper's walkers doze (`sleep_rate`) and lie dormant: a dormant walker
pays no upkeep, does not eat, does not breed and does not move. It wakes on
the crystals' scent (`wake_scent` 1), or on alarm (0.3 in the run) once its
rest timer (`sleep_rest` 600 steps) is out; a waking walker lays alarm, so
one shot wakes the body round it. Asleep, it dies slowly (`sleep_death`
1e-4 a step). The run's card: "It wakes at your first shots, and all of it
comes".

So while the squad farms with its fire held, nothing it does reaches the
slime, and the slime only shrinks. Measured on planet 6, run seed 1 (a
squad of 4 pumping the western crystals, fire held): 137,706 walkers at
landing, 129,764 at 0:30, 115,022 at 2:00. Waking it later meets a smaller
slime with more food in the bank: waiting is strictly better.

## The candidate: it grows in its sleep

`LAB_DREAM=P`: each step a dormant walker buds a dormant child beside it
with chance P, at no energy cost, under the slime's birth ceiling and into
the same free cells a birth takes. The bud is born asleep with a full rest
timer, and only on ground off the crystals' scent (under half
`wake_scent`): on the scent a bud wakes at once, and its wake's alarm woke
the body round it in waves of up to 100,000 walkers with no shot fired
(first try, P 3e-3 and 1e-2). Off from the game's first second, so the
landing is the off arm's.

The card would read: **"It grows while it sleeps. Every minute you farm,
there is more to wake."** The choice: wake it early and small, poor; or
farm long and rich, and wake a bigger slime. The picture is the data: the
sleeping body spreads over the bare ground round it.

Picked: P 3e-3 (30 times the death rate a step; 1e-2 measured too). Costs one hash a sleeping walker a step in `breed`, and one
scent read a bud.

## Results

Planet 6, run seed 1, bank 3,000, lavapipe, one seed. Hand `wake@T,334,452`
(lab only): pump the crystals 40 units or more west of the post with fire
held until game second T, then buy soldiers with the bank (up to 16) and
walk to (334, 452), the slime's western crystal, on free fire.

**Asleep** (fire held, no shot fired; walkers, awake + asleep):

| Time | Off | P 3e-3 | P 1e-2 |
|---|---|---|---|
| 0:01 | 137,706 | 137,706 | 137,706 |
| 0:30 | 129,764 | 146,518 | 167,743 |
| 1:00 | 122,130 | 153,285 | 180,640 |
| 2:00 | 115,022 | 162,813 | 195,239 |

The buds land on the body's edge (inside it every cell is full), so it
grows out over the bare ground round it, slowing as the edge fills.

**Woken at 0:30 against 2:00** (food farmed by the wake; walkers at the
wake; the squad at 1:00 after the wake):

| Arm | Wake | Farmed | Walkers at the wake | Squad 1:00 later | Killed by then |
|---|---|---|---|---|---|
| off | 0:30 | 499 | 129,764 | 16 | 19,827 |
| off | 2:00 | 12,320 | 115,022 | 16 | 24,805 |
| P 3e-3 | 0:30 | 499 | 146,518 | 14 | 22,283 |
| P 3e-3 | 2:00 | 12,320 | 162,813 | wiped at 2:56 | 25,781 |
| P 1e-2 | 0:30 | 499 | 167,743 | 15 | 22,349 |
| P 1e-2 | 2:00 | 12,320 | 195,239 | wiped at 2:49 | 23,766 |

Off, the late wake meets an 11% smaller slime with 25 times the food, and
the squad stands. With the slime growing asleep at 3e-3, the late wake
meets an 11% bigger slime (42% bigger than off's), and the 16 soldiers are
wiped 56 seconds in; at 1e-2, 16% bigger (70% over off's) and wiped 49
seconds in. The early wake costs the squad little in any arm (14 to 16
standing a minute later).

Between 1:00 and 2:00, with no shot fired, a wave woke up to 65% of the
slime in the off arm and at 3e-3 (it slept again by 2:00), and was starting
at 2:00 at 1e-2. The off arm has it, so it is not this change; its cause is
not found.

**Frame cost** (lavapipe, bench 40 s, planet 6 seed 1, hand `pumps`, the
two arms run side by side, every pass timed alone, median of 30 steps;
lavapipe is for checking, not timing): `breed` 1.710 ms a run off against
1.735 with P 0.003, `born` 0.393 against 0.388, every pass 149.7 against
143.1 ms (noise at this size); the frame's p50 254.7 against 254.2 ms.

## Recommendation

**Build it as the Sleeper's thing**, at P 3e-3 with buds off the scent:
the time before it wakes becomes a choice the player can see (the sleeping
body spreads toward the farm) and name. Before it lands:

- Enemy: P by play, one seed here. The growth is bounded by the birth
  ceiling (208,000 on this planet, 1.5 times the landing) and by the body's
  free edge, so it slows by itself.
- The card (Direction): the old card's "all of it comes" stays true; the new
  line is the growth.
- Look: nothing needed; the dormant body's spread is the picture. A sleeping
  bud could carry a faint new-growth tint so the spread reads before it
  wakes (untested).

## Not tried

Waking by a slow clock (it wakes by itself at a time you can see); pumps
that hum (lay alarm at a rate the pump's level sets, so a bigger farm wakes
it sooner); a bud's chance that follows the food pumped.

## Raw

`raw/run6.sh` (the run: env, hand, frames, camera), `raw/m6.py` (walkers
awake and asleep a second, from the dump's `life.txt`, and the log's squad
lines).
