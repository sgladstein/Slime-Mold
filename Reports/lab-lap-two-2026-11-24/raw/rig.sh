#!/bin/bash
# rig.sh OUT PLANET SEED [HAND] [pair|base] [more keys...]: one of lap two's planets
# (9 to 15) on the lab branch. LAB_LAP2=1 lets run=SEED:PLANET through: levelgen's own
# map at planet 8's size, the ladder's tier again in order (9 Drifter ... 15 Grudge),
# every per-planet row planet 8's. ARM pair (default) adds the second trait's keys from
# pairs.txt; base leaves them off. LAB_LAP2_SIZE (default here SIZE, else 0.6) scales the
# slime. More keys go after, for lap two's numbers.
out=$1; planet=$2; seed=$3; hand=${4:-greedy}; arm=${5:-pair}; shift 5 2>/dev/null || shift $#
here="$(cd "$(dirname "$0")" && pwd)"
keys=$(grep "^$planet|" "$here/pairs.txt" | cut -d'|' -f4)
[ "$arm" = base ] && keys=""
mkdir -p "$out"
cd "$here/../../.."
export VK_ICD_FILENAMES=${VK_ICD_FILENAMES:-/usr/share/vulkan/icd.d/lvp_icd.json}
LAB_LAP2=1 LAB_LAP2_SIZE=${SIZE:-0.6} ./target/release/larder-world look="$out" run=$seed:$planet mode=squads hand=$hand bank=3000 at=${AT:-1,60} $keys "$@" > "$out/log.txt" 2>&1
