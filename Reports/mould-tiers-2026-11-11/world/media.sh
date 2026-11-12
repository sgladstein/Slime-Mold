#!/usr/bin/env bash
# The page's world videos (seed 1), same settings as run.sh, style=life
# (amber: ordinary walkers, violet: dormant, white: spores, red: alarm).
# Frames every 24 ticks (0.2 s), played at 20 frames a second: four times
# real time. From the repo root after cargo build --release --example bigdish:
#   bash Reports/mould-tiers-2026-11-11/world/media.sh [WORK] [PICS]
set -u
WORK=${1:-/tmp/tiers-world-media}; PICS=${2:-Reports/mould-tiers-2026-11-11/pics}
mkdir -p "$WORK" "$PICS"
BIN=./target/release/examples/bigdish
W="preset=today scale=2 substeps=2 scent_offset=36 scent_coarse=0 piles=23 rect=1 crystal_charge=512 crystal_eat=0.005 walkers=5000 capacity=50000 upkeep=0.001 crystal_regrow=0"
B="$W report=5 lifeline=1 threads=1 strike_r=8 seeds=1 style=life every=24"
FIRE="strike=10,10.5,11,11.5,12,12.5,13,13.5,14,14.5,15 strike_pile=0"
LIST="w-plain|$B seconds=60
w-sporer|$B seconds=60 tier=sporer
w-sleeper|$B seconds=30 tier=sleeper strike=14 strike_pile=0
w-bait|$B seconds=30 tier=sleeper bait=10 bait_pile=0 strike=14 strike_pile=0
w-swarm|$B seconds=30 tier=swarmer $FIRE
w-fire|$B seconds=30 alarm_keep=0.98 $FIRE"
ONLY=${ONLY:-}
n=0
while IFS='|' read -r name args; do
  [[ -n "$ONLY" && " $ONLY " != *" $name "* ]] && continue
  rm -rf "$WORK/$name"; mkdir -p "$WORK/$name"
  ( $BIN $args tag=$name frames="$WORK/$name" > "$WORK/$name.log" 2>&1 < /dev/null
    ffmpeg -nostdin -y -loglevel error -framerate 20 -i "$WORK/$name/$name-s1-%04d.png" -vf "scale=768:480:flags=neighbor" -c:v libx264 -pix_fmt yuv420p -crf 28 -preset slow "$PICS/$name.mp4" < /dev/null
    ffmpeg -nostdin -y -loglevel error -sseof -0.05 -i "$PICS/$name.mp4" -frames:v 1 -q:v 4 "$PICS/$name-poster.jpg" < /dev/null ) &
  n=$((n+1)); (( n % 2 == 0 )) && wait
done <<< "$LIST"
wait
