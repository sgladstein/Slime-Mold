# The Trunk's fort (2026-11-18)

The owner in the Enemy lane's thread, 11-18 03:11Z, on the walls that form
where the Trunk is shot ([the switch settlement](../enemy-switches-2026-11-17/README.md)):
"This is interesting and I want to play test it, but when it was original
suggested, I thought the walls would form protecting the core of the slime
or the slime crystals giving them a base of sorts". This is that base, built
by the Enemy lane as a slime rule of the GPU engine: on planet 3 the Trunk
walls each crystal it holds, with gates. Hardening where it is shot stays
on, so both can be played.

Page: https://claude.ai/artifact/1D13WjouotXkrob7zpPWHt (source `page/`; open `index.html`).
Films: `film.sh`; clips from their frames: `make_clips.py`.

## The rule

- **Who holds a crystal.** Every 2 game seconds (`FORT_EVERY`) the game
  sums the walkers on each crystal's disc from the walkers' grid
  (`Game::time_fort`, `bulwark.rs`). A crystal with a walker on a quarter
  of its cells is held (`FORT_HOLD`), until it is dry or the cover falls
  under a twentieth (`FORT_LOSE`). So a raid on a crystal the squad is
  draining leaves no ring, and a crystal the slime is driven off loses its
  walls.
- **The rings.** Each held crystal gets a ring 3 dish cells out from its
  edge (`FORT_GAP`) and `Rules::fort` dish cells thick, with four gates
  (`FORT_GATES`) 4 dish cells wide (`FORT_GATE`). The home crystal's first
  gate faces the middle of the map and every other crystal's faces home,
  where its tube runs; a ring whose gates would face rock is turned until
  at least two open on ground (`open_gate`). Crystals close together share
  one outline: no wall stands inside another ring's court (`fort_mask`).
- **The walls.** The rings are a mask, one word a 2 x 2 block, written to
  the engine when a crystal is taken, lost or dry (`World::set_fort`). A
  walker on a masked block with crust under it (`set_crust` 0.2) sets with
  no scar and stays set (`fort_at`, `set_step` in `engine.wgsl`); off the
  mask the rule is as before, so shot slime still sets where it is hit. A
  set walker takes three hits and neither moves nor feeds, and five round a
  cell make it wall to soldiers (`set_block` 5, the bulwark of 11-17). With
  the mask cleared the walkers wake.
- **The slime's own ways** (charges, the far smell, the lure, raids) are
  now built on the ground less the wall cells (`Game::slime_ground`,
  `raids.rs`), so they go through the gates and round a wall the shots
  made. A walker cannot step into a cell a set walker holds, so before
  this a way through the slime's own wall pressed a charge against it.
- **The weight** is one rule, `fort`: 0 by default, 3 in `run::TRUNK_RULES`
  (`set=fort=N` on the command line; 0 is the Trunk of 11-17). It is on for
  one tier, so it has no row in the switch list.
