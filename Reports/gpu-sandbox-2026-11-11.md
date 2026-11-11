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
drops a food pile. *Note (2026-11-11 04:04Z, the owner's playtest):*
"pulls" misdescribed it. The walkers taken are picked at random over the
whole dish and jump into the disc, so left drag reads as dropping
(painting) mould at the mouse; the panel and the page now say "drops
walkers". Keys 1-8 load the presets:

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

## The looks as controls (2026-11-11)

The next step after the review: the looks from the thread "Reproduce
Jenson's missing mould looks" ([`jenson-looks-2026-11-11.md`](jenson-looks-2026-11-11.md),
PR #45) as controls. Every rule that report added to `BigParams` now runs
on the GPU, off by default, on two new panel pages (Tab turns the page):

| Page | Controls | The CPU's `BigParams` |
| --- | --- | --- |
| MAIN | Start: spread, home blob, or a disc at the centre | `blob_r` |
| LOOKS | Disc radius; haze at start (the share of walkers spread over the dish outside the disc) | `blob_r`, `haze` |
| LOOKS | Trail scale; reach, angle, turn and step by trail, each an amount and a power (a setting is its base plus amount x s^power, s = trail x scale capped at 1); trail power sets all four powers at once | `sense_scale`, `offset_mod`, `angle_mod`, `turn_mod`, `step_mod` |
| LOOKS | Square-root deposit cap (a cell's walkers lay deposit x sqrt(min(n, cap)) between them) | `deposit_cap` |
| LOOKS | Sensing floor (all three sensors below it: turn a random way); wander a tick (that share of walkers take a random heading) | `sense_floor`, `wander` |
| CONTAGION | Sick for, immune for (ticks); catches at germ; germ laid and kept a tick; sick speed, trail, reach and turn (multipliers); seeded at (seconds) and seed radius | `sick_steps`, `immune_steps`, `sick_catch`, `sick_deposit`, `sick_keep`, `sick_speed`, `sick_trail`, `sick_reach`, `sick_turn`, `sick_seed_r`; `BigDish::infect` |

The mouse: middle drag, or I once, makes the walkers under it sick; right
drag wipes germ with the trail. Germ is drawn in red over the trail, full at
four times the catch level (the CPU's `style=sick`), so the wave is the
germ field itself, not a painted effect. The GPU keeps one counter a walker
(sick, then immune) and a germ field in two more buffers.

Seven presets, from the looks report's dishes (1M walkers, Jenson's
settings, 1024 x 640):

| Key | Preset | Settings on top of key 4 |
| --- | --- | --- |
| 9 | Trunk ring | disc 180 fine cells, haze 0.3, respawn 0.1%, trail scale 0.02, step by trail (1, 1) |
| 0 | Contagion wave | cap 100; sick 120 ticks, immune 480, catch 0.5, sick trail x3; seeded at 4 s, radius 40 |
| Shift+1 | Sweeping ring | trunk ring with turn by trail (-20 degrees, 1) in place of step |
| Shift+2 | Long curves | trunk ring with angle by trail (-30 degrees, 1) in place of step |
| Shift+3 | Even polygons | disc and haze, respawn 0.1%, cap 100, no trail-set settings |
| Shift+4 | Endemic | the wave with sick 60, immune 120 |
| Shift+5 | Sick ropes | the wave with sick 240, immune 6000, catch 5, sick reach x0.5, sick turn x2, sick trail x1 |

### Twin-tested to the bit

A new twin test, `the_looks_match_the_cpu_for_one_tick`, runs one GPU tick
from a CPU dish's state against one `BigDish::tick` on six dishes (512 x
320, a quarter of the walkers, warmed 40 ticks on the CPU): trail-set
settings (trail scale 0.01, reach (4, 0.5), angle (-10 degrees, 2), turn
(-5 degrees, 1.5), step (0.5, 0.75)); floor 20 with wander 1%; cap 50;
contagion on a disc of 60 with haze 0.3, seeded at the start; contagion
with cap 100 and respawn 1%; and contagion one a cell. **On software Vulkan
every walker, the trail, the scent, the counters and the germ field match
to the bit on all six** (the bounds are as before, plus counters off for at
most 0.1% of walkers and germ L1 1e-4). The same-seed test now also runs a
looks and contagion dish with a mouse infection and compares the counters:
the same twice.

Planted bugs: the angle's power read from the turn's put 7165 walkers off;
the floor's random turn on another stream put 3046 off. The first test
dishes caught both only weakly (trail scale 0.05 with equal powers: 171
off; floor 0.5 and 3: 2 and 43 off), so the dishes were changed to the
ones above, where each setting moves most walkers.

### What they look like

Seed 1, 10 s, on the cloud's software GPU (`looks/run.sh`, raw
`looks/raw/`), with the GTX 1070's own dish in brackets
(`looks/gtx1070/raw/`). Looked at frame by frame before measuring:

