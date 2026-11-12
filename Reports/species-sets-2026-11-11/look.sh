#!/usr/bin/env bash
# The pictures on the page (2026-11-11): the `together` arm of run.sh on
# seeds 1-3, drawn by species (Drifter amber, Sprawler teal) at 1, 5, 10 and
# 20 s. From the repo root, after cargo build --release --example bigdish:
#   bash Reports/species-sets-2026-11-11/look.sh [OUT]
set -u
OUT=${1:-/tmp/coexist}
BIN=${BIN:-./target/release/examples/bigdish}
G="preset=dense piles=1 food_deposit=0 crowd=0 walkers=1030000 deposit=0.5 rotate_angle=22.5 sensor_offset=18 blob=0 species=2 species_zones=1 species_share=0.029 s0_crowd=1 s0_sensor_angle=90 s0_rotate_angle=45 s0_sensor_offset=9 s0_deposit=5 s0_keep=0.93 s0_blocked_deposit=1 s0_graze_weight=10"
for s in 1 2 3; do
  mkdir -p "$OUT/s$s"
  $BIN $G seeds=$s seconds=20 report=5 threads=1 tag=coexist style=species look="$OUT/s$s/" at=1,5,10,20 &
done
wait
