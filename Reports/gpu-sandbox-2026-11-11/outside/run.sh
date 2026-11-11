#!/bin/sh
# The outside rules presets (Shift+6 to Shift+0, presets 16-20) and the
# creatures report's (Ctrl+1 and Ctrl+2, presets 21-22) for the sandbox
# page (2026-11-11). From the repo root: cargo build --release
# --locked --bin mouldgpu, then this. As ../looks/run.sh: seed 1, a PNG
# every 4 frames (15 a second) for the video, the measures, the cost of each
# pass, and whether the same seed gives the same frame twice. Territories
# and the grazing species run 20 s (their reports' time), the explorer
# front 6 s (best at about 2 s), the rest 10 s. On the cloud this runs on llvmpipe (a CPU
# emulating a GPU), so its costs are not GPU numbers.
set -e
BIN=${BIN:-target/release/mouldgpu}
F=${F:-/tmp/mouldgpu-frames}
D=Reports/gpu-sandbox-2026-11-11/outside
mkdir -p "$F" "$D/raw" "$D/video"
for p in ${PRESETS:-16 17 18 19 20 21 22}; do
  case $p in
    16|22) sec=20 ;;
    18) sec=6 ;;
    *) sec=10 ;;
  esac
  "$BIN" preset=$p look="$F" tag=p$p seconds=$sec frames=1 every=4 twice=1 > "$D/raw/p$p.txt" 2>&1
  ffmpeg -v error -y -framerate 15 -i "$F/p$p/%05d.png" -c:v libx264 -pix_fmt yuv420p -crf 28 -preset slow -movflags +faststart "$D/video/p$p.mp4"
  # The still at the end, and for the explorer front also at 2 s (frame
  # k is 4k+1 frames, (4k+1)/60 s, in).
  still=$(printf '%s/p%s-%03ds.png' "$F" "$p" "$sec")
  python3 -I -c "from PIL import Image; Image.open('$still').convert('RGB').save('$D/video/p$p-${sec}s.jpg', quality=88)"
  if [ $p = 18 ]; then
    python3 -I -c "from PIL import Image; Image.open('$F/p$p/00030.png').convert('RGB').save('$D/video/p$p-2s.jpg', quality=88)"
  fi
done