- **Where it lives.** The fourth quarter of the cells' region of the
  engine's `news` buffer, after the charge paths', the far smell's and the
  lure's blocks; Params word 888 is 1 while a mask is written. GPU engine
  only, no CPU twin (the owner's card, 11-17 19:21Z).

## Choices made here

- **Rings round the crystals it holds, not one keep.** His note names "the
  core of the slime or the slime crystals". The slime has no core body
  since the no-king ruling, so its home crystal is the core and each
  crystal it takes is an outwork.
- **Gates.** A closed ring would starve the court of new walkers and shut
  a squad out of a crystal for good. Four gates keep the tube running and
  give a squad a way in that the slime also comes out of.
- **Thickness 3.** At 2 the ring was one or two walkers deep and read as a
  line (972 wall cells at 0:30 against 1,551 at 3, same film).
- **The crust decides when.** A walker sets only on crust, which grows
  where slime stands, so a ring appears 10 to 15 s after a crystal is held
  and not at the moment of the mask.
- **Tried and dropped:** a rule that a set walker on the ring keeps the
  crust under it up, so that old walls would still burn. A fire lit on the
  wall at 2:30 burnt the same without it, so it is not in the engine.

## Films (run seed 1, planet 3, software GPU, one film each)

The squad stands still where it lands (`hand=idle`), with the fort and
with `set=fort=0`:

| | Slime alive at 1:00, 2:00, 3:00 | Walkers set | Wall cells | Soldiers at 3:00 |
|---|---|---|---|---|
| The fort | 44,191, 29,863, 33,801 | 1,895, 2,806, 3,916 | 1,829, 2,708, 3,681 | 1 of 4, health 52 |
| No fort | 44,738, 31,541, 37,314 | 0, 452, 1,321 | 0, 449, 1,339 | 1 of 4, health 1 |

- The home ring: no walker is set at 0:10, 1,422 are at 0:15 (1,371 wall
  cells), 1,608 at 0:30 and 1,672 at 0:40.
- By 3:00 the second big crystal and four of the southern crystals have
  walls too (`page/map-3min.jpg`). Where two crystals' rings meet, the
  walls are the arcs of one outline.
- The slime with a fort is 9% smaller at 3:00. A set walker does not feed,
  and about a tenth of the slime stands in the walls.
- Idle does not win: the raid reaches the squad in both films.

The scripted rush (`hand=greedy`):

| | Soldiers at 1:00, 2:00 | Walkers killed | Slime alive | Walkers set at 1:00, 2:00 |
|---|---|---|---|---|
| The fort | 12, 6 (health 61) | 3,734, 4,938 | 40,147, 19,362 | 7,067, 9,677 |
| No fort | 12, 0 | 3,611, 4,712 | 40,362, 20,062 | 6,330, 9,370 |

The rush reaches the second crystal before its ring forms, and from 1:00
most of the set walkers in both films are the ones its shots made. The six
soldiers against none is one seed's raid, not a result.

## Fire on the wall

`P-fire-z6`: the squad standing still, a fire lit at 1:00 on the home
ring's south stretch (`light=60,43.5,48,3`, dish cells).

| Time | Cells burning | Cells charred | Walkers set | Wall cells | Walkers killed | Slime alive |
|---|---|---|---|---|---|---|
| 0:58 | 0 | 0 | 1,889 | 1,823 | 69 | 44,128 |
| 1:01 | 36 | 0 | 1,808 | 1,734 | 283 | 44,158 |
| 1:10 | 123 | 5 | 2,326 | 1,660 | 1,481 | 43,994 |
| 1:20 | 171 | 54 | 3,463 | 1,673 | 4,778 | 42,648 |
| 1:30 | 126 | 108 | 4,593 | 1,673 | 8,808 | 28,276 |
| 2:00 | 61 | 117 | 6,385 | 2,452 | 12,149 | 25,855 |
| 2:10 | 51 | 105 | 6,757 | 2,583 | 12,990 | 26,677 |

With no fire, at 2:00: 2,806 set, 2,708 wall cells, 1,888 killed, 29,863
alive (`G-idle-on`). Charred counts cells with char over 0.3.

- The south stretch burns out between its two gates. The fed slime in the
  gates does not burn, so the fire does not run round the ring: the other
  three stretches stand. The burnt stretch had not set again at 2:10.
- The fire does not stop at the wall. It runs on over the crusted ground
  and the thin, hungry slime outside the ring to the south and east, and
  most of the 10,000 more walkers killed by 2:00 are its.
- Slime round a fire is alarmed, and alarmed slime sets where it stands
  (the 11-17 rule), so more than twice as many walkers are set by 2:00 and
  the wall cells come back to about where they were, mostly away from the
  ring.

## Frame cost

Not timed on the GTX 1070; it goes in the Milestone 1 batch.

- On every planet but the Trunk's: nothing in the engine (`fort_at` returns
  on its flag, and only where setting is on), and `time_fort` returns on
  its rule.
- On the Trunk's planet: each walker that is in no lump reads one more word
  of `news` a step, its own block's.
- The game's look: every 2 game seconds one copy of the walkers' grid (the
  square every other reader asks for, so no new read-back) and a sum over
  each crystal's box. The mask is rebuilt and written, one word a block,
  only when a crystal is taken, lost or dry.
- The slime's ways: one pass over the ground to clear the wall cells when a
  way is started (once a game second at most, a way).

## Tests

- `the_forts_mask_rings_a_crystal_with_gates` (`bulwark.rs`): the mask's
  court, band, four gates, their turn with the ring, and two overlapping
  rings sharing one outline.
- `the_slime_walls_the_crystal_it_holds` (`src/gpu/larder_world.rs`), on
  the software GPU: two race worlds alike with setting on and nobody
  shooting, `fort` 2 and 0. At 20 s 312 walkers are set, none off the ring, the bulwark has 209
  cells, and 2,139 walkers are alive against 2,153 with no fort. With the rule turned off the mask
  is cleared and no walker is set 4 s later.
- `every_ladder_tier_does_something_to_you` holds the fort to the Trunk.

## Not checked

- A squad fighting its way through a ring by hand. The Trunk's blanket is
  wide, so most of a wall stands inside slime.
- Whether a wall chewed by shots refills. The burnt stretch did not, in the
  70 s the film ran: its crust burnt with it.
- Any planet but 3 of run seed 1, and a crystal in a pocket of rock
  (`open_gate` has no film).
- The look. Set slime is drawn as a flat amber slab, which was a detail
  when it formed only under fire and is now a standing part of the
  picture; Look and interface reviews the clips.
