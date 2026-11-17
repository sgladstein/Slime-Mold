"""Does the sick slime read as sick on screen? (the game designer, 11-17
08:22Z: Detonate's lesson is "detonate late, once the wave has spread", on
the condition that the player can see when to fire.)

python3 -I red_vs_sick.py FILMS_DIR CLIP...

For each clip made by film.sh (the near view at zoom 8, 8 pixels a cell,
centred on the watched point), each game second from 20 to 40: the sick
walkers within 24 cells (`watch=` in its log) beside the share of the
24-cell disc on screen drawn red (R > 180, G < 110, B < 90: the engine's
germ tint, `engine.wgsl`, "With contagion on, the germ in red").
"""
import glob
import re
import sys

import numpy as np
from PIL import Image

films, clips = sys.argv[1], sys.argv[2:]


def secs(f):
    m = re.search(r"-(\d+)(ms|s)-near\.png$", f)
    return int(m.group(1)) / (1000 if m.group(2) == "ms" else 1)


for clip in clips:
    frames = glob.glob(f"{films}/{clip}/*-near.png")
    log = open(f"{films}/{clip}/log.txt").read()
    sick = {int(t): int(s) for t, s in re.findall(r"watch t (\d+) s: .*?, (\d+) sick within 24", log)}
    print(clip)
    for t in range(20, 41):
        f = min(frames, key=lambda f: abs(secs(f) - t))
        a = np.asarray(Image.open(f).convert("RGB")).astype(int)
        h, w, _ = a.shape
        yy, xx = np.mgrid[0:h, 0:w]
        disc = (xx - w / 2) ** 2 + (yy - h / 2) ** 2 <= (24 * 8) ** 2
        red = (a[..., 0] > 180) & (a[..., 1] < 110) & (a[..., 2] < 90)
        print(f"  t {t:2d} s (frame {secs(f):.2f} s): sick within 24 {sick.get(t, 0):5d}, red share of the disc {100 * red[disc].mean():5.1f}%")
