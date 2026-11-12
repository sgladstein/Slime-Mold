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
8 KB (plus the mould grid when asked: 10 KB at scale 1 in squares of 4
dish cells, 40 KB at scale 2) that the frame's own command
buffer fills after its last pass: the squad's unit and shot outputs, each
shot's kills, every pile's run and sums, and the wall counters. A frame
then waits once, for that one submission, and creates no buffer. Before,
`frame` called `squad_out` (two read-backs: the whole 77 KB squad buffer
and the kills) and `world_counts` (the piles and the wall counters).
`GpuWorld::frame_start` and `frame_finish` split the frame, so a caller
can draw while the GPU runs it; `frame` is the two in a row, as before.

**The engine counts the living walkers** (`Engine::record_count`, the new
`count` pass, number 36 after the one-body port took 34 and 35). `World::living` read every slot's energy and,
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
the GPU's ms for a step's ticks and its drawing. The window (and so the
bench) now takes any engine setting as a key over the race world's
(`gpu::set`: `tier=sleeper`, or `alarm_keep=0.98 alarm_weight=-1` for the
hit avoidance the Larder thread is making the default; on software Vulkan
at 5,000 walkers that pair moved the step frames' step p50 from 15.16 to
15.64 ms, one pair; the tech lead review's +10 ms a tick at 100k is for the
1070 run to check).

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

**The two engines over 32 seeds: no bias** (`gpu=both seeds=1-32
cap=120`, the five default hands, at 9c18a0a, 320 games;
[`raw/census-both-s1-32-cap120.txt`](gpu-readout-2026-11-12/raw/census-both-s1-32-cap120.txt),
summed by `merge.py`). Deciding measures, named before the run: each
hand's score, CPU against GPU (Mann-Whitney two-sided, Holm over the five
hands), and the mould at the end, GPU bigger or smaller by seed (sign
test, Holm over five).

| hand | median score CPU / GPU | median paired difference | Mann-Whitney p | mould: GPU bigger / smaller | sign p | median size ratio |
|---|---|---|---|---|---|---|
| idle | -0.214 / -0.210 | +0.003 | 0.82 | 14 / 18 | 0.60 | 0.975 |
| random | +0.133 / +0.123 | -0.000 | 1.00 | 15 / 16 | 1.00 | 0.999 |
| farm | +0.291 / +0.285 | +0.001 | 0.87 | 14 / 18 | 0.60 | 0.996 |
| rush | -1.674 / -1.629 | +0.008 | 0.97 | 15 / 17 | 0.86 | 0.996 |
| wall | +0.281 / +0.286 | +0.000 | 0.98 | 16 / 16 | 1.00 | 1.002 |

Nothing survives Holm, and every plan-against-control sign test holds on
both engines with the same verdicts (farm and wall over idle 32 to 0 on
both; rush under random 0 to 32 on both). The idle mould alone, 60 s, 32
seeds: at scale 2 the GPU's bigger in 19 and smaller in 13 (p 0.38,
median ratio 1.018), at scale 1 15 and 12 (p 0.70, 1.000)
([`raw/idle-both-*`](gpu-readout-2026-11-12/raw/)). So the first look's
3-5% was six correlated pairs, not a rule difference, and the tech lead's
bisection was not needed. The CPU census stands in for the GPU game on
these hands.

**One open case: the Rim mission** (the campaign thread saw the mould
leak out of the Rim's north-east gap by 5:00 in the GPU window, seed 1,
claim hand, where the CPU census held). `examples/campaign.rs -- mission=3-the-gaps
hands=claim seeds=1-4 gpu=both` (the second PR's option,
[`raw/rim-claim-both-s1-4.txt`](gpu-readout-2026-11-12/raw/rim-claim-both-s1-4.txt)):
the CPU meets the goal on 4 of 4, the GPU on 3 of 4. On seed 1 both
hold (GPU at 696 s with a peak leak of 0.050, CPU at 701 s, 0.058), so
the census on the GPU holds where the window was seen to leak. Seed 2
fails on the GPU (peak leak 0.168, capped at 900 s, 0 stars; the CPU held
at 600 s). The GPU's peak leak is higher on 3 of 4 seeds (sign p 0.63):
four seeds call nothing, and the campaign thread runs 32.

## The step in halves, and the grid once a game second

