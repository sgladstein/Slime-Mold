#!/usr/bin/env bash
# The Sporer's pace (2026-11-11). The owner, 14:59Z: no mould may take the
# whole map in about 10 s. At spore_rate 1e-4 the Sporer holds all 7 piles by
# 10 s on 6 of 8 seeds, so this sets how slow it can be made. From the repo
# root, after cargo build --release --example bigdish:
#   bash Reports/mould-tiers-2026-11-11/pace.sh [JOBS] [OUT]
# Measure, named before the run (seeds 1-3, descriptive): piles held at 10,
# 20, 30 and 60 s for spore_rate 1e-5 and 3e-6, against no spores.
set -u
JOBS=${1:-4}; OUT=${2:-Reports/mould-tiers-2026-11-11/raw}; mkdir -p "$OUT"
BIN=./target/release/examples/bigdish
SEEDS=${SEEDS:-"1 2 3"}
B="preset=dense walkers=20000 blob=1 blob_r=40 report=10 threads=1 seconds=60"
SP="spore_scent=1 spore_reach=36 spore_land=1 spore_deposit=0.2"
LIST="p5-1e5|$B $SP spore_rate=0.00001
p5-3e6|$B $SP spore_rate=0.000003
p5-off|$B lifeline=1"
n=0
for s in $SEEDS; do
  while IFS='|' read -r name args; do
    ( $BIN $args seeds=$s tag=$name 2>&1 | grep -E "^seed|^==" ) > "$OUT/$name-s$s.txt" &
    n=$((n+1)); (( n % JOBS == 0 )) && wait
  done <<< "$LIST"
done
wait
grep -h "life walk" "$OUT"/p5-*-s*.txt | grep -E "t +(10|20|30|60)\.0s"
