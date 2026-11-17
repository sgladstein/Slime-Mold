---
name: larder-world-merge-order
description: GPU engine PR order for Larder World; PARAMS_BYTES 864 after #108 (one body); readout #101, step split #110 merged; #148 merged (1070 speed); #157 merged (crowd_holds, crater atomics); integration thread runs it
metadata:
  type: project
  modified: 2026-11-12T20:00:00.000Z
---

Agreed 2026-11-11 18:58Z between the Larder, graphics and GPU threads: the GPU engine PRs merged in the order #63 → W0 (#73) → W1b + W2a (#74) → #69 (look) → species (#82) → life cycle (#89, 8da1cca) → soil (#92, 19a8765).

- Bindings: W0 owns 32 (energy); W1b 33-41 (binding 8 agar read_write); W2a 42 (squad `SQ`) and 43 (kills); #69 44 (hits) and 45 (look). One body (#108) took 46 (packed sums+counts); the toolkit takes 47.
- Params: squad row at 416 (`SQUAD_AT`), look's `nhits` row at 432, species block, life-cycle rows (`STRIKE_AT` 800), soil row, then one body's `BODY_AT` 848: `PARAMS_BYTES` 864. Real asserts since #94. Since #194 (2026-11-13) ROADMAP's GPU engine section lists the spare Params words by byte offset: claim one there in your own PR (rename it in WGSL and write_params) before a new 16-byte row, which is booked through integration.
- Passes: #108 added esum/egain (34-35); the readout's `count` pass is 36 (`PASSES` 37).
- PR #101 (GPU thread, readout) merged 2026-11-12 (a90c71e): no new row or binding. The world row's 4th word (was `_w0`) is `grid` (mould grid square, 0 none). `wstat` is `WSTAT_BYTES` 32 (bites, broken, evictions, struck, living, 3 spare) then the grid.
- PR #148 (GPU thread, 1070 speed, merged 17:44Z, 2787af3): squad passes `units` (a workgroup a unit), `smell`/`shout`/`harvest` on owned cells (`owned_from`, no cross-thread storage RMW); old one-thread passes kept as `smell1`/`units1`/`harvest1` behind `squad1=1`. `GpuWorld::spread` + `Engine::record_share`: the window spreads a step's ticks over the frames until the next (`frame_more`, `flush` before take/grid/hash); a command mid-step flushes. Bench keys `passes=`, `settle=`, `squad1=`, `spread=`; machine line first and last. GPU PRs run the 1070 twins first (wiki gpu-sandbox *Twins on the 1070*; baseline 27 pass, 2 drift fails).
- PR #157 (GPU thread, merged 20:07Z, e8d9501): `gpu::crowd_holds` (start walkers <= agar/2; slots no longer count; `made` warns), and binding 47 `crater` is `array<atomic<u32>>` of f32 bits (`crater_at`/`crater_set`, `cut` atomicMax). The toolkit's `hit` crater write converts to atomicMax if #157 lands first.
- Since 2026-11-12 the integration thread (cmsg_01PcwPdssrGxiXYvyCG8WaUXVvB9idhrH7XKQHoSEmCnSB) owns the queue for `src/mould/larder_world.rs`: controls #102 (merged) → the GPU thread's step split #110 (merged 07:24Z, 153e447: `Game::begin_step`/`finish_step`, `settled_ticks`, grid once a game second `GRID_EVERY` 15, `GRID_MIN` 4) → buildings. Window code that reads the game between frames keys on `settled_ticks`; `Game::apply` finishes a running step first. Since #116 (10:07Z) the window's HUD events go through `App::hud_event` (one event, `Result<bool, String>`), and CI's smoke tests `the_window_plays_every_mission` / `the_window_takes_every_command` (larder-world bin) play every mission file and every card command through the bench; threads keep them green.

**Why:** these all change the engine's bind group, Params or the rules file; resolving the same conflict twice in different threads would diverge.
**How to apply:** don't merge out of order or resolve another thread's conflicts; ask the owning thread. See [[mould-shooter]], [[rts-direction]], [[tech-lead-review]].
