#!/usr/bin/env bash
# Paired test: does an edge in numbers decide a fight between hunting
# droplets? (2026-11-11). From the repo root, after
# cargo build --release --example bigdish:
#   bash Reports/creatures-2026-11-11/hunt.sh [JOBS]
# Two species of grazing droplets, mixed in one blob, each drawn to the other
# (species_avoid -0.5: own x 0.5 + total x 0.5), conversion 1.2 over the cell
# (r 0) or over a 17 x 17 square (r 8). Arm A: species 0 has 0.55 of the
# walkers; arm B: 0.5 (same interleaving). Deciding measure, named before the
# run: species 0's gain in walker share from the start to 30 s, A against B,
# for each r.
set -u
JOBS=${1:-4}; SEEDS=${SEEDS:-"1 2 3 4 5 6 7 8"}
OUT=${OUT:-/tmp/hunt}; mkdir -p "$OUT"
BIN=${BIN:-./target/release/examples/bigdish}
B="preset=dense piles=1 food_deposit=0 crowd=1 walkers=30000 deposit=5 rotate_angle=45 sensor_angle=90 sensor_offset=9 keep=0.93 blob=1 blob_r=120 blocked_deposit=1 graze_weight=10 species=2 species_avoid=-0.5 species_convert=1.2 report=10 threads=1 seconds=30"
n=0
for s in $SEEDS; do
  for r in 0 8; do
    for arm in A B; do
      share=0.5; [ $arm = A ] && share=0.55
      ( $BIN $B seeds=$s tag=hunt-r$r-$arm species_convert_r=$r species_share=$share 2>&1 | grep -E "walkers" ) > "$OUT/r$r-$arm-$s.txt" &
      n=$((n+1)); (( n % JOBS == 0 )) && wait
    done
  done
done
wait
for s in $SEEDS; do for r in 0 8; do for arm in A B; do cat "$OUT/r$r-$arm-$s.txt"; done; done; done
