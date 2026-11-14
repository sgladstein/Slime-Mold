# Building: raise from reach, stay on your side, queue builds (2026-11-14)

The owner, 2026-11-14 08:10Z: "I should be able to queue builds. Right now
soilders center themselves on the build spot but it would be best if they
just started building when the got in range. If I am building a turret
enemy, I want to be able to keep my soliders on one side of the building
turret. review what else should be improved".

## What changed (PR 1)

- **Builders stop once in reach** (`BuildRules::reach`, 6 dish cells from
  the building's middle, or from a wall's line). A builder walks straight at
  the site and stands the moment it is that near; it no longer walks to a
  place in its squad's sunflower round the ghost. Too near (on the
  footprint, or on a wall's line) it steps out to the edge on its own side.
  Raising counts up to reach plus one elbow-room separation, so a builder a
  neighbour pushed back still raises. `Game::build_band`, `Game::at_site`,
  the `Order::Build` arm of `Game::goal_raw`.
- **Shift + clicks queue builds** on one builder when nobody is selected:
  the soldier sent for the first ghost (`Cmd::Send`) queues the next ones
  (`Cmd::Queue`) instead of each ghost taking another soldier. With
  soldiers selected, Shift already queued (`order_all`); now the window's
  `builders` does both.
- **Squad mode** (`mode=squads`): a build placed without Shift sends the
  controlled squad as a squad (`Cmd::SquadOrder(q, Build)`), which also ends
  a move it was on; before, the squad's move took the build order back the
  next step. A squad sent somewhere keeps what is queued (only steering and
  a new squad move drop it), and a queued order waits until the squad has
  arrived, so Shift + a build while it walks is raised when it gets there.

## Measured

Test `builders_raise_from_reach_and_stay_on_the_side_they_came_from` (race
world seed 15, eight soldiers from the larder to a turret 30 cells toward
the core, raise 6 s), the same test run on main with reach 3 in its own
check:

| | main | this PR |
|---|---|---|
| builders past the ghost (far side) when it stands | 2 of 8 | 0 of 8 |
| nearest builder to the ghost's middle | 1.91 cells (on its 2-cell footprint) | 3.50 cells |
| mean distance | 7.62 | 7.68 |
| nearest builder when raising began | 8.85 cells | 7.89 cells |

Raising already began from about 9 cells on main (the old reach counted the
builder's sunflower place and elbow room), so the time to raise did not
change: in the clip below the three turrets start at 5.0, 6.25 and 8.0 s on
main and 5.0, 6.5 and 8.25 s here, and all stand by 6.25, 7.5 and 9.25 s
in both. What changed is where the builders stand while they raise.

`a_squad_sent_off_keeps_a_build_queued_for_when_it_gets_there` fails on
main (the queue is gone 3 steps into the squad's move) and passes here;
`shift_queued_builds_rise_in_turn_by_one_builder` passes on both (one
soldier's Shift queue already worked in the rules).

## Clip

`page/build-queue.gif`: `larder-world look=DIR hand=buildq
centre=256,178,6 at=2,2.25,...,12 screen=960x600` on llvmpipe, main
(68870d0 plus the same `buildq` hand) on the left and this PR on the right,
cropped 660x280 at 150,20 and scaled to 3/4 by `gif.py`. The Fork, seed 1:
eight soldiers at the larder are ordered at 2 s to a turret 28 cells toward
the core and Shift-queue two more beside it; the bank is topped up to 2,000
so all three are paid. Counts per frame in `t-lines-before.txt` and
`t-lines-after.txt`.

## Not measured

No census: this is a controls change (CLAUDE.md, build first). The player
AI and the census hands send builders the same way, so a building plan's
numbers may shift a little.
