#!/bin/bash
# Films for the raid march and the breakaway tether (2026-11-18): planet 1
# of run seed 1 with the pumps hand, the first raid forced at 0:20 so the
# films stay short, a frame every half game second from 0:20 to 0:56
# (the tether's films: every quarter second from 0:02 to 0:20, before it).
#   march6, march12   the raid as it lands (a march), zoom 6 and 12
#   stream6, stream12 the same raid as before (set=raid_march=0)
#   tether0, tether2  the slime's own edge at zoom 6 before the raid,
#                     specks untethered (tether=0) and as they land (2)
# Software GPU:
#   apt-get install -y mesa-vulkan-drivers
#   export VK_ICD_FILENAMES=/usr/share/vulkan/icd.d/lvp_icd.json
# usage: film.sh BIN OUT_DIR   (run from the repo root)
set -e
B=$1; OUT=$2
AT=$(seq -s, 20 0.5 56)
AT_T=$(seq -s, 2 0.25 20)
RAID="set=raid_first=20 set=raid_first_sd=0 set=raid_early=0"
film() { # film NAME AT keys...
  local n=$1 at=$2; shift 2
  mkdir -p $OUT/$n
  $B look=$OUT/$n run=1:1 mode=squads hand=pumps at=$at $RAID "$@" > $OUT/$n/out.txt 2> $OUT/$n/err.txt
}
film march6 $AT centre=230,70,6 &
film stream6 $AT centre=230,70,6 set=raid_march=0 &
film march12 $AT centre=172,98,12 &
wait
film stream12 $AT centre=172,98,12 set=raid_march=0 &
film tether0 $AT_T centre=150,80,6 tether=0 &
film tether2 $AT_T centre=150,80,6 &
wait
