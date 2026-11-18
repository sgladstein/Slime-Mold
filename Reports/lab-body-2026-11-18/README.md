# Lab: a slime that holds together when it moves away from its body (2026-11-18)

The Lab lane's fourth prototype, on the lab branch `claude/project-thread-4tc75h`
only (code `22f468a`), not in the game. Brief from Direction 2026-11-18 15:58Z,
from the owner's play notes that day: "Often, especially during raids the slime
feels more like a bunch of individual walkers instead of a single slime unit",
and "Seeing the individual walkers doesn't bother me when I am attacking a big
slime blob and the walkers surge towards me, but when they all leave the blob
and most of what I see is individual walkers that is problematic". Aim: slime
that moves away from its body (a raid, a breakaway, a walk to a new crystal);
leave the surge at the squad as it is. Measure: how much of the moving slime is
out of contact with the rest.

Page: "One Body", https://claude.ai/artifact/T3rN3s95quKLVKLzdCpNk4 (source in `page/`).

## From the model

Most walkers steer by trail: three sensors read the trail the crowd lays and
the walker turns toward the strongest. That is what holds the body: walkers
crowd onto the trail they share, and the trail is where they are.

A walker that moves away from its body is almost always a *scared* walker in
the GPU engine's walk pass: one marching on a raid (`raid_at_sensor`), one
roaming to food (`food_at_sensor`, the specks), or one in a panic. These follow
a distance field to their goal and read no trail at all, so nothing ties one
marcher to the next. Each takes its own path down the field at its own pace,
and a raid of about 1,700 walkers crosses the map as a cloud of single dots.
The surge at the squad (`goal_unit`, the charge at soldiers in reach) is left
alone. The engine's slowing for lone walkers (`P.lone`) skips scared walkers.

So the question is what a marcher should read of the walkers round it.
The CPU tries of 11-12 ([one body](../one-body-2026-11-12.md)) put a
tether and a pull on every walker and split the mould into hunks
(`Reports/dead-ends.md`, `body_pull` and `tether`); these act only on
marchers, so the body at rest is untouched.

## Built (GPU engine only; plant words 71-74)

1. **Wait for the crowd** (`LAB_WAIT` n, `LAB_WAIT_R` 2, `LAB_WAIT_FLOOR` 0.1):
   a marcher counts walkers in the 5 x 5 square round it and walks at
   (count - 1) / n of its pace, never under the floor. The front waits for the
   rear.
2. **Tether** (`LAB_TETHER` k): a marcher does not step into a cell that
   touches fewer than k other walkers (its own cell left out); it turns round
   instead. A marcher that would lose contact stays with the crowd.
3. **Both** (`LAB_WAIT=6 LAB_TETHER=2`).

`LAB_BODY_LOG=1` logs once a game second, from the walker grid: walkers, loose
(a walker with at most one walker among its 8 neighbours), alone (no
neighbour), in specks (pieces under 10), the largest 8-connected piece and the
number of pieces of 10 or more.

## Results

Planet 1, run seed 1, greedy hand, `run=1:1 mode=squads hand=greedy`. Raid 1
leaves at 0:59 with about 1,736 walkers. Counts are of all walkers (about
12,000); the body at rest has few loose walkers, so the loose count is mostly
the moving slime. Raw logs in `raw/`.

| Arm | Loose at 0:30 (calm) | Loose at 1:03 (raid out) | Loose at 1:10 | Largest piece at 1:03 | Killed by 1:16 | Slime at 1:16 |
|---|---|---|---|---|---|---|
| Base | 407 (3.2%) | 365 (3.0%) | 216 | 96.7% | 1,212 | 11,761 |
| Tether 1 | | 261 | | 97.2% | | |
| Tether 2 | 192 (1.5%) | 114 (1.0%) | 121 | 97.7% | 799 | 11,908 |
| Wait 6 + tether 2 | 207 (1.7%) | 134 (1.1%) | 137 | 97.9% | 835 | 11,881 |

Wait 6 alone (first run, alone counts only): 213 walkers alone at 1:03 and
270 at 1:16, against 249 and 81 without; it slowed the raid but left it
spread, and it lingered longer. Tether 1 cut loose walkers by under a
third (261 at 1:03).

What the clips show (zoom 6 at 165,95 and zoom 12 at 195,70, 0:56 to 1:16; a
calm breakaway walking to a new crystal at 100,120 zoom 6 and 105,125 zoom 12,
0:15 to 0:35):

- **Base.** The raid is a cloud of single orange dots crossing the river to
  the squad. The calm breakaway trails a long stream of dots behind its blob.
- **Tether 2.** Loose walkers fall to a third. The breakaway walks as a blob
  with a short tail. But much of the raid never leaves: marchers at the body's
  edge turn back when the cell ahead would leave them alone, so fewer reach
  the squad, and those that do come as a column of small blobs. Kills fall a
  third (799 against 1,212), and the slime is 147 walkers bigger at 1:16.
- **Wait 6 + tether 2.** Much the same as the tether alone; waiting adds
  nothing the tether does not.

Frame cost (not timed; llvmpipe is never timed): the tether reads the 8
neighbours of the target cell, once per marcher step; the wait reads 25 cells
per marcher. Only marchers pay, about 1,700 walkers during a raid and a few
hundred otherwise, against 12,000 walkers in the pass.

## Recommendation

**Tether 2, with the raid's size retuned.** It is the one mechanism that
makes moving slime read as a body: a third of the loose walkers, the
breakaway a blob, the body's surge at the squad untouched. Its cost is that a
tethered raid sends fewer walkers out, so it is a softer raid; if the raid's
weight should stay, raise the raid's walker count (a tuning number) rather
than loosen the tether. Not for the game yet: it goes to Direction to place.
To bring it in, Enemy would move the rule into the walk pass of
`src/gpu/engine.wgsl` with its word, film it on planets 1 to 3, and check that
a raid still reaches the squad.

Not built: a pull toward a moving piece's centre (the 11-12 dead end's
re-test condition for align), and marchers that lay and follow their own
trail.
