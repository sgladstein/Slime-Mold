"""Clips and counts for the seek page (gun-system review, step C).

python3 make_clips.py FILMS_DIR PAGE_DIR [--keep]

--keep reuses MP4s already in PAGE_DIR.

FILMS_DIR holds one folder a clip made by films.sh (`larder-world look=DIR
... rounds=plain load=29,CARDS at=29,29.033,...`). Each clip's near-view
frames (`*-near.png`, by their game time) become an H.264 MP4 at 30 frames
a second; its first and last count lines (the game's shots and kills; the
rounds line's seeker turns, soldier hits and the dish cells they fell in;
a mini-boss's members) are printed as JSON for the page. A folder whose
name ends in `-10s` has no frames: only its counts, over 10 s.
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
    rounds = re.findall(r"rounds t +(\d+) s: .*?, (\d+) seeker turns, (\d+) soldier hits in (\d+) cells", log)
    boss = re.findall(r"boss t ([\d.]+) s: lump 0 ([A-Za-z]+)[^,]*, (\d+) members", log)
    if not game:
        return {}
    (m0, s0, k0), (m1, s1, k1) = game[0], game[-1]
    # The squad's health after the last count line only (a squad wiped out
    # prints none there).
    last = log[log.rfind(f"s: mould {m1},"):]
    health = re.findall(r"health (\d+) mean", last)
    soldiers = re.findall(r"s: mould \d+, soldiers (\d+)", log)
    out = {"mould_at_end": int(m1), "soldiers_at_end": int(soldiers[-1]), "shots": int(s1) - int(s0), "killed": int(k1) - int(k0), "health_at_end": int(health[-1]) if health else None}
    if len(rounds) >= 2:
        r0, r1 = rounds[0], rounds[-1]
        out.update(seek_turns=int(r1[1]) - int(r0[1]), hits=int(r1[2]) - int(r0[2]), hit_cells=int(r1[3]) - int(r0[3]))
    if boss:
        out.update(boss_members=[int(boss[0][2]), int(boss[-1][2])], boss_end=boss[-1][1], boss_to=float(boss[-1][0]))
    return out


out = {}
for name in sorted(os.listdir(films)):
    d = os.path.join(films, name)
    log = os.path.join(d, "log.txt")
    if not os.path.isdir(d) or not os.path.exists(log):
        continue
    entry = counts(open(log).read())
    frames = sorted((f for f in os.listdir(d) if f.endswith("-near.png")), key=secs)
    mp4 = os.path.join(page, f"{name}.mp4")
    if len(frames) >= 10 and os.path.exists(mp4) and "--keep" in sys.argv:
        entry.update(frames=len(frames), kb=os.path.getsize(mp4) // 1024)
    elif len(frames) >= 10:
        with tempfile.TemporaryDirectory() as t:
            for i, f in enumerate(frames):
                os.symlink(os.path.join(d, f), os.path.join(t, f"f{i:03d}.png"))
            subprocess.run(["ffmpeg", "-y", "-loglevel", "error", "-framerate", "30", "-i", os.path.join(t, "f%03d.png"),
                            "-c:v", "libx264", "-pix_fmt", "yuv420p", "-crf", "22", "-preset", "slow", "-movflags", "+faststart", mp4], check=True)
        entry.update(frames=len(frames), kb=os.path.getsize(mp4) // 1024)
    out[name] = entry
print(json.dumps(out, indent=1))
