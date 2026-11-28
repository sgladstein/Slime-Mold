# Formations on the move (2026-11-28)

Owner, 2026-11-28 06:12Z: "If I change a formation at rest the solider make
it good but on some when i tell them to move they get all disorganized."
Thread "Improve squad formations on the move".

## Why they fell apart

Measured with the ignored test `move_tests::probe_order` (CPU world, window
scale): squad 0 as a blob at spread steps 0, 2 and 4 and a ring at 0 and 3,
12, 24 and 40 soldiers, run seeds 1 and 2, on open ground and on the run's
planets 1 and 8, sent to the furthest crystal within 120 cells.

- On open ground every spread holds its shape (about half a cell from each
  place, the walk's own lag).
- On planets 1 and 8 the funnel (`funnel.rs`) measured the ground against the
  formation's full width, which the spread multiplies (2.4 times at the
  widest). A wide blob of 24 or 40 found narrow ground almost everywhere and
  walked 99% of the walk as a column (both planets, seed 1), its soldiers 11
  to 22 cells from their places; three seconds after arriving they were
  still 10 to 21 cells off.
- A column matched its soldiers to its places again every step: 25 to 80
  changes of place per soldier on one walk, the visible shuffle. Tight blobs
  did it too, whenever they funnelled.

## What changed

His card, 07:08Z: "Keep its spread, squeeze in gaps".

- `Rules::squeeze` (on): a moving squad squeezes its spread to the
  narrowest free width at its anchor, its next point and a formation's width
  ahead (`Game::squeeze_turn`), at once and a gap short of it, and opens up
  again at `WIDEN` (0.6 a second) past it. The funnel then makes a column only
  where even the tightest blob doesn't fit.
- In a column each soldier keeps its place, matched again once a second.
- `set=squeeze=0` plays the walk before.

## Before and after

`walks.txt` holds every walk (off: mean cells from its place while moving;
col: share of the walk in a column; swaps: place changes per soldier; kept:
spread walked against the formation's own; after: cells off 3 s after
arriving). The 36 walks at a spread above the tightest:

| Measure | Before | After | Better / worse |
|---|---|---|---|
| Cells from its place while moving | 9.24 | 6.22 | 29 / 4 (sign test p < 1e-5) |
| Place changes per soldier | 22.1 | 3.9 | 31 / 0 |
| Share of the walk in a column | 41% | 31% | 17 / 14 |
| Cells off 3 s after arriving | 8.5 | 7.1 | 19 / 11 |

All 60 walks: place changes 21.5 to 4.2 (47 better, 0 worse). Open: on
planet 1, seed 2, spread squads went into a column more often (24% to 63%
for a wide blob of 12); the column there still keeps its places.

Tried and reverted: ending a column once every soldier is within the
squad's own spread's width (not the squeezed one) cut those columns but
took a wide blob of 24 through the long 6-cell door in 20 to 22 s against
11 (`a_wide_squad_funnels_through_a_door_and_a_bridge`). Never matching a
column again: planet 8's 40 took 30 s to settle (`a_squad_crosses_planet_8
_and_settles`, bound 10 s).

## Films

Software Vulkan, `run=1:1 mode=squads hand=movetour,30,clear,wide
set=start_bank=100000 centre=squad,Z`, frames every half second from 2 to
30 s, played at twice speed, `set=squeeze=0` on the left. The two runs
part after a few seconds (the squads take different ways).
`wide-squad-z6.mp4`, `wide-squad-z12.mp4`, and a still at zoom 12
(`zoom12-before-after.jpg`): before, a wide squad strung out in a column
with stragglers; after, a squeezed blob.

## Part two: shapes and the drag ghost

The Formation page holds four shapes again, blob, ring, line and column (the
owner's 07:09Z "add column to the list"). Line and column face the way the
squad was last sent. Holding the right button on a move shows a ghost of the
squad's places at the target, and a drag sets the spread from 1.0 to 3.0.

- `line-ghost-tight-wide.jpg`: 24 soldiers on planet 1 at zoom 6, a line
  sent east, its ghost at spread 1.0 (left) and 2.6 (right).
- `column-ghost-z6.mp4`, `column-ghost-z12.mp4`: the same squad as a column
  at spread 1.3, sent east, forming two abreast along the move inside its
  ghost (13 s, real time).

Filmed with `larder-world look=DIR run=1:1 mode=squads set=start_bank=100000
centre=squad,Z card=buy20/column/ghost1.3 at=2,2.2,...`; the headless `ghost[X]`
step holds the move button with the squad sent 25 cells east.
