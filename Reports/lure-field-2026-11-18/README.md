# The lure's field, in the game (2026-11-18)

The owner's card, 11-18 03:03Z: "Lure now, in the three planets". The Lab
lane's steering prototype ([report](../lab-steering-2026-11-17/README.md))
showed that one more term in the walkers' sensors, a walking-distance field
they lean down, draws a colony into a kill box. This is that field as a
slime rule of the game, built by the Enemy lane. The Lure building's return
to the run's shop is Squad and arsenal's change, and the field's rings on
the ground are Look and interface's (`View::set_steer`, #458; the game
calls it each frame while a field stands). Until the building is back,
nothing in a run stands a lure, so this change alone shows the player
nothing.

Page: https://claude.ai/artifact/EhGraPKu1QZvqUd54MGEWH (source `page/`; open `index.html`).
Films: `film.sh`; clips from their frames: `make_clips.py`.

## The rule

- **The field.** Once a game second the game takes every Lure that is whole
  and on, and every lure mark (`LureMark` in `Game::lure_marks`: a lure
  with no building, until its tick, for a thrown or shot lure), and builds
  the walking distance to the nearest on a worker thread, out to
  `Rules::lure_reach` (60 dish cells of walking; `set=lure_reach=N`). It is
  set a game second after it was started, so a game does not depend on the
  thread's speed, and cleared the second after the last lure goes
  (`raids.rs`, `time_lure_paths`). Walking distance goes round rock and
  walls, so a wall with a gap funnels what a lure draws.
- **The lean.** A calm walker whose block the field reaches adds
  `lure_pull` x (its own distance less the sensor's) to each sensor
  (`engine.wgsl`, the lure block in `sense`). A sensor on a block the field
  does not reach reads no lean.
- **The weight** is one engine key, `lure_pull`: 0 by default, and
  `run::LURE_PULL` 10 on every run planet (`run::enemy_keys`), Direction's
  to tune. The sandbox and the frozen campaign keep the scent-only lure.
- **Where it lives.** One word a 2 x 2 block (the distance + 1, 0 where no
  lure reaches) in the third quarter of the cells' region of the engine's
  `news` buffer, after the charge paths' and the far smell's blocks; the
  pull is Params word 844. Both the far smell and the lure are on at once.
  GPU engine only, no CPU twin (the owner's card, 11-17 19:21Z).

## Choices made here

- **Charging slime ignores a lure.** Only the calm branch of `sense` reads
  the field: a panicking, surging, raiding or breakaway walker is on its
  own errand, and a mini-boss's member follows its body. A lure draws the
  body at rest and is no shield from a charge. (Asked of the lane by
  Direction: "Whether a panic or surge ignores the pull is your call".)
- **Blind and Sour apply to it** as to any reading: a blinded walker does
  not see the lure and a soured one runs from it. Not filmed.
- **No hunger gate.** The far smell pulls only hungry walkers; the lure
  pulls fed ones too, since the lab's weights were measured that way.
- **A run key, not a global default.** The owner's card places the lure in
  the run; the campaign is frozen.
- **A second film key.** `lure=X,Y[,T]` already stands a Lure building in a
  film; `lure_mark=T,X,Y,SECS` lays a mark.

## Films (run seed 1, planet 1, software GPU, one film each)

The lab's scene: three rendering turrets standing from 0:20, eight soldiers
holding beside them, the lure between the turrets from 0:30. At 1:30:

| | Walkers killed | Food rendered | Slime alive | Walkers within 30 of the lure at 0:45, 1:00, 1:15, 1:30 |
|---|---|---|---|---|
| No lure | 1,813 | 382 | 15,397 | 1, 104, 67, 9 |
| A Lure building (150 health) | 4,461 | 1,087 | 12,981 | 144, 155, 57, 1 |
| A lure mark for 60 s | 5,978 | 1,489 | 12,355 | 242, 202, 365, 337 |

- The first raid, at 0:59, went for a turret with no lure and turned for
  the Lure building when one stood ("A raid on your turret turns for the
  lure", the game's rule since 11-15). The building was worn down by 1:00,
  half a minute after it stood, and the stream thinned from there.
- The lab's rig at pull 10 with a mark, with the far smell off: 6,024
  killed, 1,503 rendered, 13,929 alive; with no lure 1,623, 344, 15,915.
  The game's rule gives the lab's result with the far smell on.
- With a lure the home colony is smaller at 1:30 (12,355 against 15,397)
  and thinner over its crystal from 0:45: at pull 10 the lure takes part of
  the body, not only its rim.
- The clips' own films (ten frames a game second) differ from the table by
  under 1% (no lure 1,804 killed, the building 4,449): a film's frame times
  shift the game's steps.

## Frame cost

Not timed on the GTX 1070; it goes in the Milestone 1 batch.

- With no lure standing: nothing. The pull's Params word is 0 until a field
  is written, and no walker reads the field (the test plays pull 10 with no
  lure and pull 0 to the same hash).
- With a lure: a calm walker reads one more word of `news` a step (its own
  block), and three more where the field reaches it (one a sensor).
- The field: one block search over the planet a game second, off the frame
  on a worker thread (the charge paths' search, 2.0 ms on planet 3 and 7.4
  ms on planet 8 in [S1](../sim-use-2026-11-17/s1-unit-paths.md)), and one
  write of a word a block.

## Test

`a_lure_draws_the_calm_slime_down_its_field` (`src/gpu/larder_world.rs`), on
the software GPU: three race worlds alike. With a mark 25 cells from the
slime's home for 12 s at pull 10, the field is set a second after it is
started, is 0 walking at the mark and stops at the reach (2,281 of 10,240
blocks); at 12 s, 192 walkers stand within 8 cells of the mark against 27
with no lure; the field is cleared when the mark runs out; pull 0 builds no
field and plays the game with no lure to the bit.

## Not checked

- Planets 2 and 3 (the lab's films there went the same way, 1.5x and 2.7x).
- Whether pull 10 is right once the Lure costs 300 food and can be eaten:
  the building lasted 30 s here. That is Direction's number and Squad and
  arsenal's building.
- A lure behind a wall with a gap (the lab's fence film) on the game's rule.
