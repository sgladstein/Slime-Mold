---
name: plants
description: plants+fire thread: #295, #304 merged 11-14; planet review fixes (under-slime, fire core, flora per biome, fen hummocks, purple core) #315 merged 11-15 04:07Z; 1070 timing to do
metadata:
  type: project
  modified: 2026-11-14T17:34:54.963Z
---

Thread "Build plants as a new simulation" (cmsg_01PcwPdssrGxiXYvyCG8WaUXWqAyC8qQ4TM2NdtPPPpKm8), branch claude/project-thread-vuwr36. Code: src/mould/plants.rs, BigDish::plant/plant_kind/plant_cap, BigParams::plant_*, examples/plantprobe.rs; brainstorm Reports/plants-2026-11-14/brainstorm.md.

**Map format (agreed with levelgen 11-14):** level PNG B = kind<<6 | density(1-63), 0 = no plant. Kinds 0 meadow, 1 jungle, 2 reeds (wet), 3 scrub. Soil 4 deep water, 5 bridge: capacity 0.

**Owner 11-14 17:29Z rulings:** slime does NOT eat plants. Cover: "slime senses by smells so that is a little convoluted. Lets think about it" (undecided; idea: plants muffle soldier scent). Slow ground: test, may be annoying. Fire: yes, but must not make wiping out the whole slime easy. Blocks shots, thickets, slime-eating plants, fertile battlefields: try, playtest for fun. Harvest and sowing: No.

**How to apply:** fire self-limits (living slime is a firebreak); 5-8 behind switches for playtest; any look via graphics lead before view.wgsl. Related: [[levelgen]].

**Graphics lead spec (11-14 17:55Z):** plants muted, below slime value; fire emissive ember->yellow-white, char darkens soil; GIF to graphics before merging; quote frame cost.

**Status 11-14 ~19:00Z:** #290 merged (plants+fire CPU/GPU bit-exact, menu Plants on).

**Owner 11-14 19:18Z on fire:** "Fire should spread 5x slower. It shouldn't just be a wavefront. It should stay burning in some areas... more heterogeneity... a little more challenging to spread... Sometimes it should burn out in areas. ...a heterogeneous moisture field that affects how fast it spreads, how long it burns, and the chance of it going out. It should hurt the player too."
**Fire round 2 (11-14 ~20:10Z):** PR #295 (moisture field, chance-based fire ~1 cell/s dry, smoulder/out where damp, flicker+embers, Rules::fire_hurt 25, UnitOut::heat, GPU unit out 8 words). Moisture from map .txt `moisture BASE SPREAD` (levelgen writes per biome: Meadow .25 .3, Barrens 0 .2, Fen .5 .3, River valley .3 .3, Jungle .4 .3, Highlands .1 .2, Cave .35 .2, Core .2 .35). Waiting graphics advice before queue. Owner order: fire first, then plants look ("mostly just green blobs"). Graphics plants note: muted dark blue-green, below thin slime value, canopy raised darker mass, grass fine texture, no gold; GIF zoom 1 and 6 + grey thumbnails; coordinate view.wgsl with ground thread (#292).
No assignee/review requests on PRs (owner 11-14 19:35Z, emails).

**Look round 3 (11-14 ~22:10Z):** #295 (fire v3) merged. Plants per pixel as crowns sized by sqrt(kind density), green hue 130-150 (teal rejected: crystals own cyan), fire per crown + PB_FRESH block 8 (view only) + near-black char with ember grains: PR #304 merged 22:14Z, graphics lead accepted; awaits owner playtest. Page https://claude.ai/artifact/7ZFRbDuC4ahCxKVV6wDui5; Reports/plants-look-2026-11-14. llvmpipe zoom 12 frame 56.6 -> 66.1 ms; 1070 timing still to do. mcell is 4 words (vis). Levelgen told moisture parser is on main.
**Review fixes (11-15):** art lead items #8 (plants/char/soil gone under slime by mid-film), #11 (white-hot fresh front, luma p99 155->214), #3 (map `flora N` line = biome -> view `flora_form` per biome) in PR #315, merged 04:07Z. Follow-up in it: fen olive-brown hummocks on wet ground (Form.wet, moisture block 7), core flora purple 290-320 (style guide rule 3). Ground thread reads P.flora (Biome as u8) for soils; keep its meaning. Page https://claude.ai/artifact/GfhdLAM5txddNwy9kT43e4. Levelgen pins re-pinned for the flora line.
**Rivers under reeds (11-15 ~12:30Z):** whole-game review item; view only: plants thinner over water (WATER_THIN .65), none over deep, far-view veil (WATER_VEIL .6). PR #349 merged 11-15 13:02Z. Page EAvniuwXaf5D6Nh2AgGsd9.
