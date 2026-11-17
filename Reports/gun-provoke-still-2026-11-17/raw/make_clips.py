# make_clips.py FILMDIR PAGEDIR: the page's clips and counts from film.sh's
# frame folders (FILMDIR/SCENE-VARIANT-zZOOM). Each clip is the plain game on
# the left and the card's on the right, the near view's frames in time order
# at 8 a second (real speed), 1600 x 500, H.264. Writes PAGEDIR/SCENE-zZOOM.mp4
# and raw/counts.json beside this file: for each scene and variant, the
# counts between the clip's first and last frame from that game's log.
import glob
import json
import os
import re
import subprocess
import sys
import tempfile

film, page = sys.argv[1], sys.argv[2]
os.makedirs(page, exist_ok=True)
T_LINE = re.compile(r"^  t +([\d.]+) s: mould (\d+), soldiers (\d+), bank [\d.-]+, harvested [\d.]+, rendered \d+, shots (\d+), killed (\d+)", re.M)
SQUAD = re.compile(r"^  squad: touching ([\d.]+) mean, (\d+) most; health (\d+) mean", re.M)


def ms(path):
    m = re.search(r"-(\d+)(ms|s)-near\.png$", path)
    return int(m.group(1)) * (1 if m.group(2) == "ms" else 1000)


def frames(d):
    return sorted(glob.glob(os.path.join(film, d, "*-near.png")), key=ms)


def counts(d):
    t = open(os.path.join(film, d, "log.txt")).read()
    g, h = T_LINE.findall(t), SQUAD.findall(t)
    mind = re.findall(r"(\d+) Mind casts", t)
    return dict(
        t0=float(g[0][0]), t1=float(g[-1][0]),
        killed=int(g[-1][4]) - int(g[0][4]), shots=int(g[-1][3]) - int(g[0][3]),
        slime0=int(g[0][1]), slime1=int(g[-1][1]), soldiers=int(g[-1][2]),
        health=int(h[-1][2]), touched=max(int(x[1]) for x in h),
        touching=round(sum(float(x[0]) for x in h) / len(h), 2),
        mind=int(mind[-1]) - int(mind[0]) if mind else 0,
    )


out = {}
for scene, card in [("provoke", "provoke"), ("still", "still")]:
    for z in (6, 12):
        a, b = frames(f"{scene}-plain-z{z}"), frames(f"{scene}-{card}-z{z}")
        assert a and len(a) == len(b), (scene, z, len(a), len(b))
        with tempfile.TemporaryDirectory() as tmp:
            for side, fs in (("a", a), ("b", b)):
                for i, f in enumerate(fs):
                    os.symlink(os.path.abspath(f), os.path.join(tmp, f"{side}{i:03d}.png"))
            subprocess.run(
                ["ffmpeg", "-y", "-loglevel", "error", "-framerate", "8", "-i", os.path.join(tmp, "a%03d.png"), "-framerate", "8", "-i", os.path.join(tmp, "b%03d.png"),
                 "-filter_complex", "[0:v]scale=800:500[a];[1:v]scale=800:500[b];[a][b]hstack,fps=24", "-c:v", "libx264", "-pix_fmt", "yuv420p", "-crf", "24", "-movflags", "+faststart",
                 os.path.join(page, f"{scene}-z{z}.mp4")],
                check=True,
            )
    out[scene] = {"plain": counts(f"{scene}-plain-z6"), card: counts(f"{scene}-{card}-z6")}
json.dump(out, open(os.path.join(os.path.dirname(os.path.abspath(__file__)), "counts.json"), "w"), indent=1)
print(json.dumps(out, indent=1))
