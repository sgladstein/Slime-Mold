# Adversarial review brief (three reviewers)

You are one of three independent reviewers of the procedural planet work in
the Slime-Mold repo: one on aesthetics, one on fun, one on technical checks.
The snapshot to review is the git worktree /home/user/sm-review (commit
the 6x build, once merged); its history holds the planet PRs #277, #284, #289, #302, #306, #307
and the 6x size change (`git log --oneline -15`, `git show`). The owner
asked for an adversarial review: your job is to find what is wrong, missing
or weak, and to say how to make it better, not to confirm it.

## What the work is for

The game is Larder World: soldiers against a slime mould on a GPU dish. A
roguelike run takes a squad through 8 generated planets toward the slime's
core world. `levelgen::level(seed, depth) -> Map` makes each planet: its
ground (rock, wet, soil kinds, deep water, bridges), plants, crystals (the
energy both sides want), slime nests, the landing, moisture for fire, and
the map's size. The owner's rulings for planets (quoted in
wiki/decisions-game.md and Reports/levelgen-2026-11-14/README.md):

- Fun over fidelity: interesting simulations that make a game; real
  landscapes, ecology and geology are inspiration, not ignored and not the
  bar.
- Way more variability between biomes; biomes are dials rolled per planet.
- No biome forces a strategy: every map is beatable by any build (guns,
  turrets and walls, kill boxes, sneaking, sniping, artillery, jumping).
- Open early, mazes late. Sizes from 0.6x today's map on planet 1 to 6x
  area at the core.
- Deep water stops soldiers; slime crawls through it slowly.
- Everything on screen is the simulation. The owner plays zoomed in, so
  every edge must be crisp at close zoom.

Planets now grow to the owner's sizes: 0.6x, 0.8x, 1x, 1.4x, 2x, 2.8x, 4x
and 6x today's map area on planets 1-8 (`AREA`, `GROW_CAP`). The core world
(planet 8) has a few open arenas and a hedge round every nest. The work
also makes big planets run smoothly: a sparse ground refresh
(`gpu::Gather`, src/bin/larder-world/draw.rs `refresh_ground`, `live_cells`,
`live_code`; walls re-mark only their tiles, src/world/look.rs
`mark_far_box`) and soldiers' walking maps built a slice a step
(src/mould/larder_world/paths.rs `MapBuild`, `Game::map_for`,
`Game::pump_maps`; squads.rs `guide`). And it sows plants per biome with
fading edges (levelgen.rs `Sow`, `sow`).

Read first: CLAUDE.md, wiki/looks/art-direction.md (the style guide; every
reviewer should know the look rules), Reports/levelgen-2026-11-14/README.md
(the report),
Reports/levelgen-2026-11-14/biomes-plan.md, wiki/world/planets.md, then the
code above.

## Lead with the big picture

1. Is the approach right for the goal (a run of planets that play well and
   differ, and run smoothly at 6x)? What would you do differently?
2. What did the tests and measurements miss: controls, confounds, planets
   never looked at, claims with no evidence behind them?
3. Inspiration from outside: published procedural map generation (strategy
   games' map fairness and chokepoint scoring, roguelike level design,
   terrain synthesis, pathfinding at scale such as flow fields and
   hierarchical maps) and real landscapes, where they suggest something
   better.
4. Ranked next directions, each with a first experiment that would settle it.

Then correctness: concrete failures with file:line, the input that breaks
it, and what you ran to see it.

## Every feature of the world

Whatever your angle, examine each feature a planet can have, one by one,
and say how it does from your angle, with the planets and seeds you looked
at:

- Ground: open ground, outcrops, pillars, ridges and their passes, rocky
  rooms and gates, caves and tunnels, the core world's arenas.
- Water: pools, lakes, rivers with deep middles, fords and bridges,
  streams, the way `cut_fords` makes a crossing shallow.
- Soil: plain, sand, dunes, barren bands, oases, clay, cracked clay, mixed.
- Soft rock: hedges, rings round nests, grown soft rock.
- Plants: each biome's cover, reed beds at water, patch edges, moisture for
  fire.
- Crystals: scattered, fields, veins, one jackpot, at the oases, richer
  toward the slime; big and small crystals.
- Nests, the landing, the map's size and shape, and the fallback map.

## Your angle

Each reviewer gets one of the three angles below, and only that one.

### AESTHETICS

You review only how the planets look. Leave fun and code to the other two.
Looks are the owner's first concern, so be exacting: judge it as a shipped
game's art, not as a prototype.

