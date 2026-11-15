# Aesthetics review: procedural planets at 4494ad4f

Saved by the levelgen thread from the reviewer's hand-back (the harness
blocked the reviewer from writing it). Pictures and measurements are in
`/home/user/review-aesthetics/`.

**The 6x core world's whole-map view and its minimap are drawn by a different, unshaded picture at 1280x800 (finding 1).**

All frames come from the prebuilt `larder-world` on llvmpipe, `hand=idle`, 1280x800 unless stated. Planets 1-8, seeds 1 and 2: the whole map at 1 s and 60 s (`frames/main/`), plus 116 close views at zoom 6, 12 and 24 (`frames/close/`). Numbers are grey luma, registered to the map files (`scripts/measure.py`, `measures-table.txt`). Nothing was built; the container restarted once mid-run and the missing renders were redone.

## Big picture

1. **The biomes differ only in layout, and layout can't be seen at the owner's zoom of 12.**
   - On planets 1-6 the ground's mean colour stays inside (32-53, 44-56, 35-53), with median luma 33-51. Every biome is the same dark green-grey with different shapes on it.
   - That passes at zoom 1. Close up, only Jungle, Highlands, Caves and Barrens can be told apart.
   - Each biome needs something you can recognise at zoom 12, drawn from its own fields.
2. **The run doesn't build toward the core.**
   - The palette stays flat from planet 1 to 6. On Caves it jumps: the screen becomes a mid-grey rock field (median luma 98).
   - On the core world it falls back again; the core looks like a busier Meadow.
   - The slime's share of the whole map at 60 s falls from 0.24-0.26 (planet 1) to 0.10 (P5), 0.05 (P7) and 0.03 (P8). The slime's home world shows the least slime.
3. **What the earlier tests missed:**
   - No frame below zoom 1 was looked at, so the 6x far view went unseen (finding 1).
   - The value checks never compared walls and rock with the thin slime film.
   - Every thumbnail is dominated by the slime's start disc, the same bright circle on every biome.
   - There was no close-zoom identity test, and nobody looked at the 6x minimap.
4. **Ideas from other games, done from this simulation's own fields:**
   - **Late planets:** Nuclear Throne and Enter the Gungeon change palette per floor, and Mindustry's late sectors have spore moss. Here the deeper planets could carry the slime's history: pre-run it off screen so its trail, and the char of the plants it ate, stain the ground. The core world would arrive already digested.
   - **Caves:** The Riftbreaker and Noita light their caves with whatever glows. Caves could lose the sky light so only crystals and slime light the caverns.
   - **Close-zoom identity:** RimWorld and Don't Starve give each ground type its own texture and plant species, so a biome reads at any zoom.
   - **Water:** Noita's look is change. Here water never moves: deep water is one flat colour, (11,16,29).

## Tests

1. **Grey thumbnails** (`thumbs-001s-grey.png`, blind copy `thumbs-001s-grey-blind.png`, colour `thumbs-001s-colour.png`).
   - By eye I can name 5 of 8: Caves, Core, Jungle (rings and hedge worms), Highlands (crack net) and Barrens (stripes).
   - Meadow, Fen and River valley are all dark ground with black water.
   - A simple image matcher pairs each seed-1 thumbnail with the right seed-2 biome 6 of 8 times; Fen and River valley both land on Meadow.
2. **Close-zoom identity** at zoom 12 with no nest or crystal in frame (`ident-z12-colour.png`, `ident-z12-grey.png`).
   - 4 of 8 by eye. The matcher scores 4 of 8 in colour and 2 of 8 in grey.
   - Meadow, Fen, River valley and Core are interchangeable; Core at zoom 12 is just a meadow.
3. **Slime against every ground** (`measures-table.txt`).
   - The thin film (10th percentile 49-86, 25th percentile 68-119) is above every bare soil. Only 1-10% of newly covered pixels are not at least 10 levels above the ground under them.
   - It is below rock (98) and soft rock (107-123) on all 14 planet-seeds measured.
   - Jungle: film 49-51 against lit canopy tops at 87.
   - Meadow seed 1: film 71 against grass and sand at 81.
   - Slime over deep water: 32-38, darker than every bare soil (42 and up).
