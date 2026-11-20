#!/usr/bin/env python3
"""Joins a film's near-view frames into an H.264 clip.

usage: clip.py FRAMES_DIR OUT.mp4 T0 T1 FPS
Frames whose game time falls in [T0, T1], in time order, scaled to 960 wide.
"""
import os, re, subprocess, sys, tempfile

def frames(folder):
    out = []
    for f in os.listdir(folder):
        m = re.search(r"-(\d+)(ms|s)-near\.png$", f)
        if m:
            out.append((int(m.group(1)) / (1000 if m.group(2) == "ms" else 1), os.path.join(folder, f)))
    return sorted(out)

src, dst, t0, t1, fps = sys.argv[1], sys.argv[2], float(sys.argv[3]), float(sys.argv[4]), sys.argv[5]
fs = [p for t, p in frames(src) if t0 <= t <= t1]
with tempfile.TemporaryDirectory() as d:
    for i, p in enumerate(fs):
        os.symlink(os.path.abspath(p), os.path.join(d, "%04d.png" % i))
    subprocess.run(["ffmpeg", "-y", "-loglevel", "error", "-framerate", fps, "-i", os.path.join(d, "%04d.png"),
                    "-vf", "scale=960:-2", "-c:v", "libx264", "-pix_fmt", "yuv420p", "-crf", "26", dst], check=True)
print(dst, len(fs), "frames")
