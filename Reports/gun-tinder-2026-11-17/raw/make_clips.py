"""Clips and counts for the Tinder page (the gun cards' element family,
2026-11-17).

python3 make_clips.py FILMS_DIR PAGE_DIR FPS NAME...

FILMS_DIR holds one folder a clip made by film.sh (or play.sh). Each named
clip's near-view frames (`*-near.png`, by their game time) become an H.264
MP4 at FPS frames a second (real time), the HUD cropped off. Its counts are
printed as JSON: from `cross=` each second the walkers beyond the line, the
crossings so far, the walkers within 3 of the line, the cells burning on it
and the fire's kills; from `watch=` the walkers within 12 and the fire's
kills; and the slime's mass at each count line.
"""
import json
import os
import re
import subprocess
import sys
import tempfile

films, page, fps, names = sys.argv[1], sys.argv[2], sys.argv[3], sys.argv[4:]


def secs(name):
    m = re.search(r"-(\d+)(ms|s)-near\.png$", name)
    return int(m.group(1)) / (1000 if m.group(2) == "ms" else 1)


def counts(log):
    cross = re.findall(r"cross t (\d+) s: (\d+) walkers beyond, (\d+) crossed, (\d+) within 3 of the line(?:, (\d+) cells burning on the line, (\d+) killed by fire)?", log)
    watch = re.findall(r"watch t (\d+) s: (\d+) walkers within [\d.]+ of [\d.,]+, (\d+) from .*?(?:, (\d+) cells burning within [\d.]+, (\d+) killed by fire)?$", log, re.M)
    mould = re.findall(r"t +(\d+) s: mould (\d+),", log)
    rounds = re.findall(r"rounds t +([\d.]+) s: .*?(\d+) trail bursts", log)
    out = {"mould": {int(t): int(m) for t, m in mould}}
    if cross:
        out["beyond"] = {int(c[0]): int(c[1]) for c in cross}
        out["crossed"] = {int(c[0]): int(c[2]) for c in cross}
        out["on_line"] = {int(c[0]): int(c[3]) for c in cross}
        out["burning"] = {int(c[0]): int(c[4] or 0) for c in cross}
        out["killed_by_fire"] = {int(c[0]): int(c[5] or 0) for c in cross}
    if watch:
        out["within_12"] = {int(w[0]): int(w[1]) for w in watch}
        out["killed_by_fire"] = {int(w[0]): int(w[4] or 0) for w in watch}
    return out


out = {}
for name in names:
    d = os.path.join(films, name)
    entry = counts(open(os.path.join(d, "log.txt")).read())
    frames = sorted((f for f in os.listdir(d) if f.endswith("-near.png")), key=secs)
    mp4 = os.path.join(page, f"{name}.mp4")
    with tempfile.TemporaryDirectory() as t:
        for i, f in enumerate(frames):
            os.symlink(os.path.join(d, f), os.path.join(t, f"f{i:03d}.png"))
        subprocess.run(["ffmpeg", "-y", "-loglevel", "error", "-framerate", fps, "-i", os.path.join(t, "f%03d.png"),
                        "-vf", "crop=960:456:0:40", "-c:v", "libx264", "-pix_fmt", "yuv420p", "-crf", "27", "-preset", "slow", "-movflags", "+faststart", mp4], check=True)
    entry.update(clip=os.path.basename(mp4), frames=len(frames), kb=os.path.getsize(mp4) // 1024)
    out[name] = entry
print(json.dumps(out, indent=1))
