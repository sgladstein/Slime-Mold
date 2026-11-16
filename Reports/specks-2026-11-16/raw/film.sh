#!/bin/sh
# film.sh ARM: planet 1 of run 1, the squad hand, the whole map every 1 s to 4:00.
# From the repo root; LW is larder-world (release) built at this branch, OUT a scratch folder.
# ARM on: the run's keys (specks at roam 300, roam_share 0.05); off: roam=0.
case $1 in on) K="";; off) K="roam=0";; esac
AT=$(seq -s, 1 240)
timeout 7000 nice $LW run=1:1 mode=squads hand=squads look=$OUT/film-$1 at=$AT screen=960x600 $K > $OUT/film-$1.txt 2>&1
