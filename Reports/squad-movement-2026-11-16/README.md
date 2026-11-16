# Squad movement, started over (2026-11-16)

Owner, 2026-11-16 05:17Z, of squad movement and pathfinding in Larder
World: "It is still clunky overall", start over rather than patch (after
the narrow-gap funnel of 11-15, #326). Squads must scale to 200 soldiers
(the soldier-cap thread, #380). Thread: *Make squad movement feel smooth*.

NUMBERS_AND_PAGE

## What was clunky

Measured with `src/mould/larder_world/move_tests.rs` (`measure_moves`,
CPU world at the window's scale 2, seeds 1-2) and filmed at 60 frames a
second with the window's own drawing (`src/bin/larder-world/film.rs`).
The measures, named before the runs (`move_tests::feel`), over the steps a
squad moves as one: **still** (share of soldier-steps a member barely
moved while its squad moved: stop-and-go), **jerk** (mean change of a
member's step from the last, in full walk steps: 0 smooth, 1 a stop or a
snap), **turn** (mean turn of a walking member's heading a step), **pace**
(the middle's speed over a soldier's), **back** and **settle** (after the
squad stops: cells walked back against the way it went, and seconds until
all stand), **waits** (walks and anchor steps that waited on a walking map
being built) and the movement's **cost** a step.

1. **Drawn 15 times a second.** The game steps 15 times a game second and
   the window drew every soldier where the last step left it, so at 60
   frames a second each soldier sat still for three frames and jumped on
   the fourth, while the camera eased after the squad smoothly: every
   figure shook on the screen. The 11-15 films saved one frame a step and
   could not show it. Figures also snapped to a new heading at once.
2. **No speed of its own.** Each step a soldier walked full speed straight
   at its place and stopped dead there; elbow room then shoved it. The
   anchor goes at three quarters of a soldier's speed and stopped dead
   whenever it was 4 cells ahead of the squad's middle, so members dashed,
   stood and dashed.
3. **A map per member.** A member whose place it could not see asked a
   walking map to that place, which moved with the anchor every step, and
   stood still while each was built.
4. **Places in rock went to the anchor.** Most of a big squad's places on a
   planet are in rock, so most of the squad walked to one point and shoved.
5. **45-degree legs.** Past rock both the anchor and the soldiers walked
   cell to cell down the map.
6. **Never settling.** Members whose place was taken pushed and were
   pushed for good after arriving.
7. **Cost.** Matching every member to every place each step (a sort of n²
   pairs), a list of members per soldier per question and allocating
   straight-line checks made 200 soldiers' movement 6-8 ms a step.

## How the good RTS games move groups

- **Group plan apart from each unit's walk.** Age of Empires moves a
  formation along one path at its slowest member's pace; units steer to
  their slots and the formation deforms through choke points and forms up
  after (Pottinger, [Coordinated Unit
  Movement](https://gamedeveloper.com/programming/coordinated-unit-movement)
  and [Implementing Coordinated
  Movement](https://gamedeveloper.com/programming/implementing-coordinated-movement),
  1999; [Pathing in Age of
  Empires](https://www.gdcvault.com/play/1027659/Pathing-in-Age-of-Empires),
  GDC; [formations
  lecture](https://faculty.cc.gatech.edu/~surban6/2018sp-gameAI/lectures/2018_02_08_formations.pdf)).
- **One field per goal, read by every unit.** Supreme Commander 2's flow
  fields: one integration field per goal shared by everyone going there,
  read as a direction at each unit and smoothed to any angle, with local
  steering on top; the cost is per goal, not per unit
  ([basic flow fields](https://howtorts.github.io/2014/01/04/basic-flow-fields.html),
  [continuum crowds](https://howtorts.github.io/2014/01/09/continuum-crowds.html)).
- **Velocity and steering.** Units have a velocity changed by bounded
  forces: seek, arrive (slow into the goal), separation, and queuing
  (brake behind a slower unit instead of pushing it) (Reynolds,
  [Steering Behaviors for Autonomous
  Characters](https://www.red3d.com/cwr/steer/gdc99/)).
- **StarCraft II**: units keep their offsets when the goal is outside the
  group's box and gather when it is inside ([magic
  boxes](https://liquipedia.net/starcraft/Magic_Boxes)); separation is a
  soft push.
- **Drawing a fixed step**: a game stepping at a fixed rate draws each
  object between its last two states, or everything moves in visible jumps
  (Fiedler, [Fix Your
  Timestep!](https://gafferongames.com/post/fix_your_timestep/)).

## What changed

The structure stays (a squad's anchor walks the way, members keep
formation places round it); how each soldier walks is new
(`src/mould/larder_world/steer.rs`, `Rules::steer`, on; `set=steer=0`
brings the old walk back for films and measures).

1. **A velocity of its own.** Each soldier has `Soldier::vx, vy`, changed
   by at most a full speed in `RISE` (0.2 s) and capped at its wading
   speed. Walking with its squad it takes the anchor's velocity and closes
   on its place from there (`CATCH`), so the squad walks in step instead of
   dash, stop, dash. Alone it slows into its goal (`ARRIVE`).
2. **Places that can't be stood in** (rock, past a wall, out of the
   anchor's sight) are a distance from the anchor instead, the place's own
   (`Game::stand_off`): a big squad round rock packs round whatever ground
   is free instead of 150 soldiers walking to one point.
3. **One walking map per squad.** A member that can't walk straight to its
   place follows the squad's own way (`Squad::map`: the anchor's map, or
   the map to the squad's goal), never a map of its own to a place that
   moves every step. A squad standing at a point uses one map to it.
4. **Any angle.** Down a walking map a soldier, and the anchor's straight
   step, head for the furthest of the next `LOOK` (10) cells it can walk
   straight to (`Game::ahead_on_map`), not cell to cell in 45-degree legs.
5. **Elbow room and queuing.** Two soldiers inside elbow room steer apart
   (`SPREAD`; overlaps under a twentieth of it don't push, so a packed
   squad stands still). A soldier walking into a slower one brakes to its
   pace; behind one of its own squad that stands, from a quarter of elbow
   room out, so it stops short instead of being pushed out step after step.
   One coming at it, or another squad's soldier standing in its way, it
   turns aside round, on the side it meant to pass (head on, its right);
   one of its own squad standing in its way steps on while the squad walks
   as one, or while the one behind is on a job (building, guarding): a
   builder stood behind a squad-mate in a wall's footprint until the wall
   shut it in (`wall_side_tests`, seed 3). Walking to its place in a squad that stands, it doesn't queue
   behind one its straight walk there passes clear of (a line's places
   stand a gap apart, inside that reach, and its members crept the last
   cell in for 3 s). It slides along rock and round a crystal's rim.
6. **The anchor.** It eases off as its squad falls behind (full pace up to
   half the lead, still at one and a half) instead of stopping dead at the
   lead; members that have all but stopped (jammed) are not waited for. Its
   lag is the members' middle's or their mean distance from where they
   head, whichever is more (`Game::steer_short`). It
   slows into its goal over `EASE` (0.5 s) and a let-go WASD squad stops
   where its members can stop from their speed, so nobody runs past its
   place and walks back. It arrives when its members stand round the goal
   or have all but stopped (each over its own wading speed, members
   fighting where they stand left out). Members are matched to places only
   when the squad starts, gains or loses a member, changes formation,
   starts or stops a column, or a line or wedge turns 30 degrees, not every
   step; after the nearest-pair match, any two whose walks are shorter
   swapped trade places (`UNCROSS`, up to 4 passes over every pair from a
   table of walks, under a millisecond for 200), so no two walks cross.
   Matched nearest pair first alone, the last few got the places across
   the squad and walked through it: a line of 40 formed from a blob still
   had members 5 cells off their places on arriving, queued behind the
   rank in front.
7. **Drawn between steps.** The window draws each soldier between where its
   last step began and ended (`Game::drawn_between`, `Soldier::was`) by how
   far the frame is to the next step, the camera follows the drawn middle,
   and a figure turns toward where it walks or aims at `TURN_RATE` (half a
   turn in a third of a second) instead of snapping. This is drawing only
   and also smooths the RTS round and the missions.

Columns through narrow gaps (`Rules::funnel`, 11-15) stay on: with the new
walk a squad queues at a door without them too, but stalls there (40
soldiers through the 6-cell door never arrive in 22 s without the column,
18.7 s with it; table below). A column now ends only once the squad's tail
is within the formation's width of the anchor: ended as soon as every
member could see the anchor, the tail of 24 stood in a 30-cell door.

## Rejected on the way

- **The anchor's lag measured from its formation's middle** (`shape_off`)
  with places in rock held at a stand-off: the members' middle lagged by
  more than the anchor waits for, and planet 1's 200 never arrived. Now
  from where the members head (`Game::steer_rest`).
- **The anchor waiting on its members' middle alone**: two squads of 200
  crossing each other never arrived in 26 s (both seeds): the middles kept
  up while the members were strung out round each other. With their mean
  distance from where they head as well: 22 and 24 s.
- **A map per member toward its moving place**: 5,600 waits on planet 8
  with 200 soldiers. The squad's own map instead (about 60).
- **Arriving only once the middle is within 2 cells of the goal**: a squad
  of 40 round a planet 8 crystal's rim, packed against itself, never got
  there and waited for good. It also arrives when its members have all but
  stopped.
- **Queuing reaching past elbow room behind every soldier**: braked squads
  walking in step (steer 200 jerk 0.09 -> 0.11, a 6-cell door's 40 walked
  back 0.18 -> 0.71 cells on arriving). Only behind a standing one now.
- **Braking alone when two soldiers meet**: two squads of 20 meeting on the
  8-cell bridge stood face to face for 14 s (28 s to cross, or never, on
  seed 2; main 19.6 s). Turning aside: 17.5 s.
- **The anchor stopping dead at its goal or on letting go of WASD**: every
  member ran past its place and walked back (0.36 cells each on the WASD
  move, 0.88 sent at 100). Easing in and stopping at braking distance: 0.00
  and 0.01.
- **Turning aside round its own squad's standing soldiers too**: at a
  6-cell door the queue spread along the wall (24 soldiers through in 19 s,
  the test's bar 16). They give way instead.
- **Heading for the next cell down the map when nothing was a straight
  walk**: a soldier on a cell's edge walked into a crystal's rim and stood
  (a squad of 8 sent round a cup of walls, seed 4, never got round). Nearer
  cells, then its own cell's middle, and a slide round the rim.
- **The crystal-disc test squared** for speed: it can differ from `hypot`
  at a rim's edge in f32, so walking maps (which test with `hypot`) and the
  walk could disagree and old replays would drift. A bounding box first,
  then `hypot`, keeps the speed and the old answers.

## Open

Ranked, from the independent review (brief: `review-brief.md`) and the
numbers above.

1. **Places fitted to the ground** (the review's first pick). Formation
   places are laid out for open ground and rock is patched several ways
   (stand-offs, the column, the squad's map, the anchor's wait). Flooding
   the open ground from the anchor whenever it enters a new cell and
   handing out lattice points in order of walk would let a blob pour
   through a corridor and re-form past it by itself, and give every member
   a place it can reach. First experiment: the blob alone on planet 8 with
   200 (still, pace, settle, share within a cell of its place).
2. **Big squads in caves** are still slow and slow to settle: on planet 8
   with 200 a member stands still in four steps of ten while the squad
   moves, and the last stand 20-30 s after it arrives.
3. **Formation sizes.** The blob spaces soldiers about twice elbow room
   apart, so 200 make a blob about 70 cells across; `STRAY` (14) is less
   than its radius, so the squad's middle and pace see only its inner
   members. A line of 200 is 8 wide and 25 deep. A ring or a wedge of 200
   is one soldier a gap round or along, 160 cells across: on open ground
   they walk as a packed knot (their places out of the anchor's sight) and
   spread 80 cells once there (10 cells walked back each, 10 s). Rings in
   several bands, a wedge with a cap, `STRAY` from the formation's size and
   a closer blob are cheap to try.
4. **Shots while walking** leave from where the game has the soldier, up to
   a step ahead of where its figure is drawn (a few pixels at close zoom).
5. **Cost**: `Game::place` counts the squad on every call, about five a
   soldier a step; a count kept a step would take most of what is left.
6. **Not measured yet**: wading in thick slime, an attack-move into slime,
   several squads at one crystal, linked squads.
7. **A wall through a standing squad** (`wall_side_tests::measure_wall_sides`,
   seeds 1-8): with the new walk 3 soldiers end past the wall on seed 8
   (the old walk: none). They are builders that stood on the far side, where
   the home-side spot by their nearest point of the wall is rock, so they
   raise it from where they stand (`goal_raw`, the wall-sides rule). The CI
   test (seeds 1-4) is clean. A home-side spot searched along the wall
   would fix it; that code is the walls thread's.
