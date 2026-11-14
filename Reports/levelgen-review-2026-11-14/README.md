# Art review of the procedural planets, 2026-11-14

The owner (11-14 15:44Z): "Please review the level generation work when it
has outputs for you to review". This reviews PR #277 (branch
`claude/project-thread-0tcg97` at 9bb43c8), the roguelike's eight biomes
(level generation report, `Reports/levelgen-2026-11-14/` on that branch), against the
[art direction](../../wiki/looks/art-direction.md).

Frames: the generation thread's whole-map films (`map=planet:D:S seed=S
hand=idle`, llvmpipe, 1280 x 800), the 0:10 frame of each, seeds 1 and 2, in
colour and grey (`page/`, https://claude.ai/artifact/6yJ7zce7Ak8k28aonrNxQc).

## Findings

1. **Most biomes don't read in grey.** Barrens, Canyon, Claypan and Spires
   are the same picture: dark ground, mid-grey patches, short rock sticks.
   Marsh's water has the ground's value. Dunes and the ringed Thicket and Core
   stand apart. Barrens on planet 1 reads as plain ground.
2. **Layout** (level generation thread): one signature shape per biome at
   zoom 1. Barrens in wide bands; Canyon with long unbroken ridges two to
   three times wider (now stubs that read as debris); Spires many pillars;
   Marsh big connected pools; Claypan mostly clay. Dunes' clay islands are
   exact circles that read as holes: larger, noise-edged. Soft-rock gates
   (pale bars) and the Thicket/Core rings (perfect arcs) read as UI: wobble
   and vary thickness so they look grown.
3. **Ground materials** (terrain thread, the review's issue 7): each soil
   kind its own material from the soil field (sand ripples, clay cracks,
   barren ash, wet gloss with a lit edge, rock raised and lit top left);
   sand's value down to just above the ground (on Dunes it is brighter than
   rock). This also gives zoom 6 something to read.
4. **Kept**: nests, crystals (cyan hexagons) and the landing marker.

The test for both: each biome recognisable from a grey 266 x 164 thumbnail.
