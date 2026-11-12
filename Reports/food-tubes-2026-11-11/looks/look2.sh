#!/usr/bin/env bash
# Look 2: growth paid for by crystals; children start as explorers (2026-11-11).
SP=/tmp/claude-0/-home-user/0052dd69-4ec9-5a77-9d4b-17dd6d4ad826/scratchpad
BIN=$SP/bd; OUT=$SP/tl/l2; mkdir -p $OUT
SECS=${SECS:-90}
P="preset=dense piles=12 blob=1 blob_r=15 walkers=3000 capacity=80000 crystal_charge=200 crystal_eat=0.05 graze_eat=0 upkeep=0.0005"
LIST="grow|$P
growexp|$P explore_steps=200 explore_spread=1
growexpflow|$P explore_steps=200 explore_spread=1 energy_flow=0.05
growflow|$P energy_flow=0.05"
n=0
while IFS='|' read -r name args; do
  ( $BIN $args seeds=1 tag=$name report=5 threads=1 seconds=$SECS look=$OUT at=5,15,30,60,90 2>&1 | grep -E "^seed" > $OUT/$name-s1.txt ) &
  n=$((n+1)); (( n % 4 == 0 )) && wait
done <<< "$LIST"
wait
