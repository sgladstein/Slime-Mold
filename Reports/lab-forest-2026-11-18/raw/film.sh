#!/bin/bash
# film.sh OUTDIR NAME "ENVS" START SECS FPS args...  : frames every 1/FPS game s from START for SECS, then NAME.mp4 at PLAY fps (env PLAY, default 30) from the near view.
out=$1; name=$2; envs=$3; start=$4; secs=$5; fps=$6; shift 6
at=$(python3 -c "print(','.join('%.4f' % ($start + i / $fps) for i in range(int($secs * $fps) + 1)))")
/tmp/claude-0/-home-claude/5c0635d8-7d2e-5925-b45c-f915531a682c/scratchpad/run.sh "$out" "$name" "$envs" "$@" at="$at"
python3 - "$out/$name" "${PLAY:-30}" "${VIEW:-near}" <<'PY'
import os, re, sys, subprocess, tempfile
d, play, view = sys.argv[1], sys.argv[2], sys.argv[3]
def secs(n):
    m = re.search(r"-(\d+)(ms|s)-%s\.png$" % view, n)
    return int(m.group(1)) / (1000 if m.group(2) == "ms" else 1)
fs = sorted([f for f in os.listdir(d) if f.endswith("-%s.png" % view)], key=secs)
with tempfile.TemporaryDirectory() as t:
    for i, f in enumerate(fs):
        os.symlink(os.path.join(d, f), os.path.join(t, "%05d.png" % i))
    subprocess.run(["ffmpeg", "-y", "-loglevel", "error", "-framerate", play, "-i", os.path.join(t, "%05d.png"), "-c:v", "libx264", "-pix_fmt", "yuv420p", "-crf", "23", "-preset", "slow", "-movflags", "+faststart", d + "-" + view + ".mp4"], check=True)
print(d + "-" + view + ".mp4", len(fs), "frames")
PY