| Key | Loops at 10 s | Width | What it does |
| --- | --- | --- | --- |
| 9 trunk ring | 53 (52) | 4.94 (5.07) | the disc's edge stays a bright thick ring inside a fine mesh |
| 0 contagion wave | 46 (46) | 4.73 (4.74) | red runs out from the centre along the tubes in about 2 s and is gone by 7 s |
| Shift+1 sweeping ring | 57 (56) | 4.09 (4.12) | the ring sweeps out past the disc to the dish's edge, leaving long veins |
| Shift+2 long curves | 34 (27) | 4.04 (4.03) | long curving ropes and a small bright ring at the centre |
| Shift+3 even polygons | 83 (76) | 4.60 (4.58) | an even mesh of polygons over the disc |
| Shift+4 endemic | 44 (45) | 5.39 (5.44) | the wave never ends: the dish stays red, and the share sick cycles every 1.5 s |
| Shift+5 sick ropes | 44 (41) | 4.67 (4.73) | a slower wave (most sick at 7 s, not 5.5) along thinner red tubes; behind it, at 8 s, the centre's tubes have faded |

The share of walkers sick every half second from the seed at 4 s (1070):

| Key | 4.5 s | 5 | 5.5 | 6 | 6.5 | 7 | 7.5 | 8 | 9 | 10 |
| --- | --- | --- | --- | --- | --- | --- | --- | --- | --- | --- |
| 0 contagion wave | 14% | 43% | 68% | 56% | 17% | 0 | 0 | 0 | 0 | 0 |
| Shift+4 endemic | 14% | 29% | 39% | 29% | 31% | 39% | 29% | 31% | 29% | 39% |
| Shift+5 sick ropes | 5% | 14% | 27% | 47% | 64% | 72% | 69% | 53% | 14% | 0 |
| key 1 + the wave's sickness, 100k one a cell | 2.5% | 6% | 7% | 9% | 10% | 14% | 20% | 20% | 22% | 16% |

