#!/usr/bin/env bash
# Look 3: growth with energy flow, and short explorer stages for the newborn (2026-11-11).
SP=/tmp/claude-0/-home-user/0052dd69-4ec9-5a77-9d4b-17dd6d4ad826/scratchpad
BIN=$SP/bdj; OUT=$SP/tl/l3; mkdir -p $OUT
SECS=${SECS:-90}
P="preset=dense piles=12 blob=1 blob_r=15 walkers=3000 capacity=80000 crystal_charge=200 crystal_eat=0.05 graze_eat=0 upkeep=0.0005 energy_flow=0.05"
LIST="gfexp30|$P explore_steps=30 explore_spread=1
gfexp80|$P explore_steps=80 explore_spread=1
gfup|$P upkeep=0.002
gfflow02|$P energy_flow=0.02"
n=0
while IFS='|' read -r name args; do
  ( $BIN $args seeds=1 tag=$name report=5 joined=1 threads=1 seconds=$SECS look=$OUT at=5,15,30,60,90 2>&1 | grep -E "^seed" > $OUT/$name-s1.txt ) &
  n=$((n+1)); (( n % 4 == 0 )) && wait
done <<< "$LIST"
wait
