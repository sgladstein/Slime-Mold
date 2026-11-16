# Walls: slime eating through, thickness, and doors (2026-11-16)

The owner, 2026-11-16 06:06Z: "Start work exploring how soldiers get
through our own walls. So we need to build doors or can we just jump them
or something else? Also we should be able to build 2-4x thick walls, not
sure if it should just be available but cost 2-4x or it is upgraded option.
Also slime should not be able to go over walls but they should be able to
slowly eat through. This should just use the real physics simulation
damage if possible."

Page: https://claude.ai/artifact/TQvqcvpKwK1EBFiqrhPW11 (copy in `page/`).

## What the game did

- **Slime cannot cross a standing wall.** A wall cell is not agar, a
  walker's move checks only its landing cell, and its longest step (2 fine
  cells in a surge or panic lunge, at scale 2) is shorter than the
  thinnest player wall (`Rules::wall_width` 2 dish cells, 4 fine cells).
  In every run below no walker was inside the ring until it was eaten
  through. (At scale 1, the census size, a 2-cell wall is 2 fine cells and
  a lunge could in principle hop it; not measured.)
- **Slime already eats walls in the real simulation** (`BigDish::bite_walls`,
  both engines since W1a/W1b): a blocked walker takes `wall_bite` (1) off
  the wall cell it tried to enter, of `wall_hp` (400); at 0 the cell is
  rubble, which is ground (agar, wet, scent open). No timer.
- **Why slime seemed to go over walls:** the view drew rubble as hard
  terrain, a low wall with the wall's own outline drawn over the slime
  (`view.wgsl` `ter_pack`). A 2-cell ring three-quarters eaten still looked
  whole while slime crossed it (`page/truth.mp4`, one run drawn both ways;
  the simulation is the same, only the shader differs).
- Soldiers cannot cross walls at all (not standable), and walls stop sight
  and shots both ways.

## Measured: a ring round a squad, 2, 4 and 8 cells thick

`larder-world look=DIR hand=pen mode=squads map=planet:1:7
set=wall_width=W at=...` on llvmpipe (main f1be524 plus this branch's
`pen` hand). Ten soldiers walk 0.3 of the way from the landing to the core;
at 12 s a 16-sided ring 12 cells in radius stands whole round them
(`Game::raise`), and they hold. The default slime (shape C) surges and
raids at them. Each game second: the ring's fine cells still wall, walkers
inside (mould grid, 4 x 4 bins), soldiers alive. One seed, one run each:
size, not a verdict. Raw: `raw/`.

| Wall | Cells | Left at 4:00 | Left at 8:00 | Slime inside from |
|---|---|---|---|---|
| 2 | 643 | 27% | (run ended at 4:00) | 2:45, 90 s after first contact (1:15) |
| 4 | 1402 | 81% | 23% | 6:31, about 290 s after first contact (1:42) |
| 8 | 3200 | 95% | 76% | not by 8:00 |

Twice as thick lasted about three times as long: the slime bites only the
face it touches. Raids 1-4 came at 1:19, 2:06, 2:32, 3:17 in every run.

## Built here: rubble drawn as ground

`view.wgsl`: rubble is no longer hard terrain (`ter_pack` returns open for
it; it is left out of the soil-height, plant and hard-terrain branches), and
its base colour is gravel, `RUBBLE_RGB` (52, 47, 41) times 0.75-1.25 by a
hash of the fine cell, in `cell_mat_c` and `ground_cell` alike, so the
slime is drawn over it as over any ground. `page/rubble-fix-356.png`: the
2-cell ring at 3:56, the holes full of slime. Test
`slime_on_rubble_is_drawn_and_rubble_is_ground` (fails on main: the band
drew as dark rubble over the gold). `look::mark_far` still counts rubble as
hard, which only means pixels near it take the full terrain path (correct,
slightly slower); not timed on the 1070.

Also: `Rules::set` takes `wall_width` and `wall_cost` (`set=wall_width=4`
in the window), and the `pen` hand films a ring.

## Open: the owner's two picks

Asked on the page and in the thread (11-16 ~06:35Z), recommendations first:

1. Getting through: **doors** (a stretch of wall soldiers walk through and
   the slime treats as wall, eaten at the same pace; touches only which
   cells are standable), gates that open while soldiers pass (slime can
   pour in; needs a wall-lifting step on both engines), or climbing over
   any wall (changes path costs, which the levelgen and squad-movement
   threads own and are reworking).
2. Thickness: **Thicken upgrade on a standing wall** (2 to 4 to 6 to 8,
   grows on the home side), a width chosen when placing (cost by cells), or
   a run-shop unlock.

Separate question noted on the page: soldiers can't shoot over their own
walls (nor can turrets behind them).
