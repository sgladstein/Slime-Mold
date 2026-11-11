# Technical feasibility review (independent agent, 2026-11-11)

Prompt: [common.txt](common.txt) + [feasibility.txt](feasibility.txt). Report as returned:

# Technical feasibility review: Larder World roadmap

The checkout has uncommitted W1 work: `src/mould/big.rs` is 211 lines longer than HEAD 8393dda (`crystal_charge`, terrain, `rect`). This review covers HEAD plus that diff.

## The three most important points

**1. W0 leaves out the hard part, and Larder does not run on the big dish at all.**
- `big.rs` has no energy, no alive flag and no slot pool. Its walkers are a fixed `Vec` (big.rs:383, "walker count is fixed", :7-9).
- Larder is built on the CPU `Dish` (larder.rs:38, :333). It reads `dish.walkers` (:551, :587, :713), `dish.fire` (:543), pile food (:761), writes `dish.scent` directly (:683, :885, :919) and switches births off with `params.divide` (:702).
- That economy pools food per piece, using the BFS census (`economy_by_piece`, mod.rs:2878). Births draw from one sequential `self.rng` against a running per-piece budget, and deaths use `swap_remove` (mod.rs:2930-2970). None of that ports to the GPU as it is.
- **W0 has to decide first:**
  - (a) Per-walker energy, all on the GPU, no readback. Cheap to twin, but untested for whether tubes far from food survive.
  - (b) The phase-4 design: an 80 KB fine bit-mask read back, BFS on a worker thread, labels uploaded (gpu-plan:107). It is safe to run late: labels 4 times a second and two frames late met every dish's ship bar, with nothing surviving Bonferroni (gpu-plan:317-319).
- Either way, W2 is a re-port of Larder, so the M1-M3 censuses do not carry over.

**2. The world-size numbers are mislabelled, and the size is fixed in code.**
- "2048 x 1280 at 8-10 ms" (roadmap:27) is really 1M walkers with Jenson's settings at **1024 x 640** (gpu-sandbox:137, keys 4-5, 7.8-9.7 ms). Nobody has run 2048 x 1280 on a GPU; the only figure is the CPU's 110.8 ms a step at 1M (gpu-plan:36).
- My estimate from the 1070's per-pass times (gpu-sandbox:296-312):
  - Field passes cost 0.29 ms a tick at 655k cells, so about 1.15 ms at 2.6M.
  - 400k walkers at today's density add about 0.6-0.75 ms (sense 0.93 ms per 1M at key 8).
  - Each extra species trail field adds about 0.5 ms a tick.
  - So a frame of 2 ticks is about 3.7 ms with one species and about 5.7 ms with three, plus about 0.1-0.3 ms of drawing.
  - At Larder's slow clock (`GAME_SECOND` 15, larder.rs:41; the `nose` rule already moves 1/4 dish cell a tick), about 1 tick a frame is enough, which halves those figures.
- One `gputime` run on the 1070 would replace this estimate.
- **Fixed in code:**
  - `w = W * scale` locks the world to the dish's 8:5 shape (big.rs:335-341).
  - `place_piles` and the coarse scent grid use `W, H` (big.rs:546-566, :467-469).
  - The GPU holds `MAX_PILES = 64` (mouldgpu.rs:880), which crystals plus soldier bodies plus drops will exceed.
  - The `frame` buffer is sized to the whole world (mouldgpu.rs:2017), so a camera needs a draw pass with a viewport.
- A world bigger than the screen needs its width and height in dish cells, separate from `scale`.

**3. The W2 gate "replays reach the same hash" holds only on the same GPU and driver.**
- The 1070 and llvmpipe already differ (192 against 174 loops, gpu-sandbox:146), and fixed point is a recorded dead end (dead-ends.md:11). So CI cannot check a 1070 replay. Gate on "same machine, same hash" plus the one-tick twin test.
- Readbacks must be applied at a fixed lag counted in ticks, not "whenever the map finishes", or replays will depend on frame timing.
- `read_back` (mouldgpu.rs:2346) blocks on `poll(wait_indefinitely)`. Use a ring of staging buffers, as `Stamps` does (:2361).

## What is missing, and how hard each part is to twin

