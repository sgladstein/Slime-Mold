#!/usr/bin/env bash
# Look 6: is it growth or the small start that makes the race start spread?
# race without growth (upkeep 0), and race from a start as dense as the
# graphics thread's GPU blob (400,000 of 2.6M fine cells, about 15%: 25,000
# here), with and without growth. Seeds 1-3, 90 s (2026-11-12).
SP=/tmp/claude-0/-home-user/0052dd69-4ec9-5a77-9d4b-17dd6d4ad826/scratchpad
BIN=$SP/bdft; OUT=$SP/tl/l6; mkdir -p $OUT
R="preset=today scale=2 substeps=2 scent_offset=36 scent_coarse=0 piles=23 rect=1 crystal_charge=512 crystal_eat=0.005 terrain_rock=0.2 terrain_wet=0.2 blob=1"
LIST="nogrow|$R walkers=5000
big|$R walkers=25000 capacity=250000 upkeep=0.001
bignogrow|$R walkers=25000"
n=0
for s in 1 2 3; do
while IFS='|' read -r name args; do
  extra=""; [ $s -eq 1 ] && extra="look=$OUT at=5,15,30,60,90"
  ( $BIN $args seeds=$s tag=$name report=5 joined=1 threads=1 seconds=90 $extra 2>&1 | grep -E "^seed" > $OUT/$name-s$s.txt ) &
  n=$((n+1)); (( n % 4 == 0 )) && wait
done <<< "$LIST"
done
wait
