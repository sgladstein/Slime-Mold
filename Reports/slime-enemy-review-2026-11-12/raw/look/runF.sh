#!/bin/bash
# Fixed population (no growth), race-like dish, seeds 1-3, raw at 60 and 120 game s.
B=/home/user/Slime-Mold/target/release/examples/bigdish
RACE="preset=today scent_offset=36 scent_coarse=0 piles=23 rect=1 crystal_charge=512 crystal_eat=0.005 terrain_rock=0.2 terrain_wet=0.2 scale=2 substeps=2 upkeep=0"
OUT=rawF; mkdir -p $OUT
run() { tag=$1; shift; $B $RACE tag=$tag seeds=1,2,3 seconds=15 report=15 threads=1 look=$OUT at=7.5,15 style=raw "$@" > $OUT/$tag.log 2>&1; grep -E "ms/tick" $OUT/$tag.log | grep "t  15.0s" ; }
run fix5      walkers=5000
run fix10     walkers=10000
run fix25     walkers=25000
run fix5keep95  walkers=5000 keep=0.95
run fix5nocrowd walkers=5000 crowd=0
run fix5blur2   walkers=5000 trail_passes=2
