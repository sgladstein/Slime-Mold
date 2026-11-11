#!/usr/bin/env bash
# The wall-rail check (2026-11-11): the creature review warned that the dish
# wall can act as a rail a rule leans on. The wall reads as a wall to sensors
# 9 dish cells out, so walkers leave a 2-cell band in 0.2 s; a rider would
# sit within 10. From the repo root, after cargo build --release --example
# bigdish:
#   bash Reports/mould-tiers-2026-11-11/wall.sh [JOBS] [OUT]
# Measure, named before the run (seeds 1-3, descriptive): `rim` at 20 and
# 60 s, the share of grounded living walkers within 2 / 10 dish cells of the
# agar edge, each tier against its control. Spores fly to the edge (a burst
# reaches it), so the struck Sporer is the arm most likely to ride it.
set -u
JOBS=${1:-4}; OUT=${2:-Reports/mould-tiers-2026-11-11/raw}; mkdir -p "$OUT"
BIN=./target/release/examples/bigdish
SEEDS=${SEEDS:-"1 2 3"}
B="preset=dense walkers=20000 blob=1 blob_r=40 report=10 threads=1 seconds=60"
SP="spore_scent=1 spore_reach=36 spore_land=1 spore_deposit=0.2"
SL="sleep_rate=0.01 wake_scent=1 sleep_deposit=0.3 alarm_keep=0.98 alarm_relay=20 wake_alarm=1 alarm_weight=3 sleep_rest=600 strike_x=161 strike_r=8 strike=14"
FIRE="alarm_keep=0.98 strike=10,10.5,11,11.5,12,12.5,13,13.5,14,14.5,15 strike_x=95 strike_r=8"
LIST="w-plain|$B lifeline=1
w-sporer|$B $SP spore_rate=0.0001
w-burst|$B $SP spore_burst=0.05 spore_alarm=1 alarm_keep=0.98 strike=10 strike_x=95
w-sleeper|$B $SL
w-swarm|$B $FIRE alarm_weight=3
w-flee|$B $FIRE alarm_weight=-10"
n=0
for s in $SEEDS; do
  while IFS='|' read -r name args; do
    ( $BIN $args seeds=$s tag=$name < /dev/null 2>&1 | grep -E "^seed|^==" ) > "$OUT/$name-s$s.txt" &
    n=$((n+1)); (( n % JOBS == 0 )) && wait
  done <<< "$LIST"
done
wait
grep -h "life walk" "$OUT"/w-*-s*.txt | grep -E "t +(20|60)\.0s"
