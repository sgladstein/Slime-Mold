#!/usr/bin/env python3
"""Turns film.sh's close frames into the page's H.264 clips.

usage: make_clips.py FILMS_DIR PAGE_DIR
Each clip is a film's close frames (`-near.png`, at the film's `centre=`
and zoom) in time order at 10 frames a second, scaled to 960 x 600: the
raid's every half game second from 0:20 to 0:56 (5 times real speed), the
tether's every quarter second from 0:02 to 0:20 (2.5 times).
"""
import os, re, subprocess, sys, tempfile

CLIPS = ["stream6", "march6", "stream12", "march12", "tether0", "tether2"]

def frames(folder):
    out = []
    for f in os.listdir(folder):
        m = re.search(r"-(\d+)(s|ms)-near\.png$", f)
        if m:
            t = int(m.group(1)) / (1000 if m.group(2) == "ms" else 1)
            out.append((t, os.path.join(folder, f)))
    return sorted(out)

def main(src, page):
    for film in CLIPS:
        fs = [p for _, p in frames(os.path.join(src, film))]
        name = film + ".mp4"
        with tempfile.TemporaryDirectory() as d:
            for i, p in enumerate(fs):
                os.symlink(os.path.abspath(p), os.path.join(d, f"{i:04d}.png"))
            subprocess.run(["ffmpeg", "-y", "-loglevel", "error", "-framerate", "10", "-i", os.path.join(d, "%04d.png"),
                            "-vf", "scale=960:600:flags=lanczos", "-c:v", "libx264", "-pix_fmt", "yuv420p", "-crf", "26",
                            "-preset", "slow", "-movflags", "+faststart", os.path.join(page, name)], check=True)
        print(name, len(fs), "frames", os.path.getsize(os.path.join(page, name)) // 1024, "KB")

if __name__ == "__main__":
    main(sys.argv[1], sys.argv[2])
