# Wiki index

What is true now about the mould game, one page per subject. Read
[HANDOFF.md](../HANDOFF.md) first for where the game stands, then the page
for what you are about to touch. How the wiki works: [README.md](README.md).
Keep this file under 150 lines; lint checks it.

## Dishes

- [Keep It Alive](dishes/keep-it-alive.md): the endless dish; works, but which of two humps a game lands in is luck so far.
- [The Wild One](dishes/the-wild-one.md): a rival mould; scored by share of food eaten; first star is the opening, the rest is aim.
- [The lesson and the three puzzle dishes](dishes/puzzle-dishes.md): Behind the Wall, Beat the Ruler, Running Dry; pass their bar at 45 degrees.

## Tools

- [The lamp](tools/lamp.md): pushes tube aside without cutting it; dries ground on wet dishes.
- [The knife (snip)](tools/knife.md): a 4 s wound; the weakest tool, moves nothing in The Wild One.
- [Oats](tools/oats.md): the only tool that leads the mould somewhere new; a lean in under a second.
- [Salt](tools/salt.md): a wall walkers and scent cannot cross; one per dish; wins The Wild One's first star alone.

## Model

- [Water and sleep](model/water-and-sleep.md): dry mould sleeps, water wakes it; Keep It Alive only.
- [Sensor angle and reach](model/sensors.md): 45 obeys then freezes, 22.5 lives but ignores you; reach stays 9.
- [The mould at scale](model/scale.md): the GPU plan, the 1024 x 640 big dish and GPU timing test; why Jenson's clips look different (no limit, drawing, respawn); trunks from a dense start, contagion on the big dish, fronts still open.

## Looks

- [The simulation look](looks/simulation-look.md): the default; the mould drawn straight from trail, hunger and walkers; F7 the old tube look.
- [Sound](looks/sound.md): on by default, S toggles; every sound a count off the dish; the owner has not listened yet.

## Games in design

- [The RTS directions](rts/directions.md): build phase; on the field the habit cycle holds on 1 layout of 10 and salt overturns the opening; two demos.
- [The field skirmish](rts/skirmish.md): v1 (habits, salt, lamp) set aside after play; v2, a tithe spent on beacons and spores: one plan (soft) wins 127/128, no cycle.
- [How RTS games are designed](rts/rts-craft.md): outside research (22 sources) and what it says about our demos: dominant strategies and unmanaged snowballs, fix the structure first.
- [Larder, the mould as the enemy](rts/shooter.md): shooter; a held ring keeps the mould off your larder; priced, ring-forever never wins but ring-then-mow tops every seed: a ladder, not a cycle; M2 (spending, mould answers) still a ladder; M3 soldiers that fire from range loosened it (no plan best on 13+/16).
- [The world camera and UI](rts/world-camera.md): pan, zoom, minimap, GPU marks and Larder's controls over a world bigger than the screen; in the sandbox (W, Q, M); 0.3-0.4 ms a frame on the 1070.
- [Two Fevers](rts/two-fevers.md): playable in the browser; owned ground with a 15 s hold; plan beats random 16/16 but by 0.537; Hush is noise so far.

## Method and owner

- [The census](method/census.md): scripted hands over paired seeds, sign tests, controls, seed counts.
- [Decisions](decisions.md): the owner's rulings and decided choices, quoted with times.

## Not yet written

Write one only when there is a synthesis to add (rule 6). Candidates, with
the reports that would feed them: the free dish and the storm
(`mould-probe-2026-11-07.md`, `dead-ends.md`); food, scent and food flow
(`mould-probe`, *Food that flows*); the Wanderer and the Organ
(`mould-game-2026-11-08/prototypes/`); the forest floor
(`mould-real-world-2026-11-09.md`). The 2026-11-07 brainstorm is background,
linked from [Reports/README.md](../Reports/README.md), not compiled.