On the software GPU the wave reads 12%, 41%, 66%, 58%, 21%, then 0, and
the sick ropes 5%, 13%, 24%, 42%, 60%, 69%, 69%, 58%, 18%, 0 (the same
shapes; each GPU's seed is its own dish). As on the CPU the wave
crosses the dish in about 2 s, and with immunity of 1 s (endemic) it never
ends: the whole dish falls sick together every 1.5 s, sampled here at
three phases of that beat (29-39%; the CPU report gave 30-36%). On the
owner's 100k dish one a cell the wave is slower and never takes more than
a quarter of the walkers by 10 s, probably because a tenth of the walkers
lay a tenth of the germ, so less of a tube reaches the catch level (not
tested).

### What they cost on the GTX 1070

Build b1f0383 against main before it (cf06d9c), headless, seed 1, 600
frames of two ticks and the drawing (`looks/run-1070.ps1`; raw
`looks/gtx1070/raw/`, checked against the PC's files by hash:
[`looks/gtx1070/README.md`](gpu-sandbox-2026-11-11/looks/gtx1070/README.md)).

**With every look off the new code costs 0.01 ms of GPU a frame.** Three
rounds each, alternating builds:

| Key | main: GPU ticks, wall median | looks build | Dish |
| --- | --- | --- | --- |
| 1 (100k) | 1.378-1.382 ms; 1.569-1.574 | 1.385-1.394 ms; 1.594-1.596 | the same counts on both builds |
| 4 (1M, Jenson) | 7.816-7.843; 8.234-8.248 | 7.812-7.832; 8.233-8.319 | the same |

On key 1 the new build was slower in all three rounds, by 0.01 ms of GPU
ticks and 0.02 ms of wall time a frame (the trail pass 0.128 -> 0.132 ms a
tick, the walk pass 0.050 -> 0.052: the extra parameters read); on key 4
the difference is inside the noise.

**Each look switched on**, GPU ms a frame (ticks + drawing):

| Run | GPU a frame | Wall median | Where it goes, a tick |
| --- | --- | --- | --- |
| Key 1, looks build (for reference) | 1.39 + 0.07 | 1.59 | sense 0.305, trail 0.132 |
| Key 1 + floor 0.25 + wander 1% | 1.40 + 0.07 | 1.60 | sense 0.310 |
| Key 1 + trail-set turn and step | 1.45 + 0.07 | 1.66 | sense 0.333 |
| Key 1 + contagion (the wave's) | 1.69 + 0.08 | 1.90 | trail 0.219 (the germ's spread), scent 0.175, sense 0.326 |
| 9 trunk ring | 5.81 + 0.27 | 6.25 | sense 2.31, walk 0.68 |
| 0 contagion wave | 9.79 + 0.29 | 10.34 | sense 2.62, walk 0.64, trail 0.23 |
| Shift+1 sweeping ring | 5.99 + 0.27 | 6.45 | sense 1.64, walk 0.65 |
| Shift+2 long curves | 3.86 + 0.26 | 4.31 | sense 1.09, walk 0.45 |
| Shift+3 even polygons | 7.58 + 0.27 | 7.97 | sense 2.76, walk 0.92 |
| Shift+4 endemic | 9.69 + 0.29 | 10.15 | sense 2.37, walk 1.38 |
| Shift+5 sick ropes | 9.45 + 0.29 | 9.84 | sense 2.19, walk 0.59 |

- **Contagion costs 0.30 ms a frame at 100k** (22% on key 1), most of it
  the germ field's 3x3 spread, which nearly doubles the trail pass, and
  0.01 ms of drawing. At 1M the contagion presets take 9.5-9.8 ms of GPU
  ticks a frame against key 4's 7.8: they fit a 60 Hz frame but not phase
  3's half-frame bar (8.3 ms). The pass profile, taken at 10 s once the
  wave has gone, accounts for 0.4 ms of the 1.6-2.0 ms over key 4 (sense
  +0.11 and trail +0.09 ms a tick); the rest (the cap, the wave's own
  frames) is not broken down.
- **The trail-set presets (9, Shift+1, Shift+2) are cheaper than key 4**
  (3.9-6.0 ms against 7.8), probably because the walkers bunch onto fewer
  paths, so their sensor reads hit the same memory (not tested). Floor and wander cost nothing
  measurable (0.01 ms).
- The same seed gave the same picture twice on all seven presets, on both
  GPUs.

## Next

- The owner's hands on it: the PC's window runs build b1f0383, with the
  looks, open on key 0 (the contagion wave).
- *Done 2026-11-11:* the looks of
  [`jenson-looks-2026-11-11.md`](jenson-looks-2026-11-11.md) are controls,
  each twin-tested (*The looks as controls*, above).
- Phase 3 also plans the tools as field writes.
