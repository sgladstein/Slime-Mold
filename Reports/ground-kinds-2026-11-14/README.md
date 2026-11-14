# New ground kinds: deep water and bridges (2026-11-14)

Thread "Add new ground kinds, deep water first", from the owner's answer on
the procedural levels thread, 2026-11-14 17:02Z: "7) New ground. Yes!", with
"deep water stops soldiers, slime crawls slowly" from the same answers
(levelgen biomes plan) and "any maps should be beatable by any strategy".

## What was built

| Ground | Map G byte | Slime | Soldiers, buildings | Shots |
| --- | --- | --- | --- | --- |
| Deep water | `G_DEEP` 0x41: soil kind 4 + `G_WET` | step x `wet_step` 0.5 x `deep_step` 0.4 = 0.2 of plain; no birth there | `World::open` false: can't stand or build; paths go round | `World::clear` true: fly over |
| Ford | `G_WET` (unchanged) | x 0.5 | walk | fly |
| Bridge | `G_BRIDGE` 0x50: soil kind 5, dry | plain | walk | fly |

- Encoding agreed with the procedural levels thread (17:21Z). A build from
  before this reads deep water as wet (the wet bit) and a bridge as plain.
  Kinds 6-15 are free; this thread numbers them.
- Engines: `BigParams::deep_step` (CPU `big.rs`), and on the GPU a new
  ground row in `Params` (`GROUND_AT` 928; Params 928 -> 944 bytes; three
  words kept for later kinds). `engine.wgsl` multiplies the step after the
  wet step, as the CPU does, and skips a birth into deep water.
- Rules: `World::open` (soldier ground) now excludes deep water; the new
  `World::clear` (rock, walls and the rim stop it, water doesn't) is what
  bullets (`Game::fly`) and the line of fire (`Game::clear_line`) read.
- Maps: `map::standable(g)` is the one predicate for soldier ground (the
  levelgen thread's planet check calls it); `Map::cut_off` floods from the
  landing (4-neighbour) and lists crystals (within r + 8) and nests
  (within 8) the squad can't walk to. The editor (`larder-edit`) warns of
  them, of a landing in deep water, and has Deep (7) and Bridge (8)
  brushes; over either, wet paints a ford and rock plain rock.
- A sixth sandbox map, the River (`map=river`, in the menu's map list):
  a deep river across the middle, fords at x 170-229 and 794-853, a bridge
  at x 496-527 (`raw/make_river.py`).

## Checks

- `big::deep_water_slows_and_drowns_births`: a dish all deep water against
  one all wet (scale 1, 30 ticks): no births in deep water (some on wet);
  the first walkers' mean drift under 0.7 of wet's.
- `larder_world::soldiers_cross_deep_water_only_by_ford_or_bridge`: on
  `map::river` with only a ford, then only a bridge, a soldier sent across
  crosses inside the ford's or bridge's rows and is on open ground at every
  step; a bullet across the river flies its whole leg.
- `map::deep_water_cuts_off_until_a_ford_or_bridge`, editor tests.
- GPU twin (`mouldgpu` species/soil one-tick test, llvmpipe): the soil
  bands now include deep water (wet bit set): 1,053 living walkers in deep
  water, 0 walkers off, trail L1 0; two-species variant 1,040, 0 off.
- Full `cargo test --release`, clippy clean.

Frame cost: not timed. The GPU walk adds one read of the ground word it
already reads for wet; the birth check one compare; the CPU one branch per
walker step where the map has soil.

## Films (seed 1, shape C, llvmpipe)

[page](https://claude.ai/artifact/9kdKYHvn4wv3zPHvsbfLQy), source `page/`,
logs `raw/idle.log`, `raw/rush.log`, GIFs by `raw/make_gifs.py`.

- Idle, 150 s: the slime holds the south bank (walkers 25,323 at 15 s,
  25,063 at 150 s; crystals left 0.965) and reaches north only up the
  bridge in a thin tongue.
- Rush: the squad's path takes the bridge, the nearest crossing; 30 shots,
  528 walkers killed, the squad fell by 12 s on the slime's bank.

## Not yet

- The look: deep water draws as ordinary wet, so fords don't show. The
  graphics lead (17:15Z, via the coordinator) gave it to the terrain
  thread (#281): a deep-water branch on soil kind 4 inside the WET block of
  `cell_mat_c`, a bridge base on kind 5 and a plank height in
  `cell_height`. The editor's own colours already differ (`ground_rgb`).
- Player-built bridges, and an ability to jump water, are not built.
- `deep_step` 0.4 is a first number for play, not measured.
