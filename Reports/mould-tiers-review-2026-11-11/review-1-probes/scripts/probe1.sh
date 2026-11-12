#!/usr/bin/env bash
# Reviewer probes (not part of the repo). 2 jobs at a time, threads=1.
set -u
cd /tmp/review-1/repo
OUT=/tmp/review-1/out; mkdir -p $OUT
BIN=./target/release/examples/bigdish
B="preset=dense walkers=20000 blob=1 blob_r=40 report=1 threads=1 seconds=20"
SP="spore_scent=1 spore_reach=36 spore_land=1 spore_deposit=0.2"
SL="sleep_rate=0.01 wake_scent=1 sleep_deposit=0.3 alarm_keep=0.98 alarm_relay=20 wake_alarm=1 alarm_weight=3 sleep_rest=600 strike_x=161 strike=14"
LIST="a-big|$B $SP spore_rate=0.00001 spore_burst=0.05 spore_alarm=1 alarm_keep=0.98 strike=10 strike_x=95 strike_r=10|1 2 3
a-bullet|$B $SP spore_rate=0.00001 spore_burst=0.05 spore_alarm=1 alarm_keep=0.98 strike=10 strike_x=95 strike_r=1.75|1 2 3
b-twice|$B $SL strike_r=8 strike=10,14|1 2
b-baittop|$B $SL strike_r=8 bait=10 bait_x=128 bait_y=30|1 2
b-bullet|$B $SL strike_r=1.75|1 2"
n=0
while IFS='|' read -r name args seeds; do
  for s in $seeds; do
    ( $BIN $args seeds=$s tag=$name < /dev/null 2>&1 | grep -E "^seed" ) > "$OUT/$name-s$s.txt" &
    n=$((n+1)); (( n % 2 == 0 )) && wait
  done
done <<< "$LIST"
wait
echo done