4. **Crystals.**
   - In colour they pass: crystals are the only cyan on the map. The only stray cyan pixels are the tops of small crystals' hexagons.
   - In grey they fail. Crystal median 93-126 is under soft rock (123) on 12 of the 13 planet-seeds with walls. Their dark faces (66-77) are under rock (98).
5. **Seed variety** (rows of `thumbs-001s-colour.png`): the rolled settings do give different maps (Meadow seed 1 is streams on sand, seed 2 a lake on clay). Caves and Core seeds are the same picture rearranged.

**The three most alike pairs:**

| Zoom | Pairs | Pictures |
| --- | --- | --- |
| 1 | Meadow/Fen; Barrens/River valley (the matcher's closest pair); Jungle/Highlands | `alike-pairs-zoom1-*.png` |
| 12 | Meadow/Fen (closest in both colour and grey); River valley/Core; Fen/Highlands (the same clay-crack mesh) | `alike-pairs-zoom12-*.png` |

**In motion** (`gif-front-p{1,3,4,5}s1-z12.gif`, `gif-fire-p1s1-z12.gif`; stills in `strip-*.png`):
- From 55 to 59 s the slime front stands still at the reeds and the water; only the body shimmers.
- Fire is a soft orange-red blob about 40 px across. Its brightest pixels (179) don't beat the slime's (181), so burning isn't the brightest thing on screen (rule 2), and its hue sits right beside the slime's amber (rule 3).

## Each feature

- **Ground:**
  - Rock is a flat slab at luma 98 with no texture. At zoom 24 its lit rim is crisp, but the shaded side is a soft 20-25 px band (`crops/p1s1-z24-rock-edge.png`, rule 4).
  - Where a soft-rock gate meets rock, the seam is a 25 px blur (`crops/p3s2-z12-hedge-across-water.png`).
  - Barrens at zoom 12 is flat, empty grey (`frames/close/2-1-ident-z12/`).
  - Caves: rock is (96,98,102), the same colour as the backdrop outside the world, so the map's edge vanishes. The caverns read as a node graph (`caves-p7s1-fit-rock-equals-backdrop.png`).
  - The core world's arenas can't be read at 6x (`core-6x-fit-1600x1000-shaded-s1.png`).
- **Water:**
  - Lakes and reed fringes look good.
  - Deep water is flat, and a cell-sized stair shows at the deep/shallow line when contrast is stretched 6x (`crops/p4s1-z24-deep-water-x6-contrast.png`).
  - Fords are only 6-7 luma off deep water: invisible at zoom 1, and box-shaped (`crops/p4s1-river-fords-zoom1-and-x3.png`).
  - The bridge deck inherits the river's rough outline and has plank artefacts (`crops/p4s2-z24-bridge.png`; levelgen.rs:1441-1443).
  - On Core seed 2 the streams are a web over the whole map (`core-6x-fit-1600x1000-shaded-s2-streams.png`). levelgen.rs:619 and 643 scale the stream count with the square root of the area.
- **Soil:**
  - Sand ripples and dunes look good. Barren bands read at zoom 1 only.
  - Clay cracks are a uniform mesh that reads as floor tiles at zoom 12, and they are drawn through the slime (`crops/p1s2-z12-clay-cracks-over-slime.png`).
  - Neither seed rolled an oasis.
- **Soft rock:** hedges are beige tubes with a soft shadow. They are brighter than crystals, cross lakes and don't look grown. The rings round nests make bullseyes (`frames/close/8-2-nest-z6/`).
- **Plants:** the jungle canopy is the best close-zoom look in the game. Patch edges are smooth at zoom 24, so that problem is fixed. Plant crowns show as black specks through the slime (`crops/front-p3s1-z12-mid.png`).
- **Crystals:** good in colour; they fail the grey check (test 4).
- **Nests:** 6 of the 16 first nests sit 55 world cells from a map edge, so the slime's home is cut off by the frame (`frames/close/2-2-nest-z6/`). At 1 s every start is a perfect gold disc (`sheet-p4-5.png`).
- **Landing:** a clean pad.
- **Size and minimap:** see finding 1.
- **Fallback map:** no planet used it, so I never saw it.

## Already known: fixed or not

- **Busy core world:** not fixed, and worse at 6x (seed 1 is covered in rock fragments, seed 2 in the stream web).
- **Plants edge to edge on the bare biomes:** fixed (Barrens 10.6% cover, Caves 4%).
- **The old grey-thumbnail failure:** improved, but Meadow, Fen and River valley still blur together.
- **Soft rock as bright as thin slime:** still true (123 against the film's 68-119).

## Findings, most serious first

1. **CONFIRMED. Below zoom 1 the view skips its shading** (`src/world/view.rs:731-737`, `cam.zoom >= 1.0`).
   - The 6x core world fits at zoom 0.857 at 1280x800. Its whole-map view and minimap switch to a different picture: navy ground, brown rock, and flat crystals brighter than the slime.
   - Compare `core-6x-fit-1280x800-zoom0.86-far-view.png` with `core-6x-fit-1600x1000-shaded-s1.png`, and see `crops/minimap-p8s1-6x-vs-p3s1-1x.png`.
   - The same happens on any screen under about 900 px tall.
   - **Fix (drawing):** run the shaded path below zoom 1 at `fine=1`; failing that, give the far view the shaded view's colours.
2. **CONFIRMED. Value order is broken on all 14 planet-seeds.** Rock (98) and walls (107-123) are brighter than the thin film (10th percentile 49-86), and walls are brighter than crystals (rule 2).
   - **Fix (drawing):** rock to about 70 and soft rock to about 80, or a film floor of about 110. Rerun `scripts/measure.py` until the film beats the walls everywhere.
3. **CONFIRMED. The core world is no climax:** Meadow's palette, the least slime on screen (3%), the stream web, bullseye rings.
   - **Fix (levelgen, with the run thread):** pre-grow the slime by area so its trail and char stain the ground; cap streams at the 1x count; break the rings into two or three offset arcs.
4. **CONFIRMED. Clay cracks and plant crowns are drawn through the slime.** `view.wgsl:1592-1593` fades them by the film's computed thickness (`film_th`), not by the film as drawn.
   - **Fix (drawing):** fade them by how much the drawn film covers.
5. **CONFIRMED. Caves:** rock is the backdrop's colour, rock fields are flat, and the layout reads as a node graph.
   - **Fix:** a darker backdrop, and darker rock with a texture (drawing); irregular caverns and tunnels of varying width (levelgen).
6. **CONFIRMED. Close-zoom identity is 4 of 8.**
   - **Fix:** a zoom-12 signature per biome from soil materials and plant kinds (drawing), plus signature ground such as cracked ash on Barrens (levelgen).
7. **CONFIRMED. Hedges read as UI, and gate seams blur.**
   - **Fix:** a plant-like hedge material and a crisp joint (drawing); keep hedges off water (levelgen).
8. **CONFIRMED. Fords can't be seen, and bridges are ragged.**
   - **Fix:** a straight deck that runs past both banks and a ford edge without the roughness noise (levelgen.rs:1441-1446); shallow water at least 15 luma above deep (drawing).
9. **CONFIRMED. Soft bands up to 40 px wide:** the rock's shaded side, slime over water, fire (rule 4).
   - **Fix (drawing):** a 2 px shade plus a cast shadow; a crisp fire edge, with fire brighter than the slime.
10. **CONFIRMED in these renders, SUSPECTED in a real run** (it depends on the run's slime tier; these renders used the default 25k walkers): the slime's share of the screen falls with depth.
    - **Fix:** scale starting walkers with area (run thread).
11. **CONFIRMED. Nests sit on map edges, and every start is a perfect disc.**
    - **Fix:** keep nests at least 120 world cells off the edge (levelgen); a ragged start (slime thread).
