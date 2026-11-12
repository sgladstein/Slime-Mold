---
title: Sensor angle and reach
kind: mechanism
status: current
updated: 2026-11-12T17:15Z
sources: [Reports/mould-review-2026-11-10.md, Reports/mould-wild-one-2026-11-10.md, Reports/mould-probe-2026-11-07.md, handoff/project-memory/mould-depth-2026-11-09.md, Reports/slime-enemy-review-2026-11-12.md]
code: [Params::sensor_angle, Params::rotate_angle, Params::sensor_offset, Params::settle_seconds, Params::explore_angle, Dish::sensor_angle_now]
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
[slime enemy review](../../Reports/slime-enemy-review-2026-11-12.md)). Not
yet tried with growth in Larder World.

Each dish sets its own angle (`level.rs` `sensor_deg`). **Explore, then
settle** (`settle_seconds`, easing from `explore_angle` to `sensor_angle`)
is built and off (0).

**Reach stays at 9.** A shorter reach thins the tubes only a little (they
branch more), but the mould then senses food and oats from less far, so the
oat trails that solve each dish stop reaching. The thin look belongs in the
drawing ([the simulation look](../looks/simulation-look.md)).


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
- Sensor-angle page archive: [`Reports/mould-game-2026-11-08/pages/9-sensor-angle/`](../../Reports/mould-game-2026-11-08/pages/9-sensor-angle/index.html).
