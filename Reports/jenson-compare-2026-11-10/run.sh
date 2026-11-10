#!/usr/bin/env bash
# Paired counts behind the Jenson comparison (2026-11-10). From the repo root:
#   cargo build --release --example bigdish
#   bash Reports/jenson-compare-2026-11-10/run.sh
# Deciding measures, named before the runs: loops at 10 s (and tube width
# for the one-per-cell pair). No food (one pile, no scent), uniform start.
#   crowd:   100,000 walkers, the dish's numbers, one walker a cell (crowd 1)
#            against no limit (crowd 0, Jenson's rule); seeds 1-8.
#   respawn: 1,000,000 walkers, no limit, rotate 22.5, sensors 18 out,
#            respawn 0 against 0.01 a step; seeds 1-8 (7 and 8 added after six could not pass the correction).
set -u
BIN=${BIN:-./target/release/examples/bigdish}
OUT=${OUT:-Reports/jenson-compare-2026-11-10/raw}
JOBS=${JOBS:-4}
BASE="preset=dense blob=0 piles=1 food_deposit=0 seconds=10 report=5 threads=1"
run() { local tag=$1; shift; "$BIN" $BASE tag=$tag "$@" > "$OUT/$tag.txt" 2>&1; }
n=0
for s in 1 2 3 4 5 6 7 8; do
  for c in 1 0; do
    run crowd$c-s$s seeds=$s crowd=$c & n=$((n+1)); (( n % JOBS == 0 )) && wait
  done
done
for s in 1 2 3 4 5 6 7 8; do
  for r in 0 0.01; do
    run respawn$r-s$s seeds=$s crowd=0 walkers=1000000 deposit=0.5 rotate_angle=22.5 sensor_offset=18 respawn=$r & n=$((n+1)); (( n % JOBS == 0 )) && wait
  done
done
wait
