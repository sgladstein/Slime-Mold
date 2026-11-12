#!/usr/bin/env bash
# Reviewer probe 2 (not in the repo): the tiers at the game's operating point.
# Larder CPU world as in the roadmap's crystal sizing (512 x 320, 23 crystals
# that run dry, growth from 5,000 walkers in 50,000 slots, upkeep 0.001),
# charge 8 so the plain mould starves inside about 2 minutes. Seed 1, 90 s.
set -u
cd /tmp/review-1/repo
OUT=/tmp/review-1/out; BIN=./target/release/examples/bigdish
W="preset=today seconds=90 report=10 threads=1 scale=2 substeps=2 scent_offset=36 scent_coarse=0 piles=23 rect=1 crystal_charge=8 crystal_eat=0.005 walkers=5000 capacity=50000 upkeep=0.001 crystal_regrow=0"
SP="spore_scent=1 spore_reach=36 spore_land=1 spore_deposit=0.2 spore_rate=0.0001"
SL="sleep_rate=0.01 wake_scent=1 sleep_deposit=0.3 sleep_rest=600"
LIST="g-plain|$W lifeline=1
g-sporer|$W $SP
g-sleeper|$W $SL"
n=0
while IFS='|' read -r name args; do
  ( $BIN $args seeds=1 tag=$name < /dev/null 2>&1 | grep -E "^seed|^==" ) > "$OUT/$name-s1.txt" &
  n=$((n+1)); (( n % 2 == 0 )) && wait
done <<< "$LIST"
wait
echo done
