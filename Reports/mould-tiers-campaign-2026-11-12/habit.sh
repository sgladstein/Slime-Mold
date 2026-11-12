#!/usr/bin/env bash
# Habituation (the scar field) in the race world, paired seeds 1-8.
#   bash Reports/mould-tiers-campaign-2026-11-12/habit.sh [OUT] [JOBS]
# then python3 Reports/mould-tiers-campaign-2026-11-12/habit.py OUT
#
# World: as look.sh (larder_world::race_params at scale 2). Fire: 16
# strikes of radius 8 dish cells, every 2 s from 10 s to 40 s, at the
# crystal nearest home (strike_pile=1), alarm 50 out to 16; run to 60 s.
#
# Deciding measures, named before the run (sign test over 8 paired seeds,
# Holm over the 3):
#  H1 wary:   walkers within 16 dish cells of the strike point at 60 s
#             (20 s after the last strike), Wary against the same alarm
#             with no scar (flee). Expect fewer.
#  H2 grudge: the same count, Grudge against the Swarmer's alarm with no
#             scar (swarm). Expect more.
#  H3 habit:  walkers killed by the last 8 strikes, scar_habit 1 (the
#             alarm term / (1 + scar), scar weight 0) against flee. Expect
#             more (a mould that stops fleeing a repeated trick).
set -u
OUT=${1:-/tmp/habit}; JOBS=${2:-4}; mkdir -p "$OUT"
BIN=${BIN:-./target/release/examples/bigdish}
W="preset=today scale=2 substeps=2 scent_offset=36 scent_coarse=0 piles=23 rect=1 crystal_charge=512 crystal_eat=0.005 walkers=5000 capacity=50000 upkeep=0.001 crystal_regrow=0 terrain_rock=0.2 terrain_wet=0.2"
F="strike=10,12,14,16,18,20,22,24,26,28,30,32,34,36,38,40 strike_pile=1 strike_r=8 alarm_keep=0.98"
S="scar_keep=0.9999 scar_gain=0.01"
LIST="flee|$F alarm_weight=-1
wary|$F alarm_weight=-1 $S scar_weight=-0.5
swarm|$F alarm_weight=3
grudge|$F alarm_weight=3 $S scar_weight=0.5
habit|$F alarm_weight=-1 $S scar_habit=1"
n=0
for s in ${SEEDS:-1 2 3 4 5 6 7 8}; do
  while IFS='|' read -r name args; do
    ( $BIN $W $args seeds=$s seconds=60 report=5 lifeline=1 threads=1 tag=$name > "$OUT/$name-s$s.txt" 2>&1 < /dev/null ) &
    n=$((n+1)); (( n % JOBS == 0 )) && wait
  done <<< "$LIST"
done
wait
