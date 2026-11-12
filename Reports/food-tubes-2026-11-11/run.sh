#!/usr/bin/env bash
# Food joined by tubes: the food-reach droplets against a network that grows
# on crystals, on the Larder race world's numbers (2026-11-11).
# The owner (21:36Z, on the graphics thread's videos): the food-reach
# settings "shoot out in waves that reach the crystals but never ... connect
# a network to them". From the repo root, after
# cargo build --release --example bigdish:
#   bash Reports/food-tubes-2026-11-11/run.sh [JOBS] [OUT]
# The world is larder_world::race_params as bigdish keys (BigParams::today()
# with scale 2, substeps 2, 23 crystals of charge 512 that run dry, eaten at
# 0.005, rect, rock and wet ground 0.2, fine scent smelled 36 fine cells out).
# Arms:
# - droplets: the creature thread's food-reach settings as the graphics
#   thread ran them (crowd 1, sensor angle 90, turn 45, reach 9, deposit 5,
#   keep 0.93, blocked deposit 1, graze weight 10), no growth, 7,500 walkers
#   loose in a disc of 75 dish cells at the world's centre (about 10% full,
#   the graphics thread's density and share of the world).
# - race: the race world as the Larder game starts it: today's network
#   rules (sensor angle 45, turn 45, reach 9, deposit 5, keep 0.9, crowd 1)
#   with W0 growth (upkeep 0.001, start energy 0.5, birth at 1.0), 5,000
#   walkers packed on the home crystal, room for 50,000.
# - racecry (dropped after seed 1): race with graze_eat 0, meant as race
#   without grass. Its seed 1 matched race to the end hash
#   (180c6fe0cd15b653): with crystals on, growth makes no grass unless a
#   graze_weight asks for it (big.rs), so the two arms are one arm.
# A first look (seeds 1-3, 30 s) also ran race's growth from the droplets'
# wide centred disc: it joined 20-22 of 23 crystals by 5 s, so a wide start
# is not a test of spreading; the race start is.
# Deciding measure, named before the run: crystals on the mould's main body
# at 60 s (the `joined` line: the largest 4-connected piece of tube; pile 0,
# the home, not counted), race minus droplets and racecry minus droplets on
# paired seeds 1-16, sign test with ties dropped, Holm over the two. With
# racecry dropped as the same arm, the one comparison is quoted with its p
# doubled, as Holm over the two named would have given.
# Secondary: joined at 15, 30 and 90 s (the pace); crystals first reached by
# 60 s (`firsts`); and of the crystals reached by 60 s, the share joined.
set -u
JOBS=${1:-4}; OUT=${2:-/tmp/food-tubes}; mkdir -p "$OUT"
BIN=${BIN:-./target/release/examples/bigdish}
SEEDS=${SEEDS:-"$(seq -s ' ' 1 16)"}; SECS=${SECS:-90}
LOOK=${LOOK:-}   # a folder: stills of seeds 1 and 2 at 5, 15, 30, 60, 90 s
R="preset=today scale=2 substeps=2 scent_offset=36 scent_coarse=0 piles=23 rect=1 crystal_charge=512 crystal_eat=0.005 terrain_rock=0.2 terrain_wet=0.2 blob=1"
LIST="droplets|$R walkers=7500 blob_r=75 crowd=1 sensor_angle=90 rotate_angle=45 sensor_offset=9 deposit=5 keep=0.93 blocked_deposit=1 graze_weight=10
race|$R walkers=5000 capacity=50000 upkeep=0.001"
n=0
for s in $SEEDS; do
  while IFS='|' read -r name args; do
    f="$OUT/$name-s$s.txt"
    [ -s "$f" ] && grep -q "firsts" "$f" && continue   # done before a restart
    extra=""
    if [ -n "$LOOK" ] && [ "$s" -le 2 ]; then mkdir -p "$LOOK"; extra="look=$LOOK at=5,15,30,60,90"; fi
    ( $BIN $args seeds=$s tag=$name report=5 joined=1 threads=1 seconds=$SECS $extra 2>&1 | grep -E "^seed" ) > "$f" &
    n=$((n+1)); (( n % JOBS == 0 )) && wait
  done <<< "$LIST"
done
wait
grep -h -E " (60|90)\.0s  joined|firsts" "$OUT"/*.txt
