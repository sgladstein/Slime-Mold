# S1 Charges go round walls (2026-11-17)

The simulation-use plan's S1 ([plan.md](plan.md), draft 3, Part S: "A units
distance field. The raid's search (`raid_field_for`) with every soldier as
a source, on 2-cell blocks, rebuilt once a game second. Surges and panic
descend it instead of steering at a picked unit: walls bend the charge").
Page: https://claude.ai/artifact/5aGbKQjUbXnULqYqGnY7tB (source `page-s1/`,
blind; the key is below).

**Status: built on both engines and in the game as `Rules::unit_paths`,
off (`set=unit_paths=1`) until the owner's blind film.** With it, a
charge at a soldier behind a wall stops piling against the wall's slime
side (half as many on 7 of 7 seeds that piled at all) and flows round its
ends. Within 15 s about as many walkers reach the soldier as without
(more on 4 seeds, fewer on 4).
Watched 30 s the same holds: 4 more, 4 fewer, and the pile's difference
shrinks as the surge ends. The field changes the charge's shape, not its
reach.

## What was built

- `big::units_field_for(agar, w, h, units)`: the raid's Dijkstra on
  `RAID_BLOCK` (2-cell) blocks, costs 2 straight and 3 diagonal, from many
  sources at once (`walk_field`, now shared with `raid_field_for`, which is
  unchanged). A source in a closed block (a soldier standing by a wall
  or on a crystal's edge) starts from the nearest open block within 4
  blocks. Every block a walker can reach holds its walking distance to the
  nearest soldier; closed or cut-off blocks hold `u32::MAX`.
- `BigDish::units_field` (empty = off). A walker charging a unit (a surge,
  or panic aimed at a unit in reach) senses with the field instead of the
  unit's bearing: each sensor scores minus the distance under it, so it
  goes downhill, round walls and rock. Roaming, a lump and panic toward
  food are unchanged, and a walker on a block the field never reached
  steers straight as before.
- The GPU: the field rides in the dense words' first cell region (distance
  + 1, 0 = none), written by `Engine::write_units_field` when the game sets
  it; `goal_unit` marks a walker whose goal is a unit, and the sense branch
  reads `units_at_sensor`. The CPU mirror copies the field on inject.
- The game (`raids.rs`, `Game::time_unit_paths`): once a game second it
  takes where the frame's units stood (`raid_cell`, up to `UNITS_MAX`) and
  builds the field on a worker thread, as raids do; the next second it sets
  the finished field (`World::set_units_field`), so replays stay the same
  at any frame rate.
- A masked soldier (the toolkit's mask, smell 0) is left out of the
  sources, as the plan asks, so the charge's paths don't lead to it.

## Tests

| Test | Result |
|---|---|
| `charges_go_down_the_units_walking_distance_on_the_gpu` (llvmpipe; a grown dish with crystals and a long wall at x 70, every walker's news made stale so it panics at three units at x 100, one tick) | 4,155 walkers panicking, 0 off the CPU's; the same tick without the field moves 780 elsewhere, so a GPU that ignores it fails |
| `the_game_sets_the_units_walking_distance_once_a_second` | after 2.5 s the world holds the field: 8,496 of 10,240 blocks reached, 0 at the first soldier, 239 twenty cells east; off, none; every soldier masked, 6 at the first soldier (the nearest other unit) |
| `all_off_switches_on_together_play_on_both_engines` | `unit_paths` on with the other off switches, CPU and GPU |
| `cargo test --release --locked` with the GPU twins on llvmpipe, clippy | pass (595 lib, 41 window, 50 mouldgpu); clippy clean |

## Measure

`examples/wallprobe.rs`, raw lines in `s1-raw/`. Shape C on the CPU race
world (scale 2, 25,000 walkers), grown 63 s with the surges held. A unit
placed 8 dish cells clear of the slime, 30 from the rim, and a cup of
walls round it opening away from the slime: a back wall 8 cells each way
3 cells in front of the unit, and sides running 9 cells past it. A surge
at the unit 1 s in, watched 15 s. Three arms on the same grown dish: no
wall, the wall with charges steering straight (today), the wall with the
field. **Deciding measure, named before the run:** walkers that came within
3 dish cells of the unit in the 15 s (each counted once), the field against
straight, paired by seed. Also counted: walkers piled against the wall's
slime side at the end.

| Seed | Unit (x, y) | Came within 3: no wall | straight | field | Piled at the wall: straight | field |
|---|---|---|---|---|---|---|
| 1 | (188, 216) | 181 | 35 | **43** | 63 | **36** |
| 2 | (104, 108) | 255 | 84 | **107** | 54 | **43** |
| 3 | (216, 188) | 226 | 63 | **98** | 58 | **3** |
| 4 | (164, 232) | 175 | 63 | 57 | 62 | **51** |
| 5 | (204, 196) | 253 | 136 | 127 | 67 | **49** |
| 6 | (128, 212) | 96 | 87 | 74 | 0 | 0 |
| 7 | (116, 212) | 261 | 111 | 99 | 55 | **33** |
| 8 | (140, 236) | 181 | 15 | **75** | 57 | **0** |
| sum | | 1,628 | 594 | 680 | 416 | 215 |

- **Deciding:** more reached the unit with the field on 4 seeds, fewer on
  4 (sign p 1.0); 14% more in all, carried by seeds 3 and 8. Not a result.
- **The pile at the wall** halves (416 to 215; fewer on 7 of 7 seeds that
  piled, sign p 0.016). Steering straight, the charge presses against the
  back of the cup; on the field it splits round both ends.
- **The wall works either way**: both walled arms let through about a third
  to two fifths of what reached the unit with no wall.
- **Watched 30 s** (the same seeds, after the deciding run): came within 3,
  straight 674 against field 765, more on 4 and fewer on 4 again (seed 8:
  20 against 79; seed 6: 87 against 75). Piled at the end, 415 against
  267: fewer on 4, more on 2 (seed 1, 62 against 67; seed 4 by one), 2
  ties. With no wall, 1,975 came in 30 s against 1,628 in 15 (a fifth
  more); behind the wall about an eighth more either way. Most of a surge
  arrives in its first 15 s, so walkers going round the ends come late
  and few.
- **The page's key** (kept off the page): **A is the field, B is
  straight** (a coin toss). Its film is seed 1 watched 30 s
  (`wallprobe seeds=1-1 watch=30 look=DIR`, `page-s1/film.py DIR field
  straight`).
- **What it looks like** (the page, seed 1, blind): in one panel the
  surge fills the ground behind the cup and stops there; in the other it
  thins at the back and runs round the cup's ends towards the open side.

## Frame cost

Off: nothing (the game builds no field, the walkers' test is one empty
check). On: the build runs on a worker thread, 7.4 ms on planet 8 from 40
sources and 2.0 ms on planet 3 (`examples/raidpaths.rs`, this machine); the
frame copies the ground and starts the thread, as raids do (0.25 ms worst
of 20 on planet 8), and once a second uploads the field, one word a cell.
A charging walker reads three field words instead of the unit's bearing.
Not timed on the 1070.

## Left

- The owner's blind film decides `unit_paths`; then the plan's films on
  planets 1-3 (seconds to first contact, walkers reaching the squad per
  surge, pump health at 3:00) if it goes on.
- 1070 timing with the rest of the plan's passes.
