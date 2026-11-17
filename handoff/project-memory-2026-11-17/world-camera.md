---
name: world-camera
description: Camera/UI thread for the Larder world (2026-11-11): slime_mould::world (camera, GPU view, marks, controls) and slime_mould::gpu engine library; interfaces agreed with Larder and GPU threads
metadata:
  type: project
  modified: 2026-11-11T14:41:06.366Z
---

Owner asked 2026-11-11 14:06Z for a camera and UI thread for the Larder world, which is moving onto the GPU as a persistent world bigger than the screen.

Split: GPU thread = economy on the GPU; Larder thread = world, crystals, rules and the world binary; creature thread = species; camera/UI thread = `slime_mould::world`.

Built 11-11 on branch claude/project-thread-ccgpci:
- PR #63 (merged 2026-11-11 19:20Z, 5b12fa5) moved the engine to `src/gpu/` (engine.wgsl, pub Engine and Settings) and added `slime_mould::world` (camera, view, marks, controls). The tests stay in src/bin/mouldgpu.rs.
- Wiki page: wiki/world/world-camera.md (moved from rts/ by the wiki thread's #72).
- Graphics thread builds a look pass and a Soldier mark on view.wgsl/marks.rs; GPU thread's W0 (#73) stacks on #63.

Larder thread's interface asks:
- units carry a team id, and only the local team can be selected;
- about 1,000 marks, with rect marks for buildings;
- a fog-of-war hook only (the owner has not decided on fog);
- crystals and terrain drawn into the world frame;
- world-space labels.
The controls emit `UiCommand`s (Select, Order, Burn, Key), and the Larder thread maps them to its own Cmd.

Sandbox keys: W = 2048x1280 world, Q = stand-in squad, M = minimap, middle drag = pan, I = infect (it was the middle button).

Page: https://claude.ai/artifact/X4b5Nmwgkd11paVF8RS1mj

Gotchas:
- never run `cargo fmt` (the repo isn't fmt-clean; it rewrites about 30 files);
- remote control or list_devices needs a message the owner wrote in this thread.

GTX 1070 view cost: 0.33 ms zoom 1, 0.32 zoom 4, 0.42 fit (1920x1080, 400k walkers). Open: the owner has not played it yet. Related: [[mould-shooter]], [[software-gpu]], [[scott-desktop]].
