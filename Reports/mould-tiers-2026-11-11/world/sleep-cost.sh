#!/usr/bin/env bash
# The Sleeper's sleep cost (sleep_upkeep, since removed: see dead-ends.md)
# in the world race: run.sh's three Sleeper arms at one cost, the sleeper
# arm to 90 s, into raw-uCOST/. Ran on the 21:05Z build, where tier=sleeper
# had no sleep cost of its own and `sleep_upkeep=` set the share of upkeep a
# dormant walker paid (0.5 was run as the tier's own key, raw-u0.5/).
#   SU=0.1 bash Reports/mould-tiers-2026-11-11/world/sleep-cost.sh [JOBS]
# Measures, named before the run: W3 (bait) as in run.sh; and whether the
# world still freezes: dormant walkers at 90 s against 30 s, and the
# walkers alive at 30 s against 90 s (a frozen world has them equal).
set -u
JOBS=${1:-4}; SU=${SU:-0.1}; OUT=Reports/mould-tiers-2026-11-11/world/raw-u$SU; mkdir -p "$OUT"
BIN=./target/release/examples/bigdish
SEEDS=${SEEDS:-"1 2 3 4 5 6 7 8"}
W="preset=today scale=2 substeps=2 scent_offset=36 scent_coarse=0 piles=23 rect=1 crystal_charge=512 crystal_eat=0.005 walkers=5000 capacity=50000 upkeep=0.001 crystal_regrow=0"
B="$W report=1 lifeline=1 threads=1 strike_r=8 tier=sleeper sleep_upkeep=$SU"
LIST="sleeper|$B seconds=90
bait-on|$B seconds=17 bait=10 bait_pile=0 strike=14 strike_pile=0
bait-off|$B seconds=17 strike=14 strike_pile=0"
n=0
for s in $SEEDS; do
  while IFS='|' read -r name args; do
    ( $BIN $args seeds=$s tag=$name 2>&1 < /dev/null | grep -E "^seed" ) > "$OUT/$name-s$s.txt" &
    n=$((n+1)); (( n % JOBS == 0 )) && wait
  done <<< "$LIST"
done
wait
