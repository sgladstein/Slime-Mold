---
id: bleuje-interactive-physarum
title: interactive-physarum
authors: Bleuje
year: 2024 (screenshots dated May and Nov 2024)
url: https://github.com/Bleuje/interactive-physarum
read: shaders and settings in full (move, deposit, diffusion, ofApp.h, points_basematrix.h), 2026-11-10
copy: none (do not copy code)
licence: CC BY-NC-SA 3.0, incompatible with our proprietary code: read for ideas only
used-in: ../../Reports/jenson-compare-2026-11-10.md, ../../Reports/mould-game-2026-11-08/depth-2026-11-09/sources-research.md, ../../Reports/jenson-physarum-2026-11-10.md, ../../Reports/gpu-plan-2026-11-10.md (phase 3 reading)
---

# Bleuje, interactive-physarum

openFrameworks installation piece, compute shaders only, gamepad-driven.
The readable version of Jenson's 36 Points: 17 of its parameter sets,
blended across space under a pen; particle inertia; spawning; waves; 9
colour modes; deposit from per-pixel particle counts (atomic counters).

## Key facts for us (code, 2026-11-10)

- 1280 x 736, 512 x 512 x 22 = 5.8 million particles (6 a pixel), no
  one-a-cell limit, wraps; particles packed as 16-bit position, heading and
  a respawn clock.
- Move: sense the trail at the particle (offset by two biases), scale and
  clamp to 0-1 as s; sensor distance, sensor angle, turn and step are each
  base + amplitude x s^exponent ("Technique/formulas from Sage Jenson").
  Then the classic three-sensor turn and an atomicAdd of 1 on the new pixel.
- Respawn: each particle jumps to a random spot every 1,000 steps (clock
  +0.001 a step), credited to Jenson.
- Deposit: 0.003 x sqrt(min(count, 100)) per pixel. Diffusion: 3x3 mean,
  then x 0.75.
- Drawn from the particle count a pixel through
  pow(tanh(7.5 x ((n - 1) / 1000)^0.3), 8.5), with 9 colour gradients; the
  trail is not what is shown.

**Open:** nothing for phase 3 beyond the layout above.
