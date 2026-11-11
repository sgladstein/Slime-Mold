#!/usr/bin/env bash
# Paired test for grazing (2026-11-11): do droplets move? From the repo root,
# after cargo build --release --example bigdish:
#   bash Reports/creatures-2026-11-11/graze.sh [OUT]
# Deciding measure, named before the run: median drift of body centres over
# the last 2 s of 10 s (drift.py), graze_weight 10 vs 0. Also body count.
set -u
OUT=${1:-/tmp/graze}; mkdir -p "$OUT"
BIN=./target/release/examples/bigdish
B="preset=dense piles=1 food_deposit=0 crowd=1 walkers=30000 deposit=5 rotate_angle=45 sensor_angle=90 sensor_offset=9 keep=0.93 blob=1 blob_r=120 blocked_deposit=1 report=10 threads=1 seconds=10 every=12 style=count"
n=0
for s in 1 2 3 4 5 6 7 8; do
  for w in 0 10; do
    d="$OUT/w$w-s$s"; rm -rf "$d"; mkdir -p "$d"
    $BIN $B seeds=$s graze_weight=$w tag=w$w frames="$d" > "$d.log" 2>&1 &
    n=$((n+1)); (( n % 4 == 0 )) && wait
  done
done
wait
for s in 1 2 3 4 5 6 7 8; do for w in 0 10; do
  echo "seed $s w$w $(python3 "$(dirname "$0")/drift.py" "$OUT/w$w-s$s") $(grep -o 'ms/tick *[0-9.]*' "$OUT/w$w-s$s.log" | tail -1)"
done; done
