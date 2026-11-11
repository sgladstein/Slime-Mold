#!/usr/bin/env bash
# Two species with their own settings on one dish (`species_sets`, 2026-11-11).
# From the repo root, after cargo build --release --example bigdish:
#   bash Reports/species-sets-2026-11-11/run.sh [JOBS] [OUT]
# Species 0 is the Drifter (grazing droplets: crowd 1, SA 90, RA 45, SO 9,
# deposit 5, keep 0.93, blocked_deposit 1, graze 10), 29,870 walkers in the
# left half; species 1 is a Sprawler (the Jenson network: crowd 0, SA 45,
# RA 22.5, SO 18, deposit 0.5), 1,000,130 walkers in the right half
# (`species_zones`; avoidance 1, Lague's mask; no food, no conversion).
# Arms: both together; the Drifter alone (same walkers, same half); the
# Sprawler alone (a species_share so small that species 0 gets none).
# Deciding measure, named before the run: each species' own area (`own`
# line: cells over the tube cut scaled by its deposit, dish cells) at 30 s
# and 60 s, together over alone; they coexist if the ratio is within 0.5-2
# for both species on 8 of 8 seeds. Secondary: droplet pieces (bodies).
set -u
JOBS=${1:-4}; OUT=${2:-/tmp/species-sets}; mkdir -p "$OUT"
BIN=${BIN:-./target/release/examples/bigdish}
SEEDS=${SEEDS:-"1 2 3 4 5 6 7 8"}; SECS=${SECS:-60}
G="preset=dense piles=1 food_deposit=0 crowd=0 deposit=0.5 rotate_angle=22.5 sensor_offset=18 blob=0 species=2 species_zones=1"
S0="s0_crowd=1 s0_sensor_angle=90 s0_rotate_angle=45 s0_sensor_offset=9 s0_deposit=5 s0_keep=0.93 s0_blocked_deposit=1 s0_graze_weight=10"
LIST="together|$G walkers=1030000 species_share=0.029 $S0
drifter|$G walkers=29870 species_share=1 $S0
sprawler|$G walkers=1000130 species_share=0.000000001 $S0"
n=0
for s in $SEEDS; do
  while IFS='|' read -r name args; do
    f="$OUT/$name-s$s.txt"
    [ -s "$f" ] && grep -q " t  $SECS.0s  own" "$f" && continue   # done before a restart
    ( $BIN $args seeds=$s tag=$name report=10 threads=1 seconds=$SECS 2>&1 | grep -E "^seed" ) > "$f" &
    n=$((n+1)); (( n % JOBS == 0 )) && wait
  done <<< "$LIST"
done
wait
for f in "$OUT"/*.txt; do grep -E " own |tube" "$f"; done
