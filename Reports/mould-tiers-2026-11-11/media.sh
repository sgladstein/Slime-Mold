#!/usr/bin/env bash
# The page's videos and stills (seed 1), from the repo root after
# cargo build --release --example bigdish:
#   bash Reports/mould-tiers-2026-11-11/media.sh [WORK] [PICS]
# Same settings as run.sh; style=life (amber: ordinary walkers, violet:
# dormant, white: spores, red: alarm, dim grey: trail). Frames every 12
# ticks (0.1 s), played at 20 frames a second: twice real time.
set -u
WORK=${1:-/tmp/mould-tiers-media}; PICS=${2:-Reports/mould-tiers-2026-11-11/pics}
mkdir -p "$WORK" "$PICS"
BIN=./target/release/examples/bigdish
B="preset=dense walkers=20000 blob=1 blob_r=40 report=1 threads=1 seconds=20 seeds=1 style=life every=12"
SP="spore_scent=1 spore_reach=36 spore_land=1 spore_deposit=0.2"
SL="sleep_rate=0.01 wake_scent=1 sleep_deposit=0.3 alarm_keep=0.98 alarm_relay=20 wake_alarm=1 alarm_weight=3 sleep_rest=600 strike_x=161 strike_r=8 strike=14"
FIRE="alarm_keep=0.98 strike=10,10.5,11,11.5,12,12.5,13,13.5,14,14.5,15 strike_x=95 strike_r=8"
LIST="sporer|$B $SP spore_rate=0.0001
plain|$B
burst|$B $SP spore_burst=0.05 spore_alarm=1 alarm_keep=0.98 strike=10 strike_x=95
struck|$B $SP alarm_keep=0.98 strike=10 strike_x=95
sleeper|$B $SL
bait|$B $SL bait=10 bait_x=95
swarm|$B $FIRE alarm_weight=3
fire|$B $FIRE
flee|$B $FIRE alarm_weight=-10"
# ONLY="bait struck" renders just those arms. ffmpeg runs with -nostdin and
# both tools read /dev/null: a background ffmpeg otherwise eats the list
# this loop reads (it did, 2026-11-11: the bait arm came out mangled).
ONLY=${ONLY:-}
n=0
while IFS='|' read -r name args; do
  [[ -n "$ONLY" && " $ONLY " != *" $name "* ]] && continue
  rm -rf "$WORK/$name"; mkdir -p "$WORK/$name"
  ( $BIN $args tag=$name frames="$WORK/$name" look="$WORK/$name" at=2,5,8,10,10.5,11,12,14,14.5,15,16,20 > "$WORK/$name.log" 2>&1 < /dev/null
    ffmpeg -nostdin -y -loglevel error -framerate 20 -i "$WORK/$name/$name-s1-%04d.png" -vf "scale=768:480" -c:v libx264 -pix_fmt yuv420p -crf 30 -preset slow "$PICS/$name.mp4" ) &
  n=$((n+1)); (( n % 4 == 0 )) && wait
done <<< "$LIST"
wait
