#!/usr/bin/env bash
# The three tiers in the Larder world's race (2026-11-11, after the merge of
# #70), from the repo root after cargo build --release --example bigdish:
#   bash Reports/mould-tiers-2026-11-11/world/run.sh [JOBS] [OUT]
# Then: python3 Reports/mould-tiers-2026-11-11/world/pairs.py OUT
#
# World: the growth sizing's race (Reports/larder-world-roadmap-2026-11-11.md,
# "charge 512"): 512 x 320, 23 crystals that run dry, 5,000 walkers in a
# blob on the home crystal in 50,000 slots, upkeep 0.001, births and
# starvation on. Tiers are `tier=` (src/mould/tiers.rs). Strikes and baits
# aim at the home crystal (strike_pile=0), radius 8 dish cells, alarm 50
# out to 16. Seeds 1-8.
#
# Deciding measures, named before the run (sign test over 8 paired seeds,
# Bonferroni over the 4 tests):
#  W1 threat:  walkers alive at 60 s, Sporer against plain. Expect more (an
#              idle player should face a stronger Sporer). Also shown:
#              piles held at 10 s (the owner's 10-second rule) and 60 s.
#  W2 burst:   piles held at 40 s after one strike at 20 s, the Sporer
#              against the Sporer with spore_burst 0. Expect more.
#  W3 bait:    the Sleeper, walkers within 16 dish cells of a strike at
#              14 s, 2 s after it (t 16 s); a bait (alarm, no kill) on the
#              same crystal at 10 s against none. Expect fewer.
#  W4 swarm:   walkers killed by the 10 follow-up strikes (10.5-15 s, every
#              0.5 s, after the first at 10 s), the Swarmer against plain
#              with the same alarm field (alarm_keep 0.98). Expect more.
# Not a test, shown: the Sleeper's walkers alive at 60 s against plain.
set -u
JOBS=${1:-3}; OUT=${2:-Reports/mould-tiers-2026-11-11/world/raw}; mkdir -p "$OUT"
BIN=./target/release/examples/bigdish
SEEDS=${SEEDS:-"1 2 3 4 5 6 7 8"}
W="preset=today scale=2 substeps=2 scent_offset=36 scent_coarse=0 piles=23 rect=1 crystal_charge=512 crystal_eat=0.005 walkers=5000 capacity=50000 upkeep=0.001 crystal_regrow=0"
B="$W report=1 lifeline=1 threads=1 strike_r=8"
FIRE="strike=10,10.5,11,11.5,12,12.5,13,13.5,14,14.5,15 strike_pile=0"
LIST="plain|$B seconds=60
sporer|$B seconds=60 tier=sporer
sleeper|$B seconds=60 tier=sleeper
burst-on|$B seconds=40 tier=sporer strike=20 strike_pile=0
burst-off|$B seconds=40 tier=sporer spore_burst=0 strike=20 strike_pile=0
bait-on|$B seconds=17 tier=sleeper bait=10 bait_pile=0 strike=14 strike_pile=0
bait-off|$B seconds=17 tier=sleeper strike=14 strike_pile=0
swarm-on|$B seconds=16 tier=swarmer $FIRE
swarm-off|$B seconds=16 alarm_keep=0.98 $FIRE"
# ONLY="sleeper bait-on bait-off" runs just those arms.
ONLY=${ONLY:-}
n=0
for s in $SEEDS; do
  while IFS='|' read -r name args; do
    [[ -n "$ONLY" && " $ONLY " != *" $name "* ]] && continue
    ( $BIN $args seeds=$s tag=$name 2>&1 < /dev/null | grep -E "^seed" ) > "$OUT/$name-s$s.txt" &
    n=$((n+1)); (( n % JOBS == 0 )) && wait
  done <<< "$LIST"
done
wait
