#!/usr/bin/env bash
# Looks first (seed 1): the three tiers in the Larder world's race, the
# settings of its growth sizing (Reports/larder-world-roadmap-2026-11-11.md,
# "charge 512"): 512 x 320, 23 crystals that run dry, the mould starting as
# a blob of 5,000 walkers on the home crystal (pile 0) in 50,000 slots,
# upkeep 0.001. Strikes and baits aim at a crystal (strike_pile=N).
#   bash Reports/mould-tiers-2026-11-11/world/look.sh [WORK]
set -u
WORK=${1:-/tmp/tiers-world-look}; mkdir -p "$WORK"
BIN=./target/release/examples/bigdish
W="preset=today scale=2 substeps=2 scent_offset=36 scent_coarse=0 piles=23 rect=1 crystal_charge=512 crystal_eat=0.005 walkers=5000 capacity=50000 upkeep=0.001 crystal_regrow=0"
B="$W seeds=1 seconds=60 report=5 lifeline=1 threads=2 style=life"
SP="spore_scent=1 spore_reach=36 spore_land=1 spore_deposit=0.2"
SL="sleep_rate=0.01 wake_scent=1 sleep_deposit=0.3 alarm_keep=0.98 alarm_relay=20 wake_alarm=1 alarm_weight=3 sleep_rest=600 strike_pile=0 strike_r=8 strike=14"
FIRE="alarm_keep=0.98 strike=10,10.5,11,11.5,12,12.5,13,13.5,14,14.5,15 strike_pile=0 strike_r=8"
LIST="plain|$B
sporer5|$B $SP spore_rate=0.00001
sporer4|$B $SP spore_rate=0.0001
sleeper|$B $SL
fire|$B $FIRE
swarm|$B $FIRE alarm_weight=3"
ONLY=${ONLY:-}
n=0
while IFS='|' read -r name args; do
  [[ -n "$ONLY" && " $ONLY " != *" $name "* ]] && continue
  rm -rf "$WORK/$name"; mkdir -p "$WORK/$name"
  ( $BIN $args tag=$name look="$WORK/$name" at=2,5,10,14,16,20,30,45,60 > "$WORK/$name.log" 2>&1 < /dev/null ) &
  n=$((n+1)); (( n % 2 == 0 )) && wait
done <<< "$LIST"
wait
