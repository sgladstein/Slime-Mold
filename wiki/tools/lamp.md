---
title: The lamp
kind: tool
status: paused
updated: 2026-11-10
sources: [Reports/mould-probe-2026-11-07.md, Reports/mould-keep-alive-2026-11-10.md, Reports/mould-wild-one-2026-11-10.md, Reports/mould-review-2-2026-11-10.md, Reports/mould-looks-2026-11-10.md]
code: [Params::push, Dish::push_light, Params::lamp_reach, LAMP_R, Params::light_weight, Params::light_fade]
---

# The lamp

## Now

Held and unlimited. Light reaches `LAMP_R` 16 cells. Since the
2026-11-08 playtest it **pushes**: each step it carries a share of the trail
in lit cells, and lit walkers, one cell outward (`Params::push` 0.9,
`push_light`), so a tube held under it parts round the light and stays
joined. The old sensor-repulsion levers (`light_weight`, `light_fade`) are
kept at 0. The mould recoils in about 0.02-0.07 s.

It has a second job on wet dishes: it **dries the ground**, which puts mould
to sleep ([water and sleep](../model/water-and-sleep.md)). It steers and
shapes but cannot lead the mould somewhere new; oats do that. In
[The Wild One](../dishes/the-wild-one.md) the lamp alone wins (8-0), and the
scripted warden holds it for 102-150 s of 150. In
[Keep It Alive](../dishes/keep-it-alive.md) it is on for only 3-22 s of a
3-10 minute game.


## Owner's word

- None recorded about the lamp alone. The 2026-11-08 playtest's top complaint
  (testers, not the owner) was that the lamp erased the tube it was meant to
  steer; that drove the push lamp.

## Tried and rejected

- The lamp as a repulsive term in every sensor, with a trail fade and burn
  (the Jones-model way): cut the join on 5 of 5 seeds at every setting tried.
  [`dead-ends.md`](../../Reports/dead-ends.md), `Params::light_weight`;
  re-test if the lamp is meant to cut.

## Open

- In The Wild One the second and third stars need the lamp, snips and oats
  aimed within about 2 cells; which of the three needs the precision is not
  separated yet.
- The looks pass draws the lamp as the light the simulation shines
  ([looks report](../../Reports/mould-looks-2026-11-10.md)).

## Sources

- [mould-probe-2026-11-07.md](../../Reports/mould-probe-2026-11-07.md), *Measurements*, *Playtest fixes*.
- [mould-keep-alive-2026-11-10.md](../../Reports/mould-keep-alive-2026-11-10.md), *Water and sleep*.
- [mould-wild-one-2026-11-10.md](../../Reports/mould-wild-one-2026-11-10.md), *Numbers*.
- [mould-review-2-2026-11-10.md](../../Reports/mould-review-2-2026-11-10.md): lamp time per game.
