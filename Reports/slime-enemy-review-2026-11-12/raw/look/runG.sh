#!/bin/bash
# Growth arms on the CPU race-like dish (bigdish), seeds 1-3, raw fields at 60 and 120 game s.
B=/home/user/Slime-Mold/target/release/examples/bigdish
RACE="preset=today scent_offset=36 scent_coarse=0 piles=23 rect=1 crystal_charge=512 crystal_eat=0.005 terrain_rock=0.2 terrain_wet=0.2"
OUT=rawG; mkdir -p $OUT
run() { tag=$1; shift; $B $RACE tag=$tag seeds=1,2,3 seconds=15 report=7.5 threads=1 look=$OUT at=7.5,15 style=raw "$@" > $OUT/$tag.log 2>&1; grep -E "alive|ms/tick" $OUT/$tag.log | grep "t  15.0s" ; }
run base    scale=2 substeps=2 capacity=50000 walkers=5000 upkeep=0.001
run dense   scale=2 substeps=2 capacity=250000 walkers=25000 upkeep=0.0002
run coarse  scale=1 substeps=1 scent_offset=18 capacity=50000 walkers=5000 upkeep=0.001
run keep95  scale=2 substeps=2 capacity=50000 walkers=5000 upkeep=0.001 keep=0.95
run nocrowd scale=2 substeps=2 capacity=50000 walkers=5000 upkeep=0.001 crowd=0
run blur2   scale=2 substeps=2 capacity=50000 walkers=5000 upkeep=0.001 trail_passes=2
