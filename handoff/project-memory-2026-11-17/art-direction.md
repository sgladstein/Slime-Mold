---
name: art-direction
description: Art director thread (11-14) leads all Larder World graphics; style guide wiki/looks/art-direction.md; review page and who owns which visual issue
metadata:
  type: project
  modified: 2026-11-14T02:34:26.879Z
---

Owner 2026-11-14 02:24Z asked for one thread to review all graphics, manage graphics work and start new work: thread cmsg_01PcwPdssrGxiXYvyCG8WaUX9zcGHdA2XYJgcvA8tL8cUU (art director). Style guide: wiki/looks/art-direction.md ("living light on dark ground"). Review: Reports/art-review-2026-11-14/ (merged #222, c5cc049, 11-14 02:44Z), page https://claude.ai/artifact/6FHfyB88A8Mtk5cQkkuVDm.

Who owns what (11-14):
- slime sharpness cmsg_01PcwPdssrGxiXYvyCG8WaUXKJA1ExyXnFzv5sgkHbtaFn: slime focus + slime material (veins, gloss, contrast)
- crisp crystals cmsg_01PcwPdssrGxiXYvyCG8WaUXBdHwiVqQHRK28GRxQ3brv9: all terrain edges (crystals, rock, walls, rubble), then open ground; takes over the slime thread's hexagon code
- gun rounds cmsg_01PcwPdssrGxiXYvyCG8WaUXNMGAy5sBZGmGbjVvVaNv7S: bullets (#219), gas cloud
- explosion cards cmsg_01PcwPdssrGxiXYvyCG8WaUXH94pm2aUm5hhHCLJ6fWv5g: blasts; share impact light with gun rounds
- gun card HUD cmsg_01PcwPdssrGxiXYvyCG8WaUXEaehHcWBVGebEDMRCYeL1V: compact card strip, then HUD layout (<=15% of screen)
- units and buildings cmsg_01PcwPdssrGxiXYvyCG8WaUXXftESW48xjfbBSn6gxVhcn ("Make soldiers and buildings read clearly") (marks.rs, sprites, draw.rs units part)

**How to apply:** new graphics work checks the guide and this split first, to avoid two threads in view.wgsl at once. Cloud renders: apt-get update && install mesa-vulkan-drivers, then `larder-world map=fork look=DIR hand=volley rounds=X centre=X,Y,ZOOM at=...`. Map editor (#209) not yet reviewed. Related: [[world-look]], [[gun-rounds]].

Merge queue live 11-14 04:20Z: label `queue` to merge, no merging main in again (CLAUDE.md). Order set by art director: #230 sharp slime before #228 terrain edges (#228 resolves view.rs/view.wgsl). #227 units merged; #232 group rings + ghost icons.

11-14 08:15Z owner: slime grows OVER crystals (film by thickness, crystal still reads; crystals thread). Turrets need to look better (units thread). After page https://claude.ai/artifact/RuF8Qyn3518CMbBdyUbsqk (PR #248). Open: frontier pebbles + foil specular (slime), spent crystals + ground/tint discs (crystals), map editor review (me).

11-14 16:20Z planet review (owner asked 15:44Z to review level generation): page https://claude.ai/artifact/6yJ7zce7Ak8k28aonrNxQc, PR #279 queued. Biomes don't read in grey; layout fixes with levelgen thread (next PR after #277), soil materials (sand ripples, clay cracks, barren ash, wet gloss, raised rock, sand darker) sent to crisp crystals/terrain thread. Re-review both when they send GIFs.

11-14 21:20Z: planets pass the grey-thumbnail test (#302 queued: deep water, bridges, plants). Ground materials #281/#286/#292 (terrain thread; soil detail ~16 ms llvmpipe on River, 1070 timing pending). Plants thread cmsg_01PcwPdssrGxiXYvyCG8WaUXWqAyC8qQ4TM2NdtPPPpKm8 told: muted dark blue-green below slime value. Open: Core planet too busy (open arenas, later pass, levelgen thread).

11-14 22:31Z 1070 A/B (main 2b1a851 vs 004b7cb, fork farm): day's ground/plants/fire add (NOT #305: it conflicted, re-queued 22:58Z, now slightly cheaper than main on llvmpipe) ~+1.9 ms GPU at every zoom (z1 3.9->5.7, z6 1.4->3.4, z12 1.0->2.9); frame p95 4.3-6.6 ms. llvmpipe overstates ~10x relative. Raw: claude/project-thread-26drse @3df31c1 runner-out/gfx-ab-2b1a851c-vs-004b7cb/. Ask for another 1070 run after ~+2 ms more.

11-15 01:20Z 3-reviewer planet review (artifact Kcy1AoquWJjJ3EciyA3KxP, Reports/levelgen-review-2026-11-15/). Drawing items split: ground thread (BdHw) #2 value order (grey targets: ground 20-45, rock 50-60, soft rock 60-72, thin film >=80, crystal 110+, thick slime 140+), #1 shade below zoom 1, #9 shallow vs deep water, #7 cave backdrop, #10 hedges not tubes, #11 rock bevel, #3 ground signature at zoom 12. Plants thread (WqAy) #8 detail under slime, fire brighter + red-orange hue, #3 plant signature. Plants owns plant/char/fire fns in view.wgsl, ground owns rest.

11-15 08:04Z 1070 A/B main 17f2e5b vs 2b1a851 (fork): GPU p50 z1 6.0->2.4 (far-view path), z6 3.4->3.8, z12 2.9->3.3; frame p95 <6 ms. Raw claude/project-thread-26drse @f995a04. Planet-review drawing items all merged (#315, #317, #328); walked film p10 73-78 on jungle/highlands left as is (edge sharpness wins).
