#!/bin/sh
# The page's videos (2026-11-11): seed 1, the census's fed and starved
# dishes, 30 s, drawn grey from the trail, a frame every 8 ticks (7.5 a
# second of dish time, played at 15: twice speed), and stills at 1, 6, 12
# and 30 s. From the repo root after building the bigdish example.
set -e
B=${B:-target/release/examples/bigdish}
F=${F:-/tmp/growth-frames}
D=Reports/growth-2026-11-11
G="preset=nose walkers=10000 capacity=100000 upkeep=0.01 graze_weight=10 seconds=30 report=1 threads=2 style=grey every=8"
mkdir -p "$D/video" "$D/raw"
for arm in fed starved; do
  r=0.002; [ $arm = starved ] && r=0
  rm -rf "$F/$arm"; mkdir -p "$F/$arm"
  $B $G graze_regrow=$r tag=$arm frames="$F/$arm" look="$F/$arm-still" at=1,6,12,30 > "$D/raw/video-$arm.txt" 2>&1 &
done
wait
for arm in fed starved; do
  ffmpeg -v error -y -framerate 15 -pattern_type glob -i "$F/$arm/*.png" -vf scale=768:-2 -c:v libx264 -pix_fmt yuv420p -crf 28 -preset slow -movflags +faststart "$D/video/$arm.mp4"
  for t in 001 006 012 030; do
    python3 -I -c "import glob; from PIL import Image; Image.open(glob.glob('$F/$arm-still/*-s1-${t}s*.png')[0]).convert('RGB').resize((768,480)).save('$D/video/$arm-${t}s.jpg', quality=88)"
  done
done
