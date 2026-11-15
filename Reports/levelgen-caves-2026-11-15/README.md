# Cave tunnels and rooms (2026-11-15)

The planet reviews ([report](../levelgen-review-2026-11-15/README.md)) found
two things wrong with the caves (planet 7, 4x today's area). The looks review
(item 5): "the layout reads as a node graph", round caverns joined by even
tubes. The play review (item 7, suspected): "the caves are a slog at 4x", and
its fix was tunnels 5-15 wide and a cavern arena or two.

**Outcome:** caverns are lobed, tunnels pinch and widen along their way (8 to
about 25 dish cells, from 12-18 all along), and each cave planet has big
caverns to fight in, one per today's map area (four on the caves' 4x map),
away from the landing and the nests. First-try builds stay at 19 of 20.

- Before: main at 17f2e5b8 (PR #329). After: this change.
- Page: [Cave Tunnels](https://claude.ai/artifact/JYjpXwy9GvRkmBKvxkyX5N), copy in `page/`.
- 20 caves (planet 7, seeds 1-20):
  `cargo run --release --example planets -- seeds=1-20 depths=7 out=DIR`
  ([before](raw/survey-before.txt), [after](raw/survey-after.txt)).
- Tunnel widths: [`raw/widths.py`](raw/widths.py) on those maps
  ([before](raw/widths-before.txt), [after](raw/widths-after.txt)); the
  widest room away from home: [`raw/room.py`](raw/room.py)
  ([both](raw/rooms.txt)).
- In-game frames: `larder-world` on llvmpipe, `hand=idle`, caves 2, 8 and 13
  ([`raw/renders.sh`](raw/renders.sh), camera spots by
  [`raw/spots.py`](raw/spots.py); films of the first nest's first minute by
  [`raw/renders-films.sh`](raw/renders-films.sh), as the landing's films in
  `renders.sh` showed nothing moving); pictures by
  [`raw/make_pics.py`](raw/make_pics.py). Render times in
  [`raw/render-timing.txt`](raw/render-timing.txt).

The play review's widths were in its own bins and read low: on the exact
measure here, before, no tunnel was under 12 dish cells at the 10th
percentile, not 4.

## What changed (`src/mould/levelgen.rs`, caves only)

- **Lobed caverns.** A cavern's radius wanders with the angle by three
  harmonics (2, 3 and 5 lobes, up to about a third), on top of the rough
  noise that wobbled the rim before. The landing's and the nests' caverns
  only bulge, so they keep their room.
- **Tunnels of varying width.** A tunnel's half-width follows a smooth noise
  (`girth`, 55-cell fbm) from 0.55 to 1.65 of its own, never under 8.5 map
  cells. A gate's soft rock spans the tunnel's width where it stands.
- **Rooms to fight in.** Caves take `Recipe::arenas` 1 (scaled by area, so
  four on the 4x map): sites away from the nests and the landing whose
  cavern is 0.85-0.95 of the regions' spacing across, nearly twice an
  ordinary one, with every tunnel to a neighbour open (the core world's
  arena rule).

New test `cave_tunnels_vary_and_open_into_rooms` (it fails on main: cave 1's
tunnels are 12.6-17.7 dish cells wide from the 10th to the 90th percentile).
The cave pins are re-pinned (`PINNED[6]`, dial pin 7:2); no other planet
changed.

## Measured (20 caves)

| | Before | After |
|---|---|---|
| Tunnel width, 10th / 50th / 90th percentile (dish cells) | 12 / 15 / 17 | 9 / 15 / 22 |
| Tunnel ridge under 8 / over 14 dish cells | 1% / 59% | 2% / 54% |
| Widest room away from home (clearance, map cells) | 76-109 | 120-235 |
| Cave floor 20+ dish cells from rock | 9.8% | 14.5% |
| Rock share (median) | 51% | 44.5% |
| First-try builds | 19 | 19 |
| Crystals (median) | 31 | 35 |
| Walk to the nearest nest (median, map cells) | 1,212 | 1,167 |
| Door / way round (dish cells) | 6-10 / 16-20 | 6-8 / 18-20 |
| Build (median of three runs, once a level) | 362 ms | 438 ms |

**In game** (the page): at zoom 1 the caves no longer read as discs on
tubes; rooms run into each other and tunnels narrow and open as they go. The
slime fills the tunnels it reaches, thin where they pinch. The landing's
cavern keeps its room. The rooms away from home show as wide dark floors
between the rock slabs at zoom 3. The caves' colours are unchanged (rock the
backdrop's grey), which is the drawing half of the same review item.

## Tried first

**Pinches down to 6 dish cells** (0.4 to 1.65 of a tunnel's width, floor 6
map cells either side): crystals were cut off on 17 of 20 caves' first
tries, 5 fell back to the open plan, and a build took 5.6 s on average. `walkable` narrows the
ground by a bin, so the check (and a squad) counts a pinch under about 8
dish cells as shut. See `Reports/dead-ends.md`.

## Not done here

- The caves' rock and backdrop colours and a rock texture (looks review item
  5's drawing half): the drawing threads'.
- Whether the caves still feel like a slog at 4x is a question for the
  owner's play; the walk to the nearest nest is 4% shorter, the rooms are new.
- A landing crystal per biome, nests off river crossings.
