---
title: Larder World maps
kind: design
status: open
updated: 2026-11-12T16:44Z
sources: [Reports/larder-world-roadmap-2026-11-11.md, Reports/larder-maps-2026-11-11.md]
code: [Rules::home_dry, Map::load, Map::of_dish, BigDish::with_map, Engine::with_map, GpuWorld::with_map, World::landing, BigDish::pile_rs, BigDish::pile_fill, BigDish::homes, build_wall_hp, ground_bits]
---

# Larder World maps

## Now

A Larder World can be a designed map, not only noise. The loader (ROADMAP WM1) reads `maps/<name>.png` (G byte: wet, rock, and the soil kind in the high nibble; R height is parked) and `maps/<name>.txt` (crystal, mould, landing and wall lines). `BigDish::with_map` builds the world with each crystal its own size and charge, the walkers shared among the `mould` starts, the larder on `landing`, and the walls at their own health. `GpuWorld::with_map` puts the same world on the engine. The gate passed: the Fork and the Nest play within 1% on the CPU and the GPU, and a saved noise world (`Map::of_dish`) loads back hash-identical ([report, *WM1*](../../Reports/larder-world-roadmap-2026-11-11.md)). Soil plays on the engine too since 2026-11-12, and the window draws it: one GPU tick on soil bands matches the CPU's, and the four soil maps' game twin stays within 2.1% ([report, *Soil on the GPU*](../../Reports/larder-world-roadmap-2026-11-11.md)). In the window the Rim's sand ring holds the mould in its clay core, and the Passes' sand lane breaks its tube ([page](https://claude.ai/artifact/1ifC4Qrw6fXe5Aq7vVSRzZ)). `larderworld census map=NAME` runs W2b's hands on a map. The five designed maps (v2: Rim, Fork, Flats, Nest, Passes), each drawn for a different winning plan, are the maps thread's ([plan and v2](../../Reports/larder-maps-2026-11-11.md)). Their WM2 gate is next.

## Owner's word

- 2026-11-11 19:29Z: "We will want maps though. That should be part of the plan" (project chat; [decision](../decisions-game.md)).
- 2026-11-11 20:06Z: "I am skeptical how much fun/complexity/strategy this adds to the game" (on height, maps thread; height is parked).

## Tried and rejected

- Four rules that send a harvester home with what it carries when its crystal runs dry (WM1): under half its ask alone, with "the crystal can't fill the load", with a quarter-load floor, and with "the crystal holds under one frame's ask". Under W2b's score each cost the farm or wall hand at the cap (the narrowest: farm lower on 10 of 10), because that score counted a crystal with crumbs left as held ([dead end](../../Reports/dead-ends.md)). Under score v2 (charge taken) the narrowest costs nothing, so it is on (`Rules::home_dry`): the carrier walks home, banks and holds at the larder ([report, *Score v2 and the dry crystal*](../../Reports/larder-world-roadmap-2026-11-11.md)).

## Open

- The WM2 gate on each map: idle and random lose, and the winning plan changes between maps.
- Rival moulds as different species (the Passes' two starts are one species today).
- Map look (WM4) and an editor (WM5).

## Sources

- [Larder World roadmap](../../Reports/larder-world-roadmap-2026-11-11.md), *Maps in the plan* and *WM1: maps load into the world*.
- [Larder maps](../../Reports/larder-maps-2026-11-11.md), *Loader files*, *Which ground the mould feels* and *Maps, version 2*.
