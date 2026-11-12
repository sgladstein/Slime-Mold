#!/usr/bin/env bash
# Look 4: the Larder race world's numbers (larder_world::race_params) on bigdish,
# with explorer stages and energy flow (2026-11-11).
SP=/tmp/claude-0/-home-user/0052dd69-4ec9-5a77-9d4b-17dd6d4ad826/scratchpad
BIN=$SP/bdj2; OUT=$SP/tl/l4; mkdir -p $OUT
SECS=${SECS:-90}
R="preset=today scale=2 substeps=2 walkers=5000 capacity=50000 scent_offset=36 scent_coarse=0 piles=23 rect=1 crystal_charge=512 crystal_eat=0.005 terrain_rock=0.2 terrain_wet=0.2 upkeep=0.001 blob=1"
LIST="race|$R
raceexp|$R explore_steps=30 explore_spread=1
raceflow|$R energy_flow=0.05
raceexpflow|$R explore_steps=30 explore_spread=1 energy_flow=0.05"
n=0
while IFS='|' read -r name args; do
  ( $BIN $args seeds=1 tag=$name report=5 joined=1 threads=1 seconds=$SECS look=$OUT at=5,15,30,60,90 2>&1 | grep -E "^seed" > $OUT/$name-s1.txt ) &
  n=$((n+1)); (( n % 4 == 0 )) && wait
done <<< "$LIST"
wait
