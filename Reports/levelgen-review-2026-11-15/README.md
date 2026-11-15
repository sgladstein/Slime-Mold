# Three reviews of the procedural planets (2026-11-15)

The owner asked for a three-agent review of the planet work (2026-11-14
19:12Z), then shaped it (23:59Z): "one agent fully focused on aesthetics,
another in fun, another on technical checks. Aesthetic and fun should both
check for enough variety in the worlds/biomes and that there feels like a
satisfying progression as you go through the game". After the owner's edits
to the brief (00:06-00:09Z: top-down games with generated maps to compare
with, plus Noita; every feature of the world examined; "Can an agent really
evaluate fun?"; "Ask the graphics agent to review your prompts. Looks is my
number one goal/concern") and the graphics lead's rewrite of the looks
section, the owner said go at 00:12Z.

- Reviewed: main at 4494ad4f (planets 1-8 at full size, the core world at 6x).
- Briefs: [aesthetics](raw/aesthetics/brief.md), [fun](raw/fun/brief.md), [technical](raw/technical/brief.md).
- Full reports: [aesthetics](reports/aesthetics.md), [fun](reports/fun.md), [technical](reports/technical.md).
- Page: [Planet Reviews](https://claude.ai/artifact/Kcy1AoquWJjJ3EciyA3KxP), copy in `page/` (pictures chosen and converted by `raw/make_pics.py`).

## How each review was run

- **Looks.** Prebuilt `larder-world` on llvmpipe, `hand=idle`, 1280x800 unless stated. Planets 1-8, seeds 1 and 2: whole map at 1 s and 60 s, and 116 close views at zoom 6, 12 and 24. Five 4 s clips at zoom 12. Grey luma registered to the map files (`raw/aesthetics/measure.py`, table `raw/aesthetics/measures-table.txt`). To rerun the luma table: make the frames with [`raw/aesthetics/main_runs.sh`](raw/aesthetics/main_runs.sh) (`larder-world look=frames/main map=planet:D:S seed=S hand=idle at=1,60 screen=1280x800`) and the map files with `cargo run --release --example planets -- seeds=1-2 depths=1-8 out=maps`, set `R` in [`measure.py`](raw/aesthetics/measure.py) to the folder holding `frames/main/` and `maps/`, and run it from `raw/aesthetics/` (it imports `worldrect.py` there).
- **Play.** 64 planets (seeds 1-8, depths 1-8) from the map files, measured on 4-cell bins (`raw/fun/*.py`): walks, the widest route to each nest's gun range, the least wall between home and the nests, the direct way (the widest route at most 15% longer than the shortest), sight lines, crystal energy along the route. The six designed maps as a control. Six short games by the AI hands (rush on planet 1 for 20 s; take_claim for 150 s at 1x area on five planets). Each call marked measured or judgement; ends with questions for the owner's play.
- **Technical.** A sweep of 320 planets (seeds 1-40, depths 1-8) and of seeds 51-250; measurement-only tests in a copy of the tree (`raw/technical/review_tests.rs`): steering round rock at 1x and 6x, the walking-map cache under 8-32 live goals, walls going up every 2-16 steps, stale maps behind asks, the game's own walking grid on the planets the checks flagged, forced fallbacks.

## Ranked findings

Looks first where two are close (owner, 00:09Z). Who would make each change: map (the level generation thread), drawing (graphics and ground threads), run (the roguelike run thread).

1. **Zoomed out, the big planets switch to an unshaded picture** (looks, drawing). Below zoom 1 the view skips its shading (`src/world/view.rs:731-737`). The 6x core world fits a 1280x800 window at zoom 0.86, so its whole-map view is navy ground, brown rock and flat crystals brighter than the slime, on any screen under about 900 px tall. The minimap uses that picture on every planet.
2. **Rock and soft rock outshine the thin slime** (looks, drawing). On all 14 planet-seeds measured, rock (luma 98) and soft rock (107-123) are brighter than the thin film (10th percentile 49-86), and soft rock is brighter than the crystals on 12 of the 13 with soft rock. The film stays above every bare soil (only 1-10% of newly covered pixels within 10 levels of the ground).
3. **The biomes differ only in layout** (looks and play; drawing and map). Ground median luma 33-51 on planets 1-6. Grey thumbnails: 5 of 8 named by eye. Zoom 12 with no nest or crystal: 4 of 8 (Meadow, Fen, River valley and Core interchangeable). In play barrens and fen are 1.01 apart (1.0 = two seeds of one biome), fen and valley 1.11, meadow and barrens 1.13; 6 of 8 fens rolled only pools, which soldiers walk through.
4. **Almost no nest has both a narrow front door and a wide way round: 4 of 112** (play, map). Open planets (1-3, most jungles and cores) have no cheap choke: a line takes 40-110 cells of wall, 800-2,200 food against a 2,000 start. Funnel planets (4, 6, 7) have no route 16 wide to any nest on 8 of 8 seeds. Per biome in the page's table, from `raw/fun/localchoke-summary.txt`.
5. **The core world doesn't read as the climax** (looks and play; map and run). A meadow at zoom 12; rock fragments (seed 1) or a web of streams (seed 2; the stream count grows with the square root of the area); bullseye hedge rings. The slime covers 3% of the map at 60 s against 24-26% on planet 1 (default slime, not the run's). Nests kept 280 cells apart at every size: at 6x the nearest pair is 0.14-0.19 of the map's width on 5 of 8 seeds, and seed 1's three nests share 7% of the map. Seed 5 has four parallel rivers and a widest route of 4.
6. **The run's pace** (play; run and map). The slime grows about 1.6x a planet with no breather; the clock is 900 s on every planet; the first crystal is 12-26 cells from the landing on 64 of 64; crystal energy per unit of area falls from 4 to 1 between planets 1 and 8.
7. **Caves** (looks and play; drawing and map). Rock (96, 98, 102) is the backdrop's colour; the caverns read as a node graph. Every route 4-6 wide on 8 of 8 seeds; mean sight 15-17 under the gun's 24.
8. **Clay cracks and plant crowns drawn through the slime** (looks, drawing). `view.wgsl:1592-1593` fades them by the film's computed thickness, not its drawn cover.
9. **Fords, bridges and the moat** (looks and play; map and drawing). Fords are 6-7 luma off deep water; the bridge deck takes the river's ragged outline (`levelgen.rs:1441-1446`); the slime crosses deep water at 0.2 speed and on valley seed 1 reached the squad's bank by 150 s.
10. **Hedges read as interface; nests on the map's edge** (looks; drawing and map). Beige tubes brighter than crystals, laid across lakes, a 25 px blurred joint with rock. 6 of 16 first nests 55 world cells from an edge. Every start a perfect gold disc.
11. **Soft edges and a dim fire** (looks, drawing). Rock's shaded side a 20-25 px soft band at zoom 24; fire's brightest pixels (179) under the slime's (181), its hue beside the slime's amber; the front still from 55 to 59 s at reeds and water.

What works: the jungle canopy (the best close look in the game), smooth plant patch edges at zoom 24, plant cover on the bare biomes (Barrens 10.6%, Caves 4%; fixed since the first art review), crystals the only cyan, lakes and reed fringes, sand ripples and dunes, a clean landing pad, the thin film above every soil, caves distinct in play, two seeds of a biome giving different maps.

Questions only the owner's play can answer (fun review): is a moat that stops only you fun or unfair; do planets 2-3 feel like the tutorial again; is 15 minutes enough at 6x and a minute's walk to the front fine; are 4-wide cave tunnels tense or tedious; do the jungle's hedges read as a maze or clutter; is the same opening on every planet a comfort or sameness.

## The technical findings, fixed

All seven were fixed in this thread on top of #309, each with a test.

1. **A steered squad pressed against rock stalled** (`guide`, squads.rs). The steered goal moves a lead each time the anchor does, farther than the half-lead within which another map served it, so every move asked for a new map and the anchor waited for it. Round a 400-cell bar: 26.3 s at 1x against 12.9 s with whole maps, and never in 60 s at 6x (anchor still 798 of 900 steps). Now a map within `STEER_NEAR` (10 dish cells) serves: 13.6 s at 1x and 16.4 s at 6x (tests `a_steered_squad_gets_round_rock_with_sliced_maps` and `_at_6x`, bound 1.5x the whole-map time).
2. **The walking-map cache thrashed above 17 live goals at 6x** (`MAP_CELLS_MAX` 16M cells). With 24 goals asked every step, 26% of goal-steps went unserved and `pump_maps` averaged 9.2 ms while maps were rebuilt for ever. Now 32M cells (128 MB of distances, 34 maps at 6x): 24 goals all served and none lost over 600 steps (`walking_maps_serve_24_goals_at_6x`). Not taken, from the reviewer's suggestions: u16 distances (a long walk on a maze could pass 65,535 at 5 a side step), and a count cap of 48 maps (188 MB at 6x).
3. **Walls in quick succession starved the map being built** (`stale_maps`). Each wall threw the half-built map away and started it again; at 6x a goal waiting behind walls every 6 steps or less got no map in 600 steps. Now the build finishes on the ground it started on and joins the stale list (`Paths::build_stale`): a wall every 2 steps still serves the goal within 30 steps at 1x and 6x, and the finished map is marked stale (`walls_in_quick_succession_let_a_walking_map_finish`).
4. **The checks walked the dish's rim** (`walkable`). The dish has no agar within 3 dish cells (12 map cells) of its edge, but the checks treated the edge as open: planet 5 seed 39 passed with 100% of its charge open while the game's own grid reached 3 of 20 crystals without breaking soft rock (round a ridge's end through the rim). The bins within `RIM_MAP` (12 map cells) of the edge are now shut after the narrowing (`the_checks_walk_inside_the_dish_rim`). Seed 1's eight pins are unchanged.
5. **The fallback was neither open nor checked.** 6 of 96 forced fallbacks failed their own checks (planet 8 seed 11 with 46% of its charge open; a cave fallback 54% rock). `fallback` is now an open plan (no fronts, pillars, hedges, rings or gates; outcrops off), tried up to 4 times for one that passes: 32 of 32 (seeds 1-4, depths 1-8) pass (`every_fallback_passes_its_checks`). One try of the open plan left 2 of 32 failing: the far nest too far (planet 1 seed 2, 457 cells; planet 2 seed 1, 535).
6. **A wall worn to rubble read back the whole agar buffer** (`GpuWorld::keep`): 3.9 MB at 6x, about 3 ms on the 1070 by the reviewer's estimate, each step the broken count moves. Now a `Gather` of the mirror's wall cells, renewed when a wall goes up; only the walls pass opens agar on the GPU, and only at a wall cell (`a_wall_worn_to_rubble_opens_in_the_mirror`: two walls bitten to nothing on a race world, the mirror equal to the whole buffer read back).
7. **Test gaps.** The 6x pin repeated `PINNED[7]`. It is replaced by six pins over the dials seed 1 never rolls (`every_dial_is_pinned`: pillars 1:17, oases 2:7, lakes 3:3, ridges over a river 4:2, cave pools 7:2, a 6x core of pillars and streams 8:3). The walking-map scheduling now has the tests above. Open: the pins have only run on Linux; the PC runner can run them on Windows.

## Next

- Map items in this thread: a front door and a back road per nest (first experiment: 50 seeds a biome, count the nests that pass), core world nest spacing by map width, streams capped at the 1x count, ring arcs, nests off the edges, hedges off water, straight bridge decks and clean ford edges, fens weighted to lakes, cave tunnels of varying width.
- Drawing and run items routed to their threads through the coordinator (2026-11-15).
