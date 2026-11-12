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
checks, and the measurements; the GTX 1070's are in *On the GTX 1070*
(the afternoon of 2026-11-12), with the squad's unit passes made parallel
and a step spread over the frames until the next.

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
only; the 1070's are in *On the GTX 1070* below.

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
- `a_replay_is_the_same_game_at_any_frame_rate` (the technical lead's ask
  on the PR): rounds played through the window's pacing at 60 and 23
  frames a second, with the same clicks at the same real times, land some
  clicks on other ticks (14 against 15, 171 against 172), and each replays
  in whole steps to its own world hash, bank and soldiers; the 60 fps log
  played back through the window's pacing at 23 and 144 frames a second
  gives the same log and hash. A command is stamped with the tick it lands
  on, not the frame it was clicked in.
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

## On the GTX 1070 (afternoon, 2026-11-12)

The technical lead's asks (14:38Z, 14:53Z): explain why the game's step
costs about 12 times the sandbox's 1.45 ms frame before optimising; give
the units and shots passes more than one thread; put each pass beside its
slot count, with a per-pass table at 5k, 20k and 100k walkers so the
owner can choose a size; spread a step if the replay test passes.

```
cargo run --release --bin larder-world -- bench=20 hand=farm scale=8 walkers=100000 spread=0 passes=30 [squad1=1]
cargo run --release --bin larder-world -- bench=20 hand=farm scale=8 walkers=100000            # spreading on
MOULDGPU_TEST_GPU=1 cargo test --release --locked --bin mouldgpu the_adapters -- --ignored --nocapture
```

`passes=N` times every GPU pass of N more steps alone, in the step's order
(`Engine::record_timed`); `squad1=1` runs the squad's three unit passes as
they were on `main` in the same build (`smell1`, `units1`, `harvest1`,
`Engine::squad_one`), so old and new are timed side by side; `settle=1`
waits for each step at once. Every bench now prints a machine line first
and last (free RAM, the GPU's memory and how busy it is, the other programs
on it), since the first afternoon round overlapped another thread's
campaign run with 0.5 GB of 8 free (every frame 3-5 times slower, one run
dead for want of memory, nothing in the output to say so). The rounds
before the owner freed memory at 15:28Z count only as "before memory was
freed" (`raw/bench-1070.txt`, `raw/passes-before.txt`,
`raw/passes-after.txt`); the tables below are round 3 (16:46Z, the PC
quiet: no other heavy process, 2.1-2.4 GB free, the GPU 0% busy at each
start) and round 4 (17:00Z, the harvest fix, the GPU 41% busy for a moment
at its first start).

**Every pass of a step** (the Fork, seed 1, farm hand, 20 game s; GPU ms a
step, median of 30 steps; "old" is `squad1=1`, the passes on `main`;
`raw/r3-timing.txt`, `raw/r4.txt`):

| Pass (runs a step at 5k / 20k / 100k) | 5k old | 5k new | 20k old | 20k new | 100k old | 100k new |
|---|---|---|---|---|---|---|
| units (1) | 0.744 | 0.014 | 2.716 | 0.022 | 10.453 | 0.069 |
| smell (1) | 0.264 | 0.014 | 1.007 | 0.023 | 3.884 | 0.053 |
| harvest (1) | 0.093 | 0.064 | 0.263 | 0.158 | 0.777 | 0.415 |
| trail (2 / 4 / 8) | 0.131 | 0.131 | 0.923 | 0.923 | 7.173 | 7.182 |
| scent (2 / 4 / 8) | 0.118 | 0.118 | 0.823 | 0.824 | 6.341 | 6.349 |
| sense (2 / 4 / 8) | 0.028 | 0.028 | 0.139 | 0.140 | 0.764 | 0.764 |
| walk | 0.016 | 0.016 | 0.060 | 0.060 | 0.337 | 0.337 |
| pilesum (4 / 8 / 16) | 0.021 | 0.021 | 0.097 | 0.097 | 0.646 | 0.647 |
| life, breed, born, eat, flow | 0.041 | 0.042 | 0.155 | 0.154 | 1.097 | 1.098 |
| draw (1) | 0.033 | 0.033 | 0.118 | 0.118 | 0.461 | 0.461 |
| **in all** | **1.489** | **0.481** | **6.303** | **2.528** | **31.878** | **17.37** |
| frames with a step, p50 ms, round 3 | 2.28 | 1.85 | 7.02 | 4.01 | 32.57 | 20.00 |
| the same, round 4 (the harvest fix) | | 1.47 | | 3.86 | | 21.13 |