| Feature | What it needs | Twin |
|---|---|---|
| Crystals with finite charge | The CPU diff exists (big.rs:735). With no crowd limit, `charge[i] -= t` runs in walker order, and `pile_charge` is an f32 sum (:757-759). WGSL has no f32 atomics: do a per-cell pass `charge -= min(n*eat, charge)` and sum per pile in u32 fixed point. Scent is scaled by charge share (:1163) | Cheap, exact |
| Terrain that changes speed | `wet` in the diff. On the GPU, a per-cell ground word read by `sense` and `walk` | Cheap |
| Walls | `agar` is read-only (mouldgpu.rs:8 binding). Use a ground buffer patched by `write_buffer`. A wall cell that already holds walkers must kill or move them (`cell().expect` at big.rs:654) | Cheap |
| Births and deaths | An alive flag; dispatch over the pool's capacity. Births claim an empty neighbour cell with `atomicMin`, as `jump`/`land` already do (mouldgpu.rs:541-576). New walkers take slots in a set order: a prefix sum over dead slots, matched to births ranked in parent order. Draws are keyed on the slot (`keyed`), so the slot order decides behaviour | **Hard**; the twin must count 0 mismatches |
| Several species, separate economies | Species are channels of one `BigParams`; only `species_boost` differs (big.rs:178-192). Drifter, Sprawler and the rest need a parameter table per species, indexed by `spec[k]` | Medium; this is a data-layout choice to make in W0 |
| Soldiers reading bins | 4-cell bins on a 1024 x 640 world are 64 x 40 = 10 KB of u32 | Exact |
| Shots zeroing walkers | Upload line segments; a `fire` pass zeroes trail, sets a wound-until time and clears the alive flag. **Depends on W0's alive flag** | Exact |
| Scent from a hoard, soldiers, drops | A source list (disc, strength), at most 128 entries, about 2 KB a tick, stamped in the scent pass. The `lay` buffer is static today (mouldgpu.rs:2094) | Exact |
| Bite and touch | `walkers_near` scans every walker for each unit (larder.rs:587). At 100k walkers and 45 units that is about 4.5M tests a step, roughly 5-9 ms on the CPU (my estimate). Count on the GPU into one u32 per unit | Exact |

**What is read back each tick:** the bins, one count per unit, per-pile charge, alive count per species and kills per shot, about 11-15 KB. The PCIe cost is negligible; the cost is 1-3 frames of latency. The census bit-mask, if option (b) is chosen, is 80 KB a few times a second.

## Order of code changes

**W0**
1. Add `capacity` and `alive` to `BigDish`. With the rules off, the hash must not change.
2. Per-walker energy: eat, pay upkeep, die by a keyed draw, give birth into a claimed cell, with slots from the prefix sum. Settle option (a) or (b) first.
3. GPU passes: `economy`, `scan` and `born`, with new energy and alive buffers. Extend `inject` (mouldgpu.rs:2134) and twin one tick with births and deaths.
4. Time it on the 1070 at 100k alive in a 200k-400k pool.

**W1**
5. World width and height in dish cells, separate from `scale`; a pile buffer to replace `MAX_PILES`.
6. A ground word per cell (rock, wet, wound-until) on the GPU.
7. Per-pile charge in u32 on the GPU, feeding the scent pass.
8. Landing and race layouts; the race needs W0's births.

**W2**
9. Source list, shot segments and the `fire` pass.
10. Readback ring with a fixed tick lag.
11. A `World` backend under `Game` that replaces each `dish.*` use listed in point 1, with `Rules` converted to world units.
12. A viewport draw pass for the camera.

## Interfaces between threads

- **`Cmd`:** keep it as it is (larder.rs:216), in dish-cell coordinates, with a team id added, plus `Wall(rect)` and `Build`. It is logged by tick.
- **`world` module:** `apply(Cmd)`, `step()` and `view(tick) -> View`, with a CPU-twin backend and a GPU backend.
- **`View`:** coarse bins per species, crystal charges, units, ground patches, an optional visibility mask, and the tick it describes. It is built only from the readbacks, so the AI never sees walker positions.

## Main risks, with numbers

- **Player AI censuses on the CPU are too slow at full scale.** 100k walkers take 23.6 ms a tick on 4 threads (gpu-plan:594). A 15-minute round is about 54k ticks, so about 21 minutes a game, and M3's 256-game census would take about 90 hours. On the 1070 at about 0.7 ms a tick it is about 38 s a game, about 2.7 hours. Since that PC is often off, plan a small CPU twin world for development censuses.
- **Births are the one part that is hard to make parallel and deterministic.** Budget for a twin that has to reach exactly 0 mismatches.
- **The window frame cost moves.** M3's 7.2 ms frame was 6.1 ms of CPU drawing; on the GPU the drawing is 0.07 ms. The new cost is the CPU game layer waiting on readbacks.
- **Binding headroom.** The shader already uses 32 storage bindings (mouldgpu.rs:1863) and relies on the adapter's limits (`required_limits: a.limits()`, :2754). New buffers should be packed into existing ones where possible.

