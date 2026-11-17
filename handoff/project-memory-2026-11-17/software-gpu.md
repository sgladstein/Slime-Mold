---
name: software-gpu
description: Cloud containers have no GPU, but `apt-get update && apt-get install -y --no-install-recommends mesa-vulkan-drivers` gives llvmpipe Vulkan so wgpu code runs (slowly) for correctness checks
metadata:
  type: reference
  modified: 2026-11-12T17:22:00.000Z
---

Cloud sessions have no GPU and no Vulkan ICD by default (libvulkan1 loader only). On 2026-11-10 the GPU plan thread ran `apt-get update` then `apt-get install -y --no-install-recommends mesa-vulkan-drivers`; that adds `lvp_icd.json` (llvmpipe/lavapipe) and wgpu then finds a Vulkan adapter "llvmpipe ... (Cpu)", with timestamp queries. `examples/gputime.rs` ran on it and its dish matched the CPU dish's measures.

**How to apply:** use it to check that wgpu compute or render code runs and gives the right dish. Its timings are a CPU emulating a GPU, never GPU numbers; real timing is on the owner's GTX 1070 via Remote Control. The install does not persist across containers. See [[gpu-plan]].

**llvmpipe hides races (2026-11-12, GPU thread):** workgroup squad passes that shared cells between threads through a storage buffer (read by one thread after another wrote it, a storageBarrier between) passed every twin test on llvmpipe but lost updates on the GTX 1070 (Vulkan): the Larder World bench's game changed from run to run. The 1070 also failed the older shot-alarm twin test written the same way. **How to apply:** in one dispatch, give each cell to one thread (the cell's index decides the thread) instead of relying on barriers between threads for storage-buffer data; any new pass where threads hand storage data to each other gets the squad twin tests run on the 1070 (`MOULDGPU_TEST_GPU=1 cargo test --release --locked --bin mouldgpu squad`) before merging. Record: Reports/gpu-readout-2026-11-12.md, *On the GTX 1070*.

**The 1070's float maths differs from the CPU's (2026-11-12, GPU thread):** `sin`/`cos` differ in 91% of results (to 1.1e-6), `sqrt` in 17%, `exp`/`pow` by about 1 ulp, and the NVIDIA driver always fuses `a*b+c` into an FMA (lavapipe never does). One-tick twins still pass; game-length twins drift (2 of 29 fail on main, no lean over 32 seeds). **How to apply:** don't chase a drift-only twin failure on the 1070 as a bug; compare against main's baseline. Exact cross-machine results would need integer or table maths (HANDOFF decision 6).
