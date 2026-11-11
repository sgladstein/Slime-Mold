#!/usr/bin/env bash
# Paired tests for the outside rules (2026-11-11). From the repo root:
#   cargo build --release --example bigdish
#   bash Reports/outside-rules-2026-11-11/run.sh TEST [JOBS]
# TEST is one of territory, chance, size, cycle, area. Prints the report
# lines of the last two report times of each run. Deciding measures, named
# before the runs:
#   territory: left0 at 20 s (species 0's trail in its own half), avoid 1 vs 0
#   chance:    loops at 10 s, chance_exp 4 vs the Jones turn
#   size:      contrast at 10 s (p99 / median of tube trail), size_spread 0.6 vs 0
#   cycle:     changed at 10 s (tube cells whose leading species changed since 8 s), cyclic vs plain avoidance
#   area:      loops at 10 s, sensor_r 1 vs 0
set -u
TEST=$1; JOBS=${2:-4}
BIN=${BIN:-./target/release/examples/bigdish}
OUT=${OUT:-/tmp/outside-rules}; mkdir -p "$OUT"
BASE="preset=dense piles=1 food_deposit=0 crowd=0 walkers=1000000 deposit=0.5 rotate_angle=22.5 sensor_offset=18 threads=1 blob=0"
case $TEST in
  territory) SEC=20; A="species=2 species_zones=1 species_avoid=1"; B="species=2 species_zones=1 species_avoid=0";;
  chance) SEC=10; A="chance_exp=4"; B="";;
  size) SEC=10; COMMON="respawn=0.002 rotate_angle=45 sensor_offset=12 step=1.2 keep=0.97"; A="$COMMON size_spread=0.6"; B="$COMMON";;
  cycle) SEC=10; COMMON="species=3 sensor_angle=20 rotate_angle=20 sensor_offset=32 step=2"; A="$COMMON species_cycle=1"; B="$COMMON";;
  area) SEC=10; A="sensor_r=1"; B="";;
  *) echo "unknown test $TEST"; exit 1;;
esac
n=0
for s in 1 2 3 4 5 6 7 8; do
  for arm in A B; do
    args=$A; [ $arm = B ] && args=$B
    ( $BIN $BASE seconds=$SEC report=2 seeds=$s tag=$TEST-$arm $args 2>&1 | grep -E "t +($((SEC-2))|$SEC)\.0s" ) > "$OUT/$TEST-$arm-$s.txt" &
    n=$((n+1)); (( n % JOBS == 0 )) && wait
  done
done
wait
for s in 1 2 3 4 5 6 7 8; do for arm in A B; do cat "$OUT/$TEST-$arm-$s.txt"; done; done
