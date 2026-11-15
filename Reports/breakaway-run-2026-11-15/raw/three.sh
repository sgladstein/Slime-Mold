#!/bin/sh
# two.sh RUN VARIANT: RUN=SEED:DEPTH, the squad hand.
# From the repo root; LW is larder-world (release) built at main 6d072828, OUT a scratch folder.
case $2 in c300only) K="condense=300 roam=0";; c300) K="condense=300 roam=300";; off) K="roam=0";; esac
n=$(echo $1 | tr : -)
timeout 2400 nice $LW run=$1 mode=squads hand=squads look=$OUT/$n-$2 at=30,60,90 screen=640x400 $K > $OUT/$n-$2.txt 2>&1
