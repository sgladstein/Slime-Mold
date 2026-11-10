---
id: nicoptere-physarum
title: physarum (three.js)
authors: Nicolas Barradeau
year: not seen
url: https://github.com/nicoptere/physarum
read: src/glsl shaders and index.js (2026-11-10)
copy: none
licence: public domain (Unlicense)
used-in: ../../Reports/jenson-compare-2026-11-10.md
---

# Barradeau, physarum in three.js

WebGL ping-pong textures: particles in a 512 x 512 texture (up to 262,144),
sensor angle, turn, sensor offset and step by slider (defaults sa 2, ra 4,
so 12, ss 1.1 in its own units), trail x 0.9 a step after a 3x3 blur, wraps.
The Jones one-a-cell check is written and commented out ("move only if the
destination is free"), like every other GPU port read on 2026-11-10.
