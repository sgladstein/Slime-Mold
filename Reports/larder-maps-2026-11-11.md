# Larder World maps: ground kinds and five paper layouts (2026-11-11)

Owner, 2026-11-11 19:29Z: "We will want maps though. That should be part of the plan." The Larder thread added a Maps track to the roadmap (WM1 to WM5, PR #74's thread). This report covers this thread's part:

- **WM3**, new ground kinds that the simulation itself uses (brainstorm, ranked).
- **WM2**, three to five designed maps, each drawn for a different winning plan (paper layouts).
- **WM5**, a map editor, which is only noted here.

It is a plan only: no walker has run on these maps. The sketches are drawings of a map spec, made by [`sketch_maps.py`](larder-maps-2026-11-11/sketch_maps.py), which writes [`maps.json`](larder-maps-2026-11-11/maps.json) and the [sketches](larder-maps-2026-11-11/sketches/). WM1 (the map file and loader) belongs to the Larder thread, and WM4 (the ground's look) to the graphics thread. By their agreement, byte 3 of the ground word is height.

> **2026-11-11 21:20Z: height is parked, and the maps are rebuilt flat.** The owner, 20:06Z: "Your map generation is heavily focused on heights. How much does height actually influence our slime mold behaviors? I am skeptical how much fun/complexity/strategy this adds to the game. You should also be starting from the improve solider and slime graphics work which already has added terrain to their map". Nothing below about height was measured. The ground kinds were then tested ([*Which ground the mould feels*](#which-ground-the-mould-feels-2026-11-11)), and the maps redrawn flat from rock, water, soil kinds and soft rock ([*Maps, version 2*](#maps-version-2-flat-2026-11-11)). The sections below are kept as written.

## The brief, stated from the model

The world is a grid of cells. Walkers sense two fields ahead of them (their own trail, and scent given off by crystals), turn, step and lay trail. Trail and scent spread and fade. Walkers eat crystal charge, pay upkeep, divide when fed and die when starved. Some cells are rock (no walker enters), some are wet (a step there is halved), and some are walls with health that walkers bite down. Soldiers walk, shoot along a line that stops at the first slime, and harvest crystals.

So a ground kind can only matter if it changes one of these terms in a cell: **step length, turning, sensing, trail kept, scent spread, upkeep, births, or what a shot or a soldier does**. Every idea below is written as which term it changes. Ideas that change none (a coloured floor, a painted river) are out under the owner's rule that everything on screen is the simulation.

## WM3: ground kinds, ranked

| # | Kind | Term it changes | What you would see | The play it makes | Cost a step (guess, to measure) |
| --- | --- | --- | --- | --- | --- |
| 1 | **Height** (byte 3) | Step × (1 − `climb` × rise ahead), for walkers and soldiers alike | Tubes follow contour lines and run up gullies, not straight up a slope | High ground is slow to take and cheap to hold. Soldiers on a rim shoot down at a mould that climbs slowly | One extra ground read for the cell ahead, which the walk already reads for rock and wet. Close to free |
| 2 | **Downhill drift** (from height) | A share `drift` of each cell's scent moves to its lowest neighbour, total kept | Scent pools in hollows. The mould in a valley smells crystals up the slopes, while a mould on a ridge cannot smell what lies below it | Valley crystals call the mould from far up the valley. A crystal in a hollow is hidden from the plateau. Streams of scent become roads | One more pass over the scent grid (coarse), or a term in the existing spread |
| 3 | **Water table** (from height) | Wet = height below the map's `waterline` (replaces noise wet on designed maps) | Lakes and marsh sit in the hollows, where the slow wet ground already is | Wet is no longer random: it is where scent pools, so the richest smell is in the slowest ground | None (set at load) |
| 4 | **Soil kinds** (4 bits, a table of multipliers) | Per kind: trail kept, step, upkeep, births | Sand: tubes fade fast, so the net there is thin and flickers. Clay: tubes stay, old roads remain. Warm: fast, hungry growth. Cold: slow and frugal | Sand makes a mould that cannot hold ground (a fragile front to cut). Clay makes ground it never gives up. Warm and cold set each region's tempo | One table lookup a walker step (step, upkeep) and one a trail cell (kept) |
| 5 | **Soft rock** (map-placed walls) | Walls with health, already built (`wall_hp`, bites, rubble) | Thickets and crust the mould chews through slowly | Natural gates and cages the map designer places. Zero new rules | None |
| 6 | **High ground for shots** (from height) | A shot's line stops at a cell higher than the line between shooter and target. A shooter gains range per unit of height | The squad on a ridge sees and hits far; a ridge between two valleys blocks fire | Gives the player a reason to want the heights that the mould avoids. Symmetric with rule 1 | A height read per cell on the shot's walk, which already walks cell by cell |
| 7 | **Dry heights** (from height) | Upkeep × (1 + `dry` × height) | A mould spread over a plateau starves thin; it lives in the valleys | Pairs with the Sleeper tier (dry mould sleeps, an old rule from Keep It Alive's water) | One multiply |
| 8 | **Mould-made ground** (bits 16-23) | Where the mould sits long, a slow "crust" level rises; crust raises trail kept and lets a killed mould regrow from it | Old territory stays marked after the mould is burned off | The map remembers the fight. Burning the mould is not the same as clearing it | A slow field, updated every N ticks |
| 9 | **Barren ground** (a soil kind) | No births, upkeep × 2 | Bare patches the mould crosses but never fills | A gap the mould must bridge with tubes, which soldiers can cut | Part of 4 |

**Held back.** *Light and shade* (a repellent per cell) overlaps dry heights and barren ground, so it waits until those are seen. *Wind* (a drift vector per cell) is a general form of rule 2; start with drift taken from height, which needs no new data. *Ground that carries energy* (veins) waits on `energy_flow`, which is CPU-only and off.

**Order.** Height with rules 1-3 first, because one byte drives three effects, and the graphics thread can shade height once the simulation uses it (their agreement). Then rule 6, so the squad uses height too. Then soil kinds (4, 9), then dry heights (7), then mould-made ground (8). Soft rock (5) costs nothing and is on the maps from the start.

**Proposed ground word** (today's bits 0-11: pile 0-7, wet 8, wall 9, scent wall 10, rock 11):
- bits 12-15: soil kind (loam 0, sand 1, clay 2, warm 3, cold 4, barren 5);
- bits 16-23: kept for mould-made ground;
- bits 24-31: height (agreed with the graphics thread).

**First tests, each measure named before the run** (CPU big dish first, a look first, then paired seeds):
- *Slope step*: a cone with a crystal on top against the same crystal on flat ground. Measure: seconds to the first bite, 8 paired seeds.
- *Downhill drift*: two equal crystals, one in a hollow and one on a rise, the mould between them. Measure: which crystal is eaten first, 16 seeds, sign test.
- *Sand and clay*: the same world with a sand band across the mould's road and a clay band. Measure: walkers alive past the band at 60 s, and the share of the road still drawn 10 s after a cut.
- Frame cost of each, quoted per tick, as every rule here runs every step.

## WM2: five paper maps

All five are 1024 × 640, the W2 world. Each is drawn for one of the player AI's styles (turtle, boom, rush, lure, adapt) to win, and names the mould tier that would turn it, because the owner's ruling is that difficulty comes from mould types. Picture legend: grey-green is ground shaded by height with contour lines, blue is water below the waterline, black is rock, brown is soft rock, cyan hexagons are crystals (size is charge), the yellow ring is the mould's start, and the white square is where the squad lands. Soil tints: yellow sand, red clay, orange warm, blue cold.

| Map | Ground it tests | Mould | Drawn for | Why that plan should win | What should turn it |
| --- | --- | --- | --- | --- | --- |
| [The Basin](larder-maps-2026-11-11/sketches/basin.png) | Height, drift, water table, shots from high ground | Drifter or Sprawler, at the lake's edge in the bowl, with three big crystals | **Turtle** (hold the rim) | Rim scent drifts down into the bowl and calls the mould up the slopes, where it is slow and in the squad's sight. A rock arc covers the north rim. Rushing down into the bowl means fighting in wet ground in the mould's own pooled scent | A Sporer, whose spores fly over the slope |
| [The Fork](larder-maps-2026-11-11/sketches/fork.png) | Height, drift, clay | Trunk, at the low end of a Y-shaped valley | **Lure** (bait one fork, harvest the other) | Both forks' crystals drift scent down to the mould, so it is pulled two ways. Bait tips it into one fork while the squad, landing on the high wedge between them, harvests the other. A rush meets the whole Trunk in the narrow stem | A Hunter, which follows soldiers rather than bait |
| [The Flats](larder-maps-2026-11-11/sketches/flats.png) | Sand, clay islands | Sprawler, on the largest clay island | **Boom** (out-harvest) | 32 small crystals on sand, where the mould's tubes fade fast; only the five clay islands hold its network. A spread squad harvests faster than a mould that cannot hold the sand. Turtling cedes the islands | A Trunk, which holds ground by thickness, not by soil |
| [The Nest](larder-maps-2026-11-11/sketches/nest.png) | Soft rock ring, warm soil | Small, in a soft-rock ring with two rich crystals on warm ground; one thin gap faces the squad | **Rush** (strike the nest early) | Warm ground makes the mould inside fast and hungry. Left alone it fattens on 1,280 charge and breaks out at full strength, so idle loses badly. Struck early through the gap, it is small | A Sleeper, which sleeps through a strike and wakes behind it |
| [The Passes](larder-maps-2026-11-11/sketches/passes.png) | Rock ranges, a soft-rock gate, cold and warm soil | Two rivals: one in a cold valley (slow, frugal), one in a warm valley (fast, hungry) | **Adapt** (set the two on each other) | The squad holds the middle band. The warm rival comes first and fast through the open southern pass. Bait drawn across the middle leads it to the cold rival's border, and the soft-rock gate in the north range sets when the cold rival arrives | A Converter, which grows by winning that border fight |

Sketch numbers (from `maps.json`): crystals 9-32 a map, total charge 2,496-4,000; rock 0-5% of cells; water 0-3.5%. These are first guesses, to be set by the W2b census, as W1c's sizing was ([Larder roadmap](larder-world-roadmap-2026-11-11.md), *Crystal sizing with growth*).

**The gate (WM2, from W2b's harness).** On each map, 16 paired seeds, the five styles plus idle and random, scored by W2b's score.
1. The map is kept only if the best plan beats idle and random (sign test, Holm over the maps).
2. Across the maps the winning plan must change: at least three different winners among the five, with each winner ahead of the second-best style on its map after Holm.
3. Each map's "what should turn it" is tested the same way once that tier runs in the world. The winner should change with the tier, which is the owner's difficulty ladder.

A map that fails is redrawn, not tuned by hand: the spec is a list of shapes, so a change is a line in `sketch_maps.py`.

## Order and what waits on what

- **Now, without WM1**: height rules 1-3 on the CPU big dish (a test world built in code, as `terrain_rock` is today), the look first, then the three tests above. Then the GPU port with a twin test, in the same shape as W1b's.
- **On WM1 and W2b**: load the five maps, run the gate.
- **WM5, the editor**: later. The spec is already a list of shapes (bumps, trenches, ridges, discs, rings with gaps, crystals, starts), so an editor places and drags shapes and rasterises them, rather than painting cells.

## Loader files, agreed with the Larder thread (19:40Z)

`sketch_maps.py` now also writes what WM1's loader reads, so the crate needs no JSON parser. `maps.json` stays the source. For each map:
- [`maps/<name>.png`](larder-maps-2026-11-11/maps/) is RGBA at 1024 × 640. R is height (ground byte 3). G is ground bits 8-15: wet 0x01, rock 0x08, and the soil kind in the high nibble. B is 0 (kept for mould-made ground), and A is 255. Crystal and wall cells are not drawn in, because the engine makes them.
- `maps/<name>.txt` has one item a line, in 1024 × 640 cells: `crystal x y r charge`, `mould x y`, `landing x y`, and `wall x0 y0 x1 y1 thick hp`. Soft rock is written as wall lines (thick is the full width, hp 400, the player wall's default), so the engine's own `build_wall` draws it with health and leaky scent. The Nest's ring is 10-degree chords with the gap left out.

The ground-word layout above is agreed. The loader resamples these files to the world's size, so 512 × 320 takes every other cell.

## Which ground the mould feels, 2026-11-11

Before any ground goes on a map, it has to change the mould. Harness: `examples/worldground.rs`. Ground kinds: `BigParams::sand_keep` (trail on sand x 0.8 each fine step), `clay_keep` (x 1.05), `barren_upkeep` (upkeep x 3, no births), all acting through `BigDish::soil` (one kind a fine cell; empty, the default, changes nothing, so every GPU twin is untouched). Wet ground (the step x 0.5, built in W1a) is the baseline: it is the kind of rule a slope would be.

**World.** 512 x 320, 23 crystals of charge 512 eaten at 0.005, 5,000 walkers growing into 50,000 slots, upkeep 0.001, 60 s. A patch of keyed blobs covers 30% of the ground, clear of the crystals. The cells are the same in every arm of a seed, and only what the patch is changes: plain (control), wet, sand, clay or barren. 16 seeds. A first look at the default `crystal_eat` 0.05 ate 98-99% of the charge by 60 s and filled all 50,000 slots by 30 s in every arm, so size could not differ; it is not counted (the seed 1 picture at 60 s on the fed world is in `ground/look/`: the mould shuns sand and spreads onto clay even there).

**Measures, named first**, each paired against plain with a two-sided sign test: the share of living walkers on the patch at 30 and 60 s; walkers alive at 60 s; crystal charge eaten by 60 s. 16 tests, Holm. Raw: [`ground/`](larder-maps-2026-11-11/ground/) (`run-q1..4.txt`, seeds in four parallel runs; a container restart cut the first runs, and the lost seeds were run again; seed 4 rerun alone gives the same hashes).

| Ground | Share on the patch at 30 s (plain 0.108) | Alive at 60 s (plain 7,165) | Charge eaten (plain 0.025) |
| --- | --- | --- | --- |
| Wet (step x 0.5) | 0.154, up 15/16 | **4,044, down 16/16** | 0.015, down 16/16 |
| Sand (trail fades) | **0.070, down 15/16** | **11,462, up 14/16** | 0.037, up 15/16 |
| Clay (trail lasts) | 0.138, up 13/15 | 6,199, down 13/16 (p 0.021, not after Holm) | 0.018, down 15/16 |
| Barren (upkeep x 3, no births) | 0.088, down 13/16 (p 0.021, not after Holm) | **3,603, down 15/16** | 0.014, down 15/16 |

Medians of 16 seeds. All the other results survive Holm (p 0.0005-0.007).

- **Every kind changes the mould, and in a way you can see** ([seed 4 at 60 s](larder-maps-2026-11-11/ground/look/): plain, wet, sand, clay, barren).
- **Slow ground halves the mould.** Wet patches crowd walkers onto them (they stay longer), and the mould reaches fewer crystals: 16 of 16 seeds smaller. This corrects the guess in this thread's reply to the owner (20:08Z) that a speed rule would do little. A slope that slows would likely do the same. Flat wet ground already gives that effect without a height field, so height stays parked.
- **Sand makes the mould bigger, not weaker.** Walkers leave sand quickly (share 0.108 to 0.070), but the whole mould grows by about 60% and eats about 50% more. Read: trail that fades stops walkers circling their own old roads, so they explore and find crystals sooner. This was not the brainstorm's prediction ("a fragile front to cut"). For maps, sand is fast-spreading ground, a racecourse for the mould.
- **Clay traps the mould.** Walkers gather on it, and the mould eats less (15 of 16). Old roads hold walkers in place.
- **Barren ground starves it.** It is half the size on 15 of 16 seeds. Walkers do not clearly avoid it; they die there.
- Frame cost: one multiply a cell a fine step for sand and clay, one lookup a walker for barren. The ms a tick tracked walker count, not the soil (sand 7.1, plain 6.4, barren 5.7 ms; 1 thread each, 4 runs at once), so the pass itself is below what this harness resolves.

**For maps:** wet ground and barren ground slow and starve the mould (a moat, a buffer zone); clay holds it where it is (a nest it is slow to leave); sand is where it spreads fastest. Each makes a different place on the map. Defaults stay as tested. All are off unless a map sets soil.

## Maps, version 2 (flat), 2026-11-11

The five maps are redrawn flat (R, height, is 0), and the sketches now use colours close to the graphics thread's world look (soil, grey rock plateaus with a shadow, glossy water), with sand pale, clay red-brown and soft rock as thicket green. Crystal radii are 3x version 1's (the Larder world's `pile_r` of 6 dish cells is 24 in 1024 x 640 cells; version 1's 7-15 were under it, and at 2x the maps held 25-43% of the race world's 8,671 crystal cells at scale 2, as the Larder thread's loader found, 21:32Z). At 3x they hold about 4,900-8,400 cells, 56-97% of the race world's. Spec, loader files and sketches: [`sketch_maps.py`](larder-maps-2026-11-11/sketch_maps.py), [`maps/`](larder-maps-2026-11-11/maps/), [`sketches/`](larder-maps-2026-11-11/sketches/).

| Map | Ground | Drawn for |
| --- | --- | --- |
| The Rim (was The Basin) | A rock ring with four gaps round a clay floor (the mould holds it), a sand belt outside the gaps, a pool inside | Turtle: hold the gaps |
| The Fork | Rock walls make a Y; clay at the mould's end; a pool in one fork | Lure |
| The Flats | Sand everywhere, five clay islands | Boom |
| The Nest | A soft-rock ring with one gap round clay and two rich crystals; rock outcrops and pools outside | Rush |
| The Passes | Rock ranges with an open pass and a soft-rock gate; a clay valley and a sand valley, one rival in each | Adapt |

The test changes two of version 1's guesses. Sand helps the mould spread, so on The Flats it is the mould's ground as well as the squad's, and The Flats now asks whether boom can still out-harvest a mould that spreads fast but holds only the clay. In The Passes, the sand rival is the fast one, and the clay rival the slow, entrenched one. Next: draw these with the graphics thread's look (agreed 20:29Z: sand, clay and barren become materials in `view.wgsl`'s `cell_mat`, keyed on ground bits 12-15) once its PR merges, and run the WM2 gate once the Larder thread's loader and W2b's harness are in.

## Soil in the world look, 2026-11-11

The graphics thread's look (PR #69, merged) now draws soil: `world::look::Fields::soil` puts the kind in the high nibble of the ground code's byte 0 (agreed with that thread, 21:49Z; the kind keeps the low nibble), and `view.wgsl`'s `cell_mat` draws sand pale, clay red-brown and barren ash-grey under the mould. The view WGSL validates with naga 29.0.4; unit test `soil_sits_in_the_high_nibble_of_the_kind_byte`. Pictures wait until the GPU engine itself acts on soil (the picture is the data: soil drawn on a GPU world that ignores it would be paint), which is the GPU port of `sand_keep`, `clay_keep` and `barren_upkeep`.

## Sources

- [larder-world-roadmap-2026-11-11.md](larder-world-roadmap-2026-11-11.md): W1a/W1b fields, wall health, crystal sizing, the W2 plan.
- [wiki/world/mould-tiers.md](../wiki/world/mould-tiers.md): the tiers named as each map's counter.
- [wiki/world/player-ai.md](../wiki/world/player-ai.md): the five styles.
- [wiki/model/water-and-sleep.md](../wiki/model/water-and-sleep.md): dry mould sleeps (rule 7's source).
