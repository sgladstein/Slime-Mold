# One wait a game step, and the census on the GPU (2026-11-12)

The technical lead review of 2026-11-12 (Claude Doc
https://claude.ai/artifact/PutmsG8iWZpDv6DBuBW5tT, *Scale and the 1070
budget* and *Two copies of every rule*) found three gaps in the Larder
world's GPU game (`gpu::larder_world::GpuWorld`, the window's world):

1. Each game step made about four blocking read-backs, each on a new
   staging buffer (`gpu::read_back`: copy, submit, wait for the whole
   queue), and the game has no frame time on the GTX 1070 against W2c's
   gate of a frame under 10 ms p95.
2. Every balance call came from the CPU census; nothing checked that the
   GPU engine's games land on the same spread of outcomes.
3. The CPU mirror's open ground never saw a wall worn to rubble.

The coordinator gave this thread the first two (relayed 04:35Z, from the
review's *New threads* section). This report covers what changed, the
checks, and the measurements so far. The 1070's numbers wait for the
owner's PC.

```
cargo run --release --bin larder-world -- bench=20 hand=farm [scale=8 walkers=100000] [stamps=0]
cargo run --release --example larderworld -- census gpu=both seeds=1-16 threads=1
cargo test --release --locked --lib the_readout_matches -- --nocapture
```

## What changed

**One packed readout a frame.** `GpuWorld` keeps one buffer of about
8 KB (plus 2.5 KB with the mould grid) that the frame's own command
buffer fills after its last pass: the squad's unit and shot outputs, each
shot's kills, every pile's run and sums, and the wall counters. A frame
then waits once, for that one submission, and creates no buffer. Before,
`frame` called `squad_out` (two read-backs: the whole 77 KB squad buffer
and the kills) and `world_counts` (the piles and the wall counters).
`GpuWorld::frame_start` and `frame_finish` split the frame, so a caller
can draw while the GPU runs it; `frame` is the two in a row, as before.

**The engine counts the living walkers** (`Engine::record_count`, the new
`count` pass, number 34). `World::living` read every slot's energy and,
with the life cycle on, every `spec` word back once a game second: at
100,000 walkers' 1M slots, 8 MB through two more waits. The count pass
adds one atomic per living walker (a slot alive and not dead, the CPU's
`living`) into a spare word of the wall counters, which the readout
copies. A first version summed each workgroup through shared memory: on
software Vulkan its barriers cost 1.5 ms a game step (step frames' p50
16.5-16.8 ms against 15.0-15.2 ms without the pass); the atomic version
is within the noise of no pass (15.1-15.3).

**The mould grid in the same readout** (the integration thread's ask,
05:02Z: the campaign's positional goals and the player AI need it).
`World::mould_grid` on the GPU world returned None. Now the count pass
also sums each square's walkers holding a cell (`occ`) when a square is
set, into the words after the wall counters; the first ask for a square
reads `occ` back once and sums it on the CPU, and every frame after sums
it on the GPU. No new params row or binding: the world row's spare word
(`_w0`) is now `grid`, and the wall counters' buffer grows from 16 bytes
to 32 plus room for squares down to one dish cell.

**The mirror follows rubble.** When the readout's broken-cell count moves,
`GpuWorld` reads the agar back once and updates the mirror, so soldiers
can cross rubble as they do on the CPU. A cell breaks once, so this read
is rare.

**A bench for the 1070** (`larder-world -- bench=SECS`): the window's
frame without a window, so it can run over Remote Control with nobody at
the screen. Each frame does what the window's does (the game's steps
through the same accumulator, the ground, marks and HUD, the view and
canvas recorded and submitted), then waits for that frame's submission as
a present with vsync off would. It prints the frame time's p50, p95 and
max, split into the game's steps (which wait on the GPU inside),
preparing, recording and the GPU's tail, for every frame and for the
frames with a step; the read-backs and waits a step; and with timestamps
the GPU's ms for a step's ticks and its drawing.

**The census on the GPU** (`examples/larderworld.rs`, `gpu=1` or
`gpu=both`). The hands are generic over the world, and `gpu=both` plays
every (seed, hand) game on both engines, prints each engine's census
(means, ends, the paired sign tests with Holm), then compares the engines
hand by hand: medians, the median paired difference, and a two-sided
Mann-Whitney test with Holm over the hands. The two engines run the same
rules but add floats in other orders, so a seed's two games part within
seconds; the spread is what must agree. `crystals left` on the GPU uses
the CPU mirror's `pile_full`, which builds the same walls.

## Checks

- **The readout gives what the read-backs gave.** A new library test
  (`the_readout_matches_the_read_backs`) wraps the GPU world in a world
  that checks every frame: the frame's squad output equals `squad_out`
  read back after it, the crystals equal `world_counts`, the living count
  equals `alive`, and the mould grid equals the sum of `occ` read back;
  the frame made no read-back of its own and at most one wait. A squad
  attacks on a small race world (scale 1, 2,000 walkers) for 45 s, on the
  plain mould and on the Sleeper (the life cycle, dying asleep): 675
  frames each, all equal; 46 and 30 shots that killed, 2,835 and 4,787
  touches, up to 513 walkers dead at once on the Sleeper, which the count
  leaves out.
- **The game twin tests still hold** (`mouldgpu`'s
  `the_larder_game_runs_on_the_engine_as_on_the_cpu` and
  `a_designed_map_plays_on_the_engine_as_on_the_cpu`): the same banked
  food, the moulds within the 8% bound at every 5 s.
- **The CPU census is unchanged.** The census before and after the edit,
  on the CPU, seeds 1-3, cap 120 s, hands idle, random, farm, rush, wall
  and claim: all 18 rows and the summary identical, bar the ms a tick.
  (A first version took `crystals left` from the start's charge and
  changed one row, random seed 3, 0.992 to 0.986: a wall laid on crystal
  cells lowers `pile_full`. It uses `pile_full` again.)

## Measurements

All on software Vulkan (llvmpipe, 4 cores), so the times are relative
only; the 1070's are to come.

**The bench, before and after** (`bench=20 hand=farm`, the Fork, seed 1,
scale 2, 5,000 walkers, 300 game steps at 60 frames a second; `main` at
19a8765 with the bench patched in, [`bench-on-main.patch`](gpu-readout-2026-11-12/bench-on-main.patch)):

| | read-backs a step | frames with a step, step p50 ms | every frame, p50 / p95 ms |
|---|---|---|---|
| main | 4.13 | 15.02, 15.25, 15.29 | 27.0-27.2 / 42.1-43.0 |
| this change | 0.07 | 15.15, 15.07, 15.48 | 26.9-27.5 / 42.0-43.7 |

Three pairs run alternately, timestamps off on both
([`raw/bench-pairs.txt`](gpu-readout-2026-11-12/raw/bench-pairs.txt)): the
step is higher in 2 of 3 pairs, within the noise. Earlier pairs with the
first count pass (shared-memory sum) read 16.5-16.9 against 14.8-15.4.

The 0.07 left is the window's own ground refresh (`refresh_ground`: the
crystals' charge once a game second, and the walls' health on a walled
map). On software Vulkan a read-back is a memory copy, so cutting four to
one saves nothing there; on a discrete GPU each is a submit and a fence
round trip, which is what the 1070 run will show. The every-frame time
on software Vulkan is mostly the view's drawing at 1280 x 800 (the GPU
tail, 24-25 ms p50), which the 1070 does in a fraction of a millisecond.

**A first look at the two engines** (`gpu=both seeds=1-2 cap=60
hands=idle,farm,claim`, before the full census): scores within 0.002 to
0.007 of each other, the same verdicts, but the GPU mould ended bigger
in 6 of 6 pairs (1,840 to 1,744, 3,036 to 2,919, 3,011 to 2,868, and
+3% to +5.5% in the others). Six pairs from two seeds are not
independent; the full census below decides whether it is a bias.

## Files

- `src/gpu/larder_world.rs`: the readout, `frame_start` and
  `frame_finish`, the mould grid, the mirror's rubble, and the test.
- `src/gpu/mod.rs`, `src/gpu/engine.wgsl`: `COUNT`, `record_count`,
  `grid`, `WSTAT_BYTES`, `reads`.
- `src/bin/larder-world.rs`: `bench`.
- `examples/larderworld.rs`: `gpu=`, `compare`, `mann_whitney_p`.
- [`gpu-readout-2026-11-12/`](gpu-readout-2026-11-12/): the bench patch
  for `main`, the census merge script (`merge.py`, for seed slices run
  apart), raw outputs (`raw/`: the bench pairs, the CPU census on `main`
  and after).
