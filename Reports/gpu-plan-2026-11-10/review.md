# Independent review of the GPU plan (2026-11-10)

The owner asked for an independent review before the plan in
[`../gpu-plan-2026-11-10.md`](../gpu-plan-2026-11-10.md) goes ahead. A fresh
agent, given the plan's text and this repo but not the planner's reasoning,
checked it against the code and the pixels 0.17.2 source and wrote the report
below, unedited apart from the benchmark script's path. The planner then
re-checked the main claims (`FED_EVERY` at `mod.rs:493` and the census call in
`step`; pixels' public `device`/`queue`, `render_with` and its early return on
`Occluded | Timeout`; no `/usr/share/vulkan/icd.d` in the container) before
folding it into the plan's section *What the independent review changed*.

Line numbers refer to `main` at the time of the review.

**Verdict.** The plan holds up technically. pixels 0.17 does give access to its wgpu device, queue and command encoder, so a compute path fits the current stack. Phases 1 and 2 are the right shape: cheap first, with a real fork at phase 2. But it underestimates three things. (a) The CPU/GPU split is not one-way: the economy reads the census every step. (b) A grid 4x finer changes how fast the mould moves and how far it smells in dish terms. That is the same loss as the `sensor_offset` dead end, not a phase-5 retune. (c) The repo's way of measuring (hundreds of paired-seed runs in cloud sessions) has no GPU to run on. The determinism story also has holes. Fix these on paper and add two cheap checks at today's scale before any GPU code is written.

## Verified findings, most severe first

1. **The census feeds the GPU economy; it is not just read off it.** `FED_EVERY = 6` (mod.rs:493): the census runs 20 times a second (mod.rs:1640). `economy_by_piece` runs every step and reads `fed[i]` and `piece_of` for each walker's starve, hungry and divide rolls (mod.rs:2282, 2739, 2774). It also pools food per piece and spends a per-piece birth budget in order (`food[g] -= 1.0`, mod.rs:2316-2366). A "4 Hz readback, a frame or two late" means fed status and piece labels must go back up to the GPU, roughly 300 ms stale against 50 ms today. Nobody has checked "the census does not notice".
   **Change:** in phase 1, run today's dishes on the CPU with the census cut to 4 Hz plus a 2-frame delay, paired against today's. Name the upload path in the design.

2. **The finer grid changes the game's pace and smell range.** Walkers move 1 cell a step, so on a 4x grid they cross the dish 4x slower at 120 steps/s. A 3x3 diffusion per step gives about 4-16x less spread in coarse units, so scent from `scent_keep` 0.97 reaches about 4x less far. `dead-ends.md` (sensor_offset 9 to 6) shows that smelling food from less far broke every planned hand.
   **Change:** decide the scaling rule before phase 2, and render phase 2 under it. Options: keep scent, light, wet and salt on the 256x160 grid and only trail fine; and/or run 4x steps a frame (4x GPU cost).

3. **Nowhere to measure.** This container has 4 cores, a Vulkan loader and no driver (`/usr/share/vulkan/icd.d` is missing), so sessions cannot run the GPU path. Every GPU gate, GIF and census needs the owner's machine. Seeds also cannot be paired across engines, because the random streams differ, so "keeps its shape" in phase 4 cannot use the sign test.
   **Change:** make the simulation all-integer:
   - fixed-point positions;
   - headings as an index into a shared sin/cos table;
   - fixed-point diffusion.

   Then CPU, GPU and lavapipe give the same bits, the CPU reference can run the census in containers, and census tables no longer need to name the machine. Plan Remote Control runs for the frame-time gates. Quote the census cost at the chosen scale (finding 10).

4. **The determinism claim only covers sums.** Integer `atomicAdd` makes totals order-free. It does not make outcomes order-free when they depend on an atomic's return value. Four places in the plan do:
   - the crowd limit N on moves;
   - births into empty neighbour cells (mod.rs:2350-2353);
   - per-piece birth budgets;
   - push-light moving walkers into free cells (mod.rs:1754).

   The twin test ("compare fields within a tolerance" over a run) will also fail. The model is chaotic, and WGSL's sin/cos accuracy is looser than the CPU's, so turn decisions flip within steps.
   **Change:** use a claim pass to settle conflicts (e.g. `atomicMin` of hash/id per cell), and a segmented prefix sum in parent-id order for budgets. Make the twin test one step from injected state, with flips counted, plus statistical equivalence for long runs. Or adopt finding 3 and require exact bits.

5. **A coarse max-pooled census over-joins, and its look breaks the owner's rule.** The census is a 4-neighbour BFS over `trail > tube` cells, with joins found where searches meet (mod.rs:1945-2008). Max-pooling 4x4 blocks merges parallel tubes and hides fine cuts and re-formed one-cell bridges, so snips and storm cuts would read as still joined. Drawing hunger and fed state from a coarse mask over fine tubes gives blocky fed/unfed edges, which the owner banned.
   **Change:** read back a fine bit-mask instead: about 80 KB at 1024x640, trivial bandwidth. Run the BFS at full resolution on a CPU thread (milliseconds), or as GPU label propagation. Draw hunger from a fine held quantity.

