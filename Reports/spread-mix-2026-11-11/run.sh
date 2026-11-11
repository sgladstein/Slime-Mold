#!/usr/bin/env bash
# Spread from one spot, and mixed walkers in one mould (2026-11-11).
# From the repo root, after cargo build --release --example bigdish:
#   bash Reports/spread-mix-2026-11-11/run.sh GROUP [JOBS] [OUT]
# GROUP: spread-net (network types, 1M walkers), spread-drop (droplet
# types, 30k), mix-net, mix-drop. Each run starts from one spot in the dish
# centre and prints report lines every 2 s (tube area, reach, and with
# mixes the per-kind `kinds` line, and `extent`: median and 90th-percentile
# walker distance from the start); stills in style=soft or style=kinds.
# blob_r is in dish units (x scale 4 = fine cells): 20 is an 80-cell disc for
# the network; 50 (a 200-cell disc, 24% full) for the 30k droplet walkers,
# since packed one a cell (26) they jam and do not move at all.
set -u
G=$1; JOBS=${2:-4}; OUT=${3:-/tmp/spread-mix}; mkdir -p "$OUT"
BIN=./target/release/examples/bigdish
SEEDS=${SEEDS:-"1 2 3"}; SECS=${SECS:-20}
N="preset=dense piles=1 food_deposit=0 crowd=0 walkers=1000000 deposit=0.5 rotate_angle=22.5 sensor_offset=18 blob=1 blob_r=20"
D="preset=dense piles=1 food_deposit=0 crowd=1 walkers=30000 deposit=5 rotate_angle=45 sensor_angle=90 sensor_offset=9 keep=0.93 blob=1 blob_r=50 blocked_deposit=1"
case $G in
  spread-net) STYLE=soft; LIST="net|$N
chance|$N chance_exp=4
explore|$N explore_steps=900 explore_spread=1" ;;
  spread-drop) STYLE=soft; LIST="droplets|$D
grazers|$D graze_weight=10
beads|$D sensor_offset=27" ;;
  mix-net) STYLE=kinds; LIST="scouts|$N mix_share=0.1 mix_reach=3 mix_step=2 mix_deposit=0.3
builders|$N mix_share=0.3 mix_step=0.5 mix_deposit=3
wanderers|$N mix_share=0.2 mix_turn=4 mix_angle=2
twoscale|$N mix_share=0.5 mix_reach=3" ;;
  mix-drop) STYLE=kinds; LIST="dropscouts|$D mix_share=0.2 mix_reach=3 mix_deposit=0.3
dropfast|$D mix_share=0.2 mix_step=2 mix_deposit=0.3
grazescouts|$D graze_weight=10 mix_share=0.2 mix_reach=3 mix_deposit=0.3" ;;
  *) echo "unknown group $G"; exit 1 ;;
esac
n=0
for s in $SEEDS; do
  while IFS='|' read -r name args; do
    ( $BIN $args seeds=$s tag=$name report=2 threads=1 seconds=$SECS style=$STYLE look="$OUT/$name-s$s/" at=2,5,10,20 2>&1 | grep -E "^seed" ) > "$OUT/$name-s$s.txt" &
    n=$((n+1)); (( n % JOBS == 0 )) && wait
  done <<< "$LIST"
done
wait
for f in "$OUT"/*.txt; do cat "$f"; done
