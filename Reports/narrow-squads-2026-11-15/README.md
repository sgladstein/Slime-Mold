# Squads through narrow gaps (2026-11-15)

Owner, 2026-11-15 03:00Z: "We need to improve the squad movement in
pathfinding it gets awkward moving with large squads through narrow areas."

Page (films with counts): https://claude.ai/artifact/Pa4SQzNxAAp9WSTrikKZfS,
source `index.html` and `page/` here.

## What was wrong

A squad moving as one keeps its formation round an anchor
(`Reports/squad-mode-2026-11-14`). A place in rock or deep water falls back
to the anchor itself (`Game::place_at`). At a bridge or door narrower than
the blob, most of the squad's places were in the rock, so most of the squad
walked to one point and elbow room shoved them apart: a 24-soldier blob went
through an 8-cell bridge as one knot, up to 9 other soldiers within one
formation gap of a soldier.

## What changed (`src/mould/larder_world/funnel.rs`, `Rules::funnel`, on)

- The squad looks a formation's width plus a gap ahead along its way
  (`guide`). Where the free ground across the way there or at the anchor is
  narrower than its formation (`Game::across`, `Game::formation_width`), it
  funnels.
- Funnelling, the anchor jumps to the squad's front (its furthest member
  along the way), and the places become rows a gap apart back along the
  anchor's trail (`Column::trail`). Each row is as wide as the ground there
  allows, at most the square root of the squad's size, at most one wider
  than the row ahead (a taper into the mouth), and centred on the free
  ground across the way.
- A funnelling head waits only for the front of its queue: no member within
  `LEAD` of it.
- It stops funnelling once the ground at the head and ahead is wide again and
  every member has a straight walk to the anchor. The anchor then waits for
  the squad's middle as before, so the squad forms up just past the gap.
- `Game::place_at` asks no straight line for a column's place (it is on the
  way the anchor came, often round a corner from it).

## Measured (`narrow_tests::measure_bridge`, CPU world, seeds 1-2)

Three gaps (`narrow_tests::gaps`, saved as maps in `maps/`): the River test
map's bridge (8 cells wide, 16 long), a door 6 wide in rock 30 long, and a
door 8 wide in rock 10 thick approached at a slant. The squad is sent from
the landing to a point past the gap. Decided before the runs: the worst knot
near the gap (most other soldiers within one gap of a soldier within 20 cells
of the gap), with the time until all are through as the cost.

Worst knot, seeds 1 and 2, before -> after:

| squad | bridge 8 | door 6 long | door 8 slant |
|---|---|---|---|
| 12 | 5, 5 -> 6, 6 | 5, 5 -> 6, 6 | 5, 5 -> 6, 6 |
| 16 | 5, 5 -> 6, 7 | 6, 6 -> 6, 6 | 5, 5 -> 7, 7 |
| 24 | 9, 8 -> 7, 7 | 10, 9 -> 6, 6 | 7, 7 -> 8, 9 |
| 32 | 10, 10 -> 7, 8 | 13, 14 -> 6, 6 | 9, 10 -> 9, 8 |

Smaller on 10 of the 12 runs of 24-32 soldiers at the straight gaps (bridge
and long door: 8 of 8), the same or worse at the slant and for 12-16, which
never knotted badly. Too few runs for a sign test to mean much (the 8 of 8 at
straight gaps is p 0.008 one-sided, but seeds 1 and 2 are near-identical
starts); the films are the main evidence.

Time until all 24 are through: bridge 12.8 -> 13.9 s, long door 13.3 ->
13.9 s, slant 15.0 -> 15.6 s; 32 soldiers 0.6-1.6 s later. A queue is slower
than a shove.

Frame cost (`narrow_tests::time_funnel`, 32 soldiers, long door, release,
cloud CPU): the squads' turn 82 -> 88-96 microseconds a step, half the steps
funnelling. Not timed on the 1070; it is CPU game code, the same on both
worlds.

## Rejected on the way

- **The head waiting for the squad's middle while funnelling.** The column's
  places reach back along the trail, so the middle of a squad queuing at a
  door lags its places' middle by more than `LEAD` while everything goes
  well: the head stood at the door's mouth for 0.6 s until the squad had
  squeezed up behind it, 2 s slower in all for 24 soldiers.
- **Looking half a formation ahead.** The front of the blob met the rock
  before the column started, and the squad crushed into a square at the
  mouth. It now looks a whole formation's width ahead.
- **Rows all as wide as the ground allows.** The squad stood square across
  the door's mouth; a taper of at most one more a row reads as a funnel and
  queues better.

## Tests

- `a_big_squad_funnels_through_a_door_and_forms_up_after`: 24 through the
  6-cell door, worst knot at most 7 and below the old rule's on seeds 1-2,
  all through within 16 s, out of the column on arrival, and no wider after
  than the old rule's blob plus a cell.
- `a_steered_squad_crosses_a_bridge_and_open_ground_stays_a_blob`: steered
  with WASD from off to one side it funnels and crosses; steered along the
  far bank it funnels under half the steps.

## Films

`raw/film.rs` (append to `src/bin/larder-world/tests.rs` to film):
`FILM=DIR FILM_MAP=Reports/narrow-squads-2026-11-15/maps/door6long FILM_N=24
FILM_FUNNEL=1 FILM_GOAL=190,114 FILM_CENTRE=130,114 cargo test --release
--bin larder-world film_narrow -- --ignored`. Software Vulkan (llvmpipe),
zoom 6. GIFs from frames 50-199 at 15 fps with ffmpeg.

## Open

- The slanted approach still bunches a moment at the mouth: the anchor's
  trail runs along the wall, so the first rows behind the head press into it.
- The rule decides by the ground across the way at two points; a winding
  pass narrower than the squad between them is met as it comes.
- No owner playtest yet.
