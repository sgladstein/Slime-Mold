#!/bin/bash
# film.sh OUTDIR NAME "ENVS" START SECS FPS args...
# Frames every 1/FPS game seconds from START for SECS (run.sh), then
# OUTDIR/NAME.mp4 at PLAY frames a second (default 30) from the near view,
# 960 x 600. Run from the repo root.
#
# The page's clips (planet 1, run seed 1):
#   B="run=1:1 mode=squads bank=20000"
#   K="turret2=275,75,20 turret2=292,106,20 turret2=258,106,20 watch=275,95,30 hand=hold@325,80"
#   film.sh OUT p1-lure  "LAB_MARKS=lure:30:275:95:60" 26 64 15 $B $K far_smell=-30 centre=255,85,6
#   film.sh OUT p1-off   "LAB_X=0"                     26 64 15 $B $K far_smell=-30 centre=255,85,6
#   film.sh OUT p1-w10   "LAB_MARKS=lure:30:275:95:60" 26 64 15 $B $K far_smell=-10 centre=255,85,6
#   film.sh OUT p1-close "LAB_MARKS=lure:30:275:95:60" 36 15 30 $B $K far_smell=-30 centre=272,92,12
#   film.sh OUT p1-fence "LAB_MARKS=lure:30:282:77:60" 26 64 15 $B hand=hold@328,80 \
#       wall=248,15,248,68,20 wall=248,86,248,135,20 turret2=266,58,20 turret2=266,96,20 \
#       turret2=296,77,20 watch=282,77,30 far_smell=-30 centre=262,80,6
#   film.sh OUT p1-push  "LAB_MARKS=push:30:210:47:40 LAB_PUSH=40" 26 40 15 $B $K far_smell=-30 centre=225,70,6
here=$(dirname "$0")
out=$1; name=$2; envs=$3; start=$4; secs=$5; fps=$6; shift 6
at=$(python3 -c "print(','.join('%.4f' % ($start + i / $fps) for i in range(int($secs * $fps) + 1)))")
"$here/run.sh" "$out" "$name" "$envs" "$@" at="$at"
python3 - "$out/$name" "${PLAY:-30}" <<'PY'
import os, re, sys, subprocess, tempfile
d, play = sys.argv[1], sys.argv[2]
def secs(n):
    m = re.search(r"-(\d+)(ms|s)-near\.png$", n)
    return int(m.group(1)) / (1000 if m.group(2) == "ms" else 1)
fs = sorted([f for f in os.listdir(d) if f.endswith("-near.png")], key=secs)
with tempfile.TemporaryDirectory() as t:
    for i, f in enumerate(fs):
        os.symlink(os.path.join(d, f), os.path.join(t, "%05d.png" % i))
    subprocess.run(["ffmpeg", "-y", "-loglevel", "error", "-framerate", play, "-i", os.path.join(t, "%05d.png"), "-vf", "scale=960:600:flags=lanczos", "-c:v", "libx264", "-pix_fmt", "yuv420p", "-crf", "28", "-preset", "slow", "-movflags", "+faststart", d + ".mp4"], check=True)
print(d + ".mp4", len(fs), "frames")
PY
