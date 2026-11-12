#!/usr/bin/env bash
# Does the Sleeper still freeze a world whose crystals run dry? Review 1's
# probe (charge 8, so the plain mould starves within about 2 minutes),
# 120 s, seeds 1-4: the Sleeper with free sleep (sleep_death=0) against
# the tier (sleep_death 1e-4), and plain. Measure, named before the run:
# walkers alive at 60 s against 120 s (a frozen world has them equal) and
# dormant at 120 s. Output in lean/.
set -u
OUT=Reports/mould-tiers-2026-11-11/world/lean; mkdir -p "$OUT"
BIN=./target/release/examples/bigdish
W="preset=today scale=2 substeps=2 scent_offset=36 scent_coarse=0 piles=23 rect=1 crystal_charge=8 crystal_eat=0.005 walkers=5000 capacity=50000 upkeep=0.001 crystal_regrow=0 report=10 lifeline=1 threads=1 seconds=120"
LIST="plain|$W
free|$W tier=sleeper sleep_death=0
tier|$W tier=sleeper"
n=0
for s in 1 2 3 4; do
  while IFS='|' read -r name args; do
    ( $BIN $args seeds=$s tag=$name 2>&1 < /dev/null | grep -E "^seed" ) > "$OUT/$name-s$s.txt" &
    n=$((n+1)); (( n % 4 == 0 )) && wait
  done <<< "$LIST"
done
wait
