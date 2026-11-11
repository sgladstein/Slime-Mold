#!/bin/sh
# The looks presets (keys 9, 0 and Shift+1-5) for the sandbox page
# (2026-11-11). From the repo root: cargo build --release --locked --bin
# mouldgpu, then this. As ../run.sh: seed 1, 10 s, a PNG every 4 frames
# (15 a second) for the video, the measures, the cost of each pass, and
# whether the same seed gives the same frame twice; with contagion on, the
# share sick every half second. On the cloud this runs on llvmpipe (a CPU
# emulating a GPU), so its costs are not GPU numbers.
set -e
BIN=${BIN:-target/release/mouldgpu}
F=${F:-/tmp/mouldgpu-frames}
D=Reports/gpu-sandbox-2026-11-11/looks
mkdir -p "$F" "$D/raw" "$D/video"
for p in ${PRESETS:-9 10 11 12 13 14 15}; do
  "$BIN" preset=$p look="$F" tag=p$p seconds=10 frames=1 every=4 twice=1 > "$D/raw/p$p.txt" 2>&1
  ffmpeg -v error -y -framerate 15 -i "$F/p$p/%05d.png" -c:v libx264 -pix_fmt yuv420p -crf 28 -preset slow -movflags +faststart "$D/video/p$p.mp4"
  # The still: 10 s, or for the contagion keys (10, 14, 15) 6 s, while
  # the wave is on the dish (frame k is 4k+1 frames, (4k+1)/60 s, in).
  case $p in
    10|14|15) src="$F/p$p/00090.png"; at=6s ;;
    *) src="$F/p$p-010s.png"; at=10s ;;
  esac
  python3 -I -c "from PIL import Image; Image.open('$src').convert('RGB').save('$D/video/p$p-$at.jpg', quality=88)"
done
