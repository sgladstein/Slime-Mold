#!/usr/bin/env bash
# Paired test for an edge in numbers in a species fight (2026-11-11). From the repo root,
# after cargo build --release --example bigdish:
#   SEEDS="1 2 3 4" bash Reports/creatures-2026-11-11/numbers.sh [JOBS]
# Two species in equal halves of the 1M network, conversion 1.2, no
# avoidance. Arm A: species 0 has 0.6 of the walkers (species_share 0.6),
# so its half is 1.5x as dense; arm B: 0.5 (the same interleaving, even).
# Deciding measure, named before the run: species 0's gain in walker share
# from the start to 16 s (share at 16 s minus 0.6 or 0.5), A against B.
# Prints the report lines at 8 and 16 s.
set -u
JOBS=${1:-4}; SEEDS=${SEEDS:-"1 2 3 4 5 6 7 8"}
OUT=${OUT:-/tmp/numbers}; mkdir -p "$OUT"
BIN=./target/release/examples/bigdish
B="preset=dense piles=1 food_deposit=0 crowd=0 walkers=1000000 deposit=0.5 rotate_angle=22.5 sensor_offset=18 blob=0 species=2 species_zones=1 species_avoid=0 species_convert=1.2 report=8 threads=1 seconds=16"
n=0
for s in $SEEDS; do
  for arm in A B; do
    share=0.5; [ $arm = A ] && share=0.6
    ( $BIN $B seeds=$s tag=numbers-$arm species_share=$share 2>&1 | grep -E "t +(8|16)\.0s" ) > "$OUT/$arm-$s.txt" &
    n=$((n+1)); (( n % JOBS == 0 )) && wait
  done
done
wait
for s in $SEEDS; do for arm in A B; do cat "$OUT/$arm-$s.txt"; done; done
