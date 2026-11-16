"""Clips and counts for the Mind page (gun-system review step D, 2026-11-16).

python3 make_clips.py FILMS_DIR PAGE_DIR

FILMS_DIR holds one folder a clip made by film.sh (plain, blind, sour,
spin). Each clip's near-view frames (`*-near.png`, by their game time)
become an H.264 MP4 at 15 frames a second (real time). Its counts are
printed as JSON: the boom's kills, the walkers `watch=` counted within 8
cells of the hit and from 8 to 16 at each second, the slime's mass at the
first and last count line, and the rounds' Mind casts.

The blind page names the clips by QUIZ (a fixed shuffle): the MP4s are
written as quiz-1.mp4 .. quiz-4.mp4 and the key is printed, to go in the
README, never on the page.
"""
import json
import os
import re
import subprocess
import sys
import tempfile

films, page = sys.argv[1], sys.argv[2]
# The shuffle (drawn once with random.shuffle, then fixed here).
QUIZ = ["plain", "spin", "sour", "blind"]


def secs(name):
    m = re.search(r"-(\d+)(ms|s)-near\.png$", name)
    return int(m.group(1)) / (1000 if m.group(2) == "ms" else 1)


def counts(log):
    boom = re.search(r"boom t [\d.]+ s at [\d.,]+: (\d+) killed, (\d+) flung", log)
    watch = {int(t): (int(a), int(b)) for t, a, b in re.findall(r"watch t (\d+) s: (\d+) walkers within [\d.]+ of [\d.,]+, (\d+) from", log)}
    mould = re.findall(r"t \d+ s: mould (\d+),", log)
    minds = re.findall(r", (\d+) Mind casts", log)
    return {
        "boom_killed": int(boom.group(1)) if boom else None,
        "boom_flung": int(boom.group(2)) if boom else None,
        "within_8": {t: v[0] for t, v in sorted(watch.items())},
        "from_8_to_16": {t: v[1] for t, v in sorted(watch.items())},
        "mould_first": int(mould[0]) if mould else None,
        "mould_last": int(mould[-1]) if mould else None,
        "mind_casts": int(minds[-1]) if minds else None,
    }


out = {}
for name in QUIZ:
    d = os.path.join(films, name)
    entry = counts(open(os.path.join(d, "log.txt")).read())
    frames = sorted((f for f in os.listdir(d) if f.endswith("-near.png")), key=secs)
    mp4 = os.path.join(page, f"quiz-{QUIZ.index(name) + 1}.mp4")
    with tempfile.TemporaryDirectory() as t:
        for i, f in enumerate(frames):
            os.symlink(os.path.join(d, f), os.path.join(t, f"f{i:03d}.png"))
        subprocess.run(["ffmpeg", "-y", "-loglevel", "error", "-framerate", "15", "-i", os.path.join(t, "f%03d.png"),
                        "-c:v", "libx264", "-pix_fmt", "yuv420p", "-crf", "22", "-preset", "slow", "-movflags", "+faststart", mp4], check=True)
    entry.update(clip=os.path.basename(mp4), frames=len(frames), kb=os.path.getsize(mp4) // 1024)
    out[name] = entry
print(json.dumps(out, indent=1))
