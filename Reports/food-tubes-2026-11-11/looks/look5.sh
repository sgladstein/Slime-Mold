#!/usr/bin/env bash
# Look 5 (run inline, 2026-11-11): the race world's growth from the
# droplets' wide disc (seeds 1-3), and the droplets on the race world
# (seed 1), 30 s. The wide start joined 19-22 of 23 crystals by 5 s, so the
# census starts the race arm as the Larder game does instead.
SP=${SP:-/tmp/food-tubes-scratch}; BIN=${BIN:-./target/release/examples/bigdish}
mkdir -p $SP/tl/l5; R="preset=today scale=2 substeps=2 scent_offset=36 scent_coarse=0 piles=23 rect=1 crystal_charge=512 crystal_eat=0.005 terrain_rock=0.2 terrain_wet=0.2 blob=1"
for s in 1 2 3; do ( $BIN $R walkers=7500 capacity=75000 blob_r=75 upkeep=0.001 graze_eat=0 seeds=$s tag=grow report=5 joined=1 threads=1 seconds=30 look=$SP/tl/l5 at=5,15,30 2>&1 | grep -E "joined|alive" > $SP/tl/l5/grow-s$s.txt ) & done
( $BIN $R walkers=7500 blob_r=75 crowd=1 sensor_angle=90 rotate_angle=45 sensor_offset=9 deposit=5 keep=0.93 blocked_deposit=1 graze_weight=10 seeds=1 tag=droplets report=5 joined=1 threads=1 seconds=30 look=$SP/tl/l5 at=5,15,30 2>&1 | grep -E "joined" > $SP/tl/l5/droplets-s1.txt ) &
wait