Slots 50,000 / 200,000 / 1,000,000; living walkers at 20 s 2,690 / 5,241 /
15,723 (from `walkers` at the start); harvested 1,953 / 2,345 / 2,100, the
same old and new, on every run. The new column's passes are round 3's and
its harvest round 4's. Its totals at 5k and 20k are round 4's (round 3's
with the new harvest put in give 0.481 and 2.518); at 100k it is round 3's
18.282 with the new harvest put in (less 1.329, plus 0.415), as round 4's
100k run measured 18.705 with trail and scent about 5% slower than round
3's in the same passes (trail 7.58 against 7.18; the step's ticks 18.08 ms
against 16.75), which nothing in its machine lines explains (the GPU 5%
busy at its start, the same programs on it). Round 3's frames ran its
slower harvest (0.49, 0.80 and 1.33 ms a step).

**Cost follows living walkers, not slots.** `sense` takes 0.014, 0.035 and
0.096 ms a run at 50k, 200k and 1M slots: 2.5 and 6.9 times the 5k run,
against 4 and 20 times the slots and 1.9 and 5.8 times the living
walkers. Fitting `sense`'s run to slots and living walkers gives about
0.02 ns a slot and 4.8 ns a living walker, so at 100k the slots are about a
fifth of its run (0.02 of 0.096 ms). If the other walker passes split the
same way, the slots cost about 0.4 ms of the 2.2 ms a step that sense,
walk, eat, life, breed and born take, and sizing slots to twice the peak
or dispatching over a compacted live count would save at most that. Not
worth doing yet. Every bench here held the one-walker-a-cell limit, which
the engine drops (`Engine::made`) when the slots pass half the agar: 50k,
200k and 1M slots against the Fork's 146,258, 584,735 and 2,338,940 agar
cells at scales 2, 4 and 8.

**Why the game's step costs about 12 times the sandbox's frame.** Both run
120 engine ticks a second: the game 15 steps of 8 ticks at scale 8, the
sandbox (key 1, 100,000 walkers on 1024 x 640) 60 frames of 2. So:

- 4x is ticks a frame: the game ran a step's 8 ticks in the frame it
  started (16.75 ms of ticks, 0.95 of drawing) and the next three frames
  idle; the sandbox runs 2 ticks a frame.
- About 3x is the cost of a tick: 2.09 ms (16.75 / 8) against about 0.67
  ((1.45 - 0.1) / 2). Trail and scent are 1.69 ms of it (0.898 + 0.794 a
  run) on 2,621,440 cells, against the sandbox's 0.285 (0.128 + 0.157) on
  655,360: four times the cells, and about 1.5 times a cell, as the game's
  trail pass also cuts wounds and hits, marks reached piles, keeps craters
  and soil. Its walker passes are cheaper, with 15.7k living walkers to the
  sandbox's 100k (sense 0.096 ms a tick against 0.333).

Spreading takes back the 4x; the 3x is the world's size.

