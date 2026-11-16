"""Clips and counts for the bullet-shape page (gun-system review, step A).

python3 make_clips.py FILMS_DIR PAGE_DIR

FILMS_DIR holds before/, after/ and paired/, each with one folder a clip
made by film.sh (`larder-world look=DIR ... at=30,30.033,...`). Each clip's near
view frames (`*-near.png`, by their game time) become an H.264 MP4 at 30
frames a second; its first and last count lines (game shots and kills, the
rounds' casts, kills and walkers thrown) are printed as JSON for the page.
"""
import json
import os
import re
import subprocess
import sys
import tempfile

films, page = sys.argv[1], sys.argv[2]


def secs(name):
    m = re.search(r"-(\d+)(ms|s)-near\.png$", name)
    return int(m.group(1)) / (1000 if m.group(2) == "ms" else 1)


def counts(log):
    game = re.findall(r"t \d+ s: mould (\d+), soldiers \d+, bank [\d.-]+, harvested [\d.]+, rendered \d+, shots (\d+), killed (\d+)", log)
    rounds = re.findall(r"rounds t +\d+ s: (\d+) casts .*?\[([\d, ]+)\]\), (\d+) walkers killed, (\d+) flung", log)
    if not game:
        return {}
    (m0, s0, k0), (m1, s1, k1) = game[0], game[-1]
    health = re.findall(r"health (\d+) mean", log)
    out = {
        "mould_at_end": int(m1),
        "shots_in_clip": int(s1) - int(s0),
        "killed_in_clip": int(k1) - int(k0),
        "shots_by_end": int(s1),
        "killed_by_end": int(k1),
        "health_at_end": int(health[-1]),
    }
    # The army's round's counts (none printed for an empty round).
    if rounds:
        r0, r1 = rounds[0], rounds[-1]
        out.update(thrown_in_clip=int(r1[3]) - int(r0[3]), thrown_by_end=int(r1[3]), casts_by_end=int(r1[0]))
    return out


out = {}
# The page shows every paired clip and the bomb before and after.
for side in ("before", "after", "paired"):
    root = os.path.join(films, side)
    if not os.path.isdir(root):
        continue
    for name in sorted(os.listdir(root)):
        d = os.path.join(root, name)
        if not os.path.isdir(d) or (side != "paired" and name != "bomb"):
            continue
        frames = sorted((f for f in os.listdir(d) if f.endswith("-near.png")), key=secs)
        if len(frames) < 10:
            continue
        with tempfile.TemporaryDirectory() as t:
            for i, f in enumerate(frames):
                os.symlink(os.path.join(d, f), os.path.join(t, f"f{i:03d}.png"))
            mp4 = os.path.join(page, f"{name}-{side}.mp4")
            subprocess.run(["ffmpeg", "-y", "-loglevel", "error", "-framerate", "30", "-i", os.path.join(t, "f%03d.png"),
                            "-c:v", "libx264", "-pix_fmt", "yuv420p", "-crf", "22", "-preset", "slow", "-movflags", "+faststart", mp4], check=True)
        out[f"{name}-{side}"] = dict(counts(open(os.path.join(d, "log.txt")).read()), frames=len(frames), kb=os.path.getsize(mp4) // 1024)
print(json.dumps(out, indent=1))
