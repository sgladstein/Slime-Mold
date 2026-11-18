#!/usr/bin/env python3
"""Turns film.sh's near-view frames into the page's H.264 clips.

usage: make_clips.py FRAMES_DIR PAGE_DIR
Each clip is the frames whose game time falls in [t0, t1], in time order,
at `fps` frames a second, scaled to 960 x 600.
"""
import os, re, subprocess, sys, tempfile

CLIPS = [
    # name, film folder, t0, t1, frames a second
    ("walls-zoom6.mp4", "w6", 20, 75, 8),      # a frame a game second: 8x
    ("fire-zoom6.mp4", "w6", 76, 100, 8),      # two frames a game second: 4x
    ("fight-zoom12.mp4", "w12", 60, 66, 10),   # ten frames a game second: real time
    ("fire-zoom12.mp4", "w12", 76, 90, 8),     # four frames a game second: 2x
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
                            "-vf", "scale=960:600:flags=lanczos", "-c:v", "libx264", "-pix_fmt", "yuv420p", "-crf", "27",
                            "-preset", "slow", "-movflags", "+faststart", os.path.join(page, name)], check=True)
        print(name, len(fs), "frames", os.path.getsize(os.path.join(page, name)) // 1024, "KB")

if __name__ == "__main__":
    main(sys.argv[1], sys.argv[2])
