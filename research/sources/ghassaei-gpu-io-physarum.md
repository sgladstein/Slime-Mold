---
id: ghassaei-gpu-io-physarum
title: gpu-io physarum example
authors: Amanda Ghassaei
url: https://apps.amandaghassaei.com/gpu-io/examples/physarum/
read: examples/physarum/index.js in full (2026-11-10, by git clone; the web fetch had been refused)
copy: none
licence: MIT (code may be reused with its notice)
used-in: ../../Reports/jenson-compare-2026-11-10.md, ../../Reports/jenson-physarum-2026-11-10.md
---

# Ghassaei, gpu-io physarum

Browser demo on WebGL fragment shaders; touch adds attractant; presets.
Credits Jones 2010 and Jenson.

## Key facts for us (code, 2026-11-10)

- 0.35 particles a pixel; no one-a-cell limit; wraps.
- Presets keep 0.9 and deposit 4; "Fibers": sensors 18 px out at 5.5
  degrees, turn 45, step 2; others sensor angle 70-90 with sensors
  7.5-18 out.
- The random turn direction is one uniform for the whole frame, not per
  particle. Drawn as trail x 0.03.
