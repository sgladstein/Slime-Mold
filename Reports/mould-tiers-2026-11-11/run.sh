#!/usr/bin/env bash
# New ladder tiers and how each answers the player's strike (2026-11-11).
# From the repo root, after cargo build --release --example bigdish:
#   bash Reports/mould-tiers-2026-11-11/run.sh [JOBS] [OUT]
# Then: python3 Reports/mould-tiers-2026-11-11/pairs.py OUT
#
# Body: 20,000 walkers in a centre disc of radius 40 dish cells on the
# 1024 x 640 dish (preset dense, one walker a cell), 7 food piles; it grows
# a meshy body with arms to the nearest piles. Seeds 1-8, 20 s each.
#
# Deciding measures, named before the run (one a test, sign test over 8
# paired seeds, Bonferroni over the 5 tests):
#  T1 sporer:  piles held at 20 s (piles with 20+ grounded walkers within
#              pile_r + 4 dish cells), spores (spore_rate 1e-4, fruiting
#              only on food) against none. Expect more.
#  T2 burst:   piles held at 20 s after one strike at the body's left edge
#              at 10 s, spore_burst 0.05 against none. Expect more (striking
#              a Sporer spreads it).
#  T3 bait:    Sleeper, grounded walkers within 16 dish cells of a strike at
#              the right edge at 14 s, 2 s after it (t 16 s); a bait shock
#              (no kill) at the left edge at 10 s against none. Expect fewer
#              (the bait leaves the crust deaf).
#  T4 swarm:   walkers killed by 11 strikes (every 0.5 s, 10-15 s, left
#              edge, radius 8), alarm_weight +3 against 0. Expect more.
#  T5 flee:    tube bodies of 4+ dish cells at 20 s under the same fire,
#              alarm_weight -10 against 0. Expect more (it breaks up).
set -u
JOBS=${1:-4}; OUT=${2:-Reports/mould-tiers-2026-11-11/raw}; mkdir -p "$OUT"
BIN=./target/release/examples/bigdish
SEEDS=${SEEDS:-"1 2 3 4 5 6 7 8"}
B="preset=dense walkers=20000 blob=1 blob_r=40 report=1 threads=1 seconds=20"
SP="spore_scent=1 spore_reach=36 spore_land=1 spore_deposit=0.2"
SL="sleep_rate=0.01 wake_scent=1 sleep_deposit=0.3 alarm_keep=0.98 alarm_relay=20 wake_alarm=1 alarm_weight=3 sleep_rest=600 strike_x=161 strike_r=8 strike=14"
FIRE="alarm_keep=0.98 strike=10,10.5,11,11.5,12,12.5,13,13.5,14,14.5,15 strike_x=95 strike_r=8"
LIST="t1-on|$B $SP spore_rate=0.0001
t1-off|$B lifeline=1
t2-on|$B $SP spore_burst=0.05 spore_alarm=1 alarm_keep=0.98 strike=10 strike_x=95
t2-off|$B $SP alarm_keep=0.98 strike=10 strike_x=95
t3-on|$B $SL bait=10 bait_x=95
t3-off|$B $SL
t4-on|$B $FIRE alarm_weight=3
t4-off|$B $FIRE
t5-on|$B $FIRE alarm_weight=-10"
n=0
for s in $SEEDS; do
  while IFS='|' read -r name args; do
    ( $BIN $args seeds=$s tag=$name 2>&1 | grep -E "^seed" ) > "$OUT/$name-s$s.txt" &
    n=$((n+1)); (( n % JOBS == 0 )) && wait
  done <<< "$LIST"
done
wait
