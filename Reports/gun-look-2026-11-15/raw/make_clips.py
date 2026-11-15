"""Clips for the gun look page: each film's frames (f000.png, f001.png,
...; `film.sh`) as an H.264 MP4 at 30 frames a second, 960 x 600.

python3 make_clips.py FILMS_DIR PAGE_DIR  (FILMS_DIR holds before/ and after/)
"""
import os
import re
import subprocess
import sys

films, page = sys.argv[1], sys.argv[2]
for side in ("before", "after"):
    for name in sorted(os.listdir(os.path.join(films, side))):
        d = os.path.join(films, side, name)
        if not os.path.isdir(d):
            continue
        n = len([f for f in os.listdir(d) if re.fullmatch(r"f\d{3}\.png", f)])
        if n < 10:
            continue
        out = os.path.join(page, f"{name}-{side}.mp4")
        subprocess.run(["ffmpeg", "-y", "-loglevel", "error", "-framerate", "30", "-i", os.path.join(d, "f%03d.png"),
                        "-c:v", "libx264", "-pix_fmt", "yuv420p", "-crf", "20", "-preset", "slow", "-movflags", "+faststart", out], check=True)
        print(out, n, "frames", os.path.getsize(out) // 1024, "KB")