The second PR (the integration thread's queue: after the controls PR
#102, before the buildings thread).

**`Game::step` in halves.** `begin_step` is the squad's turn (walking,
banking, aims and shots) and hands the frame to the world
(`World::frame_start`, which may return without waiting); `finish_step`
takes the frame's results (`World::frame_finish`, waiting only if they are
not in) and does what they do: kills, harvest, touches, theft, the core,
the end. `step` is the two in a row, so the census, the AI and the replay
are unchanged. A command (`Game::apply`) finishes a running step first,
so a round replays the same however the window splits it. The window
begins a step and finishes it on a later frame as soon as its readout is
in (`Game::step_ready`, a non-blocking poll), or at once when the next
step is due, so it draws while the GPU runs the step. The squad toolkit's
calm touch (#104: gassed walkers come off a unit's touch) is read in
`finish_step`, after the frame, as `step` read it. The campaign's tracker
in the window keys on `Game::settled_ticks` (a running step's start), so
it reads whole steps, as the census does.

**The mould grid once a game second** (the tech lead review's ask,
2026-11-12, for the campaign's positional goals). At the campaign's
squares of 4 dish cells the grid is 40 KB at scale 2, 160 KB at scale 4
and 655 KB at scale 8; the first PR summed and copied it every step once
asked. Now the GPU sums it on the frames that reach a whole game second
(`GRID_EVERY`, 15 game ticks), and on the first frame after a new square.
The CPU world's grid is the tick's own, so a goal read off it lands up to
a second later on the GPU; the campaign reads it once a second anyway. The
wall counters now have room for squares down to 4 dish cells at every
scale (`GRID_MIN`): room down to a game cell at scale 8 fitted 40,960
squares, not the 163,840 of the campaign's grid on the 2048 x 1280 dish,
so every ask there would have read all 10.5 MB of `occ` back.

**Checks.**

- `a_step_in_halves_plays_as_a_whole_one`: 600 steps on a world that
  holds each frame's results until asked, a third finished at once and
  the rest when the next began, eight commands at five ticks (attacks, two
  buys, a wall, a stance) and sleep gas on the mould at the first soldier
  it touched: the same log, world hash, soldiers, bank, kills and shots as
  `step`; a running step's `settled_ticks` is the tick it began on.
- `the_readout_matches_the_read_backs` now checks the grid on the frames
  that sum it (46 of 675: the first frame, then each game second) against
  the cells read back, and that the grid is never a game second old.
- The game twin test (`mouldgpu`) checks the GPU's grid against
  `BigDish::mould_grid` itself: equal at the start (the same walkers), and
  at 5, 10, 15 and 20 s 0.91, 0.90, 0.88 and 0.78 of the mould in the same
  squares on both (bound 0.7; the engines' floats part, most at 20 s, just
  after the first walkers starve).

**Measurements** (llvmpipe in the cloud, so the split's ordering and
costs, not GPU timing; the 1070 run is queued). `larder-world -- bench=20
hand=farm stamps=0` (fork, seed 1, scale 2, 5,000 walkers, 300 steps in
1,200 frames), main at a90c71e against the split, two pairs in turn
(`raw/bench-split-pairs.txt`):

| | main (whole steps) | split | split, `grid=4` every step |
|---|---|---|---|
| waits a step | 1.00 (14.8, 15.1 ms) | 0.00 (0.05 ms) | 0.00 (0.05 ms) |
| step part, step frames, p50 ms | 15.26, 15.38 | 0.41, 0.39 | 0.43, 0.41 |
| record part, step frames, p50 ms | 0.12, 0.12 | 15.27, 14.47 | 15.32, 14.84 |
| frame p50, every frame, ms | 27.12, 27.38 | 27.48, 27.23 | 27.47, 27.28 |
| frame p95, every frame, ms | 43.63, 44.06 | 44.07, 41.41 | 44.30, 42.78 |
| prepare p95, every frame, ms | 0.24, 0.25 | 0.26, 0.24 | 0.27, 0.25 |
| at 20 s: mould, harvested, shots, killed | 2701, 1953, 3, 4 | the same | the same |

The step no longer waits: its 15 ms leave the step and land in the next
submit (`record`), because llvmpipe runs compute on the CPU inside the
submit, so the frame totals are equal here (a discrete GPU returns from
the submit and runs the step while the window prepares and records the
picture; that is what the 1070 bench measures). The game is the same one
to the count. The grid in squares of 4 asked every step costs nothing
measurable here (step-frame p50 within 0.7 ms either way, the pairs'
own spread), and reads back once, on the first ask (20 read-backs in 300
steps against 19 without it; the rest are the window's ground, once a game
second). Summed once a game second, it copies 40 KB a second at scale 2,
where the first PR copied 40 KB a step.

Two bench fixes came out of the first pairs (`raw/bench-split-first.txt`):
the split's rows printed mould 3170, as the bench planned on a step still
running (the last step's bank and crystals) and printed before the last
step finished (with both fixed it matches main to the count); and its step-frame prepare p95 was 14.9 ms against
1.3 ms, the window's ground read-back (once a game second) waiting for
the running step. The bench now plans on `settled_ticks` and settles at
the end, and the ground's timed refresh waits for a frame between steps,
up to a second late (a new wall still reads at once).

## Files

- `src/gpu/larder_world.rs`: the readout, `frame_start` and
  `frame_finish`, the mould grid, the mirror's rubble, and the test.
- `src/gpu/mod.rs`, `src/gpu/engine.wgsl`: `COUNT`, `record_count`,
  `grid`, `WSTAT_BYTES`, `reads`.
- `src/bin/larder-world.rs`: `bench` (`grid=BIN`), the window's step in
  halves, `settle`, the ground's refresh between steps.
- `src/mould/larder_world.rs`: `World::frame_start`, `frame_finish`,
  `frame_ready`; `Game::begin_step`, `finish_step`, `step_ready`,
  `settled_ticks`, and the halves test.
- `src/bin/mouldgpu.rs`: the game twin test's mould grid check.
- `examples/larderworld.rs`: `gpu=`, `compare`, `mann_whitney_p`.
- `examples/campaign.rs`: `gpu=0|1|both`.
- [`gpu-readout-2026-11-12/`](gpu-readout-2026-11-12/): the bench patch
  for `main`, the census merge script (`merge.py`, for seed slices run
  apart), raw outputs (`raw/`: the bench pairs, the CPU census on `main`
  and after, the 32-seed census on both engines, the idle runs, the Rim
  mission's four seeds, the split's bench pairs).
