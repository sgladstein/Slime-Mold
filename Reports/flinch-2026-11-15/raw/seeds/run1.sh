#!/bin/sh
# run1.sh SEED VARIANT: planet 1 of run SEED, the squad hand, to 120 s.
# B: larder-world built at 26262751 (the alarm-haze PR, #354), before the
# flinch key; the raw alarm keys below play the same as flinch=0.9 / 0.95 on
# the flinch build (checked to the walker at 30 s on seed 1).
# Run as: (for s in 2 3 4 5; do for v in off k90 k95; do echo "$s $v"; done; done) | xargs -P 3 -n 2 ./run1.sh
# Seed 1 ran the same command with frames every 0.25 s from 10 to 20 s and
# at 30, 45, 60, 90 and 120 s.
S=${S:-seeds}
B=${B:-larder-world}
case $2 in off) K="";; k90) K="alarm_keep=0.9 alarm_weight=-1";; k95) K="alarm_keep=0.95 alarm_weight=-1";; esac
mkdir -p $S/$1-$2
timeout 1800 nice $B run=$1:1 mode=squads hand=squads look=$S/$1-$2 at=30,60,90,120 screen=320x200 $K > $S/$1-$2.txt 2>&1
