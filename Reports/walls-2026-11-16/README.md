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

## The owner's two picks (11-16 14:24Z and 14:25Z)

Asked on the page and in the thread at about 06:35Z, recommendations first;
the owner took both recommendations on the cards: **doors** (not gates
that open, not climbing over) and **Thicken on a standing wall** (not a
width chosen when placing, not a run-shop unlock).

Separate question noted on the page, still open: soldiers can't shoot over
their own walls (nor can turrets behind them).

## Built: doors and Thicken (11-16)

**Doors** (`Game::door`, `Cmd::Door`, `doors.rs`). A door is a wall
building with `Building::door` set. Its cells are the world's wall cells,
so the slime can't step on them and bites them at a wall's pace (same
health, same bite), and shots stop at them (`World::clear`); to soldiers
they are ground: `Game::doors` marks a standing door's wall cells, and
`Game::ground` (open or door) replaces `World::open` in `standable`, the
walking map (`standable_cells`), and the walk's, steer's and squad
anchor's checks (agreed with the levelgen and squad-movement threads,
11-16 08:14Z and 08:17Z). On the build page (G, the Door button) a door is
drawn like a wall: drawn along a standing wall it snaps onto its line,
takes its thickness and home side and is cut at once (its cells are
already there; rubble in its stretch is rebuilt); across a gap it is a
ghost builders raise. It costs as a wall that long. Drawn as a timber
grate (`Ground::Door`, `view.wgsl`): hard terrain, lower than a wall, with
bars a cell apart; a first slate blue read as water in the film's lake.

**Thicken** (`Game::upgrade` on a wall, the wall's card, U). Each level
writes one more band of the wall's width on its home side at once (2, 4,
6, 8 cells at the default width, `WALL_LEVELS` 4), each at the wall's first
price; soldiers standing in the new band step out to its far side first;
doors cut into it thicken with it, and a door's own card thickens its
wall. Thick walls are picked by their whole footprint
(`nearest_building`).

**Film** (`hand=door`, planet 1 seed 7, same ring as above): the ring
stands at 0:12, a door is cut at 0:16 in the segment facing the core with
ground both sides, the squad is ordered out at 0:18 (6 of 9 outside by
0:20, all 9 by 0:22) and back at 0:34 (all in by 0:38), and the door's
segment and two each side are thickened twice at 0:43 (the ring's cells
646 to 1000). The slime surrounds the ring by 2:50; by 2:49 it has eaten
192 of the 1000 cells, and no walker is inside. Raw: `raw/door.txt`; clip
`page/door.mp4` by `door_clip.py`.

**Tests**: `doors.rs` (a door is wall to the slime and shots and ground to
soldiers; a soldier walks through a wall-to-wall wall only with a door;
Thicken grows bands on the home side, at the first price, to the top
level, and the door with it; a door has a wall's health), `view.rs`
`a_door_draws_as_a_timber_grate`, and the window's sparse ground refresh
now cuts doors (`the_sparse_ground_refresh_matches_a_whole_one`).

**Cost**: nothing new runs every step but `Game::ground`'s check (an
empty test with no doors, one read with them); the door mask is rebuilt
only when a wall stands or thickens, and the walking map ORs it in when a
map is built. The shader reads one more ground word per pixel under hard
terrain. Not timed on the 1070.

## Turrets fire over walls, soldiers' guns do not (11-16 19:50Z)

Owner, 19:50Z: "What about turrets can fire over walls, lobbed fire goes
over walls, but regular guns shots do not?" Built as asked:

- **Turrets** see and shoot over built walls (map walls too; rock and
  water still block). `UnitIn::over` lets a unit's sight pass walls
  (`squad::sees_over`), `ShotIn::over` and `Bullet::over` let its shell fly
  on over them (`squad::over_wall`: a wall cell standing, not rubble), and
  `Game::lob_line` replaces `clear_line` for a turret's check. The GPU
  reads both as bit 31 (`squad::OVER`) of the unit's sight word and the
  shot's cell count; its `over_wall` is a cell with `WALL` in its ground
  word, which the `walls` pass clears on the break, so both engines skip
  the same cells. `set=turret_over=0` (`BuildRules::turret_over`, default
  on) is the old rule, for films.
- **Soldiers' guns** are unchanged: every shell stops at walls and doors. Owner, 2026-11-17 03:08Z, asked whether bullets should pass door grates: "No".
- **Lobbed fire**: thrown tools already land anywhere in reach and on open
  ground, line or no line (`Kit::use_tool`); now tested
  (`a_thrown_tool_goes_over_a_wall`). Gun shells (bomb, gas, germ) fly as
  bullets and stop at walls; a mortar-style card would be new work.

**Film** (`hand=turretring`, planet 1 seed 7, the pen's ring plus three
turrets 6 cells in toward the core at 0:14, 9 riflemen inside; on vs
`set=turret_over=0`, one run each): by 3:00 the turrets killed 5,546
walkers with the rule and 0 without until a breach (564 after it, from
2:35); the ring kept 646 of 646 cells with the rule and 335 without. The
riflemen killed 25 in both (before the ring). Raw `raw/turrets-on.txt`,
`raw/turrets-off.txt`; clip `page/turrets.mp4` by `turret_clip.py`.
Balance note, not a gate: a turret ring now holds against the default
slime with no wall lost in 3 minutes (the turtle question,
[defense payoff](../../wiki/world/larder.md)).

**Tests**: `a_turret_fires_over_a_wall_on_both_engines` (GPU twin on a
box of wall: the plain unit and shot stop, the `over` ones see and hit
past it, GPU as CPU), `a_turret_fires_over_its_own_wall_and_a_rifle_does_not`
(turret in a box: 18 of 19 shots hit outside, 39 killed; a rifleman in
it fires none), `a_thrown_tool_goes_over_a_wall`.

**Cost**: one more flag test per line cell walked for a turret's sight
and shots (CPU and GPU), nothing for soldiers. Not timed on the 1070.

## Builders whose home-side spot is rock (11-16, after #392)

The squad movement thread's new walk (#392) found `measure_wall_sides`
seed 8 (wall through the squad) leaving 3 builders past the wall: the
home-side spot beside each one's nearest point on the line was rock, so
`goal_raw`'s Build arm let them raise it from where they stood, on the
slime's side. Now they take the nearest open home-side spot along the
wall, a cell at a time either way. Seeds 1-8, main fe41620 against this
change: wall in front 0 and 0; wall through the squad 3 (all seed 8) and 0
(`raw/wall-sides-before.txt`, `raw/wall-sides-after.txt`).
