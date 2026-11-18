#!/usr/bin/env python3
"""Turns film.sh's near-view frames into the page's H.264 clips.

usage: make_clips.py FRAMES_DIR PAGE_DIR
Each clip is the frames whose game time falls in [t0, t1], in time order,
at `fps` frames a second, scaled to 800 x 500 (small on purpose: the
repo's clone is over a gigabyte).
"""
import os, re, subprocess, sys, tempfile

CLIPS = [
    # name, film folder, t0, t1, frames a second (the films hold ten frames a game second)
    ("no-lure-zoom6.mp4", "P-off-z6", 26, 90, 30),   # 3x
    ("lure-zoom6.mp4", "P-bld-z6", 26, 90, 30),      # 3x
    ("lure-zoom12.mp4", "P-mark-z12", 34, 56, 20),   # 2x
]

def frames(folder):
    out = []
    for f in os.listdir(folder):
        m = re.search(r"-(\d+)(ms|s)-near\.png$", f)
        if m:
            out.append((int(m.group(1)) / (1000 if m.group(2) == "ms" else 1), os.path.join(folder, f)))
    return sorted(out)

def main(src, page):
    for name, film, t0, t1, fps in CLIPS:
        fs = [p for t, p in frames(os.path.join(src, film)) if t0 <= t <= t1]
        with tempfile.TemporaryDirectory() as d:
            for i, p in enumerate(fs):
                os.symlink(os.path.abspath(p), os.path.join(d, f"{i:04d}.png"))
            subprocess.run(["ffmpeg", "-y", "-loglevel", "error", "-framerate", str(fps), "-i", os.path.join(d, "%04d.png"),
                            "-vf", "scale=800:500:flags=lanczos", "-c:v", "libx264", "-pix_fmt", "yuv420p", "-crf", "30",
                            "-preset", "slow", "-movflags", "+faststart", os.path.join(page, name)], check=True)
        print(name, len(fs), "frames", os.path.getsize(os.path.join(page, name)) // 1024, "KB")

if __name__ == "__main__":
    main(sys.argv[1], sys.argv[2])
