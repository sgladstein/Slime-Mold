---
id: lague-slime-simulation
title: Slime Simulation (and the video "Coding Adventure: Ant and Slime Simulations")
authors: Sebastian Lague
year: 2021 (Unity 2020.3)
url: https://github.com/SebLague/Slime-Simulation
read: SlimeSim.compute, Simulation.cs and the four presets in full (2026-11-10); video not watched
copy: none (do not copy code)
licence: GPL-3.0, incompatible with our proprietary code: read for ideas only
used-in: ../../Reports/jenson-compare-2026-11-10.md, ../../Reports/jenson-physarum-2026-11-10.md, ../../Reports/gpu-plan-2026-11-10.md (phase 3 reading)
---

# Lague, Slime Simulation

A clear compute-shader walk-through of Jones, now read in full.

## Key facts for us (code, 2026-11-10)

- Preset A: 250,000 agents on 1920 x 1080, two steps a frame, fixed time
  step 0.02 s. No one-a-cell limit.
- Sensors sum a (2 x size + 1)^2 area (size 1: 3x3), 35 px out at 30
  degrees; the turn is a random share of turnSpeed x 2 pi x dt (about 14
  degrees) a step; step 20 x 0.02 = 0.4 px.
- Deposit min(1, trail + 5 x 0.02): the trail is capped at 1. Diffusion
  blends 3 x 0.02 = 6% a step toward the 3x3 mean; decay subtracts
  0.2 x 0.02 = 0.004 a step (linear, not multiplicative).
- Species: each senses with weight +1 on its own channel and -1 on the
  others, so they follow their own trail and avoid the rest (confirmed).
- At a wall: clamp and a random new heading.