6. **pixels details the design must handle** (pixels-0.17.2 source):
   - **Access works.** `PixelsContext` exposes `device`, `queue` and `texture` publicly (lib.rs:74-103), and `render_with` hands over the encoder (lib.rs:547).
   - **Skipped frames.** `render_with` returns without calling the closure when the surface is Occluded or times out (lib.rs:573). Simulation passes inside it would freeze, so submit them on their own encoder.
   - **Compositing.** `ScalingRenderer` clears the target (renderers.rs:258) and the full CPU frame is uploaded every frame (lib.rs:588). HUD and dish overlays need a compositor that blends layers.
   - **Timestamps.** The device is requested with default (empty) features (builder.rs:309-310), so the GPU timestamps phase 3 relies on need `PixelsBuilder::device_descriptor` with `TIMESTAMP_QUERY`.
   - **Headless.** The device is tied to a window (builder.rs:279). The probe, CI and twin test need a headless device through the `pixels::wgpu` re-export (lib.rs:39), and simulation code should take `&Device`, not `Pixels`.
   - **Atomics.** f32 atomics are native-only (Metal on Apple7+/MSL 3, or Vulkan with an extension; not DX12), and 64-bit atomic add is Vulkan/DX12 only (wgpu-types features.rs:612-622, 1174-1180). That leaves u32 fixed point, so budget its overflow: piles hold up to 80,000 food (level.rs:310).

7. **The drawing port is understated.** `look.rs` is 5,235 lines, including:
   - smoothing, stain history, wet and sleep fields, hunger;
   - tube insides and walker specks;
   - piles, storm, alarm and the lamp.

   "Removes the 3.83 ms" is a claim until that is ported. Also, the dish already fills 1024x640 pixels (`frame_size`, render.rs:22-23; SCALE 4, bin/mould.rs:58). A 1024x640 grid is one cell per pixel, so CPU drawing cost does not grow there. A 2048x1280 grid is finer than a pixel unless the screen is HiDPI or there is zoom.
   **Change:** add a blind A/B of the look ported to WGSL against the CPU look, on the same data.

8. **The rival is a second whole dish, not "a second trail channel."** It has its own census, pieces and economy, mutual cell exclusion (`hold_foe_cells`), fights that read foe occupancy of 8 neighbours plus fed state, and pile reconciliation by `pile_key` (mod.rs:1434-1525, 1527-1553). The Wild One's tuning assumes one walker per cell.

9. **Some constants are counted in walkers, not cells.** Examples: `table` 40 (one pile feeds about 300 walkers), `INOCULUM` 400, `WILD_START` 600, `START_STORE`, and `crowd` 7 in a 5x5 window. Phase 5 lists only constants measured in cells.

10. **Phase 2 cost, and what it can answer.** A single-core scratch benchmark (bare Jones move plus two-field diffusion, walkers spread evenly; script at [`step-bench.rs`](step-bench.rs), `rustc -O step-bench.rs`):

    | Grid | Walkers | Step cost |
    |---|---|---|
    | 256x160 | 2,200 | 0.13 ms |
    | 1024x640 | 100k | 9.5 ms |
    | 1024x640 | 1M | 98 ms |
    | 2048x1280 | 1M | 112 ms |

    On 4 cores, a 90 s dish at 1M walkers takes about 6 min, or 25 min at the pace kept from finding 2. A sweep is hours to a day, and review 2's 270 runs at that scale would be about 27 hours. The gate is visual only, and judging a look from pictures does not answer the go/no-go question. The 22.5-degree review already found the mould "joins all the food untouched", and a denser mesh may let doing nothing win any dish scored on joins.
    **Change:** drop 2048x1280 from the CPU sweep. Put the economy and coarse census in the phase 2 sim. Add to the gate: on 8-16 paired seeds of one join dish and Running Dry, plan beats idle and random. Use the existing `pull=1` and push-offset instruments as measures of whether the lamp and oats can still be seen working.

## Judgement calls

- **Two checks at today's scale in phase 1**, both cheap and decisive for how risky the port is:
  - a "GPU semantics" CPU mode (synchronous moves, hashed random numbers, no ordered occupancy), with a paired census on every dish;
  - the delayed-census arm from finding 1.
- **A GPU spike on the owner's machine** (about 300 lines of wgpu, 1M walkers, timed) alongside phase 2. It settles decision 2 and the frame budget before months go in, and shows atomic contention on integrated or Apple GPUs.
- **A cheaper route** for phase 2: a 512x320 grid with about 20k walkers is probably playable on multi-core CPU. GPU drawing alone adds nothing visible with today's data.
- **Port selectively.** Scale does not fix the open design problems (Keep It Alive's luck, The Wild One's middle). Decide which dishes' goals survive a dense network before porting every dish, so phase 6 thinking comes before phase 5.
- **Smaller points:**
  - The contagion gate ("16 of 16") is a yes/no; state it as a spread.
  - A lavapipe CI job costs billed minutes against ci.yml's stated two-job budget; fold it into the existing test job, small.
  - "The lamp's push becomes a force field the move reads" risks bringing back the lamp dead end, where walkers only steered away and the tube broke (`dead-ends.md`, `light_weight`). Port it as moving trail and walkers outward, as `push_light` does now.
