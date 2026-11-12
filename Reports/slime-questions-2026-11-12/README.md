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

## The owner's answers (20:21Z), built

"1) remove the king rule. As long as the slime has food/crystal energy it
can make walkers. Headless. 2) I will playtest the shapes. Not sure what it
best from these."

- **Headless for every mould.** `Core::of` is `CoreKind::Headless`, so the
  plain mould and every tier (the Grudge keeps its pull of 16, the Sprawler
  its 0) make walkers while they have food; holding the home stops nothing.
  The home keeps its pull (8): the pull was what stopped random walking into
  the hub (random wiped the plain mould 11 of 32 with craters, 5 of 64 with
  the pull), and it is a draw, not a weak spot. The window drops the core
  ring and reads "Wipe out the mould, or out-harvest it and cut it down by
  15:00".
- **Smoke checks** (`raw/headless-smoke.txt`; `larderworld census
  seeds=1-8 hands=idle,random,farm,rush,strike cap=900`, CPU race world,
  10 min on 4 cores). The mould was wiped in 0 of 40 games (before: core
  taken by rush 34 of 64, strike 12, random 5, in the census above). Idle
  reaches the cap 8 of 8 (mean score -0.34); farm (+0.26) and the strike AI
  (+0.33) beat idle and random 8-0 each (sign test p 0.008); strike beats
  farm 8-0. Random loses its squad 7 of 8, the rush 4 of 8; the shortest
  game ends at 2:46 (a rush squad wiped). Idle and farm stand still for 0 s.
  Tuning notes, not vetoes: no hand wipes the mould in 15 minutes now, so
  every game is a stalemate scored at the cap; the review's escalation
  (`Rules::overrun`) and a mould that starves when its crystals are held
  are the next levers.
- **The shapes as keys** (`larder-world -- shape=a|b|c|d|e`, the films on
  the page) and **walker mass** (`Rules::mass`, `Rules::mass_for`: a walker
  hits for 5,000 / the start walkers at scale 2, so 0.2 at 25,000; the
  window and the census harness follow the walkers by default, `mass=`
  sets it). Idle squad on the Fork with both on (`raw/shapes/`, software
  Vulkan): A alive at 4:00 (3,855 walkers); B wiped at 2:50, was 0:47
  (48,617 walkers, the mesh never starves at upkeep 0.00001); C alive at
  4:00, was wiped at 1:56 (29,137 at 1:00, 10,869 at 2:00, 13,406 at 4:00);
  D alive at 4:00 (28,841, 14,733, 16,140); E alive at 4:00, but starves
  to 3,033 by 2:00.

## The owner plays D and C (22:01-22:17Z)

- **D:** "defeated mold d in 8:35, harvested 28k food, score of 2.43. First
  try." ... "I just harvested crystals until i got 40 soliders and hunted
  it down." The bots' 0-of-40 wipes (above) said nothing about a person.
- **C:** "I defeated c in 5:51 with a score of 2.61. C is a better slime
  than D".
- Notes on D (22:06Z, relayed): fire from one spot barely hurts a big body
  while walking in destroys it (the Larder thread owns the crater rule and
  is on it); "after killing almost all the mold, having the kill the few
  random stragglers spread across the map to win is annoying"; "There are
  still clearly lone walkers moving around"; building from afar went to a
  new thread.

## Stragglers: the broken mould

`examples/loneprobe.rs`: shape D on the CPU race world (scale 2), idle
squad, 4 seeds; at 180 s a sweep kills every walker within twice a
crystal's radius of one and all but 2% of the rest (the owner's endgame:
crystals held, stragglers everywhere else), then follows the remnant to
300 s.

- **A lone walker's own upkeep** (`BigParams::lone_upkeep`, the one-body
  dead end, re-tested because the front is now a sheet): the share of
  lone walkers at 180 s is 1.0% off, 1.2% at 3x, 1.4% at 10x; 10x reaches
  15.5 crystals by 180 s against 19.5 (fewer on 4 of 4). Rejected
  (`raw/lone/lone-upkeep-d.txt`; its sweep lines are the first sweep,
  which kept 2% of every walker, crystals too, and bred back to 3,000 in
  5 s).
- **The broken mould** (`Rules::broken` 0.03, `broken_starve` 50): under
  3% of its peak walkers the mould's upkeep is 50 times, and given back
  once it is over the line. After the sweep, without it the remnant
  (187-304 walkers at +5 s) finds food and grows back to 9,751-23,807 by
  300 s; with it the remnant is 90-135 at +5 s, 22-37 at +10 s, 3-10 at
  +15 s and gone by +25-30 s, on 4 of 4 seeds (`raw/lone/broken-off-d.txt`,
  `raw/lone/broken-on-d.txt`). A walker on a crystal still eats, so a
  remnant left on food can breed the mould back over the line.
