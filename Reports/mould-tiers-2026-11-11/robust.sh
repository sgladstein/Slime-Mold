#!/usr/bin/env bash
# Robustness of the Sporer and Sleeper (2026-11-11), from the creature
# review's traps: does a tier hold for minutes with food, at other body
# sizes, and away from the dish wall? From the repo root, after
# cargo build --release --example bigdish:
#   bash Reports/mould-tiers-2026-11-11/robust.sh [JOBS] [OUT]
# Measures, named before the run (seeds 1-3, descriptive, no test):
#  R1 long Sporer: piles held and tube bodies at 60 and 120 s, on vs off.
#  R2 long Sleeper: dormant share, piles held at 60 and 120 s, against the
#     plain mould under the same strike at 14 s.
#  R3 sizes: Sporer piles held at 10 and 20 s at 5k walkers (blob_r 20) and
#     80k (blob_r 70), both about a quarter full like the 20k disc of 40.
#  R4 wall rail: `rim` (the share of grounded living walkers within 2 dish
#     cells of the agar edge) at 20 and 120 s, each tier against its control.
#     It read 0 on every arm: the wall reads as a wall 9 dish cells out, so
#     the 2-cell band empties in 0.2 s. `rim` now also gives the 10-cell
#     share (the later runs here print both), and wall.sh is the check.
set -u
JOBS=${1:-4}; OUT=${2:-Reports/mould-tiers-2026-11-11/raw}; mkdir -p "$OUT"
BIN=./target/release/examples/bigdish
SEEDS=${SEEDS:-"1 2 3"}
B="preset=dense walkers=20000 blob=1 blob_r=40 report=10 threads=1"
SP="spore_scent=1 spore_reach=36 spore_land=1 spore_deposit=0.2 spore_rate=0.0001"
SL="sleep_rate=0.01 wake_scent=1 sleep_deposit=0.3 alarm_keep=0.98 alarm_relay=20 wake_alarm=1 alarm_weight=3 sleep_rest=600 strike_x=161 strike_r=8 strike=14"
LIST="r1-on|$B $SP seconds=120
r1-off|$B lifeline=1 seconds=120
r2-on|$B $SL seconds=120
r2-off|$B alarm_keep=0.98 strike_x=161 strike_r=8 strike=14 lifeline=1 seconds=120
r3-5k-on|preset=dense walkers=5000 blob=1 blob_r=20 report=10 threads=1 $SP seconds=20
r3-5k-off|preset=dense walkers=5000 blob=1 blob_r=20 report=10 threads=1 lifeline=1 seconds=20
r3-80k-on|preset=dense walkers=80000 blob=1 blob_r=70 report=10 threads=1 $SP seconds=20
r3-80k-off|preset=dense walkers=80000 blob=1 blob_r=70 report=10 threads=1 lifeline=1 seconds=20"
n=0
for s in $SEEDS; do
  while IFS='|' read -r name args; do
    ( $BIN $args seeds=$s tag=$name 2>&1 | grep -E "^seed|^==" ) > "$OUT/$name-s$s.txt" &
    n=$((n+1)); (( n % JOBS == 0 )) && wait
  done <<< "$LIST"
done
wait
grep -h "life walk" "$OUT"/r[1-3]-*-s*.txt | grep -E "t +(10|20|60|120)\.0s"
