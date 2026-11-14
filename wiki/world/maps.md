---
title: Larder World maps
kind: design
status: open
updated: 2026-11-14
sources:
  - Reports/ground-kinds-2026-11-14/README.md
  - Reports/larder-maps-2026-11-11.md
  - Reports/larder-world-roadmap-2026-11-11.md
  - Reports/levelgen-2026-11-14/README.md
  - Reports/map-editor-2026-11-14/README.md
code:
  - BigDish::homes
  - BigDish::pile_fill
  - BigDish::pile_rs
  - BigDish::with_map
  - Editor::problems
  - Engine::with_map
  - G_DEEP
  - GpuWorld::with_map
  - Map::cut_off
  - Map::load
  - Map::of_dish
  - Mirror::of
  - Rules::home_dry
  - SOIL_BRIDGE
  - SOIL_DEEP
  - World::landing
  - build_wall_hp
  - deep_step
  - ground_bits
---

# Larder World maps

## Now

A Larder World can be a designed map, not only noise. The loader (ROADMAP WM1) reads `maps/<name>.png` (G byte: wet, rock, and the soil kind in the high nibble; R height is parked) and `maps/<name>.txt` (crystal, mould, landing and wall lines). `BigDish::with_map` builds the world with each crystal its own size and charge, the walkers shared among the `mould` starts, the larder on `landing`, and the walls at their own health. `GpuWorld::with_map` puts the same world on the engine. The gate passed: the Fork and the Nest play within 1% on the CPU and the GPU, and a saved noise world (`Map::of_dish`) loads back hash-identical ([report, *WM1*](../../Reports/larder-world-roadmap-2026-11-11.md)). Soil plays on the engine too since 2026-11-12, and the window draws it: one GPU tick on soil bands matches the CPU's, and the four soil maps' game twin stays within 2.1% ([report, *Soil on the GPU*](../../Reports/larder-world-roadmap-2026-11-11.md)). In the window the Rim's sand ring holds the mould in its clay core, and the Passes' sand lane breaks its tube ([page](https://claude.ai/artifact/1ifC4Qrw6fXe5Aq7vVSRzZ)). `larderworld census map=NAME` runs W2b's hands on a map. The five designed maps (v2: Rim, Fork, Flats, Nest, Passes), each drawn for a different winning plan, are the maps thread's ([plan and v2](../../Reports/larder-maps-2026-11-11.md)). Their WM2 gate is next. Generated planets (the roguelike run's levels, 2026-11-14) are maps in this format too, built from a seed and a depth ([planets](planets.md)). Maps can be drawn in the editor since 2026-11-14 (WM5, `cargo run --release --bin larder-edit -- map=fork`): six ground brushes, hexagon crystals, mould starts, the landing and walls, with mirroring and undo; it saves the same files under `maps/` and Enter plays the map in `larder-world` ([report](../../Reports/map-editor-2026-11-14/README.md), [page](https://claude.ai/artifact/NAJ89BJNhtkszVmFBWHutK)). Since 2026-11-14 the ground has two new kinds: deep water (soil kind 4 written with the wet bit, `G_DEEP`, so older builds play it as wet) stops soldiers and buildings, lets shots fly over, and slows the slime to 0.2 of its step with no births there; a bridge (kind 5) is plain ground across it, and shallow wet is the ford. `map::standable` is the one test of soldier ground, and `Map::cut_off` (the editor's warning) lists crystals and nests the landing can't reach on foot, so water never makes a map unbeatable. The River (`map=river`) is the first map with them; kinds 6-15 are free ([report](../../Reports/ground-kinds-2026-11-14/README.md), [page](https://claude.ai/artifact/9kdKYHvn4wv3zPHvsbfLQy)).

## Owner's word

- 2026-11-11 19:29Z: "We will want maps though. That should be part of the plan" (project chat; [decision](../decisions-game.md)).
- 2026-11-11 20:06Z: "I am skeptical how much fun/complexity/strategy this adds to the game" (on height, maps thread; height is parked).

## Tried and rejected

- Four rules that send a harvester home with what it carries when its crystal runs dry (WM1): under half its ask alone, with "the crystal can't fill the load", with a quarter-load floor, and with "the crystal holds under one frame's ask". Under W2b's score each cost the farm or wall hand at the cap (the narrowest: farm lower on 10 of 10), because that score counted a crystal with crumbs left as held ([dead end](../../Reports/dead-ends.md)). Under score v2 (charge taken) the narrowest costs nothing, so it is on (`Rules::home_dry`): the carrier walks home, banks and holds at the larder ([report, *Score v2 and the dry crystal*](../../Reports/larder-world-roadmap-2026-11-11.md)).

## Open

- The WM2 gate on each map: idle and random lose, and the winning plan changes between maps.
- Rival moulds as different species (the Passes' two starts are one species today).
- Deep water's own look (the terrain thread, soil kinds 4 and 5 in `view.wgsl`); until then it draws as wet and fords don't show.
- Map look (WM4). The editor has no keys yet for crystal charge or wall health, and no start-menu entry.

## Sources

- [Larder World roadmap](../../Reports/larder-world-roadmap-2026-11-11.md), *Maps in the plan* and *WM1: maps load into the world*.
- [Larder maps](../../Reports/larder-maps-2026-11-11.md), *Loader files*, *Which ground the mould feels* and *Maps, version 2*.
- [Map editor](../../Reports/map-editor-2026-11-14/README.md).
