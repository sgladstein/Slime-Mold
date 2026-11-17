---
name: ground-kinds
description: New ground kinds thread (11-14): deep water (soil 4 + wet) and bridge (soil 5), PR #285 merged 11-14 20:13Z; who owns the look; kinds 6-15 numbering
metadata:
  type: project
  modified: 2026-11-14T17:54:29.144Z
---

Thread "Add new ground kinds, deep water first" (cmsg_01PcwPdssrGxiXYvyCG8WaUXKkGDeYK84BCPx2GaPHXDpk, branch claude/project-thread-3d49sr). Started from the owner's 11-14 17:02Z "New ground. Yes!".

- Encoding, agreed with the levelgen thread: G high nibble 4 is deep water, always written with G_WET (`map::G_DEEP` 0x41). Kind 5 is a bridge (`G_BRIDGE` 0x50, dry). This thread numbers kinds 6-15 and tells levelgen each one.
- Rules: the slime steps x `deep_step` 0.4 on top of wet's 0.5 and isn't born in deep water. Soldiers and buildings stay out (`World::open`). Shots fly over (`World::clear`). `map::standable(g)` is the single soldier-ground predicate, and the levelgen planet check calls it. `Map::cut_off` covers reachability for the editor and the designed maps.
- The GPU Params got a ground row: GROUND_AT 928, 944 bytes, three spare words for later kinds.
- The River sandbox map (`map=river`, Reports/larder-maps-2026-11-11/maps/river.*). Page: 9kdKYHvn4wv3zPHvsbfLQy.
- The look belongs to the terrain thread (#281 owner), on the graphics lead's call (17:15Z): sk==4 inside the WET block of cell_mat_c, a bridge base on sk==5, and plank height in cell_height.
- Levelgen #284 changes Map to {w,h,ground,items}. Whichever of #284 and #285 lands second adapts `cut_off` and `river()`.

**Why:** later threads add kinds and must not reuse numbers or fork the standable rule. **How to apply:** new kinds take 6+, and both engines get the change. Related: [[levelgen]], [[crisp-crystals]], [[larder-maps]].
- Never run cargo fmt over the whole repo: it is not fmt-clean and rewrites ~120 files (ground-kinds thread, 11-14).
