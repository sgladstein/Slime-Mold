# Cave gates off pools (2026-11-15)

The map-fix report ([report](../levelgen-mapfix-2026-11-15/README.md), *Not
fixed here*) left the last of the three planet reviews' map items open:
"Hedges on water in the caves rose (164 -> 217 pieces): they are gates across
tunnels with a pool in them. A gate is the pass's lock, so it stays; a gate
could move to the dry end of its tunnel next." The looks review had found
soft rock laid across water ([review](../levelgen-review-2026-11-15/README.md)).
A cave gate stood at its tunnel's middle, wherever the tunnel's pool was.

**Outcome:** a cave gate whose tunnel is wet at the middle moves along the
tunnel to the nearest dry place with rock at both ends. Where the tunnel is
wet all along, the gate stands on a sill: the water under it and 16-30 map
cells either side is let out, so the soft rock sits on a bank. Over 20 caves,
gates with soft rock in water went from 40 of 163 to 1 of 164, and pieces in
water from 138 of 652 to 2 of 656.

- Before: main at b016fd0f (PR #353; the caves are unchanged since the
  openings change, PR #345). After: this change.
- Page: [Cave Gates](https://claude.ai/artifact/VQ7tvbpMgYVTQrgoLy5KKD), copy in `page/`.
- Surveys: caves (planet 7), seeds 1-20,
  `cargo run --release --example planets -- seeds=1-20 depths=7 out=DIR`
  ([before](raw/survey-before.txt), [after](raw/survey-after.txt)).
- Soft rock on water: [`raw/gatewater.py`](raw/gatewater.py) on those maps
  ([before](raw/gatewater-before.txt), [after](raw/gatewater-after.txt)). A
  chain is a gate, ring arc or hedge; a piece is on water if a point on it,
  every 4 map cells, is wet.
- Gates moved and on sills: a debug line per wet gate in `build_with` (not
  committed).
- In-game frames: `larder-world` on llvmpipe, `hand=idle`, at 1 s, three gates
  that stood in a pool, at zoom 3 and zoom 6
  ([`raw/renders.sh`](raw/renders.sh)); pictures and the false-colour layouts
  by [`raw/make_pics.py`](raw/make_pics.py). Render times in
  [`raw/render-timing.txt`](raw/render-timing.txt).

## What changed (`src/mould/levelgen.rs`, cave gates only)

- **A cave tunnel keeps its way.** Carving a tunnel records a point every 8
  map cells along it, with the way's direction and the tunnel's half-width
  there (`Edge::way`; empty off the caves, so no other biome changes).
- **A wet gate moves along its tunnel.** The middle is wet if any ground
  across the tunnel (to 10 cells past each wall) is wet there or 6 and 12 cells
  either side along the way. Then the way's points are tried, nearest the
  middle first, for one that is dry by the same test with rock at both of
  the gate's ends (a tunnel, not a cavern). 19 gates moved, a median of 38
  map cells (at most 84).
- **Otherwise the gate stands on a sill.** Where no dry place has rock at both
  ends, the water across the tunnel under the gate and 16-30 cells either side
  (a ragged edge, by the fine noise) becomes dry ground; the clay under a pool
  stays, so the bank is dark. 49 gates stand on a sill, most of them gates
  whose own pieces were dry but had water within a few cells.
- **One more gate on cave seed 3.** A gate is left out where it would cross a
  road or a nest's door or breach. On seed 3 a gate whose tunnel's middle
  crossed one, and so was left out, now stands further along, off the road
  (gates 163 -> 164). Its checks pass.

New test `cave_gates_keep_out_of_pools`: on cave seeds 2, 3, 5 and 6 (which
rolled pools) no soft-rock piece crosses water. It fails on main (cave seed
2: soft rock at (1947, 345)-(1933, 344) in water at (1942, 344)). One dial
pin moved: the cave 7:2, which rolled pools (`MORE_PINNED`). `PINNED[6]` is a
dry cave and did not move.

## Measured

Map cells, two to a dish cell. Caves, seeds 1-20.

| | Before | After |
|---|---|---|
| Gates (and other soft rock) with a piece in water | 40 of 163 | 1 of 164 |
| Soft-rock pieces in water | 138 of 652 | 2 of 656 |
| Caves with soft rock in water (11 rolled pools) | 11 | 1 |
| Gates moved along their tunnel | | 19 (median 38, at most 84 cells) |
| Gates on a sill | | 49 |
| Dry caves (9) | | byte for byte the same maps |
| First-try builds | 19 of 20 | 19 of 20 |
| Build, mean per planet with retries (median of three runs) | 595 ms | 571 ms |
| Wet ground, share of the map | | 1 point less on 4 of the 11 pool caves (seed 11: 18% -> 17%) |
| Walks, open charge, doors (direct/round per nest) | | the same on every seed |

The one left: a gate on cave seed 18 touches water at one sample point, at a
joint of its pieces.

**In game** (the page): on seeds 2 and 11 the gate stood across a pool with
water at one side or both; after, it stands at the tunnel's dry end. On seed
16 the tunnel is a pool all along; after, the gate stands on a dry bank with
water past it on both sides.

## Tried first

- **Moving the gate alone:** 40 -> 31 gates in water. Many tunnels are wet
  all along, so the sill was added.
- **A sill 14 cells either side:** at zoom 6 the bank read as a thin dark line
  beside the gate. 16-30 cells, ragged, reads as a bank.

## Not done here

- Whether the dark clay bank on a sill reads well at zoom 6 and closer is for
  the owner's eyes: the page shows it on seed 16.
- The three reviews' map items are now all done.
