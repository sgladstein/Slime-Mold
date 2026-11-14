#!/usr/bin/env python3
"""The River test map (new ground kinds, 2026-11-14): a deep river across the
middle, two shallow fords near the edges and a bridge in the middle; the squad
lands north, the mould starts south.  python3 make_river.py Reports/larder-maps-2026-11-11/maps"""
import math, sys
from pathlib import Path
from PIL import Image

W, H = 1024, 640
G_WET, G_DEEP, G_BRIDGE = 0x01, 0x41, 0x50
out = Path(sys.argv[1])
img = Image.new("RGBA", (W, H), (0, 0, 0, 255))
px = img.load()
for x in range(W):
    mid = 320 + 22 * math.sin(x / 140.0)  # a gentle meander
    for y in range(H):
        if abs(y - mid) < 28:
            g = G_DEEP
            if 170 <= x < 230 or 794 <= x < 854:
                g = G_WET  # fords
            elif 496 <= x < 528:
                g = G_BRIDGE
            px[x, y] = (0, g, 0, 255)
img.save(out / "river.png")
(out / "river.txt").write_text("""# River (new ground kinds, 2026-11-14): deep water across the middle,
# fords at x 170-229 and 794-853, a bridge at x 496-527.
landing 512 110
mould 512 560
crystal 512 590 34 384
crystal 300 180 26 384
crystal 724 180 26 384
crystal 300 470 26 384
crystal 724 470 26 384
crystal 90 220 22 256
crystal 934 430 22 256
""")
