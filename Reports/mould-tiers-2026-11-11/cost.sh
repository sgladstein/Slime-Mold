#!/usr/bin/env bash
# Frame cost of the life-cycle rules (2026-11-11), from the repo root after
# cargo build --release --example bigdish:
#   bash Reports/mould-tiers-2026-11-11/cost.sh
# preset dense (100k walkers, one a cell) spread over the dish, one core, 10 s
# (1,200 fine steps); ms/tick is the mean over the run. Arms run one at a
# time, interleaved over two rounds, so drift on the machine hits all alike.
set -u
BIN=./target/release/examples/bigdish
B="preset=dense blob=0 piles=1 threads=1 seconds=10 report=10 seeds=1"
SP="spore_rate=0.0001 spore_scent=1 spore_reach=36 spore_land=1 spore_deposit=0.2"
SL="sleep_rate=0.01 wake_scent=1 sleep_deposit=0.3 sleep_rest=600 alarm_relay=20"
AL="alarm_keep=0.98 alarm_weight=3 strike=1 strike_x=128 strike_r=8"
for round in 1 2; do
  while IFS='|' read -r name args; do
    ms=$($BIN $B $args tag=$name 2>&1 | grep -E "t +10\.0s +tube" | grep -o "ms/tick [0-9.]*")
    echo "round $round $name $ms"
  done <<< "off|
lifeline|lifeline=1
alarm|$AL
spores|$SP
sleep|$SL alarm_keep=0.98
all|$SP $SL $AL"
done
