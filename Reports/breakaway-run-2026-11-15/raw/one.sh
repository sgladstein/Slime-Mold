#!/bin/sh
# one.sh SEED VARIANT: planet 2 of run SEED, the squad hand, to 180 s.
# From the repo root; LW is larder-world (release) built at main 6d072828, OUT a scratch folder.
case $2 in off) K="roam=0";; r300) K="roam=300";; c300) K="condense=300 roam=300";; esac
timeout 2400 nice $LW run=$1:2 mode=squads hand=squads look=$OUT/$1-$2 at=30,60,90,120,150,180 screen=640x400 $K > $OUT/$1-$2.txt 2>&1
