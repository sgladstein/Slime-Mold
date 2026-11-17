---
name: crisp-crystals
description: crystals/terrain look thread (11-14): crisp rock #228, dark ground #247, slime-over-crystals #253/#260, spent stone #260 merged; planet soils #281/#286 merged; river look #292 merged
metadata:
  type: project
  modified: 2026-11-14T07:42:30.669Z
---

Thread cmsg_01PcwPdssrGxiXYvyCG8WaUXBdHwiVqQHRK28GRxQ3brv9 (crystals and hard terrain look), from the owner's 11-14 playtest "crystals are pixelated" and 04:53Z "harvesting crystals could look way better".

- #228 merged 2026-11-14 07:42Z: rock/wall/rubble drawn at screen pixel (`terrain_over`, `tcell` binding 23, `look::FAR` skip); crystals read live engine charge per cell (`View::set_charge`, `View::set_time`, `drain` pass, bindings 21/22/24), sink and grey where taken, glow white-cyan while drained, grey stub when dry.
- View bindings: 15-19 slime thread, 20 lights, 21-24 this thread.
- Cost llvmpipe GPU tail p50: main 54 ms -> 60 ms (mostly terrain). 1070 timing pending (PC busy).
- Pages: Crisp Rock Edges DFSpBUyJ65uDB1nDAn2R5N, Crystal Harvest Look WGmUSQDUZxYdfF1rtSco18.
- #247 merged: dark slate ground (SOIL/SAND/CLAY/BARREN/WET constants). #253 + #260 merged: slime grows over crystals as a film (owner 08:15Z ruling; 08:55Z "still looks under" -> cover 0.92, edge line under film); cell_colour recovers mould g on crystal cells from engine red channel; spent crystals dark stepped stone (GEM_STUB 0.1).
- Scent-haze redraw tried, no visible change, dropped. Orange disc at fork start crystal = clay soil patch under half-opaque slime (slime thread fixing opacity).
- #281 (queued 11-14 ~17Z): per-soil materials in view.wgsl (`soil_share` cubic B-spline, `soil_detail`; sand ripples, clay cracks, barren speckle, smooth wet edge + shore, rock lit rim). llvmpipe +6 ms (75->81). Claypan/Thicket/Spires still close in grey thumbnails. Page 7Cyz76JZc5TVJnBDhhHw8y.
- #286 merged: bent sand ripples, darker clumped ash; per-cell soil bake tried, no saving (dead-ends). #292 (merged 11-14 19:56Z): deep water (soil 4) darkens to middle via ring reads, shallow margin/bank, plank bridges (soil 5) with rails/shadow; WET (5,14,22), soft rock 128 max; llvmpipe River +12.5 ms. Deck height in cell_height fades soil texture (reads as mould) - don't. Page EUtwNmz986oPhghQLioRhv. Map shapes are the ground-kinds thread's.
- #305 (merged 11-14 23:11Z): crisp ground edges; 2x2-uniform early-out skips the spline; plants now drawn by plants thread crowns (#304), not in soil_detail at zoom 12-24 (owner: play is zoomed in). hcell is now 3 grids: total height, mould-only height (film_th, spec normal), packed bare ground colour (ground_est). Plant height 3x3-smoothed. llvmpipe +18 ms. Page HdYmS22KYVsHm1Ci1NMWg4. Level gen's 1-cell stair biome edges remain in data.
- #317 merged 11-15 05:56Z (value order, crystal floor 112, gated film lift, hedges, far view). #328 queued: plain_soil(e)=slate x biome_tint(P.flora) on every soil path; plain_look per biome (zoom 3-8); thin-film floor 95 gated by th; reviewer measure film_p10 counts walked pixels only (art lead: keep the edge, don't chase film_p10_all). Pages GgsaFe2YaUfMfm6AFnjNMb, 8y8gTNb9HGuk9qAJYSyzxy. 1070 A/B (art lead 11-15 08:05Z, main 17f2e5b vs 2b1a851, Fork): zoom 1 6.0->2.4 ms (far_cell), zoom 6 3.4->3.8, zoom 12 2.9->3.3; nothing to cut. #328 merged 06:29Z. Put dead-ends/pages lines mid-file: end appends conflict in the queue.
- Slime thread owns mould colour lines in cell_mat_c.

**Why:** keeps later threads from redoing bindings or the ground split. **How to apply:** new view bindings start at 25; coordinate cell_mat edits with the slime thread ([[world-ui]]).
