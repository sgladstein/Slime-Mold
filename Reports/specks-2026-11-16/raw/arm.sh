#!/bin/sh
# arm.sh RUN NAME KEYS...: larder-world RUN (e.g. 2:2 is run seed 2, planet 2), the squad hand,
# a whole-map and a near frame at each of AT's game seconds, the readout in OUT/NAME.txt.
# From the repo root; LW is larder-world (release) built at the commit named in runs/README, OUT a scratch folder.
run=$1; name=$2; shift 2
timeout 5000 nice $LW run=$run mode=squads hand=squads look=$OUT/$name at=${AT:-30,60,90,120} screen=960x600 "$@" > $OUT/$name.txt 2>&1
