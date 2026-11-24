# Crystals cut as gems; spent ones broken (2026-11-24)

Look lane, Milestone 2's Look C (planets and terrain), the polish
review's third and fourth findings ("crystals look like clip-art cubes",
"a spent crystal reads as a grey umbrella"). Page:
https://claude.ai/artifact/8U9XRkm4Wo9MjSq9TBEDSw (source `index.html`
here, clips beside it).

**Before.** A crystal was one hexagonal pyramid of slope 9 tall(v) / a:
six faces lit in three tones, a cube icon, the same on every planet. A
spent husk was shaded at 0.45 of full height, albedo 66-76, in eleven
spoked shards: as big and nearly as tall as a live one.

**After** (`shaded` in `src/world/view.wgsl`, `GEM_*`):
- from the edge in, a girdle bevel to 12% of the way to the middle
  (`GEM_GIRDLE`, slope `GEM_BEVEL` 2.4), a crown to 58% (`GEM_TABLE`,
  slope `GEM_CROWN` 0.85) whose six faces each split at the side's middle
  and tilt `GEM_STAR` 0.38 rad apart (twelve tones), and a flat table at
  70% of the face colour;
- a dark seam a pixel wide where facets meet, from zoom 4;
- two short fracture lines a crystal (`GEM_VEINS`), lit by its charge;
- spent: `GEM_HUSK` 0.2, `GEM_SPENT` 56-64 (about rock's value), 7 shards
  (`GEM_RAYS`), finer cracks, and under half charge the rim crumbles in by
  up to `GEM_CRUMBLE` 0.45 of the way by two octaves of noise
  (`gem_crumb`); the cast shadow follows the crumble.

The charge's height and glow, the take's white-hot glow, the slime's film
over a crystal and the cast shadow are as before. Cost: a crystal's own
pixels only, a few dozen operations each (two fracture segments, three
smoothsteps); none elsewhere.

**Tried and dropped:**
- fracture lines right across the stone: scratches on its face;
- cyan light pooled on the ground round a full crystal (1.4 cells): barely
  seen at zoom 6, and a blur past the crisp edge (art rule 4;
  `crystals_are_their_hexagons_to_the_pixel` failed on it);
- a crumble of one noise at 1.7 a cell: a spray of pixel islands at zoom
  12;
- a moving glint (suggested by the review): not from the simulation.

**Clips:** planet 1, run seed 1, `hand=strat:econ,go=4,aim=6`,
`centre=210,47` at zoom 6 and 12, 20.0-21.8 s; planet 2, run seed 1,
`hand=strat:bot`, `centre=squad,12`, 57.2-59.0 s (with a frame at 40 s:
the `at=` list changes the bot's game, so both sides use the same list).
16 frames at 120 ms (mp4); only the view shader differs (`LARDER_VIEW_WGSL`).
