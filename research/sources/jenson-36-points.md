---
id: jenson-36-points
title: 36 Points (and Primordium)
authors: Sage Jenson
year: 2019-2022 (36 Points); 2022 (Primordium)
venue: Feral File; https://sagejenson.com/36points/
url: https://sagejenson.com/36points/
read: viewer page only (JavaScript, no model text); model read through Bleuje's port (2026-11-09)
copy: none
licence: CC BY-NC-SA 3.0 (per the viewer page)
used-in: ../../Reports/mould-game-2026-11-08/depth-2026-11-09/sources-research.md, ../../Reports/jenson-physarum-2026-11-10.md, ../../Reports/gpu-plan-2026-11-10.md
---

# Jenson, 36 Points

## Key facts for us

- Each particle senses the trail value s where it stands and sets its own
  sensor distance, sensor angle, turn angle and step from it, each as
  base + amplitude x s^exponent. Thin wanderers in sparse areas and tight
  bundles in dense ones at once.
- Particles respawn at random after a while.
- Parameter table readable in `Bleuje/interactive-physarum`
  (`points_basematrix.h`); see [bleuje-interactive-physarum](bleuje-interactive-physarum.md).
- Primordium (2022, Feral File): "The environment acts as both an arena and a
  message board for the agents." No technical detail.
