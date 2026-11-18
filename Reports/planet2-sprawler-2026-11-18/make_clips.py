#!/usr/bin/env python3
"""Turns film.sh's whole-map frames into the page's H.264 clips.

usage: make_clips.py FILMS_DIR PAGE_DIR
Each clip is a film's whole-map frames (`-all.png`, one every 2 game
seconds from 0:02 to 2:30) in time order at 10 frames a second, so 20
times real speed, scaled to 960 x 600.
"""
import os, re, subprocess, sys, tempfile

CLIPS = [
    ("seed1-worn-down.mp4", "clip1"),
    ("seed2-builds-up.mp4", "clip2"),
]

def frames(folder):
    out = []
    for f in os.listdir(folder):
        m = re.search(r"-(\d+)s-all\.png$", f)
        if m:
            out.append((int(m.group(1)), os.path.join(folder, f)))
    return sorted(out)

def main(src, page):
    for name, film in CLIPS:
        fs = [p for _, p in frames(os.path.join(src, film))]
        with tempfile.TemporaryDirectory() as d:
            for i, p in enumerate(fs):
                os.symlink(os.path.abspath(p), os.path.join(d, f"{i:04d}.png"))
            subprocess.run(["ffmpeg", "-y", "-loglevel", "error", "-framerate", "10", "-i", os.path.join(d, "%04d.png"),
                            "-vf", "scale=960:600:flags=lanczos", "-c:v", "libx264", "-pix_fmt", "yuv420p", "-crf", "28",
                            "-preset", "slow", "-movflags", "+faststart", os.path.join(page, name)], check=True)
        print(name, len(fs), "frames", os.path.getsize(os.path.join(page, name)) // 1024, "KB")

if __name__ == "__main__":
    main(sys.argv[1], sys.argv[2])
