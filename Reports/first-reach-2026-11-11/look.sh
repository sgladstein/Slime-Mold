#!/usr/bin/env bash
# The first-reach pictures on the page (2026-11-11): four arms of run.sh on
# seeds 1 and 2, drawn in the game's amber at 10, 30 and 60 s. The runs are
# the census runs again (same `firsts` line). From the repo root, after
# cargo build --release --example bigdish:
#   bash Reports/first-reach-2026-11-11/look.sh [OUT]
set -u
OUT=${1:-/tmp/first-reach-look}; mkdir -p "$OUT"
BIN=${BIN:-./target/release/examples/bigdish}
D="preset=dense piles=12 crowd=1 walkers=30000 deposit=5 rotate_angle=45 sensor_angle=90 sensor_offset=9 keep=0.93 blob=1 blob_r=50 blocked_deposit=1"
L() { name=$1; s=$2; shift 2; $BIN $D "$@" seeds=$s tag=$name report=30 threads=1 seconds=60 look="$OUT" at=10,30,60 2>&1 | grep -E "firsts|piles" > "$OUT/$name-s$s.log"; }
for s in 1 2; do
  ( L droplets $s; L dropfast $s mix_share=0.2 mix_step=2 mix_deposit=0.3 ) &
  ( L grazers $s graze_weight=10; L grazescouts $s graze_weight=10 mix_share=0.2 mix_reach=3 mix_deposit=0.3 ) &
  wait
done
