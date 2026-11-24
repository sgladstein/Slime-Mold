#!/bin/bash
# rig9.sh OUT SEED [HAND] [ARM] [more keys...]: lap two's planet 9 on the lab branch.
# LAB_LAP2=1 lets run=SEED:9 through: planet 9's own map (levelgen depth 9, planet 8's
# size), the Drifter as its tier, every per-planet row planet 8's. ARM pair (default)
# adds the Sporer's bursts (pairs.txt); ARM base leaves them off. More keys go after,
# e.g. set=raid_first=90 or capacity=..., for lap two's numbers.
out=$1; seed=$2; hand=${3:-greedy}; arm=${4:-pair}; shift 4 2>/dev/null || shift $#
keys="alarm_keep=0.98 spore_rate=0.00001 spore_burst=0.2 spore_alarm=0.3"
[ "$arm" = base ] && keys=""
mkdir -p $out
cd "$(dirname "$0")/../../.."
export VK_ICD_FILENAMES=${VK_ICD_FILENAMES:-/usr/share/vulkan/icd.d/lvp_icd.json}
LAB_LAP2=1 ./target/release/larder-world look=$out run=$seed:9 mode=squads hand=$hand bank=3000 at=${AT:-1,60} $keys "$@" > $out/log.txt 2>&1
