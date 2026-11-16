# Walls leave the squad on its own side (2026-11-16)

The owner, 2026-11-16 01:45Z: "Walls are also problematic when building
because some soliders always end up on the wrong side, trapped with the
slime." Squad mode (the default).

## Why

Three causes, all in squad mode's building:

1. **Guards posted past the wall.** A working squad with slime near sends a
   third of itself to guard (`cover_one`), each posted two cells from the
   work on the slime's bearing. The work's point for a building was its
   `at`, which for a wall is its first end, so every post sat beyond the
   line. This alone put one soldier past the wall on every seed.
2. **Builders stood on whichever side they reached** (#255's "the side it
   came from"). A squad straddling the line, which is where a wall drawn
   through or next to it ends up, raised it from both sides and was split.
3. **A soldier shut in the footprint walked out to the nearest open
   ground** (`walk`'s `shut`), either side.

## What changed

- `Building::home`, a unit vector square to a wall's line, chosen at
  placement by `Game::wall_home`: the side with less slime when the two
  sides differ by `WALL_SLIME` (5) walkers within `WALL_LOOK` (8 cells, or
  half the wall's length); else the larder's (landing's) side; else the
  side most soldiers near it stand on. A majority vote first, before the
  larder, sent a straddling squad with its larger half.
- The `Order::Build` arm of `goal_raw` (paths.rs; the levelgen thread had
  nothing open there): a wall's builder that stands short of the home
  side's band walks to the home side of its nearest point on the line, if
  that point is open ground (else the old rule; levelgen's note that a goal
  moved off rock can land past the line).
- `cover_one`: a wall's guards keep `Order::Build` with `cover` set, so they
  stand with the builders on the home side, fire at the full rate and don't
  raise (`build_turn` skips them). The squad's work point for a wall is its
  middle. A row of posts behind the wall was tried first: a move's goal is
  the post plus the soldier's formation place (`place_at`), which put one
  guard back over the line, and the next item then held the wall at 99.5%
  for good on the Fork (the first in-game film).
- `build_turn`: a wall's last sliver (`WALL_WAIT` 0.995) waits while a live
  soldier stands within half its width plus half a cell, at most
  `WALL_WAIT_SECS` (4 s, timed from `Building::ready`), then closes anyway.

## Measured

`wall_side_tests::measure_wall_sides` (ignored; CPU race world, squad mode,
8 soldiers walked 20 cells toward the core, `CoverRules::near` 0 so guards
come out as by slime, a 20-cell wall square to the core's way): soldiers on
the core's side of the line 5 s after the wall stands, and soldiers in its
cells when it stands. Seeds 1-8, the same seeds on main (a119ce9 plus the
test file) and here. Raw: `measure-before.txt`, `measure-after.txt`.

| wall | main: past | fix: past | main: shut in | fix: shut in |
|---|---|---|---|---|
| 4 cells in front of the squad | 9 of 64 (1-2 every seed) | 0 of 64 | 0 | 0 |
| through the squad's middle | 35 of 64 (3-8 every seed) | 1 of 64 (seed 8) | 2 | 0 |

Both rows better on 8 of 8 seeds, none worse (sign test p = 0.008 each;
both survive a Bonferroni correction for two). The one soldier left past
the wall on seed 8 was not traced. Test
`a_squad_ends_on_its_own_side_of_the_wall_it_builds` holds seeds 1-4 at 0;
`a_walls_home_side_is_away_from_the_slime` checks the slime vote.

## Clip

`page/wall-sides.mp4`, `page/building.png` (10.5 s), `page/standing.png`
(15 s): `larder-world look=DIR hand=wallq mode=squads at=8,8.5,...,22
screen=960x600` on llvmpipe, main (a119ce9 plus the same `wallq` hand) on
the left and this change on the right, cropped 560x380 at 200,120 by
`gif.py`, then `ffmpeg -framerate 2.5`. The Fork, seed 1: eight soldiers
walk 16 cells from the landing away from the core; at 8 s a 30-cell wall
goes up through the squad's middle, the slime to the south. Counts per
second (`wallq t`) in `t-lines-before.txt` and `t-lines-after.txt`: main
3 of 8 past while it rises and 1 left with the slime for good after it
stands; here 0 throughout. The wall stands at 13 s in both.

Page: https://claude.ai/artifact/JsMqRiqL4vEqS1tSZerNnE

## Not measured

No census (CLAUDE.md, build first). Not timed on the 1070: `wall_home` runs
once a placement, `wall_occupied` only while a wall's last sliver waits,
and the goal change is a dot product a builder a step. The RTS mode's walls
get the same home side and builder rule; the campaign is frozen and was not
looked at beyond its tests passing.
