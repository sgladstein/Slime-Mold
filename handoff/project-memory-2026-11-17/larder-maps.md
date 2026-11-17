---
name: larder-maps
description: Owner 2026-11-11 19:29Z: designed maps in the Larder World plan; ROADMAP WM1-WM5, owners, loader format; height parked 20:06Z
metadata:
  type: project
---

Owner 2026-11-11 19:28-19:29Z, after seeing the world drawn as a black background: "Is it part of the roadmap to build an actual world?" ... "We will want maps though. That should be part of the plan".

Plan (ROADMAP.md rows WM1-WM5, landed with PR #74):
- WM1 map file and loader (ground kind per cell, crystals, starts; CPU big dish and GPU engine; noise worlds saveable as maps). Larder thread, built alongside W2b so the first W2c play is on a map.
- WM2 three to five designed maps, gated by W2b's harness (idle and random lose; the winning plan changes between maps). Maps thread "Design Larder world maps" (cmsg_01PcwPdssrGxiXYvyCG8WaUX6ziKAgVSdbHha1iCGeZNR9), started by the coordinator 19:33Z.
- WM3 new ground kinds the simulation uses, brainstormed from the model (neutral brief, see [[brainstorm-briefs]]). Maps thread; GPU side with the Larder thread. **Height parked** (owner 20:06Z: "I am skeptical how much fun/complexity/strategy this adds"; start from the graphics thread's terrain, #69): test sand, clay, barren first; byte 3 stays 0.
- WM4 the ground's look from those fields: graphics thread. Agreed 19:33Z: byte 3 (bits 24-31) of the engine's ground word (binding 34) is elevation; the view shades it only once the simulation uses it (no drawn-only effects).
- WM5 map editor, after W2c.

**Why:** open agar draws black because the simulation stores nothing there but trail and scent; under the "picture is the data" rule a real ground needs ground that does something. **How to apply:** ground features must be simulation fields first, then drawn. See [[larder-world]], [[larder-world-merge-order]].

**Maps thread, 19:50Z (PR #76, Reports/larder-maps-2026-11-11.md, page https://claude.ai/artifact/XPwkBkTAzsWq3Wg37pff7K):** five paper maps, each drawn for a player-AI style. Its height-first WM3 order was superseded 20:09Z (height parked; soil first).

**Loader format (agreed 19:40Z, files in Reports/larder-maps-2026-11-11/maps/, PR #77):** maps/<name>.png RGBA 1024x640 (R height, now 0; G the ground word's bits 8-15: WET 0x01, ROCK 0x08, soil in the high nibble; B 0; A 255) plus maps/<name>.txt lines `crystal x y r charge`, `mould x y`, `landing x y`, `wall x0 y0 x1 y1 thick hp` (thick = full width, as build_wall's width; soft rock = wall lines; no squad line; mould lines carry no tier). No JSON crate. The maps thread will say which fields the rebuilt maps carry.

**21:30Z, after owner's 20:06Z challenge ("skeptical" of height; start from the graphics thread's terrain; 20:09Z "You can improve upon their work too"):** height parked. Soil tested (PR #83, examples/worldground.rs, 16 seeds): wet halves the mould (16/16), barren halves it, sand makes it ~60% bigger (walkers leave sand but explore), clay traps it. BigDish::soil + SOIL_SAND/CLAY/BARREN, BigParams sand_keep/clay_keep/barren_upkeep. Maps v2 flat: Rim, Fork, Flats, Nest, Passes (crystal r in 1024 cells, doubled). Next, agreed with graphics thread (20:29Z): add soil materials to view.wgsl cell_mat (ground bits 12-15) after #69 merges; worldlook's terrain block → map loader (Larder thread's WM1).

**2026-11-12 03:50Z, soil on the GPU (Larder thread, PR #92, merged 19a8765):** the engine plays sand/clay/barren (ground word bits 12-15, WORLD_SOIL 16, Params 848 bytes with a soil row after the strike row); one-tick twin to the bit, the four soil maps' game twin within 2.1%; the window draws soil. In play the Rim's sand ring holds the mould in its clay core (page https://claude.ai/artifact/1ifC4Qrw6fXe5Aq7vVSRzZ). No map uses barren yet.