**The bar.** The game's style guide is `wiki/looks/art-direction.md` ("living
light on dark ground"); read it before rendering anything. Judge against its
rules, and say which rule each finding breaks:
- the picture is the data: every pixel from a field or game state, no
  painted decoration, no thresholded edges on graded quantities;
- value order, darkest to brightest: ground and soils, rock and walls, thin
  slime, crystals, thick slime, units, light. Check it in grey;
- one hue a role: the slime gold to amber, crystals the only cyan, plants
  green, water dark blue, the team cream and olive, red for danger only;
- sharp at every zoom from 1 to 24: no cell stairs, seams or square grain.
A fix you propose must keep these, so a mood or palette change per planet
must come from something the simulation has (ground, water, plants, light
from the slime and crystals), not a tint painted over the screen.

**What is already known** (don't spend time re-finding it; say if it is
worse than described): the core world was too busy for its route to read;
plants covered bare biomes edge to edge; reviews and their pictures are in
`Reports/levelgen-review-2026-11-14/` and `Reports/levelgen-2026-11-14/`
(pages 1-3), `Reports/soil-materials-2026-11-14/`,
`Reports/river-look-2026-11-14/`. Check whether each was fixed.

**Look first, in motion, at the owner's zoom.** The owner plays zoomed in
and judges looks from moving pictures, never stills alone. For every planet
1-8, seeds 1 and 2:
- whole map at 1 s and 60 s (the slime grown);
- close views at zoom 6, 12 and 24 (`centre=X,Y,ZOOM`, world units; pick a
  shore, a rock edge, a plant patch's edge, a nest and the landing);
- a short GIF (3-5 s, 10 fps) at zoom 12 on at least four planets: the slime
  front crossing plants, soil and water, and one fire if a planet has one.

**Tests to run and quote** (numbers, not impressions):
1. *Grey thumbnail test:* every planet's 1 s frame shrunk to 266x164 in
   grey. Shown one, can you name the biome? Show a contact sheet.
2. *Close-zoom identity:* a 1280x800 frame at zoom 12 with no nest or
   crystal in it. Can you still tell the biome? The owner spends most of
   the game here, so this matters more than the overview.
3. *Slime stands out on every ground:* grey luminance of thick slime, thin
   slime film and the ground it covers, per biome. Thin slime must stay
   above every soil, rock, wall and plant; flag any biome where it doesn't.
4. *Crystals pop:* crystal against its surroundings in colour and grey on
   every biome; nothing else on the map near cyan.
5. *Seed variety:* two seeds of one biome side by side. Same place, a
   different map?

**Progression.** Played 1 to 8, does the look build toward the slime's home
world (rising strangeness, threat, density, a shift in palette or light)?
Or is it eight unrelated maps? Where does it sag or jump? Does the core
world read as a climax in the first second you see it? At 6x, what does the
whole-map view and the minimap show?

**Craft.** Edges at zoom 24 (rock, water, shore, plant patches, hedges,
bridges); seams, stair steps, tiling or repetition, noise that reads as
noise, flat empty stretches with nothing to look at, things that read as UI
(rings, bars, discs), anything gold that isn't slime or cyan that isn't a
crystal.

**Inspiration.** Top-down games that give each biome or floor its own look:
RimWorld's biomes, They Are Billions' map themes, The Riftbreaker's biomes,
Against the Storm's biomes, Mindustry's generated sectors, Don't Starve's
biomes, and the floor changes in The Binding of Isaac, Enter the Gungeon and
Nuclear Throne. Also Noita, which the owner cites: its look comes from the
world changing (burning, flowing, collapsing), not from decoration. Say
what they do that this doesn't, and how it could be done from this
simulation's fields.

**Output.** Picture paths for every claim (contact sheets in colour and
grey, close crops, GIFs). The three biome pairs that look most alike, at
zoom 1 and at zoom 12. The style-guide rule each finding breaks. Ranked
changes, each with the smallest visible fix and who would make it: the map
(levelgen) or the drawing (view.wgsl, the ground thread).

### FUN

You review only whether the planets make good play, fully focused on that.
Leave the look and the code to the other two. You can't feel fun the way a
player does, so judge it as a game designer would, from what you can see
and measure: routes, chokes, distances, build spots, how the AI hands'
games go, and how each map compares with designs known to work. Say which
of your calls rest on a measure and which are judgement, and end with the
questions only the owner's own play can answer.

- **Variety.** Does each biome ask for different play: different routes,
  chokes, places to build, risks, timing? Or is it the same fight on a
  different floor? Do two seeds of one biome play differently? Which biomes
  play alike?
- **Progression.** Over a run of 1 to 8, does it feel like a satisfying
  climb? Look at size (0.6x to 6x), nests (one, then two, then three),
  mazes late, and the pace from 15-20 minute levels toward an hour. Is each
  planet a step up, with breathers and a climax? Where would a player get
  bored, stuck or swamped? Is 6x at the core fun or a slog?
- **Every strategy can win.** For each planet, is there an open approach
  and a choke for each nest? Are there maps that only one build beats
  (turtle, rush, sniping, sneaking)? Name the planets and seeds.
- **Play it, or the nearest thing.** Run `larder-world` with `hand=rush`,
  `hand=every` and `hand=idle` on a few planets. Watch frames of what
  happens, and read the end lines (kills, crystals, who holds what). Look at
  the map files with the planets' numbers (walk to the first crystal, open
  charge, attempts).
