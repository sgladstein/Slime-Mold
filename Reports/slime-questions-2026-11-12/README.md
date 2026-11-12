# Two questions about the slime (page, 2026-11-12)

The owner's ask (11-12 17:24Z), on the two questions from the
[slime-enemy review](../slime-enemy-review-2026-11-12.md): "should the king
rule go, and do you want mesh at the fronts with trunks behind? do we have
videos of these. explain the rules and questions in more detail".

Page: https://claude.ai/artifact/7LyE6uCco3xQNLukrcJFBz (source `page.html`,
films in `img/`, run scripts and logs in `raw/`).

- **King rule films.** `king-taken.gif`: the CPU census engine, race world
  seed 202, the census's rush hand (7 soldiers); the core falls at 0:48 and
  the mould goes 5,043 (0:08), 332 (0:32), 260 (1:00), 185 (4:00)
  (`raw/cpu-rush-202.log`; `larderworld look=DIR seeds=202 hands=rush
  every=4 cap=240`). `king-held.gif`: the GPU game look, race world seed 6,
  `core_pull=0`, the window's rush hand (4 soldiers, no reinforcements): the
  core never falls, the squad is gone by 1:40 and the mould regrows from 709
  (0:32) to 2,599 (3:00) (`raw/king0-6.log`). Two tries without
  `core_pull=0` (seeds 1, 2, 201, 202 in the window) never took the core
  either: the window's rush hand does not buy soldiers, the census's does.
- **Core census tally** (race world, 64 seeds, `core/census-pull8-race-*`
  of the Larder roadmap report): core taken by rush 34, strike 12, random 5,
  others 0; farm, wall and fort reach the cap 64 of 64 each; idle 62 of 64.
- **Front films** (Fork, idle, 25,000 walkers, `capacity=50000`):
  `front-angle.gif` (`sensor_angle=22.5 sensor_offset=5`: 29,051 walkers at
  1:00, 14,609 at 1:30, 15,031 at 2:00, squad alive) and `front-slow.gif`
  (`step=0.15 trail_scale=0.02 step_by=0.85:2`: 25,290 at 1:00, 3,025 at
  2:00) are new (`raw/front.sh`, `raw/*-dense.log`); `front-mesh.gif`,
  `front-sheet.gif` and `front-today.gif` are copies of the one-body page's
  `film-idle-all-pure-on`, `film-idle-all-dense-on` and `film-idle-all-B`.
