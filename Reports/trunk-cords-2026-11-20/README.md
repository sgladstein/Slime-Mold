# The Trunk's cords (Look B, 2026-11-20)

Planet 3's slime, the Trunk, gets its own look (`SlimeLook::Cords`), the
third after the Drifter's sheet and the Sprawler's threads (Scott, 11-19:
several looks, one per slime).

The Trunk keeps 0.95 of its trail, so the trail runs as long straight cords
between a few big bodies. Plain, those cords were a few loose specks on bare
ground. Off the body (faded out by the wide count, `smoothstep(0.25, 0.55,
wide)`), the trail now stands up as a rounded translucent tube:
`CORD_VEIL` 0.3 of the colour's thickness and `CORD_RISE` 0.5 of height by
trail squared (`view.wgsl`, `prep`). The body keeps its grain.

Page: https://claude.ai/artifact/Xi8hjRyFbnbnWgsxsx9f7c (`index.html` here,
with the clips and the map still).

Film: `larder-world look=DIR run=1:3 mode=squads hand=strat:econ,go=4,aim=6
bank=20000 at=40,40.5,...,50 centre=420,250,6` (and `centre=400,240,12`);
the before with `LARDER_VIEW_WGSL` set to main's `view.wgsl`. Pairs joined
side by side, 4 frames a second.

Tried and dropped (filmed at 45 s):
- Veil 0.55: the cords read as solid slime, the body's own amber.
- Veil and height over the whole body: the body smoothed into a glossy sheet.
- Edges sharpened with `smoothstep(0.2, 0.7, trail)`: the cords nearly
  vanished, most of the Trunk's trail being under 0.5.
- Height 1.0: hardly differs from 0.5.

At zoom 12 the cords are soft: the trail is on the dish's coarse grid.
