# The GPU sandbox (2026-11-11)

GPU plan phase 3, started ([`gpu-plan-2026-11-10.md`](gpu-plan-2026-11-10.md),
*Phases*). The owner asked, 2026-11-11 00:06Z: "can we take all the
different things you tested for the jenson comparison as options that I can
play with in a sandbox". The GTX 1070 timing had just passed the phase 3 bar
(100,000 walkers: 1.37 ms for a frame of two ticks), so the sandbox is
phase 3's toy, built on the timing test's step.

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

## Next

- The run on the GTX 1070: the frame with drawing, per preset, and the
  owner's hands on it.
- The thread "Reproduce Jenson's missing mould looks" tests feathery fronts,
  trunks and contagion on the CPU; what works comes here as controls.
  Phase 3 also plans contagion on a key and the tools as field writes.
- The twin test in CI on lavapipe (phase 3's gate).
