---
title: Sensor angle and reach
kind: mechanism
status: current
updated: 2026-11-17
sources:
  - Reports/gun-mind-2026-11-16/README.md
  - Reports/mould-probe-2026-11-07.md
  - Reports/mould-review-2026-11-10.md
  - Reports/mould-wild-one-2026-11-10.md
  - Reports/sim-use-2026-11-17/b-provoke-still.md
  - Reports/slime-enemy-review-2026-11-12.md
  - Reports/slime-questions-2026-11-12/README.md
  - handoff/project-memory/mould-depth-2026-11-09.md
code:
  - BigDish::mind
  - Dish::sensor_angle_now
  - MIND_DEAF
  - MIND_STILL
  - Params::explore_angle
  - Params::rotate_angle
  - Params::sensor_angle
  - Params::sensor_offset
  - Params::settle_seconds
  - mind_reads
---

# Sensor angle and reach

## Now

Each walker senses trail and scent at three points `sensor_offset` 9 cells
ahead, straight on and `sensor_angle` either side, and turns by
`rotate_angle` 45 degrees toward the strongest (Jones 2010). **The angle
sets the game's character**:

- **45 degrees** (the default, and Behind the Wall, Beat the Ruler, Running
  Dry): the mould obeys every oat, pulls tight and then stops moving.
- **22.5 degrees** (Keep It Alive, The Wild One): it never settles, keeps
  sprouting across gaps and joins food by itself, so what the player does is
  lost in its rewiring. On the free dish every hand, doing nothing too,
  joined all the food by 55-58 s (6 seeds, 7 hands; [project-memory
  note](../../handoff/project-memory/mould-depth-2026-11-09.md)).

On the big race world (25,000 walkers, one walker a cell, no food) 22.5
degrees with `sensor_offset` 5 keeps a fine mesh: 51 loops from 30 s to 60 s
against 12-13 at 45 degrees, tubes 7 cells wide against 17.6 (CPU, 2 seeds;
[slime enemy review](../../Reports/slime-enemy-review-2026-11-12.md)).
In the game look grown from the core on Fork (25,000 walkers) it goes from
a sheet to a mesh near the core with trunks out to the crystals, then mostly
trunks by 2:00 (one seed; [two questions page](../../Reports/slime-questions-2026-11-12/README.md)).
Not a default: the owner played D and C and picked C, today's 45 degrees
(11-12 22:19Z: "C is better how it looks, the shape it takes, how it
networks the crystals"); `larder-world -- shape=d` still plays this one.

Each dish sets its own angle (`level.rs` `sensor_deg`). **Explore, then
settle** (`settle_seconds`, easing from `explore_angle` to `sensor_angle`)
is built and off (0).

**Reach stays at 9.** A shorter reach thins the tubes only a little (they
branch more), but the mould then senses food and oats from less far, so the
oat trails that solve each dish stop reaching. The thin look belongs in the
drawing ([the simulation look](../looks/simulation-look.md)).

**Mind statuses** (big dish, both engines, 2026-11-16; the gun cards' Mind
bullets): a walker can carry a status for some steps (`BigDish::mind`, one
word a walker with its end step) that changes its three readings after
everything else has made them (`mind_reads`; a wall stays a wall). Blind
reads 0 and takes only the plain rule, so it walks straight on; sour reads
them backwards; spin turns `spin_curl` more after its turn each step. Since
the change comes last it works on every walker: a sour explorer flips twice
and so follows the trail, and a sour raider, speck or surging walker reads
its goal backwards and runs from the pump, crystal or squad it was heading
for ([Mind bullets](../../Reports/gun-mind-2026-11-16/README.md)).
Two kinds since 11-17 change what a walker hears and wants rather than what
it reads: deaf (`MIND_DEAF`, Provoke) hears no food news, so after the
wither lag it believes it is cut off, greys and panics; still
(`MIND_STILL`) neither panics, surges, raids nor breaks away and takes no
crystal ([Provoke and Still](../../Reports/sim-use-2026-11-17/b-provoke-still.md)).


## Owner's word

- 2026-11-09, after the 22.5 vs 45 page: picked "Keep 22.5, new goals" on
  a card ([project-memory note](../../handoff/project-memory/mould-depth-2026-11-09.md)).
  The code credits explore-then-settle to the owner the same day
  (`Params::settle_seconds` doc comment); it shipped off.

## Tried and rejected

- Reach 6, 7 or 8 for finer tubes: every planned solution got worse.
  [`dead-ends.md`](../../Reports/dead-ends.md), `Params::sensor_offset`;
  re-test when dishes are retuned for a shorter reach. Do not try 8.5.

## Open

- Whether explore-then-settle gives the tools their job at 22.5 is not
  measured on a shipped dish.
- Review 1's point stands: 45 has intention and visible consequence and no
  life; 22.5 has life and loses the consequence. Its proposed fix is on the
  tools (marks that stay), not the angle.

## Sources

- [mould-review-2026-11-10.md](../../Reports/mould-review-2026-11-10.md), *Summary*, *What playing showed*, *Looks*.
- [mould-wild-one-2026-11-10.md](../../Reports/mould-wild-one-2026-11-10.md), *Finer tubes*.
- [mould-probe-2026-11-07.md](../../Reports/mould-probe-2026-11-07.md), *What was built*.
- [gun-mind-2026-11-16](../../Reports/gun-mind-2026-11-16/README.md), *What was built*.
- Sensor-angle page archive: [`Reports/mould-game-2026-11-08/pages/9-sensor-angle/`](../../Reports/mould-game-2026-11-08/pages/9-sensor-angle/index.html).