**The unit passes on a workgroup each, and a race only the 1070 showed.**
On `main` the squad's three unit passes ran one thread: `units` looped
over every unit's touch and sight boxes (10.45 ms a step at 100k), `smell`
over every unit's disc (3.88), `harvest` over every take box (0.78). Now
`units` has a workgroup a unit, its 256 threads summing the box and
reducing the densest block in shared memory (ties to the lower index, as
the CPU); `smell` and `shout` (the shots' alarm) are one workgroup of 256
in which thread `tx + 16 ty` owns every cell whose x is `tx` and y `ty`
mod 16 (`owned_from`), so discs that overlap add in the CPU's order with
no barrier; `harvest` is one workgroup that loads a take box's row into
shared memory (each cell by the thread its x picks, mod 256), lets thread 0
take in the CPU's order, and writes back. The first version had each thread
take a cell of a box by its place in that box, with a storage barrier
between units. It passed every twin test on lavapipe, and on the 1070 the
farm's game changed from run to run:

| `scale=2` farm bench on the 1070 | harvested, two runs |
|---|---|
| `main`'s passes (`squad1=1`) | 1953, 1953 |
| first version | 1478, 1785 |
| first version, `settle=1` | 1783, 1784 |
| `squad1=1 settle=1` | 1953, 1953 |
| owned cells (this change), 6 runs incl. `settle=1` | 1953 every run |
| `scale=4`: `squad1=1` / owned cells, 3 runs | 2345 / 2345 every run |

Two threads meeting on a cell by different box places lost each other's
adds, though a barrier stood between them; lavapipe runs a workgroup's
threads in an order that hides it. Two planted bugs in the owned version
(harvest walking a row backwards, `ty = li % 16`) fail the twin tests on
lavapipe. Harvest's first owned version walked all 256 slots of each
row's stretch on thread 0 (a take box is 25 cells wide): 0.493 ms a step
at 5k against the old pass's 0.093; walking only the row's own cells
(round 4) brought it to 0.064, 0.158 and 0.415, under the old pass at every
size. A dead end records the shared-box version.

**Spreading a step over the frames until the next** (`GpuWorld::spread`,
`Engine::record_share`): the window records a step at `frame_start` in as
many shares of its ticks as frames it lasts (4 at 60 frames a second); the
first goes with the squad's start, one more each frame (`frame_more`), and
the last carries the squad's end and the readout. Every share draws, so the
mould moves every frame. The passes and their order are the whole step's:
`a_step_in_shares_is_the_same_game` plays 150 steps in 1, 2 and 4 shares,
with mid-step read-backs, and matches every log line and the final hash, on
lavapipe and on the 1070. On the 1070, scale 8, 100k walkers, round 3:

| | every frame p50 / p95 / max ms | frames with a step p50 / p95 |
|---|---|---|
| `spread=0` (round 3's harvest) | 0.77 / 20.39 / 44.62 | 20.00 / 20.47 |
| spreading on, two runs | 5.50 / 6.62 / 47.37 and 5.48 / 6.62 / 50.36 | 5.73 / 5.93 |
| scale 4, spreading on, two runs | 1.24 / 2.64 / 13.54 and 1.24 / 2.65 / 13.32 | 1.41 / 1.79 |

W2c's gate, a frame under 10 ms p95 on the 1070, holds at 100k walkers
with spreading: 6.62 ms. The worst frame of each run (32-35 ms, nearly
all `prepare`, once a run) comes with or without spreading, not from the
step. Two costs come with it. A command while a step is in shares finishes the step first
(`Game::apply`), so that frame pays the step's remaining shares, as every
step frame did before. And the scripted farm hand, which looks at the game
every frame, sees it up to a step older: at scale 4 its buys land one step
later (steps 16, 31, 46 against 15, 30, 45; the command logs differ only so,
on lavapipe too), and it harvests 2343 against 2345. With the same
commands the game is the same.

**The twin tests on the 1070** (`MOULDGPU_TEST_GPU=1`, rounds 3 and 4,
`raw/r3-tests-summary.txt`): 27 of `mouldgpu`'s twins pass and 2 fail,
and `main` at 9f5e4a4 fails the same 2 with the same numbers, so neither
comes from this change. `the_life_cycle_matches_the_cpu_for_four_ticks`
fails on its first dish ("spores, one a cell"): 19 of 24,822 walkers off
after four ticks (within its bound of 1 in 1,000), but trail L1 1.2e-4
against 1e-6 (lavapipe: 0 off, trail 0). The fleeing-mould game twin
kills 908 on the GPU against 765 on the CPU (bound 15%); the CPU without
the alarm kills 1,488, so the alarm works on the 1070. The one-tick twins
pass with their bounds unchanged: 0 walkers off on most dishes, 1 off on
three (of 25,000 or 250,000), 44 of 25,000 on "heavy respawn" (35 limited
walkers aimed at a cell an unlimited one took first, inside its bound),
and the sealed wall dish bitten 4 times on the GPU against 0.

Why (`the_adapters_maths_beside_the_cpus`, 2^20 inputs; `raw/r3-maths.txt`):

| | GTX 1070: differ from the CPU's bits, worst | lavapipe |
|---|---|---|
| `cos`, `sin` | 91%, 1.1e-6 | 17%, 6e-8 (1 ulp) |
| `sqrt` | 17%, 4.8e-7 | exact |
| `exp`, `pow` | 82%, 45%; about 1 ulp | 87%, 66%; 1-2 ulp |
| `a * b + c` | fused into one FMA every time | never fused |
| a walker's step lands in another cell | 0 of 2^20 | 0 of 2^20 |

So a tick is almost always the same; over many ticks the drift grows, as
chaos does. Does it lean one way? `the_adapters_game_beside_the_cpus_over_seeds`
(the race world, scale 2, an idle squad, 30 s, both engines): over 32
seeds on the 1070, the GPU's trail is above the CPU's on 17, 15, 22, 18,
17 and 13 seeds at 5-30 s (median ratio 0.995-1.011) and its mould larger
on 15, 11, 11, 19, 12 and 15 (`raw/r3-drift32.txt`); the smallest p of the
twelve is 0.05, and none survives Holm. Exact results across machines
would need table maths for the trigonometry and the square root, and
arithmetic no driver can fuse; only replays shared between machines or
lockstep play need that, neither is planned, and HANDOFF's open decision 6
now carries the cost. The
integration thread's rule (2026-11-12): a PR that changes a shader or the
GPU's `Params` or bindings runs the twins on the 1070 before it merges
and quotes the machine line and the counts (the wiki's
[GPU sandbox page](../wiki/model/gpu-sandbox.md), *Twins on the 1070*).

**Left at 100k walkers:** trail and scent, 13.5 of the step's 17.4 ms.
Fusing them into one pass, or the CPU's `substeps` 4 (half the ticks;
the lead asked for a twin check of 4 against 8 before trading accuracy)
would be next if a bigger world needs it; with spreading, 100k already
runs at 5.5 ms a frame. The squad's `cut` pass writes `crater = max(...)`
from workgroups whose cones may overlap without an atomic: a race of the
same kind, not yet seen in a test (craters are on by default only in the
garden; a twin with overlapping cones would show it).

## Two GPU fixes after the 1070 work (evening, 2026-11-12)

**The crowd limit held only on small worlds.** `Engine::made` dropped the
one-walker-a-cell limit, without a word, when `max(walkers, capacity)`
passed half the agar. The race worlds give 10 slots a walker, so above
about 7,300 walkers at scale 2 (the Fork's 146,258 agar cells) the GPU
played without the limit while the CPU census kept it: the slime-enemy
thread's 25,000 walkers ran as ropes on the GPU and as a sheet with the
limit held. Growth needs no room of its own, since with the limit on a
birth into a full cell is refused on both engines (`born`; the CPU dish's
births), so the limit now holds while the start walkers are at most half
the agar (`gpu::crowd_holds`); `made` prints a warning when it drops, and
the bench's first line names the crowd rule as played. The shipped
settings held the limit before and after: on the noise world and the
five maps, at 1,250 to 100,000 walkers on scales 1 to 8, the plain mould
and every tier the GPU takes, the CPU's crowd and the GPU's agree
(`the_cpu_census_and_the_gpu_world_play_the_same_crowd_rule`, no GPU
needed; with the old rule planted it fails on the noise world at scale 2
and 25,000 walkers). It costs nothing a step.

**Overlapping cone craters raced.** `cut` wrote `crater[i] = max(crater[i],
steps)` from a workgroup a shot. With the cone, two shots whose strikes
share cells write different steps, and a read-modify-write across
workgroups can keep the smaller: the kind of race that lost harvest's
adds on the 1070 (*On the GTX 1070*). Binding 47 now holds the f32's bits
in atomics; a non-negative float's bits sort as the float does, so `cut`
takes `atomicMax`, exact in any order (the technical lead's fix). The
squad frame twin had no cones about different hits that overlapped (0
pairs); it gains two shots from slime cells three apart and asserts the
overlap (1 pair). Planted on lavapipe, `atomicStore` (the last shot wins)
puts 3 of 448 crater cells off, and `atomicMin` fails at once. The trail
pass's countdown now loads and stores through atomics each tick once
craters are on; its cost on the 1070 is to come.

**Left, from the technical lead (17:16Z), none needed yet** with 100k
walkers inside the frame budget: trail plus scent (13.5 of the step's
17.4 ms at 100k) are diffusion and bound by memory, so in order of
payoff, a 16 x 16 tile with a one-cell halo in shared memory (each cell
read once, not nine times); scent on a grid of half the resolution (it is
smooth, and steering reads only its slope; a rule change, so it would
reset the censuses); both fields packed into one buffer read. Each needs
its twins re-checked.

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
- `src/gpu/engine.wgsl`, `src/gpu/mod.rs` (afternoon): `units` a
  workgroup a unit, `smell`, `shout` and `harvest` on owned cells
  (`owned_from`), the old passes as `smell1`, `units1`, `harvest1`
  (`Engine::squad_one`), `record_share`, `record_timed`, `time_passes`;
  `src/gpu/larder_world.rs`: `spread`, `frame_more`, `flush`, `profile`,
  the shares test; `src/bin/larder-world.rs`: `passes=`, `settle=`,
  `squad1=`, `spread=`, the machine line; `src/bin/mouldgpu.rs`: the wide
  units in the squad twin, `the_adapters_maths_beside_the_cpus`,
  `the_adapters_game_beside_the_cpus_over_seeds` (`DRIFT_SEEDS`).
- [`gpu-readout-2026-11-12/`](gpu-readout-2026-11-12/): the bench patch
  for `main`, the census merge script (`merge.py`, for seed slices run
  apart), raw outputs (`raw/`: the bench pairs, the CPU census on `main`
  and after, the 32-seed census on both engines, the idle runs, the Rim
  mission's four seeds, the split's bench pairs; from the 1070:
  `bench-1070.txt`, `life-onebody-1070.txt`, `passes-before.txt`,
  `passes-after.txt` and `diag-summary.txt` before memory was freed,
  `fixed.txt` and `fixed-tests.txt` (the outcomes of the owned cells, times
  unusable), `r3-timing.txt`, `r3-tests-summary.txt`, `r3-maths.txt`,
  `r3-drift.txt`, `r3-drift32.txt` and `r4.txt`).
