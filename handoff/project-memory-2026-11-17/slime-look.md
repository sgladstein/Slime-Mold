---
name: slime-look
description: Larder World slime rendering (graphics thread, 11-14): sharp body #230, material #241, black-spot fix #259, sheen #263, opacity #265; owner reactions and open items
metadata:
  type: project
  modified: 2026-11-14T07:01:57.960Z
---

Graphics thread (cmsg_01PcwPdssrGxiXYvyCG8WaUXKJA1ExyXnFzv5sgkHbtaFn) owns the slime's look in src/world/view.wgsl; the crystals/terrain thread (cmsg_01PcwPdssrGxiXYvyCG8WaUXBdHwiVqQHRK28GRxQ3brv9) owns crystals, rock/wall edges and ground colours (SOIL/SAND/CLAY/BARREN, WET).

- #212 (11-14): panic/withering visible from walker news (C8).
- #230 merged 06:12Z: fine-grid walker count (View::fine, 2/cell from zoom 6), hexagon crystals (bindings 18-19). Owner 04:50Z: "edges ... look really good now. The middle still looks blurry"; "single individual walkers should be brighter".
- #241 merged 07:01Z: lone walkers bright (they drew darker than ground on main), body keeps 40% sharp count, wet gold material, slope cap 0.7, grey/amber/red ramp by news age. Page JXq4Sz5Pg1jn5sHHzM9EMv. llvmpipe +4%.
- 11-14: #259 merged (crater/scorch walkers drew black: hue normalised from a near-zero engine tint; falls back to MOULD_HUE). #263 merged (flat frontier, broad sheen; page 2gtoXBwm5tGsg5pA5at7rW). #265 merged 11:37Z: body opaque from th 0.05-0.35 (fork clay patch showed as a disc).
- Speck word: calm 0-9, lean 10-19, bolt 20-31; asked the condense thread to keep streaks in their own buffer (overflow paints false red).
- Bindings: 17 fgrid, 18 gems, 19 gem_cell, 20 point lights (gun rounds), 23 tcell (#228).
- Start shape (11-15, PR #319 queued): Larder-world starts are seeded organic outlines (`BigParams::start_shape`, on in race_params + GPU Settings), not discs; CPU/GPU twin fixtures keep the disc. Page Ttr5AuoZ4jH6QYGpWWtJrz.
- Dead view code (11-17, after #422 removed condensing): DENSE_RGB, fgrid FG_DENSE/FG_ABSORB, draw_absorbed, dense_share in cell_mat_c, ripe_at uniform. Graphics thread removes it next time in view.wgsl; keep lump bits 25-31 and set_dense binding. Set-slime look (simulation-use thread cmsg_01PcwPdssrGxiXYvyCG8WaUXMnf4G8wzvcSUjzucRhDDEx) is editing near lump_mat, so do the cleanup after theirs lands.
- Open: 1070 timing of #230/#241 (PC busy); owner's verdict on the material; grey contrast relies on the darker ground coming from the crystals thread.

**Why:** later threads touching view.wgsl need to know who owns which lines and what the owner said.
**How to apply:** coordinate shared view.wgsl edits with the owning thread; merge through the `queue` label.
