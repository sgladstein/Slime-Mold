# The GPU sandbox (2026-11-11)

GPU plan phase 3, started ([`gpu-plan-2026-11-10.md`](gpu-plan-2026-11-10.md),
*Phases*). The owner asked, 2026-11-11 00:06Z: "can we take all the
different things you tested for the jenson comparison as options that I can
play with in a sandbox". The GTX 1070 timing had just passed the phase 3 bar
(100,000 walkers: 1.37 ms for a frame of two ticks), so the sandbox is
phase 3's toy, built on the timing test's step.

Page: https://claude.ai/artifact/1rdrxadVcNdxTmbxp2KJGy (source, videos,
run script and raw output in [`gpu-sandbox-2026-11-11/`](gpu-sandbox-2026-11-11/)).

```
cargo run --release --bin mouldgpu
cargo run --release --bin mouldgpu -- preset=5          # opens on a preset
cargo run --release --bin mouldgpu -- preset=4 look=out seconds=10 gif=1 twice=1   # headless
```

## What it is

`src/bin/mouldgpu.rs`: the big dish's step (`mould::big`, the `nose` rule)
as the timing test's four compute passes a tick, drawn live by three more
passes on the GPU `pixels` opens for the window. No economy, census or
scoring; a toy for looking. Every control changes the running dish except
the walker count and the start, which restart it.

