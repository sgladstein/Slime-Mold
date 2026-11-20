#!/bin/bash
# boss census on Scott's own maps: run seed, planet
export VK_ICD_FILENAMES=/usr/share/vulkan/icd.d/lvp_icd.json
BIN=${BIN:-target/release/larder-world}
OUT=/tmp/claude-0/cen/$1
run() { s=$1; p=$2; d=$OUT/$s-$p; mkdir -p $d; $BIN look=$d run=$s:$p mode=squads hand=strat:econ,go=4,aim=6 at=${SECS:-420} screen=320x200 > $d/log.txt 2>&1; }
shift
for sp in "$@"; do run ${sp%:*} ${sp#*:} & 
  while [ $(jobs -r | wc -l) -ge ${PAR:-3} ]; do sleep 2; done
done
wait
