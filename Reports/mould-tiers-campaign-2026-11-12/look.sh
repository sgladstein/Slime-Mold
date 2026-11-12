#!/usr/bin/env bash
# Looks first: candidate Drifter, Sprawler and Trunk settings in the Larder
# world's race (the census world, larder_world::race_params at scale 2:
# 512 x 320, 23 crystals of charge 512 that run dry, rock and wet ground,
# 5,000 walkers on the home crystal in 50,000 slots, upkeep 0.001).
#   bash Reports/mould-tiers-campaign-2026-11-12/look.sh WORK [ARMS...]
# ARMS=FILE picks the arms (candidates-1..3.txt, habit-look.txt); arms are NAME|ARGS lines in the ARMS file (default: arms.txt beside this).
set -u
WORK=${1:-/tmp/tiers-look}; mkdir -p "$WORK"
ARMS=${ARMS:-$(dirname "$0")/arms.txt}
BIN=${BIN:-./target/release/examples/bigdish}
W="preset=today scale=2 substeps=2 scent_offset=36 scent_coarse=0 piles=23 rect=1 crystal_charge=512 crystal_eat=0.005 walkers=5000 capacity=50000 upkeep=0.001 crystal_regrow=0 terrain_rock=0.2 terrain_wet=0.2"
SECS=${SECS:-60}; AT=${AT:-5,15,30,60}; SEED=${SEED:-1}; JOBS=${JOBS:-4}
ONLY=${ONLY:-}
n=0
while IFS='|' read -r name args; do
  [[ -z "$name" || "$name" == \#* ]] && continue
  [[ -n "$ONLY" && " $ONLY " != *" $name "* ]] && continue
  rm -rf "$WORK/$name"; mkdir -p "$WORK/$name"
  ( $BIN $W $args seeds=$SEED seconds=$SECS report=5 lifeline=1 threads=1 style=life tag=$name look="$WORK/$name" at=$AT > "$WORK/$name.log" 2>&1 < /dev/null ) &
  n=$((n+1)); (( n % JOBS == 0 )) && wait
done < "$ARMS"
wait
