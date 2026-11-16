#!/usr/bin/env python3
"""On-screen judder of a film's first members (film.rs writes drawn.csv:
frame, member, world x/y, screen x/y). For each member, its screen step
frame to frame; judder is the mean change of that step from the frame
before, in screen pixels (0 for a figure gliding at a steady pace, large
for one that sits still three frames and jumps on the fourth). Also the
share of frames a member's figure stood still on screen (under 0.05 px)
while the next frame moved it more than 1 px: a hitch.

python3 judder.py DIR [DIR ...]"""
import csv
import sys
from collections import defaultdict


def judder(path):
    tracks = defaultdict(list)
    with open(path) as f:
        for row in csv.DictReader(f):
            tracks[int(row["k"])].append((int(row["f"]), float(row["sx"]), float(row["sy"])))
    total, n, hitch, hn = 0.0, 0, 0, 0
    for pts in tracks.values():
        pts.sort()
        steps = [(b[1] - a[1], b[2] - a[2]) for a, b in zip(pts, pts[1:]) if b[0] == a[0] + 1]
        for s0, s1 in zip(steps, steps[1:]):
            total += ((s1[0] - s0[0]) ** 2 + (s1[1] - s0[1]) ** 2) ** 0.5
            n += 1
            l0 = (s0[0] ** 2 + s0[1] ** 2) ** 0.5
            l1 = (s1[0] ** 2 + s1[1] ** 2) ** 0.5
            hitch += l0 < 0.05 and l1 > 1.0
            hn += 1
    return total / max(n, 1), hitch / max(hn, 1)


for d in sys.argv[1:]:
    j, h = judder(f"{d}/drawn.csv")
    print(f"{d}: judder {j:.2f} px a frame, hitches {100 * h:.1f}% of frames")
