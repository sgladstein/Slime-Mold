#!/usr/bin/env bash
# Paired test for an edge in a species fight (2026-11-11). From the repo root,
# after cargo build --release --example bigdish:
#   SEEDS="1 2 3 4" bash Reports/creatures-2026-11-11/fight.sh [JOBS]
# Two species in their own halves of the 1M network, conversion 1.2, no
# avoidance. Arm A: species 0 lays 1.5x the trail (species_boost 1.5);
# arm B: even (1.0). Deciding measure, named before the run: species 0's
# share of walkers at 16 s. Prints the report lines at 8 and 16 s.
set -u
JOBS=${1:-4}; SEEDS=${SEEDS:-"1 2 3 4 5 6 7 8"}
OUT=${OUT:-/tmp/fight}; mkdir -p "$OUT"
BIN=./target/release/examples/bigdish
B="preset=dense piles=1 food_deposit=0 crowd=0 walkers=1000000 deposit=0.5 rotate_angle=22.5 sensor_offset=18 blob=0 species=2 species_zones=1 species_avoid=0 species_convert=1.2 report=8 threads=1 seconds=16"
n=0
for s in $SEEDS; do
  for arm in A B; do
    boost=1.0; [ $arm = A ] && boost=1.5
    ( $BIN $B seeds=$s tag=fight-$arm species_boost=$boost 2>&1 | grep -E "t +(8|16)\.0s" ) > "$OUT/$arm-$s.txt" &
    n=$((n+1)); (( n % JOBS == 0 )) && wait
  done
done
wait
for s in $SEEDS; do for arm in A B; do cat "$OUT/$arm-$s.txt"; done; done
