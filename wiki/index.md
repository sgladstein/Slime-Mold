# Wiki index

What is true now about the mould game, one page per subject. Read
[HANDOFF.md](../HANDOFF.md) first for where the game stands, then the page
for what you are about to touch. How the wiki works: [README.md](README.md).
Keep this file under 150 lines; lint checks it.

## Dishes

Paused (owner, 2026-11-10 20:55Z): not tuned as a side effect of other
work; a thread working on one directly may change it.

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
- [The mould at scale](model/scale.md): the GPU plan; phases 1-3 done, fields stay f32, 1.37 ms a frame at 100k walkers on the 1070; phase 4 starts as Larder World W0.
- [The GPU sandbox](model/gpu-sandbox.md): the big dish live on the GPU with every tested option as a control; matches the CPU; 1.45 ms a frame at 100k.
- [Outside looks and creature rules](model/outside-looks.md): why Jenson's clips look different; trunks, contagion, species, explorers, grazing droplets; fronts still open.
- [Growth and starving](model/growth.md): the big dish's walkers eat, divide and starve (off by default); size follows food; matches on the GPU; on crystals that run dry it eats only its roads.
- [The life cycle and the answer to attack](model/life-cycle.md): spores, sleep and an alarm; Sporer, Sleeper and Swarmer each beat their control 8/8 and have a counter.

## Looks

- [The simulation look](looks/simulation-look.md): the default; the mould drawn straight from trail, hunger and walkers; F7 the old tube look.
- [Art direction](looks/art-direction.md): the style guide for Larder World's look (living light on dark ground); the 11-14 review's ten issues and which thread has each.
- [Sound](looks/sound.md): on by default (S in the dish, O in Larder World); every sound a count off the game; Larder shots weigh their kills; the owner has not listened yet.

## Larder World

- [Larder World](world/larder.md): a squad against several mould species in a persistent GPU world with crystals; phases W0-W5; W1a fields done; the small-dish M1-M3 history.
- [Larder World maps](world/maps.md): designed worlds from a picture and an item list (WM1 loader, CPU and GPU); five maps each drawn for a different plan.
- [The Larder player AI](world/player-ai.md): plays soldiers through the player's commands, one brain per side; on the Larder world four of five styles beat the scripted farm 15-16/16 held out.
- [The player's own mould](world/own-mould.md): gardeners (owner, 11-12); soldiers grow a second species that fights the enemy by conversion; forms trade eating for fighting.
- [The mould tiers](world/mould-tiers.md): the difficulty ladder; Drifter, Sprawler, Trunk (the campaign), Sporer, Sleeper, Swarmer, and Grudge, which remembers where it was shot.
- [The world camera and UI](world/world-camera.md): pan, zoom, minimap, GPU marks and Larder's controls and the game UI over a world bigger than the screen; view 0.3-0.4 ms, UI 0.1 ms on the 1070.
- [The squad toolkit](world/squad-toolkit.md): six tools from the mould's model (blast and crater, mine, det cord, germ, sleep gas, smell); with tools carried, the best plan changes by tier.

## Other games in design

- [The RTS directions](rts/directions.md): build phase; three demos, each with one strategy on top so far; the main line is now Larder World.
- [The field skirmish](rts/skirmish.md): v2 budget skirmish; soft won 127/128; Strike and Fortify dethrone it, but brace then wins every pairing.
- [How RTS games are designed](rts/rts-craft.md): outside research (22 sources) and what it says about our demos: dominant strategies and unmanaged snowballs, fix the structure first.
- [Two Fevers](rts/two-fevers.md): playable in the browser, version 3 (call, seed, pinch): Trunk > Web > Raider, a ladder, and the leader at 1:30 keeps it; not yet played by hand.

## Method and owner

- [The census](method/census.md): scripted hands over paired seeds, sign tests, controls, seed counts.
- [Decisions](decisions.md): the owner's standing rulings, quoted with times.
- [Game decisions](decisions-game.md): the owner's decided choices about each game, quoted with times.

## Not yet written

Write one only when there is a synthesis to add (rule 6). Candidates, with
the reports that would feed them: the free dish and the storm
(`mould-probe-2026-11-07.md`, `dead-ends.md`); food, scent and food flow
(`mould-probe`, *Food that flows*); the Wanderer and the Organ
(`mould-game-2026-11-08/prototypes/`); the forest floor
(`mould-real-world-2026-11-09.md`). The 2026-11-07 brainstorm is background,
linked from [Reports/README.md](../Reports/README.md), not compiled.
