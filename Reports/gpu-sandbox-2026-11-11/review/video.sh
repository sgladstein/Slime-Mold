#!/bin/sh
# The page's videos of keys 1, 2, 5 and 6 after the review's fixes
# (build 9d4a2d1, software Vulkan in the cloud), as ../run.sh drew them:
# seed 1, 10 s, a PNG every 4 frames, then 15 frames a second in H.264
# (crf 26; key 6, the walker counts, crf 33 to keep it under 1 MB).
set -e
BIN=${BIN:-target/release/mouldgpu}
F=${F:-/tmp/mouldgpu-frames}
V=Reports/gpu-sandbox-2026-11-11/video
mkdir -p "$F" Reports/gpu-sandbox-2026-11-11/review/video-raw
for p in 1 2 5 6; do
  "$BIN" preset=$p look="$F" tag=p$p seconds=10 frames=1 every=4 twice=1 > Reports/gpu-sandbox-2026-11-11/review/video-raw/p$p.txt 2>&1
  crf=26; [ $p = 6 ] && crf=33
  ffmpeg -v error -y -framerate 15 -i "$F/p$p/%05d.png" -c:v libx264 -pix_fmt yuv420p -crf $crf -preset slow -movflags +faststart "$V/p$p.mp4"
  python3 -I -c "from PIL import Image; Image.open('$F/p$p-010s.png').convert('RGB').save('$V/p$p-10s.jpg', quality=88)"
done
