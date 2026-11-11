#!/bin/sh
# Grass and crystals in one world (2026-11-11, owner's question 16:05Z): Larder's world, seed 1, 60 s,
# crystals that run dry with crystal_flow 0.01 (PR #64 at f292952 plus growth, scratch 65d914f).
S=${S:?set S to a scratch folder}
B=${B:-target/release/examples/bigdish}
G="preset=today scale=2 substeps=2 scent_offset=36 piles=23 rect=1 crystal_eat=0.005 walkers=5000 capacity=50000 upkeep=0.001 seconds=60 report=5 threads=1 crystal_regrow=0 look=$S/gc at=5,10,20,40,60"
$B $G crystal_charge=1 tag=cry > $S/gc/cry.txt 2>&1 &
$B $G crystal_charge=1 graze_weight=10 graze_regrow=0 tag=crypantry > $S/gc/crypantry.txt 2>&1 &
$B $G crystal_charge=1 graze_weight=10 graze_regrow=0.0001 tag=crygrass > $S/gc/crygrass.txt 2>&1 &
$B $G crystal_charge=0 graze_weight=10 graze_regrow=0.0001 tag=grass > $S/gc/grass.txt 2>&1 &
wait