| Control | Range | What the comparison found with it |
| --- | --- | --- |
| Walkers | 10,000 to 8,000,000 (the GPU's buffer limit) | 4 million mostly smooth the grain |
| One walker a cell | on, off (off when walkers outnumber half the agar) | off with our numbers: ropes, 0 loops at 10 s against 151.5 |
| Sensor angle, turn, sensor reach, step | degrees and fine cells | 45 / 22.5 / 18 with no limit: polygons |
| Trail laid per 100,000 walkers | the deposit is laid x 100,000 / walkers | kept the trail over the dish the same across walker counts |
| Trail kept a tick | 0 to 0.999 | |
| Respawn a tick | off to 30% | 1%: a fine living mesh and a haze, 196 loops against 43.5 |
| Dish food, food smelled at | the seven piles; how far out scent is smelled | the nose rule smells food 36 fine cells out |
| Colour, drawn from, curve, knee, soft power | amber or grey; trail or walkers a cell; the game's 1 - e^(-v/knee) or (v / p99.9)^power | half the look is drawing: the soft curve and walker counts show thin veins |
| Ticks a frame | 1 to 32 | |
| Start | spread, or a blob at home | |

The mouse: left drag pulls walkers to the mouse (0.2% of them a tick jump
into a disc there, as respawned walkers jump; Jenson's 2020 performance
spawned particles with a stylus), right drag wipes the trail under it, F
drops a food pile. Keys 1-8 load the presets:

| Key | Preset | Numbers |
| --- | --- | --- |
| 1 | Big dish (phase 2) | nose rule, 100,000 walkers, one a cell, food, amber |
| 2 | 32,000 walkers | the same with 32,000 (today's deposit a walker) |
| 3 | No limit, our numbers | 100,000, no limit, no food, grey: the ropes |
| 4 | Jenson settings, 1M | 1,000,000, no limit, sensor 45, turn 22.5, reach 18, soft curve: the polygons |
| 5 | Jenson + respawn 1% | 4 with respawn 0.01: the living mesh and haze |
| 6 | Drawn by walkers | 5 drawn from walkers a cell |
| 7 | 4M walkers | 4 with 4,000,000 |
| 8 | 4M, our numbers | 4,000,000 with our angles and reach, no limit |

## What differs from the CPU dish

- **A jump is a step.** Respawn and the pull replace a walker's step for that
  tick and take part in the same claim, so with the limit on a jump into a
  full or claimed cell fails like a blocked step. The CPU's respawn
  (`BigParams::respawn`) runs after the move.
- **No fed scent.** A pile keeps calling at full strength once reached (the
  CPU drops it to `fed_scent`, which needs the trail read back).
- **The soft curve's p99.9** comes from a 256-bin log histogram on the GPU
  each frame (within 9% of the true percentile).

*Correction (2026-11-11, independent review):* the first two differences
are gone. Jumps now run after the step, as on the CPU, and a reached pile
lays `fed_scent` of its scent on the GPU too. "No fed scent" was not a
small difference: with the spread start the CPU counts all seven piles
reached after tick 1, so every food dish (keys 1 and 2) differed from the
first ticks. The histogram now has 512 bins, 2^-16 to 2^48: the old top of
2^16 clipped 0.66% of cells at laid 2000. See *Independent review*, below.

## Checked

On the cloud's software Vulkan driver (llvmpipe), seed 1, 10 s, spread
start, no food: the sandbox's dish against the comparison's CPU runs
(`jenson-compare-2026-11-10/raw/`, seed 1).

| Dish | Loops, sandbox / CPU | Width | Tube area | Strays |
| --- | --- | --- | --- | --- |
| 100,000, one a cell (preset 1, food off) | 148 / 151 | 5.21 / 5.21 | 12,610 / 12,615 | 0 / 0 |
| 100,000, no limit (preset 3) | 0 / 0 | 5.52 / 5.85 | 3,695 / 3,759 | 0 / 0 |
| 1,000,000, Jenson settings (preset 4) | 43 / 42 | 5.74 / 5.69 | 9,663 / 9,641 | 0 / 0 |
| the same, respawn 1% (preset 5) | 200 / 200 | 8.33 / 7.99 | 22,654 / 21,892 | 1.4% / 1.7% |

The respawn arm is wider by 0.34 cells and lays 3% more tube: the GPU's
jumps compete in the claim and the CPU's do not, but with no limit that
cannot fail, so the difference is the GPU drawing its spot and heading
from different keyed draws.

*Correction (2026-11-11, independent review):* that explanation was wrong.
The keyed draws were the same; 26% of jump targets fall off the agar, and
the GPU blocked those walkers (they stayed, turned at random and laid
nothing) where the CPU drops the jump and keeps the step. The GPU's width
and tube sat outside all 8 CPU seeds. The table is one seed, and row 1 has
no raw file (`raw/p1.txt` is the food-on run). The software-GPU costs below
are, in the raw, 28.0 and 22.6 ms at 100,000 and 75.9 and 77.7 ms at
1,000,000. Eight and sixteen seeds after the fixes: *Independent review*. Same seed, same pictures: preset 1 at 2 s
gave the same frame bit for bit twice (`twice=1`). The window was opened
under a virtual display (Xvfb) and its frame saved with the panel
(`MOULDGPU_SCREENSHOT_AFTER_FRAMES`).

Cost on llvmpipe, a CPU emulating a GPU (not GPU numbers): a frame of two
ticks and the drawing took 27 ms at 100,000 walkers and 80 ms at 1,000,000;
the drawing passes 3-7 ms of it.

## On the GTX 1070

Built and run through Remote Control at Scott-Desktop (main at 3ac5b20,
build 32 s), then left open for the owner on preset 1. Every preset 10 s
headless (`twice=1`), 600 frames of two ticks and the drawing, each
submitted and waited for; four presets also in the window with vsync
(`MOULDGPU_SCREENSHOT_AFTER_FRAMES=600`). Raw:
[`gpu-sandbox-2026-11-11/raw/gtx1070.txt`](gpu-sandbox-2026-11-11/raw/gtx1070.txt).

| Key | Dish | Frame, median / worst | GPU: ticks + drawing | Window: GPU a frame, frames a second | Loops, width at 10 s |
| --- | --- | --- | --- | --- | --- |
| 1 | Big dish, 100,000 | 1.42 / 2.18 ms | 1.23 + 0.07 | 4.62 ms, 58.8 | 192, 4.60 |
| 2 | 32,000 | 0.88 / 1.21 | 0.70 + 0.07 | | 101, 3.26 |
| 3 | No limit, 100,000 | 0.99 / 1.71 | 0.81 + 0.06 | | 0, 6.14 |
| 4 | Jenson settings, 1M | 8.00 / 12.22 | 7.69 + 0.18 | 7.02 ms, 60.2 | 43, 5.83 |
| 5 | + respawn 1% | 9.84 / 12.22 | 9.51 + 0.16 | 9.60 ms, 60.4 | 211, 8.44 |
| 6 | Drawn by walkers | 9.83 / 12.21 | 9.52 + 0.16 | | (the dish of 5) |
| 7 | 4M walkers | 30.07 / 47.08 | 29.82 + 0.18 | 26.29 ms, 37.2 | 36, 5.85 |
| 8 | 4M, our numbers | 12.84 / 47.25 | 12.50 + 0.17 | | 1, 5.90 |

- **Phase 3's bar holds at the owner's pick:** 100,000 walkers at 60 frames
  a second with the GPU busy 1.4 ms of the 16.7 ms frame headless, 4.6 ms in
  the window. The same seed gave the same frame twice on all eight presets.
  The twin test is now in CI (*The twin test in CI*, below).
- **A million walkers fit a 60 Hz frame** (8-10 ms) but not the half-frame
  bar; four million do not fit (37 frames a second).
- **The drawing costs 0.06-0.18 ms** a frame, the histogram included.
- **Not explained:** key 1 costs 4.25 ms of ticks in the window against 1.23
  ms headless, while key 4 costs a little less in the window. A guess, not
  measured: the GPU lowers its clock while it idles between vsynced frames
  at light load. Also not explained: key 4's 1M dish costs 7.7 ms for two
  ticks against the timing test's 3.5 ms at 1M with today's numbers (sensors
  twice as far, and a mesh instead of a few ropes).
- The 1070's dishes differ a little from the software GPU's (192 loops
  against 174 on key 1, 211 against 200 on key 5): the GPUs round their
  sums differently, and the dish amplifies it.

*Correction (2026-11-11, independent review):*

- "Busy 1.4 ms" is the wall time; the GPU's own figure is 1.30 ms.
- Key 4's GPU time (7.87 ms headless, 7.02 in the window) is under the
  half-frame bar; only the respawn keys are over it. Key 8, four million
  walkers with our numbers, fits a 60 Hz frame (12.84 ms median); key 7
  does not.
- Key 1's drawing read 0.36 ms in the window, outside the 0.06-0.18 quoted.
- The window column is a different statistic from the headless one: a
  running 0.9/0.1 average that started at 0, read at frame 600, and frames
  a second from one half-second.
- The window gap is now explained by measurement (the GPU's clocks), and
  key 4's cost by its settings (the per-pass profile): *Independent
  review*, below.
- "Differ a little" was untested. A seed on the two GPUs gives two
  different dishes, by more than the CPU's spread over seeds (key 3 width
  6.14 against 5.52, outside the CPU's 5.52-5.96); over 8 seeds each GPU's
  spread matches the CPU's.

## The page's pictures

The videos are the first 10 s of each preset at real speed (a frame every
four, 15 a second), drawn by the sandbox's own code on the cloud's software
GPU (`run.sh`; raw output `raw/p1.txt`-`raw/p8.txt`), so the counts beside
them are the software GPU's, with the 1070's in brackets. The window
screenshot is preset 5 under Xvfb.

*Correction (2026-11-11, independent review):* the headers of `raw/p1.txt`
to `raw/p8.txt` lack the "drawn" field the committed code prints, so they
came from a build just before the commit (the simulation is the same in
both). `run.sh` says the 1070 ran the same command;
it ran without `frames=1`. The fixes changed keys 1, 2, 5 and 6, so
their videos and stills were drawn again with build 9d4a2d1 (raw output
`review/video-raw/`); the page's counts are now those runs'. Keys 3, 4, 7
and 8 have no food and no respawn, and their dishes are unchanged (keys 3
and 4 gave the same counts on the new build).

## The twin test in CI

Phase 3's gate asks for a twin test: the GPU's step against the CPU's. The
fields are `f32`, so a GPU dish and the CPU dish drift apart over a run
(the GPUs round `cos`, `sin` and sums differently, and the dish amplifies
it), and the plan's fallback applies: **one GPU tick from the CPU dish's
own state against one `BigDish::tick`**. `Engine::inject` loads a CPU
dish's walkers, trail, scent, pile scent and step into the GPU's buffers.
Three tests in `src/bin/mouldgpu.rs`, run by `cargo test`:

| Test | What it checks | On software Vulkan, 2026-11-11 |
| --- | --- | --- |
| `one_gpu_tick_matches_one_cpu_tick` | presets 1, 3 and 4 at a quarter of the walkers on a 512 x 320 dish, warmed 40 ticks on the CPU, then one tick each | every walker within 0.001 (0.1% not to the bit), trail and scent to the bit, on all three |
| `the_same_seed_gives_the_same_picture_twice` | preset 5 (respawn 1%, soft curve) for 40 ticks with the pull and the wipe, twice | the same picture and walkers to the bit |
| `the_crowd_limit_holds_through_jumps` | preset 1 with respawn 1% and the pull, 60 ticks | the GPU's count a cell equals the walkers there, none above one, all on agar |

The bounds (at most 0.1% of walkers off by more than 0.001, trail L1 1e-4,
scent L1 1e-6) leave room for a real GPU's rounding to flip a near-tie.
Two planted bugs show the test bites: the tick's step off by one put 0.6%
to 58% of walkers off and the trail 5e-4 to 8e-3 off; the blocked
walkers' draw on the wrong stream put 52% off on preset 1.

CI's test job installs Mesa's software Vulkan (`mesa-vulkan-drivers`) and
sets `MOULDGPU_TEST_GPU=1`, so a missing GPU fails the tests there; on a
machine with no GPU and without that variable they skip and say so. The
three take about 1.5 s on the cloud's software Vulkan.

## Independent review (2026-11-11)

The owner asked, 01:44Z: "Next steps or independent review of your work so
far?" Two fresh reviewers went through the work, in parallel. One read and
ran the code against `mould::big` with a harness of its own. The other
checked every number in this report, in the GPU plan's timing sections and
in the Jenson comparison against the raw output. The findings that held up
were fixed in build 9d4a2d1 and re-measured on 8 or 16 seeds, on the
software GPU and on the GTX 1070, and the summaries were corrected (dated
notes above, in the [GPU plan](gpu-plan-2026-11-10.md) and in the
[Jenson comparison](jenson-compare-2026-11-10.md)). Files:
[`gpu-sandbox-2026-11-11/review/`](gpu-sandbox-2026-11-11/review/).

### What the code review found, and the fixes

| Finding | Before | Fix |
| --- | --- | --- |
| No fed scent on the GPU. With the spread start the CPU counts all seven piles reached after tick 1 and turns them down; the GPU's piles called at full strength for good | key 2: strays 3.6% (3.3% on the 1070) against the CPU's 0.8-1.8% over 16 seeds; width 3.24, below the CPU's lowest seed (3.37) | the trail pass flags a pile once its trail passes the tube level (the CPU's end-of-tick `update_reached`); the scent pass lays `fed_scent` (0.15) of a flagged pile's scent |
| A jump replaced the step. 26% of respawn targets are off the agar, and the GPU blocked those walkers (they stayed, turned at random and laid nothing); the CPU drops the jump and keeps the step. A jumper also laid its trail where it landed | key 5, one tick from the same state: 4,090 walkers stayed put against the CPU's 1,463. Width 8.33 against the CPU's 7.70-8.01 | `jump` and `land` passes after `walk`, only while respawn or the pull is on. A jump off the agar is lost; with the limit on, a jump into a full cell is lost and two into one empty cell go to the lower walker. One tick now matches to the bit on key 5 and on key 1 with respawn |
| Switching the limit on mid-run | key 3 for 240 ticks, then the limit on: 19,589 cells held more than one walker, and 3,906 still did 720 ticks later | the switch restarts the dish |
| The soft curve's histogram stopped at 2^16 | laid 2000: the true p99.9 was 119,012, and 0.66% of cells clipped | 512 bins, 2^-16 to 2^48. New test: the reference is 1.027 times the true p99.9 |
| The scaling chosen at start-up was not applied | a window smaller than the dish was drawn cropped | `resize_surface` after `set_scaling_mode` |
| Window timings | a reading kept when pixels skipped the frame; the average started at 0 | stamps only from frames drawn; the average seeded with the first reading; the screenshot hook prints the median of every timed frame |
| Command line | `ticks=0` never ended; `crowd=false` meant on; misspelt names fell back silently; `smell=0` smelled at the walker (the CPU's 0 means the reach); 30% printed as 30.000002% | strict switches and names, ticks 1-32, `smell=0` is the reach |

Not fixed, and not in the timings: pixels uploads its own 2.6 MB frame
every frame, unused, before the sandbox's copy overwrites it.

The code reviewer found the rest sound. With food off, the step matches
`big.rs` for 8 ticks: the trail is the same to the bit, every walker is in
the same cell, and the largest position difference is 1.8e-4. Pass order,
buffer layouts, histogram barriers, timestamps, headless readback and the
panel were also sound. Twin tests: 9, about 3.5 s on software Vulkan, among
them one tick on keys 1, 3, 4 and 5 and on key 1 with respawn, and a reached
pile quietening over two ticks.

### The dishes after the fixes, against the CPU

At 10 s from a spread start, the CPU's raw runs (the Jenson comparison's
8 seeds; the scaling rule's 16 for the food dish) against `review/run.sh`
on the software GPU and `review/run-1070.ps1` on the 1070, compared by
`review/compare.py` (outputs `compare-llvmpipe.txt`, `compare-gtx1070.txt`).
The deciding measures were loops, width, tube and strays, each a paired
sign test by seed with Bonferroni over 20 tests a GPU.

| Dish | Measure | CPU | Software GPU | GTX 1070 |
| --- | --- | --- | --- | --- |
| 32,000 with food (key 2), 16 seeds | loops | 86-100 | 88-98 | 92-101 |
| | width | 3.37-3.45 | 3.38-3.47 | 3.34-3.46 |
| | strays | 0.8-1.8% | 0.5-1.4% | 0.8-1.7% |
| 100,000 one a cell (key 1, food off), 8 seeds | loops | 144-160 | 143-156 | 150-161 |
| | width | 5.15-5.23 | 5.18-5.24 | 5.13-5.24 |
| 100,000 no limit (key 3) | loops | 0-2 | 0-2 | 0-1 |
| | width | 5.52-5.96 | 5.51-6.05 | 5.65-6.14 |
| 1M, Jenson settings (key 4) | loops | 38-52 | 38-48 | 37-51 |
| | width | 5.69-5.92 | 5.74-5.91 | 5.83-5.92 |
| 1M, respawn 1% (key 5) | loops | 187-208 | 181-202 | 192-206 |
| | width | 7.70-8.01 | 7.83-8.00 | 7.82-8.12 |
| | tube | 21,670-21,956 | 21,625-21,994 | 21,733-22,022 |

- **No difference survives the correction on either GPU.** The smallest p
  is the 1070's food dish, more loops on 12 of 16 seeds (p 0.013, against
  the 0.0025 Bonferroni needs). Before the fixes, key 5 on seed 1 had width
  8.33 and tube 22,654, outside every CPU seed; now it has 8.00 and 21,907.
- **Key 1 with food**, seed 1 on the 1070: 171 loops, width 4.90, strays
  1.7%, where before it was 192, 4.60 and 3.3%. The code reviewer's CPU run
  of the same seed gave 164, 4.96 and 1.5%.
- **A seed is not the same dish on two GPUs.** The software GPU and the
  1070 differ by a median of 3 loops on the 100,000 dishes (up to 12), and
  by up to 18 with respawn: their `f32` sums round differently, and a few
  seconds make that a new seed. The sign test pairs by seed but tests the
  spread. The check that holds to the bit is the twin test, one tick from a
  CPU state.

### What a frame costs on the GTX 1070 now

Build 9d4a2d1, headless, seed 1: 600 frames of two ticks and the drawing,
and each pass timed alone (median of 11 ticks). Raw:
`review/gtx1070/raw/p1-s1.txt` to `p8-s1.txt`.

| Key | Frame, wall median / worst | GPU: ticks + drawing | Before the fixes | A tick's passes, ms |
| --- | --- | --- | --- | --- |
| 1 | 1.58 / 2.18 ms | 1.385 + 0.068 | 1.23 + 0.07 | sense 0.307, walk 0.049, trail 0.127, scent 0.162 |
| 2 | 1.01 / 1.21 | 0.818 + 0.067 | 0.70 + 0.07 | sense 0.080, walk 0.021, trail 0.128, scent 0.162 |
| 3 | 1.09 / 1.80 | 0.914 + 0.057 | 0.81 + 0.06 | sense 0.103, walk 0.036, trail 0.128, scent 0.159 |
| 4 | 8.24 / 37.73 | 7.845 + 0.286 | 7.69 + 0.18 | sense 2.506, walk 0.639, trail 0.127, scent 0.157 |
| 5 | 10.12 / 35.98 | 9.680 + 0.266 | 9.51 + 0.16 | sense 3.045, walk 1.331, jump 0.055, trail 0.128, scent 0.157 |
| 6 | 10.12 / 12.56 | 9.686 + 0.265 | 9.52 + 0.16 | as key 5 |
| 7 | 30.11 / 47.20 | 29.792 + 0.285 | 29.82 + 0.18 | sense 10.422, walk 2.370, trail 0.128, scent 0.160 |
| 8 | 12.98 / 47.40 | 12.569 + 0.276 | 12.50 + 0.17 | sense 3.723, walk 1.039, trail 0.128, scent 0.160 |

- **The fixes cost 0.10-0.17 ms of ticks a frame** on keys 1-6, from the
  pile reads in the trail and scent passes and, with respawn, the jump
  passes (0.055 ms a tick). The 512-bin histogram adds 0.1 ms of drawing on
  the soft-curve keys (4-8). Keys 7 and 8 show no change, within their
  noise. Each figure is one run, an hour after the first, on the same PC.
  The owner's pick, key 1, takes 1.45 ms of GPU time a frame.
- **The drawing passes**, a frame: draw 0.05-0.06 ms; on the soft curve,
  histogram 0.03-0.05 and `top` 0.17, which reads the 512 bins in one
  workgroup and could be made cheaper.
- **Why Jenson's settings cost twice ours at a million walkers:** the sense
  pass. It takes 2.5 ms a tick at 1M with sensors 18 out, against 0.93 ms
  a million with ours (key 8: 3.72 ms at 4M). The walk pass doubles too
  (0.64 against 0.26). The far sensors read memory farther from the
  walker. This answers the "not explained" above.
- The same seed gave the same picture twice on all eight keys. The worst
  frames of 36-38 ms on keys 4 and 5 are single hitches on the PC (key 6,
  the same dish, had a worst of 12.56).

### The window gap is the GPU's clocks

nvidia-smi was polled about every 0.3 s through four runs
(`review/gtx1070/clocks.txt`).

| Run | GPU a frame, median of about 300 timed frames | Clocks |
| --- | --- | --- |
| headless, key 1 | 1.394 + 0.068 ms | 1936 MHz, memory 4006 MHz, throughout |
| window, key 1 | 1.795 + 0.080 ms, 58.7 frames a second | 1594 MHz, then 1012 MHz, then for the last 2 s 797 MHz with the memory at 810 MHz (P5) |
| window, key 1, 8 ticks a frame | 4.938 + 0.067 ms (0.62 a tick), 60.5 | 1936 MHz throughout |
| window, key 4 | 7.836 + 0.272 ms, 60.4 | 1936 MHz throughout |

At 100,000 walkers the GPU is busy a tenth of each vsynced frame, so the
driver steps its clocks down. In P5 the memory runs at a fifth of its speed
and the step reads memory everywhere, so a tick takes about three times as
long. The old window figure (4.62 ms) was a running average read at frame
600, in that slow tail; the median over the run is 1.88 ms. Given more work
(8 ticks a frame, or a million walkers), the GPU holds its full clock, and
the window costs what headless does. So the window's extra cost at light
load is the GPU idling, not lost time; headless is the figure to quote.
The driver's "prefer maximum performance" setting would hold the clock for
window timings (not tried).

## Next

- The owner's hands on it: the PC's window runs build 9d4a2d1, with the
  fixes.
- The thread "Reproduce Jenson's missing mould looks" has reported
  ([`jenson-looks-2026-11-11.md`](jenson-looks-2026-11-11.md)): trunks from a dense start with slow respawn, contagion along the
  tubes, and trail-set turn and angle, all off by default in `BigParams`.
  Those come here as controls next, each with a twin test. Phase 3 also
  plans the tools as field writes.
