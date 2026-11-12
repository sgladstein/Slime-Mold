#!/usr/bin/env bash
# First looks for "explore, then connect food with tubes" (2026-11-11).
SP=/tmp/claude-0/-home-user/0052dd69-4ec9-5a77-9d4b-17dd6d4ad826/scratchpad
BIN=$SP/bd; OUT=$SP/tl/l1; mkdir -p $OUT
P="preset=dense piles=12 blob=1 blob_r=50"
DROP="crowd=1 walkers=30000 deposit=5 rotate_angle=45 sensor_angle=90 sensor_offset=9 keep=0.93 blocked_deposit=1"
NET="walkers=30000"
CRY="crystal_charge=200 crystal_eat=0.05"
JEN="crowd=0 sensor_angle=45 rotate_angle=22.5 sensor_offset=18 deposit=0.5 walkers=200000"
LIST="foodreach|$P $DROP graze_weight=10
net|$P $NET
netexp|$P $NET explore_steps=600 explore_spread=1
netcry|$P $NET $CRY
jencry|$P $JEN $CRY
netexpcry|$P $NET explore_steps=600 explore_spread=1 $CRY"
n=0
while IFS='|' read -r name args; do
  ( $BIN $args seeds=1 tag=$name report=5 threads=1 seconds=60 look=$OUT at=5,15,30,60 2>&1 | grep -E "^seed" > $OUT/$name-s1.txt ) &
  n=$((n+1)); (( n % 4 == 0 )) && wait
done <<< "$LIST"
wait
