"""Clips and counts for the element page (gun-system review step D, the
element family, 2026-11-17; the readers' script with the fire's counts).

python3 make_clips.py FILMS_DIR PAGE_DIR NAME...

FILMS_DIR holds one folder a clip made by film.sh. Each named clip's
near-view frames (`*-near.png`, by their game time) become an H.264 MP4 at 15
frames a second (real time), the HUD cropped off as on the Mind page. Its
counts are printed as JSON: each boom's time, kills and gassed, the walkers
`watch=` counted within 12 cells of the point and from 12 to 24 at each
second, the cells burning within 24 and the walkers killed by fire so far
(on a world with plants), and the slime's mass at the last count line.
"""
import json
import os
import re
import subprocess
import sys
import tempfile

films, page, names = sys.argv[1], sys.argv[2], sys.argv[3:]


def secs(name):
    m = re.search(r"-(\d+)(ms|s)-near\.png$", name)
    return int(m.group(1)) / (1000 if m.group(2) == "ms" else 1)


def counts(log):
    booms = [{"t": float(t), "killed": int(k), "gassed": int(g)} for t, k, g in re.findall(r"boom t ([\d.]+) s at [\d.,]+: (\d+) killed, \d+ flung, (\d+) gassed", log)]
    watch = re.findall(r"watch t (\d+) s: (\d+) walkers within [\d.]+ of [\d.,]+, (\d+) from [\d.]+ to [\d.]+(?:, (\d+) sick within [\d.]+)?", log)
    fire = re.findall(r"watch t (\d+) s: .*, (\d+) cells burning within [\d.]+, (\d+) killed by fire", log)
    mould = re.findall(r"t \d+ s: mould (\d+),", log)
    out = {
        "booms": booms,
        "within_12": {int(t): int(a) for t, a, _, _ in watch},
        "from_12_to_24": {int(t): int(b) for t, _, b, _ in watch},
        "mould_last": int(mould[-1]) if mould else None,
    }
    if fire:
        out["burning_within_24"] = {int(t): int(b) for t, b, _ in fire}
        out["killed_by_fire"] = {int(t): int(k) for t, _, k in fire}
    if any(s for *_, s in watch):
        out["sick_within_24"] = {int(t): int(s) for t, _, _, s in watch if s}
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
        subprocess.run(["ffmpeg", "-y", "-loglevel", "error", "-framerate", "15", "-i", os.path.join(t, "f%03d.png"),
                        "-vf", "crop=960:456:0:40", "-c:v", "libx264", "-pix_fmt", "yuv420p", "-crf", "27", "-preset", "slow", "-movflags", "+faststart", mp4], check=True)
    entry.update(clip=os.path.basename(mp4), frames=len(frames), kb=os.path.getsize(mp4) // 1024)
    out[name] = entry
print(json.dumps(out, indent=1))
