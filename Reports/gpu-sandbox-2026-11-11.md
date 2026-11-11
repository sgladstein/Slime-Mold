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
from different keyed draws. Same seed, same pictures: preset 1 at 2 s
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

## The page's pictures

The videos are the first 10 s of each preset at real speed (a frame every
four, 15 a second), drawn by the sandbox's own code on the cloud's software
GPU (`run.sh`; raw output `raw/p1.txt`-`raw/p8.txt`), so the counts beside
them are the software GPU's, with the 1070's in brackets. The window
screenshot is preset 5 under Xvfb.

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

## Next

- The owner's hands on it.
- The thread "Reproduce Jenson's missing mould looks" tests feathery fronts,
  trunks and contagion on the CPU; what works comes here as controls.
  Phase 3 also plans contagion on a key and the tools as field writes.
