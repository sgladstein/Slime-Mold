"""Contact sheets and loop counts for Jenson's physarum GIFs (2026-11-10).

Run: python3 -I gif_frames.py   (from this folder; needs Pillow and numpy)

Writes <name>_sheet.png (eight evenly spaced frames per GIF) and prints, for
network_2.gif, the number of enclosed loops at four frames: pixels brighter
than a cut-off count as tube, and dark 4-connected regions inside the central
disc (radius 0.45 x width) of 15 to 20,000 pixels count as loops. Two
cut-offs (110 and 150 of 255) give the range quoted in the report. It also
prints how much ef_9.gif changes between its first and last frame, centre
box against corners.
"""
import os
from collections import deque

import numpy as np
from PIL import Image, ImageSequence

HERE = os.path.dirname(os.path.abspath(__file__))


def frames(name):
    return [np.array(f.convert("L")) for f in ImageSequence.Iterator(Image.open(os.path.join(HERE, name)))]


def sheet(name, fr):
    n = len(fr)
    idx = [round(i * (n - 1) / 7) for i in range(8)]
    h, w = fr[0].shape
    s = min(1.0, 300 / w)
    tw, th = int(w * s), int(h * s)
    out = Image.new("L", (tw * 4, th * 2))
    for j, i in enumerate(idx):
        out.paste(Image.fromarray(fr[i]).resize((tw, th)), ((j % 4) * tw, (j // 4) * th))
    out.save(os.path.join(HERE, name.replace(".gif", "_sheet.png")))


def region_sizes(mask):
    h, w = mask.shape
    seen = np.zeros_like(mask, bool)
    sizes = []
    for y in range(h):
        for x in range(w):
            if mask[y, x] and not seen[y, x]:
                q = deque([(y, x)])
                seen[y, x] = True
                n = 0
                while q:
                    a, b = q.popleft()
                    n += 1
                    for da, db in ((1, 0), (-1, 0), (0, 1), (0, -1)):
                        c, d = a + da, b + db
                        if 0 <= c < h and 0 <= d < w and mask[c, d] and not seen[c, d]:
                            seen[c, d] = True
                            q.append((c, d))
                sizes.append(n)
    return sizes


for g in ("network_2.gif", "explosion.gif", "ef_9.gif"):
    sheet(g, frames(g))

net = frames("network_2.gif")
h, w = net[0].shape
yy, xx = np.mgrid[:h, :w]
disc = ((xx - w / 2) ** 2 + (yy - h / 2) ** 2) < (0.45 * w) ** 2
for i in (0, 37, 74, len(net) - 1):
    for cut in (110, 150):
        tube = net[i] > cut
        loops = sum(1 for v in region_sizes((~tube) & disc) if 15 <= v < 20000)
        print(f"network_2 frame {i} cut {cut}: loops {loops}")

ef = frames("ef_9.gif")
d = np.abs(ef[0].astype(float) - ef[-1].astype(float))
corners = np.concatenate([d[:120, :120].ravel(), d[-120:, -120:].ravel()])
print(f"ef_9 first->last mean change: centre {d[210:510, 210:510].mean():.1f}, corners {corners.mean():.1f}")