- **Inspiration.** Top-down games with generated maps and a climb from one
  map to the next: They Are Billions' survival maps (a horde against your
  walls), Creeper World's generated maps (a spreading enemy you hold back),
  Mindustry's sectors, The Riftbreaker's survival maps, and Against the
  Storm's roguelite run of settlements; and Noita (a side view, but its
  generated world and its descent are a strong progression). For fairness, chokes and
  expansions, look at Age of Empires II's random map scripts, Northgard's
  generated maps and Tooth and Tail's procedural maps.
- **Output.** Name the three biome pairs that play most alike. Give a curve
  of how the run's difficulty and novelty would feel, planet by planet, and
  ranked changes, each with a first experiment that would settle it.

### TECHNICAL

You review whether the planet work is correct, fast, deterministic and well
tested, fully focused on that. Leave the look and fun to the other two.

- **Determinism.** Same seed and depth give the same planet on every
  machine: no platform maths, and pinned fingerprints. Find anything that
  could differ on Windows or another GPU: float functions, hash-map order,
  thread timing.
- **Checks.** Are the fairness checks (reachable crystals and nests, a
  crystal near the landing, open charge) right? Find seeds that pass the
  checks but play badly, or that need many attempts or the fallback. Run
  the planets example over many seeds and depths.
- **Speed at 6x.** Check the claims in the report against runs: walking
  maps a slice a step (`MapBuild`, `map_for`, `pump_maps`), stale maps,
  the cache cap, the sparse ground refresh (`Gather`), and the gas
  read-back (`read_back_range`). Look for a hitch the benches miss:
  first-second costs, many squads, walls built in quick succession, maps
  larger than any timed. Check memory too.
- **Correctness.** Do sliced maps give the same distances as whole ones? Can
  a soldier wait forever for a map? What happens when goals change every
  step? Are the edge cases right (map edges, 4096-cell maps, deep water
  with no ford)?
- **Tests.** Which claims have no test? Which tests would pass even if the
  feature broke?
- **Output.** Concrete failures with file:line, the input that breaks it
  and what you ran, marked CONFIRMED or SUSPECTED, ranked, each with the
  smallest fix.


## Rules

- Do not edit files in /home/user/Slime-Mold or /home/user/sm-review, do not
  commit, push or comment on GitHub. Work in your own folder: its own folder.
- Prebuilt binaries of the snapshot are in /home/user/sm-review-bin
  (larder-world, planets). Do not run cargo in /home/user/Slime-Mold or
  /home/user/sm-review. The machine has 4 cores shared by three reviewers
  and the author: if you must build changed code, copy the repo to your
  folder and build there with its own target folder (a first build takes
  10+ minutes), and say how long it took. Prefer the binaries and reading.
- Useful commands (from its own folder):
  `/home/user/sm-review-bin/larder-world look=DIR map=planet:DEPTH:SEED seed=1 hand=rush at=5,20 screen=1280x800`
  renders frames (PNG) of a planet in play (`centre=X,Y,ZOOM` in world
  units for close views; software Vulkan, about 1 s a frame;  takes
  fractions, so  gives GIF frames, which PIL joins);
  `/home/user/sm-review-bin/larder-world bench=10 seed=1 map=planet:D:S:AREA hand=rush spread=0`
  times frames (the "frames with a step" line's step max is the game step);
  `/home/user/sm-review-bin/planets seeds=1-3 depths=1-8 out=DIR` writes
  planets as map files (PNG: G byte ground, B byte plants; TXT items) and
  prints their numbers. Test names are in the source's tests modules. The
  page images are in /home/user/sm-review/Reports/levelgen-2026-11-14/page3/.
- Look at pictures before measuring. Quote numbers with what you ran.
- Keep it under 1,500 words. End with your findings ranked most serious
  first, each marked CONFIRMED (you reproduced it) or SUSPECTED, with the
  smallest fix you would make.
