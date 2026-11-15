# An organic start, not a disc (2026-11-15)

The graphics thread, from the three-agent planet review
(https://claude.ai/artifact/Kcy1AoquWJjJ3EciyA3KxP, item 10): "every start is
a perfect gold disc".

Page: https://claude.ai/artifact/Ttr5AuoZ4jH6QYGpWWtJrz (`page/`).

## Cause

`BigDish::place_walkers` draws each start walker at angle `a * TAU` and
distance `sqrt(b) * r`: an even disc with a hard round edge, the same
outline on every map and seed.

## Change (`src/mould/big.rs`, `BigParams::start_shape`)

In the Larder world (`race_params`, and on the GPU through `Settings`), each
start takes its own outline, `start_shape(seed, cell)`:

- Its radius by angle is 1 plus harmonics 2 to 7, each with a strength of
  up to 0.5 / (k - 1)^0.7 and a phase from the seed and the start's cell.
  The outline is kept between 0.3 and 1.4 of the disc's radius and scaled
  to the disc's area. Starts listed twice at one place (the garden's) draw
  one outline.
- A walker in the outer half may sit up to 45% further out or in (a ragged
  rim), and the distance is `b^0.45`, not `sqrt(b)`, so the middle is denser
  and the edge frays.
- No walker of an organic start is placed within 12 dish cells of the
  squad's landing. Uncapped, the Fork's start reached up the middle channel
  onto the squad at 0:00.

Other dishes keep the disc (`start_shape` off by default).

A first try at about a quarter of these strengths (lobes up to 0.22,
harmonics 2-5, rim 0.3) still read as a disc at the whole map's zoom.

## Measures

- New test `a_start_is_an_organic_shape_not_a_disc` (open ground, 500
  walkers): the 90th-percentile reach of the walkers in 24 sectors varies by
  0.216, 0.287 and 0.207 of its mean on seeds 1-3, against 0.023, 0.027 and
  0.024 for the disc. Every walker is placed, and two seeds differ.
- Smoke, idle hand, seed 3, 60 s: Fork, mould 29,566 before and 30,137 now
  (squad dead by 60 s in both); planet 2:1:1, 31,767 and 32,200 (squad
  alive in both).
- Films (`raw/mk.py`): `larder-world look=DIR hand=idle seed=3 map=M
  at=0.2,...,12` for planet:2:1:1, planet:2:2:1 and fork, before and now.

## Tests that assumed the disc

The start moves where the mould first meets the squad, so four
chaos-sensitive fixtures moved:

- `a_squad_firing_on_the_fleeing_mould...` and `a_squad_laying_craters...`
  (CPU-GPU twins, seed 3): with the outline, the engines parted past their
  bounds (killed 1,239 GPU against 1,056 CPU; 1,725 against 1,564 walkers at
  25 s). They keep the disc, like the game-layer rules those twins already
  turn off. On main they also part by 10 walkers at 5 s.
- `a_gassed_mould_spares_the_squad...`: the gas claim holds where the
  mould is thin, and the outline moved the squad to another edge (squad
  health 380.4 against 381.5). This fixture keeps the disc too.
- `gpu_garden_keeps_each_sides_birth_reserve_as_the_cpu`: the player's
  mould now ends 994 against 1,007 between the capped and free runs, from
  conversion where the two moulds meet. The tolerance went from 5 to 15.
- `gpu_garden_shots_spare_the_players_mould`: a shot from 30 cells out
  stopped on the enemy start's thin rim and killed none, so it is now fired
  from 4 cells out.
- `every_shot_fires_the_next_card...`: the empty-bank blast check runs 20 s,
  not 10.
