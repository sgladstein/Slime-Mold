# Rivers off the nests (2026-11-15)

The map-fix report ([report](../levelgen-mapfix-2026-11-15/README.md), *Not
fixed here*) left one of the three planet reviews' map items open: "River
valley seed 2's near nest now sits by the bridge, so the slime is at the
bridge from the start." Rivers run across the route at fixed shares of the
way to the first nest, and the second nest often stands partway along it.
Water is cleared within 100 map cells of a nest, so a river through a nest's
bowl ran dry there: the nest itself was the crossing.

**Outcome:** a river now moves along the route, the least it must, to keep
its water at least 220 map cells (`RIVER_OFF`, outside a nest's bowl) from
every nest, and its fords and bridge at least 320 (`CROSSING_OFF`). Where
there is no room for a second river, it is left out. Over 20 river valleys,
river water within 220 of a nest went from 19 of 40 nests to none, and fords
and bridges within 320 of a nest from 31 of 57 to none.

- Before: main at 2f6ac3f4 (PR #345). After: this change.
- Page: [Rivers Off the Nests](https://claude.ai/artifact/X37QSmDaK192GKjKai3UPC), copy in `page/`.
- Surveys: river valleys (planet 4) and core worlds (planet 8), seeds 1-20,
  `cargo run --release --example planets -- seeds=1-20 depths=D out=DIR`
  ([before](raw/survey-before-1-8.txt), all eight planets on the same code;
  [after](raw/survey-after-4-8.txt)). Only planets that roll rivers change:
  every valley and 5 of 20 core worlds.
- River water by each nest: [`raw/nestwater.py`](raw/nestwater.py) on those
  maps ([before](raw/nestwater-before.txt), [after](raw/nestwater-after.txt)).
- Fords and bridges by each nest, where the river is drawn: a debug line per
  river in `build_with` (not committed), counted by
  [`raw/crossdbg.py`](raw/crossdbg.py) ([before](raw/dbg-before.txt) and
  [after](raw/dbg-after.txt); [counts before](raw/crossings-before.txt),
  [after](raw/crossings-after.txt)).
- In-game frames: `larder-world` on llvmpipe, `hand=idle`: the whole valley at
  zoom 2 at 1 s and films of the slime's first minute from the second nest at
  zoom 3 ([`raw/renders.sh`](raw/renders.sh)); pictures and the false-colour
  layouts by [`raw/make_pics.py`](raw/make_pics.py). Render times in
  [`raw/render-timing.txt`](raw/render-timing.txt). The core world's film
  went unused: at zoom 3 round its second nest the river's move barely shows;
  its layout is on the page.

## What changed (`src/mould/levelgen.rs`, rivers only)

- **Rivers keep off the nests.** For each river, the places along the route
  from a quarter of the way to the first nest (and at least 230 cells, past
  the landing's dry ground) to nine tenths are tried, every 1/80 of the way,
  130 cells from the other river. The river keeps its old place if that
  clears every nest by `RIVER_OFF` 220, else takes the clearing place nearest
  it. How near a place runs to a nest is read through the same warp the river
  is drawn with, from points every 6 cells round each nest.
- **A second river with no room is left out.** With the second nest partway
  along the route there is often no place for two: 3 of the 7 valleys that
  rolled two rivers keep one. A first river with no room takes the clearest
  place (none needed it here).
- **Fords and the bridge keep `CROSSING_OFF` 320 from the nests** where the
  river has room (it did on all 25 river planets).

New test `rivers_keep_off_the_nests` (it fails on main: valley seed 1's
second nest has river water 100 cells off, the edge of the water cleared
round it). The river valley is re-pinned (`PINNED[3]`, dial pin 4:2); no
other pin moved.

## Measured

Distances in map cells (two to a dish cell), straight from a nest's centre.

| | Before | After |
|---|---|---|
| Valley nests with river water within 220 (40 nests) | 19 | 0 |
| Valley nests' nearest river water, median (min) | 302 (118) | 450 (221) |
| Core-world nests with river water within 220 (15 nests, 5 planets) | 7 | 0 |
| Core-world nests' nearest river water, median (min) | 223 (114) | 400 (231) |
| Valley fords and bridges within 220 / 320 of a nest | 16 / 31 of 57 | 0 / 0 of 51 |
| Valley fords and bridges, nearest / median | 45 / 286 | 320 / 437 |
| Core-world fords and bridges within 220 / 320 of a nest | 6 / 8 of 26 | 0 / 0 of 26 |
| Valleys with two rivers (7 rolled two) | 7 | 4 |
| First-try builds, valleys / core worlds | 18 / 17 | 17 / 17 |
| Build, valley, mean per planet with retries (median of three runs) | 265 ms | 277 ms |

Shallow crossings the map makes later can still come near a nest: the back
road to a nest's breach (`route`, the front-door work) is made shallow where
it crosses deep water. By the map measure (a wet cell more than 20 cells from
deep water), 18 of 40 valley nests have one within 300, outside the bowl; the
fords and bridges the river is given are all 320 or more away.

**In game** (the page): before, on valley seed 1 the river stopped at the
second nest's bowl, and the slime spread across the dry gap in its first
minute. After, the river runs the map's height between the landing and the
nests, crossed by its fords and bridge away from both bowls.

## Tried first

- **A second river at the clearest place when nothing cleared both nests:**
  on 4 of 20 valleys the clearest place ran between the two nests, through
  both bowls (river water 132-160 cells from both). See
  `Reports/dead-ends.md`.

## Not done here

- Cave gates off pools (the map-fix report's other open item).
- Whether a valley with one river plays as well as one with two is for the
  owner's play.
